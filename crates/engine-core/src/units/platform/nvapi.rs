//! The platform's side of `nvapi.dll` (the maintainer's decision, 2026-10-22): the game's
//! statically linked NvAPI start-up (`009fea2a`) loads the DLL, finds `nvapi_QueryInterface` and
//! asks it for functions by id; `Renderer::Init`'s multi-GPU probe (004da670) calls five of them.
//! The platform can't run driver code, so every answer comes from the machine table:
//! `scripts/nvapi-capture` loads the player's `nvapi.dll` in a 32-bit process, makes the same
//! calls in the same order and records each status and the bytes it wrote (lines `NvApi*` of
//! `d3d9_adapter.tsv`, git-ignored, never committed).
//!
//! - `LoadLibraryA("nvapi.dll")`: [`NVAPI_MODULE`] when the capture loaded it; 0 with the
//!   captured `GetLastError` when it did not (a machine without NVIDIA's driver).
//! - `GetProcAddress(module, "nvapi_QueryInterface")`: [`CODE_BASE`], or 0 (error 127,
//!   `ERROR_PROC_NOT_FOUND`) when the capture did not find it.
//! - `nvapi_QueryInterface(id)` (cdecl): for an id the capture asked, a code address of the
//!   platform's when the DLL gave a function, 0 when it gave none. An id the capture did not ask
//!   is not provided.
//! - The functions (cdecl): the captured status of the call with the same words; the out bytes
//!   the call wrote (those that read the same after the capture's zero-filled and ff-filled
//!   calls) are written to the game's buffers, the others are left as they are. A call the
//!   capture does not have stops the run ("not provided").
//!
//! The machine table must have the `NvApi*` lines; without them `LoadLibraryA("nvapi.dll")`
//! stops the run and says how to capture them.

use crate::{Engine, Ret};
use std::collections::HashMap;

/// `nvapi_QueryInterface` and, from [`FUNCTION_CODE`], the functions it hands out (code
/// addresses no exe code uses).
pub const CODE_BASE: u32 = 0x7e04_0000;
const FUNCTION_CODE: u32 = CODE_BASE + 0x100;
/// The module handle `LoadLibraryA("nvapi.dll")` answers (a value no image uses).
pub const NVAPI_MODULE: u32 = 0x7e13_0000;

/// The ids the multi-GPU probe calls, by the words they take (from the game's calls,
/// `renderer.rs` `probe_multi_gpu_first_vendor`). Their NvAPI names are NVIDIA's published ones
/// (the exe holds only the ids).
const INITIALIZE: u32 = 0x0150_e828;
const DRIVER_VERSION: u32 = 0xf951_a4d1;
const DISPLAY_HANDLE: u32 = 0x9abd_d40d;
const LOGICAL_GPUS: u32 = 0x48b3_ea59;
const PHYSICAL_GPUS: u32 = 0xe5ac_921f;
const SLI_STATE: u32 = 0x4b70_8b54;

const ERROR_PROC_NOT_FOUND: u32 = 127;

/// How to capture the answers, for the stop when the table has none.
pub const CAPTURE_HINT: &str = "run `cargo run --release --target i686-pc-windows-msvc \
     --manifest-path scripts/nvapi-capture/Cargo.toml -- <FalloutNV.exe> >> <table>` on this \
     machine (the table NV_D3D9_ADAPTER names, default %LOCALAPPDATA%/nv-rs/d3d9_adapter.tsv)";

/// One captured call: its status and the bytes it wrote (`None` for a byte it left alone).
#[derive(Debug, Clone)]
struct Call {
    status: u32,
    bytes: Vec<Option<u8>>,
}

/// The `NvApi*` lines of the machine table.
#[derive(Debug, Default, Clone)]
pub struct Table {
    /// `LoadLibraryA`: found, and `GetLastError` when not.
    load: Option<(bool, u32)>,
    procedure: Option<bool>,
    queries: HashMap<u32, bool>,
    calls: HashMap<String, Call>,
}

fn bytes(text: &str) -> Vec<u8> {
    (0..text.len() / 2)
        .map(|i| u8::from_str_radix(&text[2 * i..2 * i + 2], 16).expect("hex bytes"))
        .collect()
}

