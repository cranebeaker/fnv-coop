//! `Havok SDK/run_00d85fa0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue` import slot and the TLS index of the thread's profiling stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const PROFILE_SLOT: u32 = 0x0126_835c;
/// The label of the record that ends a timed section.
const LABEL_END: u32 = 0x010c_41f8;
/// The labels `"TtconBrokenCb"` and `"TtconRepairedCb"` (the leading `"Tt"` marks a timer
/// record).
const LABEL_BROKEN: u32 = 0x010c_cabc;
const LABEL_REPAIRED: u32 = 0x010c_cacc;
/// `cleanupNullPointers_ov2` (`00d85d40`, Xbox PDB name): the small array of pointers in `EAX`.
const CLEANUP_NULL_POINTERS: u32 = 0x00d8_5d40;

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the host
/// clock stands in (only the profiling records take it).
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte profiling record `(label, time stamp)` to the thread's stream while its
/// cursor (`+4`) is below its limit (`+0xc`, unsigned compare).
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

/// The walk the two `fireConstraint...` functions share: for the instance's listener array
/// (an `hkSmallArray`, data pointer at `+0`, 16-bit size at `+4`, at `[instance + 4] + 0x20`),
/// from the last listener to the first and skipping null entries, a timer record with
/// `label`, the listener's virtual slot `slot` with `instance`, and a timer record with the end
/// label; then the null entries are dropped (`00d85d40`, which gets the array).
fn fire_listeners(e: &mut Engine, instance: u32, label: u32, slot: u32) {
    let array = e.mem.u32(instance + 4).wrapping_add(0x20);
    let mut index = u32::from(e.mem.u16(array + 4)).wrapping_sub(1) as i32;
    while index >= 0 {
        let data = e.mem.u32(array);
        if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) != 0 {
            profile_record(e, label);
            let data = e.mem.u32(array);
            let listener = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            e.vcall(listener, slot, &args![instance]);
            profile_record(e, LABEL_END);
        }
        index = index.wrapping_sub(1);
    }
    e.call(CLEANUP_NULL_POINTERS, &args![array]);
}

// Translated from 00d85fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintCallbackUtil::fireConstraintBroken` (Xbox PDB; the exe's timer label is
/// `"TtconBrokenCb"`): cdecl `(instance)`. Calls virtual slot `0x14` of each non-null listener
/// of `[instance + 4] + 0x20` with `instance`, from the last to the first, each call between two
/// timer records, then ends with `00d85d40` on the array (a tail jump: its result is this
/// function's).
pub fn fn_00d85fa0(e: &mut Engine, instance: u32) {
    fire_listeners(e, instance, LABEL_BROKEN, 0x14);
}

// Translated from 00d86050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintCallbackUtil::fireConstraintRepaired` (Xbox PDB; the exe's timer label is
/// `"TtconRepairedCb"`): as `00d85fa0` with virtual slot `0x18`.
pub fn fn_00d86050(e: &mut Engine, instance: u32) {
    fire_listeners(e, instance, LABEL_REPAIRED, 0x18);
}

/// Writes (or reads) the two 4-byte fields at `+0xc` and `+0x10` of `data` through the
/// function pointer at `callback_offset` of the stream object at `[context + stream_offset]`:
/// cdecl `(stream, field, 4, &count, 1)` with `count` a stack word that holds 4 before each
/// call (the exe uses its own argument slot), and `result` added to the stream's total at
/// `+4`. Returns the second call's result (`EAX`).
fn transfer_two_fields(
    e: &mut Engine,
    context: u32,
    data: u32,
    stream_offset: u32,
    callback_offset: u32,
) -> u32 {
    e.with_stack(4, |e, count| {
        let count = count.addr();
        let mut result = 0;
        for field in [0xcu32, 0x10] {
            let stream = e.mem.u32(context.wrapping_add(stream_offset));
            let callback = e.mem.u32(stream.wrapping_add(callback_offset));
            e.mem.set_u32(count, 4);
            result = e
                .call(
                    callback,
                    &args![stream, data.wrapping_add(field), 4u32, count, 1u32],
                )
                .u32();
            let total = e.mem.u32(stream.wrapping_add(4));
            e.mem
                .set_u32(stream.wrapping_add(4), total.wrapping_add(result));
        }
        result
    })
}

