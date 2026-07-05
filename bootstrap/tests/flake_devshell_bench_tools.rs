//! Phase A RED — flake.nix devShell additions per /loop 2026-07-05 Arc 3c
//! + Seam Phase D `91e79c8` §7 canonical execution.
//!
//! Adds performance tooling to flake.nix devShell:
//! - sccache (compilation cache; big win on incremental rebuilds)
//! - cargo-nextest (per-process test isolation; 30% typical speedup)
//! - cargo-audit (discharges the `audit` target in mirror.spec, currently
//!   partial via tool-unavailable carve-out per lib.rs:~1276)
//! - Platform linker (mold Linux / sold Darwin / lld fallback per Seam
//!   §8 signal-to-Alex #4)
//!
//! **Precondition**: Sub-arc 3a landed (bench dispatch); Sub-arc 3b
//! optional but recommended before 3c.
//!
//! **RED phase**: `flake.nix` currently declares `pkgs.git pkgs.just
//! pkgs.jq pkgs.openssl pkgs.zlib pkgs.gfortran pkgs.lapack pkgs.blas`
//! + darwin-only flang stack. No sccache, cargo-nextest, cargo-audit,
//! or platform linker.
//!
//! **GREEN phase** (Mara or Reed): add packages to devShell buildInputs.
//! Per Seam signal-to-Alex #4, mold/sold both add complexity; sccache +
//! nextest + audit are pure wins; conservative bundle includes one
//! linker (lld as cross-platform fallback OR platform-specific via
//! `pkgs.lib.optionals stdenv.isDarwin`).

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (repo root)")
        .to_path_buf()
}

fn read_source(rel: &str) -> String {
    let path = repo_root().join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {:?}: {}", path, e))
}

#[test]
fn flake_nix_has_sccache() {
    let content = read_source("flake.nix");
    assert!(
        content.contains("sccache"),
        "flake.nix devShell must include `sccache` per Seam `91e79c8` \
         §7 Sub-arc 3c. Compilation cache; big win on incremental \
         rebuilds. `[build] rustc-wrapper = \"sccache\"` opt-in via \
         `.cargo/config.toml` may follow in a later tick."
    );
}

#[test]
fn flake_nix_has_cargo_nextest() {
    let content = read_source("flake.nix");
    let has_nextest = content.contains("cargo-nextest") || content.contains("nextest");
    assert!(
        has_nextest,
        "flake.nix devShell must include `cargo-nextest` per Seam \
         `91e79c8` §7 Sub-arc 3c. Per-process test isolation; 30% \
         typical speedup; needed by /loop 2026-07-05 Arc 2 (thread-\
         safety Option A) once source migration lands."
    );
}

#[test]
fn flake_nix_has_cargo_audit() {
    let content = read_source("flake.nix");
    let has_audit = content.contains("cargo-audit") || content.contains("cargoAudit");
    assert!(
        has_audit,
        "flake.nix devShell must include `cargo-audit` per Seam \
         `91e79c8` §7 Sub-arc 3c. Discharges the `audit` target in \
         mirror.spec (currently PARTIAL via tool-unavailable carve-out \
         at bootstrap/src/lib.rs:~1276). Wiring cargo-audit into the \
         devShell converts audit verdict from `partial` to `success`."
    );
}

#[test]
fn flake_nix_has_platform_linker() {
    let content = read_source("flake.nix");
    // Per Seam §8 signal-to-Alex #4: mold Linux-only, sold Darwin-only,
    // lld cross-platform fallback. Conservative: at least one.
    let has_platform_linker =
        content.contains("mold") || content.contains("sold") || content.contains("lld");
    assert!(
        has_platform_linker,
        "flake.nix devShell must include per-platform linker (`mold` \
         Linux, `sold` or `lld` Darwin) per Seam `91e79c8` §7 Sub-arc \
         3c. Alex §8 signal-to-Alex #4 pending direction; conservative \
         landing includes at least one linker replacement."
    );
}
