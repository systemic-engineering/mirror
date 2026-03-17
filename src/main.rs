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
//! # Test mode — run package tests:
//! conversation test glue.conv [path]
//!
//! # Shebang:
//! #!/usr/bin/env conversation
//! ```

use std::io;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: conversation <file.conv> [path]");
        eprintln!("       conversation test <file.conv> [path]");
        eprintln!("       conversation -e '<expr>' [path]");
        eprintln!("       conversation shell [path]");
        process::exit(1);
    }

    match args[1].as_str() {
        "test" => {
            if args.len() < 3 {
                eprintln!("usage: conversation test <file.conv> [path]");
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
            let path = args.get(3).map(|s| s.as_str()).unwrap_or(".");
            process::exit(conversation::test::run_file(&source, path, conv_path));
        }
        "shell" => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            eprintln!("conversation shell — {}", path);
            eprintln!("type expressions, ctrl+d to exit\n");
            let stdin = io::stdin();
            conversation::shell::repl(
                path,
                &mut stdin.lock(),
                &mut io::stdout(),
                &mut io::stderr(),
            );
        }
        "-e" => {
            if args.len() < 3 {
                eprintln!("usage: conversation -e '<expr>' [path]");
                process::exit(1);
            }
            let source = format!("out {}\n", &args[2]);
            let path = args.get(3).map(|s| s.as_str()).unwrap_or(".");
            if let Err(msg) = conversation::shell::eval(&source, path, &mut io::stdout()) {
                eprintln!("{}", msg);
                process::exit(1);
            }
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
            if let Err(msg) = conversation::shell::eval(&source, path, &mut io::stdout()) {
                eprintln!("{}", msg);
                process::exit(1);
            }
        }
    }
}
