# Boot Sequence Reorganization Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Split boot into kernel (sorted, numbered) and standard library (unsorted, package-resolved). The kernel defines the language. The standard library uses it.

**Architecture:** The kernel is `boot/` — 13 numbered files defining the language from optics through package resolution. The standard library is `boot/std/` — unsorted `.mirror` files that declare `in @X` dependencies and are resolved by `@package`. The std is the first consumer of the package system it depends on.

**Tech Stack:** Mirror grammar files (.mirror), Rust parser/tests

---

## The Split

### Kernel: `boot/` (sorted, numbered)

```
boot/
  00-prism.mirror           the optics
  01-meta.mirror            pure types, operators, imperfect
  01a-meta-action.mirror    action boundary, collapse
  01b-meta-io.mirror        io, mut, effect — the real
  02-shatter.mirror         crystallization (first io)
  03-code.mirror            code observation
  03a-code-rust.mirror      rust grammar
  04-actor.mirror           actors
  05-property.mirror        properties
  06-package.mirror         package types, version
  06a-package-git.mirror    git integration
  06b-package-spec.mirror   spec, deployment
```

13 files. Sorted by number. Each depends only on files before it. The kernel bootstraps the language — after file 06b, the language is complete enough to resolve packages.

### Standard Library: `boot/std/` (unsorted, package-resolved)

```
boot/std/
  mirror.mirror             the compiler describes itself
  time.mirror               timeline, snapshots, replay
  tui.mirror                terminal
  benchmark.mirror          measurement
  cli.mirror                commands as optic compositions
```

Unsorted. No numbers. Each file declares `in @X` dependencies. The package resolver (defined in `06-package.mirror`) determines the load order. The standard library IS the first test of the package system.

### Why This Split

The kernel answers: "what is the language?"
The std answers: "what does the language ship with?"

The kernel can't use packages — it defines them. The std must use packages — it proves they work. The boundary is `06-package.mirror`. Everything before it is bootstrap. Everything after it is a consumer.

---

## Current → New Mapping

| Current | New | Change |
|---------|-----|--------|
| `00-prism.mirror` | `boot/00-prism.mirror` | unchanged |
| `01-meta.mirror` | `boot/01-meta.mirror` | strip impure types, add pure/real/loss |
| — | `boot/01a-meta-action.mirror` | NEW: from 04-action + action types |
| — | `boot/01b-meta-io.mirror` | NEW: io, mut, effect types |
| — | `boot/02-shatter.mirror` | NEW: materialization |
| `02-code.mirror` | `boot/03-code.mirror` | renumber |
| `02a-code-rust.mirror` | `boot/03a-code-rust.mirror` | renumber |
| `03-actor.mirror` | `boot/04-actor.mirror` | renumber |
| `04-action.mirror` | REMOVED | folded into 01a |
| `05-property.mirror` | `boot/05-property.mirror` | fix `in @form` → `in @meta` |
| `08-git.mirror` | `boot/06a-package-git.mirror` | folded into package |
| `09-package.mirror` | `boot/06-package.mirror` | renumber, merge types |
| `11-spec.mirror` | `boot/06b-package-spec.mirror` | folded into package |
| `10-mirror.mirror` | `boot/std/mirror.mirror` | → std, fix refs |
| `15-time.mirror` | `boot/std/time.mirror` | → std |
| `16-tui.mirror` | `boot/std/tui.mirror` | → std |
| `17-benchmark.mirror` | `boot/std/benchmark.mirror` | → std |
| `20-cli.mirror` | `boot/std/cli.mirror` | → std, add kintsugi + flags |

---

## Tasks

### Task 1: Write baseline inventory test

**Files:**
- Modify: `src/mirror_runtime.rs`

- [ ] **Step 1: Write the test**

```rust
/// Captures the current boot inventory before reorganization.
/// This is training data — we measure before we change.
#[test]
fn boot_file_inventory_before_reorg() {
    let boot = boot_dir();
    let mut files: Vec<String> = std::fs::read_dir(&boot)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|f| f.ends_with(".mirror"))
        .collect();
    files.sort();

    assert_eq!(files.len(), 18, "baseline boot file count: {:?}", files);
    assert!(files.contains(&"00-prism.mirror".to_string()));
    assert!(files.contains(&"20-cli.mirror".to_string()));
    // No std/ directory yet
    assert!(!boot.join("std").exists(), "std/ should not exist yet");
}
```

