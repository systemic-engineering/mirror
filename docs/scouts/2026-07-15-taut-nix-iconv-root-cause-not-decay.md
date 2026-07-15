# Taut root-cause scout — nix `-liconv` linker failure was insufficient, not decayed

**Date:** 2026-07-15
**Author:** Taut (read-only grep scout)
**Trigger:** `cargo test --test crystallize_persistence` failed at link time with `ld: library not found for -liconv` while Reed was landing Bridge γ. Alex flagged that Task #85 (2026-07-14) had "fixed" this. Reed spawned Taut to root-cause the recurrence.
**Vocabulary constraint:** "Workaround" is forbidden (Alex 2026-07-15). Root-cause fixes only.

---

## 1. Previous fix inventory (Task #85, 2026-07-14)

- **Commit:** `422076d` — house-cleanup bundle
- **Doc:** `docs/scouts/2026-07-14-reed-house-cleanup-nix-iconv-and-stale-precommit.md`
- **Change:** added `-L${pkgs.libiconv}/lib` to the manual `NIX_LDFLAGS` assignment in `flake.nix:130`, alongside pre-existing `-L${flangRtLibDir}`, `-L${pkgs.lapack}/lib`, `-L${pkgs.blas}/lib`
- **Comment claim:** "Manual NIX_LDFLAGS OVERRIDES nix's auto-computed path from buildInputs"

## 2. Decay analysis — INSUFFICIENT, not decayed

The fix is still in-tree unchanged since 2026-07-14. It was insufficient from the start.

**Mechanism of insufficiency:** `nix.mkShell` does NOT treat manual `NIX_LDFLAGS` assignments as overrides. It **appends** the buildInputs' auto-computed library paths to any manual `NIX_LDFLAGS` assignment, producing a 1700+ char cascade with duplicates. The manual libiconv path IS in the resulting string, but the string as a whole exceeds the cc-wrapper's validation + darwin linker's ordering tolerances.

**Second-order cause:** cargo's `rustc` invocation does NOT consume `NIX_LDFLAGS` directly for the -L search paths passed to `cc`. It consumes `RUSTFLAGS`. The prior fix targeted the wrong environment-variable mechanism.

## 3. Actual root cause

- **Who requests `-liconv`:** `openssl-sys` + `libgit2-sys` build.rs emit `cargo:rustc-link-lib=iconv` on darwin. Substrate-correct — iconv IS required.
- **Who should provide iconv:** `pkgs.libiconv` in `buildInputs` (already present).
- **The gap:** cargo needs `-L <libiconv path>` in its own linker invocation. `RUSTFLAGS` is the vehicle cargo respects. `NIX_LDFLAGS` was addressing a different (nix-cc-wrapper) mechanism that doesn't reach cargo's rustc.

## 4. Durable fix

Replace the manual `NIX_LDFLAGS` assignment in `flake.nix` with a `RUSTFLAGS` assignment containing the same -L paths:

```nix
RUSTFLAGS = "-L ${flangRtLibDir} -L ${pkgs.lapack}/lib -L ${pkgs.blas}/lib -L ${pkgs.libiconv}/lib";
```

**Why this is structurally durable:**
- Cargo processes `RUSTFLAGS` directly; no nix mkShell post-processing; no cascade.
- Paths are explicit and versioned via nix-store references (buildInputs entries).
- The mechanism matches the actual consumer (cargo/rustc), not an adjacent-altitude env var.
- Future additions append cleanly; no fighting nix semantics.

**Why the prior fix will not repeat:** naming the mechanism honestly (RUSTFLAGS for cargo, NIX_LDFLAGS for nix-cc-wrapper) makes it structurally clear which env var goes where. The 2026-07-14 fix was mechanism-mismatched; the 2026-07-15 fix is mechanism-matched.

## 5. Verification path

1. `direnv reload` at repo root
2. `echo $RUSTFLAGS` should include the libiconv path
3. `cargo test --test crystallize_persistence` should compile + link + pass (**verified 2026-07-15 20:37: 3/3 tests pass**)
4. `cargo build --release` should link without `-liconv` errors

## Meta-lesson

When a fix "decays," first re-examine whether it was ever load-bearing. Prior-fix + return-of-symptom is stronger evidence of mechanism-mismatch than of decay. The vocabulary "workaround" (Alex 2026-07-15 forbidden term) captures exactly this failure mode: a symptom-suppressor that doesn't address the root cause looks like a fix until the mechanism reasserts itself.
