{
  description = "Flake with RISC-V 32-bit and 64-bit toolchains";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = { flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem = { pkgs, ... }: rec {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            pkgsCross.riscv32.stdenv.cc
            pkgsCross.riscv64.stdenv.cc
            dtc
          ];
        };
      };
    };
}
