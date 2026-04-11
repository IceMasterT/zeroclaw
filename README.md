<p align="center">
  <img src="https://raw.githubusercontent.com/IceMasterT/zeroclaw-aurelion/main/Aurelion-Sigil.png" alt="Aurelion Sigil" width="460" />
</p>

<h1 align="center">ZeroClaw Aurelion</h1>

<p align="center">
  Rust-first autonomous runtime with desktop + gateway workflows, configurable memory backends, and optional protocol bridges.
</p>

<p align="center">
Built by students and members of the Harvard, MIT, and Sundai. Club communities. Then IceMasterT went absolutely goblin mode on it 💀 bro modded it into oblivion, like OD levels of tweaking. Straight up violated the original code 😭🙏 no cap, maxed out rizz energy fr fr
</p>

## What This Build Adds

- Desktop-first launcher flow with local easy-bind defaults
- Personality wizard and trait/profile CLI commands
- Memory tab stability hardening + payload safety checks
- Operations scripts for smoke, reset, and secure desktop rebinding
- Optional MCP bridge for GLUV protocol tooling
- Optional MemPalace memory backend bridge (`memory.backend = "mempalace"`)

## Step-by-Step Startup Procedures

### 1) First-time setup

```bash
git clone https://github.com/IceMasterT/zeroclaw-aurelion.git
cd zeroclaw-aurelion
cargo build --release --locked
./target/release/zeroclaw --help
```

### 2) Start in CLI agent mode

```bash
./target/release/zeroclaw agent
```

### 3) Start gateway/dashboard mode

```bash
./target/release/zeroclaw gateway --host 127.0.0.1 --port 9573
```

### 4) Start desktop mode (recommended)

```bash
bash scripts/desktop-connect.sh
```

Default desktop behavior:
- forces `gateway.host = 127.0.0.1`
- forces `gateway.port = 9573`
- sets `gateway.require_pairing = false` for local use
- kills stale gateway process before launch
- browser fallback if native desktop exits unexpectedly

Secure desktop mode (keep pairing enabled):

```bash
bash scripts/desktop-connect.sh --require-pairing
```

### 5) Verify runtime health

```bash
bash scripts/desktop-smoke.sh
```

This checks gateway health plus memory payload safety (including internal ws-history filtering).

## New Features and How to Use Them

### Personality control commands

```bash
zeroclaw personality wizard
zeroclaw personality show
zeroclaw personality profile balanced
zeroclaw personality trait curiosity 0.70
```

### Desktop recovery/reset workflow

If desktop gets unstable, run:

```bash
bash scripts/desktop-reset.sh
```

Useful flags:
- `--reinstall-web` reinstall web dependencies from scratch
- `--no-build` skip release rebuild
- `--no-launch` reset + smoke only

### Optional GLUV protocol bridge (MCP)

Start GLUV MCP gateway bridge:

```bash
bash scripts/gluv-protocol-bridge.sh
```

Add to `~/.zeroclaw/config.toml`:

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

### Optional MemPalace memory backend

ZeroClaw now supports `mempalace` as a memory backend bridge with SQLite fallback.

1. Install MemPalace in a dedicated venv:

```bash
python3 -m venv ~/.zeroclaw/venvs/mempalace
~/.zeroclaw/venvs/mempalace/bin/python -m pip install -U pip
~/.zeroclaw/venvs/mempalace/bin/python -m pip install -e /home/artiq/zeroclaw/mempalace-main
```

2. Enable backend:

```bash
zeroclaw config set memory.backend mempalace
```

3. Optional overrides:

```bash
export ZEROCLAW_MEMPALACE_PYTHON="$HOME/.zeroclaw/venvs/mempalace/bin/python"
export ZEROCLAW_MEMPALACE_PATH="/custom/path/to/palace"
```

4. Confirm:

```bash
zeroclaw status
```

Look for: `Memory: mempalace (auto-save: on)`.

## Common Operations

Re-enable strict desktop pairing defaults:

```bash
bash scripts/desktop-secure-bind.sh
```

Check current status:

```bash
zeroclaw status
```

## Troubleshooting Quick Notes

- RustEmbed `web/dist` missing error: ensure repo includes `web/dist/.gitkeep` and rebuild.
- Desktop blank/crash: run `bash scripts/desktop-reset.sh`.
- Port collisions during smoke: `desktop-smoke.sh` now auto-selects a free localhost port.
- MemPalace bridge unavailable: SQLite memory still works; fix Python env and retry.

## License

Dual licensed under:

- MIT (`LICENSE-MIT`)
- Apache 2.0 (`LICENSE-APACHE`)

You may choose either license.
