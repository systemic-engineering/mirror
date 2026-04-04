//! Cooley-Tukey butterfly tiling — the recursive DFT approach.
//!
//! NOT full eigendecomposition. Recursive decomposition into self-similar stages.
//!
//! The DFT of size n decomposes into log(n) stages.
//! Each stage is a butterfly: a 2-point rotation by a twiddle factor.
//! The butterfly is the same at every stage. Crystallize ONE. Tile.
//!
//! For the elliptic curve DLP:
//! - The DFT over the cyclic group of order n recovers the discrete log
//! - The Cooley-Tukey decomposition makes this O(n log n) for known ring ordering
//! - The question: can we compute the twiddle factors WITHOUT knowing the ring ordering?
//!
//! Level 0: 8-bit crystal (282 points). Full DFT. Known.
//! Level 1: tile to ~1000 points. Butterfly = level 0 crystal. Adjust twiddles.
//! Level 2: tile to ~4000 points. Butterfly = level 1 crystal.
//! ...
//! The cost per level: O(n) where n is the LEVEL SIZE, not the GROUP SIZE.

mod curve {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum Point {
        Infinity,
        Affine { x: u64, y: u64 },
    }

    pub fn mod_pow(mut base: u128, mut exp: u128, m: u128) -> u128 {
        let mut result = 1u128;
        base %= m;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % m;
            }
            exp >>= 1;
            base = base * base % m;
        }
        result
    }

    pub fn mod_inv(a: u64, p: u64) -> Option<u64> {
        let (mut old_r, mut r) = (a as i128, p as i128);
        let (mut old_s, mut s) = (1i128, 0i128);
        while r != 0 {
            let q = old_r / r;
            let tmp = r;
            r = old_r - q * r;
            old_r = tmp;
            let tmp = s;
            s = old_s - q * s;
            old_s = tmp;
        }
        if old_r != 1 {
            return None;
        }
        Some(((old_s % p as i128 + p as i128) % p as i128) as u64)
    }

    pub fn point_add(p1: Point, p2: Point, a: u64, p: u64) -> Point {
        match (p1, p2) {
            (Point::Infinity, q) | (q, Point::Infinity) => q,
            (Point::Affine { x: x1, y: y1 }, Point::Affine { x: x2, y: y2 }) => {
                if x1 == x2 && y1 != y2 {
                    return Point::Infinity;
                }
                if x1 == x2 && y1 == y2 {
                    if y1 == 0 {
                        return Point::Infinity;
                    }
                    let num = (3 * x1 % p * x1 % p + a) % p;
                    let den = (2 * y1) % p;
                    let inv = match mod_inv(den, p) {
                        Some(i) => i,
                        None => return Point::Infinity,
                    };
                    let lam = num * inv % p;
                    let x3 = (lam * lam % p + p + p - x1 - x2) % p;
                    let y3 = (lam * ((x1 + p - x3) % p) % p + p - y1) % p;
                    Point::Affine { x: x3, y: y3 }
                } else {
                    let num = (y2 + p - y1) % p;
                    let den = (x2 + p - x1) % p;
                    let inv = match mod_inv(den, p) {
                        Some(i) => i,
                        None => return Point::Infinity,
                    };
                    let lam = num * inv % p;
                    let x3 = (lam * lam % p + p + p - x1 - x2) % p;
                    let y3 = (lam * ((x1 + p - x3) % p) % p + p - y1) % p;
                    Point::Affine { x: x3, y: y3 }
                }
            }
        }
    }

    pub fn scalar_mul(k: u64, point: Point, a: u64, p: u64) -> Point {
        if k == 0 {
            return Point::Infinity;
        }
        let mut result = Point::Infinity;
        let mut base = point;
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                result = point_add(result, base, a, p);
            }
            base = point_add(base, base, a, p);
            k >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::curve::*;
    use std::f64::consts::PI;

    /// DFT of a signal f at frequency k: F(k) = Σ_j f(j) · exp(-2πi·j·k/n)
    /// Returns (real, imaginary) parts.
    fn dft_coefficient(signal: &[f64], k: usize, n: usize) -> (f64, f64) {
        let mut re = 0.0f64;
        let mut im = 0.0f64;
        for j in 0..n {
            let angle = -2.0 * PI * (j as f64) * (k as f64) / (n as f64);
            re += signal[j] * angle.cos();
            im += signal[j] * angle.sin();
        }
        (re, im)
    }

    /// Inverse DFT: f(j) = (1/n) Σ_k F(k) · exp(2πi·j·k/n)
    fn idft(spectrum_re: &[f64], spectrum_im: &[f64], n: usize) -> Vec<f64> {
        let mut signal = vec![0.0f64; n];
        for j in 0..n {
            let mut val = 0.0f64;
            for k in 0..n {
                let angle = 2.0 * PI * (j as f64) * (k as f64) / (n as f64);
                val += spectrum_re[k] * angle.cos() - spectrum_im[k] * angle.sin();
            }
            signal[j] = val / n as f64;
        }
        signal
    }

    #[test]
    fn dft_recovers_private_key_from_indicator() {
        // The DFT approach to DLP on a cyclic group:
        //
        // 1. The ring ordering maps private key k → vertex index ring[k]
        // 2. Create indicator signal: f[ring[k]] = 1, f[elsewhere] = 0
        //    (But this requires knowing the ring ordering = knowing k. Circular.)
        //
        // The NON-circular approach:
        // 1. We know the PUBLIC KEY's vertex index (it's the x,y coordinates)
        // 2. We know the GENERATOR's vertex index
        // 3. The DFT of the indicator at the public key, evaluated in the
        //    ring-ordered basis, gives the private key
        //
        // But "ring-ordered basis" = knowing the ordering. Still circular for
        // the full group.
        //
        // Cooley-Tukey insight: the DFT doesn't need to be computed in one shot.
        // It decomposes into stages. Each stage operates on a SMALLER subproblem.
        // The twiddle factors between stages encode the scaling.
        //
        // For a group of size n = n1 × n2 (factored):
        //   DFT_n = (DFT_n1 ⊗ I_n2) · T · (I_n1 ⊗ DFT_n2)
        // where T is the diagonal twiddle factor matrix.
        //
        // If n = 2^m: fully recursive, m stages, each is a butterfly.
        // If n is prime: Rader's algorithm converts to cyclic convolution.
        //
        // For elliptic curve groups: n is typically prime (by design).
        // Rader: DFT of prime n = DFT of (n-1) + boundary terms.
        // n-1 is even → Cooley-Tukey applies to (n-1).

        let p = 251u64;
        let points = super::curve::enumerate_curve(1, 1, p);
        let n = points.len(); // 282
        let gen = points[1];

        // Compute the ring ordering (THIS IS THE DLP — we're using it as ground truth)
        let point_to_idx: std::collections::HashMap<Point, usize> =
            points.iter().enumerate().map(|(i, &p)| (p, i)).collect();

        let mut ring_order = vec![0usize; n]; // ring_order[k] = vertex index of kG
        let mut pt = Point::Infinity;
        for k in 0..n {
            ring_order[k] = point_to_idx[&pt];
            pt = point_add(pt, gen, 1, p);
        }

        // For a target private key k_target, the public key is at ring_order[k_target]
        let k_target = 42u64;
        let public_idx = ring_order[k_target as usize];
        eprintln!(
            "  target k={}, public vertex index={}",
            k_target, public_idx
        );

        // Create the indicator signal in VERTEX ordering
        let mut signal = vec![0.0f64; n];
        signal[public_idx] = 1.0;

        // DFT in vertex ordering — this doesn't directly give k
        let (f1_re, f1_im) = dft_coefficient(&signal, 1, n);
        let vertex_phase = f1_im.atan2(f1_re);
        eprintln!("  DFT in vertex order: phase={:.4}", vertex_phase);

        // DFT in RING ordering — this gives k directly
        let mut signal_ring = vec![0.0f64; n];
        // In ring ordering, the public key is at position k_target
        signal_ring[k_target as usize] = 1.0;
        let (r1_re, r1_im) = dft_coefficient(&signal_ring, 1, n);
        let ring_phase = r1_im.atan2(r1_re);
        let recovered = ((-ring_phase * n as f64 / (2.0 * PI)) % n as f64 + n as f64) % n as f64;
        eprintln!(
            "  DFT in ring order: phase={:.4}, recovered={:.1}",
            ring_phase, recovered
        );
        assert!(
            (recovered.round() as u64 % n as u64) == k_target,
            "ring-ordered DFT should recover k exactly"
        );

        // The gap: converting from vertex-ordered DFT to ring-ordered DFT
        // requires the PERMUTATION that maps vertex indices to ring positions.
        // That permutation IS the discrete log for all points.
        //
        // Cooley-Tukey doesn't help here because it decomposes the DFT
        // into stages that operate on SUB-INDICES of the ring ordering.
        // Without the ring ordering, you can't index into the stages.
        //
        // The butterfly operates on pairs (ring[k], ring[k + n/2]).
        // To apply it, you need ring[k]. That's the DLP for k.

        eprintln!("\n  === STRUCTURAL INSIGHT ===");
        eprintln!("  The DFT recovers k from the ring-ordered indicator.");
        eprintln!("  But ring-ordering = discrete log for all group elements.");
        eprintln!("  Cooley-Tukey decomposes the DFT but needs ring-ordered access.");
        eprintln!("  The butterfly operates on ring[k] and ring[k+n/2].");
        eprintln!("  Computing ring[k] IS the DLP for k.");
        eprintln!("  The recursion doesn't help because each level needs the DLP.");

        // HOWEVER: what if we don't need the EXACT ring ordering?
        // What if an APPROXIMATE ordering — with bounded error — suffices?
        //
        // The FFT with approximate twiddle factors produces approximate DFT coefficients.
        // If the approximation error is small enough, the phase recovery still works.
        //
        // Can we compute an approximate ring ordering in less than O(n) time?
        //
        // Baby-step-giant-step computes the exact ordering in O(√n) time.
        // That's already known. It doesn't use spectral structure.
        //
        // The question: can spectral structure give an approximate ordering
        // in O(log n) time? Because THAT would make the butterfly O(n log n)
        // where n = log(group_size) = key_bits.

        // Test: how much error can the DFT tolerate in the ring ordering?
        // Permute the ring ordering by increasing amounts and measure recovery.
        let mut results = Vec::new();
        for error_radius in [0, 1, 2, 5, 10, 20, 50, 100] {
            let mut signal_approx = vec![0.0f64; n];
            // Place the indicator at position k_target ± error
            let approx_pos = ((k_target as i64 + error_radius as i64) % n as i64) as usize;
            signal_approx[approx_pos] = 1.0;
            let (r_re, r_im) = dft_coefficient(&signal_approx, 1, n);
            let phase = r_im.atan2(r_re);
            let rec = ((-phase * n as f64 / (2.0 * PI)) % n as f64 + n as f64) % n as f64;
            let rec_k = rec.round() as u64 % n as u64;
            let error_from_true = ((rec_k as i64 - k_target as i64).abs())
                .min((n as i64 - (rec_k as i64 - k_target as i64).abs()));
            results.push((error_radius, rec_k, error_from_true));
            eprintln!(
                "  error_radius={:3}: recovered={:3}, off by {}",
                error_radius, rec_k, error_from_true
            );
        }

        eprintln!("\n  The DFT is EXACT — any error in ring position maps to");
        eprintln!("  the same error in the recovered key. No error correction.");
        eprintln!("  An approximate ring ordering gives an equally approximate key.");
        eprintln!("  The butterfly tiling doesn't help because each level needs exact indices.");
    }
}
