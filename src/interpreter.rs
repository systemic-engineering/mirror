//! The mirror interpreter — one Rust function, grammars all the way down.
//!
//! Contract:
//! - `io_exec` is ONE function. The only door to reality.
//! - The prism executor: five operations on MirrorAST.
//! - Grammar dispatch: CLI commands are grammar refs (`@mirror/<command>`).
//! - Git storage: `@git/crystal` via `io_exec`. No separate cache module.
//!
//! Everything else is grammar. `io_exec` is `@io`. This is the socket.

use crate::kernel::Oid;
use crate::mirror_ast::MirrorAST;

// ---------------------------------------------------------------------------
// @io — one function. The only door to reality.
// ---------------------------------------------------------------------------

/// Execute an external command. This is ALL the Rust that touches the outside world.
///
/// `io_exec("git", &["hash-object", "-w", "--stdin"], Some(blob))` stores a git blob.
/// `io_exec("echo", &["hello"], None)` prints hello.
///
/// One function. The socket. The door. Everything above is grammar.
pub fn io_exec(command: &str, args: &[&str], stdin: Option<&[u8]>) -> Vec<u8> {
    let mut cmd = std::process::Command::new(command);
    cmd.args(args);
    if let Some(input) = stdin {
        use std::io::Write;
        cmd.stdin(std::process::Stdio::piped());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::null());
        let mut child = cmd.spawn().expect("io_exec: spawn");
        child.stdin.take().unwrap().write_all(input).unwrap();
        child.wait_with_output().unwrap().stdout
    } else {
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::null());
        cmd.output().expect("io_exec: output").stdout
    }
}

// ---------------------------------------------------------------------------
// Prism executor — five operations on MirrorAST
// ---------------------------------------------------------------------------

/// `focus` — look closer. Descend into a named child.
pub fn focus<'a>(ast: &'a MirrorAST, name: &str) -> Option<&'a MirrorAST> {
    let children = match ast {
        MirrorAST::Focus(f) => &f.children,
        MirrorAST::Module(m) => &m.children,
        MirrorAST::Split(s) => &s.children,
        MirrorAST::Zoom(z) => &z.children,
        MirrorAST::Refract(r) => &r.children,
        MirrorAST::Project(p) => &p.children,
        MirrorAST::Abstract { inner, .. } => return focus(inner, name),
    };
    children.iter().find(|child| child.name() == name)
}

/// `project` — extract a view. Filter children matching a predicate.
pub fn project(ast: &MirrorAST, predicate: fn(&MirrorAST) -> bool) -> Vec<&MirrorAST> {
    let children = match ast {
        MirrorAST::Focus(f) => &f.children,
        MirrorAST::Module(m) => &m.children,
        MirrorAST::Split(s) => &s.children,
        MirrorAST::Zoom(z) => &z.children,
        MirrorAST::Refract(r) => &r.children,
        MirrorAST::Project(p) => &p.children,
        MirrorAST::Abstract { inner, .. } => return project(inner, predicate),
    };
    children.iter().filter(|child| predicate(child)).collect()
}

/// `split` — enumerate. List all children.
pub fn split(ast: &MirrorAST) -> &[MirrorAST] {
    match ast {
        MirrorAST::Focus(f) => &f.children,
        MirrorAST::Module(m) => &m.children,
        MirrorAST::Split(s) => &s.children,
        MirrorAST::Zoom(z) => &z.children,
        MirrorAST::Refract(r) => &r.children,
        MirrorAST::Project(p) => &p.children,
        MirrorAST::Abstract { inner, .. } => split(inner),
    }
}

/// `zoom` — transform. Apply a function to each child, return new Module.
pub fn zoom(ast: &MirrorAST, transform: fn(&MirrorAST) -> MirrorAST) -> MirrorAST {
    let children = split(ast);
    let transformed: Vec<MirrorAST> = children.iter().map(|c| transform(c)).collect();
    MirrorAST::Module(crate::mirror_ast::ModuleNode {
        name: crate::mirror_ast::Identifier::new(ast.name()),
        children: transformed,
    })
}

/// `refract` — settle. Compute OID (content-address).
pub fn refract(ast: &MirrorAST) -> Oid {
    ast.content_oid()
}

// ---------------------------------------------------------------------------
// @git/crystal — compilation cache backed by git, via io_exec
// ---------------------------------------------------------------------------

