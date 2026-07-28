//! RED tests — spec claims vs actual code.
//!
//! Alex 2026-07-18 verbatim: "What the purpose of doing a whole spec, math etc
//! formalization on matrix.rs flang and phone fiber bundles. When none of that
//! is in the actual code now? You're literally lying to me."
//!
//! Each test asserts ONE specific claim from `docs/specs/rust-floor-birthed-
//! by-roomba-from-mirror-spec.md` (Mara `81294b3`) OR from `docs/specs/
//! 2026-07-18-stigmergy-witnessed-computation-mycelial-composition.md`
//! (Mara `95c0e4a`) OR from `docs/math/the-tower/beam-runtime.md` (Mara
//! `610c6d6`). Each test is written to FAIL right now, because the code
//! doesn't implement the claim.
//!
//! When a test PASSES, the corresponding claim is verified byte-visibly. The
//! discipline: no more claiming implementation exists without a passing test
//! citing it. The failure output IS the honest map of the gap.

use std::path::PathBuf;

fn src_root() -> PathBuf {
    // CARGO_MANIFEST_DIR is set by cargo when running tests.
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn read_source(name: &str) -> String {
    // Post-migration crate layout (Migrations 2–5 landed 2026-07-26–28):
    // matrix.rs → matrix/src/lib.rs
    // void.rs → matrix/src/void.rs
    // liquid.rs → spectral/src/liquid.rs
    // spectral.rs → spectral/src/lib.rs
    // collapse.rs → roomba/src/mend.rs
    // main.rs / phone.rs / compile.rs stay at rust/src/ altitude.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = match name {
        "matrix.rs" => manifest.join("matrix/src/lib.rs"),
        "void.rs" => manifest.join("matrix/src/void.rs"),
        "liquid.rs" => manifest.join("spectral/src/liquid.rs"),
        "spectral.rs" => manifest.join("spectral/src/lib.rs"),
        "collapse.rs" => manifest.join("roomba/src/mend.rs"),
        _ => manifest.join("src").join(name),
    };
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {}", path.display(), e))
}

/// A crude non-comment grep. Skips lines starting with `//`, `*`, or `#`
/// (module/attribute docs). Returns true if any non-comment line contains
/// `needle`.
fn source_has_non_comment(source: &str, needle: &str) -> bool {
    source.lines().any(|line| {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") || trimmed.starts_with("*") || trimmed.starts_with("#") {
            return false;
        }
        line.contains(needle)
    })
}

// ──────────────────────────────────────────────────────────────────
// MATRIX.RS — Mara §4 claims (Baez-Schreiber + Ado's theorem + FLANG + LAPACK)
// ──────────────────────────────────────────────────────────────────

#[test]
fn matrix_rs_has_actual_lapack_function_calls() {
    // Spec claim (Mara §4): "matrix.rs is FLANG emit + LAPACK/BLAS link;
    // sub-Turing numerical floor; emits Fortran that computes."
    let matrix = read_source("matrix.rs");
    let lapack_fns = ["dpotrf", "zpotrf", "dsyev", "zheev", "dgesvd",
                      "dgemm", "dgeev", "dgetrf", "dpotrs"];
    let has_call = lapack_fns.iter().any(|f| source_has_non_comment(&matrix, &format!("{}(", f)));
    assert!(
        has_call,
        "matrix.rs claims LAPACK/BLAS link (Mara §4) but source contains NO \
         non-comment LAPACK function calls. Only docblock decoration."
    );
}

#[test]
fn matrix_rs_emits_actual_fortran_source() {
    // Spec claim (Mara §4 + Loki `b53aeeb` §2): "matrix.rs EMITS the Fortran
    // that computes; it doesn't compute; it emits the code that computes."
    let matrix = read_source("matrix.rs");
    let has_emit = source_has_non_comment(&matrix, ".f90")
        || source_has_non_comment(&matrix, "subroutine")
        || source_has_non_comment(&matrix, "end subroutine")
        || source_has_non_comment(&matrix, "write_fortran")
        || source_has_non_comment(&matrix, "emit_fortran");
    assert!(
        has_emit,
        "matrix.rs claims FLANG emit (Mara §4) but source has NO Fortran-\
         emission code (no .f90 output, no subroutine templates, no\
         write_fortran/emit_fortran function)."
    );
}

