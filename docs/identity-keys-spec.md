# Identity + Keys Spec

Identity is the root. Keys serve identity. Visibility serves keys.
Consent IS Imperfect over visibility. Visibility IS key operations.

---

## Boot Order

```
00-prism               optics
01-meta                primitives
01a-config             defaults (NOT visibility — that moves to identity)
02-identity            who you are
02a-identity-keys      keys + visibility + consent
03-code                abstract @code
03a-code-rust          @code/rust
03b-code-nix           @code/nix
04-lang                abstract @lang
04a-lang-eng           @lang/eng
05-actor               abstract @actor
05a-actor-user         @actor/user (consent from identity-keys)
05b-actor-ai           @actor/ai
06-action              action prism
07-property            verification
08-git                 @git
09-package             @package + mirver
10-mirror              the mirror grammar
11-spec                deployment
12-ci                  measure
13-ca                  observe + suggest + enforce
14-lsp                 editor protocol
15-time                time travel
16-tui                 panels + gutter
17-benchmark           measurement
18-license             @license
18a-license-apache2    observation = @license(@apache2)
18b-license-sel        action = @license(@sel)
20-cli                 command surface
```

## 02-identity.mirror

```mirror
in @prism
in @meta
in @actor

type identity(actor) {
    name: string,
    oid: oid,
}

out identity
```

Identity IS an actor. An identity can start, send, stop. An identity
can be observed. An identity can consent. The identity IS the base
actor with a name and a content-addressed OID.

Every commit is signed by an identity. Every shard carries an identity.
Every observation references an identity. Every consent belongs to an
identity.

## 02a-identity-keys.mirror

```mirror
in @prism
in @meta
in @identity

type key
type keypair(key, key)

type visibility(identity) = private(@keys.encrypt)
                          | protected(@keys.sign)
                          | public(@keys.verify)

type consent(visibility) = imperfect

grammar @keys {
    action encrypt(identity, type) -> imperfect
    action sign(identity, type) -> imperfect
    action verify(identity, type) -> imperfect
}

default(visibility) = protected

out key
out keypair
out visibility
out consent
out @keys
```

## The Type Collapse

```
visibility = private | protected | public
           = @keys.encrypt | @keys.sign | @keys.verify
           = the crypto operation IS the visibility level

consent = imperfect over visibility
        = Success(public/@keys.verify)
        | Partial(protected/@keys.sign, consent_loss)
        | Failure(private/@keys.encrypt, consent_loss)

consent(private)   = Failure  = encrypted  = can't read without key
consent(protected) = Partial  = signed     = can read, signature proves who
consent(public)    = Success  = verified   = open, provenance verified
```

One type. Visibility is keys is consent is Imperfect. All the same.

## How It Flows

### type(private) — encrypted by construction

```mirror
type(private) patient_data { name: string, diagnosis: string }
```

This is `type(@keys.encrypt) patient_data`. The type IS encrypted.
The compiler requires `@keys.encrypt(identity, patient_data)` before
storage. The encryption is not a policy. It's the type.

### type(protected) — signed by construction

```mirror
type(protected) team_metrics { velocity: f64, holonomy: f64 }
```

This is `type(@keys.sign) team_metrics`. The type IS signed. The
compiler requires `@keys.sign(identity, team_metrics)` at authorship
boundaries. The signature proves who produced it.

### type(public) — verifiable by construction

```mirror
type(public) api_response { status: string, data: string }
```

This is `type(@keys.verify) api_response`. The type IS verifiable.
Anyone can read. The compiler requires `@keys.verify(identity, api_response)`
for provenance. The verification proves it wasn't tampered with.

## Consent in Actions

```mirror
action observe(identity, user) -> imperfect {
    -- user.consent is imperfect over visibility
    -- the match IS the consent check

    match user.consent {
        Success(public) => {
            -- full consent. proceed. data is verifiable.
            let data = record(user);
            @keys.verify(identity, data)
        }
        Partial(protected, loss) => {
            -- conditional consent. proceed within scope. data is signed.
            let data = record_scoped(user, loss.scope);
            @keys.sign(identity, data)
        }
        Failure(private, _) => {
            -- no consent. stop. data stays encrypted.
            Failure(no_consent, zero)
        }
    }
}
```

The match on consent IS the license check. The compiler verifies
that every path respects the consent state. The Failure path can't
proceed. The Partial path is scoped. The Success path is open.

## License Properties — Now Structural

The heuristic-based checks (v1) become structural with typed identity:

```rust
fn check_no_implicit_consent(form: &Form) -> Option<LicenseViolation> {
    for action in collect_actions(form) {
        // structural: does action take a user type?
        let takes_user = params_have_type(&action, "user");
        // structural: does action match on user.consent?
        let checks_consent = body_matches_on("consent", &action);

        if takes_user && !checks_consent {
            return Some(LicenseViolation {
                clause: "§3.2.2".into(),
                property: "no_implicit_consent".into(),
                message: format!(
                    "action '{}' takes user without matching on consent",
                    action.name
                ),
            });
        }
    }
    None
}
```

Not "does a string contain 'consent'." Does the action match on the
consent type. The type IS the check. Rename anything you want. The
types are the types.

## Connection to Existing Types

### @actor/user uses consent

```mirror
type user(actor) {
    identity: identity,
    consent: consent(visibility),
    observations: [observation],
    data: [type(private)],
}
```

The user's consent IS Imperfect over visibility. The user's data
IS private (encrypted). The user's observations are typed. The
identity ties it together.

### Shard carries identity

```rust
pub struct Shard<V, H: HashAlg = CoincidenceHash> {
    value: V,
    oid: MirrorOid<H>,
    identity: Identity,          // who produced this
    visibility: Visibility,      // consent level
    signature: Option<Signature>, // proof of identity
}
```

Every shard knows who made it, what visibility it has, and carries
the cryptographic proof. The Shard IS the witnessed artifact.

### @git uses identity for commits

```mirror
action commit(identity, shard) in @code/rust {
    @keys.sign(identity, shard);
    git_commit(shard, identity);
}
```

The commit is signed by the identity. The signature is the proof.
The identity is the author. The OID is the content. All one type.

## What This Enables

1. **No heuristic license checks.** The types enforce consent.
2. **Crypto is structural.** Private = encrypted. Protected = signed.
3. **Identity is the root.** Everything traces back to who.
4. **Consent is Imperfect.** Three states. Measured loss. Not a boolean.
5. **Visibility is keys.** One concept. Not three systems bolted together.
6. **The compiler IS the security model.** Not policy. Types.

---

*Identity before keys. Keys before visibility. Visibility before
everything else. You need to know WHO before you can know WHAT
THEY CAN SEE.*
