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

  # TODO extract to `tests/deps.nix` maybe?
  test-deps = with pkgs; [
    (python3.withPackages (pp: with pp; [
      jsonpickle
      requests
    ]))
  ];

in
  pkgs.mkShell {
    buildInputs = app-deps ++ dev-deps ++ test-deps;
    LD_LIBRARY_PATH="${pkgs.openssl.out}/lib";
  }
