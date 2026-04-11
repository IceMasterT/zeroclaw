<p align="center">
  <img src="https://raw.githubusercontent.com/IceMasterT/zeroclaw-aurelion/main/Aurelion-Sigil.png" alt="Aurelion Sigil" width="460" />
</p>

<h1 align="center">ZeroClaw Aurelion</h1>

<p align="center">
  Rust-first autonomous runtime with desktop + gateway workflows, memory tooling, and swappable providers/channels/tools.
</p>

<p align="center">
Built by students and members of the Harvard, MIT, and Sundai. Club communities. Then IceMasterT went absolutely goblin mode on it 💀 bro modded it into oblivion, like OD levels of tweaking. Straight up violated the original code 😭🙏 no cap, maxed out rizz energy fr fr
</p>

<p align="center">
  <a href="LICENSE-APACHE"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache%202.0-blue.svg" alt="License: MIT OR Apache-2.0" /></a>
  <a href="NOTICE"><img src="https://img.shields.io/github/contributors/IceMasterT/zeroclaw-aurelion?color=green" alt="Contributors" /></a>
  <a href="docs/README.md"><img src="https://img.shields.io/badge/docs-hub-0A66C2" alt="Docs Hub" /></a>
</p>

<p align="center">
  <strong>Languages:</strong>
  <a href="README.md">English</a> ·
  <a href="docs/i18n/zh-CN/README.md">zh-CN</a> ·
  <a href="docs/i18n/ja/README.md">ja</a> ·
  <a href="docs/i18n/ru/README.md">ru</a> ·
  <a href="docs/i18n/fr/README.md">fr</a> ·
  <a href="docs/i18n/vi/README.md">vi</a> ·
  <a href="docs/i18n/el/README.md">el</a>
</p>

## What This Fork Focuses On

- Desktop-first runtime and launcher scripts (`web/src-tauri`, `scripts/desktop-connect.sh`)
- Faster, safer agent loop behavior under heavy context pressure
- Memory tab crash hardening and payload filtering
- Local bind quality-of-life defaults for single-machine usage
- Operational scripts for smoke/reset/debug workflows

## Quick Start

```bash
git clone https://github.com/IceMasterT/zeroclaw-aurelion.git
cd zeroclaw-aurelion
cargo build --release --locked
./target/release/zeroclaw --help
```

Run gateway + dashboard API:

```bash
./target/release/zeroclaw gateway --host 127.0.0.1 --port 9573
```

Run desktop launcher flow:

```bash
bash scripts/desktop-connect.sh
```

Run smoke validation:

```bash
bash scripts/desktop-smoke.sh
```

## Optional: GLUV Protocol Bridge

You can run GLUV Click Clack (`/media/artiq/DATA/gluv-click-clack`) as an optional MCP server without changing the Rust build.

1. Start the bridge process:

```bash
bash scripts/gluv-protocol-bridge.sh
```

2. Add this to `~/.zeroclaw/config.toml`:

```toml
[mcp]
enabled = true

[[mcp.servers]]
name = "gluv"
transport = "stdio"
command = "bash"
args = ["/home/artiq/zeroclaw/scripts/gluv-protocol-bridge.sh"]
tool_timeout_secs = 60
```

3. Restart ZeroClaw and verify MCP tools are loaded in your session.

## Personality Commands

```bash
zeroclaw personality wizard
zeroclaw personality show
zeroclaw personality profile balanced
zeroclaw personality trait curiosity 0.70
```

## Docs

- Docs hub: `docs/README.md`
- Full TOC: `docs/SUMMARY.md`
- Operations: `docs/operations/README.md`
- Security: `docs/security/README.md`
- Troubleshooting: `docs/troubleshooting.md`

## Credit

This project builds on major work from the original ZeroClaw community and the student/member contributors from Harvard, MIT, and Sundai club ecosystems. This fork continues that foundation with additional runtime, UX, and operations changes.

## License

Dual licensed under:

- MIT (`LICENSE-MIT`)
- Apache 2.0 (`LICENSE-APACHE`)

You may choose either license.
