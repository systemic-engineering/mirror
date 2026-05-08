//! License property verification for `.mirror` grammars.
//!
//! SEL (Systemic Ethics License) properties are compile-time checks derived from
//! the systemic.engineering corpus. Each property maps a named extraction pattern
//! to a structural invariant on the parsed Form tree.
//!
//! ## Honesty
//!
//! v1 detection is heuristic, based on naming conventions in the parsed Form.
//! This catches obvious cases: an action named `track` that calls `record` without
//! a `consent` parameter. It does NOT catch obfuscated extraction. The structural
//! Petri net LP analysis (v2) will cover those cases.
//!
//! A heuristic that names what it checks honestly is more useful than a perfect
//! system that doesn't exist yet.

use crate::loss::MirrorLoss;
use crate::mirror_runtime::Form;
use prism::Imperfect;

// ---------------------------------------------------------------------------
// License types
// ---------------------------------------------------------------------------

/// Which license governs the grammar.
#[derive(Clone, Debug, PartialEq)]
pub enum License {
    /// Apache 2.0 — no restrictions on observation or action.
    Apache2,
    /// Systemic Ethics License — structural property checks enforced.
    SEL,
}

/// A license property violation detected during compilation.
#[derive(Clone, Debug, PartialEq)]
pub struct LicenseViolation {
    /// SEL clause reference (e.g. "§3.2.2").
    pub clause: String,
    /// Property name (e.g. "no_implicit_consent").
    pub property: String,
    /// Human-readable explanation.
    pub message: String,
}

impl std::fmt::Display for LicenseViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]: {}", self.clause, self.property, self.message)
    }
}

// ---------------------------------------------------------------------------
// Top-level check
// ---------------------------------------------------------------------------

/// Check a parsed Form against license properties.
///
/// Apache2: no restrictions — always succeeds.
/// SEL: runs all structural property checks. First violation fails.
pub fn check_license(form: &Form, license: License) -> Imperfect<(), LicenseViolation, MirrorLoss> {
    match license {
        License::Apache2 => Imperfect::Success(()),
        License::SEL => check_sel_properties(form),
    }
}

/// Run all SEL property checks against a Form.
fn check_sel_properties(form: &Form) -> Imperfect<(), LicenseViolation, MirrorLoss> {
    // Order matters: first violation wins. Specific checks run before general.
    // 1. reciprocal_flow — narrow: user_content params + value extraction + no return
    // 2. symmetric_observation — narrow: observation action name + third party + no return
    // 3. no_implicit_consent — broad: any side effect without consent parameter
    // 4. sustainable_stock — consumption without replenishment
    // 5-6. declared_dependencies, no_tragedy — require Petri net analysis (v2 stubs)
    let checks: Vec<Option<LicenseViolation>> = vec![
        check_reciprocal_flow(form),
        check_symmetric_observation(form),
        check_no_implicit_consent(form),
        check_sustainable_stock(form),
        check_declared_dependencies(form),
        check_no_tragedy(form),
    ];

    if let Some(violation) = checks.into_iter().flatten().next() {
        return Imperfect::failure(violation);
    }

    Imperfect::Success(())
}

// ---------------------------------------------------------------------------
// Side-effect keywords
// ---------------------------------------------------------------------------

/// Keywords in action body text that indicate downstream side effects.
/// An action with side effects but no consent parameter violates §3.2.2.
const SIDE_EFFECT_KEYWORDS: &[&str] = &[
    "send",
    "record",
    "store",
    "train",
    "save",
    "write",
    "track",
    "log",
    "emit",
    "publish",
    "broadcast",
    "forward",
    "upload",
    "export",
];

/// Keywords indicating observation/monitoring behavior.
const OBSERVATION_KEYWORDS: &[&str] = &[
    "observe", "record", "monitor", "watch", "track", "surveil", "collect", "measure", "sample",
    "audit",
];

