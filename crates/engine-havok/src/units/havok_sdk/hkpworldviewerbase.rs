//! `Havok SDK/hkpworldviewerbase.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator delete` (`00401030`, cdecl, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `hkpPhysicsContext`'s functions taking the listener at `this + 0x20`: `00cd2210` removes it,
/// `00cd2470` adds it (thiscall on the context, `RET 4`).
const CONTEXT_REMOVE_LISTENER: u32 = 0x00cd_2210;
const CONTEXT_ADD_LISTENER: u32 = 0x00cd_2470;
/// `hkReferencedObject`'s `removeReference` (`00c905b0`) and `addReference` (`00c90510`),
/// thiscall with no arguments.
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// `hkString::strCmp` (Xbox PDB, `00cb0fa0`, cdecl, two words) and the text `"Physics"`.
const STRING_COMPARE: u32 = 0x00cb_0fa0;
const PHYSICS_TEXT: u32 = 0x010c_b2c0;
/// Vtable pointers stored by these functions: the listener's own (`+0x20`), the ones the
/// finished constructor installs at +0, +8 and +0x20, and the ones the destructor ends with.
const LISTENER_VTABLE: u32 = 0x010d_7f04;
const VIEWER_VTABLE_PRIMARY: u32 = 0x010d_7f44;
const VIEWER_VTABLE_SECONDARY: u32 = 0x010d_7f24;
const VIEWER_VTABLE_LISTENER: u32 = 0x010d_7f14;
const BASE_VTABLE_SECONDARY: u32 = 0x010d_1508;
const BASE_VTABLE_PRIMARY: u32 = 0x0102_e388;

// Translated from 00ddcc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhysicsContextWorldListener::scalar deleting destructor` (Xbox PDB), thiscall, `RET 4`:
/// stores the listener's vtable pointer (`010d7f04`) at +0 and, when bit 0 of `flags` is
/// set, calls `operator delete` on `this`. Returns `this`.
pub fn hkp_physics_context_world_listener_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), LISTENER_VTABLE);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00ddcc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the world viewer base (it is the destructor the memory viewer calls), thiscall,
/// no stack arguments: installs the viewer vtable pointers (+0, +8, +0x20), and when the
/// physics context at +0x24 is set removes the listener at +0x20 from it, drops a
/// reference on it and clears the field; then installs the base vtable pointers (`+0x20`
/// the listener's, `+8` `010d1508`, `+0` `0102e388`).
pub fn fn_00ddcc60(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let context = e.mem.u32(me + 0x24);
    e.mem.set_u32(me, VIEWER_VTABLE_PRIMARY);
    e.mem.set_u32(me + 8, VIEWER_VTABLE_SECONDARY);
    e.mem.set_u32(me + 0x20, VIEWER_VTABLE_LISTENER);
    if context != 0 {
        e.call(CONTEXT_REMOVE_LISTENER, &args![context, me + 0x20]);
        let context = e.mem.u32(me + 0x24);
        e.call(REMOVE_REFERENCE, &args![context]);
        e.mem.set_u32(me + 0x24, 0);
    }
    e.mem.set_u32(me + 0x20, LISTENER_VTABLE);
    e.mem.set_u32(me + 8, BASE_VTABLE_SECONDARY);
    e.mem.set_u32(me, BASE_VTABLE_PRIMARY);
}

