//! The bounds and transforms the culling reads, as the update pass makes them: an object's world
//! transform, a bound moved by a transform, a geometry's and a node's world bound, a plane through a
//! point.

use super::layout::{
    NiAVObject, NiBound, NiGeometry, NiGeometryData, NiMatrix3, NiNode, NiPlane, NiPoint3,
    NiTransform,
};
use super::{
    bound, copy_float, fld, fld_local, fstp, m32, m32_local, radius_not_zero, FAMILY, NULL_BOUND,
    POINT_ZERO,
};
use crate::dispatch;
use engine_core::ptr::{Field, Ptr};
use engine_core::x87::{x87_add, x87_mul};
use engine_core::{args, typed_call, Engine};

/// The collision object's world update, virtual slot 0x90 (`thiscall(collision object, word)`).
const SLOT_COLLISION_UPDATE: u32 = 0x90;
/// The object's world-bound setter, virtual slot 0x78 (`thiscall(object, bound)`).
const SLOT_SET_WORLD_BOUND: u32 = 0x78;
/// The pool the geometry's new bound comes from, and its allocation (`thiscall(pool, size)`).
const BOUND_POOL: u32 = 0x011f_6238;
const POOL_ALLOCATE: u32 = 0x00aa_3e40;
/// `operator new` (`cdecl(size)`) and the bound block's set-up (`thiscall(block, 0x100, 0)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const BOUND_BLOCK_SETUP: u32 = 0x0054_6890;
/// The skin instance's bound update (`thiscall(skin, bound)`).
const SKIN_BOUND: u32 = 0x00a8_6530;
/// Whether the world bound changed since the copy (`thiscall(bound, old copy)`, a bool).
const BOUND_CHANGED: u32 = 0x00a9_2660;

/// `m_uFlags` bits the updates read.
const FLAG_WORLD_IS_PARENTS: u32 = 0x200;
const FLAG_BOUND_FIXED: u32 = 0x2000;
const FLAG_BOUND_FROM_CHILDREN: u32 = 0x0008_0000;
const FLAG_BOUND_CHANGED: u32 = 0x0040_0000;

/// The thirteen words of a transform, as `REP MOVSD` copies them.
#[inline(always)]
fn copy_transform(e: &mut Engine, to: Ptr<NiTransform>, from: Ptr<NiTransform>) {
    for word in 0..13 {
        let value = e.mem.u32(from.addr().wrapping_add(4 * word));
        e.mem.set_u32(to.addr().wrapping_add(4 * word), value);
    }
}

// Typed from 00a68bf0
/// The world-transform part of `NiAVObject::UpdateWorldData` (`thiscall(object)`): without a
/// parent the world transform is the local one; with bit 9 of the flags it is the parent's world
/// transform; otherwise the parent's world times the local (`0062c250`, into a local), copied in.
pub fn update_world_transform(e: &mut Engine, this: Ptr<NiAVObject>) {
    let world = this.at(NiAVObject::m_kWorld);
    let parent = e.get(this, NiAVObject::m_pkParent);
    if parent.is_null() {
        copy_transform(e, world, this.at(NiAVObject::m_kLocal));
        return;
    }
    let parent_world = parent.cast::<NiAVObject>().at(NiAVObject::m_kWorld);
    if e.get(this, NiAVObject::m_uFlags) & FLAG_WORLD_IS_PARENTS != 0 {
        copy_transform(e, world, parent_world);
        return;
    }
    e.with_stack(0x34, |e, product| {
        let local = this.at(NiAVObject::m_kLocal);
        let result = typed_call!(
            dispatch::FAMILY,
            0x0062_c250,
            dispatch::math::compose(e, parent_world, product.cast(), local)
        );
        copy_transform(e, world, result);
    });
}

