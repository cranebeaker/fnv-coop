//! `BSShader/bsfadenode.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSFadeNode` is an `NiNode` (0xAC bytes on PC) that fades its subtree in and out with the
//! camera distance. The globals below are `BSFadeNode`'s statics; their names follow the
//! order of the Xbox PDB's static members next to the addresses the code reads (not confirmed
//! one by one).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BSFadeNode` (Xbox PDB, 0x110 bytes there), 0xE4 bytes on PC (the size `CreateClone`
    /// allocates). The `NiAVObject` and `NiNode` fields used here sit at their PC offsets
    /// (the PC build does not align the transforms to 16 bytes).
    pub struct BSFadeNode: 0xE4 {
        /// `NiAVObject::m_kWorldBound` (Xbox PDB): the world bound, radius at +0xC.
        0x20 m_kWorldBound: u32,
        /// `NiAVObject::m_uFlags` (Xbox PDB).
        0x30 m_uFlags: u32,
        /// `NiNode::m_kChildren` (Xbox PDB) element pointer and `m_usSize`.
        0xA0 m_kChildren_pBase: u32,
        0xA6 m_kChildren_usSize: u16,
        /// `fNearDistSqr` (Xbox PDB; the code uses it as the near fade distance).
        0xAC fNearDistSqr: u32,
        /// `fFarDistSqr` (Xbox PDB; the far fade distance).
        0xB0 fFarDistSqr: u32,
        /// `fLastFade` (Xbox PDB).
        0xB4 fLastFade: u32,
        /// `fCurrentFade` (Xbox PDB).
        0xB8 fCurrentFade: u32,
        /// `fBoundRadius` (Xbox PDB).
        0xBC fBoundRadius: u32,
        /// `fTimeSinceUpdate` (Xbox PDB).
        0xC0 fTimeSinceUpdate: u32,
        /// `eMultType` (Xbox PDB): `ENUM_LOD_MULT`, an index into the multiplier table.
        0xC4 eMultType: u32,
        /// `iFrameCounter` (Xbox PDB).
        0xC8 iFrameCounter: u32,
        /// `pUserData` (Xbox PDB).
        0xCC pUserData: u32,
        /// `pBoundArray` (Xbox PDB).
        0xD0 pBoundArray: u32,
    }
}

/// `ParentRoomList` and `ParentPortalList` (Xbox PDB): `BSSimpleList`s (item, next).
const PARENT_ROOM_LIST: u32 = 0xd4;
const PARENT_PORTAL_LIST: u32 = 0xdc;
/// `NiAVObject::m_kWorld` (Xbox PDB) translation: x at +0x8C, y, z (the same offset in
/// the camera, also an `NiAVObject`).
const WORLD_TRANSLATE: u32 = 0x8c;
/// `NiCamera::m_fLODAdjust` (Xbox PDB name; +0x110 on PC).
const CAMERA_LOD_ADJUST: u32 = 0x110;

/// `BSFadeNode::bFadeEnabled` (`bool`).
const FADE_ENABLED: u32 = 0x011a_d7b4;
/// The multiplier per `eMultType` (eleven floats; `pfFadeOutMultiplierA` in the PDB's
/// statics).
const MULTIPLIERS: u32 = 0x011a_d7b8;
/// `fFadeInTime`, `fFadeOutTime`, `fFadeInAlphaThreshold`, `fFadeOutAlphaThreshold`,
/// `fDistanceMultiplier`, `maxFadeIncrement` (floats, in this order).
const FADE_IN_TIME: u32 = 0x011a_d7e4;
const FADE_OUT_TIME: u32 = 0x011a_d7e8;
const FADE_IN_THRESHOLD: u32 = 0x011a_d7ec;
const FADE_OUT_THRESHOLD: u32 = 0x011a_d7f0;
const DISTANCE_MULTIPLIER: u32 = 0x011a_d7f4;
const MAX_FADE_INCREMENT: u32 = 0x011a_d7f8;
/// The frame's elapsed time (`float` at `011f63a0`).
const FRAME_TIME: u32 = 0x011f_63a0;
/// The fade frame counter (`int`; `iFadeCounter` in the PDB's statics).
const FADE_COUNTER: u32 = 0x011f_9138;
/// `pMaxAlphaAccessFunc`: a `cdecl` function of the node answering a float, or 0.
const MAX_ALPHA_ACCESS: u32 = 0x011f_9134;
/// `NiRTTI BSFadeNode::ms_RTTI`.
const FADE_NODE_RTTI: u32 = 0x011f_9140;
/// The integer `SetRange` takes as its least near distance (`iMinNearDistance`'s value).
const MIN_NEAR_DISTANCE: u32 = 0x011f_914c;
/// `pFadeNodeAnimName` (`NiFixedString*`) and `spFadeAlpha` (`NiPointer<NiAlphaProperty>`).
const FADE_NODE_ANIM_NAME: u32 = 0x011f_913c;
const FADE_ALPHA: u32 = 0x011f_9154;
/// `"FadeNode Anim"`.
const FADE_NODE_ANIM_TEXT: u32 = 0x010a_9090;
/// The float both distances start at (`01016970`).
const DEFAULT_DISTANCE: u32 = 0x0101_6970;
/// `2600.0` and `1300.0`: the far and near distances `CheckFadeRadius` caps to.
const CAPPED_FAR: u32 = 0x010a_90a4;
const CAPPED_NEAR: u32 = 0x010a_90a0;
/// `0.2`: the distance scale `OnVisible` uses when the multiplier is not positive.
const UNSCALED_DISTANCE: u32 = 0x0101_df44;
/// `600.0`: the bound radius below which `OnVisible` snaps the fade.
const SNAP_RADIUS: u32 = 0x0101_786c;
/// `float` 1.0 bits, a constant the code loads with `FLD1`.
const ONE: u32 = 0x3f80_0000;
/// The `NiFixedString` handle value `pFadeNodeAnimName`'s destructor leaves alone (`0109b220`).
const EMPTY_STRING_HANDLE: u32 = 0x0109_b220;
/// The global whose object at +8 is the shader accumulator (`011f4748`).
const SCENE_GLOBAL: u32 = 0x011f_4748;
/// `0043b300` (`cdecl`, two words: `011fa000` and the accumulator).
const ACCUMULATOR_CHECK_0043B300: u32 = 0x0043_b300;
const ACCUMULATOR_CHECK_ARGUMENT: u32 = 0x011f_a000;
/// The bound `OnVisible` passes when the node has none (`011f4288`).
const DEFAULT_BOUND: u32 = 0x011f_4288;

/// `NiNode::UpdateDownwardPass` (`00a5dd70`, `thiscall`, two words).
const NODE_UPDATE_DOWNWARD_PASS: u32 = 0x00a5_dd70;
/// `NiNode::LoadBinary` (`00a5dd40`, `thiscall`, one word).
const NODE_LOAD_BINARY: u32 = 0x00a5_dd40;
/// `NiNode::UpdateSelectedDownwardPass` (`00a5ded0`, `thiscall`, two words).
const NODE_UPDATE_SELECTED_DOWNWARD_PASS: u32 = 0x00a5_ded0;
/// `NiNode::OnVisible` (`00a5dbe0`, `thiscall`, one word).
const NODE_ON_VISIBLE: u32 = 0x00a5_dbe0;
/// `NiNode` constructor (`00a5ecb0`, `thiscall`, one word) and destructor (`00a5e930`).
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const NODE_DESTRUCT: u32 = 0x00a5_e930;
/// `00a5dc80` (`thiscall`, two words: the clone, the cloning process): the node's member
/// copy for `CreateClone`.
const NODE_COPY_MEMBERS: u32 = 0x00a5_dc80;
/// `BSFadeNode`'s vtable.
const FADE_NODE_VTABLE: u32 = 0x010a_8f90;
/// `NiAlphaProperty`'s vtable and the property constructor `InitSDM` runs (`00a5c200`).
const ALPHA_PROPERTY_VTABLE: u32 = 0x0101_62dc;
const PROPERTY_CONSTRUCT: u32 = 0x00a5_c200;
/// `00a5b690` (`cdecl`, one word: a C string): an `NiFixedString` handle for the string.
const FIXED_STRING_FROM_TEXT: u32 = 0x00a5_b690;
/// `00c80090` (`cdecl`, one word: the node).
const NODE_CHECK_00C80090: u32 = 0x00c8_0090;
/// `00653290` (`thiscall` on a controller, one word) and its argument (`011f36ac`).
const CONTROLLER_CALL_00653290: u32 = 0x0065_3290;
const CONTROLLER_ARGUMENT: u32 = 0x011f_36ac;
/// `NiPoint3::Length` (`00457990`, `thiscall`, result in ST0).
const POINT_LENGTH: u32 = 0x0045_7990;
/// `_CIsqrt` (`00ec6040`): the square root of ST0 (a leading `f64` argument).
const SQRT: u32 = 0x00ec_6040;
/// `0040ebd0` and `00404010` (`cdecl`, two floats, result in ST0): the smaller
/// (`b` when `b <= a`) and the larger (`a` when `b < a`) of two floats.
const FLOAT_MIN: u32 = 0x0040_ebd0;
const FLOAT_MAX: u32 = 0x0040_4010;
/// `BSShaderUtil::RecursiveSetPropertyFadeAlpha` (`00b6bb30`, `cdecl`: node, float).
const SET_FADE_ALPHA: u32 = 0x00b6_bb30;
/// `BSShaderAccumulator::StartGroupingAlphas` (`00b63500`, `thiscall`, two words) and
/// `StopGroupingAlphas` (`00b63090`, `thiscall`).
const START_GROUPING_ALPHAS: u32 = 0x00b6_3500;
const STOP_GROUPING_ALPHAS: u32 = 0x00b6_3090;
/// The allocator: `00aa13e0` (`cdecl`, size), `00aa1460` (`cdecl`, block and size),
/// `MemoryManager::Allocate` / `Deallocate` (`thiscall` on `011f6238`).
const NEW_OBJECT: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `InterlockedIncrement` / `InterlockedDecrement` (KERNEL32 import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00b4dec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode::SetLODMultType` (Xbox PDB): `thiscall`, one word, stored in `eMultType`.
pub fn bs_fade_node_set_lod_mult_type(e: &mut Engine, this: Ptr<BSFadeNode>, mult_type: u32) {
    e.set(this, BSFadeNode::eMultType, mult_type);
}

// Translated from 00b4ded0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode`'s vtable slot +0xA4 (the Xbox PDB's `UpdateDownwardPass` slot; Ghidra made no
/// function of it): a jump to `NiNode::UpdateDownwardPass` (`00a5dd70`) with the same two
/// stack words.
pub fn fn_00b4ded0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(NODE_UPDATE_DOWNWARD_PASS, &args![this, first, second]);
}

// Translated from 00b4dee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode::LoadBinary` (Xbox PDB): a jump to `NiNode::LoadBinary` (`00a5dd40`) with the
/// stream.
pub fn bs_fade_node_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(NODE_LOAD_BINARY, &args![this, stream]);
}

// Translated from 00b4df30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `BSSimpleList<T>`'s clear (`thiscall`, no stack words). While
/// the list has a next node: takes it out (keeping its next), clears that node's own list
/// recursively, frees it (`MemoryManager::Deallocate`) and links its next in; then clears the
/// item word.
pub fn fn_00b4df30(e: &mut Engine, this: Ptr) {
    let list = this.addr();
    if e.mem.u32(list.wrapping_add(4)) != 0 {
        loop {
            let node = e.mem.u32(list.wrapping_add(4));
            let next = e.mem.u32(node.wrapping_add(4));
            e.mem.set_u32(node.wrapping_add(4), 0);
            let node = e.mem.u32(list.wrapping_add(4));
            if node != 0 {
                fn_00b4df30(e, Ptr::new(node));
                e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, node]);
            }
            e.mem.set_u32(list.wrapping_add(4), next);
            if next == 0 {
                break;
            }
        }
    }
    e.mem.set_u32(list, 0);
}

