//! `BSShader/volumetricfogshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// `NiXenonShaderDeclaration::Create` (engine map, `00e76700`, `cdecl`: renderer, entry count,
/// stream count).
const DECLARATION_CREATE: u32 = 0x00e7_6700;
/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `VolumetricFogShader::VolumetricFogShader` (engine map, `00bc58a0`, `thiscall`, no words).
const CONSTRUCT: u32 = 0x00bc_58a0;

// Translated from 00bc59b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VolumetricFogShader::CreateShader` (engine map): the creator `GetShader` (`00b55560`)
/// calls for shader 0x22. No arguments. Makes a declaration
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508`, 2 entries, 1 stream),
/// sets its one entry through its vtable slot +0x8c (five words: (0, 0, 0, 2, 0)) and calls
/// slot +0xb4. Then allocates 0xa8 bytes and runs the constructor (`00bc58a0`; a null
/// allocation is used as it is), calls the shader's slots +0x4c, +0x13c, +0x148, +0x11c, then
/// +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00bc59b0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn volumetric_fog_shader_create_shader(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 2u32, 1u32])
        .u32();
    e.vcall(declaration, 0x8c, &args![0u32, 0u32, 0u32, 2u32, 0u32]);
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0xa8u32]).u32();
    let shader = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x13c, 0x148, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The volumetric fog shader's vtable (`010baf60`; no RTTI locator before it).
const VOLUMETRIC_FOG_VTABLE_010BAF60: u32 = 0x010b_af60;

// Translated from 00bc58a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VolumetricFogShader::VolumetricFogShader` (Xbox PDB), `thiscall`, no stack words. Runs
/// `BSShader::BSShader` (`00be2200`), stores the vtable `010baf60`, constructs the `NiPointer`
/// arrays at +0x6c (3, destructor `00b79930`), +0x78 and +0x80 (2 each, destructor
/// `00b66f20`), stores 0.0 at +0x88 to +0xa4 and sets +0x68 to 0x13. Answers `this`.
// Translation incomplete: 00bc58a0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the arrays) is not translated.
pub fn volumetric_fog_shader_volumetric_fog_shader(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VOLUMETRIC_FOG_VTABLE_010BAF60);
    construct_pointer_array(e, base + 0x6c, 3, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x78, 2, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x80, 2, DESTRUCT_00B66F20);
    for offset in (0x88u32..=0xa4).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x68, 0x13);
    this
}

/// `eh vector destructor iterator` (`00ec5fce`, `stdcall`: array, element size, count,
/// destructor; `RET 0x10`).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// `BSShader::~BSShader` (engine map, `00be0f10`, `thiscall`, no stack words).
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
/// The sized Ni free `00aa1460` (`cdecl`: block, size).
const NI_DELETE: u32 = 0x00aa_1460;

// Translated from 00bc5320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `volumetricfogshader.cpp`; `thiscall`, no stack words): the
/// volumetric fog shader's destructor body. Stores the vtable `010baf60`, releases and clears
/// the two `NiPointer`s at +0x78 and the two at +0x80 (each when set), runs the
/// `eh vector destructor iterator` on the arrays at +0x80 (2, `00b66f20`), +0x78 (2,
/// `00b66f20`) and +0x6c (3, `00b79930`), then `BSShader::~BSShader`.
// Translation incomplete: 00bc5320 - the C++ exception frame (FS:[0] chain, unwind states 3 to 0 for the arrays and the base) is not translated.
pub fn fn_00bc5320(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VOLUMETRIC_FOG_VTABLE_010BAF60);
    for offset in [0x78u32, 0x7c, 0x80, 0x84] {
        ni_pointer_clear(e, base + offset);
    }
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x80, 4u32, 2u32, DESTRUCT_00B66F20],
    );
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x78, 4u32, 2u32, DESTRUCT_00B66F20],
    );
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x6c, 4u32, 3u32, DESTRUCT_00B79930],
    );
    e.call(BS_SHADER_DESTRUCT, &args![this]);
}

// Translated from 00bc5980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `volumetricfogshader.cpp`; `thiscall`, one stack word,
/// `RET 4`): the scalar deleting destructor. Runs the destructor body ([`fn_00bc5320`]), then
/// frees the 0xa8 bytes of `this` when bit 0 of `flags` is set. Answers `this`.
pub fn fn_00bc5980(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bc5320(e, this);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0xa8u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_58a0, volumetric_fog_shader_volumetric_fog_shader(Ptr) -> Ptr),
        entry!(
            0x00bc_59b0,
            volumetric_fog_shader_create_shader() -> Ptr
        ),
        entry!(0x00bc_5320, fn_00bc5320(Ptr)),
        entry!(0x00bc_5980, fn_00bc5980(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn volumetric_fog_shader_create_shader_makes_the_declaration_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[]);
        let (shader, log) = run(&mut e, 0x00bc_59b0);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 2, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0xa8]),
                (CONSTRUCT, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (shader_slot(0x148), vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn volumetric_fog_shader_constructor_builds_the_shader() {
        let mut e = rig();
        let this = e.mem.alloc(0xa8);
        e.mem.write(this, &[0xcc; 0xa8]);
        assert_eq!(e.call(CONSTRUCT, &args![this]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), VOLUMETRIC_FOG_VTABLE_010BAF60);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 3, DESTRUCT_00B79930),
                pointer_array(this + 0x78, 2, DESTRUCT_00B66F20),
                pointer_array(this + 0x80, 2, DESTRUCT_00B66F20),
            ]
        );
        for offset in (0x88u32..=0xa4).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x68), 0x13);
    }
}

#[cfg(test)]
mod destructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00bc5320_releases_the_pointers_then_destroys_the_arrays_and_base() {
        let mut e = rig();
        e.register(VECTOR_DESTRUCT, |_, _| Ret::default());
        e.register(BS_SHADER_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0xa8);
        let last = object(&mut e, 1);
        let kept = object(&mut e, 2);
        e.mem.set_u32(this + 0x78, last);
        e.mem.set_u32(this + 0x84, kept);
        e.call_log = Some(vec![]);
        e.call(0x00bc_5320, &args![this]);
        assert_eq!(e.mem.u32(this), VOLUMETRIC_FOG_VTABLE_010BAF60);
        for offset in [0x78u32, 0x7c, 0x80, 0x84] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![last + 4], vec![kept + 4]]
        );
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![last]]);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![
                vec![this + 0x80, 4, 2, DESTRUCT_00B66F20],
                vec![this + 0x78, 4, 2, DESTRUCT_00B66F20],
                vec![this + 0x6c, 4, 3, DESTRUCT_00B79930],
            ]
        );
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn fn_00bc5980_destructs_then_frees_on_bit_zero() {
        for (flags, frees) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            let mut e = rig();
            e.register(VECTOR_DESTRUCT, |_, _| Ret::default());
            e.register(BS_SHADER_DESTRUCT, |_, _| Ret::default());
            e.register(NI_DELETE, |_, _| Ret::default());
            let this = e.mem.alloc(0xa8);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00bc_5980, &args![this, flags]).u32(), this);
            assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
            let expected = if frees {
                vec![vec![this, 0xa8]]
            } else {
                vec![]
            };
            assert_eq!(calls(&e, NI_DELETE), expected);
        }
    }
}
