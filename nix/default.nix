{ lib
, stdenv
, rustPlatform
, darwin
, version ? "git"
}:

rustPlatform.buildRustPackage rec {
  pname = "joshuto";
  inherit version;

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  buildInputs = [
  ]
  ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.SystemConfiguration
    darwin.apple_sdk.frameworks.Foundation
  ];

  patchPhase = ''
    sed -i 's/env!("CARGO_PKG_VERSION")/\"${version}\"/g' src/main.rs
  '';

  meta = with lib;{
    description = "Ranger-like terminal file manager written in Rust";
    homepage = "https://github.com/kamiyaa/joshuto";
    license = licenses.lgpl3Only;
    mainProgram = "joshuto";
  };
}
