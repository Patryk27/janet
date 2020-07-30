let
  pkgs = import <nixpkgs> {};

in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      lld
      pkg-config
      openssl
    ];

    LD_LIBRARY_PATH="${pkgs.openssl.out}/lib";
  }
