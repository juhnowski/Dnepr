{
  description = "Полноценное окружение для разработки на Rust с поддержкой GUI";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
            "rustfmt"
          ];
        };

        # Список графических библиотек, необходимых для eframe/egui в NixOS
        runtimeLibs = with pkgs; [
          libX11
          libXcursor
          libXrandr
          libXi
          wayland
          libxkbcommon
          vulkan-loader
          libGL
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          # Пакеты, которые добавятся в ваш $PATH при активации окружения [1]
          buildInputs = with pkgs; [
            rustToolchain
            cargo-edit
            pkg-config
            openssl
          ] ++ runtimeLibs; # Добавляем графические пакеты в shell [1]

          # Переменные окружения, помогающие инструментам находить std и библиотеки [2]
          shellHook = ''
            export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"

            # Пробрасываем динамические библиотеки в рантайм, чтобы winit увидел Wayland/X11 [2]
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath runtimeLibs}"

            echo "🦀 Стек Rust и графическое окружение GUI активированы!"
            echo "🔧 Версия компилятора: $(rustc --version)"
          '';
        };
      }
    );
}
