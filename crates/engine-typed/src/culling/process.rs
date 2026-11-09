//! The culling processes: the frustum test of one object (`NiCullingProcess::DoCulling`), the
//! Bethesda process's choice of test (`BSCullingProcess::Process`) and its walk of a scene from a
//! camera, the hand-off of a visible geometry to the accumulator, the fade-node culler, and the
//! multi-bound box test.

use super::layout::{
    BSCompoundFrustum, BSCullingProcess, BSMultiBoundAABB, NiAVObject, NiBound, NiCullingProcess,
    NiFrustum, NiFrustumPlanes, NiGeometry, NiPlane, NiPoint3, NiTransform,
};
use super::{bound, fld, fld_local, fstp, m32, radius_not_zero, FAMILY};
use crate::dispatch;
use engine_core::ptr::Ptr;
use engine_core::typed::leaf;
use engine_core::x87::{x87_add, x87_mul, x87_sub};
use engine_core::{args, typed_call, Engine};

/// `NiAVObject::OnVisible` (Xbox PDB), virtual slot 0xd4: `thiscall(object, process)`.
pub(crate) const SLOT_ON_VISIBLE: u32 = 0xd4;
/// The accumulator's `RegisterObject`, virtual slot 0x98: `thiscall(accumulator, geometry)`, a bool.
const SLOT_REGISTER_OBJECT: u32 = 0x98;
/// The accumulator's camera setter, virtual slot 0x8c: `thiscall(accumulator, camera)`.
const SLOT_SET_CAMERA: u32 = 0x8c;
/// `NiRefObject`'s deleting destructor, virtual slot 4: `thiscall(object)`.
const SLOT_DELETE: u32 = 0x4;
/// The fade-node culler's question to a reference, virtual slot 0x10: its fade node, or null.
const SLOT_FADE_NODE: u32 = 0x10;

/// `BSCullingProcess::kCullMode` values (Xbox PDB `BSCPCullingType`): every object visible, none,
/// the planes only (no compound frustum).
const MODE_ALL_VISIBLE: u32 = 1;
const MODE_NONE_VISIBLE: u32 = 2;
const MODE_PLANES_ONLY: u32 = 3;

/// `m_uFlags` bits the culling reads: the fade-node culler's verdict is in (`0x1000`) and says
/// culled (`0x100000`); the object is always visible (`0x800`).
const FLAG_FADE_TESTED: u32 = 0x1000;
const FLAG_FADE_CULLED: u32 = 0x0010_0000;
const FLAG_ALWAYS_VISIBLE: u32 = 0x800;

/// `BSCompoundFrustum`'s active-plane state: get (`thiscall`, a word), test an object
/// (`thiscall(frustum, object)`, a bool), set back (`thiscall(frustum, state)`).
const COMPOUND_GET_STATE: u32 = 0x00c4_9050;
const COMPOUND_PROCESS: u32 = 0x00c4_91a0;
const COMPOUND_SET_STATE: u32 = 0x00c4_9100;
/// `NiPointer` copy (`thiscall(holder, from holder)`) and assignment (`thiscall(holder, object)`).
const POINTER_COPY: u32 = 0x00c4_6270;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The holder of the default accumulator, at +8 (`011f4748` holds its address).
const DEFAULT_ACCUMULATOR_HOLDER: u32 = 0x011f_4748;
/// `InterlockedDecrement`, its import slot.
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The debug print (empty in this build) and what `AppendVirtual` prints with it.
const DEBUG_PRINT: u32 = 0x00a2_9680;
const APPEND_FAILED_FORMAT: u32 = 0x0109_c2e0;
const EMPTY_TEXT: u32 = 0x0101_f7a8;
/// The `double` zero the box test compares its third half extent with.
const ZERO_DOUBLE: u32 = 0x0101_2060;

/// Plane `index` of a set of frustum planes.
#[inline(always)]
pub(crate) fn plane(planes: Ptr<NiFrustumPlanes>, index: u32) -> Ptr<NiPlane> {
    planes.cast::<NiPlane>().index(index, 0x10)
}

