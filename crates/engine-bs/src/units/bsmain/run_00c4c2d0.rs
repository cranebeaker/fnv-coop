//! `BSMain/run_00c4c2d0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::{BSSimpleArray, NiTArray};

/// `NiTransform::InvertNonUniform` (`004b4880`, `thiscall`, the result's
/// address): the inverse transform (rotation 3x3, translation, scale; 0x34
/// bytes).
const TRANSFORM_INVERT_NON_UNIFORM: u32 = 0x004b_4880;
/// `D3DXMatrixInverse` (`00ee6dda`, `stdcall`, `(out, determinant, in)`).
const D3DX_MATRIX_INVERSE: u32 = 0x00ee_6dda;
/// The translation `00c4c2d0` puts in the matrix it inverts (three floats).
const FIXED_TRANSLATION: u32 = 0x011f_474c;
/// `00aa1070` (`cdecl`, one word): the block allocation; `00aa10f0`
/// (`cdecl`, one word): its free.
const NI_ALLOC: u32 = 0x00aa_1070;
const NI_FREE: u32 = 0x00aa_10f0;
/// The renderer singleton pointer, its Direct3D device (+0x288) and the
/// device methods used here by vtable offset (COM, `this` first): +0x1d8
/// with `(9, &value)`; on the object `00c4ed40` holds, +0x08 (no
/// arguments); on the one `00c4ed70` uses, +0x18 with `(2)`.
const RENDERER: u32 = 0x011f_4748;
const RENDERER_DEVICE: u32 = 0x288;
const SLOT_DEVICE_QUERY: u32 = 0x1d8;
const SLOT_RELEASE: u32 = 0x08;
const SLOT_ISSUE: u32 = 0x18;
/// The byte `00c4ed70` sets the first time.
const ISSUED_ONCE: u32 = 0x0120_30a4;

/// A float copied through the x87 stack and multiplied by `scale` (a float
/// memory operand), stored as a float.
fn scaled(value: u32, scale: u32) -> u32 {
    x87_store32(x87_mul(x87_load32(value), x87_m32(scale)))
}

// Translated from 00c4c2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c4c2d0`; `cdecl`): the inverse of
/// a transform as a 4x4 row-major matrix at `out`. With `fixed` zero:
/// `NiTransform::InvertNonUniform` of `transform`, then the inverse
/// rotation's columns times its scale as the first three rows (each
/// element copied through a float temporary, then multiplied), 0.0 in the
/// fourth column, its translation as the last row and 1.0. Otherwise
/// `D3DXMatrixInverse` of the identity with the translation at `011f474c`
/// (the transform is not read).
pub fn fn_00c4c2d0(e: &mut Engine, transform: Ptr, fixed: u32, out: Ptr) {
    let out = out.addr();
    if fixed == 0 {
        e.with_stack(0x34, |e, inverse| {
            let inverse = inverse.addr();
            e.call(TRANSFORM_INVERT_NON_UNIFORM, &args![transform, inverse]);
            let scale = e.mem.u32(inverse + 0x30);
            for column in 0..3u32 {
                for row in 0..3u32 {
                    let element = e.mem.u32(inverse + row * 12 + column * 4);
                    let copied = x87_quiet_bits(element);
                    let value = scaled(copied, scale);
                    e.mem
                        .set_u32(out.wrapping_add(column * 16 + row * 4), value);
                }
                e.mem.set_f32(out.wrapping_add(column * 16 + 12), 0.0);
            }
            for k in 0..3u32 {
                let value = x87_quiet_bits(e.mem.u32(inverse + 0x24 + 4 * k));
                e.mem.set_u32(out.wrapping_add(0x30 + 4 * k), value);
            }
            e.mem.set_f32(out.wrapping_add(0x3c), 1.0);
        });
        return;
    }
    e.with_stack(0x40, |e, matrix| {
        let matrix = matrix.addr();
        for k in 0..16u32 {
            let value = if k % 5 == 0 { 1.0 } else { 0.0 };
            e.mem.set_f32(matrix + 4 * k, value);
        }
        for k in 0..3u32 {
            let value = x87_quiet_bits(e.mem.u32(FIXED_TRANSLATION + 4 * k));
            e.mem.set_u32(matrix + 0x30 + 4 * k, value);
        }
        e.call(D3DX_MATRIX_INVERSE, &args![out, 0u32, matrix]);
    });
}

// Translated from 00c4c920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c4c2d0`): an `NiTArray`'s
/// `SetSize` (the template instance in this unit). Nothing when `size` is
/// the capacity. A size below the used size clears the slots from `size`
/// on (counting each non-null one off) and becomes the used size. Then the
/// capacity becomes `size`: a new block of that many words (`00aa1070`)
/// takes the used slots and zeros for the rest, or no block for 0; the old
/// block is freed (`00aa10f0`).
pub fn fn_00c4c920(e: &mut Engine, this: Ptr<NiTArray>, size: u32) {
    let capacity = u32::from(e.get(this, NiTArray::m_usMaxSize));
    if size == capacity {
        return;
    }
    if size < u32::from(e.get(this, NiTArray::m_usSize)) {
        let mut i = size as u16;
        while i < e.get(this, NiTArray::m_usSize) {
            let base = e.get(this, NiTArray::m_pBase);
            let slot = base.wrapping_add(u32::from(i) * 4);
            if e.mem.u32(slot) != 0 {
                e.mem.set_u32(slot, 0);
                let count = e.get(this, NiTArray::m_usESize).wrapping_sub(1);
                e.set(this, NiTArray::m_usESize, count);
            }
            i = i.wrapping_add(1);
        }
        e.set(this, NiTArray::m_usSize, size as u16);
    }
    let old = e.get(this, NiTArray::m_pBase);
    e.set(this, NiTArray::m_usMaxSize, size as u16);
    if size == 0 {
        e.set(this, NiTArray::m_pBase, 0);
    } else {
        let capacity = u32::from(e.get(this, NiTArray::m_usMaxSize));
        let block = e.call(NI_ALLOC, &args![capacity << 2]).u32();
        e.set(this, NiTArray::m_pBase, block);
        let mut i = 0u16;
        while i < e.get(this, NiTArray::m_usSize) {
            let word = e.mem.u32(old.wrapping_add(u32::from(i) * 4));
            let base = e.get(this, NiTArray::m_pBase);
            e.mem.set_u32(base.wrapping_add(u32::from(i) * 4), word);
            i = i.wrapping_add(1);
        }
        let mut i = e.get(this, NiTArray::m_usSize);
        while i < e.get(this, NiTArray::m_usMaxSize) {
            let base = e.get(this, NiTArray::m_pBase);
            e.mem.set_u32(base.wrapping_add(u32::from(i) * 4), 0);
            i = i.wrapping_add(1);
        }
    }
    e.call(NI_FREE, &args![old]);
}

// Translated from 00c4ecf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c4c2d0`): clears the word at
/// `this`, then has the renderer's Direct3D device (the pointer at
/// `011f4748`, +0x288) fill it through its method at vtable +0x1d8 with 9.
/// Returns `this`.
pub fn fn_00c4ecf0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    let renderer = e.mem.u32(RENDERER);
    let device = e.mem.u32(renderer.wrapping_add(RENDERER_DEVICE));
    e.vcall(device, SLOT_DEVICE_QUERY, &args![9u32, this]);
    this
}

// Translated from 00c4ed40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c4c2d0`): releases the COM object
/// the word at `this` holds (its method at vtable +0x08), when there is
/// one.
pub fn fn_00c4ed40(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(this.addr());
    if object != 0 {
        e.vcall(object, SLOT_RELEASE, &args![]);
    }
}

// Translated from 00c4ed70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c4c2d0`): only the first time (the
/// byte at `012030a4`), calls the method at vtable +0x18 of the COM object
/// the word at `this` holds with 2, and returns true; false afterwards.
pub fn fn_00c4ed70(e: &mut Engine, this: Ptr) -> bool {
    if e.mem.u8(ISSUED_ONCE) != 0 {
        return false;
    }
    e.mem.set_u8(ISSUED_ONCE, 1);
    let object = e.mem.u32(this.addr());
    e.vcall(object, SLOT_ISSUE, &args![2u32]);
    true
}

