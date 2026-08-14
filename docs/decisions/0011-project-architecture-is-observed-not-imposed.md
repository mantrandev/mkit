# 0011 Project architecture is observed, not imposed

Date: 2026-08-14

## Status

Accepted

## Context

mkit already constrains change size and unnecessary abstraction, but it does
not tell an agent how to preserve a repository's module ownership and dependency
direction. Adding a universal Clean Architecture rule would solve that gap only
by forcing one design onto projects where it does not fit.

Initialization also has two different starting conditions. An existing codebase
has architecture that must be discovered and preserved. An empty repository has
no architecture to document yet.

## Decision

mkit enforces engineering integrity without imposing a universal architecture.
The current repository structure and its explicit local rules take precedence.
Clean Architecture is one possible project profile, used only when the project
already follows it or deliberately adopts it.

When mkit is initialized in a repository with product source, the agent records
the observed module ownership, dependency direction, important cross-boundary
flows, and verification commands in `docs/architecture.md`. It does not change
source code while doing so. An existing architecture document is preserved and
used as the source of truth.

When mkit is initialized in an empty repository, it does not create placeholder
layers or a speculative architecture record. The first implementation chooses
the smallest structure sufficient for the work and records the architecture
once real boundaries exist.

## Technical constraints

Before changing product code, implementation and fix workflows must read the
repository's explicit architecture guidance. If none exists, they inspect source,
tests, and build configuration to identify the current owners and dependency
direction.

Changes must place behavior in the owning module, preserve documented dependency
direction, avoid circular dependencies, avoid duplicated business rules, and use
existing seams before creating new ones.

`docs/architecture.md` describes current architecture. A decision record is
created only when future work must inherit a consequential architectural choice
and its rationale.

Initialization must distinguish repositories with product source from empty
repositories. It may create a fully populated architecture record for an
existing codebase, but it must never install an unfilled record or generate
speculative layers.

## Alternatives considered

1. **Require Clean Architecture everywhere.** This gives one familiar dependency
   model but adds layers and abstractions to small projects that do not need them.
2. **Keep architecture entirely implicit.** This minimizes documentation but
   makes later agents rediscover boundaries and increases accidental coupling.
3. **Create an empty architecture record during every init.** This makes the
   file predictable but presents placeholders as if architecture had been
   established.

## Tradeoffs

Architecture validation remains project-specific. Repositories without an
automated boundary check require a manual dependency review until they add one.
This costs more agent attention but preserves each project's actual design
instead of replacing it with mkit's preference.
