//! mirror — the native binary, Rust port.
//!
//! Bit-exact CoincidenceHash<3> + content_oid compatibility with the C
//! original at native/mirror.c. The body-capture fix for LLVM IR keyword
//! forms (target datalayout = "...", source_filename = "...") is shared.

mod ast;
mod content;
mod exec;
mod git;
mod grammar;
mod hash;
mod pipeline;
mod render;
mod tokenize;

use std::fs;
use std::io::{self, Read, Write};

use crate::content::content_oid;
use crate::git::{git_crystal_exists, git_store_crystal};
use crate::grammar::{grammar_for_file, load_grammar};
use crate::hash::canonical_hash;
use crate::pipeline::{execute_pipeline, is_mq_query, split_pipeline};
use crate::render::render_ast;
use crate::tokenize::tokenize;

fn usage() {
    eprintln!("usage:");
    eprintln!("  mirror <command> [args...]            (legacy subcommand surface)");
    eprintln!("  mirror '<mq-query>' < input           (mq pipeline over stdin)");
    eprintln!("  mirror <input> '<mq-query>'           (mq pipeline over input file)");
    eprintln!("commands: compile <file>, craft <target>, kintsugi <file>");
    eprintln!("examples:");
    eprintln!("  cat mirror.ll | mirror '@code/llvm/ir |> @mirror/kintsugi |> @mirror/butterfly'");
}

fn read_stdin_all() -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;
    Ok(buf)
}

fn cmd_compile(file: &str, no_cache: bool) -> i32 {
    let source = match fs::read(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cannot read file {}: {}", file, e);
            return 1;
        }
    };
    let grammar_path = grammar_for_file(file);
    let grammar = match load_grammar(grammar_path) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("cannot read grammar {}: {}", grammar_path, e);
            return 1;
        }
    };

    if !no_cache {
        let source_oid = canonical_hash(&source);
        if let Some(cached) = git_crystal_exists(&source_oid) {
            eprintln!("(cached)");
            println!("{}", cached);
            return 0;
        }
    }

    let ast = tokenize(&source, &grammar);
    let oid = content_oid(&ast);
    if !no_cache {
        let source_oid = canonical_hash(&source);
        git_store_crystal(&source_oid, &oid);
    }
    println!("{}", oid);
    0
}

fn collect_files(dir: &str, ext: &str, out: &mut Vec<String>) {
    let read_dir = match fs::read_dir(dir) {
        Ok(r) => r,
        Err(_) => return,
    };
    for entry in read_dir.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        let path = format!("{}/{}", dir, name);
        let md = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if md.is_dir() {
            collect_files(&path, ext, out);
        } else if name.ends_with(ext) {
            out.push(path);
        }
    }
}

fn cmd_craft(target: &str, no_cache: bool) -> i32 {
    let mut files: Vec<String> = Vec::new();
    match target {
        "boot" | "std" => collect_files("boot", ".mirror", &mut files),
        "cargo" => collect_files("src", ".rs", &mut files),
        _ => {
            eprintln!("unknown target: {}", target);
            return 1;
        }
    }
    files.sort();

    let mut hasher_buf: Vec<u8> = Vec::new();
    let mut hits = 0;
    let total = files.len();

    for file in &files {
        let grammar_path = grammar_for_file(file);
        let grammar = match load_grammar(grammar_path) {
            Ok(g) => g,
            Err(_) => {
                eprintln!("  skip {} (grammar error)", file);
                continue;
            }
        };
        let source = match fs::read(file) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("  skip {} (read error)", file);
                continue;
            }
        };
        let mut oid = String::new();
        let mut cached = false;
        if !no_cache {
            let source_oid = canonical_hash(&source);
            if let Some(c) = git_crystal_exists(&source_oid) {
                oid = c;
                cached = true;
                hits += 1;
            }
        }
        if !cached {
            let ast = tokenize(&source, &grammar);
            oid = content_oid(&ast);
            if !no_cache {
                let source_oid = canonical_hash(&source);
                git_store_crystal(&source_oid, &oid);
            }
        }
        if cached {
            eprintln!("  {} -> {} (cached)", file, oid);
        } else {
            eprintln!("  {} -> {}", file, oid);
        }
        hasher_buf.extend_from_slice(oid.as_bytes());
    }

    let crystal = canonical_hash(&hasher_buf);
    if hits > 0 {
        eprintln!("cache: {}/{} hits", hits, total);
    }
    println!("{}", crystal);
    0
}

