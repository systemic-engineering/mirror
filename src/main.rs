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
use mirror::model::Domain;
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
        eprintln!("       conversation settle <input>         loop until convergence");
        eprintln!("       conversation -e '<expr>' [path]     evaluate expression");
        process::exit(1);
    }

    match args[1].as_str() {
        "settle" => {
            if args.len() < 3 {
                eprintln!("usage: abyss settle <file.conv|dir>");
                process::exit(1);
            }
            let conv_path = &args[2];
            let source = if std::path::Path::new(conv_path).is_dir() {
                String::new() // directory mode — settle_cmd reads files
            } else {
                std::fs::read_to_string(conv_path).unwrap_or_else(|e| {
                    eprintln!("abyss: {}: {}", conv_path, e);
                    process::exit(1);
                })
            };
            settle_cmd(&source, conv_path);
        }
        "test" => {
            if args.len() < 3 {
                eprintln!("usage: conversation test <file.conv>");
                process::exit(1);
            }
            let conv_path = &args[2];
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
        "shell" => {
            let self_dir = std::env::current_dir().unwrap_or(std::path::PathBuf::from("."));
            let resolve = load_packages(&self_dir);
            let path = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            shell(path, &resolve);
        }
        "-e" => {
            if args.len() < 3 {
                eprintln!("usage: conversation -e '<expr>' [path]");
                process::exit(1);
            }
            let self_dir = std::env::current_dir().unwrap_or(std::path::PathBuf::from("."));
            let resolve = load_packages(&self_dir);
            let source = format!("out {}\n", &args[2]);
            let path = args.get(3).map(|s| s.as_str()).unwrap_or(".");
            run(&source, path, &resolve);
        }
        "actor" => {
            actor_cmd(&args[2..]);
        }
        #[cfg(feature = "db")]
        "db" => {
            let db_args: Vec<String> = args[2..].to_vec();
            mirror::db::cli(&db_args);
        }
        // LSP moved to standalone binary: conversation-lsp
        _ => {
            let conv_path = &args[1];
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
            let path = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            run(&source, path, &resolve);
        }
    }
}

fn settle_cmd(source: &str, path: &str) {
    use mirror::abyss::{self, AbyssConfig, PrismLoop, Termination};
    use prism::{Beam, Oid, Precision, ShannonLoss};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // If path is a directory, settle ALL .conv files as lenses on one graph.
    // If path is a file, settle that single file.
    let sources: Vec<(String, String)> = if std::path::Path::new(path).is_dir() {
        let mut entries: Vec<_> = std::fs::read_dir(path)
            .unwrap_or_else(|e| { eprintln!("abyss: {}: {}", path, e); process::exit(1); })
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
                eprintln!("abyss: {}: parse error: {}", file_path, e);
                continue;
            }
        };
        // Each AST child is a node in the combined graph
        for child in ast.children() {
            graph.push(format!("{}:{}", child.data().name, child.data().value));
        }
        eprintln!("  lens {}: +{} nodes → {} total",
            std::path::Path::new(file_path).file_name().unwrap().to_str().unwrap(),
            ast.children().len(),
            graph.len());
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
            projection.iter().enumerate()
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
            let before = v.len();
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
            eprintln!("  oscillation after {} cycles ({} attractors)", cycles, attractors.len());
        }
    }
    eprintln!("  nodes: {}", beam.result.len());
    eprintln!("  path: {} steps", beam.path.len());
    eprintln!("  loss: {}", beam.loss);

    for node in &beam.result {
        println!("  {}", node);
    }
}

fn actor_cmd(args: &[String]) {
    if args.is_empty() {
        eprintln!("usage: conversation actor <observe|init|mount|unmount|status> [args]");
        process::exit(1);
    }

    match args[0].as_str() {
        "observe" => {
            if args.len() < 3 {
                eprintln!("usage: conversation actor observe <actor-home> <repo-path>");
                process::exit(1);
            }
            let _actor_home = &args[1];
            let repo_path = std::path::Path::new(&args[2]);
            actor_observe(repo_path);
        }
        "init" => {
            if args.len() < 2 {
                eprintln!("usage: conversation actor init <path> --role <role>");
                process::exit(1);
            }
            let path = std::path::Path::new(&args[1]);
            let role = args
                .iter()
                .position(|a| a == "--role")
                .and_then(|i| args.get(i + 1))
                .map(|s| s.as_str())
                .unwrap_or("default");
            match mirror::actor::init::init(path, role) {
                Ok(()) => eprintln!("conversation actor init: {} ({})", path.display(), role),
                Err(e) => {
                    eprintln!("conversation actor init: {e}");
                    process::exit(1);
                }
            }
        }
        "mount" => {
            if args.len() < 3 {
                eprintln!("usage: conversation actor mount <actor-home> <workspace-path>");
                process::exit(1);
            }
            let actor_home = std::path::Path::new(&args[1]);
            let workspace_path = std::path::Path::new(&args[2]);
            match mirror::actor::mount::mount(actor_home, workspace_path) {
                Ok(name) => {
                    eprintln!("mounted: {} → {}", name, workspace_path.display());
                    // Run observe on the mounted workspace
                    let mounted = actor_home.join("workspace").join(&name);
                    if let Ok(deps) = mirror::actor::observe::scan_repo(&mounted) {
                        let packages: Vec<_> = deps
                            .iter()
                            .filter(|d| d.is_package)
                            .map(|d| d.name.as_str())
                            .collect();
                        if !packages.is_empty() {
                            eprintln!("  packages needed: {}", packages.join(", "));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("conversation actor mount: {e}");
                    process::exit(1);
                }
            }
        }
        "unmount" => {
            if args.len() < 3 {
                eprintln!("usage: conversation actor unmount <actor-home> <name>");
                process::exit(1);
            }
            let actor_home = std::path::Path::new(&args[1]);
            let name = &args[2];
            match mirror::actor::mount::unmount(actor_home, name) {
                Ok(()) => eprintln!("unmounted: {}", name),
                Err(e) => {
                    eprintln!("conversation actor unmount: {e}");
                    process::exit(1);
                }
            }
        }
        "status" => {
            let candidates = mirror::actor::status::home_candidates();
            let actors = mirror::actor::status::discover_actors(&candidates);
            println!("{}", mirror::actor::status::format_status(&actors));
        }
        other => {
            eprintln!("conversation actor: unknown subcommand '{other}'");
            eprintln!("available: observe, init, mount, unmount, status");
            process::exit(1);
        }
    }
}

fn actor_observe(repo_path: &std::path::Path) {
    use mirror::actor::emit_nix;
    use mirror::actor::observe;

    eprintln!("observing: {}", repo_path.display());

    let deps: Vec<observe::ResolvedDep> = match observe::scan_repo(repo_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("conversation actor observe: {e}");
            process::exit(1);
        }
    };

    let packages: Vec<_> = deps.iter().filter(|d| d.is_package).collect();
    let core: Vec<_> = deps.iter().filter(|d| !d.is_package).collect();

    eprintln!(
        "  core domains: {}",
        core.iter()
            .map(|d| d.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
    eprintln!(
        "  packages: {}",
        packages
            .iter()
            .map(|d| d.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );

    let repo_name = repo_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    let flake = emit_nix::emit_flake(repo_name, &deps);

    let flake_path = repo_path.join("flake.nix");
    match std::fs::write(&flake_path, &flake) {
        Ok(()) => eprintln!("  wrote: {}", flake_path.display()),
        Err(e) => {
            eprintln!("conversation actor observe: write flake.nix: {e}");
            process::exit(1);
        }
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
                if let Ok(domain) = Domain::from_grammar(child) {
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
