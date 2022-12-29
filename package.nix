{ lib, rustPlatform }:

rustPlatform.buildRustPackage {

  pname = "derputils";

  version =
    lib.readVersionCargo ./Cargo.toml;

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

}
