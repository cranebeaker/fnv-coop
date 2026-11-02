//! nv-call: call functions of a PE32 image on the real CPU.
//!
//! This is only the host. The work is done by `nv_call_engine.dll`, which
//! must sit next to this program. They are split for one reason: the image
//! being measured (the game exe) wants to live at 0x00400000, and that
//! range must be free when it is mapped. Every Windows process starts with
//! a stack, heaps and system data at unpredictable low addresses, so a
//! program that asks for 0x00400000 later may find it taken. This host is
//! linked at 0x00400000 with a large zero-filled placeholder, so the
//! operating system reserves the whole range before anything else, and the
//! engine (linked at 0x10000000) then replaces the placeholder with the
//! image. After that the host's own code no longer exists, which is why it
//! does nothing except hand over and why the engine ends the process itself.
//!
//! The host is built without the Rust runtime (`no_main`) so the Rust
//! runtime installs no handlers that point into its own code. The C startup
//! code still registers a top-level exception filter there, which is why the
//! engine replaces the filter before it overwrites anything.

#![no_main]

#[cfg(all(windows, target_arch = "x86"))]
mod host {
    include!(concat!(env!("OUT_DIR"), "/reserve.rs"));

    /// Zero-filled, so it takes no space in the file. Only its size matters.
    #[used]
    static mut PLACEHOLDER: [u8; RESERVE_BYTES] = [0; RESERVE_BYTES];

    #[no_mangle]
    pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
        // A reference the linker cannot drop, so the placeholder stays.
        // SAFETY: reads the first byte of a static array.
        let touched =
            unsafe { std::ptr::read_volatile(std::ptr::addr_of!(PLACEHOLDER) as *const u8) };
        if touched != 0 {
            return 1;
        }
        let engine = match std::env::current_exe() {
            Ok(p) => p.with_file_name("nv_call_engine.dll"),
            Err(e) => {
                eprintln!("nv-call: cannot find its own folder: {e}");
                return 1;
            }
        };
        let wide = nv_win::wide(engine.as_os_str());
        // SAFETY: loading a DLL by path and calling its one exported entry point.
        unsafe {
            let h = nv_win::LoadLibraryW(wide.as_ptr());
            if h.is_null() {
                eprintln!(
                    "nv-call: cannot load {}: {}",
                    engine.display(),
                    nv_win::error_text(nv_win::GetLastError())
                );
                return 1;
            }
            let f = nv_win::GetProcAddress(h, c"nv_call_run".as_ptr().cast());
            if f.is_null() {
                eprintln!("nv-call: {} does not export nv_call_run", engine.display());
                return 1;
            }
            // The engine is told where the placeholder is, so it knows which
            // part of this program's range it may give to the image.
            let run: extern "C" fn(u32, u32) -> i32 = std::mem::transmute(f);
            run(
                std::ptr::addr_of!(PLACEHOLDER) as usize as u32,
                RESERVE_BYTES as u32,
            )
        }
    }
}

#[cfg(not(all(windows, target_arch = "x86")))]
#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    eprintln!("nv-call only runs as a 32-bit Windows program (target i686-pc-windows-gnu or -msvc); see README.md");
    2
}
