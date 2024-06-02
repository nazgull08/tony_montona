{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
    pkgs.openssl
    pkgs.libclang
    pkgs.cmake
    pkgs.python3
  ];

  shellHook = ''
    echo "Setting up environment for TON smart contracts on Rust..."
    rustup default stable
  '';
}
