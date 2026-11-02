//! The MinGW linker warns that `DllMain` is stdcall-decorated; allow the
//! automatic fixup so the warning does not appear in every build.

fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    if os == "windows" && env == "gnu" {
        println!("cargo:rustc-link-arg-cdylib=-Wl,--enable-stdcall-fixup");
    }
}
