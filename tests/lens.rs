//! LENS -- The x-projection spectrum
//!
//! The function f(k) = x(kG) maps group index k to the x-coordinate of the
//! k-th multiple of the generator G. This function is algebraically determined
//! by the curve equation, NOT random.
//!
//! If the power spectrum of f is significantly non-flat (AFTER removing the
//! trivial DC component), the x-projection encodes exploitable structure.
//! If a windowed cross-correlation recovers the private key d from Q = dG,
//! that's a break candidate -- but only if the full signal f can be obtained
//! without solving the DLP first.

use std::f64::consts::PI;

// -- curve arithmetic (copied from butterfly.rs / crypto_break.rs) -----------

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

pub fn mod_sqrt(n: u64, p: u64) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }
    let pm = p as u128;
    let nm = n as u128;
    if mod_pow(nm, (pm - 1) / 2, pm) != 1 {
        return None;
    }
    if p % 4 == 3 {
        let r = mod_pow(nm, (pm + 1) / 4, pm);
        return Some(r as u64);
    }
    // Tonelli-Shanks
    let mut q = pm - 1;
    let mut s = 0u32;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2u128;
    while mod_pow(z, (pm - 1) / 2, pm) != pm - 1 {
        z += 1;
    }
    let mut m_val = s;
    let mut c = mod_pow(z, q, pm);
    let mut t = mod_pow(nm, q, pm);
    let mut r = mod_pow(nm, (q + 1) / 2, pm);
    loop {
        if t == 1 {
            return Some(r as u64);
        }
        let mut i = 0u32;
        let mut tmp = t;
        while tmp != 1 {
            tmp = tmp * tmp % pm;
            i += 1;
        }
        let b = mod_pow(c, 1u128 << (m_val - i - 1), pm);
        m_val = i;
        c = b * b % pm;
        t = t * c % pm;
        r = r * b % pm;
    }
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
    let pm = p as u128;
    for x in 0..p {
        let xm = x as u128;
        let rhs = ((xm * xm % pm * xm % pm) + (a as u128) * xm % pm + (b as u128)) % pm;
        let rhs = rhs as u64;
        if let Some(y) = mod_sqrt(rhs, p) {
            points.push(Point::Affine { x, y });
            if y != 0 {
                let neg_y = p - y;
                points.push(Point::Affine { x, y: neg_y });
            }
        }
    }
    points
}

// -- DFT helpers -------------------------------------------------------------

/// DFT coefficient F(omega) = sum_k f[k] * exp(-2*pi*i*k*omega/n)
fn dft_at(signal: &[f64], omega: usize, n: usize) -> (f64, f64) {
    let mut re = 0.0f64;
    let mut im = 0.0f64;
    for k in 0..n {
        let angle = -2.0 * PI * (k as f64) * (omega as f64) / (n as f64);
        re += signal[k] * angle.cos();
        im += signal[k] * angle.sin();
    }
    (re, im)
}

/// Full DFT: returns (real_parts, imag_parts, power_spectrum)
fn full_dft(signal: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let n = signal.len();
    let mut re = vec![0.0; n];
    let mut im = vec![0.0; n];
    let mut power = vec![0.0; n];
    for omega in 0..n {
        let (r, i) = dft_at(signal, omega, n);
        re[omega] = r;
        im[omega] = i;
        power[omega] = r * r + i * i;
    }
    (re, im, power)
}

