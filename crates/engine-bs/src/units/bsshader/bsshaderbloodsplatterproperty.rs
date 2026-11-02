//! `BSShader/bsshaderbloodsplatterproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::bsshaderproperty::{
    viewer_strings_add, VIEWER_STRING, VIEWER_STRING_TEXT,
};

/// `BSShaderProperty::GetViewerStrings` (`00ba9890`, `thiscall`, one word), the class-name
/// record (`011ff0f4`), `BSUtilities::GetTextureName` (`00c4bb40`, `cdecl`, one word), the
/// viewer-string maker of a name and an object (`00a7d5a0`) and the label (`"base texture"`,
/// `010ae79c`).
const SHADER_PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00ba_9890;
const CLASS_RTTI_011FF0F4: u32 = 0x011f_f0f4;
const GET_TEXTURE_NAME: u32 = 0x00c4_bb40;
const VIEWER_STRING_OBJECT: u32 = 0x00a7_d5a0;
const LABEL_BASE_TEXTURE: u32 = 0x010a_e79c;

// Translated from 00b81520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderBloodSplatterProperty::GetViewerStrings` (engine map; the body matches):
/// `thiscall`, one word (the strings). The `BSShaderProperty` strings (`00ba9890`), the class
/// name, then for each of the two textures at `+0x60` and `+0x64` a `"base texture"` line
/// (by name through `00c4bb40` when it has one, else as the object), each added inline.
pub fn bs_shader_blood_splatter_property_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: u32,
) {
    e.call(SHADER_PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(CLASS_RTTI_011FF0F4);
    let text = e.call(VIEWER_STRING, &args![class_name]).u32();
    viewer_strings_add(e, strings, text);
    for slot in [this.addr() + 0x60, this.addr() + 0x64] {
        let texture = e.mem.u32(slot);
        let name = e.call(GET_TEXTURE_NAME, &args![texture]).u32();
        let text = if name != 0 {
            e.call(VIEWER_STRING_TEXT, &args![LABEL_BASE_TEXTURE, name])
                .u32()
        } else {
            let texture = e.mem.u32(slot);
            e.call(VIEWER_STRING_OBJECT, &args![LABEL_BASE_TEXTURE, texture])
                .u32()
        };
        viewer_strings_add(e, strings, text);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00b8_1520,
        bs_shader_blood_splatter_property_get_viewer_strings(Ptr, u32)
    )]
}

#[cfg(test)]
mod n012_tests {
    use super::*;
    use crate::units::bsshader::bsshaderproperty::{STRINGS_SET_AT, STRINGS_SET_SIZE};

    #[test]
    fn get_viewer_strings_names_both_textures() {
        let mut e = Engine::new();
        e.map(0x011f_f000, 0x1000);
        e.mem.set_u32(CLASS_RTTI_011FF0F4, 0x00c1_0000);
        e.register(SHADER_PROPERTY_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING, |_, a| (a[0] | 1).into_ret());
        e.register(GET_TEXTURE_NAME, |_, a| {
            if a[0] == 0x1000 { 0x5000u32 } else { 0 }.into_ret()
        });
        e.register(VIEWER_STRING_TEXT, |_, a| (a[1] + 2).into_ret());
        e.register(VIEWER_STRING_OBJECT, |_, a| (a[1] + 3).into_ret());
        e.register(STRINGS_SET_SIZE, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let v = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], v);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x20);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 8, 8);
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x60, 0x1000);
        e.mem.set_u32(this + 0x64, 0x2000);
        e.call_log = Some(vec![]);
        e.call(0x00b8_1520, &args![this, strings]);
        assert_eq!(
            (e.mem.u32(base), e.mem.u32(base + 4), e.mem.u32(base + 8)),
            (0x00c1_0001, 0x5002, 0x2003)
        );
        assert_eq!(e.mem.u16(strings + 0xa), 3);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(SHADER_PROPERTY_GET_VIEWER_STRINGS, vec![this, strings])));
        assert!(log.contains(&(VIEWER_STRING_TEXT, vec![LABEL_BASE_TEXTURE, 0x5000])));
        assert!(log.contains(&(VIEWER_STRING_OBJECT, vec![LABEL_BASE_TEXTURE, 0x2000])));
    }
}
