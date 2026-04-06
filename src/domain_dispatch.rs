//! Domain dispatch — `mirror @domain action [args]` CLI routing.
//!
//! When the first CLI arg starts with `@`, parse as a domain invocation.
//! Structurally: a fold on the domain's action space.
//!
//! `mirror @fate abyss`       → run the Abyss model (focus)
//! `mirror @ai project`       → run the Mirror classifier (2,892 params)
//! `mirror @ai coherence`     → check ghost echo coherence
//! `mirror @json parse '{}'`  → fold input through the JSON domain

/// A parsed domain invocation from CLI args.
#[derive(Debug, Clone)]
pub struct DomainInvocation {
    /// Domain name (without the `@` prefix).
    pub domain: String,
    /// Action name — the fold target.
    pub action: String,
    /// Remaining arguments passed to the action.
    pub args: Vec<String>,
}

impl DomainInvocation {
    /// Parse CLI positional args as a domain invocation.
    /// Returns None if the first arg doesn't start with `@` or no action follows.
    pub fn parse(positional: &[&str]) -> Option<Self> {
        let first = positional.first()?;
        if !first.starts_with('@') {
            return None;
        }
        let domain = first.trim_start_matches('@').to_string();
        let action = positional.get(1)?.to_string();
        let args = positional[2..].iter().map(|s| s.to_string()).collect();
        Some(DomainInvocation {
            domain,
            action,
            args,
        })
    }
}

/// Dispatch a domain invocation. Returns output string or error.
pub fn dispatch(inv: &DomainInvocation) -> Result<String, String> {
    match inv.domain.as_str() {
        "fate" => dispatch_fate(&inv.action, &inv.args),
        "ai" => dispatch_ai(&inv.action, &inv.args),
        _ => Err(format!("unknown domain: @{}", inv.domain)),
    }
}

// ---------------------------------------------------------------------------
// @fate — the five models (425 params, Fate selector)
// ---------------------------------------------------------------------------

fn dispatch_fate(action: &str, args: &[String]) -> Result<String, String> {
    let model = match action {
        "abyss" | "focus" => fate::Model::Abyss,
        "pathfinder" | "project" => fate::Model::Pathfinder,
        "cartographer" | "split" => fate::Model::Cartographer,
        "explorer" | "zoom" => fate::Model::Explorer,
        "fate" | "refract" => fate::Model::Fate,
        _ => return Err(format!("@fate: unknown action: {}", action)),
    };

    let input_features = if args.is_empty() {
        [0.0; fate::FEATURE_DIM]
    } else {
        crate::features::extract_from_source(&args.join(" "))
    };

    let rt = fate::runtime::CompiledFateRuntime::new();
    let next = rt.select(model, &input_features);

    let mut output = String::new();
    output.push_str(&format!("@fate {}\n", action));
    output.push_str(&format!("  model:  {:?}\n", model));
    output.push_str(&format!("  next:   {:?}\n", next));
    Ok(output)
}

// ---------------------------------------------------------------------------
// @ai — the Mirror model (2,892 params, classifier + ghost echo)
// ---------------------------------------------------------------------------

fn dispatch_ai(action: &str, args: &[String]) -> Result<String, String> {
    match action {
        "project" => dispatch_ai_project(args),
        "coherence" => dispatch_ai_coherence(args),
        "settle" => dispatch_ai_settle(args),
        _ => Err(format!("@ai: unknown action: {}", action)),
    }
}

/// Default project: run the 2,892-parameter classifier on spectral features.
fn dispatch_ai_project(args: &[String]) -> Result<String, String> {
    let source = args.join(" ");
    let spectral = crate::features::extract_from_source(&source);
    let mut input = [0.0; crate::classifier::INPUT_DIM];
    let n = crate::features::FEATURE_DIM.min(crate::classifier::INPUT_DIM);
    input[..n].copy_from_slice(&spectral[..n]);
    let weights = crate::classifier::trained();
    let (optic, confidence, _) = crate::classifier::classify(&weights, &input);
    Ok(format!(
        "@ai project → {:?} ({:.1}%)",
        optic,
        confidence * 100.0
    ))
}

/// Check ghost echo coherence.
fn dispatch_ai_coherence(args: &[String]) -> Result<String, String> {
    let source = args.join(" ");
    let spectral = crate::features::extract_from_source(&source);
    let echo = crate::ghost::default_echo();
    let distance = echo.coherence_distance(&spectral);
    let score = echo.coherence_score(&spectral, 1.0);
    let cluster = if score > 0.5 {
        "echo (exploring)"
    } else {
        "shadow (conserving)"
    };

    let mut output = String::new();
    output.push_str("@ai coherence\n");
    output.push_str(&format!("  distance:  {:.4}\n", distance));
    output.push_str(&format!("  score:     {:.4}\n", score));
    output.push_str(&format!("  cluster:   {}\n", cluster));
    Ok(output)
}

