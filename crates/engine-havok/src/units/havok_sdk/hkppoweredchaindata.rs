//! `Havok SDK/hkppoweredchaindata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use super::hkpsphereshape::{
    load_bits, load_vec, shufps, store_bits, store_vec, zip_with, Vec4, SIGN_MASK,
};

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e3c210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e3_b9a0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e3c210(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e3_b9a0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The vtable the constructors install (`hkpPoweredChainData` by its RTTI).
const POWERED_CHAIN_VTABLE: u32 = 0x010e_b9d4;
/// The base object's vtable the destructor ends with (`hkBaseObject` by its RTTI).
const BASE_VTABLE: u32 = 0x0102_e388;
/// `hkpBridgeConstraintAtom::init` (`00d21ce0`, `this` = the atom, one word).
const ATOM_INIT: u32 = 0x00d2_1ce0;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The grow-by-one reserve (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The reference count increment (`00c90510`, `this` = the object).
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// The reference count decrement (`00c905b0`, `this` = the object).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The query-input header record (`00d6f2b0`, cdecl `(input, output, input[0x19], 8)`).
const HEADER_RECORD: u32 = 0x00d6_f2b0;
/// Two unnamed functions of the output buffer (cdecl, one word `output[0]`): the first returns
/// the cursor the ball-and-socket jacobians are written at, the second (given the link count)
/// the cursor of the motor rows.
const BALL_CURSOR: u32 = 0x00dd_13d0;
const MOTOR_CURSOR: u32 = 0x00dd_13e0;
/// `hkLoadVelocityAccumulators` (`00dcee70`, cdecl, the query input copy).
const LOAD_VELOCITY_ACCUMULATORS: u32 = 0x00dc_ee70;
/// `hkStabilizedBallSocketConstraintBuildJacobian_noSchema` (`00dcfc00`, cdecl: pivot in A's
/// frame, pivot in B's, a `float`, the input copy, the cursor).
const STABILIZED_BALL_SOCKET: u32 = 0x00dc_fc00;
/// `hkQuaternion::set` (`00cb26e0`, `this` = the quaternion, `(rotation)`, `RET 4`).
const QUATERNION_SET: u32 = 0x00cb_26e0;
/// `hkQuaternion::setMul` (`00c66320`, `this` = the product, `(a, b)`, `RET 8`).
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
/// An unnamed function of `hkrotation.cpp` (`00cb2d90`, `this` = three vectors, `(quaternion)`,
/// `RET 4`).
const ROTATION_FROM_QUATERNION: u32 = 0x00cb_2d90;
/// `hk1dAngularVelocityMotorBeginJacobian` (`00d6cd70`, cdecl: axis, input copy, cursor, scratch).
const MOTOR_BEGIN_JACOBIAN: u32 = 0x00d6_cd70;
/// `hkCalcMotorData` (`00dcfff0`, cdecl: motor, scratch, six `float`s out).
const CALC_MOTOR_DATA: u32 = 0x00dc_fff0;
/// The unnamed function that finishes the jacobians (`00df21c0`, cdecl: block, input, output).
const FINISH_JACOBIANS: u32 = 0x00df_21c0;
/// The mask `0` that flips the signs of the first three lanes (`010c4c00`).
const NEGATE_XYZ: u32 = 0x010c_4c00;
/// The zero vector the constructor-like function stores as an entry's rotation (`012683b0`).
const ZERO_VECTOR: u32 = 0x0126_83b0;
/// The `float`s the constructor stores (`01018180`, `010d0c80`, `0101e2bc`).
const DEFAULT_TAU: u32 = 0x0101_8180;
const DEFAULT_DAMPING: u32 = 0x010d_0c80;
const DEFAULT_MAX_ERROR: u32 = 0x0101_e2bc;

const CAPACITY_MASK: u32 = 0x3fff_ffff;
const DONT_DEALLOCATE: u32 = 0x8000_0000;

/// `(a.yzx * b.zxy) - (a.zxy * b.yzx)`, the way the code forms the cross product `a x b`
/// (`SHUFPS` 0xC9 and 0xD2 with the multiplies in this order).
fn cross_yzx(a: Vec4, b: Vec4) -> Vec4 {
    let plus = zip_with(shufps(a, a, 0xc9), shufps(b, b, 0xd2), |x, y| x * y);
    let minus = zip_with(shufps(a, a, 0xd2), shufps(b, b, 0xc9), |x, y| x * y);
    zip_with(plus, minus, |x, y| x - y)
}

/// The memory router's allocator (`router + 0x10`).
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// A 16-byte copy, bit for bit.
fn copy_bits(e: &mut Engine, from: u32, to: u32) {
    let v = load_bits(e, from);
    store_bits(e, to, v);
}

/// `FLD` of the `float` at `from`, `FSTP` to `to`: a copy that quiets a signalling NaN.
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let value = x87_quiet(e.mem.f32(from));
    e.mem.set_f32(to, value);
}

/// A point carried through a matrix of four rows: `(y * row1 + x * row0) + (z * row2 + row3)`.
fn transform_point(e: &Engine, matrix: u32, point: Vec4) -> Vec4 {
    let rows: [Vec4; 4] = std::array::from_fn(|row| load_vec(e, matrix + 0x10 * row as u32));
    let along = |lane: usize, row: Vec4| zip_with([point[lane]; 4], row, |a, b| a * b);
    let first = zip_with(along(1, rows[1]), along(0, rows[0]), |a, b| a + b);
    let second = zip_with(along(2, rows[2]), rows[3], |a, b| a + b);
    zip_with(first, second, |a, b| a + b)
}

// Translated from 00e3b670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPoweredChainData::getType` (named in the engine map): the constraint type, 0x66.
pub fn powered_chain_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0x66
}

// Translated from 00e3b680 (decompiled, FalloutNV.exe 1.4.0.525)
/// An unnamed function of the unit with a register convention of its own (`EAX` = `a`, `EDX` =
/// `out`, the stack word `b`, nothing popped); here the words are `(a, b, out)`. With `A` and `B`
/// the quaternion-like vectors at `a` and `b` it forms `c = (B x A) + A.w * B`, then
/// `out = 2 * (c - B.w * A)` and negates `out` (all four lanes) when the four-lane dot product
/// `B . A` is negative.
pub fn fn_00e3b680(e: &mut Engine, a: u32, b: u32, out: u32) {
    let cross = cross_yzx(load_vec(e, b), load_vec(e, a));
    store_vec(e, out, cross);
    let a_vec = load_vec(e, a);
    let b_vec = load_vec(e, b);
    let with_w = zip_with(
        zip_with([a_vec[3]; 4], b_vec, |x, y| x * y),
        cross,
        |x, y| x + y,
    );
    store_vec(e, out, with_w);
    let b_vec = load_vec(e, b);
    let a_vec = load_vec(e, a);
    let scaled = zip_with([b_vec[3]; 4], a_vec, |x, y| x * y);
    let diff = zip_with(with_w, scaled, |x, y| x - y);
    let doubled = zip_with(diff, diff, |x, y| x + y);
    store_vec(e, out, doubled);
    let a_vec = load_vec(e, a);
    let b_vec = load_vec(e, b);
    let product = zip_with(b_vec, a_vec, |x, y| x * y);
    let pairs = zip_with(shufps(product, product, 0x4e), product, |x, y| x + y);
    let dot = zip_with(shufps(pairs, pairs, 0xb1), pairs, |x, y| x + y)[0];
    if dot.is_sign_negative() {
        let mask = load_bits(e, SIGN_MASK);
        let flipped = std::array::from_fn(|lane| doubled[lane].to_bits() ^ mask[lane]);
        store_bits(e, out, flipped);
    }
}

