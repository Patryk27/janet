let
  pkgs = import ./nix/pkgs.nix;

in
import (pkgs.src + "/nixos/tests/make-test-python.nix") {
  machine = { ... }: {
    virtualisation = {
      cores = 2;
      memorySize = 512;

      lxd = {
        enable = true;
        package = pkgs.lxd;
      };
    };
  };

  testScript = ''
    machine.succeed("echo yay")
  '';
}
