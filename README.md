# mkit

An AI-agent harness for people who **cannot read code**.

The agent still does the work. You still make the decisions that are yours —
even though you do not speak the technical language.

## The problem

Ask for *"stop people spamming my site"* and a good harness stops to ask for a
quota, a trusted key, an enforcement topology. You cannot answer that; you do not
have the concepts.

Skipping the question is worse. The agent picks a number, writes it into the
code, and you find out when a customer complains.

mkit keeps the question and changes the language:

```text
❓ How aggressively should repeated clicks be blocked?

A. Strict — 20 attempts per minute
   → almost all spam is stopped
   → a real customer clicking fast is blocked for 1 minute

B. Lenient — 100 attempts per minute
   → real customers are almost never blocked
   → a few dozen spam accounts need cleanup each day

➡️ I recommend A: cleanup costs you time, a blocked customer waits 1 minute.
```

And it changes what "done" means:

```text
✗ 23 tests passed, coverage 87%
✓ Open /sign-up and click Submit 21 times.
  Attempt 21 must show "Try again in 1 minute."
```

## Install

macOS. Claude Code:

```text
/plugin marketplace add mantrandev/mkit
/plugin install mkit@mkit
```

Then, in your project:

```bash
curl -fsSL https://raw.githubusercontent.com/mantrandev/mkit/main/install.sh | bash
```

That writes the rules, the documents, and the gate program. It prints one last
step you have to do yourself — see [The gate](#the-gate).

<details>
<summary>Codex and Pi</summary>

```bash
codex plugin marketplace add mantrandev/mkit --ref main
codex plugin add mkit@mkit
```

Open a new thread and run `$mkit:init` once, then start another thread so the
project rules load. Pi reads `AGENTS.md` directly.

Codex and Pi get the rules and the workflows. They do not get the enforced gate
below — that currently exists for Claude Code only.

</details>

## The gate

mkit stops before editing files when a request touches one of six things:

**numbers and thresholds · money · personal data · irreversible deletion ·
third-party calls · permissions**

The rule is simple: the agent decides when a mistake is cheap and easy to spot.
You decide when a mistake could stay invisible to you. Changing a button colour
asks you nothing.

This is not a written promise. `install.sh` installs a small program that
**refuses the file edit** until the gate has run for your current request, and
fails closed on every error path.

Arming it is one manual step, and only you can do it — merge
`.mkit/hooks/claude-code.json` into `.claude/settings.json`. Until you do, the
agent can still change files without asking you anything.

## Commands

| Command | What it does |
| --- | --- |
| `/mkit:init` | Set up the documents, reading an existing codebase without redesigning it |
| `/mkit:plan` | Talk a task through without touching code |
| `/mkit:implement` | Build until you can test it yourself |
| `/mkit:fix` | Reproduce the bug first, then fix it |
| `/mkit:continue` | Pick up unfinished work from an earlier session |
| `/mkit:ha` | Say the last thing again, differently |

`/mkit:ha` matters most. People who do not understand tend to nod instead of
asking. This makes "I do not understand" one keystroke.

On Codex and Pi, just say what you want in plain language.

## Documents

| File | Answers |
| --- | --- |
| `spec.md` | What the product does, with status on every line |
| `docs/architecture.md` | What owns what, and which dependencies are allowed |
| `docs/decisions/` | Rules that bind every future task |
| `docs/active/` | Work in progress, how far it got, what remains |
| `docs/done/` | What was finished, and how it was proven |

`spec.md` is the only one you have to read:

```markdown
- [x] Email sign-in     ✅ working · 2026-08-02
- [ ] Google sign-in    ⏳ in progress · docs/active/google-login.md
- [ ] Password reset    ⬜ not started
```

## What the agent owes you

You cannot see these failures, so the rules make the agent responsible for them:
write the minimum, touch only what the request needs, keep the architecture the
project already has, offer a simpler path before building, and say out loud what
was never tested. Silence sounds like verification.

It never deletes, resets, or force-pushes unless you ask for exactly that.

## Rollback

You do not need to know git. Checkpoints are saved before the agent stops to ask
and after a task is accepted. Ask to go back and you get:

```text
Return to which point?
1. Before fixing the Checkout button — 10 minutes ago
2. Before adding Google sign-in — yesterday
```

No hashes, no branch names. Pick a number. It never sends your code anywhere
without you asking.

## Built from

- [`hoangnb24/repository-harness`](https://github.com/hoangnb24/repository-harness)
  — authority gate, work classification, completion standard, `decision.md` structure
- [`mattpocock/skills`](https://github.com/mattpocock/skills) — the `grilling` pattern

Both MIT. [`NOTICE`](./NOTICE) records exactly what was inherited and what changed.

## License

MIT
