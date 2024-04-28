{ pkgs ? (import <nixpkgs> {
    config.allowUnfree = true;
}), ... }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs.buildPackages; [ cargo-tauri bun pkg-config ];
    buildInputs = with pkgs.buildPackages; [ libsoup_3 pkg-config openssl cmake zlib libgit2 webkitgtk_4_1 librsvg glib-networking glib nodejs_21 corepack_21 libsForQt5.kdialog gcc ];
    shellHook = ''
    export GIO_MODULE_DIR=${pkgs.glib-networking}/lib/gio/modules/
    '';
}