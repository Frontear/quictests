{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  pname = "quictests";
  version = "0.1.0";

  src = with lib.fileset; toSource {
    root = ../../.;
    fileset = unions [
      ../../src
      ../../Cargo.lock
      ../../Cargo.toml
    ];
  };

  cargoLock.lockFile = ../../Cargo.lock;

  meta = with lib; {
  };
}
