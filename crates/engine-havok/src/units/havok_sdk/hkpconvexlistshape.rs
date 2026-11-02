//! `Havok SDK/hkpconvexlistshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `TlsGetValue` import slot (`kernel32`).
const TLS_GET_VALUE: u32 = 0x00fd_f044;
/// The TLS index of the profiling record stream (a word in `.data`).
const PROFILE_SLOT_0126835C: u32 = 0x0126_835c;
/// The record labels: `"TtrcCxList"` and the end label shared with other
/// Havok profiling scopes.
const CX_LIST_LABEL: u32 = 0x010d_1094;
const END_LABEL: u32 = 0x010c_41f8;

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low_00d5e500() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a profiling record (`label`, time stamp) to the thread's record
/// stream (`TlsGetValue` of the index at `0126835c`) while its cursor
/// (`+4`) is below its limit (`+0xc`); a record is 12 bytes.
fn profile_record_00d5e500(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT_0126835C);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low_00d5e500());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

// Translated from 00d5e500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListShape::castRayImpl` (map name). Thiscall, three words
/// (`RET 0xc`); returns `result`. Writes a profiling record, counts a level
/// in the collector (`+0x40`), and walks the child shapes with the iterator
/// embedded at `this + 0x14` (virtual slots `+8` first key, `+0xc` next
/// key, `+0x14` child for a key and a 0x200-byte scratch buffer; `-1`
/// ends). Each child's slot `+0x20` is called with a one-byte output,
/// `input` and `collector`; the key of the last child that reports a hit is
/// kept. The level is then popped; a kept key is stored at
/// `collector + 0x20 + level * 4` and `result` gets whether there was one.
/// `RDTSC` is the host clock.
pub fn hkp_convex_list_shape_cast_ray_impl(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    input: Ptr,
    collector: Ptr,
) -> Ptr {
    profile_record_00d5e500(e, CX_LIST_LABEL);
    let collector_a = collector.addr();
    let level = e.mem.u32(collector_a + 0x40).wrapping_add(1);
    e.mem.set_u32(collector_a + 0x40, level);
    let iterator = this.addr() + 0x14;
    let mut best_key = u32::MAX;
    e.with_stack(0x214, |e, frame| {
        let byte = frame.addr() + 0x1f;
        let buffer = frame.addr() + 0x20;
        let mut key = e.vcall(iterator, 8, &args![]).u32();
        while key != u32::MAX {
            let child = e.vcall(iterator, 0x14, &args![key, buffer]).u32();
            e.vcall(child, 0x20, &args![byte, input, collector]);
            if e.mem.u8(byte) != 0 {
                best_key = key;
            }
            key = e.vcall(iterator, 0xc, &args![key]).u32();
        }
    });
    let level = e.mem.u32(collector_a + 0x40).wrapping_sub(1);
    e.mem.set_u32(collector_a + 0x40, level);
    if best_key != u32::MAX {
        e.mem
            .set_u32(collector_a + 0x20 + level.wrapping_mul(4), best_key);
    }
    e.mem.set_u8(result.addr(), (best_key != u32::MAX) as u8);
    profile_record_00d5e500(e, END_LABEL);
    result
}

/// `getAllocatedSize` of the statistics collector (`00d06f80`, cdecl: pointer, bytes).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// `hkpShapeCollection`-side base report (`00c90360`, thiscall on the shape: collector, the
/// class record).
const BASE_CALC_CONTENT_STATISTICS: u32 = 0x00c9_0360;
/// `"ChildPtrs"`, the label of the child pointer array.
const CHILD_PTRS_LABEL: u32 = 0x010c_c3a4;
/// The class record the base report is given.
const CLASS_RECORD_01268940: u32 = 0x0126_8940;

