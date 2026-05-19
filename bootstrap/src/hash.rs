//! SHA-256 helpers + CoincidenceHash<5,5>.
//!
//! This is the Cluster D rewrite. The bootstrap now implements the geometry
//! declared in `boot/std/hash/coincidence.mirror`:
//!
//!   DIM = 5             — one dimension per Prism operation:
//!                         [0]=focus, [1]=project, [2]=split, [3]=zoom, [4]=refract
//!   NUM_PROJECTIONS = 5 — one projection per gutter-lens duality:
//!                         [0]=entropy, [1]=spectral, [2]=cheeger,
//!                         [3]=ricci,  [4]=mixing
//!   LEX_ORDER = [0,1,2,3,4]
//!                       — lex-of-decimal-strings ordering collapses to identity
//!                         when there are 5 elements (only single-digit indices).
//!   EPSILON   unchanged — IEEE-754 double machine epsilon.
//!
//! The seed format `coincidence:projection:{i}:{NUM_PROJECTIONS}` is unchanged.
//! The index `i` implicitly references the duality at the same position in the
//! grammar's `duality = entropy | spectral | cheeger | ricci | mixing` axis.
//!
//! Tags (`prism-core:dark:`, `prism-core:coincidence:`, `coincidence:`) are
//! unchanged. Verification: @epistemologic/property/coincidence_matches.

use sha2::{Digest, Sha256};

pub const DIM: usize = 5;
pub const NUM_PROJECTIONS: usize = 5;
pub const EPSILON: f64 = 2.2204460492503131e-16;

/// LEX_ORDER: the canonical traversal order. For 5 elements the
/// lex-of-decimal-strings ordering collapses to identity. This order is what
/// `for &j in LEX_ORDER` walks — the canonical traversal of vectors and
/// matrices so different impls agree byte-for-byte.
pub const LEX_ORDER: [usize; DIM] = [0, 1, 2, 3, 4];

/// Pack 8 little-endian bytes of `v` into out.
fn u64_le(v: u64, out: &mut [u8; 8]) {
    for k in 0..8 {
        out[k] = ((v >> (k * 8)) & 0xff) as u8;
    }
}

#[derive(Clone)]
pub struct Projection {
    pub entries: [[f64; DIM]; DIM],
    pub present: [[bool; DIM]; DIM],
}

impl Projection {
    pub fn zero() -> Self {
        Self {
            entries: [[0.0; DIM]; DIM],
            present: [[false; DIM]; DIM],
        }
    }
}

/// Build a Projection from the canonical seed string. Structurally identical
/// to the C `projection_from_seed`; only the dimensions shrink.
pub fn projection_from_seed(seed: &str) -> Projection {
    let seed_bytes = seed.as_bytes();
    let mut components = [0.0_f64; DIM];

    for j in 0..DIM {
        let mut hasher = Sha256::new();
        hasher.update(seed_bytes);
        hasher.update(b":");
        let mut j_bytes = [0u8; 8];
        u64_le(j as u64, &mut j_bytes);
        hasher.update(&j_bytes);
        let hash = hasher.finalize();
        let mut raw: u64 = 0;
        for k in 0..8 {
            raw |= (hash[k] as u64) << (k * 8);
        }
        // C: ((double)raw / (double)18446744073709551615ULL) * 2.0 - 1.0
        let val = (raw as f64 / u64::MAX as f64) * 2.0 - 1.0;
        components[j] = val;
    }

    // Norm in LEX_ORDER iteration order.
    let mut norm = 0.0_f64;
    for &j in LEX_ORDER.iter() {
        norm += components[j] * components[j];
    }
    norm = norm.sqrt();
    if norm < EPSILON {
        norm = 1.0;
    }
    for j in 0..DIM {
        components[j] /= norm;
    }

    let mut proj = Projection::zero();
    for i in 0..DIM {
        if components[i].abs() < EPSILON {
            // Row is zero — already zeroed.
            continue;
        }
        for j in 0..DIM {
            let val = components[i] * components[j];
            if val.abs() > EPSILON {
                proj.entries[i][j] = val;
                proj.present[i][j] = true;
            }
        }
    }
    proj
}

/// Apply projection to v, write result into out. Iteration order in LEX_ORDER
/// for both row and column so different impls agree byte-for-byte.
pub fn projection_apply(proj: &Projection, v: &[f64; DIM], out: &mut [f64; DIM]) {
    for i in 0..DIM {
        out[i] = 0.0;
    }
    for ri in 0..DIM {
        let row = LEX_ORDER[ri];
        for ci in 0..DIM {
            let col = LEX_ORDER[ci];
            if !proj.present[row][col] {
                continue;
            }
            let coeff = v[col];
            if coeff.abs() > EPSILON {
                out[row] += proj.entries[row][col] * coeff;
            }
        }
    }
    for i in 0..DIM {
        if out[i].abs() <= EPSILON {
            out[i] = 0.0;
        }
    }
}

