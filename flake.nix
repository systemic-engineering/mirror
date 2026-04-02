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
      # Each package's src/ files are copied into the merged src/.
      # Each package's gleam.toml [dependencies] are appended to the merged gleam.toml.
      lib.beam = { pkgs, name, src, packages ? {} }:
        let
          mergedSrc = pkgs.runCommand "${name}-merged" { nativeBuildInputs = [ pkgs.gawk ]; } ''
            mkdir -p $out/src $out/test

            # Copy core sources
            if [ -d "${src}/src" ]; then
              cp -rn ${src}/src/* $out/src/ 2>/dev/null || true
            fi
            if [ -d "${src}/test" ]; then
              cp -rn ${src}/test/* $out/test/ 2>/dev/null || true
            fi

            # Copy gleam.toml from core (start with core's deps, writable for appending)
            if [ -f "${src}/gleam.toml" ]; then
              install -m 644 ${src}/gleam.toml $out/gleam.toml
            fi

            # Mount each package's sources and merge their gleam.toml [dependencies]
            ${builtins.concatStringsSep "\n" (builtins.attrValues (
              builtins.mapAttrs (pname: pkg:
                let pkgSrc = if builtins.isAttrs pkg && pkg ? src then pkg.src else pkg;
                in ''
                  if [ -d "${pkgSrc}/src" ]; then
                    cp -rn ${pkgSrc}/src/* $out/src/ 2>/dev/null || true
                  fi
                  # Merge [dependencies] from package gleam.toml into merged gleam.toml.
                  # New deps are inserted inside the [dependencies] section (before [dev-dependencies]).
                  # Deduplicates: only adds keys not already present in the merged file.
                  if [ -f "${pkgSrc}/gleam.toml" ] && [ -f "$out/gleam.toml" ]; then
                    # Step 1: collect new (non-duplicate) dep lines from the package
                    gawk '
                      NR==FNR {
                        if (/^\[dependencies\]/) { in_deps=1; next }
                        if (/^\[/) { in_deps=0 }
                        if (in_deps && /^[a-zA-Z]/) { key=gensub(/[ =].*/, "", 1, $0); existing[key]=1 }
                        next
                      }
                      /^\[dependencies\]/ { in_deps=1; next }
                      /^\[/ { in_deps=0 }
                      in_deps && /^[a-zA-Z]/ {
                        key=gensub(/[ =].*/, "", 1, $0)
                        if (!(key in existing)) print
                      }
                    ' "$out/gleam.toml" "${pkgSrc}/gleam.toml" > "$out/gleam.toml.pkgdeps_${pname}" || true
                    # Step 2: if there are new deps, splice them into [dependencies] section
                    if [ -s "$out/gleam.toml.pkgdeps_${pname}" ]; then
                      gawk -v newdeps="$out/gleam.toml.pkgdeps_${pname}" '
                        /^\[dev-dependencies\]/ && !inserted {
                          # flush new deps before the [dev-dependencies] header
                          print ""
                          while ((getline line < newdeps) > 0) print line
                          close(newdeps)
                          print ""
                          inserted=1
                        }
                        { print }
                      ' "$out/gleam.toml" > "$out/gleam.toml.new"
                      mv "$out/gleam.toml.new" "$out/gleam.toml"
                    fi
                    rm -f "$out/gleam.toml.pkgdeps_${pname}"
                  fi
                ''
              ) packages
            ))}
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
