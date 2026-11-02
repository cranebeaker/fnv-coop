//! `BSShader/run_00b6b330` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    construct_pointer_array, ni_pointer_assign, ni_pointer_clear, release_reference,
    DESTRUCT_00B66F20,
};

/// `NiObject`'s constructor (`00a5d3a0`, unit `niobject.cpp`; `thiscall`, no stack words) and
/// the object's vtable (`010ae500`; no RTTI locator before it).
const NI_OBJECT_CONSTRUCT_00A5D3A0: u32 = 0x00a5_d3a0;
const VTABLE_010AE500: u32 = 0x010a_e500;
/// The destructor the four-pointer array at +0x30 passes (`0045cec0`).
const DESTRUCT_0045CEC0: u32 = 0x0045_cec0;
/// The value maker `00b6b180` (unnamed; `cdecl`: the texture and the third argument) and the
/// render target maker `00ee8760` (unit `nirendertargetgroup.cpp`, unnamed; `cdecl`, three
/// words: a buffer, the word at `011f4748`, the value).
const VALUE_00B6B180: u32 = 0x00b6_b180;
const RENDER_TARGET_00EE8760: u32 = 0x00ee_8760;
const GLOBAL_011F4748: u32 = 0x011f_4748;
/// `NiRenderedCubeMap::ms_RTTI` (`012708e4`: its static initializer `00fc7950` gives it the
/// name `"NiRenderedCubeMap"`).
const RENDERED_CUBE_MAP_RTTI_012708E4: u32 = 0x0127_08e4;

// Translated from 00b6b380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6b330`): the constructor of a 0x40-byte object over a
/// texture, `thiscall` with three stack words: the texture, a byte and a word. Runs
/// `NiObject`'s constructor, stores the vtable `010ae500`, constructs the six `NiPointer`s at
/// +8 (destructor `00b66f20`), clears +0x20 to +0x28, constructs the four `NiPointer`s at
/// +0x30 (destructor `0045cec0`) and assigns the texture to the first of them. The value is
/// `00b6b180(texture, word)` when the byte is set, else 0. When the texture's class
/// (`GetRTTI`, vtable slot +8, then each base through the RTTI's +4) is `NiRenderedCubeMap`:
/// each of the six pointers at +8 gets `00ee8760` of the texture's word at +0x4c + 4 i, the
/// word at `011f4748` and the value. Otherwise (also for a null texture, whose vtable the code
/// still reads) the first gets `00ee8760` of what the texture's vtable slot +0xa4 answers (the
/// word at `011f4748` read first) and the other five are cleared. Then the pointers at +0x20,
/// +0x24 and +0x28 are cleared, +0x2c is set to -1 and `this` is answered.
// Translation incomplete: 00b6b380 - the C++ exception frame (FS:[0] chain, unwind states 0 to 5 for the base and the arrays) is not translated.
pub fn fn_00b6b380(e: &mut Engine, this: Ptr, texture: Ptr, make_value: u8, word: u32) -> Ptr {
    let base = this.addr();
    let texture = texture.addr();
    e.call(NI_OBJECT_CONSTRUCT_00A5D3A0, &args![base]);
    e.mem.set_u32(base, VTABLE_010AE500);
    construct_pointer_array(e, base + 8, 6, DESTRUCT_00B66F20);
    for offset in [0x20u32, 0x24, 0x28] {
        e.mem.set_u32(base + offset, 0);
    }
    construct_pointer_array(e, base + 0x30, 4, DESTRUCT_0045CEC0);
    ni_pointer_assign(e, base + 0x30, texture);
    let value = if make_value != 0 {
        e.call(VALUE_00B6B180, &args![texture, word]).u32()
    } else {
        0
    };
    let mut cube_map = false;
    if texture != 0 {
        let mut rtti = e.vcall(texture, 8, &args![]).u32();
        while rtti != 0 {
            if rtti == RENDERED_CUBE_MAP_RTTI_012708E4 {
                cube_map = true;
                break;
            }
            rtti = e.mem.u32(rtti + 4);
        }
    }
    if cube_map {
        for index in 0..6u32 {
            let buffer = e.mem.u32(texture + 0x4c + 4 * index);
            let renderer = e.global::<u32>(GLOBAL_011F4748);
            let target = e
                .call(RENDER_TARGET_00EE8760, &args![buffer, renderer, value])
                .u32();
            ni_pointer_assign(e, base + 8 + 4 * index, target);
        }
    } else {
        let renderer = e.global::<u32>(GLOBAL_011F4748);
        let buffer = e.vcall(texture, 0xa4, &args![]).u32();
        let target = e
            .call(RENDER_TARGET_00EE8760, &args![buffer, renderer, value])
            .u32();
        ni_pointer_assign(e, base + 8, target);
        for index in 1..6u32 {
            ni_pointer_clear(e, base + 8 + 4 * index);
        }
    }
    for offset in [0x20u32, 0x24, 0x28] {
        ni_pointer_clear(e, base + offset);
    }
    e.mem.set_u32(base + 0x2c, 0xffff_ffff);
    this
}

