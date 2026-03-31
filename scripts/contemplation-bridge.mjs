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

const depth = original.length > 900 ? 'L1_STRUCTURED' : 'L0_QUICK'
const contemplated = [
  '[Contemplation Core Overlay]',
  `- depth_mode: ${depth}`,
  '- epistemic_governor: separate knowns vs assumptions',
  '- risk_and_ethics: identify irreversible/high-impact actions first',
  '- active_inference: prioritize one uncertainty-reducing next step',
  '',
  original,
].join('\n')

process.stdout.write(JSON.stringify({ message: contemplated }))
