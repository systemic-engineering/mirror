{
  description = "conversation — gradients over trees";

  inputs = {
    nixpkgs.url     = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flakes.url      = "path:/Users/reed/.flakes";
    flakes.inputs.nixpkgs.follows = "nixpkgs";
    flakes.inputs.flake-utils.follows = "flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, flakes }:
    let
      # ── lib functions (system-independent) ──────────────────────────────────

      # Declare a conversation package (for use as a packages.{name} entry).
      lib.package = { name, src, grammar ? null }:
        { inherit name src grammar; _type = "conversation-package"; };

      # Merge core + packages into a unified source tree for gleam/rebar builds.
      lib.beam = { pkgs, name, src, packages ? {} }:
        let
          mergedSrc = pkgs.runCommand "${name}-merged" {} ''
            mkdir -p $out/src $out/test

            # Copy core sources
            if [ -d "${src}/src" ]; then
              cp -rn ${src}/src/* $out/src/ 2>/dev/null || true
            fi
            if [ -d "${src}/test" ]; then
              cp -rn ${src}/test/* $out/test/ 2>/dev/null || true
            fi

            # Mount each package's sources into the tree
            ${builtins.concatStringsSep "\n" (builtins.attrValues (
              builtins.mapAttrs (pname: pkg:
                let pkgSrc = if builtins.isAttrs pkg && pkg ? src then pkg.src else pkg;
                in ''
                  if [ -d "${pkgSrc}/src" ]; then
                    cp -rn ${pkgSrc}/src/* $out/src/ 2>/dev/null || true
                  fi
                ''
              ) packages
            ))}

            # Copy gleam.toml from core (the merged tree needs it)
            if [ -f "${src}/gleam.toml" ]; then
              cp ${src}/gleam.toml $out/gleam.toml
            fi
          '';
        in {
          inherit mergedSrc name;
          src = mergedSrc;
        };

    in flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rust = flakes.lib.${system}.rust;
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.git pkgs.just pkgs.jq
            pkgs.openssl pkgs.zlib
            pkgs.erlang pkgs.rebar3
            pkgs.gleam
            pkgs.gfortran
          ] ++ rust.rustTools
            ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isDarwin [
            pkgs.libiconv
          ];
          shellHook = ''
            export LANG=en_US.UTF-8
          '' + rust.rustHook;
        };
      }) // { inherit lib; };
}