/// Store a blob in git via `io_exec`. Returns the git blob OID.
pub fn git_store(content: &str) -> String {
    let out = io_exec("git", &["hash-object", "-w", "--stdin"], Some(content.as_bytes()));
    String::from_utf8(out).unwrap_or_default().trim().to_string()
}

/// Check if a git object exists. Returns true if the object is in the store.
pub fn git_exists(oid: &str) -> bool {
    let out = io_exec("git", &["cat-file", "-t", oid], None);
    !out.is_empty()
}

/// Read a git object's content via `io_exec`.
pub fn git_lookup(ref_name: &str) -> Option<String> {
    let out = io_exec("git", &["cat-file", "-p", ref_name], None);
    if out.is_empty() {
        None
    } else {
        Some(String::from_utf8(out).unwrap_or_default().trim().to_string())
    }
}

/// Store a crystal OID in git as a ref, via `io_exec`.
///
/// Creates a blob containing the crystal OID string, then points
/// `refs/crystals/<source_hash>` at that blob.
pub fn git_store_crystal(source_hash: &str, crystal_oid: &str) -> Option<String> {
    let blob = git_store(crystal_oid);
    if blob.is_empty() {
        return None;
    }
    let ref_name = format!("refs/crystals/{}", source_hash);
    let out = io_exec("git", &["update-ref", &ref_name, &blob], None);
    // update-ref produces no output on success; we check by trying to read it back
    let _ = out;
    Some(blob)
}

/// Check if a crystal exists for this source hash. Returns the crystal OID if cached.
pub fn git_crystal_exists(source_hash: &str) -> Option<String> {
    let ref_name = format!("refs/crystals/{}", source_hash);
    git_lookup(&ref_name)
}

/// Delete a crystal ref (for cleanup in tests).
pub fn git_delete_crystal(source_hash: &str) {
    let ref_name = format!("refs/crystals/{}", source_hash);
    let _ = io_exec("git", &["update-ref", "-d", &ref_name], None);
}

/// Hash source content to produce the cache key.
pub fn source_hash(source: &str) -> String {
    Oid::hash(source.as_bytes()).as_ref().to_string()
}

/// Compile with git crystal cache: hash source, check git refs, tokenize only on miss.
pub fn compile_cached(
    source: &str,
    grammar: &crate::tokenize::Grammar,
) -> (Oid, bool) {
    let hash = source_hash(source);

    // Check git crystal cache
    if let Some(cached_oid) = git_crystal_exists(&hash) {
        return (Oid::new(cached_oid), true);
    }

    // Miss: tokenize
    let ast = crate::tokenize::tokenize(source, grammar);
    let oid = ast.content_oid();

    // Store in git (best-effort)
    let _ = git_store_crystal(&hash, oid.as_ref());

    (oid, false)
}

// ---------------------------------------------------------------------------
// Grammar dispatch — CLI commands are grammar refs (@mirror/<command>)
// ---------------------------------------------------------------------------

/// Dispatch a CLI command through the interpreter.
///
/// Each command maps to a grammar ref: `@mirror/<command>`.
/// For now, this is a scaffold that calls existing functions.
/// When grammar execution lands, this match disappears — the grammar does the dispatch.
///
/// TODO(grammar): replace this match with grammar ref resolution via @fate.
pub fn dispatch(command: &str, args: &[String]) {
    let cmd = crate::cli::parse_command(
        &std::iter::once(command.to_string())
            .chain(args.iter().cloned())
            .collect::<Vec<_>>(),
    );

    // Print the flag pipeline if flags are present
    if !cmd.flags.is_empty() {
        let pipeline = crate::cli::format_pipeline(&cmd.flags);
        eprintln!("pipeline: {}", pipeline);
    }

    // Temporary scaffold: match on known commands.
    // Grammar ref: @mirror/<command>
    // When grammars execute, this entire match becomes:
    //   let grammar = load_grammar(format!("boot/std/mirror/{}.mirror", command));
    //   execute(grammar, args);
    match cmd.name.as_str() {
        "compile" => dispatch_compile(&cmd),
        "craft" => dispatch_craft(&cmd),
        "kintsugi" => dispatch_kintsugi(&cmd),
        "bench" => dispatch_bench(&cmd),
        _ => {
            eprintln!("unknown command: {} (grammar ref: @mirror/{})", cmd.name, cmd.name);
            std::process::exit(1);
        }
    }
}

