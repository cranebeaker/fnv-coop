//! `NiMain/nicullingprocess.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiCullingProcess::ms_RTTI` (the class's `NiRTTI` record).
const NI_CULLING_PROCESS_RTTI: u32 = 0x011f_4990;

// Translated from 00a693f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProcess::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_culling_process_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_CULLING_PROCESS_RTTI)
}

/// `NiCullingProcess`'s vtable (`0109c28c`, RTTI `.?AVNiCullingProcess@@`).
const NI_CULLING_PROCESS_VTABLE: u32 = 0x0109_c28c;
/// The frustum constructor `00a71b80` (`NiMain/nifrustum.cpp` in the engine map; `thiscall`,
/// `RET 0x1c`: six floats and a word).
const FRUSTUM_CONSTRUCT: u32 = 0x00a7_1b80;
/// `NiPlane::NiPlane` (engine map; `thiscall`, no stack words).
const PLANE_CONSTRUCT: u32 = 0x00a6_9940;

// Translated from 00a69400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProcess::NiCullingProcess` (unnamed in the engine map; the vtable it sets is
/// `NiCullingProcess`'s by its RTTI). `thiscall`, one word (`RET 4`): the visible set. Sets
/// the vtable, clears `m_bUseVirtualAppend` (+4), constructs the frustum `m_kFrustum` (+0x10)
/// with six 0.0 floats and 0 (`00a71b80`), constructs the six planes of `m_kPlanes` (+0x2c,
/// 0x10 bytes each, `NiPlane::NiPlane`), sets `m_uiActivePlanes` (+0x8c) to 0x3f, stores the
/// visible set at +8 (`m_pkVisibleSet`) and clears +0xc (`m_pkCamera`). Field names from the
/// Xbox PDB, at the PC offsets this body uses. Answers `this`.
pub fn ni_culling_process_ni_culling_process(e: &mut Engine, this: Ptr, visible_set: u32) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, NI_CULLING_PROCESS_VTABLE);
    e.mem.set_u8(at + 4, 0);
    // FLDZ, then FST float into the six float words: +0.0 each, pushed as its bit pattern.
    let zero = 0.0f32.to_bits();
    e.call(
        FRUSTUM_CONSTRUCT,
        &args![at + 0x10, zero, zero, zero, zero, zero, zero, 0u32],
    );
    let planes = at + 0x2c;
    let mut plane = planes;
    // EBP counts 5 down to 0 (`SUB 1; JNS`): six planes.
    let mut remaining: i32 = 5;
    loop {
        e.call(PLANE_CONSTRUCT, &args![plane]);
        plane += 0x10;
        remaining -= 1;
        if remaining < 0 {
            break;
        }
    }
    e.mem.set_u32(planes + 0x60, 0x3f);
    e.mem.set_u32(at + 8, visible_set);
    e.mem.set_u32(at + 0xc, 0);
    this
}

/// `NiFrustumPlanes::Set` (`00a74e10`, `thiscall` on the planes, `RET 8`: the
/// frustum and the camera's world transform).
const FRUSTUM_PLANES_SET: u32 = 0x00a7_4e10;

// Translated from 00a694a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nicullingprocess.cpp`; `thiscall`, one
/// stack word: a frustum of seven words). With a camera (`m_pkCamera`, +0xc):
/// copies the frustum into `m_kFrustum` (+0x10, `REP MOVSD` of 7 words), sets
/// the planes `m_kPlanes` (+0x2c) from it and the camera's `+0x68`
/// (`NiFrustumPlanes::Set`) and makes all six active (`m_uiActivePlanes`
/// +0x8c = 0x3f). Without a camera nothing changes.
pub fn fn_00a694a0(e: &mut Engine, this: Ptr, frustum: Ptr) {
    let t = this.addr();
    let camera = e.mem.u32(t + 0xc);
    if camera == 0 {
        return;
    }
    for i in 0..7 {
        let word = e.mem.u32(frustum.addr() + 4 * i);
        e.mem.set_u32(t + 0x10 + 4 * i, word);
    }
    e.call(
        FRUSTUM_PLANES_SET,
        &args![t + 0x2c, t + 0x10, camera + 0x68],
    );
    e.mem.set_u32(t + 0x8c, 0x3f);
}

