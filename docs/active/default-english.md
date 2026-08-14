# Make English the default language

Started: 2026-08-14
Related `spec.md` entry: English-default language across every surface

## Desired outcome

Translate every tracked user-facing and agent-facing sentence into English,
including README, skills, templates, metadata, installer output, product status,
and durable records. English becomes the default while agents may follow a user
who explicitly uses another language.

## Completed

- [x] Defined the tracked-content acceptance boundary · 2026-08-14
- [x] Preserved durable decision filenames to avoid breaking links and history · 2026-08-14
- [x] Translated every tracked content surface · 2026-08-14
- [x] Verified that no Vietnamese prose remains in tracked file contents · 2026-08-14
- [x] Validated all skills, plugin manifests, JSON, shell syntax, source parity, and installer behavior · 2026-08-14
- [x] Installed the working tree twice in a temporary repository and verified English output without duplicated blocks · 2026-08-14
- [x] Split the change into documentation and runtime-default commits, then pushed both · 2026-08-14
- [x] Upgraded the Git marketplace and reinstalled global `mkit@mkit` at `0.1.0+codex.20260814095715` · 2026-08-14
- [x] Confirmed the marketplace snapshot matches the pushed commit and installed skills match source · 2026-08-14

## Remaining

- [ ] Start a fresh Codex thread and confirm the reinstalled plugin defaults to English

## Acceptance

Search every tracked file for Vietnamese characters and language-specific
phrases; no Vietnamese prose may remain. Install mkit into a temporary repository
and confirm its instructions, templates, workflow prompts, and terminal output
are English. Then reinstall `mkit@mkit` globally and confirm Codex reports the
new cachebuster version.

## Task decisions

- 2026-08-14: Translate tracked content but keep accepted decision filenames stable.
- 2026-08-14: Default to English and follow another language only when the user explicitly establishes it.

## Technical notes

The change touches source skills, core blocks, installed-template mirrors,
plugin manifests, marketplace metadata, installer strings, README, NOTICE,
specification, active tasks, and decisions.
