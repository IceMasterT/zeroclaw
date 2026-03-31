#!/usr/bin/env node

const chunks = []
for await (const chunk of process.stdin) chunks.push(chunk)
const raw = Buffer.concat(chunks).toString('utf8').trim()

if (!raw) {
  process.stdout.write(JSON.stringify({ message: '' }))
  process.exit(0)
}

let payload
try {
  payload = JSON.parse(raw)
} catch {
  process.stdout.write(JSON.stringify({ message: raw }))
  process.exit(0)
}

const original = String(payload.message ?? '').trim()
if (!original) {
  process.stdout.write(JSON.stringify({ message: '' }))
  process.exit(0)
}

const mode = String(process.env.CC_BRIDGE_MODE ?? 'lite').toLowerCase()

if (mode === 'lite') {
  const riskWords = /(delete|drop|destroy|production|irreversible|downtime|outage|shutdown|revoke)/i
  const needsCaution = riskWords.test(original)
  const lines = [
    '[Contemplation Core Overlay]',
    '- depth_mode: L0_QUICK',
    '- epistemic_governor: separate knowns vs assumptions',
    needsCaution
      ? '- risk_and_ethics: high-impact action detected; require rollback plan + approval'
      : '- risk_and_ethics: validate assumptions before irreversible actions',
    '- active_inference_next: ask one uncertainty-reducing question first',
    '',
    original,
  ]
  process.stdout.write(JSON.stringify({ message: lines.join('\n') }))
  process.exit(0)
}

let ContemplationCore
try {
  ;({ ContemplationCore } = await import('../Contemplation Core/dist/src/index.js'))
} catch {
  // Dist not available: graceful passthrough.
  process.stdout.write(JSON.stringify({ message: original }))
  process.exit(0)
}

const core = new ContemplationCore()
const sessionId = String(payload.session_id ?? payload.sessionId ?? 'zeroclaw-session')
const input = { sessionId, message: original }

let enhanced = original
try {
  const result = core.process(input)
  const keyAssumptions = (result.assumptions ?? []).slice(0, 3)
  const topTensions = (result.debate?.unresolvedTensions ?? []).slice(0, 2)
  const nextProbe = result.activeInference?.recommendedAction ?? null
  const confidenceCap = result.epistemic?.confidenceCap

  const lines = [
    '[Contemplation Core Overlay]',
    `- depth_mode: ${result.activation?.depthMode ?? 'L0_QUICK'}`,
    nextProbe ? `- active_inference_next: ${nextProbe}` : null,
    Number.isFinite(confidenceCap)
      ? `- confidence_cap: ${Number(confidenceCap).toFixed(2)}`
      : null,
    keyAssumptions.length ? `- assumptions: ${keyAssumptions.join(' | ')}` : null,
    topTensions.length ? `- debate_tensions: ${topTensions.join(' | ')}` : null,
    '',
    original,
  ].filter(Boolean)

  enhanced = lines.join('\n')
} catch {
  enhanced = original
}

process.stdout.write(JSON.stringify({ message: enhanced }))
