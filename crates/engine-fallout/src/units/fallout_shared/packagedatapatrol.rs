//! `fallout shared/packagedatapatrol.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The patrol package data: a float at +0 and, at +4, the 0x10-byte
//! `PackageEventAction` block (`run_0067da20`, `packageeventaction`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0067da40` (`ECX` = the block at +4): zeroes it.
const ACTION_CLEAR: u32 = 0x0067_da40;
/// `0067da80` (`ECX` = the block at +4): releases it.
const ACTION_RELEASE: u32 = 0x0067_da80;
/// `0041fd00` (`ECX` = the block at +4, one word; `RET 4`).
const ACTION_SET_0041FD00: u32 = 0x0041_fd00;
/// `PackageEventAction::InitItem` (Xbox PDB name of `0067e190`; `ECX` = the
/// block at +4, the owner; `RET 4`).
const ACTION_INIT_ITEM: u32 = 0x0067_e190;

// Translated from 0067c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the action block at +4 (`0067da80`). The engine map gives it no
/// name.
pub fn fn_0067c670(e: &mut Engine, this: Ptr) {
    e.call(ACTION_RELEASE, &args![this.addr() + 4]);
}

// Translated from 0067c690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the float at `source` into `this` (`FLD`/`FSTP`, a signalling NaN
/// quieted) and zeroes the action block at +4 (`0067da40`); `RET 4`.
/// Returns `this`. The engine map gives it no name.
pub fn fn_0067c690(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let value = x87_quiet_bits(e.mem.u32(source.addr()));
    e.mem.set_u32(this.addr(), value);
    e.call(ACTION_CLEAR, &args![this.addr() + 4]);
    this
}

// Translated from 0067c6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0041fd00` with 3 on the action block at +4, then
/// `PackageEventAction::InitItem` on it with the owner (`RET 4`). The engine
/// map gives it no name.
pub fn fn_0067c6c0(e: &mut Engine, this: Ptr, owner: Ptr) {
    e.call(ACTION_SET_0041FD00, &args![this.addr() + 4, 3u32]);
    e.call(ACTION_INIT_ITEM, &args![this.addr() + 4, owner]);
}

/// `__RTDynamicCast` (`cdecl`, five words) and the RTTI type descriptors of
/// `TESPackageData` and `TESPatrolPackageData` (read from the exe).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TES_PACKAGE_DATA_TYPE: u32 = 0x0119_bdec;
const TES_PATROL_PACKAGE_DATA_TYPE: u32 = 0x0119_beb0;
/// `00403d30` (`cdecl`: destination, value, count): fills bytes.
const FILL_BYTES: u32 = 0x0040_3d30;
/// `00401500` (no parameters): whether saved data is byte-swapped (`AL`).
const NEEDS_SWAP: u32 = 0x0040_1500;
/// `00483710` (`thiscall`, no stack word): byte-swaps the 16-bit value at
/// `this`.
const SWAP_16: u32 = 0x0048_3710;
/// `TESForm::__AddChunkData` (engine map, `00485990`, `cdecl`: type, data,
/// size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// The save buffer's write (`00865e50`, `thiscall`: data, size, flag) and
/// the load buffer's read (`00864980`, `thiscall`: destination, size).
const SAVE_WRITE: u32 = 0x0086_5e50;
const LOAD_READ: u32 = 0x0086_4980;
/// `0067dad0` (`thiscall`, `RET 4`): assigns a `PackageEventAction` from
/// another.
const ACTION_ASSIGN: u32 = 0x0067_dad0;
/// `PackageEventAction::Compare` (engine map, `0067dbf0`, `thiscall`, `RET
/// 4`).
const ACTION_COMPARE: u32 = 0x0067_dbf0;

/// `dynamic_cast` of `other` to `TESPatrolPackageData`.
fn cast_patrol(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![
            other,
            0u32,
            TES_PACKAGE_DATA_TYPE,
            TES_PATROL_PACKAGE_DATA_TYPE,
            0u32
        ],
    )
    .u32()
}

// Translated from 0067c4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPatrolPackageData::Copy` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): when `other` is a `TESPatrolPackageData`, its
/// bytes at +4 and +5.
pub fn tes_patrol_package_data_copy(e: &mut Engine, this: Ptr, other: Ptr) {
    let cast = cast_patrol(e, other);
    if cast == 0 {
        return;
    }
    let first = e.mem.u8(cast + 4);
    e.mem.set_u8(this.addr() + 4, first);
    let second = e.mem.u8(cast + 5);
    e.mem.set_u8(this.addr() + 5, second);
}

// Translated from 0067c500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPatrolPackageData::Compare` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): true (they differ) when `other` is not a
/// `TESPatrolPackageData` or a byte at +4 or +5 differs.
pub fn tes_patrol_package_data_compare(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let cast = cast_patrol(e, other);
    if cast == 0 {
        return true;
    }
    e.mem.u8(this.addr() + 4) != e.mem.u8(cast + 4)
        || e.mem.u8(this.addr() + 5) != e.mem.u8(cast + 5)
}

