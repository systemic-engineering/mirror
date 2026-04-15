//! Integration test: full project lifecycle.
//! mirror new → write .mirror files → mirror run

#[test]
fn full_lifecycle() {
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("void-and-form");

    // 1. Create project
    mirror::scaffold::scaffold_project(&project, "void-and-form").unwrap();

    // 2. Verify spec
    let spec_content = std::fs::read_to_string(project.join("mirror.spec")).unwrap();
    let spec = mirror::spec::parse_spec_source(&spec_content).unwrap();
    assert_eq!(spec.oid, "@void-and-form");
    assert!(!spec.run.default.is_empty());

    // 3. Write a .mirror file (the first grammar of void-and-form)
    std::fs::write(
        project.join("mirror/distinction.mirror"),
        "\
-- The distinction axiom: a type with two provably unequal elements.
-- Wielsch's D₀ in mirror.

type side = left | right

type distinction {
  carrier: side,
}

property coverage(distinction) = iso
",
    )
    .unwrap();

    // 4. Run the project
    let result = mirror::run::run_project(&project).unwrap();
    assert!(result.contains("@void-and-form"));
    assert!(result.contains("distinction.mirror"));

    // 5. Verify git + store
    assert!(project.join(".git").exists());
    assert!(project.join(".git/mirror").exists());
}

#[test]
fn new_project_is_pure_mirror() {
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("pure-test");
    mirror::scaffold::scaffold_project(&project, "pure-test").unwrap();

    // No Rust files anywhere in the project root
    let has_rust = std::fs::read_dir(&project)
        .unwrap()
        .filter_map(|e| e.ok())
        .any(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.ends_with(".rs") || name == "Cargo.toml"
        });
    assert!(!has_rust, "consumer project must contain no Rust files");

    // Expected contents: mirror.spec, mirror/, .gitignore, .git
    let entries: Vec<String> = std::fs::read_dir(&project)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    assert!(entries.contains(&"mirror.spec".to_string()));
    assert!(entries.contains(&"mirror".to_string()));
    assert!(entries.contains(&".gitignore".to_string()));
    assert!(entries.contains(&".git".to_string()));
}

#[test]
fn scaffolded_spec_has_all_blocks() {
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().join("blocks-test");
    mirror::scaffold::scaffold_project(&project, "blocks-test").unwrap();
    let content = std::fs::read_to_string(project.join("mirror.spec")).unwrap();
    let spec = mirror::spec::parse_spec_source(&content).unwrap();

    // All blocks present
    assert_eq!(spec.oid, "@blocks-test");
    assert!(!spec.craft.default.is_empty(), "craft default should be set");
    assert!(!spec.run.default.is_empty(), "run default should be set");
    assert!(!spec.properties.requires.is_empty(), "properties requires should be set");
    assert!(!spec.properties.invariant.is_empty(), "properties invariant should be set");
    assert!(!spec.properties.ensures.is_empty(), "properties ensures should be set");
}
