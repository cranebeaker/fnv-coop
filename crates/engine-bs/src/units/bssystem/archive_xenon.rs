//! `BSSystem/archive_xenon.cpp` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// CRT `strncmp` (`00ec8a19`, `cdecl`: two texts and a count).
const STRNCMP: u32 = 0x00ec_8a19;
/// `"Data\\"` in `.rdata` (`010a4820`).
const DATA_PREFIX_UPPER: u32 = 0x010a_4820;
/// `"data\\"` in `.rdata` (`010a4828`).
const DATA_PREFIX_LOWER: u32 = 0x010a_4828;

// Translated from 00b014b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ArchiveManager::TrimFileName` (Xbox PDB), `cdecl`, one word (the name):
/// steps over one leading `\`, then over a leading `Data\` or `data\` (each
/// compared with `strncmp` over 5 characters, the second only when the first
/// differs) and answers the rest.
pub fn archive_manager_trim_file_name(e: &mut Engine, name: Ptr) -> Ptr {
    let mut name = name.addr();
    if e.mem.u8(name) as i8 as i32 == 0x5c {
        name = name.wrapping_add(1);
    }
    let upper = e.call(STRNCMP, &args![name, DATA_PREFIX_UPPER, 5u32]).u32();
    if upper == 0 || e.call(STRNCMP, &args![name, DATA_PREFIX_LOWER, 5u32]).u32() == 0 {
        name = name.wrapping_add(5);
    }
    Ptr::new(name)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b0_14b0, archive_manager_trim_file_name(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A `strncmp` double over the engine's memory.
    fn strncmp(e: &mut Engine, a: &[u32]) -> Ret {
        for i in 0..a[2] {
            let x = e.mem.u8(a[0] + i);
            let y = e.mem.u8(a[1] + i);
            if x != y {
                return (i32::from(x) - i32::from(y)).into_ret();
            }
            if x == 0 {
                break;
            }
        }
        0u32.into_ret()
    }

    fn trim(text: &[u8]) -> (u32, u32) {
        let mut e = Engine::new();
        e.map(DATA_PREFIX_UPPER, 0x10);
        e.mem.set_cstr(DATA_PREFIX_UPPER, b"Data\\");
        e.mem.set_cstr(DATA_PREFIX_LOWER, b"data\\");
        e.register(STRNCMP, strncmp);
        let name = e.mem.alloc(0x20);
        e.mem.set_cstr(name, text);
        (e.call(0x00b0_14b0, &args![name]).u32(), name)
    }

    #[test]
    fn archive_manager_trim_file_name_drops_the_slash_and_the_data_folder() {
        let (r, n) = trim(b"\\Data\\a.nif");
        assert_eq!(r, n + 6);
        let (r, n) = trim(b"data\\a.nif");
        assert_eq!(r, n + 5);
        let (r, n) = trim(b"\\meshes\\a.nif");
        assert_eq!(r, n + 1);
        let (r, n) = trim(b"DATA\\a.nif");
        assert_eq!(r, n);
    }
}
