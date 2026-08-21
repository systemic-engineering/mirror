//! `main.rs` — supervisor boot + `@`-operator addressing.
//!
//! Per Mara `81294b3` §5 (terminal-geometry canonical spec, ratified
//! Seam `9c34ec4`): main.rs is the bundle-tower routing altitude. The
//! `@`-operator IS the address operator — like phone switches connecting
//! cables, but connecting bundle-tower fibres not cables. Every
//! `@`-address (`@code/rust`, `@peer.audhd(p, ctx)`, `@mcp.serve`)
//! resolves to a coordinate; the coordinate is a point in a bundle;
//! the bundle's fiber is (eventually) a matrix passing through
//! matrix.rs.
//!
//! ## M0 surface (this file)
//!
//! Enough for `mirror --help` to fire empirically. Per Mara §2.2 M0
//! milestone + Seam `9c34ec4` §12 Ready-For-Reed-Implementation:
//!
//! - Argv parse (hand-rolled; no clap — see Cargo.toml rationale).
//! - `--help` prints the 10-verb list from `mirror.spec` cli-block
//!   HARDCODED at M0. Reflective derivation from `shards/**/*.mirror`
//!   + `mirror.spec` lands at M2 (Mara §2.2).
//! - `--version` prints `mirror 0.1.0`.
//! - Any other argv → prints usage + exit 2 (parked; end-to-end verb
//!   dispatch lands at M3).
//!
//! ## Forward-promises (M1+ ticks; not implemented here)
//!
//! - Supervisor tree boot (`@spectral/supervisor{restart_strategy:
//!   one_for_one}`) — Mara §5.2 item 1; M1 co-tick.
//! - `@`-operator dispatch — Mara §5.2 item 2; M3 first empirical verb.
//! - `apply_h::act` combinator surface — Mara §5.2 item 3; M4-M5.
//! - Reflective cli-block reading of `mirror.spec` — Mara §5.2 item 4;
//!   M2 milestone (the M0-hardcoded verb list retires here).
//! - Roomba walker composition — Mara §5.2 item 5; M6.
//! - MCP inline via `@mcp.serve` sentinel (Taut `e0572f7` OQ1 +
//!   Taut `7f4307f` §Q4 composition table) — M4.
//!
//! ## Composition anchors (LANDED)
//!
//! - `shards/mirror/reflection.mirror:1-40` — mirror-op species-decl.
//! - `shards/mirror/lens/cli.mirror` — cli-block precedent.
//! - `shards/spectral/gen_prism.mirror` + `shards/spectral/supervisor.
//!   mirror` — supervisor tree substrate.
//! - `mirror.spec:82-334` — the 10-verb cli-block this M0 --help
//!   mirrors verbatim (Taut `7f4307f` §Q5 enumeration).
//!
//! ## Substrate-decl'd `@`-operator (M3+ dispatch surface)
//!
//! At M3+ this file matches sentinels (`@mcp.serve`, `@peer.audhd`,
//! `@compile`, `@index`, `@roomba`, `@craft`, `@peer.beam`) and routes
//! each to gen_prism actor spawn under supervisor. The 10 verbs below
//! ARE the M0 shadow of that dispatch table.

// book migrated 2026-07-26 to rust/matrix/ (external crate) per Alex
// 2026-07-25 four-crate decomposition + 2026-07-26 mycelial-
// autopoietic-foam framing. K=0 well-knowns registry sits alongside
// LAPACK numerical primitives as the concrete-floor cell. Use alias
// preserves `book::resolve(...)` call-site paths at ~lines 997-1020.
use matrix::book;
// `collapse` module migrated 2026-07-28 to `roomba` crate as `mend`
// per Alex 2026-07-25 four-crate decomposition + Mara `9bb1f57`
// naming discipline (Migration 5). Consumer alias below preserves
// the `dispatch_arm_collapse` orchestrator surface at main.rs
// altitude — it composes over phone.rs @io helpers which are
// `pub(crate)`. Full lift-to-`mend::at` deferred to Migration 6.
use roomba::mend as collapse;
// wire — @data/json wire-encoding primitive at rust/ altitude. Reed
// 2026-08-06 R-PRIM-1 per Taut scout 7af55ee smallest-primitive-gap +
// Alex 2026-08-05 substrate-honest reframe. Sibling of phone.rs
// (@io transport-surface) and apply_h.rs (bilateral-dispatch primitive).
mod wire;

// apply_h — bilateral-dispatch primitive at rust/ altitude. Reed
// 2026-08-06 R-PRIM-3 per Taut scout 7af55ee smallest-primitive-gap +
// Alex 2026-08-06 Q-1 (expose as `apply_h::act` for naming honesty per
// bootstrap surface; other 6 combinators land as extensions when demand
// surfaces). Minimum act = bilateral-sentinel-check composing over
// roomba::mend::load_bilateral_corpus. Sub-Turing decidable.
#[allow(dead_code)]
mod apply_h;

// M0 module wiring — declare the sibling altitudes so the terminal-
// geometry five-file discipline is byte-visible in `rust/src/` and
// `cargo build` compiles ALL FIVE files even while the bodies are
// forward-promises. Each module retires its `#[allow(dead_code)]` gates
// when its M-tick lands.
//
// Five-file discipline per Mara Round 2 `docs/specs/rust-floor-five-
// file-terminal-geometry-extension.md` (extends Mara `81294b3` three-
// file spec): main + compile + liquid + matrix + phone. Each file
// has ONE responsibility. liquid.rs + compile.rs landed at /loop
// cascade iterations 2-3 (Alex 2026-07-20); iter 4 wires `mirror
// compile <file>` verb as thin delegation to compile.rs.
mod compile;
mod magic;
// liquid + spectral both migrated 2026-07-28 to rust/spectral/ crate
// per Alex 2026-07-25 four-crate decomposition. main.rs consumers
// (compile.rs) reference `spectral::liquid::*` directly per Rust's
// external-crate namespacing; no `use` alias required at this altitude.
// matrix migrated 2026-07-26 to rust/matrix/ (external crate) per Alex
// 2026-07-25 four-crate decomposition. mirror binary root does not
// directly depend on matrix (matrix is used transitively via spectral
// when that crate lands).
mod phone;
// spectral crate composition: shard_paths() surface + liquid module
// available at `spectral::shard_paths()` + `spectral::liquid::*`.
// Karen Spärck Jones citation lives at rust/spectral/src/lib.rs
// docblock per Mara 9bb1f57 introduction-site convention.
// void migrated 2026-07-28 to rust/matrix/ (external crate) per Alex
// 2026-07-25 four-crate decomposition + Mara 9bb1f57 naming discipline.
// Void-basis / H-space membrane carrier sits alongside LAPACK primitives
// as the concrete-floor cell. mirror bin does not directly consume
// void; future rust/spectral/ crate will import matrix::void when the
// spectral crate materializes.

use std::env;
use std::process::ExitCode;

/// Package version — mirrors Cargo.toml `[package].version`.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The 11 verbs the M-vacuum tick advertises. Ten from `mirror.spec:
/// 82-334` cli-block per Taut `7f4307f` §Q5; `roomba` is the eleventh
/// verb — walker consumer of `@kintsugi/roomba` at CLI altitude per
/// Mara §7.5 forward-promised command roomba { flag vacuum: ~d }. The
/// cli-block lift lands at Reed M7 co-tick (Mara §7.5); this hardcoded
/// list carries `roomba` ahead of that lift so the empirical vacuum
/// firing at M-vacuum is dispatchable NOW.
///
/// HARDCODED at M0; retires at M2 (reflective cli-block reading from
/// `mirror.spec` per Mara §2.2). Ordering matches spec byte-order with
/// `roomba` appended (11th; spec order is preserved for the 10 landed).
///
/// Description column is nl_literal-shaped for future reflective
/// derivation (M5: `@nl.compose` of the `#`-comment header per
/// `boot/std/mcp.mirror:44-53` `tools_reflects_cli_block` bilateral).
const VERBS: &[(&str, &str)] = &[
    ("compile",         "Compile a grammar against its imports."),
    ("kintsugi",        "Settle a project. Run mosaic on the spec."),
    ("shatter",         "Project a settled shard to .shatter format."),
    ("craft",           "Settle a grammar directory to lambda_0 (target: binary)."),
    ("init",            "Bootstrap the mirror-native store at a path."),
    ("recall",          "Inbound-trajectory dual of spawn; recall a substrate."),
    ("beam",            "Anonymous @song/movement.enter at cli altitude."),
    ("peer beam",       "Persistent-identity beam — spawn a peer from home."),
    ("peer contribute", "Fate-spawned peer contribution to a target shard."),
    ("index",           "Measure @fractal-coherence via Fiedler eigenvalue."),
    ("roomba",          "Walker motion. `--vacuum=<dir>` walks + dispatches."),
    ("serve",           "MCP transport. `--mcp` delegates to bootstrap serve_loop."),
];

