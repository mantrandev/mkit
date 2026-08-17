# 0016 Installing the plugin arms the gate

Date: 2026-08-17

## Status

Superseded by 0018

## Context

The gate is the reason mkit exists, and it shipped switched off. The installer
put the program in place and then printed a last line asking the user to merge a
file into their Claude Code settings themselves.

mkit is written for a user who cannot read code. That user cannot merge JSON
into a configuration file, so for exactly the person the product is for, the
central feature never turned on.

The work these users do makes it worse. They build web and app projects by
describing what they want, and that work touches the gate constantly: sign-up
stores personal data, plans and prices are money, spam limits are numbers,
payment and login providers are third-party calls, a delete button is
irreversible. The gate would fire on almost every feature, and it was asleep.

Wiring the hook from the installer was rejected earlier for a good reason: it
edits a file outside anything the user described, and merging JSON in shell
without `jq` risks destroying hooks they already have.

## Decision

Installing the mkit plugin arms the gate. The user does nothing extra, and the
installer no longer asks them to.

The trigger is written so a project without mkit is unaffected. If the gate
program is not present and runnable in that project, the trigger does nothing
and reports success. It never edits any file the user owns.

The gate program itself is still installed by `install.sh`, so a project only
becomes protected once mkit has been installed into it.

## Technical constraints

The plugin ships its triggers. They are guarded, and the guard must never
swallow a refusal:

```
turn:  R="${CLAUDE_PROJECT_DIR:-.}"; [ -x "$R/.mkit/bin/mkit-gate" ] && MKIT_ROOT="$R" "$R/.mkit/bin/mkit-gate" turn >/dev/null 2>&1; exit 0
check: R="${CLAUDE_PROJECT_DIR:-.}"; [ -x "$R/.mkit/bin/mkit-gate" ] || exit 0; MKIT_ROOT="$R" exec "$R/.mkit/bin/mkit-gate" check
```

`turn` always reports success, because a trigger on user input that fails would
block the user from speaking. `check` uses `exec` so the program's own exit code
reaches the tool layer unchanged; a refusal must stay a refusal.

The trigger names the project explicitly through `MKIT_ROOT` instead of letting
the program infer it from the working directory. Without that, the two disagree
whenever the working directory is not the project: opening a request silently
does nothing, every later check refuses because no request was ever opened, and
the user can never edit a file again with no message explaining why.

Appending `|| true` to `check` is forbidden. It converts the refusal exit code
into success, disabling the gate while leaving every visible sign that it works.

Four states are required to behave as follows, and are verified before release:

| State | `turn` | `check` |
| --- | --- | --- |
| No program in the project | 0 | 0 |
| Program present, gate not declared | 0 | refuse |
| Program present, gate declared | 0 | 0 |
| Program present but not executable | 0 | 0 |

No installer writes into `.claude/settings.json` or any other file the user owns.

## Alternatives considered

1. **Keep asking the user to merge the file.** Costs nothing to maintain and
   leaves the product's central feature switched off for its intended user.
2. **Have `install.sh` merge the settings file.** Arms the gate without the
   plugin, but edits a file outside the request and can destroy existing hooks
   when `jq` is unavailable.
3. **Ship the trigger unguarded.** Simpler, but every project without mkit
   installed would report a missing command on every file edit.

## Tradeoffs

The trigger is registered for the user's whole Claude Code installation, so it
runs in every project on their machine, including projects that have nothing to
do with mkit. In those projects it exits immediately and does nothing, but it is
still one extra process for each file edit and each message sent.

mkit accepts that cost because the alternative was a safety feature that only
people who can read code were able to turn on.
