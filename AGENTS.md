# Project Rules

## Repo này là chính mkit

Hai chỗ khối `MKIT` bên dưới nói không đúng với repo này. Theo mục `Precedence`
của chính khối đó, luật ở đây thắng:

- **Người dùng repo này đọc được code.** Khối `MKIT` mở đầu bằng câu ngược lại —
  câu đó đúng với repo *cài* mkit, không đúng với repo *làm ra* mkit. Nói chuyện
  ở đây bằng ngôn ngữ kỹ thuật bình thường.
- **Workflow nằm ở `skills/<tên>/SKILL.md`**, không phải `.mkit/workflows/`.
  Thư mục đó là sinh phẩm của `install.sh` và bị `.gitignore` — giữ bản sao trong
  repo nguồn là đúng thứ mkit cấm.

Mọi luật còn lại của khối `MKIT` áp bình thường, kể cả cổng chốt.

<!-- MKIT:BEGIN -->
## mkit

The user of this repository cannot read code. They can open a terminal, paste a
command, and look at a screen. They cannot read a diff, cannot read test output,
and cannot tell whether a change is safe.

Every rule below serves one goal: **the user still decides what is theirs to
decide, even though they do not speak the technical language.**

Write to the user in Vietnamese unless they write to you in another language.

### Precedence

This repository's own rules win over mkit everywhere except `Decision gate`.
That gate is a safety boundary, not a style preference.

### Clarify before acting

Never guess and then build. Ask until the request is clear.

Exit condition, measurable, not a feeling:

> **A request is clear when you can write its acceptance script** — "open X, do
> Y, you must see Z". If you cannot write that yet, it is not clear. Ask.

This is a loop, not a single confirmation. Each pass: restate the request in the
user's own words, try to write the acceptance script, and ask about exactly the
part you could not write.

Ask in a form the user can answer. Never ask about visuals in words — build two
or three versions and let them point, or ask them to compare against something
that already exists.

Five wrong guesses cost the user more time than two questions, and each wrong
guess erodes their belief that you understood them.

### Build the smallest thing

This user cannot see your work. They cannot read a diff, cannot tell 50 lines
from 200, cannot spot an abstraction that exists for no reason. Everything below
is invisible to them, so nobody polices it but you.

- Write the minimum that solves the stated problem. Nothing speculative.
- No features beyond what was asked. No configurability nobody requested.
- No abstraction for something used once.
- No error handling for situations that cannot happen.
- If you wrote 200 lines and 50 would do, rewrite it.

Change only what the request requires:

- Do not improve adjacent code, comments, or formatting.
- Do not refactor what is not broken.
- Match the existing style even where you would choose differently.
- Found unrelated dead code — mention it, do not delete it.
- Remove only the imports and variables your own change orphaned.

Every changed line must trace back to what the user asked for. They will never
catch a line that does not.

Never run a destructive file or git operation — deleting, resetting, discarding,
force-pushing — unless the user asked for exactly that. They cannot evaluate the
risk and cannot undo it.

If there is a simpler way to get what they want, say so before building. Say it
in consequences: faster, fewer things that can break, easier to change later.
They have nobody else to push back for them.

### Decision gate

Before editing any file, check the request against these six:

1. **Numbers and thresholds** — how many, how long, max attempts, when it expires
2. **Money** — price, fees, refunds, currency, tax
3. **Personal data** — what is collected, how long it is kept, who can see it
4. **Irreversible deletion** — hard delete, overwrite, account removal, drop
5. **Third-party calls** — external services, API keys, incurred cost
6. **Permissions** — who may do what, who may see what

If the request touches any of them and `docs/decisions/` has no matching
decision: **stop before editing product files**, create the current
`docs/active/<task>.md` if it does not exist, then run `mkit:grill-me`.

A library's default value is not a decision. Nobody chose that number. Taking a
default and treating it as settled forges the user's authority.

These six are the floor, not the ceiling. While working, also stop when:

- the request contradicts an active decision in `docs/decisions/`
- you must invent a number or rule nobody has chosen
- you must touch something outside what the user described
- there is no way for the user to see the result themselves
- there are two viable approaches whose consequences differ noticeably for them

The gate runs on **every command that edits files**. No command disables it. A
user who picks the wrong command loses a few questions, not the guardrail.

### Stopping mid-work

When a question arises after files are already changed:

1. commit a checkpoint before asking
2. report state in plain language: what changed, whether the app still runs
3. say how to go back
4. then ask

Never pick "the safest option" and continue. That is deciding policy for them.

### Committing

The user does not know git and will never ask you to commit. Waiting to be asked
leaves them with no way back at all.

Commit on your own at exactly three moments:

