//! `Havok SDK/hkpshapedisplaybuilder.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    load_bits, load_vec, store_bits, store_vec, zip_with, HALF_F32, ZERO_F32,
};

/// The memory router (`00c85750`): its first word is the stack-like scratch allocator's top and
/// its `+0x10` word is the allocator (virtual slot 4 allocates, slot `0x10` frees).
const ROUTER: u32 = 0x00c8_5750;
/// The array helpers (cdecl): grow by one element (`00c90940`: allocator, array, element size)
/// and reserve (`00c908c0`: allocator, array, capacity, element size).
const ARRAY_GROW: u32 = 0x00c9_0940;
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkVector4::setTransformedPos` (`00c7f640`, thiscall on the output vector: transform, point),
/// the display geometry's `setTransform` (`00c6cd60`, thiscall on `object + 0x10`: transform) and
/// `hkTransform::setMul` (`00cb2240`, thiscall on the output transform: a, b).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
const SET_TRANSFORM: u32 = 0x00c6_cd60;
const SET_MUL: u32 = 0x00cb_2240;
/// The display geometry constructors: the sphere (`00e0af50`: vector, then the two builder
/// words), the box (`00de1f50`), the capsule (`00d22dc0`: top, bottom, radius, 6, 1), the
/// cylinder (`00d58510`: top, bottom, radius), the box-shaped bounds (`00de4f20`: shape
/// words, axis, centre, half extents) and the convex (`00de1d20`: raw geometry); all
/// thiscall on the new object and answering it. `00d571a0` is `hkpCylinderShape::
/// getCylinderRadius` (thiscall on the shape, two words left on the stack, answers `st0`),
/// `00cce5e0` the raw geometry constructor and `00d5a5e0`
/// `hkGeometryUtility::createConvexGeometry` (cdecl: input record, geometry).
const SPHERE_CONSTRUCTOR: u32 = 0x00e0_af50;
const BOX_CONSTRUCTOR: u32 = 0x00de_1f50;
const CAPSULE_CONSTRUCTOR: u32 = 0x00d2_2dc0;
const CYLINDER_CONSTRUCTOR: u32 = 0x00d5_8510;
const BOUNDS_CONSTRUCTOR: u32 = 0x00de_4f20;
const CONVEX_CONSTRUCTOR: u32 = 0x00de_1d20;
const CYLINDER_RADIUS: u32 = 0x00d5_71a0;
const RAW_GEOMETRY_CONSTRUCTOR: u32 = 0x00cc_e5e0;
const CREATE_CONVEX_GEOMETRY: u32 = 0x00d5_a5e0;
/// The table of display builders for shape types this function does not know: a count at `+0xC`
/// and, at `+8`, 8-byte entries `(function, shape type)` (function cdecl: shape, transform,
/// array, builder).
const EXTRA_BUILDERS: u32 = 0x0126_9ce0;
/// The key that ends a walk of a shape container.
const NO_KEY: u32 = 0xffff_ffff;
/// `float` constants: 0.01 (`01013ea4`), the bounds tolerance (`01017d00`), the mask that clears
/// the sign (`01017d10`), the identity transform record passed to the bounds query (`01268370`),
/// the vector whose last lane the positioned spheres use (`01267e30`) and the doubles 0.0
/// (`01012060`) and 1.0 (`01012070`) the height field adds; 0 and 0.5 are the shared constants.
const SMALL_FLOAT: u32 = 0x0101_3ea4;
const BOUNDS_TOLERANCE: u32 = 0x0101_7d00;
const ABSOLUTE_MASK: u32 = 0x0101_7d10;
const IDENTITY_TRANSFORM: u32 = 0x0126_8370;
const POSITION_TAIL: u32 = 0x0126_7e30;
const DOUBLE_ZERO: u32 = 0x0101_2060;
const DOUBLE_ONE: u32 = 0x0101_2070;

/// The allocator (`+0x10`) of the memory router.
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// A new display object of `size` bytes: the allocator hands out the block (no failure check)
/// and the 16-bit word at `+4` becomes `size`.
fn allocate_object(e: &mut Engine, size: u32) -> u32 {
    let allocator = router_allocator(e);
    let object = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(object + 4, size as u16);
    object
}

/// Appends `value` to the `hkArray` of pointers at `array` (data `+0`, size `+4`, capacity and
/// flags `+8`); the router is asked first and a full array grows by one element.
fn push_back(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data + 4 * size, value);
    e.mem.set_u32(array + 4, size + 1);
}

/// The reserve the exe has inlined for the arrays of 16-byte elements: the router is asked and
/// the array is reserved to the larger of `need` and twice its capacity when it holds fewer.
fn reserve_to(e: &mut Engine, array: u32, need: i32) {
    let allocator = router_allocator(e);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < need {
        let doubled = capacity.wrapping_add(capacity);
        let target = if need >= doubled { need } else { doubled };
        e.call(ARRAY_RESERVE, &args![allocator, array, target, 0x10u32]);
    }
}

/// Makes room for `more` elements at the end of the array of 16-byte elements at `array` and
/// answers its size before.
fn reserve_more(e: &mut Engine, array: u32, more: i32) -> u32 {
    let size = e.mem.u32(array + 4);
    reserve_to(e, array, (size as i32).wrapping_add(more));
    size
}

/// The address of element `index` of the array of 16-byte elements at `array`.
fn element(e: &Engine, array: u32, index: u32) -> u32 {
    e.mem.u32(array) + 16 * index
}

/// Writes the words of a triangle record (three indices and the terminator -1).
fn set_triangle(e: &mut Engine, at: u32, first: u32, second: u32, third: u32) {
    e.mem.set_u32(at, first);
    e.mem.set_u32(at + 4, second);
    e.mem.set_u32(at + 8, third);
    e.mem.set_u32(at + 0xc, NO_KEY);
}

// Translated from 00ddfb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeDisplayBuilder::hkpShapeDisplayBuilderEnvironment::
/// hkpShapeDisplayBuilderEnvironment` (Xbox PDB), `__fastcall`: both words of the record
/// (`+0` and `+4`) become 8. Answers `this`.
pub fn hkp_shape_display_builder_environment_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 8);
    e.mem.set_u32(this.addr() + 4, 8);
    this
}

// Translated from 00ddfb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeDisplayBuilder::hkpShapeDisplayBuilder` (Xbox PDB), thiscall with one stack word
/// (`RET 4`): the environment record. Sets the 16-bit word at `+6` to 1, the vtable, the two
/// words at `+8` and `+0xC` from the environment and the raw geometry at `+0x10` to none.
/// Answers `this`.
pub fn hkp_shape_display_builder_constructor(e: &mut Engine, this: Ptr, environment: Ptr) -> Ptr {
    let at = this.addr();
    e.mem.set_u16(at + 6, 1);
    e.mem.set_u32(at, 0x010d_8094);
    let first = e.mem.u32(environment.addr());
    e.mem.set_u32(at + 8, first);
    let second = e.mem.u32(environment.addr() + 4);
    e.mem.set_u32(at + 0xc, second);
    e.mem.set_u32(at + 0x10, 0);
    this
}

// Translated from 00ddfbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `__fastcall`: forgets the raw geometry of the builder (the word at `+0x10`).
pub fn fn_00ddfbb0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x10, 0);
}

// Translated from 00ddfc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeDisplayBuilder::getCurrentRawGeometry` (Xbox PDB), thiscall with one stack word
/// (`RET 4`): the array of display geometries. Without a raw geometry (`+0x10`) one is made: an
/// 0x18-byte record with two empty arrays (the allocation is checked, a failed one gives no
/// record), a display convex (0x60 bytes, no check) is built on it and appended to `array`.
/// Answers the display convex.
pub fn hkp_shape_display_builder_get_current_raw_geometry(
    e: &mut Engine,
    this: Ptr,
    array: Ptr,
) -> u32 {
    let this = this.addr();
    let array = array.addr();
    if e.mem.u32(this + 0x10) == 0 {
        let allocator = router_allocator(e);
        let allocated = e.vcall(allocator, 4, &args![0x18u32]).u32();
        let mut record = 0;
        if allocated != 0 {
            e.mem.set_u32(allocated, 0);
            e.mem.set_u32(allocated + 4, 0);
            e.mem.set_u32(allocated + 8, 0x8000_0000);
            e.mem.set_u32(allocated + 0xc, 0);
            e.mem.set_u32(allocated + 0x10, 0);
            e.mem.set_u32(allocated + 0x14, 0x8000_0000);
            record = allocated;
        }
        let convex = allocate_object(e, 0x60);
        let made = e.call(CONVEX_CONSTRUCTOR, &args![convex, record]).u32();
        e.mem.set_u32(this + 0x10, made);
        let stored = e.mem.u32(this + 0x10);
        push_back(e, array, stored);
    }
    e.mem.u32(this + 0x10)
}

// Translated from 00de0f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeDisplayBuilder::buildDisplayGeometries` (Xbox PDB), thiscall with two stack words
/// (`RET 8`): the `shape` and the array of display geometries. Forgets the raw geometry,
/// empties the array and builds the shape with the identity transform (three unit rows and a
/// zero translation, kept on the stack).
pub fn hkp_shape_display_builder_build_display_geometries(
    e: &mut Engine,
    this: Ptr,
    shape: Ptr,
    array: Ptr,
) {
    e.with_stack(0x40, |e, matrix| {
        let m = matrix.addr();
        for row in 0..4 {
            store_bits(e, m + 0x10 * row, [0; 4]);
        }
        e.mem.set_u32(m, 1.0f32.to_bits());
        e.mem.set_u32(m + 0x14, 1.0f32.to_bits());
        e.mem.set_u32(m + 0x28, 1.0f32.to_bits());
        fn_00ddfbb0(e, this);
        e.mem.set_u32(array.addr() + 4, 0);
        hkp_shape_display_builder_build_shape_display(e, this, shape, matrix, array);
    });
}

/// The arguments of one build and the 0x360-byte stack frame the exe keeps for it.
#[derive(Clone, Copy)]
struct Build {
    this: u32,
    shape: u32,
    transform: u32,
    array: u32,
    frame: u32,
}

/// The tail the box-like cases share: the transform goes to the display geometry (`+0x10`) and
/// the geometry is appended to the array.
fn finish_object(e: &mut Engine, b: Build, object: u32) {
    e.call(SET_TRANSFORM, &args![object + 0x10, b.transform]);
    push_back(e, b.array, object);
}

/// Builds the shape `child` with `transform` into the same array.
fn build_child(e: &mut Engine, b: Build, child: u32, transform: u32) {
    hkp_shape_display_builder_build_shape_display(
        e,
        Ptr::new(b.this),
        Ptr::new(child),
        Ptr::new(transform),
        Ptr::new(b.array),
    );
}

/// The two words of the builder environment the sphere constructor wants, last one first.
fn environment_words(e: &Engine, b: Build) -> (u32, u32) {
    (e.mem.u32(b.this + 0xc), e.mem.u32(b.this + 8))
}

