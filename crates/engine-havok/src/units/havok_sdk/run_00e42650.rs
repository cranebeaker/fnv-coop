//! `Havok SDK/run_00e42650` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Virtual slot 0x24 of each element, called with one stack word (`00e42650`).
const ELEMENT_SLOT_WITH_ARGUMENT: u32 = 0x24;
/// Virtual slot 0x28 of each element, called with no stack words (`00e42680`).
const ELEMENT_SLOT: u32 = 0x28;

// Translated from 00e42650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4`: calls virtual slot `0x24` with `argument` on each of the `[+0xc]` objects
/// of the pointer array at `*(this + 8)`, in order. The count is read once; the array pointer
/// is read again for every element.
pub fn fn_00e42650(e: &mut Engine, this: Ptr, argument: u32) {
    let this = this.addr();
    let count = e.mem.i32(this.wrapping_add(0xc));
    for index in 0..count.max(0) as u32 {
        let list = e.mem.u32(this.wrapping_add(8));
        let object = e.mem.u32(list.wrapping_add(index.wrapping_mul(4)));
        e.vcall(object, ELEMENT_SLOT_WITH_ARGUMENT, &args![argument]);
    }
}

// Translated from 00e42680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall: calls virtual slot `0x28` (no stack words) on each of the `[+0xc]` objects of the
/// pointer array at `*(this + 8)`, in order.
pub fn fn_00e42680(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let count = e.mem.i32(this.wrapping_add(0xc));
    for index in 0..count.max(0) as u32 {
        let list = e.mem.u32(this.wrapping_add(8));
        let object = e.mem.u32(list.wrapping_add(index.wrapping_mul(4)));
        e.vcall(object, ELEMENT_SLOT, &[]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_2650, fn_00e42650(Ptr, u32)),
        entry!(0x00e4_2680, fn_00e42680(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The words each virtual call received (the object first).
    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// Two objects whose vtable slots 9 (`0x24`) and 10 (`0x28`) are doubles, in a container
    /// that holds `count` of them.
    fn container(e: &mut Engine, count: u32) -> (u32, [u32; 2], Calls) {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let mut slots = [0u32; 11];
        for (slot, address) in [(9, 0x0ff0_0024u32), (10, 0x0ff0_0028)] {
            slots[slot] = address;
            let log = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                let mut words = vec![address];
                words.extend_from_slice(a);
                log.borrow_mut().push(words);
                Ret::default()
            });
        }
        e.put_vtable(0x0126_0000, &slots);
        let objects = [e.mem.alloc(8), e.mem.alloc(8)];
        for object in objects {
            e.mem.set_u32(object, 0x0126_0000);
        }
        let this = e.mem.alloc(0x20);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, objects[0]);
        e.mem.set_u32(list + 4, objects[1]);
        e.mem.set_u32(this + 8, list);
        e.mem.set_u32(this + 0xc, count);
        (this, objects, calls)
    }

    #[test]
    fn slot_0x24_is_called_on_every_object_with_the_argument() {
        let mut e = engine();
        let (this, objects, calls) = container(&mut e, 2);
        e.call(0x00e4_2650, &args![this, 0x1234u32]);
        assert_eq!(
            *calls.borrow(),
            vec![
                vec![0x0ff0_0024, objects[0], 0x1234],
                vec![0x0ff0_0024, objects[1], 0x1234]
            ]
        );
    }

    #[test]
    fn slot_0x28_is_called_on_every_object() {
        let mut e = engine();
        let (this, objects, calls) = container(&mut e, 2);
        e.call(0x00e4_2680, &args![this]);
        assert_eq!(
            *calls.borrow(),
            vec![vec![0x0ff0_0028, objects[0]], vec![0x0ff0_0028, objects[1]]]
        );
    }

    #[test]
    fn no_objects_means_no_calls() {
        let mut e = engine();
        let (this, _, calls) = container(&mut e, 0);
        e.call(0x00e4_2650, &args![this, 1u32]);
        e.call(0x00e4_2680, &args![this]);
        let (this, _, other) = container(&mut e, 0xffff_ffff);
        e.call(0x00e4_2650, &args![this, 1u32]);
        assert!(calls.borrow().is_empty());
        assert!(other.borrow().is_empty());
    }
}
