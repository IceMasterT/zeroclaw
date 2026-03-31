#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WEB_DIR="${ROOT_DIR}/web"
DESKTOP_BIN="${WEB_DIR}/src-tauri/target/release/zeroclaw-desktop"

if ! command -v npm >/dev/null 2>&1; then
  printf 'Error: npm is required. Install Node.js/npm first.\n' >&2
  exit 1
fi

if [[ -z "${ZEROCLAW_BIN:-}" ]]; then
  if command -v zeroclaw >/dev/null 2>&1; then
    export ZEROCLAW_BIN
    ZEROCLAW_BIN="$(command -v zeroclaw)"
  else
    printf 'Error: zeroclaw binary not found on PATH. Set ZEROCLAW_BIN manually.\n' >&2
    exit 1
  fi
fi

if [[ ! -d "${WEB_DIR}/node_modules" ]]; then
  printf 'Installing web dependencies...\n'
  npm --prefix "${WEB_DIR}" install
fi

if [[ -x "${DESKTOP_BIN}" ]]; then
  exec "${DESKTOP_BIN}" "$@"
fi

exec npm --prefix "${WEB_DIR}" run desktop:fast -- "$@"