// Translated from 00e3b720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPoweredChainData::getConstraintInfo` (named in the engine map): `info` gets the atoms
/// (`this + 0xc`, `0xc` bytes of them), a size word of `((n * 0x534 + 0x53) & !0xf) + 0x10` for
/// the link count `n` at `this + 0x1c`, and `6 * n` in its two count words.
pub fn powered_chain_get_constraint_info(e: &mut Engine, this: Ptr, info: u32) {
    e.mem.set_u32(info + 0x10, this.addr() + 0xc);
    e.mem.set_u32(info + 0x8, 0);
    e.mem.set_u32(info + 0xc, 0);
    e.mem.set_u32(info, 0);
    e.mem.set_u32(info + 0x4, 0x10);
    e.mem.set_u32(info + 0x14, 0xc);
    let links = e.mem.u32(this.addr() + 0x1c);
    let counts = links.wrapping_mul(3).wrapping_mul(2);
    let size = (links.wrapping_mul(0x534).wrapping_add(0x53) & !0xf).wrapping_add(0x10);
    e.mem.set_u32(info + 0x4, size);
    e.mem.set_u32(info + 0x8, counts);
    e.mem.set_u32(info + 0xc, counts);
}

// Translated from 00e3b770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPoweredChainData::getRuntimeInfo` (named in the engine map): with the link count `n` at
/// `this + 0x1c`, `info[1] = 6 * n` and `info[0] = ((n + 3) & !3) + 64 * n`.
pub fn powered_chain_get_runtime_info(e: &mut Engine, this: Ptr, _unused_1: u32, info: u32) {
    let links = e.mem.u32(this.addr() + 0x1c);
    let counts = links.wrapping_mul(3).wrapping_mul(2);
    e.mem.set_u32(info + 4, counts);
    let size = (links.wrapping_add(3) & !3)
        .wrapping_add(counts.wrapping_add(links.wrapping_mul(2)).wrapping_mul(8));
    e.mem.set_u32(info, size);
}

// Translated from 00e3b850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a link to the array at `this + 0x18` (an unnamed function of the unit; the elements
/// are `0x50` bytes). The array grows by one element through `00c90940` when it is full. The
/// element gets the vectors at `pivot_a`, `pivot_b` and `target` (at `+0`, `+0x10`, `+0x20`),
/// the three motors (at `+0x40`, `+0x44`, `+0x48`, each with a reference added), a zero byte at
/// `+0x4c` and the zero vector (`012683b0`) at `+0x30`.
pub fn fn_00e3b850(
    e: &mut Engine,
    this: Ptr,
    pivot_a: u32,
    pivot_b: u32,
    target: u32,
    motor_x: u32,
    motor_y: u32,
    motor_z: u32,
) {
    let array = this.addr() + 0x18;
    let allocator = allocator(e);
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 0x50u32]);
    }
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    let element = size.wrapping_mul(0x50).wrapping_add(e.mem.u32(array));
    copy_bits(e, pivot_a, element);
    copy_bits(e, pivot_b, element + 0x10);
    copy_bits(e, target, element + 0x20);
    e.mem.set_u32(element + 0x40, motor_x);
    e.call(ADD_REFERENCE, &args![motor_x]);
    e.mem.set_u32(element + 0x44, motor_y);
    e.call(ADD_REFERENCE, &args![motor_y]);
    e.mem.set_u32(element + 0x48, motor_z);
    e.call(ADD_REFERENCE, &args![motor_z]);
    e.mem.set_u8(element + 0x4c, 0);
    copy_bits(e, ZERO_VECTOR, element + 0x30);
}

// Translated from 00e3b8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPoweredChainData::hkpPoweredChainData` (named in the engine map): reference count 1 (at
/// `+6`), the vtable, an atom word of 1 at `+0xc`, an empty link array (`+0x18`, size `+0x1c`,
/// capacity word `0x80000000` at `+0x20`), the `float` defaults at `+0x24` to `+0x3c` and the
/// atom initialised by `00d21ce0`. Returns `this`.
pub fn powered_chain_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let data = this.addr();
    e.mem.set_u16(data + 6, 1);
    e.mem.set_u32(data + 8, 0);
    e.mem.set_u32(data, POWERED_CHAIN_VTABLE);
    e.mem.set_u16(data + 0xc, 1);
    e.mem.set_u32(data + 0x18, 0);
    e.mem.set_u32(data + 0x1c, 0);
    e.mem.set_u32(data + 0x20, DONT_DEALLOCATE);
    copy_float(e, DEFAULT_TAU, data + 0x24);
    e.mem.set_f32(data + 0x28, 1.0);
    copy_float(e, DEFAULT_DAMPING, data + 0x2c);
    copy_float(e, DEFAULT_DAMPING, data + 0x34);
    e.mem.set_f32(data + 0x30, 1.0);
    e.mem.set_f32(data + 0x38, 1.0);
    copy_float(e, DEFAULT_MAX_ERROR, data + 0x3c);
    e.call(ATOM_INIT, &args![data + 0xc, data]);
    this
}

// Translated from 00e3b960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor taking one flag word (an unnamed function of the unit, the shape of the
/// "finish loading" constructors): installs the vtable and, when `flag` is non-zero, calls the
/// atom initialiser on the atom at `this + 0xc` with the atom's own word at `+0x14` and then,
/// the flag being tested a second time as the code does, with `this`. Returns `this`.
pub fn fn_00e3b960(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let data = this.addr();
    e.mem.set_u32(data, POWERED_CHAIN_VTABLE);
    let atom = data + 0xc;
    if flag != 0 {
        let word = e.mem.u32(atom + 8);
        e.call(ATOM_INIT, &args![atom, word]);
        if flag != 0 {
            e.call(ATOM_INIT, &args![atom, data]);
        }
    }
    this
}

// Translated from 00e3b9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body (an unnamed function of the unit): installs the vtable, removes the
/// reference from each of the three motors (`+0x40`, `+0x44`, `+0x48`) of every `0x50`-byte link
/// that has one, empties the link array and returns its storage to the allocator (slot `0x10`,
/// data and byte size) unless it is not the array's own, then installs the base vtable.
pub fn fn_00e3b9a0(e: &mut Engine, this: Ptr) {
    let data = this.addr();
    e.mem.set_u32(data, POWERED_CHAIN_VTABLE);
    let mut link: i32 = 0;
    while link < e.mem.i32(data + 0x1c) {
        for motor in 0..3u32 {
            let slot = e
                .mem
                .u32(data + 0x18)
                .wrapping_add((link as u32).wrapping_mul(0x50))
                .wrapping_add(0x40 + 4 * motor);
            let object = e.mem.u32(slot);
            if object != 0 {
                e.call(REMOVE_REFERENCE, &args![object]);
            }
        }
        link += 1;
    }
    let allocator = allocator(e);
    let flags = e.mem.u32(data + 0x20);
    e.mem.set_u32(data + 0x1c, 0);
    if flags & DONT_DEALLOCATE == 0 {
        let array = e.mem.u32(data + 0x18);
        let bytes = (flags & CAPACITY_MASK).wrapping_mul(0x50);
        e.vcall(allocator, 0x10, &args![array, bytes]);
    }
    e.mem.set_u32(data + 0x18, 0);
    e.mem.set_u32(data + 0x20, DONT_DEALLOCATE);
    e.mem.set_u32(data, BASE_VTABLE);
}

