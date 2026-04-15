//! emit_code — generic code emitter driven by @code grammar templates.
//!
//! Phase 1: IoList + CodeGrammar::rust() skeleton.

use crate::declaration::{DeclKind, MirrorFragment, OpticOp};
use crate::mirror_runtime::{CompiledShatter, Form};
use prism::{Imperfect, Loss};

// ---------------------------------------------------------------------------
// IoList — the output type
// ---------------------------------------------------------------------------

/// A tree of byte slices. No allocation for concatenation.
#[derive(Clone, Debug)]
pub enum IoList {
    Chunk(Vec<u8>),
    Nested(Vec<IoList>),
    Empty,
}

impl IoList {
    pub fn text(s: &str) -> Self {
        IoList::Chunk(s.as_bytes().to_vec())
    }

    pub fn join(parts: Vec<IoList>) -> Self {
        IoList::Nested(parts)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.flatten_into(&mut out);
        out
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf8(self.to_bytes()).unwrap_or_default()
    }

    fn flatten_into(&self, out: &mut Vec<u8>) {
        match self {
            IoList::Chunk(bytes) => out.extend_from_slice(bytes),
            IoList::Nested(children) => {
                for child in children {
                    child.flatten_into(out);
                }
            }
            IoList::Empty => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Supporting types
// ---------------------------------------------------------------------------

pub struct FormField {
    pub name: String,
    pub type_ref: String,
}

pub struct FormParam {
    pub name: String,
    pub type_ref: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Visibility {
    Public,
    Private,
}

// ---------------------------------------------------------------------------
// CodeGrammar + TemplateSet
// ---------------------------------------------------------------------------

pub struct CodeGrammar {
    pub name: String,
    pub templates: TemplateSet,
}

pub struct TemplateSet {
    pub map_type: Box<dyn Fn(&str) -> IoList>,
    pub type_name: Box<dyn Fn(&str) -> String>,
    pub field_name: Box<dyn Fn(&str) -> String>,
    pub function_name: Box<dyn Fn(&str) -> String>,
    pub module_name: Box<dyn Fn(&str) -> String>,
    pub variant_name: Box<dyn Fn(&str) -> String>,
    pub emit_enum: Box<dyn Fn(&str, &[String], &[String]) -> IoList>,
    pub emit_struct: Box<dyn Fn(&str, &[FormField], &[String]) -> IoList>,
    pub emit_unit_type: Box<dyn Fn(&str) -> IoList>,
    pub emit_generic_type: Box<dyn Fn(&str, &[String]) -> IoList>,
    pub emit_module: Box<dyn Fn(&str, &[IoList]) -> IoList>,
    pub emit_function: Box<dyn Fn(&str, &[FormParam], Option<&str>, Visibility) -> IoList>,
    pub emit_property: Box<dyn Fn(&str, &[FormParam]) -> IoList>,
    pub emit_comment: Box<dyn Fn(&str) -> IoList>,
    pub emit_header: Box<dyn Fn(&str) -> IoList>,
}

// ---------------------------------------------------------------------------
// EmitError, EmitLoss
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum EmitError {
    UnsupportedDeclKind(DeclKind),
    TemplateError(String),
}

impl std::fmt::Display for EmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmitError::UnsupportedDeclKind(k) => write!(f, "unsupported DeclKind: {}", k.as_str()),
            EmitError::TemplateError(s) => write!(f, "template error: {}", s),
        }
    }
}

impl std::error::Error for EmitError {}

#[derive(Clone, Debug, Default)]
pub struct EmitLoss {
    pub skipped: Vec<String>,
    pub unmapped_types: Vec<String>,
}

impl Loss for EmitLoss {
    fn zero() -> Self {
        EmitLoss::default()
    }

    fn total() -> Self {
        EmitLoss {
            skipped: vec!["<total>".to_string()],
            unmapped_types: vec![],
        }
    }

    fn is_zero(&self) -> bool {
        self.skipped.is_empty() && self.unmapped_types.is_empty()
    }