// Translated from 00b4df80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the node's most alpha (`thiscall`, no stack words). Calls
/// `pMaxAlphaAccessFunc` (`011f9134`, `cdecl`, the node) when set and answers its result
/// rounded to a float; else 1.0.
pub fn fn_00b4df80(e: &mut Engine, this: Ptr) -> f32 {
    let access = e.global::<u32>(MAX_ALPHA_ACCESS);
    if access != 0 {
        let value = e.call(access, &args![this]).st0;
        f32::from_bits(x87_quiet_bits(x87_store32(value)))
    } else {
        1.0
    }
}

// Translated from 00b4dfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra made no function of it): `cdecl`, one word (an object).
/// When the object is set and its virtual +0x10 (`IsFadeNode` in the Xbox PDB's slots)
/// answers a node, sets flag `0x20000` in that node's `m_uFlags`.
pub fn fn_00b4dfb0(e: &mut Engine, object: u32) {
    if object == 0 {
        return;
    }
    let node = e.vcall(object, 0x10, &args![]).u32();
    if node != 0 {
        let flags = e.mem.u32(node.wrapping_add(0x30));
        e.mem.set_u32(node.wrapping_add(0x30), flags | 0x2_0000);
    }
}

// Translated from 00b4dfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode::SetRange` (Xbox PDB): `thiscall`, two floats (near, far). The least near
/// distance (the integer at `011f914c`, as a float) replaces a smaller near distance, and
/// then twice it replaces a smaller far distance; the two land in `fNearDistSqr` and
/// `fFarDistSqr`.
pub fn bs_fade_node_set_range(e: &mut Engine, this: Ptr<BSFadeNode>, near: f32, far: f32) {
    let least = x87_store32(f64::from(e.global::<u32>(MIN_NEAR_DISTANCE) as i32));
    let least = x87_load32(least);
    let mut near_bits = near.to_bits();
    let mut far_bits = far.to_bits();
    // FLD near; FLD least; FCOM; TEST AH,0x41; JNZ: keep both unless least > near.
    if least > x87_load32(near_bits) {
        near_bits = x87_store32(least);
        let twice = x87_add(least, least);
        // FLD far; FCOMP (far vs twice); TEST AH,0x5; JP: only when far < twice.
        if x87_load32(far_bits) < twice {
            far_bits = x87_store32(twice);
        }
    }
    e.set(this, BSFadeNode::fNearDistSqr, x87_quiet_bits(near_bits));
    e.set(this, BSFadeNode::fFarDistSqr, x87_quiet_bits(far_bits));
}

/// The sum of squares the fade code computes inline for a distance vector loaded in the
/// order `first`, `second`, `third`: `(first² + second²) + third²` with the additions in
/// the exe's operand order (`FADDP ST2,ST0` adds the first square to the second).
fn sum_of_squares(first: f64, second: f64, third: f64) -> f64 {
    let first_square = x87_mul(first, first);
    let second_square = x87_mul(second, second);
    let sum = x87_add(second_square, first_square);
    let third_square = x87_mul(third, third);
    x87_add(sum, third_square)
}

/// The node's world position minus the camera's, each component rounded to a float (bits).
fn distance_vector(e: &Engine, this: u32, camera: u32) -> [u32; 3] {
    let mut out = [0u32; 3];
    for (i, slot) in out.iter_mut().enumerate() {
        let offset = WORLD_TRANSLATE + 4 * i as u32;
        let own = x87_load32(e.mem.u32(this.wrapping_add(offset)));
        let other = x87_m32(e.mem.u32(camera.wrapping_add(offset)));
        *slot = x87_store32(x87_sub(own, other));
    }
    out
}

// Translated from 00b4e030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the node is near enough to be drawn (`thiscall`, two
/// stack words: the camera and a float scale). `eMultType` 9 answers false. For a type in
/// 0..=10 whose multiplier is not 0, the camera's `m_fLODAdjust` (+0x110) is divided by it.
/// The distance to the camera (`_CIsqrt` of the squared length, rounded to a float) times the
/// scale, at least 0, times that adjust, must not exceed the far distance times
/// `fDistanceMultiplier`.
pub fn fn_00b4e030(e: &mut Engine, this: Ptr<BSFadeNode>, camera: Ptr, scale: f32) -> bool {
    let t = this.addr();
    let cam = camera.addr();
    let mult_type = e.get(this, BSFadeNode::eMultType) as i32;
    let mut adjust = x87_quiet_bits(e.mem.u32(cam.wrapping_add(CAMERA_LOD_ADJUST)));
    if mult_type == 9 {
        return false;
    }
    if (0..0xb).contains(&mult_type) {
        let multiplier = x87_load32(e.mem.u32(MULTIPLIERS + 4 * mult_type as u32));
        // FUCOM (multiplier vs 0); TEST AH,0x44; JNP: no division when it equals 0.
        if multiplier != 0.0 {
            adjust = x87_store32(x87_div(x87_m32(adjust), multiplier));
        }
    }
    let [dx, dy, dz] = distance_vector(e, t, cam);
    let squared = x87_store32(sum_of_squares(
        x87_load32(dy),
        x87_load32(dx),
        x87_load32(dz),
    ));
    let root = e.call(SQRT, &args![x87_load32(squared)]).st0;
    let root = x87_store32(root);
    let mut distance = x87_store32(x87_mul(x87_load32(root), x87_m32(scale.to_bits())));
    // FLDZ; FCOM [distance]; TEST AH,0x41; JNZ: 0 replaces a negative distance.
    if 0.0 > x87_m32(distance) {
        distance = x87_store32(0.0);
    }
    let distance = x87_store32(x87_mul(x87_load32(distance), x87_m32(adjust)));
    let far = x87_mul(
        x87_load32(e.get(this, BSFadeNode::fFarDistSqr)),
        x87_m32(e.global::<u32>(DISTANCE_MULTIPLIER)),
    );
    // FCOMPP (far vs distance); TEST AH,0x5; JNP: false when far < distance.
    !(far < x87_load32(distance))
}

// Translated from 00b4e150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode::BSFadeNode` (Xbox PDB): `thiscall`, no stack words. Constructs the `NiNode`
/// (`00a5ecb0` with 0), sets the vtable, empties both lists, sets both distances to the float
/// at `01016970`, clears flags `0x2C000`, clears `pBoundArray`, `eMultType` and `pUserData`
/// and zeroes the four fade floats; answers the node.
pub fn bs_fade_node_construct(e: &mut Engine, this: Ptr<BSFadeNode>) -> Ptr<BSFadeNode> {
    e.call(NODE_CONSTRUCT, &args![this, 0u32]);
    let distance = x87_quiet_bits(e.global::<u32>(DEFAULT_DISTANCE));
    let t = this.addr();
    e.mem.set_u32(t, FADE_NODE_VTABLE);
    for offset in [0xd4, 0xd8, 0xdc, 0xe0] {
        e.mem.set_u32(t.wrapping_add(offset), 0);
    }
    e.set(this, BSFadeNode::fNearDistSqr, distance);
    let flags = e.get(this, BSFadeNode::m_uFlags);
    e.set(this, BSFadeNode::m_uFlags, flags & 0xfffd_3fff);
    e.set(this, BSFadeNode::fFarDistSqr, distance);
    e.set(this, BSFadeNode::pBoundArray, 0);
    e.set(this, BSFadeNode::fLastFade, 0);
    e.set(this, BSFadeNode::eMultType, 0);
    e.set(this, BSFadeNode::fCurrentFade, 0);
    e.set(this, BSFadeNode::pUserData, 0);
    e.set(this, BSFadeNode::fBoundRadius, 0);
    e.set(this, BSFadeNode::fTimeSinceUpdate, 0);
    this
}

// Translated from 00b4e1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode::GetRTTI` (vtable slot +8; Ghidra made no function of it): `&ms_RTTI`.
pub fn bs_fade_node_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    FADE_NODE_RTTI
}

// Translated from 00b4e1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode::~BSFadeNode` (by its body): `thiscall`, no stack words. Sets the vtable,
/// clears `ParentRoomList` and `ParentPortalList`, then again (the lists' own destructors:
/// the portal list first), and runs the `NiNode` destructor (`00a5e930`).
// Translation incomplete: 00b4e1e0 - the C++ exception frame (FS:[0] chain, unwind states 2, 1, 0 between the list destructors) is not translated.
pub fn fn_00b4e1e0(e: &mut Engine, this: Ptr<BSFadeNode>) {
    let t = this.addr();
    e.mem.set_u32(t, FADE_NODE_VTABLE);
    let rooms = Ptr::new(t.wrapping_add(PARENT_ROOM_LIST));
    let portals = Ptr::new(t.wrapping_add(PARENT_PORTAL_LIST));
    fn_00b4df30(e, rooms);
    fn_00b4df30(e, portals);
    fn_00b4df30(e, portals);
    fn_00b4df30(e, rooms);
    e.call(NODE_DESTRUCT, &args![this]);
}

/// `NiPointer` assignment as the code inlines it: releases the old object
/// (`InterlockedDecrement`, `DeleteThis` at vtable +4 when it reaches 0) unless it is the new
/// one, stores the new one and takes a reference on it.
fn assign_fade_alpha(e: &mut Engine, object: u32) {
    let old = e.global::<u32>(FADE_ALPHA);
    if old == object {
        return;
    }
    if old != 0
        && e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(old, 4, &args![]);
    }
    e.set_global(FADE_ALPHA, object);
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

// Translated from 00b4e270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode::InitSDM` (Xbox PDB): no arguments. Makes the shared alpha property (0x1C
/// bytes: the property constructor `00a5c200`, `NiAlphaProperty`'s vtable, flags `0xEC`,
/// test reference 0) into `spFadeAlpha`, sets bit 0 of the property's flags (+0x18), and
/// allocates `pFadeNodeAnimName` holding `"FadeNode Anim"` (0 when the allocation failed).
// Translation incomplete: 00b4e270 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the two allocations) is not translated.
pub fn bs_fade_node_init_sdm(e: &mut Engine) {
    let property = e.call(NEW_OBJECT, &args![0x1cu32]).u32();
    if property != 0 {
        e.call(PROPERTY_CONSTRUCT, &args![property]);
        e.mem.set_u32(property, ALPHA_PROPERTY_VTABLE);
        e.mem.set_u16(property.wrapping_add(0x18), 0xec);
        e.mem.set_u8(property.wrapping_add(0x1a), 0);
    }
    assign_fade_alpha(e, property);
    let alpha = e.global::<u32>(FADE_ALPHA);
    let flags = e.mem.u16(alpha.wrapping_add(0x18));
    e.mem.set_u16(alpha.wrapping_add(0x18), flags | 1);
    let name = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    if name != 0 {
        let handle = e
            .call(FIXED_STRING_FROM_TEXT, &args![FADE_NODE_ANIM_TEXT])
            .u32();
        e.mem.set_u32(name, handle);
        e.set_global(FADE_NODE_ANIM_NAME, name);
    } else {
        e.set_global(FADE_NODE_ANIM_NAME, 0u32);
    }
}

// Translated from 00b4e380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `InitSDM`'s counterpart (no arguments). Releases `spFadeAlpha`
/// (`InterlockedDecrement`, `DeleteThis` at 0) and clears it when it was set; destroys
/// `pFadeNodeAnimName` (its handle's count at handle - 8 decremented unless it is the handle
/// at `0109b220`) and frees it; clears the pointer.
pub fn fn_00b4e380(e: &mut Engine) {
    let alpha = e.global::<u32>(FADE_ALPHA);
    if alpha != 0 {
        if e.call(INTERLOCKED_DECREMENT, &args![alpha.wrapping_add(4)])
            .u32()
            == 0
        {
            e.vcall(alpha, 4, &args![]);
        }
        e.set_global(FADE_ALPHA, 0u32);
    }
    let name = e.global::<u32>(FADE_NODE_ANIM_NAME);
    if name != 0 {
        let handle = e.mem.u32(name);
        if handle != e.global::<u32>(EMPTY_STRING_HANDLE) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, name]);
    }
    e.set_global(FADE_NODE_ANIM_NAME, 0u32);
}

