# 0008 English is the default language

Date: 2026-08-14

## Status

Superseded by 0009

## Context

mkit previously mixed English agent instructions with Vietnamese examples,
templates, metadata, installer output, and user-facing defaults. That made the
repository inconsistent and made English-speaking installations inherit
Vietnamese behavior.

## Decision

English is the default language for every mkit surface: repository documents,
agent instructions, skills, examples, templates, plugin metadata, and installer
output.

Agents still follow a user who explicitly establishes another language. English
is the fallback, not a ban on multilingual conversations.

## Technical constraints

Tracked file contents contain no Vietnamese prose. Installed templates and
workflows use English headings and status labels. Plugin manifests and installer
messages are English.

Durable decision filenames remain unchanged so existing links and history stay
valid; their contents are English.

## Alternatives considered

1. **Translate only README and plugin metadata.** Existing installations would
   still receive Vietnamese instructions and templates.
2. **Force English even when the user writes another language.** This would
   reduce accessibility without improving the default installation.

## Tradeoffs

Historical filenames retain Vietnamese slugs. mkit accepts that visible remnant
to preserve stable paths and the rule that accepted decision records are never moved.