impl Table {
    /// Takes one table line; false when it is not an `NvApi*` line.
    pub fn take_line(&mut self, f: &[&str]) -> bool {
        let hex = |i: usize| u32::from_str_radix(f[i], 16).expect("a hex word in an NvApi line");
        match f[0] {
            "NvApiLoad" => self.load = Some((hex(2) != 0, hex(3))),
            "NvApiProc" => self.procedure = Some(hex(2) != 0),
            "NvApiQuery" => {
                self.queries.insert(hex(1), hex(2) != 0);
            }
            "NvApiCall" => {
                let zero = bytes(f.get(3).copied().unwrap_or(""));
                let ff = bytes(f.get(4).copied().unwrap_or(""));
                let written = zero
                    .iter()
                    .enumerate()
                    .map(|(i, a)| (ff.get(i) == Some(a)).then_some(*a))
                    .collect();
                self.calls.insert(
                    f[1].to_string(),
                    Call {
                        status: hex(2),
                        bytes: written,
                    },
                );
            }
            "NvApiEnd" | "NvApiDevice" => {}
            _ => return false,
        }
        true
    }
}

/// The run's NvAPI state (`Graphics::nvapi`): the table, the code addresses handed out by id.
#[derive(Debug, Default)]
pub struct State {
    pub(crate) table: Table,
    functions: Vec<u32>,
    registered: bool,
}

fn table(e: &Engine) -> &Table {
    &e.kernel.graphics.nvapi.table
}

/// `LoadLibraryA("nvapi.dll")`'s answer.
pub fn load_library(e: &mut Engine) -> u32 {
    match table(e).load {
        Some((true, _)) => NVAPI_MODULE,
        Some((false, error)) => {
            e.kernel.last_error = error;
            0
        }
        None => panic!("LoadLibraryA(\"nvapi.dll\") is not provided: the machine table has no NvApi lines; {CAPTURE_HINT}"),
    }
}

/// `GetProcAddress(NVAPI_MODULE, name)`.
pub fn proc_address(e: &mut Engine, name: &[u8]) -> u32 {
    if name != b"nvapi_QueryInterface" {
        panic!(
            "GetProcAddress(nvapi.dll, {:?}) is not provided",
            String::from_utf8_lossy(name)
        );
    }
    match table(e).procedure {
        Some(true) => {
            if !e.kernel.graphics.nvapi.registered {
                e.register(CODE_BASE, query_interface);
                e.kernel.graphics.nvapi.registered = true;
            }
            CODE_BASE
        }
        Some(false) => {
            e.kernel.last_error = ERROR_PROC_NOT_FOUND;
            0
        }
        None => panic!(
            "GetProcAddress(nvapi.dll, nvapi_QueryInterface) was not captured: {CAPTURE_HINT}"
        ),
    }
}

// Platform: replaces nvapi.dll nvapi_QueryInterface (found by name)
/// `nvapi_QueryInterface(id)` (cdecl): the function for `id` as the capture found it.
fn query_interface(e: &mut Engine, args: &[u32]) -> Ret {
    let id = args[0];
    let found = match table(e).queries.get(&id) {
        Some(found) => *found,
        None => panic!("nvapi_QueryInterface({id:08x}) was not captured: not provided"),
    };
    if !found {
        return Ret::default();
    }
    let functions = &mut e.kernel.graphics.nvapi.functions;
    let index = match functions.iter().position(|f| *f == id) {
        Some(i) => i,
        None => {
            functions.push(id);
            functions.len() - 1
        }
    };
    let address = FUNCTION_CODE + 4 * index as u32;
    e.register(address, function);
    Ret {
        eax: address,
        ..Ret::default()
    }
}

