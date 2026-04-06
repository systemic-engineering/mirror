//! Boot sequence — sequential compilation by layer.
//!
//! Reads `boot/` directory, groups .mirror files by number prefix,
//! compiles each layer sequentially. Barrier between layers.
//! Same prefix = same layer = no cross-dependencies.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::check;
use crate::model::Mirror;
use crate::parse::Parse;
use crate::runtime::Runtime;
use crate::Vector;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A layer number parsed from the filename prefix (e.g. `03` from `03-actor.mirror`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BootLayer(u8);

impl BootLayer {
    pub fn new(n: u8) -> Self {
        Self(n)
    }

    pub fn as_u8(self) -> u8 {
        self.0
    }
}

impl std::fmt::Display for BootLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

/// A single .mirror file in the boot sequence.
#[derive(Debug, Clone)]
pub struct BootEntry {
    pub layer: BootLayer,
    pub path: PathBuf,
    pub source: String,
}

/// The full boot sequence: layers of entries, ordered by layer number.
#[derive(Debug)]
pub struct BootSequence {
    pub layers: BTreeMap<BootLayer, Vec<BootEntry>>,
}

// ---------------------------------------------------------------------------
// Reading
// ---------------------------------------------------------------------------

impl BootSequence {
    /// Read the boot sequence from a directory.
    /// Files must be named `NN-name.mirror` where NN is the layer number.
    pub fn from_dir(dir: &Path) -> Result<Self, String> {
        let mut layers: BTreeMap<BootLayer, Vec<BootEntry>> = BTreeMap::new();

        let entries = std::fs::read_dir(dir)
            .map_err(|e| format!("boot: read_dir {}: {}", dir.display(), e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("boot: entry: {}", e))?;
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str()) != Some("mirror") {
                continue;
            }

            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| format!("boot: invalid filename: {}", path.display()))?;

            let layer = parse_layer_prefix(filename)?;
            let source = std::fs::read_to_string(&path)
                .map_err(|e| format!("boot: read {}: {}", path.display(), e))?;

            layers.entry(layer).or_default().push(BootEntry {
                layer,
                path,
                source,
            });
        }

        Ok(BootSequence { layers })
    }

    /// Total number of entries across all layers.
    pub fn len(&self) -> usize {
        self.layers.values().map(|v| v.len()).sum()
    }

    /// Whether the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }

    /// Number of layers.
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
}

/// Parse the layer prefix from a filename like `03-actor.mirror` → BootLayer(3).
fn parse_layer_prefix(filename: &str) -> Result<BootLayer, String> {
    let prefix = filename
        .split('-')
        .next()
        .ok_or_else(|| format!("boot: no prefix in: {}", filename))?;

    let n: u8 = prefix
        .parse()
        .map_err(|_| format!("boot: invalid layer number '{}' in: {}", prefix, filename))?;

    Ok(BootLayer(n))
}

// ---------------------------------------------------------------------------
// Compilation
// ---------------------------------------------------------------------------

/// Compile one source into a Verified domain.
fn compile_source(source: &str) -> Result<check::Verified, String> {
    let ast = Parse
        .trace(source.to_string())
        .into_result()
        .map_err(|e| format!("parse: {}", e))?;

    let grammar = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .ok_or_else(|| "boot: no grammar block found".to_string())?;

    let domain = Mirror::from_grammar(grammar).map_err(|e| format!("model: {}", e))?;
    check::verify(domain).map_err(|v| format!("check: {:?}", v))
}

/// Boot the full sequence: compile each layer sequentially, barrier between layers.
pub fn boot<R: Runtime>(
    runtime: &R,
    sequence: &BootSequence,
) -> Result<Vec<prism::Beam<Mirror>>, String> {
    let mut all_artifacts = Vec::new();

    for (layer, entries) in &sequence.layers {
        for entry in entries {
            let verified = compile_source(&entry.source)
                .map_err(|e| format!("layer {}, {}: {}", layer, entry.path.display(), e))?;
            let beam = runtime
                .compile(verified)
                .map_err(|e| format!("layer {}: runtime: {}", layer, e))?;
            all_artifacts.push(beam);
        }
    }

    Ok(all_artifacts)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeError;

    /// A trivial synchronous runtime for testing boot.
    struct TestRuntime;

    impl Runtime for TestRuntime {
        type Actor = ();
        type Error = RuntimeError;

        fn compile(&self, verified: check::Verified) -> Result<prism::Beam<Mirror>, RuntimeError> {
            Ok(prism::Beam::new(verified.into_mirror()))
        }
    }

    #[test]
    fn boot_layer_ordering() {
        assert!(BootLayer(0) < BootLayer(1));
        assert!(BootLayer(3) < BootLayer(7));
        assert_eq!(BootLayer(5), BootLayer(5));
    }

    #[test]
    fn boot_layer_display() {
        assert_eq!(format!("{}", BootLayer(3)), "03");
        assert_eq!(format!("{}", BootLayer(12)), "12");
    }

    #[test]
    fn parse_layer_prefix_valid() {
        assert_eq!(parse_layer_prefix("03-actor.mirror").unwrap(), BootLayer(3));
        assert_eq!(parse_layer_prefix("00-main.mirror").unwrap(), BootLayer(0));
        assert_eq!(
            parse_layer_prefix("07-projection.mirror").unwrap(),
            BootLayer(7)
        );
    }

    #[test]
    fn parse_layer_prefix_invalid() {
        assert!(parse_layer_prefix("actor.mirror").is_err());
        assert!(parse_layer_prefix("xx-bad.mirror").is_err());
    }

    #[test]
    fn boot_sequence_from_dir() {
        let seq =
            BootSequence::from_dir(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("boot.backup"))
                .unwrap();

        assert!(!seq.is_empty());
        assert!(seq.len() >= 15); // at least our 15 boot files
        assert!(seq.layer_count() >= 7); // layers 0-6

        // Layer 0 has main.mirror
        let layer0 = seq.layers.get(&BootLayer(0)).unwrap();
        assert!(layer0.iter().any(|e| e.source.contains("@conversation")));

        // Layer 3 has multiple entries (actor, beam, git, mail)
        let layer3 = seq.layers.get(&BootLayer(3)).unwrap();
        assert!(layer3.len() >= 4);
    }

    #[test]
    fn compile_source_valid() {
        let source = "grammar @test {\n  type = a | b\n}\n";
        let verified = compile_source(source).unwrap();
        assert_eq!(verified.domain().name.as_str(), "test");
    }

    #[test]
    fn compile_source_invalid() {
        let result = compile_source("not a grammar");
        assert!(result.is_err());
    }

    #[test]
    fn boot_sequence_compiles() {
        let seq =
            BootSequence::from_dir(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("boot.backup"))
                .unwrap();

        let runtime = TestRuntime;
        let artifacts = boot(&runtime, &seq).unwrap();

        // Should have compiled all boot entries
        assert_eq!(artifacts.len(), seq.len());

        // All should be lossless (clean compilation)
        for beam in &artifacts {
            assert!(beam.is_lossless());
        }
    }
}
