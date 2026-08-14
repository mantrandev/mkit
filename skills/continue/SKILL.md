---
name: continue
description: Show what is in progress and pick it back up. Use when the user opens a new session and does not remember where they left off.
---

Answer exactly one question: **what was I doing, and how far did I get.**

Use the user's language. Default to English when the user has not established one.

## 1. Read

Read every file in `docs/active/`. For each: task name, `Desired outcome`, how
many steps are in `Completed` versus `Remaining`, start date.

If there are none, check `spec.md` for stray `⏳ in progress` lines — if any exist,
`docs/active/` was deleted by accident. Tell the user.

## 2. Report

Plain language. Never list filenames.

> You have 2 unfinished tasks:
>
> **1. Add Google sign-in** — started August 11, 3 of 5 steps complete.
>    Remaining: get the Google key and test a real sign-in.
>
> **2. Fix the cart page** — started yesterday; the outcome is recorded but no
>    implementation has started.
>
> Which one should we continue?

One task: just ask whether to continue it.

No tasks: say so, and point at `/mkit:plan` or `/mkit:implement`.

## 3. Resume

Once they pick, read that task's whole file including
`Task decisions`, then continue from step 3 of `/mkit:implement`.

Do not re-ask anything already recorded in the file. That is why it exists.

## 4. Tasks left too long

If a task was started more than two weeks ago and untouched since, ask before
resuming:

> This task has been untouched for two weeks, and the product has changed since
> then. Do you still want it, or should we cancel it?

If dropped: move the file to `docs/done/` noting it was cancelled, and remove
the matching `⏳ in progress` line from `spec.md`. Never leave a dangling status —
a user reading `spec.md` would think it is still being worked on.
