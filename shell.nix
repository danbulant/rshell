{ pkgs ? import <nixpkgs> {} }:
let
    # rust-rover things
    fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") { };
    rust-toolchain =
        fenix.default.toolchain;
in
pkgs.mkShell rec {
    buildInputs = with pkgs;[
        openssl
        pkg-config
        cmake
        zlib
        rust-toolchain

        dbus

        # common glutin
        libxkbcommon
        libGL

        # winit wayland
        wayland
    ];
    nativeBuildInputs = with pkgs; [
        pkg-config
        fontconfig
        libclang

        # data gathering
        pipewire.dev
        networkmanager
        rustPlatform.bindgenHook
    ];
    LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
    OPENSSL_DIR="${pkgs.openssl.dev}";
    OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib";
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
}
