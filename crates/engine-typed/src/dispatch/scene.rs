//! The scene graph's walks and getters: an object's controllers and their updates, a node's children
//! and the downward update passes, the collision object and its RTTI cast, a reference's loaded 3D,
//! the frustum test of a bound and the geometry's immediate render.

use super::{copy_words, FAMILY};
use crate::culling::layout::{NiAVObject, NiBound, NiNode, NiPlane};
use crate::culling::{self, NULL_BOUND};
use engine_core::ptr::Ptr;
use engine_core::typed::leaf;
use engine_core::x87::{x87_m32, x87_quiet_bits, x87_store32};
use engine_core::{args, typed_call, Engine};

/// `BSSimpleList::GetNext` of a list position (`0057cbe0`).
const LIST_GET_NEXT: u32 = 0x0057_cbe0;
/// What a node calls before it takes its first child's bound as its own (`005467e0`).
const BEFORE_BOUND_COPY: u32 = 0x0054_67e0;
/// The RTTI type descriptor of `bhkCollisionObject` (`012043f8`).
const BHK_COLLISION_OBJECT: u32 = 0x0120_43f8;
/// The shared zero vector's getter (`00458b20`).
const ZERO_VECTOR: u32 = 0x0045_8b20;
/// The frustum planes kept for the bound test, their guard, camera and stamp.
const FRUSTUM_PLANES: u32 = 0x011c_6330;
const FRUSTUM_GUARD: u32 = 0x011c_6394;
const FRUSTUM_CAMERA: u32 = 0x011c_632c;
const FRUSTUM_STAMP: u32 = 0x0118_90e0;
/// `NiFrustumPlanes::NiFrustumPlanes` (`0045c620`) and `::Set` from a camera (`00a755d0`).
const FRUSTUM_PLANES_CONSTRUCT: u32 = 0x0045_c620;
const FRUSTUM_PLANES_SET: u32 = 0x00a7_55d0;
/// The camera state stamp (`00825c00`) of its owner (`011f6394`).
const CAMERA_STAMP: u32 = 0x0082_5c00;
const CAMERA_STAMP_OWNER: u32 = 0x011f_6394;
/// The thread's reference whose 3D is being built, and that 3D, in its thread block.
const TLS_REFERENCE: u32 = 0x264;
const TLS_REFERENCE_3D: u32 = 0x260;
/// The renderer's draw mode, its immediate flag, and its `DrawTriStrips` slot.
const RENDERER_MODE: u32 = 0x200;
const RENDERER_IMMEDIATE: u32 = 0x208;
const DRAW_TRI_STRIPS: u32 = 0x1b8;

/// A node's flags (`+0x30`).
#[inline(always)]
fn flags(e: &Engine, node: u32) -> u32 {
    e.mem.u32(node + 0x30)
}

// Typed from 0043b230
/// `NiObjectNET::GetControllers` (`thiscall(object)`): the head of the controller list (`+0xc`).
pub fn first_controller(e: &mut Engine, object: Ptr<NiAVObject>) -> Ptr {
    let head = object.addr().wrapping_add(0xc);
    Ptr::new(typed_call!(
        leaf::FAMILY,
        0x0055_9450,
        leaf::word_at_0(e, head)
    ))
}

// Typed from 004a8a90
/// `NiTimeController::GetNext` (`thiscall(this)`): the next controller (`+0x30`).
pub fn next_controller(e: &mut Engine, this: Ptr) -> u32 {
    let next = this.addr().wrapping_add(0x30);
    typed_call!(leaf::FAMILY, 0x0055_9450, leaf::word_at_0(e, next))
}

