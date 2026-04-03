# Extract @admin Package — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extract the web admin panel from conversation-beam into a standalone conversation-admin package with an `@admin` grammar contract.

**Architecture:** Move Gleam web modules (router, api, dashboard, web) + their FFI bridges out of conversation-beam into a new conversation-admin repo. Write the `@admin` grammar as the interface contract. conversation-beam stays lean (actors, cluster, store, MCP). The admin package is the first proof of the conversation package system.

**Tech Stack:** Gleam, Mist/Wisp, Erlang/OTP, .conv grammars

**Spec:** `projects/conversation/docs/specs/2026-04-02-package-system-design.md`

**Repos:**
- conversation-beam (core): `/Users/alexwolf/dev/projects/conversation-beam/`
- conversation-admin (new): `/Users/alexwolf/dev/projects/conversation-admin/`
- conversation (grammars): `/Users/alexwolf/dev/projects/conversation/`

**Build/test:**
```bash
# conversation-beam (core)
cd /Users/alexwolf/dev/projects/conversation-beam && gleam test

# conversation-admin (package)
cd /Users/alexwolf/dev/projects/conversation-admin && gleam test
```

---

## File Map

### conversation-beam — REMOVE these (move to admin)
- `src/router.gleam`
- `src/api.gleam`
- `src/api_ffi.erl`
- `src/dashboard.gleam`
- `src/web.gleam`
- `src/cluster.gleam`
- `src/cluster_ffi.erl`
- `test/router_test.gleam`

### conversation-beam — KEEP (core)
- `src/conversation_actor.erl`
- `src/conversation_cluster.erl`
- `src/conversation_agent_protocol.erl`
- `src/conversation_store.erl`
- `src/conversation_mcp.erl`
- `src/conversation_beam_app.erl`
- `src/conversation_beam_sup.erl`
- `src/conversation_beam_main.erl`
- `src/conversation_cli.erl`
- `src/conversation_test_graph.erl`
- `test/conversation_cluster_tests.erl`

### conversation-beam — MODIFY
- `gleam.toml` — remove mist, wisp, gleam_http, gleam_json deps
- `conversation_beam_main.erl` — remove web:start(Port) call

### conversation-admin — CREATE (new repo)
- `gleam.toml`
- `admin.conv` — the grammar contract
- `src/router.gleam`
- `src/api.gleam`
- `src/api_ffi.erl`
- `src/dashboard.gleam`
- `src/web.gleam`
- `src/cluster.gleam`
- `src/cluster_ffi.erl`
- `test/router_test.gleam`
- `test/conversation_admin_test.gleam`
- `flake.nix`
- `Justfile`
- `.gitignore`

### conversation — ADD grammar
- `conv/admin.conv` — the @admin domain grammar

---

### Task 1: Write @admin grammar

**Files:**
- Create: `/Users/alexwolf/dev/projects/conversation/conv/admin.conv`

- [ ] **Step 1: Write the grammar**

```conv
in @beam

grammar @admin {
  type = route | panel | api

  type route = path | handler

  type panel = dashboard | garden | prism

  type api = state | grammar | cluster

  action serve {
    port: route
  }

  action mount {
    panel: panel
  }

  action respond {
    endpoint: api
  }
}
```

This follows the existing patterns in `conv/`:
- `in @beam` — admin extends the beam domain (needs process, supervision, module types)
- Types declare the interface surface
- Actions declare what the package can do

- [ ] **Step 2: Verify it parses**

```bash
cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo run -- -e 'in @admin' conv/
```

If the conversation parser is available and the grammar resolves, this should succeed. If the parser doesn't support resolving from the conv/ directory yet, just verify the file is syntactically valid:

```bash
cd /Users/alexwolf/dev/projects/conversation && nix develop -c cargo run -- test conv/admin.conv
```

Note: the grammar may have tests after a `---` separator. For now, the grammar without tests is fine.

- [ ] **Step 3: Commit to conversation repo**

```bash
cd /Users/alexwolf/dev/projects/conversation
git add conv/admin.conv
git commit -m "🟢 @admin grammar: route, panel, api types + serve/mount/respond actions"
```

---

### Task 2: Create conversation-admin repo

