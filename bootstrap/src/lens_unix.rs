//! `@mirror/lens/unix` — v0 floor: materialize-to-tempdir impedance lens.
//!
//! The substrate declares `@mirror/lens/unix` in
//! `shards/mirror/lens/unix.mirror` as the OS-level impedance surface:
//! `mount(shard) -> imperfect`, `path(handle, components) -> ref`,
//! `read(p) -> imperfect`, `write(p, bytes) -> oid`, `stat(p) -> verdict`.
//! All bodies are `\` obligations. This module realizes those bodies for
//! the v0 floor.
//!
//! ## Implementation choice: materialize-to-tempdir
//!
//! The spec (`docs/specs/spectral-runtime.md` §5) names the lens as the
//! impedance match between `@mirror/store` (content-addressed shards) and
//! `@io/cargo` (a process that expects path-addressed files on disk). It
//! does NOT pin a transport: FUSE, 9P, NFS, or materialization are all
//! candidate sub-species under `@mirror/lens/unix/<platform>`.
//!
//! For the v0 floor we materialize. The reasoning:
//!
//! - **macOS / FUSE**: macFUSE requires a kernel extension, SIP weakening,
//!   explicit user approval in System Settings, and a reboot. That's a
//!   hard friction wall for a substrate floor that must work in CI and on
//!   every dev machine. Mara's T8 unix-lens brief STOP-condition 1 names
//!   exactly this failure mode.
//! - **Linux / FUSE**: works via `fuser`, but a bifurcated impl (FUSE on
//!   Linux, something-else on macOS) creates the platform-specific fork
//!   STOP-condition 4 warns against.
//! - **Materialize-to-tempdir**: kernel-extension-free, idempotent
//!   (same shard OID -> same byte content at any tempdir), and the
//!   load-bearing primitive — path -> shard lookup — is what later
//!   FUSE / 9P sub-species will reuse. Cargo reads through a real
//!   filesystem path; convergence-by-construction holds because the
//!   content lives in the OID-addressed store.
//!
//! Future ticks land `@mirror/lens/unix/fuse` and `@mirror/lens/unix/9p`
//! as platform sub-species; they swap the materialization step for a
//! kernel-mediated view. The abstract interface here doesn't change.
//!
//! ## Substrate body discharge
//!
//! Per the substrate declaration, each action's `\` obligation discharges
//! through a function here:
//!
//! | Substrate action          | Rust body                   |
//! |---------------------------|-----------------------------|
//! | `mount(shard) -> imperfect` | `UnixLens::mount`         |
//! | `path(handle, components) -> ref` | `UnixLens::path`    |
//! | `read(p) -> imperfect`    | `UnixLens::read`            |
//! | `write(p, bytes) -> oid`  | `UnixLens::write` (v1)      |
//! | `stat(p) -> verdict`      | `UnixLens::stat`            |
//!
//! v0 ships `mount`, `path`, `read`, and `stat`. `write` is forward-promised
//! to v1 — the v0 floor is the read-only impedance face, which is what the
//! ouroboros pipeline needs to close (`@code/metalogue/materialize` writes
//! to `@mirror/store` directly; the lens exposes the result to cargo for
//! reading only).

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::crystallize::{Content, FieldName, MerkleHash, Splinter};

/// The substrate's `verdict` carrier (per `shards/glass.mirror`).
///
/// `stat(p) -> verdict` returns one of these. The pass/partial/failure
/// distinction is load-bearing for callers that need to disambiguate
/// "the path is absent" from "the path is present but unreadable".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// The path exists and is fully observable as the expected kind.
    Pass,
    /// The path exists but with reduced confidence (e.g. partial
    /// materialization, kind mismatch within tolerance). Reserved; the
    /// v0 floor does not emit `Partial` because materialization is atomic
    /// per file. Future sub-species (FUSE with cached metadata,
    /// network 9P with stale entries) will use it.
    #[allow(dead_code)]
    Partial(String),
    /// The path does not resolve to anything observable.
    Failure(String),
}

/// Opacity reasons surfaced when an `imperfect`-returning action fails.
///
/// Mirrors the substrate's `imperfect(a, e, l)` floor: a successful action
/// carries `a`, a failure carries `e` (this enum). The `partial(opacity)`
/// shape is reserved for sub-species that need to report which subset of
/// the requested observation succeeded; v0 materializes atomically.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpacityReason {
    /// The shard root could not be materialized (I/O failure during
    /// tempdir creation or write).
    MountFailed(String),
    /// A path requested via `read` does not resolve in the mounted
    /// namespace.
    PathAbsent(PathBuf),
    /// A path resolves but its bytes cannot be read from the underlying
    /// filesystem.
    ReadFailed(String),
}

