//! `Havok SDK/run_00cc0820` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The global pointer (`01268b08`) to the object whose virtual method `+0xc` this function
/// asks first (the object's class is unconfirmed).
const SOURCE_OBJECT: u32 = 0x0126_8b08;
/// `00d47f90` (takes no argument and returns the constant `01268b54`, a class record whose first
/// word is the class name; named by neither the map nor Ghidra).
const FIND_CLASS: u32 = 0x00d4_7f90;
/// `hkClass::getName` (`00d46d40`, named by Ghidra from the Xbox PDB; `this` only).
const CLASS_GET_NAME: u32 = 0x00d4_6d40;

// Translated from 00cc0820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks the global object (`01268b08`) for a word with its virtual method `+0xc` and pushes it;
/// `00d47f90` (which takes no argument, so that word stays on the stack) gives the class record,
/// `hkClass::getName` (`00d46d40`) its name, and `this`'s virtual method `+0x18` is called with the
/// name and, below it on the stack, the first word. The vtable of `this` is read
/// before the helpers run and its slot after them, as the exe does. The class is
/// unconfirmed.
pub fn fn_00cc0820(e: &mut Engine, this: Ptr) {
    let source: u32 = e.global(SOURCE_OBJECT);
    let word = e.vcall(source, 0xc, &args![]).u32();
    let vtable = e.mem.u32(this.addr());
    let class = e.call(FIND_CLASS, &args![]).u32();
    let name = e.call(CLASS_GET_NAME, &args![class]).u32();
    let target = e.mem.u32(vtable.wrapping_add(0x18));
    e.call(target, &args![this, name, word]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cc_0820, fn_00cc0820(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const SOURCE_SLOT_DOUBLE: u32 = 0x0900_00c1;
    const NAME_SLOT_DOUBLE: u32 = 0x0900_0181;

    /// The four calls run in order with each result passed on.
    #[test]
    fn name_of_the_class_is_passed_to_the_object() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(SOURCE_SLOT_DOUBLE, move |_, a| {
            sink.borrow_mut().push((1, a.to_vec()));
            Ret {
                eax: 0x111,
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(FIND_CLASS, move |_, a| {
            sink.borrow_mut().push((2, a.to_vec()));
            Ret {
                eax: 0x222,
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(CLASS_GET_NAME, move |_, a| {
            sink.borrow_mut().push((3, a.to_vec()));
            Ret {
                eax: 0x333,
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(NAME_SLOT_DOUBLE, move |_, a| {
            sink.borrow_mut().push((4, a.to_vec()));
            Ret::default()
        });
        let source_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(source_vtable + 0xc, SOURCE_SLOT_DOUBLE);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, source_vtable);
        e.set_global(SOURCE_OBJECT, source);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x18, NAME_SLOT_DOUBLE);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        e.call(0x00cc_0820, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                (1, vec![source]),
                (2, vec![]),
                (3, vec![0x222]),
                (4, vec![this, 0x333, 0x111])
            ]
        );
    }
}
