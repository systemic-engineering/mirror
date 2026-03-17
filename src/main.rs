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

    let resolve = load_packages();

    match args[1].as_str() {
        "shell" => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            shell(path, &resolve);
        }
        "-e" => {
            if args.len() < 3 {
                eprintln!("usage: conversation -e '<expr>' [path]");
                process::exit(1);
            }
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
            let path = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            run(&source, path, &resolve);
        }
    }
}

/// Discover packages from CONVERSATION_PACKAGES or ~/.conversation.
fn load_packages() -> Resolve {
    let packages_dir = PackageRegistry::packages_dir();
    if !packages_dir.exists() {
        return Resolve::new();
    }
    let registry = match PackageRegistry::discover(&packages_dir) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("conversation: packages: {}", e);
            return Resolve::new();
        }
    };
    match registry.to_namespace() {
        Ok(namespace) => Resolve::new().with_namespace(namespace),
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
