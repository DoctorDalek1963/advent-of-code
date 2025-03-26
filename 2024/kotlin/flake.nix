{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux"];

      perSystem = {system, ...}: let
        pkgs = inputs.nixpkgs.legacyPackages.${system};
      in {
        devShells = let
          inputs = with pkgs; [
            gradle
            kotlin
            just
            ktlint
          ];
        in {
          default = pkgs.mkShell {buildInputs = inputs;};

          ide = pkgs.mkShell {
            buildInputs = inputs ++ [pkgs.jetbrains.idea-community];
          };
        };

        checks = {};
      };
    };
}
