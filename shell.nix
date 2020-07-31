let
  pkgs = import ./nix/pkgs.nix;

in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      lld
      nixpkgs-fmt
      openssl
      pkg-config
      sqlite
    ];

    LD_LIBRARY_PATH="${pkgs.openssl.out}/lib";
  }