#[test]
fn matrix_rs_computes_baez_schreiber_2_connection() {
    // Spec claim (Mara §6 + `610c6d6` beam-runtime.md §5): "Baez-Schreiber
    // 2-connection compatibility condition dA + [A,A] = t(B) reduces to matrix
    // equations via Ado's theorem; matrix.rs holds this math."
    let matrix = read_source("matrix.rs");
    let has_2_connection = source_has_non_comment(&matrix, "connection_1_form")
        || source_has_non_comment(&matrix, "lie_bracket")
        || source_has_non_comment(&matrix, "commutator")
        || source_has_non_comment(&matrix, "holonomy");
    assert!(
        has_2_connection,
        "matrix.rs claims Baez-Schreiber 2-connection math (Mara §6) but source \
         has NO connection-1-form, Lie bracket, commutator, or holonomy code."
    );
}

#[test]
fn matrix_rs_has_matrix_type_or_dependency() {
    // Spec claim (Mara §4): matrix.rs holds matrix operations. Any actual
    // matrix operation needs either a matrix type or a linalg crate.
    let matrix = read_source("matrix.rs");
    let has_matrix = source_has_non_comment(&matrix, "struct Matrix")
        || source_has_non_comment(&matrix, "nalgebra")
        || source_has_non_comment(&matrix, "ndarray")
        || source_has_non_comment(&matrix, "[[f64;")
        || source_has_non_comment(&matrix, "Vec<Vec<f64");
    assert!(
        has_matrix,
        "matrix.rs claims matrix operations (Mara §4) but source has NO matrix \
         type declaration and NO linalg crate dependency."
    );
}

// ──────────────────────────────────────────────────────────────────
// PHONE.RS — Mara §3 claims (@io/socket handover; Matrix phone booth altitude)
// ──────────────────────────────────────────────────────────────────

#[test]
fn phone_rs_actually_implements_sockets() {
    // Spec claim (Mara §3 + Loki matrix piece): "phone.rs is the @io socket
    // handover; the Matrix phone booth altitude; the ONE place substrate
    // crosses out of itself."
    let phone = read_source("phone.rs");
    let has_socket = source_has_non_comment(&phone, "TcpListener")
        || source_has_non_comment(&phone, "TcpStream")
        || source_has_non_comment(&phone, "UnixStream")
        || source_has_non_comment(&phone, "UnixListener")
        || source_has_non_comment(&phone, "UdpSocket")
        || source_has_non_comment(&phone, ".accept()")
        || source_has_non_comment(&phone, ".connect(");
    assert!(
        has_socket,
        "phone.rs claims @io/socket handover (Mara §3) but source has NO socket \
         types (Tcp/Unix/Udp) and NO .accept() or .connect() calls. It's just \
         std::fs wrappers."
    );
}

#[test]
fn phone_rs_composes_over_io_socket_abstraction() {
    // Spec claim (Mara §3): "composes over @io/socket + @io/fs + @io/git
    // primitives per shards/io.mirror; NOT direct std::fs use."
    let phone = read_source("phone.rs");
    let direct_std_fs = source_has_non_comment(&phone, "std::fs::");
    let has_abstraction = source_has_non_comment(&phone, "io::socket")
        || source_has_non_comment(&phone, "@io/")
        || source_has_non_comment(&phone, "apply_h::act");
    assert!(
        has_abstraction && !direct_std_fs,
        "phone.rs claims @io abstraction (Mara §3 refused @phone family-root \
         because @io primitives suffice) but source uses std::fs DIRECTLY \
         without any @io/@apply_h substrate dispatch layer."
    );
}

