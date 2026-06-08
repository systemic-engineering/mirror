//! T20 — portal shard handoff + SCM_RIGHTS at the CLI pipe boundary.
//!
//! Terminal tick of the 5-tick cascade closing the pretty-printer LRM
//! (T16 → T17 → T18 → T19 → **T20**). The substrate's
//! `@mirror/spectral/portal` species (shards/mirror/spectral/portal.mirror)
//! declares the four-field carrier — `socket`, `subspace`,
//! `frame`, `actor` — with three of the four fields shaped as
//! `shift(oid, T)` (the 26th-instance typed-capability primitive).
//! This module is the OS-layer realisation: detect that stdout is a
//! socket capable of carrying ancillary messages, then send the
//! 96-byte three-OID frame and a SCM_RIGHTS-borne fd to the
//! fragmentation shard in one atomic sendmsg. The substrate state
//! never re-serialises; the OID names the shard, the fd carries the
//! shard's bytes, the downstream pipeline stage settles it.
//!
//! ## The "presence IS the signal" recognition
//!
//! Per Alex 2026-06-08, the load-bearing substrate-pull correction:
//!
//!   > *"The presence of the portal itself ought to be the signal.
//!     Think Rust `Box<>` memory ownership passing. It's a pointer.
//!     It's a Crystal in superposition. It's a fragmentation shard
//!     that settles through the pipeline."*
//!
//! No magic-byte handshake. No poll-for-ack. No protocol
//! negotiation. The TYPE of message — its cmsg list — carries the
//! meaning. A peer that does `recvmsg` and finds a SCM_RIGHTS
//! ancillary in the cmsg list KNOWS this is a portal handoff, the
//! same way reading `Box<T>` IS the signal of heap ownership
//! transfer. Same way deserialising a `fragmentation::Shard` from
//! its content-addressed reference IS the signal that the shard's
//! superposition just crossed the boundary.
//!
//! ## Wire protocol (this tick — the minimum viable LRM closure)
//!
//!   1. **Probe** — `fstat(stdout)`; if `S_IFSOCK` is not set, return
//!      `NoPortal::NotSocket` and the caller falls through to the
//!      text branch.
//!   2. **Stage** — `pipe(2)`; write the settled bytes (the
//!      fragmentation shard's content-addressed serialization) into
//!      the write end; close the write end so the peer's
//!      `read_to_end` on the receiving fd sees EOF.
//!   3. **One sendmsg** — `iov` carries the 96-byte three-OID frame
//!      (`[from_oid:32][to_oid:32][delta_oid:32]` per the substrate's
//!      `@mirror/spectral/portal/codec` wire format); `cmsg` carries
//!      the SCM_RIGHTS fd. The peer's `recvmsg` delivers both
//!      atomically. **Presence of the SCM_RIGHTS ancillary IS the
//!      portal signal.**
//!
//! `from_oid` and `delta_oid` are zeros this tick; the gen_prism
//! stream layer fills them in a follow-up tick once the Scheduler
//! Tower's bounded-iteration primitive ships. `to_oid` names the
//! shard's content-addressed identity at the time of handoff —
//! the receiver verifies by computing the same OID over the fd's
//! contents.
//!
//! Stage 1 is the cheap discriminator. Stages 2–3 are the substrate's
//! 26th-instance primitive's OS-layer realisation: `shift(oid, T)` at
//! the AF_UNIX local altitude. The fragmentation shard travels by
//! reference (the fd), not by value (no byte-by-byte re-serialisation
//! across the process boundary).
//!
//! ## Substrate-pull discipline
//!
//! Per `[[architecture-fragmentation-is-the-rust-substrate]]`: the
//! OS layer IS the Rust substrate at this altitude. Per
//! `[[feedback-substrate-already-had-the-word]]` (29th instance —
//! the LRM-closing recognition): SCM_RIGHTS itself IS the substrate's
//! typed-capability primitive at the OS altitude. The substrate's
//! WS-style handshake at `shards/mirror/spectral/portal/handshake.
//! mirror` is the *network* variant (RFC 6455 HTTP upgrade) where
//! both ends need an explicit protocol negotiation because the wire
//! is byte-only; the AF_UNIX local case has SCM_RIGHTS, which is the
//! kernel's own typed-capability discriminator — no negotiation
//! needed because the cmsg list IS the type tag. Same family, two
//! altitudes — the WS shape for cross-network portals, the
//! SCM_RIGHTS shape for local pipes. The substrate's
//! `@mirror/spectral/portal` family root names the discipline;
//! this module realises the local-pipe species.
//!
//! ## Graceful degradation
//!
//! Every fallible call returns through [`Imperfect`]; the discriminator's
//! `Failure` arm names the boundary via [`Transparency::opaque`] so the
//! caller can choose to text-fall-through silently (the CLI default) or
//! surface the opacity (debug mode, future).
//!
//! ## Cross-platform
//!
//! The whole module is `#![cfg(unix)]`. On non-unix (Windows),
//! `try_outbound_handoff` returns `NoPortal::Unsupported` via the
//! stub re-export at the bottom of the file; the caller's branch
//! reads identically and the Windows path takes the text branch
//! unconditionally.

