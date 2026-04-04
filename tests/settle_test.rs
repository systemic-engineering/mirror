//! Integration tests: every fixture in fixtures/settle/ must settle.
//!
//! Each .conv file is parsed, accumulated into a graph, and run through
//! the Abyss settle loop. The test passes if the graph reaches a fixed
//! point (Termination::Settled) within the budget.

use mirror::abyss::{self, AbyssConfig, PrismLoop, Termination};
use mirror::parse::Parse;
use mirror::Vector;
use prism::{Beam, Oid, Precision};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct SettlePrism;

impl prism::Prism for SettlePrism {
    type Input = Vec<String>;
    type Eigenvalues = Vec<String>;
    type Projection = Vec<String>;
    type Node = String;
    type Convergence = Vec<String>;
    type Crystal = Vec<String>;

    fn fold(&self, input: &Vec<String>) -> Beam<Vec<String>> {
        Beam::new(input.clone())
    }

    fn prism(&self, ev: &Vec<String>, _p: Precision) -> Beam<Vec<String>> {
        Beam::new(ev.clone())
    }

    fn traversal(&self, proj: &Vec<String>) -> Vec<Beam<String>> {
        proj.iter().map(|s| Beam::new(s.clone())).collect()
    }

    fn lens(
        &self,
        beam: Beam<Vec<String>>,
        f: &dyn Fn(Vec<String>) -> Vec<String>,
    ) -> Beam<Vec<String>> {
        beam.map(f)
    }

    fn iso(&self, beam: Beam<Vec<String>>) -> Vec<String> {
        beam.result
    }
}

impl PrismLoop for SettlePrism {
    fn fold_from_projection(&self, p: &Vec<String>) -> Vec<String> {
        p.clone()
    }
}

fn hash_strings(v: &Vec<String>) -> Oid {
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    Oid::new(format!("{:016x}", hasher.finish()))
}

fn settle_file(path: &std::path::Path) -> (Vec<String>, Termination) {
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

    let config = AbyssConfig {
        max_cycles: 64,
        ..Default::default()
    };

    let (beam, term) = abyss::settle_loop(
        &SettlePrism,
        &graph,
        &config,
        &|mut v| {
            v.sort();
            v.dedup();
            v
        },
        &hash_strings,
    );

    (beam.result, term)
}

#[test]
fn all_settle_fixtures_settle() {
    let fixtures_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/settle");

    let mut entries: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .unwrap_or_else(|e| panic!("read fixtures/settle: {}", e))
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("conv"))
        .collect();
    entries.sort_by_key(|e| e.path());

    assert!(
        !entries.is_empty(),
        "no .conv fixtures found in fixtures/settle/"
    );

    let mut total = 0;
    let mut settled = 0;

    for entry in &entries {
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap();
        let (result, term) = settle_file(&path);

        total += 1;
        match &term {
            Termination::Settled { cycles } => {
                settled += 1;
                eprintln!(
                    "  SETTLED  {}: {} nodes, {} cycles",
                    name,
                    result.len(),
                    cycles
                );
            }
            Termination::BudgetExhausted { cycles, .. } => {
                eprintln!(
                    "  BUDGET   {}: {} nodes, {} cycles",
                    name,
                    result.len(),
                    cycles
                );
            }
            Termination::Oscillation { cycles, attractors } => {
                eprintln!(
                    "  OSCILLATE {}: {} nodes, {} cycles, {} attractors",
                    name,
                    result.len(),
                    cycles,
                    attractors.len()
                );
            }
        }

        assert!(
            matches!(term, Termination::Settled { .. }),
            "{} did not settle: {:?}",
            name,
            term
        );
    }

    eprintln!("\n  {}/{} fixtures settled", settled, total);
}

#[test]
fn boot_sequence_settles_as_fixture() {
    let boot_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("boot");

    let mut entries: Vec<_> = std::fs::read_dir(&boot_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("conv"))
        .collect();
    entries.sort_by_key(|e| e.path());

    // Accumulate all boot files into one graph
    let mut graph: Vec<String> = Vec::new();
    for entry in &entries {
        let source = std::fs::read_to_string(entry.path()).unwrap();
        if let Ok(ast) = Parse.trace(source).into_result() {
            for child in ast.children() {
                graph.push(format!("{}:{}", child.data().name, child.data().value));
            }
        }
    }

    let config = AbyssConfig {
        max_cycles: 64,
        ..Default::default()
    };

    let (beam, term) = abyss::settle_loop(
        &SettlePrism,
        &graph,
        &config,
        &|mut v| {
            v.sort();
            v.dedup();
            v
        },
        &hash_strings,
    );

    assert!(
        matches!(term, Termination::Settled { .. }),
        "boot sequence must settle: {:?}",
        term
    );
    assert_eq!(beam.result.len(), 17, "boot settles to 17 keywords");
    assert!(beam.is_lossless(), "boot settlement must be lossless");
}
