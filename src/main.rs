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
use conversation::packages::PackageRegistry;
use conversation::resolve::{Conversation, Resolve};
use conversation::Vector;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: conversation <file.conv> [path]");
        eprintln!("       conversation -e '<expr>' [path]");
        eprintln!("       conversation shell [path]");
        process::exit(1);
    }

    match args[1].as_str() {
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
