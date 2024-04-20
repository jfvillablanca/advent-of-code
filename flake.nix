{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    devenv.url = "github:cachix/devenv";
  };

  outputs = {
    devenv,
    flake-parts,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.devenv.flakeModule
      ];
      systems = ["x86_64-linux"];
      perSystem = {
        # config,
        # self',
        # inputs',
        pkgs,
        # system,
        ...
      }: let
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
      in {
        devenv.shells.default = {
          name = "advent-of-code";
          languages = {
            rust.enable = true;
            javascript.enable = true;
            python.enable = true;
            nix.enable = true;
            go.enable = true;
          };
          packages =
            [cargo-pretty-test]
            ++ (with pkgs; [
              just
              gofumpt
              alejandra
            ])
            ++ (with pkgs.nodePackages_latest; [
              ts-node

              yarn
            ])
            ++ (with pkgs.python3.pkgs; [
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
        };
      };
    };
}
