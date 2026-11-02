//! nv-inject: load a DLL into a new or running 32-bit process.
//!
//!   nv-inject [--dll <path>] [--wait] --launch <exe> [args...]
//!   nv-inject [--dll <path>] --pid <n>
//!
//! The DLL is loaded with `CreateRemoteThread(LoadLibraryW)`. With
//! `--launch` the program is created suspended, the DLL is loaded, and then
//! the program is resumed, so a probe in `mode = inject` has installed its
//! hooks before the program runs.

#[cfg(any(windows, test))]
mod cmdline;

#[cfg(windows)]
mod app;

#[cfg(windows)]
fn main() {
    std::process::exit(app::main());
}

#[cfg(not(windows))]
fn main() {
    eprintln!("nv-inject only runs on Windows; see README.md");
    std::process::exit(2);
}
