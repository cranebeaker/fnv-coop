//! The engine DLL is linked at 0x10000000, well away from the range the
//! mapped image occupies, and must not be moved by address randomisation.

fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    if os != "windows" || arch != "x86" {
        return;
    }
    match env.as_str() {
        "msvc" => {
            println!("cargo:rustc-link-arg-cdylib=/BASE:0x10000000");
            println!("cargo:rustc-link-arg-cdylib=/DYNAMICBASE:NO");
        }
        _ => {
            println!("cargo:rustc-link-arg-cdylib=-Wl,--image-base=0x10000000");
            println!("cargo:rustc-link-arg-cdylib=-Wl,--disable-dynamicbase");
            println!("cargo:rustc-link-arg-cdylib=-Wl,--enable-stdcall-fixup");
        }
    }
}
