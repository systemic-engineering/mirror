{
  description = "spectral — git for graphs";

  inputs = {
    # Pinned to the exact nixpkgs rev that builds flang 21.1.8 on
    # aarch64-darwin (verified-identical narHash to nixos-unstable @ 26.05).
    # The pin is load-bearing: flang-rt below is built from the SAME LLVM
    # monorepo source the flang compiler was built from, so the compiler and
    # its runtime are version-locked. Bumping this rev means rebuilding both.
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

        llvm = pkgs.llvmPackages;
        # The flang compiler nixpkgs already builds (FLANG_STANDALONE_BUILD=true).
        flang = llvm.flang;

        # flang-rt: the Fortran runtime nixpkgs forgets to build.
        #
        # Root cause (NOT a stale pin): nixpkgs builds flang with
        # FLANG_STANDALONE_BUILD=true and never defines a flang-rt package.
        # Without it, any runtime-backed intrinsic (SUM → _FortranASumReal8,
        # MATMUL → _FortranAMatmulReal8Real8, …) is an unresolved symbol and
        # the flang driver's implicit `-lflang_rt.runtime` fails to link.
        #
        # Fix: build flang-rt from the LLVM monorepo source nixpkgs already
        # pins (llvm.libllvm.monorepoSrc), via the runtimes/ build system with
        # LLVM_ENABLE_RUNTIMES=flang-rt. Three required tweaks:
        #   1. CMAKE_Fortran_COMPILER_WORKS=ON — CMake's compiler check tries to
        #      link the very runtime it is building (chicken-and-egg); bypass it.
        #   2. python3 in nativeBuildInputs — the runtimes build invokes it.
        #   3. AddFlangRT.cmake hardcodes a macOS 10.7 deployment target that is
        #      *appended* (so it wins) and predates clock_gettime / CLOCK_REALTIME
        #      (10.12) used in random.cpp. Bump to 11.0, the platform minimum.
        #
        # Output: <flang-rt>/lib/clang/21/lib/darwin/libflang_rt.runtime.a.
        monorepoSrc = llvm.libllvm.monorepoSrc;
        release_version = llvm.release_version;
        flang-rt = pkgs.stdenv.mkDerivation (finalAttrs: {
          pname = "flang-rt";
          version = release_version;
          src = pkgs.runCommand "flang-rt-src-${release_version}" { } ''
            mkdir -p "$out"
            cp -r ${monorepoSrc}/runtimes "$out/"
            cp -r ${monorepoSrc}/flang-rt "$out/"
            cp -r ${monorepoSrc}/flang    "$out/"
            cp -r ${monorepoSrc}/cmake    "$out/"
            cp -r ${monorepoSrc}/llvm     "$out/"
            cp -r ${monorepoSrc}/third-party "$out/"
            chmod -R u+w "$out"
            substituteInPlace "$out/flang-rt/cmake/modules/AddFlangRT.cmake" \
              --replace 'DARWIN_osx_BUILTIN_MIN_VER 10.7' 'DARWIN_osx_BUILTIN_MIN_VER 11.0'
          '';
          # nix unpacks src to a dir named after the store path; runtimes/ is inside.
          setSourceRoot = "sourceRoot=$(echo */runtimes)";
          nativeBuildInputs = [ pkgs.cmake pkgs.ninja pkgs.python3 flang llvm.clang ];
          cmakeFlags = [
            "-DLLVM_ENABLE_RUNTIMES=flang-rt"
            "-DCMAKE_Fortran_COMPILER=${flang}/bin/flang"
            "-DCMAKE_Fortran_COMPILER_WORKS=ON"
            "-DFLANG_RT_INCLUDE_TESTS=OFF"
            "-DFLANG_RT_ENABLE_STATIC=ON"
            "-DFLANG_RT_ENABLE_SHARED=OFF"
            "-DLLVM_VERSION_MAJOR=21"
            "-DLLVM_VERSION_MINOR=1"
            "-DLLVM_VERSION_PATCH=8"
            "-DLLVM_INCLUDE_TESTS=OFF"
          ];
          meta = { description = "LLVM Fortran runtime (flang-rt) for ${release_version}"; };
        });
        # flang-rt installs into the clang resource-dir layout. The flang driver
        # searches here for libflang_rt.runtime.a; we also expose it for -L.
        flangRtLibDir = "${flang-rt}/lib/clang/21/lib/darwin";
      in {
        # Expose flang-rt as a package output (darwin only; it is the macOS
        # Fortran runtime). On non-darwin this output is omitted.
        packages = pkgs.lib.optionalAttrs isDarwin { inherit flang-rt; };

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
            # Darwin-only: flang + flang-rt for the numerical substrate compiler.
            flang flang-rt llvm.clang
            pkgs.cargo pkgs.rustc
          ];
          shellHook = ''
            export LANG=en_US.UTF-8
          '' + rust.rustHook;
        }
        # LAPACK/BLAS discoverable to the linker and to build.rs scripts
        # without hand-wiring store paths. Flang-rt is darwin-only.
        // {
          LAPACK_DIR = "${pkgs.lapack}";
          BLAS_DIR = "${pkgs.blas}";
        }
        // pkgs.lib.optionalAttrs isDarwin {
          FLANG = "${flang}/bin/flang";
          FLANG_RT_DIR = flangRtLibDir;
          # Darwin link paths for the openssl-sys / libgit2-sys / libz-sys
          # transitive chain (libiconv) + the prismqueer LAPACK feature
          # (lapack, blas) + the flang-rt Fortran runtime.
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
          RUSTFLAGS = "-L ${flangRtLibDir} -L ${pkgs.lapack}/lib -L ${pkgs.blas}/lib -L ${pkgs.libiconv}/lib";
        });
      }
    );
}