// Translated from 00b4e3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode::CreateClone` (Xbox PDB): `thiscall`, one word (the cloning process). A new
/// node (0xE4 bytes, the constructor; 0 when the allocation failed), the `NiNode` member copy
/// (`00a5dc80`), flags `0x2C000` cleared, and the near and far distances and `eMultType`
/// copied.
// Translation incomplete: 00b4e3f0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the constructor) is not translated.
pub fn bs_fade_node_create_clone(e: &mut Engine, this: Ptr<BSFadeNode>, cloning: u32) -> Ptr {
    let block = e.call(NEW_OBJECT, &args![0xe4u32]).u32();
    let clone = if block != 0 {
        bs_fade_node_construct(e, Ptr::new(block))
    } else {
        Ptr::new(0)
    };
    e.call(NODE_COPY_MEMBERS, &args![this, clone, cloning]);
    let flags = e.get(clone, BSFadeNode::m_uFlags);
    e.set(clone, BSFadeNode::m_uFlags, flags & 0xfffd_3fff);
    let near = x87_quiet_bits(e.get(this, BSFadeNode::fNearDistSqr));
    e.set(clone, BSFadeNode::fNearDistSqr, near);
    let far = x87_quiet_bits(e.get(this, BSFadeNode::fFarDistSqr));
    e.set(clone, BSFadeNode::fFarDistSqr, far);
    let mult_type = e.get(this, BSFadeNode::eMultType);
    e.set(clone, BSFadeNode::eMultType, mult_type);
    clone.cast()
}

/// `RecursiveSetPropertyFadeAlpha` on the node with the float `bits`.
fn set_fade_alpha(e: &mut Engine, this: u32, bits: u32) {
    e.call(SET_FADE_ALPHA, &args![this, bits]);
}

/// `fCurrentFade` and `fLastFade` both `bits` (as `FST`; `FSTP` stores them).
fn set_fades(e: &mut Engine, this: Ptr<BSFadeNode>, bits: u32) {
    e.set(this, BSFadeNode::fCurrentFade, bits);
    e.set(this, BSFadeNode::fLastFade, bits);
}

/// The fade-in step of `OnVisible`: `fCurrentFade` grows by the frame time over
/// `fFadeInTime` (at most `maxFadeIncrement`), up to 1; the alpha goes down the subtree,
/// `fLastFade` follows, and flag `0x4000` is set once the fade reaches 1.
fn fade_in_step(e: &mut Engine, this: Ptr<BSFadeNode>) {
    let t = this.addr();
    let step = x87_store32(x87_div(
        x87_load32(e.global::<u32>(FRAME_TIME)),
        x87_m32(e.global::<u32>(FADE_IN_TIME)),
    ));
    let increment = x87_quiet_bits(e.global::<u32>(MAX_FADE_INCREMENT));
    let first = e.call(FLOAT_MIN, &args![step, increment]).st0;
    let sum = x87_store32(x87_add(
        first,
        x87_m32(e.get(this, BSFadeNode::fCurrentFade)),
    ));
    let value = e.call(FLOAT_MIN, &args![ONE, sum]).st0;
    let value = x87_quiet_bits(x87_store32(value));
    e.set(this, BSFadeNode::fCurrentFade, value);
    set_fade_alpha(e, t, value);
    let current = x87_quiet_bits(e.get(this, BSFadeNode::fCurrentFade));
    e.set(this, BSFadeNode::fLastFade, current);
    // FLD1; FCOMP [fCurrentFade]; TEST AH,0x41; JP: the flag only once 1 <= the fade.
    if 1.0 <= x87_m32(e.get(this, BSFadeNode::fCurrentFade)) {
        let flags = e.get(this, BSFadeNode::m_uFlags);
        e.set(this, BSFadeNode::m_uFlags, flags | 0x4000);
    }
}

/// The fade-out step of `OnVisible`: `fCurrentFade` shrinks by the frame time over
/// `fFadeOutTime` (at most `maxFadeIncrement`), down to 0; the alpha goes down the subtree,
/// `fLastFade` follows, and flag `0x4000` is cleared once the fade is 0.
fn fade_out_step(e: &mut Engine, this: Ptr<BSFadeNode>) {
    let t = this.addr();
    // FLD float; FSTP double: the current fade, exactly.
    let current = x87_load32(e.get(this, BSFadeNode::fCurrentFade));
    let step = x87_store32(x87_div(
        x87_load32(e.global::<u32>(FRAME_TIME)),
        x87_m32(e.global::<u32>(FADE_OUT_TIME)),
    ));
    let increment = x87_quiet_bits(e.global::<u32>(MAX_FADE_INCREMENT));
    let decrement = e.call(FLOAT_MIN, &args![step, increment]).st0;
    let lowered = x87_store32(x87_sub(current, decrement));
    let value = e.call(FLOAT_MAX, &args![0u32, lowered]).st0;
    let value = x87_quiet_bits(x87_store32(value));
    e.set(this, BSFadeNode::fCurrentFade, value);
    set_fade_alpha(e, t, value);
    let current = x87_quiet_bits(e.get(this, BSFadeNode::fCurrentFade));
    e.set(this, BSFadeNode::fLastFade, current);
    // FLDZ; FCOMP [fCurrentFade]; TEST AH,0x1; JNZ: the flag goes only once 0 >= the fade.
    if x87_m32(e.get(this, BSFadeNode::fCurrentFade)) <= 0.0 {
        let flags = e.get(this, BSFadeNode::m_uFlags);
        e.set(this, BSFadeNode::m_uFlags, flags & 0xffff_bfff);
    }
}

// Translated from 00b4e490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode::OnVisible` (vtable slot +0xD4, which `NiNode::OnVisible` `00a5dbe0` fills
/// for `NiNode`; Ghidra made no function of it): `thiscall`, one word (the culling process).
///
/// With fading off (`bFadeEnabled` clear) or flag `0x8000` set the node is simply visible.
/// `eMultType` 6 only fades in (unless flag `0x4000` says it is in). Otherwise the fade
/// target comes from the distance to the process's camera (+0xC) scaled by its
/// `m_fLODAdjust` over the type's multiplier (a multiplier that is not positive marks the
/// node faded in and scales by 0.2): 1 up to the near distance times `fDistanceMultiplier`,
/// then falling linearly to the far distance times it; type 10 takes 1 or 0 from flag
/// `0x10000000`. A node of radius under 600 not seen for 20 fade frames (and not type 8)
/// snaps in or out; then the frame counter is stored, and the most alpha
/// (`pMaxAlphaAccessFunc`) either groups the node's alphas in the accumulator (strictly
/// between 0 and 1, while faded in) or the fade steps in or out against the thresholds.
/// The node is visible (`NiNode::OnVisible`) while its fade and most alpha are not 0; a
/// grouping started here is stopped at the end.
pub fn bs_fade_node_on_visible(e: &mut Engine, this: Ptr<BSFadeNode>, process: Ptr) {
    let t = this.addr();
    let flags = e.get(this, BSFadeNode::m_uFlags);
    if e.global::<u8>(FADE_ENABLED) == 0 || (flags >> 15) & 1 != 0 {
        e.call(NODE_ON_VISIBLE, &args![this, process]);
        return;
    }
    let mult_type = e.get(this, BSFadeNode::eMultType);
    if mult_type == 6 {
        if (flags >> 14) & 1 == 0 {
            fade_in_step(e, this);
        }
        e.call(NODE_ON_VISIBLE, &args![this, process]);
        return;
    }
    // The type's multiplier, read without a range check.
    let multiplier = e
        .mem
        .u32(MULTIPLIERS.wrapping_add(mult_type.wrapping_mul(4)));
    let multiplier = x87_quiet_bits(multiplier);
    let camera = e.mem.u32(process.addr().wrapping_add(0xc));
    let scale;
    // FLD multiplier; FCOM (vs 0); TEST AH,0x41; JNZ: only a positive multiplier divides.
    if x87_load32(multiplier) > 0.0 {
        let adjust = x87_m32(e.mem.u32(camera.wrapping_add(CAMERA_LOD_ADJUST)));
        scale = x87_store32(x87_div(adjust, x87_load32(multiplier)));
    } else {
        scale = x87_quiet_bits(e.global::<u32>(UNSCALED_DISTANCE));
        e.set(this, BSFadeNode::m_uFlags, flags | 0x4000);
        set_fades(e, this, ONE);
        let current = x87_quiet_bits(e.get(this, BSFadeNode::fCurrentFade));
        set_fade_alpha(e, t, current);
    }
    let [dx, dy, dz] = distance_vector(e, t, camera);
    let squared = x87_store32(sum_of_squares(
        x87_load32(dx),
        x87_load32(dy),
        x87_load32(dz),
    ));
    let root = x87_store32(e.call(SQRT, &args![x87_load32(squared)]).st0);
    let distance = x87_store32(x87_mul(x87_load32(root), x87_m32(scale)));
    let mut fade = ONE;
    let factor = x87_load32(e.global::<u32>(DISTANCE_MULTIPLIER));
    let near = x87_store32(x87_mul(
        x87_load32(e.get(this, BSFadeNode::fNearDistSqr)),
        factor,
    ));
    let near_value = x87_load32(near);
    let distance_value = x87_load32(distance);
    // FCOM (near vs distance); TEST AH,0x5; JP: the fall-off only beyond the near distance.
    if near_value < distance_value {
        let beyond = x87_sub(distance_value, near_value);
        let far = x87_store32(x87_mul(
            factor,
            x87_load32(e.get(this, BSFadeNode::fFarDistSqr)),
        ));
        let range = x87_sub(x87_load32(far), near_value);
        let ratio = x87_div(beyond, range);
        fade = x87_store32(x87_sub(1.0, ratio));
    }
    let mult_type = e.get(this, BSFadeNode::eMultType);
    if mult_type == 10 {
        fade = if (e.get(this, BSFadeNode::m_uFlags) >> 28) & 1 != 0 {
            ONE
        } else {
            x87_store32(0.0)
        };
    }
    let fade_value = x87_load32(fade);
    let counter = e.global::<u32>(FADE_COUNTER) as i32;
    let last_seen = e.get(this, BSFadeNode::iFrameCounter) as i32;
    // FLD 600; FCOMP [fBoundRadius]; TEST AH,0x41; JNZ: only a radius under 600 snaps.
    if mult_type != 8
        && last_seen < counter.wrapping_sub(0x14)
        && x87_load32(e.global::<u32>(SNAP_RADIUS)) > x87_m32(e.get(this, BSFadeNode::fBoundRadius))
    {
        let fade_in = x87_load32(e.global::<u32>(FADE_IN_THRESHOLD));
        let snap_in = if fade_in < fade_value {
            true
        } else {
            (e.get(this, BSFadeNode::m_uFlags) >> 14) & 1 != 0
                && x87_load32(e.global::<u32>(FADE_OUT_THRESHOLD)) < fade_value
        };
        if snap_in {
            set_fade_alpha(e, t, ONE);
            e.set(this, BSFadeNode::fCurrentFade, ONE);
            let flags = e.get(this, BSFadeNode::m_uFlags);
            e.set(this, BSFadeNode::m_uFlags, flags | 0x4000);
            e.set(this, BSFadeNode::fLastFade, ONE);
        } else {
            set_fade_alpha(e, t, 0);
            e.set(this, BSFadeNode::fCurrentFade, 0);
            let flags = e.get(this, BSFadeNode::m_uFlags);
            e.set(this, BSFadeNode::m_uFlags, flags & 0xffff_bfff);
            e.set(this, BSFadeNode::fLastFade, 0);
        }
    }
    let counter = e.global::<u32>(FADE_COUNTER);
    e.set(this, BSFadeNode::iFrameCounter, counter);
    let most = fn_00b4df80(e, this.cast()).to_bits();
    let most_value = x87_load32(most);
    let faded_in = (e.get(this, BSFadeNode::m_uFlags) >> 14) & 1 != 0;
    let mut grouping = false;
    if faded_in && 0.0 < most_value && 1.0 > most_value {
        let accumulator = e.mem.u32(e.global::<u32>(SCENE_GLOBAL).wrapping_add(8));
        if e.call(
            ACCUMULATOR_CHECK_0043B300,
            &args![ACCUMULATOR_CHECK_ARGUMENT, accumulator],
        )
        .u8()
            != 0
        {
            let bound = match e.get(this, BSFadeNode::m_kWorldBound) {
                0 => DEFAULT_BOUND,
                bound => bound,
            };
            e.call(START_GROUPING_ALPHAS, &args![accumulator, bound, 1u32]);
            grouping = true;
            set_fade_alpha(e, t, most);
            set_fades(e, this, most);
        }
    } else {
        let fade_in = x87_load32(e.global::<u32>(FADE_IN_THRESHOLD));
        let current = x87_m32(e.get(this, BSFadeNode::fCurrentFade));
        let step_in = if !faded_in && fade_value > fade_in {
            true
        } else {
            0.0 < current && 1.0 > current && fade_in < fade_value
        };
        if step_in {
            fade_in_step(e, this);
        } else {
            let fade_out = x87_load32(e.global::<u32>(FADE_OUT_THRESHOLD));
            let step_out = if faded_in && fade_out > fade_value {
                true
            } else {
                let current = x87_m32(e.get(this, BSFadeNode::fCurrentFade));
                1.0 > current && 0.0 < current && fade_value < fade_out
            };
            if step_out {
                fade_out_step(e, this);
            }
        }
    }
    // FLDZ; FCOM [fCurrentFade] (JP when not 0 < it); FCOMP [most] (JNP when 0 == it).
    if 0.0 < x87_m32(e.get(this, BSFadeNode::fCurrentFade)) && x87_m32(most) != 0.0 {
        e.call(NODE_ON_VISIBLE, &args![this, process]);
    }
    if grouping {
        let accumulator = e.mem.u32(e.global::<u32>(SCENE_GLOBAL).wrapping_add(8));
        e.call(STOP_GROUPING_ALPHAS, &args![accumulator]);
    }
}

