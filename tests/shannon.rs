//! Shannon Information Recovery — ECDSA signature error landscape analysis.
//!
//! Each ECDSA signature provides 50% Shannon recovery: one equation, two unknowns,
//! cuts the joint (d, k) space from 2D to 1D. With m signatures: m equations, m+1
//! unknowns. The remaining degree of freedom IS d.
//!
//! The information to determine d is IN the system — it's underdetermined by exactly
//! 1 DOF. The question: can a spectral prior on the x-projection f(k) = x(kG) collapse
//! that last DOF?
//!
//! For a candidate d_hat:
//!   k_hat_i = s_i⁻¹ · (h_i + r_i · d_hat) mod n
//!   R_hat_i = k_hat_i · G
//!   Check: x(R_hat_i) == r_i?
//!
//! For d_hat = d_true: ALL checks pass.
//! For d_hat ≠ d_true: checks fail (probability ~1/n each).
//!
//! The error function error(d_hat) = Σ_i |x(k_hat_i · G) - r_i| has a UNIQUE ZERO
//! at d_hat = d_true. This test explores the structure of that error landscape.

mod curve {
    use std::sync::atomic::{AtomicU64, Ordering};

    /// Global counter for group operations.
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

    /// ECDSA signature tuple.
    #[derive(Debug, Clone)]
    struct Signature {
        r: u64,
        s: u64,
        h: u64,
        #[allow(dead_code)]
        k: u64, // ground truth nonce (for validation only)
    }

    /// Find the order of a point on the curve.
    fn point_order(pt: Point, a: u64, p: u64, max_order: u64) -> u64 {
        let mut current = pt;
        for i in 1..=max_order {
            if current == Point::Infinity {
                return i;
            }
            current = point_add(current, pt, a, p);
        }
        0
    }

    /// Check if n is prime (trial division, fine for small n).
    fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n < 4 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5u64;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    /// Find a generator of a prime-order subgroup on y² = x³ + ax + b (mod p).
    /// For ECDSA we need the subgroup order to be prime so mod_inv always works.
    fn find_prime_order_generator(a: u64, b: u64, p: u64) -> (Point, u64) {
        let points = enumerate_curve(a, b, p);
        let group_size = points.len() as u64; // includes infinity
        eprintln!("  Curve has {} points (including infinity)", group_size);

        // Factor group_size to find its largest prime factor
        let mut n = group_size;
        let mut factors = Vec::new();
        let mut d = 2u64;
        while d * d <= n {
            while n % d == 0 {
                factors.push(d);
                n /= d;
            }
            d += 1;
        }
        if n > 1 {
            factors.push(n);
        }
        factors.sort();
        factors.dedup();
        eprintln!("  Group size {} factors: {:?}", group_size, factors);

        // Find the largest prime factor as subgroup order
        let target_order = *factors.iter().rev().find(|&&f| is_prime(f)).unwrap();
        let cofactor = group_size / target_order;
        eprintln!(
            "  Target prime subgroup order: {}, cofactor: {}",
            target_order, cofactor
        );

        // Find a generator of the prime-order subgroup:
        // Multiply a random point by the cofactor to get into the subgroup.
        for &pt in &points {
            if pt == Point::Infinity {
                continue;
            }
            let candidate = scalar_mul(cofactor, pt, a, p);
            if candidate == Point::Infinity {
                continue;
            }
            // Verify it has the right order
            let ord = point_order(candidate, a, p, target_order + 1);
            if ord == target_order {
                return (candidate, target_order);
            }
        }
        panic!("No generator found for prime-order subgroup");
    }