/// The object's world bound, or the empty one (`0043d450`).
#[inline(always)]
pub(crate) fn world_bound(e: &mut Engine, object: Ptr<NiAVObject>) -> Ptr<NiBound> {
    Ptr::new(typed_call!(
        leaf::FAMILY,
        0x0043_d450,
        leaf::world_bound(e, object)
    ))
}

// Typed from 00a694e0
/// `NiCullingProcess::DoCulling` (Xbox PDB, `thiscall(process, object)`): only for an object with a
/// bound whose radius isn't zero (a NaN radius goes on). For each of the six planes still active
/// (bit `i` of `m_uiActivePlanes`, read at each step), which side of it the world bound is
/// (`004b6100`, the world bound fetched again each time): side 2 culls the object and stops, side 1
/// drops the plane for the walk below. After all six, the object's `OnVisible` runs with the process.
/// The active planes are put back as they were.
pub fn do_culling(e: &mut Engine, this: Ptr<NiCullingProcess>, object: Ptr<NiAVObject>) {
    let bound = e.get(object, NiAVObject::m_kWorldBound);
    if bound.is_null() || !radius_not_zero(e, bound) {
        return;
    }
    let planes = this.at(NiCullingProcess::m_kPlanes);
    let saved = e.get(planes, NiFrustumPlanes::m_uiActivePlanes);
    let mut index = 0;
    let mut mask = 1u32;
    while index < 6 {
        if e.get(planes, NiFrustumPlanes::m_uiActivePlanes) & mask != 0 {
            let world = world_bound(e, object);
            let side = typed_call!(
                FAMILY,
                0x004b_6100,
                bound::which_side(e, world, plane(planes, index))
            );
            if side == 2 {
                break;
            }
            if side == 1 {
                let active = e.get(planes, NiFrustumPlanes::m_uiActivePlanes) & !mask;
                e.set(planes, NiFrustumPlanes::m_uiActivePlanes, active);
            }
        }
        index += 1;
        mask = mask.rotate_left(1);
    }
    if index == 6 {
        object_on_visible(e, object, this.addr());
    }
    e.set(planes, NiFrustumPlanes::m_uiActivePlanes, saved);
}

/// The object's `OnVisible` with the process (what makes it visible: a geometry appends itself, a
/// node walks its children).
#[inline(always)]
fn on_visible(e: &mut Engine, this: Ptr<BSCullingProcess>, object: Ptr<NiAVObject>) {
    object_on_visible(e, object, this.addr());
}

/// `object->OnVisible(process)` (slot 0xd4): directly when the slot holds the typed `OnVisible` of
/// `NiNode` (`00a5dbe0`), `NiGeometry` (`00a7fd90`, every geometry class) or `BSMultiBoundNode`
/// (`00c46f60`), else through the vtable.
#[inline(always)]
pub(crate) fn object_on_visible(e: &mut Engine, object: Ptr<NiAVObject>, process: u32) {
    match crate::slot_target(e, object.addr(), SLOT_ON_VISIBLE) {
        0x00a5_dbe0 => typed_call!(
            FAMILY,
            0x00a5_dbe0,
            super::visible::node_on_visible(e, object.cast(), Ptr::new(process))
        ),
        0x00a7_fd90 => typed_call!(
            FAMILY,
            0x00a7_fd90,
            super::visible::geometry_on_visible(e, object.cast(), Ptr::new(process))
        ),
        0x00c4_6f60 => typed_call!(
            FAMILY,
            0x00c4_6f60,
            super::visible::multibound_node_on_visible(e, object.cast(), Ptr::new(process))
        ),
        _ => {
            e.vcall(object.addr(), SLOT_ON_VISIBLE, &args![process]);
        }
    }
}

/// Whether the compound frustum is there and holds operations.
#[inline(always)]
fn compound_in_use(e: &Engine, this: Ptr<BSCullingProcess>) -> bool {
    let frustum = e.get(this, BSCullingProcess::pCompoundFrustum);
    !frustum.is_null() && e.get(frustum, BSCompoundFrustum::iFreeOp) != 0
}

/// The active planes of the process's frustum, as a word.
#[inline(always)]
fn active_planes(e: &Engine, this: Ptr<BSCullingProcess>) -> u32 {
    let planes = this
        .at(BSCullingProcess::base)
        .at(NiCullingProcess::m_kPlanes);
    e.get(planes, NiFrustumPlanes::m_uiActivePlanes)
}