/// `NiAVObject::GetWorldBound` (`0043d450`, `thiscall`, no stack words).
const AV_OBJECT_GET_WORLD_BOUND: u32 = 0x0043_d450;
/// `NiBound::WhichSide` (`004b6100`, `thiscall` on the bound, `RET 4`: the plane).
const BOUND_WHICH_SIDE: u32 = 0x004b_6100;

// Translated from 00a694e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProcess::DoCulling` (Xbox PDB), `thiscall`, one stack word (the
/// object). Only when the object has a bound (`object+0x20`) whose radius
/// (`+0xc`) is not 0.0 (a NaN radius goes on, as `FCOMP` with `TEST AH,0x44`
/// does): for each of the six planes (+0x2c, 0x10 bytes each) still active in
/// `m_uiActivePlanes` (+0x8c, bit per plane), asks which side of it the
/// object's world bound is (`GetWorldBound`, then `WhichSide`); side 2 stops
/// (the object is culled), side 1 drops the plane for the children. When all
/// six were looked at, the object's vtable slot `+0xd4` runs with this process.
/// The active planes are restored afterwards.
pub fn ni_culling_process_do_culling(e: &mut Engine, this: Ptr, object: Ptr) {
    let t = this.addr();
    let bound = e.mem.u32(object.addr() + 0x20);
    if bound == 0 {
        return;
    }
    if x87_m32(e.mem.u32(bound + 0xc)) == 0.0 {
        return;
    }
    let saved = e.mem.u32(t + 0x8c);
    let mut plane_index: u32 = 0;
    let mut mask: u32 = 1;
    let mut plane = t + 0x2c;
    while plane_index < 6 {
        if e.mem.u32(t + 0x8c) & mask != 0 {
            let world_bound = e.call(AV_OBJECT_GET_WORLD_BOUND, &args![object]).u32();
            let side = e.call(BOUND_WHICH_SIDE, &args![world_bound, plane]).u32();
            if side == 2 {
                break;
            }
            if side == 1 {
                let active = e.mem.u32(t + 0x8c) & !mask;
                e.mem.set_u32(t + 0x8c, active);
            }
        }
        plane_index += 1;
        plane += 0x10;
        mask = mask.rotate_left(1);
    }
    if plane_index == 6 {
        e.vcall(object.addr(), 0xd4, &args![this]);
    }
    e.mem.set_u32(t + 0x8c, saved);
}

/// The memory manager (`011f6238`) and its `Deallocate` (`00aa4060`, `thiscall`, the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00a69470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProcess::_scalar_deleting_destructor_` (Xbox PDB): sets `NiCullingProcess`'s
/// vtable and, when bit 0 of the flags byte is set, gives the block back to the memory
/// manager (`00aa4060` on `011f6238`). Answers `this`.
pub fn ni_culling_process_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_CULLING_PROCESS_VTABLE);
    if flags as u8 & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The global object (`011f4748`) whose word +8 `Process` takes as the visible set when it