// Typed from 00a68c60
/// `NiAVObject::UpdateWorldData` (Xbox PDB, `thiscall(object, word)`): an object with a collision
/// object hands the word to it (slot 0x90); the others update their world transform.
pub fn update_world_data(e: &mut Engine, this: Ptr<NiAVObject>, word: u32) {
    let collision = e.get(this, NiAVObject::m_spCollisionObject);
    if collision != 0 {
        e.vcall(collision, SLOT_COLLISION_UPDATE, &args![word]);
        return;
    }
    typed_call!(FAMILY, 0x00a6_8bf0, update_world_transform(e, this));
}

/// Row `row` of `matrix` times `point`, summed in the order `first + second + third` the caller
/// gives as `(entry column, point component)` pairs, each entry loaded and each point component a
/// memory operand; stored to a float.
#[inline(always)]
fn row_dot(
    e: &Engine,
    matrix: Ptr<NiMatrix3>,
    row: u32,
    point: Ptr<NiPoint3>,
    order: [(u32, Field<NiPoint3, f32>); 3],
) -> f32 {
    let product = |(column, component): (u32, Field<NiPoint3, f32>)| {
        x87_mul(
            fld(e, matrix, NiMatrix3::entry(row, column)),
            m32(e, point, component),
        )
    };
    fstp(x87_add(
        x87_add(product(order[0]), product(order[1])),
        product(order[2]),
    ))
}

/// The rows' dot products scaled, plus the translation, stored: the part `00a7fdd0` and `00a7fea0`
/// share once the three dot products are stored.
#[inline(always)]
fn scale_and_translate(e: &Engine, transform: Ptr<NiTransform>, rows: [f32; 3]) -> [f32; 3] {
    // FLD scale; FSTP (a stack slot); then FLD row0; FLD scale; FLD ST0; FMULP ST2; FXCH; FSTP;
    // FLD row1; FMUL ST1; FSTP; FMUL row2 (memory); FSTP.
    let scale = fld_local(fstp(fld(e, transform, NiTransform::m_fScale)));
    let scaled = [
        fstp(x87_mul(fld_local(rows[0]), scale)),
        fstp(x87_mul(fld_local(rows[1]), scale)),
        fstp(x87_mul(scale, m32_local(rows[2]))),
    ];
    let translate = transform.at(NiTransform::m_Translate);
    [
        fstp(x87_add(
            fld(e, translate, NiPoint3::x),
            m32_local(scaled[0]),
        )),
        fstp(x87_add(
            fld(e, translate, NiPoint3::y),
            m32_local(scaled[1]),
        )),
        fstp(x87_add(
            fld(e, translate, NiPoint3::z),
            m32_local(scaled[2]),
        )),
    ]
}

/// Stores a centre.
#[inline(always)]
fn set_center(e: &mut Engine, bound: Ptr<NiBound>, center: [f32; 3]) {
    let at = bound.at(NiBound::m_kCenter);
    e.set(at, NiPoint3::x, center[0]);
    e.set(at, NiPoint3::y, center[1]);
    e.set(at, NiPoint3::z, center[2]);
}

// Typed from 00a7fdd0
/// `NiBound::Update` (Xbox PDB, `thiscall(this, bound, transform)`): `this` becomes `bound` moved by
/// the transform: centre = translation + scale * (rotation * centre), each row's dot product and
/// each step stored to a float; radius = radius * scale. The first row multiplies its second term
/// the other way round (`centre.x` loaded, the entry a memory operand), as the exe does.
pub fn bound_update(
    e: &mut Engine,
    this: Ptr<NiBound>,
    from: Ptr<NiBound>,
    transform: Ptr<NiTransform>,
) {
    let matrix = transform.at(NiTransform::m_Rotate);
    let center = from.at(NiBound::m_kCenter);
    let row0 = fstp(x87_add(
        x87_add(
            x87_mul(
                fld(e, matrix, NiMatrix3::entry(0, 1)),
                m32(e, center, NiPoint3::y),
            ),
            x87_mul(
                fld(e, center, NiPoint3::x),
                m32(e, matrix, NiMatrix3::entry(0, 0)),
            ),
        ),
        x87_mul(
            fld(e, matrix, NiMatrix3::entry(0, 2)),
            m32(e, center, NiPoint3::z),
        ),
    ));
    let natural = [(0, NiPoint3::x), (1, NiPoint3::y), (2, NiPoint3::z)];
    let row1 = row_dot(e, matrix, 1, center, natural);
    let row2 = row_dot(e, matrix, 2, center, natural);
    let moved = scale_and_translate(e, transform, [row0, row1, row2]);
    set_center(e, this, moved);
    let radius = fstp(x87_mul(
        fld(e, from, NiBound::m_fRadius),
        m32(e, transform, NiTransform::m_fScale),
    ));
    e.set(this, NiBound::m_fRadius, radius);
}

