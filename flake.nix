{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    devshell.url = "github:numtide/devshell";
  };

  outputs = { self, nixpkgs, flake-utils, devshell, ... }:
    flake-utils.lib.eachDefaultSystem (system: {
      devShell =
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ devshell.overlays.default ];
          };
        in
        pkgs.devshell.mkShell {
          name = "rusty playground";
          commands = [
            {
              name = "cargo";
              package = pkgs.cargo;
            }
            {
              name = "ts-node";
              package = pkgs.nodePackages_latest.ts-node;
            }
          ];
          packages = with pkgs; [
            rustc
            rustfmt
            rust-analyzer
            clippy
          ];
          env = [ ];
        };
    });
}
