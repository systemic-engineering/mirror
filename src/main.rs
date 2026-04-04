//! conversation — the human interface to the Abyss.
//!
//! ```sh
//! conversation schema.conv ./input     # apply grammar to input
//! conversation settle ./src            # loop until convergence
//! conversation test cogito.conv        # run tests
//! conversation shell                   # REPL
//! #!/usr/bin/env conversation
//! ```

use std::io::{self, BufRead, Write};
use std::process;

use mirror::domain::filesystem::{Filesystem, Folder};
use mirror::model::Mirror;
use mirror::packages::{self, PackageRegistry};
use mirror::property;
use mirror::resolve::{Conversation, Resolve};
use mirror::{Parse, Vector};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("conversation — fold | prism | traversal | lens | iso");
        eprintln!();
        eprintln!("usage: conversation <grammar> <input>     apply grammar to input");
        eprintln!("       conversation test <file.conv>      run tests");
        eprintln!("       conversation shell [path]           REPL");
        eprintln!("       conversation boot [dir]             boot the garden");
        eprintln!("       conversation fmt [--settle] [--train] <file>  format / settle / train");
        eprintln!("       conversation settle <input>         loop until convergence");
        eprintln!("       conversation train                  retrain from fixtures");
        eprintln!("       conversation resolve                resolve tension → resolved");
        eprintln!("       conversation -e '<expr>' [path]     evaluate expression");
        process::exit(1);
    }

    // Parse flags
    let has_settle = args.iter().any(|a| a == "--settle");
    let has_train = args.iter().any(|a| a == "--train");
    let positional: Vec<&String> = args
        .iter()
        .skip(1)
        .filter(|a| !a.starts_with("--"))
        .collect();

    match positional.first().map(|s| s.as_str()) {
        Some("fmt") => {
            if positional.len() < 2 {
                eprintln!("usage: conversation fmt [--settle] [--train] <file.conv>");
                process::exit(1);
            }
            let conv_path = positional[1];
            fmt_cmd(conv_path, has_settle, has_train);
        }
        Some("settle") => {
            if positional.len() < 2 {
                eprintln!("usage: abyss settle <file.conv|dir>");
                process::exit(1);
            }
            let conv_path = positional[1].as_str();
            let source = if std::path::Path::new(conv_path).is_dir() {
                String::new()
            } else {
                std::fs::read_to_string(conv_path).unwrap_or_else(|e| {
                    eprintln!("mirror: {}: {}", conv_path, e);
                    process::exit(1);
                })
            };
            settle_cmd(&source, conv_path);
        }
        Some("test") => {
            if positional.len() < 2 {
                eprintln!("usage: conversation test <file.conv>");
                process::exit(1);
            }
            let conv_path = positional[1];
            let source = match std::fs::read_to_string(conv_path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("conversation: {}: {}", conv_path, e);
                    process::exit(1);
                }
            };
            let self_dir = std::path::Path::new(conv_path)
                .parent()
                .unwrap_or(std::path::Path::new("."))
                .to_path_buf();
            let resolve = load_packages(&self_dir);
            run_tests(&source, &resolve);
        }
        Some("shell") => {
            let self_dir = std::env::current_dir().unwrap_or(std::path::PathBuf::from("."));
            let resolve = load_packages(&self_dir);
            let path = positional.get(1).map(|s| s.as_str()).unwrap_or(".");
            shell(path, &resolve);
        }
        Some("-e") => {
            if positional.len() < 2 {
                eprintln!("usage: conversation -e '<expr>' [path]");
                process::exit(1);
            }
            let self_dir = std::env::current_dir().unwrap_or(std::path::PathBuf::from("."));
            let resolve = load_packages(&self_dir);
            let source = format!("out {}\n", positional[1]);
            let path = positional.get(2).map(|s| s.as_str()).unwrap_or(".");
            run(&source, path, &resolve);
        }
        Some("train") => {
            train_cmd();
        }
        Some("resolve") => {
            resolve_cmd();
        }
        #[cfg(feature = "db")]
        Some("db") => {
            let db_args: Vec<String> = args[2..].to_vec();
            mirror::db::cli(&db_args);
        }
        None => {
            eprintln!("conversation — fold | prism | traversal | lens | iso");
            process::exit(1);
        }
        // Default: treat first positional as grammar file
        _ => {
            let conv_path = positional[0];
            let source = match std::fs::read_to_string(conv_path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("conversation: {}: {}", conv_path, e);
                    process::exit(1);
                }
            };
            let self_dir = std::path::Path::new(conv_path)
                .parent()
                .unwrap_or(std::path::Path::new("."))
                .to_path_buf();
            let resolve = load_packages(&self_dir);
            let path = positional.get(1).map(|s| s.as_str()).unwrap_or(".");
            run(&source, path, &resolve);
        }
    }
}

