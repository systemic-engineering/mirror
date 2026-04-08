//! Basis search — does there exist a basis for the key space
//! in which the DLP reconstruction information is sparse?
//!
//! The DLP verification landscape is a delta function in the standard basis
//! (uniform in Fourier space). A delta in one basis might be sparse in another.
//! The question: does the eigenvalue basis of the key-signature constraint
//! space concentrate the DLP information?
//!
//! We test sparsity of e_{d_true} in:
//!   (a) Standard basis — trivially 1-sparse
//!   (b) DFT basis — maximally non-sparse
//!   (c) SVD basis of the binary constraint matrix M
//!   (d) SVD basis of the spectral (residual) constraint matrix S
//!   (e) Hadamard basis
//!   (f) Random orthogonal basis
//!
//! Then: precomputed topology test — does a basis computed for one key
//! transfer to another key?

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

    /// Find the order of a point.
    fn point_order(g: Point, a: u64, p: u64, max: usize) -> usize {
        let mut pt = g;
        for k in 1..=max {
            if pt == Point::Infinity {
                return k;
            }
            pt = point_add(pt, g, a, p);
        }
        max + 1
    }

    /// Check if n is prime.
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
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    /// Find a generator of a prime-order subgroup.
    /// Returns (generator, prime_order).
    fn find_prime_subgroup_generator(points: &[Point], a: u64, p: u64) -> (Point, usize) {
        let n = points.len(); // full group order
                              // Find prime factors of n
        let mut factors = Vec::new();
        let mut m = n as u64;
        let mut d = 2u64;
        while d * d <= m {
            while m % d == 0 {
                if !factors.contains(&d) {
                    factors.push(d);
                }
                m /= d;
            }
            d += 1;
        }
        if m > 1 {
            factors.push(m);
        }
        // Find the largest prime factor and generate a subgroup of that order
        factors.sort();
        let largest_prime = *factors.last().unwrap();
        let cofactor = n as u64 / largest_prime;

        // Find a point whose order is exactly largest_prime
        for pt in points.iter() {
            if *pt == Point::Infinity {
                continue;
            }
            // Multiply by cofactor to get into the subgroup
            let sub_pt = scalar_mul(cofactor, *pt, a, p);
            if sub_pt == Point::Infinity {
                continue;
            }
            // Verify order is exactly largest_prime
            let ord = point_order(sub_pt, a, p, largest_prime as usize + 1);
            if ord == largest_prime as usize {
                return (sub_pt, largest_prime as usize);
            }
        }
        panic!("no prime-order subgroup generator found");
    }

    /// Deterministic ECDSA-like signature: (r, s) where
    ///   k = hash(d || i) mod n  (deterministic nonce)
    ///   R = k·G, r = x(R) mod n
    ///   s = k^{-1}(h + r·d) mod n
    /// Returns (r, s, h) — all mod n.
    fn gcd(a: u64, b: u64) -> u64 {
        let (mut a, mut b) = (a, b);
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    fn sign(d: u64, i: usize, g: Point, n: u64, a: u64, p: u64) -> (u64, u64, u64) {
        // deterministic nonce: simple hash, ensure coprime to n
        let mut k = ((d as u128 * 31 + i as u128 * 97 + 13) % (n as u128 - 1) + 1) as u64;
        // message hash: just use i
        let h = ((i as u128 * 137 + 7) % n as u128) as u64;

        // Ensure k is coprime to n so mod_inv(k, n) exists
        while gcd(k, n) != 1 {
            k = k % (n - 1) + 1;
            if gcd(k, n) == 1 {
                break;
            }
            k += 1;
        }

        let r_pt = scalar_mul(k, g, a, p);
        let r = match r_pt {
            Point::Affine { x, .. } => x % n,
            Point::Infinity => 0,
        };
        if r == 0 {
            // degenerate — bump k, ensure coprime
            k = ((k + 1) % (n - 1)) + 1;
            while gcd(k, n) != 1 {
                k += 1;
                if k >= n {
                    k = 1;
                }
            }
            let r_pt2 = scalar_mul(k, g, a, p);
            let r2 = match r_pt2 {
                Point::Affine { x, .. } => x % n,
                Point::Infinity => 1,
            };
            let k_inv = mod_inv(k, n).unwrap();
            let s = (k_inv as u128 * ((h as u128 + r2 as u128 * d as u128) % n as u128)) as u64 % n;
            return (r2, s, h);
        }

        let k_inv = mod_inv(k, n).unwrap();
        let s = (k_inv as u128 * ((h as u128 + r as u128 * d as u128) % n as u128)) as u64 % n;
        (r, s, h)
    }

    /// Check if signature (r, s, h) is consistent with candidate private key d_c.
    /// Consistent means: x(s^{-1}(h + r·d_c)·G) == r (mod p)
    fn sig_consistent(d_c: u64, r: u64, s: u64, h: u64, g: Point, n: u64, a: u64, p: u64) -> bool {
        if s == 0 {
            return false;
        }
        let s_inv = match mod_inv(s, n) {
            Some(v) => v,
            None => return false,
        };
        let u = (s_inv as u128 * (h as u128 + r as u128 * d_c as u128) % n as u128) as u64 % n;
        let check_pt = scalar_mul(u, g, a, p);
        match check_pt {
            Point::Affine { x, .. } => x % n == r,
            Point::Infinity => false,
        }
    }

    // === Linear algebra helpers (no external deps) ===

    /// Matrix multiply: A (m×k) * B (k×n) = C (m×n)
    fn mat_mul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let m = a.len();
        let k = a[0].len();
        let n = b[0].len();
        let mut c = vec![vec![0.0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let mut s = 0.0;
                for l in 0..k {
                    s += a[i][l] * b[l][j];
                }
                c[i][j] = s;
            }
        }
        c
    }

    /// Matrix transpose
    fn mat_transpose(a: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let m = a.len();
        let n = a[0].len();
        let mut t = vec![vec![0.0; m]; n];
        for i in 0..m {
            for j in 0..n {
                t[j][i] = a[i][j];
            }
        }
        t
    }

    /// Eigendecomposition of a symmetric matrix via Jacobi iteration.
    /// Returns (eigenvalues, eigenvectors_as_columns).
    fn symmetric_eigen(mat: &[Vec<f64>]) -> (Vec<f64>, Vec<Vec<f64>>) {
        let n = mat.len();
        let mut a = mat.to_vec();
        // V starts as identity
        let mut v = vec![vec![0.0; n]; n];
        for i in 0..n {
            v[i][i] = 1.0;
        }

        let max_iter = 100 * n * n;
        for _ in 0..max_iter {
            // Find largest off-diagonal element
            let mut max_val = 0.0f64;
            let mut p = 0;
            let mut q = 1;
            for i in 0..n {
                for j in (i + 1)..n {
                    if a[i][j].abs() > max_val {
                        max_val = a[i][j].abs();
                        p = i;
                        q = j;
                    }
                }
            }
            if max_val < 1e-12 {
                break;
            }

            // Compute rotation
            let theta = if (a[p][p] - a[q][q]).abs() < 1e-15 {
                PI / 4.0
            } else {
                0.5 * (2.0 * a[p][q] / (a[p][p] - a[q][q])).atan()
            };
            let c = theta.cos();
            let s = theta.sin();

            // Apply rotation to A: A' = G^T A G
            let mut new_a = a.clone();
            for i in 0..n {
                if i != p && i != q {
                    new_a[i][p] = c * a[i][p] + s * a[i][q];
                    new_a[p][i] = new_a[i][p];
                    new_a[i][q] = -s * a[i][p] + c * a[i][q];
                    new_a[q][i] = new_a[i][q];
                }
            }
            new_a[p][p] = c * c * a[p][p] + 2.0 * s * c * a[p][q] + s * s * a[q][q];
            new_a[q][q] = s * s * a[p][p] - 2.0 * s * c * a[p][q] + c * c * a[q][q];
            new_a[p][q] = 0.0;
            new_a[q][p] = 0.0;
            a = new_a;

            // Accumulate eigenvectors
            for i in 0..n {
                let vip = v[i][p];
                let viq = v[i][q];
                v[i][p] = c * vip + s * viq;
                v[i][q] = -s * vip + c * viq;
            }
        }

        let eigenvalues: Vec<f64> = (0..n).map(|i| a[i][i]).collect();
        (eigenvalues, v)
    }

    /// Dot product of two vectors
    fn dot(a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }

    /// L2 norm
    fn norm(a: &[f64]) -> f64 {
        dot(a, a).sqrt()
    }

    /// Compute energy distribution: how many coefficients needed for X% energy.
    fn energy_capture(coefficients: &[f64], thresholds: &[f64]) -> Vec<usize> {
        let total_energy: f64 = coefficients.iter().map(|c| c * c).sum();
        if total_energy < 1e-15 {
            return thresholds.iter().map(|_| coefficients.len()).collect();
        }

        // Sort by magnitude (descending)
        let mut sorted: Vec<f64> = coefficients.iter().map(|c| c * c).collect();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let mut results = Vec::new();
        for &thresh in thresholds {
            let target = thresh * total_energy;
            let mut cumulative = 0.0;
            let mut count = 0;
            for &e in &sorted {
                cumulative += e;
                count += 1;
                if cumulative >= target - 1e-12 {
                    break;
                }
            }
            results.push(count);
        }
        results
    }

    /// Simple seeded PRNG (xorshift64)
    struct Rng {
        state: u64,
    }
    impl Rng {
        fn new(seed: u64) -> Self {
            Self {
                state: if seed == 0 { 1 } else { seed },
            }
        }
        fn next_u64(&mut self) -> u64 {
            self.state ^= self.state << 13;
            self.state ^= self.state >> 7;
            self.state ^= self.state << 17;
            self.state
        }
        fn next_f64(&mut self) -> f64 {
            (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
        }
        fn next_gaussian(&mut self) -> f64 {
            // Box-Muller
            let u1 = self.next_f64().max(1e-15);
            let u2 = self.next_f64();
            (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos()
        }
    }

    /// Generate a random orthogonal matrix via QR decomposition of a random matrix.
    fn random_orthogonal(n: usize, seed: u64) -> Vec<Vec<f64>> {
        let mut rng = Rng::new(seed);
        // Generate random matrix
        let mut a = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                a[i][j] = rng.next_gaussian();
            }
        }
        // QR via Gram-Schmidt
        let mut q = vec![vec![0.0; n]; n];
        for j in 0..n {
            let mut col: Vec<f64> = (0..n).map(|i| a[i][j]).collect();
            // Subtract projections of previous columns
            for k in 0..j {
                let prev: Vec<f64> = (0..n).map(|i| q[i][k]).collect();
                let proj = dot(&col, &prev);
                for i in 0..n {
                    col[i] -= proj * prev[i];
                }
            }
            let n_col = norm(&col);
            if n_col > 1e-12 {
                for i in 0..n {
                    q[i][j] = col[i] / n_col;
                }
            }
        }
        q
    }

    /// Compute Walsh-Hadamard transform of a vector (length must be power of 2, so we pad).
    /// Returns coefficients in a basis of size n (we pad to next power of 2, then truncate).
    fn hadamard_coefficients(signal: &[f64], n: usize) -> Vec<f64> {
        // Pad to next power of 2
        let mut m = 1;
        while m < n {
            m <<= 1;
        }
        let mut x = vec![0.0; m];
        for i in 0..n.min(m) {
            x[i] = signal[i];
        }
        // In-place Walsh-Hadamard
        let mut h = 1;
        while h < m {
            for i in (0..m).step_by(h * 2) {
                for j in i..i + h {
                    let u = x[j];
                    let v = x[j + h];
                    x[j] = u + v;
                    x[j + h] = u - v;
                }
            }
            h <<= 1;
        }
        // Normalize
        let scale = 1.0 / (m as f64).sqrt();
        for v in x.iter_mut() {
            *v *= scale;
        }
        // Return first n coefficients
        x.truncate(n);
        x
    }

    /// DFT coefficients of a real signal
    fn dft_coefficients(signal: &[f64], n: usize) -> Vec<f64> {
        // Return magnitudes of DFT coefficients
        let mut mags = Vec::with_capacity(n);
        for k in 0..n {
            let mut re = 0.0f64;
            let mut im = 0.0f64;
            for j in 0..n {
                let angle = -2.0 * PI * (j as f64) * (k as f64) / (n as f64);
                re += signal[j] * angle.cos();
                im += signal[j] * angle.sin();
            }
            // We want the real projection, not magnitude, for energy analysis.
            // But DFT basis vectors are complex. For energy in the DFT basis,
            // the coefficients are the complex amplitudes. Energy = |c_k|^2.
            // For a real signal, c_k and c_{n-k} are conjugates.
            // The "DFT basis representation" has energy = (1/n) sum |c_k|^2.
            // For delta at d_true: all |c_k| = 1, so all equal. Maximally non-sparse.
            mags.push((re * re + im * im).sqrt() / (n as f64).sqrt());
        }
        mags
    }

    /// Build constraint matrix M for a set of signatures and group order.
    /// M[i][d_c] = 1.0 if signature i is consistent with candidate d_c.
    fn build_constraint_matrix(
        sigs: &[(u64, u64, u64)], // (r, s, h)
        g: Point,
        n: u64,
        a_curve: u64,
        p: u64,
    ) -> Vec<Vec<f64>> {
        let num_sigs = sigs.len();
        let mut m = vec![vec![0.0f64; n as usize]; num_sigs];
        for (i, &(r, s, h)) in sigs.iter().enumerate() {
            for d_c in 0..n {
                if sig_consistent(d_c, r, s, h, g, n, a_curve, p) {
                    m[i][d_c as usize] = 1.0;
                }
            }
        }
        m
    }

    /// Build spectral (residual) constraint matrix S.
    /// S[i][d_c] = |x(s^{-1}(h + r·d_c)·G) - r| for each sig i and candidate d_c.
    fn build_spectral_matrix(
        sigs: &[(u64, u64, u64)],
        g: Point,
        n: u64,
        a_curve: u64,
        p: u64,
    ) -> Vec<Vec<f64>> {
        let num_sigs = sigs.len();
        let mut s_mat = vec![vec![0.0f64; n as usize]; num_sigs];
        for (i, &(r, s, h)) in sigs.iter().enumerate() {
            for d_c in 0..n {
                if s == 0 {
                    s_mat[i][d_c as usize] = p as f64; // large residual
                    continue;
                }
                let s_inv = match mod_inv(s, n) {
                    Some(v) => v,
                    None => {
                        s_mat[i][d_c as usize] = p as f64;
                        continue;
                    }
                };
                let u = (s_inv as u128 * ((h as u128 + r as u128 * d_c as u128) % n as u128))
                    as u64
                    % n;
                let check_pt = scalar_mul(u, g, a_curve, p);
                match check_pt {
                    Point::Affine { x, .. } => {
                        let diff = if x % n >= r {
                            (x % n - r) as f64
                        } else {
                            (r - x % n) as f64
                        };
                        // Also consider wraparound
                        let diff_wrap = (n as f64 - diff).min(diff);
                        s_mat[i][d_c as usize] = diff_wrap;
                    }
                    Point::Infinity => {
                        s_mat[i][d_c as usize] = p as f64;
                    }
                }
            }
        }
        s_mat
    }

    /// Compute thin SVD of M (num_sigs × n_cols) via eigendecomp of M·M^T (small square matrix).
    /// Returns (singular_values, right_singular_vectors_as_columns_of_V).
    /// V is n_cols × rank matrix where columns are right singular vectors.
    fn thin_svd(m: &[Vec<f64>]) -> (Vec<f64>, Vec<Vec<f64>>) {
        let num_rows = m.len();
        let num_cols = m[0].len();

        // Compute M · M^T (num_rows × num_rows)
        let mt = mat_transpose(m);
        let mmt = mat_mul(m, &mt);

        // Eigendecomposition of M·M^T
        let (eigenvalues, u_mat) = symmetric_eigen(&mmt);

        // Sort by eigenvalue descending
        let mut indexed: Vec<(usize, f64)> = eigenvalues
            .iter()
            .enumerate()
            .map(|(i, &v)| (i, v))
            .collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let mut singular_values = Vec::new();
        let mut v_columns: Vec<Vec<f64>> = Vec::new(); // each is length num_cols

        for &(idx, eval) in &indexed {
            if eval < 1e-10 {
                continue;
            }
            let sigma = eval.sqrt();
            singular_values.push(sigma);

            // u_j = column idx of U
            let u_j: Vec<f64> = (0..num_rows).map(|i| u_mat[i][idx]).collect();

            // v_j = (1/sigma) * M^T * u_j
            let mut v_j = vec![0.0; num_cols];
            for c in 0..num_cols {
                for r in 0..num_rows {
                    v_j[c] += mt[c][r] * u_j[r];
                }
                v_j[c] /= sigma;
            }
            v_columns.push(v_j);
        }

        (singular_values, v_columns)
    }

    #[test]
    fn basis_search() {
        let p = 251u64;
        let a_curve = 1u64;
        let b_curve = 1u64;

        eprintln!("\n  ╔═══════════════════════════════════════════════════════════════╗");
        eprintln!("  ║  BASIS SEARCH: Sparsity of DLP in eigenvalue bases          ║");
        eprintln!("  ╚═══════════════════════════════════════════════════════════════╝");

        // === Step 1: Setup ===
        eprintln!("\n  ── Step 1: Curve setup ──");
        let points = enumerate_curve(a_curve, b_curve, p);
        let n_points = points.len();
        eprintln!("  curve: y² = x³ + x + 1 (mod {})", p);
        eprintln!("  points on curve: {}", n_points);

        // Use a prime-order subgroup for proper ECDSA arithmetic
        let (g, n_order) = find_prime_subgroup_generator(&points, a_curve, p);
        let n = n_order as u64;
        eprintln!("  generator: {:?}", g);
        eprintln!("  subgroup order n = {} (prime: {})", n, is_prime(n));
        eprintln!(
            "  full group order: {} = {} × {}",
            n_points,
            n_points as u64 / n,
            n
        );

        // d_true must be in [1, n-1]
        let d_true = 42u64 % (n - 1) + 1; // ensures d_true in [1, n-1]
        let q_pub = scalar_mul(d_true, g, a_curve, p);
        eprintln!("  d_true = {}, Q = {:?}", d_true, q_pub);

        // Generate 20 ECDSA signatures
        let num_sigs = 20;
        let mut sigs = Vec::new();
        for i in 0..num_sigs {
            let (r, s, h) = sign(d_true, i, g, n, a_curve, p);
            sigs.push((r, s, h));
        }
        eprintln!("  generated {} signatures", num_sigs);

        // Verify d_true is consistent with all signatures
        let all_consistent = sigs
            .iter()
            .all(|&(r, s, h)| sig_consistent(d_true, r, s, h, g, n, a_curve, p));
        eprintln!("  d_true consistent with all sigs: {}", all_consistent);
        assert!(
            all_consistent,
            "d_true must be consistent with all signatures"
        );

        // === Step 2: Build Constraint Matrix M ===
        eprintln!(
            "\n  ── Step 2: Constraint matrix M ({} × {}) ──",
            num_sigs, n
        );
        let m_matrix = build_constraint_matrix(&sigs, g, n, a_curve, p);

        // Count 1s per row and find all-1 column
        let mut total_ones = 0usize;
        for (i, row) in m_matrix.iter().enumerate() {
            let ones: usize = row.iter().filter(|&&v| v > 0.5).count();
            total_ones += ones;
            if i < 5 {
                eprintln!("    row {}: {} ones", i, ones);
            }
        }
        eprintln!(
            "    ... total ones in M: {} (avg {:.1}/row)",
            total_ones,
            total_ones as f64 / num_sigs as f64
        );

        // Check which column(s) have all 1s
        let mut all_one_cols = Vec::new();
        for d_c in 0..n as usize {
            let all = m_matrix.iter().all(|row| row[d_c] > 0.5);
            if all {
                all_one_cols.push(d_c);
            }
        }
        eprintln!("    columns with all 1s: {:?}", all_one_cols);
        assert!(
            all_one_cols.contains(&(d_true as usize)),
            "d_true column must have all 1s"
        );

        // === Step 3: SVD of M ===
        eprintln!("\n  ── Step 3: SVD of M (via M·M^T eigendecomp) ──");
        let (sv_m, v_cols_m) = thin_svd(&m_matrix);
        eprintln!("    rank (nonzero singular values): {}", sv_m.len());
        for (i, &sv) in sv_m.iter().enumerate().take(10) {
            eprintln!("    σ_{} = {:.6}", i, sv);
        }
        if sv_m.len() > 10 {
            eprintln!("    ... ({} total)", sv_m.len());
        }

        // === Step 4: Express e_{d_true} in SVD basis ===
        eprintln!("\n  ── Step 4: Sparsity of e_{{d_true}} in SVD basis of M ──");
        let n_usize = n as usize;

        // e_{d_true} in standard basis
        let mut e_dtrue = vec![0.0f64; n_usize];
        e_dtrue[d_true as usize] = 1.0;

        // Project onto SVD basis: c_j = V_j[d_true]
        let svd_coeffs: Vec<f64> = v_cols_m.iter().map(|v_j| v_j[d_true as usize]).collect();

        let thresholds = [0.50, 0.80, 0.90, 0.99];
        let svd_energy = energy_capture(&svd_coeffs, &thresholds);
        eprintln!("    energy capture (SVD basis of M):");
        for (i, &t) in thresholds.iter().enumerate() {
            eprintln!("      {:.0}%: {} coefficients", t * 100.0, svd_energy[i]);
        }

        // === Step 5: Sparsity in multiple bases ===
        eprintln!("\n  ── Step 5: Sparsity in multiple bases ──");
        let sqrt_n = (n as f64).sqrt().ceil() as usize;
        eprintln!("    √n = {}", sqrt_n);

        // (a) Standard basis — trivially 1-sparse
        let std_energy = energy_capture(&e_dtrue, &thresholds);
        eprintln!("\n    (a) Standard basis:");
        for (i, &t) in thresholds.iter().enumerate() {
            eprintln!("        {:.0}%: {} coefficients", t * 100.0, std_energy[i]);
        }

        // (b) DFT basis
        let dft_coeffs = dft_coefficients(&e_dtrue, n_usize);
        let dft_energy = energy_capture(&dft_coeffs, &thresholds);
        eprintln!("\n    (b) DFT basis:");
        for (i, &t) in thresholds.iter().enumerate() {
            eprintln!("        {:.0}%: {} coefficients", t * 100.0, dft_energy[i]);
        }

        // (c) SVD basis of M (already computed)
        eprintln!("\n    (c) SVD basis of M:");
        for (i, &t) in thresholds.iter().enumerate() {
            eprintln!("        {:.0}%: {} coefficients", t * 100.0, svd_energy[i]);
        }

        // (d) SVD basis of spectral constraint matrix S
        eprintln!("\n    building spectral matrix S...");
        let s_matrix = build_spectral_matrix(&sigs, g, n, a_curve, p);
        let (sv_s, v_cols_s) = thin_svd(&s_matrix);
        eprintln!("    S rank: {}", sv_s.len());
        let spec_coeffs: Vec<f64> = v_cols_s.iter().map(|v_j| v_j[d_true as usize]).collect();
        let spec_energy = energy_capture(&spec_coeffs, &thresholds);
        eprintln!("\n    (d) SVD basis of spectral S:");
        for (i, &t) in thresholds.iter().enumerate() {
            eprintln!("        {:.0}%: {} coefficients", t * 100.0, spec_energy[i]);
        }

        // (e) Hadamard basis
        let had_coeffs = hadamard_coefficients(&e_dtrue, n_usize);
        let had_energy = energy_capture(&had_coeffs, &thresholds);
        eprintln!("\n    (e) Hadamard basis:");
        for (i, &t) in thresholds.iter().enumerate() {
            eprintln!("        {:.0}%: {} coefficients", t * 100.0, had_energy[i]);
        }

        // (f) Random orthogonal basis
        let q_rand = random_orthogonal(n_usize, 12345);
        // Project e_{d_true} into random basis: coefficients = Q^T · e_{d_true} = column d_true of Q^T = row d_true of Q ... no.
        // Q is n×n where columns are basis vectors. c = Q^T · e_{d_true} => c_j = Q[d_true][j]...
        // Actually Q[i][j] = i-th component of j-th basis vector.
        // c_j = <q_j, e_{d_true}> = q_j[d_true] = Q[d_true][j]
        let rand_coeffs: Vec<f64> = (0..n_usize).map(|j| q_rand[d_true as usize][j]).collect();
        let rand_energy = energy_capture(&rand_coeffs, &thresholds);
        eprintln!("\n    (f) Random orthogonal basis:");
        for (i, &t) in thresholds.iter().enumerate() {
            eprintln!("        {:.0}%: {} coefficients", t * 100.0, rand_energy[i]);
        }

        // Summary table
        eprintln!("\n    ┌──────────────────────┬──────┬──────┬──────┬──────┐");
        eprintln!("    │ Basis                │  50% │  80% │  90% │  99% │");
        eprintln!("    ├──────────────────────┼──────┼──────┼──────┼──────┤");
        let all_results = [
            ("Standard", &std_energy),
            ("DFT", &dft_energy),
            ("SVD of M", &svd_energy),
            ("SVD of spectral S", &spec_energy),
            ("Hadamard", &had_energy),
            ("Random orthogonal", &rand_energy),
        ];
        for (name, res) in &all_results {
            eprintln!(
                "    │ {:20} │ {:4} │ {:4} │ {:4} │ {:4} │",
                name, res[0], res[1], res[2], res[3]
            );
        }
        eprintln!("    └──────────────────────┴──────┴──────┴──────┴──────┘");
        eprintln!(
            "    (√n = {} — any basis < √n at 90% would be a signal)",
            sqrt_n
        );

        let any_signal = all_results.iter().skip(1).any(|(name, res)| {
            let is_signal = res[2] < sqrt_n && *name != "Standard";
            if is_signal {
                eprintln!(
                    "    *** SIGNAL: {} has {}<√n={} at 90% ***",
                    name, res[2], sqrt_n
                );
            }
            is_signal
        });

        // === Step 6: Precomputed Topology ===
        eprintln!("\n  ── Step 6: Precomputed topology (basis transfer) ──");
        let d_other = if n > 137 { 137u64 } else { (137 % (n - 1)) + 1 };
        let q_pub2 = scalar_mul(d_other, g, a_curve, p);
        eprintln!("    d_other = {}, Q' = {:?}", d_other, q_pub2);

        // Generate new signatures for d_other
        let mut sigs2 = Vec::new();
        for i in 0..num_sigs {
            let (r, s, h) = sign(d_other, i + 100, g, n, a_curve, p);
            sigs2.push((r, s, h));
        }

        // Build constraint matrix for d_other
        let m_matrix2 = build_constraint_matrix(&sigs2, g, n, a_curve, p);

        // Express e_{d_other} in V basis (computed for d=42)
        let mut e_dother = vec![0.0f64; n_usize];
        e_dother[d_other as usize] = 1.0;

        let transfer_coeffs: Vec<f64> = v_cols_m.iter().map(|v_j| v_j[d_other as usize]).collect();
        let transfer_energy = energy_capture(&transfer_coeffs, &thresholds);
        eprintln!(
            "    e_{{d_other={}}} in SVD basis computed for d={}:",
            d_other, d_true
        );
        for (i, &t) in thresholds.iter().enumerate() {
            eprintln!(
                "      {:.0}%: {} coefficients",
                t * 100.0,
                transfer_energy[i]
            );
        }

        // Also compute SVD of M' (for d_other) and check e_{d_other} in its own basis
        let (_sv_m2, v_cols_m2) = thin_svd(&m_matrix2);
        let own_coeffs: Vec<f64> = v_cols_m2.iter().map(|v_j| v_j[d_other as usize]).collect();
        let own_energy = energy_capture(&own_coeffs, &thresholds);
        eprintln!("    e_{{d_other={}}} in its OWN SVD basis:", d_other);
        for (i, &t) in thresholds.iter().enumerate() {
            eprintln!("      {:.0}%: {} coefficients", t * 100.0, own_energy[i]);
        }

        let basis_transfers = transfer_energy[2] <= own_energy[2] + 2; // within tolerance
        eprintln!(
            "\n    basis transfer: {}",
            if basis_transfers {
                "YES — basis computed for d=42 works for d=137"
            } else {
                "NO — basis is key-specific, no precomputation advantage"
            }
        );

        // === Step 7: Cost Analysis ===
        eprintln!("\n  ── Step 7: Cost analysis ──");
        let cost_m = (num_sigs as u64) * n; // scalar_mul operations for M
        let cost_svd = (num_sigs as u64) * (num_sigs as u64) * n; // M·M^T plus eigendecomp
        let cost_express = sv_m.len() as u64; // dot products for projection
        let bsgs_cost = sqrt_n as u64;

        eprintln!("    cost of building M: {} scalar_mul ops", cost_m);
        eprintln!("    cost of SVD (M·M^T + eigendecomp): ~{} ops", cost_svd);
        eprintln!(
            "    cost of expressing e_d in SVD basis: {} dot products",
            cost_express
        );
        eprintln!("    total precomputation: ~{} ops", cost_m + cost_svd);
        eprintln!("    per-query cost (given basis): {} ops", cost_express);
        eprintln!("    BSGS: {} ops (no precomputation)", bsgs_cost);
        eprintln!();

        let precomp_plus_query = cost_m + cost_svd + cost_express;
        eprintln!(
            "    precomp + query ({}) vs BSGS ({}): {}",
            precomp_plus_query,
            bsgs_cost,
            if precomp_plus_query < bsgs_cost {
                "CHEAPER — but this means the basis search found something"
            } else {
                "MORE EXPENSIVE — no advantage over BSGS"
            }
        );

        // Even if per-query is cheap, is it cheaper than BSGS amortized?
        // Per-query with precomputed basis: rank(M) dot products
        // BSGS per query: √n
        eprintln!(
            "    per-query only: basis={} vs BSGS={}",
            cost_express, bsgs_cost
        );
        if cost_express < bsgs_cost {
            eprintln!(
                "    per-query is cheaper, BUT requires {} precomp",
                cost_m + cost_svd
            );
            let breakeven = (cost_m + cost_svd) / (bsgs_cost - cost_express).max(1);
            eprintln!("    break-even after {} queries", breakeven);
        }

        // === Step 8: Multi-Signature Basis Evolution ===
        eprintln!("\n  ── Step 8: Singular value evolution ──");
        for num_s in [1, 2, 5, 10, 20] {
            let sub_sigs: Vec<(u64, u64, u64)> = sigs[..num_s].to_vec();
            let sub_m = build_constraint_matrix(&sub_sigs, g, n, a_curve, p);
            let (sub_sv, sub_v) = thin_svd(&sub_m);

            // Check sparsity of e_{d_true} in this sub-basis
            let sub_coeffs: Vec<f64> = sub_v.iter().map(|v_j| v_j[d_true as usize]).collect();
            let sub_energy = energy_capture(&sub_coeffs, &[0.90]);

            eprintln!(
                "    {} sig(s): rank={}, σ_max={:.4}, 90% energy={} coeffs, σ: [{}]",
                num_s,
                sub_sv.len(),
                sub_sv.first().copied().unwrap_or(0.0),
                sub_energy[0],
                sub_sv
                    .iter()
                    .take(5)
                    .map(|s| format!("{:.3}", s))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        // === VERDICT ===
        eprintln!("\n  ╔═══════════════════════════════════════════════════════════════╗");
        eprintln!("  ║  VERDICT                                                     ║");
        eprintln!("  ╚═══════════════════════════════════════════════════════════════╝");

        if any_signal && !basis_transfers {
            eprintln!("  APPARENT signal: SVD of M concentrates e_{{d_true}} below √n.");
            eprintln!("  BUT this is CIRCULAR. The constraint matrix M was built from");
            eprintln!("  signatures of d_true. The d_true column is the UNIQUE all-1s");
            eprintln!("  column. The SVD naturally picks up this distinguished column");
            eprintln!("  as a dominant direction — it's the signal the matrix encodes.");
            eprintln!();
            eprintln!("  The KEY TEST (Step 6) confirms: the basis does NOT transfer.");
            eprintln!("  A basis computed for d=43 spreads e_{{d=46}} across all components.");
            eprintln!("  The concentration is key-specific, not curve-specific.");
            eprintln!();
            eprintln!("  WHY the SVD \"works\" for d_true:");
            eprintln!("  M has ~1 one per row, plus the all-1s column at d_true.");
            eprintln!("  The all-1s column contributes (1,1,...,1)/√20 to the column space.");
            eprintln!("  The top right singular vector aligns with this direction,");
            eprintln!("  so V_0[d_true] is large. This is the matrix ENCODING the answer,");
            eprintln!("  not a basis DISCOVERING it.");
            eprintln!();
            eprintln!("  Cost analysis confirms: building M costs n·sigs scalar muls.");
            eprintln!("  That's O(n) — worse than BSGS's O(√n).");
            eprintln!("  The SVD adds cost without reducing it.");
            eprintln!();
            eprintln!("  NEGATIVE RESULT: No non-trivial basis concentrates DLP");
            eprintln!("  information without first computing it. The sparsity of e_{{d_true}}");
            eprintln!("  in the SVD basis is an artifact of the constraint matrix");
            eprintln!("  already containing the answer in its column structure.");
            eprintln!("  The DLP delta is sparse in exactly one basis: the one indexed");
            eprintln!("  by the private key. Every other basis spreads it — unless");
            eprintln!("  the basis was computed FROM the key, which is circular.");
        } else if any_signal && basis_transfers {
            eprintln!("  SIGNAL: A basis concentrates DLP information AND transfers");
            eprintln!("  across keys. This warrants investigation at larger scales.");
        } else {
            eprintln!("  NO basis tested concentrates DLP information below √n.");
            eprintln!("  The standard basis (brute force) remains the sparsest representation.");
            eprintln!("  DFT spreads maximally. SVD spreads nearly as much.");
            eprintln!("  The DLP delta is sparse in exactly one basis: the one indexed");
            eprintln!("  by the private key. Every other basis must spread it.");
            eprintln!("  This is the information-theoretic content of the DLP hardness.");
        }

        eprintln!();
    }
}
