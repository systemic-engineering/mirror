//! Polyglot cascade translation smoke — Reed Tick M3 empirical witness.
//!
//! Marker: [substrate-floor:@io-boundary] Reed 2026-07-17.
//!
//! Audit chain:
//! - Mara `1ce68c3` polyglot-loss-aware-translation spec §9 M3
//! - Mara `7186410` + `62d1b1c` M1 endpoints (@code/turing + @code/llvm)
//! - Mara `eee446b` + `c9328ec` + `3322825` M2 cascade species (Edges 1–3)
//! - Reed `7a962ab` M3 FLOOR resolver arms in bootstrap/src/apply_h.rs
//! - Alex 2026-07-17 verbatim ratification: "So we can have
//!   @cascade/code/llvm/turing and @cascade/code/rust/llvm. And boom.
//!   The loop closes."
//!
//! This smoke test dispatches each cascade `apply` action in sequence
//! and asserts the shape of the returned Verdict at each edge. It does
//! NOT assert semantic correctness of the LLVM→Turing lowering — MVP is
//! bytes-first per Reed Tick M3 brief; correctness lifts in a subsequent
//! tick when a smoke test dispatches per-instruction Alive2-style
//! translation-validation.

use mirror::apply_h::{self, Value, Verdict};
use std::fs;
use std::path::PathBuf;

/// Locate the emitted value under the expected located_opacity key.
/// Returns None if the verdict isn't Partial or the key isn't present.
fn located(verdict: &Verdict, key: &str) -> Option<String> {
    match verdict {
        Verdict::Partial(t) => t
            .located_opacity
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone()),
        _ => None,
    }
}

