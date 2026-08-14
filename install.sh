#!/usr/bin/env bash
set -euo pipefail

REPO_URL="${MKIT_REPO:-https://github.com/mantrandev/MKit.git}"
REPO_BRANCH="${MKIT_BRANCH:-main}"
TARGET="${1:-$PWD}"

say() { printf '%s\n' "$*"; }
die() { printf 'mkit: %s\n' "$*" >&2; exit 1; }

[ -d "$TARGET" ] || die "không thấy thư mục: $TARGET"
TARGET="$(cd "$TARGET" && pwd)"

command -v git >/dev/null 2>&1 || die "cần git"

if [ -d "$TARGET/.git" ]; then
  IN_GIT=yes
else
  IN_GIT=no
fi

SRC="$(mktemp -d)"
trap 'rm -rf "$SRC"' EXIT

say "mkit: đang tải…"
git clone --depth 1 --branch "$REPO_BRANCH" --quiet "$REPO_URL" "$SRC/mkit" \
  || die "không tải được $REPO_URL — repo còn private thì chạy: gh repo clone mantrandev/MKit"

SRC="$SRC/mkit"

install_block() {
  local block_file="$1" dest="$2"
  local block
  block="$(cat "$block_file")"

  if [ ! -f "$dest" ]; then
    printf '# Project Rules\n\n%s\n' "$block" > "$dest"
    say "mkit: tạo $(basename "$dest")"
    return
  fi

  if grep -q '<!-- MKIT:BEGIN -->' "$dest"; then
    awk -v blockfile="$block_file" '
      /<!-- MKIT:BEGIN -->/ { while ((getline line < blockfile) > 0) print line; skip=1; next }
      /<!-- MKIT:END -->/   { skip=0; next }
      !skip { print }
    ' "$dest" > "$dest.mkit.tmp"
    mv "$dest.mkit.tmp" "$dest"
    say "mkit: cập nhật khối trong $(basename "$dest")"
  else
    printf '\n%s\n' "$block" >> "$dest"
    say "mkit: nối khối vào cuối $(basename "$dest")"
  fi
}

mkdir -p "$TARGET/docs/decisions" "$TARGET/docs/active" "$TARGET/docs/done"
say "mkit: tạo docs/decisions, docs/active, docs/done"

install_block "$SRC/core/AGENTS.block.md" "$TARGET/AGENTS.md"
install_block "$SRC/core/CLAUDE.block.md" "$TARGET/CLAUDE.md"

mkdir -p "$TARGET/docs/templates"
cp "$SRC/core/templates/active.md" "$TARGET/docs/templates/active.md"
cp "$SRC/core/templates/decision.md" "$TARGET/docs/templates/decision.md"

if [ ! -f "$TARGET/spec.md" ]; then
  cp "$SRC/core/templates/spec.md" "$TARGET/spec.md"
  say "mkit: tạo spec.md"
else
  say "mkit: spec.md đã có, giữ nguyên"
fi

rm -rf "$TARGET/.mkit/workflows"
mkdir -p "$TARGET/.mkit/workflows"
mkdir -p "$TARGET/.claude/skills"
count=0
for dir in "$SRC/skills"/*/; do
  name="$(basename "$dir")"
  cp "$dir/SKILL.md" "$TARGET/.mkit/workflows/$name.md"
  out="$TARGET/.claude/skills/mkit-$name"
  mkdir -p "$out"
  sed "s/^name: $name$/name: mkit-$name/" "$dir/SKILL.md" > "$out/SKILL.md"
  count=$((count + 1))
done
say "mkit: $count workflow vào .mkit/workflows (Codex, Pi, mọi agent)"
say "mkit: $count lệnh vào .claude/skills (Claude Code)"

say ""
say "Xong. Dùng được ở cả ba agent:"
say ""
say "  Claude Code   gõ /mkit-plan, /mkit-implement, /mkit-fix, /mkit-continue,"
say "                /mkit-ha, /mkit-init"
say ""
say "  Codex, Pi     nói bằng tiếng thường — 'bàn về X', 'làm cho tôi X',"
say "                'bị lỗi rồi', 'hôm trước làm tới đâu'. Agent đọc AGENTS.md"
say "                rồi tự tìm đúng workflow."
say ""

if [ "$IN_GIT" = no ]; then
  say "Lưu ý: thư mục này chưa phải repo git. Chạy 'git init' trước khi làm gì,"
  say "không có git thì không quay lui được."
fi
