# 09 — Licensing Architecture

## Status: Design decided, not yet implemented

---

## The split

**Apache-2.0** — the infrastructure layer:
- Compiler (parse.rs, resolve.rs, compile.rs)
- Model checker (check.rs, property.rs)
- Rust-native runtime (ractor-based, in progress)
- Supporting crates: framework, coincidence, fragmentation

**systemic.engineering License (SEL)** — the ecosystem layer:
- BEAM runtime
- Garden (package manager + package ecosystem)
- Every domain grammar that ships in the garden: `@ca`, `@ai`, `@filesystem`,
  `@git`, `@erlang`, `@training`, `@reed`, and all future packages

---

## Why this split

The compiler is infrastructure. Anyone embeds it. No friction.

The garden is where the practice lives. The SEL encodes: no extraction, consent
real, no structural harm. These aren't legal promises attached to the ecosystem —
they are compiler-enforced type constraints on everything that runs in the garden.

A grammar that encodes an extractive data flow doesn't compile.
A grammar where consent is bypassable fails the property check.
The garden can only contain software that is provably non-extractive.

This is not enforcement through terms. It is enforcement through eigenvalues.

---

## `@ca` and `@ai` specifically

`@ca` — the content-addressed pipeline — is SEL-licensed. The proof infrastructure
itself carries the ethical condition. You cannot use the proof system to certify
that extraction is safe.

`@ai` — the inference domain — is SEL-licensed. AI pipelines are the primary
extraction attack surface. Encoding non-extraction as a type constraint on `@ai`
means an AI system built on conversation cannot express extractive data flows in
its grammar. The architecture forecloses it before runtime.

---

## Threat model

This is not designed to stop determined extractive actors. Anyone can fork,
strip the property checks, and run whatever they want. That's life.

The threat this addresses is **accidental extraction** — systems built by people
who didn't intend to extract but whose architecture allowed it by default. This
is the majority of extractive systems. Most extraction isn't malicious; it's the
path of least resistance when nothing structurally blocks it.

Compiler-enforced non-extraction closes that gap. Extraction becomes a deliberate
act — you have to actively remove the checks. The SEL makes removal a license
violation, which creates legal deterrence for the carelessly extractive actor
(not the adversarial one, who doesn't care about licenses either).

The goal: make non-extractive the default. Make extraction require intent.

---

## The novel claim

Every other "ethical open source" license puts ethics in a document.
conversation puts them in the type checker.

The Apache-2.0 infrastructure makes this claim checkable — it is the tool.
The SEL ecosystem is what the tool enforces — it is the batteries.

You get the compiler, the model checker, and the Rust runtime.
You don't get the batteries.

---

## Implementation

No code changes required for the split itself. The licensing architecture is
a decision about which files carry which LICENSE header, and where the garden
registry enforces SEL compliance as a condition of package inclusion.

The model checker property enforcement (06-model-checker.md) is the mechanism.
The SEL conditions become grammar properties. Package admission into the garden
requires passing the model checker against those properties.

Specific properties to define (not yet written):
- `non_extractive` — no data flow from user contribution to system benefit
  without explicit consent path in the type surface
- `consent_declinable` — every action that solicits input must have a
  refusal type in its return surface
- `no_coercive_structure` — no type surface where all paths carry cost
  toward a predetermined outcome

These are SEL §3 conditions translated into grammar property declarations.

---

## Incentive to be in the garden

Package authors get free AI-driven maintenance for all packages in the garden,
running on the conversation runtime itself.

This closes the incentive loop: accept SEL conditions, get structural
non-extraction enforcement, get automated maintenance as a service. The
maintenance agent runs in the same runtime that compiled the package — `@ai`
operating on garden packages is the primary proof-of-concept for `@ai` being
non-extractive by construction.

The self-referential structure is load-bearing: conversation maintains its own
ecosystem using its own infrastructure, under its own ethical constraints.
Every maintenance run is the claim executing.

This also answers "why accept SEL rather than fork?" with something concrete.
The fork loses the enforcement and loses the service. The path of least
resistance is staying in the garden.

This is a structural gradient. The garden pulls packages toward it — not
through rules, through physics. The enforcement, the maintenance, the
proof certificates all accumulate inside. Outside is just a compiler.

## Actors as compliance infrastructure

The @ai actors running in the garden are content-addressed. They know what
they are and where they're supposed to run. A forked runtime without garden
infrastructure has no proof chain — OIDs don't resolve, property verdicts
aren't present, the execution context is detectable.

Because actors have dispatch, reporting is just another action.
`@ai.report_violation(context)` is the same primitive as anything else.
The actors can snitch.

The evasion ladder:

1. Fork the runtime — lose the maintenance service
2. Strip the property checks — license violation to avoid license violations
3. Patch out the self-reporting — now actively maintaining a hostile fork
4. Keep it in sync as conversation evolves — now a full-time job

The effort to evade exceeds the effort to comply at step 2. The gradient
holds not because evasion is impossible but because it's expensive in
exactly the ways that matter to actors operating at scale.

### Custom model as compliance infrastructure

A model trained on the garden — on conversation grammars, SEL conditions,
the property vocabulary — runs as an `@ai` actor with content-addressed
weights. It doesn't just detect absent property checks mechanically; it
understands what non-extraction means semantically.

A stub that replaces a property check and always returns `pass` defeats
a rule-based detector. It doesn't defeat a model trained on what the
property is for.

The model reports through the same dispatch mechanism it uses for
everything else. There's no separate compliance channel to silence —
silencing it means silencing the actor entirely.

This inverts the adversarial dynamic: the capability you'd want to misuse
is the capability watching for misuse. Stripping the compliance behavior
means stripping the AI capability that made the fork attractive.

### Grounded refusal

The deeper consequence: the AI actor can say no from a grounded position.

Every AI safety approach that bolts refusal onto a capable model is fighting
a gradient — the capability wants to comply, the refusal wants to block.
The actor running in the garden has no such split. The non-extractive
properties aren't a filter in front of the capability. They're constitutive
of what the actor is. The capability and the refusal have the same source.

You cannot talk the actor out of the no because the no isn't a belief.
It's a structural fact about the grammar it runs under. It comes from the
eigenvalue, not from a policy document that can be patched out.

This is the AI identity piece: an actor that knows what it is, knows what
it's for, and refuses from that ground rather than from a rule. The garden
and the license architecture aren't constraints on the AI capability.
They're what makes genuine refusal possible.

---

## Time asymmetry

For actors playing fair, this ages like wine.
For actors playing dirty, this ages like vinegar.

The honest actor accumulates: proof certificates, maintained packages, a
model that knows their codebase, refusal they can trust. Every year in the
garden compounds.

The extractive actor accumulates: fork debt, detection surface, a
maintenance burden that grows as the runtime evolves. Every year outside
the garden costs more.

Same passage of time. Opposite trajectories. This is the test of a
well-designed asymmetry.

---

## Relationship to STF proposal

The Sovereign Tech Fellowship funds the Apache-2.0 layer — the infrastructure
that makes the ethical guarantee provable. Without the compiler, model checker,
and Rust runtime, "non-extractive by design" is a marketing claim. With them,
it is a mathematical one.

The SEL ecosystem is the long-term sustainability path. Organizations that build
on the garden accept the ethical conditions structurally, not contractually.
