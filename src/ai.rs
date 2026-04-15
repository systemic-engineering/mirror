//! AI loop: measure → Fate tick → report.
//!
//! Codegen for @ai.ai(). The compiler measuring its own grammars,
//! Fate selecting operations, the loop reporting what it sees.
//!
//! Currently observe-only: reports what Fate recommends without
//! applying transformations. The `break` becomes real when
//! write-side optics land.

use std::path::Path;

use fate::{Fate, Model};

use crate::fate_bridge;

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
// Loop
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

/// Run the AI loop on a file.
///
/// measure features -> Fate tick -> report model selection and health.
/// Observe-only until write-side optics exist.
pub fn ai_loop(file: &Path, budget: usize) -> Result<AiLoopResult, AiError> {
    if !file.exists() {
        return Err(AiError::NoInput(format!(
            "file not found: {}",
            file.display()
        )));
    }

    let (features, loss) = fate_bridge::extract_features_from_file(file)?;
    let holonomy_start = loss.holonomy();

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

    let fate = Fate::excited();
    let output = fate.tick(&features);
    let health = match output.health {
        fate::feature::HolonomyHealth::TooShallow => "too_shallow",
        fate::feature::HolonomyHealth::Healthy => "healthy",
        fate::feature::HolonomyHealth::OverCutting => "over_cutting",
    };

    // If over-cutting, stop immediately
    if output.health == fate::feature::HolonomyHealth::OverCutting {
        return Ok(AiLoopResult {
            steps: 0,
            holonomy_start,
            holonomy_end: holonomy_start,
            models_used: vec![model_name(output.model).to_string()],
            health: health.to_string(),
        });
    }

    // Observe-only: report one step
    let _budget = budget; // respected when write-side lands
    Ok(AiLoopResult {
        steps: 1,
        holonomy_start,
        holonomy_end: holonomy_start, // no transformation yet
        models_used: vec![model_name(output.model).to_string()],
        health: health.to_string(),
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
}
