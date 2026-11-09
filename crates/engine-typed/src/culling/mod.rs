//! The culling family (T7 of the migration plan, stage 1): the scene graph's visibility walk. Each
//! frame a culling process walks the scene (`NiAVObject::Cull`, Xbox PDB); for each object it tests
//! the object's world bound against the camera's frustum planes (`NiCullingProcess::DoCulling`,
//! `NiBound::WhichSide`), and a visible object's `OnVisible` either walks its children (a node) or
//! hands itself to the process to be drawn (a geometry). The bounds themselves come from the update
//! pass (`UpdateWorldBound`, `NiBound::Update`, `NiBound::Merge`), which this family types as well.
//!
//! Stage 1: the objects stay in guest memory where the game made them and are read through their
//! [`layout`]. Each function stays registered at its exe address, so the vtables reach the typed
//! version unchanged; a call by address inside the family is a `typed_call!`.
//!
//! **Floats.** The per-thread census (measure/typed/t3, `threads.tsv`) has the culling walk on the
//! main thread and on game thread 12 (`00aa64d0`; about 1 call in 700), and the bound updates on
//! threads 14 to 16 as well. So the family's row is [`Threads::Both`], and its x87 code keeps the
//! exact helpers ([`engine_core::x87`]), which round as the running thread's control word says:
//! plain `f32` would match the main thread only, `f64` rounded at each store the other threads only.
//! Every load, operation and store below is the exe's, in its order, with its memory operands kept
//! as memory operands (a signalling NaN there is not quieted before the operation).

use engine_core::ptr::{Field, Ptr};
use engine_core::typed::{Family, FamilyId, Threads};
use engine_core::x87::{x87_load32, x87_m32, x87_store32};
use engine_core::{entry, AbiFn, Engine};
use layout::{
    BSCompoundFrustum, BSCullingProcess, BSMultiBoundAABB, BSMultiBoundNode, NiAVObject, NiBound,
    NiCullingProcess, NiFrustum, NiFrustumPlanes, NiGeometry, NiNode, NiPlane, NiPoint3,
    NiTransform,
};

pub mod bound;
pub mod compound;
pub mod layout;
pub mod planes;
pub mod process;
pub mod update;
pub mod visible;

/// This family's bit: what each `typed_call!` of it names.
pub const FAMILY: FamilyId = FamilyId::new(2);

/// `NV_TYPED=culling`. Past its speed gate (2026-11-08, together with the other five families of this crate: real
/// threads, the fixes preset, the same exe off and on, BAAB): Goodsprings 55.0 to 60.1 fps (+9.3%), the crowded
/// street at 2560x1440 8x AA 52.3 to 57.7 fps (+10.3%), the sky above it 117 to 134 fps (+14%). On unless
/// `NV_TYPED` says otherwise (`NV_TYPED=off` or `culling:off`).
pub static CULLING: Family = Family {
    name: "culling",
    id: FAMILY,
    past_gate: true,
    threads: Threads::Both,
    funcs,
};

/// `m_uFlags` bit 0: the application culled the object (`APP_CULLED_MASK`).
pub const FLAG_APP_CULLED: u32 = 0x1;

/// The empty bound an object without one reads (`NiAVObject`'s static `NullBoundS`, Xbox PDB).
pub const NULL_BOUND: Ptr<NiBound> = Ptr::new(0x011f_4288);
/// `NiPoint3::ZERO`.
pub const POINT_ZERO: u32 = 0x011f_426c;

/// `FLD dword` of a float field: the value the x87 stack holds (a signalling NaN quieted).
#[inline(always)]
pub(crate) fn fld<S>(e: &Engine, p: Ptr<S>, f: Field<S, f32>) -> f64 {
    x87_load32(e.get(p, f).to_bits())
}

/// A float field as the memory operand of an x87 arithmetic instruction (signalling kept).
#[inline(always)]
pub(crate) fn m32<S>(e: &Engine, p: Ptr<S>, f: Field<S, f32>) -> f64 {
    x87_m32(e.get(p, f).to_bits())
}

/// `FSTP dword`: the stack value rounded to a float.
#[inline(always)]
pub(crate) fn fstp(value: f64) -> f32 {
    f32::from_bits(x87_store32(value))
}

/// `FLD dword` of a float held in a local.
#[inline(always)]
pub(crate) fn fld_local(value: f32) -> f64 {
    x87_load32(value.to_bits())
}

/// A float local as the memory operand of an x87 arithmetic instruction.
#[inline(always)]
pub(crate) fn m32_local(value: f32) -> f64 {
    x87_m32(value.to_bits())
}

