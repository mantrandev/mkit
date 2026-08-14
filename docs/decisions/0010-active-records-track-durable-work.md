# 0010 Active records track durable work

Date: 2026-08-14

## Status

Accepted

## Context

Decision 0007 required every planning answer to start in `docs/active/`. That
made a discussion look like implementation work and created durable files for
bounded choices that could be settled in one conversation.

`docs/active/` must describe work that is actually underway. Product truth and
the reason behind lasting choices have different homes.

## Decision

Keep bounded planning and unsettled choices in the current conversation.

Route a settled answer by meaning:

- update `spec.md` when it defines current product behavior or acceptance
- create `docs/decisions/NNNN-*.md` when future work must inherit a consequential
  choice and its reason
- record task-local execution or recovery detail in `docs/active/<task>.md` only
  when that active record already represents durable work in progress

Create an active record only when work is underway and must survive the current
conversation because it spans sessions, requires coordination, or needs a
recovery trail. A bounded task may remain ephemeral. Every active record is
work in progress, but not every in-progress task needs an active record.

One settled answer may update both `spec.md` and a decision: the specification
states what is true now, while the decision preserves why that lasting choice
was made.

This decision supersedes 0007.

## Technical constraints

The decision gate and `grill-me` must not create an active record merely to ask
a question or store its answer.

`plan` must classify the work shape. Keep bounded planning in the conversation.
Create or reuse an active record only for durable planning work. When planning
finishes, update `spec.md` and any warranted decision, then close an existing
active record.

`implement` and `fix` must use the same work-shape test. If bounded work becomes
durable before the agent stops, create an active record with the outcome,
completed work, remaining work, acceptance steps, and task-local recovery
choices.

`grill-me` must report which durable source was updated. Task-local details may
remain ephemeral when no active record is warranted.

## Alternatives considered

1. **Create an active record for every question.** This guarantees a written
   trace but fills the active list with work that is not being performed.
2. **Put every settled answer in a decision.** This makes choices easy to find
   but mixes lasting policy with routine acceptance and implementation details.
3. **Put every settled answer only in `spec.md`.** This keeps current behavior
   visible but loses the rationale and tradeoffs future work may need.

## Tradeoffs

Bounded choices may exist only in conversation and, after implementation, Git
history. mkit accepts that lighter trace so `docs/active/` remains a truthful
list of resumable work and `docs/decisions/` remains a focused history of
consequential choices.