// Translated from 00b4ea20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode`'s vtable slot +0xA8 (the Xbox PDB's `UpdateSelectedDownwardPass` slot;
/// Ghidra made no function of it): `thiscall`, two words (the update data, flags). With
/// fading off: the update's first float goes to `fTimeSinceUpdate`, and when it is positive
/// and `00c80090` answers false, the first child's controller (+0xC) gets `00653290` with
/// `011f36ac`. Then `NiNode::UpdateSelectedDownwardPass` with the same two words.
pub fn fn_00b4ea20(e: &mut Engine, this: Ptr<BSFadeNode>, update: Ptr, flags: u32) {
    if e.global::<u8>(FADE_ENABLED) == 0 {
        let time = x87_quiet_bits(e.mem.u32(update.addr()));
        e.set(this, BSFadeNode::fTimeSinceUpdate, time);
        // FLDZ; FCOMP [time]; TEST AH,0x5; JP: only when 0 < the time.
        if 0.0 < x87_m32(e.mem.u32(update.addr()))
            && e.call(NODE_CHECK_00C80090, &args![this]).u8() == 0
            && e.get(this, BSFadeNode::m_kChildren_usSize) != 0
        {
            let child = e.mem.u32(e.get(this, BSFadeNode::m_kChildren_pBase));
            if child != 0 {
                let controller = e.mem.u32(child.wrapping_add(0xc));
                if controller != 0 {
                    e.call(
                        CONTROLLER_CALL_00653290,
                        &args![controller, CONTROLLER_ARGUMENT],
                    );
                }
            }
        }
    }
    e.call(
        NODE_UPDATE_SELECTED_DOWNWARD_PASS,
        &args![this, update, flags],
    );
}

// Translated from 00b4eaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `BSFadeNode`'s create function (no arguments): a new node
/// (0xE4 bytes, the constructor), or 0 when the allocation failed.
// Translation incomplete: 00b4eaa0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the constructor) is not translated.
pub fn fn_00b4eaa0(e: &mut Engine) -> Ptr {
    let block = e.call(NEW_OBJECT, &args![0xe4u32]).u32();
    if block != 0 {
        bs_fade_node_construct(e, Ptr::new(block)).cast()
    } else {
        Ptr::new(0)
    }
}

// Translated from 00b4eb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode::CheckFadeRadius` (Xbox PDB): `thiscall`, two words (the camera; the second
/// is not read). Answers whether the node is drawn faded in. Fading off: faded in at once.
/// Type 6 fades out (flag `0x4000` and both fades cleared); type 10 answers true at once; far
/// distances over 2600 are capped to 2600 / 1300 and a node not seen for 20 fade frames
/// fades out as type 6. The bound radius is copied. Types 1, 4, 5 and 6 only store the frame
/// counter and answer false. With a positive multiplier, the camera distance (at least 0)
/// times `m_fLODAdjust` over it answers false beyond the far distance (times
/// `fDistanceMultiplier`), and between near and far when the linear fade is under
/// `fFadeOutAlphaThreshold`; else the node is faded in (flag `0x4000`, fades 1, the alpha
/// down the subtree) and the answer is true.
pub fn bs_fade_node_check_fade_radius(
    e: &mut Engine,
    this: Ptr<BSFadeNode>,
    camera: Ptr,
    _unused_2: u32,
) -> bool {
    let t = this.addr();
    if e.global::<u8>(FADE_ENABLED) != 0 {
        let cam = camera.addr();
        let adjust = x87_quiet_bits(e.mem.u32(cam.wrapping_add(CAMERA_LOD_ADJUST)));
        let mult_type = e.get(this, BSFadeNode::eMultType) as i32;
        let [dx, dy, dz] = distance_vector(e, t, cam);
        let mut multiplier = ONE;
        if mult_type > 0 && mult_type < 0xb {
            multiplier = x87_quiet_bits(e.mem.u32(MULTIPLIERS + 4 * mult_type as u32));
        }
        let mut fade_out = false;
        if mult_type == 6 {
            fade_out = true;
        } else {
            if mult_type == 10 {
                return true;
            }
            let capped_far = e.global::<u32>(CAPPED_FAR);
            // FLD 2600; FCOM [far]; TEST AH,0x5; JP: cap only when 2600 < far.
            if x87_load32(capped_far) < x87_m32(e.get(this, BSFadeNode::fFarDistSqr)) {
                e.set(this, BSFadeNode::fFarDistSqr, x87_quiet_bits(capped_far));
                let capped_near = x87_quiet_bits(e.global::<u32>(CAPPED_NEAR));
                e.set(this, BSFadeNode::fNearDistSqr, capped_near);
                let counter = e.global::<u32>(FADE_COUNTER) as i32;
                if (e.get(this, BSFadeNode::iFrameCounter) as i32) < counter.wrapping_sub(0x14) {
                    fade_out = true;
                }
            }
        }
        if fade_out {
            let flags = e.get(this, BSFadeNode::m_uFlags);
            e.set(this, BSFadeNode::m_uFlags, flags & 0xffff_bfff);
            set_fades(e, this, x87_store32(0.0));
        }
        let bound = e.get(this, BSFadeNode::m_kWorldBound);
        if bound != 0 {
            let radius = x87_quiet_bits(e.mem.u32(bound.wrapping_add(0xc)));
            e.set(this, BSFadeNode::fBoundRadius, radius);
        }
        if matches!(mult_type, 1 | 5 | 4 | 6) {
            let counter = e.global::<u32>(FADE_COUNTER);
            e.set(this, BSFadeNode::iFrameCounter, counter);
            return false;
        }
        let multiplier = x87_load32(multiplier);
        // FCOM (multiplier vs 0); TEST AH,0x41; JNZ: only a positive multiplier tests.
        if multiplier > 0.0 {
            let adjust = x87_store32(x87_div(x87_m32(adjust), multiplier));
            let length = e.with_stack(0xc, |e, vector| {
                for (i, word) in [dx, dy, dz].iter().enumerate() {
                    e.mem.set_u32(vector.addr() + 4 * i as u32, *word);
                }
                e.call(POINT_LENGTH, &args![vector]).st0
            });
            let mut length = x87_store32(length);
            // FLDZ; FCOM [length]; TEST AH,0x41; JNZ: 0 replaces a negative length.
            if 0.0 > x87_m32(length) {
                length = x87_store32(0.0);
            }
            let distance = x87_store32(x87_mul(x87_load32(length), x87_m32(adjust)));
            let factor = x87_load32(e.global::<u32>(DISTANCE_MULTIPLIER));
            let near = x87_store32(x87_mul(
                x87_load32(e.get(this, BSFadeNode::fNearDistSqr)),
                factor,
            ));
            let far = x87_store32(x87_mul(
                factor,
                x87_m32(e.get(this, BSFadeNode::fFarDistSqr)),
            ));
            let distance_value = x87_load32(distance);
            // FLD far; FCOM (far vs distance); TEST AH,0x5; JP: false only when far < it.
            if x87_load32(far) < distance_value {
                return false;
            }
            let near_value = x87_load32(near);
            // FLD near; FCOM ST2 (near vs distance); TEST AH,0x5; JP: in when not near < it.
            if near_value < distance_value {
                let beyond = x87_sub(distance_value, near_value);
                let range = x87_store32(x87_sub(x87_load32(far), near_value));
                let ratio = x87_div(beyond, x87_m32(range));
                let fade = x87_store32(x87_sub(1.0, ratio));
                // FLD threshold; FCOMPP (threshold vs fade); TEST AH,0x41; JNZ: in unless
                // the threshold is above the fade.
                if x87_load32(e.global::<u32>(FADE_OUT_THRESHOLD)) > x87_load32(fade) {
                    return false;
                }
            }
        }
    }
    let flags = e.get(this, BSFadeNode::m_uFlags);
    e.set(this, BSFadeNode::m_uFlags, flags | 0x4000);
    set_fades(e, this, ONE);
    set_fade_alpha(e, t, ONE);
    true
}

// Translated from 00b4ed40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode`'s scalar deleting destructor: `thiscall`, one word (flags). The destructor,
/// then with bit 0 the block goes back (`00aa1460`, 0xE4 bytes); answers the node.
pub fn fn_00b4ed40(e: &mut Engine, this: Ptr<BSFadeNode>, flags: u32) -> Ptr<BSFadeNode> {
    fn_00b4e1e0(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xe4u32]);
    }
    this
}

/// `NiNode::RegisterStreamables` (`00a5e5b0`), `NiNode::SaveBinary` (`00a5e880`) and
/// `NiNode::IsEqual` (`00a5e600`) (engine map; thiscall, one word each).
const NODE_REGISTER_STREAMABLES: u32 = 0x00a5_e5b0;
const NODE_SAVE_BINARY: u32 = 0x00a5_e880;
const NODE_IS_EQUAL: u32 = 0x00a5_e600;

// Translated from 00b4def0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode::RegisterStreamables` (Xbox PDB): thiscall, one word (the stream). Answers
/// whether `NiNode::RegisterStreamables` (`00a5e5b0`) answered a non-zero byte.
pub fn bs_fade_node_register_streamables(e: &mut Engine, this: Ptr, stream: u32) -> bool {
    e.call(NODE_REGISTER_STREAMABLES, &args![this, stream]).u8() != 0
}

// Translated from 00b4df10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode::SaveBinary` (Xbox PDB): a jump to `NiNode::SaveBinary` (`00a5e880`) with the
/// stream.
pub fn bs_fade_node_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(NODE_SAVE_BINARY, &args![this, stream]);
}

