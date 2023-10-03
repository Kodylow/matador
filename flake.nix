{
  description = "Matador: A Bitcoin Payments Reverse Proxy for any API";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";

    flakebox = {
      url = "github:rustshop/flakebox?rev=e1cb8e13203bd9c8ea35ff86cda7ceaba473940f";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flakebox, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        flakeboxLib = flakebox.lib.${system} { };

        rustSrc = flakeboxLib.filter.filterSubdirs {
          root = builtins.path {
            name = "matador";
            path = ./.;
          };
          dirs = [
            "Cargo.toml"
            "Cargo.lock"
            ".cargo"
            "src"
          ];
        };

        multibuild =
          (flakeboxLib.craneMultiBuild { }) (craneLib':
            let
              craneLib = (craneLib'.overrideArgs {
                pname = "flexbox-multibuild";
                src = rustSrc;
              });
            in
            rec {
              workspaceDeps = craneLib.buildWorkspaceDepsOnly { };
              workspaceBuild = craneLib.buildWorkspace {
                cargoArtifacts = workspaceDeps;
              };
              matador = craneLib.buildPackage { };
            });
       in
       {
        legacyPackages = multibuild;
        devShells = {
          default = flakeboxLib.mkDevShell {
            packages = [ ];
          };
        };
      });
}