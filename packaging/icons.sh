#!/usr/bin/env bash
# Regenerate every derived Lumen Motion icon from assets/app-icon/lumen-motion.svg (the master; a
# 512-unit full-bleed tile with rx=112) and its bolder variants for small sizes:
# lumen-motion-small.svg (32-40 px) and lumen-motion-tiny.svg (16-24 px); plus the document icon
# (lumen-motion-document.svg) that the Windows installer gives .lumeneffects files.
#
# Needs: resvg (brew install resvg / cargo install resvg). On macOS, iconutil also writes the
# .icns. The outputs are committed, so builds and packaging never need these tools.
#
#   packaging/icons.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/assets/app-icon"
SVG="$DIR/lumen-motion.svg"
SMALL="$DIR/lumen-motion-small.svg"
TINY="$DIR/lumen-motion-tiny.svg"
DOC="$DIR/lumen-motion-document.svg"
APP_ID="net.sifg.lumenmotion"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

command -v resvg >/dev/null || { echo "error: resvg not found (brew install resvg)" >&2; exit 1; }

# macOS icons follow Apple's grid: the tile body is 824 of 1024 px with a transparent margin, so
# the Dock shows it at the same size as other apps. 512 * 1024 / 824 = 636.27 units, margin 62.14.
mac() {
  sed 's/viewBox="0 0 512 512"/viewBox="-62.14 -62.14 636.27 636.27"/' "$1" >"$2"
  grep -q 'viewBox="-62.14' "$2" || { echo "error: unexpected viewBox in $1" >&2; exit 1; }
}
MAC="$TMP/mac.svg"
mac "$SVG" "$MAC"
mac "$SMALL" "$TMP/mac-small.svg"
mac "$TINY" "$TMP/mac-tiny.svg"

render() { resvg -w "$2" -h "$2" "$1" "$3" </dev/null; }
# The artwork for a size: the tiny variant up to 24 px, the small one up to 40 px, else the master.
pick() { if [ "$1" -le 24 ]; then echo "$TINY"; elif [ "$1" -le 40 ]; then echo "$SMALL"; else echo "$SVG"; fi; }
pick_mac() { if [ "$1" -le 24 ]; then echo "$TMP/mac-tiny.svg"; elif [ "$1" -le 40 ]; then echo "$TMP/mac-small.svg"; else echo "$MAC"; fi; }

render "$SVG" 1024 "$DIR/lumen-motion-1024.png"
# Runtime window/Dock icon on macOS (apps/lumen-motion/src/main.rs embeds it).
render "$MAC" 512 "$DIR/lumen-motion-macos-512.png"

# Linux hicolor theme. Windows and Linux use the full-bleed tile.
for s in 16 24 32 48 64 128 256 512; do
  mkdir -p "$DIR/hicolor/${s}x${s}/apps"
  render "$(pick "$s")" "$s" "$DIR/hicolor/${s}x${s}/apps/$APP_ID.png"
done
mkdir -p "$DIR/hicolor/scalable/apps"
cp "$SVG" "$DIR/hicolor/scalable/apps/$APP_ID.svg"

# Web app favicons and the README's icon.
cp "$SMALL" "$ROOT/apps/lumen-motion-web/web/favicon.svg"

render "$SVG" 240 "$ROOT/docs/lumen/icon.png"

# Windows .ico: the app and its documents.
ICO_PNGS=()
DOC_PNGS=()
for s in 16 20 24 32 40 48 64 128 256; do
  render "$(pick "$s")" "$s" "$TMP/ico-$s.png"
  render "$DOC" "$s" "$TMP/doc-$s.png"
  ICO_PNGS+=("$TMP/ico-$s.png")
  DOC_PNGS+=("$TMP/doc-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- ico "$DIR/lumen-motion.ico" "${ICO_PNGS[@]}")
(cd "$ROOT" && cargo run -q -p xtask -- ico "$DIR/lumen-motion-document.ico" "${DOC_PNGS[@]}")

# macOS .icns.
if command -v iconutil >/dev/null; then
  SET="$TMP/lumen-motion.iconset"
  mkdir -p "$SET"
  for s in 16 32 128 256 512; do
    render "$(pick_mac "$s")" "$s" "$SET/icon_${s}x${s}.png"
    render "$(pick_mac $((s * 2)))" $((s * 2)) "$SET/icon_${s}x${s}@2x.png"
  done
  iconutil -c icns -o "$DIR/lumen-motion.icns" "$SET"
else
  echo "warning: iconutil not found (macOS only); lumen-motion.icns not regenerated" >&2
fi
echo "icons written to $DIR"
