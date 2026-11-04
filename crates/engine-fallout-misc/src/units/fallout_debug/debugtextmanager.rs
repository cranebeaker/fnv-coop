//! `fallout/debug/debugtextmanager.cpp` (Xbox PDB source unit), subsystem `fallout/debug`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    funcs_n086()
}

/// The vtables (RTTI names): `NiTMap<unsigned int, IDebugTextPage*>`
/// (`01079170`) and its base `NiTMapBase<DFALL<..>, unsigned int,
/// IDebugTextPage*>` (`01079190`).
const NI_T_MAP_VTABLE: u32 = 0x0107_9170;
const NI_T_MAP_BASE_VTABLE: u32 = 0x0107_9190;
/// `00401030` (`cdecl`, one word): frees a block.
const FREE: u32 = 0x0040_1030;
/// `00438af0` (`thiscall`, no words): empties the map.
const MAP_REMOVE_ALL_00438AF0: u32 = 0x0043_8af0;
/// `00aa1070` (`cdecl`, one word: bytes) allocates the bucket array;
/// `00aa10f0` (`cdecl`, one word) frees it.
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
/// `00403d30` (`cdecl`: block, byte, size): fills a block.
const FILL_00403D30: u32 = 0x0040_3d30;

// Translated from 00802b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int, IDebugTextPage*>`'s scalar deleting destructor
/// (engine map; `thiscall`, `RET 4`): the destructor (`00802bf0`), then
/// `00401030` frees `this` when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_debug_text_page_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00802bf0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00802b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` constructor (unnamed in the engine map; `thiscall`,
/// `RET 4`: the bucket count): the vtable `01079190`, the count at `+4`, no
/// items (`+0xc`), and a bucket array of `count * 4` bytes (`00aa1070`) at
/// `+8`, zero-filled (`00403d30`). Returns `this`.
pub fn fn_00802b80(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_T_MAP_BASE_VTABLE);
    e.mem.set_u32(base + 4, buckets);
    e.mem.set_u32(base + 0xc, 0);
    let size = e.mem.u32(base + 4) << 2;
    let block = e.call(ALLOCATE_00AA1070, &args![size]).u32();
    e.mem.set_u32(base + 8, block);
    let size = e.mem.u32(base + 4) << 2;
    let block = e.mem.u32(base + 8);
    e.call(FILL_00403D30, &args![block, 0u32, size]);
    this
}

// Translated from 00802bf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00802bf0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The `NiTMap<unsigned int, IDebugTextPage*>` destructor (unnamed in the
/// engine map; `thiscall`, plain `RET`): the vtable `01079170`, the map
/// emptied (`00438af0`), then the base destructor (`00802c50`).
pub fn fn_00802bf0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_T_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL_00438AF0, &args![this]);
    fn_00802c50(e, this);
}

// Translated from 00802c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` destructor (unnamed in the engine map; `thiscall`,
/// plain `RET`): the vtable `01079190`, the map emptied (`00438af0`), then
/// the bucket array at `+8` freed (`00aa10f0`).
pub fn fn_00802c50(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_T_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL_00438AF0, &args![this]);
    let block = e.mem.u32(this.addr() + 8);
    e.call(FREE_00AA10F0, &args![block]);
}

// Translated from 00802c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` scalar deleting destructor (engine map; `thiscall`,
/// `RET 4`): the destructor (`00802c50`), then `00401030` frees `this` when
/// bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_base_debug_text_page_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00802c50(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's n086 translations, by exe address.
fn funcs_n086() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0080_2b50,
            ni_t_map_debug_text_page_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0080_2b80, fn_00802b80(Ptr, u32) -> Ptr),
        entry!(0x0080_2bf0, fn_00802bf0(Ptr)),
        entry!(0x0080_2c50, fn_00802c50(Ptr)),
        entry!(
            0x0080_2c80,
            ni_t_map_base_debug_text_page_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod n086_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine) {
        for a in [MAP_REMOVE_ALL_00438AF0, FREE_00AA10F0, FREE, FILL_00403D30] {
            e.register(a, |_, _| Ret::default());
        }
    }

    #[test]
    fn constructor_makes_the_bucket_array() {
        let mut e = Engine::new();
        quiet(&mut e);
        e.register(ALLOCATE_00AA1070, |_, _| 0x7000u32.into_ret());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_2b80, &args![this, 0x25u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_T_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x25);
        assert_eq!(e.mem.u32(this + 8), 0x7000);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls(&e, ALLOCATE_00AA1070), vec![vec![0x94]]);
        assert_eq!(calls(&e, FILL_00403D30), vec![vec![0x7000, 0, 0x94]]);
    }

    #[test]
    fn destructors_empty_free_and_delete() {
        let mut e = Engine::new();
        quiet(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x0080_2bf0, &args![this]);
        assert_eq!(e.mem.u32(this), NI_T_MAP_BASE_VTABLE);
        assert_eq!(
            calls(&e, MAP_REMOVE_ALL_00438AF0),
            vec![vec![this], vec![this]]
        );
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x7000]]);
        for addr in [0x0080_2b50, 0x0080_2c80] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![this, 0u32]).u32(), this);
            assert!(calls(&e, FREE).is_empty());
            assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x7000]]);
            e.call(addr, &args![this, 1u32]);
            assert_eq!(calls(&e, FREE), vec![vec![this]]);
        }
        // Only the NiTMap destructor empties the map twice.
        e.call_log = Some(vec![]);
        e.call(0x0080_2c80, &args![this, 0u32]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL_00438AF0).len(), 1);
    }
}