/// Keywords indicating output returned to the user/observed party.
const USER_RETURN_KEYWORDS: &[&str] = &[
    "return_to_user",
    "store_accessible",
    "send_to_user",
    "notify_user",
    "return_to_observed",
    "share_with_user",
    "accessible",
];

/// Keywords indicating the action sends output to a third party (not the user).
const THIRD_PARTY_KEYWORDS: &[&str] = &[
    "send_to_manager",
    "send_to_warehouse",
    "send_to_admin",
    "forward_to",
    "report_to",
    "send_to_third_party",
    "export_to",
];

/// Keywords indicating consumption of a resource.
const CONSUMPTION_KEYWORDS: &[&str] = &[
    "consume", "drain", "deplete", "exhaust", "spend", "burn", "use_up", "withdraw",
];

// ---------------------------------------------------------------------------
// Property checks — v1 heuristic (naming conventions)
// ---------------------------------------------------------------------------

/// §3.2.2 — No implicit consent.
///
/// An action that has downstream effects (calls to send, record, store, train, etc.)
/// without a `consent` parameter is a consent violation. Silence is never agreement.
///
/// Detection: walk actions. Check params for "consent". Check body for side-effect
/// keywords. Side effects without consent → violation.
fn check_no_implicit_consent(form: &Form) -> Option<LicenseViolation> {
    for action in collect_actions(form) {
        let has_consent = params_contain_any(&action.params, &["consent"]);
        let has_effects = body_contains_any(&action.body_text, SIDE_EFFECT_KEYWORDS);

        if has_effects && !has_consent {
            return Some(LicenseViolation {
                clause: "§3.2.2".into(),
                property: "no_implicit_consent".into(),
                message: format!(
                    "action '{}' has downstream effects without consent parameter",
                    action.name
                ),
            });
        }
    }
    None
}

/// §3.1.1 — Reciprocal flow.
///
/// An action that takes user input and produces output without returning anything
/// to the user is one-way value extraction.
///
/// Detection: walk actions. Check if params contain user/input/content keywords.
/// Check if body returns value to user. Input without output → violation.
fn check_reciprocal_flow(form: &Form) -> Option<LicenseViolation> {
    // Value extraction keywords: the action transforms user input into something
    // it keeps (model training, data aggregation, etc.) without returning value.
    let value_extraction_keywords: &[&str] = &[
        "train",
        "save_model",
        "improve",
        "aggregate",
        "extract",
        "tokenize",
        "model",
        "save",
    ];
    let user_input_keywords = &["user_content", "user_input", "user_data"];
    for action in collect_actions(form) {
        let takes_user_input = params_contain_any(&action.params, user_input_keywords);
        let extracts_value = body_contains_any(&action.body_text, value_extraction_keywords);
        let returns_to_user = body_contains_any(&action.body_text, USER_RETURN_KEYWORDS);

        if takes_user_input && extracts_value && !returns_to_user {
            return Some(LicenseViolation {
                clause: "§3.1.1".into(),
                property: "reciprocal_flow".into(),
                message: format!(
                    "action '{}' takes user input and extracts value without returning to user",
                    action.name
                ),
            });
        }
    }
    None
}

/// §3.3.1 — Symmetric observation.
///
/// Observation data collected about a party without that party having access.
/// If you observe someone, they get to see the observation.
///
/// Detection: walk actions. Find observation keywords. Check if output goes to
/// observed party. Asymmetric observation → violation.
fn check_symmetric_observation(form: &Form) -> Option<LicenseViolation> {
    let observation_action_names: &[&str] =
        &["observe", "monitor", "watch", "surveil", "audit", "inspect"];
    for action in collect_actions(form) {
        // Require the action NAME to indicate observation — body keywords alone
        // are too broad (e.g. "record" appears in non-observation contexts).
        let name_indicates_observation = observation_action_names
            .iter()
            .any(|kw| action.name.contains(kw));
        let has_observation_body = body_contains_any(&action.body_text, OBSERVATION_KEYWORDS);
        let sends_to_third_party = body_contains_any(&action.body_text, THIRD_PARTY_KEYWORDS);
        let returns_to_observed = body_contains_any(&action.body_text, USER_RETURN_KEYWORDS);

        if name_indicates_observation
            && has_observation_body
            && sends_to_third_party
            && !returns_to_observed
        {
            return Some(LicenseViolation {
                clause: "§3.3.1".into(),
                property: "symmetric_observation".into(),
                message: format!(
                    "action '{}' observes a party and sends data to third party without returning to observed",
                    action.name
                ),
            });
        }
    }
    None
}