fn dispatch_compile(cmd: &crate::cli::ParsedCommand) {
    if cmd.positional.is_empty() {
        eprintln!("usage: mirror compile [flags] <file>");
        std::process::exit(1);
    }
    let file = &cmd.positional[0];

    let source = std::fs::read_to_string(file).unwrap_or_else(|e| {
        eprintln!("cannot read file {}: {}", file, e);
        std::process::exit(1);
    });

    let grammar_path = crate::tokenize::grammar_for_file(file);
    let grammar = crate::tokenize::load_grammar(grammar_path).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let no_cache = cmd.flags.iter().any(|f| f.grammar_ref == "@cli/no-cache");

    if no_cache {
        let ast = crate::tokenize::tokenize(&source, &grammar);
        let oid = ast.content_oid();
        println!("{}", oid);
    } else {
        let (oid, cached) = compile_cached(&source, &grammar);
        if cached {
            eprintln!("(cached)");
        }
        println!("{}", oid);
    }
}

fn dispatch_craft(cmd: &crate::cli::ParsedCommand) {
    if cmd.positional.is_empty() {
        eprintln!("usage: mirror craft [flags] <target>");
        eprintln!("targets: boot, cargo, std");
        std::process::exit(1);
    }
    let target = &cmd.positional[0];
    let no_cache = cmd.flags.iter().any(|f| f.grammar_ref == "@cli/no-cache");

    if no_cache {
        let crystal = crate::tokenize::craft_target(target);
        println!("{}", crystal);
    } else {
        let (crystal, hits, total) = crate::tokenize::craft_target_cached(target, true);
        if hits > 0 {
            eprintln!("cache: {}/{} hits", hits, total);
        }
        println!("{}", crystal);
    }
}

fn dispatch_kintsugi(cmd: &crate::cli::ParsedCommand) {
    if cmd.positional.is_empty() {
        eprintln!("usage: mirror kintsugi [flags] <file>");
        std::process::exit(1);
    }
    let file = &cmd.positional[0];

    let source = std::fs::read_to_string(file).unwrap_or_else(|e| {
        eprintln!("cannot read file {}: {}", file, e);
        std::process::exit(1);
    });

    let grammar_path = crate::tokenize::grammar_for_file(file);
    let grammar = crate::tokenize::load_grammar(grammar_path).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let ast = crate::tokenize::tokenize(&source, &grammar);
    let output = crate::tokenize::canonical_form(&ast);
    print!("{}", output);
}

fn dispatch_bench(cmd: &crate::cli::ParsedCommand) {
    let is_cascade = cmd.flags.iter().any(|f| f.grammar_ref == "@cli/cascade");
    let is_compare = cmd.flags.iter().any(|f| f.grammar_ref == "@cli/compare");

    if is_cascade {
        let dir = if cmd.positional.is_empty() { "boot/" } else { &cmd.positional[0] };
        let result = crate::bench::cascade(dir);
        print!("{}", crate::bench::format_cascade(&result));
        return;
    }

    if is_compare && cmd.positional.len() >= 2 {
        let a = &cmd.positional[0];
        let b = &cmd.positional[1];

        let result_a = if is_dir(a) {
            crate::bench::bench_dir(a)
        } else {
            crate::bench::BenchSuite {
                results: vec![crate::bench::bench_file(a)],
                total_time_ns: crate::bench::bench_file(a).time_ns,
            }
        };
        let result_b = if is_dir(b) {
            crate::bench::bench_dir(b)
        } else {
            crate::bench::BenchSuite {
                results: vec![crate::bench::bench_file(b)],
                total_time_ns: crate::bench::bench_file(b).time_ns,
            }
        };

        println!("--- {} ---", a);
        print!("{}", crate::bench::format_suite(&result_a));
        println!("--- {} ---", b);
        print!("{}", crate::bench::format_suite(&result_b));

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
        std::process::exit(1);
    }

    let path = &cmd.positional[0];

    if is_dir(path) {
        let suite = crate::bench::bench_dir(path);
        print!("{}", crate::bench::format_suite(&suite));
    } else {
        let result = crate::bench::bench_file(path);
        println!("{}", crate::bench::format_result(&result));
    }
}

