//! The platform's side of `ATIMGPUD.dll`, AMD's multi-GPU query, which the game ships in its
//! folder. `Renderer::Init`'s multi-GPU probe for the vendor selector 2 (`004da670`) loads it,
//! finds `AtiQueryMgpuCount`, calls it and frees it. The DLL asks AMD's driver, which the
//! platform can't run, so every answer comes from the machine table: `scripts/nvapi-capture`
//! loads the DLL as the game does (the game folder's copy first, as Windows searches the
//! program's folder first) in a 32-bit process and records the answers (lines `Ati*` of
//! `d3d9_adapter.tsv`, git-ignored, never committed).
//!
//! - `LoadLibraryA("ATIMGPUD.dll")`: [`ATI_MODULE`] when the capture loaded it; 0 with the
//!   captured `GetLastError` when it did not.
//! - `GetProcAddress(module, "AtiQueryMgpuCount")`: [`QUERY_CODE`], or 0 (error 127,
//!   `ERROR_PROC_NOT_FOUND`) when the capture did not find it.
//! - `AtiQueryMgpuCount()` (no arguments): the captured answer.
//!
//! Without the `Ati*` lines, `LoadLibraryA("ATIMGPUD.dll")` stops the run and says how to
//! capture them.

use crate::{Engine, Ret};

/// `AtiQueryMgpuCount`'s code address (a value no exe code uses).
pub const QUERY_CODE: u32 = 0x7e05_0000;
/// The module handle `LoadLibraryA("ATIMGPUD.dll")` answers (a value no image uses).
pub const ATI_MODULE: u32 = 0x7e15_0000;

const ERROR_PROC_NOT_FOUND: u32 = 127;

/// The `Ati*` lines of the machine table.
#[derive(Debug, Default, Clone)]
pub struct Table {
    /// `LoadLibraryA`: found, and `GetLastError` when not.
    load: Option<(bool, u32)>,
    procedure: Option<bool>,
    count: Option<u32>,
}

impl Table {
    /// Takes one table line. Answers false when it is not an `Ati*` line.
    pub fn take_line(&mut self, f: &[&str]) -> bool {
        let hex = |i: usize| u32::from_str_radix(f[i], 16).expect("a hex word in an Ati line");
        match f[0] {
            "AtiLoad" => self.load = Some((hex(2) != 0, hex(3))),
            "AtiProc" => self.procedure = Some(hex(2) != 0),
            "AtiCall" => self.count = Some(hex(2)),
            _ => return false,
        }
        true
    }
}

fn table(e: &Engine) -> &Table {
    &e.kernel.graphics.ati
}

fn not_captured(what: &str) -> ! {
    panic!(
        "{what} was not captured: not provided; {}",
        super::nvapi::CAPTURE_HINT
    )
}

/// `LoadLibraryA("ATIMGPUD.dll")`'s answer.
pub fn load_library(e: &mut Engine) -> u32 {
    match table(e).load {
        Some((true, _)) => ATI_MODULE,
        Some((false, error)) => {
            e.kernel.last_error = error;
            0
        }
        None => not_captured("LoadLibraryA(\"ATIMGPUD.dll\")"),
    }
}

/// `GetProcAddress(ATI_MODULE, name)`.
pub fn proc_address(e: &mut Engine, name: &[u8]) -> u32 {
    if name != b"AtiQueryMgpuCount" {
        panic!(
            "GetProcAddress(ATIMGPUD.dll, {:?}) is not provided",
            String::from_utf8_lossy(name)
        );
    }
    match table(e).procedure {
        Some(true) => {
            e.register(QUERY_CODE, query_count);
            QUERY_CODE
        }
        Some(false) => {
            e.kernel.last_error = ERROR_PROC_NOT_FOUND;
            0
        }
        None => not_captured("GetProcAddress(ATIMGPUD.dll, AtiQueryMgpuCount)"),
    }
}

// Platform: replaces ATIMGPUD.DLL AtiQueryMgpuCount (found by name)
/// `AtiQueryMgpuCount()`: the captured answer (the number of GPUs the driver reports).
fn query_count(e: &mut Engine, _args: &[u32]) -> Ret {
    let Some(count) = table(e).count else {
        not_captured("AtiQueryMgpuCount()");
    };
    Ret {
        eax: count,
        ..Ret::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::platform::{d3d9, loader};

    /// TEST DATA, not a capture: the lines in `scripts/nvapi-capture`'s format.
    fn engine(lines: &str) -> Engine {
        let mut e = Engine::new();
        d3d9::load_adapter_table(&mut e, lines);
        e
    }

    fn load(e: &mut Engine) -> u32 {
        let name = e.mem.alloc(16);
        e.mem.write(name, b"ATIMGPUD.dll\0");
        loader::load_library_a(e, crate::Ptr::new(name))
    }

    /// The probe's calls on a machine whose capture loaded the game folder's DLL and found
    /// the function.
    #[test]
    fn the_probe_gets_the_captured_count() {
        let mut e = engine("AtiLoad\tgame\t1\t0\nAtiProc\t\t1\nAtiCall\t\t2\n");
        let module = load(&mut e);
        assert_eq!(module, ATI_MODULE);
        let name = e.mem.alloc(32);
        e.mem.write(name, b"AtiQueryMgpuCount\0");
        let query = loader::get_proc_address(&mut e, module, name);
        assert_eq!(query, QUERY_CODE);
        assert_eq!(e.call(query, &[]).u32(), 2);
        assert_eq!(loader::free_library(&mut e, module), 1);
    }

    #[test]
    fn a_dll_the_capture_did_not_find_is_not_loaded() {
        let mut e = engine("AtiLoad\t\t0\t7e\n");
        assert_eq!(load(&mut e), 0);
        assert_eq!(e.kernel.last_error, 0x7e);
    }

    #[test]
    fn a_procedure_the_capture_did_not_find_is_0() {
        let mut e = engine("AtiLoad\tgame\t1\t0\nAtiProc\t\t0\n");
        assert_eq!(proc_address(&mut e, b"AtiQueryMgpuCount"), 0);
        assert_eq!(e.kernel.last_error, ERROR_PROC_NOT_FOUND);
    }

    #[test]
    #[should_panic(expected = "was not captured")]
    fn without_the_lines_the_run_stops() {
        let mut e = engine("");
        load(&mut e);
    }
}
