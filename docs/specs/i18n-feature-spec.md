# Mirror i18n Feature Spec

**Author:** Reed
**Date:** 2026-04-14
**Status:** Draft (research + spec, no code)

---

## 1. Research: GNU gettext / .po File Format

### Format Structure

The `.po` (Portable Object) file format is the GNU gettext standard for human-readable
translation catalogs. The pipeline:

```
source code → xgettext → .pot (template) → translator → .po (translated) → msgfmt → .mo (binary)
```

A `.po` file is a sequence of entries. Each entry has:

```po
# translator comment
#. extracted comment (from source)
#: source reference (file:line)
#, flags (fuzzy, c-format, etc.)
#| previous msgid (for fuzzy matching)
msgctxt "disambiguation context"
msgid "untranslated string"
msgstr "translated string"
```

**Header entry:** The first entry has an empty `msgid ""` and a `msgstr` containing
metadata as key-value pairs separated by `\n`:

```po
msgid ""
msgstr ""
"Content-Type: text/plain; charset=UTF-8\n"
"Plural-Forms: nplurals=2; plural=(n != 1);\n"
```

**Plural forms:** A different entry structure handles plurals:

```po
msgid "one file changed"
msgid_plural "%d files changed"
msgstr[0] "eine Datei geaendert"
msgstr[1] "%d Dateien geaendert"
```

**Context (`msgctxt`):** Disambiguates identical source strings with different meanings.
An entry with `msgctxt "menu"` and one with `msgctxt "button"` for the same `msgid "Open"`
are distinct translations. Empty context and absent `msgctxt` are NOT the same.

### xgettext Extraction

xgettext scans source files for marked strings (e.g., `gettext("...")`, `_("...")`)
and produces a `.pot` template. It supports:
- Configurable marker functions via `-k` flag
- Automatic comment extraction (`///` or configurable prefix)
- Source location tracking (`#: file:line`)
- Multiple input languages (C, Python, Java, etc.)

### msgfmt Compilation

`msgfmt` compiles `.po` to `.mo` (Machine Object) binary format:
- Hash table for O(1) lookup by msgid
- NUL-terminated strings
- Plural forms stored as singular + NUL + plural in the same entry
- Only the singular participates in hash table lookup

### Plural Form Rules

The `Plural-Forms` header contains a C expression evaluated at runtime:

| Language | nplurals | Expression |
|----------|----------|-----------|
| English, German | 2 | `n != 1` |
| French, Brazilian Portuguese | 2 | `n > 1` |
| Japanese, Chinese, Korean | 1 | `0` (no plural) |
| Polish | 3 | `n==1 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 \|\| n%100>=20) ? 1 : 2` |
| Arabic | 6 | `n==0 ? 0 : n==1 ? 1 : n==2 ? 2 : n%100>=3 && n%100<=10 ? 3 : n%100>=11 ? 4 : 5` |
| Russian | 3 | `n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 \|\| n%100>=20) ? 1 : 2` |

These are CLDR plural categories mapped to integer indices. The categories are:
`zero`, `one`, `two`, `few`, `many`, `other`.

### Tooling Ecosystem

- **Poedit** — GUI editor for .po files, built-in xgettext integration
- **Weblate** — web-based translation management, git integration
- **Crowdin** — cloud translation platform, .po import/export
- **Lokalise** — API-first TMS with .po support
- **Transifex** — collaborative translation, gettext as first-class format

The .po ecosystem is the most mature translation tooling pipeline in existence.
Every TMS supports it. Every translator knows it.

---

## 2. Research: Rust i18n Crates

### `gettext-rs` / `gettext`

GNU gettext FFI bindings for Rust. Calls into libintl at runtime.

- **Extraction:** Uses standard xgettext on Rust source files
- **Compilation:** Standard msgfmt
- **Runtime lookup:** `gettext::gettext("msgid")` calls libintl
- **Fallback:** Relies on system locale negotiation (LANGUAGE env var chain)
- **Plurals:** `ngettext(singular, plural, n)` — uses the .mo plural formula
- **Platform:** Linux and Windows only (links C library). Not macOS-native.
- **Verdict for mirror:** Wrong fit. C FFI dependency, platform-limited, runtime-only.

### `fluent` / `fluent-rs`

Mozilla's Project Fluent implementation in pure Rust. The most expressive
localization system available.

