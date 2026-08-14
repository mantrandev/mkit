---
name: init
description: Install mkit into the current repository, preserve an existing codebase's architecture, and leave an empty repository unstructured until its first implementation. Create the document folders and insert the instruction block into AGENTS.md and CLAUDE.md. Run once per project.
---

Install mkit into the repository that is currently open. Safe to run repeatedly;
later runs only refresh the instruction block.

Use the user's language. Default to English when the user has not established one.

## 1. Check

Confirm this is a git repository. If not, ask whether to `git init` before doing
anything else — without git, every checkpoint is meaningless.

## 2. Inspect the starting repository

Determine whether product source already exists. Ignore Git metadata, mkit
records, agent instruction files, and a README by themselves.

Read existing repository-local instructions and any existing architecture
document. If the repository has product source, inspect its modules, tests,
build configuration, dependency direction, important cross-boundary flows, and
verification commands. Do not edit product source during initialization.

If the repository has no product source, classify it as empty. Do not invent an
architecture for it.

## 3. Create the folders

```
docs/decisions/
docs/active/
docs/done/
docs/templates/
```

Leave any that already exist untouched.

Copy `active.md`, `architecture.md`, `decision.md`, and `spec.md` from
`core/templates/` into `docs/templates/`. Refresh only these mkit-owned
templates.

## 4. Insert the instruction block

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

## 5. Write the workflows for other agents

Copy every `SKILL.md` from this plugin's own `skills/` directory into
`.mkit/workflows/<name>.md` — you are reading one of those files right now, so
its parent directory is where they live.

Slash commands exist only in Claude Code. Codex and Pi reach the same workflows
through the lookup table at the end of the `MKIT` block. Without this step that
table points at nothing.

Skip `init` itself; it is not a workflow.

## 6. Create `spec.md`

If missing, create it from `docs/templates/spec.md`. Ask one question to fill the
first line:

> What does this product do, and for whom? One sentence.

If `spec.md` already exists, leave it alone.

## 7. Establish the architecture record

If `docs/architecture.md` already exists, preserve it and use it. Never replace
an existing project architecture document during initialization.

If product source exists and the record is missing, create
`docs/architecture.md` from its template and replace every placeholder with
observed facts. Record the current system shape, module ownership, allowed
dependencies, important flows, applicable verification commands, and uncertain
boundaries under `Known gaps`. Do not redesign or reorganize the source.

If the repository is empty, do not create `docs/architecture.md`, placeholder
layers, or speculative modules. The first `mkit:implement` run creates a fully
populated record after real source establishes a boundary.

## 8. Report

Briefly, in plain language: installed, which commands now exist, which one to
run first.

For a repository with source, say whether its existing architecture record was
preserved or an observed record was created without changing source. For an
empty repository, say that no architecture was invented and the first
implementation will record the structure it actually creates.

Add one line that matters:

> Whenever I say something you do not understand, type `/mkit:ha` and I will
> explain it a different way.
