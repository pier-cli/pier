let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  
  rust_nightly = ((nixpkgs.rustChannelOf {
    rustToolchain = ./rust-toolchain;
  }).rust.override {
    extensions = [
      "rust-src"
      "rust-analysis"
      "clippy-preview"
      "rustfmt-preview"
      "rls-preview"
    ];
  });
in
with nixpkgs; stdenv.mkDerivation {
  name = "pier_dev_shell";
  buildInputs = [ 
    rust_nightly
    rustfmt
  ];
  shellHook = ''
        alias help="
            echo 'cargo new _'
            echo 'cargo build'
            echo 'cargo check'
            echo 'cargo run'
            echo 'cargo build --release'
        ";
    '';
}
