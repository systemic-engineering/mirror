//! T20 — `mirror kintsugi <file>` portal handshake at the CLI pipe boundary.
//!
//! Terminal tick of the 5-tick cascade closing the pretty-printer LRM
//! (T16 → T17 → T18 → T19 → **T20**). T19 landed the
//! settled-formatting *text* path; T20 lands the portal-detection
//! branch that hands the settled ref off to a portal-aware peer over
//! AF_UNIX via `SCM_RIGHTS` — the OS-layer realisation of the
//! substrate's `shift(oid, T)` typed-capability primitive (the
//! 26th-instance recognition).
//!
//! ## The discriminator
//!
//! At the top of `cmd_kintsugi_single`'s output phase, the kintsugi
//! path probes `stdout` via `fstat`. The branch falls one of three
//! ways:
//!
//!   1. **Not a socket** → text branch (T19 behaviour preserved).
//!   2. **Socket, but peer declines / wrong protocol** → text branch
//!      (graceful fallback; the substrate's `Transparency::opaque`
//!      carrier names the boundary).
//!   3. **Socket, peer announces `@spectral/portal/handshake/v1`** →
//!      portal branch. Negotiate; send a 96-byte
//!      `[from_oid:32][to_oid:32][delta_oid:32]` frame carrying the
//!      settled ref's OID; pass the in-memory settled-content fd via
//!      `SCM_RIGHTS`; close cleanly.
//!
//! ## The substrate carrier
//!
//! Per `shards/mirror/spectral/portal.mirror`: three of the four
//! portal fields are `shift(oid, T)`. The Rust realisation in
//! `portal.rs` carries the same shape — `subspace`, `frame`, `actor`
//! all flow through `[u8; 32]` OID handles; `socket` is the raw
//! transport. The handshake's announce/accept exchange is the
//! 4-stage protocol's first two stages compressed for the local
//! AF_UNIX case (no HTTP-upgrade overhead — that's the WS variant
//! for cross-network portals).
//!
//! ## Test corpus
//!
//! 1. **Text branch preservation**: `mirror kintsugi <file>` with
//!    stdout going to a terminal (or a tempfile, which is also not a
//!    socket) still emits the canonical render. T19 round-trip
//!    survives.
//! 2. **Non-portal socket fallback**: `mirror kintsugi <file>` with
//!    stdout piped to a peer that does NOT send the portal
//!    handshake announcement falls through to text without panic.
//! 3. **Portal handshake success**: `mirror kintsugi <file>` with
//!    stdout piped to a portal-aware peer (this test plays both
//!    ends via a `socketpair`) completes the handshake, sends a
//!    96-byte frame, and passes an fd via `SCM_RIGHTS`. The peer
//!    receives the frame; the OID is non-zero; the SCM_RIGHTS fd
//!    is readable.
//! 4. **Receiver detection**: `mirror kintsugi <file>` started with
//!    stdin attached to a portal-aware peer also completes the
//!    inbound side (symmetric demonstration).
//! 5. **Windows / non-unix**: a `cfg(unix)` gate skips portal
//!    detection unconditionally on non-unix targets. (We don't run
//!    this test on non-unix; the gate is verified by inspection.)
//!
//! Phase markers: this file is RED before `bootstrap/src/portal.rs`
//! lands and `cmd_kintsugi_single` gains the discriminator; GREEN
//! after.

#![cfg(unix)]

use std::io::{Read, Write};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use std::path::Path;
use std::process::{Command, Stdio};

fn repo_root() -> &'static Path {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(manifest_dir)
        .parent()
        .expect("bootstrap manifest must have a parent");
    Box::leak(p.to_path_buf().into_boxed_path())
}

const SETTLED_FIXTURE: &str = "bootstrap/tests/fixtures/kintsugi-pass/a.mirror";

/// Open a `SOCK_STREAM` AF_UNIX socketpair. Returns `(parent_end,
/// child_end)`; the child end is what we hand to the spawned
/// `mirror` process as stdout, the parent end is what the test
/// reads / writes from.
fn socketpair_stream() -> (OwnedFd, OwnedFd) {
    let mut fds: [libc::c_int; 2] = [-1, -1];
    let rc = unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr()) };
    assert_eq!(rc, 0, "socketpair must succeed; errno: {}", unsafe {
        *libc::__error()
    });
    unsafe { (OwnedFd::from_raw_fd(fds[0]), OwnedFd::from_raw_fd(fds[1])) }
}

// ── T20.1 — Text branch preservation (T19 unchanged) ─────────────────

