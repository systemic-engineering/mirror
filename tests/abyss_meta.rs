//! ABYSS META — Spectral meta-graph navigation for elliptic curve DLP.
//!
//! Nine experiments have returned negative results. Each negative loses different
//! information about the private key d. This experiment asks: if we build a
//! meta-graph where nodes are the mathematical views and edges are spectral
//! distances between them, does navigating by minimum-meaning-loss (the Abyss
//! principle) through a circular path have a fixed point that encodes d?
//!
//! Five views:
//!   1. Fold  — DFT row for Q's vertex (eigendecomposition fingerprint)
//!   2. Lens  — power spectrum of x-projection f(k) = x(kG)
//!   3. Traverse — coordinate-order difference signal
//!   4. Iso   — windowed cross-correlation of Q-shifted window with f
//!   5. Hodge — Catalan/Hodge structural constants
//!
//! The circular navigation starts from a view, finds the nearest neighbor,
//! interferes (element-wise product), normalizes, repeats. If d is a fixed
//! point of this iteration, it will emerge as the argmax of the converged state.

use std::f64::consts::PI;

// -- curve arithmetic (copied from butterfly.rs) --------------------------------

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

// -- combinatorics (f64 to avoid overflow for large n) ---------------------------

fn binomial_f64(n: u64, k: u64) -> f64 {
    if k > n {
        return 0.0;
    }
    if k == 0 || k == n {
        return 1.0;
    }
    let k = k.min(n - k);
    let mut result = 1.0f64;
    for i in 0..k {
        result *= (n - i) as f64 / (i + 1) as f64;
    }
    result
}

fn catalan_f64(m: u64) -> f64 {
    binomial_f64(2 * m, m) / (m + 1) as f64
}

// -- DFT helpers -----------------------------------------------------------------

fn dft_full(signal: &[f64], n: usize) -> Vec<(f64, f64)> {
    let mut spectrum = Vec::with_capacity(n);
    for k in 0..n {
        let mut re = 0.0f64;
        let mut im = 0.0f64;
        for j in 0..n {
            let angle = -2.0 * PI * (j as f64) * (k as f64) / (n as f64);
            re += signal[j] * angle.cos();
            im += signal[j] * angle.sin();
        }
        spectrum.push((re, im));
    }
    spectrum
}

fn l2_norm(v: &[f64]) -> f64 {
    v.iter().map(|x| x * x).sum::<f64>().sqrt()
}

/// Spectral distance: sort both vectors, compute normalized L2 distance.
/// Vectors are padded/truncated to the same length first.
fn spectral_distance(a: &[f64], b: &[f64]) -> f64 {
    let len = a.len().max(b.len());
    let mut sa = vec![0.0f64; len];
    let mut sb = vec![0.0f64; len];
    for (i, &v) in a.iter().enumerate() {
        sa[i] = v;
    }
    for (i, &v) in b.iter().enumerate() {
        sb[i] = v;
    }
    sa.sort_by(|x, y| x.partial_cmp(y).unwrap());
    sb.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let diff_norm: f64 = sa
        .iter()
        .zip(sb.iter())
        .map(|(x, y)| (x - y) * (x - y))
        .sum::<f64>()
        .sqrt();
    let denom = l2_norm(&sa).max(l2_norm(&sb)).max(1e-12);
    diff_norm / denom
}

fn normalize(v: &mut [f64]) {
    let norm = l2_norm(v);
    if norm > 1e-15 {
        for x in v.iter_mut() {
            *x /= norm;
        }
    }
}

fn argmax(v: &[f64]) -> usize {
    v.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(0)
}

// -- Chebyshev polynomials -------------------------------------------------------

/// T_k(x) via recurrence: T_0=1, T_1=x, T_{k+1} = 2x·T_k - T_{k-1}
fn chebyshev(k: usize, x: f64) -> f64 {
    if k == 0 {
        return 1.0;
    }
    if k == 1 {
        return x;
    }
    let mut t_prev = 1.0f64;
    let mut t_curr = x;
    for _ in 2..=k {
        let t_next = 2.0 * x * t_curr - t_prev;
        t_prev = t_curr;
        t_curr = t_next;
    }
    t_curr
}

// =================================================================================

