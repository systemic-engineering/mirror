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
        eprintln!("commands: compile <file>, craft <target>, kintsugi <file>, bench <path>");
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
        "bench" => cmd_bench(&cmd),
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

fn cmd_bench(cmd: &mirror::cli::ParsedCommand) {
    // Check for --cascade flag
    let is_cascade = cmd.flags.iter().any(|f| f.grammar_ref == "@cli/cascade");
    // Check for --compare flag
    let is_compare = cmd.flags.iter().any(|f| f.grammar_ref == "@cli/compare");

    if is_cascade {
        // mirror bench --cascade <dir>
        let dir = if cmd.positional.is_empty() { "boot/" } else { &cmd.positional[0] };
        let result = mirror::bench::cascade(dir);
        print!("{}", mirror::bench::format_cascade(&result));
        return;
    }

    if is_compare && cmd.positional.len() >= 2 {
        // mirror bench --compare <a> <b>
        let a = &cmd.positional[0];
        let b = &cmd.positional[1];

        let result_a = if is_dir(a) {
            mirror::bench::bench_dir(a)
        } else {
            mirror::bench::BenchSuite {
                results: vec![mirror::bench::bench_file(a)],
                total_time_ns: mirror::bench::bench_file(a).time_ns,
            }
        };
        let result_b = if is_dir(b) {
            mirror::bench::bench_dir(b)
        } else {
            mirror::bench::BenchSuite {
                results: vec![mirror::bench::bench_file(b)],
                total_time_ns: mirror::bench::bench_file(b).time_ns,
            }
        };

        println!("--- {} ---", a);
        print!("{}", mirror::bench::format_suite(&result_a));
        println!("--- {} ---", b);
        print!("{}", mirror::bench::format_suite(&result_b));

        let ratio = result_b.total_time_ns as f64 / result_a.total_time_ns as f64;
        println!("speedup: {:.2}x ({} vs {})", ratio, a, b);
        return;
    }

    if cmd.positional.is_empty() {
        eprintln!("usage: mirror bench [flags] <path>");
        eprintln!("       mirror bench boot/std/kintsugi.mirror");
        eprintln!("       mirror bench boot/");
        eprintln!("       mirror bench --cascade boot/");
        eprintln!("       mirror bench --compare boot/ src/");
        process::exit(1);
    }

    let path = &cmd.positional[0];

    if is_dir(path) {
        // Bench all files in directory
        let suite = mirror::bench::bench_dir(path);
        print!("{}", mirror::bench::format_suite(&suite));
    } else {
        // Bench single file
        let result = mirror::bench::bench_file(path);
        println!("{}", mirror::bench::format_result(&result));
    }
}

fn is_dir(path: &str) -> bool {
    std::fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false)
}