/// Imported `LeaveCriticalSection` (`kernel32`, `stdcall`: the section).
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

// Translated from 00b6b330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6b330`), `thiscall` on the renderer, no stack words: when
/// its mode (+0x200) is 1 or 2, its byte +0x208 is 1 and its slot +0x198 answers true, leaves
/// the critical section at +0x80, clears the byte and answers true; else false (only `AL` is
/// set).
pub fn fn_00b6b330(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let mode = e.mem.u32(base + 0x200);
    if (mode == 1 || mode == 2)
        && e.mem.u8(base + 0x208) == 1
        && e.vcall(base, 0x198, &args![]).bool()
    {
        e.call(IMPORT_LEAVE_CRITICAL_SECTION, &args![base + 0x80]);
        e.mem.set_u8(base + 0x208, 0);
        return true;
    }
    false
}

/// The Direct3D renderer (`011f9508`), the rendered-texture maker `00a7fc00` (unit
/// `nirenderedtexture.cpp`, unnamed; `cdecl`, eight words) and `operator new` `00aa13e0`
/// (unnamed, `cdecl`, the size).
const D3D_RENDERER_011F9508: u32 = 0x011f_9508;
const MAKE_RENDERED_TEXTURE_00A7FC00: u32 = 0x00a7_fc00;
const OPERATOR_NEW_00AA13E0: u32 = 0x00aa_13e0;

