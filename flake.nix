{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    crate2nix = {
      url = "github:kolloch/crate2nix";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crate2nix, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        crateName = "testrust";

        inherit (import "${crate2nix}/tools.nix" { inherit pkgs; })
          generatedCargoNix;

        project = import (generatedCargoNix {
          name = crateName;
          src = ./.;
        }) {
          inherit pkgs;
          defaultCrateOverrides = pkgs.defaultCrateOverrides // {
            # Crate dependency overrides go here
          };
        };

      in with crateName; {

        packages.${crateName} = project.rootCrate.build;
        defaultPackage = self.packages.${system}.${crateName};

        defaultApp = flake-utils.lib.mkApp {
          drv = self.packages.${system}.${crateName};
          name = crateName;
        };

        devShell = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.packages.${system};
          buildInputs = with pkgs; [ cargo rustup rustc rustfmt rust-analyzer clippy ];
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          shellHook = ''
            mkdir -p .vim
            echo '{"rust-analyzer.serverPath": "${pkgs.rust-analyzer}/bin/rust-analyzer", "rust-client.disableRustup": true}' > .vim/coc-settings.json
          '';
        };
      });
}
