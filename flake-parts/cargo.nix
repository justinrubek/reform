{
  inputs,
  self,
  ...
} @ part-inputs: {
  imports = [];

  perSystem = {
    config,
    pkgs,
    lib,
    system,
    inputs',
    self',
    ...
  }: let
    devTools = [
      # rust tooling
      self'.packages.rust-toolchain
      pkgs.cargo-audit
      pkgs.cargo-udeps
      pkgs.bacon
      # version control
      pkgs.cocogitto
      # formatting
      self'.packages.treefmt
      # misc
      pkgs.wasm-bindgen-cli
      pkgs.miniserve
    ];

    # packages required for building the rust packages
    extraPackages = [
      pkgs.pkg-config
    ];
    withExtraPackages = base: base ++ extraPackages;

    craneLib = inputs.crane.lib.${system}.overrideToolchain self'.packages.rust-toolchain;

    common-build-args = rec {
      src = inputs.nix-filter.lib {
        root = ../.;
        include = [
          "crates"
          "Cargo.toml"
          "Cargo.lock"
        ];
      };

      pname = "reform";

      nativeBuildInputs = withExtraPackages [];
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeBuildInputs;
    };

    deps-only = craneLib.buildDepsOnly ({} // common-build-args);

    packages = let
      buildWasmPackage = {
        name,
        wasm-bindgen-target ? "web",
      }: let
        underscore_name = pkgs.lib.strings.replaceStrings ["-"] ["_"] name;

        cargo-derivation = craneLib.mkCargoDerivation ({
            pname = name;
            cargoArtifacts = deps-only;
            cargoExtraArgs = "-p ${name} --target wasm32-unknown-unknown";
            doCheck = false;
            doInstallCargoArtifacts = false;

            buildPhaseCargoCommand = ''
              cargoBuildLog=$(mktemp cargoBuildLogXXXX.json)
              cargoWithProfile build -p ${name} --target wasm32-unknown-unknown --message-format json-render-diagnostics > $cargoBuildLog

              mkdir -p $out
              cp -r target $out
            '';
          }
          // common-build-args);

        wasm-derivation = pkgs.stdenv.mkDerivation {
          name = "${name}-wasm-bindgen";
          buildInputs = [pkgs.wasm-bindgen-cli];
          nativeBuildInputs = [pkgs.binaryen];
          src = "";
          buildCommand = ''
            ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen \
              ${cargo-derivation}/target/wasm32-unknown-unknown/release/${underscore_name}.wasm \
              --out-dir $out \
              --target ${wasm-bindgen-target} \

            ${pkgs.binaryen}/bin/wasm-opt \
              -Oz \
              --output $out/${underscore_name}_bg.wasm \
              $out/${underscore_name}_bg.wasm
          '';
        };
      in
        wasm-derivation;
    in {
      cargo-doc = craneLib.cargoDoc ({
          cargoArtifacts = deps-only;
        }
        // common-build-args);

      injector = buildWasmPackage {
        name = "reform-injector";
      };
    };

    checks = {
      clippy = craneLib.cargoClippy ({
          cargoArtifacts = deps-only;
          cargoClippyExtraArgs = "--all-features -- --deny warnings";
        }
        // common-build-args);

      rust-fmt = craneLib.cargoFmt ({
          inherit (common-build-args) src;
        }
        // common-build-args);

      rust-tests = craneLib.cargoNextest ({
          cargoArtifacts = deps-only;
          partitions = 1;
          partitionType = "count";
        }
        // common-build-args);
    };
  in rec {
    inherit packages checks;

    devShells.default = pkgs.mkShell rec {
      packages = withExtraPackages devTools;
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;

      shellHook = ''
        ${config.pre-commit.installationScript}
      '';
    };
  };
}
