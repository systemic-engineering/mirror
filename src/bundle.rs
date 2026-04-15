//! Mirror's Bundle — compilation target.

/// Compilation target.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Target {
    /// BEAM (Erlang VM)
    #[default]
    Beam,
    /// WebAssembly
    Wasm,
    /// Metal/GPU
    Metal,
}
