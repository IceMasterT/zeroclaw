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

const splitSentences = (text) =>
  String(text)
    .split(/(?<=[.!?])\s+/)
    .map((s) => s.trim())
    .filter(Boolean)

const hasAny = (text, terms) => terms.some((term) => text.includes(term))

const buildOverlay = ({
  originalText,
  depthMode,
  riskLine,
  nextStep,
  assumptions = [],
  tensions = [],
  confidenceCap,
}) => {
  const lines = [
    '[Contemplation Core Overlay]',
    `- depth_mode: ${depthMode}`,
    '- epistemic_governor: separate knowns vs assumptions',
    `- risk_and_ethics: ${riskLine}`,
    `- active_inference_next: ${nextStep}`,
    Number.isFinite(confidenceCap) ? `- confidence_cap: ${Number(confidenceCap).toFixed(2)}` : null,
    assumptions.length ? `- assumptions: ${assumptions.join(' | ')}` : null,
    tensions.length ? `- debate_tensions: ${tensions.join(' | ')}` : null,
    '',
    originalText,
  ].filter(Boolean)
  return lines.join('\n')
}

if (mode === 'lite') {
  const riskWords = /(delete|drop|destroy|production|irreversible|downtime|outage|shutdown|revoke)/i
  const needsCaution = riskWords.test(original)
  const overlay = buildOverlay({
    originalText: original,
    depthMode: 'L0_QUICK',
    riskLine: needsCaution
      ? 'high-impact action detected; require rollback plan + approval'
      : 'validate assumptions before irreversible actions',
    nextStep: 'ask one uncertainty-reducing question first',
  })
  process.stdout.write(JSON.stringify({ message: overlay }))
  process.exit(0)
}

// Self-contained "full" mode (no external repo dependency)
const lower = original.toLowerCase()
const riskTerms = [
  'delete', 'drop', 'destroy', 'production', 'irreversible', 'downtime',
  'outage', 'shutdown', 'revoke', 'deploy', 'migrate', 'rollback', 'database',
]
const uncertaintyTerms = ['maybe', 'probably', 'might', 'not sure', 'guess', 'assume']
const urgencyTerms = ['now', 'immediately', 'asap', 'urgent', 'right away']

const riskHigh = hasAny(lower, riskTerms)
const uncertaintyHigh = hasAny(lower, uncertaintyTerms)
const urgencyHigh = hasAny(lower, urgencyTerms)

const depthMode = riskHigh ? 'L2_ANALYTIC' : uncertaintyHigh ? 'L1_STRUCTURED' : 'L0_QUICK'
const riskLine = riskHigh
  ? 'high-impact pathway detected; require rollback and staged execution checks'
  : 'moderate/low impact; verify constraints before execution'
const nextStep = riskHigh
  ? 'confirm rollback path + blast radius before any execution'
  : uncertaintyHigh
    ? 'ask one clarification question to reduce uncertainty'
    : 'confirm intended outcome and proceed with smallest reversible step'

const assumptions = []
if (urgencyHigh) assumptions.push('Urgency may be over-weighting risk tradeoffs')
if (!/\b(test|staging|dry\s*run|sandbox)\b/.test(lower)) {
  assumptions.push('Execution environment may be production-like')
}
const firstSentence = splitSentences(original)[0]
if (firstSentence) assumptions.push(`Primary objective: ${firstSentence}`)

const tensions = []
if (riskHigh && urgencyHigh) tensions.push('Safety vs urgency')
if (uncertaintyHigh) tensions.push('Confidence vs incomplete information')

const confidenceCap = riskHigh ? 0.65 : uncertaintyHigh ? 0.7 : 0.8

const enhanced = buildOverlay({
  originalText: original,
  depthMode,
  riskLine,
  nextStep,
  assumptions: assumptions.slice(0, 3),
  tensions: tensions.slice(0, 2),
  confidenceCap,
})

process.stdout.write(JSON.stringify({ message: enhanced }))
