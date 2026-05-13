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
        eprintln!("commands: compile <file>, craft <target>, kintsugi <file>");
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "compile" => cmd_compile(&args[2..]),
        "craft" => cmd_craft(&args[2..]),
        "kintsugi" => cmd_kintsugi(&args[2..]),
        _ => {
            eprintln!("unknown command: {}", command);
            process::exit(1);
        }
    }
}

fn cmd_compile(args: &[String]) {
    if args.is_empty() {
        eprintln!("usage: mirror compile <file>");
        process::exit(1);
    }
    let file = &args[0];

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

fn cmd_craft(args: &[String]) {
    if args.is_empty() {
        eprintln!("usage: mirror craft <target>");
        eprintln!("targets: boot, cargo, std");
        process::exit(1);
    }
    let target = &args[0];
    let crystal = mirror::tokenize::craft_target(target);
    println!("{}", crystal);
}

fn cmd_kintsugi(args: &[String]) {
    if args.is_empty() {
        eprintln!("usage: mirror kintsugi <file>");
        process::exit(1);
    }
    let file = &args[0];

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
