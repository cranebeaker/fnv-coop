//! Link the host at the base the mapped image wants (0x00400000) and size its
//! placeholder, so the operating system reserves that range before anything
//! else can land in it.
//!
//! `NV_CALL_RESERVE_MB` sets the placeholder size (default 24, which covers
//! 0x00400000 to 0x01C00000).

use std::io::Write;

fn main() {
    println!("cargo:rerun-if-env-changed=NV_CALL_RESERVE_MB");
    let mb: usize = std::env::var("NV_CALL_RESERVE_MB")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(24);
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
    let mut f = std::fs::File::create(out.join("reserve.rs")).expect("create reserve.rs");
    writeln!(f, "pub const RESERVE_BYTES: usize = {};", mb * 1024 * 1024)
        .expect("write reserve.rs");

    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    if os != "windows" || arch != "x86" {
        return;
    }
    match env.as_str() {
        "msvc" => {
            println!("cargo:rustc-link-arg-bins=/BASE:0x00400000");
            println!("cargo:rustc-link-arg-bins=/DYNAMICBASE:NO");
            println!("cargo:rustc-link-arg-bins=/LARGEADDRESSAWARE");
            // rustc asks for a safe-handler table on x86. The loader records
            // that table for the address range of this program when it starts,
            // and that range is where the image will be mapped: every handler
            // of the mapped code would be missing from it and be refused. This
            // argument comes after rustc's own and replaces it. And do not
            // opt in to DEP: a handler in memory that is not part of an image
            // (an image mapped outside the placeholder) is refused when DEP is
            // on for the process.
            println!("cargo:rustc-link-arg-bins=/SAFESEH:NO");
            println!("cargo:rustc-link-arg-bins=/NXCOMPAT:NO");
        }
        _ => {
            println!("cargo:rustc-link-arg-bins=-Wl,--image-base=0x400000");
            println!("cargo:rustc-link-arg-bins=-Wl,--disable-dynamicbase");
            println!("cargo:rustc-link-arg-bins=-Wl,--large-address-aware");
            // Same reason as /NXCOMPAT:NO for MSVC above.
            println!("cargo:rustc-link-arg-bins=-Wl,--disable-nxcompat");
        }
    }
}
