//! `Havok SDK/run_00c983e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `EnterCriticalSection` and `LeaveCriticalSection` import slots.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

/// Calls virtual method `slot` of every listener of a listener list with the five words it was
/// given, inside the list's critical section. The list (`this`) holds the array of listener
/// pointers at `+8` (data) and `+0xc` (count) and a pointer to its critical section at `+0x14`;
/// the section pointer is read again to leave it, and the count and the data pointer are read
/// again for every listener, as the exe does.
fn notify_listeners(e: &mut Engine, this: Ptr, slot: u32, words: [u32; 5]) {
    let base = this.addr();
    let section = e.mem.u32(base + 0x14);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let mut index = 0;
    if e.mem.i32(base + 0xc) > 0 {
        loop {
            let listener = e.mem.u32(e.mem.u32(base + 8).wrapping_add(index * 4));
            e.vcall(listener, slot, &words);
            index += 1;
            if index as i32 >= e.mem.i32(base + 0xc) {
                break;
            }
        }
    }
    let section = e.mem.u32(base + 0x14);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00c983e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls virtual method `+4` of every listener of the list `this` with the five arguments
/// (see [`notify_listeners`]); the class is unconfirmed.
pub fn fn_00c983e0(
    e: &mut Engine,
    this: Ptr,
    arg_0: u32,
    arg_1: u32,
    arg_2: u32,
    arg_3: u32,
    arg_4: u32,
) {
    notify_listeners(e, this, 4, [arg_0, arg_1, arg_2, arg_3, arg_4]);
}

// Translated from 00c98440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls virtual method `+8` of every listener of the list `this` with the five arguments
/// (see [`notify_listeners`]); the class is unconfirmed.
pub fn fn_00c98440(
    e: &mut Engine,
    this: Ptr,
    arg_0: u32,
    arg_1: u32,
    arg_2: u32,
    arg_3: u32,
    arg_4: u32,
) {
    notify_listeners(e, this, 8, [arg_0, arg_1, arg_2, arg_3, arg_4]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_83e0, fn_00c983e0(Ptr, u32, u32, u32, u32, u32)),
        entry!(0x00c9_8440, fn_00c98440(Ptr, u32, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const SLOT_4_DOUBLE: u32 = 0x0900_0041;
    const SLOT_8_DOUBLE: u32 = 0x0900_0081;

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    /// A list of three listeners with a vtable whose slots 4 and 8 log their words, and the
    /// critical-section imports as doubles.
    fn rig(count: u32) -> (Engine, Log, u32, Vec<u32>) {
        let mut e = Engine::new();
        e.map(0x00fd_f000, 0x1000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(ENTER_CRITICAL_SECTION, move |_, a| {
            sink.borrow_mut().push(("enter", a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(LEAVE_CRITICAL_SECTION, move |_, a| {
            sink.borrow_mut().push(("leave", a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SLOT_4_DOUBLE, move |_, a| {
            sink.borrow_mut().push(("slot4", a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SLOT_8_DOUBLE, move |_, a| {
            sink.borrow_mut().push(("slot8", a.to_vec()));
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, SLOT_4_DOUBLE);
        e.mem.set_u32(vtable + 8, SLOT_8_DOUBLE);
        let listeners: Vec<u32> = (0..3)
            .map(|_| {
                let listener = e.mem.alloc(0x10);
                e.mem.set_u32(listener, vtable);
                listener
            })
            .collect();
        let data = e.mem.alloc(0x10);
        for (i, listener) in listeners.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *listener);
        }
        let list = e.mem.alloc(0x20);
        e.mem.set_u32(list + 8, data);
        e.mem.set_u32(list + 0xc, count);
        e.mem.set_u32(list + 0x14, 0x0222_2000);
        (e, log, list, listeners)
    }

    /// Every listener gets the five words in order through the slot, between entering and
    /// leaving the section; an empty list only enters and leaves.
    #[test]
    fn listeners_are_notified_inside_the_section() {
        for (address, slot) in [(0x00c9_83e0, "slot4"), (0x00c9_8440, "slot8")] {
            let (mut e, log, list, listeners) = rig(3);
            e.call(address, &args![list, 1u32, 2u32, 3u32, 4u32, 5u32]);
            let mut expected = vec![("enter", vec![0x0222_2000])];
            for listener in &listeners {
                expected.push((slot, vec![*listener, 1, 2, 3, 4, 5]));
            }
            expected.push(("leave", vec![0x0222_2000]));
            assert_eq!(*log.borrow(), expected);
            let (mut e, log, list, _) = rig(0);
            e.call(address, &args![list, 1u32, 2u32, 3u32, 4u32, 5u32]);
            assert_eq!(
                *log.borrow(),
                vec![("enter", vec![0x0222_2000]), ("leave", vec![0x0222_2000])]
            );
        }
    }
}
