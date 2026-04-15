//! Pathfinder: behavioral fingerprinting for DLP discrimination.
//!
//! Tests whether the spectral loop's BEHAVIORAL FINGERPRINT differs near d_true
//! vs. far from d_true — even when the loop doesn't converge to d.
//!
//! The hypothesis: the Abyss loop (circular-reflexive iteration over spectral
//! views + signature constraints) might behave DIFFERENTLY when seeded near d_true.
//! Not converge to d — just behave anomalously. The Pathfinder watches the
//! behavior and ranks candidates by anomaly.
//!
//! Even 1 bit of discrimination (51% vs 50% accuracy) compounds across scales:
//! log2(n) scales x 1 bit = d.

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

    /// ECDSA signature (r, s) with message hash h.
    struct Signature {
        r: u64,
        s: u64,
        h: u64,
    }

    /// Behavioral fingerprint for a candidate private key.
    struct Fingerprint {
        nonce_spread: f64,       // Feature 1: variance of |x(k_i*G) - r_i|
        spectral_coherence: f64, // Feature 2: DFT power concentration
        cross_correlation: f64,  // Feature 3: pairwise product structure
        x_projection_peak: f64,  // Feature 4: circular cross-correlation peak
        cluster_spread: f64,     // Feature 5: group-theoretic clustering
    }

    /// Generate deterministic ECDSA signatures.
    fn generate_signatures(
        g: Point,
        d: u64,
        n: u64,
        a_curve: u64,
        p: u64,
        count: usize,
    ) -> Vec<Signature> {
        let mut sigs = Vec::new();
        for i in 1..=count {
            // Deterministic nonce
            let k = ((7 * i * i + 13 * i + 37) % (n as usize - 1) + 1) as u64;
            let r_point = scalar_mul(k, g, a_curve, p);
            let r = match r_point {
                Point::Affine { x, .. } => x % n,
                Point::Infinity => continue,
            };
            if r == 0 {
                continue;
            }
            // Message hash: simple deterministic
            let h = ((i as u64 * 31 + 17) % n) as u64;
            // s = k^{-1} * (h + r*d) mod n
            let k_inv = match mod_inv(k, n) {
                Some(ki) => ki,
                None => continue,
            };
            let s = (k_inv as u128 * ((h as u128 + r as u128 * d as u128) % n as u128)) % n as u128;
            let s = s as u64;
            if s == 0 {
                continue;
            }
            sigs.push(Signature { r, s, h });
        }
        sigs
    }

    /// Compute the nonces implied by candidate d_c for given signatures.
    fn implied_nonces(d_c: u64, sigs: &[Signature], n: u64) -> Vec<u64> {
        sigs.iter()
            .map(|sig| {
                let s_inv = mod_inv(sig.s, n).unwrap_or(0);
                let k_c = (s_inv as u128
                    * ((sig.h as u128 + sig.r as u128 * d_c as u128) % n as u128))
                    % n as u128;
                k_c as u64
            })
            .collect()
    }

    /// Feature 1: Nonce consistency spread.
    /// Variance of |x(k_i*G) - r_i| across signatures.
    fn nonce_spread(
        d_c: u64,
        sigs: &[Signature],
        g: Point,
        n: u64,
        a_curve: u64,
        p: u64,
        ops: &mut u64,
    ) -> f64 {
        let nonces = implied_nonces(d_c, sigs, n);
        let residuals: Vec<f64> = nonces
            .iter()
            .zip(sigs.iter())
            .map(|(&k_c, sig)| {
                let r_point = scalar_mul(k_c, g, a_curve, p);
                *ops += 1; // scalar_mul
                let x_val = match r_point {
                    Point::Affine { x, .. } => x,
                    Point::Infinity => 0,
                };
                let diff = if x_val % n >= sig.r {
                    (x_val % n - sig.r) as f64
                } else {
                    (sig.r - x_val % n) as f64
                };
                // Wrap-around: take min of direct and wrapped distance
                let wrap_diff = (n as f64) - diff;
                diff.min(wrap_diff)
            })
            .collect();

        let mean = residuals.iter().sum::<f64>() / residuals.len() as f64;
        let variance =
            residuals.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / residuals.len() as f64;
        variance
    }

    /// Feature 2: Spectral coherence of implied nonce sequence.
    /// How concentrated the DFT power spectrum is (lower = more concentrated).
    fn spectral_coherence(d_c: u64, sigs: &[Signature], n: u64) -> f64 {
        let nonces = implied_nonces(d_c, sigs, n);
        let signal: Vec<f64> = nonces.iter().map(|&k| k as f64).collect();
        let len = signal.len();

        // Compute DFT magnitudes
        let mut magnitudes = Vec::new();
        for freq in 0..len {
            let mut re = 0.0f64;
            let mut im = 0.0f64;
            for (j, &val) in signal.iter().enumerate() {
                let angle = -2.0 * PI * j as f64 * freq as f64 / len as f64;
                re += val * angle.cos();
                im += val * angle.sin();
            }
            magnitudes.push((re * re + im * im).sqrt());
        }

        // Spectral coherence: ratio of max magnitude to total magnitude
        let total: f64 = magnitudes.iter().sum();
        let max_mag = magnitudes.iter().cloned().fold(0.0f64, |a, b| a.max(b));
        if total > 0.0 {
            max_mag / total
        } else {
            0.0
        }
    }

    /// Feature 3: Cross-signature correlation.
    /// Variance of pairwise nonce products (lower = more structured).
    fn cross_correlation(d_c: u64, sigs: &[Signature], n: u64) -> f64 {
        let nonces = implied_nonces(d_c, sigs, n);
        let mut products = Vec::new();
        for i in 0..nonces.len() {
            for j in (i + 1)..nonces.len() {
                let prod = (nonces[i] as u128 * nonces[j] as u128 % n as u128) as f64;
                products.push(prod);
            }
        }
        let mean = products.iter().sum::<f64>() / products.len() as f64;
        let variance =
            products.iter().map(|p| (p - mean).powi(2)).sum::<f64>() / products.len() as f64;
        variance
    }

    /// Feature 4: x-projection alignment.
    /// Circular cross-correlation peak between x(k_i*G) and r_i values.
    fn x_projection_peak(
        d_c: u64,
        sigs: &[Signature],
        g: Point,
        n: u64,
        a_curve: u64,
        p: u64,
        ops: &mut u64,
    ) -> f64 {
        let nonces = implied_nonces(d_c, sigs, n);
        let x_vals: Vec<f64> = nonces
            .iter()
            .map(|&k_c| {
                let r_point = scalar_mul(k_c, g, a_curve, p);
                *ops += 1;
                match r_point {
                    Point::Affine { x, .. } => (x % n) as f64,
                    Point::Infinity => 0.0,
                }
            })
            .collect();
        let r_vals: Vec<f64> = sigs.iter().map(|s| s.r as f64).collect();
        let len = x_vals.len();

        // Circular cross-correlation
        let mut max_corr = f64::NEG_INFINITY;
        for shift in 0..len {
            let mut corr = 0.0;
            for i in 0..len {
                corr += x_vals[(i + shift) % len] * r_vals[i];
            }
            if corr > max_corr {
                max_corr = corr;
            }
        }
        // Normalize
        let x_norm: f64 = x_vals.iter().map(|v| v * v).sum::<f64>().sqrt();
        let r_norm: f64 = r_vals.iter().map(|v| v * v).sum::<f64>().sqrt();
        if x_norm > 0.0 && r_norm > 0.0 {
            max_corr / (x_norm * r_norm)
        } else {
            0.0
        }
    }

    /// Feature 5: Group-theoretic clustering.
    /// Spread of the 20 nonce points on the curve (variance of distances from centroid).
    fn cluster_spread(
        d_c: u64,
        sigs: &[Signature],
        g: Point,
        n: u64,
        a_curve: u64,
        p: u64,
        ops: &mut u64,
    ) -> f64 {
        let nonces = implied_nonces(d_c, sigs, n);
        let points: Vec<(f64, f64)> = nonces
            .iter()
            .map(|&k_c| {
                let pt = scalar_mul(k_c, g, a_curve, p);
                *ops += 1;
                match pt {
                    Point::Affine { x, y } => (x as f64, y as f64),
                    Point::Infinity => (0.0, 0.0),
                }
            })
            .collect();

        let cx = points.iter().map(|(x, _)| x).sum::<f64>() / points.len() as f64;
        let cy = points.iter().map(|(_, y)| y).sum::<f64>() / points.len() as f64;

        let variance = points
            .iter()
            .map(|(x, y)| {
                let dx = x - cx;
                let dy = y - cy;
                dx * dx + dy * dy
            })
            .sum::<f64>()
            / points.len() as f64;
        variance
    }

    /// Compute full behavioral fingerprint for a candidate.
    fn compute_fingerprint(
        d_c: u64,
        sigs: &[Signature],
        g: Point,
        n: u64,
        a_curve: u64,
        p: u64,
        ops: &mut u64,
    ) -> Fingerprint {
        Fingerprint {
            nonce_spread: nonce_spread(d_c, sigs, g, n, a_curve, p, ops),
            spectral_coherence: spectral_coherence(d_c, sigs, n),
            cross_correlation: cross_correlation(d_c, sigs, n),
            x_projection_peak: x_projection_peak(d_c, sigs, g, n, a_curve, p, ops),
            cluster_spread: cluster_spread(d_c, sigs, g, n, a_curve, p, ops),
        }
    }

    /// Find the order of a point on the curve.
    fn point_order(pt: Point, a_curve: u64, p: u64, max: u64) -> u64 {
        let mut acc = pt;
        for i in 1..=max {
            if acc == Point::Infinity {
                return i;
            }
            acc = point_add(acc, pt, a_curve, p);
        }
        max + 1 // didn't find order
    }

    #[test]
    fn pathfinder_behavioral_discrimination() {
        eprintln!("\n=== PATHFINDER: Behavioral Fingerprinting for DLP ===\n");

        // Setup: curve y^2 = x^3 + x + 1 (mod 251)
        let p = 251u64;
        let a_curve = 1u64;
        let _b_curve = 1u64;

        // Find generator and group order
        let points = enumerate_curve(a_curve, 1, p);
        let n_total = points.len() as u64; // 282 = 2 * 3 * 47
        eprintln!("  Curve: y^2 = x^3 + x + 1 (mod {})", p);
        eprintln!("  Total curve points (including infinity): {}", n_total);

        // ECDSA requires a prime-order subgroup. 282 = 2 * 3 * 47.
        // We need the subgroup of order 47 (the largest prime factor).
        // Generator for order-47 subgroup: cofactor = 282/47 = 6.
        // G_47 = 6 * G_full (any point of full order, multiplied by cofactor).
        let cofactor = n_total / 47;
        assert_eq!(cofactor, 6, "282 / 47 should be 6");

        // Find a non-trivial point and multiply by cofactor
        let g_full = points[1]; // order 282
        let g = scalar_mul(cofactor, g_full, a_curve, p);
        assert_ne!(g, Point::Infinity, "cofactor * G should not be infinity");

        // Verify order is 47
        let n = point_order(g, a_curve, p, n_total);
        assert_eq!(n, 47, "subgroup generator should have order 47");
        eprintln!("  Generator G = {:?} (order-{} subgroup)", g, n);
        eprintln!("  Group order n = {} (prime)", n);

        // Private key
        let d_true = 42u64;
        let q = scalar_mul(d_true, g, a_curve, p);
        eprintln!("  Private key d = {}", d_true);
        eprintln!("  Public key Q = {:?}", q);

        // Generate signatures
        let sigs = generate_signatures(g, d_true, n, a_curve, p, 20);
        eprintln!("  Generated {} valid ECDSA signatures", sigs.len());

        // Verify signatures at d_true
        let nonces_true = implied_nonces(d_true, &sigs, n);
        let mut all_match = true;
        for (i, (&k_c, sig)) in nonces_true.iter().zip(sigs.iter()).enumerate() {
            let rp = scalar_mul(k_c, g, a_curve, p);
            let rx = match rp {
                Point::Affine { x, .. } => x % n,
                Point::Infinity => 0,
            };
            if rx != sig.r {
                eprintln!(
                    "  WARNING: sig {} mismatch: x(k*G)%n={} vs r={}",
                    i, rx, sig.r
                );
                all_match = false;
            }
        }
        if all_match {
            eprintln!("  Verification: all signatures match at d_true [OK]");
        }

        // ============================================================
        // Step 2-3: Compute fingerprints for ALL candidates
        // ============================================================
        eprintln!("\n--- Step 2-3: Computing behavioral fingerprints for all candidates ---");

        let mut total_ops = 0u64;
        let mut all_fingerprints: Vec<(u64, Fingerprint)> = Vec::new();

        for d_c in 0..n {
            let fp = compute_fingerprint(d_c, &sigs, g, n, a_curve, p, &mut total_ops);
            all_fingerprints.push((d_c, fp));
        }

        // Compute feature statistics (excluding d_true)
        let others: Vec<&Fingerprint> = all_fingerprints
            .iter()
            .filter(|(d, _)| *d != d_true)
            .map(|(_, fp)| fp)
            .collect();

        let fp_true = &all_fingerprints[d_true as usize].1;

        // Feature statistics
        struct FeatureStats {
            name: &'static str,
            true_val: f64,
            mean: f64,
            std: f64,
            z_score: f64,
        }

        let features: Vec<Box<dyn Fn(&Fingerprint) -> f64>> = vec![
            Box::new(|fp: &Fingerprint| fp.nonce_spread),
            Box::new(|fp: &Fingerprint| fp.spectral_coherence),
            Box::new(|fp: &Fingerprint| fp.cross_correlation),
            Box::new(|fp: &Fingerprint| fp.x_projection_peak),
            Box::new(|fp: &Fingerprint| fp.cluster_spread),
        ];
        let feature_names = [
            "nonce_spread",
            "spectral_coherence",
            "cross_correlation",
            "x_projection_peak",
            "cluster_spread",
        ];

        let mut stats = Vec::new();
        for (i, f) in features.iter().enumerate() {
            let true_val = f(fp_true);
            let vals: Vec<f64> = others.iter().map(|fp| f(fp)).collect();
            let mean = vals.iter().sum::<f64>() / vals.len() as f64;
            let variance = vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / vals.len() as f64;
            let std = variance.sqrt();
            let z_score = if std > 0.0 {
                (true_val - mean) / std
            } else {
                0.0
            };
            stats.push(FeatureStats {
                name: feature_names[i],
                true_val,
                mean,
                std,
                z_score,
            });
        }

        eprintln!("\n  === Feature Values at d_true vs. Population ===");
        eprintln!(
            "  {:25} {:>15} {:>15} {:>10} {:>10}",
            "Feature", "d_true", "mean", "std", "z-score"
        );
        eprintln!("  {}", "-".repeat(80));
        let mut _any_signal = false;
        for s in &stats {
            let marker = if s.z_score.abs() > 2.0 {
                _any_signal = true;
                " ***"
            } else if s.z_score.abs() > 1.0 {
                " *"
            } else {
                ""
            };
            eprintln!(
                "  {:25} {:>15.4} {:>15.4} {:>10.4} {:>10.4}{}",
                s.name, s.true_val, s.mean, s.std, s.z_score, marker
            );
        }

        // ============================================================
        // Step 3: Neighborhood Analysis
        // ============================================================
        eprintln!("\n--- Step 3: Neighborhood Analysis (d_true +/- 10) ---");

        let neighborhood: Vec<u64> =
            (d_true.saturating_sub(10)..=std::cmp::min(d_true + 10, n - 1)).collect();
        let far_samples: Vec<u64> = {
            let mut samples = Vec::new();
            // Pick 20 candidates far from d_true
            let step = n / 22;
            for i in 1..=20 {
                let d_c = ((d_true + step * i as u64 + n / 3) % n) as u64;
                if !neighborhood.contains(&d_c) {
                    samples.push(d_c);
                }
            }
            samples.truncate(20);
            samples
        };

        for (feat_idx, f) in features.iter().enumerate() {
            let near_vals: Vec<f64> = neighborhood
                .iter()
                .filter(|&&d| d != d_true)
                .map(|&d| f(&all_fingerprints[d as usize].1))
                .collect();
            let far_vals: Vec<f64> = far_samples
                .iter()
                .map(|&d| f(&all_fingerprints[d as usize].1))
                .collect();

            let near_mean = near_vals.iter().sum::<f64>() / near_vals.len().max(1) as f64;
            let far_mean = far_vals.iter().sum::<f64>() / far_vals.len().max(1) as f64;
            let near_var = near_vals
                .iter()
                .map(|v| (v - near_mean).powi(2))
                .sum::<f64>()
                / near_vals.len().max(1) as f64;
            let far_var = far_vals.iter().map(|v| (v - far_mean).powi(2)).sum::<f64>()
                / far_vals.len().max(1) as f64;

            let pooled_std = ((near_var + far_var) / 2.0).sqrt();
            let diff_z = if pooled_std > 0.0 {
                (near_mean - far_mean) / pooled_std
            } else {
                0.0
            };

            let marker = if diff_z.abs() > 2.0 {
                " [SIGNIFICANT]"
            } else {
                " [not significant]"
            };
            eprintln!(
                "  {:25} near_mean={:>12.2} far_mean={:>12.2} diff_z={:>6.2}{}",
                feature_names[feat_idx], near_mean, far_mean, diff_z, marker
            );
        }

        // ============================================================
        // Step 4: Multi-Scale Decomposition
        // ============================================================
        eprintln!("\n--- Step 4: Multi-Scale Decomposition ---");

        let max_scale = (n as f64).log2().ceil() as u32;
        let mut scales_discriminating_1sigma = 0u32;
        let mut scales_discriminating_2sigma = 0u32;

        for scale in 1..=max_scale {
            let modulus = 1u64 << scale;
            if modulus >= n {
                break;
            }
            let true_bucket = d_true % modulus;

            // For each feature, check if d_true's bucket is anomalous
            for (feat_idx, f) in features.iter().enumerate() {
                // Compute average feature value per bucket
                let mut bucket_sums: std::collections::HashMap<u64, (f64, u64)> =
                    std::collections::HashMap::new();
                for &(d_c, ref fp) in &all_fingerprints {
                    let bucket = d_c % modulus;
                    let entry = bucket_sums.entry(bucket).or_insert((0.0, 0));
                    entry.0 += f(fp);
                    entry.1 += 1;
                }
                let bucket_means: Vec<(u64, f64)> = bucket_sums
                    .iter()
                    .map(|(&b, &(sum, cnt))| (b, sum / cnt as f64))
                    .collect();

                let true_bucket_mean = bucket_means
                    .iter()
                    .find(|&&(b, _)| b == true_bucket)
                    .map(|&(_, m)| m)
                    .unwrap_or(0.0);

                let other_means: Vec<f64> = bucket_means
                    .iter()
                    .filter(|&&(b, _)| b != true_bucket)
                    .map(|&(_, m)| m)
                    .collect();

                if other_means.is_empty() {
                    continue;
                }

                let om_mean = other_means.iter().sum::<f64>() / other_means.len() as f64;
                let om_var = other_means
                    .iter()
                    .map(|v| (v - om_mean).powi(2))
                    .sum::<f64>()
                    / other_means.len() as f64;
                let om_std = om_var.sqrt();

                let z = if om_std > 0.0 {
                    (true_bucket_mean - om_mean) / om_std
                } else {
                    0.0
                };

                if scale <= 4 || z.abs() > 1.0 {
                    // Only print first few scales and anomalous ones
                    if feat_idx == 0 {
                        // Print scale header once
                        if z.abs() > 2.0 {
                            scales_discriminating_2sigma += 1;
                        }
                        if z.abs() > 1.0 {
                            scales_discriminating_1sigma += 1;
                        }
                    }
                }
                if scale <= 4 && feat_idx == 0 {
                    eprintln!(
                        "  Scale {} (mod {}): bucket {} — {} z={:.2}",
                        scale, modulus, true_bucket, feature_names[feat_idx], z
                    );
                }
            }
        }
        eprintln!(
            "  Scales with > 1sigma discrimination: {}",
            scales_discriminating_1sigma
        );
        eprintln!(
            "  Scales with > 2sigma discrimination: {}",
            scales_discriminating_2sigma
        );

        // ============================================================
        // Step 5: Pathfinder Ranking
        // ============================================================
        eprintln!("\n--- Step 5: Pathfinder Ranking ---");

        // Normalize features to [0,1] and compute behavioral score
        let mut feature_vecs: Vec<Vec<f64>> = vec![Vec::new(); 5];
        for &(_, ref fp) in &all_fingerprints {
            feature_vecs[0].push(fp.nonce_spread);
            feature_vecs[1].push(fp.spectral_coherence);
            feature_vecs[2].push(fp.cross_correlation);
            feature_vecs[3].push(fp.x_projection_peak);
            feature_vecs[4].push(fp.cluster_spread);
        }

        // Normalize each feature
        let mut normalized: Vec<Vec<f64>> = Vec::new();
        for fv in &feature_vecs {
            let min_v = fv.iter().cloned().fold(f64::INFINITY, |a, b| a.min(b));
            let max_v = fv.iter().cloned().fold(f64::NEG_INFINITY, |a, b| a.max(b));
            let range = max_v - min_v;
            let norm: Vec<f64> = if range > 0.0 {
                fv.iter().map(|v| (v - min_v) / range).collect()
            } else {
                vec![0.5; fv.len()]
            };
            normalized.push(norm);
        }

        // NON-CIRCULAR anomaly ranking.
        // We do NOT use knowledge of d_true's feature values.
        // Instead, we define anomaly = how far each candidate's features are from
        // the POPULATION MEAN. The hypothesis is that d_true has anomalous features
        // (nonce_spread near 0, high x_projection coherence, etc.)
        //
        // Also: rank by LOWEST nonce_spread (theoretically 0 at d_true).
        // This IS a non-circular metric: any attacker could rank by nonce_spread.

        // Compute population mean and std for each feature
        let pop_means: Vec<f64> = (0..5)
            .map(|f| {
                let vals: Vec<f64> = normalized[f].clone();
                vals.iter().sum::<f64>() / vals.len() as f64
            })
            .collect();
        let pop_stds: Vec<f64> = (0..5)
            .map(|f| {
                let vals: Vec<f64> = normalized[f].clone();
                let mean = pop_means[f];
                let var = vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / vals.len() as f64;
                var.sqrt()
            })
            .collect();

        // Anomaly score using ALL features (dominated by verification metrics)
        let mut anomaly_scores_all: Vec<(u64, f64)> = Vec::new();
        for d_c in 0..n {
            let z_total: f64 = (0..5)
                .map(|f| {
                    if pop_stds[f] > 0.0 {
                        ((normalized[f][d_c as usize] - pop_means[f]) / pop_stds[f]).abs()
                    } else {
                        0.0
                    }
                })
                .sum();
            anomaly_scores_all.push((d_c, z_total));
        }
        anomaly_scores_all.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let anomaly_rank_all = anomaly_scores_all
            .iter()
            .position(|(d, _)| *d == d_true)
            .unwrap_or(n as usize)
            + 1;

        eprintln!("  Ranking by ALL-feature anomaly (includes verification, non-circular):");
        eprintln!(
            "  d_true = {} rank: {} out of {}",
            d_true, anomaly_rank_all, n
        );
        eprintln!("  Top 10:");
        for (rank, (d_c, score)) in anomaly_scores_all.iter().take(10).enumerate() {
            let marker = if *d_c == d_true { " <-- d_true" } else { "" };
            eprintln!(
                "    rank {:3}: d_c={:4} anomaly={:.4}{}",
                rank + 1,
                d_c,
                score,
                marker
            );
        }

        // Anomaly score using ONLY genuine (non-verification) features: indices 1, 2
        let genuine_features = [1usize, 2];
        let mut anomaly_scores_genuine: Vec<(u64, f64)> = Vec::new();
        for d_c in 0..n {
            let z_total: f64 = genuine_features
                .iter()
                .map(|&f| {
                    if pop_stds[f] > 0.0 {
                        ((normalized[f][d_c as usize] - pop_means[f]) / pop_stds[f]).abs()
                    } else {
                        0.0
                    }
                })
                .sum();
            anomaly_scores_genuine.push((d_c, z_total));
        }
        anomaly_scores_genuine.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let anomaly_rank_genuine = anomaly_scores_genuine
            .iter()
            .position(|(d, _)| *d == d_true)
            .unwrap_or(n as usize)
            + 1;

        eprintln!("\n  Ranking by GENUINE-feature anomaly (no scalar_mul, non-circular):");
        eprintln!(
            "  d_true = {} rank: {} out of {}",
            d_true, anomaly_rank_genuine, n
        );
        eprintln!("  Top 10:");
        for (rank, (d_c, score)) in anomaly_scores_genuine.iter().take(10).enumerate() {
            let marker = if *d_c == d_true { " <-- d_true" } else { "" };
            eprintln!(
                "    rank {:3}: d_c={:4} anomaly={:.4}{}",
                rank + 1,
                d_c,
                score,
                marker
            );
        }

        // Use the genuine anomaly rank for the verdict
        let anomaly_rank = anomaly_rank_genuine;

        // Also rank by a feature-agnostic approach: use nonce_spread alone
        // (since it's the most theoretically motivated — exact zero at d_true)
        let mut spread_ranking: Vec<(u64, f64)> = all_fingerprints
            .iter()
            .map(|&(d_c, ref fp)| (d_c, fp.nonce_spread))
            .collect();
        spread_ranking.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let spread_rank = spread_ranking
            .iter()
            .position(|(d, _)| *d == d_true)
            .unwrap_or(n as usize)
            + 1;

        eprintln!("\n  Ranking by nonce_spread alone (lower = better):");
        eprintln!("  d_true = {} rank: {} out of {}", d_true, spread_rank, n);
        eprintln!("  Top 10:");
        for (rank, (d_c, spread)) in spread_ranking.iter().take(10).enumerate() {
            let marker = if *d_c == d_true { " <-- d_true" } else { "" };
            eprintln!(
                "    rank {:3}: d_c={:4} spread={:.4}{}",
                rank + 1,
                d_c,
                spread,
                marker
            );
        }

        // Binary classifier approach
        // Split candidates: odd d_c for training, even for test
        // Threshold: find the nonce_spread threshold that best separates d_true from others
        // (but d_true only appears once, so this is really about the feature distribution)
        let true_spread = fp_true.nonce_spread;
        let count_below_true = all_fingerprints
            .iter()
            .filter(|&&(d_c, ref fp)| d_c != d_true && fp.nonce_spread <= true_spread + 1e-10)
            .count();
        eprintln!(
            "\n  Binary classifier: {} candidates have nonce_spread <= d_true's ({:.4})",
            count_below_true, true_spread
        );

        // ============================================================
        // Step 6: Cost Accounting
        // ============================================================
        eprintln!("\n--- Step 6: Cost Accounting ---");

        let bsgs_cost = (n as f64).sqrt().ceil() as u64;
        let per_candidate_ops = total_ops / n;
        eprintln!("  Total scalar_mul operations: {}", total_ops);
        eprintln!("  Per-candidate cost: {} scalar_muls", per_candidate_ops);
        eprintln!("  Candidates evaluated: {}", n);
        eprintln!("  Total cost: {} scalar_muls", total_ops);
        eprintln!("  BSGS cost: ~{} operations (sqrt({}))", bsgs_cost, n);
        eprintln!(
            "  Cost ratio (pathfinder / BSGS): {:.1}x",
            total_ops as f64 / bsgs_cost as f64
        );

        // Could we evaluate fewer candidates?
        let sqrt_n = bsgs_cost;
        if anomaly_rank <= sqrt_n as usize {
            eprintln!(
                "  d_true in top sqrt(n)={} by anomaly: YES (rank {})",
                sqrt_n, anomaly_rank
            );
        } else {
            eprintln!(
                "  d_true in top sqrt(n)={} by anomaly: NO (rank {})",
                sqrt_n, anomaly_rank
            );
        }
        if spread_rank <= sqrt_n as usize {
            eprintln!(
                "  d_true in top sqrt(n)={} by spread: YES (rank {})",
                sqrt_n, spread_rank
            );
        } else {
            eprintln!(
                "  d_true in top sqrt(n)={} by spread: NO (rank {})",
                sqrt_n, spread_rank
            );
        }

        // ============================================================
        // Verdict
        // ============================================================
        eprintln!("\n=== VERDICT ===");

        let top_10_pct = (n as f64 * 0.1).ceil() as usize;
        let has_anomaly_ranking_signal = anomaly_rank <= top_10_pct;
        let has_spread_ranking_signal = spread_rank <= top_10_pct;
        let has_zscore_signal = stats.iter().any(|s| s.z_score.abs() > 2.0);
        let has_scale_signal = scales_discriminating_2sigma >= 3;
        let spread_is_zero = true_spread < 1e-10;

        eprintln!(
            "  d_true anomaly rank: {} / {} (top {:.1}%)",
            anomaly_rank,
            n,
            anomaly_rank as f64 / n as f64 * 100.0
        );
        eprintln!(
            "  d_true spread rank: {} / {} (top {:.1}%)",
            spread_rank,
            n,
            spread_rank as f64 / n as f64 * 100.0
        );
        eprintln!(
            "  Anomaly ranking signal (top 10%): {}",
            if has_anomaly_ranking_signal {
                "YES"
            } else {
                "NO"
            }
        );
        eprintln!(
            "  Spread ranking signal (top 10%): {}",
            if has_spread_ranking_signal {
                "YES"
            } else {
                "NO"
            }
        );
        eprintln!(
            "  Z-score signal (any > 2sigma): {}",
            if has_zscore_signal { "YES" } else { "NO" }
        );
        eprintln!(
            "  Multi-scale signal (>= 3 scales): {}",
            if has_scale_signal { "YES" } else { "NO" }
        );

        if spread_is_zero {
            eprintln!("\n  CRITICAL OBSERVATION: nonce_spread is EXACTLY ZERO at d_true.");
            eprintln!("  This is the VERIFICATION delta function repackaged.");
            eprintln!("  Ranking by nonce_spread = brute-force verification of each candidate.");
            eprintln!(
                "  Cost per candidate: {} scalar_muls (same as BSGS per step).",
                sigs.len()
            );
            eprintln!(
                "  Full scan: {} candidates x {} sigs = {} ops (vs BSGS: {}).",
                n,
                sigs.len(),
                n as u64 * sigs.len() as u64,
                bsgs_cost
            );
            eprintln!("  The nonce_spread feature IS verification — it finds d perfectly");
            eprintln!("  but at O(n) cost. No advantage over BSGS.");
        }

        // CRITICAL ANALYSIS: which features are genuinely non-verification?
        //
        // Features that use scalar_mul(k_c, G) are VERIFICATION metrics — they check
        // whether k_c*G matches the signature. They're just the ECDSA verification
        // equation repackaged. These are:
        //   - nonce_spread (variance of |x(k_c*G) - r_i|) — VERIFICATION
        //   - x_projection_peak (correlation of x(k_c*G) with r_i) — VERIFICATION
        //   - cluster_spread (spread of k_c*G points) — USES scalar_mul
        //
        // Features that operate ONLY on the implied nonce values k_c = s^{-1}(h+rd_c)
        // without computing k_c*G are genuinely non-verification:
        //   - spectral_coherence (DFT of nonce sequence) — GENUINE
        //   - cross_correlation (pairwise nonce products) — GENUINE
        //
        // Only the genuine features matter. The verification features are known to be
        // perfect (delta function at d_true) but cost O(n) scalar_muls to evaluate.

        eprintln!("\n  FEATURE CLASSIFICATION:");
        eprintln!("  Verification-based (use scalar_mul, = known delta):");
        eprintln!(
            "    nonce_spread     z={:.2} (computes k*G for each candidate)",
            stats[0].z_score
        );
        eprintln!(
            "    x_projection_peak z={:.2} (computes k*G for each candidate)",
            stats[3].z_score
        );
        eprintln!(
            "    cluster_spread   z={:.2} (computes k*G for each candidate)",
            stats[4].z_score
        );
        eprintln!("  Genuinely non-verification (no scalar_mul, only mod arithmetic):");
        eprintln!(
            "    spectral_coherence z={:.2} (DFT of implied nonces only)",
            stats[1].z_score
        );
        eprintln!(
            "    cross_correlation  z={:.2} (pairwise nonce products only)",
            stats[2].z_score
        );

        // The genuine features are indices 1 (spectral_coherence) and 2 (cross_correlation)
        let genuine_signal = stats[1].z_score.abs() > 2.0 || stats[2].z_score.abs() > 2.0;

        if genuine_signal {
            eprintln!("\n  GENUINE SIGNAL: non-verification features discriminate d_true!");
            eprintln!("  This would mean the implied nonce SEQUENCE has detectable structure");
            eprintln!("  at d_true that doesn't require computing k*G. Investigate further.");
        } else {
            eprintln!("\n  NEGATIVE RESULT: No genuine behavioral discrimination.");
            eprintln!("  The non-verification features (spectral_coherence, cross_correlation)");
            eprintln!("  show NO anomaly at d_true (|z| < 2 for both).");
            eprintln!("  All detected signal comes from verification-based features,");
            eprintln!("  which are just the ECDSA verification equation repackaged.");
            eprintln!("  The implied nonce SEQUENCE k_i(d_c) = s_i^{{-1}}(h_i + r_i*d_c) mod n");
            eprintln!("  has no detectable structure at d_true without computing k_i*G.");
            eprintln!("  ");
            eprintln!("  WHY: k_i(d_c) is a LINEAR function of d_c modulo a prime n.");
            eprintln!("  As d_c varies, the nonces rotate uniformly through Z/nZ.");
            eprintln!(
                "  There is no special structure at d_true visible in the nonce values alone."
            );
            eprintln!("  The structure only appears when you MAP nonces to curve points (k*G)");
            eprintln!(
                "  and CHECK against the signature — which is verification, not discrimination."
            );
        }

        // Bits accounting
        let anomaly_bits = if anomaly_rank <= 1 {
            (n as f64).log2()
        } else {
            (n as f64 / anomaly_rank as f64).log2().max(0.0)
        };
        let spread_bits = if spread_rank <= 1 {
            (n as f64).log2()
        } else {
            (n as f64 / spread_rank as f64).log2().max(0.0)
        };
        eprintln!("\n  Bits of discrimination (anomaly): {:.1}", anomaly_bits);
        eprintln!("  Bits of discrimination (spread): {:.1}", spread_bits);
        eprintln!(
            "  (Full key = {:.1} bits, BSGS = {:.1} bits per sqrt(n) ops)\n",
            (n as f64).log2(),
            (n as f64).log2() / 2.0
        );
    }
}
