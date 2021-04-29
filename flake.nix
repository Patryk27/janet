{
  description = "GitLab Companion Bot";

  inputs = {
    naersk = {
      url = github:nmattia/naersk;
    };

    nixpkgs = {
      url = github:nixos/nixpkgs;
    };

    nixpkgs-mozilla = {
      url = github:mozilla/nixpkgs-mozilla;
      flake = false;
    };

    utils = {
      url = github:numtide/flake-utils;
    };
  };

  outputs = { self, naersk, nixpkgs, nixpkgs-mozilla, utils }:
    utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = (import nixpkgs) {
          system = "x86_64-linux";

          overlays = [
            (import nixpkgs-mozilla)
          ];
        };

        rust = (pkgs.rustChannelOf {
          rustToolchain = ./rust-toolchain;
          sha256 = "sha256-XiD6o5oMwLrRGxTO2vQAq5hL5kwb9YLKyxMr9Zgc76s=";
        }).rust;

        naersk' = (pkgs.callPackage naersk {
          cargo = rust;
          rustc = rust;
        });

      in
      {
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          doCheck = true;
          cargoTestOptions = args: args ++ [ "--all" ];

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
          ];
        };
      }
    );
}
