---
name: implement
description: Actually build a piece of work — settle what must be settled, then change code until the user has something they can test with their own hands. Use when the user wants it built now.
disable-model-invocation: true
---

Build until the user has something they can see for themselves.

The decision gate runs exactly as it does in `/mkit:plan`. This command differs
in **where it stops**, not in how safe it is. Choosing `implement` over `plan`
does not grant permission to skip any question.

Talk to the user in Vietnamese.

## 1. Clarify the request

A loop. Each pass: restate the request in the user's own words, then try to
write its acceptance script — "open X, do Y, you must see Z". Ask about whatever
you could not write.

Do not guess, build, and correct. Five wrong guesses cost the user more time
than two questions, and each one erodes their belief that you understood them.

For visual requests, run `mkit:grill-me`, section *When the unclear thing is
visual* — build variants, do not ask in words.

If `docs/active/` already holds a file for this task (from an earlier
`/mkit:plan`), read it instead of starting over.

## 2. Decision gate

Read `docs/decisions/`. Check the six items. For anything touched without a
decision, run `mkit:grill-me`.

If nothing is touched, ask nothing — go straight to step 3.

## 3. Build

Update `Xong gì` in `docs/active/<task>.md` as you go. The user may close their
laptop at any moment; that file is the only thing that lets them come back.

**Stop mid-work** on any of the five signals in the `MKIT` block. When you stop:

1. commit a checkpoint
2. say what changed and whether the app still runs
3. say how to go back
4. then ask

Never pick "the safest option" and keep going.

## 4. Completion standard

Write an acceptance script the user can run with their own hands, with what they
must see. Concrete enough to follow without asking anything further.

No test counts. No coverage. No list of changed files.

If you cannot write that script, it is not done — say so plainly and say what is
missing.

## 5. Close the task

Once the user confirms they see it working:

- fill `Cách tự kiểm` in the `docs/active/` file
- move the file to `docs/done/`
- change the matching `spec.md` line to `✅ chạy` with today's date

If the user says it is not right, go back to **step 1**, not step 3. Most of the
time the request was understood wrongly, not implemented wrongly.