fn settle_cmd(source: &str, path: &str) {
    use mirror::abyss::{self, AbyssConfig, PrismLoop, Termination};
    use prism::{Beam, Oid, Precision};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // If path is a directory, settle ALL .conv files as lenses on one graph.
    // If path is a file, settle that single file.
    let sources: Vec<(String, String)> = if std::path::Path::new(path).is_dir() {
        let mut entries: Vec<_> = std::fs::read_dir(path)
            .unwrap_or_else(|e| {
                eprintln!("mirror: {}: {}", path, e);
                process::exit(1);
            })
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("conv"))
            .map(|e| {
                let p = e.path();
                let s = std::fs::read_to_string(&p).unwrap_or_default();
                (p.display().to_string(), s)
            })
            .collect();
        entries.sort_by(|a, b| a.0.cmp(&b.0)); // sort by filename = layer order
        entries
    } else {
        vec![(path.to_string(), source.to_string())]
    };

    // Parse all sources into AST nodes, accumulate into one graph.
    let mut graph: Vec<String> = Vec::new();

    for (file_path, file_source) in &sources {
        let ast = match Parse.trace(file_source.to_string()).into_result() {
            Ok(tree) => tree,
            Err(e) => {
                eprintln!("mirror: {}: parse error: {}", file_path, e);
                continue;
            }
        };
        // Each AST child is a node in the combined graph
        for child in ast.children() {
            graph.push(format!("{}:{}", child.data().name, child.data().value));
        }
        eprintln!(
            "  lens {}: +{} nodes → {} total",
            std::path::Path::new(file_path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
            ast.children().len(),
            graph.len()
        );
    }

    // Now settle the combined graph.
    // The Prism over the graph: fold = identity (already decomposed),
    // lens = sort + dedup (the settling transform).
    struct GraphPrism;

    impl prism::Prism for GraphPrism {
        type Input = Vec<String>;
        type Eigenvalues = Vec<String>;
        type Projection = Vec<String>;
        type Node = String;
        type Convergence = Vec<String>;
        type Crystal = Vec<String>;

        fn fold(&self, input: &Vec<String>) -> Beam<Vec<String>> {
            Beam::new(input.clone())
        }

        fn prism(&self, eigenvalues: &Vec<String>, _precision: Precision) -> Beam<Vec<String>> {
            Beam::new(eigenvalues.clone())
        }

        fn traversal(&self, projection: &Vec<String>) -> Vec<Beam<String>> {
            projection
                .iter()
                .enumerate()
                .map(|(i, s)| Beam::new(s.clone()).with_step(Oid::new(format!("{}", i))))
                .collect()
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

    impl PrismLoop for GraphPrism {
        fn fold_from_projection(&self, projection: &Vec<String>) -> Vec<String> {
            projection.clone()
        }
    }

    let prism = GraphPrism;
    let config = AbyssConfig {
        max_cycles: 64,
        precision: Precision::new(0.0),
        oscillation_window: 4,
    };

    let hash_fn = |v: &Vec<String>| -> Oid {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        Oid::new(format!("{:016x}", hasher.finish()))
    };

    // The settling transform: sort, dedup, and measure loss
    let (beam, termination) = abyss::settle_loop(
        &prism,
        &graph,
        &config,
        &|mut v| {
            v.sort();
            v.dedup();
            v
        },
        &hash_fn,
    );

    // Output
    eprintln!();
    eprintln!("abyss settle: {}", path);
    match &termination {
        Termination::Settled { cycles } => {
            eprintln!("  settled in {} cycles", cycles);
        }
        Termination::BudgetExhausted { cycles, .. } => {
            eprintln!("  budget exhausted after {} cycles", cycles);
        }
        Termination::Oscillation { cycles, attractors } => {
            eprintln!(
                "  oscillation after {} cycles ({} attractors)",
                cycles,
                attractors.len()
            );
        }
    }
    eprintln!("  nodes: {}", beam.result.len());
    eprintln!("  path: {} steps", beam.path.len());
    eprintln!("  loss: {}", beam.loss);

    for node in &beam.result {
        println!("  {}", node);
    }
}

fn train_cmd() {
    use mirror::classifier::{self, Example, TrainConfig};
    use mirror::parse::Parse;
    use mirror::Vector;

    let fixtures_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/settle");
    let training_data_path = fixtures_dir.join("training_data.json");

    let training_json = std::fs::read_to_string(&training_data_path).unwrap_or_else(|e| {
        eprintln!("train: {}: {}", training_data_path.display(), e);
        process::exit(1);
    });
    let entries: Vec<serde_json::Value> = serde_json::from_str(&training_json).unwrap();

    let mut examples = Vec::new();

    for entry in &entries {
        let fixture_name = entry["fixture"].as_str().unwrap();
        let label = entry["optic"].as_u64().unwrap() as usize;
        let fixture_path = fixtures_dir.join(fixture_name);

        let source = std::fs::read_to_string(&fixture_path).unwrap_or_else(|e| {
            eprintln!("train: {}: {}", fixture_path.display(), e);
            process::exit(1);
        });

        // Parse and extract graph features
        let ast = Parse.trace(source).into_result().unwrap_or_else(|e| {
            eprintln!("train: parse {}: {}", fixture_name, e);
            process::exit(1);
        });

        let graph: Vec<String> = ast
            .children()
            .iter()
            .map(|c| format!("{}:{}", c.data().name, c.data().value))
            .collect();

        let features = spectral_features(&graph);

        eprintln!("  {:30} label={:2} n={:.2} dup={:.2} f={:.2} p={:.2} t={:.2} l={:.2} i={:.2} ent={:.2} edge={:.2} par={:.2} var={:.2} div={:.2}",
            fixture_name, label,
            features[0], features[1], features[2], features[3], features[4],
            features[5], features[6], features[7], features[8], features[9],
            features[10], features[11]);

        examples.push(Example { features, label });
    }

    eprintln!("\ntraining on {} labeled examples...", examples.len());

    let config = TrainConfig {
        learning_rate: 0.05,
        epochs: 3000,
        augmentation_noise: 0.08,
        augmentation_factor: 100,
    };
    let (weights, loss, accuracy) = classifier::train(&examples, &config);

    eprintln!("  loss:     {:.4}", loss);
    eprintln!("  accuracy: {:.1}%", accuracy * 100.0);

    // Verify each example
    let mut misses = 0;
    for (i, ex) in examples.iter().enumerate() {
        let (optic, confidence, _) = classifier::classify(&weights, &ex.features);
        let correct = optic as usize == ex.label;
        if !correct {
            misses += 1;
        }
        eprintln!(
            "  {:30} → {:?} ({:.1}%) {}",
            entries[i]["fixture"].as_str().unwrap(),
            optic,
            confidence * 100.0,
            if correct { "✓" } else { "✗" }
        );
    }

    if misses > 0 {
        eprintln!("\n  {} misclassified — weights NOT written", misses);
        process::exit(1);
    }

    // Write weights
    let weights_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("mirror.weights");
    let bytes = weights.to_bytes();
    std::fs::write(&weights_path, &bytes).unwrap_or_else(|e| {
        eprintln!("train: write {}: {}", weights_path.display(), e);
        process::exit(1);
    });
    eprintln!(
        "\n  wrote {} bytes to {}",
        bytes.len(),
        weights_path.display()
    );
}

/// Extract spectral features from a graph (shared between train_cmd and settle_cmd).
///
/// 12 features using the first 12 of 32 input dimensions:
///  0: node_count_norm      — graph size / 50
///  1: duplicate_ratio      — 1 - unique/n
///  2: fold_ratio           — fraction of fold-def nodes
///  3: prism_ratio          — fraction of prism-def nodes
///  4: traversal_ratio      — fraction of traversal-def nodes
///  5: lens_ratio           — fraction of lens-def nodes
///  6: iso_ratio            — fraction of iso-def nodes
///  7: prefix_entropy       — unique prefixes / 10
///  8: edge_density         — namespace co-occurrence
///  9: param_ratio          — fraction of parameterized nodes
/// 10: variant_ratio        — fraction of variant nodes
/// 11: keyword_diversity    — how many of the 5 crystal types appear (/ 5)
pub fn spectral_features(graph: &[String]) -> [f64; mirror::classifier::INPUT_DIM] {
    let mut features = [0.0; mirror::classifier::INPUT_DIM];
    let n = graph.len() as f64;
    if n == 0.0 {
        return features;
    }

    // 0: node count
    features[0] = (n / 50.0).min(1.0);

    // 1: duplicate ratio
    let mut sorted = graph.to_vec();
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

    // 8: edge density — co-occurrence within same prefix
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

    // 9: param ratio — nodes containing parentheses in value
    let param_count = graph
        .iter()
        .filter(|s| s.split(':').nth(1).is_some_and(|v| v.contains('(')))
        .count() as f64;
    features[9] = param_count / n;

    // 10: variant ratio — nodes named "variant"
    let variant_count = graph.iter().filter(|s| s.starts_with("variant:")).count() as f64;
    features[10] = variant_count / n;

    // 11: keyword diversity — how many of the 5 crystal keywords appear
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

fn fmt_cmd(conv_path: &str, settle: bool, train: bool) {
    use mirror::ast::{self as ast_mod, AstNode, Span};
    use mirror::classifier::{self, Optic};
    use mirror::domain::conversation::Kind;
    use mirror::emit;
    use mirror::parse::Parse;
    use mirror::prism::Prism as PrismTree;
    use mirror::Vector;

    let source = std::fs::read_to_string(conv_path).unwrap_or_else(|e| {
        eprintln!("fmt: {}: {}", conv_path, e);
        process::exit(1);
    });

    let ast = Parse
        .trace(source.clone())
        .into_result()
        .unwrap_or_else(|e| {
            eprintln!("fmt: {}: parse error: {}", conv_path, e);
            process::exit(1);
        });

    let graph: Vec<String> = ast
        .children()
        .iter()
        .map(|c| format!("{}:{}", c.data().name, c.data().value))
        .collect();
    let initial_len = graph.len();

    if !settle {
        // Plain fmt: just emit canonical form (sorted, no dedup)
        let mut children = ast.children().to_vec();
        children.sort_by(|a, b| {
            let ka = format!("{}:{}", a.data().name, a.data().value);
            let kb = format!("{}:{}", b.data().name, b.data().value);
            ka.cmp(&kb)
        });
        let sorted = ast_mod::ast_branch(Kind::Decl, "root", "fmt", Span::new(0, 0), children);
        let output = emit::emit(&sorted);
        print!("{}", output);
        return;
    }

    // --settle: classify and apply optic
    let weights = classifier::trained();
    let features = spectral_features(&graph);
    let (optic, confidence, _) = classifier::classify(&weights, &features);

    let resolved_ast: PrismTree<AstNode> = match optic {
        Optic::Noop => ast.clone(),
        _ => {
            let mut children = ast.children().to_vec();
            children.sort_by(|a, b| {
                let ka = format!("{}:{}", a.data().name, a.data().value);
                let kb = format!("{}:{}", b.data().name, b.data().value);
                ka.cmp(&kb)
            });
            children.dedup_by(|a, b| {
                let ka = format!("{}:{}", a.data().name, a.data().value);
                let kb = format!("{}:{}", b.data().name, b.data().value);
                ka == kb
            });
            ast_mod::ast_branch(Kind::Decl, "root", "resolved", Span::new(0, 0), children)
        }
    };

    let resolved_len = resolved_ast.children().len();
    let output = emit::emit(&resolved_ast);

    eprintln!(
        "  {:?} ({:.0}%) {} → {} nodes",
        optic,
        confidence * 100.0,
        initial_len,
        resolved_len
    );

    if train {
        // --train: write the tension/resolved pair + fine-tune weights
        let fixtures_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures");
        let tension_dir = fixtures_dir.join("tension");
        let resolved_dir = fixtures_dir.join("resolved");
        std::fs::create_dir_all(&tension_dir).unwrap();
        std::fs::create_dir_all(&resolved_dir).unwrap();

        let name = std::path::Path::new(conv_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        // Tension = the original source
        std::fs::write(tension_dir.join(name), &source).unwrap();
        // Resolved = what the Abyss produced
        std::fs::write(resolved_dir.join(name), &output).unwrap();

        // Fine-tune on this example
        let example = classifier::Example {
            features,
            label: optic as usize,
        };
        let (new_weights, loss, _) = classifier::fine_tune(weights, &[example]);
        let weights_path =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("mirror.weights");
        std::fs::write(&weights_path, new_weights.to_bytes()).unwrap();

        eprintln!(
            "  train: loss={:.4} → {}",
            loss,
            weights_path.file_name().unwrap().to_str().unwrap()
        );
    }

    // Write the settled output
    print!("{}", output);
}

fn resolve_cmd() {
    use mirror::ast::{self as ast_mod, AstNode, Span};
    use mirror::classifier::{self, Optic};
    use mirror::domain::conversation::Kind;
    use mirror::emit;
    use mirror::parse::Parse;
    use mirror::prism::Prism as PrismTree;
    use mirror::Vector;

    let tension_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/tension");
    let resolved_dir =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/resolved");

    std::fs::create_dir_all(&resolved_dir).unwrap();

    let weights = classifier::trained();

    let mut entries: Vec<_> = std::fs::read_dir(&tension_dir)
        .unwrap_or_else(|e| {
            eprintln!("resolve: {}: {}", tension_dir.display(), e);
            process::exit(1);
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("conv"))
        .collect();
    entries.sort_by_key(|e| e.path());

    for entry in &entries {
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap();
        let source = std::fs::read_to_string(&path).unwrap();

        let ast = match Parse.trace(source).into_result() {
            Ok(tree) => tree,
            Err(e) => {
                eprintln!("  SKIP {:25} parse error: {}", name, e);
                continue;
            }
        };

        // Extract features from the graph representation
        let graph: Vec<String> = ast
            .children()
            .iter()
            .map(|c| format!("{}:{}", c.data().name, c.data().value))
            .collect();
        let initial_len = graph.len();

        let features = spectral_features(&graph);
        let (optic, confidence, _) = classifier::classify(&weights, &features);

        // Apply the optic transform at the AST level
        let resolved_ast: PrismTree<AstNode> = match optic {
            Optic::Noop => ast.clone(),
            _ => {
                // Sort children by content address, dedup by name:value
                let mut children = ast.children().to_vec();
                children.sort_by(|a, b| {
                    let ka = format!("{}:{}", a.data().name, a.data().value);
                    let kb = format!("{}:{}", b.data().name, b.data().value);
                    ka.cmp(&kb)
                });
                children.dedup_by(|a, b| {
                    let ka = format!("{}:{}", a.data().name, a.data().value);
                    let kb = format!("{}:{}", b.data().name, b.data().value);
                    ka == kb
                });
                ast_mod::ast_branch(Kind::Decl, "root", "resolved", Span::new(0, 0), children)
            }
        };

        let resolved_len = resolved_ast.children().len();
        let reduced = initial_len - resolved_len;

        eprintln!(
            "  {:25} {:?} ({:.0}%) {:2} → {:2} ({:+})",
            name,
            optic,
            confidence * 100.0,
            initial_len,
            resolved_len,
            -(reduced as i32)
        );

        // Emit back to .conv syntax
        let output = emit::emit(&resolved_ast);
        let resolved_path = resolved_dir.join(name);
        std::fs::write(&resolved_path, &output).unwrap();
    }
}

/// Discover packages from priority-ordered roots relative to `self_dir`.
fn load_packages(self_dir: &std::path::Path) -> Resolve {
    let roots = PackageRegistry::package_roots(self_dir);
    if roots.is_empty() {
        return Resolve::new();
    }
    match PackageRegistry::discover_ordered(&roots) {
        Ok(r) => match r.to_namespace() {
            Ok(ns) => Resolve::new().with_namespace(ns),
            Err(e) => {
                eprintln!("conversation: packages: {}", e);
                Resolve::new()
            }
        },
        Err(e) => {
            eprintln!("conversation: packages: {}", e);
            Resolve::new()
        }
    }
}

fn run_tests(source: &str, resolve: &Resolve) {
    let (_, test_section) = packages::split_test_section(source);
    let test_text = match test_section {
        Some(t) => t,
        None => {
            println!("no test section found");
            return;
        }
    };

    // Build namespace: start with packages, then add the file's own grammars.
    let mut namespace = resolve.namespace().clone();
    if let Ok(ast) = Parse.trace(source.to_string()).into_result() {
        for child in ast.children() {
            if child.data().is_decl("grammar") {
                if let Ok(domain) = Mirror::from_grammar(child) {
                    let domain_name = domain.domain_name().to_string();
                    namespace.register_domain(&domain_name, domain);
                }
            }
        }
    }
    let results = match property::check_all(&namespace, test_text) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("conversation: test: {}", e);
            process::exit(1);
        }
    };

    let mut failed = 0;
    for result in &results {
        match &result.verdict {
            property::Verdict::Pass => {
                println!("PASS {}", result.name);
            }
            property::Verdict::Fail(msg) => {
                println!("FAIL {}", result.name);
                println!("     {}", msg);
                failed += 1;
            }
        }
    }

    if results.is_empty() {
        println!("no tests");
    } else {
        println!(
            "\n{} tests, {} passed, {} failed",
            results.len(),
            results.len() - failed,
            failed
        );
    }

    if failed > 0 {
        process::exit(1);
    }
}

fn run(source: &str, input_path: &str, resolve: &Resolve) {
    let resolved = match Conversation::<Filesystem>::from_source_with(source, resolve.clone()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("conversation: {}", e);
            process::exit(1)
        }
    };
    let tree = Folder::read_tree(input_path);
    let value = resolved.trace(tree).into_result().unwrap();
    let json = serde_json::to_string_pretty(&value).unwrap();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let _ = out.write_all(json.as_bytes());
    let _ = out.write_all(b"\n");
}

fn shell(path: &str, resolve: &Resolve) {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut stdout = io::stdout();

    eprintln!("conversation shell — {}", path);
    eprintln!("type expressions, ctrl+d to exit\n");

    for line in reader.lines() {
        let _ = write!(stdout, "conversation> ");
        let _ = stdout.flush();

        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("conversation: read error: {}", e);
                break;
            }
        };

        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }

        // Build a .conv source from the expression
        let source = format!("out {}\n", line);

        let resolved = match Conversation::<Filesystem>::from_source_with(&source, resolve.clone())
        {
            Ok(conv) => conv,
            Err(e) => {
                eprintln!("  error: {}", e);
                continue;
            }
        };

        let tree = Folder::read_tree(path);

        let value = resolved.trace(tree).into_result().unwrap();
        let json = serde_json::to_string_pretty(&value).unwrap();
        println!("{}", json);
    }

    let _ = writeln!(stdout);
}