- **Format:** `.ftl` (Fluent Translation List) files, not `.po`
- **Extraction:** No standard extractor. Manual or custom tooling.
- **Compilation:** No binary compilation step. `.ftl` files parsed at startup.
- **Runtime lookup:** `FluentBundle::get_message(id)` then `format_pattern()`
- **Fallback:** `fluent-fallback` crate provides `Localization` struct with
  locale fallback chain negotiation. `fluent-langneg` handles BCP47 negotiation.
- **Plurals:** CLDR plural rules via `intl_pluralrules` crate. Categories
  (zero/one/two/few/many/other) in selector syntax:
  ```ftl
  emails =
    { $count ->
        [one] You have one email.
       *[other] You have { $count } emails.
    }
  ```
- **Key advantage:** Asymmetric localization. Each language controls its own
  grammar. English might not need gender, but German does — each `.ftl` file
  can use different variables and selectors. The source language doesn't
  constrain the target.
- **Verdict for mirror:** Strong conceptual fit. Pure Rust, CLDR-native,
  asymmetric localization matches mirror's philosophy. But: no standard
  extraction, no binary compilation, `.ftl` format not `.po`.

### `rust-i18n`

Macro-based i18n. `t!("key")` macro expanded at compile time.

- **Format:** YAML, JSON, or TOML translation files
- **Extraction:** `cargo i18n` CLI extracts `t!()` calls from source
- **Compilation:** Embedded in binary at compile time via proc macro
- **Runtime lookup:** HashMap lookup, locale set globally
- **Fallback:** Simple chain: `zh-CN` -> `zh` -> default
- **Plurals:** No built-in plural support
- **Verdict for mirror:** Too simple. No plurals, no CLDR, no asymmetric localization.

### `i18n-embed`

Trait-based i18n with embedded resources. Supports both Fluent and gettext backends.

- **Architecture:** `LanguageLoader` trait with `FluentLanguageLoader` and
  `GettextLanguageLoader` implementations
- **Extraction:** `cargo-i18n` subcommand handles extraction for both backends
- **Compilation:** Translations embedded in binary via `rust-embed`
- **Runtime lookup:** Via the loader trait, backend-specific
- **Fallback:** `LanguageRequester` trait for system locale detection,
  `fluent-langneg` for negotiation
- **Plurals:** Delegated to backend (Fluent: CLDR, gettext: formula)
- **Verdict for mirror:** Best existing architecture. The trait abstraction
  is right. But mirror doesn't need a generic backend — it needs OID-keyed
  lookup with content-addressed extraction.

### Summary Table

| Crate | Format | Extraction | Plurals | Pure Rust | Asymmetric |
|-------|--------|-----------|---------|-----------|------------|
| gettext-rs | .po/.mo | xgettext | C formula | No (FFI) | No |
| fluent-rs | .ftl | Manual | CLDR | Yes | Yes |
| rust-i18n | YAML/JSON | cargo i18n | No | Yes | No |
| i18n-embed | .ftl or .po | cargo-i18n | Backend | Yes | If Fluent |

---

## 3. Research: Elixir/BEAM i18n

### `gettext` (Elixir)

The standard i18n library for Elixir/Phoenix. Part of every new Phoenix project.

- **Macro-based extraction:** `gettext("text")`, `dgettext("domain", "text")`,
  `pgettext("context", "text")`, `dngettext("domain", "singular", "plural", n)`
- **Compile-time validation:** Macros require string literals — `gettext(variable)`
  is a compile error. This enables automatic extraction without runtime analysis.
- **Domain system:** Each `.po` file is a domain. `errors.po` holds error messages,
  `default.po` holds everything else. `dgettext("errors", "not found")` looks in
  `errors.po`.
- **PO file structure:** `priv/gettext/LOCALE/LC_MESSAGES/DOMAIN.po`
- **Extraction:** `mix gettext.extract` scans all modules for gettext macro calls,
  produces `.pot` templates. `mix gettext.merge` merges templates into existing
  `.po` files, preserving existing translations.
- **Runtime:** `Gettext.put_locale("de")` sets locale for the current process.
  Process-local — no global mutable state. Perfect for concurrent web servers.
- **Interpolation:** `gettext("Hello %{name}", name: "Alex")` — compile-time
  validated interpolation keys.

**What mirror can learn:** Compile-time extraction via macros is the right model.
Mirror's `~l` sigil IS the extraction marker. The domain system maps to grammar
domains (`@changelog`, `@release`, `@cli`). Process-local locale is the BEAM
pattern mirror inherits.

