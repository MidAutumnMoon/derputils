{

  inputs = {
      nixpkgs.url =
        "github:NixOS/nixpkgs/nixos-unstable-small";
      nuclage.url =
        "github:MidAutumnMoon/Nuclage";
      nulib.url =
        "github:MidAutumnMoon/Nulib";
    };

  outputs = { self, nixpkgs, nulib, nuclage, ... }:

    let

      lib = nulib.initWith nixpkgs;

      pkgsForSystem =
        lib.importNixpkgs {
          inherit nixpkgs;
          overlays = nuclage.totalOverlays;
        };

    in {

      devShells =
        lib.hexaShell pkgsForSystem [
          "rustc"
          "cargo"
          "clippy"
          "rust-analyzer"
          "latestClangStdenv.cc"
        ];

      packages = pkgsForSystem ( p:
        p.callPackage ./package.nix { inherit lib; }
      );

    };

}
