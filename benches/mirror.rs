//! Benchmarks for mirror — the cost of compilation, measured.
//!
//! Four measurements:
//! 1. parse_form — tokenize + parse a .mirror file
//! 2. compile_source — full pipeline (parse + content-address + fragment)
//! 3. compile_boot_dir — the entire boot sequence (all .mirror files, registry)
//! 4. crystal materialization — Shatter compile_form + decompile round-trip

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mirror::mirror_runtime::{parse_form, MirrorRuntime, Shatter};
use std::path::Path;

// ---------------------------------------------------------------------------
// Source fixtures
// ---------------------------------------------------------------------------

const MINIMAL_SOURCE: &str = r#"
in @prism

type point(f64, f64)

out point
"#;

const GRAMMAR_SOURCE: &str = r#"
in @prism
in @meta

type duration(u64)
type measurement {
  name: ref,
  duration: duration,
  loss: loss,
}

type baseline {
  measurement: measurement,
  label: ref,
}

type speedup {
  ratio: precision,
  measurement: measurement,
  baseline: baseline,
}

grammar @bench {
  abstract action bench(action) -> imperfect
  abstract action compare(measurement, baseline) -> speedup
  abstract action profile(grammar) -> imperfect

  recover |result, loss| {
    result
  }

  rescue |error| {
    error
  }
}

out duration
out measurement
out baseline
out speedup
out @bench
"#;

// ---------------------------------------------------------------------------
// 1. parse_form — tokenize + parse
// ---------------------------------------------------------------------------

fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");

    group.bench_function("minimal", |b| {
        b.iter(|| parse_form(black_box(MINIMAL_SOURCE)).ok().unwrap())
    });

    group.bench_function("grammar", |b| {
        b.iter(|| parse_form(black_box(GRAMMAR_SOURCE)).ok().unwrap())
    });

    // Parse each boot file individually
    let boot_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("boot");
    if boot_dir.exists() {
        let mut entries: Vec<_> = std::fs::read_dir(&boot_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("mirror"))
            .collect();
        entries.sort();

        for path in &entries {
            let stem = path.file_stem().unwrap().to_str().unwrap().to_string();
            let source = std::fs::read_to_string(path).unwrap();
            group.bench_function(&format!("boot/{}", stem), |b| {
                b.iter(|| parse_form(black_box(&source)).ok().unwrap())
            });
        }
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// 2. compile_source — full pipeline
// ---------------------------------------------------------------------------

fn bench_compile_source(c: &mut Criterion) {
    let mut group = c.benchmark_group("compile_source");
    let runtime = MirrorRuntime::new();

    group.bench_function("minimal", |b| {
        b.iter(|| runtime.compile_source(black_box(MINIMAL_SOURCE)).unwrap())
    });

    group.bench_function("grammar", |b| {
        b.iter(|| runtime.compile_source(black_box(GRAMMAR_SOURCE)).unwrap())
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// 3. compile_boot_dir — the entire boot sequence
// ---------------------------------------------------------------------------

fn bench_compile_boot(c: &mut Criterion) {
    let mut group = c.benchmark_group("compile_boot");
    let runtime = MirrorRuntime::new();
    let boot_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("boot");

    group.sample_size(20); // boot is heavier, fewer samples for stability

    group.bench_function("full_boot", |b| {
        b.iter(|| {
            let tmp = tempfile::tempdir().unwrap();
            runtime
                .compile_boot_dir(black_box(&boot_dir), tmp.path())
                .unwrap()
        })
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// 4. crystal materialization — Shatter compile + decompile
// ---------------------------------------------------------------------------

fn bench_crystal(c: &mut Criterion) {
    let mut group = c.benchmark_group("crystal");
    let runtime = MirrorRuntime::new();

    // Pre-compile a form to use as input
    let compiled = runtime.compile_source(GRAMMAR_SOURCE).unwrap();

    group.bench_function("compile_form", |b| {
        let shatter = Shatter;
        b.iter(|| shatter.compile_form(black_box(&compiled.form)))
    });

    group.bench_function("decompile", |b| {
        let shatter = Shatter;
        b.iter(|| shatter.decompile(black_box(&compiled.fragment)))
    });

    group.bench_function("round_trip", |b| {
        let shatter = Shatter;
        b.iter(|| {
            let form = black_box(compiled.form.clone());
            let frag = shatter.compile_form(&form);
            shatter.decompile(&frag)
        })
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// The cascade
// ---------------------------------------------------------------------------

criterion_group!(
    benches,
    bench_parse,
    bench_compile_source,
    bench_compile_boot,
    bench_crystal,
);
criterion_main!(benches);
