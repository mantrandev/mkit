# 0001 Distill instead of forking repository-harness

Date: 2026-08-14

## Status

Accepted

## Context

`hoangnb24/repository-harness` already provides a strong implementation of an
AI-agent harness. mkit would not justify its existence as a copy. It needed a
clear difference large enough to make a fork the wrong shape.

## Decision

mkit is an independent repository that adopts ideas rather than source code:
the authority gate, work classification, completion standard, and
`decision.md` structure.

The single but substantial difference is the target user. repository-harness
assumes the user can read code; mkit does not. Because that difference affects
the first instructions an agent reads, it cannot be layered on top. Those
instructions must be rewritten.

## Technical constraints

mkit has no runtime dependency on repository-harness. It uses neither Rust nor
SQLite. The entire product is Markdown and skills.

Preserve the MIT attribution for both `hoangnb24/repository-harness` and
`mattpocock/skills` in `NOTICE`, stating what mkit inherits and changes.

## Alternatives considered

1. **Fork repository-harness's Rust code.** This would inherit roughly 21,000
   lines from the layer its author froze in decision 0022 and removed from the
   default workflow.
2. **Layer a skill pack over the default repository-harness installation.** The
   translation layer must change the core `AGENTS.md` itself. It cannot live
   beside it, and tracking a fast-moving upstream would replace product work
   with synchronization work.

## Tradeoffs

mkit is much smaller than a repository-harness rewrite. After the upstream
project moved away from SQLite, risk lanes, and scoring, the remaining product
gap is mainly the plain-language layer. mkit accepts that narrow boundary in
exchange for a clean wedge and no obligation to maintain someone else's code.
