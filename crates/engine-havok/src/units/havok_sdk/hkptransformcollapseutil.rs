//! `Havok SDK/hkptransformcollapseutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The util folds `hkpTransformShape`s into the shapes they transform. It works on a "shared
//! shape data" array of 12-byte entries `{shape, replacement, use count}` (`this`+0 data,
//! +4 size, +8 capacity word: low 30 bits the capacity, bit 31 set when the storage is not
//! owned), a policy block `{mode, limit, byte flag}` and four counters (shapes collapsed into a
//! new simple shape, into a child, wrapped in a translate shape, lists rebuilt). Reference
//! counting goes through `00c90510` (add) and `00c905b0` (remove), both `this` = the object.
//! The names of the functions that are not in the Xbox PDB are inferred from what they do.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpboxmotion::{add, mul, splat};
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, load_bits, load_vec, store_bits, store_vec, sum3, ZERO_F32,
};

/// The memory router (`00c85750`): its field `+0x10` is the allocator (virtual `4` allocates a
/// block, virtual `0x10` frees `(data, bytes)`).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkArray::grow by one` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_GROW_BY_ONE: u32 = 0x00c9_0940;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkVector4::setTransformedPos` and `setRotatedDir` (`this`, `(transform, vector)`).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// `hkTransform::isApproximatelyEqual` (`this`, `(flag out, other, epsilon)`, answers the flag
/// pointer), `hkTransform::setMul` (`this`, `(a, b)`) and the `hkMatrix3` checks.
const TRANSFORM_IS_APPROX_EQUAL: u32 = 0x00cb_2160;
const TRANSFORM_SET_MUL: u32 = 0x00cb_2240;
const MATRIX3_IS_APPROX_EQUAL: u32 = 0x00cd_4e00;
const MATRIX3_IS_ORTHONORMAL: u32 = 0x00cb_2e70;
const MATRIX3_IS_OK: u32 = 0x00cb_2d70;
/// Constructors: `hkpTransformShape` (this, (child, transform)), `hkpListShape` (this, (child
/// array data, count, true)), `hkpConvexTransformShape` (this, (child, transform, true)),
/// `hkpConvexTranslateShape` (this, (0xb, radius, child, true)), `hkpBoxShape` (this, (half
/// extents, radius)), `hkpCapsuleShape` (this, (a, b, radius)), `hkpCylinderShape` (this, (a, b,
/// cylinder radius, convex radius)), `hkpConvexVerticesShape` (this, (vertices, planes, radius)).
const TRANSFORM_SHAPE_CONSTRUCT: u32 = 0x00cf_5990;
const LIST_SHAPE_CONSTRUCT: u32 = 0x00cf_5640;
const CONVEX_TRANSFORM_SHAPE_CONSTRUCT: u32 = 0x00d5_d7a0;
const CONVEX_TRANSLATE_SHAPE_CONSTRUCT: u32 = 0x00c9_e340;
const BOX_SHAPE_CONSTRUCT: u32 = 0x00c9_ddb0;
const CAPSULE_SHAPE_CONSTRUCT: u32 = 0x00d2_2590;
const CYLINDER_SHAPE_CONSTRUCT: u32 = 0x00d5_7aa0;
const CONVEX_VERTICES_SHAPE_CONSTRUCT: u32 = 0x00d5_9bd0;
/// The cylinder's radius getter (`this`, answers the radius in `ST0`), the convex vertices
/// shape's `getOriginalVertices` (`this`, `(array)`) and `getPlaneEquations` (answers the array).
const CYLINDER_RADIUS: u32 = 0x00d5_71a0;
const GET_ORIGINAL_VERTICES: u32 = 0x00d5_9200;
const GET_PLANE_EQUATIONS: u32 = 0x00d5_8d50;
/// What runs on the shared array once the shapes were added (`00d74a20`, `this`), and the
/// reset of the four counters (`00d73400`, `this` = the counters).
const SHARED_SHAPES_FINISH: u32 = 0x00d7_4a20;
const COUNTERS_RESET: u32 = 0x00d7_3400;

/// This unit's own functions, called through the table as the exe calls them.
const COLLECT_CHILD_SHAPES: u32 = 0x00d7_37d0;
const SHARED_SHAPES_ADD_SHAPE: u32 = 0x00d7_38f0;
const COLLAPSE_TRANSFORM_SHAPE: u32 = 0x00d7_39f0;
const COLLAPSE_SHAPES_RECURSIVELY: u32 = 0x00d7_4480;
const COLLAPSE_HOLDERS: u32 = 0x00d7_4850;
const SHARED_SHAPES_GET_USE_COUNT: u32 = 0x00d7_4980;
const SHARED_SHAPES_SET_REPLACEMENT: u32 = 0x00d7_49c0;
const SHARED_SHAPES_CLEAR: u32 = 0x00d7_4aa0;
const SHARED_SHAPES_BUILD: u32 = 0x00d7_4b20;

/// The identity transform (`01268370`; its columns follow at `+0x10`, `+0x20`).
const IDENTITY_TRANSFORM: u32 = 0x0126_8370;
/// The float `0.001` (`01017d00`) passed to every approximate comparison.
const EPSILON: u32 = 0x0101_7d00;
/// The vector mask `(~0, ~0, ~0, 0)` (`010c4c10`) that clears the fourth lane of a translation.
const TRANSLATION_MASK: u32 = 0x010c_4c10;
/// The vtable `hkpConvexTranslateShape` objects get after their constructor.
const CONVEX_TRANSLATE_VTABLE: u32 = 0x010d_0e5c;
/// The double `0.0001` (`01032980`) that decides whether a matrix entry is 0 or 1 and whether a
/// translation is zero, and the float `0.05` (`011b153c`) used as the box radius.
const AXIS_TOLERANCE: u32 = 0x0103_2980;
const BOX_RADIUS: u32 = 0x011b_153c;

const CAPACITY_MASK: u32 = 0x3fff_ffff;
const NOT_OWNED: u32 = 0x8000_0000;

/// The shape kinds this unit looks at (field `+0xc` of a shape).
const KIND_SPHERE: u32 = 1;
const KIND_LIST: u32 = 9;
const KIND_CONVEX_TRANSLATE: u32 = 0xb;
const KIND_CONVEX_TRANSFORM: u32 = 0xc;
const KIND_TRANSFORM: u32 = 0xf;

// Frame slots of `collapse_transform_shape` (offsets from the stack pointer of its body): the
// transform of the shape as four columns, the flags the comparisons write, and the vectors
// handed to the constructors.
const FRAME_FLAG: u32 = 0x17;
const FRAME_PLANES: u32 = 0x24;
const FRAME_VERTICES: u32 = 0x44;
const FRAME_STRIDED: u32 = 0x50;
const FRAME_PLANE: u32 = 0x60;
const FRAME_TRANSFORM: u32 = 0x70;
const FRAME_TRANSLATION: u32 = 0xa0;
const FRAME_COPY: u32 = 0xb0;
const FRAME_SIZE: u32 = 0xc0;

/// The allocator the memory router hands out.
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// A block from the allocator (virtual `4`) with its size stored as the word at `+4`.
fn allocate_object(e: &mut Engine, size: u16) -> u32 {
    let allocator = allocator(e);
    let block = e.vcall(allocator, 4, &args![u32::from(size)]).u32();
    e.mem.set_u16(block + 4, size);
    block
}

/// The bits an `FLD dword` / `FSTP dword` pair leaves for the float at `address`.
fn quiet_bits(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(address))
}

fn set_empty_array(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

fn signed_size(e: &Engine, array: u32) -> i32 {
    e.mem.u32(array + 4) as i32
}

/// Appends `value` to an array of words: the router first, then the grow-by-one when the
/// size has reached the capacity.
fn array_push_word(e: &mut Engine, array: u32, value: u32) {
    let allocator = allocator(e);
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_GROW_BY_ONE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// What every local array of this unit does when it goes out of scope: the router, the size
/// back to 0 and, unless the storage is not owned, the free (virtual `0x10`) of `capacity *
/// element size` bytes. Answers what the exe leaves in `EAX`: the capacity word when it did
/// not free, else the answer of the free.
fn release_array(e: &mut Engine, array: u32, element_size: u32) -> u32 {
    let allocator = allocator(e);
    let capacity_word = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity_word & NOT_OWNED != 0 {
        return capacity_word;
    }
    let data = e.mem.u32(array);
    let bytes = (capacity_word & CAPACITY_MASK).wrapping_mul(element_size);
    e.vcall(allocator, 0x10, &args![data, bytes]).u32()
}

fn add_reference(e: &mut Engine, object: u32) {
    e.call(ADD_REFERENCE, &args![object]);
}

fn remove_reference(e: &mut Engine, object: u32) {
    e.call(REMOVE_REFERENCE, &args![object]);
}

/// The exit most paths take: one more reference on `object`, which is the answer.
fn add_reference_and_answer(e: &mut Engine, object: u32) -> u32 {
    add_reference(e, object);
    object
}

/// `*(counters + 4 * index) += 1`.
fn bump_counter(e: &mut Engine, counters: u32, index: u32) {
    let address = counters + 4 * index;
    let value = e.mem.u32(address).wrapping_add(1);
    e.mem.set_u32(address, value);
}

/// The index of the first 12-byte entry of the shared array whose first word is `key` (or,
/// when `either`, whose first or second word is), scanning while the index is below the signed
/// size.
fn find_entry(e: &Engine, array: u32, key: u32, either: bool) -> Option<u32> {
    let size = signed_size(e, array);
    let data = e.mem.u32(array);
    let mut entry = data;
    for index in 0..size {
        if e.mem.u32(entry) == key || (either && e.mem.u32(entry + 4) == key) {
            return Some(index as u32);
        }
        entry = entry.wrapping_add(12);
    }
    None
}

fn entry_address(e: &Engine, array: u32, index: u32) -> u32 {
    e.mem.u32(array).wrapping_add(index.wrapping_mul(12))
}

/// `setReplacement` on the shared array as the callers use it.
fn set_replacement(e: &mut Engine, shared: u32, shape: u32, replacement: u32) {
    e.call(
        SHARED_SHAPES_SET_REPLACEMENT,
        &args![shared, shape, replacement],
    );
}

/// A convex translate shape (`0x30` bytes) around `child` whose translation is the vector at
/// `translation` with the fourth lane cleared.
fn make_convex_translate(e: &mut Engine, child: u32, translation: u32) -> u32 {
    let block = allocate_object(e, 0x30);
    let radius = quiet_bits(e, child + 0x10);
    e.call(
        CONVEX_TRANSLATE_SHAPE_CONSTRUCT,
        &args![block, 0xbu32, radius, child, 1u32],
    );
    e.mem.set_u32(block, CONVEX_TRANSLATE_VTABLE);
    let mask = load_bits(e, TRANSLATION_MASK);
    let source = load_bits(e, translation);
    store_bits(
        e,
        block + 0x20,
        std::array::from_fn(|lane| source[lane] & mask[lane]),
    );
    e.mem.set_u32(block + 0x1c, 0);
    block
}

/// A transform shape (`0x70` bytes) around `shape` with `transform`.
fn make_transform_shape(e: &mut Engine, shape: u32, transform: u32) -> u32 {
    let block = allocate_object(e, 0x70);
    e.call(TRANSFORM_SHAPE_CONSTRUCT, &args![block, shape, transform])
        .u32()
}

/// A convex transform shape (`0x60` bytes) around `child` with `transform`.
fn make_convex_transform_shape(e: &mut Engine, child: u32, transform: u32) -> u32 {
    let block = allocate_object(e, 0x60);
    e.call(
        CONVEX_TRANSFORM_SHAPE_CONSTRUCT,
        &args![block, child, transform, 1u32],
    )
    .u32()
}

/// Runs a comparison that writes a flag byte and answers the flag's pointer; the answer is
/// the byte behind that pointer.
fn comparison_holds(e: &mut Engine, function: u32, words: &[u32]) -> bool {
    let flag = e.call(function, words).u32();
    e.mem.u8(flag) != 0
}

// Translated from 00d74980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `getUseCount` (name inferred; `thiscall`, `this` = the shared array, `(key)`): the use count
/// of the first entry whose shape or replacement is `key`, 0 when there is none.
pub fn shared_shapes_get_use_count(e: &mut Engine, this: Ptr, key: Ptr) -> u32 {
    let this = this.addr();
    match find_entry(e, this, key.addr(), true) {
        Some(index) => {
            let entry = entry_address(e, this, index);
            e.mem.u32(entry + 8)
        }
        None => 0,
    }
}

// Translated from 00d749c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `setReplacement` (name inferred; `thiscall`, `this` = the shared array, `(shape,
/// replacement)`): the first entry whose shape or replacement is `shape` gets `replacement`
/// (one more reference on it, one less on the one it had). Answers 0, or 1 when there is no
/// such entry.
pub fn shared_shapes_set_replacement(
    e: &mut Engine,
    this: Ptr,
    shape: Ptr,
    replacement: Ptr,
) -> u32 {
    let this = this.addr();
    let replacement = replacement.addr();
    let Some(index) = find_entry(e, this, shape.addr(), true) else {
        return 1;
    };
    add_reference(e, replacement);
    let old = e.mem.u32(entry_address(e, this, index) + 4);
    if old != 0 {
        remove_reference(e, old);
    }
    let entry = entry_address(e, this, index);
    e.mem.set_u32(entry + 4, replacement);
    0
}

// Translated from 00d74aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `clear` of the shared array (name inferred; `fastcall`, `this`): releases the shape and the
/// replacement of every entry, then the storage, and leaves an empty array that owns nothing.
pub fn shared_shapes_clear(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let mut index = 0i32;
    let mut offset = 0u32;
    while index < signed_size(e, this) {
        let shape = e.mem.u32(e.mem.u32(this).wrapping_add(offset));
        if shape != 0 {
            remove_reference(e, shape);
        }
        let replacement = e
            .mem
            .u32(e.mem.u32(this).wrapping_add(offset).wrapping_add(4));
        if replacement != 0 {
            remove_reference(e, replacement);
        }
        index += 1;
        offset = offset.wrapping_add(12);
    }
    release_array(e, this, 12);
    e.mem.set_u32(this + 8, NOT_OWNED);
    e.mem.set_u32(this, 0);
}

// Translated from 00d74b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `build` of the shared array from shape holders (name inferred; `thiscall`, `this` = the new
/// array, `(holders)` an array of pointers to holders, each with its shape at `+0x10`): starts
/// empty, adds every shape with its children, then runs `00d74a20`. Answers `this`.
pub fn shared_shapes_build(e: &mut Engine, this: Ptr, holders: Ptr) -> Ptr {
    let (array, holders) = (this.addr(), holders.addr());
    set_empty_array(e, array);
    let mut index = 0i32;
    while index < signed_size(e, holders) {
        let holder = e.mem.u32(e.mem.u32(holders).wrapping_add(index as u32 * 4));
        let shape = e.mem.u32(holder + 0x10);
        e.call(SHARED_SHAPES_ADD_SHAPE, &args![array, shape]);
        index += 1;
    }
    e.call(SHARED_SHAPES_FINISH, &args![array]);
    this
}

// Translated from 00d737d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `collectChildShapes` (name inferred; `cdecl` with the array in `ESI`: `(array, shape)`):
/// appends the direct children of `shape` to the array of words: every child of a list shape
/// (kind 9; the child count is the virtual `4` of the list part at `+0x10`, the children are
/// 0x10 bytes apart behind the pointer at `+0x18`), the child at `+0x18` of a convex
/// translate (kind 0xb) or convex transform shape (kind 0xc) and the child at `+0x14` of a
/// transform shape (kind 0xf). Other kinds have none.
pub fn collect_child_shapes(e: &mut Engine, array: Ptr, shape: Ptr) {
    let (array, shape) = (array.addr(), shape.addr());
    match e.mem.u32(shape + 0xc).wrapping_sub(KIND_LIST) {
        0 => {
            let list = shape + 0x10;
            let count = e.vcall(list, 4, &args![]).i32();
            if count > 0 {
                let mut index = 0i32;
                let mut offset = 0u32;
                loop {
                    let children = e.mem.u32(shape + 0x18);
                    let child = e.mem.u32(children.wrapping_add(offset));
                    array_push_word(e, array, child);
                    index += 1;
                    offset = offset.wrapping_add(0x10);
                    let count = e.vcall(list, 4, &args![]).i32();
                    if index >= count {
                        break;
                    }
                }
            }
        }
        2 | 3 => {
            let child = e.mem.u32(shape + 0x18);
            array_push_word(e, array, child);
        }
        6 => {
            let child = e.mem.u32(shape + 0x14);
            array_push_word(e, array, child);
        }
        _ => {}
    }
}

// Translated from 00d738f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `addShape` of the shared array (name inferred; `thiscall`, `this` = the shared array,
/// `(shape)`): when `shape` already has an entry its use count goes up and the answer is the
/// address of that count. Otherwise a new entry `{shape, 0, 1}` is appended (grow by one when
/// full), the shape gets a reference and its children (`collectChildShapes`) are added the same
/// way. The answer of that path is what the exe leaves in `EAX` and the callers ignore it: the
/// capacity word of the child array when it does not own its storage, else the answer of the
/// free.
pub fn shared_shapes_add_shape(e: &mut Engine, this: Ptr, shape: Ptr) -> u32 {
    let (array, shape) = (this.addr(), shape.addr());
    if let Some(index) = find_entry(e, array, shape, false) {
        let entry = entry_address(e, array, index);
        let count = e.mem.u32(entry + 8).wrapping_add(1);
        e.mem.set_u32(entry + 8, count);
        return entry + 8;
    }
    let allocator = allocator(e);
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_GROW_BY_ONE, &args![allocator, array, 12u32]);
    }
    let size = e.mem.u32(array + 4);
    let entry = e.mem.u32(array).wrapping_add(size.wrapping_mul(12));
    if entry != 0 {
        e.mem.set_u32(entry, shape);
        e.mem.set_u32(entry + 4, 0);
        e.mem.set_u32(entry + 8, 1);
    }
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    add_reference(e, shape);
    e.with_stack(0xc, |e, children| {
        let children = children.addr();
        set_empty_array(e, children);
        e.call(COLLECT_CHILD_SHAPES, &args![children, shape]);
        let mut index = 0i32;
        let mut offset = 0u32;
        while index < signed_size(e, children) {
            let child = e.mem.u32(e.mem.u32(children).wrapping_add(offset));
            e.call(SHARED_SHAPES_ADD_SHAPE, &args![array, child]);
            index += 1;
            offset = offset.wrapping_add(4);
        }
        release_array(e, children, 4)
    })
}

