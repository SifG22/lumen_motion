//! Windows only: embed the app icon and version info (VERSIONINFO) into `lumen-motion.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `LUMEN_MOTION_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/lumen-motion.ico");
    println!("cargo:rerun-if-env-changed=LUMEN_MOTION_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/lumen-motion.ico")
        .set("ProductName", "Lumen Motion")
        .set("FileDescription", "Lumen Motion")
        .set("CompanyName", "SifG22")
        .set("LegalCopyright", "Copyright (c) 2026 SifG22 and the original authors (LICENSE-MIT). MIT OR Apache-2.0.")
        .set("OriginalFilename", "LumenMotion.exe")
        .set("InternalName", "lumen-motion");
    if let Err(e) = res.compile() {
        if std::env::var_os("LUMEN_MOTION_REQUIRE_WINRES").is_some() {
            panic!("embedding Windows resources failed: {e}");
        }
        println!("cargo:warning=lumen-motion.exe built without icon/version resources: {e}");
    }
}