/// Full settle: Fate selection + classifier + coherence.
fn dispatch_ai_settle(args: &[String]) -> Result<String, String> {
    let source = args.join(" ");
    let spectral = crate::features::extract_from_source(&source);

    // Fate: what model should run?
    let rt = fate::runtime::CompiledFateRuntime::new();
    let next = rt.select(fate::Model::Abyss, &spectral);

    // Classifier: what optic?
    let mut classifier_input = [0.0; crate::classifier::INPUT_DIM];
    let n = crate::features::FEATURE_DIM.min(crate::classifier::INPUT_DIM);
    classifier_input[..n].copy_from_slice(&spectral[..n]);
    let weights = crate::classifier::trained();
    let (optic, confidence, _) = crate::classifier::classify(&weights, &classifier_input);

    // Ghost echo: coherent?
    let echo = crate::ghost::default_echo();
    let score = echo.coherence_score(&spectral, 1.0);
    let cluster = if score > 0.5 {
        "echo (exploring)"
    } else {
        "shadow (conserving)"
    };

    let mut output = String::new();
    output.push_str("@ai settle\n");
    output.push_str(&format!(
        "  model:      {:?} → {:?}\n",
        fate::Model::Abyss,
        next
    ));
    output.push_str(&format!(
        "  optic:      {:?} ({:.1}%)\n",
        optic,
        confidence * 100.0
    ));
    output.push_str(&format!("  coherence:  {:.4}\n", score));
    output.push_str(&format!("  cluster:    {}\n", cluster));
    Ok(output)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_domain_invocation() {
        let inv = DomainInvocation::parse(&["@fate", "abyss"]).unwrap();
        assert_eq!(inv.domain, "fate");
        assert_eq!(inv.action, "abyss");
        assert!(inv.args.is_empty());
    }

    #[test]
    fn parse_domain_invocation_with_args() {
        let inv = DomainInvocation::parse(&["@ai", "project", "some input"]).unwrap();
        assert_eq!(inv.domain, "ai");
        assert_eq!(inv.action, "project");
        assert_eq!(inv.args, vec!["some input"]);
    }

    #[test]
    fn parse_domain_invocation_missing_action() {
        assert!(DomainInvocation::parse(&["@fate"]).is_none());
    }

    #[test]
    fn parse_non_domain_returns_none() {
        assert!(DomainInvocation::parse(&["test", "file.conv"]).is_none());
    }

    #[test]
    fn dispatch_fate_abyss() {
        let inv = DomainInvocation {
            domain: "fate".to_string(),
            action: "abyss".to_string(),
            args: vec![],
        };
        let result = dispatch(&inv);
        assert!(result.is_ok(), "fate dispatch should succeed: {:?}", result);
        assert!(result.unwrap().contains("Abyss"));
    }

    #[test]
    fn dispatch_fate_unknown_fails() {
        let inv = DomainInvocation {
            domain: "fate".to_string(),
            action: "nonexistent".to_string(),
            args: vec![],
        };
        assert!(dispatch(&inv).is_err());
    }

    #[test]
    fn dispatch_unknown_domain_fails() {
        let inv = DomainInvocation {
            domain: "nonexistent".to_string(),
            action: "fold".to_string(),
            args: vec![],
        };
        assert!(dispatch(&inv).is_err());
    }

    #[test]
    fn dispatch_ai_project_returns_optic() {
        let inv = DomainInvocation {
            domain: "ai".to_string(),
            action: "project".to_string(),
            args: vec![],
        };
        let result = dispatch(&inv);
        assert!(result.is_ok());
    }

    #[test]
    fn dispatch_ai_coherence_returns_score() {
        let inv = DomainInvocation {
            domain: "ai".to_string(),
            action: "coherence".to_string(),
            args: vec![],
        };
        let result = dispatch(&inv);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("coherence"));
    }

    #[test]
    fn dispatch_ai_settle_returns_full_report() {
        let inv = DomainInvocation {
            domain: "ai".to_string(),
            action: "settle".to_string(),
            args: vec![],
        };
        let result = dispatch(&inv);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("model:"));
        assert!(output.contains("optic:"));
        assert!(output.contains("cluster:"));
    }
}
