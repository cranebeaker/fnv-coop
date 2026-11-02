//! `BSShader/run_00bb7210` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSDistantTreeShaderProperty`'s methods: its vtable is `010b9c40` (its `GetRTTI`,
//! `fn_00bb7200`, answers the record `012007c8` that the static initializer `00fba6e0` names
//! `"BSDistantTreeShaderProperty"`). On PC it is 0x68 bytes (the allocations here), the Xbox
//! PDB's `uiInstanceCount` and `pInstancesA` 8 bytes lower (+0x60, +0x64) as the base
//! `BSShaderProperty` is 0x60 bytes on PC; `pInstancesA` points to 0x20-byte
//! `BSDistantTreeShader::InstanceData` (Xbox PDB: position +0, `fScale` +0xc, `fzrot1` +0x10,
//! `fzrot2` +0x14).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::NiTArray;
use crate::units::bsshader::bsshaderproperty::BSShaderProperty;

/// The vtable (`010b9c40`) and the RTTI record (`012007c8`).
const DISTANT_TREE_VTABLE: u32 = 0x010b_9c40;
const DISTANT_TREE_RTTI: u32 = 0x0120_07c8;
/// `BSShaderNoLightingProperty::ms_RTTI` (`011fa05c`; the static initializer `00fb99d0` names
/// it), whose name `fn_00bb74e0` adds.
const NO_LIGHTING_RTTI_011FA05C: u32 = 0x011f_a05c;
/// The allocation `00aa13e0` (`cdecl`, one word) and the sized free `00aa1460` (`cdecl`, two
/// words); the memory manager (`011f6238`) and its deallocation (`00aa4060`, `thiscall`, one
/// word).
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const FREE_SIZED_00AA1460: u32 = 0x00aa_1460;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `BSDistantTreeShaderProperty::BSDistantTreeShaderProperty` (engine map, `00bb7170`,
/// `thiscall`, no words), `BSShaderProperty`'s destructor body (`00baa0e0`) and member copy
/// (`00ba8b10`, two words: the clone and the cloning process).
const DISTANT_TREE_CONSTRUCT: u32 = 0x00bb_7170;
const SHADER_PROPERTY_DESTROY: u32 = 0x00ba_a0e0;
const SHADER_PROPERTY_COPY_MEMBERS: u32 = 0x00ba_8b10;
/// The CRT routines `00ec9f70` and `00eca0a0` (the argument and the result in ST0: a leading
/// `f64` and `ST0`; the cosine and sine as other units read them).
const CRT_00EC9F70: u32 = 0x00ec_9f70;
const CRT_00ECA0A0: u32 = 0x00ec_a0a0;
/// `BSShaderProperty::SetAlpha` (`00ba8ab0`, `thiscall`, one float),
/// `CheckCreateRenderPassArray` (`00ba95e0`, one word), the silhouette pass list's make
/// (`00ba9820`, one word) and `RenderPassArray::Add` (`00ba9ee0`, eight words).
const SET_ALPHA: u32 = 0x00ba_8ab0;
const CHECK_CREATE_RENDER_PASS_ARRAY: u32 = 0x00ba_95e0;
const CHECK_CREATE_SILHOUETTE_PASSES_00BA9820: u32 = 0x00ba_9820;
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
/// `BSShaderProperty::GetViewerStrings` (`00ba9890`, `thiscall`, one word), `NiGetViewerString`
/// (`00a7d330`, `cdecl`) and the string array's grow (`0096ad30`) and set (`0096ae90`).
const SHADER_PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00ba_9890;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const ARRAY_SET_SIZE_0096AD30: u32 = 0x0096_ad30;
const ARRAY_SET_AT_0096AE90: u32 = 0x0096_ae90;

/// 0x68 bytes from `00aa13e0` constructed as a `BSDistantTreeShaderProperty` (`00bb7170`), or
/// null when the allocation gave none.
fn new_distant_tree(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE_00AA13E0, &args![0x68u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(DISTANT_TREE_CONSTRUCT, &args![block]).u32()
}

// Translated from 00bb7200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: `GetRTTI` (vtable slot +8): answers the RTTI record
/// `012007c8`.
pub fn fn_00bb7200(_e: &mut Engine, _this: Ptr) -> u32 {
    DISTANT_TREE_RTTI
}

// Translated from 00bb7210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable slot +0x48 (`CreateClone` in the Xbox PDB's slot list;
/// `thiscall`, one word: the cloning process). A new distant-tree property (null when the
/// allocation gave none) gets `BSShaderProperty`'s members (`00ba8b10`). Answers the clone.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00bb7210 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00bb7210(e: &mut Engine, this: Ptr, process: u32) -> u32 {
    let clone = new_distant_tree(e);
    e.call(SHADER_PROPERTY_COPY_MEMBERS, &args![this, clone, process]);
    clone
}

// Translated from 00bb7290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, six words: an instance index, the position, an angle
/// and a scale; `SetupShaderProperty` `006f9142` calls it): fills instance `index` of
/// `pInstancesA` (`+0x64`): the position, the scale (`+0xc`), and the angle through
/// `00ec9f70` (`+0x10`) and `00eca0a0` (`+0x14`), each stored as a `float` (`FLD`/`FSTP`).
pub fn fn_00bb7290(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    x: f32,
    y: f32,
    z: f32,
    angle: f32,
    scale: f32,
) {
    let base = this.addr();
    let offset = index << 5;
    let put = |e: &mut Engine, field: u32, bits: u32| {
        let instances = e.mem.u32(base + 0x64);
        e.mem
            .set_u32(instances.wrapping_add(offset).wrapping_add(field), bits);
    };
    put(e, 0, x87_quiet_bits(x.to_bits()));
    put(e, 4, x87_quiet_bits(y.to_bits()));
    put(e, 8, x87_quiet_bits(z.to_bits()));
    put(e, 0xc, x87_quiet_bits(scale.to_bits()));
    let first = e
        .call(CRT_00EC9F70, &args![x87_load32(angle.to_bits())])
        .f64();
    put(e, 0x10, x87_quiet_bits(x87_store32(first)));
    let second = e
        .call(CRT_00ECA0A0, &args![x87_load32(angle.to_bits())])
        .f64();
    put(e, 0x14, x87_quiet_bits(x87_store32(second)));
}

// Translated from 00bb7300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra's library match `CreateObject` is the decompiler's): the
/// create function `RegisterStreamables` (`00b4f2c3`) records for
/// `BSDistantTreeShaderProperty` (`cdecl`, no arguments): a new property, or null when the
/// allocation gave none.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00bb7300 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00bb7300(e: &mut Engine) -> u32 {
    new_distant_tree(e)
}