/// Sphere (type 1): a display sphere whose vector is `(0, 0, 0, radius)`.
fn build_sphere(e: &mut Engine, b: Build) {
    let s = b.frame;
    store_bits(e, s + 0x40, [0; 4]);
    let radius = x87_quiet_bits(e.mem.u32(b.shape + 0x10));
    e.mem.set_u32(s + 0x4c, radius);
    let object = allocate_object(e, 0x80);
    let (second, first) = environment_words(e, b);
    let made = e
        .call(SPHERE_CONSTRUCTOR, &args![object, s + 0x40, second, first])
        .u32();
    finish_object(e, b, made);
}

/// Cylinder (type 2): the two end points go through the transform, the radius is asked of the
/// shape (the helper leaves its two argument words on the stack).
fn build_cylinder(e: &mut Engine, b: Build) {
    let s = b.frame;
    e.call(
        SET_TRANSFORMED_POS,
        &args![s + 0xc0, b.transform, b.shape + 0x30],
    );
    e.call(
        SET_TRANSFORMED_POS,
        &args![s + 0xa0, b.transform, b.shape + 0x20],
    );
    let object = allocate_object(e, 0x90);
    // 00de0051: `00d571a0` takes no stack word (plain RET); the 9 and 1 pushed before it
    // are the last two words of the cylinder constructor (`00d58510`, RET 0x14).
    let radius = e.call(CYLINDER_RADIUS, &args![b.shape]).st0;
    let made = e
        .call(
            CYLINDER_CONSTRUCTOR,
            &args![
                object,
                s + 0xc0,
                s + 0xa0,
                (radius as f32).to_bits(),
                9u32,
                1u32
            ],
        )
        .u32();
    finish_object(e, b, made);
}

/// Triangle (type 3): three vertices with a face, and, when the extrusion vector at `+0x50` has
/// a length above 0, the three offset vertices, two faces on top and four on the sides.
fn build_triangle(e: &mut Engine, b: Build) {
    let s = b.frame;
    let current =
        hkp_shape_display_builder_get_current_raw_geometry(e, Ptr::new(b.this), Ptr::new(b.array));
    let raw = e.mem.u32(current + 8);
    let base = e.mem.u32(raw + 4);
    e.mem.set_u32(s + 0x2c, base);
    let first = reserve_more(e, raw, 3);
    e.mem.set_u32(raw + 4, first + 3);
    let slot = element(e, raw, first);
    e.call(
        SET_TRANSFORMED_POS,
        &args![slot, b.transform, b.shape + 0x20],
    );
    e.call(
        SET_TRANSFORMED_POS,
        &args![slot + 0x10, b.transform, b.shape + 0x30],
    );
    e.call(
        SET_TRANSFORMED_POS,
        &args![slot + 0x20, b.transform, b.shape + 0x40],
    );
    let faces = raw + 0xc;
    let face = reserve_more(e, faces, 1);
    e.mem.set_u32(faces + 4, face + 1);
    let record = element(e, faces, face);
    set_triangle(e, record, base, base + 1, base + 2);
    let extrusion = load_vec(e, b.shape + 0x50);
    let squares = zip_with(extrusion, extrusion, |x, y| x * y);
    let sum = squares[2] + (squares[1] + squares[0]);
    let length = sum.sqrt();
    e.mem.set_u32(s + 0xc, length.to_bits());
    if 0.0 < length {
        let extrusion = load_vec(e, b.shape + 0x50);
        let first_point = load_vec(e, b.shape + 0x20);
        let second_point = load_vec(e, b.shape + 0x30);
        let third_point = load_vec(e, b.shape + 0x40);
        store_vec(e, s + 0xa0, zip_with(extrusion, first_point, |x, y| x + y));
        store_vec(e, s + 0xc0, zip_with(second_point, extrusion, |x, y| x + y));
        store_vec(e, s + 0x40, zip_with(third_point, extrusion, |x, y| x + y));
        let first = reserve_more(e, raw, 3);
        e.mem.set_u32(raw + 4, first + 3);
        let slot = element(e, raw, first);
        e.call(SET_TRANSFORMED_POS, &args![slot, b.transform, s + 0xa0]);
        e.call(
            SET_TRANSFORMED_POS,
            &args![slot + 0x10, b.transform, s + 0xc0],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![slot + 0x20, b.transform, s + 0x40],
        );
        let face = reserve_more(e, faces, 7);
        e.mem.set_u32(faces + 4, face + 7);
        let record = element(e, faces, face);
        let sides = [
            [base + 3, base + 1, base],
            [base + 3, base + 4, base + 1],
            [base + 4, base + 5, base + 1],
            [base + 5, base + 2, base + 1],
            [base + 5, base, base + 2],
            [base + 5, base + 3, base],
            [base + 5, base + 4, base + 3],
        ];
        for (index, side) in sides.iter().enumerate() {
            set_triangle(e, record + 16 * index as u32, side[0], side[1], side[2]);
        }
    }
}

/// Box (type 4): a display box from the half extents at `+0x20`.
fn build_box(e: &mut Engine, b: Build) {
    let object = allocate_object(e, 0x70);
    let made = e
        .call(BOX_CONSTRUCTOR, &args![object, b.shape + 0x20])
        .u32();
    finish_object(e, b, made);
}

/// Capsule (type 5): a display capsule between the points at `+0x20` and `+0x30` with the radius
/// at `+0x10` (the radius also goes to a stack scratch word).
fn build_capsule(e: &mut Engine, b: Build) {
    let object = allocate_object(e, 0x90);
    let radius = x87_quiet_bits(e.mem.u32(b.shape + 0x10));
    e.mem.set_u32(b.frame + 0xc, radius);
    let made = e
        .call(
            CAPSULE_CONSTRUCTOR,
            &args![object, b.shape + 0x30, b.shape + 0x20, radius, 6u32, 1u32],
        )
        .u32();
    finish_object(e, b, made);
}

/// Convex vertices (type 6): the shape's vertex count (slot `0x2C`) is carved off the router's
/// scratch stack (rounded up to 128 bytes), the shape writes its vertices there (slot `0x30`),
/// the vertices are transformed into a heap array, a raw geometry is made from them
/// (`createConvexGeometry`), a display convex is built on it and the scratch is given back.
fn build_convex_vertices(e: &mut Engine, b: Build) {
    let s = b.frame;
    let count = e.vcall(b.shape, 0x2c, &args![]).u32();
    e.mem.set_u32(s + 0x40, 0);
    e.mem.set_u32(s + 0x44, 0);
    e.mem.set_u32(s + 0x48, 0x8000_0000);
    let router = e.call(ROUTER, &args![]).u32();
    let top = e.mem.u32(router);
    let rounded = ((count << 4).wrapping_add(0x7f)) & 0xffff_ff80;
    e.mem.set_u32(router, rounded.wrapping_add(top));
    e.mem.set_u32(s + 0x40, top);
    e.mem.set_u32(s + 0x48, count | 0x8000_0000);
    e.mem.set_u32(s + 0x4c, top);
    reserve_to(e, s + 0x40, count as i32);
    let scratch = e.mem.u32(s + 0x40);
    e.mem.set_u32(s + 0x44, count);
    let vertices = e.vcall(b.shape, 0x30, &args![scratch]).u32();
    e.mem.set_u32(s + 0xc, vertices);
    e.mem.set_u32(s + 0x80, 0);
    e.mem.set_u32(s + 0x84, 0);
    e.mem.set_u32(s + 0x88, 0x8000_0000);
    reserve_to(e, s + 0x80, count as i32);
    e.mem.set_u32(s + 0x84, count);
    if (count as i32) > 0 {
        let mut offset = 0;
        let mut left = count;
        loop {
            let source = e.mem.u32(s + 0xc);
            let target = e.mem.u32(s + 0x80);
            e.call(
                SET_TRANSFORMED_POS,
                &args![target + offset, b.transform, source + offset],
            );
            offset += 0x10;
            left = left.wrapping_sub(1);
            if left == 0 {
                break;
            }
        }
    }
    let allocator = router_allocator(e);
    let allocated = e.vcall(allocator, 4, &args![0x18u32]).u32();
    let mut record = 0;
    if allocated != 0 {
        e.mem.set_u32(allocated, 0);
        e.mem.set_u32(allocated + 4, 0);
        e.mem.set_u32(allocated + 8, 0x8000_0000);
        e.mem.set_u32(allocated + 0xc, 0);
        e.mem.set_u32(allocated + 0x10, 0);
        e.mem.set_u32(allocated + 0x14, 0x8000_0000);
        record = allocated;
    }
    let data = e.mem.u32(s + 0x80);
    let size = e.mem.u32(s + 0x84);
    e.mem.set_u32(s + 0xd4, data);
    e.mem.set_u32(s + 0xd8, size);
    e.mem.set_u32(s + 0xdc, 0x10);
    e.call(CREATE_CONVEX_GEOMETRY, &args![s + 0xd4, record]);
    let convex = allocate_object(e, 0x60);
    let made = e.call(CONVEX_CONSTRUCTOR, &args![convex, record]).u32();
    push_back(e, b.array, made);
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(s + 0x88);
    e.mem.set_u32(s + 0x84, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(s + 0x80);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 4]);
    }
    let saved = e.mem.u32(s + 0x4c);
    e.mem.set_u32(s + 0x80, 0);
    e.mem.set_u32(s + 0x88, 0x8000_0000);
    if saved == e.mem.u32(s + 0x40) {
        e.mem.set_u32(s + 0x44, 0);
    }
    let router = e.call(ROUTER, &args![]).u32();
    e.mem.set_u32(router, saved);
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(s + 0x48);
    e.mem.set_u32(s + 0x44, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(s + 0x40);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 4]);
    }
}

/// The types that wrap a shape container behind virtual slot `0x10` (types 7, 8, 10, 14, 16, 22
/// and 23): every child of the container, in key order (slot 8 first key, slot `0xC` next, slot
/// `0x14` child of a key with a 0x200-byte scratch buffer), is built with the same transform.
fn build_container_children(e: &mut Engine, b: Build) {
    let container = e.vcall(b.shape, 0x10, &args![]).u32();
    let mut key = e.vcall(container, 8, &args![]).u32();
    while key != NO_KEY {
        let child = e.vcall(container, 0x14, &args![key, b.frame + 0x160]).u32();
        build_child(e, b, child, b.transform);
        key = e.vcall(container, 0xc, &args![key]).u32();
    }
}

/// List shape (type 9): the container embedded at `+0x10` walks the keys; the child of a key is
/// the pointer in the 16-byte record of the array at `+0x18`.
fn build_list_children(e: &mut Engine, b: Build) {
    let container = b.shape + 0x10;
    let mut key = e.vcall(container, 8, &args![]).u32();
    while key != NO_KEY {
        let records = e.mem.u32(b.shape + 0x18);
        let child = e.mem.u32(records + 16 * key);
        build_child(e, b, child, b.transform);
        key = e.vcall(container, 0xc, &args![key]).u32();
    }
}