// Translated from 00d5e480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexListShape::calcContentStatistics`: thiscall, two stack words (`RET 8`), the
/// second unused. When the child pointer array (`this+0x44`, size `this+0x48`, capacity word
/// `this+0x4c`) owns its memory (the capacity word's sign bit is clear) or the collector's
/// byte `+0x10` is set, reports it through the collector's slot `0x14` (kind `3`, label
/// `"ChildPtrs"`, the array, its used bytes, the allocated bytes from `00d06f80`). Then reports
/// each child through slot `0xc` and ends with the base report (`00c90360`).
pub fn calc_content_statistics(e: &mut Engine, this: Ptr, collector: Ptr, _unused_0: u32) {
    let (this, collector) = (this.addr(), collector.addr());
    let capacity = e.mem.u32(this + 0x4c);
    if (capacity as i32) >= 0 || e.mem.u8(collector + 0x10) != 0 {
        let array = e.mem.u32(this + 0x44);
        let allocated = e
            .call(
                GET_ALLOCATED_SIZE,
                &args![array, (capacity & 0x3fff_ffff).wrapping_mul(4)],
            )
            .u32();
        let array = e.mem.u32(this + 0x44);
        let used = e.mem.u32(this + 0x48).wrapping_mul(4);
        e.vcall(
            collector,
            0x14,
            &args![3u32, CHILD_PTRS_LABEL, array, used, allocated],
        );
    }
    let mut index = 0u32;
    while (e.mem.u32(this + 0x48) as i32) > index as i32 {
        let array = e.mem.u32(this + 0x44);
        let child = e.mem.u32(array + 4 * index);
        e.vcall(collector, 0xc, &args![0u32, child, 0u32]);
        index = index.wrapping_add(1);
    }
    e.call(
        BASE_CALC_CONTENT_STATISTICS,
        &args![this, collector, CLASS_RECORD_01268940],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d5e480, calc_content_statistics(Ptr, Ptr, u32)),
        entry!(0x00d5e500, hkp_convex_list_shape_cast_ray_impl(Ptr, Ptr, Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    type CastLog = Rc<RefCell<Vec<Vec<u32>>>>;

    const ITERATOR_FIRST: u32 = 0x0090_5000;
    const ITERATOR_NEXT: u32 = 0x0090_5100;
    const ITERATOR_CHILD: u32 = 0x0090_5200;
    const CHILD_CAST: u32 = 0x0090_5300;

    /// A list shape whose iterator walks keys 0, 1, 2 (then -1); the child
    /// of key `k` reports a hit when `hits[k]`. Returns (engine, this,
    /// collector, record stream, log of the cast calls).
    fn shape_00d5e500(hits: [bool; 3]) -> (Engine, u32, u32, u32, CastLog) {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        let stream = e.mem.alloc(0x40);
        let records = e.mem.alloc(0x30);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x30);
        e.set_global(PROFILE_SLOT_0126835C, 5u32);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a, [5]);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        let mut slots = vec![0u32; 8];
        slots[2] = ITERATOR_FIRST;
        slots[3] = ITERATOR_NEXT;
        slots[5] = ITERATOR_CHILD;
        e.put_vtable(0x0090_2000, &slots);
        let mut child_slots = vec![0u32; 9];
        child_slots[8] = CHILD_CAST;
        e.put_vtable(0x0090_3000, &child_slots);
        let children: Vec<u32> = (0..3)
            .map(|_| {
                let child = e.mem.alloc(8);
                e.mem.set_u32(child, 0x0090_3000);
                child
            })
            .collect();
        e.register(ITERATOR_FIRST, |_, _| Ret::default());
        e.register(ITERATOR_NEXT, |_, a| Ret {
            eax: if a[1] >= 2 { u32::MAX } else { a[1] + 1 },
            ..Ret::default()
        });
        e.register_double(ITERATOR_CHILD, move |_, a| Ret {
            eax: children[a[1] as usize],
            ..Ret::default()
        });
        let casts = Rc::new(RefCell::new(Vec::new()));
        let cast_log = Rc::clone(&casts);
        e.register_double(CHILD_CAST, move |e, a| {
            let index = cast_log.borrow().len();
            cast_log.borrow_mut().push(a.to_vec());
            e.mem.set_u8(a[1], hits[index] as u8);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, 0x0090_2000);
        let collector = e.mem.alloc(0x60);
        e.mem.set_u32(collector + 0x40, 2);
        (e, this, collector, stream, casts)
    }

    #[test]
    fn hkp_convex_list_shape_cast_ray_impl_keeps_the_last_hitting_key() {
        let (mut e, this, collector, stream, casts) = shape_00d5e500([true, false, true]);
        let result = e.mem.alloc(4);
        let input = e.mem.alloc(0x20);
        let back = e.call(
            0x00d5_e500,
            &args![
                Ptr::<()>::new(this),
                Ptr::<()>::new(result),
                Ptr::<()>::new(input),
                Ptr::<()>::new(collector)
            ],
        );
        assert_eq!(back.u32(), result);
        assert_eq!(e.mem.u8(result), 1);
        // Level 2 was pushed and popped; the key of the last hit (2) is
        // stored in the slot of the popped level.
        assert_eq!(e.mem.u32(collector + 0x40), 2);
        assert_eq!(e.mem.u32(collector + 0x20 + 2 * 4), 2);
        let casts = casts.borrow();
        assert_eq!(casts.len(), 3);
        assert_eq!(casts[0][2..], [input, collector]);
        // Two profile records: the scope label and the end label.
        let records = e.mem.u32(stream + 4);
        assert_eq!(records, e.mem.u32(stream + 0xc) - 0x30 + 24);
        let first = records - 24;
        assert_eq!(e.mem.u32(first), CX_LIST_LABEL);
        assert_eq!(e.mem.u32(first + 12), END_LABEL);
    }

    #[test]
    fn hkp_convex_list_shape_cast_ray_impl_reports_no_hit() {
        let (mut e, this, collector, _, _) = shape_00d5e500([false, false, false]);
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 0xee);
        e.mem.set_u32(collector + 0x20 + 2 * 4, 0x1234);
        let input = e.mem.alloc(0x20);
        e.call(
            0x00d5_e500,
            &args![
                Ptr::<()>::new(this),
                Ptr::<()>::new(result),
                Ptr::<()>::new(input),
                Ptr::<()>::new(collector)
            ],
        );
        assert_eq!(e.mem.u8(result), 0);
        assert_eq!(e.mem.u32(collector + 0x20 + 2 * 4), 0x1234);
        assert_eq!(e.mem.u32(collector + 0x40), 2);
    }
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

    fn list_shape(e: &mut Engine, children: &[u32], capacity: u32) -> (u32, u32) {
        let array = e.mem.alloc(4 * children.len().max(1) as u32);
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *child);
        }
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x44, array);
        e.mem.set_u32(this + 0x48, children.len() as u32);
        e.mem.set_u32(this + 0x4c, capacity);
        (this, array)
    }

    #[test]
    fn the_content_statistics_report_the_array_the_children_and_the_base() {
        let (mut e, log, collector) = rig();
        let (this, array) = list_shape(&mut e, &[0x11, 0x22], 0x4000_0003);
        e.call(0x00d5_e480, &args![this, collector, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (GET_ALLOCATED_SIZE, vec![array, 12]),
                (
                    0x0900_0014,
                    vec![collector, 3, CHILD_PTRS_LABEL, array, 8, 0x777]
                ),
                (0x0900_000c, vec![collector, 0, 0x11, 0]),
                (0x0900_000c, vec![collector, 0, 0x22, 0]),
                (
                    BASE_CALC_CONTENT_STATISTICS,
                    vec![this, collector, CLASS_RECORD_01268940]
                ),
            ]
        );
    }

    #[test]
    fn the_content_statistics_skip_the_array_report_for_borrowed_memory() {
        let (mut e, log, collector) = rig();
        // A capacity word with the sign bit set: memory not owned.
        let (this, array) = list_shape(&mut e, &[0x33], 0x8000_0001);
        e.call(0x00d5_e480, &args![this, collector, 0u32]);
        let kinds: Vec<u32> = log.borrow().iter().map(|(address, _)| *address).collect();
        assert_eq!(kinds, vec![0x0900_000c, BASE_CALC_CONTENT_STATISTICS],);
        log.borrow_mut().clear();
        // The collector's flag byte asks for it anyway; the size passed is the masked one.
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00d5_e480, &args![this, collector, 0u32]);
        assert_eq!(log.borrow()[0], (GET_ALLOCATED_SIZE, vec![array, 4]));
        assert_eq!(log.borrow().len(), 4);
    }

    #[test]
    fn the_content_statistics_of_an_empty_list_report_only_the_array_and_the_base() {
        let (mut e, log, collector) = rig();
        let (this, _) = list_shape(&mut e, &[], 0);
        e.call(0x00d5_e480, &args![this, collector, 0u32]);
        let kinds: Vec<u32> = log.borrow().iter().map(|(address, _)| *address).collect();
        assert_eq!(
            kinds,
            vec![
                GET_ALLOCATED_SIZE,
                0x0900_0014,
                BASE_CALC_CONTENT_STATISTICS
            ]
        );
    }
}
