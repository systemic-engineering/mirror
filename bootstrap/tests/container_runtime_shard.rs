//! Arc 5 TICK 5 RED — `@container/runtime` species with `runtime_daemon_absent` predicate.
//!
//! **Directly resolves StageFreight-daemon blocker (task #540)** via Splinter-pole:
//! podman / buildah / containerd-direct dispatch WITHOUT requiring a docker daemon.
//!
//! Per Mara spec `docs/specs/docker-container-substrate-decl-v0.1.md` (`ec636d3`)
//! §6.3 + §10.4. Under `@container` family-root at runtime altitude.
//!
//! Post cascade:
//! - TICK 1 `aaa9a81` @container family-root
//! - TICK 2 `49f0486` @kintsugi/shift substrate-decl primitive
//! - TICK 3 `b66b280` @container/image first witness
//! - TICK 4 `a1fb4bd` @code/docker species (three-surface partition closed)
//! - TICK 5 (this): @container/runtime with runtime_daemon_absent

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn read_runtime_shard() -> String {
    let path = repo_root().join("shards/container/runtime.mirror");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read shards/container/runtime.mirror at {:?}: {}", path, e))
}

// === T1-T8: canonical species shape + daemon-absent discipline ===

#[test]
fn t01_container_runtime_declares_species_prism() {
    let content = read_runtime_shard();
    assert!(
        content.contains("prism @container/runtime"),
        "T1: must declare `prism @container/runtime` species per path-namespace pact"
    );
}

#[test]
fn t02_container_runtime_declares_five_op_body() {
    let content = read_runtime_shard();
    for op in ["focus", "project", "split", "shift", "settle"] {
        assert!(
            content.contains(op),
            "T2: prism @container/runtime must declare five-op body operation `{}`",
            op
        );
    }
}

#[test]
fn t03_runtime_declares_runtime_backend_carrier() {
    let content = read_runtime_shard();
    assert!(
        content.contains("type runtime_backend = ref") || content.contains("runtime_backend: ref"),
        "T3: must declare `runtime_backend` typed carrier (identifies which runtime: docker / podman / buildah / containerd)"
    );
}

#[test]
fn t04_runtime_declares_daemon_absent_predicate() {
    let content = read_runtime_shard();
    // THE key predicate that resolves the StageFreight blocker
    assert!(
        content.contains("runtime_daemon_absent"),
        "T4: must declare `runtime_daemon_absent(rt: runtime_backend) -> verdict` — the Splinter-pole discipline; daemonless runtimes (podman/buildah/containerd-direct) discharge PASS"
    );
}

#[test]
fn t05_runtime_declares_spawn_or_run_action() {
    let content = read_runtime_shard();
    // Spawn container from image + runtime
    assert!(
        content.contains("spawn(") || content.contains("run(") || content.contains("exec("),
        "T5: must declare a typed action (spawn / run / exec) that takes image_witness + runtime_backend and produces a running container"
    );
}

#[test]
fn t06_runtime_composes_with_container_image() {
    let content = read_runtime_shard();
    assert!(
        content.contains("@container/image"),
        "T6: must compose with `@container/image` (TICK 3 `b66b280`) — runtime consumes image_witness at container altitude"
    );
}

#[test]
fn t07_runtime_names_splinter_pole_daemonless() {
    let content = read_runtime_shard();
    // Splinter-pole = the K_n complete-graph pole vs Narcissus K_{1,n-1} star-graph pole
    // Daemonless runtimes = Splinter-pole (peer-to-peer, no central hub)
    // Docker-with-daemon = Narcissus-pole-adjacent (hub-and-spoke)
    let has_splinter_framing = content.contains("Splinter") || content.contains("splinter") || content.contains("daemonless") || content.contains("podman") || content.contains("buildah");
    assert!(
        has_splinter_framing,
        "T7: narrative must name Splinter-pole / daemonless discipline (podman / buildah / containerd-direct) — the substrate-honest resolution of StageFreight-daemon blocker (#540)"
    );
}

#[test]
fn t08_runtime_cites_stagefreight_blocker_resolution() {
    let content = read_runtime_shard();
    // Explicit citation of the blocker this species resolves
    let cites_blocker = content.contains("StageFreight") || content.contains("stagefreight") || content.contains("#540") || content.contains("daemon blocker");
    assert!(
        cites_blocker,
        "T8: narrative must cite the StageFreight-daemon blocker (#540) that this species resolves via runtime_daemon_absent Splinter-pole path"
    );
}

// === T9-T14: Interpretation B + inheritance discipline ===

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

#[test]
fn t09_first_nonempty_line_is_narrative_docblock() {
    let content = read_runtime_shard();
    let first = first_nonempty_line(&content).expect("T9: must have non-empty content");
    assert!(
        first.trim_start().starts_with('#'),
        "T9: first non-empty line must be `#`-narrative per Interpretation B; got `{}`",
        first
    );
    assert_ne!(first.trim(), "---", "T9: line-1 `---` is drift");
    assert!(
        !first.trim_start().starts_with("in "),
        "T9: `in @...` clauses live BELOW the seam"
    );
}

#[test]
fn t10_exactly_one_seam_at_column_zero() {
    let content = read_runtime_shard();
    let seams = seam_line_indices(&content);
    assert_eq!(
        seams.len(),
        1,
        "T10: exactly one `---` at column 0 required; found {}",
        seams.len()
    );
}

#[test]
fn t11_in_clauses_below_seam() {
    let content = read_runtime_shard();
    let seams = seam_line_indices(&content);
    let seam_idx = *seams.first().expect("T11 depends on T10 (one seam present)");
    for (i, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("in @") {
            assert!(
                i > seam_idx,
                "T11: `in @...` at line {} appears ABOVE seam at line {}. Line: `{}`",
                i + 1,
                seam_idx + 1,
                line
            );
        }
    }
}

#[test]
fn t12_runtime_inherits_universal_prism_meta_glass() {
    let content = read_runtime_shard();
    for req in ["in @prism", "in @meta", "in @glass"] {
        assert!(
            content.contains(req),
            "T12: must inherit `{}`",
            req
        );
    }
}

#[test]
fn t13_runtime_inherits_container_parent_family() {
    let content = read_runtime_shard();
    assert!(
        content.contains("in @container") && !content.contains("in @container/runtime"),
        "T13: must inherit `in @container` (parent family per path-namespace pact); species does not inherit itself"
    );
}

#[test]
fn t14_runtime_narrative_names_process_side_placement() {
    let content = read_runtime_shard();
    // @container is form-side per #55; runtime is where process happens
    // Runtime species specifically deals with process-side operations (spawn/run/exec)
    assert!(
        content.contains("process") || content.contains("runtime") || content.contains("lifecycle"),
        "T14: narrative must name process-side / runtime / lifecycle discipline"
    );
}
