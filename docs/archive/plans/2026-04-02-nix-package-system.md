# @nix Domain + Package System — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the conversation package system: grammars for @ci/@ca/@ai, Nix DSL functions (conversation.lib.beam, conversation.lib.package), and prove the mount works end-to-end.

**Architecture:** Write .conv grammars, scaffold packages, build Nix functions that validate + project + build. The @nix grammar is declared but the full conversation→flake compilation is deferred — the pragmatic path is Nix functions first, grammar-driven generation later.

**Tech Stack:** .conv grammars, Nix flakes, fragmentation.lib.project, Gleam

---

## Task 1: Write ODA grammars

**Files:**
- Create: `/Users/alexwolf/dev/projects/conversation/conv/ci.conv`
- Create: `/Users/alexwolf/dev/projects/conversation/conv/ca.conv`
- Create: `/Users/alexwolf/dev/projects/conversation/conv/ai.conv`
- Create: `/Users/alexwolf/dev/projects/conversation/conv/nix.conv`

ci.conv:
```conv
in @beam

grammar @ci {
  type = phase | result | measurement

  type phase = check | build | test | settle

  type result = green | red | shifting

  type measurement = shannon | spectral | convergence

  action measure {
    target: measurement
  }

  action settle {
    phase: phase
  }
}
```

ca.conv:
```conv
in @ci

grammar @ca {
  type = observation | decision | action

  type observation = shift | settlement | drift

  type decision = spawn | notify | wait

  type action = agent | crystal | converge

  action observe {
    source: measurement
  }

  action decide {
    observation: observation
  }

  action act {
    decision: decision
  }
}
```

ai.conv:
```conv
in @ca

grammar @ai {
  type = collapse | tension | branch

  type collapse = clear | partial | ambiguous

  type tension = competing | complementary | contradictory

  action project {
    input: collapse
  }

  action branch {
    tension: tension
  }

  action escalate {
    tension: tension
  }
}
```

nix.conv:
```conv
in @beam

grammar @nix {
  type = flake | input | output | derivation

  type input = url | follows | path

  type output = package | shell | lib

  type derivation = build | check | dev

  action emit {
    target: flake
  }

  action build {
    target: derivation
  }
}
```

Commit to conversation repo.

## Task 2: Scaffold @ci, @ca, @ai packages

Create three package repos at `/Users/alexwolf/dev/projects/`:

Each follows the conversation-admin pattern:
- `gleam.toml` (package deps)
- `<name>.conv` (grammar copy)
- `flake.nix` (dev shell)
- `Justfile`
- `.gitignore`
- `src/` with stub .gleam module
- `test/` with gleeunit entry

**conversation-ci:**
```toml
name = "conversation_ci"
version = "0.1.0"
description = "conversation @ci — continuous integration as eigenvalue observation"

[dependencies]
gleam_stdlib = ">= 0.44.0 and < 2.0.0"
gleam_erlang = ">= 0.25.0 and < 2.0.0"
gleam_otp = ">= 1.0.0 and < 2.0.0"

[dev-dependencies]
gleeunit = ">= 1.0.0 and < 2.0.0"
```

Stub `src/ci.gleam`:
```gleam
/// @ci — continuous integration as eigenvalue observation.
/// Subscribes to spectral-db eigenvalue shifts.
/// Measures Shannon equivalence on grammar changes.
/// Green is settlement. Red is shift.
pub fn name() -> String { "@ci" }
```

Same pattern for conversation-ca and conversation-ai with appropriate stubs.

Git init each, commit.

## Task 3: conversation.lib.beam + conversation.lib.package

**File:** `/Users/alexwolf/dev/projects/conversation/flake.nix`

Add two Nix library functions to the conversation flake:

**conversation.lib.package** — declares a package:
```nix
lib.package = { name, grammar, src }:
  { inherit name grammar src; type = "conversation-package"; };
```

**conversation.lib.beam** — builds a BEAM app from core + mounted packages:
```nix
lib.beam = { pkgs, name, src, packages ? {} }:
  let
    # Merge source trees: core src/ + each package's src/
    mergedSrc = pkgs.runCommand "${name}-merged" {} ''
      mkdir -p $out/src $out/test
      # Core sources
      cp -r ${src}/src/* $out/src/ 2>/dev/null || true
      cp -r ${src}/test/* $out/test/ 2>/dev/null || true
      # Package sources (the mount)
      ${builtins.concatStringsSep "\n" (builtins.attrValues (
        builtins.mapAttrs (pname: pkg: ''
          cp -r ${pkg.src}/src/* $out/src/ 2>/dev/null || true
        '') packages
      ))}
      # Copy gleam.toml from core
      cp ${src}/gleam.toml $out/gleam.toml 2>/dev/null || true
    '';
  in {
    inherit mergedSrc;
    # The merged source tree is what gleam compiles
    src = mergedSrc;
  };
```

This is the minimal viable version. It merges source trees. Validation (checking grammar requirements) comes later when the conversation compiler can parse grammars at Nix eval time.

Also update the conversation flake inputs to include fragmentation:
```nix
inputs.fragmentation.url = "git+ssh://git@github.com/systemic-engineering/fragmentation";
```

## Task 4: Update conversation-beam flake to use lib.beam

**File:** `/Users/alexwolf/dev/projects/conversation-beam/flake.nix`

Create a flake.nix that uses conversation.lib.beam:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    conversation.url = "git+ssh://git@github.com/systemic-engineering/conversation";
    admin.url = "git+ssh://git@github.com/systemic-engineering/conversation-admin";
  };

  outputs = { self, nixpkgs, flake-utils, conversation, admin }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        beamPkgs = pkgs.beam.packages.erlang_27;

        # Use conversation.lib.beam to build with optional packages
        app = conversation.lib.beam {
          inherit pkgs;
          name = "conversation-beam";
          src = ./.;
          packages = {
            admin = admin;
          };
        };
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

        # Dev shell with admin mounted — gleam sees both core + admin sources
        devShells.with-admin = pkgs.mkShell {
          buildInputs = [
            pkgs.gleam pkgs.erlang_27 beamPkgs.rebar3
            pkgs.git pkgs.just
          ];
          shellHook = ''
            export LANG=en_US.UTF-8
            echo "admin package mounted"
            echo "merged source: ${app.mergedSrc}"
          '';
        };
      });
}
```

## Task 5: End-to-end test

Verify the mount works:

1. `nix develop .#with-admin` in conversation-beam → should show merged source path
2. Copy the merged source tree to a temp dir
3. Run `gleam build` on the merged tree → should compile core + admin together
4. Run `gleam test` → all tests pass

This proves: conversation.lib.beam merges source trees, and the merge produces a valid Gleam build.
