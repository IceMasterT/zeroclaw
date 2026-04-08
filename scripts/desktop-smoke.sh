#!/usr/bin/env bash
set -euo pipefail

HOST="127.0.0.1"
PORT="${ZEROCLAW_SMOKE_PORT:-}"

if [[ -z "${PORT}" ]]; then
  PORT="$(python3 - <<'PY'
import socket
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.bind(("127.0.0.1", 0))
print(s.getsockname()[1])
s.close()
PY
)"
fi

if [[ -n "${ZEROCLAW_BIN:-}" ]]; then
  BIN="${ZEROCLAW_BIN}"
elif [[ -x "/home/artiq/zeroclaw/target/release/zeroclaw" ]]; then
  BIN="/home/artiq/zeroclaw/target/release/zeroclaw"
elif command -v zeroclaw >/dev/null 2>&1; then
  BIN="$(command -v zeroclaw)"
else
  printf 'Error: zeroclaw binary not found. Set ZEROCLAW_BIN or build target/release/zeroclaw.\n' >&2
  exit 1
fi

if [[ ! -x "${BIN}" ]]; then
  printf 'Error: zeroclaw binary is not executable: %s\n' "${BIN}" >&2
  exit 1
fi

printf 'Using zeroclaw binary: %s\n' "${BIN}"
printf 'Smoke target: %s:%s\n' "${HOST}" "${PORT}"

TMP_LOG="/tmp/zeroclaw-desktop-smoke.log"

cleanup() {
  if [[ -n "${GW_PID:-}" ]]; then
    kill "${GW_PID}" >/dev/null 2>&1 || true
    wait "${GW_PID}" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

pkill -f "zeroclaw gateway" >/dev/null 2>&1 || true

"${BIN}" config set gateway.host "${HOST}" >/dev/null
"${BIN}" config set gateway.port "${PORT}" >/dev/null
"${BIN}" config set gateway.require_pairing false >/dev/null

"${BIN}" gateway --host "${HOST}" --port "${PORT}" >"${TMP_LOG}" 2>&1 &
GW_PID=$!

for _ in $(seq 1 30); do
  if curl -fsS "http://${HOST}:${PORT}/health" >/dev/null 2>&1; then
    break
  fi
  sleep 0.2
done

HEALTH_JSON="$(curl -fsS "http://${HOST}:${PORT}/health")"
MEMORY_JSON="$(curl -fsS "http://${HOST}:${PORT}/api/memory?limit=20")"

python3 - <<'PY' "${HEALTH_JSON}" "${MEMORY_JSON}"
import json, sys
health = json.loads(sys.argv[1])
memory = json.loads(sys.argv[2])
entries = memory.get("entries", [])

if not isinstance(entries, list):
    raise SystemExit("Memory payload invalid: entries is not a list")

has_ws_history = any(str(e.get("key", "")).startswith("gateway_ws_history:") for e in entries)
max_content = max((len(str(e.get("content", ""))) for e in entries), default=0)

print("health_ok:", bool(health))
print("memory_entries:", len(entries))
print("memory_has_ws_history:", has_ws_history)
print("memory_max_content_len:", max_content)

if has_ws_history:
    raise SystemExit("FAILED: gateway_ws_history entries leaked into dashboard memory payload")

print("OK: desktop memory payload smoke checks passed")
PY

printf 'Gateway smoke log: %s\n' "${TMP_LOG}"
