{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    devenv.url = "github:cachix/devenv";
  };

  outputs = { self, nixpkgs, devenv, flake-utils, ... } @ inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        cargo-pretty-test = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-pretty-test";
          version = "v0.2.3";
          src = pkgs.fetchFromGitHub {
            owner = "josecelano";
            repo = "cargo-pretty-test";
            rev = "main";
            hash = "sha256-VnnhSgvNfqXLKTYe+Sef9H80+Ym7BBo7Jnfd2eMWF4U=";
          };
          cargoLock = {
            lockFile = src + "/Cargo.lock";
          };
          doCheck = false;
        };
      in
      {
        devShells.default = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            {
              languages = {
                rust.enable = true;
                javascript.enable = true;
                python.enable = true;
              };
              packages = [ cargo-pretty-test ] ++
                (with pkgs.nodePackages_latest; [
                  ts-node

                  yarn
                ]) ++
                (with pkgs.python3.pkgs; [
                  python-lsp-server
                  pylint
                  black
                  isort
                  flake8
                  flake8-length
                  pytest
                  mypy

                  icecream
                  asttokens
                  colorama
                  executing
                  pygments

                  six
                ]);
            }
          ];
        };
      });
}
