---
name: plan
description: Talk through a piece of work, record task-local choices in its active plan, and promote only lasting choices — without touching code. Use when the user wants to explore before committing.
---

Talk only. When this skill ends, **no code file has changed**.

This is the user's safe command for exploring. Keep that promise exactly.

Talk to the user in Vietnamese.

## 1. Clarify the request

A loop, not a single confirmation.

Each pass: restate the request in the user's own words, then **try to write its
acceptance script** — "open X, do Y, you must see Z". Whatever you cannot write
is what is still unclear. Ask about exactly that.

Once the script is writable, the request is clear enough. Move on. Never proceed
while still guessing — every later question would be about a problem that is not
theirs.

For visual requests, do not ask in words. Run `mkit:grill-me`, section *When the
unclear thing is visual*.

## 2. Start the active plan

Create `docs/active/<task-name>.md` from `docs/templates/active.md`. Fill
`Muốn gì`, `Còn gì`, and the acceptance script in `Cách tự kiểm`. Leave
`Xong gì` empty.

This file must exist before the first decision question. It is the default home
for every answer given while planning.

## 3. Decision gate

Read `docs/decisions/` first. Check the request against the six items in the
`MKIT` block of `AGENTS.md`.

For each item touched with no existing decision, run `mkit:grill-me`, one
question at a time. Record every answer in the active plan first. `grill-me`
decides whether a lasting choice also needs a separate decision record.

## 4. Finish the plan

Update the active plan with anything settled by the decision gate.

Add the matching line to `spec.md` as `⬜ chưa làm`, or `⏳ đang làm` with a link
to the file if the user says they want it built now.

## 5. Stop

Report three things, briefly, in plain language:

- what you understood they want
- what got settled
- that `/mkit:implement` builds it for real

Do not drift into building. Typing `plan` was them saying they are not ready.
