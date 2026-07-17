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

fn walk_into(dir: &Path, out: &mut Vec<WalkEntry>) -> io::Result<()> {
    // Skip walker-invisible directories at the @io boundary — .git and
    // target/ are substrate-invisible per walker discipline. NOT a
    // policy decision; both are non-substrate by construction (VCS
    // metadata; build artifacts).
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
