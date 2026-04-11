#!/usr/bin/env bash
set -euo pipefail

GLUV_PROTOCOL_DIR="${GLUV_PROTOCOL_DIR:-/media/artiq/DATA/gluv-click-clack}"

if [[ ! -d "${GLUV_PROTOCOL_DIR}" ]]; then
  printf 'Error: GLUV protocol directory not found: %s\n' "${GLUV_PROTOCOL_DIR}" >&2
  printf 'Set GLUV_PROTOCOL_DIR to your gluv-click-clack path.\n' >&2
  exit 1
fi

if ! command -v npm >/dev/null 2>&1; then
  printf 'Error: npm not found in PATH. Install Node.js/npm first.\n' >&2
  exit 1
fi

if [[ ! -f "${GLUV_PROTOCOL_DIR}/package.json" ]]; then
  printf 'Error: package.json missing in %s\n' "${GLUV_PROTOCOL_DIR}" >&2
  exit 1
fi

if [[ ! -d "${GLUV_PROTOCOL_DIR}/node_modules" ]]; then
  printf 'Installing GLUV protocol dependencies in %s ...\n' "${GLUV_PROTOCOL_DIR}"
  npm --prefix "${GLUV_PROTOCOL_DIR}" install
fi

printf 'Starting GLUV MCP gateway from %s\n' "${GLUV_PROTOCOL_DIR}"
exec npm --prefix "${GLUV_PROTOCOL_DIR}" run --silent aip:mcp:gateway
