# Como rodar o programa

Pré-requisitos
- Rust e Cargo instalados (https://www.rust-lang.org/tools/install)
- Terminal/console

Executando com Cargo (modo padrão - TUI)
- Executar em modo de desenvolvimento:
  - `cargo run`
- Executar em modo release:
  - `cargo run --release`

Executando apenas em modo CLI (sem interface TUI)
- Passar a flag `--no-ui` para o binário:
  - `cargo run -- --no-ui`
- Em release:
  - `cargo run --release -- --no-ui`

Compilar e executar o binário diretamente
- Compilar em release:
  - `cargo build --release`
- Executar (Linux/macOS):
  - `./target/release/emergency-backpack`
- Executar (Windows):
  - `target\release\emergency-backpack.exe`
- Executar o binário compilado sem interface:
  - Linux/macOS: `./target/release/emergency-backpack --no-ui`
  - Windows: `target\release\emergency-backpack.exe --no-ui`

Observações
- Por padrão o programa inicia com a interface TUI; use `--no-ui` para ativar o modo somente CLI.
