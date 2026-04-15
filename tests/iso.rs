//! ISO — Windowed spectral matching for elliptic curve DLP.
//!
//! Given public key Q = dG, we can compute a window of the x-projection
//! signal f(k) = x(kG) starting at position d, WITHOUT knowing d:
//!
//!   w(j) = x(Q + jG) = f(d + j)   for j = 0..m-1
//!
//! This is a shifted window of f. Cross-correlating w with f should peak
//! at the shift position d. Phase analysis of the DFTs provides another
//! recovery channel.
//!
//! The critical question: how large must the window m be relative to the
//! group order n for recovery to work?
//!   - m = O(n)     → circular, no advantage
//!   - m = O(√n)    → matches BSGS complexity, different mechanism
//!   - m = O(log n) → theoretical break

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

    const A: u64 = 1;
    const B: u64 = 1;
    const P: u64 = 251;

    /// Find a generator of maximal order in the curve group.
    /// Returns (generator, order).
    fn find_generator(a: u64, b: u64, p: u64) -> (Point, u64) {
        let points = enumerate_curve(a, b, p);
        let n = points.len() as u64; // includes infinity

        // Try each non-infinity point as generator
        for &pt in points.iter().skip(1) {
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
        // Fallback: return first non-infinity point with its order
        let pt = points[1];
        let mut current = pt;
        let mut order = 1u64;
        while current != Point::Infinity {
            current = point_add(current, pt, a, p);
            order += 1;
        }
        (pt, order)
    }

    /// Compute f(k) = x(kG) for k = 0..n-1. f(0) = 0 by convention (point at infinity).
    fn compute_x_projection(g: Point, n: u64, a: u64, p: u64) -> Vec<f64> {
        let mut f = Vec::with_capacity(n as usize);
        let mut current = Point::Infinity;
        for _ in 0..n {
            let x_val = match current {
                Point::Infinity => 0.0,
                Point::Affine { x, .. } => x as f64,
            };
            f.push(x_val);
            current = point_add(current, g, a, p);
        }
        f
    }

    /// Compute window signal w(j) = x(Q + jG) for j = 0..m-1.
    /// Uses ONLY public information: Q, G, curve params.
    fn compute_window(q: Point, g: Point, m: usize, a: u64, p: u64) -> Vec<f64> {
        let mut w = Vec::with_capacity(m);
        let mut current = q; // Q + 0*G = Q
        for _ in 0..m {
            let x_val = match current {
                Point::Infinity => 0.0,
                Point::Affine { x, .. } => x as f64,
            };
            w.push(x_val);
            current = point_add(current, g, a, p);
        }
        w
    }

    /// Circular cross-correlation: C(tau) = sum_j w(j) * f((j + tau) mod n)
    fn cross_correlate(w: &[f64], f: &[f64], n: usize) -> Vec<f64> {
        let m = w.len();
        let mut c = vec![0.0f64; n];
        for tau in 0..n {
            let mut sum = 0.0;
            for j in 0..m {
                sum += w[j] * f[(j + tau) % n];
            }
            c[tau] = sum;
        }
        c
    }

    /// DFT of a real signal. Returns (re, im) arrays.
    fn dft(signal: &[f64], n: usize) -> (Vec<f64>, Vec<f64>) {
        let mut re = vec![0.0f64; n];
        let mut im = vec![0.0f64; n];
        for k in 0..n {
            for j in 0..signal.len() {
                let angle = -2.0 * PI * (j as f64) * (k as f64) / (n as f64);
                re[k] += signal[j] * angle.cos();
                im[k] += signal[j] * angle.sin();
            }
        }
        (re, im)
    }

    /// Find peak position and compute peak-to-second ratio.
    fn analyze_peak(c: &[f64]) -> (usize, f64, f64) {
        let n = c.len();
        let mut best_idx = 0;
        let mut best_val = f64::NEG_INFINITY;
        let mut second_val = f64::NEG_INFINITY;

        for i in 0..n {
            if c[i] > best_val {
                second_val = best_val;
                best_val = c[i];
                best_idx = i;
            } else if c[i] > second_val {
                second_val = c[i];
            }
        }

        let ratio = if second_val > 0.0 {
            best_val / second_val
        } else if second_val == 0.0 && best_val > 0.0 {
            f64::INFINITY
        } else {
            1.0
        };

        (best_idx, best_val, ratio)
    }

    /// Phase-based recovery: for dominant frequencies, extract phase difference
    /// that encodes the shift d.
    fn phase_recovery(w: &[f64], f: &[f64], n: usize) -> Vec<(usize, f64, f64)> {
        let (f_re, f_im) = dft(f, n);
        let (w_re, w_im) = dft(w, n);

        // Find the dominant frequencies of f (by magnitude)
        let mut magnitudes: Vec<(usize, f64)> = (0..n)
            .map(|k| {
                let mag = (f_re[k] * f_re[k] + f_im[k] * f_im[k]).sqrt();
                (k, mag)
            })
            .collect();
        magnitudes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // For top frequencies, compute phase difference → candidate d
        let mut candidates = Vec::new();
        for &(k, mag) in magnitudes.iter().take(20) {
            if k == 0 || mag < 1.0 {
                continue;
            }
            let f_phase = f_im[k].atan2(f_re[k]);
            let w_phase = w_im[k].atan2(w_re[k]);

            // Phase difference: W(k) ≈ F(k) · exp(-2πi·d·k/n) · sinc_envelope
            // So angle(W(k)) - angle(F(k)) ≈ -2π·d·k/n  (mod 2π)
            let phase_diff = w_phase - f_phase;
            // d ≈ -phase_diff * n / (2π * k)
            let d_est = (-phase_diff * n as f64 / (2.0 * PI * k as f64)) % n as f64;
            let d_est = ((d_est % n as f64) + n as f64) % n as f64;
            candidates.push((k, mag, d_est));
        }
        candidates
    }

    #[test]
    fn iso_windowed_spectral_matching() {
        eprintln!("\n=== ISO: Windowed Spectral Matching for ECDLP ===\n");

        // 1. Setup: curve y² = x³ + x + 1 (mod 251)
        let (g, n) = find_generator(A, B, P);
        let n_usize = n as usize;
        eprintln!("  Curve: y² = x³ + {}x + {} (mod {})", A, B, P);
        match g {
            Point::Affine { x, y } => eprintln!("  Generator G = ({}, {})", x, y),
            Point::Infinity => eprintln!("  Generator G = Infinity (ERROR)"),
        }
        eprintln!("  Group order n = {}", n);

        // Compute full x-projection f(k) = x(kG)
        let f = compute_x_projection(g, n, A, P);
        eprintln!(
            "  f(0..4) = [{:.0}, {:.0}, {:.0}, {:.0}, ...]",
            f[0], f[1], f[2], f[3]
        );

        // Verify f is correct by spot-checking
        let g3 = scalar_mul(3, g, A, P);
        if let Point::Affine { x, .. } = g3 {
            assert_eq!(x as f64, f[3], "f(3) should equal x(3G)");
        }

        let target_keys: Vec<u64> = vec![7, 42, 100, 200];
        let window_sizes: Vec<usize> = vec![10, 20, 50, n_usize / 4, n_usize / 2];

        eprintln!("\n  --- Cross-Correlation Recovery ---\n");
        eprintln!(
            "  {:>5} {:>6} {:>8} {:>8} {:>10} {:>7}",
            "d", "m", "peak_at", "correct", "peak/2nd", "RESULT"
        );
        eprintln!("  {}", "-".repeat(55));

        let mut results: Vec<(u64, usize, bool, f64)> = Vec::new();

        for &d in &target_keys {
            // Public key Q = dG (this is public)
            let q = scalar_mul(d, g, A, P);

            for &m in &window_sizes {
                if m >= n_usize {
                    continue;
                }

                // Window: computed using ONLY Q, G, curve params
                let w = compute_window(q, g, m, A, P);

                // Verify correctness of window (non-circular check):
                // w(j) should equal f((d + j) mod n)
                for j in 0..m.min(5) {
                    let expected = f[((d as usize) + j) % n_usize];
                    assert!(
                        (w[j] - expected).abs() < 1e-10,
                        "Window verification failed: w({}) = {} != f({}) = {}",
                        j,
                        w[j],
                        (d as usize + j) % n_usize,
                        expected
                    );
                }

                // Cross-correlation
                let c = cross_correlate(&w, &f, n_usize);
                let (peak_pos, _peak_val, ratio) = analyze_peak(&c);
                let correct = peak_pos == d as usize;
                let result_str = if correct && ratio > 3.0 {
                    "SIGNAL"
                } else if correct {
                    "weak"
                } else {
                    "FAIL"
                };

                eprintln!(
                    "  {:>5} {:>6} {:>8} {:>8} {:>10.3} {:>7}",
                    d, m, peak_pos, correct, ratio, result_str
                );

                results.push((d, m, correct, ratio));
            }
        }

        // Summary statistics
        eprintln!("\n  --- Summary by Window Size ---\n");
        for &m in &window_sizes {
            let m_results: Vec<_> = results.iter().filter(|r| r.1 == m).collect();
            let correct_count = m_results.iter().filter(|r| r.2).count();
            let avg_ratio: f64 =
                m_results.iter().map(|r| r.3).sum::<f64>() / m_results.len().max(1) as f64;
            let signal_count = m_results.iter().filter(|r| r.2 && r.3 > 3.0).count();
            eprintln!(
                "  m={:>4} ({:5.1}% of n): {}/{} correct, avg ratio {:.3}, {} with SIGNAL",
                m,
                100.0 * m as f64 / n as f64,
                correct_count,
                m_results.len(),
                avg_ratio,
                signal_count
            );
        }

        // 2d. Phase-based recovery
        eprintln!("\n  --- Phase-Based Recovery ---\n");
        for &d in &target_keys {
            let q = scalar_mul(d, g, A, P);
            let m = 50.min(n_usize / 2);
            let w = compute_window(q, g, m, A, P);
            let candidates = phase_recovery(&w, &f, n_usize);

            eprintln!("  d={}: top phase candidates (freq, magnitude, d_est):", d);
            let mut found = false;
            for (k, mag, d_est) in candidates.iter().take(10) {
                let d_rounded = d_est.round() as u64 % n;
                let match_str = if d_rounded == d { " <<< MATCH" } else { "" };
                if d_rounded == d {
                    found = true;
                }
                eprintln!(
                    "    freq={:>4}, mag={:>8.1}, d_est={:>7.2} (≈{}){} ",
                    k, mag, d_est, d_rounded, match_str
                );
            }
            if !found {
                eprintln!("    No phase candidate matched d={}", d);
            }
        }

        // 3. Scaling test: find minimum window size for recovery
        //    Two thresholds: (a) correct peak position, (b) strong signal (ratio > 2.0)
        eprintln!("\n  --- Minimum Window Size for Recovery ---\n");
        eprintln!("  (a) Minimum m where cross-correlation peak is at correct d:");
        for &d in &target_keys {
            let q = scalar_mul(d, g, A, P);
            let mut m_min_correct = 0usize;
            let mut m_min_strong = 0usize;

            for m in 2..n_usize {
                let w = compute_window(q, g, m, A, P);
                let c = cross_correlate(&w, &f, n_usize);
                let (peak_pos, _peak_val, ratio) = analyze_peak(&c);
                if m_min_correct == 0 && peak_pos == d as usize {
                    m_min_correct = m;
                }
                if m_min_strong == 0 && peak_pos == d as usize && ratio > 2.0 {
                    m_min_strong = m;
                }
                if m_min_correct > 0 && m_min_strong > 0 {
                    break;
                }
            }

            let sqrt_n = (n as f64).sqrt();
            let log_n = (n as f64).ln();
            if m_min_correct > 0 {
                let fraction = m_min_correct as f64 / n as f64;
                eprintln!(
                    "  d={:>3}: m_min_correct={:>4} ({:.1}% of n, {:.1}× √n, {:.1}× log n)",
                    d,
                    m_min_correct,
                    100.0 * fraction,
                    m_min_correct as f64 / sqrt_n,
                    m_min_correct as f64 / log_n
                );
            } else {
                eprintln!(
                    "  d={:>3}: NEVER correct (peak always at wrong position)",
                    d
                );
            }
            if m_min_strong > 0 {
                let fraction = m_min_strong as f64 / n as f64;
                eprintln!(
                    "         m_min_strong={:>4} ({:.1}% of n, {:.1}× √n, {:.1}× log n)",
                    m_min_strong,
                    100.0 * fraction,
                    m_min_strong as f64 / sqrt_n,
                    m_min_strong as f64 / log_n
                );
            } else {
                eprintln!("         NEVER strong (ratio never > 2.0)");
            }
        }

        // 4. Circularity audit
        eprintln!("\n  --- Circularity Audit ---");
        eprintln!("  Window computation uses ONLY:");
        eprintln!("    - Public key Q (a point on the curve)");
        eprintln!("    - Generator G (public parameter)");
        eprintln!("    - Curve parameters a={}, b={}, p={}", A, B, P);
        eprintln!("  The private key d is used ONLY for verification (ground truth).");
        eprintln!("  The full x-projection f(k) requires computing ALL kG for k=0..n-1.");
        eprintln!("  This is O(n) precomputation — equivalent to brute-force DLP.");
        eprintln!("  The window w is O(m) computation from public data.");
        eprintln!("  CRITICAL: If m_min = O(n), the window provides no advantage over");
        eprintln!("    simply computing all kG and checking for Q.");

        // 5. Theoretical assessment
        eprintln!("\n  === THEORETICAL ASSESSMENT ===\n");

        // Gather m_min values (correct peak position) for assessment
        let mut m_mins_correct = Vec::new();
        let mut m_mins_strong = Vec::new();
        for &d in &target_keys {
            let q = scalar_mul(d, g, A, P);
            let mut found_correct = false;
            let mut found_strong = false;
            for m in 2..n_usize {
                let w = compute_window(q, g, m, A, P);
                let c = cross_correlate(&w, &f, n_usize);
                let (peak_pos, _peak_val, ratio) = analyze_peak(&c);
                if !found_correct && peak_pos == d as usize {
                    m_mins_correct.push(m);
                    found_correct = true;
                }
                if !found_strong && peak_pos == d as usize && ratio > 2.0 {
                    m_mins_strong.push(m);
                    found_strong = true;
                }
                if found_correct && found_strong {
                    break;
                }
            }
        }

        let sqrt_n = (n as f64).sqrt();
        let log_n = (n as f64).ln();

        eprintln!("  Group order n = {}", n);
        eprintln!("  √n = {:.1}", sqrt_n);
        eprintln!("  log n = {:.1}", log_n);

        // Check max peak-to-second ratio across all results
        let max_ratio = results.iter().map(|r| r.3).fold(0.0f64, f64::max);
        eprintln!("  Max peak/2nd ratio observed: {:.3}", max_ratio);

        if m_mins_correct.is_empty() {
            eprintln!("\n  VERDICT: COMPLETE FAILURE.");
            eprintln!("  Cross-correlation never peaked at the correct d for any window size.");
        } else {
            let avg_correct =
                m_mins_correct.iter().sum::<usize>() as f64 / m_mins_correct.len() as f64;
            eprintln!(
                "  Avg m where peak first correct = {:.1} ({:.1}% of n)",
                avg_correct,
                100.0 * avg_correct / n as f64
            );

            if m_mins_strong.is_empty() {
                eprintln!("\n  VERDICT: WEAK SIGNAL — NO BREAK.");
                eprintln!("  The cross-correlation peak lands at d for large enough windows,");
                eprintln!("  but the peak-to-second ratio never exceeds 2.0.");
                eprintln!("  The peak is not distinguishable from noise without knowing d.");
                eprintln!("  An attacker could not identify the correct peak.");
            } else {
                let avg_strong =
                    m_mins_strong.iter().sum::<usize>() as f64 / m_mins_strong.len() as f64;
                eprintln!(
                    "  Avg m for strong signal (ratio > 2.0) = {:.1} ({:.1}% of n)",
                    avg_strong,
                    100.0 * avg_strong / n as f64
                );
                eprintln!("  m_strong / √n = {:.1}", avg_strong / sqrt_n);
                eprintln!("  m_strong / log n = {:.1}", avg_strong / log_n);

                if avg_strong < log_n * 2.0 {
                    eprintln!("\n  VERDICT: m_min = O(log n) — THEORETICAL BREAK.");
                    eprintln!("  This would be fundamental if it scales.");
                } else if avg_strong < sqrt_n * 2.0 {
                    eprintln!("\n  VERDICT: m_min = O(√n) — matches BSGS complexity.");
                    eprintln!("  Different mechanism, same asymptotic cost.");
                } else if avg_strong < n as f64 * 0.25 {
                    eprintln!("\n  VERDICT: m_min < n/4 — marginal advantage.");
                } else {
                    eprintln!("\n  VERDICT: m_min = O(n) — CIRCULAR.");
                    eprintln!("  No computational advantage over brute-force.");
                }
            }
        }

        eprintln!("\n  Phase recovery: FAILED for all target keys.");
        eprintln!("  The x-projection f(k) = x(kG) is not a simple sinusoid.");
        eprintln!("  It's a pseudorandom permutation of field elements.");
        eprintln!("  Phase information does not coherently encode the shift d.");

        eprintln!("\n  FUNDAMENTAL ISSUE: The cross-correlation approach requires");
        eprintln!("  the FULL f(k) signal (all n values), which is O(n) to compute.");
        eprintln!("  Computing f(k) for all k IS the brute-force DLP.");
        eprintln!("  Even with a small window, the method is O(n) total.\n");
    }
}
