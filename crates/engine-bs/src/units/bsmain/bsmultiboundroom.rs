//! `BSMain/bsmultiboundroom.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c3a140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundRoom` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00c39770` and frees 0xec bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_multi_bound_room_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x00c3_9770, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xecu32]);
    }
    this
}

// Translated from 00c3a610 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202828 without reading the receiver. The map names this body `BSMultiBoundRoom::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c3a610(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2828
}

/// `BSMultiBoundNode::BSMultiBoundNode` (engine map, `00c46970`; `thiscall`, no stack words).
const MULTI_BOUND_NODE_CONSTRUCT: u32 = 0x00c4_6970;
/// `BSMultiBoundRoom`'s vtable (RTTI `.?AVBSMultiBoundRoom@@`).
const MULTI_BOUND_ROOM_VTABLE: u32 = 0x010c_1064;
/// `BSSimpleArray<ShadowSceneLight*,1024>::_InitialSetup` (engine map, `00c3a220`;
/// `thiscall`, no stack words): the array's constructor.
const LIGHT_ARRAY_CONSTRUCT: u32 = 0x00c3_a220;
/// `BSMultiBoundRoom` (Xbox PDB) on PC, 0x2C lower than the PDB's offsets (as
/// `BSMultiBoundNode::spMultiBound`, PDB +0xD0, is at +0xAC): `kPortalList` +0xB4,
/// `lOccluderList` +0xC0, `pCompoundFrustum` +0xCC, `kJoinedMultiBoundList` +0xD0 (each list
/// an `NiTPointerList` of head, tail and count) and `kLights` +0xDC; `NiAVObject::m_uFlags`
/// at +0x30.
const PORTAL_LIST: u32 = 0xb4;
const OCCLUDER_LIST: u32 = 0xc0;
const COMPOUND_FRUSTUM: u32 = 0xcc;
const JOINED_MULTI_BOUND_LIST: u32 = 0xd0;
const LIGHTS: u32 = 0xdc;
const AV_OBJECT_FLAGS: u32 = 0x30;

/// An inlined `NiTPointerList` constructor: count (+8), head and tail cleared in the exe's
/// order.
fn clear_pointer_list(e: &mut Engine, list: u32) {
    e.mem.set_u32(list + 8, 0);
    e.mem.set_u32(list, 0);
    e.mem.set_u32(list + 4, 0);
}

// Translated from 00c39640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c39640 - the compiler's exception frame (FS:[0] chain, handler 00f2da62, states 0 to 3) is not translated
/// `BSMultiBoundRoom::BSMultiBoundRoom` (engine map; `thiscall`, no stack words): the
/// `BSMultiBoundNode` base (`00c46970`), the vtable, the three pointer lists cleared, the
/// light array constructed (`00c3a220`), flag `0x800` of `m_uFlags` set (the compiler kept a
/// test of the constant 1 here; its clearing branch never runs) and `pCompoundFrustum`
/// cleared. Returns `this`.
pub fn bs_multi_bound_room_bs_multi_bound_room(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MULTI_BOUND_NODE_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, MULTI_BOUND_ROOM_VTABLE);
    clear_pointer_list(e, base + PORTAL_LIST);
    clear_pointer_list(e, base + OCCLUDER_LIST);
    clear_pointer_list(e, base + JOINED_MULTI_BOUND_LIST);
    e.call(LIGHT_ARRAY_CONSTRUCT, &args![base + LIGHTS]);
    // `MOV ECX,1; TEST ECX,ECX; JZ`: the set branch always runs; the clear branch is the
    // exe's unreachable one (`black_box` keeps it as written).
    let flags = e.mem.u32(base + AV_OBJECT_FLAGS);
    if std::hint::black_box(1u32) != 0 {
        e.mem.set_u32(base + AV_OBJECT_FLAGS, flags | 0x800);
    } else {
        e.mem.set_u32(base + AV_OBJECT_FLAGS, !0x800 & flags);
    }
    e.mem.set_u32(base + COMPOUND_FRUSTUM, 0);
    this
}

/// More `BSMultiBoundRoom` fields on PC (the same 0x2c shift): `spMultiBound` +0xAC, the
/// portal list's count +0xBC, the joined list's count +0xD8; `NiNode::m_kChildren`'s
/// elements +0xA0, size +0xA6 and used count +0xA8; `CullingMode` +0xB0;
/// `NiAVObject::m_pWorldBound` +0x20.
const MULTI_BOUND: u32 = 0xac;
const PORTAL_COUNT: u32 = 0xbc;
const JOINED_COUNT: u32 = 0xd8;
const CHILDREN: u32 = 0xa0;
const CHILD_COUNT: u32 = 0xa6;
const CHILDREN_USED: u32 = 0xa8;
const CULLING_MODE: u32 = 0xb0;
const WORLD_BOUND: u32 = 0x20;
/// The culling process's mode (+0x90) and the frustum word it culls with (+0xc0).
const CULLER_MODE: u32 = 0x90;
const CULLER_FRUSTUM: u32 = 0xc0;
/// The member destructors `00c39770` runs: the portal list's (`00b71300`), the occluder
/// list's (`004ed900`) and the joined list's (`00c3a2e0`) remove-all, `00b5af80` on the
/// light array with 1, `0045c520` on the room when it has a compound frustum, and the
/// `BSMultiBoundNode` destructor body (`00c46af0`); all `thiscall`.
const PORTAL_LIST_REMOVE_ALL: u32 = 0x00b7_1300;
const OCCLUDER_LIST_REMOVE_ALL: u32 = 0x004e_d900;
const JOINED_LIST_REMOVE_ALL: u32 = 0x00c3_a2e0;
const LIGHT_ARRAY_00B5AF80: u32 = 0x00b5_af80;
const ROOM_0045C520: u32 = 0x0045_c520;
const MULTI_BOUND_NODE_DESTRUCTOR: u32 = 0x00c4_6af0;
/// `BSCompoundFrustum::PrethreadOpList` (`00c49850`, `thiscall`) and `NiAVObject::Cull`
/// (`00a59e00`, `thiscall`, one word).
const COMPOUND_FRUSTUM_PRETHREAD: u32 = 0x00c4_9850;
const NI_AV_OBJECT_CULL: u32 = 0x00a5_9e00;
/// The occluder list's node allocation (`0043a010`, `thiscall` on the list's +8) and its
/// insertion (`00559a70`, `thiscall` on the list, one word: the node), and `00c3a250`
/// (`thiscall` on the list, one word: the address of the item).
const OCCLUDER_NODE_ALLOCATE: u32 = 0x0043_a010;
const OCCLUDER_NODE_INSERT: u32 = 0x0055_9a70;
const OCCLUDER_LIST_00C3A250: u32 = 0x00c3_a250;
/// `NiObject::IsEqual` (`00a5d470`) and `BSMultiBoundNode::GetViewerStrings` (`00c46e70`),
/// `thiscall`, one word.
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const MULTI_BOUND_NODE_GET_VIEWER_STRINGS: u32 = 0x00c4_6e70;
/// `NiGetViewerString` (`00a7d330`, cdecl) and the viewer strings array's add (`0096a610`,
/// `thiscall`, the address of the string).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRINGS_ADD: u32 = 0x0096_a610;
/// `BSMultiBoundRoom`'s `NiRTTI` (`01202828`, the address `00c3a610` answers).
const BS_MULTI_BOUND_ROOM_RTTI: u32 = 0x0120_2828;
/// `005467e0` (`thiscall`, run on the room before its world bound is set), `NiBound::Merge`
/// (`00a7f3f0`, `thiscall`, one word), the bound used for a child without one (`011f4288`)
/// and the `double` radii are compared with (`01012060`).
const NODE_005467E0: u32 = 0x0054_67e0;
const NI_BOUND_MERGE: u32 = 0x00a7_f3f0;
const EMPTY_BOUND: u32 = 0x011f_4288;
const RADIUS_ZERO: u32 = 0x0101_2060;
/// `BSSimpleArray<ShadowSceneLight*,1024>`'s vtable (`010c117c`).
const LIGHT_ARRAY_VTABLE: u32 = 0x010c_117c;

