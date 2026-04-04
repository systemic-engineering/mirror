//! Character sum approach — the curve equation as spectral constraint.
//!
//! The Cayley graph was the wrong graph. The Laplacian threw away
//! the algebraic structure. The right structure is:
//!
//! 1. The Legendre symbol pattern: which x-values are on the curve
//! 2. The Frobenius trace: 2 numbers that characterize the entire curve
//! 3. Character sums: the DFT of the Legendre pattern over GF(p)
//!
//! The question: does the character sum at a public key's x-coordinate
//! encode information about the discrete log?

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

    /// Legendre symbol: (a/p) = a^((p-1)/2) mod p.
    /// Returns 1 if quadratic residue, -1 if non-residue, 0 if a ≡ 0.
    pub fn legendre(a: u64, p: u64) -> i64 {
        if a % p == 0 {
            return 0;
        }
        let r = mod_pow(a as u128, ((p - 1) / 2) as u128, p as u128) as u64;
        if r == 1 {
            1
        } else {
            -1
        }
    }

    /// Evaluate the curve RHS: x³ + ax + b (mod p).
    pub fn curve_rhs(x: u64, a: u64, b: u64, p: u64) -> u64 {
        let xm = x as u128;
        let pm = p as u128;
        ((xm * xm % pm * xm % pm + (a as u128) * xm % pm + b as u128) % pm) as u64
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
}

#[cfg(test)]
mod tests {
    use super::curve::*;

    const A: u64 = 1;
    const B: u64 = 1;
    const P: u64 = 251;

    #[test]
    fn legendre_pattern_is_not_random() {
        // The Legendre symbol pattern χ(x) = (x³+ax+b / p) for x = 0..p-1
        // is NOT random. It's constrained by the curve equation.
        // The Frobenius trace t = p + 1 - #E tells us the SUM of the pattern.
        let points = enumerate_curve(A, B, P);
        let n = points.len(); // includes infinity

        let frobenius_trace = P as i64 + 1 - n as i64;
        eprintln!("  frobenius trace: {}", frobenius_trace);

        // Sum of Legendre symbols over all x: Σ χ(x³+ax+b) = n - 1 - p = -t
        let mut legendre_sum = 0i64;
        for x in 0..P {
            let rhs = curve_rhs(x, A, B, P);
            legendre_sum += legendre(rhs, P);
        }
        eprintln!("  legendre sum: {}", legendre_sum);
        eprintln!("  expected (= -trace): {}", -frobenius_trace);

        assert_eq!(
            legendre_sum, -frobenius_trace,
            "sum of Legendre symbols should equal negative Frobenius trace"
        );
    }

