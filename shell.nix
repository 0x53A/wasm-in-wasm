{ pkgs ? import <nixpkgs> {} }:

let
  # Runtime libs winit/eframe dlopens for the native build.
  nativeRuntimeLibs = with pkgs; [
    libGL
    libxkbcommon
    wayland
    libx11
    libxcursor
    libxi
    libxrandr
  ];
in

pkgs.mkShell {
  packages = with pkgs; [
    cargo-component # demo-component: cargo component build --release
    trunk           # app: web build
    wasm-tools
    pkg-config
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeRuntimeLibs;

  shellHook = ''
    echo "tools: cargo-component, trunk, wasm-tools"
    echo "native GUI libs: wayland, x11, libxkbcommon, libGL"
  '';
}
