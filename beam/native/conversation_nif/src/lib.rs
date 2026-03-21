//! Rustler NIF bridge for the conversation crate.
//!
//! Replaces the C NIF wrapper with safe Rust.
//! Exposes parse_conv/1 to the BEAM runtime.

use rustler::Atom;

mod atoms {
    rustler::atoms! {
        ok,
        error,
    }
}

/// Parse .conv source → content OID.
///
/// Returns `{ok, OidString}` or `{error, ErrorString}`.
#[rustler::nif]
fn parse_conv(source: String) -> (Atom, String) {
    match conversation::ffi::parse_to_oid(&source) {
        Ok(oid) => (atoms::ok(), oid),
        Err(e) => (atoms::error(), e),
    }
}

rustler::init!("conversation_nif");
