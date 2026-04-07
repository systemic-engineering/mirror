//! mirror — fold | prism | traversal | lens | iso | setter
//!
//! The CLI surface for the mirror runtime. Two modes:
//!
//! - `mirror compile <file>` — parse a `.mirror` file via `MirrorRuntime`,
//!   write its content-addressed `Shatter` artifact to `<file>.shatter`,
//!   print the crystal OID to stdout.
//!
//! - `mirror '<query>' <file> [--compile <target>]` — TBD. The query is
//!   parsed as a Form, the file is parsed as a Form, and a TBD message
//!   names what would happen if the runtime supported form-as-operation
//!   semantics. Exits with status 2.
//!
//! Five spectral dimensions: meets-and-exceeds the 3+1 of the cosmos.

use std::io::{IsTerminal, Read as _};
use std::path::Path;
use std::process;

use fragmentation::sha::HashAlg;
use mirror::mirror_runtime::{emit_form, parse_form, Form, MirrorRuntime};

use coincidence::declaration::DeclKind;

use fate::{Features, Model, FEATURE_DIM};
use fate::runtime::FateRuntime;

// ---------------------------------------------------------------------------
// Usage
// ---------------------------------------------------------------------------

const USAGE: &str = "\
mirror — fold | prism | traversal | lens | iso | setter

usage: mirror compile <file>                          compile to <file>.shatter
       mirror replay <chain.shatter> <input>          re-run a chain against an input
       mirror '<query>' <file> [--compile <target>]   run query against file (TBD)
       mirror ai <subcmd> [file|-]                    fate-driven inference
       mirror fmt <file>                              alias: ai --train --out=<file>

ai subcommands (each reads <file>, or stdin via '-' / pipe; emits a mirror
form on stdout that the next 'mirror ai' invocation can read; the chain
of stages accumulates through pipes via a leading '# chain:' comment):

  mirror ai abyss        <file>   apply abyss        (focus / observe)
  mirror ai pathfinder   <file>   apply pathfinder   (project / cut)
  mirror ai cartographer <file>   apply cartographer (split / map)
  mirror ai explorer     <file>   apply explorer     (zoom / boundary)
  mirror ai fate         <file>   apply fate         (refract / select)
  mirror ai              <file>   alias for: mirror ai fate <file>
  mirror ai '<form>'     <file>   anonymous form invocation (TBD)

flags:
  --out=<file>           write output to <file> instead of stdout
  --capture=<file>       write the accumulated chain to <file>.shatter
                         (the chain is the program; replay reproduces it)
  --train                training pass (TBD; --out is honored)

chain-as-shatter: a .shatter file is a human-readable chain expression
plus input/output content addresses. Replaying the chain against the
same input is bit-for-bit identical to running the live pipeline. We
don't compute. We crystallize.

mirror compiles single .mirror files via MirrorRuntime to spectral
content-addressed shatter artifacts (CoincidenceHash<5>: five spectral
dimensions, meets-and-exceeds the 3+1 of the cosmos).
";

fn print_usage_and_exit() -> ! {
    eprintln!("{}", USAGE);
    process::exit(1);
}

// ---------------------------------------------------------------------------
// .shatter serialization — chain-as-shatter
// ---------------------------------------------------------------------------
//
// On-disk format: human-readable text. Three lines.
//
//     chain: <model> |> <model> |> <model>
//     input: <coincidence-hash-5>
//     output: <coincidence-hash-5>
//
// The chain is the program. The .shatter file is the chain expression plus
// the input fingerprint plus the output fingerprint. Replaying requires the
// input (or any input with the same fingerprint), and re-runs the chain
// deterministically. Same input + same chain → bit-identical output.
//
// We don't compute. We crystallize.

const CHAIN_PREFIX: &str = "# chain: ";

/// Format a chain-shatter file's text content.
fn format_chain_shatter(chain: &[String], input_oid: &str, output_oid: &str) -> String {
    let mut s = String::new();
    s.push_str("chain: ");
    s.push_str(&chain.join(" |> "));
    s.push('\n');
    s.push_str("input: ");
    s.push_str(input_oid);
    s.push('\n');
    s.push_str("output: ");
    s.push_str(output_oid);
    s.push('\n');
    s
}