// Typed from 004eff50
/// `NiAVObject::UpdateObjectControllers` (`thiscall(this, data, properties)`): with `properties` set
/// and flag 0x40000 (`004f0000`), `Update` (vtable +0x90) of each property in the property list
/// (`+0x24`) that has controllers; then `Update` (vtable +0x94) of each of the object's controllers.
pub fn update_object_controllers(e: &mut Engine, this: Ptr<NiAVObject>, data: Ptr, properties: u8) {
    if properties != 0 && typed_call!(FAMILY, 0x004f_0000, has_property_controllers(e, this)) {
        let list = this.addr().wrapping_add(0x24);
        e.with_stack(4, |e, position| {
            let head = typed_call!(leaf::FAMILY, 0x0055_9450, leaf::word_at_0(e, list));
            e.mem.set_u32(position.addr(), head);
            while e.mem.u32(position.addr()) != 0 {
                let element = e.call(LIST_GET_NEXT, &args![list, position]).u32();
                let property = typed_call!(leaf::FAMILY, 0x0055_9450, leaf::word_at_0(e, element));
                if property != 0
                    && !typed_call!(FAMILY, 0x0043_b230, first_controller(e, Ptr::new(property)))
                        .is_null()
                {
                    e.vcall(property, 0x90, &args![data]);
                }
            }
        });
    }
    let mut controller = typed_call!(FAMILY, 0x0043_b230, first_controller(e, this)).addr();
    while controller != 0 {
        e.vcall(controller, 0x94, &args![data]);
        controller = typed_call!(
            FAMILY,
            0x004a_8a90,
            next_controller(e, Ptr::new(controller))
        );
    }
}

// Typed from 004f0000
/// Whether the object's flags (`+0x30`) have 0x40000 (`00456630`).
pub fn has_property_controllers(e: &mut Engine, this: Ptr<NiAVObject>) -> bool {
    typed_call!(
        leaf::FAMILY,
        0x0045_6630,
        leaf::flags_at_30_any(e, this, 0x0004_0000u32)
    )
}

// Typed from 0043b480
/// `NiNode::GetArrayCount` (`thiscall(this)`): the size of the children array (`+0x9c`, `00658930`).
pub fn child_count(e: &mut Engine, this: Ptr<NiNode>) -> u16 {
    typed_call!(FAMILY, 0x0065_8930, array_size(e, this.byte_add(0x9c)))
}

// Typed from 0043b4a0
/// `NiNode::GetAt` (`thiscall(this, index)`): the child pointer at `index` of the children array
/// (`00877a30`), dereferenced.
pub fn child(e: &mut Engine, this: Ptr<NiNode>, index: u32) -> Ptr {
    let children = this.addr().wrapping_add(0x9c);
    let slot = typed_call!(
        leaf::FAMILY,
        0x0087_7a30,
        leaf::array_element(e, children, index)
    );
    Ptr::new(typed_call!(
        leaf::FAMILY,
        0x0055_9450,
        leaf::word_at_0(e, slot)
    ))
}

// Typed from 00658930
/// `NiTArray::GetSize` (`thiscall(this)`): the array's size (`+0xa`).
pub fn array_size(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr().wrapping_add(0xa))
}

/// The child at `index` of a node's children array.
#[inline(always)]
fn child_at(e: &Engine, node: u32, index: u32) -> u32 {
    let base = e.mem.u32(node + 0xa0);
    e.mem.u32(base.wrapping_add(index.wrapping_mul(4)))
}

/// A bound's radius not 0.0 (`FCOMP` against it as a memory operand: NaN is not 0.0).
#[inline(always)]
fn radius_not_zero(e: &Engine, bound: u32) -> bool {
    x87_m32(e.mem.u32(bound + 0xc)) != 0.0
}

/// A node whose bound is merged from its children's: flag 0x80000 without 0x2000.
#[inline(always)]
fn merges_bounds(flags: u32) -> bool {
    (flags >> 0xd) & 1 == 0 && (flags >> 0x13) & 1 != 0
}

