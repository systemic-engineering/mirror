//! ShatterFormat — frontmatter serialization for `.shatter` files.
//!
//! A `.shatter` file is valid `.mirror` syntax with a YAML-like frontmatter
//! header. The frontmatter carries the compilation trace (OID, luminosity,
//! holonomy, per-fold loss breakdown, and beam identity). The body is the
//! `.mirror` source.
//!
//! ## Format
//!
//! ```text
//! ---
//! oid: <content-hash>
//! luminosity: light | dimmed | dark
//! holonomy: <f64>
//! loss:
//!   parse: <f64>
//!   resolution: <f64>
//!   properties: <f64>
//!   emit: <f64>
//! beam:
//!   compiler: mirror-v<semver>
//!   prism: shatter
//!   target: mirror
//! ---
//!
//! <body>
//! ```
//!
//! No serde. No YAML crate. Line-by-line parsing only.

#[cfg(test)]
use crate::loss::UnrecognizedDecl;
use crate::loss::{Convergence, MirrorLoss};
use crate::mirror_runtime::CompiledShatter;
#[cfg(test)]
use crate::mirror_runtime::MirrorRuntime;
use crate::prism_crate::Loss;
use fragmentation::sha::HashAlg;

// ---------------------------------------------------------------------------
// Luminosity — compilation health in a single word
// ---------------------------------------------------------------------------

/// Luminosity summarises compilation health.
///
/// - `Light` — zero loss, crystal settled.
/// - `Dimmed` — partial success, loss measured.
/// - `Dark` — failure, `BudgetExhausted` convergence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Luminosity {
    Light,
    Dimmed,
    Dark,
}