    /// Generate ECDSA signatures given a generator and its order.
    fn generate_signatures(
        num_sigs: usize,
        g: Point,
        n: u64,
        a: u64,
        p: u64,
        d: u64,
    ) -> Vec<Signature> {
        let mut sigs = Vec::new();
        for i in 0..num_sigs {
            // Deterministic nonce: k_i = (7*i*i + 13*i + 37) mod (n-1) + 1
            let ii = i as u64;
            let k = (7 * ii * ii + 13 * ii + 37) % (n - 1) + 1;

            // Ensure k is coprime to n
            if mod_inv(k, n).is_none() {
                continue;
            }

            // R = k·G
            let r_point = scalar_mul(k, g, a, p);
            let r = match r_point {
                Point::Affine { x, .. } => x % n,
                Point::Infinity => continue, // skip degenerate
            };
            if r == 0 {
                continue;
            }

            // Message hash (deterministic)
            let h = ((i as u64 * 17 + 5) % n).max(1);

            // s = k⁻¹ · (h + r·d) mod n
            let k_inv = mod_inv(k, n).unwrap();
            let s = (k_inv as u128 * ((h as u128 + r as u128 * d as u128) % n as u128)) as u64 % n;
            if s == 0 {
                continue;
            }

            sigs.push(Signature { r, s, h, k });
        }
        sigs
    }

    /// Compute score(d_c) = number of signatures verified by candidate d_c.
    fn score_candidate(d_c: u64, sigs: &[Signature], g: Point, n: u64, a: u64, p: u64) -> usize {
        let mut count = 0;
        for sig in sigs {
            let s_inv = match mod_inv(sig.s, n) {
                Some(v) => v,
                None => continue,
            };
            // k_c = s⁻¹ · (h + r · d_c) mod n
            let k_c = (s_inv as u128 * ((sig.h as u128 + sig.r as u128 * d_c as u128) % n as u128))
                as u64
                % n;
            if k_c == 0 {
                continue;
            }
            // R_c = k_c · G
            let r_point = scalar_mul(k_c, g, a, p);
            match r_point {
                Point::Affine { x, .. } => {
                    if x % n == sig.r {
                        count += 1;
                    }
                }
                Point::Infinity => {}
            }
        }
        count
    }

    /// Compute soft error for candidate d_c.
    fn soft_error_candidate(d_c: u64, sigs: &[Signature], g: Point, n: u64, a: u64, p: u64) -> f64 {
        let mut total = 0.0f64;
        for sig in sigs {
            let s_inv = match mod_inv(sig.s, n) {
                Some(v) => v,
                None => continue,
            };
            let k_c = (s_inv as u128 * ((sig.h as u128 + sig.r as u128 * d_c as u128) % n as u128))
                as u64
                % n;
            if k_c == 0 {
                total += (sig.r as f64) * (sig.r as f64);
                continue;
            }
            let r_point = scalar_mul(k_c, g, a, p);
            let x_pred = match r_point {
                Point::Affine { x, .. } => x,
                Point::Infinity => 0,
            };
            let diff = if x_pred >= sig.r {
                x_pred - sig.r
            } else {
                sig.r - x_pred
            };
            let diff = diff.min(p - diff); // wrap-around distance
            total += (diff as f64) * (diff as f64);
        }
        total
    }

    /// DFT of a real signal at frequency k.
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