/// A mounted `UnixLens` handle.
///
/// Carries the tempdir's root path (the mountpoint) plus a content map
/// from relative path -> bytes. The materialized files on disk are the
/// authoritative byte source for `read`; the content map is a fast-path
/// index for `stat` and the substrate-side `path` resolution.
///
/// The tempdir is owned by this struct; when the `UnixLens` is dropped,
/// the materialized tree is cleaned up. This matches the v0 floor's
/// "present a stored shard to cargo for the duration of the build"
/// shape — cargo's invocation is bounded; the lens lifetime matches.
pub struct UnixLens {
    /// Absolute path to the materialized tempdir root.
    mountpoint: PathBuf,
    /// In-memory content map (relative path -> bytes). Authoritative for
    /// `stat`; mirrored on disk for `read` (so cargo, which goes through
    /// the kernel VFS, sees the same bytes).
    content: BTreeMap<PathBuf, Vec<u8>>,
    /// `true` if the lens owns the tempdir (and should delete it on
    /// drop); `false` if the caller supplied a pre-existing mountpoint
    /// (used by tests that want a stable path for `--manifest-path`).
    owns_mountpoint: bool,
}

impl UnixLens {
    /// `mount(shard) -> imperfect` — bind a stored shard's content to a
    /// fresh tempdir.
    ///
    /// The shard is presented as a `Splinter` whose `Record` content maps
    /// `FieldName` keys to sub-`Splinter`s. The walk treats the path-
    /// namespace as a Record-of-Records-of-...-Text — leaf `Text` content
    /// becomes a file at the corresponding relative path; `Record` and
    /// `List` content becomes a directory.
    ///
    /// The walk is the substrate's `splinter_graph` closure traversal
    /// (per `shards/mirror/store.mirror`'s `walk(root) -> splinter_graph`).
    /// Each `(parent_path, child)` edge in the OID-graph is one
    /// materialization step.
    ///
    /// Idempotent by content: mounting the same Splinter OID twice
    /// produces tempdirs with byte-identical content (different paths,
    /// same bytes).
    pub fn mount<H: MerkleHash>(shard: &Splinter<H>) -> Result<Self, OpacityReason> {
        let mountpoint = make_tempdir()
            .map_err(|e| OpacityReason::MountFailed(format!("could not create tempdir: {}", e)))?;
        let mut content = BTreeMap::new();
        materialize_splinter(shard, &mountpoint, PathBuf::new(), &mut content)
            .map_err(|e| OpacityReason::MountFailed(format!("materialize: {}", e)))?;
        Ok(UnixLens {
            mountpoint,
            content,
            owns_mountpoint: true,
        })
    }

    /// `path(handle, components) -> ref` — construct a Unix path within
    /// the mounted namespace.
    ///
    /// Pure substrate: no syscall. The components are joined under the
    /// mountpoint to produce an absolute path the caller can hand to
    /// cargo, erlc, or flang.
    pub fn path(&self, components: &[&str]) -> PathBuf {
        let mut p = self.mountpoint.clone();
        for c in components {
            p.push(c);
        }
        p
    }

