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

say ""
say "Done. Talk to your agent in plain language:"
say ""
say "  'plan X'                discuss a change without building it"
say "  'build X for me'        implement it"
say "  'this is broken'        fix a bug"
say "  'where did we stop?'    resume unfinished work"
say "  'I do not understand'   explain it differently"
say ""
say "For slash commands in Claude Code, install the plugin:"
say "  /plugin marketplace add mantrandev/mkit"
say "  /plugin install mkit@mkit"
say ""

if [ "$IN_GIT" = no ]; then
  say "Note: this directory is not a git repository. Run 'git init' before making changes;"
  say "without git there is no reliable rollback path."
fi