impl Luminosity {
    pub fn from_loss(loss: &MirrorLoss) -> Self {
        if loss.convergence == Convergence::BudgetExhausted {
            Luminosity::Dark
        } else if loss.is_zero() {
            Luminosity::Light
        } else {
            Luminosity::Dimmed
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Luminosity::Light => "light",
            Luminosity::Dimmed => "dimmed",
            Luminosity::Dark => "dark",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "light" => Some(Luminosity::Light),
            "dimmed" => Some(Luminosity::Dimmed),
            "dark" => Some(Luminosity::Dark),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// ShatterLossBreakdown — per-fold holonomy values
// ---------------------------------------------------------------------------

/// Per-fold holonomy values from the four compilation phases.
#[derive(Clone, Debug, PartialEq)]
pub struct ShatterLossBreakdown {
    pub parse: f64,
    pub resolution: f64,
    pub properties: f64,
    pub emit: f64,
}

// ---------------------------------------------------------------------------
// ShatterBeamInfo — compiler identity
// ---------------------------------------------------------------------------

/// Identity of the compiler that produced this artifact.
#[derive(Clone, Debug, PartialEq)]
pub struct ShatterBeamInfo {
    pub compiler: String,
    pub prism: String,
    pub target: String,
}

// ---------------------------------------------------------------------------
// ShatterMeta — the complete frontmatter payload
// ---------------------------------------------------------------------------

/// Full frontmatter for a `.shatter` file.
#[derive(Clone, Debug, PartialEq)]
pub struct ShatterMeta {
    pub oid: String,
    pub luminosity: Luminosity,
    pub holonomy: f64,
    pub loss: ShatterLossBreakdown,
    pub beam: ShatterBeamInfo,
}

impl ShatterMeta {
    pub fn from_compiled(compiled: &CompiledShatter, loss: &MirrorLoss) -> Self {
        ShatterMeta {
            oid: compiled.crystal().as_str().to_string(),
            luminosity: Luminosity::from_loss(loss),
            holonomy: loss.holonomy(),
            loss: ShatterLossBreakdown {
                parse: loss.parse.holonomy(),
                resolution: loss.resolution.holonomy(),
                properties: loss.properties.holonomy(),
                emit: loss.emit.holonomy(),
            },
            beam: ShatterBeamInfo {
                compiler: format!("mirror-v{}", env!("CARGO_PKG_VERSION")),
                prism: "shatter".into(),
                target: "mirror".into(),
            },
        }
    }
}

// ---------------------------------------------------------------------------
// emit_shatter_with_frontmatter — serialize
// ---------------------------------------------------------------------------

/// Emit a `.shatter` file: YAML-like frontmatter followed by the body.
pub fn emit_shatter_with_frontmatter(meta: &ShatterMeta, body: &str) -> String {
    let mut out = String::new();
    out.push_str("---\n");
    out.push_str(&format!("oid: {}\n", meta.oid));
    out.push_str(&format!("luminosity: {}\n", meta.luminosity.as_str()));
    out.push_str(&format!("holonomy: {}\n", meta.holonomy));
    out.push_str("loss:\n");
    out.push_str(&format!("  parse: {}\n", meta.loss.parse));
    out.push_str(&format!("  resolution: {}\n", meta.loss.resolution));
    out.push_str(&format!("  properties: {}\n", meta.loss.properties));
    out.push_str(&format!("  emit: {}\n", meta.loss.emit));
    out.push_str("beam:\n");
    out.push_str(&format!("  compiler: {}\n", meta.beam.compiler));
    out.push_str(&format!("  prism: {}\n", meta.beam.prism));
    out.push_str(&format!("  target: {}\n", meta.beam.target));
    out.push_str("---\n\n");
    out.push_str(body);
    out
}

// ---------------------------------------------------------------------------
// parse_shatter_frontmatter — deserialize
// ---------------------------------------------------------------------------

/// Parse a `.shatter` file into `(ShatterMeta, body)`.
///
/// Returns `Err(String)` if the frontmatter is missing, malformed, or a
/// required field is absent. No serde, no YAML crate — line-by-line only.
pub fn parse_shatter_frontmatter(source: &str) -> Result<(ShatterMeta, &str), String> {
    // Must start with "---\n"
    if !source.starts_with("---\n") {
        return Err("shatter file must start with '---'".into());
    }

    // Find the closing "---\n" (starting after the opening delimiter)
    let after_open = &source[4..];
    let close_pos = after_open
        .find("\n---\n")
        .ok_or_else(|| "shatter file has no closing '---'".to_string())?;

    let frontmatter = &after_open[..close_pos];
    // body starts after "\n---\n" + optional blank line
    let body_start = 4 + close_pos + 5; // skip "\n---\n"
    let body = if source.len() > body_start && source.as_bytes()[body_start] == b'\n' {
        &source[body_start + 1..]
    } else {
        &source[body_start..]
    };

    // Parse the frontmatter line by line.
    let mut oid: Option<String> = None;
    let mut luminosity: Option<Luminosity> = None;
    let mut holonomy: Option<f64> = None;
    let mut loss_parse: Option<f64> = None;
    let mut loss_resolution: Option<f64> = None;
    let mut loss_properties: Option<f64> = None;
    let mut loss_emit: Option<f64> = None;
    let mut beam_compiler: Option<String> = None;
    let mut beam_prism: Option<String> = None;
    let mut beam_target: Option<String> = None;

    // Track which section we're in for indented sub-keys
    #[derive(PartialEq)]
    enum Section {
        Top,
        Loss,
        Beam,
    }
    let mut section = Section::Top;

    for line in frontmatter.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("  ") {
            // Indented sub-key
            let trimmed = line.trim();
            let (key, val) = split_kv(trimmed)?;
            match section {
                Section::Loss => match key {
                    "parse" => loss_parse = Some(parse_f64(val)?),
                    "resolution" => loss_resolution = Some(parse_f64(val)?),
                    "properties" => loss_properties = Some(parse_f64(val)?),
                    "emit" => loss_emit = Some(parse_f64(val)?),
                    _ => {} // unknown sub-key — tolerate
                },
                Section::Beam => match key {
                    "compiler" => beam_compiler = Some(val.to_string()),
                    "prism" => beam_prism = Some(val.to_string()),
                    "target" => beam_target = Some(val.to_string()),
                    _ => {}
                },
                Section::Top => {
                    return Err(format!("unexpected indented key '{}' outside section", key))
                }
            }
        } else {
            // Top-level key
            if line == "loss:" {
                section = Section::Loss;
                continue;
            }
            if line == "beam:" {
                section = Section::Beam;
                continue;
            }
            section = Section::Top;
            let (key, val) = split_kv(line)?;
            match key {
                "oid" => oid = Some(val.to_string()),
                "luminosity" => {
                    luminosity = Some(
                        Luminosity::parse(val)
                            .ok_or_else(|| format!("unknown luminosity '{}'", val))?,
                    )
                }
                "holonomy" => holonomy = Some(parse_f64(val)?),
                _ => {} // tolerate unknown top-level keys
            }
        }
    }

    let meta = ShatterMeta {
        oid: oid.ok_or("missing 'oid' in frontmatter")?,
        luminosity: luminosity.ok_or("missing 'luminosity' in frontmatter")?,
        holonomy: holonomy.ok_or("missing 'holonomy' in frontmatter")?,
        loss: ShatterLossBreakdown {
            parse: loss_parse.ok_or("missing 'loss.parse' in frontmatter")?,
            resolution: loss_resolution.ok_or("missing 'loss.resolution' in frontmatter")?,
            properties: loss_properties.ok_or("missing 'loss.properties' in frontmatter")?,
            emit: loss_emit.ok_or("missing 'loss.emit' in frontmatter")?,
        },
        beam: ShatterBeamInfo {
            compiler: beam_compiler.ok_or("missing 'beam.compiler' in frontmatter")?,
            prism: beam_prism.ok_or("missing 'beam.prism' in frontmatter")?,
            target: beam_target.ok_or("missing 'beam.target' in frontmatter")?,
        },
    };

    Ok((meta, body))
}

// ---------------------------------------------------------------------------
// ShatterNotification — Phase 3 (LSP) handoff type
// ---------------------------------------------------------------------------

/// Notification that a `.shatter` artifact was updated in the store.
///
/// Phase 3 (LSP) will consume these notifications to push diagnostics
/// back to the editor. Produced by `compile_to_shatter` / `cmd_compile`.
#[derive(Clone, Debug)]
pub struct ShatterNotification {
    /// Absolute or workspace-relative path to the source `.mirror` file.
    pub file_path: String,
    /// Content-address of the stored `.shatter` artifact.
    pub oid: String,
    /// Compilation health summary.
    pub luminosity: Luminosity,
}

/// Split `"key: value"` → `("key", "value")`.
fn split_kv(line: &str) -> Result<(&str, &str), String> {
    let colon = line
        .find(": ")
        .ok_or_else(|| format!("expected 'key: value', got '{}'", line))?;
    Ok((&line[..colon], &line[colon + 2..]))
}

/// Parse a `&str` as `f64`.
fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse::<f64>()
        .map_err(|_| format!("expected f64, got '{}'", s))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loss::MirrorLoss;
    use prism::{Imperfect, Loss};

    // -- Luminosity --

    #[test]
    fn luminosity_from_loss_zero() {
        let loss = MirrorLoss::zero();
        assert_eq!(Luminosity::from_loss(&loss), Luminosity::Light);
    }

    #[test]
    fn luminosity_from_loss_partial() {
        let mut loss = MirrorLoss::zero();
        loss.parse.unrecognized.push(UnrecognizedDecl {
            keyword: "widget".into(),
            line: 1,
            content: "foo".into(),
        });
        assert_eq!(Luminosity::from_loss(&loss), Luminosity::Dimmed);
    }

    #[test]
    fn luminosity_from_loss_failure() {
        let loss = MirrorLoss::total();
        assert_eq!(Luminosity::from_loss(&loss), Luminosity::Dark);
    }

    #[test]
    fn luminosity_as_str_roundtrip() {
        for (lum, s) in [
            (Luminosity::Light, "light"),
            (Luminosity::Dimmed, "dimmed"),
            (Luminosity::Dark, "dark"),
        ] {
            assert_eq!(lum.as_str(), s);
            assert_eq!(Luminosity::parse(s), Some(lum));
        }
    }

    #[test]
    fn luminosity_parse_unknown() {
        assert_eq!(Luminosity::parse("unknown"), None);
        assert_eq!(Luminosity::parse(""), None);
    }

    // -- ShatterMeta::from_compiled --

    #[test]
    fn shatter_meta_from_compiled() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("type color = red | blue");
        let compiled = result.ok().unwrap();
        let loss = MirrorLoss::zero();
        let meta = ShatterMeta::from_compiled(&compiled, &loss);
        assert_eq!(meta.luminosity, Luminosity::Light);
        assert_eq!(meta.holonomy, 0.0);
        assert_eq!(meta.beam.prism, "shatter");
    }

