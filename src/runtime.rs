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

// ---------------------------------------------------------------------------
// MetalRuntime — compile a Prism to Metal instructions and execute them.
// ---------------------------------------------------------------------------

use prism::metal::Instruction;
use prism::Prism as PrismTrait;

/// The MetalRuntime trait: compiles a Prism to Metal and executes it.
///
/// Prism describes. Mirror compiles. Metal executes.
///
/// Implementors provide `compile`, which translates a Prism value into a
/// flat sequence of Metal instructions. `execute` has a default implementation
/// that delegates to `prism::metal::execute`, so most implementors only need
/// to implement `compile`.
pub trait MetalRuntime<P: PrismTrait> {
    /// Compile a Prism into Metal instructions.
    fn compile(&self, prism: &P) -> Vec<Instruction>;

    /// Execute Metal instructions with input.
    ///
    /// Default implementation delegates to `prism::metal::execute`.
    fn execute(&self, program: &[Instruction], input: &[u8]) -> Vec<u8> {
        prism::metal::execute(program, input)
    }
}
