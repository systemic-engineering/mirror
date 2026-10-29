# Loop state

*Written by the loop at every firing. The loop acts on this file and `CHARTER.md`, never on memory.*

## Status: PRE-CHARTER (blocked, by design)

`docs/loop/CHARTER.md` does not exist. The loop has no signed done-property, checker, corpus commit or budget, so step 1 (measure) can't run. The loop does not write its own charter (it may never edit the charter; a definition of done the loop could author is one it could also move).

**What has to exist before the first measured iteration:**
1. **Certificate format** (what an explanation is): **done**, `docs/specs/mirror-certificate.md` (d77a4a07). Shape agreed with Alex 2026-10-29: an explanation is a non-linear trace through the past, i.e. the past cone of each output in the derivation partial order:
   - Foata layers (the engine's ticks/deposits);
   - derivation edges (rule instance + premise addresses; provenance);
   - erasures (`shaped_by`);
   - holes (questions).
2. **Independent checker:** design audited (`docs/audits/2026-10-29-seam-checker.md`). Python at `tests/checker/`, author neither Reed nor Seam, pinned by tree address. Mara folding the audit into the certificate spec. Not written yet. It replays a certificate in *any* linear extension of the partial order and must rebuild the identical root. Written separately from the compiler; Seam audits it; then frozen.
3. **Corpus commit:** `shards/` at a fixed address, plus Seam's adversarial fixtures.
4. **Budgets:** per iteration and per level, as declared rulers.
5. **Alex signs `CHARTER.md`** (draft: `CHARTER.draft.md`).

## Failure set

- Address: none (nothing measurable yet).
- Honest baseline: **zero shards explained.** `mirror compile` is still the old property loop; T1 is red; the parser doesn't exist. Level 1's first stretch is T1 itself.

## Last iterations

| firing | result |
|---|---|
| 2026-10-29 #1 | BOOT: no charter → no iteration. Pre-charter work started (this file, the draft charter, Mara's certificate spec). |
| 2026-10-29 #3 (woken by Seam) | BOOT: no charter → no iteration. Checker audit landed: four wrong certificates passed the spec's checks as written (stable model with a period-2 witness; a missing derivation; clear hiding a violation; a spurious fact), and `render(N, ε)` admitted N = ∅. Mara folding. Charter draft updated: the checker pin, the fix procedure, the UNKNOWN rule, K out of done. |
| 2026-10-29 #2 (woken by Mara) | BOOT: still no charter → no iteration. Certificate spec landed (`docs/specs/mirror-certificate.md`, d77a4a07): past cone as a partial order; all first-tick derivations; explained = soundness + closure; checker replays any linear extension; an independent `rust/checker` crate. Seam now auditing the checker design. |

## Commas waiting for Alex

1. Sign or amend `docs/loop/CHARTER.draft.md` (levels, done-property, checker, corpus, budgets, permissions).
2. **Who writes the checker** (neither Reed nor Seam; Seam's audit requires it). Candidates: Mara, Taut, a fresh agent, or Alex. And whether Python is the trust root (Seam's lean).
3. Standing permission inside the loop: push `reed/ship` and fast-forward `main` when the gate is green (granted ad hoc today; confirm it stands for the loop).
4. Still open: D1 (approval per ruler version), D5 (the name for the ruler fractal), D7 (gate checks). None blocks level 1.