#![allow(clippy::needless_return)]

use terni::{Diagnostic, Imperfect, PropertyVerdict, Transparency};

/// One-liner: tag a substrate boundary opacity with a short reason.
/// The `path` is the [`NoPortal`] variant (which substrate seam went
/// dark); the `verdict` carries a `Fail` diagnostic naming why.
fn opacity(path: NoPortal) -> Transparency<NoPortal> {
    let msg = format!("{:?}", path);
    Transparency::opaque(path, PropertyVerdict::Fail(Diagnostic::new(msg)))
}

/// The 96-byte three-OID wire frame per the substrate's
/// `@mirror/spectral/portal/codec` declaration (boot/std/spectral/
/// portal/codec.mirror): `[from_oid:32][to_oid:32][delta_oid:32]`.
/// Pinned as a constant so the unit test below catches accidental
/// drift if the substrate ever reshapes the wire format.
const FRAME_LEN: usize = 96;

/// The substrate's `[from_oid:32][to_oid:32][delta_oid:32]` frame
/// — same shape as `boot/std/fragmentation/frame.mirror`'s 96-byte
/// three-OID wire format.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PortalFrame {
    pub from_oid: [u8; 32],
    pub to_oid: [u8; 32],
    pub delta_oid: [u8; 32],
}

impl PortalFrame {
    /// Construct a frame with `to_oid` set and the other two slots
    /// zero (the minimum viable closure at the CLI boundary; the
    /// gen_prism stream layer fills `from_oid` and `delta_oid` in a
    /// follow-up tick).
    pub fn with_to_oid(to_oid: [u8; 32]) -> Self {
        Self {
            from_oid: [0u8; 32],
            to_oid,
            delta_oid: [0u8; 32],
        }
    }

    /// Serialise to the 96-byte wire form. Same layout the substrate's
    /// `@mirror/spectral/portal/codec.serialize` action names; this
    /// realises it at the OS altitude.
    pub fn to_bytes(&self) -> [u8; FRAME_LEN] {
        let mut buf = [0u8; FRAME_LEN];
        buf[0..32].copy_from_slice(&self.from_oid);
        buf[32..64].copy_from_slice(&self.to_oid);
        buf[64..96].copy_from_slice(&self.delta_oid);
        buf
    }

    /// Read a 96-byte wire frame back into the typed carrier. Same
    /// shape as `@mirror/spectral/portal/codec.deserialize`. Used by
    /// the inbound receiver surface (the consumer-pull tick).
    #[allow(dead_code)]
    pub fn from_bytes(buf: &[u8; FRAME_LEN]) -> Self {
        let mut from_oid = [0u8; 32];
        let mut to_oid = [0u8; 32];
        let mut delta_oid = [0u8; 32];
        from_oid.copy_from_slice(&buf[0..32]);
        to_oid.copy_from_slice(&buf[32..64]);
        delta_oid.copy_from_slice(&buf[64..96]);
        Self {
            from_oid,
            to_oid,
            delta_oid,
        }
    }
}