### `ex_cldr`

Unicode CLDR data for Elixir. 700+ locales. Handles:
- Number formatting (1,234.56 vs 1.234,56)
- Date/time formatting
- Currency formatting
- List formatting ("A, B, and C" vs "A, B und C")
- Unit formatting
- Plural rules (CLDR categories, not gettext C expressions)

**Integration with gettext:** `ex_cldr` transliterates gettext's POSIX locale
names (underscores: `en_US`) to Unicode format (hyphens: `en-US`) automatically.

**What mirror can learn:** CLDR is the right plural rule source. Mirror should
use CLDR categories (one/other/few/many/zero/two), not gettext C expressions.
The C expressions are an implementation detail of GNU gettext that predates CLDR.

### BEAM UTF-8 Handling

The BEAM handles UTF-8 strings natively:
- Erlang binaries are byte sequences; `<<"hello"/utf8>>` is explicit UTF-8
- Elixir strings are always UTF-8 binaries
- Pattern matching works on grapheme clusters
- `String.length/1` counts graphemes, not bytes
- No encoding conversion needed at the NIF boundary — mirror's Rust side
  produces UTF-8, the BEAM side consumes it directly

---

## 4. Design: OID as msgid

### The Core Insight

Traditional gettext uses the English source text as `msgid`:
```po
msgid "The file was removed"
msgstr "Die Datei wurde entfernt"
```

This has three problems:
1. **Key instability:** Changing the English text changes the key, orphaning all translations.
2. **Ambiguity:** Same English text, different meaning — requires `msgctxt` as a patch.
3. **No type safety:** The msgid is a plain string. Nothing connects it to the
   grammar type that produced it.

Mirror uses content-addressed OIDs as translation keys. Every `~l` sigil in
mirror source produces an OID from the SHA-512 hash of:

```
content_address = SHA-512(grammar_domain + ":" + sigil_type + ":" + template_text)
```

This OID is the `msgid`. It is:
- **Stable:** The OID changes only when the template text OR its grammar context
  changes. Renaming an English phrase to a better English phrase is a deliberate
  act that correctly orphans the old translation.
- **Unique:** The grammar domain + sigil type prevent collisions between identical
  text in different contexts. No `msgctxt` needed — the context is in the hash.
- **Typed:** The OID connects to a grammar type. The `.po` entry carries the type
  as an extracted comment. Translators see the type. Tools can validate the
  interpolation variables match the type's fields.

### .po File Format for Mirror

```po
# Mirror i18n — generated by `mirror craft --i18n`
# Grammar: @changelog
# Language: en (source)

msgid ""
msgstr ""
"Content-Type: text/plain; charset=UTF-8\n"
"Language: en\n"
"Plural-Forms: nplurals=2; plural=(n != 1);\n"
"X-Mirror-Grammar: @changelog\n"
"X-Mirror-Version: 0.1.0\n"
"X-Mirror-OID-Algorithm: sha512\n"

#: src/release.mirror:12
#. grammar: @changelog
#. type: breaking_change
#. template: "The {type} was removed"
#. variables: type:nominative
msgctxt "@changelog.breaking_change"
msgid "a1b2c3d4e5f6"
msgstr "The {type} was removed"

#: src/release.mirror:15
#. grammar: @changelog
#. type: summary
#. template: "{count} file changed"
#. template_plural: "{count} files changed"
#. variables: count:cardinal
msgctxt "@changelog.summary"
msgid "f7e8d9c0b1a2"
msgid_plural "3456789abcde"
msgstr[0] "{count} file changed"
msgstr[1] "{count} files changed"
```

### Design Decisions

**Q: Should msgid be the full OID or a short hash?**

Short hash (first 12 hex chars = 48 bits). Full SHA-512 is 128 hex chars — too
long for translator tools. 12 chars gives 2^48 = 281 trillion possible values.
Collision probability for 10,000 strings: ~1.8 * 10^-8. Acceptable.

The full OID is stored in an extracted comment (`#.`) for tooling that needs it.
The short hash is the working key.

**Q: Should the original English template be in a comment or in `msgstr`?**

Both. The English template appears as:
- `#. template:` extracted comment — for translator context, always present
- `msgstr` in the source-language `.po` file (e.g., `en/mirror.po`) — the
  English IS a translation, not a special case

