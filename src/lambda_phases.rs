//! Named lambda phases for the mirror compiler.
//!
//! Two composition surfaces:
//!
//! 1. **Term-level** — `Composable<T>` / `Into<Lambda<T>>` / `.then()`.
//!    Content-addressed lambda terms. The pipeline IS a term.
//!
//! 2. **Typed-level** — `LambdaFn` with typed Input/Output.
//!    Composition checked at compile time. Loss accumulates through the chain.
//!    `Parse.typed().then(Resolve.typed())` — Rust proves the chain.
//!
//! Each compiler phase is a struct with `#[derive(Lambda)]` and `#[oid("@X")]`.
//! No handler functions. No strings.

// The derive macro expands to `prism_core::` paths.
// In this crate, prism-core is imported as `prism`, so we alias it.
use prism as prism_core;

use prism::lambda::{Composable, Lambda, LambdaFn};
use prism::{DeriveLambda, Imperfect, Loss};

use crate::declaration::MirrorFragment;
use crate::emit_code::{emit_code_fragment, CodeGrammar};
use crate::loss::{MirrorLoss, ResolutionLoss};
use crate::mirror_runtime::{parse_form, MirrorRegistry, MirrorRuntimeError};

use std::path::PathBuf;
use std::sync::LazyLock;

// ---------------------------------------------------------------------------
// Phase structs — each is a named lambda (term-level)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, DeriveLambda)]
#[oid("@parse")]
pub struct Parse;

#[derive(Debug, Clone, DeriveLambda)]
#[oid("@resolve")]
pub struct Resolve {
    /// Path to the `.frgmnt/` store for resolution context.
    /// If None, resolution is a pass-through (no registry to check against).
    pub store_path: Option<PathBuf>,
}

impl Resolve {
    /// Create a Resolve phase with a registry store path.
    pub fn with_store(path: impl Into<PathBuf>) -> Self {
        Resolve {
            store_path: Some(path.into()),
        }
    }

    /// Create a pass-through Resolve (no registry).
    pub fn pass_through() -> Self {
        Resolve { store_path: None }
    }
}

#[derive(Debug, Clone, DeriveLambda)]
#[oid("@emit")]
pub struct Emit;

#[derive(Debug, Clone, DeriveLambda)]
#[oid("@kintsugi")]
pub struct Kintsugi;

#[derive(Debug, Clone, DeriveLambda)]
#[oid("@strict")]
pub struct Strict;

#[derive(Debug, Clone, DeriveLambda)]
#[oid("@properties")]
pub struct Properties;

// ---------------------------------------------------------------------------
// Static pipelines — pre-composed, content-addressed (term-level)
// ---------------------------------------------------------------------------

/// The standard compilation pipeline: parse -> resolve -> properties -> emit.
pub static CRAFT: LazyLock<Lambda<MirrorFragment>> = LazyLock::new(|| {
    Composable::<MirrorFragment>::then(Parse, Resolve::pass_through())
        .then(Properties)
        .then(Emit)
});

/// Kintsugi pipeline: parse -> resolve -> kintsugi (loss-tolerant emit).
pub static KINTSUGI_PIPELINE: LazyLock<Lambda<MirrorFragment>> = LazyLock::new(|| {
    Composable::<MirrorFragment>::then(Parse, Resolve::pass_through()).then(Kintsugi)
});

/// CI pipeline: parse -> resolve -> properties (no emit).
pub static CI: LazyLock<Lambda<MirrorFragment>> = LazyLock::new(|| {
    Composable::<MirrorFragment>::then(Parse, Resolve::pass_through()).then(Properties)
});

// ---------------------------------------------------------------------------
// Typed surface — LambdaFn with typed Input/Output
// ---------------------------------------------------------------------------

/// Source text entering the pipeline. No strings in the type system.
#[derive(Debug)]
pub struct SourceText(pub String);

/// AST after parsing.
#[derive(Debug)]
pub struct ParsedAst(pub MirrorFragment);

/// AST after resolution (currently same as parsed — resolution is inside parse_form).
#[derive(Debug)]
pub struct ResolvedAst(pub MirrorFragment);

/// AST after property checking.
#[derive(Debug)]
pub struct CheckedAst(pub MirrorFragment);

/// Emitted Rust source code.
#[derive(Debug)]
pub struct EmittedCode(pub String);

// -- Parse: SourceText -> ParsedAst --

impl LambdaFn for Parse {
    type Input = SourceText;
    type Output = ParsedAst;
    type Error = MirrorRuntimeError;
    type Loss = MirrorLoss;

