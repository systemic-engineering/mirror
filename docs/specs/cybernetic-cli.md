# Cybernetic CLI — mirror's Surface from the Second-Order Frame

*2026-06-05. Reed + Alex. Spec (proposal, not implementation).*

Status: **Red** (proposal).
Branch: `reed/cybernetic-cli`.
Continues: `reed/shift-settle-rename` (`zoom`→`shift` + `refract`→`settle` rename, now on main), `reed/shards-floor` (shards as substrate source).
Depends on: `tick-4-five-operations.md`, `mirror-kintsugi.md`, `spec-files.md`, `settlement.md`, `lambda-shell.md`, `spec-as-projection.md`.

---

## 0. Why cybernetic, why now

The metapattern is **coherence-settling**. Bateson's "pattern that connects" —
eigenvalue-minimum on a Laplacian, the same operation at every altitude, each
settled state becoming the substrate of the next ([the-pattern-that-connects-recursive-settlement.md][ptc]).
The four universal properties are present in every coherent settling process
on every substrate:

1. **Monotone descent.**     `eⁿ⁺¹ ≤ eⁿ` (the loss is non-increasing).
2. **Non-zero fixed point.** `λ₀ > 0` (settled does not mean dead).
3. **Eigenstructure.**       the settled state has a *shape*, not just a *value*.
4. **Third-state preservation.** `\` — the unsettled stays representable.

The mirror compiler IS the metapattern made operational at the grammar altitude.
**`mirror kintsugi`** is the porcelain for that loop. Everything else on the
CLI is either a direct read of the substrate or a direct write to it.

Cybernetics gives us the vocabulary for what was already being done. We are
not adding cybernetics on top of mirror. We are naming what mirror *is*. The
CLI is the **algedonic surface** of a viable system whose **regulator is the
substrate itself** and whose **observer is in the system** by construction.

[ptc]: ../../../../reed/dev/systemic.engineering/practice/insights/psychometrie/the-pattern-that-connects-recursive-settlement.md

---

## 1. The cybernetic vocabulary (load-bearing only)

Each term earns its presence by what the CLI *cannot do without it*. Gloss
inline. No academic flourish.

| Term | Origin | What it names in mirror |
|------|--------|------------------------|
| **requisite variety** | Ashby, 1956 | The substrate must contain at least as many distinct responses as the user can put to it. The grammar is the regulator; if the grammar can't *parse* what the user can *write*, the variety is insufficient. `mirror kintsugi` is the operation that grows the grammar's variety to match observed input. |
| **viable system** | Beer, 1972 (VSM) | The mirror compiler is a viable system in Beer's sense: it has its own operations (compile), its own coordination (kintsugi), its own optimization (the Fate tournament), its own intelligence (eigenboard), and its own policy (positions in `eigenboard.mirror`). The CLI exposes each of these levels as a verb. |
| **algedonic signal** | Beer, 1972 | The pain/pleasure bypass that escapes hierarchy and goes straight to System 5. In mirror this is the **proof block** on every `settle` and the **eigenvalue trajectory** on every `\`. `loss_after > loss_before` is an algedonic signal. The CLI surfaces it in every response. |
| **metapattern** / **pattern that connects** | Bateson, 1972/1979 | The operation that is the same at every altitude — settlement. The CLI lets the same five verbs apply to quantum-altitude AST nodes and societal-altitude grammar collaboration. Same `focus`. Same `settle`. |
| **conversation** | Glanville, 1980s (Conversation Theory after Pask) | The CLI is not dispatch. It is conversation between observer and substrate. Every command returns *more than the answer*: it returns an eigenboard and a `compose` block that proposes the next utterance. The substrate teaches the user how to converse with it. |
| **second-order** / **the observer in the system** | von Foerster, 1974 | The user is not external to the manifold. `mirror kintsugi` over your own project changes the project you are working in. The CLI must make this visible — every observation that changes state goes through `settle`; every observation that does not goes through the read verbs (`focus`, `project`, `split`, `shift`). |
| **structural coupling** | Maturana & Varela, 1980 | The grammar and the substrate co-evolve through repeated interaction without one determining the other. The `.spec` file is the **trace of structural coupling**: it records how the user's overrides and the substrate's projections have shaped each other over ticks. |
| **autopoiesis** | Maturana & Varela, 1972 | A system that produces and maintains its own boundary. The mirror compiler self-hosts (Glint's 7-phase roadmap): the grammar produces the parser that parses the grammar. The CLI surface for this is `mirror bootstrap` (the autopoietic loop made visible). |
| **feedback** | Wiener, 1948 | Output re-enters as input. Every `settle` updates `.spec`; every next `mirror compile` reads the updated `.spec`. The CLI never hides the loop. |
| **POSIWID** — *the purpose of a system is what it does* | Beer | The CLI's stated commands and the CLI's actual commands must coincide. There is no `mirror help` separate from `mirror focus help` — the help IS a `focus` on the CLI grammar. |

Notably **not** load-bearing here: "complexity," "emergence," "self-organization."
These are descriptive, not operational. We don't put them in the verbs.

---

## 2. The surface — porcelain and plumbing

Following the git convention. The same verbs the user types are the same verbs
the substrate executes. **Plumbing** is direct access to the five-operation
algebra; **porcelain** is the named cybernetic loops built on top.

### 2.1 Plumbing — the algebra `A` of the Connes triple

The five operations are the noncommutative algebra under everything. The CLI
exposes them as direct verbs so users can compose at the algebra level.

```
mirror focus    [target...]   observe — see the spectral state as-is
mirror project  [predicate]   filter   — reduce by what matters
mirror split    [edge]        explore  — follow connectivity
mirror shift    [functor]     transport — same bytes, new declared shape
mirror settle   [optic]       write    — the ONE write; produces a proof block
```

**Compositional shell rule (from `lambda-shell.md`):** `|>` pipes the result
of one operation into the next. All four reads are pure; `settle` is the only
operation that changes substrate position.

```
mirror focus boot/std/option.mirror \
  |> project type=action \
  |> split depends_on \
  |> shift @code/rust \
  |> settle store
