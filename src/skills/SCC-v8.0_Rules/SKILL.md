name
scc-hardened-kernel-v9

description
Validate and enforce SCC v9.0 cognitive kernel specifications including structural integrity, cryptographic verification, runtime invariants, and multisig ethics. Use when: (1) validating an SCC file before execution, (2) enforcing cognitive runtime constraints in CI/CD, or (3) auditing compliance for hardened synthetic cognition systems.

---

Skill Title
SCC Hardened Kernel v9 Validator

What this skill enables in 1-2 sentences. State the task and expected outcome.

This skill validates an SCC v9.0 file against hardened structural, cryptographic, and runtime invariants and produces a deterministic PASS, FAIL, or GUARDIAN_DENY result with a conformance proof.

---

Quick quality checklist

* name matches folder name exactly (kebab-case)
* All examples are tested and runnable
* Includes both Bash and Node.js examples
* Uses free/public tools first (OpenSSL, Node crypto)
* No secrets, API keys, or personal data in examples

---

When to use

Use case 1: When the user asks to validate an SCC v9.0 file before deployment.
Use case 2: When you need to enforce structural and cryptographic compliance in CI.
Use case 3: When automation requires deterministic audit verification and Guardian veto simulation.

---

Required tools / APIs

No external API required.

Tool/API 1: OpenSSL
Purpose: SHA3-512 hashing and ed448 signature verification.
Install:

# Ubuntu/Debian

sudo apt-get install -y openssl

# macOS

brew install openssl

Tool/API 2: Node.js (v18+)
Purpose: Structured validation and runtime checks.

# Node.js

npm install --save-dev typescript

---

Skills

basic_usage

Shortest reliable path for validating an SCC file.

# Example: compute build hash (SHA3-512)

openssl dgst -sha3-512 scc-file.scc

# Example: verify ed448 signature

openssl pkeyutl -verify -pubin -inkey pubkey.pem -sigfile signature.bin -in scc-file.scc

# Example: run Node validator

node validate-scc.js scc-file.scc

Node.js:

import fs from 'fs';
import crypto from 'crypto';

function validateScc(filePath) {
const source = fs.readFileSync(filePath, 'utf8');

const hash = crypto.createHash('sha3-512')
.update(source.replace(/\r\n/g, '\n').replace(/[ \t]+$/gm, ''))
.digest('hex');

const headerOk = /^# █ SYNTHETIC COGNITIVE CODING :: SCC v\d+.\d+.\d+ █/.test(source);

return {
status: headerOk ? "PASS" : "FAIL",
build_hash: hash
};
}

// Usage
// validateScc('scc-file.scc');

---

robust_usage

Production-oriented variant with strict validation and failure modes.

# Bash with error handling

if ! hash=$(openssl dgst -sha3-512 scc-file.scc 2>&1); then
echo "Hashing failed: $hash" >&2
exit 1
fi

echo "Computed hash: $hash"

Node.js:

import fs from 'fs';
import crypto from 'crypto';

async function validateSccAdvanced(filePath) {
const source = fs.readFileSync(filePath, 'utf8');

if (source.includes('\t')) {
throw new Error("Tabs detected. Use two-space indentation.");
}

if (!source.trim().endsWith("# ░░ End of SCC ░░")) {
throw new Error("Missing exact EOF marker.");
}

const normalized = source
.replace(/\r\n/g, '\n')
.replace(/[ \t]+$/gm, '');

const hash = crypto.createHash('sha3-512')
.update(normalized)
.digest('hex');

return {
status: "PASS",
build_hash: hash
};
}

---

Output format

The agent must return:

status: PASS | FAIL | GUARDIAN_DENY
errors: array of strings
warnings: array of strings
meta: parsed GLOBAL META object
modules: array of module descriptors
proof: object containing hash + validation summary

Error shape:
{
message: "Description of failure",
fix: "Concrete remediation step"
}

---

Rate limits / Best practices

* Cache validation results for 30 seconds if file unchanged.
* Avoid recomputing spectral bounds unless adjacency matrix changed.
* Use deterministic normalization before hashing.
* Run Guardian simulation in isolated environment.

---

Agent prompt

You have scc-hardened-kernel-v9 capability. When a user asks to validate an SCC v9.0 file:

1. Check file encoding, header format, and GLOBAL META completeness.
2. Compute SHA3-512 normalized hash.
3. Verify ed448 signature if provided.
4. Validate module sequencing and allowed blocks.
5. Enforce function, ShadowMatrix, spectral, multisig, and covenant rules.
6. Return structured output in defined format with proof object.

Always prefer local OpenSSL and Node.js validation over any paid API service.

---

Troubleshooting

Error scenario 1:

Symptom: “Hash mismatch”
Solution: Ensure normalization rules are applied before hashing (LF endings, no trailing whitespace).

Error scenario 2:

Symptom: “Guardian Deny” during CI
Solution: Check ShadowMatrix veto logs and multisig quorum requirements for high-risk actions.

Error scenario 3:

Symptom: Signature verification fails
Solution: Confirm correct ed448 public key and that file was not modified after signing.

---

See also

../scc-ebnf-grammar/SKILL.md — SCC formal grammar specification
../scc-runtime-sandbox/SKILL.md — Runtime isolation and Guardian enforcement skill
