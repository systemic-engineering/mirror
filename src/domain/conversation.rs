//! The conversation domain. The AST's own vocabulary.
//!
//! A .conv file parsed into a tree is a tree in this domain.
//! The crate describes itself.

use super::Setting;

/// The conversation context.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Script;

/// What a conversation node can be.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    /// `in @domain`
    In,
    /// `out name { ... }`
    Out,
    /// `template $name { ... }`
    Template,
    /// `slug`, `excerpt`
    Field,
    /// `h2`, `article`
    Qualifier,
    /// `| @html`
    Pipe,
    /// `pieces { ... }`
    Group,
    /// `draft: 1draft { $corpus }`
    Select,
    /// `$corpus`
    TemplateRef,
    /// `@filesystem`
    DomainRef,
    /// `@git(branch: "master") | HEAD | @git(branch: "test")`
    Pipeline,
    /// `branch: "master"` — key-value parameter on a domain ref
    DomainParam,
    /// `HEAD` — a bare reference in a pipeline
    Ref,
    /// `$a` in `in @number as $a` — binding alias
    Alias,
    /// `$a + $b` — expression in output block
    Expr,
    /// `use $name from @domain` — import statement
    Use,
    /// `when error.rate > 0.1` — guard clause; the Op IS the comparison
    When(Op),
    /// `error.rate` — dot-separated path (Script space navigation)
    Path,
    /// `0.1`, `"health"`, `true` — literal value in a predicate
    Literal,
    /// `case error.rate { ... }` — multi-arm dispatch. value = subject path.
    Case,
    /// One arm in a case block. Children: [pattern, Expr].
    Arm,
    /// `> 0.1`, `== "active"` — comparison pattern in a case arm. value = literal.
    Cmp(Op),
    /// `_` — wildcard pattern. Matches anything.
    Wild,
}

/// Comparison operator — shared by `When` guards and `Cmp` case arm patterns.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Op {
    Gt,  // >
    Lt,  // <
    Gte, // >=
    Lte, // <=
    Eq,  // ==
    Ne,  // !=
}

impl Setting for Script {
    type Token = crate::ast::AstNode;

    fn id() -> &'static str {
        "conversation"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversation_id() {
        assert_eq!(Script::id(), "conversation");
    }

    #[test]
    fn conversation_is_scene() {
        fn requires_scene<C: Setting>() -> &'static str {
            C::id()
        }
        assert_eq!(requires_scene::<Script>(), "conversation");
    }
}
