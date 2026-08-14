---
name: init
description: Install mkit into the current repository — create the document folders and insert the instruction block into AGENTS.md and CLAUDE.md. Run once per project.
disable-model-invocation: true
---

Install mkit into the repository that is currently open. Safe to run repeatedly;
later runs only refresh the instruction block.

Talk to the user in Vietnamese.

## 1. Check

Confirm this is a git repository. If not, ask whether to `git init` before doing
anything else — without git, every checkpoint is meaningless.

## 2. Create the folders

```
docs/decisions/
docs/active/
docs/done/
```

Leave any that already exist untouched.

## 3. Insert the instruction block

Into `AGENTS.md`: the contents of `core/AGENTS.block.md`.
Into `CLAUDE.md`: the contents of `core/CLAUDE.block.md`.

Insertion rules, same for both files:

- file already contains `<!-- MKIT:BEGIN -->` → replace everything between
  `BEGIN` and `END`, touching nothing outside
- file exists without the markers → append the block to the **end**
- file does not exist → create it with a `# Project Rules` heading, then the block

Never overwrite a whole file. The user's own rules live outside the block and
must survive.

`AGENTS.md` is what Codex and Pi read. `CLAUDE.md` only imports it, so the rules
exist in exactly one place and cannot drift.

## 4. Write the workflows for other agents

Copy every `SKILL.md` from this plugin's own `skills/` directory into
`.mkit/workflows/<name>.md` — you are reading one of those files right now, so
its parent directory is where they live.

Slash commands exist only in Claude Code. Codex and Pi reach the same workflows
through the lookup table at the end of the `MKIT` block. Without this step that
table points at nothing.

Skip `init` itself; it is not a workflow.

## 5. Create `spec.md`

If missing, create it from `docs/templates/spec.md`. Ask one question to fill the
first line:

> Sản phẩm này làm gì, cho ai? Một câu thôi.

If `spec.md` already exists, leave it alone.

## 6. Report

Briefly, in plain language: installed, which commands now exist, which one to
run first.

Add one line that matters:

> Lúc nào tôi nói thứ gì bạn không hiểu, gõ `/mkit:ha` — tôi sẽ nói lại kiểu khác.