#[allow(dead_code)]
fn dump_ast(node: &crate::ast::AstNode, depth: usize) {
    let indent = "  ".repeat(depth);
    let kind_str = format!("{:?}", node.kind);
    let body_marker = node.body.as_deref().map(|b| format!(" body={:?}", b)).unwrap_or_default();
    let kw_marker = if node.keyword.is_empty() { String::new() } else { format!(" kw={}", node.keyword) };
    let tag_marker = if node.grammar_tag.is_empty() { String::new() } else { format!(" tag={}", node.grammar_tag) };
    eprintln!("{}{} name={:?}{}{}{} oid={}",
        indent, kind_str, node.name, kw_marker, tag_marker, body_marker,
        crate::content::content_oid(node));
    for c in &node.children {
        dump_ast(c, depth + 1);
    }
}

#[allow(dead_code)]
fn cmd_dump(file: &str) -> i32 {
    let source = match fs::read(file) {
        Ok(s) => s,
        Err(e) => { eprintln!("cannot read {}: {}", file, e); return 1; }
    };
    let grammar_path = grammar_for_file(file);
    let grammar = match load_grammar(grammar_path) {
        Ok(g) => g,
        Err(_) => return 1,
    };
    let ast = tokenize(&source, &grammar);
    dump_ast(&ast, 0);
    0
}

fn cmd_kintsugi(file: &str) -> i32 {
    let source = match fs::read(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cannot read file {}: {}", file, e);
            return 1;
        }
    };
    let grammar_path = grammar_for_file(file);
    let grammar = match load_grammar(grammar_path) {
        Ok(g) => g,
        Err(_) => return 1,
    };
    let ast = tokenize(&source, &grammar);
    let mut out = Vec::new();
    render_ast(&ast, 0, &mut out);
    let _ = io::stdout().write_all(&out);
    0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage();
        std::process::exit(1);
    }

    // Path A: mq query as single argument (stdin input)
    if args.len() == 2 && is_mq_query(&args[1]) {
        let segs = split_pipeline(&args[1]);
        if segs.is_empty() {
            eprintln!("empty query");
            std::process::exit(1);
        }
        let source = match read_stdin_all() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("stdin read error: {}", e);
                std::process::exit(1);
            }
        };
        std::process::exit(execute_pipeline(&segs, &source));
    }

    // Path B: file + mq query
    if args.len() == 3 && is_mq_query(&args[2]) {
        let segs = split_pipeline(&args[2]);
        if segs.is_empty() {
            eprintln!("empty query");
            std::process::exit(1);
        }
        let source = match fs::read(&args[1]) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("cannot read {}: {}", args[1], e);
                std::process::exit(1);
            }
        };
        std::process::exit(execute_pipeline(&segs, &source));
    }

    if args.len() < 3 {
        usage();
        std::process::exit(1);
    }

    // Path C: legacy subcommand
    let mut no_cache = false;
    for a in &args[2..] {
        if a == "--no-cache" {
            no_cache = true;
        }
    }
    let positional: Option<&str> = args[2..]
        .iter()
        .find(|a| !a.starts_with("--"))
        .map(|s| s.as_str());

    let rc = match args[1].as_str() {
        "compile" => match positional {
            Some(p) => cmd_compile(p, no_cache),
            None => {
                eprintln!("usage: mirror compile <file>");
                1
            }
        },
        "craft" => match positional {
            Some(p) => cmd_craft(p, no_cache),
            None => {
                eprintln!("usage: mirror craft <target>");
                1
            }
        },
        "kintsugi" => match positional {
            Some(p) => cmd_kintsugi(p),
            None => {
                eprintln!("usage: mirror kintsugi <file>");
                1
            }
        },
        other => {
            eprintln!("unknown: {}", other);
            1
        }
    };
    std::process::exit(rc);
}
