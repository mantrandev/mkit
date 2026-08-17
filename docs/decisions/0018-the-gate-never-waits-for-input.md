# 0018 The gate never waits for input

Date: 2026-08-17

## Status

Accepted

## Context

Decision 0016 arms the gate through the plugin and specifies the trigger
commands. Those commands were correct, but the program behind them had a fault
that only appears outside the tool.

`check` reads standard input to recognise a decision being written, and it did
that whenever standard input was not a terminal. Under the trigger that is safe,
because the tool sends its payload and closes the stream. Anywhere else — a
shell script, a scheduled job, any command whose input is an open pipe that
nobody writes to — the program waits for input that never arrives and never
returns.

That is the worst shape a fault can take here. It does not fail, it does not
report, and it does not stop. It was found by running the released program in an
ordinary script, and no test caught it, because a Rust test closes the child's
input by default and so never reproduces the condition.

## Decision

The gate never waits for input.

It reads standard input only when the trigger explicitly says the payload is
there. Run any other way, it reads nothing and returns immediately.

Everything decision 0016 settled still holds. Installing the plugin arms the
gate, the user configures nothing, a project without mkit installed is
unaffected, the trigger never converts a refusal into success, and the trigger
names the project explicitly rather than letting the program infer it.

This decision supersedes 0016.

## Technical constraints

`check` accepts one optional argument, `--from-hook`, and reads standard input
only when it is present. Without it, the decision exemption cannot apply, which
fails in the safe direction: work is refused rather than allowed.

No code path may read standard input unconditionally. Any future reader must be
gated behind an explicit flag from the caller that guarantees the stream is
delivered and closed.

The trigger commands become:

```
turn:  R="${CLAUDE_PROJECT_DIR:-.}"; [ -x "$R/.mkit/bin/mkit-gate" ] && MKIT_ROOT="$R" "$R/.mkit/bin/mkit-gate" turn >/dev/null 2>&1; exit 0
check: R="${CLAUDE_PROJECT_DIR:-.}"; [ -x "$R/.mkit/bin/mkit-gate" ] || exit 0; MKIT_ROOT="$R" exec "$R/.mkit/bin/mkit-gate" check --from-hook
```

A test must spawn `check` with an open input stream that is never written and
never closed, and fail if the program has not exited within a bounded wait.
Tests that let the runtime close the child's input do not cover this.

Requirements carried forward from 0016: `turn` always reports success; `check`
uses `exec` so the exit code reaches the tool unchanged; appending `|| true` to
`check` is forbidden; the trigger passes `MKIT_ROOT`; no installer writes into
any file the user owns; and the four guard states are verified before release.

## Alternatives considered

1. **Read input with a timeout on a background thread.** Keeps the exemption
   working however the program is invoked, but makes behaviour depend on timing
   and leaves a thread waiting on a stream that may never close.
2. **Stop reading input entirely and drop the decision exemption.** Removes the
   fault completely, but restores the deadlock decision 0016 fixed: the gate
   would again block the very file needed to satisfy it.
3. **Detect readable input before reading.** No portable way to do this with the
   standard library alone, and a wrong answer reintroduces the hang.

## Tradeoffs

Anyone still running the trigger shipped before this decision keeps a working
gate but loses the decision exemption, so writing a decision record is refused
until they update the plugin. The refusal is visible and explains what to do, so
the failure is recoverable rather than silent.

`check` run by hand can no longer recognise a decision being written. That path
is for diagnosis, where refusing is the correct answer anyway.