/// The node's world bound becomes `source`'s (or the null bound's): centre and radius, the radius
/// through the x87 stack, in the exe's order of stores.
fn copy_bound(e: &mut Engine, node: u32, source: u32) {
    let source = if source != 0 {
        source
    } else {
        NULL_BOUND.addr()
    };
    let x = e.mem.u32(source);
    let radius = x87_quiet_bits(e.mem.u32(source + 0xc));
    let target = e.mem.u32(node + 0x20);
    e.mem.set_u32(target, x);
    let y = e.mem.u32(source + 4);
    e.mem.set_u32(target + 4, y);
    let z = e.mem.u32(source + 8);
    e.mem.set_u32(target + 0xc, radius);
    e.mem.set_u32(target + 8, z);
}

/// The children loop of the three downward passes: `visit` on each child; a node whose bound comes
/// from its children (`merges_bounds`) takes its first child's with a radius (`copy_bound`, the old
/// radius zeroed first when `fresh`) and merges the rest's into it (`00a7f3f0`).
fn update_children(
    e: &mut Engine,
    node: u32,
    count: u32,
    fresh: bool,
    visit: &mut dyn FnMut(&mut Engine, u32),
) {
    let flags_now = flags(e, node);
    let mut index = 0;
    if (flags_now >> 0x13) & 1 != 0 {
        if (flags_now >> 0xd) & 1 == 0 {
            let bound = e.mem.u32(node + 0x20);
            if fresh && bound != 0 {
                e.mem.set_u32(bound + 0xc, x87_store32(0.0));
            }
            if count == 0 {
                return;
            }
            loop {
                let child = child_at(e, node, index);
                if child != 0 {
                    visit(e, child);
                    let bound = e.mem.u32(child + 0x20);
                    if bound != 0 && radius_not_zero(e, bound) && fresh {
                        e.call(BEFORE_BOUND_COPY, &args![node]);
                        let source = e.mem.u32(child + 0x20);
                        copy_bound(e, node, source);
                        index += 1;
                        break;
                    }
                }
                index += 1;
                if index >= count {
                    return;
                }
            }
        } else {
            e.call(BEFORE_BOUND_COPY, &args![node]);
        }
    }
    while index < count {
        let child = child_at(e, node, index);
        if child != 0 {
            visit(e, child);
            if merges_bounds(flags(e, node)) {
                let bound = e.mem.u32(child + 0x20);
                if bound != 0 && radius_not_zero(e, bound) {
                    let own = Ptr::<NiBound>::new(e.mem.u32(node + 0x20));
                    typed_call!(
                        culling::FAMILY,
                        0x00a7_f3f0,
                        culling::bound::merge(e, own, Ptr::new(bound))
                    );
                }
            }
        }
        index += 1;
    }
}

/// The number of children (`+0xa6`).
#[inline(always)]
fn children_size(e: &Engine, node: u32) -> u32 {
    u32::from(e.mem.u16(node + 0xa6))
}

// Typed from 00a5dd70
/// `NiNode::UpdateDownwardPass` (`thiscall(this, data, flags)`): the object's controllers when the
/// data says so (`004eff50`), `UpdateWorldData` (vtable +0xb8), then each child's `UpdateDownwardPass`
/// (vtable +0xa4) with the flags (bit 0 added from the node's), the bound merged from theirs.
pub fn update_downward(e: &mut Engine, this: Ptr<NiNode>, data: u32, update_flags: u32) {
    let t = this.addr();
    if e.mem.u8(data + 4) != 0 {
        typed_call!(
            FAMILY,
            0x004e_ff50,
            update_object_controllers(e, this.cast(), Ptr::new(data), 1)
        );
    }
    // `NiAVObject::UpdateWorldData` (`00a68c60`, every class this reaches) directly when the slot
    // holds it.
    if crate::slot_target(e, t, 0xb8) == 0x00a6_8c60 {
        typed_call!(
            crate::culling::FAMILY,
            0x00a6_8c60,
            crate::culling::update::update_world_data(e, this.cast(), data)
        );
    } else {
        e.vcall(t, 0xb8, &args![data]);
    }
    let update_flags = update_flags | (flags(e, t) & 1);
    let count = children_size(e, t);
    update_children(e, t, count, true, &mut |e, child| {
        // A child node's `UpdateDownwardPass` (`00a5dd70`, this function) directly.
        if crate::slot_target(e, child, 0xa4) == 0x00a5_dd70 {
            typed_call!(
                FAMILY,
                0x00a5_dd70,
                update_downward(e, Ptr::new(child), data, update_flags)
            );
        } else {
            e.vcall(child, 0xa4, &args![data, update_flags]);
        }
    });
}

