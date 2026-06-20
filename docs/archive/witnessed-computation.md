# Witnessed Computation

The code that ran is the proof that it ran.

---

## The Audit Chain

The spectral hash encodes the path, not just the result.

```
artifact_oid: a7f3...
trace: [
    grammar_oid: b2c1...       the .mirror file that compiled
    parse_oid: 4f8a...         the parse tree
    resolve_oid: 7d2e...       the resolved types
    compile_oid: e3b9...       the emitted output
    fate_trace: [abyss(2), explorer(1), crystal]
    holonomy: 0.003
    loss: ConvergenceLoss(0.23)
]
```

Every OID is a content address. Every content address is verifiable.
The client walks the chain backwards from the artifact to the source
code that produced it.

Not the binary. The grammar. The `.mirror` file. The human-readable
code that ran.

## The Audit Is a Proof

The client doesn't read a log file and trust it. The client recomputes
the hash at any point in the chain.

If `parse_oid` hashes the parse tree and produces `4f8a...`, the parse
was real. If it doesn't, someone lied.

Every step is independently verifiable. The chain is tamper-evident by
construction. Not because someone added signing. Because content
addressing IS signing. The content IS the proof. The address IS the
signature.

The auditor doesn't need access to the server.
The auditor doesn't need trust.
The auditor doesn't need credentials.
The auditor needs the artifact and a hash function.

## Sub-Turing Decidability

The `.mirror` files are sub-Turing. Decidable. The auditor doesn't
just verify that the code ran. The auditor can verify:

- The code terminates
- It cannot loop
- The grammar is well-formed
- The types resolve
- The properties hold

Not "we tested it." Not "we reviewed it." Proven. By the structure
of the language itself.

---

## The systemic.engineering License as Grammar

The SEL was a legal document that described structural properties.
The `@systemic/license` grammar is those properties made executable.

### §3.1 Anti-Extraction → Hash Attribution

The spectral hash chain proves what was computed. Cognitive labor
appears in the trace. The content address IS the attribution.
Extraction becomes detectable by hash — not by lawyers, by the
protocol.

You can't remove attribution without changing the hash. Changing
the hash breaks verification. Breaking verification is detectable.

### §3.2 Consent → Typed State

```mirror
type consent = granted | withdrawn | silent

action proceed(consent) -> imperfect {
    silent    -> Failure(no_consent, zero)
    withdrawn -> Failure(consent_withdrawn, accumulated_loss)
    granted   -> Partial(result, exchange_loss)
}
```

Silence returns `Failure(no_consent, L::zero())` — a failure that
cost nothing because the system didn't proceed. The grammar cannot
advance past a consent boundary without an explicit signal.

Not by design choice. By sub-Turing decidability. The compiler
proves it terminates without consent.

### §3.3 Protect the Witnessed → Hash Lookup

The Witnessed can verify what was observed by recomputing the hash.
Access to their own observation data isn't a policy — it's a hash
lookup.

The human decision point between observation and action is a typed
boundary in the grammar. Automated intervention without it doesn't
compile.

### §3.4 Structural Harm → Measured Curvature

Discriminatory effects accumulate in the holonomy. A system that
concentrates load along any axis produces measurable curvature.

The MirrorLoss tells you what the system is actually optimizing for.
Not what the docs say. Not what the founder claims. What the graph
shows.

### §3.5 Attribution → Content Addressing

Content addressing IS attribution. The provenance is in the hash
chain. Removing the attribution changes the hash. Changing the hash
breaks verification. The proof is structural.

---

## The Protocol Enforces the License

The license doesn't need enforcement. The protocol enforces it.

Not "we'll sue you if you extract." The grammar won't compile if
you extract. The hash chain proves you extracted. The loss shows
where the extraction happened. The Witnessed can verify independently
with nothing but the artifact and a hash function.

Every exchange that runs on Mirror runs on the SEL. Not because
you agreed to it. Because the grammar won't compile without it.

---

## The Line

Line 25 of the systemic.engineering License:

> "The ethics of this practice are not cosmetic. They are structural."

Now they're in the type system.

---

*The work is witnessed. Do it accordingly.*
