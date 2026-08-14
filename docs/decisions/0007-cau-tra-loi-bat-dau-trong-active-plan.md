# 0007 Answers start in the active plan

Date: 2026-08-14

## Status

Accepted

## Context

The `plan` workflow called `grill-me` before creating a file in `docs/active/`.
With no task-local destination, an answer could become a separate file in
`docs/decisions/` even when it applied only to the current task.

## Decision

Follow the boundary used by `repository-harness`: every answer given during
planning starts in the task's active plan. Promote it to a separate decision
only when future tasks must inherit it and it materially changes product
behavior, architecture, data ownership, security or recovery policy, public
compatibility, validation requirements, or the source-of-truth/default workflow.

Touching one of the six gate items determines whether the agent must ask. It
does not automatically turn the answer into a lasting rule.

## Technical constraints

`plan` creates `docs/active/<task>.md` before calling `grill-me`.

`grill-me` records every answer under `Task decisions` first. It creates a file
in `docs/decisions/` only when both the future-effect and lasting-change tests pass.

## Alternatives considered

1. **Create one decision file for every answer.** This is easy to search but
   mixes lasting rules with details of individual tasks.
2. **Ask only whether another task next week would reuse the answer.** This is
   too broad. A feature's acceptance criteria may remain true next week without
   deserving project-wide authority.

## Tradeoffs

A promoted choice appears both in the active plan and in its separate decision
record. mkit accepts this deliberate duplication so the plan retains its full
context while future tasks can find lasting rules without reading old plans.
