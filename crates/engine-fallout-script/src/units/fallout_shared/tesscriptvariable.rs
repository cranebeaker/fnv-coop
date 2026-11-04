//! `fallout shared/tesscriptvariable.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The file reader: chunk type (`004726b0`), chunk data (`00472890`:
/// buffer, size) and its byte-swap flag (`00401680`).
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_GET_CHUNK_DATA: u32 = 0x0047_2890;
const FILE_SWAP_FLAG: u32 = 0x0040_1680;
/// In-place byte swaps, cdecl with an address and a flag byte: four bytes
/// (`00401080`) and eight bytes (`004717f0`).
const SWAP_4_BYTES: u32 = 0x0040_1080;
const SWAP_8_BYTES: u32 = 0x0047_17f0;
/// The `SLSD` chunk tag, as the little-endian word the reader returns.
const CHUNK_SLSD: u32 = 0x4453_4c53;
/// `sizeof(SCRIPT_LOCAL)` (Xbox PDB), the bytes `Load` reads.
const SCRIPT_LOCAL_SIZE: u32 = 0x18;

// Translated from 005e2870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps a script variable's data in place: `uiID` (`+0`, four bytes)
/// and `fValue` (`+8`, eight bytes) of `SCRIPT_LOCAL` (Xbox PDB), each with
/// a zero flag byte.
pub fn fn_005e2870(e: &mut Engine, this: Ptr) {
    e.call(SWAP_4_BYTES, &args![this, 0u32]);
    e.call(SWAP_8_BYTES, &args![this.addr() + 8, 0u32]);
}

// Translated from 005e2a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptVariable::Load` (Xbox PDB): with a file whose current chunk is
/// `SLSD`, reads the chunk's 0x18 bytes into the variable's data (`this`)
/// and byte-swaps them ([`fn_005e2870`]) when the reader swaps. Nothing
/// without a file or for another chunk.
pub fn script_variable_load(e: &mut Engine, this: Ptr, file: Ptr) {
    if file == Ptr::NULL {
        return;
    }
    if e.call(FILE_CHUNK_TYPE, &args![file]).u32() != CHUNK_SLSD {
        return;
    }
    e.call(FILE_GET_CHUNK_DATA, &args![file, this, SCRIPT_LOCAL_SIZE]);
    if e.call(FILE_SWAP_FLAG, &args![file]).bool() {
        fn_005e2870(e, this);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e2870, fn_005e2870(Ptr)),
        entry!(0x005e2a20, script_variable_load(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The file object holds the chunk type at `+0` and the swap flag at
    /// `+4` for the doubles.
    fn file_doubles(e: &mut Engine) -> u32 {
        e.register(FILE_CHUNK_TYPE, |e, a| eax(e.mem.u32(a[0])));
        e.register(
            FILE_SWAP_FLAG,
            |e, a| eax(0xffff_ff00 | e.mem.u32(a[0] + 4)),
        );
        e.register(FILE_GET_CHUNK_DATA, |_, _| Ret::default());
        e.register(SWAP_4_BYTES, |_, _| Ret::default());
        e.register(SWAP_8_BYTES, |_, _| Ret::default());
        e.mem.alloc(8)
    }

    #[test]
    fn swap_covers_the_id_and_the_value() {
        let mut e = Engine::new();
        file_doubles(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x005e_2870, &[0x4000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (SWAP_4_BYTES, vec![0x4000, 0]),
                (SWAP_8_BYTES, vec![0x4008, 0])
            ]
        );
    }

    #[test]
    fn load_reads_slsd_and_swaps_when_the_reader_swaps() {
        let mut e = Engine::new();
        let file = file_doubles(&mut e);
        e.mem.set_u32(file, CHUNK_SLSD);
        e.mem.set_u32(file + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x005e_2a20, &[0x4000, file]);
        assert_eq!(calls_to(&e, FILE_GET_CHUNK_DATA), [[file, 0x4000, 0x18]]);
        assert_eq!(calls_to(&e, SWAP_4_BYTES), [[0x4000, 0]]);
        // Not swapping: the data is read, nothing swapped.
        e.mem.set_u32(file + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_2a20, &[0x4000, file]);
        assert_eq!(calls_to(&e, FILE_GET_CHUNK_DATA).len(), 1);
        assert!(calls_to(&e, SWAP_4_BYTES).is_empty());
    }

    #[test]
    fn load_ignores_other_chunks_and_a_null_file() {
        let mut e = Engine::new();
        let file = file_doubles(&mut e);
        e.mem.set_u32(file, 0x4d41_4e43);
        e.call_log = Some(vec![]);
        e.call(0x005e_2a20, &[0x4000, file]);
        e.call(0x005e_2a20, &[0x4000, 0]);
        assert!(calls_to(&e, FILE_GET_CHUNK_DATA).is_empty());
        assert_eq!(calls_to(&e, FILE_CHUNK_TYPE), [[file]]);
    }
}
