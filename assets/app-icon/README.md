# Lumen Motion app icon

A white **ball in motion** (three speed streaks behind it) with a **keyframe diamond** cut out of
it, on a rounded Lumen Motion-teal tile. Drawn by SifG22 in the same style as the Lumen Photo
(orange) and Lumen Video (violet) icons: simple, flat and readable on light and dark taskbars.

## Palette

| Colour | Hex | Used for |
|---|---|---|
| Lumen Motion teal, top | `#34d9c3` | the tile, a soft vertical fade… |
| Lumen Motion teal, bottom | `#0e9aa8` | …to this colour |
| White | `#ffffff` | the ball and the streaks |

## Geometry

- A 512-unit viewBox tile, rounded square `rx=112`, full bleed (no frame or border).
- The ball (r = 104) sits right of centre; the keyframe diamond is cut out of it, so the tile shows
  through. Three rounded streaks trail it on the left, the middle one longest.
- macOS renders (`lumen-motion.icns`, `lumen-motion-macos-512.png`) put the tile in Apple's grid
  (824 of 1024 px, transparent margin). Windows, Linux and the web use the full-bleed tile.

## Files

| File | What |
|---|---|
| `lumen-motion.svg` | master, used for 48 px and up (and the hicolor scalable icon) |
| `lumen-motion-small.svg` | bolder variant for 32–40 px: two thick streaks, a bigger diamond |
| `lumen-motion-tiny.svg` | plain variant for 16–24 px: one streak and the ball, no cut-out |
| `lumen-motion-document.svg` | document icon: a page with a folded corner carrying the tile |
| `lumen-motion-1024.png` | 1024 px render |
| `lumen-motion-macos-512.png` | runtime Dock icon on macOS (embedded by `apps/lumen-motion/src/main.rs`) |
| `lumen-motion.icns` | macOS bundle icon (`CFBundleIconFile`) |
| `lumen-motion.ico` | Windows icon, 16–256 px (embedded in `LumenMotion.exe` by `apps/lumen-motion/build.rs`) |
| `lumen-motion-document.ico` | Windows icon for `.lumeneffects`, `.lumeneffectstemplate` and `.lumeneffectspreset` files (installer) |
| `hicolor/<size>/apps/net.sifg.lumenmotion.png`, `hicolor/scalable/…svg` | Linux icon theme, used by `packaging/linux/net.sifg.lumenmotion.desktop`; the 256 px one is the runtime icon on Windows and Linux |

The web app's `apps/lumen-motion-web/web/favicon.svg` and the README's
`docs/lumen/icon.png` use the same artwork.

## Regenerate

Edit the SVGs, then run `packaging/icons.sh` (needs `resvg`; `iconutil` on macOS for the `.icns`).
Licence: [LICENSE.txt](LICENSE.txt).
