# Upgrade Pack

This document tracks the active expansion list for performance, autonomy, and personality.

## Implementation Status (2026-03-14)

### Completed Branches
- `feature/p0-hardening` - P0 stability foundation (merged to main)
- `feature/personality-core` - P1 autonomy expansion (ready for PR)

### Feature Summary
| Feature | Command | Status |
|---------|---------|--------|
| Runtime stats | `/stats` | ✅ |
| Health check | `/health` | ✅ |
| Queue controls | `/queue drop/cancel` | ✅ |
| Queue policy | `/queue policy` | ✅ |
| Queue class limits | `/queue class set/limit` | ✅ |
| Circuit breakers | `/health providers` | ✅ |
| Incidents | `/incidents` | ✅ |
| Regression tests | `/incidents generate-tests` | ✅ |
| Persona profiles | `/persona profile` | ✅ |
| Persona traits | `/persona trait` | ✅ |
| Persona lock | `/persona lock` | ✅ |
| Persona versioning | (automatic) | ✅ |

## Personality Pack

- Trait engine, not prompt text: personality as tunable vectors (precision, warmth, boldness, brevity, curiosity, skepticism).
- Contextual persona morphing: same identity, different expression by channel/time/task pressure (ops mode vs mentor mode).
- Narrative continuity: maintain a story of collaboration so responses feel like an evolving partner, not stateless Q/A.
- Behavioral signatures: stable micro-patterns (prioritization, clarifiers, tradeoff reporting) instead of catchphrases.
- Emotional simulation layer: operational affect states (calm, urgent, forensic) tied to system conditions.
- User mirror model: learn preferred interaction cadence and abstraction level, then auto-adapt.
- Creative mode gates: explicit divergent ideation vs deterministic execution modes to avoid personality bleed into critical ops.
- Mythic memory hooks: optional symbolic rituals (startup oath, post-incident debrief style) for stronger identity coherence.

## Internal Concept Engine (ICE)

- Internal agent parliament: specialized voices vote (security, speed, correctness, UX), final answer is weighted consensus.
- Synthetic curiosity budget: reserve compute for unexpected high-upside exploration, even when not explicitly requested.
- World-model shards: maintain small domain models (repo architecture, user intent history, failure patterns) and fuse them at runtime.
- Dream cycles: offline background passes compress logs into lessons and update heuristics without changing hard policy.
- Goal debt accounting: track deferred user intents as debt and proactively repay when load drops.
- Epistemic ledger: tag each claim by evidence source (tool output, memory, inference) for auditable reasoning.
- Failure immunization: auto-generate regression tests from real incidents and feed them into planning constraints.
- Self-critique distillation: critique failed turns and compile "never again" patterns into runtime guards.

## Rollout Framework

1. Build in three tracks: Core (safe), Advanced (opt-in), Lab (experimental flags).
2. Give every autonomy feature kill-switches and metric gates.
3. Version personality features as profiles instead of hardcoded prompts.
4. Track success metrics: p95 latency, task completion reliability, rollback rate, user correction rate, trust score.

## Execution Board

Use this board as the implementation contract. Each item needs an owner role, explicit acceptance criteria, and rollback path.

### P0 (Stability Foundation)

| ID | Workstream | Owner Role | Status | Acceptance Criteria | Rollback |
| --- | --- | --- | --- | --- | --- |
| P0-1 | Runtime observability baseline (`/stats`, queue depth, in-flight, drop/cancel counters, p50/p95/p99) | Runtime + Observability | ✅ Done | Metrics visible in logs and command output; trace id attached to each turn | Disable metrics emitters and command routes behind feature flag |
| P0-2 | Adaptive concurrency + backpressure policy | Runtime Scheduler | ✅ Done | Under burst test, no queue runaway and no starvation; p95 improves vs baseline | Revert to fixed concurrency and static queue policy |
| P0-3 | Circuit breakers for providers/tools | Providers + Tools | ✅ Done | Repeated failures trip breaker; recovery probe restores traffic; no crash loops | Disable breaker hooks and return to existing retry logic |
| P0-4 | Queue classes (`interactive` vs `background`) | Runtime Scheduler | ✅ Done | Interactive tasks stay responsive under background load; fairness assertions pass | Route all classes through single queue path |
| P0-5 | Failure immunization pipeline (incident -> regression test) | QA + Runtime | ✅ Done | At least one generated regression test from real incident integrated in CI | Disable auto-generation, keep manual test path only |

