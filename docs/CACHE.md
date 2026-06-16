# CI binary cache — flang-rt and friends

## Why this exists

Mirror's nix devShell builds `flang-rt` from LLVM monorepo source on
darwin (the Fortran runtime nixpkgs declines to package — see
`flake.nix` for the root-cause writeup). That build is ~30–60 min on
a single GHA macos runner. First-run on a clean cache pays the full
cost; subsequent runs reuse `/nix/store` paths via cache hits.

The release workflow targets three platforms:

| target                       | runner       | bottleneck                      |
|------------------------------|--------------|---------------------------------|
| `x86_64-unknown-linux-gnu`   | ubuntu-latest | none — flang-rt is darwin-only |
| `aarch64-apple-darwin`       | macos-14     | flang-rt cold build             |
| `x86_64-apple-darwin`        | macos-13     | flang-rt cold build             |

## Two cache layers

### 1. Magic Nix Cache (default, free, no setup)

Wired by default via
`systemic-engineering/ci/actions/nix-setup@main` →
`DeterminateSystems/magic-nix-cache-action@main`. This uses GitHub's
own `actions/cache` infrastructure under the hood: each workflow run
populates the cache; subsequent runs in the same repo (within GitHub's
10 GB / 7-day retention window) get hits.

**Pros:** zero config, zero account, zero token, lives entirely
inside GitHub's cache infrastructure.

**Cons:** repo-scoped (not cross-repo / cross-fork). Cold start on
each new flake-input revision. GitHub's 10 GB cache eviction is
LRU — frequently-used paths stay warm, rarely-used ones drop.

For v0.1.1 we ship with Magic Nix Cache only. If the first release
build of a given flake pin succeeds (even slowly), all subsequent
release tags reuse the cached `/nix/store/<hash>-flang-rt-21.1.8`
path within the retention window.

### 2. Cachix (opt-in, persistent, requires account + secret)

If Magic Nix Cache proves insufficient (timeouts on cold runners,
cache evictions during long quiet periods, want to share across
forks), add Cachix. **This requires manual setup by Alex** — no
automation can create a cache namespace on a third party.

#### Manual steps (Alex)

1. Sign up at https://app.cachix.org (Reed/Alex login).
2. Create a cache named `systemic-engineering` (the release.yml is
   hard-coded to this name; change `cachix-name:` in release.yml
   if you pick a different name).
3. Choose "I want my cache to be public" — readers don't need a
   token, only the push step does.
4. From the cache settings, generate a write auth token.
5. Add it as a repo secret:
   - https://github.com/systemic-engineering/mirror/settings/secrets/actions
   - Name: `CACHIX_AUTH_TOKEN`
   - Value: the token from step 4.
6. (Optional) Locally push the existing flang-rt build to seed the
   cache so the very first CI run is a hit:
   ```sh
   cachix authtoken <token>
   nix build .#flang-rt --json | jq -r '.[].outputs | to_entries[].value' \
     | cachix push systemic-engineering
   ```

Once `CACHIX_AUTH_TOKEN` is set as a secret, the release workflow
will automatically enable cachix on the next tagged run. No code
change needed — release.yml uses `secrets.CACHIX_AUTH_TOKEN != ''`
as the enable flag.

## What v0.1.1 ships with

- Both macOS targets restored to the release matrix.
- Magic Nix Cache wired (default of `nix-setup@main`).
- Cachix opt-in path wired but inactive (no secret set).
- If the v0.1.1 release run times out on macOS: cancel, add the
  Cachix secret per steps above, re-trigger via `workflow_dispatch`
  with `tag: v0.1.1`.

## Future: FlakeHub cache

Determinate's hosted cache offering. The nix-setup composite already
installs via DeterminateSystems/nix-installer-action which would
authenticate to FlakeHub *if* the workflow had FlakeHub credentials.
Per earlier release.yml runs the magic-nix-cache step logged
"FlakeHub: cache initialized failed: Unauthenticated" — wired but
unauthenticated.

If/when systemic-engineering subscribes to FlakeHub, swap or stack
it under the same `enable-` pattern. Not a v0.1.1 concern.
