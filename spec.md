# mkit

An AI-agent harness workflow for people who cannot read code.

Every line below declares its own status. Only a `✅ working` line has been
verified with executable or observable evidence; written code alone does not count.

## Agent rules

- [x] Six-item decision gate — stop for numbers, money, personal data, irreversible deletion, third parties, or permissions — ✅ working · 2026-08-14
- [x] Five mid-work stop signals — ✅ working · 2026-08-14
- [x] Clarification loop that ends only when an acceptance script can be written — ✅ working · 2026-08-14
- [x] Completion requires steps the user can perform; test counts are not the user-facing evidence — ✅ working · 2026-08-14
- [x] Restraint rules — write the minimum and touch only what the request requires — ✅ working · 2026-08-14
- [x] Checkpoint rules — commit automatically at three moments and never push without a user request — ✅ working · 2026-08-14
- [ ] English-default language across every surface — ⏳ in progress · [English default](docs/active/default-english.md)

## Commands

- [x] `plan` — discuss without editing code — ✅ working · 2026-08-14
- [x] `implement` — build until the user can test it — ✅ working · 2026-08-14
- [x] `fix` — reproduce before changing anything — ✅ working · 2026-08-14
- [x] `continue` — recover unfinished work — ✅ working · 2026-08-14
- [ ] `grill-me` — ask one question at a time and show each option's gain and tradeoff — ⏳ in progress · [answer classification](docs/active/phan-loai-cau-tra-loi-grill-me.md)
- [x] `ha` — explain the last point a different way — ✅ working · 2026-08-14
- [x] `init` — install mkit into a target repository — ✅ working · 2026-08-14

## Documents created in a target repository

- [x] `spec.md` — one product inventory with status on every line — ✅ working · 2026-08-14
- [x] `docs/decisions/` — lasting rules with `Superseded` history — ✅ working · 2026-08-14
- [x] `docs/active/` and `docs/done/` — ✅ working · 2026-08-14

## Installation

- [x] `install.sh` — updates marked blocks without duplicating them — ✅ working · 2026-08-14
- [x] Claude Code — `/plugin marketplace add` plus `/plugin install` — ✅ working · 2026-08-14
- [x] Codex — `codex plugin marketplace add` plus `codex plugin add` — ✅ working · 2026-08-14
- [ ] Install the marketplace directly from GitHub at `mantrandev/mkit` — ⬜ not tested
- [ ] Run on Pi — ⬜ not tested
- [ ] Run with a non-technical user — ⬜ not tested

## Not started

- [ ] Hook that blocks dangerous git commands, derived from `git-guardrails-claude-code` — ⬜ not started
- [ ] `handoff` — compress the conversation into `docs/active/` — ⬜ not started
- [ ] `to-questionnaire` — route decisions outside the user's authority to another person — ⬜ not started
- [ ] Preview and screenshot loop so the agent can inspect its own result — ⬜ not started
