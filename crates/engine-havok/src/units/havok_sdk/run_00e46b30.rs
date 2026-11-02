//! `Havok SDK/run_00e46b30` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor (`00cbfba0`): thiscall, `this` only.
const BASE_CONSTRUCTOR: u32 = 0x00cb_fba0;
/// `hkReferencedObject::addReference` (Xbox PDB, `00c90510`): thiscall, `this` only.
const ADD_REFERENCE: u32 = 0x00c9_0510;

// Translated from 00e46b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall constructor, `RET 4`: runs the base constructor (`00cbfba0`), stores the vtables
/// (`010e7874` at `+0`, `010e7868`, `010e7854`, `010e7848` and `010e783c` at `+8`, `+0xc`,
/// `+0x10`, `+0x14`, `010e781c` at `+0x30`, after `010cb1e4` which it overwrites at once) and
/// `object` at `+0x34`, taking a reference on it (`00c90510`) when it is not null. Returns
/// `this`.
pub fn fn_00e46b30(e: &mut Engine, this: Ptr, object: u32) -> u32 {
    let this = this.addr();
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.wrapping_add(0x30), 0x010c_b1e4);
    e.mem.set_u32(this, 0x010e_7874);
    e.mem.set_u32(this.wrapping_add(8), 0x010e_7868);
    e.mem.set_u32(this.wrapping_add(0xc), 0x010e_7854);
    e.mem.set_u32(this.wrapping_add(0x10), 0x010e_7848);
    e.mem.set_u32(this.wrapping_add(0x14), 0x010e_783c);
    e.mem.set_u32(this.wrapping_add(0x30), 0x010e_781c);
    e.mem.set_u32(this.wrapping_add(0x34), object);
    if object != 0 {
        e.call(ADD_REFERENCE, &args![object]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e4_6b30, fn_00e46b30(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn doubles(e: &mut Engine) -> Calls {
        let seen: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in [BASE_CONSTRUCTOR, ADD_REFERENCE] {
            let log = Rc::clone(&seen);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        seen
    }

    fn fields(e: &Engine, this: u32) -> Vec<u32> {
        [0u32, 8, 0xc, 0x10, 0x14, 0x30, 0x34]
            .iter()
            .map(|offset| e.mem.u32(this + offset))
            .collect()
    }

    #[test]
    fn object_gets_a_reference() {
        let mut e = Engine::new();
        let seen = doubles(&mut e);
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00e4_6b30, &args![this, 0x1234u32]).u32(), this);
        assert_eq!(
            fields(&e, this),
            vec![
                0x010e_7874,
                0x010e_7868,
                0x010e_7854,
                0x010e_7848,
                0x010e_783c,
                0x010e_781c,
                0x1234
            ]
        );
        assert_eq!(
            *seen.borrow(),
            vec![
                (BASE_CONSTRUCTOR, vec![this]),
                (ADD_REFERENCE, vec![0x1234])
            ]
        );
    }

    #[test]
    fn null_object_takes_no_reference() {
        let mut e = Engine::new();
        let seen = doubles(&mut e);
        let this = e.mem.alloc(0x40);
        e.call(0x00e4_6b30, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(*seen.borrow(), vec![(BASE_CONSTRUCTOR, vec![this])]);
    }
}
