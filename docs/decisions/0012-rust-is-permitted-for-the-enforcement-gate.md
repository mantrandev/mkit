# 0012 Rust is permitted for the enforcement gate

Date: 2026-08-17

## Status

Accepted

## Context

Decision 0001 forbade Rust and SQLite so that mkit stayed markdown and skills
only, keeping installation at the level of copying files. That constraint kept
mkit light, but it left the authority gate — the thing mkit exists to protect —
as a paragraph inside `core/AGENTS.block.md`. An agent can read that paragraph
and skip it, and nothing records that it happened. The sentence "the gate runs
on every command that edits files, no command disables it" is a promise, not a
mechanism.

Claude Code hooks can enforce, but Codex and Pi do not have hooks. mkit claims
to support all three, so the real level of protection differs by agent and the
user has no way to know which one they are getting.

## Decision

mkit remains an independent repository that takes ideas from
`repository-harness` rather than its source: the authority gate, work
classification, the completion standard, and the `decision.md` structure. The
founding difference still holds — mkit is written for a user who cannot read
code.

What changes: mkit may ship a compiled program, and that program is required
for mkit to run.

The program does exactly two things: it blocks work when the authority gate has
not run, and it records that the gate ran. All rule content stays in markdown.
The program is not where rules live; it is what makes rules impossible to skip.

In exchange, installing mkit is no longer a file copy. The user must download a
program, and their operating system may refuse to run it.

This decision supersedes 0001.

## Technical constraints

Rust is permitted in the mkit repository. SQLite is permitted but unused until a
question genuinely requires relational queries; the ledger stays JSONL until
then.

No dependency on `repository-harness` at any level: no source, no crate, no
schema, no protocol format. Preserve the existing MIT attribution for
`hoangnb24/repository-harness` and `mattpocock/skills` in `NOTICE`.

The program is named `mkit-gate`, lives in `crates/mkit-gate/`, and exposes only
gate commands. It enforces **procedure**, not **judgment**. Whether a request
touches one of the six gate items is a semantic judgment; the program cannot
decide that and must not try. It only guarantees that the judgment was made and
recorded before product files are edited.

A compiled program the agent must call voluntarily is still an instruction the
agent can skip. Enforcement therefore requires a per-agent trigger at the tool
layer that runs `mkit-gate check` before any file write. On Claude Code that
trigger is a `PreToolUse` hook; exit code 2 cancels the tool call. The program
holds the shared logic so every agent enforces identically; the trigger is what
makes it non-bypassable.

Every failure path fails closed. A missing session identifier, an unreadable
marker, a malformed payload, or any unexpected error blocks the write rather
than allowing it.

Rule text is never copied into Rust source. The only rule sources are
`core/AGENTS.block.md` and `core/CLAUDE.block.md`.

`install.sh` downloads the platform build and verifies its checksum before
installing. When no valid build can be obtained, the installer stops with a
clear message instead of installing half of mkit.

Three items are not chosen here and this decision does not choose them: who pays
for and signs the macOS build, where builds are hosted, and which fields
`.mkit/ledger.jsonl` may contain.

## Alternatives considered

1. **Stay markdown only.** Costs nothing and installation stays a file copy,
   but the authority gate remains a paragraph an agent can skip.
2. **Use a Claude Code shell hook only.** Real enforcement with no build step,
   but Codex and Pi have no hooks, so three agents give three different levels
   of safety while mkit promises to support all three.
3. **Ship the program as optional.** Installation never breaks, but an
   enforcement mechanism that can be switched off is not an enforcement
   mechanism, and the ledger no longer says who it represents.
4. **Move the whole control plane into Rust and SQLite like harness.** The
   strongest option, but 15k or more lines to maintain alone, and the upstream
   author removed exactly that layer from the default install path in their own
   decision 0022 after two months of operation.

## Tradeoffs

Installing mkit is no longer a file copy. On macOS an unsigned program is
blocked by the operating system with a message saying the developer cannot be
verified. mkit users are people who cannot read code — they will stop at that
dialog with no way past it. mkit accepts this risk in exchange for an authority
gate that cannot be skipped, and defers the signing decision.

mkit also loses the ability to run on any platform without a published build,
and from now on maintains Rust alongside markdown.
