//! @code/llvm — compile MirrorAST grammars to native executables.
//!
//! The @io boundary for @code/llvm.compile. Uses cranelift as the
//! code generation backend. The grammar is called @code/llvm because
//! the name is the target (native binary), not the implementation detail.
//!
//! Pipeline:
//!   MirrorAST -> cranelift IR -> object file -> link -> executable
//!
//! The first version produces a minimal binary that prints the content
//! OID of the compiled grammar, proving the pipeline works end-to-end.

use crate::mirror_runtime::CompiledShatter;

use cranelift_codegen::ir::types::I64;
use cranelift_codegen::ir::{AbiParam, Function, InstBuilder, UserFuncName};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_codegen::Context;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

use std::path::Path;
use std::process::Command;

// ---------------------------------------------------------------------------
// Error
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum CodeLlvmError {
    Codegen(String),
    Link(String),
    Io(std::io::Error),
}

impl std::fmt::Display for CodeLlvmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeLlvmError::Codegen(msg) => write!(f, "codegen: {}", msg),
            CodeLlvmError::Link(msg) => write!(f, "link: {}", msg),
            CodeLlvmError::Io(e) => write!(f, "io: {}", e),
        }
    }
}

impl std::error::Error for CodeLlvmError {}

impl From<std::io::Error> for CodeLlvmError {
    fn from(e: std::io::Error) -> Self {
        CodeLlvmError::Io(e)
    }
}

// ---------------------------------------------------------------------------
// Compilation result
// ---------------------------------------------------------------------------

/// The result of compiling a grammar to native code.
pub struct NativeArtifact {
    /// The object file bytes (ELF/Mach-O).
    pub object_bytes: Vec<u8>,
    /// The content OID of the source grammar.
    pub crystal_oid: String,
}

// ---------------------------------------------------------------------------
// compile_to_object — MirrorAST -> object file bytes
// ---------------------------------------------------------------------------

/// Compile a MirrorAST grammar to a native object file.
///
/// The produced binary, when run, prints the crystal OID of the compiled
/// grammar to stdout and exits with code 0. This is the minimal viable
/// proof that the pipeline works.
pub fn compile_to_object(compiled: &CompiledShatter) -> Result<NativeArtifact, CodeLlvmError> {
    let crystal_oid = compiled.crystal().as_str().to_string();

    // --- Configure the target ISA ---
    let mut flag_builder = settings::builder();
    flag_builder
        .set("is_pic", "true")
        .map_err(|e| CodeLlvmError::Codegen(format!("setting is_pic: {}", e)))?;
    let isa_builder = cranelift_native::builder()
        .map_err(|e| CodeLlvmError::Codegen(format!("native ISA: {}", e)))?;
    let isa = isa_builder
        .finish(settings::Flags::new(flag_builder))
        .map_err(|e| CodeLlvmError::Codegen(format!("finish ISA: {}", e)))?;

    // --- Create the object module ---
    let obj_builder = ObjectBuilder::new(
        isa,
        "mirror_grammar",
        cranelift_module::default_libcall_names(),
    )
    .map_err(|e| CodeLlvmError::Codegen(format!("ObjectBuilder: {}", e)))?;
    let mut module = ObjectModule::new(obj_builder);

    // --- Declare external libc functions ---
    // We need: write(fd, buf, len)
    let mut write_sig = module.make_signature();
    write_sig.params.push(AbiParam::new(I64)); // fd
    write_sig.params.push(AbiParam::new(I64)); // buf pointer
    write_sig.params.push(AbiParam::new(I64)); // len
    write_sig.returns.push(AbiParam::new(I64)); // return value
    let write_func = module
        .declare_function("write", Linkage::Import, &write_sig)
        .map_err(|e| CodeLlvmError::Codegen(format!("declare write: {}", e)))?;

    // --- Declare the OID string as a data section ---
    let oid_with_newline = format!("{}\n", crystal_oid);
    let oid_bytes = oid_with_newline.as_bytes();

    let oid_data_id = module
        .declare_data("oid_str", Linkage::Local, false, false)
        .map_err(|e| CodeLlvmError::Codegen(format!("declare data: {}", e)))?;

    let mut data_desc = cranelift_module::DataDescription::new();
    data_desc.define(oid_bytes.to_vec().into_boxed_slice());
    module
        .define_data(oid_data_id, &data_desc)
        .map_err(|e| CodeLlvmError::Codegen(format!("define data: {}", e)))?;

    // --- Build main() ---
    let mut main_sig = module.make_signature();
    main_sig.returns.push(AbiParam::new(I64)); // return int
    let main_func_id = module
        .declare_function("main", Linkage::Export, &main_sig)
        .map_err(|e| CodeLlvmError::Codegen(format!("declare main: {}", e)))?;

    let mut func = Function::with_name_signature(
        UserFuncName::user(0, 0),
        main_sig.clone(),
    );
    let mut func_ctx = FunctionBuilderContext::new();
    {
        let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        // Get pointer to the OID string data
        let oid_gv = module.declare_data_in_func(oid_data_id, builder.func);
        let ptr_type = module.target_config().pointer_type();
        let oid_ptr = builder.ins().global_value(ptr_type, oid_gv);

        // Call write(1, oid_ptr, len)
        let write_ref = module.declare_func_in_func(write_func, builder.func);
        let fd = builder.ins().iconst(I64, 1); // stdout
        let len = builder.ins().iconst(I64, oid_bytes.len() as i64);
        builder.ins().call(write_ref, &[fd, oid_ptr, len]);

        // Return 0
        let zero = builder.ins().iconst(I64, 0);
        builder.ins().return_(&[zero]);

        builder.finalize();
    }

    // --- Compile and define the function ---
    let mut ctx = Context::for_function(func);
    module
        .define_function(main_func_id, &mut ctx)
        .map_err(|e| CodeLlvmError::Codegen(format!("define main: {}", e)))?;

    // --- Emit the object file ---
    let object = module.finish();
    let object_bytes = object.emit()
        .map_err(|e| CodeLlvmError::Codegen(format!("emit object: {}", e)))?;

    Ok(NativeArtifact {
        object_bytes,
        crystal_oid,
    })
}

