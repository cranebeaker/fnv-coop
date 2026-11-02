//! `BSHavok/bhkpointtopathconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkPointToPathConstraintCinfo` (vtable `010cc2e0`, RTTI-confirmed) keeps the
//! constraint data (0xB0 bytes) at `+4`: a `float` at `+0x1C`, a byte at `+0x20` and
//! 16-byte blocks from `+0x30` to `+0xA0`. Slot `4` is `CloneData`, `8` `LoadBinary`,
//! `0xC` `SaveBinary` and `0x10` `CreateData`.
//!
//! `CloneData` and `CreateData` raise the per-thread allocation word at `TLS + 0x2b4` to 0x10
//! for their duration and put the old value back; their exception-unwinding frames
//! (`FS:[0]` registrations) are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhklimitedhingeconstraint::{allocate_sized, enter_scope, stream_transfer};

/// Size of the constraint data block.
const DATA_SIZE: u32 = 0xb0;
/// `hkpPointToPathConstraintData` constructor (`00dee750`, `ECX` = the block, answers it).
const DATA_CONSTRUCT: u32 = 0x00de_e750;
/// `hkConstraintCinfo::CloneData` (`00c8a1b0`, `(this, data, info)`), `LoadBinary`
/// (`00c8a230`, `(this, stream)`) and `SaveBinary` (`00c8a280`, `(this, stream)`).
const CINFO_CLONE_DATA: u32 = 0x00c8_a1b0;
const CINFO_LOAD_BINARY: u32 = 0x00c8_a230;
const CINFO_SAVE_BINARY: u32 = 0x00c8_a280;

// Translated from 00d86300 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d86300 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkPointToPathConstraintCinfo::CloneData` (Xbox PDB), slot `4` of the cinfo vtable
/// (`this` is the cinfo, `+4` its data): copies the data block into `target`, which is
/// allocated (0xB0 bytes, size word at `+4`) and constructed first when 0. The copy is the
/// `float` at `+0x1C`, the byte at `+0x20` and the eight 16-byte blocks `+0x30..+0xA0`;
/// unless the scale `info + 0x10` equals 1.0 (a NaN is not equal) all four lanes of the
/// blocks at `+0x60` and `+0xA0` are multiplied by it. `hkConstraintCinfo::CloneData`
/// (`00c8a1b0`) finishes. The allocation-scope word is 0x10 meanwhile.
pub fn hk_point_to_path_constraint_cinfo_clone_data(
    e: &mut Engine,
    this: Ptr,
    target: u32,
    info: u32,
) {
    let (word, saved) = enter_scope(e);
    let data = e.mem.u32(this.addr() + 4);
    let mut target = target;
    if target == 0 {
        let block = allocate_sized(e, DATA_SIZE);
        target = e.call(DATA_CONSTRUCT, &args![block]).u32();
    }
    let gain = x87_quiet_bits(e.mem.u32(data + 0x1c));
    e.mem.set_u32(target + 0x1c, gain);
    let flag = e.mem.u8(data + 0x20);
    e.mem.set_u8(target + 0x20, flag);
    for block in 3..11u32 {
        let bytes = e.mem.bytes(data + block * 0x10, 0x10);
        e.mem.write(target + block * 0x10, &bytes);
    }
    let scale = f32::from_bits(x87_quiet_bits(e.mem.u32(info + 0x10)));
    if scale != 1.0 {
        for block in [0x60, 0xa0] {
            for lane in 0..4 {
                let address = target + block + 4 * lane;
                let value = e.mem.f32(address);
                e.mem.set_f32(address, scale * value);
            }
        }
    }
    e.call(CINFO_CLONE_DATA, &args![this, target, info]);
    e.mem.set_u32(word, saved);
}

// Translated from 00d86470 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d86470 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkPointToPathConstraintCinfo::CreateData` (Xbox PDB), slot `0x10` of the cinfo vtable
/// (`this` is the cinfo): when `+4` (the data) is empty, allocates 0xB0 bytes (size word at
/// `+4`), constructs the data block there (`00dee750`) and keeps what that returns.
/// Answers 0xB0. The allocation-scope word is 0x10 meanwhile.
pub fn hk_point_to_path_constraint_cinfo_create_data(e: &mut Engine, this: Ptr) -> u32 {
    let (word, saved) = enter_scope(e);
    if e.mem.u32(this.addr() + 4) == 0 {
        let block = allocate_sized(e, DATA_SIZE);
        let data = e.call(DATA_CONSTRUCT, &args![block]).u32();
        e.mem.set_u32(this.addr() + 4, data);
    }
    e.mem.set_u32(word, saved);
    DATA_SIZE
}