/// Create a fresh temp-dir with a minimal Rust source fixture.
fn write_fixture() -> PathBuf {
    let base = std::env::temp_dir().join(format!(
        "mirror-polyglot-cascade-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&base).expect("mk temp dir");
    let rs = base.join("add.rs");
    fs::write(
        &rs,
        "pub fn add(a: i32, b: i32) -> i32 { a + b }\n",
    )
    .expect("write fixture .rs");
    rs
}

#[test]
fn edge1_rust_to_llvm_returns_partial_with_llvm_ir_module_key() {
    // If rustc isn't available on PATH, skip cleanly — the cascade
    // capability lands even in environments without rustc; the smoke
    // test only asserts the arm's shape when the tool is present.
    if std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("polyglot_cascade_translation_smoke: rustc unavailable; skipping edge1");
        return;
    }

    let rs = write_fixture();
    let verdict = apply_h::act(
        "@cascade/code/rust/llvm.apply_rust_llvm".to_string(),
        vec![Value {
            oid: rs.to_string_lossy().into_owned(),
        }],
    );
    let ir = located(&verdict, "@code/llvm/llvm_ir_module").unwrap_or_else(|| {
        panic!(
            "expected Verdict::Partial with @code/llvm/llvm_ir_module key; got {:?}",
            verdict
        )
    });
    assert!(
        !ir.is_empty(),
        "emitted LLVM IR bytes must be non-empty; rustc failed?"
    );
    // Sanity: LLVM IR text form contains "target datalayout" (per LLVM
    // LangRef §"Module Structure"). This is not a correctness assertion
    // — it just verifies rustc actually emitted IR text (not e.g. an
    // empty file or a bitcode blob).
    assert!(
        ir.contains("target datalayout") || ir.contains("ModuleID"),
        "emitted bytes don't look like LLVM IR text: first 200 chars = {:?}",
        &ir[..ir.len().min(200)]
    );
}

#[test]
fn edge2_llvm_to_turing_returns_partial_with_program_key() {
    // Direct empirical: pass a minimal LLVM-IR-shaped byte string
    // (a few text lines) into Edge 2 without shelling to rustc.
    // Bytes-first MVP: any non-empty text works.
    let fake_ir = "target datalayout = \"e-m:o\"\n\
                   define i32 @add(i32 %a, i32 %b) {\n\
                     %sum = add i32 %a, %b\n\
                     ret i32 %sum\n\
                   }\n";
    let verdict = apply_h::act(
        "@cascade/code/llvm/turing.apply_llvm_turing".to_string(),
        vec![Value {
            oid: fake_ir.to_string(),
        }],
    );
    let program = located(&verdict, "@code/turing/program").unwrap_or_else(|| {
        panic!(
            "expected Verdict::Partial with @code/turing/program key; got {:?}",
            verdict
        )
    });
    assert!(
        program.contains("@code/turing.program"),
        "tape program serialization missing header; got {:?}",
        &program[..program.len().min(200)]
    );
    assert!(
        program.contains("transitions:"),
        "tape program serialization missing transitions block; got {:?}",
        &program[..program.len().min(200)]
    );
    // Halt state must be present so the tape terminates cleanly.
    assert!(
        program.contains("q_halt"),
        "tape program missing q_halt terminator; got {:?}",
        &program[..program.len().min(200)]
    );
}

#[test]
fn edge3_turing_to_mirror_returns_partial_with_mirror_value_key() {
    let fake_program = "@code/turing.program {\n\
                       \x20\x20initial_state: q0\n\
                       \x20\x20halt_states: [q_halt]\n\
                       \x20\x20transitions: [\n\
                       \x20\x20\x20\x20(q0, \"noop\") -> (q1, \"noop\", right),\n\
                       \x20\x20\x20\x20(q1, \"\") -> (q_halt, \"\", stay),\n\
                       \x20\x20]\n\
                       }\n";
    let verdict = apply_h::act(
        "@cascade/code/turing/mirror.apply_turing_mirror".to_string(),
        vec![Value {
            oid: fake_program.to_string(),
        }],
    );
    let value = located(&verdict, "@code/mirror/value").unwrap_or_else(|| {
        panic!(
            "expected Verdict::Partial with @code/mirror/value key; got {:?}",
            verdict
        )
    });
    assert!(!value.is_empty(), "mirror-substrate-value bytes must be non-empty");
    // Substrate-honest wrap: the value contains a @glass Focus node + a
    // @code/turing/program Project child.
    assert!(
        value.contains("focus @code/turing/mirror/value"),
        "mirror value missing Focus header; got {:?}",
        &value[..value.len().min(200)]
    );
    assert!(
        value.contains("project @code/turing/program"),
        "mirror value missing Project child header; got {:?}",
        &value[..value.len().min(200)]
    );
    // The tape-program bytes round-trip verbatim as a Dark region.
    assert!(
        value.contains("q_halt"),
        "mirror value missing round-tripped tape-program bytes"
    );
}

#[test]
fn full_cascade_composition_yields_mirror_substrate_value() {
    // End-to-end (edges 2 + 3 only; edge 1 requires rustc and is
    // exercised by `edge1_rust_to_llvm_returns_partial_with_llvm_ir_module_key`
    // separately). Confirms the composition chain works when the
    // located_opacity outputs are threaded through as inputs.
    let fake_ir = "target datalayout = \"e-m:o\"\n\
                   define i32 @noop() { ret i32 0 }\n";

    // Edge 2.
    let v2 = apply_h::act(
        "@cascade/code/llvm/turing.apply_llvm_turing".to_string(),
        vec![Value {
            oid: fake_ir.to_string(),
        }],
    );
    let program = located(&v2, "@code/turing/program").expect("edge 2 output");

    // Edge 3 — thread the edge-2 output into the edge-3 input.
    let v3 = apply_h::act(
        "@cascade/code/turing/mirror.apply_turing_mirror".to_string(),
        vec![Value { oid: program }],
    );
    let value = located(&v3, "@code/mirror/value").expect("edge 3 output");
    assert!(!value.is_empty(), "final mirror-substrate-value ref must be non-empty");
    assert!(
        value.contains("focus @code/turing/mirror/value"),
        "composition-chain output missing Focus header"
    );
}

#[test]
fn arms_fail_cleanly_on_missing_args() {
    // Substrate-honest: each arm returns Verdict::Fail (not panic, not
    // silent Pass) when called without required args. Mirrors the arg-
    // count guard in every other landed arm.
    let v1 = apply_h::act(
        "@cascade/code/rust/llvm.apply_rust_llvm".to_string(),
        vec![],
    );
    assert!(matches!(v1, Verdict::Fail(_)), "edge 1 must Fail on 0 args; got {:?}", v1);

    let v2 = apply_h::act(
        "@cascade/code/llvm/turing.apply_llvm_turing".to_string(),
        vec![],
    );
    assert!(matches!(v2, Verdict::Fail(_)), "edge 2 must Fail on 0 args; got {:?}", v2);

    let v3 = apply_h::act(
        "@cascade/code/turing/mirror.apply_turing_mirror".to_string(),
        vec![],
    );
    assert!(matches!(v3, Verdict::Fail(_)), "edge 3 must Fail on 0 args; got {:?}", v3);
}
