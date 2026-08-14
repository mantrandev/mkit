---
name: plan
description: Talk through a piece of work without touching code. Keep bounded planning in the conversation, update spec.md with settled product truth, record lasting rationale in decisions, and create an active record only for durable planning work. Use when the user wants to explore before committing.
---

Talk only. When this skill ends, **no code file has changed**.

This is the user's safe command for exploring. Keep that promise exactly.

Use the user's language. Default to English when the user has not established one.

## 1. Clarify the request

A loop, not a single confirmation.

Each pass: restate the request in the user's own words, then **try to write its
acceptance script** — "open X, do Y, you must see Z". Whatever you cannot write
is what is still unclear. Ask about exactly that.

Once the script is writable, the request is clear enough. Move on. Never proceed
while still guessing — every later question would be about a problem that is not
theirs.

For visual requests, identify the unresolved comparison without asking an open
visual question. Do not invoke `grill-me` or edit files yet. Resolve it after
the gate check.

## 2. Run the decision gate

Read `docs/decisions/` and check the request against the six items in the `MKIT`
block of `AGENTS.md`. Determine every unanswered gate question without editing
any file or invoking `grill-me`.

The gate check is complete when every touched item is either covered by an
existing decision or listed as an open question.

## 3. Resolve open choices

For every unresolved visual comparison, run `mkit:grill-me`, section *When the
unclear thing is visual*.

For each open gate question, run `mkit:grill-me`, one
question at a time. `grill-me` routes each settled answer to `spec.md`, a lasting
decision, an existing active record, or the current conversation.

## 4. Choose the work record

Keep a bounded plan in the current conversation when it can finish here without
coordination or a recovery trail.

Create or reuse `docs/active/<task-name>.md` only when the planning work itself
is underway and must survive this conversation because it spans sessions,
requires coordination, or needs recovery. Fill every template placeholder.

The work-shape choice is complete when an active record exists for durable work
and no active record was created for bounded work.

## 5. Record the settled outcome

Update `spec.md` with settled product behavior and acceptance. Add or update the
matching feature line as `⬜ not started`; use `⏳ in progress` with an active
link only when implementation is actually underway. Preserve existing headings
and status labels. In new mkit records, keep the exact English template schema;
translate only filled prose.

Create a decision only for a consequential choice future work must inherit. If
an active planning record exists, keep it current while planning remains open;
move it to `docs/done/` when planning is complete.

## 6. Stop

Report four things, briefly, in plain language:

- what you understood they want
- what got settled
- where the settled result was recorded
- that `/mkit:implement` builds it for real

Do not drift into building. Typing `plan` was them saying they are not ready.
