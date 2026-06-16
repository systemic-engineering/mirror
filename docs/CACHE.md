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

### 1. Magic Nix Cache (default on linux, BROKEN on darwin)

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

**KNOWN BROKEN ON DARWIN (2026-06-16).** Empirically observed in
release run https://github.com/systemic-engineering/mirror/actions/runs/27605989894
on the v0.1.1 attempt: the magic-nix-cache static binary fails to
load its dylibs on macOS-14 (the `Referenced from: ...magic-nix-cache`
fragment in the log is the dyld stub before the link error). The
step then hangs in `Waiting for magic-nix-cache to start...` until
GitHub's 6-hour job timeout fires. Job log evidence:

```
  2026-06-16T08:53:53.3445Z Waiting for magic-nix-cache to start...
  2026-06-16T08:53:53.3467Z   Referenced from: <...> magic-nix-cache
  2026-06-16T13:01:06.2956Z ##[error]The operation was canceled.
```

Until this is fixed upstream (or in the nix-setup composite via
an `enable-magic-nix-cache` opt-out), **macOS releases require
Cachix**, which needs Alex's manual setup (next section).

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

Once `CACHIX_AUTH_TOKEN` is set as a secret, the release workflow's
nix-setup step automatically picks up the cache (it uses
`secrets.CACHIX_AUTH_TOKEN != ''` as its enable flag — accessible
at step level even though it is not at job-`if:` level).

## What v0.1.1 ships with

- Linux: builds and ships as before (the same as v0.1.0, with the
  cachix step-level wiring in place under the linux job too — no-op
  on linux since linux doesn't need flang-rt).
- macOS: deferred to v0.1.2. The matrix lists only the linux target;
  the commented block at the top of `release.yml`'s matrix shows
  exactly what to restore for v0.1.2 once the secret is set.

## Why macOS isn't in the v0.1.1 matrix

Alternatives considered:

1. **Job-level `if:` to skip darwin entries when the secret is
   missing.** Rejected because GitHub Actions' `matrix` and
   `secrets` contexts are **not available** in job-level `if:`
   expressions (only `github`, `inputs`, `needs`, `vars` are). The
   v0.1.1 attempts at 27619624866 / 27619706640 / 27619862915 /
   27619885842 all failed at workflow validation for this reason.
2. **Ship macOS without the gate and let it fail.** Rejected
   because the failure mode is a 6-hour hang in nix-setup (the
   MNC dyld hang), not a clean error. `needs: build` waits for all
   matrix entries to settle; a hang stalls the linux release of
   the publish step for the full 6 hours.
3. **Drop macOS from the matrix; document the path to v0.1.2.**
   Chosen. v0.1.1 ships linux cleanly. Restoring macOS for v0.1.2
   is a 3-line workflow change (uncomment the matrix entries) plus
   the Cachix manual setup above.

## Future: FlakeHub cache

Determinate's hosted cache offering. The nix-setup composite already
installs via DeterminateSystems/nix-installer-action which would
authenticate to FlakeHub *if* the workflow had FlakeHub credentials.
Per earlier release.yml runs the magic-nix-cache step logged
"FlakeHub: cache initialized failed: Unauthenticated" — wired but
unauthenticated.

If/when systemic-engineering subscribes to FlakeHub, swap or stack
it under the same `enable-` pattern. Not a v0.1.1 concern.