    #[test]
    fn shannon_error_landscape() {
        eprintln!("\n=== SHANNON ERROR LANDSCAPE ===");
        eprintln!(
            "  Hypothesis: ECDSA signature verification error has navigable spectral structure\n"
        );

        // Use a larger field for meaningful statistics.
        // y² = x³ + x + 1 (mod 251) has 282 = 2·3·47 points → prime subgroup of order 47.
        // Too small. Try y² = x³ + x + 1 (mod 4093) for a larger group.
        // We'll find the largest prime-order subgroup automatically.
        let a_curve = 1u64;
        let b_curve = 1u64;
        let p = 251u64;
        let num_sigs = 20;

        // ---- Step 1: Setup ----
        eprintln!("--- Step 1: Setup ---");
        reset_ops();

        // First find the generator/order so we can pick d_true < n
        let (g, n) = find_prime_order_generator(a_curve, b_curve, p);
        let d_true = 42u64 % (n - 1) + 1; // ensure 1 <= d_true < n
        eprintln!("  Using d_true = {} (n = {})", d_true, n);

        let sigs = generate_signatures(num_sigs, g, n, a_curve, p, d_true);
        eprintln!(
            "  Generated {} signatures with d_true={}",
            sigs.len(),
            d_true
        );
        eprintln!("  Group order n={}", n);

        // Verify ground truth: all signatures should verify with d_true
        let true_score = score_candidate(d_true, &sigs, g, n, a_curve, p);
        eprintln!(
            "  Verification: score(d_true={}) = {}/{}",
            d_true,
            true_score,
            sigs.len()
        );
        assert_eq!(
            true_score,
            sigs.len(),
            "All signatures must verify with true private key"
        );

        // ---- Step 2: Error Landscape ----
        eprintln!("\n--- Step 2: Error Landscape ---");
        reset_ops();

        let mut scores = vec![0usize; n as usize];
        let mut soft_errors = vec![0.0f64; n as usize];

        for d_c in 0..n {
            scores[d_c as usize] = score_candidate(d_c, &sigs, g, n, a_curve, p);
            soft_errors[d_c as usize] = soft_error_candidate(d_c, &sigs, g, n, a_curve, p);
        }

        let ops_landscape = get_ops();
        eprintln!("  Group operations for full landscape: {}", ops_landscape);

        // Statistics
        let max_score = *scores.iter().max().unwrap();
        let num_perfect = scores.iter().filter(|&&s| s == sigs.len()).count();
        let num_nonzero = scores.iter().filter(|&&s| s > 0).count();
        let avg_score: f64 = scores.iter().map(|&s| s as f64).sum::<f64>() / n as f64;

        eprintln!("  Score landscape:");
        eprintln!("    max score: {}", max_score);
        eprintln!("    perfect scores (= {}): {}", sigs.len(), num_perfect);
        eprintln!("    nonzero scores: {} / {}", num_nonzero, n);
        eprintln!("    average score: {:.4}", avg_score);

        // Show scores near d_true
        eprintln!("\n  Score near d_true={}:", d_true);
        for delta in [-10i64, -5, -2, -1, 0, 1, 2, 5, 10] {
            let d_c = ((d_true as i64 + delta) % n as i64 + n as i64) as u64 % n;
            eprintln!(
                "    d_c={:3} (delta={:+3}): score={:2}, soft_error={:.1}",
                d_c, delta, scores[d_c as usize], soft_errors[d_c as usize]
            );
        }

        // Basin of attraction: how wide is the peak around d_true?
        let mut basin_left = 0u64;
        let mut basin_right = 0u64;
        for delta in 1..n {
            let d_c = ((d_true as i64 - delta as i64) % n as i64 + n as i64) as u64 % n;
            if scores[d_c as usize] > 0 {
                basin_left = delta;
            } else {
                break;
            }
        }
        for delta in 1..n {
            let d_c = (d_true + delta) % n;
            if scores[d_c as usize] > 0 {
                basin_right = delta;
            } else {
                break;
            }
        }
        eprintln!(
            "\n  Basin of attraction (score > 0): left={}, right={}, total width={}",
            basin_left,
            basin_right,
            basin_left + basin_right + 1
        );

        // ---- Step 3: DFT of Score Landscape ----
        eprintln!("\n--- Step 3: DFT of Score Landscape ---");

        let score_signal: Vec<f64> = scores.iter().map(|&s| s as f64).collect();
        let nn = n as usize;

        // Full power spectrum
        let mut power = vec![0.0f64; nn];
        let mut total_energy = 0.0f64;
        for k in 0..nn {
            let (re, im) = dft_coefficient(&score_signal, k, nn);
            power[k] = re * re + im * im;
            total_energy += power[k];
        }

        // Spectral entropy
        let mut spectral_entropy = 0.0f64;
        for k in 0..nn {
            let pk = power[k] / total_energy;
            if pk > 1e-15 {
                spectral_entropy -= pk * pk.ln();
            }
        }
        let max_entropy = (nn as f64).ln();
        let normalized_entropy = spectral_entropy / max_entropy;

        // Peak-to-average ratio
        let avg_power = total_energy / nn as f64;
        let max_power = power.iter().cloned().fold(0.0f64, |a, b| a.max(b));
        let peak_to_avg = max_power / avg_power;

        // Energy concentration: how many frequencies hold 90% of energy?
        let mut sorted_power = power.clone();
        sorted_power.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let mut cumulative = 0.0f64;
        let mut freq_for_90 = 0usize;
        for &pw in &sorted_power {
            cumulative += pw;
            freq_for_90 += 1;
            if cumulative >= 0.90 * total_energy {
                break;
            }
        }

        eprintln!("  Score DFT spectrum:");
        eprintln!("    total energy: {:.2}", total_energy);
        eprintln!("    peak-to-average: {:.2}", peak_to_avg);
        eprintln!(
            "    spectral entropy: {:.4} / {:.4} (normalized: {:.4})",
            spectral_entropy, max_entropy, normalized_entropy
        );
        eprintln!(
            "    frequencies for 90% energy: {} / {} ({:.1}%)",
            freq_for_90,
            nn,
            100.0 * freq_for_90 as f64 / nn as f64
        );

        // Top 10 frequencies
        let mut freq_idx: Vec<(usize, f64)> =
            power.iter().enumerate().map(|(i, &p)| (i, p)).collect();
        freq_idx.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        eprintln!("    top 10 frequencies:");
        for &(k, pw) in freq_idx.iter().take(10) {
            eprintln!(
                "      k={:4}: power={:.2} ({:.1}% of total)",
                k,
                pw,
                100.0 * pw / total_energy
            );
        }

        // Also DFT the soft error
        let soft_signal: Vec<f64> = soft_errors.clone();
        let mut soft_power = vec![0.0f64; nn];
        let mut soft_total = 0.0f64;
        for k in 0..nn {
            let (re, im) = dft_coefficient(&soft_signal, k, nn);
            soft_power[k] = re * re + im * im;
            soft_total += soft_power[k];
        }
        let soft_avg = soft_total / nn as f64;
        let soft_max = soft_power.iter().cloned().fold(0.0f64, |a, b| a.max(b));
        eprintln!("\n  Soft error DFT:");
        eprintln!("    peak-to-average: {:.2}", soft_max / soft_avg);

        // ---- Step 4: Gradient Signal ----
        eprintln!("\n--- Step 4: Gradient Signal ---");

        // Compute gradient: Δ(d_c) = score(d_c + 1) - score(d_c)
        let mut gradient = vec![0i64; nn];
        for d_c in 0..nn {
            let next = (d_c + 1) % nn;
            gradient[d_c] = scores[next] as i64 - scores[d_c] as i64;
        }

        eprintln!("  Gradient near d_true={}:", d_true);
        for delta in [-10i64, -5, -3, -2, -1, 0, 1, 2, 3, 5, 10] {
            let d_c = ((d_true as i64 + delta) % n as i64 + n as i64) as u64 % n;
            let grad = gradient[d_c as usize];
            let direction = if delta < 0 && grad > 0 {
                "→ toward d_true"
            } else if delta > 0 && grad < 0 {
                "→ toward d_true"
            } else if delta == 0 {
                "(at d_true)"
            } else if grad == 0 {
                "flat"
            } else {
                "→ AWAY from d_true"
            };
            eprintln!(
                "    d_c={:3} (delta={:+3}): gradient={:+2} {}",
                d_c, delta, grad, direction
            );
        }

        // Soft error gradient
        eprintln!("\n  Soft error gradient near d_true={}:", d_true);
        for delta in [-10i64, -5, -2, -1, 0, 1, 2, 5, 10] {
            let d_c = ((d_true as i64 + delta) % n as i64 + n as i64) as u64 % n;
            let next = ((d_c as i64 + 1) % n as i64 + n as i64) as u64 % n;
            let grad = soft_errors[next as usize] - soft_errors[d_c as usize];
            let direction = if delta < 0 && grad < 0.0 {
                "→ toward d_true (downhill)"
            } else if delta > 0 && grad > 0.0 {
                "→ toward d_true (downhill from other side)"
            } else if delta == 0 {
                "(at d_true)"
            } else {
                "no clear signal"
            };
            eprintln!(
                "    d_c={:3} (delta={:+3}): Δ_soft={:+.1} {}",
                d_c, delta, grad, direction
            );
        }

        // ---- Step 5: Iterative Refinement ----
        eprintln!("\n--- Step 5: Iterative Refinement ---");

        // Method A: Greedy local search
        eprintln!("\n  Method A: Greedy local search");
        let mut method_a_successes = 0;
        let num_trials = 20;
        let mut method_a_total_steps = 0u64;
        for trial in 0..num_trials {
            reset_ops();
            let d_hat = (trial * 13 + 7) as u64 % n; // deterministic starting points
            let mut current = d_hat;
            let mut best_score = scores[current as usize];
            let mut steps = 0u64;

            for _ in 0..(n * 2) {
                let left = (current + n - 1) % n;
                let right = (current + 1) % n;
                let sl = scores[left as usize];
                let sr = scores[right as usize];

                if sl > best_score && sl >= sr {
                    current = left;
                    best_score = sl;
                } else if sr > best_score {
                    current = right;
                    best_score = sr;
                } else {
                    break; // local optimum
                }
                steps += 1;
            }

            let found = current == d_true;
            if found {
                method_a_successes += 1;
            }
            method_a_total_steps += steps;
            if trial < 5 || found {
                eprintln!(
                    "    trial {}: start={:3}, converged to {:3} in {} steps. {}",
                    trial,
                    d_hat,
                    current,
                    steps,
                    if found { "FOUND" } else { "missed" }
                );
            }
        }
        eprintln!(
            "    Success: {}/{} trials, avg steps: {:.1}",
            method_a_successes,
            num_trials,
            method_a_total_steps as f64 / num_trials as f64
        );

        // Method B: Spectral-guided search (window DFT)
        eprintln!("\n  Method B: Spectral-guided search");
        let mut method_b_successes = 0;
        let mut method_b_total_ops = 0u64;
        for trial in 0..num_trials {
            reset_ops();
            let mut d_hat = (trial * 13 + 7) as u64 % n;
            let mut found = false;

            for iteration in 0..50 {
                if scores[d_hat as usize] == sigs.len() {
                    found = true;
                    if trial < 5 {
                        eprintln!(
                            "    trial {}: found d_true at iteration {}",
                            trial, iteration
                        );
                    }
                    break;
                }

                // Window around d_hat
                let window_size = 32usize.min(nn);
                let mut window_signal = vec![0.0f64; window_size];
                for w in 0..window_size {
                    let d_c = ((d_hat as i64 - window_size as i64 / 2 + w as i64) % n as i64
                        + n as i64) as u64
                        % n;
                    window_signal[w] = scores[d_c as usize] as f64;
                }

                // Find dominant frequency in window
                let mut best_freq = 0;
                let mut best_mag = 0.0f64;
                for k in 1..window_size {
                    let (re, im) = dft_coefficient(&window_signal, k, window_size);
                    let mag = re * re + im * im;
                    if mag > best_mag {
                        best_mag = mag;
                        best_freq = k;
                    }
                }

                // Use dominant frequency phase to suggest direction
                let (re, im) = dft_coefficient(&window_signal, best_freq, window_size);
                let phase = im.atan2(re);
                let shift = (-phase * window_size as f64 / (2.0 * PI * best_freq as f64)) as i64;
                let step = shift.clamp(-(n as i64 / 4), n as i64 / 4);

                d_hat = ((d_hat as i64 + step) % n as i64 + n as i64) as u64 % n;
            }

            method_b_total_ops += get_ops();
            if found {
                method_b_successes += 1;
            }
        }
        eprintln!(
            "    Success: {}/{} trials, avg group ops: {:.0}",
            method_b_successes,
            num_trials,
            method_b_total_ops as f64 / num_trials as f64
        );

        // Method C: Multi-signature gradient (per-signature d-recovery)
        eprintln!("\n  Method C: Multi-signature gradient (per-signature consistency)");
        let mut method_c_successes = 0;
        let mut method_c_total_ops = 0u64;
        for trial in 0..num_trials {
            reset_ops();
            let mut d_hat = (trial * 13 + 7) as u64 % n;
            let mut found = false;

            for iteration in 0..100 {
                if scores[d_hat as usize] == sigs.len() {
                    found = true;
                    if trial < 5 {
                        eprintln!(
                            "    trial {}: found d_true at iteration {}",
                            trial, iteration
                        );
                    }
                    break;
                }

                // For each signature, find the d value in a local window that minimizes its residual
                let window = 10u64;
                let mut votes = std::collections::HashMap::new();
                for sig in &sigs {
                    let s_inv = match mod_inv(sig.s, n) {
                        Some(v) => v,
                        None => continue,
                    };
                    let mut best_d = d_hat;
                    let mut best_err = u64::MAX;
                    for delta_i in 0..(2 * window + 1) {
                        let d_c = ((d_hat as i64 - window as i64 + delta_i as i64) % n as i64
                            + n as i64) as u64
                            % n;
                        let k_c = (s_inv as u128
                            * ((sig.h as u128 + sig.r as u128 * d_c as u128) % n as u128))
                            as u64
                            % n;
                        if k_c == 0 {
                            continue;
                        }
                        let r_point = scalar_mul(k_c, g, a_curve, p);
                        let x_pred = match r_point {
                            Point::Affine { x, .. } => x,
                            Point::Infinity => continue,
                        };
                        let err = if x_pred % n >= sig.r {
                            (x_pred % n) - sig.r
                        } else {
                            sig.r - (x_pred % n)
                        };
                        if err < best_err {
                            best_err = err;
                            best_d = d_c;
                        }
                        if err == 0 {
                            break;
                        }
                    }
                    *votes.entry(best_d).or_insert(0u32) += 1;
                }

                // Move to the d_c with most votes
                if let Some((&best_d, _)) = votes.iter().max_by_key(|&(_, &v)| v) {
                    d_hat = best_d;
                } else {
                    // Random jump
                    d_hat = (d_hat + 17) % n;
                }
            }

            method_c_total_ops += get_ops();
            if found {
                method_c_successes += 1;
            }
        }
        eprintln!(
            "    Success: {}/{} trials, avg group ops: {:.0}",
            method_c_successes,
            num_trials,
            method_c_total_ops as f64 / num_trials as f64
        );

        // Method D: Bayesian update (full enumeration, for comparison)
        eprintln!("\n  Method D: Bayesian update (full enumeration baseline)");
        reset_ops();

        // With increasing numbers of signatures, count how many candidates survive
        for m in [1usize, 2, 5, 10, sigs.len()] {
            let sub_sigs = &sigs[..m.min(sigs.len())];
            let mut survivors = 0usize;
            for d_c in 0..n {
                let sc = score_candidate(d_c, sub_sigs, g, n, a_curve, p);
                if sc == sub_sigs.len() {
                    survivors += 1;
                }
            }
            eprintln!(
                "    m={:2} signatures: {} / {} candidates survive ({:.2}%)",
                sub_sigs.len(),
                survivors,
                n,
                100.0 * survivors as f64 / n as f64
            );
        }
        let bayesian_ops = get_ops();
        eprintln!("    Total group ops for Bayesian: {}", bayesian_ops);

        // Method E: Partial evaluation (BSGS-like)
        eprintln!("\n  Method E: Partial evaluation (sqrt(n) probe)");
        reset_ops();
        let sqrt_n = (n as f64).sqrt().ceil() as u64;
        eprintln!("    sqrt(n) = {}", sqrt_n);

        // Precompute: for each signature, for d_c in [0, sqrt_n), compute k_c and x(k_c · G)
        // Build a table: (sig_index, r_i) → d_c values that match
        let mut baby_steps: std::collections::HashMap<(usize, u64), Vec<u64>> =
            std::collections::HashMap::new();

        for (sig_idx, sig) in sigs.iter().enumerate() {
            let s_inv = match mod_inv(sig.s, n) {
                Some(v) => v,
                None => continue,
            };
            for d_c in 0..sqrt_n {
                let k_c = (s_inv as u128
                    * ((sig.h as u128 + sig.r as u128 * d_c as u128) % n as u128))
                    as u64
                    % n;
                if k_c == 0 {
                    continue;
                }
                let r_point = scalar_mul(k_c, g, a_curve, p);
                if let Point::Affine { x, .. } = r_point {
                    if x % n == sig.r {
                        baby_steps.entry((sig_idx, sig.r)).or_default().push(d_c);
                    }
                }
            }
        }

        let baby_ops = get_ops();
        let found_partial = baby_steps.values().any(|v| v.contains(&d_true));
        eprintln!("    Baby step phase: {} group ops", baby_ops);
        eprintln!(
            "    d_true={} in baby step range [0, {}): {}",
            d_true,
            sqrt_n,
            if found_partial { "YES" } else { "NO" }
        );

        // Giant steps: for each signature, test d_c = baby + j * sqrt_n
        reset_ops();
        let mut giant_found = false;
        let mut giant_d = 0u64;
        'outer: for j in 0..sqrt_n {
            for d_c_base in 0..sqrt_n {
                let d_c = (d_c_base + j * sqrt_n) % n;
                // Quick check with first signature only
                let sig = &sigs[0];
                let s_inv = mod_inv(sig.s, n).unwrap();
                let k_c = (s_inv as u128
                    * ((sig.h as u128 + sig.r as u128 * d_c as u128) % n as u128))
                    as u64
                    % n;
                if k_c == 0 {
                    continue;
                }
                let r_point = scalar_mul(k_c, g, a_curve, p);
                if let Point::Affine { x, .. } = r_point {
                    if x % n == sig.r {
                        // Verify with all signatures
                        let full_score = score_candidate(d_c, &sigs, g, n, a_curve, p);
                        if full_score == sigs.len() {
                            giant_found = true;
                            giant_d = d_c;
                            break 'outer;
                        }
                    }
                }
            }
        }
        let giant_ops = get_ops();
        eprintln!(
            "    Giant step phase: {} group ops, found={}, d={}",
            giant_ops, giant_found, giant_d
        );

