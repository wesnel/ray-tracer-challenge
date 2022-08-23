{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      rust = pkgs.rust-bin.nightly.latest.default;

      platform = pkgs.makeRustPlatform {
        cargo = rust;
        rustc = rust;
      };

      package = platform.buildRustPackage {
        pname = "ray_tracer_challenge";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };

      image = pkgs.dockerTools.buildImage {
        name = "generate-image";

        config = {
          Cmd = [ "${package}/bin/ray_tracer_challenge" ];
        };
      };
    in {
      packages = {
        ray-tracer-challenge = package;
        generate-image = image;
        default = image;
      };

      devShell = pkgs.mkShell {
        buildInputs = [
          (rust.override {
            extensions = [ "rust-src" ];
          })
        ];
      };
    });
}
