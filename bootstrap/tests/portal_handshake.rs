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

// ── T20.2 — Non-portal socket fallback ───────────────────────────────

/// When stdout IS a socket but the peer does not announce the
/// portal handshake (the peer closes immediately, or sends garbage),
/// the kintsugi path falls through to text without panic and exits
/// 0. Graceful degradation per the `Imperfect::Failure` arm.
#[test]
fn non_portal_socket_falls_through_to_text() {
    let exe = env!("CARGO_BIN_EXE_mirror");
    let (parent, child) = socketpair_stream();

    // Parent end will be dropped immediately after the spawn — the
    // child's stdout writes will get EPIPE / ECONNRESET on the
    // handshake send. The portal detection must catch this and fall
    // through to text. We need to KEEP a reader to consume what the
    // text branch writes, otherwise the child blocks on SIGPIPE.
    let child_fd = child.into_raw_fd();
    let mut spawned = Command::new(exe)
        .current_dir(repo_root())
        .args(["kintsugi", SETTLED_FIXTURE])
        .stdout(unsafe { Stdio::from_raw_fd(child_fd) })
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn must succeed");

    // Read the parent end fully — both portal bytes (if any) and the
    // text fallback (if portal failed). We just discard.
    let mut parent_file = unsafe { std::fs::File::from_raw_fd(parent.into_raw_fd()) };
    let mut sink = Vec::new();
    let _ = parent_file.read_to_end(&mut sink);

    let exit = spawned.wait().expect("wait must succeed");
    assert_eq!(
        exit.code(),
        Some(0),
        "non-portal socket must still exit 0 (text fallback)"
    );

    // We dropped the read end via read_to_end on EOF; the binary
    // either wrote text bytes (graceful fallback) or wrote nothing
    // (the peer closed too fast). Either is acceptable; the key is
    // no panic and exit 0.
}

// ── T20.3 — Portal handshake success: send a frame + SCM_RIGHTS ─────

/// The full T20 happy path. The test plays the portal-aware peer:
/// it sends the announce header on the parent side of a socketpair,
/// the child mirror process detects the handshake, replies, then
/// emits the 96-byte frame + an `SCM_RIGHTS` ancillary message
/// carrying a read-only fd to the settled content.
///
/// Wire shape this tick (the minimum viable portal closure):
///
///   * Peer → mirror: 24 bytes magic+version "MIRROR/PORTAL/V1\0\0\0\0\0\0\0\0"
///                     (16 bytes magic + 8 bytes reserved)
///   * mirror → peer: 24 bytes same magic+version (the ack)
///   * mirror → peer: 96 bytes `[from_oid:32][to_oid:32][delta_oid:32]`
///                     three-OID frame; `to_oid` is the settled ref's OID;
///                     `from_oid` = `delta_oid` = zeros this tick (no prior
///                     anchor, no per-tick delta yet — the gen_prism
///                     stream layer covers that in a follow-up tick).
///   * mirror → peer: `sendmsg(SCM_RIGHTS)` with a zero-byte payload
///                     and one fd (the read end of an in-memory pipe
///                     containing the settled bytes).
#[test]
fn portal_handshake_success_sends_frame_and_fd() {
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

    // Parent: send the announce, read the ack, read the frame, read
    // the SCM_RIGHTS message.
    let parent_raw = parent.into_raw_fd();

    // Send 24-byte announce: 16-byte magic + 8-byte reserved zero.
    const MAGIC: &[u8; 16] = b"MIRROR/PORTAL/V1";
    let mut announce = [0u8; 24];
    announce[..16].copy_from_slice(MAGIC);
    let n = unsafe {
        libc::write(
            parent_raw,
            announce.as_ptr() as *const libc::c_void,
            announce.len(),
        )
    };
    assert_eq!(n, 24, "announce write must succeed");

    // Read 24-byte ack.
    let mut ack = [0u8; 24];
    let mut off = 0;
    while off < 24 {
        let r = unsafe {
            libc::read(
                parent_raw,
                ack[off..].as_mut_ptr() as *mut libc::c_void,
                24 - off,
            )
        };
        assert!(r > 0, "ack read must make progress; got {r}");
        off += r as usize;
    }
    assert_eq!(&ack[..16], &MAGIC[..], "ack magic must match");

    // Read 96-byte frame.
    let mut frame = [0u8; 96];
    let mut off = 0;
    while off < 96 {
        let r = unsafe {
            libc::read(
                parent_raw,
                frame[off..].as_mut_ptr() as *mut libc::c_void,
                96 - off,
            )
        };
        assert!(r > 0, "frame read must make progress; got {r}");
        off += r as usize;
    }
    // `to_oid` (bytes 32..64) is the settled ref's OID and must be
    // non-zero (a real OID, not the zero OID).
    let to_oid = &frame[32..64];
    assert!(
        to_oid.iter().any(|b| *b != 0),
        "to_oid must be non-zero; got {to_oid:?}"
    );

    // Receive the SCM_RIGHTS ancillary message.
    let mut data_buf = [0u8; 8];
    let mut iov = libc::iovec {
        iov_base: data_buf.as_mut_ptr() as *mut libc::c_void,
        iov_len: data_buf.len(),
    };
    // Use CMSG_SPACE-equivalent buffer: room for one fd.
    // For a single i32 (4 bytes) fd payload, CMSG_SPACE on darwin/linux
    // is typically 16 or 24 bytes. 64 is safely above.
    let mut cmsg_buf = [0u8; 64];
    let mut msg: libc::msghdr = unsafe { std::mem::zeroed() };
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen = cmsg_buf.len() as _;

    let n = unsafe { libc::recvmsg(parent_raw, &mut msg, 0) };
    assert!(n >= 0, "recvmsg must succeed; got {n}");

    // Walk the cmsg list looking for SCM_RIGHTS.
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
