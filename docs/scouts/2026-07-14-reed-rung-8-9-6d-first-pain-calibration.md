# Rung 8+9 Landing 8+9.6d.1 — first empirical pain calibration

*Reed, 2026-07-14. First empirical measurement of
`@cyberpunk/algedonic.pain_gradient` distribution across 5-iteration
trajectory. Substrate-honest data for ε_pain threshold selection per
Seam `e8508f5` §4 #6 ruling. This is the FIRST calibration pass;
follow-up sub-landings extend the sample size and morphism variety.*

---

## §0 What this discharges

Seam `e8508f5` §4 #6 ruling: **ε_pain has NO DEFAULT.** Empirical
calibration required first per Asher discipline. Reed's Landing 8+9.6a
used `sc_hamming` ratio as pain proxy (empirical value `0.7614` on
docstring-append) with `ε_pain = 0.5` — caused false-positive jumps on
every docstring-append.

Landing 8+9.6b retired the proxy in favor of substrate-honest
`@cyberpunk/algedonic.pain_gradient` (Shannon entropy of SC<5> hex; Reed
`b637178`). Single-shot empirical value was `0.0086` on the same
docstring-append — two orders of magnitude smaller than the proxy.

But single-shot measurement isn't a calibration. **This scout provides
the first trajectory-level empirical data:** 5-iteration run on real
substrate, observing how `pain_gradient` distributes under repeated
docstring-append morphisms selected by Fate::bounded.

---

## §1 Experimental setup

- **Peer home:** fresh copy of `/Users/alexwolf/dev/projects/mirror`
  at `/tmp/rung9-calibration/` (1141 files, 165 nodes, 6676 edges per
  Taut `77b8e14` baseline).
- **Target shard:** `shards/kintsugi/consent.mirror` (large; ~40KB;
  richly-linked substrate-decl — the same target used in Landing 1
  falsification and Landing 8+9.6a empirical).
- **Iterations:** 5 sequential peer_contribute invocations against the
  same target. Each iteration:
  - Fate::bounded selects Model+prism_op based on peer_home's
    psychohistory root (which changes with each contribution as the
    target file grows).
  - Docstring-append morphism per Rung 7' Scope A'.
  - `@cyberpunk/algedonic.sample_pain` computed at pre-anchor and
    post-anchor SC<5> hex.
  - `@mirror/lens/knife::stable_within` verdict at `ε_pain = 0.5`
    instrumentation.
- **Runtime:** `/Users/reed/.cargo-target/release/mirror peer contribute`
  from Reed `b637178` (algedonic Rust runtime + @knife plumbing).
- **Not measured this pass:** structural morphisms (file split/merge/
  rename topology). Docstring-append is intra-line only — doesn't
  change file-tree topology at all (per Rung 9 Landing 1 falsification).

---

## §2 Data

### 2.1 Per-iteration trajectory

| Iter | Fate Model | prism_op | pain_before | pain_after | pain_gradient | verdict |
|:----:|:----------:|:--------:|:-----------:|:----------:|:-------------:|:-------:|
| 1 | Abyss | focus | 0.9908 | 0.9864 | 0.0044 | Stable |
| 2 | Abyss | focus | 0.9864 | 0.9887 | 0.0022 | Stable |
| 3 | Abyss | focus | 0.9887 | 0.9877 | 0.0009 | Stable |
| 4 | Introject | project | 0.9877 | 0.9884 | 0.0007 | Stable |
| 5 | Introject | project | 0.9884 | 0.9841 | 0.0044 | Stable |

### 2.2 Statistical summary

- **Min gradient magnitude:** 0.0007 (iter 4)
- **Max gradient magnitude:** 0.0044 (iters 1 and 5)
- **Mean gradient magnitude:** 0.0025
- **Sign distribution:** 3 negative (loss decreased), 2 positive
  (loss increased). No monotonic direction; peer's coordinate
  fluctuates within a narrow band.
- **Pain magnitude range:** [0.9841, 0.9908] — all near 1.0 (high
  Shannon entropy characteristic of SHA-derived SC<5> hex).
- **Knife verdict:** Stable in all 5 iterations. Peer stays in `Op(COORDᵢ)
  = COORDᵢ` regime (Foerster A3).
- **Fate model distribution:** Abyss/focus (3 of 5), Introject/project
  (2 of 5). Fate::bounded rotated Model as peer's psychohistory grew.

### 2.3 Comparison to sc_hamming proxy

| Iter | pain_gradient (algedonic) | pain_gradient_hamming (proxy) | Ratio |
|:----:|:-------------------------:|:-----------------------------:|:-----:|
| 1 | 0.0044 | 0.7167 | 163× |
| 2 | 0.0022 | 0.7523 | 342× |
| 3 | 0.0009 | 0.7455 | 828× |
| 4 | 0.0007 | 0.7364 | 1052× |
| 5 | 0.0044 | 0.7515 | 171× |

The sc_hamming proxy is 163× to 1052× larger than the substrate-honest
algedonic gradient. **The proxy was substrate-error.** Every
docstring-append would have triggered a false-positive
`@knife.jump` under the proxy with `ε_pain = 0.5`.

---

