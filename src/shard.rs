//! .shard — the executable artifact.
//!
//! A compiled .mirror grammar carrying its KernelSpec.
//! .mirror → compilation (Transport) → .shard
//!
//! The .shard IS the NakedSingularity seed.
//! No parent. Just IS.

use crate::declaration::MirrorHash;
use prism::{Decomposition, KernelSpec};

use super::bundle::Target;

/// A compiled .shard artifact.
/// Contains everything needed to execute: grammar OID + kernel spec + target.
#[derive(Clone, Debug)]
pub struct Shard {
    /// Content-addressed OID of the compiled grammar.
    pub grammar_oid: MirrorHash,
    /// The kernel specification for runtime dispatch.
    pub kernel_spec: KernelSpec,
    /// The compilation target.
    pub target: Target,
}

impl Shard {
    pub fn new(grammar_oid: MirrorHash, kernel_spec: KernelSpec, target: Target) -> Self {
        Shard {
            grammar_oid,
            kernel_spec,
            target,
        }
    }

    /// Number of preserved dimensions in the kernel.
    pub fn rank(&self) -> usize {
        self.kernel_spec.rank()
    }

    /// The decomposition type.
    pub fn decomposition(&self) -> Decomposition {
        self.kernel_spec.decomposition
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fragmentation::sha::HashAlg;
    use prism::Precision;

    #[test]
    fn shard_carries_kernel_spec() {
        let spec = KernelSpec::new(
            vec![0, 1, 2, 3],
            Decomposition::Eigenvalue,
            Precision::new(0.01),
        );
        let oid = MirrorHash::from_hex("abc123");
        let shard = Shard::new(oid, spec, Target::Beam);
        assert_eq!(shard.rank(), 4);
        assert_eq!(shard.decomposition(), Decomposition::Eigenvalue);
        assert_eq!(shard.target, Target::Beam);
    }

    #[test]
    fn shard_with_wasm_target() {
        let spec = KernelSpec::new(
            vec![0, 2, 4, 6, 8, 10, 12, 14],
            Decomposition::Svd,
            Precision::new(0.001),
        );
        let oid = MirrorHash::from_hex("def456");
        let shard = Shard::new(oid, spec, Target::Wasm);
        assert_eq!(shard.rank(), 8);
        assert_eq!(shard.target, Target::Wasm);
    }
}
