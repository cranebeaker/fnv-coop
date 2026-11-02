//! `NiXenonRenderer/nid3dshaderconstantmanager.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The destructor body `00e989c0` (unnamed in the engine map; `thiscall`, no stack words) and
/// the sized free `00aa1460` (`cdecl`: block, size).
const DESTRUCT_00E989C0: u32 = 0x00e9_89c0;
const FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

// Translated from 00e98a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderConstantManager::scalar deleting destructor` (engine map), `thiscall` with the
/// flags: the destructor `00e989c0`, then with bit 0 the 0x88 bytes freed (`00aa1460`).
/// Answers `this`.
pub fn ni_d3d_shader_constant_manager_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(DESTRUCT_00E989C0, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_SIZED_00AA1460, &args![this, 0x88u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e9_8a70,
        ni_d3d_shader_constant_manager_scalar_deleting_destructor(Ptr, u32) -> Ptr
    )]
}

#[cfg(test)]
mod n008_tests {
    use super::*;

    #[test]
    fn scalar_deleting_destructor_frees_with_bit_zero() {
        for (flags, frees) in [(1u32, true), (0, false)] {
            let mut e = Engine::new();
            e.register(DESTRUCT_00E989C0, |_, _| Ret::default());
            e.register(FREE_SIZED_00AA1460, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            let this = 0x0e00_1000u32;
            assert_eq!(e.call(0x00e9_8a70, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![(DESTRUCT_00E989C0, vec![this])];
            if frees {
                expected.push((FREE_SIZED_00AA1460, vec![this, 0x88]));
            }
            assert_eq!(log[1..].to_vec(), expected);
        }
    }
}
