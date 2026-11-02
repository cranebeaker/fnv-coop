//! `NiMain/nipropertystate.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiPropertyState` (Xbox PDB, 0x1c bytes): `m_aspProps`, seven `NiPointer<NiProperty>`
//! at `+0x00..+0x18`. A held object's reference count is at `+4` and its virtual slot 1
//! (`+4`, `DeleteThis`) runs when the count drops to zero.

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedIncrement` / `InterlockedDecrement` import slots.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// Number of property pointers (`m_aspProps`).
const PROPERTY_COUNT: u32 = 7;

/// The default property objects `Reset` puts in the first six slots, in slot order: the
/// pointers the `_SDMInit`s of `nialphaproperty.cpp` (`011f4410`), `nicullingproperty.cpp`
/// (`011f5d28`), `nimaterialproperty.cpp` (`011f4a64`), the shade property's unit
/// (`011f5ae8`), `nistencilproperty.cpp` (`011f4a14`, written by `00a6f450`/`00a6f4d0`) and
/// `nitexturingproperty.cpp` (`011f49ac`) store their objects in.
const DEFAULT_PROPERTIES: [u32; 6] = [
    0x011f_4410,
    0x011f_5d28,
    0x011f_4a64,
    0x011f_5ae8,
    0x011f_4a14,
    0x011f_49ac,
];

/// `InterlockedDecrement` of the count at `object + 4`; `DeleteThis` (slot `+4`) at zero.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).i32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

// Translated from 00a89ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPropertyState::NiPropertyState` (Xbox PDB): clears the seven pointers, then `Reset`
/// (`00a89960`). Answers `this`.
pub fn ni_property_state_ni_property_state(e: &mut Engine, this: Ptr) -> Ptr {
    for slot in 0..PROPERTY_COUNT {
        e.mem.set_u32(this.addr() + 4 * slot, 0);
    }
    ni_property_state_reset(e, this);
    this
}

// Translated from 00a89960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPropertyState::Reset` (Xbox PDB): each of the first six slots gets the default
/// property its global holds (`DEFAULT_PROPERTIES`; the global is read before the slot
/// changes): nothing when the slot holds it already, otherwise the old object is released,
/// the default stored and counted. The seventh slot is left as it is.
pub fn ni_property_state_reset(e: &mut Engine, this: Ptr) {
    for (slot, global) in DEFAULT_PROPERTIES.iter().enumerate() {
        let value = e.mem.u32(*global);
        let holder = this.addr() + 4 * slot as u32;
        let old = e.mem.u32(holder);
        if old != value {
            if old != 0 {
                release(e, old);
            }
            e.mem.set_u32(holder, value);
            if value != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![value + 4]);
            }
        }
    }
}

// Translated from 00a89900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nipropertystate.cpp`; the body is that of
/// `NiPropertyState::operator=`, `thiscall`, `RET 4`): for each of the seven slots in turn,
/// when the pointer of `other` differs from the one of `this`, releases the old object,
/// stores the one of `other` (read again after the release) and counts it. Answers `this`.
pub fn fn_00a89900(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let delta = other.addr().wrapping_sub(this.addr());
    for slot in 0..PROPERTY_COUNT {
        let holder = this.addr() + 4 * slot;
        let old = e.mem.u32(holder);
        if old != e.mem.u32(holder.wrapping_add(delta)) {
            if old != 0 {
                release(e, old);
            }
            let value = e.mem.u32(holder.wrapping_add(delta));
            e.mem.set_u32(holder, value);
            if value != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![value + 4]);
            }
        }
    }
    this
}

