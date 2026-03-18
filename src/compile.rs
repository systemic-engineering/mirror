//! Compile transformation trees to Erlang Abstract Format.
//!
//! Emits valid EAF that `compile:forms/1` can consume.
//! Output is ETF-encoded (Erlang External Term Format) bytes.

use eetf::{Atom, FixInteger, List, Term, Tuple};

use crate::prism::Prism;
use crate::resolve::{BranchAction, BranchPattern, OutputNode, TypeRegistry};

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

/// Emit an actor dispatch module from a grammar's TypeRegistry.
///
/// Each act declared in the grammar becomes an exported function that
/// dispatches to the registered actor process via gen_server:call.
///
/// For `grammar @compiler { action compile { source: target } }`:
/// ```erlang
/// -module('@compiler').
/// -export([compile/1]).
/// compile(Args) -> gen_server:call('@compiler', {compile, Args}).
/// ```
pub fn emit_actor_module(registry: &TypeRegistry) -> Vec<u8> {
    let module_name = &registry.domain;
    let act_names = registry.act_names();

    let mut forms = Vec::new();

    // {attribute, 1, module, ModuleName}
    forms.push(eaf_tuple(vec![
        eaf_atom("attribute"),
        eaf_int(1),
        eaf_atom("module"),
        eaf_atom(module_name),
    ]));

    // {attribute, 2, export, [{act1, 1}, {act2, 1}, ...]}
    let exports: Vec<Term> = act_names
        .iter()
        .map(|name| eaf_tuple(vec![eaf_atom(name), eaf_int(1)]))
        .collect();
    forms.push(eaf_tuple(vec![
        eaf_atom("attribute"),
        eaf_int(2),
        eaf_atom("export"),
        eaf_list(exports),
    ]));

    // One function per act:
    // {function, Line, Name, 1, [{clause, Line, [Args], [], [Body]}]}
    // Body = local dispatch + cross-actor calls
    let mut line = 3i32;
    for name in &act_names {
        let calls = registry.action_calls(name);
        forms.push(emit_act_function(module_name, name, calls, line));
        line += 1;
    }

    let term = Term::from(List::from(forms));
    let mut buf = Vec::new();
    term.encode(&mut buf).expect("ETF encoding should not fail");
    buf
}

/// Emit a single act dispatch function.
///
/// Without cross-actor calls:
/// ```erlang
/// name(Args) -> gen_server:call('module', {name, Args}).
/// ```
///
/// With cross-actor calls:
/// ```erlang
/// commit(Args) ->
///     gen_server:call('integration', {commit, Args}),
///     gen_server:call('filesystem', {write, Args}).
/// ```
fn emit_act_function(
    module: &str,
    act_name: &str,
    calls: &[(String, String, Vec<String>)],
    line: i32,
) -> Term {
    // The argument variable: {var, Line, 'Args'}
    let args_var = eaf_tuple(vec![eaf_atom("var"), eaf_int(line), eaf_atom("Args")]);

    // Local dispatch: gen_server:call(Module, {ActName, Args})
    let local_call = emit_gen_server_call(module, act_name, &args_var, line);

    // Build body: local dispatch + cross-actor calls
    let mut body = vec![local_call];
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