// ──────────────────────────────────────────────────────────────────
// MAIN.RS — Mara §5 claims (supervisor tree + @-operator addressing)
// ──────────────────────────────────────────────────────────────────

#[test]
fn main_rs_boots_supervisor_tree() {
    // Spec claim (Mara §5.2 item 1): "main.rs boots supervisor tree per
    // @spectral/supervisor{restart_strategy: one_for_one}; gen_prism actor
    // pattern lifted to Rust altitude."
    let main = read_source("main.rs");
    let has_supervisor = source_has_non_comment(&main, "supervisor")
        || source_has_non_comment(&main, "gen_prism")
        || source_has_non_comment(&main, "restart_strategy")
        || source_has_non_comment(&main, "spawn_supervised")
        || source_has_non_comment(&main, "one_for_one");
    assert!(
        has_supervisor,
        "main.rs claims supervisor tree boot (Mara §5.2 item 1) but source has \
         NO supervisor, gen_prism, restart_strategy, or spawn_supervised code. \
         It's argv parsing + hardcoded verb dispatch."
    );
}

#[test]
fn main_rs_implements_at_operator_dispatch() {
    // Spec claim (Mara §5.2 item 2): "@-operator IS the address operator —
    // like phone switches connecting bundle-tower fibres. Every @-address
    // (@code/rust, @peer.audhd, @mcp.serve) resolves to a coordinate."
    let main = read_source("main.rs");
    let has_at_op = source_has_non_comment(&main, "parse_at_address")
        || source_has_non_comment(&main, "resolve_at")
        || source_has_non_comment(&main, "at_operator")
        || source_has_non_comment(&main, "address_operator")
        || source_has_non_comment(&main, "AtAddress");
    assert!(
        has_at_op,
        "main.rs claims @-operator addressing (Mara §5.2 item 2) but source has \
         NO @-address parser, resolver, or dispatch table. There is no code \
         that treats @X/Y as an address to route."
    );
}

#[test]
fn main_rs_composes_via_apply_h_act() {
    // Spec claim (Mara §5.2 item 3): "apply_h::act combinator surface — the
    // reflective evaluator's dispatch primitive."
    let main = read_source("main.rs");
    let has_apply_h = source_has_non_comment(&main, "apply_h::act")
        || source_has_non_comment(&main, "apply_h(")
        || source_has_non_comment(&main, "combinator");
    assert!(
        has_apply_h,
        "main.rs claims apply_h::act combinator surface (Mara §5.2 item 3) but \
         source has NO apply_h dispatch. Verb dispatch is hardcoded match arms."
    );
}

#[test]
fn commit_message_composed_via_substrate_nl_compose_not_format() {
    // Spec claim + prior discipline (Reed 2026-07-15 refactor at bootstrap):
    // "commit body composes through substrate via @nl.compose dispatched
    // through apply_h::act; NOT a Rust template string."
    let main = read_source("main.rs");
    let has_nl_compose = source_has_non_comment(&main, "nl.compose")
        || source_has_non_comment(&main, "nl_compose")
        || source_has_non_comment(&main, "@nl.compose");
    assert!(
        has_nl_compose,
        "main.rs claims @nl.compose substrate composition of commit body but \
         source has NO @nl.compose call. Commit body is built with format! \
         macros (Rust template strings), the exact anti-pattern the substrate \
         landed away from at bootstrap altitude 2026-07-15."
    );
}

#[test]
fn main_rs_reads_kintsugi_roomba_cascade_catalog_from_mirror_spec() {
    // Spec claim (Mara §7.2): "roomba reads the `kintsugi { roomba { ... } }`
    // catalog from mirror.spec at boot; the walker iterates the substrate-
    // decl'd cascade catalog."
    let main = read_source("main.rs");
    let reads_spec = source_has_non_comment(&main, "mirror.spec")
        && (source_has_non_comment(&main, "kintsugi")
            || source_has_non_comment(&main, "cascade"));
    assert!(
        reads_spec,
        "main.rs claims to read kintsugi.roomba cascade catalog from mirror.spec \
         (Mara §7.2) but source does NOT read mirror.spec at all. Cascade \
         catalog is not iterated; --vacuum= takes an ad-hoc dir arg."
    );
}