/// Parse a chain-shatter file's text content.
/// Returns (chain, input_oid, output_oid).
fn parse_chain_shatter(text: &str) -> Result<(Vec<String>, String, String), String> {
    let mut chain: Vec<String> = Vec::new();
    let mut input_oid = String::new();
    let mut output_oid = String::new();
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("chain:") {
            chain = rest
                .split("|>")
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        } else if let Some(rest) = line.strip_prefix("input:") {
            input_oid = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("output:") {
            output_oid = rest.trim().to_string();
        }
    }
    if chain.is_empty() {
        return Err("chain-shatter: no chain line".to_string());
    }
    Ok((chain, input_oid, output_oid))
}

/// Strip a leading `# chain: <chain>` comment from input source if present.
/// Returns (accumulated_chain, source_without_chain_comment).
fn strip_chain_comment(source: &str) -> (Vec<String>, String) {
    if let Some(rest) = source.strip_prefix(CHAIN_PREFIX) {
        if let Some(newline_idx) = rest.find('\n') {
            let chain_str = &rest[..newline_idx];
            let chain: Vec<String> = chain_str
                .split("|>")
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let body = rest[newline_idx + 1..].to_string();
            return (chain, body);
        }
    }
    (Vec::new(), source.to_string())
}

/// Prepend a `# chain: <chain>` comment line to a body.
fn prepend_chain_comment(chain: &[String], body: &str) -> String {
    let mut s = String::new();
    s.push_str(CHAIN_PREFIX);
    s.push_str(&chain.join(" |> "));
    s.push('\n');
    s.push_str(body);
    s
}

/// Compute a content address for a parsed Form by routing through the runtime.
fn form_oid(form: &Form) -> String {
    let runtime = MirrorRuntime::new();
    let text = emit_form(form);
    match runtime.compile_source(&text) {
        Ok(c) => c.crystal().as_str().to_string(),
        Err(_) => "<unhashable>".to_string(),
    }
}

// ---------------------------------------------------------------------------
// compile mode
// ---------------------------------------------------------------------------

fn cmd_compile(file: &str) -> ! {
    let runtime = MirrorRuntime::new();
    let path = Path::new(file);
    let compiled = match runtime.compile_file(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("mirror: compile {}: {}", file, e);
            process::exit(1);
        }
    };
    // Single-stage chain: the identity transformation. The .shatter file
    // records the input and output OIDs (which are equal for the identity
    // case) and the trivial chain `compile`.
    let oid = compiled.crystal().as_str().to_string();
    let chain = vec!["compile".to_string()];
    let text = format_chain_shatter(&chain, &oid, &oid);
    let target_str = format!("{}.shatter", file);
    let target = Path::new(&target_str);
    if let Err(e) = std::fs::write(target, &text) {
        eprintln!("mirror: write {}: {}", target.display(), e);
        process::exit(1);
    }
    println!("{}", target.display());
    println!("crystal: {}", oid);
    process::exit(0);
}

// ---------------------------------------------------------------------------
// query mode (stub)
// ---------------------------------------------------------------------------

fn cmd_query(query_src: &str, file: &str, compile_target: Option<&str>) -> ! {
    // Parse the query as a Form.
    let query_form = match parse_form(query_src) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("mirror: query parse: {}", e);
            process::exit(1);
        }
    };

    // Parse the target file as a Form (via the runtime).
    let runtime = MirrorRuntime::new();
    let compiled = match runtime.compile_file(Path::new(file)) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("mirror: target {}: {}", file, e);
            process::exit(1);
        }
    };

    // If --compile target.shatter was passed, write a chain-shatter for the
    // identity-compile of the target.
    if let Some(target) = compile_target {
        let oid = compiled.crystal().as_str().to_string();
        let chain = vec!["compile".to_string()];
        let text = format_chain_shatter(&chain, &oid, &oid);
        if let Err(e) = std::fs::write(target, &text) {
            eprintln!("mirror: write {}: {}", target, e);
            process::exit(1);
        }
        println!("{}", target);
        println!("crystal: {}", oid);
    }

    // The form-as-operation semantics are TBD — same posture as `split` and
    // `zoom` on the Shatter Prism trait. Be honest.
    eprintln!(
        "mirror: query parsed as form `{}`, target parsed as form `{}` — \
         applying form-as-operation semantics is TBD; the runtime does not \
         yet implement this.",
        if query_form.name.is_empty() { "<anon>" } else { &query_form.name },
        compiled.form_name(),
    );
    process::exit(2);
}