// Translated from 00bb7370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor (slot 0 of `010b9c40`;
/// `thiscall`, one word: the flags). The vtable `010b9c40`; `pInstancesA` given back to the
/// memory manager (null or not); `BSShaderProperty`'s destructor body; with bit 0 of the flags
/// the 0x68 bytes freed (`00aa1460`). Answers `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00bb7370 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00bb7370(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, DISTANT_TREE_VTABLE);
    let instances = e.mem.u32(base + 0x64);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, instances]);
    e.call(SHADER_PROPERTY_DESTROY, &args![base]);
    if flags & 1 != 0 {
        e.call(FREE_SIZED_00AA1460, &args![base, 0x68u32]);
    }
    this
}

// Translated from 00bb73f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable slot +0xa0 (`GetRenderPasses` in the Xbox PDB's slot list;
/// `thiscall`, six words: the geometry, a render-state word, an unread word, a state word that
/// goes above it in the key, and two unread words). Null for state 7. Unless the geometry is
/// at -0x9c (the code tests the address of its `+0x9c`), the alpha is set (`SetAlpha`) to
/// `fFadeAlpha` (`+0x2c`), times the float `+0x3c` of the geometry's `+0xa4` object when there
/// is one. For state 0xf the silhouette list (`+0x54`, made by `00ba9820` with 1) gets pass
/// 0x237 when empty (its first pass's byte `+6` cleared) and is answered. Otherwise, when the
/// key (`state << 8 | word`) differs from `iLastRenderPassState`, the pass list is made
/// (`CheckCreateRenderPassArray` with 1), emptied and given pass 0x236 (its first pass's byte
/// `+6` set to 10), and the key kept; the pass list is answered.
pub fn fn_00bb73f0(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    state_word: u32,
    _unused_2: u32,
    state: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let key = (state << 8) | state_word;
    if state == 7 {
        return 0;
    }
    if geometry.wrapping_add(0x9c) != 0 {
        let modifier = e.mem.u32(geometry.wrapping_add(0xa4));
        let fade = e.get(this, BSShaderProperty::fFadeAlpha);
        let alpha = if modifier != 0 {
            x87_mul(x87_load32(e.mem.u32(modifier + 0x3c)), x87_m32(fade))
        } else {
            x87_load32(fade)
        };
        let alpha = x87_quiet_bits(x87_store32(alpha));
        e.call(SET_ALPHA, &args![this, alpha]);
    }
    if state == 0xf {
        e.call(CHECK_CREATE_SILHOUETTE_PASSES_00BA9820, &args![this, 1u32]);
        let list = e.get(this, BSShaderProperty::pSilhouettePassList);
        if e.mem.u32(list.wrapping_add(0x10)) == 0 {
            e.call(
                RENDER_PASS_ARRAY_ADD,
                &args![list, geometry, 0x237u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
            );
            let list = e.get(this, BSShaderProperty::pSilhouettePassList);
            let first = e.mem.u32(e.mem.u32(list.wrapping_add(4)));
            e.mem.set_u8(first.wrapping_add(6), 0);
        }
        return e.get(this, BSShaderProperty::pSilhouettePassList);
    }
    if e.get(this, BSShaderProperty::iLastRenderPassState) as u32 != key {
        e.call(CHECK_CREATE_RENDER_PASS_ARRAY, &args![this, 1u32]);
        let list = e.get(this, BSShaderProperty::pRenderPassList);
        e.mem.set_u32(list.wrapping_add(0x10), 0);
        let list = e.get(this, BSShaderProperty::pRenderPassList);
        e.call(
            RENDER_PASS_ARRAY_ADD,
            &args![list, geometry, 0x236u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        let list = e.get(this, BSShaderProperty::pRenderPassList);
        let first = e.mem.u32(e.mem.u32(list.wrapping_add(4)));
        e.mem.set_u8(first.wrapping_add(6), 10);
        e.set(this, BSShaderProperty::iLastRenderPassState, key as i32);
    }
    e.get(this, BSShaderProperty::pRenderPassList)
}

// Translated from 00bb74e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable slot +0x60 (`GetViewerStrings` in the Xbox PDB's slot
/// list; `thiscall`, one word: the string array). `BSShaderProperty::GetViewerStrings` runs;
/// then the viewer string of the name in `BSShaderNoLightingProperty::ms_RTTI` (`011fa05c`,
/// as the code reads it) is added at the array's end (grown by `m_usGrowBy` first when full).
pub fn fn_00bb74e0(e: &mut Engine, this: Ptr, strings: u32) {
    e.call(SHADER_PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(NO_LIGHTING_RTTI_011FA05C);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    let array: Ptr<NiTArray> = Ptr::new(strings);
    let size = u32::from(e.get(array, NiTArray::m_usSize));
    let capacity = u32::from(e.get(array, NiTArray::m_usMaxSize));
    e.with_stack(4, |e, element| {
        e.mem.set_u32(element.addr(), text);
        if size >= capacity {
            let grow = u32::from(e.get(array, NiTArray::m_usGrowBy));
            e.call(
                ARRAY_SET_SIZE_0096AD30,
                &args![strings, grow.wrapping_add(size)],
            );
        }
        e.call(ARRAY_SET_AT_0096AE90, &args![strings, size, element]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bb_7200, fn_00bb7200(Ptr) -> u32),
        entry!(0x00bb_7210, fn_00bb7210(Ptr, u32) -> u32),
        entry!(0x00bb_7290, fn_00bb7290(Ptr, u32, f32, f32, f32, f32, f32)),
        entry!(0x00bb_7300, fn_00bb7300() -> u32),
        entry!(0x00bb_7370, fn_00bb7370(Ptr, u8) -> Ptr),
        entry!(
            0x00bb_73f0,
            fn_00bb73f0(Ptr<BSShaderProperty>, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00bb_74e0, fn_00bb74e0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const STUB: AbiFn = |_, _| Ret::default();

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(DISTANT_TREE_CONSTRUCT, |_, a| a[0].into_ret());
        for quiet in [
            SHADER_PROPERTY_COPY_MEMBERS,
            SHADER_PROPERTY_DESTROY,
            FREE_SIZED_00AA1460,
            MEMORY_DEALLOCATE,
            SET_ALPHA,
            CHECK_CREATE_RENDER_PASS_ARRAY,
            CHECK_CREATE_SILHOUETTE_PASSES_00BA9820,
        ] {
            e.register(quiet, STUB);
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn rtti_clone_and_create() {
        let mut e = rig();
        assert_eq!(e.call(0x00bb_7200, &args![0u32]).u32(), 0x0120_07c8);
        let clone = e.call(0x00bb_7210, &args![0x900u32, 0x55u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(calls(&e, ALLOCATE_00AA13E0), vec![vec![0x68]]);
        assert_eq!(calls(&e, DISTANT_TREE_CONSTRUCT), vec![vec![clone]]);
        assert_eq!(
            calls(&e, SHADER_PROPERTY_COPY_MEMBERS),
            vec![vec![0x900, clone, 0x55]]
        );
        let made = e.call(0x00bb_7300, &args![]).u32();
        assert_ne!(made, 0);
        e.register(ALLOCATE_00AA13E0, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00bb_7300, &args![]).u32(), 0);
        assert_eq!(calls(&e, DISTANT_TREE_CONSTRUCT).len(), 2);
    }

    #[test]
    fn fn_00bb7290_fills_the_instance() {
        let mut e = rig();
        e.register(CRT_00EC9F70, |_, a| Ret {
            st0: f64::take(a, &mut 0) + 1.0,
            ..Ret::default()
        });
        e.register(CRT_00ECA0A0, |_, a| Ret {
            st0: f64::take(a, &mut 0) - 1.0,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x68);
        let instances = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x64, instances);
        e.call(
            0x00bb_7290,
            &args![this, 1u32, 1.0f32, 2.0f32, 3.0f32, 0.5f32, 4.0f32],
        );
        let words: Vec<f32> = (0..6)
            .map(|i| e.mem.f32(instances + 0x20 + 4 * i))
            .collect();
        assert_eq!(words, vec![1.0, 2.0, 3.0, 4.0, 1.5, -0.5]);
    }

    #[test]
    fn fn_00bb7370_frees_the_instances_and_itself_on_bit_0() {
        let mut e = rig();
        let this = e.mem.alloc(0x68);
        e.mem.set_u32(this + 0x64, 0x4000);
        assert_eq!(e.call(0x00bb_7370, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), DISTANT_TREE_VTABLE);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x4000]]
        );
        assert_eq!(calls(&e, SHADER_PROPERTY_DESTROY), vec![vec![this]]);
        assert!(calls(&e, FREE_SIZED_00AA1460).is_empty());
        e.call(0x00bb_7370, &args![this, 1u32]);
        assert_eq!(calls(&e, FREE_SIZED_00AA1460), vec![vec![this, 0x68]]);
    }

    #[test]
    fn render_passes_by_state() {
        let mut e = rig();
        let first_pass = e.mem.alloc(8);
        e.register_double(RENDER_PASS_ARRAY_ADD, move |e, a| {
            let array = e.mem.u32(a[0] + 4);
            e.mem.set_u32(array, first_pass);
            let size = e.mem.u32(a[0] + 0x10);
            e.mem.set_u32(a[0] + 0x10, size + 1);
            Ret::default()
        });
        let this = e.new_object::<BSShaderProperty>();
        let list = e.mem.alloc(0x14);
        let passes = e.mem.alloc(4);
        e.mem.set_u32(list + 4, passes);
        let silhouette = e.mem.alloc(0x14);
        let silhouette_passes = e.mem.alloc(4);
        e.mem.set_u32(silhouette + 4, silhouette_passes);
        e.set(this, BSShaderProperty::pRenderPassList, list);
        e.set(this, BSShaderProperty::pSilhouettePassList, silhouette);
        e.set(this, BSShaderProperty::fFadeAlpha, 0.5f32.to_bits());
        let modifier = e.mem.alloc(0x40);
        e.mem.set_f32(modifier + 0x3c, 0.5);
        let geometry = e.mem.alloc(0xa8);
        e.mem.set_u32(geometry + 0xa4, modifier);
        // State 7: nothing.
        let r = e.call(
            0x00bb_73f0,
            &args![this, geometry, 1u32, 0u32, 7u32, 0u32, 0u32],
        );
        assert_eq!(r.u32(), 0);
        assert!(calls(&e, SET_ALPHA).is_empty());
        // State 0xf: the silhouette pass.
        let r = e.call(
            0x00bb_73f0,
            &args![this, geometry, 1u32, 0u32, 0xfu32, 0u32, 0u32],
        );
        assert_eq!(r.u32(), silhouette);
        assert_eq!(
            calls(&e, SET_ALPHA),
            vec![vec![this.addr(), 0.25f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, RENDER_PASS_ARRAY_ADD)[0][..3],
            [silhouette, geometry, 0x237]
        );
        assert_eq!(e.mem.u8(first_pass + 6), 0);
        // Another state: the main pass for a new key.
        e.mem.set_u32(geometry + 0xa4, 0);
        let r = e.call(
            0x00bb_73f0,
            &args![this, geometry, 1u32, 0u32, 2u32, 0u32, 0u32],
        );
        assert_eq!(r.u32(), list);
        assert_eq!(calls(&e, SET_ALPHA)[1], vec![this.addr(), 0.5f32.to_bits()]);
        assert_eq!(
            calls(&e, RENDER_PASS_ARRAY_ADD)[1][..3],
            [list, geometry, 0x236]
        );
        assert_eq!(e.mem.u8(first_pass + 6), 10);
        assert_eq!(e.get(this, BSShaderProperty::iLastRenderPassState), 0x201);
        // The same key again: no pass added.
        e.call(
            0x00bb_73f0,
            &args![this, geometry, 1u32, 0u32, 2u32, 0u32, 0u32],
        );
        assert_eq!(calls(&e, RENDER_PASS_ARRAY_ADD).len(), 2);
    }

    #[test]
    fn fn_00bb74e0_adds_the_name() {
        let mut e = rig();
        e.map(0x011f_a000, 0x1000);
        e.set_global(NO_LIGHTING_RTTI_011FA05C, 0x0102_0000u32);
        e.register(SHADER_PROPERTY_GET_VIEWER_STRINGS, STUB);
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(ARRAY_SET_SIZE_0096AD30, STUB);
        e.register(ARRAY_SET_AT_0096AE90, STUB);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xa, 1);
        e.call(0x00bb_74e0, &args![0x900u32, strings]);
        assert_eq!(
            calls(&e, SHADER_PROPERTY_GET_VIEWER_STRINGS),
            vec![vec![0x900, strings]]
        );
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0102_0000]]);
        assert!(calls(&e, ARRAY_SET_SIZE_0096AD30).is_empty());
        assert_eq!(calls(&e, ARRAY_SET_AT_0096AE90)[0][..2], [strings, 1]);
    }
}