- [ ] **Step 2: Run, verify green**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test --lib boot_file_inventory_before`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/mirror_runtime.rs
git commit -m "🔴 boot reorg: baseline inventory test"
```

---

### Task 2: Create kernel files (01a, 01b, 02-shatter)

**Files:**
- Modify: `boot/01-meta.mirror`
- Create: `boot/01a-meta-action.mirror`
- Create: `boot/01b-meta-io.mirror`
- Create: `boot/02-shatter.mirror`

- [ ] **Step 1: Rewrite 01-meta.mirror (pure only)**

```mirror
in @prism

focus type(id)
project ref(type)

split |(ref, ref)
fold <=(ref, imperfect)
unfold =>(ref, ref)
subset <(ref, ref)
superset >(ref, ref)
iso =(ref, ref)
not-iso !=(ref, ref)
zoom |>(ref, prism)
zoom <|(prism, ref)
zoom /(ref, focus)
zoom ->(ref, ref)
zoom <-(ref, ref)
refract ..(ref)

type pure = iso
type real != pure

type observation = pure
type template(grammar, block) = pure
type error(observation)
type loss = pure - real

type precision(f64)
type grammar
type block(grammar)
type imperfect(observation, error(observation), loss) {
  recover |observation, loss| <= imperfect
  rescue |error(observation), loss| <= imperfect
}
type abstract(grammar)
type abstract(action)

type beam(result) {
  result: result,
  path: [prism],
  loss: loss
  precision: precision
}

grammar @meta {
  focus @
  project |>
  split |
  fold <=
  unfold =>
  subset <
  superset >
  iso =
  not-iso !=
  zoom /
  refract ..
}

out type
out ref
out |
out <=
out =>
out <
out >
out =
out !=
out |>
out /
out ..
out pure
out real
out observation
out template
out error
out loss
out precision
out beam
out grammar
out block
out imperfect
out abstract
out @meta
```

- [ ] **Step 2: Create 01a-meta-action.mirror**

```mirror
in @prism
in @meta
in @actor

type action(grammar, effect) != iso
type collapse(block) != iso

prism action {
  focus  type -> grammar
  project grammar -> block
  split  block(grammar) -> imperfect
  zoom   imperfect -> transport
  refract transport -> crystal
}

action action(receiver: type, in: grammar, body: block(grammar)) {
  focus  receiver
  project in
  split  body(in)
  zoom   body -> imperfect
  refract imperfect
}

out action
out collapse
```

- [ ] **Step 3: Create 01b-meta-io.mirror**

```mirror
in @prism
in @meta
in @actor

type mut(block) != iso
type effect = mut(block)
type io(effect) != observation

type path(text)
type content(block)
type channel(actor, actor)

grammar @io {
  io read(path) => imperfect
  io write(path, content) => imperfect
  io send(channel, message) => imperfect
  io spawn(actor) => process
  io stop(process) => imperfect
}

out io
out effect
out mut
out path
out content
out channel
out @io
```

- [ ] **Step 4: Create 02-shatter.mirror**

```mirror
in @prism
in @meta
in @io

grammar @shatter {
  io materialize(grammar, loss) => shatter
  io crystallize(boot) => crystal
  io learn(path, grammar) => grammar
}

out @shatter
```

- [ ] **Step 5: Run tests**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test`
Expected: Pass (new files are additive, parser reads alphabetically)

- [ ] **Step 6: Commit**

```bash
git add boot/01-meta.mirror boot/01a-meta-action.mirror boot/01b-meta-io.mirror boot/02-shatter.mirror
git commit -m "🟢 kernel: 01-meta(pure), 01a-action, 01b-io, 02-shatter"
```

---

### Task 3: Renumber code, actor; remove action

**Files:**
- Rename: `boot/02-code.mirror` → `boot/03-code.mirror`
- Rename: `boot/02a-code-rust.mirror` → `boot/03a-code-rust.mirror`
- Rename: `boot/03-actor.mirror` → `boot/04-actor.mirror`
- Delete: `boot/04-action.mirror`

- [ ] **Step 1: Rename and remove**

```bash
cd /Users/alexwolf/dev/projects/mirror/boot
git mv 02-code.mirror 03-code.mirror
git mv 02a-code-rust.mirror 03a-code-rust.mirror
git mv 03-actor.mirror 04-actor.mirror
git rm 04-action.mirror
```

- [ ] **Step 2: Run tests, update baselines**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test`