// The frame of `buildJacobian` (the stack of the original, with the offsets it uses from the
// first byte above the `0xaa4` bytes it reserves; the vectors are 16-byte aligned there).
/// The `0x20` words copy of the query input, which the jacobian builders are handed.
const FRAME_INPUT_COPY: u32 = 0x134;
/// The squared length of the persisted rotation, stored and compared as a `float`.
const FRAME_LENGTH_SQUARED: u32 = 0x50;
/// The quaternions of the link being built: the one for body A and the one for body B.
const FRAME_QUATERNION_A: u32 = 0x34;
const FRAME_QUATERNION_B: u32 = 0x54;
/// The scratch the quaternion swap goes through.
const FRAME_SWAP: u32 = 0x84;
/// The two angular differences (the second is negated together with the first).
const FRAME_DIFFERENCE: u32 = 0x94;
const FRAME_DIFFERENCE_B: u32 = 0x114;
/// The motor query scratch handed to `hk1dAngularVelocityMotorBeginJacobian` and
/// `hkCalcMotorData`: a word written by the first, the input copy pointer, two words from the
/// motor table and the two differences.
const FRAME_MOTOR_QUERY: u32 = 0x6c;
const FRAME_MOTOR_TABLE: u32 = 0x68;
/// The copy of the rotation row handed to `BeginJacobian`, and the three rows of the rotation.
const FRAME_ROW_COPY: u32 = 0xa4;
const FRAME_ROTATION: u32 = 0xb4;
/// The six `float`s `hkCalcMotorData` writes.
const FRAME_MOTOR_OUT: u32 = 0xfc;
/// The vector of the persisted rotation (`0x124`) the products are formed with.
const FRAME_PERSISTED: u32 = 0x124;
/// The products `setMul` writes.
const FRAME_PRODUCT_A: u32 = 0x244;
const FRAME_PRODUCT_B: u32 = 0x254;
const FRAME_PRODUCT_Q: u32 = 0x264;
/// The pivots in the frames of the bodies.
const FRAME_PIVOT_A: u32 = 0x274;
const FRAME_PIVOT_B: u32 = 0x284;
/// `hkArray`s (data, size, capacity word) and their inline storage: the index list (capacity
/// `0x20` words) and the row buffer (capacity `0x20` rows of `0x40` bytes).
const FRAME_INDEX_ARRAY: u32 = 0x1b4;
const FRAME_INDEX_STORAGE: u32 = 0x1c0;
const FRAME_ROW_ARRAY: u32 = 0x294;
const FRAME_ROW_STORAGE: u32 = 0x2a0;
const FRAME_SIZE: u32 = 0xb00;

