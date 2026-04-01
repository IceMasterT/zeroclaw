#!/usr/bin/env bash
set -euo pipefail

if ! command -v zeroclaw >/dev/null 2>&1; then
  printf 'Error: zeroclaw not found on PATH.\n' >&2
  exit 1
fi

zeroclaw config set gateway.require_pairing true >/dev/null
printf 'gateway.require_pairing=true (6-digit key / pairing required).\n'
