//! mirror — focus | project | refract
//!
//! The CLI surface for the mirror runtime.

use std::process;

use mirror::cli::Cli;

const USAGE: &str = "\
mirror — focus | project | refract

usage:
  mirror compile <file>             compile a .mirror file
  mirror crystal [output]           materialize mirror.shatter from boot/
  mirror crystal --oid              print the loaded crystal OID
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

    let cli = Cli::open("spec.mirror").unwrap_or_default();

    match cli.dispatch(&args[1], &args[2..]) {
        Ok(output) => {
            println!("{}", output);
            process::exit(0);
        }
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}
