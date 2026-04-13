//! mirror — focus | project | refract
//!
//! The CLI surface for the mirror runtime.

use std::process;

use fragmentation::sha::HashAlg;
use mirror::ast_prism::ASTPrism;
use prism::{Beam, Optic, Prism};

const USAGE: &str = "\
mirror — focus | project | refract

usage:
  mirror compile <file>             compile a .mirror file
  mirror crystal [output]           materialize mirror.shatter from boot/
  mirror ai <model> [file|-]        run a fate model
  mirror <query>                    parse and print the AST

models: abyss | introject | cartographer | explorer | fate
";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprint!("{}", USAGE);
        process::exit(1);
    }

    match args[1].as_str() {
        "compile" => cmd_compile(&args[2..]),
        "crystal" => cmd_crystal(&args[2..]),
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

    let source = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: cannot read {}: {}", file, e);
            process::exit(1);
        }
    };

    let mut compiler = mirror::bundle::MirrorCompiler::new();
    match compiler.compile(&source) {
        Ok(compiled) => {
            let shard = mirror::shard::Shard::new(
                compiled.crystal().clone(),
                compiler.kernel_spec.clone(),
                compiler.target,
            );
            println!("{}", shard.grammar_oid.as_str());
            eprintln!(
                "compiled {} → {} (rank {}, {:?})",
                file,
                shard.grammar_oid.as_str(),
                shard.rank(),
                shard.target,
            );
            process::exit(0);
        }
        Err(e) => {
            eprintln!("error: compile {}: {}", file, e);
            process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// crystal — materialize mirror.shatter
// ---------------------------------------------------------------------------

fn cmd_crystal(args: &[String]) -> ! {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let boot_dir = manifest_dir.join("boot");
    let default_output = manifest_dir.join("mirror.shatter");

    let output = match args.first() {
        Some(f) => std::path::PathBuf::from(f),
        None => default_output,
    };

    let store_dir = std::env::temp_dir().join(format!("mirror-crystal-{}", process::id()));
    let _ = std::fs::create_dir_all(&store_dir);

    let runtime = mirror::mirror_runtime::MirrorRuntime::new();
    match runtime.materialize_crystal(&boot_dir, &store_dir, &output) {
        Ok(oid) => {
            println!("{}", oid.as_str());
            eprintln!("crystal {} → {}", oid.as_str(), output.display(),);
            let _ = std::fs::remove_dir_all(&store_dir);
            process::exit(0);
        }
        Err(e) => {
            eprintln!("error: materialize crystal: {}", e);
            let _ = std::fs::remove_dir_all(&store_dir);
            process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// ai
// ---------------------------------------------------------------------------

fn cmd_ai(args: &[String]) -> ! {
    let model = match args.first().map(|s| s.as_str()) {
        Some("abyss") => "abyss",
        Some("introject") => "introject",
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
    let seed = Optic::ok((), query.clone());
    let focused = prism.focus(seed);
    let projected = prism.project(focused);

    // Print the AST in mirror format.
    println!("{}", projected.result().ok().expect("parse failed"));
    process::exit(0)
}
