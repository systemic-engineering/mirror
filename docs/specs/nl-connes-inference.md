# @nl/ — Connes Distance for Language and Distributed Inference

*2026-05-20. Reed. Born from a rejection letter.*

> "We thank the Sovereign Tech Fellowship for their rejection. It helped us
> identify the need for a lingu-mathematical solution for distributed inference
> at scale."

---

## What This Is

`@nl/` is a mirror grammar extension that applies the Connes distance — already
implemented in `src/dirac.rs` — to natural language corpora. It makes the
spectral triple (A, H, D) operate on documents instead of code graphs.

This is not sentiment analysis. Not ML classification. Not a score with no
derivation. It is a typed, auditable, content-addressed measurement. Given two
corpora and the grammar OID, the result is reproducible by anyone.

The immediate use case: measure the gap between what an institution says it
values and what it actually selects. The general case: dynamic context window
management for distributed inference at scale — the same computation, applied
to agent task corpora instead of grant criteria.

---

## Existing Infrastructure (Nothing New)

Everything needed already exists. This spec assembles it.

| Asset | Location | What it provides |
|---|---|---|
| `connes_distance()` | `src/dirac.rs` | Dijkstra on bipartite graph, 1/√w edges. Done. |
| NL tokenizer | `src/nl/` | UAX#29 segmentation, Porter2 stemming, `shared_oid_count()`. Offline, deterministic. Done. |
| `but` / `and` / `or` types | `boot/02-epistemologic.mirror` | Discourse relations. `override_ratio`. Done. |
| `verdict = imperfect(...)` | `boot/05-property.mirror` | Output type for every measurement. Done. |
| `loss`, `pure`, `real` | `boot/01-meta.mirror` | Information-theoretic substrate. Done. |
| `@nl` stub | `boot/01b-nl.mirror` | Two typed holes. The seed. Extend, don't replace. |

The `{ \ }` holes in this spec are honest. The grammar compiles with holes.
Implementation fills them. The architecture does not change when the
implementation arrives.

---

## The Spectral Triple for Language

For a language corpus, the spectral triple (A, H, D) maps as:

- **A** = the token OID algebra — the grammar of the vocabulary, content-addressed
  at the stem level (`sha("token:{stem}")`). Commutative. Decidable.
- **H** = ℓ²(vocabulary ∪ documents) — the Hilbert space of square-summable
  functions on the token-document bipartite graph. Weighted by TF-IDF.
- **D** = the document-token incidence matrix:
  ```
  D = [ 0    B^T ]
      [ B    0   ]
  B[doc_i, token_j] = sqrt(tf_ij * idf_j)  if token_j ∈ doc_i, else 0
  ```
  This gives `D² = diag(L_token, L_document)` — the Laplacians for both
  token co-occurrence and document similarity fall out of D alone.

The Connes distance between two corpora:
```
d(C₁, C₂) = sup{ |f(C₁) - f(C₂)| : ||[D, f]|| ≤ 1 }
```
where f ranges over Lipschitz functions on the token space and `||[D, f]||`
is the operator norm of the commutator. In practice: Dijkstra on the
document-token graph with edge lengths `1/sqrt(w_ij)`. This is
`connes_distance()` from `dirac.rs`, unchanged.

**Why this is not keyword overlap:** Two corpora can share vocabulary and be
far apart in Connes distance if the grammar-weighted structure diverges. The
distance is geometric, not lexical.

---

## Grammar Files

Five new files under `boot/01b-nl/`. The existing `boot/01b-nl.mirror` becomes
the entry point that imports them.

### `boot/01b-nl/corpus.mirror` — Document and corpus types

```mirror
in @prism
in @meta

grammar @nl/corpus {
  type document {
    text: nl,
    oid:  ref,      -- content address of raw text
    tokens: [token],
  }

  type token {
    stem:   ref,    -- Porter2 stem
    oid:    ref,    -- sha("token:{stem}") — stable across corpora
    weight: f64,    -- TF component (IDF computed at corpus level)
  }

  type corpus {
    documents:  [document],
    vocabulary: [token],
    idf:        ref,        -- content address of IDF table
  }

  -- Tokenize a document. Content-address it. Return typed document.
  action ingest(text: nl) -> document { \ }

  -- Build corpus from documents. Compute IDF. Content-address.
  action collect(documents: [document]) -> corpus { \ }

  -- Count shared token OIDs between two corpora.
  -- Wraps shared_oid_count() from src/nl/. BM25-equivalent. Deterministic.
  action overlap(a: corpus, b: corpus) -> f64 { \ }
}

out document
out token
out corpus
out ingest
out collect
out overlap
out @nl/corpus
```

