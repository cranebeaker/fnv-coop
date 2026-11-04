//! `fallout shared/pathfinding/run_006e1280` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006e6da0`: placement `operator new` (`cdecl`: size, address; returns
/// the address).
const PLACEMENT_NEW: u32 = 0x006e_6da0;

// Translated from 006e1280 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: "constructs" `count` 1-byte elements at `elements`
/// (placement `new` of size 1 on each; the type has no constructor). `this`
/// is kept but not read.
pub fn fn_006e1280(e: &mut Engine, _this: Ptr, elements: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        e.call(PLACEMENT_NEW, &args![1u32, elements.wrapping_add(index)]);
        index = index.wrapping_add(1);
    }
}

/// `006d07d0` (`thiscall`, `RET 0xc`: destination, source, count): moves
/// bytes in the safe direction.
const MOVE_BYTES_006D07D0: u32 = 0x006d_07d0;
/// `006a8500` (`thiscall`): run on the array once the elements have moved
/// to the new buffer.
const ARRAY_RELEASE_006A8500: u32 = 0x006a_8500;
/// `PathingLOSMap`'s vtable (`0106cf5c`, RTTI `.?AVPathingLOSMap@@`).
const PATHING_LOS_MAP_VTABLE: u32 = 0x0106_cf5c;
/// `004968b0` (`thiscall`): the `NiRefObject` base's constructor.
const REF_OBJECT_CONSTRUCT_004968B0: u32 = 0x0049_68b0;
/// `006e2080` (`thiscall`, `RET 4`): the `NiTMap` at `+8`'s constructor,
/// with 0x25 buckets.
const MAP_CONSTRUCT_006E2080: u32 = 0x006e_2080;
/// `0042f800` (`thiscall`): the `BSSimpleArray` at `+0x1c`'s constructor.
const ARRAY_CONSTRUCT_0042F800: u32 = 0x0042_f800;

