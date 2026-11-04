//! `fallout shared/distant terrain system/run_006fd010` (Xbox PDB source unit), subsystem `fallout shared/distant terrain system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00485b60` (`thiscall`, no arguments; in `tesform.cpp`): read from the object passed in.
const FORM_00485B60: u32 = 0x0048_5b60;
/// `006fcef0` (`thiscall`, `RET 4`; in `bgsterrainmanager.cpp`).
const TERRAIN_MANAGER_006FCEF0: u32 = 0x006f_cef0;
/// `006f92a0` (`thiscall`, `RET 4`; in `bgsdistanttreeblock.cpp`).
const TREE_BLOCK_006F92A0: u32 = 0x006f_92a0;
/// `00a69400` (`thiscall`, `RET 4`; in `nicullingprocess.cpp`): constructs the 0x90-byte
/// local; `00a693e0` (`thiscall`) destroys it.
const CULLING_CONSTRUCT_00A69400: u32 = 0x00a6_9400;
const CULLING_DESTROY_00A693E0: u32 = 0x00a6_93e0;
/// `0041fd00` (`thiscall`, `RET 4`): the setter the engine map names
/// `NonActorMagicCaster::SetCurrentSpell` (folded code), called on the local.
const SETTER_0041FD00: u32 = 0x0041_fd00;
/// `0045bbe0` (`thiscall`, no arguments; in `tes.cpp`), and `00a694a0` (`thiscall`,
/// `RET 4`; in `nicullingprocess.cpp`) given its result, on the local.
const TES_0045BBE0: u32 = 0x0045_bbe0;
const CULLING_00A694A0: u32 = 0x00a6_94a0;
/// `006fa6d0` (`cdecl`, two words) and `006fa860`, `006f5840` (no arguments, `ECX`
/// unused: both start by loading their own list heads `011d8624` / `011d8524`).
const BLOCKS_006FA6D0: u32 = 0x006f_a6d0;
const BLOCKS_006FA860: u32 = 0x006f_a860;
const BLOCKS_006F5840: u32 = 0x006f_5840;
/// `TES::GetWorldSpace` (`004fd3e0`, `thiscall` on the `TES` singleton at `011dea10`).
const TES_GET_WORLD_SPACE: u32 = 0x004f_d3e0;
const TES_SINGLETON: u32 = 0x011d_ea10;
/// `TESWorldSpace::GetTerrainManager` (`00586170`, `thiscall`).
const WORLD_SPACE_GET_TERRAIN_MANAGER: u32 = 0x0058_6170;
/// `00759d80` (`thiscall` on the terrain manager, returns a `bool` in `AL`).
const TERRAIN_MANAGER_00759D80: u32 = 0x0075_9d80;
/// The byte `006fd1d0` sets.
const FLAG_011F5B3C: u32 = 0x011f_5b3c;
/// `006fa920` (`cdecl`, one byte argument).
const BLOCKS_006FA920: u32 = 0x006f_a920;

// Translated from 006fd010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`, one word: an object with a vtable). Reads `00485b60` on
/// the object, then its virtual slot +0x1f4; that result goes to `006fcef0` on `this`, and
/// when that returns non-zero, `006f92a0` is called on it with the `00485b60` value.
pub fn fn_006fd010(e: &mut Engine, this: Ptr, object: Ptr) {
    let value = e.call(FORM_00485B60, &args![object]).u32();
    let slot = e.vcall(object.addr(), 0x1f4, &args![]).u32();
    let block = e.call(TERRAIN_MANAGER_006FCEF0, &args![this, slot]).u32();
    if block != 0 {
        e.call(TREE_BLOCK_006F92A0, &args![block, value]);
    }
}