/// Boundary opacity — names *which* substrate seam went dark.
///
/// Carried inside [`Transparency`] when [`try_outbound_handshake`]
/// returns `Imperfect::Failure`; the caller can pattern-match to
/// decide between silent text-fallback (the default) and a stderr
/// trace (future debug mode).
///
/// `WriteFailed` / `ReadFailed` are reserved variants the
/// [`try_inbound_handshake`] (receiver-side) surface uses; the
/// outbound caller this tick collapses all sendmsg failures to
/// `PeerDeclined`. Both variants stay declared so the receiver
/// consumer-pull (next subcommand-wiring tick) doesn't need to
/// re-extend the enum.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum NoPortal {
    /// `fstat` says stdout is not a socket — text branch is correct.
    NotSocket,
    /// `fstat` failed at the syscall level (errno surfaces in the
    /// `i32`); text branch is the safe fallback.
    FstatFailed(i32),
    /// The peer did not respond with the portal magic — could be
    /// truncated read, wrong protocol, EOF, or genuine peer decline.
    PeerDeclined,
    /// Write failure during handshake/frame/SCM_RIGHTS — peer closed
    /// mid-protocol; text branch is no longer viable on this socket
    /// (the bytes are interleaved), but the caller still exits 0
    /// since the *kintsugi* settle succeeded.
    WriteFailed(i32),
    /// Read failure during handshake — peer didn't send the magic.
    ReadFailed(i32),
    /// Could not stage the settled bytes into a memfd / pipe for the
    /// SCM_RIGHTS handoff.
    StageFailed(i32),
    /// Non-unix build: portal detection is a no-op.
    #[allow(dead_code)]
    Unsupported,
}

impl PartialOrd for NoPortal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NoPortal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Discriminator ordering — purely for `Transparency<NoPortal>`'s
        // `P: Ord` bound. The semantic content is the variant, not the
        // order; deriving via `Debug` strings is stable and total.
        format!("{:?}", self).cmp(&format!("{:?}", other))
    }
}

/// What a successful outbound handoff produced. Empty this tick —
/// the side effects (24 bytes ack, 96 bytes frame, SCM_RIGHTS fd)
/// have already flushed by the time this returns. The caller reads
/// `Imperfect::Success(_)` and stops; no further bytes go to stdout.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PortalHandoff;

