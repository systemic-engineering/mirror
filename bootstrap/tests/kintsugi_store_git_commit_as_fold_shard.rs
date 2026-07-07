//! N5 TICK 1 RED — `shards/kintsugi/store/git.mirror` species:
//! `commit_as_fold` action + `mirror kintsugi --commit IS git commit` at
//! git-projection altitude.
//!
//! **Terminal tick of the N-cascade.** Wires everything together:
//! - N1 (`2857fb1`): verdict predicate authorizes memoization
//! - N2 (`0a72c42`): action_cache species provides the surface
//! - N3 (`756f2f7`): Rust wiring makes it live
//! - N4 (`6bf05cb`): impacted_by enables surgical invalidation
//! - **N5 (this tick)**: commit-as-fold projects the whole cascade to git
//!
//! **Business-observable outcome**: every `mirror kintsugi --commit`
//! folds the verdict cache into a git commit. Every git rebase walks
//! `impacted_by` over the fold to determine which cache entries are
//! affected. The substrate's persistence layer IS git via `@mirror/store/git`;
//! the substrate's transformation layer IS the kintsugi fold via this species.
//! Together: commit-as-fold = cli verb + species action pair.
//!
//! **Third witness for `cli-verb-pair-specialises-species-action-pair`**:
//! - Witness 1: `spawn/kintsugi` ⇔ `@song/movement.enter/close` (M2 TICK 2)
//! - Witness 2: (waiting)
//! - Witness 3 (this tick): `kintsugi --commit` ⇔ `@kintsugi/store/git.commit_as_fold`
//!
//! Landing this species PROMOTES the recognition from CANDIDATE to LANDED.
//!
//! **Third witness for `cross-species-discharge-is-first-class`**:
//! - Witness 1: N3 cmd_kintsugi_spec calling @mirror/store/action_cache
//! - Witness 2: N4 impacted_by → action_cache invalidation composition
//! - Witness 3 (this tick): commit_as_fold composes N2 action_cache +
//!   N4 impacted_by + @mirror/store/git.set_ref cross three species boundaries
//!
//! Landing this species PROMOTES that candidate too.
//!
//! **Form/process partition** (#55): this species sits on the @kintsugi
//! (process) side of the partition, sibling to @mirror/store/git
//! (form/state-observation side). `commit_as_fold` is the transformation;
//! `@mirror/store/git.set_ref` is the state-mutation the transformation
//! discharges to.
//!
//! **Interpretation B canonical DOES apply** (green-field species; new
//! shard file; narrative-above `---` seam, `in @` clauses below).

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_commit_as_fold_shard() -> String {
    let path = repo_root().join("shards/kintsugi/store/git.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/kintsugi/store/git.mirror at {:?}: {}", path, e))
}

fn first_nonempty_line(content: &str) -> Option<&str> {
    content.lines().find(|l| !l.trim().is_empty())
}

fn seam_line_indices(content: &str) -> Vec<usize> {
    content
        .lines()
        .enumerate()
        .filter_map(|(i, l)| if l == "---" { Some(i) } else { None })
        .collect()
}

// === T1-T4: canonical shape + species declaration ===

#[test]
fn t01_shard_declares_species_at_path_pact() {
    let content = read_commit_as_fold_shard();
    assert!(
        content.contains("@kintsugi/store/git"),
        "T1: shard MUST declare `@kintsugi/store/git` per path-namespace pact"
    );
}

#[test]
fn t02_first_nonempty_line_is_narrative_docblock() {
    let content = read_commit_as_fold_shard();
    let first = first_nonempty_line(&content).expect("T2: must have non-empty content");
    assert!(
        first.trim_start().starts_with('#'),
        "T2: first non-empty line must be `#`-narrative per Interpretation B; got `{}`",
        first
    );
    assert_ne!(first.trim(), "---", "T2: line-1 `---` is drift");
    assert!(
        !first.trim_start().starts_with("in "),
        "T2: `in @...` clauses live BELOW the seam"
    );
}

#[test]
fn t03_exactly_one_seam_at_column_zero_and_in_clauses_below() {
    let content = read_commit_as_fold_shard();
    let seams = seam_line_indices(&content);
    assert_eq!(
        seams.len(),
        1,
        "T3: exactly one `---` at column 0 required; found {}",
        seams.len()
    );
    let seam_idx = seams[0];
    for (i, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("in @") {
            assert!(
                i > seam_idx,
                "T3: `in @...` at line {} appears ABOVE seam at line {}. Line: `{}`",
                i + 1,
                seam_idx + 1,
                line
            );
        }
    }
}

#[test]
fn t04_species_inherits_universal_transparency_and_kintsugi_family_root() {
    let content = read_commit_as_fold_shard();
    for req in ["in @prism", "in @meta", "in @glass", "in @kintsugi"] {
        assert!(
            content.contains(req),
            "T4: species MUST inherit `{}` (universal + transparency + @kintsugi family-root)",
            req
        );
    }
}

// === T5-T7: commit_as_fold action ===

#[test]
fn t05_declares_commit_as_fold_action() {
    let content = read_commit_as_fold_shard();
    let has_action = content.contains("commit_as_fold(")
        || content.contains("commit_as_fold ->")
        || content.contains("commit_as_fold ");
    assert!(
        has_action,
        "T5: species MUST declare `commit_as_fold` action — the fold-then-commit substrate primitive"
    );
}

