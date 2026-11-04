//! `fallout shared/region system/tesregiondata.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f1430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionData` scalar deleting destructor (engine map name): runs the destructor body `004f14a0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn tes_region_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_14a0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESRegionData`'s vtable (RTTI `.?AVTESRegionData@@`, read at the word before it).
pub(crate) const TES_REGION_DATA_VTABLE: u32 = 0x0102_3ac4;
/// `004f15a0` (run_004f15a0 unit): the setter `LoadRegionData` passes the source's `bOverride` to.
const SET_OVERRIDE_004F15A0: u32 = 0x004f_15a0;
/// `004f15c0` (run_004f15a0 unit): the priority setter (stores a byte up to 100).
const SET_PRIORITY_004F15C0: u32 = 0x004f_15c0;

layout! {
    /// `TESRegionData` (Xbox PDB), 8 bytes; the PC code uses the same offsets.
    pub struct TESRegionData: 0x8 {
        /// The vtable.
        0x00 vtable: u32,
        /// `bOverride` (Xbox PDB).
        0x04 bOverride: u8,
        /// `bIgnore` (Xbox PDB).
        0x05 bIgnore: u8,
        /// `cPriority` (Xbox PDB).
        0x06 cPriority: u8,
    }
}

// Translated from 004f1400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionData::TESRegionData` (Xbox PDB): stores the vtable, clears
/// `bOverride`, sets `cPriority` to 50 (0x32), clears `bIgnore` (in that
/// order). Returns `this`.
pub fn tes_region_data_tes_region_data(
    e: &mut Engine,
    this: Ptr<TESRegionData>,
) -> Ptr<TESRegionData> {
    e.set(this, TESRegionData::vtable, TES_REGION_DATA_VTABLE);
    e.set(this, TESRegionData::bOverride, 0);
    e.set(this, TESRegionData::cPriority, 0x32);
    e.set(this, TESRegionData::bIgnore, 0);
    this
}

// Translated from 004f1460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionData` copy constructor (unnamed in the engine map; `thiscall`,
/// `RET 4`): stores the vtable, then copies `bOverride`, `cPriority` and
/// `bIgnore` from `source` in that order. Returns `this`.
pub fn fn_004f1460(
    e: &mut Engine,
    this: Ptr<TESRegionData>,
    source: Ptr<TESRegionData>,
) -> Ptr<TESRegionData> {
    e.set(this, TESRegionData::vtable, TES_REGION_DATA_VTABLE);
    let over = e.get(source, TESRegionData::bOverride);
    e.set(this, TESRegionData::bOverride, over);
    let priority = e.get(source, TESRegionData::cPriority);
    e.set(this, TESRegionData::cPriority, priority);
    let ignore = e.get(source, TESRegionData::bIgnore);
    e.set(this, TESRegionData::bIgnore, ignore);
    this
}

// Translated from 004f14a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionData` destructor body (unnamed in the engine map): stores the
/// class's vtable back at `this`, nothing else.
pub fn fn_004f14a0(e: &mut Engine, this: Ptr<TESRegionData>) {
    e.set(this, TESRegionData::vtable, TES_REGION_DATA_VTABLE);
}

// Translated from 004f1560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionData::LoadRegionData` (Xbox PDB; `thiscall`, `RET 4`): with a
/// null `source` returns false; otherwise passes the source's byte at +4
/// (`bOverride`) to `004f15a0` and its byte at +5 (`bIgnore`) to the
/// priority setter `004f15c0` (both on `this`, zero-extended words, as the
/// exe pushes them), then returns true.
pub fn tes_region_data_load_region_data(
    e: &mut Engine,
    this: Ptr<TESRegionData>,
    source: Ptr<TESRegionData>,
) -> bool {
    if source.is_null() {
        return false;
    }
    let over = u32::from(e.get(source, TESRegionData::bOverride));
    e.call(SET_OVERRIDE_004F15A0, &args![this, over]);
    let ignore = u32::from(e.get(source, TESRegionData::bIgnore));
    e.call(SET_PRIORITY_004F15C0, &args![this, ignore]);
    true
}

/// `memset` (`00403d30`, cdecl).
const MEMSET: u32 = 0x0040_3d30;
/// `004f1540` (`thiscall`, this unit, untranslated): the byte stored at
/// +4 of the saved header.
const HEADER_BYTE_004F1540: u32 = 0x004f_1540;
/// `005bb4d0` (`thiscall`): the byte stored at +5 of the saved header.
const HEADER_BYTE_005BB4D0: u32 = 0x005b_b4d0;
/// `00401500` (no argument): whether saved data is byte-swapped.
const SAVE_SWAPS: u32 = 0x0040_1500;
/// `00503210` (`thiscall` on the word): swaps the word's bytes.
const SWAP_WORD: u32 = 0x0050_3210;
/// `TESForm::__AddChunkData` (`00485990`, cdecl: chunk, data, size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `RDAT`.
const CHUNK_RDAT: u32 = 0x5441_4452;

