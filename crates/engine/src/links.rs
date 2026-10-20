//! Product name, publisher and links (shown in Help ▸, the About dialog, the start screen and the
//! header's buttons). Lumen Motion is published by SifG22: the web site is sifg.net and the code,
//! releases and issues live in the GitHub repository.

pub const APP_NAME: &str = "Lumen Motion";
pub const APP_SLUG: &str = "lumen-motion";
/// Who publishes Lumen Motion.
pub const PUBLISHER: &str = "SifG22";
/// The app version (`[workspace.package] version`, the same as the repository's `VERSION` file and
/// the release tag `vX.Y.Z`).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const WEBSITE: &str = "https://sifg.net";
pub const APP_PAGE: &str = "https://sifg.net";
pub const GITHUB: &str = "https://github.com/SifG22/lumen_motion";
pub const ISSUES: &str = "https://github.com/SifG22/lumen_motion/issues";
/// Every published version, with its installers and release notes.
pub const RELEASES: &str = "https://github.com/SifG22/lumen_motion/releases";
/// The update notice the desktop app reads (written by the release workflow).
pub const LATEST_JSON: &str = "https://github.com/SifG22/lumen_motion/releases/latest/download/latest.json";

#[cfg(test)]
mod tests {
    #[test]
    fn the_version_file_matches_the_crate_version() {
        let file = include_str!("../../../VERSION");
        assert_eq!(file.trim(), super::VERSION, "VERSION and [workspace.package] version must match (the release tag checks both)");
    }

    #[test]
    fn every_link_points_at_sifg_or_the_repository() {
        for url in [super::WEBSITE, super::APP_PAGE, super::GITHUB, super::ISSUES, super::RELEASES, super::LATEST_JSON] {
            assert!(url.starts_with("https://sifg.net") || url.starts_with("https://github.com/SifG22/lumen_motion"), "{url}");
        }
    }
}
