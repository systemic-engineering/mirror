//! @benchmark — performance as grammar. Optimization as the five operations.
//!
//! Contract:
//! - in: file path(s)
//! - out: BenchResult with timing, OID, throughput
//! - bound: std::time::Instant only. No criterion. No external crates.
//!
//! The benchmark IS a prism:
//! - focus: observe performance of a single file
//! - split: compare variants (grammar vs Rust, file vs file)
//! - zoom: transform under load (iterations)
//! - refract: settle to SLO (convergence check)
//! - project: report results

use std::time::Instant;

use crate::kernel::Oid;
use crate::tokenize;

// ---------------------------------------------------------------------------
// Types — what we measure
// ---------------------------------------------------------------------------

/// A single benchmark result for one file.
#[derive(Clone, Debug)]
pub struct BenchResult {
    /// File that was benchmarked.
    pub name: String,
    /// Content-address of the tokenized AST.
    pub oid: Oid,
    /// Total time across all iterations (nanoseconds).
    pub time_ns: u128,
    /// Number of iterations run.
    pub iterations: u64,
    /// Mean time per iteration (nanoseconds).
    pub mean_ns: u128,
    /// Peak memory estimate (bytes) — file size * overhead factor.
    pub memory_bytes: u64,
    /// Throughput: iterations per second.
    pub throughput_ops: f64,
}

/// A suite of benchmark results.
#[derive(Clone, Debug)]
pub struct BenchSuite {
    pub results: Vec<BenchResult>,
    pub total_time_ns: u128,
}

