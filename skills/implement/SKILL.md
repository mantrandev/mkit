---
name: implement
description: Actually build a piece of work — settle what must be settled, then change code until the user has something they can test with their own hands. Use when the user wants it built now.
---

Build until the user has something they can see for themselves.

The decision gate runs exactly as it does in `/mkit:plan`. This command differs
in **where it stops**, not in how safe it is. Choosing `implement` over `plan`
does not grant permission to skip any question.

Use the user's language. Default to English when the user has not established one.

## 1. Clarify the request

A loop. Each pass: restate the request in the user's own words, then try to
write its acceptance script — "open X, do Y, you must see Z". Ask about whatever
you could not write.

Do not guess, build, and correct. Five wrong guesses cost the user more time
than two questions, and each one erodes their belief that you understood them.

For visual requests, run `mkit:grill-me`, section *When the unclear thing is
visual* — build variants, do not ask in words.

If `docs/active/` already holds a file for this task, read it instead of
starting over.

## 2. Decision gate

Read `docs/decisions/`. Check the six items. For anything touched without a
decision, run `mkit:grill-me`.

If nothing is touched, ask nothing — go straight to step 3.

## 3. Read the architecture

Read explicit repository-local architecture rules first, then
`docs/architecture.md` when it exists. If neither defines the changed path,
inspect source, tests, and build configuration to identify the current owner and
dependency direction before editing product code.

Preserve the architecture that exists. Do not introduce Clean Architecture or
another profile unless the repository already uses it or a lasting decision
adopts it.

For an empty repository, choose the smallest structure sufficient for the first
real feature. Do not create placeholder layers or an unfilled architecture
record. Once the implementation creates a real module, owner, dependency rule,
or cross-boundary flow, create a fully populated `docs/architecture.md` from its
template.

Treat routine file and module placement as an implementation detail. Run
`mkit:grill-me` only when viable architectures have noticeably different user
consequences or a lasting project decision is genuinely open.

## 4. Choose the work record

Keep bounded work in the current conversation when it can finish in this
session without coordination or a recovery trail.

For work that spans sessions, requires coordination, or needs recovery, create
or reuse `docs/active/<task>.md` from the template and fill every placeholder.
Mark the matching `spec.md` line `⏳ in progress` with a link to that record.

If bounded work becomes durable before you stop, create the active record then
with everything completed, remaining, and needed to resume.

## 5. Build

When an active record exists, update `Completed` as you go.

Put behavior in the module that owns it. Preserve dependency direction, use an
existing seam before creating a new one, avoid circular dependencies, keep
dependencies explicit at established boundaries, and keep each business rule in
one owner. Update `docs/architecture.md` when the work establishes or changes a
durable boundary or flow.

**Stop mid-work** on any of the five signals in the `MKIT` block. When you stop:

1. create or update the active record with what is done and what remains
2. commit a checkpoint
3. say what changed and whether the app still runs
4. say how to go back
5. then ask

Never pick "the safest option" and keep going.

## 6. Completion standard

Run the repository's applicable format, static analysis, type, test, and
architecture checks for the changed path. If no automated architecture check
exists, inspect every changed cross-boundary dependency and report that gap.

Write an acceptance script the user can run with their own hands, with what they
must see. Concrete enough to follow without asking anything further.

No test counts. No coverage. No list of changed files.

If you cannot write that script, it is not done — say so plainly and say what is
missing.

## 7. Close the task

Once the user confirms they see it working:

- if an active record exists, fill `Acceptance` and move it to `docs/done/`
- change the matching `spec.md` line to `✅ working` with today's date

If the user says it is not right, go back to **step 1**, not step 3. Most of the
time the request was understood wrongly, not implemented wrongly.
