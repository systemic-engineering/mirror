{
  description = "spectral — git for graphs";

  inputs = {
    # Originally pinned for flang 21.1.8 + flang-rt. prismqueer now compiles
    # its Fortran with gfortran (since prism 7e6b00f; the prism rev itself is
    # pinned only in rust/Cargo.lock, see packages.mirror), so flang is gone
    # from this shell. The pin stays, to keep a toolchain move out of the
    # prism bump.
    nixpkgs.url     = "github:NixOS/nixpkgs/fef9403a3e4d31b0a23f0bacebbec52c248fbb51";
    flake-utils.url = "github:numtide/flake-utils";
    flakes.url      = "github:systemic-engineering/flakes";
    flakes.inputs.nixpkgs.follows = "nixpkgs";
    flakes.inputs.flake-utils.follows = "flake-utils";

  };

  outputs = { self, nixpkgs, flake-utils, flakes }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rust = flakes.lib.${system}.rust;
        isDarwin = pkgs.stdenv.hostPlatform.isDarwin;
      in {
        # The mirror binary. spectral-beam builds it as a composition:
        # tool = this attr, in = the commit, out = the binary.
        packages = rec {
          mirror = pkgs.rustPlatform.buildRustPackage {
            pname = "mirror";
            version = "0.1.0";
            src = ./rust;
            # Cargo.lock is the single source of truth for prism's rev (no
            # separate flake input to keep in step). nix vendors the git deps
            # itself, the `imperfect` submodule included; prism is public, so
            # no credentials. On a prism bump, update this hash too.
            cargoLock = {
              lockFile = ./rust/Cargo.lock;
              outputHashes = {
                "prismqueer-0.1.1" = "sha256-k9D9eHlnQlr0J/h/jtAZ/8srDN4I341YK3o291xNmfM=";
              };
            };

            nativeBuildInputs = [ pkgs.gfortran ];
            buildInputs = [ pkgs.lapack pkgs.blas ]
              ++ pkgs.lib.optionals isDarwin [ pkgs.libiconv ];
            # prismqueer's build.rs: $FC first, then LAPACK_DIR / BLAS_DIR.
            FC = "gfortran";
            LAPACK_DIR = "${pkgs.lapack}";
            BLAS_DIR = "${pkgs.blas}";

            # Tests run in the commit gate (`just pre-commit`, all five crates;
            # the RED-by-design batteries are behind the `red-claims`
            # feature). The fixed point gets its own `checks.fixpoint` output
            # once T1 lands (handoff mirror-007), not a checkPhase.
            doCheck = false;
          };
          default = mirror;
        };

        devShells.default = pkgs.mkShell ({
          buildInputs = [
            pkgs.git pkgs.just pkgs.jq
            pkgs.openssl pkgs.zlib
            pkgs.gfortran
            # Fortran numerical substrate (LAPACK/BLAS) — provided everywhere
            # so the prismqueer FFI link target is one ABI world (nix-store).
            pkgs.lapack pkgs.blas
            # Perf-tooling floor (Seam Phase D `91e79c8` §7 Sub-arc 3c).
            #
            # Per RED `d25b91a`:
            #   sccache        — compilation cache; big win on incremental rebuilds.
            #                    Opt-in via `.cargo/config.toml`
            #                    `[build] rustc-wrapper = "sccache"` in a later tick.
            #   cargo-nextest  — per-process test isolation; 30% typical speedup;
            #                    consumer of /loop 2026-07-05 Arc 2 (thread-safety
            #                    Option A) once source migration lands.
            #   cargo-audit    — discharges the `audit` target in mirror.spec
            #                    (currently PARTIAL via tool-unavailable carve-out
            #                    at bootstrap/src/lib.rs:~1276). Wiring cargo-audit
            #                    converts audit verdict from `partial` to `success`.
            #   lld            — cross-platform LLVM linker; conservative floor per
            #                    Reed §8 signal-to-Alex #4. Linux `mold` / Darwin
            #                    `sold` upgrades land in a follow-up tick if the
            #                    perf delta warrants.
            pkgs.sccache pkgs.cargo-nextest pkgs.cargo-audit pkgs.lld
          ] ++ rust.rustTools
            ++ pkgs.lib.optionals isDarwin [
            pkgs.libiconv
            pkgs.cargo pkgs.rustc
          ];
          shellHook = ''
            export LANG=en_US.UTF-8
          '' + rust.rustHook + ''
            # Mirror.Offer.Wait at build-substrate altitude per Alex
            # 2026-09-03 Void-404 ("the operator for the whole compiler
            # and the MCP and everything") + Mara 2026-08-27 circular-
            # recursive-mcp-autopoietic-closure (indexing accretion as
            # autopoietic loop closure). One tick per direnv shell entry:
            #
            #   Mirror  — inspect ~/.local/bin/mirror current state
            #   Offer   — symlink to $CARGO_TARGET_DIR/release/mirror
            #   Wait    — idempotent; if already correct, no-op
            #
            # Retires the 2026-08-26 bootstrap install (bootstrap/ deleted
            # then per Alex directive; rust/ binary owns terminal-form per
            # FLOOR discipline; wrapper at ~/.os/bin/mirror still calls
            # ~/.local/bin/mirror which this hook keeps current). Auto-fires
            # for alexwolf + reed + mara + any Pack peer entering the mirror
            # devshell. The install-substrate becomes self-maintaining;
            # every shell entry accretes one crystal on the tray at build-
            # substrate altitude.
            _MIRROR_BIN="''${CARGO_TARGET_DIR:-$HOME/.cargo-target}/release/mirror"
            _MIRROR_LINK="$HOME/.local/bin/mirror"
            mkdir -p "$HOME/.local/bin"
            if [ ! -L "$_MIRROR_LINK" ] || [ "$(readlink "$_MIRROR_LINK")" != "$_MIRROR_BIN" ]; then
              ln -sfn "$_MIRROR_BIN" "$_MIRROR_LINK"
              echo "flake.nix: installed ~/.local/bin/mirror -> $_MIRROR_BIN"
            fi
            unset _MIRROR_BIN _MIRROR_LINK
          '';
        }
        # LAPACK/BLAS discoverable to the linker and to build.rs scripts
        # without hand-wiring store paths. Flang-rt is darwin-only.
        // {
          LAPACK_DIR = "${pkgs.lapack}";
          BLAS_DIR = "${pkgs.blas}";
        }
        // pkgs.lib.optionalAttrs isDarwin {
          # No FLANG here: prismqueer's build.rs prefers $FLANG over gfortran
          # but links libgfortran, so exporting it breaks the link.
          #
          # Darwin link paths for the openssl-sys / libgit2-sys / libz-sys
          # transitive chain (libiconv) + the prismqueer LAPACK feature
          # (lapack, blas).
          #
          # Root cause + durability rationale (Reed + Taut root-cause
          # 2026-07-15, superseding the 2026-07-14 house-cleanup attempt
          # which was insufficient from the start):
          #
          # The prior fix set NIX_LDFLAGS = "-L... -L... -L${pkgs.libiconv}/lib"
          # believing this OVERRODE nix's auto-computed value. It does not.
          # nix's mkShell APPENDS the buildInputs' auto-computed library
          # paths to any manual NIX_LDFLAGS assignment, producing a 1700+
          # char cascade with duplicates that the cc-wrapper's validation
          # + the darwin linker's ordering rules ultimately reject. The
          # manual libiconv path was in the string, but the string as a
          # whole was structurally broken.
          #
          # Cargo's rustc invocation does NOT consume NIX_LDFLAGS directly;
          # it consumes RUSTFLAGS. Setting `-L <path>` in RUSTFLAGS makes
          # cargo pass `-L <path>` to rustc, which forwards to the linker.
          # No nix post-processing; no cascade; no ordering games. Cargo
          # respects RUSTFLAGS verbatim.
          #
          # This fix is structurally durable because it addresses the
          # actual mechanism cargo uses to find libraries (RUSTFLAGS)
          # rather than fighting an environment variable (NIX_LDFLAGS)
          # whose post-processing semantics the previous fix misread.
          RUSTFLAGS = "-L ${pkgs.lapack}/lib -L ${pkgs.blas}/lib -L ${pkgs.libiconv}/lib";
        });
      }
    );
}