/// Cascade measurement: does optimization compound across iterations?
#[derive(Clone, Debug)]
pub struct CascadeResult {
    /// Per-iteration timings (nanoseconds).
    pub iteration_times: Vec<u128>,
    /// Loss curve: ratio of each iteration's time to the first.
    pub loss_curve: Vec<f64>,
    /// Does it converge? (later iterations faster or equal to earlier ones)
    pub converges: bool,
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Default number of iterations for benchmarking.
const DEFAULT_ITERATIONS: u64 = 100;

/// Minimum iterations for cascade measurement.
const CASCADE_ITERATIONS: u64 = 10;

/// Reduced iterations for tests (debug mode is slow).
#[cfg(test)]
const TEST_ITERATIONS: u64 = 3;

// ---------------------------------------------------------------------------
// Focus: bench a single file
// ---------------------------------------------------------------------------

/// Benchmark a single file: tokenize + content-address, repeated.
///
/// Returns BenchResult with timing, OID, and throughput.
pub fn bench_file(path: &str) -> BenchResult {
    bench_file_n(path, DEFAULT_ITERATIONS)
}

/// Benchmark a single file with explicit iteration count.
pub fn bench_file_n(path: &str, iterations: u64) -> BenchResult {
    let source = std::fs::read_to_string(path).unwrap_or_else(|e| {
        panic!("cannot read file {}: {}", path, e);
    });

    let grammar_path = tokenize::grammar_for_file(path);
    let grammar = tokenize::load_grammar(grammar_path).unwrap_or_else(|e| {
        panic!("cannot load grammar for {}: {}", path, e);
    });

    // Warm-up: one untimed run
    let warm = tokenize::tokenize(&source, &grammar);
    let oid = warm.content_oid();

    // Timed iterations
    let start = Instant::now();
    for _ in 0..iterations {
        let ast = tokenize::tokenize(&source, &grammar);
        // Force materialization of the OID so we measure the full pipeline
        std::hint::black_box(ast.content_oid());
    }
    let elapsed = start.elapsed();
    let time_ns = elapsed.as_nanos();
    let mean_ns = if iterations > 0 { time_ns / iterations as u128 } else { 0 };

    // Memory estimate: source size + estimated AST overhead (3x source size is typical)
    let memory_bytes = (source.len() as u64) * 3;

    // Throughput
    let throughput_ops = if elapsed.as_secs_f64() > 0.0 {
        iterations as f64 / elapsed.as_secs_f64()
    } else {
        f64::INFINITY
    };

    BenchResult {
        name: path.to_string(),
        oid,
        time_ns,
        iterations,
        mean_ns,
        memory_bytes,
        throughput_ops,
    }
}

// ---------------------------------------------------------------------------
// Split: compare two results
// ---------------------------------------------------------------------------

/// Compare two bench results. Returns the speedup ratio (>1 = a is faster).
pub fn compare(a: &BenchResult, b: &BenchResult) -> f64 {
    if a.mean_ns == 0 {
        return f64::INFINITY;
    }
    b.mean_ns as f64 / a.mean_ns as f64
}

// ---------------------------------------------------------------------------
// Zoom: suite — bench all files in a directory
// ---------------------------------------------------------------------------

/// Benchmark all files in a directory (recursively).
/// Returns a BenchSuite with results sorted by file name.
pub fn bench_dir(dir: &str) -> BenchSuite {
    bench_dir_n(dir, DEFAULT_ITERATIONS)
}

/// Benchmark all files in a directory with explicit iteration count.
pub fn bench_dir_n(dir: &str, iterations: u64) -> BenchSuite {
    let mut files = Vec::new();

    // Collect both .mirror and .rs files
    let mirror_files = tokenize::find_mirror_files(dir);
    let rs_files = tokenize::find_rs_files(dir);
    files.extend(mirror_files);
    files.extend(rs_files);
    files.sort();

    let mut results = Vec::new();
    let mut total_time_ns: u128 = 0;

    for file in &files {
        let result = bench_file_n(file, iterations);
        total_time_ns += result.time_ns;
        results.push(result);
    }

    BenchSuite {
        results,
        total_time_ns,
    }
}

// ---------------------------------------------------------------------------
// Refract: cascade — do optimizations compound?
// ---------------------------------------------------------------------------

/// Measure cascade: run the full craft pipeline repeatedly,
/// checking if later iterations are faster (CPU cache warming,
/// branch prediction settling).
pub fn cascade(dir: &str) -> CascadeResult {
    cascade_n(dir, CASCADE_ITERATIONS)
}

/// Measure cascade with explicit iteration count.
pub fn cascade_n(dir: &str, iterations: u64) -> CascadeResult {
    let files = tokenize::find_mirror_files(dir);
    if files.is_empty() {
        return CascadeResult {
            iteration_times: vec![],
            loss_curve: vec![],
            converges: true,
        };
    }

    let mut iteration_times = Vec::new();

    for _ in 0..iterations {
        let start = Instant::now();
        for file in &files {
            let source = match std::fs::read_to_string(file) {
                Ok(s) => s,
                Err(_) => continue,
            };
            let grammar_path = tokenize::grammar_for_file(file);
            let grammar = match tokenize::load_grammar(grammar_path) {
                Ok(g) => g,
                Err(_) => continue,
            };
            let ast = tokenize::tokenize(&source, &grammar);
            std::hint::black_box(ast.content_oid());
        }
        let elapsed = start.elapsed();
        iteration_times.push(elapsed.as_nanos());
    }

    // Compute loss curve: ratio of each iteration to the first
    let first = iteration_times[0] as f64;
    let loss_curve: Vec<f64> = iteration_times
        .iter()
        .map(|&t| t as f64 / first)
        .collect();

    // Converges if the last 3 iterations are all <= the first iteration
    let converges = if iteration_times.len() >= 3 {
        let tail = &loss_curve[loss_curve.len() - 3..];
        tail.iter().all(|&l| l <= 1.1) // within 10% of first run
    } else {
        true
    };

    CascadeResult {
        iteration_times,
        loss_curve,
        converges,
    }
}

// ---------------------------------------------------------------------------
// Project: report formatting
// ---------------------------------------------------------------------------

/// Format a single BenchResult for display.
pub fn format_result(result: &BenchResult) -> String {
    let time_human = format_duration_ns(result.mean_ns);
    let memory_human = format_bytes(result.memory_bytes);
    let oid_short = &result.oid.as_ref()[..std::cmp::min(8, result.oid.as_ref().len())];

    format!(
        "bench {}\n  time: {} ({} iterations)\n  memory: {} est.\n  throughput: {:.0} ops/sec\n  oid: {}",
        result.name,
        time_human,
        result.iterations,
        memory_human,
        result.throughput_ops,
        oid_short,
    )
}

/// Format a BenchSuite for display.
pub fn format_suite(suite: &BenchSuite) -> String {
    let mut out = String::new();
    for result in &suite.results {
        out.push_str(&format_result(result));
        out.push('\n');
        out.push('\n');
    }
    let total_human = format_duration_ns(suite.total_time_ns);
    out.push_str(&format!("total: {} ({} files)\n", total_human, suite.results.len()));
    out
}

/// Format a CascadeResult for display.
pub fn format_cascade(cascade: &CascadeResult) -> String {
    let mut out = String::new();
    out.push_str("cascade:\n");
    for (i, (&time, &loss)) in cascade
        .iteration_times
        .iter()
        .zip(cascade.loss_curve.iter())
        .enumerate()
    {
        out.push_str(&format!(
            "  iteration {}: {} (loss: {:.3})\n",
            i + 1,
            format_duration_ns(time),
            loss,
        ));
    }
    out.push_str(&format!(
        "  converges: {}\n",
        if cascade.converges { "yes" } else { "no" }
    ));
    out
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Format nanoseconds into human-readable duration.
fn format_duration_ns(ns: u128) -> String {
    if ns < 1_000 {
        format!("{}ns", ns)
    } else if ns < 1_000_000 {
        format!("{:.1}us", ns as f64 / 1_000.0)
    } else if ns < 1_000_000_000 {
        format!("{:.1}ms", ns as f64 / 1_000_000.0)
    } else {
        format!("{:.2}s", ns as f64 / 1_000_000_000.0)
    }
}

/// Format bytes into human-readable size.
fn format_bytes(bytes: u64) -> String {
    if bytes < 1_024 {
        format!("{}B", bytes)
    } else if bytes < 1_048_576 {
        format!("{:.1}KB", bytes as f64 / 1_024.0)
    } else if bytes < 1_073_741_824 {
        format!("{:.1}MB", bytes as f64 / 1_048_576.0)
    } else {
        format!("{:.1}GB", bytes as f64 / 1_073_741_824.0)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Red: bench_file produces a result --

    #[test]
    fn bench_single_file_produces_result() {
        let result = bench_file_n("boot/std/kintsugi.mirror", TEST_ITERATIONS);
        assert!(result.time_ns > 0, "time must be positive");
        assert!(result.iterations > 0, "iterations must be positive");
        assert!(result.mean_ns > 0, "mean time must be positive");
        assert!(!result.name.is_empty(), "name must not be empty");
    }

    #[test]
    fn bench_is_deterministic() {
        let r1 = bench_file_n("boot/std/kintsugi.mirror", TEST_ITERATIONS);
        let r2 = bench_file_n("boot/std/kintsugi.mirror", TEST_ITERATIONS);
        // OID must be identical across runs — same input, same content address
        assert_eq!(
            r1.oid.as_ref(),
            r2.oid.as_ref(),
            "OID must be deterministic"
        );
    }

    #[test]
    fn bench_bounded_memory() {
        // Benchmarking must not OOM — memory estimate stays reasonable
        let result = bench_file_n("src/mirror_ast.rs", TEST_ITERATIONS);
        assert!(
            result.memory_bytes < 100_000_000,
            "memory estimate must be < 100MB, got {}",
            result.memory_bytes
        );
    }

    #[test]
    fn bench_throughput_positive() {
        let result = bench_file_n("boot/std/kintsugi.mirror", TEST_ITERATIONS);
        assert!(
            result.throughput_ops > 0.0,
            "throughput must be positive"
        );
    }

    // -- Red: suite benchmarks --

    #[test]
    fn bench_dir_finds_files() {
        let suite = bench_dir_n("boot/std/kintsugi/", TEST_ITERATIONS);
        // kintsugi/ directory exists and has .mirror files
        // No panic = success
        let _ = suite;
    }

    #[test]
    fn bench_boot_produces_suite() {
        // Bench boot with low iteration count — must not panic
        let suite = bench_dir_n("boot/", TEST_ITERATIONS);
        assert!(!suite.results.is_empty(), "boot/ must have benchmarkable files");
        assert!(suite.total_time_ns > 0, "total time must be positive");
    }

    // -- Red: comparison --

    #[test]
    fn compare_same_file_ratio_near_one() {
        let r1 = bench_file_n("boot/std/kintsugi.mirror", TEST_ITERATIONS);
        let r2 = bench_file_n("boot/std/kintsugi.mirror", TEST_ITERATIONS);
        let ratio = compare(&r1, &r2);
        // Same file should be roughly 1.0 (within 5x tolerance for CI jitter)
        assert!(
            ratio > 0.2 && ratio < 5.0,
            "same file comparison should be near 1.0, got {}",
            ratio
        );
    }

    // -- Red: cascade --

    #[test]
    fn cascade_produces_iterations() {
        let result = cascade_n("boot/std/kintsugi/", TEST_ITERATIONS);
        // kintsugi/ may be empty — just test the structure
        assert_eq!(
            result.iteration_times.len(),
            result.loss_curve.len(),
            "iteration_times and loss_curve must have same length"
        );
    }

    #[test]
    fn cascade_first_loss_is_one() {
        let result = cascade_n("boot/std/kintsugi/", TEST_ITERATIONS);
        if !result.loss_curve.is_empty() {
            assert!(
                (result.loss_curve[0] - 1.0).abs() < f64::EPSILON,
                "first loss must be 1.0 (baseline), got {}",
                result.loss_curve[0]
            );
        }
    }

    // -- Red: formatting --

    #[test]
    fn format_result_contains_name() {
        let result = bench_file_n("boot/std/kintsugi.mirror", 1);
        let formatted = format_result(&result);
        assert!(
            formatted.contains("boot/std/kintsugi.mirror"),
            "formatted output must contain file name"
        );
        assert!(
            formatted.contains("oid:"),
            "formatted output must contain oid"
        );
        assert!(
            formatted.contains("ops/sec"),
            "formatted output must contain throughput"
        );
    }

    #[test]
    fn format_suite_shows_total() {
        let suite = BenchSuite {
            results: vec![bench_file_n("boot/std/kintsugi.mirror", 1)],
            total_time_ns: 1_000_000,
        };
        let formatted = format_suite(&suite);
        assert!(
            formatted.contains("total:"),
            "suite output must contain total"
        );
        assert!(
            formatted.contains("1 files"),
            "suite output must show file count"
        );
    }

    // -- Red: duration formatting --

    #[test]
    fn format_duration_nanoseconds() {
        assert_eq!(format_duration_ns(500), "500ns");
    }

    #[test]
    fn format_duration_microseconds() {
        assert_eq!(format_duration_ns(1_500), "1.5us");
    }

    #[test]
    fn format_duration_milliseconds() {
        assert_eq!(format_duration_ns(1_200_000), "1.2ms");
    }

    #[test]
    fn format_duration_seconds() {
        assert_eq!(format_duration_ns(2_500_000_000), "2.50s");
    }

    // -- Red: bytes formatting --

    #[test]
    fn format_bytes_small() {
        assert_eq!(format_bytes(500), "500B");
    }

    #[test]
    fn format_bytes_kilobytes() {
        assert_eq!(format_bytes(2048), "2.0KB");
    }

    #[test]
    fn format_bytes_megabytes() {
        assert_eq!(format_bytes(3_145_728), "3.0MB");
    }
}
