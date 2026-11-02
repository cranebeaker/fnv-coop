//! `Havok SDK/run_00dfc070` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfc070 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_f9f4.
pub fn return_constant_00dfc070(_e: &mut Engine) -> u32 {
    0x010d_f9f4
}

// Translated from 00dfc040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word `object`): calls the object's virtual function at
/// vtable offset 0 (the destructor) with the one stack word 0 and answers what it answers. It is
/// the cleanup function of a type-info record (word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject`
/// calls it with `(object)`): the start-up initializer at `00fc1c69` stores its address there.
pub fn fn_00dfc040(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfc060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words `(object, _unused_1)`): with a non-null `object`
/// stores the vtable word 0x010d_f9f4 in its first word. `EAX` is `object` on every path, which is what it
/// answers. It is the finish function of a type-info record (word `+4`;
/// `hkTypeInfoRegistry::finishLoadedObject` calls it with `(object, extra)`): the start-up
/// initializer at `00fc1c5f` stores its address there.
pub fn fn_00dfc060(e: &mut Engine, object: Ptr, _unused_1: u32) -> u32 {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010d_f9f4);
    }
    object.addr()
}

// Translated from 00dfc020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a bare `RET`; no instruction reads a stack word and the exe has no
/// reference to it, so it is declared without words. It sits before the cleanup function
/// `00dfc040` and the finish function `00dfc060` of the type-info record the initializer at
/// `00fc1c50` fills. It does nothing.
pub fn fn_00dfc020(_e: &mut Engine) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfc040, fn_00dfc040(Ptr) -> u32),
        entry!(0x00dfc060, fn_00dfc060(Ptr, u32) -> u32),
        entry!(0x00dfc020, fn_00dfc020()),
        entry!(0x00dfc070, return_constant_00dfc070() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00dfc040_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        e.put_vtable(0x0900_0000, &[0x0900_0100]);
        e.mem.set_u32(object, 0x0900_0000);
        e.register(0x0900_0100, |_, a| Ret {
            eax: a[0].wrapping_add(a[1]).wrapping_add(7),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00df_c040, &args![object]).u32();
        assert_eq!(answer, object + 7);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0900_0100)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x0900_0100, vec![object, 0])]);
    }

    #[test]
    fn fn_00dfc060_stores_the_vtable_in_a_non_null_object() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        assert_eq!(e.call(0x00df_c060, &args![object, 1u32]).u32(), object);
        assert_eq!(e.mem.u32(object), 0x010d_f9f4);
    }

    #[test]
    fn fn_00dfc060_leaves_a_null_object_alone() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c060, &args![0u32, 1u32]).u32(), 0);
    }

    #[test]
    fn fn_00dfc020_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00df_c020, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn return_constant_00dfc070_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c070, &args![]).u32(), 0x010d_f9f4);
    }
}
