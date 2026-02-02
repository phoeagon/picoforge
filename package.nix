{
  lib,
  rustPlatform,
  buildNpmPackage,
  fetchFromGitHub,
  makeDesktopItem,

  pkg-config,
  cargo-tauri,
  wrapGAppsHook3,
  copyDesktopItems,

  glib,
  gtk3,
  openssl,
  pcsclite,
  udev,
  webkitgtk_4_1,
}:
rustPlatform.buildRustPackage (finalAttrs: {

  pname = "picoforge";
  version = "0.3.1";

  src = fetchFromGitHub {
    owner = "librekeys";
    repo = "picoforge";
    rev = "v${finalAttrs.version}";
    hash = "sha256-v3N/E80mS8KZafWJ5T6BD3+O9LL+iwNXFEThbo4Lf0Y=";
  };

  cargoRoot = "src-tauri";
  buildAndTestSubdir = finalAttrs.cargoRoot;

  cargoHash = "sha256-DB54egPebUniP/yjEZc+/AY9vOChJRBA+tqnbISmEgg=";

  postPatch = ''
    sed -i src-tauri/tauri.conf.json -e '/beforeBuildCommand/d'
  '';

  nativeBuildInputs = [
    pkg-config
    cargo-tauri.hook
    wrapGAppsHook3
    copyDesktopItems
  ];

  buildInputs = [
    glib
    gtk3
    openssl
    pcsclite
    udev
    webkitgtk_4_1
  ];

  frontendDist = buildNpmPackage {
    name = "${finalAttrs.pname}-${finalAttrs.version}-frontend-dist";
    inherit (finalAttrs) src;

    npmDepsHash = "sha256-yCs/Fvtf0m5eW/m+Revzn3q1P7wwkwinUBHLOcV06/M=";

    installPhase = ''
      runHook preInstall

      mkdir -p $out
      cp -r build/* $out

      runHook postInstall
    '';
  };

  preBuild = ''
    cp -r ${finalAttrs.frontendDist} build
  '';

  postInstall = ''
    install -Dm644 ${finalAttrs.src}/static/in.suyogtandel.picoforge.svg $out/share/icons/hicolor/scalable/apps/picoforge.svg
  '';

  desktopItems = [
    (makeDesktopItem {
      name = "in.suyogtandel.picoforge";
      desktopName = "PicoForge";
      exec = "picoforge";
      terminal = false;
      icon = "picoforge";
      comment = finalAttrs.meta.description;
      categories = [ "Utility" ];
      dbusActivatable = true;
      keywords = [ "Config" ];
      startupNotify = true;
    })
  ];

  meta = {
    changelog = "https://github.com/librekeys/picoforge/releases/tag/v${finalAttrs.version}";
    description = "Open source commissioning tool for Pico FIDO security keys";
    homepage = "https://github.com/librekeys/picoforge";
    license = lib.licenses.agpl3Only;
    mainProgram = "picoforge";
    platforms = lib.platforms.linux;
  };
})
