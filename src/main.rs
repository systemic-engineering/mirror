//! mirror — the compiler entry point.
//!
//! Opens the socket. Reads args. Dispatches through grammar.
//! Every `--flag` is a grammar reference: `--strict` = `@cli/strict`.
//! Everything else is grammar. This is @io.

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: mirror <command> [flags...] [args...]");
        eprintln!("commands: compile <file>, craft <target>, kintsugi <file>");
        eprintln!();
        eprintln!("flags are grammar references:");
        eprintln!("  --strict      = @cli/strict      (nullary)");
        eprintln!("  --format json = @cli/format       (unary)");
        eprintln!("  --git/commit  = @git/commit       (namespaced)");
        process::exit(1);
    }

    // Parse the full command: name + flags + positional args
    let cmd = mirror::cli::parse_command(&args[1..]);

    // Print the flag pipeline if flags are present
    if !cmd.flags.is_empty() {
        let pipeline = mirror::cli::format_pipeline(&cmd.flags);
        eprintln!("pipeline: {}", pipeline);
    }

    match cmd.name.as_str() {
        "compile" => cmd_compile(&cmd),
        "craft" => cmd_craft(&cmd),
        "kintsugi" => cmd_kintsugi(&cmd),
        _ => {
            eprintln!("unknown command: {}", cmd.name);
            process::exit(1);
        }
    }
}

fn cmd_compile(cmd: &mirror::cli::ParsedCommand) {
    if cmd.positional.is_empty() {
        eprintln!("usage: mirror compile [flags] <file>");
        process::exit(1);
    }
    let file = &cmd.positional[0];

    let source = std::fs::read_to_string(file).unwrap_or_else(|e| {
        eprintln!("cannot read file {}: {}", file, e);
        process::exit(1);
    });

    let grammar_path = mirror::tokenize::grammar_for_file(file);
    let grammar = mirror::tokenize::load_grammar(grammar_path).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    let ast = mirror::tokenize::tokenize(&source, &grammar);
    let oid = ast.content_oid();
    println!("{}", oid);
}

fn cmd_craft(cmd: &mirror::cli::ParsedCommand) {
    if cmd.positional.is_empty() {
        eprintln!("usage: mirror craft [flags] <target>");
        eprintln!("targets: boot, cargo, std");
        process::exit(1);
    }
    let target = &cmd.positional[0];
    let crystal = mirror::tokenize::craft_target(target);
    println!("{}", crystal);
}

fn cmd_kintsugi(cmd: &mirror::cli::ParsedCommand) {
    if cmd.positional.is_empty() {
        eprintln!("usage: mirror kintsugi [flags] <file>");
        process::exit(1);
    }
    let file = &cmd.positional[0];

    let source = std::fs::read_to_string(file).unwrap_or_else(|e| {
        eprintln!("cannot read file {}: {}", file, e);
        process::exit(1);
    });

    let grammar_path = mirror::tokenize::grammar_for_file(file);
    let grammar = mirror::tokenize::load_grammar(grammar_path).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    let ast = mirror::tokenize::tokenize(&source, &grammar);
    let output = mirror::tokenize::canonical_form(&ast);
    print!("{}", output);
}
