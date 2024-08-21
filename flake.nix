{
  description = "Flake with RISC-V 32-bit and 64-bit toolchains";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = { self, nixpkgs, flake-parts }:
    flake-parts.lib.mkFlake {  inherit self nixpkgs;

      systems = ["x86_64-linux"]; # You can add other systems if needed

      devShells = {
        riscv32 = {
          inputs = [];
          nativeInputs = [];

          packages = let
            riscv32-pkgs = import nixpkgs {
              crossSystem = nixpkgs.lib.systems.examples.riscv32-embedded;
            };
          in [
            # Add other packages if needed
          ];
        };

        riscv64 = {
          inputs = [
            # Uncomment if you want to include QEMU
            # riscv64-pkgs.buildPackages.buildPackages.qemu
          ];
          nativeInputs = [
            riscv64-pkgs.buildPackages.gdb
          ];

          packages = let
            riscv64-pkgs = import nixpkgs {
              crossSystem = nixpkgs.lib.systems.examples.riscv64;
            };
          in [
            # Add other packages if needed
          ];
        };
      };
    };
}