// Translated from 004f14c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionData::Save` (Xbox PDB; `thiscall`, no stack word): an 8-byte
/// header on the stack, zeroed (`memset`): the data's type (vtable `+0x10`)
/// at +0, the bytes from `004f1540` and `005bb4d0` at +4 and +5. When
/// `00401500` answers true the first word is swapped (`00503210`) before
/// the `RDAT` chunk is added (`TESForm::__AddChunkData`, 8 bytes) and
/// swapped back after.
pub fn tes_region_data_save(e: &mut Engine, this: Ptr<TESRegionData>) {
    e.with_stack(8, |e, header| {
        e.call(MEMSET, &args![header, 0u32, 8u32]);
        let kind = e.vcall(this.addr(), 0x10, &args![]).u32();
        e.mem.set_u32(header.addr(), kind);
        let first = e.call(HEADER_BYTE_004F1540, &args![this]).u8();
        e.mem.set_u8(header.addr() + 4, first);
        let second = e.call(HEADER_BYTE_005BB4D0, &args![this]).u8();
        e.mem.set_u8(header.addr() + 5, second);
        if e.call(SAVE_SWAPS, &args![]).bool() {
            e.call(SWAP_WORD, &args![header]);
        }
        e.call(ADD_CHUNK_DATA, &args![CHUNK_RDAT, header, 8u32]);
        if e.call(SAVE_SWAPS, &args![]).bool() {
            e.call(SWAP_WORD, &args![header]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_14c0, tes_region_data_save(Ptr<TESRegionData>)),
        entry!(0x004f_1430, tes_region_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x004f_1400,
            tes_region_data_tes_region_data(Ptr<TESRegionData>) -> Ptr<TESRegionData>
        ),
        entry!(
            0x004f_1460,
            fn_004f1460(Ptr<TESRegionData>, Ptr<TESRegionData>) -> Ptr<TESRegionData>
        ),
        entry!(0x004f_14a0, fn_004f14a0(Ptr<TESRegionData>)),
        entry!(
            0x004f_1560,
            tes_region_data_load_region_data(Ptr<TESRegionData>, Ptr<TESRegionData>) -> bool
        ),
    ]
}

#[cfg(test)]
mod region_data_tests {
    use super::*;

    #[test]
    fn at_004f1400_sets_vtable_and_defaults() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0xffff_ffff);
        let r = e.call(0x004f_1400, &args![this]).u32();
        assert_eq!(r, this);
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_VTABLE);
        assert_eq!(e.mem.u8(this + 4), 0);
        assert_eq!(e.mem.u8(this + 5), 0);
        assert_eq!(e.mem.u8(this + 6), 0x32);
        assert_eq!(e.mem.u8(this + 7), 0xff);
    }

    #[test]
    fn at_004f1460_copies_the_three_bytes() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 0x1234);
        e.mem.set_u32(source + 4, 0x7708_0501);
        let r = e.call(0x004f_1460, &args![this, source]).u32();
        assert_eq!(r, this);
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x0008_0501);
    }

    #[test]
    fn at_004f14a0_restores_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x55);
        e.call(0x004f_14a0, &args![this]);
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x55);
    }

    #[test]
    fn at_004f1560_passes_the_source_bytes_to_the_setters() {
        let mut e = Engine::new();
        e.register(SET_OVERRIDE_004F15A0, |_, _| Ret::default());
        e.register(SET_PRIORITY_004F15C0, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_1560, &args![this, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 == 0x004f_1560));

        let source = e.mem.alloc(8);
        e.mem.set_u32(source + 4, 0x0033_80ff);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_1560, &args![this, source]).bool());
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.into_iter().filter(|c| c.0 != 0x004f_1560).collect();
        assert_eq!(
            calls,
            vec![
                (SET_OVERRIDE_004F15A0, vec![this, 0xff]),
                (SET_PRIORITY_004F15C0, vec![this, 0x80]),
            ]
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004f1430_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_14a0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_1430, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_1430)
                .collect();
            let mut expected = vec![(0x004f_14a0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn save(swaps: bool) -> (Vec<u8>, Vec<u32>) {
        let mut e = Engine::new();
        e.put_vtable(0x0150_0000, &[0, 0, 0, 0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| 0x0000_0102u32.into_ret());
        let this: Ptr<TESRegionData> = e.new_object();
        e.mem.set_u32(this.addr(), 0x0150_0000);
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(HEADER_BYTE_004F1540, |_, _| 0x1107u32.into_ret());
        e.register(HEADER_BYTE_005BB4D0, |_, _| 0x2208u32.into_ret());
        if swaps {
            e.register(SAVE_SWAPS, |_, _| 1u32.into_ret());
        } else {
            e.register(SAVE_SWAPS, |_, _| 0u32.into_ret());
        }
        e.register(SWAP_WORD, |e, a| {
            let v = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], v.swap_bytes());
            Ret::default()
        });
        let added: Rc<RefCell<(Vec<u8>, Vec<u32>)>> = Rc::default();
        let log = added.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            *log.borrow_mut() = (e.mem.bytes(a[1], a[2]), vec![a[0], a[2]]);
            Ret::default()
        });
        e.call(0x004f_14c0, &args![this]);
        added.take()
    }

    #[test]
    fn save_adds_the_rdat_header() {
        let (bytes, words) = save(false);
        assert_eq!(bytes, vec![2, 1, 0, 0, 7, 8, 0, 0]);
        assert_eq!(words, vec![CHUNK_RDAT, 8]);
    }

    #[test]
    fn save_swaps_the_type_word_when_the_save_swaps() {
        let (bytes, _) = save(true);
        assert_eq!(bytes, vec![0, 0, 1, 2, 7, 8, 0, 0]);
    }
}