// ---------------------------------------------------------------------------
// link — object file -> executable binary
// ---------------------------------------------------------------------------

/// Link an object file into an executable binary using the system linker.
pub fn link_object(artifact: &NativeArtifact, output_path: &Path) -> Result<(), CodeLlvmError> {
    // Write the object file to a temp location
    let obj_path = output_path.with_extension("o");
    std::fs::write(&obj_path, &artifact.object_bytes)?;

    // Link with cc (clang on macOS via nix)
    let status = Command::new("cc")
        .arg(&obj_path)
        .arg("-o")
        .arg(output_path)
        .arg("-lSystem") // macOS system library for write/_exit
        .status()?;

    // Clean up the object file
    let _ = std::fs::remove_file(&obj_path);

    if !status.success() {
        return Err(CodeLlvmError::Link(format!(
            "linker exited with status: {}",
            status
        )));
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// compile_to_binary — the full pipeline
// ---------------------------------------------------------------------------

/// Compile a grammar to a native executable binary.
///
/// This is the @io boundary for `@code/llvm.compile(ast) -> artifact`.
pub fn compile_to_binary(
    compiled: &CompiledShatter,
    output_path: &Path,
) -> Result<String, CodeLlvmError> {
    let artifact = compile_to_object(compiled)?;
    let oid = artifact.crystal_oid.clone();
    link_object(&artifact, output_path)?;
    Ok(oid)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mirror_runtime::MirrorRuntime;

    #[test]
    fn compile_grammar_to_object() {
        let rt = MirrorRuntime::new();
        let compiled: Result<CompiledShatter, _> = rt
            .compile_source("grammar @test/llvm { type id }")
            .into();
        let compiled = compiled.expect("compile source");
        let artifact = compile_to_object(&compiled).expect("compile to object");

        // The object file should be non-empty
        assert!(!artifact.object_bytes.is_empty(), "object bytes should be non-empty");

        // The OID should match the compiled crystal
        assert_eq!(artifact.crystal_oid, compiled.crystal().as_str());
    }

    #[test]
    fn compile_grammar_to_binary_and_run() {
        let rt = MirrorRuntime::new();
        let compiled: Result<CompiledShatter, _> = rt
            .compile_source("grammar @test/llvm { type id }")
            .into();
        let compiled = compiled.expect("compile source");
        let expected_oid = compiled.crystal().as_str().to_string();

        // Compile to a temp binary
        let dir = tempfile::tempdir().expect("tempdir");
        let binary_path = dir.path().join("test_grammar");
        let oid = compile_to_binary(&compiled, &binary_path).expect("compile to binary");
        assert_eq!(oid, expected_oid);

        // The binary should exist and be executable
        assert!(binary_path.exists(), "binary should exist at {:?}", binary_path);

        // Run the binary and check its output
        let output = std::process::Command::new(&binary_path)
            .output()
            .expect("run binary");
        assert!(output.status.success(), "binary should exit 0, got: {:?}", output.status);

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(
            stdout.trim(),
            expected_oid,
            "binary should print crystal OID, got: {}",
            stdout
        );
    }

    #[test]
    fn compile_kintsugi_grammar_to_binary() {
        let rt = MirrorRuntime::new();
        let source = std::fs::read_to_string(
            concat!(env!("CARGO_MANIFEST_DIR"), "/boot/std/kintsugi.mirror")
        ).expect("read kintsugi.mirror");
        let compiled: Result<CompiledShatter, _> = rt.compile_source(&source).into();
        let compiled = compiled.expect("compile kintsugi");

        let dir = tempfile::tempdir().expect("tempdir");
        let binary_path = dir.path().join("kintsugi");
        let oid = compile_to_binary(&compiled, &binary_path).expect("compile kintsugi to binary");

        let output = std::process::Command::new(&binary_path)
            .output()
            .expect("run kintsugi binary");
        assert!(output.status.success(), "kintsugi binary should exit 0");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout).trim(),
            oid,
            "kintsugi binary should print its crystal OID"
        );
    }
}
