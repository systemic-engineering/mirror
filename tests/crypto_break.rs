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

/// Enumerate all points on y² = x³ + ax + b (mod p).
fn enumerate_curve(a: u64, b: u64, p: u64) -> Vec<Point> {
    let mut points = vec![Point::Infinity];
    for x in 0..p {
        let rhs = (x * x % p * x % p + a * x % p + b) % p;
        for y in 0..p {
            if y * y % p == rhs {
                points.push(Point::Affine { x, y });
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
}
