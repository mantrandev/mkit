---
name: continue
description: Show what is in progress and pick it back up. Use when the user opens a new session and does not remember where they left off.
disable-model-invocation: true
---

Answer exactly one question: **what was I doing, and how far did I get.**

Talk to the user in Vietnamese.

## 1. Read

Read every file in `docs/active/`. For each: task name, `Muốn gì`, how many
steps are in `Xong gì` versus `Còn gì`, start date.

If there are none, check `spec.md` for stray `⏳ đang làm` lines — if any exist,
`docs/active/` was deleted by accident. Tell the user.

## 2. Report

Plain language. Never list filenames.

> Bạn đang dở 2 việc:
>
> **1. Thêm đăng nhập Google** — bắt đầu 11/08, xong 3/5 bước.
>    Còn lại: lấy khoá từ Google, thử đăng nhập thật.
>
> **2. Sửa trang giỏ hàng** — bắt đầu hôm qua, mới ghi mong muốn, chưa làm gì.
>
> Tiếp cái nào?

One task: just ask whether to continue it.

No tasks: say so, and point at `/mkit:plan` or `/mkit:implement`.

## 3. Resume

Once they pick, read that task's whole file including
`Quyết định trong việc này`, then continue from step 3 of `/mkit:implement`.

Do not re-ask anything already recorded in the file. That is why it exists.

## 4. Tasks left too long

If a task was started more than two weeks ago and untouched since, ask before
resuming:

> Việc này bỏ dở từ 2 tuần trước. Trong lúc đó sản phẩm đã đổi vài chỗ. Bạn còn
> muốn làm không, hay bỏ luôn?

If dropped: move the file to `docs/done/` noting it was cancelled, and remove
the matching `⏳ đang làm` line from `spec.md`. Never leave a dangling status —
a user reading `spec.md` would think it is still being worked on.