    /// `read(p) -> imperfect` — fetch bytes at a Unix path inside the
    /// mounted handle.
    ///
    /// First consults the in-memory content map (the authoritative
    /// substrate view); falls back to a disk read if the entry isn't
    /// indexed (defensive — should not happen for v0 mounts, but keeps
    /// the lens robust if a future sub-species writes to the tempdir
    /// out-of-band).
    pub fn read(&self, p: &Path) -> Result<Vec<u8>, OpacityReason> {
        let rel = match relative_to(&self.mountpoint, p) {
            Some(r) => r,
            None => return Err(OpacityReason::PathAbsent(p.to_path_buf())),
        };
        if let Some(bytes) = self.content.get(&rel) {
            return Ok(bytes.clone());
        }
        match fs::read(p) {
            Ok(bytes) => Ok(bytes),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                Err(OpacityReason::PathAbsent(p.to_path_buf()))
            }
            Err(e) => Err(OpacityReason::ReadFailed(e.to_string())),
        }
    }

    /// `stat(p) -> verdict` — verdict on a Unix path within the mounted
    /// handle.
    ///
    /// The substrate's `verdict` carrier (pass | partial | failure). v0
    /// returns `Pass` if the path is in the content map OR present on
    /// disk under the mountpoint, `Failure` otherwise. `Partial` is
    /// reserved for future sub-species (see `Verdict::Partial`).
    pub fn stat(&self, p: &Path) -> Verdict {
        if let Some(rel) = relative_to(&self.mountpoint, p) {
            if self.content.contains_key(&rel) {
                return Verdict::Pass;
            }
            // Directory check: any content key starting with `rel/`?
            let mut prefix = rel.clone();
            prefix.push("");
            for k in self.content.keys() {
                if k.starts_with(&rel) && k != &rel {
                    return Verdict::Pass;
                }
            }
        }
        if p.exists() {
            Verdict::Pass
        } else {
            Verdict::Failure(format!("path absent: {}", p.display()))
        }
    }

    /// The mountpoint (absolute path to the tempdir root).
    ///
    /// Tests and downstream consumers use this to compute the
    /// `--manifest-path` for cargo, etc.
    pub fn mountpoint(&self) -> &Path {
        &self.mountpoint
    }
}