// Typed from 00c4ee90
/// `BSCullingProcess::Process` (Xbox PDB, `thiscall(process, object)`). Modes 1 and 2: the object is
/// visible in mode 1, not in mode 2. Otherwise a fade-node verdict, when there is one, decides
/// (visible unless culled). Without one: with no compound frustum in use, or in mode 3, an object
/// marked always visible, or any object while no plane is active, is visible, else `DoCulling`
/// decides. With the compound frustum: an always-visible object is visible, and so is the rest only
/// as the compound frustum says (its active-plane state kept around the test); with no frustum in
/// use any more and no plane active the object is visible too.
pub fn bs_process(e: &mut Engine, this: Ptr<BSCullingProcess>, object: Ptr<NiAVObject>) {
    let mode = e.get(this, BSCullingProcess::kCullMode);
    if mode == MODE_NONE_VISIBLE || mode == MODE_ALL_VISIBLE {
        if e.get(this, BSCullingProcess::kCullMode) == MODE_ALL_VISIBLE {
            on_visible(e, this, object);
        }
        return;
    }
    if e.get(object, NiAVObject::m_uFlags) & FLAG_FADE_TESTED != 0 {
        if e.get(object, NiAVObject::m_uFlags) & FLAG_FADE_CULLED == 0 {
            on_visible(e, this, object);
        }
        return;
    }
    if !compound_in_use(e, this) || e.get(this, BSCullingProcess::kCullMode) == MODE_PLANES_ONLY {
        if e.get(object, NiAVObject::m_uFlags) & FLAG_ALWAYS_VISIBLE == 0
            && active_planes(e, this) != 0
        {
            typed_call!(
                FAMILY,
                0x00a6_94e0,
                do_culling(e, this.cast::<NiCullingProcess>(), object)
            );
        } else {
            on_visible(e, this, object);
        }
        return;
    }
    if e.get(object, NiAVObject::m_uFlags) & FLAG_ALWAYS_VISIBLE != 0
        || (!compound_in_use(e, this) && active_planes(e, this) == 0)
    {
        on_visible(e, this, object);
        return;
    }
    let frustum = e.get(this, BSCullingProcess::pCompoundFrustum);
    let state = e.call(COMPOUND_GET_STATE, &args![frustum]).u32();
    let frustum = e.get(this, BSCullingProcess::pCompoundFrustum);
    if e.call(COMPOUND_PROCESS, &args![frustum, object]).bool() {
        on_visible(e, this, object);
    }
    let frustum = e.get(this, BSCullingProcess::pCompoundFrustum);
    e.call(COMPOUND_SET_STATE, &args![frustum, state]);
}

// Typed from 00c4f1d0
/// `BSCullingProcess::AppendVirtual` (Xbox PDB, `thiscall(process, geometry)`): registers the
/// geometry with the accumulator (slot 0x98, a bool). Without an accumulator, or when it refuses,
/// the debug print (empty here) gets the failure text and the geometry's name (the empty text for
/// none).
pub fn append_virtual(e: &mut Engine, this: Ptr<BSCullingProcess>, geometry: Ptr<NiGeometry>) {
    let mut registered = false;
    if e.get(this, BSCullingProcess::spAccumulator) != 0 {
        let accumulator = e.get(this, BSCullingProcess::spAccumulator);
        // `BSShaderAccumulator::RegisterObject` (`00b63f10`) directly when the slot holds it.
        let accepted = if crate::slot_target(e, accumulator, SLOT_REGISTER_OBJECT) == 0x00b6_3f10 {
            typed_call!(
                crate::shader::FAMILY,
                0x00b6_3f10,
                crate::shader::accumulator::register_object(
                    e,
                    Ptr::new(accumulator),
                    geometry.cast()
                )
            )
        } else {
            e.vcall(accumulator, SLOT_REGISTER_OBJECT, &args![geometry])
                .bool()
        };
        if accepted {
            registered = true;
        }
    }
    if !registered {
        let object = geometry.cast::<NiAVObject>();
        let name = if e.get(object, NiAVObject::m_kName) != 0 {
            e.get(object, NiAVObject::m_kName)
        } else {
            EMPTY_TEXT
        };
        e.call(DEBUG_PRINT, &args![APPEND_FAILED_FORMAT, name]);
    }
}