/// Appends `word` to the index list, growing it through `00c90940` when it is full.
fn push_index(e: &mut Engine, array: u32, word: u32) {
    let allocator = allocator(e);
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(4));
    if slot != 0 {
        e.mem.set_u32(slot, word);
    }
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Gives back an array's storage the way the code ends: size 0, then the allocator's slot
/// `0x10` (data, byte size) unless the storage is not the array's own.
fn release_array(e: &mut Engine, array: u32, element_size: u32) {
    let allocator = allocator(e);
    let flags = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if flags & DONT_DEALLOCATE == 0 {
        let data = e.mem.u32(array);
        let bytes = (flags & CAPACITY_MASK).wrapping_mul(element_size);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

// Translated from 00e3ba40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPoweredChainData::buildJacobian` (named in the engine map). `input` is the query input
/// (a link description at `+0x60` whose body list is at `+0x38`, its count at `+0x3c` and
/// first body at `+0x14`, a word at `+0x30`, a persistent buffer at `+0x64`) and `output` the
/// query output. The function writes the header record (8), copies the input into its frame and
/// builds, for each of the `n = count - 1` links, the stabilised ball-and-socket jacobian of the
/// pivots carried into the frames of the two bodies, the persisted rotation difference and three
/// angular motor rows (one per axis, through the motor virtual `+0x28` and `hkCalcMotorData`),
/// then hands the block it filled to `00df21c0`. The names of the callees outside the engine map
/// and of the link fields are unconfirmed.
pub fn powered_chain_build_jacobian(e: &mut Engine, this: Ptr, input: u32, output: u32) {
    let data = this.addr();
    let record = e.mem.u32(input + 0x64);
    e.call(HEADER_RECORD, &args![input, output, record, 8u32]);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let m = frame.addr();
        let copy = m + FRAME_INPUT_COPY;
        for word in 0..0x20 {
            let value = e.mem.u32(input + 4 * word);
            e.mem.set_u32(copy + 4 * word, value);
        }
        let chain = e.mem.u32(input + 0x60);
        let index_array = m + FRAME_INDEX_ARRAY;
        let row_array = m + FRAME_ROW_ARRAY;
        e.mem.set_u32(copy + 0x60, 0);
        e.mem.set_u32(copy + 0x64, 0);
        e.mem.set_u32(index_array, m + FRAME_INDEX_STORAGE);
        e.mem.set_u32(index_array + 4, 0);
        e.mem.set_u32(index_array + 8, 0x8000_0020);
        let first_body = e.mem.u32(chain + 0x14);
        let base = e
            .mem
            .u32(input + 0x30)
            .wrapping_sub(e.mem.u32(first_body + 0xa4));
        let links = e.mem.i32(chain + 0x3c).wrapping_sub(1);
        e.mem.set_u32(row_array, m + FRAME_ROW_STORAGE);
        e.mem.set_u32(row_array + 4, 0);
        e.mem.set_u32(row_array + 8, 0x8000_0020);
        let allocator = allocator(e);
        let capacity = (e.mem.u32(row_array + 8) & CAPACITY_MASK) as i32;
        if capacity < links {
            let doubled = capacity.wrapping_mul(2);
            let wanted = if links < doubled { doubled } else { links };
            e.call(
                ARRAY_RESERVE,
                &args![allocator, row_array, wanted as u32, 0x40u32],
            );
        }
        e.mem.set_i32(row_array + 4, links);
        let bodies = e.mem.u32(chain + 0x38);
        let body = e.mem.u32(bodies);
        let body_word = e.mem.u32(body + 0xa4);
        e.mem.set_u32(copy + 0x34, body_word.wrapping_add(base));
        let slot = e
            .mem
            .u32(index_array)
            .wrapping_add(e.mem.u32(index_array + 4).wrapping_mul(4));
        if slot != 0 {
            e.mem.set_u32(slot, body_word);
        }
        let size = e.mem.u32(index_array + 4);
        e.mem.set_u32(index_array + 4, size.wrapping_add(1));
        e.mem.set_f32(copy + 0x24, 1.0);
        e.mem.set_u32(copy + 0x3c, body.wrapping_add(0xf0));
        let word = e.mem.u32(copy + 8);
        e.mem.set_u32(copy + 0x20, word);
        let mut ball_cursor = e.call(BALL_CURSOR, &args![e.mem.u32(output)]).u32();
        let mut motor_cursor = e
            .call(MOTOR_CURSOR, &args![e.mem.u32(output), links as u32])
            .u32();
        let mut link_offset = 0u32;
        let mut pair_start = 0x18u32;
        for link in 0..links.max(0) as u32 {
            e.mem.set_u32(copy + 0x30, e.mem.u32(copy + 0x34));
            e.mem.set_u32(copy + 0x38, e.mem.u32(copy + 0x3c));
            let next_body = e.mem.u32(e.mem.u32(chain + 0x38) + 4 + 4 * link);
            let next_word = e.mem.u32(next_body + 0xa4);
            e.mem.set_u32(copy + 0x34, next_word.wrapping_add(base));
            push_index(e, index_array, next_word);
            e.mem.set_u32(copy + 0x3c, next_body.wrapping_add(0xf0));
            let entry = e.mem.u32(data + 0x18).wrapping_add(link_offset);
            let previous_matrix = e.mem.u32(copy + 0x38);
            let this_matrix = e.mem.u32(copy + 0x3c);
            let pivot_a = transform_point(e, previous_matrix, load_vec(e, entry));
            store_vec(e, m + FRAME_PIVOT_A, pivot_a);
            let pivot_b = transform_point(e, this_matrix, load_vec(e, entry + 0x10));
            store_vec(e, m + FRAME_PIVOT_B, pivot_b);
            e.call(LOAD_VELOCITY_ACCUMULATORS, &args![copy]);
            let damping = x87_quiet(e.mem.f32(data + 0x3c));
            e.call(
                STABILIZED_BALL_SOCKET,
                &args![
                    m + FRAME_PIVOT_A,
                    m + FRAME_PIVOT_B,
                    damping,
                    copy,
                    ball_cursor
                ],
            );
            ball_cursor = ball_cursor.wrapping_add(0x90);
            let (quaternion_a, quaternion_b) = (m + FRAME_QUATERNION_A, m + FRAME_QUATERNION_B);
            e.call(QUATERNION_SET, &args![quaternion_a, e.mem.u32(copy + 0x38)]);
            e.call(QUATERNION_SET, &args![quaternion_b, e.mem.u32(copy + 0x3c)]);
            if e.mem.u8(entry + 0x4c) != 0 {
                copy_bits(e, quaternion_a, m + FRAME_SWAP);
                copy_bits(e, quaternion_b, quaternion_a);
                copy_bits(e, m + FRAME_SWAP, quaternion_b);
            }
            e.call(
                QUATERNION_SET_MUL,
                &args![m + FRAME_PRODUCT_Q, quaternion_b, entry + 0x30],
            );
            copy_bits(e, m + FRAME_PRODUCT_Q, quaternion_b);
            let cells = e.mem.u32(input + 0x64);
            let count = e.mem.u32(data + 0x1c);
            let cell = link
                .wrapping_add(count.wrapping_mul(3))
                .wrapping_mul(16)
                .wrapping_add(count.wrapping_add(3) & !3)
                .wrapping_add(cells);
            let persisted = m + FRAME_PERSISTED;
            copy_bits(e, cell, persisted);
            let squares = load_vec(e, persisted);
            let squares = zip_with(squares, squares, |a, b| a * b);
            let pairs = zip_with(shufps(squares, squares, 0x4e), squares, |a, b| a + b);
            let length = zip_with(shufps(pairs, pairs, 0xb1), pairs, |a, b| a + b)[0];
            e.mem.set_f32(m + FRAME_LENGTH_SQUARED, length);
            if e.mem.f32(m + FRAME_LENGTH_SQUARED) == 0.0 {
                copy_bits(e, entry + 0x20, persisted);
            }
            e.call(
                QUATERNION_SET_MUL,
                &args![m + FRAME_PRODUCT_A, quaternion_a, entry + 0x20],
            );
            e.call(
                QUATERNION_SET_MUL,
                &args![m + FRAME_PRODUCT_B, quaternion_a, persisted],
            );
            for lane in 0..4 {
                copy_float(e, entry + 0x20 + 4 * lane, cell + 4 * lane);
            }
            let (difference, difference_b) = (m + FRAME_DIFFERENCE, m + FRAME_DIFFERENCE_B);
            fn_00e3b680(e, m + FRAME_PRODUCT_A, quaternion_b, difference);
            fn_00e3b680(e, m + FRAME_PRODUCT_B, quaternion_b, difference_b);
            let first = zip_with(
                load_vec(e, difference),
                load_vec(e, difference_b),
                |a, b| a - b,
            );
            store_vec(e, difference, first);
            if e.mem.u8(entry + 0x4c) != 0 {
                let mask = load_bits(e, NEGATE_XYZ);
                let flip = |v: Vec4| -> [u32; 4] {
                    std::array::from_fn(|lane| v[lane].to_bits() ^ mask[lane])
                };
                let second = load_vec(e, difference_b);
                store_bits(e, difference, flip(first));
                store_bits(e, difference_b, flip(second));
            }
            let rotation = m + FRAME_ROTATION;
            e.call(ROTATION_FROM_QUATERNION, &args![rotation, quaternion_b]);
            let mut row_bytes = link << 6;
            let mut motor_slot = link_offset.wrapping_add(0x40);
            let mut pair = pair_start;
            for axis in 0..3u32 {
                copy_bits(e, rotation + 0x10 * axis, m + FRAME_ROW_COPY);
                let argument = e.mem.u32(e.mem.u32(chain + 0x30) + 0x1c);
                let table = e.vcall(data, 0x28, &args![argument]).u32();
                e.mem.set_u32(m + FRAME_MOTOR_TABLE, table);
                let query = m + FRAME_MOTOR_QUERY;
                e.mem.set_f32(query + 8, 0.0);
                e.mem.set_f32(query + 0xc, 0.0);
                e.call(
                    MOTOR_BEGIN_JACOBIAN,
                    &args![m + FRAME_ROW_COPY, copy, motor_cursor, query],
                );
                copy_float(e, difference + 4 * axis, query + 0x10);
                copy_float(e, difference_b + 4 * axis, query + 0x14);
                e.mem.set_u32(query + 4, copy);
                let low = e.mem.u32(pair.wrapping_add(table));
                let high = e.mem.u32(pair.wrapping_add(table).wrapping_add(4));
                e.mem.set_u32(query + 8, low);
                e.mem.set_u32(query + 0xc, high);
                let motor = e.mem.u32(e.mem.u32(data + 0x18).wrapping_add(motor_slot));
                e.call(CALC_MOTOR_DATA, &args![motor, query, m + FRAME_MOTOR_OUT]);
                let out: [f32; 6] =
                    std::array::from_fn(|i| e.mem.f32(m + FRAME_MOTOR_OUT + 4 * i as u32));
                let row = e.mem.u32(row_array).wrapping_add(row_bytes).wrapping_add(4);
                let first_out = e.mem.f32(copy + 4) * out[2];
                e.mem.set_f32(row, first_out);
                pair = pair.wrapping_add(8);
                motor_slot = motor_slot.wrapping_add(4);
                let second_out = out[3] * e.mem.f32(copy + 4);
                row_bytes = row_bytes.wrapping_add(0x14);
                e.mem.set_f32(row + 4, second_out);
                e.mem.set_f32(row + 8, x87_quiet(out[1]));
                e.mem.set_f32(row + 0xc, x87_quiet(out[4]));
                e.mem.set_f32(row + 0x10, x87_quiet(out[5]));
                motor_cursor = motor_cursor.wrapping_add(0x20);
                let last = e.mem.f32(copy + 8) * out[0];
                e.mem.set_f32(motor_cursor.wrapping_sub(4), last);
            }
            link_offset = link_offset.wrapping_add(0x50);
            pair_start = pair_start.wrapping_add(0x30);
        }
        let flags_base = e
            .mem
            .u32(e.mem.u32(chain + 0x30) + 0x1c)
            .wrapping_add(e.mem.u32(data + 0x1c).wrapping_mul(0x30));
        for link in 0..links.max(0) as u32 {
            let flag = e.mem.u8(flags_base.wrapping_add(link));
            let slot = e.mem.u32(row_array).wrapping_add(link.wrapping_mul(0x40));
            e.mem.set_u8(slot, flag);
        }
        let block = m + FRAME_ROTATION;
        e.mem.set_u32(block, links as u32);
        for word in 0..6 {
            copy_float(e, data + 0x24 + 4 * word, block + 4 + 4 * word);
        }
        let indices = e.mem.u32(index_array);
        e.mem.set_u32(block + 0x1c, indices);
        e.mem.set_u32(block + 0x20, base);
        let rows = e.mem.u32(row_array);
        e.mem.set_u32(block + 0x24, rows);
        e.mem.set_f32(block + 0x28, 0.0);
        e.mem.set_u32(block + 0x2c, flags_base);
        e.mem.set_u32(block + 0x30, motor_cursor);
        e.call(FINISH_JACOBIANS, &args![block, input, output]);
        release_array(e, row_array, 0x40);
        release_array(e, index_array, 4);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_b670, powered_chain_get_type(Ptr) -> u32),
        entry!(0x00e3_b680, fn_00e3b680(u32, u32, u32)),
        entry!(0x00e3_b720, powered_chain_get_constraint_info(Ptr, u32)),
        entry!(0x00e3_b770, powered_chain_get_runtime_info(Ptr, u32, u32)),
        entry!(0x00e3_b850, fn_00e3b850(Ptr, u32, u32, u32, u32, u32, u32)),
        entry!(0x00e3_b8f0, powered_chain_construct(Ptr) -> Ptr),
        entry!(0x00e3_b960, fn_00e3b960(Ptr, u32) -> Ptr),
        entry!(0x00e3_b9a0, fn_00e3b9a0(Ptr)),
        entry!(0x00e3_ba40, powered_chain_build_jacobian(Ptr, u32, u32)),
        entry!(0x00e3_c210, deleting_destructor_00e3c210(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00e3c210_frees_only_when_bit_zero_is_set() {
        check(0x00e3_c210, 0x00e3_b9a0);
    }
}

#[cfg(test)]
mod a05_powered_chain_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0210;
    const MOTOR_TABLE_DOUBLE: u32 = 0x0900_0228;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    fn world() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_8000,
            0x010d_0000,
            0x0101_e000,
            0x0126_8000,
            0x010c_4000,
        ] {
            e.map(page, 0x1000);
        }
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        for lane in 0..3 {
            e.mem.set_u32(NEGATE_XYZ + 4 * lane, 0x8000_0000);
        }
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A memory router whose allocator frees through a double that logs `(data, bytes)`;
    /// returns the allocator and the log.
    fn install_router(e: &mut Engine) -> (u32, Log) {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let frees = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            frees.borrow_mut().push(vec![a[1], a[2]]);
            Ret::default()
        });
        (allocator, log)
    }

    #[test]
    fn type_constraint_info_and_runtime_info() {
        let mut e = world();
        let this = e.mem.alloc(0x60);
        assert_eq!(e.call(0x00e3_b670, &args![this]).u32(), 0x66);
        e.mem.set_u32(this + 0x1c, 2);
        let info = e.mem.alloc(0x20);
        e.call(0x00e3_b720, &args![this, info]);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(info + 4 * i)).collect();
        assert_eq!(words, vec![0, 0xac0, 12, 12, this + 0xc, 0xc]);
        let info = e.mem.alloc(8);
        e.call(0x00e3_b770, &args![this, 0xdeadu32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (132, 12));
        e.mem.set_u32(this + 0x1c, 0);
        e.call(0x00e3_b770, &args![this, 0u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0, 0));
    }

    #[test]
    fn quaternion_difference_flips_when_the_dot_product_is_negative() {
        let mut e = world();
        let (a, b, out) = (e.mem.alloc(0x10), e.mem.alloc(0x10), e.mem.alloc(0x10));
        store_vec(&mut e, a, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, b, [5.0, 6.0, 7.0, 8.0]);
        e.call(0x00e3_b680, &args![a, b, out]);
        assert_eq!(load_vec(&e, out), [32.0, 0.0, 16.0, 0.0]);
        store_vec(&mut e, a, [-1.0, -2.0, -3.0, -4.0]);
        e.call(0x00e3_b680, &args![a, b, out]);
        assert_eq!(load_vec(&e, out), [32.0, 0.0, 16.0, 0.0]);
        // The zero lanes come out as negative zeros: the sign mask is applied to all four.
        assert_eq!(e.mem.u32(out + 4), 0x8000_0000);
        assert_eq!(e.mem.u32(out + 12), 0x8000_0000);
        // The inputs are untouched.
        assert_eq!(load_vec(&e, b), [5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn adding_a_link_grows_the_array_only_when_it_is_full() {
        let mut e = world();
        e.register_double(ARRAY_RESERVE_MORE, |e, a| {
            let storage = e.mem.alloc(0x50 * 4);
            e.mem.set_u32(a[1], storage);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        e.register_double(ADD_REFERENCE, |_, _| Ret::default());
        let (allocator, _) = install_router(&mut e);
        for (lane, value) in [7.0f32, 8.0, 9.0, 10.0].into_iter().enumerate() {
            e.mem.set_f32(ZERO_VECTOR + 4 * lane as u32, value);
        }
        let this = e.mem.alloc(0x40);
        let (a, b, c) = (e.mem.alloc(0x10), e.mem.alloc(0x10), e.mem.alloc(0x10));
        store_vec(&mut e, a, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, b, [5.0, 6.0, 7.0, 8.0]);
        store_vec(&mut e, c, [9.0, 10.0, 11.0, 12.0]);
        e.call(
            0x00e3_b850,
            &args![this, a, b, c, 0xa1u32, 0xa2u32, 0xa3u32],
        );
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE_MORE),
            vec![vec![allocator, this + 0x18, 0x50]]
        );
        assert_eq!(e.mem.u32(this + 0x1c), 1);
        let first = e.mem.u32(this + 0x18);
        e.mem.set_u8(first + 0x50 + 0x4c, 0xff);
        assert_eq!(load_vec(&e, first), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, first + 0x10), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(load_vec(&e, first + 0x20), [9.0, 10.0, 11.0, 12.0]);
        assert_eq!(load_vec(&e, first + 0x30), [7.0, 8.0, 9.0, 10.0]);
        let motors: Vec<u32> = (0..3).map(|i| e.mem.u32(first + 0x40 + 4 * i)).collect();
        assert_eq!(motors, vec![0xa1, 0xa2, 0xa3]);
        assert_eq!(e.mem.u8(first + 0x4c), 0);
        e.call(
            0x00e3_b850,
            &args![this, c, b, a, 0xb1u32, 0xb2u32, 0xb3u32],
        );
        assert_eq!(calls_to(&e, ARRAY_RESERVE_MORE).len(), 1);
        assert_eq!(e.mem.u32(this + 0x1c), 2);
        assert_eq!(load_vec(&e, first + 0x50), [9.0, 10.0, 11.0, 12.0]);
        assert_eq!(e.mem.u8(first + 0x50 + 0x4c), 0);
        let references: Vec<u32> = calls_to(&e, ADD_REFERENCE).iter().map(|c| c[0]).collect();
        assert_eq!(references, vec![0xa1, 0xa2, 0xa3, 0xb1, 0xb2, 0xb3]);
    }

    #[test]
    fn construct_sets_the_defaults_and_initialises_the_atom() {
        let mut e = world();
        e.register_double(ATOM_INIT, |_, _| Ret::default());
        e.mem.set_f32(DEFAULT_TAU, 0.75);
        e.mem.set_f32(DEFAULT_DAMPING, 0.125);
        e.mem.set_f32(DEFAULT_MAX_ERROR, 0.25);
        let this = e.mem.alloc(0x50);
        for word in [8, 0x18, 0x1c] {
            e.mem.set_u32(this + word, 0xdead_beef);
        }
        assert_eq!(e.call(0x00e3_b8f0, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this), POWERED_CHAIN_VTABLE);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0x8000_0000);
        let floats: Vec<f32> = (0..7).map(|i| e.mem.f32(this + 0x24 + 4 * i)).collect();
        assert_eq!(floats, vec![0.75, 1.0, 0.125, 1.0, 0.125, 1.0, 0.25]);
        assert_eq!(calls_to(&e, ATOM_INIT), vec![vec![this + 0xc, this]]);
    }

    #[test]
    fn construct_quiets_a_signalling_default() {
        let mut e = world();
        e.register_double(ATOM_INIT, |_, _| Ret::default());
        e.mem.set_u32(DEFAULT_TAU, 0x7fa0_0000);
        let this = e.mem.alloc(0x50);
        e.call(0x00e3_b8f0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x24), 0x7fe0_0000);
    }

    #[test]
    fn flag_constructor_initialises_the_atom_twice_only_when_the_flag_is_set() {
        let mut e = world();
        e.register_double(ATOM_INIT, |_, _| Ret::default());
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0xc + 8, 0x1234);
        assert_eq!(e.call(0x00e3_b960, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), POWERED_CHAIN_VTABLE);
        assert!(calls_to(&e, ATOM_INIT).is_empty());
        assert_eq!(e.call(0x00e3_b960, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls_to(&e, ATOM_INIT),
            vec![vec![this + 0xc, 0x1234], vec![this + 0xc, this]]
        );
    }

    #[test]
    fn destructor_releases_every_motor_and_frees_the_array() {
        let mut e = world();
        e.register_double(REMOVE_REFERENCE, |_, _| Ret::default());
        let (_, frees) = install_router(&mut e);
        let this = e.mem.alloc(0x40);
        let data = e.mem.alloc(0x50 * 3);
        e.mem.set_u32(this + 0x18, data);
        e.mem.set_u32(this + 0x1c, 2);
        e.mem.set_u32(this + 0x20, 3);
        for (slot, motor) in [
            (0x40, 0xa1),
            (0x48, 0xa3),
            (0x90, 0xb1),
            (0x94, 0xb2),
            (0x98, 0xb3),
        ] {
            e.mem.set_u32(data + slot, motor);
        }
        e.call(0x00e3_b9a0, &args![this]);
        let released: Vec<u32> = calls_to(&e, REMOVE_REFERENCE)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(released, vec![0xa1, 0xa3, 0xb1, 0xb2, 0xb3]);
        assert_eq!(*frees.borrow(), vec![vec![data, 3 * 0x50]]);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn destructor_keeps_storage_that_is_not_its_own() {
        let mut e = world();
        e.register_double(REMOVE_REFERENCE, |_, _| Ret::default());
        let (_, frees) = install_router(&mut e);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 0x1000);
        e.mem.set_u32(this + 0x1c, 0);
        e.mem.set_u32(this + 0x20, 0x8000_0002);
        e.call(0x00e3_b9a0, &args![this]);
        assert!(calls_to(&e, REMOVE_REFERENCE).is_empty());
        assert!(frees.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    type BallCall = (Vec<u32>, Vec4, Vec4, Vec<u32>);

    struct FinishCall {
        block: Vec<u32>,
        indices: Vec<u32>,
        flag_bytes: Vec<u8>,
    }

    /// What the doubles of `buildJacobian` saw, read at the time of each call (the frame is
    /// gone afterwards).
    #[derive(Default)]
    struct Seen {
        ball: Vec<BallCall>,
        products: Vec<Vec4>,
        begin: Vec<(u32, Vec4, u32)>,
        calc: Vec<(u32, Vec<u32>)>,
        finish: Vec<FinishCall>,
        table_arguments: Vec<u32>,
    }

    struct Setup {
        e: Engine,
        seen: Rc<RefCell<Seen>>,
        allocator: u32,
        frees: Log,
        this: u32,
        input: u32,
        output: u32,
        cursors: u32,
        cells: u32,
        flags: u32,
        bodies: Vec<u32>,
    }

    /// Replaces the storage of the array at `array` by a fresh one of `capacity` elements of
    /// `element` bytes, keeping the elements it holds.
    fn grow(e: &mut Engine, array: u32, element: u32, capacity: u32) {
        let storage = e.mem.alloc(capacity * element);
        let old = e.mem.u32(array);
        for word in 0..e.mem.u32(array + 4) * element / 4 {
            let value = e.mem.u32(old + 4 * word);
            e.mem.set_u32(storage + 4 * word, value);
        }
        e.mem.set_u32(array, storage);
        e.mem.set_u32(array + 8, capacity);
    }

    /// A chain of `count` bodies (so `count - 1` links, one `0x50`-byte entry each) with the
    /// swap flag `flag` on every entry and `persisted` as the stored rotation of the first
    /// link. Body `i`'s word at `+0xa4` is `100 + i`; the first body's matrix translates by
    /// `(10, 20, 30)`, the others' by `(100, 200, 300)`.
    fn setup(count: u32, flag: u8, persisted: Vec4) -> Setup {
        let links = count.saturating_sub(1);
        let mut e = world();
        let (allocator, frees) = install_router(&mut e);
        let seen = Rc::new(RefCell::new(Seen::default()));
        let bodies: Vec<u32> = (0..count).map(|_| e.mem.alloc(0x110)).collect();
        let matrix_a = bodies[0] + 0xf0;
        for address in [HEADER_RECORD, LOAD_VELOCITY_ACCUMULATORS] {
            e.register_double(address, |_, _| Ret::default());
        }
        e.register_double(BALL_CURSOR, |_, _| Ret {
            eax: 0x1000,
            ..Ret::default()
        });
        let cursors = e.mem.alloc(0x60 * links + 0x100);
        e.register_double(MOTOR_CURSOR, move |_, _| Ret {
            eax: cursors,
            ..Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(STABILIZED_BALL_SOCKET, move |e, a| {
            let copy: Vec<u32> = (0..0x20).map(|i| e.mem.u32(a[3] + 4 * i)).collect();
            let call = (a.to_vec(), load_vec(e, a[0]), load_vec(e, a[1]), copy);
            log.borrow_mut().ball.push(call);
            Ret::default()
        });
        e.register_double(QUATERNION_SET, move |e, a| {
            // The first body's matrix gives (0, 0, 0, 1), any other (0, 1, 0, 0).
            let q = if a[1] == matrix_a {
                [0.0, 0.0, 0.0, 1.0]
            } else {
                [0.0, 1.0, 0.0, 0.0]
            };
            store_vec(e, a[0], q);
            Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(QUATERNION_SET_MUL, move |e, a| {
            // Logs the first operand and makes the product the second.
            log.borrow_mut().products.push(load_vec(e, a[1]));
            let second = load_vec(e, a[2]);
            store_vec(e, a[0], second);
            Ret::default()
        });
        e.register_double(ROTATION_FROM_QUATERNION, |e, a| {
            store_vec(e, a[0], [1.0, 0.0, 0.0, 0.0]);
            store_vec(e, a[0] + 0x10, [0.0, 1.0, 0.0, 0.0]);
            store_vec(e, a[0] + 0x20, [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(MOTOR_BEGIN_JACOBIAN, move |e, a| {
            let call = (a[2], load_vec(e, a[0]), a[1]);
            log.borrow_mut().begin.push(call);
            Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(CALC_MOTOR_DATA, move |e, a| {
            let query: Vec<u32> = (1..6).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut().calc.push((a[0], query));
            for (i, value) in [0.5f32, 1.5, 2.5, 3.5, 4.5, 5.5].into_iter().enumerate() {
                e.mem.set_f32(a[2] + 4 * i as u32, value);
            }
            Ret::default()
        });
        let table = e.mem.alloc(0x80 * links + 0x80);
        for link in 0..links {
            for axis in 0..3 {
                let pair = table + 0x18 + 0x30 * link + 8 * axis;
                e.mem.set_u32(pair, 0x11 + 0x10 * axis);
                e.mem.set_u32(pair + 4, 0x12 + 0x10 * axis);
            }
        }
        let log = Rc::clone(&seen);
        e.register_double(MOTOR_TABLE_DOUBLE, move |_, a| {
            log.borrow_mut().table_arguments.push(a[1]);
            Ret {
                eax: table,
                ..Ret::default()
            }
        });
        let log = Rc::clone(&seen);
        e.register_double(FINISH_JACOBIANS, move |e, a| {
            let block: Vec<u32> = (0..13).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            let indices = (0..block[0].min(3) + 1)
                .map(|i| e.mem.u32(block[7] + 4 * i))
                .collect();
            let flag_bytes = (0..block[0])
                .map(|i| e.mem.u8(block[9] + 0x40 * i))
                .collect();
            log.borrow_mut().finish.push(FinishCall {
                block,
                indices,
                flag_bytes,
            });
            Ret::default()
        });

        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x28, MOTOR_TABLE_DOUBLE);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this, vtable);
        let entries = e.mem.alloc(0x50 * links + 0x50);
        e.mem.set_u32(this + 0x18, entries);
        e.mem.set_u32(this + 0x1c, links);
        for (i, value) in [0.5f32, 1.5, 2.5, 3.5, 4.5, 5.5, 6.5]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(this + 0x24 + 4 * i as u32, value);
        }
        for link in 0..links {
            let entry = entries + 0x50 * link;
            store_vec(&mut e, entry, [1.0, 2.0, 3.0, 0.0]);
            store_vec(&mut e, entry + 0x10, [4.0, 5.0, 6.0, 0.0]);
            store_vec(&mut e, entry + 0x20, [0.0, 0.0, 0.0, 1.0]);
            store_vec(&mut e, entry + 0x30, [0.0, 1.0, 0.0, 0.0]);
            for (i, motor) in [0xaau32, 0xbb, 0xcc].into_iter().enumerate() {
                e.mem.set_u32(entry + 0x40 + 4 * i as u32, motor);
            }
            e.mem.set_u8(entry + 0x4c, flag);
        }

        for (i, &body) in bodies.iter().enumerate() {
            e.mem.set_u32(body + 0xa4, 100 + i as u32);
            let shift = if i == 0 { 10.0 } else { 100.0 };
            for row in 0..3 {
                let mut v = [0.0f32; 4];
                v[row] = 1.0;
                store_vec(&mut e, body + 0xf0 + 0x10 * row as u32, v);
            }
            store_vec(&mut e, body + 0x120, [shift, shift * 2.0, shift * 3.0, 1.0]);
        }
        let list = e.mem.alloc(4 * count);
        for (i, &body) in bodies.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, body);
        }
        let side = e.mem.alloc(0x40);
        let flags = e.mem.alloc(0x30 * links + 0x100);
        e.mem.set_u32(side + 0x1c, flags);
        for link in 0..links {
            e.mem
                .set_u8(flags + 0x30 * links + link, 0x50 + (link & 0xf) as u8);
        }
        let chain = e.mem.alloc(0x50);
        e.mem.set_u32(chain + 0x14, bodies[0]);
        e.mem.set_u32(chain + 0x30, side);
        e.mem.set_u32(chain + 0x38, list);
        e.mem.set_u32(chain + 0x3c, count);

        let input = e.mem.alloc(0x100);
        e.mem.set_f32(input + 4, 2.0);
        e.mem.set_f32(input + 8, 3.0);
        e.mem.set_u32(input + 0x30, 1000);
        e.mem.set_u32(input + 0x60, chain);
        let cells = e.mem.alloc(0x40 * links + 0x100);
        e.mem.set_u32(input + 0x64, cells);
        store_vec(
            &mut e,
            cells + ((links + 3) & !3) + 3 * links * 16,
            persisted,
        );
        let output = e.mem.alloc(0x20);
        e.mem.set_u32(output, 0x77);
        Setup {
            e,
            seen,
            allocator,
            frees,
            this,
            input,
            output,
            cursors,
            cells,
            flags,
            bodies,
        }
    }

    #[test]
    fn build_jacobian_builds_the_ball_socket_and_three_motor_rows() {
        let mut s = setup(2, 0, [0.0; 4]);
        s.e.call(0x00e3_ba40, &args![s.this, s.input, s.output]);
        let e = &s.e;
        let seen = s.seen.borrow();
        let record = e.mem.u32(s.input + 0x64);
        assert_eq!(
            calls_to(e, HEADER_RECORD),
            vec![vec![s.input, s.output, record, 8]]
        );
        assert_eq!(calls_to(e, BALL_CURSOR), vec![vec![0x77]]);
        assert_eq!(calls_to(e, MOTOR_CURSOR), vec![vec![0x77, 1]]);
        assert_eq!(seen.ball.len(), 1);
        let (ball_args, pivot_a, pivot_b, copy) = &seen.ball[0];
        assert_eq!(ball_args[2], 6.5f32.to_bits());
        assert_eq!(ball_args[4], 0x1000);
        assert_eq!(*pivot_a, [11.0, 22.0, 33.0, 1.0]);
        assert_eq!(*pivot_b, [104.0, 205.0, 306.0, 1.0]);
        // The input copy: word 8 is word 2, word 9 is 1.0, the body words are shifted along
        // with the matrices, words 0x18 and 0x19 are cleared.
        assert_eq!(copy[1], 2.0f32.to_bits());
        assert_eq!(copy[8], 3.0f32.to_bits());
        assert_eq!(copy[9], 1.0f32.to_bits());
        assert_eq!((copy[0xc], copy[0xd]), (1000, 1001));
        assert_eq!(
            (copy[0xe], copy[0xf]),
            (s.bodies[0] + 0xf0, s.bodies[1] + 0xf0)
        );
        assert_eq!((copy[0x18], copy[0x19]), (0, 0));
        let copy_pointer = ball_args[3];
        assert_eq!(
            calls_to(e, LOAD_VELOCITY_ACCUMULATORS),
            vec![vec![copy_pointer]]
        );
        let cursors: Vec<u32> = seen.begin.iter().map(|b| b.0).collect();
        assert_eq!(cursors, vec![s.cursors, s.cursors + 0x20, s.cursors + 0x40]);
        assert!(seen.begin.iter().all(|b| b.2 == copy_pointer));
        let rows: Vec<Vec4> = seen.begin.iter().map(|b| b.1).collect();
        assert_eq!(
            rows,
            vec![
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0]
            ]
        );
        assert_eq!(seen.table_arguments, vec![s.flags; 3]);
        // The motors and the table words per axis; the two products agree (the stored
        // rotation was zero, so the entry's was used), so the first difference is zero and the
        // second is the doubled y axis of the relative rotation.
        let motors: Vec<u32> = seen.calc.iter().map(|c| c.0).collect();
        assert_eq!(motors, vec![0xaa, 0xbb, 0xcc]);
        for (axis, (low, high)) in [(0x11, 0x12), (0x21, 0x22), (0x31, 0x32)]
            .into_iter()
            .enumerate()
        {
            let query = &seen.calc[axis].1;
            assert_eq!(&query[..3], &[copy_pointer, low, high]);
            assert_eq!(
                (f32::from_bits(query[3]), f32::from_bits(query[4])),
                (0.0, [0.0, 2.0, 0.0][axis])
            );
        }
        assert_eq!(
            seen.products,
            vec![
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        );
        assert_eq!(load_vec(e, s.cells + 4 + 48), [0.0, 0.0, 0.0, 1.0]);
        // The finished block and the buffers it describes.
        assert_eq!(seen.finish.len(), 1);
        let finish = &seen.finish[0];
        assert_eq!(finish.block[0], 1);
        let tuning: Vec<f32> = finish.block[1..7]
            .iter()
            .map(|&w| f32::from_bits(w))
            .collect();
        assert_eq!(tuning, vec![0.5, 1.5, 2.5, 3.5, 4.5, 5.5]);
        assert_eq!(finish.indices, vec![100, 101]);
        assert_eq!(finish.block[8], 900);
        assert_eq!(finish.block[10], 0);
        assert_eq!(finish.block[11], s.flags + 0x30);
        assert_eq!(finish.block[12], s.cursors + 0x60);
        assert_eq!(finish.flag_bytes, vec![0x50]);
        assert!(s.frees.borrow().is_empty());
    }

    #[test]
    fn build_jacobian_swaps_the_quaternions_and_negates_the_differences_for_the_flag() {
        let mut s = setup(2, 1, [0.0, 0.0, 1.0, 0.0]);
        s.e.call(0x00e3_ba40, &args![s.this, s.input, s.output]);
        let seen = s.seen.borrow();
        assert_eq!(
            seen.products,
            vec![
                [0.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0]
            ]
        );
        // The differences are (-2, 2, 0, 0) and (2, 0, 0, 0) with their first three lanes
        // negated; each axis picks its lane.
        let lanes: Vec<(f32, f32)> = seen
            .calc
            .iter()
            .map(|c| (f32::from_bits(c.1[3]), f32::from_bits(c.1[4])))
            .collect();
        assert_eq!(lanes, vec![(2.0, -2.0), (-2.0, 0.0), (0.0, 0.0)]);
        // The stored rotation was non-zero, so it is kept for the product and overwritten
        // by the entry's afterwards.
        assert_eq!(load_vec(&s.e, s.cells + 4 + 48), [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn build_jacobian_with_one_body_has_no_links() {
        let mut s = setup(1, 0, [0.0; 4]);
        s.e.call(0x00e3_ba40, &args![s.this, s.input, s.output]);
        let seen = s.seen.borrow();
        assert!(seen.ball.is_empty() && seen.begin.is_empty() && seen.calc.is_empty());
        assert_eq!(calls_to(&s.e, MOTOR_CURSOR), vec![vec![0x77, 0]]);
        assert_eq!(seen.finish.len(), 1);
        assert_eq!(seen.finish[0].block[0], 0);
        assert_eq!(seen.finish[0].indices, vec![100]);
        assert!(seen.finish[0].flag_bytes.is_empty());
    }

    #[test]
    fn build_jacobian_grows_both_buffers_for_a_long_chain() {
        let mut s = setup(0x23, 0, [0.0; 4]);
        s.e.register_double(ARRAY_RESERVE, |e, a| {
            grow(e, a[1], a[3], a[2]);
            Ret::default()
        });
        s.e.register_double(ARRAY_RESERVE_MORE, |e, a| {
            let capacity = e.mem.u32(a[1] + 8) & CAPACITY_MASK;
            grow(e, a[1], a[2], capacity * 2);
            Ret::default()
        });
        s.e.call(0x00e3_ba40, &args![s.this, s.input, s.output]);
        let reserve = calls_to(&s.e, ARRAY_RESERVE);
        assert_eq!(reserve.len(), 1);
        assert_eq!(
            (reserve[0][0], reserve[0][2], reserve[0][3]),
            (s.allocator, 0x40, 0x40)
        );
        let more = calls_to(&s.e, ARRAY_RESERVE_MORE);
        assert_eq!(more.len(), 1);
        assert_eq!((more[0][0], more[0][2]), (s.allocator, 4));
        let seen = s.seen.borrow();
        assert_eq!(seen.ball.len(), 0x22);
        let finish = &seen.finish[0];
        assert_eq!(finish.block[0], 0x22);
        assert_eq!(finish.indices, vec![100, 101, 102, 103]);
        let expected: Vec<u8> = (0..0x22).map(|link| 0x50 + (link & 0xf) as u8).collect();
        assert_eq!(finish.flag_bytes, expected);
        // Both grown buffers are given back, the rows first.
        let frees = s.frees.borrow();
        assert_eq!(frees.len(), 2);
        assert_eq!(frees[0][1], 0x40 * 0x40);
        assert_eq!(frees[1][1], 0x40 * 4);
    }
}
