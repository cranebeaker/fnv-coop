//! `Havok SDK/run_00dfeb00` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfeb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_5ae4.
pub fn return_constant_00dfeb00(_e: &mut Engine) -> u32 {
    0x010e_5ae4
}

// Translated from 00dfeaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word `object`): calls the object's virtual function at
/// vtable offset 0 (the destructor) with the one stack word 0 and answers what it answers. It is
/// the cleanup function of a type-info record (word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject`
/// calls it with `(object)`): the start-up initializer at `00fc4149` stores its address there.
pub fn fn_00dfeaa0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfeac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words `(object, extra)`): the finish function of a
/// type-info record (word `+4`; the start-up initializer at `00fc413f` stores its address there,
/// the cleanup function `00dfeaa0` follows at `+8`). Nothing happens for a null `object`.
/// Otherwise it sets the fields a chain of inlined constructors leaves, in the order they write
/// them: the vtable `010dec04`; with a non-zero `extra` the byte at `+9` is cleared; the vtable
/// `010e5ae4`; the words at `+0x1c` (`010c925c`), `+0x14` (`010debf4`) and `+0x1c` again
/// (`010debe4`); with `extra` the byte at `+9` is set to 1. Answers `object` (`EAX`).
pub fn fn_00dfeac0(e: &mut Engine, object: Ptr, extra: u32) -> u32 {
    let base = object.addr();
    if base == 0 {
        return 0;
    }
    e.mem.set_u32(base, 0x010d_ec04);
    if extra != 0 {
        e.mem.set_u8(base.wrapping_add(9), 0);
    }
    e.mem.set_u32(base, 0x010e_5ae4);
    e.mem.set_u32(base.wrapping_add(0x1c), 0x010c_925c);
    e.mem.set_u32(base.wrapping_add(0x14), 0x010d_ebf4);
    e.mem.set_u32(base.wrapping_add(0x1c), 0x010d_ebe4);
    if extra != 0 {
        e.mem.set_u8(base.wrapping_add(9), 1);
    }
    base
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfeaa0, fn_00dfeaa0(Ptr) -> u32),
        entry!(0x00dfeac0, fn_00dfeac0(Ptr, u32) -> u32),
        entry!(0x00dfeb00, return_constant_00dfeb00() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00dfeaa0_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        e.put_vtable(0x0900_0000, &[0x0900_0100]);
        e.mem.set_u32(object, 0x0900_0000);
        e.register(0x0900_0100, |_, a| Ret {
            eax: a[0].wrapping_add(a[1]).wrapping_add(7),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00df_eaa0, &args![object]).u32();
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
    fn fn_00dfeac0_without_extra_sets_the_vtable_and_two_words() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.mem.set_u8(object + 9, 0x55);
        assert_eq!(e.call(0x00df_eac0, &args![object, 0u32]).u32(), object);
        assert_eq!(e.mem.u32(object), 0x010e_5ae4);
        assert_eq!(e.mem.u32(object + 0x14), 0x010d_ebf4);
        assert_eq!(e.mem.u32(object + 0x1c), 0x010d_ebe4);
        assert_eq!(e.mem.u8(object + 9), 0x55);
    }

    #[test]
    fn fn_00dfeac0_with_extra_sets_the_flag_byte() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00df_eac0, &args![object, 1u32]).u32(), object);
        assert_eq!(e.mem.u8(object + 9), 1);
        assert_eq!(e.mem.u32(object), 0x010e_5ae4);
    }

    #[test]
    fn fn_00dfeac0_leaves_a_null_object_alone() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_eac0, &args![0u32, 1u32]).u32(), 0);
    }

    #[test]
    fn return_constant_00dfeb00_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_eb00, &args![]).u32(), 0x010e_5ae4);
    }
}
