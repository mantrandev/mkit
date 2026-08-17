# 0014 The ledger records structure, never prose

Date: 2026-08-17

## Status

Accepted

## Context

Decisions 0012 and 0013 left one item unchosen: what `.mkit/ledger.jsonl` may
contain. Nothing can be measured about how mkit performs until that is settled,
so the improvement loop cannot begin.

The ledger lives inside the user's own repository. Anything it stores is stored
in a place the user cannot read and may later publish without realising it. A
ledger holding what the user typed would carry API keys, passwords, and customer
details into their git history permanently, and mkit users are people who cannot
read the file to check.

The questions the improvement loop actually asks are counts and labels: which
command was misunderstood most often, how often the gate refused work, which
rules contradicted each other, how many attempts a task took. None of them need
the text of anything.

## Decision

The ledger records what happened, never what was said.

It stores which command ran, which gate items a request touched, which decision
settled them, whether work was refused, how many attempts a task needed, and
which rules collided. It never stores what the user typed, what a file contains,
or which file was touched.

This is enforced by the program rather than trusted to the agent. The recording
command accepts only values drawn from fixed lists and short identifiers; it
refuses anything that looks like a sentence. An agent cannot write prose into
the ledger even if it tries.

Because it holds no personal content, the ledger is safe to keep in the
repository's history, and it survives moving to another machine.

## Technical constraints

`.mkit/ledger.jsonl` is append-only, one JSON object per line, and is tracked by
git. `.mkit/bin/` and `.mkit/gate/` are not tracked; `install.sh` writes a
`.mkit/.gitignore` that excludes them.

Recorded fields are exactly: an epoch-second timestamp, the mkit version, the
agent name from a fixed list, an event kind from a fixed list, an optional
originating command from a fixed list, an optional decision number, an optional
attempt count, an optional list of gate items, and an optional list of short
identifier tags.

Every identifier matches `[a-z0-9-]{1,32}`. Free text, whitespace, and any
character outside that set are refused, not sanitised. A line that would exceed
1024 bytes is refused, which also keeps a single append atomic on POSIX.

The recording command never fails the caller's work. Recording is observation,
not enforcement; a ledger that cannot be written must not block an edit.

Nothing is ever uploaded. Collecting ledgers from other people is a separate
decision nobody has made.

## Alternatives considered

1. **Record everything including what the user typed.** Allows replaying exactly
   how a misunderstanding happened, but puts secrets into git history forever in
   a file the user cannot audit, and adds nothing the improvement questions need.
2. **Record everything but keep the ledger out of git.** No leak into a public
   repository, but the evidence dies whenever the user changes machine or clones
   elsewhere, and it still sits readable on their disk.
3. **Restrict the content by rule rather than by the program.** Shorter to
   build, but it repeats the failure mkit exists to prevent: a written promise
   that nothing enforces.

## Tradeoffs

When a user reports that the agent misunderstood them, the ledger cannot show
what was said. Diagnosis stays at the level of counts and patterns, and the
conversation itself is gone. mkit accepts a coarser signal in exchange for a
record that can never leak anything about the person using it.
