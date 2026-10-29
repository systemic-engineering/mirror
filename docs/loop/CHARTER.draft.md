# Loop charter (DRAFT, unsigned)

*Drafted by Reed 2026-10-29 for Alex to sign. Becomes `CHARTER.md` only by Alex's act (a signed commit by Alex, or Alex's explicit in-transcript approval recorded with its date). The loop may read this file and must never edit `CHARTER.md`.*

## Definition of done (Alex, 2026-10-29)

> "The definition of done is when the compiler can explain its own compilation at every step for any arbitrary shard. And the definition moves to the next logic level where we can cleanly compose without unresolved tritones (after compile → kintsugi)."

> "What is an explanation if not a (non-)linear trace through the past?"

## Level 1: compile explains itself

- **Property:** for every shard S in the corpus, `mirror compile --explain S` emits a certificate (the format is in Mara's spec, TBD), and the independent checker, replaying it in any linear extension of its partial order, rebuilds the identical root. Nothing is unexplained.
- **Checker:** TBD (written independently of the compiler; Seam-audited; frozen at commit TBD).
- **Corpus:** `shards/` at commit TBD, plus Seam's adversarial fixtures at commit TBD. Generated shards (from `mirror.mirror`'s grammar) join when T1 lands, under a generation budget ruler.
- **Evidence, not proof:** "any arbitrary shard" rests on Mara's engine theorems (tick shape ⇒ every derived atom has a derivation from earlier ticks), the corpus, and generation. The level report says which leg covers what.

## Level 2: kintsugi composes without unresolved tritones

- **Property (sketch; Alex defines it when level 1 is signed off):** over the corpus, every comma between shards is either resolved (approved) or explicitly kept; every promised tritone is resolved; and the gluing itself explains itself (why each comma, why each gap).

## Budgets (declared rulers)

- **Per iteration:** TBD (wall-clock or agent count).
- **Per level:** TBD. When exhausted, the loop stops with UNKNOWN and reports.

## The loop may never change

The checker, the corpus commits, expected values, hooks/guards, audit scopes, this charter, and the level definitions. Anything that would change one of these is a comma for Alex.

## Permissions (Alex to confirm)

- **May:** commit on `reed/ship`; push `reed/ship`; fast-forward `main` when the full gate is green.
- **Never:** publish; touch private or consent-bound content; sign as anyone but its own identity; auto-approve a comma resolution; edit this charter.

## Signature

- [ ] Alex: level 1 property, checker commit, corpus commit, budgets, permissions.