/// A function `nvapi_QueryInterface` handed out (cdecl): the captured answer for its id and
/// words. The words the key holds and the out buffers, by id, are the ones the game's calls
/// pass; any other id is not provided.
fn function(e: &mut Engine, args: &[u32]) -> Ret {
    let address = *e.call_stack.last().expect("called through Engine::call");
    let id = e.kernel.graphics.nvapi.functions[((address - FUNCTION_CODE) / 4) as usize];
    let word = |i: usize| args.get(i).copied().unwrap_or(0);
    let (key, out) = match id {
        INITIALIZE => (format!("{id:x}"), 0),
        DRIVER_VERSION => (
            format!("{id:x} {:x} {:x}", word(0), e.mem.u32(word(1))),
            word(1),
        ),
        DISPLAY_HANDLE => (format!("{id:x} {:x}", word(0)), word(1)),
        LOGICAL_GPUS | PHYSICAL_GPUS => (format!("{id:x}"), 0),
        SLI_STATE => {
            assert!(
                super::d3d9::is_direct3d(e, word(0)),
                "NvAPI {id:08x} on {:#x} (not the IDirect3D9 object) is not provided",
                word(0)
            );
            (format!("{id:x} direct3d {:x}", e.mem.u32(word(1))), word(1))
        }
        _ => panic!("the NvAPI function {id:08x} is not provided"),
    };
    let Some(call) = table(e).calls.get(&key).cloned() else {
        panic!("the NvAPI call {key} was not captured: not provided");
    };
    // The handle array (0x100 bytes) and the count go to two buffers; the others to one.
    let regions: Vec<(u32, std::ops::Range<usize>)> = match id {
        LOGICAL_GPUS | PHYSICAL_GPUS => vec![(word(0), 0..0x100), (word(1), 0x100..0x104)],
        INITIALIZE => vec![],
        _ => vec![(out, 0..call.bytes.len())],
    };
    for (at, range) in regions {
        for (i, b) in call.bytes[range].iter().enumerate() {
            if let Some(b) = b {
                e.mem.set_u8(at + i as u32, *b);
            }
        }
    }
    Ret {
        eax: call.status,
        ..Ret::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::platform::d3d9;

    /// A made-up capture (not a real machine's): two display handles, one logical and one
    /// physical GPU, the SLI query refused.
    fn table_text() -> String {
        let mut t = String::new();
        t += "NvApiDevice\t\t0\n";
        t += "NvApiLoad\t\t1\t0\nNvApiProc\t\t1\n";
        t += "NvApiQuery\t150e828\t1\nNvApiCall\t150e828\t0\t\t\t0\n";
        t += "NvApiQuery\t6c2d048c\t1\nNvApiQuery\tf951a4d1\t1\nNvApiQuery\t12345678\t0\n";
        let version_zero = format!("8c000100{}{}", "11223344", "00".repeat(0x84));
        let version_ff = format!("8c000100{}{}", "11223344", "ff".repeat(0x84));
        t += &format!("NvApiCall\tf951a4d1 0 1008c\t0\t{version_zero}\t{version_ff}\t0\n");
        t += "NvApiEnd\n";
        t
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        let mut table = Table::default();
        for line in table_text().lines() {
            let f: Vec<&str> = line.split('\t').collect();
            assert!(table.take_line(&f));
        }
        e.kernel.graphics.nvapi.table = table;
        e
    }

    #[test]
    fn the_start_up_finds_the_captured_functions() {
        let mut e = engine();
        assert_eq!(load_library(&mut e), NVAPI_MODULE);
        let query = proc_address(&mut e, b"nvapi_QueryInterface");
        let init = e.call(query, &[INITIALIZE]).u32();
        assert_ne!(init, 0);
        assert_eq!(e.call(init, &[]).u32(), 0);
        assert_eq!(e.call(query, &[0x1234_5678]).u32(), 0);
        let version = e.call(query, &[DRIVER_VERSION]).u32();
        assert_ne!(version, 0);
        assert_ne!(version, init);
        // The same id answers the same address.
        assert_eq!(e.call(query, &[INITIALIZE]).u32(), init);
    }

    #[test]
    fn a_call_writes_only_the_bytes_the_capture_saw_written() {
        let mut e = engine();
        load_library(&mut e);
        let query = proc_address(&mut e, b"nvapi_QueryInterface");
        let version = e.call(query, &[DRIVER_VERSION]).u32();
        let buffer = e.mem.alloc(0x8c);
        e.mem.write(buffer, &[0x5a; 0x8c]);
        e.mem.set_u32(buffer, 0x1008c);
        assert_eq!(e.call(version, &[0, buffer]).u32(), 0);
        assert_eq!(e.mem.u32(buffer + 4), 0x4433_2211);
        assert_eq!(e.mem.u8(buffer + 8), 0x5a);
    }

    #[test]
    #[should_panic(expected = "not captured")]
    fn an_id_the_capture_did_not_ask_stops() {
        let mut e = engine();
        load_library(&mut e);
        let query = proc_address(&mut e, b"nvapi_QueryInterface");
        e.call(query, &[0x0bad_0bad]);
    }

    #[test]
    #[should_panic(expected = "not captured")]
    fn a_call_the_capture_does_not_have_stops() {
        let mut e = engine();
        load_library(&mut e);
        let query = proc_address(&mut e, b"nvapi_QueryInterface");
        let version = e.call(query, &[DRIVER_VERSION]).u32();
        let buffer = e.mem.alloc(0x8c);
        e.mem.set_u32(buffer, 0x1008c);
        e.call(version, &[1, buffer]);
    }

    #[test]
    fn a_machine_without_the_dll_answers_not_found() {
        let mut e = Engine::new();
        let mut table = Table::default();
        assert!(table.take_line(&["NvApiLoad", "", "0", "7e"]));
        e.kernel.graphics.nvapi.table = table;
        assert_eq!(load_library(&mut e), 0);
        assert_eq!(e.kernel.last_error, 0x7e);
        let _ = d3d9::D3D_OK;
    }

    #[test]
    #[should_panic(expected = "no NvApi lines")]
    fn without_the_lines_the_load_stops() {
        let mut e = Engine::new();
        load_library(&mut e);
    }
}
