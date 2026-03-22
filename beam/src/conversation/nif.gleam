//// NIF — bridge to the Rust conversation crate.
////
//// Calls the Rustler NIF compiled from `beam/native/conversation_nif`.
//// On load, the NIF is loaded from `priv/conversation_nif.so`.
////
//// Build the NIF before running tests:
////   just build-nif

/// Parse a .conv source string.
/// Returns the content OID of the parsed tree on success,
/// or an error message string on failure.
@external(erlang, "conversation_nif", "parse_conv")
pub fn parse_conv(source: String) -> Result(String, String)

/// Compile a .conv grammar source into ETF bytes.
/// Returns ETF-encoded EAF ready for `compile:forms/1` on success,
/// or an error message string on failure.
@external(erlang, "conversation_nif", "compile_grammar")
pub fn compile_grammar(source: String) -> Result(BitArray, String)