    // -- emit_shatter_with_frontmatter --

    #[test]
    fn emit_shatter_frontmatter() {
        let meta = ShatterMeta {
            oid: "a3f8c2d1".into(),
            luminosity: Luminosity::Light,
            holonomy: 0.0,
            loss: ShatterLossBreakdown {
                parse: 0.0,
                resolution: 0.0,
                properties: 0.0,
                emit: 0.0,
            },
            beam: ShatterBeamInfo {
                compiler: "mirror-v0.1".into(),
                prism: "shatter".into(),
                target: "rust".into(),
            },
        };
        let body = "type color = red | blue\n";
        let output = emit_shatter_with_frontmatter(&meta, body);
        assert!(output.starts_with("---\n"));
        assert!(output.contains("oid: a3f8c2d1"));
        assert!(output.contains("luminosity: light"));
        assert!(output.contains("holonomy: 0"));
        assert!(output.ends_with("type color = red | blue\n"));
    }

    // -- parse_shatter_frontmatter --

    #[test]
    fn parse_shatter_frontmatter_roundtrip() {
        let meta = ShatterMeta {
            oid: "a3f8c2d1".into(),
            luminosity: Luminosity::Light,
            holonomy: 0.0,
            loss: ShatterLossBreakdown {
                parse: 0.0,
                resolution: 0.0,
                properties: 0.0,
                emit: 0.0,
            },
            beam: ShatterBeamInfo {
                compiler: "mirror-v0.1".into(),
                prism: "shatter".into(),
                target: "rust".into(),
            },
        };
        let body = "type color = red | blue\n";
        let serialized = emit_shatter_with_frontmatter(&meta, body);
        let (parsed_meta, parsed_body) = parse_shatter_frontmatter(&serialized).unwrap();
        assert_eq!(parsed_meta.oid, "a3f8c2d1");
        assert_eq!(parsed_meta.luminosity, Luminosity::Light);
        assert_eq!(parsed_meta.holonomy, 0.0);
        assert_eq!(parsed_body.trim(), body.trim());
    }