/// The probe + handshake + frame + SCM_RIGHTS quartet. Called from
/// `cmd_kintsugi_single` after settle, with the settled bytes and
/// the settled-ref OID. Returns `Imperfect::Success(PortalHandoff)`
/// when the peer accepted and the fd is across the wire;
/// `Imperfect::Failure(NoPortal::*, Transparency)` otherwise — the
/// caller falls through to the text branch.
#[cfg(unix)]
pub fn try_outbound_handshake(
    socket_fd: std::os::unix::io::RawFd,
    settled_bytes: &[u8],
    to_oid: [u8; 32],
) -> Imperfect<PortalHandoff, NoPortal, Transparency<NoPortal>> {
    use std::os::unix::io::FromRawFd;

    // Stage 1: discriminator — is stdout actually a socket?
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    let rc = unsafe { libc::fstat(socket_fd, &mut stat) };
    if rc < 0 {
        let err = unsafe { *libc::__error() };
        return Imperfect::Failure(
            NoPortal::FstatFailed(err),
            opacity(NoPortal::FstatFailed(err)),
        );
    }
    if (stat.st_mode & libc::S_IFMT) != libc::S_IFSOCK {
        return Imperfect::Failure(NoPortal::NotSocket, opacity(NoPortal::NotSocket));
    }

    // Stage 2: confirm the socket can carry SCM_RIGHTS at all. AF_UNIX
    // can; the standard sockets that can't (AF_INET, etc.) reject the
    // ancillary message at sendmsg time, but we can pre-discriminate
    // by reading `SO_DOMAIN` where available. macOS lacks SO_DOMAIN as
    // a getsockopt name; we let `sendmsg` itself be the discriminator
    // and read EINVAL/ENOTSUP/EOPNOTSUPP as "not portal-capable".
    //
    // **The presence of the portal is the signal** (per Alex's
    // substrate-pull, 2026-06-08): the SCM_RIGHTS ancillary message
    // IS the handshake. The peer that does `recvmsg` and finds a fd
    // in the cmsg list KNOWS this is a portal — the same way Rust's
    // `Box<T>` IS the signal of heap ownership transfer, no protocol
    // negotiation needed. No 24-byte magic exchange; no poll-for-ack;
    // no symmetric blocking. The discriminator IS the type of
    // message, not a side-channel announcement.
    //
    // The 96-byte three-OID frame travels as the iov payload of the
    // same sendmsg; the SCM_RIGHTS cmsg carries the fd. One syscall.
    // Atomic from the receiver's perspective (recvmsg delivers iov +
    // cmsg in one call).

    // Stage 3 (consolidated): stage the settled bytes into a pipe.
    // `pipe(2)` + write + close-write gives a read fd the peer can
    // drain; substrate state never re-serialises (the fd points at
    // kernel-managed in-memory bytes that already exist in this
    // process).
    let mut pipe_fds: [libc::c_int; 2] = [-1, -1];
    let rc = unsafe { libc::pipe(pipe_fds.as_mut_ptr()) };
    if rc < 0 {
        let err = unsafe { *libc::__error() };
        return Imperfect::Failure(
            NoPortal::StageFailed(err),
            opacity(NoPortal::StageFailed(err)),
        );
    }
    let read_fd = pipe_fds[0];
    let write_fd = pipe_fds[1];

    // Spill the settled bytes into the write end. For payloads
    // larger than the pipe buffer (typ. 64KB on macOS/Linux), we'd
    // need a memfd or a backgrounded writer thread; the
    // settled-source size is bounded by the input file, and the
    // pretty-printed mirror text is comfortably within pipe-buffer
    // size for the corpus we have today. Future tick lifts this to
    // memfd_create where available.
    if let Err(err) = write_all(write_fd, settled_bytes) {
        unsafe {
            libc::close(read_fd);
            libc::close(write_fd);
        }
        return Imperfect::Failure(
            NoPortal::StageFailed(err),
            opacity(NoPortal::StageFailed(err)),
        );
    }
    // Close the write end so the peer's read_to_end sees EOF.
    unsafe { libc::close(write_fd) };

    // Stage 4: the single sendmsg — iov carries the 96-byte three-OID
    // frame; cmsg carries the SCM_RIGHTS fd. The receiver's recvmsg
    // delivers both in one call; the presence of the SCM_RIGHTS
    // cmsg IS the portal signal.
    let frame = PortalFrame::with_to_oid(to_oid).to_bytes();
    let mut iov = libc::iovec {
        iov_base: frame.as_ptr() as *mut libc::c_void,
        iov_len: frame.len(),
    };

    // CMSG_SPACE(sizeof(int)) is typically 16 or 24 bytes; 64 is safe.
    let mut cmsg_buf = [0u8; 64];
    let mut msg: libc::msghdr = unsafe { std::mem::zeroed() };
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen =
        unsafe { libc::CMSG_SPACE(std::mem::size_of::<libc::c_int>() as u32) } as _;

    // Fill the cmsg header + payload.
    unsafe {
        let cmsg = libc::CMSG_FIRSTHDR(&msg);
        if cmsg.is_null() {
            libc::close(read_fd);
            return Imperfect::Failure(NoPortal::StageFailed(0), opacity(NoPortal::StageFailed(0)));
        }
        (*cmsg).cmsg_level = libc::SOL_SOCKET;
        (*cmsg).cmsg_type = libc::SCM_RIGHTS;
        (*cmsg).cmsg_len = libc::CMSG_LEN(std::mem::size_of::<libc::c_int>() as u32) as _;
        let data_ptr = libc::CMSG_DATA(cmsg) as *mut libc::c_int;
        *data_ptr = read_fd;
    }

    let sent = unsafe { libc::sendmsg(socket_fd, &msg, 0) };
    // Our side keeps no hold on read_fd after sendmsg; the kernel
    // duplicates it into the peer's process. Close locally either way.
    unsafe { libc::close(read_fd) };
    if sent < 0 {
        // `sendmsg` on a non-AF_UNIX socket returns EOPNOTSUPP /
        // EINVAL when SCM_RIGHTS is attached; treat any sendmsg
        // failure as the discriminator's "not portal-capable" arm and
        // fall through silently. The non-AF_UNIX socket case lands
        // here naturally without needing SO_DOMAIN. The errno is
        // observable via `libc::__error()` for future debug surfaces;
        // this tick collapses all failure modes to PeerDeclined.
        let _err = unsafe { *libc::__error() };
        return Imperfect::Failure(NoPortal::PeerDeclined, opacity(NoPortal::PeerDeclined));
    }

    // The io::stdout we received was borrowed via raw fd from
    // `cmd_kintsugi_single`; we do not own it, so we do not close
    // it here. The FromRawFd import above is intentionally unused —
    // kept for parity with the receiver-side helper below.
    let _ = std::os::unix::io::OwnedFd::from_raw_fd; // suppress unused

    Imperfect::Success(PortalHandoff)
}

