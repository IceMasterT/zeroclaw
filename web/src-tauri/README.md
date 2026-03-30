# ZeroClaw Desktop

This Tauri wrapper launches the local ZeroClaw gateway and opens the dashboard in a desktop window.

## Prerequisites

- Rust toolchain
- Node.js + npm
- `zeroclaw` available on your PATH (or set `ZEROCLAW_BIN`)

## Run in development

```bash
cd web
npm install
npm run desktop:dev
```

The desktop runtime starts:

- `zeroclaw gateway --host 127.0.0.1 --port 9573`
- a native window pointed at `http://127.0.0.1:9573`

## Build desktop bundles

```bash
cd web
npm run build
npm run desktop:build
```
