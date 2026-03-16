//! Compile transformation trees to Erlang Abstract Format.
//!
//! Emits valid EAF that `compile:forms/1` can consume.
//! Output is ETF-encoded (Erlang External Term Format) bytes.

use eetf::{Atom, FixInteger, List, Term, Tuple};

use crate::resolve::{BranchAction, BranchPattern, OutputNode};
use crate::tree::Tree;

/// Emit Erlang Abstract Format from a transformation tree.
///
/// Returns ETF-encoded bytes (starting with version byte 131).
/// The encoded forms represent a valid Erlang module that
/// `compile:forms/1` can compile to bytecode.
pub fn emit_eaf(tree: &Tree<OutputNode>) -> Vec<u8> {
    let module_name = tree.data().name();
    let forms = build_forms(module_name, tree);
    let term = Term::from(List::from(forms));
    let mut buf = Vec::new();
    term.encode(&mut buf).expect("ETF encoding should not fail");
    buf
}

/// Build the three forms: module attribute, export attribute, tree/0 function.
fn build_forms(module_name: &str, tree: &Tree<OutputNode>) -> Vec<Term> {
    vec![
        // {attribute, 1, module, ModuleName}
        eaf_tuple(vec![
            eaf_atom("attribute"),
            eaf_int(1),
            eaf_atom("module"),
            eaf_atom(module_name),
        ]),
        // {attribute, 2, export, [{tree, 0}]}
        eaf_tuple(vec![
            eaf_atom("attribute"),
            eaf_int(2),
            eaf_atom("export"),
            eaf_list(vec![eaf_tuple(vec![eaf_atom("tree"), eaf_int(0)])]),
        ]),
        // {function, 3, tree, 0, [{clause, 3, [], [], [Body]}]}
        eaf_tuple(vec![
            eaf_atom("function"),
            eaf_int(3),
            eaf_atom("tree"),
            eaf_int(0),
            eaf_list(vec![eaf_tuple(vec![
                eaf_atom("clause"),
                eaf_int(3),
                eaf_list(vec![]), // no args
                eaf_list(vec![]), // no guards
                eaf_list(vec![emit_body_expr(tree, 4)]),
            ])]),
        ]),
    ]
}

/// Build the EAF expression for an OutputNode tree (the function body).
///
/// Group → `{group, <<"name">>, [children...]}`
/// Select → `{select, <<"output">>, <<"folder">>, <<"template">>}`
fn emit_body_expr(tree: &Tree<OutputNode>, line: i32) -> Term {
    match tree.data() {
        OutputNode::Group { ref name } => {
            let children: Vec<Term> = tree
                .children()
                .iter()
                .enumerate()
                .map(|(i, c)| emit_body_expr(c, line + i as i32 + 1))
                .collect();
            // {tuple, Line, [{atom, Line, group}, {bin, Line, ...}, ConsChildren]}
            eaf_tuple_expr(line, vec![
                eaf_tuple(vec![eaf_atom("atom"), eaf_int(line), eaf_atom("group")]),
                eaf_bin(line, name),
                eaf_cons_list(&children, line),
            ])
        }
        OutputNode::Select {
            ref output_name,
            ref folder_name,
            ref template_name,
        } => {
            // {tuple, Line, [{atom, Line, select}, {bin, ...}, {bin, ...}, {bin, ...}]}
            eaf_tuple_expr(line, vec![
                eaf_tuple(vec![eaf_atom("atom"), eaf_int(line), eaf_atom("select")]),
                eaf_bin(line, output_name),
                eaf_bin(line, folder_name),
                eaf_bin(line, template_name),
            ])
        }
        OutputNode::Branch {
            ref query,
            ref arms,
        } => {
            // {tuple, Line, [{atom, Line, branch}, {bin, Line, Query}, ConsArms]}
            let arm_terms: Vec<Term> = arms
                .iter()
                .enumerate()
                .map(|(i, arm)| emit_branch_arm(arm, line + i as i32 + 1))
                .collect();
            eaf_tuple_expr(line, vec![
                eaf_tuple(vec![eaf_atom("atom"), eaf_int(line), eaf_atom("branch")]),
                eaf_bin(line, query),
                eaf_cons_list(&arm_terms, line),
            ])
        }
    }
}

/// Emit EAF for a single branch arm.
///
/// `{tuple, Line, [{atom, Line, PatternType}, {bin, ...}, {atom, Line, Action}]}`
fn emit_branch_arm(arm: &crate::resolve::BranchArm, line: i32) -> Term {
    let (pat_atom, pat_value) = match &arm.pattern {
        BranchPattern::Literal(s) => ("literal", s.as_str()),
        BranchPattern::Wild => ("wild", ""),
    };
    let action_atom = match &arm.action {
        BranchAction::Pass => "pass",
        BranchAction::Exit => "exit",
        BranchAction::Expr(e) => e.as_str(),
    };
    eaf_tuple_expr(line, vec![
        eaf_tuple(vec![eaf_atom("atom"), eaf_int(line), eaf_atom(pat_atom)]),
        eaf_bin(line, pat_value),
        eaf_tuple(vec![eaf_atom("atom"), eaf_int(line), eaf_atom(action_atom)]),
    ])
}

/// EAF binary literal: `<<"text">>`.
///
/// Abstract format: `{bin, Line, [{bin_element, Line, {string, Line, Chars}, default, default}]}`
fn eaf_bin(line: i32, s: &str) -> Term {
    let chars: Vec<Term> = s.bytes().map(|b| eaf_int(b as i32)).collect();
    eaf_tuple(vec![
        eaf_atom("bin"),
        eaf_int(line),
        eaf_list(vec![eaf_tuple(vec![
            eaf_atom("bin_element"),
            eaf_int(line),
            eaf_tuple(vec![eaf_atom("string"), eaf_int(line), eaf_list(chars)]),
            eaf_atom("default"),
            eaf_atom("default"),
        ])]),
    ])
}

/// Build an EAF cons list from a vec of expressions.
///
/// `[A, B]` → `{cons, L, A, {cons, L, B, {nil, L}}}`
/// `[]` → `{nil, L}`
fn eaf_cons_list(elements: &[Term], line: i32) -> Term {
    let mut result = eaf_tuple(vec![eaf_atom("nil"), eaf_int(line)]);
    for elem in elements.iter().rev() {
        result = eaf_tuple(vec![eaf_atom("cons"), eaf_int(line), elem.clone(), result]);
    }
    result
}

// ---- ETF term constructors ----

fn eaf_atom(name: &str) -> Term {
    Term::from(Atom::from(name))
}

fn eaf_int(n: i32) -> Term {
    Term::from(FixInteger::from(n))
}

fn eaf_tuple(elements: Vec<Term>) -> Term {
    Term::from(Tuple::from(elements))
}

fn eaf_list(elements: Vec<Term>) -> Term {
    Term::from(List::from(elements))
}

/// `{tuple, Line, [Elements...]}` — an abstract-format tuple expression.
fn eaf_tuple_expr(line: i32, elements: Vec<Term>) -> Term {
    eaf_tuple(vec![eaf_atom("tuple"), eaf_int(line), eaf_list(elements)])
}
