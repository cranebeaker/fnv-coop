//! `BSShader/run_00bcb460` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both functions sit in the vtable at `010bb588`, the one `GeometryDecalShader`'s
//! constructor (`00bcbf00`) installs: slot +8 (`GetRTTI`) and slot +0x64
//! (`SetupBoneMatrices` in the Xbox PDB's `BSShader` vtable).

#[allow(unused_imports)]
use crate::prelude::*;

/// The record `00bcb450` answers (`01202228`).
const RTTI_01202228: u32 = 0x0120_2228;
/// The state object the code reads at `011f9508` and its `00e6fe30` (thiscall, five words).
const STATE_OBJECT_011F9508: u32 = 0x011f_9508;
const STATE_00E6FE30: u32 = 0x00e6_fe30;
/// The word at `0126f74c` (an object whose +0x5c holds the object whose virtual slot +0x78
/// takes five words).
const RENDER_STATE_0126F74C: u32 = 0x0126_f74c;

// Translated from 00bcb450 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// `GeometryDecalShader`'s `GetRTTI` (slot +8 of the vtable at `010bb588`): the address of the
/// record at `01202228`.
pub fn geometry_decal_shader_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(RTTI_01202228)
}

// Translated from 00bcb460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GeometryDecalShader`'s `SetupBoneMatrices` (slot +0x64 of the vtable at `010bb588`;
/// thiscall, `RET 0x20`: eight words, the first and the fourth to sixth and eighth unread).
/// With non-null second and third words, no word at +0x38 and a word at +0x8c: the state
/// object's `00e6fe30` (the second word, the seventh, 0, 3, 1), then the virtual slot +0x78 of
/// the object at `[[0126f74c] + 0x5c]` with the word at +0x8c, the second word's +0x28, the
/// third word's 16-bit +0x20, 3 and the third word's +4. Answers 0.
pub fn geometry_decal_shader_setup_bone_matrices(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    first: u32,
    second: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    word: u32,
    _unused_8: u32,
) -> u32 {
    let base = this.addr();
    if first == 0 || second == 0 {
        return 0;
    }
    if e.mem.u32(base.wrapping_add(0x38)) != 0 || e.mem.u32(base.wrapping_add(0x8c)) == 0 {
        return 0;
    }
    let state = e.global::<u32>(STATE_OBJECT_011F9508);
    e.call(STATE_00E6FE30, &args![state, first, word, 0u32, 3u32, 1u32]);
    let render_state = e.global::<u32>(RENDER_STATE_0126F74C);
    let target = e.mem.u32(render_state.wrapping_add(0x5c));
    let last = e.mem.u32(second.wrapping_add(4));
    let count = u32::from(e.mem.u16(second.wrapping_add(0x20)));
    let source = e.mem.u32(first.wrapping_add(0x28));
    let own = e.mem.u32(base.wrapping_add(0x8c));
    e.vcall(target, 0x78, &args![own, source, count, 3u32, last]);
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_b450, geometry_decal_shader_get_rtti() -> Ptr),
        entry!(
            0x00bc_b460,
            geometry_decal_shader_setup_bone_matrices(
                Ptr,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> u32
        ),
    ]
}

#[cfg(test)]
mod n014_tests {
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

    #[test]
    fn get_rtti_answers_the_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00bc_b450, &args![]).u32(), RTTI_01202228);
    }

    #[test]
    fn setup_bone_matrices_hands_the_bones_on_when_everything_is_there() {
        const SLOT_78: u32 = 0x03f1_0000;
        let mut e = Engine::new();
        e.register(STATE_00E6FE30, |_, _| Ret::default());
        e.register(SLOT_78, |_, _| Ret::default());
        e.mem.map(STATE_OBJECT_011F9508, 4);
        e.mem.set_u32(STATE_OBJECT_011F9508, 0x4444);
        let vtable = e.mem.alloc(0x80);
        e.mem.set_u32(vtable + 0x78, SLOT_78);
        let target = e.mem.alloc(4);
        e.mem.set_u32(target, vtable);
        let render_state = e.mem.alloc(0x60);
        e.mem.set_u32(render_state + 0x5c, target);
        e.mem.map(RENDER_STATE_0126F74C, 4);
        e.mem.set_u32(RENDER_STATE_0126F74C, render_state);
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x8c, 0x8c8c);
        let first = e.mem.alloc(0x30);
        e.mem.set_u32(first + 0x28, 0x2828);
        let second = e.mem.alloc(0x24);
        e.mem.set_u32(second + 4, 0x0404);
        e.mem.set_u32(second + 0x20, 0x1_0020);
        e.call_log = Some(vec![]);
        let words = |first: u32, second: u32| {
            args![this, 1u32, first, second, 4u32, 5u32, 6u32, 0x77u32, 8u32]
        };
        assert_eq!(e.call(0x00bc_b460, &words(first, second)).u32(), 0);
        assert_eq!(
            calls_to(&e, STATE_00E6FE30),
            vec![vec![0x4444, first, 0x77, 0, 3, 1]]
        );
        assert_eq!(
            calls_to(&e, SLOT_78),
            vec![vec![target, 0x8c8c, 0x2828, 0x20, 3, 0x0404]]
        );
        // Any missing piece: nothing.
        e.call(0x00bc_b460, &words(0, second));
        e.call(0x00bc_b460, &words(first, 0));
        e.mem.set_u32(this + 0x38, 1);
        e.call(0x00bc_b460, &words(first, second));
        e.mem.set_u32(this + 0x38, 0);
        e.mem.set_u32(this + 0x8c, 0);
        e.call(0x00bc_b460, &words(first, second));
        assert_eq!(calls_to(&e, SLOT_78).len(), 1);
    }
}