// Typed from 00a7fea0
/// The in-place form of [`bound_update`] (`thiscall(bound, transform)`): the same steps on the
/// bound itself, its first row summed `e01*y + e00*x` (both entries loaded); the radius is the
/// scale loaded times the radius as a memory operand.
pub fn bound_transform(e: &mut Engine, this: Ptr<NiBound>, transform: Ptr<NiTransform>) {
    let matrix = transform.at(NiTransform::m_Rotate);
    let center = this.at(NiBound::m_kCenter);
    let row0 = row_dot(
        e,
        matrix,
        0,
        center,
        [(1, NiPoint3::y), (0, NiPoint3::x), (2, NiPoint3::z)],
    );
    let natural = [(0, NiPoint3::x), (1, NiPoint3::y), (2, NiPoint3::z)];
    let row1 = row_dot(e, matrix, 1, center, natural);
    let row2 = row_dot(e, matrix, 2, center, natural);
    let moved = scale_and_translate(e, transform, [row0, row1, row2]);
    set_center(e, this, moved);
    let radius = fstp(x87_mul(
        fld(e, transform, NiTransform::m_fScale),
        m32(e, this, NiBound::m_fRadius),
    ));
    e.set(this, NiBound::m_fRadius, radius);
}

// Typed from 00a80710
/// `NiGeometry::UpdateWorldBound` (Xbox PDB, `thiscall(geometry)`): keeps a copy of the world bound
/// (the empty one without), then goes on only with bit 19 of the flags set and bit 13 clear, or when
/// the copy's radius is zero (a NaN radius stops). Without a bound one comes from the bound pool
/// (slot 0x78), centred at zero with radius 0. A skinned geometry has its skin update the bound and
/// moves it by the world transform in place (`00a7fea0`); the others move the model's bound
/// (`00a7fdd0`). With bit 22 of the flags set, it stays set only when the bound changed
/// (`00a92660` against the copy).
pub fn geometry_update_world_bound(e: &mut Engine, this: Ptr<NiGeometry>) {
    let object = this.cast::<NiAVObject>();
    let bound = e.get(object, NiAVObject::m_kWorldBound);
    let source = if bound.is_null() { NULL_BOUND } else { bound };
    // The copy: `00a92660` takes its address.
    e.with_stack(0x10, |e, old| {
        let old = old.cast::<NiBound>();
        let (from, to) = (source.at(NiBound::m_kCenter), old.at(NiBound::m_kCenter));
        copy_float(e, from, NiPoint3::x, to, NiPoint3::x);
        copy_float(e, from, NiPoint3::y, to, NiPoint3::y);
        copy_float(e, from, NiPoint3::z, to, NiPoint3::z);
        copy_float(e, source, NiBound::m_fRadius, old, NiBound::m_fRadius);
        let flags = e.get(object, NiAVObject::m_uFlags);
        let forced = flags & FLAG_BOUND_FROM_CHILDREN != 0 && flags & FLAG_BOUND_FIXED == 0;
        // FLDZ; FCOMP radius; TEST AH,0x44; JP: on only for an ordered equal.
        if !forced && radius_not_zero(e, old) {
            return;
        }
        if bound.is_null() {
            let block = e.call(POOL_ALLOCATE, &args![BOUND_POOL, 0x10u32]).u32();
            e.vcall(object.addr(), SLOT_SET_WORLD_BOUND, &args![block]);
            let made = e.get(object, NiAVObject::m_kWorldBound);
            let zero = Ptr::<NiPoint3>::new(POINT_ZERO);
            let center = made.at(NiBound::m_kCenter);
            copy_float(e, zero, NiPoint3::x, center, NiPoint3::x);
            copy_float(e, zero, NiPoint3::y, center, NiPoint3::y);
            copy_float(e, zero, NiPoint3::z, center, NiPoint3::z);
            e.set(made, NiBound::m_fRadius, 0.0);
        }
        let world = object.at(NiAVObject::m_kWorld);
        let skin = e.get(this, NiGeometry::m_spSkinInstance);
        if skin != 0 {
            let bound = e.get(object, NiAVObject::m_kWorldBound);
            e.call(SKIN_BOUND, &args![skin, bound]);
            let bound = e.get(object, NiAVObject::m_kWorldBound);
            typed_call!(FAMILY, 0x00a7_fea0, bound_transform(e, bound, world));
        } else {
            let data = e.get(this, NiGeometry::m_spModelData);
            let bound = e.get(object, NiAVObject::m_kWorldBound);
            let model = data.at(NiGeometryData::m_kBound);
            typed_call!(FAMILY, 0x00a7_fdd0, bound_update(e, bound, model, world));
        }
        let flags = e.get(object, NiAVObject::m_uFlags);
        if flags & FLAG_BOUND_CHANGED != 0 {
            let bound = e.get(object, NiAVObject::m_kWorldBound);
            let changed = e.call(BOUND_CHANGED, &args![bound, old]).bool();
            let flags = if changed {
                flags | FLAG_BOUND_CHANGED
            } else {
                flags & !FLAG_BOUND_CHANGED
            };
            e.set(object, NiAVObject::m_uFlags, flags);
        }
    });
}

