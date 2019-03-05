let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
with nixpkgs; stdenv.mkDerivation {
    name = "moz_overlay_shell";
    buildInputs = [ 
        nixpkgs.latest.rustChannels.stable.rust
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
