{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:figsoda/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        fenixPkgs = fenix.packages.${system};
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust
            (fenixPkgs.complete.withComponents [
              "cargo"
              "clippy-preview"
              "rust-src"
              "rust-std"
              "rustc"
              "rustfmt-preview"
            ])
            fenixPkgs.rust-analyzer
            cargo-edit

            # C
            ccls

            # x86-64 Assembly
            nasm

            # Haskell
            haskell-language-server
            ghc
            stack
            ormolu

            # Extras
            nixpkgs-fmt
            exercism
          ];
        };
      });
}
