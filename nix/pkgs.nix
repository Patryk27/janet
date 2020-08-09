# TODO revisit after flakes get stabilized
let
  nixpkgs-src = builtins.fetchTarball {
    url = "https://github.com/nixos/nixpkgs-channels/archive/c9f5211b769a2edc46037cafcdf4e15b694820d6.tar.gz";
    sha256 = "1qra1lrj3chmyrxmybaqgidp562wcss69461svwx8mzhli50xq5c";
  };

  nixpkgs-mozilla-src = import (
    builtins.fetchTarball {
      url = "https://github.com/mozilla/nixpkgs-mozilla/archive/efda5b357451dbb0431f983cca679ae3cd9b9829.tar.gz";
      sha256 = "11wqrg86g3qva67vnk81ynvqyfj0zxk83cbrf0p9hsvxiwxs8469";
    }
  );

  gitignore-src = import (
    builtins.fetchGit {
      url = "https://github.com/hercules-ci/gitignore";
      rev = "647d0821b590ee96056f4593640534542d8700e5";
    }
  );

  naersk-src = import (
    builtins.fetchGit {
      url = "https://github.com/nmattia/naersk";
      rev = "dcee40445cfe301ced2d1b11290d6a94ff3aadb9";
    }
  );

  pkgs = import nixpkgs-src {
    overlays = [
      nixpkgs-mozilla-src
    ];
  };

  gitignore = (pkgs.callPackage gitignore-src {}).gitignoreSource;

  naersk = pkgs.callPackage naersk-src {
    cargo = rust;
    rustc = rust;
  };

  rust = (
    pkgs.rustChannelOf {
      rustToolchain = ../rust-toolchain;
    }
  ).rust;

in
pkgs // {
  src = nixpkgs-src;

  inherit gitignore naersk rust;
}
