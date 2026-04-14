//! Cli -- the mirror command-line interface.
//!
//! Wraps `MirrorRuntime` and tracks the crystal OID for the loaded spec.
//! Dispatch routes commands to the appropriate handler.

use crate::declaration::MirrorHash;
use crate::mirror_runtime::{MirrorRuntime, MirrorRuntimeError};
use fragmentation::sha::HashAlg;

use std::path::Path;

// ---------------------------------------------------------------------------
// CliError
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum CliError {
    Runtime(MirrorRuntimeError),
    Usage(String),
    Io(std::io::Error),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Runtime(e) => write!(f, "{}", e),
            CliError::Usage(msg) => write!(f, "{}", msg),
            CliError::Io(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for CliError {}

impl From<MirrorRuntimeError> for CliError {
    fn from(e: MirrorRuntimeError) -> Self {
        CliError::Runtime(e)
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Io(e)
    }
}

// ---------------------------------------------------------------------------
// Cli
// ---------------------------------------------------------------------------

/// The mirror CLI. Wraps a `MirrorRuntime` and optionally holds the
/// crystal OID from compiling `spec.mirror`.
pub struct Cli {
    pub runtime: MirrorRuntime,
    crystal_oid: Option<MirrorHash>,
}

/// Information about a branch's merge readiness.
struct BranchInfo {
    name: String,
    commits_ahead: usize,
    has_conflicts: bool,
}

impl Cli {
    /// Open the CLI. If `spec_path` exists, compile it and store
    /// the crystal OID. If it doesn't exist, continue without one.
    pub fn open(spec_path: &str) -> Result<Self, CliError> {
        let runtime = MirrorRuntime::new();
        let crystal_oid = if Path::new(spec_path).exists() {
            let compiled = runtime.compile_file(Path::new(spec_path))?;
            Some(compiled.crystal().clone())
        } else {
            None
        };
        Ok(Cli {
            runtime,
            crystal_oid,
        })
    }

    /// Return the loaded crystal's OID, if any.
    pub fn crystal_oid(&self) -> Option<&MirrorHash> {
        self.crystal_oid.as_ref()
    }

    /// Dispatch a command by name.
    pub fn dispatch(&self, command: &str, args: &[String]) -> Result<String, CliError> {
        match command {
            "help" | "--help" | "-h" => Ok(Self::help_text().to_string()),
            "compile" => self.cmd_compile(args),
            "crystal" => self.cmd_crystal(args),
            "ai" => self.cmd_ai(args),
            "ci" => self.cmd_ci(args),
            "lsp" => self.cmd_lsp(args),
            "ca" => {
                if args.iter().any(|a| a == "--merge") {
                    self.cmd_ca_merge()
                } else {
                    self.cmd_ca(args)
                }
            }
            "merge" => self.cmd_merge(args),
            "bench" => self.cmd_bench(args),
            "verify" => self.cmd_verify(args),
            "init" => self.cmd_init(args),
            "repl" => self.cmd_repl(args),
            "kintsugi" => self.cmd_kintsugi(args),
            "focus" | "project" | "split" | "zoom" | "refract" => self.cmd_optic(command, args),
            "registry" => self.cmd_registry(args),
            "git" => self.cmd_git(args),
            _ => self.cmd_query(command, args),
        }
    }

    /// The help text. Matches the grammar declared in boot/20-cli.mirror.
    pub fn help_text() -> &'static str {
        "\
mirror -- an honest compiler

usage: mirror <command> [args]

optics:
  focus <path>       observe structure
  project <path>     filter by what matters
  split <path>       explore connections
  zoom <path>        transform
  refract <path>     settle into crystal

compiler:
  compile <path>     compile a .mirror file
  crystal [output]   materialize the standard library
  ci <path>          measure holonomy
  ca <path>          observe, suggest, enforce
  kintsugi <path>    canonical ordering (the formatter)
  verify <file>      verify signed .shatter

session:
  init               initialize .git/mirror/
  merge [target]     structural merge with crystal delta
  repl               interactive shard> prompt

tools:
  ai <model> [path]  run a Fate model
  bench <path>       benchmark compilation
  git <subcommand>   read-only prism over git's ref space

flags:
  --oid              print content address only
  --sign             sign compilation output (compile)
  --strict           reject Partial results (compile, ci)
  --check            verify canonical order (kintsugi)
  --help             this message"
    }

    /// Per-command help.
    pub fn command_help(command: &str) -> Option<&'static str> {
        match command {
            "focus" => Some("focus <path> -- observe the structure of a .mirror file\n\nParses the file and prints the content-addressed AST.\nThe focus optic reads without changing."),
            "project" => Some("project <path> -- filter by what matters\n\nExtracts the eigenvalues: kind, name, params, variants.\nThe projection keeps what survives."),
            "split" => Some("split <path> -- explore connections\n\nShows the branches: variants, forks, alternatives.\nThe split optic maps one-to-many."),
            "zoom" => Some("zoom <path> -- transform\n\nMoves between levels of abstraction.\nThe zoom optic changes coordinates."),
            "refract" => Some("refract <path> -- settle into crystal\n\nRuns the full compilation loop until the OID stabilizes.\nThe refract optic scatters and reconverges."),
            "compile" => Some("compile <path> [--sign] [--target rust] -- compile a .mirror file\n\nParses, resolves, and content-addresses the source.\nPrints the crystal OID to stdout.\nWith --sign: produces .shatter.sig alongside .shatter.\nWith --target rust: emits .rs file alongside .shatter."),
            "crystal" => Some("crystal [output] -- materialize the standard library\n\nCompiles boot/ in order and emits mirror.shatter.\nWith --oid: prints the loaded crystal OID."),
            "ci" => Some("ci <path> -- measure holonomy\n\nCompiles and reports the MirrorLoss.\nZero holonomy means crystal. Nonzero means alive."),
            "kintsugi" => Some("kintsugi <path> [--check] -- canonical ordering\n\nReorders declarations: in, type, traversal, lens, grammar, property, action.\nThe OID doesn't change. The surface does.\nWith --check: exit 0 if already canonical, exit 1 if not."),
            "init" => Some("init -- initialize .git/mirror/\n\nSets up the mirror store in the current git repository."),
            "merge" => Some("merge [target] [--ai] -- structural merge with crystal delta\n\nCompiles current and target branches, diffs crystals.\nWith --ai: generates merge commit message from delta."),
            "repl" => Some("repl -- interactive shard> prompt\n\nStarts an interactive session.\nType .mirror expressions and see them compiled live."),
            "ai" => Some("ai <model> [path] -- run a Fate model\n\nModels: abyss | introject | cartographer | explorer | fate\n\nReads from <path> or stdin. Routes through the named model."),
            "ca" => Some("ca <path> [--enforce] -- observe, suggest, enforce\n\nRuns CI, then if holonomy > 0, produces suggestions.\nWith --enforce: applies suggestions (not yet implemented)."),
            "bench" => Some("bench <path> -- benchmark compilation\n\nMeasures compilation time and structural loss.\nPrints timing and MirrorLoss summary."),
            "verify" => Some("verify <file> -- verify a signed .shatter file\n\nChecks the Ed25519 signature (.shatter.sig) against the content.\nUses the public key from CONVERSATION_KEYS hierarchy.\nExits 0 if valid, nonzero if tampered or unsigned."),
            "git" => Some("git <subcommand> -- read-only prism over git's ref space\n\nSubcommands:\n  refs              list all refs (branches, tags, HEAD)\n  tree <ref>        show the tree at a ref\n  show <ref>:<path> read a blob without checkout\n  diff <a> <b>      structural diff between two refs\n  log               commit history (short)"),
            _ => None,
        }
    }

    // -----------------------------------------------------------------------
    // compile
    // -----------------------------------------------------------------------

    fn cmd_compile(&self, args: &[String]) -> Result<String, CliError> {
        let sign = args.iter().any(|a| a == "--sign");
        let strict = args.iter().any(|a| a == "--strict");
        let target_rust = args.iter().any(|a| a == "--target" || a == "--rust");
        let file_args: Vec<&String> = args
            .iter()
            .filter(|a| !a.starts_with("--"))
            .filter(|a| a.as_str() != "rust") // skip target value after --target
            .collect();
        let file = file_args.first().ok_or_else(|| {
            CliError::Usage(
                "usage: mirror compile <file> [--sign] [--strict] [--target rust]".to_string(),
            )
        })?;

        let source = std::fs::read_to_string(file.as_str())?;
        let mut compiler = crate::bundle::MirrorCompiler::new();

        // --strict: Partial becomes Failure (the Prism applied to the result)
        if strict {
            let result = self.runtime.compile_source(&source);
            if result.is_partial() {
                return Err(CliError::Runtime(MirrorRuntimeError(format!(
                    "compile {} --strict: partial result rejected (holonomy: {:.4})",
                    file,
                    result.loss().holonomy()
                ))));
            }
        }

        match compiler.compile(&source) {
            Ok(compiled) => {
                let shard = crate::shard::Shard::new(
                    compiled.crystal().clone(),
                    compiler.kernel_spec.clone(),
                    compiler.target,
                );
                let oid = shard.grammar_oid.clone();

                // Write .shatter output alongside the source
                let shatter_path = std::path::Path::new(file.as_str()).with_extension("shatter");
                std::fs::write(&shatter_path, &source)?;

                // Best-effort: store .shatter artifact in .git/mirror/ if we're in a git repo.
                // Silently skips if there's no git repo or the store can't be opened.
                if let Ok(git_store) = crate::git_store::MirrorGitStore::open(
                    &std::env::current_dir().unwrap_or_default(),
                ) {
                    use crate::loss::MirrorLoss;
                    use crate::shatter_format::{emit_shatter_with_frontmatter, ShatterMeta};
                    use prism::Loss as _;
                    let loss = MirrorLoss::zero();
                    let meta = ShatterMeta::from_compiled(&compiled, &loss);
                    let shatter_content = emit_shatter_with_frontmatter(&meta, &source);
                    git_store.store_shatter(&meta.oid, &shatter_content);
                    let _ = git_store.set_file_ref(file.as_str(), &meta.oid);
                }

                eprintln!(
                    "compiled {} -> {} (rank {}, {:?})",
                    file,
                    oid.as_str(),
                    shard.rank(),
                    shard.target,
                );

                // --target rust: emit Rust source alongside .shatter
                if target_rust {
                    let rust_code = crate::emit_rust::emit_rust(&compiled);
                    let rs_path = std::path::Path::new(file.as_str()).with_extension("rs");
                    std::fs::write(&rs_path, &rust_code)?;
                    eprintln!("emitted {}", rs_path.display());
                }

                if sign {
                    #[cfg(feature = "git")]
                    {
                        let content_oid = crate::Oid::hash(source.as_bytes());
                        let private_key = crate::sign::resolve_private_key()
                            .map_err(|e| CliError::Usage(format!("--sign: {}", e)))?;
                        let sig_pem = crate::sign::sign_oid(&private_key, &content_oid)
                            .map_err(|e| CliError::Usage(format!("--sign: {}", e)))?;
                        let sig_path = shatter_path.with_extension("shatter.sig");
                        std::fs::write(&sig_path, &sig_pem)?;
                        eprintln!("signed {} -> {}", oid.as_str(), sig_path.display());
                    }
                    #[cfg(not(feature = "git"))]
                    {
                        return Err(CliError::Usage(
                            "--sign requires the \'git\' feature (ssh-key crate)".to_string(),
                        ));
                    }
                }

                Ok(oid.as_str().to_string())
            }
            Err(e) => Err(CliError::Runtime(MirrorRuntimeError(format!(
                "compile {}: {}",
                file, e
            )))),
        }
    }

    // -----------------------------------------------------------------------
    // crystal
    // -----------------------------------------------------------------------

    fn cmd_crystal(&self, args: &[String]) -> Result<String, CliError> {
        // Check for --oid flag
        if args.iter().any(|a| a == "--oid") {
            return match &self.crystal_oid {
                Some(oid) => Ok(oid.as_str().to_string()),
                None => Err(CliError::Usage(
                    "no crystal loaded (no spec.mirror found)".to_string(),
                )),
            };
        }

        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let boot_dir = manifest_dir.join("boot");
        let default_output = manifest_dir.join("mirror.shatter");

        let output = match args.first() {
            Some(f) => std::path::PathBuf::from(f),
            None => default_output,
        };

        let store_dir = std::env::temp_dir().join(format!("mirror-crystal-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&store_dir);

        match self
            .runtime
            .materialize_crystal(&boot_dir, &store_dir, &output)
        {
            Ok(oid) => {
                eprintln!("crystal {} -> {}", oid.as_str(), output.display());
                let _ = std::fs::remove_dir_all(&store_dir);
                Ok(oid.as_str().to_string())
            }
            Err(e) => {
                let _ = std::fs::remove_dir_all(&store_dir);
                Err(CliError::Runtime(MirrorRuntimeError(format!(
                    "materialize crystal: {}",
                    e
                ))))
            }
        }
    }

    // -----------------------------------------------------------------------
    // ai
    // -----------------------------------------------------------------------

    fn cmd_ai(&self, args: &[String]) -> Result<String, CliError> {
        let model = match args.first().map(|s| s.as_str()) {
            Some("abyss") => "abyss",
            Some("introject") => "introject",
            Some("cartographer") => "cartographer",
            Some("explorer") => "explorer",
            Some("fate") => "fate",
            Some(other) => {
                eprintln!("ai fate {}", other);
                return Ok(format!("ai fate {}", other));
            }
            None => {
                return Err(CliError::Usage(
                    "usage: mirror ai <model> [file|-]".to_string(),
                ));
            }
        };

        let file = args.get(1).map(|s| s.as_str());
        eprintln!("ai {} {}", model, file.unwrap_or("<stdin>"));
        Ok(format!("ai {} {}", model, file.unwrap_or("<stdin>")))
    }

    // -----------------------------------------------------------------------
    // optic commands -- focus, project, split, zoom, refract
    // -----------------------------------------------------------------------

    fn cmd_optic(&self, optic: &str, args: &[String]) -> Result<String, CliError> {
        if args.iter().any(|a| a == "--help" || a == "-h") {
            return Ok(Self::command_help(optic).unwrap_or("").to_string());
        }

        let file = args
            .first()
            .ok_or_else(|| CliError::Usage(format!("usage: mirror {} <path>", optic)))?;

        let source = std::fs::read_to_string(file)?;
        let compiled: Result<_, _> = self.runtime.compile_source(&source).into();
        let compiled = compiled?;

        match optic {
            "focus" => {
                let text = crate::mirror_runtime::emit_fragment(&compiled.fragment);
                Ok(text)
            }
            "project" => {
                let mut out = String::new();
                project_fragment(&compiled.fragment, 0, &mut out);
                Ok(out)
            }
            "refract" => Ok(compiled.crystal().as_str().to_string()),
            _ => {
                // split, zoom -- same as focus for now
                let text = crate::mirror_runtime::emit_fragment(&compiled.fragment);
                Ok(text)
            }
        }
    }

    // -----------------------------------------------------------------------
    // kintsugi -- canonical ordering (the formatter)
    // -----------------------------------------------------------------------

    fn cmd_kintsugi(&self, args: &[String]) -> Result<String, CliError> {
        let check = args.iter().any(|a| a == "--check");
        let file_args: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
        let file = file_args.first().ok_or_else(|| {
            CliError::Usage("usage: mirror kintsugi <file> [--check]".to_string())
        })?;

        let source = std::fs::read_to_string(file.as_str())?;
        let compiled: Result<_, _> = self.runtime.compile_source(&source).into();
        let compiled = compiled?;

        let canonical = crate::mirror_runtime::kintsugi_fragment(&compiled.fragment);
        let output = crate::mirror_runtime::emit_fragment(&canonical);

        if check {
            let original = crate::mirror_runtime::emit_fragment(&compiled.fragment);
            if output == original {
                Ok("ok".to_string())
            } else {
                Err(CliError::Usage(
                    "kintsugi --check: source is not canonical".to_string(),
                ))
            }
        } else {
            Ok(output)
        }
    }

    // -----------------------------------------------------------------------
    // ci -- measure holonomy
    // -----------------------------------------------------------------------

    fn ci_single_file(&self, path: &str) -> Result<(String, crate::loss::MirrorLoss), CliError> {
        use prism::{Loss, Transport};
        let source = std::fs::read_to_string(path)?;
        let compiler = crate::bundle::MirrorCompiler::new();
        let result = compiler.transport(&source);
        match result {
            prism::Imperfect::Success(oid) => Ok((oid, crate::loss::MirrorLoss::zero())),
            prism::Imperfect::Partial(oid, loss) => Ok((oid, loss)),
            prism::Imperfect::Failure(_, _) => Err(CliError::Runtime(MirrorRuntimeError(
                "compilation failed".to_string(),
            ))),
        }
    }

    fn cmd_ci(&self, args: &[String]) -> Result<String, CliError> {
        if args.iter().any(|a| a == "--help" || a == "-h") {
            return Ok(Self::command_help("ci").unwrap_or("").to_string());
        }
        let path = args
            .first()
            .ok_or_else(|| CliError::Usage("usage: mirror ci <path>".to_string()))?;
        use prism::Loss;
        let p = Path::new(path);
        if p.is_dir() {
            let mut entries: Vec<_> = std::fs::read_dir(p)?
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "mirror"))
                .collect();
            entries.sort_by_key(|e| e.file_name());
            if entries.is_empty() {
                return Err(CliError::Usage(format!("no .mirror files in {}", path)));
            }
            let mut total_loss = crate::loss::MirrorLoss::zero();
            let mut file_count = 0usize;
            let mut out = String::new();
            for entry in &entries {
                let file_path = entry.path();
                let name = file_path.file_name().unwrap().to_string_lossy();
                match self.ci_single_file(file_path.to_str().unwrap()) {
                    Ok((_oid, loss)) => {
                        let h = loss.holonomy();
                        out.push_str(&format!("  {} holonomy: {:.4}\n", name, h));
                        total_loss = total_loss.combine(loss);
                        file_count += 1;
                    }
                    Err(e) => {
                        out.push_str(&format!("  {} FAIL: {}\n", name, e));
                    }
                }
            }
            let total_h = total_loss.holonomy();
            out.insert_str(
                0,
                &format!(
                    "ci {} ({} files)\nholonomy: {:.4}\n",
                    path, file_count, total_h
                ),
            );
            Ok(out)
        } else {
            let (_oid, loss) = self.ci_single_file(path)?;
            let holonomy = loss.holonomy();
            let mut out = String::new();
            if loss.is_zero() {
                out.push_str(&format!("crystal\nholonomy: {:.4}", holonomy));
            } else {
                out.push_str(&format!(
                    "partial\nholonomy: {:.4}\nphases: {}\nresolution: {:.2}",
                    holonomy,
                    loss.emit.phases.len(),
                    loss.resolution.resolution_ratio
                ));
                if !loss.resolution.unresolved_refs.is_empty() {
                    out.push_str(&format!(
                        "\nunresolved: {}",
                        loss.resolution.unresolved_refs.len()
                    ));
                    for (name, _oid) in &loss.resolution.unresolved_refs {
                        out.push_str(&format!("\n  - {}", name));
                    }
                }
            }
            Ok(out)
        }
    }

    // -----------------------------------------------------------------------
    // ca -- observe + suggest + enforce
    // -----------------------------------------------------------------------

    fn cmd_ca(&self, args: &[String]) -> Result<String, CliError> {
        if args.iter().any(|a| a == "--help" || a == "-h") {
            return Ok(Self::command_help("ca").unwrap_or("").to_string());
        }
        let path = args
            .first()
            .ok_or_else(|| CliError::Usage("usage: mirror ca <path> [--enforce]".to_string()))?;
        use prism::Loss;
        let ci_result = self.cmd_ci(args)?;
        let p = Path::new(path);
        let loss = if p.is_dir() {
            let mut total = crate::loss::MirrorLoss::zero();
            if let Ok(entries) = std::fs::read_dir(p) {
                for entry in entries.filter_map(|e| e.ok()) {
                    if entry.path().extension().is_some_and(|ext| ext == "mirror") {
                        if let Ok((_oid, l)) = self.ci_single_file(entry.path().to_str().unwrap()) {
                            total = total.combine(l);
                        }
                    }
                }
            }
            total
        } else {
            match self.ci_single_file(path) {
                Ok((_oid, l)) => l,
                Err(_) => crate::loss::MirrorLoss::total(),
            }
        };
        if loss.is_zero() {
            return Ok("crystal. nothing to do.".to_string());
        }
        let mut out = ci_result;
        out.push_str("\n---\nsuggestions:");
        for phase in &loss.emit.phases {
            if phase.structural_loss > 0.0 {
                out.push_str(&format!(
                    "\n  {:?} phase: loss {:.4}",
                    phase.phase, phase.structural_loss
                ));
            }
        }
        if !loss.resolution.unresolved_refs.is_empty() {
            out.push_str(&format!(
                "\n  unresolved refs: {}",
                loss.resolution.unresolved_refs.len()
            ));
        }
        let enforce = args.iter().any(|a| a == "--enforce");
        if enforce {
            out.push_str("\nenforce: not yet implemented");
        }
        Ok(out)
    }

    // -----------------------------------------------------------------------
    // ca --merge -- merge branches into main
    // -----------------------------------------------------------------------

    /// List all local branches except main and the current branch.
    fn list_branches(repo_dir: &Path) -> Result<Vec<String>, CliError> {
        let output = std::process::Command::new("git")
            .args(["branch", "--list", "--format=%(refname:short)"])
            .current_dir(repo_dir)
            .output()
            .map_err(CliError::Io)?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|b| !b.is_empty() && b != "main")
            .collect())
    }

    /// Count commits on `branch` that are not on main.
    fn commits_ahead(repo_dir: &Path, branch: &str) -> Result<usize, CliError> {
        let output = std::process::Command::new("git")
            .args(["rev-list", "--count", &format!("main..{}", branch)])
            .current_dir(repo_dir)
            .output()
            .map_err(CliError::Io)?;
        Ok(String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse()
            .unwrap_or(0))
    }

    /// Attempt a merge. On conflict, abort and return Err.
    fn try_merge(repo_dir: &Path, branch: &str) -> Result<(), CliError> {
        let output = std::process::Command::new("git")
            .args(["merge", "--no-edit", "--no-ff", branch])
            .current_dir(repo_dir)
            .output()
            .map_err(CliError::Io)?;
        if output.status.success() {
            Ok(())
        } else {
            let _ = std::process::Command::new("git")
                .args(["merge", "--abort"])
                .current_dir(repo_dir)
                .output();
            Err(CliError::Runtime(MirrorRuntimeError(format!(
                "merge conflict: {}",
                branch
            ))))
        }
    }

    /// Run `cargo test --quiet` in the given directory and return whether it passed.
    fn cargo_test_passes(repo_dir: &Path) -> bool {
        std::process::Command::new("cargo")
            .args(["test", "--lib", "--quiet"])
            .current_dir(repo_dir)
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    /// Undo the last merge commit (used when tests fail after merge).
    fn undo_last_merge(repo_dir: &Path) -> Result<(), CliError> {
        let output = std::process::Command::new("git")
            .args(["reset", "--hard", "HEAD~1"])
            .current_dir(repo_dir)
            .output()
            .map_err(CliError::Io)?;
        if output.status.success() {
            Ok(())
        } else {
            Err(CliError::Runtime(MirrorRuntimeError(
                "failed to undo merge".to_string(),
            )))
        }
    }

    /// Get the current branch name.
    fn current_branch(repo_dir: &Path) -> Result<String, CliError> {
        let output = std::process::Command::new("git")
            .args(["branch", "--show-current"])
            .current_dir(repo_dir)
            .output()
            .map_err(CliError::Io)?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Analyze a branch: count commits ahead and check for conflicts.
    fn analyze_branch(repo_dir: &Path, branch: &str) -> Result<BranchInfo, CliError> {
        let ahead = Self::commits_ahead(repo_dir, branch)?;

        // Dry-run merge to check for conflicts
        let output = std::process::Command::new("git")
            .args(["merge", "--no-commit", "--no-ff", branch])
            .current_dir(repo_dir)
            .output()
            .map_err(CliError::Io)?;
        let has_conflicts = !output.status.success();

        // Always abort the dry-run
        let _ = std::process::Command::new("git")
            .args(["merge", "--abort"])
            .current_dir(repo_dir)
            .output();

        Ok(BranchInfo {
            name: branch.to_string(),
            commits_ahead: ahead,
            has_conflicts,
        })
    }

    fn cmd_ca_merge(&self) -> Result<String, CliError> {
        self.cmd_ca_merge_in(&std::env::current_dir().map_err(CliError::Io)?)
    }

    fn cmd_ca_merge_in(&self, repo_dir: &Path) -> Result<String, CliError> {
        // 1. Must be on main
        let current = Self::current_branch(repo_dir)?;
        if current != "main" {
            return Err(CliError::Usage(
                "ca --merge must be run from main".to_string(),
            ));
        }

        // 2. List branches
        let branches = Self::list_branches(repo_dir)?;
        if branches.is_empty() {
            return Ok("nothing to merge".to_string());
        }

        // 3. Analyze each branch
        let mut infos: Vec<BranchInfo> = Vec::new();
        for branch in &branches {
            match Self::analyze_branch(repo_dir, branch) {
                Ok(info) => infos.push(info),
                Err(e) => eprintln!("  skip {}: {}", branch, e),
            }
        }

        // 4. Sort: no-conflict first, then by commits ahead ascending
        infos.sort_by(|a, b| {
            a.has_conflicts
                .cmp(&b.has_conflicts)
                .then(a.commits_ahead.cmp(&b.commits_ahead))
        });

        // 5. Merge loop
        let mut merged: Vec<String> = Vec::new();
        let mut skipped: Vec<String> = Vec::new();

        for info in &infos {
            eprintln!(
                "  trying {} ({} ahead, conflicts: {})",
                info.name, info.commits_ahead, info.has_conflicts
            );

            if info.has_conflicts {
                eprintln!("  skipped (conflicts): {}", info.name);
                skipped.push(info.name.clone());
                continue;
            }

            match Self::try_merge(repo_dir, &info.name) {
                Ok(()) => {
                    if Self::cargo_test_passes(repo_dir) {
                        eprintln!("  merged: {}", info.name);
                        merged.push(info.name.clone());
                    } else {
                        eprintln!("  skipped (tests fail): {}", info.name);
                        if let Err(e) = Self::undo_last_merge(repo_dir) {
                            eprintln!("  warning: failed to undo merge: {}", e);
                        }
                        skipped.push(info.name.clone());
                    }
                }
                Err(_) => {
                    eprintln!("  skipped (merge failed): {}", info.name);
                    skipped.push(info.name.clone());
                }
            }
        }

        // 6. Re-crystallize if anything merged
        let crystal_note = if !merged.is_empty() {
            match self.cmd_crystal(&["mirror.shatter".into()]) {
                Ok(oid) => format!("\ncrystal: {}", oid),
                Err(e) => format!("\ncrystal: failed ({})", e),
            }
        } else {
            String::new()
        };

        // 7. Report
        let report = format!(
            "ca --merge: {} merged, {} skipped{}",
            merged.len(),
            skipped.len(),
            crystal_note
        );
        Ok(report)
    }

    // -----------------------------------------------------------------------
    // bench -- benchmark compilation
    // -----------------------------------------------------------------------

    fn cmd_bench(&self, args: &[String]) -> Result<String, CliError> {
        if args.iter().any(|a| a == "--help" || a == "-h") {
            return Ok(Self::command_help("bench").unwrap_or("").to_string());
        }
        let file = args
            .first()
            .ok_or_else(|| CliError::Usage("usage: mirror bench <path>".to_string()))?;

        let source = std::fs::read_to_string(file)?;
        let start = std::time::Instant::now();
        let compiled: Result<_, _> = self.runtime.compile_source(&source).into();
        let compiled = compiled?;
        let elapsed = start.elapsed();
        Ok(format!(
            "compiled {} in {:.3}ms\noid: {}",
            file,
            elapsed.as_secs_f64() * 1000.0,
            compiled.crystal().as_str()
        ))
    }

    // -----------------------------------------------------------------------
    // verify -- check Ed25519 signature on .shatter
    // -----------------------------------------------------------------------

    fn cmd_verify(&self, args: &[String]) -> Result<String, CliError> {
        if args.iter().any(|a| a == "--help" || a == "-h") {
            return Ok(Self::command_help("verify").unwrap_or("").to_string());
        }

        #[cfg(feature = "git")]
        {
            let file = args
                .first()
                .ok_or_else(|| CliError::Usage("usage: mirror verify <file>".to_string()))?;

            let sig_path_arg = args
                .iter()
                .position(|a| a == "--sig")
                .and_then(|i| args.get(i + 1));

            let sig_path = match sig_path_arg {
                Some(p) => std::path::PathBuf::from(p),
                None => {
                    let p = std::path::Path::new(file.as_str());
                    p.with_extension("shatter.sig")
                }
            };

            let content = std::fs::read_to_string(file)?;
            let sig_pem = std::fs::read_to_string(&sig_path).map_err(|_| {
                CliError::Usage(format!(
                    "signature file not found: {} (compile with --sign first)",
                    sig_path.display()
                ))
            })?;

            let oid = crate::Oid::hash(content.as_bytes());
            let public_key = crate::sign::resolve_public_key()
                .map_err(|e| CliError::Usage(format!("verify: {}", e)))?;

            match crate::sign::verify_oid(&public_key, &oid, &sig_pem) {
                Ok(()) => Ok(format!("verified {}", oid.as_ref())),
                Err(e) => Err(CliError::Usage(format!(
                    "verification failed for {}: {}",
                    file, e
                ))),
            }
        }

        #[cfg(not(feature = "git"))]
        {
            let _ = args;
            Err(CliError::Usage(
                "verify requires the \'git\' feature (ssh-key crate)".to_string(),
            ))
        }
    }

    // -----------------------------------------------------------------------
    // init -- initialize .git/mirror/
    // -----------------------------------------------------------------------

    fn cmd_init(&self, args: &[String]) -> Result<String, CliError> {
        if args.iter().any(|a| a == "--help" || a == "-h") {
            return Ok(Self::command_help("init").unwrap_or("").to_string());
        }
        let mirror_dir = std::path::PathBuf::from(".git/mirror");
        if mirror_dir.exists() {
            return Ok("mirror store already initialized".to_string());
        }
        std::fs::create_dir_all(&mirror_dir)?;
        Ok(format!(
            "initialized mirror store at {}",
            mirror_dir.display()
        ))
    }

    // -----------------------------------------------------------------------
    // merge -- structural merge with crystal delta
    // -----------------------------------------------------------------------

    fn cmd_merge(&self, args: &[String]) -> Result<String, CliError> {
        if args.iter().any(|a| a == "--help" || a == "-h") {
            return Ok(Self::merge_help().to_string());
        }

        let ai_mode = args.iter().any(|a| a == "--ai");
        let target = args
            .iter()
            .find(|a| !a.starts_with("--"))
            .cloned()
            .unwrap_or_else(|| "main".to_string());

        let cwd = std::env::current_dir().map_err(CliError::Io)?;
        self.cmd_merge_in(&cwd, &target, ai_mode)
    }

    fn merge_help() -> &'static str {
        "\
merge [target] [--ai] -- structural merge with crystal delta

Compiles the current branch and the target branch, diffs
their crystals structurally, and generates a merge summary.

  --ai    generate merge commit message from crystal delta

Without --ai: prints the structural diff summary.
With --ai: executes git merge with a generated message."
    }

    fn cmd_merge_in(
        &self,
        repo_dir: &std::path::Path,
        target: &str,
        ai_mode: bool,
    ) -> Result<String, CliError> {
        // 1. Get current branch name
        let current = Self::current_branch(repo_dir)?;
        if current == target {
            return Err(CliError::Usage(format!(
                "already on target branch: {}",
                target
            )));
        }

        // 2. Compile current branch's .mirror files
        let current_crystal = self.compile_branch_crystal(repo_dir)?;

        // 3. Compile target branch's .mirror files
        // For now, compile from the same worktree (both see the same files).
        // Full implementation would use git worktrees or checkout.
        let target_crystal = self.compile_branch_crystal(repo_dir)?;

        // 4. Structural diff via fragmentation
        let changes = fragmentation::diff::diff(&current_crystal, &target_crystal);
        let (added, removed, modified, unchanged) = fragmentation::diff::summary(&changes);

        // 5. Generate summary
        let summary = format!(
            "crystal delta: {} → {}\n  added: {}\n  removed: {}\n  modified: {}\n  unchanged: {}",
            current, target, added, removed, modified, unchanged
        );

        if !ai_mode {
            return Ok(summary);
        }

        // 6. --ai mode: generate merge message from delta
        let merge_message = format!(
            "merge {} → {}\n\n{}\n\nstructural delta: +{} -{} ~{} ={}\ncrystal: {}",
            current,
            target,
            summary,
            added,
            removed,
            modified,
            unchanged,
            fragmentation::fragment::content_oid(&current_crystal),
        );

        // 7. Execute git merge
        let status = std::process::Command::new("git")
            .args(["merge", target, "-m", &merge_message])
            .current_dir(repo_dir)
            .status()
            .map_err(CliError::Io)?;

        if !status.success() {
            return Err(CliError::Usage(format!(
                "git merge failed (exit {})",
                status.code().unwrap_or(-1)
            )));
        }

        Ok(format!("merged {} → {}", target, current))
    }

    /// Compile all .mirror files in the repo to produce a single crystal.
    fn compile_branch_crystal(
        &self,
        repo_dir: &std::path::Path,
    ) -> Result<fragmentation::fragment::Fractal<String>, CliError> {
        use fragmentation::encoding;

        // Look for spec.mirror or boot/ directory
        let spec_path = repo_dir.join("spec.mirror");
        if spec_path.exists() {
            let compiled = self.runtime.compile_file(&spec_path)?;
            return Ok(encoding::encode(compiled.crystal().as_str()));
        }

        let boot_dir = repo_dir.join("boot");
        if boot_dir.exists() {
            let store_dir = repo_dir.join(".git").join("mirror");
            let _ = std::fs::create_dir_all(&store_dir);
            let resolution = self.runtime.compile_boot_dir(&boot_dir, &store_dir)?;
            return Ok(encoding::encode(resolution.collapsed.crystal().as_str()));
        }

        // Fallback: encode the repo path itself as a minimal crystal
        Ok(encoding::encode(&format!("crystal:{}", repo_dir.display())))
    }

    // -----------------------------------------------------------------------
    // repl -- interactive prompt (stub)
    // -----------------------------------------------------------------------

    fn cmd_repl(&self, args: &[String]) -> Result<String, CliError> {
        if args.iter().any(|a| a == "--help" || a == "-h") {
            return Ok(Self::command_help("repl").unwrap_or("").to_string());
        }
        Err(CliError::Usage("repl not yet implemented".to_string()))
    }

    // -----------------------------------------------------------------------
    // registry -- inspect boot state
    // -----------------------------------------------------------------------

    fn cmd_registry(&self, args: &[String]) -> Result<String, CliError> {
        let boot_dir_arg = args.first().ok_or_else(|| {
            CliError::Usage("usage: mirror registry <boot-dir> [--store <frgmnt-dir>]".to_string())
        })?;

        let boot_dir = std::path::PathBuf::from(boot_dir_arg);

        // Parse --store flag
        let store_path = args
            .iter()
            .position(|a| a == "--store")
            .and_then(|i| args.get(i + 1))
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                std::env::temp_dir().join(format!("mirror-registry-{}", std::process::id()))
            });

        let boot = self.runtime.compile_boot_dir(&boot_dir, &store_path)?;

        let mut out = String::new();
        out.push_str(&format!("registry {}\n", store_path.display()));
        for (name, compiled) in &boot.resolved {
            out.push_str(&format!("  {} OK {}\n", name, compiled.crystal().as_str()));
        }
        for (name, err) in &boot.failed {
            out.push_str(&format!("  {} FAIL {}\n", name, err));
        }

        // List refs
        let registry = crate::mirror_runtime::MirrorRegistry::open(&store_path)
            .map_err(|e| CliError::Runtime(MirrorRuntimeError(format!("reopen: {}", e))))?;
        for name in registry.ref_names() {
            out.push_str(&format!("  ref {}\n", name));
        }

        Ok(out)
    }

    // -----------------------------------------------------------------------
    // lsp -- language server protocol integration
    // -----------------------------------------------------------------------

    fn cmd_lsp(&self, args: &[String]) -> Result<String, CliError> {
        if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
            return Ok("\
mirror lsp -- language server protocol integration

usage: mirror lsp learn @code/<language> [files...]

subcommands:
  learn @code/<language>   generate a @code grammar from tree-sitter + LSP

options:
  --node-types <path>      path to node-types.json (overrides auto-detection)
  --out <path>             write grammar to <path> instead of garden
  --no-lsp                 skip LSP capability detection"
                .to_string());
        }
        match args[0].as_str() {
            "learn" => self.cmd_lsp_learn(&args[1..]),
            other => Err(CliError::Usage(format!(
                "mirror lsp: unknown subcommand '{}'\nusage: mirror lsp learn @code/<language>",
                other
            ))),
        }
    }

    fn cmd_lsp_learn(&self, args: &[String]) -> Result<String, CliError> {
        use crate::lsp::{generate, language, node_types};
        let mut node_types_path: Option<String> = None;
        let mut out_path: Option<String> = None;
        let mut no_lsp = false;
        let mut positional: Vec<&str> = Vec::new();
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--node-types" => {
                    node_types_path = args.get(i + 1).cloned();
                    i += 2;
                }
                "--out" => {
                    out_path = args.get(i + 1).cloned();
                    i += 2;
                }
                "--no-lsp" => {
                    no_lsp = true;
                    i += 1;
                }
                other => {
                    positional.push(other);
                    i += 1;
                }
            }
        }
        let domain = positional
            .first()
            .ok_or_else(|| CliError::Usage("usage: mirror lsp learn @code/<language>".into()))?;
        let lang_name = domain.strip_prefix("@code/").ok_or_else(|| {
            CliError::Usage(format!("expected @code/<language>, got '{}'", domain))
        })?;
        let config = language::detect(lang_name).ok_or_else(|| {
            CliError::Usage(format!(
                "unknown language '{}'. supported: python, rust, gleam, javascript, typescript, nix",
                lang_name
            ))
        })?;
        let json = match &node_types_path {
            Some(path) => std::fs::read_to_string(path)
                .map_err(|e| CliError::Usage(format!("read {}: {}", path, e)))?,
            None => {
                return Err(CliError::Usage(
                    "auto-detection of node-types.json not yet implemented.\nuse --node-types <path> to provide it manually.".into()
                ));
            }
        };
        let types = node_types::parse_node_types(&json)
            .map_err(|e| CliError::Usage(format!("parse node-types.json: {}", e)))?;
        let capabilities = if no_lsp {
            language::LspCapabilities::default()
        } else {
            language::LspCapabilities::all()
        };
        let grammar = generate::generate_grammar(&config, &types, &capabilities);
        match &out_path {
            Some(path) => {
                if let Some(parent) = std::path::Path::new(path).parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(path, &grammar)?;
                Ok(format!("wrote @code/{} grammar to {}", config.name, path))
            }
            None => {
                let garden_path = format!(
                    ".git/mirror/garden/@code/{}/{}.mirror",
                    config.name, config.name
                );
                let garden_dir = std::path::Path::new(&garden_path).parent().unwrap();
                if std::path::Path::new(".git/mirror").exists() {
                    std::fs::create_dir_all(garden_dir)?;
                    std::fs::write(&garden_path, &grammar)?;
                    Ok(format!(
                        "wrote @code/{} grammar to {}",
                        config.name, garden_path
                    ))
                } else {
                    Ok(grammar)
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // git -- read-only prism over git's ref space
    // -----------------------------------------------------------------------

    fn cmd_git(&self, args: &[String]) -> Result<String, CliError> {
        use crate::git_prism::GitPrism;

        let cwd = std::env::current_dir()?;
        let prism = GitPrism::open(&cwd)
            .map_err(|e| CliError::Usage(format!("not a git repository: {}", e)))?;

        match args.first().map(|s| s.as_str()) {
            Some("refs") => {
                let refs = prism.refs();
                let mut out = String::new();
                for (name, oid) in &refs {
                    out.push_str(&format!("{} {}\n", &oid[..12.min(oid.len())], name));
                }
                Ok(out)
            }
            Some("tree") => {
                let refname = args.get(1).map(|s| s.as_str()).unwrap_or("HEAD");
                let tree = prism.tree_at(refname).map_err(|e| {
                    CliError::Usage(format!("cannot read tree at '{}': {}", refname, e))
                })?;
                let mut out = String::new();
                for entry in &tree {
                    out.push_str(&format!("{} {} {}\n", entry.kind, entry.oid, entry.name));
                }
                Ok(out)
            }
            Some("show") => {
                let spec = args.get(1).ok_or_else(|| {
                    CliError::Usage("usage: mirror git show <ref>:<path>".to_string())
                })?;
                let (refname, path) = spec.split_once(':').ok_or_else(|| {
                    CliError::Usage(
                        "expected <ref>:<path> (e.g., main:boot/00-prism.mirror)".to_string(),
                    )
                })?;
                let content = prism.show(refname, path).map_err(|e| {
                    CliError::Usage(format!("cannot show '{}:{}': {}", refname, path, e))
                })?;
                Ok(content)
            }
            Some("diff") => {
                let a = args.get(1).ok_or_else(|| {
                    CliError::Usage("usage: mirror git diff <ref-a> <ref-b>".to_string())
                })?;
                let b = args.get(2).ok_or_else(|| {
                    CliError::Usage("usage: mirror git diff <ref-a> <ref-b>".to_string())
                })?;
                let diff = prism.diff(a, b).map_err(|e| {
                    CliError::Usage(format!("cannot diff '{}' vs '{}': {}", a, b, e))
                })?;
                if diff.is_empty() {
                    Ok("no differences\n".to_string())
                } else {
                    let mut out = String::new();
                    for entry in &diff {
                        out.push_str(&format!("{} {}\n", entry.status, entry.path));
                    }
                    Ok(out)
                }
            }
            Some("log") => {
                let count = args
                    .get(1)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(20);
                let log = prism
                    .log(count)
                    .map_err(|e| CliError::Usage(format!("cannot read log: {}", e)))?;
                let mut out = String::new();
                for entry in &log {
                    out.push_str(&format!(
                        "{} {} — {}\n",
                        &entry.oid[..12.min(entry.oid.len())],
                        entry.author,
                        entry.message
                    ));
                }
                Ok(out)
            }
            _ => Ok(Self::command_help("git")
                .unwrap_or("mirror git <subcommand>")
                .to_string()),
        }
    }

    // -----------------------------------------------------------------------
    // query -- the fallback
    // -----------------------------------------------------------------------

    fn cmd_query(&self, query: &str, _args: &[String]) -> Result<String, CliError> {
        use crate::ast_prism::ASTPrism;
        use prism::{Beam, Optic, Prism as PrismTrait};

        let prism = ASTPrism;
        let seed = Optic::ok((), query.to_string());
        let focused = prism.focus(seed);
        let projected = prism.project(focused);

        match projected.result().ok() {
            Some(ast) => Ok(format!("{}", ast)),
            None => Err(CliError::Runtime(MirrorRuntimeError(
                "parse failed".to_string(),
            ))),
        }
    }
}

/// Print eigenvalues of a fragment tree (kind, name, params, variants).
fn project_fragment(frag: &crate::declaration::MirrorFragment, depth: usize, out: &mut String) {
    use crate::declaration::{MirrorData, MirrorFragmentExt};
    let data = MirrorData::decode_from_fragment(frag.mirror_data());
    for _ in 0..depth {
        out.push_str("  ");
    }
    out.push_str(data.kind.as_str());
    if !data.name.is_empty() {
        out.push(' ');
        out.push_str(&data.name);
    }
    if !data.params.is_empty() {
        out.push_str(&format!("({})", data.params.join(", ")));
    }
    if !data.variants.is_empty() {
        out.push_str(&format!(" = {}", data.variants.join(" | ")));
    }
    out.push('\n');
    for child in frag.mirror_children() {
        project_fragment(child, depth + 1, out);
    }
}

impl Default for Cli {
    fn default() -> Self {
        Cli {
            runtime: MirrorRuntime::new(),
            crystal_oid: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_default_has_no_crystal() {
        let cli = Cli::default();
        assert!(cli.crystal_oid().is_none());
    }

    #[test]
    fn cli_open_nonexistent_spec_succeeds_with_none() {
        let cli = Cli::open("nonexistent_spec.mirror").unwrap();
        assert!(cli.crystal_oid().is_none());
    }

    #[test]
    fn cli_open_valid_spec_produces_oid() {
        let dir = tempfile::TempDir::new().unwrap();
        let spec = dir.path().join("spec.mirror");
        std::fs::write(
            &spec,
            "grammar @test {\n  type greeting\n  type farewell\n  action speak(self)\n}\n",
        )
        .unwrap();
        let cli = Cli::open(spec.to_str().unwrap()).unwrap();
        let oid = cli.crystal_oid().expect("should produce crystal OID");
        assert!(
            oid.as_str().len() == 64 && oid.as_str().chars().all(|c| c.is_ascii_hexdigit()),
            "OID should be 64-char hex, got: {}",
            oid.as_str()
        );
    }

    #[test]
    fn dispatch_compile_valid_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let file = dir.path().join("test.mirror");
        std::fs::write(
            &file,
            "grammar @deploy {\n  type state\n  action transform(self) in @code/rust {\n    self.apply()\n  }\n}\n",
        )
        .unwrap();

        let cli = Cli::default();
        let result = cli.dispatch("compile", &[file.to_str().unwrap().to_string()]);
        assert!(result.is_ok());
        let oid = result.unwrap();
        assert!(
            oid.len() == 64 && oid.chars().all(|c| c.is_ascii_hexdigit()),
            "should be hex OID: {}",
            oid
        );
        // Compile again: deterministic
        let oid2 = cli
            .dispatch("compile", &[file.to_str().unwrap().to_string()])
            .unwrap();
        assert_eq!(oid, oid2, "compile must be deterministic");
    }

    #[test]
    fn dispatch_compile_no_args_is_usage_error() {
        let cli = Cli::default();
        let result = cli.dispatch("compile", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn dispatch_query_fallback() {
        let cli = Cli::default();
        let result = cli.dispatch("type greeting", &[]);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(
            output.contains("greeting"),
            "query output should contain the type name 'greeting', got: {}",
            output
        );
        // Also test a more complex query
        let result2 = cli.dispatch("grammar @test { type id }", &[]);
        assert!(result2.is_ok());
        let output2 = result2.unwrap();
        assert!(
            output2.contains("@test"),
            "complex query should parse and return form name, got: {}",
            output2
        );
    }

    #[test]
    fn dispatch_ai_no_model_is_usage_error() {
        let cli = Cli::default();
        let result = cli.dispatch("ai", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn dispatch_ai_known_model() {
        let cli = Cli::default();
        let result = cli.dispatch("ai", &["abyss".to_string()]);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("abyss"));
    }

    #[test]
    fn dispatch_crystal_oid_flag_without_crystal() {
        let cli = Cli::default();
        let result = cli.dispatch("crystal", &["--oid".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn dispatch_crystal_oid_flag_with_crystal() {
        let dir = tempfile::TempDir::new().unwrap();
        let spec = dir.path().join("spec.mirror");
        std::fs::write(&spec, "type greeting\n").unwrap();
        let cli = Cli::open(spec.to_str().unwrap()).unwrap();

        let result = cli.dispatch("crystal", &["--oid".to_string()]);
        assert!(result.is_ok());
        let oid = result.unwrap();
        assert!(
            oid.len() == 64 && oid.chars().all(|c| c.is_ascii_hexdigit()),
            "crystal --oid should print hex OID: {}",
            oid
        );
    }

    #[test]
    fn cli_error_display() {
        let e = CliError::Usage("test".to_string());
        assert_eq!(format!("{}", e), "test");

        let e = CliError::Runtime(MirrorRuntimeError("rt error".to_string()));
        assert_eq!(format!("{}", e), "rt error");

        let e = CliError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "nope"));
        assert!(format!("{}", e).contains("nope"));
    }

    #[test]
    fn cli_error_is_error() {
        let e: Box<dyn std::error::Error> = Box::new(CliError::Usage("test".to_string()));
        assert_eq!(format!("{}", e), "test");
    }

    #[test]
    fn cli_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "nope");
        let cli_err: CliError = io_err.into();
        assert!(matches!(cli_err, CliError::Io(_)));
    }

    #[test]
    fn cli_error_from_runtime() {
        let rt_err = MirrorRuntimeError("broken".to_string());
        let cli_err: CliError = rt_err.into();
        assert!(matches!(cli_err, CliError::Runtime(_)));
    }

    /// The meta-property: a spec compiled twice yields the same crystal OID.
    #[test]
    fn binary_is_its_own_spec() {
        let dir = tempfile::TempDir::new().unwrap();
        let spec = dir.path().join("spec.mirror");
        std::fs::write(&spec, "type identity\ntype mirror\n").unwrap();

        let cli1 = Cli::open(spec.to_str().unwrap()).unwrap();
        let cli2 = Cli::open(spec.to_str().unwrap()).unwrap();

        assert_eq!(
            cli1.crystal_oid().unwrap().as_str(),
            cli2.crystal_oid().unwrap().as_str(),
            "two compilations of the same spec must produce the same crystal OID"
        );
    }

    #[test]
    #[ignore = "requires Store wiring (Tick 2)"]
    fn running_crystal_matches_compiled_shatter() {
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let boot_dir = manifest_dir.join("boot");
        let store_dir = std::env::temp_dir().join("mirror-meta-test");
        let _ = std::fs::create_dir_all(&store_dir);
        let shatter = std::env::temp_dir().join("mirror-meta-test.shatter");

        let runtime = MirrorRuntime::new();
        let crystal_oid = runtime
            .materialize_crystal(&boot_dir, &store_dir, &shatter)
            .expect("materialize crystal");

        let compiled = runtime.compile_file(&shatter).expect("compile shatter");

        let _ = std::fs::remove_dir_all(&store_dir);
        let _ = std::fs::remove_file(&shatter);

        assert_eq!(
            crystal_oid.as_str(),
            compiled.crystal().as_str(),
            "crystal OID from materialize must equal OID from compiling the shatter"
        );
    }

    // -----------------------------------------------------------------------
    // Help tests
    // -----------------------------------------------------------------------

    #[test]
    fn dispatch_help_returns_help_text() {
        let cli = Cli::default();
        let result = cli.dispatch("help", &[]);
        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(text.contains("mirror -- an honest compiler"));
        assert!(text.contains("focus"));
        assert!(text.contains("compile"));
        assert!(text.contains("ci"));
        assert!(text.contains("ai"));
    }

    #[test]
    fn dispatch_dashdash_help_returns_help_text() {
        let cli = Cli::default();
        let result = cli.dispatch("--help", &[]);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("mirror -- an honest compiler"));
    }

    #[test]
    fn command_help_returns_per_command_help() {
        for cmd in [
            "focus", "project", "split", "zoom", "refract", "compile", "crystal", "ci", "ca",
            "init", "repl", "ai", "bench", "verify",
        ] {
            assert!(
                Cli::command_help(cmd).is_some(),
                "command '{}' should have help text",
                cmd
            );
        }
        assert!(Cli::command_help("nonexistent").is_none());
    }

    // -----------------------------------------------------------------------
    // Optic command tests
    // -----------------------------------------------------------------------

    #[test]
    fn dispatch_focus_on_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let file = dir.path().join("test.mirror");
        std::fs::write(&file, "grammar @test {\n  type id\n  type name\n}\n").unwrap();

        let cli = Cli::default();
        let result = cli.dispatch("focus", &[file.to_str().unwrap().to_string()]);
        assert!(result.is_ok(), "focus should succeed: {:?}", result.err());
        let output = result.unwrap();
        assert!(output.contains("grammar"));
        assert!(output.contains("@test"));
    }

    #[test]
    fn dispatch_refract_on_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let file = dir.path().join("test.mirror");
        std::fs::write(&file, "type greeting\n").unwrap();

        let cli = Cli::default();
        let result = cli.dispatch("refract", &[file.to_str().unwrap().to_string()]);
        assert!(result.is_ok());
        let oid = result.unwrap();
        assert!(
            oid.len() == 64 && oid.chars().all(|c| c.is_ascii_hexdigit()),
            "refract should return OID, got: {}",
            oid
        );
    }

    #[test]
    fn dispatch_project_on_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let file = dir.path().join("test.mirror");
        std::fs::write(&file, "grammar @test {\n  type id\n  type name\n}\n").unwrap();

        let cli = Cli::default();
        let result = cli.dispatch("project", &[file.to_str().unwrap().to_string()]);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("type id"));
        assert!(output.contains("type name"));
    }

    #[test]
    fn dispatch_optic_no_args_is_usage_error() {
        let cli = Cli::default();
        for cmd in ["focus", "project", "split", "zoom", "refract"] {
            let result = cli.dispatch(cmd, &[]);
            assert!(
                result.is_err(),
                "{} with no args should be usage error",
                cmd
            );
        }
    }

    #[test]
    fn dispatch_optic_help_flag() {
        let cli = Cli::default();
        for cmd in ["focus", "project", "split", "zoom", "refract"] {
            let result = cli.dispatch(cmd, &["--help".to_string()]);
            assert!(result.is_ok(), "{} --help should succeed", cmd);
        }
    }

    // -----------------------------------------------------------------------
    // CI and bench command tests
    // -----------------------------------------------------------------------

    #[test]
    fn dispatch_ci_on_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let file = dir.path().join("test.mirror");
        std::fs::write(&file, "type greeting\n").unwrap();

        let cli = Cli::default();
        let result = cli.dispatch("ci", &[file.to_str().unwrap().to_string()]);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("holonomy"));
    }

    #[test]
    fn dispatch_bench_on_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let file = dir.path().join("test.mirror");
        std::fs::write(&file, "type greeting\n").unwrap();

        let cli = Cli::default();
        let result = cli.dispatch("bench", &[file.to_str().unwrap().to_string()]);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("compiled"));
        assert!(output.contains("oid:"));
    }

    // -----------------------------------------------------------------------
    // CI holonomy tests
    // -----------------------------------------------------------------------

    #[test]
    fn ci_on_simple_file_reports_holonomy_value() {
        let dir = tempfile::TempDir::new().unwrap();
        let file = dir.path().join("test.mirror");
        std::fs::write(&file, "type greeting\n").unwrap();
        let cli = Cli::default();
        let result = cli.dispatch("ci", &[file.to_str().unwrap().to_string()]);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(
            output.contains("holonomy:"),
            "should report holonomy value, got: {}",
            output
        );
    }

    #[test]
    fn ci_on_long_file_reports_nonzero_holonomy() {
        let dir = tempfile::TempDir::new().unwrap();
        let file = dir.path().join("test.mirror");
        let block = "form @test {\n  prism focus\n  prism split\n  prism zoom\n  prism project\n  prism refract\n}\n";
        let source = block.repeat(20);
        std::fs::write(&file, &source).unwrap();
        let cli = Cli::default();
        let result = cli.dispatch("ci", &[file.to_str().unwrap().to_string()]);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(
            output.contains("partial"),
            "long source should report partial, got: {}",
            output
        );
        assert!(
            output.contains("holonomy:"),
            "should contain holonomy value, got: {}",
            output
        );
    }

    #[test]
    fn ci_on_directory_aggregates() {
        let dir = tempfile::TempDir::new().unwrap();
        std::fs::write(dir.path().join("a.mirror"), "type alpha\n").unwrap();
        std::fs::write(dir.path().join("b.mirror"), "type beta\n").unwrap();
        std::fs::write(dir.path().join("c.txt"), "not mirror\n").unwrap();
        let cli = Cli::default();
        let result = cli.dispatch("ci", &[dir.path().to_str().unwrap().to_string()]);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(
            output.contains("2 files"),
            "should report 2 files, got: {}",
            output
        );
        assert!(
            output.contains("holonomy:"),
            "should contain holonomy, got: {}",
            output
        );
    }

    #[test]
    fn ci_empty_dir_is_usage_error() {
        let dir = tempfile::TempDir::new().unwrap();
        let cli = Cli::default();
        let result = cli.dispatch("ci", &[dir.path().to_str().unwrap().to_string()]);
        assert!(result.is_err(), "ci on empty dir should fail");
    }

    // -----------------------------------------------------------------------
    // CA tests
    // -----------------------------------------------------------------------

    #[test]
    fn ca_on_crystal_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let file = dir.path().join("test.mirror");
        // Empty file: transport returns Success with zero loss
        std::fs::write(&file, "").unwrap();
        let cli = Cli::default();
        let result = cli.dispatch("ca", &[file.to_str().unwrap().to_string()]);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(
            output.contains("nothing to do"),
            "crystal file should say nothing to do, got: {}",
            output
        );
    }

    #[test]
    fn ca_on_file_with_holonomy_produces_suggestions() {
        let dir = tempfile::TempDir::new().unwrap();
        let file = dir.path().join("test.mirror");
        let block = "form @test {\n  prism focus\n  prism split\n  prism zoom\n  prism project\n  prism refract\n}\n";
        let source = block.repeat(20);
        std::fs::write(&file, &source).unwrap();
        let cli = Cli::default();
        let result = cli.dispatch("ca", &[file.to_str().unwrap().to_string()]);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(
            output.contains("suggestions"),
            "file with holonomy should produce suggestions, got: {}",
            output
        );
    }

    #[test]
    fn ca_enforce_flag_prints_stub() {
        let dir = tempfile::TempDir::new().unwrap();
        let file = dir.path().join("test.mirror");
        let block = "form @test {\n  prism focus\n  prism split\n  prism zoom\n  prism project\n  prism refract\n}\n";
        let source = block.repeat(20);
        std::fs::write(&file, &source).unwrap();
        let cli = Cli::default();
        let result = cli.dispatch(
            "ca",
            &[file.to_str().unwrap().to_string(), "--enforce".to_string()],
        );
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(
            output.contains("enforce: not yet implemented"),
            "enforce flag should report stub, got: {}",
            output
        );
    }

    #[test]
    fn ca_no_args_is_usage_error() {
        let cli = Cli::default();
        let result = cli.dispatch("ca", &[]);
        assert!(result.is_err());
    }

    // -----------------------------------------------------------------------
    // Sign and verify tests
    // -----------------------------------------------------------------------

    #[test]
    fn dispatch_compile_writes_shatter() {
        let dir = tempfile::TempDir::new().unwrap();
        let file = dir.path().join("test.mirror");
        std::fs::write(&file, "type greeting\n").unwrap();

        let cli = Cli::default();
        let result = cli.dispatch("compile", &[file.to_str().unwrap().to_string()]);
        assert!(result.is_ok());
        let shatter = dir.path().join("test.shatter");
        assert!(shatter.exists(), ".shatter should be written");
        let sig = dir.path().join("test.shatter.sig");
        assert!(
            !sig.exists(),
            ".shatter.sig should not exist without --sign"
        );
    }

    /// Sign+verify round trip using direct API (no env vars = no races).
    /// Tests the full flow: sign content, verify untampered, fail on tampered.
    #[cfg(feature = "git")]
    #[test]
    fn sign_verify_round_trip_direct_api() {
        let key = ssh_key::PrivateKey::random(
            &mut ssh_key::rand_core::OsRng,
            ssh_key::Algorithm::Ed25519,
        )
        .unwrap();

        // Compile some content
        let source = "type greeting\n";
        let mut compiler = crate::bundle::MirrorCompiler::new();
        let compiled = compiler.compile(source).expect("compile should succeed");
        let crystal_oid = compiled.crystal().clone();

        // Sign the content OID
        let content_oid = crate::Oid::hash(source.as_bytes());
        let sig_pem = crate::sign::sign_oid(&key, &content_oid).expect("sign should succeed");
        assert!(sig_pem.contains("BEGIN SSH SIGNATURE"));

        // Write .shatter and .shatter.sig
        let dir = tempfile::TempDir::new().unwrap();
        let shatter = dir.path().join("test.shatter");
        let sig_file = dir.path().join("test.shatter.sig");
        std::fs::write(&shatter, source).unwrap();
        std::fs::write(&sig_file, &sig_pem).unwrap();

        // Verify untampered: succeeds
        let loaded_sig = std::fs::read_to_string(&sig_file).unwrap();
        crate::sign::verify_oid(&key.public_key().clone(), &content_oid, &loaded_sig)
            .expect("untampered verify should succeed");

        // Tamper with the .shatter
        let tampered = "type TAMPERED\n";
        std::fs::write(&shatter, tampered).unwrap();
        let tampered_oid = crate::Oid::hash(tampered.as_bytes());

        // Verify tampered: fails
        let result = crate::sign::verify_oid(&key.public_key().clone(), &tampered_oid, &loaded_sig);
        assert!(result.is_err(), "tampered content must fail verification");
        assert!(
            result.unwrap_err().message.contains("verification failed"),
            "should indicate verification failure"
        );

        // Wrong key: also fails
        let other_key = ssh_key::PrivateKey::random(
            &mut ssh_key::rand_core::OsRng,
            ssh_key::Algorithm::Ed25519,
        )
        .unwrap();
        let result =
            crate::sign::verify_oid(&other_key.public_key().clone(), &content_oid, &loaded_sig);
        assert!(result.is_err(), "wrong key must fail verification");

        // Crystal OID is deterministic
        let mut compiler2 = crate::bundle::MirrorCompiler::new();
        let compiled2 = compiler2.compile(source).unwrap();
        assert_eq!(
            crystal_oid.as_str(),
            compiled2.crystal().as_str(),
            "compilation must be deterministic"
        );
    }

    /// Test that compile --sign produces the expected files (env-dependent).
    /// Grouped with other env-dependent tests in sign::tests to minimize races.
    /// This test only checks file production, not verification via CLI dispatch.
    #[cfg(feature = "git")]
    #[test]
    fn compile_sign_produces_files() {
        // Generate test keypair and write to a temp dir
        let key_dir = tempfile::TempDir::new().unwrap();
        let key = ssh_key::PrivateKey::random(
            &mut ssh_key::rand_core::OsRng,
            ssh_key::Algorithm::Ed25519,
        )
        .unwrap();
        let pem = key.to_openssh(ssh_key::LineEnding::LF).unwrap();
        let pem_bytes: &[u8] = pem.as_ref();
        let priv_path = key_dir.path().join("id_ed25519");
        std::fs::write(&priv_path, pem_bytes).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&priv_path, std::fs::Permissions::from_mode(0o600)).unwrap();
        }

        // Set MIRROR_CI_SIGN_KEY to the private key file path (most specific env var)
        std::env::set_var("MIRROR_CI_SIGN_KEY", priv_path.as_os_str());

        let dir = tempfile::TempDir::new().unwrap();
        let file = dir.path().join("test.mirror");
        std::fs::write(&file, "type greeting\n").unwrap();

        let cli = Cli::default();
        let result = cli.dispatch(
            "compile",
            &[file.to_str().unwrap().to_string(), "--sign".to_string()],
        );
        assert!(result.is_ok(), "compile --sign failed: {:?}", result.err());

        let shatter = dir.path().join("test.shatter");
        let sig = dir.path().join("test.shatter.sig");
        assert!(shatter.exists(), ".shatter should exist");
        assert!(sig.exists(), ".shatter.sig should exist after --sign");

        // Verify the sig file contains a valid SSH signature PEM
        let sig_content = std::fs::read_to_string(&sig).unwrap();
        assert!(
            sig_content.contains("BEGIN SSH SIGNATURE"),
            "sig file should contain SSH signature PEM"
        );

        std::env::remove_var("MIRROR_CI_SIGN_KEY");
    }

    #[test]
    fn dispatch_verify_no_args_is_usage_error() {
        let cli = Cli::default();
        let result = cli.dispatch("verify", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn ca_help_flag() {
        let cli = Cli::default();
        let result = cli.dispatch("ca", &["--help".to_string()]);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(
            output.contains("observe, suggest, enforce"),
            "ca --help should describe the command, got: {}",
            output
        );
    }

    #[test]
    fn dispatch_verify_help_flag() {
        let cli = Cli::default();
        let result = cli.dispatch("verify", &["--help".to_string()]);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("verify"));
    }

    #[test]
    fn dispatch_verify_missing_sig_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let file = dir.path().join("test.shatter");
        std::fs::write(&file, "type greeting\n").unwrap();

        let cli = Cli::default();
        let result = cli.dispatch("verify", &[file.to_str().unwrap().to_string()]);
        assert!(result.is_err());
        let err = format!("{}", result.unwrap_err());
        assert!(
            err.contains("signature file not found") || err.contains("requires the"),
            "should report missing sig or missing feature, got: {}",
            err
        );
    }

    #[test]
    fn help_text_includes_ca() {
        let text = Cli::help_text();
        assert!(
            text.contains("ca"),
            "help text should include ca command, got: {}",
            text
        );
    }

    #[test]
    fn help_text_includes_verify_and_sign() {
        let text = Cli::help_text();
        assert!(text.contains("verify"), "help should mention verify");
        assert!(text.contains("--sign"), "help should mention --sign");
    }

    // -----------------------------------------------------------------------
    // ca --merge tests
    // -----------------------------------------------------------------------

    #[test]
    fn ca_merge_flag_detected() {
        // Verify --merge routes to cmd_ca_merge (not cmd_ca).
        // cmd_ca without a path arg gives "usage: mirror ca <path>".
        // cmd_ca_merge without being on main gives "must be run from main".
        // If we get the ca_merge error, the flag routing works.
        let cli = Cli::default();
        let result = cli.dispatch("ca", &["--merge".to_string()]);
        // We're not on main in tests (or we are — either way the output
        // should NOT be the ca usage error about <path>).
        match &result {
            Ok(msg) => {
                // If on main with no branches: "nothing to merge"
                assert!(
                    msg.contains("nothing to merge") || msg.contains("ca --merge"),
                    "should route to ca_merge, got: {}",
                    msg
                );
            }
            Err(e) => {
                let err = format!("{}", e);
                // ca_merge error: "must be run from main"
                // ca error: "usage: mirror ca <path>"
                assert!(
                    err.contains("must be run from main"),
                    "should route to ca_merge, got ca error: {}",
                    err
                );
            }
        }
    }

    /// Helper: create a temp git repo with main branch and initial commit.
    fn make_test_repo() -> tempfile::TempDir {
        let dir = tempfile::TempDir::new().unwrap();
        let p = dir.path();
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(p)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["checkout", "-b", "main"])
            .current_dir(p)
            .output()
            .unwrap();
        // Set local git config so commits work
        std::process::Command::new("git")
            .args(["config", "user.email", "test@test"])
            .current_dir(p)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(p)
            .output()
            .unwrap();
        std::fs::write(p.join("file.txt"), "hello").unwrap();
        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(p)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["commit", "--no-verify", "--no-gpg-sign", "-m", "init"])
            .current_dir(p)
            .output()
            .unwrap();
        dir
    }

    #[test]
    fn ca_merge_list_branches_finds_feature_branch() {
        let dir = make_test_repo();
        let p = dir.path();

        // Create a feature branch with a commit
        std::process::Command::new("git")
            .args(["checkout", "-b", "feature-a"])
            .current_dir(p)
            .output()
            .unwrap();
        std::fs::write(p.join("feature.txt"), "feature").unwrap();
        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(p)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["commit", "--no-verify", "--no-gpg-sign", "-m", "feature"])
            .current_dir(p)
            .output()
            .unwrap();

        // Back to main
        std::process::Command::new("git")
            .args(["checkout", "main"])
            .current_dir(p)
            .output()
            .unwrap();

        // list_branches should find feature-a
        let branches = Cli::list_branches(p).unwrap();
        assert!(
            branches.contains(&"feature-a".to_string()),
            "should find feature-a branch, got: {:?}",
            branches
        );
    }

    #[test]
    fn ca_merge_on_clean_repo_reports_nothing() {
        let dir = make_test_repo();
        let cli = Cli::default();
        let result = cli.cmd_ca_merge_in(dir.path());
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "nothing to merge",
            "repo with only main should report nothing to merge"
        );
    }

    #[test]
    fn ca_merge_not_on_main_is_error() {
        let dir = make_test_repo();
        let p = dir.path();
        // Switch to a non-main branch
        std::process::Command::new("git")
            .args(["checkout", "-b", "other"])
            .current_dir(p)
            .output()
            .unwrap();

        let cli = Cli::default();
        let result = cli.cmd_ca_merge_in(p);
        assert!(result.is_err());
        let err = format!("{}", result.unwrap_err());
        assert!(
            err.contains("must be run from main"),
            "should require main, got: {}",
            err
        );
    }

    #[test]
    fn ca_merge_commits_ahead_counts_correctly() {
        let dir = make_test_repo();
        let p = dir.path();

        // Create feature branch with 2 commits
        std::process::Command::new("git")
            .args(["checkout", "-b", "feat"])
            .current_dir(p)
            .output()
            .unwrap();
        for i in 0..2 {
            std::fs::write(p.join(format!("f{}.txt", i)), format!("{}", i)).unwrap();
            std::process::Command::new("git")
                .args(["add", "."])
                .current_dir(p)
                .output()
                .unwrap();
            std::process::Command::new("git")
                .args([
                    "commit",
                    "--no-verify",
                    "--no-gpg-sign",
                    "-m",
                    &format!("feat {}", i),
                ])
                .current_dir(p)
                .output()
                .unwrap();
        }

        std::process::Command::new("git")
            .args(["checkout", "main"])
            .current_dir(p)
            .output()
            .unwrap();

        let ahead = Cli::commits_ahead(p, "feat").unwrap();
        assert_eq!(ahead, 2, "feat should be 2 commits ahead of main");
    }

    #[test]
    fn ca_merge_analyze_detects_clean_branch() {
        let dir = make_test_repo();
        let p = dir.path();

        // Create a clean feature branch
        std::process::Command::new("git")
            .args(["checkout", "-b", "clean-feat"])
            .current_dir(p)
            .output()
            .unwrap();
        std::fs::write(p.join("new.txt"), "new").unwrap();
        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(p)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["commit", "--no-verify", "--no-gpg-sign", "-m", "add new"])
            .current_dir(p)
            .output()
            .unwrap();

        std::process::Command::new("git")
            .args(["checkout", "main"])
            .current_dir(p)
            .output()
            .unwrap();

        let info = Cli::analyze_branch(p, "clean-feat").unwrap();
        assert_eq!(info.name, "clean-feat");
        assert_eq!(info.commits_ahead, 1);
        assert!(
            !info.has_conflicts,
            "clean branch should not have conflicts"
        );
    }

    #[test]
    fn ca_merge_analyze_detects_conflicting_branch() {
        let dir = make_test_repo();
        let p = dir.path();

        // Modify file.txt on a branch
        std::process::Command::new("git")
            .args(["checkout", "-b", "conflict-feat"])
            .current_dir(p)
            .output()
            .unwrap();
        std::fs::write(p.join("file.txt"), "branch version").unwrap();
        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(p)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args([
                "commit",
                "--no-verify",
                "--no-gpg-sign",
                "-m",
                "branch change",
            ])
            .current_dir(p)
            .output()
            .unwrap();

        // Modify same file on main
        std::process::Command::new("git")
            .args(["checkout", "main"])
            .current_dir(p)
            .output()
            .unwrap();
        std::fs::write(p.join("file.txt"), "main version").unwrap();
        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(p)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args([
                "commit",
                "--no-verify",
                "--no-gpg-sign",
                "-m",
                "main change",
            ])
            .current_dir(p)
            .output()
            .unwrap();

        let info = Cli::analyze_branch(p, "conflict-feat").unwrap();
        assert!(info.has_conflicts, "should detect merge conflict");
    }

    #[test]
    fn ca_merge_sorts_by_readiness() {
        // Verify sort order: no-conflict first, then by commits ahead
        let mut infos = vec![
            BranchInfo {
                name: "conflict".to_string(),
                commits_ahead: 1,
                has_conflicts: true,
            },
            BranchInfo {
                name: "big-clean".to_string(),
                commits_ahead: 5,
                has_conflicts: false,
            },
            BranchInfo {
                name: "small-clean".to_string(),
                commits_ahead: 1,
                has_conflicts: false,
            },
        ];

        infos.sort_by(|a, b| {
            a.has_conflicts
                .cmp(&b.has_conflicts)
                .then(a.commits_ahead.cmp(&b.commits_ahead))
        });

        assert_eq!(infos[0].name, "small-clean");
        assert_eq!(infos[1].name, "big-clean");
        assert_eq!(infos[2].name, "conflict");
    }

    // -----------------------------------------------------------------------
    // --strict flag
    // -----------------------------------------------------------------------

    /// --strict on compile turns Partial into Failure (exit code 1).
    #[test]
    fn strict_flag_rejects_partial() {
        let dir = std::env::temp_dir().join(format!("mirror-strict-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let file = dir.join("test.mirror");
        // Source with unrecognized keyword → Partial
        std::fs::write(&file, "widget foo\ntype bar\n").unwrap();

        let cli = Cli::open("/nonexistent/spec.mirror").unwrap();
        let result = cli.dispatch(
            "compile",
            &[file.to_string_lossy().to_string(), "--strict".to_string()],
        );
        assert!(result.is_err(), "--strict must reject Partial compilation");
        let _ = std::fs::remove_dir_all(&dir);
    }

    // -----------------------------------------------------------------------
    // kintsugi command
    // -----------------------------------------------------------------------

    /// kintsugi command reorders declarations to canonical order.
    #[test]
    fn kintsugi_command_reorders() {
        let dir = std::env::temp_dir().join(format!("mirror-kintsugi-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let file = dir.join("test.mirror");
        std::fs::write(&file, "type x\nin @prism\naction do_thing\n").unwrap();

        let cli = Cli::open("/nonexistent/spec.mirror").unwrap();
        let result = cli.dispatch("kintsugi", &[file.to_string_lossy().to_string()]);
        assert!(result.is_ok(), "kintsugi command must succeed");
        let output = result.unwrap();
        // in @prism must come before type x in the output
        let in_pos = output.find("in @prism");
        let type_pos = output.find("type x");
        assert!(
            in_pos.is_some() && type_pos.is_some(),
            "output must contain both declarations"
        );
        assert!(
            in_pos.unwrap() < type_pos.unwrap(),
            "in @prism must come before type x in canonical order"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// kintsugi --check passes for already-canonical source.
    #[test]
    fn kintsugi_check_passes_canonical() {
        let dir = std::env::temp_dir().join(format!("mirror-kintsugi-chk-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let file = dir.join("test.mirror");
        // Already in canonical order: in, type, action
        std::fs::write(&file, "in @prism\ntype x\naction do_thing\n").unwrap();

        let cli = Cli::open("/nonexistent/spec.mirror").unwrap();
        let result = cli.dispatch(
            "kintsugi",
            &[file.to_string_lossy().to_string(), "--check".to_string()],
        );
        assert!(
            result.is_ok(),
            "kintsugi --check must pass for canonical source"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// kintsugi --check fails for non-canonical source.
    #[test]
    fn kintsugi_check_fails_non_canonical() {
        let dir = std::env::temp_dir().join(format!("mirror-kintsugi-chk2-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let file = dir.join("test.mirror");
        // Not canonical: type before in
        std::fs::write(&file, "type x\nin @prism\n").unwrap();

        let cli = Cli::open("/nonexistent/spec.mirror").unwrap();
        let result = cli.dispatch(
            "kintsugi",
            &[file.to_string_lossy().to_string(), "--check".to_string()],
        );
        assert!(
            result.is_err(),
            "kintsugi --check must fail for non-canonical source"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    // -- merge tests --

    #[test]
    fn merge_help() {
        let cli = Cli::default();
        let result = cli.dispatch("merge", &["--help".to_string()]);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("crystal delta"));
    }

    #[test]
    fn merge_same_branch_is_error() {
        let dir = tempfile::TempDir::new().unwrap();
        let repo = git2::Repository::init(dir.path()).unwrap();

        // Create an initial commit so HEAD is valid
        let tree_oid = {
            let mut index = repo.index().unwrap();
            index.write_tree().unwrap()
        };
        let tree = repo.find_tree(tree_oid).unwrap();
        let sig = git2::Signature::now("test", "test@test.com").unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "initial", &tree, &[])
            .unwrap();

        let cli = Cli::default();
        // On "main" trying to merge "main" — should fail
        let result = cli.cmd_merge_in(dir.path(), "main", false);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("already on target"), "got: {}", err);
    }

    #[test]
    fn merge_no_ai_produces_summary() {
        let dir = tempfile::TempDir::new().unwrap();
        let repo = git2::Repository::init(dir.path()).unwrap();

        // Create an initial commit on main
        let tree_oid = {
            let mut index = repo.index().unwrap();
            index.write_tree().unwrap()
        };
        let tree = repo.find_tree(tree_oid).unwrap();
        let sig = git2::Signature::now("test", "test@test.com").unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "initial", &tree, &[])
            .unwrap();

        // Create a feature branch
        let head_commit = repo.head().unwrap().peel_to_commit().unwrap();
        repo.branch("feature", &head_commit, false).unwrap();

        let cli = Cli::default();
        let result = cli.cmd_merge_in(dir.path(), "feature", false);
        assert!(result.is_ok(), "merge should succeed: {:?}", result);
        let output = result.unwrap();
        assert!(
            output.contains("crystal delta"),
            "should contain crystal delta: {}",
            output
        );
        assert!(
            output.contains("unchanged"),
            "should contain unchanged: {}",
            output
        );
    }

    #[test]
    fn merge_command_help_in_dispatch() {
        let cli = Cli::default();
        let help = Cli::command_help("merge");
        assert!(help.is_some());
        assert!(help.unwrap().contains("crystal delta"));
    }
}
