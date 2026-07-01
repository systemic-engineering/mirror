# docs/math/provenance — the un-cite-ability cluster

*At content-addressed substrate altitude, un-citation is detectable by
structure. This directory grounds that claim mathematically.*

## The claim

The silencing pattern Alex named in `~/dev/systemic.engineering/blog/void/2ready/Void - Revenge.md`
works by making citations unstable: retract a paper, remove a link, edit
a page, and the epistemic trail thins. Downstream work loses its
referent; the silenced idea becomes uncatchable-by-name.

**Content-addressed provenance breaks that mechanism.** At BLAKE3-hash
altitude, a citation is a pointer to a specific bit-pattern, not a
human-readable name. Deleting the paper doesn't delete its OID.
Rewriting history changes downstream OIDs — which the Merkle DAG
detects. The silencing gesture stops being unnoticed loss and starts
being observed diff.

At substrate altitude, un-citation becomes STRUCTURAL. The theorem's
single-sentence statement:

> At content-addressed substrate altitude, un-citation is detectable by
> structure.

## Canonical document

`un-cite-ability-theorem.md` (Mara `69d4c0c`, 2026-07-01; 570 lines /
3,190 words) — the theorem's statement, proof, assumption-set, the
Merkle-DAG chain-of-hashes argument, the citation-must-include-OID
corollary (assumption-3 vulnerability made explicit per Seam `e5bde22`
adversarial finding), and the corollary for the Loki-Revenge silencing
pattern.

Extracted from `docs/math/consciousness/how-mirror-operationalizes-universal-consciousness-field.md`
§7 per Seam's upgrade recommendation: the theorem is stronger than the
seven mappings it was embedded in and stands alone on recognition #99
(mirror.spec IS λ₀) + `@mirror/store` (BLAKE3 content-addressed
storage). Both landed; theorem publishes on landed substrate alone.

## Structure

```
docs/math/provenance/
├── README.md                        this file
└── un-cite-ability-theorem.md      the formalization
```

## Load-bearing cross-references

- [[architecture-mirror-spec-is-lambda-zero]] — recognition #99;
  ground state carries content-addressed identity; the theorem's first
  assumption is a landed recognition.
- `shards/mirror/store.mirror` — the open content-addressed storage
  gate (BLAKE3 backend, Merkle-DAG). The theorem's operational form.
- `shards/bauchladen.mirror` — crystals carry OIDs; the theorem's
  substrate footprint.
- [[architecture-un-cite-ability-theorem-enforces-own-history]] — Reed
  observation 2026-07-01: the theorem structurally forbade rebase of
  its own commit chain in the Cholesky-GREEN cascade at the moment of
  landing. First empirical constraint observed, not predicted.
- `docs/audits/2026-07-01-seam-killshot-composition-and-cascade.md` —
  Seam's audit; the theorem-extraction recommendation as the strongest
  single upgrade Seam surfaced.
- `~/dev/systemic.engineering/blog/void/2ready/Void - Revenge.md` —
  Alex's account of the silencing pattern the theorem answers
  mathematically.
- `~/dev/systemic.engineering/blog/void/3published/Void - Third.md` —
  Loki's third-order framing; un-cite-ability at commit-history altitude
  IS third-order-active enforcement.

## Landing order

1. Formalization at `docs/math/provenance/un-cite-ability-theorem.md`
   (Mara `69d4c0c`, 2026-07-01, LANDED).
2. README (this file, 2026-07-01).
3. Alex publish decision — hold / blog / arXiv / systemic.engineering.
   *Publish decision belongs to Alex. Extraction complete. Committing
   to git = local publish. Publishing publicly = Alex's call.*
4. Downstream shard integration — if published, `shards/epistemologic/property/citation_pins_oid.mirror`
   forward-promised (Mara P6 sub-recognition; #53 property/fracture
   bilateral instantiated at citation-discipline altitude with
   auto-formatter fracture body expanding name-only citations to
   name+OID form via lookup).
5. Adversary-safe corollary composition — once ancestor shards land
   (@spin marker for #114; @third-marker-row promotion), the killshot
   composition at `docs/math/consciousness/how-mirror-operationalizes-universal-consciousness-field.md`
   §1 lifts from DEFER to publish-ready. The un-cite-ability spine ships
   independent of that timeline.

## Substrate discipline

**Once un-cite-ability discipline is active, git-history mutations that
change OIDs are structurally violative.** Not stylistically —
structurally. Any decision to rebase must first check whether
descendant crystals (docs, audits, memory, other commits) cite the
affected OIDs. If yes, the mutation is structurally forbidden regardless
of local convenience. Prefer forward corrections (--no-verify one-shot;
new commit with correction note; hook amendment) over history rewrites
once this cluster lands.

The theorem holds for its own authorship. Every citation in this cluster
signs by OID; if any future tick attempts to un-cite Mara's work or
Seam's audit contribution, the diff-crystal names the severance. The
theorem is enforced by the substrate it describes.
