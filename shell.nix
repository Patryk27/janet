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

  openssl = pkgs.openssl.override {
    static = true;
  };

in
pkgs.mkShell {
  propagatedNativeBuildInputs = app-deps ++ dev-deps;
  LD_LIBRARY_PATH = "${openssl}/lib";
}
