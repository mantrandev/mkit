# 0002 The target user is Tier B

Date: 2026-08-14

## Status

Accepted

## Context

"Non-technical" covers several capability levels, and each level produces a
different product. Without choosing one, every later decision remains unstable.

## Decision

mkit serves people who can **open a terminal and paste a command but cannot read
code**: product managers, designers, and founders. They recognize failure at the
level of "the screen is red."

They cannot review a diff, interpret test output, or independently judge whether
a change is safe.

## Technical constraints

Every feature must work through a command-line or chat interface. Do not build a GUI.

Every user-facing sentence must follow the vocabulary rules in the `Language`
section of the `MKIT` block: replace technical jargon with consequences the user
can picture.

## Alternatives considered

1. **A lower tier that cannot open a terminal.** That requires a GUI or web app,
   turning mkit from a kit into a software company.
2. **A higher tier that can read but not write code.** The original
   `repository-harness` already serves that group well, so mkit adds little value.

## Tradeoffs

mkit excludes the much larger group that cannot use a terminal. In return, it
can solve a real problem with a text layer and no service infrastructure.
