{

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    nix2container.url = "github:nlewo/nix2container";
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
    mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    mission-control.url = "github:Platonic-Systems/mission-control";
    flake-root.url = "github:srid/flake-root";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, nixpkgs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devenv.flakeModule
        inputs.treefmt-nix.flakeModule
        inputs.flake-root.flakeModule
        inputs.mission-control.flakeModule
        inputs.flake-parts.flakeModules.easyOverlay
      ];
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem = { config, self', inputs', pkgs, system, final, ... }: {
        treefmt.config = {
          inherit (config.flake-root) projectRootFile;
          programs.rustfmt.enable = true;
          package = pkgs.treefmt;
          programs.nixpkgs-fmt.enable = true;
          programs.prettier.enable = true;
          programs.taplo.enable = true;
          programs.beautysh = {
            enable = true;
            indent_size = 4;
          };
        };
        mission-control.scripts = {
          fmt = {
            description = "Format source code";
            exec = config.treefmt.build.wrapper;
            category = "Dev Tools";
          };
          run = {
            description = "Run app";
            exec = "cargo run";
            category = "Dev Tools";
          };
        };

        packages.default = pkgs.callPackage ./utils/nix { };
        overlayAttrs = {
          inherit (config.packages) joshuto;
        };
        packages.joshuto = pkgs.callPackage ./utils/nix { };

        devShells.default = pkgs.mkShell {
          inputsFrom = [
            config.flake-root.devShell
            config.mission-control.devShell
            self'.devShells.my-shell
          ];
        };
        devenv.shells.my-shell = {
          languages.rust = {
            enable = true;
            version = "latest";
          };
          packages = [
          ];
          enterShell = ''
            echo $'\e[1;32mWelcom to joshuto project~\e[0m'
          '';
        };
      };
    };
}
