# 0015 macOS is the supported platform

Date: 2026-08-17

## Status

Accepted

## Context

Decision 0013 settled how the program reaches users and committed the release
pipeline to five platform builds, including Windows. That list was chosen before
anyone knew who the users were.

They are now known. mkit users build web and app projects by describing what
they want, and they work on macOS. Nobody has asked for Windows, and no Windows
user has ever installed mkit.

Every published platform is a promise. A Windows build that nobody runs is a
promise that will break silently: the installer path for it was never written,
so the build exists in the release while `install.sh` refuses to run on that
system. Shipping a file that cannot be installed is worse than shipping nothing.

## Decision

macOS is the platform mkit supports, on both Apple Silicon and Intel.

Windows is dropped. No Windows build is produced, and the installer says plainly
that the system is not supported rather than implying a path that does not
exist.

Linux builds continue to be published because they already work and cost almost
nothing to produce, but Linux is not a supported platform and is not tested with
real users. It carries no promise.

Everything decision 0013 settled about how builds reach users still holds: they
are published through GitHub Releases in the same place the source lives, they
are not signed with a paid certificate, the installer verifies a published
checksum before installing anything, and a verification failure aborts the
installation and leaves nothing behind.

This decision supersedes 0013.

## Technical constraints

Release builds are produced for `aarch64-apple-darwin`,
`x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, and
`aarch64-unknown-linux-gnu`. No `x86_64-pc-windows-msvc` target is built,
released, or checksummed.

Continuous integration runs on macOS and Linux. Windows is removed from the
matrix, because testing a platform that is never shipped spends time on a
promise nobody made.

`install.sh` resolves the platform from `uname` and refuses any system other
than Darwin and Linux with a message naming macOS as the supported platform.

Releases published before this decision keep the assets they already carry.
History is not rewritten.

## Alternatives considered

1. **Keep publishing the Windows build.** Costs one CI job, but leaves an
   installable-looking file that the installer refuses, which is a trap rather
   than a feature.
2. **Write the missing Windows installer path.** Real support instead of a
   half-promise, but it means a PowerShell installer, a second hook format, and
   a platform nobody has asked for.
3. **Drop Linux as well.** Matches the instruction most literally, but removes a
   working path for no gain; the cost of keeping it is one CI job.

## Tradeoffs

A Windows user who finds mkit now cannot install it at all, and there is no
partial path for them. mkit accepts losing that audience entirely rather than
maintaining a second platform for people who have not asked.

Linux sits in an honest but uncomfortable middle: builds exist, nothing is
promised, and a Linux problem will not be treated as a release blocker.