    fn reduce(self, input: SourceText) -> Imperfect<ParsedAst, MirrorRuntimeError, MirrorLoss> {
        parse_form(&input.0).map(ParsedAst)
    }
}

// -- Resolve: ParsedAst -> ResolvedAst --
// Opens a MirrorRegistry at the configured store path and resolves the fragment.
// If no store path is set, passes through (for contexts without a registry).

impl LambdaFn for Resolve {
    type Input = ParsedAst;
    type Output = ResolvedAst;
    type Error = MirrorRuntimeError;
    type Loss = MirrorLoss;

    fn reduce(
        self,
        input: ParsedAst,
    ) -> Imperfect<ResolvedAst, MirrorRuntimeError, MirrorLoss> {
        let Some(store_path) = self.store_path else {
            // No registry — pass through
            return Imperfect::Success(ResolvedAst(input.0));
        };

        let registry = match MirrorRegistry::open(&store_path) {
            Ok(r) => r,
            Err(e) => {
                return Imperfect::Failure(e, MirrorLoss::zero());
            }
        };

        match registry.resolve_fragment(&input.0) {
            Ok(()) => Imperfect::Success(ResolvedAst(input.0)),
            Err(e) => {
                // Resolution failed but the AST is still usable — record as loss
                let mut loss = MirrorLoss::zero();
                loss.resolution = ResolutionLoss {
                    unresolved_refs: vec![(
                        e.0.clone(),
                        crate::kernel::TraceOid::new("resolve"),
                    )],
                    resolution_ratio: 0.0,
                };
                Imperfect::Partial(ResolvedAst(input.0), loss)
            }
        }
    }
}

// -- Properties: ResolvedAst -> CheckedAst --
// Property checking pass-through for now.

impl LambdaFn for Properties {
    type Input = ResolvedAst;
    type Output = CheckedAst;
    type Error = MirrorRuntimeError;
    type Loss = MirrorLoss;

    fn reduce(
        self,
        input: ResolvedAst,
    ) -> Imperfect<CheckedAst, MirrorRuntimeError, MirrorLoss> {
        Imperfect::Success(CheckedAst(input.0))
    }
}

// -- Emit: CheckedAst -> EmittedCode --
// Emits Rust source code from the checked AST fragment.

impl LambdaFn for Emit {
    type Input = CheckedAst;
    type Output = EmittedCode;
    type Error = MirrorRuntimeError;
    type Loss = MirrorLoss;

