# Seam Phase D — Arc 2 + Arc 3 combined (commit `7d1ec39`)

Adversarial audit; RATIFY-WITH-CORRECTIONS. Reed-decidable.

## §1 Verdict

**RATIFY-WITH-CORRECTIONS.** Substrate advance is real: `Ctx` threads
cwd explicitly through the dispatch chain, `kintsugi_main_lock` retires,
concurrent test-thread cwd race is structurally impossible, bench floor
lands, cargo-audit + sccache + nextest + lld in devShell. Six corrections
are minor (2 stale non-`_in` call sites, 1 dead-code non-Ctx entry,
1 unused fn, 1 docblock reference to the retired mutex, 1 process-wide
`set_current_dir` still in `mcp.rs` outside dispatch chain). None
Rice-hazardous at Arc 2's scope; fold-forward to Arc 4 is safe.

## §2 Rice-hazard scan

**Dual API (`load_grammar` vs `load_grammar_in`, etc.):** legacy variants
are retained as public compat wrappers with docstrings that point new
callers at the `_in` variant. Substrate-pull-honest as a migration path.
BUT two live non-`_in` call sites remain outside the dispatch chain:
`bootstrap/src/pipeline.rs` (`tokenize_with_ref` uses
`grammar_path_for_ref` + `load_grammar`) and `bootstrap/src/spectral.rs`
(fold-reducer uses `grammar_path_for_ref` + `load_grammar`). These are
called from paths reached by dispatch. They read against process cwd,
not `ctx.cwd()`. Not a hazard TODAY (Arc 2 tests pass without
`--test-threads=1`), but a latent implicit-process-cwd surface future
sub-agents could regress on.

**`Ctx::command`:** sets `.current_dir(ctx.cwd())` only; process env
inherits by default (Rust's `Command::new` semantics). No Rice hazard;
env override is a future extension, not a current gap.

**`dispatch(` call sites:** one — `src/main.rs`. Legacy single-arg
signature is gone. Clean.

**`kintsugi_main_lock` references:** zero live code references; one
stale mention in `bootstrap/tests/thread_safety_option_a.rs:~7-8`
DOCSTRING (test now passes — the docstring is describing the RED state it
was written against).

## §3 Arc discipline

Reed's template called for two commits (Arc 2 first, Arc 3 rebased).
Mara combined. Both mutate `bootstrap/src/lib.rs`; hunks are
non-overlapping (Arc 2: lines 93/357/451/etc.; Arc 3: `cargo_args_for_check`
at line ~1181) and structurally independent. A split was mechanically
possible; the combined commit is not substrate-dishonest — pre-commit
`mirror kintsugi mirror.spec` returned `partial` (gate-passing), local
`cargo test` green. Judgment call within Mara's discretion; the arc
message names both scopes explicitly. Accept.

## §4 Test discipline verify

Ran `cargo test --test thread_safety_option_a --test spec_target_bench
--test mirror_spec_bench_target --test flake_devshell_bench_tools`:

- `thread_safety_option_a` **6/6 pass** (ctx_carries_cwd, dispatch_takes_ctx,
  grammar_loader_takes_ctx, kintsugi_main_in_does_not_set_process_cwd,
  kintsugi_main_lock_removed, lib_declares_ctx_type)
- `flake_devshell_bench_tools` **4/4 pass**
- `mirror_spec_bench_target` **3/3 pass**
- `spec_target_bench` **2/2 pass**

Total **15/15** across the two REDs.

## §5 Corrections required

1. **`cmd_dump` (`bootstrap/src/lib.rs:~796`)** — `#[allow(dead_code)]`
   sibling still uses `grammar_for_file` + `load_grammar`. Thread `ctx`
   or delete.
2. **`pipeline.rs::tokenize_with_ref`** — thread `ctx` through
   `grammar_path_for_ref_in` + `load_grammar_in`. Live via dispatch.
3. **`spectral.rs` fold-reducer (`~line 619`)** — same fix; thread `ctx`.
4. **`bootstrap/src/mcp.rs:~703`** — `std::env::set_current_dir(&home)`
   on MIRROR_HOME. Outside dispatch chain but process-wide cwd mutation
   still lives; Arc 2 intent is zero `set_current_dir` in the mirror
   binary. Consider a follow-up tick.
5. **`parse_ci_format` (`src/lib.rs:1546`)** — dead-code warning. Delete
   or wire.
6. **`thread_safety_option_a.rs` docstring lines 7-13** — describes RED
   state (`kintsugi_main_lock` at `~line 3737`, docstring stale). Update
   to reflect GREEN reality.

None block ratification; all fold-forward safely.

## §6 Signal-to-Reed

- **Task #534 (thread-safety Option A) UNBLOCKS.** Arc 2 substrate-pull
  is complete at the dispatch-chain altitude.
- Next arc: choose between (a) Arc 4 sub-agent-directed follow-through on
  corrections above, or (b) advance to `.cargo/config.toml`
  `RUST_TEST_THREADS = "1"` removal empirical verification (the
  downstream promise in the RED docstring at line 37-40).
- Six-arm `cargo_args_for_check` collapse: pre-`bench` there were 7
  arms; now 8. Substrate-pull recognition candidate — the arm table IS
  a keyword-companion source shape that could lift to
  `shards/io/cargo.mirror` if it grows another 2-3 arms.

## §7 Canonical execution

If Reed accepts RATIFY-WITH-CORRECTIONS, follow-up ticks:

- **Tick A** — thread `ctx` through `pipeline.rs::tokenize_with_ref` and
  `spectral.rs` fold-reducer; migrate the two remaining live non-`_in`
  callers to the `_in` variants. Deletes the last implicit-process-cwd
  reads inside the dispatch chain.
- **Tick B** — delete `cmd_dump` (dead code) or thread `ctx`.
- **Tick C** — refresh `thread_safety_option_a.rs` docstring lines
  7-13 to reflect GREEN reality (retire "RED" framing).
- **Tick D (deferrable)** — audit `mcp.rs`'s `MIRROR_HOME`
  `set_current_dir` for Ctx-migration; outside Arc 2's promised scope
  but the last remaining process-wide cwd mutation in the binary.
- **Tick E** — attempt `.cargo/config.toml` `RUST_TEST_THREADS = "1"`
  removal; empirically verify no regression. If green, drop the config.

—

*Seam adversarial review, 2026-07-05. Read-only investigation of source;
audit is the only write.*
