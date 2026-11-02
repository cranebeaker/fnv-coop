//! `NiMain/nishaderdeclaration.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiShaderDeclaration`'s vtable (`011011ec`), the `NiObject` destructor body `00a5d3d0` and
/// the sized delete `00aa1460` (cdecl: block, size).
const NI_SHADER_DECLARATION_VTABLE: u32 = 0x0110_11ec;
const NI_OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
const SIZED_DELETE: u32 = 0x00aa_1460;

// Translated from 00ee88e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderDeclaration::_scalar_deleting_destructor_` (Xbox PDB): the vtable `011011ec`,
/// `NiObject`'s destructor body (`00a5d3d0`), then the sized delete of 8 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_shader_declaration_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), NI_SHADER_DECLARATION_VTABLE);
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 8u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00ee_88e0,
        ni_shader_declaration_scalar_deleting_destructor(Ptr, u32) -> Ptr
    )]
}

#[cfg(test)]
mod n019_tests {
    use super::*;

    #[test]
    fn scalar_deleting_destructor_frees_on_bit_0() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_DESTRUCTOR, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_88e0, &args![object, 0u32]).u32(), object);
        assert_eq!(e.mem.u32(object), NI_SHADER_DECLARATION_VTABLE);
        e.call(0x00ee_88e0, &args![object, 1u32]);
        let log = e.call_log.take().unwrap();
        let deletes: Vec<_> = log.iter().filter(|c| c.0 == SIZED_DELETE).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![object, 8]);
        assert_eq!(
            log.iter().filter(|c| c.0 == NI_OBJECT_DESTRUCTOR).count(),
            2
        );
    }
}