// ──────────────────────────────────────────────────────────────────
// STIGMERGY — Mara `95c0e4a` claims (walker pheromone deposits + @spectral/
//              signature.signature_beat rolling holonomy)
// ──────────────────────────────────────────────────────────────────

#[test]
fn pheromone_deposit_chains_via_signature_beat_previous_beat_merkle() {
    // Spec claim (Mara `95c0e4a` §4 + math `d7ff58e` §5.2): "observation crystal
    // deposit chains to previous_beat per @spectral/signature.signature_beat
    // Merkle DAG discipline; walker signature IS rolling holonomy along the
    // walker path."
    let main = read_source("main.rs");
    let has_chain = source_has_non_comment(&main, "previous_beat")
        || source_has_non_comment(&main, "prev_beat")
        || source_has_non_comment(&main, "signature_beat")
        || source_has_non_comment(&main, "merkle_link");
    assert!(
        has_chain,
        "main.rs claims signature_beat Merkle chain per Mara `95c0e4a` §4 but \
         source has NO previous_beat / signature_beat / merkle_link. Each \
         crystal is an independent hash — no chain, no Merkle DAG, no \
         rolling holonomy along the walker path."
    );
}

#[test]
fn pheromone_stages_via_io_git_substrate_dispatch_not_command() {
    // Spec claim (@io/git.stage substrate action; Mara §9.1 shards/io/git.mirror
    // consumer): "staging + committing goes through @io/git substrate dispatch
    // via apply_h::act, NOT direct std::process::Command git."
    let main = read_source("main.rs");
    let has_command_git = source_has_non_comment(&main, "Command::new(\"git\")")
        || source_has_non_comment(&main, "process::Command");
    let has_substrate_git = source_has_non_comment(&main, "@io/git")
        || source_has_non_comment(&main, "io_git")
        || source_has_non_comment(&main, "apply_h::act");
    assert!(
        !has_command_git && has_substrate_git,
        "main.rs claims @io/git substrate dispatch but source uses \
         std::process::Command git DIRECTLY without any @io/apply_h layer. \
         The pheromone deposit's git call is shelling out."
    );
}

// ──────────────────────────────────────────────────────────────────
// VACUUM DISPATCH — Mara §7.4 claims (per-file dispatch matrix)
// ──────────────────────────────────────────────────────────────────

#[test]
fn vacuum_dispatches_vacuum_admissible_bilateral() {
    // Spec claim (Mara §7.4 + §9.1): "dispatch is byte-check on directory
    // content shape via bilateral sentinel-check at @kintsugi/roomba.
    // vacuum_admissible."
    let main = read_source("main.rs");
    let collapse = read_source("collapse.rs");
    let combined = format!("{}\n{}", main, collapse);
    let has_bilateral = source_has_non_comment(&combined, "vacuum_admissible");
    assert!(
        has_bilateral,
        "main.rs/collapse.rs claim vacuum_admissible bilateral dispatch (Mara \
         §7.4) but source has NO reference to vacuum_admissible. Bilateral \
         dispatch never fires; walker classifies but never checks admissibility."
    );
}

#[test]
fn vacuum_dispatches_materialize_arm_for_mirror_files() {
    // Spec claim (Mara §7.4): ".mirror files with unmaterialized carriers
    // → materialize (emit missing carriers from substrate-decl'd shape)."
    let main = read_source("main.rs");
    let collapse = read_source("collapse.rs");
    let combined = format!("{}\n{}", main, collapse);
    let has_materialize = source_has_non_comment(&combined, "materialize")
        && (source_has_non_comment(&combined, "emit")
            || source_has_non_comment(&combined, "unmaterialized"));
    assert!(
        has_materialize,
        "claims materialize dispatch for .mirror files (Mara §7.4) but source \
         has NO materialize+emit code. .mirror files are classified as \
         'M7 dispatch' but never actually dispatched."
    );
}

