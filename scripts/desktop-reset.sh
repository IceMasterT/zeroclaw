#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WEB_DIR="${ROOT_DIR}/web"

REBUILD_BINARY=1
REINSTALL_WEB=0
LAUNCH=1

while [[ $# -gt 0 ]]; do
  case "$1" in
    --reinstall-web)
      REINSTALL_WEB=1
      shift
      ;;
    --no-build)
      REBUILD_BINARY=0
      shift
      ;;
    --no-launch)
      LAUNCH=0
      shift
      ;;
    --help)
      cat <<'USAGE'
Usage: ./scripts/desktop-reset.sh [options]

Options:
  --reinstall-web   Remove web/node_modules and reinstall
  --no-build        Skip cargo release rebuild
  --no-launch       Do not launch desktop app after reset
  --help            Show this help

What this does:
  1) stops stale zeroclaw gateway processes
  2) cleans web dist and tauri build artifacts
  3) rebuilds web frontend
  4) rebuilds zeroclaw release binary (unless --no-build)
  5) runs desktop smoke checks
  6) launches desktop app via scripts/desktop-connect.sh (unless --no-launch)
USAGE
      exit 0
      ;;
    *)
      printf 'Unknown option: %s\n' "$1" >&2
      exit 1
      ;;
  esac
done

if ! command -v npm >/dev/null 2>&1; then
  printf 'Error: npm is required.\n' >&2
  exit 1
fi
if ! command -v cargo >/dev/null 2>&1; then
  printf 'Error: cargo is required.\n' >&2
  exit 1
fi

printf '==> Stopping stale gateway processes...\n'
pkill -f "zeroclaw gateway" >/dev/null 2>&1 || true

printf '==> Cleaning desktop/web artifacts...\n'
rm -rf "${WEB_DIR}/dist" "${WEB_DIR}/src-tauri/target"
if [[ ${REINSTALL_WEB} -eq 1 ]]; then
  rm -rf "${WEB_DIR}/node_modules"
fi

if [[ ! -d "${WEB_DIR}/node_modules" ]]; then
  printf '==> Installing web dependencies...\n'
  npm --prefix "${WEB_DIR}" install
fi

printf '==> Building web frontend...\n'
npm --prefix "${WEB_DIR}" run build

if [[ ${REBUILD_BINARY} -eq 1 ]]; then
  printf '==> Building zeroclaw release binary...\n'
  cargo build --release --manifest-path "${ROOT_DIR}/Cargo.toml"
fi

printf '==> Running desktop smoke checks...\n'
ZEROCLAW_BIN="${ROOT_DIR}/target/release/zeroclaw" "${ROOT_DIR}/scripts/desktop-smoke.sh"

if [[ ${LAUNCH} -eq 1 ]]; then
  printf '==> Launching desktop app...\n'
  ZEROCLAW_BIN="${ROOT_DIR}/target/release/zeroclaw" "${ROOT_DIR}/scripts/desktop-connect.sh"
else
  printf '==> Reset complete. Launch skipped (--no-launch).\n'
fi