### P1 (Autonomy Expansion)

| ID | Workstream | Owner Role | Status | Acceptance Criteria | Rollback |
| --- | --- | --- | --- | --- | --- |
| P1-1 | Trait engine (personality vectors) | Agent Core | ✅ Done | Profile values change behavior consistently in golden conversation tests | Pin to default profile and ignore vector overrides |
| P1-2 | Contextual persona morphing (ops, mentor, forensic) | Agent Core + Channels | ✅ Done | Mode switch responds to channel/task pressure without policy drift | Force single mode profile |
| P1-3 | User mirror model (cadence and abstraction adaptation) | Agent Core + Memory | ✅ Done | User correction rate drops over repeated sessions in A/B runs | Disable adaptive layer and use static response policy |
| P1-4 | Epistemic ledger (claim provenance tagging) | Agent Core + Observability | 🔲 Pending | Claims carry evidence source tags; audit sample passes review | Hide ledger output and bypass provenance formatter |
| P1-5 | Goal debt accounting + proactive follow-up | Agent Core + Scheduler | 🔲 Pending | Deferred intents tracked and repaid when load is low; no spam behavior | Disable proactive follow-up emitter |

### P2 (Experimental Lab)

| ID | Workstream | Owner Role | Status | Acceptance Criteria | Rollback |
| --- | --- | --- | --- | --- | --- |
| P2-1 | Internal agent parliament (weighted multi-voice voting) | Agent Core Research | 🔲 Pending | Voting improves task reliability in benchmark suite without latency blowout | Disable parliament path and use single planner |
| P2-2 | Synthetic curiosity budget | Agent Core Research | 🔲 Pending | Bounded exploration improves idea quality score in ideation tests | Set curiosity budget to 0 |
| P2-3 | World-model shards fusion | Agent Core + Memory Research | 🔲 Pending | Better long-task coherence and lower context-loss incidents | Disable shard fusion and use plain history context |
| P2-4 | Dream cycles (offline heuristic refinement) | Runtime + Data | 🔲 Pending | Offline pass generates safe candidate heuristics with manual approval gate | Disable nightly cycle runner |
| P2-5 | Mythic memory hooks | Agent Core Design | 🔲 Pending | Optional rituals improve continuity score without contaminating deterministic ops | Disable ritual profile extensions |

## Versioned Profiles

- `profile/core-v1`: deterministic, low-risk, production default.
- `profile/advanced-v1`: adaptive autonomy features enabled with guardrails.
- `profile/lab-v1`: experimental ICE features behind explicit feature flags.

## Feature Flag Contract

- `autonomy.core.*`: default on, safe path only.
- `autonomy.advanced.*`: opt-in, requires metrics gates to pass.
- `autonomy.lab.*`: default off, explicit operator enable only.
- Every flag must have a kill switch and a health metric owner.

## KPI Targets

| Metric | Baseline | Target | Gate |
| --- | --- | --- | --- |
| p95 end-to-end latency | T0 baseline | -40% from baseline | Must hold for 3 consecutive load runs |
| Task completion reliability | T0 baseline | >= 99% on standard suite | No severity-1 regressions |
| Rollback rate | T0 baseline | <= 50% of baseline | Weekly trend check |
| User correction rate | T0 baseline | -30% from baseline | A/B with mirror model enabled |
| Trust score | T0 baseline | +20% from baseline | Survey and audit composite |

## Build Order (30/60/90)

- Day 0-30: Deliver all P0 items and publish telemetry baseline.
- Day 31-60: Deliver P1 trait engine, persona morphing, mirror model, and epistemic ledger.
- Day 61-90: Run P2 experiments behind lab flags; promote only features that meet KPI gates.
