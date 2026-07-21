//! `phone.rs` — the @io socket-handover altitude.
//!
//! Per Mara `81294b3` §3 (terminal-geometry canonical spec, ratified
//! Seam `9c34ec4`) + Loki `b53aeeb` §4 (matrix.rs knife-cut essay,
//! phenomenological grounding): phone.rs is the Matrix phone booth.
//! The ONE place in `rust/` the substrate crosses out of itself and
//! back.
//!
//! > phone.rs "handles the CONNECTIONS between actors. Each connection
//! > carries a state that eventually needs a matrix operation applied
//! > to it. phone.rs doesn't do that operation; it hands the state to
//! > matrix.rs, which hands it to Fortran, which hands the result back."
//! > — Loki `b53aeeb` §4
//!
//! ## M0 surface (this file)
//!
//! MODULE STUB. Per Mara §2.2 M0 milestone: empty @io stub. Signatures
//! declared as forward-promises; no bodies. Empirical firing lands at
//! M2+ (MCP handshake alive: Mara §2.2 M4) and M8 (peer socket boot).
//!
//! ## Forward-promises (M2+ ticks; not implemented here)
//!
//! - `read_stdin` / `write_stdout` — JSON-RPC framing over stdio for
//!   `@mcp.serve` sentinel; M4 milestone.
//! - `@io/socket` accept / read / write — peer socket boot for
//!   `mirror peer beam`; M8 milestone. Per Taut `7f4307f` §Q2:
//!   boot-altitude declaration at `boot/std/io/socket.mirror` is
//!   empirically sufficient for M4 stdio (stdin/stdout are @io/bytes
//!   streams, not socket connections); mirror-altitude lift-tick for
//!   `@io/socket` fires at M8 when peer beam opens TCP/UnixStream.
//! - `@io/git` process spawn + pipe management — M6 co-tick when
//!   `roomba --commit` composes `@nl.compose + @io/git.commit`.
//! - `@io/fs` file descriptor management — M3 first empirical verb
//!   (`mirror compile <file>`).
//! - The `unsafe extern "C"` process/socket/fd plumbing boundary —
//!   phone.rs's `unsafe` is @io plumbing; matrix.rs's `unsafe` is
//!   LAPACK/BLAS numerical FFI (per Mara §3.2 item 4 + §4.2 item 4).
//!
//! ## What phone.rs does NOT hold (per Mara §3.3)
//!
//! - Numerical computation → matrix.rs.
//! - Actor supervision → main.rs.
//! - Grammar / parsing → main.rs reflective read of `shards/**/*.mirror`.
//! - Per-prism business logic → shard-body + @io lift.
//!
//! ## Composition anchors (LANDED)
//!
//! - `shards/io.mirror` (T21 family root + landed sub-species
//!   `@io/fs`, `@io/git`, `@io/socket`, `@io/network`, `@io/bytes`)
//!   — Mara §3.4 composition anchor.
//! - `boot/std/io/socket.mirror:1-105` — boot-altitude socket
//!   primitives; two opaque handle types (`connection`, `listener`);
//!   actions `read_bytes` / `write_bytes` / `close`.
//!   Per Taut `7f4307f` §Q2: PERMANENT @io RESIDENT per glass-wall
//!   recognition (blocking syscall behavior is irreducibly non-mirror).
//! - `boot/std/mcp.mirror:71-134` — `grammar @mcp` at boot altitude;
//!   `request` / `response` types + `serve` action.
//! - `shards/mirror/lens/mcp.mirror:1-66` — MCP lens family-header.
//! - `shards/spectral/gen_prism/mcp_session.mirror` — MCP session
//!   state machine (Reed `e8378ca` M1 TICK 1; Taut `7f4307f` §Q3).
//! - `docs/insights/2026-07-17-loki-matrix-rs-knife-cut-essay.md`
//!   Loki `b53aeeb` §4 phenomenology.
//!
//! ## Recognition candidates this altitude witnesses (HELD)
//!
//! - `#R-mcp-is-composition-not-family-root` (Taut `7f4307f` §9) —
//!   MCP integrates as composition-over-existing-primitives; three-
//!   altitude partition (boot grammar / lens family / gen_prism
//!   species) IS the substrate answer. phone.rs's M4 empirical firing
//!   is the second-witness gate.
//! - `#R-io-socket-lift-consumer-pull-is-M8-not-M4` (Taut `7f4307f`
//!   §9) — boot-altitude sufficiency for M4 stdio; @io/socket lift
//!   fires at M8.