// Typed from 00a694a0
/// `NiCullingProcess::SetFrustum` (Xbox PDB, `thiscall(process, frustum)`): with a camera, copies the
/// frustum (seven words) in and sets the six planes from it and the camera's world transform
/// (`00a74e10`); then all six are active.
pub fn set_frustum(e: &mut Engine, this: Ptr<NiCullingProcess>, frustum: Ptr<NiFrustum>) {
    let camera = e.get(this, NiCullingProcess::m_pkCamera);
    if camera == 0 {
        return;
    }
    let own = this.at(NiCullingProcess::m_kFrustum);
    for word in 0..7 {
        let value = e.mem.u32(frustum.addr().wrapping_add(4 * word));
        e.mem.set_u32(own.addr().wrapping_add(4 * word), value);
    }
    let planes = this.at(NiCullingProcess::m_kPlanes);
    let world = Ptr::<NiTransform>::new(camera.wrapping_add(0x68));
    typed_call!(
        FAMILY,
        0x00a7_4e10,
        super::planes::frustum_planes_set(e, planes, own, world)
    );
    e.set(planes, NiFrustumPlanes::m_uiActivePlanes, 0x3f);
}

// Typed from 00c4f070
/// `BSCullingProcess::Process` (Xbox PDB, the scene form, `thiscall(process, camera, scene, visible
/// set)`): with a camera and a scene, keeps the camera, sets the frustum from the camera's (+0xdc),
/// and puts the visible set in for the walk when one is given. Without a visible set the accumulator
/// (copied into an `NiPointer` local; the default accumulator when the process has none) gets the
/// camera. Then the scene is culled (`NiAVObject::Cull`), the visible set put back and the local
/// released (the last reference deletes the accumulator).
pub fn bs_process_scene(
    e: &mut Engine,
    this: Ptr<BSCullingProcess>,
    camera: u32,
    scene: Ptr<NiAVObject>,
    visible_set: u32,
) {
    if camera == 0 || scene.is_null() {
        return;
    }
    let base = this.at(BSCullingProcess::base);
    e.set(base, NiCullingProcess::m_pkCamera, camera);
    let camera_frustum =
        Ptr::<NiFrustum>::new(e.get(base, NiCullingProcess::m_pkCamera).wrapping_add(0xdc));
    typed_call!(FAMILY, 0x00a6_94a0, set_frustum(e, base, camera_frustum));
    let mut saved = 0;
    if visible_set != 0 {
        saved = e.get(base, NiCullingProcess::m_pkVisibleSet);
        e.set(base, NiCullingProcess::m_pkVisibleSet, visible_set);
    }
    // The `NiPointer` local: the copy and assignment take its address.
    e.with_stack(4, |e, holder| {
        if e.get(base, NiCullingProcess::m_pkVisibleSet) == 0 {
            let own = this
                .addr()
                .wrapping_add(BSCullingProcess::spAccumulator.off);
            e.call(POINTER_COPY, &args![holder, own]);
            if e.mem.u32(holder.addr()) == 0 {
                let default_holder: u32 = e.global(DEFAULT_ACCUMULATOR_HOLDER);
                let fallback = e.mem.u32(default_holder.wrapping_add(8));
                e.call(POINTER_ASSIGN, &args![holder, fallback]);
            }
            let accumulator = e.mem.u32(holder.addr());
            if accumulator != 0 {
                e.vcall(accumulator, SLOT_SET_CAMERA, &args![camera]);
            }
        }
        typed_call!(
            FAMILY,
            0x00a5_9e00,
            super::visible::cull(e, scene, this.cast::<NiCullingProcess>())
        );
        if visible_set != 0 {
            e.set(base, NiCullingProcess::m_pkVisibleSet, saved);
        }
        let accumulator = e.mem.u32(holder.addr());
        if accumulator != 0 {
            let count = e
                .call(INTERLOCKED_DECREMENT, &args![accumulator.wrapping_add(4)])
                .u32();
            if count == 0 {
                e.vcall(accumulator, SLOT_DELETE, &[]);
            }
        }
    });
}

