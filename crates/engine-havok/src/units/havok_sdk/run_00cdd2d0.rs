//! `Havok SDK/run_00cdd2d0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Copies one 16-byte vector (`MOVAPS` load and store), bit for bit.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from.wrapping_add(4 * lane));
        e.mem.set_u32(to.wrapping_add(4 * lane), word);
    }
}

/// For every entry `i` of the object's flag table (`+0x18`: entries of 8 bytes, the byte at
/// `+4` is the flag; the count is the signed word at `+0x1c`) whose flag is not zero, copies
/// the first 16 bytes of the `0x30`-byte record `i` from `from_base + i * 0x30` to `to_base +
/// i * 0x30`. The flag table's data pointer is read again for every entry.
fn copy_flagged_records(e: &mut Engine, object: u32, from_base: u32, to_base: u32) {
    let count = e.mem.i32(object + 0x1c);
    let mut index = 0u32;
    if count > 0 {
        let mut offset = 0u32;
        loop {
            let flags = e.mem.u32(object + 0x18);
            if e.mem.u8(flags.wrapping_add(index * 8 + 4)) != 0 {
                copy_vector(
                    e,
                    from_base.wrapping_add(offset),
                    to_base.wrapping_add(offset),
                );
            }
            index += 1;
            offset = offset.wrapping_add(0x30);
            if index as i32 >= count {
                break;
            }
        }
    }
}

// Translated from 00cdd2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(object, destination)`: copies the first vector of each flagged `0x30`-byte record
/// of the object's own record array (the data pointer at `object + 0x24`, read again for every
/// entry) to the same record of `destination` (see [`copy_flagged_records`]). The class is
/// unconfirmed.
pub fn fn_00cdd2d0(e: &mut Engine, object: Ptr, destination: Ptr) {
    let object = object.addr();
    let count = e.mem.i32(object + 0x1c);
    let mut index = 0u32;
    if count > 0 {
        let mut offset = 0u32;
        loop {
            let flags = e.mem.u32(object + 0x18);
            if e.mem.u8(flags.wrapping_add(index * 8 + 4)) != 0 {
                let records = e.mem.u32(object + 0x24);
                copy_vector(
                    e,
                    records.wrapping_add(offset),
                    destination.addr().wrapping_add(offset),
                );
            }
            index += 1;
            offset = offset.wrapping_add(0x30);
            if index as i32 >= count {
                break;
            }
        }
    }
}

// Translated from 00cdd320 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(object, source, destination)`: copies the first vector of each flagged `0x30`-byte
/// record of `source` to the same record of `destination` (see [`copy_flagged_records`]).
/// The class is unconfirmed.
pub fn fn_00cdd320(e: &mut Engine, object: Ptr, source: Ptr, destination: Ptr) {
    copy_flagged_records(e, object.addr(), source.addr(), destination.addr());
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd_d2d0, fn_00cdd2d0(Ptr, Ptr)),
        entry!(0x00cd_d320, fn_00cdd320(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An object with a flag table of four entries (flags 1, 0, 2, 0) and records of `0x30`
    /// bytes whose words are numbered, in the object (`+0x24`) and in `other`.
    fn rig() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        let flags = e.mem.alloc(0x20);
        for (i, flag) in [1u8, 0, 2, 0].into_iter().enumerate() {
            e.mem.set_u8(flags + 8 * i as u32 + 4, flag);
        }
        e.mem.set_u32(object + 0x18, flags);
        e.mem.set_u32(object + 0x1c, 4);
        let records = e.mem.alloc(0x30 * 4);
        let other = e.mem.alloc(0x30 * 4);
        for i in 0..48 {
            e.mem.set_u32(records + 4 * i, 0x1000 + i);
            e.mem.set_u32(other + 4 * i, 0x2000 + i);
        }
        e.mem.set_u32(object + 0x24, records);
        (e, object, records, other)
    }

    fn word(e: &Engine, base: u32, record: u32, index: u32) -> u32 {
        e.mem.u32(base + 0x30 * record + 4 * index)
    }

    /// Only the flagged records change, and only their first four words.
    #[test]
    fn own_records_are_copied_where_flagged() {
        let (mut e, object, records, other) = rig();
        e.call(0x00cd_d2d0, &args![object, other]);
        for lane in 0..4 {
            assert_eq!(word(&e, other, 0, lane), word(&e, records, 0, lane));
            assert_eq!(word(&e, other, 2, lane), word(&e, records, 2, lane));
            assert_eq!(word(&e, other, 1, lane), 0x2000 + 12 + lane);
            assert_eq!(word(&e, other, 3, lane), 0x2000 + 36 + lane);
        }
        assert_eq!(word(&e, other, 0, 4), 0x2000 + 4);
        assert_eq!(word(&e, other, 2, 4), 0x2000 + 24 + 4);
    }

    /// The same copy from an explicit source.
    #[test]
    fn explicit_source_records_are_copied_where_flagged() {
        let (mut e, object, records, other) = rig();
        e.call(0x00cd_d320, &args![object, records, other]);
        assert_eq!(word(&e, other, 0, 3), word(&e, records, 0, 3));
        assert_eq!(word(&e, other, 2, 0), word(&e, records, 2, 0));
        assert_eq!(word(&e, other, 1, 0), 0x2000 + 12);
        assert_eq!(word(&e, other, 0, 4), 0x2000 + 4);
        // A count of zero copies nothing.
        e.mem.set_u32(object + 0x1c, 0);
        e.mem.set_u32(records, 0x7777);
        e.call(0x00cd_d320, &args![object, records, other]);
        assert_ne!(word(&e, other, 0, 0), 0x7777);
    }
}