// Translated from 006e12c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the byte array (buffer `+4`, capacity `+0xc`) a buffer of
/// `capacity` (`thiscall`; capacity, count; `RET 8`; no name in the engine
/// map): without a buffer, virtual slot 4 with the capacity, which is also
/// stored; when `count` equals the capacity, slot 0xc with the old buffer
/// and the capacity; else a new buffer from slot 4, the `count` bytes moved
/// (`006d07d0`), `006a8500` on the array and the new buffer kept.
pub fn fn_006e12c0(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    let base = this.addr();
    if e.mem.u32(base + 4) == 0 {
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, capacity);
    } else if count == e.mem.u32(base + 0xc) {
        let old = e.mem.u32(base + 4);
        let buffer = e.vcall(base, 0xc, &args![old, capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
    } else {
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        let old = e.mem.u32(base + 4);
        e.call(MOVE_BYTES_006D07D0, &args![this, buffer, old, count]);
        e.call(ARRAY_RELEASE_006A8500, &args![this]);
        e.mem.set_u32(base + 4, buffer);
    }
}

// Translated from 006e1370 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e1370 - the exception-unwinding frame (FS:[0]) is not translated.
/// `PathingLOSMap`'s constructor (`thiscall`; the centre as three words
/// and the radius; `RET 0x10`; Xbox PDB layout, the same offsets on PC):
/// the `NiRefObject` base (`004968b0`), the vtable, `NavMeshToOffsetMap`
/// at `+8` with 0x25 buckets (`006e2080`), `NavMeshes` at `+0x1c`
/// (`0042f800`); then `Center` (`+0x3c`) is copied, `fRadius` (`+0x48`)
/// stored through `FLD`/`FSTP`, the three triangle counts (`+0x2c` to
/// `+0x34`) and `pTriangleScores` (`+0x18`) cleared, and `iMaxScore`
/// (`+0x38`) and `iMinScore` (`+0x39`) are 0. Returns `this`.
pub fn fn_006e1370(
    e: &mut Engine,
    this: Ptr,
    center_x: u32,
    center_y: u32,
    center_z: u32,
    radius: f32,
) -> Ptr {
    let base = this.addr();
    e.call(REF_OBJECT_CONSTRUCT_004968B0, &args![this]);
    e.mem.set_u32(base, PATHING_LOS_MAP_VTABLE);
    e.call(MAP_CONSTRUCT_006E2080, &args![base + 8, 0x25u32]);
    e.call(ARRAY_CONSTRUCT_0042F800, &args![base + 0x1c]);
    e.mem.set_u32(base + 0x3c, center_x);
    e.mem.set_u32(base + 0x40, center_y);
    e.mem.set_u32(base + 0x44, center_z);
    e.mem.set_u32(base + 0x48, x87_quiet_bits(radius.to_bits()));
    e.mem.set_u32(base + 0x2c, 0);
    e.mem.set_u32(base + 0x30, 0);
    e.mem.set_u32(base + 0x34, 0);
    e.mem.set_u32(base + 0x18, 0);
    e.mem.set_u8(base + 0x38, 0);
    e.mem.set_u8(base + 0x39, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_1280, fn_006e1280(Ptr, u32, u32)),
        entry!(0x006e_12c0, fn_006e12c0(Ptr, u32, u32)),
        entry!(0x006e_1370, fn_006e1370(Ptr, u32, u32, u32, f32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn places_each_byte() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| a[1].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006e_1280, &args![0u32, 0x5000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        let placed: Vec<_> = log
            .iter()
            .filter(|c| c.0 == PLACEMENT_NEW)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            placed,
            vec![vec![1, 0x5000], vec![1, 0x5001], vec![1, 0x5002]]
        );
    }

    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn at_006e12c0_allocates_reallocates_or_moves() {
        let mut e = Engine::new();
        e.register(MOVE_BYTES_006D07D0, |_, _| Ret::default());
        e.register(ARRAY_RELEASE_006A8500, |_, _| Ret::default());
        e.register(0x0f00_0004, |_, a| (0x9000 + a[1]).into_ret());
        e.register(0x0f00_000c, |_, a| (0xa000 + a[2]).into_ret());
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_0004, 0, 0x0f00_000c]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x0f00_1000);
        e.call(0x006e_12c0, &args![this, 6u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0x9006);
        assert_eq!(e.mem.u32(this + 0xc), 6);
        e.call_log = Some(vec![]);
        e.call(0x006e_12c0, &args![this, 9u32, 6u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0f00_000c), vec![vec![this, 0x9006, 9]]);
        assert_eq!(e.mem.u32(this + 4), 0xa009);
        e.call_log = Some(vec![]);
        e.call(0x006e_12c0, &args![this, 0x10u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MOVE_BYTES_006D07D0),
            vec![vec![this, 0x9010, 0xa009, 3]]
        );
        assert_eq!(calls(&log, ARRAY_RELEASE_006A8500), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 4), 0x9010);
        assert_eq!(e.mem.u32(this + 0xc), 6);
    }

    #[test]
    fn at_006e1370_constructs_a_los_map() {
        let mut e = Engine::new();
        for addr in [
            REF_OBJECT_CONSTRUCT_004968B0,
            MAP_CONSTRUCT_006E2080,
            ARRAY_CONSTRUCT_0042F800,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x50);
        for offset in (0..0x50).step_by(4) {
            e.mem.set_u32(this + offset, 0xcccc_cccc);
        }
        e.call_log = Some(vec![]);
        let result = e
            .call(
                0x006e_1370,
                &args![this, 1.0f32, 2.0f32, 3.0f32, f32::from_bits(0x7f80_0001)],
            )
            .u32();
        assert_eq!(result, this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, REF_OBJECT_CONSTRUCT_004968B0), vec![vec![this]]);
        assert_eq!(
            calls(&log, MAP_CONSTRUCT_006E2080),
            vec![vec![this + 8, 0x25]]
        );
        assert_eq!(
            calls(&log, ARRAY_CONSTRUCT_0042F800),
            vec![vec![this + 0x1c]]
        );
        assert_eq!(e.mem.u32(this), PATHING_LOS_MAP_VTABLE);
        assert_eq!(e.mem.f32(this + 0x3c), 1.0);
        assert_eq!(e.mem.f32(this + 0x40), 2.0);
        assert_eq!(e.mem.f32(this + 0x44), 3.0);
        // The radius goes through FLD/FSTP: a signalling NaN comes out quiet.
        assert_eq!(e.mem.u32(this + 0x48), 0x7fc0_0001);
        for offset in [0x18, 0x2c, 0x30, 0x34] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x38), 0xcccc_0000);
        assert_eq!(e.mem.u32(this + 0x4c), 0xcccc_cccc);
    }
}