// Translated from 00b4df20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode::IsEqual` (Xbox PDB): a jump to `NiNode::IsEqual` (`00a5e600`) with the other
/// object; answers the callee's `EAX` unchanged (its `bool` is the low byte).
pub fn bs_fade_node_is_equal(e: &mut Engine, this: Ptr, other: u32) -> u32 {
    e.call(NODE_IS_EQUAL, &args![this, other]).u32()
}

/// `NiNode::GetViewerStrings` (engine map, `00a5ea70`; thiscall, the string array).
const NODE_GET_VIEWER_STRINGS: u32 = 0x00a5_ea70;
/// `NiGetViewerString` overloads (engine map; cdecl): the class name (`00a7d330`, one word),
/// a name with a `float` (`00a7d550`), an integer (`00a7d4d0`) and a `bool` word
/// (`00a7d370`).
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const VIEWER_STRING_INT: u32 = 0x00a7_d4d0;
const VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
/// The viewer-string array's grow (`0096ad30`; thiscall, the new size) and set-at
/// (`0096ae90`; thiscall: the index, the address of the string word).
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The bounded print `00406d00` (cdecl: buffer, size, format, values).
const PRINT_00406D00: u32 = 0x0040_6d00;
/// The `double` the two distances are compared with (`010231b0`: `FLT_MAX`).
const DISABLED_DISTANCE: u32 = 0x0102_31b0;
/// The texts (`.rdata`).
const NEAR_DISABLED_TEXT: u32 = 0x010a_917c;
const NEAR_FORMAT: u32 = 0x010a_9168;
const FAR_DISABLED_TEXT: u32 = 0x010a_9150;
const FAR_FORMAT: u32 = 0x010a_913c;
const MULT_TYPE_FORMAT: u32 = 0x010a_912c;
const LAST_FADE_NAME: u32 = 0x010a_9120;
const CURRENT_FADE_NAME: u32 = 0x010a_9110;
const TIME_SINCE_UPDATE_NAME: u32 = 0x010a_90fc;
const FRAME_COUNTER_NAME: u32 = 0x010a_90ec;
const FADED_IN_NAME: u32 = 0x010a_90e0;
const IGNORE_FADE_NAME: u32 = 0x010a_90d4;
const LOD_FADING_OUT_NAME: u32 = 0x010a_90c4;
const HAS_MOVING_SOUND_NAME: u32 = 0x010a_90b4;
const HIGH_DETAIL_NAME: u32 = 0x010a_90a8;
/// `NiObjectNET::GetExtraData` (engine map, `00a5bdd0`; thiscall, the name) and the global
/// holding the `BSXFlags` extra data's name (`01202ddc`).
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
const BSX_FLAGS_NAME: u32 = 0x0120_2ddc;

/// Appends the string word in `slot` to the viewer-string array: grown first (by its step
/// `+0xE` plus the size) when its 16-bit size (`+0xA`) has reached the capacity (`+0x8`).
fn append_viewer_string(e: &mut Engine, strings: u32, slot: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &[strings, step + size]);
    }
    e.call(STRINGS_SET_AT, &[strings, size, slot]);
}

/// A 0x40-byte buffer from the memory manager, kept in `slot`, printed with either the
/// disabled text (when `value` equals the `double` at `010231b0`) or `format` and the value
/// as a `double`.
fn print_distance(e: &mut Engine, slot: u32, value: f64, disabled: u32, format: u32) {
    let buffer = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 0x40]).u32();
    e.mem.set_u32(slot, buffer);
    // FCOM double; TEST AH,0x44; JP: the disabled text only when equal (not unordered).
    if value == e.global::<f64>(DISABLED_DISTANCE) {
        e.call(PRINT_00406D00, &[buffer, 0x40, disabled]);
    } else {
        let mut words = vec![buffer, 0x40, format];
        value.put(&mut words);
        e.call(PRINT_00406D00, &words);
    }
}

// Translated from 00b4ed70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNode::GetViewerStrings` (Xbox PDB): thiscall, one word (the viewer-string array).
/// Runs `NiNode::GetViewerStrings`, then appends the class name, `fNearDistSqr` and
/// `fFarDistSqr` (printed into 0x40-byte buffers, `DISABLED` at `FLT_MAX`), `eMultType`,
/// `fLastFade`, `fCurrentFade`, `fTimeSinceUpdate`, `iFrameCounter`, and the flags
/// `bFadedIn` (bit 14 of `m_uFlags`), `bIgnoreFade` (bit 15), `bLODFadingOut` (bit 16),
/// `bHasMovingSound` (bit 17) and `bHighDetail` (bit 24); then, when the node has
/// `BSXFlags` extra data (`GetExtraData` with the name at `01202ddc`), that object's
/// virtual slot +0x60 with the array.
///
/// The string words live in the argument's own stack word (the array's pointer is kept in a
/// register): each `bool` is stored into that word's low byte and the whole word pushed, so
/// its upper bytes are those of the previous string pointer.
pub fn bs_fade_node_get_viewer_strings(e: &mut Engine, this: Ptr<BSFadeNode>, strings: Ptr) {
    let node = this.addr();
    let strings = strings.addr();
    e.call(NODE_GET_VIEWER_STRINGS, &[node, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, strings);
        let class_name = e.global::<u32>(FADE_NODE_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &[class_name]).u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);

        // FLD/FSTP into locals before the first allocation.
        let near = x87_load32(e.get(this, BSFadeNode::fNearDistSqr));
        let far = x87_load32(e.get(this, BSFadeNode::fFarDistSqr));
        print_distance(e, slot, near, NEAR_DISABLED_TEXT, NEAR_FORMAT);
        append_viewer_string(e, strings, slot);
        print_distance(e, slot, far, FAR_DISABLED_TEXT, FAR_FORMAT);
        append_viewer_string(e, strings, slot);

        let buffer = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 0x40]).u32();
        let mult_type = e.get(this, BSFadeNode::eMultType);
        e.mem.set_u32(slot, buffer);
        e.call(PRINT_00406D00, &[buffer, 0x40, MULT_TYPE_FORMAT, mult_type]);
        append_viewer_string(e, strings, slot);

        // `fLastFade`, `fCurrentFade`, `fTimeSinceUpdate`, each loaded just before its call
        // and pushed through FLD/FSTP.
        for (name, field) in [
            (LAST_FADE_NAME, BSFadeNode::fLastFade),
            (CURRENT_FADE_NAME, BSFadeNode::fCurrentFade),
            (TIME_SINCE_UPDATE_NAME, BSFadeNode::fTimeSinceUpdate),
        ] {
            let bits = x87_quiet_bits(e.get(this, field));
            let text = e.call(VIEWER_STRING_FLOAT, &[name, bits]).u32();
            e.mem.set_u32(slot, text);
            append_viewer_string(e, strings, slot);
        }

        let counter = e.get(this, BSFadeNode::iFrameCounter);
        let text = e
            .call(VIEWER_STRING_INT, &[FRAME_COUNTER_NAME, counter])
            .u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);

        for (name, bit) in [
            (FADED_IN_NAME, 14),
            (IGNORE_FADE_NAME, 15),
            (LOD_FADING_OUT_NAME, 16),
            (HAS_MOVING_SOUND_NAME, 17),
            (HIGH_DETAIL_NAME, 24),
        ] {
            let flag = (e.get(this, BSFadeNode::m_uFlags) >> bit) & 1;
            e.mem.set_u8(slot, flag as u8);
            let word = e.mem.u32(slot);
            let text = e.call(VIEWER_STRING_BOOL, &[name, word]).u32();
            e.mem.set_u32(slot, text);
            append_viewer_string(e, strings, slot);
        }
    });
    let name = e.global::<u32>(BSX_FLAGS_NAME);
    let extra = e.call(GET_EXTRA_DATA, &[node, name]).u32();
    if extra != 0 {
        e.vcall(extra, 0x60, &[strings]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b4_def0,
            bs_fade_node_register_streamables(Ptr, u32) -> bool
        ),
        entry!(0x00b4_df10, bs_fade_node_save_binary(Ptr, u32)),
        entry!(0x00b4_df20, bs_fade_node_is_equal(Ptr, u32) -> u32),
        entry!(
            0x00b4_ed70,
            bs_fade_node_get_viewer_strings(Ptr<BSFadeNode>, Ptr)
        ),
        entry!(
            0x00b4_dec0,
            bs_fade_node_set_lod_mult_type(Ptr<BSFadeNode>, u32)
        ),
        entry!(0x00b4_ded0, fn_00b4ded0(Ptr, u32, u32)),
        entry!(0x00b4_dee0, bs_fade_node_load_binary(Ptr, u32)),
        entry!(0x00b4_df30, fn_00b4df30(Ptr)),
        entry!(0x00b4_df80, fn_00b4df80(Ptr) -> f32),
        entry!(0x00b4_dfb0, fn_00b4dfb0(u32)),
        entry!(
            0x00b4_dfd0,
            bs_fade_node_set_range(Ptr<BSFadeNode>, f32, f32)
        ),
        entry!(0x00b4_e030, fn_00b4e030(Ptr<BSFadeNode>, Ptr, f32) -> bool),
        entry!(
            0x00b4_e150,
            bs_fade_node_construct(Ptr<BSFadeNode>) -> Ptr<BSFadeNode>
        ),
        entry!(0x00b4_e1d0, bs_fade_node_get_rtti(Ptr) -> u32),
        entry!(0x00b4_e1e0, fn_00b4e1e0(Ptr<BSFadeNode>)),
        entry!(0x00b4_e270, bs_fade_node_init_sdm()),
        entry!(0x00b4_e380, fn_00b4e380()),
        entry!(
            0x00b4_e3f0,
            bs_fade_node_create_clone(Ptr<BSFadeNode>, u32) -> Ptr
        ),
        entry!(0x00b4_e490, bs_fade_node_on_visible(Ptr<BSFadeNode>, Ptr)),
        entry!(0x00b4_ea20, fn_00b4ea20(Ptr<BSFadeNode>, Ptr, u32)),
        entry!(0x00b4_eaa0, fn_00b4eaa0() -> Ptr),
        entry!(
            0x00b4_eb10,
            bs_fade_node_check_fade_radius(Ptr<BSFadeNode>, Ptr, u32) -> bool
        ),
        entry!(
            0x00b4_ed40,
            fn_00b4ed40(Ptr<BSFadeNode>, u32) -> Ptr<BSFadeNode>
        ),
    ]
}

