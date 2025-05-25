{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    pkg-config
    openssl
    python3
    gcc
    clang
    #postgresql
    #libpq
    #sea-orm-cli
    xorg.libxcb.dev
    #sqlite
  ];
  shellHook = ''
    export LIBRARY_PATH="${pkgs.libpq}/lib"
    export LD_LIBRARY_PATH="${pkgs.libpq}/lib"
  '';

  # Set the RUST_SRC_PATH environment variable for tools like rust-analyzer
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