/// The float bits of a field moved by `MOV` (a copy that never touches the x87 stack).
#[inline(always)]
pub(crate) fn copy_float<S, D>(
    e: &mut Engine,
    from: Ptr<S>,
    from_field: Field<S, f32>,
    to: Ptr<D>,
    to_field: Field<D, f32>,
) {
    let v = e.get(from, from_field);
    e.set(to, to_field, v);
}

/// `FLDZ; FCOMP radius; FNSTSW; TEST AH,0x44; JP`: whether a radius is anything but zero (a NaN
/// radius is not zero).
#[inline(always)]
pub(crate) fn radius_not_zero(e: &Engine, bound: Ptr<NiBound>) -> bool {
    let radius = m32(e, bound, NiBound::m_fRadius);
    // Unordered sets C3 and C2 (even parity, taken); equal sets C3 alone (odd, not taken).
    radius.is_nan() || radius != 0.0
}

/// The family's functions, by exe address.
fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004b_6160,
            bound::plane_distance(Ptr<NiPlane>, Ptr<NiPoint3>) -> f32
        ),
        entry!(
            0x004b_6100,
            bound::which_side(Ptr<NiBound>, Ptr<NiPlane>) -> u32
        ),
        entry!(0x00a7_f3f0, bound::merge(Ptr<NiBound>, Ptr<NiBound>)),
        entry!(
            0x00a5_9e00,
            visible::cull(Ptr<NiAVObject>, Ptr<NiCullingProcess>)
        ),
        entry!(
            0x00a6_94e0,
            process::do_culling(Ptr<NiCullingProcess>, Ptr<NiAVObject>)
        ),
        entry!(
            0x00a5_dbe0,
            visible::node_on_visible(Ptr<NiNode>, Ptr<NiCullingProcess>)
        ),
        entry!(
            0x00c4_ee90,
            process::bs_process(Ptr<BSCullingProcess>, Ptr<NiAVObject>)
        ),
        entry!(
            0x00c4_f1d0,
            process::append_virtual(Ptr<BSCullingProcess>, Ptr<NiGeometry>)
        ),
        entry!(
            0x00a6_94a0,
            process::set_frustum(Ptr<NiCullingProcess>, Ptr<NiFrustum>)
        ),
        entry!(
            0x00c4_f070,
            process::bs_process_scene(Ptr<BSCullingProcess>, u32, Ptr<NiAVObject>, u32)
        ),
        entry!(
            0x00bc_2b30,
            process::fade_culler_process(Ptr<NiCullingProcess>, u32)
        ),
        entry!(
            0x00c3_87f0,
            process::within_frustum(Ptr<BSMultiBoundAABB>, Ptr<NiFrustumPlanes>) -> bool
        ),
        entry!(
            0x00a7_fd90,
            visible::geometry_on_visible(Ptr<NiGeometry>, Ptr<NiCullingProcess>)
        ),
        entry!(
            0x00c4_6f60,
            visible::multibound_node_on_visible(Ptr<BSMultiBoundNode>, Ptr<BSCullingProcess>)
        ),
        entry!(0x00a6_8bf0, update::update_world_transform(Ptr<NiAVObject>)),
        entry!(0x00a6_8c60, update::update_world_data(Ptr<NiAVObject>, u32)),
        entry!(
            0x00a7_fdd0,
            update::bound_update(Ptr<NiBound>, Ptr<NiBound>, Ptr<NiTransform>)
        ),
        entry!(
            0x00a7_fea0,
            update::bound_transform(Ptr<NiBound>, Ptr<NiTransform>)
        ),
        entry!(
            0x00a8_0710,
            update::geometry_update_world_bound(Ptr<NiGeometry>)
        ),
        entry!(
            0x004a_51d0,
            update::set_center_and_radius(Ptr<NiBound>, Ptr<NiPoint3>, f32)
        ),
        entry!(0x0054_67e0, update::ensure_world_bound(Ptr<NiAVObject>)),
        entry!(0x00a5_e2d0, update::node_update_world_bound(Ptr<NiNode>)),
        entry!(
            0x00a6_9990,
            update::plane_through(Ptr<NiPlane>, Ptr<NiPoint3>, Ptr<NiPoint3>) -> Ptr
        ),
        entry!(
            0x00a7_4e10,
            planes::frustum_planes_set(Ptr<NiFrustumPlanes>, Ptr<NiFrustum>, Ptr<NiTransform>)
        ),
        entry!(
            0x00c4_7980,
            compound::set_camera(Ptr<BSCompoundFrustum>, u32)
        ),
        entry!(
            0x00c4_9050,
            compound::active_state(Ptr<BSCompoundFrustum>) -> u32
        ),
        entry!(
            0x00c4_9100,
            compound::set_active_state(Ptr<BSCompoundFrustum>, u32)
        ),
        entry!(
            0x00c4_91a0,
            compound::process(Ptr<BSCompoundFrustum>, Ptr<NiAVObject>) -> bool
        ),
    ]
}
