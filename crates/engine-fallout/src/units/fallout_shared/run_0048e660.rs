//! `fallout shared/run_0048e660` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// This class's vtable (RTTI `.?AVTESValueForm@@`).
const VTABLE: u32 = 0x0101_d81c;
/// The `InitializeDataComponent` body the constructor calls (`008c6060`: a shared empty body; the engine map names it ActorPackage::NullInstanceData because the linker folded it).
const INITIALIZE_DATA: u32 = 0x008c_6060;

// Translated from 0048e7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `TESValueForm` (the engine map gives it
/// no name; the RTTI name of the vtable it stores is `.?AVTESValueForm@@`): runs the
/// base constructor, stores the vtable at `+0`, then calls the component's
/// `InitializeDataComponent` body directly. Returns `this`.
pub fn fn_0048e7a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(INITIALIZE_DATA, &args![this]);
    this
}

/// `00406d30` (cdecl: destination, size, source): `strcpy_s`.
const STRCPY_S: u32 = 0x0040_6d30;
/// `0040ab30` (cdecl: text, character): `strrchr`.
const STRRCHR: u32 = 0x0040_ab30;
/// `"_n.dds"`.
const NORMAL_MAP_SUFFIX: u32 = 0x0101_d7ec;
/// `"_sk.dds"`.
const SKIN_MAP_SUFFIX: u32 = 0x0101_d7f4;
/// `"Textures\"`.
const TEXTURES_FOLDER: u32 = 0x0101_d7fc;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, vfptr offset, source type,
/// target type, is-reference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The RTTI type descriptor of `TESForm`.
const TES_FORM_TYPE: u32 = 0x0118_3028;
/// The RTTI type descriptor of `TESTexture`.
const TES_TEXTURE_TYPE: u32 = 0x0118_3218;
/// `00480ef0` (cdecl, two words; unit `tesbipedmodelform.cpp` in the
/// engine map).
const BIPED_TEXTURE_PATH: u32 = 0x0048_0ef0;
/// `00408da0` (thiscall): the text of a `BSString` (the engine map's name
/// `MapMarkerData::GetLocationName` is a folded body).
const STRING_TEXT: u32 = 0x0040_8da0;
/// The empty string the lookup answers when nothing is found.
const EMPTY_STRING: u32 = 0x0101_1584;
/// `MAX_PATH`, the size of the destination buffers.
const PATH_SIZE: u32 = 0x104;

/// Copies `source` into `dest` and replaces everything from the last '.'
/// with `suffix`.
fn replace_extension(e: &mut Engine, source: u32, dest: Ptr, suffix: u32) {
    e.call(STRCPY_S, &args![dest, PATH_SIZE, source]);
    let dot = e.call(STRRCHR, &args![dest, 0x2eu32]).u32();
    if dot != 0 {
        let used = dot.wrapping_sub(dest.addr());
        e.call(STRCPY_S, &args![dot, PATH_SIZE.wrapping_sub(used), suffix]);
    }
}

// Translated from 0048e660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the normal-map path of a texture path: copies
/// `source` into `dest` (0x104 bytes) and replaces its extension with
/// `"_n.dds"`.
pub fn fn_0048e660(e: &mut Engine, source: u32, dest: Ptr) {
    replace_extension(e, source, dest, NORMAL_MAP_SUFFIX);
}

// Translated from 0048e6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: like `0048e660` with `"_sk.dds"`.
pub fn fn_0048e6c0(e: &mut Engine, source: u32, dest: Ptr) {
    replace_extension(e, source, dest, SKIN_MAP_SUFFIX);
}

// Translated from 0048e720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the texture folder string at `0101d7fc`
/// (`this` is not read).
pub fn fn_0048e720(_e: &mut Engine, _this: Ptr) -> u32 {
    TEXTURES_FOLDER
}

// Translated from 0048e730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a form's texture path: the text of the
/// `TESTexture` the form is (`__RTDynamicCast` from `TESForm`), or else
/// `00480ef0(form, which)`, or the empty string when that is null.
pub fn fn_0048e730(e: &mut Engine, form: u32, which: u32) -> u32 {
    let texture = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TES_FORM_TYPE, TES_TEXTURE_TYPE, 0u32],
        )
        .u32();
    if texture != 0 {
        return e.call(STRING_TEXT, &args![texture]).u32();
    }
    let path = e.call(BIPED_TEXTURE_PATH, &args![form, which]).u32();
    if path != 0 {
        path
    } else {
        EMPTY_STRING
    }
}