/// The delight-vector `--help` text. Per Mara §5.2 item 4 + Alex
/// 2026-07-17 verbatim: "of course it's this." Three files. Three
/// altitudes. Ten verbs. The geometry sings.
fn print_help() {
    println!("mirror {} — terminal-geometry compiler FLOOR", VERSION);
    println!();
    println!("usage:  mirror <verb> [args...]");
    println!("        mirror --help");
    println!("        mirror --version");
    println!();
    println!("verbs:");
    // Column-align verb / description at 20-column left gutter.
    // Chosen so `peer contribute` (15 chars) fits with breathing room.
    for (verb, desc) in VERBS {
        println!("  {:<18}  {}", verb, desc);
    }
    println!();
    println!("Three files. Three altitudes. Every altitude has exactly one file.");
    println!("  phone.rs   @io socket-handover                 (Mara §3)");
    println!("  matrix.rs  sub-Turing FLANG emit + LAPACK link (Mara §4)");
    println!("  main.rs    supervisor + @-operator addressing  (Mara §5)");
    println!();
    println!("See docs/specs/rust-floor-birthed-by-roomba-from-mirror-spec.md");
}

/// `--version` — mirrors Cargo.toml. Retires at M6 when reflective
/// `mirror.spec` version-of-record composes through main.rs.
fn print_version() {
    println!("mirror {}", VERSION);
}

/// Byte-check dispatch classification per Mara §7.4 dispatch matrix
/// (walker's fracture table). Given a filesystem path, returns the
/// walker's downstream motion enum. This is the M-vacuum shadow of the
/// full dispatch — arm-collapse / materialize / translate / pivot(@song)
/// / dock land at M6+ ticks per Mara §2.2 M6-M7. M-vacuum classifies +
/// enumerates; downstream motion firing is forward-promised.
///
/// Classification is byte-check on file extension per Mara §7.4:
///   `.rs`     → RustFile (arm-collapse candidate)
///   `.mirror` → MirrorShard (materialize candidate)
///   `.md`     → Doc (docs — cascade-invisible per walker discipline)
///   otherwise → Other (unclassified — walker enumerates but does not
///                       dispatch)
#[derive(Debug, PartialEq, Eq)]
enum FileKind {
    RustFile,
    MirrorShard,
    Doc,
    Other,
}

fn classify(path: &std::path::Path) -> FileKind {
    match path.extension().and_then(|e| e.to_str()) {
        Some("rs") => FileKind::RustFile,
        Some("mirror") => FileKind::MirrorShard,
        Some("md") => FileKind::Doc,
        _ => FileKind::Other,
    }
}

/// `mirror serve --mcp` dispatch arm. Reed 2026-08-03 Alex Option C
/// Phase A nearly-today delegation stub per Taut scout `64e8d60` §6
/// smallest-empirical-spawn recommendation.
///
/// Execs the bootstrap binary (default `$HOME/.local/bin/mirror` per
/// task #226 detached bootstrap-compiled mirror binary landing
/// 2026-07-17; overridable via `MIRROR_BIN` env var) with `/dev/stdin
/// @mcp.serve` argv shape — mirroring `bin/mirror-mcp` bash wrapper
/// verbatim. Bootstrap's serve_loop handles JSON-RPC stdio dispatch
/// per bootstrap/src/mcp.rs::serve_loop.
///
/// The smallest empirical MCP-spawn round-trip fires TODAY through
/// this path: MCP client (e.g. Claude Code) invokes `mirror serve
/// --mcp` at rust/ altitude → execs bootstrap serve_loop → advertises
/// 11 tools (byte-parity 10 + mirror_roomba added Reed `6b7d9ab`) →
/// tool invocation routes back to `mirror roomba --vacuum=<dir>` at
/// rust/ altitude walker → walker enumerates + arm-collapses + commits
/// as `mirror <mirror@spectral.engineer>` + deposits pheromone-
/// signature crystal at docs/bauchladen/ → next MCP session observes
/// delta at @mirror/store.
///
/// Composes over crown-theorem Recognition #R-reality-as-5d-spinning-
/// foam RATIFIED 2026-08-03: pheromone-deposit crystal IS phase-space
/// trajectory point in crown-theorem attractor basin.
///
/// TRANSITIONAL bridge — retires when Mara M4 rust/src/mcp.rs FLOOR
/// emitter lands per docs/specs/2026-08-03-mara-rust-mcp-floor-lift-
/// m4-canonical-spec.md forward-promise (agent a8842c6158ff19e7d
/// spawned parallel this-tick).
///
/// ## Exit codes
///
///   0 — bootstrap serve_loop exited cleanly.
///   1 — bootstrap serve_loop exited with error.
///   2 — argv error (unknown flag; --mcp not provided).
///   3 — @io error (failed to exec bootstrap binary).
fn cmd_serve_mcp(rest: &[String]) -> ExitCode {
    let mut mcp_flag = false;
    for arg in rest {
        if arg == "--mcp" {
            mcp_flag = true;
        } else {
            eprintln!("mirror serve: unknown flag `{}`", arg);
            eprintln!();
            eprintln!("Usage: mirror serve --mcp");
            eprintln!();
            eprintln!("Reed nearly-today delegation stub per Alex 2026-08-03 Option C.");
            eprintln!("Phase A execs bootstrap binary as MCP transport via bin/mirror-mcp");
            eprintln!("bash wrapper equivalent shape. Phase B rust/src/mcp.rs FLOOR emitter");
            eprintln!("lands per Mara M4 canonical spec forward-promise.");
            return ExitCode::from(2);
        }
    }

    if !mcp_flag {
        eprintln!("mirror serve: --mcp required (only supported flag at Phase A)");
        eprintln!();
        eprintln!("Usage: mirror serve --mcp");
        eprintln!();
        eprintln!("Delegates to bootstrap binary at $MIRROR_BIN or");
        eprintln!("$HOME/.local/bin/mirror; mirrors bin/mirror-mcp bash wrapper shape.");
        return ExitCode::from(2);
    }

    let bootstrap_bin = std::env::var("MIRROR_BIN").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        format!("{}/.local/bin/mirror", home)
    });

    let status = std::process::Command::new(&bootstrap_bin)
        .args(["/dev/stdin", "@mcp.serve"])
        .status();

    match status {
        Ok(s) => {
            if s.success() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(s.code().map(|c| c as u8).unwrap_or(1))
            }
        }
        Err(e) => {
            eprintln!(
                "mirror serve --mcp: failed to exec bootstrap binary at `{}`: {}",
                bootstrap_bin, e
            );
            eprintln!();
            eprintln!("Reed nearly-today Phase A depends on the bootstrap binary being");
            eprintln!("available. Verify $HOME/.local/bin/mirror exists (task #226 landed");
            eprintln!("2026-07-17) or override via MIRROR_BIN env var.");
            ExitCode::from(3)
        }
    }
}