// ---------------------------------------------------------------------
// M0 module stub. No implementations; signatures are forward-promises.
// The `#[allow(dead_code)]` gate keeps M0 compiling cleanly under
// `-W dead_code`; each item retires the gate when its M-tick lands.
// ---------------------------------------------------------------------

/// Read one JSON-RPC line-delimited frame from stdin. M4 forward-promise
/// per Mara §3.2 item 2. Composition: `@io/bytes` + `@data/json.parse`
/// per Taut `7f4307f` §Q4 `initialize` composition path.
#[allow(dead_code)]
pub(crate) fn read_stdin_frame() -> ! {
    unimplemented!("M4 forward-promise: JSON-RPC frame read over @io/bytes")
}

/// Write one JSON-RPC line-delimited frame to stdout. M4 forward-promise
/// per Mara §3.2 item 2. Composition: `@data/json.emit` + `@io/bytes`.
#[allow(dead_code)]
pub(crate) fn write_stdout_frame(_frame: &[u8]) -> ! {
    unimplemented!("M4 forward-promise: JSON-RPC frame write over @io/bytes")
}

/// Open peer socket for `mirror peer beam`. M8 forward-promise per
/// Mara §3.2 item 3. Composition: `@io/socket` mirror-altitude lift-tick
/// (currently boot-only at `boot/std/io/socket.mirror`; lift lands at
/// M8 per Taut `7f4307f` §Q4 substrate-honest deferral).
#[allow(dead_code)]
pub(crate) fn open_peer_socket(_peer_home: &str) -> ! {
    unimplemented!(
        "M8 forward-promise: @io/socket connection for peer beam \
         (consumer-pull mirror-altitude lift of boot/std/io/socket.mirror)"
    )
}

// ---------------------------------------------------------------------
// M-vacuum surface (this file) — @io/fs directory walker.
// ---------------------------------------------------------------------
//
// Per Mara §3.2 item 1 (@io/fs file descriptor management) + Mara §7
// (`roomba --vacuum=~dir` unified motion flag) + §7.4 dispatch matrix
// (walker's fracture table): the ONE @io crossing the vacuum motion
// makes is `list_dir` — recursive directory read yielding path handles.
// Dispatch classification lives at main.rs (`@`-operator dispatch
// altitude); phone.rs holds ONLY the socket-handover altitude read.
//
// M-vacuum uses `std::fs` directly per the delightfully-boring shortest
// path: the @io/fs sub-species at `boot/std/io/fs.mirror` is
// boot-altitude sufficient (glass-wall recognition: blocking syscall
// behavior is irreducibly non-mirror). Mirror-altitude @io/fs lift-tick
// remains forward-promised at `shards/io.mirror:389-390`; land when a
// downstream consumer (M6+ walker composition through gen_prism)
// pulls it.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use fractal::Subject;

/// One walker-visited path with a byte-check on file-kind. Discovered
/// classification is left to main.rs's `@`-operator dispatch per Mara
/// §7.4 (dispatch is byte-check on directory content shape). phone.rs
/// returns raw path + is_dir bit; nothing more.
#[derive(Debug)]
pub(crate) struct WalkEntry {
    pub(crate) path: PathBuf,
    pub(crate) is_dir: bool,
}

