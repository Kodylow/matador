{
  description = "Matador: A Bitcoin Payments Reverse Proxy for any API";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flakebox = {
      url =
        "github:rustshop/flakebox?rev=389987aadbc291d0dff842b898b643d5e6a8d140";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flakebox, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        isDarwin = system == "x86_64-darwin" || system == "aarch64-darwin";
        darwinFrameworks = with pkgs.darwin.apple_sdk.frameworks; [
          SystemConfiguration
          CoreFoundation
          Security
        ];

        flakeboxLib = flakebox.lib.${system} { };

        rustSrc = flakeboxLib.filter.filterSubdirs {
          root = builtins.path {
            name = "matador";
            path = ./.;
          };
          dirs = [ "Cargo.toml" "Cargo.lock" ".cargo" "src" ];
        };

        multibuild = (flakeboxLib.craneMultiBuild { }) (craneLib':
          let
            craneLib = (craneLib'.overrideArgs {
              pname = "flexbox-multibuild";
              src = rustSrc;
              buildInputs = [ pkgs.openssl ]
                ++ (if isDarwin then darwinFrameworks else [ ]);
              propagatedBuildInputs =
                if isDarwin then darwinFrameworks else [ ];
              nativeBuildInputs = [ pkgs.pkg-config ];
              PKG_CONFIG_PATH = if isDarwin then
                "${
                  pkgs.lib.getDev
                  pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
                }/lib/pkgconfig"
              else
                null;
            });
          in rec {
            workspaceDeps = craneLib.buildWorkspaceDepsOnly { };
            workspaceBuild =
              craneLib.buildWorkspace { cargoArtifacts = workspaceDeps; };
            matador = craneLib.buildPackage { };
          });
      in {
        legacyPackages = multibuild;
        devShells = {
          default = flakeboxLib.mkDevShell {
            buildInputs = [ pkgs.openssl ];
            nativeBuildInputs = [ pkgs.pkg-config ];
            packages = [ ];
          };
        };
      });
}
