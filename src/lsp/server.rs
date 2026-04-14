//! LSP server — pure mapping functions from MirrorLoss to diagnostics.
//!
//! No external LSP dependencies. These are the pure functions that map
//! mirror's domain types (MirrorLoss, Convergence, PropertyVerdict) into
//! protocol-shaped structs (MirrorDiagnostic, CompletionItem).
//!
//! The tower-lsp adapter (Phase 3 Task 3.5) will wrap these.

use crate::loss::MirrorLoss;
use crate::shatter_format::Luminosity;

// ---------------------------------------------------------------------------
// DiagnosticSeverity
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
}

// ---------------------------------------------------------------------------
// MirrorDiagnostic
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct MirrorDiagnostic {
    /// 0-indexed line number.
    pub line: usize,
    /// 0-indexed column (start).
    pub col: usize,
    /// 0-indexed column (end, exclusive).
    pub end_col: usize,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub code: Option<String>,
}

// ---------------------------------------------------------------------------
// loss_to_diagnostics — STUB (red)
// ---------------------------------------------------------------------------

pub fn loss_to_diagnostics(_loss: &MirrorLoss) -> Vec<MirrorDiagnostic> {
    Vec::new() // intentionally empty — tests will fail
}

// ---------------------------------------------------------------------------
// CompletionItem / CompletionKind — STUB (red)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct CompletionItem {
    pub label: String,
    pub detail: String,
    pub kind: CompletionKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompletionKind {
    Keyword,
    Operator,
}

pub fn mirror_completion_items() -> Vec<CompletionItem> {
    Vec::new() // intentionally empty — tests will fail
}

// ---------------------------------------------------------------------------
// MirrorLspBackend — STUB (red)
// ---------------------------------------------------------------------------

pub struct MirrorLspBackend {
    pub shatter_cache: std::collections::HashMap<String, crate::shatter_format::ShatterMeta>,
    runtime: crate::mirror_runtime::MirrorRuntime,
}

impl MirrorLspBackend {
    pub fn new() -> Self {
        MirrorLspBackend {
            shatter_cache: std::collections::HashMap::new(),
            runtime: crate::mirror_runtime::MirrorRuntime::new(),
        }
    }

    pub fn compile_and_diagnose(&self, _source: &str) -> (Luminosity, Vec<MirrorDiagnostic>) {
        (Luminosity::Dark, Vec::new()) // intentionally wrong — tests will fail
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::TraceOid;
    use crate::loss::{Convergence, PropertyVerdict, UnrecognizedDecl};
    use prism::{Imperfect, Loss};

    // -- loss_to_diagnostics --

    #[test]
    fn loss_to_diagnostics_empty_for_zero_loss() {
        let loss = MirrorLoss::zero();
        let diags = loss_to_diagnostics(&loss);
        assert!(diags.is_empty());
    }

    #[test]
    fn loss_to_diagnostics_warning_for_unrecognized() {
        let mut loss = MirrorLoss::zero();
        loss.parse.unrecognized.push(UnrecognizedDecl {
            keyword: "widget".into(),
            line: 5,
            content: "foo".into(),
        });
        let diags = loss_to_diagnostics(&loss);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Warning);
        assert!(diags[0].message.contains("widget"));
        assert_eq!(diags[0].line, 4); // 0-indexed
        assert_eq!(diags[0].code.as_deref(), Some("M1001"));
    }

