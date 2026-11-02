//! `Havok SDK/hkpconstraintcallbackutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The three callback walkers go through the listener array of an `hkpConstraintInstance`
//! (`m_listeners`, an `hkSmallArray` at +0x20: data pointer, 16-bit size at +4) from the last
//! entry to the first, bracket each listener call in two timer records (the profiling stream of
//! the thread, `TlsGetValue` of the index at `0126835c`; the `RDTSC` time stamp comes from the
//! host clock), and finish by dropping the null entries.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue` import slot and the TLS index of the profiling stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const PROFILE_SLOT: u32 = 0x0126_835c;
/// The label of the record that ends a timed section.
const LABEL_END: u32 = 0x010c_41f8;
/// The labels `"TtconAddCb"`, `"TtconRemCb"` and `"TtconDeletedCb"`.
const LABEL_ADD: u32 = 0x010c_ca98;
const LABEL_REMOVE: u32 = 0x010c_caa4;
const LABEL_DELETED: u32 = 0x010d_3824;

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the host
/// clock stands in (only the profiling records take it).
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte profiling record `(label, time stamp)` to the thread's stream while its
/// cursor (+4) is below its limit (+0xC).
fn profile_record(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0xc);
    }
}

// Translated from 00d85d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cleanupNullPointers_ov2` (Xbox PDB), called with the array in `EAX` (`array` here, first):
/// an `hkSmallArray` of pointers (data pointer at +0, 16-bit size at +4). Walking from the last
/// entry to the first, every null entry is removed: the size drops by one and the entries
/// behind it move down one place.
pub fn cleanup_null_pointers_ov2(e: &mut Engine, array: Ptr) {
    let array = array.addr();
    let mut index = i32::from(e.mem.u16(array + 4)) - 1;
    while index >= 0 {
        let data = e.mem.u32(array);
        if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) == 0 {
            let size = e.mem.u16(array + 4).wrapping_add(0xffff);
            e.mem.set_u16(array + 4, size);
            if index < i32::from(size) {
                let mut at = index;
                loop {
                    let data = e.mem.u32(array);
                    let next = e
                        .mem
                        .u32(data.wrapping_add((at as u32).wrapping_mul(4)) + 4);
                    e.mem
                        .set_u32(data.wrapping_add((at as u32).wrapping_mul(4)), next);
                    at += 1;
                    if at >= i32::from(e.mem.u16(array + 4)) {
                        break;
                    }
                }
            }
        }
        index -= 1;
    }
}

/// The body the three walkers share: every non-null listener, last to first, gets its virtual
/// function at `slot` called with `instance`, between a record labelled `label` and a record
/// labelled `LABEL_END`; then the null entries are removed.
fn fire_listeners(e: &mut Engine, instance: Ptr, label: u32, slot: u32) {
    let array = instance.addr() + 0x20;
    let mut index = i32::from(e.mem.u16(array + 4)) - 1;
    while index >= 0 {
        let data = e.mem.u32(array);
        if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) != 0 {
            profile_record(e, label);
            let data = e.mem.u32(array);
            let listener = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            e.vcall(listener, slot, &args![instance]);
            profile_record(e, LABEL_END);
        }
        index -= 1;
    }
    cleanup_null_pointers_ov2(e, Ptr::new(array));
}

// Translated from 00d85d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (`hkpConstraintCallbackUtil`, by its unit; the walker labelled
/// `"TtconAddCb"`), cdecl `(instance)`: calls the listeners' virtual function at `+4` with the
/// instance, as described above.
pub fn fn_00d85d90(e: &mut Engine, instance: Ptr) {
    fire_listeners(e, instance, LABEL_ADD, 4);
}

// Translated from 00d85e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the walker labelled `"TtconRemCb"`), cdecl `(instance)`: calls the
/// listeners' virtual function at `+8` with the instance.
pub fn fn_00d85e40(e: &mut Engine, instance: Ptr) {
    fire_listeners(e, instance, LABEL_REMOVE, 8);
}