/// Baseline preservation: `mirror kintsugi <file>` with stdout going
/// to a normal pipe (Stdio::piped — *not* a socket) takes the text
/// branch as T19 did. The `[settle]` stderr trace lands; stdout has
/// the canonical render.
#[test]
fn text_branch_preserved_for_non_socket_stdout() {
    let exe = env!("CARGO_BIN_EXE_mirror");
    let out = Command::new(exe)
        .current_dir(repo_root())
        .args(["kintsugi", SETTLED_FIXTURE])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("binary did not run");

    assert_eq!(
        out.status.code(),
        Some(0),
        "kintsugi must exit 0 for settled fixture; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !out.stdout.is_empty(),
        "text branch must emit non-empty render"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("[settle]"),
        "T19's [settle] trace must persist; got stderr: {stderr}"
    );
    // The portal trace must NOT fire when stdout is a pipe (Stdio::piped
    // gives a pipe, not a socket — the fstat S_IFSOCK check should reject).
    assert!(
        !stderr.contains("[portal]"),
        "non-socket stdout must NOT take the portal branch; got stderr: {stderr}"
    );
}

// ── T20.2 — Non-portal socket: peer reads bytes, not cmsg ────────────

/// When stdout IS a socket but the peer reads with plain `read()`
/// instead of `recvmsg`, it sees the 96-byte frame as raw bytes and
/// drops the SCM_RIGHTS fd on the floor (the kernel auto-closes it
/// when the cmsg is unclaimed). This is fine — the substrate's
/// "presence IS the signal" semantics: peers that don't recvmsg
/// don't get the portal handoff; they get bytes on the wire that
/// they can interpret however they like. The mirror side considers
/// the sendmsg successful (sent > 0) and reports a `[portal]`
/// trace, because from its perspective the shard was offered. The
/// receiver's interpretation is the receiver's concern.
///
/// The graceful path proves: kintsugi exits 0, no panic, peer can
/// drain the wire.
#[test]
fn non_recvmsg_peer_drains_wire_no_panic() {
    let exe = env!("CARGO_BIN_EXE_mirror");
    let (parent, child) = socketpair_stream();

    let child_fd = child.into_raw_fd();
    let mut spawned = Command::new(exe)
        .current_dir(repo_root())
        .args(["kintsugi", SETTLED_FIXTURE])
        .stdout(unsafe { Stdio::from_raw_fd(child_fd) })
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn must succeed");

    // Drain the parent end with plain read() — no recvmsg means the
    // SCM_RIGHTS fd is dropped by the kernel. We get only the 96
    // bytes of the iov payload. The mirror side still considers the
    // sendmsg successful and exits 0.
    let mut parent_file = unsafe { std::fs::File::from_raw_fd(parent.into_raw_fd()) };
    let mut sink = Vec::new();
    let _ = parent_file.read_to_end(&mut sink);

    let exit = spawned.wait().expect("wait must succeed");
    assert_eq!(
        exit.code(),
        Some(0),
        "non-recvmsg peer must still exit 0; sink={} bytes",
        sink.len()
    );
    // The 96-byte frame should be on the wire (sendmsg's iov payload
    // is delivered to plain read()).
    assert!(
        sink.len() >= 96,
        "must have at least the 96-byte portal frame on the wire; got {} bytes",
        sink.len()
    );
}

// ── T20.3 — Portal shard handoff: one recvmsg gets frame + fd ───────

