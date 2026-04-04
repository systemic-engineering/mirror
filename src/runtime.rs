//! Runtime trait — the compilation backend interface.
//!
//! Mirror defines the contract. Implementations live elsewhere.
//! - MetalRuntime: GPU kernels (in mirror, settled/cold path)
//! - RactorRuntime: ractor actors (in conversation, hot path)

use std::fmt;

use crate::check::Verified;
use crate::model::Mirror;

/// The compiler backend.
/// - compile: Verified → Mirror (pure, storable)
/// - spawn: Mirror → Handle (side effect, ephemeral — implemented by runtimes)
pub trait Runtime: Send + Sync {
    type Actor;
    type Error: fmt::Display + Send;

    fn compile(&self, verified: Verified) -> Result<prism::Beam<Mirror>, Self::Error>;
}

/// A runtime-level error.
#[derive(Debug)]
pub struct RuntimeError(pub String);

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
