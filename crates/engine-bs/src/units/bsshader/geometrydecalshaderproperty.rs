//! `BSShader/geometrydecalshaderproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::bsshaderproperty::{viewer_strings_add, VIEWER_STRING};

/// `BSShaderLightingProperty::GetViewerStrings` (`00b71450`, `thiscall`, one word) and the
/// class-name record (`012005b8`).
const LIGHTING_GET_VIEWER_STRINGS: u32 = 0x00b7_1450;
const CLASS_RTTI_012005B8: u32 = 0x0120_05b8;

// Translated from 00bb44a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GeometryDecalShaderProperty::GetViewerStrings` (engine map; the body matches):
/// `thiscall`, one word (the strings): the `BSShaderLightingProperty` strings (`00b71450`),
/// then the class name added inline.
pub fn geometry_decal_shader_property_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    e.call(LIGHTING_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(CLASS_RTTI_012005B8);
    let text = e.call(VIEWER_STRING, &args![class_name]).u32();
    viewer_strings_add(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00bb_44a0,
        geometry_decal_shader_property_get_viewer_strings(Ptr, u32)
    )]
}

#[cfg(test)]
mod n012_tests {
    use super::*;
    use crate::units::bsshader::bsshaderproperty::{STRINGS_SET_AT, STRINGS_SET_SIZE};

    #[test]
    fn get_viewer_strings_adds_the_class_name_after_the_base() {
        let mut e = Engine::new();
        e.map(0x0120_0000, 0x1000);
        e.mem.set_u32(CLASS_RTTI_012005B8, 0x00de_ca10);
        e.register(LIGHTING_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(STRINGS_SET_SIZE, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |_, _| Ret::default());
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xa, 3);
        e.mem.set_u16(strings + 8, 3);
        e.mem.set_u16(strings + 0xe, 4);
        let this = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        e.call(0x00bb_44a0, &args![this, strings]);
        let log = e.call_log.take().unwrap();
        let addrs: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            addrs,
            vec![
                0x00bb_44a0,
                LIGHTING_GET_VIEWER_STRINGS,
                VIEWER_STRING,
                STRINGS_SET_SIZE,
                STRINGS_SET_AT
            ]
        );
        assert_eq!(log[1].1, vec![this, strings]);
        assert_eq!(log[2].1, vec![0x00de_ca10]);
        assert_eq!(log[3].1, vec![strings, 7]);
        assert_eq!(log[4].1[..2], [strings, 3]);
    }
}