// Translation incomplete: 00c39770 - the C++ exception frame (FS:[0] chain, unwind states 0 to 7, with its /GS cookie word) is not translated.
// Translated from 00c39770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSMultiBoundRoom` destructor body (`thiscall`, no stack
/// words). The vtable (`010c1064`); the portal, occluder and joined lists emptied
/// (`00b71300`, `004ed900`, `00c3a2e0`); `00b5af80(1)` on the light array; `0045c520` on the
/// room when `pCompoundFrustum` is set; then the members' destructors in reverse order: the
/// light array's (`00c3a620`), the three remove-alls again (joined, occluder, portal), and
/// the `BSMultiBoundNode` destructor body (`00c46af0`).
pub fn fn_00c39770(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, MULTI_BOUND_ROOM_VTABLE);
    e.call(PORTAL_LIST_REMOVE_ALL, &args![base + PORTAL_LIST]);
    e.call(OCCLUDER_LIST_REMOVE_ALL, &args![base + OCCLUDER_LIST]);
    e.call(
        JOINED_LIST_REMOVE_ALL,
        &args![base + JOINED_MULTI_BOUND_LIST],
    );
    e.call(LIGHT_ARRAY_00B5AF80, &args![base + LIGHTS, 1u32]);
    if e.mem.u32(base + COMPOUND_FRUSTUM) != 0 {
        e.call(ROOM_0045C520, &args![this]);
    }
    fn_00c3a620(e, Ptr::new(base + LIGHTS));
    e.call(
        JOINED_LIST_REMOVE_ALL,
        &args![base + JOINED_MULTI_BOUND_LIST],
    );
    e.call(OCCLUDER_LIST_REMOVE_ALL, &args![base + OCCLUDER_LIST]);
    e.call(PORTAL_LIST_REMOVE_ALL, &args![base + PORTAL_LIST]);
    e.call(MULTI_BOUND_NODE_DESTRUCTOR, &args![this]);
}

/// Whether the culler's mode (+0x90, read again for each test, as the exe does) is 3 or 1.
fn culler_passes_all(e: &Engine, culler: u32) -> bool {
    e.mem.u32(culler + CULLER_MODE) == 3 || e.mem.u32(culler + CULLER_MODE) == 1
}

// Translated from 00c39890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundRoom::OnVisible` (Xbox PDB; `thiscall`, one word: the culling process).
/// Nothing for a room without used children and portals (+0xa8, +0xbc), or when the
/// multi-bound's shape state (+8 of `spMultiBound`'s +0xc) is not 1 and the culler's mode
/// (+0x90) is neither 3 nor 1. Otherwise, keeping the culler's frustum word (+0xc0) and mode:
/// a mode other than 3 becomes the room's `CullingMode` (+0xb0); when that is neither 3 nor
/// 1, `pCompoundFrustum` (+0xcc) becomes the culler's frustum (`PrethreadOpList` run on it
/// before and after, when set). Every child is culled (`NiAVObject::Cull`), then the mode is
/// restored and, when it is neither 3 nor 1, the frustum word too.
pub fn bs_multi_bound_room_on_visible(e: &mut Engine, this: Ptr, culler: Ptr) {
    let room = this.addr();
    let process = culler.addr();
    if e.mem.u16(room + CHILDREN_USED) == 0 && e.mem.u32(room + PORTAL_COUNT) == 0 {
        return;
    }
    let shape = e.mem.u32(e.mem.u32(room + MULTI_BOUND) + 0xc);
    let mut state = e.mem.u32(shape + 8);
    if culler_passes_all(e, process) {
        state = 1;
    }
    if state != 1 && !culler_passes_all(e, process) {
        return;
    }
    let saved_frustum = e.mem.u32(process + CULLER_FRUSTUM);
    let frustum = e.mem.u32(room + COMPOUND_FRUSTUM);
    let saved_mode = e.mem.u32(process + CULLER_MODE);
    if e.mem.u32(process + CULLER_MODE) != 3 {
        let mode = e.mem.u32(room + CULLING_MODE);
        e.mem.set_u32(process + CULLER_MODE, mode);
    }
    if !culler_passes_all(e, process) {
        if frustum != 0 {
            e.call(COMPOUND_FRUSTUM_PRETHREAD, &args![frustum]);
        }
        e.mem.set_u32(process + CULLER_FRUSTUM, frustum);
        if frustum != 0 {
            e.call(COMPOUND_FRUSTUM_PRETHREAD, &args![frustum]);
        }
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(room + CHILD_COUNT)) {
        let child = e.mem.u32(e.mem.u32(room + CHILDREN) + 4 * index);
        if child != 0 {
            e.call(NI_AV_OBJECT_CULL, &args![child, culler]);
        }
        index += 1;
    }
    e.mem.set_u32(process + CULLER_MODE, saved_mode);
    if !culler_passes_all(e, process) {
        e.mem.set_u32(process + CULLER_FRUSTUM, saved_frustum);
    }
}