        // ---- Step 6: Information-Theoretic Analysis ----
        eprintln!("\n--- Step 6: Information-Theoretic Analysis ---");

        let log2_n = (n as f64).log2();
        eprintln!("  log₂(n) = {:.2} bits", log2_n);

        for m in [1usize, 2, 3, 5, 10, 15, sigs.len()] {
            let sub_sigs = &sigs[..m.min(sigs.len())];

            // Count survivors: candidates where all m sigs verify
            let mut survivors = 0usize;
            for d_c in 0..n {
                let sc = score_candidate(d_c, sub_sigs, g, n, a_curve, p);
                if sc == sub_sigs.len() {
                    survivors += 1;
                }
            }

            // Entropy of the posterior
            let entropy = if survivors > 0 {
                (survivors as f64).log2()
            } else {
                0.0
            };

            let bits_recovered = log2_n - entropy;

            eprintln!(
                "    m={:2}: survivors={:4}, H(d|sigs)={:.2} bits, recovered={:.2}/{:.2} bits",
                sub_sigs.len(),
                survivors,
                entropy,
                bits_recovered,
                log2_n
            );
        }

        // ---- Step 7: Report ----
        eprintln!("\n=== STEP 7: FINAL REPORT ===\n");

        eprintln!("  Curve: y² = x³ + {}x + {} (mod {})", a_curve, b_curve, p);
        eprintln!("  Generator order: n = {}", n);
        eprintln!("  Private key: d = {}", d_true);
        eprintln!("  Signatures: {}", sigs.len());

