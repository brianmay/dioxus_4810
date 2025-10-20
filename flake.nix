{
  description = "Test Dioxus Project";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  inputs.nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.devenv.url = "github:cachix/devenv";
  inputs.crane.url = "github:ipetkov/crane";
  inputs.flockenzeit.url = "github:balsoft/flockenzeit";

  outputs =
    inputs@{
      self,
      nixpkgs,
      nixpkgs-unstable,
      flake-utils,
      rust-overlay,
      devenv,
      crane,
      flockenzeit,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        pkgs-unstable = nixpkgs-unstable.legacyPackages.${system};
        wasm-bindgen-cli = pkgs-unstable.wasm-bindgen-cli_0_2_104;

        dioxus-cli = pkgs.callPackage ./nix/dioxus-cli.nix { };

        rustPlatform = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
          extensions = [ "rust-src" ];
        };
        # craneLib = (crane.mkLib pkgs).overrideToolchain rustPlatform;

        nodejs = pkgs.nodejs_20;

        port = 8080;

        devShell = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            {
              packages = [
                rustPlatform
                pkgs-unstable.rust-analyzer
                wasm-bindgen-cli
                pkgs.binaryen
                nodejs
                pkgs.cargo-watch
                pkgs.sqlx-cli
                pkgs.openssl
                pkgs.prefetch-npm-deps
                dioxus-cli
                pkgs.diesel-cli
                pkgs.diesel-cli-ext
                pkgs.watchman
              ];
              enterShell = ''
                export PORT="${toString port}"
                export BASE_URL="http://localhost:$PORT/"
                # export DIOXUS_PUBLIC_PATH="$PWD/target/dx/my_dioxus_app/debug/web/"
              '';
            }
          ];
        };
      in
      {
        packages = {
          devenv-up = devShell.config.procfileScript;
        };
        devShells.default = devShell;
      }
    )
    // {
      nixosModules.default = import ./nix/module.nix { inherit self; };
    };
}
