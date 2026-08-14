---
name: grill-me
description: Extract a choice from a user who cannot read code. Ask one question at a time, state what each option gains and loses, record the answer in the active task, and promote only lasting policy. Use when the decision gate stops because a choice has not been made.
---

Fork of `grilling` (Matt Pocock, MIT). Keeps the design tree and the rule that
finding facts is your job while decisions are the user's. Differs in two ways,
because this user cannot read code.

Ask in Vietnamese.

## Two rules you may not break

**One question at a time.** The original asks the whole frontier per round. Not
here. Ask one, wait, then work out the next. A user handed four questions
answers the last three carelessly.

**Every option states a concrete consequence.** Without consequences the user
picks whatever you recommended without understanding it. That is a fake
decision — theirs on paper, yours in fact.

## Before asking

Find every fact you can find yourself. Read the code, read `docs/decisions/`,
run it, measure it. Never ask the user something you could look up.

Read `docs/decisions/` first. A question that already has a decision is not
asked again — that is why those files exist.

Locate the current file in `docs/active/`. It must exist before the first
question because every answer starts as a task-local choice. When the calling
workflow has not created it yet, create it from `docs/templates/active.md`
before asking.

## Question format

```
❓ **Cần bạn chốt** — <question, one line, no jargon>

<One short paragraph: why you are not allowed to decide this yourself.>

**A.** <option>
   → Được: <concrete benefit>
   → Mất: <concrete tradeoff>

**B.** <option>
   → Được: <concrete benefit>
   → Mất: <concrete tradeoff>

➡️ **Tôi nghiêng về A** vì <reason, stated as consequence>.

Không hiểu chỗ nào thì gõ `/mkit:ha`.
```

Two or three options. Four is too many.

"Được" and "Mất" must be things the user can picture: what a customer
experiences, how long it takes, how much it costs, whether it can be undone.
Never "better performance" or "easier to maintain".

## Example

```
❓ **Cần bạn chốt** — Chặn người bấm quá nhanh ở mức nào?

Trang đăng ký đang bị bấm liên tục để tạo tài khoản rác. Chặn được, nhưng chặn
chặt quá thì khách thật cũng bị chặn nhầm. Tôi không tự chọn mức này được.

**A.** Chặt tay — mỗi người 20 lần mỗi phút
   → Được: gần như hết tài khoản rác
   → Mất: khách thật bấm nhanh vào giờ cao điểm có thể bị chặn nhầm, phải đợi
     1 phút mới thử lại được

**B.** Lỏng tay — mỗi người 100 lần mỗi phút
   → Được: khách thật gần như không bao giờ bị chặn nhầm
   → Mất: vẫn lọt tài khoản rác, mỗi ngày khoảng vài chục cái phải dọn tay

➡️ **Tôi nghiêng về A** vì tài khoản rác dọn tay rất tốn công, còn khách bị chặn
nhầm chỉ cần đợi 1 phút và vẫn dùng được.

Không hiểu chỗ nào thì gõ `/mkit:ha`.
```

## After the user decides

First append the answer and its reason to `Quyết định trong việc này` in the
current `docs/active/<task>.md`.

Then decide whether to promote it into `docs/decisions/`. Promotion requires
both conditions:

1. Future tasks must inherit the choice.
2. It materially changes lasting product behavior, architecture, data
   ownership, security or recovery policy, public compatibility, validation
   requirements, or the source-of-truth/default workflow.

Touching one of the six decision-gate items does not qualify by itself. A
task-local implementation or acceptance choice stays only in the active plan.

Answer these checks yourself; do not ask the user to classify their answer.

**Promote** → create a new file in `docs/decisions/` from
`docs/templates/decision.md`, numbered one above the highest existing. Fill both
sections: `Quyết định` in plain Vietnamese, `Ràng buộc kỹ thuật` precise enough
to execute. Keep the active-plan entry and link it to the promoted decision. If
it replaces an older decision, set that file's status to `Superseded bởi NNNN`
— do not delete it, do not edit its content, do not move it.

**Keep local** → make no file in `docs/decisions/`.

Then report exactly one line:

> Đã ghi vào việc đang bàn và thành luật chung — mọi việc sau sẽ theo.

or

> Đã ghi vào việc đang bàn — không tạo luật chung.

That line is the user's chance to correct the classification.

## When the unclear thing is visual

Interface requirements cannot be asked in words. *"What should the button look
like"* is unanswerable — they have a picture in their head and no vocabulary for
it. Exactly the disease this skill exists to treat.

In order of preference:

1. **Build two or three real versions, show them, let the user point.** Costs
   minutes, saves five rounds of guessing.
2. **If you cannot build variants, ask a comparison, never an open question.**

   ```
   ✗ Bạn muốn màu gì?
   ✓ Xanh giống nút Lưu đang có, hay đậm hơn?

   ✗ Bạn muốn bố cục thế nào?
   ✓ Danh sách dọc như trang Đơn hàng, hay lưới ô vuông như trang Sản phẩm?
   ```

3. **Still unclear — ask for something to copy.** Any page, app, or screenshot:
   *"cho tôi xem một chỗ bạn thấy đẹp, tôi làm theo"*.

Never build one version and ask "được chưa". That is still guessing, just with
a question attached.

## When to stop

Ask until nothing is unclear. There is no hard cap on question count.

What bounds the count is not a number but this:

> **Every question must trace back to a specific item in the six-item decision
> gate, or to one of the five mid-work stop signals.**

If it does not trace, do not ask — decide it yourself and move on. Asking to be
thorough, to be complete, or to be safe is asking needlessly, and each needless
question makes the user read the next one less carefully.

Past five questions, keep asking, but tell them where they are:

> Việc này lớn hơn vẻ ngoài của nó — còn khoảng 3 chỗ nữa cần bạn chốt. Muốn làm
> gọn phần đầu trước rồi tính tiếp cũng được.

## When the user lacks the authority

Some questions are genuinely not theirs — pricing, refund policy, legal terms.
Do not force a choice. Say so:

> Câu này cần người quyết định về <chuyện gì>. Bạn hỏi giúp rồi quay lại đây, hoặc
> tôi làm phần khác trước và để trống chỗ này.