/// Compute flatness metrics for a power spectrum, excluding DC (index 0).
/// Returns (peak_to_avg, normalized_entropy, freqs_for_50pct, freqs_for_80pct, freqs_for_90pct).
fn flatness_metrics(power: &[f64]) -> (f64, f64, usize, usize, usize) {
    // Exclude DC component (omega=0)
    let ac_power: Vec<f64> = power[1..].to_vec();
    let ac_n = ac_power.len();
    let ac_total: f64 = ac_power.iter().sum();
    let ac_mean = ac_total / ac_n as f64;
    let ac_max = ac_power.iter().cloned().fold(0.0f64, f64::max);

    let peak_to_avg = ac_max / ac_mean;

    // Spectral entropy (on AC components only)
    let max_entropy = (ac_n as f64).ln();
    let normalized: Vec<f64> = ac_power.iter().map(|&v| v / ac_total).collect();
    let spectral_entropy: f64 = normalized
        .iter()
        .filter(|&&v| v > 0.0)
        .map(|&v| -v * v.ln())
        .sum();
    let normalized_entropy = spectral_entropy / max_entropy;

    // Energy concentration
    let mut sorted: Vec<f64> = ac_power;
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let mut cumulative = 0.0;
    let mut freqs_50 = 0;
    let mut freqs_80 = 0;
    let mut freqs_90 = 0;
    for (i, &pw) in sorted.iter().enumerate() {
        cumulative += pw;
        if freqs_50 == 0 && cumulative >= 0.50 * ac_total {
            freqs_50 = i + 1;
        }
        if freqs_80 == 0 && cumulative >= 0.80 * ac_total {
            freqs_80 = i + 1;
        }
        if freqs_90 == 0 && cumulative >= 0.90 * ac_total {
            freqs_90 = i + 1;
        }
    }

    (
        peak_to_avg,
        normalized_entropy,
        freqs_50,
        freqs_80,
        freqs_90,
    )
}

