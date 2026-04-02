//! Actor lifecycle: observe, init, spawn, mount, status.
//!
//! `conversation actor observe <home> <repo>` — the first verb.
//! The actor reads a repo's grammars and emits a flake.nix.

pub mod emit_nix;
pub mod init;
pub mod mount;
pub mod observe;