// Translated from 00d73590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `transformTransformShape` (name inferred; `cdecl`, `(shape, transform)`): the shape that
/// `shape` becomes when `transform` is applied to it. Answers `shape` (one more reference)
/// when the transform is the identity; for a convex translate shape (kind 0xb) whose rotation
/// part is the identity, a convex transform shape (kind 0xc) or a transform shape (kind 0xf)
/// the transforms are multiplied and, when the product is the identity, the child (one more
/// reference) is the answer. Otherwise a new shape: a convex translate (0x30 bytes) for a
/// translate shape, a convex transform shape (0x60) for kind 0xc, and a transform shape
/// (0x70 bytes) for kind 0xf and for every other kind.
pub fn transform_transform_shape(e: &mut Engine, shape: Ptr, transform: Ptr) -> u32 {
    let (shape, transform) = (shape.addr(), transform.addr());
    e.with_stack(0x90, |e, frame| {
        let frame = frame.addr();
        let epsilon = quiet_bits(e, EPSILON);
        let identity = comparison_holds(
            e,
            TRANSFORM_IS_APPROX_EQUAL,
            &args![transform, frame + 0xc, IDENTITY_TRANSFORM, epsilon],
        );
        if identity {
            return add_reference_and_answer(e, shape);
        }
        match e.mem.u32(shape + 0xc) {
            KIND_CONVEX_TRANSLATE => {
                let plain_rotation = comparison_holds(
                    e,
                    MATRIX3_IS_APPROX_EQUAL,
                    &args![transform, frame + 0xd, IDENTITY_TRANSFORM, epsilon],
                );
                if !plain_rotation {
                    return make_transform_shape(e, shape, transform);
                }
                let translate = frame + 0x10;
                store_vec(e, translate, [1.0, 0.0, 0.0, 0.0]);
                store_vec(e, translate + 0x10, [0.0, 1.0, 0.0, 0.0]);
                store_vec(e, translate + 0x20, [0.0, 0.0, 1.0, 0.0]);
                copy_vec(e, shape + 0x20, translate + 0x30);
                let product = frame + 0x50;
                e.call(TRANSFORM_SET_MUL, &args![product, transform, translate]);
                let identity = comparison_holds(
                    e,
                    TRANSFORM_IS_APPROX_EQUAL,
                    &args![product, frame + 0xe, IDENTITY_TRANSFORM, epsilon],
                );
                let child = e.mem.u32(shape + 0x18);
                if identity {
                    return add_reference_and_answer(e, child);
                }
                make_convex_translate(e, child, shape + 0x20)
            }
            KIND_CONVEX_TRANSFORM => {
                let product = frame + 0x10;
                e.call(TRANSFORM_SET_MUL, &args![product, transform, shape + 0x20]);
                let identity = comparison_holds(
                    e,
                    TRANSFORM_IS_APPROX_EQUAL,
                    &args![product, frame + 0xf, IDENTITY_TRANSFORM, epsilon],
                );
                let child = e.mem.u32(shape + 0x18);
                if identity {
                    return add_reference_and_answer(e, child);
                }
                make_convex_transform_shape(e, child, product)
            }
            KIND_TRANSFORM => {
                let product = frame + 0x10;
                e.call(TRANSFORM_SET_MUL, &args![product, transform, shape + 0x30]);
                let identity = comparison_holds(
                    e,
                    TRANSFORM_IS_APPROX_EQUAL,
                    &args![product, frame + 0xc, IDENTITY_TRANSFORM, epsilon],
                );
                let child = e.mem.u32(shape + 0x14);
                if identity {
                    return add_reference_and_answer(e, child);
                }
                make_transform_shape(e, child, product)
            }
            _ => make_transform_shape(e, shape, transform),
        }
    })
}

/// What `collapseShapesRecursively` passes down unchanged: the policy, the shared array and the
/// counters.
#[derive(Clone, Copy)]
struct Recursion {
    policy: u32,
    shared: u32,
    counters: u32,
}

impl Recursion {
    fn collapse(&self, e: &mut Engine, shape: u32) -> u32 {
        e.call(
            COLLAPSE_SHAPES_RECURSIVELY,
            &args![shape, self.policy, self.shared, self.counters],
        )
        .u32()
    }

    /// The case of a shape with one child: the child collapses; when it changed, `rebuild`
    /// `(collapsed child, shape)` makes the new shape that becomes the replacement. Either way
    /// the shape (new or old) then goes through `collapseTransformShape` and the reference
    /// taken for it goes again.
    fn through_child(
        &self,
        e: &mut Engine,
        shape: u32,
        child: u32,
        rebuild: fn(&mut Engine, u32, u32) -> u32,
    ) -> u32 {
        let collapsed = self.collapse(e, child);
        let current = if collapsed == child {
            remove_reference(e, collapsed);
            add_reference(e, shape);
            shape
        } else {
            let replacement = rebuild(e, collapsed, shape);
            remove_reference(e, collapsed);
            set_replacement(e, self.shared, shape, replacement);
            replacement
        };
        let answer = e
            .call(
                COLLAPSE_TRANSFORM_SHAPE,
                &args![current, self.policy, self.shared, self.counters],
            )
            .u32();
        remove_reference(e, current);
        answer
    }

    /// The case of a list shape: its children collapse one by one. When any changed, a new list
    /// shape (`0x70` bytes) of the results becomes the replacement; otherwise the shape stays.
    fn collapse_list(&self, e: &mut Engine, shape: u32) -> u32 {
        let list = shape + 0x10;
        let count = e.vcall(list, 4, &args![]).i32();
        e.with_stack(0xc, |e, results| {
            let results = results.addr();
            set_empty_array(e, results);
            let mut unchanged = true;
            if count > 0 {
                let mut offset = 0u32;
                let mut remaining = count;
                loop {
                    let children = e.mem.u32(shape + 0x18);
                    let child = e.mem.u32(children.wrapping_add(offset));
                    let collapsed = self.collapse(e, child);
                    array_push_word(e, results, collapsed);
                    if collapsed != child {
                        unchanged = false;
                    }
                    offset = offset.wrapping_add(0x10);
                    remaining -= 1;
                    if remaining == 0 {
                        break;
                    }
                }
            }
            if !unchanged {
                let block = allocate_object(e, 0x70);
                let data = e.mem.u32(results);
                let replacement = e
                    .call(LIST_SHAPE_CONSTRUCT, &args![block, data, count, 1u32])
                    .u32();
                release_results(e, results, count);
                set_replacement(e, self.shared, shape, replacement);
                release_array(e, results, 4);
                return replacement;
            }
            release_results(e, results, count);
            add_reference(e, shape);
            release_array(e, results, 4);
            shape
        })
    }
}

/// One reference less on each of the first `count` words of the array of results.
fn release_results(e: &mut Engine, results: u32, count: i32) {
    let mut index = 0i32;
    while index < count {
        let data = e.mem.u32(results);
        let collapsed = e.mem.u32(data.wrapping_add(index as u32 * 4));
        remove_reference(e, collapsed);
        index += 1;
    }
}

// Translated from 00d74480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `collapseShapesRecursively` (name inferred; `cdecl`, `(shape, policy, shared, counters)`):
/// answers the shape `shape` collapses to, with a reference taken for the caller. A shape that
/// has a replacement in the shared array is answered by it. A list shape (kind 9) collapses its
/// children one by one; when any changed, a new list shape (`0x70` bytes) of the results
/// becomes its replacement, otherwise the shape stays. A transform shape (kind 0xf), convex
/// transform shape (kind 0xc) or convex translate shape (kind 0xb) collapses its child; when
/// that changed, a new shape of the same kind around the result becomes the replacement, and
/// either way the shape (the new one, or the old one) goes through `collapseTransformShape`.
/// Every other kind stays as it is.
pub fn collapse_shapes_recursively(
    e: &mut Engine,
    shape: Ptr,
    policy: Ptr,
    shared: Ptr,
    counters: Ptr,
) -> u32 {
    let (shape, policy, shared, counters) =
        (shape.addr(), policy.addr(), shared.addr(), counters.addr());
    if let Some(index) = find_entry(e, shared, shape, false) {
        let replacement = e.mem.u32(entry_address(e, shared, index) + 4);
        if replacement != 0 {
            return add_reference_and_answer(e, replacement);
        }
    }
    let recursion = Recursion {
        policy,
        shared,
        counters,
    };
    match e.mem.u32(shape + 0xc).wrapping_sub(KIND_LIST) {
        0 => recursion.collapse_list(e, shape),
        2 => {
            let child = e.mem.u32(shape + 0x18);
            recursion.through_child(e, shape, child, |e, collapsed, shape| {
                make_convex_translate(e, collapsed, shape + 0x20)
            })
        }
        3 => {
            let child = e.mem.u32(shape + 0x18);
            recursion.through_child(e, shape, child, |e, collapsed, shape| {
                make_convex_transform_shape(e, collapsed, shape + 0x20)
            })
        }
        6 => {
            let child = e.mem.u32(shape + 0x14);
            recursion.through_child(e, shape, child, |e, collapsed, shape| {
                make_transform_shape(e, collapsed, shape + 0x30)
            })
        }
        _ => add_reference_and_answer(e, shape),
    }
}

// Translated from 00d74850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `collapseHolders` (name inferred; `cdecl`, `(holders, policy, counters)`): resets the
/// counters (`00d73400`), builds the shared array from the holders, and replaces the shape of
/// each holder (field `+0x10`, one reference less on the old one) by what it collapses to.
/// Answers 0.
pub fn collapse_holders(e: &mut Engine, holders: Ptr, policy: Ptr, counters: Ptr) -> u32 {
    let (holders, policy, counters) = (holders.addr(), policy.addr(), counters.addr());
    e.call(COUNTERS_RESET, &args![counters]);
    e.with_stack(0xc, |e, shared| {
        let shared = shared.addr();
        e.call(SHARED_SHAPES_BUILD, &args![shared, holders]);
        let mut index = 0i32;
        while index < signed_size(e, holders) {
            let holder = e.mem.u32(e.mem.u32(holders).wrapping_add(index as u32 * 4));
            let old = e.mem.u32(holder + 0x10);
            let new = e
                .call(
                    COLLAPSE_SHAPES_RECURSIVELY,
                    &args![old, policy, shared, counters],
                )
                .u32();
            remove_reference(e, old);
            e.mem.set_u32(holder + 0x10, new);
            index += 1;
        }
        e.call(SHARED_SHAPES_CLEAR, &args![shared]);
    });
    0
}

