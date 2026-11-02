//! `Havok SDK/hkpconvexshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpConvexShape` fields used here: the radius (`float`) at +0x10. Virtual slots called on
//! the shape itself: `+0x1c` (`getAabb`: `(transform, tolerance, out aabb)`), `+0x20` (the ray
//! cast: `(out hit flag, input, out output)`) and `+0x38` (the supporting vertex:
//! `(direction, out vertex)`); the slot numbers come from the calls, not from a vtable dump.

#[allow(unused_imports)]
use crate::prelude::*;

type Lanes = [f32; 4];

/// The identity transform global (`01268370`, 64 zero bytes in the exe) that `getCentreImpl`
/// hands to `getAabb`.
const GETAABB_TRANSFORM: u32 = 0x0126_8370;
/// The `float` `0.5` the centre is scaled by (`01016248`).
const HALF: u32 = 0x0101_6248;
/// `hkReferencedObject::addReference` (`00c90510`, `this` only).
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// The vtable of `hkpConvexTransformShapeBase` (RTTI `.?AVhkpConvexTransformShapeBase@@`
/// confirmed) and of its embedded `hkpSingleShapeContainer` (RTTI confirmed) at +0x14.
const TRANSFORM_SHAPE_VTABLE: u32 = 0x010c_710c;
const SINGLE_SHAPE_CONTAINER_VTABLE: u32 = 0x010c_6fd0;

fn load_lanes(e: &Engine, addr: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_lanes(e: &mut Engine, addr: u32, v: Lanes) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, *value);
    }
}

// Translated from 00c9e1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexShape::weldContactPointImpl` (Xbox PDB): ignores its seven arguments and
/// returns `2`.
#[allow(clippy::too_many_arguments)]
pub fn weld_contact_point_impl(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
) -> u32 {
    2
}

// Translated from 00c9e1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexShape::getMaximumProjection` (Xbox PDB): asks the shape for its supporting
/// vertex in `direction` (slot `0x38`) and returns `radius + dot`, where the dot product is
/// the compiler's `z + (y + x)` of the lane products and the sum is made in `double`
/// precision and rounded to `float`.
pub fn hkp_convex_shape_get_maximum_projection(e: &mut Engine, this: Ptr, direction: Ptr) -> f32 {
    let vertex = e.with_stack(0x10, |e, out| {
        e.vcall(this.addr(), 0x38, &args![direction, out]);
        load_lanes(e, out.addr())
    });
    let d = load_lanes(e, direction.addr());
    let product: Lanes = std::array::from_fn(|lane| d[lane] * vertex[lane]);
    let dot = product[2] + (product[1] + product[0]);
    let radius = x87_quiet(e.mem.f32(this.addr() + 0x10));
    x87_quiet((f64::from(radius) + f64::from(dot)) as f32)
}

// Translated from 00c9e250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexShape::getCentreImpl` (Xbox PDB): the shape's box through slot `0x1c`
/// (`getAabb(identity transform, tolerance 0.0, out)`), then `out = (max + min) * 0.5` in all
/// four lanes (`0.5` read from `01016248`).
pub fn hkp_convex_shape_get_centre_impl(e: &mut Engine, this: Ptr, out: Ptr) {
    let (low, high) = e.with_stack(0x20, |e, aabb| {
        e.vcall(this.addr(), 0x1c, &args![GETAABB_TRANSFORM, 0.0f32, aabb]);
        (
            load_lanes(e, aabb.addr()),
            load_lanes(e, aabb.addr() + 0x10),
        )
    });
    let half = e.mem.f32(HALF);
    let sum: Lanes = std::array::from_fn(|lane| high[lane] + low[lane]);
    store_lanes(e, out.addr(), sum.map(|v| half * v));
}

// Translated from 00c9e2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexShape::castRayWithCollectorImpl` (Xbox PDB): builds an `hkpShapeRayCastOutput`
/// on the stack (hit fraction = the collector's float at +4, a `-1` at +0x14 and +0x20, a
/// zero at +0x40), casts through slot `0x20`, and on a hit rotates the output normal by the
/// 3x3 part of the transform at `cd_body + 8` (`y * c1 + x * c0 + z * c2`, four lanes) and
/// calls the collector's slot 0 with `(cd_body, output)`.
pub fn hkp_convex_shape_cast_ray_with_collector_impl(
    e: &mut Engine,
    this: Ptr,
    input: u32,
    cd_body: Ptr,
    collector: Ptr,
) {
    e.with_stack(0x60, |e, base| {
        let hit_flag = base.addr() + 0xf;
        let output = base.addr() + 0x10;
        let fraction_bits = x87_quiet_bits(e.mem.u32(collector.addr() + 4));
        e.mem.set_u32(output + 0x10, fraction_bits);
        e.mem.set_u32(output + 0x14, 0xffff_ffff);
        e.mem.set_u32(output + 0x20, 0xffff_ffff);
        e.mem.set_u32(output + 0x40, 0);
        e.vcall(this.addr(), 0x20, &args![hit_flag, input, output]);
        if e.mem.u8(hit_flag) != 0 {
            let normal = load_lanes(e, output);
            let transform = e.mem.u32(cd_body.addr() + 8);
            let (c0, c1, c2) = (
                load_lanes(e, transform),
                load_lanes(e, transform + 0x10),
                load_lanes(e, transform + 0x20),
            );
            let rotated: Lanes = std::array::from_fn(|lane| {
                (normal[1] * c1[lane] + normal[0] * c0[lane]) + normal[2] * c2[lane]
            });
            store_lanes(e, output, rotated);
            e.vcall(collector.addr(), 0, &args![cd_body, output]);
        }
    });
}

