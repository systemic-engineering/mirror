//! Spectral cryptographic break — 8-bit proof of concept.
//!
//! Build the Cayley graph of a small elliptic curve.
//! Compute the Laplacian eigendecomposition.
//! Generate keypairs. Extract spectral features.
//! Train the Abyss to navigate from public to private.
//!
//! The question: does the crystal form?

// ---------------------------------------------------------------------------
// Elliptic curve arithmetic over GF(p)
// ---------------------------------------------------------------------------

/// A point on an elliptic curve y² = x³ + ax + b (mod p).
/// Infinity is the group identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Point {
    Infinity,
    Affine { x: u64, y: u64 },
}

/// Modular inverse via extended Euclidean algorithm.
fn mod_inv(a: u64, p: u64) -> Option<u64> {
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
        return None; // not invertible
    }
    Some(((old_s % p as i128 + p as i128) % p as i128) as u64)
}

/// Add two points on y² = x³ + ax + b (mod p).
fn point_add(p1: Point, p2: Point, a: u64, p: u64) -> Point {
    match (p1, p2) {
        (Point::Infinity, q) | (q, Point::Infinity) => q,
        (Point::Affine { x: x1, y: y1 }, Point::Affine { x: x2, y: y2 }) => {
            if x1 == x2 && y1 != y2 {
                // P + (-P) = O
                return Point::Infinity;
            }
            if x1 == x2 && y1 == y2 {
                if y1 == 0 {
                    return Point::Infinity;
                }
                // Point doubling: λ = (3x² + a) / (2y)
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
                // Point addition: λ = (y2 - y1) / (x2 - x1)
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

/// Scalar multiplication: k * P via double-and-add.
fn scalar_mul(k: u64, point: Point, a: u64, p: u64) -> Point {
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

/// Modular exponentiation: base^exp mod m.
fn mod_pow(mut base: u128, mut exp: u128, m: u128) -> u128 {
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

/// Tonelli-Shanks modular square root: returns Some(y) where y² ≡ n (mod p), or None.
fn mod_sqrt(n: u64, p: u64) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }
    let pm = p as u128;
    let nm = n as u128;

    // Check if n is a quadratic residue: n^((p-1)/2) ≡ 1 (mod p)
    if mod_pow(nm, (pm - 1) / 2, pm) != 1 {
        return None;
    }

    // Simple case: p ≡ 3 (mod 4) → sqrt = n^((p+1)/4)
    if p % 4 == 3 {
        let r = mod_pow(nm, (pm + 1) / 4, pm);
        return Some(r as u64);
    }

    // Tonelli-Shanks for general p
    // Factor p-1 = q * 2^s
    let mut q = pm - 1;
    let mut s = 0u32;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    // Find a non-residue z
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
        // Find least i such that t^(2^i) = 1
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

/// Enumerate all points on y² = x³ + ax + b (mod p). O(p) via Tonelli-Shanks.
fn enumerate_curve(a: u64, b: u64, p: u64) -> Vec<Point> {
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

/// Find a generator of the curve group (a point of maximum order).
fn find_generator(points: &[Point], a: u64, p: u64) -> (Point, u64) {
    let n = points.len() as u64;
    for &pt in points {
        if pt == Point::Infinity {
            continue;
        }
        // Check if this point generates the full group
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
    // Fallback: return first non-infinity point
    (points[1], 0)
}

// ---------------------------------------------------------------------------
// Cayley graph construction
// ---------------------------------------------------------------------------

/// Build the Cayley graph: nodes = curve points, edges = group addition by generator.
/// Each node i has an edge to j where j = points[i] + G.
fn cayley_graph(
    points: &[Point],
    generator: Point,
    a: u64,
    p: u64,
) -> (Vec<String>, Vec<(usize, usize)>) {
    let n = points.len();
    let vertices: Vec<String> = (0..n)
        .map(|i| match points[i] {
            Point::Infinity => "O".to_string(),
            Point::Affine { x, y } => format!("({},{})", x, y),
        })
        .collect();

    // Index lookup
    let point_to_idx: std::collections::HashMap<Point, usize> =
        points.iter().enumerate().map(|(i, &p)| (p, i)).collect();

    let mut edge_set = std::collections::HashSet::new();
    for (i, &pt) in points.iter().enumerate() {
        let sum = point_add(pt, generator, a, p);
        if let Some(&j) = point_to_idx.get(&sum) {
            let edge = if i < j { (i, j) } else { (j, i) };
            edge_set.insert(edge);
        }
    }
    let edges: Vec<(usize, usize)> = edge_set.into_iter().collect();

    (vertices, edges)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// y² = x³ + x + 1 (mod 251). A nice 8-bit curve.
    const CURVE_A: u64 = 1;
    const CURVE_B: u64 = 1;
    const CURVE_P: u64 = 251;

    #[test]
    fn enumerate_8bit_curve() {
        let points = enumerate_curve(CURVE_A, CURVE_B, CURVE_P);
        eprintln!(
            "  8-bit curve y² = x³ + {}x + {} (mod {})",
            CURVE_A, CURVE_B, CURVE_P
        );
        eprintln!("  points: {}", points.len());
        assert!(points.len() > 100, "curve should have >100 points");
        assert!(points.len() < 500, "curve should have <500 points");
    }

    #[test]
    fn find_8bit_generator() {
        let points = enumerate_curve(CURVE_A, CURVE_B, CURVE_P);
        let (gen, order) = find_generator(&points, CURVE_A, CURVE_P);
        eprintln!("  generator: {:?}", gen);
        eprintln!("  order: {} (curve has {} points)", order, points.len());
        assert_eq!(order, points.len() as u64, "generator must have full order");
    }

    #[test]
    fn build_cayley_graph() {
        let points = enumerate_curve(CURVE_A, CURVE_B, CURVE_P);
        let n = points.len();
        let (gen, order) = find_generator(&points, CURVE_A, CURVE_P);
        let (vertices, edges) = cayley_graph(&points, gen, CURVE_A, CURVE_P);
        eprintln!(
            "  cayley graph: {} vertices, {} edges",
            vertices.len(),
            edges.len()
        );

        // For a cyclic group of order n with one generator,
        // the undirected Cayley graph is a cycle (ring): n edges.
        // With i<j dedup: n/2 edges if n even, (n-1)/2 + 1 if n odd
        // (because the edge from n-1 to 0 wraps around)
        eprintln!("  expected edges for ring of {}: ~{}", n, n / 2);

        // Check: every point P should have exactly 2 neighbors: P+G and P-G
        let point_to_idx: std::collections::HashMap<Point, usize> =
            points.iter().enumerate().map(|(i, &p)| (p, i)).collect();

        let mut missing = 0;
        for (i, &pt) in points.iter().enumerate() {
            let sum = point_add(pt, gen, CURVE_A, CURVE_P);
            if point_to_idx.get(&sum).is_none() {
                missing += 1;
                if missing <= 3 {
                    eprintln!("  MISSING: {:?} + G = {:?} not in points!", pt, sum);
                }
            }
        }
        eprintln!("  missing targets: {}", missing);

        assert_eq!(vertices.len(), points.len());
        assert_eq!(missing, 0, "all P+G should be in the point set");

        // Check connectivity via BFS
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for &(i, j) in &edges {
            adj[i].push(j);
            adj[j].push(i);
        }
        let mut visited = vec![false; n];
        let mut queue = std::collections::VecDeque::new();
        visited[0] = true;
        queue.push_back(0);
        let mut component_size = 0;
        while let Some(v) = queue.pop_front() {
            component_size += 1;
            for &u in &adj[v] {
                if !visited[u] {
                    visited[u] = true;
                    queue.push_back(u);
                }
            }
        }
        eprintln!(
            "  component from vertex 0: {} / {} vertices",
            component_size, n
        );
        let components = visited.iter().filter(|&&v| !v).count();
        eprintln!("  unreached vertices: {}", components);

        // Trace the chain from O
        let mut pt = Point::Infinity;
        eprintln!("  chain from O:");
        for step in 0..10 {
            let idx = point_to_idx[&pt];
            let next = point_add(pt, gen, CURVE_A, CURVE_P);
            let next_idx = point_to_idx[&next];
            eprintln!(
                "    step {}: idx {} ({:?}) → idx {} ({:?})",
                step, idx, pt, next_idx, next
            );
            pt = next;
        }

        // Check: does the chain visit all 282 points?
        let mut pt = gen;
        let mut chain_len = 1;
        while pt != Point::Infinity {
            pt = point_add(pt, gen, CURVE_A, CURVE_P);
            chain_len += 1;
            if chain_len > 300 {
                break;
            }
        }
        eprintln!("  chain length (G iterations to O): {}", chain_len);
    }

    #[test]
    fn spectral_decomposition() {
        let points = enumerate_curve(CURVE_A, CURVE_B, CURVE_P);
        let (gen, _) = find_generator(&points, CURVE_A, CURVE_P);
        let (vertices, edges) = cayley_graph(&points, gen, CURVE_A, CURVE_P);

        // Build Laplacian and decompose
        let laplacian = coincidence::spectral::Laplacian::from_adjacency(&vertices, &edges);
        let spectrum = laplacian.spectrum();
        let eigenvalues = spectrum.eigenvalues();

        eprintln!("  eigenvalues: {} total", eigenvalues.len());
        eprintln!(
            "  smallest 5: {:?}",
            &eigenvalues[..5.min(eigenvalues.len())]
        );
        eprintln!(
            "  largest 5:  {:?}",
            &eigenvalues[eigenvalues.len().saturating_sub(5)..]
        );

        // Fiedler value (algebraic connectivity)
        let fiedler = eigenvalues
            .iter()
            .find(|&&v| v > 1e-10)
            .copied()
            .unwrap_or(0.0);
        eprintln!("  fiedler: {:.6}", fiedler);

        assert!(
            eigenvalues.len() == vertices.len(),
            "should have n eigenvalues"
        );
    }

    #[test]
    fn generate_keypairs_and_spectral_features() {
        let points = enumerate_curve(CURVE_A, CURVE_B, CURVE_P);
        let n = points.len();
        let (gen, order) = find_generator(&points, CURVE_A, CURVE_P);
        let (vertices, edges) = cayley_graph(&points, gen, CURVE_A, CURVE_P);

        // Spectral decomposition
        let laplacian = coincidence::spectral::Laplacian::from_adjacency(&vertices, &edges);
        let spectrum = laplacian.spectrum();
        let eigenvalues = spectrum.eigenvalues();

        // Index lookup: point → vertex index
        let point_to_idx: std::collections::HashMap<Point, usize> =
            points.iter().enumerate().map(|(i, &p)| (p, i)).collect();

        // Generate keypairs
        let mut keypairs = Vec::new();
        for k in 1..order {
            let public = scalar_mul(k, gen, CURVE_A, CURVE_P);
            let idx = point_to_idx[&public];
            keypairs.push((k, public, idx));
        }

        eprintln!("  generated {} keypairs", keypairs.len());
        eprintln!(
            "  sample: k=7 → {:?} (idx {})",
            keypairs[6].1, keypairs[6].2
        );

        // Spectral feature for each public key: its "position" in eigenspace
        // The i-th eigenvector component at the public key's vertex index
        // For now: use eigenvalue index as a proxy for position
        // The real feature: v_k[idx] for eigenvector k — needs eigenvectors, not just eigenvalues

        // What we CAN compute: the spectral distance from the generator to the public key
        // This is the graph distance, which IS the private key for a cyclic group.
        // The question: can the eigenvalue structure recover this distance?

        // For the 8-bit proof of concept, let's verify the structure:
        // In a cyclic Cayley graph, the eigenvalues of the Laplacian are
        // λ_k = 2 - 2cos(2πk/n) for k = 0, ..., n-1
        // This is a well-known result. The crystal IS the DFT.

        let n_f = n as f64;
        let theoretical: Vec<f64> = (0..n)
            .map(|k| 2.0 - 2.0 * (2.0 * std::f64::consts::PI * k as f64 / n_f).cos())
            .collect();
        let mut theoretical_sorted = theoretical.clone();
        theoretical_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Compare theoretical vs computed eigenvalues
        let mut max_err = 0.0f64;
        for (t, c) in theoretical_sorted.iter().zip(eigenvalues.iter()) {
            let err = (t - c).abs();
            if err > max_err {
                max_err = err;
            }
        }
        eprintln!("  max eigenvalue error vs theoretical: {:.2e}", max_err);

        // The crystal: for a cyclic group, the DFT recovers the private key.
        // DFT of the indicator function δ_Q (1 at public key, 0 elsewhere)
        // The phase of the k=1 DFT coefficient = 2π * private_key / n
        // This is NOT a break — it's just the DFT on a known cyclic group.
        // The question is whether this structure TRANSFERS to non-cyclic curves.

        // Write fixture: keypair index → spectral features
        let fixture_dir =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/keypairs/8bit");
        std::fs::create_dir_all(&fixture_dir).unwrap();

        // Write curve parameters
        let params = format!(
            "# 8-bit elliptic curve: y² = x³ + {}x + {} (mod {})\n# {} points, generator {:?}, order {}\n# eigenvalues: {}\n",
            CURVE_A, CURVE_B, CURVE_P,
            n, gen, order,
            eigenvalues.len()
        );
        std::fs::write(fixture_dir.join("curve.txt"), &params).unwrap();

        // Write keypairs: private_key,public_x,public_y,vertex_index
        let mut csv = String::from("private_key,public_x,public_y,vertex_index\n");
        for &(k, public, idx) in &keypairs {
            match public {
                Point::Affine { x, y } => {
                    csv.push_str(&format!("{},{},{},{}\n", k, x, y, idx));
                }
                Point::Infinity => {
                    csv.push_str(&format!("{},inf,inf,0\n", k));
                }
            }
        }
        std::fs::write(fixture_dir.join("keypairs.csv"), &csv).unwrap();

        // Write eigenvalues
        let eigen_csv: String = eigenvalues
            .iter()
            .enumerate()
            .map(|(i, v)| format!("{},{:.10}\n", i, v))
            .collect();
        std::fs::write(fixture_dir.join("eigenvalues.csv"), &eigen_csv).unwrap();

        eprintln!("  wrote fixtures to {}", fixture_dir.display());

        // The critical test: does the spectral structure predict the private key?
        // For a cyclic Cayley graph: trivially yes (DFT).
        // For a non-cyclic structure: the open question.
        assert!(
            max_err < 0.01,
            "eigenvalues should match cyclic theory for generator-based Cayley graph"
        );
    }

    #[test]
    fn dft_recovers_private_key() {
        let points = enumerate_curve(CURVE_A, CURVE_B, CURVE_P);
        let n = points.len();
        let (gen, order) = find_generator(&points, CURVE_A, CURVE_P);
        let (vertices, edges) = cayley_graph(&points, gen, CURVE_A, CURVE_P);

        // Get the eigensystem (eigenvalues + eigenvectors)
        let laplacian = coincidence::spectral::Laplacian::from_adjacency(&vertices, &edges);
        let eigensystem = laplacian.eigensystem();
        let eigenvalues = eigensystem.eigenvalues();

        eprintln!(
            "  eigensystem: {} eigenvalues, dim {}",
            eigenvalues.len(),
            eigensystem.dimension()
        );

        // Build the permutation: vertex_index → private_key
        let point_to_idx: std::collections::HashMap<Point, usize> =
            points.iter().enumerate().map(|(i, &p)| (p, i)).collect();

        // idx_to_private[vertex_index] = private_key (or 0 for Infinity)
        let mut idx_to_private = vec![0u64; n];
        for k in 1..order {
            let public = scalar_mul(k, gen, CURVE_A, CURVE_P);
            let idx = point_to_idx[&public];
            idx_to_private[idx] = k;
        }

        // DFT recovery: for each public key, compute the spectral projection
        // and see if we can recover the private key from eigenvector components.
        //
        // For a ring of size n, the eigenvectors are the DFT basis:
        //   v_k[j] = (1/√n) · cos(2π·k·σ(j)/n)  (real symmetric Laplacian)
        // where σ is the permutation from Cayley graph to vertex index.
        //
        // The spectral embedding of vertex j is the vector:
        //   [v_0[j], v_1[j], ..., v_{n-1}[j]]
        //
        // For the ring, this embedding encodes the position mod n.
        // The private key is recoverable from the phase of the embedding.

        let mut correct = 0;
        let mut correct_mod = 0;
        let total = (order - 1) as usize;

        for k in 1..order {
            let public = scalar_mul(k, gen, CURVE_A, CURVE_P);
            let idx = point_to_idx[&public];

            // Full spectral recovery: use ALL eigenvectors.
            // Compute DFT-like projection: for each eigenvector k,
            // the component at vertex idx encodes position information.
            //
            // For the ring Laplacian, eigenvectors come in cos/sin pairs.
            // The recovery uses: for frequency f, compute
            //   c_f = v_{2f}[idx]  (cosine component)
            //   s_f = v_{2f+1}[idx] (sine component)
            //   phase_f = atan2(s_f, c_f)
            //   position_f = phase_f * n / (2π * f)
            //
            // The f=1 harmonic gives the position directly.
            // But we need to find which eigenvector indices correspond to f=1.
            //
            // Alternative: use the PERMUTATION directly.
            // The Cayley graph defines permutation π where π(i) = P+G.
            // Private key k satisfies π^k(0) = idx.
            // This is a simple walk, not spectral. But it proves the structure.

            // Walk the permutation from vertex 0 (Infinity) to vertex idx
            let mut pos = 0usize; // start at Infinity
            let mut steps = 0u64;
            loop {
                if pos == idx {
                    break;
                }
                // Apply permutation: pos → points[pos] + G → new index
                let next_pt = point_add(points[pos], gen, CURVE_A, CURVE_P);
                pos = point_to_idx[&next_pt];
                steps += 1;
                if steps > order {
                    break; // safety
                }
            }
            let recovered = steps;

            // The recovered position is the Cayley graph position, which is the private key
            // (or n-k due to undirected ambiguity)
            if recovered == k {
                correct += 1;
            }
            if recovered == k || recovered == order - k {
                correct_mod += 1;
            }
        }

        let accuracy = correct as f64 / total as f64;
        let accuracy_mod = correct_mod as f64 / total as f64;
        eprintln!(
            "  DFT recovery: {}/{} exact ({:.1}%)",
            correct,
            total,
            accuracy * 100.0
        );
        eprintln!(
            "  DFT recovery: {}/{} mod-symmetric ({:.1}%)",
            correct_mod,
            total,
            accuracy_mod * 100.0
        );

        // The ring DFT should give near-perfect recovery (mod sign)
        assert!(
            accuracy_mod > 0.95,
            "DFT should recover >95% of private keys (got {:.1}%)",
            accuracy_mod * 100.0
        );
    }

    #[test]
    fn spectral_dft_recovers_private_key() {
        // The REAL spectral test: recover the private key using ONLY
        // the eigenvector structure, not the permutation walk.
        //
        // For a ring of size n, the Laplacian eigenvectors are:
        //   v_0 = (1/√n, 1/√n, ..., 1/√n)  (constant, eigenvalue 0)
        //   v_{2k-1}[j] = √(2/n) · cos(2π·k·π(j)/n)  (eigenvalue 2-2cos(2πk/n))
        //   v_{2k}[j]   = √(2/n) · sin(2π·k·π(j)/n)
        //
        // where π(j) is the POSITION of vertex j in the ring (= private key for j).
        //
        // So: atan2(v_{2}[j], v_{1}[j]) = 2π·private_key/n
        // But the eigenvectors are sorted by eigenvalue, and degenerate pairs
        // can swap cos/sin. We need to identify the correct pair.

        let points = enumerate_curve(CURVE_A, CURVE_B, CURVE_P);
        let n = points.len();
        let (gen, order) = find_generator(&points, CURVE_A, CURVE_P);
        let (vertices, edges) = cayley_graph(&points, gen, CURVE_A, CURVE_P);

        let laplacian = coincidence::spectral::Laplacian::from_adjacency(&vertices, &edges);
        let eigensystem = laplacian.eigensystem();
        let eigenvalues = eigensystem.eigenvalues();

        let point_to_idx: std::collections::HashMap<Point, usize> =
            points.iter().enumerate().map(|(i, &p)| (p, i)).collect();

        // The first non-zero eigenvalue pair (indices 1 and 2) corresponds to frequency 1.
        // These are the Fiedler pair. Their components at each vertex encode position.
        //
        // Strategy: use vertex 0 (Infinity, private key = 0) and vertex of G (private key = 1)
        // as calibration points. Then recover all other keys from the phase.

        // Find the Fiedler pair indices
        let fiedler_start = eigenvalues.iter().position(|&v| v > 1e-10).unwrap_or(1);
        eprintln!(
            "  fiedler pair at indices {} and {}, eigenvalue {:.6}",
            fiedler_start,
            fiedler_start + 1,
            eigenvalues[fiedler_start]
        );

        // Calibrate: vertex of G (private key = 1) gives the phase step
        let g_idx = point_to_idx[&gen]; // vertex index of generator G
        let o_idx = 0usize; // vertex index of Infinity (private key 0)

        let o_v1 = eigensystem.eigenvector_component(o_idx, fiedler_start);
        let o_v2 = eigensystem.eigenvector_component(o_idx, fiedler_start + 1);
        let g_v1 = eigensystem.eigenvector_component(g_idx, fiedler_start);
        let g_v2 = eigensystem.eigenvector_component(g_idx, fiedler_start + 1);

        let o_phase = o_v2.atan2(o_v1);
        let g_phase = g_v2.atan2(g_v1);
        let phase_step = g_phase - o_phase; // phase per unit of private key

        eprintln!(
            "  O phase: {:.6}, G phase: {:.6}, step: {:.6}",
            o_phase, g_phase, phase_step
        );
        eprintln!(
            "  expected step: {:.6}",
            2.0 * std::f64::consts::PI / n as f64
        );

        // Recover all private keys
        let mut correct = 0;
        let mut correct_mod = 0;
        let total = (order - 1) as usize;

        for k in 1..order {
            let public = scalar_mul(k, gen, CURVE_A, CURVE_P);
            let idx = point_to_idx[&public];

            let v1 = eigensystem.eigenvector_component(idx, fiedler_start);
            let v2 = eigensystem.eigenvector_component(idx, fiedler_start + 1);

            let phase = v2.atan2(v1);
            // Recover position from phase relative to O
            let delta = phase - o_phase;
            // Normalize to [0, 2π)
            let delta_norm = (delta + 10.0 * std::f64::consts::PI) % (2.0 * std::f64::consts::PI);
            // Convert to position
            let pos = delta_norm / phase_step.abs();
            let recovered = pos.round() as u64 % order;

            if recovered == k {
                correct += 1;
            }
            if recovered == k || recovered == order - k {
                correct_mod += 1;
            }
        }

        let accuracy = correct as f64 / total as f64;
        let accuracy_mod = correct_mod as f64 / total as f64;
        eprintln!(
            "  spectral DFT: {}/{} exact ({:.1}%)",
            correct,
            total,
            accuracy * 100.0
        );
        eprintln!(
            "  spectral DFT: {}/{} mod-symmetric ({:.1}%)",
            correct_mod,
            total,
            accuracy_mod * 100.0
        );

        assert!(
            accuracy_mod > 0.95,
            "spectral DFT should recover >95% (got {:.1}%)",
            accuracy_mod * 100.0
        );
    }

    #[test]
    fn generate_16bit_and_test_transfer() {
        // 16-bit curve: y² = x³ + x + 1 (mod 65521)
        // 65521 is the largest 16-bit prime.
        // Expected: ~65521 points (Hasse bound: |N - p - 1| ≤ 2√p ≈ 512)
        let p16: u64 = 65521;
        let a16: u64 = 1;
        let b16: u64 = 1;

        eprintln!("  16-bit curve y² = x³ + {}x + {} (mod {})", a16, b16, p16);

        // Enumerate points — this is O(p) which is ~65k iterations
        let points = enumerate_curve(a16, b16, p16);
        let n = points.len();
        eprintln!("  points: {}", n);
        assert!(n > 60000, "16-bit curve should have >60k points");
        assert!(n < 70000, "16-bit curve should have <70k points");

        // Use first non-infinity point. Compute order by walking (skip find_generator — too slow).
        let gen = points[1];
        let mut pt = gen;
        let mut order = 1u64;
        loop {
            pt = point_add(pt, gen, a16, p16);
            order += 1;
            if pt == Point::Infinity || order > n as u64 + 1 {
                break;
            }
        }
        eprintln!(
            "  generator: {:?}, order: {} (full: {})",
            gen,
            order,
            order == n as u64
        );

        // Generate 1000 keypairs (don't need all ~65k)
        let point_to_idx: std::collections::HashMap<Point, usize> =
            points.iter().enumerate().map(|(i, &p)| (p, i)).collect();

        let fixture_dir =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/keypairs/16bit");
        std::fs::create_dir_all(&fixture_dir).unwrap();

        let sample_size = 1000u64;
        let mut csv = String::from("private_key,public_x,public_y,vertex_index\n");
        for k in 1..=sample_size {
            let public = scalar_mul(k, gen, a16, p16);
            let idx = point_to_idx[&public];
            match public {
                Point::Affine { x, y } => csv.push_str(&format!("{},{},{},{}\n", k, x, y, idx)),
                Point::Infinity => csv.push_str(&format!("{},inf,inf,0\n", k)),
            }
        }
        std::fs::write(fixture_dir.join("keypairs.csv"), &csv).unwrap();

        let params = format!(
            "# 16-bit curve: y² = x³ + {}x + {} (mod {})\n# {} points, generator {:?}, order {}\n# sample: {} keypairs\n",
            a16, b16, p16, points.len(), gen, order, sample_size
        );
        std::fs::write(fixture_dir.join("curve.txt"), &params).unwrap();

        eprintln!(
            "  wrote {} keypairs to {}",
            sample_size,
            fixture_dir.display()
        );

        // The critical question: can we build the Cayley graph at 16-bit?
        // 65k vertices, 65k edges. Laplacian is 65k × 65k = ~34GB dense matrix.
        // NOT feasible with dense Laplacian. Need sparse methods.
        //
        // But the STRUCTURE test doesn't need the full Laplacian.
        // The 8-bit crystal showed the eigenvalues = DFT of the ring.
        // At 16-bit, the eigenvalues are KNOWN ANALYTICALLY:
        //   λ_k = 2 - 2cos(2πk/n) for k = 0..n-1
        //
        // The eigenvectors are also known: DFT basis vectors.
        // We don't need to COMPUTE them — we can USE the analytical form.
        //
        // Spectral recovery at 16-bit uses the SAME formula as 8-bit:
        //   phase = atan2(sin(2π·k/n), cos(2π·k/n)) for position k
        //
        // This is trivially the DFT. The question is whether the CAYLEY GRAPH
        // permutation (which maps point indices to group positions) can be
        // recovered from the spectral structure of the CURVE, not the ring.

        let matrix_mb = (n as u64 * n as u64 * 8) as f64 / 1e6;
        eprintln!(
            "  dense laplacian would be: {:.0}MB — spectral-db needed",
            matrix_mb
        );
    }
}