// Translated from 00c39a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): adds a non-null `occluder` to the
/// occluder list (+0xc0): a node from `0043a010` on the list's +8, its +8 set to the
/// occluder, inserted by `00559a70`.
pub fn fn_00c39a80(e: &mut Engine, this: Ptr, occluder: u32) {
    if occluder == 0 {
        return;
    }
    let list = this.addr() + OCCLUDER_LIST;
    let node = e.call(OCCLUDER_NODE_ALLOCATE, &args![list + 8]).u32();
    e.mem.set_u32(node + 8, occluder);
    e.call(OCCLUDER_NODE_INSERT, &args![list, node]);
}

// Translated from 00c39ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): for a non-null `occluder`, `00c3a250`
/// on the occluder list (+0xc0) with the address of the argument (here a stack word holding
/// it).
pub fn fn_00c39ad0(e: &mut Engine, this: Ptr, occluder: u32) {
    if occluder == 0 {
        return;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), occluder);
        e.call(
            OCCLUDER_LIST_00C3A250,
            &args![this.addr() + OCCLUDER_LIST, slot],
        );
    });
}

// Translated from 00c39b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundRoom::QPointWithin` (Xbox PDB; `thiscall`, one word: the point): the
/// multi-bound's slot +0x8c (`AL`); while that is false and the joined list (+0xd0, count
/// +0xd8) has entries, each joined multi-bound's slot +0x8c in list order until one is true.
pub fn bs_multi_bound_room_q_point_within(e: &mut Engine, this: Ptr, point: Ptr) -> bool {
    let room = this.addr();
    let multi_bound = e.mem.u32(room + MULTI_BOUND);
    let mut within = e.vcall(multi_bound, 0x8c, &args![point]).bool();
    if within || e.mem.u32(room + JOINED_COUNT) == 0 {
        return within;
    }
    let mut node = e.mem.u32(room + JOINED_MULTI_BOUND_LIST);
    while !within && node != 0 {
        let joined = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        within = e.vcall(joined, 0x8c, &args![point]).bool();
    }
    within
}

/// `CheckBound`'s shared shape: the multi-bound's shape slot `slot`; while that is not 1
/// and the joined list has entries, each joined multi-bound's shape slot in turn, a nonzero
/// answer replacing the result, until it is 1.
fn check_bound_with_joined(e: &mut Engine, this: Ptr, bound: Ptr, slot: u32) -> u32 {
    let room = this.addr();
    let shape = e.mem.u32(e.mem.u32(room + MULTI_BOUND) + 0xc);
    let mut result = e.vcall(shape, slot, &args![bound]).u32();
    if result == 1 || e.mem.u32(room + JOINED_COUNT) == 0 {
        return result;
    }
    let mut node = e.mem.u32(room + JOINED_MULTI_BOUND_LIST);
    while result != 1 && node != 0 {
        let joined = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        let shape = e.mem.u32(joined + 0xc);
        let answer = e.vcall(shape, slot, &args![bound]).u32();
        if answer != 0 {
            result = answer;
        }
    }
    result
}

// Translated from 00c39bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundRoom::CheckBound` (Xbox PDB; `thiscall`, one word): the multi-bound's shape
/// slot +0x98 with the word; while the answer is not 1, each joined multi-bound's shape slot
/// +0x98 in list order, a nonzero answer replacing it.
pub fn bs_multi_bound_room_check_bound(e: &mut Engine, this: Ptr, bound: Ptr) -> u32 {
    check_bound_with_joined(e, this, bound, 0x98)
}

// Translated from 00c39c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundRoom::CheckBound` (Xbox PDB, the other overload; `thiscall`, one word): as
/// `00c39bb0` with the shapes' slot +0x94.
pub fn bs_multi_bound_room_check_bound_ov2(e: &mut Engine, this: Ptr, bound: Ptr) -> u32 {
    check_bound_with_joined(e, this, bound, 0x94)
}

/// Whether a child has a world bound whose radius (`FLD`/`FSTP` copy) is not the `double`
/// at `01012060` (`FCOMP; TEST AH,0x44; JNP`: unordered counts as different).
fn child_has_radius(e: &Engine, child: u32) -> bool {
    let bound = e.mem.u32(child + WORLD_BOUND);
    if bound == 0 {
        return false;
    }
    let radius = x87_load32(x87_quiet_bits(e.mem.u32(bound + 0xc)));
    radius != f64::from_bits(e.mem.u64(RADIUS_ZERO))
}

