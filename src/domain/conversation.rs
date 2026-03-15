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
    /// `$HOME` — root node reference (tree root)
    Home,
    /// `$SELF` — current node reference (where you are in the tree)
    Self_,
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
    /// `branch(.path) { "value" => action }` — value dispatch on a path.
    Branch,
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

impl Op {
    /// CSS-addressable name for this operator.
    pub fn local_name(&self) -> &'static str {
        match self {
            Op::Gt => "gt",
            Op::Lt => "lt",
            Op::Gte => "gte",
            Op::Lte => "lte",
            Op::Eq => "eq",
            Op::Ne => "ne",
        }
    }
}

impl Kind {
    /// CSS-addressable name for this AST node kind.
    ///
    /// Source of truth for gestalt's `Domain::local_name`. Gestalt delegates here.
    pub fn local_name(&self) -> &'static str {
        match self {
            Kind::In => "in",
            Kind::Out => "out",
            Kind::Template => "template",
            Kind::Field => "field",
            Kind::Qualifier => "qualifier",
            Kind::Pipe => "pipe",
            Kind::Group => "group",
            Kind::Select => "select",
            Kind::TemplateRef => "template-ref",
            Kind::DomainRef => "domain-ref",
            Kind::Pipeline => "pipeline",
            Kind::DomainParam => "domain-param",
            Kind::Ref => "ref",
            Kind::Alias => "alias",
            Kind::Expr => "expr",
            Kind::Use => "use",
            Kind::Home => "home",
            Kind::Self_ => "self",
            Kind::Path => "path",
            Kind::Literal => "literal",
            Kind::Case => "case",
            Kind::Arm => "arm",
            Kind::Wild => "wild",
            Kind::Branch => "branch",
            Kind::When(Op::Gt) => "when/gt",
            Kind::When(Op::Lt) => "when/lt",
            Kind::When(Op::Gte) => "when/gte",
            Kind::When(Op::Lte) => "when/lte",
            Kind::When(Op::Eq) => "when/eq",
            Kind::When(Op::Ne) => "when/ne",
            Kind::Cmp(Op::Gt) => "cmp/gt",
            Kind::Cmp(Op::Lt) => "cmp/lt",
            Kind::Cmp(Op::Gte) => "cmp/gte",
            Kind::Cmp(Op::Lte) => "cmp/lte",
            Kind::Cmp(Op::Eq) => "cmp/eq",
            Kind::Cmp(Op::Ne) => "cmp/ne",
        }
    }
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

    // -- Op::local_name --

    #[test]
    fn op_local_name_gt() {
        assert_eq!(Op::Gt.local_name(), "gt");
    }

    #[test]
    fn op_local_name_lt() {
        assert_eq!(Op::Lt.local_name(), "lt");
    }

    #[test]
    fn op_local_name_gte() {
        assert_eq!(Op::Gte.local_name(), "gte");
    }

    #[test]
    fn op_local_name_lte() {
        assert_eq!(Op::Lte.local_name(), "lte");
    }

    #[test]
    fn op_local_name_eq() {
        assert_eq!(Op::Eq.local_name(), "eq");
    }

    #[test]
    fn op_local_name_ne() {
        assert_eq!(Op::Ne.local_name(), "ne");
    }

    // -- Kind::local_name: simple variants --

    #[test]
    fn kind_local_name_in() {
        assert_eq!(Kind::In.local_name(), "in");
    }

    #[test]
    fn kind_local_name_out() {
        assert_eq!(Kind::Out.local_name(), "out");
    }

    #[test]
    fn kind_local_name_template() {
        assert_eq!(Kind::Template.local_name(), "template");
    }

    #[test]
    fn kind_local_name_field() {
        assert_eq!(Kind::Field.local_name(), "field");
    }

    #[test]
    fn kind_local_name_qualifier() {
        assert_eq!(Kind::Qualifier.local_name(), "qualifier");
    }

    #[test]
    fn kind_local_name_pipe() {
        assert_eq!(Kind::Pipe.local_name(), "pipe");
    }

    #[test]
    fn kind_local_name_group() {
        assert_eq!(Kind::Group.local_name(), "group");
    }

    #[test]
    fn kind_local_name_select() {
        assert_eq!(Kind::Select.local_name(), "select");
    }

    #[test]
    fn kind_local_name_template_ref() {
        assert_eq!(Kind::TemplateRef.local_name(), "template-ref");
    }

    #[test]
    fn kind_local_name_domain_ref() {
        assert_eq!(Kind::DomainRef.local_name(), "domain-ref");
    }

    #[test]
    fn kind_local_name_pipeline() {
        assert_eq!(Kind::Pipeline.local_name(), "pipeline");
    }

    #[test]
    fn kind_local_name_domain_param() {
        assert_eq!(Kind::DomainParam.local_name(), "domain-param");
    }

    #[test]
    fn kind_local_name_ref() {
        assert_eq!(Kind::Ref.local_name(), "ref");
    }

    #[test]
    fn kind_local_name_alias() {
        assert_eq!(Kind::Alias.local_name(), "alias");
    }

    #[test]
    fn kind_local_name_expr() {
        assert_eq!(Kind::Expr.local_name(), "expr");
    }

    #[test]
    fn kind_local_name_use() {
        assert_eq!(Kind::Use.local_name(), "use");
    }

    #[test]
    fn kind_local_name_home() {
        assert_eq!(Kind::Home.local_name(), "home");
    }

    #[test]
    fn kind_local_name_self() {
        assert_eq!(Kind::Self_.local_name(), "self");
    }

    #[test]
    fn kind_local_name_path() {
        assert_eq!(Kind::Path.local_name(), "path");
    }

    #[test]
    fn kind_local_name_literal() {
        assert_eq!(Kind::Literal.local_name(), "literal");
    }

    #[test]
    fn kind_local_name_case() {
        assert_eq!(Kind::Case.local_name(), "case");
    }

    #[test]
    fn kind_local_name_arm() {
        assert_eq!(Kind::Arm.local_name(), "arm");
    }

    #[test]
    fn kind_local_name_wild() {
        assert_eq!(Kind::Wild.local_name(), "wild");
    }

    #[test]
    fn kind_local_name_branch() {
        assert_eq!(Kind::Branch.local_name(), "branch");
    }

    // -- Kind::local_name: When(Op) --

    #[test]
    fn kind_local_name_when_gt() {
        assert_eq!(Kind::When(Op::Gt).local_name(), "when/gt");
    }

    #[test]
    fn kind_local_name_when_lt() {
        assert_eq!(Kind::When(Op::Lt).local_name(), "when/lt");
    }

    #[test]
    fn kind_local_name_when_gte() {
        assert_eq!(Kind::When(Op::Gte).local_name(), "when/gte");
    }

    #[test]
    fn kind_local_name_when_lte() {
        assert_eq!(Kind::When(Op::Lte).local_name(), "when/lte");
    }

    #[test]
    fn kind_local_name_when_eq() {
        assert_eq!(Kind::When(Op::Eq).local_name(), "when/eq");
    }

    #[test]
    fn kind_local_name_when_ne() {
        assert_eq!(Kind::When(Op::Ne).local_name(), "when/ne");
    }

    // -- Kind::local_name: Cmp(Op) --

    #[test]
    fn kind_local_name_cmp_gt() {
        assert_eq!(Kind::Cmp(Op::Gt).local_name(), "cmp/gt");
    }

    #[test]
    fn kind_local_name_cmp_lt() {
        assert_eq!(Kind::Cmp(Op::Lt).local_name(), "cmp/lt");
    }

    #[test]
    fn kind_local_name_cmp_gte() {
        assert_eq!(Kind::Cmp(Op::Gte).local_name(), "cmp/gte");
    }

    #[test]
    fn kind_local_name_cmp_lte() {
        assert_eq!(Kind::Cmp(Op::Lte).local_name(), "cmp/lte");
    }

    #[test]
    fn kind_local_name_cmp_eq() {
        assert_eq!(Kind::Cmp(Op::Eq).local_name(), "cmp/eq");
    }

    #[test]
    fn kind_local_name_cmp_ne() {
        assert_eq!(Kind::Cmp(Op::Ne).local_name(), "cmp/ne");
    }
}