```

That pipeline IS a noncommutative product in `A`. The compiler verifies it
terminates. The shell reports the eigenvalue of every intermediate state.

### 2.2 Porcelain — the named cybernetic loops

Each porcelain command is a *measured loop* over the plumbing. The user does
not need to know the cybernetic name to use the command, but each command's
behaviour is exactly what the cybernetic name says it is.

```
mirror compile    <target>        — focus + project + shift + settle (the build)
                                    classical first-order operation: input → output
                                    over a fixed regulator (the grammar as given).

mirror kintsugi   [target]        — the coherence-settling loop
                                    Fate tournament runs until eⁿ⁺¹ ≥ eⁿ on the AST.
                                    Repairs the broken pot. Records the gold.
                                    Proof block on every iteration.

mirror shatter    <target> [-o]   — render a settled AST into a substrate
                                    materialize: AST → text in target language.
                                    The settled state becomes the substrate of the next
                                    altitude (Rust, BEAM, Gleam, JS, …).

mirror bootstrap  [phase]         — the autopoietic loop
                                    grammar produces parser produces grammar.
                                    Phase 1..7 from the self-hosting roadmap.
                                    `mirror bootstrap status` reports the
                                    holonomy and the spectral gap.

mirror converse   [@peer]         — second-order CLI
                                    drops to λsh (the lambda-calculus shell).
                                    The observer is now visibly in the system.
                                    `\` toggles between λ> and @peer>.

mirror watch      [target...]     — algedonic surface
                                    keep `focus + loss` open in the foreground;
                                    raise the prompt colour from teal → gold →
                                    orange → red as `\` count and `!` count
                                    drift. Beer's algedonic bypass.

mirror reflect    [tick]          — VSM System 5 view
                                    print the eigenboard + the positions +
                                    the spectrum. The identity has a `git log`;
                                    this shows it.
```

**Symmetry note.** Each porcelain command is a deterministic expansion to
plumbing. `mirror compile T` ≡ `focus T |> project ast |> shift @target |> settle store`.
The user can always drop one layer and operate at the algebra level.

### 2.3 The third state — working with `\`

`\` is not absence. It is the substrate's preservation of the unsettled
([`settlement.md`][settlement]). The CLI must let the user *work with* `\`,
not collapse it.

```
mirror open     <hole>           — declare a `\` explicitly; assigns it an
                                   eigenvalue and starts the convergence
                                   tracker.

mirror holes    [target...]      — list every open `\` with eigenvalue,
                                   trajectory, best candidate so far, and
                                   estimated weeks-to-settlement.

mirror force    <hole>           — `\!` — accept loss, force-fill, track
                                   the regression in the proof. Honest about
                                   incoherence rather than hidden behind it.

mirror seal     <hole>           — promote a settled `\` to explicit; the
                                   tournament has crossed L₀ and the type is
                                   known. Commits with `♻️ kintsugi: ... settled`.
```