// ---------------------------------------------------------------------------
// ai mode — fate-driven inference over mirror forms
// ---------------------------------------------------------------------------

/// Read input source: from `path` if Some and not "-", else from stdin.
/// Returns (source_text, label_for_diagnostics).
fn read_input(path: Option<&str>) -> (String, String) {
    match path {
        Some(p) if p != "-" => match std::fs::read_to_string(p) {
            Ok(s) => (s, p.to_string()),
            Err(e) => {
                eprintln!("mirror: read {}: {}", p, e);
                process::exit(1);
            }
        },
        _ => {
            let mut buf = String::new();
            if let Err(e) = std::io::stdin().read_to_string(&mut buf) {
                eprintln!("mirror: read stdin: {}", e);
                process::exit(1);
            }
            (buf, "<stdin>".to_string())
        }
    }
}

/// Deterministic feature extraction from a parsed Form.
///
/// 16 dimensions. Pure structural counts of the form's declarations,
/// normalized to a finite range. No randomness, no time, no env.
/// Same Form → same Features, forever.
fn features_of(form: &Form) -> Features {
    let mut f: Features = [0.0; FEATURE_DIM];
    fn walk(form: &Form, f: &mut Features) {
        // Map DeclKind to a feature slot. We use the discriminant via
        // a stable string label so the mapping survives DeclKind changes.
        let slot = match format!("{:?}", form.kind).as_str() {
            "Form" => 0,
            "Prism" => 1,
            "Lens" => 2,
            "Fold" => 3,
            "Traversal" => 4,
            "Iso" => 5,
            "Setter" => 6,
            "Property" => 7,
            "Requires" => 8,
            "Invariant" => 9,
            "Ensures" => 10,
            "In" => 11,
            "Type" => 12,
            "Boundary" => 13,
            _ => 14,
        };
        f[slot] += 1.0;
        // Slot 15: a structural width signal (params + variants).
        f[15] += (form.params.len() + form.variants.len()) as f64;
        for child in &form.children {
            walk(child, f);
        }
    }
    walk(form, &mut f);
    // Normalize to keep magnitudes small and bounded. Deterministic.
    let total: f64 = f.iter().sum::<f64>().max(1.0);
    for v in f.iter_mut() {
        *v /= total;
    }
    f
}

fn model_name(m: Model) -> &'static str {
    match m {
        Model::Abyss => "abyss",
        Model::Pathfinder => "pathfinder",
        Model::Cartographer => "cartographer",
        Model::Explorer => "explorer",
        Model::Fate => "fate",
    }
}

fn parse_model(name: &str) -> Option<Model> {
    match name {
        "abyss" => Some(Model::Abyss),
        "pathfinder" => Some(Model::Pathfinder),
        "cartographer" => Some(Model::Cartographer),
        "explorer" => Some(Model::Explorer),
        "fate" => Some(Model::Fate),
        _ => None,
    }
}

/// Build the result Form: a small mirror form encoding the selection.
fn selection_form(input_name: &str, from: Model, to: Model) -> Form {
    // form @selection {
    //   prism input <name>
    //   prism from <model>
    //   prism next <model>
    // }
    let child = |label: &str, value: &str| {
        Form::new(
            DeclKind::Prism,
            label.to_string(),
            vec![value.to_string()],
            vec![],
            vec![],
        )
    };
    Form::new(
        DeclKind::Form,
        "@selection".to_string(),
        vec![],
        vec![],
        vec![
            child("input", if input_name.is_empty() { "<anon>" } else { input_name }),
            child("from", model_name(from)),
            child("next", model_name(to)),
        ],
    )
}