// Typed from 004a51d0
/// `NiBound::SetCenterAndRadius` (`thiscall(bound, centre, radius)`): the centre's words copied,
/// the radius through the x87 stack (a signalling NaN quieted).
pub fn set_center_and_radius(
    e: &mut Engine,
    this: Ptr<NiBound>,
    center: Ptr<NiPoint3>,
    radius: f32,
) {
    let to = this.at(NiBound::m_kCenter);
    copy_float(e, center, NiPoint3::x, to, NiPoint3::x);
    copy_float(e, center, NiPoint3::y, to, NiPoint3::y);
    copy_float(e, center, NiPoint3::z, to, NiPoint3::z);
    e.set(this, NiBound::m_fRadius, fstp(fld_local(radius)));
}

// Typed from 005467e0
/// Gives an object without a world bound one (`thiscall(object)`): a 16-byte block (`operator
/// new`, then `00546890` on it), set through slot 0x78 (null when the allocation failed), then
/// centred at zero with radius 0 (`004a51d0`).
pub fn ensure_world_bound(e: &mut Engine, this: Ptr<NiAVObject>) {
    if !e.get(this, NiAVObject::m_kWorldBound).is_null() {
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let bound = if block == 0 {
        0
    } else {
        e.call(BOUND_BLOCK_SETUP, &args![block, 0x100u32, 0u32])
            .u32()
    };
    e.vcall(this.addr(), SLOT_SET_WORLD_BOUND, &args![bound]);
    let bound = e.get(this, NiAVObject::m_kWorldBound);
    typed_call!(
        FAMILY,
        0x004a_51d0,
        set_center_and_radius(e, bound, Ptr::new(POINT_ZERO), 0.0)
    );
}

// Typed from 00a5e2d0
/// `NiNode::UpdateWorldBound` (Xbox PDB, `thiscall(node)`): only with bit 13 of the flags clear and
/// bit 19 set. The first child with a world bound of a radius other than zero (a NaN radius
/// counts; the slots walked from the array read once) gives the node a bound if it has none
/// (`005467e0`) and is copied into it (the radius through a float local); each later such child
/// (the array read at each step) is merged in (`00a7f3f0`). The child count is read once.
pub fn node_update_world_bound(e: &mut Engine, this: Ptr<NiNode>) {
    let object = this.cast::<NiAVObject>();
    let flags = e.get(object, NiAVObject::m_uFlags);
    if flags & FLAG_BOUND_FIXED != 0 || flags & FLAG_BOUND_FROM_CHILDREN == 0 {
        return;
    }
    let count = u32::from(e.get(this, NiNode::m_usSize));
    if count == 0 {
        return;
    }
    let slots = e.get(this, NiNode::m_pBase);
    let mut index = 0;
    let first = loop {
        let child = Ptr::<NiAVObject>::new(e.mem.u32(slots.index(index, 4).addr()));
        if !child.is_null() {
            let bound = e.get(child, NiAVObject::m_kWorldBound);
            if !bound.is_null() && radius_not_zero(e, bound) {
                break child;
            }
        }
        index += 1;
        if index >= count {
            return;
        }
    };
    typed_call!(FAMILY, 0x0054_67e0, ensure_world_bound(e, object));
    let source = e.get(first, NiAVObject::m_kWorldBound);
    let source = if source.is_null() { NULL_BOUND } else { source };
    // FLD radius; FSTP local; FLD local; ... FSTP into the node's bound.
    let radius = fstp(fld_local(fstp(fld(e, source, NiBound::m_fRadius))));
    let own = e.get(object, NiAVObject::m_kWorldBound);
    let (from, to) = (source.at(NiBound::m_kCenter), own.at(NiBound::m_kCenter));
    copy_float(e, from, NiPoint3::x, to, NiPoint3::x);
    copy_float(e, from, NiPoint3::y, to, NiPoint3::y);
    copy_float(e, from, NiPoint3::z, to, NiPoint3::z);
    e.set(own, NiBound::m_fRadius, radius);
    index += 1;
    while index < count {
        let slots = e.get(this, NiNode::m_pBase);
        let child = Ptr::<NiAVObject>::new(e.mem.u32(slots.index(index, 4).addr()));
        if !child.is_null() {
            let bound = e.get(child, NiAVObject::m_kWorldBound);
            if !bound.is_null() && radius_not_zero(e, bound) {
                let own = e.get(object, NiAVObject::m_kWorldBound);
                typed_call!(FAMILY, 0x00a7_f3f0, bound::merge(e, own, bound));
            }
        }
        index += 1;
    }
}

// Typed from 00a69990
/// `NiPlane::NiPlane` (Xbox PDB, `thiscall(plane, normal, point)`): the normal's words copied, the
/// constant `(ny*py + nx*px) + nz*pz` from the normal given (not the copy), stored. Returns the
/// plane, as a constructor does.
pub fn plane_through(
    e: &mut Engine,
    this: Ptr<NiPlane>,
    normal: Ptr<NiPoint3>,
    point: Ptr<NiPoint3>,
) -> Ptr<NiPlane> {
    let to = this.at(NiPlane::m_kNormal);
    copy_float(e, normal, NiPoint3::x, to, NiPoint3::x);
    copy_float(e, normal, NiPoint3::y, to, NiPoint3::y);
    copy_float(e, normal, NiPoint3::z, to, NiPoint3::z);
    let y = x87_mul(fld(e, normal, NiPoint3::y), m32(e, point, NiPoint3::y));
    let x = x87_mul(fld(e, normal, NiPoint3::x), m32(e, point, NiPoint3::x));
    let z = x87_mul(fld(e, normal, NiPoint3::z), m32(e, point, NiPoint3::z));
    e.set(this, NiPlane::m_fConstant, fstp(x87_add(x87_add(y, x), z)));
    this
}
