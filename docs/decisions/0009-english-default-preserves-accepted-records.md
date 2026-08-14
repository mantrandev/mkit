# 0009 English defaults preserve accepted records

Date: 2026-08-14

## Status

Accepted

## Context

Decision 0008 made English the default but also required translating every
accepted decision. That conflicts with the existing rule that accepted records
are immutable. It also left fixed English output in workflows that promise to
follow a user's established language.

## Decision

English is the default for source instructions, skills, templates, metadata,
installer output, README content, and all new project records.

Previously accepted decision records keep their original content and filenames,
regardless of language. When a user establishes another language, user-facing
questions, summaries, and prose written into task records use that language.
Stable document headings and status labels remain English schema. English
examples of user-facing messages are fallbacks, not fixed output.

This decision supersedes 0008.

## Technical constraints

Never translate or otherwise rewrite the body of an accepted or superseded
decision. Its status may change only when a newer decision supersedes it.

English-only content checks exclude immutable decision records created before
this decision. New decisions and distributed mkit surfaces default to English.

Any workflow that specifies user-facing wording must require the current user's
language and treat English wording as a fallback example. Installed document
headings and status labels remain English so every workflow reads one schema.

## Alternatives considered

1. **Translate accepted records in place.** This makes the tree uniformly
   English but destroys the immutable record guarantee.
2. **Force English for every user.** This simplifies examples but contradicts
   the multilingual behavior mkit promises.

## Tradeoffs

The repository retains Vietnamese prose in historical decisions. mkit accepts
that visible inconsistency to preserve trustworthy records while keeping all
new and distributed surfaces English by default.
