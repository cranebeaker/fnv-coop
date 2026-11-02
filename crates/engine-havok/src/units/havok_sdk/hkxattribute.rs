//! `Havok SDK/hkxattribute.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkStringPtr::hkStringPtr()` (`00cb0a60`, `thiscall`).
const STRING_PTR_CONSTRUCT: u32 = 0x00cb_0a60;

// Translated from 00e3f8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxAttribute::hkxAttribute` (Xbox PDB), `thiscall`, no stack words: constructs the name
/// `hkStringPtr` at `this` (`00cb0a60`), clears the value pointer at `+4` and returns `this`.
pub fn hkx_attribute_hkx_attribute(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(STRING_PTR_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e3f8e0, hkx_attribute_hkx_attribute(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Registers a double at `addr` that logs its argument words and answers `answer`.
    fn log_calls(e: &mut Engine, addr: u32, answer: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let kept = log.clone();
        e.register_double(addr, move |_, a| {
            kept.borrow_mut().push(a.to_vec());
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn the_constructor_builds_the_name_and_clears_the_value() {
        let mut e = Engine::new();
        let log = log_calls(&mut e, STRING_PTR_CONSTRUCT, 0);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x1234);
        assert_eq!(e.call(0x00e3_f8e0, &args![this]).u32(), this);
        assert_eq!(*log.borrow(), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 4), 0);
    }
}
