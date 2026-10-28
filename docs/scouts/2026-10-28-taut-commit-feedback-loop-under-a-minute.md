# Taut scout: commit feedback loop under a minute

2026-10-28, Taut (grep-first drift scout). Worktree `/Users/alexwolf/dev/projects/mirror-taut-loop`, branch `taut/feedback-loop-scout` off `reed/ship@5c94cdf`.

Ask (Alex): "How can we speed up the feedback loop? What is taking so long? And how can we reduce the time?" Target: a `.rs` commit through the hooks in under 60s warm.

Prior art: Taut scout 2026-07-01 (Win 1 = debug build in the gate, `d323918`; Wins 1+5 = codegen units, `510f3a8`). That gate (`mirror kintsugi`) is gone. The interim gate (`8744181`) is five sequential `cargo test --manifest-path` runs. This scout covers the hook around it as well as the gate.

Legend: **M** = measured tonight. **R** = reconstructed from artifact mtimes in `rust/target` (read-only) of the 22:46 gate run. **E** = estimate, not measured.

## 0. The headline: the dominant stage is the macOS first-exec assessment queue

From about 22:40, **every exec of a freshly written file blocks for minutes** at `_dyld_start+0`. The kernel has mapped the image and is waiting on an exec-policy verdict from `syspolicyd`. The process is killable with SIGKILL.

| probe (M, 23:05-23:13) | result |
|---|---|
| `cp /usr/bin/true X; X` | >15s, timed out (twice) |
| same, `codesign -f -s -` re-signed | >12s, timed out |
| same, `xattr -d com.apple.provenance` | xattr reappears; >12s, timed out |
| fresh `#!/bin/sh` script, exec'd directly | >15s, timed out |
| same script run as `/bin/sh script` (no exec of the fresh file) | 0.18s |
| `just` linewise recipe | 0.09s |
| `just` **shebang** recipe (just writes a temp script and execs it) | >12s, timed out |
| `just` `[script("bash")]` recipe (temp file passed to bash) | 0.14s |
| previously-assessed binary (`~/.cargo-target/debug/mirror --version`) | 0.42s |
| `.githooks/commit-msg` in a fresh worktree | >10s, timed out |
| long probe: fresh `true`, started 23:08 | **1351.3s** (22.5 min) |
| this scout's own `📝` commit (2e8c032); the only fresh exec is the worktree's `.githooks/commit-msg` | **875s**; no cargo, no nix |
| Reed's probe: fresh `true` | **11m09.9s**, 0.00s user/sys |