// Translated from 00b6b610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b6b610 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
/// Unnamed in the engine map (`run_00b6b330`), `cdecl`, nine words: makes a rendered texture
/// (`00a7fc00` with the first three words, the renderer `011f9508`, the fourth and fifth and
/// the eighth and ninth words); with one, a 0x40-byte object (`operator new`; nothing when
/// that fails) built over it by `00b6b380` with the sixth word (its low byte) and the seventh,
/// which is answered; else 0.
pub fn fn_00b6b610(
    e: &mut Engine,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    make_value: u32,
    word: u32,
    eighth: u32,
    ninth: u32,
) -> Ptr {
    let renderer = e.global::<u32>(D3D_RENDERER_011F9508);
    let texture = e
        .call(
            MAKE_RENDERED_TEXTURE_00A7FC00,
            &args![first, second, third, renderer, fourth, fifth, eighth, ninth],
        )
        .u32();
    if texture == 0 {
        return Ptr::NULL;
    }
    let block = e.call(OPERATOR_NEW_00AA13E0, &args![0x40u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    fn_00b6b380(
        e,
        Ptr::new(block),
        Ptr::new(texture),
        make_value as u8,
        word,
    )
}

/// The renderer's held targets: their count (`011fa020`) and the array after it (`011fa024`).
const TARGET_COUNT_011FA020: u32 = 0x011f_a020;
const TARGETS_011FA024: u32 = 0x011f_a024;

// Translated from 00b6b6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6b330`), no arguments: pops every held target (the
/// count at `011fa020` lowered before each one): a non-null one is released (its count at +4,
/// deleted through its slot +4 at 0) and its slot cleared, the count read again after the
/// release. Then, with the renderer's (`011f4748`) byte +0x208 set, `00b6b330` on it (a tail
/// call).
pub fn fn_00b6b6c0(e: &mut Engine) {
    let mut count = e.global::<u32>(TARGET_COUNT_011FA020);
    while count != 0 {
        count = count.wrapping_sub(1);
        let slot = TARGETS_011FA024.wrapping_add(count.wrapping_mul(4));
        let target = e.mem.u32(slot);
        e.set_global(TARGET_COUNT_011FA020, count);
        if target != 0 {
            release_reference(e, target);
            count = e.global::<u32>(TARGET_COUNT_011FA020);
            e.mem.set_u32(slot, 0);
        }
    }
    let renderer = e.global::<u32>(GLOBAL_011F4748);
    if e.mem.u8(renderer + 0x208) != 0 {
        fn_00b6b330(e, Ptr::new(renderer));
    }
}

// Translated from 00b6b730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6b330`), no arguments, on the renderer at `011f4748`
/// (read before): `00b6b6c0`; then in mode 1 (+0x200) its slot +0x180 answering true moves it
/// to mode 3, and in mode 3 its slot +0x184 answering true counts +0x204 up and moves it to
/// mode 0.
pub fn fn_00b6b730(e: &mut Engine) {
    let renderer = e.global::<u32>(GLOBAL_011F4748);
    fn_00b6b6c0(e);
    if e.mem.u32(renderer + 0x200) == 1 && e.vcall(renderer, 0x180, &args![]).bool() {
        e.mem.set_u32(renderer + 0x200, 3);
    }
    if e.mem.u32(renderer + 0x200) == 3 && e.vcall(renderer, 0x184, &args![]).bool() {
        let frames = e.mem.u32(renderer + 0x204).wrapping_add(1);
        e.mem.set_u32(renderer + 0x204, frames);
        e.mem.set_u32(renderer + 0x200, 0);
    }
}

// Translated from 00b6b790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6b330`), no arguments, on the renderer at `011f4748`
/// (read before): `00b6b6c0`; then in mode 2 (+0x200) its slot +0x180 answering true moves it
/// to mode 0.
pub fn fn_00b6b790(e: &mut Engine) {
    let renderer = e.global::<u32>(GLOBAL_011F4748);
    fn_00b6b6c0(e);
    if e.mem.u32(renderer + 0x200) == 2 && e.vcall(renderer, 0x180, &args![]).bool() {
        e.mem.set_u32(renderer + 0x200, 0);
    }
}

/// `NiXenonRenderer::BeginUsingRenderTargetGroup` (engine map, `00b6b2a0`, `thiscall` on the
/// renderer: the group, the flags) and `00b6b050` (unnamed, `cdecl`, the group).
const BEGIN_USING_TARGET_00B6B2A0: u32 = 0x00b6_b2a0;
const KEEP_TARGET_00B6B050: u32 = 0x00b6_b050;

// Translated from 00b6b7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6b330`), `cdecl`, two words (a render-target group, the
/// flags), on the renderer at `011f4748`: without a group the renderer's slot +0xc8 answers
/// one; with the byte +0x208 set `00b6b330` runs first; then the group (asked of slot +0xc8
/// again when still null) begins (`BeginUsingRenderTargetGroup` with the flags) and `00b6b050`
/// is given the group.
pub fn fn_00b6b7d0(e: &mut Engine, group: u32, flags: u32) {
    let renderer = e.global::<u32>(GLOBAL_011F4748);
    let group = if group != 0 {
        group
    } else {
        e.vcall(renderer, 0xc8, &args![]).u32()
    };
    if e.mem.u8(renderer + 0x208) != 0 {
        fn_00b6b330(e, Ptr::new(renderer));
    }
    let begun = if group != 0 {
        group
    } else {
        let renderer = e.global::<u32>(GLOBAL_011F4748);
        e.vcall(renderer, 0xc8, &args![]).u32()
    };
    let renderer = e.global::<u32>(GLOBAL_011F4748);
    e.call(BEGIN_USING_TARGET_00B6B2A0, &args![renderer, begun, flags]);
    e.call(KEEP_TARGET_00B6B050, &args![group]);
}

/// `00b6b890` and `00b6b8d0`: with the renderer (`011f4748`) in mode 0 and its slot +0x17c
/// answering true, the mode becomes `mode`; then `00b6b7d0(group, flags)`.
fn begin_targets(e: &mut Engine, mode: u32, flags: u32, group: u32) {
    let renderer = e.global::<u32>(GLOBAL_011F4748);
    if e.mem.u32(renderer + 0x200) == 0 && e.vcall(renderer, 0x17c, &args![]).bool() {
        e.mem.set_u32(renderer + 0x200, mode);
    }
    fn_00b6b7d0(e, group, flags);
}

// Translated from 00b6b890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6b330`), `cdecl`, two words (the flags, a render-target
/// group): in mode 0 the renderer's slot +0x17c answering true moves it to mode 1; then
/// `00b6b7d0` with the group and the flags.
pub fn fn_00b6b890(e: &mut Engine, flags: u32, group: u32) {
    begin_targets(e, 1, flags, group);
}

// Translated from 00b6b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6b330`), `cdecl`, two words (the flags, a render-target
/// group): in mode 0 the renderer's slot +0x17c answering true moves it to mode 2; then
/// `00b6b7d0` with the group and the flags.
pub fn fn_00b6b8d0(e: &mut Engine, flags: u32, group: u32) {
    begin_targets(e, 2, flags, group);
}

/// `00b6b0b0` (unit `run_00b6adb0`, unnamed; no arguments) and `00b6b100` (unnamed; no
/// arguments, answers a render-target group or null).
const FN_00B6B0B0: u32 = 0x00b6_b0b0;
const HELD_GROUP_00B6B100: u32 = 0x00b6_b100;

// Translated from 00b6b840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6b330`), no arguments: with targets held (`011fa020` not
/// 0), runs `00b6b330` on the renderer at `011f4748`, then `00b6b0b0`, and begins the group
/// `00b6b100` answers (the renderer's slot +0xc8 when null) with flags 0
/// (`BeginUsingRenderTargetGroup`).
pub fn fn_00b6b840(e: &mut Engine) {
    if e.global::<u32>(TARGET_COUNT_011FA020) == 0 {
        return;
    }
    let renderer = e.global::<u32>(GLOBAL_011F4748);
    fn_00b6b330(e, Ptr::new(renderer));
    e.call(FN_00B6B0B0, &args![]);
    let mut group = e.call(HELD_GROUP_00B6B100, &args![]).u32();
    if group == 0 {
        let renderer = e.global::<u32>(GLOBAL_011F4748);
        group = e.vcall(renderer, 0xc8, &args![]).u32();
    }
    let renderer = e.global::<u32>(GLOBAL_011F4748);
    e.call(BEGIN_USING_TARGET_00B6B2A0, &args![renderer, group, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b6_b840, fn_00b6b840()),
        entry!(0x00b6_b380, fn_00b6b380(Ptr, Ptr, u8, u32) -> Ptr),
        entry!(0x00b6_b330, fn_00b6b330(Ptr) -> bool),
        entry!(
            0x00b6_b610,
            fn_00b6b610(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00b6_b6c0, fn_00b6b6c0()),
        entry!(0x00b6_b730, fn_00b6b730()),
        entry!(0x00b6_b790, fn_00b6b790()),
        entry!(0x00b6_b7d0, fn_00b6b7d0(u32, u32)),
        entry!(0x00b6_b890, fn_00b6b890(u32, u32)),
        entry!(0x00b6_b8d0, fn_00b6b8d0(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_inline::VECTOR_CONSTRUCT;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    const TEXTURE_VTABLE: u32 = 0x0e80_0000;
    const GET_RTTI: u32 = 0x0e81_0000;
    const BUFFER_OF: u32 = 0x0e81_0010;

    /// The rig with a texture whose `GetRTTI` answers an RTTI chain ending in `base_rtti`
    /// (one more RTTI in front of it), render targets made as new objects (count 0).
    fn engine(base_rtti: u32) -> (Engine, u32) {
        let mut e = rig();
        e.map(0x011f_4000, 0x1000);
        e.set_global(GLOBAL_011F4748, 0x0e90_0000u32);
        let mut slots = vec![0u32; 0x2a];
        slots[1] = REFERENCE_DELETE;
        slots[2] = GET_RTTI;
        slots[0x29] = BUFFER_OF;
        e.put_vtable(TEXTURE_VTABLE, &slots);
        let rtti = e.mem.alloc(8);
        e.mem.set_u32(rtti + 4, base_rtti);
        e.register_double(GET_RTTI, move |_, _| rtti.into_ret());
        e.register(BUFFER_OF, |_, _| 0x0e91_0000u32.into_ret());
        e.register(NI_OBJECT_CONSTRUCT_00A5D3A0, |_, a| a[0].into_ret());
        e.register(VALUE_00B6B180, |_, _| 0x55u32.into_ret());
        e.register(RENDER_TARGET_00EE8760, |e, _| object(e, 0).into_ret());
        let texture = e.mem.alloc(0x70);
        e.mem.set_u32(texture, TEXTURE_VTABLE);
        for index in 0..6 {
            e.mem
                .set_u32(texture + 0x4c + 4 * index, 0x0e92_0000 + index);
        }
        (e, texture)
    }

    #[test]
    fn fn_00b6b380_takes_six_targets_from_a_cube_map() {
        let (mut e, texture) = engine(RENDERED_CUBE_MAP_RTTI_012708E4);
        let this = e.mem.alloc(0x40);
        let answer = e.call(0x00b6_b380, &args![this, texture, 1u32, 9u32]).u32();
        assert_eq!(answer, this);
        assert_eq!(e.mem.u32(this), VTABLE_010AE500);
        assert_eq!(e.mem.u32(this + 0x30), texture);
        assert_eq!(calls(&e, VALUE_00B6B180), vec![vec![texture, 9]]);
        let targets = calls(&e, RENDER_TARGET_00EE8760);
        let expected: Vec<Vec<u32>> = (0..6)
            .map(|i| vec![0x0e92_0000 + i, 0x0e90_0000, 0x55])
            .collect();
        assert_eq!(targets, expected);
        for index in 0..6 {
            let target = e.mem.u32(this + 8 + 4 * index);
            assert_ne!(target, 0);
            assert_eq!(e.mem.u32(target + 4), 1);
        }
        assert_eq!(e.mem.bytes(this + 0x20, 0xc), vec![0; 0xc]);
        assert_eq!(e.mem.u32(this + 0x2c), 0xffff_ffff);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 8, 6, DESTRUCT_00B66F20),
                pointer_array(this + 0x30, 4, DESTRUCT_0045CEC0)
            ]
        );
        assert!(calls(&e, BUFFER_OF).is_empty());
    }

    #[test]
    fn fn_00b6b380_takes_one_target_from_another_texture() {
        let (mut e, texture) = engine(0);
        let this = e.mem.alloc(0x40);
        e.call(0x00b6_b380, &args![this, texture, 0u32, 9u32]);
        assert!(calls(&e, VALUE_00B6B180).is_empty());
        assert_eq!(calls(&e, BUFFER_OF), vec![vec![texture]]);
        assert_eq!(
            calls(&e, RENDER_TARGET_00EE8760),
            vec![vec![0x0e91_0000, 0x0e90_0000, 0]]
        );
        assert_ne!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.bytes(this + 0xc, 0x14), vec![0; 0x14]);
        assert_eq!(calls(&e, GET_RTTI), vec![vec![texture]]);
    }
}

#[cfg(test)]
mod renderer_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    const RENDERER_SLOTS: u32 = 0x0e7a_0000;

    /// A renderer at `011f4748` in `mode`, its byte +0x208 as given, its slots +0xc8, +0x17c,
    /// +0x180, +0x184 and +0x198 answering the given words, `LeaveCriticalSection`, the
    /// begin and keep calls doubled.
    fn renderer_rig(mode: u32, held: u8, answers: [u32; 5]) -> (Engine, u32) {
        let mut e = rig();
        e.map(0x011f_4000, 0x1000);
        e.map(0x011f_a000, 0x1000);
        let mut slots = vec![0u32; 0x67];
        for (offset, answer) in [0xc8u32, 0x17c, 0x180, 0x184, 0x198].iter().zip(answers) {
            let target = RENDERER_SLOTS + 0x1000 + offset;
            slots[(offset / 4) as usize] = target;
            e.register_double(target, move |_, _| answer.into_ret());
        }
        e.put_vtable(RENDERER_SLOTS, &slots);
        for callee in [
            IMPORT_LEAVE_CRITICAL_SECTION,
            BEGIN_USING_TARGET_00B6B2A0,
            KEEP_TARGET_00B6B050,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let renderer = e.mem.alloc(0x210);
        e.mem.set_u32(renderer, RENDERER_SLOTS);
        e.mem.set_u32(renderer + 0x200, mode);
        e.mem.set_u8(renderer + 0x208, held);
        e.set_global(GLOBAL_011F4748, renderer);
        (e, renderer)
    }

    fn slot(offset: u32) -> u32 {
        RENDERER_SLOTS + 0x1000 + offset
    }

    #[test]
    fn fn_00b6b330_leaves_the_section_in_modes_1_and_2() {
        let (mut e, renderer) = renderer_rig(2, 1, [0, 0, 0, 0, 1]);
        assert!(e.call(0x00b6_b330, &args![renderer]).bool());
        assert_eq!(
            calls(&e, IMPORT_LEAVE_CRITICAL_SECTION),
            vec![vec![renderer + 0x80]]
        );
        assert_eq!(e.mem.u8(renderer + 0x208), 0);
        // The byte now clear, mode 0, and a slot answering false: nothing.
        assert!(!e.call(0x00b6_b330, &args![renderer]).bool());
        let (mut e, renderer) = renderer_rig(0, 1, [0, 0, 0, 0, 1]);
        assert!(!e.call(0x00b6_b330, &args![renderer]).bool());
        let (mut e, renderer) = renderer_rig(1, 1, [0, 0, 0, 0, 0]);
        assert!(!e.call(0x00b6_b330, &args![renderer]).bool());
        assert_eq!(e.mem.u8(renderer + 0x208), 1);
        assert!(calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).is_empty());
    }

    #[test]
    fn fn_00b6b610_wraps_a_rendered_texture() {
        let mut e = rig();
        e.map(0x011f_9000, 0x1000);
        e.set_global(D3D_RENDERER_011F9508, 0x0e7b_0000u32);
        e.register(MAKE_RENDERED_TEXTURE_00A7FC00, |_, _| Ret::default());
        e.register(OPERATOR_NEW_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        let words = args![1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32];
        assert_eq!(e.call(0x00b6_b610, &words).u32(), 0);
        assert_eq!(
            calls(&e, MAKE_RENDERED_TEXTURE_00A7FC00),
            vec![vec![1, 2, 3, 0x0e7b_0000, 4, 5, 8, 9]]
        );
        assert!(calls(&e, OPERATOR_NEW_00AA13E0).is_empty());
        // A texture but no memory: 0 too.
        e.register(MAKE_RENDERED_TEXTURE_00A7FC00, |_, _| {
            0x0e7c_0000u32.into_ret()
        });
        e.register(OPERATOR_NEW_00AA13E0, |_, _| Ret::default());
        assert_eq!(e.call(0x00b6_b610, &words).u32(), 0);
        assert_eq!(calls(&e, OPERATOR_NEW_00AA13E0), vec![vec![0x40]]);
    }

    #[test]
    fn fn_00b6b610_builds_the_object_over_the_texture() {
        let mut e = rig();
        e.map(0x011f_9000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.set_global(GLOBAL_011F4748, 0x0e7f_0000u32);
        let mut slots = vec![0u32; 0x2a];
        slots[1] = REFERENCE_DELETE;
        slots[2] = 0x0e80_0008;
        slots[0x29] = 0x0e80_00a4;
        e.put_vtable(0x0e81_0000, &slots);
        e.register(0x0e80_0008, |_, _| Ret::default());
        e.register(0x0e80_00a4, |_, _| 0x0e82_0000u32.into_ret());
        let texture = e.mem.alloc(0x70);
        e.mem.set_u32(texture, 0x0e81_0000);
        e.register_double(MAKE_RENDERED_TEXTURE_00A7FC00, move |_, _| {
            texture.into_ret()
        });
        e.register(OPERATOR_NEW_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_OBJECT_CONSTRUCT_00A5D3A0, |_, a| a[0].into_ret());
        e.register(VALUE_00B6B180, |_, _| 0x66u32.into_ret());
        e.register(RENDER_TARGET_00EE8760, |e, _| object(e, 0).into_ret());
        let words = args![1u32, 2u32, 3u32, 4u32, 5u32, 0x101u32, 7u32, 8u32, 9u32];
        let built = e.call(0x00b6_b610, &words).u32();
        assert_ne!(built, 0);
        assert_eq!(e.mem.u32(built), VTABLE_010AE500);
        assert_eq!(e.mem.u32(built + 0x30), texture);
        // The sixth word's low byte (1) asks for the value with the seventh word.
        assert_eq!(calls(&e, VALUE_00B6B180), vec![vec![texture, 7]]);
        assert_eq!(
            calls(&e, RENDER_TARGET_00EE8760),
            vec![vec![0x0e82_0000, 0x0e7f_0000, 0x66]]
        );
    }

    #[test]
    fn fn_00b6b6c0_releases_every_held_target() {
        let (mut e, renderer) = renderer_rig(1, 1, [0, 0, 0, 0, 1]);
        let first = object(&mut e, 1);
        let second = object(&mut e, 2);
        e.set_global(TARGET_COUNT_011FA020, 3u32);
        e.set_global(TARGETS_011FA024, first);
        e.set_global(TARGETS_011FA024 + 8, second);
        e.call(0x00b6_b6c0, &args![]);
        assert_eq!(e.global::<u32>(TARGET_COUNT_011FA020), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![first]]);
        assert_eq!(e.mem.u32(second + 4), 1);
        assert_eq!(e.global::<u32>(TARGETS_011FA024), 0);
        assert_eq!(e.global::<u32>(TARGETS_011FA024 + 8), 0);
        // The byte +0x208 was set: 00b6b330 ran on the renderer.
        assert_eq!(e.mem.u8(renderer + 0x208), 0);
    }

    #[test]
    fn fn_00b6b730_steps_the_modes() {
        let (mut e, renderer) = renderer_rig(1, 0, [0, 0, 1, 1, 0]);
        e.call(0x00b6_b730, &args![]);
        assert_eq!(e.mem.u32(renderer + 0x200), 0);
        assert_eq!(e.mem.u32(renderer + 0x204), 1);
        let (mut e, renderer) = renderer_rig(1, 0, [0, 0, 1, 0, 0]);
        e.call(0x00b6_b730, &args![]);
        assert_eq!(e.mem.u32(renderer + 0x200), 3);
        let (mut e, renderer) = renderer_rig(2, 0, [0, 0, 1, 1, 0]);
        e.call(0x00b6_b730, &args![]);
        assert_eq!(e.mem.u32(renderer + 0x200), 2);
        assert!(calls(&e, slot(0x180)).is_empty());
    }

    #[test]
    fn fn_00b6b790_leaves_mode_2() {
        let (mut e, renderer) = renderer_rig(2, 0, [0, 0, 1, 0, 0]);
        e.call(0x00b6_b790, &args![]);
        assert_eq!(e.mem.u32(renderer + 0x200), 0);
        let (mut e, renderer) = renderer_rig(2, 0, [0, 0, 0, 0, 0]);
        e.call(0x00b6_b790, &args![]);
        assert_eq!(e.mem.u32(renderer + 0x200), 2);
        let (mut e, renderer) = renderer_rig(1, 0, [0, 0, 1, 0, 0]);
        e.call(0x00b6_b790, &args![]);
        assert_eq!(e.mem.u32(renderer + 0x200), 1);
    }

    #[test]
    fn fn_00b6b7d0_begins_the_group() {
        let (mut e, renderer) = renderer_rig(0, 0, [0x0e7d_0000, 0, 0, 0, 0]);
        e.call(0x00b6_b7d0, &args![0x0e7e_0000u32, 5u32]);
        assert!(calls(&e, slot(0xc8)).is_empty());
        assert_eq!(
            calls(&e, BEGIN_USING_TARGET_00B6B2A0),
            vec![vec![renderer, 0x0e7e_0000, 5]]
        );
        assert_eq!(calls(&e, KEEP_TARGET_00B6B050), vec![vec![0x0e7e_0000]]);
        e.call(0x00b6_b7d0, &args![0u32, 6u32]);
        assert_eq!(calls(&e, slot(0xc8)).len(), 1);
        assert_eq!(
            calls(&e, BEGIN_USING_TARGET_00B6B2A0)[1],
            vec![renderer, 0x0e7d_0000, 6]
        );
        // The renderer's group null too: it is asked again and the null kept.
        let (mut e, _) = renderer_rig(0, 0, [0, 0, 0, 0, 0]);
        e.call(0x00b6_b7d0, &args![0u32, 6u32]);
        assert_eq!(calls(&e, slot(0xc8)).len(), 2);
        assert_eq!(calls(&e, KEEP_TARGET_00B6B050), vec![vec![0]]);
    }

    #[test]
    fn fn_00b6b890_and_00b6b8d0_set_the_mode_and_begin() {
        let (mut e, renderer) = renderer_rig(0, 0, [0, 1, 0, 0, 0]);
        e.call(0x00b6_b890, &args![7u32, 0x0e7e_0000u32]);
        assert_eq!(e.mem.u32(renderer + 0x200), 1);
        assert_eq!(
            calls(&e, BEGIN_USING_TARGET_00B6B2A0),
            vec![vec![renderer, 0x0e7e_0000, 7]]
        );
        let (mut e, renderer) = renderer_rig(0, 0, [0, 1, 0, 0, 0]);
        e.call(0x00b6_b8d0, &args![7u32, 0x0e7e_0000u32]);
        assert_eq!(e.mem.u32(renderer + 0x200), 2);
        // Not in mode 0: the mode stays and the slot is not asked.
        let (mut e, renderer) = renderer_rig(3, 0, [0, 1, 0, 0, 0]);
        e.call(0x00b6_b8d0, &args![7u32, 0x0e7e_0000u32]);
        assert_eq!(e.mem.u32(renderer + 0x200), 3);
        assert!(calls(&e, slot(0x17c)).is_empty());
    }
}

#[cfg(test)]
mod n070_tests {
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

    const GROUP_SLOT: u32 = 0x0e40_0000;

    /// A renderer in mode 0 (so `00b6b330` answers false) whose slot +0xc8 answers 0x4444;
    /// `00b6b100` answers `held`.
    fn rig(count: u32, held: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.map(0x011f_a000, 0x1000);
        e.set_global(TARGET_COUNT_011FA020, count);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + 0xc8, GROUP_SLOT);
        let renderer = e.mem.alloc(0x210);
        e.mem.set_u32(renderer, vtable);
        e.set_global(GLOBAL_011F4748, renderer);
        e.register(GROUP_SLOT, |_, _| 0x4444u32.into_ret());
        e.register(FN_00B6B0B0, |_, _| Ret::default());
        e.register_double(HELD_GROUP_00B6B100, move |_, _| held.into_ret());
        e.register(BEGIN_USING_TARGET_00B6B2A0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, renderer)
    }

    #[test]
    fn fn_00b6b840_begins_the_held_group() {
        let (mut e, renderer) = rig(1, 0x5555);
        e.call(0x00b6_b840, &args![]);
        assert_eq!(calls(&e, FN_00B6B0B0).len(), 1);
        assert!(calls(&e, GROUP_SLOT).is_empty());
        assert_eq!(
            calls(&e, BEGIN_USING_TARGET_00B6B2A0),
            vec![vec![renderer, 0x5555, 0]]
        );
    }

    #[test]
    fn fn_00b6b840_asks_the_renderer_without_a_held_group() {
        let (mut e, renderer) = rig(2, 0);
        e.call(0x00b6_b840, &args![]);
        assert_eq!(calls(&e, GROUP_SLOT), vec![vec![renderer]]);
        assert_eq!(
            calls(&e, BEGIN_USING_TARGET_00B6B2A0),
            vec![vec![renderer, 0x4444, 0]]
        );
    }

    #[test]
    fn fn_00b6b840_does_nothing_without_targets() {
        let (mut e, _) = rig(0, 0x5555);
        e.call(0x00b6_b840, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
