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
    flakes.url      = "path:/Users/alexwolf/.flakes";
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
          ] ++ rust.rustTools
            ++ pkgs.lib.optionals isDarwin [
            pkgs.libiconv
            # Fortran numerical substrate (flang → flang-rt → LAPACK/BLAS).
            flang flang-rt pkgs.lapack pkgs.blas llvm.clang
            pkgs.cargo pkgs.rustc
          ];
          shellHook = ''
            export LANG=en_US.UTF-8
          '' + rust.rustHook;
        }
        # Make the Fortran runtime + numerical libs discoverable to the linker
        # and to build.rs scripts without hand-wiring store paths. So that
        # `flang foo.f90 -o foo` links the runtime out of the box.
        // pkgs.lib.optionalAttrs isDarwin {
          FLANG = "${flang}/bin/flang";
          FLANG_RT_DIR = flangRtLibDir;
          LAPACK_DIR = "${pkgs.lapack}";
          BLAS_DIR = "${pkgs.blas}";
          NIX_LDFLAGS = "-L${flangRtLibDir} -L${pkgs.lapack}/lib -L${pkgs.blas}/lib";
        });
      }
    );
}