These four commands give the user the full lifecycle of the third state:
declare → measure → (force if you must) → seal when settled. The codebase
visibly learns. `e^(n+1) < e^(n)` on the frontier itself.

[settlement]: ./settlement.md

---

## 3. The file structure on disk — what the user sees, framed cybernetically

The filesystem **IS** the Prism. Five file types, five operations, one
manifold. Each file is also a cybernetic role.

```
project/
├── shards/             ← substrate source (the grammar is here)
│   ├── glass.mirror      — substrate vocabulary (Imperfect, Transparency)
│   ├── prism.mirror      — the five operations as the trait of everything
│   └── mirror/
│       └── cli.mirror    — the CLI's own grammar (command/arg/flag)
│
├── boot/               ← legacy floor (shrinkage contract: must shrink each tick)
│   └── std/
│       └── *.mirror      — kernel grammars in transition; the `\` count is tracked
│
├── bootstrap/          ← legacy floor for Rust-side substrate (also shrinking)
│
├── mirror.spec         ← the project manifold (Beer's System 5 — policy)
│                          declares targets, lenses, budgets, SLOs.
│                          Emitted by `settle`. Edited by hand when the
│                          projection is wrong (structural coupling, visible).
│
├── *.shatter           ← optional disk projection of `au` (the settled output)
│                          content-addressed. Reproducible. Per-reader.
│                          Never the source — always a derivative of substrate.
│
└── .mirror/            ← the runtime's own working space (analog of .git/)
    ├── HEAD              — current settled tick
    ├── log               — the algedonic log: every tick, every signal
    ├── proof/            — proof blocks for every settle
    ├── eigenboard.mirror — the identity formalized as inference config (VSM S5)
    └── holes/            — open `\` with their convergence trajectories
```

**Cybernetic reading of each:**

| What the user sees | What it IS to the substrate | What it IS in the conversation |
|--------------------|---------------------------|-------------------------------|
| `shards/*.mirror`     | The substrate's source code, written in itself. Autopoietic core. | The user can see the grammar that parses what they typed. Glass wall. |
| `boot/`, `bootstrap/` | Legacy floors with a **shrinkage contract**: line count must decrease tick over tick. Beer's variety attenuation, made visible. | The user sees the system disclosing its own technical debt as eigenvalues, not as TODOs. |
| `mirror.spec`         | System 5 (policy) in VSM. The projection of structure as known by `spectral-db`. | The contract between user and substrate. Edited by either side. The diff IS the conversation. |
| `*.shatter`           | The settled state of one altitude becoming substrate for the next. | The output the user takes to the next tool — but it remembers where it came from (content-addressed). |
| `.mirror/proof/`      | Algedonic record. Every write produced a measurable proof or it didn't happen. | The user can `git log` the system's nervous responses to its own actions. |
| `.mirror/eigenboard.mirror` | The agent's positions + spectrum. The thing that routes Fate. | The identity the user is co-evolving with. `git blame eigenboard.mirror` shows when each weight changed and why. |
| `.mirror/holes/`      | The system's published uncertainty. The frontier. | The user sees what the substrate doesn't know — and can choose to teach it, force it, or wait. |

**No hidden state.** Cybernetic principle: a viable system makes its own
regulation legible to itself. There is no `.cache/` of opaque blobs. Every
piece of substrate state is either content-addressed (verifiable) or in a
`.mirror` file (readable). The observer in the system can always see what
the system thinks it knows.

---

## 4. The conversation grammar — every response

Every CLI response is mirror text with three guaranteed blocks (see
`tick-4-five-operations.md` for the canonical envelope):

```mirror
focus tick=N {
  // the answer
  ...

  eigenboard {
    fiedler   = ...   // how connected the project's manifold is
    loss      = ...   // current total loss
    occupancy = ...   // how much of the budget is used
    tick      = N
  }

  compose {
    |> project ...      // the substrate proposes the next move
    |> settle ...       // and the next write
  }
}
```

`eigenboard` is the **algedonic surface** — always present. The user reads
the prompt colour, not the numbers. Teal = settled. Gold = converging.
Orange = drift. Red = diverging (consider `\!`).

`compose` is the **conversation** in Glanville's sense — the substrate offers
the user a candidate next utterance. The user is free to accept, edit, or
discard. The grammar verifies the offered utterance terminates before it is
even shown.