// Typed from 00a5ded0
/// `NiNode::UpdateSelectedDownwardPass` (`thiscall(this, data, flags)`): the controllers with the
/// data's flag at +4 set to the node's selected bit (bit 3) for the call; `UpdateWorldData` when bit 2
/// is set and the data's +5 is clear; then each child with bit 1, by its bit 4 `UpdateRigidDownward`
/// (+0xac) or `UpdateSelectedDownward` (+0xa8).
pub fn update_selected_downward(e: &mut Engine, this: Ptr<NiNode>, data: u32, update_flags: u32) {
    let t = this.addr();
    let saved = e.mem.u8(data + 4);
    let selected = u8::from((flags(e, t) >> 3) & 1 != 0);
    e.mem.set_u8(data + 4, selected);
    typed_call!(
        FAMILY,
        0x004e_ff50,
        update_object_controllers(e, this.cast(), Ptr::new(data), 1)
    );
    let fresh = e.mem.u8(data + 5) == 0;
    e.mem.set_u8(data + 4, saved);
    if (flags(e, t) >> 2) & 1 != 0 && fresh {
        e.vcall(t, 0xb8, &args![data]);
    }
    let update_flags = update_flags | (flags(e, t) & 1);
    let count = children_size(e, t);
    update_children(e, t, count, fresh, &mut |e, child| {
        let child_flags = flags(e, child);
        if (child_flags >> 1) & 1 != 0 {
            let slot = if (child_flags >> 4) & 1 != 0 {
                0xac
            } else {
                0xa8
            };
            e.vcall(child, slot, &args![data, update_flags]);
        }
    });
}

// Typed from 00a5e0b0
/// `NiNode::UpdateTransformAndBounds` (`thiscall(this, data)`): `UpdateWorldData` when bit 2 is set,
/// then each child's `UpdateTransformAndBounds` (vtable +0xc0), the bound merged from theirs.
pub fn update_transform_and_bounds(e: &mut Engine, this: Ptr<NiNode>, data: u32) {
    let t = this.addr();
    if (flags(e, t) >> 2) & 1 != 0 {
        e.vcall(t, 0xb8, &args![data]);
    }
    let count = children_size(e, t);
    update_children(e, t, count, true, &mut |e, child| {
        e.vcall(child, 0xc0, &args![data]);
    });
}

// Typed from 00a80610
/// `NiGeometry::RenderImmediate` (`thiscall(this, renderer)`): the renderer's property state becomes
/// the geometry's (`+0x9c`), then each controller's `OnPreDisplay` (vtable +0xa8), along `+0x30`.
pub fn geometry_render_immediate(e: &mut Engine, this: Ptr, renderer: Ptr) {
    let base = this.addr();
    e.mem.set_u32(renderer.addr() + 0xc, base + 0x9c);
    let mut controller = e.mem.u32(base + 0xc);
    while controller != 0 {
        e.vcall(controller, 0xa8, &args![]);
        controller = e.mem.u32(controller + 0x30);
    }
}

