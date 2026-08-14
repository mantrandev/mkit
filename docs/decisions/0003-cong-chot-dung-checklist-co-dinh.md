# 0003 The decision gate uses a fixed checklist

Date: 2026-08-14

## Status

Accepted

## Context

The decision gate matters only if it activates at the right time. If detection
is unreliable, the rest of the workflow is meaningless. mkit needed a concrete
detection mechanism.

## Decision

Check six fixed items every time: **numbers and thresholds, money, personal
data, irreversible deletion, third-party calls, and permissions**.

These six items are a floor, not a ceiling. Five additional mid-work signals can
also stop the task.

For Tier B, false negatives cost much more than false positives. A missed stop
is discovered only when a customer complains; an extra stop costs about 30 seconds.

## Technical constraints

The checklist must be mechanically testable. A request containing an
irreversible deletion policy that does not stop is a failure.

Each item must include a plain-language question pattern so the agent does not
invent technical wording.

## Alternatives considered

1. **Let the agent interpret prose principles.** The upstream project does this
   successfully because developers can detect invented policy. Tier B users
   cannot, so copying that model transfers risk to the person least able to
   carry it.
2. **Always ask unless the request is allowlisted.** This creates noise. When
   obvious work triggers questions, users start approving without reading.

repository-harness decision 0019 states that sensitive terminology alone is not
an automatic approval gate when expected behavior is explicit. That conclusion
fits users who can express explicit behavior; Tier B users usually cannot.

## Tradeoffs

A fixed list misses policy outside the six categories. The five mid-work signals
reduce but do not eliminate that gap. mkit accepts it because high recall and
testability matter more than theoretical completeness.