/// Receiver-side: read the inbound portal frame + SCM_RIGHTS fd.
///
/// Symmetric to [`try_outbound_handshake`]. Called from the *consumer*
/// half of `mirror kintsugi <file> | mirror something` — the
/// downstream subcommand's stdin-detection step. This tick exports
/// the function so the symmetric protocol is namable; consumer wiring
/// (which subcommands probe stdin) lands in a follow-up tick that
/// also touches the per-subcommand input dispatch — out of scope for
/// the LRM-closing T20.
#[cfg(unix)]
#[allow(dead_code)]
pub fn try_inbound_handshake(
    socket_fd: std::os::unix::io::RawFd,
) -> Imperfect<(PortalFrame, std::os::unix::io::RawFd), NoPortal, Transparency<NoPortal>> {
    // Stage 1: discriminator.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    let rc = unsafe { libc::fstat(socket_fd, &mut stat) };
    if rc < 0 {
        let err = unsafe { *libc::__error() };
        return Imperfect::Failure(
            NoPortal::FstatFailed(err),
            opacity(NoPortal::FstatFailed(err)),
        );
    }
    if (stat.st_mode & libc::S_IFMT) != libc::S_IFSOCK {
        return Imperfect::Failure(NoPortal::NotSocket, opacity(NoPortal::NotSocket));
    }

    // Stage 2: single recvmsg gets both the 96-byte frame (iov) and
    // the SCM_RIGHTS fd (cmsg). **The presence of the SCM_RIGHTS
    // ancillary IS the portal signal** — symmetric to the outbound
    // sendmsg. Box<T> semantics: the type of the message IS the
    // meaning. A peer that didn't send SCM_RIGHTS is not a portal.
    let mut frame_buf = [0u8; FRAME_LEN];
    let mut iov = libc::iovec {
        iov_base: frame_buf.as_mut_ptr() as *mut libc::c_void,
        iov_len: frame_buf.len(),
    };
    let mut cmsg_buf = [0u8; 64];
    let mut msg: libc::msghdr = unsafe { std::mem::zeroed() };
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen = cmsg_buf.len() as _;

    let n = unsafe { libc::recvmsg(socket_fd, &mut msg, 0) };
    if n < 0 {
        let err = unsafe { *libc::__error() };
        return Imperfect::Failure(
            NoPortal::ReadFailed(err),
            opacity(NoPortal::ReadFailed(err)),
        );
    }
    if (n as usize) < FRAME_LEN {
        // Short read — not a portal handoff.
        return Imperfect::Failure(NoPortal::PeerDeclined, opacity(NoPortal::PeerDeclined));
    }
    let frame = PortalFrame::from_bytes(&frame_buf);

    // Walk cmsg list for SCM_RIGHTS — the presence of the fd IS the
    // signal that this peer offered a portal handoff.
    let mut found_fd: Option<libc::c_int> = None;
    let mut cmsg = unsafe { libc::CMSG_FIRSTHDR(&msg) };
    while !cmsg.is_null() {
        let level = unsafe { (*cmsg).cmsg_level };
        let cmsg_type = unsafe { (*cmsg).cmsg_type };
        if level == libc::SOL_SOCKET && cmsg_type == libc::SCM_RIGHTS {
            let data_ptr = unsafe { libc::CMSG_DATA(cmsg) } as *const libc::c_int;
            found_fd = Some(unsafe { *data_ptr });
            break;
        }
        cmsg = unsafe { libc::CMSG_NXTHDR(&msg, cmsg) };
    }
    match found_fd {
        Some(fd) => Imperfect::Success((frame, fd)),
        None => Imperfect::Failure(NoPortal::PeerDeclined, opacity(NoPortal::PeerDeclined)),
    }
}