/// Transform shape (type 11): the local transform is the identity with the translation at
/// `+0x20`; it is multiplied with the transform and the child at `+0x18` is built with it.
fn build_translated_child(e: &mut Engine, b: Build) {
    let s = b.frame;
    for row in 0..3 {
        store_bits(e, s + 0xe0 + 0x10 * row, [0; 4]);
        e.mem.set_u32(s + 0xe0 + 0x14 * row, 1.0f32.to_bits());
    }
    let translation = load_bits(e, b.shape + 0x20);
    store_bits(e, s + 0x110, translation);
    e.call(SET_MUL, &args![s + 0x120, b.transform, s + 0xe0]);
    let child = e.mem.u32(b.shape + 0x18);
    build_child(e, b, child, s + 0x120);
}

/// The shapes that carry a transform (types 12 and 15): the transform at `offset` is multiplied
/// with `transform` and the child at `child_offset` is built with the product.
fn build_transformed_child(e: &mut Engine, b: Build, offset: u32, child_offset: u32) {
    let s = b.frame;
    e.call(SET_MUL, &args![s + 0x120, b.transform, b.shape + offset]);
    let child = e.mem.u32(b.shape + child_offset);
    build_child(e, b, child, s + 0x120);
}

/// Height field (type 13): for every cell of the grid (resolution at `+0x10` by `+0x14`, scale
/// vector at `+0x20`) the four corners `(x, height, z) * scale` (heights from slot `0x34`) are
/// transformed and appended to the raw geometry, followed by two faces whose diagonal depends on
/// the byte slot `0x38` answers.
fn build_height_field(e: &mut Engine, b: Build) {
    let s = b.frame;
    let current =
        hkp_shape_display_builder_get_current_raw_geometry(e, Ptr::new(b.this), Ptr::new(b.array));
    let raw = e.mem.u32(current + 8);
    e.mem.set_u32(s + 0x10, raw);
    let scale = load_bits(e, b.shape + 0x20);
    store_bits(e, s + 0x40, scale);
    e.mem.set_u32(s + 0x28, 0);
    if e.mem.i32(b.shape + 0x10) - 1 <= 0 {
        return;
    }
    let one: f64 = e.global(DOUBLE_ONE);
    let zero: f64 = e.global(DOUBLE_ZERO);
    let zero_float = f32::from_bits(e.global::<u32>(ZERO_F32));
    loop {
        let x = e.mem.i32(s + 0x28);
        e.mem.set_u32(s + 0xc, 0);
        if e.mem.i32(b.shape + 0x14) - 1 > 0 {
            let faces = raw + 0xc;
            let x_float = x as f32;
            let x_low = (f64::from(x_float) + zero) as f32;
            let x_high = (f64::from(x_float) + one) as f32;
            e.mem.set_u32(s + 0x2c, x_low.to_bits());
            e.mem.set_u32(s + 0x90, x_high.to_bits());
            loop {
                let z = e.mem.i32(s + 0xc);
                let height_00 = e.vcall(b.shape, 0x34, &args![x, z]).f32();
                let z_float = z as f32;
                let z_low = (f64::from(z_float) + zero) as f32;
                let height_01 = e.vcall(b.shape, 0x34, &args![x, z + 1]).f32();
                let z_high = (f64::from(z_float) + one) as f32;
                let height_10 = e.vcall(b.shape, 0x34, &args![x + 1, z]).f32();
                let height_11 = e.vcall(b.shape, 0x34, &args![x + 1, z + 1]).f32();
                e.mem.set_i32(s + 0xc, z + 1);
                let scale = load_vec(e, s + 0x40);
                let corner = |left: f32, height: f32, depth: f32| {
                    zip_with([left, height, depth, zero_float], scale, |point, factor| {
                        point * factor
                    })
                };
                store_vec(e, s + 0xa0, corner(x_low, height_00, z_low));
                store_vec(e, s + 0xc0, corner(x_low, height_01, z_high));
                store_vec(e, s + 0x80, corner(x_high, height_10, z_low));
                store_vec(e, s + 0xe0, corner(x_high, height_11, z_high));
                let base = e.mem.u32(raw + 4);
                e.mem.set_u32(s + 0x30, base);
                let first = reserve_more(e, raw, 4);
                e.mem.set_u32(raw + 4, first + 4);
                let slot = element(e, raw, first);
                for (index, corner_at) in [0xa0, 0xc0, 0x80, 0xe0].iter().enumerate() {
                    e.call(
                        SET_TRANSFORMED_POS,
                        &args![slot + 0x10 * index as u32, b.transform, s + corner_at],
                    );
                }
                e.vcall(b.shape, 0x38, &args![s + 0x5f]);
                let flipped = e.mem.u8(s + 0x5f) != 0;
                let face = reserve_more(e, faces, 2);
                e.mem.set_u32(faces + 4, face + 2);
                let record = element(e, faces, face);
                if flipped {
                    set_triangle(e, record, base, base + 1, base + 3);
                    set_triangle(e, record + 0x10, base, base + 3, base + 2);
                } else {
                    set_triangle(e, record, base, base + 1, base + 2);
                    set_triangle(e, record + 0x10, base + 3, base + 2, base + 1);
                }
                if e.mem.i32(s + 0xc) >= e.mem.i32(b.shape + 0x14) - 1 {
                    break;
                }
            }
        }
        let next = e.mem.i32(s + 0x28) + 1;
        e.mem.set_i32(s + 0x28, next);
        if next >= e.mem.i32(b.shape + 0x10) - 1 {
            break;
        }
    }
}

/// Triangle list (type 20): the shape's children (container at `+0x20`, key array `+0x24`,
/// count `+0x28`) are triangles; every child adds a face and its three vertices to a new raw
/// geometry (record made by `00cce5e0`, face indices counted from the vertices already in
/// it), and a display convex is built on the raw geometry.
fn build_triangle_list(e: &mut Engine, b: Build) {
    let s = b.frame;
    let allocator = router_allocator(e);
    let allocated = e.vcall(allocator, 4, &args![0x18u32]).u32();
    let raw = if allocated != 0 {
        e.call(RAW_GEOMETRY_CONSTRUCTOR, &args![allocated]).u32()
    } else {
        0
    };
    e.mem.set_u32(s + 0x28, 0);
    if e.mem.i32(b.shape + 0x28) > 0 {
        loop {
            let index = e.mem.u32(s + 0x28);
            let container = e.mem.u32(b.shape + 0x20);
            let keys = e.mem.u32(b.shape + 0x24);
            let key = e.mem.u32(keys + 4 * index);
            let child = e
                .vcall(container + 0x10, 0x14, &args![key, s + 0x160])
                .u32();
            e.mem.set_u32(s + 0xc, child);
            let faces = raw + 0xc;
            let face = reserve_more(e, faces, 1);
            e.mem.set_u32(raw + 0x10, face + 1);
            let base = e.mem.u32(raw + 4);
            let record = element(e, faces, face);
            set_triangle(e, record, base, base + 1, base + 2);
            let mut point = child + 0x20;
            for _ in 0..3 {
                let vertex = reserve_more(e, raw, 1);
                e.mem.set_u32(raw + 4, vertex + 1);
                let slot = element(e, raw, vertex);
                e.call(SET_TRANSFORMED_POS, &args![slot, b.transform, point]);
                point += 0x10;
            }
            let next = index + 1;
            e.mem.set_u32(s + 0x28, next);
            if (next as i32) >= e.mem.i32(b.shape + 0x28) {
                break;
            }
        }
    }
    let convex = allocate_object(e, 0x60);
    let made = e.call(CONVEX_CONSTRUCTOR, &args![convex, raw]).u32();
    push_back(e, b.array, made);
}

/// Positioned boxes (type 21): one display sphere per entry (`+0x20`, 16 bytes each, count at
/// `+0x10`), with an identity orientation and the entry as its position.
fn build_positioned_spheres(e: &mut Engine, b: Build) {
    let s = b.frame;
    if e.mem.i32(b.shape + 0x10) <= 0 {
        return;
    }
    store_bits(e, s + 0x80, [0; 4]);
    let mut cursor = b.shape + 0x20;
    let mut index = 0;
    loop {
        let tail = load_bits(e, POSITION_TAIL);
        store_bits(e, s + 0x40, tail);
        let radius = x87_quiet_bits(e.mem.u32(cursor + 0xc));
        e.mem.set_u32(s + 0x4c, radius);
        let object = allocate_object(e, 0x80);
        let (second, first) = environment_words(e, b);
        let made = e
            .call(SPHERE_CONSTRUCTOR, &args![object, s + 0x40, second, first])
            .u32();
        let zeros = load_bits(e, s + 0x80);
        store_bits(e, made + 0x10, zeros);
        store_bits(e, made + 0x20, zeros);
        store_bits(e, made + 0x30, zeros);
        e.mem.set_u32(made + 0x10, 1.0f32.to_bits());
        e.mem.set_u32(made + 0x24, 1.0f32.to_bits());
        e.mem.set_u32(made + 0x38, 1.0f32.to_bits());
        store_bits(e, made + 0x40, zeros);
        let position = load_bits(e, cursor);
        store_bits(e, made + 0x40, position);
        push_back(e, b.array, made);
        cursor += 0x10;
        index += 1;
        if index >= e.mem.i32(b.shape + 0x10) {
            break;
        }
    }
}

/// Segments (type 24): every 32-byte entry of the array at `+0x10` (count at `+0x14`) is a pair
/// of points; each adds the transformed first point, the first point moved by `(0.01, 0, 0)`
/// (not transformed) and the transformed second point to the raw geometry, and one face.
fn build_segments(e: &mut Engine, b: Build) {
    let current =
        hkp_shape_display_builder_get_current_raw_geometry(e, Ptr::new(b.this), Ptr::new(b.array));
    let raw = e.mem.u32(current + 8);
    if e.mem.i32(b.shape + 0x14) <= 0 {
        return;
    }
    let zero = f32::from_bits(e.global::<u32>(ZERO_F32));
    let small = f32::from_bits(e.global::<u32>(SMALL_FLOAT));
    let offset = [small, zero, zero, zero];
    let faces = raw + 0xc;
    let mut index: u32 = 2;
    let mut byte_offset = 0;
    let mut item = 0;
    loop {
        let data = e.mem.u32(b.shape + 0x10) + byte_offset;
        let first_point = load_bits(e, data);
        let second_point = load_bits(e, data + 0x10);
        let at = reserve_more(e, raw, 1);
        e.mem.set_u32(raw + 4, at + 1);
        let slot = element(e, raw, at);
        store_bits(e, slot, first_point);
        e.call(SET_TRANSFORMED_POS, &args![slot, b.transform, slot]);
        let at = reserve_more(e, raw, 1);
        e.mem.set_u32(raw + 4, at + 1);
        let slot = element(e, raw, at);
        let moved = zip_with(first_point.map(f32::from_bits), offset, |x, y| x + y);
        store_vec(e, slot, moved);
        let at = reserve_more(e, raw, 1);
        e.mem.set_u32(raw + 4, at + 1);
        let slot = element(e, raw, at);
        store_bits(e, slot, second_point);
        e.call(SET_TRANSFORMED_POS, &args![slot, b.transform, slot]);
        let face = reserve_more(e, faces, 1);
        e.mem.set_u32(raw + 0x10, face + 1);
        let record = element(e, faces, face);
        set_triangle(e, record, index - 2, index - 1, index);
        byte_offset += 0x20;
        index += 3;
        item += 1;
        if item >= e.mem.i32(b.shape + 0x14) {
            break;
        }
    }
}

