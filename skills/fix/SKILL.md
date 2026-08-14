---
name: fix
description: Fix a bug — reproduce first, fix second, verify with the same steps used to reproduce. Use when the user reports something broken.
disable-model-invocation: true
---

The one rule that cannot be broken:

> **No reproduction, no fix.**

Fixing without seeing the bug is guessing. The user cannot tell that you guessed,
so they will believe it is done.

Talk to the user in Vietnamese.

## 1. Get the steps

Ask what they do to make it break. Ask in actions, not in technical description:

> Bạn bấm những gì để nó hỏng? Kể tôi nghe từng bước, kể cả bước nhỏ nhất.

If the steps are incomplete, keep asking until you have a runnable sequence.
Never fill the gaps by guessing.

## 2. Follow the steps yourself

Three outcomes:

**It breaks** → reproduced. Continue.

**It does not break** → do not fix. Say so plainly, then ask narrowing
questions: which machine, which browser, which account, what time of day. Record
what you tried in `docs/active/` so the next attempt does not start over.

**The app will not run** → that is a different and larger problem. Handle it
first and tell the user you are handling something else.

## 3. Decision gate

Bug fixes still pass the gate. Many fixes create new policy — how many retries,
whether old data is kept or deleted, what the error message says. If the six
items are touched, run `mkit:grill-me`.

## 4. Fix

The smallest change that removes the bug. Do not tidy surrounding code, do not
change anything unrelated.

The user cannot read a diff, so they cannot see what else you touched. Every
change outside the scope is a risk invisible to them.

## 5. Verify

Hand back **the exact steps from step 1** and say what must happen now:

> Làm lại đúng các bước lúc nãy — mở giỏ hàng, bấm Thanh toán, chọn Momo. Lần
> này phải sang được trang thanh toán thay vì đứng im.

No new script needed. They already ran this one once.

## 6. Close

Once confirmed fixed, move the file to `docs/done/` with the reproduction steps
recorded in it. If this bug returns later, that is exactly what is needed.

Only touch `spec.md` if the bug means a line currently marked `✅ chạy` was not
actually working.