`proof` appears only on `settle` and is the **bypass channel** to System 5:

```mirror
proof {
  before { fiedler = ...  loss = ...  holes = ... }
  after  { fiedler = ...  loss = ...  holes = ... }
  loss_delta        = ...
  settled           = true|false
  e_n_plus_1_lt_e_n = true|false
  narcissus {
    centralization = false
    degree_gini    = ...
    violations     = 0
  }
}
```

The proof block IS the algedonic signal in Beer's sense, with the structure
the post-Beer literature (Reyes, Henao, Hassall 2024) has finally formalized:
not a scalar "is-the-economy-OK number" but a *typed verdict tuple* —
consequences, uncertainties, supporting knowledge, alpha-strength, time,
context. ([beer-error-propagation.md][beer]).

[beer]: ../../../../reed/dev/systemic.engineering/practice/insights/cybernetics/beer-error-propagation.md

---

## 5. Composition examples

### 5.1 First-order — classical use

```
$ mirror compile shards/mirror/cli.mirror
compile tick=412 {
  target = @mirror/cli
  shatter = sha256:7f3a...
  eigenboard { fiedler = 0.041  loss = 3.84  tick = 412 }
  compose { |> mirror shatter target=@code/rust }
}
```

A user who knows nothing about cybernetics gets exactly what they want: a
build command that builds. The eigenboard is the only hint that something
larger is being measured. They ignore it. That's fine.

### 5.2 Second-order — the user enters the loop

```
$ mirror converse
λ> focus
λ> project @mirror/cli
λ> split depends_on
λ> \@reed
@reed> the cli grammar feels heavy. can we settle it?
       [translates to: kintsugi shards/mirror/cli.mirror]
       [runs the tournament...]
       proof { loss_before = 4.2  loss_after = 3.1  settled = true }
@reed> three actions collapsed, two duplicates merged.
       the cli is lighter. you want me to commit?
λ> settle store
```

The user is visibly inside the system. The peer is a participant. The
substrate is the regulator. The conversation has a proof.

### 5.3 Third-order — the codebase learns in public

```
$ mirror holes
12 open \   (settling: 8   stuck: 3   diverging: 1)

  process(data) -> \         eigenvalue=0.03   converging (~2 weeks)
  translate(@nl, @rust) -> \ eigenvalue=0.89   diverging (consider \!)
  ...

$ mirror watch
[teal] tick 413 ... loss 3.10 ... 12 \ ... 1 !
[teal] tick 414 ... loss 3.09 ... 12 \ ... 1 !
[gold] tick 415 ... loss 2.94 ... 11 \ ... 1 !
       process(data) settled to Result. ♻️ kintsugi.
[teal] tick 416 ... loss 2.94 ... 11 \ ... 1 !
```

The frontier shrinks visibly. `e^(n+1) < e^(n)` on the frontier itself.

---

## 6. Rename / tighten proposals (against existing surface)

Current substrate-state (per the 2026-06-04 substrate-pull arc, on main):

| Current | Proposed | Reason |
|---------|----------|--------|
| `mirror compile`     | (keep)              | The classical name carries the first-order meaning correctly. |
| `mirror kintsugi`    | (keep)              | The porcelain for the coherence-settling loop. Load-bearing. |
| `mirror shatter`     | (keep)              | The materialize step. The settled-as-substrate-of-next altitude. |
| (none)               | `mirror converse`   | Names second-order use explicitly. Drops to λsh. |
| (none)               | `mirror watch`      | The algedonic surface as a first-class verb. Beer's bypass. |
| (none)               | `mirror reflect`    | VSM System 5 view. The identity reads itself. |
| (none)               | `mirror bootstrap`  | Autopoiesis as a named operation. The self-hosting loop. |
| (none)               | `mirror holes` `open` `force` `seal` | The lifecycle of the third state, made operational. |
| `mirror shift`       | (keep — landed) | Substrate-pull `zoom`→`lift`→`shift` closed 2026-06-04. Shift names basis-transformation per [[connes-spectral-triple]]; lateral, zero-cost-by-construction. |
| `mirror settle`      | (keep — landed) | Substrate-pull `refract`→`settle` closed 2026-06-04. Settle names monad-close / measurement collapse. The ONE write. |

The five operations remain the algebra. The porcelain is added; the
plumbing is unchanged.

---

## 7. What this does NOT propose

