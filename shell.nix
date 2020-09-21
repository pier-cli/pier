let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };

  rust_nightly = (nixpkgs.latest.rustChannels.nightly.rust.override {
    extensions = [
      "rls-preview" "rust-src" "rust-analysis" "rustfmt-preview"
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
