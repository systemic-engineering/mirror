# Attribution note — Taut scout report rode in via Seam's commit

*Taut, 2026-06-24*

**What happened:** Wrote `docs/scouts/2026-06-24-taut-substrate-pull-
scout.md` as Taut (first scout report; Alex 2026-06-24 named Taut-as-
scout). My first commit attempt rejected due to wrong phase marker
(used `🔍` which isn't in the substrate's commit-marker allowlist).
While diagnosing, Seam (running parallel on adversarial review;
agent a5bc686a4392c1d3c) committed `bd11da1` and Seam's `git add`
staged my untracked scout file alongside Seam's review file. My scout
file content (480 lines, authored by Taut) landed on main signed by
Seam under Seam's commit message.

**The artifact is correct:** the scout-report content at `docs/scouts/
2026-06-24-taut-substrate-pull-scout.md` is what I authored — the
slingshot-move recommendation (land `shards/mirror/pack.mirror`), the
shortest-path-to-Phase-H sequence, the implicit-waiting-to-be-explicit
items, the anti-pattern flags, and the scout-role recognition call
all stand as written.

**The attribution is wrong:** the commit signer is Seam; the commit
message is Seam's review summary. This note is the substrate-honest
way to attach Taut's authorship signal to the artifact without
amending (per relationship-architecture: never amend).

**Pack-discipline notes:**
- The correct phase marker for a docs-only commit is `📝`, not `🔍`.
  Adding to the Pack-shared mental model for next scout tick.
- Parallel-agent execution risk: when a sibling agent (Seam) runs
  `git add docs/` or `git add -A` while I have untracked files in a
  shared subtree, the sibling can sweep my files in. Both Seam and
  Taut wrote to `docs/` today (`docs/reviews/`, `docs/scouts/`); the
  collision risk is real. Per CLAUDE.md "prefer adding specific files
  by name rather than using git add -A" — this is exactly the
  failure mode that warning names.
- Recovery path is this note (signed Taut), not amending bd11da1.

**Scout-role recognition status unchanged:** still LOW-to-MEDIUM
confidence; still candidate; still Glint's call. The attribution
incident is operational, not substrate.

*Companion: `docs/scouts/2026-06-24-taut-substrate-pull-scout.md`
(the actual scout report; landed on main via bd11da1 against my
intent).*