/// `00483710` (thiscall): the base destructor (an empty body).
const BASE_COMPONENT_DESTRUCT: u32 = 0x0048_3710;

// Translated from 0048e7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESValueForm` destructor (the engine map gives it no name): stores
/// the class vtable (`0101d81c`) and runs the base destructor (`00483710`).
pub fn fn_0048e7d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(BASE_COMPONENT_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048e7a0, fn_0048e7a0(Ptr) -> Ptr),
        entry!(0x0048_e660, fn_0048e660(u32, Ptr)),
        entry!(0x0048_e6c0, fn_0048e6c0(u32, Ptr)),
        entry!(0x0048_e720, fn_0048e720(Ptr) -> u32),
        entry!(0x0048_e730, fn_0048e730(u32, u32) -> u32),
        entry!(0x0048_e7d0, fn_0048e7d0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0048e7a0_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0048e7a0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x008c_6060, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0101_d81c)],
        });
    }
    use super::*;

    fn string_doubles(e: &mut Engine) {
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            assert!(text.len() < a[1] as usize);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            let at = text.iter().rposition(|&c| c as u32 == a[1]);
            Ret {
                eax: at.map_or(0, |i| a[0] + i as u32),
                ..Ret::default()
            }
        });
        e.map(NORMAL_MAP_SUFFIX, 0x20);
        e.mem.set_cstr(NORMAL_MAP_SUFFIX, b"_n.dds");
        e.mem.set_cstr(SKIN_MAP_SUFFIX, b"_sk.dds");
    }

    #[test]
    fn fn_0048e660_and_0048e6c0_swap_the_extension() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        let source = e.mem.alloc(0x20);
        e.mem.set_cstr(source, b"a.b/face.dds");
        let dest = e.mem.alloc(0x104);
        e.call(0x0048_e660, &args![source, dest]);
        assert_eq!(e.mem.cstr(dest), b"a.b/face_n.dds".to_vec());
        e.call(0x0048_e6c0, &args![source, dest]);
        assert_eq!(e.mem.cstr(dest), b"a.b/face_sk.dds".to_vec());
        e.mem.set_cstr(source, b"noext");
        e.call(0x0048_e660, &args![source, dest]);
        assert_eq!(e.mem.cstr(dest), b"noext".to_vec());
    }

    #[test]
    fn fn_0048e720_returns_the_textures_folder() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x0048_e720, &args![0x1234u32]).u32(),
            TEXTURES_FOLDER
        );
    }

    #[test]
    fn fn_0048e730_prefers_the_texture_then_the_biped_path() {
        let mut e = Engine::new();
        // A form is a texture when its word at +0 is 1; the biped path is
        // its word at +4.
        e.register(RT_DYNAMIC_CAST, |e, a| Ret {
            eax: if e.mem.u32(a[0]) == 1 { a[0] + 8 } else { 0 },
            ..Ret::default()
        });
        e.register(STRING_TEXT, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        e.register(BIPED_TEXTURE_PATH, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        let form = e.mem.alloc(8);
        e.mem.set_u32(form, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_e730, &args![form, 2u32]).u32(), form + 0x108);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (
                RT_DYNAMIC_CAST,
                vec![form, 0, TES_FORM_TYPE, TES_TEXTURE_TYPE, 0]
            )
        );
        e.mem.set_u32(form, 0);
        assert_eq!(e.call(0x0048_e730, &args![form, 2u32]).u32(), EMPTY_STRING);
        e.mem.set_u32(form + 4, 0x4444);
        assert_eq!(e.call(0x0048_e730, &args![form, 2u32]).u32(), 0x4444);
    }

    #[test]
    fn fn_0048e7d0_restores_the_vtable_then_runs_the_base_destructor() {
        let mut e = Engine::new();
        e.register(BASE_COMPONENT_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0048_e7d0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (BASE_COMPONENT_DESTRUCT, vec![this])
        );
    }
}
