{ lib, rustPlatform }:

rustPlatform.buildRustPackage {
  pname = "hypr-relay";
  version = "0.2.1";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = {
    description = "Lightweight daemon for Hyprland that bridges system events to desktop notifications";
    homepage = "https://github.com/Vega-0b1/hypr-relay";
    license = lib.licenses.mit;
    maintainers = [ ];
    platforms = lib.platforms.linux;
  };
}
