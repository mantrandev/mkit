#!/usr/bin/env bash
set -euo pipefail

REPO_URL="${MKIT_REPO:-https://github.com/mantrandev/mkit.git}"
REPO_BRANCH="${MKIT_BRANCH:-main}"
TARGET="${1:-$PWD}"

say() { printf '%s\n' "$*"; }
die() { printf 'mkit: %s\n' "$*" >&2; exit 1; }

[ -d "$TARGET" ] || die "directory not found: $TARGET"
TARGET="$(cd "$TARGET" && pwd)"

command -v git >/dev/null 2>&1 || die "git is required"
command -v curl >/dev/null 2>&1 || die "curl is required"

if command -v sha256sum >/dev/null 2>&1; then
  digest_of() { sha256sum "$1" | cut -d' ' -f1; }
elif command -v shasum >/dev/null 2>&1; then
  digest_of() { shasum -a 256 "$1" | cut -d' ' -f1; }
else
  die "sha256sum or shasum is required to verify the download"
fi

case "$(uname -s)" in
  Darwin)
    case "$(uname -m)" in
      arm64) GATE_TARGET=aarch64-apple-darwin ;;
      x86_64) GATE_TARGET=x86_64-apple-darwin ;;
      *) die "unsupported macOS architecture: $(uname -m)" ;;
    esac
    ;;
  Linux)
    case "$(uname -m)" in
      x86_64) GATE_TARGET=x86_64-unknown-linux-gnu ;;
      aarch64 | arm64) GATE_TARGET=aarch64-unknown-linux-gnu ;;
      *) die "unsupported Linux architecture: $(uname -m)" ;;
    esac
    ;;
  *)
    die "unsupported system: $(uname -s). Install through the Claude Code or Codex plugin instead."
    ;;
esac

if [ -d "$TARGET/.git" ]; then
  IN_GIT=yes
else
  IN_GIT=no
fi

SRC="$(mktemp -d)"
trap 'rm -rf "$SRC"' EXIT

say "mkit: downloading…"
git clone --depth 1 --branch "$REPO_BRANCH" --quiet "$REPO_URL" "$SRC/mkit" \
  || die "could not download $REPO_URL — if the repository is private, run: gh repo clone mantrandev/mkit"

SRC="$SRC/mkit"

install_block() {
  local block_file="$1" dest="$2"
  local block
  block="$(cat "$block_file")"

  if [ ! -f "$dest" ]; then
    printf '# Project Rules\n\n%s\n' "$block" > "$dest"
    say "mkit: created $(basename "$dest")"
    return
  fi

  if grep -q '<!-- MKIT:BEGIN -->' "$dest"; then
    awk -v blockfile="$block_file" '
      /<!-- MKIT:BEGIN -->/ { while ((getline line < blockfile) > 0) print line; skip=1; next }
      /<!-- MKIT:END -->/   { skip=0; next }
      !skip { print }
    ' "$dest" > "$dest.mkit.tmp"
    mv "$dest.mkit.tmp" "$dest"
    say "mkit: updated block in $(basename "$dest")"
  else
    printf '\n%s\n' "$block" >> "$dest"
    say "mkit: appended block to $(basename "$dest")"
  fi
}

VERSION="$(sed -n 's/.*"version"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' "$SRC/.claude-plugin/plugin.json" | head -1)"
[ -n "$VERSION" ] || die "cannot read the mkit version"

ASSET="mkit-gate-${GATE_TARGET}"
BASE="https://github.com/mantrandev/mkit/releases/download/v${VERSION}"
STAGE="$SRC/gate"
mkdir -p "$STAGE"

say "mkit: downloading the gate for ${GATE_TARGET}"
curl -fsSL -o "$STAGE/$ASSET" "$BASE/$ASSET" \
  || die "release v${VERSION} has no build for ${GATE_TARGET} - nothing was installed"
curl -fsSL -o "$STAGE/SHA256SUMS" "$BASE/SHA256SUMS" \
  || die "release v${VERSION} has no checksum file - nothing was installed"

EXPECTED="$(awk -v name="$ASSET" '$2 == name || $2 == "*" name { print $1 }' "$STAGE/SHA256SUMS" | head -1)"
[ -n "$EXPECTED" ] || die "${ASSET} is not listed in the release checksums - nothing was installed"

ACTUAL="$(digest_of "$STAGE/$ASSET")"
if [ "$ACTUAL" != "$EXPECTED" ]; then
  die "the downloaded gate does not match its checksum - nothing was installed"
fi
say "mkit: verified the gate download"

mkdir -p "$TARGET/docs/decisions" "$TARGET/docs/active" "$TARGET/docs/done"
say "mkit: created docs/decisions, docs/active, docs/done"

install_block "$SRC/core/AGENTS.block.md" "$TARGET/AGENTS.md"
install_block "$SRC/core/CLAUDE.block.md" "$TARGET/CLAUDE.md"

mkdir -p "$TARGET/docs/templates"
cp "$SRC/core/templates/active.md" "$TARGET/docs/templates/active.md"
cp "$SRC/core/templates/architecture.md" "$TARGET/docs/templates/architecture.md"
cp "$SRC/core/templates/decision.md" "$TARGET/docs/templates/decision.md"
cp "$SRC/core/templates/spec.md" "$TARGET/docs/templates/spec.md"

if [ ! -f "$TARGET/spec.md" ]; then
  cp "$SRC/core/templates/spec.md" "$TARGET/spec.md"
  say "mkit: created spec.md"
else
  say "mkit: kept existing spec.md"
fi

rm -rf "$TARGET/.mkit/workflows"
mkdir -p "$TARGET/.mkit/workflows"
count=0
for dir in "$SRC/skills"/*/; do
  name="$(basename "$dir")"
  [ "$name" = init ] && continue
  cp "$dir/SKILL.md" "$TARGET/.mkit/workflows/$name.md"
  count=$((count + 1))
done
say "mkit: installed $count workflows into .mkit/workflows"

mkdir -p "$TARGET/.mkit/bin"
cp "$STAGE/$ASSET" "$TARGET/.mkit/bin/mkit-gate"
chmod +x "$TARGET/.mkit/bin/mkit-gate"
say "mkit: verified and installed the gate into .mkit/bin"

mkdir -p "$TARGET/.mkit/hooks"
cp "$SRC/hooks/claude-code.json" "$TARGET/.mkit/hooks/claude-code.json"

printf 'bin/\ngate/\n' > "$TARGET/.mkit/.gitignore"

say ""
say "Done. Talk to your agent in plain language:"
say ""
say "  'plan X'                discuss a change without building it"
say "  'build X for me'        implement it"
say "  'this is broken'        fix a bug"
say "  'where did we stop?'    resume unfinished work"
say "  'I do not understand'   explain it differently"
say ""
say "One step is left, and only you can do it. The gate is installed but not armed."
say "In Claude Code, merge this file into .claude/settings.json:"
say ""
say "  .mkit/hooks/claude-code.json"
say ""
say "Until you do, the agent can still change files without answering your questions."
say ""
say "For slash commands in Claude Code, install the plugin:"
say "  /plugin marketplace add mantrandev/mkit"
say "  /plugin install mkit@mkit"
say ""

if [ "$IN_GIT" = no ]; then
  say "Note: this directory is not a git repository. Run 'git init' before making changes;"
  say "without git there is no reliable rollback path."
fi
