# Make English the default language

Started: 2026-08-14
Related `spec.md` entry: English-default language across every surface

## Desired outcome

Make distributed mkit surfaces and new records default to English while agents
follow a user who explicitly uses another language. Preserve the original body
and filename of every accepted historical decision.

## Completed

- [x] Defined the tracked-content acceptance boundary · 2026-08-14
- [x] Preserved durable decision filenames to avoid breaking links and history · 2026-08-14
- [x] Translated every mutable and distributed tracked content surface · 2026-08-14
- [x] Verified that Vietnamese prose remains only in immutable historical decisions · 2026-08-14
- [x] Validated all skills, plugin manifests, JSON, shell syntax, source parity, and installer behavior · 2026-08-14
- [x] Installed the working tree twice in a temporary repository and verified English output without duplicated blocks · 2026-08-14
- [x] Split the change into documentation and runtime-default commits, then pushed both · 2026-08-14
- [x] Upgraded the Git marketplace and reinstalled global `mkit@mkit` at `0.1.0+codex.20260814095715` · 2026-08-14
- [x] Confirmed the marketplace snapshot matches the pushed commit and installed skills match source · 2026-08-14
- [x] Restored the original bodies of accepted decisions 0001 through 0007 · 2026-08-14
- [x] Superseded the conflicting language rule with decision 0009 · 2026-08-14
- [x] Fixed gate ordering, visual ordering, fallback plan creation, and localized output · 2026-08-14
- [x] Generated Codex cachebuster `0.1.0+codex.20260814101904` and reran validation · 2026-08-14
- [x] Forward-tested English and Vietnamese planning flows in isolated repositories · 2026-08-14

## Remaining

- [ ] Upgrade and reinstall `mkit@mkit` at `0.1.0+codex.20260814101904`
- [ ] Start a fresh Codex thread and confirm the reinstalled plugin defaults to English

## Acceptance

Confirm that Vietnamese prose exists only in immutable historical decisions.
Install mkit into a temporary repository and confirm its instructions,
templates, workflow prompts, and terminal output are English. Run planning in
English and Vietnamese: the gate must run before the first edit, every active
plan must be fully populated, and user-facing output must follow the established
language. Reinstall `mkit@mkit` and confirm Codex reports a new cachebuster.

## Task decisions

- 2026-08-14: Translate tracked content but keep accepted decision filenames stable.
- 2026-08-14: Default to English and follow another language only when the user explicitly establishes it.
- 2026-08-14: Preserve accepted decision bodies; English applies to distributed surfaces and new records.

## Technical notes

The change touches source skills, core blocks, installed-template mirrors,
plugin manifests, marketplace metadata, installer strings, README, NOTICE,
specification, active tasks, and decisions.