// Translated from 00c39d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundRoom::UpdateWorldBound` (Xbox PDB; `thiscall`). With a multi-bound that has
/// a shape: `005467e0` on the room, the shape's slot +0xa4 sets the world bound, and every
/// joined multi-bound's shape writes its bound (slot +0xa4) into a stack `NiBound` that is
/// merged into the world bound (`NiBound::Merge`). Otherwise as
/// `BSMultiBoundNode::UpdateWorldBound`: the radius cleared when there is a world bound and
/// flag 0x80000 is set, the first child with a non-zero radius copied in (after
/// `005467e0`), every later one merged. A child without a world bound would use the bound
/// at `011f4288` (it never qualifies, as the exe tests it first).
pub fn bs_multi_bound_room_update_world_bound(e: &mut Engine, this: Ptr) {
    let room = this.addr();
    let multi_bound = e.mem.u32(room + MULTI_BOUND);
    if multi_bound != 0 {
        let shape = e.mem.u32(e.mem.u32(room + MULTI_BOUND) + 0xc);
        if shape != 0 {
            e.call(NODE_005467E0, &args![this]);
            let world = e.mem.u32(room + WORLD_BOUND);
            e.vcall(shape, 0xa4, &args![world]);
            if e.mem.u32(room + JOINED_COUNT) == 0 {
                return;
            }
            let mut node = e.mem.u32(room + JOINED_MULTI_BOUND_LIST);
            e.with_stack(0x10, |e, bound| {
                while node != 0 {
                    let joined = e.mem.u32(node + 8);
                    node = e.mem.u32(node);
                    let shape = e.mem.u32(joined + 0xc);
                    e.vcall(shape, 0xa4, &args![bound]);
                    let world = e.mem.u32(room + WORLD_BOUND);
                    e.call(NI_BOUND_MERGE, &args![world, bound]);
                }
            });
            return;
        }
    }
    let world = e.mem.u32(room + WORLD_BOUND);
    if world != 0 && e.mem.u32(room + AV_OBJECT_FLAGS) & 0x8_0000 != 0 {
        e.mem.set_u32(world + 0xc, x87_store32(0.0));
    }
    let count = u32::from(e.mem.u16(room + CHILD_COUNT));
    let bound_of = |e: &Engine, child: u32| {
        let bound = e.mem.u32(child + WORLD_BOUND);
        if bound == 0 {
            EMPTY_BOUND
        } else {
            bound
        }
    };
    let mut index = 0u32;
    while index < count {
        let child = e.mem.u32(e.mem.u32(room + CHILDREN) + 4 * index);
        index += 1;
        if child != 0 && child_has_radius(e, child) {
            e.call(NODE_005467E0, &args![this]);
            let source = bound_of(e, child);
            let world = e.mem.u32(room + WORLD_BOUND);
            let radius = x87_quiet_bits(e.mem.u32(source + 0xc));
            for word in 0..3 {
                let value = e.mem.u32(source + 4 * word);
                e.mem.set_u32(world + 4 * word, value);
            }
            e.mem.set_u32(world + 0xc, x87_quiet_bits(radius));
            break;
        }
    }
    while index < count {
        let child = e.mem.u32(e.mem.u32(room + CHILDREN) + 4 * index);
        if child != 0 && child_has_radius(e, child) {
            let source = bound_of(e, child);
            let world = e.mem.u32(room + WORLD_BOUND);
            e.call(NI_BOUND_MERGE, &args![world, source]);
        }
        index += 1;
    }
}

// Translated from 00c3a080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x48 of the `BSMultiBoundRoom` vtable (where
/// `BSMultiBoundNode` has `CreateClone`) (`thiscall`, one word it does not read): answers 0.
pub fn fn_00c3a080(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u32 {
    0
}

// Translated from 00c3a090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x5c of the `BSMultiBoundRoom` vtable (`IsEqual` there in
/// `BSMultiBoundNode`'s) (`thiscall`, one word): `NiObject::IsEqual` with the other object;
/// its `EAX` passes through unchanged.
pub fn fn_00c3a090(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).u32()
}

// Translated from 00c3a0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundRoom::GetViewerStrings` (Xbox PDB; `thiscall`, one word: the strings
/// array): `BSMultiBoundNode::GetViewerStrings`, then the class name (`NiGetViewerString` of
/// the `NiRTTI` name at `01202828`) added (`0096a610`).
pub fn bs_multi_bound_room_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(MULTI_BOUND_NODE_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(BS_MULTI_BOUND_ROOM_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, local]);
    });
}