This means the source language `.po` file is self-describing: a translator
opening `en/mirror.po` sees every OID mapped to its English text. A translator
opening `de/mirror.po` sees OIDs with German text, plus the English template
in the `#.` comment for reference.

**Q: How to handle plural forms with OID keys?**

Each plural form gets its own OID. The singular template hashes to one OID
(`msgid`), the plural template hashes to another (`msgid_plural`). Both are
content-addressed independently. This means:
- Changing only the plural text orphans only the plural translations
- Changing only the singular text orphans only the singular translations
- Each is independently tracked for staleness

The `Plural-Forms` header uses CLDR-based expressions, not arbitrary C. Mirror
normalizes the gettext plural formula to CLDR categories at extraction time and
validates that each target language's `.po` file has the correct `nplurals` for
its CLDR definition.

**Q: How to handle context (same English text, different meaning)?**

The grammar domain + type are part of the content address input. Two `~l` sigils
with identical template text but different grammar origins produce different OIDs.
The `msgctxt` field carries `@domain.type_name` for human readability, but the
OID already encodes the distinction.

`msgctxt` is still emitted for compatibility with standard `.po` tools that
group and filter by context.

**Q: How to handle interpolation type safety in `.po` files?**

Each `~l` sigil's variables are declared in the grammar type. The extracted
comment carries the variable names and their grammar types:

```po
#. variables: type:nominative, count:cardinal
```

`mirror craft --i18n --check` validates that every `msgstr` in every language
uses exactly the variables declared in the extracted comment. A German translation
that adds `{gender}` or drops `{type}` is a type error — reported as loss, not
silently accepted.

This is where mirror diverges from every existing i18n system. gettext has
`c-format` flag checking for printf specifiers. Fluent has runtime type
checking. Mirror has compile-time type checking against the grammar.

---

## 5. The Feature Flag

### Cargo.toml

```toml
[features]
default = []
i18n = ["dep:fluent-bundle", "dep:fluent-syntax", "dep:intl_pluralrules", "dep:unic-langid"]

[dependencies]
fluent-bundle = { version = "0.15", optional = true }
fluent-syntax = { version = "0.11", optional = true }
intl_pluralrules = { version = "7.0", optional = true }
unic-langid = { version = "0.9", optional = true }
```

**Why Fluent internals, not Fluent format?**

Mirror does NOT use `.ftl` files. It uses `.po` files — the ecosystem is too
valuable to abandon. But mirror uses Fluent's Rust implementation for:
- `intl_pluralrules` — CLDR plural rule evaluation (pure Rust, no C dependency)
- `unic-langid` — BCP47 language tag parsing and locale negotiation
- `fluent-bundle` — the resolution engine for selector expressions, if
  mirror ever needs Fluent-style selectors inside templates (future)
- `fluent-syntax` — not used initially, reserved for potential `.ftl` import

This gives mirror CLDR-correct plural handling without GNU gettext's C library
dependency, while keeping `.po` as the translator-facing format.

### Conditional Compilation

```rust
// In lib.rs or a dedicated i18n module
#[cfg(feature = "i18n")]
pub mod i18n;

// In parse.rs — sigil extraction
#[cfg(feature = "i18n")]
fn extract_sigil_translations(ast: &Ast) -> Vec<TranslationEntry> { ... }

// In shard.rs — translation table embedding
#[cfg(feature = "i18n")]
pub struct TranslationTable { ... }
```

When `i18n` is disabled:
- `~l` sigils still parse and compile (they are grammar features, not i18n features)
- No extraction, no `.po` generation, no translation table in `.shard`
- The sigil renders its template text directly (source language only)

When `i18n` is enabled:
- `~l` sigils are extracted during `mirror craft --i18n`
- `.po` files are generated and validated
- `.shard` artifacts include an embedded translation table
- Runtime locale selection is available

---

## 6. CLI Surface

### Extraction

```
mirror craft . --i18n
```

Walks all `.mirror` files in the project. For each `~l` sigil:
1. Compute the OID from `grammar_domain:sigil_type:template_text`
2. Record the source location, grammar type, template, and variables
3. Write `locales/mirror.pot` (template) with all entries
4. For each existing `locales/LANG/mirror.po`, run merge:
   - New OIDs (in source, not in `.po`) → added with empty `msgstr`
   - Stale OIDs (in `.po`, not in source) → marked with `#~ ` (obsolete prefix)
   - Unchanged OIDs → preserved with existing translation

