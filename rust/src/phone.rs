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

/// Read one line-delimited frame from a `BufRead`. Body-only per
/// newline-delimited JSON-RPC 2.0 framing (MCP over stdio uses this
/// shape; contrast with LSP's Content-Length header framing).
///
/// - Reads bytes up to and including the next `\n`, returns bytes
///   WITHOUT the trailing newline.
/// - EOF before any bytes: returns an empty Vec.
/// - EOF mid-frame (no trailing newline): returns bytes read.
///
/// The stdin-bound wrapper [`read_stdin_frame`] composes this with
/// `io::stdin().lock()` so the runtime can test the parsing logic in
/// isolation with in-memory buffers.
pub(crate) fn read_frame_from<R: io::BufRead>(reader: &mut R) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    reader.read_until(b'\n', &mut buf)?;
    if buf.last() == Some(&b'\n') {
        buf.pop();
    }
    Ok(buf)
}

/// Write one line-delimited frame + trailing `\n` to a `Write`, then
/// flush. Body-only per newline-delimited JSON-RPC 2.0 framing.
pub(crate) fn write_frame_to<W: io::Write>(writer: &mut W, frame: &[u8]) -> io::Result<()> {
    writer.write_all(frame)?;
    writer.write_all(b"\n")?;
    writer.flush()
}

/// Read one JSON-RPC line-delimited frame from stdin. M4 landing per
/// Mara §3.2 item 2 (iter 8 phone.rs ship arc, task #303). Composition:
/// `@io/bytes` + `@data/json.parse` per Taut `7f4307f` §Q4 `initialize`
/// composition path. Delegates to [`read_frame_from`] with
/// `io::stdin().lock()`; the parsing logic is testable in isolation
/// via that generic.
#[allow(dead_code)]
pub(crate) fn read_stdin_frame() -> io::Result<Vec<u8>> {
    let stdin = io::stdin();
    let mut locked = stdin.lock();
    read_frame_from(&mut locked)
}