    #[test]
    fn parse_shatter_dimmed() {
        let meta = ShatterMeta {
            oid: "def456".into(),
            luminosity: Luminosity::Dimmed,
            holonomy: 2.3,
            loss: ShatterLossBreakdown {
                parse: 1.0,
                resolution: 1.3,
                properties: 0.0,
                emit: 0.0,
            },
            beam: ShatterBeamInfo {
                compiler: "mirror-v0.1".into(),
                prism: "shatter".into(),
                target: "mirror".into(),
            },
        };
        let body = "widget foo\ntype bar\n";
        let serialized = emit_shatter_with_frontmatter(&meta, body);
        let (parsed, _) = parse_shatter_frontmatter(&serialized).unwrap();
        assert_eq!(parsed.luminosity, Luminosity::Dimmed);
        assert!((parsed.holonomy - 2.3).abs() < 0.01);
        assert!((parsed.loss.parse - 1.0).abs() < 0.01);
        assert!((parsed.loss.resolution - 1.3).abs() < 0.01);
    }

    #[test]
    fn parse_shatter_error_no_opening_delimiter() {
        let result = parse_shatter_frontmatter("oid: foo\n");
        assert!(result.is_err());
    }

    #[test]
    fn parse_shatter_error_no_closing_delimiter() {
        let result = parse_shatter_frontmatter("---\noid: foo\n");
        assert!(result.is_err());
    }

    // -- Integration: full compile pipeline --

    #[test]
    fn compile_to_shatter_format() {
        let runtime = MirrorRuntime::new();
        let result = runtime.compile_source("type visibility = public | protected | private\n");

        let (compiled, loss) = match result {
            Imperfect::Success(c) => (c, MirrorLoss::zero()),
            Imperfect::Partial(c, l) => (c, l),
            Imperfect::Failure(e, _) => panic!("should compile: {}", e),
        };

        let meta = ShatterMeta::from_compiled(&compiled, &loss);
        let body = crate::mirror_runtime::emit_fragment(&compiled.fragment);
        let shatter = emit_shatter_with_frontmatter(&meta, &body);

        let (parsed_meta, parsed_body) = parse_shatter_frontmatter(&shatter).unwrap();
        assert_eq!(parsed_meta.luminosity, Luminosity::Light);
        assert_eq!(parsed_meta.holonomy, 0.0);
        assert!(!parsed_body.is_empty());
    }
}
