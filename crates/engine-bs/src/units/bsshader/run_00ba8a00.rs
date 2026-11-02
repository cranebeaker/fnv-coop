//! `BSShader/run_00ba8a00` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The allocation `00aa13e0` (`cdecl`, one word: the size).
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
/// `BSShaderTextureSet::BSShaderTextureSet` (engine map, `0051d0d0`, `thiscall`; answers
/// `this`).
const BS_SHADER_TEXTURE_SET_CONSTRUCTOR: u32 = 0x0051_d0d0;
/// The base `LinkObject` `00a950c0` (engine map: `NiAlphaProperty::LinkObject`, shared code).
const BASE_LINK_OBJECT_00A950C0: u32 = 0x00a9_50c0;

// Translated from 00ba8a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The stream's creation function for `"BSShaderTextureSet"` (the shader manager's
/// registration table pairs the name at `010a9284` with this address): allocates 0x38 bytes
/// (`00aa13e0`) and constructs a `BSShaderTextureSet` in them (`0051d0d0`), answering what
/// the constructor answers; 0 when the allocation fails.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba8a00 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn bs_shader_texture_set_create_object(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE_00AA13E0, &args![0x38u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(BS_SHADER_TEXTURE_SET_CONSTRUCTOR, &args![block])
        .u32()
}

// Translated from 00ba8a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A jump to `00a950c0` (a `LinkObject`; vtable slot +0x50 of the shader-property vtables,
/// reached from `00b6f460`): `thiscall`, one word (the stream), passed on unchanged.
pub fn fn_00ba8a70(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(BASE_LINK_OBJECT_00A950C0, &args![this, stream]);
}

// Translated from 00ba8aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot +0x94 (`CopyTo` in the Xbox PDB's slot list) of the shader-property vtables:
/// a jump to the object's own slot +0x98 (`CopyToMembers`) with the same word (the
/// destination).
pub fn fn_00ba8aa0(e: &mut Engine, this: Ptr, dest: u32) {
    e.vcall(this.addr(), 0x98, &args![dest]);
}

/// Where `fn_00ba8a80` jumps (`00a99e40`).
const REGISTER_STREAMABLES_00A99E40: u32 = 0x00a9_9e40;

// Translated from 00ba8a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a jump to `00a99e40` (vtable slot +0x54 of five shader-property
/// vtables, `RegisterStreamables` in the Xbox PDB's slot list, and the target of `00bc3770`):
/// `thiscall`, one word (the stream), passed on unchanged; answers the target's `EAX`.
pub fn fn_00ba8a80(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    e.call(REGISTER_STREAMABLES_00A99E40, &args![this, stream])
        .u32()
}

// Translated from 00ba8a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a vtable slot (+0xa0 of `DistantLODShaderProperty`'s vtable
/// `010adad0` and six others, `GetRenderPasses` in the Xbox PDB's slot list; `thiscall`, six
/// words it does not read): answers 0 (`XOR EAX,EAX; RET 0x18`).
pub fn fn_00ba8a90(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_8a80, fn_00ba8a80(Ptr, u32) -> u32),
        entry!(
            0x00ba_8a90,
            fn_00ba8a90(Ptr, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00ba_8a00, bs_shader_texture_set_create_object() -> u32),
        entry!(0x00ba_8a70, fn_00ba8a70(Ptr, u32)),
        entry!(0x00ba_8aa0, fn_00ba8aa0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn create_object_constructs_in_a_new_block() {
        let mut e = Engine::new();
        e.register(ALLOCATE_00AA13E0, |e, _| e.mem.u32(0x0f00_0000).into_ret());
        e.register(BS_SHADER_TEXTURE_SET_CONSTRUCTOR, |_, a| {
            (a[0] + 1).into_ret()
        });
        e.map(0x0f00_0000, 0x1000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ba_8a00, &[]).u32(), 0);
        assert!(calls(&e, BS_SHADER_TEXTURE_SET_CONSTRUCTOR).is_empty());
        e.mem.set_u32(0x0f00_0000, 0x4000);
        assert_eq!(e.call(0x00ba_8a00, &[]).u32(), 0x4001);
        assert_eq!(calls(&e, ALLOCATE_00AA13E0), vec![vec![0x38], vec![0x38]]);
        assert_eq!(
            calls(&e, BS_SHADER_TEXTURE_SET_CONSTRUCTOR),
            vec![vec![0x4000]]
        );
    }

    #[test]
    fn fn_00ba8a70_passes_on_to_the_base_link() {
        let mut e = Engine::new();
        e.register(BASE_LINK_OBJECT_00A950C0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ba_8a70, &args![0x5000u32, 0x6000u32]);
        assert_eq!(
            calls(&e, BASE_LINK_OBJECT_00A950C0),
            vec![vec![0x5000, 0x6000]]
        );
    }

    #[test]
    fn fn_00ba8aa0_calls_slot_0x98() {
        let mut e = Engine::new();
        let mut slots = [0u32; 0x27];
        slots[0x98 / 4] = 0x0f00_0098;
        e.put_vtable(0x0f00_1000, &slots);
        e.register(0x0f00_0098, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x0f00_1000);
        e.call_log = Some(vec![]);
        e.call(0x00ba_8aa0, &args![this, 0x6000u32]);
        assert_eq!(calls(&e, 0x0f00_0098), vec![vec![this, 0x6000]]);
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    #[test]
    fn fn_00ba8a80_jumps_on() {
        let mut e = Engine::new();
        e.register(REGISTER_STREAMABLES_00A99E40, |_, a| (a[1] + 1).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ba_8a80, &args![0x10u32, 0x20u32]).u32(), 0x21);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(REGISTER_STREAMABLES_00A99E40, vec![0x10, 0x20])));
    }

    #[test]
    fn fn_00ba8a90_answers_zero() {
        let mut e = Engine::new();
        let r = e.call(
            0x00ba_8a90,
            &args![1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32],
        );
        assert_eq!(r.u32(), 0);
    }
}
