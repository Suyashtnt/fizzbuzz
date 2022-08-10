{
  description = "Build a blazingly fast fizzbuzz project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, advisory-db, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        inherit (pkgs) lib;

        craneLib = crane.lib.${system};
        src = ./.;

        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;
        };

        fizzbuzz = craneLib.buildPackage {
          inherit cargoArtifacts src;
        };
      in
      {
        checks = {
          inherit fizzbuzz;

          fizzbuzz-clippy = craneLib.cargoClippy {
            inherit cargoArtifacts src;
            cargoClippyExtraArgs = "-- --deny warnings";
          };

          fizzbuzz-fmt = craneLib.cargoFmt {
            inherit src;
          };

          fizzbuzz-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          fizzbuzz-nextest = craneLib.cargoNextest {
            inherit cargoArtifacts src;
            partitions = 1;
            partitionType = "count";
          };
        } // lib.optionalAttrs (system == "x86_64-linux") {
          fizzbuzz-coverage = craneLib.cargoTarpaulin {
            inherit cargoArtifacts src;
          };
        };

        packages.default = fizzbuzz;

        apps.default = flake-utils.lib.mkApp {
          drv = fizzbuzz;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          nativeBuildInputs = with pkgs; [
            cargo
            rustc

            rnix-lsp
            rust-analyzer
          ];
        };
      });
}
