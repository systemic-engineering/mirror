//! Integration tests: every fixture in fixtures/settle/ must settle.
//!
//! Each .conv file is parsed into a graph. The Abyss runs the settle loop
//! with the TRAINED CLASSIFIER choosing which optic to apply each cycle.
//! The test passes if the graph reaches a fixed point within budget.

use mirror::abyss::{self, AbyssConfig, PrismLoop, Termination};
use mirror::classifier::{self, Optic, INPUT_DIM};
use mirror::parse::Parse;
use mirror::Vector;
use prism::{Beam, Oid, Precision, ShannonLoss};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// ---------------------------------------------------------------------------
// The Prism implementation for settling grammars
// ---------------------------------------------------------------------------

struct GrammarPrism {
    weights: classifier::Weights,
}

impl GrammarPrism {
    fn new() -> Self {
        GrammarPrism {
            weights: classifier::trained(),
        }
    }

    /// Extract spectral features from the current graph state.
    /// 12 features matching the training pipeline in main.rs.
    fn spectral_features(graph: &Vec<String>) -> [f64; INPUT_DIM] {
        let mut features = [0.0; INPUT_DIM];
        let n = graph.len() as f64;
        if n == 0.0 {
            return features;
        }

        // 0: node count
        features[0] = (n / 50.0).min(1.0);

        // 1: duplicate ratio
        let mut sorted = graph.clone();
        sorted.sort();
        sorted.dedup();
        let unique = sorted.len() as f64;
        features[1] = 1.0 - (unique / n);

        // 2-6: crystal keyword distribution
        let fold_count = graph.iter().filter(|s| s.starts_with("fold-def:")).count() as f64;
        let prism_count = graph.iter().filter(|s| s.starts_with("prism-def:")).count() as f64;
        let traversal_count = graph
            .iter()
            .filter(|s| s.starts_with("traversal-def:"))
            .count() as f64;
        let lens_count = graph.iter().filter(|s| s.starts_with("lens-def:")).count() as f64;
        let iso_count = graph.iter().filter(|s| s.starts_with("iso-def:")).count() as f64;
        features[2] = fold_count / n;
        features[3] = prism_count / n;
        features[4] = traversal_count / n;
        features[5] = lens_count / n;
        features[6] = iso_count / n;

        // 7: prefix entropy
        let prefixes: Vec<&str> = graph.iter().filter_map(|s| s.split(':').next()).collect();
        let unique_prefixes: std::collections::HashSet<&&str> = prefixes.iter().collect();
        features[7] = (unique_prefixes.len() as f64 / 10.0).min(1.0);

        // 8: edge density — namespace co-occurrence
        let mut edges = 0usize;
        for i in 0..graph.len() {
            for j in (i + 1)..graph.len() {
                let pi = graph[i].split(':').next().unwrap_or("");
                let pj = graph[j].split(':').next().unwrap_or("");
                if pi == pj {
                    edges += 1;
                }
            }
        }
        let possible = graph.len() * graph.len().saturating_sub(1) / 2;
        features[8] = if possible > 0 {
            edges as f64 / possible as f64
        } else {
            0.0
        };

        // 9: param ratio
        let param_count = graph
            .iter()
            .filter(|s| s.split(':').nth(1).map_or(false, |v| v.contains('(')))
            .count() as f64;
        features[9] = param_count / n;

        // 10: variant ratio
        let variant_count = graph.iter().filter(|s| s.starts_with("variant:")).count() as f64;
        features[10] = variant_count / n;

        // 11: keyword diversity
        let mut keyword_types = 0u8;
        if fold_count > 0.0 {
            keyword_types += 1;
        }
        if prism_count > 0.0 {
            keyword_types += 1;
        }
        if traversal_count > 0.0 {
            keyword_types += 1;
        }
        if lens_count > 0.0 {
            keyword_types += 1;
        }
        if iso_count > 0.0 {
            keyword_types += 1;
        }
        features[11] = keyword_types as f64 / 5.0;

        features
    }
}

impl prism::Prism for GrammarPrism {
    type Input = Vec<String>;
    type Eigenvalues = Vec<String>;
    type Projection = Vec<String>;
    type Node = String;
    type Convergence = Vec<String>;
    type Crystal = Vec<String>;
    type Precision = prism::Precision;

    fn focus(&self, input: &Vec<String>) -> Beam<Vec<String>> {
        Beam::new(input.clone())
    }

    fn project(&self, ev: &Vec<String>, precision: Precision) -> Beam<Vec<String>> {
        // Filter: keep only nodes above the precision threshold (by frequency)
        let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for s in ev {
            *counts.entry(s.as_str()).or_default() += 1;
        }
        let threshold = (precision.as_f64() * ev.len() as f64) as usize;
        let kept: Vec<String> = ev
            .iter()
            .filter(|s| counts.get(s.as_str()).copied().unwrap_or(0) > threshold)
            .cloned()
            .collect();
        let lost = ev.len() - kept.len();
        if kept.is_empty() {
            // Don't filter to nothing — keep all
            Beam::new(ev.clone())
        } else {
            Beam::new(kept)
                .with_loss(ShannonLoss::new(lost as f64))
                .with_precision(precision)
        }
    }

    fn split(&self, proj: &Vec<String>) -> Vec<Beam<String>> {
        proj.iter()
            .enumerate()
            .map(|(i, s)| Beam::new(s.clone()).with_step(Oid::new(format!("{}", i))))
            .collect()
    }

