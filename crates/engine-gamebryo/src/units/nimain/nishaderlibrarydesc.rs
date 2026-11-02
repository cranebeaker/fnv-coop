//! `NiMain/nishaderlibrarydesc.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiShaderLibraryDesc` fields (PC offsets as the code reads them): the shader descriptions
/// (+0x3c, an array of pointers) and their count (+0x44, `u16`).
const SHADER_DESCS: u32 = 0x3c;
const SHADER_COUNT: u32 = 0x44;

// Translated from 00a9ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderLibraryDesc::GetShaderCount` (Xbox PDB): the `u16` count at +0x44, zero-extended.
pub fn ni_shader_library_desc_get_shader_count(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u16(this.addr() + SHADER_COUNT))
}

// Translated from 00a9ffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderLibraryDesc::GetShaderDesc` (Xbox PDB): element `index` of the description array
/// at +0x3c (not range-checked).
pub fn ni_shader_library_desc_get_shader_desc(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    let descs = e.mem.u32(this.addr() + SHADER_DESCS);
    Ptr::new(e.mem.u32(descs.wrapping_add(index.wrapping_mul(4))))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9ffd0, ni_shader_library_desc_get_shader_count(Ptr) -> u32),
        entry!(0x00a9ffe0, ni_shader_library_desc_get_shader_desc(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod n019_tests {
    use super::*;

    #[test]
    fn count_and_descriptions() {
        let mut e = Engine::new();
        let desc = e.mem.alloc(0x48);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array + 4, 0x1234);
        e.mem.set_u32(desc + SHADER_DESCS, array);
        e.mem.set_u32(desc + SHADER_COUNT, 0xffff_0002);
        assert_eq!(e.call(0x00a9_ffd0, &args![desc]).u32(), 2);
        assert_eq!(e.call(0x00a9_ffe0, &args![desc, 1u32]).u32(), 0x1234);
    }
}