    fn reduce(
        self,
        input: CheckedAst,
    ) -> Imperfect<EmittedCode, MirrorRuntimeError, MirrorLoss> {
        let grammar = CodeGrammar::rust();
        let rust_code = emit_code_fragment(&input.0, &grammar).to_string_lossy();
        Imperfect::Success(EmittedCode(rust_code))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use prism::oid::Addressable;
    use prism::Oid;

    // -------------------------------------------------------------------
    // Term-level composition tests (Composable<T>)
    // -------------------------------------------------------------------

    #[test]
    fn parse_is_composable() {
        let lambda: Lambda<String> = Parse.into();
        assert!(matches!(lambda, Lambda::Abs(_)));
    }

    #[test]
    fn parse_then_resolve() {
        let pipeline: Lambda<MirrorFragment> =
            Composable::<MirrorFragment>::then(Parse, Resolve::pass_through());
        assert!(matches!(pipeline, Lambda::Abs(_)));
        assert!(!pipeline.oid().is_dark());
    }

    #[test]
    fn craft_pipeline_composes_four_phases() {
        let craft: Lambda<MirrorFragment> =
            Composable::<MirrorFragment>::then(Parse, Resolve::pass_through())
                .then(Properties)
                .then(Emit);
        assert!(!craft.oid().is_dark());
    }

    #[test]
    fn same_composition_same_oid() {
        let a: Lambda<MirrorFragment> = Composable::<MirrorFragment>::then(Parse, Resolve::pass_through());
        let b: Lambda<MirrorFragment> = Composable::<MirrorFragment>::then(Parse, Resolve::pass_through());
        assert_eq!(a.oid(), b.oid());
    }

    #[test]
    fn different_composition_different_oid() {
        let a: Lambda<MirrorFragment> = Composable::<MirrorFragment>::then(Parse, Resolve::pass_through());
        let b: Lambda<MirrorFragment> = Composable::<MirrorFragment>::then(Parse, Emit);
        assert_ne!(a.oid(), b.oid());
    }

    #[test]
    fn named_lambda_display() {
        assert_eq!(format!("{}", Parse), "@parse");
        assert_eq!(format!("{}", Resolve::pass_through()), "@resolve");
    }

    #[test]
    fn craft_static_is_deterministic() {
        let craft_oid = CRAFT.oid();
        let manual: Lambda<MirrorFragment> =
            Composable::<MirrorFragment>::then(Parse, Resolve::pass_through())
                .then(Properties)
                .then(Emit);
        assert_eq!(craft_oid, manual.oid());
    }

    #[test]
    fn craft_and_kintsugi_differ() {
        assert_ne!(CRAFT.oid(), KINTSUGI_PIPELINE.oid());
    }

    #[test]
    fn craft_and_ci_differ() {
        assert_ne!(CRAFT.oid(), CI.oid());
    }

    #[test]
    fn ci_static_is_deterministic() {
        let ci_oid = CI.oid();
        let manual: Lambda<MirrorFragment> =
            Composable::<MirrorFragment>::then(Parse, Resolve::pass_through()).then(Properties);
        assert_eq!(ci_oid, manual.oid());
    }

    #[test]
    fn parse_oid_matches_manual() {
        assert_eq!(Parse.oid(), Oid::hash(b"@parse"));
    }

    #[test]
    fn resolve_oid_matches_manual() {
        assert_eq!(Resolve::pass_through().oid(), Oid::hash(b"@resolve"));
    }

    #[test]
    fn phase_oids_are_unique() {
        let oids = [
            Parse.oid(),
            Resolve::pass_through().oid(),
            Emit.oid(),
            Kintsugi.oid(),
            Strict.oid(),
            Properties.oid(),
        ];
        for (i, a) in oids.iter().enumerate() {
            for (j, b) in oids.iter().enumerate() {
                if i != j {
                    assert_ne!(a, b, "phases {} and {} have same oid", i, j);
                }
            }
        }
    }

    #[test]
    fn order_matters_for_pipeline() {
        let ab: Lambda<MirrorFragment> = Composable::<MirrorFragment>::then(Parse, Resolve::pass_through());
        let ba: Lambda<MirrorFragment> = Composable::<MirrorFragment>::then(Resolve::pass_through(), Parse);
        assert_ne!(ab.oid(), ba.oid());
    }

    // -------------------------------------------------------------------
    // Typed composition tests (LambdaFn)
    // -------------------------------------------------------------------

    #[test]
    fn parse_implements_lambda_fn() {
        let result = Parse.reduce(SourceText("type color = red | blue".into()));
        assert!(result.is_ok());
    }

    #[test]
    fn parse_then_resolve_typed() {
        let pipeline = LambdaFn::then(Parse, Resolve::pass_through());
        let result = pipeline.reduce(SourceText("type color = red | blue".into()));
        assert!(result.is_ok());
    }

    #[test]
    fn full_pipeline_types_check() {
        // The full pipeline from source to emitted code.
        // Type: Composed<Composed<Composed<Parse, Resolve>, Properties>, Emit>
        // Input = SourceText, Output = EmittedCode
        // Rust PROVES the chain at compile time.
        let craft = LambdaFn::then(
            LambdaFn::then(LambdaFn::then(Parse, Resolve::pass_through()), Properties),
            Emit,
        );
        let result = craft.reduce(SourceText("type color = red | blue".into()));
        assert!(result.is_ok());
    }

    #[test]
    fn loss_accumulates_through_typed_composition() {
        let pipeline = LambdaFn::then(Parse, Resolve::pass_through());
        let result = pipeline.reduce(SourceText("type x\nwidget y".into()));
        // Parse produces Partial (widget is unrecognized).
        // Loss should propagate through Resolve.
        assert!(
            result.is_partial(),
            "unrecognized 'widget' should produce Partial, got {:?}",
            if result.is_ok() {
                "Success"
            } else {
                "Failure"
            }
        );
    }

    #[test]
    fn typed_parse_partial_has_holonomy() {
        let result = Parse.reduce(SourceText("widget y\ntype x".into()));
        if result.is_partial() {
            match result {
                Imperfect::Partial(_, loss) => {
                    assert!(
                        loss.holonomy() > 0.0,
                        "parse loss should have non-zero holonomy"
                    );
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn resolve_pass_through_preserves_fragment() {
        // Get a real fragment from parse
        let parsed = Parse.reduce(SourceText("type x".into()));
        let fragment = parsed.ok().unwrap().0;
        let result = Resolve::pass_through().reduce(ParsedAst(fragment));
        assert!(result.is_ok());
    }

    #[test]
    fn properties_pass_through() {
        let parsed = Parse.reduce(SourceText("type x".into()));
        let fragment = parsed.ok().unwrap().0;
        let result = Properties.reduce(ResolvedAst(fragment));
        assert!(result.is_ok());
    }

    #[test]
    fn emit_produces_rust_code() {
        let parsed = Parse.reduce(SourceText("type x".into()));
        let fragment = parsed.ok().unwrap().0;
        let result = Emit.reduce(CheckedAst(fragment));
        assert!(result.is_ok());
        let code = result.ok().unwrap();
        assert!(
            code.0.contains("pub struct X;"),
            "emit should produce Rust struct, got: {}",
            code.0
        );
    }

    #[test]
    fn emit_produces_enum_for_variants() {
        let parsed = Parse.reduce(SourceText("type color = red | blue".into()));
        let fragment = parsed.ok().unwrap().0;
        let result = Emit.reduce(CheckedAst(fragment));
        assert!(result.is_ok());
        let code = result.ok().unwrap();
        assert!(code.0.contains("pub enum Color"), "should contain enum Color, got: {}", code.0);
        assert!(code.0.contains("Red"), "should contain Red variant, got: {}", code.0);
        assert!(code.0.contains("Blue"), "should contain Blue variant, got: {}", code.0);
    }

    // -------------------------------------------------------------------
    // End-to-end pipeline tests
    // -------------------------------------------------------------------

    #[test]
    fn craft_pipeline_compiles_real_source() {
        let craft = LambdaFn::then(
            LambdaFn::then(
                LambdaFn::then(Parse, Resolve::pass_through()),
                Properties,
            ),
            Emit,
        );
        let result = craft.reduce(SourceText("type color = red | blue".into()));
        assert!(result.is_ok(), "full pipeline must produce code");
        let code = result.ok().unwrap();
        assert!(code.0.contains("Color"), "emitted Rust must contain Color");
        assert!(code.0.contains("Red"), "emitted Rust must contain Red");
        assert!(code.0.contains("Blue"), "emitted Rust must contain Blue");
    }

    #[test]
    fn craft_pipeline_with_warnings_is_partial() {
        let craft = LambdaFn::then(
            LambdaFn::then(
                LambdaFn::then(Parse, Resolve::pass_through()),
                Properties,
            ),
            Emit,
        );
        let result = craft.reduce(SourceText("type color = red | blue\nwidget foo".into()));
        // "widget" is unrecognized — Parse produces Partial
        // Loss should propagate through the whole chain
        assert!(result.is_ok(), "pipeline should still produce output");
        let loss = result.loss();
        assert!(loss.holonomy() > 0.0, "unrecognized widget should produce loss");
    }

    #[test]
    fn craft_pipeline_matches_old_compiler() {
        use crate::mirror_runtime::MirrorRuntime;
        let source = "type color = red | blue";

        // Old path
        let runtime = MirrorRuntime::new();
        let old_result = runtime.compile_source(source);
        let old_fragment = old_result.ok().unwrap();
        let grammar = CodeGrammar::rust();
        let old_rust = emit_code_fragment(&old_fragment.fragment, &grammar).to_string_lossy();

        // New path
        let pipeline = LambdaFn::then(
            LambdaFn::then(
                LambdaFn::then(Parse, Resolve::pass_through()),
                Properties,
            ),
            Emit,
        );
        let new_result = pipeline.reduce(SourceText(source.into()));
        let new_rust = new_result.ok().unwrap();

        assert_eq!(old_rust, new_rust.0, "pipeline must produce same output as old compiler");
    }

    #[test]
    fn resolve_with_store_succeeds_for_simple_type() {
        // Simple types don't have `in @X` references, so resolve succeeds
        // even against an empty store.
        let tmp = std::env::temp_dir().join(format!("mirror-test-resolve-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&tmp);
        let parsed = Parse.reduce(SourceText("type x".into()));
        let fragment = parsed.ok().unwrap().0;
        let result = Resolve::with_store(&tmp).reduce(ParsedAst(fragment));
        assert!(result.is_ok(), "simple type should resolve against empty store");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn resolve_with_store_partial_for_unresolved_ref() {
        // `in @missing` should produce Partial (unresolved ref as loss)
        let tmp = std::env::temp_dir().join(format!("mirror-test-unresolved-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&tmp);
        let parsed = Parse.reduce(SourceText("in @missing\ntype x".into()));
        let fragment = parsed.ok().unwrap().0;
        let result = Resolve::with_store(&tmp).reduce(ParsedAst(fragment));
        assert!(
            result.is_partial(),
            "unresolved `in @missing` should produce Partial, got {:?}",
            if result.is_ok() { "Success" } else { "Failure" }
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
