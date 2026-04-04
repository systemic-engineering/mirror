//! TRAVERSE — Coordinate-order accumulation experiment.
//!
//! Walk curve points in x-coordinate order (not group order).
//! For consecutive points in this ordering, compute the group difference.
//! Analyze the resulting signal via DFT and autocorrelation to detect
//! whether the group-to-coordinate permutation has exploitable structure.

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

    pub fn point_neg(pt: Point, p: u64) -> Point {
        match pt {
            Point::Infinity => Point::Infinity,
            Point::Affine { x, y } => Point::Affine { x, y: (p - y) % p },
        }
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
    use std::collections::HashMap;
    use std::f64::consts::PI;

    /// DFT of a signal f at frequency k: F(k) = sum_j f(j) * exp(-2*pi*i*j*k/n)
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

    /// Power spectrum: |F(k)|^2 for all k
    fn power_spectrum(signal: &[f64]) -> Vec<f64> {
        let n = signal.len();
        (0..n)
            .map(|k| {
                let (re, im) = dft_coefficient(signal, k, n);
                re * re + im * im
            })
            .collect()
    }

    /// Autocorrelation via Wiener-Khinchin: IFFT(|FFT(x)|^2) normalized
    fn autocorrelation(signal: &[f64]) -> Vec<f64> {
        let n = signal.len();
        let ps = power_spectrum(signal);
        // IDFT of power spectrum gives autocorrelation
        let mut ac = vec![0.0f64; n];
        for lag in 0..n {
            let mut val = 0.0f64;
            for k in 0..n {
                let angle = 2.0 * PI * (lag as f64) * (k as f64) / (n as f64);
                val += ps[k] * angle.cos();
            }
            ac[lag] = val / n as f64;
        }
        // Normalize by ac[0]
        let ac0 = ac[0];
        if ac0 > 0.0 {
            for v in ac.iter_mut() {
                *v /= ac0;
            }
        }
        ac
    }

    #[test]
    fn traverse_coordinate_order() {
        let p = 251u64;
        let a = 1u64;
        let b = 1u64;

        eprintln!("\n  ====================================================");
        eprintln!("  TRAVERSE — Coordinate-order accumulation");
        eprintln!("  Curve: y^2 = x^3 + x + 1 (mod 251)");
        eprintln!("  ====================================================\n");

        // ── Step 1: Setup ──
        let all_points = enumerate_curve(a, b, p);
        let n_total = all_points.len();
        eprintln!("  Total curve points (incl. infinity): {}", n_total);

        // Find a generator of maximal order
        let affine_points: Vec<Point> = all_points
            .iter()
            .copied()
            .filter(|p| matches!(p, Point::Affine { .. }))
            .collect();

        // Try each affine point as generator, find one with maximal order
        let mut gen = Point::Infinity;
        let mut group_order = 0u64;
        for &candidate in &affine_points {
            let mut pt = candidate;
            let mut ord = 1u64;
            loop {
                pt = point_add(pt, candidate, a, p);
                ord += 1;
                if pt == Point::Infinity {
                    break;
                }
                if ord > n_total as u64 + 1 {
                    break; // safety
                }
            }
            if ord > group_order {
                group_order = ord;
                gen = candidate;
            }
            // If we found a generator of the full group, stop
            if ord as usize == n_total {
                break;
            }
        }
        eprintln!("  Generator: {:?}", gen);
        eprintln!("  Group order: {}", group_order);

        // ── Build ground truth: point -> group index ──
        // kG for k = 0..group_order-1
        let mut point_to_group_idx: HashMap<Point, u64> = HashMap::new();
        let mut pt = Point::Infinity;
        for k in 0..group_order {
            point_to_group_idx.insert(pt, k);
            pt = point_add(pt, gen, a, p);
        }

        // ── Step 2: Coordinate-ordered point list ──
        // Sort affine points by (x, y). Exclude infinity.
        let mut coord_sorted: Vec<Point> = affine_points.clone();
        coord_sorted.sort_by(|a, b| match (a, b) {
            (Point::Affine { x: x1, y: y1 }, Point::Affine { x: x2, y: y2 }) => {
                x1.cmp(x2).then(y1.cmp(y2))
            }
            _ => std::cmp::Ordering::Equal,
        });

        let n_coord = coord_sorted.len();
        eprintln!("  Affine points in coordinate order: {}", n_coord);

        // Print first few for sanity
        eprintln!("\n  First 10 coordinate-ordered points:");
        for (i, pt) in coord_sorted.iter().take(10).enumerate() {
            if let Point::Affine { x, y } = pt {
                let gidx = point_to_group_idx.get(pt).copied().unwrap_or(u64::MAX);
                eprintln!("    [{}] ({}, {}) -> group index {}", i, x, y, gidx);
            }
        }

        // ── Step 3: Group differences ──
        // For consecutive coordinate-ordered points, compute P_{i+1} - P_i
        let n_diffs = n_coord - 1;
        let mut diff_x_coords: Vec<f64> = Vec::with_capacity(n_diffs);
        let mut group_idx_diffs: Vec<i64> = Vec::with_capacity(n_diffs);
        let mut infinity_count = 0usize;

        for i in 0..n_diffs {
            let p_i = coord_sorted[i];
            let p_next = coord_sorted[i + 1];

            // D_i = P_{i+1} - P_i = P_{i+1} + (-P_i)
            let neg_p_i = point_neg(p_i, p);
            let diff = point_add(p_next, neg_p_i, a, p);

            match diff {
                Point::Affine { x, .. } => {
                    diff_x_coords.push(x as f64);
                }
                Point::Infinity => {
                    // P_{i+1} = P_i means consecutive identical points (shouldn't happen)
                    // or P_{i+1} = -P_i (same x, opposite y)
                    diff_x_coords.push(0.0); // placeholder
                    infinity_count += 1;
                }
            }

            // Ground truth: group index difference
            let gi = point_to_group_idx.get(&p_i).copied().unwrap_or(0) as i64;
            let gi_next = point_to_group_idx.get(&p_next).copied().unwrap_or(0) as i64;
            let mut d = gi_next - gi;
            // Normalize to [-n/2, n/2]
            let n_half = group_order as i64 / 2;
            if d > n_half {
                d -= group_order as i64;
            }
            if d < -n_half {
                d += group_order as i64;
            }
            group_idx_diffs.push(d);
        }

        eprintln!("\n  Difference signal length: {}", n_diffs);
        eprintln!(
            "  Infinity differences (same x, opposite y): {}",
            infinity_count
        );

        // ── Step 4: Statistics on group index differences ──
        let mean_gid = group_idx_diffs.iter().sum::<i64>() as f64 / n_diffs as f64;
        let var_gid = group_idx_diffs
            .iter()
            .map(|&d| {
                let dev = d as f64 - mean_gid;
                dev * dev
            })
            .sum::<f64>()
            / n_diffs as f64;
        let std_gid = var_gid.sqrt();

        // Expected for uniform random permutation differences:
        // Differences mod n mapped to [-n/2, n/2] have variance ~= n^2/12
        // (uniform on {-n/2, ..., n/2} gives E[X^2] = n^2/12)
        let expected_var = (group_order as f64).powi(2) / 12.0;

        eprintln!("\n  === GROUP INDEX DIFFERENCE STATISTICS ===");
        eprintln!("  Mean of group-index diffs:     {:.2}", mean_gid);
        eprintln!("  StdDev of group-index diffs:   {:.2}", std_gid);
        eprintln!("  Variance:                      {:.2}", var_gid);
        eprintln!("  Expected variance (uniform):   {:.2}", expected_var);
        eprintln!(
            "  Ratio var/expected:            {:.4}",
            var_gid / expected_var
        );

        // Distribution: bucket the absolute group index differences
        let mut buckets = vec![0u64; 10];
        let bucket_size = group_order / 10 + 1;
        for &d in &group_idx_diffs {
            let abs_d = d.unsigned_abs();
            let b = (abs_d / bucket_size) as usize;
            if b < 10 {
                buckets[b] += 1;
            }
        }
        eprintln!("\n  Distribution of |group-index diff| (10 buckets):");
        for (i, &count) in buckets.iter().enumerate() {
            let lo = i as u64 * bucket_size;
            let hi = lo + bucket_size - 1;
            let bar: String = std::iter::repeat('#').take(count as usize / 2).collect();
            eprintln!("    [{:3}-{:3}]: {:3} {}", lo, hi, count, bar);
        }

        // ── Step 5: DFT of difference signal ──
        eprintln!("\n  === DFT OF DIFFERENCE SIGNAL (x-coords of D_i) ===");
        let ps = power_spectrum(&diff_x_coords);
        let n_ps = ps.len();

        // DC component
        let dc = ps[0];
        // Average power (excluding DC)
        let avg_power: f64 = ps[1..].iter().sum::<f64>() / (n_ps - 1) as f64;
        let max_power = ps[1..].iter().cloned().fold(0.0f64, f64::max);
        let max_freq = ps[1..]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i + 1)
            .unwrap_or(0);

        let peak_to_avg = max_power / avg_power;

        eprintln!("  DC component:          {:.2}", dc);
        eprintln!("  Avg power (excl DC):   {:.2}", avg_power);
        eprintln!(
            "  Max power (excl DC):   {:.2} at freq {}",
            max_power, max_freq
        );
        eprintln!("  Peak-to-average ratio: {:.4}", peak_to_avg);

        // Top 10 frequencies by power
        let mut freq_power: Vec<(usize, f64)> =
            ps.iter().enumerate().map(|(i, &v)| (i, v)).collect();
        freq_power.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        eprintln!("\n  Top 10 frequencies by power:");
        for (i, (freq, pow)) in freq_power.iter().take(10).enumerate() {
            eprintln!("    [{}] freq={:3}, power={:.2}", i, freq, pow);
        }

        // ── Step 6: Autocorrelation ──
        eprintln!("\n  === AUTOCORRELATION OF DIFFERENCE SIGNAL ===");
        let ac = autocorrelation(&diff_x_coords);

        // Find peaks: ac[lag] > mean + 2*sigma (excluding lag=0)
        let ac_slice = &ac[1..];
        let ac_mean: f64 = ac_slice.iter().sum::<f64>() / ac_slice.len() as f64;
        let ac_var: f64 =
            ac_slice.iter().map(|&v| (v - ac_mean).powi(2)).sum::<f64>() / ac_slice.len() as f64;
        let ac_std = ac_var.sqrt();
        let threshold = ac_mean + 2.0 * ac_std;

        eprintln!("  AC mean (excl lag 0):  {:.6}", ac_mean);
        eprintln!("  AC stddev:             {:.6}", ac_std);
        eprintln!("  Threshold (2-sigma):   {:.6}", threshold);

        let mut peaks: Vec<(usize, f64)> = Vec::new();
        for lag in 1..ac.len() {
            if ac[lag] > threshold {
                peaks.push((lag, ac[lag]));
            }
        }
        peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        eprintln!("  Peaks above 2-sigma threshold: {}", peaks.len());
        for (i, (lag, val)) in peaks.iter().take(20).enumerate() {
            eprintln!("    [{}] lag={:3}, ac={:.6}", i, lag, val);
        }

        // ── Step 7: Random permutation baseline ──
        eprintln!("\n  === RANDOM PERMUTATION BASELINE ===");

        // Use a simple LCG for deterministic "random" permutation
        // Fisher-Yates with LCG seed
        let n_rand = n_coord;
        let mut rand_perm: Vec<u64> = (0..n_rand as u64).collect();

        // LCG: x_{n+1} = (a*x_n + c) mod m
        let mut lcg_state = 12345u64;
        let lcg_a = 1103515245u64;
        let lcg_c = 12345u64;
        let lcg_m = 1u64 << 31;
        for i in (1..n_rand).rev() {
            lcg_state = (lcg_a.wrapping_mul(lcg_state).wrapping_add(lcg_c)) % lcg_m;
            let j = (lcg_state as usize) % (i + 1);
            rand_perm.swap(i, j);
        }

        // Compute differences of the random permutation (same normalization as curve)
        let n_half = group_order as i64 / 2;
        let mut rand_gid_diffs: Vec<i64> = Vec::with_capacity(n_rand - 1);
        let mut rand_diffs: Vec<f64> = Vec::with_capacity(n_rand - 1);
        for i in 0..n_rand - 1 {
            let mut d = rand_perm[i + 1] as i64 - rand_perm[i] as i64;
            if d > n_half {
                d -= group_order as i64;
            }
            if d < -n_half {
                d += group_order as i64;
            }
            rand_gid_diffs.push(d);
            // Use absolute value as x-coordinate proxy for fair comparison
            rand_diffs.push(d.unsigned_abs() as f64);
        }

        // Random permutation variance (for comparison)
        let rand_mean_gid = rand_gid_diffs.iter().sum::<i64>() as f64 / rand_gid_diffs.len() as f64;
        let rand_var_gid = rand_gid_diffs
            .iter()
            .map(|&d| {
                let dev = d as f64 - rand_mean_gid;
                dev * dev
            })
            .sum::<f64>()
            / rand_gid_diffs.len() as f64;
        eprintln!("  Random perm GID variance:      {:.2}", rand_var_gid);
        eprintln!(
            "  Random perm var/expected:       {:.4}",
            rand_var_gid / expected_var
        );

        // DFT of random baseline
        let rand_ps = power_spectrum(&rand_diffs);
        let rand_avg_power: f64 = rand_ps[1..].iter().sum::<f64>() / (rand_ps.len() - 1) as f64;
        let rand_max_power = rand_ps[1..].iter().cloned().fold(0.0f64, f64::max);
        let rand_peak_to_avg = rand_max_power / rand_avg_power;

        eprintln!("  Random baseline:");
        eprintln!("    Avg power (excl DC):   {:.2}", rand_avg_power);
        eprintln!("    Max power (excl DC):   {:.2}", rand_max_power);
        eprintln!("    Peak-to-average ratio: {:.4}", rand_peak_to_avg);

        // Autocorrelation of random baseline
        let rand_ac = autocorrelation(&rand_diffs);
        let rand_ac_slice = &rand_ac[1..];
        let rand_ac_mean: f64 = rand_ac_slice.iter().sum::<f64>() / rand_ac_slice.len() as f64;
        let rand_ac_var: f64 = rand_ac_slice
            .iter()
            .map(|&v| (v - rand_ac_mean).powi(2))
            .sum::<f64>()
            / rand_ac_slice.len() as f64;
        let rand_ac_std = rand_ac_var.sqrt();
        let rand_threshold = rand_ac_mean + 2.0 * rand_ac_std;

        let rand_peaks: Vec<(usize, f64)> = (1..rand_ac.len())
            .filter(|&lag| rand_ac[lag] > rand_threshold)
            .map(|lag| (lag, rand_ac[lag]))
            .collect();

        eprintln!("    AC peaks above 2-sigma: {}", rand_peaks.len());

        // ── Step 8: Also look at group-index difference DFT ──
        eprintln!("\n  === DFT OF GROUP-INDEX DIFFERENCES (ground truth) ===");
        let gid_signal: Vec<f64> = group_idx_diffs.iter().map(|&d| d as f64).collect();
        let gid_ps = power_spectrum(&gid_signal);
        let gid_avg_power: f64 = gid_ps[1..].iter().sum::<f64>() / (gid_ps.len() - 1) as f64;
        let gid_max_power = gid_ps[1..].iter().cloned().fold(0.0f64, f64::max);
        let gid_max_freq = gid_ps[1..]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i + 1)
            .unwrap_or(0);
        let gid_peak_to_avg = gid_max_power / gid_avg_power;

        eprintln!("  Avg power (excl DC):   {:.2}", gid_avg_power);
        eprintln!(
            "  Max power (excl DC):   {:.2} at freq {}",
            gid_max_power, gid_max_freq
        );
        eprintln!("  Peak-to-average ratio: {:.4}", gid_peak_to_avg);

        // Autocorrelation of group-index diffs
        let gid_ac = autocorrelation(&gid_signal);
        let gid_ac_slice = &gid_ac[1..];
        let gid_ac_mean: f64 = gid_ac_slice.iter().sum::<f64>() / gid_ac_slice.len() as f64;
        let gid_ac_var: f64 = gid_ac_slice
            .iter()
            .map(|&v| (v - gid_ac_mean).powi(2))
            .sum::<f64>()
            / gid_ac_slice.len() as f64;
        let gid_ac_std = gid_ac_var.sqrt();
        let gid_threshold = gid_ac_mean + 2.0 * gid_ac_std;

        let mut gid_peaks: Vec<(usize, f64)> = (1..gid_ac.len())
            .filter(|&lag| gid_ac[lag] > gid_threshold)
            .map(|lag| (lag, gid_ac[lag]))
            .collect();
        gid_peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        eprintln!("  AC peaks above 2-sigma: {}", gid_peaks.len());
        for (i, (lag, val)) in gid_peaks.iter().take(10).enumerate() {
            eprintln!("    [{}] lag={:3}, ac={:.6}", i, lag, val);
        }

        // ── Step 9: Comparison and verdict ──
        eprintln!("\n  ====================================================");
        eprintln!("  COMPARISON SUMMARY");
        eprintln!("  ====================================================");
        eprintln!();
        eprintln!("                          Curve     Random   Ratio");
        eprintln!(
            "  DFT peak/avg (x-diff):  {:.4}  {:.4}  {:.4}",
            peak_to_avg,
            rand_peak_to_avg,
            peak_to_avg / rand_peak_to_avg
        );
        eprintln!(
            "  AC peaks (2-sigma):      {:5}     {:5}",
            peaks.len(),
            rand_peaks.len()
        );
        eprintln!(
            "  GID var/expected:        {:.4}  {:.4}",
            var_gid / expected_var,
            rand_var_gid / expected_var
        );
        eprintln!("  GID DFT peak/avg:        {:.4}", gid_peak_to_avg);
        eprintln!();

        // Signal detection criteria
        let dft_signal = peak_to_avg > rand_peak_to_avg * 1.5;
        let ac_signal = peaks.len() > rand_peaks.len() * 2;
        // Compare curve variance to random baseline variance (>15% deviation)
        let gid_var_ratio = var_gid / rand_var_gid;
        let gid_signal_detected = (gid_var_ratio - 1.0).abs() > 0.15;
        eprintln!("  GID var(curve)/var(random):     {:.4}", gid_var_ratio);

        eprintln!(
            "  DFT significantly non-flat vs random?      {}",
            if dft_signal { "YES — SIGNAL" } else { "NO" }
        );
        eprintln!(
            "  Autocorrelation periodic structure?         {}",
            if ac_signal { "YES — SIGNAL" } else { "NO" }
        );
        eprintln!(
            "  Group-index diffs non-uniform?              {}",
            if gid_signal_detected {
                "YES — SIGNAL"
            } else {
                "NO"
            }
        );

        if dft_signal || ac_signal || gid_signal_detected {
            eprintln!("\n  >>> POSITIVE: Structure detected in coordinate-order traversal.");
            eprintln!("  >>> The group-to-coordinate permutation is not fully random.");
        } else {
            eprintln!("\n  >>> NEGATIVE: No significant structure detected.");
            eprintln!("  >>> Coordinate-order traversal looks indistinguishable from random.");
        }

        eprintln!("\n  ====================================================\n");
    }
}
