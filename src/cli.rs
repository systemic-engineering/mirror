//! Cli — the mirror command-line interface.
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
            "compile" => self.cmd_compile(args),
            "crystal" => self.cmd_crystal(args),
            "ai" => self.cmd_ai(args),
            _ => self.cmd_query(command, args),
        }
    }

    // -----------------------------------------------------------------------
    // compile
    // -----------------------------------------------------------------------

    fn cmd_compile(&self, args: &[String]) -> Result<String, CliError> {
        let file = args.first().ok_or_else(|| {
            CliError::Usage("usage: mirror compile <file>".to_string())
        })?;

        let source = std::fs::read_to_string(file)?;
        let mut compiler = crate::bundle::MirrorCompiler::new();
        match compiler.compile(&source) {
            Ok(compiled) => {
                let shard = crate::shard::Shard::new(
                    compiled.crystal().clone(),
                    compiler.kernel_spec.clone(),
                    compiler.target,
                );
                eprintln!(
                    "compiled {} → {} (rank {}, {:?})",
                    file,
                    shard.grammar_oid.as_str(),
                    shard.rank(),
                    shard.target,
                );
                Ok(shard.grammar_oid.as_str().to_string())
            }
            Err(e) => Err(CliError::Runtime(
                MirrorRuntimeError(format!("compile {}: {}", file, e)),
            )),
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

        let store_dir =
            std::env::temp_dir().join(format!("mirror-crystal-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&store_dir);

        match self.runtime.materialize_crystal(&boot_dir, &store_dir, &output) {
            Ok(oid) => {
                eprintln!("crystal {} → {}", oid.as_str(), output.display());
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
    // query — the fallback
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
        std::fs::write(&spec, "type greeting\n").unwrap();
        let cli = Cli::open(spec.to_str().unwrap()).unwrap();
        assert!(cli.crystal_oid().is_some());
    }

    #[test]
    fn dispatch_compile_valid_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let file = dir.path().join("test.mirror");
        std::fs::write(&file, "type greeting\n").unwrap();

        let cli = Cli::default();
        let result = cli.dispatch("compile", &[file.to_str().unwrap().to_string()]);
        assert!(result.is_ok());
        let oid = result.unwrap();
        assert!(
            oid.len() == 64 && oid.chars().all(|c| c.is_ascii_hexdigit()),
            "should be hex OID: {}",
            oid
        );
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
        assert!(output.contains("greeting"));
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
        let e: Box<dyn std::error::Error> =
            Box::new(CliError::Usage("test".to_string()));
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
    /// Content addressing guarantees determinism — the crystal IS its content.
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

    /// The real meta-property: the running binary's crystal OID matches
    /// a fresh compilation of the shatter file.
    /// Ignored until Store is wired (Tick 2) — the full spec.mirror
    /// references grammars that require the boot sequence.
    #[test]
    #[ignore = "requires Store wiring (Tick 2) — full spec.mirror needs boot sequence"]
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

        let compiled = runtime
            .compile_file(&shatter)
            .expect("compile shatter");

        let _ = std::fs::remove_dir_all(&store_dir);
        let _ = std::fs::remove_file(&shatter);

        assert_eq!(
            crystal_oid.as_str(),
            compiled.crystal().as_str(),
            "crystal OID from materialize must equal OID from compiling the shatter"
        );
    }
}
