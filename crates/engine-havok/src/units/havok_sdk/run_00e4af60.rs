//! `Havok SDK/run_00e4af60` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkStringPtr::hkStringPtr` (Xbox PDB, `00cb0a60`): thiscall on the string pointer, `this` only.
const STRING_CONSTRUCTOR: u32 = 0x00cb_0a60;
/// `hkStringPtr::hkStringPtr(const char *)` (`00cb0a70`): thiscall on the string pointer, one
/// stack word, `RET 4`.
const STRING_CONSTRUCTOR_WITH_TEXT: u32 = 0x00cb_0a70;

// Translated from 00e4af60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall constructor: stores `1` at `+6` (a word), the vtable `010ec1fc` and builds the
/// string pointer at `+0x14` (`00cb0a60`). Returns `this`.
pub fn fn_00e4af60(e: &mut Engine, this: Ptr) -> u32 {
    let this = this.addr();
    e.mem.set_u16(this.wrapping_add(6), 1);
    e.mem.set_u32(this, 0x010e_c1fc);
    e.call(STRING_CONSTRUCTOR, &args![this.wrapping_add(0x14)]);
    this
}

// Translated from 00e4af80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall constructor, `RET 4`: stores the vtable `010ec1fc` and builds the string pointer at
/// `+0x14` from `text` (`00cb0a70`). Returns `this`.
pub fn fn_00e4af80(e: &mut Engine, this: Ptr, text: u32) -> u32 {
    let this = this.addr();
    e.mem.set_u32(this, 0x010e_c1fc);
    e.call(
        STRING_CONSTRUCTOR_WITH_TEXT,
        &args![this.wrapping_add(0x14), text],
    );
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_af60, fn_00e4af60(Ptr) -> u32),
        entry!(0x00e4_af80, fn_00e4af80(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn doubles() -> (Engine, Calls) {
        let mut e = Engine::new();
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in [STRING_CONSTRUCTOR, STRING_CONSTRUCTOR_WITH_TEXT] {
            let log = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        (e, calls)
    }

    #[test]
    fn plain_constructor_builds_the_empty_string() {
        let (mut e, calls) = doubles();
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00e4_af60, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010e_c1fc);
        assert_eq!(
            *calls.borrow(),
            vec![(STRING_CONSTRUCTOR, vec![this + 0x14])]
        );
    }

    #[test]
    fn text_constructor_passes_the_text_on() {
        let (mut e, calls) = doubles();
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00e4_af80, &args![this, 0x5555u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010e_c1fc);
        assert_eq!(
            *calls.borrow(),
            vec![(STRING_CONSTRUCTOR_WITH_TEXT, vec![this + 0x14, 0x5555])]
        );
    }
}
