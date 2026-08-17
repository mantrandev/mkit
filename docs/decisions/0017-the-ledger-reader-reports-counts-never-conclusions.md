# 0017 The ledger reader reports counts, never conclusions

Date: 2026-08-17

## Status

Accepted

## Context

Decision 0014 settled what the ledger stores and the program has been recording
since. Nothing reads it, so the evidence accumulates and changes nothing.

The obvious next step is the one repository-harness took: turn repeated evidence
into generated proposals for improving the rules. That system needed a proposal
identity, a lifecycle with six states, evidence links kept after closure, and
recurrence classification, because it produced enough proposals that they had to
be managed.

mkit produces a handful of signals a month. Building that machinery here would
cost more than reading the file by hand, and mkit has already watched its
upstream freeze exactly that layer once it stopped paying for itself.

There is also a subtler risk. A reader that prints "the implement command is
confusing, rewrite it" invents a conclusion from data that cannot support it.
Fourteen requests for a different explanation might mean the wording is bad, or
that the command is used for the hardest work, or that one user was tired. The
count is a fact; the cause is a judgment, and it belongs to a person.

## Decision

`mkit-gate study` reads the ledger and reports what happened, ranked by how
often it happened. It never says what to change.

It reports which commands were followed by a request for a different
explanation, which commands produced work the user rejected and how many
attempts that took, which pairs of rules could not both be followed, which of
the six gate items stopped work with no decision on record, and how often work
was attempted before the gate had run.

Every line carries its count and the number of requests it is measured against,
so a reader can tell a pattern from a coincidence themselves.

It sets no thresholds. Nobody has chosen how many occurrences make a pattern, so
the program does not pretend one exists: everything observed is listed in order.

It reads and prints. It never edits a rule, opens a task, or writes to the
ledger.

## Technical constraints

`mkit-gate study` reads `.mkit/ledger.jsonl` in the current repository and
writes a plain-text report to standard output.

A malformed line is skipped, not fatal. A missing ledger prints that nothing has
been recorded yet and exits successfully.

No thresholds, no percentages presented as verdicts, no suggested actions, no
severity labels. Counts and totals only.

The command never writes anything. It is safe to run at any time and does not
require the gate.

Aggregating ledgers from more than one repository is not built, and collecting
them from other people remains unbuilt and undecided, as decision 0014 states.

## Alternatives considered

1. **Generate proposals like repository-harness.** Turns evidence into candidate
   rule changes automatically, but requires proposal identity, a lifecycle, and
   recurrence classification to stay usable, which is more machinery than a
   handful of monthly signals can justify.
2. **Print conclusions rather than counts.** Shorter to read and immediately
   actionable, but manufactures a cause from data that only shows frequency, and
   a wrong conclusion stated confidently is worse than a number.
3. **Set a threshold so only repeated signals are shown.** Reduces noise, but the
   threshold is a number nobody has chosen, which is precisely what the gate
   exists to prevent.

## Tradeoffs

Reading the report requires judgment, and mkit's own user cannot supply it. This
command is for whoever maintains the rules, not for the person the rules
protect. It is the one part of mkit that assumes a reader who can weigh
evidence.

Because there are no thresholds, a ledger with many kinds of rare events
produces a long report. That is accepted so nothing is hidden by a number nobody
chose.