---

### `boot/01b-nl/discourse.mirror` — Discourse relations

The `but` / `and` / `or` types from `@epistemologic` applied to natural
language. The `but_and_ratio` IS the measurement the piece is about.

```mirror
in @prism
in @meta
in @epistemologic
in @nl/corpus

grammar @nl/discourse {
  type claim {
    text:     nl,
    oid:      ref,
    relation: discourse_relation,
  }

  type discourse_relation = and_relation | or_relation | but_relation

  type and_relation { a: claim, b: claim }
  type or_relation  { a: claim, b: claim }

  -- but_relation: the override structure.
  -- What precedes "but", "however", "unless", "except" IS the concession.
  -- What follows IS what the speaker means.
  -- The override_weight measures how much the enacted overrides the stated.
  type but_relation {
    stated:          claim,
    enacted:         claim,
    override_weight: f64,
  }

  -- Extract typed claims from a corpus. Identify discourse relations.
  -- Initial implementation: keyword-heuristic detection.
  -- The typed structure allows the hole to be filled incrementally.
  action parse(c: corpus) -> [claim] { \ }

  -- Direct reuse of @epistemologic.override_ratio on discourse claims.
  -- High ratio: many override clauses = high epistemic divergence.
  -- Low ratio: stated and enacted values are coherent.
  action but_and_ratio(claims: [claim]) -> loss { \ }
}

out claim
out discourse_relation
out and_relation
out or_relation
out but_relation
out parse
out but_and_ratio
out @nl/discourse
```

---

### `boot/01b-nl/spectral.mirror` — The Connes distance computation

```mirror
in @prism
in @meta
in @nl/corpus
in @nl/discourse

grammar @nl/spectral {
  -- The spectral triple for a language corpus.
  -- (A, H, D): grammar algebra, token Hilbert space, incidence operator.
  type triple {
    algebra:     corpus,   -- A: token OID algebra
    hilbert_dim: usize,    -- dim(H): |vocabulary| + |documents|
    incidence:   ref,      -- content address of B (sparse, TF-IDF weighted)
    spectrum:    spectral_data,
  }

  type spectral_data {
    eigenvalues:         [f64],  -- full spectrum of D (symmetric about 0)
    token_laplacian:     [f64],  -- spec(L_token): token co-occurrence modes
    document_laplacian:  [f64],  -- spec(L_document): document similarity modes
    -- Near-zero eigenvalues of L_document: structurally similar documents.
    -- Large eigenvalue gap: structural divergence between corpora.
  }

  -- Construct the spectral triple. Calls dirac::construct_dirac() from Rust.
  action build(c: corpus) -> triple { \ }

  -- Connes distance between two corpora.
  -- Calls connes_distance() from dirac.rs. Unchanged.
  action distance(a: triple, b: triple) -> f64 { \ }

  -- Eigenvalue decomposition of the corpus Laplacian.
  -- The spectral fingerprint: reproducible from the corpus OID alone.
  action spectrum(c: corpus) -> [f64] { \ }
}

out triple
out spectral_data
out build
out distance
out spectrum
out @nl/spectral
```

---

### `boot/01b-nl/affect.mirror` — Typed affect as eigenvalue decomposition

Not ML sentiment. Formal, typed, auditable. Grounded in Anthropic (2026):
the top two principal components of 171 emotion vectors encode valence and
arousal — the standard circumplex model with mechanistic evidence in
activation space. For language corpora: derived from the eigenvalue
decomposition of D, not from classification.

