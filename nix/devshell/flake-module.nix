{ inputs, lib, ... }: {
  imports = [
    inputs.devshell.flakeModule
  ];

  config.perSystem =
    { pkgs
    , ...
    }: {
      config.devshells.default = {
        imports = [
          "${inputs.devshell}/extra/language/c.nix"
          # "${inputs.devshell}/extra/language/rust.nix"
        ];

        devshell = {
          name = "rshell devshell";

          packages = with pkgs; [
            libxkbcommon
            libGL
            dbus

            wayland

            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libX11
            openssl
            pkg-config
          ];
        };

        commands = with pkgs; [
          { package = rust-toolchain; category = "rust"; }
        ];

        language.c = {
          libraries =
            (with pkgs; [
              libxkbcommon
              libGL
              dbus

              wayland

              xorg.libXcursor
              xorg.libXrandr
              xorg.libXi
              xorg.libX11
              openssl
              pkg-config
            ]) ++
            (lib.optional pkgs.stdenv.isDarwin pkgs.libiconv);
        };
      };
    };
}
