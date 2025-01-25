{
  description = "Flake with RISC-V 32-bit and 64-bit toolchains";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem = { system, pkgs, ... }: rec {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;

          overlays = [
            (import inputs.rust-overlay)
          ];
        };
      
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            (rust-bin.stable.latest.default.override {
              targets = [ "riscv32imac-unknown-none-elf" "riscv64imac-unknown-none-elf" ];
            })
            
            pkgsCross.riscv32.stdenv.cc
            pkgsCross.riscv64.stdenv.cc
            qemu
            dtc
          ];
        };
      };
    };
}