fn cmd_ai(
    starting: Model,
    file: Option<&str>,
    out: Option<&str>,
    _train: bool,
    capture: Option<&str>,
) -> ! {
    let (source, label) = read_input(file);

    // Strip any leading `# chain: ...` comment from the input source.
    // This is how the chain accumulates across pipeline stages.
    let (mut accumulated_chain, body) = strip_chain_comment(&source);

    let form = match parse_form(&body) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("mirror: ai parse {}: {}", label, e);
            process::exit(1);
        }
    };

    // Compute features and run the model.
    let features = features_of(&form);
    let runtime = FateRuntime::new();
    let next = runtime.select(starting, &features);
    let result = selection_form(&form.name, starting, next);
    let result_text = emit_form(&result);

    // Append this stage's model name to the accumulated chain.
    accumulated_chain.push(model_name(starting).to_string());

    // Output: prepend the accumulated chain comment to the result text.
    // This is what makes the next stage in the pipeline able to read the
    // chain so far.
    let stamped = prepend_chain_comment(&accumulated_chain, &result_text);

    // Capture: if --capture <file> was passed, write the chain-shatter
    // file. We compute input/output OIDs by routing through the runtime.
    if let Some(cap) = capture {
        let input_oid = form_oid(&form);
        let output_oid = form_oid(&result);
        let shatter_text =
            format_chain_shatter(&accumulated_chain, &input_oid, &output_oid);
        if let Err(e) = std::fs::write(cap, &shatter_text) {
            eprintln!("mirror: write capture {}: {}", cap, e);
            process::exit(1);
        }
    }

    match out {
        Some(path) => {
            if let Err(e) = std::fs::write(path, &stamped) {
                eprintln!("mirror: write {}: {}", path, e);
                process::exit(1);
            }
        }
        None => {
            print!("{}", stamped);
            if !stamped.ends_with('\n') {
                println!();
            }
        }
    }
    process::exit(0);
}

// ---------------------------------------------------------------------------
// replay mode — re-run a chain against an input file
// ---------------------------------------------------------------------------

/// Pure ai_step: take a model and a parsed Form, return the result Form.
/// Used by both cmd_ai (with I/O around it) and cmd_replay (in a loop).
fn ai_step(starting: Model, form: &Form) -> Form {
    let features = features_of(form);
    let runtime = FateRuntime::new();
    let next = runtime.select(starting, &features);
    selection_form(&form.name, starting, next)
}

fn cmd_replay(shatter_file: &str, input_file: &str) -> ! {
    let shatter_text = match std::fs::read_to_string(shatter_file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("mirror: replay read {}: {}", shatter_file, e);
            process::exit(1);
        }
    };
    let (chain, _input_oid, _output_oid) = match parse_chain_shatter(&shatter_text) {
        Ok(parts) => parts,
        Err(e) => {
            eprintln!("mirror: replay parse: {}", e);
            process::exit(1);
        }
    };
    let input_text = match std::fs::read_to_string(input_file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("mirror: replay input {}: {}", input_file, e);
            process::exit(1);
        }
    };
    // Strip any pre-existing chain comment from the input file (it's
    // possible the input was itself the output of a previous stage).
    let (_pre_chain, body) = strip_chain_comment(&input_text);
    let mut current = match parse_form(&body) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("mirror: replay input parse: {}", e);
            process::exit(1);
        }
    };

    // Run each model in the chain in sequence.
    let mut accumulated: Vec<String> = Vec::new();
    for model_str in &chain {
        // The "compile" pseudo-model in single-stage chains is the identity
        // — it preserves the form unchanged.
        if model_str == "compile" {
            accumulated.push("compile".to_string());
            continue;
        }
        let m = match parse_model(model_str) {
            Some(m) => m,
            None => {
                eprintln!("mirror: replay unknown model: {}", model_str);
                process::exit(1);
            }
        };
        current = ai_step(m, &current);
        accumulated.push(model_name(m).to_string());
    }

    let result_text = emit_form(&current);
    let stamped = prepend_chain_comment(&accumulated, &result_text);
    print!("{}", stamped);
    if !stamped.ends_with('\n') {
        println!();
    }
    process::exit(0);
}