// Typed from 00bc2b30
/// `BSFadeNodeCuller::Process` (Xbox PDB, `thiscall(culler, reference)`): for the reference's fade
/// node (slot 0x10; nothing without one) marks the verdict as in (`0x1000`), then tests its world
/// bound against all six planes: the dot product of normal and centre (`(ny*cy + nx*cx) + cz*nz`,
/// stored), minus the constant (stored); behind a plane when that is at most minus the radius
/// (ordered). Behind any plane: culled (`0x100000` set), else cleared. The flags word is read once,
/// after the mark, and written back whole.
pub fn fade_culler_process(e: &mut Engine, this: Ptr<NiCullingProcess>, reference: u32) {
    let node = Ptr::<NiAVObject>::new(e.vcall(reference, SLOT_FADE_NODE, &[]).u32());
    if node.is_null() {
        return;
    }
    let marked = e.get(node, NiAVObject::m_uFlags) | FLAG_FADE_TESTED;
    e.set(node, NiAVObject::m_uFlags, marked);
    let flags = e.get(node, NiAVObject::m_uFlags);
    let planes = this.at(NiCullingProcess::m_kPlanes);
    let mut index = 0;
    while index < 6 {
        let world = world_bound(e, node);
        let center = world.at(NiBound::m_kCenter);
        let normal = plane(planes, index).at(NiPlane::m_kNormal);
        let y = x87_mul(fld(e, normal, NiPoint3::y), m32(e, center, NiPoint3::y));
        let x = x87_mul(fld(e, normal, NiPoint3::x), m32(e, center, NiPoint3::x));
        let z = x87_mul(fld(e, center, NiPoint3::z), m32(e, normal, NiPoint3::z));
        let dot = fstp(x87_add(x87_add(y, x), z));
        let distance = fstp(x87_sub(
            fld_local(dot),
            m32(e, plane(planes, index), NiPlane::m_fConstant),
        ));
        // FLD distance; FLD radius; FCHS; FCOMPP; TEST AH,1; JZ: -radius not below the distance.
        let below = -fld(e, world, NiBound::m_fRadius);
        if below >= fld_local(distance) {
            break;
        }
        index += 1;
    }
    let flags = if index == 6 {
        flags & !FLAG_FADE_CULLED
    } else {
        flags | FLAG_FADE_CULLED
    };
    e.set(node, NiAVObject::m_uFlags, flags);
}

// Typed from 00c387f0
/// `BSMultiBoundAABB::WithinFrustum` (Xbox PDB, `thiscall(box, frustum planes)`, a bool): the box's
/// eight corners (`00c39270`, into a local) against each active plane (`0049da80`); outside (false)
/// once every corner tested is behind one plane. A box whose third half extent is zero (compared
/// with the `double` zero; a NaN isn't zero) tests its odd corners only.
pub fn within_frustum(
    e: &mut Engine,
    this: Ptr<BSMultiBoundAABB>,
    planes: Ptr<NiFrustumPlanes>,
) -> bool {
    let zero: f64 = e.global(ZERO_DOUBLE);
    let half_extents = this.at(BSMultiBoundAABB::HalfExtents);
    let flat = fld(e, half_extents, NiPoint3::z) == zero;
    // Eight points of 12 bytes, a local `00c39270` writes.
    e.with_stack(96, |e, corners| {
        typed_call!(
            dispatch::FAMILY,
            0x00c3_9270,
            dispatch::math::box_corners(e, this.cast(), corners)
        );
        for index in 0..6u32 {
            if (1u32 << index) & e.get(planes, NiFrustumPlanes::m_uiActivePlanes) == 0 {
                continue;
            }
            let mut all_behind = true;
            for corner in 0..8u32 {
                if flat && corner & 1 == 0 {
                    continue;
                }
                let side = typed_call!(
                    leaf::FAMILY,
                    0x0049_da80,
                    leaf::plane_side(e, plane(planes, index), corners.byte_add(12 * corner))
                );
                if side != 2 {
                    all_behind = false;
                }
            }
            if all_behind {
                return false;
            }
        }
        true
    })
}