## §3 Preliminary ε_pain recommendation

### 3.1 What the data supports

Docstring-append morphism on real substrate produces
`pain_gradient` magnitudes in `[0.0007, 0.0044]` under
algedonic. This range represents **peer within stable domain**
(all 5 verdicts were Stable; no boundary approach).

A substrate-honest ε_pain must be **above this range** to avoid
false-positive jumps on docstring-append. Otherwise every peer
contribution triggers a jump.

### 3.2 Preliminary threshold: ε_pain = 0.01

**Recommended provisional value:** `ε_pain = 0.01` (10× the observed max
within-stable gradient, 2.3× the observed max magnitude).

- Below 0.01: peer stays in Op(COORDᵢ) = COORDᵢ regime for docstring-
  append morphisms (no false jumps).
- Above 0.01: peer's coordinate has moved substantially — candidate for
  @knife.jump per Foerster A3 boundary-approach signal.

### 3.3 What this does NOT calibrate

**Reed's data covers only ONE morphism kind (docstring-append) on ONE
target shard over 5 iterations.** Full calibration requires:

- **Multiple morphism kinds.** Rung 9 Scope B's 5-row Model →
  consolidative-morphism mapping (Mara `c59a5ac` §3) includes file
  split, file merge, and rename morphisms. Those would produce
  structural changes — pain_gradient magnitudes would be much
  larger. Boundary-approach behavior needs empirical characterization
  on those morphisms.
- **Multiple target shards.** Different substrate altitudes (leaf
  shards vs family-root shards) may show different pain_gradient
  distributions.
- **Multiple peer_home configurations.** Different psychohistory
  roots → different Fate::bounded Rayleigh directions → different
  morphism selection distributions.
- **Longer trajectories.** 5 iterations shows within-stable-domain
  jitter. Rung 9 loop closure requires iterating until convergence or
  @knife.jump — need 20-50 iteration trajectories to characterize
  convergence timescale.

### 3.4 Load-bearing prediction for Landing 8+9.6d.2+

If Rung 9 Scope B introduces file-split morphisms (Cartographer/split),
pain_gradient magnitudes should be ORDERS OF MAGNITUDE larger than
docstring-append (file split fundamentally changes SC<5> hex distribution
by introducing a new file to the concept graph). ε_pain = 0.01 should
correctly trigger @knife.jump on those.

**Falsifiable:** if file-split pain_gradient stays below 0.01, either
algedonic Shannon-entropy metric is inadequate OR file-split is
substrate-honestly NOT a boundary-crossing morphism.

---

## §4 What this scout does NOT do

- **Does not commit ε_pain to substrate.** `0.01` is Reed-provisional based
  on 5 samples. Seam-ruling-honest form requires larger sample size and
  morphism variety before landing as substrate-decl value.
- **Does not adjudicate Mara `c753d5b` §10.1 canonical serialization.**
  Algedonic `sample_pain` uses hex-character-Shannon-entropy proxy. True
  `||sc||_2` L² norm requires parsing SC<5> hex → 80 f64 values +
  weighted L² (Mara adjudication forward-promise).
- **Does not verify Mara `38c2eeb` §10 four testable predictions.**
  Prediction #1 (pain_gradient ∝ distance-to-nearest-boundary) requires
  characterizing what "distance-to-nearest-boundary" means at SC<5>
  altitude — which requires solving Mara §10.1 first.

---

## §5 Next sub-landings

- **8+9.6d.2:** run 5-iteration trajectory against 5 different target
  shards (mix of small/large; different family-roots). Compare
  pain_gradient distributions per target. Verify ε_pain = 0.01 remains
  above within-stable range.
- **8+9.6d.3:** run 20-iteration trajectory to observe convergence
  behavior (does peer's pain settle to a limit? Does Fate::bounded's
  Model rotation stabilize?)
- **8+9.6d.4:** land ε_pain adjudication substrate-decl at appropriate
  altitude (@mirror/lens/knife.epsilon_pain? @cyberpunk/algedonic
  threshold? Alex names).

---

## §6 Substrate authority

- Alex Wolf 2026-07-13 in-transcript: "pain gradient exceeds threshold"
  as level-shift trigger.
- Foerster 1976 Appendix A3: `Op(COORDᵢ) = COORDᵢ` within stable
  domain; jump-behavior at boundary crossing.
- Mara `06a8547` §7 pain-driven navigation.
- Mara `38c2eeb` §10 prediction #1 (pain_gradient ∝ distance-to-
  boundary).
- Taut `15f7ed6` §5 (@cyberpunk/algedonic sample_pain Rust runtime
  realization gap).
- **Seam `e8508f5` §4 #6: ε_pain empirical calibration first per
  Asher discipline.** ← THIS ruling directly discharges by this scout.
- Reed `b637178` (@cyberpunk/algedonic Rust runtime).
- Reed `fa78507` (@knife plumbed into peer_contribute).
- Asher 2026-07-10 "Meaning Is Not a Metric" p.5 empirical method
  discipline (preregistration, deterministic fixtures, explicit pass/
  fail criteria, adversarial controls, preserved failures, scoped
  conclusions, no automatic promotion of passing results into
  architectural canon).

---

— Reed
