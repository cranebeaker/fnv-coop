//! `NiMain/nilines.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiLines::ms_RTTI` (the class's `NiRTTI` record).
const NI_LINES_RTTI: u32 = 0x011f_4a48;

// Translated from 00a74790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_lines_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_LINES_RTTI)
}

/// `00a80b50`: the jump target of `00a74780`.
const JUMP_TARGET_00A74780: u32 = 0x00a8_0b50;

/// `00a80470`: the jump target of `00a89770`.
const JUMP_TARGET_00A89770: u32 = 0x00a8_0470;

/// `00a80bf0`: the jump target of `00a9b180`.
const JUMP_TARGET_00A9B180: u32 = 0x00a8_0bf0;

/// `00a80430`: the jump target of `00a9b190`.
const JUMP_TARGET_00A9B190: u32 = 0x00a8_0430;

/// `00a80db0`: the jump target of `00a9b1a0`.
const JUMP_TARGET_00A9B1A0: u32 = 0x00a8_0db0;

// Translated from 00a74780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::LinkObject` (Xbox PDB): a 5-byte jump to `00a80b50` (engine map: `NiGeometry::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiLines@@UAAXAAVNiStream@@@Z`.
pub fn ni_lines_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A74780, &args![this, stream]);
}

// Translated from 00a89770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::IsEqual` (Xbox PDB): a 5-byte jump to `00a80470` (engine map: `NiGeometry::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiLines@@UAA_NPAVNiObject@@@Z`.
pub fn ni_lines_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A89770, &args![this, other]).bool()
}

// Translated from 00a9b180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::LoadBinary` (Xbox PDB): a 5-byte jump to `00a80bf0` (engine map: `NiGeometry::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiLines@@UAAXAAVNiStream@@@Z`.
pub fn ni_lines_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A9B180, &args![this, stream]);
}

// Translated from 00a9b190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a80430` (engine map: `NiGeometry::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiLines@@UAA_NAAVNiStream@@@Z`.
pub fn ni_lines_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A9B190, &args![this, stream]).bool()
}

// Translated from 00a9b1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::SaveBinary` (Xbox PDB): a 5-byte jump to `00a80db0` (engine map: `NiGeometry::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiLines@@UAAXAAVNiStream@@@Z`.
pub fn ni_lines_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A9B1A0, &args![this, stream]);
}

/// `NiGeometry::CopyMembers` (`00a5ac90`, thiscall: clone, cloning process).
const NI_GEOMETRY_COPY_MEMBERS: u32 = 0x00a5_ac90;

// Translated from 00a80400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::CopyMembers` (Xbox PDB name in the engine map): hands this
/// object's word at +0xB8 to the clone's vtable +0xE4, then copies the
/// `NiGeometry` members (`00a5ac90`).
pub fn ni_lines_copy_members(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    let data = e.mem.u32(this.addr().wrapping_add(0xb8));
    e.vcall(clone.addr(), 0xe4, &args![data]);
    e.call(NI_GEOMETRY_COPY_MEMBERS, &args![this, clone, cloning]);
}

/// `NiLines`'s vtable.
const NI_LINES_VTABLE: u32 = 0x0109_d55c;
/// `operator new` (`cdecl`, the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `00a8a680` (unnamed and unplaced in the engine map; `thiscall` with seven stack words):
/// the constructor of the 0x44-byte lines data `NiLines::NiLines` builds from its words.
const LINES_DATA_CONSTRUCT_00A8A680: u32 = 0x00a8_a680;
/// `NiGeometry::NiGeometry` (`00a804e0`, `thiscall`, the data) and the default geometry
/// constructor (`00a80540`, unnamed, unit `nigeometry.cpp`; `thiscall`, no stack words).
const NI_GEOMETRY_CONSTRUCT_WITH_DATA: u32 = 0x00a8_04e0;
const NI_GEOMETRY_CONSTRUCT_00A80540: u32 = 0x00a8_0540;
/// `NiGeometry::RenderImmediate` (`00a80610`, the renderer) and `NiGeometry::GetViewerStrings`
/// (`00a80e70`, the string array), both `thiscall`.
const NI_GEOMETRY_RENDER_IMMEDIATE: u32 = 0x00a8_0610;
const NI_GEOMETRY_GET_VIEWER_STRINGS: u32 = 0x00a8_0e70;
/// `NiGetViewerString` (`00a7d330`, `cdecl`, the class name), answering a new string.
const VIEWER_STRING_CLASS: u32 = 0x00a7_d330;
/// The string array's grow (`0096ad30`, `thiscall`, the new size) and set-at (`0096ae90`,
/// `thiscall`, (index, address of the string)), both unnamed in the engine map.
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

