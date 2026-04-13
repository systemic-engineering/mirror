# Visibility as Crypto Boundary

The visibility modifier on a type IS the encryption boundary.
Not access control bolted on. Typed crypto compiled into the grammar.

---

## The Three Levels

```mirror
type(private) patient_data { name: string, diagnosis: string }
type(protected) team_metrics { velocity: f64, holonomy: f64 }
type(public) api_response { status: string, data: string }
```

**`private`** — encrypted with the owner's key. At rest. In transit.
In the shard. In the graph. Everywhere. Only the owner decrypts.

**`protected`** — encrypted with the team's key. Clear within the
organization boundary. Encrypted between organizations. The `in`
keyword determines the boundary — `in @org` defines the team.

**`public`** — signed but not encrypted. Anyone can read. The
signature proves who produced it. The MirrorOid proves the content
wasn't tampered with. Clear text, verified authorship.

---

## The Compiler Enforces It

A `private` type cannot leak into a `public` context:

```mirror
type(private) secret { key: string }
type(public) response { data: string }

action leak(secret) -> response {
    response { data: secret.key }  -- COMPILE ERROR
}
```

The grammar won't compile. The type system prevents the leak.
Not a runtime check. Not a linter warning. The types are
different shapes. `private(string)` is not `public(string)`.
They don't fit.

Explicit declassification requires `ehncrypt!`:

```mirror
action declassify(secret) -> response {
    ehncrypt! {
        let cleared = authorize(secret)?;  -- checks consent
        response { data: cleared.key }
    }
}
```

The `ehncrypt!` macro:
1. Checks consent (the authorization is a typed state)
2. Decrypts the private value
3. Returns the declassified value with `DeclassificationLoss`
4. The loss records: who declassified, when, why, what consent was given

The declassification is `Imperfect`:
- `Success` — authorized, consent given, declassified
- `Partial` — authorized but with conditions (time-limited, scope-limited)
- `Failure` — not authorized, consent not given, stays encrypted

---

## The Macros

```
eh!        the terni pipeline — loss accumulates
ehncrypt!  the pipeline with encryption at visibility boundaries
ehnsign!   the pipeline with signatures at authorship boundaries
```

### ehncrypt!

Every `Imperfect` crossing a visibility boundary gets encrypted:

```rust
ehncrypt! {
    let private_data = fetch_patient(id)?;     -- private, encrypted
    let summary = summarize(private_data)?;     -- still private
    let report = anonymize(summary)?;           -- declassify to protected
    report                                       -- protected output
}
```

The macro tracks visibility through the pipeline. If the output
visibility is lower than the input visibility (private → protected
or protected → public), the macro requires explicit consent at the
boundary. The consent is typed. The loss records the crossing.

### ehnsign!

Every shard crossing an authorship boundary gets signed:

```rust
ehnsign! {
    let compiled = mirror.compile(source)?;     -- signed by compiler
    let reviewed = seam.review(compiled)?;       -- signed by reviewer
    let deployed = spec.apply(reviewed)?;        -- signed by deployer
    deployed                                     -- carries all three signatures
}
```

The shard carries a signature chain. Each step adds a signature.
The chain proves: who compiled it, who reviewed it, who deployed it.
Content-addressed AND signed. The MirrorOid proves the content.
The signatures prove the chain of custody.

---

## The Shard

```
Shard {
    value: V,
    oid: MirrorOid,                    -- content hash (coincidence)
    visibility: Visibility,             -- private | protected | public
    signatures: Vec<Signature>,         -- chain of custody
    encryption: Option<EncryptedEnvelope>,  -- if private or protected
}
```

A `private` shard in the store is encrypted. The MirrorOid is
computed from the PLAINTEXT (so the content address is stable),
but the stored bytes are encrypted. Only the key holder can read.

A `protected` shard is encrypted with a group key. Anyone `in @org`
can decrypt. Cross-org requests go through the consent boundary.

A `public` shard is plaintext with signatures. Anyone can read.
The signatures prove provenance.

---

## The SEL Connection

The systemic.engineering License (§3) maps directly:

**§3.1 Anti-extraction** → the signature chain proves attribution.
Removing attribution changes the hash. Changing the hash breaks
verification. The content address IS the attribution.

**§3.2 Consent** → declassification requires typed consent.
`ehncrypt!` checks consent before crossing visibility boundaries.
Silence returns `Failure(no_consent, L::zero())`. The grammar
won't compile without consent.

**§3.3 Protect the Witnessed** → `private` type on observation data.
The Witnessed's data is encrypted. Access requires their key.
The audit trail (who accessed, when, why) is in the `DeclassificationLoss`.

**§3.4 Structural harm** → the loss accumulates. Discriminatory
patterns produce measurable curvature in the holonomy. The
eigenvalue structure of the loss graph shows what the system
is actually optimizing for.

**§3.5 Attribution** → content addressing IS attribution.
The MirrorOid IS the provenance. The signature IS the author.

---

## The Properties

```mirror
property no_private_leak(grammar) {
    fold all_actions
    traversal visibility_crossings
    lens authorized
    refract verdict
}

property signed_chain(grammar) {
    fold all_shards
    traversal signatures
    lens complete_chain
    refract verdict
}

property consent_at_boundary(grammar) {
    fold all_declassifications
    traversal consent_state
    lens granted
    refract verdict
}
```

The compiler verifies:
- No private type leaks to public without `ehncrypt!`
- Every shard has a complete signature chain
- Every declassification has explicit consent

At compile time. Not runtime. The grammar is sub-Turing.
The verification is decidable. The properties are provable.

---

*The visibility modifier is not access control.
It's a crypto boundary compiled into the type system.
The encryption is structural. The consent is typed.
The loss records every crossing.
The compiler proves no leaks.
The license enforces itself.*
