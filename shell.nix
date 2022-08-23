let
  overlays = let
    mozilla-overlay = let
      rev = "master";
    in fetchTarball {
      url = "https://github.com/mozilla/nixpkgs-mozilla/archive/${rev}.tar.gz";
    };
  in [
    (import mozilla-overlay)
  ];

  pkgs = let
    rev = "master";
  in import (fetchTarball {
    url = "https://github.com/nixos/nixpkgs/archive/${rev}.tar.gz";
  }) {
    inherit overlays;
  };

  rustSrc = pkgs.stdenv.mkDerivation {
    inherit (pkgs.rustc) src;
    inherit (pkgs.rustc.src) name;
    phases = ["unpackPhase" "installPhase"];
    installPhase = "cp -r src $out";
  };
in pkgs.mkShell {
  RUST_SRC_PATH = rustSrc;

  buildInputs = with pkgs; [
    cargo
    clippy
    imagemagick
    latest.rustChannels.nightly.rust
    rls
    rust-analyzer
    rustfmt
    rustracer
  ];
}