pub fn vec_is_zero(v: &[f64; DIM]) -> bool {
    for i in 0..DIM {
        if v[i].abs() > EPSILON {
            return false;
        }
    }
    true
}

/// Encode data bytes into the DIM-dimensional coefficient vector.
pub fn encode_into_basis(data: &[u8]) -> [f64; DIM] {
    let mut coeffs = [0.0_f64; DIM];
    if data.is_empty() {
        return coeffs;
    }
    for (i, &byte) in data.iter().enumerate() {
        let mut seed_hasher = Sha256::new();
        seed_hasher.update(b"encode:");
        let mut i_bytes = [0u8; 8];
        u64_le(i as u64, &mut i_bytes);
        seed_hasher.update(&i_bytes);
        seed_hasher.update(&[byte]);
        let seed = seed_hasher.finalize();
        // seed is 32 bytes
        for j in 0..DIM {
            let mut dim_hasher = Sha256::new();
            dim_hasher.update(&seed);
            dim_hasher.update(b":");
            let mut j_bytes = [0u8; 8];
            u64_le(j as u64, &mut j_bytes);
            dim_hasher.update(&j_bytes);
            let dim_hash = dim_hasher.finalize();
            let mut raw: u64 = 0;
            for k in 0..8 {
                raw |= (dim_hash[k] as u64) << (k * 8);
            }
            let val = (raw as f64 / u64::MAX as f64) * 2.0 - 1.0;
            coeffs[j] += val;
        }
    }
    for j in 0..DIM {
        if coeffs[j].abs() <= EPSILON {
            coeffs[j] = 0.0;
        }
    }
    coeffs
}

/// Lazy initialisation of canonical projections.
fn canonical_projections() -> &'static [Projection; NUM_PROJECTIONS] {
    use std::sync::OnceLock;
    static PROJ: OnceLock<[Projection; NUM_PROJECTIONS]> = OnceLock::new();
    PROJ.get_or_init(|| {
        let mut arr = [
            Projection::zero(),
            Projection::zero(),
            Projection::zero(),
            Projection::zero(),
            Projection::zero(),
        ];
        for i in 0..NUM_PROJECTIONS {
            let seed = format!("coincidence:projection:{}:{}", i, NUM_PROJECTIONS);
            arr[i] = projection_from_seed(&seed);
        }
        arr
    })
}

/// CoincidenceHash<5,5>. Returns a 64-char hex string.
pub fn canonical_hash(data: &[u8]) -> String {
    let projs = canonical_projections();
    let coeffs = encode_into_basis(data);

    if vec_is_zero(&coeffs) {
        let mut h = Sha256::new();
        h.update(b"prism-core:dark:");
        h.update(data);
        return hex_str(&h.finalize());
    }

    let mut focus_results = [[0.0_f64; DIM]; NUM_PROJECTIONS];
    let mut any_zero = false;
    for p in 0..NUM_PROJECTIONS {
        projection_apply(&projs[p], &coeffs, &mut focus_results[p]);
        if vec_is_zero(&focus_results[p]) {
            any_zero = true;
            break;
        }
    }

    if any_zero {
        let mut h = Sha256::new();
        h.update(b"prism-core:dark:");
        h.update(data);
        return hex_str(&h.finalize());
    }

    // Build eigenvalue_bytes:
    //   "coincidence:" (12 bytes)
    //   NUM_PROJECTIONS as little-endian u64 (8 bytes)
    //   for each p,j: f64 bits as little-endian u64 (8 bytes)
    let total = 12 + 8 + NUM_PROJECTIONS * DIM * 8;
    let mut buf: Vec<u8> = Vec::with_capacity(total);
    buf.extend_from_slice(b"coincidence:");
    let mut n_bytes = [0u8; 8];
    u64_le(NUM_PROJECTIONS as u64, &mut n_bytes);
    buf.extend_from_slice(&n_bytes);
    for p in 0..NUM_PROJECTIONS {
        for j in 0..DIM {
            let bits = focus_results[p][j].to_bits();
            let mut bbytes = [0u8; 8];
            u64_le(bits, &mut bbytes);
            buf.extend_from_slice(&bbytes);
        }
    }

    let mut h = Sha256::new();
    h.update(b"prism-core:coincidence:");
    h.update(&buf);
    hex_str(&h.finalize())
}

fn hex_str(hash: &[u8]) -> String {
    let mut s = String::with_capacity(64);
    for b in hash.iter() {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// `<tag>:<content>` -> canonical_hash.
pub fn hash_tagged(tag: &str, content: &[u8]) -> String {
    let mut buf: Vec<u8> = Vec::with_capacity(tag.len() + 1 + content.len());
    buf.extend_from_slice(tag.as_bytes());
    buf.push(b':');
    buf.extend_from_slice(content);
    canonical_hash(&buf)
}
