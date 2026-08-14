# mkit

An AI-agent harness workflow for people who **cannot read code**.

## The problem

Existing agent harnesses are strong, but they usually assume the user is a
developer. They stop at the right moment for human judgment, then ask in terms a
non-technical user cannot answer:

> *Add rate limiting* lacks a quota, trusted key, enforcement topology, and
> response contract. Stop.

The user said, "Stop people from spamming my website." They cannot answer the
technical question because they do not have the concepts behind it.

Skipping the question is worse. The agent silently chooses a number and writes
it into code, while the user cannot detect the invented policy. They discover it
only after a customer complains.

## What mkit does

mkit keeps the decision gate and acts as a **two-way translator** at that gate.

**Translate down** — turn a technical question into consequences the user can picture:

```text
❓ Decision needed — How aggressively should repeated clicks be blocked?

A. Strict — 20 attempts per minute
   → Gain: almost all spam accounts are stopped
   → Tradeoff: a real customer clicking quickly may be blocked for 1 minute

B. Lenient — 100 attempts per minute
   → Gain: real customers are almost never blocked by mistake
   → Tradeoff: several dozen spam accounts may need manual cleanup each day

➡️ I recommend A because manual cleanup is expensive, while a customer blocked
by mistake can retry after 1 minute.
```

**Translate up** — turn technical evidence into steps the user can perform:

```text
✗ 23 tests passed, coverage 87%
✓ Open /sign-up and click Submit 21 times. Attempt 21 must show "Try again in 1 minute."
```

The user still decides. Only the language changes.

## Install

### Claude Code

Two commands:

```text
/plugin marketplace add mantrandev/mkit
/plugin install mkit@mkit
```

Open your project and run `/mkit:init` once.

### Codex

Run:

```bash
codex plugin marketplace add mantrandev/mkit --ref main
codex plugin add mkit@mkit
```

Open a new Codex thread in your project and run `$mkit:init` once. Start another
new thread afterward so Codex loads the newly installed project rules.

You can also open `/plugins`, select the `mkit` marketplace, and install the
`mkit` plugin. Start a new thread after installation.

<details>
<summary>Install without a plugin using one terminal command</summary>

```bash
curl -fsSL https://raw.githubusercontent.com/mantrandev/mkit/main/install.sh | bash
```

This installs into the current directory. Add a target path after the command to
install elsewhere. Running it again refreshes the marked instruction blocks and
preserves everything outside them.

This path installs rules and workflows for Codex and Pi without a plugin.

</details>

## Three agents, one rule set

| Agent | Reads | Usage |
| --- | --- | --- |
| **Claude Code** | `CLAUDE.md` → `AGENTS.md` | Run `/mkit:plan`, `/mkit:fix`, and the other slash commands |
| **Codex** | plugin → `AGENTS.md` | Run `$mkit:init` once, then use plain-language requests |
| **Pi** | `AGENTS.md` | Use plain-language requests |

Rules live in exactly one place: `AGENTS.md`. `CLAUDE.md` contains only an
`@AGENTS.md` import. Parallel copies inevitably drift and leave the agent without
a clear source of truth.

Slash commands are a Claude Code convenience. Codex and Pi select the same
workflow through the routing table at the end of the `MKIT` block, which points
to `.mkit/workflows/`.

## Six commands

| Command | Purpose |
| --- | --- |
| `/mkit:init` | Create the document structure and ask what the product does |
| `/mkit:plan` | Discuss and settle a task without editing code |
| `/mkit:implement` | Build until the user can test the result themselves |
| `/mkit:fix` | Reproduce a bug first, then fix it |
| `/mkit:continue` | Recover unfinished work from an earlier session |
| `/mkit:ha` | Explain the last point a different way |

After `$mkit:init`, Codex and Pi users can speak normally. The agent selects the
matching workflow.

`/mkit:ha` is the most important command. A user who does not understand often
agrees instead of asking. This command makes "I do not understand" easy to say.

## Four documents

| File | Answers |
| --- | --- |
| `spec.md` | What the product can do, with status on every line |
| `docs/decisions/` | Lasting rules that future work must inherit |
| `docs/active/` | What is in progress, how far it got, and what remains |
| `docs/done/` | What was completed and how it was verified |

Every answer given during planning starts in `docs/active/`. A separate file in
`docs/decisions/` is created only for a lasting product, architecture, data,
security, compatibility, validation, or default-workflow choice that future
tasks must inherit.

`spec.md` is the only document the target user must read. Each line declares its
own status:

```markdown
- [x] Email sign-in     ✅ working · 2026-08-02
- [ ] Google sign-in    ⏳ in progress · docs/active/google-login.md
- [ ] Password reset    ⬜ not started
```

## Decision gate

mkit stops when a task touches one of six categories:

**numbers and thresholds · money · personal data · irreversible deletion · third-party calls · permissions**

Outside those categories, the agent decides implementation details such as
variable names, file boundaries, libraries, and routine layout choices. The gate
is designed to activate rarely. Changing a button color requires no policy question.

The dividing rule is simple: the agent decides when a mistake is cheap and easy
to detect. The user decides when a mistake could remain invisible to them.

Needing a question does not automatically create a shared rule. The gate decides
**whether the agent must ask**. The answer's lifetime decides **whether it stays
in the current task or is promoted for future work**.

## What the user cannot see

People who cannot read code also cannot detect several common failure modes.
mkit makes the agent responsible for them:

- **Write the minimum.** Do not add unrequested features or abstractions for a
  single use. Rewrite 200 lines when 50 are enough.
- **Touch only what the request requires.** Do not clean adjacent code, reformat
  working files, or alter unrelated behavior.
- **Do not delete, reset, or force-push** unless the user requests that exact operation.
- **Present a simpler path before building** when it is faster, safer, or easier to change.
- **Disclose every untested or unfinished part.** Silence sounds like complete verification.

## Rollback

The user does not need to know git. The agent creates local checkpoints before
stopping mid-work and after a task is accepted.

When asked to go back, the agent presents choices like:

```text
Return to which point?
1. Before fixing the Checkout button — 10 minutes ago
2. Before adding Google sign-in — yesterday
```

No hashes and no branch names. The user chooses a number.

The agent never pushes by default. A local checkpoint is a safety duty; sending
code elsewhere requires an explicit user request.

## Built from

- [`hoangnb24/repository-harness`](https://github.com/hoangnb24/repository-harness) — authority gate, work classification, completion standard, and `decision.md` structure
- [`mattpocock/skills`](https://github.com/mattpocock/skills) — the `grilling` pattern

Both projects use the MIT license. See [`NOTICE`](./NOTICE) for the exact inherited
and changed behavior.

## License

MIT