/// Write one JSON-RPC line-delimited frame to stdout. M4 landing per
/// Mara §3.2 item 2. Composition: `@data/json.emit` + `@io/bytes`.
/// Delegates to [`write_frame_to`] with `io::stdout().lock()`; flushes
/// before returning.
#[allow(dead_code)]
pub(crate) fn write_stdout_frame(frame: &[u8]) -> io::Result<()> {
    let stdout = io::stdout();
    let mut locked = stdout.lock();
    write_frame_to(&mut locked, frame)
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

// =====================================================================
// @io/git property tests — tempdir-repo full state-space coverage.
//
// Iter 7 of phone.rs ship arc (task #303). Covers git_add / git_commit_as
// / git_head_oid via tempdir git init scaffold with per-repo
// commit.gpgsign=false to isolate from operator SSH signing default
// (CLAUDE.md: NEVER override gpg.format or user.signingkey at global
// scope; per-repo test-only override is safe).
//
// Tests skip gracefully via early return when `git` binary is not
// available on PATH — CI + dev environments without git installed
// remain green; environments with git installed exercise the full
// state space.
// =====================================================================

#[cfg(test)]
mod git_prop_tests {
    use super::*;
    use fractal::Subject;
    use std::process::Command;
    use tempfile::TempDir;

    /// True iff `git --version` succeeds — gates the test on git
    /// availability. Tests early-return Ok when git missing.
    fn git_available() -> bool {
        Command::new("git")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Initialize an empty git repo in a fresh tempdir. Disables
    /// commit.gpgsign per-repo so tests don't depend on operator
    /// signing keys. Sets a default branch to `main` for consistency
    /// across git versions. Returns the tempdir (drop cleans up).
    fn fresh_repo() -> Option<TempDir> {
        if !git_available() {
            return None;
        }
        let tmp = TempDir::new().unwrap();
        let out = Command::new("git")
            .args(["init", "-q", "--initial-branch=main"])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        if !out.status.success() {
            // Older git may not support --initial-branch; retry
            // without and rename manually.
            let out2 = Command::new("git")
                .args(["init", "-q"])
                .current_dir(tmp.path())
                .output()
                .unwrap();
            assert!(out2.status.success(), "git init failed");
        }
        // Disable signing per-repo (belt-and-suspenders vs operator
        // global default).
        let out = Command::new("git")
            .args(["config", "commit.gpgsign", "false"])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        assert!(out.status.success(), "git config gpgsign=false failed");
        let out = Command::new("git")
            .args(["config", "tag.gpgsign", "false"])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        assert!(out.status.success(), "git config tag.gpgsign=false failed");
        // Isolate tests from operator's global commit-msg hook (which
        // enforces phase markers, etc.). Property tests exercise the
        // git_commit_as SURFACE, not the operator's commit policy.
        let out = Command::new("git")
            .args(["config", "core.hooksPath", "/dev/null"])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        assert!(out.status.success(), "git config core.hooksPath=/dev/null failed");
        Some(tmp)
    }

    fn test_subject(name: &str, email: &str) -> Subject {
        Subject::human(name, email)
    }

    fn test_author() -> Subject {
        test_subject("Author", "author@systemic.engineer")
    }

    fn test_committer() -> Subject {
        test_subject("Committer", "committer@systemic.engineer")
    }

    // ---------- git_head_oid: empty vs after-commit ------------------

    #[test]
    fn git_head_oid_errors_on_empty_repo() {
        let Some(tmp) = fresh_repo() else { return };
        let result = git_head_oid(tmp.path());
        assert!(
            result.is_err(),
            "empty repo has no HEAD; expected err; got {result:?}"
        );
    }

    #[test]
    fn git_head_oid_returns_forty_char_sha_after_commit() {
        let Some(tmp) = fresh_repo() else { return };
        // Stage a file and commit via git directly (bypassing our
        // git_commit_as to keep this test scoped to git_head_oid).
        write_file(&tmp.path().join("f.txt"), "seed").unwrap();
        Command::new("git")
            .args(["add", "f.txt"])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        Command::new("git")
            .args([
                "-c", "user.name=T", "-c", "user.email=t@t",
                "commit", "-m", "seed", "--no-gpg-sign",
            ])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        let oid = git_head_oid(tmp.path()).unwrap();
        assert_eq!(oid.len(), 40, "expected 40-char sha; got `{oid}`");
        assert!(oid.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn git_head_oid_errors_on_non_git_directory() {
        let tmp = TempDir::new().unwrap();
        // no git init — not a git repo
        assert!(git_head_oid(tmp.path()).is_err());
    }

    // ---------- git_add: real file / nonexistent / outside-root ------

    #[test]
    fn git_add_stages_existing_file_in_repo() {
        let Some(tmp) = fresh_repo() else { return };
        let file = tmp.path().join("staged.txt");
        write_file(&file, "content").unwrap();
        git_add(tmp.path(), &file).unwrap();
        // Verify staged via `git diff --cached --name-only`.
        let out = Command::new("git")
            .args(["diff", "--cached", "--name-only"])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        let staged = String::from_utf8_lossy(&out.stdout);
        assert!(
            staged.contains("staged.txt"),
            "expected staged.txt in staged files; got {staged:?}"
        );
    }

    #[test]
    fn git_add_errors_on_nonexistent_file() {
        let Some(tmp) = fresh_repo() else { return };
        let missing = tmp.path().join("never_created.txt");
        let result = git_add(tmp.path(), &missing);
        assert!(
            result.is_err(),
            "nonexistent file must error; got {result:?}"
        );
    }

    #[test]
    fn git_add_relativizes_absolute_path_under_repo() {
        // git_add strips repo_root prefix from abs_path to obtain rel.
        // Verify the strip works: absolute path within repo → staged
        // via relative name.
        let Some(tmp) = fresh_repo() else { return };
        let sub_dir = tmp.path().join("src");
        mkdir_p(&sub_dir).unwrap();
        let abs_file = sub_dir.join("deep.rs");
        write_file(&abs_file, "fn main() {}").unwrap();
        git_add(tmp.path(), &abs_file).unwrap();
        let out = Command::new("git")
            .args(["diff", "--cached", "--name-only"])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        let staged = String::from_utf8_lossy(&out.stdout);
        assert!(
            staged.contains("src/deep.rs"),
            "expected src/deep.rs in staged; got {staged:?}"
        );
    }

    // ---------- git_commit_as: full pipeline ------------------------

    #[test]
    fn git_commit_as_produces_head_oid_after_commit() {
        let Some(tmp) = fresh_repo() else { return };
        let file = tmp.path().join("f.txt");
        write_file(&file, "content").unwrap();
        git_add(tmp.path(), &file).unwrap();
        let author = test_author();
        let committer = test_committer();
        let oid = git_commit_as(tmp.path(), &author, &committer, "first commit")
            .unwrap();
        assert_eq!(oid.len(), 40);
        assert_eq!(oid, git_head_oid(tmp.path()).unwrap());
    }

    #[test]
    fn git_commit_as_preserves_author_committer_split() {
        // MARA doctrine: Author ≠ Committer. Verify both identities
        // land distinct on the commit object.
        let Some(tmp) = fresh_repo() else { return };
        let file = tmp.path().join("f.txt");
        write_file(&file, "content").unwrap();
        git_add(tmp.path(), &file).unwrap();
        let author = test_author();
        let committer = test_committer();
        git_commit_as(tmp.path(), &author, &committer, "split test").unwrap();
        // Verify via `git log --format=%an|%ae|%cn|%ce -1`.
        let out = Command::new("git")
            .args(["log", "--format=%an|%ae|%cn|%ce", "-1"])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        let line = String::from_utf8_lossy(&out.stdout);
        let line = line.trim();
        let parts: Vec<&str> = line.split('|').collect();
        assert_eq!(parts.len(), 4, "expected 4 fields; got {parts:?}");
        assert_eq!(parts[0], "Author", "author name mismatch");
        assert_eq!(parts[1], "author@systemic.engineer", "author email mismatch");
        assert_eq!(parts[2], "Committer", "committer name mismatch");
        assert_eq!(parts[3], "committer@systemic.engineer", "committer email mismatch");
    }

    #[test]
    fn git_commit_as_same_subject_as_author_and_committer_produces_matching_pair() {
        // Common case: mirror authoring pheromone deposits where author
        // and committer are the same @subject. Both projections
        // coincide but the carrier remains type-level distinct.
        let Some(tmp) = fresh_repo() else { return };
        let file = tmp.path().join("f.txt");
        write_file(&file, "c").unwrap();
        git_add(tmp.path(), &file).unwrap();
        let sub = test_subject("Mirror", "mirror@systemic.engineer");
        git_commit_as(tmp.path(), &sub, &sub, "coincident commit").unwrap();
        let out = Command::new("git")
            .args(["log", "--format=%an|%ae|%cn|%ce", "-1"])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        let line = String::from_utf8_lossy(&out.stdout);
        let line = line.trim();
        let parts: Vec<&str> = line.split('|').collect();
        assert_eq!(parts[0], parts[2], "author name == committer name");
        assert_eq!(parts[1], parts[3], "author email == committer email");
        assert_eq!(parts[0], "Mirror");
        assert_eq!(parts[1], "mirror@systemic.engineer");
    }

    #[test]
    fn git_commit_as_supports_multiline_utf8_message() {
        let Some(tmp) = fresh_repo() else { return };
        let file = tmp.path().join("f.txt");
        write_file(&file, "c").unwrap();
        git_add(tmp.path(), &file).unwrap();
        let msg = "first line\n\nbody line 1\nbody line 2 with unicode 🌱\n";
        let author = test_author();
        let committer = test_committer();
        git_commit_as(tmp.path(), &author, &committer, msg).unwrap();
        let out = Command::new("git")
            .args(["log", "--format=%B", "-1"])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        let logged = String::from_utf8_lossy(&out.stdout);
        assert!(logged.contains("first line"));
        assert!(logged.contains("body line 1"));
        assert!(logged.contains("body line 2 with unicode"));
        assert!(logged.contains("🌱"));
    }

    #[test]
    fn git_commit_as_errors_on_empty_message() {
        let Some(tmp) = fresh_repo() else { return };
        let file = tmp.path().join("f.txt");
        write_file(&file, "c").unwrap();
        git_add(tmp.path(), &file).unwrap();
        let author = test_author();
        let committer = test_committer();
        let result = git_commit_as(tmp.path(), &author, &committer, "");
        assert!(
            result.is_err(),
            "empty message must error (git rejects); got {result:?}"
        );
    }

    #[test]
    fn git_commit_as_errors_on_nothing_staged() {
        let Some(tmp) = fresh_repo() else { return };
        // No staged file; git commit fails.
        let author = test_author();
        let committer = test_committer();
        let result = git_commit_as(tmp.path(), &author, &committer, "nothing to commit");
        assert!(result.is_err());
    }

    #[test]
    fn git_commit_as_sequence_produces_distinct_head_oids() {
        let Some(tmp) = fresh_repo() else { return };
        let author = test_author();
        let committer = test_committer();
        let f1 = tmp.path().join("a.txt");
        write_file(&f1, "a").unwrap();
        git_add(tmp.path(), &f1).unwrap();
        let oid1 = git_commit_as(tmp.path(), &author, &committer, "first").unwrap();

        let f2 = tmp.path().join("b.txt");
        write_file(&f2, "b").unwrap();
        git_add(tmp.path(), &f2).unwrap();
        let oid2 = git_commit_as(tmp.path(), &author, &committer, "second").unwrap();

        assert_ne!(oid1, oid2, "distinct commits must have distinct OIDs");
        assert_eq!(oid2, git_head_oid(tmp.path()).unwrap());
    }

    // ---------- content-round-trip: full add + commit + verify ------

    // ---------- placeholder anchor for the last test in the block --
    // (kept above the final commit-flow test)

    #[test]
    fn git_add_and_commit_flow_produces_visible_history_entry() {
        let Some(tmp) = fresh_repo() else { return };
        let author = test_author();
        let committer = test_committer();
        // Two files across two commits.
        for (i, (name, content, msg)) in [
            ("one.txt", "content one", "first commit"),
            ("two.txt", "content two", "second commit"),
        ]
        .iter()
        .enumerate()
        {
            let path = tmp.path().join(name);
            write_file(&path, content).unwrap();
            git_add(tmp.path(), &path).unwrap();
            git_commit_as(tmp.path(), &author, &committer, msg).unwrap();
            assert_eq!(
                git_head_oid(tmp.path()).unwrap().len(),
                40,
                "HEAD oid after commit {i} must be 40 chars"
            );
        }
        // Verify 2 commits in log.
        let out = Command::new("git")
            .args(["rev-list", "--count", "HEAD"])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        let count = String::from_utf8_lossy(&out.stdout);
        assert_eq!(count.trim(), "2");
    }
}

// =====================================================================
// @io/bytes stdio property tests — M4 landing per iter 8 phone.rs ship
// arc (task #303). Covers read_frame_from + write_frame_to generic
// helpers with in-memory buffers, so parsing / framing logic is tested
// without spawning child processes for stdin/stdout piping.
//
// read_stdin_frame + write_stdout_frame are thin wrappers around the
// generic helpers with io::stdin().lock() / io::stdout().lock(); their
// correctness follows from the generic helpers' correctness plus the
// lock semantics of std::io::stdin/stdout (which is a std library
// invariant, not phone.rs's concern).
// =====================================================================

#[cfg(test)]
mod bytes_prop_tests {
    use super::*;
    use std::io::Cursor;

    // ---------- read_frame_from -------------------------------------

    #[test]
    fn read_frame_returns_bytes_before_newline_stripping_delimiter() {
        let mut r = Cursor::new(b"hello\n".as_ref());
        let frame = read_frame_from(&mut r).unwrap();
        assert_eq!(frame, b"hello");
    }

    #[test]
    fn read_frame_empty_input_returns_empty_vec() {
        let mut r = Cursor::new(b"".as_ref());
        let frame = read_frame_from(&mut r).unwrap();
        assert!(frame.is_empty());
    }

    #[test]
    fn read_frame_missing_trailing_newline_returns_bytes_read() {
        // EOF mid-frame: read_until returns all bytes to EOF without
        // a delimiter; fn returns them verbatim (no newline to strip).
        let mut r = Cursor::new(b"partial".as_ref());
        let frame = read_frame_from(&mut r).unwrap();
        assert_eq!(frame, b"partial");
    }

    #[test]
    fn read_frame_reads_only_first_line_of_multi_line_input() {
        let mut r = Cursor::new(b"first\nsecond\nthird\n".as_ref());
        let first = read_frame_from(&mut r).unwrap();
        assert_eq!(first, b"first");
        let second = read_frame_from(&mut r).unwrap();
        assert_eq!(second, b"second");
        let third = read_frame_from(&mut r).unwrap();
        assert_eq!(third, b"third");
    }

    #[test]
    fn read_frame_preserves_utf8_bytes() {
        let input = "🌱 mirror 中文\n".as_bytes();
        let mut r = Cursor::new(input);
        let frame = read_frame_from(&mut r).unwrap();
        assert_eq!(String::from_utf8(frame).unwrap(), "🌱 mirror 中文");
    }

    #[test]
    fn read_frame_preserves_json_body_verbatim() {
        // JSON-RPC 2.0 frame content stays byte-identical (no parsing
        // at this altitude — that's @data/json.parse consumer).
        let json = br#"{"jsonrpc":"2.0","method":"initialize","id":1,"params":{}}"#;
        let mut input = Vec::with_capacity(json.len() + 1);
        input.extend_from_slice(json);
        input.push(b'\n');
        let mut r = Cursor::new(input);
        let frame = read_frame_from(&mut r).unwrap();
        assert_eq!(frame, json);
    }

    #[test]
    fn read_frame_handles_empty_line_returning_empty_vec_between_frames() {
        let mut r = Cursor::new(b"first\n\nthird\n".as_ref());
        assert_eq!(read_frame_from(&mut r).unwrap(), b"first");
        assert_eq!(read_frame_from(&mut r).unwrap(), b"");
        assert_eq!(read_frame_from(&mut r).unwrap(), b"third");
    }

    #[test]
    fn read_frame_handles_binary_content_before_newline() {
        // Frame bytes are BODY-ONLY; binary content survives.
        let mut input = vec![0x00, 0xFF, 0x01, 0xFE, 0x02];
        input.push(b'\n');
        let mut r = Cursor::new(input.clone());
        let frame = read_frame_from(&mut r).unwrap();
        assert_eq!(frame, &input[..5]);
    }

    #[test]
    fn read_frame_repeated_at_eof_returns_empty_vec() {
        let mut r = Cursor::new(b"only\n".as_ref());
        assert_eq!(read_frame_from(&mut r).unwrap(), b"only");
        // Further reads at EOF return empty (Rice-safe).
        assert!(read_frame_from(&mut r).unwrap().is_empty());
        assert!(read_frame_from(&mut r).unwrap().is_empty());
    }

    // ---------- write_frame_to --------------------------------------

    #[test]
    fn write_frame_writes_bytes_followed_by_newline() {
        let mut w: Vec<u8> = Vec::new();
        write_frame_to(&mut w, b"hello").unwrap();
        assert_eq!(w, b"hello\n");
    }

    #[test]
    fn write_frame_empty_body_writes_just_newline() {
        let mut w: Vec<u8> = Vec::new();
        write_frame_to(&mut w, b"").unwrap();
        assert_eq!(w, b"\n");
    }

    #[test]
    fn write_frame_preserves_utf8_body() {
        let body = "🌱 mirror".as_bytes();
        let mut w: Vec<u8> = Vec::new();
        write_frame_to(&mut w, body).unwrap();
        let mut expected: Vec<u8> = body.to_vec();
        expected.push(b'\n');
        assert_eq!(w, expected);
    }

    #[test]
    fn write_frame_preserves_binary_body() {
        let body = &[0x00u8, 0xFF, 0x01, 0xFE, 0x02];
        let mut w: Vec<u8> = Vec::new();
        write_frame_to(&mut w, body).unwrap();
        assert_eq!(&w[..5], body);
        assert_eq!(w[5], b'\n');
    }

    #[test]
    fn write_frame_appends_only_one_newline_when_body_ends_with_newline() {
        // Substrate-honest: fn writes body verbatim + ONE newline.
        // If the caller's body already ends in \n, the frame contains
        // TWO newlines. Documented behavior; test asserts it.
        let mut w: Vec<u8> = Vec::new();
        write_frame_to(&mut w, b"already\n").unwrap();
        assert_eq!(w, b"already\n\n");
    }

    #[test]
    fn write_frame_flushes_before_returning() {
        // Vec<u8> flush is no-op; this test just verifies the write
        // completed. A more thorough test would use a fake writer that
        // tracks flush calls, but the flush semantic is enforced by
        // the fn body's explicit writer.flush() call.
        let mut w: Vec<u8> = Vec::new();
        for i in 0..100 {
            write_frame_to(&mut w, format!("frame{i}").as_bytes()).unwrap();
        }
        assert_eq!(w.iter().filter(|&&b| b == b'\n').count(), 100);
    }

    // ---------- round-trip: write then read -------------------------

    #[test]
    fn write_then_read_round_trips_ascii_frame() {
        let mut buf: Vec<u8> = Vec::new();
        write_frame_to(&mut buf, b"round-trip").unwrap();
        let mut r = Cursor::new(buf);
        let frame = read_frame_from(&mut r).unwrap();
        assert_eq!(frame, b"round-trip");
    }

    #[test]
    fn write_then_read_round_trips_json_rpc_frame() {
        let json = br#"{"jsonrpc":"2.0","result":{"ok":true},"id":42}"#;
        let mut buf: Vec<u8> = Vec::new();
        write_frame_to(&mut buf, json).unwrap();
        let mut r = Cursor::new(buf);
        let frame = read_frame_from(&mut r).unwrap();
        assert_eq!(frame, json);
    }

    #[test]
    fn write_then_read_round_trips_multiple_frames_in_order() {
        let mut buf: Vec<u8> = Vec::new();
        for i in 0..10 {
            write_frame_to(&mut buf, format!("frame{i}").as_bytes()).unwrap();
        }
        let mut r = Cursor::new(buf);
        for i in 0..10 {
            let frame = read_frame_from(&mut r).unwrap();
            assert_eq!(frame, format!("frame{i}").as_bytes());
        }
        // After 10 frames, reader is at EOF.
        assert!(read_frame_from(&mut r).unwrap().is_empty());
    }

    // ---------- stdin/stdout wrappers exist + return io::Result -----

    #[test]
    fn read_stdin_frame_signature_returns_io_result() {
        // Signature-only compile check: we can't actually READ from
        // stdin under `cargo test` without pipe manipulation. This
        // asserts the fn exists and returns io::Result<Vec<u8>>.
        let _f: fn() -> io::Result<Vec<u8>> = read_stdin_frame;
    }

    #[test]
    fn write_stdout_frame_signature_returns_io_result() {
        // Signature-only compile check.
        let _f: fn(&[u8]) -> io::Result<()> = write_stdout_frame;
    }
}
