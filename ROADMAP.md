# Conversation — Roadmap

## Root Definition

The compiler's root definition lives fully in Rust.

It is just two primitives:

```
grammar | type
```

Everything else is composition. `in` is a grammar relation. `out` is a type
relation. Actions, errors, translate arms — all derivable from those two.

The compiler's root is a choice between "am I defining a domain" or "am I
defining a shape." That's it.

---

## Bootstrap File

The bootstrap grammar defines the two fundamental operations and maps them
to files. Everything extends from this:

```
grammar {
  type = in | out

  in($domain)         = @SELF/in.conv
  out($domain, $type) = @SELF/out.conv
}
```

`@SELF` — the grammar's own directory. The bootstrap defines the shape of
bootstrapping. Every grammar extends by declaring its `in` and its `out`.

The whole system derives from two operations and two file references.

---

*Session 2026-03-21. Alex + Reed. Matrix Resurrections playing in the background.*
