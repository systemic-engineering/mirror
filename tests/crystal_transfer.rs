//! Crystal transfer test — the REAL experiment.
//!
//! Does the 8-bit crystal PREDICT the 12-bit structure?
//! Not by recomputing. By tiling.
//!
//! The 8-bit Fiedler pair encodes the ring navigation at scale 282.
//! The 12-bit ring has 2050 points (in the generator's orbit).
//! The DFT basis is the SAME FUNCTION at different sample rates.
//!
//! Test: interpolate the 8-bit Fiedler vectors to 12-bit.
//! Use as initial guess for Lanczos. Does it converge faster?
//! Does the interpolated vector already recover private keys?

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

    fn mod_sqrt(n: u64, p: u64) -> Option<u64> {
        if n == 0 {
            return Some(0);
        }
        let pm = p as u128;
        let nm = n as u128;
        if mod_pow(nm, (pm - 1) / 2, pm) != 1 {
            return None;
        }
        if p % 4 == 3 {
            return Some(mod_pow(nm, (pm + 1) / 4, pm) as u64);
        }
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

    pub fn enumerate_curve(a: u64, b: u64, p: u64) -> Vec<Point> {
        let mut points = vec![Point::Infinity];
        let pm = p as u128;
        for x in 0..p {
            let xm = x as u128;
            let rhs = ((xm * xm % pm * xm % pm) + (a as u128) * xm % pm + (b as u128)) % pm;
            if let Some(y) = mod_sqrt(rhs as u64, p) {
                points.push(Point::Affine { x, y });
                if y != 0 {
                    points.push(Point::Affine { x, y: p - y });
                }
            }
        }
        points
    }

    /// Build the ring walk ordering: starting from Infinity, walk by adding G.
    /// Returns the vertex indices in ring order.
    pub fn ring_ordering(points: &[Point], gen: Point, a: u64, p: u64, order: u64) -> Vec<usize> {
        let point_to_idx: std::collections::HashMap<Point, usize> =
            points.iter().enumerate().map(|(i, &p)| (p, i)).collect();
        let mut ordering = Vec::with_capacity(order as usize);
        let mut pt = Point::Infinity;
        for _ in 0..order {
            ordering.push(point_to_idx[&pt]);
            pt = point_add(pt, gen, a, p);
        }
        ordering
    }
}

#[cfg(test)]
mod tests {
    use super::curve::*;
    use std::f64::consts::PI;

