{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShell = with pkgs; mkShell {
          buildInputs = [
            cargo
            rustc
            rustfmt
            pre-commit
            pkg-config

            gobject-introspection
            atkmm
            pango
            gtk3-x11
          ];
          LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${
            lib.makeLibraryPath [
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              libxkbcommon
              xorg.libxcb
              vulkan-loader
            ]
          }";

          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}