// Translated from 00d748d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `collapseHolder` (name inferred; `cdecl`, `(holder, policy, counters)`): `collapseHolders`
/// for one holder, put in an array of one. Answers what that answers (0).
pub fn collapse_holder(e: &mut Engine, holder: Ptr, policy: Ptr, counters: Ptr) -> u32 {
    let holder = holder.addr();
    let (policy, counters) = (policy.addr(), counters.addr());
    e.with_stack(0xc, |e, holders| {
        let holders = holders.addr();
        set_empty_array(e, holders);
        array_push_word(e, holders, holder);
        let answer = e
            .call(COLLAPSE_HOLDERS, &args![holders, policy, counters])
            .u32();
        release_array(e, holders, 4);
        answer
    })
}

// Translated from 00d74420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `collapseShape` (name inferred; `cdecl`, `(shape, policy, counters, out)`): collapses one
/// shape with a shared array built for it alone (the shape and its children added, then
/// `00d74a20`), stores the answer at `*out` and clears the array. Answers 0.
pub fn collapse_shape_with_new_shared_data(
    e: &mut Engine,
    shape: Ptr,
    policy: Ptr,
    counters: Ptr,
    out: Ptr,
) -> u32 {
    let (shape, policy, counters, out) = (shape.addr(), policy.addr(), counters.addr(), out.addr());
    e.with_stack(0xc, |e, shared| {
        let shared = shared.addr();
        set_empty_array(e, shared);
        e.call(SHARED_SHAPES_ADD_SHAPE, &args![shared, shape]);
        e.call(SHARED_SHAPES_FINISH, &args![shared]);
        let collapsed = e
            .call(
                COLLAPSE_TRANSFORM_SHAPE,
                &args![shape, policy, shared, counters],
            )
            .u32();
        e.mem.set_u32(out, collapsed);
        e.call(SHARED_SHAPES_CLEAR, &args![shared]);
    });
    0
}

/// The state of one `collapseTransformShape` call: the shape (a convex translate, convex
/// transform or transform shape), its child, and the frame that holds the shape's transform
/// as four columns at `+0x70`, `+0x80`, `+0x90` and the translation at `+0xa0`.
#[derive(Clone, Copy)]
struct Collapse {
    shape: u32,
    shape_type: u32,
    policy: u32,
    shared: u32,
    counters: u32,
    frame: u32,
    child: u32,
    child_type: u32,
}

impl Collapse {
    fn transform(&self) -> u32 {
        self.frame + FRAME_TRANSFORM
    }

    fn translation(&self) -> u32 {
        self.frame + FRAME_TRANSLATION
    }

    /// The exit of the shapes that cannot be collapsed: the shape itself, one more reference.
    fn keep_shape(&self, e: &mut Engine) -> u32 {
        add_reference_and_answer(e, self.shape)
    }

    /// The shape gets `replacement` and counter `counter` goes up; the answer is the new shape.
    fn replace_with(&self, e: &mut Engine, replacement: u32, counter: u32) -> u32 {
        set_replacement(e, self.shared, self.shape, replacement);
        bump_counter(e, self.counters, counter);
        replacement
    }

    /// List child (kind 9): unless the policy forbids it (byte flag at `+8`, or `allowed` is
    /// false) a transform shape (`0x70` bytes) is made for each child of the list: the child
    /// under the shape's transform, collapsed on its own. The results become a new list shape,
    /// the replacement of the shape (counter 3).
    fn rebuild_list(&self, e: &mut Engine, allowed: bool) -> u32 {
        if e.mem.u8(self.policy + 8) != 0 || !allowed {
            return self.keep_shape(e);
        }
        bump_counter(e, self.counters, 3);
        let (shape, child) = (self.shape, self.child);
        let list = child + 0x10;
        e.with_stack(0xc, |e, results| {
            let results = results.addr();
            set_empty_array(e, results);
            let count = e.vcall(list, 4, &args![]).i32();
            if count > 0 {
                let mut index = 0i32;
                let mut offset = 0u32;
                loop {
                    let children = e.mem.u32(child + 0x18);
                    let element = e.mem.u32(children.wrapping_add(offset));
                    let wrapper = make_transform_shape(e, element, self.transform());
                    let collapsed = e
                        .call(
                            COLLAPSE_TRANSFORM_SHAPE,
                            &args![wrapper, self.policy, self.shared, self.counters],
                        )
                        .u32();
                    array_push_word(e, results, collapsed);
                    remove_reference(e, wrapper);
                    index += 1;
                    offset = offset.wrapping_add(0x10);
                    let count = e.vcall(list, 4, &args![]).i32();
                    if index >= count {
                        break;
                    }
                }
            }
            let block = allocate_object(e, 0x70);
            let size = e.mem.u32(results + 4);
            let data = e.mem.u32(results);
            let replacement = e
                .call(LIST_SHAPE_CONSTRUCT, &args![block, data, size, 1u32])
                .u32();
            let mut index = 0i32;
            while index < signed_size(e, results) {
                let data = e.mem.u32(results);
                let collapsed = e.mem.u32(data.wrapping_add(index as u32 * 4));
                remove_reference(e, collapsed);
                index += 1;
            }
            set_replacement(e, self.shared, shape, replacement);
            release_array(e, results, 4);
            replacement
        })
    }

    /// Capsule child (kind 5): both end points go through the transform, the radius stays.
    fn capsule_case(&self, e: &mut Engine) -> u32 {
        let (frame, child) = (self.frame, self.child);
        let radius = quiet_bits(e, child + 0x10);
        e.call(
            SET_TRANSFORMED_POS,
            &args![frame + FRAME_STRIDED, self.transform(), child + 0x20],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![frame + FRAME_PLANE, self.transform(), child + 0x30],
        );
        let block = allocate_object(e, 0x40);
        let capsule = e
            .call(
                CAPSULE_SHAPE_CONSTRUCT,
                &args![block, frame + FRAME_STRIDED, frame + FRAME_PLANE, radius],
            )
            .u32();
        self.replace_with(e, capsule, 0)
    }

    /// Cylinder child (kind 2): both end points go through the transform; the cylinder radius
    /// (`00d571a0`) and the convex radius (`+0x10`) stay.
    fn cylinder_case(&self, e: &mut Engine) -> u32 {
        let (frame, child) = (self.frame, self.child);
        let cylinder_radius = x87_quiet(e.call(CYLINDER_RADIUS, &args![child]).f32());
        let radius = quiet_bits(e, child + 0x10);
        e.call(
            SET_TRANSFORMED_POS,
            &args![frame + FRAME_STRIDED, self.transform(), child + 0x20],
        );
        e.call(
            SET_TRANSFORMED_POS,
            &args![frame + FRAME_PLANE, self.transform(), child + 0x30],
        );
        let block = allocate_object(e, 0x60);
        let cylinder = e
            .call(
                CYLINDER_SHAPE_CONSTRUCT,
                &args![
                    block,
                    frame + FRAME_STRIDED,
                    frame + FRAME_PLANE,
                    cylinder_radius,
                    radius
                ],
            )
            .u32();
        self.replace_with(e, cylinder, 0)
    }

    /// Box child (kind 4) under a transform whose rotation is orthonormal and not the
    /// identity: when every column holds one entry of magnitude 1 (within 0.0001) and the
    /// others 0, the rotation is a permutation of the axes, and a new box (radius 0.05) with the
    /// half extents permuted replaces the shape; a translation of 0.0001 or more is kept by a
    /// convex translate around the box. Any other rotation falls to `matrix_case`.
    fn box_case(&self, e: &mut Engine, epsilon: u32) -> u32 {
        let (frame, child) = (self.frame, self.child);
        let orthonormal = comparison_holds(
            e,
            MATRIX3_IS_ORTHONORMAL,
            &args![self.transform(), frame + 0x43, epsilon],
        );
        if !orthonormal {
            return self.matrix_case(e);
        }
        let valid = comparison_holds(e, MATRIX3_IS_OK, &args![self.transform(), frame + 0x42]);
        if !valid {
            return self.matrix_case(e);
        }
        let identity = comparison_holds(
            e,
            MATRIX3_IS_APPROX_EQUAL,
            &args![self.transform(), frame + 0x3b, IDENTITY_TRANSFORM, epsilon],
        );
        if identity {
            return self.matrix_case(e);
        }
        let tolerance = e.mem.f64(AXIS_TOLERANCE);
        // The axis each row of the rotation picks, filled in while the columns are scanned. An
        // orthonormal rotation fills all three; for any other the exe would read the stack.
        let picks = frame + FRAME_STRIDED;
        let mut column = self.transform();
        for outer in 0..3u32 {
            let mut found = false;
            for inner in 0..3u32 {
                let magnitude = e.mem.f32(column + 4 * inner).abs();
                let near_zero = f64::from(magnitude) < tolerance;
                let near_one = f64::from((magnitude - 1.0).abs()) < tolerance;
                if near_one {
                    e.mem.set_u32(picks + 4 * inner, outer);
                    found = true;
                } else if !near_zero {
                    // An entry that is neither 0 nor 1: the scan of this column stops here.
                    return self.matrix_case(e);
                }
            }
            if !found {
                return self.matrix_case(e);
            }
            let by_y = e.mem.f32(column + 4).abs();
            let by_x = e.mem.f32(column).abs();
            let by_z = e.mem.f32(column + 8).abs();
            let total = (by_y + by_x) + by_z;
            let within = f64::from(total) - 1.0 < tolerance;
            if !within {
                return self.matrix_case(e);
            }
            column += 0x10;
        }
        let extent =
            |e: &Engine, row: u32| e.mem.u32(child + 0x20 + 4 * e.mem.u32(picks + 4 * row));
        let half_extents = [
            extent(e, 0),
            extent(e, 1),
            extent(e, 2),
            e.mem.u32(ZERO_F32),
        ];
        store_bits(e, frame + FRAME_COPY, half_extents);
        let block = allocate_object(e, 0x30);
        let radius = quiet_bits(e, BOX_RADIUS);
        let new_box = e
            .call(
                BOX_SHAPE_CONSTRUCT,
                &args![block, frame + FRAME_COPY, radius],
            )
            .u32();
        let translation = load_vec(e, self.translation());
        let length = sum3(mul(translation, translation)).sqrt();
        if f64::from(length) < tolerance {
            return self.replace_with(e, new_box, 0);
        }
        let wrapper = make_convex_translate(e, new_box, self.translation());
        remove_reference(e, new_box);
        self.replace_with(e, wrapper, 2)
    }

    /// Convex vertices child (kind 6): the vertices go through the transform, the plane
    /// equations are rotated and their offsets moved by the translation, and a new convex
    /// vertices shape (`0x70` bytes, the old radius) replaces the shape.
    fn convex_vertices_case(&self, e: &mut Engine) -> u32 {
        let (frame, child) = (self.frame, self.child);
        let vertices = frame + FRAME_VERTICES;
        set_empty_array(e, vertices);
        e.call(GET_ORIGINAL_VERTICES, &args![child, vertices]);
        let columns = [
            load_vec(e, self.transform()),
            load_vec(e, self.transform() + 0x10),
            load_vec(e, self.transform() + 0x20),
            load_vec(e, self.translation()),
        ];
        // The loop runs from the last vertex to the first and tests after the body, so an
        // empty array is read one vertex before its start, as the exe does.
        let mut remaining = (e.mem.u32(vertices + 4) as i32).wrapping_sub(1);
        let mut address = (remaining as u32)
            .wrapping_shl(4)
            .wrapping_add(e.mem.u32(vertices));
        loop {
            let vertex = load_vec(e, address);
            let by_xy = add(
                mul(splat(vertex, 1), columns[1]),
                mul(splat(vertex, 0), columns[0]),
            );
            let by_z = add(mul(splat(vertex, 2), columns[2]), columns[3]);
            store_vec(e, address, add(by_xy, by_z));
            remaining = remaining.wrapping_sub(1);
            address = address.wrapping_sub(0x10);
            if remaining < 0 {
                break;
            }
        }
        let strided = frame + FRAME_STRIDED;
        let data = e.mem.u32(vertices);
        let size = e.mem.u32(vertices + 4);
        e.mem.set_u32(strided, data);
        e.mem.set_u32(strided + 4, size);
        e.mem.set_u32(strided + 8, 0x10);
        let source = e.call(GET_PLANE_EQUATIONS, &args![child]).u32();
        let planes = frame + FRAME_PLANES;
        set_empty_array(e, planes);
        let plane_count = e.mem.u32(source + 4) as i32;
        let allocator = allocator(e);
        let capacity = (e.mem.u32(planes + 8) & CAPACITY_MASK) as i32;
        if capacity < plane_count {
            let wanted = capacity.wrapping_add(capacity).max(plane_count);
            e.call(ARRAY_RESERVE, &args![allocator, planes, wanted, 0x10u32]);
        }
        copy_vec(e, self.translation(), frame + FRAME_COPY);
        e.mem.set_u32(planes + 4, plane_count as u32);
        if e.mem.u32(source + 4) as i32 > 0 {
            let mut index = 0i32;
            let mut offset = 0u32;
            loop {
                let plane = e.mem.u32(source).wrapping_add(offset);
                copy_vec(e, plane, frame + FRAME_PLANE);
                let distance = e.mem.f32(frame + FRAME_PLANE + 0xc);
                let moved = mul(
                    load_vec(e, frame + FRAME_PLANE),
                    load_vec(e, frame + FRAME_COPY),
                );
                let adjusted = x87_quiet(distance) - sum3(moved);
                e.call(
                    SET_ROTATED_DIR,
                    &args![frame + FRAME_PLANE, self.transform(), frame + FRAME_PLANE],
                );
                e.mem.set_u32(
                    frame + FRAME_PLANE + 0xc,
                    x87_quiet_bits(adjusted.to_bits()),
                );
                let out = e.mem.u32(planes);
                copy_vec(e, frame + FRAME_PLANE, out.wrapping_add(offset));
                index += 1;
                offset = offset.wrapping_add(0x10);
                if index >= e.mem.u32(source + 4) as i32 {
                    break;
                }
            }
        }
        let block = allocate_object(e, 0x70);
        let radius = quiet_bits(e, child + 0x10);
        let shape = e
            .call(
                CONVEX_VERTICES_SHAPE_CONSTRUCT,
                &args![block, strided, planes, radius],
            )
            .u32();
        let answer = self.replace_with(e, shape, 0);
        release_array(e, planes, 0x10);
        e.mem.set_u32(planes, 0);
        e.mem.set_u32(planes + 8, NOT_OWNED);
        release_array(e, vertices, 0x10);
        answer
    }