**Files:**
- Create: `/Users/alexwolf/dev/projects/conversation-admin/gleam.toml`
- Create: `/Users/alexwolf/dev/projects/conversation-admin/admin.conv`
- Create: `/Users/alexwolf/dev/projects/conversation-admin/.gitignore`
- Create: `/Users/alexwolf/dev/projects/conversation-admin/Justfile`
- Create: `/Users/alexwolf/dev/projects/conversation-admin/flake.nix`

- [ ] **Step 1: Create directory**

```bash
mkdir -p /Users/alexwolf/dev/projects/conversation-admin/src
mkdir -p /Users/alexwolf/dev/projects/conversation-admin/test
```

- [ ] **Step 2: Create gleam.toml**

```toml
name = "conversation_admin"
version = "0.1.0"
description = "conversation @admin package — web dashboard for conversation-beam"

[dependencies]
gleam_stdlib = ">= 0.44.0 and < 2.0.0"
gleam_erlang = ">= 0.25.0 and < 2.0.0"
gleam_http = ">= 4.0.0 and < 5.0.0"
gleam_json = ">= 3.0.0 and < 4.0.0"
mist = ">= 5.0.0 and < 7.0.0"
wisp = ">= 2.0.0 and < 3.0.0"

[dev-dependencies]
gleeunit = ">= 1.0.0 and < 2.0.0"
```

Note: no dependency on conversation-beam. The admin package calls core modules via FFI — those modules exist at runtime because the source trees are merged. At compile time, the Erlang modules are resolved from the merged source tree.

- [ ] **Step 3: Copy admin.conv**

Copy `conv/admin.conv` from the conversation repo into the package root:

```bash
cp /Users/alexwolf/dev/projects/conversation/conv/admin.conv /Users/alexwolf/dev/projects/conversation-admin/admin.conv
```

- [ ] **Step 4: Create .gitignore**

```
/build/
/_gleam_artefacts/
/manifest.toml
/.nix-cargo/
/target/
```

- [ ] **Step 5: Create Justfile**

```just
# conversation-admin

check: lint test format-check

test:
    gleam test

lint:
    gleam check

format:
    gleam format src/ test/

format-check:
    gleam format --check src/ test/

pre-commit: check
pre-push: check
```

- [ ] **Step 6: Create flake.nix**

```nix
{
  description = "conversation @admin — web dashboard package";
  inputs = {
    nixpkgs.url     = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs     = nixpkgs.legacyPackages.${system};
        beamPkgs = pkgs.beam.packages.erlang_27;
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.gleam pkgs.erlang_27 beamPkgs.rebar3
            pkgs.git pkgs.just
          ];
          shellHook = ''
            export LANG=en_US.UTF-8
          '';
        };
      });
}
```

- [ ] **Step 7: Init git repo**

```bash
cd /Users/alexwolf/dev/projects/conversation-admin
git init
git add gleam.toml admin.conv .gitignore Justfile flake.nix
git commit -m "🔧 scaffold conversation-admin package"
```

---

### Task 3: Move web modules from conversation-beam to conversation-admin

**Files:**
- Move: `conversation-beam/src/router.gleam` → `conversation-admin/src/router.gleam`
- Move: `conversation-beam/src/api.gleam` → `conversation-admin/src/api.gleam`
- Move: `conversation-beam/src/api_ffi.erl` → `conversation-admin/src/api_ffi.erl`
- Move: `conversation-beam/src/dashboard.gleam` → `conversation-admin/src/dashboard.gleam`
- Move: `conversation-beam/src/web.gleam` → `conversation-admin/src/web.gleam`
- Move: `conversation-beam/src/cluster.gleam` → `conversation-admin/src/cluster.gleam`
- Move: `conversation-beam/src/cluster_ffi.erl` → `conversation-admin/src/cluster_ffi.erl`
- Move: `conversation-beam/test/router_test.gleam` → `conversation-admin/test/router_test.gleam`

- [ ] **Step 1: Copy files**

