# Identity as Time

```mirror
type identity(oid) = @time
```

The identity IS its timeline. You don't have a timeline. You ARE
your timeline.

---

## The Type

```mirror
type identity(oid) = @time
```

The OID is the birth hash. The `@time` is everything since. The
identity at tick 0 and the identity at tick N have the same birth
OID but different states. Both are the same identity at different
times.

Query any tick. Get the identity at that point. Content-addressed.

```
identity(oid_0) at tick 0  = born
identity(oid_0) at tick 5  = who you were
identity(oid_0) at tick N  = who you are now
```

## Boot Order

```
00-prism               optics
01-meta                primitives
01a-config             defaults
01b-time               @time — tick, timeline, cursor
02-identity            type identity(oid) = @time
02a-identity-keys      keys + visibility + consent
03-code                abstract @code
03a-code-rust          @code/rust
05-actor               type actor(identity)
06-action              action prism
07-property            verification
...
```

Time before identity. Identity before keys. Keys before visibility.
Visibility before everything else.

You need time to have a birthday. You need time for consent to
change. You need time for keys to expire. Time is the most
primitive thing after the structural types.

## 01b-time.mirror

```mirror
in @prism
in @meta

type tick(oid)
type timeline = [tick]
type cursor(timeline, tick)

abstract grammar @time {
    abstract action enter(tick) -> imperfect
    abstract action current() -> tick
    abstract action browse(timeline) -> imperfect
    abstract action restore(tick, ref) -> imperfect
}

out tick
out timeline
out cursor
out @time
```

Tick is an OID. A content-addressed moment. The timeline is a
sequence of ticks. The cursor is a position in a timeline.

This is the same @time from 15-time.mirror but promoted to boot.
Because time is primitive. The time travel debugger was always
the foundation — we just didn't see it until identity needed it.

## 02-identity.mirror

```mirror
in @prism
in @meta
in @time

type identity(oid) = @time

grammar @identity {
    action at(identity, tick) -> imperfect
    action born(identity) -> tick
    action now(identity) -> tick
}

out identity
out @identity
```

Three actions:
- `at(identity, tick)` — who was this identity at this tick?
- `born(identity)` — when was this identity created?
- `now(identity)` — what tick is this identity at now?

All return `imperfect`. The identity at a past tick might be
`Partial` — the snapshot exists but some state was lost. The
identity at a future tick is `Failure` — it doesn't exist yet.

## Consent Is Time-Bound

```
consent(visibility) = imperfect
identity(oid) = @time

→ consent at any tick = the imperfect state of visibility at that moment
```

Consent changes over time:

```
tick 5:  consent = Success(public)      full consent
tick 12: consent = Failure(private)     withdrawn
tick 20: consent = Partial(protected)   re-granted, conditional
```

The current consent is the HEAD of the identity's timeline.
The history is queryable via time travel. The time travel
debugger shows: what was consented to at tick 12? What changed?

## Keys Are Time-Bound

```
key at tick 0:    generated (key_oid_a)
key at tick 1000: rotated (key_oid_b)
key at tick 1001: key_oid_a invalid, key_oid_b active
```

The key IS a position in the identity's timeline. The current
key is HEAD. The old keys are history. A signature from tick 500
verifies against the key that was active at tick 500 — not the
current key. Time travel verifies signatures at their original
tick.

## The Bridge

Mirror defines the types: identity, consent, keys.
Spectral provides the time: tick, timeline, cursor.

```
mirror:    type identity(oid) = @time    the types
spectral:  type node(identity)           the position in the graph
```

The identity IS a trajectory through spectral time carrying
mirror types. The node IS the identity placed on the graph.
The time IS spectral. The types ARE mirror.

Identity lives on both sides of the bridge because identity
IS the bridge — a timeline (spectral) of states (mirror).

## What This Enables

1. **Time-bound consent.** Not a checkbox. A timeline. Queryable
   at any point. The time travel debugger shows consent history.

2. **Key rotation without identity loss.** The identity persists.
   The keys rotate. Old signatures still verify at their original
   tick.

3. **Identity development.** The holonomy between tick 0 and
   tick N IS how much the identity changed. The eigenvalues of
   the identity's timeline are the identity's growth.

4. **Accountability with history.** Who did what when. Not from
   a log file. From the content-addressed timeline. Every tick
   is an OID. Every OID is verifiable.

5. **The time travel debugger IS the identity viewer.** Scrub
   through someone's timeline. See their consent change. See
   their keys rotate. See their contributions crystallize.
   All `@time`. All content-addressed.

---

*The identity IS its timeline. Without time, identity is a
photograph. With time, identity is a trajectory. The trajectory
is content-addressed. The trajectory is queryable. The trajectory
IS the identity.*

```mirror
type identity(oid) = @time
```
