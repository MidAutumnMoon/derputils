{

  inputs = {
      nixpkgs.url =
        "github:NixOS/nixpkgs/nixos-unstable-small";
      nulib.url =
        "github:MidAutumnMoon/Nulib";
    };

  outputs = { self, nixpkgs, nulib, ... }:

    let

      lib = nulib.initWith nixpkgs;

      pkgsForSystem =
        lib.importNixpkgs {
          inherit nixpkgs;
          overlays = lib.attrValues self.overlays;
        };

    in {

      overlays.default = final: prev: {
          derputils =
            final.callPackage ./package.nix { inherit lib; };
        };

      devShells =
        lib.hexaShell pkgsForSystem [
          "rustc"
          "cargo"
          "clippy"
          "rust-analyzer"
          "clang_14"
        ];

      packages = pkgsForSystem ( p: p.derputils );

    };

}
