//! The engine behind `nv-call`: maps a PE32 image and calls its functions
//! on the real CPU. `nv-call.exe` loads this DLL and calls [`nv_call_run`];
//! see `nv-call/src/main.rs` for why the work is split in two.

#![cfg(all(windows, target_arch = "x86"))]

mod call;
mod image;
mod run;

/// Entry point called by `nv-call.exe`. Reads the command line itself.
/// `placeholder` and `len` are the address and length of the zero-filled array
/// that the host reserved for the image.
///
/// Returns only if the target image was not mapped (usage or setup errors).
/// Once the image has replaced the host's pages the process ends here
/// instead, because the host's code no longer exists.
#[no_mangle]
pub extern "C" fn nv_call_run(placeholder: u32, len: u32) -> i32 {
    let code = run::main(image::Placeholder {
        start: placeholder,
        len,
    });
    if image::took_over_host() {
        use std::io::Write;
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().flush();
        // SAFETY: ends this process without running exit handlers, which
        // would otherwise call into the replaced host image.
        unsafe { nv_win::TerminateProcess(nv_win::GetCurrentProcess(), code as u32) };
    }
    code
}
