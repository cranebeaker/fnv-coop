//! `NiParticle/bsstripparticlesystem.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c1b0b0`: the jump target of `00c21940`.
const JUMP_TARGET_00C21940: u32 = 0x00c1_b0b0;

/// `00c1b1d0`: the jump target of `00c21950`.
const JUMP_TARGET_00C21950: u32 = 0x00c1_b1d0;

/// `00a7fd90`: the jump target of `00c256f0`.
const JUMP_TARGET_00C256F0: u32 = 0x00a7_fd90;

/// `00c1b3a0`: the jump target of `00c25700`.
const JUMP_TARGET_00C25700: u32 = 0x00c1_b3a0;

/// `00c1b810`: the jump target of `00c25710`.
const JUMP_TARGET_00C25710: u32 = 0x00c1_b810;

/// `00c1b400`: the jump target of `00c25720`.
const JUMP_TARGET_00C25720: u32 = 0x00c1_b400;

// Translated from 00c21940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00c1b0b0` (engine map: `NiParticleSystem::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@BSStripParticleSystem@@UAA_NAAVNiStream@@@Z`.
pub fn bs_strip_particle_system_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00C21940, &args![this, stream]).bool()
}

// Translated from 00c21950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::IsEqual` (Xbox PDB): a 5-byte jump to `00c1b1d0` (engine map: `NiParticleSystem::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSStripParticleSystem@@UAA_NPAVNiObject@@@Z`.
pub fn bs_strip_particle_system_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00C21950, &args![this, other]).bool()
}

// Translated from 00c256f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::OnVisible` (Xbox PDB): a 5-byte jump to `00a7fd90` (engine map: `NiGeometry::OnVisible`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?OnVisible@BSStripParticleSystem@@UAAXAAVNiCullingProcess@@@Z`.
pub fn bs_strip_particle_system_on_visible(e: &mut Engine, this: Ptr, arg: Ptr) {
    e.call(JUMP_TARGET_00C256F0, &args![this, arg]);
}

// Translated from 00c25700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::LoadBinary` (Xbox PDB): a 5-byte jump to `00c1b3a0` (engine map: `NiParticleSystem::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@BSStripParticleSystem@@UAAXAAVNiStream@@@Z`.
pub fn bs_strip_particle_system_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C25700, &args![this, stream]);
}

// Translated from 00c25710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::LinkObject` (Xbox PDB): a 5-byte jump to `00c1b810` (engine map: `NiParticleSystem::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@BSStripParticleSystem@@UAAXAAVNiStream@@@Z`.
pub fn bs_strip_particle_system_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C25710, &args![this, stream]);
}

// Translated from 00c25720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::SaveBinary` (Xbox PDB): a 5-byte jump to `00c1b400` (engine map: `NiParticleSystem::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@BSStripParticleSystem@@UAAXAAVNiStream@@@Z`.
pub fn bs_strip_particle_system_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C25720, &args![this, stream]);
}

// Translated from 00c257c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2560.
pub fn b_s_strip_particle_system_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2560
}

/// `NiTPointerListBase<..., NiPointer<NiPSysModifier>>::RemoveAll` (engine map, `00e74d40`,
/// `thiscall` on the list, no stack words).
const MODIFIER_LIST_REMOVE_ALL: u32 = 0x00e7_4d40;
/// `NiGeometry::~NiGeometry` (engine map, `00a80580`, `thiscall`, no stack words).
const GEOMETRY_DESTRUCT: u32 = 0x00a8_0580;
/// The sized Ni free `00aa1460` (`cdecl`: block, size).
const NI_DELETE: u32 = 0x00aa_1460;

// Translated from 00c1b9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::_scalar_deleting_destructor_` (engine map): `thiscall`, one stack
/// word, `RET 4`. Empties the modifier list at +0xc8, runs `NiGeometry::~NiGeometry`, then
/// frees the 0x110 bytes of `this` when bit 0 of `flags` is set. Answers `this`.
pub fn bs_strip_particle_system_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(MODIFIER_LIST_REMOVE_ALL, &args![this.byte_add(0xc8)]);
    e.call(GEOMETRY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x110u32]);
    }
    this
}

