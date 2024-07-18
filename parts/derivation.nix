{
  lib,
  rustPlatform,
  self,
}:
rustPlatform.buildRustPackage {
  pname = "ukubot-rs";
  version = self.shortRev or self.dirtyShortRev or "dirty";

  cargoLock.lockFile = ../Cargo.lock;

  src = self;

  meta = with lib; {
    mainProgram = "ukubot-rs";
    description = "uku's silly discord bot";
    homepage = "https://github.com/uku3lig/ukubot-rs";
    license = licenses.mit;
    platforms = platforms.unix;
  };
}
