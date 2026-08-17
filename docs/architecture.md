# Architecture

Updated: 2026-08-17
Profile: Documentation-first plugin and installer, with one compiled gate

## System shape

mkit has one source rule block, seven source skills, project-document templates,
two plugin manifests, and one compiled program. The installer distributes the
rule block, templates, and workflow copies into a target repository. Plugin
clients load the source skills directly and use `mkit:init` to install
project-local records and rules.

`mkit-gate` is the only compiled component. It holds no rule text; it refuses
file edits until the authority gate has been declared for the current request.
Per-agent triggers in `hooks/` invoke it, because a program the agent may choose
to call is still only an instruction.

## Modules and ownership

| Module or path | Owns | May depend on |
| --- | --- | --- |
| `core/AGENTS.block.md` | Installed agent rules | Stable document and workflow contracts |
| `skills/` | Source workflows for agent actions | Agent rules and installed project records |
| `core/templates/` | Templates distributed to target repositories | Document contracts in the agent rules |
| `install.sh` | Non-plugin installation and refresh | `core/`, `skills/` |
| `.codex-plugin/` and `.claude-plugin/` | Client discovery and release metadata | Source skills |
| `crates/mkit-gate/` | Request lifecycle and the refusal to edit files before the gate runs | Nothing inside mkit |
| `hooks/` | Per-agent triggers that invoke `mkit-gate` at the tool layer | `crates/mkit-gate/` |
| `docs/` and `spec.md` | Current product truth, architecture, work, and rationale | Source behavior |

## Dependency rules

- Edit source workflows only in `skills/`; `.mkit/workflows/` is generated in target repositories.
- Keep the marked block in root `AGENTS.md` byte-equivalent to `core/AGENTS.block.md`.
- Keep each file in `docs/templates/` byte-equivalent to its source in `core/templates/`.
- Keep plugin manifests pointed at source skills; never create client-specific workflow forks.
- Treat `install.sh` as a distributor, not as a second source of workflow behavior.
- Never copy rule text into `crates/mkit-gate/`; the rule sources stay in `core/`.
- Every `mkit-gate` failure path fails closed, so an unusable state blocks edits rather than allowing them.

## Cross-boundary flows

- Plugin install → client manifest → source skill → `mkit:init` → project-local rules and records
- Shell install → `install.sh` → core rule and templates plus source skills → target repository
- Source behavior change → validation → client version bump → plugin update

## Verification

- `bash -n install.sh` — the installer parses as shell.
- `python3 "${CODEX_HOME:-$HOME/.codex}/skills/.system/skill-creator/scripts/quick_validate.py" skills/<name>` — a source skill has valid metadata.
- `python3 "${CODEX_HOME:-$HOME/.codex}/skills/.system/plugin-creator/scripts/validate_plugin.py" .` — the Codex plugin is valid.
- `claude plugin validate .` — the Claude plugin and marketplace are valid.
- `git diff --check` — edited files contain no whitespace errors.
- `cargo test --release` — the gate blocks, permits, and fails closed as specified.
- `cargo fmt --check` and `cargo clippy -- -D warnings` — the gate source is formatted and lint-clean.

## Known gaps

- No automated boundary checker currently verifies source-to-installed mirror parity.
- Agent behavior still requires forward-testing in isolated target repositories.
