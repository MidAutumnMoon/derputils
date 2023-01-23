with import <nixpkgs> {};

rustPlatform.buildRustPackage {

  pname = "derputils";

  version = "dev";

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

}