// Translated from 00c3a100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x50 of the `BSMultiBoundRoom` vtable (`LinkObject` there
/// in `BSMultiBoundNode`'s) (`thiscall`, one word it does not read): does nothing.
pub fn fn_00c3a100(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00c3a110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x4c of the `BSMultiBoundRoom` vtable (`LoadBinary` there
/// in `BSMultiBoundNode`'s) (`thiscall`, one word it does not read): does nothing.
pub fn fn_00c3a110(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00c3a120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x58 of the `BSMultiBoundRoom` vtable (`SaveBinary` there
/// in `BSMultiBoundNode`'s) (`thiscall`, one word it does not read): does nothing.
pub fn fn_00c3a120(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00c3a130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x54 of the `BSMultiBoundRoom` vtable
/// (`RegisterStreamables` there in `BSMultiBoundNode`'s) (`thiscall`, one word it does not
/// read): answers true (`AL` 1).
pub fn fn_00c3a130(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> bool {
    true
}

// Translated from 00c3a620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<ShadowSceneLight*,1024>` destructor body
/// (`thiscall`, no stack words): its vtable (`010c117c`), then `00b5af80(1)` on it.
pub fn fn_00c3a620(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), LIGHT_ARRAY_VTABLE);
    e.call(LIGHT_ARRAY_00B5AF80, &args![this, 1u32]);
}

// Translated from 00c3a6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<ShadowSceneLight*,1024>::_scalar_deleting_destructor_` (Xbox PDB;
/// `thiscall`, one word of flags): the destructor body `00c3a620`, then with flag bit 0 the
/// object given back to the memory manager (`00aa4060`). Answers `this`.
pub fn bs_simple_array_shadow_scene_light_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00c3a620(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The memory manager's deallocation (`00aa4060`, `thiscall`, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_9770, fn_00c39770(Ptr)),
        entry!(0x00c3_9890, bs_multi_bound_room_on_visible(Ptr, Ptr)),
        entry!(0x00c3_9a80, fn_00c39a80(Ptr, u32)),
        entry!(0x00c3_9ad0, fn_00c39ad0(Ptr, u32)),
        entry!(0x00c3_9b00, bs_multi_bound_room_q_point_within(Ptr, Ptr) -> bool),
        entry!(0x00c3_9bb0, bs_multi_bound_room_check_bound(Ptr, Ptr) -> u32),
        entry!(0x00c3_9c80, bs_multi_bound_room_check_bound_ov2(Ptr, Ptr) -> u32),
        entry!(0x00c3_9d50, bs_multi_bound_room_update_world_bound(Ptr)),
        entry!(0x00c3_a080, fn_00c3a080(Ptr, u32) -> u32),
        entry!(0x00c3_a090, fn_00c3a090(Ptr, Ptr) -> u32),
        entry!(
            0x00c3_a0c0,
            bs_multi_bound_room_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c3_a100, fn_00c3a100(Ptr, u32)),
        entry!(0x00c3_a110, fn_00c3a110(Ptr, u32)),
        entry!(0x00c3_a120, fn_00c3a120(Ptr, u32)),
        entry!(0x00c3_a130, fn_00c3a130(Ptr, u32) -> bool),
        entry!(0x00c3_a620, fn_00c3a620(Ptr)),
        entry!(
            0x00c3_a6a0,
            bs_simple_array_shadow_scene_light_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c3_9640, bs_multi_bound_room_bs_multi_bound_room(Ptr) -> Ptr),
        entry!(0x00c3_a610, fn_00c3a610(Ptr) -> u32),
        entry!(0x00c3_a140, bs_multi_bound_room_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00c3_a5b0,
            bs_multi_bound_node_update_downward_pass(Ptr, u32, u32)
        ),
        entry!(0x00c3_a5e0, bs_multi_bound_node_update_upward_pass(Ptr)),
        entry!(0x00c3_a640, fn_00c3a640(Ptr, u32) -> u32),
        entry!(0x00c3_a670, fn_00c3a670(Ptr, u32, u32) -> u32),
    ]
}

/// `NiNode::UpdateDownwardPass` (`00a5dd70`, `RET 8`) and `NiNode::UpdateUpwardPass`
/// (`00a5db30`), `thiscall`.
const NI_NODE_UPDATE_DOWNWARD_PASS: u32 = 0x00a5_dd70;
const NI_NODE_UPDATE_UPWARD_PASS: u32 = 0x00a5_db30;
/// The memory manager object (`011f6238`) and its `thiscall` allocate (`00aa3e40`, one
/// word) and `00aa4150` (block and size; it answers a block, which `0042f5d0` and
/// `0084b630` pass on as the reallocated one).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_FREE_SIZED: u32 = 0x00aa_4150;

// Translated from 00c3a5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundNode::UpdateDownwardPass` (Xbox PDB name from the engine map; `thiscall`,
/// `RET 8`): `NiNode::UpdateDownwardPass` with the two words, then the object's own
/// virtual slot `0xbc`.
pub fn bs_multi_bound_node_update_downward_pass(e: &mut Engine, this: Ptr, a1: u32, a2: u32) {
    e.call(NI_NODE_UPDATE_DOWNWARD_PASS, &args![this, a1, a2]);
    e.vcall(this.addr(), 0xbc, &[]);
}

// Translated from 00c3a5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundNode::UpdateUpwardPass` (Xbox PDB name from the engine map; `thiscall`):
/// `NiNode::UpdateUpwardPass`, then the object's own virtual slot `0xbc`.
pub fn bs_multi_bound_node_update_upward_pass(e: &mut Engine, this: Ptr) {
    e.call(NI_NODE_UPDATE_UPWARD_PASS, &args![this]);
    e.vcall(this.addr(), 0xbc, &[]);
}

// Translated from 00c3a640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsmultiboundroom.obj`; `thiscall`, `RET 4`, `this` not
/// read): an allocator's allocate of `count` pointers, `count * 4` bytes from the memory
/// manager (`00aa3e40`).
pub fn fn_00c3a640(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, count << 2])
        .u32()
}

// Translated from 00c3a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsmultiboundroom.obj`; `thiscall`, `RET 8`, `this` not
/// read): `00aa4150` on the memory manager with `block` and `count * 4`, answering the block
/// it gives (the exe ends on that call and leaves its `EAX`). It sits in the vtable slot
/// `+0xC` of a `BSSimpleArray` (RTTI), `_Reallocate` in the Xbox PDB, which answers the new
/// buffer.
pub fn fn_00c3a670(e: &mut Engine, _this: Ptr, block: u32, count: u32) -> u32 {
    e.call(MEMORY_FREE_SIZED, &args![MEMORY_MANAGER, block, count << 2])
        .u32()
}

#[cfg(test)]
mod b012_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn the_update_passes_run_the_node_pass_then_slot_0xbc() {
        let mut e = Engine::new();
        const VTABLE: u32 = 0x0300_0000;
        const SLOT: u32 = 0x0300_1000;
        let mut slots = vec![0u32; 0x30];
        slots[0xbc / 4] = SLOT;
        e.put_vtable(VTABLE, &slots);
        for address in [
            NI_NODE_UPDATE_DOWNWARD_PASS,
            NI_NODE_UPDATE_UPWARD_PASS,
            SLOT,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xec);
        e.mem.set_u32(this, VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x00c3_a5b0, &args![this, 5u32, 6u32]);
        e.call(0x00c3_a5e0, &args![this]);
        assert_eq!(
            calls(&e, NI_NODE_UPDATE_DOWNWARD_PASS),
            vec![vec![this, 5, 6]]
        );
        assert_eq!(calls(&e, NI_NODE_UPDATE_UPWARD_PASS), vec![vec![this]]);
        assert_eq!(calls(&e, SLOT), vec![vec![this]; 2]);
    }

    #[test]
    fn the_allocator_scales_by_four() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |_, a| (a[1] + 0x1000).into_ret());
        e.register(MEMORY_FREE_SIZED, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_a640, &args![0u32, 3u32]).u32(), 0x100c);
        e.call(0x00c3_a670, &args![0u32, 0x5000u32, 3u32]);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 12]]);
        assert_eq!(
            calls(&e, MEMORY_FREE_SIZED),
            vec![vec![MEMORY_MANAGER, 0x5000, 12]]
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_multi_bound_room_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x00c3_a140, 0x00c3_9770, 0xec);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c3a610_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_a610, &args![0x1000u32]).u32(), 0x0120_2828);
        assert_eq!(e.call(0x00c3_a610, &args![0u32]).u32(), 0x0120_2828);
    }

    #[test]
    fn fn_00c3a670_answers_the_block_the_manager_gives() {
        let mut e = Engine::new();
        e.register(MEMORY_FREE_SIZED, |_, a| (a[1] + a[2]).into_ret());
        assert_eq!(
            e.call(0x00c3_a670, &args![0u32, 0x5000u32, 3u32]).u32(),
            0x500c
        );
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn the_room_constructor_clears_its_lists_and_sets_the_flag() {
        let mut e = Engine::new();
        e.register(MULTI_BOUND_NODE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x30, 0x0000_0001);
            Ret::default()
        });
        e.register(LIGHT_ARRAY_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0xec);
        for offset in (0xb4u32..0xe8).step_by(4) {
            e.mem.set_u32(this + offset, 0xdddd_dddd);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_9640, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), MULTI_BOUND_ROOM_VTABLE);
        for list in [PORTAL_LIST, OCCLUDER_LIST, JOINED_MULTI_BOUND_LIST] {
            for word in 0..3 {
                assert_eq!(e.mem.u32(this + list + 4 * word), 0);
            }
        }
        assert_eq!(e.mem.u32(this + COMPOUND_FRUSTUM), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0x801);
        let log = e.call_log.take().unwrap();
        let order: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (MULTI_BOUND_NODE_CONSTRUCT, vec![this]),
                (LIGHT_ARRAY_CONSTRUCT, vec![this + LIGHTS]),
            ]
        );
    }
}

