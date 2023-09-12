let
  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  version = cargoToml.package.version;
in
{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "coverage-scope";
  inherit version;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
}