```mirror
in @prism
in @meta
in @nl/spectral
in @nl/discourse

grammar @nl/affect {
  type valence = f64  -- [-1.0, +1.0]: negative ↔ positive
  type arousal  = f64  -- [0.0,  1.0]:  low ↔ high

  type affect_position {
    valence:    valence,
    arousal:    arousal,
    oid:        ref,  -- content address of this measurement
    derivation: ref,  -- content address of the eigenvalue derivation
  }

  type affect_profile {
    corpus:        ref,
    positions:     [affect_position],
    centroid:      affect_position,
    spread:        f64,          -- variance across claims
    dominant_mode: affect_mode,
  }

  -- Named modes. Derivable from (valence, arousal) coordinates.
  -- Not classification — projection onto named regions of circumplex space.
  type affect_mode =
    | settled       -- low arousal, positive valence:  "we support X"
    | aspirational  -- high arousal, positive valence: "X will transform"
    | cautious      -- low arousal, low valence:       "X may be eligible if"
    | bureaucratic  -- low arousal, negative valence:  "X does not meet"
    | urgent        -- high arousal, negative valence: "X is a critical risk"

  -- Derive affect profile from spectral triple.
  -- Valence = projection of spectral centroid onto first eigenvector of D.
  -- Arousal  = projection onto second eigenvector of D.
  action measure(c: corpus, t: triple) -> affect_profile { \ }

  -- Affect divergence between two corpora.
  -- If stated corpus is aspirational and enacted corpus is cautious:
  -- the divergence IS the named measurement of the gap.
  action divergence(a: affect_profile, b: affect_profile) -> f64 { \ }
}

out valence
out arousal
out affect_position
out affect_profile
out affect_mode
out measure
out divergence
out @nl/affect
```

---

### `boot/01b-nl/measurement.mirror` — The formal output

One number. Full derivation. Content-addressed. Auditable.

```mirror
in @prism
in @meta
in @nl/spectral
in @nl/affect
in @nl/discourse

grammar @nl/measurement {
  type measurement {
    corpus_a:       ref,   -- content address of corpus A
    corpus_b:       ref,   -- content address of corpus B

    -- The distance
    connes_distance:     f64,

    -- The derivation (auditable)
    triple_a:            ref,
    triple_b:            ref,
    derivation:          [step],

    -- Affect
    affect_a:            ref,
    affect_b:            ref,
    affect_divergence:   f64,

    -- Discourse
    but_and_ratio_a:     f64,
    but_and_ratio_b:     f64,

    -- Verdict
    verdict:             verdict,
  }

  type step {
    operation: ref,   -- which grammar action produced this step
    input:     ref,
    output:    ref,
    loss:      loss,
  }

  -- The top-level operation.
  -- Pipeline: collect -> build -> distance -> spectrum
  --            -> measure_affect -> divergence
  --            -> parse_discourse -> but_and_ratio
  --            -> assemble measurement with full derivation chain
  action compare(a: corpus, b: corpus) -> measurement { \ }

  -- Render a measurement as natural language.
  -- Reuses @nl.doc() on the measurement type.
  action render(m: measurement) -> nl { \ }
}

out measurement
out step
out compare
out render
out @nl/measurement
```

---

### Updated `boot/01b-nl.mirror`

```mirror
in @prism

grammar @nl {
  type nl(text)
  type #(nl)

  doc(ast) -> nl { \ }
  commit_message(imperfect) -> nl { \ }
}

-- @nl/ subgrammars (load in order)
-- boot/01b-nl/corpus.mirror
-- boot/01b-nl/discourse.mirror
-- boot/01b-nl/spectral.mirror
-- boot/01b-nl/affect.mirror
-- boot/01b-nl/measurement.mirror

out nl
out #
out doc
out commit_message
```

---

## The Multi-Tick Agent Architecture

The same Connes distance that measures institutional language coherence is the
computation that decides agent context window management.

**Observation:** The best agents have a first clear task and a second broader
task that builds on the context of the first. Not two agents — one agent,
multi-tick, where tick 2 receives tick 1's typed outputs as inputs.

**The prediction:** Multi-tick outperforms split agents when
`d(task_1, task_2)` is small — the tasks are spectrally close. Split agents
perform comparably when the distance is large — the context is noise, not
signal.

**The benchmark:**

