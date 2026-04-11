//! mirror — focus | project | refract
//!
//! The CLI surface for the mirror runtime.

use std::process;

use mirror::ast_prism::ASTPrism;
use prism::{Beam, Prism, PureBeam};

const USAGE: &str = "\
mirror — focus | project | refract

usage:
  mirror compile <file>             compile a .mirror file
  mirror ai <model> [file|-]        run a fate model
  mirror <query>                    parse and print the AST

models: abyss | pathfinder | cartographer | explorer | fate
";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprint!("{}", USAGE);
        process::exit(1);
    }

    match args[1].as_str() {
        "compile" => cmd_compile(&args[2..]),
        "ai" => cmd_ai(&args[2..]),
        _ => cmd_query(&args[1..]),
    }
}

// ---------------------------------------------------------------------------
// compile
// ---------------------------------------------------------------------------

fn cmd_compile(args: &[String]) -> ! {
    let file = match args.first() {
        Some(f) => f,
        None => {
            eprintln!("usage: mirror compile <file>");
            process::exit(1);
        }
    };

    // TODO: wire up MirrorRuntime::compile_file
    eprintln!("compile: {}", file);
    process::exit(0)
}

// ---------------------------------------------------------------------------
// ai
// ---------------------------------------------------------------------------

fn cmd_ai(args: &[String]) -> ! {
    let model = match args.first().map(|s| s.as_str()) {
        Some("abyss") => "abyss",
        Some("pathfinder") => "pathfinder",
        Some("cartographer") => "cartographer",
        Some("explorer") => "explorer",
        Some("fate") => "fate",
        Some(other) => {
            eprintln!("ai fate {}", other);
            process::exit(0)
        }
        None => {
            eprintln!("usage: mirror ai <model> [file|-]");
            process::exit(1);
        }
    };

    let file = args.get(1).map(|s| s.as_str());
    eprintln!("ai {} {}", model, file.unwrap_or("<stdin>"));
    process::exit(0)
}

// ---------------------------------------------------------------------------
// query — the litmus test
// ---------------------------------------------------------------------------

fn cmd_query(args: &[String]) -> ! {
    let query = match args.first() {
        Some(q) => q,
        None => {
            eprint!("{}", USAGE);
            process::exit(1);
        }
    };

    // The parsing pipeline IS the ASTPrism.
    // focus: source → tokens.  project: tokens → AST.
    let prism = ASTPrism;
    let seed = PureBeam::ok((), query.clone());
    let focused = prism.focus(seed);
    let projected = prism.project(focused);

    // Print the AST in mirror format.
    println!("{}", projected.result().ok().expect("parse failed"));
    process::exit(0)
}
