//! The compound frustum's part in the walk: its active-plane state, kept around a test, and the test
//! of one object against the view frustum and its program of portal and occluder plane sets.

use super::layout::{BSCompoundFrustum, FunctionOp, NiAVObject, NiBound, NiFrustumPlanes};
use super::layout::{NiFrustum, NiPoint3, NiTransform};
use super::{copy_float, fld, fld_local, fstp, FAMILY, NULL_BOUND};
use engine_core::ptr::Ptr;
use engine_core::{args, typed_call, Engine};

/// The thread's scrap heap (`thiscall(heap manager)`, the heap), its allocation (`thiscall(heap,
/// bytes, alignment)`) and its release (`thiscall(heap, block)`).
const HEAP_MANAGER: u32 = 0x011f_6238;
const SCRAP_HEAP: u32 = 0x00aa_42e0;
const SCRAP_ALLOCATE: u32 = 0x00aa_54a0;
const SCRAP_FREE: u32 = 0x00aa_5610;
/// The alignment word the state block is allocated with.
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
/// A bound against a plane set: inside it (`thiscall(frustum, bound, planes)`, a bool), and
/// outside an occluder (the same form).
const BOUND_IN_PLANES: u32 = 0x00c4_a6e0;
const BOUND_PAST_OCCLUDER: u32 = 0x00c4_a790;
/// The `double` zero the radius is compared with.
const ZERO_DOUBLE: u32 = 0x0101_2060;

/// `FunctionOp::op` values: the program ends with a pass or a fail; test a portal's planes; test an
/// occluder's planes. The plane set is the next operation's `op` word.
const OP_PASS: u32 = 2;
const OP_FAIL: u32 = 3;
const OP_PORTAL: u32 = 7;
const OP_OCCLUDER: u32 = 8;

/// Plane set `index` of the frustum's array.
#[inline(always)]
fn plane_set(e: &Engine, this: Ptr<BSCompoundFrustum>, index: u32) -> Ptr<NiFrustumPlanes> {
    e.get(this, BSCompoundFrustum::kPlanes_pBuffer)
        .index(index, 0x64)
}

/// Whether `index` is still within the sets the state covers: up to the last set in use, and below
/// the array's reserved size (both read at each step, compared signed).
#[inline(always)]
fn within_state(e: &Engine, this: Ptr<BSCompoundFrustum>, index: i32) -> bool {
    let reserved = e.get(this, BSCompoundFrustum::kPlanes_iReservedSize);
    index < e.get(this, BSCompoundFrustum::iFreePlane).wrapping_add(1)
        && index < i32::from_ne_bytes(reserved.to_ne_bytes())
}

/// Word `index` of a state block.
#[inline(always)]
fn state_word(state: u32, index: i32) -> u32 {
    state.wrapping_add(u32::from_ne_bytes(index.to_ne_bytes()).wrapping_mul(4))
}

/// `index` as the unsigned word the plane-set address arithmetic uses.
#[inline(always)]
fn word(index: i32) -> u32 {
    u32::from_ne_bytes(index.to_ne_bytes())
}

// Typed from 00c49050
/// `BSCompoundFrustum::GetActivePlaneState` (Xbox PDB, `thiscall(frustum)`): a block from the
/// thread's scrap heap, one word for each set the array has room for: the view frustum's active
/// planes, then each set's in use. The rest of the block is as the heap gave it.
pub fn active_state(e: &mut Engine, this: Ptr<BSCompoundFrustum>) -> u32 {
    let heap = e.call(SCRAP_HEAP, &args![HEAP_MANAGER]).u32();
    let reserved = e.get(this, BSCompoundFrustum::kPlanes_iReservedSize);
    let alignment: u32 = e.global(SCRAP_ALIGNMENT);
    let state = e
        .call(SCRAP_ALLOCATE, &args![heap, reserved << 2, alignment])
        .u32();
    let view = this.at(BSCompoundFrustum::kViewFrustum);
    let mask = e.get(view, NiFrustumPlanes::m_uiActivePlanes);
    e.mem.set_u32(state, mask);
    let mut index = 1i32;
    while within_state(e, this, index) {
        let set = plane_set(e, this, word(index - 1));
        let mask = e.get(set, NiFrustumPlanes::m_uiActivePlanes);
        e.mem.set_u32(state_word(state, index), mask);
        index += 1;
    }
    state
}

// Typed from 00c49100
/// `BSCompoundFrustum::SetActivePlaneState` (Xbox PDB, `thiscall(frustum, state)`): puts a block of
/// [`active_state`] back, and gives the block back to the thread's scrap heap.
pub fn set_active_state(e: &mut Engine, this: Ptr<BSCompoundFrustum>, state: u32) {
    let view = this.at(BSCompoundFrustum::kViewFrustum);
    let mask = e.mem.u32(state);
    e.set(view, NiFrustumPlanes::m_uiActivePlanes, mask);
    let mut index = 1i32;
    while within_state(e, this, index) {
        let mask = e.mem.u32(state_word(state, index));
        let set = plane_set(e, this, word(index - 1));
        e.set(set, NiFrustumPlanes::m_uiActivePlanes, mask);
        index += 1;
    }
    let heap = e.call(SCRAP_HEAP, &args![HEAP_MANAGER]).u32();
    e.call(SCRAP_FREE, &args![heap, state]);
}