        eprintln!("\n  SCORE LANDSCAPE:");
        eprintln!("    Unique perfect peaks: {}", num_perfect);
        eprintln!(
            "    Basin width (score > 0): {}",
            basin_left + basin_right + 1
        );
        eprintln!("    Average score: {:.4}", avg_score);
        let landscape_type = if basin_left + basin_right > 0 {
            "HAS local gradient structure"
        } else if num_nonzero == 1 {
            "DELTA function — no gradient, no navigation"
        } else {
            "scattered peaks — no basin"
        };
        eprintln!("    Structure: {}", landscape_type);

        eprintln!("\n  DFT OF SCORE LANDSCAPE:");
        eprintln!("    Peak-to-average: {:.2}", peak_to_avg);
        eprintln!("    Normalized spectral entropy: {:.4}", normalized_entropy);
        eprintln!(
            "    Frequencies for 90% energy: {} / {} ({:.1}%)",
            freq_for_90,
            nn,
            100.0 * freq_for_90 as f64 / nn as f64
        );
        let spectral_type = if freq_for_90 as f64 / nn as f64 <= 0.05 {
            "SPECTRALLY SPARSE — navigable"
        } else if freq_for_90 as f64 / nn as f64 <= 0.20 {
            "moderately concentrated"
        } else {
            "SPECTRALLY FLAT — not navigable"
        };
        eprintln!("    Assessment: {}", spectral_type);

