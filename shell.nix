{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    fontconfig
    freetype
    libxkbcommon
    wayland
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    vulkan-loader
    mesa
    udev
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath (with pkgs; [
      fontconfig
      freetype
      libxkbcommon
      wayland
      xorg.libX11
      xorg.libXcursor
      xorg.libXi
      xorg.libXrandr
      vulkan-loader
      mesa
      udev
    ])}:$LD_LIBRARY_PATH"
  '';
}