#[test]
fn abyss_meta_navigation() {
    eprintln!("\n=== ABYSS META: Spectral meta-graph navigation ===\n");

    // -------------------------------------------------------------------------
    // Step 1: Curve setup (8-bit)
    // -------------------------------------------------------------------------
    let field_p = 251u64;
    let curve_a = 1u64;
    let curve_b = 1u64;
    let points = enumerate_curve(curve_a, curve_b, field_p);
    eprintln!(
        "  Curve: y^2 = x^3 + {}x + {} (mod {})",
        curve_a, curve_b, field_p
    );
    eprintln!("  Points enumerated: {}", points.len());

    // Find a generator: try points[1], check it generates all affine points.
    let gen = points[1];
    let mut order = 1u64;
    let mut pt = gen;
    while pt != Point::Infinity {
        pt = point_add(pt, gen, curve_a, field_p);
        order += 1;
        if order > points.len() as u64 + 1 {
            break;
        }
    }
    // If points[1] doesn't generate the full group, search for a generator
    let (gen, n) = if order == points.len() as u64 {
        (gen, order as usize)
    } else {
        // Try each point
        let mut found = (Point::Infinity, 0usize);
        for &candidate in &points[1..] {
            let mut ord = 1u64;
            let mut pt = candidate;
            while pt != Point::Infinity {
                pt = point_add(pt, candidate, curve_a, field_p);
                ord += 1;
                if ord > points.len() as u64 + 1 {
                    break;
                }
            }
            if ord == points.len() as u64 {
                found = (candidate, ord as usize);
                break;
            }
        }
        found
    };
    eprintln!("  Generator G = {:?}", gen);
    eprintln!("  Group order n = {}", n);
    assert!(n > 0, "Failed to find generator");

    // Build ring ordering: ring[k] = kG
    let mut ring: Vec<Point> = Vec::with_capacity(n);
    let mut pt = Point::Infinity;
    for _ in 0..n {
        ring.push(pt);
        pt = point_add(pt, gen, curve_a, field_p);
    }

    // Point-to-index map (vertex ordering in the enumeration)
    let point_to_idx: std::collections::HashMap<Point, usize> =
        points.iter().enumerate().map(|(i, &p)| (p, i)).collect();

    // Target
    let d: u64 = 42;
    let q = scalar_mul(d, gen, curve_a, field_p);
    eprintln!("  Target d = {}, Q = dG = {:?}", d, q);
    let q_vertex_idx = point_to_idx[&q];
    eprintln!("  Q vertex index in enumeration = {}", q_vertex_idx);

    // x-projection signal: f(k) = x(kG) for k = 0..n-1
    // For k=0 (Infinity), use 0.
    let f_signal: Vec<f64> = ring
        .iter()
        .map(|p| match p {
            Point::Infinity => 0.0,
            Point::Affine { x, .. } => *x as f64,
        })
        .collect();

    // -------------------------------------------------------------------------
    // Step 2: Build the Five Views
    // -------------------------------------------------------------------------
    eprintln!("\n--- Building Five Views ---\n");

    // View 1: Fold (DFT row for Q's vertex)
    // Entry k = cos(2*pi*k*vertex_idx/n)
    let view_fold: Vec<f64> = (0..n)
        .map(|k| (2.0 * PI * (k as f64) * (q_vertex_idx as f64) / (n as f64)).cos())
        .collect();
    eprintln!(
        "  View 1 (Fold): DFT row for vertex {}, len={}, norm={:.4}",
        q_vertex_idx,
        view_fold.len(),
        l2_norm(&view_fold)
    );

    // View 2: Lens (power spectrum of x-projection)
    let spectrum = dft_full(&f_signal, n);
    let view_lens: Vec<f64> = spectrum.iter().map(|(re, im)| re * re + im * im).collect();
    eprintln!(
        "  View 2 (Lens): power spectrum of f(k)=x(kG), len={}, norm={:.4}",
        view_lens.len(),
        l2_norm(&view_lens)
    );

    // View 3: Traverse (coordinate-order difference signal)
    // Sort affine points by x-coordinate, compute group difference x-coords.
    let mut affine_sorted: Vec<(u64, u64)> = points
        .iter()
        .filter_map(|p| match p {
            Point::Affine { x, y } => Some((*x, *y)),
            _ => None,
        })
        .collect();
    affine_sorted.sort_by_key(|&(x, _)| x);
    let view_traverse: Vec<f64> = affine_sorted
        .windows(2)
        .map(|w| {
            let p1 = Point::Affine {
                x: w[0].0,
                y: w[0].1,
            };
            // Negate p2 then add to p1 to get difference
            let p2_neg = Point::Affine {
                x: w[1].0,
                y: (field_p - w[1].1) % field_p,
            };
            let diff = point_add(p1, p2_neg, curve_a, field_p);
            match diff {
                Point::Infinity => 0.0,
                Point::Affine { x, .. } => x as f64,
            }
        })
        .collect();
    eprintln!(
        "  View 3 (Traverse): coordinate-order diffs, len={}, norm={:.4}",
        view_traverse.len(),
        l2_norm(&view_traverse)
    );

    // View 4: Iso (windowed cross-correlation)
    let m = n / 4; // window size
    let window: Vec<f64> = (0..m)
        .map(|j| {
            let pt = scalar_mul((d + j as u64) % n as u64, gen, curve_a, field_p);
            match pt {
                Point::Infinity => 0.0,
                Point::Affine { x, .. } => x as f64,
            }
        })
        .collect();
    // Cross-correlation: C(tau) = sum_j w(j) * f((j + tau) mod n)
    let view_iso: Vec<f64> = (0..n)
        .map(|tau| {
            let mut c = 0.0f64;
            for j in 0..m {
                c += window[j] * f_signal[(j + tau) % n];
            }
            c
        })
        .collect();
    let iso_argmax = argmax(&view_iso);
    eprintln!(
        "  View 4 (Iso): cross-correlation, len={}, norm={:.4}, argmax={}",
        view_iso.len(),
        l2_norm(&view_iso),
        iso_argmax
    );

    // View 5: Hodge (Catalan structure)
    // dim Hdg^p(E^g) = sum_{a=0}^{p} C(g,a) * C(g-a, 2(p-a)) * Cat(p-a)
    // Use g = n, p = 1..min(10, g-1)
    let g_val = n as u64;
    let max_p = 10u64.min(g_val - 1);
    let view_hodge: Vec<f64> = (1..=max_p)
        .map(|pp| {
            let mut dim = 0.0f64;
            for a in 0..=pp {
                let pa = pp - a;
                let c1 = binomial_f64(g_val, a);
                let arg2 = if g_val >= a { g_val - a } else { 0 };
                let c2 = binomial_f64(arg2, 2 * pa);
                let cat = catalan_f64(pa);
                dim += c1 * c2 * cat;
            }
            dim
        })
        .collect();
    eprintln!(
        "  View 5 (Hodge): Catalan structure, len={}, values={:?}",
        view_hodge.len(),
        &view_hodge[..view_hodge.len().min(5)]
    );

    // Collect views
    let view_names = ["Fold", "Lens", "Traverse", "Iso", "Hodge"];
    let views: Vec<&[f64]> = vec![
        &view_fold,
        &view_lens,
        &view_traverse,
        &view_iso,
        &view_hodge,
    ];

    // -------------------------------------------------------------------------
    // Step 3: Spectral Distance Matrix
    // -------------------------------------------------------------------------
    eprintln!("\n--- Spectral Distance Matrix (5x5) ---\n");

    let mut dist_matrix = vec![vec![0.0f64; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            dist_matrix[i][j] = spectral_distance(views[i], views[j]);
        }
    }

    eprint!("          ");
    for name in &view_names {
        eprint!("{:>10}", name);
    }
    eprintln!();
    for i in 0..5 {
        eprint!("  {:>8}", view_names[i]);
        for j in 0..5 {
            eprint!("    {:.4}", dist_matrix[i][j]);
        }
        eprintln!();
    }

    // -------------------------------------------------------------------------
    // Step 4: Circular Navigation
    // -------------------------------------------------------------------------
    eprintln!("\n--- Circular Navigation (element-wise product) ---\n");

    let unified_len = n; // pad/truncate all views to length n

    // Prepare padded views
    let padded_views: Vec<Vec<f64>> = views
        .iter()
        .map(|v| {
            let mut padded = vec![0.0f64; unified_len];
            for (i, &val) in v.iter().enumerate() {
                if i < unified_len {
                    padded[i] = val;
                }
            }
            padded
        })
        .collect();

    // Navigation starting from Iso (view 3, index 3)
    let start_view = 3; // Iso
    let mut state = padded_views[start_view].clone();
    normalize(&mut state);

    eprintln!(
        "  Starting from: {} (view {})",
        view_names[start_view], start_view
    );
    eprintln!("  Initial argmax: {} (true d={})", argmax(&state), d);
    eprintln!();

    let mut any_hit = false;
    for iter in 0..20 {
        // Find closest view (excluding self-distance = 0 check via actual distance)
        let mut best_view = 0;
        let mut best_dist = f64::MAX;
        for v in 0..5 {
            let dist = spectral_distance(&state, &padded_views[v]);
            if dist < best_dist {
                best_dist = dist;
                best_view = v;
            }
        }

        // Interfere: element-wise product
        let mut next_state = vec![0.0f64; unified_len];
        for i in 0..unified_len {
            next_state[i] = state[i] * padded_views[best_view][i];
        }
        normalize(&mut next_state);

        let am = argmax(&next_state);
        let hit = am == d as usize;
        if hit {
            any_hit = true;
        }

        eprintln!(
            "  iter {:2}: closest={:>8} (dist={:.4}), argmax={:3} {}",
            iter,
            view_names[best_view],
            best_dist,
            am,
            if hit { "<-- HIT" } else { "" }
        );

        state = next_state;
    }

    eprintln!(
        "\n  Product navigation result: {}",
        if any_hit {
            "SIGNAL — d was argmax at some iteration"
        } else {
            "negative — d never appeared as argmax"
        }
    );

    // -------------------------------------------------------------------------
    // Step 5: Chebyshev Bridge
    // -------------------------------------------------------------------------
    eprintln!("\n--- Chebyshev Bridge ---\n");

    let max_cheb_order = 20;

    for (vi, view) in padded_views.iter().enumerate() {
        let mut best_k = 0;
        let mut best_corr = 0.0f64;
        let mut correlations = Vec::new();

        for k in 0..=max_cheb_order {
            // Compute Chebyshev T_k(cos(2*pi*j/n)) for j=0..n-1
            let cheb_signal: Vec<f64> = (0..unified_len)
                .map(|j| {
                    let x = (2.0 * PI * (j as f64) / (unified_len as f64)).cos();
                    chebyshev(k, x)
                })
                .collect();

            // Correlation: dot product (both are roughly unit-normalized or we normalize)
            let mut cheb_norm = cheb_signal.clone();
            normalize(&mut cheb_norm);
            let mut view_norm = view.clone();
            normalize(&mut view_norm);
            let corr: f64 = cheb_norm
                .iter()
                .zip(view_norm.iter())
                .map(|(a, b)| a * b)
                .sum();
            let corr_abs = corr.abs();
            correlations.push((k, corr));

            if corr_abs > best_corr {
                best_corr = corr_abs;
                best_k = k;
            }
        }

        eprintln!(
            "  {:>8}: dominant Chebyshev order T_{} (|corr|={:.4})",
            view_names[vi], best_k, best_corr
        );
        // Print top 3
        let mut sorted_corrs: Vec<(usize, f64)> =
            correlations.iter().map(|&(k, c)| (k, c.abs())).collect();
        sorted_corrs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        for &(k, c) in sorted_corrs.iter().take(3) {
            eprintln!("             T_{:2}: |corr|={:.4}", k, c);
        }
    }

    // Check for shared dominant Chebyshev orders
    eprintln!("\n  Checking for spectral coincidences (shared dominant Chebyshev orders)...");
    let dominant_orders: Vec<usize> = padded_views
        .iter()
        .map(|view| {
            let mut best_k = 0;
            let mut best_corr = 0.0f64;
            for k in 0..=max_cheb_order {
                let cheb_signal: Vec<f64> = (0..unified_len)
                    .map(|j| {
                        let x = (2.0 * PI * (j as f64) / (unified_len as f64)).cos();
                        chebyshev(k, x)
                    })
                    .collect();
                let mut cn = cheb_signal;
                normalize(&mut cn);
                let mut vn = view.clone();
                normalize(&mut vn);
                let corr: f64 = cn.iter().zip(vn.iter()).map(|(a, b)| a * b).sum();
                if corr.abs() > best_corr {
                    best_corr = corr.abs();
                    best_k = k;
                }
            }
            best_k
        })
        .collect();

    for i in 0..5 {
        for j in (i + 1)..5 {
            if dominant_orders[i] == dominant_orders[j] {
                eprintln!(
                    "  COINCIDENCE: {} and {} share dominant T_{}",
                    view_names[i], view_names[j], dominant_orders[i]
                );
            }
        }
    }

    // Catalan ratio check
    eprintln!("\n  Catalan ratio check (Cat(m+1)/Cat(m) = (4m+2)/(m+2)):");
    for (vi, view) in padded_views.iter().enumerate() {
        let mut sorted_view: Vec<f64> = view.iter().copied().filter(|x| x.abs() > 1e-12).collect();
        sorted_view.sort_by(|a, b| b.abs().partial_cmp(&a.abs()).unwrap());
        if sorted_view.len() < 4 {
            continue;
        }
        let mut catalan_score = 0.0f64;
        let mut count = 0;
        for m in 0..sorted_view.len().min(15).saturating_sub(1) {
            if sorted_view[m + 1].abs() < 1e-12 {
                break;
            }
            let observed_ratio = sorted_view[m].abs() / sorted_view[m + 1].abs();
            let expected_ratio = (4 * m + 2) as f64 / (m + 2) as f64;
            let err = (observed_ratio - expected_ratio).abs() / expected_ratio.max(1e-12);
            catalan_score += err;
            count += 1;
        }
        if count > 0 {
            eprintln!(
                "  {:>8}: avg Catalan ratio error = {:.4} (over {} consecutive pairs)",
                view_names[vi],
                catalan_score / count as f64,
                count
            );
        }
    }

    // -------------------------------------------------------------------------
    // Step 6: Fixed Point Detection — all starting views
    // -------------------------------------------------------------------------
    eprintln!("\n--- Fixed Point Detection (all starting views) ---\n");

    // Method A: Element-wise product
    eprintln!("  Method A: Element-wise product interference\n");
    let mut method_a_hits = vec![false; 5];

    for start in 0..5 {
        let mut state = padded_views[start].clone();
        normalize(&mut state);
        let mut converged_to = argmax(&state);
        let mut hit = false;

        for _iter in 0..20 {
            let mut best_view = 0;
            let mut best_dist = f64::MAX;
            for v in 0..5 {
                let dist = spectral_distance(&state, &padded_views[v]);
                if dist < best_dist {
                    best_dist = dist;
                    best_view = v;
                }
            }

            let mut next_state = vec![0.0f64; unified_len];
            for i in 0..unified_len {
                next_state[i] = state[i] * padded_views[best_view][i];
            }
            normalize(&mut next_state);

            let am = argmax(&next_state);
            if am == d as usize {
                hit = true;
            }
            converged_to = am;
            state = next_state;
        }

        method_a_hits[start] = hit;
        let relation = if converged_to == d as usize {
            "= d".to_string()
        } else if converged_to == (n - d as usize) % n {
            "= n-d".to_string()
        } else if d as usize > 0 && converged_to % (d as usize) == 0 {
            format!("= {}*d", converged_to / d as usize)
        } else {
            "unrelated".to_string()
        };
        eprintln!(
            "    start={:>8}: converged argmax={:3} ({}) {}",
            view_names[start],
            converged_to,
            relation,
            if hit { "<-- HIT" } else { "" }
        );
    }

    // Method B: Circular convolution
    eprintln!("\n  Method B: Circular convolution interference\n");
    let mut method_b_hits = vec![false; 5];

    for start in 0..5 {
        let mut state = padded_views[start].clone();
        normalize(&mut state);
        let mut converged_to = argmax(&state);
        let mut hit = false;

        for _ in 0..20 {
            let mut best_view = 0;
            let mut best_dist = f64::MAX;
            for v in 0..5 {
                let dist = spectral_distance(&state, &padded_views[v]);
                if dist < best_dist {
                    best_dist = dist;
                    best_view = v;
                }
            }

            // Circular convolution: (state * view)(tau) = sum_j state(j) * view((tau-j) mod n)
            let view_ref = &padded_views[best_view];
            let mut next_state = vec![0.0f64; unified_len];
            for tau in 0..unified_len {
                let mut val = 0.0f64;
                for j in 0..unified_len {
                    val += state[j] * view_ref[(tau + unified_len - j) % unified_len];
                }
                next_state[tau] = val;
            }
            normalize(&mut next_state);

            let am = argmax(&next_state);
            if am == d as usize {
                hit = true;
            }
            converged_to = am;
            state = next_state;
        }

        method_b_hits[start] = hit;
        let relation = if converged_to == d as usize {
            "= d".to_string()
        } else if converged_to == (n - d as usize) % n {
            "= n-d".to_string()
        } else if d as usize > 0 && converged_to % (d as usize) == 0 {
            format!("= {}*d", converged_to / d as usize)
        } else {
            "unrelated".to_string()
        };
        eprintln!(
            "    start={:>8}: converged argmax={:3} ({}) {}",
            view_names[start],
            converged_to,
            relation,
            if hit { "<-- HIT" } else { "" }
        );
    }

    // Method C: Addition + peak sharpening (raise to power, renormalize)
    eprintln!("\n  Method C: Addition + peak sharpening (power=4)\n");
    let mut method_c_hits = vec![false; 5];
    let sharpen_power = 4.0f64;

    for start in 0..5 {
        let mut state = padded_views[start].clone();
        normalize(&mut state);
        let mut converged_to = argmax(&state);
        let mut hit = false;

        for _ in 0..20 {
            let mut best_view = 0;
            let mut best_dist = f64::MAX;
            for v in 0..5 {
                let dist = spectral_distance(&state, &padded_views[v]);
                if dist < best_dist {
                    best_dist = dist;
                    best_view = v;
                }
            }

            // Add, then sharpen: raise absolute values to a power, preserve sign
            let view_ref = &padded_views[best_view];
            let mut vn = view_ref.to_vec();
            normalize(&mut vn);
            let mut next_state = vec![0.0f64; unified_len];
            for i in 0..unified_len {
                next_state[i] = state[i] + vn[i];
            }
            // Peak sharpening
            for i in 0..unified_len {
                let sign = if next_state[i] >= 0.0 { 1.0 } else { -1.0 };
                next_state[i] = sign * next_state[i].abs().powf(sharpen_power);
            }
            normalize(&mut next_state);

            let am = argmax(&next_state);
            if am == d as usize {
                hit = true;
            }
            converged_to = am;
            state = next_state;
        }

        method_c_hits[start] = hit;
        let relation = if converged_to == d as usize {
            "= d".to_string()
        } else if converged_to == (n - d as usize) % n {
            "= n-d".to_string()
        } else if d as usize > 0 && converged_to % (d as usize) == 0 {
            format!("= {}*d", converged_to / d as usize)
        } else {
            "unrelated".to_string()
        };
        eprintln!(
            "    start={:>8}: converged argmax={:3} ({}) {}",
            view_names[start],
            converged_to,
            relation,
            if hit { "<-- HIT" } else { "" }
        );
    }

    // -------------------------------------------------------------------------
    // Summary
    // -------------------------------------------------------------------------
    eprintln!("\n=== SUMMARY ===\n");

    let any_a = method_a_hits.iter().any(|&h| h);
    let any_b = method_b_hits.iter().any(|&h| h);
    let any_c = method_c_hits.iter().any(|&h| h);

    eprintln!(
        "  Method A (product):     {}",
        if any_a { "SIGNAL" } else { "negative" }
    );
    eprintln!(
        "  Method B (convolution): {}",
        if any_b { "SIGNAL" } else { "negative" }
    );
    eprintln!(
        "  Method C (sharpen):     {}",
        if any_c { "SIGNAL" } else { "negative" }
    );

    let any_signal = any_a || any_b || any_c;
    eprintln!(
        "\n  Overall: {}",
        if any_signal {
            "SIGNAL DETECTED — circular navigation recovered d for at least one configuration"
        } else {
            "NEGATIVE — no configuration recovered d. The five views do not form a \
             navigable meta-graph whose fixed point encodes the private key."
        }
    );

    // Report what we learned regardless
    eprintln!("\n  Meta-structure observations:");
    eprintln!("  - Spectral distance matrix reveals which views are 'close' in eigenvalue space");
    eprintln!(
        "  - Dominant Chebyshev orders: {:?}",
        view_names
            .iter()
            .zip(dominant_orders.iter())
            .map(|(n, &o)| format!("{}=T_{}", n, o))
            .collect::<Vec<_>>()
    );

    // Don't assert — this is exploratory. Report the result.
    eprintln!("\n=== END ABYSS META ===\n");
}
