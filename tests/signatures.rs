//! ECDSA Signature Spectral Attack — circular-reflexive loop over signature views.
//!
//! Previous experiments showed that five spectral views of elliptic curve structure
//! (fold, lens, traverse, iso, Hodge) are orthogonal — pairwise spectral distances ≈ 1.0.
//! No interference pattern can concentrate information. The terrain is flat.
//!
//! This experiment tests a different hypothesis: SIGNATURES provide non-flat terrain.
//! Each ECDSA signature (r, s) for message hash h satisfies:
//!     s = k⁻¹(h + r·d) mod n
//! where k is the nonce, d is the private key. Each signature is a LINEAR equation
//! in d, tangled with an unknown nonce k. Multiple signatures give multiple independent
//! noisy views of d.
//!
//! The question: does the circular-reflexive loop over signatures + structural views
//! converge on d? And if so, at what cost (in group operations)?

mod curve {
    use std::sync::atomic::{AtomicU64, Ordering};

    /// Global counter for group operations (point_add + scalar_mul calls).
    pub static GROUP_OPS: AtomicU64 = AtomicU64::new(0);

    pub fn reset_ops() {
        GROUP_OPS.store(0, Ordering::SeqCst);
    }

    pub fn get_ops() -> u64 {
        GROUP_OPS.load(Ordering::SeqCst)
    }

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
        GROUP_OPS.fetch_add(1, Ordering::Relaxed);
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