// Typed from 00a71d70
/// `NiTriStrips::RenderImmediate` (`thiscall(this, renderer)`): `00a80610`, then, in draw mode 1 or 2
/// with the immediate flag, the renderer's `DrawTriStrips` (vtable +0x1b8).
pub fn tri_strips_render_immediate(e: &mut Engine, this: Ptr, renderer: Ptr) {
    typed_call!(
        FAMILY,
        0x00a8_0610,
        geometry_render_immediate(e, this, renderer)
    );
    let mode = e.mem.u32(renderer.addr().wrapping_add(RENDERER_MODE));
    if (mode == 1 || mode == 2) && e.mem.u8(renderer.addr().wrapping_add(RENDERER_IMMEDIATE)) == 1 {
        e.vcall(renderer.addr(), DRAW_TRI_STRIPS, &args![this]);
    }
}

// Typed from 0043fcd0
/// `TESObjectREFR::Get3D` (`thiscall(this)`): while this thread builds this reference's 3D, the one
/// it holds in its thread block; otherwise the loaded data's (`+0x64`) root (`+0x14`), or null.
pub fn reference_3d(e: &mut Engine, this: Ptr) -> Ptr {
    let tls = e.tls();
    if this.addr() == e.mem.u32(tls + TLS_REFERENCE) {
        return Ptr::new(e.mem.u32(tls + TLS_REFERENCE_3D));
    }
    if e.mem.u32(this.addr() + 0x64) == 0 {
        return Ptr::NULL;
    }
    let data = e.mem.u32(this.addr() + 0x64);
    Ptr::new(typed_call!(
        leaf::FAMILY,
        0x0055_9450,
        leaf::word_at_0(e, data + 0x14)
    ))
}

// Typed from 0043b4f0
/// The loaded 3D of a model handle into the pointer at `out` (`thiscall(this, out)`, `008c71b0`):
/// the held entry's (`0043b560`) node (`0043b540`), or null.
pub fn loaded_3d(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let holder = typed_call!(FAMILY, 0x0043_b560, loaded_handle(e, this));
    let value = if holder != 0 {
        typed_call!(FAMILY, 0x0043_b540, handle_3d(e, Ptr::new(holder)))
    } else {
        0
    };
    typed_call!(FAMILY, 0x008c_71b0, super::refs::store_word(e, out, value));
    out
}

// Typed from 0043b540
/// The node of a loaded entry: the word at +8 of its `+0x14`.
pub fn handle_3d(e: &mut Engine, this: Ptr) -> u32 {
    let at = this.addr().wrapping_add(0x14);
    typed_call!(leaf::FAMILY, 0x0044_ddc0, leaf::word_at_8(e, at))
}

// Typed from 0043b560
/// The value a counted holder holds (`thiscall(this)`): its object (`00620b80`), locked (`0043b5b0`)
/// for the read of its `+0x10` (`00460140`) and unlocked (`0043b5f0`); 0 without one.
pub fn loaded_handle(e: &mut Engine, this: Ptr) -> u32 {
    let held = typed_call!(FAMILY, 0x0062_0b80, word_8(e, this.addr()));
    if held == 0 {
        return 0;
    }
    let held = Ptr::new(held);
    typed_call!(FAMILY, 0x0043_b5b0, lock_loaded(e, held));
    let value = typed_call!(FAMILY, 0x0046_0140, super::refs::at_10(e, held)).addr();
    typed_call!(FAMILY, 0x0043_b5f0, unlock_loaded(e, held));
    value
}

// Typed from 0043b5b0
/// The entry's lock (`0043b5d0`) taken: `004534f0` with 1, which does nothing.
pub fn lock_loaded(e: &mut Engine, this: Ptr) {
    let held = typed_call!(FAMILY, 0x0043_b5d0, loaded_lock(e, this));
    typed_call!(
        FAMILY,
        0x0045_34f0,
        super::refs::nothing_2(e, held.addr(), 1)
    );
}

// Typed from 0043b5d0
/// The entry's lock: its address `+0x6c`.
pub fn loaded_lock(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x6c)
}

