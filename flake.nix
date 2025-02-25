{
  description = "Dessins";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustPlatform = pkgs.rustPlatform;
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo rustc wasm-bindgen-cli lld pkg-config binaryen
          ];
        };

        packages.wasm = rustPlatform.buildRustPackage {
          pname = "dessins";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "bevy_nannou-0.1.0" = "sha256-C03XqdM+1lW49VqT8hwX3Lta0QIHtu5BYHfRgMzmJGQ=";
            };
          };
          cargoToml = ./crates/dessins/Cargo.toml;
          cargoBuildTarget = "wasm32-unknown-unknown";

          nativeBuildInputs = with pkgs; [ wasm-bindgen-cli lld pkg-config binaryen ];

          buildPhase = ''
            set -e
            mkdir -p target

            cargo build --profile wasm-release --bin dessins --target wasm32-unknown-unknown --verbose

            WASM_RELEASE_DIR="target/wasm32-unknown-unknown/wasm-release"
            DESSINS_WASM=$WASM_RELEASE_DIR/dessins.wasm
            DESSINS_BG_WASM=$WASM_RELEASE_DIR/dessins_bg.wasm
            OPTIMIZED_DESSINS_BG_WASM=$WASM_RELEASE_DIR/dessins_bg.optimized.wasm

            if [ ! -f $DESSINS_WASM ]; then
                echo "error: $DESSINS_WASM not found"
                exit 1
            fi

            wasm-bindgen --out-name dessins \
              --out-dir $WASM_RELEASE_DIR \
              --target web $DESSINS_WASM

            if [ ! -f $DESSINS_BG_WASM ]; then
                echo "error: $DESSINS_BG_WASM not found after wasm-bindgen"
                exit 1
            fi

            wasm-opt -Oz --output $OPTIMIZED_DESSINS_BG_WASM $DESSINS_BG_WASM
            mv $OPTIMIZED_DESSINS_BG_WASM $DESSINS_BG_WASM
          '';

          installPhase = ''
            mkdir -p $out/target
            cp -r target/wasm32-unknown-unknown/wasm-release/* $out/target/
            cp index.html $out/target/
          '';

          meta = with pkgs.lib; {
            description = "Rust WASM build with Nix";
            license = licenses.mit;
          };
        };
      }
    );
}
