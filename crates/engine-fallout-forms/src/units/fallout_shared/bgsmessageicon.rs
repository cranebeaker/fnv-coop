//! `fallout shared/bgsmessageicon.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_0047a300` calls on the member (`00408da0`).
const CALLEE_00408DA0: u32 = 0x0040_8da0;

// Translated from 0047a300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00408da0` with `ECX` = `this + 0x4` and passes on its result.
pub fn fn_0047a300(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_00408DA0, &args![this.byte_add(0x4)]).u32()
}

// Translated from 0047a200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMessageIcon::InitializeDataComponent` (Xbox PDB): runs the
/// `InitializeDataComponent` (vtable slot 0) of the icon's texture
/// component at `this + 4` and passes on its result.
pub fn bgs_message_icon_initialize_data_component(e: &mut Engine, this: Ptr) -> u32 {
    e.vcall(this.addr() + 4, 0, &[]).u32()
}

// Translated from 0047a220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMessageIcon::ClearDataComponent` (Xbox PDB): runs the
/// `ClearDataComponent` (vtable slot 4) of the texture component at
/// `this + 4` and passes on its result.
pub fn bgs_message_icon_clear_data_component(e: &mut Engine, this: Ptr) -> u32 {
    e.vcall(this.addr() + 4, 4, &[]).u32()
}

// Translated from 0047a290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` (icon, file): loads the icon's texture chunk:
/// `TESTexture::LoadTextureChunk` (`0048e3d0`) on the texture `00717e50`
/// returns for `icon` (the engine map's name for `00717e50` is
/// `BaseProcess::GetActorPackageThatIsRunning`, a folded body that returns
/// `this + 4`), with `file`.
pub fn fn_0047a290(e: &mut Engine, icon: Ptr, file: Ptr) {
    let texture = e.call(0x0071_7e50, &args![icon]).u32();
    e.call(0x0048_e3d0, &args![texture, file]);
}

/// `0048cee0` (`ECX` = texture): the length of its file name string (`004048e0` on the
/// string at +4).
const TEXTURE_NAME_LENGTH: u32 = 0x0048_cee0;
/// `TESForm::AddChunkArray` (`004856f0`; `cdecl`: chunk signature, address, size).
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `BaseFormComponent` and `BGSMessageIcon` type descriptors (`01183040`,
/// `01183234`).
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_BGS_MESSAGE_ICON: u32 = 0x0118_3234;
/// `0047a380` (`ECX` = icon, `RET 4`): sets the texture's file name (`00489100` on
/// `icon + 4`).
const SET_ICON_NAME: u32 = 0x0047_a380;
/// `00404dc0` (`cdecl`, two strings): their comparison (`00ec68e4`), 0 when equal.
const COMPARE_STRINGS: u32 = 0x0040_4dc0;

/// `__RTDynamicCast` of `other` from `BaseFormComponent` to `BGSMessageIcon`.
fn cast_to_message_icon(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![
            other,
            0u32,
            TYPE_BASE_FORM_COMPONENT,
            TYPE_BGS_MESSAGE_ICON,
            0u32
        ],
    )
    .u32()
}

// Translated from 0047a240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMessageIcon::Save` (Xbox PDB; `RET 4`, the chunk signature): when the icon's
/// texture (`this + 4`) has a file name (`0048cee0` non-zero), saves the name with its
/// terminating zero (length + 1 bytes, text from `00408da0`) as a chunk (`004856f0`).
pub fn bgs_message_icon_save(e: &mut Engine, this: Ptr, chunk: u32) {
    let texture = this.addr() + 4;
    if e.call(TEXTURE_NAME_LENGTH, &args![texture]).u32() == 0 {
        return;
    }
    let size = e
        .call(TEXTURE_NAME_LENGTH, &args![texture])
        .u32()
        .wrapping_add(1);
    let text = e.call(CALLEE_00408DA0, &args![texture]).u32();
    e.call(ADD_CHUNK_ARRAY, &args![chunk, text, size]);
}

// Translated from 0047a2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMessageIcon::CopyComponent` (Xbox PDB; `RET 4`): casts `other` to a
/// `BGSMessageIcon` and, when it is one, sets this icon's file name (`0047a380`) to the
/// other's (`0047a300`).
pub fn bgs_message_icon_copy_component(e: &mut Engine, this: Ptr, other: Ptr) {
    let cast = cast_to_message_icon(e, other);
    if cast == 0 {
        return;
    }
    let name = fn_0047a300(e, Ptr::new(cast));
    e.call(SET_ICON_NAME, &args![this, name]);
}

