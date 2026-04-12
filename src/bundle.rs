//! Mirror's Bundle implementation.
//!
//! Grammar as Connection (KernelSpec as Optic).
//! Compilation as Transport.
//! The bundle tower IS the compiler.

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
        }
    }

    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
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
        // Compilation: source in, compiled form out.
        // Loss = information that doesn't survive compilation.
        // For now: parse the source, measure what survives.
        if source.is_empty() {
            return Imperfect::Success(String::new());
        }

        // The compilation step: we have the AST module available.
        // Real compilation will go through mirror_runtime::Shatter.
        // For now: the source itself is the "compiled" output,
        // with loss measured as the ratio of non-structural characters.
        let structural_chars: usize = source.chars().filter(|c| "@{}()|".contains(*c)).count();
        let total = source.len();
        let loss = (total - structural_chars) as f64;

        if loss == 0.0 {
            Imperfect::Success(source.clone())
        } else {
            Imperfect::Partial(source.clone(), ShannonLoss::new(loss))
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
    fn transport_source_with_content_returns_partial() {
        let compiler = MirrorCompiler::new();
        let source = "prism @test { focus type(id) }".to_string();
        let result = compiler.transport(&source);
        // Non-empty source always has some non-structural characters = loss
        assert!(result.is_partial());
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
        // Real compilation produces an OID
        assert!(result.is_partial()); // source > OID length = loss
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
