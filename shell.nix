let
  pkgs = import ./nix/pkgs.nix;

  app-deps = with pkgs; [
    lld
    openssl
    pkg-config
    rust
  ];

  dev-deps = with pkgs; [
    nixpkgs-fmt
    sqlite
  ];

in
  pkgs.mkShell {
    buildInputs = app-deps ++ dev-deps;
    LD_LIBRARY_PATH="${pkgs.openssl.out}/lib";
  }
