# Classify answers from grill-me

Started: 2026-08-14
Related `spec.md` entry: `grill-me` workflow and `docs/decisions/`

## Desired outcome

When `grill-me` runs during planning, it must not write every answer into
`docs/decisions/`. Follow the `repository-harness` boundary: record task choices
in the active plan and promote only lasting choices into separate decisions.

## Completed

- [x] Compared the current `repository-harness` workflow and template · 2026-08-14
- [x] Found that `plan` called `grill-me` before creating an active plan · 2026-08-14
- [x] Changed workflow ordering and classification criteria · 2026-08-14
- [x] Installed the working tree into a temporary repository and inspected generated files · 2026-08-14

## Remaining

- [ ] Run a fresh planning session and confirm that a task-local answer creates no separate decision

## Acceptance

In a test repository, plan a feature with two open choices: one that affects only
the current task and one that changes shared policy. After answering,
`docs/active/<task>.md` must contain both answers while `docs/decisions/` gains
only the shared-policy choice.

## Task decisions

- 2026-08-14: Use the `repository-harness` boundary: task-local choices stay in the active plan, and only lasting choices are promoted into separate decisions.

## Technical notes

Check parity between `AGENTS.md` and `core/AGENTS.block.md`, between both active
templates, and in files produced by a temporary installation.
