{ inputs, lib, ... }: {
  imports = [
    inputs.devshell.flakeModule
  ];

  config.perSystem =
    { pkgs
    , ...
    }: let
      packages = with pkgs; [
        libxkbcommon
        libGL
        dbus

        wayland

        openssl
        libclang
        glib

        # data gathering
        pipewire.dev
        networkmanager
      ];
    in {
      config.devshells.default = {
        imports = [
          "${inputs.devshell}/extra/language/c.nix"
          # "${inputs.devshell}/extra/language/rust.nix"
        ];

        devshell = {
          name = "rshell devshell";

          packages = packages ++ (with pkgs;[ pkg-config rustPlatform.bindgenHook rust-cbindgen autoAddDriverRunpath clang ]);
          packagesFrom = with pkgs; [libclang clang libclang.lib];
        };

        env = [{
          name = "LD_LIBRARY_PATH";
          value = lib.makeLibraryPath packages;
        } {
          name = "LDFLAGS";
          eval = "-L$DEVSHELL_DIR/lib";
        } {
          name = "C_INCLUDE_PATH";
          prefix = "$DEVSHELL_DIR/include";
        } {
          name = "PKG_CONFIG_PATH";
          value = lib.concatStringsSep ":"
            ( map
              ( pkg: "${pkg.dev}/lib/pkgconfig" )
              ( packages )
            );
          #"${pkgs.openssl.dev}/lib/pkgconfig";
        } {
          name = "LIBCLANG_PATH";
          value = "${pkgs.libclang.lib}/lib";
        } {
          # some *-sys crates require additional includes
          name = "CFLAGS";
          # append in case it needs to be modified
          eval = "\"-I $DEVSHELL_DIR/include ${lib.optionalString pkgs.stdenv.isDarwin "-iframework $DEVSHELL_DIR/Library/Frameworks"}\"";
        }];

        commands = with pkgs; [
          { package = rust-toolchain; category = "rust"; }
        ];

        language.c = {
          libraries =
            packages ++
            (lib.optional pkgs.stdenv.isDarwin pkgs.libiconv);
        };
      };
    };
}