    #[test]
    fn character_sum_at_public_key_x() {
        // For a public key Q = kG with x-coordinate x_Q,
        // compute the "character sum up to x_Q":
        //   S(x_Q) = Σ_{x=0}^{x_Q} χ(x³+ax+b)
        //
        // This is a partial sum of the Legendre pattern.
        // If it correlates with k, we have leverage.

        let points = enumerate_curve(A, B, P);
        let gen = points[1]; // first non-infinity point

        // Compute generator order
        let mut pt = gen;
        let mut order = 1u64;
        loop {
            pt = point_add(pt, gen, A, P);
            order += 1;
            if pt == Point::Infinity || order > points.len() as u64 + 1 {
                break;
            }
        }
        eprintln!("  generator: {:?}, order: {}", gen, order);

        // Precompute cumulative Legendre sum
        let mut cum_legendre = vec![0i64; P as usize + 1];
        for x in 0..P {
            let rhs = curve_rhs(x, A, B, P);
            cum_legendre[x as usize + 1] = cum_legendre[x as usize] + legendre(rhs, P);
        }

        // For each keypair, compute:
        // - private key k
        // - public key x-coordinate x_Q
        // - cumulative Legendre sum S(x_Q)
        // - does S(x_Q) correlate with k?
        let mut pairs: Vec<(u64, u64, i64)> = Vec::new(); // (k, x_Q, S(x_Q))
        for k in 1..order {
            let public = scalar_mul(k, gen, A, P);
            if let Point::Affine { x, .. } = public {
                let s = cum_legendre[x as usize + 1];
                pairs.push((k, x, s));
            }
        }

        // Compute Spearman rank correlation between k and S(x_Q)
        let n = pairs.len();
        let mut k_ranks: Vec<(u64, usize)> = pairs
            .iter()
            .enumerate()
            .map(|(i, &(k, _, _))| (k, i))
            .collect();
        k_ranks.sort_by_key(|&(k, _)| k);
        let mut rank_k = vec![0usize; n];
        for (rank, &(_, orig_idx)) in k_ranks.iter().enumerate() {
            rank_k[orig_idx] = rank;
        }

        let mut s_ranks: Vec<(i64, usize)> = pairs
            .iter()
            .enumerate()
            .map(|(i, &(_, _, s))| (s, i))
            .collect();
        s_ranks.sort_by_key(|&(s, _)| s);
        let mut rank_s = vec![0usize; n];
        for (rank, &(_, orig_idx)) in s_ranks.iter().enumerate() {
            rank_s[orig_idx] = rank;
        }

        let d_squared_sum: f64 = (0..n)
            .map(|i| {
                let d = rank_k[i] as f64 - rank_s[i] as f64;
                d * d
            })
            .sum();
        let nf = n as f64;
        let spearman = 1.0 - 6.0 * d_squared_sum / (nf * (nf * nf - 1.0));

        eprintln!("  keypairs: {}", n);
        eprintln!("  spearman(k, S(x_Q)): {:.4}", spearman);
        eprintln!("  (0 = no correlation, ±1 = perfect)");

        // Also check: does x_Q alone correlate with k?
        let mut x_ranks: Vec<(u64, usize)> = pairs
            .iter()
            .enumerate()
            .map(|(i, &(_, x, _))| (x, i))
            .collect();
        x_ranks.sort_by_key(|&(x, _)| x);
        let mut rank_x = vec![0usize; n];
        for (rank, &(_, orig_idx)) in x_ranks.iter().enumerate() {
            rank_x[orig_idx] = rank;
        }

        let d_squared_x: f64 = (0..n)
            .map(|i| {
                let d = rank_k[i] as f64 - rank_x[i] as f64;
                d * d
            })
            .sum();
        let spearman_x = 1.0 - 6.0 * d_squared_x / (nf * (nf * nf - 1.0));
        eprintln!("  spearman(k, x_Q): {:.4}", spearman_x);

        // The character sum approach: compute the DFT of the Legendre pattern
        // S_t = Σ_x χ(x³+ax+b) · ω^(tx)  where ω = exp(2πi/p)
        // This is the multiplicative character sum. For t = x_Q, it encodes
        // how the curve's structure interacts with the public key's position.

        // Compute a few DFT coefficients of the Legendre pattern
        let pi2 = 2.0 * std::f64::consts::PI;
        let pf = P as f64;

        // Coefficient at frequency t: S_t = Σ_x χ(x³+ax+b) · exp(2πi·t·x/p)
        // We compute |S_t|² (power spectrum) for the first 10 frequencies
        eprintln!("  character sum power spectrum (first 10):");
        for t in 0..10u64 {
            let mut re = 0.0f64;
            let mut im = 0.0f64;
            for x in 0..P {
                let rhs = curve_rhs(x, A, B, P);
                let chi = legendre(rhs, P) as f64;
                let angle = pi2 * (t as f64) * (x as f64) / pf;
                re += chi * angle.cos();
                im += chi * angle.sin();
            }
            let power = re * re + im * im;
            eprintln!(
                "    S_{}: |S|² = {:.2}, |S| = {:.2}",
                t,
                power,
                power.sqrt()
            );
        }

        // The Hasse-Weil bound: |S_t| ≤ 2√p for all t ≠ 0.
        // For p = 251: bound = 2√251 ≈ 31.7
        let hasse_bound = 2.0 * (P as f64).sqrt();
        eprintln!("  Hasse-Weil bound: |S_t| ≤ {:.1}", hasse_bound);

        // The KEY question: for a public key Q = kG with x_Q,
        // does S_{x_Q} (the character sum at frequency x_Q) encode k?
        // If yes: character sums give leverage on the DLP.
        // If no: the algebraic structure doesn't help.

        let mut char_sum_pairs: Vec<(u64, f64)> = Vec::new(); // (k, |S_{x_Q}|)
        for &(k, x_q, _) in &pairs {
            let mut re = 0.0f64;
            let mut im = 0.0f64;
            for x in 0..P {
                let rhs = curve_rhs(x, A, B, P);
                let chi = legendre(rhs, P) as f64;
                let angle = pi2 * (x_q as f64) * (x as f64) / pf;
                re += chi * angle.cos();
                im += chi * angle.sin();
            }
            let magnitude = (re * re + im * im).sqrt();
            char_sum_pairs.push((k, magnitude));
        }

        // Correlation between k and |S_{x_Q}|
        let mut mag_ranks: Vec<(usize, f64)> = char_sum_pairs
            .iter()
            .enumerate()
            .map(|(i, &(_, m))| (i, m))
            .collect();
        mag_ranks.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let mut rank_mag = vec![0usize; n];
        for (rank, &(orig_idx, _)) in mag_ranks.iter().enumerate() {
            rank_mag[orig_idx] = rank;
        }

        let d_squared_mag: f64 = (0..n)
            .map(|i| {
                let d = rank_k[i] as f64 - rank_mag[i] as f64;
                d * d
            })
            .sum();
        let spearman_mag = 1.0 - 6.0 * d_squared_mag / (nf * (nf * nf - 1.0));
        eprintln!("  spearman(k, |S_{{x_Q}}|): {:.4}", spearman_mag);

        // Report
        eprintln!("\n  === SUMMARY ===");
        eprintln!("  cumulative legendre ↔ k:  {:.4}", spearman);
        eprintln!("  x-coordinate ↔ k:         {:.4}", spearman_x);
        eprintln!("  character sum mag ↔ k:     {:.4}", spearman_mag);
        eprintln!("  (any |ρ| > 0.1 is signal)");
    }
}
