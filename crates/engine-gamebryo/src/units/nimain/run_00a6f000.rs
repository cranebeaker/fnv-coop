//! `NiMain/run_00a6f000` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The game's `operator new` (`cdecl`, the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The constructor `00a5d3a0` (`thiscall`, unit `NiMain/niobject.cpp` in the engine map).
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// The allocator of the map's bucket table (`cdecl`, the size).
const TABLE_ALLOCATE: u32 = 0x00aa_1070;
/// `_memset` (`cdecl`, (destination, value, size)).
const MEMSET: u32 = 0x00ec_61c0;
/// The vtables stored: `NiDefaultAVObjectPalette` (RTTI), then the map at +8 first as
/// `NiTFixedStringMapBase<...>` and then as `NiTFixedStringMap<NiAVObject *>` (RTTI).
const PALETTE_VTABLE: u32 = 0x0109_6284;
const FIXED_STRING_MAP_BASE_VTABLE: u32 = 0x0109_6114;
const FIXED_STRING_MAP_VTABLE: u32 = 0x0109_618c;

// Translated from 00a6f000 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function, no engine-map name; by its body the creation function of
/// `NiDefaultAVObjectPalette`, the RTTI of the vtable it stores; no arguments): a new
/// 0x1c-byte block from `operator new`; null when that fails. Otherwise the constructor
/// `00a5d3a0`, the palette vtable `01096284`, at +8 a map (base vtable `01096114`, 0x25
/// buckets at +0xC, count +0x14 zero, a bucket table of 0x94 bytes from `00aa1070` at +0x10
/// cleared with `_memset` to bucket count * 4 bytes, then the map vtable `0109618c`), and
/// +0x18 zero. Answers the block.
pub fn fn_00a6f000(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(OBJECT_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, PALETTE_VTABLE);
    e.mem.set_u32(block + 8, FIXED_STRING_MAP_BASE_VTABLE);
    e.mem.set_u32(block + 0xc, 0x25);
    e.mem.set_u32(block + 0x14, 0);
    let table = e.call(TABLE_ALLOCATE, &args![0x94u32]).u32();
    let buckets = e.mem.u32(block + 0xc);
    e.mem.set_u32(block + 0x10, table);
    e.call(MEMSET, &args![table, 0u32, buckets.wrapping_mul(4)]);
    e.mem.set_u32(block + 8, FIXED_STRING_MAP_VTABLE);
    e.mem.set_u32(block + 0x18, 0);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a6_f000, fn_00a6f000() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a6f000_builds_an_empty_palette() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(TABLE_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let block = e.call(0x00a6_f000, &[]).u32();
        assert_ne!(block, 0);
        assert_eq!(e.mem.u32(block), PALETTE_VTABLE);
        assert_eq!(e.mem.u32(block + 8), FIXED_STRING_MAP_VTABLE);
        assert_eq!(e.mem.u32(block + 0xc), 0x25);
        assert_eq!(e.mem.u32(block + 0x18), 0);
        let table = e.mem.u32(block + 0x10);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(OBJECT_CONSTRUCT, vec![block])));
        assert!(log.contains(&(TABLE_ALLOCATE, vec![0x94])));
        assert!(log.contains(&(MEMSET, vec![table, 0, 0x94])));

        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_f000, &[]).u32(), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }
}
