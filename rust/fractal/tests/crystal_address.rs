//! The Crystal content address keeps its documented guarantee.
//!
//! `crystallize`'s docblock (`rust/fractal/src/crystal.rs`) promises:
//! same address iff identical serialized bytes. An XOR fold over the
//! input bytes can't keep that promise: any two inputs whose bytes
//! differ only by the same value at positions `p` and `p + 32` fold to
//! the same 32 bytes. Taut's scout `26ed4c9`
//! (`docs/scouts/2026-10-29-taut-liquid-types-inference-statespace.md`
//! §4 link 2, §6 item 9) names the collision pair; BLAKE3 over
//! canonical bytes is Alex's decision (fixpoint spec decision 5).
//!
//! These tests hold the address to the docblock. They don't test which
//! hash is used.

use fractal::{crystallize, Author, Committer, Oid, Timestamp, Witnessed};

fn witnessed() -> Witnessed {
    Witnessed::new(
        Author::new("reed", "reed@systemic.engineer"),
        Committer::new("mirror", "mirror@spectral.engineer"),
        Timestamp("1700000000".to_string()),
    )
}

/// Taut's pair: two 33-byte inputs that differ only at bytes 0 and 32,
/// by the same value.
#[test]
fn taut_collision_pair_gets_distinct_addresses() {
    let a = vec![0u8; 33];
    let mut b = vec![0u8; 33];
    b[0] = 0x01;
    b[32] = 0x01;
    assert_ne!(a, b, "the inputs are distinct");

    let ca = crystallize(a, witnessed(), Oid::GENESIS);
    let cb = crystallize(b, witnessed(), Oid::GENESIS);
    assert_ne!(
        ca.oid, cb.oid,
        "distinct serialized bytes must get distinct addresses \
         (crystal.rs docblock: same address iff identical bytes)"
    );
}

/// Deterministic xorshift64*: enough randomness for a property check
/// without a new dependency.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    fn bytes(&mut self, len: usize) -> Vec<u8> {
        (0..len).map(|_| self.next() as u8).collect()
    }
}

/// Property: random distinct inputs get distinct addresses. Half the
/// pairs are independent random byte strings; the other half are a
/// random input and a copy perturbed by the same nonzero byte at `p`
/// and `p + 32`, the shape a fold of width 32 can't see.
#[test]
fn random_distinct_inputs_get_distinct_addresses() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for round in 0..512 {
        let len = 33 + rng.below(96);
        let a = rng.bytes(len);
        let b = if round % 2 == 0 {
            let mut b = a.clone();
            let p = rng.below(len - 32);
            let d = 1 + rng.below(255) as u8;
            b[p] ^= d;
            b[p + 32] ^= d;
            b
        } else {
            rng.bytes(len)
        };
        if a == b {
            continue;
        }
        let ca = crystallize(a, witnessed(), Oid::GENESIS);
        let cb = crystallize(b, witnessed(), Oid::GENESIS);
        assert_ne!(
            ca.oid, cb.oid,
            "round {round}: two distinct {len}-byte inputs share an address"
        );
    }
}

/// Same bytes, same address: the other half of the "iff".
#[test]
fn identical_inputs_get_identical_addresses() {
    let mut rng = Rng(0xD1B5_4A32_D192_ED03);
    for _ in 0..64 {
        let len = rng.below(200);
        let a = rng.bytes(len);
        let ca = crystallize(a.clone(), witnessed(), Oid::GENESIS);
        let cb = crystallize(a, witnessed(), Oid::GENESIS);
        assert_eq!(ca.oid, cb.oid);
    }
}

/// Field boundaries are part of the serialized bytes: moving a byte
/// from the author's name into the author's email is a different
/// witness, so a different address.
#[test]
fn witness_field_boundaries_are_addressed() {
    let w1 = Witnessed::new(
        Author::new("ab", "c"),
        Committer::new("mirror", "mirror@spectral.engineer"),
        Timestamp("1700000000".to_string()),
    );
    let w2 = Witnessed::new(
        Author::new("a", "bc"),
        Committer::new("mirror", "mirror@spectral.engineer"),
        Timestamp("1700000000".to_string()),
    );
    let c1 = crystallize(b"same".to_vec(), w1, Oid::GENESIS);
    let c2 = crystallize(b"same".to_vec(), w2, Oid::GENESIS);
    assert_ne!(c1.oid, c2.oid);
}
