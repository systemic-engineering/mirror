# First CA Task: The Compiler Merges Itself

The first `mirror ca` task is self-maintenance. 33 branches from a
42-hour marathon session. The compiler's first job is cleaning up
after itself.

---

## The State

33 unmerged branches. Each contains real work. Some depend on each
other. Some overlap. Some conflict. The work happened concurrently
across multiple agents — Mara, Glint, Taut, Seam — sometimes
touching the same files.

18 stashes. Accumulated during conflict resolution.

Main is behind. The real work is scattered across branches.

## The Process

### Step 1: Observe (mirror ci)

Measure the holonomy of each branch relative to main:

```bash
for branch in $(git branch --list | grep -v main); do
    echo "$branch: $(mirror ci --branch $branch --holonomy)"
done
```

Output:
```
branch                          holonomy   ahead   conflicts
mara/mirror-loss-bundle-update  0.12       8       0
mara/action-prism               0.08       12      0
mara/materialize-crystal        0.15       6       1
mara/optic-op-kernel            0.03       3       0
mara/ci-ca-commands             0.21       55      3
mara/entropy-derivation         0.09       4       0
glint/parse-store               0.06       5       0
glint/mirror-optic              0.11       7       1
glint/tick-1-mirror-delivers    0.14       50      2
glint/tick-2-spectral-consumes  0.04       3       0
seam-mara/codebase-review       0.07       10      0
taut/benchmark                  0.05       4       0
...
```

Low holonomy = close to main = safe to merge first.
High holonomy = diverged = merge later, more carefully.

### Step 2: Sort (split)

Group branches by dependency:

**Tier 1 — no dependencies, low holonomy (merge first):**
```
mara/optic-op-kernel            0.03   OpticOp enum
taut/benchmark                  0.05   criterion benchmarks
glint/parse-store               0.06   Store trait + parse re-export
seam-mara/codebase-review       0.07   test honesty fixes
```

**Tier 2 — depends on Tier 1:**
```
mara/action-prism               0.08   action parsing (needs OpticOp)
mara/mirror-loss-bundle-update  0.12   MirrorLoss (needs parse)
mara/entropy-derivation         0.09   Fate derivation (independent)
```

**Tier 3 — depends on Tier 2:**
```
glint/mirror-optic              0.11   MirrorOptic (needs action + loss)
mara/materialize-crystal        0.15   mirror.shatter (needs MirrorOptic)
```

**Tier 4 — depends on everything:**
```
glint/tick-1-mirror-delivers    0.14   Cli struct (needs all above)
mara/ci-ca-commands             0.21   ci/ca commands (needs Cli)
```

**Tier 5 — spectral (separate repo):**
```
glint/tick-2-spectral-consumes  0.04   spectral wiring
```

### Step 3: Merge (zoom)

For each tier, in order:

```bash
# Tier 1
git checkout main
git merge mara/optic-op-kernel --no-edit
mirror ci .  # measure holonomy after merge
# if green → continue
# if amber → investigate, fix, then continue
# if red → stop, the merge introduced a problem

git merge taut/benchmark --no-edit
mirror ci .

git merge glint/parse-store --no-edit
mirror ci .

git merge seam-mara/codebase-review --no-edit
mirror ci .

# Tier 2
git merge mara/action-prism --no-edit
mirror ci .
# ... etc
```

After EVERY merge: `mirror ci .` measures holonomy. The holonomy
tells you if the merge introduced problems. Green: proceed. Amber:
investigate. Red: stop and fix.

The merge process IS the CI process. The tool testing itself.

### Step 4: Resolve conflicts (recover/rescue)

When a merge has conflicts:

```
recover |value, loss| {
    -- the merge is Partial
    -- value: the merged code with conflict markers
    -- loss: which files conflict, how many hunks
    -- the engineer resolves
    value
}

rescue |error| {
    -- the merge failed entirely
    -- revert, try a different order
    git merge --abort
}
```

Conflict resolution produces MirrorLoss. The loss records which
files conflicted, which hunks, how they were resolved. The history
is in the shard.

### Step 5: Crystal (refract)

After all tiers merged:

```bash
mirror crystal mirror.shatter
mirror ci .
```

Re-crystallize. Measure holonomy of the clean main. It should be
lower than any individual branch — the divergence has been resolved.

The final holonomy of main IS the cost of the marathon. The number
that tells you what concurrent development cost in structural terms.

### Step 6: Clean up

```bash
# Delete merged branches
for branch in $(git branch --merged main | grep -v main); do
    git branch -d $branch
done

# Clear stashes
git stash clear

# Final measurement
mirror ci .
```

## The Meta-Property

After the merge:

```bash
mirror crystal --oid
```

The OID is the content address of the clean main. The crystal carries
the work of five agents across forty-two hours, merged in dependency
order, verified at every step.

The binary loads this crystal. The binary IS this crystal. The
meta-property test passes: the binary is its own spec.

## Why This Is the Right First Task

1. **Self-maintenance is the hardest test.** If the compiler can
   clean up its own mess, it can clean up anyone's.

2. **The branches ARE the training data.** The holonomy of each
   branch, the conflict patterns, the merge order — this is the
   loss history that Fate derives from. The first real eigenvalue
   derivation happens on the compiler's own merge history.

3. **The demo writes itself.** "The compiler's first task was
   merging its own 33 branches from a 42-hour development marathon.
   Here's the holonomy graph." That's the conference talk.

4. **It proves the architecture.** CI measures. CA acts. The merge
   order is the suggestion. The conflict resolution is the recovery.
   The crystal is the settlement. Every concept in the architecture
   gets tested on real data — the compiler's own development history.

---

## The Heartbeat

After the merge is clean, the heartbeat starts:

```bash
mirror ca --watch .
```

The compiler watches itself. Every file save triggers a tick. The
holonomy is measured. If it rises above threshold, CA suggests. If
the engineer approves, CA enforces. The gutter breathes on the
compiler's own source.

The first heartbeat cycle:
```
tick → mirror ci . → holonomy 0.000 → crystal → sleep
... (engineer edits a file)
tick → mirror ci . → holonomy 0.014 → amber → suggest
... (engineer approves)
tick → mirror ca --enforce → holonomy 0.003 → settling
tick → mirror ci . → holonomy 0.000 → crystal → sleep
```

The compiler maintains itself. On a heartbeat. The gutter breathes.
The eigenvalues settle. The crystal holds.

---

*The first CA task is self-maintenance. The first customer is the
compiler. The first proof is the merge. The first heartbeat is
the compiler watching itself and knowing what it costs.*

*33 branches → 1 crystal. That's the demo.*