What a `.rs` commit has to exec fresh: two just shebang recipes (`format`, `pre-commit`), twice over when the nix re-exec fires (an outer and an inner `just`); the repo hook after any checkout that rewrites it; every build script cargo relinks; every proc-macro dylib rustc dlopens (Reed's rustc jobs on `terni`, `zerofrom`, `criterion` sat at 0% CPU for 20+ min); and every relinked test binary (12 per gate run in `rust/target/debug/deps`). With a healthy syspolicyd, each of these costs a first-exec assessment. With a wedged one, the commit never finishes.

**Backlog, not deadlock** (Reed, M, 23:2x): a fresh copy of `/usr/bin/true` took **11m09.9s** to exec (0.00s user/sys), with syspolicyd at 64% CPU. Every new build script, proc-macro and test binary from every session (mirror, prism, others) joins one assessment queue.

**Fresh executables per `.rs` commit** (M; counted from `rust/target/debug`, read-only):

| kind | warm `.rs` commit (22:46 run) | cold | as a workspace (E) |
|---|---|---|---|
| test / bin executables relinked | **9**: `mirror` bin + unit-test bin, `red_spec_claims`, `red_liquid_pillar_i_commutator_antisymmetric`, `fractal`, `fire_d_measure`, `matrix`, `spectral`, `roomba` | 9-12 | 9 (one per test target; only fewer if integration tests are merged) |
| doctest binaries (rustdoc rebuilds them every run) | ~1-2 (3 lib crates with doc examples) | ~1-2 | same, or 0 with `doctest = false` where the doc examples aren't load-bearing |
| build-script binaries | 0 | **18** (prismqueer ×6 compiled, libc ×3, quote ×2, proc-macro2 ×2, ...) | ~10 (one per package) |
| proc-macro dylibs (dlopened by rustc) | 0 | **8** (`terni_macros` ×4, `prismqueer_projections` ×4) | 2 |
| just shebang temp scripts | 2 (path B) / 4 (path A) | same | 0 with `[script("bash")]` |
| repo hook after a checkout rewrote it | 0-1 | 1 | 0 with `bash "$repo_hook"` |
| **total** | **~12-16** | **~40-45** | warm ~10-11, cold ~22 |

cargo runs test binaries one after another, so under a backlog the waits stack. At tonight's 11-minute queue, even one fresh exec puts the commit far past target. The Justfile/hook changes cut 2-5 per commit; a workspace cuts about half of the cold set. **Only an exemption takes assessment out of the loop.** Without one, a warm commit still carries about 10 queued first-execs.

Which process to exempt. Assessment is skipped when the **responsible** process holds Developer Tools.
- iTerm already holds it, yet we still queue. So the tree's responsible process is probably not iTerm.
- nix-daemon is not in the tree: cargo, rustc and the test binaries are children of `claude`, not of the daemon.
- Candidates: `iTermServer-3.6.11` (if iTerm disclaims responsibility when spawning it), or `claude` itself.

Two checks settle it:
- **Root:** `sudo launchctl procinfo <pid of claude> | grep -i responsible`.
- **No-root bisect (Alex, 2 min):** in an iTerm tab as `alexwolf`, no sudo, run `cp /usr/bin/true /tmp/x1 && time /tmp/x1`. In the same tab, run `sudo -u reed sh -c 'cp /usr/bin/true /tmp/x2 && time /tmp/x2'`.
  - Both instant: iTerm's grant works, and the claude process tree is what escapes it.
  - First instant, second slow: the sudo/uid hop breaks it.
  - Both slow: the iTerm grant isn't in effect (responsibility goes to iTermServer).

Then add the identified binary in System Settings → Privacy & Security → Developer Tools ("+"). For `claude`, the Caskroom path is versioned (`claude-code@latest/2.1.283/claude`), so re-check after updates. A stable wrapper as the session entry point would keep the grant from going stale.

System state at 23:04-23:12 (M):
- `syspolicyd` (pid 76996, restarted 22:21:16) is in state `U` (uninterruptible). So are `XprotectService` (78904, started with the restart) and `mds_stores`.
- `kTCCServiceDeveloperTool|com.googlecode.iterm2|2` **is** granted in the system TCC.db, yet the hang persists.
- The Claude tree is `iTerm2 → iTermServer-3.6.11 → login → zsh(alexwolf) → sudo → sudo → claude (uid reed)`. The Developer Tools exemption is keyed on the **responsible** process. If iTerm spawns `iTermServer` with responsibility disclaimed, the responsible binary is `iTermServer-3.6.11`, whose identity is not `com.googlecode.iterm2`, and the iTerm grant never applies. **Hypothesis, not verified** (needs root, see §4).
- Disk: a `ugrep` (pid 13175, child of claude pid 18590 via zsh 13174, running 1h48m at ~135% CPU) is crawling `/Users/reed/identity` including `.git.backup/objects` at ~147 MB/s and ~37k IOPS (`iostat`). An I/O-bound syspolicyd, XprotectService and mds_stores next to a saturating crawler is consistent with the scanner starving. **Correlation, not proven cause.** Not Taut's process, so not killed.
- `log show` as `reed` returns "Could not open local log store: Operation not permitted", so the syspolicyd log needs Alex.

## 1. Timeline of one `.rs` commit, stage by stage

Path A: `git commit` from a shell without `IN_NIX_SHELL` (`git -C`, a fresh Claude Code Bash call, a non-direnv shell). Path B: from a direnv-loaded shell (re-exec guard is a no-op).

| # | stage | A | B | source |
|---|---|---|---|---|
| 1 | household hook: marker checks, `just --list` ×2-3 | ~0.4s | ~0.4s | M (0.13s per `just --list`) |
| 2 | repo guard `.githooks/commit-msg` | ~0.1s; **hangs if fresh** | same | M |
| 3a | `just format`: shebang exec | assessment; **hangs while wedged** | same | M |
| 3b | `nix develop --command just format` on a dirty tree | **37-96s** (3 runs: 96.5, 80.9, 36.9) | 0 | M |
| 3c | `cargo fmt --all` | 2.0s | 2.0s | M |
| 4 | `run_staged` bookkeeping (stash create, diff, ls-files, checkout) | ~0.9s | ~0.9s | M (0.30+0.20+0.20+0.15) |
| 5a | `just pre-commit`: shebang exec | assessment; **hangs while wedged** | same | M |
| 5b | `nix develop --command just pre-commit` | **37-96s** | 0 | M |
| 5c | cargo, rust/ graph: rebuild path crates + mirror, link test bins | ~16s | ~16s | R (22:45:58 to 22:46:14) |
| 5d | cargo, rust/ tests run | ~29s (Alex saw ~18s) | same | R |
| 5e | cargo, fractal + matrix + spectral + roomba graphs, serial | ~23s+ | same | R (22:46:43 to 22:47:06, plus roomba tests) |
| 6 | restore (git apply, mv untracked, stash drop) | <1s | <1s | E |
| | **warm total** | **~150-270s** | **~72s** | |

Outside the hook, every Claude Code Bash call that `cd`s into the repo as `reed` triggers direnv `use flake`. `reed` has no nix-direnv (`~/.config/direnv` doesn't exist; `~/.os/home/alexwolf/shell.nix:166` enables it for `alexwolf` only). That is a full `nix print-dev-env` on a dirty tree: **84.96s wall, 2.92s CPU** (M). This is the "taking a while to execute" message.

Why nix is slow (M): a dirty tree disables the flake eval cache. During a commit the tree is always dirty relative to HEAD, including inside `run_staged`, because index ≠ HEAD. Each eval copies the ~50 MB tracked tree into the store (`docs/math/**/prior-art/*.pdf`, 8.5 MB + 1.9 MB; `void.png`, 1.4 MB) and instantiates 2817 derivations with 392 store copies. CPU is about 3s; the rest is waiting on nix-daemon store writes. With a clean tree and a warm eval cache: 2.3-5.1s. Sourcing a cached `print-dev-env` rc: **1.0s**.

**Cold cargo was not measured.** The wedge blocks every build script. Structure (M, from `Cargo.lock` × 5 and the build dir):
- 174 package resolutions across five lockfiles, 55 unique name+version pairs, 46 unique names.
- Lockfiles diverge: `cc` 1.3.0 (rust/) vs 1.4.0 (matrix/spectral/roomba); `libc` 0.2.186 vs 0.2.189; `proc-macro2`, `quote`, `fastrand` likewise. `fractal` uses prismqueer without `lapack,bundle`.
- So prismqueer builds in ≥3 variants. `rust/target/debug/build` has 12 `prismqueer-*` dirs; 3 contain the gfortran `libspectral_native.a`. Historical cargo-timings: prismqueer lib 13.1s + 6.5s, build-script run 2.1s per variant.
- `fractal` built three times in the 22:46 run: three distinct rlib hashes, one per graph (rust, fractal, matrix).

## 2. Ranked levers

| rank | lever | saving | cost / risk | where |
|---|---|---|---|---|
| 1 | **Fix the fresh-exec wedge.** Find the real responsible process, grant *it* Developer Tools, stop disk-saturating crawls (§4). | ∞ → 0 while wedged; per-binary assessment × ~20+ fresh execs per commit when healthy (unmeasured) | Low; diagnosis needs root | Alex / macOS |
| 2 | **Get `nix develop` off the hot path.** Justfile guard: instead of `exec nix develop --command just X`, source a cached rc keyed on `sha256(flake.nix flake.lock)`, built once with `nix print-dev-env --profile .direnv/devshell` (GC-rooted). Rebuild only when the key changes. Plus nix-direnv for `reed`. | **74-192s per commit** on path A (M: 2× 37-96s → 2× ~1s); **~85s per repo `cd`** for reed | Low; stale env only if the key misses an input (the devShell reads only flake.nix/flake.lock) | Justfile (mirror) + `~/.os` (nix-direnv for reed) |
| 3 | **`[script("bash")]` instead of shebang** on `format`, `pre-commit` and the other hook recipes. | removes 2-4 fresh execs per commit; 0.14s vs hang (M) | Trivial; just 1.46 needs no `set unstable` (M) | Justfile (mirror) |
| 4 | **Make rust/ a cargo workspace:** one lock, `cargo test --workspace`. | Warm: most of stage 5e, ~15-23s (E from R: path crates rebuilt once, not 2-3×; one fingerprint pass; test bins built in parallel instead of 5 serial phases). Cold: 2 of 3 prismqueer+Fortran builds, ~40s (E). Fewer fresh binaries. | Medium. Unify `cc`/`libc`/`proc-macro2`/`quote` versions. Feature unification gives `fractal` the lapack prismqueer. Repoint flake `cargoLock.lockFile` to the workspace lock. `rust/Cargo.toml:5` records a deliberate deferral ("post-dock workspace lift MAY land"), so read that reason first. | rust/ (mirror) |
| 5 | **Format only staged `.rs`, and land one fmt-clean commit.** `just format` takes the hook's staged list; `rustfmt --edition 2021 <files>` is 0.06s/file (M). | 2s direct (M). Indirect: no more rebuild churn. `cargo fmt --all --check` at HEAD shows 180 hunks, in exactly the files that sit ` M` unstaged in Reed's tree (`fractal/src/{crystal,singularity,subject}.rs`, `fractal/tests/fire_d_measure.rs`, `matrix/src/book.rs`, `spectral/src/liquid.rs`, `src/{apply_h,compile,phone}.rs`, `tests/red_spec_claims.rs`). `run_staged`'s `checkout -- .` and the restore bump their mtimes twice per commit, so cargo recompiles crates with no staged change. In the 22:46 run, fractal and spectral (unstaged edits only) rebuilt in every graph (R). ~5-15s warm (E). | Low. **Correctness fix too:** the hook stops rewriting files outside the staged set. | Justfile + hook (`~/.os`) |
| 6 | **Test only the staged diff's reverse-dependency closure.** roomba-only → `-p roomba -p mirror`; fractal → all; any Cargo.toml/lock → all. | 0-25s depending on the diff (E) | Low with a workspace (static map of path deps); needs a "Cargo change → full" fallback | Justfile (mirror) |
| 7 | **Run the gate in a persistent shadow worktree of the index**, not `git checkout -- .` on the user's tree. `git -C .git/hook-tree checkout --detach $(git write-tree)` rewrites only changed files, so mtimes are stable. Use its own persistent `CARGO_TARGET_DIR`. | Removes the mtime churn at the source (stacks with #5). Lets agents commit while another agent edits the same tree. | Medium; one more target dir (~600 MB) | hook (`~/.os`) |
| 8 | Test-profile tweaks: `[profile.dev] debug = "line-tables-only"`; lld (already in the devShell). | Link time on 12 test bins, unmeasured; typically 10-30% of link (E) | Low; worse backtraces | rust/ |
| 9 | sccache / nextest | sccache helps cold dependency builds only (path crates and linking aren't cached). nextest only pays if a few tests dominate the 18-29s run, **unmeasured** (wedge). | Low | later |

## 3. A concrete path to <60s warm

1. #3 + #2 (one Justfile commit, ~20 lines): hook overhead goes from 74-192s (path A) to ~3s. Path B is already there.
2. #5: one fmt-clean `♻️` commit, then a format recipe that takes files. Stops cross-crate churn.
3. #4: workspace + `cargo test --workspace`. Warm cargo estimate (from R): edited crate and dependents compiled once (~10-16s), test bins linked in parallel, mirror tests ~18-29s, other crates' tests ~5-10s. **≈ 35-50s.**
4. #6 on top for leaf-crate commits (roomba, spectral): ~20-30s (E).

Projected warm `.rs` commit ≈ 3s hook + 35-50s cargo = **~40-55s** (E). This needs re-measuring once the wedge clears, especially whether a few slow tests dominate the 18-29s mirror run.

None of this is under 60s while #1 stands: one wedged first-exec makes the commit unbounded.

## 4. What needs Alex vs what lands in mirror

**Alex (root / `~/.os`):**
1. Diagnose the wedge:
   - `sudo log show --last 1h --predicate 'process == "syspolicyd" OR process == "XprotectService" OR subsystem == "com.apple.syspolicy.exec"' > /tmp/sp.log`
   - `sudo launchctl procinfo 18590 | grep -i responsible`: is it `iTerm2` or `iTermServer-3.6.11`?
   - `sudo sample 76996 5`: what is syspolicyd blocked on?
2. If the responsible process is iTermServer (or `login`/`sudo`), add *that* binary to System Settings → Privacy & Security → Developer Tools. Don't disable Gatekeeper globally.
3. Stale crawler: `ugrep` pid 13175 belongs to Reed's own claude (18590); Reed can kill it. Keep `.git.backup` out of recursive greps.
4. `~/.os`: enable nix-direnv for user `reed` (today it's `alexwolf` only, `shell.nix:166`).
5. `~/.os` git-hooks.nix:
   - run the repo hook as `bash "$repo_hook"`, not `"$repo_hook"` (no fresh exec after checkouts);
   - pass the staged `.rs` list to `just format` and stop redirecting its stderr to `/dev/null` (that hides the 40-90s silent nix wait);
   - lever #7, a shadow worktree in place of `git checkout -- .`.

**Mirror repo (Reed / Pack, no Alex needed):** levers #2 (Justfile half), #3, #4, #5 (format recipe + fmt-clean commit), #6, #8.

## 5. Method and caveats

- Every measurement ran in the Taut worktree under `/Users/alexwolf/dev/projects/` (so the `.cargo/config.toml` `[patch]` for prism applies). The gate's cargo stage was **not** run: the first attempt hung at exec of its own fresh harness script (pid 16206 at `_dyld_start`; killed). Nothing ran in Reed's working tree. `rust/target` there was only read (mtimes, file lists).
- Load average was 8-10 on 8 cores throughout: Reed's builds, prism's builds, and the crawler. Nix timings carry daemon contention (37-96s spread). The 2.3-5.1s clean-tree figures are the floor.
- R numbers come from one warm run (22:45:58-22:47:06). Its staged set (matrix, roomba, src/love.rs, src/main.rs, one test) is typical of tonight's commits.
- Open measurements once the wedge clears: cold `just pre-commit`; warm no-op; warm one-crate edit; same after a workspace spike; per-test timings for the mirror suite; per-binary first-exec assessment latency when syspolicyd is healthy.