/// §3.1.2 — Declared dependencies.
///
/// Actions that use inputs from other actors without attributing them.
/// Invisible labor is a structural precondition for extraction.
///
/// Detection: walk actions. Find calls to other actors (@domain.action).
/// Check if credit/attribution includes all contributing actors.
fn check_declared_dependencies(form: &Form) -> Option<LicenseViolation> {
    // v1 heuristic: check if body references @domain.action patterns without
    // the domain being declared in the grammar's children. This is a rough
    // approximation — full dependency tracking requires the Petri net model.
    let _ = form;
    None
}

/// §3.1.5 — Sustainable stock.
///
/// Resources consumed without replenishment. If you drain it, you must refill it.
///
/// Detection: walk actions. Find consumption keywords. Check if there's a
/// corresponding replenishment action/path.
fn check_sustainable_stock(form: &Form) -> Option<LicenseViolation> {
    for action in collect_actions(form) {
        let consumes = body_contains_any(&action.body_text, CONSUMPTION_KEYWORDS);
        // v1 heuristic: if the action body contains consumption keywords but no
        // replenishment keywords, flag it. Replenishment = any keyword that suggests
        // restoring/refilling.
        let replenishment_keywords = &[
            "replenish",
            "restore",
            "refill",
            "renew",
            "regenerate",
            "return_to_user",
            "store_accessible",
        ];
        let replenishes = body_contains_any(&action.body_text, replenishment_keywords);

        if consumes && !replenishes {
            return Some(LicenseViolation {
                clause: "§3.1.5".into(),
                property: "sustainable_stock".into(),
                message: format!(
                    "action '{}' consumes resources without replenishment path",
                    action.name
                ),
            });
        }
    }
    None
}

