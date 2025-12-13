{
  lib,
  rustPlatform,
  self,
}:
let
  cargoToml = lib.importTOML ../Cargo.toml;
  rev = self.shortRev or self.dirtyShortRev or "dirty";
in
rustPlatform.buildRustPackage {
  pname = cargoToml.package.name;
  version = "${cargoToml.package.version}+git.${rev}";

  src = self;

  cargoLock.lockFile = ../Cargo.lock;

  meta = with lib; {
    mainProgram = cargoToml.package.name;
    description = "uku's silly discord bot";
    homepage = "https://github.com/uku3lig/ukubot-rs";
    license = licenses.mit;
    platforms = platforms.unix;
  };
}