// Translated from 0067c570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPatrolPackageData::Save` (Xbox PDB name in the engine map; no stack
/// word): a two-byte `PKPT` chunk (`00485990`) holding the byte at +4 and a
/// 0 (a stack word cleared by `00403d30`), byte-swapped before and after
/// (`00483710`) when `00401500` says so.
pub fn tes_patrol_package_data_save(e: &mut Engine, this: Ptr) {
    e.with_stack(4, |e, buffer| {
        e.call(FILL_BYTES, &args![buffer, 0u32, 2u32]);
        let value = e.mem.u8(this.addr() + 4);
        e.mem.set_u8(buffer.addr(), value);
        if e.call(NEEDS_SWAP, &args![]).bool() {
            e.call(SWAP_16, &args![buffer]);
        }
        e.call(ADD_CHUNK_DATA, &args![0x5450_4b50u32, buffer, 2u32]);
        if e.call(NEEDS_SWAP, &args![]).bool() {
            e.call(SWAP_16, &args![buffer]);
        }
    });
}

// Translated from 0067c5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPatrolPackageData::SaveGame` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`, the save buffer): the bytes at +4 and +5, one each
/// (`00865e50` with 1 and 0).
pub fn tes_patrol_package_data_save_game(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(SAVE_WRITE, &args![buffer, this.addr() + 4, 1u32, 0u32]);
    e.call(SAVE_WRITE, &args![buffer, this.addr() + 5, 1u32, 0u32]);
}

// Translated from 0067c620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPatrolPackageData::LoadGame` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`, the load buffer): the bytes at +4 and +5, one each
/// (`00864980` with 1).
pub fn tes_patrol_package_data_load_game(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(LOAD_READ, &args![buffer, this.addr() + 4, 1u32]);
    e.call(LOAD_READ, &args![buffer, this.addr() + 5, 1u32]);
}

// Translated from 0067c650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The patrol reference data's constructor (no stack word; the engine map
/// gives it no name): the `float` at +0 is 0.0 (`FLDZ`) and the event
/// action at +4 cleared (`0067da40`). Returns `this`.
pub fn fn_0067c650(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.call(ACTION_CLEAR, &args![this.addr() + 4]);
    this
}

// Translated from 0067c6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns patrol reference data from `other` (`thiscall`, `RET 4`; the
/// engine map gives it no name): the `float` at +0 (through the x87, a
/// signalling NaN quieted) and the event action at +4 (`0067dad0`).
pub fn fn_0067c6f0(e: &mut Engine, this: Ptr, other: u32) {
    let value = x87_quiet_bits(e.mem.u32(other));
    e.mem.set_u32(this.addr(), value);
    e.call(
        ACTION_ASSIGN,
        &args![this.addr() + 4, other.wrapping_add(4)],
    );
}