// Translated from 006fd080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`, one word). Constructs a 0x90-byte local on the stack
/// (`00a69400` with 0), gives it the argument (`0041fd00`) and `0045bbe0` of the argument
/// (`00a694a0`); when the word at `this + 4` is non-zero, `006fa6d0(argument, &local)` and
/// `006f5840()` run. The local is destroyed (`00a693e0`) at the end.
// Translation incomplete: 006fd080 - the C++ exception frame (FS:[0] chain, unwind state 0 for the local) is not translated.
pub fn fn_006fd080(e: &mut Engine, this: Ptr, argument: u32) {
    e.with_stack(0x90, |e, local| {
        e.call(CULLING_CONSTRUCT_00A69400, &args![local, 0u32]);
        e.call(SETTER_0041FD00, &args![local, argument]);
        let value = e.call(TES_0045BBE0, &args![argument]).u32();
        e.call(CULLING_00A694A0, &args![local, value]);
        if e.mem.u32(this.addr().wrapping_add(4)) != 0 {
            e.call(BLOCKS_006FA6D0, &args![argument, local]);
            e.call(BLOCKS_006F5840, &args![]);
        }
        e.call(CULLING_DESTROY_00A693E0, &args![local]);
    });
}

// Translated from 006fd130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, plain `RET`): when the word at `this + 4` is non-zero, runs
/// `006fa860()` then `006f5840()`.
pub fn fn_006fd130(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr().wrapping_add(4)) != 0 {
        e.call(BLOCKS_006FA860, &args![]);
        e.call(BLOCKS_006F5840, &args![]);
    }
}

// Translated from 006fd150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments, plain `RET`; `ECX` is reloaded from the `TES` singleton): true
/// when the current world space (`TES::GetWorldSpace`) exists, has a terrain manager
/// (`TESWorldSpace::GetTerrainManager`, called twice as the exe does) and `00759d80` on
/// that manager returns true.
pub fn fn_006fd150(e: &mut Engine) -> bool {
    let tes: u32 = e.global(TES_SINGLETON);
    let world_space = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
    if world_space == 0 {
        return false;
    }
    if e.call(WORLD_SPACE_GET_TERRAIN_MANAGER, &args![world_space])
        .u32()
        == 0
    {
        return false;
    }
    let manager = e
        .call(WORLD_SPACE_GET_TERRAIN_MANAGER, &args![world_space])
        .u32();
    e.call(TERRAIN_MANAGER_00759D80, &args![manager]).u32() as u8 != 0
}

// Translated from 006fd1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): `006fd1d0(1)`.
pub fn fn_006fd1c0(e: &mut Engine) {
    fn_006fd1d0(e, 1);
}

// Translated from 006fd1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): stores the argument's low byte in the flag at `011f5b3c`.
pub fn fn_006fd1d0(e: &mut Engine, value: u8) {
    e.set_global(FLAG_011F5B3C, value);
}

// Translated from 006fd1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): `006fd1d0(0)`.
pub fn fn_006fd1e0(e: &mut Engine) {
    fn_006fd1d0(e, 0);
}

// Translated from 006fd1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): passes the argument's low byte, zero-extended, to
/// `006fa920`.
pub fn fn_006fd1f0(e: &mut Engine, value: u8) {
    e.call(BLOCKS_006FA920, &args![u32::from(value)]);
}

/// `006ff4c0` (`thiscall`, no stack words; returns nothing): called on the manager's word at
/// `+4` by [`fn_006fd060`].
const TERRAIN_006FF4C0: u32 = 0x006f_f4c0;
/// The two chunk-list walks of `bgsterrainchunk.cpp` (no arguments, no result).
const CHUNKS_006FB5D0: u32 = 0x006f_b5d0;
const CHUNKS_006FB640: u32 = 0x006f_b640;

// Translated from 006fd060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): thin wrapper, `006ff4c0` with `ECX` = the word at `+4`; the callee
/// returns nothing, so nothing is passed on.
pub fn fn_006fd060(e: &mut Engine, this: Ptr) {
    let held = e.mem.u32(this.addr() + 4);
    e.call(TERRAIN_006FF4C0, &args![held]);
}

// Translated from 006fd1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): thin wrapper, `006fb5d0` (which returns nothing).
pub fn fn_006fd1a0(e: &mut Engine) {
    e.call(CHUNKS_006FB5D0, &args![]);
}