// Translated from 00d86520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPointToPathConstraintCinfo::LoadBinary` (Xbox PDB), slot `8` of the cinfo vtable
/// (`this` is the cinfo, `stream` the `NiStream`, whose reader is at `+0x24C`): the base
/// `LoadBinary` (`00c8a230`), then reads 4 bytes to data `+0x1C`, 1 byte to `+0x20` and
/// 0x40 bytes each to `+0x30` and `+0x70`, advancing the reader's position.
pub fn hk_point_to_path_constraint_cinfo_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(CINFO_LOAD_BINARY, &args![this, stream]);
    let data = e.mem.u32(this.addr() + 4);
    for (offset, size) in [(0x1c, 4), (0x20, 1), (0x30, 0x40), (0x70, 0x40)] {
        let io = e.mem.u32(stream + 0x24c);
        stream_transfer(e, io, 8, data + offset, size, true);
    }
}

// Translated from 00d865d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPointToPathConstraintCinfo::SaveBinary` (Xbox PDB), slot `0xC` of the cinfo vtable
/// (`this` is the cinfo, `stream` the `NiStream`, whose writer is at `+0x250`): the base
/// `SaveBinary` (`00c8a280`), then writes 4 bytes from data `+0x1C`, 1 byte from `+0x20`
/// and 0x40 bytes each from `+0x30` and `+0x70`, advancing the writer's position.
pub fn hk_point_to_path_constraint_cinfo_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let data = e.mem.u32(this.addr() + 4);
    e.call(CINFO_SAVE_BINARY, &args![this, stream]);
    for (offset, size) in [(0x1c, 4), (0x20, 1), (0x30, 0x40), (0x70, 0x40)] {
        let io = e.mem.u32(stream + 0x250);
        stream_transfer(e, io, 0xc, data + offset, size, true);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d8_6300,
            hk_point_to_path_constraint_cinfo_clone_data(Ptr, u32, u32)
        ),
        entry!(
            0x00d8_6470,
            hk_point_to_path_constraint_cinfo_create_data(Ptr) -> u32
        ),
        entry!(
            0x00d8_6520,
            hk_point_to_path_constraint_cinfo_load_binary(Ptr, u32)
        ),
        entry!(
            0x00d8_65d0,
            hk_point_to_path_constraint_cinfo_save_binary(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhklimitedhingeconstraint::support::*;
    use super::*;
    use std::rc::Rc;

    fn source_data(e: &mut Engine) -> u32 {
        let data = e.mem.alloc(DATA_SIZE);
        for word in 0..(DATA_SIZE / 4) {
            e.mem.set_f32(data + word * 4, 1.0 + word as f32);
        }
        // A signalling NaN comes out quiet through FLD / FSTP.
        e.mem.set_u32(data + 0x1c, 0x7fa0_0000);
        e.mem.set_u8(data + 0x20, 0x5a);
        data
    }

    #[test]
    fn clone_data_copies_the_blocks_and_applies_the_scale_to_two_of_them() {
        let (mut e, word, _) = engine_with_allocator();
        let data = source_data(&mut e);
        let cinfo = e.mem.alloc(0x14);
        e.mem.set_u32(cinfo + 4, data);
        let target = e.mem.alloc(DATA_SIZE);
        let info = e.mem.alloc(0x20);
        e.mem.set_f32(info + 0x10, 1.0);
        let log: Calls = Rc::default();
        record(&mut e, CINFO_CLONE_DATA, 0, &log);
        record(&mut e, DATA_CONSTRUCT, 0, &log);
        e.call(0x00d8_6300, &args![cinfo, target, info]);
        assert_eq!(e.mem.u32(target + 0x1c), 0x7fe0_0000);
        assert_eq!(e.mem.u8(target + 0x20), 0x5a);
        assert_eq!(
            e.mem.bytes(target + 0x30, 0x80),
            e.mem.bytes(data + 0x30, 0x80)
        );
        // Outside the copied ranges nothing is written.
        assert_eq!(e.mem.u32(target + 0x24), 0);
        assert!(calls_to(&log, DATA_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&log, CINFO_CLONE_DATA), [[cinfo, target, info]]);
        assert_eq!(e.mem.u32(word), 0x77);
        // A scale other than 1.0 multiplies the blocks at +0x60 and +0xA0 only.
        e.mem.set_f32(info + 0x10, 2.0);
        e.call(0x00d8_6300, &args![cinfo, target, info]);
        for lane in 0..4 {
            assert_eq!(
                e.mem.f32(target + 0x60 + 4 * lane),
                2.0 * e.mem.f32(data + 0x60 + 4 * lane)
            );
            assert_eq!(
                e.mem.f32(target + 0xa0 + 4 * lane),
                2.0 * e.mem.f32(data + 0xa0 + 4 * lane)
            );
            assert_eq!(
                e.mem.f32(target + 0x50 + 4 * lane),
                e.mem.f32(data + 0x50 + 4 * lane)
            );
        }
        // A NaN scale counts as not equal to 1.0.
        e.mem.set_f32(info + 0x10, f32::NAN);
        e.call(0x00d8_6300, &args![cinfo, target, info]);
        assert!(e.mem.f32(target + 0x60).is_nan());
    }

    #[test]
    fn clone_data_allocates_and_constructs_a_missing_target() {
        let (mut e, word, fresh) = engine_with_allocator();
        let data = source_data(&mut e);
        let cinfo = e.mem.alloc(0x14);
        e.mem.set_u32(cinfo + 4, data);
        let info = e.mem.alloc(0x20);
        e.mem.set_f32(info + 0x10, 1.0);
        let log: Calls = Rc::default();
        record(&mut e, CINFO_CLONE_DATA, 0, &log);
        record(&mut e, DATA_CONSTRUCT, fresh, &log);
        e.call(0x00d8_6300, &args![cinfo, 0u32, info]);
        assert_eq!(e.mem.u16(fresh + 4), 0xb0);
        assert_eq!(calls_to(&log, DATA_CONSTRUCT), [[fresh]]);
        assert_eq!(e.mem.u8(fresh + 0x20), 0x5a);
        assert_eq!(calls_to(&log, CINFO_CLONE_DATA), [[cinfo, fresh, info]]);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn create_data_builds_the_block_once() {
        let (mut e, word, fresh) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, DATA_CONSTRUCT, fresh, &log);
        let cinfo = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00d8_6470, &args![cinfo]).u32(), 0xb0);
        assert_eq!(e.mem.u32(cinfo + 4), fresh);
        assert_eq!(e.mem.u16(fresh + 4), 0xb0);
        assert_eq!(calls_to(&log, DATA_CONSTRUCT), [[fresh]]);
        assert_eq!(e.mem.u32(word), 0x77);
        // Already built: nothing is allocated or constructed again.
        assert_eq!(e.call(0x00d8_6470, &args![cinfo]).u32(), 0xb0);
        assert_eq!(calls_to(&log, DATA_CONSTRUCT).len(), 1);
    }

    #[test]
    fn load_binary_reads_the_four_fields_through_the_reader() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        let stream = stream_fixture(&mut e, &log);
        let io = e.mem.u32(stream + 0x24c);
        record(&mut e, CINFO_LOAD_BINARY, 0, &log);
        let data = e.mem.alloc(DATA_SIZE);
        let cinfo = e.mem.alloc(0x14);
        e.mem.set_u32(cinfo + 4, data);
        e.call(0x00d8_6520, &args![cinfo, stream]);
        let reads = calls_to(&log, 0x0900_0008);
        let targets: Vec<(u32, u32, u32)> =
            reads.iter().map(|r| (r[0], r[1] - data, r[2])).collect();
        assert_eq!(
            targets,
            [
                (io, 0x1c, 4),
                (io, 0x20, 1),
                (io, 0x30, 0x40),
                (io, 0x70, 0x40)
            ]
        );
        assert!(reads.iter().all(|r| r[4] == 1));
        assert_eq!(e.mem.u32(io + 4), 4 + 1 + 0x40 + 0x40);
        assert_eq!(calls_to(&log, CINFO_LOAD_BINARY), [[cinfo, stream]]);
        // The fixture's reader fills 0x10, 0x11, ...
        assert_eq!(e.mem.u32(data + 0x1c), 0x1312_1110);
        assert_eq!(e.mem.u8(data + 0x20), 0x10);
    }

    #[test]
    fn save_binary_writes_the_four_fields_after_the_base_save() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        let stream = stream_fixture(&mut e, &log);
        let io = e.mem.u32(stream + 0x250);
        record(&mut e, CINFO_SAVE_BINARY, 0, &log);
        let data = e.mem.alloc(DATA_SIZE);
        let cinfo = e.mem.alloc(0x14);
        e.mem.set_u32(cinfo + 4, data);
        e.call(0x00d8_65d0, &args![cinfo, stream]);
        let writes = calls_to(&log, 0x0900_000c);
        let targets: Vec<(u32, u32, u32)> =
            writes.iter().map(|w| (w[0], w[1] - data, w[2])).collect();
        assert_eq!(
            targets,
            [
                (io, 0x1c, 4),
                (io, 0x20, 1),
                (io, 0x30, 0x40),
                (io, 0x70, 0x40)
            ]
        );
        assert_eq!(e.mem.u32(io + 4), 4 + 1 + 0x40 + 0x40);
        assert_eq!(calls_to(&log, CINFO_SAVE_BINARY), [[cinfo, stream]]);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order[0], CINFO_SAVE_BINARY);
    }
}
