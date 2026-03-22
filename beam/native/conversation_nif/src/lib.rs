//! Rustler NIF bridge for the conversation crate.
//!
//! Exposes parse_conv/1 and compile_grammar/1 to the BEAM runtime.

use rustler::{Atom, Binary, Encoder, Env, NewBinary};

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

/// Compile .conv grammar source → ETF bytes for actor dispatch module.
///
/// Returns `{ok, Binary}` or `{error, ErrorString}`.
/// Binary contains ETF-encoded EAF ready for `compile:forms/1`.
#[rustler::nif(schedule = "DirtyCpu")]
fn compile_grammar<'a>(env: Env<'a>, source: String) -> (Atom, rustler::Term<'a>) {
    match conversation::ffi::compile_grammar_to_etf(&source) {
        Ok(etf) => {
            let mut binary = NewBinary::new(env, etf.len());
            binary.as_mut_slice().copy_from_slice(&etf);
            (atoms::ok(), Binary::from(binary).to_term(env))
        }
        Err(e) => (atoms::error(), e.encode(env)),
    }
}

rustler::init!("conversation_nif");