/// Recursively walk `root` and yield every file + directory found.
/// Skips `.git/` (per landed roomba walker precedent — walker observes
/// substrate, not its VCS metadata). Skips `target/` (Rust build
/// artifact directory; not substrate). Follows symlinks NOT (per
/// boot-altitude @io/fs discipline — symlink traversal is a distinct
/// motion, not the vacuum's default read).
///
/// Returns entries in depth-first order. Errors bubble up as `io::Error`;
/// the caller (main.rs `@roomba` dispatch arm) decides walker-halt vs
/// walker-continue per @kintsugi/roomba.vacuum_admissible bilateral
/// (forward-promised at Reed M7 co-tick per Mara §9.1).
pub(crate) fn list_dir_recursive(root: &Path) -> io::Result<Vec<WalkEntry>> {
    let mut out = Vec::new();
    walk_into(root, &mut out)?;
    Ok(out)
}

/// Write `contents` to `path` — substrate-honest @io/fs.write at
/// terminal-floor altitude. M-vacuum arm-collapse tick empirical firing
/// per Mara §7.4 dispatch matrix (walker's fracture table row 1:
/// `.rs` → arm-collapse dispatch).
///
/// Errors bubble as `io::Error`; caller decides retry vs halt per
/// `@kintsugi/roomba.vacuum_admissible` bilateral.
pub(crate) fn write_file(path: &Path, contents: &str) -> io::Result<()> {
    fs::write(path, contents)
}

/// Read `path` bytes as UTF-8 — @io/fs.read at terminal-floor altitude.
pub(crate) fn read_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Append `contents` to `path` — @io/fs.append at terminal-floor
/// altitude. Creates the file if it does not exist. Substrate-honest
/// firing surface of the M-vacuum pheromone-deposit tick per Mara
/// `95c0e4a` (canonical spec) + Mara `d7ff58e` (math root §5): the
/// walker's observation crystal is deposited by appending a markdown
/// entry to `docs/bauchladen/mirror-observations.md`. The rolling
/// signature (§5.2 holonomy trace) is the SHA-256 of the observation
/// blob computed at the caller altitude.
pub(crate) fn append_to(path: &Path, contents: &str) -> io::Result<()> {
    use std::io::Write;
    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    f.write_all(contents.as_bytes())
}