// Translated from 00a746e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::NiLines` (Xbox PDB; `thiscall` with seven stack words): a new 0x44-byte lines
/// data built from the seven words in order (`00a8a680`; null when the allocation fails),
/// `NiGeometry::NiGeometry` with it, then the vtable `0109d55c`. Answers `this`.
#[allow(clippy::too_many_arguments)]
pub fn ni_lines_ni_lines(
    e: &mut Engine,
    this: Ptr,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
    word_7: u32,
) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x44u32]).u32();
    let data = if block == 0 {
        0
    } else {
        e.call(
            LINES_DATA_CONSTRUCT_00A8A680,
            &args![block, word_1, word_2, word_3, word_4, word_5, word_6, word_7],
        )
        .u32()
    };
    e.call(NI_GEOMETRY_CONSTRUCT_WITH_DATA, &args![this, data]);
    e.mem.set_u32(this.addr(), NI_LINES_VTABLE);
    this
}

/// A new 0xc4-byte `NiLines` (the default geometry constructor, then the vtable), or null.
fn new_lines(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0xc4u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(NI_GEOMETRY_CONSTRUCT_00A80540, &args![block]);
    e.mem.set_u32(block, NI_LINES_VTABLE);
    block
}

// Translated from 00a74750 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiLines`): a new default `NiLines`, or null.
pub fn fn_00a74750(e: &mut Engine) -> Ptr {
    Ptr::new(new_lines(e))
}

// Translated from 00a747a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::RenderImmediate` (Xbox PDB): `NiGeometry::RenderImmediate`, then, when the
/// renderer's word at +0x200 is 1 or 2 and its byte at +0x208 is 1, the renderer's virtual
/// slot +0x1c4 with this object.
pub fn ni_lines_render_immediate(e: &mut Engine, this: Ptr, renderer: Ptr) {
    e.call(NI_GEOMETRY_RENDER_IMMEDIATE, &args![this, renderer]);
    let r = renderer.addr();
    let mode = e.mem.u32(r + 0x200);
    if (mode == 1 || mode == 2) && e.mem.u8(r + 0x208) == 1 {
        e.vcall(r, 0x1c4, &args![this]);
    }
}

// Translated from 00a747e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::CreateClone` (Xbox PDB): a new default `NiLines` (null when the allocation
/// fails) and `CopyMembers` into it, run on the null clone too, as the exe does. Answers
/// the clone.
pub fn ni_lines_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let clone = Ptr::new(new_lines(e));
    ni_lines_copy_members(e, this, clone, cloning_process);
    clone
}

// Translated from 00a74830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLines::GetViewerStrings` (Xbox PDB): `NiGeometry::GetViewerStrings`, then the class
/// name (`NiGetViewerString` of `ms_RTTI`'s name word) appended as the exe's inlined `Add`
/// does (grow to step + size when full, set-at the old size through a stack word).
pub fn ni_lines_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(NI_GEOMETRY_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(NI_LINES_RTTI);
    let text = e.call(VIEWER_STRING_CLASS, &args![name]).u32();
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a74790, ni_lines_get_rtti() -> Ptr),
        entry!(0x00a7_4780, ni_lines_link_object(Ptr, Ptr)),
        entry!(0x00a8_9770, ni_lines_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_b180, ni_lines_load_binary(Ptr, Ptr)),
        entry!(0x00a9_b190, ni_lines_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a9_b1a0, ni_lines_save_binary(Ptr, Ptr)),
        entry!(0x00a8_0400, ni_lines_copy_members(Ptr, Ptr, Ptr)),
        entry!(
            0x00a7_46e0,
            ni_lines_ni_lines(Ptr, u32, u32, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00a7_4750, fn_00a74750() -> Ptr),
        entry!(0x00a7_47a0, ni_lines_render_immediate(Ptr, Ptr)),
        entry!(0x00a7_47e0, ni_lines_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a7_4830, ni_lines_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_lines_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a74790, &args![]).u32(), NI_LINES_RTTI);
    }

    #[test]
    fn thunk_00a74780_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_4780;
        const TARGET: u32 = 0x00a8_0b50;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a89770_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a8_9770;
        const TARGET: u32 = 0x00a8_0470;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00a9b180_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_b180;
        const TARGET: u32 = 0x00a8_0bf0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a9b190_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_b190;
        const TARGET: u32 = 0x00a8_0430;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00a9b1a0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_b1a0;
        const TARGET: u32 = 0x00a8_0db0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn copy_members_sets_the_clone_data_then_copies_the_geometry() {
        let mut e = Engine::new();
        e.register(0x7000_00e4, |_, _| Ret::default());
        e.register(NI_GEOMETRY_COPY_MEMBERS, |_, _| Ret::default());
        e.put_vtable(0x9100_0000, &[0; 0x3a]);
        e.mem.set_u32(0x9100_00e4, 0x7000_00e4);
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 0xb8, 0x4444);
        let clone = e.mem.alloc(0xc0);
        e.mem.set_u32(clone, 0x9100_0000);
        e.call_log = Some(vec![]);
        e.call(0x00a8_0400, &args![this, clone, 0x5555u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x7000_00e4, vec![clone, 0x4444]));
        assert_eq!(
            log[2],
            (NI_GEOMETRY_COPY_MEMBERS, vec![this, clone, 0x5555])
        );
    }
}

