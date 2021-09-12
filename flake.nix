{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [ cargo rustup rustc rustfmt rust-analyzer clippy ];
          # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          shellHook = ''
            mkdir -p .vim
            echo '{"rust-analyzer.serverPath": "${pkgs.rust-analyzer}/bin/rust-analyzer", "rust-client.disableRustup": true}' > .vim/coc-settings.json
          '';
        };
      });
}
