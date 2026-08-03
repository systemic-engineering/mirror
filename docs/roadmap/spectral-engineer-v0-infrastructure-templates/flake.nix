# spectral.engineer v0.1 Nix flake template
#
# Authored: 2026-08-03 by Reed (authored-not-deployed).
# Realizes: shape-doc step 6 (reproducible Gleam Lustre build via Nix).
# Target: copy to `flake.nix` at app repo root when Alex-altitude Gleam
#         Lustre scaffold lands.
#
# Composition anchors:
# - NixOS dockerTools.buildLayeredImage (Dolstra 2004-present)
# - Gleam-nix community package (Pilfold 2016-present, verify current name)
# - flake-utils systemwise multi-arch discipline
#
# [ALEX-VERIFY] markers below indicate where Reed authored reasonable-shape
# defaults that require verification against actual Alex-altitude Gleam
# Lustre scaffold + current gleam-nix package name/API.

{
  description = "spectral.engineer v0.1 — landing served through mirror substrate";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    # [ALEX-VERIFY] gleam-nix input if using community package; else remove
    # and use nixpkgs `gleam` directly.
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        # [ALEX-VERIFY] Gleam Lustre build derivation.
        # Adjust `src`, `buildPhase`, `installPhase` per actual Alex-altitude
        # scaffold shape. Assumes standard `gleam build --target javascript`
        # produces static assets in `build/dev/javascript/spectral_engineer/`.
        spectral-engineer = pkgs.stdenv.mkDerivation {
          pname = "spectral-engineer";
          version = "0.1.0";

          # [ALEX-VERIFY] src should point at Alex-altitude Gleam project root.
          src = ./.;

          nativeBuildInputs = with pkgs; [
            gleam
            erlang     # BEAM elders discipline; runtime for gleam build tools
            nodejs     # for target=javascript build artifact production
          ];

          buildPhase = ''
            # [ALEX-VERIFY] gleam build target; adjust per scaffold
            gleam build --target javascript

            # [ALEX-VERIFY] static asset production; may need lustre_dev_tools
            # or manual index.html + CSS + design-token emission step here.
            # Reference: shards/docs/design.mirror §2-§5 for design-token
            # source-grammar; @cascade/code/gleam/js cascade for loss-lens.
          '';

          installPhase = ''
            mkdir -p $out/public
            # [ALEX-VERIFY] copy static artifacts; adjust source path per
            # actual gleam-lustre output shape.
            cp -r build/dev/javascript/spectral_engineer/* $out/public/
            cp index.html $out/public/index.html || true
            cp -r assets/* $out/public/ 2>/dev/null || true
          '';

          meta = with pkgs.lib; {
            description = "spectral.engineer v0.1 static site (fable-note register)";
            license = licenses.unfree; # [ALEX-VERIFY] choose license
            platforms = platforms.all;
          };
        };

        # Content-addressed docker image assembly per math §3.2 freight stage.
        # BLAKE3 content-address discipline landed downstream via StageFreight;
        # dockerTools.buildLayeredImage produces deterministic layered image.
        spectral-engineer-image = pkgs.dockerTools.buildLayeredImage {
          name = "spectral-engineer";
          tag = "v0.1.0";

          contents = [
            spectral-engineer
            pkgs.caddy   # [ALEX-VERIFY] static server; alternatives: nginx, static-web-server
          ];

          config = {
            Cmd = [
              "${pkgs.caddy}/bin/caddy"
              "file-server"
              "--root"
              "${spectral-engineer}/public"
              "--listen"
              ":8080"
            ];
            ExposedPorts = {
              "8080/tcp" = { };
            };
          };
        };
      in
      {
        packages = {
          default = spectral-engineer;
          spectral-engineer = spectral-engineer;
          spectral-engineer-image = spectral-engineer-image;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            gleam
            erlang
            nodejs
            caddy
            flyctl    # for local fly.io deploys via `nix develop -c flyctl deploy`
          ];
        };

        apps.deploy = {
          type = "app";
          program = toString (pkgs.writeShellScript "deploy" ''
            # [ALEX-VERIFY] deploy dispatch; composes with .stagefreight.yml
            # per shards/io/stagefreight.mirror freight action.
            set -euo pipefail
            echo "Building spectral-engineer v0.1.0..."
            nix build .#spectral-engineer-image
            echo "Loading image into Docker..."
            docker load < result
            echo "Dispatching via flyctl..."
            flyctl deploy --image spectral-engineer:v0.1.0
          '');
        };
      });
}