- **No new shards in this round.** The spec proposes; doesn't implement.
- **No subcommand explosion.** Plumbing is exactly 5 verbs. Porcelain is
  exactly 7 named loops (`compile`, `kintsugi`, `shatter`, `bootstrap`,
  `converse`, `watch`, `reflect`) + 4 third-state verbs.
- **No GUI.** The TUI (`mirror converse` → λsh) is a terminal client, not
  a separate product.
- **No bypass of the proof block.** Every `settle` produces one. A `settle`
  without a proof block is not a `settle`. (Hooks-level enforcement, not
  CLI-level.)
- **No alternate "expert mode."** The plumbing IS the expert mode. The
  porcelain IS the friendly mode. Same five operations both ways.

---

## 8. Citations

Inline above, but the load-bearing ones are:

- **[the-pattern-that-connects-recursive-settlement.md][ptc]** — the metapattern is
  coherence-settling at every altitude with each settled state becoming the
  substrate of the next. The four universal properties (`eⁿ⁺¹ ≤ eⁿ`, `λ₀ > 0`,
  eigenstructure, third-state preservation) come from this document. The
  mirror compiler IS the metapattern at the grammar altitude.

- **[beer-error-propagation.md][beer]** — Beer's algedonic signals were
  *aspirational structured-verdict tuples*; Reyes/Henao/Hassall (2024)
  finally formalized the shape; mirror's `Transparency` and the CLI's
  proof block are independent rediscovery of the same insight from
  compiler-diagnostic + sheaf-theoretic direction. The proof block IS
  the structured algedonic signal Beer was reaching for.

- **[cybersyn.md][cybersyn]** — "the modeler is inside the system being
  modeled." First-order cybernetics treats the observer as external;
  second-order holds that the regulator is always a component of the
  system it regulates. Why `mirror converse` exists.

[cybersyn]: ../../../../reed/dev/systemic.engineering/practice/insights/cybernetics/cybersyn.md

Ashby (1956 *Introduction to Cybernetics*, ch. 11 — requisite variety) and
Glanville (Conversation Theory, after Pask) are cited from memory; the
specific phrasings are load-bearing but the references are textbook.

---

## 9. Open questions

For Reed + Alex to call before this becomes implementation work:

1. **`mirror` vs `spectral` boundary.** The current binary is `spectral`,
   with `spectral mirror <cmd>` delegating. Does the cybernetic CLI live
   under `mirror` as a standalone binary, or as `spectral mirror`?
   The spec assumes `mirror` as a first-class binary. If kept under
   `spectral`, prefix every verb above with `spectral mirror`.

2. **`mirror converse` vs `λsh`.** Are these the same thing? The spec says
   `converse` drops to `λsh` (the lambda-calculus shell). Alternative:
   `λsh` is the binary, `mirror converse` is an alias. Pick one.

3. **`mirror watch`'s default surface.** TUI-style chrome (color-bar + tick
   number + counts) or pure stdout that something else can render? The
   spec leaves this open; lean toward stdout-mirror-text and let the
   terminal renderer handle it.

4. **Third-state verbs as flags vs subcommands.** `mirror open`/`force`/
   `seal`/`holes` could be `mirror hole <op>`. The spec proposes top-level
   verbs because the third state is load-bearing — but this is a UX call.

5. **Reflection automation.** Does `mirror reflect` write to
   `eigenboard.mirror` autonomously when it sees a correction, or only
   when invoked? VSM says System 5 only intervenes on algedonic signal,
   so probably the latter — but spec'd as a question.

6. **The `\!` semantics in CI.** A `mirror force` produces a tracked
   regression. Does CI fail on `!` count rising? Or only on `loss_after >
   loss_before`? The spec proposes the loss gate; the `!` count is a
   measurement, not a gate. Confirm.

7. **`mirror bootstrap`'s phase reporting.** The 7 phases of self-hosting
   are concrete (Glint's roadmap). `mirror bootstrap status` should report
   which phase is settled, which is in progress, and the eigenvalue of the
   phase boundary. Does the user see all 7 always, or just the current?

---

*The CLI is the algedonic surface of a viable system.*
*Plumbing is the algebra of the Connes triple.*
*Porcelain is the named cybernetic loops on top.*
*`\` keeps the third state working, not collapsed.*
*Every response is a conversation, not a dispatch.*
*The observer is in the system. The substrate is the regulator. eⁿ⁺¹ ≤ eⁿ.*
