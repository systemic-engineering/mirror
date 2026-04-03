//! Compile transformation trees to Erlang Abstract Format.
//!
//! Emits valid EAF that `compile:forms/1` can consume.
//! Output is ETF-encoded (Erlang External Term Format) bytes.

use eetf::{Atom, FixInteger, List, Term, Tuple};

use crate::ast::AstNode;
use crate::prism::Prism;
use crate::resolve::{BranchAction, BranchPattern, OutputNode, Visibility};

/// Emit Erlang Abstract Format from a transformation tree.
///
/// Returns ETF-encoded bytes (starting with version byte 131).
/// The encoded forms represent a valid Erlang module that
/// `compile:forms/1` can compile to bytecode.
pub fn emit_eaf(tree: &Prism<OutputNode>) -> Vec<u8> {
    let module_name = tree.data().name();
    let forms = build_forms(module_name, tree);
    let term = Term::from(List::from(forms));
    let mut buf = Vec::new();
    term.encode(&mut buf).expect("ETF encoding should not fail");
    buf
}

/// Build the three forms: module attribute, export attribute, tree/0 function.
fn build_forms(module_name: &str, tree: &Prism<OutputNode>) -> Vec<Term> {
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
fn emit_body_expr(tree: &Prism<OutputNode>, line: i32) -> Term {
    match tree.data() {
        OutputNode::Group { ref name } => {
            let children: Vec<Term> = tree
                .children()
                .iter()
                .enumerate()
                .map(|(i, c)| emit_body_expr(c, line + i as i32 + 1))
                .collect();
            // {tuple, Line, [{atom, Line, group}, {bin, Line, ...}, ConsChildren]}
            eaf_tuple_expr(
                line,
                vec![
                    eaf_tuple(vec![eaf_atom("atom"), eaf_int(line), eaf_atom("group")]),
                    eaf_bin(line, name),
                    eaf_cons_list(&children, line),
                ],
            )
        }
        OutputNode::Select {
            ref output_name,
            ref folder_name,
            ref template_name,
        } => {
            // {tuple, Line, [{atom, Line, select}, {bin, ...}, {bin, ...}, {bin, ...}]}
            eaf_tuple_expr(
                line,
                vec![
                    eaf_tuple(vec![eaf_atom("atom"), eaf_int(line), eaf_atom("select")]),
                    eaf_bin(line, output_name),
                    eaf_bin(line, folder_name),
                    eaf_bin(line, template_name),
                ],
            )
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
            eaf_tuple_expr(
                line,
                vec![
                    eaf_tuple(vec![eaf_atom("atom"), eaf_int(line), eaf_atom("branch")]),
                    eaf_bin(line, query),
                    eaf_cons_list(&arm_terms, line),
                ],
            )
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
    eaf_tuple_expr(
        line,
        vec![
            eaf_tuple(vec![eaf_atom("atom"), eaf_int(line), eaf_atom(pat_atom)]),
            eaf_bin(line, pat_value),
            eaf_tuple(vec![eaf_atom("atom"), eaf_int(line), eaf_atom(action_atom)]),
        ],
    )
}

/// Emit an actor dispatch module from a `Domain` model.
///
/// Each act declared in the domain becomes an exported function that
/// dispatches to the registered actor process via gen_server:call.
///
/// For `grammar @compiler { action compile { source: target } }`:
/// ```erlang
/// -module('conv_compiler').
/// -export([compile/1]).
/// compile(Args) -> gen_server:call('compiler', {compile, Args}).
/// ```
pub fn emit_actor_module_for_domain(
    domain: &crate::model::Domain,
    lenses: &[String],
    extends: &[String],
) -> Vec<u8> {
    let domain_name = domain.domain_name();
    let beam_module = format!("conv_{}", domain_name);
    let act_names = domain.act_names();

    let mut forms = Vec::new();

    // {attribute, 1, module, conv_Domain}
    forms.push(eaf_tuple(vec![
        eaf_atom("attribute"),
        eaf_int(1),
        eaf_atom("module"),
        eaf_atom(&beam_module),
    ]));

    // {attribute, 2, export, [{act1, 1}, ..., {lenses, 0}, {extends, 0}, {visibility, 0}]}
    // Private actions are NOT exported.
    let mut exports: Vec<Term> = act_names
        .iter()
        .filter(|name| domain.action_visibility(name) != Visibility::Private)
        .map(|name| eaf_tuple(vec![eaf_atom(name), eaf_int(1)]))
        .collect();
    exports.push(eaf_tuple(vec![eaf_atom("lenses"), eaf_int(0)]));
    exports.push(eaf_tuple(vec![eaf_atom("extends"), eaf_int(0)]));
    exports.push(eaf_tuple(vec![eaf_atom("visibility"), eaf_int(0)]));
    exports.push(eaf_tuple(vec![eaf_atom("requires"), eaf_int(0)]));
    exports.push(eaf_tuple(vec![eaf_atom("invariants"), eaf_int(0)]));
    exports.push(eaf_tuple(vec![eaf_atom("ensures"), eaf_int(0)]));
    forms.push(eaf_tuple(vec![
        eaf_atom("attribute"),
        eaf_int(2),
        eaf_atom("export"),
        eaf_list(exports),
    ]));

    // One function per act:
    // Body depends on visibility:
    //   public    → {ok, Args} (direct return, no gen_server)
    //   protected → gen_server:call('module', {action, Args})
    //   private   → gen_server:call('module', {action, Args}) (same body, not exported)
    let mut line = 3i32;
    for name in &act_names {
        let vis = domain.action_visibility(name);
        let calls = domain.action_calls(name);
        let mut calls_owned: Vec<(String, String, Vec<String>)> = Vec::new();
        for (d, a, args) in &calls {
            let mut owned_args = Vec::new();
            for s in args {
                owned_args.push(s.to_string());
            }
            calls_owned.push((d.to_string(), a.to_string(), owned_args));
        }
        forms.push(emit_act_function(
            domain_name,
            name,
            &vis,
            &calls_owned,
            line,
        ));
        line += 1;
    }

    // lenses/0 → [<<"domain1">>, <<"domain2">>, ...]
    forms.push(emit_string_list_function("lenses", lenses, line));
    line += 1;

    // extends/0 → [<<"parent1">>, <<"parent2">>, ...]
    forms.push(emit_string_list_function("extends", extends, line));
    line += 1;

    // visibility/0 → [{<<"action">>, <<"vis">>}, ...]
    forms.push(emit_visibility_function_from_domain(domain, line));
    line += 1;

    // requires/0 → [<<"shannon_equivalence">>, ...]
    let requires: Vec<String> = domain
        .required_properties()
        .iter()
        .map(|s| s.to_string())
        .collect();
    forms.push(emit_string_list_function("requires", &requires, line));
    line += 1;

    // invariants/0 → [<<"connected">>, ...]
    let invariants: Vec<String> = domain.invariants().iter().map(|s| s.to_string()).collect();
    forms.push(emit_string_list_function("invariants", &invariants, line));
    line += 1;

    // ensures/0 → [<<"response_time">>, ...]
    let ensures: Vec<String> = domain.ensures().iter().map(|s| s.to_string()).collect();
    forms.push(emit_string_list_function("ensures", &ensures, line));

    let term = Term::from(List::from(forms));
    let mut buf = Vec::new();
    term.encode(&mut buf).expect("ETF encoding should not fail");
    buf
}

/// Emit a single act dispatch function.
///
/// Visibility determines the body:
/// - **Public**: `name(Args) -> {ok, Args}.` (direct return, no gen_server)
/// - **Protected**: `name(Args) -> gen_server:call('module', {name, Args}).`
/// - **Private**: same body as protected (but not exported from module)
///
/// Cross-actor calls always use gen_server:call regardless of visibility.
fn emit_act_function(
    module: &str,
    act_name: &str,
    visibility: &Visibility,
    calls: &[(String, String, Vec<String>)],
    line: i32,
) -> Term {
    // The argument variable: {var, Line, 'Args'}
    let args_var = eaf_tuple(vec![eaf_atom("var"), eaf_int(line), eaf_atom("Args")]);

    let mut body = match visibility {
        Visibility::Public => {
            // Public: return {ok, Args} directly — no gen_server round-trip
            vec![eaf_tuple_expr(
                line,
                vec![
                    eaf_tuple(vec![eaf_atom("atom"), eaf_int(line), eaf_atom("ok")]),
                    args_var.clone(),
                ],
            )]
        }
        Visibility::Protected | Visibility::Private => {
            // Protected/Private: dispatch through gen_server:call
            vec![emit_gen_server_call(module, act_name, &args_var, line)]
        }
    };

    // Cross-actor calls always go through gen_server:call
    for (domain, action, _args) in calls {
        body.push(emit_gen_server_call(domain, action, &args_var, line));
    }

    // {function, Line, Name, 1, [{clause, Line, [ArgsVar], [], [Body...]}]}
    eaf_tuple(vec![
        eaf_atom("function"),
        eaf_int(line),
        eaf_atom(act_name),
        eaf_int(1),
        eaf_list(vec![eaf_tuple(vec![
            eaf_atom("clause"),
            eaf_int(line),
            eaf_list(vec![args_var]),
            eaf_list(vec![]), // no guards
            eaf_list(body),
        ])]),
    ])
}

/// Emit `visibility/0` function from a Domain: returns a list of `{<<"action">>, <<"vis">>}` tuples.
fn emit_visibility_function_from_domain(domain: &crate::model::Domain, line: i32) -> Term {
    let pairs: Vec<Term> = domain
        .act_names()
        .iter()
        .map(|name| {
            let vis = match domain.action_visibility(name) {
                Visibility::Public => "public",
                Visibility::Protected => "protected",
                Visibility::Private => "private",
            };
            eaf_tuple_expr(line, vec![eaf_bin(line, name), eaf_bin(line, vis)])
        })
        .collect();

    eaf_tuple(vec![
        eaf_atom("function"),
        eaf_int(line),
        eaf_atom("visibility"),
        eaf_int(0),
        eaf_list(vec![eaf_tuple(vec![
            eaf_atom("clause"),
            eaf_int(line),
            eaf_list(vec![]),
            eaf_list(vec![]),
            eaf_list(vec![if pairs.is_empty() {
                eaf_tuple(vec![eaf_atom("nil"), eaf_int(line)])
            } else {
                eaf_cons_list(&pairs, line)
            }]),
        ])]),
    ])
}

/// Emit `gen_server:call('module', {action, Args})`.
fn emit_gen_server_call(module: &str, action: &str, args_var: &Term, line: i32) -> Term {
    let dispatch_tuple = eaf_tuple_expr(
        line,
        vec![
            eaf_tuple(vec![eaf_atom("atom"), eaf_int(line), eaf_atom(action)]),
            args_var.clone(),
        ],
    );

    eaf_tuple(vec![
        eaf_atom("call"),
        eaf_int(line),
        eaf_tuple(vec![
            eaf_atom("remote"),
            eaf_int(line),
            eaf_tuple(vec![
                eaf_atom("atom"),
                eaf_int(line),
                eaf_atom("gen_server"),
            ]),
            eaf_tuple(vec![eaf_atom("atom"), eaf_int(line), eaf_atom("call")]),
        ]),
        eaf_list(vec![
            eaf_tuple(vec![eaf_atom("atom"), eaf_int(line), eaf_atom(module)]),
            dispatch_tuple,
        ]),
    ])
}

/// Emit a zero-arity function returning a list of binaries.
///
/// ```erlang
/// name() -> [<<"a">>, <<"b">>].
/// ```
fn emit_string_list_function(name: &str, values: &[String], line: i32) -> Term {
    let elements: Vec<Term> = values.iter().map(|v| eaf_bin(line, v)).collect();
    let body = eaf_cons_list(&elements, line);

    eaf_tuple(vec![
        eaf_atom("function"),
        eaf_int(line),
        eaf_atom(name),
        eaf_int(0),
        eaf_list(vec![eaf_tuple(vec![
            eaf_atom("clause"),
            eaf_int(line),
            eaf_list(vec![]), // no args
            eaf_list(vec![]), // no guards
            eaf_list(vec![body]),
        ])]),
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

/// Emit an actor dispatch module from a `Domain` model.
///
/// This is the primary Domain-based entry point for compilation.
/// Uses Domain query methods directly.
pub fn emit_actor_module_from_domain(domain: &crate::model::Domain) -> Vec<u8> {
    // Filter out self-lenses (e.g. @filesystem in a @filesystem grammar).
    let domain_name = domain.name.as_str();
    let mut lenses = Vec::new();
    for l in &domain.lenses {
        let target = l.target.as_str().to_owned();
        if target != domain_name {
            lenses.push(target);
        }
    }
    // Extends from domain model.
    let mut extends = Vec::new();
    for d in &domain.extends {
        extends.push(d.as_str().to_string());
    }
    emit_actor_module_for_domain(domain, &lenses, &extends)
}

/// Emit a test module from an `annotate(@test)` subtree.
///
/// Produces an Erlang module the BEAM can load:
/// ```erlang
/// -module('@test_domain').
/// -export([tests/0]).
/// tests() ->
///   [{test, <<"name">>, [<<"assertion1">>, ...]},
///    {property, <<"name">>, [<<"check1">>, ...]}].
/// ```
pub fn emit_test_module(domain: &str, annotate: &Prism<AstNode>) -> Vec<u8> {
    let module_name = format!("@test_{}", domain);

    let mut forms = Vec::new();

    // {attribute, 1, module, ModuleName}
    forms.push(eaf_tuple(vec![
        eaf_atom("attribute"),
        eaf_int(1),
        eaf_atom("module"),
        eaf_atom(&module_name),
    ]));

    // {attribute, 2, export, [{tests, 0}]}
    forms.push(eaf_tuple(vec![
        eaf_atom("attribute"),
        eaf_int(2),
        eaf_atom("export"),
        eaf_list(vec![eaf_tuple(vec![eaf_atom("tests"), eaf_int(0)])]),
    ]));

    // Build the tests/0 body: a list of test descriptors
    let descriptors: Vec<Term> = annotate
        .children()
        .iter()
        .enumerate()
        .map(|(i, child)| emit_test_descriptor(child, 4 + i as i32))
        .collect();

    // {function, 3, tests, 0, [{clause, 3, [], [], [Body]}]}
    forms.push(eaf_tuple(vec![
        eaf_atom("function"),
        eaf_int(3),
        eaf_atom("tests"),
        eaf_int(0),
        eaf_list(vec![eaf_tuple(vec![
            eaf_atom("clause"),
            eaf_int(3),
            eaf_list(vec![]),
            eaf_list(vec![]),
            eaf_list(vec![eaf_cons_list(&descriptors, 3)]),
        ])]),
    ]));

    let term = Term::from(List::from(forms));
    let mut buf = Vec::new();
    term.encode(&mut buf).expect("ETF encoding should not fail");
    buf
}

/// Emit an actor dispatch module from a `Verified` domain.
///
/// Accepts the proof wrapper from `check::verify()`, guaranteeing that the
/// domain passed all property checks before compilation.
pub fn emit_actor_module_from_verified(verified: &crate::check::Verified) -> Vec<u8> {
    emit_actor_module_from_domain(verified.domain())
}

/// Emit a single test descriptor from a Form child of annotate(@test).
///
/// `{tuple, Line, [{atom, Line, test}, {bin, Line, Name}, ConsAssertions]}`
fn emit_test_descriptor(child: &Prism<AstNode>, line: i32) -> Term {
    let data = child.data();
    let kind_atom = &data.name; // "test", "property", or "generate"
    let name = &data.value;

    let leaf_terms: Vec<Term> = child
        .children()
        .iter()
        .map(|leaf| eaf_bin(line, &leaf.data().value))
        .collect();

    eaf_tuple_expr(
        line,
        vec![
            eaf_tuple(vec![eaf_atom("atom"), eaf_int(line), eaf_atom(kind_atom)]),
            eaf_bin(line, name),
            eaf_cons_list(&leaf_terms, line),
        ],
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Domain;
    use crate::parse::Parse;
    use crate::Vector;

    fn parse_grammar_node(source: &str) -> crate::prism::Prism<crate::ast::AstNode> {
        let ast = Parse
            .trace(source.to_string())
            .into_result()
            .expect("parse should succeed");
        ast.children()
            .iter()
            .find(|c: &&crate::prism::Prism<crate::ast::AstNode>| c.data().is_decl("grammar"))
            .expect("grammar block should exist")
            .clone()
    }

    #[test]
    fn emit_actor_module_from_domain_produces_valid_etf() {
        let source = "grammar @test {\n  type = a | b\n  action ping(target: a)\n}\n";
        let grammar_node = parse_grammar_node(source);
        let domain = Domain::from_grammar(&grammar_node).unwrap();

        let etf = emit_actor_module_from_domain(&domain);

        assert!(!etf.is_empty());
        assert_eq!(etf[0], 131); // ETF version byte
                                 // Should contain conv_test module name
        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let s = format!("{:?}", term);
        assert!(
            s.contains("conv_test"),
            "should have conv_test module: {}",
            s
        );
    }

    #[test]
    fn emit_actor_module_from_domain_deterministic() {
        let source = "grammar @test {\n  type = a | b\n  action ping(target: a)\n}\n";
        let grammar_node = parse_grammar_node(source);
        let domain = Domain::from_grammar(&grammar_node).unwrap();

        let etf1 = emit_actor_module_from_domain(&domain);
        let etf2 = emit_actor_module_from_domain(&domain);

        // Same domain → same output (deterministic).
        assert_eq!(etf1, etf2);
    }

    #[test]
    fn emit_actor_module_from_domain_includes_lenses() {
        let source = "grammar @fs {\n  type = file | folder\n}\n";
        let grammar_node = parse_grammar_node(source);
        let lenses = vec!["@reality".to_string()];
        let domain = Domain::from_grammar_with_lenses(&grammar_node, &lenses).unwrap();

        let etf = emit_actor_module_from_domain(&domain);
        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let s = format!("{:?}", term);

        let reality_bytes: Vec<u8> = "reality".bytes().collect();
        assert!(
            s.contains(&format!("{:?}", reality_bytes)),
            "should contain reality lens: {}",
            s
        );
    }

    #[test]
    fn emit_actor_module_from_verified_produces_valid_etf() {
        let source = "grammar @clean {\n  type = x | y\n}\n";
        let grammar_node = parse_grammar_node(source);
        let domain = Domain::from_grammar(&grammar_node).unwrap();
        let verified = crate::check::verify(domain).unwrap();

        let etf = emit_actor_module_from_verified(&verified);

        assert!(!etf.is_empty());
        assert_eq!(etf[0], 131);
        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let s = format!("{:?}", term);
        assert!(
            s.contains("conv_clean"),
            "should have conv_clean module: {}",
            s
        );
    }
}