    fn combine(mut self, other: Self) -> Self {
        self.skipped.extend(other.skipped);
        self.unmapped_types.extend(other.unmapped_types);
        self
    }
}

// ---------------------------------------------------------------------------
// Stubs — to be implemented in 🟢
// ---------------------------------------------------------------------------

/// Emit code from a compiled shatter artifact using a @code grammar.
pub fn emit_code(
    _compiled: &CompiledShatter,
    _grammar: &CodeGrammar,
) -> Imperfect<IoList, EmitError, EmitLoss> {
    // Stub: returns empty output. Tests will fail.
    Imperfect::Success(IoList::Empty)
}

/// Emit code from a Form using a @code grammar.
pub(crate) fn emit_code_form(
    _form: &Form,
    _grammar: &CodeGrammar,
) -> Imperfect<IoList, EmitError, EmitLoss> {
    // Stub: returns empty output. Tests will fail.
    Imperfect::Success(IoList::Empty)
}

impl CodeGrammar {
    /// Build the Rust code grammar — stub, to be implemented in 🟢.
    pub fn rust() -> Self {
        CodeGrammar {
            name: "rust".to_string(),
            templates: TemplateSet {
                map_type: Box::new(|_| IoList::Empty),
                type_name: Box::new(|s| s.to_string()),
                field_name: Box::new(|s| s.to_string()),
                function_name: Box::new(|s| s.to_string()),
                module_name: Box::new(|s| s.to_string()),
                variant_name: Box::new(|s| s.to_string()),
                emit_enum: Box::new(|_, _, _| IoList::Empty),
                emit_struct: Box::new(|_, _, _| IoList::Empty),
                emit_unit_type: Box::new(|_| IoList::Empty),
                emit_generic_type: Box::new(|_, _| IoList::Empty),
                emit_module: Box::new(|_, _| IoList::Empty),
                emit_function: Box::new(|_, _, _, _| IoList::Empty),
                emit_property: Box::new(|_, _| IoList::Empty),
                emit_comment: Box::new(|_| IoList::Empty),
                emit_header: Box::new(|_| IoList::Empty),
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Tests — 🔴 these must FAIL until the implementation is done
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit_rust;
    use crate::mirror_runtime::MirrorRuntime;
    use std::path::PathBuf;

    // IoList unit tests (these pass immediately)
    #[test]
    fn iolist_empty() {
        assert_eq!(IoList::Empty.to_bytes(), Vec::<u8>::new());
    }

    #[test]
    fn iolist_chunk() {
        assert_eq!(IoList::text("hello").to_bytes(), b"hello");
    }

    #[test]
    fn iolist_nested() {
        let io = IoList::Nested(vec![IoList::text("a"), IoList::text("b"), IoList::text("c")]);
        assert_eq!(io.to_string_lossy(), "abc");
    }

    #[test]
    fn iolist_deeply_nested() {
        let io = IoList::Nested(vec![
            IoList::text("a"),
            IoList::Nested(vec![IoList::text("b"), IoList::text("c")]),
            IoList::Empty,
            IoList::text("d"),
        ]);
        assert_eq!(io.to_string_lossy(), "abcd");
    }

    // Parallel emit tests — these FAIL until implementation
    fn assert_identical(source: &str) {
        let runtime = MirrorRuntime::new();
        let compiled: Result<CompiledShatter, _> = runtime.compile_source(source).into();
        let compiled = compiled.unwrap();

        let rust_output = emit_rust::emit_rust(&compiled);
        let grammar = CodeGrammar::rust();
        let code_result = emit_code(&compiled, &grammar);
        let code_output = match code_result {
            Imperfect::Success(io) => io.to_string_lossy(),
            Imperfect::Partial(io, _) => io.to_string_lossy(),
            Imperfect::Failure(e, _) => panic!("emit_code failed: {:?}", e),
        };

        assert_eq!(
            rust_output, code_output,
            "emit_rust and emit_code must produce identical output for:\n{}",
            source
        );
    }

    #[test]
    fn emit_code_simple_enum() {
        assert_identical("type color = red | blue");
    }

    #[test]
    fn emit_code_unit_struct() {
        assert_identical("type point");
    }

    #[test]
    fn emit_code_struct_with_fields() {
        assert_identical("type user {\n  name: text,\n  email: text,\n}");
    }

    #[test]
    fn emit_code_grammar_becomes_module() {
        assert_identical("grammar @test {\n  type x\n}");
    }

    #[test]
    fn emit_code_action_becomes_function() {
        assert_identical("action boot(identity)");
    }

    #[test]
    fn emit_code_property_becomes_function() {
        assert_identical("property types_lowercase(grammar)");
    }

    #[test]
    fn emit_code_boot_meta() {
        let runtime = MirrorRuntime::new();
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("boot/01-meta.mirror");
        let compiled = runtime.compile_file(&path).unwrap();
        let rust_output = emit_rust::emit_rust(&compiled);
        let grammar = CodeGrammar::rust();
        let code_result = emit_code(&compiled, &grammar);
        let code_output = match code_result {
            Imperfect::Success(io) => io.to_string_lossy(),
            Imperfect::Partial(io, _) => io.to_string_lossy(),
            Imperfect::Failure(e, _) => panic!("emit_code failed: {:?}", e),
        };
        assert_eq!(rust_output, code_output, "boot/01-meta.mirror must match");
    }
}
