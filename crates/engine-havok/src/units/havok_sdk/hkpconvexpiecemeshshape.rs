//! `Havok SDK/hkpconvexpiecemeshshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use crate::prelude::*;

/// `getAllocatedSize` of the statistics collector (`00d06f80`, cdecl: pointer, bytes).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// The base report of the shape collection (`00d213e0`, thiscall on the shape: collector, the
/// class record).
const BASE_CALC_CONTENT_STATISTICS: u32 = 0x00d2_13e0;
/// `"Stream"`, the label of the three stream arrays.
const STREAM_LABEL: u32 = 0x010e_c1d0;
/// `"DisplayMesh"`, the label of the display mesh reference.
const DISPLAY_MESH_LABEL: u32 = 0x010e_c1c4;
/// The class record the base report is given.
const CLASS_RECORD_01269198: u32 = 0x0126_9198;

/// Reports the array at `array` (data pointer, size, capacity word at `+8`) through the
/// collector's slot `0x14` as the exe does for each of the three streams: when the capacity
/// word's sign bit is clear or the collector's byte `+0x10` is set.
fn report_stream(e: &mut Engine, collector: u32, array: u32) {
    let capacity = e.mem.u32(array + 8);
    if (capacity as i32) < 0 && e.mem.u8(collector + 0x10) == 0 {
        return;
    }
    let data = e.mem.u32(array);
    let allocated = e
        .call(
            GET_ALLOCATED_SIZE,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
        )
        .u32();
    let data = e.mem.u32(array);
    let used = e.mem.u32(array + 4).wrapping_mul(4);
    e.vcall(
        collector,
        0x14,
        &args![3u32, STREAM_LABEL, data, used, allocated],
    );
}

// Translated from 00e49410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexPieceMeshShape::calcContentStatistics`: thiscall, two stack words (`RET 8`), the
/// second unused. Reports the three arrays of the piece stream (`this+0x18`; arrays at
/// `+0x14`, `+0x20` and `+0x8`, each data, size, capacity word) as `"Stream"` through the
/// collector's slot `0x14` (kind `3`, the array, its used bytes, the allocated bytes from
/// `00d06f80`) when they own their memory or the collector's byte `+0x10` is set, then the
/// display mesh reference at `this+0x1c` through slot `0xc`, and ends with the base report
/// (`00d213e0`).
pub fn calc_content_statistics(e: &mut Engine, this: Ptr, collector: Ptr, _unused_0: u32) {
    let (this, collector) = (this.addr(), collector.addr());
    for offset in [0x14, 0x20, 0x8] {
        let stream = e.mem.u32(this + 0x18);
        report_stream(e, collector, stream + offset);
    }
    let mesh = e.mem.u32(this + 0x1c);
    e.vcall(collector, 0xc, &args![DISPLAY_MESH_LABEL, mesh, 0u32]);
    e.call(
        BASE_CALC_CONTENT_STATISTICS,
        &args![this, collector, CLASS_RECORD_01269198],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e49410, calc_content_statistics(Ptr, Ptr, u32))]
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles for the callees; the collector's slots `0xc` and `0x14` log under their
    /// pseudo addresses `0x0900_000c` / `0x0900_0014`.
    fn rig() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for (address, answer) in [
            (GET_ALLOCATED_SIZE, 0x777u32),
            (BASE_CALC_CONTENT_STATISTICS, 0),
            (0x0900_000c, 0),
            (0x0900_0014, 0),
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax: answer,
                    ..Ret::default()
                }
            });
        }
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0xc, 0x0900_000c);
        e.mem.set_u32(table + 0x14, 0x0900_0014);
        let collector = e.mem.alloc(0x20);
        e.mem.set_u32(collector, table);
        (e, log, collector)
    }

    /// A piece stream whose arrays at `+0x14`, `+0x20`, `+0x8` hold the data pointers
    /// `0x1000`, `0x2000`, `0x3000`, sizes `1`, `2`, `3` and the given capacity words; the
    /// shape has the display mesh `0x9999`. Returns `(shape, stream)`.
    fn shape(e: &mut Engine, capacities: [u32; 3]) -> (u32, u32) {
        let stream = e.mem.alloc(0x40);
        for (index, offset) in [0x14, 0x20, 0x8].into_iter().enumerate() {
            let index = index as u32;
            e.mem.set_u32(stream + offset, 0x1000 * (index + 1));
            e.mem.set_u32(stream + offset + 4, index + 1);
            e.mem
                .set_u32(stream + offset + 8, capacities[index as usize]);
        }
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, stream);
        e.mem.set_u32(this + 0x1c, 0x9999);
        (this, stream)
    }

    #[test]
    fn the_content_statistics_report_three_streams_the_mesh_and_the_base() {
        let (mut e, log, collector) = rig();
        let (this, _) = shape(&mut e, [10, 20, 0x4000_0030]);
        e.call(0x00e4_9410, &args![this, collector, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (GET_ALLOCATED_SIZE, vec![0x1000, 40]),
                (
                    0x0900_0014,
                    vec![collector, 3, STREAM_LABEL, 0x1000, 4, 0x777]
                ),
                (GET_ALLOCATED_SIZE, vec![0x2000, 80]),
                (
                    0x0900_0014,
                    vec![collector, 3, STREAM_LABEL, 0x2000, 8, 0x777]
                ),
                (GET_ALLOCATED_SIZE, vec![0x3000, 0xc0]),
                (
                    0x0900_0014,
                    vec![collector, 3, STREAM_LABEL, 0x3000, 12, 0x777]
                ),
                (0x0900_000c, vec![collector, DISPLAY_MESH_LABEL, 0x9999, 0]),
                (
                    BASE_CALC_CONTENT_STATISTICS,
                    vec![this, collector, CLASS_RECORD_01269198]
                ),
            ]
        );
    }

    #[test]
    fn the_content_statistics_skip_streams_with_borrowed_memory_unless_asked() {
        let (mut e, log, collector) = rig();
        // The first and third arrays do not own their memory.
        let (this, _) = shape(&mut e, [0x8000_0001, 5, 0x8000_0002]);
        e.call(0x00e4_9410, &args![this, collector, 0u32]);
        let addresses: Vec<u32> = log.borrow().iter().map(|(address, _)| *address).collect();
        assert_eq!(
            addresses,
            vec![
                GET_ALLOCATED_SIZE,
                0x0900_0014,
                0x0900_000c,
                BASE_CALC_CONTENT_STATISTICS
            ]
        );
        assert_eq!(log.borrow()[0].1, vec![0x2000, 20]);
        log.borrow_mut().clear();
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00e4_9410, &args![this, collector, 0u32]);
        assert_eq!(log.borrow().len(), 8);
        assert_eq!(log.borrow()[0].1, vec![0x1000, 4]);
    }
}
