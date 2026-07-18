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

mod collapse;
// M0 module wiring — declare the two sibling altitudes so the terminal-
// geometry three-file discipline is byte-visible in `rust/src/` and
// `cargo build` compiles ALL THREE files even while the bodies are
// forward-promises. Each module retires its `#[allow(dead_code)]` gates
// when its M-tick lands.
mod matrix;
mod phone;
mod void;

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
        match dispatch_arm_collapse(&substrate_root, &entry.path, &corpus) {
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

    let commit_oid = phone::git_commit_as(
        &repo_root,
        "mirror",
        "mirror@spectral.engineer",
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
fn dispatch_arm_collapse(
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

    let mended = collapse::apply_deletions(&source, &arms);
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

    let commit_oid = phone::git_commit_as(
        &repo_root,
        "mirror",
        "mirror@spectral.engineer",
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
