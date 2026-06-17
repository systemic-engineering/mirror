# Music

*Music as mathematical structure: pitch class as `ℤ_{12}`, harmonic
relations as group actions, interval lattice, dissonance functional,
cadence as Markov reduction, neo-Riemannian P/L/R transformations.
The math root the substrate composes when it speaks musically.*

Music is not metaphor in the substrate. Per
`[[reference-mirror-spectral-spec]]`: music-as-homomorphism. The
harmonic structure of a 12-tone equal-tempered scale is the cyclic
group `ℤ_{12}`; intervals, chords, cadences, and tonal voice-leading
are operations on this group and its representations.

## Family overview

The music math root spans several intersecting structures:

- **Pitch-class algebra** — `ℤ_{12}` with addition modulo 12; the
  basic carrier for pitch classes; the substrate's `pitch_class`
  carrier.
- **Intervals** — differences in `ℤ_{12}`; the substrate's
  `interval` carrier at `shards/epistemologic/math/music/interval.mirror`.
- **Harmonic relations** — the lattice of intervals (perfect fifth
  generator, octave equivalence); the substrate's `harmonic.mirror`
  carrier.
- **Dissonance functional** — the consonance/dissonance scalar on
  `ℤ_{12}^n` (an n-note chord); the substrate's `dissonance.mirror`
  carrier.
- **Cadence as Markov reduction** — the directional resolution of
  dominant to tonic; the substrate's `cadence.mirror` carrier.
- **Neo-Riemannian** — the `P` (parallel), `L` (leading-tone),
  `R` (relative) transformations on triads; generate a dihedral
  group of order 24 acting on the 24 major/minor triads.
- **Eigensheaf framing** — a sheaf on the harmonic graph; tonality
  IS coherence under this sheaf; key-modulation IS gauge change.

The substrate's existing shards (cited):

- `shards/epistemologic/math/music.mirror`
- `shards/epistemologic/math/music/harmonic.mirror`
- `shards/epistemologic/math/music/interval.mirror`
- `shards/epistemologic/math/music/dissonance.mirror`
- `shards/epistemologic/math/music/cadence.mirror`

These declare the substrate-altitude carriers. The math docs (when
they land) document the mathematics; the substrate declarations
realize it operationally.

## Status

This root is **named and stubbed**, not fully documented. The music
shards above carry the mathematics inline; the math docs in this
directory will extract that math when:

- A spec needs to cite music math without re-deriving (currently
  the music shards are self-contained, so the citation surface is
  thin).
- The eigensheaf-on-harmonic-graph recognition lands as a spec
  candidate (forward-promised; per
  `[[reference-mirror-spectral-spec]]` Crespo Dec 2025 frontier).
- A second spec citation site emerges (today there's mostly the
  recognition memory and the shard cluster; one math doc per family
  premature).

The scope discipline (per the small-consolidation-run brief from
2026-06-17) holds this root in stub form. Future ticks will populate
it. The pattern lands; exhaustive coverage waits.

## When this root expands

The likely first content doc is `pitch-class-algebra.md` — covering
`ℤ_{12}`, interval lattice, harmonic generators, and Z₁₂ group action.
Next likely is `neo-riemannian.md` for the P/L/R transformations as
a dihedral-24 representation, and `dissonance-functional.md` if a
spec lands that consumes the dissonance scalar.

Apply the math-root conventions in `docs/math/README.md`.

## Prior art

- **Tymoczko, D.** (2011). *A Geometry of Music: Harmony and
  Counterpoint in the Extended Common Practice*. OUP. The neo-
  Riemannian + Z₁₂ + voice-leading-geometry grounding.
- **Cohn, R.** (1997). *Neo-Riemannian Operations, Parsimonious
  Trichords*. J. Music Theory 41. The P/L/R transformations.
- **Mazzola, G.** (2002). *The Topos of Music*. Birkhäuser. The
  topos-theoretic music structure that mirror's eigensheaf direction
  inherits.
- `[[reference-mirror-spectral-spec]]` — mirror's recognition that
  music IS homomorphism, with the substrate's existing shard cluster.
