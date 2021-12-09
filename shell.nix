{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ unstable.rustc unstable.cargo gcc ];
  buildInputs = with pkgs; [ unstable.rustfmt unstable.clippy ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
