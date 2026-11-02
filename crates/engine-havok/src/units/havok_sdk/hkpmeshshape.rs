//! `Havok SDK/hkpmeshshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00d06f80` (cdecl `(data, byte size)`): returns a word that `calcContentStatistics` hands
/// on to the collector.
const BLOCK_QUERY: u32 = 0x00d0_6f80;
/// `00d213e0` (thiscall, `RET 8`, `(collector, constant)`): the base shape's statistics.
const BASE_CALC_STATISTICS: u32 = 0x00d2_13e0;
/// The string `010d7724` and the constant `01269198` handed to the collector and the base
/// (unconfirmed).
const SUBPART_ARRAY_NAME: u32 = 0x010d_7724;
const BASE_ARGUMENT: u32 = 0x0126_9198;
/// Bytes per entry of the array at +0x34 (`size * 7 * 8`, the exe's shift-and-subtract
/// sequence).
const ENTRY_SIZE: u32 = 0x38;

// Translated from 00dcd140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMeshShape::calcContentStatistics` (Xbox PDB), `RET 8` with the second word unread: the
/// array at +0x34 (size at +0x38, capacity and flags at +0x3C, 0x38 bytes per entry) is
/// reported to the collector's slot `0x14` as `(3, name, data, used bytes,
/// 00d06f80(data, capacity bytes))` when it owns its storage (bit 31 of the capacity word
/// clear) or the byte at `collector + 0x10` is not 0; then `00d213e0(this, collector,
/// constant)`.
pub fn hkp_mesh_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    _unused_1: u32,
) {
    let me = this.addr();
    let capacity = e.mem.u32(me + 0x3c);
    if (capacity as i32) >= 0 || e.mem.u8(collector.addr() + 0x10) != 0 {
        let data = e.mem.u32(me + 0x34);
        let capacity_bytes = (capacity & 0x3fff_ffff).wrapping_mul(ENTRY_SIZE);
        let word = e.call(BLOCK_QUERY, &args![data, capacity_bytes]).u32();
        let used = e.mem.u32(me + 0x38).wrapping_mul(ENTRY_SIZE);
        let data = e.mem.u32(me + 0x34);
        e.vcall(
            collector.addr(),
            0x14,
            &args![3u32, SUBPART_ARRAY_NAME, data, used, word],
        );
    }
    e.call(BASE_CALC_STATISTICS, &args![this, collector, BASE_ARGUMENT]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00dcd140,
        hkp_mesh_shape_calc_content_statistics(Ptr, Ptr, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const SLOT_DOUBLE: u32 = 0x0900_0011;

    #[test]
    fn the_array_is_reported_only_when_owned_or_asked_for() {
        let mut e = Engine::new();
        let log = Rc::new(RefCell::new(Vec::<Vec<u32>>::new()));
        for (addr, tag, result) in [
            (BLOCK_QUERY, 1u32, 0xabc),
            (SLOT_DOUBLE, 2, 0),
            (BASE_CALC_STATISTICS, 3, 0),
        ] {
            let sink = Rc::clone(&log);
            e.register_double(addr, move |_, a| {
                let mut row = vec![tag];
                row.extend_from_slice(a);
                sink.borrow_mut().push(row);
                Ret {
                    eax: result,
                    ..Ret::default()
                }
            });
        }
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x14, SLOT_DOUBLE);
        let collector = e.mem.alloc(0x20);
        e.mem.set_u32(collector, vtable);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x34, 0x7000);
        e.mem.set_u32(this + 0x38, 3);
        e.mem.set_u32(this + 0x3c, 5);
        e.call(0x00dcd140, &args![this, collector, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![1, 0x7000, 5 * 56],
                vec![2, collector, 3, SUBPART_ARRAY_NAME, 0x7000, 3 * 56, 0xabc],
                vec![3, this, collector, BASE_ARGUMENT],
            ]
        );
        // Storage that is not owned is skipped unless the collector asks for it.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x3c, 0x8000_0005);
        e.call(0x00dcd140, &args![this, collector, 0u32]);
        assert_eq!(*log.borrow(), vec![vec![3, this, collector, BASE_ARGUMENT]]);
        log.borrow_mut().clear();
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00dcd140, &args![this, collector, 0u32]);
        assert_eq!(log.borrow().len(), 3);
        assert_eq!(log.borrow()[0], vec![1, 0x7000, 5 * 56]);
    }
}
