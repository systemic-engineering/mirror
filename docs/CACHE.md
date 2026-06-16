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

Once `CACHIX_AUTH_TOKEN` is set as a secret, the release workflow
will automatically enable cachix on the next tagged run. No code
change needed — release.yml uses `secrets.CACHIX_AUTH_TOKEN != ''`
as the enable flag.

## What v0.1.1 ships with

- Linux: builds and ships as before.
- Both macOS targets present in the release matrix but **gated on
  `secrets.CACHIX_AUTH_TOKEN`** via a job-level `if:`. Until the
  secret exists, the macos-14 and macos-13 jobs are skipped (not
  attempted, not failed — skipped). The release attaches the linux
  binary only.
- The wiring is in place so the moment Alex completes the Cachix
  manual steps above, the next `git tag v0.1.2 && git push origin
  v0.1.2` (or `workflow_dispatch` against v0.1.1) attaches the
  macOS binaries automatically with zero further code changes.

## Why macOS is gated, not attempted

Alternative considered: ship macOS in the matrix without the gate,
let it fail visibly. Rejected because:

1. The failure mode is a 6-hour hang in nix-setup (the MNC dyld
   hang above), not a clean error. That blocks the release job's
   `needs: build` and starves the linux release of its publish step
   for 6 hours.
2. `fail-fast: false` doesn't help — `needs: build` waits for all
   matrix entries to settle (succeed or fail), and a hang is
   neither.
3. A skipped job is a clear signal in the run summary that the
   target needs the secret, which is more actionable than a stuck
   job that looks like it might still finish.

## Future: FlakeHub cache

Determinate's hosted cache offering. The nix-setup composite already
installs via DeterminateSystems/nix-installer-action which would
authenticate to FlakeHub *if* the workflow had FlakeHub credentials.
Per earlier release.yml runs the magic-nix-cache step logged
"FlakeHub: cache initialized failed: Unauthenticated" — wired but
unauthenticated.

If/when systemic-engineering subscribes to FlakeHub, swap or stack
it under the same `enable-` pattern. Not a v0.1.1 concern.
