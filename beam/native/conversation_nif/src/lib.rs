//! Rustler NIF bridge for the conversation crate.
//!
//! Exposes parse_conv/1, compile_grammar/1, and coincidence measurement
//! functions to the BEAM runtime.

use mirror::Vector;
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
    match mirror::ffi::parse_to_oid(&source) {
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
    match mirror::ffi::compile_grammar_to_etf(&source) {
        Ok(etf) => {
            let mut binary = NewBinary::new(env, etf.len());
            binary.as_mut_slice().copy_from_slice(&etf);
            (atoms::ok(), Binary::from(binary).to_term(env))
        }
        Err(e) => (atoms::error(), e.encode(env)),
    }
}

/// Compile with phase OIDs for traced compilation chain.
///
/// Returns `{ok, {Binary, ParseOid, ResolveOid, CompileOid}}` or `{error, ErrorString}`.
/// Each OID is a hex-encoded SHA-512 content address for that compilation phase.
#[rustler::nif(schedule = "DirtyCpu")]
fn compile_grammar_traced<'a>(env: Env<'a>, source: String) -> (Atom, rustler::Term<'a>) {
    match mirror::ffi::compile_grammar_with_phases(&source) {
        Ok(result) => {
            let mut binary = NewBinary::new(env, result.etf.len());
            binary.as_mut_slice().copy_from_slice(&result.etf);
            let etf_term = Binary::from(binary).to_term(env);
            let inner = (
                etf_term,
                result.parse_oid,
                result.resolve_oid,
                result.compile_oid,
            )
                .encode(env);
            (atoms::ok(), inner)
        }
        Err(e) => (atoms::error(), e.encode(env)),
    }
}

// ---------------------------------------------------------------------------
// Coincidence measurement NIFs
// ---------------------------------------------------------------------------

/// Parse source into a Mirror for property checking.
///
/// Shared helper for all measurement NIFs. Parses the source, finds the
/// grammar block, and compiles it into a Mirror.
fn domain_from_source(source: &str) -> Result<mirror::model::Mirror, String> {
    let ast = mirror::parse::Parse
        .trace(source.to_string())
        .into_result()
        .map_err(|e| e.to_string())?;
    let grammar = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .ok_or_else(|| "no grammar block".to_string())?;
    mirror::model::Mirror::from_grammar(grammar)
}

/// Check a built-in property by name against a grammar source (internal helper).
///
/// Returns `(ok, reason)` or `(error, reason)`.
fn do_check_property(source: &str, property: &str) -> (Atom, String) {
    let domain = match domain_from_source(source) {
        Ok(r) => r,
        Err(e) => return (atoms::error(), e),
    };
    match mirror::property::check_builtin(&domain, property) {
        Some((true, reason)) => (atoms::ok(), reason),
        Some((false, reason)) => (atoms::error(), reason),
        None => (atoms::error(), format!("unknown property: {}", property)),
    }
}

/// Check a built-in property by name against a grammar source.
///
/// Returns `{ok, ReasonString}` or `{error, ReasonString}`.
#[rustler::nif]
fn check_property(source: String, property: String) -> (Atom, String) {
    do_check_property(&source, &property)
}

/// Check shannon equivalence — content address uniqueness across all derivations.
///
/// Returns `{ok, ReasonString}` or `{error, ReasonString}`.
#[rustler::nif]
fn check_shannon_equivalence(source: String) -> (Atom, String) {
    do_check_property(&source, "shannon_equivalence")
}

/// Check type graph connectivity via spectral analysis.
///
/// Returns `{ok, ReasonString}` or `{error, ReasonString}`.
#[rustler::nif]
fn check_connected(source: String) -> (Atom, String) {
    do_check_property(&source, "connected")
}

/// Check type graph bipartiteness via spectral analysis.
///
/// Returns `{ok, ReasonString}` or `{error, ReasonString}`.
#[rustler::nif]
fn check_bipartite(source: String) -> (Atom, String) {
    do_check_property(&source, "bipartite")
}

/// Check that every declared type has at least one variant.
///
/// Returns `{ok, ReasonString}` or `{error, ReasonString}`.
#[rustler::nif]
fn check_exhaustive(source: String) -> (Atom, String) {
    do_check_property(&source, "exhaustive")
}

rustler::init!("conversation_nif");
