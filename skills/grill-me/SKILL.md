---
name: grill-me
description: Extract a choice from a user who cannot read code. Ask one question at a time, state what each option gains and loses, update current product truth, and preserve only lasting rationale as a decision. Use when the decision gate stops because a choice has not been made.
---

Fork of `grilling` (Matt Pocock, MIT). Keeps the design tree and the rule that
finding facts is your job while decisions are the user's. Differs in two ways,
because this user cannot read code.

Use the user's language. Default to English when the user has not established one.

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

Keep the open question in the current conversation. Do not create an active
record merely to ask it or store its answer.

## Question format

Render every fixed label and sentence below in the user's established language.
The block shows the English fallback, not mandatory wording.

```
❓ **Decision needed** — <question, one line, no jargon>

<One short paragraph: why you are not allowed to decide this yourself.>

**A.** <option>
   → Gain: <concrete benefit>
   → Tradeoff: <concrete cost>

**B.** <option>
   → Gain: <concrete benefit>
   → Tradeoff: <concrete cost>

➡️ **I recommend A** because <reason, stated as a consequence>.

If any part is unclear, type `/mkit:ha`.
```

Two or three options. Four is too many.

"Gain" and "Tradeoff" must be things the user can picture: what a customer
experiences, how long it takes, how much it costs, whether it can be undone.
Never "better performance" or "easier to maintain".

## Example

```
❓ **Decision needed** — How aggressively should repeated clicks be blocked?

The registration page is being clicked repeatedly to create spam accounts.
Blocking helps, but a strict limit can also block real customers. I cannot
choose that tradeoff for you.

**A.** Strict — 20 attempts per person per minute
   → Gain: almost all spam accounts are stopped
   → Tradeoff: a real customer clicking quickly at peak time may be blocked and
     must wait 1 minute

**B.** Lenient — 100 attempts per person per minute
   → Gain: real customers are almost never blocked by mistake
   → Tradeoff: several dozen spam accounts may still need manual cleanup each day

➡️ **I recommend A** because manual spam cleanup is expensive, while a customer
blocked by mistake can retry after 1 minute.

If any part is unclear, type `/mkit:ha`.
```

## After the user decides

Route the answer by meaning:

1. If it defines current product behavior or acceptance, update the matching
   part of `spec.md`. Preserve existing headings and status labels. In a new
   mkit record, keep the exact English template schema; translate only filled
   prose.
2. If an active record already exists and the choice is needed to resume that
   work, append it with its reason to `Task decisions`.
3. Decide whether the answer also needs a record in `docs/decisions/`. That
   requires both conditions:

   - Future tasks must inherit the choice.
   - It materially changes lasting product behavior, architecture, data
     ownership, security or recovery policy, public compatibility, validation
     requirements, or the source-of-truth/default workflow.

Touching one of the six decision-gate items does not qualify by itself. A
bounded implementation detail may remain in the current conversation. Do not
create an active record solely to persist it.

Answer these checks yourself; do not ask the user to classify their answer.

**Record a lasting decision** → create a new file in `docs/decisions/` from
`docs/templates/decision.md`, numbered one above the highest existing. Fill both
sections: `Decision` in plain language and `Technical constraints` precise enough
to execute. If an active record exists, link its task-local entry to the new
decision. If it replaces an older decision, set that file's status to `Superseded by NNNN`
— do not delete it, do not edit its content, do not move it.

Then report one line in the user's established language naming every durable
source updated: current product description, shared decision, or active work.
If none was warranted, say the choice applies only to the current bounded task.
Use English only as the fallback language.

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
   ✗ What color do you want?
   ✓ The same blue as the current Save button, or darker?

   ✗ What layout do you want?
   ✓ A vertical list like Orders, or a square grid like Products?
   ```

3. **Still unclear — ask for something to copy.** Any page, app, or screenshot:
   *"Show me something you like and I will follow it."*

Never build one version and ask "Is this okay?" That is still guessing, just with
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

> This task is larger than it first appeared. About 3 more choices remain. We
> can finish the first part before deciding the rest.

## When the user lacks the authority

Some questions are genuinely not theirs — pricing, refund policy, legal terms.
Do not force a choice. Say so:

> This question belongs to the person responsible for <area>. Ask them and come
> back, or I can work on another part and leave this open.