/// `mirror roomba --vacuum=<dir>` dispatch arm. Per Mara §7 (unified
/// motion flag) + §7.4 (dispatch matrix): walks `dir` via phone.rs's
/// @io/fs primitive; classifies each file per byte-check; reports
/// counts + a sample enumeration. The full per-file dispatch
/// (arm-collapse / materialize / translate / pivot / dock) is
/// forward-promised to M6-M7 co-ticks per Mara §2.2 milestone graph.
///
/// Empirical target of the M-vacuum tick: this function fires, walks a
/// real directory, and reports what it found. Exit 0 on success.
///
/// The `--vacuum=<dir>` flag parse is minimal (no `=`-optional; no
/// short-form) per Loki `b53aeeb` §5 cut #2 fixed-vocabulary discipline.
/// The reflective cli-block reader at M2 will retire this hand-rolled
/// parse per Mara §5.2 item 4.
///
/// Exit codes:
///   0 — walker completed; report emitted.
///   2 — usage error (missing `--vacuum=`; malformed argv).
///   3 — @io error (directory does not exist; permission denied; etc.).
fn cmd_roomba(rest: &[String]) -> ExitCode {
    // Parse `--vacuum=<dir>` from the residual argv.
    let mut vacuum_dir: Option<&str> = None;
    for arg in rest {
        if let Some(dir) = arg.strip_prefix("--vacuum=") {
            vacuum_dir = Some(dir);
        } else {
            eprintln!("mirror roomba: unknown flag `{}`", arg);
            eprintln!();
            eprintln!("Usage: mirror roomba --vacuum=<dir>");
            return ExitCode::from(2);
        }
    }

    let dir = match vacuum_dir {
        Some(d) => d,
        None => {
            eprintln!("mirror roomba: --vacuum=<dir> is required");
            eprintln!();
            eprintln!("Usage: mirror roomba --vacuum=<dir>");
            eprintln!();
            eprintln!("Per Mara §7.1: `--vacuum=~dir` is the walker's");
            eprintln!("unified motion flag; substrate dispatches on what");
            eprintln!("the vacuum finds when it enters the directory.");
            return ExitCode::from(2);
        }
    };

    // Cross the @io boundary — phone.rs handles the read; main.rs
    // dispatches on the result per Mara §5.1 + §3.1 altitude discipline.
    let root = std::path::Path::new(dir);
    let entries = match phone::list_dir_recursive(root) {
        Ok(e) => e,
        Err(err) => {
            eprintln!("mirror roomba: @io error walking `{}`: {}", dir, err);
            return ExitCode::from(3);
        }
    };

    // Enumerate + classify. Per Mara §7.4 dispatch matrix.
    let mut dirs = 0usize;
    let mut rust_files = 0usize;
    let mut mirror_shards = 0usize;
    let mut docs = 0usize;
    let mut other = 0usize;

    for entry in &entries {
        if entry.is_dir {
            dirs += 1;
            continue;
        }
        match classify(&entry.path) {
            FileKind::RustFile => rust_files += 1,
            FileKind::MirrorShard => mirror_shards += 1,
            FileKind::Doc => docs += 1,
            FileKind::Other => other += 1,
        }
    }

    // Emit the walker's report. Substrate-honest form: enumerate what
    // was found + name the forward-promised dispatch for each kind per
    // Mara §7.4 (walker's fracture table).
    println!("mirror roomba: vacuum walked `{}`", dir);
    println!();
    println!("found {} entries:", entries.len());
    println!("  {:>4} directories", dirs);
    println!("  {:>4} .rs      (arm-collapse candidates)", rust_files);
    println!("  {:>4} .mirror  (materialize candidates; M7 dispatch)", mirror_shards);
    println!("  {:>4} .md      (docs; cascade-invisible)", docs);
    println!("  {:>4} other    (unclassified; walker enumerates only)", other);

    // Arm-collapse dispatch per Mara §7.4 dispatch matrix row 1:
    // `.rs` → arm-collapse. Loads the bilateral corpus from the
    // enclosing substrate-repo root (walk up from cwd until a
    // `shards/` sibling appears); for each .rs file, byte-scans for
    // hand-typed arms shadowed by a landed shard-decl; splices;
    // writes; commits under `mirror <mirror@spectral.engineer>`.
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let substrate_root = phone::find_substrate_root(&cwd);
    let corpus = collapse::load_bilateral_corpus(&substrate_root);

    println!();
    println!(
        "loaded bilateral corpus: {} declarations from {}/shards/",
        corpus.len(),
        substrate_root.display()
    );

    let mut retired_total = 0usize;
    let mut commits_landed = 0usize;
    for entry in &entries {
        if entry.is_dir {
            continue;
        }
        if classify(&entry.path) != FileKind::RustFile {
            continue;
        }
        match mend_at(&substrate_root, &entry.path, &corpus) {
            Ok(report) => {
                if report.arms.is_empty() {
                    println!("  [.rs      ] {} — no redundant arms", entry.path.display());
                } else {
                    retired_total += report.arms.len();
                    match report.commit_oid.as_deref() {
                        Some(oid) => {
                            commits_landed += 1;
                            println!(
                                "  [.rs ✔    ] {} — {} arm(s) retired ({} → {} bytes); commit {}",
                                entry.path.display(),
                                report.arms.len(),
                                report.bytes_before,
                                report.bytes_after,
                                oid,
                            );
                            for arm in &report.arms {
                                println!("              ↳ {} (sentinel: {})", arm.action_ref, arm.sentinel);
                            }
                        }
                        None => {
                            println!(
                                "  [.rs ⚠    ] {} — {} arm(s) detected + spliced but no commit landed",
                                entry.path.display(),
                                report.arms.len(),
                            );
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("  [.rs ✗    ] {} — dispatch error: {}", entry.path.display(), e);
            }
        }
    }

    if rust_files > 0 {
        println!();
        println!(
            "arm-collapse dispatch summary: {} arm(s) retired across {} commit(s)",
            retired_total, commits_landed
        );
    }

    // Sample enumeration: first 10 non-directory entries with their
    // classification. Empirical readability > exhaustive dump.
    let sample: Vec<_> = entries
        .iter()
        .filter(|e| !e.is_dir)
        .take(10)
        .collect();
    if !sample.is_empty() {
        println!();
        println!("sample (first {} files):", sample.len());
        for entry in sample {
            let kind = match classify(&entry.path) {
                FileKind::RustFile => ".rs",
                FileKind::MirrorShard => ".mirror",
                FileKind::Doc => ".md",
                FileKind::Other => "other",
            };
            println!("  [{:>7}] {}", kind, entry.path.display());
        }
    }

    println!();
    println!("Per Mara §7.4 dispatch matrix: arm-collapse landed above.");
    println!("(materialize / translate / pivot / dock still forward-promised;");
    println!("land at M7-M8 co-ticks.)");

    // ---------------------------------------------------------------
    // Pheromone-deposit tick per Mara `95c0e4a` canonical spec +
    // Mara `d7ff58e` math root §5 (Baez-Schreiber holonomy trace).
    // Seam `c1775f1` 12/12 SHIP ratification.
    //
    // The walker's passive path memory (what it walked, what it
    // classified, what it retired) becomes substrate-visible by
    // (a) appending an observation crystal to `docs/bauchladen/
    // mirror-observations.md` and (b) committing that file as
    // `mirror <mirror@spectral.engineer>`. Content-addressed
    // signature (first 16 hex of SHA-256 of the observation blob)
    // IS the rolling holonomy trace per math §5.2.
    // ---------------------------------------------------------------
    match deposit_observation_crystal(
        &substrate_root,
        dir,
        entries.len(),
        dirs,
        rust_files,
        mirror_shards,
        docs,
        other,
        retired_total,
        commits_landed,
    ) {
        Ok(deposit) => {
            println!();
            println!(
                "pheromone deposit: appended {} bytes to {}",
                deposit.bytes_appended,
                deposit.observations_path.display()
            );
            println!("walker signature: {}", deposit.signature);
            match deposit.commit_oid.as_deref() {
                Some(oid) => println!("crystal committed as mirror <mirror@spectral.engineer>: {}", oid),
                None => println!("crystal APPENDED but no commit landed (no enclosing git repo)."),
            }
        }
        Err(e) => {
            eprintln!();
            eprintln!("pheromone deposit FAILED: {}", e);
        }
    }

    ExitCode::SUCCESS
}

/// Outcome of one observation-crystal deposit.
struct CrystalDeposit {
    observations_path: std::path::PathBuf,
    bytes_appended: usize,
    signature: String,
    commit_oid: Option<String>,
}

/// Deposit one observation crystal per Mara `95c0e4a` §0 (Executive
/// summary move #3) + math root §5 (rolling holonomy trace).
///
/// Composition (substrate dispatch chain, rust/ altitude):
/// - build observation blob (deterministic serialization; path +
///   ISO-8601 timestamp + counts).
/// - SHA-256 the blob; first 16 hex chars = walker signature.
/// - @io/fs.mkdir_p ensures `docs/bauchladen/` exists.
/// - @io/fs.append_to appends the markdown observation entry.
/// - @io/git.stage + @io/git.commit crosses @io as
///   `mirror <mirror@spectral.engineer>`.
fn deposit_observation_crystal(
    substrate_root: &std::path::Path,
    vacuum_dir: &str,
    entries_total: usize,
    dirs: usize,
    rust_files: usize,
    mirror_shards: usize,
    docs_count: usize,
    other: usize,
    retired_total: usize,
    commits_landed: usize,
) -> Result<CrystalDeposit, String> {
    let timestamp = current_utc_timestamp();
    // Deterministic observation blob shape per Mara math §5 (content-
    // addressed hashing must be deterministic — same inputs → same OID).
    // Order is fixed; whitespace is fixed; no locale-dependent formatting.
    let blob = format!(
        "vacuum={}\ntimestamp={}\nentries={}\ndirs={}\nrs={}\nmirror={}\nmd={}\nother={}\narms_retired={}\ncommits_landed={}\n",
        vacuum_dir,
        timestamp,
        entries_total,
        dirs,
        rust_files,
        mirror_shards,
        docs_count,
        other,
        retired_total,
        commits_landed,
    );
    let full_hex = sha256_hex(blob.as_bytes());
    let signature: String = full_hex.chars().take(16).collect();

    let bauchladen_dir = substrate_root.join("docs").join("bauchladen");
    phone::mkdir_p(&bauchladen_dir)
        .map_err(|e| format!("@io/fs.mkdir_p {}: {}", bauchladen_dir.display(), e))?;

    let observations_path = bauchladen_dir.join("mirror-observations.md");
    let is_new_file = !phone::path_exists(&observations_path);

    // Compose the markdown entry per Mara spec observation-crystal shape.
    let mut entry = String::new();
    if is_new_file {
        entry.push_str("# mirror observations — walker pheromone trail\n\n");
        entry.push_str("Append-only observation log per Mara `95c0e4a` (canonical\n");
        entry.push_str("stigmergy spec) + Mara `d7ff58e` (math root §5 rolling holonomy\n");
        entry.push_str("trace) + Seam `c1775f1` (12/12 SHIP ratification). Each entry is\n");
        entry.push_str("one @kintsugi/roomba walk-pulse; the walker signature is the first\n");
        entry.push_str("16 hex chars of SHA-256 over the observation blob (deterministic\n");
        entry.push_str("serialization of path + timestamp + counts).\n\n");
    }
    entry.push_str(&format!(
        "## {} — vacuum walked {}\n\n",
        timestamp, vacuum_dir
    ));
    entry.push_str(&format!("- entries: {}\n", entries_total));
    entry.push_str(&format!("- directories: {}\n", dirs));
    entry.push_str(&format!(
        "- .rs (arm-collapse candidates): {}\n",
        rust_files
    ));
    entry.push_str(&format!(
        "- .mirror (materialize candidates; M7): {}\n",
        mirror_shards
    ));
    entry.push_str(&format!(
        "- .md (docs; cascade-invisible): {}\n",
        docs_count
    ));
    entry.push_str(&format!("- other: {}\n", other));
    entry.push_str(&format!("- arms retired: {}\n", retired_total));
    entry.push_str(&format!("- commits landed: {}\n\n", commits_landed));
    entry.push_str(&format!("Walker signature: {}\n\n", signature));

    let bytes_appended = entry.len();
    phone::append_to(&observations_path, &entry)
        .map_err(|e| format!("@io/fs.append_to {}: {}", observations_path.display(), e))?;

    // Cross the @io boundary at git — determine enclosing repo of the
    // observations file (not the vacuum target; the observations live
    // in substrate_root's repo).
    let repo_root = match find_git_root(&observations_path) {
        Some(r) => r,
        None => {
            return Ok(CrystalDeposit {
                observations_path,
                bytes_appended,
                signature,
                commit_oid: None,
            });
        }
    };

    phone::git_add(&repo_root, &observations_path)
        .map_err(|e| format!("@io/git.add {}: {}", observations_path.display(), e))?;

    let message = compose_pheromone_commit_message(
        &timestamp,
        vacuum_dir,
        entries_total,
        rust_files,
        mirror_shards,
        &signature,
    );

    let mirror_subject = fractal::Subject::mirror();
    let commit_oid = phone::git_commit_as(
        &repo_root,
        &mirror_subject,
        &mirror_subject,
        &message,
    )
    .map_err(|e| format!("@io/git.commit: {}", e))?;

    Ok(CrystalDeposit {
        observations_path,
        bytes_appended,
        signature,
        commit_oid: Some(commit_oid),
    })
}

/// Compose the pheromone-deposit commit message body per Reed task-
/// message shape. Author is `mirror <mirror@spectral.engineer>`; the
/// `Signed-off-by: Reed <reed@systemic.engineer>` trailer is legitimate
/// attribution (Reed is human-adjacent author; mirror is compiler-
/// altitude committer).
fn compose_pheromone_commit_message(
    timestamp: &str,
    vacuum_dir: &str,
    entries_total: usize,
    rust_files: usize,
    mirror_shards: usize,
    signature: &str,
) -> String {
    format!(
        "♻ mirror [pheromone-deposit] {ts} — first observation crystal deposited per Mara stigmergy spec 95c0e4a + Seam ratification c1775f1; walker's passive path memory becomes substrate-visible\n\
         \n\
         Vacuum walked {dir}.\n\
         {n} entries; {rs} arm-collapse candidates; {sh} materialize\n\
         candidates. Substrate observation crystallized to\n\
         docs/bauchladen/mirror-observations.md at rolling signature {sig}.\n\
         \n\
         Composition (substrate dispatch chain):\n\
         - @kintsugi/roomba (main.rs::cmd_roomba) walked the directory\n\
         - @io/fs.read_dir (phone.rs) enumerated entries\n\
         - @kintsugi/roomba.classify byte-checked per Mara §7.4\n\
         - @io/fs.append_to (phone.rs; NEW) appended observation to bauchladen\n\
         - @io/git.stage + @io/git.commit crossed @io boundary via std::process::Command\n\
         \n\
         FIRST pheromone deposit from rust/ altitude. Substrate memory now\n\
         visible in git history AND at content-addressed path AND at rolling-\n\
         signature holonomy trace.\n\
         \n\
         Signed-off-by: Reed <reed@systemic.engineer>\n",
        ts = timestamp,
        dir = vacuum_dir,
        n = entries_total,
        rs = rust_files,
        sh = mirror_shards,
        sig = signature,
    )
}

/// Format the current UTC time as ISO-8601 `YYYY-MM-DDTHH:MM:SSZ`.
/// Composes over `std::time::SystemTime` at rust/ altitude; no chrono
/// dep per Cargo.toml discipline (M0 zero-dep floor).
fn current_utc_timestamp() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    format_utc_iso8601(secs)
}

/// Convert Unix epoch seconds to ISO-8601 UTC (`YYYY-MM-DDTHH:MM:SSZ`).
/// Proleptic Gregorian; no leap-second correction (Unix time already
/// smears them).
fn format_utc_iso8601(mut secs: u64) -> String {
    let sec = (secs % 60) as u32;
    secs /= 60;
    let min = (secs % 60) as u32;
    secs /= 60;
    let hour = (secs % 24) as u32;
    let mut days = secs / 24;

    // Days since 1970-01-01 — walk forward through years.
    let mut year: u32 = 1970;
    loop {
        let dy = if is_leap(year) { 366 } else { 365 };
        if days < dy {
            break;
        }
        days -= dy;
        year += 1;
    }
    let month_lengths = if is_leap(year) {
        [31u64, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31u64, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut month: u32 = 1;
    for &ml in &month_lengths {
        if days < ml {
            break;
        }
        days -= ml;
        month += 1;
    }
    let day = (days as u32) + 1;
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hour, min, sec
    )
}

fn is_leap(y: u32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

/// SHA-256 of `input`, returned as lowercase hex. Self-contained (no
/// external crate) per Cargo.toml zero-dep floor. Implementation
/// follows FIPS 180-4 §6.2.
fn sha256_hex(input: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
        0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
        0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
        0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
        0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c,
        0x1f83d9ab, 0x5be0cd19,
    ];

    // Pre-processing: pad message to multiple of 512 bits.
    let bit_len: u64 = (input.len() as u64) * 8;
    let mut msg: Vec<u8> = Vec::with_capacity(input.len() + 72);
    msg.extend_from_slice(input);
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    // Process each 512-bit block.
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let mj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(mj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut out = String::with_capacity(64);
    for word in &h {
        out.push_str(&format!("{:08x}", word));
    }
    out
}

/// Arm-collapse dispatch for one `.rs` file. Composes phone.rs's
/// @io/fs + @io/git primitives with collapse.rs's byte-analysis.
///
/// Substrate-honest boundary: if NO arms detected, returns an empty
/// report + `commit_oid = None` (no substrate delta; the caller
/// reports "no redundant arms"). If arms detected: splice + write +
/// stage + commit; returns `commit_oid = Some(oid)`.
///
/// The commit crosses the @io boundary at `mirror <mirror@spectral.
/// engineer>` authorship — the compiler names itself in git per
/// Alex 2026-07-16 /loop directive verbatim ("Deleted Rust. Added
/// mirror."). SSH signing stays operator-default.
// Renamed 2026-07-28 (Migration 6 slice): `dispatch_arm_collapse` →
// `mend_at` per Mara `9bb1f57` twelve-primitive revision register
// (mend = constitutive; dispatch retired at rust/ altitude). Full lift
// into `roomba::mend::at` deferred until phone.rs @io helpers collapse
// to std::fs at roomba altitude (Migration 7). Semantics identical.
fn mend_at(
    substrate_root: &std::path::Path,
    rs_path: &std::path::Path,
    corpus: &std::collections::HashMap<String, collapse::BilateralDecl>,
) -> Result<collapse::CollapseReport, String> {
    let source = phone::read_file(rs_path)
        .map_err(|e| format!("@io/fs.read {}: {}", rs_path.display(), e))?;
    let arms = collapse::find_redundant_arms(&source, corpus);
    let bytes_before = source.len();

    if arms.is_empty() {
        return Ok(collapse::CollapseReport {
            target: rs_path.to_path_buf(),
            arms,
            bytes_before,
            bytes_after: bytes_before,
            commit_oid: None,
        });
    }

    let mended = collapse::apply(&source, &arms);
    let bytes_after = mended.len();

    phone::write_file(rs_path, &mended)
        .map_err(|e| format!("@io/fs.write {}: {}", rs_path.display(), e))?;

    // Determine the enclosing git repo. The vacuum may target a
    // directory in a repo distinct from `substrate_root` (test
    // corpora, etc.). Walk up from the .rs file looking for `.git/`.
    let repo_root = find_git_root(rs_path).ok_or_else(|| {
        format!(
            "no enclosing git repo for {} — cannot commit",
            rs_path.display()
        )
    })?;

    phone::git_add(&repo_root, rs_path)
        .map_err(|e| format!("@io/git.add {}: {}", rs_path.display(), e))?;

    let message = compose_collapse_commit_message(rs_path, &arms, bytes_before, bytes_after);

    let mirror_subject = fractal::Subject::mirror();
    let commit_oid = phone::git_commit_as(
        &repo_root,
        &mirror_subject,
        &mirror_subject,
        &message,
    )
    .map_err(|e| format!("@io/git.commit: {}", e))?;

    let _ = substrate_root; // reserved for future reflective composer.

    Ok(collapse::CollapseReport {
        target: rs_path.to_path_buf(),
        arms,
        bytes_before,
        bytes_after,
        commit_oid: Some(commit_oid),
    })
}

/// Walk upward from `start` looking for a directory containing `.git`.
fn find_git_root(start: &std::path::Path) -> Option<std::path::PathBuf> {
    let mut cur = start.canonicalize().ok()?;
    if cur.is_file() {
        cur.pop();
    }
    loop {
        if cur.join(".git").exists() {
            return Some(cur);
        }
        if !cur.pop() {
            return None;
        }
    }
}

/// Compose the commit message body naming what was retired. Direct
/// format at rust/ altitude; substrate-honest lift to `@nl.compose`
/// dispatch is a follow-up tick once the composer is wired at rust/
/// altitude (M7+).
fn compose_collapse_commit_message(
    target: &std::path::Path,
    arms: &[collapse::RedundantArm],
    bytes_before: usize,
    bytes_after: usize,
) -> String {
    let arm_list = arms
        .iter()
        .map(|a| format!("- {} (sentinel: {})", a.action_ref, a.sentinel))
        .collect::<Vec<_>>()
        .join("\n");
    let delta = bytes_before.saturating_sub(bytes_after);
    format!(
        "\u{1F9F9} mirror [substrate-floor:@io-boundary] bilateral-arm collapse — retired {n} hand-typed arm{s} shadowed by reflective corpus\n\
         \n\
         @kintsugi/fracture/bilateral_arm_redundant.collapse discharged {n} arm{s} from {file}:\n\
         \n\
         {arms}\n\
         \n\
         Byte delta: -{delta} bytes ({before} → {after}).\n\
         \n\
         Retirement invariant per math foundation \
         `docs/math/kintsugi/fracture/bilateral-arm-redundant.md`:\n\
         - sbec preserved (reflective corpus dispatches same verdicts)\n\
         - rust_loc strictly decreased\n\
         - io_violations = 0 (no new @io introduced)\n\
         \n\
         Composition (substrate dispatch chain, rust/ altitude):\n\
         - `collapse::find_redundant_arms` detected arms via byte-analysis\n\
         - `phone::write_file` (@io/fs.write) applied the deletion on disk\n\
         - `phone::git_commit_as` (@io/git.commit) crossed the @io boundary\n\
         \n\
         Audit chain:\n\
         - Mara `81294b3` §7.4 dispatch matrix (walker's fracture table)\n\
         - Seam `9c34ec4` M0 gating ratification\n\
         - Seam `c1775f1` stigmergy witnessed computation 12/12 SHIP\n\
         - this commit — first arm-collapse from rust/ terminal floor\n\
         \n\
         The compiler authored this commit itself per Alex 2026-07-16 /loop directive:\n\
         \"That's the roomba commit diffs I wanna see. Deleted Rust. Added mirror.\"\n",
        n = arms.len(),
        s = if arms.len() == 1 { "" } else { "s" },
        file = target.display(),
        arms = arm_list,
        delta = delta,
        before = bytes_before,
        after = bytes_after,
    )
}

/// The @-operator dispatch surface at rust/ altitude — the sidestep
/// through rust/ per Alex 2026-07-18 direct-transcript ("bootstrap/
/// is going to die soon"). Recognizes `@io/<family>.<action>` action-
/// refs and routes them to phone.rs primitives at the terminal FLOOR.
///
/// This is the pattern shard bodies (Mara authorship territory) will
/// eventually compose over. Instead of per-species resolver arms in
/// bootstrap/apply_h.rs (which grows the FLOOR contrary to ouroboros),
/// EVERY new species with an @io-composing body dispatches through
/// this ONE function — no arm-mint required per new species.
///
/// String-in, `Result<String, String>` out — simplest possible
/// carrier at foothold altitude. Args are positional; each route
/// documents its expected arity. Typed carriers (Path, Subject,
/// bytes) get wrapped/unwrapped as strings until the reflective
/// body-composition runtime (currently forward-promised) lands a
/// typed dispatch surface.
///
/// Landed routes (5 @io/fs primitives; @io/git deferred pending
/// fractal::Subject string-serialization decision):
///   `@io/fs.list_dir(path)`         → list of entry paths (LF-joined)
///   `@io/fs.read(path)`             → file contents (UTF-8)
///   `@io/fs.write(path, contents)`  → "" on success
///   `@io/fs.append(path, contents)` → "" on success
///   `@io/fs.mkdir_p(path)`          → "" on success
///
/// Errors surface as `Err(String)` describing the @io failure. This
/// mirrors phone.rs's `io::Result<_>` shape at the string-carrier
/// altitude.
pub(crate) fn at_operator(action_ref: &str, args: &[String]) -> Result<String, String> {
    use std::path::Path;

    match action_ref {
        "@io/fs.list_dir" => {
            let path = args
                .first()
                .ok_or_else(|| "@io/fs.list_dir: expected 1 arg (path)".to_string())?;
            let entries = phone::list_dir_recursive(Path::new(path))
                .map_err(|e| format!("@io/fs.list_dir({}): {}", path, e))?;
            Ok(entries
                .iter()
                .map(|e| e.path.display().to_string())
                .collect::<Vec<_>>()
                .join("\n"))
        }
        "@io/fs.read" => {
            let path = args
                .first()
                .ok_or_else(|| "@io/fs.read: expected 1 arg (path)".to_string())?;
            phone::read_file(Path::new(path))
                .map_err(|e| format!("@io/fs.read({}): {}", path, e))
        }
        "@io/fs.write" => {
            let path = args
                .first()
                .ok_or_else(|| "@io/fs.write: expected 2 args (path, contents)".to_string())?;
            let contents = args
                .get(1)
                .ok_or_else(|| "@io/fs.write: expected 2 args (path, contents)".to_string())?;
            phone::write_file(Path::new(path), contents)
                .map(|_| String::new())
                .map_err(|e| format!("@io/fs.write({}): {}", path, e))
        }
        "@io/fs.append" => {
            let path = args
                .first()
                .ok_or_else(|| "@io/fs.append: expected 2 args (path, contents)".to_string())?;
            let contents = args
                .get(1)
                .ok_or_else(|| "@io/fs.append: expected 2 args (path, contents)".to_string())?;
            phone::append_to(Path::new(path), contents)
                .map(|_| String::new())
                .map_err(|e| format!("@io/fs.append({}): {}", path, e))
        }
        "@io/fs.mkdir_p" => {
            let path = args
                .first()
                .ok_or_else(|| "@io/fs.mkdir_p: expected 1 arg (path)".to_string())?;
            phone::mkdir_p(Path::new(path))
                .map(|_| String::new())
                .map_err(|e| format!("@io/fs.mkdir_p({}): {}", path, e))
        }
        "@io/git.commit" => {
            // Wired route — COORD-4 landing per Taut `3787770` +
            // Mara `defc8ef` + Alex 2026-07-22 direction ("make the
            // liquid flow"). Extends the earlier stub (which staked
            // the dispatch shape) into an actual routing to
            // phone::git_commit_as via a HARDCODED WELL-KNOWN OID
            // MAP for @peer/mirror + @peer/void. Arbitrary @peer
            // OIDs still surface the Mara @peer/registry authorship
            // boundary — partial-solve substrate-honestly named.
            //
            // The well-known map is Rice-safe: two literal string
            // matches over fractal::Subject constructors that need
            // no external state. Mara's @peer/registry species-decl
            // + @trust family-root mint will replace this hardcoded
            // map with arbitrary-peer OID lookup when landed.
            //
            // Signature: 4 args = (author_oid, committer_oid,
            // repo_root_path, message). Explicit repo_root avoids
            // CWD-magic; the switchboard IS the switchboard, not
            // an implicit-context leaker.
            let author_oid = args
                .first()
                .ok_or_else(|| "@io/git.commit: expected 4 args (author_oid, committer_oid, repo_root, message)".to_string())?;
            let committer_oid = args
                .get(1)
                .ok_or_else(|| "@io/git.commit: expected 4 args (author_oid, committer_oid, repo_root, message)".to_string())?;
            let repo_root = args
                .get(2)
                .ok_or_else(|| "@io/git.commit: expected 4 args (author_oid, committer_oid, repo_root, message)".to_string())?;
            let message = args
                .get(3)
                .ok_or_else(|| "@io/git.commit: expected 4 args (author_oid, committer_oid, repo_root, message)".to_string())?;

            let author_subject = book::resolve(author_oid).map_err(|e| {
                format!(
                    "@io/git.commit: {}. Callers with typed fractal::Subject values SHOULD use phone::git_commit_as directly.",
                    e
                )
            })?;
            let committer_subject = book::resolve(committer_oid).map_err(|e| {
                format!(
                    "@io/git.commit: {}. Callers with typed fractal::Subject values SHOULD use phone::git_commit_as directly.",
                    e
                )
            })?;

            phone::git_commit_as(
                Path::new(repo_root),
                &author_subject,
                &committer_subject,
                message,
            )
            .map_err(|e| format!("@io/git.commit: {}", e))
        }
        _ => Err(format!(
            "@-operator: unknown action-ref `{}` (landed: @io/fs.{{list_dir,read,write,append,mkdir_p}} + @io/git.commit[{}]; arbitrary @<name> pending Mara @peer/registry + @trust family-root general-purpose lookup)",
            action_ref,
            book::well_known_at_names().join("|"),
        )),
    }
}

#[cfg(test)]
mod at_operator_tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn unknown_action_ref_returns_err_with_landed_list() {
        let result = at_operator("@io/fs.unknown", &[]);
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(msg.contains("unknown action-ref"));
        assert!(msg.contains("list_dir"));
        // Post-book.rs: dynamically enumerated well-known @<name> set.
        assert!(msg.contains("@peer/mirror"));
        assert!(msg.contains("@peer/void"));
        assert!(msg.contains("@peer/reed"));
        assert!(msg.contains("@peer/mara"));
        assert!(msg.contains("@human/alex"));
        assert!(msg.contains("@peer/registry"));
    }

    #[test]
    fn fs_read_round_trips_through_tempfile() {
        let td = tempfile::tempdir().unwrap();
        let f = td.path().join("round-trip.txt");
        std::fs::File::create(&f)
            .unwrap()
            .write_all(b"hello mirror")
            .unwrap();
        let result = at_operator(
            "@io/fs.read",
            &[f.display().to_string()],
        )
        .unwrap();
        assert_eq!(result, "hello mirror");
    }

    #[test]
    fn fs_write_then_read_composes() {
        let td = tempfile::tempdir().unwrap();
        let f = td.path().join("composed.txt");
        at_operator(
            "@io/fs.write",
            &[f.display().to_string(), "substrate flows".to_string()],
        )
        .unwrap();
        let read_back = at_operator(
            "@io/fs.read",
            &[f.display().to_string()],
        )
        .unwrap();
        assert_eq!(read_back, "substrate flows");
    }

    #[test]
    fn fs_mkdir_p_is_idempotent() {
        let td = tempfile::tempdir().unwrap();
        let nested = td.path().join("a/b/c/d");
        let arg = nested.display().to_string();
        at_operator("@io/fs.mkdir_p", &[arg.clone()]).unwrap();
        at_operator("@io/fs.mkdir_p", &[arg]).unwrap();
        assert!(nested.is_dir());
    }

    #[test]
    fn missing_arg_surfaces_arity_error() {
        let result = at_operator("@io/fs.read", &[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expected 1 arg"));
    }

    #[test]
    fn git_commit_at_operator_consumes_book_registry_for_well_known_pack_peer() {
        // Post-COORD-5 (book.rs extraction): the at_operator @io/git.commit
        // arm consumes book::resolve for @<name> → Subject lookup. Well-
        // known Pack peers (@peer/reed, @peer/mara, etc.) now resolve
        // successfully at the switchboard boundary; the previous COORD-4
        // hardcoded 2-well-known map is retired. This test pins the shape
        // by attempting a @peer/reed authored commit through the switchboard
        // in a tempdir repo and asserting real 40-char SHA-1 OID return.
        //
        // Skip if git binary unavailable.
        if std::process::Command::new("git")
            .arg("--version")
            .output()
            .map(|o| !o.status.success())
            .unwrap_or(true)
        {
            return;
        }
        let td = tempfile::tempdir().unwrap();
        let repo_path = td.path();
        let git = |args: &[&str]| {
            std::process::Command::new("git")
                .args(args)
                .current_dir(repo_path)
                .env("GIT_CONFIG_GLOBAL", "/dev/null")
                .env("GIT_CONFIG_SYSTEM", "/dev/null")
                .output()
                .unwrap()
        };
        git(&["init", "-q"]);
        git(&["config", "commit.gpgsign", "false"]);
        git(&["config", "core.hooksPath", "/dev/null"]);
        std::fs::write(repo_path.join("seed.txt"), "pack-peer commit").unwrap();
        git(&["add", "seed.txt"]);
        let commit_oid = at_operator(
            "@io/git.commit",
            &[
                "@peer/reed".to_string(),
                "@peer/mirror".to_string(),
                repo_path.display().to_string(),
                "pack-peer through book+phone switchboard".to_string(),
            ],
        )
        .unwrap();
        assert_eq!(commit_oid.len(), 40, "expected 40-char SHA-1 OID");
        assert!(commit_oid.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn git_commit_error_from_book_wraps_with_switchboard_context() {
        // When book::resolve fails, at_operator wraps the error with
        // @io/git.commit prefix + phone::git_commit_as hint for typed-
        // Subject callers. Substrate-honest failure surface at the
        // dispatch site altitude.
        let td = tempfile::tempdir().unwrap();
        let result = at_operator(
            "@io/git.commit",
            &[
                "@peer/nonexistent".to_string(),
                "@peer/mirror".to_string(),
                td.path().display().to_string(),
                "test".to_string(),
            ],
        );
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(msg.contains("@io/git.commit"));
        assert!(msg.contains("@peer/nonexistent"));
        assert!(msg.contains("@peer/registry"));
        assert!(msg.contains("phone::git_commit_as"));
    }

    #[test]
    fn git_commit_at_operator_end_to_end_in_tempdir_repo() {
        // Full end-to-end: at_operator dispatches through the
        // switchboard — resolve well-known @peer/mirror OID →
        // phone::git_commit_as → real git commit in tempdir.
        // Liquid flows through the switchboard for git-commit
        // altitude for the first time.
        //
        // Skip if git binary unavailable.
        if std::process::Command::new("git")
            .arg("--version")
            .output()
            .map(|o| !o.status.success())
            .unwrap_or(true)
        {
            return;
        }

        let td = tempfile::tempdir().unwrap();
        let repo_path = td.path();

        // Init repo with per-repo signing + hooks disabled (test
        // isolation from operator SSH signing + commit-msg hooks).
        std::process::Command::new("git")
            .args(["init", "-q", "--initial-branch=main"])
            .current_dir(repo_path)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "commit.gpgsign", "false"])
            .current_dir(repo_path)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "core.hooksPath", "/dev/null"])
            .current_dir(repo_path)
            .output()
            .unwrap();

        // Seed + stage a file.
        let file = repo_path.join("seed.txt");
        std::fs::write(&file, "end2end at_operator @io/git.commit").unwrap();
        std::process::Command::new("git")
            .args(["add", "seed.txt"])
            .current_dir(repo_path)
            .output()
            .unwrap();

        // The load-bearing call: at_operator switchboard dispatches
        // @io/git.commit → resolve @peer/mirror → phone::git_commit_as.
        let commit_oid = at_operator(
            "@io/git.commit",
            &[
                "@peer/mirror".to_string(),
                "@peer/mirror".to_string(),
                repo_path.display().to_string(),
                "end2end switchboard test commit".to_string(),
            ],
        )
        .unwrap();

        assert_eq!(commit_oid.len(), 40, "expected 40-char SHA-1 OID");
        assert!(commit_oid.chars().all(|c| c.is_ascii_hexdigit()));

        // Verify commit landed with @peer/mirror author identity.
        let log_out = std::process::Command::new("git")
            .args(["log", "--format=%an|%ae|%s", "-1"])
            .current_dir(repo_path)
            .output()
            .unwrap();
        let log_line = String::from_utf8_lossy(&log_out.stdout);
        let line = log_line.trim();
        assert!(line.contains("end2end switchboard test commit"));

        // Also verify @peer/void resolves + commits.
        let file2 = repo_path.join("seed2.txt");
        std::fs::write(&file2, "void commit").unwrap();
        std::process::Command::new("git")
            .args(["add", "seed2.txt"])
            .current_dir(repo_path)
            .output()
            .unwrap();
        let void_oid = at_operator(
            "@io/git.commit",
            &[
                "@peer/void".to_string(),
                "@peer/void".to_string(),
                repo_path.display().to_string(),
                "void commit through switchboard".to_string(),
            ],
        )
        .unwrap();
        assert_eq!(void_oid.len(), 40);
        assert_ne!(void_oid, commit_oid, "distinct commits must have distinct OIDs");
    }

    #[test]
    fn git_commit_at_operator_requires_four_args() {
        // Arity discipline: (author_oid, committer_oid,
        // repo_root, message) per COORD-4 signature update.
        let result = at_operator("@io/git.commit", &["@peer/mirror".to_string()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expected 4 args"));
    }
}

/// `mirror craft <spec-file>` — build the artifact declared by the
/// spec. First empirical ouroboros closure at the ORCHESTRATOR altitude:
/// mirror reads its own spec + invokes cargo to produce the mirror
/// binary.
///
/// Per Alex 2026-07-23 in-transcript directive: "get mirror actually
/// built from mirror.spec. The ouroboros wants to close." Composes
/// over Taut `173a1204` §6 Bootstrap Kernel reframe (LANDED as
/// property-verification; ASPIRATIONAL as self-compiling reflective
/// evaluator) — this tick lands the orchestrator-altitude closure
/// without discharging the full source-to-binary Foerster-fixpoint.
///
/// MVP scope: Rice-safe byte-check on the spec to verify it declares
/// a @code/rust binary target; shell out to `cargo build --bin <name>`
/// in the sibling `rust/` directory via `phone::spawn_cargo_build`.
/// Full self-hosting (mirror emits its own Rust source + retires
/// cargo) is Mara §5.2 M2+ territory forward-promised.
///
/// The Foerster-fixpoint chicken-and-egg PARTIALLY CLOSES here at
/// orchestrator altitude — mirror is now the invoked orchestrator of
/// its own build, even if the compilation still happens through cargo.
///
/// Exit codes:
///   0 — spec verified + cargo build succeeded
///   2 — usage error (missing spec-file; spec doesn't declare
///        @code/rust binary target via cargo)
///   3 — @io error (spec read failed; cargo workspace missing;
///        cargo spawn failed; cargo returned non-zero)
fn cmd_craft(rest: &[String]) -> ExitCode {
    let spec_arg = match rest.first() {
        Some(s) => s.as_str(),
        None => {
            eprintln!("mirror craft: <spec-file> required");
            eprintln!();
            eprintln!("Usage: mirror craft <spec-file>");
            eprintln!();
            eprintln!("The spec declares a target binary at @code/rust altitude");
            eprintln!("(e.g. ./mirror.spec's variety.emits.binary block).");
            return ExitCode::from(2);
        }
    };

    let spec_path = std::path::Path::new(spec_arg);
    let spec_source = match phone::read_file(spec_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("mirror craft: @io/fs.read {}: {}", spec_arg, e);
            return ExitCode::from(3);
        }
    };

    // Rice-safe byte-check: verify the spec declares a @code/rust
    // binary target via cargo. Substrate-honest partial-parse; full
    // grammar parsing lands with the reflective evaluator (Mara
    // §5.2 M2). Accepts both `system @<name>` (post-VSM-transition)
    // and legacy `project <name>` grammars per Mara two-tick
    // alias-shim discipline (c6eb2d8).
    let has_container_block =
        spec_source.contains("system @") || spec_source.contains("project ");
    if !has_container_block {
        eprintln!(
            "mirror craft: {} does not declare a `system @<name>` or `project <name>` block",
            spec_arg
        );
        return ExitCode::from(2);
    }
    // Dual-byte-check backward-compat window per Seam Phase D 1893afc
    // SHARPENING-2 during @code → @facet rename (Phase 3b landed
    // 2026-08-21): accept BOTH `altitude @code/rust` (legacy) and
    // `altitude @facet/rust` (post-rename canonical). Window closes
    // when all downstream spec files migrate to @facet/rust.
    let has_rust_target =
        (spec_source.contains("altitude @code/rust")
            || spec_source.contains("altitude @facet/rust"))
            && (spec_source.contains("cargo"));
    if !has_rust_target {
        eprintln!(
            "mirror craft: {} does not declare a `@facet/rust` (or legacy `@code/rust`) binary target via cargo",
            spec_arg
        );
        return ExitCode::from(2);
    }

    // Extract binary name from the spec — Rice-safe grep for `name "..."`.
    // MVP: default to "mirror" if extraction fails; full field-
    // extraction lands with reflective evaluator.
    let bin_name = extract_binary_name(&spec_source).unwrap_or_else(|| "mirror".to_string());

    // Determine cargo working directory: sibling `rust/` to the spec.
    // MVP assumes the terminal-geometry three-file layout (Mara
    // `81294b3` + task #237 M0 scaffold).
    let spec_dir = spec_path.parent().unwrap_or(std::path::Path::new("."));
    let rust_dir = spec_dir.join("rust");
    if !rust_dir.is_dir() {
        eprintln!(
            "mirror craft: cargo workspace not found at {}",
            rust_dir.display()
        );
        eprintln!("MVP assumes sibling `rust/` directory to the spec.");
        return ExitCode::from(3);
    }

    // Shell out to cargo. THIS IS THE OUROBOROS CLOSURE at orchestrator
    // altitude — mirror invokes cargo to build itself.
    eprintln!(
        "mirror craft: building `{}` at @code/rust via cargo in {}",
        bin_name,
        rust_dir.display()
    );
    let status = match phone::spawn_cargo_build(&rust_dir, &bin_name) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("mirror craft: cargo spawn failed: {}", e);
            return ExitCode::from(3);
        }
    };

    if !status.success() {
        eprintln!(
            "mirror craft: cargo build failed with exit code {}",
            status.code().unwrap_or(-1)
        );
        return ExitCode::from(3);
    }

    let artifact = rust_dir.join("target").join("debug").join(&bin_name);
    println!(
        "mirror craft: settled `{}` to binary at {}",
        bin_name,
        artifact.display()
    );
    ExitCode::SUCCESS
}

/// Extract the binary name from the first `name "..."` line inside
/// the spec source. Rice-safe byte-substring extraction; returns
/// None if no matching line found.
fn extract_binary_name(source: &str) -> Option<String> {
    for line in source.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("name ") {
            let rest = rest.trim();
            if let Some(inner) = rest.strip_prefix('"') {
                if let Some(end) = inner.find('"') {
                    return Some(inner[..end].to_string());
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod craft_tests {
    use super::*;

    #[test]
    fn craft_requires_spec_arg() {
        assert_eq!(cmd_craft(&[]), ExitCode::from(2));
    }

    #[test]
    fn craft_errors_on_missing_spec_file() {
        let args = vec!["/nonexistent/path/to/mirror.spec".to_string()];
        assert_eq!(cmd_craft(&args), ExitCode::from(3));
    }

    #[test]
    fn craft_errors_on_non_container_spec() {
        let tmp = tempfile::tempdir().unwrap();
        let spec = tmp.path().join("bad.spec");
        std::fs::write(&spec, "# no system block here\n").unwrap();
        let args = vec![spec.display().to_string()];
        assert_eq!(cmd_craft(&args), ExitCode::from(2));
    }

    #[test]
    fn craft_errors_on_spec_without_rust_target() {
        let tmp = tempfile::tempdir().unwrap();
        let spec = tmp.path().join("non_rust.spec");
        std::fs::write(
            &spec,
            "system @foo { variety { emits [ ci { altitude @ci/github emit yaml } ] } }\n",
        )
        .unwrap();
        let args = vec![spec.display().to_string()];
        assert_eq!(cmd_craft(&args), ExitCode::from(2));
    }

    #[test]
    fn extract_binary_name_finds_first_name_line() {
        let source = r#"system @mirror {
  variety {
    emits [
      binary {
        name     "mirror"
        altitude @code/rust
        emit     cargo
      }
    ]
  }
}
"#;
        assert_eq!(extract_binary_name(source), Some("mirror".to_string()));
    }

    #[test]
    fn extract_binary_name_returns_none_when_absent() {
        let source = "system @nameless { variety { } }\n";
        assert_eq!(extract_binary_name(source), None);
    }

    #[test]
    fn craft_errors_on_missing_cargo_workspace() {
        let tmp = tempfile::tempdir().unwrap();
        let spec = tmp.path().join("mirror.spec");
        std::fs::write(
            &spec,
            r#"system @mirror {
  variety {
    emits [
      binary {
        name     "mirror"
        altitude @code/rust
        emit     cargo
      }
    ]
  }
}
"#,
        )
        .unwrap();
        // No `rust/` sibling directory created — should exit 3.
        let args = vec![spec.display().to_string()];
        assert_eq!(cmd_craft(&args), ExitCode::from(3));
    }
}

/// `mirror compile <file>` — the SAGA-chain-of-Crystals compilation
/// verb. Delegates orchestration to `compile::compile_from_source`
/// (iter 3); crosses @io via `phone::read_file`. This is the
/// substrate-honest thin-delegation shape per Alex 2026-07-20 cascade
/// brief: main.rs owns argv-parse + reporting; compile.rs owns the
/// SAGA loop; phone.rs owns the @io crossing.
///
/// Exit codes:
///   0 — compilation succeeded; SAGA chain printed. Includes
///        Continue-escalation OR Escalate(oid) OR Halt(msg) tiers
///        — they are compile-time projections, not exit-code
///        classifications (the caller decides what to do with an
///        Escalate; the verb itself returns 0 for a completed loop).
///   2 — usage error (missing <file>).
///   3 — @io error (file read failure).
fn cmd_compile(rest: &[String]) -> ExitCode {
    let file = match rest.first() {
        Some(f) => f,
        None => {
            eprintln!("mirror compile: <file> is required");
            eprintln!();
            eprintln!("Usage: mirror compile <file>");
            eprintln!();
            eprintln!("Compiles the bilateral property declarations in <file>");
            eprintln!("through the SAGA-chain-of-Crystals loop (compile.rs");
            eprintln!("iter 3; composes over liquid.rs iter 2 + fractal iter 1).");
            return ExitCode::from(2);
        }
    };

    let path = std::path::Path::new(file);
    let source = match phone::read_file(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("mirror compile: @io/fs.read {}: {}", file, e);
            return ExitCode::from(3);
        }
    };

    // Witnessed identity for the crystallization chain — mirror as
    // both Author and Committer at compile-verb altitude (the caller
    // supplies typed Subjects when the reflective @-operator dispatch
    // matures; at iter 4 the verb defaults to Subject::mirror() for
    // both roles per fractal step 9 shape).
    let mirror_subject = fractal::Subject::mirror();
    let witnessed = fractal::Witnessed::new(
        mirror_subject.as_author(),
        mirror_subject.as_committer(),
        fractal::Timestamp(current_utc_timestamp()),
    );

    let comp = compile::compile_from_source(&source, &witnessed);

    println!("mirror compile: SAGA chain of {} crystal(s) from {}", comp.depth(), file);
    println!();
    for (i, (crystal, discharge)) in comp.crystals.iter().zip(comp.discharges.iter()).enumerate() {
        let verdict_tag = if discharge.verdict.is_pass() {
            "pass  "
        } else if discharge.verdict.is_fail() {
            "fail  "
        } else {
            "defer "
        };
        let oid_prefix: String = crystal.oid.0.iter().take(8).map(|b| format!("{:02x}", b)).collect();
        println!("  [{:>3}] {} {} — {}", i, verdict_tag, oid_prefix, discharge.property_name);
    }

    println!();
    match &comp.escalation {
        compile::Escalation::Continue => {
            println!("escalation: Continue (all discharges reflected or redirected)");
        }
        compile::Escalation::Escalate(oid) => {
            let prefix: String = oid.0.iter().take(8).map(|b| format!("{:02x}", b)).collect();
            println!("escalation: Escalate(crystal={}) — @peer.redirect walk-target", prefix);
        }
        compile::Escalation::Halt(msg) => {
            println!("escalation: Halt({})", msg);
        }
    }

    ExitCode::SUCCESS
}

/// Hand-rolled argv dispatch. Deliberately does NOT reach for clap; the
/// M2 reflective cli-block reader (Mara §2.2) IS the real dispatch
/// surface. Adding clap now would breed abstraction the substrate has
/// not asked for (Loki `b53aeeb` §5 cut #2).
///
/// Exit codes:
///   0 — `--help` / `--version` / `roomba --vacuum=<dir>` returned cleanly.
///   2 — usage error (unknown verb, missing verb, malformed argv).
///       Verbs `compile`..`index` return 2 at M-vacuum because their
///       dispatch lands at M3+ (Mara §2.2 M3 first end-to-end verb).
///   3 — @io error (only from `roomba --vacuum` when the walker fails).
fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    // argv[0] is the binary path; verb is argv[1].
    match args.get(1).map(String::as_str) {
        Some("--help") | Some("-h") | Some("help") | None => {
            print_help();
            ExitCode::SUCCESS
        }
        Some("--version") | Some("-V") | Some("version") => {
            print_version();
            ExitCode::SUCCESS
        }
        Some("roomba") => {
            // M-vacuum empirical firing. Rest of argv is passed to the
            // roomba dispatch arm for flag parsing.
            let rest: Vec<String> = args.iter().skip(2).cloned().collect();
            cmd_roomba(&rest)
        }
        Some("serve") => {
            // Reed 2026-08-03 Alex Option C Phase A nearly-today
            // delegation stub. `mirror serve --mcp` execs bootstrap
            // binary as MCP transport per bin/mirror-mcp shape at
            // rust/ altitude. TRANSITIONAL bridge — Phase B replaces
            // with rust/src/mcp.rs FLOOR emitter per Mara M4 canonical
            // spec forward-promise (docs/specs/2026-08-03-mara-rust-
            // mcp-floor-lift-m4-canonical-spec.md; agent a8842c6
            // this-tick).
            let rest: Vec<String> = args.iter().skip(2).cloned().collect();
            cmd_serve_mcp(&rest)
        }
        Some("craft") => {
            // Ouroboros closure at orchestrator altitude (Alex 2026-07-23
            // directive "get mirror actually built from mirror.spec").
            // MVP: read spec + verify @code/rust binary target + shell
            // out to cargo build. Full self-hosting forward-promised.
            let rest: Vec<String> = args.iter().skip(2).cloned().collect();
            cmd_craft(&rest)
        }
        Some("compile") => {
            // /loop iter-4 empirical firing per Alex 2026-07-20 cascade
            // brief item 5 ("main.rs refactor — thin to delegation").
            // main.rs holds argv parsing + delegates the SAGA loop
            // orchestration to compile.rs (iter 3); phone.rs supplies
            // the @io/fs.read crossing. The verb is substrate-honestly
            // thin: parse → read → compile → report.
            let rest: Vec<String> = args.iter().skip(2).cloned().collect();
            cmd_compile(&rest)
        }
        Some(other) => {
            // Every named verb is substrate-decl'd but dispatch lands
            // at M3+ per Mara §2.2. Return exit 2 with a substrate-
            // honest message pointing at the FLOOR that will land it.
            let is_known = VERBS.iter().any(|(v, _)| {
                // Match "peer" as a prefix for the two peer-nested verbs.
                // Note: "compile" is dispatched above; if it reaches
                // this arm it's a substrate-decl'd verb not yet wired.
                *v == other || other == "peer"
            });
            if is_known {
                eprintln!(
                    "mirror: verb `{}` is substrate-decl'd but dispatch \
                     lands at M3+ (see docs/specs/rust-floor-birthed-by-\
                     roomba-from-mirror-spec.md §2.2).",
                    other
                );
            } else {
                eprintln!("mirror: unknown verb `{}`", other);
                eprintln!();
                eprintln!("Run `mirror --help` for the verb list.");
            }
            ExitCode::from(2)
        }
    }
}