    fn zoom(
        &self,
        beam: Beam<Vec<String>>,
        f: &dyn Fn(Vec<String>) -> Vec<String>,
    ) -> Beam<Vec<String>> {
        beam.map(f)
    }

    fn refract(&self, beam: Beam<Vec<String>>) -> Vec<String> {
        beam.result
    }
}

impl PrismLoop for GrammarPrism {
    fn fold_from_projection(&self, p: &Vec<String>) -> Vec<String> {
        p.clone()
    }
}

fn hash_strings(v: &Vec<String>) -> Oid {
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    Oid::new(format!("{:016x}", hasher.finish()))
}

/// The classifier-driven transform. Each cycle:
/// 1. Extract spectral features from the graph
/// 2. Classify tension → optic
/// 3. Apply the optic as a transform
///
/// Every non-Noop optic sorts and deduplicates. The classifier chooses
/// the strategy (which optic), not whether to reduce. Noop is the only
/// optic that passes through unchanged.
fn classifier_transform(weights: &classifier::Weights) -> impl Fn(Vec<String>) -> Vec<String> + '_ {
    move |mut graph: Vec<String>| {
        let features = GrammarPrism::spectral_features(&graph);
        let (optic, _confidence, _) = classifier::classify(weights, &features);

        match optic {
            Optic::Noop => graph,
            _ => {
                graph.sort();
                graph.dedup();
                graph
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

fn settle_file(path: &std::path::Path) -> (Vec<String>, Termination, usize) {
    let source =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {}", path.display(), e));

    let ast = Parse
        .trace(source)
        .into_result()
        .unwrap_or_else(|e| panic!("parse {}: {}", path.display(), e));

    let graph: Vec<String> = ast
        .children()
        .iter()
        .map(|c| format!("{}:{}", c.data().name, c.data().value))
        .collect();

    let initial_size = graph.len();

    let gp = GrammarPrism::new();
    let config = AbyssConfig {
        max_cycles: 64,
        precision: Precision::new(0.0),
        oscillation_window: 4,
    };

    let transform = classifier_transform(&gp.weights);

    let (beam, term) = abyss::settle_loop(&gp, &graph, &config, &transform, &hash_strings);

    (beam.result, term, initial_size)
}

#[test]
fn all_settle_fixtures_settle_with_classifier() {
    let fixtures_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/settle");

    let mut entries: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("conv"))
        .collect();
    entries.sort_by_key(|e| e.path());

    assert!(!entries.is_empty());

    let mut total = 0;
    let mut settled = 0;

    for entry in &entries {
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap();
        let (result, term, initial) = settle_file(&path);

        total += 1;
        match &term {
            Termination::Settled { cycles } => {
                settled += 1;
                let reduced = initial - result.len();
                eprintln!(
                    "  SETTLED  {:25} {:2} → {:2} nodes ({:+}) in {} cycles",
                    name,
                    initial,
                    result.len(),
                    -(reduced as i32),
                    cycles
                );
            }
            Termination::BudgetExhausted { cycles, .. } => {
                eprintln!("  BUDGET   {:25} {} cycles", name, cycles);
            }
            Termination::Oscillation { cycles, .. } => {
                eprintln!("  OSCILLATE {:24} {} cycles", name, cycles);
            }
        }

        assert!(
            matches!(term, Termination::Settled { .. }),
            "{} did not settle: {:?}",
            name,
            term
        );
    }

    eprintln!("\n  {}/{} fixtures settled with classifier", settled, total);
}

#[test]
fn messy_types_reduces() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/settle/messy_types.conv");
    let (result, term, initial) = settle_file(&path);

    assert!(matches!(term, Termination::Settled { .. }));
    // Must actually reduce — messy_types has many duplicates
    assert!(
        result.len() < initial,
        "messy_types should reduce: {} → {}",
        initial,
        result.len()
    );
}

#[test]
fn redundant_layers_reduces() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/settle/redundant_layers.conv");
    let (result, term, initial) = settle_file(&path);

    assert!(matches!(term, Termination::Settled { .. }));
    assert!(
        result.len() < initial,
        "redundant_layers should reduce: {} → {}",
        initial,
        result.len()
    );
}

#[test]
fn boot_settles_with_classifier() {
    let boot_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("boot");

    let mut entries: Vec<_> = std::fs::read_dir(&boot_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("conv"))
        .collect();
    entries.sort_by_key(|e| e.path());

    let mut graph: Vec<String> = Vec::new();
    for entry in &entries {
        let source = std::fs::read_to_string(entry.path()).unwrap();
        if let Ok(ast) = Parse.trace(source).into_result() {
            for child in ast.children() {
                graph.push(format!("{}:{}", child.data().name, child.data().value));
            }
        }
    }

    let initial = graph.len();
    let gp = GrammarPrism::new();
    let config = AbyssConfig {
        max_cycles: 64,
        ..Default::default()
    };

    let (beam, term) = abyss::settle_loop(
        &gp,
        &graph,
        &config,
        &classifier_transform(&gp.weights),
        &hash_strings,
    );

    assert!(matches!(term, Termination::Settled { .. }));
    assert_eq!(beam.result.len(), 19, "boot settles to 19 keywords");
    eprintln!(
        "  boot: {} → {} nodes in {:?}",
        initial,
        beam.result.len(),
        term
    );
}