#[test]
fn t06_commit_as_fold_body_is_obligation_block() {
    let content = read_commit_as_fold_shard();
    let has_body = content.contains("commit_as_fold") && content.contains("{ \\ }");
    assert!(
        has_body,
        "T6: commit_as_fold body MUST be an obligation block `{{ \\\\ }}` per substrate-decl discipline."
    );
}

#[test]
fn t07_commit_as_fold_composes_action_cache_and_impacted_by() {
    let content = read_commit_as_fold_shard();
    // The fold IS a fold over verdict cache entries;
    // rebases walk impacted_by. Both must be cited.
    let has_cache =
        content.contains("action_cache") || content.contains("@mirror/store/action_cache");
    let has_impacted = content.contains("impacted_by");
    assert!(
        has_cache && has_impacted,
        "T7: commit_as_fold MUST cite composition with both `@mirror/store/action_cache` (fold subject) AND `impacted_by` (rebase walk). Cache=`{}`, impacted=`{}`",
        has_cache, has_impacted
    );
}

// === T8-T9: cli-verb-pair recognition + form/process partition ===

#[test]
fn t08_cites_cli_verb_pair_recognition_or_third_witness_shape() {
    let content = read_commit_as_fold_shard();
    // Third-witness gate: species must document its position in the
    // cli-verb-pair-specialises-species-action-pair recognition.
    let has_recognition = content.contains("cli-verb-pair")
        || content.contains("cli verb pair")
        || content.contains("kintsugi --commit")
        || content.contains("mirror kintsugi")
        || (content.contains("third witness") || content.contains("third-witness"));
    assert!(
        has_recognition,
        "T8: species narrative MUST cite the cli-verb-pair-specialises-species-action-pair recognition it witnesses (e.g. `mirror kintsugi --commit` maps to `commit_as_fold`)."
    );
}

#[test]
fn t09_cites_form_process_partition_or_mirror_store_git_sibling() {
    let content = read_commit_as_fold_shard();
    // #55 form/process partition: @kintsugi is process-side; @mirror/store/git
    // is form/state-observation side. commit_as_fold discharges through the
    // form-side sibling.
    let has_partition = content.contains("form/process")
        || content.contains("@mirror/store/git")
        || content.contains("mirror/store/git")
        || content.contains("partition")
        || content.contains("#55");
    assert!(
        has_partition,
        "T9: species narrative MUST cite the form/process partition (#55) or sibling `@mirror/store/git` — commit_as_fold sits on the process side."
    );
}

// === T10-T11: N-cascade positioning + terminal-tick discipline ===

#[test]
fn t10_cites_n_cascade_positioning() {
    let content = read_commit_as_fold_shard();
    let has_n_cascade = content.contains("N5")
        || content.contains("N-cascade")
        || (content.contains("N1") && content.contains("N4"));
    assert!(
        has_n_cascade,
        "T10: species narrative MUST position within the N-cascade (cites N1/N2/N3/N4/N5 or `N-cascade`)."
    );
}

#[test]
fn t11_cites_recognition_43_content_addressed_discipline() {
    let content = read_commit_as_fold_shard();
    let has_recognition = content.contains("#43")
        || content.contains("content-addressed")
        || content.contains("content addressed")
        || content.contains("mirror IS content-addressed");
    assert!(
        has_recognition,
        "T11: species narrative MUST cite Recognition #43 (mirror IS content-addressed build system) — commit_as_fold is a consumer."
    );
}

// === T12-T13: bilateral composition + exports ===

#[test]
fn t12_commit_as_fold_action_body_declared() {
    let content = read_commit_as_fold_shard();
    // The species has at least one action with an obligation block.
    // T5+T6 already check commit_as_fold; T12 asserts general shape.
    let has_prism_decl = content.contains("prism @kintsugi/store/git")
        || (content.contains("prism") && content.contains("@kintsugi/store/git"));
    assert!(
        has_prism_decl,
        "T12: species MUST declare via `prism @kintsugi/store/git <= ...` shape (species-decl discipline)."
    );
}

#[test]
fn t13_out_block_exports_commit_as_fold() {
    let content = read_commit_as_fold_shard();
    assert!(
        content.contains("out commit_as_fold"),
        "T13: `commit_as_fold` MUST appear in the `out` block at file tail — action is not consumable downstream otherwise."
    );
}

// === T14: git-projection altitude discipline ===

#[test]
fn t14_narrative_describes_git_projection_semantics() {
    let content = read_commit_as_fold_shard();
    // `mirror kintsugi --commit IS git commit` at git-projection altitude.
    // Narrative must document this projection semantics.
    let has_projection = (content.contains("git commit") || content.contains("git-commit"))
        && (content.contains("projection")
            || content.contains("altitude")
            || content.contains("IS"));
    assert!(
        has_projection,
        "T14: narrative MUST document git-projection altitude semantics — `mirror kintsugi --commit IS git commit at git-projection altitude` or equivalent."
    );
}

// === T15: Recognition #55 form/process narrative ===

#[test]
fn t15_narrative_grounds_transformation_versus_state_observation() {
    let content = read_commit_as_fold_shard();
    // commit_as_fold is transformation (@kintsugi side); it discharges to
    // state-mutation (@mirror/store/git side). Narrative must ground this.
    let has_grounding = content.contains("transformation")
        || content.contains("process")
        || content.contains("discharge")
        || content.contains("fold")
        || content.contains("@mirror/store");
    assert!(
        has_grounding,
        "T15: narrative MUST ground the transformation vs. state-observation partition — commit_as_fold IS the transformation; discharges to @mirror/store/git IS the state mutation."
    );
}
