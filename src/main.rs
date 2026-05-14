// FROZEN -- see AGENTS.md. Do not modify without explicit approval.
// This file is Rust substrate. All extensions happen through .mirror grammars.
// If you're adding code here, you're probably wrong. Write a grammar instead.

//! mirror -- the compiler entry point.
//!
//! Opens the socket. Reads args. Dispatches through the interpreter.
//! Every command is a grammar ref: `@mirror/<command>`.
//! Everything else is grammar. This is @io.

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: mirror <command> [flags...] [args...]");
        eprintln!("commands: compile <file>, craft <target>, kintsugi <file>, bench <path>");
        eprintln!();
        eprintln!("flags are grammar references:");
        eprintln!("  --strict      = @cli/strict      (nullary)");
        eprintln!("  --format json = @cli/format       (unary)");
        eprintln!("  --git/commit  = @git/commit       (namespaced)");
        std::process::exit(1);
    }

    mirror::interpreter::dispatch(&args[1], &args[2..]);
}
