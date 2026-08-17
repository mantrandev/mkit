# mkit

Your AI agent stops and asks you, instead of guessing.

For people who build things by describing them, and cannot read code.

## What changes

Without mkit:

```
You    add paid plans
Agent  done — 99k, 199k and 499k per month

       it picked your prices. you never found out.
```

With mkit:

```
You    add paid plans
Agent  I cannot pick your prices. How much per month?

       it stopped. you decide.
```

## It stops for six things

```
  numbers      how many, how long, when it runs out
  money        prices, fees, refunds
  personal     emails, names, anything about a person
  deleting     things nobody can get back
  other apps   Stripe, Google, sending mail
  who can      who is allowed to see or do what
```

Everything else it just builds. Changing a button colour asks you nothing.

## How it works

```
  you speak
      │
      ▼
  does this touch one of the six?
      │
      ├── no ───▶ builds it
      │
      └── yes ──▶ already decided before? ── yes ─▶ builds it
                        │
                        no
                        │
                        ▼
                  asks you, then writes your answer down
                  so it never asks the same thing twice
```

The stop is real. A small program refuses the change until the question has been
asked. It is not a note the agent can skip.

## Install

macOS. Two steps.

**1.** In Claude Code:

```
/plugin marketplace add mantrandev/mkit
/plugin install mkit@mkit
```

**2.** In your project folder:

```bash
curl -fsSL https://raw.githubusercontent.com/mantrandev/mkit/main/install.sh | bash
```

Nothing to configure. Projects without mkit are left alone.

<details>
<summary>Codex and Pi</summary>

```bash
codex plugin marketplace add mantrandev/mkit --ref main
codex plugin add mkit@mkit
```

New thread, run `$mkit:init` once, then start another thread. Pi reads
`AGENTS.md` directly. Both get the rules. Only Claude Code gets the real stop.

</details>

## How you know it worked

Never this:

```
23 tests passed, coverage 87%
```

Always this:

```
Open /sign-up. Click Submit 21 times.
Attempt 21 must say "Try again in 1 minute."
```

You run it. You see it. That is the proof.

## Going back

You never need to learn git.

```
Return to which point?
1. Before fixing the Checkout button — 10 minutes ago
2. Before adding Google sign-in — yesterday
```

Pick a number. Your code is never sent anywhere unless you ask.

## Talking to it

| Say | It does |
| --- | --- |
| "plan X" | talks it through, changes nothing |
| "build X" | builds until you can try it |
| "this is broken" | reproduces it first, then fixes |
| "where did we stop?" | picks up unfinished work |
| "I don't understand" | says it again, differently |

In Claude Code these are also `/mkit:plan`, `/mkit:implement`, `/mkit:fix`,
`/mkit:continue`, `/mkit:ha`.

`/mkit:ha` matters most. People who do not understand tend to nod. This makes
saying so one keystroke.

## What it writes down

| File | Answers |
| --- | --- |
| `spec.md` | what your product does, and what actually works |
| `docs/decisions/` | what you decided, so nobody asks twice |
| `docs/active/` | what is half finished |
| `docs/done/` | what is finished, and how it was proven |

Only `spec.md` is for you:

```
- [x] Email sign-in    ✅ working · 2026-08-02
- [ ] Google sign-in   ⏳ in progress
- [ ] Password reset   ⬜ not started
```

## What the agent owes you

You cannot see these, so the rules make it responsible for them.

```
  write the least that solves it
  touch only what you asked about
  say the simpler way before building
  say out loud what was never tested
  never delete or undo unless you asked for exactly that
```

Silence sounds like everything was checked. It is not.

## Built from

- [`hoangnb24/repository-harness`](https://github.com/hoangnb24/repository-harness)
- [`mattpocock/skills`](https://github.com/mattpocock/skills)

Both MIT. [`NOTICE`](./NOTICE) says exactly what was taken and what changed.

## License

MIT
