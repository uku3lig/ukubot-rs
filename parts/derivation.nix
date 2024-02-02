{
  lib,
  rustPlatform,
  openssl,
  pkg-config,
  self,
}:
rustPlatform.buildRustPackage {
  pname = "ukubot-rs";
  version = builtins.substring 0 8 self.lastModifiedDate or "dirty";

  cargoLock.lockFile = ../Cargo.lock;

  src = self;

  buildInputs = [openssl];
  nativeBuildInputs = [pkg-config];

  meta = with lib; {
    mainProgram = "ukubot-rs";
    description = "backend for api.uku3lig.net";
    homepage = "https://github.com/uku3lig/ukubot-rs";
    license = licenses.mit;
    platforms = platforms.unix;
  };
}