// Typed from 00c491a0
/// `BSCompoundFrustum::Process` (Xbox PDB, `thiscall(frustum, object)`, a bool): false without a
/// world bound or with a radius of zero (stored to a float, compared with the `double` zero; a NaN
/// radius goes on). A copy of the bound is tested against the view frustum (unless skipped), then,
/// with operations in use, the program runs from its first: a pass or a fail ends it, a portal or an
/// occluder tests the copy against the next operation's plane set and goes on at `jp` when it held
/// and at `jf` when not (any other operation goes on at `jf`).
pub fn process(e: &mut Engine, this: Ptr<BSCompoundFrustum>, object: Ptr<NiAVObject>) -> bool {
    let bound = e.get(object, NiAVObject::m_kWorldBound);
    if bound.is_null() {
        return false;
    }
    let radius = fld_local(fstp(fld(e, bound, NiBound::m_fRadius)));
    // TEST AH,0x44; JNP: an ordered equal is no bound.
    if radius == e.global::<f64>(ZERO_DOUBLE) {
        return false;
    }
    let bound = e.get(object, NiAVObject::m_kWorldBound);
    let source = if bound.is_null() { NULL_BOUND } else { bound };
    // The copy: the plane tests take its address.
    e.with_stack(16, |e, copy| {
        let copy = copy.cast::<NiBound>();
        let (from, to) = (source.at(NiBound::m_kCenter), copy.at(NiBound::m_kCenter));
        copy_float(e, from, NiPoint3::x, to, NiPoint3::x);
        copy_float(e, from, NiPoint3::y, to, NiPoint3::y);
        copy_float(e, from, NiPoint3::z, to, NiPoint3::z);
        copy_float(e, source, NiBound::m_fRadius, copy, NiBound::m_fRadius);
        let mut visible = true;
        if e.get(this, BSCompoundFrustum::bSkipViewFrustum) == 0 {
            let view = this.at(BSCompoundFrustum::kViewFrustum);
            visible = e.call(BOUND_IN_PLANES, &args![this, copy, view]).bool();
        }
        if !visible || e.get(this, BSCompoundFrustum::iFreeOp) <= 0 {
            return visible;
        }
        let mut at = e.get(this, BSCompoundFrustum::iFirstOp);
        loop {
            let op = |e: &Engine, index: i32| {
                e.get(this, BSCompoundFrustum::kFunctionOperators_pBuffer)
                    .index(word(index), 0xc)
            };
            let code = e.get(op(e, at), FunctionOp::op);
            if code == OP_PASS || code == OP_FAIL {
                return e.get(op(e, at), FunctionOp::op) == OP_PASS;
            }
            let mut held = false;
            if code == OP_PORTAL || code == OP_OCCLUDER {
                let set = e.get(op(e, at.wrapping_add(1)), FunctionOp::op);
                let planes = plane_set(e, this, set);
                let test = if code == OP_PORTAL {
                    BOUND_IN_PLANES
                } else {
                    BOUND_PAST_OCCLUDER
                };
                held = e.call(test, &args![this, copy, planes]).bool();
            }
            let next = if held {
                e.get(op(e, at), FunctionOp::jp)
            } else {
                e.get(op(e, at), FunctionOp::jf)
            };
            at = i32::from_ne_bytes(next.to_ne_bytes());
        }
    })
}

// Typed from 00c47980
/// `BSCompoundFrustum::SetCamera` (Xbox PDB, `thiscall(frustum, camera)`): with a camera, the view
/// frustum's planes from the camera's frustum (+0xdc) and world transform (+0x68) (`00a74e10`), and
/// the view position from the camera's (+0x8c, three words). Then no plane set or operation is in
/// use, and neither flag is set.
pub fn set_camera(e: &mut Engine, this: Ptr<BSCompoundFrustum>, camera: u32) {
    if camera != 0 {
        let view = this.at(BSCompoundFrustum::kViewFrustum);
        let frustum = Ptr::<NiFrustum>::new(camera.wrapping_add(0xdc));
        let world = Ptr::<NiTransform>::new(camera.wrapping_add(0x68));
        typed_call!(
            FAMILY,
            0x00a7_4e10,
            super::planes::frustum_planes_set(e, view, frustum, world)
        );
        let position = this.at(BSCompoundFrustum::kViewPosition);
        let from = Ptr::<NiPoint3>::new(camera.wrapping_add(0x8c));
        copy_float(e, from, NiPoint3::x, position, NiPoint3::x);
        copy_float(e, from, NiPoint3::y, position, NiPoint3::y);
        copy_float(e, from, NiPoint3::z, position, NiPoint3::z);
    }
    e.set(this, BSCompoundFrustum::iFreePlane, 0);
    e.set(this, BSCompoundFrustum::iFreeOp, 0);
    e.set(this, BSCompoundFrustum::bPrethreaded, 0);
    e.set(this, BSCompoundFrustum::iFirstOp, 0);
    e.set(this, BSCompoundFrustum::bSkipViewFrustum, 0);
}
