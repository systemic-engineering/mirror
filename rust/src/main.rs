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

// M0 module wiring — declare the two sibling altitudes so the terminal-
// geometry three-file discipline is byte-visible in `rust/src/` and
// `cargo build` compiles ALL THREE files even while the bodies are
// forward-promises. Each module retires its `#[allow(dead_code)]` gates
// when its M-tick lands.
mod matrix;
mod phone;

use std::env;
use std::process::ExitCode;

/// Package version — mirrors Cargo.toml `[package].version`.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The 10 verbs from `mirror.spec:82-334` cli-block (per Taut `7f4307f`
/// §Q5). HARDCODED at M0; retires at M2 (reflective cli-block reading
/// from `mirror.spec` per Mara §2.2). Ordering matches spec byte-order:
/// compile / kintsugi / shatter / craft / init / recall / beam /
/// peer beam (depth-2 grammar) / peer contribute / index.
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

/// Hand-rolled argv dispatch. Deliberately does NOT reach for clap; the
/// M2 reflective cli-block reader (Mara §2.2) IS the real dispatch
/// surface. Adding clap now would breed abstraction the substrate has
/// not asked for (Loki `b53aeeb` §5 cut #2).
///
/// Exit codes:
///   0 — `--help` or `--version` returned cleanly.
///   2 — usage error (unknown verb, missing verb, malformed argv).
///       Verbs `compile`..`index` return 2 at M0 because dispatch
///       lands at M3+ (Mara §2.2 M3 first end-to-end verb).
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
        Some(other) => {
            // Every named verb is substrate-decl'd but dispatch lands
            // at M3+ per Mara §2.2. Return exit 2 with a substrate-
            // honest message pointing at the FLOOR that will land it.
            let is_known = VERBS.iter().any(|(v, _)| {
                // Match "peer" as a prefix for the two peer-nested verbs.
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