fn is_dir(path: &str) -> bool {
    std::fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mirror_ast::*;

    // -- @io tests --

    #[test]
    fn io_exec_echo() {
        let out = io_exec("echo", &["hello"], None);
        assert_eq!(String::from_utf8(out).unwrap().trim(), "hello");
    }

    #[test]
    fn io_exec_with_stdin() {
        let out = io_exec("cat", &[], Some(b"test input"));
        assert_eq!(String::from_utf8(out).unwrap(), "test input");
    }

    // -- Prism executor tests --

    #[test]
    fn focus_finds_named_child() {
        let child = MirrorAST::Split(SplitNode {
            name: Identifier::new("color"),
            variants: vec![],
            params: vec![],
            body: None,
            children: vec![],
        });
        let parent = MirrorAST::Module(ModuleNode {
            name: Identifier::new("root"),
            children: vec![child],
        });
        let found = focus(&parent, "color");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name(), "color");
    }

    #[test]
    fn focus_returns_none_for_missing() {
        let parent = MirrorAST::Module(ModuleNode {
            name: Identifier::new("root"),
            children: vec![],
        });
        assert!(focus(&parent, "missing").is_none());
    }

    #[test]
    fn project_filters_children() {
        let zoom_child = MirrorAST::Zoom(ZoomNode {
            name: Identifier::new("action1"),
            target: None,
            params: vec![],
            grammar_ref: None,
            children: vec![],
            body: None,
        });
        let split_child = MirrorAST::Split(SplitNode {
            name: Identifier::new("type1"),
            variants: vec![],
            params: vec![],
            body: None,
            children: vec![],
        });
        let parent = MirrorAST::Module(ModuleNode {
            name: Identifier::new("root"),
            children: vec![zoom_child, split_child],
        });
        let zooms = project(&parent, |node| matches!(node, MirrorAST::Zoom(_)));
        assert_eq!(zooms.len(), 1);
        assert_eq!(zooms[0].name(), "action1");
    }

    #[test]
    fn split_enumerates_children() {
        let child1 = MirrorAST::Split(SplitNode {
            name: Identifier::new("a"),
            variants: vec![],
            params: vec![],
            body: None,
            children: vec![],
        });
        let child2 = MirrorAST::Split(SplitNode {
            name: Identifier::new("b"),
            variants: vec![],
            params: vec![],
            body: None,
            children: vec![],
        });
        let parent = MirrorAST::Module(ModuleNode {
            name: Identifier::new("root"),
            children: vec![child1, child2],
        });
        assert_eq!(split(&parent).len(), 2);
    }

    #[test]
    fn zoom_transforms_children() {
        let child = MirrorAST::Split(SplitNode {
            name: Identifier::new("original"),
            variants: vec![],
            params: vec![],
            body: None,
            children: vec![],
        });
        let parent = MirrorAST::Module(ModuleNode {
            name: Identifier::new("root"),
            children: vec![child],
        });
        let result = zoom(&parent, |node| {
            MirrorAST::Split(SplitNode {
                name: Identifier::new(&format!("{}_transformed", node.name())),
                variants: vec![],
                params: vec![],
                body: None,
                children: vec![],
            })
        });
        let children = split(&result);
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name(), "original_transformed");
    }

    #[test]
    fn refract_produces_oid() {
        let ast = MirrorAST::Split(SplitNode {
            name: Identifier::new("test"),
            variants: vec![],
            params: vec![],
            body: None,
            children: vec![],
        });
        let oid = refract(&ast);
        assert!(!oid.as_ref().is_empty());
    }

    #[test]
    fn refract_is_deterministic() {
        let ast = MirrorAST::Split(SplitNode {
            name: Identifier::new("test"),
            variants: vec![],
            params: vec![],
            body: None,
            children: vec![],
        });
        assert_eq!(refract(&ast), refract(&ast));
    }

    // -- @git/crystal tests --

    #[test]
    fn git_store_and_lookup() {
        let oid = git_store("test crystal content from interpreter");
        assert!(!oid.is_empty(), "git_store must return an OID");
        assert!(git_exists(&oid), "stored object must exist");
    }

    #[test]
    fn git_crystal_roundtrip() {
        let test_hash = "interpreter_test_roundtrip_001";
        let crystal_oid = "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";

        let blob = git_store_crystal(test_hash, crystal_oid);
        assert!(blob.is_some(), "git_store_crystal must succeed");

        let cached = git_crystal_exists(test_hash);
        assert_eq!(cached, Some(crystal_oid.to_string()));

        git_delete_crystal(test_hash);

        let after = git_crystal_exists(test_hash);
        assert_eq!(after, None, "crystal must be gone after delete");
    }

    #[test]
    fn dispatch_compile() {
        // mirror compile --no-cache boot/std/kintsugi.mirror should produce an OID
        // --no-cache is nullary so boot/std/kintsugi.mirror stays positional
        // no panic = success
        dispatch("compile", &["boot/std/kintsugi.mirror".to_string(), "--no-cache".to_string()]);
    }
}