#[cfg(test)]
mod b048_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    /// The fade statics and the doubles every fade path calls.
    fn rig(e: &mut Engine) {
        e.map(FADE_ENABLED, 0x50);
        e.map(MAX_ALPHA_ACCESS, 0x30);
        e.map(FRAME_TIME, 4);
        e.map(SCENE_GLOBAL, 4);
        for a in [
            UNSCALED_DISTANCE,
            SNAP_RADIUS,
            DEFAULT_DISTANCE,
            CAPPED_NEAR,
            EMPTY_STRING_HANDLE,
        ] {
            e.map(a, 8);
        }
        e.set_global(FADE_ENABLED, 1u8);
        for i in 0..11u32 {
            e.set_global(MULTIPLIERS + 4 * i, bits(1.0));
        }
        e.set_global(FADE_IN_TIME, bits(2.0));
        e.set_global(FADE_OUT_TIME, bits(2.0));
        e.set_global(FADE_IN_THRESHOLD, bits(0.7));
        e.set_global(FADE_OUT_THRESHOLD, bits(0.3));
        e.set_global(DISTANCE_MULTIPLIER, bits(1.0));
        e.set_global(MAX_FADE_INCREMENT, bits(0.1));
        e.set_global(FRAME_TIME, bits(0.5));
        e.set_global(FADE_COUNTER, 100u32);
        e.set_global(UNSCALED_DISTANCE, bits(0.2));
        e.set_global(SNAP_RADIUS, bits(600.0));
        e.set_global(CAPPED_FAR, bits(2600.0));
        e.set_global(CAPPED_NEAR, bits(1300.0));
        let scene = e.mem.alloc(0x10);
        e.mem.set_u32(scene + 8, 0xacc0);
        e.set_global(SCENE_GLOBAL, scene);
        for a in [
            NODE_ON_VISIBLE,
            SET_FADE_ALPHA,
            START_GROUPING_ALPHAS,
            STOP_GROUPING_ALPHAS,
        ] {
            e.register(a, nop);
        }
        e.register(FLOAT_MIN, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            st0(f64::from(if !(y > x) { y } else { x }))
        });
        e.register(FLOAT_MAX, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            st0(f64::from(if !(y < x) { y } else { x }))
        });
        e.register(SQRT, |_, a| {
            st0(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt())
        });
        e.register(POINT_LENGTH, |e, a| {
            let c: Vec<f64> = (0..3)
                .map(|i| f64::from(f32::from_bits(e.mem.u32(a[0] + 4 * i))))
                .collect();
            st0((c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt())
        });
        e.register(ACCUMULATOR_CHECK_0043B300, |_, _| 1u32.into_ret());
    }

    /// A fade node at (0, y, z) with near 1000 and far 2000, a bound of radius 10, and a
    /// culling process whose camera sits at the origin with an LOD adjust of 1.
    fn node_and_process(e: &mut Engine, y: f32, z: f32) -> (Ptr<BSFadeNode>, Ptr, Ptr) {
        let node = e.new_object::<BSFadeNode>();
        let n = node.addr();
        e.mem.set_u32(n + 0x90, bits(y));
        e.mem.set_u32(n + 0x94, bits(z));
        e.set(node, BSFadeNode::fNearDistSqr, bits(1000.0));
        e.set(node, BSFadeNode::fFarDistSqr, bits(2000.0));
        let bound = e.mem.alloc(0x10);
        e.mem.set_u32(bound + 0xc, bits(10.0));
        e.set(node, BSFadeNode::m_kWorldBound, bound);
        e.set(node, BSFadeNode::iFrameCounter, 95);
        let camera = e.mem.alloc(0x120);
        e.mem.set_u32(camera + CAMERA_LOD_ADJUST, bits(1.0));
        let process = e.mem.alloc(0x10);
        e.mem.set_u32(process + 0xc, camera);
        (node, Ptr::new(process), Ptr::new(camera))
    }

    #[test]
    fn small_setters_and_forwarders() {
        let mut e = Engine::new();
        let node = e.new_object::<BSFadeNode>();
        e.call(0x00b4_dec0, &args![node, 7u32]);
        assert_eq!(e.get(node, BSFadeNode::eMultType), 7);
        e.register(NODE_UPDATE_DOWNWARD_PASS, nop);
        e.register(NODE_LOAD_BINARY, nop);
        e.call_log = Some(vec![]);
        e.call(0x00b4_ded0, &args![node, 1u32, 2u32]);
        e.call(0x00b4_dee0, &args![node, 3u32]);
        assert_eq!(
            calls(&e, NODE_UPDATE_DOWNWARD_PASS),
            vec![vec![node.addr(), 1, 2]]
        );
        assert_eq!(calls(&e, NODE_LOAD_BINARY), vec![vec![node.addr(), 3]]);
        assert_eq!(e.call(0x00b4_e1d0, &args![node]).u32(), FADE_NODE_RTTI);
    }

    #[test]
    fn list_clear_frees_every_node_and_clears_the_item() {
        let mut e = Engine::new();
        e.register(MEMORY_DEALLOCATE, nop);
        let list = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        let third = e.mem.alloc(8);
        e.mem.set_u32(list, 0x11);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(second, 0x22);
        e.mem.set_u32(second + 4, third);
        e.mem.set_u32(third, 0x33);
        e.call_log = Some(vec![]);
        e.call(0x00b4_df30, &args![list]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, second], vec![MEMORY_MANAGER, third]]
        );
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(e.mem.u32(list + 4), 0);
        // Each freed node had its own item cleared by the recursion.
        assert_eq!(e.mem.u32(second), 0);
        assert_eq!(e.mem.u32(third), 0);
    }

    #[test]
    fn most_alpha_calls_the_access_function_when_set() {
        let mut e = Engine::new();
        rig(&mut e);
        let node = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00b4_df80, &args![node]).f32(), 1.0);
        e.set_global(MAX_ALPHA_ACCESS, 0x6200_0000u32);
        e.register(0x6200_0000, |_, a| st0(f64::from(a[0] & 0xff) + 0.1));
        let value = e.call(0x00b4_df80, &args![node]).f32();
        assert_eq!(value, (f64::from(node & 0xff) + 0.1) as f32);
    }

    #[test]
    fn fade_node_flag_on_what_virtual_0x10_answers() {
        let mut e = Engine::new();
        let table: Vec<u32> = (0..8).map(|n| 0x6100_0000 + 4 * n).collect();
        e.put_vtable(0x6000, &table);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, 0x6000);
        e.register(0x6100_0010, |_, a| a[0].into_ret());
        e.call(0x00b4_dfb0, &args![0u32]);
        e.call(0x00b4_dfb0, &args![object]);
        assert_eq!(e.mem.u32(object + 0x30), 0x2_0000);
        e.register(0x6100_0010, |_, _| 0u32.into_ret());
        e.mem.set_u32(object + 0x30, 0);
        e.call(0x00b4_dfb0, &args![object]);
        assert_eq!(e.mem.u32(object + 0x30), 0);
    }

    #[test]
    fn set_range_raises_short_ranges_to_the_least_near_distance() {
        let mut e = Engine::new();
        rig(&mut e);
        e.set_global(MIN_NEAR_DISTANCE, 100u32);
        let node = e.new_object::<BSFadeNode>();
        e.call(0x00b4_dfd0, &args![node, 50.0f32, 150.0f32]);
        assert_eq!(e.get(node, BSFadeNode::fNearDistSqr), bits(100.0));
        assert_eq!(e.get(node, BSFadeNode::fFarDistSqr), bits(200.0));
        e.call(0x00b4_dfd0, &args![node, 50.0f32, 300.0f32]);
        assert_eq!(e.get(node, BSFadeNode::fFarDistSqr), bits(300.0));
        e.call(0x00b4_dfd0, &args![node, 150.0f32, 120.0f32]);
        assert_eq!(e.get(node, BSFadeNode::fNearDistSqr), bits(150.0));
        assert_eq!(e.get(node, BSFadeNode::fFarDistSqr), bits(120.0));
    }

    #[test]
    fn near_enough_compares_the_scaled_distance_with_the_far_distance() {
        let mut e = Engine::new();
        rig(&mut e);
        let (node, _, camera) = node_and_process(&mut e, 300.0, 400.0);
        e.mem.set_u32(camera.addr() + CAMERA_LOD_ADJUST, bits(4.0));
        e.set_global(MULTIPLIERS, bits(2.0));
        // Distance 500 * 1.0, times 4 / 2: 1000 against far 2000.
        assert!(e.call(0x00b4_e030, &args![node, camera, 1.0f32]).bool());
        e.set(node, BSFadeNode::fFarDistSqr, bits(900.0));
        assert!(!e.call(0x00b4_e030, &args![node, camera, 1.0f32]).bool());
        // A negative scale counts as 0.
        assert!(e.call(0x00b4_e030, &args![node, camera, -1.0f32]).bool());
        e.set(node, BSFadeNode::eMultType, 9);
        e.set(node, BSFadeNode::fFarDistSqr, bits(5000.0));
        assert!(!e.call(0x00b4_e030, &args![node, camera, 1.0f32]).bool());
    }

    #[test]
    fn constructor_clone_and_create() {
        let mut e = Engine::new();
        rig(&mut e);
        e.set_global(DEFAULT_DISTANCE, bits(1500.0));
        e.register(NODE_CONSTRUCT, nop);
        e.register(NODE_COPY_MEMBERS, nop);
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        let node = e.new_object::<BSFadeNode>();
        e.set(node, BSFadeNode::m_uFlags, 0xffff_ffff);
        e.set(node, BSFadeNode::fCurrentFade, 7);
        e.set(node, BSFadeNode::iFrameCounter, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_e150, &args![node]).u32(), node.addr());
        assert_eq!(calls(&e, NODE_CONSTRUCT), vec![vec![node.addr(), 0]]);
        assert_eq!(e.mem.u32(node.addr()), FADE_NODE_VTABLE);
        assert_eq!(e.get(node, BSFadeNode::m_uFlags), 0xfffd_3fff);
        assert_eq!(e.get(node, BSFadeNode::fNearDistSqr), bits(1500.0));
        assert_eq!(e.get(node, BSFadeNode::fFarDistSqr), bits(1500.0));
        assert_eq!(e.get(node, BSFadeNode::fCurrentFade), 0);
        // iFrameCounter is left as it was.
        assert_eq!(e.get(node, BSFadeNode::iFrameCounter), 9);
        e.set(node, BSFadeNode::fNearDistSqr, bits(10.0));
        e.set(node, BSFadeNode::fFarDistSqr, bits(20.0));
        e.set(node, BSFadeNode::eMultType, 4);
        let clone = e.call(0x00b4_e3f0, &args![node, 0x55u32]).u32();
        let clone_node: Ptr<BSFadeNode> = Ptr::new(clone);
        assert_eq!(
            calls(&e, NODE_COPY_MEMBERS),
            vec![vec![node.addr(), clone, 0x55]]
        );
        assert_eq!(e.mem.u32(clone), FADE_NODE_VTABLE);
        assert_eq!(e.get(clone_node, BSFadeNode::fNearDistSqr), bits(10.0));
        assert_eq!(e.get(clone_node, BSFadeNode::fFarDistSqr), bits(20.0));
        assert_eq!(e.get(clone_node, BSFadeNode::eMultType), 4);
        let created = e.call(0x00b4_eaa0, &args![]).u32();
        assert_eq!(e.mem.u32(created), FADE_NODE_VTABLE);
        e.register(NEW_OBJECT, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00b4_eaa0, &args![]).u32(), 0);
    }

    #[test]
    fn destructors_clear_both_lists_twice() {
        let mut e = Engine::new();
        e.register(NODE_DESTRUCT, nop);
        e.register(SIZED_DELETE, nop);
        e.register(MEMORY_DEALLOCATE, nop);
        let node = e.new_object::<BSFadeNode>();
        let room = e.mem.alloc(8);
        e.mem.set_u32(node.addr() + PARENT_ROOM_LIST + 4, room);
        e.mem.set_u32(node.addr() + PARENT_PORTAL_LIST, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_ed40, &args![node, 1u32]).u32(), node.addr());
        assert_eq!(e.mem.u32(node.addr()), FADE_NODE_VTABLE);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, room]]
        );
        assert_eq!(e.mem.u32(node.addr() + PARENT_PORTAL_LIST), 0);
        assert_eq!(calls(&e, NODE_DESTRUCT), vec![vec![node.addr()]]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![node.addr(), 0xe4]]);
        e.call(0x00b4_ed40, &args![node, 0u32]);
        assert_eq!(calls(&e, SIZED_DELETE).len(), 1);
    }

    #[test]
    fn init_sdm_and_its_counterpart() {
        let mut e = Engine::new();
        rig(&mut e);
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(PROPERTY_CONSTRUCT, nop);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_DEALLOCATE, nop);
        let handle_block = e.mem.alloc(0x10);
        let handle = handle_block + 8;
        e.register_double(FIXED_STRING_FROM_TEXT, move |_, _| handle.into_ret());
        let counts = Rc::new(RefCell::new(Vec::new()));
        let seen = counts.clone();
        e.register_double(INTERLOCKED_INCREMENT, move |e, a| {
            seen.borrow_mut().push(a[0]);
            let v = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.call(0x00b4_e270, &args![]);
        let alpha = e.global::<u32>(FADE_ALPHA);
        assert_ne!(alpha, 0);
        assert_eq!(e.mem.u32(alpha), ALPHA_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(alpha + 0x18), 0xed);
        assert_eq!(e.mem.u32(alpha + 4), 1);
        assert_eq!(*counts.borrow(), vec![alpha + 4]);
        let name = e.global::<u32>(FADE_NODE_ANIM_NAME);
        assert_eq!(e.mem.u32(name), handle);
        // The counterpart: the reference drops to 0 (DeleteThis), the handle count too.
        let table: Vec<u32> = (0..4).map(|n| 0x6100_0000 + 4 * n).collect();
        e.put_vtable(ALPHA_PROPERTY_VTABLE, &table);
        e.register(0x6100_0004, nop);
        e.mem.set_u32(handle_block, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b4_e380, &args![]);
        assert_eq!(calls(&e, 0x6100_0004), vec![vec![alpha]]);
        assert_eq!(e.global::<u32>(FADE_ALPHA), 0);
        assert_eq!(e.mem.u32(handle_block), 0);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, name]]
        );
        assert_eq!(e.global::<u32>(FADE_NODE_ANIM_NAME), 0);
    }

    #[test]
    fn on_visible_without_fading_is_just_visible() {
        let mut e = Engine::new();
        rig(&mut e);
        let (node, process, _) = node_and_process(&mut e, 0.0, 10.0);
        e.set_global(FADE_ENABLED, 0u8);
        e.call_log = Some(vec![]);
        e.call(0x00b4_e490, &args![node, process]);
        e.set_global(FADE_ENABLED, 1u8);
        e.set(node, BSFadeNode::m_uFlags, 0x8000);
        e.call(0x00b4_e490, &args![node, process]);
        assert_eq!(calls(&e, NODE_ON_VISIBLE).len(), 2);
        assert!(calls(&e, SET_FADE_ALPHA).is_empty());
    }

    #[test]
    fn on_visible_type_6_steps_the_fade_in() {
        let mut e = Engine::new();
        rig(&mut e);
        let (node, process, _) = node_and_process(&mut e, 0.0, 10.0);
        e.set(node, BSFadeNode::eMultType, 6);
        e.set(node, BSFadeNode::fCurrentFade, bits(0.95));
        e.call_log = Some(vec![]);
        e.call(0x00b4_e490, &args![node, process]);
        // min(0.5 / 2, 0.1) + 0.95, at most 1.
        assert_eq!(e.get(node, BSFadeNode::fCurrentFade), bits(1.0));
        assert_eq!(e.get(node, BSFadeNode::fLastFade), bits(1.0));
        assert_eq!(e.get(node, BSFadeNode::m_uFlags), 0x4000);
        assert_eq!(
            calls(&e, SET_FADE_ALPHA),
            vec![vec![node.addr(), bits(1.0)]]
        );
        // Faded in: only visible.
        e.call(0x00b4_e490, &args![node, process]);
        assert_eq!(calls(&e, SET_FADE_ALPHA).len(), 1);
        assert_eq!(calls(&e, NODE_ON_VISIBLE).len(), 2);
    }

    #[test]
    fn on_visible_near_steps_in_and_stores_the_counter() {
        let mut e = Engine::new();
        rig(&mut e);
        let (node, process, _) = node_and_process(&mut e, 300.0, 400.0);
        e.call_log = Some(vec![]);
        e.call(0x00b4_e490, &args![node, process]);
        let step = 0.1f32;
        assert_eq!(e.get(node, BSFadeNode::fCurrentFade), bits(step));
        assert_eq!(e.get(node, BSFadeNode::fLastFade), bits(step));
        assert_eq!(e.get(node, BSFadeNode::m_uFlags), 0);
        assert_eq!(e.get(node, BSFadeNode::iFrameCounter), 100);
        assert_eq!(
            calls(&e, NODE_ON_VISIBLE),
            vec![vec![node.addr(), process.addr()]]
        );
        assert!(calls(&e, START_GROUPING_ALPHAS).is_empty());
    }

    #[test]
    fn on_visible_snaps_a_small_node_not_seen_for_a_while() {
        let mut e = Engine::new();
        rig(&mut e);
        let (node, process, _) = node_and_process(&mut e, 300.0, 400.0);
        e.set(node, BSFadeNode::iFrameCounter, 0);
        e.set(node, BSFadeNode::fBoundRadius, bits(10.0));
        e.call_log = Some(vec![]);
        e.call(0x00b4_e490, &args![node, process]);
        assert_eq!(
            calls(&e, SET_FADE_ALPHA),
            vec![vec![node.addr(), bits(1.0)]]
        );
        assert_eq!(e.get(node, BSFadeNode::fCurrentFade), bits(1.0));
        assert_eq!(e.get(node, BSFadeNode::m_uFlags), 0x4000);
        assert_eq!(calls(&e, NODE_ON_VISIBLE).len(), 1);
        // Far away (fade below both thresholds) it snaps out and is not visible.
        let (far_node, far_process, _) = node_and_process(&mut e, 0.0, 2500.0);
        e.set(far_node, BSFadeNode::iFrameCounter, 0);
        e.set(far_node, BSFadeNode::fBoundRadius, bits(10.0));
        e.set(far_node, BSFadeNode::fCurrentFade, bits(0.5));
        e.call(0x00b4_e490, &args![far_node, far_process]);
        assert_eq!(e.get(far_node, BSFadeNode::fCurrentFade), 0);
        assert_eq!(e.get(far_node, BSFadeNode::m_uFlags), 0);
        assert_eq!(calls(&e, NODE_ON_VISIBLE).len(), 1);
    }

    #[test]
    fn on_visible_far_steps_the_fade_out() {
        let mut e = Engine::new();
        rig(&mut e);
        let (node, process, _) = node_and_process(&mut e, 0.0, 2500.0);
        e.set(node, BSFadeNode::m_uFlags, 0x4000);
        e.set(node, BSFadeNode::fCurrentFade, bits(1.0));
        e.call_log = Some(vec![]);
        e.call(0x00b4_e490, &args![node, process]);
        let expected = (1.0 - f64::from(0.1f32)) as f32;
        assert_eq!(e.get(node, BSFadeNode::fCurrentFade), bits(expected));
        assert_eq!(e.get(node, BSFadeNode::m_uFlags), 0x4000);
        assert_eq!(
            calls(&e, SET_FADE_ALPHA),
            vec![vec![node.addr(), bits(expected)]]
        );
        assert_eq!(calls(&e, NODE_ON_VISIBLE).len(), 1);
        // From 0.05 the fade reaches 0: the flag goes and the node is not visible.
        e.set(node, BSFadeNode::fCurrentFade, bits(0.05));
        e.call(0x00b4_e490, &args![node, process]);
        assert_eq!(e.get(node, BSFadeNode::fCurrentFade), 0);
        assert_eq!(e.get(node, BSFadeNode::m_uFlags), 0);
        assert_eq!(calls(&e, NODE_ON_VISIBLE).len(), 1);
    }

    #[test]
    fn on_visible_groups_alphas_for_a_partial_most_alpha() {
        let mut e = Engine::new();
        rig(&mut e);
        let (node, process, _) = node_and_process(&mut e, 300.0, 400.0);
        e.set(node, BSFadeNode::m_uFlags, 0x4000);
        e.set(node, BSFadeNode::fCurrentFade, bits(1.0));
        e.set_global(MAX_ALPHA_ACCESS, 0x6200_0000u32);
        e.register(0x6200_0000, |_, _| st0(0.5));
        e.call_log = Some(vec![]);
        e.call(0x00b4_e490, &args![node, process]);
        let bound = e.get(node, BSFadeNode::m_kWorldBound);
        assert_eq!(
            calls(&e, ACCUMULATOR_CHECK_0043B300),
            vec![vec![ACCUMULATOR_CHECK_ARGUMENT, 0xacc0]]
        );
        assert_eq!(
            calls(&e, START_GROUPING_ALPHAS),
            vec![vec![0xacc0, bound, 1]]
        );
        assert_eq!(
            calls(&e, SET_FADE_ALPHA),
            vec![vec![node.addr(), bits(0.5)]]
        );
        assert_eq!(e.get(node, BSFadeNode::fCurrentFade), bits(0.5));
        assert_eq!(e.get(node, BSFadeNode::fLastFade), bits(0.5));
        assert_eq!(calls(&e, NODE_ON_VISIBLE).len(), 1);
        assert_eq!(calls(&e, STOP_GROUPING_ALPHAS), vec![vec![0xacc0]]);
    }

    #[test]
    fn on_visible_with_a_non_positive_multiplier_marks_the_node_faded_in() {
        let mut e = Engine::new();
        rig(&mut e);
        let (node, process, _) = node_and_process(&mut e, 300.0, 400.0);
        e.set(node, BSFadeNode::eMultType, 7);
        e.set_global(MULTIPLIERS + 0x1c, bits(-1.0));
        e.call_log = Some(vec![]);
        e.call(0x00b4_e490, &args![node, process]);
        assert_eq!(
            calls(&e, SET_FADE_ALPHA),
            vec![vec![node.addr(), bits(1.0)]]
        );
        assert_eq!(e.get(node, BSFadeNode::fCurrentFade), bits(1.0));
        assert_eq!(e.get(node, BSFadeNode::m_uFlags), 0x4000);
    }

    #[test]
    fn update_selected_pass_records_the_time_without_fading() {
        let mut e = Engine::new();
        rig(&mut e);
        e.register(NODE_UPDATE_SELECTED_DOWNWARD_PASS, nop);
        e.register(NODE_CHECK_00C80090, |_, _| 0u32.into_ret());
        e.register(CONTROLLER_CALL_00653290, nop);
        let node = e.new_object::<BSFadeNode>();
        let child = e.mem.alloc(0x20);
        e.mem.set_u32(child + 0xc, 0x7700);
        let children = e.mem.alloc(8);
        e.mem.set_u32(children, child);
        e.set(node, BSFadeNode::m_kChildren_pBase, children);
        e.set(node, BSFadeNode::m_kChildren_usSize, 1);
        let update = e.mem.alloc(8);
        e.mem.set_u32(update, bits(0.25));
        e.call_log = Some(vec![]);
        e.call(0x00b4_ea20, &args![node, update, 3u32]);
        // Fading on: only the node's pass.
        assert!(calls(&e, CONTROLLER_CALL_00653290).is_empty());
        assert_eq!(e.get(node, BSFadeNode::fTimeSinceUpdate), 0);
        e.set_global(FADE_ENABLED, 0u8);
        e.call(0x00b4_ea20, &args![node, update, 3u32]);
        assert_eq!(e.get(node, BSFadeNode::fTimeSinceUpdate), bits(0.25));
        assert_eq!(
            calls(&e, CONTROLLER_CALL_00653290),
            vec![vec![0x7700, CONTROLLER_ARGUMENT]]
        );
        assert_eq!(
            calls(&e, NODE_UPDATE_SELECTED_DOWNWARD_PASS),
            vec![vec![node.addr(), update, 3], vec![node.addr(), update, 3]]
        );
    }

    #[test]
    fn check_fade_radius_answers_by_distance() {
        let mut e = Engine::new();
        rig(&mut e);
        let (node, _, camera) = node_and_process(&mut e, 300.0, 400.0);
        e.call_log = Some(vec![]);
        // 500: inside the near distance.
        assert!(e.call(0x00b4_eb10, &args![node, camera, 0u32]).bool());
        assert_eq!(e.get(node, BSFadeNode::fCurrentFade), bits(1.0));
        assert_eq!(e.get(node, BSFadeNode::m_uFlags), 0x4000);
        assert_eq!(e.get(node, BSFadeNode::fBoundRadius), bits(10.0));
        assert_eq!(
            calls(&e, SET_FADE_ALPHA),
            vec![vec![node.addr(), bits(1.0)]]
        );
        // 2500: beyond the far distance.
        e.mem.set_u32(node.addr() + 0x94, bits(2500.0));
        e.mem.set_u32(node.addr() + 0x90, 0);
        assert!(!e.call(0x00b4_eb10, &args![node, camera, 0u32]).bool());
        // 1900: fade 0.1, under the fade-out threshold.
        e.mem.set_u32(node.addr() + 0x94, bits(1900.0));
        assert!(!e.call(0x00b4_eb10, &args![node, camera, 0u32]).bool());
        // 1500: fade 0.5.
        e.mem.set_u32(node.addr() + 0x94, bits(1500.0));
        assert!(e.call(0x00b4_eb10, &args![node, camera, 0u32]).bool());
    }

    #[test]
    fn check_fade_radius_types_and_caps() {
        let mut e = Engine::new();
        rig(&mut e);
        let (node, _, camera) = node_and_process(&mut e, 300.0, 400.0);
        e.call_log = Some(vec![]);
        e.set(node, BSFadeNode::eMultType, 10);
        assert!(e.call(0x00b4_eb10, &args![node, camera, 0u32]).bool());
        assert!(calls(&e, SET_FADE_ALPHA).is_empty());
        e.set(node, BSFadeNode::eMultType, 1);
        e.set(node, BSFadeNode::iFrameCounter, 3);
        assert!(!e.call(0x00b4_eb10, &args![node, camera, 0u32]).bool());
        assert_eq!(e.get(node, BSFadeNode::iFrameCounter), 100);
        e.set(node, BSFadeNode::eMultType, 6);
        e.set(node, BSFadeNode::m_uFlags, 0x4001);
        e.set(node, BSFadeNode::fCurrentFade, bits(1.0));
        assert!(!e.call(0x00b4_eb10, &args![node, camera, 0u32]).bool());
        assert_eq!(e.get(node, BSFadeNode::m_uFlags), 1);
        assert_eq!(e.get(node, BSFadeNode::fCurrentFade), 0);
        // A far distance over 2600 is capped; a node not seen for 20 frames fades out.
        e.set(node, BSFadeNode::eMultType, 2);
        e.set(node, BSFadeNode::fFarDistSqr, bits(3000.0));
        e.set(node, BSFadeNode::iFrameCounter, 10);
        e.set(node, BSFadeNode::m_uFlags, 0x4000);
        assert!(e.call(0x00b4_eb10, &args![node, camera, 0u32]).bool());
        assert_eq!(e.get(node, BSFadeNode::fFarDistSqr), bits(2600.0));
        assert_eq!(e.get(node, BSFadeNode::fNearDistSqr), bits(1300.0));
        // Fading off: faded in at once.
        e.set_global(FADE_ENABLED, 0u8);
        e.set(node, BSFadeNode::m_uFlags, 0);
        assert!(e.call(0x00b4_eb10, &args![node, camera, 0u32]).bool());
        assert_eq!(e.get(node, BSFadeNode::m_uFlags), 0x4000);
    }
}

