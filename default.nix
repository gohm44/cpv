{ lib
, stdenv
, rustPlatform
, pkg-config
}:

rustPlatform.buildRustPackage {
  pname = "cpv";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = with lib; {
    description = "A modern file copy utility with progress visualization";
    homepage = "https://github.com/gohm44/cpv";
    license = licenses.mit;
    maintainers = [ ];
    platforms = platforms.all;
  };
}
