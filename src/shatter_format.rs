//! ShatterFormat — frontmatter serialization for `.shatter` files.
//!
//! Phase 1 of the mirror LSP plan. See task: Tick 3 LSP server.

use crate::loss::{Convergence, MirrorLoss, UnrecognizedDecl};
use crate::mirror_runtime::{CompiledShatter, MirrorRuntime};
use crate::prism_crate::Loss;
use fragmentation::sha::HashAlg;

// ---------------------------------------------------------------------------
// Tests — written first (TDD 🔴 phase)
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
        let body = crate::mirror_runtime::emit_form(&compiled.form);
        let shatter = emit_shatter_with_frontmatter(&meta, &body);

        let (parsed_meta, parsed_body) = parse_shatter_frontmatter(&shatter).unwrap();
        assert_eq!(parsed_meta.luminosity, Luminosity::Light);
        assert_eq!(parsed_meta.holonomy, 0.0);
        assert!(!parsed_body.is_empty());
    }
}