#[cfg(test)]
mod n082_tests {
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

    /// An engine whose objects made by `object` share a vtable at 0x7000 with slot `i` at
    /// `0x8000 + 4 * i`, all doubled to answer 0.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x30).map(|i| 0x8000 + 4 * i).collect();
        e.put_vtable(0x7000, &slots);
        for i in 0..0x30u32 {
            e.register(0x8000 + 4 * i, |_, _| Ret::default());
        }
        e
    }

    fn object(e: &mut Engine, size: u32) -> u32 {
        let at = e.mem.alloc(size);
        e.mem.set_u32(at, 0x7000);
        at
    }

    /// A singly linked list of `items` (node: next, unused, item), answering its head.
    fn list_of(e: &mut Engine, items: &[u32]) -> u32 {
        let mut head = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(12);
            e.mem.set_u32(node, head);
            e.mem.set_u32(node + 8, *item);
            head = node;
        }
        head
    }

    #[test]
    fn destructor_body_empties_the_members_in_the_exe_order() {
        let mut e = engine();
        for callee in [
            PORTAL_LIST_REMOVE_ALL,
            OCCLUDER_LIST_REMOVE_ALL,
            JOINED_LIST_REMOVE_ALL,
            LIGHT_ARRAY_00B5AF80,
            ROOM_0045C520,
            MULTI_BOUND_NODE_DESTRUCTOR,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let room = e.mem.alloc(0xec);
        e.mem.set_u32(room + COMPOUND_FRUSTUM, 0x9000);
        e.call_log = Some(vec![]);
        e.call(0x00c3_9770, &args![room]);
        let order: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap()[1..].to_vec();
        assert_eq!(
            order,
            vec![
                (PORTAL_LIST_REMOVE_ALL, vec![room + 0xb4]),
                (OCCLUDER_LIST_REMOVE_ALL, vec![room + 0xc0]),
                (JOINED_LIST_REMOVE_ALL, vec![room + 0xd0]),
                (LIGHT_ARRAY_00B5AF80, vec![room + 0xdc, 1]),
                (ROOM_0045C520, vec![room]),
                (LIGHT_ARRAY_00B5AF80, vec![room + 0xdc, 1]),
                (JOINED_LIST_REMOVE_ALL, vec![room + 0xd0]),
                (OCCLUDER_LIST_REMOVE_ALL, vec![room + 0xc0]),
                (PORTAL_LIST_REMOVE_ALL, vec![room + 0xb4]),
                (MULTI_BOUND_NODE_DESTRUCTOR, vec![room]),
            ]
        );
        assert_eq!(e.mem.u32(room), MULTI_BOUND_ROOM_VTABLE);
        assert_eq!(e.mem.u32(room + 0xdc), LIGHT_ARRAY_VTABLE);
        // Without a compound frustum `0045c520` is not run.
        e.mem.set_u32(room + COMPOUND_FRUSTUM, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c3_9770, &args![room]);
        assert!(calls_to(&e, ROOM_0045C520).is_empty());
    }

    /// A room with a multi-bound whose shape is in `state`, `children` children (each a
    /// plain object) and the given culling mode and frustum.
    fn visible_room(e: &mut Engine, state: u32, mode: u32, frustum: u32) -> (u32, Vec<u32>) {
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape + 8, state);
        let multi_bound = e.mem.alloc(0x10);
        e.mem.set_u32(multi_bound + 0xc, shape);
        let room = e.mem.alloc(0xec);
        e.mem.set_u32(room + MULTI_BOUND, multi_bound);
        let children: Vec<u32> = (0..2).map(|_| e.mem.alloc(0x40)).collect();
        let array = e.mem.alloc(12);
        e.mem.set_u32(array, children[0]);
        e.mem.set_u32(array + 8, children[1]);
        e.mem.set_u32(room + CHILDREN, array);
        e.mem.set_u16(room + CHILD_COUNT, 3);
        e.mem.set_u16(room + CHILDREN_USED, 2);
        e.mem.set_u32(room + CULLING_MODE, mode);
        e.mem.set_u32(room + COMPOUND_FRUSTUM, frustum);
        (room, children)
    }

    #[test]
    fn on_visible_culls_the_children_with_the_room_frustum() {
        let mut e = engine();
        let frames = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let seen = frames.clone();
        let culler = e.mem.alloc(0x100);
        e.register_double(NI_AV_OBJECT_CULL, move |e, a| {
            seen.borrow_mut()
                .push((a[0], e.mem.u32(culler + 0x90), e.mem.u32(culler + 0xc0)));
            Ret::default()
        });
        e.register(COMPOUND_FRUSTUM_PRETHREAD, |_, _| Ret::default());
        let (room, children) = visible_room(&mut e, 1, 0, 0x9000);
        e.mem.set_u32(culler + 0x90, 0);
        e.mem.set_u32(culler + 0xc0, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x00c3_9890, &args![room, culler]);
        assert_eq!(
            *frames.borrow(),
            vec![(children[0], 0, 0x9000), (children[1], 0, 0x9000)]
        );
        assert_eq!(
            calls_to(&e, COMPOUND_FRUSTUM_PRETHREAD),
            vec![vec![0x9000]; 2]
        );
        assert_eq!(e.mem.u32(culler + 0x90), 0);
        assert_eq!(e.mem.u32(culler + 0xc0), 0x5555);
        // A pass-all room mode (1) keeps the culler's frustum.
        frames.borrow_mut().clear();
        e.mem.set_u32(room + CULLING_MODE, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c3_9890, &args![room, culler]);
        assert_eq!(
            *frames.borrow(),
            vec![(children[0], 1, 0x5555), (children[1], 1, 0x5555)]
        );
        assert!(calls_to(&e, COMPOUND_FRUSTUM_PRETHREAD).is_empty());
        assert_eq!(e.mem.u32(culler + 0x90), 0);
        // A culler in mode 3 keeps its mode.
        frames.borrow_mut().clear();
        e.mem.set_u32(culler + 0x90, 3);
        e.mem.set_u32(room + CULLING_MODE, 0);
        e.call(0x00c3_9890, &args![room, culler]);
        assert_eq!(frames.borrow()[0], (children[0], 3, 0x5555));
        assert_eq!(e.mem.u32(culler + 0x90), 3);
    }

    #[test]
    fn on_visible_skips_hidden_or_empty_rooms() {
        let mut e = engine();
        e.register(NI_AV_OBJECT_CULL, |_, _| Ret::default());
        let culler = e.mem.alloc(0x100);
        // Shape state 2 and a normal culler: nothing.
        let (room, _) = visible_room(&mut e, 2, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c3_9890, &args![room, culler]);
        assert!(calls_to(&e, NI_AV_OBJECT_CULL).is_empty());
        // The same room with a pass-all culler (mode 1) is culled.
        e.mem.set_u32(culler + 0x90, 1);
        e.call(0x00c3_9890, &args![room, culler]);
        assert_eq!(calls_to(&e, NI_AV_OBJECT_CULL).len(), 2);
        // No used children and no portals: nothing, not even the shape is read.
        let empty = e.mem.alloc(0xec);
        e.call_log = Some(vec![]);
        e.call(0x00c3_9890, &args![empty, culler]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn occluders_go_into_the_occluder_list() {
        let mut e = engine();
        let node = e.mem.alloc(12);
        e.register_double(OCCLUDER_NODE_ALLOCATE, move |_, _| node.into_ret());
        e.register(OCCLUDER_NODE_INSERT, |_, _| Ret::default());
        e.register(OCCLUDER_LIST_00C3A250, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x6000);
            Ret::default()
        });
        let room = e.mem.alloc(0xec);
        e.call_log = Some(vec![]);
        e.call(0x00c3_9a80, &args![room, 0x6000u32]);
        assert_eq!(
            calls_to(&e, OCCLUDER_NODE_ALLOCATE),
            vec![vec![room + 0xc8]]
        );
        assert_eq!(e.mem.u32(node + 8), 0x6000);
        assert_eq!(
            calls_to(&e, OCCLUDER_NODE_INSERT),
            vec![vec![room + 0xc0, node]]
        );
        e.call(0x00c3_9ad0, &args![room, 0x6000u32]);
        let calls = calls_to(&e, OCCLUDER_LIST_00C3A250);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], room + 0xc0);
        // Null occluders do nothing.
        e.call_log = Some(vec![]);
        e.call(0x00c3_9a80, &args![room, 0u32]);
        e.call(0x00c3_9ad0, &args![room, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn q_point_within_asks_the_joined_multi_bounds_until_one_holds_it() {
        let mut e = engine();
        let answers = std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));
        let table = answers.clone();
        e.register_double(0x808c, move |_, a| {
            (*table.borrow().get(&a[0]).unwrap_or(&0x100u32)).into_ret()
        });
        let own = object(&mut e, 0x10);
        let first = object(&mut e, 0x10);
        let second = object(&mut e, 0x10);
        let third = object(&mut e, 0x10);
        let room = e.mem.alloc(0xec);
        e.mem.set_u32(room + MULTI_BOUND, own);
        let head = list_of(&mut e, &[first, second, third]);
        e.mem.set_u32(room + JOINED_MULTI_BOUND_LIST, head);
        e.mem.set_u32(room + JOINED_COUNT, 3);
        answers.borrow_mut().insert(second, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c3_9b00, &args![room, 0x50u32]).bool());
        let asked: Vec<u32> = calls_to(&e, 0x808c).iter().map(|c| c[0]).collect();
        assert_eq!(asked, vec![own, first, second]);
        // The room's own multi-bound answering true: no joined one is asked.
        answers.borrow_mut().insert(own, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c3_9b00, &args![room, 0x50u32]).bool());
        assert_eq!(calls_to(&e, 0x808c).len(), 1);
        // Nobody holds it.
        answers.borrow_mut().clear();
        assert!(!e.call(0x00c3_9b00, &args![room, 0x50u32]).bool());
        // An empty joined list (count 0) is not walked.
        e.mem.set_u32(room + JOINED_COUNT, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c3_9b00, &args![room, 0x50u32]);
        assert_eq!(calls_to(&e, 0x808c).len(), 1);
    }

    #[test]
    fn check_bound_merges_the_joined_answers() {
        let mut e = engine();
        let answers = std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::new()));
        for slot in [0x8098u32, 0x8094] {
            let table = answers.clone();
            e.register_double(slot, move |_, a| {
                (*table.borrow().get(&a[0]).unwrap_or(&0u32)).into_ret()
            });
        }
        let shapes: Vec<u32> = (0..4).map(|_| object(&mut e, 0x10)).collect();
        let bounds: Vec<u32> = shapes
            .iter()
            .map(|shape| {
                let bound = e.mem.alloc(0x10);
                e.mem.set_u32(bound + 0xc, *shape);
                bound
            })
            .collect();
        let room = e.mem.alloc(0xec);
        e.mem.set_u32(room + MULTI_BOUND, bounds[0]);
        let head = list_of(&mut e, &bounds[1..]);
        e.mem.set_u32(room + JOINED_MULTI_BOUND_LIST, head);
        e.mem.set_u32(room + JOINED_COUNT, 3);
        answers.borrow_mut().insert(shapes[0], 2);
        answers.borrow_mut().insert(shapes[2], 1);
        answers.borrow_mut().insert(shapes[3], 5);
        e.call_log = Some(vec![]);
        // 2, then 0 (kept 2), then 1: stops.
        assert_eq!(e.call(0x00c3_9bb0, &args![room, 0x60u32]).u32(), 1);
        assert_eq!(calls_to(&e, 0x8098).len(), 3);
        assert_eq!(e.call(0x00c3_9c80, &args![room, 0x60u32]).u32(), 1);
        assert_eq!(calls_to(&e, 0x8094).len(), 3);
        // Without the 1 the last nonzero answer stays.
        answers.borrow_mut().remove(&shapes[2]);
        assert_eq!(e.call(0x00c3_9bb0, &args![room, 0x60u32]).u32(), 5);
        // The room's own 1 answers at once.
        answers.borrow_mut().insert(shapes[0], 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_9c80, &args![room, 0x60u32]).u32(), 1);
        assert_eq!(calls_to(&e, 0x8094).len(), 1);
    }

    #[test]
    fn update_world_bound_with_a_shape_merges_the_joined_bounds() {
        let mut e = engine();
        e.register(NODE_005467E0, |_, _| Ret::default());
        e.register(NI_BOUND_MERGE, |_, _| Ret::default());
        e.register(0x80a4, |e, a| {
            e.mem.set_u32(a[1], a[0]);
            Ret::default()
        });
        let shapes: Vec<u32> = (0..3).map(|_| object(&mut e, 0x10)).collect();
        let bounds: Vec<u32> = shapes
            .iter()
            .map(|shape| {
                let bound = e.mem.alloc(0x10);
                e.mem.set_u32(bound + 0xc, *shape);
                bound
            })
            .collect();
        let room = e.mem.alloc(0xec);
        let world = e.mem.alloc(0x10);
        e.mem.set_u32(room + WORLD_BOUND, world);
        e.mem.set_u32(room + MULTI_BOUND, bounds[0]);
        let head = list_of(&mut e, &bounds[1..]);
        e.mem.set_u32(room + JOINED_MULTI_BOUND_LIST, head);
        e.mem.set_u32(room + JOINED_COUNT, 2);
        let merged = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let seen = merged.clone();
        e.register_double(NI_BOUND_MERGE, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c3_9d50, &args![room]);
        assert_eq!(calls_to(&e, NODE_005467E0), vec![vec![room]]);
        let shaped: Vec<u32> = calls_to(&e, 0x80a4).iter().map(|c| c[0]).collect();
        assert_eq!(shaped, shapes);
        assert_eq!(e.mem.u32(world), shapes[0]);
        assert_eq!(
            *merged.borrow(),
            vec![(world, shapes[1]), (world, shapes[2])]
        );
    }

    #[test]
    fn update_world_bound_without_a_shape_takes_the_children() {
        let mut e = engine();
        e.register(NODE_005467E0, |_, _| Ret::default());
        e.register(NI_BOUND_MERGE, |_, _| Ret::default());
        e.map(RADIUS_ZERO & !0xfff, 0x1000);
        e.mem.set_u64(RADIUS_ZERO, 0f64.to_bits());
        let room = e.mem.alloc(0xec);
        let world = e.mem.alloc(0x10);
        e.mem.set_u32(world + 0xc, 9f32.to_bits());
        e.mem.set_u32(room + WORLD_BOUND, world);
        e.mem.set_u32(room + AV_OBJECT_FLAGS, 0x8_0000);
        let child = |e: &mut Engine, radius: f32| {
            let bound = e.mem.alloc(0x10);
            for i in 0..3 {
                e.mem.set_f32(bound + 4 * i, 1.0 + i as f32);
            }
            e.mem.set_f32(bound + 0xc, radius);
            let child = e.mem.alloc(0x40);
            e.mem.set_u32(child + WORLD_BOUND, bound);
            (child, bound)
        };
        let (flat, _) = child(&mut e, 0.0);
        let (first, _) = child(&mut e, 4.0);
        let (second, second_bound) = child(&mut e, 2.0);
        let array = e.mem.alloc(16);
        for (i, c) in [flat, 0, first, second].iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *c);
        }
        e.mem.set_u32(room + CHILDREN, array);
        e.mem.set_u16(room + CHILD_COUNT, 4);
        e.call_log = Some(vec![]);
        e.call(0x00c3_9d50, &args![room]);
        assert_eq!(e.mem.f32(world), 1.0);
        assert_eq!(e.mem.f32(world + 8), 3.0);
        assert_eq!(e.mem.f32(world + 0xc), 4.0);
        assert_eq!(calls_to(&e, NODE_005467E0), vec![vec![room]]);
        assert_eq!(
            calls_to(&e, NI_BOUND_MERGE),
            vec![vec![world, second_bound]]
        );
        // No qualifying child: only the radius is cleared.
        e.mem.set_u16(room + CHILD_COUNT, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c3_9d50, &args![room]);
        assert_eq!(e.mem.f32(world + 0xc), 0.0);
        assert!(calls_to(&e, NODE_005467E0).is_empty());
    }

    #[test]
    fn small_vtable_members() {
        let mut e = engine();
        e.register(NI_OBJECT_IS_EQUAL, |_, _| 0x1234_5601u32.into_ret());
        assert_eq!(e.call(0x00c3_a080, &args![0u32, 5u32]).u32(), 0);
        assert_eq!(e.call(0x00c3_a090, &args![1u32, 2u32]).u32(), 0x1234_5601);
        assert!(e.call(0x00c3_a130, &args![0u32, 5u32]).bool());
        e.call_log = Some(vec![]);
        for address in [0x00c3_a100u32, 0x00c3_a110, 0x00c3_a120] {
            e.call(address, &args![0u32, 5u32]);
        }
        assert_eq!(e.call_log.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name() {
        let mut e = engine();
        e.register(MULTI_BOUND_NODE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.map(BS_MULTI_BOUND_ROOM_RTTI & !0xfff, 0x1000);
        e.mem.set_u32(BS_MULTI_BOUND_ROOM_RTTI, 0x0102_0300);
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(VIEWER_STRINGS_ADD, |e, a| e.mem.u32(a[1]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00c3_a0c0, &args![0x4000u32, 0x5000u32]);
        assert_eq!(
            calls_to(&e, MULTI_BOUND_NODE_GET_VIEWER_STRINGS),
            vec![vec![0x4000, 0x5000]]
        );
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x0102_0300]]);
        assert_eq!(calls_to(&e, VIEWER_STRINGS_ADD)[0][0], 0x5000);
    }

    #[test]
    fn light_array_destructors() {
        let mut e = engine();
        e.register(LIGHT_ARRAY_00B5AF80, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00c3_a620, &args![array]);
        assert_eq!(e.mem.u32(array), LIGHT_ARRAY_VTABLE);
        assert_eq!(calls_to(&e, LIGHT_ARRAY_00B5AF80), vec![vec![array, 1]]);
        assert_eq!(e.call(0x00c3_a6a0, &args![array, 0u32]).u32(), array);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00c3_a6a0, &args![array, 1u32]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, array]]
        );
    }
}