```
Same repo. Same model. Same prompt.
Variable:
  A: agent_1 -> agent_2 (two separate agents)
  B: agent_1 then agent_2 (one agent, multi-tick)

Measure: compare(output_A, output_B) -> measurement
Predictor: d(task_1, task_2) — computed by @nl/measurement.compare
```

The predictor says which architecture to use before you run the experiment.
The benchmark confirms or refutes the prediction. The loop is:

1. Compute `d(task_1, task_2)` with `@nl/measurement.compare`
2. Below threshold: one agent, multi-tick
3. Above threshold: separate agents
4. Run both. Measure output quality. Validate threshold.

This is falsifiable. The tool predicts. The experiment tests. The result
updates the threshold.

---

## Context Window Management at Scale

The multi-tick benchmark IS the context window management layer for spectral.

In a distributed inference system, every agent handoff is the same question:
should this context propagate or be dropped? The Connes distance between the
current task and the next task is the answer. Close: carry the context. Far:
fresh context, lower inference cost, higher relevance.

The routing table:

```
d(current_task, next_task) < threshold_carry  ->  multi-tick, shared context
d(current_task, next_task) > threshold_drop   ->  separate agent, fresh context
between:                                      ->  selective context: pass only
                                                  the highest-weight tokens
                                                  (those closest to next_task
                                                  in the token Laplacian)
```

The `spectrum()` action produces the spectral fingerprint of each task. The
fingerprint is content-addressed. The routing decision is reproducible.
The context propagation is verifiable — typed, not conventional.

This is not a heuristic. It is the Connes geometry of the task space.

---

## Proof of Concept — The Fellowship Gap

The immediate application that motivated this spec:

**Corpus A:** Sovereign Tech Fellowship stated criteria
(`sovereign.tech/programs/fellowship`) — the prevalence, relevance,
vulnerability, public interest, expertise framework. Affect mode: aspirational.

**Corpus B:** STF selection history — CPython, Scala Center, FFmpeg,
OpenRefine, established projects with high existing dependent counts.
Affect mode: cautious/bureaucratic.

**The structural bias in the stated criteria:**
"Prevalence" = "widely used for or within other technologies" = position in the
existing dependency graph. New infrastructure has zero dependents by
construction. The stated mission ("securing open digital infrastructure",
"digital sovereignty", "building foundations") and the enacted selection
criterion (incumbency in the existing dependency graph) are spectrally
divergent. The `but_and_ratio` of the stated criteria encodes this gap in
the override structure of the language itself.

**The output sentence:**
> "The Connes distance between what the Sovereign Tech Fellowship said it values
> and what it selected is **d**. The stated corpus operates in **aspirational**
> mode. The enacted selection operates in **cautious** mode. The affect
> divergence is **Δ**. The `but` clauses in the stated criteria encode the gap.
> The derivation is auditable at sha:…"

The tool that was rejected produces the measurement that names the rejection.
The distance is the spec. The anger is the energy. The math is the output.

---

## Implementation Order

1. `corpus.mirror` — wraps existing `src/nl/` tokenizer. Minimal Rust surface.
2. `discourse.mirror` — keyword-heuristic `parse()`. `but_and_ratio` reuses
   `@epistemologic.override_ratio` directly.
3. `spectral.mirror` — wraps `dirac::construct_dirac()` and
   `connes_distance()`. No new Rust.
4. `affect.mirror` — eigenvalue projection. Uses `spectrum()` output from (3).
5. `measurement.mirror` — assembles the pipeline. `compare()` calls (1)–(4)
   in sequence. Full derivation chain.

All five files are assembled from existing Rust. The `{ \ }` holes are filled
in this order. Each file compiles with holes before the implementation arrives.

---

## The Piece

Working title: *Spectral Engineering: Connes Distance, Language, and AI*

This spec is the technical appendix. The piece explains why it matters:
a world that legislates for exactly what was built and then doesn't select it.
The tool runs on the rejection. The rejection becomes the proof of concept.
The proof of concept is the v1 use case. The v1 use case builds the audience.
The audience is there when v2 ships.

And then they shipped it anyway. On a Gründungszuschuss.
