# mirror ca --merge — Implementation Plan

## What exists

- `mirror ci <path>` — measures holonomy (implemented, tested)
- `mirror ca <path>` — reports suggestions (implemented, tested)
- `Cli::dispatch` — routes commands
- MirrorLoss with `holonomy()` method
- `compile_boot_dir` — compiles all boot files
- `mirror crystal` — materializes the shatter file

## What doesn't exist

- `mirror ca --merge` — the command
- Branch graph analysis
- Holonomy measurement per branch
- Dependency ordering of branches
- Automated merge execution
- rerere shard recording

## The plan

### Step 1: Branch analysis

Add to `cli.rs`:

```rust
fn cmd_ca_merge(&self) -> Imperfect<String, CliError, MirrorLoss> {
    let branches = list_branches()?;
    let main_oid = current_head()?;

    for branch in &branches {
        let ahead = commits_ahead(&branch, "main")?;
        let conflicts = check_conflicts(&branch, "main")?;
        println!("{}: {} ahead, {} conflicts", branch, ahead, conflicts);
    }

    // Sort by: conflicts ascending, then ahead ascending
    // Low conflict + low divergence = merge first

    // Execute merges in order
    // After each: cargo test
    // If fail: abort that merge, skip branch, continue
}
```

This is git operations from Rust. Use `git2` (already a dependency)
or shell out to `git`.

### Step 2: The merge loop

```rust
for branch in sorted_branches {
    match merge_branch(&branch) {
        Ok(()) => {
            if cargo_test_passes() {
                println!("  merged: {}", branch);
                merged.push(branch);
            } else {
                abort_merge();
                println!("  skipped (tests fail): {}", branch);
                skipped.push(branch);
            }
        }
        Err(conflicts) => {
            abort_merge();
            println!("  skipped ({} conflicts): {}", conflicts, branch);
            skipped.push(branch);
        }
    }
}
```

### Step 3: Crystal after merge

```rust
if !merged.is_empty() {
    let crystal = self.cmd_crystal(&["mirror.shatter".into()])?;
    println!("crystal: {}", crystal);
}
```

### Step 4: Wire into dispatch

```rust
"ca" => {
    if args.iter().any(|a| a == "--merge") {
        self.cmd_ca_merge()
    } else {
        self.cmd_ca(args)
    }
}
```

### Step 5: Tests

```rust
#[test]
fn ca_merge_on_clean_main_is_noop() {
    // no branches to merge → "nothing to merge"
}

#[test]
fn ca_merge_reports_branch_count() {
    // create a test branch, verify it shows up
}
```

## What this does NOT include

- rerere shard recording (future)
- Octopus merge (sequential is fine for now)
- Holonomy measurement per branch (uses conflict count as proxy)
- Automated promotion to staging/production (future)

## Build/test

```bash
cd /Users/alexwolf/dev/projects/mirror
nix develop -c cargo test
```

## After implementation

1. Amend the dishonest commit messages
2. Reset main to pre-merge
3. Run `mirror ca --merge` for real
4. The test goes green from the tool, not from Reed
