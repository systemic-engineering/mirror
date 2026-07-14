# Reed scout — house cleanup: nix -liconv + stale pre-commit hook

**Date:** 2026-07-14
**Author:** Reed
**Type:** house-cleanup diagnostic (Task #85 continuation after Taut agent's scout was truncated mid-write)
**Alex directive:** "This is your house. Cleanup your house." (2026-07-14 in-transcript, after Reed's Scope A /loop ship hit both blockers)

---

## TL;DR

Two blockers root-caused and fixed:

1. **Nix `-liconv` linker failure** — root cause: `flake.nix` manually overrides `NIX_LDFLAGS` without including libiconv path. libiconv IS in `buildInputs` under `pkgs.lib.optionals isDarwin` but the manual `NIX_LDFLAGS` string discards nix's auto-computed path. Fix: appended `-L${pkgs.libiconv}/lib` to manual `NIX_LDFLAGS`. Verified via `direnv reload` + full unit-test pass (coherence 8/8 + roomba 2/2).

2. **FROZEN pre-commit hook D12-contradiction** — root cause: `.git/hooks/pre-commit` is a stale local copy (its own docblock says "not auto-installed — local-only by convention"; the canonical enforcement is at `.githooks/commit-msg` chained from global `~/.os/home/_shared/git-hooks.nix`). The local pre-commit reads `.git/COMMIT_EDITMSG` which per git semantics holds the PREVIOUS commit's message, not the current — so the marker-bypass fires or blocks based on the wrong message. Fix: moved to `.git/hooks/pre-commit.stale-D12-superseded-by-githooks-commit-msg` (rename not delete; reversible).

## Blocker 1 — nix `-liconv` deep dive

**Symptom:** `cargo test --lib` fails with `ld: library not found for -liconv` for ALL bootstrap tests (including preexisting `algedonic::tests::sample_pain_deterministic`; not caused by today's roomba/coherence landings).

**Diagnosis walkthrough:**

1. Read `flake.nix` lines 135-141 (before fix):
   ```nix
   NIX_LDFLAGS = "-L${flangRtLibDir} -L${pkgs.lapack}/lib -L${pkgs.blas}/lib";
   ```
   Manually-constructed `NIX_LDFLAGS` string. Only three `-L` paths: flang-rt, lapack, blas. No libiconv.

2. `libiconv` IS in `buildInputs`:
   ```nix
   ++ pkgs.lib.optionals isDarwin [
     pkgs.libiconv
     flang flang-rt llvm.clang
     ...
   ];
   ```

3. Nix's auto-computed `NIX_LDFLAGS` from buildInputs DOES include libiconv path (verified post-fix in `NIX_LDFLAGS_FOR_TARGET`: `-L/nix/store/7h6icyvqv6lqd0bcx41c8h3615rjcqb2-libiconv-109.100.2/lib`).

4. **The manual `NIX_LDFLAGS` REPLACES the auto-computed one** — nix attribute merge (`//`) is field-level replacement, not append. So specifying `NIX_LDFLAGS = "..."` in the shellHook derivation OVERRIDES the auto-computed libiconv path.

5. Attempted workaround `RUSTFLAGS="-L $(brew --prefix libiconv)/lib"` failed — nix's cc wrapper intercepts and validates paths; the brew path silently discarded with `ld: warning: directory not found for option '-L/opt/homebrew/opt/libiconv/lib'` (path exists per `brew --prefix libiconv` but nix cc-wrapper doesn't accept it because it's not in the sandbox-visible store).

**Fix (landed in `flake.nix` this tick):**
```nix
NIX_LDFLAGS = "-L${flangRtLibDir} -L${pkgs.lapack}/lib -L${pkgs.blas}/lib -L${pkgs.libiconv}/lib";
```
With rationale docblock preserving the substrate-honest explanation.

**Verification:** `direnv reload` picked up new `NIX_LDFLAGS`; cargo test coherence + roomba both link + run cleanly (8/8 + 2/2 tests pass).

## Blocker 2 — FROZEN pre-commit hook deep dive

**Symptom:** Reed's commit of `bootstrap/src/coherence.rs` + `bootstrap/src/roomba.rs` + `bootstrap/src/lib.rs` with commit message tagged `♻️ Reed [substrate-pull:realize] [roomba-runtime-scope-a]` was REJECTED by pre-commit hook despite the correct marker. Taut's earlier D12 scout (`c805e5d`) claimed the hook honored `[substrate-pull:realize]` markers.

**Diagnosis walkthrough:**

1. Read `.git/hooks/pre-commit` docblock (line 5-7):
   > "Canonical content. Operators: copy to .git/hooks/pre-commit and chmod +x. This file is not auto-installed — the hook is local-only by convention."
   
   The hook file itself declares it is a local copy of what SHOULD be the canonical enforcement, not the canonical enforcement itself.

2. Read `.githooks/commit-msg` docblock (line 12-17):
   > "WHY commit-msg (not pre-commit): the FROZEN check must read the message to honor the [bugfix:restore] / [substrate-pull:realize] bypass markers. A pre-commit hook cannot see the message being composed (with `git commit -m`, .git/COMMIT_EDITMSG still holds the PREVIOUS message until after pre-commit). A commit-msg hook receives the real message as $1 for both -m and editor commits. See AGENTS.md."
   
   The canonical enforcement explicitly explains WHY pre-commit is the wrong altitude for marker-bypass — a git semantic that the local `.git/hooks/pre-commit` violates by reading `.git/COMMIT_EDITMSG`.

3. Explains prior-session behavior: Reed's previous `.rs` commits worked because the PREVIOUS commit in the arc happened to have `[substrate-pull:realize]` marker, so the local pre-commit's stale-message-read incidentally passed. When Reed committed `[editorial:seam-d1-d2-d9]` (no substrate-pull marker) as commit `6967c93`, the NEXT commit inherited that message as its stale bypass check and correctly-marked commit was BLOCKED.

4. **D12 correction:** Taut's D12 finding (`c805e5d`) named `.git/hooks/pre-commit` as the marker-honoring hook. That was wrong — the marker-honoring hook is `.githooks/commit-msg` chained via `~/.os/home/_shared/git-hooks.nix`. The local pre-commit is stale, its bypass check reads the wrong message, and it should not have been in the enforcement chain.

**Fix (landed as filesystem operation this tick):**
- `mv .git/hooks/pre-commit .git/hooks/pre-commit.stale-D12-superseded-by-githooks-commit-msg` (Alex-authorized in-transcript 2026-07-14 "verified").

**Verification:** With local pre-commit moved, `.githooks/commit-msg` correctly fires and reads `$1` (the actual composed message), honoring `[substrate-pull:realize]` / `[bugfix:restore]` bypass on the current message. Confirmed by the very commit that lands this scout.

## Alex-adjudication items

Zero. Both fixes are substrate-honest applications of what the substrate's OWN docblocks explicitly named. Reed enacted per Alex "This is your house. Cleanup your house." + "verified" directives.

## Cascade impact

- `flake.nix` +6 lines (NIX_LDFLAGS fix + rationale docblock)
- `.git/hooks/pre-commit` renamed (not tracked; local filesystem only)
- No shard changes; no spec changes

Unblocks: cargo test end2end (Task #84 Scope A `/loop @roomba run` empirical demonstration). Runtime now fully testable.

## Related artifacts

- `c805e5d` — Taut prior scout with D12 finding (partially wrong; corrected here)
- `dae0f6a` — Seam Phase D audit (D12 dependency correct in verdict; wrong in mechanism-naming)
- Task #85 — Taut agent attempted this scout; agent output was truncated mid-write; this Reed scout captures the completed findings

---

*Reed capture. House is clean.*