// Translated from 00d86100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(context, data)`: hands the 4-byte fields at `data + 0xc` and `data + 0x10` to the
/// function at `+8` of the stream object `[context + 0x24c]` (one call each, the stream read
/// again for the second), adding each result to the stream's counter at `+4`. Returns the
/// second call's result (`EAX` on return). The class is unconfirmed.
pub fn fn_00d86100(e: &mut Engine, context: u32, data: u32) -> u32 {
    transfer_two_fields(e, context, data, 0x24c, 8)
}

// Translated from 00d86160 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00d86100` with the stream object `[context + 0x250]` and the function at `+0xc`.
pub fn fn_00d86160(e: &mut Engine, context: u32, data: u32) -> u32 {
    transfer_two_fields(e, context, data, 0x250, 0xc)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d8_5fa0, fn_00d85fa0(u32)),
        entry!(0x00d8_6050, fn_00d86050(u32)),
        entry!(0x00d8_6100, fn_00d86100(u32, u32) -> u32),
        entry!(0x00d8_6160, fn_00d86160(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FIRST_SLOT: u32 = 0x0900_0001;
    const SECOND_SLOT: u32 = 0x0900_0002;
    const STREAM_WRITE: u32 = 0x0900_0010;
    const STREAM_READ: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(PROFILE_SLOT, 5u32);
        e
    }

    /// A profiling stream with room for `records` 12-byte records; the double of
    /// `TlsGetValue` hands it out for slot 5.
    fn install_stream(e: &mut Engine, records: u32) -> u32 {
        let stream = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(12 * records.max(1));
        e.mem.set_u32(stream + 4, buffer);
        e.mem.set_u32(stream + 0xc, buffer + 12 * records);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 5);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        stream
    }

    /// An instance whose `+4` object holds the listener array at `+0x20`; the listeners are
    /// objects whose vtable slots `0x14` and `0x18` are the doubles. A zero in `listeners`
    /// stays a null entry.
    fn instance(e: &mut Engine, listeners: &[u32]) -> (u32, u32) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x14, FIRST_SLOT);
        e.mem.set_u32(vtable + 0x18, SECOND_SLOT);
        let data = e.mem.alloc(4 * listeners.len() as u32 + 4);
        for (i, id) in listeners.iter().enumerate() {
            if *id != 0 {
                let object = e.mem.alloc(8);
                e.mem.set_u32(object, vtable);
                e.mem.set_u32(object + 4, *id);
                e.mem.set_u32(data + 4 * i as u32, object);
            }
        }
        let holder = e.mem.alloc(0x40);
        e.mem.set_u32(holder + 0x20, data);
        e.mem.set_u16(holder + 0x24, listeners.len() as u16);
        let instance = e.mem.alloc(0x10);
        e.mem.set_u32(instance + 4, holder);
        (instance, holder + 0x20)
    }

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn record_listeners(e: &mut Engine, slots: &[u32]) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for slot in slots {
            let log = Rc::clone(&calls);
            let slot = *slot;
            e.register_double(slot, move |e, a| {
                log.borrow_mut()
                    .push((slot, vec![e.mem.u32(a[0] + 4), a[1]]));
                Ret::default()
            });
        }
        calls
    }

    fn cleanup_double(e: &mut Engine) -> Rc<RefCell<Vec<u32>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(CLEANUP_NULL_POINTERS, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        seen
    }

    #[test]
    fn broken_calls_slot_0x14_of_each_listener_from_the_last_with_timer_records() {
        let mut e = engine();
        let stream = install_stream(&mut e, 8);
        let cursor = e.mem.u32(stream + 4);
        let (instance, array) = instance(&mut e, &[1, 0, 3]);
        let calls = record_listeners(&mut e, &[FIRST_SLOT, SECOND_SLOT]);
        let cleaned = cleanup_double(&mut e);
        e.call(0x00d8_5fa0, &args![instance]);
        assert_eq!(
            *calls.borrow(),
            [
                (FIRST_SLOT, vec![3, instance]),
                (FIRST_SLOT, vec![1, instance])
            ]
        );
        assert_eq!(*cleaned.borrow(), [array]);
        // Two listeners: begin and end records each, 12 bytes apiece.
        assert_eq!(e.mem.u32(stream + 4), cursor + 48);
        assert_eq!(e.mem.u32(cursor), LABEL_BROKEN);
        assert_eq!(e.mem.u32(cursor + 12), LABEL_END);
        assert_eq!(e.mem.u32(cursor + 24), LABEL_BROKEN);
        assert_eq!(e.mem.u32(cursor + 36), LABEL_END);
    }

    #[test]
    fn repaired_calls_slot_0x18_and_a_full_stream_takes_no_records() {
        let mut e = engine();
        let stream = install_stream(&mut e, 0);
        let (instance, array) = instance(&mut e, &[7]);
        let calls = record_listeners(&mut e, &[FIRST_SLOT, SECOND_SLOT]);
        let cleaned = cleanup_double(&mut e);
        e.call(0x00d8_6050, &args![instance]);
        assert_eq!(*calls.borrow(), [(SECOND_SLOT, vec![7, instance])]);
        assert_eq!(*cleaned.borrow(), [array]);
        // The cursor equals the limit: nothing was written.
        assert_eq!(e.mem.u32(stream + 4), e.mem.u32(stream + 0xc));
    }

    #[test]
    fn an_empty_listener_array_only_cleans_up() {
        let mut e = engine();
        install_stream(&mut e, 2);
        let (instance, array) = instance(&mut e, &[]);
        let calls = record_listeners(&mut e, &[FIRST_SLOT]);
        let cleaned = cleanup_double(&mut e);
        e.call(0x00d8_5fa0, &args![instance]);
        assert!(calls.borrow().is_empty());
        assert_eq!(*cleaned.borrow(), [array]);
    }

    /// A context whose stream object at `offset` has the transfer function at `callback_offset`.
    fn stream_context(
        e: &mut Engine,
        offset: u32,
        callback_offset: u32,
        callback: u32,
    ) -> (u32, u32) {
        let stream = e.mem.alloc(0x20);
        e.mem.set_u32(stream + 4, 100);
        e.mem.set_u32(stream + callback_offset, callback);
        let context = e.mem.alloc(0x260);
        e.mem.set_u32(context + offset, stream);
        (context, stream)
    }

    fn transfer_double(e: &mut Engine, address: u32) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&calls);
        e.register_double(address, move |e, a| {
            // The count word holds 4 when the function is called.
            log.borrow_mut()
                .push((address, vec![a[0], a[1], a[2], e.mem.u32(a[3]), a[4]]));
            // The function answers the bytes it moved and leaves the count changed.
            e.mem.set_u32(a[3], 0x99);
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        calls
    }

    #[test]
    fn stream_transfers_hand_both_fields_to_the_streams_function() {
        let mut e = engine();
        let (context, stream) = stream_context(&mut e, 0x24c, 8, STREAM_WRITE);
        let calls = transfer_double(&mut e, STREAM_WRITE);
        let data = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d8_6100, &args![context, data]).u32(), 4);
        assert_eq!(
            *calls.borrow(),
            [
                (STREAM_WRITE, vec![stream, data + 0xc, 4, 4, 1]),
                (STREAM_WRITE, vec![stream, data + 0x10, 4, 4, 1]),
            ]
        );
        assert_eq!(e.mem.u32(stream + 4), 108);
    }

    #[test]
    fn the_second_transfer_uses_the_other_stream_slot() {
        let mut e = engine();
        let (context, stream) = stream_context(&mut e, 0x250, 0xc, STREAM_READ);
        let calls = transfer_double(&mut e, STREAM_READ);
        let data = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d8_6160, &args![context, data]).u32(), 4);
        assert_eq!(calls.borrow().len(), 2);
        assert_eq!(calls.borrow()[1].1[1], data + 0x10);
        assert_eq!(e.mem.u32(stream + 4), 108);
    }
}