// Translated from 0047a320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMessageIcon::CompareComponent` (Xbox PDB; `RET 4`, result in `AL`): true
/// ("differs") when `other` is not a `BGSMessageIcon` or the string comparison
/// (`00404dc0`, this icon's name first) of the two file names (`0047a300`) is non-zero.
pub fn bgs_message_icon_compare_component(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let cast = cast_to_message_icon(e, other);
    if cast == 0 {
        return true;
    }
    let theirs = fn_0047a300(e, Ptr::new(cast));
    let ours = fn_0047a300(e, this);
    e.call(COMPARE_STRINGS, &args![ours, theirs]).u32() != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_a300, fn_0047a300(Ptr) -> u32),
        entry!(0x0047_a200, bgs_message_icon_initialize_data_component(Ptr) -> u32),
        entry!(0x0047_a220, bgs_message_icon_clear_data_component(Ptr) -> u32),
        entry!(0x0047_a290, fn_0047a290(Ptr, Ptr)),
        entry!(0x0047_a240, bgs_message_icon_save(Ptr, u32)),
        entry!(0x0047_a2b0, bgs_message_icon_copy_component(Ptr, Ptr)),
        entry!(0x0047_a320, bgs_message_icon_compare_component(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_0047a300_passes_this_plus_4_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00408DA0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_a300, &args![0x4100u32]).u32(), 0x1451);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00408DA0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x4]);
    }
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != skip)
            .collect()
    }

    fn icon(e: &mut Engine) -> Ptr {
        e.put_vtable(0x0150_0000, &[0x0000_1000, 0x0000_1004]);
        e.register(0x0000_1000, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(0x0000_1004, |_, a| Ret {
            eax: a[0] + 2,
            ..Ret::default()
        });
        let p = e.mem.alloc(0x10);
        e.mem.set_u32(p + 4, 0x0150_0000);
        Ptr::new(p)
    }

    #[test]
    fn b020_0047a200_initializes_the_texture_component() {
        let mut e = Engine::new();
        let p = icon(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_a200, &args![p]).u32(), p.addr() + 5);
        assert_eq!(
            calls(&mut e, 0x0047_a200),
            vec![(0x0000_1000, vec![p.addr() + 4])]
        );
    }

    #[test]
    fn b020_0047a220_clears_the_texture_component() {
        let mut e = Engine::new();
        let p = icon(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_a220, &args![p]).u32(), p.addr() + 6);
        assert_eq!(
            calls(&mut e, 0x0047_a220),
            vec![(0x0000_1004, vec![p.addr() + 4])]
        );
    }

    #[test]
    fn b020_0047a290_loads_the_texture_chunk() {
        let mut e = Engine::new();
        e.register(0x0071_7e50, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.register(0x0048_e3d0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_a290, &args![0x5000u32, 0x9000u32]);
        assert_eq!(
            calls(&mut e, 0x0047_a290),
            vec![
                (0x0071_7e50, vec![0x5000]),
                (0x0048_e3d0, vec![0x5004, 0x9000])
            ]
        );
    }
}

#[cfg(test)]
mod tests_n036 {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn save_writes_the_name_with_its_terminator_only_when_set() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(TEXTURE_NAME_LENGTH, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(CALLEE_00408DA0, |_, a| (a[0] | 0x0900_0000).into_ret());
        e.register(ADD_CHUNK_ARRAY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_a240, &args![this, 0x4e4f_4349u32]);
        assert!(calls(&e, ADD_CHUNK_ARRAY).is_empty());
        e.mem.set_u32(this + 8, 5);
        e.call_log = Some(vec![]);
        e.call(0x0047_a240, &args![this, 0x4e4f_4349u32]);
        assert_eq!(calls(&e, TEXTURE_NAME_LENGTH), vec![vec![this + 4]; 2]);
        assert_eq!(
            calls(&e, ADD_CHUNK_ARRAY),
            vec![vec![0x4e4f_4349, (this + 4) | 0x0900_0000, 6]]
        );
    }

    #[test]
    fn copy_component_sets_the_other_name() {
        let mut e = Engine::new();
        e.register(CALLEE_00408DA0, |_, a| (a[0] + 0x100).into_ret());
        e.register(SET_ICON_NAME, |_, _| Ret::default());
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0047_a2b0, &args![0x5000u32, 0x6000u32]);
        assert!(calls(&e, SET_ICON_NAME).is_empty());
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![
                0x6000,
                0,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_BGS_MESSAGE_ICON,
                0
            ]]
        );
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0047_a2b0, &args![0x5000u32, 0x6000u32]);
        assert_eq!(calls(&e, SET_ICON_NAME), vec![vec![0x5000, 0x6104]]);
    }

    #[test]
    fn compare_component_compares_the_names() {
        let mut e = Engine::new();
        e.register(CALLEE_00408DA0, |_, a| (a[0] + 0x100).into_ret());
        e.register(COMPARE_STRINGS, |_, a| {
            u32::from(a[0] & 0xfff != a[1] & 0xfff).into_ret()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_a320, &args![0x5000u32, 0x6100u32]).bool());
        assert_eq!(calls(&e, COMPARE_STRINGS), vec![vec![0x5104, 0x6204]]);
        assert!(!e.call(0x0047_a320, &args![0x5000u32, 0x6000u32]).bool());
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0047_a320, &args![0x5000u32, 0x6000u32]).bool());
    }
}