/// `NiGeometry::RenderImmediate` (`00a80610`, `thiscall`, one word: the renderer).
const GEOMETRY_RENDER_IMMEDIATE_00A80610: u32 = 0x00a8_0610;
/// `NiParticleSystem::GetViewerStrings` (`00c1b850`), its constructor (`00c1b5c0`) and
/// `CopyMembers` (`00c1b6f0`, `thiscall`, two words: the clone and the cloning process).
const SYSTEM_GET_VIEWER_STRINGS_00C1B850: u32 = 0x00c1_b850;
const SYSTEM_CONSTRUCT_00C1B5C0: u32 = 0x00c1_b5c0;
const SYSTEM_COPY_MEMBERS_00C1B6F0: u32 = 0x00c1_b6f0;
/// The Ni allocation (`00aa13e0`, `cdecl`: size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `BSStripParticleSystem`'s vtable and RTTI record (its first word the class name).
const BS_STRIP_PARTICLE_SYSTEM_VTABLE: u32 = 0x010b_e35c;
const BS_STRIP_PARTICLE_SYSTEM_RTTI: u32 = 0x0120_2560;
/// `NiGetViewerString` (`00a7d330`, `cdecl`) and the string array's grow (`0096ad30`) and
/// set-at (`0096ae90`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

// Translated from 00c25730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::RenderImmediate` (Xbox PDB, `thiscall`, one word: the renderer):
/// `NiGeometry::RenderImmediate`, then, when the renderer's word +0x200 is 1 or 2 and its
/// byte +0x208 is 1, the renderer's virtual slot +0x1c0 with the system.
pub fn bs_strip_particle_system_render_immediate(e: &mut Engine, this: Ptr, renderer: Ptr) {
    e.call(GEOMETRY_RENDER_IMMEDIATE_00A80610, &args![this, renderer]);
    let mode = e.mem.u32(renderer.addr() + 0x200);
    if (mode == 1 || mode == 2) && e.mem.u8(renderer.addr() + 0x208) == 1 {
        e.vcall(renderer.addr(), 0x1c0, &args![this]);
    }
}