/// has none; `004e9bb0` (`thiscall` on that object, the camera) runs between the two calls
/// on it.
const RENDERER_OBJECT: u32 = 0x011f_4748;
const RENDERER_UPDATE_FOR_CAMERA: u32 = 0x004e_9bb0;
/// `0066b0d0` (`thiscall` on a `NiPointer`, the new object): the counted pointer assignment.
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiAVObject::Cull` (`00a59e00`, `thiscall` on the scene, the culling process).
const AV_OBJECT_CULL: u32 = 0x00a5_9e00;
/// The import slot of `InterlockedDecrement`.
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00a69570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProcess::Process` (Xbox PDB): with a camera and a scene, makes the camera
/// current (`m_pkCamera` +0xc) and sets the frustum from the camera's at +0xdc
/// ([`fn_00a694a0`]); a given visible set replaces the one at +8 for the call. When there is
/// still none, the object at word +8 of `011f4748` is taken into a counted local pointer
/// (`0066b0d0`) and, when it is not null, its slot +0x8c runs with the camera. The scene
/// culls itself with this process (`NiAVObject::Cull`); then, with that object, `004e9bb0`
/// runs on `011f4748` with the camera and the object's slot +0x90. The previous visible set
/// comes back, the camera is cleared, and the local pointer is released (its slot +4 when
/// the count reaches 0). Nothing without a camera or a scene.
///
/// Ghidra drops the paths through the local pointer as unreachable: it does not see
/// `0066b0d0` write it.
pub fn ni_culling_process_process(
    e: &mut Engine,
    this: Ptr,
    camera: Ptr,
    scene: Ptr,
    visible_set: u32,
) {
    let t = this.addr();
    if camera.addr() == 0 || scene.addr() == 0 {
        return;
    }
    e.mem.set_u32(t + 0xc, camera.addr());
    fn_00a694a0(e, this, Ptr::new(camera.addr() + 0xdc));
    let mut saved = 0;
    if visible_set != 0 {
        saved = e.mem.u32(t + 8);
        e.mem.set_u32(t + 8, visible_set);
    }
    // The camera argument's stack word becomes the counted local pointer, set to null.
    let object = e.with_stack(4, |e, local| {
        if e.mem.u32(t + 8) == 0 {
            let renderer = e.global::<u32>(RENDERER_OBJECT);
            let candidate = e.mem.u32(renderer + 8);
            e.call(POINTER_ASSIGN, &args![local, candidate]);
            let object = e.mem.u32(local.addr());
            if object != 0 {
                e.vcall(object, 0x8c, &args![camera]);
            }
            object
        } else {
            0
        }
    });
    e.call(AV_OBJECT_CULL, &args![scene, this]);
    if object != 0 {
        let renderer = e.global::<u32>(RENDERER_OBJECT);
        e.call(RENDERER_UPDATE_FOR_CAMERA, &args![renderer, camera]);
        e.vcall(object, 0x90, &args![]);
    }
    if visible_set != 0 {
        e.mem.set_u32(t + 8, saved);
    }
    e.mem.set_u32(t + 0xc, 0);
    if object != 0 && e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00a69640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProcess::Process_ov2` (Xbox PDB): one object by its flags (+0x30). With bit 12
/// set it is skipped when bit 20 is also set and otherwise appended (its slot +0xd4 with this
/// process); with bit 12 clear it is appended when bit 11 is set or no plane is active
/// (`m_uiActivePlanes` +0x8c is 0), and culled ([`ni_culling_process_do_culling`]) otherwise.
pub fn ni_culling_process_process_ov2(e: &mut Engine, this: Ptr, object: Ptr) {
    let flags = e.mem.u32(object.addr() + 0x30);
    if (flags >> 0xc) & 1 != 0 {
        if (flags >> 0x14) & 1 != 0 {
            return;
        }
    } else if (flags >> 0xb) & 1 == 0 && e.mem.u32(this.addr() + 0x8c) != 0 {
        ni_culling_process_do_culling(e, this, object);
        return;
    }
    e.vcall(object.addr(), 0xd4, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a693f0, ni_culling_process_get_rtti() -> Ptr),
        entry!(0x00a6_9400, ni_culling_process_ni_culling_process(Ptr, u32) -> Ptr),
        entry!(0x00a6_94a0, fn_00a694a0(Ptr, Ptr)),
        entry!(0x00a6_94e0, ni_culling_process_do_culling(Ptr, Ptr)),
        entry!(0x00a6_9470, ni_culling_process_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a6_9570, ni_culling_process_process(Ptr, Ptr, Ptr, u32)),
        entry!(0x00a6_9640, ni_culling_process_process_ov2(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_culling_process_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a693f0, &args![]).u32(), NI_CULLING_PROCESS_RTTI);
    }
}

#[cfg(test)]
mod b000o_tests {
    use super::*;

    #[test]
    fn ni_culling_process_constructor_builds_frustum_and_planes() {
        let mut e = Engine::new();
        e.register(FRUSTUM_CONSTRUCT, |e, a| {
            // Runs after the vtable and the flag are set.
            assert_eq!(e.mem.u32(a[0] - 0x10), NI_CULLING_PROCESS_VTABLE);
            assert_eq!(e.mem.u8(a[0] - 0xc), 0);
            a[0].into_ret()
        });
        e.register(PLANE_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x90);
        e.mem.write(this, &[0xcd; 0x90]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_9400, &args![this, 0x5150u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let frustum: Vec<_> = log.iter().filter(|c| c.0 == FRUSTUM_CONSTRUCT).collect();
        assert_eq!(frustum.len(), 1);
        assert_eq!(frustum[0].1, vec![this + 0x10, 0, 0, 0, 0, 0, 0, 0]);
        let planes: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == PLANE_CONSTRUCT)
            .map(|c| c.1[0])
            .collect();
        let expected: Vec<u32> = (0..6).map(|i| this + 0x2c + 0x10 * i).collect();
        assert_eq!(planes, expected);
        assert_eq!(e.mem.u32(this), NI_CULLING_PROCESS_VTABLE);
        assert_eq!(e.mem.u8(this + 4), 0);
        assert_eq!(e.mem.u8(this + 5), 0xcd);
        assert_eq!(e.mem.u32(this + 8), 0x5150);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x8c), 0x3f);
    }
}

#[cfg(test)]
mod tests_b008 {
    use super::*;

    #[test]
    fn fn_00a694a0_sets_the_frustum_only_with_a_camera() {
        let mut e = Engine::new();
        e.register(FRUSTUM_PLANES_SET, |_, _| Ret::default());
        let this = e.mem.alloc(0x90);
        let frustum = e.mem.alloc(0x1c);
        for i in 0..7 {
            e.mem.set_u32(frustum + 4 * i, 0x100 + i);
        }
        e.call_log = Some(vec![]);
        e.call(0x00a6_94a0, &args![this, frustum]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        e.mem.set_u32(this + 0xc, 0x4000);
        e.call_log = Some(vec![]);
        e.call(0x00a6_94a0, &args![this, frustum]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (FRUSTUM_PLANES_SET, vec![this + 0x2c, this + 0x10, 0x4068])
        );
        for i in 0..7 {
            assert_eq!(e.mem.u32(this + 0x10 + 4 * i), 0x100 + i);
        }
        assert_eq!(e.mem.u32(this + 0x8c), 0x3f);
    }

    const ON_VISIBLE: u32 = 0x0123_7000;

    /// Runs `DoCulling` with the planes `active` and the sides `sides` (one per
    /// plane asked); answers the planes asked, whether the object's slot ran,
    /// the active mask seen during that slot and the mask afterwards.
    fn cull(radius: f32, active: u32, sides: Vec<u32>) -> (Vec<u32>, Option<u32>, u32) {
        let mut e = Engine::new();
        e.register(AV_OBJECT_GET_WORLD_BOUND, |_, _| 0x8888u32.into_ret());
        let mut sides = sides.into_iter();
        e.register_double(BOUND_WHICH_SIDE, move |_, _| {
            sides.next().unwrap().into_ret()
        });
        e.register_double(ON_VISIBLE, |e, a| {
            let mask = e.mem.u32(a[1] + 0x8c);
            e.mem.set_u32(0x0123_8000, mask);
            Ret::default()
        });
        e.map(0x0123_8000, 4);
        e.mem.set_u32(0x0123_8000, 0xdead);
        let vtable = e.mem.alloc(0xd8);
        e.mem.set_u32(vtable + 0xd4, ON_VISIBLE);
        let bound = e.mem.alloc(0x10);
        e.mem.set_f32(bound + 0xc, radius);
        let object = e.mem.alloc(0x24);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 0x20, bound);
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x8c, active);
        e.call_log = Some(vec![]);
        e.call(0x00a6_94e0, &args![this, object]);
        let planes = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == BOUND_WHICH_SIDE)
            .map(|c| (c.1[1] - this - 0x2c) / 0x10)
            .collect();
        let seen = e.mem.u32(0x0123_8000);
        assert_eq!(e.mem.u32(this + 0x8c), active);
        (planes, (seen != 0xdead).then_some(seen), active)
    }

    #[test]
    fn ni_culling_process_do_culling_tests_the_active_planes() {
        let (planes, seen, _) = cull(1.0, 0b10_0101, vec![0, 1, 0]);
        assert_eq!(planes, vec![0, 2, 5]);
        assert_eq!(seen, Some(0b10_0001));
    }

    #[test]
    fn ni_culling_process_do_culling_stops_on_side_2_and_skips_flat_bounds() {
        let (planes, seen, _) = cull(1.0, 0x3f, vec![0, 2]);
        assert_eq!(planes, vec![0, 1]);
        assert_eq!(seen, None);
        let (planes, seen, _) = cull(0.0, 0x3f, vec![]);
        assert!(planes.is_empty());
        assert_eq!(seen, None);
        let (planes, seen, _) = cull(f32::NAN, 0, vec![]);
        assert!(planes.is_empty());
        assert_eq!(seen, Some(0));
    }
}

#[cfg(test)]
mod n015_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::calls_to;

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_9470, &args![this, 0x100u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_CULLING_PROCESS_VTABLE);
        assert!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
        assert_eq!(e.call(0x00a6_9470, &args![this, 3u32]).u32(), this);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }

    const SLOT_8C: u32 = 0x0123_1000;
    const SLOT_90: u32 = 0x0123_1004;
    const SLOT_DELETE: u32 = 0x0123_1008;

    /// An engine for `Process`: doubles for every callee, the renderer object at `011f4748`
    /// holding `held` at +8 (a counted object with count 1 when not 0).
    fn process_engine(held: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.register(FRUSTUM_PLANES_SET, |_, _| Ret::default());
        e.register(AV_OBJECT_CULL, |e, a| {
            // The visible set and camera seen while the scene culls.
            let seen = e.mem.u32(a[1] + 8);
            e.mem.set_u32(0x011f_4f00, seen);
            let camera = e.mem.u32(a[1] + 0xc);
            e.mem.set_u32(0x011f_4f04, camera);
            Ret::default()
        });
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            if a[1] != 0 {
                let count = e.mem.u32(a[1] + 4) + 1;
                e.mem.set_u32(a[1] + 4, count);
            }
            a[0].into_ret()
        });
        e.register(RENDERER_UPDATE_FOR_CAMERA, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        for slot in [SLOT_8C, SLOT_90, SLOT_DELETE] {
            e.register(slot, |_, _| Ret::default());
        }
        let renderer = e.mem.alloc(0x10);
        e.mem.set_u32(RENDERER_OBJECT, renderer);
        let mut object = 0;
        if held {
            let vtable = e.mem.alloc(0x100);
            e.mem.set_u32(vtable + 4, SLOT_DELETE);
            e.mem.set_u32(vtable + 0x8c, SLOT_8C);
            e.mem.set_u32(vtable + 0x90, SLOT_90);
            object = e.mem.alloc(0x10);
            e.mem.set_u32(object, vtable);
            e.mem.set_u32(object + 4, 1);
            e.mem.set_u32(renderer + 8, object);
        }
        e.call_log = Some(vec![]);
        (e, renderer, object)
    }

    #[test]
    fn process_with_a_given_visible_set_restores_the_old_one() {
        let (mut e, _, _) = process_engine(false);
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 8, 0x7777);
        let camera = e.mem.alloc(0x100);
        e.call(0x00a6_9570, &args![this, camera, 0x5000u32, 0x6000u32]);
        assert_eq!(calls_to(&e, AV_OBJECT_CULL), vec![vec![0x5000, this]]);
        assert_eq!(e.mem.u32(0x011f_4f00), 0x6000);
        assert_eq!(e.mem.u32(0x011f_4f04), camera);
        assert!(calls_to(&e, POINTER_ASSIGN).is_empty());
        assert_eq!(
            calls_to(&e, FRUSTUM_PLANES_SET),
            vec![vec![this + 0x2c, this + 0x10, camera + 0x68]]
        );
        assert_eq!(e.mem.u32(this + 8), 0x7777);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn process_without_a_visible_set_takes_the_renderer_object() {
        let (mut e, renderer, object) = process_engine(true);
        let this = e.mem.alloc(0x90);
        let camera = e.mem.alloc(0x100);
        e.call(0x00a6_9570, &args![this, camera, 0x5000u32, 0u32]);
        assert_eq!(calls_to(&e, POINTER_ASSIGN)[0][1], object);
        assert_eq!(calls_to(&e, SLOT_8C), vec![vec![object, camera]]);
        assert_eq!(
            calls_to(&e, RENDERER_UPDATE_FOR_CAMERA),
            vec![vec![renderer, camera]]
        );
        assert_eq!(calls_to(&e, SLOT_90), vec![vec![object]]);
        // Taken and released: the count is back to 1, nothing deleted.
        assert_eq!(e.mem.u32(object + 4), 1);
        assert!(calls_to(&e, SLOT_DELETE).is_empty());
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // The last reference deletes it.
        e.mem.set_u32(object + 4, 0);
        e.call(0x00a6_9570, &args![this, camera, 0x5000u32, 0u32]);
        assert_eq!(calls_to(&e, SLOT_DELETE), vec![vec![object]]);
    }

    #[test]
    fn process_with_no_object_and_no_camera() {
        let (mut e, _, _) = process_engine(false);
        let this = e.mem.alloc(0x90);
        let camera = e.mem.alloc(0x100);
        e.call(0x00a6_9570, &args![this, camera, 0x5000u32, 0u32]);
        assert_eq!(calls_to(&e, POINTER_ASSIGN)[0][1], 0);
        assert!(calls_to(&e, RENDERER_UPDATE_FOR_CAMERA).is_empty());
        assert_eq!(calls_to(&e, AV_OBJECT_CULL).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x00a6_9570, &args![this, 0u32, 0x5000u32, 0u32]);
        e.call(0x00a6_9570, &args![this, camera, 0u32, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn process_ov2_chooses_by_the_object_flags() {
        let mut e = Engine::new();
        e.register(SLOT_90, |_, _| Ret::default());
        e.register(AV_OBJECT_GET_WORLD_BOUND, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0xd4, SLOT_90);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x8c, 0x3f);
        let run = |e: &mut Engine, flags: u32| {
            e.mem.set_u32(object + 0x30, flags);
            e.call_log = Some(vec![]);
            e.call(0x00a6_9640, &args![this, object]);
            calls_to(e, SLOT_90).len()
        };
        assert_eq!(run(&mut e, 1 << 12), 1);
        assert_eq!(run(&mut e, (1 << 12) | (1 << 20)), 0);
        assert_eq!(run(&mut e, 1 << 11), 1);
        // Culled: the object has no bound, so nothing is appended.
        assert_eq!(run(&mut e, 0), 0);
        assert!(calls_to(&e, AV_OBJECT_GET_WORLD_BOUND).is_empty());
        e.mem.set_u32(this + 0x8c, 0);
        assert_eq!(run(&mut e, 0), 1);
    }
}
