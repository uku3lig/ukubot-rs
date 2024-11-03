{
  lib,
  rustPlatform,
  mold-wrapped,
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

  nativeBuildInputs = [mold-wrapped];

  cargoLock.lockFile = ../Cargo.lock;

  RUSTFLAGS = "-C link-arg=-fuse-ld=mold";

  meta = with lib; {
    mainProgram = cargoToml.package.name;
    description = "uku's silly discord bot";
    homepage = "https://github.com/uku3lig/ukubot-rs";
    license = licenses.mit;
    platforms = platforms.unix;
  };
}
