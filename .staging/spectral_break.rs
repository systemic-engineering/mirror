//! Wire spectral-db to the Cayley graph. Let it settle.
//!
//! spectral_db.open("curve-8bit")
//! spectral_db.ingest(cayley_graph)
//! spectral_db.tick() // settle
//! spectral_db.tick()
//! // ...settled.
//! // The crystal forms at whatever pace the hardware allows.

mod curve {
    //! Elliptic curve arithmetic — shared with crypto_break.rs

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum Point {
        Infinity,
        Affine { x: u64, y: u64 },
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

    const SCHEMA: &str = "grammar @curve {\n  type = point\n}";

    #[test]
    fn spectral_db_ingests_8bit_cayley_graph() {
        let a = 1u64;
        let b = 1u64;
        let p = 251u64;

        let points = enumerate_curve(a, b, p);
        let n = points.len();
        let gen = points[1]; // first non-infinity point

        // Compute generator order
        let mut pt = gen;
        let mut order = 1u64;
        loop {
            pt = point_add(pt, gen, a, p);
            order += 1;
            if pt == Point::Infinity || order > n as u64 + 1 {
                break;
            }
        }
        eprintln!("  8-bit: {} points, generator order {}", n, order);

        // Open spectral-db
        let dir = tempfile::tempdir().unwrap();
        let db = spectral_db::SpectralDb::open(dir.path(), SCHEMA, 1e-10, 50_000_000).unwrap();

        // Ingest all points as nodes
        let point_to_idx: std::collections::HashMap<Point, usize> =
            points.iter().enumerate().map(|(i, &p)| (p, i)).collect();

        let mut oids = Vec::with_capacity(n);
        for (i, pt) in points.iter().enumerate() {
            let label = match pt {
                Point::Infinity => "O".to_string(),
                Point::Affine { x, y } => format!("({},{})", x, y),
            };
            let oid = db.insert("point", label.as_bytes()).unwrap();
            oids.push(oid);
        }
        eprintln!("  inserted {} nodes", oids.len());

        // Ingest Cayley graph edges: for each point P, connect P to P+G
        let mut edge_count = 0;
        for (i, &pt) in points.iter().enumerate() {
            let sum = point_add(pt, gen, a, p);
            if let Some(&j) = point_to_idx.get(&sum) {
                db.connect(&oids[i], &oids[j]).unwrap();
                edge_count += 1;
            }
        }
        eprintln!("  connected {} edges", edge_count);

        let (nodes, edges) = db.graph_stats();
        eprintln!("  spectral-db: {} nodes, {} edges", nodes, edges);

        // Tick until settled
        let mut ticks = 0;
        loop {
            let result = db.scheduler_tick();
            ticks += 1;
            match result.convergence {
                spectral_db::scheduler::Convergence::Settled => {
                    eprintln!("  settled in {} ticks", ticks);
                    break;
                }
                spectral_db::scheduler::Convergence::FirstTick => {
                    eprintln!("  tick {}: first", ticks);
                }
                spectral_db::scheduler::Convergence::Changed => {
                    eprintln!("  tick {}: changed", ticks);
                }
            }
            if ticks > 10 {
                break;
            }
        }

        // Compute spectral coordinates
        db.compute_spectral_coordinates();

        // Test spectral distance: G (private key 1) should be close to 2G (private key 2)
        // and far from (n/2)G (opposite side of the ring)
        let g_oid = &oids[point_to_idx[&gen]];
        let two_g = scalar_mul(2, gen, a, p);
        let two_g_oid = &oids[point_to_idx[&two_g]];
        let half_g = scalar_mul(order / 2, gen, a, p);
        let half_g_oid = &oids[point_to_idx[&half_g]];

        let dist_near = db.spectral_distance(g_oid, two_g_oid);
        let dist_far = db.spectral_distance(g_oid, half_g_oid);

        eprintln!("  spectral distance G→2G: {:?}", dist_near);
        eprintln!("  spectral distance G→(n/2)G: {:?}", dist_far);

        if let (Some(near), Some(far)) = (dist_near, dist_far) {
            eprintln!("  ratio far/near: {:.2}", far / near.max(1e-15));
            // Near should be smaller than far
            assert!(
                near < far,
                "adjacent keys should be spectrally closer than opposite keys"
            );
        }

        // Crystallize
        let crystals = db.crystallize();
        eprintln!("  crystals: {}", crystals.len());

        let status = db.status();
        eprintln!(
            "  final: {} nodes, {} edges, {} crystals, {} queries",
            status.node_count, status.edge_count, status.crystals, status.query_count
        );
    }

    #[test]
    fn spectral_db_ingests_12bit_cayley_graph() {
        // 12-bit: intermediate scale between 8-bit (282) and 16-bit (65k).
        // ~4000 points. Dense enough to test transfer, small enough to compute.
        let a = 1u64;
        let b = 1u64;
        let p = 4093u64; // largest 12-bit prime

        eprintln!("  12-bit curve y² = x³ + {}x + {} (mod {})", a, b, p);

        let points = enumerate_curve(a, b, p);
        let n = points.len();
        eprintln!("  points: {}", n);

        let gen = points[1];

        // Generator order
        let mut pt = gen;
        let mut order = 1u64;
        loop {
            pt = point_add(pt, gen, a, p);
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

        let point_to_idx: std::collections::HashMap<Point, usize> =
            points.iter().enumerate().map(|(i, &p)| (p, i)).collect();

        // Open spectral-db with 100MB budget
        let dir = tempfile::tempdir().unwrap();
        let db = spectral_db::SpectralDb::open(dir.path(), SCHEMA, 1e-10, 100_000_000).unwrap();

        // Ingest points in batches
        eprintln!("  inserting {} nodes...", n);
        let mut oids = Vec::with_capacity(n);
        for pt in &points {
            let label = match pt {
                Point::Infinity => "O".to_string(),
                Point::Affine { x, y } => format!("({},{})", x, y),
            };
            let oid = db.insert("point", label.as_bytes()).unwrap();
            oids.push(oid);
        }
        eprintln!("  inserted {} nodes", oids.len());

        // Connect Cayley graph edges
        eprintln!("  connecting edges...");
        let mut edge_count = 0;
        for (i, &pt) in points.iter().enumerate() {
            let sum = point_add(pt, gen, a, p);
            if let Some(&j) = point_to_idx.get(&sum) {
                db.connect(&oids[i], &oids[j]).unwrap();
                edge_count += 1;
            }
        }
        eprintln!("  connected {} edges", edge_count);

        let (nodes, edges) = db.graph_stats();
        eprintln!("  spectral-db: {} nodes, {} edges", nodes, edges);

        // Tick until settled
        let mut ticks = 0;
        loop {
            let result = db.scheduler_tick();
            ticks += 1;
            match result.convergence {
                spectral_db::scheduler::Convergence::Settled => {
                    eprintln!("  settled in {} ticks", ticks);
                    break;
                }
                spectral_db::scheduler::Convergence::FirstTick => {
                    eprintln!("  tick {}: first", ticks);
                }
                spectral_db::scheduler::Convergence::Changed => {
                    eprintln!("  tick {}: changed", ticks);
                }
            }
            if ticks > 10 {
                break;
            }
        }

        // Compute spectral coordinates (ego-graph based — bounded memory)
        eprintln!("  computing spectral coordinates...");
        db.compute_spectral_coordinates();

        // Test: spectral distance should distinguish near from far
        let g_oid = &oids[point_to_idx[&gen]];
        let two_g = scalar_mul(2, gen, a, p);
        let two_g_oid = &oids[point_to_idx[&two_g]];
        let far_g = scalar_mul(order / 2, gen, a, p);
        let far_g_oid = &oids[point_to_idx[&far_g]];

        let dist_near = db.spectral_distance(g_oid, two_g_oid);
        let dist_far = db.spectral_distance(g_oid, far_g_oid);

        eprintln!("  spectral distance G→2G: {:?}", dist_near);
        eprintln!("  spectral distance G→(n/2)G: {:?}", dist_far);

        if let (Some(near), Some(far)) = (dist_near, dist_far) {
            eprintln!("  ratio far/near: {:.2}", far / near.max(1e-15));
        }

        // Eigenvector distance (if coords were computed)
        let eigen_near = db.spectral_distance_eigen(g_oid, two_g_oid);
        let eigen_far = db.spectral_distance_eigen(g_oid, far_g_oid);
        eprintln!("  eigen distance G→2G: {:?}", eigen_near);
        eprintln!("  eigen distance G→(n/2)G: {:?}", eigen_far);

        if let (Some(near), Some(far)) = (eigen_near, eigen_far) {
            eprintln!("  eigen ratio far/near: {:.2}", far / near.max(1e-15));
        }

        // Generate 100 keypairs and test spectral ordering
        // If spectral distance correlates with private key distance,
        // we have the seed of a spectral break.
        let sample = 100u64;
        let mut distances: Vec<(u64, f64)> = Vec::new();
        for k in 1..=sample {
            let pub_k = scalar_mul(k, gen, a, p);
            let pub_oid = &oids[point_to_idx[&pub_k]];
            if let Some(d) = db.spectral_distance(g_oid, pub_oid) {
                distances.push((k, d));
            }
        }

        // Check monotonicity: does spectral distance grow with private key?
        let mut monotone_count = 0;
        for w in distances.windows(2) {
            if w[1].1 >= w[0].1 {
                monotone_count += 1;
            }
        }
        let monotonicity = monotone_count as f64 / (distances.len().saturating_sub(1)) as f64;
        eprintln!(
            "  monotonicity (spectral dist vs private key): {:.1}%",
            monotonicity * 100.0
        );
        eprintln!("  (50% = random, 100% = perfect correlation)");

        let status = db.status();
        eprintln!(
            "  final: {} nodes, {} edges, {} crystals",
            status.node_count, status.edge_count, status.crystals
        );
    }

    #[test]
    fn full_laplacian_12bit_dft_recovery() {
        // The REAL transfer test: does the 8-bit DFT approach
        // work at 12-bit with full-graph eigenvectors?
        // 4100 points → 4100×4100 = 134MB dense matrix. Feasible.
        let a = 1u64;
        let b = 1u64;
        let p = 4093u64;

        let points = enumerate_curve(a, b, p);
        let n = points.len();
        let gen = points[1];

        let mut pt = gen;
        let mut order = 1u64;
        loop {
            pt = point_add(pt, gen, a, p);
            order += 1;
            if pt == Point::Infinity || order > n as u64 + 1 {
                break;
            }
        }
        eprintln!("  12-bit: {} points, order {}", n, order);

        let point_to_idx: std::collections::HashMap<Point, usize> =
            points.iter().enumerate().map(|(i, &p)| (p, i)).collect();

        // Build Cayley graph
        let mut edge_set = std::collections::HashSet::new();
        let vertices: Vec<String> = (0..n)
            .map(|i| match points[i] {
                Point::Infinity => "O".to_string(),
                Point::Affine { x, y } => format!("({},{})", x, y),
            })
            .collect();

        for (i, &pt) in points.iter().enumerate() {
            let sum = point_add(pt, gen, a, p);
            if let Some(&j) = point_to_idx.get(&sum) {
                let edge = if i < j { (i, j) } else { (j, i) };
                edge_set.insert(edge);
            }
        }
        let edges: Vec<(usize, usize)> = edge_set.into_iter().collect();
        eprintln!(
            "  cayley: {} vertices, {} edges",
            vertices.len(),
            edges.len()
        );

        let matrix_mb = (n * n * 8) as f64 / 1e6;
        eprintln!("  laplacian: {:.0}MB", matrix_mb);

        // Full eigendecomposition
        eprintln!("  computing eigensystem...");
        let laplacian = coincidence::spectral::Laplacian::from_adjacency(&vertices, &edges);
        let eigensystem = laplacian.eigensystem();
        let eigenvalues = eigensystem.eigenvalues();

        let zero_count = eigenvalues.iter().filter(|&&v| v.abs() < 1e-10).count();
        eprintln!(
            "  eigenvalues: {}, components: {}",
            eigenvalues.len(),
            zero_count
        );

        // Fiedler pair DFT recovery — same method as 8-bit
        let fiedler_start = eigenvalues.iter().position(|&v| v > 1e-10).unwrap_or(1);
        eprintln!("  fiedler eigenvalue: {:.8}", eigenvalues[fiedler_start]);

        let o_v1 = eigensystem.eigenvector_component(0, fiedler_start);
        let o_v2 = eigensystem.eigenvector_component(0, fiedler_start + 1);
        let o_phase = o_v2.atan2(o_v1);

        let g_idx = point_to_idx[&gen];
        let g_v1 = eigensystem.eigenvector_component(g_idx, fiedler_start);
        let g_v2 = eigensystem.eigenvector_component(g_idx, fiedler_start + 1);
        let g_phase = g_v2.atan2(g_v1);
        let phase_step = g_phase - o_phase;

        eprintln!(
            "  phase step: {:.8}, expected: {:.8}",
            phase_step,
            2.0 * std::f64::consts::PI / order as f64
        );

        // Test on first 500 keypairs
        let sample = 500u64.min(order - 1);
        let mut correct = 0u64;
        let mut correct_mod = 0u64;

        for k in 1..=sample {
            let public = scalar_mul(k, gen, a, p);
            let idx = point_to_idx[&public];

            let v1 = eigensystem.eigenvector_component(idx, fiedler_start);
            let v2 = eigensystem.eigenvector_component(idx, fiedler_start + 1);
            let phase = v2.atan2(v1);
            let delta =
                (phase - o_phase + 10.0 * std::f64::consts::PI) % (2.0 * std::f64::consts::PI);
            let pos = delta / phase_step.abs();
            let recovered = pos.round() as u64 % order;

            if recovered == k {
                correct += 1;
            }
            if recovered == k || recovered == order - k {
                correct_mod += 1;
            }
        }

        eprintln!(
            "  12-bit DFT: {}/{} exact ({:.1}%)",
            correct,
            sample,
            correct as f64 / sample as f64 * 100.0
        );
        eprintln!(
            "  12-bit DFT: {}/{} mod-symmetric ({:.1}%)",
            correct_mod,
            sample,
            correct_mod as f64 / sample as f64 * 100.0
        );

        // Compare theoretical eigenvalues
        let n_ring = order as usize; // the subgroup ring size
        let theoretical_fiedler = 2.0 - 2.0 * (2.0 * std::f64::consts::PI / n_ring as f64).cos();
        eprintln!(
            "  theoretical fiedler: {:.8}, actual: {:.8}, diff: {:.2e}",
            theoretical_fiedler,
            eigenvalues[fiedler_start],
            (theoretical_fiedler - eigenvalues[fiedler_start]).abs()
        );

        assert!(
            correct_mod as f64 / sample as f64 > 0.90,
            "12-bit DFT should recover >90% (got {:.1}%)",
            correct_mod as f64 / sample as f64 * 100.0
        );
    }

    #[test]
    fn sparse_lanczos_12bit_dft_recovery() {
        // The SPARSE test: same as full_laplacian_12bit but using
        // SparseLaplacian + Lanczos. O(n) memory instead of O(n²).
        let a = 1u64;
        let b = 1u64;
        let p = 4093u64;

        let points = enumerate_curve(a, b, p);
        let n = points.len();
        let gen = points[1];

        let mut pt = gen;
        let mut order = 1u64;
        loop {
            pt = point_add(pt, gen, a, p);
            order += 1;
            if pt == Point::Infinity || order > n as u64 + 1 {
                break;
            }
        }
        eprintln!("  12-bit sparse: {} points, order {}", n, order);

        let point_to_idx: std::collections::HashMap<Point, usize> =
            points.iter().enumerate().map(|(i, &p)| (p, i)).collect();

        // Build edges
        let vertices: Vec<String> = (0..n)
            .map(|i| match points[i] {
                Point::Infinity => "O".to_string(),
                Point::Affine { x, y } => format!("({},{})", x, y),
            })
            .collect();

        let mut edge_set = std::collections::HashSet::new();
        for (i, &pt) in points.iter().enumerate() {
            let sum = point_add(pt, gen, a, p);
            if let Some(&j) = point_to_idx.get(&sum) {
                let edge = if i < j { (i, j) } else { (j, i) };
                edge_set.insert(edge);
            }
        }
        let edges: Vec<(usize, usize)> = edge_set.into_iter().collect();

        // Sparse Lanczos — O(n) memory
        eprintln!("  building SparseLaplacian...");
        let sparse = coincidence::spectral::SparseLaplacian::from_edges(&vertices, &edges);

        let num_comp = sparse
            .components()
            .iter()
            .copied()
            .max()
            .map_or(0, |m| m + 1);
        eprintln!("  components: {}", num_comp);

        // Use component-aware Fiedler pair: decompose within the orbit of vertex 0
        eprintln!("  computing component Fiedler pair via Lanczos...");
        let (fiedler_val, v1, v2) = sparse
            .component_fiedler_pair(0)
            .expect("should find Fiedler pair in component of vertex 0");

        let expected = 2.0 - 2.0 * (2.0 * std::f64::consts::PI / order as f64).cos();
        eprintln!(
            "  fiedler: {:.8} (expected: {:.8}, diff: {:.2e})",
            fiedler_val,
            expected,
            (fiedler_val - expected).abs()
        );

        // Phase recovery using Lanczos Fiedler pair
        let o_phase = v2[0].atan2(v1[0]); // vertex 0 = Infinity
        let g_idx = point_to_idx[&gen];
        let g_phase = v2[g_idx].atan2(v1[g_idx]);
        let phase_step = g_phase - o_phase;

        eprintln!("  phase step: {:.8}", phase_step);

        let sample = 500u64.min(order - 1);
        let mut correct_mod = 0u64;

        for k in 1..=sample {
            let public = scalar_mul(k, gen, a, p);
            let idx = point_to_idx[&public];
            let phase = v2[idx].atan2(v1[idx]);
            let delta =
                (phase - o_phase + 10.0 * std::f64::consts::PI) % (2.0 * std::f64::consts::PI);
            let pos = delta / phase_step.abs();
            let recovered = pos.round() as u64 % order;
            if recovered == k || recovered == order - k {
                correct_mod += 1;
            }
        }

        eprintln!(
            "  sparse 12-bit DFT: {}/{} mod-symmetric ({:.1}%)",
            correct_mod,
            sample,
            correct_mod as f64 / sample as f64 * 100.0
        );

        // Memory comparison
        let dense_bytes = n * n * 8;
        let sparse_bytes = n * 2 * 16 + n * 2 * 8; // adj list + fiedler pair
        eprintln!(
            "  memory: dense={}MB, sparse={}KB, ratio={:.0}x",
            dense_bytes / 1_000_000,
            sparse_bytes / 1000,
            dense_bytes as f64 / sparse_bytes as f64
        );
    }
}
