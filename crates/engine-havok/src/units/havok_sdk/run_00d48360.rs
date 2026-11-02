//! `Havok SDK/run_00d48360` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d48360 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_f9a4.
pub fn return_constant_00d48360(_e: &mut Engine) -> u32 {
    0x010c_f9a4
}

// Translated from 00d48330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fbec69): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00d48330(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00d48350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fbec5f): cdecl with two stack words, `object` and `finishing` (never read). When `object` is not null, stores the vtable word 0x010c_f9a4 at its start.
pub fn fn_00d48350(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010c_f9a4);
    }
}

// Translated from 00d48300 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name; thiscall with one stack word (`RET 4`), `this` and `finishing`: runs the `hkStringPtr` function `00cb0a70` (thiscall, one stack word) on `this` and then on `this + 4`, each with `finishing`, and answers `this`. Called by `00d47f40`.
pub fn fn_00d48300(e: &mut Engine, this: Ptr, finishing: u32) -> Ptr {
    e.call(0x00cb_0a70, &args![this.addr(), finishing]);
    e.call(0x00cb_0a70, &args![this.addr() + 4, finishing]);
    this
}

// Translated from 00d48320 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name; thiscall with one stack word (`RET 4`, never read), `this` and `_unused_1`: answers `this`. Called by `00d47f70`.
pub fn fn_00d48320(_e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d48360, return_constant_00d48360() -> u32),
        entry!(0x00d48330, fn_00d48330(Ptr) -> u32),
        entry!(0x00d48350, fn_00d48350(Ptr, u32)),
        entry!(0x00d48300, fn_00d48300(Ptr, u32) -> Ptr),
        entry!(0x00d48320, fn_00d48320(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00d48360_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00d4_8360, &args![]).u32(), 0x010c_f9a4);
    }
}

#[cfg(test)]
mod tests_a07_gap {
    use super::*;

    /// The argument words of every call to `address` in the engine's call log (which is taken).
    fn calls_to(e: &mut Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words)
            .collect()
    }

    /// The address the doubled virtual slot 0 of [`object_with_slot_0`] answers at.
    const SLOT_0_TARGET: u32 = 0x0900_0001;

    /// An object whose vtable's slot 0 is a double answering 0x1234.
    fn object_with_slot_0(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable, SLOT_0_TARGET);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        e.register(SLOT_0_TARGET, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        object
    }

    #[test]
    fn fn_00d48330_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d4_8330, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00d48350_stores_the_vtable_word_unless_the_object_is_null() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.call(0x00d4_8350, &args![0u32, 1u32]);
        assert_eq!(e.mem.u32(object), 0);
        e.call(0x00d4_8350, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010c_f9a4);
    }

    #[test]
    fn fn_00d48300_runs_the_string_function_on_both_members() {
        let mut e = Engine::new();
        e.register(0x00cb_0a70, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d4_8300, &args![this, 3u32]).u32(), this);
        assert_eq!(
            calls_to(&mut e, 0x00cb_0a70),
            vec![vec![this, 3], vec![this + 4, 3]]
        );
    }

    #[test]
    fn fn_00d48320_answers_this() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d4_8320, &args![0x1234u32, 9u32]).u32(), 0x1234);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