        eprintln!("\n  ITERATIVE METHODS:");
        eprintln!(
            "    Method A (greedy local): {}/{} success, avg {:.0} steps",
            method_a_successes,
            num_trials,
            method_a_total_steps as f64 / num_trials as f64
        );
        eprintln!(
            "    Method B (spectral window): {}/{} success",
            method_b_successes, num_trials
        );
        eprintln!(
            "    Method C (multi-sig gradient): {}/{} success",
            method_c_successes, num_trials
        );
        eprintln!(
            "    Method D (Bayesian full): always succeeds, {} ops",
            bayesian_ops
        );
        eprintln!(
            "    Method E (BSGS-like): found={}, {} + {} ops",
            giant_found, baby_ops, giant_ops
        );

        eprintln!("\n  COST COMPARISON:");
        eprintln!("    Brute force: {} group ops (= n)", n);
        eprintln!("    BSGS: ~{} group ops (= √n)", sqrt_n);
        // Method A only succeeds if you start at d_true — it gets stuck at score 0 or 1
        let method_a_from_random = method_a_successes > 1; // more than just hitting d_true directly
        eprintln!(
            "    Method A converges from random start: {} ({}/{} — only if starting at/near d_true)",
            if method_a_from_random { "YES" } else { "NO" },
            method_a_successes,
            num_trials
        );
        // Method C's "success" must be evaluated against cost — if it uses more ops
        // than brute force, it's disguised enumeration, not a real signal.
        let method_c_avg_ops = method_c_total_ops as f64 / num_trials as f64;
        let method_c_real_signal = method_c_successes > num_trials / 2
            && method_c_avg_ops < (n as f64 * sigs.len() as f64); // must beat brute force

