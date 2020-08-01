let
  pkgs = import ./nix/pkgs.nix;
  janet = (import ./default.nix) + "/bin/janet";
  tests = ./tests;

in
import (pkgs.src + "/nixos/tests/make-test-python.nix") {
  machine = { pkgs, ... }: {
    environment = {
      systemPackages = with pkgs; [
        (python3.withPackages (pp: with pp; [
          jsonpickle
        ]))
      ];
    };

    virtualisation = {
      cores = 2;
      memorySize = 512;
    };
  };

  testScript = ''
    machine.succeed(
        "python3 ${tests}/run.py ${janet}"
    )
  '';
}
