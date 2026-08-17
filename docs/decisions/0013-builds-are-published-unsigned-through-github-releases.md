# 0013 Builds are published unsigned through GitHub Releases

Date: 2026-08-17

## Status

Accepted

## Context

Decision 0012 made a compiled program required for mkit to run but deliberately
left three items unchosen: who signs the macOS build, where builds are hosted,
and what the ledger may contain. Distribution cannot be built until the first
two are settled.

The signing question was raised on a false premise. The claim was that a
downloaded program is blocked by macOS with a message saying the developer
cannot be verified, and that avoiding it costs an Apple Developer ID at 99 USD
per year. Measurement on macOS 25.5 shows otherwise. A file fetched with `curl`
carries only `com.apple.provenance`, not `com.apple.quarantine`, and Gatekeeper
raises that refusal only for quarantined files. Quarantine is applied by web
browsers, not by `curl`. Rust also links macOS arm64 binaries with an ad-hoc
signature, so the program executes without any developer certificate.

The premise holds only for a user who downloads the program through a browser.
The mkit installer does not.

## Decision

mkit publishes its program through GitHub Releases, in the same place the source
already lives, and does not pay for a signing certificate.

The installer fetches the build for the user's platform, checks it against a
published checksum, and refuses to install anything that does not match. A user
who cannot obtain a valid build is told so plainly and is left with no partial
installation.

Users who download the program through a web browser instead of the installer
will meet the macOS refusal described above. That path is not supported, and
supporting it later means buying and maintaining a signing certificate, which is
a separate decision nobody has made.

## Technical constraints

Releases are produced by tag-triggered CI for `aarch64-apple-darwin`,
`x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`,
and `x86_64-pc-windows-msvc`. Each release carries one `SHA256SUMS` file
covering every archive.

`install.sh` resolves the platform from `uname`, downloads the matching archive
and `SHA256SUMS` from the release matching the installed mkit version, verifies
the digest with `shasum` or `sha256sum`, and installs the program into
`.mkit/bin/` inside the target repository. A verification failure aborts the
installation and leaves no program behind.

Nothing is uploaded to any service. The installer only downloads.

No Apple Developer ID, no notarization, and no code-signing step exist in the
release pipeline. Adding any of them is a new decision.

The ledger remains unchosen and unbuilt.

## Alternatives considered

1. **Buy an Apple Developer ID and notarize.** Removes the browser-download
   failure, but costs money every year and adds a signing step to every release
   for a path the installer never takes.
2. **Require users to build from source with `cargo install`.** No hosting and
   no signing at all, and locally built programs are never quarantined, but it
   forces a full Rust toolchain onto a user who cannot read code.
3. **Host builds somewhere other than GitHub.** No benefit; the source is
   already there, and a second host is a second thing that can disappear.

## Tradeoffs

A user who finds the program through the GitHub web interface and downloads it
by hand gets a macOS refusal with no way past it. mkit accepts that because the
supported path is the installer, and because avoiding it would cost money every
year for a case the product does not ask anyone to perform.

mkit also accepts that a release now depends on CI producing five builds. A
platform whose build fails has no installable mkit until it is fixed.
