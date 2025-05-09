{
  outputs = {nixpkgs, ...}: let
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: (forSystem system f));

    forSystem = system: f:
      f rec {
        inherit system;
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
      };
  in {
    devShells = forAllSystems ({pkgs, ...}: {
      default = pkgs.mkShell {
        buildInputs = builtins.attrValues {
          inherit
            (pkgs)
            cargo
            rustc
            rustfmt
            lld
            binaryen
            spacetimedb
            ;
        };
      };
    });
  };
}
