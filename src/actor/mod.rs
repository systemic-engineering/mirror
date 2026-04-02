//! Actor lifecycle: observe, init, spawn, mount, status.
//!
//! `conversation actor observe <home> <repo>` — the first verb.
//! The actor reads a repo's grammars and emits a flake.nix.

pub mod emit_nix;
pub mod observe;

use std::path::Path;

/// Run the `actor` subcommand.
pub fn run(args: &[String]) {
    if args.is_empty() {
        eprintln!("usage: conversation actor <observe|init|spawn|mount|status> [args]");
        std::process::exit(1);
    }

    match args[0].as_str() {
        "observe" => {
            if args.len() < 3 {
                eprintln!("usage: conversation actor observe <actor-home> <repo-path>");
                std::process::exit(1);
            }
            let _actor_home = &args[1];
            let repo_path = Path::new(&args[2]);
            cmd_observe(repo_path);
        }
        other => {
            eprintln!("conversation actor: unknown subcommand '{other}'");
            eprintln!("available: observe");
            std::process::exit(1);
        }
    }
}

fn cmd_observe(repo_path: &Path) {
    eprintln!("observing: {}", repo_path.display());

    let deps = match observe::scan_repo(repo_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("conversation actor observe: {e}");
            std::process::exit(1);
        }
    };

    let packages: Vec<_> = deps.iter().filter(|d| d.is_package).collect();
    let core: Vec<_> = deps.iter().filter(|d| !d.is_package).collect();

    eprintln!(
        "  core domains: {}",
        core.iter()
            .map(|d| d.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
    eprintln!(
        "  packages: {}",
        packages
            .iter()
            .map(|d| d.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );

    let repo_name = repo_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    let flake = emit_nix::emit_flake(repo_name, &deps);

    let flake_path = repo_path.join("flake.nix");
    match std::fs::write(&flake_path, &flake) {
        Ok(()) => eprintln!("  wrote: {}", flake_path.display()),
        Err(e) => {
            eprintln!("conversation actor observe: write flake.nix: {e}");
            std::process::exit(1);
        }
    }
}