// Translated from 00ddccb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldViewerBase::hkpWorldViewerBase` (Xbox PDB), thiscall, `RET 4`: sets the word at +6
/// to 1, +8 to `010d1508`, the byte at +0xC to 1 and +0x20 to the listener vtable, then the
/// viewer vtable pointers (+0, +8, +0x20) and clears +0x24. Walks the array of objects
/// `contexts` (data pointer at +0, signed count at +4); the first whose virtual function
/// at +4 answers a name equal to `"Physics"` becomes the context at +0x24 (the object minus 8)
/// and the walk stops. A context that was found gets the listener at +0x20 added and a
/// reference added. Returns `this`.
pub fn hkp_world_viewer_base_construct(e: &mut Engine, this: Ptr, contexts: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, BASE_VTABLE_SECONDARY);
    e.mem.set_u8(me + 0xc, 1);
    e.mem.set_u32(me + 0x20, LISTENER_VTABLE);
    e.mem.set_u32(me, VIEWER_VTABLE_PRIMARY);
    e.mem.set_u32(me + 8, VIEWER_VTABLE_SECONDARY);
    e.mem.set_u32(me + 0x20, VIEWER_VTABLE_LISTENER);
    e.mem.set_u32(me + 0x24, 0);
    let count = e.mem.i32(contexts.addr() + 4);
    let mut index = 0i32;
    if count > index {
        loop {
            let data = e.mem.u32(contexts.addr());
            let object = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            let name = e.vcall(object, 4, &args![]).u32();
            let equal = e.call(STRING_COMPARE, &args![PHYSICS_TEXT, name]).u32() == 0;
            if equal {
                let data = e.mem.u32(contexts.addr());
                let object = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
                // The exe tests the object for null here (giving 0), but it has just called
                // through the same pointer's vtable, so a null object never gets this far.
                let context = if object != 0 {
                    object.wrapping_sub(8)
                } else {
                    0
                };
                e.mem.set_u32(me + 0x24, context);
                break;
            }
            index += 1;
            if index >= count {
                break;
            }
        }
    }
    let context = e.mem.u32(me + 0x24);
    if context != 0 {
        e.call(CONTEXT_ADD_LISTENER, &args![context, me + 0x20]);
        let context = e.mem.u32(me + 0x24);
        e.call(ADD_REFERENCE, &args![context]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_cc40, hkp_physics_context_world_listener_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00dd_cc60, fn_00ddcc60(Ptr)),
        entry!(0x00dd_ccb0, hkp_world_viewer_base_construct(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// Doubles for every callee outside this file, all logging `(address, arguments)`.
    fn log_callees(e: &mut Engine) -> Log {
        let log: Log = Rc::default();
        for address in [
            OPERATOR_DELETE,
            CONTEXT_REMOVE_LISTENER,
            CONTEXT_ADD_LISTENER,
            REMOVE_REFERENCE,
            ADD_REFERENCE,
        ] {
            let log = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        log
    }

    #[test]
    fn listener_destructor_deletes_only_with_the_flag() {
        let mut e = engine();
        let log = log_callees(&mut e);
        let this = e.mem.alloc(0x40);
        let got = e.call(0x00dd_cc40, &args![this, 0u32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), LISTENER_VTABLE);
        assert!(log.borrow().is_empty());
        let got = e.call(0x00dd_cc40, &args![this, 3u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![(OPERATOR_DELETE, vec![this])]);
    }

    #[test]
    fn destructor_detaches_from_the_context() {
        let mut e = engine();
        let log = log_callees(&mut e);
        let this = e.mem.alloc(0x40);
        let context = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x24, context);
        e.call(0x00dd_cc60, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                (CONTEXT_REMOVE_LISTENER, vec![context, this + 0x20]),
                (REMOVE_REFERENCE, vec![context]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this), BASE_VTABLE_PRIMARY);
        assert_eq!(e.mem.u32(this + 8), BASE_VTABLE_SECONDARY);
        assert_eq!(e.mem.u32(this + 0x20), LISTENER_VTABLE);
    }

    #[test]
    fn destructor_without_a_context_only_resets_the_vtables() {
        let mut e = engine();
        let log = log_callees(&mut e);
        let this = e.mem.alloc(0x40);
        e.call(0x00dd_cc60, &args![this]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this), BASE_VTABLE_PRIMARY);
        assert_eq!(e.mem.u32(this + 8), BASE_VTABLE_SECONDARY);
        assert_eq!(e.mem.u32(this + 0x20), LISTENER_VTABLE);
    }

    /// An array of context objects whose name functions answer the given words.
    fn contexts_named(e: &mut Engine, names: &[u32]) -> (Ptr, Vec<u32>) {
        let list = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(list, data);
        e.mem.set_u32(list + 4, names.len() as u32);
        let mut objects = Vec::new();
        for (i, &name) in names.iter().enumerate() {
            let hook = 0x0900_0100 + i as u32;
            let table = e.mem.alloc(0x10);
            e.mem.set_u32(table + 4, hook);
            e.register_double(hook, move |_, _| Ret {
                eax: name,
                ..Ret::default()
            });
            let object = e.mem.alloc(0x20);
            e.mem.set_u32(object, table);
            e.mem.set_u32(data + 4 * i as u32, object);
            objects.push(object);
        }
        (Ptr::new(list), objects)
    }

    /// `strCmp` double: 0 when the second word is 0x1111 (standing for the name "Physics").
    fn register_compare(e: &mut Engine) {
        e.register_double(STRING_COMPARE, |_, a| {
            assert_eq!(a[0], PHYSICS_TEXT);
            Ret {
                eax: u32::from(a[1] != 0x1111),
                ..Ret::default()
            }
        });
    }

    #[test]
    fn constructor_picks_the_first_physics_context_and_attaches() {
        let mut e = engine();
        let log = log_callees(&mut e);
        register_compare(&mut e);
        let (list, objects) = contexts_named(&mut e, &[0x2222, 0x1111, 0x1111]);
        let this = e.mem.alloc(0x40);
        let got = e.call(0x00dd_ccb0, &args![this, list]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        assert_eq!(e.mem.u32(this), VIEWER_VTABLE_PRIMARY);
        assert_eq!(e.mem.u32(this + 8), VIEWER_VTABLE_SECONDARY);
        assert_eq!(e.mem.u32(this + 0x20), VIEWER_VTABLE_LISTENER);
        let context = objects[1] - 8;
        assert_eq!(e.mem.u32(this + 0x24), context);
        assert_eq!(
            *log.borrow(),
            vec![
                (CONTEXT_ADD_LISTENER, vec![context, this + 0x20]),
                (ADD_REFERENCE, vec![context]),
            ]
        );
    }

    #[test]
    fn constructor_without_a_match_or_with_no_entries_attaches_nothing() {
        let mut e = engine();
        let log = log_callees(&mut e);
        register_compare(&mut e);
        let (list, _) = contexts_named(&mut e, &[0x2222, 0x3333]);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x24, 0x7777);
        e.call(0x00dd_ccb0, &args![this, list]);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        let (empty, _) = contexts_named(&mut e, &[]);
        e.call(0x00dd_ccb0, &args![this, empty]);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        // A negative count walks nothing.
        e.mem.set_u32(empty.addr() + 4, 0xffff_ffff);
        e.call(0x00dd_ccb0, &args![this, empty]);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert!(log.borrow().is_empty());
    }
}
