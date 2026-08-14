# 0004 Commands select intent, not safety level

Date: 2026-08-14

## Status

Accepted

## Context

mkit has four commands that can modify files. If each command applies a
different safety level, users can accidentally choose their own protection by
typing the wrong command.

`plan` and `implement` are the most dangerous pair. A weaker gate would force
the user to decide whether their request is already clear enough to skip
discussion, which requires knowing what they do not know.

## Decision

The decision gate runs for **every command that edits files**. No command disables it.

`plan` and `implement` differ by **where they stop**, not by safety level.
`plan` settles the work and stops; `implement` settles it and continues building.
Users can express that difference plainly as "discuss it" or "build it now."

Choosing the wrong command may add questions, but it cannot bypass the guardrail.

## Technical constraints

`plan`, `implement`, and `fix` use the same checklist before the first product-file
edit. `plan` must preserve its promise not to edit code because it is the safe
command for exploration.

## Alternatives considered

1. **One entry point that classifies intent internally.** Rejected because Tier B
   users can reliably distinguish "it is broken" and "continue the task";
   combining those paths adds confusion.
2. **Let `implement` skip grilling for speed.** This fails the exact user mkit
   serves: the person choosing speed is also least able to detect invented policy.

## Tradeoffs

Users who know exactly what they want still answer a few questions when the task
touches the six gate items. mkit accepts this because skipping would require
them to perform the risk assessment they cannot perform.