/// Parse `--out=<file>`, `--train`, `--capture=<file>` flags out of an arg slice.
/// Returns (out, train, capture, remaining positional args).
/// Both `--capture=<file>` and `--capture <file>` (two-arg) forms are accepted.
fn parse_ai_flags(args: &[String]) -> (Option<String>, bool, Option<String>, Vec<String>) {
    let mut out = None;
    let mut train = false;
    let mut capture = None;
    let mut rest = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        if let Some(v) = a.strip_prefix("--out=") {
            out = Some(v.to_string());
        } else if let Some(v) = a.strip_prefix("--capture=") {
            capture = Some(v.to_string());
        } else if a == "--capture" && i + 1 < args.len() {
            capture = Some(args[i + 1].clone());
            i += 1;
        } else if a == "--train" {
            train = true;
        } else {
            rest.push(a.clone());
        }
        i += 1;
    }
    (out, train, capture, rest)
}

/// Resolve the file argument: explicit positional, or None which means
/// "stdin if not a tty, otherwise error".
fn resolve_file_arg(positional: &[String]) -> Option<String> {
    if let Some(p) = positional.first() {
        return Some(p.clone());
    }
    if !std::io::stdin().is_terminal() {
        return Some("-".to_string());
    }
    eprintln!("mirror: ai: no input file and stdin is a tty");
    process::exit(1);
}

// ---------------------------------------------------------------------------
// argv
// ---------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage_and_exit();
    }

    // `mirror fmt <file>` → ai fate --train --out=<file> <file>
    if args[1] == "fmt" {
        if args.len() != 3 {
            print_usage_and_exit();
        }
        let file = args[2].clone();
        cmd_ai(Model::Fate, Some(&file), Some(&file), true, None);
    }

    // `mirror ai ...`
    if args[1] == "ai" {
        let rest: Vec<String> = args[2..].to_vec();
        let (out, train, capture, positional) = parse_ai_flags(&rest);
        // First positional may be a model name OR a file path/'-'.
        let (starting, file_args): (Model, &[String]) = match positional.first() {
            Some(first) => match parse_model(first) {
                Some(m) => (m, &positional[1..]),
                None => (Model::Fate, &positional[..]),
            },
            None => (Model::Fate, &positional[..]),
        };
        let file = resolve_file_arg(file_args);
        cmd_ai(starting, file.as_deref(), out.as_deref(), train, capture.as_deref());
    }

    // `compile <file>`
    if args[1] == "compile" {
        if args.len() != 3 {
            print_usage_and_exit();
        }
        cmd_compile(&args[2]);
    }

    // `replay <shatter-file> <input-file>`
    if args[1] == "replay" {
        if args.len() != 4 {
            eprintln!("usage: mirror replay <chain.shatter> <input.mirror>");
            process::exit(1);
        }
        cmd_replay(&args[2], &args[3]);
    }

    // `<query> <file> [--compile <target>]`
    //
    // Heuristic: a query string contains characters that aren't valid in a
    // bare subcommand name (`{`, `@`, space, etc.). Anything else falls to
    // usage.
    if args.len() >= 3 && looks_like_query(&args[1]) {
        let query = args[1].clone();
        let file = args[2].clone();
        let mut compile_target: Option<String> = None;
        let mut i = 3;
        while i < args.len() {
            if args[i] == "--compile" && i + 1 < args.len() {
                compile_target = Some(args[i + 1].clone());
                i += 2;
            } else {
                eprintln!("mirror: unrecognized argument: {}", args[i]);
                print_usage_and_exit();
            }
        }
        cmd_query(&query, &file, compile_target.as_deref());
    }

    print_usage_and_exit();
}

fn looks_like_query(s: &str) -> bool {
    // Anything containing whitespace, braces, or starting with a known
    // mirror keyword is treated as a query string.
    s.contains(' ')
        || s.contains('{')
        || s.contains('}')
        || s.starts_with("form")
        || s.starts_with("prism")
        || s.starts_with("fold")
        || s.starts_with("lens")
        || s.starts_with("traversal")
        || s.starts_with("iso")
        || s.starts_with("setter")
}
