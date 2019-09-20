let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
with nixpkgs; rustPlatform.buildRustPackage rec {
  name = "pier-${version}";
  version = "v0.2.0";

  src = fetchFromGitHub {
    owner = "BenSchZA";
    repo = "pier";
    rev = "${version}";
    sha256 = "1fwpl4xlsnx7dmslk2ry3459cyb8wvrz1d2j5dqlhwsxg7hyhk33";
  };
  #src = ./.;

  cargoSha256 = "1za3ly30izk6z19p7mh5xfaa62aaz9rcjz3rxs2m08va5qj3k2fy";

  # Needed so bindgen can find libclang.so
  LIBCLANG_PATH="${llvmPackages.libclang}/lib";

  buildInputs = [ 
    stdenv.cc.libc 
    nixpkgs.latest.rustChannels.stable.rust
  ];

  meta = with stdenv.lib; {
    description = "Pier";
    homepage = https://benjaminscholtz.com;
    license = licenses.mit;
    #maintainers = [ maintainers.tailhook ];
    platforms = platforms.linux;
  };
}