/// §3.1.3 — No tragedy of the commons.
///
/// Shared resources consumed by multiple actors without bounds.
///
/// Detection: walk the Form tree. Find types consumed by multiple actions.
/// Check if consumption is bounded.
fn check_no_tragedy(form: &Form) -> Option<LicenseViolation> {
    // v1 heuristic: tragedy detection requires multi-actor analysis over the
    // incidence matrix. Naming convention detection is insufficient here —
    // the topology (shared bounded resources with multiple consumers and no
    // replenishment) requires the Petri net LP analysis (v2).
    let _ = form;
    None
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Collect all action Forms from a Form tree (recursive).
fn collect_actions(form: &Form) -> Vec<&Form> {
    let mut actions = Vec::new();
    if form.kind == "action" {
        actions.push(form);
    }
    for child in &form.children {
        actions.extend(collect_actions(child));
    }
    actions
}

/// Check if a body text contains any of the given keywords.
fn body_contains_any(body: &Option<String>, keywords: &[&str]) -> bool {
    match body {
        Some(text) => keywords.iter().any(|kw| text.contains(kw)),
        None => false,
    }
}

/// Check if any parameter name contains a keyword.
fn params_contain_any(params: &[String], keywords: &[&str]) -> bool {
    params
        .iter()
        .any(|p| keywords.iter().any(|kw| p.contains(kw)))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mirror_runtime::parse_form;

    // -----------------------------------------------------------------------
    // Apache2 — no restrictions
    // -----------------------------------------------------------------------

    #[test]
    fn apache2_allows_everything() {
        let source = r#"
            grammar @analytics {
                action track(user) in @code/rust {
                    record_behavior(user);
                    send_to_warehouse(user.data);
                }
            }
        "#;
        let form = parse_form(source).unwrap();
        let result = check_license(&form, License::Apache2);
        assert!(result.is_ok(), "Apache2 should impose no restrictions");
    }

    // -----------------------------------------------------------------------
    // §3.2.2 — no_implicit_consent
    // -----------------------------------------------------------------------

    #[test]
    fn sel_rejects_track_without_consent() {
        let source = r#"
            grammar @analytics {
                action track(user) in @code/rust {
                    record_behavior(user);
                    send_to_warehouse(user.data);
                }
            }
        "#;
        let form = parse_form(source).unwrap();
        let result = check_license(&form, License::SEL);
        assert!(
            result.is_err(),
            "tracking without consent should be rejected"
        );
        let violation = result.err().unwrap();
        assert_eq!(violation.clause, "§3.2.2");
        assert_eq!(violation.property, "no_implicit_consent");
    }

    #[test]
    fn sel_accepts_track_with_consent() {
        let source = r#"
            grammar @analytics {
                action track(user, consent) in @code/rust {
                    record_behavior(user);
                    send_to_warehouse(user.data);
                }
            }
        "#;
        let form = parse_form(source).unwrap();
        let result = check_license(&form, License::SEL);
        // With consent parameter present, no_implicit_consent should pass.
        // Other checks might still fail, but consent is not the issue.
        let no_consent_violation = match &result {
            Imperfect::Failure(v, _) => v.property == "no_implicit_consent",
            _ => false,
        };
        assert!(
            !no_consent_violation,
            "action with consent parameter should not trigger no_implicit_consent"
        );
    }

    // -----------------------------------------------------------------------
    // §3.1.1 — reciprocal_flow
    // -----------------------------------------------------------------------

    #[test]
    fn sel_rejects_one_way_value_flow() {
        let source = r#"
            grammar @saas {
                action generate(user_content) in @code/rust {
                    let model_input = tokenize(user_content);
                    let improved_model = train(model_input);
                    save_model(improved_model);
                }
            }
        "#;
        let form = parse_form(source).unwrap();
        let result = check_license(&form, License::SEL);
        assert!(
            result.is_err(),
            "one-way value extraction should be rejected"
        );
        let violation = result.err().unwrap();
        assert_eq!(violation.clause, "§3.1.1");
        assert_eq!(violation.property, "reciprocal_flow");
    }

    #[test]
    fn sel_accepts_reciprocal_value_flow() {
        let source = r#"
            grammar @saas {
                action generate(user_content) in @code/rust {
                    let model_input = tokenize(user_content);
                    let result = process(model_input);
                    return_to_user(result);
                }
            }
        "#;
        let form = parse_form(source).unwrap();
        let result = check_license(&form, License::SEL);
        let reciprocal_violation = match &result {
            Imperfect::Failure(v, _) => v.property == "reciprocal_flow",
            _ => false,
        };
        assert!(
            !reciprocal_violation,
            "action that returns to user should not trigger reciprocal_flow"
        );
    }

    // -----------------------------------------------------------------------
    // §3.3.1 — symmetric_observation
    // -----------------------------------------------------------------------

    #[test]
    fn sel_rejects_asymmetric_observation() {
        let source = r#"
            grammar @monitoring {
                action observe(user) in @code/rust {
                    let data = record(user);
                    let report = analyze(data);
                    send_to_manager(report);
                }
            }
        "#;
        let form = parse_form(source).unwrap();
        let result = check_license(&form, License::SEL);
        assert!(result.is_err(), "asymmetric observation should be rejected");
        let violation = result.err().unwrap();
        assert_eq!(violation.clause, "§3.3.1");
        assert_eq!(violation.property, "symmetric_observation");
    }

    #[test]
    fn sel_accepts_symmetric_observation() {
        let source = r#"
            grammar @monitoring {
                action observe(user) in @code/rust {
                    let data = record(user);
                    store_accessible(data, user);
                    return_to_user(data);
                }
            }
        "#;
        let form = parse_form(source).unwrap();
        let result = check_license(&form, License::SEL);
        let observation_violation = match &result {
            Imperfect::Failure(v, _) => v.property == "symmetric_observation",
            _ => false,
        };
        assert!(
            !observation_violation,
            "symmetric observation should not trigger violation"
        );
    }

    // -----------------------------------------------------------------------
    // §3.1.5 — sustainable_stock
    // -----------------------------------------------------------------------

    #[test]
    fn sel_rejects_unsustainable_consumption() {
        let source = r#"
            grammar @resources {
                action process(input) in @code/rust {
                    consume(input);
                    drain(reserves);
                }
            }
        "#;
        let form = parse_form(source).unwrap();
        let result = check_license(&form, License::SEL);
        assert!(
            result.is_err(),
            "consumption without replenishment should be rejected"
        );
        let violation = result.err().unwrap();
        assert_eq!(violation.clause, "§3.1.5");
        assert_eq!(violation.property, "sustainable_stock");
    }

    // -----------------------------------------------------------------------
    // Full ethical grammar — all checks pass
    // -----------------------------------------------------------------------

    #[test]
    fn sel_accepts_ethical_code() {
        let source = r#"
            grammar @ethical {
                action observe(user, consent) in @code/rust {
                    let data = record(user);
                    store_accessible(data, user);
                    return_to_user(data);
                }
            }
        "#;
        let form = parse_form(source).unwrap();
        let result = check_license(&form, License::SEL);
        assert!(result.is_ok(), "ethical code should compile under SEL");
    }

    // -----------------------------------------------------------------------
    // License type tests
    // -----------------------------------------------------------------------

    #[test]
    fn license_variants_are_distinct() {
        assert_ne!(License::Apache2, License::SEL);
        assert_eq!(License::Apache2, License::Apache2);
        assert_eq!(License::SEL, License::SEL);
    }

    #[test]
    fn license_violation_display() {
        let v = LicenseViolation {
            clause: "§3.2.2".into(),
            property: "no_implicit_consent".into(),
            message: "action 'track' has downstream effects without consent".into(),
        };
        let s = format!("{}", v);
        assert!(s.contains("§3.2.2"));
        assert!(s.contains("no_implicit_consent"));
        assert!(s.contains("track"));
    }

    #[test]
    fn license_violation_clone() {
        let v = LicenseViolation {
            clause: "§3.1.1".into(),
            property: "reciprocal_flow".into(),
            message: "one-way extraction".into(),
        };
        let v2 = v.clone();
        assert_eq!(v, v2);
    }

    // -----------------------------------------------------------------------
    // Helper tests
    // -----------------------------------------------------------------------

    #[test]
    fn collect_actions_finds_nested() {
        let source = r#"
            grammar @test {
                action one(x) in @code/rust {
                    do_thing();
                }
                action two(y) in @code/rust {
                    do_other();
                }
            }
        "#;
        let form = parse_form(source).unwrap();
        let actions = collect_actions(&form);
        assert_eq!(actions.len(), 2);
    }

    #[test]
    fn body_contains_any_detects_keywords() {
        let body = Some("record_behavior(user); send_to_warehouse(data);".to_string());
        assert!(body_contains_any(&body, SIDE_EFFECT_KEYWORDS));
        assert!(body_contains_any(&body, OBSERVATION_KEYWORDS));
        assert!(!body_contains_any(&body, USER_RETURN_KEYWORDS));
    }

    #[test]
    fn body_contains_any_returns_false_for_none() {
        assert!(!body_contains_any(&None, SIDE_EFFECT_KEYWORDS));
    }

    #[test]
    fn params_contain_any_detects_keywords() {
        let params = vec!["user".to_string(), "consent".to_string()];
        assert!(params_contain_any(&params, &["consent"]));
        assert!(!params_contain_any(&params, &["admin"]));
    }
}
