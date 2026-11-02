//! The test fixture is linked at a fixed base without relocation, so the function list it
//! prints stays valid for the run.

fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    if os == "windows" && arch == "x86" && env == "msvc" {
        println!("cargo:rustc-link-arg-bin=nv-trace-fixture=/BASE:0x00400000");
        println!("cargo:rustc-link-arg-bin=nv-trace-fixture=/DYNAMICBASE:NO");
        println!("cargo:rustc-link-arg-bin=nv-trace-fixture=/FIXED");
    }
}
