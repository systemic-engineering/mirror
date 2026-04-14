//! mirror -- an honest compiler
//!
//! The CLI surface for the mirror runtime.

use std::process;

use mirror::cli::Cli;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("{}", Cli::help_text());
        process::exit(1);
    }

    // Per-command --help: `mirror compile --help`
    if args.len() >= 3 && (args[2] == "--help" || args[2] == "-h") {
        if let Some(help) = Cli::command_help(&args[1]) {
            println!("{}", help);
            process::exit(0);
        }
    }

    let cli = Cli::open("spec.mirror").unwrap_or_default();

    use prism::Imperfect;
    match cli.dispatch(&args[1], &args[2..]) {
        Imperfect::Success(output) => {
            println!("{}", output);
            process::exit(0);
        }
        Imperfect::Partial(output, _loss) => {
            println!("{}", output);
            // Partial is still ok — value present, some loss measured
            process::exit(0);
        }
        Imperfect::Failure(e, _loss) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}
