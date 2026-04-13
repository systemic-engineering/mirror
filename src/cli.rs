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
            "bench" => self.cmd_bench(args),
            "verify" => self.cmd_verify(args),
            "init" => self.cmd_init(args),
            "repl" => self.cmd_repl(args),
            "focus" | "project" | "split" | "zoom" | "refract" => self.cmd_optic(command, args),
            "registry" => self.cmd_registry(args),
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
  verify <file>      verify signed .shatter

session:
  init               initialize .git/mirror/
  repl               interactive shard> prompt

tools:
  ai <model> [path]  run a Fate model
  bench <path>       benchmark compilation

flags:
  --oid              print content address only
  --sign             sign compilation output (compile)
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
            "compile" => Some("compile <path> [--sign] -- compile a .mirror file\n\nParses, resolves, and content-addresses the source.\nPrints the crystal OID to stdout.\nWith --sign: produces .shatter.sig alongside .shatter."),
            "crystal" => Some("crystal [output] -- materialize the standard library\n\nCompiles boot/ in order and emits mirror.shatter.\nWith --oid: prints the loaded crystal OID."),
            "ci" => Some("ci <path> -- measure holonomy\n\nCompiles and reports the MirrorLoss.\nZero holonomy means crystal. Nonzero means alive."),
            "init" => Some("init -- initialize .git/mirror/\n\nSets up the mirror store in the current git repository."),
            "repl" => Some("repl -- interactive shard> prompt\n\nStarts an interactive session.\nType .mirror expressions and see them compiled live."),
            "ai" => Some("ai <model> [path] -- run a Fate model\n\nModels: abyss | introject | cartographer | explorer | fate\n\nReads from <path> or stdin. Routes through the named model."),
            "bench" => Some("bench <path> -- benchmark compilation\n\nMeasures compilation time and structural loss.\nPrints timing and MirrorLoss summary."),
            "verify" => Some("verify <file> -- verify a signed .shatter file\n\nChecks the Ed25519 signature (.shatter.sig) against the content.\nUses the public key from CONVERSATION_KEYS hierarchy.\nExits 0 if valid, nonzero if tampered or unsigned."),
            _ => None,
        }
    }

    // -----------------------------------------------------------------------
    // compile
    // -----------------------------------------------------------------------

    fn cmd_compile(&self, args: &[String]) -> Result<String, CliError> {
        let sign = args.iter().any(|a| a == "--sign");
        let file_args: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
        let file = file_args
            .first()
            .ok_or_else(|| CliError::Usage("usage: mirror compile <file> [--sign]".to_string()))?;

        let source = std::fs::read_to_string(file.as_str())?;
        let mut compiler = crate::bundle::MirrorCompiler::new();
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

                eprintln!(
                    "compiled {} -> {} (rank {}, {:?})",
                    file,
                    oid.as_str(),
                    shard.rank(),
                    shard.target,
                );

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
        let compiled = self.runtime.compile_source(&source)?;

        match optic {
            "focus" => {
                let text = crate::mirror_runtime::emit_form(&compiled.form);
                Ok(text)
            }
            "project" => {
                let mut out = String::new();
                project_form(&compiled.form, 0, &mut out);
                Ok(out)
            }
            "refract" => Ok(compiled.crystal().as_str().to_string()),
            _ => {
                // split, zoom -- same as focus for now
                let text = crate::mirror_runtime::emit_form(&compiled.form);
                Ok(text)
            }
        }
    }

    // -----------------------------------------------------------------------
    // ci -- measure holonomy
    // -----------------------------------------------------------------------

    fn cmd_ci(&self, args: &[String]) -> Result<String, CliError> {
        if args.iter().any(|a| a == "--help" || a == "-h") {
            return Ok(Self::command_help("ci").unwrap_or("").to_string());
        }
        let file = args
            .first()
            .ok_or_else(|| CliError::Usage("usage: mirror ci <path>".to_string()))?;

        let source = std::fs::read_to_string(file)?;
        let compiler = crate::bundle::MirrorCompiler::new();
        use prism::Transport;
        let result = compiler.transport(&source);
        match result {
            prism::Imperfect::Success(oid) => Ok(format!("crystal {}\nholonomy: 0", oid)),
            prism::Imperfect::Partial(oid, loss) => {
                let phase_count = loss.phases.len();
                let structural: f64 = loss.phases.iter().map(|p| p.structural_loss).sum();
                Ok(format!(
                    "partial {}\nholonomy: {:.2}\nphases: {}\nresolution: {:.2}",
                    oid, structural, phase_count, loss.resolution_ratio
                ))
            }
            prism::Imperfect::Failure(_, _) => Err(CliError::Runtime(MirrorRuntimeError(
                "compilation failed".to_string(),
            ))),
        }
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
        let compiled = self.runtime.compile_source(&source)?;
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

/// Print eigenvalues of a form tree (kind, name, params, variants).
fn project_form(form: &crate::mirror_runtime::Form, depth: usize, out: &mut String) {
    for _ in 0..depth {
        out.push_str("  ");
    }
    out.push_str(form.kind.as_str());
    if !form.name.is_empty() {
        out.push(' ');
        out.push_str(&form.name);
    }
    if !form.params.is_empty() {
        out.push_str(&format!("({})", form.params.join(", ")));
    }
    if !form.variants.is_empty() {
        out.push_str(&format!(" = {}", form.variants.join(" | ")));
    }
    out.push('\n');
    for child in &form.children {
        project_form(child, depth + 1, out);
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
            "focus", "project", "split", "zoom", "refract", "compile", "crystal", "ci", "init",
            "repl", "ai", "bench", "verify",
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
    fn help_text_includes_verify_and_sign() {
        let text = Cli::help_text();
        assert!(text.contains("verify"), "help should mention verify");
        assert!(text.contains("--sign"), "help should mention --sign");
    }
}