/// The polygon these functions work on (its class is unconfirmed; the engine map names the
/// deleting destructor that runs [`fn_00c4ccd0`], `0049e3e0`, `BSPolygon2D::~BSPolygon2D`):
/// the array of its points (a `BSSimpleArray` of `NiPoint3`, by pointer, +0), the outline
/// index array (+4) and the triangle index array (+8) (16-bit indices: buffer at +4, count at
/// +8), the triangle data's vertices (+0xC), indices (+0x10), normals (+0x14), texture
/// coordinates (+0x18) and colours (+0x1C), the point count (+0x24), the triangle count
/// (+0x28), the node (+0x2C) and the shape (+0x30) as counted pointers, and the centre
/// (+0x34).
const POINTS: u32 = 0x00;
const OUTLINE: u32 = 0x04;
const TRIANGLES: u32 = 0x08;
const VERTICES: u32 = 0x0c;
const INDICES: u32 = 0x10;
const NORMALS: u32 = 0x14;
const TEXTURE_COORDINATES: u32 = 0x18;
const COLORS: u32 = 0x1c;
const POINT_COUNT: u32 = 0x24;
const TRIANGLE_COUNT: u32 = 0x28;
const NODE: u32 = 0x2c;
const SHAPE: u32 = 0x30;
const CENTER: u32 = 0x34;
/// The global `NiPoint3` the code copies as the zero point (`011f426c`).
const ZERO_POINT: u32 = 0x011f_426c;
/// `double` constants: 0.5 (`01011588`), 2.0 (`01011590`) and 0.0 (`01012060`); the `float`
/// the texture coordinates divide by (`01016410`).
const HALF: u32 = 0x0101_1588;
const TWO: u32 = 0x0101_1590;
const ZERO: u32 = 0x0101_2060;
const TEXTURE_SCALE: u32 = 0x0101_6410;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The memory manager (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, `RET 4`);
/// `NiNew` (`00aa13e0`, `cdecl`: size); `memcpy` (`00ec44d0`, `cdecl`); `fabs` (`00ec6cde`,
/// `cdecl`: a `double`, answered in ST0).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const NI_NEW: u32 = 0x00aa_13e0;
const MEMCPY: u32 = 0x00ec_44d0;
const FABS: u32 = 0x00ec_6cde;
/// `0066b0d0` (`thiscall` on the counted pointer, `RET 4`): assigns it the object given.
const ASSIGN_POINTER: u32 = 0x0066_b0d0;
/// `NiTArray::SetAt` of this unit's array (`0096ae90`, `thiscall`, `RET 8`: index, element).
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `NiPoint3` helpers (`thiscall`): `==` (`004390c0`, `RET 4`, a byte), `-` (`00439ef0`,
/// `RET 8`: result address, other point; answers the result), `+=` (`0063c8a0`, `RET 4`) and
/// division by a `float` (`0053d280`, `RET 8`: result address, divisor).
const POINT_EQUALS: u32 = 0x0043_90c0;
const POINT_MINUS: u32 = 0x0043_9ef0;
const POINT_ADD_TO: u32 = 0x0063_c8a0;
const POINT_DIVIDE: u32 = 0x0053_d280;
/// The 16-bit index arrays: the constructor of a 0x10-byte one (`006917d0`, `thiscall`), its
/// set-up (`00692290`, `thiscall`, `RET 8`) after the virtual table `0106abe4` is stored, the
/// add of the index at an address (`00691820`, `thiscall`, `RET 4`) and the test of one
/// (`0069dee0`, `thiscall`, `RET 4`, a byte).
const INDEX_ARRAY_CONSTRUCTOR: u32 = 0x0069_17d0;
const INDEX_ARRAY_SET_UP: u32 = 0x0069_2290;
const INDEX_ARRAY_VTABLE: u32 = 0x0106_abe4;
const INDEX_ARRAY_ADD: u32 = 0x0069_1820;
const INDEX_ARRAY_TEST: u32 = 0x0069_dee0;
/// The stack array of point numbers `00c4cf80` builds: its virtual table (`01016024`), set-up
/// (`006b3eb0`, `thiscall`, `RET 8`), the append of the word at an address (`007cb2e0`,
/// `BSSimpleArray<BGSBodyPart*,1024>::AddUninitialized` in the engine map; `thiscall`,
/// `RET 4`) and destructor (`00438690`, `thiscall`).
const ORDER_ARRAY_VTABLE: u32 = 0x0101_6024;
const ORDER_ARRAY_SET_UP: u32 = 0x006b_3eb0;
const ORDER_ARRAY_APPEND: u32 = 0x007c_b2e0;
const ORDER_ARRAY_DESTRUCTOR: u32 = 0x0043_8690;
/// The element moves of `BSSimpleArray` removals (`thiscall`, `RET 0xC`: destination,
/// source, count) for 12-byte (`006dc6e0`), 4-byte (`0042fb60`) and 2-byte (`005efdd0`)
/// elements.
const MOVE_12: u32 = 0x006d_c6e0;
const MOVE_4: u32 = 0x0042_fb60;
const MOVE_2: u32 = 0x005e_fdd0;
/// `NiTriShape::NiTriShape_ov2` (`00a74480`, engine map; `thiscall`, `RET 4`: the data),
/// `NiTriShapeData::NiTriShapeData` (`00a7b630`, engine map; `thiscall`, `RET 0x24`) and the
/// `NiNode` constructor (`00a5ecb0`, `thiscall`, `RET 4`).
const TRI_SHAPE_CONSTRUCTOR: u32 = 0x00a7_4480;
const TRI_SHAPE_DATA_CONSTRUCTOR: u32 = 0x00a7_b630;
const NODE_CONSTRUCTOR: u32 = 0x00a5_ecb0;
/// The node's virtual slot `+0xDC` (two words: the child, 1).
const SLOT_ATTACH: u32 = 0xdc;

/// The address of point `index` of the polygon (the points array's buffer is read each
/// time, as the exe does).
fn point_at(e: &Engine, this: u32, index: u32) -> u32 {
    let points = e.mem.u32(this.wrapping_add(POINTS));
    let buffer = e.get(Ptr::<BSSimpleArray>::new(points), BSSimpleArray::pBuffer);
    buffer.wrapping_add(index.wrapping_mul(0xc))
}

/// The three words of the point at `address`.
fn point_words(e: &Engine, address: u32) -> [u32; 3] {
    [
        e.mem.u32(address),
        e.mem.u32(address.wrapping_add(4)),
        e.mem.u32(address.wrapping_add(8)),
    ]
}

/// The inlined construction of a null counted pointer at `slot` (the dead increment of the
/// just-stored null included).
fn construct_null_pointer(e: &mut Engine, slot: u32) {
    e.mem.set_u32(slot, 0);
    let object = e.mem.u32(slot);
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

/// The inlined release of the counted pointer at `slot`: its count (+4) goes down and at zero
/// its virtual `+4` runs.
fn release_pointer(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object != 0
        && e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// Adds the low 16 bits of `index` to the index array `array` (by the address of a local).
fn add_index(e: &mut Engine, array: u32, index: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u16(slot.addr(), index as u16);
        e.call(INDEX_ARRAY_ADD, &args![array, slot]);
    });
}

/// One row of a transform applied to the point at `point` (x87, in the exe's order): the
/// row's three elements copied through `float` locals, multiplied by the coordinates and
/// summed.
fn row_dot(e: &Engine, transform: u32, row: u32, point: u32) -> f64 {
    let element = |column: u32| x87_quiet_bits(e.mem.u32(transform + row * 0xc + column * 4));
    let (a, b, c) = (element(0), element(1), element(2));
    let first = x87_mul(x87_load32(a), x87_m32(e.mem.u32(point)));
    let second = x87_mul(x87_load32(b), x87_m32(e.mem.u32(point.wrapping_add(4))));
    let sum = x87_add(first, second);
    let third = x87_mul(x87_load32(c), x87_m32(e.mem.u32(point.wrapping_add(8))));
    x87_add(sum, third)
}

// Translated from 00c4c530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Transforms `count` points and normals (unnamed in the engine map; `cdecl`: transform,
/// count, points, transformed points, normals, transformed normals). With an output for the
/// points: each coordinate is the rotation row times the point, times the scale (+0x30), plus
/// the translation (+0x24), stored through a `float` local. With an output for the normals:
/// the rotation row times the normal. Each output coordinate is stored before the next one
/// reads the input (x87 arithmetic in the exe's order).
pub fn fn_00c4c530(
    e: &mut Engine,
    transform: Ptr,
    count: u32,
    points: Ptr,
    transformed_points: Ptr,
    normals: Ptr,
    transformed_normals: Ptr,
) {
    let transform = transform.addr();
    let mut index = 0u32;
    while index < count {
        if !transformed_points.is_null() {
            for row in 0..3u32 {
                let point = points.addr().wrapping_add(index.wrapping_mul(0xc));
                let dot = row_dot(e, transform, row, point);
                let scaled = x87_mul(dot, x87_m32(e.mem.u32(transform + 0x30)));
                let moved = x87_add(scaled, x87_m32(e.mem.u32(transform + 0x24 + row * 4)));
                let value = x87_quiet_bits(x87_store32(moved));
                let out = transformed_points
                    .addr()
                    .wrapping_add(index.wrapping_mul(0xc) + row * 4);
                e.mem.set_u32(out, value);
            }
        }
        if !transformed_normals.is_null() {
            for row in 0..3u32 {
                let normal = normals.addr().wrapping_add(index.wrapping_mul(0xc));
                let value = x87_store32(row_dot(e, transform, row, normal));
                let out = transformed_normals
                    .addr()
                    .wrapping_add(index.wrapping_mul(0xc) + row * 4);
                e.mem.set_u32(out, value);
            }
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00c4c8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `NiTArray`'s `Add` (the template instance in this unit; `thiscall`, `RET 4`): the
/// element goes to the index of the used size (`m_usSize`), the array growing first
/// ([`fn_00c4c920`] to the size plus `m_usGrowBy`) when that index is not below the capacity;
/// stored by `SetAt` (`0096ae90`). Answers the index.
pub fn fn_00c4c8d0(e: &mut Engine, this: Ptr<NiTArray>, element: u32) -> u32 {
    let index = u32::from(e.get(this, NiTArray::m_usSize));
    if index >= u32::from(e.get(this, NiTArray::m_usMaxSize)) {
        let size = u32::from(e.get(this, NiTArray::m_usGrowBy)).wrapping_add(index);
        fn_00c4c920(e, this, size);
    }
    e.call(ARRAY_SET_AT, &args![this, index, element]);
    index
}

/// The constructors' common body after the points: the index arrays, counts and the
/// pointers cleared (`0066b0d0`) and the centre set to the zero point.
fn finish_construction(e: &mut Engine, this: u32) {
    e.mem.set_u32(this + 0x8, 0);
    e.mem.set_u32(this + OUTLINE, 0);
    e.mem.set_u32(this + TRIANGLE_COUNT, 0);
    e.call(ASSIGN_POINTER, &args![this + SHAPE, 0u32]);
    e.call(ASSIGN_POINTER, &args![this + NODE, 0u32]);
    for k in 0..3u32 {
        let word = e.mem.u32(ZERO_POINT + 4 * k);
        e.mem.set_u32(this + CENTER + 4 * k, word);
    }
}

// Translated from 00c4ca80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c4ca80 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2e9d6, state variable) is not translated
/// The polygon's default constructor (unnamed in the engine map; `thiscall`, plain `RET`):
/// null node and shape pointers, no points (count 0, null array), no index arrays, no
/// triangles, the pointers assigned null again (`0066b0d0`) and the centre at the zero point
/// (`011f426c`). Answers `this`.
pub fn fn_00c4ca80(e: &mut Engine, this: Ptr) -> Ptr {
    let address = this.addr();
    construct_null_pointer(e, address + NODE);
    construct_null_pointer(e, address + SHAPE);
    e.mem.set_u32(address + POINT_COUNT, 0);
    e.mem.set_u32(address + POINTS, 0);
    finish_construction(e, address);
    this
}

// Translated from 00c4cba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c4cba0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2ea07, state variable) is not translated
/// The polygon's constructor from a points array (unnamed in the engine map; `thiscall`,
/// `RET 4`): as [`fn_00c4ca80`] with the array and its size (+8) as the points and their
/// count, then the outline is built ([`fn_00c4cdc0`]). Answers `this`.
pub fn fn_00c4cba0(e: &mut Engine, this: Ptr, points: Ptr) -> Ptr {
    let address = this.addr();
    construct_null_pointer(e, address + NODE);
    construct_null_pointer(e, address + SHAPE);
    let count = e.get(points.cast::<BSSimpleArray>(), BSSimpleArray::iSize);
    e.mem.set_u32(address + POINT_COUNT, count);
    e.mem.set_u32(address + POINTS, points.addr());
    finish_construction(e, address);
    fn_00c4cdc0(e, this);
    this
}

// Translated from 00c4ccd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c4ccd0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2ea38, state variable) is not translated
/// The polygon's destructor (unnamed in the engine map; `thiscall`): without triangles
/// (+0x28 zero) the points array is deleted (its virtual `+0` with 1) when there is one;
/// then the shape and the node drop their references (count at +4; at zero their virtual
/// `+4`).
pub fn fn_00c4ccd0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    if e.mem.u32(this + TRIANGLE_COUNT) == 0 {
        let points = e.mem.u32(this + POINTS);
        if points != 0 {
            e.vcall(points, 0, &args![1u32]);
        }
    }
    release_pointer(e, this + SHAPE);
    release_pointer(e, this + NODE);
}

// Translated from 00c4cdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the polygon's outline (unnamed in the engine map; `thiscall`): when the signed area
/// ([`fn_00c4d9d0`]) is not below 0.0 (`FCOMP` with the `double` at `01012060`; a NaN area
/// skips it), the points are reversed ([`fn_00c4e900`]); then [`fn_00c4db20`].
pub fn fn_00c4cdc0(e: &mut Engine, this: Ptr) {
    let area = f64::from(fn_00c4d9d0(e, this));
    let zero = f64::from_bits(e.mem.u64(ZERO));
    if area >= zero {
        let points = e.mem.u32(this.addr() + POINTS);
        fn_00c4e900(e, Ptr::new(points));
    }
    fn_00c4db20(e, this);
}

// Translated from 00c4ce00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Centres the polygon (unnamed in the engine map; `thiscall`): the points are summed
/// (`0063c8a0`, from the zero point), the sum divided by the point count (`0053d280`, the
/// count converted by `FILD` and stored as a `float`), each point replaced by itself minus
/// that average (`00439ef0`), and the average stored as the centre (+0x34).
pub fn fn_00c4ce00(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.with_stack(0x24, |e, locals| {
        let sum = locals.addr();
        let average = sum + 0xc;
        let difference = sum + 0x18;
        for k in 0..3u32 {
            let word = e.mem.u32(ZERO_POINT + 4 * k);
            e.mem.set_u32(sum + 4 * k, word);
        }
        let mut index = 0i32;
        while index < e.mem.i32(this + POINT_COUNT) {
            let point = point_at(e, this, index as u32);
            e.call(POINT_ADD_TO, &args![sum, point]);
            index = index.wrapping_add(1);
        }
        let divisor = x87_store32(f64::from(e.mem.i32(this + POINT_COUNT)));
        e.call(POINT_DIVIDE, &args![sum, average, divisor]);
        let mut index = 0i32;
        while index < e.mem.i32(this + POINT_COUNT) {
            let point = point_at(e, this, index as u32);
            let result = e
                .call(POINT_MINUS, &args![point, difference, average])
                .u32();
            let words = point_words(e, result);
            let point = point_at(e, this, index as u32);
            for (k, word) in words.iter().enumerate() {
                e.mem.set_u32(point + 4 * k as u32, *word);
            }
            index = index.wrapping_add(1);
        }
        let words = point_words(e, average);
        for (k, word) in words.iter().enumerate() {
            e.mem.set_u32(this + CENTER + 4 * k as u32, *word);
        }
    });
}

// Translated from 00c4cf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flattens the polygon (unnamed in the engine map; `thiscall`): every point's z becomes 0.0
/// (each point copied out, its z set, and written back).
pub fn fn_00c4cf00(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let mut index = 0i32;
    while index < e.mem.i32(this + POINT_COUNT) {
        let point = point_at(e, this, index as u32);
        let [x, y, _] = point_words(e, point);
        let point = point_at(e, this, index as u32);
        e.mem.set_u32(point, x);
        e.mem.set_u32(point + 4, y);
        e.mem.set_f32(point + 8, 0.0);
        index = index.wrapping_add(1);
    }
}

/// `v` brought into `0..size` the way the triangulation does (signed `IDIV`: a negative `v`
/// gets `v % size + size`); left as it is when `size` is negative as a signed word.
fn wrap_index(v: i32, size: u32) -> i32 {
    let size = size as i32;
    if size < 0 {
        return v;
    }
    if v < 0 {
        v.wrapping_rem(size).wrapping_add(size)
    } else {
        v.wrapping_rem(size)
    }
}

/// The position of the 16-bit index `value` in the outline array `outline` (skipping the
/// search while its buffer is null), or -1.
fn outline_position(e: &Engine, outline: u32, value: u32) -> u32 {
    let mut position = 0u32;
    while position < e.mem.u32(outline + 8) {
        let buffer = e.mem.u32(outline + 4);
        if buffer != 0 && u32::from(e.mem.u16(buffer.wrapping_add(position * 2))) == value {
            return position;
        }
        position = position.wrapping_add(1);
    }
    0xffff_ffff
}

/// [`fn_00c4dcf0`] of the points with the numbers `first`, `second` and `third`.
fn orientation_of(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) -> i32 {
    let [a0, a1, a2] = point_words(e, point_at(e, this.addr(), first));
    let [b0, b1, b2] = point_words(e, point_at(e, this.addr(), second));
    let [c0, c1, c2] = point_words(e, point_at(e, this.addr(), third));
    fn_00c4dcf0(
        e,
        this,
        f32::from_bits(a0),
        f32::from_bits(a1),
        f32::from_bits(a2),
        f32::from_bits(b0),
        f32::from_bits(b1),
        f32::from_bits(b2),
        f32::from_bits(c0),
        f32::from_bits(c1),
        f32::from_bits(c2),
    )
}

// Translated from 00c4cf80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c4cf80 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2ea78, state variable) is not translated
/// Triangulates the polygon by clipping ears (unnamed in the engine map; `thiscall`). A
/// stack array (virtual table `01016024`, set up by `006b3eb0`) gets every point number
/// (`007cb2e0` with the address of the counter). From the third point, while the current
/// point is not equal to the first (`004390c0`) and more than three numbers remain: the
/// numbers before the current one (k - 2, k - 1, wrapped into the array) and the current one
/// form a candidate ear ([`fn_00c4dd60`]). Not an ear: the next point becomes current. An
/// ear: the triangle index array (+8; made when missing: 0x10 bytes from the memory manager,
/// virtual table `0106abe4`, `00692290`) gets the three numbers (`00691820`; current, k - 1,
/// k - 2), the triangle count goes up, the number k - 1 leaves the stack array
/// ([`fn_00c4e9b0`] with shrinking), and with an outline array its entries for the two
/// neighbours are removed ([`fn_00c4eb50`]) when those points have become convex
/// ([`fn_00c4dcf0`] answering 1); when the number before the current one is 0 the next point
/// becomes current. After the loop, three remaining numbers make the last triangle (when the
/// triangle array exists). With triangles, the triangle data ([`fn_00c4e1e0`]) gets a shape
/// ([`fn_00c4e160`], stored at +0x30), a new node (`NiNew` 0xAC, `00a5ecb0` with 0; stored
/// at +0x2C) gets the shape (its virtual `+0xDC` with 1), and when the shape exists the data
/// pointers +0xC to +0x1C are cleared. The stack array is destroyed (`00438690`).
pub fn fn_00c4cf80(e: &mut Engine, this: Ptr) {
    let address = this.addr();
    e.with_stack(0x40, |e, locals| {
        let order = locals.addr();
        let counter = order + 0x10;
        let current = order + 0x18;
        let first = order + 0x24;
        e.mem.set_u32(order, ORDER_ARRAY_VTABLE);
        e.call(ORDER_ARRAY_SET_UP, &args![order, 0u32, 0u32]);
        e.mem.set_u32(counter, 0);
        loop {
            let points = e.mem.u32(address + POINTS);
            if e.mem.u32(counter) >= e.mem.u32(points + 8) {
                break;
            }
            e.call(ORDER_ARRAY_APPEND, &args![order, counter]);
            let next = e.mem.u32(counter).wrapping_add(1);
            e.mem.set_u32(counter, next);
        }
        let order_ptr = Ptr::<BSSimpleArray>::new(order);
        let number = |e: &Engine, position: u32| {
            let buffer = e.get(order_ptr, BSSimpleArray::pBuffer);
            e.mem.u32(buffer.wrapping_add(position.wrapping_mul(4)))
        };
        let copy_point = |e: &mut Engine, to: u32, point_number: u32| {
            let words = point_words(e, point_at(e, address, point_number));
            for (k, word) in words.iter().enumerate() {
                e.mem.set_u32(to + 4 * k as u32, *word);
            }
        };
        let mut current_position: u32 = 2;
        copy_point(e, current, 2);
        // 00c4d029: the first point is copied once, before the loop (whose head is 00c4d04c).
        copy_point(e, first, 0);
        loop {
            if e.call(POINT_EQUALS, &args![current, first]).u8() != 0 {
                break;
            }
            if e.get(order_ptr, BSSimpleArray::iSize) <= 3 {
                break;
            }
            let size = e.get(order_ptr, BSSimpleArray::iSize);
            let before_previous =
                wrap_index((current_position as i32).wrapping_sub(2), size) as u32;
            let previous = wrap_index((current_position as i32).wrapping_sub(1), size) as u32;
            let (corner, before, current_number) = (
                number(e, previous),
                number(e, before_previous),
                number(e, current_position),
            );
            let ear = fn_00c4dd60(e, this, corner, before, current_number);
            if !ear {
                current_position = current_position.wrapping_add(1);
                let size = e.get(order_ptr, BSSimpleArray::iSize);
                let point = number(e, current_position % size);
                copy_point(e, current, point);
                continue;
            }
            if e.mem.u32(address + TRIANGLES) == 0 {
                let block = e
                    .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
                    .u32();
                if block != 0 {
                    e.mem.set_u32(block, INDEX_ARRAY_VTABLE);
                    e.call(INDEX_ARRAY_SET_UP, &args![block, 0u32, 0u32]);
                }
                e.mem.set_u32(address + TRIANGLES, block);
            }
            for position in [current_position, previous, before_previous] {
                let value = number(e, position);
                let triangles = e.mem.u32(address + TRIANGLES);
                add_index(e, triangles, value);
            }
            let triangles = e.mem.u32(address + TRIANGLE_COUNT).wrapping_add(1);
            e.mem.set_u32(address + TRIANGLE_COUNT, triangles);
            current_position = current_position.wrapping_sub(1);
            fn_00c4e9b0(e, order_ptr, current_position, 1);
            let outline = e.mem.u32(address + OUTLINE);
            if outline != 0 {
                let k = current_position as i32;
                let size = e.get(order_ptr, BSSimpleArray::iSize);
                let before = wrap_index(k.wrapping_sub(1), size) as u32;
                let after = wrap_index(k.wrapping_add(1), size) as u32;
                let found = outline_position(e, outline, number(e, current_position));
                if found != 0xffff_ffff {
                    let (a, b, c) = (
                        number(e, current_position),
                        number(e, before),
                        number(e, after),
                    );
                    if orientation_of(e, this, a, b, c) == 1 {
                        let outline = e.mem.u32(address + OUTLINE);
                        fn_00c4eb50(e, Ptr::new(outline), found, 1);
                    }
                }
                let size = e.get(order_ptr, BSSimpleArray::iSize);
                let before = wrap_index(k.wrapping_sub(1), size) as u32;
                let before_that = wrap_index(k.wrapping_sub(2), size) as u32;
                let outline = e.mem.u32(address + OUTLINE);
                let found = outline_position(e, outline, number(e, before));
                if found != 0xffff_ffff {
                    let (a, b, c) = (
                        number(e, before),
                        number(e, before_that),
                        number(e, current_position),
                    );
                    if orientation_of(e, this, a, b, c) == 1 {
                        let outline = e.mem.u32(address + OUTLINE);
                        fn_00c4eb50(e, Ptr::new(outline), found, 1);
                    }
                }
            }
            let size = e.get(order_ptr, BSSimpleArray::iSize);
            let previous = wrap_index((current_position as i32).wrapping_sub(1), size) as u32;
            if number(e, previous) == 0 {
                current_position = current_position.wrapping_add(1);
                let size = e.get(order_ptr, BSSimpleArray::iSize);
                let point = number(e, current_position % size);
                copy_point(e, current, point);
            }
        }
        if e.mem.u32(address + TRIANGLES) != 0 && e.get(order_ptr, BSSimpleArray::iSize) == 3 {
            for position in [2u32, 1, 0] {
                let value = number(e, position);
                let triangles = e.mem.u32(address + TRIANGLES);
                add_index(e, triangles, value);
            }
            let triangles = e.mem.u32(address + TRIANGLE_COUNT).wrapping_add(1);
            e.mem.set_u32(address + TRIANGLE_COUNT, triangles);
        }
        if e.mem.i32(address + TRIANGLE_COUNT) > 0 {
            let data = fn_00c4e1e0(e, this);
            let shape = fn_00c4e160(e, this, data);
            e.call(ASSIGN_POINTER, &args![address + SHAPE, shape]);
            let block = e.call(NI_NEW, &args![0xacu32]).u32();
            let node = if block == 0 {
                0
            } else {
                e.call(NODE_CONSTRUCTOR, &args![block, 0u32]).u32()
            };
            e.call(ASSIGN_POINTER, &args![address + NODE, node]);
            let node = e.mem.u32(address + NODE);
            let shape = e.mem.u32(address + SHAPE);
            e.vcall(node, SLOT_ATTACH, &args![shape, 1u32]);
            if e.mem.u32(address + SHAPE) != 0 {
                for field in [VERTICES, NORMALS, TEXTURE_COORDINATES, COLORS, INDICES] {
                    e.mem.set_u32(address + field, 0);
                }
            }
        }
        e.call(ORDER_ARRAY_DESTRUCTOR, &args![order]);
    });
}

// Translated from 00c4d9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The polygon's signed area (unnamed in the engine map; `thiscall`, answered in ST0):
/// `0.5 * sum(x[i] * y[i+1] - y[i] * x[i+1])` over the points (the next index wrapping by a
/// signed `IDIV`), summed in a `float` local, x87 in the exe's order, the product with the
/// `double` 0.5 stored as a `float`.
pub fn fn_00c4d9d0(e: &mut Engine, this: Ptr) -> f32 {
    let this = this.addr();
    let mut sum = 0.0f32.to_bits();
    let mut index = 0i32;
    while index < e.mem.i32(this + POINT_COUNT) {
        let next = index
            .wrapping_add(1)
            .wrapping_rem(e.mem.i32(this + POINT_COUNT));
        let point = point_at(e, this, index as u32);
        let other = point_at(e, this, next as u32);
        let product = x87_mul(x87_load32(e.mem.u32(point)), x87_m32(e.mem.u32(other + 4)));
        sum = x87_store32(x87_add(product, x87_m32(sum)));
        let point = point_at(e, this, index as u32);
        let other = point_at(e, this, next as u32);
        let product = x87_mul(x87_load32(e.mem.u32(point + 4)), x87_m32(e.mem.u32(other)));
        sum = x87_store32(x87_sub(x87_m32(sum), product));
        index = index.wrapping_add(1);
    }
    let half = f64::from_bits(e.mem.u64(HALF));
    let area = x87_store32(x87_mul(x87_load32(sum), half));
    x87_quiet(f32::from_bits(area))
}

// Translated from 00c4da80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The area of the triangle of three points (unnamed in the engine map; `thiscall`,
/// `RET 0xC`: the point numbers; answered in ST0): the edges from the first point
/// (`00439ef0`), `(e2.y * e1.x - e2.x * e1.y) / 2.0` (x87, the `double` at `01011590`) stored
/// as a `float`, then `fabs` (`00ec6cde`) of it, stored as a `float` again.
pub fn fn_00c4da80(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) -> f32 {
    let this = this.addr();
    e.with_stack(0x18, |e, locals| {
        let one = locals.addr();
        let two = one + 0xc;
        let origin = point_at(e, this, first);
        let end = point_at(e, this, second);
        e.call(POINT_MINUS, &args![end, one, origin]);
        let origin = point_at(e, this, first);
        let end = point_at(e, this, third);
        e.call(POINT_MINUS, &args![end, two, origin]);
        let left = x87_mul(x87_load32(e.mem.u32(two)), x87_m32(e.mem.u32(one + 4)));
        let right = x87_mul(x87_load32(e.mem.u32(two + 4)), x87_m32(e.mem.u32(one)));
        let cross = x87_sub(right, left);
        let two_value = f64::from_bits(e.mem.u64(TWO));
        let half = x87_store32(x87_div(cross, two_value));
        let pushed = x87_load32(x87_quiet_bits(half));
        let absolute = e.call(FABS, &args![pushed]).f64();
        x87_quiet(f32::from_bits(x87_store32(absolute)))
    })
}

// Translated from 00c4db20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c4db20 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2eaa1, state variable) is not translated
/// Builds the outline index array (unnamed in the engine map; `thiscall`): only when there
/// is none yet (+4). A new array (0x10 bytes from the memory manager, `006917d0`; null when
/// that fails) is stored, then each point is classed by [`fn_00c4dcf0`] with its neighbours
/// (the previous one wrapping to the last, the next by a signed `IDIV`): -1 adds its number
/// (`00691820`); 0 removes the point ([`fn_00c4e760`] with shrinking), the count drops and
/// the same index is looked at again.
pub fn fn_00c4db20(e: &mut Engine, this: Ptr) {
    let address = this.addr();
    if e.mem.u32(address + OUTLINE) != 0 {
        return;
    }
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
        .u32();
    let outline = if block == 0 {
        0
    } else {
        e.call(INDEX_ARRAY_CONSTRUCTOR, &args![block]).u32()
    };
    e.mem.set_u32(address + OUTLINE, outline);
    let mut removed = false;
    let mut index = 0i32;
    while index < e.mem.i32(address + POINT_COUNT) {
        if removed {
            index = index.wrapping_sub(1);
        }
        let count = e.mem.i32(address + POINT_COUNT);
        let previous = if index == 0 {
            count.wrapping_sub(1)
        } else {
            index.wrapping_sub(1)
        };
        let next = index
            .wrapping_add(1)
            .wrapping_rem(e.mem.i32(address + POINT_COUNT));
        let class = orientation_of(e, this, index as u32, previous as u32, next as u32);
        removed = false;
        if class == -1 {
            let outline = e.mem.u32(address + OUTLINE);
            add_index(e, outline, index as u32);
        } else if class == 0 {
            let points = e.mem.u32(address + POINTS);
            fn_00c4e760(e, Ptr::new(points), index as u32, 1);
            let count = e.mem.i32(address + POINT_COUNT).wrapping_sub(1);
            e.mem.set_i32(address + POINT_COUNT, count);
            removed = true;
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00c4dcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Classes the turn at a point (unnamed in the engine map; `thiscall`, `RET 0x24`: three
/// points by value, x, y and z each; the z words are not read):
/// `(a.x - b.x) * (c.y - a.y) - (a.y - b.y) * (c.x - a.x)` (x87, stored as a `float`) answers
/// 1 when below 0.0, -1 when above, 0 when zero or a NaN.
#[allow(clippy::too_many_arguments)]
pub fn fn_00c4dcf0(
    _e: &mut Engine,
    _this: Ptr,
    a_x: f32,
    a_y: f32,
    _unused_3: f32,
    b_x: f32,
    b_y: f32,
    _unused_6: f32,
    c_x: f32,
    c_y: f32,
    _unused_9: f32,
) -> i32 {
    let first = x87_sub(x87_load32(a_x.to_bits()), x87_m32(b_x.to_bits()));
    let second = x87_sub(x87_load32(c_y.to_bits()), x87_m32(a_y.to_bits()));
    let left = x87_mul(first, second);
    let third = x87_sub(x87_load32(a_y.to_bits()), x87_m32(b_y.to_bits()));
    let fourth = x87_sub(x87_load32(c_x.to_bits()), x87_m32(a_x.to_bits()));
    let right = x87_mul(third, fourth);
    let turn = x87_load32(x87_store32(x87_sub(left, right)));
    if turn < 0.0 {
        1
    } else if turn > 0.0 {
        -1
    } else {
        0
    }
}

// Translated from 00c4dd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether point `index` with its neighbours `previous` and `next` is an ear (unnamed in the
/// engine map; `thiscall`, `RET 0xC`). True without an outline array or with an empty one.
/// False when `0069dee0` finds the low 16 bits of `index` in the outline array; otherwise
/// false as soon as an outline point other than the two neighbours lies in the triangle
/// ([`fn_00c4de80`] with its x and y, `previous`, `index`, `next`), else true.
pub fn fn_00c4dd60(e: &mut Engine, this: Ptr, index: u32, previous: u32, next: u32) -> bool {
    let address = this.addr();
    let outline = e.mem.u32(address + OUTLINE);
    if outline == 0 || e.mem.u32(outline + 8) == 0 {
        return true;
    }
    let listed = e.with_stack(4, |e, slot| {
        e.mem.set_u16(slot.addr(), index as u16);
        let outline = e.mem.u32(address + OUTLINE);
        e.call(INDEX_ARRAY_TEST, &args![outline, slot]).u8()
    });
    if listed != 0 {
        return false;
    }
    let mut position = 0u32;
    while position < e.mem.u32(e.mem.u32(address + OUTLINE) + 8) {
        let buffer = e.mem.u32(e.mem.u32(address + OUTLINE) + 4);
        let point_number = u32::from(e.mem.u16(buffer.wrapping_add(position * 2)));
        if point_number != previous && point_number != next {
            let point = point_at(e, address, point_number);
            let x = x87_quiet_bits(e.mem.u32(point));
            let y = x87_quiet_bits(e.mem.u32(point + 4));
            if fn_00c4de80(
                e,
                this,
                f32::from_bits(x),
                f32::from_bits(y),
                previous,
                index,
                next,
            ) {
                return false;
            }
        }
        position = position.wrapping_add(1);
    }
    true
}

// Translated from 00c4de80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the point (x, y) lies in the triangle of the points numbered `first`, `second`
/// and `third` (unnamed in the engine map; `thiscall`, `RET 0x14`): the three edge cross
/// products `(edge.x * (y - start.y) - edge.y * (x - start.x))`, x87, each stored as a
/// `float`, must all be at most 0.0 (a NaN fails).
pub fn fn_00c4de80(
    e: &mut Engine,
    this: Ptr,
    x: f32,
    y: f32,
    first: u32,
    second: u32,
    third: u32,
) -> bool {
    let address = this.addr();
    let coordinate =
        |e: &Engine, number: u32, offset: u32| e.mem.u32(point_at(e, address, number) + offset);
    let difference = |a: u32, b: u32| x87_store32(x87_sub(x87_load32(a), x87_m32(b)));
    let edge_one_x = difference(coordinate(e, third, 0), coordinate(e, second, 0));
    let edge_one_y = difference(coordinate(e, third, 4), coordinate(e, second, 4));
    let edge_two_x = difference(coordinate(e, first, 0), coordinate(e, third, 0));
    let edge_two_y = difference(coordinate(e, first, 4), coordinate(e, third, 4));
    let edge_three_x = difference(coordinate(e, second, 0), coordinate(e, first, 0));
    let edge_three_y = difference(coordinate(e, second, 4), coordinate(e, first, 4));
    let to_first_x = difference(x.to_bits(), coordinate(e, first, 0));
    let to_first_y = difference(y.to_bits(), coordinate(e, first, 4));
    let to_second_x = difference(x.to_bits(), coordinate(e, second, 0));
    let to_second_y = difference(y.to_bits(), coordinate(e, second, 4));
    let to_third_x = difference(x.to_bits(), coordinate(e, third, 0));
    let to_third_y = difference(y.to_bits(), coordinate(e, third, 4));
    let cross = |ax: u32, by: u32, ay: u32, bx: u32| {
        let left = x87_mul(x87_load32(ax), x87_m32(by));
        let right = x87_mul(x87_load32(ay), x87_m32(bx));
        x87_load32(x87_store32(x87_sub(left, right)))
    };
    let one = cross(edge_one_x, to_second_y, edge_one_y, to_second_x);
    let two = cross(edge_three_x, to_first_y, edge_three_y, to_first_x);
    let three = cross(edge_two_x, to_third_y, edge_two_y, to_third_x);
    let zero = f64::from_bits(e.mem.u64(ZERO));
    one <= zero && three <= zero && two <= zero
}

// Translated from 00c4e0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the point (x, y) lies in one of the polygon's triangles (unnamed in the engine
/// map; `thiscall`, `RET 8`): each triangle's three 16-bit point numbers from the triangle
/// index array (+8) go to [`fn_00c4de80`], up to the triangle count (+0x28, signed).
pub fn fn_00c4e0a0(e: &mut Engine, this: Ptr, x: f32, y: f32) -> bool {
    let address = this.addr();
    let mut triangle = 0i32;
    while triangle < e.mem.i32(address + TRIANGLE_COUNT) {
        let base = (triangle as u32).wrapping_mul(3);
        let number = |e: &Engine, k: u32| {
            let buffer = e.mem.u32(e.mem.u32(address + TRIANGLES) + 4);
            u32::from(
                e.mem
                    .u16(buffer.wrapping_add(base.wrapping_add(k).wrapping_mul(2))),
            )
        };
        let (first, second, third) = (number(e, 0), number(e, 1), number(e, 2));
        if fn_00c4de80(e, this, x, y, first, second, third) {
            return true;
        }
        triangle = triangle.wrapping_add(1);
    }
    false
}

// Translated from 00c4e160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c4e160 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2eace, state variable) is not translated
/// Makes the polygon's shape (unnamed in the engine map; `thiscall`, `RET 4`: the triangle
/// data): a 0xC4-byte `NiNew` block constructed as an `NiTriShape` from the data
/// (`00a74480`), or null when the allocation fails.
pub fn fn_00c4e160(e: &mut Engine, _this: Ptr, data: u32) -> u32 {
    let block = e.call(NI_NEW, &args![0xc4u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(TRI_SHAPE_CONSTRUCTOR, &args![block, data]).u32()
}

/// The size in bytes of `count` elements of `size` bytes as the exe's array `new`
/// computes it (`MUL`; an overflow gives `0xFFFFFFFF`).
fn array_bytes(count: u32, size: u32) -> u32 {
    u32::try_from(u64::from(count) * u64::from(size)).unwrap_or(u32::MAX)
}

// Translated from 00c4e1e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c4e1e0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2eaf8, state variable) is not translated
/// Makes the polygon's triangle data (unnamed in the engine map; `thiscall`): vertices and
/// normals (12 bytes per point), texture coordinates (8) and colours (16, each set to
/// 0, 0, 0, 0 when the block exists) from the memory manager (null when it fails), and the
/// triangle indices (6 bytes per triangle, `00aa1070`), stored at +0xC to +0x1C. The points
/// and the triangle index array's indices are copied in (`memcpy`); every normal is
/// (0, 0, 1), every texture coordinate the point's x and y divided by the `float` at
/// `01016410` (x87 `FDIVR`, stored as `float`s), every colour (1, 1, 1, 1). A 0x58-byte
/// `NiNew` block becomes the `NiTriShapeData` (`00a7b630`: the 16-bit point count, vertices,
/// normals, colours, texture coordinates, 1, 0, the 16-bit triangle count, indices); answers
/// it, or null when the allocation fails.
pub fn fn_00c4e1e0(e: &mut Engine, this: Ptr) -> u32 {
    let address = this.addr();
    let allocate = |e: &mut Engine, size: u32| {
        e.call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
            .u32()
    };
    let count = e.mem.u32(address + POINT_COUNT);
    let vertices = allocate(e, array_bytes(count, 0xc));
    e.mem.set_u32(address + VERTICES, vertices);
    let count = e.mem.u32(address + POINT_COUNT);
    let normals = allocate(e, array_bytes(count, 0xc));
    e.mem.set_u32(address + NORMALS, normals);
    let count = e.mem.u32(address + POINT_COUNT);
    let coordinates = allocate(e, array_bytes(count, 8));
    e.mem.set_u32(address + TEXTURE_COORDINATES, coordinates);
    let count = e.mem.u32(address + POINT_COUNT);
    let colors = allocate(e, array_bytes(count, 0x10));
    if colors != 0 {
        let mut remaining = count as i32;
        let mut color = colors;
        loop {
            remaining = remaining.wrapping_sub(1);
            if remaining < 0 {
                break;
            }
            for k in 0..4u32 {
                e.mem.set_f32(color + 4 * k, 0.0);
            }
            color = color.wrapping_add(0x10);
        }
    }
    e.mem.set_u32(address + COLORS, colors);
    let triangles = e.mem.u32(address + TRIANGLE_COUNT);
    let indices = e
        .call(NI_ALLOC, &args![triangles.wrapping_mul(3).wrapping_mul(2)])
        .u32();
    e.mem.set_u32(address + INDICES, indices);
    let bytes = e.mem.u32(address + POINT_COUNT).wrapping_mul(0xc);
    let source = point_at(e, address, 0);
    let destination = e.mem.u32(address + VERTICES);
    e.call(MEMCPY, &args![destination, source, bytes]);
    let bytes = e.mem.u32(address + TRIANGLE_COUNT).wrapping_mul(6);
    let source = e.mem.u32(e.mem.u32(address + TRIANGLES) + 4);
    let destination = e.mem.u32(address + INDICES);
    e.call(MEMCPY, &args![destination, source, bytes]);
    let mut index = 0i32;
    while index < e.mem.i32(address + POINT_COUNT) {
        let normal = e
            .mem
            .u32(address + NORMALS)
            .wrapping_add((index as u32).wrapping_mul(0xc));
        e.mem.set_f32(normal, 0.0);
        e.mem.set_f32(normal + 4, 0.0);
        e.mem.set_f32(normal + 8, 1.0);
        index = index.wrapping_add(1);
    }
    let scale = x87_quiet_bits(e.mem.u32(TEXTURE_SCALE));
    let mut index = 0i32;
    while index < e.mem.i32(address + POINT_COUNT) {
        let vertex = e
            .mem
            .u32(address + VERTICES)
            .wrapping_add((index as u32).wrapping_mul(0xc));
        let u = x87_store32(x87_div(x87_m32(e.mem.u32(vertex)), x87_load32(scale)));
        let vertex = e
            .mem
            .u32(address + VERTICES)
            .wrapping_add((index as u32).wrapping_mul(0xc));
        let v = x87_store32(x87_div(x87_m32(e.mem.u32(vertex + 4)), x87_load32(scale)));
        let coordinate = e
            .mem
            .u32(address + TEXTURE_COORDINATES)
            .wrapping_add((index as u32).wrapping_mul(8));
        e.mem.set_u32(coordinate, x87_quiet_bits(u));
        e.mem.set_u32(coordinate + 4, x87_quiet_bits(v));
        index = index.wrapping_add(1);
    }
    let mut index = 0i32;
    while index < e.mem.i32(address + POINT_COUNT) {
        let color = e
            .mem
            .u32(address + COLORS)
            .wrapping_add((index as u32).wrapping_mul(0x10));
        for k in 0..4u32 {
            e.mem.set_f32(color + 4 * k, 1.0);
        }
        index = index.wrapping_add(1);
    }
    let block = e.call(NI_NEW, &args![0x58u32]).u32();
    if block == 0 {
        return 0;
    }
    let points = u32::from(e.mem.u16(address + POINT_COUNT));
    let triangles = u32::from(e.mem.u16(address + TRIANGLE_COUNT));
    let words = [
        e.mem.u32(address + VERTICES),
        e.mem.u32(address + NORMALS),
        e.mem.u32(address + COLORS),
        e.mem.u32(address + TEXTURE_COORDINATES),
        e.mem.u32(address + INDICES),
    ];
    e.call(
        TRI_SHAPE_DATA_CONSTRUCTOR,
        &args![
            block, points, words[0], words[1], words[2], words[3], 1u32, 0u32, triangles, words[4]
        ],
    )
    .u32()
}

/// The removal of the element at `index` from a `BSSimpleArray` of `element`-byte elements,
/// as the three template instances below do it (`mover`: the element move, `thiscall` with
/// destination, source, count). With `shrink` set and the size at most a quarter of the
/// capacity (at most the capacity less 0x800 above a capacity of 0x400), the array moves to a
/// smaller buffer (the capacity less 0x400 above 0x800, else half; allocated by its virtual
/// `+4`, the old one freed by its virtual `+8`): the elements before `index` are moved, then
/// `size - 1` elements from after it (as the exe counts them). Otherwise the elements after
/// `index` move down by one. The size drops by one.
fn remove_at(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    index: u32,
    shrink: u8,
    element: u32,
    mover: u32,
) {
    let mut shrunk = false;
    if shrink != 0 {
        let capacity = e.get(this, BSSimpleArray::iReservedSize);
        let size = e.get(this, BSSimpleArray::iSize);
        let fits = if capacity > 0x400 {
            size <= capacity.wrapping_sub(0x800)
        } else {
            size <= capacity >> 2
        };
        if fits {
            let capacity = e.get(this, BSSimpleArray::iReservedSize);
            let new_capacity = if capacity > 0x800 {
                capacity.wrapping_sub(0x400)
            } else {
                capacity >> 1
            };
            let buffer = e.vcall(this.addr(), 4, &args![new_capacity]).u32();
            let old = e.get(this, BSSimpleArray::pBuffer);
            e.call(mover, &args![this, buffer, old, index]);
            let old = e.get(this, BSSimpleArray::pBuffer);
            let size = e.get(this, BSSimpleArray::iSize);
            let offset = index.wrapping_mul(element);
            e.call(
                mover,
                &args![
                    this,
                    buffer.wrapping_add(offset),
                    old.wrapping_add(offset).wrapping_add(element),
                    size.wrapping_sub(1)
                ],
            );
            let old = e.get(this, BSSimpleArray::pBuffer);
            e.vcall(this.addr(), 8, &args![old]);
            e.set(this, BSSimpleArray::pBuffer, 0);
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, new_capacity);
            shrunk = true;
        }
    }
    if !shrunk {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let size = e.get(this, BSSimpleArray::iSize);
        let offset = index.wrapping_mul(element);
        e.call(
            mover,
            &args![
                this,
                buffer.wrapping_add(offset),
                buffer.wrapping_add(offset).wrapping_add(element),
                size.wrapping_sub(index).wrapping_sub(1)
            ],
        );
    }
    let size = e.get(this, BSSimpleArray::iSize).wrapping_sub(1);
    e.set(this, BSSimpleArray::iSize, size);
}

// Translated from 00c4e760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPoint3>`'s removal at an index (unnamed in the engine map; `thiscall`,
/// `RET 8`: index, shrink byte): [`remove_at`] with 12-byte elements and `006dc6e0`.
pub fn fn_00c4e760(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, shrink: u8) {
    remove_at(e, this, index, shrink, 0xc, MOVE_12);
}

// Translated from 00c4e900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reverses a `BSSimpleArray<NiPoint3>` (unnamed in the engine map; `thiscall`): the first
/// and last points swap, moving inwards while the front index is below the back one
/// (signed).
pub fn fn_00c4e900(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    let mut front = 0i32;
    let mut back = (e.get(this, BSSimpleArray::iSize) as i32).wrapping_sub(1);
    while front < back {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let first = buffer.wrapping_add((front as u32).wrapping_mul(0xc));
        let last = buffer.wrapping_add((back as u32).wrapping_mul(0xc));
        let saved = point_words(e, first);
        let moved = point_words(e, last);
        for (k, word) in moved.iter().enumerate() {
            e.mem.set_u32(first + 4 * k as u32, *word);
        }
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let last = buffer.wrapping_add((back as u32).wrapping_mul(0xc));
        for (k, word) in saved.iter().enumerate() {
            e.mem.set_u32(last + 4 * k as u32, *word);
        }
        front = front.wrapping_add(1);
        back = back.wrapping_sub(1);
    }
}

// Translated from 00c4e9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray`'s removal at an index for 4-byte elements (unnamed in the engine map;
/// `thiscall`, `RET 8`: index, shrink byte): [`remove_at`] with `0042fb60`.
pub fn fn_00c4e9b0(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, shrink: u8) {
    remove_at(e, this, index, shrink, 4, MOVE_4);
}

// Translated from 00c4eb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray`'s removal at an index for 2-byte elements (unnamed in the engine map;
/// `thiscall`, `RET 8`: index, shrink byte): [`remove_at`] with `005efdd0`.
pub fn fn_00c4eb50(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, shrink: u8) {
    remove_at(e, this, index, shrink, 2, MOVE_2);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_c530, fn_00c4c530(Ptr, u32, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00c4_c8d0, fn_00c4c8d0(Ptr<NiTArray>, u32) -> u32),
        entry!(0x00c4_ca80, fn_00c4ca80(Ptr) -> Ptr),
        entry!(0x00c4_cba0, fn_00c4cba0(Ptr, Ptr) -> Ptr),
        entry!(0x00c4_ccd0, fn_00c4ccd0(Ptr)),
        entry!(0x00c4_cdc0, fn_00c4cdc0(Ptr)),
        entry!(0x00c4_ce00, fn_00c4ce00(Ptr)),
        entry!(0x00c4_cf00, fn_00c4cf00(Ptr)),
        entry!(0x00c4_cf80, fn_00c4cf80(Ptr)),
        entry!(0x00c4_d9d0, fn_00c4d9d0(Ptr) -> f32),
        entry!(0x00c4_da80, fn_00c4da80(Ptr, u32, u32, u32) -> f32),
        entry!(0x00c4_db20, fn_00c4db20(Ptr)),
        entry!(
            0x00c4_dcf0,
            fn_00c4dcf0(Ptr, f32, f32, f32, f32, f32, f32, f32, f32, f32) -> i32
        ),
        entry!(0x00c4_dd60, fn_00c4dd60(Ptr, u32, u32, u32) -> bool),
        entry!(0x00c4_de80, fn_00c4de80(Ptr, f32, f32, u32, u32, u32) -> bool),
        entry!(0x00c4_e0a0, fn_00c4e0a0(Ptr, f32, f32) -> bool),
        entry!(0x00c4_e160, fn_00c4e160(Ptr, u32) -> u32),
        entry!(0x00c4_e1e0, fn_00c4e1e0(Ptr) -> u32),
        entry!(0x00c4_e760, fn_00c4e760(Ptr<BSSimpleArray>, u32, u8)),
        entry!(0x00c4_e900, fn_00c4e900(Ptr<BSSimpleArray>)),
        entry!(0x00c4_e9b0, fn_00c4e9b0(Ptr<BSSimpleArray>, u32, u8)),
        entry!(0x00c4_eb50, fn_00c4eb50(Ptr<BSSimpleArray>, u32, u8)),
        entry!(0x00c4_c2d0, fn_00c4c2d0(Ptr, u32, Ptr)),
        entry!(0x00c4_c920, fn_00c4c920(Ptr<NiTArray>, u32)),
        entry!(0x00c4_ecf0, fn_00c4ecf0(Ptr) -> Ptr),
        entry!(0x00c4_ed40, fn_00c4ed40(Ptr)),
        entry!(0x00c4_ed70, fn_00c4ed70(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const QUERY: u32 = 0x00ee_0100;
    const RELEASE: u32 = 0x00ee_0200;
    const ISSUE: u32 = 0x00ee_0300;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    #[test]
    fn fn_00c4c2d0_builds_the_inverse_matrix() {
        let mut e = Engine::new();
        e.register(TRANSFORM_INVERT_NON_UNIFORM, |e, a| {
            for k in 0..9 {
                e.mem.set_f32(a[1] + 4 * k, k as f32 + 1.0);
            }
            for k in 0..3 {
                e.mem.set_f32(a[1] + 0x24 + 4 * k, 10.0 + k as f32);
            }
            e.mem.set_f32(a[1] + 0x30, 2.0);
            Ret::default()
        });
        let out = e.mem.alloc(0x40);
        e.call(0x00c4_c2d0, &args![0x5000u32, 0u32, out]);
        let matrix: Vec<f32> = (0..16).map(|k| e.mem.f32(out + 4 * k)).collect();
        assert_eq!(
            matrix,
            [
                2.0, 8.0, 14.0, 0.0, 4.0, 10.0, 16.0, 0.0, 6.0, 12.0, 18.0, 0.0, 10.0, 11.0, 12.0,
                1.0
            ]
        );
        // The fixed form inverts the identity with the fixed translation.
        let log = Log::default();
        let l = log.clone();
        e.register_double(D3DX_MATRIX_INVERSE, move |e, a| {
            let input: Vec<u32> = (0..16).map(|k| e.mem.u32(a[2] + 4 * k)).collect();
            l.borrow_mut().push((a[0], input));
            Ret::default()
        });
        e.map(FIXED_TRANSLATION, 12);
        e.mem.set_f32(FIXED_TRANSLATION, 7.0);
        e.mem.set_f32(FIXED_TRANSLATION + 8, 9.0);
        e.call(0x00c4_c2d0, &args![0x5000u32, 1u32, out]);
        let one = 1.0f32.to_bits();
        assert_eq!(
            *log.borrow(),
            [(
                out,
                vec![
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
                    7.0f32.to_bits(),
                    0,
                    9.0f32.to_bits(),
                    one
                ]
            )]
        );
    }

    #[test]
    fn fn_00c4c920_resizes_the_array() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0x20);
        logged(&mut e, &log, NI_ALLOC, block);
        logged(&mut e, &log, NI_FREE, 0);
        let old = e.mem.alloc(0x20);
        for k in 0..5 {
            e.mem
                .set_u32(old + 4 * k, if k == 1 { 0 } else { 0x100 + k });
        }
        let this: Ptr<NiTArray> = e.new_object();
        e.set(this, NiTArray::m_pBase, old);
        e.set(this, NiTArray::m_usMaxSize, 5);
        e.set(this, NiTArray::m_usSize, 5);
        e.set(this, NiTArray::m_usESize, 4);
        // Shrinking to 3 clears slots 3 and 4.
        e.call(0x00c4_c920, &args![this, 3u32]);
        assert_eq!(e.get(this, NiTArray::m_usSize), 3);
        assert_eq!(e.get(this, NiTArray::m_usESize), 2);
        assert_eq!(e.get(this, NiTArray::m_usMaxSize), 3);
        assert_eq!(e.get(this, NiTArray::m_pBase), block);
        assert_eq!(
            (0..3).map(|k| e.mem.u32(block + 4 * k)).collect::<Vec<_>>(),
            [0x100, 0, 0x102]
        );
        assert_eq!(*log.borrow(), [(NI_ALLOC, vec![12]), (NI_FREE, vec![old])]);
        // Growing to 6 copies the used slots and zeroes the rest.
        let bigger = e.mem.alloc(0x20);
        e.mem.write(bigger, &[0xaa; 0x20]);
        logged(&mut e, &log, NI_ALLOC, bigger);
        e.call(0x00c4_c920, &args![this, 6u32]);
        assert_eq!(e.mem.u32(bigger + 8), 0x102);
        assert_eq!(e.mem.u32(bigger + 20), 0);
        assert_eq!(e.mem.u32(bigger + 24), 0xaaaa_aaaa);
        // The same capacity: nothing; zero: no block.
        log.borrow_mut().clear();
        e.call(0x00c4_c920, &args![this, 6u32]);
        assert!(log.borrow().is_empty());
        e.set(this, NiTArray::m_usSize, 0);
        e.call(0x00c4_c920, &args![this, 0u32]);
        assert_eq!(e.get(this, NiTArray::m_pBase), 0);
        assert_eq!(*log.borrow(), [(NI_FREE, vec![bigger])]);
    }

    #[test]
    fn device_helpers_call_their_com_methods() {
        let mut e = Engine::new();
        let log = Log::default();
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + SLOT_DEVICE_QUERY, QUERY);
        e.mem.set_u32(vtable + SLOT_RELEASE, RELEASE);
        e.mem.set_u32(vtable + SLOT_ISSUE, ISSUE);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, vtable);
        let renderer = e.mem.alloc(0x290);
        e.mem.set_u32(renderer + RENDERER_DEVICE, device);
        e.map(RENDERER, 4);
        e.mem.set_u32(RENDERER, renderer);
        let l = log.clone();
        e.register_double(QUERY, move |e, a| {
            l.borrow_mut().push((QUERY, a.to_vec()));
            e.mem.set_u32(a[2], 0x55);
            Ret::default()
        });
        logged(&mut e, &log, RELEASE, 0);
        logged(&mut e, &log, ISSUE, 0);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0xffff);
        assert_eq!(e.call(0x00c4_ecf0, &args![value]).u32(), value);
        assert_eq!(e.mem.u32(value), 0x55);
        e.mem.set_u32(value, device);
        e.call(0x00c4_ed40, &args![value]);
        e.map(ISSUED_ONCE, 4);
        assert!(e.call(0x00c4_ed70, &args![value]).bool());
        assert!(!e.call(0x00c4_ed70, &args![value]).bool());
        assert_eq!(
            *log.borrow(),
            [
                (QUERY, vec![device, 9, value]),
                (RELEASE, vec![device]),
                (ISSUE, vec![device, 2]),
            ]
        );
        e.mem.set_u32(value, 0);
        e.call(0x00c4_ed40, &args![value]);
        assert_eq!(log.borrow().len(), 3);
    }
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles each address with a recorder answering `eax`.
    fn record(e: &mut Engine, log: &Log, addresses: &[u32], eax: u32) {
        for &address in addresses {
            let log = log.clone();
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                eax.into_ret()
            });
        }
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with the constants mapped and real `NiPoint3` helpers, element moves and
    /// allocations.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(ZERO, 8);
        e.mem.set_u64(ZERO, 0.0f64.to_bits());
        e.map(HALF, 8);
        e.mem.set_u64(HALF, 0.5f64.to_bits());
        e.map(TWO, 8);
        e.mem.set_u64(TWO, 2.0f64.to_bits());
        e.map(TEXTURE_SCALE, 4);
        e.mem.set_f32(TEXTURE_SCALE, 2.0);
        e.map(ZERO_POINT, 12);
        e.register(POINT_ADD_TO, |e, a| {
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) + e.mem.f32(a[1] + 4 * k);
                e.mem.set_f32(a[0] + 4 * k, v);
            }
            Ret::default()
        });
        e.register(POINT_MINUS, |e, a| {
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) - e.mem.f32(a[2] + 4 * k);
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_DIVIDE, |e, a| {
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) / f32::from_bits(a[2]);
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_EQUALS, |e, a| {
            u32::from((0..3).all(|k| e.mem.u32(a[0] + 4 * k) == e.mem.u32(a[1] + 4 * k))).into_ret()
        });
        for (mover, size) in [(MOVE_12, 12u32), (MOVE_4, 4), (MOVE_2, 2)] {
            e.register_double(mover, move |e, a| {
                let bytes = e.mem.bytes(a[2], a[3].wrapping_mul(size));
                e.mem.write(a[1], &bytes);
                Ret::default()
            });
        }
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            e.mem.alloc(a[1].max(4)).into_ret()
        });
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e
    }

    /// A `BSSimpleArray<NiPoint3>` of the given points with room for `capacity`.
    fn points_array(e: &mut Engine, points: &[(f32, f32, f32)], capacity: u32) -> u32 {
        let array = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(12 * capacity);
        for (i, (x, y, z)) in points.iter().enumerate() {
            let p = buffer + 12 * i as u32;
            e.mem.set_f32(p, *x);
            e.mem.set_f32(p + 4, *y);
            e.mem.set_f32(p + 8, *z);
        }
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, points.len() as u32);
        e.mem.set_u32(array + 0xc, capacity);
        array
    }

    fn polygon(e: &mut Engine, points: &[(f32, f32, f32)]) -> u32 {
        let array = points_array(e, points, 0x10);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + POINTS, array);
        e.mem.set_u32(this + POINT_COUNT, points.len() as u32);
        this
    }

    fn points_of(e: &Engine, array: u32) -> Vec<(f32, f32, f32)> {
        let buffer = e.mem.u32(array + 4);
        (0..e.mem.u32(array + 8))
            .map(|i| {
                let p = buffer + 12 * i;
                (e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8))
            })
            .collect()
    }

    /// A 16-bit index array (buffer +4, count +8) holding `indices`.
    fn index_array(e: &mut Engine, indices: &[u16]) -> u32 {
        let array = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(2 * indices.len() as u32 + 2);
        for (i, index) in indices.iter().enumerate() {
            e.mem.set_u16(buffer + 2 * i as u32, *index);
        }
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, indices.len() as u32);
        array
    }

    const SQUARE: [(f32, f32, f32); 4] = [
        (0.0, 0.0, 5.0),
        (1.0, 0.0, 5.0),
        (1.0, 1.0, 5.0),
        (0.0, 1.0, 5.0),
    ];

    #[test]
    fn points_and_normals_are_transformed() {
        let mut e = Engine::new();
        let transform = e.mem.alloc(0x34);
        // Rotation rows (0,1,0), (1,0,0), (0,0,1); translation (1, 2, 3); scale 2.
        for (k, v) in [
            0.0f32, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 2.0,
        ]
        .iter()
        .enumerate()
        {
            e.mem.set_f32(transform + 4 * k as u32, *v);
        }
        let input = e.mem.alloc(24);
        for (k, v) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0].iter().enumerate() {
            e.mem.set_f32(input + 4 * k as u32, *v);
        }
        let out = e.mem.alloc(24);
        let normals_out = e.mem.alloc(24);
        e.call(
            0x00c4_c530,
            &args![transform, 2u32, input, out, input, normals_out],
        );
        let read = |e: &Engine, a: u32| (0..6).map(|k| e.mem.f32(a + 4 * k)).collect::<Vec<_>>();
        assert_eq!(read(&e, out), vec![5.0, 4.0, 9.0, 11.0, 10.0, 15.0]);
        assert_eq!(read(&e, normals_out), vec![2.0, 1.0, 3.0, 5.0, 4.0, 6.0]);
        // In place: each coordinate is stored before the next row reads the point.
        e.call(
            0x00c4_c530,
            &args![transform, 1u32, input, input, 0u32, 0u32],
        );
        assert_eq!(e.mem.f32(input), 5.0);
        assert_eq!(e.mem.f32(input + 4), 2.0 * 5.0 + 2.0);
    }

    #[test]
    fn array_add_grows_when_full() {
        let mut e = Engine::new();
        let log: Log = Default::default();
        record(&mut e, &log, &[ARRAY_SET_AT, NI_FREE], 0);
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        let array: Ptr<NiTArray> = e.new_object();
        e.set(array, NiTArray::m_usMaxSize, 2);
        e.set(array, NiTArray::m_usSize, 1);
        e.set(array, NiTArray::m_usGrowBy, 3);
        let base = e.mem.alloc(8);
        e.set(array, NiTArray::m_pBase, base);
        assert_eq!(e.call(0x00c4_c8d0, &args![array, 0x77u32]).u32(), 1);
        assert_eq!(
            calls_to(&log, ARRAY_SET_AT),
            vec![vec![array.addr(), 1, 0x77]]
        );
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 2);
        e.set(array, NiTArray::m_usSize, 2);
        assert_eq!(e.call(0x00c4_c8d0, &args![array, 0x88u32]).u32(), 2);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 5);
        assert_eq!(calls_to(&log, ARRAY_SET_AT)[1], vec![array.addr(), 2, 0x88]);
    }

    #[test]
    fn constructors_start_empty_and_build_the_outline() {
        let mut e = engine();
        let log: Log = Default::default();
        record(&mut e, &log, &[ASSIGN_POINTER, INDEX_ARRAY_ADD], 0);
        let outline = e.mem.alloc(0x10);
        record(&mut e, &log, &[INDEX_ARRAY_CONSTRUCTOR], outline);
        e.mem.set_f32(ZERO_POINT, 0.5);
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0xaa; 0x40]);
        assert_eq!(e.call(0x00c4_ca80, &args![this]).u32(), this);
        for field in [
            POINTS,
            OUTLINE,
            TRIANGLES,
            POINT_COUNT,
            TRIANGLE_COUNT,
            NODE,
            SHAPE,
        ] {
            assert_eq!(e.mem.u32(this + field), 0, "field {field:#x}");
        }
        assert_eq!(e.mem.f32(this + CENTER), 0.5);
        assert_eq!(
            calls_to(&log, ASSIGN_POINTER),
            vec![vec![this + SHAPE, 0], vec![this + NODE, 0]]
        );
        // From a counter-clockwise square: its area is positive, so the points are reversed
        // before the outline is classified (every corner convex: all four added).
        let array = points_array(&mut e, &SQUARE, 8);
        let other = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00c4_cba0, &args![other, array]).u32(), other);
        assert_eq!(e.mem.u32(other + POINTS), array);
        assert_eq!(e.mem.u32(other + POINT_COUNT), 4);
        assert_eq!(e.mem.u32(other + OUTLINE), outline);
        assert_eq!(points_of(&e, array)[0], (0.0, 1.0, 5.0));
        let added = calls_to(&log, INDEX_ARRAY_ADD);
        assert_eq!(added.len(), 0, "a clockwise outline has no convex turns");
    }

    #[test]
    fn the_outline_keeps_convex_corners_and_drops_straight_ones() {
        let mut e = engine();
        let outline = e.mem.alloc(0x10);
        e.register_double(INDEX_ARRAY_CONSTRUCTOR, move |_, _| outline.into_ret());
        let added = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let inner = added.clone();
        e.register_double(INDEX_ARRAY_ADD, move |e, a| {
            assert_eq!(a[0], outline);
            inner.borrow_mut().push(e.mem.u16(a[1]));
            Ret::default()
        });
        let this = polygon(
            &mut e,
            &[
                (0.0, 0.0, 0.0),
                (1.0, 0.0, 0.0),
                (2.0, 0.0, 0.0),
                (2.0, 1.0, 0.0),
                (0.0, 1.0, 0.0),
            ],
        );
        e.call(0x00c4_db20, &args![this]);
        assert_eq!(*added.borrow(), vec![0, 1, 2, 3]);
        assert_eq!(e.mem.u32(this + POINT_COUNT), 4);
        assert_eq!(points_of(&e, e.mem.u32(this + POINTS))[1], (2.0, 0.0, 0.0));
        // An outline already there: nothing happens.
        added.borrow_mut().clear();
        e.call(0x00c4_db20, &args![this]);
        assert!(added.borrow().is_empty());
    }

    #[test]
    fn the_destructor_releases_the_pointers() {
        let mut e = Engine::new();
        let log: Log = Default::default();
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        record(&mut e, &log, &[0x00ee_b000, 0x00ee_b010], 0);
        e.put_vtable(0x0300_8000, &[0x00ee_b000, 0x00ee_b010]);
        let object = |e: &mut Engine, count: u32| {
            let o = e.mem.alloc(8);
            e.mem.set_u32(o, 0x0300_8000);
            e.mem.set_u32(o + 4, count);
            o
        };
        let points = object(&mut e, 1);
        let shape = object(&mut e, 1);
        let node = object(&mut e, 2);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + POINTS, points);
        e.mem.set_u32(this + SHAPE, shape);
        e.mem.set_u32(this + NODE, node);
        e.call(0x00c4_ccd0, &args![this]);
        assert_eq!(calls_to(&log, 0x00ee_b000), vec![vec![points, 1]]);
        assert_eq!(calls_to(&log, 0x00ee_b010), vec![vec![shape]]);
        assert_eq!(e.mem.u32(node + 4), 1);
        // With triangles the points are kept.
        e.mem.set_u32(this + TRIANGLE_COUNT, 1);
        e.mem.set_u32(this + SHAPE, 0);
        e.mem.set_u32(this + NODE, 0);
        e.call(0x00c4_ccd0, &args![this]);
        assert_eq!(calls_to(&log, 0x00ee_b000).len(), 1);
    }

    #[test]
    fn centring_moves_the_average_to_the_origin() {
        let mut e = engine();
        let this = polygon(&mut e, &[(1.0, 2.0, 3.0), (3.0, 6.0, 9.0)]);
        e.call(0x00c4_ce00, &args![this]);
        let points = points_of(&e, e.mem.u32(this + POINTS));
        assert_eq!(points, vec![(-1.0, -2.0, -3.0), (1.0, 2.0, 3.0)]);
        assert_eq!(
            (
                e.mem.f32(this + CENTER),
                e.mem.f32(this + CENTER + 4),
                e.mem.f32(this + CENTER + 8)
            ),
            (2.0, 4.0, 6.0)
        );
    }

    #[test]
    fn flattening_clears_every_z() {
        let mut e = engine();
        let this = polygon(&mut e, &SQUARE);
        e.call(0x00c4_cf00, &args![this]);
        let points = points_of(&e, e.mem.u32(this + POINTS));
        assert!(points.iter().all(|p| p.2 == 0.0));
        assert_eq!(points[2], (1.0, 1.0, 0.0));
    }

    #[test]
    fn areas() {
        let mut e = engine();
        let this = polygon(&mut e, &SQUARE);
        assert_eq!(e.call(0x00c4_d9d0, &args![this]).f32(), 1.0);
        let reversed: Vec<_> = SQUARE.iter().rev().copied().collect();
        let other = polygon(&mut e, &reversed);
        assert_eq!(e.call(0x00c4_d9d0, &args![other]).f32(), -1.0);
        e.register(FABS, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            value.abs().into_ret()
        });
        assert_eq!(
            e.call(0x00c4_da80, &args![this, 0u32, 1u32, 2u32]).f32(),
            0.5
        );
        assert_eq!(
            e.call(0x00c4_da80, &args![this, 0u32, 2u32, 1u32]).f32(),
            0.5
        );
    }

    #[test]
    fn turns_are_classed_by_sign() {
        let mut e = Engine::new();
        let turn = |e: &mut Engine, a: (f32, f32), b: (f32, f32), c: (f32, f32)| {
            e.call(
                0x00c4_dcf0,
                &args![0u32, a.0, a.1, 9.0f32, b.0, b.1, 9.0f32, c.0, c.1, 9.0f32],
            )
            .i32()
        };
        assert_eq!(turn(&mut e, (1.0, 0.0), (0.0, 0.0), (1.0, 1.0)), -1);
        assert_eq!(turn(&mut e, (1.0, 0.0), (1.0, 1.0), (0.0, 0.0)), 1);
        assert_eq!(turn(&mut e, (1.0, 0.0), (0.0, 0.0), (2.0, 0.0)), 0);
        assert_eq!(turn(&mut e, (f32::NAN, 0.0), (0.0, 0.0), (2.0, 0.0)), 0);
    }

    #[test]
    fn points_in_triangles() {
        let mut e = engine();
        let this = polygon(&mut e, &[(0.0, 0.0, 0.0), (0.0, 4.0, 0.0), (4.0, 0.0, 0.0)]);
        let inside = |e: &mut Engine, x: f32, y: f32| {
            e.call(0x00c4_de80, &args![this, x, y, 0u32, 1u32, 2u32])
                .bool()
        };
        assert!(inside(&mut e, 1.0, 1.0));
        assert!(inside(&mut e, 0.0, 0.0));
        assert!(!inside(&mut e, 3.0, 3.0));
        assert!(!inside(&mut e, f32::NAN, 1.0));
        // Through the triangle index array.
        let triangles = index_array(&mut e, &[2, 1, 0, 0, 1, 2]);
        e.mem.set_u32(this + TRIANGLES, triangles);
        e.mem.set_u32(this + TRIANGLE_COUNT, 2);
        assert!(e.call(0x00c4_e0a0, &args![this, 1.0f32, 1.0f32]).bool());
        assert!(!e.call(0x00c4_e0a0, &args![this, 9.0f32, 1.0f32]).bool());
        e.mem.set_u32(this + TRIANGLE_COUNT, 0);
        assert!(!e.call(0x00c4_e0a0, &args![this, 1.0f32, 1.0f32]).bool());
    }

    #[test]
    fn ears_need_no_outline_point_inside() {
        let mut e = engine();
        let this = polygon(
            &mut e,
            &[
                (0.0, 0.0, 0.0),
                (0.0, 4.0, 0.0),
                (4.0, 0.0, 0.0),
                (1.0, 1.0, 0.0),
                (9.0, 9.0, 0.0),
            ],
        );
        e.register(INDEX_ARRAY_TEST, |e, a| {
            u32::from(e.mem.u16(a[1]) == 7).into_ret()
        });
        // No outline: every corner is an ear.
        assert!(e.call(0x00c4_dd60, &args![this, 1u32, 0u32, 2u32]).bool());
        let outline = index_array(&mut e, &[0, 4]);
        e.mem.set_u32(this + OUTLINE, outline);
        assert!(e.call(0x00c4_dd60, &args![this, 1u32, 0u32, 2u32]).bool());
        let outline = index_array(&mut e, &[0, 3, 4]);
        e.mem.set_u32(this + OUTLINE, outline);
        assert!(!e.call(0x00c4_dd60, &args![this, 1u32, 0u32, 2u32]).bool());
        // A listed corner is no ear.
        assert!(!e.call(0x00c4_dd60, &args![this, 7u32, 0u32, 2u32]).bool());
        let empty = index_array(&mut e, &[]);
        e.mem.set_u32(this + OUTLINE, empty);
        assert!(e.call(0x00c4_dd60, &args![this, 7u32, 0u32, 2u32]).bool());
    }

    #[test]
    fn shapes_are_made_from_the_data() {
        let mut e = Engine::new();
        let log: Log = Default::default();
        record(&mut e, &log, &[NI_NEW], 0);
        assert_eq!(e.call(0x00c4_e160, &args![0u32, 0x99u32]).u32(), 0);
        record(&mut e, &log, &[NI_NEW], 0x4000);
        e.register(TRI_SHAPE_CONSTRUCTOR, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(e.call(0x00c4_e160, &args![0u32, 0x99u32]).u32(), 0x4099);
        assert_eq!(calls_to(&log, NI_NEW), vec![vec![0xc4], vec![0xc4]]);
    }

    #[test]
    fn triangle_data_gets_every_array() {
        let mut e = engine();
        let log: Log = Default::default();
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        record(&mut e, &log, &[NI_NEW], 0x5000);
        record(&mut e, &log, &[TRI_SHAPE_DATA_CONSTRUCTOR], 0x6000);
        let this = polygon(&mut e, &[(2.0, 4.0, 1.0), (6.0, 8.0, 1.0)]);
        let triangles = index_array(&mut e, &[1, 0, 1]);
        e.mem.set_u32(this + TRIANGLES, triangles);
        e.mem.set_u32(this + TRIANGLE_COUNT, 1);
        assert_eq!(e.call(0x00c4_e1e0, &args![this]).u32(), 0x6000);
        let f = |e: &Engine, field: u32, k: u32| e.mem.f32(e.mem.u32(this + field) + 4 * k);
        assert_eq!((f(&e, VERTICES, 3), f(&e, VERTICES, 4)), (6.0, 8.0));
        assert_eq!((f(&e, NORMALS, 0), f(&e, NORMALS, 2)), (0.0, 1.0));
        assert_eq!(
            (0..4)
                .map(|k| f(&e, TEXTURE_COORDINATES, k))
                .collect::<Vec<_>>(),
            vec![1.0, 2.0, 3.0, 4.0]
        );
        assert!((0..8).all(|k| f(&e, COLORS, k) == 1.0));
        let indices = e.mem.u32(this + INDICES);
        assert_eq!(e.mem.u16(indices + 4), 1);
        assert_eq!(
            calls_to(&log, TRI_SHAPE_DATA_CONSTRUCTOR),
            vec![vec![
                0x5000,
                2,
                e.mem.u32(this + VERTICES),
                e.mem.u32(this + NORMALS),
                e.mem.u32(this + COLORS),
                e.mem.u32(this + TEXTURE_COORDINATES),
                1,
                0,
                1,
                indices
            ]]
        );
        record(&mut e, &log, &[NI_NEW], 0);
        assert_eq!(e.call(0x00c4_e1e0, &args![this]).u32(), 0);
    }

    #[test]
    fn array_sizes_saturate() {
        assert_eq!(array_bytes(3, 0xc), 36);
        assert_eq!(array_bytes(0x2000_0000, 0x10), u32::MAX);
    }

    #[test]
    fn removals_move_the_tail_or_shrink_the_buffer() {
        let mut e = engine();
        let table = 0x0300_9000;
        e.put_vtable(table, &[0, 0x00ee_c000, 0x00ee_c010]);
        e.register(0x00ee_c000, |e, a| e.mem.alloc(a[1] * 12).into_ret());
        let freed = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let inner = freed.clone();
        e.register_double(0x00ee_c010, move |_, a| {
            inner.borrow_mut().push(a[1]);
            Ret::default()
        });
        let points = [(1.0, 0.0, 0.0), (2.0, 0.0, 0.0), (3.0, 0.0, 0.0)];
        // Plenty of room but no shrinking asked: the tail moves down.
        let array = points_array(&mut e, &points, 0x10);
        e.mem.set_u32(array, table);
        e.call(0x00c4_e760, &args![array, 0u32, 0u32]);
        assert_eq!(points_of(&e, array), vec![(2.0, 0.0, 0.0), (3.0, 0.0, 0.0)]);
        // Shrinking: a new buffer of half the capacity; the old one is freed.
        let array = points_array(&mut e, &points, 0x10);
        e.mem.set_u32(array, table);
        let old = e.mem.u32(array + 4);
        e.call(0x00c4_e760, &args![array, 1u32, 1u32]);
        assert_eq!(e.mem.u32(array + 0xc), 8);
        assert_ne!(e.mem.u32(array + 4), old);
        assert_eq!(*freed.borrow(), vec![old]);
        assert_eq!(e.mem.u32(array + 8), 2);
        assert_eq!(points_of(&e, array), vec![(1.0, 0.0, 0.0), (3.0, 0.0, 0.0)]);
        // Too full to shrink.
        let array = points_array(&mut e, &points, 4);
        e.mem.set_u32(array, table);
        e.call(0x00c4_e760, &args![array, 1u32, 1u32]);
        assert_eq!(e.mem.u32(array + 0xc), 4);
        // Words and 16-bit elements.
        let words = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(16);
        for k in 0..4u32 {
            e.mem.set_u32(buffer + 4 * k, 10 + k);
        }
        e.mem.write(words + 4, &buffer.to_le_bytes());
        e.mem.set_u32(words + 8, 4);
        e.mem.set_u32(words + 0xc, 4);
        e.call(0x00c4_e9b0, &args![words, 1u32, 1u32]);
        assert_eq!(
            (0..3)
                .map(|k| e.mem.u32(buffer + 4 * k))
                .collect::<Vec<_>>(),
            vec![10, 12, 13]
        );
        let halves = index_array(&mut e, &[5, 6, 7]);
        e.mem.set_u32(halves + 0xc, 3);
        e.call(0x00c4_eb50, &args![halves, 0u32, 1u32]);
        let buffer = e.mem.u32(halves + 4);
        assert_eq!((e.mem.u16(buffer), e.mem.u16(buffer + 2)), (6, 7));
        assert_eq!(e.mem.u32(halves + 8), 2);
    }

    #[test]
    fn reversing_swaps_from_both_ends() {
        let mut e = engine();
        let array = points_array(&mut e, &SQUARE, 4);
        e.call(0x00c4_e900, &args![array]);
        let reversed: Vec<_> = SQUARE.iter().rev().copied().collect();
        assert_eq!(points_of(&e, array), reversed);
        let odd = points_array(&mut e, &SQUARE[..3], 4);
        e.call(0x00c4_e900, &args![odd]);
        assert_eq!(points_of(&e, odd), vec![SQUARE[2], SQUARE[1], SQUARE[0]]);
    }

    #[test]
    fn a_square_becomes_two_triangles_and_a_node() {
        let mut e = engine();
        let log: Log = Default::default();
        let order_buffer = e.mem.alloc(0x20);
        e.register_double(ORDER_ARRAY_SET_UP, move |e, a| {
            e.mem.set_u32(a[0] + 4, order_buffer);
            e.mem.set_u32(a[0] + 8, 0);
            e.mem.set_u32(a[0] + 0xc, 8);
            Ret::default()
        });
        e.register(ORDER_ARRAY_APPEND, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            let buffer = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(buffer + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        let added = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let inner = added.clone();
        e.register_double(INDEX_ARRAY_ADD, move |e, a| {
            inner.borrow_mut().push(e.mem.u16(a[1]));
            Ret::default()
        });
        record(
            &mut e,
            &log,
            &[INDEX_ARRAY_SET_UP, ORDER_ARRAY_DESTRUCTOR, MEMCPY],
            0,
        );
        record(&mut e, &log, &[NI_NEW], 0x7000);
        record(&mut e, &log, &[TRI_SHAPE_DATA_CONSTRUCTOR], 0x7100);
        record(&mut e, &log, &[TRI_SHAPE_CONSTRUCTOR], 0x7200);
        let node = e.mem.alloc(8);
        record(&mut e, &log, &[NODE_CONSTRUCTOR], node);
        let mut slots = vec![0u32; 0x38];
        slots[0xdc / 4] = 0x00ee_d000;
        e.put_vtable(0x0300_a000, &slots);
        e.mem.set_u32(node, 0x0300_a000);
        record(&mut e, &log, &[0x00ee_d000], 0);
        e.register(ASSIGN_POINTER, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let this = polygon(&mut e, &SQUARE);
        e.call(0x00c4_cf80, &args![this]);
        assert_eq!(*added.borrow(), vec![2, 1, 0, 3, 2, 0]);
        assert_eq!(e.mem.u32(this + TRIANGLE_COUNT), 2);
        assert_eq!(e.mem.u32(this + SHAPE), 0x7200);
        assert_eq!(e.mem.u32(this + NODE), node);
        assert_eq!(calls_to(&log, 0x00ee_d000), vec![vec![node, 0x7200, 1]]);
        assert_eq!(
            calls_to(&log, TRI_SHAPE_CONSTRUCTOR),
            vec![vec![0x7000, 0x7100]]
        );
        for field in [VERTICES, NORMALS, TEXTURE_COORDINATES, COLORS, INDICES] {
            assert_eq!(e.mem.u32(this + field), 0);
        }
        assert_eq!(calls_to(&log, ORDER_ARRAY_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn the_first_point_is_copied_once_before_the_ear_loop() {
        // 00c4d029: the loop (head 00c4d04c) compares with the copy made before it, so a
        // first point that changes while ears are clipped does not end the loop early.
        let mut e = engine();
        let log: Log = Default::default();
        let order_buffer = e.mem.alloc(0x20);
        e.register_double(ORDER_ARRAY_SET_UP, move |e, a| {
            e.mem.set_u32(a[0] + 4, order_buffer);
            e.mem.set_u32(a[0] + 8, 0);
            e.mem.set_u32(a[0] + 0xc, 8);
            Ret::default()
        });
        e.register(ORDER_ARRAY_APPEND, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            let buffer = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(buffer + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        record(
            &mut e,
            &log,
            &[INDEX_ARRAY_SET_UP, ORDER_ARRAY_DESTRUCTOR, MEMCPY],
            0,
        );
        record(&mut e, &log, &[NI_NEW], 0x7000);
        record(&mut e, &log, &[TRI_SHAPE_DATA_CONSTRUCTOR], 0x7100);
        record(&mut e, &log, &[TRI_SHAPE_CONSTRUCTOR], 0x7200);
        let node = e.mem.alloc(8);
        record(&mut e, &log, &[NODE_CONSTRUCTOR], node);
        let mut slots = vec![0u32; 0x38];
        slots[0xdc / 4] = 0x00ee_d000;
        e.put_vtable(0x0300_a000, &slots);
        e.mem.set_u32(node, 0x0300_a000);
        record(&mut e, &log, &[0x00ee_d000], 0);
        e.register(ASSIGN_POINTER, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let this = polygon(
            &mut e,
            &[
                (0.0, 0.0, 5.0),
                (1.0, 0.0, 5.0),
                (2.0, 1.0, 5.0),
                (1.0, 2.0, 5.0),
                (0.0, 1.0, 5.0),
            ],
        );
        let points = e.mem.u32(e.mem.u32(this + POINTS) + 4);
        // After the first ear, point 0 becomes equal to point 3, the next current point.
        let added = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let inner = added.clone();
        e.register_double(INDEX_ARRAY_ADD, move |e, a| {
            inner.borrow_mut().push(e.mem.u16(a[1]));
            if inner.borrow().len() == 3 {
                let words = point_words(e, points + 3 * 0xc);
                for (k, word) in words.iter().enumerate() {
                    e.mem.set_u32(points + 4 * k as u32, *word);
                }
            }
            Ret::default()
        });
        e.call(0x00c4_cf80, &args![this]);
        assert_eq!(*added.borrow(), vec![2, 1, 0, 3, 2, 0, 4, 3, 0]);
        assert_eq!(e.mem.u32(this + TRIANGLE_COUNT), 3);
    }
}
