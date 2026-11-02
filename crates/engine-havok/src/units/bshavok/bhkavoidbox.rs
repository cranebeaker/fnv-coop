//! `BSHavok/bhkavoidbox.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpPhantom` (Xbox PDB) as far as the phantom listener loops read it (PC offsets).
    pub struct HkpPhantom: 0x100 {
        /// `m_overlapListeners` (Xbox PDB): `hkArray` data pointer.
        0x8c m_overlapListeners: Ptr,
        /// Size of `m_overlapListeners`.
        0x90 m_overlapListenersSize: i32,
    }
}

/// The listener at `index` of the phantom's overlap-listener array (data read afresh, as the
/// game's loop does after every callback).
fn listener_at(e: &Engine, this: Ptr<HkpPhantom>, index: i32) -> u32 {
    let data = e.get(this, HkpPhantom::m_overlapListeners).addr();
    e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)))
}

// Translated from 00ca94c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantom::fireCollidableAdded` (Xbox PDB): builds the event on its stack (the phantom,
/// `collidable`, a result word that starts at 0) and passes its address to slot 0 of every
/// non-null overlap listener, last one first. Returns the result word as the listeners left it.
pub fn hkp_phantom_fire_collidable_added(
    e: &mut Engine,
    this: Ptr<HkpPhantom>,
    collidable: u32,
) -> u32 {
    e.with_stack(12, |e, event| {
        e.mem.set_u32(event.addr(), this.addr());
        e.mem.set_u32(event.addr() + 4, collidable);
        e.mem.set_u32(event.addr() + 8, 0);
        let mut index = e.get(this, HkpPhantom::m_overlapListenersSize);
        loop {
            index = index.wrapping_sub(1);
            if index < 0 {
                break;
            }
            let listener = listener_at(e, this, index);
            if listener != 0 {
                e.vcall(listener, 0, &args![event]);
            }
        }
        e.mem.u32(event.addr() + 8)
    })
}

// Translated from 00ca9520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantom::fireCollidableRemoved` (Xbox PDB): builds the event on its stack (the phantom,
/// `collidable`, the byte `flag`) and passes its address to slot 4 of every non-null overlap
/// listener, last one first.
pub fn hkp_phantom_fire_collidable_removed(
    e: &mut Engine,
    this: Ptr<HkpPhantom>,
    collidable: u32,
    flag: u8,
) {
    e.with_stack(12, |e, event| {
        e.mem.set_u32(event.addr(), this.addr());
        e.mem.set_u32(event.addr() + 4, collidable);
        e.mem.set_u8(event.addr() + 8, flag);
        let mut index = e.get(this, HkpPhantom::m_overlapListenersSize);
        loop {
            index = index.wrapping_sub(1);
            if index < 0 {
                break;
            }
            let listener = listener_at(e, this, index);
            if listener != 0 {
                e.vcall(listener, 4, &args![event]);
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ca94c0,
            hkp_phantom_fire_collidable_added(Ptr<HkpPhantom>, u32) -> u32
        ),
        entry!(
            0x00ca9520,
            hkp_phantom_fire_collidable_removed(Ptr<HkpPhantom>, u32, u8)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const LISTENER_ADDED: u32 = 0x0900_0001;
    const LISTENER_REMOVED: u32 = 0x0900_0002;

    type EventLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A phantom with three listener slots (the middle one null); every listener's vtable
    /// slots 0 and 4 are doubles that log `(slot, listener, event words)`; the slot 0 double
    /// leaves `result` in the event's result word.
    fn phantom(e: &mut Engine, result: u32) -> (Ptr<HkpPhantom>, [u32; 3], EventLog) {
        let log: EventLog = Rc::new(RefCell::new(vec![]));
        let sink = log.clone();
        e.register_double(LISTENER_ADDED, move |e, a| {
            sink.borrow_mut()
                .push(vec![0, a[0], e.mem.u32(a[1]), e.mem.u32(a[1] + 4)]);
            e.mem.set_u32(a[1] + 8, result);
            Ret::default()
        });
        let sink = log.clone();
        e.register_double(LISTENER_REMOVED, move |e, a| {
            sink.borrow_mut().push(vec![
                4,
                a[0],
                e.mem.u32(a[1]),
                e.mem.u32(a[1] + 4),
                u32::from(e.mem.u8(a[1] + 8)),
            ]);
            Ret::default()
        });
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable, LISTENER_ADDED);
        e.mem.set_u32(vtable + 4, LISTENER_REMOVED);
        let listeners = [e.mem.alloc(4), 0, e.mem.alloc(4)];
        for listener in listeners.iter().filter(|l| **l != 0) {
            e.mem.set_u32(*listener, vtable);
        }
        let data = e.mem.alloc(12);
        for (i, listener) in listeners.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *listener);
        }
        let this: Ptr<HkpPhantom> = Ptr::new(e.mem.alloc(0x100));
        e.set(this, HkpPhantom::m_overlapListeners, Ptr::new(data));
        e.set(this, HkpPhantom::m_overlapListenersSize, 3);
        (this, listeners, log)
    }

    #[test]
    fn collidable_added_calls_the_non_null_listeners_last_first() {
        let mut e = Engine::new();
        let (this, listeners, log) = phantom(&mut e, 0);
        let result = e.call(0x00ca94c0, &args![this, 0x1234u32]).u32();
        assert_eq!(result, 0);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![0, listeners[2], this.addr(), 0x1234],
                vec![0, listeners[0], this.addr(), 0x1234],
            ]
        );
    }

    #[test]
    fn collidable_added_returns_the_word_the_listener_set() {
        let mut e = Engine::new();
        let (this, _, _) = phantom(&mut e, 7);
        assert_eq!(e.call(0x00ca94c0, &args![this, 1u32]).u32(), 7);
    }

    #[test]
    fn collidable_added_without_listeners_returns_zero() {
        let mut e = Engine::new();
        let (this, _, log) = phantom(&mut e, 7);
        e.set(this, HkpPhantom::m_overlapListenersSize, 0);
        assert_eq!(e.call(0x00ca94c0, &args![this, 1u32]).u32(), 0);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn collidable_removed_passes_the_flag_byte_to_slot_four() {
        let mut e = Engine::new();
        let (this, listeners, log) = phantom(&mut e, 0);
        e.call(0x00ca9520, &args![this, 0x55u32, 1u8]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![4, listeners[2], this.addr(), 0x55, 1],
                vec![4, listeners[0], this.addr(), 0x55, 1],
            ]
        );
    }
}