Update `mirror_ci_boot_baseline` and `boot_file_inventory_before_reorg` assertions
to match new file counts and names.

- [ ] **Step 3: Commit**

```bash
git add -A boot/ src/mirror_runtime.rs
git commit -m "♻️ boot: renumber code(03), actor(04), remove action(04→01a)"
```

---

### Task 4: Fix 05-property.mirror refs

**Files:**
- Modify: `boot/05-property.mirror`
- Modify: `boot/05-property.shatter`

- [ ] **Step 1: Update in @form → in @meta**

```mirror
grammar @property {
  in @meta

  type verdict = pass | partial | fail
  ...
```

- [ ] **Step 2: Run tests**

This may cause `05-property` to RESOLVE where it previously failed (since `@meta`
exists in the registry but `@form` didn't). Update baseline accordingly.

- [ ] **Step 3: Commit**

```bash
git add boot/05-property.mirror boot/05-property.shatter
git commit -m "🟢 05-property: in @form → in @meta (resolves now)"
```

---

### Task 5: Merge package files

**Files:**
- Create: `boot/06-package.mirror`
- Create: `boot/06a-package-git.mirror`
- Rename: `boot/11-spec.mirror` → `boot/06b-package-spec.mirror`
- Delete: `boot/08-git.mirror`
- Delete: `boot/09-package.mirror`

- [ ] **Step 1: Create 06-package.mirror**

```mirror
in @prism
in @meta

type version
type semver(version) = major(u32) | minor(u32) | patch(u32)
type surface = observation | action

type mirver(grammar, @mirror(surface)) = {
  oid: oid,
  semver: semver,
}

default(version) = mirver

type change {
  added: [type],
  removed: [type],
  modified: [type],
}

type package {
  name: string,
  repo: repo,
  version: mirver,
  license: license,
  dependencies: [package],
}

abstract grammar @package {
  abstract action resolve(package) -> imperfect
  abstract action install(package) -> imperfect
  abstract action publish(package) -> imperfect
  abstract action diff(mirver, mirver) -> change
  abstract action compatible(mirver, mirver) -> imperfect
}

out version
out semver
out surface
out mirver
out change
out package
out @package
```

- [ ] **Step 2: Create 06a-package-git.mirror**

```mirror
in @prism
in @meta
in @package

type repo(url)
type commit(oid)
type branch(string)
type tag(string, commit)

abstract grammar @git {
  abstract action clone(repo) -> imperfect
  abstract action fetch(repo) -> imperfect
  abstract action checkout(commit) -> imperfect
  abstract action tag(commit, string) -> imperfect
  abstract action branches(repo) -> [branch]
  abstract action merge(branch) -> imperfect
}

out repo
out commit
out branch
out tag
out @git
```

- [ ] **Step 3: Rename spec, delete old files**

```bash
cd /Users/alexwolf/dev/projects/mirror/boot
git mv 11-spec.mirror 06b-package-spec.mirror
git rm 08-git.mirror
git rm 09-package.mirror
```

- [ ] **Step 4: Run tests, update baselines, commit**

```bash
git add -A boot/ src/mirror_runtime.rs
git commit -m "♻️ boot: merge git+package+spec into 06-package"
```

---

### Task 6: Create boot/std/ directory

**Files:**
- Create: `boot/std/` directory
- Move: `boot/10-mirror.mirror` → `boot/std/mirror.mirror`
- Move: `boot/15-time.mirror` → `boot/std/time.mirror`
- Move: `boot/16-tui.mirror` → `boot/std/tui.mirror`
- Move: `boot/17-benchmark.mirror` → `boot/std/benchmark.mirror`
- Move: `boot/20-cli.mirror` → `boot/std/cli.mirror`

- [ ] **Step 1: Create std/ and move files**

```bash
mkdir -p /Users/alexwolf/dev/projects/mirror/boot/std
cd /Users/alexwolf/dev/projects/mirror/boot
git mv 10-mirror.mirror std/mirror.mirror
git mv 15-time.mirror std/time.mirror
git mv 16-tui.mirror std/tui.mirror
git mv 17-benchmark.mirror std/benchmark.mirror
git mv 20-cli.mirror std/cli.mirror
```

- [ ] **Step 2: Fix std/mirror.mirror**

Update `form @mirror` → `grammar @mirror`, `in @form` → `in @meta`,
remove undefined refs (`@type`, `@boundary`, `@lens`):

```mirror
grammar @mirror {
  in @meta
  in @prism
  in @property

  requires unique_variants
  requires every_type_reachable
  requires no_dead_variants
  requires types_lowercase
  requires action_is_named_type
  invariant dual_partition
  invariant idempotent
  invariant deterministic
  invariant pure
  invariant no_cycles
  ensures always_halts
}
```

- [ ] **Step 3: Update std/cli.mirror**

Add kintsugi command and flags as optics:

```mirror
in @prism
in @meta
in @code
in @spec
in @shatter

type exit_code = success | failure | partial(loss)

type command {
  name: string,
  args: [string],
  result: imperfect,
}

grammar @cli {

  -- flags as optics
  flag strict = prism(imperfect => success | failure)
  flag format(json | human) = lens(imperfect => text)
  flag check = prism(imperfect => pass | fail)
  flag verbose = lens(loss => text)

  -- commands as optic compositions
  command compile = parse . resolve . emit
  command kintsugi = parse . resolve . canonical_order
  command ci = parse . resolve . properties . emit
  command focus = parse
  command explain(code) = catalog . render

  -- five optics as commands
  action focus(path: string) -> imperfect
  action project(path: string) -> imperfect
  action split(path: string) -> imperfect
  action zoom(path: string) -> imperfect
  action refract(path: string) -> imperfect

  -- formatter
  action kintsugi(path: string) -> imperfect

  -- compiler
  action compile(path: string) -> imperfect
  action crystal(output: string) -> imperfect

  -- ci/ca
  action ci(path: string) -> imperfect
  action ca(path: string) -> imperfect

  -- session
  action init(path: string) -> imperfect
  action tick() -> imperfect
  action tock() -> imperfect
  action shatter(path: string) -> imperfect

  -- deployment
  action plan() -> imperfect
  action apply() -> imperfect
  action drift() -> imperfect
  action rollback(oid: string) -> imperfect

  -- navigation
  action diff(a: string, b: string) -> imperfect
  action log() -> imperfect
  action blame(path: string) -> imperfect

  -- lsp
  action lsp() -> imperfect
  action repl() -> imperfect

  -- garden
  action add(package: string) -> imperfect
  action remove(package: string) -> imperfect
  action list() -> imperfect

  -- ai
  action ai(path: string) -> imperfect
  action train(path: string) -> imperfect

  -- benchmark
  action bench(path: string) -> imperfect
  action profile() -> imperfect

  recover |result, loss| {
    print(loss)
    result
  }

  rescue |error| {
    print(error)
    exit(1)
  }
}

out exit_code
out command
out @cli
```

- [ ] **Step 4: Commit**

```bash
git add -A boot/
git commit -m "♻️ boot: create std/, move mirror+time+tui+bench+cli"
```

---

### Task 7: Update compiler to load std/

**Files:**
- Modify: `src/mirror_runtime.rs`

The compiler currently loads `boot/*.mirror` sorted alphabetically.
Now it needs two phases:

1. Load `boot/*.mirror` sorted (kernel)
2. Load `boot/std/*.mirror` unsorted (package resolution)

- [ ] **Step 1: Write failing test**

```rust
/// The compiler must load std/ files after kernel files.
/// std/ files are NOT sorted — they're resolved by @package.
#[test]
fn boot_loads_std_after_kernel() {
    let runtime = MirrorRuntime::new();
    let store = tempdir_for_test("boot_std");
    let boot = runtime.compile_boot_dir(&boot_dir(), &store).unwrap();

    // Kernel files resolve (sorted, ordered)
    assert!(boot.resolved.contains_key("00-prism"));
    assert!(boot.resolved.contains_key("01-meta"));

    // std/ files are loaded and attempted
    // mirror.mirror should resolve (in @meta, in @prism, in @property all in kernel)
    assert!(
        boot.resolved.contains_key("std/mirror") || boot.resolved.contains_key("mirror"),
        "std/mirror.mirror should be loaded"
    );
}
```

- [ ] **Step 2: Update compile_boot_dir**

Modify `compile_boot_dir` to:
1. Read `boot/*.mirror` (sorted) — the kernel
2. Read `boot/std/*.mirror` (unsorted) — the standard library
3. Compile kernel first, register all refs
4. Then compile std files, resolving against the kernel's registry

```rust
// After the existing kernel compilation loop:
let std_dir = dir.join("std");
if std_dir.is_dir() {
    let mut std_entries: Vec<_> = std::fs::read_dir(&std_dir)
        .map_err(|e| err(format!("read_dir {}: {}", std_dir.display(), e)))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("mirror"))
        .collect();
    // NOT sorted — package resolution determines order
    // For now, sort alphabetically as placeholder until @package.resolve works
    std_entries.sort();

    for path in std_entries {
        // Same compilation loop as kernel, but files resolve against
        // the full kernel registry
        let stem = format!("std/{}", path.file_stem().unwrap().to_str().unwrap());
        // ... same compile + resolve logic
    }
}
```

- [ ] **Step 3: Run tests**

Run: `cd /Users/alexwolf/dev/projects/mirror && nix develop -c cargo test`

- [ ] **Step 4: Commit**

```bash
git add src/mirror_runtime.rs
git commit -m "🟢 compile_boot_dir: load std/ after kernel"
```

---

### Task 8: Write final inventory test

**Files:**
- Modify: `src/mirror_runtime.rs`

- [ ] **Step 1: Write the goal test**

```rust
/// The reorganized boot: kernel + std.
#[test]
fn boot_kernel_and_std() {
    let boot = boot_dir();

    // Kernel: sorted, numbered
    let mut kernel: Vec<String> = std::fs::read_dir(&boot)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|f| f.ends_with(".mirror"))
        .collect();
    kernel.sort();

    // std: unsorted, package-resolved
    let std_dir = boot.join("std");
    let mut std_files: Vec<String> = if std_dir.exists() {
        std::fs::read_dir(&std_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|f| f.ends_with(".mirror"))
            .collect()
    } else {
        Vec::new()
    };
    std_files.sort(); // sort for assertion stability only

    // Kernel: 13 files (00 through 06b)
    assert!(kernel.len() >= 10, "kernel needs at least 10 files: {:?}", kernel);

    // Std: 5 files (mirror, time, tui, benchmark, cli)
    assert!(std_files.len() >= 5, "std needs at least 5 files: {:?}", std_files);
    assert!(std_files.contains(&"mirror.mirror".to_string()), "std must contain mirror.mirror");
    assert!(std_files.contains(&"cli.mirror".to_string()), "std must contain cli.mirror");
}
```

- [ ] **Step 2: Update all baseline tests**

Update `mirror_ci_boot_baseline` with new resolved/failed counts.
The 05-property fix (`in @form` → `in @meta`) should cause it to resolve now.
The std/mirror.mirror fix should cause it to resolve now.

- [ ] **Step 3: Run full suite, commit**

```bash
git add -A
git commit -m "🟢 boot reorganization complete: kernel(13) + std(5), purity gradient"
```

---

## Final Structure

```
boot/                          KERNEL (sorted, numbered)
  00-prism.mirror              the optics
  01-meta.mirror               pure types, operators, imperfect
  01a-meta-action.mirror       action boundary, collapse
  01b-meta-io.mirror           io, mut, effect — the real
  02-shatter.mirror            crystallization (first io)
  03-code.mirror               code observation
  03a-code-rust.mirror         rust grammar
  04-actor.mirror              actors
  05-property.mirror           properties
  06-package.mirror            package types, version
  06a-package-git.mirror       git integration
  06b-package-spec.mirror      spec, deployment

boot/std/                      STANDARD LIBRARY (unsorted, package-resolved)
  mirror.mirror                the compiler describes itself
  time.mirror                  timeline, snapshots, replay
  tui.mirror                   terminal
  benchmark.mirror             measurement
  cli.mirror                   commands as optic compositions
```

The kernel defines the language. The std uses the language.
The boundary is `06-package.mirror`.
The std is the first consumer of `@package.resolve()`.
The boot sequence IS the type theory table of contents.