// Translated from 00c9e340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTransformShapeBase::hkpConvexTransformShapeBase` (Xbox PDB; the vtable it
/// stores is RTTI-confirmed): `+0xC = first`, `+0x10 = second` (a `float`, copied through
/// the x87), `+6 = 1` (word), `+8 = 0`, the vtable, the embedded single-shape container's
/// vtable at +0x14 and `+0x18 = child`; when `add_reference` is `1` the child gets
/// `addReference` (`00c90510`). Returns `this`.
pub fn hkp_convex_transform_shape_base_ctor(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: f32,
    child: Ptr,
    add_reference: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0x10, x87_quiet(second).to_bits());
    e.mem.set_u32(base + 0xc, first);
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base, TRANSFORM_SHAPE_VTABLE);
    e.mem.set_u32(base + 0x14, SINGLE_SHAPE_CONTAINER_VTABLE);
    e.mem.set_u32(base + 0x18, child.addr());
    if add_reference == 1 {
        e.call(ADD_REFERENCE, &args![child]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c9_e1e0,
            weld_contact_point_impl(Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00c9_e1f0,
            hkp_convex_shape_get_maximum_projection(Ptr, Ptr) -> f32
        ),
        entry!(0x00c9_e250, hkp_convex_shape_get_centre_impl(Ptr, Ptr)),
        entry!(
            0x00c9_e2a0,
            hkp_convex_shape_cast_ray_with_collector_impl(Ptr, u32, Ptr, Ptr)
        ),
        entry!(
            0x00c9_e340,
            hkp_convex_transform_shape_base_ctor(Ptr, u32, f32, Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const VTABLE: u32 = 0x0a00_0000;
    const SLOT_AABB: u32 = 0x0a10_0001;
    const SLOT_CAST: u32 = 0x0a10_0002;
    const SLOT_SUPPORT: u32 = 0x0a10_0003;
    const COLLECTOR_VTABLE: u32 = 0x0a00_1000;
    const COLLECTOR_ADD: u32 = 0x0a10_0004;

    fn shape(e: &mut Engine) -> u32 {
        let mut slots = vec![0u32; 0x40 / 4];
        slots[0x1c / 4] = SLOT_AABB;
        slots[0x20 / 4] = SLOT_CAST;
        slots[0x38 / 4] = SLOT_SUPPORT;
        e.put_vtable(VTABLE, &slots);
        let shape = e.mem.alloc(0x40);
        e.mem.set_u32(shape, VTABLE);
        shape
    }

    #[test]
    fn weld_contact_point_returns_two() {
        let mut e = Engine::new();
        let args = [0u32; 8];
        assert_eq!(e.call(0x00c9_e1e0, &args).u32(), 2);
    }

    #[test]
    fn maximum_projection_adds_the_radius_to_the_dot_product() {
        let mut e = Engine::new();
        let this = shape(&mut e);
        e.mem.set_f32(this + 0x10, 0.5);
        let direction = e.mem.alloc(0x10);
        store_lanes(&mut e, direction, [1.0, 2.0, 3.0, 0.0]);
        let seen = Rc::new(RefCell::new(vec![]));
        let log = Rc::clone(&seen);
        e.register_double(SLOT_SUPPORT, move |e, a| {
            log.borrow_mut().extend_from_slice(a);
            store_lanes(e, a[2], [4.0, 5.0, 6.0, 7.0]);
            Ret::default()
        });
        let got = e.call(0x00c9_e1f0, &args![this, direction]).f32();
        // products (4, 10, 18): 18 + (10 + 4) = 32, plus the radius
        assert_eq!(got, 32.5);
        assert_eq!(seen.borrow()[..2], [this, direction]);
    }

    #[test]
    fn centre_is_half_the_sum_of_the_box_corners() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.mem.set_f32(HALF, 0.5);
        let this = shape(&mut e);
        let seen = Rc::new(RefCell::new(vec![]));
        let log = Rc::clone(&seen);
        e.register_double(SLOT_AABB, move |e, a| {
            log.borrow_mut().extend_from_slice(a);
            store_lanes(e, a[3], [1.0, 2.0, 3.0, 4.0]);
            store_lanes(e, a[3] + 0x10, [3.0, 4.0, 5.0, 6.0]);
            Ret::default()
        });
        let out = e.mem.alloc(0x10);
        e.call(0x00c9_e250, &args![this, out]);
        assert_eq!(load_lanes(&e, out), [2.0, 3.0, 4.0, 5.0]);
        let seen = seen.borrow();
        assert_eq!(seen[..3], [this, GETAABB_TRANSFORM, 0]);
    }

    fn cast_rig(e: &mut Engine) -> (u32, u32, u32, u32) {
        let this = shape(e);
        e.put_vtable(COLLECTOR_VTABLE, &[COLLECTOR_ADD]);
        let collector = e.mem.alloc(0x10);
        e.mem.set_u32(collector, COLLECTOR_VTABLE);
        e.mem.set_f32(collector + 4, 0.75);
        let matrix = e.mem.alloc(0x30);
        store_lanes(e, matrix, [1.0, 0.0, 0.0, 0.0]);
        store_lanes(e, matrix + 0x10, [0.0, 0.0, 2.0, 0.0]);
        store_lanes(e, matrix + 0x20, [0.0, 3.0, 0.0, 0.0]);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body + 8, matrix);
        (this, collector, body, matrix)
    }

    #[test]
    fn cast_ray_rotates_the_normal_and_reports_a_hit() {
        let mut e = Engine::new();
        let (this, collector, body, _matrix) = cast_rig(&mut e);
        let seen = Rc::new(RefCell::new(vec![]));
        let log = Rc::clone(&seen);
        e.register_double(SLOT_CAST, move |e, a| {
            // the output starts with the collector's fraction and the -1 markers
            let output = a[3];
            log.borrow_mut().extend([
                e.mem.u32(output + 0x10),
                e.mem.u32(output + 0x14),
                e.mem.u32(output + 0x20),
                e.mem.u32(output + 0x40),
                a[2],
            ]);
            store_lanes(e, output, [1.0, 10.0, 100.0, 0.0]);
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        let hit = Rc::new(RefCell::new(vec![]));
        let hit_log = Rc::clone(&hit);
        e.register_double(COLLECTOR_ADD, move |e, a| {
            hit_log.borrow_mut().extend_from_slice(a);
            hit_log
                .borrow_mut()
                .extend((0..4).map(|lane| e.mem.f32(a[2] + 4 * lane).to_bits()));
            Ret::default()
        });
        e.call(0x00c9_e2a0, &args![this, 0x1234u32, body, collector]);
        assert_eq!(
            *seen.borrow(),
            vec![0.75f32.to_bits(), 0xffff_ffff, 0xffff_ffff, 0, 0x1234]
        );
        let hit = hit.borrow();
        assert_eq!(hit[..2], [collector, body]);
        // x * c0 + y * c1 + z * c2 = (1, 0, 0) + 10 * (0, 0, 2) + 100 * (0, 3, 0)
        let rotated: Vec<f32> = hit[3..].iter().map(|b| f32::from_bits(*b)).collect();
        assert_eq!(rotated, vec![1.0, 300.0, 20.0, 0.0]);
    }

    #[test]
    fn cast_ray_without_a_hit_does_not_call_the_collector() {
        let mut e = Engine::new();
        let (this, collector, body, _matrix) = cast_rig(&mut e);
        e.register(SLOT_CAST, |_, _| Ret::default());
        e.register(COLLECTOR_ADD, |_, _| {
            panic!("collector called without a hit")
        });
        e.call(0x00c9_e2a0, &args![this, 0u32, body, collector]);
    }

    #[test]
    fn transform_shape_base_constructor_sets_fields_and_references_the_child() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let child = e.mem.alloc(0x20);
        let calls = Rc::new(RefCell::new(vec![]));
        let log = Rc::clone(&calls);
        e.register_double(ADD_REFERENCE, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        let got = e
            .call(
                0x00c9_e340,
                &args![this, 0x55u32, f32::from_bits(0x7f80_0001), child, 1u32],
            )
            .u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this + 0xc), 0x55);
        assert_eq!(e.mem.u32(this + 0x10), 0x7fc0_0001);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this), TRANSFORM_SHAPE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x14), SINGLE_SHAPE_CONTAINER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), child);
        assert_eq!(*calls.borrow(), vec![child]);
        // any other value than 1 leaves the count alone
        e.call(0x00c9_e340, &args![this, 0u32, 1.0f32, child, 0u32]);
        e.call(0x00c9_e340, &args![this, 0u32, 1.0f32, child, 2u32]);
        assert_eq!(calls.borrow().len(), 1);
    }
}
