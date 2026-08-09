{
  description = "Полноценное окружение для разработки на Rust";

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

        # Настройка стабильной версии Rust с необходимыми компонентами
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"       # Исходники std-библиотеки для rust-analyzer
            "rust-analyzer"  # Языковой сервер для интеграции с Zed/VS Code
            "clippy"         # Линтер
            "rustfmt"        # Форматтер кода
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          # Пакеты, которые добавятся в ваш $PATH при активации окружения
          buildInputs = with pkgs; [
            rustToolchain
            cargo-edit       # Cargo-плагин для удобного добавления зависимостей
            pkg-config       # Полезно для компиляции C-зависимостей
            openssl          # Понадобится для большинства сетевых крейтов
          ];

          # Переменные окружения, помогающие инструментам находить std и библиотеки
          shellHook = ''
            export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"
            echo "🦀 Стек Rust активирован! Версия: $(rustc --version)"
          '';
        };
      }
    );
}