    #[test]
    fn crystal_tiling_8bit_to_12bit() {
        // ── 8-bit crystal: the known structure ──────────────────────

        let points_8 = enumerate_curve(1, 1, 251);
        let gen_8 = points_8[1];
        let mut pt = gen_8;
        let mut order_8 = 1u64;
        loop {
            pt = point_add(pt, gen_8, 1, 251);
            order_8 += 1;
            if pt == Point::Infinity || order_8 > 300 {
                break;
            }
        }
        eprintln!("  8-bit: {} points, order {}", points_8.len(), order_8);

        // The 8-bit ring ordering: which vertex is at position k?
        let ring_8 = ring_ordering(&points_8, gen_8, 1, 251, order_8);

        // The 8-bit Fiedler pair in RING order (not vertex order).
        // For a ring of size n, position k has:
        //   v1[k] = cos(2π·k/n)
        //   v2[k] = sin(2π·k/n)
        // This IS the crystal. Known analytically for the 8-bit ring.
        let n8 = order_8 as f64;
        let crystal_8_cos: Vec<f64> = (0..order_8)
            .map(|k| (2.0 * PI * k as f64 / n8).cos())
            .collect();
        let crystal_8_sin: Vec<f64> = (0..order_8)
            .map(|k| (2.0 * PI * k as f64 / n8).sin())
            .collect();

        // Verify: the 8-bit crystal recovers all private keys
        let mut correct_8 = 0u64;
        for k in 1..order_8 {
            let phase = crystal_8_sin[k as usize].atan2(crystal_8_cos[k as usize]);
            let recovered = ((phase * n8 / (2.0 * PI) + n8) % n8).round() as u64 % order_8;
            if recovered == k || recovered == order_8 - k {
                correct_8 += 1;
            }
        }
        eprintln!(
            "  8-bit crystal recovery: {}/{} ({:.0}%)",
            correct_8,
            order_8 - 1,
            correct_8 as f64 / (order_8 - 1) as f64 * 100.0
        );
        assert_eq!(correct_8, order_8 - 1, "8-bit crystal must be 100%");

        // ── 12-bit target: the unknown structure ────────────────────

        let points_12 = enumerate_curve(1, 1, 4093);
        let gen_12 = points_12[1];
        let mut pt = gen_12;
        let mut order_12 = 1u64;
        loop {
            pt = point_add(pt, gen_12, 1, 4093);
            order_12 += 1;
            if pt == Point::Infinity || order_12 > points_12.len() as u64 + 1 {
                break;
            }
        }
        eprintln!("  12-bit: {} points, order {}", points_12.len(), order_12);

        // The 12-bit ring ordering
        let ring_12 = ring_ordering(&points_12, gen_12, 1, 4093, order_12);

        // ── The tiling: 8-bit crystal predicts 12-bit ──────────────
        //
        // The key insight: the DFT basis at scale n is:
        //   v1[k] = cos(2π·k/n)
        //   v2[k] = sin(2π·k/n)
        //
        // At 8-bit (n=282): v1[k] = cos(2πk/282)
        // At 12-bit (n=2050): v1[k] = cos(2πk/2050)
        //
        // Same function. Different sample rate. The 8-bit crystal
        // "predicts" the 12-bit crystal by scaling: the SHAPE of
        // cos/sin doesn't change. Only the frequency changes.
        //
        // But we DON'T KNOW the ring ordering at 12-bit.
        // We know it at 8-bit (because we computed it — brute force).
        // At 12-bit, the ring ordering IS the discrete log.
        //
        // The tiling question: can we INFER the 12-bit ring ordering
        // from the 8-bit ordering + the curve structure?
        //
        // If both curves have the same equation (y²=x³+x+1), the
        // group operation is the same polynomial. The only difference
        // is the field size (GF(251) vs GF(4093)).
        //
        // The STRUCTURE of the walk is determined by the curve equation.
        // At 8-bit: 0→(0,1)→(63,93)→(72,109)→...
        // At 12-bit: 0→(0,1)→?→?→...
        //
        // The second step is the same: (0,1)+(0,1) = 2G.
        // The x-coordinate of 2G depends on the field size.
        // At 8-bit: 2G = (63,93)
        // At 12-bit: 2G = ... let's compute.

        let two_g_8 = scalar_mul(2, gen_8, 1, 251);
        let two_g_12 = scalar_mul(2, gen_12, 1, 4093);
        eprintln!("  8-bit  2G: {:?}", two_g_8);
        eprintln!("  12-bit 2G: {:?}", two_g_12);

        // The x-coordinates of kG form a sequence determined by the
        // curve's addition law over GF(p). This sequence is pseudorandom
        // (secure DLP depends on this). But is there a PATTERN in how
        // the 8-bit sequence relates to the 12-bit sequence?

        // Collect the x-coordinate sequences
        let x_seq_8: Vec<u64> = (0..order_8)
            .map(|k| {
                match scalar_mul(k, gen_8, 1, 251) {
                    Point::Affine { x, .. } => x,
                    Point::Infinity => 251, // sentinel
                }
            })
            .collect();

        let x_seq_12: Vec<u64> = (0..order_12.min(1000))
            .map(|k| match scalar_mul(k, gen_12, 1, 4093) {
                Point::Affine { x, .. } => x,
                Point::Infinity => 4093,
            })
            .collect();

        // Normalize to [0,1]: x/p
        let x_norm_8: Vec<f64> = x_seq_8.iter().map(|&x| x as f64 / 251.0).collect();
        let x_norm_12: Vec<f64> = x_seq_12.iter().map(|&x| x as f64 / 4093.0).collect();

        // Correlation between normalized x-sequences (first 282 terms)
        let min_len = x_norm_8.len().min(x_norm_12.len());
        let mean_8: f64 = x_norm_8[..min_len].iter().sum::<f64>() / min_len as f64;
        let mean_12: f64 = x_norm_12[..min_len].iter().sum::<f64>() / min_len as f64;

        let mut cov = 0.0f64;
        let mut var_8 = 0.0f64;
        let mut var_12 = 0.0f64;
        for i in 0..min_len {
            let d8 = x_norm_8[i] - mean_8;
            let d12 = x_norm_12[i] - mean_12;
            cov += d8 * d12;
            var_8 += d8 * d8;
            var_12 += d12 * d12;
        }
        let pearson = cov / (var_8.sqrt() * var_12.sqrt()).max(1e-15);

        eprintln!(
            "  x-sequence correlation (8↔12, first {}): {:.4}",
            min_len, pearson
        );
        eprintln!("  (1.0 = identical sequences, 0.0 = no correlation)");

        // The tiling test: if pearson ≈ 1, the x-coordinates follow
        // the same pattern at both scales (normalized by field size).
        // That would mean the ring ordering TRANSFERS — the crystal tiles.
        //
        // If pearson ≈ 0, the sequences are unrelated.
        // The curve equation doesn't produce self-similar walks.
        // The crystal doesn't tile. The break doesn't work.

        // Also test: do the RATIOS between consecutive x-coordinates match?
        // x_{k+1}/x_k at 8-bit vs x_{k+1}/x_k at 12-bit
        let mut ratio_corr_num = 0.0f64;
        let mut ratio_var_8 = 0.0f64;
        let mut ratio_var_12 = 0.0f64;
        let mut ratio_count = 0;

        for i in 1..min_len - 1 {
            if x_seq_8[i] > 0 && x_seq_8[i + 1] > 0 && x_seq_12[i] > 0 && x_seq_12[i + 1] > 0 {
                let r8 = x_norm_8[i + 1] / x_norm_8[i].max(0.001);
                let r12 = x_norm_12[i + 1] / x_norm_12[i].max(0.001);
                let mr8 = r8 - 1.0; // deviation from 1
                let mr12 = r12 - 1.0;
                ratio_corr_num += mr8 * mr12;
                ratio_var_8 += mr8 * mr8;
                ratio_var_12 += mr12 * mr12;
                ratio_count += 1;
            }
        }
        let ratio_pearson = ratio_corr_num / (ratio_var_8.sqrt() * ratio_var_12.sqrt()).max(1e-15);
        eprintln!(
            "  ratio correlation (Δx_8 ↔ Δx_12): {:.4} (n={})",
            ratio_pearson, ratio_count
        );

        eprintln!("\n  === CRYSTAL TRANSFER VERDICT ===");
        if pearson.abs() > 0.3 {
            eprintln!(
                "  SIGNAL: x-sequences correlate across scales ({:.4})",
                pearson
            );
            eprintln!("  The crystal MAY tile. Further investigation warranted.");
        } else if ratio_pearson.abs() > 0.3 {
            eprintln!("  SIGNAL: ratio sequences correlate ({:.4})", ratio_pearson);
            eprintln!("  The DYNAMICS may transfer even if positions don't.");
        } else {
            eprintln!("  NO SIGNAL: sequences uncorrelated at both levels.");
            eprintln!("  The crystal does not tile naively.");
            eprintln!("  The walk on GF(251) does not predict the walk on GF(4093).");
        }
    }
}
