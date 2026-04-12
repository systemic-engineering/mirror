//! Mirror's Bundle implementation.
//!
//! Grammar as Connection (KernelSpec as Optic).
//! Compilation as Transport.
//! The bundle tower IS the compiler.

use crate::mirror_runtime::{CompiledShatter, MirrorRuntime, MirrorRuntimeError};
use fragmentation::sha::HashAlg;
use prism::{
    Closure, Connection, Decomposition, Fiber, Gauge, Imperfect, KernelSpec, Precision,
    ShannonLoss, Transport,
};

/// Compilation target.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Target {
    /// BEAM (Erlang VM)
    #[default]
    Beam,
    /// WebAssembly
    Wasm,
    /// Metal/GPU
    Metal,
}

/// The Mirror compiler as a principal bundle.
///
/// Fiber: source text (.mirror content)
/// Connection: KernelSpec (which Fortran kernel)
/// Gauge: Target (BEAM/WASM/Metal)
/// Transport: compilation (source → compiled, with loss)
/// Closure: the compilation artifact
pub struct MirrorCompiler {
    pub kernel_spec: KernelSpec,
    pub target: Target,
    pub artifact_oid: Option<String>,
    runtime: MirrorRuntime,
}

impl Default for MirrorCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl MirrorCompiler {
    pub fn new() -> Self {
        MirrorCompiler {
            kernel_spec: KernelSpec::new(
                (0..8).collect(),
                Decomposition::Eigenvalue,
                Precision::new(0.01),
            ),
            target: Target::default(),
            artifact_oid: None,
            runtime: MirrorRuntime::new(),
        }
    }

    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }

    /// Compile .mirror source through the full pipeline.
    /// Updates artifact_oid (for Closure::close).
    pub fn compile(
        &mut self,
        source: &str,
    ) -> Result<CompiledShatter, MirrorRuntimeError> {
        let compiled = self.runtime.compile_source(source)?;
        self.artifact_oid = Some(compiled.crystal().as_str().to_string());
        Ok(compiled)
    }
}

impl Fiber for MirrorCompiler {
    type State = String; // .mirror source text
}

impl Connection for MirrorCompiler {
    type Optic = KernelSpec;
    fn connection(&self) -> &KernelSpec {
        &self.kernel_spec
    }
}

impl Gauge for MirrorCompiler {
    type Group = Target;
    fn gauge(&self) -> &Target {
        &self.target
    }
}

impl Transport for MirrorCompiler {
    type Holonomy = ShannonLoss;
    fn transport(&self, source: &String) -> Imperfect<String, ShannonLoss> {
        if source.is_empty() {
            return Imperfect::Success(String::new());
        }

        match self.runtime.compile_source(source) {
            Ok(compiled) => {
                // Loss = source length - OID length.
                // The OID is the compressed representation.
                // Everything that isn't in the OID is lost.
                let oid = compiled.crystal().as_str().to_string();
                let source_len = source.len() as f64;
                let oid_len = oid.len() as f64;
                let loss = source_len - oid_len;

                if loss <= 0.0 {
                    Imperfect::Success(oid)
                } else {
                    Imperfect::Partial(oid, ShannonLoss::new(loss))
                }
            }
            Err(_) => {
                // Compilation failure = total loss
                Imperfect::Partial(
                    String::new(),
                    ShannonLoss::new(source.len() as f64),
                )
            }
        }
    }
}

impl Closure for MirrorCompiler {
    type Fixed = Option<String>; // artifact OID, None if not yet compiled
    fn close(&self) -> &Option<String> {
        &self.artifact_oid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prism::Bundle;

    #[test]
    fn mirror_compiler_is_bundle() {
        fn accepts_bundle<B: Bundle>(_b: &B) {}
        let compiler = MirrorCompiler::new();
        accepts_bundle(&compiler);
    }

    #[test]
    fn connection_returns_kernel_spec() {
        let compiler = MirrorCompiler::new();
        let spec = compiler.connection();
        assert_eq!(spec.rank(), 8);
        assert_eq!(spec.decomposition, Decomposition::Eigenvalue);
    }

    #[test]
    fn gauge_returns_target() {
        let compiler = MirrorCompiler::new().with_target(Target::Wasm);
        assert_eq!(*compiler.gauge(), Target::Wasm);
    }

    #[test]
    fn transport_empty_source_is_success() {
        let compiler = MirrorCompiler::new();
        let result = compiler.transport(&String::new());
        assert!(result.is_ok());
    }

    #[test]
    fn transport_source_with_content_compiles_to_oid() {
        let compiler = MirrorCompiler::new();
        let source = "prism @test { focus type(id) }".to_string();
        let result = compiler.transport(&source);
        // Real compilation: the OID is the output.
        // Short sources may produce an OID longer than the source (Success),
        // or shorter (Partial). Either way, transport succeeds (not Failure).
        assert!(!result.is_err());
        assert!(result.ok().is_some_and(|oid| !oid.is_empty()));
    }

    #[test]
    fn closure_none_before_compilation() {
        let compiler = MirrorCompiler::new();
        assert!(compiler.close().is_none());
    }

    #[test]
    fn transport_compiles_real_source() {
        let compiler = MirrorCompiler::new();
        let source = "form @test {\n  prism focus\n}\n".to_string();
        let result = compiler.transport(&source);
        // Real compilation produces an OID. Transport succeeds (not Failure).
        // Short sources produce OIDs >= source length → Success.
        // The OID is the content-addressed output.
        assert!(!result.is_err(), "compilation should not fail");
        let oid = result.ok().expect("should have OID");
        assert!(!oid.is_empty(), "should produce a non-empty OID");
    }

    #[test]
    fn transport_long_source_returns_partial() {
        let compiler = MirrorCompiler::new();
        // A long source should compress to an OID shorter than the source.
        // Repeat a valid form declaration many times.
        let block = "form @test {\n  prism focus\n  prism split\n  prism zoom\n  prism project\n  prism refract\n}\n";
        let source = block.repeat(20);
        let result = compiler.transport(&source);
        assert!(!result.is_err(), "compilation should not fail");
        // With a long enough source, OID < source length = Partial
        assert!(result.is_partial(), "long source should have loss");
        match result {
            Imperfect::Partial(oid, loss) => {
                assert!(!oid.is_empty(), "should produce an OID");
                assert!(loss.as_f64() > 0.0, "compilation should have loss");
            }
            _ => panic!("expected Partial"),
        }
    }

    #[test]
    fn transport_invalid_source_returns_partial_with_max_loss() {
        let compiler = MirrorCompiler::new();
        let source = "this is not valid mirror syntax {{{".to_string();
        let result = compiler.transport(&source);
        // Invalid source = compilation failure = total loss
        assert!(result.is_partial());
    }

    #[test]
    fn compile_stores_artifact_oid() {
        let mut compiler = MirrorCompiler::new();
        let compiled = compiler
            .compile("form @test {\n  prism focus\n}\n")
            .unwrap();
        assert!(compiler.artifact_oid.is_some());
        assert_eq!(
            compiler.artifact_oid.as_ref().unwrap(),
            compiled.crystal().as_str()
        );
    }
}
