{
  lib,
  rustPlatform,
  mold-wrapped,
  self,
}:
rustPlatform.buildRustPackage {
  pname = "ukubot-rs";
  version = self.shortRev or self.dirtyShortRev or "dirty";

  src = self;

  nativeBuildInputs = [mold-wrapped];

  cargoLock.lockFile = ../Cargo.lock;

  RUSTFLAGS = "-C link-arg=-fuse-ld=mold";

  meta = with lib; {
    mainProgram = "ukubot-rs";
    description = "uku's silly discord bot";
    homepage = "https://github.com/uku3lig/ukubot-rs";
    license = licenses.mit;
    platforms = platforms.unix;
  };
}