        let any_sublinear = (method_a_successes > num_trials / 2
            && (method_a_total_steps as f64 / num_trials as f64) < (n as f64).sqrt())
            || method_b_successes > num_trials / 2
            || method_c_real_signal;

        eprintln!("\n  VERDICT:");
        if basin_left + basin_right > 0 {
            eprintln!("    SIGNAL: Basin of attraction exists (width > 1)");
        }
        if freq_for_90 as f64 / nn as f64 <= 0.20 {
            eprintln!("    SIGNAL: Score landscape is spectrally concentrated");
        }
        if any_sublinear {
            eprintln!("    SIGNAL: An iterative method converges from random start in < n ops");
        }

        // Method C caveat
        if method_c_successes > num_trials / 2 && !method_c_real_signal {
            eprintln!(
                "    NOTE: Method C appears to succeed but uses {:.0} avg ops",
                method_c_avg_ops
            );
            eprintln!(
                "    vs brute force {} ops — it's disguised enumeration, not gradient descent.",
                n
            );
            eprintln!(
                "    Window ±10 on n={} covers {:.0}% of search space per iteration.",
                n,
                100.0 * 21.0f64.min(n as f64) / n as f64
            );
        }

        if num_perfect == 1
            && basin_left + basin_right == 0
            && freq_for_90 as f64 / nn as f64 > 0.20
            && !any_sublinear
        {
            eprintln!("    NEGATIVE: Error landscape is delta-at-d_true + flat noise.");
            eprintln!("    No gradient. No spectral concentration. No navigation.");
            eprintln!("    The 1 DOF ambiguity is real but the error surface provides");
            eprintln!("    no directional information. Brute force or BSGS only.");
        }
        if num_perfect == 1 && basin_left + basin_right == 0 && !any_sublinear {
            eprintln!("\n    The verification function x(kG) == r acts as a CRYPTOGRAPHIC HASH");
            eprintln!("    of the candidate d. It produces 0/1 with no intermediate signal.");
            eprintln!("    The scalar multiplication k·G destroys all local structure.");
            eprintln!("    Each candidate d maps to a pseudorandom k, which maps to a");
            eprintln!("    pseudorandom point. The probability of collision is 1/n.");
            eprintln!("    20 signatures don't help because each creates an independent");
            eprintln!("    0/1 check — there's no PARTIAL credit for being close to d_true.");
        }

        eprintln!("\n=== END SHANNON ===");
    }
}
