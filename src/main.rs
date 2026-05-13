//! mirror — the compiler entry point.
//!
//! Opens the socket. Reads args. Dispatches through grammar.
//! Everything else is grammar. This is @io.

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: mirror <command> [args...]");
        eprintln!("commands: compile <file>");
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "compile" => {
            if args.len() < 3 {
                eprintln!("usage: mirror compile <file>");
                process::exit(1);
            }
            let file = &args[2];

            let source = std::fs::read_to_string(file).unwrap_or_else(|e| {
                eprintln!("cannot read file {}: {}", file, e);
                process::exit(1);
            });

            // Detect grammar from file extension
            let grammar_path = if file.ends_with(".rs") {
                "boot/std/code/rust.mirror"
            } else if file.ends_with(".mirror") || file.ends_with(".spec") || file.ends_with(".shatter") {
                "boot/std/mirror/grammar.mirror"
            } else {
                "boot/std/code/rust.mirror" // default for now
            };

            let grammar = mirror::tokenize::load_grammar(grammar_path).unwrap_or_else(|e| {
                eprintln!("{}", e);
                process::exit(1);
            });

            let ast = mirror::tokenize::tokenize(&source, &grammar);
            let oid = ast.content_oid();
            println!("{}", oid);
        }
        _ => {
            eprintln!("unknown command: {}", command);
            process::exit(1);
        }
    }
}