// Translated from 0067c720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PatrolRefData::Compare` (Xbox PDB name in the engine map; `thiscall`,
/// `RET 4`): true (they differ) when the event actions at +4 differ
/// (`0067dbf0`) or the `float`s at +0 are not equal (`FUCOMPP`: a NaN is
/// never equal).
pub fn patrol_ref_data_compare(e: &mut Engine, this: Ptr, other: u32) -> bool {
    if e.call(
        ACTION_COMPARE,
        &args![this.addr() + 4, other.wrapping_add(4)],
    )
    .bool()
    {
        return true;
    }
    let mine = x87_load32(e.mem.u32(this.addr()));
    let theirs = x87_load32(e.mem.u32(other));
    mine != theirs
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_c4b0, tes_patrol_package_data_copy(Ptr, Ptr)),
        entry!(0x0067_c500, tes_patrol_package_data_compare(Ptr, Ptr) -> bool),
        entry!(0x0067_c570, tes_patrol_package_data_save(Ptr)),
        entry!(0x0067_c5e0, tes_patrol_package_data_save_game(Ptr, u32)),
        entry!(0x0067_c620, tes_patrol_package_data_load_game(Ptr, u32)),
        entry!(0x0067_c650, fn_0067c650(Ptr) -> Ptr),
        entry!(0x0067_c6f0, fn_0067c6f0(Ptr, u32)),
        entry!(0x0067_c720, patrol_ref_data_compare(Ptr, u32) -> bool),
        entry!(0x0067_c670, fn_0067c670(Ptr)),
        entry!(0x0067_c690, fn_0067c690(Ptr, Ptr) -> Ptr),
        entry!(0x0067_c6c0, fn_0067c6c0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log_of(e: &mut Engine, addr: u32, words: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log = Some(vec![]);
        e.call(addr, words);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect()
    }

    #[test]
    fn patrol_data_forwards_to_its_action_block() {
        let mut e = Engine::new();
        for addr in [
            ACTION_CLEAR,
            ACTION_RELEASE,
            ACTION_SET_0041FD00,
            ACTION_INIT_ITEM,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        assert_eq!(
            log_of(&mut e, 0x0067_c670, &[0x4000]),
            vec![(ACTION_RELEASE, vec![0x4004])]
        );
        assert_eq!(
            log_of(&mut e, 0x0067_c6c0, &[0x4000, 0x5000]),
            vec![
                (ACTION_SET_0041FD00, vec![0x4004, 3]),
                (ACTION_INIT_ITEM, vec![0x4004, 0x5000]),
            ]
        );
        let this = e.mem.alloc(0x14);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 0x7f80_0003);
        assert_eq!(
            log_of(&mut e, 0x0067_c690, &[this, source]),
            vec![(ACTION_CLEAR, vec![this + 4])]
        );
        assert_eq!(e.mem.u32(this), 0x7fc0_0003);
        assert_eq!(e.call(0x0067_c690, &args![this, source]).u32(), this);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    const SCRATCH: u32 = 0x0f00_0000;

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.mem.map(SCRATCH, 0xc);
        e.register(DYNAMIC_CAST, |e, _| word(e.mem.u32(SCRATCH)));
        e.register(NEEDS_SWAP, |e, _| word(e.mem.u32(SCRATCH + 4)));
        e.register(ACTION_COMPARE, |e, _| word(e.mem.u32(SCRATCH + 8)));
        e.register(ADD_CHUNK_DATA, |e, a| {
            assert_eq!(e.mem.u16(a[1]), 0x0042);
            Ret::default()
        });
        for addr in [
            FILL_BYTES,
            SWAP_16,
            SAVE_WRITE,
            LOAD_READ,
            ACTION_ASSIGN,
            ACTION_CLEAR,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    #[test]
    fn copy_and_compare_the_two_bytes() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u16(other + 4, 0x0201);
        e.call(0x0067_c4b0, &args![this, other]);
        assert_eq!(e.mem.u16(this + 4), 0);
        assert!(e.call(0x0067_c500, &args![this, other]).bool());
        e.mem.set_u32(SCRATCH, other);
        assert!(e.call(0x0067_c500, &args![this, other]).bool());
        e.call(0x0067_c4b0, &args![this, other]);
        assert_eq!(e.mem.u16(this + 4), 0x0201);
        assert!(!e.call(0x0067_c500, &args![this, other]).bool());
        e.mem.set_u8(other + 4, 9);
        assert!(e.call(0x0067_c500, &args![this, other]).bool());
        e.mem.set_u8(other + 4, 1);
        e.mem.set_u8(other + 5, 9);
        assert!(e.call(0x0067_c500, &args![this, other]).bool());
    }

    #[test]
    fn save_writes_one_byte_in_a_two_byte_chunk() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        e.mem.set_u16(this + 4, 0x7742);
        let (_, log) = run(&mut e, 0x0067_c570, &[this]);
        let buffer = log[0].1[0];
        assert_eq!(log[0], (FILL_BYTES, vec![buffer, 0, 2]));
        assert!(log.contains(&(ADD_CHUNK_DATA, vec![0x5450_4b50, buffer, 2])));
        assert!(!log.iter().any(|c| c.0 == SWAP_16));
        e.mem.set_u32(SCRATCH + 4, 1);
        let (_, log) = run(&mut e, 0x0067_c570, &[this]);
        assert_eq!(log.iter().filter(|c| c.0 == SWAP_16).count(), 2);
    }

    #[test]
    fn save_game_and_load_game_move_two_bytes() {
        let mut e = engine();
        let (_, log) = run(&mut e, 0x0067_c5e0, &[0x4000, 0x5000]);
        assert_eq!(
            log,
            vec![
                (SAVE_WRITE, vec![0x5000, 0x4004, 1, 0]),
                (SAVE_WRITE, vec![0x5000, 0x4005, 1, 0]),
            ]
        );
        let (_, log) = run(&mut e, 0x0067_c620, &[0x4000, 0x5000]);
        assert_eq!(
            log,
            vec![
                (LOAD_READ, vec![0x5000, 0x4004, 1]),
                (LOAD_READ, vec![0x5000, 0x4005, 1]),
            ]
        );
    }

    #[test]
    fn reference_data_construct_assign_and_compare() {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        let other = e.mem.alloc(0x14);
        e.mem.set_u32(this, 0xffff_ffff);
        let (result, log) = run(&mut e, 0x0067_c650, &[this]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(log, vec![(ACTION_CLEAR, vec![this + 4])]);
        e.mem.set_u32(other, 0x7f80_0001);
        let (_, log) = run(&mut e, 0x0067_c6f0, &[this, other]);
        assert_eq!(e.mem.u32(this), 0x7fc0_0001);
        assert_eq!(log, vec![(ACTION_ASSIGN, vec![this + 4, other + 4])]);
        // A NaN never equals.
        assert!(e.call(0x0067_c720, &args![this, other]).bool());
        e.mem.set_u32(this, 0x8000_0000);
        e.mem.set_u32(other, 0);
        assert!(!e.call(0x0067_c720, &args![this, other]).bool());
        e.mem.set_u32(other, 1.0f32.to_bits());
        assert!(e.call(0x0067_c720, &args![this, other]).bool());
        e.mem.set_u32(other, 0);
        e.mem.set_u32(SCRATCH + 8, 1);
        assert!(e.call(0x0067_c720, &args![this, other]).bool());
    }
}
