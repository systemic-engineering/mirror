//! MirrorOptic — a compiled grammar loaded as an executable optic.
//!
//! Extracts named actions from a `CompiledShatter` and provides
//! lookup by name. The crystal OID ties this optic to its content-addressed
//! origin. Phase 1 of the CLI bootstrap: load the grammar, list its actions.
//! Dispatch (invoke) comes in Phase 3.

use std::collections::BTreeMap;

use crate::declaration::{DeclKind, MirrorHash};
use crate::mirror_runtime::{CompiledShatter, Form, MirrorRuntimeError};

// ---------------------------------------------------------------------------
// ActionDef — one action extracted from the grammar
// ---------------------------------------------------------------------------

/// A single action defined in a `.mirror` grammar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionDef {
    /// The action's name.
    pub name: String,
    /// The receiver parameter (first param of the action).
    pub receiver: String,
    /// Grammar reference, e.g. `@code/rust`.
    pub grammar_ref: Option<String>,
    /// The raw body text, if present.
    pub body: Option<String>,
    /// Whether this action is abstract (no body).
    pub is_abstract: bool,
}

// ---------------------------------------------------------------------------
// MirrorOptic — the loaded grammar as executable
// ---------------------------------------------------------------------------

/// A compiled grammar loaded from the store, with named actions and a crystal OID.
///
/// Built from a `CompiledShatter`. Walks the form tree to extract all
/// `DeclKind::Action` children into a `BTreeMap` for O(log n) lookup.
#[derive(Clone, Debug)]
pub struct MirrorOptic {
    grammar_name: String,
    actions: BTreeMap<String, ActionDef>,
    crystal_oid: MirrorHash,
}

impl MirrorOptic {
    /// Build from a `CompiledShatter` — extract actions from the form tree.
    pub fn from_compiled(compiled: &CompiledShatter) -> Result<Self, MirrorRuntimeError> {
        let mut actions = BTreeMap::new();
        Self::collect_actions(&compiled.form, &mut actions);

        Ok(MirrorOptic {
            grammar_name: compiled.form_name().to_string(),
            actions,
            crystal_oid: compiled.crystal().clone(),
        })
    }

    /// Recursively walk the form tree and collect all Action declarations.
    fn collect_actions(form: &Form, actions: &mut BTreeMap<String, ActionDef>) {
        for child in &form.children {
            if child.kind == DeclKind::Action {
                let receiver = child.params.first().cloned().unwrap_or_default();
                let def = ActionDef {
                    name: child.name.clone(),
                    receiver,
                    grammar_ref: child.grammar_ref.clone(),
                    body: child.body_text.clone(),
                    is_abstract: child.is_abstract,
                };
                actions.insert(child.name.clone(), def);
            }
            // Recurse into nested forms to find actions at any depth.
            Self::collect_actions(child, actions);
        }
    }

    /// List available actions.
    pub fn actions(&self) -> &BTreeMap<String, ActionDef> {
        &self.actions
    }

    /// Check if an action exists.
    pub fn has_action(&self, name: &str) -> bool {
        self.actions.contains_key(name)
    }

    /// Get the crystal OID this optic was loaded from.
    pub fn crystal_oid(&self) -> &MirrorHash {
        &self.crystal_oid
    }

    /// Get the grammar name.
    pub fn grammar_name(&self) -> &str {
        &self.grammar_name
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mirror_runtime::MirrorRuntime;

    /// Helper: compile a .mirror source with action declarations.
    fn compile_with_actions() -> CompiledShatter {
        let source = r#"
form @cli {
    action focus(self) {
        parse_and_print()
    }
    action compile(self) in @code/rust {
        compile_source()
    }
    abstract action repl(self)
}
"#;
        let rt = MirrorRuntime::new();
        rt.compile_source(source).ok().expect("compile_with_actions")
    }

    #[test]
    fn from_compiled_extracts_actions() {
        let compiled = compile_with_actions();
        let optic = MirrorOptic::from_compiled(&compiled).unwrap();
        assert_eq!(
            optic.actions().len(),
            3,
            "expected exactly 3 actions (focus, compile, repl), got {:?}",
            optic.actions().keys().collect::<Vec<_>>()
        );
        assert!(optic.actions().contains_key("focus"));
        assert!(optic.actions().contains_key("compile"));
        assert!(optic.actions().contains_key("repl"));
    }

    #[test]
    fn has_action_returns_true_for_existing() {
        let compiled = compile_with_actions();
        let optic = MirrorOptic::from_compiled(&compiled).unwrap();
        assert!(optic.has_action("focus"));
        // Also verify grammar_name is correctly extracted
        assert_eq!(optic.grammar_name(), "@cli");
    }

    #[test]
    fn has_action_returns_false_for_missing() {
        let compiled = compile_with_actions();
        let optic = MirrorOptic::from_compiled(&compiled).unwrap();
        assert!(!optic.has_action("nonexistent"));
    }

    #[test]
    fn crystal_oid_matches_compiled() {
        let compiled = compile_with_actions();
        let optic = MirrorOptic::from_compiled(&compiled).unwrap();
        assert_eq!(optic.crystal_oid(), compiled.crystal());
    }

    #[test]
    fn action_preserves_grammar_ref_and_receiver() {
        let compiled = compile_with_actions();
        let optic = MirrorOptic::from_compiled(&compiled).unwrap();
        let compile_action = optic.actions().get("compile").expect("compile action");
        assert_eq!(compile_action.grammar_ref.as_deref(), Some("@code/rust"));
        assert_eq!(compile_action.receiver, "self", "receiver should be 'self'");
        assert!(
            !compile_action.is_abstract,
            "compile should not be abstract"
        );
    }

    #[test]
    fn action_preserves_body_content() {
        let compiled = compile_with_actions();
        let optic = MirrorOptic::from_compiled(&compiled).unwrap();
        let focus_action = optic.actions().get("focus").expect("focus action");
        let body = focus_action
            .body
            .as_ref()
            .expect("focus action should have a body");
        assert!(
            body.contains("parse_and_print"),
            "body should contain 'parse_and_print', got: {}",
            body
        );
    }

    #[test]
    fn abstract_action_has_no_body() {
        let compiled = compile_with_actions();
        let optic = MirrorOptic::from_compiled(&compiled).unwrap();
        let repl_action = optic.actions().get("repl").expect("repl action");
        assert!(
            repl_action.body.is_none(),
            "abstract action should have no body"
        );
        assert!(
            repl_action.is_abstract,
            "abstract action should be marked abstract"
        );
    }
}
