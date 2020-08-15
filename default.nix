let
  pkgs = import ./nix/pkgs.nix;

in
pkgs.naersk.buildPackage {
  src = pkgs.gitignore ./.;

  buildInputs = with pkgs; [
    openssl
    pkg-config
  ];

  doCheck = true;

  cargoTestOptions = args: args ++ [ "--workspace" ];
}