// Translated from 00c25770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::GetViewerStrings` (Xbox PDB, `thiscall`, one word: the string
/// array): `NiParticleSystem::GetViewerStrings`, then the class name (the first word of the
/// RTTI record `01202560`) added to the array (grown to growth + size when full; the string
/// kept in the argument's stack word).
pub fn bs_strip_particle_system_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(SYSTEM_GET_VIEWER_STRINGS_00C1B850, &args![this, strings]);
    let strings = strings.addr();
    let name = e.global::<u32>(BS_STRIP_PARTICLE_SYSTEM_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step.wrapping_add(size)]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00c257d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::CreateClone` (Xbox PDB, `thiscall`, one word: the cloning
/// process): 0x110 bytes from `00aa13e0`; when that succeeds, the `NiParticleSystem`
/// constructor and the vtable `010be35c`; then `NiParticleSystem::CopyMembers` into the clone
/// (0 when the allocation failed, as the exe does). Answers the clone.
pub fn bs_strip_particle_system_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> u32 {
    let clone = e.call(NI_ALLOCATE, &args![0x110u32]).u32();
    if clone != 0 {
        e.call(SYSTEM_CONSTRUCT_00C1B5C0, &args![clone]);
        e.mem.set_u32(clone, BS_STRIP_PARTICLE_SYSTEM_VTABLE);
    }
    e.call(SYSTEM_COPY_MEMBERS_00C1B6F0, &args![this, clone, cloning]);
    clone
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c257c0, b_s_strip_particle_system_get_r_t_t_i() -> u32),
        entry!(
            0x00c1_b9b0,
            bs_strip_particle_system_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c2_1940, bs_strip_particle_system_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00c2_1950, bs_strip_particle_system_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c2_56f0, bs_strip_particle_system_on_visible(Ptr, Ptr)),
        entry!(0x00c2_5700, bs_strip_particle_system_load_binary(Ptr, Ptr)),
        entry!(0x00c2_5710, bs_strip_particle_system_link_object(Ptr, Ptr)),
        entry!(0x00c2_5720, bs_strip_particle_system_save_binary(Ptr, Ptr)),
        entry!(
            0x00c2_5730,
            bs_strip_particle_system_render_immediate(Ptr, Ptr)
        ),
        entry!(
            0x00c2_5770,
            bs_strip_particle_system_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(
            0x00c2_57d0,
            bs_strip_particle_system_create_clone(Ptr, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c21940_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_1940;
        const TARGET: u32 = 0x00c1_b0b0;
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
    fn thunk_00c21950_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_1950;
        const TARGET: u32 = 0x00c1_b1d0;
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
    fn thunk_00c256f0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_56f0;
        const TARGET: u32 = 0x00a7_fd90;
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
    fn thunk_00c25700_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_5700;
        const TARGET: u32 = 0x00c1_b3a0;
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
    fn thunk_00c25710_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_5710;
        const TARGET: u32 = 0x00c1_b810;
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
    fn thunk_00c25720_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_5720;
        const TARGET: u32 = 0x00c1_b400;
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
    fn b_s_strip_particle_system_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_57c0, &args![]).u32(), 0x0120_2560);
    }
}

#[cfg(test)]
mod destructor_tests {
    use super::*;

    #[test]
    fn scalar_deleting_destructor_empties_the_list_destructs_and_frees_on_bit_zero() {
        for (flags, frees) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            let mut e = Engine::new();
            e.register(MODIFIER_LIST_REMOVE_ALL, |_, _| Ret::default());
            e.register(GEOMETRY_DESTRUCT, |_, _| Ret::default());
            e.register(NI_DELETE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(0x00c1_b9b0, &args![0x0e00_3000u32, flags]).u32(),
                0x0e00_3000
            );
            let log = e.call_log.take().unwrap();
            let mut expected = vec![
                (MODIFIER_LIST_REMOVE_ALL, vec![0x0e00_30c8]),
                (GEOMETRY_DESTRUCT, vec![0x0e00_3000]),
            ];
            if frees {
                expected.push((NI_DELETE, vec![0x0e00_3000, 0x110]));
            }
            assert_eq!(log[1..].to_vec(), expected);
        }
    }
}

#[cfg(test)]
mod n063_tests {
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
    fn render_immediate_hands_strips_to_the_renderer() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + 0x1c0, 0x0e60_0000);
        e.register(0x0e60_0000, |_, _| Ret::default());
        e.register(GEOMETRY_RENDER_IMMEDIATE_00A80610, |_, _| Ret::default());
        let renderer = e.mem.alloc(0x210);
        e.mem.set_u32(renderer, vtable);
        for (mode, flag, called) in [(1u32, 1u8, 1usize), (2, 1, 1), (3, 1, 0), (2, 2, 0)] {
            e.mem.set_u32(renderer + 0x200, mode);
            e.mem.set_u8(renderer + 0x208, flag);
            e.call_log = Some(vec![]);
            e.call(0x00c2_5730, &args![0x4000u32, renderer]);
            assert_eq!(
                calls(&e, GEOMETRY_RENDER_IMMEDIATE_00A80610),
                vec![vec![0x4000, renderer]]
            );
            assert_eq!(
                calls(&e, 0x0e60_0000).len(),
                called,
                "mode {mode} flag {flag}"
            );
        }
    }

    #[test]
    fn viewer_strings_and_clone() {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        e.set_global(BS_STRIP_PARTICLE_SYSTEM_RTTI, 0x0abc_0000u32);
        for addr in [
            SYSTEM_GET_VIEWER_STRINGS_00C1B850,
            STRINGS_GROW,
            STRINGS_SET_AT,
            SYSTEM_CONSTRUCT_00C1B5C0,
            SYSTEM_COPY_MEMBERS_00C1B6F0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xa, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c2_5770, &args![0x4000u32, strings]);
        assert_eq!(
            calls(&e, SYSTEM_GET_VIEWER_STRINGS_00C1B850),
            vec![vec![0x4000, strings]]
        );
        assert!(calls(&e, STRINGS_GROW).is_empty());
        let set = calls(&e, STRINGS_SET_AT);
        assert_eq!(set[0][1], 1);
        assert_eq!(e.mem.u32(set[0][2]), 0x0abc_0001);
        let block = e.mem.alloc(0x110);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_57d0, &args![0x4000u32, 0x55u32]).u32(), block);
        assert_eq!(calls(&e, SYSTEM_CONSTRUCT_00C1B5C0), vec![vec![block]]);
        assert_eq!(e.mem.u32(block), BS_STRIP_PARTICLE_SYSTEM_VTABLE);
        assert_eq!(
            calls(&e, SYSTEM_COPY_MEMBERS_00C1B6F0),
            vec![vec![0x4000, block, 0x55]]
        );
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_57d0, &args![0x4000u32, 0x55u32]).u32(), 0);
        assert!(calls(&e, SYSTEM_CONSTRUCT_00C1B5C0).is_empty());
        assert_eq!(
            calls(&e, SYSTEM_COPY_MEMBERS_00C1B6F0),
            vec![vec![0x4000, 0, 0x55]]
        );
    }
}