#[cfg(unix)]
fn write_all(fd: std::os::unix::io::RawFd, buf: &[u8]) -> Result<(), i32> {
    let mut off = 0;
    while off < buf.len() {
        let n = unsafe {
            libc::write(
                fd,
                buf[off..].as_ptr() as *const libc::c_void,
                buf.len() - off,
            )
        };
        if n < 0 {
            return Err(unsafe { *libc::__error() });
        }
        if n == 0 {
            return Err(0);
        }
        off += n as usize;
    }
    Ok(())
}

// ── Non-unix stub ────────────────────────────────────────────────────
//
// Windows / WASI / etc. take the text branch unconditionally. The
// outbound helper exists but always returns `Unsupported`; the caller
// reads `Imperfect::Failure(_, _)` and falls through.

#[cfg(not(unix))]
pub fn try_outbound_handshake(
    _socket_fd: i32,
    _settled_bytes: &[u8],
    _to_oid: [u8; 32],
) -> Imperfect<PortalHandoff, NoPortal, Transparency<NoPortal>> {
    Imperfect::Failure(NoPortal::Unsupported, opacity(NoPortal::Unsupported))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portal_frame_roundtrips_through_bytes() {
        let mut to = [0u8; 32];
        for (i, slot) in to.iter_mut().enumerate() {
            *slot = (i as u8).wrapping_add(7);
        }
        let f = PortalFrame::with_to_oid(to);
        let bytes = f.to_bytes();
        assert_eq!(bytes.len(), FRAME_LEN);
        let back = PortalFrame::from_bytes(&bytes);
        assert_eq!(back, f);
        // The substrate's three-OID layout: from is zero, to is set, delta is zero.
        assert_eq!(&bytes[0..32], &[0u8; 32]);
        assert_eq!(&bytes[32..64], &to);
        assert_eq!(&bytes[64..96], &[0u8; 32]);
    }

    #[test]
    fn frame_length_matches_substrate_three_oid_wire_format() {
        // Per shards/mirror/spectral/portal/codec.mirror — the 96-byte
        // `[from_oid:32][to_oid:32][delta_oid:32]` wire shape. This
        // test pins the constant against accidental drift.
        assert_eq!(FRAME_LEN, 96);
    }

    #[test]
    fn portal_frame_default_is_zeroed_quadruple() {
        // The Default impl produces the all-zero frame. This IS the
        // substrate's "null shard" sentinel — the from/delta zeros at
        // CLI-altitude handoff (no prior anchor, no per-tick delta
        // exposed yet), and the to_oid zeros if the caller forgets
        // to seed it. The receiver reads all zeros as "no shard
        // pointed at" — same as fragmentation's empty subspace.
        let f = PortalFrame::default();
        assert_eq!(f.from_oid, [0u8; 32]);
        assert_eq!(f.to_oid, [0u8; 32]);
        assert_eq!(f.delta_oid, [0u8; 32]);
        let bytes = f.to_bytes();
        assert_eq!(bytes, [0u8; FRAME_LEN]);
    }
}