```bash
cp /Users/alexwolf/dev/projects/conversation-beam/src/router.gleam /Users/alexwolf/dev/projects/conversation-admin/src/
cp /Users/alexwolf/dev/projects/conversation-beam/src/api.gleam /Users/alexwolf/dev/projects/conversation-admin/src/
cp /Users/alexwolf/dev/projects/conversation-beam/src/api_ffi.erl /Users/alexwolf/dev/projects/conversation-admin/src/
cp /Users/alexwolf/dev/projects/conversation-beam/src/dashboard.gleam /Users/alexwolf/dev/projects/conversation-admin/src/
cp /Users/alexwolf/dev/projects/conversation-beam/src/web.gleam /Users/alexwolf/dev/projects/conversation-admin/src/
cp /Users/alexwolf/dev/projects/conversation-beam/src/cluster.gleam /Users/alexwolf/dev/projects/conversation-admin/src/
cp /Users/alexwolf/dev/projects/conversation-beam/src/cluster_ffi.erl /Users/alexwolf/dev/projects/conversation-admin/src/
cp /Users/alexwolf/dev/projects/conversation-beam/test/router_test.gleam /Users/alexwolf/dev/projects/conversation-admin/test/
```

- [ ] **Step 2: Create test entry point**

`/Users/alexwolf/dev/projects/conversation-admin/test/conversation_admin_test.gleam`:

```gleam
import gleeunit

pub fn main() {
  gleeunit.main()
}
```

- [ ] **Step 3: Verify admin package builds independently**

```bash
cd /Users/alexwolf/dev/projects/conversation-admin && gleam build
```

This WILL have compile errors — the Gleam modules reference Erlang modules from core (conversation_actor, conversation_cluster, conversation_store). The api_ffi.erl calls these modules. At standalone build time, these modules don't exist.

**Fix:** The api_ffi.erl and cluster_ffi.erl make calls to core Erlang modules. These are runtime dependencies, not compile-time. Erlang resolves them at call time. Gleam's @external declarations also resolve at runtime. So:

- The Gleam files should compile fine (they only declare @external, no direct Erlang module reference at compile time)
- The .erl files reference conversation_actor etc. — Erlang compiles them with warnings about undefined functions, but doesn't fail

Verify:
```bash
cd /Users/alexwolf/dev/projects/conversation-admin && gleam build
```

If there are compile errors, they'll be in the .erl files. Since Erlang is lenient about undefined external calls at compile time, this should work. If not, add `-compile(nowarn_unused_function).` or similar.

- [ ] **Step 4: Run tests**

```bash
cd /Users/alexwolf/dev/projects/conversation-admin && gleam test
```

The router tests should pass — they use wisp/simulate which doesn't need the real Erlang actors.

- [ ] **Step 5: Commit admin package**

```bash
cd /Users/alexwolf/dev/projects/conversation-admin
git add src/ test/
git commit -m "🟢 move web modules from conversation-beam: router, api, dashboard, web, cluster"
```

---

### Task 4: Strip web modules from conversation-beam

**Files:**
- Remove: `conversation-beam/src/router.gleam`
- Remove: `conversation-beam/src/api.gleam`
- Remove: `conversation-beam/src/api_ffi.erl`
- Remove: `conversation-beam/src/dashboard.gleam`
- Remove: `conversation-beam/src/web.gleam`
- Remove: `conversation-beam/src/cluster.gleam`
- Remove: `conversation-beam/src/cluster_ffi.erl`
- Remove: `conversation-beam/test/router_test.gleam`
- Modify: `conversation-beam/gleam.toml`
- Modify: `conversation-beam/src/conversation_beam_main.erl`

- [ ] **Step 1: Remove web files from core**

```bash
cd /Users/alexwolf/dev/projects/conversation-beam
rm src/router.gleam src/api.gleam src/api_ffi.erl src/dashboard.gleam src/web.gleam src/cluster.gleam src/cluster_ffi.erl
rm test/router_test.gleam
```

- [ ] **Step 2: Strip web deps from gleam.toml**

Replace gleam.toml with core-only deps:

```toml
name = "conversation_beam"
version = "0.1.0"
description = "conversation-beam — core actors, cluster, store, MCP"

[dependencies]
gleam_stdlib = ">= 0.44.0 and < 2.0.0"
gleam_erlang = ">= 0.25.0 and < 2.0.0"
gen_mcp = { path = "../gen_mcp" }

[dev-dependencies]
gleeunit = ">= 1.0.0 and < 2.0.0"
```

