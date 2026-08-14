# Architecture

Updated: YYYY-MM-DD
Profile: <Existing project style or a short descriptive name>

## System shape

<Describe the smallest useful view of the running system and its major boundaries.>

## Modules and ownership

| Module or path | Owns | May depend on |
| --- | --- | --- |
| `<path>` | <Responsibility> | <Allowed dependencies> |

## Dependency rules

- <A rule an implementation can check before adding a dependency.>

## Cross-boundary flows

- <Entry point> → <owner> → <external or persistence boundary>

## Verification

- `<command>` — <What this verifies>

## Known gaps

- <Missing automated check or intentionally unresolved boundary.>
