{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      perSystem =
        { system, ... }:
        let
          pkgs = inputs.nixpkgs.legacyPackages.${system};
        in
        {
          devShells.default = pkgs.mkShell {
            nativeBuildInputs = [
              pkgs.zig_0_15
              pkgs.zls_0_15
            ];
          };
        };
    };
}