    /// What is left for the kinds without a case of their own (and for every case that could
    /// not be collapsed): a sphere, cylinder, box, capsule, convex vertices or kind-0x12 child
    /// under a transform with the identity rotation is wrapped in a convex translate (counter
    /// 2), unless the shape is itself a convex translate; a transform shape (kind 0xf) becomes
    /// a convex transform shape (counter 2); anything else is kept.
    fn matrix_case(&self, e: &mut Engine) -> u32 {
        // The exe checks `kind - 1 <= 0x11` and then a table of bytes; the kinds it picks are
        // the sphere (1), cylinder (2), box (4), capsule (5), convex vertices (6) and 0x12.
        let picked = matches!(self.child_type, 1 | 2 | 4 | 5 | 6 | 0x12);
        if !picked {
            return self.keep_shape(e);
        }
        let frame = self.frame;
        let epsilon = quiet_bits(e, EPSILON);
        e.call(
            MATRIX3_IS_APPROX_EQUAL,
            &args![
                self.transform(),
                frame + FRAME_FLAG,
                IDENTITY_TRANSFORM,
                epsilon
            ],
        );
        let identity_rotation = e.mem.u8(frame + FRAME_FLAG) != 0;
        if identity_rotation && self.shape_type != KIND_CONVEX_TRANSLATE {
            let wrapper = make_convex_translate(e, self.child, self.translation());
            return self.replace_with(e, wrapper, 2);
        }
        if self.shape_type != KIND_TRANSFORM {
            return self.keep_shape(e);
        }
        let replacement = make_convex_transform_shape(e, self.child, self.transform());
        self.replace_with(e, replacement, 2)
    }
}