/// Create `path` and all missing parents — @io/fs.mkdir_p at terminal-
/// floor altitude. Idempotent per `std::fs::create_dir_all` semantics.
pub(crate) fn mkdir_p(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// Return `true` iff `path` names an extant filesystem entry.
pub(crate) fn path_exists(path: &Path) -> bool {
    path.exists()
}

/// Stage `abs_path` for commit under repo rooted at `repo_root`.
/// Substrate-honest @io/git.add crossing at terminal floor.
///
/// Non-zero git exit surfaces as `io::Error::other` with stderr body.
pub(crate) fn git_add(repo_root: &Path, abs_path: &Path) -> io::Result<()> {
    let rel = abs_path.strip_prefix(repo_root).unwrap_or(abs_path);
    let out = std::process::Command::new("git")
        .args(["add", "--"])
        .arg(rel)
        .current_dir(repo_root)
        .output()?;
    if !out.status.success() {
        return Err(io::Error::other(format!(
            "git add failed: {}",
            String::from_utf8_lossy(&out.stderr)
        )));
    }
    Ok(())
}

/// Commit staged changes under `repo_root` with author (WHO-INTENDED)
/// and committer (WHO-EXECUTED) supplied as `fractal::Subject` values
/// per Mara `2760c2a` fractal migration spec step 9 + MARA doctrine
/// (Author≠Committer split preserved through the type-level chain per
/// Alex Q2 ratification). SSH signing stays operator-default (never
/// override `gpg.format` or `user.signingkey` per CLAUDE.md substrate
/// discipline). Message piped over stdin via `-F -`.
///
/// The committer identity flows through `git -c user.name/email`; the
/// author identity flows through `--author="Name <email>"`. When
/// author and committer are the same @subject (common case: mirror
/// authoring pheromone deposits) the two projections coincide but the
/// carrier remains type-level distinct.
///
/// Returns the HEAD OID after successful commit.
pub(crate) fn git_commit_as(
    repo_root: &Path,
    author: &Subject,
    committer: &Subject,
    message: &str,
) -> io::Result<String> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let author_arg = format!("--author={} <{}>", author.name, author.email);
    let user_name_arg = format!("user.name={}", committer.name);
    let user_email_arg = format!("user.email={}", committer.email);

    let mut child = Command::new("git")
        .args([
            "-c",
            &user_name_arg,
            "-c",
            &user_email_arg,
            "commit",
            &author_arg,
            "-F",
            "-",
        ])
        .current_dir(repo_root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(message.as_bytes())?;
    }
    let out = child.wait_with_output()?;
    if !out.status.success() {
        return Err(io::Error::other(format!(
            "git commit failed: {}\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        )));
    }
    git_head_oid(repo_root)
}

/// Read HEAD OID via `git rev-parse HEAD`.
pub(crate) fn git_head_oid(repo_root: &Path) -> io::Result<String> {
    let out = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo_root)
        .output()?;
    if !out.status.success() {
        return Err(io::Error::other(format!(
            "git rev-parse HEAD failed: {}",
            String::from_utf8_lossy(&out.stderr)
        )));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Walk upward from `start` looking for a directory containing `shards/`.
/// Returns the substrate-repo root if found, else `start` unchanged.
pub(crate) fn find_substrate_root(start: &Path) -> PathBuf {
    let mut cur = start.to_path_buf();
    loop {
        if cur.join("shards").is_dir() {
            return cur;
        }
        if !cur.pop() {
            return start.to_path_buf();
        }
    }
}

fn walk_into(dir: &Path, out: &mut Vec<WalkEntry>) -> io::Result<()> {
    // Defense-in-depth: if walk_into is entered with a .git or target/
    // directory as its argument (e.g., via a direct recursive call
    // that bypassed the per-entry filter below), skip without emitting.
    if let Some(name) = dir.file_name().and_then(|n| n.to_str()) {
        if name == ".git" || name == "target" {
            return Ok(());
        }
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        let is_dir = file_type.is_dir();

        // Skip walker-invisible directory ENTRIES per walker
        // discipline: `.git/` (VCS metadata) and `target/` (Rust build
        // artifacts) are substrate-invisible. Do not emit the entry
        // AND do not descend. Fixed 2026-07-21 iter 6 phone.rs ship
        // per property tests catching that the docblock's "skip"
        // claim wasn't honored at entry-emission altitude.
        if is_dir {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name == ".git" || name == "target" {
                    continue;
                }
            }
        }

        // Skip symlinks by not descending into them and not emitting
        // them (they materialize elsewhere; the vacuum reads content,
        // not aliases).
        if file_type.is_symlink() {
            continue;
        }

        out.push(WalkEntry {
            path: path.clone(),
            is_dir,
        });

        if is_dir {
            walk_into(&path, out)?;
        }
    }
    Ok(())
}

// =====================================================================
// @io/fs property tests — full state-space coverage per Alex 2026-07-21
// "ship phone.rs proper. Property tests and all. Full statespace
// coverage."
//
// Same discipline as FLANG matrix.rs property test surface: every
// public @io/fs fn covered by hand-authored state-space tests spanning
// the observable behavior boundary. tempfile scratch dirs; cleanup on
// TempDir drop; no leaked state.
//
// Iter 6 scope: 8 @io/fs fns — read_file / write_file / append_to /
// mkdir_p / path_exists / list_dir_recursive / find_substrate_root /
// walk_into (via list_dir_recursive). Total: ~30 property tests.
// =====================================================================

#[cfg(test)]
mod fs_prop_tests {
    use super::*;
    use std::io::Write as _;
    use tempfile::TempDir;

    // ---------- read_file / write_file round-trip -------------------

    #[test]
    fn write_then_read_returns_written_content_ascii() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("f.txt");
        let content = "hello mirror";
        write_file(&path, content).unwrap();
        assert_eq!(read_file(&path).unwrap(), content);
    }

    #[test]
    fn write_then_read_returns_written_content_empty() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("empty.txt");
        write_file(&path, "").unwrap();
        assert_eq!(read_file(&path).unwrap(), "");
    }

    #[test]
    fn write_then_read_returns_written_content_unicode() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("u.txt");
        let content = "🌱 mirror → substrate · 中文";
        write_file(&path, content).unwrap();
        assert_eq!(read_file(&path).unwrap(), content);
    }

    #[test]
    fn write_then_read_returns_written_content_multiline_with_newlines() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("m.txt");
        let content = "line1\nline2\nline3\n";
        write_file(&path, content).unwrap();
        assert_eq!(read_file(&path).unwrap(), content);
    }

    #[test]
    fn write_then_read_returns_written_content_large_1mb() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("big.txt");
        let content: String = "x".repeat(1024 * 1024);
        write_file(&path, &content).unwrap();
        let read = read_file(&path).unwrap();
        assert_eq!(read.len(), content.len());
        assert_eq!(read, content);
    }

    #[test]
    fn write_overwrites_existing_file() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("f.txt");
        write_file(&path, "first").unwrap();
        write_file(&path, "second").unwrap();
        assert_eq!(read_file(&path).unwrap(), "second");
    }

    #[test]
    fn read_nonexistent_returns_error() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("nope.txt");
        assert!(read_file(&path).is_err());
    }

    #[test]
    fn write_to_missing_parent_returns_error() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("missing/dir/f.txt");
        assert!(write_file(&path, "x").is_err());
    }

    // ---------- append_to semantics ---------------------------------

    #[test]
    fn append_to_nonexistent_creates_file_with_content() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("a.log");
        append_to(&path, "first entry\n").unwrap();
        assert_eq!(read_file(&path).unwrap(), "first entry\n");
    }

    #[test]
    fn append_to_preserves_order_across_multiple_appends() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("a.log");
        append_to(&path, "A").unwrap();
        append_to(&path, "B").unwrap();
        append_to(&path, "C").unwrap();
        assert_eq!(read_file(&path).unwrap(), "ABC");
    }

    #[test]
    fn append_to_does_not_overwrite_existing() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("a.log");
        write_file(&path, "seed").unwrap();
        append_to(&path, "+more").unwrap();
        assert_eq!(read_file(&path).unwrap(), "seed+more");
    }

    #[test]
    fn append_to_empty_string_is_noop_content() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("a.log");
        append_to(&path, "seed").unwrap();
        append_to(&path, "").unwrap();
        assert_eq!(read_file(&path).unwrap(), "seed");
    }

    // ---------- mkdir_p idempotence + nested -------------------------

    #[test]
    fn mkdir_p_creates_single_directory() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("d");
        mkdir_p(&path).unwrap();
        assert!(path.is_dir());
    }

    #[test]
    fn mkdir_p_creates_nested_directories() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("a/b/c/d");
        mkdir_p(&path).unwrap();
        assert!(path.is_dir());
        assert!(tmp.path().join("a").is_dir());
        assert!(tmp.path().join("a/b").is_dir());
        assert!(tmp.path().join("a/b/c").is_dir());
    }

    #[test]
    fn mkdir_p_is_idempotent_on_existing_directory() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("d");
        mkdir_p(&path).unwrap();
        mkdir_p(&path).unwrap();
        mkdir_p(&path).unwrap();
        assert!(path.is_dir());
    }

    #[test]
    fn mkdir_p_returns_error_when_path_is_existing_file() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("f");
        write_file(&path, "x").unwrap();
        assert!(mkdir_p(&path).is_err());
    }

    // ---------- path_exists file/dir/nonexistent --------------------

    #[test]
    fn path_exists_true_for_extant_file() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("f");
        write_file(&path, "x").unwrap();
        assert!(path_exists(&path));
    }

    #[test]
    fn path_exists_true_for_extant_directory() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("d");
        mkdir_p(&path).unwrap();
        assert!(path_exists(&path));
    }

    #[test]
    fn path_exists_false_for_nonexistent() {
        let tmp = TempDir::new().unwrap();
        assert!(!path_exists(&tmp.path().join("nope")));
    }

    #[test]
    fn path_exists_true_for_tempdir_root() {
        let tmp = TempDir::new().unwrap();
        assert!(path_exists(tmp.path()));
    }

    // ---------- list_dir_recursive full state-space -----------------

    #[test]
    fn list_dir_recursive_on_empty_dir_returns_empty() {
        let tmp = TempDir::new().unwrap();
        let entries = list_dir_recursive(tmp.path()).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn list_dir_recursive_flat_directory_finds_all_files() {
        let tmp = TempDir::new().unwrap();
        write_file(&tmp.path().join("a"), "x").unwrap();
        write_file(&tmp.path().join("b"), "y").unwrap();
        write_file(&tmp.path().join("c"), "z").unwrap();
        let entries = list_dir_recursive(tmp.path()).unwrap();
        assert_eq!(entries.len(), 3);
        assert!(entries.iter().all(|e| !e.is_dir));
        let names: Vec<String> = entries
            .iter()
            .filter_map(|e| e.path.file_name().and_then(|n| n.to_str()).map(String::from))
            .collect();
        for n in ["a", "b", "c"] {
            assert!(names.contains(&n.to_string()), "expected `{n}` in {names:?}");
        }
    }

    #[test]
    fn list_dir_recursive_nested_finds_dirs_and_files_depth_first() {
        let tmp = TempDir::new().unwrap();
        // tmp/a/b/leaf.txt
        // tmp/a/sibling.txt
        mkdir_p(&tmp.path().join("a/b")).unwrap();
        write_file(&tmp.path().join("a/b/leaf.txt"), "L").unwrap();
        write_file(&tmp.path().join("a/sibling.txt"), "S").unwrap();
        let entries = list_dir_recursive(tmp.path()).unwrap();
        // 4 entries: a/ + a/b/ + a/b/leaf.txt + a/sibling.txt (order
        // filesystem-dependent within each dir but a/ MUST come first
        // and a/b/ MUST be visited before leaf.txt).
        assert_eq!(entries.len(), 4);
        let paths: Vec<String> = entries
            .iter()
            .map(|e| {
                e.path
                    .strip_prefix(tmp.path())
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect();
        assert!(paths.contains(&"a".to_string()));
        assert!(paths.contains(&"a/b".to_string()));
        assert!(paths.contains(&"a/b/leaf.txt".to_string()));
        assert!(paths.contains(&"a/sibling.txt".to_string()));

        // Depth-first: a/ appears before a/b/leaf.txt AND a/b/ appears
        // before a/b/leaf.txt.
        let idx_a = paths.iter().position(|p| p == "a").unwrap();
        let idx_ab = paths.iter().position(|p| p == "a/b").unwrap();
        let idx_leaf = paths.iter().position(|p| p == "a/b/leaf.txt").unwrap();
        assert!(idx_a < idx_leaf, "a/ must precede a/b/leaf.txt");
        assert!(idx_ab < idx_leaf, "a/b/ must precede a/b/leaf.txt");
    }

    #[test]
    fn list_dir_recursive_skips_dot_git_at_any_level() {
        let tmp = TempDir::new().unwrap();
        // tmp/.git/HEAD  — must NOT appear
        // tmp/src/main.rs  — MUST appear
        // tmp/nested/.git/objects/pack  — nested .git/ MUST also be skipped
        mkdir_p(&tmp.path().join(".git")).unwrap();
        write_file(&tmp.path().join(".git/HEAD"), "ref").unwrap();
        mkdir_p(&tmp.path().join("src")).unwrap();
        write_file(&tmp.path().join("src/main.rs"), "fn main() {}").unwrap();
        mkdir_p(&tmp.path().join("nested/.git/objects")).unwrap();
        write_file(&tmp.path().join("nested/.git/objects/pack"), "p").unwrap();

        let entries = list_dir_recursive(tmp.path()).unwrap();
        let paths: Vec<String> = entries
            .iter()
            .map(|e| e.path.to_string_lossy().to_string())
            .collect();
        assert!(
            !paths.iter().any(|p| p.contains("/.git/") || p.ends_with("/.git")),
            "no .git entries expected; got {paths:?}"
        );
        assert!(paths.iter().any(|p| p.ends_with("main.rs")));
    }

    #[test]
    fn list_dir_recursive_skips_target_directory() {
        let tmp = TempDir::new().unwrap();
        mkdir_p(&tmp.path().join("target/debug")).unwrap();
        write_file(&tmp.path().join("target/debug/artifact"), "a").unwrap();
        write_file(&tmp.path().join("lib.rs"), "code").unwrap();
        let entries = list_dir_recursive(tmp.path()).unwrap();
        let paths: Vec<String> = entries
            .iter()
            .map(|e| e.path.to_string_lossy().to_string())
            .collect();
        assert!(
            !paths.iter().any(|p| p.contains("/target")),
            "no target/ entries expected; got {paths:?}"
        );
        assert!(paths.iter().any(|p| p.ends_with("lib.rs")));
    }

    #[test]
    fn list_dir_recursive_skips_symlinks() {
        // Cross-platform-safe: only test if symlink creation succeeds
        // (Windows may require admin; unix always works). Skip via
        // early return if symlink creation errors — the assertion
        // still holds for the platforms that support symlinks.
        let tmp = TempDir::new().unwrap();
        write_file(&tmp.path().join("real.txt"), "R").unwrap();
        let link_path = tmp.path().join("alias.txt");
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&tmp.path().join("real.txt"), &link_path).unwrap();
            let entries = list_dir_recursive(tmp.path()).unwrap();
            let paths: Vec<String> = entries
                .iter()
                .map(|e| e.path.file_name().unwrap().to_string_lossy().to_string())
                .collect();
            assert!(paths.contains(&"real.txt".to_string()));
            assert!(
                !paths.contains(&"alias.txt".to_string()),
                "symlink alias.txt MUST be skipped; got {paths:?}"
            );
        }
        #[cfg(not(unix))]
        {
            // Symlink test skipped on non-unix; sentinel to keep the
            // test structure honest.
            let _ = link_path;
        }
    }

    #[test]
    fn list_dir_recursive_is_reads_only_no_mutation() {
        // Regression guard: walker MUST NOT create/modify any files.
        let tmp = TempDir::new().unwrap();
        write_file(&tmp.path().join("seed"), "s").unwrap();
        let before = fs::read_dir(tmp.path()).unwrap().count();
        let _ = list_dir_recursive(tmp.path()).unwrap();
        let after = fs::read_dir(tmp.path()).unwrap().count();
        assert_eq!(before, after, "walker MUST NOT mutate the filesystem");
    }

    #[test]
    fn list_dir_recursive_error_on_nonexistent_root() {
        let tmp = TempDir::new().unwrap();
        let missing = tmp.path().join("does_not_exist");
        assert!(list_dir_recursive(&missing).is_err());
    }

    #[test]
    fn list_dir_recursive_dir_entries_have_is_dir_true() {
        let tmp = TempDir::new().unwrap();
        mkdir_p(&tmp.path().join("a")).unwrap();
        mkdir_p(&tmp.path().join("b")).unwrap();
        write_file(&tmp.path().join("c.txt"), "x").unwrap();
        let entries = list_dir_recursive(tmp.path()).unwrap();
        for e in &entries {
            let name = e.path.file_name().unwrap().to_string_lossy().to_string();
            let expected_is_dir = matches!(name.as_str(), "a" | "b");
            assert_eq!(
                e.is_dir, expected_is_dir,
                "is_dir mismatch for entry `{name}`"
            );
        }
    }

    // ---------- find_substrate_root upward walk ---------------------

    #[test]
    fn find_substrate_root_returns_root_when_shards_present_at_start() {
        let tmp = TempDir::new().unwrap();
        mkdir_p(&tmp.path().join("shards")).unwrap();
        let root = find_substrate_root(tmp.path());
        assert_eq!(root, tmp.path());
    }

    #[test]
    fn find_substrate_root_walks_upward_from_nested_child() {
        let tmp = TempDir::new().unwrap();
        mkdir_p(&tmp.path().join("shards")).unwrap();
        mkdir_p(&tmp.path().join("a/b/c")).unwrap();
        let root = find_substrate_root(&tmp.path().join("a/b/c"));
        assert_eq!(root, tmp.path());
    }

    #[test]
    fn find_substrate_root_returns_start_when_no_shards_ancestor() {
        // tmp/leaf/ has no shards/ ancestor within tmp (temp roots
        // don't have shards/ at OS root either). find_substrate_root
        // returns start unchanged.
        let tmp = TempDir::new().unwrap();
        let start = tmp.path().join("leaf");
        mkdir_p(&start).unwrap();
        let root = find_substrate_root(&start);
        // On systems where none of the ancestors has shards/, the fn
        // returns start unchanged.
        assert_eq!(root, start);
    }

    #[test]
    fn find_substrate_root_selects_nearest_shards_ancestor() {
        // Two shards/ present: closer one wins. Walker walks upward
        // and returns the FIRST ancestor with shards/ (nearest).
        let tmp = TempDir::new().unwrap();
        mkdir_p(&tmp.path().join("outer/inner/shards")).unwrap();
        mkdir_p(&tmp.path().join("outer/inner/deep/nested")).unwrap();
        // NB: we do NOT create tmp/shards — nearest is outer/inner/.
        let root = find_substrate_root(&tmp.path().join("outer/inner/deep/nested"));
        assert_eq!(root, tmp.path().join("outer/inner"));
    }

    // ---------- append_to atomicity (writer flushes on drop) --------

    #[test]
    fn append_to_flushes_before_returning() {
        // Regression guard for the OpenOptions.append pattern: the
        // fn MUST flush before returning so subsequent read_file
        // observes the written bytes without an explicit sync.
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("a.log");
        for i in 0..10 {
            append_to(&path, &format!("line{i}\n")).unwrap();
            let content = read_file(&path).unwrap();
            assert!(
                content.contains(&format!("line{i}")),
                "line{i} not visible after append; got {content:?}"
            );
        }
    }

    // ---------- WalkEntry Debug is stable + non-empty ---------------

    #[test]
    fn walk_entry_debug_includes_path_and_is_dir() {
        let entry = WalkEntry {
            path: PathBuf::from("/tmp/test"),
            is_dir: false,
        };
        let dbg = format!("{entry:?}");
        assert!(dbg.contains("/tmp/test"));
        assert!(dbg.contains("is_dir"));
    }

    // ---------- write_file over previous mkdir_p directory errors ---

    #[test]
    fn write_file_to_existing_directory_path_errors() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("d");
        mkdir_p(&path).unwrap();
        assert!(write_file(&path, "x").is_err());
    }

    // ---------- content-round-trip via bytes helper (append + read) -

    #[test]
    fn append_bytes_survive_arbitrary_content_boundaries() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("boundary.log");
        // Mix ASCII + unicode + newlines + tabs across appends.
        let chunks = [
            "start",
            "\t\ttab-indented",
            "\n",
            "🌱",
            "\r\ncarriage",
            "end",
        ];
        let expected: String = chunks.concat();
        for c in &chunks {
            append_to(&path, c).unwrap();
        }
        assert_eq!(read_file(&path).unwrap(), expected);
    }

    // ---------- silence unused-warnings for internal helpers --------
    //
    // Keeping Write imported for future append_writer-based tests
    // without dragging the whole io::Write consumer surface into the
    // #[test] namespace.
    #[allow(dead_code)]
    fn _use_write_trait(f: &mut fs::File, bytes: &[u8]) -> io::Result<()> {
        f.write_all(bytes)
    }
}
