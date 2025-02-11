{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    # avrdude
    gcc-arm-embedded
    probe-rs
    # ravedude
    rustup
    # cargo-generate
  ];
}
