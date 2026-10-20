//! Lumen Motion's own file types, and the extensions of the files it still opens from the program
//! it grew out of. Saving always writes the Lumen Motion extension; a file opened with an older
//! extension is saved under the same name with the new one, next to it (the original is never
//! overwritten).
//!
//! | What | Extension | Older files still opened |
//! |---|---|---|
//! | Project (JSON) | `.lumeneffects` | `.ecproj` |
//! | Project, XML copy (File ▸ Save a Copy As XML) | `.lumeneffectsx` | `.ecprojx` |
//! | Template (Essential Graphics) | `.lumeneffectstemplate` | `.ectemplate` |
//! | Animation, effect and easing preset | `.lumeneffectspreset` | `.ecpreset` |

use std::path::Path;

/// A project file: the default in every save dialog and the first in every open dialog.
pub const PROJECT_EXT: &str = "lumeneffects";
/// A project saved as XML (File ▸ Save a Copy As XML).
pub const PROJECT_XML_EXT: &str = "lumeneffectsx";
/// A template made in Essential Graphics (File ▸ Export ▸ Template).
pub const TEMPLATE_EXT: &str = "lumeneffectstemplate";
/// An animation preset (Animation ▸ Save Animation Preset), effect presets and easing presets.
pub const PRESET_EXT: &str = "lumeneffectspreset";

/// The older project extension, still opened.
pub const LEGACY_PROJECT_EXT: &str = "ecproj";
/// The older XML project extension, still opened.
pub const LEGACY_PROJECT_XML_EXT: &str = "ecprojx";
/// The older template extension, still opened.
pub const LEGACY_TEMPLATE_EXT: &str = "ectemplate";
/// The older preset extension, still opened.
pub const LEGACY_PRESET_EXT: &str = "ecpreset";

/// Every project extension Lumen Motion opens, its own first (open dialogs list them in this order).
pub const PROJECT_OPEN_EXTS: &[&str] = &[PROJECT_EXT, PROJECT_XML_EXT, LEGACY_PROJECT_EXT, LEGACY_PROJECT_XML_EXT];
/// Every template extension Lumen Motion opens, its own first.
pub const TEMPLATE_OPEN_EXTS: &[&str] = &[TEMPLATE_EXT, LEGACY_TEMPLATE_EXT];
/// Every preset extension Lumen Motion opens, its own first (plain JSON presets too).
pub const PRESET_OPEN_EXTS: &[&str] = &[PRESET_EXT, LEGACY_PRESET_EXT, "json"];

/// The default name of a new project's file.
pub const UNTITLED_PROJECT: &str = "Untitled Project.lumeneffects";

/// The format name written at the top of every project file Lumen Motion saves (`"format"`).
pub const PROJECT_FORMAT: &str = "lumeneffects";
/// The version of the project file format (`"formatVersion"`). A file whose version is higher was
/// written by a newer Lumen Motion and is refused with a clear message instead of being opened
/// half-understood. Raise it only when older versions can no longer read the files.
pub const PROJECT_FORMAT_VERSION: u32 = 1;

fn ext_of(path: &str) -> Option<String> {
    Path::new(path).extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase)
}

/// `path` names a project file, of this version or an older one (any letter case).
pub fn is_project_path(path: &str) -> bool {
    ext_of(path).is_some_and(|e| PROJECT_OPEN_EXTS.contains(&e.as_str()))
}

/// `path` names a template file, of this version or an older one.
pub fn is_template_path(path: &str) -> bool {
    ext_of(path).is_some_and(|e| TEMPLATE_OPEN_EXTS.contains(&e.as_str()))
}

/// `path` uses an extension from before Lumen Motion (`.ecproj`, `.ecprojx`, `.ectemplate`,
/// `.ecpreset`).
pub fn is_legacy_path(path: &str) -> bool {
    ext_of(path).is_some_and(|e| [LEGACY_PROJECT_EXT, LEGACY_PROJECT_XML_EXT, LEGACY_TEMPLATE_EXT, LEGACY_PRESET_EXT].contains(&e.as_str()))
}

/// Where a file opened from `path` is saved: the same path, with an older extension replaced by
/// Lumen Motion's (`Intro.ecproj` → `Intro.lumeneffects`, `Intro.ecprojx` → `Intro.lumeneffectsx`).
/// Other paths come back unchanged.
pub fn save_path_for(path: &str) -> String {
    let Some(ext) = ext_of(path) else { return path.to_string() };
    let new = match ext.as_str() {
        LEGACY_PROJECT_EXT => PROJECT_EXT,
        LEGACY_PROJECT_XML_EXT => PROJECT_XML_EXT,
        LEGACY_TEMPLATE_EXT => TEMPLATE_EXT,
        LEGACY_PRESET_EXT => PRESET_EXT,
        _ => return path.to_string(),
    };
    // The extension is the last `ext.len()` bytes (ASCII), after a dot.
    let stem = path.len().saturating_sub(ext.len());
    match path.get(..stem) {
        Some(head) if head.ends_with('.') => format!("{head}{new}"),
        _ => path.to_string(),
    }
}

/// The backup of the previous version of `path`, kept when a save replaces it (`Intro.lumeneffects`
/// → `Intro.lumeneffects.bak`).
pub fn backup_path_for(path: &str) -> String {
    format!("{path}.bak")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projects_of_both_generations_are_recognised() {
        for p in ["a.lumeneffects", "B.LUMENEFFECTS", "c.lumeneffectsx", "/x/d.ecproj", r"C:\w\E.ECPROJX"] {
            assert!(is_project_path(p), "{p}");
        }
        for p in ["a.lumeneffects.bak", "a.json", "a", "a.lumeneffectstemplate", "a.ecproj.png"] {
            assert!(!is_project_path(p), "{p}");
        }
        assert!(is_template_path("t.lumeneffectstemplate") && is_template_path("t.ECTEMPLATE") && !is_template_path("t.lumeneffects"));
    }

    #[test]
    fn older_files_are_saved_with_the_new_extension() {
        assert_eq!(save_path_for("/w/Intro.ecproj"), "/w/Intro.lumeneffects");
        assert_eq!(save_path_for(r"C:\w\Intro.ECPROJ"), r"C:\w\Intro.lumeneffects");
        assert_eq!(save_path_for("Intro.ecprojx"), "Intro.lumeneffectsx");
        assert_eq!(save_path_for("Lower.ectemplate"), "Lower.lumeneffectstemplate");
        assert_eq!(save_path_for("Bounce.ecpreset"), "Bounce.lumeneffectspreset");
        assert_eq!(save_path_for("Intro.lumeneffects"), "Intro.lumeneffects");
        assert_eq!(save_path_for("no-extension"), "no-extension");
        assert_eq!(save_path_for("ñandú.ecproj"), "ñandú.lumeneffects");
        assert!(is_legacy_path("x.ecproj") && !is_legacy_path("x.lumeneffects"));
    }

    #[test]
    fn the_new_extensions_come_first_in_open_dialogs() {
        assert_eq!(PROJECT_OPEN_EXTS.first(), Some(&PROJECT_EXT));
        assert_eq!(TEMPLATE_OPEN_EXTS.first(), Some(&TEMPLATE_EXT));
        assert_eq!(PRESET_OPEN_EXTS.first(), Some(&PRESET_EXT));
        assert!(UNTITLED_PROJECT.ends_with(".lumeneffects"));
        assert_eq!(backup_path_for("a.lumeneffects"), "a.lumeneffects.bak");
    }
}