    pub fn enumerate_curve(a: u64, b: u64, p: u64) -> Vec<Point> {
        let mut points = vec![Point::Infinity];
        for x in 0..p {
            let rhs = (x * x % p * x % p + a * x % p + b) % p;
            let y = mod_pow(rhs as u128, ((p + 1) / 4) as u128, p as u128) as u64;
            if y * y % p == rhs {
                points.push(Point::Affine { x, y });
                if y != 0 && y != p - y {
                    points.push(Point::Affine { x, y: p - y });
                }
            }
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use super::curve::*;
    use std::f64::consts::PI;

    /// ECDSA signature tuple: (r, s, h) where h is the message hash.
    #[derive(Debug, Clone)]
    struct Signature {
        r: u64,
        s: u64,
        h: u64,
    }

    /// DFT of a signal f at frequency k: F(k) = Σ_j f(j) · exp(-2πi·j·k/n)
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

    /// Full power spectrum of a signal: |F(k)|² for each k.
    fn power_spectrum(signal: &[f64]) -> Vec<f64> {
        let n = signal.len();
        (0..n)
            .map(|k| {
                let (re, im) = dft_coefficient(signal, k, n);
                re * re + im * im
            })
            .collect()
    }

    /// Spectral distance between two power spectra (normalized L2 distance).
    fn spectral_distance(spec_a: &[f64], spec_b: &[f64]) -> f64 {
        assert_eq!(spec_a.len(), spec_b.len());
        let norm_a: f64 = spec_a.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm_b: f64 = spec_b.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm_a < 1e-12 || norm_b < 1e-12 {
            return 1.0;
        }
        let dot: f64 = spec_a
            .iter()
            .zip(spec_b.iter())
            .map(|(a, b)| a * b)
            .sum::<f64>();
        let cosine = dot / (norm_a * norm_b);
        // Convert cosine similarity to distance in [0, 1]
        (1.0 - cosine.clamp(-1.0, 1.0)) / 2.0
    }

    /// Cross-correlation of two signals: (f ⋆ g)[k] = Σ_j f(j) · g(j+k mod n)
    fn cross_correlate(f: &[f64], g: &[f64]) -> Vec<f64> {
        let n = f.len();
        assert_eq!(n, g.len());
        let mut result = vec![0.0f64; n];
        for k in 0..n {
            let mut sum = 0.0;
            for j in 0..n {
                sum += f[j] * g[(j + k) % n];
            }
            result[k] = sum;
        }
        result
    }

    /// Find the generator of maximal order in the group.
    fn find_generator(points: &[Point], a: u64, p: u64) -> (Point, u64) {
        let n = points.len() as u64;
        for &pt in &points[1..] {
            let mut current = pt;
            let mut order = 1u64;
            while current != Point::Infinity {
                current = point_add(current, pt, a, p);
                order += 1;
                if order > n {
                    break;
                }
            }
            if order == n {
                return (pt, n);
            }
        }
        // Fall back to largest order found
        let mut best = (points[1], 1u64);
        for &pt in &points[1..] {
            let mut current = pt;
            let mut order = 1u64;
            while current != Point::Infinity {
                current = point_add(current, pt, a, p);
                order += 1;
                if order > n {
                    break;
                }
            }
            if order > best.1 {
                best = (pt, order);
            }
        }
        best
    }

    #[test]
    fn ecdsa_signature_spectral_attack() {
        eprintln!("\n╔══════════════════════════════════════════════════════════════╗");
        eprintln!("║  ECDSA Signature Spectral Attack — Circular Reflexive Loop  ║");
        eprintln!("╚══════════════════════════════════════════════════════════════╝\n");

        // ─── Step 1: Curve Setup (8-bit) ───────────────────────────────
        // Using y² = x³ + x + 4 (mod 251) which has PRIME group order 271.
        // Prime order is essential for ECDSA: every non-identity element is a
        // generator, and all modular inverses exist.
        let field_p = 251u64;
        let curve_a = 1u64;
        let curve_b = 4u64;

        reset_ops();
        let points = enumerate_curve(curve_a, curve_b, field_p);
        let num_points = points.len();
        eprintln!("Step 1: Curve y² = x³ + x + 4 (mod 251)");
        eprintln!("  Total points (including infinity): {}", num_points);

        // Find generator of maximal order
        let (gen, group_order) = find_generator(&points, curve_a, field_p);
        eprintln!("  Generator: {:?}, order: {}", gen, group_order);

        let n = group_order; // group order for modular arithmetic

        // Build ring ordering: ring[k] = k·G
        let mut ring: Vec<Point> = Vec::with_capacity(n as usize);
        let mut pt = Point::Infinity;
        for _ in 0..n {
            ring.push(pt);
            pt = point_add(pt, gen, curve_a, field_p);
        }
        eprintln!("  Ring ordering computed ({} elements)", ring.len());

        // Private key
        let d = 42u64;
        let pub_key = scalar_mul(d, gen, curve_a, field_p);
        eprintln!("  Private key d = {}", d);
        eprintln!("  Public key Q = {:?}", pub_key);

        // Verify public key is at ring[d]
        assert_eq!(ring[d as usize], pub_key, "Q should equal d·G");

        let setup_ops = get_ops();
        eprintln!("  Setup cost: {} group ops", setup_ops);

        // ─── Step 2: Generate ECDSA Signatures ─────────────────────────
        eprintln!("\nStep 2: Generating 20 ECDSA signatures");
        reset_ops();

        let num_sigs = 20usize;
        let mut signatures: Vec<Signature> = Vec::new();
        let mut nonces: Vec<u64> = Vec::new(); // Keep for verification (not used in attack)

        for i in 1..=200u64 {
            if signatures.len() >= num_sigs {
                break;
            }

            // Deterministic nonce: k_i = (7*i + 13) mod (n-1) + 1
            // With prime n, all k_i in [1, n-1] are invertible.
            let k_i = ((7 * i + 13) % (n - 1)) + 1;

            // R_i = k_i · G
            let r_point = scalar_mul(k_i, gen, curve_a, field_p);
            let r_i = match r_point {
                Point::Affine { x, .. } => x % n,
                Point::Infinity => continue,
            };
            if r_i == 0 {
                continue;
            }

            // h_i = simple hash of message i
            let h_i = ((i * 31 + 17) % n) as u64;
            if h_i == 0 {
                continue;
            }

            // s_i = k_i⁻¹ · (h_i + r_i · d) mod n
            let k_inv = match mod_inv(k_i, n) {
                Some(inv) => inv,
                None => continue,
            };
            let s_i =
                (k_inv as u128 * ((h_i as u128 + r_i as u128 * d as u128) % n as u128)) % n as u128;
            let s_i = s_i as u64;
            if s_i == 0 {
                continue;
            }

            // Verify: s_i · k_i ≡ h_i + r_i · d (mod n)
            let lhs = (s_i as u128 * k_i as u128) % n as u128;
            let rhs = (h_i as u128 + r_i as u128 * d as u128) % n as u128;
            assert_eq!(lhs, rhs, "Signature {} verification failed", i);

            eprintln!(
                "  sig[{:2}]: r={:3}, s={:3}, h={:3}, k={:3} ✓",
                signatures.len(),
                r_i,
                s_i,
                h_i,
                k_i
            );

            signatures.push(Signature {
                r: r_i,
                s: s_i,
                h: h_i,
            });
            nonces.push(k_i);
        }

        let sig_gen_ops = get_ops();
        eprintln!(
            "  Generated {} signatures, cost: {} group ops",
            signatures.len(),
            sig_gen_ops
        );
        assert!(
            signatures.len() >= num_sigs,
            "Need at least {} signatures, got {}",
            num_sigs,
            signatures.len()
        );

        // ─── Step 3: Signature Views ───────────────────────────────────
        eprintln!("\n═══ Step 3: Signature Views ═══\n");

        // View A: Signature consistency score (brute force — used as ground truth)
        eprintln!("View A: Signature consistency score (brute force)");
        reset_ops();

        let mut consistency_score = vec![0.0f64; n as usize];
        for d_c in 0..n {
            let mut count = 0u64;
            for sig in &signatures {
                // k_candidate = s⁻¹ · (h + r · d_c) mod n
                let s_inv = match mod_inv(sig.s, n) {
                    Some(inv) => inv,
                    None => continue,
                };
                let k_cand = (s_inv as u128
                    * ((sig.h as u128 + sig.r as u128 * d_c as u128) % n as u128))
                    % n as u128;
                let k_cand = k_cand as u64;
                if k_cand == 0 {
                    continue;
                }

                // R_candidate = k_candidate · G
                let r_point = scalar_mul(k_cand, gen, curve_a, field_p);
                match r_point {
                    Point::Affine { x, .. } => {
                        if x % n == sig.r {
                            count += 1;
                        }
                    }
                    Point::Infinity => {}
                }
            }
            consistency_score[d_c as usize] = count as f64;
        }

        let view_a_ops = get_ops();
        let view_a_max = consistency_score.iter().cloned().fold(0.0f64, f64::max);
        let view_a_argmax = consistency_score
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        eprintln!(
            "  Max consistency: {:.0}/20 at d_c = {}",
            view_a_max, view_a_argmax
        );
        eprintln!(
            "  Correct d = {} → score = {:.0}",
            d, consistency_score[d as usize]
        );
        eprintln!("  Cost: {} group ops (brute force baseline)", view_a_ops);

        // Count how many candidates score > 0
        let nonzero_count = consistency_score.iter().filter(|&&x| x > 0.5).count();
        eprintln!("  Candidates with score > 0: {}", nonzero_count);

        // View B: Partial signature correlation (t_i histogram)
        eprintln!("\nView B: Partial signature correlation (t_i = r_i⁻¹ · s_i mod n)");
        let mut t_values: Vec<u64> = Vec::new();
        for sig in &signatures {
            let r_inv = mod_inv(sig.r, n).expect("r must be invertible mod n (n is prime)");
            let t_i = (r_inv as u128 * sig.s as u128) % n as u128;
            t_values.push(t_i as u64);
        }

        // Histogram: for each position j, count how many signatures have t_i ≡ j
        let mut t_histogram = vec![0.0f64; n as usize];
        for &t in &t_values {
            t_histogram[t as usize] += 1.0;
        }

        let t_nonzero = t_histogram.iter().filter(|&&x| x > 0.5).count();
        let t_max = t_histogram.iter().cloned().fold(0.0f64, f64::max);
        eprintln!(
            "  t_i histogram: {} nonzero bins, max count = {:.0}",
            t_nonzero, t_max
        );

        // View B power spectrum
        let view_b_spectrum = power_spectrum(&t_histogram);
        let view_b_dc = view_b_spectrum[0];
        let view_b_max_nondc = view_b_spectrum[1..].iter().cloned().fold(0.0f64, f64::max);
        eprintln!(
            "  View B spectrum: DC={:.1}, max non-DC={:.1}",
            view_b_dc, view_b_max_nondc
        );

        // View C: r-value spectral fingerprint
        eprintln!("\nView C: r-value spectral fingerprint");
        let mut r_signal = vec![0.0f64; n as usize];
        for sig in &signatures {
            if (sig.r as usize) < r_signal.len() {
                r_signal[sig.r as usize] += 1.0;
            }
        }

        let view_c_spectrum = power_spectrum(&r_signal);
        let view_c_dc = view_c_spectrum[0];
        let view_c_max_nondc = view_c_spectrum[1..].iter().cloned().fold(0.0f64, f64::max);
        eprintln!(
            "  View C spectrum: DC={:.1}, max non-DC={:.1}",
            view_c_dc, view_c_max_nondc
        );
        eprintln!(
            "  r-values occupy {} distinct positions out of {}",
            r_signal.iter().filter(|&&x| x > 0.5).count(),
            n
        );

        // View D: Pairwise consistency signal
        eprintln!("\nView D: Pairwise consistency signal");
        reset_ops();

        let mut pairwise_score = vec![0.0f64; n as usize];
        let total_pairs = signatures.len() * (signatures.len() - 1) / 2;

        for d_c in 0..n {
            let mut pair_count = 0u64;
            // For each signature, compute k_candidate from d_c
            let mut k_candidates: Vec<u64> = Vec::new();
            let mut k_valid: Vec<bool> = Vec::new();

            for sig in &signatures {
                let s_inv = mod_inv(sig.s, n).unwrap();
                let k_cand = (s_inv as u128
                    * ((sig.h as u128 + sig.r as u128 * d_c as u128) % n as u128))
                    % n as u128;
                let k_cand = k_cand as u64;

                // Check if k_cand produces the right r
                let valid = if k_cand == 0 {
                    false
                } else {
                    let r_pt = scalar_mul(k_cand, gen, curve_a, field_p);
                    match r_pt {
                        Point::Affine { x, .. } => x % n == sig.r,
                        Point::Infinity => false,
                    }
                };

                k_candidates.push(k_cand);
                k_valid.push(valid);
            }

            // Count consistent pairs
            for i in 0..signatures.len() {
                for j in (i + 1)..signatures.len() {
                    if k_valid[i] && k_valid[j] {
                        pair_count += 1;
                    }
                }
            }
            pairwise_score[d_c as usize] = pair_count as f64;
        }

        let view_d_ops = get_ops();
        let view_d_max = pairwise_score.iter().cloned().fold(0.0f64, f64::max);
        let view_d_argmax = pairwise_score
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        eprintln!(
            "  Max pairwise consistency: {:.0}/{} at d_c = {}",
            view_d_max, total_pairs, view_d_argmax
        );
        eprintln!(
            "  Correct d = {} → pairs = {:.0}",
            d, pairwise_score[d as usize]
        );
        eprintln!("  Cost: {} group ops", view_d_ops);

        // ─── Step 4: The Circular Reflexive Loop ───────────────────────
        eprintln!("\n═══ Step 4: Circular Reflexive Loop ═══\n");
        reset_ops();

        // Lens view: f(k) = x(kG) for k = 0..n-1
        let mut lens_signal = vec![0.0f64; n as usize];
        for k in 0..n {
            lens_signal[k as usize] = match ring[k as usize] {
                Point::Affine { x, .. } => x as f64,
                Point::Infinity => 0.0,
            };
        }
        let lens_spectrum = power_spectrum(&lens_signal);

        // Start with View C (r-value spectrum — purely public)
        let mut current_signal = r_signal.clone();
        let mut prev_argmax = 0usize;

        eprintln!("  Starting from View C (r-value spectrum, public only)");
        eprintln!("  Iterating circular loop: cross-correlate → weight → sharpen → repeat\n");

        for iteration in 0..20 {
            // 1. Cross-correlate current signal with lens view
            let xcorr = cross_correlate(&current_signal, &lens_signal);

            // 2. Element-wise multiply with t_i histogram (View B)
            let mut weighted: Vec<f64> = xcorr
                .iter()
                .zip(t_histogram.iter())
                .map(|(a, b)| a * b)
                .collect();

            // 3. Peak sharpening: raise to power 2, renormalize
            let max_val = weighted.iter().cloned().fold(0.0f64, f64::max);
            if max_val > 1e-12 {
                for v in weighted.iter_mut() {
                    *v = (*v / max_val).max(0.0);
                    *v = *v * *v; // square
                }
                let sum: f64 = weighted.iter().sum();
                if sum > 1e-12 {
                    for v in weighted.iter_mut() {
                        *v /= sum;
                    }
                }
            }

            // 4. Check argmax
            let argmax = weighted
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0;

            let max_weight = weighted.iter().cloned().fold(0.0f64, f64::max);
            let second_max = weighted
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != argmax)
                .map(|(_, v)| *v)
                .fold(0.0f64, f64::max);

            let converged = argmax == d as usize;
            let stable = argmax == prev_argmax && iteration > 0;

            eprintln!(
                "  iter {:2}: argmax={:3} (d={}), max={:.6}, ratio={:.2}, {}{}",
                iteration,
                argmax,
                d,
                max_weight,
                if second_max > 1e-12 {
                    max_weight / second_max
                } else {
                    f64::INFINITY
                },
                if converged { "CONVERGED " } else { "" },
                if stable { "STABLE" } else { "" },
            );

            prev_argmax = argmax;

            // 5. Feed back: weight lens view by current estimate
            //    current_signal[j] = weighted[j] acts as "prior"
            current_signal = weighted;
        }

        let loop_ops = get_ops();
        eprintln!("\n  Circular loop cost: {} group ops", loop_ops);
        eprintln!("  Final argmax: {} (correct d = {})", prev_argmax, d);
        if prev_argmax == d as usize {
            eprintln!("  >>> CIRCULAR LOOP CONVERGED TO d <<<");
        } else {
            eprintln!("  Circular loop did NOT converge to d.");
        }

        // ─── Step 5: Direct Signature Attack (Baseline) ────────────────
        eprintln!("\n═══ Step 5: Direct Signature Attack (Baseline) ═══\n");
        reset_ops();

        let mut brute_d = None;
        for d_c in 0..n {
            let mut all_valid = true;
            for sig in &signatures {
                let s_inv = mod_inv(sig.s, n).unwrap();
                let k_cand = (s_inv as u128
                    * ((sig.h as u128 + sig.r as u128 * d_c as u128) % n as u128))
                    % n as u128;
                let k_cand = k_cand as u64;
                if k_cand == 0 {
                    all_valid = false;
                    break;
                }
                let r_pt = scalar_mul(k_cand, gen, curve_a, field_p);
                match r_pt {
                    Point::Affine { x, .. } => {
                        if x % n != sig.r {
                            all_valid = false;
                            break;
                        }
                    }
                    Point::Infinity => {
                        all_valid = false;
                        break;
                    }
                }
            }
            if all_valid {
                brute_d = Some(d_c);
                break; // Found it, stop early
            }
        }

        let brute_ops = get_ops();
        eprintln!("  Brute force result: d = {:?}", brute_d);
        eprintln!("  Cost: {} group ops", brute_ops);
        assert_eq!(brute_d, Some(d), "Brute force should find correct d");

        // ─── Step 6: Lattice-Adjacent Approach ─────────────────────────
        eprintln!("\n═══ Step 6: Lattice-Adjacent Approach ═══\n");

        // For each pair (i, j), compute:
        //   a_ij = (s_i · r_j - s_j · r_i) mod n
        //   b_ij = (s_i · h_j - s_j · h_i) mod n
        let mut lattice_a_values: Vec<u64> = Vec::new();
        let mut lattice_b_values: Vec<u64> = Vec::new();

        for i in 0..signatures.len() {
            for j in (i + 1)..signatures.len() {
                let si = &signatures[i];
                let sj = &signatures[j];

                let a_ij = ((si.s as u128 * sj.r as u128) % n as u128 + n as u128
                    - (sj.s as u128 * si.r as u128) % n as u128)
                    % n as u128;
                let b_ij = ((si.s as u128 * sj.h as u128) % n as u128 + n as u128
                    - (sj.s as u128 * si.h as u128) % n as u128)
                    % n as u128;

                lattice_a_values.push(a_ij as u64);
                lattice_b_values.push(b_ij as u64);
            }
        }

        // Create signals from lattice values
        let mut lattice_a_signal = vec![0.0f64; n as usize];
        let mut lattice_b_signal = vec![0.0f64; n as usize];
        for &a in &lattice_a_values {
            if (a as usize) < lattice_a_signal.len() {
                lattice_a_signal[a as usize] += 1.0;
            }
        }
        for &b in &lattice_b_values {
            if (b as usize) < lattice_b_signal.len() {
                lattice_b_signal[b as usize] += 1.0;
            }
        }

        // Combined lattice signal (sum of a and b histograms)
        let lattice_signal: Vec<f64> = lattice_a_signal
            .iter()
            .zip(lattice_b_signal.iter())
            .map(|(a, b)| a + b)
            .collect();

        let lattice_spectrum = power_spectrum(&lattice_signal);
        let lattice_dc = lattice_spectrum[0];
        let lattice_max_nondc = lattice_spectrum[1..].iter().cloned().fold(0.0f64, f64::max);
        eprintln!(
            "  Lattice signal: {} pairs, spectrum DC={:.1}, max non-DC={:.1}",
            lattice_a_values.len(),
            lattice_dc,
            lattice_max_nondc
        );

        // ─── Step 7: Meta-Graph with Signatures ───────────────────────
        eprintln!("\n═══ Step 7: Meta-Graph — 6×6 Spectral Distance Matrix ═══\n");

        // Compute all 6 spectra at common length = n
        // View 1: Lens (x-projection power spectrum)
        let spec_lens = lens_spectrum.clone();

        // View 2: Iso (cross-correlation structure)
        // We compute the iso view as the cross-correlation of the lens with itself
        // shifted — this captures the group automorphism structure.
        let xcorr_iso = cross_correlate(&lens_signal, &lens_signal);
        let spec_iso = power_spectrum(&xcorr_iso);

        // View 3: Signature consistency score (View A)
        let spec_consistency = power_spectrum(&consistency_score);

        // View 4: r-value spectrum (View C)
        let spec_r = view_c_spectrum.clone();

        // View 5: Pairwise consistency (View D)
        let spec_pairwise = power_spectrum(&pairwise_score);

        // View 6: Lattice view
        let spec_lattice = lattice_spectrum.clone();

        let all_spectra = [
            &spec_lens,
            &spec_iso,
            &spec_consistency,
            &spec_r,
            &spec_pairwise,
            &spec_lattice,
        ];
        let view_names = [
            "Lens     ",
            "Iso      ",
            "SigConsist",
            "r-values ",
            "Pairwise ",
            "Lattice  ",
        ];

        // Print 6×6 distance matrix
        eprint!("          ");
        for name in &view_names {
            eprint!("{:>10}", &name[..name.len().min(10)]);
        }
        eprintln!();

        let mut dist_matrix = vec![vec![0.0f64; 6]; 6];
        for i in 0..6 {
            eprint!("  {:10}", view_names[i]);
            for j in 0..6 {
                let d_val = spectral_distance(all_spectra[i], all_spectra[j]);
                dist_matrix[i][j] = d_val;
                eprint!("  {:8.4}", d_val);
            }
            eprintln!();
        }

        // Analyze clustering
        eprintln!("\n  Clustering analysis:");

        // Structural views: 0 (lens), 1 (iso)
        let structural_avg = (dist_matrix[0][1]) / 1.0;
        eprintln!(
            "    Structural-structural avg distance: {:.4}",
            structural_avg
        );

        // Signature views: 2 (consistency), 4 (pairwise)
        let sig_internal = dist_matrix[2][4];
        eprintln!(
            "    Signature-signature (consistency↔pairwise): {:.4}",
            sig_internal
        );

        // Cross-cluster
        let mut cross_dists = Vec::new();
        for &s in &[0usize, 1] {
            for &sig in &[2usize, 4] {
                cross_dists.push(dist_matrix[s][sig]);
            }
        }
        let cross_avg: f64 = cross_dists.iter().sum::<f64>() / cross_dists.len() as f64;
        eprintln!("    Structural↔Signature avg distance: {:.4}", cross_avg);

        // Public views: 3 (r-values), 5 (lattice)
        let public_sig = dist_matrix[3][5];
        eprintln!("    Public data (r-values↔lattice): {:.4}", public_sig);

        // ─── Meta-graph navigation from View 4 ─────────────────────────
        eprintln!("\n  Meta-graph navigation from View 4 (r-values):");
        reset_ops();

        // Start at r-value spectrum (purely public data)
        // Navigate through meta-graph: find the closest view, cross-correlate, repeat
        let mut current_spec = spec_r.clone();
        let mut visited = vec![false; 6];
        visited[3] = true; // View 4 = index 3

        for step in 0..5 {
            // Find nearest unvisited view
            let mut best_idx = 0;
            let mut best_dist = f64::MAX;
            for j in 0..6 {
                if !visited[j] {
                    let d_val = spectral_distance(&current_spec, all_spectra[j]);
                    if d_val < best_dist {
                        best_dist = d_val;
                        best_idx = j;
                    }
                }
            }

            visited[best_idx] = true;
            eprintln!(
                "    step {}: → {} (dist={:.4})",
                step, view_names[best_idx], best_dist
            );

            // If we reach the signature consistency view, check if we can extract d
            if best_idx == 2 {
                // View 3 = consistency score
                let nav_argmax = consistency_score
                    .iter()
                    .enumerate()
                    .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                    .unwrap()
                    .0;
                eprintln!(
                    "    → Reached consistency view, argmax = {} (d = {})",
                    nav_argmax, d
                );
            }

            current_spec = all_spectra[best_idx].clone();
        }

        let nav_ops = get_ops();
        eprintln!("  Navigation cost: {} group ops", nav_ops);

        // ─── Final Summary ─────────────────────────────────────────────
        eprintln!("\n╔══════════════════════════════════════════════════════════════╗");
        eprintln!("║                        RESULTS                              ║");
        eprintln!("╚══════════════════════════════════════════════════════════════╝\n");

        eprintln!(
            "  Curve: y² = x³ + x + 4 (mod 251), |E| = {}, generator order = {}",
            num_points, n
        );
        eprintln!("  Private key d = {}, public key Q = {:?}", d, pub_key);
        eprintln!("  Signatures: {}", signatures.len());
        eprintln!();

        // View A result
        eprintln!(
            "  View A (consistency): d recovered at argmax = {} ✓",
            view_a_argmax
        );
        eprintln!(
            "    Cost: {} group ops (O(n · sigs) = brute force)",
            view_a_ops
        );

        // View D result
        eprintln!(
            "  View D (pairwise): d recovered at argmax = {} ✓",
            view_d_argmax
        );
        eprintln!(
            "    Cost: {} group ops (O(n · sigs) = brute force)",
            view_d_ops
        );

        // Brute force
        eprintln!("  Brute force: d = {} ✓", brute_d.unwrap());
        eprintln!(
            "    Cost: {} group ops (early exit at d = {})",
            brute_ops, d
        );

        // Circular loop
        let loop_converged = prev_argmax == d as usize;
        eprintln!(
            "  Circular loop: final argmax = {} {}",
            prev_argmax,
            if loop_converged {
                "✓ CONVERGED"
            } else {
                "✗ DID NOT CONVERGE"
            }
        );
        eprintln!("    Cost: {} group ops", loop_ops);

        // Key insight
        eprintln!("\n  ─── KEY INSIGHT ───");
        eprintln!("  Signature views A and D uniquely identify d with cost O(n·sigs).");
        eprintln!("  This is no better than brute force over d.");
        eprintln!("  The spectral structure of the consistency signal is a DELTA at d —");
        eprintln!("  maximally concentrated. But computing it requires O(n) evaluations.");
        eprintln!();
        if loop_converged {
            eprintln!("  The circular loop DID converge, but used pre-computed lens view");
            eprintln!("  (ring ordering = DLP for all elements). This is CIRCULAR.");
            eprintln!("  The loop works because the lens view encodes the ring ordering.");
            eprintln!("  Without it, the public-data views (B, C) are too sparse to guide.");
        } else {
            eprintln!("  The circular loop did NOT converge from public data alone.");
            eprintln!("  Views B (t_i histogram) and C (r-value spectrum) are sparse signals");
            eprintln!("  over n = {} positions with only 20 nonzero entries.", n);
            eprintln!("  Cross-correlating sparse signals with the lens view (also pseudorandom)");
            eprintln!("  produces noise, not signal. The terrain remains flat.");
        }
        eprintln!();
        eprintln!("  The signature views cluster with each other (low mutual distance)");
        eprintln!("  but only because they're all functions of d — computing them");
        eprintln!("  already requires knowing d (or brute-forcing over d).");
        eprintln!();
        eprintln!("  RESULT: NEGATIVE. Signatures provide terrain but only at O(n) cost.");
        eprintln!("  The circular-reflexive loop cannot bootstrap from public signature");
        eprintln!("  data alone. The spectral shortcut does not exist here.");

        // The test passes regardless — we're documenting the result.
        // But verify the brute force works correctly.
        assert_eq!(brute_d, Some(d));
        assert_eq!(view_a_argmax, d as usize);
    }
}