/// Bounds (type 28): the shape's box (slot `0x1C`, identity transform, tolerance from
/// `01017d00`) gives its extent; the longest axis (ties favour the first, a NaN loses every
/// compare) is marked 1.0 in an axis vector, and a bounds display object (`00de4f20`) is built
/// with the centre and the half extents.
fn build_bounds(e: &mut Engine, b: Build) {
    let s = b.frame;
    let tolerance = x87_quiet_bits(e.global::<u32>(BOUNDS_TOLERANCE));
    e.vcall(
        b.shape,
        0x1c,
        &args![IDENTITY_TRANSFORM, tolerance, s + 0xe0],
    );
    let maximum = load_vec(e, s + 0xf0);
    let minimum = load_vec(e, s + 0xe0);
    let mask = load_bits(e, ABSOLUTE_MASK);
    store_bits(e, s + 0x80, [0; 4]);
    let extent = zip_with(maximum, minimum, |high, low| high - low);
    let absolute = [
        extent[0].to_bits() & mask[0],
        extent[1].to_bits() & mask[1],
        extent[2].to_bits() & mask[2],
        extent[3].to_bits() & mask[3],
    ];
    store_bits(e, s + 0x40, absolute);
    let (x, y, z) = (
        f32::from_bits(absolute[0]),
        f32::from_bits(absolute[1]),
        f32::from_bits(absolute[2]),
    );
    let axis: u32 = if y > x {
        if z > y {
            2
        } else {
            1
        }
    } else if z > x {
        2
    } else {
        0
    };
    let half = f32::from_bits(e.global::<u32>(HALF_F32));
    e.mem.set_u32(s + 0x80 + 4 * axis, 1.0f32.to_bits());
    let half_extent = zip_with([half; 4], extent, |factor, value| factor * value);
    store_vec(e, s + 0xc0, half_extent);
    let centre = zip_with(half_extent, minimum, |value, low| value + low);
    store_vec(e, s + 0xa0, centre);
    let object = allocate_object(e, 0xa0);
    let made = e
        .call(
            BOUNDS_CONSTRUCTOR,
            &args![object, b.shape + 0x10, s + 0x80, s + 0xa0, s + 0xc0],
        )
        .u32();
    finish_object(e, b, made);
}

/// Shape types the function has no case for: every entry of the extra builder table (`0126 9CE0`)
/// whose type word matches gets `(shape, transform, array, builder)`.
fn build_extra(e: &mut Engine, b: Build) {
    let shape_type = e.mem.u32(b.shape + 0xc);
    let table: u32 = e.global(EXTRA_BUILDERS);
    let mut index = 0;
    if e.mem.i32(table + 0xc) > 0 {
        let mut table = table;
        loop {
            let entry = e.mem.u32(table + 8) + 8 * index as u32;
            if e.mem.u32(entry + 4) == shape_type {
                let builder = e.mem.u32(entry);
                e.call(builder, &args![b.shape, b.transform, b.array, b.this]);
                table = e.global(EXTRA_BUILDERS);
            }
            index += 1;
            if index >= e.mem.i32(table + 0xc) {
                break;
            }
        }
    }
}

// Translated from 00ddfd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeDisplayBuilder::buildShapeDisplay` (Xbox PDB), thiscall with three stack words
/// (`RET 0xC`): the `shape`, its `transform` (rows at `+0`, `+0x10`, `+0x20`, translation at
/// `+0x30`) and the `array` of display geometries to append to. The shape type word at `+0xC`
/// picks the case: 1 sphere, 2 cylinder, 3 triangle, 4 box, 5 capsule, 6 convex vertices,
/// 7/8/10/14/16/22/23 container wrappers, 9 list, 11/12/15 transform wrappers, 13 height
/// field, 20 triangle list, 21 positioned spheres, 24 segments, 27 wrapper, 28 bounds;
/// anything else is offered to the extra builder table. Every case that appends geometries
/// follows the exe's order of router calls, reserves and writes; the stack frame (0x360
/// bytes) holds the temporaries the callees get by address.
pub fn hkp_shape_display_builder_build_shape_display(
    e: &mut Engine,
    this: Ptr,
    shape: Ptr,
    transform: Ptr,
    array: Ptr,
) {
    let shape_address = shape.addr();
    let shape_type = e.mem.u32(shape_address + 0xc);
    e.with_stack(0x400, |e, frame| {
        let b = Build {
            this: this.addr(),
            shape: shape_address,
            transform: transform.addr(),
            array: array.addr(),
            frame: frame.addr(),
        };
        match shape_type {
            1 => build_sphere(e, b),
            2 => build_cylinder(e, b),
            3 => build_triangle(e, b),
            4 => build_box(e, b),
            5 => build_capsule(e, b),
            6 => build_convex_vertices(e, b),
            7 | 8 | 10 | 14 | 16 | 22 | 23 => build_container_children(e, b),
            9 => build_list_children(e, b),
            11 => build_translated_child(e, b),
            12 => build_transformed_child(e, b, 0x20, 0x18),
            13 => build_height_field(e, b),
            15 => build_transformed_child(e, b, 0x30, 0x14),
            20 => build_triangle_list(e, b),
            21 => build_positioned_spheres(e, b),
            24 => build_segments(e, b),
            27 => {
                let child = e.mem.u32(b.shape + 0x10);
                build_child(e, b, child, b.transform);
            }
            28 => build_bounds(e, b),
            _ => build_extra(e, b),
        }
    });
}