// Translated from 00d85ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintCallbackUtil::fireConstraintDeleted` (Xbox PDB), cdecl `(instance)`: calls the
/// listeners' virtual function at `+0xc` (labelled `"TtconDeletedCb"`).
pub fn hkp_constraint_callback_util_fire_constraint_deleted(e: &mut Engine, instance: Ptr) {
    fire_listeners(e, instance, LABEL_DELETED, 0xc);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d8_5d40, cleanup_null_pointers_ov2(Ptr)),
        entry!(0x00d8_5d90, fn_00d85d90(Ptr)),
        entry!(0x00d8_5e40, fn_00d85e40(Ptr)),
        entry!(
            0x00d8_5ef0,
            hkp_constraint_callback_util_fire_constraint_deleted(Ptr)
        ),
    ]
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// A small array of pointers: data block and a header with the size at +4.
    fn array_of(e: &mut Engine, entries: &[u32]) -> u32 {
        let header = e.mem.alloc(8);
        let data = e.mem.alloc(4 * entries.len() as u32 + 4);
        for (index, entry) in entries.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *entry);
        }
        e.mem.set_u32(header, data);
        e.mem.set_u16(header + 4, entries.len() as u16);
        header
    }

    fn entries_of(e: &Engine, header: u32) -> Vec<u32> {
        let data = e.mem.u32(header);
        (0..u32::from(e.mem.u16(header + 4)))
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    #[test]
    fn null_entries_are_removed_and_the_rest_keeps_its_order() {
        let mut e = engine();
        let header = array_of(&mut e, &[0xa, 0, 0xb, 0, 0, 0xc, 0]);
        e.call(0x00d8_5d40, &args![header]);
        assert_eq!(entries_of(&e, header), [0xa, 0xb, 0xc]);
        let empty = array_of(&mut e, &[0, 0]);
        e.call(0x00d8_5d40, &args![empty]);
        assert_eq!(entries_of(&e, empty), Vec::<u32>::new());
        let none = array_of(&mut e, &[]);
        e.call(0x00d8_5d40, &args![none]);
        assert_eq!(e.mem.u16(none + 4), 0);
        let full = array_of(&mut e, &[1, 2, 3]);
        e.call(0x00d8_5d40, &args![full]);
        assert_eq!(entries_of(&e, full), [1, 2, 3]);
    }

    /// Listener objects whose three virtual functions log `(listener, slot, instance)`; a
    /// profiling stream with room for `room` records.
    fn listener_setup(e: &mut Engine, room: u32) -> (Rc<RefCell<Vec<(u32, u32, u32)>>>, u32, u32) {
        let log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        for (slot, double) in [(4u32, 0x0900_0004u32), (8, 0x0900_0008), (0xc, 0x0900_000c)] {
            e.mem.set_u32(vtable + slot, double);
            let sink = Rc::clone(&log);
            e.register_double(double, move |_, a| {
                sink.borrow_mut().push((a[0], slot, a[1]));
                Ret::default()
            });
        }
        e.set_global(0x0126_835c, 5u32);
        let stream = e.mem.alloc(0x10);
        let records = e.mem.alloc(12 * room.max(1));
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 12 * room);
        e.register_double(0x00fd_f044, move |_, a| {
            assert_eq!(a[0], 5);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        (log, vtable, stream)
    }

    fn listener(e: &mut Engine, vtable: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn listeners_are_called_last_to_first_between_timer_records() {
        let mut e = engine();
        let (log, vtable, stream) = listener_setup(&mut e, 16);
        let first = listener(&mut e, vtable);
        let last = listener(&mut e, vtable);
        let instance = e.mem.alloc(0x40);
        let header = array_of(&mut e, &[first, 0, last]);
        e.mem.set_u32(instance + 0x20, e.mem.u32(header));
        e.mem.set_u16(instance + 0x24, 3);
        for (address, slot, label) in [
            (0x00d8_5d90, 4, 0x010c_ca98),
            (0x00d8_5e40, 8, 0x010c_caa4),
            (0x00d8_5ef0, 0xc, 0x010d_3824),
        ] {
            log.borrow_mut().clear();
            let records = e.mem.u32(stream + 4);
            e.call(address, &args![instance]);
            assert_eq!(
                *log.borrow(),
                [(last, slot, instance), (first, slot, instance)]
            );
            // Four records: label, end, label, end; the null entry is gone.
            assert_eq!(e.mem.u32(stream + 4), records + 4 * 12);
            for (index, expected) in [label, 0x010c_41f8, label, 0x010c_41f8]
                .into_iter()
                .enumerate()
            {
                assert_eq!(e.mem.u32(records + 12 * index as u32), expected);
            }
            assert_eq!(e.mem.u16(instance + 0x24), 2);
            assert_eq!(entries_of(&e, instance + 0x20), [first, last]);
            // Restore the layout for the next walker.
            let data = e.mem.u32(instance + 0x20);
            e.mem.set_u32(data, first);
            e.mem.set_u32(data + 4, 0);
            e.mem.set_u32(data + 8, last);
            e.mem.set_u16(instance + 0x24, 3);
        }
    }

    #[test]
    fn records_stop_when_the_stream_is_full() {
        let mut e = engine();
        let (log, vtable, stream) = listener_setup(&mut e, 1);
        let only = listener(&mut e, vtable);
        let instance = e.mem.alloc(0x40);
        let header = array_of(&mut e, &[only]);
        e.mem.set_u32(instance + 0x20, e.mem.u32(header));
        e.mem.set_u16(instance + 0x24, 1);
        let records = e.mem.u32(stream + 4);
        e.call(0x00d8_5ef0, &args![instance]);
        assert_eq!(*log.borrow(), [(only, 0xc, instance)]);
        // Room for one record: the opening label fits, the end label does not.
        assert_eq!(e.mem.u32(stream + 4), records + 12);
        assert_eq!(e.mem.u32(records), 0x010d_3824);
    }
}
