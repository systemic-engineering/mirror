//! conversation — stories over trees.
//!
//! A unix tool. Reads a .conv spec, reads a domain tree, writes JSON.
//!
//! ```sh
//! # File mode (shebang-compatible):
//! conversation systemic.engineering.conv ./blog
//!
//! # Interactive shell (IEx-style):
//! conversation shell ./blog
//!
//! # Shebang:
//! #!/usr/bin/env conversation
//! ```

use std::io::{self, BufRead, Write};
use std::process;

use conversation::domain::filesystem::{Filesystem, Folder};
use conversation::model::Domain;
use conversation::packages::{self, PackageRegistry};
use conversation::property;
use conversation::resolve::{Conversation, Resolve};
use conversation::{Parse, Vector};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: conversation <file.conv> [path]");
        eprintln!("       conversation -e '<expr>' [path]");
        eprintln!("       conversation shell [path]");
        process::exit(1);
    }

    match args[1].as_str() {
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
            conversation::actor::run(&args[2..].to_vec());
        }
        #[cfg(feature = "db")]
        "db" => {
            let db_args: Vec<String> = args[2..].to_vec();
            conversation::db::cli(&db_args);
        }
        #[cfg(feature = "lsp")]
        "lsp" => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(conversation::lsp::run());
        }
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
