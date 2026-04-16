//! Test the AI loop against fixture files.
//! Each successful simplification becomes training data.

fn tension_of(source: &str) -> f64 {
    let (_, loss) = mirror::fate_bridge::extract_features(source);
    loss.tension()
}

#[test]
fn crystal_fixture_has_zero_tension() {
    let source = std::fs::read_to_string("fixtures/ai/crystal.mirror").unwrap();
    assert_eq!(tension_of(&source), 0.0, "crystal fixture should have zero tension");
}

#[test]
fn low_tension_fixture_is_low() {
    let source = std::fs::read_to_string("fixtures/ai/low-tension.mirror").unwrap();
    let t = tension_of(&source);
    assert!(t > 0.0 && t < 0.5, "low tension fixture should be (0, 0.5), got {}", t);
}

#[test]
fn medium_tension_fixture_is_medium() {
    let source = std::fs::read_to_string("fixtures/ai/medium-tension.mirror").unwrap();
    let t = tension_of(&source);
    assert!(t >= 0.3 && t <= 0.8, "medium tension fixture should be [0.3, 0.8], got {}", t);
}

#[test]
fn high_tension_fixture_is_high() {
    let source = std::fs::read_to_string("fixtures/ai/high-tension.mirror").unwrap();
    let t = tension_of(&source);
    assert!(t >= 0.6, "high tension fixture should be >= 0.6, got {}", t);
}

#[test]
fn grammar_fixture_has_dark_dimensions() {
    let source = std::fs::read_to_string("fixtures/ai/grammar-tension.mirror").unwrap();
    let t = tension_of(&source);
    // Has both recognized (type, grammar, in, out) and unrecognized (io) content
    assert!(t > 0.0 && t < 1.0, "grammar fixture should have partial tension, got {}", t);
}

#[test]
fn boot_fate_grammar_tension_is_not_one() {
    // THE BUG: fate.mirror was reporting tension 1.0
    let source = std::fs::read_to_string("boot/std/fate.mirror").unwrap();
    let t = tension_of(&source);
    assert!(t < 1.0, "fate.mirror has recognized content, tension should be < 1.0, got {}", t);
}

#[test]
fn tension_is_always_normalized() {
    // Test all fixtures are in [0, 1]
    for entry in std::fs::read_dir("fixtures/ai").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map_or(false, |e| e == "mirror") {
            let source = std::fs::read_to_string(&path).unwrap();
            let t = tension_of(&source);
            assert!(
                (0.0..=1.0).contains(&t),
                "{}: tension {} not in [0,1]",
                path.display(), t
            );
        }
    }
}

#[test]
fn ai_loop_preserves_all_content_in_grammar_fixture() {
    let dir = tempfile::tempdir().unwrap();
    let fixture = std::fs::read_to_string("fixtures/ai/grammar-tension.mirror").unwrap();
    let file = dir.path().join("grammar.mirror");
    std::fs::write(&file, &fixture).unwrap();

    let result = mirror::ai::ai_loop(&file, 10).unwrap();
    let after = std::fs::read_to_string(&file).unwrap();

    // io lines must survive (dark dimensions)
    if result.health == "improved" {
        assert!(after.contains("io") || after.contains("send"),
            "dark dimensions must be preserved after improvement");
    }
}