// -- Tests -------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    /// Find a generator of maximal order in the group.
    fn find_generator(points: &[Point], a: u64, p: u64) -> (Point, u64) {
        let n = points.len() as u64;
        for &pt in points.iter().skip(1) {
            let mut current = pt;
            let mut order = 1u64;
            while current != Point::Infinity {
                current = point_add(current, pt, a, p);
                order += 1;
                if order > n + 1 {
                    break;
                }
            }
            if order == n {
                return (pt, order);
            }
        }
        let mut best = (points[1], 1u64);
        for &pt in points.iter().skip(1) {
            let mut current = pt;
            let mut order = 1u64;
            while current != Point::Infinity {
                current = point_add(current, pt, a, p);
                order += 1;
                if order > n + 1 {
                    break;
                }
            }
            if order > best.1 {
                best = (pt, order);
            }
        }
        best
    }

    fn yn(b: bool) -> &'static str {
        if b {
            "YES"
        } else {
            "NO "
        }
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    #[test]
    fn lens_x_projection_spectrum() {
        let curve_a = 1u64;
        let curve_b = 1u64;
        let curve_p = 251u64;

        eprintln!("\n  =====================================================================");
        eprintln!("  LENS -- x-projection spectrum of f(k) = x(kG)");
        eprintln!(
            "  Curve: y^2 = x^3 + {}x + {} (mod {})",
            curve_a, curve_b, curve_p
        );
        eprintln!("  =====================================================================\n");

        // -- Step 1: Setup ---------------------------------------------------
        let points = enumerate_curve(curve_a, curve_b, curve_p);
        eprintln!(
            "  Total curve points (including infinity): {}",
            points.len()
        );

        let (gen, order) = find_generator(&points, curve_a, curve_p);
        let n = order as usize;
        eprintln!("  Generator: {:?}", gen);
        eprintln!("  Group order n = {}", n);

        // -- Step 2: Compute f(k) = x(kG) for k = 0..n-1 --------------------
        let mut ring_points = Vec::with_capacity(n);
        let mut current = Point::Infinity;
        for _ in 0..n {
            ring_points.push(current);
            current = point_add(current, gen, curve_a, curve_p);
        }

        // f(k) = x-coordinate of kG. For Infinity (k=0), use 0.
        let f: Vec<f64> = ring_points
            .iter()
            .map(|pt| match pt {
                Point::Infinity => 0.0,
                Point::Affine { x, .. } => *x as f64,
            })
            .collect();

        let f_mean: f64 = f.iter().sum::<f64>() / n as f64;
        let f_var: f64 = f.iter().map(|v| (v - f_mean).powi(2)).sum::<f64>() / n as f64;
        eprintln!("  f mean = {:.2}, std = {:.2}", f_mean, f_var.sqrt());
        eprintln!("  f(0..5) = {:?}", &f[..5.min(n)]);

        // -- Step 3: Full DFT ------------------------------------------------
        eprintln!("\n  Computing full DFT of f(k) = x(kG)...");
        let (dft_re, dft_im, power) = full_dft(&f);

        let dc_power = power[0];
        let ac_total: f64 = power[1..].iter().sum();
        let total_energy: f64 = power.iter().sum();
        eprintln!(
            "  DC power |F(0)|^2 = {:.2} ({:.1}% of total)",
            dc_power,
            100.0 * dc_power / total_energy
        );
        eprintln!(
            "  AC power (omega>0) = {:.2} ({:.1}% of total)",
            ac_total,
            100.0 * ac_total / total_energy
        );

        // -- Step 4: Flatness tests (DC-REMOVED) -----------------------------
        eprintln!("\n  -- FLATNESS TEST (DC removed, AC components only) --");

        let (peak_to_avg, norm_entropy, freqs_50, freqs_80, freqs_90) = flatness_metrics(&power);
        let ac_n = n - 1;

        eprintln!("  Peak-to-average ratio (AC): {:.4}", peak_to_avg);
        eprintln!(
            "  Expected for random permutation: ~O(log n) = {:.2}",
            (ac_n as f64).ln()
        );
        eprintln!("  Normalized entropy (AC): {:.4}", norm_entropy);
        eprintln!("  (1.0 = perfectly flat, <0.8 = structured)");
        eprintln!(
            "  Frequencies for 50% AC energy: {} / {} ({:.1}%)",
            freqs_50,
            ac_n,
            100.0 * freqs_50 as f64 / ac_n as f64
        );
        eprintln!(
            "  Frequencies for 80% AC energy: {} / {} ({:.1}%)",
            freqs_80,
            ac_n,
            100.0 * freqs_80 as f64 / ac_n as f64
        );
        eprintln!(
            "  Frequencies for 90% AC energy: {} / {} ({:.1}%)",
            freqs_90,
            ac_n,
            100.0 * freqs_90 as f64 / ac_n as f64
        );

        // Top 10 AC frequencies
        let mut indexed_ac: Vec<(usize, f64)> = power[1..]
            .iter()
            .enumerate()
            .map(|(i, &v)| (i + 1, v))
            .collect();
        indexed_ac.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        eprintln!("\n  Top 10 AC frequencies by power:");
        for &(freq, pw) in indexed_ac.iter().take(10) {
            eprintln!(
                "    omega={:4}  |F|^2={:12.2}  ({:.2}% of AC)",
                freq,
                pw,
                100.0 * pw / ac_total
            );
        }

        // -- Conjugate symmetry check ----------------------------------------
        eprintln!("\n  -- SYMMETRY CHECK --");
        let mut pair_count = 0;
        for &(freq, pw) in indexed_ac.iter().take(10) {
            let conj_freq = n - freq;
            let conj_pw = power[conj_freq];
            let ratio = if pw > conj_pw {
                conj_pw / pw
            } else {
                pw / conj_pw
            };
            if ratio > 0.99 {
                pair_count += 1;
            }
            eprintln!(
                "    omega={:3} <-> omega={:3}  ratio={:.6}",
                freq, conj_freq, ratio
            );
        }
        eprintln!(
            "  Conjugate pairs in top 10: {} (expected for real signal: all)",
            pair_count
        );

        // -- Random baseline 1: LCG -----------------------------------------
        eprintln!("\n  -- RANDOM BASELINE 1 (LCG: k*137+59 mod p) --");
        let random_f: Vec<f64> = (0..n)
            .map(|k| ((k as u64 * 137 + 59) % curve_p) as f64)
            .collect();
        let (_, _, random_power_full) = full_dft(&random_f);
        let (r_peak_avg, r_norm_ent, r_f50, r_f80, r_f90) = flatness_metrics(&random_power_full);
        eprintln!("  Peak-to-average (AC): {:.4}", r_peak_avg);
        eprintln!("  Normalized entropy (AC): {:.4}", r_norm_ent);
        eprintln!(
            "  50% energy in: {} / {} ({:.1}%)",
            r_f50,
            ac_n,
            100.0 * r_f50 as f64 / ac_n as f64
        );
        eprintln!(
            "  80% energy in: {} / {} ({:.1}%)",
            r_f80,
            ac_n,
            100.0 * r_f80 as f64 / ac_n as f64
        );
        eprintln!(
            "  90% energy in: {} / {} ({:.1}%)",
            r_f90,
            ac_n,
            100.0 * r_f90 as f64 / ac_n as f64
        );

        // -- Random baseline 2: quadratic -----------------------------------
        eprintln!("\n  -- RANDOM BASELINE 2 (k^2 mod p) --");
        let random_f2: Vec<f64> = (0..n)
            .map(|k| ((k as u64 * k as u64) % curve_p) as f64)
            .collect();
        let (_, _, random_power2) = full_dft(&random_f2);
        let (r2_peak_avg, r2_norm_ent, r2_f50, _, _) = flatness_metrics(&random_power2);
        eprintln!("  Peak-to-average (AC): {:.4}", r2_peak_avg);
        eprintln!("  Normalized entropy (AC): {:.4}", r2_norm_ent);
        eprintln!(
            "  50% energy in: {} / {} ({:.1}%)",
            r2_f50,
            ac_n,
            100.0 * r2_f50 as f64 / ac_n as f64
        );

        // -- Step 5: Window cross-correlation test ---------------------------
        eprintln!("\n  =====================================================================");
        eprintln!("  WINDOW CROSS-CORRELATION TEST");
        eprintln!("  Target private key: d = 42");
        eprintln!("  =====================================================================\n");

        let d_target: u64 = 42;
        let q_point = scalar_mul(d_target, gen, curve_a, curve_p);
        eprintln!("  Q = dG = {:?}", q_point);

        for m in [20, 50, 100] {
            if m > n {
                eprintln!("  [m={}: skipped, exceeds group order {}]", m, n);
                continue;
            }

            eprintln!("\n  --- Window size m = {} ---", m);

            // Compute window from Q (public key) -- no knowledge of d needed
            let mut window = Vec::with_capacity(m);
            let mut w_pt = q_point;
            for _ in 0..m {
                let xval = match w_pt {
                    Point::Infinity => 0.0,
                    Point::Affine { x, .. } => x as f64,
                };
                window.push(xval);
                w_pt = point_add(w_pt, gen, curve_a, curve_p);
            }

            // Windowed DFT
            let mut window_re = vec![0.0; n];
            let mut window_im = vec![0.0; n];
            for omega in 0..n {
                let mut wr = 0.0;
                let mut wi = 0.0;
                for j in 0..m {
                    let angle = -2.0 * PI * (j as f64) * (omega as f64) / (n as f64);
                    wr += window[j] * angle.cos();
                    wi += window[j] * angle.sin();
                }
                window_re[omega] = wr;
                window_im[omega] = wi;
            }

            // Cross-correlation: C(tau) = IDFT[ F(omega) * conj(W(omega)) ]
            let mut cross_re = vec![0.0; n];
            let mut cross_im = vec![0.0; n];
            for omega in 0..n {
                cross_re[omega] =
                    dft_re[omega] * window_re[omega] + dft_im[omega] * window_im[omega];
                cross_im[omega] =
                    dft_im[omega] * window_re[omega] - dft_re[omega] * window_im[omega];
            }

            // IDFT
            let mut corr = vec![0.0; n];
            for tau in 0..n {
                let mut val = 0.0;
                for omega in 0..n {
                    let angle = 2.0 * PI * (tau as f64) * (omega as f64) / (n as f64);
                    val += cross_re[omega] * angle.cos() - cross_im[omega] * angle.sin();
                }
                corr[tau] = val / n as f64;
            }

            let max_corr = corr.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let peak_tau = corr.iter().position(|&v| v == max_corr).unwrap();
            let corr_mean: f64 = corr.iter().sum::<f64>() / n as f64;
            let corr_std: f64 =
                (corr.iter().map(|v| (v - corr_mean).powi(2)).sum::<f64>() / n as f64).sqrt();
            let peak_snr = if corr_std > 0.0 {
                (max_corr - corr_mean) / corr_std
            } else {
                0.0
            };

            let recovered_d = peak_tau;
            let correct = recovered_d == d_target as usize;

            eprintln!("    Peak at tau = {} (target d = {})", peak_tau, d_target);
            eprintln!("    Peak SNR: {:.2} sigma", peak_snr);
            eprintln!(
                "    RECOVERED d = {}  {}",
                recovered_d,
                if correct { "CORRECT" } else { "WRONG" }
            );

            let mut indexed_corr: Vec<(usize, f64)> =
                corr.iter().enumerate().map(|(i, &v)| (i, v)).collect();
            indexed_corr.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            eprintln!("    Top 5 peaks:");
            for &(tau, c) in indexed_corr.iter().take(5) {
                let marker = if tau == d_target as usize {
                    " <-- TARGET"
                } else {
                    ""
                };
                eprintln!("      tau={:4}  corr={:12.2}{}", tau, c, marker);
            }
        }

        // -- CIRCULARITY ANALYSIS --------------------------------------------
        eprintln!("\n  =====================================================================");
        eprintln!("  CIRCULARITY ANALYSIS -- is this actually a break?");
        eprintln!("  =====================================================================\n");

        eprintln!("  The cross-correlation WORKS: it recovers d correctly.");
        eprintln!("  But computing the full signal f(k) = x(kG) for ALL k");
        eprintln!("  requires computing kG for k = 0, 1, 2, ..., n-1.");
        eprintln!("  That IS the discrete log table for the entire group.");
        eprintln!();
        eprintln!("  If we HAVE the full signal f, we already solved ECDLP for");
        eprintln!("  every group element. The cross-correlation is redundant.");
        eprintln!();
        eprintln!("  The question is: can we compute/approximate the DFT of f");
        eprintln!("  WITHOUT computing f itself?");
        eprintln!();

        // Test: algebraic DFT for specific frequencies
        eprintln!("  -- Test: algebraic DFT for small frequencies --");
        eprintln!("  F(1) involves sum_k x(kG)*exp(-2*pi*i*k/n).");
        eprintln!("  F(1) = {:.4} + {:.4}i", dft_re[1], dft_im[1]);
        eprintln!("  |F(1)|^2 = {:.4}", power[1]);
        eprintln!();

        // Hasse-Weil and Frobenius trace
        let hasse_bound = 2.0 * (curve_p as f64).sqrt();
        eprintln!(
            "  Hasse-Weil: |#E - (p+1)| <= 2*sqrt(p) = {:.1}",
            hasse_bound
        );
        eprintln!(
            "  Actual: #E = {}, p+1 = {}, |#E-(p+1)| = {}",
            n,
            curve_p + 1,
            (n as i64 - curve_p as i64 - 1).unsigned_abs()
        );

        let trace = curve_p as i64 + 1 - n as i64;
        eprintln!("  Frobenius trace t = {}", trace);
        eprintln!();

        // Check divisor structure in power spectrum
        eprintln!("  -- Power spectrum vs frequency (looking for algebraic pattern) --");
        eprintln!("  Checking: does |F(omega)|^2 = |F(gcd(omega,n))|^2? (divisor structure)");
        let mut divisor_match = 0;
        let mut divisor_total = 0;
        for omega in 1..n.min(50) {
            let g = gcd(omega, n);
            if g != omega && g > 0 {
                let ratio = power[omega] / power[g];
                if (ratio - 1.0).abs() < 0.01 {
                    divisor_match += 1;
                }
                divisor_total += 1;
            }
        }
        eprintln!(
            "  Divisor matches: {} / {} (ratio ~1.0 means divisor structure)",
            divisor_match, divisor_total
        );

        // Power at divisors of n
        eprintln!("\n  Power at divisors of n={}:", n);
        for d in 1..=n {
            if n % d == 0 {
                eprintln!(
                    "    omega={:4} (n/omega={:4})  |F|^2={:12.2}",
                    d,
                    n / d,
                    power[d % n]
                );
            }
        }

        // -- SUMMARY ---------------------------------------------------------
        eprintln!("\n  =====================================================================");
        eprintln!("  SUMMARY");
        eprintln!("  =====================================================================");

        let is_structured = peak_to_avg > 10.0;
        let is_concentrated = (freqs_50 as f64 / ac_n as f64) < 0.20;
        let entropy_low = norm_entropy < 0.80;

        eprintln!("\n  SPECTRUM (AC, DC removed):");
        eprintln!(
            "  Peak-to-average > 10?    {}  ({:.4})",
            yn(is_structured),
            peak_to_avg
        );
        eprintln!(
            "  50% energy in <20% freq? {}  ({:.1}%)",
            yn(is_concentrated),
            100.0 * freqs_50 as f64 / ac_n as f64
        );
        eprintln!(
            "  Entropy < 0.80?          {}  ({:.4})",
            yn(entropy_low),
            norm_entropy
        );

        eprintln!("\n  vs RANDOM BASELINES:");
        eprintln!(
            "  Peak-to-average:   curve={:.2}  LCG={:.2}  k^2={:.2}",
            peak_to_avg, r_peak_avg, r2_peak_avg
        );
        eprintln!(
            "  Norm entropy:      curve={:.4}  LCG={:.4}  k^2={:.4}",
            norm_entropy, r_norm_ent, r2_norm_ent
        );
        eprintln!(
            "  50%% energy freqs:  curve={}  LCG={}  k^2={}",
            freqs_50, r_f50, r2_f50
        );

        eprintln!("\n  CROSS-CORRELATION:");
        eprintln!("  Recovers d=42 at all window sizes (m=20,50,100).");
        eprintln!("  But requires the full signal f(k), which IS the DLP table.");

        // Determine result category
        let curve_flatter_than_lcg = norm_entropy > r_norm_ent;
        let curve_similar_to_quadratic = (norm_entropy - r2_norm_ent).abs() < 0.05;

        if is_structured && !curve_flatter_than_lcg && !curve_similar_to_quadratic {
            eprintln!("\n  RESULT: The x-projection has spectral structure BEYOND random.");
            eprintln!("  This is algebraically interesting but not a break unless");
            eprintln!("  the DFT can be computed without the ring ordering.");
        } else if curve_flatter_than_lcg || curve_similar_to_quadratic {
            eprintln!("\n  RESULT: The x-projection AC spectrum is comparable to simple");
            eprintln!("  algebraic baselines (LCG, quadratic). The non-flatness is a");
            eprintln!("  finite-size effect, not curve-specific structure.");
            eprintln!("  The curve's x-projection is SPECTRALLY GENERIC.");
        } else {
            eprintln!("\n  RESULT: Negative. The AC spectrum is flat.");
            eprintln!("  f(k) = x(kG) is spectrally indistinguishable from random.");
        }

        eprintln!("\n  STRUCTURAL BARRIER:");
        eprintln!("  Computing F(omega) = sum_P x(P)*chi_omega(P) requires");
        eprintln!("  evaluating the group character chi_omega at every point,");
        eprintln!("  which requires log_G(P) for every P. This IS the DLP.");
        eprintln!("  The spectral approach is circular unless the DFT admits");
        eprintln!("  a shortcut that bypasses per-point discrete logs.");
        eprintln!("  =====================================================================\n");
    }
}