#[cfg(test)]
mod n011_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn register_streamables_answers_the_node_result_as_a_bool() {
        for (result, expected) in [(0x0000_0100u32, false), (0x0000_0001, true)] {
            let mut e = Engine::new();
            e.register_double(NODE_REGISTER_STREAMABLES, move |_, _| word(result));
            e.call_log = Some(vec![]);
            let answer = e.call(0x00b4_def0, &args![0x0e00_1000u32, 0x0e00_2000u32]);
            assert_eq!(answer.u32(), u32::from(expected));
            assert_eq!(
                calls(&e, NODE_REGISTER_STREAMABLES),
                vec![vec![0x0e00_1000, 0x0e00_2000]]
            );
        }
    }

    #[test]
    fn save_binary_and_is_equal_jump_to_the_node() {
        let mut e = Engine::new();
        e.register(NODE_SAVE_BINARY, |_, _| Ret::default());
        e.register(NODE_IS_EQUAL, |_, _| word(0x1234_5601));
        e.call_log = Some(vec![]);
        e.call(0x00b4_df10, &args![0x0e00_1000u32, 0x0e00_2000u32]);
        assert_eq!(
            calls(&e, NODE_SAVE_BINARY),
            vec![vec![0x0e00_1000, 0x0e00_2000]]
        );
        let answer = e.call(0x00b4_df20, &args![0x0e00_1000u32, 0x0e00_3000u32]);
        assert_eq!(answer.u32(), 0x1234_5601);
        assert_eq!(
            calls(&e, NODE_IS_EQUAL),
            vec![vec![0x0e00_1000, 0x0e00_3000]]
        );
    }

    type Log = Rc<RefCell<Vec<u32>>>;

    /// An engine whose viewer-string array (capacity `capacity`, size 0, step 4) grows by
    /// setting its capacity, and whose set-at records the stored word and bumps the size.
    fn viewer_engine(capacity: u16) -> (Engine, u32, Log) {
        let mut e = Engine::new();
        for page in [0x0102_3000, 0x011f_9000, 0x0120_2000] {
            e.map(page, 0x1000);
        }
        e.set_global(DISABLED_DISTANCE, f64::from(f32::MAX));
        e.set_global(FADE_NODE_RTTI, 0x010a_9000u32);
        e.set_global(BSX_FLAGS_NAME, 0x0e0b_0000u32);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, capacity);
        e.mem.set_u16(strings + 0xe, 4);
        e.register(NODE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(STRINGS_GROW, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        let stored: Log = Rc::new(RefCell::new(vec![]));
        let seen = stored.clone();
        e.register_double(STRINGS_SET_AT, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[2]));
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        e.register(VIEWER_STRING_NAME, |_, _| word(0x0f00_0001));
        e.register(VIEWER_STRING_FLOAT, |_, a| {
            word(0x0f10_0000 | (a[0] & 0xfff))
        });
        e.register(VIEWER_STRING_INT, |_, _| word(0x0f20_0000));
        e.register(VIEWER_STRING_BOOL, |_, a| {
            word(0x0f30_0000 | (a[0] & 0xfff))
        });
        let next = Rc::new(RefCell::new(0x0e40_0000u32));
        e.register_double(MEMORY_ALLOCATE, move |_, _| {
            let block = *next.borrow();
            *next.borrow_mut() += 0x100;
            word(block)
        });
        e.register(PRINT_00406D00, |_, _| Ret::default());
        (e, strings, stored)
    }

    #[test]
    fn get_viewer_strings_lists_the_fields_and_flags() {
        let (mut e, strings, stored) = viewer_engine(2);
        e.register(GET_EXTRA_DATA, |_, _| word(0));
        let node: Ptr<BSFadeNode> = e.new_object();
        e.set(node, BSFadeNode::fNearDistSqr, f32::MAX.to_bits());
        e.set(node, BSFadeNode::fFarDistSqr, 1234.5f32.to_bits());
        e.set(node, BSFadeNode::eMultType, 3);
        // A signalling NaN is pushed quieted.
        e.set(node, BSFadeNode::fLastFade, 0x7f80_0001);
        e.set(node, BSFadeNode::fCurrentFade, 0.5f32.to_bits());
        e.set(node, BSFadeNode::fTimeSinceUpdate, 2.0f32.to_bits());
        e.set(node, BSFadeNode::iFrameCounter, 77);
        // Bits 14 (faded in), 16 (LOD fading out) and 24 (high detail).
        e.set(node, BSFadeNode::m_uFlags, 0x0101_4000);
        e.call_log = Some(vec![]);
        e.call(0x00b4_ed70, &args![node, strings]);

        assert_eq!(
            calls(&e, NODE_GET_VIEWER_STRINGS),
            vec![vec![node.addr(), strings]]
        );
        assert_eq!(calls(&e, VIEWER_STRING_NAME), vec![vec![0x010a_9000]]);
        let far = f64::from(1234.5f32).to_bits();
        assert_eq!(
            calls(&e, PRINT_00406D00),
            vec![
                vec![0x0e40_0000, 0x40, NEAR_DISABLED_TEXT],
                vec![
                    0x0e40_0100,
                    0x40,
                    FAR_FORMAT,
                    far as u32,
                    (far >> 32) as u32
                ],
                vec![0x0e40_0200, 0x40, MULT_TYPE_FORMAT, 3],
            ]
        );
        assert_eq!(
            calls(&e, VIEWER_STRING_FLOAT),
            vec![
                vec![LAST_FADE_NAME, 0x7fc0_0001],
                vec![CURRENT_FADE_NAME, 0.5f32.to_bits()],
                vec![TIME_SINCE_UPDATE_NAME, 2.0f32.to_bits()],
            ]
        );
        assert_eq!(
            calls(&e, VIEWER_STRING_INT),
            vec![vec![FRAME_COUNTER_NAME, 77]]
        );
        // Each bool word keeps the upper bytes of the previous string pointer.
        assert_eq!(
            calls(&e, VIEWER_STRING_BOOL),
            vec![
                vec![FADED_IN_NAME, 0x0f20_0001],
                vec![
                    IGNORE_FADE_NAME,
                    (0x0f30_0000 | (FADED_IN_NAME & 0xfff)) & !0xff
                ],
                vec![
                    LOD_FADING_OUT_NAME,
                    ((0x0f30_0000 | (IGNORE_FADE_NAME & 0xfff)) & !0xff) | 1
                ],
                vec![
                    HAS_MOVING_SOUND_NAME,
                    (0x0f30_0000 | (LOD_FADING_OUT_NAME & 0xfff)) & !0xff
                ],
                vec![
                    HIGH_DETAIL_NAME,
                    ((0x0f30_0000 | (HAS_MOVING_SOUND_NAME & 0xfff)) & !0xff) | 1
                ],
            ]
        );
        assert_eq!(
            *stored.borrow(),
            vec![
                0x0f00_0001,
                0x0e40_0000,
                0x0e40_0100,
                0x0e40_0200,
                0x0f10_0000 | (LAST_FADE_NAME & 0xfff),
                0x0f10_0000 | (CURRENT_FADE_NAME & 0xfff),
                0x0f10_0000 | (TIME_SINCE_UPDATE_NAME & 0xfff),
                0x0f20_0000,
                0x0f30_0000 | (FADED_IN_NAME & 0xfff),
                0x0f30_0000 | (IGNORE_FADE_NAME & 0xfff),
                0x0f30_0000 | (LOD_FADING_OUT_NAME & 0xfff),
                0x0f30_0000 | (HAS_MOVING_SOUND_NAME & 0xfff),
                0x0f30_0000 | (HIGH_DETAIL_NAME & 0xfff),
            ]
        );
        // Full at 2: grown by step + size.
        assert_eq!(calls(&e, STRINGS_GROW)[0], vec![strings, 6]);
        assert_eq!(
            calls(&e, GET_EXTRA_DATA),
            vec![vec![node.addr(), 0x0e0b_0000]]
        );
    }

    #[test]
    fn get_viewer_strings_prints_both_distances_and_asks_the_extra_data() {
        let (mut e, strings, _) = viewer_engine(32);
        let extra = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x80);
        e.mem.set_u32(extra, vtable);
        e.mem.set_u32(vtable + 0x60, 0x00ee_0060);
        e.register(0x00ee_0060, |_, _| Ret::default());
        e.register_double(GET_EXTRA_DATA, move |_, _| word(extra));
        let node: Ptr<BSFadeNode> = e.new_object();
        e.set(node, BSFadeNode::fNearDistSqr, 10.0f32.to_bits());
        // NaN is unordered: printed, not disabled.
        e.set(node, BSFadeNode::fFarDistSqr, 0xffc0_0000);
        e.call_log = Some(vec![]);
        e.call(0x00b4_ed70, &args![node, strings]);
        let prints = calls(&e, PRINT_00406D00);
        let near = 10.0f64.to_bits();
        assert_eq!(
            prints[0],
            vec![
                0x0e40_0000,
                0x40,
                NEAR_FORMAT,
                near as u32,
                (near >> 32) as u32
            ]
        );
        assert_eq!(&prints[1][..3], &[0x0e40_0100, 0x40, FAR_FORMAT]);
        assert!(calls(&e, STRINGS_GROW).is_empty());
        assert_eq!(calls(&e, 0x00ee_0060), vec![vec![extra, strings]]);
    }
}