No mist, no wisp, no gleam_http, no gleam_json. Core is clean.

- [ ] **Step 3: Update conversation_beam_main.erl**

Remove the `web:start(Port)` call:

```erlang
-module(conversation_beam_main).
-export([main/0, main/1]).

main() ->
    main(#{}).

main(Opts) ->
    StoreDir = maps:get(store_dir, Opts, "/tmp/conversation-beam-store"),
    ok = filelib:ensure_dir(filename:join(StoreDir, "dummy")),
    {ok, _} = application:ensure_all_started(conversation_beam),
    gen_mcp:start(conversation_mcp, #{store_dir => StoreDir}).
```

- [ ] **Step 4: Verify core builds clean**

```bash
cd /Users/alexwolf/dev/projects/conversation-beam && gleam build
```

Expected: compiles with only core modules. No web deps.

- [ ] **Step 5: Run core tests**

```bash
cd /Users/alexwolf/dev/projects/conversation-beam && gleam test
```

Expected: cluster tests pass (6 eunit tests). No router tests (they moved).

- [ ] **Step 6: Commit**

```bash
cd /Users/alexwolf/dev/projects/conversation-beam
git add -A
git commit -m "♻️ extract admin: core is actors + cluster + store + MCP only"
```

---

### Task 5: Create GitHub repo for conversation-admin

**Files:** None (infrastructure)

- [ ] **Step 1: Create remote repo**

```bash
cd /Users/alexwolf/dev/projects/conversation-admin
gh repo create systemic-engineering/conversation-admin --private --source=. --remote=origin
```

Use `git@github.com-reed:` SSH alias if needed:

```bash
git remote add origin git@github.com-reed:systemic-engineering/conversation-admin.git
```

- [ ] **Step 2: Push**

```bash
cd /Users/alexwolf/dev/projects/conversation-admin
git push -u origin main
```

- [ ] **Step 3: Verify**

```bash
gh repo view systemic-engineering/conversation-admin 2>/dev/null || \
  GIT_SSH_COMMAND="ssh -i /Users/reed/.ssh/id_ed25519_github_reed" git ls-remote origin HEAD
```

---

### Task 6: Integration test — mount admin into core

This is the proof that the package system works. In a test directory, create a project that mounts admin into core and builds.

**Files:**
- Create: `/tmp/conversation-mount-test/` (temporary)

- [ ] **Step 1: Create test project**

```bash
mkdir -p /tmp/conversation-mount-test/src
cd /tmp/conversation-mount-test
```

- [ ] **Step 2: Create a gleam.toml that includes both**

```toml
name = "mount_test"
version = "0.1.0"

[dependencies]
gleam_stdlib = ">= 0.44.0 and < 2.0.0"
gleam_erlang = ">= 0.25.0 and < 2.0.0"
gleam_http = ">= 4.0.0 and < 5.0.0"
gleam_json = ">= 3.0.0 and < 4.0.0"
mist = ">= 5.0.0 and < 7.0.0"
wisp = ">= 2.0.0 and < 3.0.0"
gen_mcp = { path = "/Users/alexwolf/dev/projects/gen_mcp" }
```

- [ ] **Step 3: Symlink both source trees into src/**

```bash
cd /tmp/conversation-mount-test
# Copy core sources
cp /Users/alexwolf/dev/projects/conversation-beam/src/*.erl src/
# Copy admin sources
cp /Users/alexwolf/dev/projects/conversation-admin/src/*.gleam src/
cp /Users/alexwolf/dev/projects/conversation-admin/src/*.erl src/
```

- [ ] **Step 4: Add a main.gleam that starts both**

`src/main.gleam`:

```gleam
import gleam/io

pub fn main() {
  io.println("mount test: core + admin compiled together")
}
```

- [ ] **Step 5: Build**

```bash
cd /tmp/conversation-mount-test && gleam build
```

Expected: compiles. Both core Erlang modules and admin Gleam modules coexist. This proves the mount works.

- [ ] **Step 6: Clean up**

```bash
rm -rf /tmp/conversation-mount-test
```

No commit needed — this is a validation step.
