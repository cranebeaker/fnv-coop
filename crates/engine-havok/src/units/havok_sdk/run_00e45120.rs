//! `Havok SDK/run_00e45120` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor run first (`00e46f20`): thiscall, one stack word, `RET 4`.
const BASE_CONSTRUCTOR: u32 = 0x00e4_6f20;

// Translated from 00e45120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall constructor, `RET 4`: runs the base constructor (`00e46f20`) on `this` with
/// `source`, stores the vtable `010ec004` and, when `source` is not null, the byte 4 at
/// `this + 8`. Returns `this`.
pub fn fn_00e45120(e: &mut Engine, this: Ptr, source: u32) -> u32 {
    let this = this.addr();
    e.call(BASE_CONSTRUCTOR, &args![this, source]);
    e.mem.set_u32(this, 0x010e_c004);
    if source != 0 {
        e.mem.set_u8(this.wrapping_add(8), 4);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e4_5120, fn_00e45120(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn constructed() -> (Engine, Rc<RefCell<Vec<Vec<u32>>>>) {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(BASE_CONSTRUCTOR, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        (e, seen)
    }

    #[test]
    fn source_sets_the_byte_after_the_base_constructor() {
        let (mut e, seen) = constructed();
        let this = e.mem.alloc(0x20);
        let result = e.call(0x00e4_5120, &args![this, 0x77u32]).u32();
        assert_eq!(result, this);
        assert_eq!(*seen.borrow(), vec![vec![this, 0x77]]);
        assert_eq!(e.mem.u32(this), 0x010e_c004);
        assert_eq!(e.mem.u8(this + 8), 4);
    }

    #[test]
    fn null_source_leaves_the_byte_alone() {
        let (mut e, _) = constructed();
        let this = e.mem.alloc(0x20);
        e.mem.set_u8(this + 8, 9);
        e.call(0x00e4_5120, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 8), 9);
        assert_eq!(e.mem.u32(this), 0x010e_c004);
    }
}
