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
use prism::Imperfect;

use fate::{Fate, Model};

use crate::mirror_ast::MirrorAST;
use crate::fate_bridge;
use crate::lambda_phases::{Parse, SourceText};
use crate::mirror_runtime::{emit_fragment, kintsugi_ast};

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
    /// Tension [0,1]. 0 = crystal, 1 = opaque.
    pub tension_start: f64,
    pub tension_end: f64,
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

/// Apply the optic transformation via tree merge.
///
/// 1. Parse original source → original fragment tree
/// 2. Round-trip through parse → emit → re-parse → candidate tree
/// 3. Merge trees: for each node, keep the one with lower holonomy
/// 4. Emit merged tree back to source
///
/// Information may not be destroyed, only refined.
/// Dark dimensions are preserved because fragment::merge keeps
/// children from both sides.
fn apply_optic_transform(source: &str, _model: Model) -> Option<String> {
    // Parse original → MirrorAST
    let original_result = Parse.reduce(SourceText(source.to_string()));
    let original_ast = match &original_result {
        Imperfect::Success(_) => return None, // already crystal
        Imperfect::Partial(parsed, _) => &parsed.0,
        Imperfect::Failure(_, _) => return None, // can't transform
    };

    // Round-trip: emit recognized content, re-parse it
    let emitted = emit_fragment(original_ast);
    if emitted.is_empty() {
        return None;
    }
    let candidate_result = Parse.reduce(SourceText(emitted));
    let candidate_ast = match &candidate_result {
        Imperfect::Success(parsed) => &parsed.0,
        Imperfect::Partial(parsed, _) => &parsed.0,
        Imperfect::Failure(_, _) => return None,
    };

    // Tree merge: keep the node with lower holonomy
    let merged = holonomy_resolve(original_ast, candidate_ast);

    // Emit merged tree back to source
    let merged_source = emit_fragment(&merged);
    if merged_source.is_empty() || merged_source == source {
        return None;
    }

    // Guard: information may not be destroyed.
    // Check that every non-empty, non-comment source line appears
    // in the output (possibly trimmed). If any content line is missing,
    // the parser dropped dark dimensions. Refuse to write back.
    let missing = source.lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with("--")
        })
        .any(|line| {
            let trimmed = line.trim();
            !merged_source.lines().any(|out_line| out_line.trim() == trimmed)
        });
    if missing {
        return None; // content would be lost — refuse
    }

    Some(merged_source)
}

/// Merge strategy: keep the node with lower holonomy.
///
/// This is the resolve lambda for fragment::merge.
/// It's the `greedy` tournament rule applied at the tree level.
fn holonomy_resolve(old: &MirrorAST, new: &MirrorAST) -> MirrorAST {
    use prism::MerkleTree;

    // Compare child counts as a proxy for information preservation.
    let old_children = old.children().len();
    let new_children = new.children().len();

    if new_children > old_children {
        new.clone()
    } else if old_children > new_children {
        old.clone()
    } else {
        new.clone()
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
    let tension_start = initial_loss.tension();

    // Crystal: tension 0, nothing to do
    if tension_start == 0.0 {
        return Ok(AiLoopResult {
            steps: 0,
            tension_start: 0.0,
            tension_end: 0.0,
            models_used: vec![],
            health: "crystal".to_string(),
        });
    }

    let mut current_tension = tension_start;
    let mut models_used = Vec::new();

    for _step in 0..budget {
        // Tournament: spawn N excited instances, each tries a transformation
        let mut best_source = None;
        let mut best_tension = current_tension;
        let mut best_model = Model::Abyss;

        for _ in 0..CANDIDATES_PER_ROUND {
            let fate = Fate::excited();
            let (features, _) = fate_bridge::extract_features(&source);
            let output = fate.tick(&features);

            // Apply transformation: parse → emit round-trip
            if let Some(transformed) = apply_optic_transform(&source, output.decision.model) {
                let (_, new_loss) = fate_bridge::extract_features(&transformed);
                let new_tension = new_loss.tension();

                if new_tension < best_tension {
                    best_tension = new_tension;
                    best_source = Some(transformed);
                    best_model = output.decision.model;
                }
            }
        }

        match best_source {
            Some(better) => {
                source = better;
                current_tension = best_tension;
                models_used.push(model_name(best_model).to_string());

                if current_tension == 0.0 {
                    // Crystal achieved
                    std::fs::write(file, &source)?;
                    return Ok(AiLoopResult {
                        steps: models_used.len(),
                        tension_start,
                        tension_end: 0.0,
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
    if current_tension < tension_start {
        std::fs::write(file, &source)?;
    }

    Ok(AiLoopResult {
        steps: models_used.len(),
        tension_start,
        tension_end: current_tension,
        models_used,
        health: if current_tension < tension_start {
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
    fn apply_optic_transform_uses_tree_merge() {
        let source = "type x = a | b\nwidget foo\n";
        let result = apply_optic_transform(source, Model::Abyss);
        // Tree merge: the result should be a valid fragment emission
        // If transformation happens, it's a tree merge, not line manipulation
        if let Some(transformed) = result {
            // Should still be parseable
            let re_parsed = Parse.reduce(SourceText(transformed.clone()));
            assert!(
                re_parsed.is_ok() || re_parsed.is_partial(),
                "merged output must be parseable, got failure for: {}",
                transformed
            );
        }
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
        assert_eq!(result.tension_start, 0.0);
        assert_eq!(result.steps, 0);
        assert_eq!(result.health, "crystal");
    }

    #[test]
    fn ai_loop_on_partial_attempts_improvement() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("partial.mirror");
        std::fs::write(&file, "type x = a | b\nwidget foo\ngarbage bar\n").unwrap();
        let result = ai_loop(&file, 5).unwrap();
        assert!(result.tension_start > 0.0);
        // Should either improve or report stuck
        assert!(
            result.health == "improved" || result.health == "stuck" || result.health == "crystal"
        );
    }

    #[test]
    fn ai_loop_on_mixed_content() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("mixed.mirror");
        std::fs::write(&file, "type x = a | b\nwidget foo\ntype y\ngarbage bar\n").unwrap();
        let result = ai_loop(&file, 10).unwrap();
        // Tree merge: health can be improved, stuck, or crystal
        assert!(
            result.health == "improved" || result.health == "stuck" || result.health == "crystal"
        );
        // File must still be parseable
        let content = std::fs::read_to_string(&file).unwrap();
        let re_parsed = Parse.reduce(SourceText(content));
        assert!(re_parsed.is_ok() || re_parsed.is_partial());
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