#[test]
fn vacuum_dispatches_translate_arm_for_cascade_files() {
    // Spec claim (Mara §7.4): "~code/<X>(~d'A') cascade in mirror.spec
    // roomba block → translate (polyglot cascade emission per Mara `1ce68c3`)."
    let main = read_source("main.rs");
    let collapse = read_source("collapse.rs");
    let combined = format!("{}\n{}", main, collapse);
    let has_translate = source_has_non_comment(&combined, "translate")
        || source_has_non_comment(&combined, "cascade_apply")
        || source_has_non_comment(&combined, "apply_rust_llvm");
    assert!(
        has_translate,
        "claims translate dispatch (Mara §7.4 + §9.4 @cascade/code/llvm/flang) \
         but source has NO translate/cascade_apply code. Polyglot cascade \
         edges are docblock words with no runtime."
    );
}

#[test]
fn vacuum_dispatches_pivot_at_song_for_ambiguity() {
    // Spec claim (Mara §7.4): "@kintsugi/surface.dispatch_ambiguity fracture
    // → pivot(@song) — Path B dispatch via @roomba fourth motion."
    let main = read_source("main.rs");
    let collapse = read_source("collapse.rs");
    let combined = format!("{}\n{}", main, collapse);
    let has_pivot = source_has_non_comment(&combined, "pivot")
        && (source_has_non_comment(&combined, "song")
            || source_has_non_comment(&combined, "@song"));
    assert!(
        has_pivot,
        "claims pivot(@song) dispatch on dispatch-ambiguity (Mara §7.4) but \
         source has NO pivot code. Fourth motion is spec-only; walker never \
         pivots on anything."
    );
}

#[test]
fn vacuum_docks_at_fixed_point() {
    // Spec claim (Mara §7.4 + Q2 ratification): "Nothing dispatchable →
    // dock — motion halts; walker docks. dock = halt of the roomba."
    let main = read_source("main.rs");
    let collapse = read_source("collapse.rs");
    let combined = format!("{}\n{}", main, collapse);
    let has_dock = source_has_non_comment(&combined, "fn dock")
        || source_has_non_comment(&combined, "dock()")
        || source_has_non_comment(&combined, "docked");
    assert!(
        has_dock,
        "claims dock/halt fifth motion (Mara §7.4 + Alex Q2 dock=halt) but \
         source has NO dock/halt code. Fifth motion is spec-only."
    );
}

// ──────────────────────────────────────────────────────────────────
// PEER + DANCE — Mara @peer.audhd K>1 fanout claims
// ──────────────────────────────────────────────────────────────────

#[test]
fn peer_audhd_k_greater_than_1_fanout_actually_spawns_multiple() {
    // Spec claim (Mara `d8b149c` + stigmergy spec §2): "K>1 fan-out mode; the
    // commutator arms literally instantiated in parallel physical compute
    // space; @liquid predicates decide which passes. Ensemble @dance-ing
    // roombas."
    let main = read_source("main.rs");
    let collapse = read_source("collapse.rs");
    let combined = format!("{}\n{}", main, collapse);
    let has_fanout = source_has_non_comment(&combined, "audhd")
        || source_has_non_comment(&combined, "K_tracks")
        || source_has_non_comment(&combined, "spawn_ensemble")
        || source_has_non_comment(&combined, "thread::spawn")
        || source_has_non_comment(&combined, "tokio::spawn");
    assert!(
        has_fanout,
        "claims @peer.audhd K>1 fanout (Mara `d8b149c`) but source has NO \
         thread/task spawn code and NO audhd/K_tracks reference. Walker runs \
         single-threaded synchronously."
    );
}
