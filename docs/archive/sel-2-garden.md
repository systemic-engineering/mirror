# SEL-2.0: The Garden Incentive

```mirror
requires hosted(@git) <= Success(@garden)
maintenance = free
```

Free maintenance. Earned by compiling under SEL-2.0 and being
accepted by the garden.

---

## The License Chain

```
SEL-1.0    the ethics (anti-extraction, consent, witnessed)
SEL-2.0    SEL-1.0 + hosted(@garden) + free maintenance
```

SEL-2.0 inherits every SEL-1.0 property. Adds one requirement
and one reward:

```mirror
-- license-sel-2.mirror
in @license/sel

@license(oid("./sel/LICENSE-2.0")) = @license(sel2)

requires hosted(@git) <= Success(@garden)

maintenance = free
```

## The Property

```mirror
requires hosted(@git) <= Success(@garden)
```

`hosted(@git)` — your project is in git. Always.

`<= Success(@garden)` — the garden accepted it. The `<=` assigns
left. The garden receives the git-hosted source. The `Success`
means: the garden verified your project and accepted it.

```
hosted(@git) <= Success(@garden)    accepted → free maintenance
hosted(@git) <= Partial(@garden)    conditional — properties partially met
hosted(@git) <= Failure(@garden)    not in garden → pay for maintenance
```

The hosting is always `@git`. The question is whether the garden
accepted it. `Success` means full acceptance, public, maintained.
`Partial` means conditions apply. `Failure` means not in garden.

The garden is selective:

```
Imperfect<@garden, RejectionReason, HostingLoss>
```

The garden might reject your project. The `RejectionReason` tells
you why. License violation. Property failure. Consent missing.
The `HostingLoss` measures what's missing.

Free maintenance is earned. Earned by compiling under SEL-2.0.
Earned by hosting in git. Earned by the garden saying `Success`.

## The Business Model

```
closed source + SEL-1.0:    10% of projected savings
open source + SEL-2.0:      free maintenance
```

Closed source: the spectral runtime maintains your project. The
billing is the MirrorLoss delta — holonomy before minus holonomy
after. The invoice is a content-addressed shard. 10% of savings.

Open source in the garden: the spectral runtime maintains your
project for free. Same maintenance. Same agents. Same eigenvalues.
No bill. The garden funds it.

## What Free Maintenance Means

```
garden.systemic.engineering/@yourproject

  nightly merge         free
  eigenvalue analysis   free
  holonomy measurement  free
  property verification free
  gutter rendering      free
  agent suggestions     free
  crystal updates       free
  timeline history      free
```

Everything the spectral runtime does. For free. For projects in
the garden.

The closed source projects that pay 10% fund the infrastructure
that maintains the open source projects for free. The open source
projects make the ecosystem valuable. The ecosystem attracts the
closed source projects.

## The Compound Growth

```
project adopts SEL-2.0
  → hosted in garden
    → free maintenance
      → holonomy decreases
        → project improves
          → more users
            → more projects adopt SEL-2.0
              → more free maintenance
                → the garden grows
```

The incentive IS the growth. The growth IS the incentive.

## The Garden Is Selective

Not every project gets in. The properties must hold:

```
requires no_implicit_consent       SEL-1.0
requires reciprocal_flow           SEL-1.0
requires symmetric_observation     SEL-1.0
requires sustainable_stock         SEL-1.0
requires consent_at_boundary       SEL-1.0
requires protect_witnessed         SEL-1.0
requires no_structural_harm        SEL-1.0
requires attribution               SEL-1.0
requires hosted(@git) <= Success(@garden)   SEL-2.0
```

All SEL-1.0 properties plus garden hosting. The compiler verifies
every property. The garden checks the verdict. `Success` = accepted.

## The Live Garden

```
garden.systemic.engineering

  open source projects:
  ● tokio          crystal   holonomy: 0.000   free
  ● serde          crystal   holonomy: 0.000   free
  ● mirror         settling  holonomy: 0.023   free
  ● your-project   partial   holonomy: 0.089   free

  maintained by spectral. funded by closed source clients.
  the gutter breathes on code that costs nothing to maintain.
```

## Content-Addressed License Chain

```
SEL-1.0   oid: a7f3...
SEL-2.0   oid: b2c1... (contains a7f3...)
```

The OID chain. SEL-2.0 contains SEL-1.0's hash. The lineage is
verifiable. Change one word in either license → the hash changes
→ the crystal changes → the projects are recompiled → the garden
re-verifies.

---

*The garden grows because open source projects maintained for free
attract more open source projects. The closed source projects fund
the maintenance. The incentive is a property. The property compiles.
The compiler is the business model.*

```mirror
requires hosted(@git) <= Success(@garden)
maintenance = free
```
