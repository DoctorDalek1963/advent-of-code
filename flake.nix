{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";

  outputs = {nixpkgs, ...}: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        fd
        just
        rustup
        (python3.withPackages (p: [p.aocd]))
      ];
    };
  };
}
