//! `Havok SDK/run_00d873f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkClassMember::getStructClass` (`00d86910`, thiscall on the member) and the class accessor
/// setup `00d873b0` (thiscall `(first word, class)`, `RET 8`).
const MEMBER_GET_STRUCT_CLASS: u32 = 0x00d8_6910;
const CLASS_ACCESSOR_INIT_PAIR: u32 = 0x00d8_73b0;

// Translated from 00d873f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall on an object whose word `+4` is a class member and whose word `+0` is a pointer, one
/// stack word `destination` (`RET 4`): asks the member for its struct class
/// (`hkClassMember::getStructClass`), runs `00d873b0` on `destination` with the object's first
/// word and the class, and answers `destination`. The exe's name for it is unknown.
pub fn fn_00d873f0(e: &mut Engine, this: u32, destination: u32) -> u32 {
    let member = e.mem.u32(this + 4);
    let class = e.call(MEMBER_GET_STRUCT_CLASS, &args![member]).u32();
    let first = e.mem.u32(this);
    e.call(CLASS_ACCESSOR_INIT_PAIR, &args![destination, first, class]);
    destination
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d8_73f0, fn_00d873f0(u32, u32) -> u32)]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    #[test]
    fn the_accessor_is_set_up_with_the_first_word_and_the_class() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(MEMBER_GET_STRUCT_CLASS, |_, a| (a[0] + 0x1000).into_ret());
        e.register(CLASS_ACCESSOR_INIT_PAIR, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0xf1f1);
        e.mem.set_u32(this + 4, 0x500);
        let result = e.call(0x00d8_73f0, &args![this, 0x9000u32]).u32();
        assert_eq!(result, 0x9000);
        let log = e.call_log.as_ref().unwrap();
        assert!(log.contains(&(MEMBER_GET_STRUCT_CLASS, vec![0x500])));
        assert!(log.contains(&(CLASS_ACCESSOR_INIT_PAIR, vec![0x9000, 0xf1f1, 0x1500])));
    }

    #[test]
    fn the_first_word_is_read_after_the_class_lookup() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        // The lookup changes the object's first word: the setup sees the new one.
        e.register_double(MEMBER_GET_STRUCT_CLASS, |e, a| {
            e.mem.set_u32(a[0] - 4, 0xbeef);
            Ret::default()
        });
        e.register(CLASS_ACCESSOR_INIT_PAIR, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0xf1f1);
        e.mem.set_u32(this + 4, this + 4);
        e.call(0x00d8_73f0, &args![this, 0x9000u32]);
        let log = e.call_log.as_ref().unwrap();
        assert!(log.contains(&(CLASS_ACCESSOR_INIT_PAIR, vec![0x9000, 0xbeef, 0])));
    }
}
