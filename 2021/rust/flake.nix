{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-parts.url = "github:hercules-ci/flake-parts";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux"];
      perSystem = {system, ...}: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        buildRustToolchain = pkgs.rust-bin.selectLatestNightlyWith;

        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (
          buildRustToolchain (toolchain: toolchain.default)
        );

        commonArgs = {
          pname = "aoc-2021-rust";
          version = "0.0.0";

          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              (pkgs.lib.hasSuffix "input\.txt" path)
              || (craneLib.filterCargoSources path type);
          };
          strictDeps = true;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            (buildRustToolchain (toolchain:
              toolchain.default.override {
                extensions = ["rust-analyzer" "rust-src" "rust-std"];
              }))
            pkgs.cargo-nextest
          ];
        };

        checks = {
          build = craneLib.cargoBuild (
            commonArgs
            // {inherit cargoArtifacts;}
          );

          fmt = craneLib.cargoFmt commonArgs;

          nextest = craneLib.cargoNextest (commonArgs
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
            });
        };
      };
    };
}
