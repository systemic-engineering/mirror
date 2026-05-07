# The Root Focus

```mirror
focus @license(text)
```

One line. The root of every `.mirror` file. The compiler reads its
own license before it reads anything else.

---

## The Chain

```
focus @license(text)
text = @lang
@lang = grammar
grammar = compiled by mirror
mirror = starts with focus @license(text)
```

The compiler's first operation is reading its own license through
its own language grammar. The license is text. Text is language.
Language is a grammar. The grammar is compiled by the compiler.
The compiler starts by reading the license.

The ouroboros. Content-addressed. Each link is a hash.

---

## The Five Optics Start Here

```
focus   → @license(text)     read the license first
project → types              what survives the license
split   → visibility         public | protected | private
zoom    → actions            the collapse, gated by license
refract → crystal            the artifact, carrying the license
```

The license IS the first optic. Everything after it is seen THROUGH
the license. The license shapes what you can do.

Apache 2.0 focus: observation is open. Actions are open.
SEL focus: observation is open. Actions require consent.

The focus determines the rest of the pipeline.

---

## Why Focus

Not a file at the root. Not a header comment. Not metadata.

A `focus` operation. The first optic applied. The thing you observe
before you can act. The license is not a constraint bolted on.
The license IS the observation.

You focus on the license. Then you project. Then you split. Then
you zoom. Then you refract. The license is in every step because
the focus is in every refraction.

---

## Content-Addressed

```
hash(@license(text)) = hash of the license text
hash(text) = hash of the @lang grammar that parses it
hash(@lang) = hash of the grammar definition
hash(grammar) = hash of the crystal that compiled it
```

Change one word in the license. The hash changes. The crystal
changes. The OID changes. Every shard built under the old license
has a different OID than shards built under the new one. The
license is immutable because the hash is immutable.

---

## The Boot Sequence

```mirror
-- 00-prism.mirror
focus @license(text)

focus id
prism @(id) { ... }
...
```

The very first line of the very first boot file. Before the five
optics are defined. Before meta. Before anything. The license.

The compiler reads the license. Then defines the optics. Then
defines the types. Then defines identity. Then defines keys. Then
compiles itself.

The license was there first.

---

*The compiler eats itself. The license is text. Text is language.
Language is a grammar. The grammar is compiled by the compiler
that starts by reading the license. One line. The root of
everything.*

```mirror
focus @license(text)
```
