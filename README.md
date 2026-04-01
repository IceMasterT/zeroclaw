# ZeroClaw (Aurelion Build)

Fast Rust agent runtime with desktop app, personality controls, contemplation bridge, and turbo latency mode.

Maintainer profile for this build:
- Owner: **IceMasterT**
- Operator: **artiq @ Sanctuary**
- Branch baseline: `master`
- Current binary: `zeroclaw 0.1.9`

---

## What Is Included In This Build

This repository includes all recent upgrades merged to `master`:

- Personality CLI + wizard:
  - `zeroclaw personality wizard`
  - `zeroclaw personality show`
  - `zeroclaw personality profile <name>`
  - `zeroclaw personality trait <name> <value>`
- Context-window hardening:
  - request token budgeting
  - adaptive context trimming
  - continuation safeguards
- Performance upgrades:
  - adaptive context budgets by model
  - aggressive retry path on context-overflow failures
  - tool-output compression in history
  - bounded enriched prompt assembly
  - token-aware memory retrieval + short TTL memory context cache
- Desktop app (Tauri wrapper):
  - native app shell over ZeroClaw gateway
  - startup deadlock fix (`--no-dev-server-wait`)
  - startup lag improvements + existing gateway attach
  - one-command launcher script
- Contemplation Core integration:
  - optional contemplation bridge in agent pipeline
  - `lite` and `full` bridge modes
  - low-signal skip optimization to reduce per-turn overhead
- Turbo mode:
  - lower-latency loop behavior
  - reduced loop depth for simple turns
  - optional tool-schema hiding for low-risk short requests

---

## Quick Start

### 1) Sync local repo

```bash
git pull --ff-only origin master
```

If your tree is dirty:

```bash
git stash push -u -m "wip"
git pull --ff-only origin master
git stash pop
```

### 2) Install / refresh binary

```bash
cargo install --path .
```

### 3) Run agent

```bash
zeroclaw agent -m "hello"
```

---

## Desktop App

Fastest path:

```bash
./scripts/desktop-connect.sh
```

Manual path:

```bash
cd web
npm install
npm run desktop:dev
```

Release-speed desktop runtime:

```bash
cd web
npm run desktop:fast
```

Build bundles:

```bash
cd web
npm run build
npm run desktop:build
```

---

## Core Commands (What Each Does)

| Command | What it does |
|---|---|
| `zeroclaw onboard` | Initializes workspace and base configuration. |
| `zeroclaw personality` | Configure behavior/personality profiles and traits. |
| `zeroclaw agent` | Starts interactive/one-shot AI agent loop. |
| `zeroclaw gateway` | Starts HTTP/WebSocket gateway + dashboard endpoints. |
| `zeroclaw daemon` | Runs long-lived autonomous runtime (gateway/channels/scheduler). |
| `zeroclaw service` | Manage OS background service lifecycle. |
| `zeroclaw doctor` | Runs diagnostics and health checks. |
| `zeroclaw status` | Shows runtime status overview. |
| `zeroclaw update` | Self-updates ZeroClaw. |
| `zeroclaw estop` | Emergency stop controls (engage/resume/check). |
| `zeroclaw security` | Security maintenance operations. |
| `zeroclaw cron` | Manage scheduled tasks. |
| `zeroclaw models` | Manage model catalogs and metadata. |
| `zeroclaw providers` | List supported AI providers. |
| `zeroclaw providers-quota` | Show provider quota/rate status. |
| `zeroclaw channel` | Manage channel connectors and channel runtime actions. |
| `zeroclaw integrations` | Browse/manage external integrations. |
| `zeroclaw skill` | Manage skills and skill loading. |
| `zeroclaw migrate` | Migrate from other agent runtimes. |
| `zeroclaw auth` | Manage provider auth/subscription profiles. |
| `zeroclaw hardware` | Detect/introspect connected hardware. |
| `zeroclaw peripheral` | Manage peripheral runtimes (STM32/RPi/etc). |
| `zeroclaw memory` | Inspect/clear memory and memory stats. |
| `zeroclaw config` | Show/get/set config and export schema. |
| `zeroclaw completions` | Generate shell completions. |

Use help at any level:

```bash
zeroclaw --help
zeroclaw <command> --help
```

---

## Performance Profile (Recommended)

Current tuned profile for smooth operation:

- `agent.turbo_mode = true`
- `agent.turbo_max_tool_iterations = 4`
- `agent.max_context_tokens = 100000`
- `agent.max_history_messages = 28`
- contemplation bridge in `lite` mode with low timeout

To inspect current values:

```bash
zeroclaw config get agent.turbo_mode
zeroclaw config get agent.turbo_max_tool_iterations
zeroclaw config get agent.max_context_tokens
zeroclaw config get contemplation.command
```

---

## Contemplation Core Bridge

ZeroClaw supports pre-response contemplation via command bridge.

Current default command:

```toml
[contemplation]
enabled = true
command = "CC_BRIDGE_MODE=lite node /home/artiq/zeroclaw/scripts/contemplation-bridge.mjs"
timeout_ms = 700
max_output_chars = 12000
```

Modes:

- `lite`: low-latency safety/epistemic overlay
- `full`: attempts full Contemplation Core processing (if built artifacts exist)

---

## Troubleshooting

### "Loop pattern detected"

Raise thresholds moderately:

```bash
zeroclaw config set agent.loop_detection_no_progress_threshold 6
zeroclaw config set agent.loop_detection_ping_pong_cycles 4
zeroclaw config set agent.loop_detection_failure_streak 6
```

### Slow startup / lag spikes

- Run `zeroclaw doctor`
- Ensure disk has healthy free space
- Clear heavy build artifacts when needed (`target/`, tauri targets)
- Keep skills directory lean if startup logs are noisy

### Desktop can’t connect to `127.0.0.1`

Use:

```bash
./scripts/desktop-connect.sh
```

or verify gateway manually:

```bash
zeroclaw gateway --host 127.0.0.1 --port 9573
```

---

## License

Dual licensed:
- MIT (`LICENSE-MIT`)
- Apache-2.0 (`LICENSE-APACHE`)

