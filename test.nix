let
  pkgs = import ./nix/pkgs.nix;
  janet = (import ./default.nix) + "/bin/janet";
  tests = ./tests;

in
(
  import (pkgs.src + "/nixos/tests/make-test-python.nix") (
    { ... }: {
      machine = { ... }: {
        environment = {
          systemPackages = with pkgs; [
            (
              python3.withPackages (
                pp: with pp; [
                  jsonpickle
                  requests
                ]
              )
            )
          ];
        };

        virtualisation = {
          cores = 2;
          memorySize = 512;
        };
      };

      testScript = ''
        machine.wait_for_unit("multi-user.target")

        machine.succeed(
            "${tests}/run.py ${janet}"
        )
      '';
    }
  )
) {}