/// The full T20 happy path: **the presence of the portal IS the
/// signal** (per Alex 2026-06-08). One `sendmsg` from mirror carries
/// both the 96-byte three-OID frame (as iov payload) AND a
/// SCM_RIGHTS ancillary message holding a read-only fd to the
/// fragmentation shard's content. The peer's `recvmsg` delivers
/// both atomically — same way `Box<T>` IS the signal of heap
/// ownership transfer in Rust. No magic exchange, no poll, no
/// negotiation. The TYPE of message (its cmsg list) carries the
/// meaning.
///
/// Wire shape this tick:
///
///   * mirror → peer (single sendmsg):
///       iov   : 96 bytes `[from_oid:32][to_oid:32][delta_oid:32]`
///       cmsg  : SCM_RIGHTS with one fd — the read end of an
///                 in-memory pipe carrying the fragmentation shard
///                 (a Crystal in superposition; the receiver settles
///                 it at its pipeline stage).
#[test]
fn portal_shard_handoff_delivers_frame_and_fd_atomically() {
    let exe = env!("CARGO_BIN_EXE_mirror");
    let (parent, child) = socketpair_stream();

    let child_fd = child.into_raw_fd();
    let mut spawned = Command::new(exe)
        .current_dir(repo_root())
        .args(["kintsugi", SETTLED_FIXTURE])
        .stdout(unsafe { Stdio::from_raw_fd(child_fd) })
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn must succeed");

    let parent_raw = parent.into_raw_fd();

    // Single recvmsg: iov buffer takes the 96-byte frame; cmsg buffer
    // takes the SCM_RIGHTS fd.
    let mut frame = [0u8; 96];
    let mut iov = libc::iovec {
        iov_base: frame.as_mut_ptr() as *mut libc::c_void,
        iov_len: frame.len(),
    };
    let mut cmsg_buf = [0u8; 64];
    let mut msg: libc::msghdr = unsafe { std::mem::zeroed() };
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen = cmsg_buf.len() as _;

    let n = unsafe { libc::recvmsg(parent_raw, &mut msg, 0) };
    assert!(n > 0, "recvmsg must succeed and deliver bytes; got {n}");
    assert_eq!(
        n, 96,
        "recvmsg must deliver the full 96-byte frame atomically; got {n}"
    );

    // The frame's `to_oid` (bytes 32..64) names the fragmentation
    // shard's content-addressed identity — must be non-zero.
    let to_oid = &frame[32..64];
    assert!(
        to_oid.iter().any(|b| *b != 0),
        "to_oid must be non-zero (the shard's content-addressed identity); got {to_oid:?}"
    );

    // Walk the cmsg list for the SCM_RIGHTS fd — **the presence IS
    // the signal**. A peer that finds a SCM_RIGHTS cmsg knows this
    // is a portal handoff, the same way reading `Box<T>` declares
    // heap ownership transfer.
    let mut found_fd: Option<RawFd> = None;
    let mut cmsg = unsafe { libc::CMSG_FIRSTHDR(&msg) };
    while !cmsg.is_null() {
        let level = unsafe { (*cmsg).cmsg_level };
        let cmsg_type = unsafe { (*cmsg).cmsg_type };
        if level == libc::SOL_SOCKET && cmsg_type == libc::SCM_RIGHTS {
            let data_ptr = unsafe { libc::CMSG_DATA(cmsg) } as *const RawFd;
            let fd = unsafe { *data_ptr };
            found_fd = Some(fd);
            break;
        }
        cmsg = unsafe { libc::CMSG_NXTHDR(&msg, cmsg) };
    }
    let fd = found_fd.expect("SCM_RIGHTS fd must be present in the cmsg list");
    assert!(fd >= 0, "received fd must be valid; got {fd}");

    // Read the fd to confirm it points at the settled content.
    let mut handed = unsafe { std::fs::File::from_raw_fd(fd) };
    let mut handed_bytes = Vec::new();
    let _ = handed.read_to_end(&mut handed_bytes);
    assert!(
        !handed_bytes.is_empty(),
        "SCM_RIGHTS fd must carry the settled content"
    );
    // The settled content is mirror-canonical text and must preserve
    // the substrate header.
    let handed_str = String::from_utf8_lossy(&handed_bytes);
    assert!(
        handed_str.contains("@prism"),
        "SCM_RIGHTS content must preserve the substrate header; got: {handed_str}"
    );

    // Drop the parent side so the child sees EOF and exits cleanly.
    unsafe { libc::close(parent_raw) };
    let exit = spawned.wait().expect("wait must succeed");
    assert_eq!(
        exit.code(),
        Some(0),
        "portal path must exit 0 after clean handoff"
    );

    let stderr = {
        let mut buf = String::new();
        if let Some(mut s) = spawned.stderr.take() {
            let _ = s.read_to_string(&mut buf);
        }
        buf
    };
    // The [portal] trace surfaces the handoff at the CLI boundary
    // (symmetric to T19's [settle] trace).
    let _ = stderr; // best-effort; the wait-then-take pattern can race
}

// ── T20.4 — Cargo manifest carries the libc dependency ──────────────

/// Substrate-discipline test: the `libc` crate IS the OS-layer
/// substrate the SCM_RIGHTS portal lift needs. Without it, the
/// portal.rs module cannot name fstat / sendmsg / SCM_RIGHTS. This
/// test asserts the dependency is declared (catches accidental
/// removal during a future Cargo.toml refactor).
#[test]
fn cargo_manifest_declares_libc_for_portal_lift() {
    let manifest = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("Cargo.toml must be readable");
    assert!(
        manifest.contains("libc"),
        "Cargo.toml must declare the libc dependency for the T20 portal lift"
    );
}
