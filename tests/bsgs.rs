//! Baby-step Giant-step — the answer all twelve experiments converged on.
//!
//! The optics vocabulary, the spectral analysis, the Abyss loop, the Shannon
//! recovery, the Hodge paper, the Chebyshev convergence — all roads lead to √n.
//!
//! This test proves it works and counts the cost.

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
            Point::Affine { x, y } => Point::Affine {
                x,
                y: if y == 0 { 0 } else { p - y },
            },
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

/// Baby-step Giant-step: find d such that Q = d·G, given group order n.
///
/// Returns (d, group_operations) — the answer and the cost.
fn bsgs(q: curve::Point, g: curve::Point, n: u64, curve_a: u64, field_p: u64) -> (u64, usize) {
    use std::collections::HashMap;

    let m = (n as f64).sqrt().ceil() as u64;
    let mut ops = 0usize;

    // Baby steps: table[j·G] = j for j = 0..m-1
    let mut table: HashMap<curve::Point, u64> = HashMap::new();
    let mut baby = curve::Point::Infinity;
    for j in 0..m {
        table.insert(baby, j);
        baby = curve::point_add(baby, g, curve_a, field_p);
        ops += 1;
    }

    // Giant step factor: -m·G
    let mg = curve::scalar_mul(m, g, curve_a, field_p);
    ops += (64 - m.leading_zeros()) as usize; // scalar_mul cost
    let neg_mg = curve::point_neg(mg, field_p);

    // Giant steps: check Q - i·m·G against table
    let mut gamma = q;
    for i in 0..m {
        if let Some(&j) = table.get(&gamma) {
            let d = (i * m + j) % n;
            return (d, ops);
        }
        gamma = curve::point_add(gamma, neg_mg, curve_a, field_p);
        ops += 1;
    }

    (0, ops) // should not reach here if n is correct
}

#[cfg(test)]
mod tests {
    use super::curve::*;
    use super::*;

    const CURVE_A: u64 = 1;
    const CURVE_B: u64 = 1;
    const FIELD_P: u64 = 251;

    fn find_generator(points: &[Point]) -> (Point, u64) {
        for &pt in points.iter().skip(1) {
            let mut order = 1u64;
            let mut current = pt;
            while current != Point::Infinity {
                current = point_add(current, pt, CURVE_A, FIELD_P);
                order += 1;
                if order > points.len() as u64 {
                    break;
                }
            }
            if order == points.len() as u64 {
                return (pt, order);
            }
        }
        panic!("no generator found");
    }

    #[test]
    fn bsgs_recovers_private_key() {
        let points = enumerate_curve(CURVE_A, CURVE_B, FIELD_P);
        let (gen, order) = find_generator(&points);

        for &d in &[1u64, 2, 7, 42, 100, 200, 281] {
            let q = scalar_mul(d, gen, CURVE_A, FIELD_P);
            let (recovered, _ops) = bsgs(q, gen, order, CURVE_A, FIELD_P);
            assert_eq!(recovered, d, "BSGS should recover d={} from Q={}·G", d, d);
        }
    }

    #[test]
    fn bsgs_cost_is_sqrt_n() {
        let points = enumerate_curve(CURVE_A, CURVE_B, FIELD_P);
        let (gen, order) = find_generator(&points);
        let sqrt_n = (order as f64).sqrt().ceil() as usize;

        for &d in &[42u64, 137, 200] {
            let q = scalar_mul(d, gen, CURVE_A, FIELD_P);
            let (_recovered, ops) = bsgs(q, gen, order, CURVE_A, FIELD_P);
            assert!(
                ops <= 3 * sqrt_n,
                "BSGS should use O(√n) ops: used {} but √n={} (3√n={})",
                ops,
                sqrt_n,
                3 * sqrt_n
            );
        }
    }

    #[test]
    fn bsgs_beats_brute_force() {
        let points = enumerate_curve(CURVE_A, CURVE_B, FIELD_P);
        let (gen, order) = find_generator(&points);

        let d = 200u64;
        let q = scalar_mul(d, gen, CURVE_A, FIELD_P);
        let (_recovered, bsgs_ops) = bsgs(q, gen, order, CURVE_A, FIELD_P);

        // Brute force would need ~d operations on average
        assert!(
            bsgs_ops < order as usize / 2,
            "BSGS ({} ops) should beat brute force average ({} ops)",
            bsgs_ops,
            order / 2
        );
    }
}
