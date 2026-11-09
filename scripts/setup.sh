#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
# Use writable project-local caches in restricted cloud environments.
export npm_config_cache="${npm_config_cache:-$PWD/.npm}"
npm ci --no-audit --no-fund
if [[ -d .tools/cargo ]] || ! command -v rustup >/dev/null 2>&1; then
  export CARGO_HOME="$PWD/.tools/cargo"
  export RUSTUP_HOME="$PWD/.tools/rustup"
  export PATH="$CARGO_HOME/bin:$PATH"
  if [[ ! -x "$CARGO_HOME/bin/rustup" ]]; then
    installer="$(mktemp)"
    trap 'rm -f "$installer"' EXIT
    curl --proto '=https' --tlsv1.2 --fail --silent --show-error --location \
      https://sh.rustup.rs -o "$installer"
    sh "$installer" -y --no-modify-path --profile minimal --default-toolchain none
  fi
fi
rustup toolchain install 1.91.1 --profile minimal --component clippy --component rustfmt
cargo build --workspace --locked
printf '%s\n' 'Setup complete. Run npm run verify. Local Rust tools are detected automatically.'
