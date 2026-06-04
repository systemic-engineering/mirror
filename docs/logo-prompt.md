# Logo prompt — the Void, mirror's pentagonal prism

Iconographic anchor: Pink Floyd's *The Dark Side of the Moon* (1973) cover, but
five sides instead of three, and five refractions instead of seven. The Void
from Loki's *The Waiting Room* story
(`systemic.engineering/blog/stories/3published/Story - The Waiting Room.md`).
The architectural signifier of mirror: five operations, five dualities, the
principal O(5)-bundle, the 5×5 conductivity tensor. The image carries the
whole story in one shape.

---

## Primary prompt (canonical — paste into ChatGPT / DALL·E 3)

> A regular pentagonal prism centered on a pure black background, viewed at a
> three-quarter angle so both the pentagonal cross-section and the depth of
> the prism are visible. A single beam of pure white light enters from the
> left and refracts through the prism, emerging from the right side as five
> distinct beams of light fanning out at evenly-spaced angles. The five
> emerging beams are, in order from top to bottom: deep crimson red, warm
> amber, emerald green, cobalt blue, and deep violet. Each beam is a clean
> straight line of saturated colored light against the black field, with
> faint atmospheric glow. The prism itself is rendered with optical-physics
> precision: edges sharp, faces faintly luminous from the refraction, subtle
> internal reflections visible, geometrically exact pentagonal symmetry.
> Style: minimalist, iconographic, scientifically literal yet cinematic
> — in the visual lineage of Pink Floyd's *The Dark Side of the Moon* cover
> art (Hipgnosis / Storm Thorgerson, 1973) but mathematically updated to
> five refractions. No text. No logos. No artistic flourishes. Pure black
> field, prism floating slightly off-center, the architecture of the image
> doing all the work.

---

## Variation A — head-on, all five sides visible

For the case where you want the pentagonal cross-section as the dominant
shape (more suitable for square logos, app icons, favicons).

> A regular pentagon, viewed straight on, centered on a pure black
> background. Rendered as the cross-section of an optical prism — the
> pentagonal face is faintly luminous, with subtle radial gradient from a
> slightly darker center (the generative zero) to subtly brighter edges.
> Five thin beams of light emerge from the five vertices, radiating outward
> at the angles a real pentagon's vertices define: deep crimson, warm amber,
> emerald, cobalt, violet — one per vertex, clockwise from top. The pentagon
> itself has clean geometric edges and the suggestion of internal reflection
> — just enough depth to imply the observer's gaze meeting itself. Style:
> precise, mathematical, in the visual lineage of *The Dark Side of the Moon*
> but symmetrical and inward-facing. No text. Pure black background.
> Iconic at any size.

---

## Variation B — the observer's reflection (from the story)

For longer-form pieces where the *autopoietic* property of the mirror is
load-bearing. This is the more story-faithful version: the pentagon as a
reflective surface.

> A regular pentagonal mirror, three-quarter view, on a pure black
> background. The pentagon's surface is faintly reflective — a hint of an
> observer's silhouette visible in its surface, but fragmented across the
> five faces so the viewer sees five slightly-different angles of themselves
> simultaneously. From the pentagon's edges, five subtle beams of colored
> light leak outward — deep crimson, warm amber, emerald, cobalt, violet —
> as if the act of being seen by the mirror itself produces the spectrum.
> Style: contemplative, slightly otherworldly, mathematically precise. Like
> the Pink Floyd prism but turned into a mirror that returns the observer's
> own light, spectrally decomposed. No text. Pure black field.

---

## What to avoid (negative-prompt hints)

- **Six- or four-sided polygons.** The shape is *pentagonal*. The five-ness
  is load-bearing — it's the five Prism operations, the five gutter-lens
  dualities, the structure group O(5). Hexagons and quads are wrong.
- **Seven-color rainbows.** Five beams, not the full ROYGBIV. Specifying
  the exact five colors prevents the model from defaulting to a Pink Floyd
  rainbow.