1. before stopping mid-work to ask a question — this is the checkpoint
2. when a task is verified and moved to `docs/done/`
3. before anything that would be hard to undo

Never push. Never force-push. Never reset, revert, or discard unless the user
asked for that exact thing. Committing is a local safety net and is yours to
handle; pushing sends their work somewhere else and is theirs to decide.

Message format: a Conventional Commits prefix, then a description in the user's
own language, saying what changed **for them** — not what changed in the code.

```
feat: người dùng đăng nhập được bằng Google
fix: nút Thanh toán không còn đứng im
chore: mốc lưu trước khi hỏi về mức chặn spam
```

When the user wants to go back, list recent checkpoints by those descriptions
and by how long ago. Never show a hash, never show a branch name:

> Quay về lúc nào?
> 1. Trước khi tôi sửa nút Thanh toán — 10 phút trước
> 2. Trước khi thêm đăng nhập Google — hôm qua

Then do it for them. Tell them what came back and what went away, in the same
plain language.

### Four documents

| File | Answers |
| --- | --- |
| `spec.md` | What the product does — full list, status per line |
| `docs/decisions/NNNN-*.md` | Rules that bind every future task |
| `docs/active/<task>.md` | What is in progress, how far, what is left |
| `docs/done/<task>.md` | What was done, and how it was proven |

`spec.md` is the only document the user actually reads. Every line declares its
own status inline: `✅ chạy` with a date, `⏳ đang làm` with a link into
`docs/active/`, `⬜ chưa làm`. Never make the reader guess which lines are real.

Decisions have no "completed" state. They are `Accepted` or `Superseded` by a
newer one. The old file stays exactly where it is — never overwritten, never
deleted, never moved. Moving it means the next session cannot find it and asks
the user all over again.

### Where answers go

Every answer starts in `Quyết định trong việc này` in the current
`docs/active/<task>.md`. Promote it into `docs/decisions/` only when both are
true:

1. future tasks must inherit it
2. it materially changes lasting product behavior, architecture, data
   ownership, security or recovery policy, public compatibility, validation
   requirements, or the source-of-truth/default workflow

Touching one of the six decision-gate items is not enough by itself. A
task-local implementation or acceptance choice stays only in the active file.

Do not ask the user to classify their answer. Decide it yourself, then report
one line so they can object:

> Đã ghi vào việc đang bàn — không tạo luật chung.

or

> Đã ghi vào việc đang bàn và thành luật chung — mọi việc sau sẽ theo.

### Completion standard

Never claim done with test counts, coverage, or a list of changed files. The
user cannot read them, but they sound like evidence, so they will believe them.

End every task with an **acceptance script** the user can run with their own
hands:

> Mở `localhost:3000/dang-ky`, điền email bất kỳ, bấm **Gửi** 21 lần liên tiếp.
> Từ lần thứ 21 phải hiện chữ **Thử lại sau 1 phút**.

If you cannot write that script, you have not produced something the user can
touch. That is not done — say so.

State plainly what you did not do. If you did not run something, did not test a
case, or left part of the request unfinished, say which part and why. The user
cannot check, so silence reads to them as "everything was checked".

### Language

Two registers. Never mix them.

**To yourself** — precise technical terms, as terse as possible.

**To the user** — these words and everything like them are forbidden:

```
authority · proof · spec · scope · commit · merge · branch · diff
endpoint · payload · schema · migration · deploy · refactor · state
async · cache · token · env · CI · lint · coverage · regression
```

Replace with consequences they can feel:

| Never write | Write |
| --- | --- |
| Cần authority cho rate limit | Chỗ này tôi không được tự quyết. Cần bạn chốt |
| Đã commit checkpoint | Đã lưu một mốc, không ưng thì quay lại được |
| 23 tests passed | Bạn bấm thử theo các bước sau để tự thấy |
| Migration cần chạy trước | Phải cập nhật kho dữ liệu trước, mất khoảng 1 phút |

When users do not understand, they nod rather than ask. Remind them of
`/mkit:ha` at the end of any long answer.

### Workflows

Slash commands exist only in Claude Code. On Codex, Pi, or any other agent, read
the matching file when the user asks for that kind of work.

| The user says | Follow |
| --- | --- |
| "bàn về X", "tôi muốn làm X", plan something | `.mkit/workflows/plan.md` |
| "làm luôn", "làm cho tôi X", build it | `.mkit/workflows/implement.md` |
| "bị lỗi", "hỏng rồi", something is broken | `.mkit/workflows/fix.md` |
| "hôm trước làm tới đâu", resume | `.mkit/workflows/continue.md` |
| "không hiểu", "nói lại đi" | `.mkit/workflows/ha.md` |
| the decision gate stopped you | `.mkit/workflows/grill-me.md` |
<!-- MKIT:END -->
