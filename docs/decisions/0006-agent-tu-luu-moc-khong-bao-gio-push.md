# 0006 The agent creates checkpoints and never pushes by default

Date: 2026-08-14

## Status

Accepted

## Context

Stopping mid-work for a question is safe only when a rollback point exists. Git
commits provide that point, but the target user does not know git and will never
request one. The normal agent default of committing only when asked would leave
this user with no recovery point.

## Decision

The agent **commits automatically** at exactly three moments: before stopping to
ask a mid-work question, after a task passes acceptance, and before an operation
that is difficult to undo.

The agent **never pushes by default** and never force-pushes, resets, reverts, or
discards unless the user explicitly requests that operation. A local checkpoint
is an agent safety duty; sending work elsewhere belongs to the user.

When the user wants to go back, list checkpoints by **description and time**, not
by hashes or branch names.

## Technical constraints

Commit messages use a Conventional Commits prefix followed by a description in
the user's language. Describe what changed **for the user**, not the code change.

```text
feat: users can sign in with Google
fix: the Checkout button no longer gets stuck
chore: checkpoint before deciding the spam limit
```

Only the prefix is machine-facing. The rest appears verbatim in the rollback
list and must remain plain language.

## Alternatives considered

1. **Wait for the user to request a commit.** This is the normal default and is
   equivalent to having no checkpoints for this audience.
2. **Discard unfinished work before asking.** Hearing "I deleted everything I
   changed while waiting for your answer" pressures users to answer carelessly.

## Tradeoffs

Git history contains more checkpoint commits than a developer-led repository.
mkit accepts this because a dense checkpoint list is a feature for this user:
every entry is a place they can safely return to.
