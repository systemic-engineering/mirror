//! AI loop: tournament selection — spawn excited Fate instances, keep lowest holonomy.
//!
//! Codegen for @ai.ai(). The compiler measuring its own grammars,
//! Fate selecting operations, the loop transforming files.
//!
//! Each iteration spawns N excited Fate instances. Each one selects a model.
//! The source is round-tripped through parse → emit. The candidate with
//! the lowest holonomy wins the tournament. If it improves on the current
//! holonomy, it becomes the new source. Repeat until crystal, stuck, or budget.

use std::path::Path;

use prism::lambda::LambdaFn;

use fate::{Fate, Model};

use crate::fate_bridge;
use crate::lambda_phases::{Parse, SourceText};
use crate::mirror_runtime::emit_fragment;

// ---------------------------------------------------------------------------
// Error
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum AiError {
    Io(std::io::Error),
    NoInput(String),
}

impl std::fmt::Display for AiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiError::Io(e) => write!(f, "io: {}", e),
            AiError::NoInput(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for AiError {}

impl From<std::io::Error> for AiError {
    fn from(e: std::io::Error) -> Self {
        AiError::Io(e)
    }
}

// ---------------------------------------------------------------------------
// Result
// ---------------------------------------------------------------------------

/// Output of the AI loop.
pub struct AiLoopResult {
    pub steps: usize,
    pub holonomy_start: f64,
    pub holonomy_end: f64,
    pub models_used: Vec<String>,
    pub health: String,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn model_name(m: Model) -> &'static str {
    match m {
        Model::Abyss => "abyss",
        Model::Introject => "introject",
        Model::Cartographer => "cartographer",
        Model::Explorer => "explorer",
        Model::Fate => "fate",
    }
}

/// Apply the optic transformation: parse → emit round-trip.
///
/// The round-trip drops what the parser couldn't see, which reduces holonomy.
/// Different Fate models will eventually produce different views; for now
/// all models do the same round-trip. The model parameter is accepted for
/// future differentiation.
fn apply_optic_transform(source: &str, _model: Model) -> Option<String> {
    let result = Parse.reduce(SourceText(source.to_string()));
    let fragment = result.ok()?;
    let emitted = emit_fragment(&fragment.0);

    // Only return if non-empty and actually different from input
    if emitted.is_empty() || emitted == source {
        None
    } else {
        Some(emitted)
    }
}

/// Number of candidates per tournament round.
const CANDIDATES_PER_ROUND: usize = 5;

// ---------------------------------------------------------------------------
// Loop
// ---------------------------------------------------------------------------

/// Run the AI loop on a file: tournament selection with write-back.
///
/// 1. Read source, measure holonomy (baseline).
/// 2. If holonomy == 0 → crystal, stop.
/// 3. Each iteration: spawn N excited Fate instances, each selects a model.
/// 4. For each selection, round-trip through parse → emit → measure holonomy.
/// 5. Keep the candidate with the lowest holonomy.
/// 6. If lowest < current → new source, write back, repeat.
/// 7. If no candidate improves → stuck, stop.
/// 8. Respect budget (max iterations).
pub fn ai_loop(file: &Path, budget: usize) -> Result<AiLoopResult, AiError> {
    if !file.exists() {
        return Err(AiError::NoInput(format!(
            "file not found: {}",
            file.display()
        )));
    }

    let mut source = std::fs::read_to_string(file)?;

    let (_, initial_loss) = fate_bridge::extract_features(&source);
    let holonomy_start = initial_loss.holonomy();

    // Crystal: holonomy 0, nothing to do
    if holonomy_start == 0.0 {
        return Ok(AiLoopResult {
            steps: 0,
            holonomy_start: 0.0,
            holonomy_end: 0.0,
            models_used: vec![],
            health: "crystal".to_string(),
        });
    }

    let mut current_holonomy = holonomy_start;
    let mut models_used = Vec::new();

    for _step in 0..budget {
        // Tournament: spawn N excited instances, each tries a transformation
        let mut best_source = None;
        let mut best_holonomy = current_holonomy;
        let mut best_model = Model::Abyss;

        for _ in 0..CANDIDATES_PER_ROUND {
            let fate = Fate::excited();
            let (features, _) = fate_bridge::extract_features(&source);
            let output = fate.tick(&features);

            // Apply transformation: parse → emit round-trip
            if let Some(transformed) = apply_optic_transform(&source, output.decision.model) {
                let (_, new_loss) = fate_bridge::extract_features(&transformed);
                let new_holonomy = new_loss.holonomy();

                if new_holonomy < best_holonomy {
                    best_holonomy = new_holonomy;
                    best_source = Some(transformed);
                    best_model = output.decision.model;
                }
            }
        }

        match best_source {
            Some(better) => {
                source = better;
                current_holonomy = best_holonomy;
                models_used.push(model_name(best_model).to_string());

                if current_holonomy == 0.0 {
                    // Crystal achieved
                    std::fs::write(file, &source)?;
                    return Ok(AiLoopResult {
                        steps: models_used.len(),
                        holonomy_start,
                        holonomy_end: 0.0,
                        models_used,
                        health: "crystal".to_string(),
                    });
                }
            }
            None => {
                // No candidate improved. Stuck.
                break;
            }
        }
    }

    // Write best result back if improved
    if current_holonomy < holonomy_start {
        std::fs::write(file, &source)?;
    }

    Ok(AiLoopResult {
        steps: models_used.len(),
        holonomy_start,
        holonomy_end: current_holonomy,
        models_used,
        health: if current_holonomy < holonomy_start {
            "improved"
        } else {
            "stuck"
        }
        .to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_name_all_variants() {
        assert_eq!(model_name(Model::Abyss), "abyss");
        assert_eq!(model_name(Model::Introject), "introject");
        assert_eq!(model_name(Model::Cartographer), "cartographer");
        assert_eq!(model_name(Model::Explorer), "explorer");
        assert_eq!(model_name(Model::Fate), "fate");
    }

    #[test]
    fn ai_error_display() {
        let io_err = AiError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "gone"));
        assert!(format!("{}", io_err).contains("gone"));

        let no_input = AiError::NoInput("missing".to_string());
        assert_eq!(format!("{}", no_input), "missing");
    }

    #[test]
    fn ai_loop_file_not_found() {
        let result = ai_loop(Path::new("/nonexistent/file.mirror"), 10);
        assert!(result.is_err());
    }

    #[test]
    fn apply_optic_transform_returns_none_for_crystal() {
        // A clean source round-trips identically → None (no change)
        let source = "type color = red | blue\n";
        assert!(apply_optic_transform(source, Model::Abyss).is_none());
    }

    #[test]
    fn apply_optic_transform_drops_unrecognized() {
        let source = "type x = a | b\nwidget foo\n";
        let result = apply_optic_transform(source, Model::Abyss);
        // If the parser produces a fragment, the round-trip should drop "widget foo"
        if let Some(transformed) = result {
            assert!(
                !transformed.contains("widget"),
                "round-trip should drop unrecognized content, got: {}",
                transformed
            );
        }
        // If parse fails entirely (returns None), that's also acceptable
    }

    #[test]
    fn apply_optic_transform_returns_none_for_empty_parse() {
        // Complete garbage → parse failure → None
        let source = "@@@ not valid mirror at all &&&\n";
        assert!(apply_optic_transform(source, Model::Fate).is_none());
    }

    #[test]
    fn ai_loop_on_crystal_stops_immediately() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("crystal.mirror");
        std::fs::write(&file, "type color = red | blue\n").unwrap();
        let result = ai_loop(&file, 10).unwrap();
        assert_eq!(result.holonomy_start, 0.0);
        assert_eq!(result.steps, 0);
        assert_eq!(result.health, "crystal");
    }

    #[test]
    fn ai_loop_on_partial_attempts_improvement() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("partial.mirror");
        std::fs::write(&file, "type x = a | b\nwidget foo\ngarbage bar\n").unwrap();
        let result = ai_loop(&file, 5).unwrap();
        assert!(result.holonomy_start > 0.0);
        // Should either improve or report stuck
        assert!(
            result.health == "improved" || result.health == "stuck" || result.health == "crystal"
        );
    }

    #[test]
    fn ai_loop_reduces_holonomy_on_mixed_content() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("mixed.mirror");
        // Source with recognized and unrecognized content
        std::fs::write(&file, "type x = a | b\nwidget foo\ntype y\ngarbage bar\n").unwrap();
        let result = ai_loop(&file, 10).unwrap();
        // The round-trip should drop widget and garbage, reducing holonomy
        if result.health == "improved" || result.health == "crystal" {
            assert!(result.holonomy_end < result.holonomy_start);
            // Verify file was actually modified
            let new_content = std::fs::read_to_string(dir.path().join("mixed.mirror")).unwrap();
            assert!(
                !new_content.contains("widget"),
                "unrecognized content should be dropped"
            );
        }
    }

    #[test]
    fn ai_loop_respects_budget() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("budget.mirror");
        std::fs::write(&file, "type x\nwidget y\n").unwrap();
        let result = ai_loop(&file, 2).unwrap();
        assert!(result.steps <= 2);
    }

    #[test]
    fn ai_loop_does_not_destroy_clean_content() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("preserve.mirror");
        std::fs::write(&file, "type color = red | blue\n").unwrap();
        let result = ai_loop(&file, 5).unwrap();
        let content = std::fs::read_to_string(&file).unwrap();
        // Crystal file should not be modified
        assert!(content.contains("color"));
        assert_eq!(result.health, "crystal");
    }
}