// Translated from 00ddfcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: cdecl, no stack words. Allocates 0x14 bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10),
/// stores the size as a 16-bit word at +4, the count 1 as a 16-bit word at +6, the vtable word `010d80a4` at +0, two zero words at +8 and +0xc and `0x80000000` at +0x10 (an empty array that owns no storage). Returns the block. Ghidra made no function here; the address is stored in the data word at 011b8524.
pub fn fn_00ddfcd0(e: &mut Engine) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x14u32]).u32();
    e.mem.set_u16(block + 4, 0x14);
    e.mem.set_u16(block + 6, 1);
    e.mem.set_u32(block, 0x010d_80a4);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 0x8000_0000);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00dd_fb70,
            hkp_shape_display_builder_environment_constructor(Ptr) -> Ptr
        ),
        entry!(
            0x00dd_fb80,
            hkp_shape_display_builder_constructor(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00dd_fbb0, fn_00ddfbb0(Ptr)),
        entry!(
            0x00dd_fc30,
            hkp_shape_display_builder_get_current_raw_geometry(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00dd_fd10,
            hkp_shape_display_builder_build_shape_display(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00de_0f90,
            hkp_shape_display_builder_build_display_geometries(Ptr, Ptr, Ptr)
        ),
        entry!(0x00dd_fcd0, fn_00ddfcd0() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };
    use crate::units::havok_sdk::hkpsphereshape::Vec4;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const ALLOCATE: u32 = 0x0a20_0004;
    const DEALLOCATE: u32 = 0x0a20_0010;
    /// Bases of the vtable slot doubles of the shapes, of the containers and of the extra
    /// builders.
    const SHAPE: u32 = 0x0a60_0000;
    const CONTAINER: u32 = 0x0a70_0000;
    const EXTRA: u32 = 0x0a80_0000;
    /// The translation of the test transform; the stand-in for `setTransformedPos` adds it.
    const TRANSLATION: Vec4 = [1.0, 2.0, 3.0, 0.0];

    /// The words a spy read at each call.
    type Words = Rc<RefCell<Vec<Vec<u32>>>>;

    struct Rig {
        log: Log,
        builder: u32,
        transform: u32,
        array: u32,
        allocator: u32,
        router: u32,
        scratch: u32,
    }

    /// An engine with the exe's constants the function reads, the memory router (slot 4 of its
    /// allocator hands out zeroed blocks, slot `0x10` frees; the first word of the router is the
    /// top of a 0x400-byte scratch stack), stand-ins for the array helpers, for
    /// `setTransformedPos` (adds the translation at `+0x30`) and for the display geometry
    /// constructors (they answer the new object; the convex one keeps its raw geometry at
    /// `+8`), a builder whose environment words are `0x1111` and `0x2222`, a transform with the
    /// translation `TRANSLATION` and an empty array of geometries.
    fn rig(e: &mut Engine) -> Rig {
        let log = new_log();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE);
        e.mem.set_u32(vtable + 0x10, DEALLOCATE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        let scratch = e.mem.alloc(0x400);
        e.mem.set_u32(router, scratch);
        log_double(e, &log, ROUTER, router);
        let allocations = Rc::clone(&log);
        e.register_double(ALLOCATE, move |e, a| {
            allocations.borrow_mut().push((ALLOCATE, a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        log_double(e, &log, DEALLOCATE, 0);
        let growths = Rc::clone(&log);
        e.register_double(ARRAY_GROW, move |e, a| {
            growths.borrow_mut().push((ARRAY_GROW, a.to_vec()));
            let size = e.mem.u32(a[1] + 4);
            let old = e.mem.u32(a[1]);
            let block = e.mem.alloc(64);
            for index in 0..size {
                let word = e.mem.u32(old + 4 * index);
                e.mem.set_u32(block + 4 * index, word);
            }
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, 16);
            Ret::default()
        });
        let reserves = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            reserves.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let size = e.mem.u32(a[1] + 4);
            let old = e.mem.u32(a[1]);
            let block = e.mem.alloc(a[2] * a[3]);
            for index in 0..size * a[3] / 4 {
                let word = e.mem.u32(old + 4 * index);
                e.mem.set_u32(block + 4 * index, word);
            }
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let positions = Rc::clone(&log);
        e.register_double(SET_TRANSFORMED_POS, move |e, a| {
            positions
                .borrow_mut()
                .push((SET_TRANSFORMED_POS, a.to_vec()));
            let point = load_vec(e, a[2]);
            let shift = load_vec(e, a[1] + 0x30);
            store_vec(e, a[0], zip_with(point, shift, |x, y| x + y));
            Ret::default()
        });
        log_double(e, &log, SET_TRANSFORM, 0);
        log_double(e, &log, SET_MUL, 0);
        for address in [
            SPHERE_CONSTRUCTOR,
            BOX_CONSTRUCTOR,
            CAPSULE_CONSTRUCTOR,
            CYLINDER_CONSTRUCTOR,
            BOUNDS_CONSTRUCTOR,
            CONVEX_CONSTRUCTOR,
            RAW_GEOMETRY_CONSTRUCTOR,
        ] {
            let constructions = Rc::clone(&log);
            e.register_double(address, move |e, a| {
                constructions.borrow_mut().push((address, a.to_vec()));
                if address == CONVEX_CONSTRUCTOR {
                    e.mem.set_u32(a[0] + 8, a[1]);
                }
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        let radii = Rc::clone(&log);
        e.register_double(CYLINDER_RADIUS, move |_, a| {
            radii.borrow_mut().push((CYLINDER_RADIUS, a.to_vec()));
            Ret {
                st0: 0.5,
                ..Ret::default()
            }
        });
        e.set_global(SMALL_FLOAT, 0.01f32);
        e.set_global(HALF_F32, 0.5f32);
        e.set_global(BOUNDS_TOLERANCE, 0.001f32);
        e.set_global(DOUBLE_ONE, 1.0f64);
        store_bits(e, ABSOLUTE_MASK, [0x7fff_ffff; 4]);
        let builder = e.mem.alloc(0x20);
        e.mem.set_u32(builder + 8, 0x1111);
        e.mem.set_u32(builder + 0xc, 0x2222);
        let transform = e.mem.alloc(0x40);
        store_vec(e, transform + 0x30, TRANSLATION);
        let array = e.mem.alloc(0x10);
        Rig {
            log,
            builder,
            transform,
            array,
            allocator,
            router,
            scratch,
        }
    }

    /// Re-registers the double at `address`: it logs, appends the words it reads at each
    /// `(argument index, word count)` to the result and answers its first argument.
    fn spy(e: &mut Engine, log: &Log, address: u32, reads: &'static [(usize, u32)]) -> Words {
        let seen: Words = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            log.borrow_mut().push((address, a.to_vec()));
            let mut words = Vec::new();
            for (index, count) in reads {
                for word in 0..*count {
                    words.push(e.mem.u32(a[*index] + 4 * word));
                }
            }
            sink.borrow_mut().push(words);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        seen
    }

    fn answer(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Puts a logging double at `address` into virtual slot `slot` of `object` (its vtable is
    /// made on first use).
    fn set_slot(
        e: &mut Engine,
        log: &Log,
        object: u32,
        slot: u32,
        address: u32,
        mut f: impl FnMut(&mut Engine, &[u32]) -> Ret + 'static,
    ) {
        let mut vtable = e.mem.u32(object);
        if vtable == 0 {
            vtable = e.mem.alloc(0x80);
            e.mem.set_u32(object, vtable);
        }
        e.mem.set_u32(vtable + slot, address);
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            log.borrow_mut().push((address, a.to_vec()));
            f(e, a)
        });
    }

    fn shape_of(e: &mut Engine, kind: u32) -> u32 {
        let shape = e.mem.alloc(0x100);
        e.mem.set_u32(shape + 0xc, kind);
        shape
    }

    /// A box shape with the half extents `(1, 2, 3, 0)`.
    fn box_shape(e: &mut Engine) -> u32 {
        let shape = shape_of(e, 4);
        store_vec(e, shape + 0x20, [1.0, 2.0, 3.0, 0.0]);
        shape
    }

    fn build(e: &mut Engine, rig: &Rig, shape: u32) {
        hkp_shape_display_builder_build_shape_display(
            e,
            Ptr::new(rig.builder),
            Ptr::new(shape),
            Ptr::new(rig.transform),
            Ptr::new(rig.array),
        );
    }

    /// The pointers in the array of geometries.
    fn items(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    /// The raw geometry of the builder (the record behind its display convex).
    fn raw_of(e: &Engine, rig: &Rig) -> u32 {
        let convex = e.mem.u32(rig.builder + 0x10);
        e.mem.u32(convex + 8)
    }

    fn vertices(e: &Engine, raw: u32) -> Vec<Vec4> {
        let data = e.mem.u32(raw);
        (0..e.mem.u32(raw + 4))
            .map(|index| load_vec(e, data + 16 * index))
            .collect()
    }

    fn faces(e: &Engine, raw: u32) -> Vec<[u32; 4]> {
        let data = e.mem.u32(raw + 0xc);
        (0..e.mem.u32(raw + 0x10))
            .map(|index| load_bits(e, data + 16 * index))
            .collect()
    }

    fn lanes(words: &[u32]) -> Vec4 {
        std::array::from_fn(|lane| f32::from_bits(words[lane]))
    }

    fn triangle(first: u32, second: u32, third: u32) -> [u32; 4] {
        [first, second, third, NO_KEY]
    }

    /// Makes the allocator refuse blocks of `size` bytes.
    fn refuse(e: &mut Engine, size: u32) {
        e.register_double(ALLOCATE, move |e, a| Ret {
            eax: if a[1] == size { 0 } else { e.mem.alloc(a[1]) },
            ..Ret::default()
        });
    }

    #[test]
    fn environment_constructor_sets_both_words() {
        let mut e = engine();
        let record = e.mem.alloc(8);
        let made = hkp_shape_display_builder_environment_constructor(&mut e, Ptr::new(record));
        assert_eq!(made.addr(), record);
        assert_eq!((e.mem.u32(record), e.mem.u32(record + 4)), (8, 8));
    }

    #[test]
    fn constructor_copies_the_environment() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 5);
        let environment = e.mem.alloc(8);
        e.mem.set_u32(environment, 0x11);
        e.mem.set_u32(environment + 4, 0x22);
        let made =
            hkp_shape_display_builder_constructor(&mut e, Ptr::new(this), Ptr::new(environment));
        assert_eq!(made.addr(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010d_8094);
        assert_eq!((e.mem.u32(this + 8), e.mem.u32(this + 0xc)), (0x11, 0x22));
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn forgetting_the_raw_geometry_clears_the_word() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0x1234);
        fn_00ddfbb0(&mut e, Ptr::new(this));
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn an_existing_raw_geometry_is_answered_untouched() {
        let mut e = engine();
        let rig = rig(&mut e);
        e.mem.set_u32(rig.builder + 0x10, 0x1234);
        let got = hkp_shape_display_builder_get_current_raw_geometry(
            &mut e,
            Ptr::new(rig.builder),
            Ptr::new(rig.array),
        );
        assert_eq!(got, 0x1234);
        assert!(rig.log.borrow().is_empty());
    }

    #[test]
    fn a_missing_raw_geometry_is_made_and_appended() {
        let mut e = engine();
        let rig = rig(&mut e);
        let got = hkp_shape_display_builder_get_current_raw_geometry(
            &mut e,
            Ptr::new(rig.builder),
            Ptr::new(rig.array),
        );
        assert_eq!(e.mem.u32(rig.builder + 0x10), got);
        assert_eq!(items(&e, rig.array), vec![got]);
        assert_eq!(e.mem.u16(got + 4), 0x60);
        assert_eq!(
            calls(&rig.log, ALLOCATE),
            vec![vec![rig.allocator, 0x18], vec![rig.allocator, 0x60]]
        );
        let made = calls(&rig.log, CONVEX_CONSTRUCTOR);
        assert_eq!(made, vec![vec![got, made[0][1]]]);
        let record = made[0][1];
        assert_ne!(record, 0);
        assert_eq!(load_bits(&e, record), [0, 0, 0x8000_0000, 0]);
        assert_eq!(e.mem.u32(record + 0x10), 0);
        assert_eq!(e.mem.u32(record + 0x14), 0x8000_0000);
    }

    #[test]
    fn a_refused_record_gives_a_display_convex_without_geometry() {
        let mut e = engine();
        let rig = rig(&mut e);
        refuse(&mut e, 0x18);
        let got = hkp_shape_display_builder_get_current_raw_geometry(
            &mut e,
            Ptr::new(rig.builder),
            Ptr::new(rig.array),
        );
        let made = calls(&rig.log, CONVEX_CONSTRUCTOR);
        assert_eq!(made, vec![vec![got, 0]]);
        assert_eq!(items(&e, rig.array), vec![got]);
    }

    #[test]
    fn build_display_geometries_starts_over_with_the_identity() {
        let mut e = engine();
        let rig = rig(&mut e);
        let matrices = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&matrices);
        e.register_double(SET_TRANSFORM, move |e, a| {
            let mut words = Vec::new();
            for row in 0..4 {
                words.push(load_bits(e, a[1] + 16 * row));
            }
            sink.borrow_mut().push(words);
            Ret::default()
        });
        let shape = box_shape(&mut e);
        e.mem.set_u32(rig.builder + 0x10, 0x77);
        e.mem.set_u32(rig.array + 4, 9);
        hkp_shape_display_builder_build_display_geometries(
            &mut e,
            Ptr::new(rig.builder),
            Ptr::new(shape),
            Ptr::new(rig.array),
        );
        let one = 1.0f32.to_bits();
        assert_eq!(
            *matrices.borrow(),
            vec![vec![[one, 0, 0, 0], [0, one, 0, 0], [0, 0, one, 0], [0; 4]]]
        );
        assert_eq!(e.mem.u32(rig.builder + 0x10), 0);
        assert_eq!(items(&e, rig.array).len(), 1);
    }

    #[test]
    fn a_sphere_is_built_from_its_radius() {
        let mut e = engine();
        let rig = rig(&mut e);
        let seen = spy(&mut e, &rig.log, SPHERE_CONSTRUCTOR, &[(1, 4)]);
        let shape = shape_of(&mut e, 1);
        e.mem.set_f32(shape + 0x10, 2.5);
        build(&mut e, &rig, shape);
        assert_eq!(*seen.borrow(), vec![vec![0, 0, 0, 2.5f32.to_bits()]]);
        let made = calls(&rig.log, SPHERE_CONSTRUCTOR);
        let object = made[0][0];
        assert_eq!(made[0][2..], [0x2222, 0x1111]);
        assert_eq!(e.mem.u16(object + 4), 0x80);
        assert_eq!(
            calls(&rig.log, SET_TRANSFORM),
            vec![vec![object + 0x10, rig.transform]]
        );
        assert_eq!(items(&e, rig.array), vec![object]);
    }

    #[test]
    fn a_cylinder_transforms_both_ends_and_asks_the_radius() {
        let mut e = engine();
        let rig = rig(&mut e);
        let shape = shape_of(&mut e, 2);
        build(&mut e, &rig, shape);
        let moves = calls(&rig.log, SET_TRANSFORMED_POS);
        assert_eq!(moves.len(), 2);
        assert_eq!(moves[0][1..], [rig.transform, shape + 0x30]);
        assert_eq!(moves[1][1..], [rig.transform, shape + 0x20]);
        assert_eq!(moves[0][0], moves[1][0] + 0x20);
        assert_eq!(calls(&rig.log, CYLINDER_RADIUS), vec![vec![shape]]);
        let made = calls(&rig.log, CYLINDER_CONSTRUCTOR);
        let object = made[0][0];
        assert_eq!(
            made,
            vec![vec![
                object,
                moves[0][0],
                moves[1][0],
                0.5f32.to_bits(),
                9,
                1
            ]]
        );
        assert_eq!(e.mem.u16(object + 4), 0x90);
        assert_eq!(items(&e, rig.array), vec![object]);
    }

    #[test]
    fn a_triangle_without_extrusion_adds_one_face() {
        let mut e = engine();
        let rig = rig(&mut e);
        let shape = shape_of(&mut e, 3);
        store_vec(&mut e, shape + 0x20, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, shape + 0x30, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, shape + 0x40, [0.0, 0.0, 1.0, 0.0]);
        build(&mut e, &rig, shape);
        let raw = raw_of(&e, &rig);
        assert_eq!(
            vertices(&e, raw),
            vec![
                [2.0, 2.0, 3.0, 0.0],
                [1.0, 3.0, 3.0, 0.0],
                [1.0, 2.0, 4.0, 0.0]
            ]
        );
        assert_eq!(faces(&e, raw), vec![triangle(0, 1, 2)]);
        assert_eq!(items(&e, rig.array).len(), 1);
    }

    #[test]
    fn an_extruded_triangle_adds_the_prism() {
        let mut e = engine();
        let rig = rig(&mut e);
        let shape = shape_of(&mut e, 3);
        store_vec(&mut e, shape + 0x20, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, shape + 0x30, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, shape + 0x40, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, shape + 0x50, [0.0, 0.0, 2.0, 0.0]);
        build(&mut e, &rig, shape);
        let raw = raw_of(&e, &rig);
        let all = vertices(&e, raw);
        assert_eq!(all.len(), 6);
        assert_eq!(all[3], [2.0, 2.0, 5.0, 0.0]);
        assert_eq!(all[4], [1.0, 3.0, 5.0, 0.0]);
        assert_eq!(all[5], [1.0, 2.0, 6.0, 0.0]);
        assert_eq!(
            faces(&e, raw),
            vec![
                triangle(0, 1, 2),
                triangle(3, 1, 0),
                triangle(3, 4, 1),
                triangle(4, 5, 1),
                triangle(5, 2, 1),
                triangle(5, 0, 2),
                triangle(5, 3, 0),
                triangle(5, 4, 3)
            ]
        );
    }

    #[test]
    fn a_triangle_with_an_unordered_extrusion_stops_after_its_face() {
        let mut e = engine();
        let rig = rig(&mut e);
        let shape = shape_of(&mut e, 3);
        store_vec(&mut e, shape + 0x50, [f32::NAN, 0.0, 0.0, 0.0]);
        build(&mut e, &rig, shape);
        let raw = raw_of(&e, &rig);
        assert_eq!(vertices(&e, raw).len(), 3);
        assert_eq!(faces(&e, raw).len(), 1);
    }

    #[test]
    fn a_box_is_built_from_its_half_extents() {
        let mut e = engine();
        let rig = rig(&mut e);
        let shape = box_shape(&mut e);
        build(&mut e, &rig, shape);
        let made = calls(&rig.log, BOX_CONSTRUCTOR);
        let object = made[0][0];
        assert_eq!(made, vec![vec![object, shape + 0x20]]);
        assert_eq!(e.mem.u16(object + 4), 0x70);
        assert_eq!(
            calls(&rig.log, SET_TRANSFORM),
            vec![vec![object + 0x10, rig.transform]]
        );
        assert_eq!(items(&e, rig.array), vec![object]);
    }

    #[test]
    fn a_capsule_is_built_from_its_ends_and_radius() {
        let mut e = engine();
        let rig = rig(&mut e);
        let shape = shape_of(&mut e, 5);
        e.mem.set_f32(shape + 0x10, 0.25);
        build(&mut e, &rig, shape);
        let made = calls(&rig.log, CAPSULE_CONSTRUCTOR);
        let object = made[0][0];
        assert_eq!(
            made,
            vec![vec![
                object,
                shape + 0x30,
                shape + 0x20,
                0.25f32.to_bits(),
                6,
                1
            ]]
        );
        assert_eq!(e.mem.u16(object + 4), 0x90);
        assert_eq!(items(&e, rig.array), vec![object]);
    }

    /// A convex vertices shape whose slot `0x2C` says `count` and whose slot `0x30` writes the
    /// vertices `(1, 2, 3, 4)` and `(5, 6, 7, 8)` where it is told.
    fn convex_shape(e: &mut Engine, rig: &Rig, count: u32) -> u32 {
        let shape = shape_of(e, 6);
        set_slot(e, &rig.log, shape, 0x2c, SHAPE + 0x2c, move |_, _| {
            answer(count)
        });
        set_slot(e, &rig.log, shape, 0x30, SHAPE + 0x30, |e, a| {
            store_vec(e, a[1], [1.0, 2.0, 3.0, 4.0]);
            store_vec(e, a[1] + 16, [5.0, 6.0, 7.0, 8.0]);
            answer(a[1])
        });
        shape
    }

    /// Records the input record of `createConvexGeometry` and the eight vertex words it points
    /// to.
    fn convex_inputs(e: &mut Engine) -> Words {
        let seen: Words = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(CREATE_CONVEX_GEOMETRY, move |e, a| {
            let data = e.mem.u32(a[0]);
            let mut words = vec![data, e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8)];
            if data != 0 {
                words.extend(load_bits(e, data));
                words.extend(load_bits(e, data + 16));
            }
            sink.borrow_mut().push(words);
            Ret::default()
        });
        seen
    }

    #[test]
    fn convex_vertices_are_transformed_into_a_geometry() {
        let mut e = engine();
        let rig = rig(&mut e);
        let inputs = convex_inputs(&mut e);
        let shape = convex_shape(&mut e, &rig, 2);
        build(&mut e, &rig, shape);
        assert_eq!(
            calls(&rig.log, SHAPE + 0x30),
            vec![vec![shape, rig.scratch]]
        );
        let seen = inputs.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0][1..3], [2, 0x10]);
        assert_eq!(lanes(&seen[0][3..7]), [2.0, 4.0, 6.0, 4.0]);
        assert_eq!(lanes(&seen[0][7..11]), [6.0, 8.0, 10.0, 8.0]);
        assert_eq!(
            calls(&rig.log, DEALLOCATE),
            vec![vec![rig.allocator, seen[0][0], 0x20]]
        );
        assert_eq!(e.mem.u32(rig.router), rig.scratch);
        let made = calls(&rig.log, CONVEX_CONSTRUCTOR);
        let record = made[0][1];
        assert_eq!(e.mem.u32(record + 8), 0x8000_0000);
        assert_eq!(e.mem.u32(record + 0x14), 0x8000_0000);
        assert_eq!(e.mem.u16(made[0][0] + 4), 0x60);
        assert_eq!(items(&e, rig.array), vec![made[0][0]]);
        assert_eq!(
            calls(&rig.log, ARRAY_RESERVE).len(),
            1,
            "only the heap array grows"
        );
    }

    #[test]
    fn convex_shape_without_vertices_builds_an_empty_geometry() {
        let mut e = engine();
        let rig = rig(&mut e);
        let inputs = convex_inputs(&mut e);
        let shape = convex_shape(&mut e, &rig, 0);
        build(&mut e, &rig, shape);
        assert_eq!(*inputs.borrow(), vec![vec![0, 0, 0x10]]);
        assert!(calls(&rig.log, SET_TRANSFORMED_POS).is_empty());
        assert!(calls(&rig.log, DEALLOCATE).is_empty());
        assert_eq!(e.mem.u32(rig.router), rig.scratch);
        assert_eq!(items(&e, rig.array).len(), 1);
    }

    #[test]
    fn a_refused_record_gives_the_convex_vertices_no_geometry() {
        let mut e = engine();
        let rig = rig(&mut e);
        refuse(&mut e, 0x18);
        convex_inputs(&mut e);
        let shape = convex_shape(&mut e, &rig, 2);
        build(&mut e, &rig, shape);
        let made = calls(&rig.log, CONVEX_CONSTRUCTOR);
        assert_eq!(made[0][1], 0);
        assert_eq!(items(&e, rig.array), vec![made[0][0]]);
    }

    #[test]
    fn container_types_build_every_child() {
        let mut e = engine();
        let rig = rig(&mut e);
        let first = box_shape(&mut e);
        let second = box_shape(&mut e);
        let container = e.mem.alloc(0x40);
        set_slot(&mut e, &rig.log, container, 8, CONTAINER + 8, |_, _| {
            answer(1)
        });
        set_slot(
            &mut e,
            &rig.log,
            container,
            0x14,
            CONTAINER + 0x14,
            move |_, a| answer(if a[1] == 1 { first } else { second }),
        );
        set_slot(&mut e, &rig.log, container, 0xc, CONTAINER + 0xc, |_, a| {
            answer(if a[1] == 1 { 2 } else { NO_KEY })
        });
        let shape = shape_of(&mut e, 7);
        set_slot(&mut e, &rig.log, shape, 0x10, SHAPE + 0x10, move |_, _| {
            answer(container)
        });
        build(&mut e, &rig, shape);
        let children = calls(&rig.log, CONTAINER + 0x14);
        assert_eq!(children.len(), 2);
        assert_eq!(children[0][..2], [container, 1]);
        assert_eq!(children[1][..2], [container, 2]);
        assert_eq!(children[0][2], children[1][2]);
        assert_eq!(
            calls(&rig.log, CONTAINER + 0xc),
            vec![vec![container, 1], vec![container, 2]]
        );
        let made = calls(&rig.log, BOX_CONSTRUCTOR);
        assert_eq!(made.len(), 2);
        assert_eq!(items(&e, rig.array), vec![made[0][0], made[1][0]]);
        assert_eq!(
            calls(&rig.log, SET_TRANSFORM)[1],
            vec![made[1][0] + 0x10, rig.transform]
        );
    }

    #[test]
    fn the_other_container_types_walk_their_container_too() {
        let mut e = engine();
        let rig = rig(&mut e);
        let container = e.mem.alloc(0x40);
        set_slot(&mut e, &rig.log, container, 8, CONTAINER + 8, |_, _| {
            answer(NO_KEY)
        });
        for kind in [8, 10, 14, 16, 22, 23] {
            let shape = shape_of(&mut e, kind);
            set_slot(&mut e, &rig.log, shape, 0x10, SHAPE + 0x10, move |_, _| {
                answer(container)
            });
            build(&mut e, &rig, shape);
        }
        assert_eq!(calls(&rig.log, SHAPE + 0x10).len(), 6);
        assert_eq!(calls(&rig.log, CONTAINER + 8).len(), 6);
        assert!(calls(&rig.log, CONTAINER + 0x14).is_empty());
        assert!(items(&e, rig.array).is_empty());
    }

    #[test]
    fn a_list_shape_walks_its_embedded_container() {
        let mut e = engine();
        let rig = rig(&mut e);
        let first = box_shape(&mut e);
        let second = box_shape(&mut e);
        let shape = shape_of(&mut e, 9);
        let records = e.mem.alloc(0x40);
        e.mem.set_u32(records, first);
        e.mem.set_u32(records + 16, second);
        e.mem.set_u32(shape + 0x18, records);
        set_slot(&mut e, &rig.log, shape + 0x10, 8, CONTAINER + 8, |_, _| {
            answer(0)
        });
        set_slot(
            &mut e,
            &rig.log,
            shape + 0x10,
            0xc,
            CONTAINER + 0xc,
            |_, a| answer(if a[1] == 0 { 1 } else { NO_KEY }),
        );
        build(&mut e, &rig, shape);
        assert_eq!(
            calls(&rig.log, CONTAINER + 0xc),
            vec![vec![shape + 0x10, 0], vec![shape + 0x10, 1]]
        );
        let made = calls(&rig.log, BOX_CONSTRUCTOR);
        assert_eq!(made.len(), 2);
        assert_eq!(made[0][1], first + 0x20);
        assert_eq!(made[1][1], second + 0x20);
        assert_eq!(items(&e, rig.array), vec![made[0][0], made[1][0]]);
    }

    #[test]
    fn a_translated_shape_builds_its_child_with_the_product() {
        let mut e = engine();
        let rig = rig(&mut e);
        let rows = spy(&mut e, &rig.log, SET_MUL, &[(2, 16)]);
        let child = box_shape(&mut e);
        let shape = shape_of(&mut e, 11);
        e.mem.set_u32(shape + 0x18, child);
        store_vec(&mut e, shape + 0x20, [7.0, 8.0, 9.0, 0.0]);
        build(&mut e, &rig, shape);
        let one = 1.0f32.to_bits();
        let seven = 7.0f32.to_bits();
        assert_eq!(
            rows.borrow()[0][..],
            [
                one,
                0,
                0,
                0,
                0,
                one,
                0,
                0,
                0,
                0,
                one,
                0,
                seven,
                8.0f32.to_bits(),
                9.0f32.to_bits(),
                0
            ]
        );
        let products = calls(&rig.log, SET_MUL);
        assert_eq!(products[0][1], rig.transform);
        let made = calls(&rig.log, BOX_CONSTRUCTOR);
        assert_eq!(
            calls(&rig.log, SET_TRANSFORM),
            vec![vec![made[0][0] + 0x10, products[0][0]]]
        );
    }

    #[test]
    fn transform_wrappers_multiply_and_build_their_child() {
        for (kind, transform_offset, child_offset) in [(12, 0x20, 0x18), (15, 0x30, 0x14)] {
            let mut e = engine();
            let rig = rig(&mut e);
            let child = box_shape(&mut e);
            let shape = shape_of(&mut e, kind);
            e.mem.set_u32(shape + child_offset, child);
            build(&mut e, &rig, shape);
            let products = calls(&rig.log, SET_MUL);
            assert_eq!(
                products,
                vec![vec![
                    products[0][0],
                    rig.transform,
                    shape + transform_offset
                ]]
            );
            assert_ne!(products[0][0], rig.transform);
            let made = calls(&rig.log, BOX_CONSTRUCTOR);
            assert_eq!(made[0][1], child + 0x20);
            assert_eq!(
                calls(&rig.log, SET_TRANSFORM),
                vec![vec![made[0][0] + 0x10, products[0][0]]]
            );
        }
    }

    #[test]
    fn a_wrapper_builds_its_child_with_the_same_transform() {
        let mut e = engine();
        let rig = rig(&mut e);
        let child = box_shape(&mut e);
        let shape = shape_of(&mut e, 27);
        e.mem.set_u32(shape + 0x10, child);
        build(&mut e, &rig, shape);
        assert!(calls(&rig.log, SET_MUL).is_empty());
        let made = calls(&rig.log, BOX_CONSTRUCTOR);
        assert_eq!(
            calls(&rig.log, SET_TRANSFORM),
            vec![vec![made[0][0] + 0x10, rig.transform]]
        );
    }

    /// A height field of `x_cells + 1` by `z_cells + 1` heights `10 * x + z` and the scale
    /// `(2, 3, 4, 1)`; slot `0x38` answers 1 for the first cell and 0 for the others.
    fn height_field(e: &mut Engine, rig: &Rig, x_count: i32, z_count: i32) -> u32 {
        let shape = shape_of(e, 13);
        e.mem.set_i32(shape + 0x10, x_count);
        e.mem.set_i32(shape + 0x14, z_count);
        store_vec(e, shape + 0x20, [2.0, 3.0, 4.0, 1.0]);
        set_slot(e, &rig.log, shape, 0x34, SHAPE + 0x34, |_, a| Ret {
            st0: f64::from(10 * a[1] as i32 + a[2] as i32),
            ..Ret::default()
        });
        let cells = Rc::new(Cell::new(0u32));
        set_slot(e, &rig.log, shape, 0x38, SHAPE + 0x38, move |e, a| {
            e.mem.set_u8(a[1], u8::from(cells.get() == 0));
            cells.set(cells.get() + 1);
            Ret::default()
        });
        shape
    }

    #[test]
    fn a_height_field_adds_two_faces_per_cell() {
        let mut e = engine();
        let rig = rig(&mut e);
        let shape = height_field(&mut e, &rig, 3, 3);
        build(&mut e, &rig, shape);
        let mut expected_calls = Vec::new();
        let mut expected_vertices = Vec::new();
        for (x, z) in [(0u32, 0u32), (0, 1), (1, 0), (1, 1)] {
            for (dx, dz) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                expected_calls.push(vec![shape, x + dx, z + dz]);
                let height = (10 * (x + dx) + z + dz) as f32;
                expected_vertices.push([
                    (x + dx) as f32 * 2.0 + 1.0,
                    height * 3.0 + 2.0,
                    (z + dz) as f32 * 4.0 + 3.0,
                    0.0,
                ]);
            }
        }
        assert_eq!(calls(&rig.log, SHAPE + 0x34), expected_calls);
        let raw = raw_of(&e, &rig);
        assert_eq!(vertices(&e, raw), expected_vertices);
        let cell = |base| {
            (
                triangle(base, base + 1, base + 2),
                triangle(base + 3, base + 2, base + 1),
            )
        };
        let mut expected_faces = vec![triangle(0, 1, 3), triangle(0, 3, 2)];
        for base in [4, 8, 12] {
            let (first, second) = cell(base);
            expected_faces.push(first);
            expected_faces.push(second);
        }
        assert_eq!(faces(&e, raw), expected_faces);
        assert_eq!(calls(&rig.log, SHAPE + 0x38).len(), 4);
        assert_eq!(items(&e, rig.array).len(), 1);
    }

    #[test]
    fn height_field_cell_counts_follow_the_resolution() {
        for (x_count, z_count) in [(1, 5), (5, 1), (2, 2)] {
            let mut e = engine();
            let rig = rig(&mut e);
            let shape = height_field(&mut e, &rig, x_count, z_count);
            build(&mut e, &rig, shape);
            let raw = raw_of(&e, &rig);
            let cells = (x_count - 1) * (z_count - 1);
            assert_eq!(vertices(&e, raw).len() as i32, 4 * cells.max(0));
            assert_eq!(calls(&rig.log, SHAPE + 0x34).len() as i32, 4 * cells.max(0));
        }
    }

    /// A triangle list of the given children, each three points `(i, 0, 0)` apart.
    fn triangle_list(e: &mut Engine, rig: &Rig, count: i32) -> u32 {
        let shape = shape_of(e, 20);
        let container = e.mem.alloc(0x40);
        let keys = e.mem.alloc(0x10);
        e.mem.set_u32(keys, 5);
        e.mem.set_u32(keys + 4, 6);
        e.mem.set_u32(shape + 0x20, container);
        e.mem.set_u32(shape + 0x24, keys);
        e.mem.set_i32(shape + 0x28, count);
        let children = [e.mem.alloc(0x60), e.mem.alloc(0x60)];
        for (index, child) in children.iter().enumerate() {
            for point in 0..3 {
                store_vec(
                    e,
                    child + 0x20 + 0x10 * point,
                    [index as f32, point as f32, 0.0, 0.0],
                );
            }
        }
        set_slot(
            e,
            &rig.log,
            container + 0x10,
            0x14,
            CONTAINER + 0x14,
            move |_, a| answer(children[(a[1] - 5) as usize]),
        );
        shape
    }

    #[test]
    fn a_triangle_list_collects_every_child_into_one_geometry() {
        let mut e = engine();
        let rig = rig(&mut e);
        let shape = triangle_list(&mut e, &rig, 2);
        build(&mut e, &rig, shape);
        let container = e.mem.u32(shape + 0x20);
        let asked = calls(&rig.log, CONTAINER + 0x14);
        assert_eq!(asked.len(), 2);
        assert_eq!(asked[0][..2], [container + 0x10, 5]);
        assert_eq!(asked[1][..2], [container + 0x10, 6]);
        let made = calls(&rig.log, CONVEX_CONSTRUCTOR);
        let raw = made[0][1];
        assert_eq!(calls(&rig.log, RAW_GEOMETRY_CONSTRUCTOR), vec![vec![raw]]);
        assert_eq!(
            vertices(&e, raw),
            vec![
                [1.0, 2.0, 3.0, 0.0],
                [1.0, 3.0, 3.0, 0.0],
                [1.0, 4.0, 3.0, 0.0],
                [2.0, 2.0, 3.0, 0.0],
                [2.0, 3.0, 3.0, 0.0],
                [2.0, 4.0, 3.0, 0.0]
            ]
        );
        assert_eq!(faces(&e, raw), vec![triangle(0, 1, 2), triangle(3, 4, 5)]);
        assert_eq!(e.mem.u16(made[0][0] + 4), 0x60);
        assert_eq!(items(&e, rig.array), vec![made[0][0]]);
    }

    #[test]
    fn an_empty_triangle_list_still_adds_a_convex() {
        let mut e = engine();
        let rig = rig(&mut e);
        let shape = triangle_list(&mut e, &rig, 0);
        build(&mut e, &rig, shape);
        let made = calls(&rig.log, CONVEX_CONSTRUCTOR);
        assert_ne!(made[0][1], 0);
        assert!(calls(&rig.log, CONTAINER + 0x14).is_empty());
        assert_eq!(items(&e, rig.array), vec![made[0][0]]);
    }

    #[test]
    fn a_refused_triangle_list_record_gives_a_convex_without_geometry() {
        let mut e = engine();
        let rig = rig(&mut e);
        refuse(&mut e, 0x18);
        let shape = triangle_list(&mut e, &rig, 0);
        build(&mut e, &rig, shape);
        let made = calls(&rig.log, CONVEX_CONSTRUCTOR);
        assert_eq!(made[0][1], 0);
        assert!(calls(&rig.log, RAW_GEOMETRY_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn positioned_spheres_get_a_translation_each() {
        let mut e = engine();
        let rig = rig(&mut e);
        let seen = spy(&mut e, &rig.log, SPHERE_CONSTRUCTOR, &[(1, 4)]);
        let shape = shape_of(&mut e, 21);
        e.mem.set_i32(shape + 0x10, 2);
        store_vec(&mut e, shape + 0x20, [1.0, 2.0, 3.0, 0.5]);
        store_vec(&mut e, shape + 0x30, [4.0, 5.0, 6.0, 0.25]);
        build(&mut e, &rig, shape);
        assert_eq!(
            *seen.borrow(),
            vec![
                vec![0, 0, 0, 0.5f32.to_bits()],
                vec![0, 0, 0, 0.25f32.to_bits()]
            ]
        );
        let made = calls(&rig.log, SPHERE_CONSTRUCTOR);
        let objects = [made[0][0], made[1][0]];
        assert_eq!(items(&e, rig.array), objects.to_vec());
        assert_eq!(made[0][2..], [0x2222, 0x1111]);
        for (object, position) in objects.iter().zip([shape + 0x20, shape + 0x30]) {
            assert_eq!(e.mem.u16(object + 4), 0x80);
            assert_eq!(
                (0..4)
                    .map(|row| load_vec(&e, object + 0x10 * (row + 1)))
                    .collect::<Vec<_>>()[..3],
                [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0]
                ]
            );
            assert_eq!(load_bits(&e, object + 0x40), load_bits(&e, position));
        }
        assert!(calls(&rig.log, SET_TRANSFORM).is_empty());
    }

    #[test]
    fn no_positioned_spheres_means_no_work() {
        let mut e = engine();
        let rig = rig(&mut e);
        let shape = shape_of(&mut e, 21);
        build(&mut e, &rig, shape);
        assert!(rig.log.borrow().is_empty());
    }

    #[test]
    fn segments_add_three_vertices_and_a_face_each() {
        let mut e = engine();
        let rig = rig(&mut e);
        let shape = shape_of(&mut e, 24);
        let items_at = e.mem.alloc(0x40);
        store_vec(&mut e, items_at, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, items_at + 0x10, [4.0, 5.0, 6.0, 0.0]);
        store_vec(&mut e, items_at + 0x20, [0.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, items_at + 0x30, [1.0, 1.0, 1.0, 0.0]);
        e.mem.set_u32(shape + 0x10, items_at);
        e.mem.set_i32(shape + 0x14, 2);
        build(&mut e, &rig, shape);
        let raw = raw_of(&e, &rig);
        assert_eq!(
            vertices(&e, raw),
            vec![
                [2.0, 4.0, 6.0, 0.0],
                [1.0 + 0.01, 2.0, 3.0, 0.0],
                [5.0, 7.0, 9.0, 0.0],
                [1.0, 2.0, 3.0, 0.0],
                [0.01, 0.0, 0.0, 0.0],
                [2.0, 3.0, 4.0, 0.0]
            ]
        );
        assert_eq!(faces(&e, raw), vec![triangle(0, 1, 2), triangle(3, 4, 5)]);
        assert_eq!(calls(&rig.log, SET_TRANSFORMED_POS).len(), 4);
    }

    #[test]
    fn no_segments_leave_the_raw_geometry_empty() {
        let mut e = engine();
        let rig = rig(&mut e);
        let shape = shape_of(&mut e, 24);
        build(&mut e, &rig, shape);
        let raw = raw_of(&e, &rig);
        assert!(vertices(&e, raw).is_empty());
        assert!(faces(&e, raw).is_empty());
    }

    /// Builds a bounds shape whose box is `minimum` to `maximum` and answers the words the
    /// bounds constructor got: the axis, the centre and the half extents.
    fn bounds(minimum: Vec4, maximum: Vec4) -> (Vec4, Vec4, Vec4) {
        let mut e = engine();
        let rig = rig(&mut e);
        let seen = spy(
            &mut e,
            &rig.log,
            BOUNDS_CONSTRUCTOR,
            &[(2, 4), (3, 4), (4, 4)],
        );
        let shape = shape_of(&mut e, 28);
        set_slot(&mut e, &rig.log, shape, 0x1c, SHAPE + 0x1c, move |e, a| {
            store_vec(e, a[3], minimum);
            store_vec(e, a[3] + 0x10, maximum);
            Ret::default()
        });
        build(&mut e, &rig, shape);
        let asked = calls(&rig.log, SHAPE + 0x1c);
        assert_eq!(asked.len(), 1);
        assert_eq!(
            asked[0][..3],
            [shape, IDENTITY_TRANSFORM, 0.001f32.to_bits()]
        );
        let made = calls(&rig.log, BOUNDS_CONSTRUCTOR);
        assert_eq!(made[0][1], shape + 0x10);
        assert_eq!(e.mem.u16(made[0][0] + 4), 0xa0);
        assert_eq!(items(&e, rig.array), vec![made[0][0]]);
        assert_eq!(
            calls(&rig.log, SET_TRANSFORM),
            vec![vec![made[0][0] + 0x10, rig.transform]]
        );
        let words = seen.borrow()[0].clone();
        (
            lanes(&words[0..4]),
            lanes(&words[4..8]),
            lanes(&words[8..12]),
        )
    }

    #[test]
    fn bounds_mark_the_longest_axis() {
        let zero = [0.0; 4];
        let cases: [(Vec4, Vec4); 9] = [
            ([3.0, 1.0, 2.0, 0.0], [1.0, 0.0, 0.0, 0.0]),
            ([1.0, 3.0, 2.0, 0.0], [0.0, 1.0, 0.0, 0.0]),
            ([1.0, 2.0, 3.0, 0.0], [0.0, 0.0, 1.0, 0.0]),
            ([2.0, 2.0, 2.0, 0.0], [1.0, 0.0, 0.0, 0.0]),
            ([1.0, 2.0, 2.0, 0.0], [0.0, 1.0, 0.0, 0.0]),
            ([1.0, 1.0, 2.0, 0.0], [0.0, 0.0, 1.0, 0.0]),
            ([-3.0, -1.0, -2.0, 0.0], [1.0, 0.0, 0.0, 0.0]),
            ([f32::NAN, 5.0, 5.0, 0.0], [1.0, 0.0, 0.0, 0.0]),
            ([1.0, f32::NAN, 5.0, 0.0], [0.0, 0.0, 1.0, 0.0]),
        ];
        for (extent, axis) in cases {
            let maximum = zip_with(zero, extent, |low, size| low + size);
            let (found, _, _) = bounds(zero, maximum);
            let found_bits: Vec<u32> = found.iter().map(|lane| lane.to_bits()).collect();
            let expected_bits: Vec<u32> = axis.iter().map(|lane| lane.to_bits()).collect();
            assert_eq!(found_bits, expected_bits, "extent {extent:?}");
        }
    }

    #[test]
    fn bounds_centre_and_half_extents_come_from_the_box() {
        let (axis, centre, half) = bounds([1.0, 2.0, 3.0, 9.0], [2.0, 5.0, 5.0, 9.0]);
        assert_eq!(axis, [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(centre, [1.5, 3.5, 4.0, 9.0]);
        assert_eq!(half, [0.5, 1.5, 1.0, 0.0]);
    }

    /// A table of extra builders with the given `(function, shape type)` entries.
    fn extra_table(e: &mut Engine, entries: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x10);
        let data = e.mem.alloc(8 * entries.len().max(1) as u32);
        for (index, (function, kind)) in entries.iter().enumerate() {
            e.mem.set_u32(data + 8 * index as u32, *function);
            e.mem.set_u32(data + 8 * index as u32 + 4, *kind);
        }
        e.mem.set_u32(table + 8, data);
        e.mem.set_i32(table + 0xc, entries.len() as i32);
        table
    }

    #[test]
    fn unknown_types_go_to_the_matching_extra_builders() {
        let mut e = engine();
        let rig = rig(&mut e);
        let table = extra_table(&mut e, &[(EXTRA, 18), (EXTRA + 4, 19), (EXTRA + 8, 19)]);
        e.set_global(EXTRA_BUILDERS, table);
        for offset in [0, 4, 8] {
            log_double(&mut e, &rig.log, EXTRA + offset, 0);
        }
        let shape = shape_of(&mut e, 19);
        build(&mut e, &rig, shape);
        let words = vec![vec![shape, rig.transform, rig.array, rig.builder]];
        assert!(calls(&rig.log, EXTRA).is_empty());
        assert_eq!(calls(&rig.log, EXTRA + 4), words);
        assert_eq!(calls(&rig.log, EXTRA + 8), words);
    }

    #[test]
    fn the_extra_builder_table_is_read_again_after_a_call() {
        let mut e = engine();
        let rig = rig(&mut e);
        let table = extra_table(&mut e, &[(EXTRA, 19), (EXTRA + 4, 19)]);
        let shorter = extra_table(&mut e, &[(EXTRA, 19)]);
        e.set_global(EXTRA_BUILDERS, table);
        set_slot_free(&mut e, &rig.log, EXTRA, move |e| {
            e.set_global(EXTRA_BUILDERS, shorter)
        });
        log_double(&mut e, &rig.log, EXTRA + 4, 0);
        let shape = shape_of(&mut e, 19);
        build(&mut e, &rig, shape);
        assert_eq!(calls(&rig.log, EXTRA).len(), 1);
        assert!(calls(&rig.log, EXTRA + 4).is_empty());
    }

    /// A logging double at `address` that runs `f`.
    fn set_slot_free(e: &mut Engine, log: &Log, address: u32, f: impl Fn(&mut Engine) + 'static) {
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            log.borrow_mut().push((address, a.to_vec()));
            f(e);
            Ret::default()
        });
    }

    #[test]
    fn types_without_a_case_or_a_table_entry_do_nothing() {
        let mut e = engine();
        let rig = rig(&mut e);
        let table = extra_table(&mut e, &[]);
        e.set_global(EXTRA_BUILDERS, table);
        for kind in [0, 17, 18, 19, 25, 26, 29, 0xffff_ffff] {
            let shape = shape_of(&mut e, kind);
            build(&mut e, &rig, shape);
        }
        assert!(rig.log.borrow().is_empty());
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    #[test]
    fn fn_00ddfcd0_allocates_and_initialises_the_block() {
        let (mut e, sizes) = allocator_engine();
        let block = e.call(0x00dd_fcd0, &args![]).u32();
        assert_eq!(*sizes.borrow(), vec![0x14]);
        assert_eq!(e.mem.u16(block + 4), 0x14);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block), 0x010d_80a4);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0x8000_0000);
    }
}
