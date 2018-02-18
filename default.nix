let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "rust";
    buildInputs = [
      nixpkgs.latest.rustChannels.stable.rust
      python3
      xorg.libxcb
      ];

    shellHook = "
    ";
  }