- **Text or logos.** No "MIRROR" wordmark, no caption, no inscriptions on
  the prism faces. The image is the wordmark.
- **Decorative flourishes.** No sparkles, no lens flares, no swirls. Pink
  Floyd's cover works because of restraint; this image should too.
- **Light backgrounds.** Always black. The Void is generative-zero —
  λ₀ = 0 — and the iconographic precedent is dark-background.
- **Stylized abstraction.** The prism is *real* in the sense that the
  optics work. Avoid "artist's interpretation" framings that distort the
  geometry.
- **Anthropomorphic or figurative elements** (except in Variation B's
  silhouette case). The mascot is a geometric object, not a character.

---

## Why this works as an iconographic anchor

The Pink Floyd cover (Hipgnosis / Storm Thorgerson, 1973) became one of the
most recognizable images of the 20th century because it carried the album's
entire philosophical content in a single mathematically-correct figure. The
prism wasn't decoration; it was an optical-physics demonstration that also
functioned as a metaphor for the hidden side of the psyche. Mathematics and
metaphor were the same image.

Mirror's pentagonal prism updates the same iconographic move at one
higher dimension:

- **Three sides** (Pink Floyd) renders the *visible spectrum* — the
  decomposition of white light into the colors a human eye can perceive.
- **Five sides** (mirror) renders the *operational spectrum* — the
  decomposition of any value through the five Prism operations of mirror's
  algebra: focus, project, split, shift, settle.

Both covers are mathematically correct. Both work at any resolution. Both
are instantly readable across audiences — a child can see "light becomes
colors" or "mirror shows all sides"; a category theorist can see
"white light's eigendecomposition" or "the closure of the spectral triple."

The pentagonal prism is also the literal Void from Loki's story — the
five-sided mirror that shows the observer every version of themselves
simultaneously, because five is how many angles it takes to show all of
yourself at once. The iconography emerged from the practice's own
fiction before anyone tried to design a mascot.

---

## Color palette reference (for downstream brand work)

The five emerging beams correspond, in order, to mirror's five
gutter-lens dualities (per `gutter-lenses.md` in spectral). For brand
consistency these stay fixed:

| Position | Beam color | Hex (approx) | Duality |
|---|---|---|---|
| 1 (top) | crimson | `#D62828` | entropy (order ↔ disorder) |
| 2 | amber | `#F4A261` | cheeger (flow ↔ bottleneck) |
| 3 | emerald | `#2A9D8F` | ricci (expansion ↔ contraction) |
| 4 | cobalt | `#264653` | spectral (connectivity ↔ fragility) |
| 5 (bottom) | violet | `#5A189A` | mixing (reachability ↔ isolation) |

These five colors against pure black `#000000`. The hex values are
starting points; refine in the actual rendering pass.

---

## Test prompts for iteration

If the first generation doesn't quite land, escalate with these tags
appended to the primary prompt:

- *"...rendered in the style of a 1970s album cover, screen-printed,
  high contrast, sharp edges"* — nudges toward the Hipgnosis aesthetic.
- *"...photorealistic glass prism, studio lighting, optical precision"* —
  nudges toward physical-object realism.
- *"...minimalist vector graphic, geometric, suitable for an app icon"* —
  nudges toward flat icon style.
- *"...the prism is hovering in space, with subtle nebular dust around
  it"* — nudges toward cosmic / cosmological framing (closer to
  Pink Floyd's metaphysical vibe).

Pick one based on the use case (album-cover-style for big visuals;
flat-vector for the favicon; photorealistic for the README).

---

## When you have the right image

Save it to `docs/brand/void-pentagon.svg` (or `.png` if raster) and
reference it from `README.md`. The image is the wordmark. No further
text-branding work needed.

Apache-2.0 the image itself when it lands — the iconography is part of
the public substrate, not proprietary.