// Typed from 0043b5f0
/// The entry's lock (`0043b5d0`) given back: `00483710`, which does nothing.
pub fn unlock_loaded(e: &mut Engine, this: Ptr) {
    let held = typed_call!(FAMILY, 0x0043_b5d0, loaded_lock(e, this));
    typed_call!(FAMILY, 0x0048_3710, super::refs::nothing_1(e, held.addr()));
}

// Typed from 0043b610
/// `bhkCollisionObject::GetbhkCollisionObject` (`cdecl(object)`): the object's collision object
/// (`006838b0`) if its RTTI is `bhkCollisionObject`'s or derives from it (`00653270`).
pub fn collision_object(e: &mut Engine, object: Ptr<NiAVObject>) -> Ptr {
    let collision = typed_call!(FAMILY, 0x0068_38b0, collision_word(e, object.cast()));
    Ptr::new(typed_call!(
        FAMILY,
        0x0065_3270,
        cast_if(e, BHK_COLLISION_OBJECT, collision)
    ))
}

// Typed from 006838b0
/// The object's collision object: the word at `+0x1c`.
pub fn collision_word(e: &mut Engine, this: Ptr) -> u32 {
    let at = this.addr().wrapping_add(0x1c);
    typed_call!(leaf::FAMILY, 0x0055_9450, leaf::word_at_0(e, at))
}

// Typed from 00653270
/// `NiDynamicCast` (`cdecl(rtti, object)`): `object` when not null and of the class (`00653290`).
pub fn cast_if(e: &mut Engine, rtti: u32, object: u32) -> u32 {
    if object == 0 {
        return 0;
    }
    typed_call!(FAMILY, 0x0065_3290, cast(e, Ptr::new(object), rtti)).addr()
}

// Typed from 00653290
/// `this` when it is of the class `rtti` (`006532c0`), else null.
pub fn cast(e: &mut Engine, this: Ptr, rtti: u32) -> Ptr {
    if typed_call!(FAMILY, 0x0065_32c0, is_kind_of(e, this, rtti)) {
        this
    } else {
        Ptr::NULL
    }
}

// Typed from 006532c0
/// `NiObject::IsKindOf` (`thiscall(this, rtti)`): the object's RTTI (vtable +8) and its bases (the word
/// at +4 of each) walked for `rtti`.
pub fn is_kind_of(e: &mut Engine, this: Ptr, wanted: u32) -> bool {
    let mut link = e.vcall(this.addr(), 8, &args![]).u32();
    while link != 0 {
        if link == wanted {
            return true;
        }
        link = typed_call!(leaf::FAMILY, 0x0072_6070, leaf::word_at_4(e, link));
    }
    false
}

// Typed from 00453470
/// The number of set entries of the node's children array (`+0x9c`, `009938b0`).
pub fn esize_of_children(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(typed_call!(
        FAMILY,
        0x0099_38b0,
        esize(e, this.byte_add(0x9c))
    ))
}

// Typed from 009938b0
/// `NiTArray::GetEffectiveSize` (`thiscall(this)`): the number of non-null entries (`+0xc`).
pub fn esize(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr().wrapping_add(0xc))
}

// Typed from 004b5fc0
/// Whether an object's world bound (`0043d450`) is in the camera's frustum (`004b5ff0`); false for a
/// null object.
pub fn bound_in_frustum(e: &mut Engine, object: Ptr<NiAVObject>, camera: Ptr) -> bool {
    if object.is_null() {
        return false;
    }
    let bound = typed_call!(leaf::FAMILY, 0x0043_d450, leaf::world_bound(e, object));
    typed_call!(FAMILY, 0x004b_5ff0, in_frustum(e, Ptr::new(bound), camera))
}