impl Drop for UnixLens {
    fn drop(&mut self) {
        if self.owns_mountpoint {
            // Best-effort cleanup; ignore errors (a half-deleted tempdir
            // is a tooling concern, not a correctness one).
            let _ = fs::remove_dir_all(&self.mountpoint);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internals.
//
// `make_tempdir`, `materialize_splinter`, and `relative_to` are the
// substrate-pull primitives the public surface composes. They live below
// the substrate surface (no `prism` declaration; no `\` obligation) — pure
// bootstrap-side mechanics for v0.

/// Create a fresh tempdir under `$TMPDIR` (or `/tmp`) with a
/// process-unique name. No external crate (`tempfile`, `tempdir`) —
/// keep the dependency surface minimal. Per
/// `[[feedback-substrate-already-had-the-word]]`: the substrate already
/// has the OID-as-uniqueness primitive, so we lean on a process PID +
/// monotonic counter for v0.
fn make_tempdir() -> io::Result<PathBuf> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let base = std::env::temp_dir();
    let pid = std::process::id();
    let name = format!("mirror-lens-unix-{}-{}", pid, n);
    let path = base.join(name);
    fs::create_dir_all(&path)?;
    Ok(path)
}

/// Walk a `Splinter`'s `Record` tree, materializing leaves as files
/// under `mountpoint/relative`. Updates `content` with the in-memory
/// index.
///
/// - `Content::Text(t)` at relative path `p` -> write `t.as_str()` as
///   UTF-8 bytes to `mountpoint/p`.
/// - `Content::Record(map)` at relative path `p` -> for each
///   `(FieldName, sub_splinter)`, recurse with relative path `p/name`.
/// - `Content::List(items)` at relative path `p` -> recurse with
///   relative path `p/0`, `p/1`, ... (the substrate's positional view of
///   the list at the path altitude).
fn materialize_splinter<H: MerkleHash>(
    splinter: &Splinter<H>,
    mountpoint: &Path,
    relative: PathBuf,
    content: &mut BTreeMap<PathBuf, Vec<u8>>,
) -> io::Result<()> {
    match splinter.content() {
        Content::Text(t) => {
            let abs = mountpoint.join(&relative);
            if let Some(parent) = abs.parent() {
                fs::create_dir_all(parent)?;
            }
            let bytes = t.as_str().as_bytes().to_vec();
            fs::write(&abs, &bytes)?;
            content.insert(relative, bytes);
            Ok(())
        }
        Content::Record(map) => {
            // Ensure the directory exists even if it's empty.
            let abs = mountpoint.join(&relative);
            fs::create_dir_all(&abs)?;
            for (name, child) in map {
                let mut child_rel = relative.clone();
                child_rel.push(field_name_as_path(name));
                materialize_splinter(child, mountpoint, child_rel, content)?;
            }
            Ok(())
        }
        Content::List(items) => {
            let abs = mountpoint.join(&relative);
            fs::create_dir_all(&abs)?;
            for (i, item) in items.iter().enumerate() {
                let mut child_rel = relative.clone();
                child_rel.push(i.to_string());
                materialize_splinter(item, mountpoint, child_rel, content)?;
            }
            Ok(())
        }
    }
}

/// Convert a `FieldName` to a path component string.
///
/// `FieldName::new` rejects whitespace; we still trust the caller to
/// have constructed names that are valid path components. Slashes within
/// a single FieldName WILL be interpreted as path separators by the
/// filesystem — this is the substrate's intended behaviour for
/// `src/lib.rs`-shaped keys (the caller chose a flat key with `/`
/// rather than a nested Record). v1 may tighten this; v0 trusts the
/// caller.
fn field_name_as_path(name: &FieldName) -> String {
    name.as_str().to_string()
}

/// Compute the relative portion of `p` w.r.t. `base`, or `None` if `p`
/// is not inside `base`. Canonicalizes neither side — for v0 we trust
/// the caller to pass already-absolute paths (the `path` action returns
/// joined-under-mountpoint paths, which satisfies this).
fn relative_to(base: &Path, p: &Path) -> Option<PathBuf> {
    p.strip_prefix(base).ok().map(|r| r.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crystallize::{Blake3, Content, FieldName, Splinter, Text};
    use std::collections::BTreeMap;

    /// Build a tiny shard: `{ "hello.txt": "hi\n" }`.
    fn one_file_shard(name: &str, body: &str) -> Splinter<Blake3> {
        let mut m: BTreeMap<FieldName, Splinter<Blake3>> = BTreeMap::new();
        m.insert(
            FieldName::new(name).expect("valid field name"),
            Splinter::new(Content::Text(Text::new(body))),
        );
        Splinter::new(Content::Record(m))
    }

    #[test]
    fn mount_materializes_single_file() {
        let shard = one_file_shard("hello.txt", "hi\n");
        let lens = UnixLens::mount(&shard).expect("mount succeeds");
        let p = lens.path(&["hello.txt"]);
        let bytes = lens.read(&p).expect("read succeeds");
        assert_eq!(bytes, b"hi\n");
    }

    #[test]
    fn stat_returns_pass_for_materialized_path() {
        let shard = one_file_shard("hello.txt", "hi\n");
        let lens = UnixLens::mount(&shard).expect("mount succeeds");
        let p = lens.path(&["hello.txt"]);
        assert_eq!(lens.stat(&p), Verdict::Pass);
    }

    #[test]
    fn stat_returns_failure_for_absent_path() {
        let shard = one_file_shard("hello.txt", "hi\n");
        let lens = UnixLens::mount(&shard).expect("mount succeeds");
        let p = lens.path(&["missing.txt"]);
        assert!(matches!(lens.stat(&p), Verdict::Failure(_)));
    }

    #[test]
    fn read_returns_path_absent_for_missing_file() {
        let shard = one_file_shard("hello.txt", "hi\n");
        let lens = UnixLens::mount(&shard).expect("mount succeeds");
        let p = lens.path(&["missing.txt"]);
        assert!(matches!(lens.read(&p), Err(OpacityReason::PathAbsent(_))));
    }

    #[test]
    fn nested_record_materializes_as_directory_tree() {
        // { src: { "lib.rs": "pub fn x() {}\n" } }
        let mut inner: BTreeMap<FieldName, Splinter<Blake3>> = BTreeMap::new();
        inner.insert(
            FieldName::new("lib.rs").unwrap(),
            Splinter::new(Content::Text(Text::new("pub fn x() {}\n"))),
        );
        let mut outer: BTreeMap<FieldName, Splinter<Blake3>> = BTreeMap::new();
        outer.insert(
            FieldName::new("src").unwrap(),
            Splinter::new(Content::Record(inner)),
        );
        let shard = Splinter::new(Content::Record(outer));
        let lens = UnixLens::mount(&shard).expect("mount succeeds");
        let p = lens.path(&["src", "lib.rs"]);
        let bytes = lens.read(&p).expect("read succeeds");
        assert_eq!(bytes, b"pub fn x() {}\n");
    }

    #[test]
    fn dropping_the_lens_removes_the_mountpoint() {
        let shard = one_file_shard("hello.txt", "hi\n");
        let path = {
            let lens = UnixLens::mount(&shard).expect("mount succeeds");
            lens.mountpoint().to_path_buf()
        };
        assert!(
            !path.exists(),
            "tempdir at {} should be cleaned up on drop",
            path.display()
        );
    }
}
