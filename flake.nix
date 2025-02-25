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
            cargo rustc wasm-bindgen-cli miniserve lld pkg-config
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

          nativeBuildInputs = with pkgs; [ wasm-bindgen-cli lld pkg-config];

          buildPhase = ''
            cargo build --release --bin dessins --target wasm32-unknown-unknown
            wasm-bindgen --out-name dessins \
              --out-dir target \
              --target web target/wasm32-unknown-unknown/release/dessins.wasm
          '';

          installPhase = ''
            mkdir -p $out/target
            cp -r target/wasm32-unknown-unknown/release/* $out/target/
          '';

          meta = with pkgs.lib; {
            description = "Rust WASM build with Nix";
            license = licenses.mit;
          };
        };
      }
    );
}
