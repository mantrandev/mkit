# 0005 spec.md is one inventory with status on every line

Date: 2026-08-14

## Status

Accepted

## Context

`spec.md` is the only document the target user is expected to read. Their
question is always, "What have I actually built?" If working behavior and future
plans look the same, they cannot distinguish them.

## Decision

`spec.md` is the inventory for the whole product, including working and unbuilt
features. **Every line declares its own status** instead of relying on file-level
status:

```text
- [x] Email sign-in     ✅ working · 2026-08-02
- [ ] Google sign-in    ⏳ in progress · docs/active/google-login.md
- [ ] Password reset    ⬜ not started
```

The reader sees status where they read the feature and does not need to remember
which section it came from.

Only `✅ working` means the behavior was verified, and it can be applied only
after the task moves to `docs/done/`.

## Technical constraints

Every `⏳ in progress` line links to a file in `docs/active/`. Cancelled work
must lose that line so the inventory never implies that abandoned work continues.

## Alternatives considered

1. **List only working features.** The user would need to open and combine two
   documents, which they will not do.
2. **Separate "Current" and "Planned" sections.** This moves the classification
   burden to the reader, who is least able to carry it and may misremember while
   scanning.

## Tradeoffs

Each feature requires two `spec.md` updates: add `⬜ not started` after planning,
then change it to `✅ working` with a date after acceptance. mkit accepts that
extra action so users never promise customers an unbuilt feature.
