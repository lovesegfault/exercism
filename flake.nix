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
            (fenixPkgs.complete.withComponents [
              "cargo"
              "clippy-preview"
              "rust-src"
              "rust-std"
              "rustc"
              "rustfmt-preview"
            ])
            fenixPkgs.rust-analyzer

            ccls

            cargo-edit
            nixpkgs-fmt
            exercism
          ];
        };
      });
}
