#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WEB_DIR="${ROOT_DIR}/web"

EASY_BIND=1
BROWSER_FALLBACK=1
PASSTHROUGH_ARGS=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --require-pairing)
      EASY_BIND=0
      shift
      ;;
    --help)
      cat <<'USAGE'
Usage: ./scripts/desktop-connect.sh [--require-pairing] [--no-browser-fallback] [-- <desktop args>]

Default behavior enables local easy-bind mode:
  - gateway.host = 127.0.0.1
  - gateway.require_pairing = false

Use --require-pairing to keep 6-digit pairing/token auth enabled.
Use --no-browser-fallback to disable automatic browser fallback if desktop app exits.
USAGE
      exit 0
      ;;
    --no-browser-fallback)
      BROWSER_FALLBACK=0
      shift
      ;;
    --)
      shift
      while [[ $# -gt 0 ]]; do
        PASSTHROUGH_ARGS+=("$1")
        shift
      done
      ;;
    *)
      PASSTHROUGH_ARGS+=("$1")
      shift
      ;;
  esac
done

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

if [[ ${EASY_BIND} -eq 1 ]]; then
  printf 'Desktop easy-bind: enforcing localhost gateway + no pairing key.\n'
  "${ZEROCLAW_BIN}" config set gateway.host 127.0.0.1 >/dev/null
  "${ZEROCLAW_BIN}" config set gateway.port 9573 >/dev/null
  "${ZEROCLAW_BIN}" config set gateway.require_pairing false >/dev/null
else
  printf 'Desktop secure-bind: pairing remains enabled.\n'
fi

# Ensure no stale gateway process is holding old code/state.
pkill -f "zeroclaw gateway" >/dev/null 2>&1 || true

if [[ ! -d "${WEB_DIR}/node_modules" ]]; then
  printf 'Installing web dependencies...\n'
  npm --prefix "${WEB_DIR}" install
fi

set +e
npm --prefix "${WEB_DIR}" run desktop:fast -- "${PASSTHROUGH_ARGS[@]}"
status=$?
set -e

if [[ ${status} -ne 0 && ${BROWSER_FALLBACK} -eq 1 ]]; then
  printf 'Desktop app exited unexpectedly (status=%s). Launching browser fallback...\n' "${status}" >&2
  "${ZEROCLAW_BIN}" gateway --host 127.0.0.1 --port 9573 >/tmp/zeroclaw-gateway-fallback.log 2>&1 &
  sleep 1
  if command -v xdg-open >/dev/null 2>&1; then
    xdg-open "http://127.0.0.1:9573" >/dev/null 2>&1 || true
  fi
fi

exit ${status}
