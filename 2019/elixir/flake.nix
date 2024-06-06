{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {
    self,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux"];
      perSystem = {system, ...}: let
        pkgs = inputs.nixpkgs.legacyPackages.${system};
        allUmbrellaBeamDeps = builtins.attrValues (import ./deps.nix {inherit (pkgs) lib beamPackages;});
      in rec {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [pkgs.elixir pkgs.mix2nix];
        };

        packages = {
          compile = pkgs.beamPackages.buildMix {
            name = "compile";
            version = "0.1.0";
            src = self;

            beamDeps = allUmbrellaBeamDeps;
          };

          doc = pkgs.beamPackages.buildMix {
            name = "doc";
            version = "0.1.0";
            src = self;

            beamDeps = allUmbrellaBeamDeps;

            postBuild = "mix docs --no-deps-check";
            postInstall = ''
              mkdir -p $out/share/doc
              cp -rv ./doc/* $out/share/doc/
            '';
          };
        };

        checks =
          packages
          // {
            credo = pkgs.beamPackages.buildMix {
              name = "credo";
              version = "0.1.0";
              src = self;

              beamDeps = allUmbrellaBeamDeps;

              doCheck = true;
              checkPhase = "mix credo";
            };

            format = pkgs.beamPackages.buildMix {
              name = "format";
              version = "0.1.0";
              src = self;

              beamDeps = allUmbrellaBeamDeps;

              doCheck = true;
              checkPhase = "mix format --check-formatted";
            };

            test = pkgs.beamPackages.buildMix {
              name = "test";
              version = "0.1.0";
              src = self;

              beamDeps = allUmbrellaBeamDeps;
              mixEnv = "test";

              # Doc tests require `module_info(:compile)` to contain `:source`,
              # but deterministic builds strip the source
              erlangDeterministicBuilds = false;

              doCheck = true;
              checkPhase = "mix test --no-deps-check";
            };
          };
      };
    };
}