    #[test]
    fn loss_to_diagnostics_error_for_unresolved() {
        let mut loss = MirrorLoss::zero();
        loss.resolution
            .unresolved_refs
            .push(("@missing".into(), TraceOid::new("t")));
        let diags = loss_to_diagnostics(&loss);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Error);
        assert!(diags[0].message.contains("@missing"));
        assert_eq!(diags[0].code.as_deref(), Some("M3001"));
    }

    #[test]
    fn loss_to_diagnostics_budget_exhausted() {
        let loss = MirrorLoss::total();
        let diags = loss_to_diagnostics(&loss);
        assert!(diags
            .iter()
            .any(|d| d.severity == DiagnosticSeverity::Error));
        assert!(diags.iter().any(|d| d.message.contains("budget")));
    }

    #[test]
    fn loss_to_diagnostics_property_partial() {
        let mut loss = MirrorLoss::zero();
        loss.properties.verdicts.push(PropertyVerdict {
            property: "reachability".into(),
            verdict: Imperfect::Partial((), 0.5),
        });
        let diags = loss_to_diagnostics(&loss);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Warning);
        assert_eq!(diags[0].code.as_deref(), Some("M4001"));
    }

    #[test]
    fn loss_to_diagnostics_property_failure() {
        let mut loss = MirrorLoss::zero();
        loss.properties.verdicts.push(PropertyVerdict {
            property: "unique_variants".into(),
            verdict: Imperfect::Failure("duplicate found".into(), 1.0),
        });
        let diags = loss_to_diagnostics(&loss);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Error);
        assert_eq!(diags[0].code.as_deref(), Some("M4002"));
    }

    #[test]
    fn loss_to_diagnostics_oscillating() {
        let mut loss = MirrorLoss::zero();
        loss.convergence = Convergence::Oscillating(3);
        let diags = loss_to_diagnostics(&loss);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Warning);
        assert_eq!(diags[0].code.as_deref(), Some("M9003"));
        assert!(diags[0].message.contains("3"));
    }

    #[test]
    fn loss_to_diagnostics_skips_success_verdicts() {
        let mut loss = MirrorLoss::zero();
        loss.properties.verdicts.push(PropertyVerdict {
            property: "ok_property".into(),
            verdict: Imperfect::Success(()),
        });
        let diags = loss_to_diagnostics(&loss);
        assert!(diags.is_empty());
    }

    #[test]
    fn loss_to_diagnostics_skips_converging_and_settled() {
        let mut loss = MirrorLoss::zero();
        loss.convergence = Convergence::Converging(5);
        assert!(loss_to_diagnostics(&loss).is_empty());

        loss.convergence = Convergence::Settled;
        assert!(loss_to_diagnostics(&loss).is_empty());
    }

    #[test]
    fn loss_to_diagnostics_multiple_sources() {
        let mut loss = MirrorLoss::zero();
        loss.parse.unrecognized.push(UnrecognizedDecl {
            keyword: "widget".into(),
            line: 1,
            content: "x".into(),
        });
        loss.resolution
            .unresolved_refs
            .push(("@missing".into(), TraceOid::new("t")));
        loss.convergence = Convergence::BudgetExhausted;
        let diags = loss_to_diagnostics(&loss);
        // 1 unrecognized + 1 unresolved + 1 budget = 3
        assert_eq!(diags.len(), 3);
    }

    // -- completion items --

    #[test]
    fn completion_items_include_keywords() {
        let items = mirror_completion_items();
        assert!(items.iter().any(|i| i.label == "grammar"));
        assert!(items.iter().any(|i| i.label == "type"));
        assert!(items.iter().any(|i| i.label == "action"));
        assert!(items.iter().any(|i| i.label == "property"));
    }

    #[test]
    fn completion_items_include_operators() {
        let items = mirror_completion_items();
        assert!(items.iter().any(|i| i.label == "<="));
        assert!(items.iter().any(|i| i.label == "="));
        assert!(items.iter().any(|i| i.label == "!="));
    }

    #[test]
    fn completion_items_all_keywords_are_keyword_kind() {
        let items = mirror_completion_items();
        for item in items.iter().filter(|i| i.kind == CompletionKind::Keyword) {
            assert!(
                item.detail.contains("keyword"),
                "keyword item '{}' missing keyword detail",
                item.label
            );
        }
    }

    #[test]
    fn completion_items_all_operators_are_operator_kind() {
        let items = mirror_completion_items();
        for item in items.iter().filter(|i| i.kind == CompletionKind::Operator) {
            assert!(
                item.detail.contains("operator"),
                "operator item '{}' missing operator detail",
                item.label
            );
        }
    }

    #[test]
    fn completion_items_include_all_decl_keywords() {
        let items = mirror_completion_items();
        let expected = [
            "form",
            "type",
            "prism",
            "in",
            "out",
            "property",
            "fold",
            "requires",
            "invariant",
            "ensures",
            "focus",
            "project",
            "split",
            "zoom",
            "refract",
            "traversal",
            "lens",
            "action",
            "recover",
            "rescue",
            "grammar",
            "default",
            "binding",
        ];
        for kw in expected {
            assert!(
                items.iter().any(|i| i.label == kw),
                "missing keyword: {}",
                kw
            );
        }
    }

    // -- MirrorLspBackend --

    #[test]
    fn backend_compile_and_diagnose_clean() {
        let backend = MirrorLspBackend::new();
        let (lum, diags) = backend.compile_and_diagnose("type color = red | blue");
        assert_eq!(lum, Luminosity::Light);
        assert!(diags.is_empty());
    }

    #[test]
    fn backend_compile_and_diagnose_with_warning() {
        let backend = MirrorLspBackend::new();
        let (lum, diags) = backend.compile_and_diagnose("type color = red | blue\nwidget foo");
        assert_eq!(lum, Luminosity::Dimmed);
        assert!(!diags.is_empty());
        assert!(diags[0].message.contains("widget"));
    }

    #[test]
    fn backend_new_has_empty_cache() {
        let backend = MirrorLspBackend::new();
        assert!(backend.shatter_cache.is_empty());
    }

    // -- DiagnosticSeverity --

    #[test]
    fn diagnostic_severity_clone_eq() {
        let s = DiagnosticSeverity::Error;
        assert_eq!(s.clone(), DiagnosticSeverity::Error);
        assert_ne!(DiagnosticSeverity::Error, DiagnosticSeverity::Warning);
        assert_ne!(DiagnosticSeverity::Warning, DiagnosticSeverity::Info);
    }

    // -- MirrorDiagnostic --

    #[test]
    fn mirror_diagnostic_clone() {
        let d = MirrorDiagnostic {
            line: 1,
            col: 2,
            end_col: 5,
            severity: DiagnosticSeverity::Warning,
            message: "test".into(),
            code: Some("M0001".into()),
        };
        let d2 = d.clone();
        assert_eq!(d2.line, 1);
        assert_eq!(d2.col, 2);
        assert_eq!(d2.end_col, 5);
        assert_eq!(d2.severity, DiagnosticSeverity::Warning);
        assert_eq!(d2.message, "test");
        assert_eq!(d2.code.as_deref(), Some("M0001"));
    }

    // -- CompletionItem / CompletionKind --

    #[test]
    fn completion_kind_clone_eq() {
        let k = CompletionKind::Keyword;
        assert_eq!(k.clone(), CompletionKind::Keyword);
        assert_ne!(CompletionKind::Keyword, CompletionKind::Operator);
    }

    #[test]
    fn completion_item_clone() {
        let item = CompletionItem {
            label: "type".into(),
            detail: "type keyword".into(),
            kind: CompletionKind::Keyword,
        };
        let item2 = item.clone();
        assert_eq!(item2.label, "type");
    }

    #[test]
    fn unrecognized_at_line_one_gives_line_zero() {
        let mut loss = MirrorLoss::zero();
        loss.parse.unrecognized.push(UnrecognizedDecl {
            keyword: "foo".into(),
            line: 1,
            content: "bar".into(),
        });
        let diags = loss_to_diagnostics(&loss);
        assert_eq!(diags[0].line, 0);
    }

    #[test]
    fn unrecognized_at_line_zero_saturates() {
        let mut loss = MirrorLoss::zero();
        loss.parse.unrecognized.push(UnrecognizedDecl {
            keyword: "foo".into(),
            line: 0,
            content: "bar".into(),
        });
        let diags = loss_to_diagnostics(&loss);
        assert_eq!(diags[0].line, 0); // saturating_sub(1) on 0 = 0
    }
}