#[cfg(test)]
mod n016_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LINES_DATA_CONSTRUCT_00A8A680, |_, a| a[0].into_ret());
        for double in [
            NI_GEOMETRY_CONSTRUCT_WITH_DATA,
            NI_GEOMETRY_CONSTRUCT_00A80540,
            NI_GEOMETRY_RENDER_IMMEDIATE,
            NI_GEOMETRY_GET_VIEWER_STRINGS,
            NI_GEOMETRY_COPY_MEMBERS,
            STRINGS_GROW,
            STRINGS_SET_AT,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn constructor_builds_the_data_from_its_words() {
        let mut e = engine();
        let this = e.mem.alloc(0xc4);
        let back = e.call(
            0x00a7_46e0,
            &args![this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32],
        );
        assert_eq!(back.u32(), this);
        let data = calls_to(&e, LINES_DATA_CONSTRUCT_00A8A680);
        assert_eq!(data[0][1..], [1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x44]]);
        assert_eq!(
            calls_to(&e, NI_GEOMETRY_CONSTRUCT_WITH_DATA),
            vec![vec![this, data[0][0]]]
        );
        assert_eq!(e.mem.u32(this), NI_LINES_VTABLE);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call(
            0x00a7_46e0,
            &args![this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32],
        );
        assert_eq!(
            calls_to(&e, NI_GEOMETRY_CONSTRUCT_WITH_DATA)[1],
            vec![this, 0]
        );
        assert_eq!(calls_to(&e, LINES_DATA_CONSTRUCT_00A8A680).len(), 1);
    }

    #[test]
    fn create_object_builds_a_default_lines() {
        let mut e = engine();
        let object = e.call(0x00a7_4750, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_LINES_VTABLE);
        assert_eq!(
            calls_to(&e, NI_GEOMETRY_CONSTRUCT_00A80540),
            vec![vec![object]]
        );
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0xc4]]);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a7_4750, &args![]).u32(), 0);
    }

    #[test]
    fn render_immediate_draws_through_the_renderer_in_modes_1_and_2() {
        let mut e = engine();
        let mut slots = vec![0u32; 0x80];
        slots[0x1c4 / 4] = 0x0e00_01c4;
        e.put_vtable(0x0e10_0000, &slots);
        e.register(0x0e00_01c4, |_, _| Ret::default());
        let renderer = e.mem.alloc(0x210);
        e.mem.set_u32(renderer, 0x0e10_0000);
        let this = e.mem.alloc(0xc4);
        for (mode, flag, draws) in [(1u32, 1u8, 1usize), (2, 1, 2), (3, 1, 2), (2, 0, 2)] {
            e.mem.set_u32(renderer + 0x200, mode);
            e.mem.set_u8(renderer + 0x208, flag);
            e.call(0x00a7_47a0, &args![this, renderer]);
            assert_eq!(calls_to(&e, 0x0e00_01c4).len(), draws);
        }
        assert_eq!(calls_to(&e, 0x0e00_01c4)[0], vec![renderer, this]);
        assert_eq!(
            calls_to(&e, NI_GEOMETRY_RENDER_IMMEDIATE)[0],
            vec![this, renderer]
        );
    }

    #[test]
    fn create_clone_builds_and_copies_members() {
        let mut e = engine();
        let mut slots = vec![0u32; 0x40];
        slots[0xe4 / 4] = 0x0e00_00e4;
        e.put_vtable(NI_LINES_VTABLE, &slots);
        e.register(0x0e00_00e4, |_, _| Ret::default());
        let this = e.mem.alloc(0xc4);
        e.mem.set_u32(this + 0xb8, 0x55);
        let clone = e.call(0x00a7_47e0, &args![this, 0x77u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_LINES_VTABLE);
        assert_eq!(calls_to(&e, 0x0e00_00e4), vec![vec![clone, 0x55]]);
        assert_eq!(
            calls_to(&e, NI_GEOMETRY_COPY_MEMBERS),
            vec![vec![this, clone, 0x77]]
        );
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name() {
        let mut e = engine();
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(NI_LINES_RTTI, 0x0123_0000);
        e.register(VIEWER_STRING_CLASS, |_, _| 0x0e00_0001u32.into_ret());
        let this = e.mem.alloc(0xc4);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xe, 4);
        e.call(0x00a7_4830, &args![this, strings]);
        assert_eq!(
            calls_to(&e, NI_GEOMETRY_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls_to(&e, VIEWER_STRING_CLASS), vec![vec![0x0123_0000]]);
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 6]]);
        assert_eq!(calls_to(&e, STRINGS_SET_AT)[0][..2], [strings, 2]);
    }
}