// Translated from 006fd1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): thin wrapper, `006fb640` (which returns nothing).
pub fn fn_006fd1b0(e: &mut Engine) {
    e.call(CHUNKS_006FB640, &args![]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f_d010, fn_006fd010(Ptr, Ptr)),
        entry!(0x006f_d080, fn_006fd080(Ptr, u32)),
        entry!(0x006f_d130, fn_006fd130(Ptr)),
        entry!(0x006f_d150, fn_006fd150() -> bool),
        entry!(0x006f_d1c0, fn_006fd1c0()),
        entry!(0x006f_d1d0, fn_006fd1d0(u8)),
        entry!(0x006f_d1e0, fn_006fd1e0()),
        entry!(0x006f_d1f0, fn_006fd1f0(u8)),
        entry!(0x006f_d060, fn_006fd060(Ptr)),
        entry!(0x006f_d1a0, fn_006fd1a0()),
        entry!(0x006f_d1b0, fn_006fd1b0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_006fd010_passes_the_block_on_only_when_found() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x200);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable + 0x1f4, 0x00ab_0001);
        e.register(0x00ab_0001, |_, _| 0x77u32.into_ret());
        e.register(FORM_00485B60, |_, _| 0x55u32.into_ret());
        e.register(TERRAIN_MANAGER_006FCEF0, |_, a| {
            u32::from(a[1] == 0x77).wrapping_mul(0x9000).into_ret()
        });
        e.register(TREE_BLOCK_006F92A0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_d010, &args![0x4000u32, object]);
        assert_eq!(
            calls_to(&e, TERRAIN_MANAGER_006FCEF0),
            vec![vec![0x4000, 0x77]]
        );
        assert_eq!(calls_to(&e, TREE_BLOCK_006F92A0), vec![vec![0x9000, 0x55]]);
        e.register(TERRAIN_MANAGER_006FCEF0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_d010, &args![0x4000u32, object]);
        assert!(calls_to(&e, TREE_BLOCK_006F92A0).is_empty());
    }

    #[test]
    fn fn_006fd080_builds_its_scratch_object_and_runs_the_blocks_when_set() {
        let mut e = Engine::new();
        for callee in [
            CULLING_CONSTRUCT_00A69400,
            CULLING_DESTROY_00A693E0,
            SETTER_0041FD00,
            CULLING_00A694A0,
            BLOCKS_006FA6D0,
            BLOCKS_006F5840,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(TES_0045BBE0, |_, _| 0x66u32.into_ret());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x006f_d080, &args![this, 0x1234u32]);
        let local = calls_to(&e, CULLING_CONSTRUCT_00A69400)[0][0];
        assert_eq!(
            calls_to(&e, CULLING_CONSTRUCT_00A69400),
            vec![vec![local, 0]]
        );
        assert_eq!(calls_to(&e, SETTER_0041FD00), vec![vec![local, 0x1234]]);
        assert_eq!(calls_to(&e, TES_0045BBE0), vec![vec![0x1234]]);
        assert_eq!(calls_to(&e, CULLING_00A694A0), vec![vec![local, 0x66]]);
        assert_eq!(calls_to(&e, BLOCKS_006FA6D0), vec![vec![0x1234, local]]);
        assert_eq!(calls_to(&e, BLOCKS_006F5840).len(), 1);
        assert_eq!(calls_to(&e, CULLING_DESTROY_00A693E0), vec![vec![local]]);
        e.mem.set_u32(this + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_d080, &args![this, 0x1234u32]);
        assert!(calls_to(&e, BLOCKS_006FA6D0).is_empty());
        assert!(calls_to(&e, BLOCKS_006F5840).is_empty());
        assert_eq!(calls_to(&e, CULLING_DESTROY_00A693E0).len(), 1);
    }

    #[test]
    fn fn_006fd130_runs_both_only_when_set() {
        let mut e = Engine::new();
        e.register(BLOCKS_006FA860, |_, _| Ret::default());
        e.register(BLOCKS_006F5840, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x006f_d130, &args![this]);
        assert!(calls_to(&e, BLOCKS_006FA860).is_empty());
        e.mem.set_u32(this + 4, 5);
        e.call(0x006f_d130, &args![this]);
        assert_eq!(calls_to(&e, BLOCKS_006FA860).len(), 1);
        assert_eq!(calls_to(&e, BLOCKS_006F5840).len(), 1);
    }

    #[test]
    fn fn_006fd150_needs_world_space_manager_and_the_test() {
        let mut e = Engine::new();
        e.map(TES_SINGLETON & !0xfff, 0x1000);
        e.set_global(TES_SINGLETON, 0x7000u32);
        e.register(TES_GET_WORLD_SPACE, |_, a| {
            u32::from(a[0] == 0x7000).wrapping_mul(0x8000).into_ret()
        });
        e.register(WORLD_SPACE_GET_TERRAIN_MANAGER, |_, _| 0x9000u32.into_ret());
        e.register(TERRAIN_MANAGER_00759D80, |_, a| {
            if a[0] == 0x9000 {
                0xffff_ff01u32.into_ret()
            } else {
                Ret::default()
            }
        });
        assert!(e.call(0x006f_d150, &args![]).bool());
        e.register(TERRAIN_MANAGER_00759D80, |_, _| 0xffff_ff00u32.into_ret());
        assert!(!e.call(0x006f_d150, &args![]).bool());
        e.register(WORLD_SPACE_GET_TERRAIN_MANAGER, |_, _| Ret::default());
        assert!(!e.call(0x006f_d150, &args![]).bool());
        e.register(TES_GET_WORLD_SPACE, |_, _| Ret::default());
        assert!(!e.call(0x006f_d150, &args![]).bool());
    }

    #[test]
    fn fn_006fd1c0_and_006fd1e0_set_and_clear_the_flag() {
        let mut e = Engine::new();
        e.map(FLAG_011F5B3C & !0xfff, 0x1000);
        e.call(0x006f_d1c0, &args![]);
        assert_eq!(e.global::<u8>(FLAG_011F5B3C), 1);
        e.call(0x006f_d1e0, &args![]);
        assert_eq!(e.global::<u8>(FLAG_011F5B3C), 0);
    }

    #[test]
    fn fn_006fd1d0_stores_the_low_byte() {
        let mut e = Engine::new();
        e.map(FLAG_011F5B3C & !0xfff, 0x1000);
        e.mem.set_u32(FLAG_011F5B3C, 0xaaaa_aaaa);
        e.call(0x006f_d1d0, &args![0x1234_5607u32]);
        assert_eq!(e.mem.u32(FLAG_011F5B3C), 0xaaaa_aa07);
    }

    #[test]
    fn fn_006fd1f0_zero_extends_the_byte() {
        let mut e = Engine::new();
        e.register(BLOCKS_006FA920, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_d1f0, &args![0x1234_56ffu32]);
        assert_eq!(calls_to(&e, BLOCKS_006FA920), vec![vec![0xff]]);
    }
}

#[cfg(test)]
mod n089_tests {
    use super::*;

    #[test]
    fn n089_006fd060_calls_the_held_object() {
        let mut e = Engine::new();
        e.register(TERRAIN_006FF4C0, |_, _| 0x99u32.into_ret());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x4440);
        e.call_log = Some(vec![]);
        e.call(0x006f_d060, &args![this]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == TERRAIN_006FF4C0).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x4440]);
    }

    #[test]
    fn n089_006fd1a0_and_006fd1b0_run_the_chunk_walks() {
        let mut e = Engine::new();
        e.register(CHUNKS_006FB5D0, |_, _| Ret::default());
        e.register(CHUNKS_006FB640, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_d1a0, &args![]);
        e.call(0x006f_d1b0, &args![]);
        let log = e.call_log.take().unwrap();
        let walks: Vec<u32> = log
            .iter()
            .map(|c| c.0)
            .filter(|a| *a == CHUNKS_006FB5D0 || *a == CHUNKS_006FB640)
            .collect();
        assert_eq!(walks, vec![CHUNKS_006FB5D0, CHUNKS_006FB640]);
    }
}