// Typed from 004b5ff0
/// Whether a bound is inside the camera's frustum (`cdecl(bound, camera)`): the planes kept at
/// `011c6330` (constructed once) are set again from the camera (`00a755d0`) unless they were made for
/// it and its stamp (`00825c00`) is unchanged; then the bound against each of the six (`004b4cf0`, a
/// copy of the plane, `004b6100`): false at the first it is wholly outside of (2). False for a null
/// camera.
pub fn in_frustum(e: &mut Engine, bound: Ptr, camera: Ptr) -> bool {
    if camera.is_null() {
        return false;
    }
    let guard: u32 = e.global(FRUSTUM_GUARD);
    if guard & 1 == 0 {
        e.set_global(FRUSTUM_GUARD, guard | 1);
        e.call(FRUSTUM_PLANES_CONSTRUCT, &args![FRUSTUM_PLANES]);
    }
    let mut current = false;
    if e.global::<u32>(FRUSTUM_CAMERA) == camera.addr() {
        let stamp = e.call(CAMERA_STAMP, &args![CAMERA_STAMP_OWNER]).u32();
        current = e.global::<u32>(FRUSTUM_STAMP) == stamp;
    }
    if !current {
        e.call(FRUSTUM_PLANES_SET, &args![FRUSTUM_PLANES, camera]);
        let stamp = e.call(CAMERA_STAMP, &args![CAMERA_STAMP_OWNER]).u32();
        e.set_global(FRUSTUM_STAMP, stamp);
        e.set_global(FRUSTUM_CAMERA, camera.addr());
    }
    for index in 0..6u32 {
        let plane = typed_call!(
            FAMILY,
            0x004b_4cf0,
            plane(e, Ptr::new(FRUSTUM_PLANES), index)
        );
        let side = e.with_stack(16, |e, copy| {
            copy_words(e, plane.addr(), copy.addr(), 4);
            typed_call!(
                culling::FAMILY,
                0x004b_6100,
                culling::bound::which_side(e, bound.cast(), copy.cast::<NiPlane>())
            )
        });
        if side == 2 {
            return false;
        }
    }
    true
}

// Typed from 004b4cf0
/// `NiFrustumPlanes::GetPlane` (`thiscall(this, index)`): the plane's address (16 bytes each).
pub fn plane(_e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    Ptr::new((index << 4).wrapping_add(this.addr()))
}

// Typed from 00620b80
/// The word at +8 (`0044ddc0`): a counted holder's object, a character controller's proxy.
pub fn word_8(e: &mut Engine, this: u32) -> u32 {
    typed_call!(leaf::FAMILY, 0x0044_ddc0, leaf::word_at_8(e, this))
}

// Typed from 004ae750
/// The proxy of a character controller (`00620b80`), or 0 for none.
pub fn character_proxy(e: &mut Engine, this: Ptr) -> u32 {
    if this.is_null() {
        return 0;
    }
    typed_call!(FAMILY, 0x0062_0b80, word_8(e, this.addr()))
}

// Typed from 00560e10
/// A reference's body vector (`thiscall(this)`): with a character proxy (`004ae750`) its part's
/// (`00560e50`), otherwise the shared zero vector (`00458b20`).
pub fn body_vector(e: &mut Engine, this: Ptr) -> Ptr {
    let entity = typed_call!(FAMILY, 0x004a_e750, character_proxy(e, this));
    if entity == 0 {
        e.call(ZERO_VECTOR, &args![]).ptr()
    } else {
        typed_call!(FAMILY, 0x0056_0e50, body_vector_of(e, Ptr::new(entity)))
    }
}

// Typed from 00560e50
/// The part at `+0xe0` (`009d9f40`) of the part at `+0xe0` of `this`.
pub fn body_vector_of(e: &mut Engine, this: Ptr) -> Ptr {
    let part = typed_call!(FAMILY, 0x009d_9f40, part_e0(e, this));
    typed_call!(FAMILY, 0x009d_9f40, part_e0(e, part))
}

// Typed from 009d9f40
/// The address `this + 0xe0`.
pub fn part_e0(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0xe0)
}