// Translated from 00d739f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `collapseTransformShape` (name inferred; `cdecl`, `(shape, policy, shared, counters)`):
/// folds the transform of `shape` (a convex translate shape, kind 0xb; a convex transform shape,
/// kind 0xc; or a transform shape, kind 0xf; any other kind is answered by itself with one more
/// reference) into its child. When the transform is the identity the child replaces the shape
/// (counter 1). The policy `(mode, limit, byte flag)` decides whether the child may be
/// rebuilt: mode 1 never, mode 2 only while the use count of the child in the shared array is
/// below `limit`, every other mode always. A list child (kind 9) is rebuilt as a list of the
/// transformed children unless the byte flag is set (counter 3); a cylinder (2), capsule (5)
/// or convex vertices (6) child is rebuilt with transformed points and a box child (4) with
/// permuted half extents (counter 0); the other kinds are kept, wrapped (counter 2) when the
/// rotation part is the identity. The new shape is the shape's replacement in the shared array.
pub fn collapse_transform_shape(
    e: &mut Engine,
    shape: Ptr,
    policy: Ptr,
    shared: Ptr,
    counters: Ptr,
) -> u32 {
    let (shape, policy, shared, counters) =
        (shape.addr(), policy.addr(), shared.addr(), counters.addr());
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let shape_type = e.mem.u32(shape + 0xc);
        let transform = frame + FRAME_TRANSFORM;
        let child = match shape_type {
            KIND_CONVEX_TRANSLATE => {
                store_vec(e, transform, [1.0, 0.0, 0.0, 0.0]);
                store_vec(e, transform + 0x10, [0.0, 1.0, 0.0, 0.0]);
                store_vec(e, transform + 0x20, [0.0, 0.0, 1.0, 0.0]);
                copy_vec(e, shape + 0x20, frame + FRAME_TRANSLATION);
                e.mem.u32(shape + 0x18)
            }
            KIND_CONVEX_TRANSFORM => {
                for column in 0..4 {
                    copy_vec(e, shape + 0x20 + 0x10 * column, transform + 0x10 * column);
                }
                e.mem.u32(shape + 0x18)
            }
            KIND_TRANSFORM => {
                for column in 0..4 {
                    copy_vec(e, shape + 0x30 + 0x10 * column, transform + 0x10 * column);
                }
                e.mem.u32(shape + 0x14)
            }
            _ => return add_reference_and_answer(e, shape),
        };
        let child_type = e.mem.u32(child + 0xc);
        if child_type == KIND_SPHERE {
            for column in 0..3 {
                copy_vec(
                    e,
                    IDENTITY_TRANSFORM + 0x10 * column,
                    transform + 0x10 * column,
                );
            }
        }
        let state = Collapse {
            shape,
            shape_type,
            policy,
            shared,
            counters,
            frame,
            child,
            child_type,
        };
        let epsilon = quiet_bits(e, EPSILON);
        let identity = comparison_holds(
            e,
            TRANSFORM_IS_APPROX_EQUAL,
            &args![transform, frame + FRAME_FLAG, IDENTITY_TRANSFORM, epsilon],
        );
        if identity {
            bump_counter(e, counters, 1);
            add_reference(e, child);
            set_replacement(e, shared, shape, child);
            return child;
        }
        let allowed = match e.mem.u32(policy) {
            1 => false,
            2 => {
                let uses = e
                    .call(SHARED_SHAPES_GET_USE_COUNT, &args![shared, child])
                    .i32();
                uses < e.mem.u32(policy + 4) as i32
            }
            _ => true,
        };
        if child_type == KIND_LIST {
            return state.rebuild_list(e, allowed);
        }
        if !allowed {
            return state.matrix_case(e);
        }
        match child_type.wrapping_sub(2) {
            0 => state.cylinder_case(e),
            2 => state.box_case(e, epsilon),
            3 => state.capsule_case(e),
            4 => state.convex_vertices_case(e),
            _ => state.matrix_case(e),
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d7_3590, transform_transform_shape(Ptr, Ptr) -> u32),
        entry!(0x00d7_37d0, collect_child_shapes(Ptr, Ptr)),
        entry!(0x00d7_38f0, shared_shapes_add_shape(Ptr, Ptr) -> u32),
        entry!(0x00d7_39f0, collapse_transform_shape(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(
            0x00d7_4420,
            collapse_shape_with_new_shared_data(Ptr, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(0x00d7_4480, collapse_shapes_recursively(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d7_4850, collapse_holders(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d7_48d0, collapse_holder(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d7_4980, shared_shapes_get_use_count(Ptr, Ptr) -> u32),
        entry!(0x00d7_49c0, shared_shapes_set_replacement(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d7_4aa0, shared_shapes_clear(Ptr)),
        entry!(0x00d7_4b20, shared_shapes_build(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    /// The allocator's virtual 4 and 0x10 and the list shape's virtual 4 (the child count).
    const ALLOCATE: u32 = 0x0900_0004;
    const FREE: u32 = 0x0900_0010;
    const COUNT: u32 = 0x0900_0020;

    type Columns = [[f32; 4]; 4];
    type Captured = Rc<RefCell<Vec<[f32; 4]>>>;
    /// What the convex vertices constructor was given: the vertices, the planes, the stride and
    /// the address of the planes.
    type ConstructorSeen = (Vec<[f32; 4]>, Vec<[f32; 4]>, u32, u32);
    /// A box scene: the columns, whether they are orthonormal and valid, the answers of the
    /// identity comparisons.
    type BoxCase = (Columns, bool, bool, Vec<bool>);

    const IDENTITY_COLUMNS: Columns = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    fn record(log: &Log, address: u32, words: &[u32]) {
        log.borrow_mut().push((address, words.to_vec()));
    }

    /// A double that logs its arguments and answers what `answer` makes of them.
    fn logged(
        e: &mut Engine,
        log: &Log,
        address: u32,
        answer: impl Fn(&mut Engine, &[u32]) -> Ret + 'static,
    ) {
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            record(&log, address, a);
            answer(e, a)
        });
    }

    /// A constructor double: logs and answers `this`.
    fn constructor_double(e: &mut Engine, log: &Log, address: u32) {
        logged(e, log, address, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
    }

    /// A constructor double that also keeps the vectors its arguments (by index into the words,
    /// `this` first) point at, as they are when it is called.
    fn capturing_constructor(
        e: &mut Engine,
        log: &Log,
        address: u32,
        vector_arguments: &'static [usize],
    ) -> Captured {
        let captured: Captured = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&captured);
        logged(e, log, address, move |e, a| {
            for index in vector_arguments {
                sink.borrow_mut().push(load_vec(e, a[*index]));
            }
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        captured
    }

    /// A comparison double (`this`, flag pointer, ...): writes the next answer to the flag byte
    /// and answers the flag pointer.
    fn comparison_double(e: &mut Engine, log: &Log, address: u32, answers: &[bool]) {
        let queue = Rc::new(RefCell::new(VecDeque::from(answers.to_vec())));
        logged(e, log, address, move |e, a| {
            let answer = queue
                .borrow_mut()
                .pop_front()
                .expect("a comparison nobody expected");
            e.mem.set_u8(a[1], u8::from(answer));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
    }

    /// An engine with the constants this unit reads, and doubles for the memory router (its
    /// allocator hands out zeroed blocks), the grow-by-one and the reference counting.
    fn setup() -> (Engine, Log) {
        let mut e = engine();
        let log = new_log();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE);
        e.mem.set_u32(vtable + 0x10, FREE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(&mut e, &log, MEMORY_ROUTER, router);
        logged(&mut e, &log, ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        log_double(&mut e, &log, FREE, 0x77);
        logged(&mut e, &log, ARRAY_GROW_BY_ONE, |e, a| {
            let (array, element) = (a[1], a[2]);
            let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK).max(2) * 2;
            let size = e.mem.u32(array + 4);
            let old = e.mem.u32(array);
            let data = e.mem.alloc(capacity * element);
            if size > 0 {
                let bytes = e.mem.bytes(old, size * element);
                e.mem.write(data, &bytes);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        log_double(&mut e, &log, ADD_REFERENCE, 0);
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        e.mem.set_f32(EPSILON, 0.001);
        e.mem.set_f64(AXIS_TOLERANCE, 0.0001);
        e.mem.set_f32(BOX_RADIUS, 0.05);
        store_bits(
            &mut e,
            TRANSLATION_MASK,
            [0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0],
        );
        for (index, column) in IDENTITY_COLUMNS.iter().take(3).enumerate() {
            store_vec(&mut e, IDENTITY_TRANSFORM + 0x10 * index as u32, *column);
        }
        (e, log)
    }

    /// The `(data, bytes)` of every free the allocator saw.
    fn frees(log: &Log) -> Vec<(u32, u32)> {
        calls(log, FREE)
            .iter()
            .map(|words| (words[1], words[2]))
            .collect()
    }

    fn word_array(e: &mut Engine, words: &[u32]) -> u32 {
        let array = e.mem.alloc(0xc);
        let data = e.mem.alloc(4 * words.len().max(1) as u32);
        for (index, word) in words.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *word);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, words.len() as u32);
        e.mem.set_u32(array + 8, words.len() as u32);
        array
    }

    fn read_words(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    fn shared_array(e: &mut Engine, entries: &[(u32, u32, u32)]) -> u32 {
        let array = e.mem.alloc(0xc);
        let data = e.mem.alloc(12 * entries.len().max(1) as u32);
        for (index, (shape, replacement, uses)) in entries.iter().enumerate() {
            let entry = data + 12 * index as u32;
            e.mem.set_u32(entry, *shape);
            e.mem.set_u32(entry + 4, *replacement);
            e.mem.set_u32(entry + 8, *uses);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, entries.len() as u32);
        e.mem.set_u32(array + 8, entries.len() as u32);
        array
    }

    fn entry(e: &Engine, shared: u32, index: u32) -> (u32, u32, u32) {
        let entry = e.mem.u32(shared) + 12 * index;
        (e.mem.u32(entry), e.mem.u32(entry + 4), e.mem.u32(entry + 8))
    }

    /// A shape-like object of 0x100 zeroed bytes with its kind at `+0xc`.
    fn object(e: &mut Engine, kind: u32) -> u32 {
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0xc, kind);
        object
    }

    /// A list shape whose children (0x10 bytes apart behind the pointer at `+0x18`) are
    /// `children`; the virtual 4 of its list part answers their number.
    fn list_shape(e: &mut Engine, log: &Log, children: &[u32]) -> u32 {
        let count = children.len() as u32;
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, COUNT);
        logged(e, log, COUNT, move |_, _| Ret {
            eax: count,
            ..Ret::default()
        });
        let list = object(e, KIND_LIST);
        e.mem.set_u32(list + 0x10, vtable);
        let block = e.mem.alloc(0x10 * children.len().max(1) as u32);
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(block + 0x10 * index as u32, *child);
        }
        e.mem.set_u32(list + 0x18, block);
        list
    }

    /// A shape of kind 0xb (translation `columns[3]` at `+0x20`), 0xc (transform at `+0x20`) or
    /// 0xf (transform at `+0x30`) around `child`.
    fn transform_shape(e: &mut Engine, kind: u32, child: u32, columns: Columns) -> u32 {
        let shape = object(e, kind);
        match kind {
            KIND_TRANSFORM => {
                e.mem.set_u32(shape + 0x14, child);
                for (index, column) in columns.iter().enumerate() {
                    store_vec(e, shape + 0x30 + 0x10 * index as u32, *column);
                }
            }
            KIND_CONVEX_TRANSFORM => {
                e.mem.set_u32(shape + 0x18, child);
                for (index, column) in columns.iter().enumerate() {
                    store_vec(e, shape + 0x20 + 0x10 * index as u32, *column);
                }
            }
            _ => {
                e.mem.set_u32(shape + 0x18, child);
                store_vec(e, shape + 0x20, columns[3]);
            }
        }
        shape
    }

    fn make_policy(e: &mut Engine, mode: u32, limit: u32, flag: u8) -> u32 {
        let policy = e.mem.alloc(0x10);
        e.mem.set_u32(policy, mode);
        e.mem.set_u32(policy + 4, limit);
        e.mem.set_u8(policy + 8, flag);
        policy
    }

    fn counter_values(e: &Engine, counters: u32) -> Vec<u32> {
        (0..4)
            .map(|index| e.mem.u32(counters + 4 * index))
            .collect()
    }

    #[test]
    fn use_count_finds_the_shape_or_its_replacement() {
        let (mut e, _log) = setup();
        let shared = shared_array(&mut e, &[(0x100, 0x200, 3), (0x300, 0, 5)]);
        let get = |e: &mut Engine, key: u32| e.call(0x00d7_4980, &args![shared, key]).u32();
        assert_eq!(get(&mut e, 0x100), 3);
        assert_eq!(get(&mut e, 0x200), 3);
        assert_eq!(get(&mut e, 0x300), 5);
        assert_eq!(get(&mut e, 0x999), 0);
        let empty = shared_array(&mut e, &[]);
        assert_eq!(e.call(0x00d7_4980, &args![empty, 0x100u32]).u32(), 0);
    }

    #[test]
    fn set_replacement_takes_a_reference_on_the_new_shape_and_drops_the_old_one() {
        let (mut e, log) = setup();
        let shared = shared_array(&mut e, &[(0x100, 0x555, 1), (0x300, 0, 1)]);
        let set = |e: &mut Engine, shape: u32, replacement: u32| {
            e.call(0x00d7_49c0, &args![shared, shape, replacement])
                .u32()
        };
        // An entry without a replacement has nothing to drop.
        assert_eq!(set(&mut e, 0x300, 0x700), 0);
        assert_eq!(entry(&e, shared, 1), (0x300, 0x700, 1));
        assert!(calls(&log, REMOVE_REFERENCE).is_empty());
        // The replacement matches too; the old one loses a reference.
        assert_eq!(set(&mut e, 0x555, 0x800), 0);
        assert_eq!(entry(&e, shared, 0), (0x100, 0x800, 1));
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![0x555]]);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![0x700], vec![0x800]]);
        // No entry: answer 1 and no reference moves.
        assert_eq!(set(&mut e, 0x999, 0x900), 1);
        assert_eq!(calls(&log, ADD_REFERENCE).len(), 2);
        assert_eq!(calls(&log, REMOVE_REFERENCE).len(), 1);
    }

    #[test]
    fn clear_releases_what_the_entries_hold_and_frees_owned_storage() {
        let (mut e, log) = setup();
        let shared = shared_array(&mut e, &[(0x100, 0x200, 1), (0x300, 0, 2)]);
        let data = e.mem.u32(shared);
        e.mem.set_u32(shared + 8, 4);
        e.call(0x00d7_4aa0, &args![shared]);
        assert_eq!(
            calls(&log, REMOVE_REFERENCE),
            vec![vec![0x100], vec![0x200], vec![0x300]]
        );
        assert_eq!(frees(&log), vec![(data, 48)]);
        assert_eq!(read_words(&e, shared), Vec::<u32>::new());
        assert_eq!(e.mem.u32(shared), 0);
        assert_eq!(e.mem.u32(shared + 8), 0x8000_0000);
    }

    #[test]
    fn clear_leaves_storage_it_does_not_own_alone() {
        let (mut e, log) = setup();
        let shared = shared_array(&mut e, &[(0x100, 0, 1)]);
        e.mem.set_u32(shared + 8, 0x8000_0002);
        e.call(0x00d7_4aa0, &args![shared]);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![0x100]]);
        assert!(frees(&log).is_empty());
        assert_eq!(e.mem.u32(shared + 4), 0);
        assert_eq!(e.mem.u32(shared + 8), 0x8000_0000);
    }

    #[test]
    fn build_adds_the_shape_of_each_holder_and_finishes() {
        let (mut e, log) = setup();
        log_double(&mut e, &log, SHARED_SHAPES_ADD_SHAPE, 0);
        log_double(&mut e, &log, SHARED_SHAPES_FINISH, 0);
        let first = e.mem.alloc(0x20);
        e.mem.set_u32(first + 0x10, 0xa1);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(second + 0x10, 0xa2);
        let holders = word_array(&mut e, &[first, second]);
        let this = e.mem.alloc(0xc);
        for offset in [0, 4, 8] {
            e.mem.set_u32(this + offset, 9);
        }
        let got = e.call(0x00d7_4b20, &args![this, holders]).u32();
        assert_eq!(got, this);
        assert_eq!(
            calls(&log, SHARED_SHAPES_ADD_SHAPE),
            vec![vec![this, 0xa1], vec![this, 0xa2]]
        );
        assert_eq!(calls(&log, SHARED_SHAPES_FINISH), vec![vec![this]]);
        assert_eq!(
            [e.mem.u32(this), e.mem.u32(this + 4), e.mem.u32(this + 8)],
            [0, 0, 0x8000_0000]
        );
    }

    #[test]
    fn the_children_of_a_list_shape_are_collected_in_order() {
        let (mut e, log) = setup();
        let list = list_shape(&mut e, &log, &[0xc1, 0xc2, 0xc3]);
        let array = e.mem.alloc(0xc);
        e.mem.set_u32(array + 8, NOT_OWNED);
        e.call(0x00d7_37d0, &args![array, list]);
        assert_eq!(read_words(&e, array), vec![0xc1, 0xc2, 0xc3]);
        // The array started without storage: the first push grows it, the next two fit.
        assert_eq!(calls(&log, ARRAY_GROW_BY_ONE).len(), 1);
    }

    #[test]
    fn an_empty_list_shape_collects_nothing() {
        let (mut e, log) = setup();
        let list = list_shape(&mut e, &log, &[]);
        let array = word_array(&mut e, &[]);
        e.call(0x00d7_37d0, &args![array, list]);
        assert_eq!(read_words(&e, array), Vec::<u32>::new());
    }

    #[test]
    fn the_single_children_are_collected_from_their_fields() {
        let (mut e, _log) = setup();
        let array = word_array(&mut e, &[]);
        let translate = object(&mut e, KIND_CONVEX_TRANSLATE);
        e.mem.set_u32(translate + 0x18, 0xd1);
        let convex = object(&mut e, KIND_CONVEX_TRANSFORM);
        e.mem.set_u32(convex + 0x18, 0xd2);
        let transform = object(&mut e, KIND_TRANSFORM);
        e.mem.set_u32(transform + 0x14, 0xd3);
        e.mem.set_u32(transform + 0x18, 0xee);
        for shape in [translate, convex, transform] {
            e.call(0x00d7_37d0, &args![array, shape]);
        }
        // The kinds in between and the ones after have no children.
        for kind in [0xa, 0xd, 0xe, 0x10, 5] {
            let other = object(&mut e, kind);
            e.mem.set_u32(other + 0x18, 0xee);
            e.call(0x00d7_37d0, &args![array, other]);
        }
        assert_eq!(read_words(&e, array), vec![0xd1, 0xd2, 0xd3]);
    }

    #[test]
    fn add_shape_appends_the_shape_and_its_children_and_counts_repeats() {
        let (mut e, log) = setup();
        let shared = e.mem.alloc(0xc);
        e.mem.set_u32(shared + 8, NOT_OWNED);
        let child = object(&mut e, 5);
        let shape = object(&mut e, KIND_CONVEX_TRANSLATE);
        e.mem.set_u32(shape + 0x18, child);
        let got = e.call(0x00d7_38f0, &args![shared, shape]).u32();
        assert_eq!(e.mem.u32(shared + 4), 2);
        assert_eq!(entry(&e, shared, 0), (shape, 0, 1));
        assert_eq!(entry(&e, shared, 1), (child, 0, 1));
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![shape], vec![child]]);
        // The array of children grew to four words and its storage was freed: the answer of
        // that free is what the exe leaves behind.
        assert_eq!(got, 0x77);
        assert_eq!(
            frees(&log).iter().map(|free| free.1).collect::<Vec<_>>(),
            vec![16]
        );
        // Adding the shape again only counts it, and answers the address of the count.
        let again = e.call(0x00d7_38f0, &args![shared, shape]).u32();
        assert_eq!(again, e.mem.u32(shared) + 8);
        assert_eq!(entry(&e, shared, 0), (shape, 0, 2));
        assert_eq!(e.mem.u32(shared + 4), 2);
        assert_eq!(calls(&log, ADD_REFERENCE).len(), 2);
    }

    #[test]
    fn transforming_by_the_identity_keeps_the_shape() {
        let (mut e, log) = setup();
        comparison_double(&mut e, &log, TRANSFORM_IS_APPROX_EQUAL, &[true]);
        let shape = object(&mut e, KIND_TRANSFORM);
        let transform = e.mem.alloc(0x40);
        let got = e.call(0x00d7_3590, &args![shape, transform]).u32();
        assert_eq!(got, shape);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![shape]]);
        let compared = calls(&log, TRANSFORM_IS_APPROX_EQUAL);
        assert_eq!(compared.len(), 1);
        assert_eq!(compared[0][0], transform);
        assert_eq!(compared[0][2..], [IDENTITY_TRANSFORM, 0x3a83_126f]);
    }

    #[test]
    fn a_transform_shape_whose_product_is_the_identity_gives_up_its_child() {
        let (mut e, log) = setup();
        comparison_double(&mut e, &log, TRANSFORM_IS_APPROX_EQUAL, &[false, true]);
        log_double(&mut e, &log, TRANSFORM_SET_MUL, 0);
        let child = object(&mut e, 5);
        let shape = transform_shape(&mut e, KIND_TRANSFORM, child, IDENTITY_COLUMNS);
        let transform = e.mem.alloc(0x40);
        let got = e.call(0x00d7_3590, &args![shape, transform]).u32();
        assert_eq!(got, child);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![child]]);
        let multiplied = calls(&log, TRANSFORM_SET_MUL);
        assert_eq!(multiplied.len(), 1);
        assert_eq!(multiplied[0][1..], [transform, shape + 0x30]);
    }

    #[test]
    fn a_transform_shape_with_another_product_gets_a_new_transform_shape() {
        let (mut e, log) = setup();
        comparison_double(&mut e, &log, TRANSFORM_IS_APPROX_EQUAL, &[false, false]);
        log_double(&mut e, &log, TRANSFORM_SET_MUL, 0);
        constructor_double(&mut e, &log, TRANSFORM_SHAPE_CONSTRUCT);
        let child = object(&mut e, 5);
        let shape = transform_shape(&mut e, KIND_TRANSFORM, child, IDENTITY_COLUMNS);
        let transform = e.mem.alloc(0x40);
        let got = e.call(0x00d7_3590, &args![shape, transform]).u32();
        let product = calls(&log, TRANSFORM_SET_MUL)[0][0];
        let allocations = calls(&log, ALLOCATE);
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0][1], 0x70);
        assert_eq!(
            calls(&log, TRANSFORM_SHAPE_CONSTRUCT),
            vec![vec![got, child, product]]
        );
        assert_eq!(e.mem.u16(got + 4), 0x70);
        assert!(calls(&log, ADD_REFERENCE).is_empty());
    }

    #[test]
    fn a_convex_transform_shape_multiplies_its_own_transform() {
        let (mut e, log) = setup();
        comparison_double(
            &mut e,
            &log,
            TRANSFORM_IS_APPROX_EQUAL,
            &[false, false, false, true],
        );
        log_double(&mut e, &log, TRANSFORM_SET_MUL, 0);
        constructor_double(&mut e, &log, CONVEX_TRANSFORM_SHAPE_CONSTRUCT);
        let child = object(&mut e, 5);
        let shape = transform_shape(&mut e, KIND_CONVEX_TRANSFORM, child, IDENTITY_COLUMNS);
        let transform = e.mem.alloc(0x40);
        // Not the identity: a new convex transform shape (0x60 bytes) around the child.
        let got = e.call(0x00d7_3590, &args![shape, transform]).u32();
        let product = calls(&log, TRANSFORM_SET_MUL)[0][0];
        assert_eq!(
            calls(&log, TRANSFORM_SET_MUL)[0][1..],
            [transform, shape + 0x20]
        );
        assert_eq!(
            calls(&log, CONVEX_TRANSFORM_SHAPE_CONSTRUCT),
            vec![vec![got, child, product, 1]]
        );
        assert_eq!(e.mem.u16(got + 4), 0x60);
        // The product is the identity: the child, one more reference on it.
        let got = e.call(0x00d7_3590, &args![shape, transform]).u32();
        assert_eq!(got, child);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![child]]);
    }

    #[test]
    fn a_convex_translate_shape_with_a_rotated_transform_is_wrapped_in_a_transform_shape() {
        let (mut e, log) = setup();
        comparison_double(&mut e, &log, TRANSFORM_IS_APPROX_EQUAL, &[false]);
        comparison_double(&mut e, &log, MATRIX3_IS_APPROX_EQUAL, &[false]);
        constructor_double(&mut e, &log, TRANSFORM_SHAPE_CONSTRUCT);
        let child = object(&mut e, 5);
        let shape = transform_shape(&mut e, KIND_CONVEX_TRANSLATE, child, IDENTITY_COLUMNS);
        let transform = e.mem.alloc(0x40);
        let got = e.call(0x00d7_3590, &args![shape, transform]).u32();
        assert_eq!(
            calls(&log, TRANSFORM_SHAPE_CONSTRUCT),
            vec![vec![got, shape, transform]]
        );
        assert_eq!(calls(&log, MATRIX3_IS_APPROX_EQUAL)[0][0], transform);
    }

    #[test]
    fn a_convex_translate_shape_multiplies_its_translation_as_a_transform() {
        let (mut e, log) = setup();
        comparison_double(
            &mut e,
            &log,
            TRANSFORM_IS_APPROX_EQUAL,
            &[false, false, false, true],
        );
        comparison_double(&mut e, &log, MATRIX3_IS_APPROX_EQUAL, &[true, true]);
        let temporary = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&temporary);
        logged(&mut e, &log, TRANSFORM_SET_MUL, move |e, a| {
            let columns: Vec<[f32; 4]> = (0..4)
                .map(|index| load_vec(e, a[2] + 0x10 * index))
                .collect();
            sink.borrow_mut().push(columns);
            Ret::default()
        });
        constructor_double(&mut e, &log, CONVEX_TRANSLATE_SHAPE_CONSTRUCT);
        let mut columns = IDENTITY_COLUMNS;
        columns[3] = [1.0, 2.0, 3.0, 7.0];
        let child = object(&mut e, 5);
        e.mem.set_f32(child + 0x10, 0.5);
        let shape = transform_shape(&mut e, KIND_CONVEX_TRANSLATE, child, columns);
        let transform = e.mem.alloc(0x40);
        // The product is not the identity: a new convex translate shape around the child.
        let got = e.call(0x00d7_3590, &args![shape, transform]).u32();
        assert_eq!(
            temporary.borrow()[0],
            vec![
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [1.0, 2.0, 3.0, 7.0]
            ]
        );
        assert_eq!(
            calls(&log, CONVEX_TRANSLATE_SHAPE_CONSTRUCT),
            vec![vec![got, 0xb, 0x3f00_0000, child, 1]]
        );
        assert_eq!(e.mem.u32(got), CONVEX_TRANSLATE_VTABLE);
        assert_eq!(load_vec(&e, got + 0x20), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(e.mem.u32(got + 0x1c), 0);
        assert_eq!(e.mem.u16(got + 4), 0x30);
        // The product is the identity: the child, one more reference on it.
        let got = e.call(0x00d7_3590, &args![shape, transform]).u32();
        assert_eq!(got, child);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![child]]);
    }

    #[test]
    fn every_other_kind_is_wrapped_in_a_transform_shape() {
        let (mut e, log) = setup();
        comparison_double(&mut e, &log, TRANSFORM_IS_APPROX_EQUAL, &[false]);
        constructor_double(&mut e, &log, TRANSFORM_SHAPE_CONSTRUCT);
        let shape = object(&mut e, 7);
        let transform = e.mem.alloc(0x40);
        let got = e.call(0x00d7_3590, &args![shape, transform]).u32();
        assert_eq!(
            calls(&log, TRANSFORM_SHAPE_CONSTRUCT),
            vec![vec![got, shape, transform]]
        );
    }

    #[test]
    fn collapsing_recursively_answers_the_replacement_of_a_shape() {
        let (mut e, log) = setup();
        let shape = object(&mut e, 5);
        let shared = shared_array(&mut e, &[(0x31, 0x32, 1), (shape, 0x99, 1)]);
        let (policy, counters) = (make_policy(&mut e, 0, 0, 0), e.mem.alloc(0x10));
        let got = e
            .call(0x00d7_4480, &args![shape, policy, shared, counters])
            .u32();
        assert_eq!(got, 0x99);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![0x99]]);
    }

    #[test]
    fn collapsing_recursively_keeps_the_kinds_it_does_not_know() {
        let (mut e, log) = setup();
        // An entry without a replacement does not answer; kinds 0xa, 0xd and 0xe are kept.
        let shapes: Vec<u32> = [5, 0xa, 0xd, 0xe, 0x10]
            .into_iter()
            .map(|kind| object(&mut e, kind))
            .collect();
        let shared = shared_array(&mut e, &[(shapes[0], 0, 1)]);
        let (policy, counters) = (make_policy(&mut e, 0, 0, 0), e.mem.alloc(0x10));
        for shape in &shapes {
            let got = e
                .call(0x00d7_4480, &args![*shape, policy, shared, counters])
                .u32();
            assert_eq!(got, *shape);
        }
        let expected: Vec<Vec<u32>> = shapes.iter().map(|shape| vec![*shape]).collect();
        assert_eq!(calls(&log, ADD_REFERENCE), expected);
    }

    #[test]
    fn a_list_whose_children_stay_stays() {
        let (mut e, log) = setup();
        log_double(&mut e, &log, LIST_SHAPE_CONSTRUCT, 0);
        let first = object(&mut e, 5);
        let second = object(&mut e, 5);
        let list = list_shape(&mut e, &log, &[first, second]);
        let shared = shared_array(&mut e, &[]);
        let (policy, counters) = (make_policy(&mut e, 0, 0, 0), e.mem.alloc(0x10));
        let got = e
            .call(0x00d7_4480, &args![list, policy, shared, counters])
            .u32();
        assert_eq!(got, list);
        assert!(calls(&log, LIST_SHAPE_CONSTRUCT).is_empty());
        assert_eq!(
            calls(&log, ADD_REFERENCE),
            vec![vec![first], vec![second], vec![list]]
        );
        assert_eq!(
            calls(&log, REMOVE_REFERENCE),
            vec![vec![first], vec![second]]
        );
        assert_eq!(
            frees(&log).iter().map(|free| free.1).collect::<Vec<_>>(),
            vec![16]
        );
    }

    #[test]
    fn a_list_with_a_changed_child_is_replaced_by_a_new_list() {
        let (mut e, log) = setup();
        constructor_double(&mut e, &log, LIST_SHAPE_CONSTRUCT);
        let first = object(&mut e, 5);
        let second = object(&mut e, 5);
        let list = list_shape(&mut e, &log, &[first, second]);
        let shared = shared_array(&mut e, &[(list, 0, 1), (second, 0x2222, 1)]);
        let (policy, counters) = (make_policy(&mut e, 0, 0, 0), e.mem.alloc(0x10));
        let got = e
            .call(0x00d7_4480, &args![list, policy, shared, counters])
            .u32();
        let built = calls(&log, LIST_SHAPE_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][0], got);
        assert_eq!(built[0][2..], [2, 1]);
        assert_eq!(e.mem.u16(got + 4), 0x70);
        assert_eq!(
            calls(&log, REMOVE_REFERENCE),
            vec![vec![first], vec![0x2222]]
        );
        assert_eq!(
            calls(&log, ADD_REFERENCE),
            vec![vec![first], vec![0x2222], vec![got]]
        );
        assert_eq!(entry(&e, shared, 0), (list, got, 1));
        assert_eq!(
            frees(&log).iter().map(|free| free.1).collect::<Vec<_>>(),
            vec![16]
        );
    }

    #[test]
    fn a_transform_shape_whose_child_stays_collapses_itself() {
        let (mut e, log) = setup();
        log_double(&mut e, &log, COLLAPSE_TRANSFORM_SHAPE, 0x4444);
        let child = object(&mut e, 5);
        let shape = transform_shape(&mut e, KIND_TRANSFORM, child, IDENTITY_COLUMNS);
        let shared = shared_array(&mut e, &[]);
        let (policy, counters) = (make_policy(&mut e, 0, 0, 0), e.mem.alloc(0x10));
        let got = e
            .call(0x00d7_4480, &args![shape, policy, shared, counters])
            .u32();
        assert_eq!(got, 0x4444);
        assert_eq!(
            calls(&log, COLLAPSE_TRANSFORM_SHAPE),
            vec![vec![shape, policy, shared, counters]]
        );
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![child], vec![shape]]);
        assert_eq!(
            calls(&log, REMOVE_REFERENCE),
            vec![vec![child], vec![shape]]
        );
    }

    #[test]
    fn a_transform_shape_with_a_changed_child_is_rebuilt_around_it() {
        let (mut e, log) = setup();
        log_double(&mut e, &log, COLLAPSE_TRANSFORM_SHAPE, 0x4444);
        constructor_double(&mut e, &log, TRANSFORM_SHAPE_CONSTRUCT);
        let child = object(&mut e, 5);
        let shape = transform_shape(&mut e, KIND_TRANSFORM, child, IDENTITY_COLUMNS);
        let shared = shared_array(&mut e, &[(shape, 0, 1), (child, 0x2222, 1)]);
        let (policy, counters) = (make_policy(&mut e, 0, 0, 0), e.mem.alloc(0x10));
        let got = e
            .call(0x00d7_4480, &args![shape, policy, shared, counters])
            .u32();
        assert_eq!(got, 0x4444);
        let built = calls(&log, TRANSFORM_SHAPE_CONSTRUCT);
        assert_eq!(built.len(), 1);
        let block = built[0][0];
        assert_eq!(built[0][1..], [0x2222, shape + 0x30]);
        assert_eq!(e.mem.u16(block + 4), 0x70);
        assert_eq!(entry(&e, shared, 0), (shape, block, 1));
        assert_eq!(
            calls(&log, COLLAPSE_TRANSFORM_SHAPE),
            vec![vec![block, policy, shared, counters]]
        );
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![0x2222], vec![block]]);
        assert_eq!(
            calls(&log, REMOVE_REFERENCE),
            vec![vec![0x2222], vec![block]]
        );
    }

    #[test]
    fn a_convex_transform_shape_with_a_changed_child_is_rebuilt_around_it() {
        let (mut e, log) = setup();
        log_double(&mut e, &log, COLLAPSE_TRANSFORM_SHAPE, 0x4444);
        constructor_double(&mut e, &log, CONVEX_TRANSFORM_SHAPE_CONSTRUCT);
        let child = object(&mut e, 5);
        let shape = transform_shape(&mut e, KIND_CONVEX_TRANSFORM, child, IDENTITY_COLUMNS);
        let shared = shared_array(&mut e, &[(shape, 0, 1), (child, 0x2222, 1)]);
        let (policy, counters) = (make_policy(&mut e, 0, 0, 0), e.mem.alloc(0x10));
        let got = e
            .call(0x00d7_4480, &args![shape, policy, shared, counters])
            .u32();
        assert_eq!(got, 0x4444);
        let built = calls(&log, CONVEX_TRANSFORM_SHAPE_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1..], [0x2222, shape + 0x20, 1]);
        assert_eq!(e.mem.u16(built[0][0] + 4), 0x60);
        assert_eq!(entry(&e, shared, 0), (shape, built[0][0], 1));
    }

    #[test]
    fn a_convex_translate_shape_with_a_changed_child_is_rebuilt_around_it() {
        let (mut e, log) = setup();
        log_double(&mut e, &log, COLLAPSE_TRANSFORM_SHAPE, 0x4444);
        constructor_double(&mut e, &log, CONVEX_TRANSLATE_SHAPE_CONSTRUCT);
        let child = object(&mut e, 5);
        let replacement = object(&mut e, 5);
        e.mem.set_f32(replacement + 0x10, 0.5);
        let shape = transform_shape(
            &mut e,
            KIND_CONVEX_TRANSLATE,
            child,
            [[0.0; 4], [0.0; 4], [0.0; 4], [4.0, 5.0, 6.0, 8.0]],
        );
        let shared = shared_array(&mut e, &[(shape, 0, 1), (child, replacement, 1)]);
        let (policy, counters) = (make_policy(&mut e, 0, 0, 0), e.mem.alloc(0x10));
        let got = e
            .call(0x00d7_4480, &args![shape, policy, shared, counters])
            .u32();
        assert_eq!(got, 0x4444);
        let built = calls(&log, CONVEX_TRANSLATE_SHAPE_CONSTRUCT);
        assert_eq!(built.len(), 1);
        let block = built[0][0];
        assert_eq!(built[0][1..], [0xb, 0x3f00_0000, replacement, 1]);
        assert_eq!(e.mem.u32(block), CONVEX_TRANSLATE_VTABLE);
        assert_eq!(load_vec(&e, block + 0x20), [4.0, 5.0, 6.0, 0.0]);
        assert_eq!(e.mem.u32(block + 0x1c), 0);
        assert_eq!(entry(&e, shared, 0), (shape, block, 1));
    }

    #[test]
    fn collapsing_holders_replaces_the_shape_of_each_holder() {
        let (mut e, log) = setup();
        log_double(&mut e, &log, COUNTERS_RESET, 0);
        log_double(&mut e, &log, SHARED_SHAPES_BUILD, 0);
        log_double(&mut e, &log, SHARED_SHAPES_CLEAR, 0);
        logged(&mut e, &log, COLLAPSE_SHAPES_RECURSIVELY, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        let first = e.mem.alloc(0x20);
        e.mem.set_u32(first + 0x10, 0x1000);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(second + 0x10, 0x2000);
        let holders = word_array(&mut e, &[first, second]);
        let (policy, counters) = (make_policy(&mut e, 0, 0, 0), e.mem.alloc(0x10));
        let got = e.call(0x00d7_4850, &args![holders, policy, counters]).u32();
        assert_eq!(got, 0);
        assert_eq!(calls(&log, COUNTERS_RESET), vec![vec![counters]]);
        let built = calls(&log, SHARED_SHAPES_BUILD);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1], holders);
        let shared = built[0][0];
        assert_eq!(
            calls(&log, COLLAPSE_SHAPES_RECURSIVELY),
            vec![
                vec![0x1000, policy, shared, counters],
                vec![0x2000, policy, shared, counters]
            ]
        );
        assert_eq!(
            calls(&log, REMOVE_REFERENCE),
            vec![vec![0x1000], vec![0x2000]]
        );
        assert_eq!(e.mem.u32(first + 0x10), 0x1001);
        assert_eq!(e.mem.u32(second + 0x10), 0x2001);
        assert_eq!(calls(&log, SHARED_SHAPES_CLEAR), vec![vec![shared]]);
    }

    #[test]
    fn collapsing_one_holder_puts_it_in_an_array_of_one() {
        let (mut e, log) = setup();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        logged(&mut e, &log, COLLAPSE_HOLDERS, move |e, a| {
            sink.borrow_mut().push(read_words(e, a[0]));
            Ret {
                eax: 0x1234,
                ..Ret::default()
            }
        });
        let (policy, counters) = (make_policy(&mut e, 0, 0, 0), e.mem.alloc(0x10));
        let got = e
            .call(0x00d7_48d0, &args![0xabcdu32, policy, counters])
            .u32();
        assert_eq!(got, 0x1234);
        assert_eq!(*seen.borrow(), vec![vec![0xabcd]]);
        let call = &calls(&log, COLLAPSE_HOLDERS)[0];
        assert_eq!(call[1..], [policy, counters]);
        // The array grew to four words and gave them back.
        assert_eq!(
            frees(&log).iter().map(|free| free.1).collect::<Vec<_>>(),
            vec![16]
        );
    }

    #[test]
    fn collapsing_one_shape_builds_shared_data_for_it_alone() {
        let (mut e, log) = setup();
        log_double(&mut e, &log, SHARED_SHAPES_ADD_SHAPE, 0);
        log_double(&mut e, &log, SHARED_SHAPES_FINISH, 0);
        log_double(&mut e, &log, SHARED_SHAPES_CLEAR, 0);
        log_double(&mut e, &log, COLLAPSE_TRANSFORM_SHAPE, 0x9999);
        let (policy, counters) = (make_policy(&mut e, 0, 0, 0), e.mem.alloc(0x10));
        let out = e.mem.alloc(4);
        let got = e
            .call(0x00d7_4420, &args![0x5150u32, policy, counters, out])
            .u32();
        assert_eq!(got, 0);
        assert_eq!(e.mem.u32(out), 0x9999);
        let added = calls(&log, SHARED_SHAPES_ADD_SHAPE);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][1], 0x5150);
        let shared = added[0][0];
        assert_eq!(calls(&log, SHARED_SHAPES_FINISH), vec![vec![shared]]);
        assert_eq!(
            calls(&log, COLLAPSE_TRANSFORM_SHAPE),
            vec![vec![0x5150, policy, shared, counters]]
        );
        assert_eq!(calls(&log, SHARED_SHAPES_CLEAR), vec![vec![shared]]);
    }

    /// The columns the comparison of `collapseTransformShape` saw, and what it was asked.
    fn watch_transform(e: &mut Engine, log: &Log, answers: &[bool]) -> Rc<RefCell<Vec<Columns>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        let queue = Rc::new(RefCell::new(VecDeque::from(answers.to_vec())));
        logged(e, log, TRANSFORM_IS_APPROX_EQUAL, move |e, a| {
            let columns = std::array::from_fn(|index| load_vec(e, a[0] + 0x10 * index as u32));
            sink.borrow_mut().push(columns);
            let answer = queue.borrow_mut().pop_front().unwrap_or(false);
            e.mem.set_u8(a[1], u8::from(answer));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        seen
    }

    fn rotated() -> Columns {
        [
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [10.0, 20.0, 30.0, 0.0],
        ]
    }

    #[test]
    fn collapsing_leaves_a_shape_of_another_kind_alone() {
        let (mut e, log) = setup();
        let shape = object(&mut e, 5);
        let (policy, shared, counters) = (
            make_policy(&mut e, 0, 0, 0),
            shared_array(&mut e, &[]),
            e.mem.alloc(0x10),
        );
        let got = e
            .call(0x00d7_39f0, &args![shape, policy, shared, counters])
            .u32();
        assert_eq!(got, shape);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![shape]]);
    }

    #[test]
    fn an_identity_transform_replaces_the_shape_with_its_child() {
        let (mut e, log) = setup();
        let seen = watch_transform(&mut e, &log, &[true]);
        log_double(&mut e, &log, SHARED_SHAPES_SET_REPLACEMENT, 0);
        let child = object(&mut e, 5);
        let shape = transform_shape(&mut e, KIND_TRANSFORM, child, rotated());
        let (policy, shared, counters) = (
            make_policy(&mut e, 0, 0, 0),
            shared_array(&mut e, &[]),
            e.mem.alloc(0x10),
        );
        let got = e
            .call(0x00d7_39f0, &args![shape, policy, shared, counters])
            .u32();
        assert_eq!(got, child);
        assert_eq!(seen.borrow()[0], rotated());
        assert_eq!(counter_values(&e, counters), vec![0, 1, 0, 0]);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![child]]);
        assert_eq!(
            calls(&log, SHARED_SHAPES_SET_REPLACEMENT),
            vec![vec![shared, shape, child]]
        );
    }

    /// Runs `collapseTransformShape` on `shape` with a fresh shared array and counters; answers
    /// `(answer, shared, counters)`.
    fn collapse(e: &mut Engine, shape: u32, policy: u32) -> (u32, u32, u32) {
        let shared = shared_array(e, &[]);
        let counters = e.mem.alloc(0x10);
        let got = e
            .call(0x00d7_39f0, &args![shape, policy, shared, counters])
            .u32();
        (got, shared, counters)
    }

    #[test]
    fn a_sphere_child_resets_the_rotation_to_the_identity() {
        let (mut e, log) = setup();
        let seen = watch_transform(&mut e, &log, &[true]);
        log_double(&mut e, &log, SHARED_SHAPES_SET_REPLACEMENT, 0);
        let child = object(&mut e, KIND_SPHERE);
        let shape = transform_shape(&mut e, KIND_TRANSFORM, child, rotated());
        let policy = make_policy(&mut e, 0, 0, 0);
        let (got, _, _) = collapse(&mut e, shape, policy);
        assert_eq!(got, child);
        // The rotation of the shape is dropped for the sphere; the translation stays.
        let mut expected = IDENTITY_COLUMNS;
        expected[3] = rotated()[3];
        assert_eq!(seen.borrow()[0], expected);
    }

    /// A transform shape (kind 0xf) with the columns of `rotated()` around a child of `kind`
    /// with radius 0.5, set up so that the transform is not the identity.
    fn transform_over(e: &mut Engine, log: &Log, shape_kind: u32, kind: u32) -> (u32, u32) {
        comparison_double(e, log, TRANSFORM_IS_APPROX_EQUAL, &[false]);
        log_double(e, log, SHARED_SHAPES_SET_REPLACEMENT, 0);
        let child = object(e, kind);
        e.mem.set_f32(child + 0x10, 0.5);
        let shape = transform_shape(e, shape_kind, child, rotated());
        (shape, child)
    }

    #[test]
    fn a_forbidden_child_with_an_identity_rotation_is_wrapped_in_a_convex_translate() {
        let (mut e, log) = setup();
        let (shape, child) = transform_over(&mut e, &log, KIND_TRANSFORM, 5);
        comparison_double(&mut e, &log, MATRIX3_IS_APPROX_EQUAL, &[true]);
        constructor_double(&mut e, &log, CONVEX_TRANSLATE_SHAPE_CONSTRUCT);
        let policy = make_policy(&mut e, 1, 0, 0);
        let (got, shared, counters) = collapse(&mut e, shape, policy);
        assert_eq!(
            calls(&log, CONVEX_TRANSLATE_SHAPE_CONSTRUCT),
            vec![vec![got, 0xb, 0x3f00_0000, child, 1]]
        );
        assert_eq!(e.mem.u32(got), CONVEX_TRANSLATE_VTABLE);
        assert_eq!(load_vec(&e, got + 0x20), [10.0, 20.0, 30.0, 0.0]);
        assert_eq!(e.mem.u32(got + 0x1c), 0);
        assert_eq!(e.mem.u16(got + 4), 0x30);
        assert_eq!(counter_values(&e, counters), vec![0, 0, 1, 0]);
        assert_eq!(
            calls(&log, SHARED_SHAPES_SET_REPLACEMENT),
            vec![vec![shared, shape, got]]
        );
        let compared = calls(&log, MATRIX3_IS_APPROX_EQUAL);
        assert_eq!(compared[0][2..], [IDENTITY_TRANSFORM, 0x3a83_126f]);
    }

    #[test]
    fn a_forbidden_child_with_a_rotation_goes_under_a_convex_transform_shape() {
        let (mut e, log) = setup();
        let (shape, child) = transform_over(&mut e, &log, KIND_TRANSFORM, 5);
        comparison_double(&mut e, &log, MATRIX3_IS_APPROX_EQUAL, &[false]);
        let transform = capturing_constructor(&mut e, &log, CONVEX_TRANSFORM_SHAPE_CONSTRUCT, &[2]);
        let policy = make_policy(&mut e, 1, 0, 0);
        let (got, shared, counters) = collapse(&mut e, shape, policy);
        let built = calls(&log, CONVEX_TRANSFORM_SHAPE_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!([built[0][0], built[0][1], built[0][3]], [got, child, 1]);
        assert_eq!(e.mem.u16(got + 4), 0x60);
        assert_eq!(transform.borrow()[0], rotated()[0]);
        assert_eq!(counter_values(&e, counters), vec![0, 0, 1, 0]);
        assert_eq!(
            calls(&log, SHARED_SHAPES_SET_REPLACEMENT),
            vec![vec![shared, shape, got]]
        );
    }

    #[test]
    fn a_forbidden_child_is_kept_when_no_wrapper_applies() {
        // A convex transform shape with a rotation, a convex translate shape, and a child kind
        // the table does not pick all keep the shape.
        for (kind, child_kind, identity) in [
            (KIND_CONVEX_TRANSFORM, 5, false),
            (KIND_CONVEX_TRANSLATE, 5, true),
            (KIND_TRANSFORM, 3, true),
            (KIND_TRANSFORM, 7, false),
        ] {
            let (mut e, log) = setup();
            comparison_double(&mut e, &log, TRANSFORM_IS_APPROX_EQUAL, &[false]);
            comparison_double(&mut e, &log, MATRIX3_IS_APPROX_EQUAL, &[identity]);
            let child = object(&mut e, child_kind);
            let shape = transform_shape(&mut e, kind, child, rotated());
            let policy = make_policy(&mut e, 1, 0, 0);
            let (got, _, counters) = collapse(&mut e, shape, policy);
            assert_eq!(got, shape, "{kind:#x} over {child_kind}");
            assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![shape]]);
            assert_eq!(counter_values(&e, counters), vec![0; 4]);
        }
    }

    #[test]
    fn mode_two_compares_the_use_count_of_the_child_with_the_limit() {
        for (uses, rebuilt) in [(2u32, true), (3, false), (4, false)] {
            let (mut e, log) = setup();
            let (shape, child) = transform_over(&mut e, &log, KIND_CONVEX_TRANSFORM, 5);
            log_double(&mut e, &log, SHARED_SHAPES_GET_USE_COUNT, uses);
            comparison_double(&mut e, &log, MATRIX3_IS_APPROX_EQUAL, &[false]);
            constructor_double(&mut e, &log, CAPSULE_SHAPE_CONSTRUCT);
            logged(&mut e, &log, SET_TRANSFORMED_POS, |_, _| Ret::default());
            let policy = make_policy(&mut e, 2, 3, 0);
            let (got, shared, counters) = collapse(&mut e, shape, policy);
            assert_eq!(
                calls(&log, SHARED_SHAPES_GET_USE_COUNT),
                vec![vec![shared, child]]
            );
            assert_eq!(
                !calls(&log, CAPSULE_SHAPE_CONSTRUCT).is_empty(),
                rebuilt,
                "{uses}"
            );
            // Forbidden: the capsule stays under the transform and, as the shape is a convex
            // transform shape with a rotation, nothing wraps it: the shape is kept.
            if !rebuilt {
                assert_eq!(got, shape);
                assert_eq!(counter_values(&e, counters), vec![0; 4]);
            }
        }
    }

    #[test]
    fn modes_zero_and_unknown_allow_the_rebuild() {
        for mode in [0, 3, 0x1234] {
            let (mut e, log) = setup();
            let (shape, _child) = transform_over(&mut e, &log, KIND_TRANSFORM, 5);
            constructor_double(&mut e, &log, CAPSULE_SHAPE_CONSTRUCT);
            logged(&mut e, &log, SET_TRANSFORMED_POS, |_, _| Ret::default());
            let policy = make_policy(&mut e, mode, 0, 0);
            collapse(&mut e, shape, policy);
            assert_eq!(calls(&log, CAPSULE_SHAPE_CONSTRUCT).len(), 1, "{mode}");
            assert!(calls(&log, SHARED_SHAPES_GET_USE_COUNT).is_empty());
        }
    }

    #[test]
    fn a_list_child_is_rebuilt_from_transform_shapes_of_its_children() {
        let (mut e, log) = setup();
        comparison_double(&mut e, &log, TRANSFORM_IS_APPROX_EQUAL, &[false]);
        log_double(&mut e, &log, SHARED_SHAPES_SET_REPLACEMENT, 0);
        let wrappers = capturing_constructor(&mut e, &log, TRANSFORM_SHAPE_CONSTRUCT, &[2]);
        constructor_double(&mut e, &log, LIST_SHAPE_CONSTRUCT);
        let first = object(&mut e, 5);
        let second = object(&mut e, 5);
        let list = list_shape(&mut e, &log, &[first, second]);
        let shape = transform_shape(&mut e, KIND_TRANSFORM, list, rotated());
        let policy = make_policy(&mut e, 0, 0, 0);
        let (got, shared, counters) = collapse(&mut e, shape, policy);
        let made = calls(&log, TRANSFORM_SHAPE_CONSTRUCT);
        assert_eq!(made.len(), 2);
        let (first_wrapper, second_wrapper) = (made[0][0], made[1][0]);
        assert_eq!((made[0][1], made[1][1]), (first, second));
        assert_eq!(made[0][2], made[1][2]);
        assert_eq!(wrappers.borrow()[0], rotated()[0]);
        assert_eq!(e.mem.u16(first_wrapper + 4), 0x70);
        // The wrappers are of an unknown kind, so each stays itself (one more reference) and
        // the reference of the loop goes again, then the list takes them over.
        let built = calls(&log, LIST_SHAPE_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][0], got);
        assert_eq!(built[0][2..], [2, 1]);
        assert_eq!(e.mem.u16(got + 4), 0x70);
        assert_eq!(
            calls(&log, REMOVE_REFERENCE),
            vec![
                vec![first_wrapper],
                vec![second_wrapper],
                vec![first_wrapper],
                vec![second_wrapper]
            ]
        );
        assert_eq!(counter_values(&e, counters), vec![0, 0, 0, 1]);
        assert_eq!(
            calls(&log, SHARED_SHAPES_SET_REPLACEMENT),
            vec![vec![shared, shape, got]]
        );
        assert_eq!(
            frees(&log).iter().map(|free| free.1).collect::<Vec<_>>(),
            vec![16]
        );
    }

    #[test]
    fn a_list_child_is_kept_when_the_policy_says_so() {
        for (mode, flag) in [(0, 1u8), (1, 0)] {
            let (mut e, log) = setup();
            comparison_double(&mut e, &log, TRANSFORM_IS_APPROX_EQUAL, &[false]);
            constructor_double(&mut e, &log, LIST_SHAPE_CONSTRUCT);
            let list = list_shape(&mut e, &log, &[]);
            let shape = transform_shape(&mut e, KIND_TRANSFORM, list, rotated());
            let policy = make_policy(&mut e, mode, 0, flag);
            let (got, _, counters) = collapse(&mut e, shape, policy);
            assert_eq!(got, shape);
            assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![shape]]);
            assert!(calls(&log, LIST_SHAPE_CONSTRUCT).is_empty());
            assert_eq!(counter_values(&e, counters), vec![0; 4]);
        }
    }

    /// Doubles for `setTransformedPos` that add 100 to the x of the point they are given and
    /// keep the transform they saw.
    fn transformed_pos_double(e: &mut Engine, log: &Log) -> Rc<RefCell<Vec<Columns>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        logged(e, log, SET_TRANSFORMED_POS, move |e, a| {
            sink.borrow_mut().push(std::array::from_fn(|index| {
                load_vec(e, a[1] + 0x10 * index as u32)
            }));
            let point = load_vec(e, a[2]);
            store_vec(e, a[0], [point[0] + 100.0, point[1], point[2], point[3]]);
            Ret::default()
        });
        seen
    }

    #[test]
    fn a_capsule_child_has_both_end_points_transformed() {
        let (mut e, log) = setup();
        let (shape, child) = transform_over(&mut e, &log, KIND_TRANSFORM, 5);
        e.mem.set_f32(child + 0x10, 0.25);
        store_vec(&mut e, child + 0x20, [1.0, 2.0, 3.0, 1.0]);
        store_vec(&mut e, child + 0x30, [4.0, 5.0, 6.0, 1.0]);
        let seen = transformed_pos_double(&mut e, &log);
        let ends = capturing_constructor(&mut e, &log, CAPSULE_SHAPE_CONSTRUCT, &[1, 2]);
        let policy = make_policy(&mut e, 0, 0, 0);
        let (got, shared, counters) = collapse(&mut e, shape, policy);
        let moved = calls(&log, SET_TRANSFORMED_POS);
        assert_eq!(moved.len(), 2);
        assert_eq!((moved[0][2], moved[1][2]), (child + 0x20, child + 0x30));
        assert_eq!(moved[0][1], moved[1][1]);
        assert_eq!(seen.borrow()[0], rotated());
        assert_eq!(
            *ends.borrow(),
            vec![[101.0, 2.0, 3.0, 1.0], [104.0, 5.0, 6.0, 1.0]]
        );
        let built = calls(&log, CAPSULE_SHAPE_CONSTRUCT);
        assert_eq!(
            built,
            vec![vec![got, moved[0][0], moved[1][0], 0x3e80_0000]]
        );
        assert_eq!(e.mem.u16(got + 4), 0x40);
        assert_eq!(counter_values(&e, counters), vec![1, 0, 0, 0]);
        assert_eq!(
            calls(&log, SHARED_SHAPES_SET_REPLACEMENT),
            vec![vec![shared, shape, got]]
        );
    }

    #[test]
    fn a_cylinder_child_keeps_its_two_radii() {
        let (mut e, log) = setup();
        let (shape, child) = transform_over(&mut e, &log, KIND_TRANSFORM, 2);
        e.mem.set_f32(child + 0x10, 0.25);
        store_vec(&mut e, child + 0x20, [1.0, 2.0, 3.0, 1.0]);
        store_vec(&mut e, child + 0x30, [4.0, 5.0, 6.0, 1.0]);
        logged(&mut e, &log, CYLINDER_RADIUS, |_, _| Ret {
            st0: 0.75,
            ..Ret::default()
        });
        transformed_pos_double(&mut e, &log);
        let ends = capturing_constructor(&mut e, &log, CYLINDER_SHAPE_CONSTRUCT, &[1, 2]);
        let policy = make_policy(&mut e, 0, 0, 0);
        let (got, _, counters) = collapse(&mut e, shape, policy);
        assert_eq!(calls(&log, CYLINDER_RADIUS), vec![vec![child]]);
        let moved = calls(&log, SET_TRANSFORMED_POS);
        assert_eq!((moved[0][2], moved[1][2]), (child + 0x20, child + 0x30));
        assert_eq!(
            *ends.borrow(),
            vec![[101.0, 2.0, 3.0, 1.0], [104.0, 5.0, 6.0, 1.0]]
        );
        assert_eq!(
            calls(&log, CYLINDER_SHAPE_CONSTRUCT),
            vec![vec![
                got,
                moved[0][0],
                moved[1][0],
                0x3f40_0000,
                0x3e80_0000
            ]]
        );
        assert_eq!(e.mem.u16(got + 4), 0x60);
        assert_eq!(counter_values(&e, counters), vec![1, 0, 0, 0]);
    }

    /// A box child with half extents (1, 2, 3) under a transform shape with `columns`; the
    /// comparisons answer `identity` (the transform), then the checks of the box case.
    fn box_scene(
        columns: Columns,
        orthonormal: bool,
        valid: bool,
        rotation_is_identity: &[bool],
    ) -> (Engine, Log, u32, u32) {
        let (mut e, log) = setup();
        comparison_double(&mut e, &log, TRANSFORM_IS_APPROX_EQUAL, &[false]);
        comparison_double(&mut e, &log, MATRIX3_IS_ORTHONORMAL, &[orthonormal]);
        comparison_double(&mut e, &log, MATRIX3_IS_OK, &[valid]);
        comparison_double(&mut e, &log, MATRIX3_IS_APPROX_EQUAL, rotation_is_identity);
        log_double(&mut e, &log, SHARED_SHAPES_SET_REPLACEMENT, 0);
        constructor_double(&mut e, &log, BOX_SHAPE_CONSTRUCT);
        constructor_double(&mut e, &log, CONVEX_TRANSLATE_SHAPE_CONSTRUCT);
        let child = object(&mut e, 4);
        store_vec(&mut e, child + 0x20, [1.0, 2.0, 3.0, 0.0]);
        let shape = transform_shape(&mut e, KIND_TRANSFORM, child, columns);
        (e, log, shape, child)
    }

    fn permutation(translation: [f32; 4]) -> Columns {
        [
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            translation,
        ]
    }

    #[test]
    fn a_box_under_a_permutation_gets_permuted_half_extents() {
        let (mut e, log, shape, _child) = box_scene(permutation([0.0; 4]), true, true, &[false]);
        let extents = capturing_constructor(&mut e, &log, BOX_SHAPE_CONSTRUCT, &[1]);
        let policy = make_policy(&mut e, 0, 0, 0);
        let (got, shared, counters) = collapse(&mut e, shape, policy);
        // Column 0 has its 1 in row 1, column 1 in row 2, column 2 in row 0.
        assert_eq!(*extents.borrow(), vec![[3.0, 1.0, 2.0, 0.0]]);
        let built = calls(&log, BOX_SHAPE_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!([built[0][0], built[0][2]], [got, 0x3d4c_cccd]);
        assert_eq!(e.mem.u16(got + 4), 0x30);
        assert_eq!(counter_values(&e, counters), vec![1, 0, 0, 0]);
        assert_eq!(
            calls(&log, SHARED_SHAPES_SET_REPLACEMENT),
            vec![vec![shared, shape, got]]
        );
        let orthonormal = calls(&log, MATRIX3_IS_ORTHONORMAL);
        assert_eq!(orthonormal[0][2], 0x3a83_126f);
        let identity = calls(&log, MATRIX3_IS_APPROX_EQUAL);
        assert_eq!(identity[0][2..], [IDENTITY_TRANSFORM, 0x3a83_126f]);
    }

    #[test]
    fn a_box_with_a_translation_is_wrapped_in_a_convex_translate() {
        let (mut e, log, shape, _child) =
            box_scene(permutation([0.001, 0.0, 0.0, 5.0]), true, true, &[false]);
        let policy = make_policy(&mut e, 0, 0, 0);
        let (got, shared, counters) = collapse(&mut e, shape, policy);
        let boxes = calls(&log, BOX_SHAPE_CONSTRUCT);
        let block = boxes[0][0];
        assert_eq!(
            calls(&log, CONVEX_TRANSLATE_SHAPE_CONSTRUCT),
            vec![vec![got, 0xb, 0, block, 1]]
        );
        assert_eq!(load_vec(&e, got + 0x20), [0.001, 0.0, 0.0, 0.0]);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![block]]);
        assert_eq!(counter_values(&e, counters), vec![0, 0, 1, 0]);
        assert_eq!(
            calls(&log, SHARED_SHAPES_SET_REPLACEMENT),
            vec![vec![shared, shape, got]]
        );
    }

    #[test]
    fn a_box_under_a_tiny_translation_keeps_the_plain_box() {
        let (mut e, log, shape, _child) =
            box_scene(permutation([0.00005, 0.0, 0.0, 0.0]), true, true, &[false]);
        let policy = make_policy(&mut e, 0, 0, 0);
        let (got, _, counters) = collapse(&mut e, shape, policy);
        assert_eq!(calls(&log, BOX_SHAPE_CONSTRUCT)[0][0], got);
        assert!(calls(&log, CONVEX_TRANSLATE_SHAPE_CONSTRUCT).is_empty());
        assert_eq!(counter_values(&e, counters), vec![1, 0, 0, 0]);
    }

    #[test]
    fn a_box_under_anything_but_a_permutation_falls_back_to_a_convex_translate() {
        let half = [0.5, 0.5, 0.0, 0.0];
        let crowded = [1.0, 1.0, 0.0, 0.0];
        let empty = [0.0; 4];
        let cases: [BoxCase; 7] = [
            // Not orthonormal, not valid, the identity.
            (permutation([1.0; 4]), false, true, vec![true]),
            (permutation([1.0; 4]), true, false, vec![true]),
            (permutation([1.0; 4]), true, true, vec![true, true]),
            // An entry that is neither 0 nor 1, a column of 2, a column without a 1.
            (
                [half, [0.0, 0.0, 1.0, 0.0], [1.0, 0.0, 0.0, 0.0], [1.0; 4]],
                true,
                true,
                vec![false, true],
            ),
            (
                [
                    crowded,
                    [0.0, 0.0, 1.0, 0.0],
                    [1.0, 0.0, 0.0, 0.0],
                    [1.0; 4],
                ],
                true,
                true,
                vec![false, true],
            ),
            (
                [empty, [0.0, 0.0, 1.0, 0.0], [1.0, 0.0, 0.0, 0.0], [1.0; 4]],
                true,
                true,
                vec![false, true],
            ),
            // NaN is neither 0 nor 1.
            (
                [
                    [f32::NAN, 0.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [1.0, 0.0, 0.0, 0.0],
                    [1.0; 4],
                ],
                true,
                true,
                vec![false, true],
            ),
        ];
        for (index, (columns, orthonormal, valid, identity)) in cases.into_iter().enumerate() {
            let (mut e, log, shape, child) = box_scene(columns, orthonormal, valid, &identity);
            let policy = make_policy(&mut e, 0, 0, 0);
            let (got, _, counters) = collapse(&mut e, shape, policy);
            assert!(calls(&log, BOX_SHAPE_CONSTRUCT).is_empty(), "case {index}");
            assert_eq!(
                calls(&log, CONVEX_TRANSLATE_SHAPE_CONSTRUCT),
                vec![vec![got, 0xb, 0, child, 1]],
                "case {index}"
            );
            assert_eq!(
                counter_values(&e, counters),
                vec![0, 0, 1, 0],
                "case {index}"
            );
        }
    }

    #[test]
    fn a_convex_vertices_child_gets_transformed_vertices_and_planes() {
        let (mut e, log) = setup();
        let columns = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [10.0, 20.0, 30.0, 0.0],
        ];
        comparison_double(&mut e, &log, TRANSFORM_IS_APPROX_EQUAL, &[false]);
        log_double(&mut e, &log, SHARED_SHAPES_SET_REPLACEMENT, 0);
        let child = object(&mut e, 6);
        e.mem.set_f32(child + 0x10, 0.125);
        let shape = transform_shape(&mut e, KIND_TRANSFORM, child, columns);
        // The vertices the shape hands out, and its array of plane equations at +0x54.
        let vertices = e.mem.alloc(0x20);
        store_vec(&mut e, vertices, [1.0, 2.0, 3.0, 1.0]);
        store_vec(&mut e, vertices + 0x10, [4.0, 5.0, 6.0, 1.0]);
        logged(&mut e, &log, GET_ORIGINAL_VERTICES, move |e, a| {
            e.mem.set_u32(a[1], vertices);
            e.mem.set_u32(a[1] + 4, 2);
            e.mem.set_u32(a[1] + 8, 2);
            Ret::default()
        });
        let planes = e.mem.alloc(0x10);
        store_vec(&mut e, planes, [0.0, 0.0, 1.0, 5.0]);
        e.mem.set_u32(child + 0x54, planes);
        e.mem.set_u32(child + 0x58, 1);
        e.mem.set_u32(child + 0x5c, 1);
        logged(&mut e, &log, GET_PLANE_EQUATIONS, |_, a| Ret {
            eax: a[0] + 0x54,
            ..Ret::default()
        });
        logged(&mut e, &log, ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(a[3] * a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        log_double(&mut e, &log, SET_ROTATED_DIR, 0);
        // The constructor keeps the strided vertices and the planes it is given.
        let seen: Rc<RefCell<Vec<ConstructorSeen>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        logged(
            &mut e,
            &log,
            CONVEX_VERTICES_SHAPE_CONSTRUCT,
            move |e, a| {
                let strided = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4), e.mem.u32(a[1] + 8));
                let moved = (0..strided.1)
                    .map(|index| load_vec(e, strided.0 + 0x10 * index))
                    .collect();
                let (data, size) = (e.mem.u32(a[2]), e.mem.u32(a[2] + 4));
                let normals = (0..size)
                    .map(|index| load_vec(e, data + 0x10 * index))
                    .collect();
                sink.borrow_mut().push((moved, normals, strided.2, data));
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            },
        );
        let policy = make_policy(&mut e, 0, 0, 0);
        let (got, shared, counters) = collapse(&mut e, shape, policy);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(
            seen[0].0,
            vec![[11.0, 22.0, 33.0, 0.0], [14.0, 25.0, 36.0, 0.0]]
        );
        // The normal stays (the rotation double does nothing); the offset 5 loses the dot
        // product 30 of the normal with the translation.
        assert_eq!(seen[0].1, vec![[0.0, 0.0, 1.0, -25.0]]);
        assert_eq!(seen[0].2, 0x10);
        let reserved = calls(&log, ARRAY_RESERVE);
        assert_eq!(reserved.len(), 1);
        assert_eq!(reserved[0][2..], [1, 0x10]);
        let built = calls(&log, CONVEX_VERTICES_SHAPE_CONSTRUCT);
        assert_eq!(built[0][0], got);
        assert_eq!(built[0][3], 0x3e00_0000);
        assert_eq!(e.mem.u16(got + 4), 0x70);
        // The planes it built and the vertices the shape gave out are freed, in that order.
        assert_eq!(frees(&log), vec![(seen[0].3, 16), (vertices, 32)]);
        assert_eq!(counter_values(&e, counters), vec![1, 0, 0, 0]);
        assert_eq!(
            calls(&log, SHARED_SHAPES_SET_REPLACEMENT),
            vec![vec![shared, shape, got]]
        );
        // The rotation of the plane normal ran in place on the plane copy.
        let rotations = calls(&log, SET_ROTATED_DIR);
        assert_eq!(rotations.len(), 1);
        assert_eq!(rotations[0][0], rotations[0][2]);
    }

    #[test]
    fn a_sphere_or_other_child_under_the_identity_rotation_is_wrapped() {
        let (mut e, log) = setup();
        let seen = watch_transform(&mut e, &log, &[false]);
        comparison_double(&mut e, &log, MATRIX3_IS_APPROX_EQUAL, &[true]);
        log_double(&mut e, &log, SHARED_SHAPES_SET_REPLACEMENT, 0);
        constructor_double(&mut e, &log, CONVEX_TRANSLATE_SHAPE_CONSTRUCT);
        let child = object(&mut e, KIND_SPHERE);
        let shape = transform_shape(&mut e, KIND_TRANSFORM, child, rotated());
        let policy = make_policy(&mut e, 0, 0, 0);
        let (got, _, counters) = collapse(&mut e, shape, policy);
        // The sphere is round: its rotation was replaced by the identity before the comparison.
        let mut expected = IDENTITY_COLUMNS;
        expected[3] = rotated()[3];
        assert_eq!(seen.borrow()[0], expected);
        assert_eq!(
            calls(&log, CONVEX_TRANSLATE_SHAPE_CONSTRUCT),
            vec![vec![got, 0xb, 0, child, 1]]
        );
        assert_eq!(load_vec(&e, got + 0x20), [10.0, 20.0, 30.0, 0.0]);
        assert_eq!(counter_values(&e, counters), vec![0, 0, 1, 0]);
    }
}