### Validation

```
mirror craft . --i18n --check
```

Reports, per language:
- **New strings:** OID in source, not in `.po` → untranslated
- **Stale strings:** OID in `.po`, not in source → orphaned
- **Incomplete strings:** `msgstr` is empty → needs translation
- **Type errors:** `msgstr` uses wrong variables → grammar violation
- **Plural errors:** Wrong `nplurals` for language, missing `msgstr[N]` forms

Exit code:
- 0 if all languages are complete and type-correct
- 1 if any language has untranslated, incomplete, or type-invalid entries

This integrates with the holonomy model. The i18n check is a property fold:

```
property i18n_complete(grammar, locale) <= verdict
```

Each untranslated string contributes 1.0 to holonomy. Each type error
contributes infinity (it's a hard failure, not a loss). The total i18n
holonomy is part of the project's overall holonomy.

### Language-Specific Validation

```
mirror craft . --i18n --check --lang de
```

Validates only the German translations. Useful for CI per-language gates.

### Compilation

```
mirror craft . --i18n --compile
```

Embeds the translation table into the `.shard` artifact. The table is a
content-addressed map:

```
TranslationTable {
    oid_to_template: HashMap<ShortOid, Template>,
    locale_tables: HashMap<Locale, HashMap<ShortOid, Vec<String>>>,
    plural_rules: HashMap<Locale, PluralRule>,
}
```

The `Vec<String>` for each OID holds one string per plural form (index 0..nplurals-1).

### Runtime

```
mirror enact app.shard --lang de
```

Sets the runtime locale. The translation lookup chain:

1. Look up OID in `locale_tables["de"]`
2. If missing, try `locale_tables["de-DE"]` (if BCP47 region was specified, try base)
3. If missing, try `locale_tables["en"]` (source language fallback)
4. If missing, use the raw template from `oid_to_template` (guaranteed present)

Step 4 never fails. Every OID has a template. The fallback chain always terminates.

---

## 7. Directory Structure

```
project/
  spec.mirror
  src/
    release.mirror
    errors.mirror
  locales/
    mirror.pot               extracted template (OID -> English, no translations)
    en/
      mirror.po              English "translations" (OID -> English msgstr)
    de/
      mirror.po              German translations
    ja/
      mirror.po              Japanese translations
    fr/
      mirror.po              French translations
  rust/                      generated (if target = native)
    src/
      i18n.rs                generated translation lookup module
```

### Why `locales/` Not `i18n/`?

Convention. Every gettext-aware tool knows `locales/`. Weblate, Crowdin,
Poedit, Transifex all look for `locales/LANG/DOMAIN.po` by default. Following
convention means zero configuration for translation management tools.

### Why One `.po` Per Language, Not Per Grammar Domain?

Simplicity. A single `mirror.po` per language contains all translations for
the entire project. The `msgctxt` field carries the grammar domain, so
translators can filter by domain within their tool.

If a project grows large enough that a single `.po` file becomes unwieldy,
mirror supports domain-specific extraction:

```
mirror craft . --i18n --domain changelog
```

This produces `locales/LANG/changelog.po` containing only `@changelog` entries.
But this is opt-in — the default is one file.

---

## 8. The @i18n Grammar

```mirror
in @prism
in @meta
in @package

grammar @i18n {
  in @meta
  in @text

  -- core types
  type locale = { language: string, region: string, script: string }
  type bcp47(locale)

  type plural_category = zero | one | two | few | many | other
  type plural_rule = { nplurals: u64, categories: [plural_category] }

  type template = { oid: oid, text: string, variables: [variable] }
  type variable = { name: string, grammar_type: string }

  type po_entry = {
    oid: oid,
    context: string,
    template: template,
    template_plural: template,
    translations: [translation],
    source_ref: string,
  }

  type translation = {
    locale: locale,
    forms: [string],
  }

  type po = {
    grammar: string,
    language: locale,
    plural_rule: plural_rule,
    entries: [po_entry],
  }

  type staleness = new | stale | modified | complete
  type coverage = { locale: locale, total: u64, translated: u64, stale: u64 }

  -- extraction: source -> po template
  io extract(grammar) => po

  -- merge: template + existing translations -> updated po
  io merge(po, po) => po

  -- validation
  property complete(po, locale) <= verdict
  property consistent(po) <= verdict
  property typed(po) <= verdict

  -- coverage measurement
  action coverage(po, locale) -> coverage

  -- rendering
  action render(template, locale, bindings: map) -> string

  recover |result, loss| {
    result
  }

  rescue |error| {
    inspect(error)
  }
}

out locale
out bcp47
out plural_category
out plural_rule
out template
out variable
out po_entry
out translation
out po
out staleness
out coverage
out @i18n
```

### Grammar Placement

`@i18n` lives in `boot/std/i18n.mirror`. It is a standard library grammar,
not a boot grammar. It depends on `@meta` and `@text` (both boot grammars)
and `@package` (boot grammar). It does NOT depend on `@cli` — the CLI
commands are defined in `@cli` using `@i18n` types, not the other way around.

### Integration with Boot Sequence

The `@i18n` grammar is loaded after the core boot sequence. It is available
to any grammar that declares `in @i18n`. The boot grammars themselves are
NOT internationalized — they are structural definitions, not user-facing text.

User-facing text starts at the CLI layer (`@cli`) and the standard library
grammars that produce output (`@changelog`, `@release`). These grammars use
`~l` sigils, which the `@i18n` grammar's `extract` action processes.

---

## 9. Integration with @changelog and @release

### The Flow

```
code change
  -> @changelog.generate(delta)     produces typed changelog entries
  -> ~l sigils in templates         each entry has translatable text
  -> @i18n.extract(grammar)         extracts OIDs from all ~l sigils
  -> translator workflow            .po files updated per language
  -> @i18n.render(template, locale) renders per-language text
  -> @release.publish(release)      ships with all languages
```

### Changelog Example

A changelog grammar might define:

```mirror
grammar @changelog {
  in @i18n
  in @package

  type entry = breaking | feature | fix | chore
  type breaking = { type: string, description: string }

  action generate(delta) -> [entry]
  action render(entry, locale: locale) -> string
}
```

The `render` action uses `~l` sigils internally:

```mirror
-- inside @changelog.render
~l"The {type} was removed in {version}"en
~l"{count} file changed"en  -- singular
~l"{count} files changed"en -- plural
```

Each `~l` sigil produces an OID. The `@i18n.extract` action collects them.
The translation `.po` files map OIDs to per-language text.

At render time:
1. `@changelog.render(entry, locale: "de")` is called
2. The `~l` sigil resolves its OID
3. The OID is looked up in the German translation table
4. The German template is interpolated with the entry's typed fields
5. The result is a German changelog entry, type-checked against the grammar

### Error Messages

Mirror error codes (M1001, M2001, etc.) are also `~l` sigils:

```mirror
-- inside error rendering
~l"unrecognized declaration `{keyword}`"en     -- M1001
~l"`{keyword}` requires a name"en              -- M2001
~l"fold operator `<=` is not valid in type declarations"en  -- M2005
```

This means `mirror explain M2001` can render in any supported language:

```
$ mirror explain M2001 --lang de

Fehler[M2001]: `{keyword}` erfordert einen Namen
...
```

The error catalog is internationalized through the same system as everything
else. No special cases.

---

## 10. Stale Detection

### OID Lifecycle

```
Template text written → OID computed → extracted to .pot → merged to .po
                                                              ↓
Template text changed → new OID computed → old OID orphaned → stale in .po
                                              ↓
                               old .po entry marked #~ (obsolete)
                               new .po entry added with empty msgstr
```

### Detection Algorithm

`mirror craft --i18n --check` compares two sets:

- **Source OIDs:** All OIDs from `~l` sigils in the current source tree
- **PO OIDs:** All OIDs present in each language's `.po` file

```
new_strings    = source_oids - po_oids      # need translation
stale_strings  = po_oids - source_oids      # orphaned
active_strings = source_oids & po_oids      # may be complete or incomplete
```

For `active_strings`, check:
- `msgstr` non-empty → complete
- `msgstr` empty → incomplete (needs translation)
- `#, fuzzy` flag → needs review (template may have changed semantically
  even if OID is the same — this happens when context changes but template
  text doesn't)

### Holonomy of i18n

The i18n holonomy for a given locale is:

```
h_i18n(locale) = |new_strings|
               + |incomplete_strings|
               + |type_errors| * infinity
```

- Each untranslated string contributes 1.0 to holonomy
- Each type error (wrong variables) contributes infinity — it's a hard failure
- Stale strings contribute 0.0 — they're dead, not lost
- The holonomy per locale is independent; total project i18n holonomy is the
  maximum across all configured locales (not the sum — you ship when the
  worst language is complete, not when the average is)

### The Property

```mirror
property i18n_complete(po, locale) <= verdict {
  traversal entries
  refract |entry| {
    entry.translations
      |> filter(|t| t.locale == locale)
      |> first
      |> present?
  }
}
```

This is a fold over all entries. Each entry either has a translation for the
requested locale (passes) or doesn't (contributes to holonomy). The verdict
is `Imperfect`: Success if all entries are translated, Partial if some are
missing, Failure if the locale is not configured at all.

---

## 11. Why .po and Not .ftl

Mozilla's Fluent format (`.ftl`) is technically superior to `.po` in several ways:
- Asymmetric localization (each language controls its own grammar)
- CLDR-native plural categories (not C expressions)
- Selector expressions for gender, case, etc.
- No header metadata format — structured by design

Mirror chooses `.po` anyway, for these reasons:

1. **Ecosystem.** Every TMS, every translation agency, every translator tool
   knows `.po`. Fluent has excellent tooling but limited adoption outside
   Mozilla. Mirror is not Mozilla-scale. The ecosystem matters more than
   format elegance.

2. **Simplicity.** Mirror's `~l` sigils are simpler than Fluent messages.
   A `~l` sigil is a template with typed interpolation variables. It doesn't
   need Fluent's selector syntax because the grammar type system handles
   the selection logic. The `.po` format is sufficient for what mirror needs.

3. **Content addressing.** The `.po` format's `msgid` field maps directly
   to OID-based lookup. Fluent uses human-readable message IDs (`emails =`),
   which would need to be mapped to OIDs. With `.po`, the mapping is native.

4. **CLDR plural rules anyway.** Mirror uses `intl_pluralrules` from the
   Fluent ecosystem for CLDR-correct plural evaluation, but writes the
   result as a gettext `Plural-Forms` header. The runtime uses CLDR; the
   file format uses gettext. Best of both.

5. **Compatibility with `@i18n.merge`.** The `.po` merge algorithm is
   well-specified (GNU msgmerge). Mirror implements its own merge (because
   OID-based keys have different merge semantics), but the file format is
   compatible with standard tools for editing and review.

Where mirror goes beyond `.po`:
- `msgid` is an OID, not the source text
- Extracted comments carry grammar types and variable declarations
- Type checking validates interpolation variables at compile time
- Holonomy measures translation completeness as a continuous value
- The `.po` file is itself content-addressed (the file's OID tracks changes)

---

## 12. Open Questions

1. **Should mirror support `.ftl` import?** A `mirror craft --i18n --import-ftl`
   command could convert Fluent files to mirror's `.po` format. This would help
   projects migrating from Fluent. Low priority — the formats are different enough
   that lossless conversion is non-trivial.

2. **Should the translation table in `.shard` be lazy-loaded?** For applications
   with many languages and many strings, embedding all translations in the binary
   could be large. Alternative: the `.shard` embeds only the source language and
   loads additional languages from a `locales/` directory at runtime. This is the
   gettext model (`.mo` files loaded at runtime).

3. **Should `@i18n` support ICU MessageFormat?** ICU MessageFormat is more
   expressive than gettext interpolation (nested plurals, select, etc.). Mirror's
   type system could validate ICU patterns at compile time. But this adds
   complexity. Decision: not in v1. The grammar type system handles selection
   logic; the template handles text interpolation. Keep them separate.

4. **How do `~l` sigils interact with `@shatter`?** When mirror shatters a
   grammar for AI training, should translations be included in the training data?
   Probably yes — the AI should learn that a template has translations and how
   they vary. But the translations are `protected` visibility by default (they
   may contain unreleased product text).

5. **Should `.po` files be committed to the same repo or a separate one?**
   Translation velocity is different from code velocity. Translators may submit
   changes weekly while code ships daily. A submodule or separate worktree for
   `locales/` would allow independent versioning. Decision: same repo by default,
   submodule opt-in for large projects.

6. **How does `mirror craft --i18n` interact with `mirror ci`?** Proposal: `mirror ci`
   includes i18n completeness in its holonomy calculation when the `i18n` feature
   is enabled. Incomplete translations produce Partial, not Failure — the code
   ships, but the holonomy is non-zero. `mirror ci --strict-i18n` would promote
   incomplete translations to Failure (exit 1).

7. **Should BEAM runtime use Erlang's built-in locale support or mirror's?**
   The BEAM has `erl_nif` locale functions and OTP's `calendar` module uses locale.
   Mirror's BEAM target should use the `TranslationTable` from the `.shard` for
   string lookup, but delegate formatting (numbers, dates, currencies) to `ex_cldr`
   when available. The `@i18n` grammar's `render` action is the boundary — it
   handles string lookup, `ex_cldr` handles formatting.

---

## 13. Prior Art Summary

| System | Keys | Extraction | Plurals | Type Safety | Format |
|--------|------|-----------|---------|-------------|--------|
| GNU gettext | Source text | xgettext | C formula | printf flags | .po/.mo |
| Project Fluent | Human IDs | Manual | CLDR | Runtime | .ftl |
| Elixir gettext | Source text | mix gettext.extract | CLDR via ex_cldr | Compile-time interpolation | .po |
| ICU MessageFormat | Human IDs | Manual | CLDR | Runtime | .properties |
| rust-i18n | Human IDs | cargo i18n | None | Compile-time | YAML/JSON |
| **Mirror** | **OID (content-addressed)** | **mirror craft --i18n** | **CLDR via intl_pluralrules** | **Compile-time grammar types** | **.po** |

Mirror's approach is unique in three ways:
1. Content-addressed keys (OIDs) instead of source text or human-written IDs
2. Grammar-typed interpolation variables checked at compile time
3. Translation completeness measured as holonomy, not as a boolean

---

## Sources

### GNU gettext / .po Format
- [PO File Entries (GNU gettext utilities)](https://www.gnu.org/software/gettext/manual/html_node/PO-File-Entries.html)
- [PO Files (GNU gettext utilities)](https://www.gnu.org/software/gettext/manual/html_node/PO-Files.html)
- [Translating plural forms (GNU gettext utilities)](https://www.gnu.org/software/gettext/manual/html_node/Translating-plural-forms.html)
- [Plural forms (GNU gettext utilities)](https://www.gnu.org/software/gettext/manual/html_node/Plural-forms.html)
- [msgfmt Invocation (GNU gettext utilities)](https://www.gnu.org/software/gettext/manual/html_node/msgfmt-Invocation.html)
- [GNU gettext utilities](https://www.gnu.org/software/gettext/manual/gettext.html)

### Rust i18n
- [fluent-rs (GitHub)](https://github.com/projectfluent/fluent-rs)
- [fluent-bundle docs.rs](https://docs.rs/fluent-bundle/latest/fluent_bundle/)
- [fluent-fallback docs.rs](https://docs.rs/fluent-fallback)
- [i18n-embed (crates.io)](https://crates.io/crates/i18n-embed)
- [i18n-embed docs.rs](https://docs.rs/i18n-embed/latest/i18n_embed/)
- [Internationalization crates (lib.rs)](https://lib.rs/internationalization)
- [cargo-i18n (GitHub)](https://github.com/kellpossible/cargo-i18n)

### Elixir/BEAM i18n
- [Gettext (Elixir) — hexdocs](https://hexdocs.pm/gettext/Gettext.html)
- [elixir-gettext (GitHub)](https://github.com/elixir-gettext/gettext)
- [ex_cldr (Hex)](https://hex.pm/packages/ex_cldr)
- [elixir-cldr (GitHub)](https://github.com/elixir-cldr/cldr)
- [I18n with Phoenix LiveView](https://dev.to/wintermeyer/i18n-with-phoenix-liveview-28mj)
- [Translating Phoenix Applications with GNU gettext (Phrase)](https://phrase.com/blog/posts/i18n-for-phoenix-applications-with-gettext/)

### Project Fluent
- [Project Fluent — Selectors](https://projectfluent.org/fluent/guide/selectors.html)
- [Project Fluent — Functions](https://projectfluent.org/fluent/guide/functions.html)
- [Fluent and Standards (wiki)](https://github.com/projectfluent/fluent/wiki/Fluent-and-Standards)
- [intl_pluralrules — CLDR plural rules in Rust (Mozilla)](https://blog.mozilla.org/l10n/2018/08/03/intl_pluralrules-a-rust-crate-for-handling-plural-forms-with-cldr-plural-rules/)
