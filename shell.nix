{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
    pkgs.pkg-config
    pkgs.openssl
    pkgs.cmake
    pkgs.libsodium
    pkgs.secp256k1
    pkgs.lz4
    pkgs.zlib
  ];

  shellHook = ''
    rustup toolchain install stable
    rustup default stable
    rustup component add rustfmt
  '';
}
