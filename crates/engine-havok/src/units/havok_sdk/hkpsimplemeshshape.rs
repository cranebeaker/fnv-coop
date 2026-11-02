//! `Havok SDK/hkpsimplemeshshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkStatisticsCollector::getAllocatedSize` (`00d06f80`, cdecl: block, size).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// `hkpShapeCollection::calcContentStatistics` (`00d213e0`, thiscall on the shape).
const SHAPE_COLLECTION_CALC_CONTENT_STATISTICS: u32 = 0x00d2_13e0;
/// The class description the base shape collection reports under.
const SHAPE_COLLECTION_CLASS: u32 = 0x0126_9198;
/// The strings the chunks are reported with: "Vertices", "Triangles" and "MaterialIds".
const VERTICES_NAME: u32 = 0x010e_ba24;
const TRIANGLES_NAME: u32 = 0x0101_ebc4;
const MATERIAL_IDS_NAME: u32 = 0x010e_ba18;

// Translated from 00e3c4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleMeshShape::calcContentStatistics` (Xbox PDB), `(this, collector)`: for each of the
/// three arrays (vertices at +0x18, size +0x1c, capacity word +0x20, 16-byte elements;
/// triangles at +0x24, +0x28, +0x2c, 16-byte elements; material ids at +0x30, +0x34, +0x38,
/// raw bytes) that does not have a negative capacity word, or all of them when the byte at
/// +0x10 of the collector is set, asks `getAllocatedSize(data, (capacity & 0x3fffffff) *
/// element size)` and calls the collector's slot `0x14` with `(3, name, data, size *
/// element size (the raw word for the ids), allocated)`. Ends with
/// `hkpShapeCollection::calcContentStatistics(this, collector, 0x01269198)`.
/// The second stack word (`RET 8`; callers pass the class record) is not read.
pub fn hkp_simple_mesh_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    _unused_2: u32,
) {
    let base = this.addr();
    let verbose = e.mem.u8(collector.addr() + 0x10) != 0;
    // (offset of the array, name, element shift); the size and capacity word follow the data.
    let chunks = [
        (0x18u32, VERTICES_NAME, 4u32),
        (0x24, TRIANGLES_NAME, 4),
        (0x30, MATERIAL_IDS_NAME, 0),
    ];
    for (data_at, name, shift) in chunks {
        let (size_at, capacity_at) = (data_at + 4, data_at + 8);
        let capacity = e.mem.u32(base + capacity_at);
        if (capacity as i32) < 0 && !verbose {
            continue;
        }
        let data = e.mem.u32(base + data_at);
        let allocated = e
            .call(
                GET_ALLOCATED_SIZE,
                &args![data, (capacity & 0x3fff_ffff) << shift],
            )
            .u32();
        let used = e.mem.u32(base + size_at) << shift;
        e.vcall(
            collector.addr(),
            0x14,
            &args![3u32, name, data, used, allocated],
        );
    }
    e.call(
        SHAPE_COLLECTION_CALC_CONTENT_STATISTICS,
        &args![this, collector, SHAPE_COLLECTION_CLASS],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e3_c4e0,
        hkp_simple_mesh_shape_calc_content_statistics(Ptr, Ptr, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type EventLog = Rc<RefCell<Vec<Vec<u32>>>>;

    const COLLECTOR_VTABLE: u32 = 0x0900_3000;
    const REPORT_DOUBLE: u32 = 0x0900_0014;

    /// A mesh with the three arrays set up and a collector whose slot `0x14` logs
    /// `[slot, this, arguments...]`; the base-class and size callees log too.
    fn setup(e: &mut Engine) -> (u32, u32, EventLog) {
        let log: EventLog = Rc::new(RefCell::new(Vec::new()));
        let mut slots = vec![0u32; 8];
        slots[5] = REPORT_DOUBLE;
        e.put_vtable(COLLECTOR_VTABLE, &slots);
        let report_log = Rc::clone(&log);
        e.register_double(REPORT_DOUBLE, move |_, a| {
            let mut entry = vec![0x14];
            entry.extend_from_slice(a);
            report_log.borrow_mut().push(entry);
            Ret::default()
        });
        e.register_double(GET_ALLOCATED_SIZE, |_, a| Ret {
            eax: 0x100 + a[1],
            ..Ret::default()
        });
        let base_log = Rc::clone(&log);
        e.register_double(SHAPE_COLLECTION_CALC_CONTENT_STATISTICS, move |_, a| {
            let mut entry = vec![SHAPE_COLLECTION_CALC_CONTENT_STATISTICS];
            entry.extend_from_slice(a);
            base_log.borrow_mut().push(entry);
            Ret::default()
        });
        let collector = e.mem.alloc(0x40);
        e.mem.set_u32(collector, COLLECTOR_VTABLE);
        let mesh = e.mem.alloc(0x60);
        for (data_at, data, size, capacity) in [
            (0x18u32, 0x5000u32, 3u32, 0x4000_0004u32),
            (0x24, 0x6000, 2, 8),
            (0x30, 0x7000, 5, 0x4000_0009),
        ] {
            e.mem.set_u32(mesh + data_at, data);
            e.mem.set_u32(mesh + data_at + 4, size);
            e.mem.set_u32(mesh + data_at + 8, capacity);
        }
        (mesh, collector, log)
    }

    #[test]
    fn every_array_is_reported_and_then_the_base_collection() {
        let mut e = Engine::new();
        let (mesh, collector, log) = setup(&mut e);
        e.call(0x00e3_c4e0, &args![mesh, collector, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![
                    0x14,
                    collector,
                    3,
                    VERTICES_NAME,
                    0x5000,
                    0x30,
                    0x100 + 0x40
                ],
                vec![
                    0x14,
                    collector,
                    3,
                    TRIANGLES_NAME,
                    0x6000,
                    0x20,
                    0x100 + 0x80
                ],
                vec![0x14, collector, 3, MATERIAL_IDS_NAME, 0x7000, 5, 0x100 + 9],
                vec![
                    SHAPE_COLLECTION_CALC_CONTENT_STATISTICS,
                    mesh,
                    collector,
                    SHAPE_COLLECTION_CLASS
                ],
            ]
        );
    }

    #[test]
    fn arrays_with_a_negative_capacity_are_reported_only_in_verbose_mode() {
        let mut e = Engine::new();
        let (mesh, collector, log) = setup(&mut e);
        for data_at in [0x18u32, 0x24, 0x30] {
            e.mem.set_u32(mesh + data_at + 8, 0x8000_0003);
        }
        e.call(0x00e3_c4e0, &args![mesh, collector, 0u32]);
        assert_eq!(log.borrow().len(), 1);
        log.borrow_mut().clear();
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00e3_c4e0, &args![mesh, collector, 0u32]);
        let log = log.borrow();
        assert_eq!(log.len(), 4);
        // The size asked for uses the capacity masked with 0x3fffffff: 0x8000_0003 -> 3 * 16.
        assert_eq!(
            log[0],
            vec![
                0x14,
                collector,
                3,
                VERTICES_NAME,
                0x5000,
                0x30,
                0x100 + 0x30
            ]
        );
    }
}