// Translated from 00a89890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPropertyState::~NiPropertyState` (Xbox PDB): releases and clears each of the seven
/// pointers from the first to the last, then (the member array's own destructor) walks the
/// slots from the last to the first and releases each one that is not null (none is after
/// the first pass, unless a `DeleteThis` stored one).
pub fn ni_property_state_destructor(e: &mut Engine, this: Ptr) {
    for slot in 0..PROPERTY_COUNT {
        let holder = this.addr() + 4 * slot;
        let object = e.mem.u32(holder);
        if object != 0 {
            release(e, object);
            e.mem.set_u32(holder, 0);
        }
    }
    for slot in (0..PROPERTY_COUNT).rev() {
        let object = e.mem.u32(this.addr() + 4 * slot);
        if object != 0 {
            release(e, object);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_9ab0, ni_property_state_ni_property_state(Ptr) -> Ptr),
        entry!(0x00a8_9960, ni_property_state_reset(Ptr)),
        entry!(0x00a8_9900, fn_00a89900(Ptr, Ptr) -> Ptr),
        entry!(0x00a8_9890, ni_property_state_destructor(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const VTABLE: u32 = 0x0109_0000;
    const DELETE_THIS: u32 = 0x0060_0000;

    /// An engine with counting interlocked imports and a vtable whose `DeleteThis` logs.
    fn fixture() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x2000);
        e.map(VTABLE, 0x1000);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.put_vtable(VTABLE, &[0, DELETE_THIS]);
        let deleted = Rc::new(RefCell::new(Vec::new()));
        let log = deleted.clone();
        e.register_double(DELETE_THIS, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        (e, deleted)
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let at = e.mem.alloc(8);
        e.mem.set_u32(at, VTABLE);
        e.mem.set_u32(at + 4, count);
        at
    }

    fn set_defaults(e: &mut Engine) -> Vec<u32> {
        let defaults: Vec<u32> = (0..6).map(|_| object(e, 1)).collect();
        for (global, value) in DEFAULT_PROPERTIES.iter().zip(&defaults) {
            e.mem.set_u32(*global, *value);
        }
        defaults
    }

    #[test]
    fn construction_puts_the_six_defaults_counted() {
        let (mut e, deleted) = fixture();
        let defaults = set_defaults(&mut e);
        let this = e.mem.alloc(0x1c);
        e.mem.write(this, &[0xcc; 0x1c]);
        assert_eq!(e.call(0x00a8_9ab0, &args![this]).u32(), this);
        for (slot, value) in defaults.iter().enumerate() {
            assert_eq!(e.mem.u32(this + 4 * slot as u32), *value);
            assert_eq!(e.mem.u32(value + 4), 2);
        }
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert!(deleted.borrow().is_empty());
    }

    #[test]
    fn reset_releases_old_objects_and_skips_slots_already_default() {
        let (mut e, deleted) = fixture();
        let defaults = set_defaults(&mut e);
        // Slot 0 holds its default; slots 1 and 3 an object about to die; slot 2 a shared
        // one; the global of slot 3 is null.
        e.mem.set_u32(DEFAULT_PROPERTIES[3], 0);
        let dying = object(&mut e, 2);
        let shared = object(&mut e, 2);
        let seventh = object(&mut e, 1);
        let this = e.mem.alloc(0x1c);
        let start = [defaults[0], dying, shared, dying, 0, 0, seventh];
        for (slot, value) in start.iter().enumerate() {
            e.mem.set_u32(this + 4 * slot as u32, *value);
        }
        e.call(0x00a8_9960, &args![this]);
        assert_eq!(e.mem.u32(defaults[0] + 4), 1);
        assert_eq!(e.mem.u32(this + 4), defaults[1]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), defaults[4]);
        assert_eq!(e.mem.u32(shared + 4), 1);
        assert_eq!(e.mem.u32(this + 0x18), seventh);
        assert_eq!(*deleted.borrow(), vec![dying]);
    }

    #[test]
    fn assignment_copies_the_seven_slots() {
        let (mut e, deleted) = fixture();
        let a = object(&mut e, 1);
        let b = object(&mut e, 1);
        let same = object(&mut e, 2);
        let this = e.mem.alloc(0x1c);
        let other = e.mem.alloc(0x1c);
        e.mem.write(this, &[0; 0x1c]);
        e.mem.write(other, &[0; 0x1c]);
        e.mem.set_u32(this, a);
        e.mem.set_u32(this + 8, same);
        e.mem.set_u32(other, b);
        e.mem.set_u32(other + 8, same);
        e.mem.set_u32(other + 0x18, b);
        assert_eq!(e.call(0x00a8_9900, &args![this, other]).u32(), this);
        assert_eq!(e.mem.u32(this), b);
        assert_eq!(e.mem.u32(this + 0x18), b);
        assert_eq!(e.mem.u32(b + 4), 3);
        assert_eq!(e.mem.u32(same + 4), 2);
        assert_eq!(*deleted.borrow(), vec![a]);
    }

    #[test]
    fn destruction_releases_and_clears_every_slot() {
        let (mut e, deleted) = fixture();
        let a = object(&mut e, 1);
        let b = object(&mut e, 2);
        let this = e.mem.alloc(0x1c);
        e.mem.write(this, &[0; 0x1c]);
        e.mem.set_u32(this + 4, a);
        e.mem.set_u32(this + 0x18, b);
        e.call(0x00a8_9890, &args![this]);
        for slot in 0..7 {
            assert_eq!(e.mem.u32(this + 4 * slot), 0);
        }
        assert_eq!(e.mem.u32(b + 4), 1);
        assert_eq!(*deleted.borrow(), vec![a]);
    }
}
