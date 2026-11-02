//! `Havok SDK/run_00d984b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{
    addps, addss, cmpeqps, cmpeqss, cmpltps, from_lane_bits, lane_bits, load_vec, movhlps, movlhps,
    movmskps, mulps, mulss, rsqrtps, rsqrtss, shufps, sqrtss, store_vec, subps, subss, unpckhps,
    unpcklps, Vec4,
};

/// The thread's memory router (`00c85750`); `+0x10` is the allocator (slot `+8` frees).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00df2750` and `00d1c2b0` (no unit in the engine map): the casts the two constructor-like
/// functions end with (cdecl).
const CAST_FIRST: u32 = 0x00df_2750;
const CAST_SECOND: u32 = 0x00d1_c2b0;
/// The float `0x01016248` (lane 0 broadcast) and the 16-byte vector `010ec7f0` the box padding
/// uses.
const PADDING: u32 = 0x0101_6248;
const BOX_SCALE: u32 = 0x010e_c7f0;
/// The `1.0` the vtable-less constructor stores (`+0x1c`) and its two vtables.
const VTABLE_FIRST: u32 = 0x010d_4094;
const VTABLE_SECOND: u32 = 0x010d_4088;
const VTABLE_FIRST_BASE: u32 = 0x010c_cfac;
/// The vtables the destructor `00d9a260` leaves behind.
const DESTROYED_VTABLE: u32 = 0x010c_cfa0;

/// The seven stack words `00d984b0` and `00d9a2a0` share (`arg1` .. `arg7`; `arg0` is theirs
/// alone).
struct CastArguments {
    arg1: u32,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    arg6: u32,
    arg7: u32,
}

/// The first half of `00d984b0` and `00d9a2a0` (the second shifts its fields by `shift` = 4):
/// records the arguments in `this`, copies the 0x38 bytes of the record at `arg4` (words `+0` to
/// `+0x14`, vectors `+0x20`, `+0x30` and `+0x40` to `this + 0x20..0x34` and `+0x40`, `+0x50`,
/// `+0x60`), takes the difference `d` of the vector at `arg2` and the translation at `[arg1 +
/// 8] + 0x30` (stored at `this + 0x70` and in the frame), its length (`SQRTSS` of the sum
/// `z + (y + x)`) at `this + 0x84`, asks the object `[arg1]` (virtual slot `0x1c`) for the
/// bounds of the shape (a 0x20-byte block in the frame) and then builds `(pad_vector - f) *
/// min + max * f` (`W1`), `W1 + d` (`W2`) and `f * (max - min)` (`W3`) in the frame. The frame
/// is the stack block the exe uses (`V`, 0x80 bytes): the byte result at `+0x17`, the length at
/// `+0x18`, the float `[arg2 + 0x14]` at `+0x1c`, `d` at `+0x20`, the bounds at `+0x30`, `W1`,
/// `W2` and `W3` at `+0x50`, `+0x60` and `+0x70`.
fn cast_setup(e: &mut Engine, this: u32, shift: u32, a: &CastArguments, frame: u32) {
    e.mem.set_u32(this + 0xc + shift, a.arg6);
    e.mem.set_u32(this + 8 + shift, a.arg3);
    e.mem.set_u32(this + 0x10 + shift, a.arg7);
    e.mem.set_u32(this + 0x14 + shift, a.arg1);
    e.mem.set_u32(this + 4 + shift, a.arg2);
    let node = e.mem.u32(a.arg1);
    let word = e.mem.u32(node + 0xc);
    e.mem.set_u32(this + 0x18 + shift, word);
    for offset in [0u32, 4, 8, 0xc, 0x10, 0x14] {
        let word = e.mem.u32(a.arg4 + offset);
        e.mem.set_u32(this + 0x20 + offset, word);
    }
    for (from, to) in [(0x20u32, 0x40u32), (0x30, 0x50), (0x40, 0x60)] {
        let vector = load_vec(e, a.arg4 + from);
        store_vec(e, this + to, vector);
    }
    e.mem.set_u32(this + 0x88, a.arg5);
    let radius = x87_quiet_bits(e.mem.u32(a.arg2 + 0x14));
    e.mem.set_u32(frame + 0x1c, radius);
    let transform = e.mem.u32(a.arg1 + 8);
    let translation = load_vec(e, transform + 0x30);
    let origin = load_vec(e, a.arg2);
    let difference = subps(origin, translation);
    store_vec(e, this + 0x70, difference);
    store_vec(e, frame + 0x20, difference);
    let squares = mulps(difference, difference);
    let y = shufps(squares, squares, 0x55);
    let z = shufps(squares, squares, 0xaa);
    let partial = addss(y, squares);
    let total = addss(z, partial);
    let length = sqrtss(total, total);
    e.mem.set_f32(frame + 0x18, length[0]);
    let stored = x87_quiet_bits(e.mem.u32(frame + 0x18));
    e.mem.set_u32(this + 0x84, stored);
    let radius_again = x87_quiet_bits(e.mem.u32(frame + 0x1c));
    e.mem.set_u32(this + 0x28, radius_again);
    let object = e.mem.u32(a.arg1);
    let shape_transform = e.mem.u32(a.arg1 + 8);
    let tolerance = x87_quiet_bits(e.mem.u32(a.arg2 + 0x14));
    e.vcall(
        object,
        0x1c,
        &args![shape_transform, f32::from_bits(tolerance), frame + 0x30],
    );
    let padding = [f32::from_bits(e.mem.u32(PADDING)), 0.0, 0.0, 0.0];
    let box_scale = load_vec(e, BOX_SCALE);
    let maximum = load_vec(e, frame + 0x40);
    let minimum = load_vec(e, frame + 0x30);
    let padding_all = shufps(padding, padding, 0);
    let scale = subps(box_scale, padding_all);
    let scaled_minimum = mulps(scale, minimum);
    let scaled_maximum = mulps(maximum, padding_all);
    let first_distance = x87_quiet_bits(e.mem.u32(a.arg2 + 0x10));
    e.mem.set_u32(this + 0x80, first_distance);
    let first = addps(scaled_minimum, scaled_maximum);
    store_vec(e, frame + 0x50, first);
    let moved = addps(first, load_vec(e, frame + 0x20));
    store_vec(e, frame + 0x60, moved);
    let extent = subps(maximum, minimum);
    let broadcast = shufps(padding, padding, 0);
    store_vec(e, frame + 0x70, mulps(broadcast, extent));
}

// Translated from 00d984b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, eight stack words `(a0, a1 .. a7)`, `RET 0x20`: prepares a ray-versus-shape query
/// in `this` (`cast_setup` with the fields as the exe has them, `this + 0xc` = `a6`, `+8` = `a3`,
/// `+0x10` = `a7`, `+0x14` = `a1`, `+4` = `a2`, `+0x18` = `[[a1] + 0xc]`) and hands it to
/// `00df2750` (cdecl) with `(frame + 0x17, a0, W1, this, float [a5])`: the byte result buffer,
/// the first word, the box built in the frame, the query and the float `[a5]`. Returns the
/// callee's `EAX`. The class is unconfirmed.
#[allow(clippy::too_many_arguments)]
pub fn fn_00d984b0(
    e: &mut Engine,
    this: Ptr,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
) -> u32 {
    let this = this.addr();
    let arguments = CastArguments {
        arg1: a1,
        arg2: a2,
        arg3: a3,
        arg4: a4,
        arg5: a5,
        arg6: a6,
        arg7: a7,
    };
    e.with_stack(0x80, |e, frame| {
        let frame = frame.addr();
        cast_setup(e, this, 0, &arguments, frame);
        let ray_float = x87_quiet_bits(e.mem.u32(a5));
        e.call(
            CAST_FIRST,
            &args![
                frame + 0x17,
                a0,
                frame + 0x50,
                this,
                f32::from_bits(ray_float)
            ],
        )
        .u32()
    })
}

// Translated from 00d9a2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00d984b0` with every field of `this` four bytes further on (`+0x10` = `a6`, `+0xc` =
/// `a3`, `+0x14` = `a7`, `+0x18` = `a1`, `+8` = `a2`, `+0x1c` = `[[a1] + 0xc]`, the rest as
/// there) and another end: `00d1c2b0` (cdecl) with `(a0, frame + 0x30, W1, this + 4, float
/// [a2 + 0x10], 0, 0)`. Returns the callee's `EAX`. The class is unconfirmed.
#[allow(clippy::too_many_arguments)]
pub fn fn_00d9a2a0(
    e: &mut Engine,
    this: Ptr,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
) -> u32 {
    let this = this.addr();
    let arguments = CastArguments {
        arg1: a1,
        arg2: a2,
        arg3: a3,
        arg4: a4,
        arg5: a5,
        arg6: a6,
        arg7: a7,
    };
    e.with_stack(0x80, |e, frame| {
        let frame = frame.addr();
        cast_setup(e, this, 4, &arguments, frame);
        let ray_float = x87_quiet_bits(e.mem.u32(a2 + 0x10));
        e.call(
            CAST_SECOND,
            &args![
                a0,
                frame + 0x30,
                frame + 0x50,
                this + 4,
                f32::from_bits(ray_float),
                0u32,
                0u32
            ],
        )
        .u32()
    })
}

// Translated from 00d98630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(out, b, a, mask)`: writes a byte at `out`, 1 when for each of the three vectors
/// `a_i - b_i` (`a_i = [a + 16 i]`, `b_i = [b + 16 i]`) the lanes selected by `mask` are all
/// negative, or all zero, or all positive (NaN lanes count as none), 0 as soon as one vector is
/// not. Returns `out`. The exe's name for it is unknown.
pub fn fn_00d98630(e: &mut Engine, out: u32, b: u32, a: u32, mask: u32) -> u32 {
    e.mem.set_u8(out, 1);
    let zero: Vec4 = [0.0; 4];
    let mut all = true;
    for index in 0..3u32 {
        let b_vector = load_vec(e, b.wrapping_add(16 * index));
        let a_vector = load_vec(e, a.wrapping_add(16 * index));
        let difference = subps(a_vector, b_vector);
        let negative = movmskps(cmpltps(difference, zero)) & mask == mask;
        let same = negative
            || movmskps(cmpeqps(difference, zero)) & mask == mask
            || movmskps(cmpltps(zero, difference)) & mask == mask;
        all = all && same;
    }
    e.mem.set_u8(out, u8::from(all));
    out
}

// Translated from 00d99fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(p, a1, a2, f3, f4, f5, a6)`: calls the traversal `00d986c0` with `([p], [p],
/// [p + 0x1c], &d, &t, a1, a2, f3, f4, f5, a6)` where `t` is a stack copy of the vector at `p +
/// 0x30` and `d` the difference `[p + 0x40] - [p + 0x30]` (the floats go through `FLD`/`FSTP`,
/// so a signalling NaN is quieted).
pub fn fn_00d99fc0(e: &mut Engine, p: u32, a1: u32, a2: u32, f3: f32, f4: f32, f5: f32, a6: u32) {
    let object = e.mem.u32(p);
    let copy_words: Vec<u32> = (0..4u32).map(|i| e.mem.u32(p + 0x30 + 4 * i)).collect();
    let far = load_vec(e, p + 0x40);
    let near = load_vec(e, p + 0x30);
    let held = e.mem.u32(p + 0x1c);
    let direction = subps(far, near);
    e.with_stack(0x20, |e, frame| {
        let frame = frame.addr();
        for (i, word) in copy_words.iter().enumerate() {
            e.mem.set_u32(frame + 4 * i as u32, *word);
        }
        store_vec(e, frame + 0x10, direction);
        fn_00d986c0(
            e,
            object,
            object,
            held,
            frame + 0x10,
            frame,
            a1,
            a2,
            f32::from_bits(x87_quiet_bits(f3.to_bits())),
            f32::from_bits(x87_quiet_bits(f4.to_bits())),
            f32::from_bits(x87_quiet_bits(f5.to_bits())),
            a6,
        );
    });
}

// Translated from 00d9a040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (thiscall, no stack words): the vtables `010d4094` at `+0` and `010d4088` at `+4`
/// (`010ccfac` is written first), the float 1.0 at `+0x1c` and zeros at `+8` .. `+0x18`.
/// Returns `this`. The class is unconfirmed.
pub fn fn_00d9a040(e: &mut Engine, this: Ptr) -> Ptr {
    let this = this.addr();
    e.mem.set_u32(this + 4, VTABLE_FIRST_BASE);
    e.mem.set_f32(this + 0x1c, 1.0);
    e.mem.set_u32(this, VTABLE_FIRST);
    e.mem.set_u32(this + 4, VTABLE_SECOND);
    for offset in [8u32, 0xc, 0x10, 0x14, 0x18] {
        e.mem.set_u32(this + offset, 0);
    }
    Ptr::new(this)
}

/// The 3x3 rotation of the transform `rotation` (columns at `+0`, `+0x10`, `+0x20`) applied
/// transposed to `v`: `v.y * row_y + v.x * row_x + v.z * row_z` with the rows built from the
/// columns by the exe's unpack / shuffle sequence (lane 3 of the rows is the exe's: `row_x` =
/// `(c0.x, c1.x, c2.x, c2.y)`, `row_y` = `(c0.y, c1.y, c2.y, c2.w)`, `row_z` = `(c0.z, c1.z,
/// c2.z, c2.w)`).
fn rotate_transposed(e: &Engine, rotation: u32, v: Vec4) -> Vec4 {
    let c0 = load_vec(e, rotation);
    let c1 = load_vec(e, rotation + 0x10);
    let c2 = load_vec(e, rotation + 0x20);
    let low = unpcklps(c0, c1);
    let high = unpckhps(c0, c1);
    let row_x = movlhps(low, c2);
    let from_low = movhlps(row_x, low);
    let row_y = shufps(from_low, c2, 0xd4);
    let row_z = shufps(high, c2, 0xe4);
    let by_y = mulps(shufps(v, v, 0x55), row_y);
    let by_x = mulps(shufps(v, v, 0x00), row_x);
    let sum = addps(by_y, by_x);
    let by_z = mulps(row_z, shufps(v, v, 0xaa));
    addps(sum, by_z)
}

// Translated from 00d9a070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, one stack word (`RET 4`): `query` points to a word `shape` (null ends it) and a
/// transform pointer at `+8`. When the object at `[this + 0xc]` exists its slot `+4` (called with
/// a result byte, `[this + 8]` and `query`) must answer a non-zero byte. Then the ray of
/// `[this + 8]` (from at `+0`, to at `+0x10`) is taken into the shape's frame
/// (`rotate_transposed` of each point less the translation at `+0x30`) into a 0x28-byte frame
/// (from, to, the word at `[[this + 8] + 0x24]`, `[this + 0x10]`). With `[this + 0x18]` non-null
/// the shape's slot `+0x24` is called with `(&frame, query, [this + 0x18])` and the float
/// `[[this + 0x18] + 4]` is kept at `this + 0x1c`; otherwise slot `+0x20` is called with
/// `(&result byte, &frame, [this + 0x14])`; a non-zero byte then stores `query` at `[this +
/// 0x14] + 0x50`, rotates the vector at `[this + 0x14]` by the transform's columns
/// (`(y * c1 + x * c0) + z * c2`) and keeps the float `[[this + 0x14] + 0x10]` at `this +
/// 0x1c`. The class is unconfirmed.
pub fn fn_00d9a070(e: &mut Engine, this: Ptr, query: u32) {
    let this = this.addr();
    let shape = e.mem.u32(query);
    if shape == 0 {
        return;
    }
    // The exe's frame: the byte result at `+0x1b`, the shape at `+0x1c`, the ray frame at
    // `+0x20` (0x28 bytes).
    e.with_stack(0x50, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x1c, shape);
        let first = e.mem.u32(this + 0xc);
        if first != 0 {
            let second = e.mem.u32(this + 8);
            let table = e.mem.u32(first);
            let function = e.mem.u32(table + 4);
            let flag = e
                .call(function, &args![first, frame + 0x1b, second, query])
                .u32();
            if e.mem.u8(flag) == 0 {
                return;
            }
        }
        let transform = e.mem.u32(query + 8);
        let ray = e.mem.u32(this + 8);
        e.mem.set_u32(frame + 0x40, 0);
        e.mem.set_u32(frame + 0x44, 0);
        let translation = load_vec(e, transform + 0x30);
        let from = load_vec(e, ray);
        let from_local = rotate_transposed(e, transform, subps(from, translation));
        store_vec(e, frame + 0x20, from_local);
        let translation = load_vec(e, transform + 0x30);
        let to = load_vec(e, ray + 0x10);
        let to_local = rotate_transposed(e, transform, subps(to, translation));
        store_vec(e, frame + 0x30, to_local);
        let word = e.mem.u32(ray + 0x24);
        let held = e.mem.u32(this + 0x10);
        e.mem.set_u32(frame + 0x44, held);
        let handler = e.mem.u32(this + 0x18);
        e.mem.set_u32(frame + 0x40, word);
        let object = e.mem.u32(frame + 0x1c);
        let table = e.mem.u32(object);
        if handler != 0 {
            let function = e.mem.u32(table + 0x24);
            e.call(function, &args![object, frame + 0x20, query, handler]);
            let handler = e.mem.u32(this + 0x18);
            let fraction = x87_quiet_bits(e.mem.u32(handler + 4));
            e.mem.set_u32(this + 0x1c, fraction);
            return;
        }
        let holder = e.mem.u32(this + 0x14);
        let function = e.mem.u32(table + 0x20);
        e.call(function, &args![object, frame + 0x1b, frame + 0x20, holder]);
        if e.mem.u8(frame + 0x1b) == 0 {
            return;
        }
        let holder = e.mem.u32(this + 0x14);
        e.mem.set_u32(holder + 0x50, query);
        let holder = e.mem.u32(this + 0x14);
        let v = load_vec(e, holder);
        let c1 = load_vec(e, transform + 0x10);
        let c0 = load_vec(e, transform);
        let by_y = mulps(shufps(v, v, 0x55), c1);
        let by_x = mulps(shufps(v, v, 0x00), c0);
        let sum = addps(by_y, by_x);
        let c2 = load_vec(e, transform + 0x20);
        let by_z = mulps(shufps(v, v, 0xaa), c2);
        store_vec(e, holder, addps(sum, by_z));
        let holder = e.mem.u32(this + 0x14);
        let fraction = x87_quiet_bits(e.mem.u32(holder + 0x10));
        e.mem.set_u32(this + 0x1c, fraction);
    });
}

// Translated from 00d9a220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, one stack word (`RET 4`), the result in `ST0`: runs `00d9a070` on `this` with
/// `[arg + 0x20]` and returns the float at `this + 0x1c`. The class is unconfirmed.
pub fn fn_00d9a220(e: &mut Engine, this: Ptr, arg: u32) -> f32 {
    let word = e.mem.u32(arg + 0x20);
    fn_00d9a070(e, this, word);
    f32::from_bits(x87_quiet_bits(e.mem.u32(this.addr() + 0x1c)))
}

// Translated from 00d9a240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, one stack word (`RET 4`), the result in `ST0`: runs `00d9a070` on `this - 4` with
/// `[arg + 0xc]` and returns the float at `this + 0x18`. The class is unconfirmed.
pub fn fn_00d9a240(e: &mut Engine, this: Ptr, arg: u32) -> f32 {
    let word = e.mem.u32(arg + 0xc);
    fn_00d9a070(e, Ptr::new(this.addr().wrapping_sub(4)), word);
    f32::from_bits(x87_quiet_bits(e.mem.u32(this.addr() + 0x18)))
}

// Translated from 00d9a260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor thunk (the exe's `SUB ECX,4` and then the body at `00d9a270`):
/// on `this - 4` it stores the vtables `010ccfac` at `+4` and `010ccfa0` at `+0` and, with bit
/// 0 of `flags` set, frees the 0x20-byte block through the memory router's allocator (slot
/// `+8`). Returns the adjusted pointer. The class is unconfirmed.
pub fn fn_00d9a260(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    let base = this.addr().wrapping_sub(4);
    e.mem.set_u32(base + 4, VTABLE_FIRST_BASE);
    e.mem.set_u32(base, DESTROYED_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![base, 0x20u32]);
    }
    base
}

// Translated from 00d9a270 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00d9a270 to the `RET 4` at 00d9a29c, inside the extent the
/// map gives `00d9a260`): the deleting destructor body [`fn_00d9a260`] reaches after its
/// `SUB ECX, 4`, entered directly from its own vtable slot. `thiscall` with one stack word: stores
/// `010ccfac` at `+4` and `010ccfa0` at `+0` and, with bit 0 of the word set, frees the 0x20-byte
/// block through the memory router's allocator (slot `+8`). Answers `this`.
pub fn fn_00d9a270(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    let base = this.addr();
    e.mem.set_u32(base + 4, VTABLE_FIRST_BASE);
    e.mem.set_u32(base, DESTROYED_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![base, 0x20u32]);
    }
    base
}

// Translated from 00d9a410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, one stack word (`RET 4`): `arg` points to a word `shape` (null ends it); with
/// `[this + 0x18]` set and different from `arg`, the object at `[this + 0xc]` slot `+4` (called
/// with a result byte, `[this + 0x18]` and `arg`) must answer a non-zero byte; then the function
/// `[[this + 0x20] + 0x9ac + 0x14 * m]` is called (cdecl) with `([this + 0x18], arg, this +
/// 0x20, [this + 0x10], [this + 0x14])`, with `m` the byte at `[this + 0x20] + 32 * ([this +
/// 0x1c] + 0xd) + [shape + 0xc]`. The class is unconfirmed.
pub fn fn_00d9a410(e: &mut Engine, this: Ptr, arg: u32) {
    let this = this.addr();
    let shape = e.mem.u32(arg);
    if shape == 0 {
        return;
    }
    let held = e.mem.u32(this + 0x18);
    if held == arg {
        return;
    }
    let first = e.mem.u32(this + 0xc);
    let table = e.mem.u32(first);
    let function = e.mem.u32(table + 4);
    e.with_stack(8, |e, scratch| {
        let flag = e
            .call(function, &args![first, scratch.addr(), held, arg])
            .u32();
        if e.mem.u8(flag) == 0 {
            return;
        }
        let kind = e.mem.u32(this + 0x1c);
        let dispatcher = e.mem.u32(this + 0x20);
        let row = kind
            .wrapping_add(0xd)
            .wrapping_shl(5)
            .wrapping_add(e.mem.u32(shape + 0xc));
        let third = e.mem.u32(this + 0x14);
        let selector = u32::from(e.mem.u8(row.wrapping_add(dispatcher)));
        let second = e.mem.u32(this + 0x10);
        let function = e.mem.u32(
            dispatcher
                .wrapping_add(selector.wrapping_mul(5).wrapping_mul(4))
                .wrapping_add(0x9ac),
        );
        let held = e.mem.u32(this + 0x18);
        e.call(function, &args![held, arg, this + 0x20, second, third]);
    });
}

// Translated from 00d9a480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, one stack word (`RET 4`), the result in `ST0`: runs `00d9a410` on `this - 4` with
/// `[arg + 0xc]` and returns the float at `[this + 0xc] + 4`. The class is unconfirmed.
pub fn fn_00d9a480(e: &mut Engine, this: Ptr, arg: u32) -> f32 {
    let word = e.mem.u32(arg + 0xc);
    fn_00d9a410(e, Ptr::new(this.addr().wrapping_sub(4)), word);
    let holder = e.mem.u32(this.addr() + 0xc);
    f32::from_bits(x87_quiet_bits(e.mem.u32(holder + 4)))
}

// Translated from 00d9a4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, one stack word (`RET 4`), the result in `ST0`: runs `00d9a410` on `this` with
/// `[arg + 0x20]` and returns the float at `[this + 0x10] + 4`. The class is unconfirmed.
pub fn fn_00d9a4a0(e: &mut Engine, this: Ptr, arg: u32) -> f32 {
    let word = e.mem.u32(arg + 0x20);
    fn_00d9a410(e, this, word);
    let holder = e.mem.u32(this.addr() + 0x10);
    f32::from_bits(x87_quiet_bits(e.mem.u32(holder + 4)))
}

// Translated from 00d9a4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, one stack word (`RET 4`): the address of element `index` of an array of 0x30-byte
/// elements whose data pointer is `[this]`.
pub fn fn_00d9a4c0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    index
        .wrapping_mul(3)
        .wrapping_shl(4)
        .wrapping_add(e.mem.u32(this.addr()))
}

// Translated from 00d9a4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, one stack word (`RET 4`): the address of element `index` (16 bits) of an array of
/// 0x14-byte elements whose data pointer is `[this]`.
pub fn fn_00d9a4d0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let element = (index & 0xffff).wrapping_mul(5).wrapping_mul(4);
    e.mem.u32(this.addr()).wrapping_add(element)
}

// Translated from 00d9a4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, one stack word (`RET 4`): copies a 0x28-byte record from `source` to `this`: two
/// vectors (`+0`, `+0x10`), the word at `+0x20` (the exe copies it twice) and the 16-bit words at
/// `+0x24` and `+0x26`. Returns `this`. The class is unconfirmed.
pub fn fn_00d9a4e0(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    let this_address = this.addr();
    for offset in [0u32, 0x10] {
        let vector = load_vec(e, source.wrapping_add(offset));
        store_vec(e, this_address.wrapping_add(offset), vector);
    }
    for _ in 0..2 {
        let word = e.mem.u32(source.wrapping_add(0x20));
        e.mem.set_u32(this_address.wrapping_add(0x20), word);
    }
    let half = e.mem.u16(source.wrapping_add(0x24));
    e.mem.set_u16(this_address.wrapping_add(0x24), half);
    let half = e.mem.u16(source.wrapping_add(0x26));
    e.mem.set_u16(this_address.wrapping_add(0x26), half);
    this
}

use crate::units::havok_sdk::run_00d6bf60::{
    andnps, andps, cmpleps, cvtdq2ps, fcom_status, maxps, minps, orps, pslld, psrld, punpckhwd,
    punpcklwd, rcpps,
};

/// `00c98210` and `00c9e0f0` (no unit in the engine map): cdecl `(float, float)`, the result in
/// `ST0`: the larger and the smaller of the two (the second one when they are equal or a NaN is
/// involved).
const FLOAT_MAX: u32 = 0x00c9_8210;
const FLOAT_MIN: u32 = 0x00c9_e0f0;
/// The `double`s `65535.0` (`010217d0`) and `0.25` (`010290b0`) of the node decoding.
const SIXTEEN_BIT_RANGE: u32 = 0x0102_17d0;
const MARGIN: u32 = 0x0102_90b0;
/// The axis mask table `1, 2, 4, 8` (`010c92e0`).
const AXIS_MASKS: u32 = 0x010c_92e0;
/// The floats broadcast in the leaf scan: `010cd644` (the weight of the high halves of the box
/// words), `0104e148` and `0104da48` (the answers for lanes whose ray component is zero).
const HIGH_WEIGHT: u32 = 0x010c_d644;
const ZERO_LANE_NEAR: u32 = 0x0104_e148;
const ZERO_LANE_FAR: u32 = 0x0104_da48;
/// The 16-byte vectors `011b6de0` (the scale of the 16-bit box words) and `010c7170` (ones).
const BOX_WORD_SCALE: u32 = 0x011b_6de0;
const ONES: u32 = 0x010c_7170;

/// The `float` at `address` loaded with `FLD`.
fn load_float(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

/// The `float` at `base + 4 * axis` as a memory operand of an x87 arithmetic instruction.
fn operand(e: &Engine, base: u32, axis: u32) -> f64 {
    x87_m32(e.mem.u32(base.wrapping_add(4 * axis)))
}

fn load_lanes(e: &Engine, address: u32) -> Vec4 {
    load_vec(e, address)
}

/// `FLD`/`FSTP` of a `float` stack argument: the bits quieted, as a `float` again.
fn quiet_float(bits: u32) -> f32 {
    f32::from_bits(x87_quiet_bits(bits))
}

/// A `float` through an x87 register and back (`FSTP float`): the stored bits as a `float`.
fn stored_float(value: f64) -> f32 {
    f32::from_bits(x87_store32(value))
}

// Translated from 00d986c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The ray traversal of a packed tree (no Xbox PDB name; it is the walk the kd-tree ray casts of
/// `hkpaabbtreeworldmanager.rs` use). Cdecl, eleven stack words: `a0` (passed on to the
/// recursion), `node` (8 bytes: a header dword and a second dword), `a2` (the leaf entries), `a3`
/// and `a4` (scale and offset vectors), `a5` (the ray origin), `a6` (the ray direction), the
/// floats `a7` and `a8` (the interval the ray is limited to), `a9` (a scale) and `a10` (the
/// collector object).
///
/// An inner node (header sign bit set) has its children at `node + (header & 0x7ffffffc)` and
/// eight bytes further; the split planes are on the axis `header & 3` at `(16-bit word /
/// 65535) * a3[axis] + a4[axis]` for the low and the high 16 bits of `[node + 4]`. When the ray
/// does not move along the axis (the direction's lane is zero) the origin's position against
/// the planes (`+/- 0.25`) picks the children: the first by recursion, the second by looping.
/// Otherwise the plane distances `(plane - origin) / direction` are taken, the children ordered
/// by the direction's sign, `a10`'s slot `+0xc` (a length, times `a9`, kept at most `a8`) decides
/// how far the interval reaches, and the first child, the second or both (the first by
/// recursion with the interval cut by `00c98210`/`00c9e0f0`, then the second when the interval is
/// still non-empty) are visited; `a10`'s slot `+8` is called once with a result byte when both
/// are. A leaf (header sign bit clear; `0x7fffffff` or a count of at most 0 ends it) has
/// `[node + 4]` entries of 16 bytes (eight 16-bit words: two boxes' corners as three words each,
/// ids in words 3 and 7) at `a2 + 16 * header`; each box is decoded with `011b6de0` and
/// `a3`/`a4`, intersected with the ray (`RCPPS` of the direction with one refinement step, zero
/// where the direction is zero) and, when the intersection of the three slabs is non-empty (or
/// the origin is inside), `a10`'s slot `+4` is called with a result byte and the id. `RCPPS` is
/// the host estimate, so results can differ in the last bit from a CPU of another maker.
#[allow(clippy::too_many_arguments)]
pub fn fn_00d986c0(
    e: &mut Engine,
    a0: u32,
    node: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: f32,
    a8: f32,
    a9: f32,
    a10: u32,
) {
    let mut node = node;
    let zero: Vec4 = [0.0; 4];
    // The frame's byte result buffer (`+0x17` in the exe's stack frame).
    e.with_stack(0x20, |e, frame| {
        let frame = frame.addr();
        let mut header = e.mem.u32(node);
        let mut field = e.mem.u32(node.wrapping_add(4));
        // `~(header >> 31)` is non-zero for a leaf.
        if (header as i32) >= 0 {
            leaf(e, frame, header, field, [a2, a3, a4, a5, a6], [a7, a8], a10);
            return;
        }
        let range = e.mem.f64(SIXTEEN_BIT_RANGE);
        let margin = e.mem.f64(MARGIN);
        loop {
            let first_child = node.wrapping_add(header & 0x7fff_fffc);
            let second_child = first_child.wrapping_add(8);
            let low = f64::from(field & 0xffff);
            let high = f64::from(field >> 16);
            let axis = header & 3;
            let direction = load_lanes(e, a6);
            let mask = e.mem.u32(AXIS_MASKS + 4 * axis);
            let zero_lanes = movmskps(cmpeqps(zero, direction)) & mask;
            let low_plane = x87_store32(x87_add(
                x87_mul(x87_div(low, range), operand(e, a3, axis)),
                operand(e, a4, axis),
            ));
            let high_plane = x87_store32(x87_add(
                x87_mul(x87_div(high, range), load_float(e, a3 + 4 * axis)),
                load_float(e, a4 + 4 * axis),
            ));
            if zero_lanes == mask {
                // The ray does not move along the axis.
                let origin = load_float(e, a5 + 4 * axis);
                let high_with_margin = x87_add(x87_load32(high_plane), margin);
                let visit_first = fcom_status(high_with_margin, origin) & 1 == 0;
                let origin_again = load_float(e, a5 + 4 * axis);
                let low_without_margin = x87_sub(x87_load32(low_plane), margin);
                let visit_second = fcom_status(origin_again, low_without_margin) & 1 == 0;
                if visit_first {
                    fn_00d986c0(
                        e,
                        a0,
                        first_child,
                        a2,
                        a3,
                        a4,
                        a5,
                        a6,
                        quiet_float(a7.to_bits()),
                        quiet_float(a8.to_bits()),
                        quiet_float(a9.to_bits()),
                        a10,
                    );
                }
                if !visit_second {
                    return;
                }
                node = second_child;
            } else {
                // The ray moves along the axis: the plane distances along the ray.
                let positive = cmpltps(zero, direction);
                let t_low = x87_store32(x87_div(
                    x87_sub(x87_load32(low_plane), operand(e, a5, axis)),
                    operand(e, a6, axis),
                ));
                let t_high = x87_store32(x87_div(
                    x87_sub(x87_load32(high_plane), operand(e, a5, axis)),
                    operand(e, a6, axis),
                ));
                let along = movmskps(positive) & mask;
                // The child to visit first, the one to visit second and their distances.
                let (first, second, mut t_first, t_second) = if along == mask {
                    (first_child, second_child, t_low, t_high)
                } else {
                    (second_child, first_child, t_high, t_low)
                };
                let length = e.vcall(a10, 0xc, &args![]).f64();
                let scaled = x87_store32(x87_mul(length, x87_m32(a9.to_bits())));
                let scaled_loaded = x87_load32(scaled);
                let end_loaded = x87_load32(a8.to_bits());
                // `FCOM`: the interval end `a8` is kept unless it is above the scaled length.
                let clamped = if fcom_status(end_loaded, scaled_loaded) & 0x41 == 0 {
                    x87_store32(scaled_loaded)
                } else {
                    x87_store32(end_loaded)
                };
                let second_t_loaded = x87_load32(t_second);
                let start_loaded = x87_load32(a7.to_bits());
                if fcom_status(start_loaded, second_t_loaded) & 0x41 == 0 {
                    // The interval starts beyond the first range: only the second child.
                    node = second;
                } else if !matches!(
                    fcom_status(x87_load32(clamped), x87_load32(t_first)) & 5,
                    0 | 5
                ) {
                    // The interval ends inside the first range: only the first child.
                    node = first;
                } else {
                    // Both children: the first by recursion, then the second.
                    e.vcall(a10, 8, &args![frame + 0x17]);
                    let entry = e
                        .call(
                            FLOAT_MAX,
                            &args![quiet_float(t_first), quiet_float(a7.to_bits())],
                        )
                        .f64();
                    t_first = x87_store32(entry);
                    let exit = e
                        .call(
                            FLOAT_MIN,
                            &args![quiet_float(a8.to_bits()), quiet_float(t_second)],
                        )
                        .f64();
                    fn_00d986c0(
                        e,
                        a0,
                        first,
                        a2,
                        a3,
                        a4,
                        a5,
                        a6,
                        quiet_float(a7.to_bits()),
                        stored_float(exit),
                        quiet_float(a9.to_bits()),
                        a10,
                    );
                    let length = e.vcall(a10, 0xc, &args![]).f64();
                    let scaled = x87_store32(x87_mul(length, x87_m32(a9.to_bits())));
                    let limit = e
                        .call(
                            FLOAT_MIN,
                            &args![quiet_float(scaled), quiet_float(a8.to_bits())],
                        )
                        .f64();
                    let entry_loaded = x87_load32(t_first);
                    if !matches!(fcom_status(entry_loaded, limit) & 5, 0 | 5) {
                        fn_00d986c0(
                            e,
                            a0,
                            second,
                            a2,
                            a3,
                            a4,
                            a5,
                            a6,
                            stored_float(entry_loaded),
                            quiet_float(a8.to_bits()),
                            quiet_float(a9.to_bits()),
                            a10,
                        );
                    }
                    return;
                }
            }
            header = e.mem.u32(node);
            field = e.mem.u32(node.wrapping_add(4));
            if (header as i32) >= 0 {
                leaf(e, frame, header, field, [a2, a3, a4, a5, a6], [a7, a8], a10);
                return;
            }
        }
    });
}

/// The leaf scan of `00d986c0`: `header` and `count` are the node's two dwords, `words` the
/// words `a2` .. `a6` and `floats` the interval `a7` and `a8` (their bits are broadcast as the
/// exe's `MOVSS`/`SHUFPS` do). See `fn_00d986c0` for the layout.
fn leaf(
    e: &mut Engine,
    frame: u32,
    header: u32,
    count: u32,
    words: [u32; 5],
    floats: [f32; 2],
    a10: u32,
) {
    if header == 0x7fff_ffff || (count as i32) <= 0 {
        return;
    }
    let [a2, a3, a4, a5, a6] = words;
    let [a7, a8] = floats;
    let zero: Vec4 = [0.0; 4];
    let weight = [quiet_float(e.mem.u32(HIGH_WEIGHT)); 4];
    let near_value = [quiet_float(e.mem.u32(ZERO_LANE_NEAR)); 4];
    let far_value = [quiet_float(e.mem.u32(ZERO_LANE_FAR)); 4];
    let start_all = [a7; 4];
    let end_all = [a8; 4];
    let mut entry = a2.wrapping_add((header & 0x7fff_ffff) << 4);
    for _ in 0..count {
        let dwords: [u32; 4] = std::array::from_fn(|lane| e.mem.u32(entry + 4 * lane as u32));
        let first_word = e.mem.u16(entry + 6);
        let second_word = e.mem.u16(entry + 0xe);
        let scale = mulps(load_lanes(e, BOX_WORD_SCALE), load_lanes(e, a3));
        let offset = load_lanes(e, a4);
        let direction = load_lanes(e, a6);
        let low_words = punpcklwd(dwords, [0; 4]);
        let high_words = punpckhwd(dwords, [0; 4]);
        let low_part = cvtdq2ps(psrld(pslld(low_words, 16), 16));
        let low_top = cvtdq2ps(psrld(low_words, 16));
        let box_min = addps(mulps(low_top, weight), low_part);
        let high_part = cvtdq2ps(psrld(pslld(high_words, 16), 16));
        let high_top = cvtdq2ps(psrld(high_words, 16));
        let box_max = addps(mulps(high_top, weight), high_part);
        let box_min = addps(mulps(box_min, scale), offset);
        let box_max = addps(mulps(box_max, scale), offset);
        let spread = unpckhps(direction, load_lanes(e, ONES));
        let packed = shufps(direction, spread, 0xc4);
        let estimate = rcpps(packed);
        let refined = subps(
            addps(estimate, estimate),
            mulps(mulps(estimate, packed), estimate),
        );
        let is_zero = cmpeqps(zero, direction);
        let inverse = orps(andnps(is_zero, refined), andps(is_zero, zero));
        let origin = load_lanes(e, a5);
        let near_raw = mulps(subps(box_min, origin), inverse);
        let far_raw = mulps(subps(box_max, origin), inverse);
        let forward = cmpleps(zero, inverse);
        let flat = cmpeqps(zero, inverse);
        let low_pick = orps(andnps(forward, far_raw), andps(forward, near_raw));
        let t_near = orps(andnps(flat, low_pick), andps(flat, near_value));
        let high_pick = orps(andps(forward, far_raw), andnps(forward, near_raw));
        let t_far = orps(andnps(flat, high_pick), andps(flat, far_value));
        let far_min = minps(
            minps(shufps(t_far, t_far, 0x00), shufps(t_far, t_far, 0x55)),
            shufps(t_far, t_far, 0xaa),
        );
        let end = minps(end_all, far_min);
        let near_max = maxps(
            maxps(shufps(t_near, t_near, 0x00), shufps(t_near, t_near, 0x55)),
            shufps(t_near, t_near, 0xaa),
        );
        let start = maxps(start_all, near_max);
        let hit = cmpltps(start, end);
        let not_flat_hit = andnps(flat, hit);
        let inside = andps(
            andps(cmpleps(box_min, origin), cmpleps(origin, box_max)),
            flat,
        );
        let result = orps(not_flat_hit, inside);
        if movmskps(result) & 7 == 7 {
            let id = u32::from(first_word) | (u32::from(second_word) << 16);
            e.vcall(a10, 4, &args![frame + 0x17, id]);
        }
        entry = entry.wrapping_add(16);
    }
}

/// A float of the image as `MOVSS xmm, [address]` loads it: lane 0, the other lanes zero.
fn scalar_vec(e: &Engine, address: u32) -> Vec4 {
    [f32::from_bits(e.mem.u32(address)), 0.0, 0.0, 0.0]
}

/// The registers of the two ray-cast walks `00d98ca0` and `00d99570`. These two functions keep
/// five values on the x87 stack across their loops and jump between more than twenty places, so
/// they are translated instruction by instruction: every register, flag and stack word of the
/// exe is here (the frame is real memory, `esp` the real address), and each instruction of the
/// exe is one statement on this state. Only the flags the two functions test are kept (`zf`,
/// `sf`, `of`, `pf`).
#[derive(Default)]
struct Cpu {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    esi: u32,
    edi: u32,
    ebp: u32,
    esp: u32,
    /// `XMM0` to `XMM7`.
    x: [Vec4; 8],
    /// The x87 stack, `ST(0)` last.
    st: Vec<f64>,
    /// The x87 status word (the condition codes `FNSTSW` stores).
    sw: u16,
    zf: bool,
    sf: bool,
    of: bool,
    pf: bool,
}

impl Cpu {
    fn set_result(&mut self, result: u32) {
        self.zf = result == 0;
        self.sf = result >> 31 != 0;
        self.pf = (result as u8).count_ones() % 2 == 0;
    }

    fn logic(&mut self, result: u32) -> u32 {
        self.set_result(result);
        self.of = false;
        result
    }

    fn and32(&mut self, a: u32, b: u32) -> u32 {
        self.logic(a & b)
    }

    fn xor32(&mut self, a: u32, b: u32) -> u32 {
        self.logic(a ^ b)
    }

    fn add32(&mut self, a: u32, b: u32) -> u32 {
        let result = a.wrapping_add(b);
        self.set_result(result);
        self.of = ((a ^ result) & (b ^ result)) >> 31 != 0;
        result
    }

    fn sub32(&mut self, a: u32, b: u32) -> u32 {
        let result = a.wrapping_sub(b);
        self.set_result(result);
        self.of = ((a ^ b) & (a ^ result)) >> 31 != 0;
        result
    }

    fn cmp32(&mut self, a: u32, b: u32) {
        self.sub32(a, b);
    }

    /// `CMP` of two bytes (the low bytes of the operands).
    fn cmp8(&mut self, a: u32, b: u32) {
        let (a, b) = (a as u8, b as u8);
        let result = a.wrapping_sub(b);
        self.zf = result == 0;
        self.sf = result >> 7 != 0;
        self.pf = result.count_ones() % 2 == 0;
        self.of = ((a ^ b) & (a ^ result)) >> 7 != 0;
    }

    fn test32(&mut self, a: u32, b: u32) {
        self.logic(a & b);
    }

    /// `TEST` of two bytes (the low bytes of the operands).
    fn test8(&mut self, a: u32, b: u32) {
        let result = (a & b) as u8;
        self.zf = result == 0;
        self.sf = result >> 7 != 0;
        self.pf = result.count_ones() % 2 == 0;
        self.of = false;
    }

    /// `SHL` by an immediate count (the flags follow the result; an undefined `OF` is cleared).
    fn shl32(&mut self, a: u32, count: u32) -> u32 {
        let count = count & 31;
        if count == 0 {
            return a;
        }
        let result = a << count;
        self.set_result(result);
        self.of = false;
        result
    }

    /// `SHR` by an immediate count.
    fn shr32(&mut self, a: u32, count: u32) -> u32 {
        let count = count & 31;
        if count == 0 {
            return a;
        }
        let result = a >> count;
        self.set_result(result);
        self.of = false;
        result
    }

    fn push(&mut self, e: &mut Engine, value: u32) {
        self.esp = self.esp.wrapping_sub(4);
        e.mem.set_u32(self.esp, value);
    }

    fn pop(&mut self, e: &Engine) -> u32 {
        let value = e.mem.u32(self.esp);
        self.esp = self.esp.wrapping_add(4);
        value
    }

    fn st_push(&mut self, value: f64) {
        self.st.push(value);
    }

    fn st_pop(&mut self) {
        self.st
            .pop()
            .expect("the exe never pops an empty x87 stack");
    }

    fn st(&self, i: usize) -> f64 {
        self.st[self.st.len() - 1 - i]
    }

    fn set_st(&mut self, i: usize, value: f64) {
        let top = self.st.len() - 1;
        self.st[top - i] = value;
    }

    fn st_exchange(&mut self, i: usize) {
        let top = self.st.len() - 1;
        self.st.swap(top, top - i);
    }
}

// Translated from 00d98ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The ray cast through a packed tree with an explicit stack: the loop form of the walk
/// `00d986c0` makes by recursion (no Xbox PDB name). Cdecl `(result, query, from, to, collector)`,
/// answers `result`. `query` describes the tree: the root node pointer at `+0`, the capacity of
/// the pending-node stack in entries at `+0xc` (a count below 1 means 0x80), the leaf entries
/// at `+0x1c`, and the box the 16-bit coordinates map into as its minimum at `+0x30` and
/// maximum at `+0x40`; `from` and `to` are the ray's end points. The pending-node stack (16
/// bytes per entry) is taken from the memory router's stack (`00c85750`, its first word is the
/// stack top, moved up by the size rounded to 0x80 and put back at the end). The collector
/// object's virtual slots are asked for a length limit (`+0xc`, a float) and told of each box
/// the ray crosses (`+4`, the two id words of the entry and the address of a result byte); at
/// the end its slot `+8` gets `result`. The walk is the exe's code instruction by instruction on
/// the `Cpu` state above (the exe keeps five values on the x87 stack through the loops and
/// jumps between more than twenty places); `RCPPS` and `RSQRTSS` are the host's estimates.
pub fn fn_00d98ca0(
    e: &mut Engine,
    result: u32,
    query: u32,
    from: u32,
    to: u32,
    collector: u32,
) -> u32 {
    run_00d98ca0(e, [result, query, from, to, collector], 0)
}

/// `00d98ca0` on a frame whose words start as `fill` (the exe's frame has whatever was on the
/// stack: the walk must not depend on it).
fn run_00d98ca0(e: &mut Engine, args: [u32; 5], fill: u32) -> u32 {
    e.with_stack(0x400, |e, frame| {
        let top = frame.addr() + 0x3c0;
        for word in 0..0x100 {
            e.mem.set_u32(frame.addr() + 4 * word, fill);
        }
        let mut cpu = Cpu {
            esp: top,
            ..Cpu::default()
        };
        e.mem.set_u32(top, 0);
        for (i, arg) in args.iter().enumerate() {
            e.mem.set_u32(top + 4 + 4 * i as u32, *arg);
        }
        let mut pc: u32 = 0x00d9_8ca0;
        'run: loop {
            match pc {
                0x00d98ca0 => {
                    // 00d98ca0: PUSH EBP
                    cpu.push(e, cpu.ebp);
                    // 00d98ca1: MOV EBP,ESP
                    cpu.ebp = cpu.esp;
                    // 00d98ca3: AND ESP,0xfffffff0
                    cpu.esp = cpu.and32(cpu.esp, 0xfffffff0);
                    // 00d98ca6: SUB ESP,0x184
                    cpu.esp = cpu.sub32(cpu.esp, 0x184);
                    // 00d98cac: MOV EAX,dword ptr [EBP + 0xc]
                    cpu.eax = e.mem.u32(cpu.ebp.wrapping_add(0xc));
                    // 00d98caf: MOV ECX,dword ptr [EAX + 0x1c]
                    cpu.ecx = e.mem.u32(cpu.eax.wrapping_add(0x1c));
                    // 00d98cb2: MOV EDX,dword ptr [EAX + 0x30]
                    cpu.edx = e.mem.u32(cpu.eax.wrapping_add(0x30));
                    // 00d98cb5: MOVAPS XMM0,xmmword ptr [EAX + 0x40]
                    cpu.x[0] = load_vec(e, cpu.eax.wrapping_add(0x40));
                    // 00d98cb9: MOVAPS XMM1,xmmword ptr [EAX + 0x30]
                    cpu.x[1] = load_vec(e, cpu.eax.wrapping_add(0x30));
                    // 00d98cbd: MOV dword ptr [ESP + 0x60],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x60), cpu.ecx);
                    // 00d98cc1: MOV ECX,dword ptr [EAX + 0x34]
                    cpu.ecx = e.mem.u32(cpu.eax.wrapping_add(0x34));
                    // 00d98cc4: PUSH EBX
                    cpu.push(e, cpu.ebx);
                    // 00d98cc5: PUSH ESI
                    cpu.push(e, cpu.esi);
                    // 00d98cc6: MOV ESI,dword ptr [EAX]
                    cpu.esi = e.mem.u32(cpu.eax);
                    // 00d98cc8: MOV dword ptr [ESP + 0x6c],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x6c), cpu.edx);
                    // 00d98ccc: MOV EDX,dword ptr [EAX + 0x38]
                    cpu.edx = e.mem.u32(cpu.eax.wrapping_add(0x38));
                    // 00d98ccf: MOV dword ptr [ESP + 0x70],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x70), cpu.ecx);
                    // 00d98cd3: MOV ECX,dword ptr [EAX + 0x3c]
                    cpu.ecx = e.mem.u32(cpu.eax.wrapping_add(0x3c));
                    // 00d98cd6: MOV EAX,dword ptr [EAX + 0xc]
                    cpu.eax = e.mem.u32(cpu.eax.wrapping_add(0xc));
                    // 00d98cd9: SUBPS XMM0,XMM1
                    cpu.x[0] = subps(cpu.x[0], cpu.x[1]);
                    // 00d98cdc: PUSH EDI
                    cpu.push(e, cpu.edi);
                    // 00d98cdd: MOV dword ptr [ESP + 0x78],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x78), cpu.edx);
                    // 00d98ce1: MOV dword ptr [ESP + 0x7c],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x7c), cpu.ecx);
                    // 00d98ce5: MOVAPS xmmword ptr [ESP + 0xe0],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0xe0), cpu.x[0]);
                    // 00d98ced: MOV dword ptr [ESP + 0x14],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x14), cpu.eax);
                    // 00d98cf1: TEST EAX,EAX
                    cpu.test32(cpu.eax, cpu.eax);
                    // 00d98cf3: JG 0x00d98cfd
                    if !cpu.zf && cpu.sf == cpu.of {
                        pc = 0x00d98cfd;
                        continue 'run;
                    }
                    // 00d98cf5: MOV dword ptr [ESP + 0x14],0x80
                    e.mem.set_u32(cpu.esp.wrapping_add(0x14), 0x80);
                    pc = 0x00d98cfd;
                    continue 'run;
                }
                0x00d98cfd => {
                    // 00d98cfd: MOV EDI,dword ptr [ESI]
                    cpu.edi = e.mem.u32(cpu.esi);
                    // 00d98cff: MOV EBX,dword ptr [ESI + 0x4]
                    cpu.ebx = e.mem.u32(cpu.esi.wrapping_add(0x4));
                    // 00d98d02: CALL 0x00c85750
                    cpu.eax = e.call(0x00c85750, &args![]).u32();
                    // 00d98d07: FLDZ
                    cpu.st_push(0.0);
                    // 00d98d09: MOV ECX,dword ptr [ESP + 0x14]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x14));
                    // 00d98d0d: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x20), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d98d11: MOV EDX,dword ptr [EAX]
                    cpu.edx = e.mem.u32(cpu.eax);
                    // 00d98d13: MOVSS XMM5,dword ptr [0x01011d78]
                    cpu.x[5] = scalar_vec(e, 0x01011d78);
                    // 00d98d1b: MOVSS XMM6,dword ptr [0x01017718]
                    cpu.x[6] = scalar_vec(e, 0x01017718);
                    // 00d98d23: SHL ECX,0x4
                    cpu.ecx = cpu.shl32(cpu.ecx, 0x4);
                    // 00d98d26: ADD ECX,0x7f
                    cpu.ecx = cpu.add32(cpu.ecx, 0x7f);
                    // 00d98d29: AND ECX,0xffffff80
                    cpu.ecx = cpu.and32(cpu.ecx, 0xffffff80);
                    // 00d98d2c: ADD ECX,EDX
                    cpu.ecx = cpu.add32(cpu.ecx, cpu.edx);
                    // 00d98d2e: MOV dword ptr [EAX],ECX
                    e.mem.set_u32(cpu.eax, cpu.ecx);
                    // 00d98d30: MOV EAX,dword ptr [EBP + 0x10]
                    cpu.eax = e.mem.u32(cpu.ebp.wrapping_add(0x10));
                    // 00d98d33: MOVAPS XMM0,xmmword ptr [EAX]
                    cpu.x[0] = load_vec(e, cpu.eax);
                    // 00d98d36: MOV dword ptr [ESP + 0x3c],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x3c), cpu.edx);
                    // 00d98d3a: MOV dword ptr [ESP + 0x24],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x24), cpu.edx);
                    // 00d98d3e: MOV EDX,dword ptr [EBP + 0x14]
                    cpu.edx = e.mem.u32(cpu.ebp.wrapping_add(0x14));
                    // 00d98d41: MOVAPS XMM3,xmmword ptr [EDX]
                    cpu.x[3] = load_vec(e, cpu.edx);
                    // 00d98d44: SUBPS XMM3,XMM0
                    cpu.x[3] = subps(cpu.x[3], cpu.x[0]);
                    // 00d98d47: MOV ECX,dword ptr [EAX]
                    cpu.ecx = e.mem.u32(cpu.eax);
                    // 00d98d49: MOV EDX,dword ptr [EAX + 0x4]
                    cpu.edx = e.mem.u32(cpu.eax.wrapping_add(0x4));
                    // 00d98d4c: MOVAPS XMM0,XMM3
                    cpu.x[0] = cpu.x[3];
                    // 00d98d4f: MULPS XMM0,XMM3
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[3]);
                    // 00d98d52: MOVAPS XMM2,XMM0
                    cpu.x[2] = cpu.x[0];
                    // 00d98d55: SHUFPS XMM2,XMM0,0x55
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[0], 85);
                    // 00d98d59: ADDSS XMM2,XMM0
                    cpu.x[2] = addss(cpu.x[2], cpu.x[0]);
                    // 00d98d5d: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00d98d60: SHUFPS XMM1,XMM0,0xaa
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[0], 170);
                    // 00d98d64: ADDSS XMM1,XMM2
                    cpu.x[1] = addss(cpu.x[1], cpu.x[2]);
                    // 00d98d68: MOVAPS XMM2,XMM1
                    cpu.x[2] = cpu.x[1];
                    // 00d98d6b: RSQRTSS XMM2,XMM2
                    cpu.x[2] = rsqrtss(cpu.x[2], cpu.x[2]);
                    // 00d98d6f: MOVAPS XMM4,XMM2
                    cpu.x[4] = cpu.x[2];
                    // 00d98d72: XORPS XMM2,XMM2
                    cpu.x[2] = [0.0; 4];
                    // 00d98d75: MOVSS XMM2,XMM5
                    {
                        let s = cpu.x[5][0];
                        cpu.x[2][0] = s;
                    }
                    // 00d98d79: MOVAPS XMM5,XMM2
                    cpu.x[5] = cpu.x[2];
                    // 00d98d7c: MOVAPS XMM2,XMM1
                    cpu.x[2] = cpu.x[1];
                    // 00d98d7f: MULSS XMM1,XMM4
                    cpu.x[1] = mulss(cpu.x[1], cpu.x[4]);
                    // 00d98d83: MULSS XMM1,XMM4
                    cpu.x[1] = mulss(cpu.x[1], cpu.x[4]);
                    // 00d98d87: CMPEQSS XMM2,XMM5
                    cpu.x[2] = cmpeqss(cpu.x[2], cpu.x[5]);
                    // 00d98d8c: XORPS XMM5,XMM5
                    cpu.x[5] = [0.0; 4];
                    // 00d98d8f: MOVSS XMM5,XMM6
                    {
                        let s = cpu.x[6][0];
                        cpu.x[5][0] = s;
                    }
                    // 00d98d93: MOVSS XMM6,dword ptr [0x01016248]
                    cpu.x[6] = scalar_vec(e, 0x01016248);
                    // 00d98d9b: SUBSS XMM5,XMM1
                    cpu.x[5] = subss(cpu.x[5], cpu.x[1]);
                    // 00d98d9f: XORPS XMM1,XMM1
                    cpu.x[1] = [0.0; 4];
                    // 00d98da2: MOVSS XMM1,XMM6
                    {
                        let s = cpu.x[6][0];
                        cpu.x[1][0] = s;
                    }
                    // 00d98da6: MULSS XMM1,XMM4
                    cpu.x[1] = mulss(cpu.x[1], cpu.x[4]);
                    // 00d98daa: MULSS XMM1,XMM5
                    cpu.x[1] = mulss(cpu.x[1], cpu.x[5]);
                    // 00d98dae: ANDNPS XMM2,XMM1
                    cpu.x[2] = andnps(cpu.x[2], cpu.x[1]);
                    // 00d98db1: SHUFPS XMM2,XMM2,0x0
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[2], 0);
                    // 00d98db5: MOVAPS XMM4,XMM2
                    cpu.x[4] = cpu.x[2];
                    // 00d98db8: MULPS XMM4,XMM3
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[3]);
                    // 00d98dbb: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00d98dbe: SHUFPS XMM1,XMM0,0x55
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[0], 85);
                    // 00d98dc2: ADDSS XMM1,XMM0
                    cpu.x[1] = addss(cpu.x[1], cpu.x[0]);
                    // 00d98dc6: MOVAPS XMM3,XMM0
                    cpu.x[3] = cpu.x[0];
                    // 00d98dc9: SHUFPS XMM3,XMM0,0xaa
                    cpu.x[3] = shufps(cpu.x[3], cpu.x[0], 170);
                    // 00d98dcd: ADDSS XMM3,XMM1
                    cpu.x[3] = addss(cpu.x[3], cpu.x[1]);
                    // 00d98dd1: MULSS XMM3,XMM2
                    cpu.x[3] = mulss(cpu.x[3], cpu.x[2]);
                    // 00d98dd5: XORPS XMM5,XMM5
                    cpu.x[5] = [0.0; 4];
                    // 00d98dd8: MOVSS dword ptr [ESP + 0x34],XMM3
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x34), cpu.x[3][0].to_bits());
                    // 00d98dde: FLD float ptr [ESP + 0x34]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x34))));
                    // 00d98de2: MOVAPS XMM3,xmmword ptr [0x010c7170]
                    cpu.x[3] = load_vec(e, 0x010c7170);
                    // 00d98de9: FSTP float ptr [ESP + 0x2c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x2c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d98ded: MOVAPS XMM1,XMM5
                    cpu.x[1] = cpu.x[5];
                    // 00d98df0: CMPEQPS XMM1,XMM4
                    cpu.x[1] = cmpeqps(cpu.x[1], cpu.x[4]);
                    // 00d98df4: MOV dword ptr [ESP + 0x50],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x50), cpu.ecx);
                    // 00d98df8: MOV ECX,dword ptr [EAX + 0x8]
                    cpu.ecx = e.mem.u32(cpu.eax.wrapping_add(0x8));
                    // 00d98dfb: MOV dword ptr [ESP + 0x54],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x54), cpu.edx);
                    // 00d98dff: MOV EDX,dword ptr [EAX + 0xc]
                    cpu.edx = e.mem.u32(cpu.eax.wrapping_add(0xc));
                    // 00d98e02: MOVAPS XMM0,XMM1
                    cpu.x[0] = cpu.x[1];
                    // 00d98e05: MOVAPS XMM2,XMM1
                    cpu.x[2] = cpu.x[1];
                    // 00d98e08: ANDNPS XMM0,XMM4
                    cpu.x[0] = andnps(cpu.x[0], cpu.x[4]);
                    // 00d98e0b: ANDPS XMM2,XMM3
                    cpu.x[2] = andps(cpu.x[2], cpu.x[3]);
                    // 00d98e0e: MOVAPS xmmword ptr [ESP + 0x150],XMM4
                    store_vec(e, cpu.esp.wrapping_add(0x150), cpu.x[4]);
                    // 00d98e16: MOV dword ptr [ESP + 0x58],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x58), cpu.ecx);
                    // 00d98e1a: MOV dword ptr [ESP + 0x5c],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x5c), cpu.edx);
                    // 00d98e1e: MOVAPS xmmword ptr [ESP + 0xa0],XMM5
                    store_vec(e, cpu.esp.wrapping_add(0xa0), cpu.x[5]);
                    // 00d98e26: MOVAPS xmmword ptr [ESP + 0x80],XMM1
                    store_vec(e, cpu.esp.wrapping_add(0x80), cpu.x[1]);
                    // 00d98e2e: ORPS XMM0,XMM2
                    cpu.x[0] = orps(cpu.x[0], cpu.x[2]);
                    // 00d98e31: FLD float ptr [0x010d4080]
                    cpu.st_push(x87_load32(e.mem.u32(0x010d4080)));
                    // 00d98e37: MOV ECX,dword ptr [EBP + 0x18]
                    cpu.ecx = e.mem.u32(cpu.ebp.wrapping_add(0x18));
                    // 00d98e3a: MOV EAX,dword ptr [ECX]
                    cpu.eax = e.mem.u32(cpu.ecx);
                    // 00d98e3c: FST float ptr [ESP + 0x40]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x40), x87_store32(cpu.st(0)));
                    // 00d98e40: MOV EDX,dword ptr [EAX + 0xc]
                    cpu.edx = e.mem.u32(cpu.eax.wrapping_add(0xc));
                    // 00d98e43: FST float ptr [ESP + 0x44]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x44), x87_store32(cpu.st(0)));
                    // 00d98e47: MOVAPS XMM2,XMM0
                    cpu.x[2] = cpu.x[0];
                    // 00d98e4a: FST float ptr [ESP + 0x48]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x48), x87_store32(cpu.st(0)));
                    // 00d98e4e: UNPCKHPS XMM0,XMM3
                    cpu.x[0] = unpckhps(cpu.x[0], cpu.x[3]);
                    // 00d98e51: FSTP float ptr [ESP + 0x4c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x4c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d98e55: SHUFPS XMM2,XMM0,0xc4
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[0], 196);
                    // 00d98e59: RCPPS XMM3,XMM2
                    cpu.x[3] = rcpps(cpu.x[2]);
                    // 00d98e5c: MOVAPS XMM0,XMM1
                    cpu.x[0] = cpu.x[1];
                    // 00d98e5f: MOVAPS XMM4,XMM3
                    cpu.x[4] = cpu.x[3];
                    // 00d98e62: MULPS XMM4,XMM2
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[2]);
                    // 00d98e65: ANDPS XMM1,XMM5
                    cpu.x[1] = andps(cpu.x[1], cpu.x[5]);
                    // 00d98e68: MOVAPS XMM2,XMM3
                    cpu.x[2] = cpu.x[3];
                    // 00d98e6b: ADDPS XMM2,XMM3
                    cpu.x[2] = addps(cpu.x[2], cpu.x[3]);
                    // 00d98e6e: MULPS XMM4,XMM3
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[3]);
                    // 00d98e71: MOVAPS XMM3,xmmword ptr [0x010c4c10]
                    cpu.x[3] = load_vec(e, 0x010c4c10);
                    // 00d98e78: SUBPS XMM2,XMM4
                    cpu.x[2] = subps(cpu.x[2], cpu.x[4]);
                    // 00d98e7b: ANDPS XMM2,XMM3
                    cpu.x[2] = andps(cpu.x[2], cpu.x[3]);
                    // 00d98e7e: ANDNPS XMM0,XMM2
                    cpu.x[0] = andnps(cpu.x[0], cpu.x[2]);
                    // 00d98e81: MOVAPS XMM2,xmmword ptr [ESP + 0x40]
                    cpu.x[2] = load_vec(e, cpu.esp.wrapping_add(0x40));
                    // 00d98e86: ORPS XMM0,XMM1
                    cpu.x[0] = orps(cpu.x[0], cpu.x[1]);
                    // 00d98e89: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00d98e8c: MULPS XMM1,xmmword ptr [ESP + 0xe0]
                    cpu.x[1] = mulps(cpu.x[1], load_vec(e, cpu.esp.wrapping_add(0xe0)));
                    // 00d98e94: MULPS XMM1,XMM2
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[2]);
                    // 00d98e97: MOVAPS xmmword ptr [ESP + 0x110],XMM1
                    store_vec(e, cpu.esp.wrapping_add(0x110), cpu.x[1]);
                    // 00d98e9f: MOVAPS XMM1,xmmword ptr [ESP + 0x70]
                    cpu.x[1] = load_vec(e, cpu.esp.wrapping_add(0x70));
                    // 00d98ea4: SUBPS XMM1,xmmword ptr [ESP + 0x50]
                    cpu.x[1] = subps(cpu.x[1], load_vec(e, cpu.esp.wrapping_add(0x50)));
                    // 00d98ea9: MULPS XMM1,XMM0
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[0]);
                    // 00d98eac: MOVAPS xmmword ptr [ESP + 0xf0],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0xf0), cpu.x[0]);
                    // 00d98eb4: MOVAPS xmmword ptr [ESP + 0x100],XMM1
                    store_vec(e, cpu.esp.wrapping_add(0x100), cpu.x[1]);
                    // 00d98ebc: CALL EDX
                    {
                        let r = e.call(cpu.edx, &args![cpu.ecx]);
                        cpu.st_push(r.st0);
                    }
                    // 00d98ebe: FLD float ptr [ESP + 0x34]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x34))));
                    // 00d98ec2: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00d98ec4: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00d98ec6: FXCH
                    cpu.st_exchange(1);
                    // 00d98ec8: FSTP float ptr [ESP + 0x30]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x30), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d98ecc: FLD float ptr [ESP + 0x30]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x30))));
                    // 00d98ed0: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00d98ed2: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00d98ed4: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00d98ed7: JP 0x00d98edd
                    if cpu.pf {
                        pc = 0x00d98edd;
                        continue 'run;
                    }
                    // 00d98ed9: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00d98edb: JMP 0x00d98edf
                    pc = 0x00d98edf;
                    continue 'run;
                }
                0x00d98edd => {
                    // 00d98edd: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00d98edf;
                    continue 'run;
                }
                0x00d98edf => {
                    // 00d98edf: MOVAPS XMM0,xmmword ptr [ESP + 0x80]
                    cpu.x[0] = load_vec(e, cpu.esp.wrapping_add(0x80));
                    // 00d98ee7: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d98eeb: MOVMSKPS EAX,XMM0
                    cpu.eax = movmskps(cpu.x[0]);
                    // 00d98eee: AND EAX,0x7
                    cpu.eax = cpu.and32(cpu.eax, 0x7);
                    // 00d98ef1: CMP AL,0x7
                    cpu.cmp8(cpu.eax & 0xff, 0x7);
                    // 00d98ef3: JZ 0x00d9951c
                    if cpu.zf {
                        pc = 0x00d9951c;
                        continue 'run;
                    }
                    // 00d98ef9: FLD float ptr [0x010cd644]
                    cpu.st_push(x87_load32(e.mem.u32(0x010cd644)));
                    // 00d98eff: MOVAPS XMM6,xmmword ptr [ESP + 0xa0]
                    cpu.x[6] = load_vec(e, cpu.esp.wrapping_add(0xa0));
                    // 00d98f07: FLD float ptr [0x0104e148]
                    cpu.st_push(x87_load32(e.mem.u32(0x0104e148)));
                    // 00d98f0d: FLD float ptr [0x0104da48]
                    cpu.st_push(x87_load32(e.mem.u32(0x0104da48)));
                    // 00d98f13: FLD float ptr [ESP + 0x20]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x20))));
                    // 00d98f17: FLD float ptr [ESP + 0x2c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x2c))));
                    // 00d98f1b: JMP 0x00d98f25
                    pc = 0x00d98f25;
                    continue 'run;
                }
                0x00d98f1d => {
                    // 00d98f1d: FXCH ST4
                    cpu.st_exchange(4);
                    // 00d98f1f: FXCH
                    cpu.st_exchange(1);
                    // 00d98f21: FXCH ST3
                    cpu.st_exchange(3);
                    // 00d98f23: FXCH
                    cpu.st_exchange(1);
                    pc = 0x00d98f25;
                    continue 'run;
                }
                0x00d98f25 => {
                    // 00d98f25: TEST EDI,EDI
                    cpu.test32(cpu.edi, cpu.edi);
                    // 00d98f27: JNS 0x00d99045
                    if !cpu.sf {
                        pc = 0x00d99045;
                        continue 'run;
                    }
                    // 00d98f2d: MOVAPS XMM0,xmmword ptr [ESP + 0x150]
                    cpu.x[0] = load_vec(e, cpu.esp.wrapping_add(0x150));
                    // 00d98f35: MOVAPS XMM1,XMM6
                    cpu.x[1] = cpu.x[6];
                    // 00d98f38: CMPEQPS XMM1,XMM0
                    cpu.x[1] = cmpeqps(cpu.x[1], cpu.x[0]);
                    // 00d98f3c: MOVMSKPS ECX,XMM1
                    cpu.ecx = movmskps(cpu.x[1]);
                    // 00d98f3f: MOV dword ptr [ESP + 0x28],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x28), cpu.ecx);
                    pc = 0x00d98f43;
                    continue 'run;
                }
                0x00d98f43 => {
                    // 00d98f43: MOV EDX,dword ptr [ESI]
                    cpu.edx = e.mem.u32(cpu.esi);
                    // 00d98f45: AND EDX,0x7ffffffc
                    cpu.edx = cpu.and32(cpu.edx, 0x7ffffffc);
                    // 00d98f4b: ADD ESI,EDX
                    cpu.esi = cpu.add32(cpu.esi, cpu.edx);
                    // 00d98f4d: MOV EDX,dword ptr [ESP + 0x28]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0x28));
                    // 00d98f51: AND EDI,0x3
                    cpu.edi = cpu.and32(cpu.edi, 0x3);
                    // 00d98f54: MOV EAX,dword ptr [EDI*0x4 + 0x10c92e0]
                    cpu.eax = e.mem.u32(cpu.edi.wrapping_mul(4).wrapping_add(0x010c92e0));
                    // 00d98f5b: AND EDX,EAX
                    cpu.edx = cpu.and32(cpu.edx, cpu.eax);
                    // 00d98f5d: LEA ECX,[ESI + 0x8]
                    cpu.ecx = cpu.esi.wrapping_add(0x8);
                    // 00d98f60: CMP EDX,EAX
                    cpu.cmp32(cpu.edx, cpu.eax);
                    // 00d98f62: MOV dword ptr [ESP + 0x14],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x14), cpu.ecx);
                    // 00d98f66: MOVZX EDX,BX
                    cpu.edx = cpu.ebx & 0xffff;
                    // 00d98f69: JNZ 0x00d99161
                    if !cpu.zf {
                        pc = 0x00d99161;
                        continue 'run;
                    }
                    // 00d98f6f: MOV dword ptr [ESP + 0x1c],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x1c), cpu.edx);
                    // 00d98f73: FILD dword ptr [ESP + 0x1c]
                    cpu.st_push(f64::from(e.mem.i32(cpu.esp.wrapping_add(0x1c))));
                    // 00d98f77: LEA EAX,[ESP + EDI*0x4 + 0xe0]
                    cpu.eax = cpu
                        .esp
                        .wrapping_add(cpu.edi.wrapping_mul(4))
                        .wrapping_add(0xe0);
                    // 00d98f7e: FLD double ptr [0x010217d0]
                    cpu.st_push(e.mem.f64(0x010217d0));
                    // 00d98f84: LEA ECX,[ESP + EDI*0x4 + 0x70]
                    cpu.ecx = cpu
                        .esp
                        .wrapping_add(cpu.edi.wrapping_mul(4))
                        .wrapping_add(0x70);
                    // 00d98f88: SHR EBX,0x10
                    cpu.ebx = cpu.shr32(cpu.ebx, 0x10);
                    // 00d98f8b: FDIV ST1,ST0
                    {
                        let v = x87_div(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    // 00d98f8d: LEA EDX,[ESP + EDI*0x4 + 0x50]
                    cpu.edx = cpu
                        .esp
                        .wrapping_add(cpu.edi.wrapping_mul(4))
                        .wrapping_add(0x50);
                    // 00d98f91: MOV dword ptr [ESP + 0x1c],EBX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x1c), cpu.ebx);
                    // 00d98f95: FLD float ptr [EAX]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.eax)));
                    // 00d98f97: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00d98f99: FLD float ptr [ECX]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.ecx)));
                    // 00d98f9b: FADDP ST2,ST0
                    {
                        let v = x87_add(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00d98f9d: FXCH
                    cpu.st_exchange(1);
                    // 00d98f9f: FSTP float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x10), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d98fa3: FLD float ptr [EDX]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.edx)));
                    // 00d98fa5: FILD dword ptr [ESP + 0x1c]
                    cpu.st_push(f64::from(e.mem.i32(cpu.esp.wrapping_add(0x1c))));
                    // 00d98fa9: FDIVRP ST2,ST0
                    {
                        let v = x87_div(cpu.st(0), cpu.st(2));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00d98fab: FLD float ptr [EAX]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.eax)));
                    // 00d98fad: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00d98faf: FLD float ptr [ECX]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.ecx)));
                    // 00d98fb1: FADDP ST2,ST0
                    {
                        let v = x87_add(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00d98fb3: FXCH
                    cpu.st_exchange(1);
                    // 00d98fb5: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d98fb9: FLD float ptr [ESP + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                    // 00d98fbd: FLD double ptr [0x01032980]
                    cpu.st_push(e.mem.f64(0x01032980));
                    // 00d98fc3: FADD ST1,ST0
                    {
                        let v = x87_add(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    // 00d98fc5: FXCH
                    cpu.st_exchange(1);
                    // 00d98fc7: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d98fcb: FLD float ptr [ESP + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                    // 00d98fcf: FCOMP ST2
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(2))) << 8;
                    cpu.st_pop();
                    // 00d98fd1: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00d98fd3: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00d98fd5: TEST AH,0x1
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x1);
                    // 00d98fd8: JNZ 0x00d98fe1
                    if !cpu.zf {
                        pc = 0x00d98fe1;
                        continue 'run;
                    }
                    // 00d98fda: MOV ECX,0x1
                    cpu.ecx = 0x1;
                    // 00d98fdf: JMP 0x00d98fe3
                    pc = 0x00d98fe3;
                    continue 'run;
                }
                0x00d98fe1 => {
                    // 00d98fe1: XOR ECX,ECX
                    cpu.ecx = cpu.xor32(cpu.ecx, cpu.ecx);
                    pc = 0x00d98fe3;
                    continue 'run;
                }
                0x00d98fe3 => {
                    // 00d98fe3: FLD float ptr [EDX]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.edx)));
                    // 00d98fe5: FLD float ptr [ESP + 0x10]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x10))));
                    // 00d98fe9: FSUBRP ST2,ST0
                    {
                        let v = x87_sub(cpu.st(0), cpu.st(2));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00d98feb: FXCH
                    cpu.st_exchange(1);
                    // 00d98fed: FSTP float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x10), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d98ff1: FLD float ptr [ESP + 0x10]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x10))));
                    // 00d98ff5: FCOMPP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    cpu.st_pop();
                    // 00d98ff7: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00d98ff9: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00d98ffc: JP 0x00d99005
                    if cpu.pf {
                        pc = 0x00d99005;
                        continue 'run;
                    }
                    // 00d98ffe: MOV EAX,0x1
                    cpu.eax = 0x1;
                    // 00d99003: JMP 0x00d99007
                    pc = 0x00d99007;
                    continue 'run;
                }
                0x00d99005 => {
                    // 00d99005: XOR EAX,EAX
                    cpu.eax = cpu.xor32(cpu.eax, cpu.eax);
                    pc = 0x00d99007;
                    continue 'run;
                }
                0x00d99007 => {
                    // 00d99007: TEST ECX,ECX
                    cpu.test32(cpu.ecx, cpu.ecx);
                    // 00d99009: JZ 0x00d9902c
                    if cpu.zf {
                        pc = 0x00d9902c;
                        continue 'run;
                    }
                    // 00d9900b: TEST EAX,EAX
                    cpu.test32(cpu.eax, cpu.eax);
                    // 00d9900d: JZ 0x00d99038
                    if cpu.zf {
                        pc = 0x00d99038;
                        continue 'run;
                    }
                    // 00d9900f: MOV EAX,dword ptr [ESP + 0x24]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x24));
                    // 00d99013: FXCH
                    cpu.st_exchange(1);
                    // 00d99015: MOV ECX,dword ptr [ESP + 0x14]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x14));
                    // 00d99019: FST float ptr [EAX + 0x4]
                    e.mem
                        .set_u32(cpu.eax.wrapping_add(0x4), x87_store32(cpu.st(0)));
                    // 00d9901c: FXCH
                    cpu.st_exchange(1);
                    // 00d9901e: MOV dword ptr [EAX],ECX
                    e.mem.set_u32(cpu.eax, cpu.ecx);
                    // 00d99020: FST float ptr [EAX + 0x8]
                    e.mem
                        .set_u32(cpu.eax.wrapping_add(0x8), x87_store32(cpu.st(0)));
                    // 00d99023: ADD EAX,0x10
                    cpu.eax = cpu.add32(cpu.eax, 0x10);
                    // 00d99026: MOV dword ptr [ESP + 0x24],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x24), cpu.eax);
                    // 00d9902a: JMP 0x00d99038
                    pc = 0x00d99038;
                    continue 'run;
                }
                0x00d9902c => {
                    // 00d9902c: TEST EAX,EAX
                    cpu.test32(cpu.eax, cpu.eax);
                    // 00d9902e: JZ 0x00d99480
                    if cpu.zf {
                        pc = 0x00d99480;
                        continue 'run;
                    }
                    // 00d99034: MOV ESI,dword ptr [ESP + 0x14]
                    cpu.esi = e.mem.u32(cpu.esp.wrapping_add(0x14));
                    pc = 0x00d99038;
                    continue 'run;
                }
                0x00d99038 => {
                    // 00d99038: MOV EDI,dword ptr [ESI]
                    cpu.edi = e.mem.u32(cpu.esi);
                    // 00d9903a: MOV EBX,dword ptr [ESI + 0x4]
                    cpu.ebx = e.mem.u32(cpu.esi.wrapping_add(0x4));
                    // 00d9903d: TEST EDI,EDI
                    cpu.test32(cpu.edi, cpu.edi);
                    // 00d9903f: JS 0x00d98f43
                    if cpu.sf {
                        pc = 0x00d98f43;
                        continue 'run;
                    }
                    pc = 0x00d99045;
                    continue 'run;
                }
                0x00d99045 => {
                    // 00d99045: MOV ECX,EDI
                    cpu.ecx = cpu.edi;
                    // 00d99047: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00d99049: XOR ECX,0x7fffffff
                    cpu.ecx = cpu.xor32(cpu.ecx, 0x7fffffff);
                    // 00d9904f: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00d99051: JZ 0x00d99484
                    if cpu.zf {
                        pc = 0x00d99484;
                        continue 'run;
                    }
                    // 00d99057: TEST EBX,EBX
                    cpu.test32(cpu.ebx, cpu.ebx);
                    // 00d99059: JLE 0x00d99484
                    if cpu.zf || cpu.sf != cpu.of {
                        pc = 0x00d99484;
                        continue 'run;
                    }
                    // 00d9905f: MOVAPS XMM5,xmmword ptr [ESP + 0xf0]
                    cpu.x[5] = load_vec(e, cpu.esp.wrapping_add(0xf0));
                    // 00d99067: FXCH ST2
                    cpu.st_exchange(2);
                    // 00d99069: MOV EDX,dword ptr [ESP + 0x6c]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0x6c));
                    // 00d9906d: FST float ptr [ESP + 0xb0]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xb0), x87_store32(cpu.st(0)));
                    // 00d99074: MOVAPS XMM7,xmmword ptr [ESP + 0x50]
                    cpu.x[7] = load_vec(e, cpu.esp.wrapping_add(0x50));
                    // 00d99079: FST float ptr [ESP + 0xb4]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xb4), x87_store32(cpu.st(0)));
                    // 00d99080: FST float ptr [ESP + 0xb8]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xb8), x87_store32(cpu.st(0)));
                    // 00d99087: PXOR XMM0,XMM0
                    cpu.x[0] = [0.0; 4];
                    // 00d9908b: FST float ptr [ESP + 0xbc]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xbc), x87_store32(cpu.st(0)));
                    // 00d99092: MOVDQA XMM2,XMM0
                    cpu.x[2] = cpu.x[0];
                    // 00d99096: FST float ptr [ESP + 0xd0]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xd0), x87_store32(cpu.st(0)));
                    // 00d9909d: MOVAPS XMM0,XMM6
                    cpu.x[0] = cpu.x[6];
                    // 00d990a0: FST float ptr [ESP + 0xd4]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xd4), x87_store32(cpu.st(0)));
                    // 00d990a7: CMPLEPS XMM0,XMM5
                    cpu.x[0] = cmpleps(cpu.x[0], cpu.x[5]);
                    // 00d990ab: FST float ptr [ESP + 0xd8]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xd8), x87_store32(cpu.st(0)));
                    // 00d990b2: MOVAPS xmmword ptr [ESP + 0x80],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0x80), cpu.x[0]);
                    // 00d990ba: FSTP float ptr [ESP + 0xdc]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xdc), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d990c1: CMPEQPS XMM6,XMM5
                    cpu.x[6] = cmpeqps(cpu.x[6], cpu.x[5]);
                    // 00d990c5: MOVAPS XMM0,XMM6
                    cpu.x[0] = cpu.x[6];
                    // 00d990c8: FST float ptr [ESP + 0x40]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x40), x87_store32(cpu.st(0)));
                    // 00d990cc: AND EDI,0x7fffffff
                    cpu.edi = cpu.and32(cpu.edi, 0x7fffffff);
                    // 00d990d2: FST float ptr [ESP + 0x44]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x44), x87_store32(cpu.st(0)));
                    // 00d990d6: SHL EDI,0x4
                    cpu.edi = cpu.shl32(cpu.edi, 0x4);
                    // 00d990d9: FST float ptr [ESP + 0x48]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x48), x87_store32(cpu.st(0)));
                    // 00d990dd: MOVDQA xmmword ptr [ESP + 0x170],XMM2
                    store_vec(e, cpu.esp.wrapping_add(0x170), cpu.x[2]);
                    // 00d990e6: FSTP float ptr [ESP + 0x4c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x4c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d990ea: MOVAPS xmmword ptr [ESP + 0x180],XMM6
                    store_vec(e, cpu.esp.wrapping_add(0x180), cpu.x[6]);
                    // 00d990f2: MOVAPS XMM1,xmmword ptr [ESP + 0x40]
                    cpu.x[1] = load_vec(e, cpu.esp.wrapping_add(0x40));
                    // 00d990f7: ANDPS XMM0,XMM1
                    cpu.x[0] = andps(cpu.x[0], cpu.x[1]);
                    // 00d990fa: FST float ptr [ESP + 0x90]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x90), x87_store32(cpu.st(0)));
                    // 00d99101: FST float ptr [ESP + 0x94]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x94), x87_store32(cpu.st(0)));
                    // 00d99108: MOVAPS xmmword ptr [ESP + 0x120],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0x120), cpu.x[0]);
                    // 00d99110: FST float ptr [ESP + 0x98]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x98), x87_store32(cpu.st(0)));
                    // 00d99117: MOVAPS XMM0,XMM6
                    cpu.x[0] = cpu.x[6];
                    // 00d9911a: FSTP float ptr [ESP + 0x9c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x9c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d99121: LEA EDI,[EDI + EDX*0x1 + 0xe]
                    cpu.edi = cpu.edi.wrapping_add(cpu.edx).wrapping_add(0xe);
                    // 00d99125: MOVAPS XMM1,xmmword ptr [ESP + 0x90]
                    cpu.x[1] = load_vec(e, cpu.esp.wrapping_add(0x90));
                    // 00d9912d: ANDPS XMM0,XMM1
                    cpu.x[0] = andps(cpu.x[0], cpu.x[1]);
                    // 00d99130: MOVAPS xmmword ptr [ESP + 0x130],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0x130), cpu.x[0]);
                    // 00d99138: MOVSS XMM0,dword ptr [ESP + 0x20]
                    cpu.x[0] = scalar_vec(e, cpu.esp.wrapping_add(0x20));
                    // 00d9913e: SHUFPS XMM0,XMM0,0x0
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[0], 0);
                    // 00d99142: MOVAPS xmmword ptr [ESP + 0x160],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0x160), cpu.x[0]);
                    // 00d9914a: MOVSS XMM0,dword ptr [ESP + 0x18]
                    cpu.x[0] = scalar_vec(e, cpu.esp.wrapping_add(0x18));
                    // 00d99150: SHUFPS XMM0,XMM0,0x0
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[0], 0);
                    // 00d99154: MOVAPS xmmword ptr [ESP + 0x140],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0x140), cpu.x[0]);
                    // 00d9915c: JMP 0x00d992c1
                    pc = 0x00d992c1;
                    continue 'run;
                }
                0x00d99161 => {
                    // 00d99161: FLD float ptr [ESP + EDI*0x4 + 0x110]
                    cpu.st_push(x87_load32(
                        e.mem.u32(
                            cpu.esp
                                .wrapping_add(cpu.edi.wrapping_mul(4))
                                .wrapping_add(0x110),
                        ),
                    ));
                    // 00d99168: MOV dword ptr [ESP + 0x10],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x10), cpu.edx);
                    // 00d9916c: FILD dword ptr [ESP + 0x10]
                    cpu.st_push(f64::from(e.mem.i32(cpu.esp.wrapping_add(0x10))));
                    // 00d99170: SHR EBX,0x10
                    cpu.ebx = cpu.shr32(cpu.ebx, 0x10);
                    // 00d99173: MOV dword ptr [ESP + 0x10],EBX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x10), cpu.ebx);
                    // 00d99177: MOVAPS XMM1,XMM6
                    cpu.x[1] = cpu.x[6];
                    // 00d9917a: FMULP
                    {
                        let v = x87_mul(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00d9917c: CMPLTPS XMM1,XMM0
                    cpu.x[1] = cmpltps(cpu.x[1], cpu.x[0]);
                    // 00d99180: FADD float ptr [ESP + EDI*0x4 + 0x100]
                    {
                        let v = x87_add(
                            cpu.st(0),
                            x87_m32(
                                e.mem.u32(
                                    cpu.esp
                                        .wrapping_add(cpu.edi.wrapping_mul(4))
                                        .wrapping_add(0x100),
                                ),
                            ),
                        );
                        cpu.set_st(0, v);
                    }
                    // 00d99187: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d9918b: FILD dword ptr [ESP + 0x10]
                    cpu.st_push(f64::from(e.mem.i32(cpu.esp.wrapping_add(0x10))));
                    // 00d9918f: FMUL float ptr [ESP + EDI*0x4 + 0x110]
                    {
                        let v = x87_mul(
                            cpu.st(0),
                            x87_m32(
                                e.mem.u32(
                                    cpu.esp
                                        .wrapping_add(cpu.edi.wrapping_mul(4))
                                        .wrapping_add(0x110),
                                ),
                            ),
                        );
                        cpu.set_st(0, v);
                    }
                    // 00d99196: FADD float ptr [ESP + EDI*0x4 + 0x100]
                    {
                        let v = x87_add(
                            cpu.st(0),
                            x87_m32(
                                e.mem.u32(
                                    cpu.esp
                                        .wrapping_add(cpu.edi.wrapping_mul(4))
                                        .wrapping_add(0x100),
                                ),
                            ),
                        );
                        cpu.set_st(0, v);
                    }
                    // 00d9919d: MOV EDI,EAX
                    cpu.edi = cpu.eax;
                    // 00d9919f: MOVMSKPS EAX,XMM1
                    cpu.eax = movmskps(cpu.x[1]);
                    // 00d991a2: AND EAX,EDI
                    cpu.eax = cpu.and32(cpu.eax, cpu.edi);
                    // 00d991a4: FSTP float ptr [ESP + 0x28]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x28), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d991a8: CMP EAX,EDI
                    cpu.cmp32(cpu.eax, cpu.edi);
                    // 00d991aa: JNZ 0x00d991b2
                    if !cpu.zf {
                        pc = 0x00d991b2;
                        continue 'run;
                    }
                    // 00d991ac: MOV EDX,ESI
                    cpu.edx = cpu.esi;
                    // 00d991ae: MOV ESI,ECX
                    cpu.esi = cpu.ecx;
                    // 00d991b0: JMP 0x00d991cc
                    pc = 0x00d991cc;
                    continue 'run;
                }
                0x00d991b2 => {
                    // 00d991b2: FLD float ptr [ESP + 0x14]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x14))));
                    // 00d991b6: MOV EDX,ECX
                    cpu.edx = cpu.ecx;
                    // 00d991b8: FSTP float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x10), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d991bc: FLD float ptr [ESP + 0x28]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x28))));
                    // 00d991c0: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d991c4: FLD float ptr [ESP + 0x10]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x10))));
                    // 00d991c8: FSTP float ptr [ESP + 0x28]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x28), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    pc = 0x00d991cc;
                    continue 'run;
                }
                0x00d991cc => {
                    // 00d991cc: FLD float ptr [ESP + 0x28]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x28))));
                    // 00d991d0: FCOMP ST2
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(2))) << 8;
                    cpu.st_pop();
                    // 00d991d2: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00d991d4: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00d991d7: JNP 0x00d992a0
                    if !cpu.pf {
                        pc = 0x00d992a0;
                        continue 'run;
                    }
                    // 00d991dd: FLD float ptr [ESP + 0x14]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x14))));
                    // 00d991e1: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00d991e5: FCOMP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    // 00d991e7: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00d991e9: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00d991ec: JP 0x00d991fc
                    if cpu.pf {
                        pc = 0x00d991fc;
                        continue 'run;
                    }
                    // 00d991ee: MOV ESI,EDX
                    cpu.esi = cpu.edx;
                    // 00d991f0: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00d991f2: MOV EDI,dword ptr [ESI]
                    cpu.edi = e.mem.u32(cpu.esi);
                    // 00d991f4: MOV EBX,dword ptr [ESI + 0x4]
                    cpu.ebx = e.mem.u32(cpu.esi.wrapping_add(0x4));
                    // 00d991f7: JMP 0x00d98f25
                    pc = 0x00d98f25;
                    continue 'run;
                }
                0x00d991fc => {
                    // 00d991fc: FSTP ST5
                    {
                        let v = cpu.st(0);
                        cpu.set_st(5, v);
                    }
                    cpu.st_pop();
                    // 00d991fe: FSTP ST3
                    {
                        let v = cpu.st(0);
                        cpu.set_st(3, v);
                    }
                    cpu.st_pop();
                    // 00d99200: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00d99202: FCOM ST2
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(2))) << 8;
                    // 00d99204: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00d99206: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00d99209: JP 0x00d9920f
                    if cpu.pf {
                        pc = 0x00d9920f;
                        continue 'run;
                    }
                    // 00d9920b: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00d9920d: JMP 0x00d99211
                    pc = 0x00d99211;
                    continue 'run;
                }
                0x00d9920f => {
                    // 00d9920f: FSTP ST2
                    {
                        let v = cpu.st(0);
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    pc = 0x00d99211;
                    continue 'run;
                }
                0x00d99211 => {
                    // 00d99211: MOV EAX,dword ptr [ESP + 0x24]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x24));
                    // 00d99215: FXCH
                    cpu.st_exchange(1);
                    // 00d99217: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d9921b: MOV dword ptr [EAX],ESI
                    e.mem.set_u32(cpu.eax, cpu.esi);
                    // 00d9921d: FLD float ptr [ESP + 0x14]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x14))));
                    // 00d99221: ADD EAX,0x10
                    cpu.eax = cpu.add32(cpu.eax, 0x10);
                    // 00d99224: FSTP float ptr [EAX + -0xc]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0xc), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d99227: MOV dword ptr [ESP + 0x24],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x24), cpu.eax);
                    // 00d9922b: MOV ESI,EDX
                    cpu.esi = cpu.edx;
                    // 00d9922d: FST float ptr [EAX + -0x8]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0x8), x87_store32(cpu.st(0)));
                    // 00d99230: FLD float ptr [ESP + 0x28]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x28))));
                    // 00d99234: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00d99236: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00d99238: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00d9923a: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00d9923d: JP 0x00d99245
                    if cpu.pf {
                        pc = 0x00d99245;
                        continue 'run;
                    }
                    // 00d9923f: FSTP float ptr [ESP + 0x2c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x2c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d99243: JMP 0x00d99247
                    pc = 0x00d99247;
                    continue 'run;
                }
                0x00d99245 => {
                    // 00d99245: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00d99247;
                    continue 'run;
                }
                0x00d99247 => {
                    // 00d99247: MOV ECX,dword ptr [EBP + 0x18]
                    cpu.ecx = e.mem.u32(cpu.ebp.wrapping_add(0x18));
                    // 00d9924a: MOV EDX,dword ptr [ECX]
                    cpu.edx = e.mem.u32(cpu.ecx);
                    // 00d9924c: MOV EAX,dword ptr [EDX + 0xc]
                    cpu.eax = e.mem.u32(cpu.edx.wrapping_add(0xc));
                    // 00d9924f: CALL EAX
                    {
                        let r = e.call(cpu.eax, &args![cpu.ecx]);
                        cpu.st_push(r.st0);
                    }
                    // 00d99251: FMUL float ptr [ESP + 0x34]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x34))));
                        cpu.set_st(0, v);
                    }
                    // 00d99255: MOVAPS XMM6,xmmword ptr [ESP + 0xa0]
                    cpu.x[6] = load_vec(e, cpu.esp.wrapping_add(0xa0));
                    // 00d9925d: FSTP float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x10), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d99261: FLD float ptr [ESP + 0x10]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x10))));
                    // 00d99265: FLD float ptr [ESP + 0x2c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x2c))));
                    // 00d99269: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00d9926b: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00d9926d: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00d99270: JNZ 0x00d9927a
                    if !cpu.zf {
                        pc = 0x00d9927a;
                        continue 'run;
                    }
                    // 00d99272: FXCH
                    cpu.st_exchange(1);
                    // 00d99274: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d99278: JMP 0x00d99280
                    pc = 0x00d99280;
                    continue 'run;
                }
                0x00d9927a => {
                    // 00d9927a: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00d9927c: FST float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    pc = 0x00d99280;
                    continue 'run;
                }
                0x00d99280 => {
                    // 00d99280: FLD float ptr [0x0104da48]
                    cpu.st_push(x87_load32(e.mem.u32(0x0104da48)));
                    // 00d99286: FLD float ptr [0x0104e148]
                    cpu.st_push(x87_load32(e.mem.u32(0x0104e148)));
                    // 00d9928c: FLD float ptr [0x010cd644]
                    cpu.st_push(x87_load32(e.mem.u32(0x010cd644)));
                    // 00d99292: FLD float ptr [ESP + 0x20]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x20))));
                    // 00d99296: FXCH
                    cpu.st_exchange(1);
                    // 00d99298: FXCH ST4
                    cpu.st_exchange(4);
                    // 00d9929a: FXCH ST2
                    cpu.st_exchange(2);
                    // 00d9929c: FXCH ST3
                    cpu.st_exchange(3);
                    // 00d9929e: FXCH ST2
                    cpu.st_exchange(2);
                    pc = 0x00d992a0;
                    continue 'run;
                }
                0x00d992a0 => {
                    // 00d992a0: MOV EDI,dword ptr [ESI]
                    cpu.edi = e.mem.u32(cpu.esi);
                    // 00d992a2: MOV EBX,dword ptr [ESI + 0x4]
                    cpu.ebx = e.mem.u32(cpu.esi.wrapping_add(0x4));
                    // 00d992a5: JMP 0x00d98f25
                    pc = 0x00d98f25;
                    continue 'run;
                }
                0x00d992b0 => {
                    // 00d992b0: MOVAPS XMM5,xmmword ptr [ESP + 0xf0]
                    cpu.x[5] = load_vec(e, cpu.esp.wrapping_add(0xf0));
                    // 00d992b8: MOVDQA XMM2,xmmword ptr [ESP + 0x170]
                    cpu.x[2] = load_vec(e, cpu.esp.wrapping_add(0x170));
                    pc = 0x00d992c1;
                    continue 'run;
                }
                0x00d992c1 => {
                    // 00d992c1: MOV AX,word ptr [EDI + -0x8]
                    cpu.eax = (cpu.eax & 0xffff_0000)
                        | (u32::from(e.mem.u16(cpu.edi.wrapping_sub(0x8))) & 0xffff);
                    // 00d992c5: MOV CX,word ptr [EDI]
                    cpu.ecx = (cpu.ecx & 0xffff_0000) | (u32::from(e.mem.u16(cpu.edi)) & 0xffff);
                    // 00d992c8: MOV EDX,dword ptr [EDI + -0xe]
                    cpu.edx = e.mem.u32(cpu.edi.wrapping_sub(0xe));
                    // 00d992cb: MOVAPS XMM4,xmmword ptr [ESP + 0xe0]
                    cpu.x[4] = load_vec(e, cpu.esp.wrapping_add(0xe0));
                    // 00d992d3: MULPS XMM4,xmmword ptr [0x011b6de0]
                    cpu.x[4] = mulps(cpu.x[4], load_vec(e, 0x011b6de0));
                    // 00d992da: MOV dword ptr [ESP + 0xc0],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xc0), cpu.edx);
                    // 00d992e1: MOV EDX,dword ptr [EDI + -0x2]
                    cpu.edx = e.mem.u32(cpu.edi.wrapping_sub(0x2));
                    // 00d992e4: MOV word ptr [ESP + 0x30],AX
                    e.mem
                        .set_u16(cpu.esp.wrapping_add(0x30), (cpu.eax & 0xffff) as u16);
                    // 00d992e9: MOV EAX,dword ptr [EDI + -0xa]
                    cpu.eax = e.mem.u32(cpu.edi.wrapping_sub(0xa));
                    // 00d992ec: MOV word ptr [ESP + 0x32],CX
                    e.mem
                        .set_u16(cpu.esp.wrapping_add(0x32), (cpu.ecx & 0xffff) as u16);
                    // 00d992f1: MOV ECX,dword ptr [EDI + -0x6]
                    cpu.ecx = e.mem.u32(cpu.edi.wrapping_sub(0x6));
                    // 00d992f4: MOV dword ptr [ESP + 0xc4],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xc4), cpu.eax);
                    // 00d992fb: MOV dword ptr [ESP + 0xc8],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xc8), cpu.ecx);
                    // 00d99302: MOV dword ptr [ESP + 0xcc],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xcc), cpu.edx);
                    // 00d99309: MOVDQA XMM1,xmmword ptr [ESP + 0xc0]
                    cpu.x[1] = load_vec(e, cpu.esp.wrapping_add(0xc0));
                    // 00d99312: MOVDQA XMM0,XMM1
                    cpu.x[0] = cpu.x[1];
                    // 00d99316: PUNPCKHWD XMM1,XMM2
                    cpu.x[1] = from_lane_bits(punpckhwd(lane_bits(cpu.x[1]), lane_bits(cpu.x[2])));
                    // 00d9931a: PUNPCKLWD XMM0,XMM2
                    cpu.x[0] = from_lane_bits(punpcklwd(lane_bits(cpu.x[0]), lane_bits(cpu.x[2])));
                    // 00d9931e: MOVDQA XMM2,XMM0
                    cpu.x[2] = cpu.x[0];
                    // 00d99322: MOVDQA XMM3,XMM1
                    cpu.x[3] = cpu.x[1];
                    // 00d99326: PSLLD XMM0,0x10
                    cpu.x[0] = from_lane_bits(pslld(lane_bits(cpu.x[0]), 16));
                    // 00d9932b: PSLLD XMM1,0x10
                    cpu.x[1] = from_lane_bits(pslld(lane_bits(cpu.x[1]), 16));
                    // 00d99330: PSRLD XMM0,0x10
                    cpu.x[0] = from_lane_bits(psrld(lane_bits(cpu.x[0]), 16));
                    // 00d99335: CVTDQ2PS XMM0,XMM0
                    cpu.x[0] = cvtdq2ps(lane_bits(cpu.x[0]));
                    // 00d99338: PSRLD XMM1,0x10
                    cpu.x[1] = from_lane_bits(psrld(lane_bits(cpu.x[1]), 16));
                    // 00d9933d: CVTDQ2PS XMM1,XMM1
                    cpu.x[1] = cvtdq2ps(lane_bits(cpu.x[1]));
                    // 00d99340: PSRLD XMM2,0x10
                    cpu.x[2] = from_lane_bits(psrld(lane_bits(cpu.x[2]), 16));
                    // 00d99345: PSRLD XMM3,0x10
                    cpu.x[3] = from_lane_bits(psrld(lane_bits(cpu.x[3]), 16));
                    // 00d9934a: CVTDQ2PS XMM2,XMM2
                    cpu.x[2] = cvtdq2ps(lane_bits(cpu.x[2]));
                    // 00d9934d: MULPS XMM2,xmmword ptr [ESP + 0xb0]
                    cpu.x[2] = mulps(cpu.x[2], load_vec(e, cpu.esp.wrapping_add(0xb0)));
                    // 00d99355: ADDPS XMM2,XMM0
                    cpu.x[2] = addps(cpu.x[2], cpu.x[0]);
                    // 00d99358: MOVAPS XMM0,xmmword ptr [ESP + 0x70]
                    cpu.x[0] = load_vec(e, cpu.esp.wrapping_add(0x70));
                    // 00d9935d: CVTDQ2PS XMM3,XMM3
                    cpu.x[3] = cvtdq2ps(lane_bits(cpu.x[3]));
                    // 00d99360: MULPS XMM3,xmmword ptr [ESP + 0xd0]
                    cpu.x[3] = mulps(cpu.x[3], load_vec(e, cpu.esp.wrapping_add(0xd0)));
                    // 00d99368: ADDPS XMM3,XMM1
                    cpu.x[3] = addps(cpu.x[3], cpu.x[1]);
                    // 00d9936b: MOVAPS XMM1,xmmword ptr [ESP + 0x80]
                    cpu.x[1] = load_vec(e, cpu.esp.wrapping_add(0x80));
                    // 00d99373: MULPS XMM2,XMM4
                    cpu.x[2] = mulps(cpu.x[2], cpu.x[4]);
                    // 00d99376: ADDPS XMM2,XMM0
                    cpu.x[2] = addps(cpu.x[2], cpu.x[0]);
                    // 00d99379: MULPS XMM3,XMM4
                    cpu.x[3] = mulps(cpu.x[3], cpu.x[4]);
                    // 00d9937c: ADDPS XMM3,XMM0
                    cpu.x[3] = addps(cpu.x[3], cpu.x[0]);
                    // 00d9937f: MOVAPS XMM4,XMM2
                    cpu.x[4] = cpu.x[2];
                    // 00d99382: SUBPS XMM4,XMM7
                    cpu.x[4] = subps(cpu.x[4], cpu.x[7]);
                    // 00d99385: MULPS XMM4,XMM5
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[5]);
                    // 00d99388: MOVAPS XMM5,XMM3
                    cpu.x[5] = cpu.x[3];
                    // 00d9938b: SUBPS XMM5,XMM7
                    cpu.x[5] = subps(cpu.x[5], cpu.x[7]);
                    // 00d9938e: MULPS XMM5,xmmword ptr [ESP + 0xf0]
                    cpu.x[5] = mulps(cpu.x[5], load_vec(e, cpu.esp.wrapping_add(0xf0)));
                    // 00d99396: MOVAPS XMM7,XMM1
                    cpu.x[7] = cpu.x[1];
                    // 00d99399: ANDNPS XMM7,XMM5
                    cpu.x[7] = andnps(cpu.x[7], cpu.x[5]);
                    // 00d9939c: ANDPS XMM1,XMM4
                    cpu.x[1] = andps(cpu.x[1], cpu.x[4]);
                    // 00d9939f: ORPS XMM7,XMM1
                    cpu.x[7] = orps(cpu.x[7], cpu.x[1]);
                    // 00d993a2: MOVAPS XMM0,XMM6
                    cpu.x[0] = cpu.x[6];
                    // 00d993a5: ANDNPS XMM0,XMM7
                    cpu.x[0] = andnps(cpu.x[0], cpu.x[7]);
                    // 00d993a8: MOVAPS XMM7,xmmword ptr [ESP + 0x80]
                    cpu.x[7] = load_vec(e, cpu.esp.wrapping_add(0x80));
                    // 00d993b0: ORPS XMM0,xmmword ptr [ESP + 0x120]
                    cpu.x[0] = orps(cpu.x[0], load_vec(e, cpu.esp.wrapping_add(0x120)));
                    // 00d993b8: ANDPS XMM7,XMM5
                    cpu.x[7] = andps(cpu.x[7], cpu.x[5]);
                    // 00d993bb: MOVAPS XMM5,xmmword ptr [ESP + 0x80]
                    cpu.x[5] = load_vec(e, cpu.esp.wrapping_add(0x80));
                    // 00d993c3: ANDNPS XMM5,XMM4
                    cpu.x[5] = andnps(cpu.x[5], cpu.x[4]);
                    // 00d993c6: ORPS XMM7,XMM5
                    cpu.x[7] = orps(cpu.x[7], cpu.x[5]);
                    // 00d993c9: MOVAPS XMM1,XMM6
                    cpu.x[1] = cpu.x[6];
                    // 00d993cc: ANDNPS XMM1,XMM7
                    cpu.x[1] = andnps(cpu.x[1], cpu.x[7]);
                    // 00d993cf: ORPS XMM1,xmmword ptr [ESP + 0x130]
                    cpu.x[1] = orps(cpu.x[1], load_vec(e, cpu.esp.wrapping_add(0x130)));
                    // 00d993d7: MOVAPS XMM5,XMM1
                    cpu.x[5] = cpu.x[1];
                    // 00d993da: SHUFPS XMM5,XMM1,0x55
                    cpu.x[5] = shufps(cpu.x[5], cpu.x[1], 85);
                    // 00d993de: MOVAPS XMM7,XMM1
                    cpu.x[7] = cpu.x[1];
                    // 00d993e1: SHUFPS XMM7,XMM1,0x0
                    cpu.x[7] = shufps(cpu.x[7], cpu.x[1], 0);
                    // 00d993e5: MINPS XMM7,XMM5
                    cpu.x[7] = minps(cpu.x[7], cpu.x[5]);
                    // 00d993e8: SHUFPS XMM1,XMM1,0xaa
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[1], 170);
                    // 00d993ec: MINPS XMM7,XMM1
                    cpu.x[7] = minps(cpu.x[7], cpu.x[1]);
                    // 00d993ef: MOVAPS XMM1,xmmword ptr [ESP + 0x140]
                    cpu.x[1] = load_vec(e, cpu.esp.wrapping_add(0x140));
                    // 00d993f7: MINPS XMM1,XMM7
                    cpu.x[1] = minps(cpu.x[1], cpu.x[7]);
                    // 00d993fa: MOVAPS XMM5,XMM0
                    cpu.x[5] = cpu.x[0];
                    // 00d993fd: MOVAPS XMM7,XMM0
                    cpu.x[7] = cpu.x[0];
                    // 00d99400: SHUFPS XMM7,XMM0,0x0
                    cpu.x[7] = shufps(cpu.x[7], cpu.x[0], 0);
                    // 00d99404: SHUFPS XMM5,XMM0,0x55
                    cpu.x[5] = shufps(cpu.x[5], cpu.x[0], 85);
                    // 00d99408: SHUFPS XMM0,XMM0,0xaa
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[0], 170);
                    // 00d9940c: MAXPS XMM7,XMM5
                    cpu.x[7] = maxps(cpu.x[7], cpu.x[5]);
                    // 00d9940f: MAXPS XMM7,XMM0
                    cpu.x[7] = maxps(cpu.x[7], cpu.x[0]);
                    // 00d99412: MOVAPS XMM0,xmmword ptr [ESP + 0x160]
                    cpu.x[0] = load_vec(e, cpu.esp.wrapping_add(0x160));
                    // 00d9941a: MAXPS XMM0,XMM7
                    cpu.x[0] = maxps(cpu.x[0], cpu.x[7]);
                    // 00d9941d: MOVAPS XMM4,XMM6
                    cpu.x[4] = cpu.x[6];
                    // 00d99420: CMPLTPS XMM0,XMM1
                    cpu.x[0] = cmpltps(cpu.x[0], cpu.x[1]);
                    // 00d99424: MOVAPS XMM7,xmmword ptr [ESP + 0x50]
                    cpu.x[7] = load_vec(e, cpu.esp.wrapping_add(0x50));
                    // 00d99429: ANDNPS XMM4,XMM0
                    cpu.x[4] = andnps(cpu.x[4], cpu.x[0]);
                    // 00d9942c: MOVAPS XMM0,XMM7
                    cpu.x[0] = cpu.x[7];
                    // 00d9942f: CMPLEPS XMM0,XMM3
                    cpu.x[0] = cmpleps(cpu.x[0], cpu.x[3]);
                    // 00d99433: CMPLEPS XMM2,XMM7
                    cpu.x[2] = cmpleps(cpu.x[2], cpu.x[7]);
                    // 00d99437: ANDPS XMM0,XMM2
                    cpu.x[0] = andps(cpu.x[0], cpu.x[2]);
                    // 00d9943a: ANDPS XMM0,XMM6
                    cpu.x[0] = andps(cpu.x[0], cpu.x[6]);
                    // 00d9943d: ORPS XMM4,XMM0
                    cpu.x[4] = orps(cpu.x[4], cpu.x[0]);
                    // 00d99440: MOVMSKPS EAX,XMM4
                    cpu.eax = movmskps(cpu.x[4]);
                    // 00d99443: AND EAX,0x7
                    cpu.eax = cpu.and32(cpu.eax, 0x7);
                    // 00d99446: CMP AL,0x7
                    cpu.cmp8(cpu.eax & 0xff, 0x7);
                    // 00d99448: JNZ 0x00d9946b
                    if !cpu.zf {
                        pc = 0x00d9946b;
                        continue 'run;
                    }
                    // 00d9944a: MOV EAX,dword ptr [ESP + 0x30]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x30));
                    // 00d9944e: MOV ECX,dword ptr [EBP + 0x18]
                    cpu.ecx = e.mem.u32(cpu.ebp.wrapping_add(0x18));
                    // 00d99451: MOV EDX,dword ptr [ECX]
                    cpu.edx = e.mem.u32(cpu.ecx);
                    // 00d99453: MOV EDX,dword ptr [EDX + 0x4]
                    cpu.edx = e.mem.u32(cpu.edx.wrapping_add(0x4));
                    // 00d99456: PUSH EAX
                    cpu.push(e, cpu.eax);
                    // 00d99457: LEA EAX,[ESP + 0x3f]
                    cpu.eax = cpu.esp.wrapping_add(0x3f);
                    // 00d9945b: PUSH EAX
                    cpu.push(e, cpu.eax);
                    // 00d9945c: CALL EDX
                    e.call(
                        cpu.edx,
                        &args![
                            cpu.ecx,
                            e.mem.u32(cpu.esp.wrapping_add(0)),
                            e.mem.u32(cpu.esp.wrapping_add(4))
                        ],
                    );
                    cpu.esp = cpu.esp.wrapping_add(8);
                    // 00d9945e: MOVAPS XMM6,xmmword ptr [ESP + 0x180]
                    cpu.x[6] = load_vec(e, cpu.esp.wrapping_add(0x180));
                    // 00d99466: MOVAPS XMM7,xmmword ptr [ESP + 0x50]
                    cpu.x[7] = load_vec(e, cpu.esp.wrapping_add(0x50));
                    pc = 0x00d9946b;
                    continue 'run;
                }
                0x00d9946b => {
                    // 00d9946b: ADD EDI,0x10
                    cpu.edi = cpu.add32(cpu.edi, 0x10);
                    // 00d9946e: SUB EBX,0x1
                    cpu.ebx = cpu.sub32(cpu.ebx, 0x1);
                    // 00d99471: JNZ 0x00d992b0
                    if !cpu.zf {
                        pc = 0x00d992b0;
                        continue 'run;
                    }
                    // 00d99477: JMP 0x00d9948e
                    pc = 0x00d9948e;
                    continue 'run;
                }
                0x00d99480 => {
                    // 00d99480: FSTP ST4
                    {
                        let v = cpu.st(0);
                        cpu.set_st(4, v);
                    }
                    cpu.st_pop();
                    // 00d99482: FSTP ST3
                    {
                        let v = cpu.st(0);
                        cpu.set_st(3, v);
                    }
                    cpu.st_pop();
                    pc = 0x00d99484;
                    continue 'run;
                }
                0x00d99484 => {
                    // 00d99484: FSTP ST2
                    {
                        let v = cpu.st(0);
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00d99486: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00d99488: JMP 0x00d9948c
                    pc = 0x00d9948c;
                    continue 'run;
                }
                0x00d9948a => {
                    // 00d9948a: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    pc = 0x00d9948c;
                    continue 'run;
                }
                0x00d9948c => {
                    // 00d9948c: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00d9948e;
                    continue 'run;
                }
                0x00d9948e => {
                    // 00d9948e: MOV EAX,dword ptr [ESP + 0x24]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x24));
                    // 00d99492: CMP EAX,dword ptr [ESP + 0x3c]
                    cpu.cmp32(cpu.eax, e.mem.u32(cpu.esp.wrapping_add(0x3c)));
                    // 00d99496: JZ 0x00d9951c
                    if cpu.zf {
                        pc = 0x00d9951c;
                        continue 'run;
                    }
                    // 00d9949c: FLD float ptr [EAX + -0x8]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.eax.wrapping_sub(0x8))));
                    // 00d9949f: MOV ESI,dword ptr [EAX + -0x10]
                    cpu.esi = e.mem.u32(cpu.eax.wrapping_sub(0x10));
                    // 00d994a2: MOV ECX,dword ptr [EBP + 0x18]
                    cpu.ecx = e.mem.u32(cpu.ebp.wrapping_add(0x18));
                    // 00d994a5: FSTP float ptr [ESP + 0x2c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x2c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d994a9: FLD float ptr [EAX + -0xc]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.eax.wrapping_sub(0xc))));
                    // 00d994ac: MOV EDI,dword ptr [ESI]
                    cpu.edi = e.mem.u32(cpu.esi);
                    // 00d994ae: MOV EBX,dword ptr [ESI + 0x4]
                    cpu.ebx = e.mem.u32(cpu.esi.wrapping_add(0x4));
                    // 00d994b1: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x20), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d994b5: SUB EAX,0x10
                    cpu.eax = cpu.sub32(cpu.eax, 0x10);
                    // 00d994b8: MOV dword ptr [ESP + 0x24],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x24), cpu.eax);
                    // 00d994bc: MOV EAX,dword ptr [ECX]
                    cpu.eax = e.mem.u32(cpu.ecx);
                    // 00d994be: MOV EDX,dword ptr [EAX + 0xc]
                    cpu.edx = e.mem.u32(cpu.eax.wrapping_add(0xc));
                    // 00d994c1: CALL EDX
                    {
                        let r = e.call(cpu.edx, &args![cpu.ecx]);
                        cpu.st_push(r.st0);
                    }
                    // 00d994c3: FMUL float ptr [ESP + 0x34]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x34))));
                        cpu.set_st(0, v);
                    }
                    // 00d994c7: FSTP float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x10), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d994cb: FLD float ptr [ESP + 0x10]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x10))));
                    // 00d994cf: FLD float ptr [ESP + 0x2c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x2c))));
                    // 00d994d3: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00d994d5: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00d994d7: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00d994da: JNZ 0x00d994e4
                    if !cpu.zf {
                        pc = 0x00d994e4;
                        continue 'run;
                    }
                    // 00d994dc: FXCH
                    cpu.st_exchange(1);
                    // 00d994de: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d994e2: JMP 0x00d994ea
                    pc = 0x00d994ea;
                    continue 'run;
                }
                0x00d994e4 => {
                    // 00d994e4: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00d994e6: FST float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    pc = 0x00d994ea;
                    continue 'run;
                }
                0x00d994ea => {
                    // 00d994ea: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00d994ee: FLD float ptr [ESP + 0x20]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x20))));
                    // 00d994f2: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00d994f4: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00d994f6: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00d994f8: TEST AH,0x1
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x1);
                    // 00d994fb: JZ 0x00d9948a
                    if cpu.zf {
                        pc = 0x00d9948a;
                        continue 'run;
                    }
                    // 00d994fd: FLD float ptr [0x0104da48]
                    cpu.st_push(x87_load32(e.mem.u32(0x0104da48)));
                    // 00d99503: MOVAPS XMM6,xmmword ptr [ESP + 0xa0]
                    cpu.x[6] = load_vec(e, cpu.esp.wrapping_add(0xa0));
                    // 00d9950b: FLD float ptr [0x0104e148]
                    cpu.st_push(x87_load32(e.mem.u32(0x0104e148)));
                    // 00d99511: FLD float ptr [0x010cd644]
                    cpu.st_push(x87_load32(e.mem.u32(0x010cd644)));
                    // 00d99517: JMP 0x00d98f1d
                    pc = 0x00d98f1d;
                    continue 'run;
                }
                0x00d9951c => {
                    // 00d9951c: CALL 0x00c85750
                    cpu.eax = e.call(0x00c85750, &args![]).u32();
                    // 00d99521: MOV ECX,dword ptr [ESP + 0x3c]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x3c));
                    // 00d99525: MOV ESI,dword ptr [EBP + 0x8]
                    cpu.esi = e.mem.u32(cpu.ebp.wrapping_add(0x8));
                    // 00d99528: MOV dword ptr [EAX],ECX
                    e.mem.set_u32(cpu.eax, cpu.ecx);
                    // 00d9952a: MOV ECX,dword ptr [EBP + 0x18]
                    cpu.ecx = e.mem.u32(cpu.ebp.wrapping_add(0x18));
                    // 00d9952d: MOV EDX,dword ptr [ECX]
                    cpu.edx = e.mem.u32(cpu.ecx);
                    // 00d9952f: MOV EAX,dword ptr [EDX + 0x8]
                    cpu.eax = e.mem.u32(cpu.edx.wrapping_add(0x8));
                    // 00d99532: PUSH ESI
                    cpu.push(e, cpu.esi);
                    // 00d99533: CALL EAX
                    e.call(cpu.eax, &args![cpu.ecx, e.mem.u32(cpu.esp.wrapping_add(0))]);
                    cpu.esp = cpu.esp.wrapping_add(4);
                    // 00d99535: POP EDI
                    cpu.edi = cpu.pop(e);
                    // 00d99536: MOV EAX,ESI
                    cpu.eax = cpu.esi;
                    // 00d99538: POP ESI
                    cpu.esi = cpu.pop(e);
                    // 00d99539: POP EBX
                    cpu.ebx = cpu.pop(e);
                    // 00d9953a: MOV ESP,EBP
                    cpu.esp = cpu.ebp;
                    // 00d9953c: POP EBP
                    cpu.ebp = cpu.pop(e);
                    // 00d9953d: RET
                    break 'run;
                }
                other => panic!("jump to {other:#x} outside the recovered function"),
            }
        }
        cpu.eax
    })
}

// Translated from 00d99570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The ray cast of four rays at once (a packet), through a packed tree with an explicit
/// stack: the sibling of `00d98ca0` (no Xbox PDB name). Cdecl `(done, query, from, to,
/// collector, mode)`; `query` is as in `00d98ca0` (the stack entries are 0x40 bytes here);
/// `from` and `to` hold the four rays' end points as three vectors each (the x, y and z of the
/// four rays), `mode` picks the 16-byte vector at `01268460 + 16 * mode`. The collector's slot
/// `+4` is told `(id, the four hit lanes)` for each box some ray crosses and its slot `+8` gets
/// the address of the result block at the end, where the byte at `done` is set to 1 and the
/// address is returned. The walk is the exe's code instruction by instruction on the `Cpu`
/// state above; `RCPPS` and `RSQRTPS` are the host's estimates.
pub fn fn_00d99570(
    e: &mut Engine,
    done: u32,
    query: u32,
    from: u32,
    to: u32,
    collector: u32,
    mode: u32,
) -> u32 {
    run_00d99570(e, [done, query, from, to, collector, mode], 0)
}

/// `00d99570` on a frame whose words start as `fill`.
fn run_00d99570(e: &mut Engine, args: [u32; 6], fill: u32) -> u32 {
    e.with_stack(0x400, |e, frame| {
        let top = frame.addr() + 0x3c0;
        for word in 0..0x100 {
            e.mem.set_u32(frame.addr() + 4 * word, fill);
        }
        let mut cpu = Cpu {
            esp: top,
            ..Cpu::default()
        };
        e.mem.set_u32(top, 0);
        for (i, arg) in args.iter().enumerate() {
            e.mem.set_u32(top + 4 + 4 * i as u32, *arg);
        }
        let mut pc: u32 = 0x00d9_9570;
        'run: loop {
            match pc {
                0x00d99570 => {
                    // 00d99570: PUSH EBP
                    cpu.push(e, cpu.ebp);
                    // 00d99571: MOV EBP,ESP
                    cpu.ebp = cpu.esp;
                    // 00d99573: AND ESP,0xfffffff0
                    cpu.esp = cpu.and32(cpu.esp, 0xfffffff0);
                    // 00d99576: SUB ESP,0x2f4
                    cpu.esp = cpu.sub32(cpu.esp, 0x2f4);
                    // 00d9957c: MOV EAX,dword ptr [EBP + 0xc]
                    cpu.eax = e.mem.u32(cpu.ebp.wrapping_add(0xc));
                    // 00d9957f: MOV ECX,dword ptr [EAX + 0x1c]
                    cpu.ecx = e.mem.u32(cpu.eax.wrapping_add(0x1c));
                    // 00d99582: MOV EDX,dword ptr [EAX + 0x30]
                    cpu.edx = e.mem.u32(cpu.eax.wrapping_add(0x30));
                    // 00d99585: MOVAPS XMM0,xmmword ptr [EAX + 0x40]
                    cpu.x[0] = load_vec(e, cpu.eax.wrapping_add(0x40));
                    // 00d99589: MOVAPS XMM1,xmmword ptr [EAX + 0x30]
                    cpu.x[1] = load_vec(e, cpu.eax.wrapping_add(0x30));
                    // 00d9958d: MOV dword ptr [ESP + 0xb0],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xb0), cpu.ecx);
                    // 00d99594: MOV ECX,dword ptr [EAX + 0x34]
                    cpu.ecx = e.mem.u32(cpu.eax.wrapping_add(0x34));
                    // 00d99597: PUSH EBX
                    cpu.push(e, cpu.ebx);
                    // 00d99598: MOV EBX,dword ptr [EAX]
                    cpu.ebx = e.mem.u32(cpu.eax);
                    // 00d9959a: MOV dword ptr [ESP + 0xb8],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xb8), cpu.edx);
                    // 00d995a1: MOV EDX,dword ptr [EAX + 0x38]
                    cpu.edx = e.mem.u32(cpu.eax.wrapping_add(0x38));
                    // 00d995a4: MOV dword ptr [ESP + 0xbc],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xbc), cpu.ecx);
                    // 00d995ab: MOV ECX,dword ptr [EAX + 0x3c]
                    cpu.ecx = e.mem.u32(cpu.eax.wrapping_add(0x3c));
                    // 00d995ae: MOV EAX,dword ptr [EAX + 0xc]
                    cpu.eax = e.mem.u32(cpu.eax.wrapping_add(0xc));
                    // 00d995b1: PUSH ESI
                    cpu.push(e, cpu.esi);
                    // 00d995b2: PUSH EDI
                    cpu.push(e, cpu.edi);
                    // 00d995b3: SUBPS XMM0,XMM1
                    cpu.x[0] = subps(cpu.x[0], cpu.x[1]);
                    // 00d995b6: MOV dword ptr [ESP + 0xc8],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xc8), cpu.edx);
                    // 00d995bd: MOV dword ptr [ESP + 0xcc],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xcc), cpu.ecx);
                    // 00d995c4: MOVAPS xmmword ptr [ESP + 0x160],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0x160), cpu.x[0]);
                    // 00d995cc: MOV EDI,EAX
                    cpu.edi = cpu.eax;
                    // 00d995ce: TEST EAX,EAX
                    cpu.test32(cpu.eax, cpu.eax);
                    // 00d995d0: JG 0x00d995d7
                    if !cpu.zf && cpu.sf == cpu.of {
                        pc = 0x00d995d7;
                        continue 'run;
                    }
                    // 00d995d2: MOV EDI,0x80
                    cpu.edi = 0x80;
                    pc = 0x00d995d7;
                    continue 'run;
                }
                0x00d995d7 => {
                    // 00d995d7: MOV EDX,dword ptr [EBX + 0x4]
                    cpu.edx = e.mem.u32(cpu.ebx.wrapping_add(0x4));
                    // 00d995da: MOV ESI,dword ptr [EBX]
                    cpu.esi = e.mem.u32(cpu.ebx);
                    // 00d995dc: MOV dword ptr [ESP + 0x54],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x54), cpu.edx);
                    // 00d995e0: CALL 0x00c85750
                    cpu.eax = e.call(0x00c85750, &args![]).u32();
                    // 00d995e5: FLD float ptr [0x010d4080]
                    cpu.st_push(x87_load32(e.mem.u32(0x010d4080)));
                    // 00d995eb: MOV ECX,dword ptr [EAX]
                    cpu.ecx = e.mem.u32(cpu.eax);
                    // 00d995ed: FST float ptr [ESP + 0x70]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x70), x87_store32(cpu.st(0)));
                    // 00d995f1: SHL EDI,0x6
                    cpu.edi = cpu.shl32(cpu.edi, 0x6);
                    // 00d995f4: FST float ptr [ESP + 0x74]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x74), x87_store32(cpu.st(0)));
                    // 00d995f8: ADD EDI,0x7f
                    cpu.edi = cpu.add32(cpu.edi, 0x7f);
                    // 00d995fb: AND EDI,0xffffff80
                    cpu.edi = cpu.and32(cpu.edi, 0xffffff80);
                    // 00d995fe: ADD EDI,ECX
                    cpu.edi = cpu.add32(cpu.edi, cpu.ecx);
                    // 00d99600: MOV dword ptr [EAX],EDI
                    e.mem.set_u32(cpu.eax, cpu.edi);
                    // 00d99602: MOV EAX,[0x01267e30]
                    cpu.eax = e.mem.u32(0x01267e30);
                    // 00d99607: MOV EDX,dword ptr [0x01267e38]
                    cpu.edx = e.mem.u32(0x01267e38);
                    // 00d9960d: MOV dword ptr [ESP + 0xa0],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xa0), cpu.eax);
                    // 00d99614: MOV EAX,[0x01267e3c]
                    cpu.eax = e.mem.u32(0x01267e3c);
                    // 00d99619: MOV dword ptr [ESP + 0xac],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xac), cpu.eax);
                    // 00d99620: MOV EAX,dword ptr [EBP + 0x14]
                    cpu.eax = e.mem.u32(cpu.ebp.wrapping_add(0x14));
                    // 00d99623: MOVAPS XMM4,xmmword ptr [EAX]
                    cpu.x[4] = load_vec(e, cpu.eax);
                    // 00d99626: MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    cpu.x[2] = load_vec(e, cpu.eax.wrapping_add(0x10));
                    // 00d9962a: MOVAPS XMM3,xmmword ptr [EAX + 0x20]
                    cpu.x[3] = load_vec(e, cpu.eax.wrapping_add(0x20));
                    // 00d9962e: MOV dword ptr [ESP + 0xa8],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xa8), cpu.edx);
                    // 00d99635: MOV EDX,dword ptr [EBP + 0x10]
                    cpu.edx = e.mem.u32(cpu.ebp.wrapping_add(0x10));
                    // 00d99638: MOVAPS XMM1,xmmword ptr [EDX]
                    cpu.x[1] = load_vec(e, cpu.edx);
                    // 00d9963b: SUBPS XMM4,XMM1
                    cpu.x[4] = subps(cpu.x[4], cpu.x[1]);
                    // 00d9963e: MOVAPS XMM1,xmmword ptr [EDX + 0x10]
                    cpu.x[1] = load_vec(e, cpu.edx.wrapping_add(0x10));
                    // 00d99642: SUBPS XMM2,XMM1
                    cpu.x[2] = subps(cpu.x[2], cpu.x[1]);
                    // 00d99645: MOVAPS XMM1,xmmword ptr [EDX + 0x20]
                    cpu.x[1] = load_vec(e, cpu.edx.wrapping_add(0x20));
                    // 00d99649: SUBPS XMM3,XMM1
                    cpu.x[3] = subps(cpu.x[3], cpu.x[1]);
                    // 00d9964c: MOVAPS XMM5,XMM2
                    cpu.x[5] = cpu.x[2];
                    // 00d9964f: MULPS XMM5,XMM2
                    cpu.x[5] = mulps(cpu.x[5], cpu.x[2]);
                    // 00d99652: MOVAPS XMM1,XMM4
                    cpu.x[1] = cpu.x[4];
                    // 00d99655: MULPS XMM1,XMM4
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[4]);
                    // 00d99658: ADDPS XMM1,XMM5
                    cpu.x[1] = addps(cpu.x[1], cpu.x[5]);
                    // 00d9965b: MOVAPS XMM5,XMM3
                    cpu.x[5] = cpu.x[3];
                    // 00d9965e: MULPS XMM5,XMM3
                    cpu.x[5] = mulps(cpu.x[5], cpu.x[3]);
                    // 00d99661: ADDPS XMM1,XMM5
                    cpu.x[1] = addps(cpu.x[1], cpu.x[5]);
                    // 00d99664: RSQRTPS XMM5,XMM1
                    cpu.x[5] = rsqrtps(cpu.x[1]);
                    // 00d99667: MOVAPS XMM6,XMM5
                    cpu.x[6] = cpu.x[5];
                    // 00d9966a: MULPS XMM6,XMM1
                    cpu.x[6] = mulps(cpu.x[6], cpu.x[1]);
                    // 00d9966d: MOVAPS XMM1,xmmword ptr [0x010c71c0]
                    cpu.x[1] = load_vec(e, 0x010c71c0);
                    // 00d99674: MULPS XMM6,XMM5
                    cpu.x[6] = mulps(cpu.x[6], cpu.x[5]);
                    // 00d99677: SUBPS XMM1,XMM6
                    cpu.x[1] = subps(cpu.x[1], cpu.x[6]);
                    // 00d9967a: MOVAPS XMM6,xmmword ptr [0x010c71d0]
                    cpu.x[6] = load_vec(e, 0x010c71d0);
                    // 00d99681: MULPS XMM5,XMM6
                    cpu.x[5] = mulps(cpu.x[5], cpu.x[6]);
                    // 00d99684: MOV dword ptr [ESP + 0x6c],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x6c), cpu.ecx);
                    // 00d99688: MOV EDI,ECX
                    cpu.edi = cpu.ecx;
                    // 00d9968a: MOV ECX,dword ptr [0x01267e34]
                    cpu.ecx = e.mem.u32(0x01267e34);
                    // 00d99690: MULPS XMM1,XMM5
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[5]);
                    // 00d99693: MOV dword ptr [ESP + 0xa4],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0xa4), cpu.ecx);
                    // 00d9969a: MOV ECX,dword ptr [EBP + 0x1c]
                    cpu.ecx = e.mem.u32(cpu.ebp.wrapping_add(0x1c));
                    // 00d9969d: RCPPS XMM5,XMM1
                    cpu.x[5] = rcpps(cpu.x[1]);
                    // 00d996a0: MOVAPS XMM6,XMM5
                    cpu.x[6] = cpu.x[5];
                    // 00d996a3: SHL ECX,0x4
                    cpu.ecx = cpu.shl32(cpu.ecx, 0x4);
                    // 00d996a6: MULPS XMM6,XMM1
                    cpu.x[6] = mulps(cpu.x[6], cpu.x[1]);
                    // 00d996a9: MULPS XMM6,XMM5
                    cpu.x[6] = mulps(cpu.x[6], cpu.x[5]);
                    // 00d996ac: ADD ECX,0x1268460
                    cpu.ecx = cpu.add32(cpu.ecx, 0x01268460);
                    // 00d996b2: MOVAPS XMM0,xmmword ptr [ECX]
                    cpu.x[0] = load_vec(e, cpu.ecx);
                    // 00d996b5: ADDPS XMM5,XMM5
                    cpu.x[5] = addps(cpu.x[5], cpu.x[5]);
                    // 00d996b8: SUBPS XMM5,XMM6
                    cpu.x[5] = subps(cpu.x[5], cpu.x[6]);
                    // 00d996bb: MOVAPS xmmword ptr [ESP + 0x90],XMM5
                    store_vec(e, cpu.esp.wrapping_add(0x90), cpu.x[5]);
                    // 00d996c3: MOV EAX,dword ptr [ESP + 0x90]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x90));
                    // 00d996ca: MOV ECX,dword ptr [ESP + 0x94]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x94));
                    // 00d996d1: MOV dword ptr [ESP + 0x80],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x80), cpu.eax);
                    // 00d996d8: MOV EAX,dword ptr [ESP + 0x98]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x98));
                    // 00d996df: MOV dword ptr [ESP + 0x84],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x84), cpu.ecx);
                    // 00d996e6: MOV ECX,dword ptr [ESP + 0x9c]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x9c));
                    // 00d996ed: MOVAPS XMM6,XMM1
                    cpu.x[6] = cpu.x[1];
                    // 00d996f0: XORPS XMM7,XMM7
                    cpu.x[7] = [0.0; 4];
                    // 00d996f3: MULPS XMM6,XMM4
                    cpu.x[6] = mulps(cpu.x[6], cpu.x[4]);
                    // 00d996f6: MULPS XMM2,XMM1
                    cpu.x[2] = mulps(cpu.x[2], cpu.x[1]);
                    // 00d996f9: MULPS XMM3,XMM1
                    cpu.x[3] = mulps(cpu.x[3], cpu.x[1]);
                    // 00d996fc: MOVAPS xmmword ptr [ESP + 0x30],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0x30), cpu.x[0]);
                    // 00d99701: MOVAPS xmmword ptr [ESP + 0x1a0],XMM6
                    store_vec(e, cpu.esp.wrapping_add(0x1a0), cpu.x[6]);
                    // 00d99709: MOVAPS xmmword ptr [ESP + 0x1b0],XMM2
                    store_vec(e, cpu.esp.wrapping_add(0x1b0), cpu.x[2]);
                    // 00d99711: MOVAPS xmmword ptr [ESP + 0x1c0],XMM3
                    store_vec(e, cpu.esp.wrapping_add(0x1c0), cpu.x[3]);
                    // 00d99719: MOVAPS xmmword ptr [ESP + 0x200],XMM7
                    store_vec(e, cpu.esp.wrapping_add(0x200), cpu.x[7]);
                    // 00d99721: MOVAPS xmmword ptr [ESP + 0xe0],XMM7
                    store_vec(e, cpu.esp.wrapping_add(0xe0), cpu.x[7]);
                    // 00d99729: MOV dword ptr [ESP + 0x88],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x88), cpu.eax);
                    // 00d99730: MOV dword ptr [ESP + 0x8c],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x8c), cpu.ecx);
                    // 00d99737: FST float ptr [ESP + 0x78]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x78), x87_store32(cpu.st(0)));
                    // 00d9973b: XOR EAX,EAX
                    cpu.eax = cpu.xor32(cpu.eax, cpu.eax);
                    // 00d9973d: FSTP float ptr [ESP + 0x7c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x7c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d99741: XOR ECX,ECX
                    cpu.ecx = cpu.xor32(cpu.ecx, cpu.ecx);
                    pc = 0x00d99743;
                    continue 'run;
                }
                0x00d99743 => {
                    // 00d99743: MOVAPS XMM2,xmmword ptr [ESP + EAX*0x1 + 0x1a0]
                    cpu.x[2] = load_vec(e, cpu.esp.wrapping_add(cpu.eax).wrapping_add(0x1a0));
                    // 00d9974b: MOVAPS XMM1,xmmword ptr [ESP + 0xa0]
                    cpu.x[1] = load_vec(e, cpu.esp.wrapping_add(0xa0));
                    // 00d99753: CMPEQPS XMM1,XMM2
                    cpu.x[1] = cmpeqps(cpu.x[1], cpu.x[2]);
                    // 00d99757: MOVAPS XMM3,XMM1
                    cpu.x[3] = cpu.x[1];
                    // 00d9975a: ANDNPS XMM3,XMM2
                    cpu.x[3] = andnps(cpu.x[3], cpu.x[2]);
                    // 00d9975d: MOVAPS XMM2,XMM1
                    cpu.x[2] = cpu.x[1];
                    // 00d99760: ANDPS XMM2,xmmword ptr [0x010c7170]
                    cpu.x[2] = andps(cpu.x[2], load_vec(e, 0x010c7170));
                    // 00d99767: ORPS XMM3,XMM2
                    cpu.x[3] = orps(cpu.x[3], cpu.x[2]);
                    // 00d9976a: RCPPS XMM4,XMM3
                    cpu.x[4] = rcpps(cpu.x[3]);
                    // 00d9976d: MOVAPS XMM2,XMM1
                    cpu.x[2] = cpu.x[1];
                    // 00d99770: ANDPS XMM1,xmmword ptr [ESP + 0xa0]
                    cpu.x[1] = andps(cpu.x[1], load_vec(e, cpu.esp.wrapping_add(0xa0)));
                    // 00d99778: MOVAPS XMM6,XMM4
                    cpu.x[6] = cpu.x[4];
                    // 00d9977b: MULPS XMM6,XMM3
                    cpu.x[6] = mulps(cpu.x[6], cpu.x[3]);
                    // 00d9977e: MOVAPS XMM3,XMM4
                    cpu.x[3] = cpu.x[4];
                    // 00d99781: ADDPS XMM3,XMM4
                    cpu.x[3] = addps(cpu.x[3], cpu.x[4]);
                    // 00d99784: MULPS XMM6,XMM4
                    cpu.x[6] = mulps(cpu.x[6], cpu.x[4]);
                    // 00d99787: SUBPS XMM3,XMM6
                    cpu.x[3] = subps(cpu.x[3], cpu.x[6]);
                    // 00d9978a: ANDNPS XMM2,XMM3
                    cpu.x[2] = andnps(cpu.x[2], cpu.x[3]);
                    // 00d9978d: ORPS XMM2,XMM1
                    cpu.x[2] = orps(cpu.x[2], cpu.x[1]);
                    // 00d99790: MOVSS XMM1,dword ptr [ESP + ECX*0x1 + 0x160]
                    cpu.x[1] = scalar_vec(e, cpu.esp.wrapping_add(cpu.ecx).wrapping_add(0x160));
                    // 00d99799: MOVAPS XMM3,XMM1
                    cpu.x[3] = cpu.x[1];
                    // 00d9979c: SHUFPS XMM3,XMM1,0x0
                    cpu.x[3] = shufps(cpu.x[3], cpu.x[1], 0);
                    // 00d997a0: MOVSS XMM1,dword ptr [ESP + ECX*0x1 + 0xc0]
                    cpu.x[1] = scalar_vec(e, cpu.esp.wrapping_add(cpu.ecx).wrapping_add(0xc0));
                    // 00d997a9: MULPS XMM3,XMM2
                    cpu.x[3] = mulps(cpu.x[3], cpu.x[2]);
                    // 00d997ac: MULPS XMM3,xmmword ptr [ESP + 0x70]
                    cpu.x[3] = mulps(cpu.x[3], load_vec(e, cpu.esp.wrapping_add(0x70)));
                    // 00d997b1: MOVAPS xmmword ptr [ESP + EAX*0x1 + 0x270],XMM3
                    store_vec(
                        e,
                        cpu.esp.wrapping_add(cpu.eax).wrapping_add(0x270),
                        cpu.x[3],
                    );
                    // 00d997b9: MOVAPS XMM3,xmmword ptr [EAX + EDX*0x1]
                    cpu.x[3] = load_vec(e, cpu.eax.wrapping_add(cpu.edx));
                    // 00d997bd: SHUFPS XMM1,XMM1,0x0
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[1], 0);
                    // 00d997c1: SUBPS XMM1,XMM3
                    cpu.x[1] = subps(cpu.x[1], cpu.x[3]);
                    // 00d997c4: MULPS XMM1,XMM2
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[2]);
                    // 00d997c7: MOVAPS xmmword ptr [ESP + EAX*0x1 + 0x2a0],XMM2
                    store_vec(
                        e,
                        cpu.esp.wrapping_add(cpu.eax).wrapping_add(0x2a0),
                        cpu.x[2],
                    );
                    // 00d997cf: MOVAPS xmmword ptr [ESP + EAX*0x1 + 0x2d0],XMM1
                    store_vec(
                        e,
                        cpu.esp.wrapping_add(cpu.eax).wrapping_add(0x2d0),
                        cpu.x[1],
                    );
                    // 00d997d7: ADD ECX,0x4
                    cpu.ecx = cpu.add32(cpu.ecx, 0x4);
                    // 00d997da: ADD EAX,0x10
                    cpu.eax = cpu.add32(cpu.eax, 0x10);
                    // 00d997dd: CMP ECX,0xc
                    cpu.cmp32(cpu.ecx, 0xc);
                    // 00d997e0: JL 0x00d99743
                    if cpu.sf != cpu.of {
                        pc = 0x00d99743;
                        continue 'run;
                    }
                    // 00d997e6: MOVSS XMM1,dword ptr [0x01012068]
                    cpu.x[1] = scalar_vec(e, 0x01012068);
                    // 00d997ee: FLD double ptr [0x010217d0]
                    cpu.st_push(e.mem.f64(0x010217d0));
                    // 00d997f4: FLD float ptr [0x010cd644]
                    cpu.st_push(x87_load32(e.mem.u32(0x010cd644)));
                    // 00d997fa: MOVAPS XMM6,xmmword ptr [ESP + 0x80]
                    cpu.x[6] = load_vec(e, cpu.esp.wrapping_add(0x80));
                    // 00d99802: SHUFPS XMM1,XMM1,0x0
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[1], 0);
                    // 00d99806: MOVAPS XMM2,XMM1
                    cpu.x[2] = cpu.x[1];
                    // 00d99809: MULPS XMM2,XMM5
                    cpu.x[2] = mulps(cpu.x[2], cpu.x[5]);
                    // 00d9980c: MINPS XMM2,XMM5
                    cpu.x[2] = minps(cpu.x[2], cpu.x[5]);
                    // 00d9980f: MOVAPS xmmword ptr [ESP + 0x170],XMM1
                    store_vec(e, cpu.esp.wrapping_add(0x170), cpu.x[1]);
                    // 00d99817: MOVAPS xmmword ptr [ESP + 0xd0],XMM2
                    store_vec(e, cpu.esp.wrapping_add(0xd0), cpu.x[2]);
                    pc = 0x00d9981f;
                    continue 'run;
                }
                0x00d9981f => {
                    // 00d9981f: TEST ESI,ESI
                    cpu.test32(cpu.esi, cpu.esi);
                    // 00d99821: JNS 0x00d99998
                    if !cpu.sf {
                        pc = 0x00d99998;
                        continue 'run;
                    }
                    pc = 0x00d99827;
                    continue 'run;
                }
                0x00d99827 => {
                    // 00d99827: MOV EDX,dword ptr [EBX]
                    cpu.edx = e.mem.u32(cpu.ebx);
                    // 00d99829: AND EDX,0x7ffffffc
                    cpu.edx = cpu.and32(cpu.edx, 0x7ffffffc);
                    // 00d9982f: ADD EBX,EDX
                    cpu.ebx = cpu.add32(cpu.ebx, cpu.edx);
                    // 00d99831: LEA EAX,[EBX + 0x8]
                    cpu.eax = cpu.ebx.wrapping_add(0x8);
                    // 00d99834: AND ESI,0x3
                    cpu.esi = cpu.and32(cpu.esi, 0x3);
                    // 00d99837: MOV dword ptr [ESP + 0x2c],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x2c), cpu.eax);
                    // 00d9983b: MOV EAX,ESI
                    cpu.eax = cpu.esi;
                    // 00d9983d: SHL EAX,0x4
                    cpu.eax = cpu.shl32(cpu.eax, 0x4);
                    // 00d99840: MOVAPS XMM1,xmmword ptr [ESP + EAX*0x1 + 0x1a0]
                    cpu.x[1] = load_vec(e, cpu.esp.wrapping_add(cpu.eax).wrapping_add(0x1a0));
                    // 00d99848: CMPEQPS XMM1,xmmword ptr [ESP + 0xa0]
                    cpu.x[1] = cmpeqps(cpu.x[1], load_vec(e, cpu.esp.wrapping_add(0xa0)));
                    // 00d99851: MOVMSKPS ECX,XMM1
                    cpu.ecx = movmskps(cpu.x[1]);
                    // 00d99854: MOVMSKPS EDX,XMM0
                    cpu.edx = movmskps(cpu.x[0]);
                    // 00d99857: MOV dword ptr [ESP + 0x28],EBX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x28), cpu.ebx);
                    // 00d9985b: TEST EDX,ECX
                    cpu.test32(cpu.edx, cpu.ecx);
                    // 00d9985d: JZ 0x00d99a22
                    if cpu.zf {
                        pc = 0x00d99a22;
                        continue 'run;
                    }
                    // 00d99863: MOV ECX,dword ptr [ESP + 0x54]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x54));
                    // 00d99867: MOVZX EDX,CX
                    cpu.edx = cpu.ecx & 0xffff;
                    // 00d9986a: MOV dword ptr [ESP + 0x24],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x24), cpu.edx);
                    // 00d9986e: LEA EBX,[ESP + ESI*0x4 + 0x160]
                    cpu.ebx = cpu
                        .esp
                        .wrapping_add(cpu.esi.wrapping_mul(4))
                        .wrapping_add(0x160);
                    // 00d99875: LEA ESI,[ESP + ESI*0x4 + 0xc0]
                    cpu.esi = cpu
                        .esp
                        .wrapping_add(cpu.esi.wrapping_mul(4))
                        .wrapping_add(0xc0);
                    // 00d9987c: FILD dword ptr [ESP + 0x24]
                    cpu.st_push(f64::from(e.mem.i32(cpu.esp.wrapping_add(0x24))));
                    // 00d99880: SHR ECX,0x10
                    cpu.ecx = cpu.shr32(cpu.ecx, 0x10);
                    // 00d99883: MOV dword ptr [ESP + 0x24],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x24), cpu.ecx);
                    // 00d99887: MOV ECX,dword ptr [EBP + 0x10]
                    cpu.ecx = e.mem.u32(cpu.ebp.wrapping_add(0x10));
                    // 00d9988a: FDIV ST0,ST2
                    {
                        let v = x87_div(cpu.st(0), cpu.st(2));
                        cpu.set_st(0, v);
                    }
                    // 00d9988c: MOVAPS XMM5,xmmword ptr [EAX + ECX*0x1]
                    cpu.x[5] = load_vec(e, cpu.eax.wrapping_add(cpu.ecx));
                    // 00d99890: MOVAPS XMM4,XMM5
                    cpu.x[4] = cpu.x[5];
                    // 00d99893: FMUL float ptr [EBX]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.ebx)));
                        cpu.set_st(0, v);
                    }
                    // 00d99895: FADD float ptr [ESI]
                    {
                        let v = x87_add(cpu.st(0), x87_m32(e.mem.u32(cpu.esi)));
                        cpu.set_st(0, v);
                    }
                    // 00d99897: FSTP float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x10), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d9989b: FILD dword ptr [ESP + 0x24]
                    cpu.st_push(f64::from(e.mem.i32(cpu.esp.wrapping_add(0x24))));
                    // 00d9989f: FDIV ST0,ST2
                    {
                        let v = x87_div(cpu.st(0), cpu.st(2));
                        cpu.set_st(0, v);
                    }
                    // 00d998a1: FMUL float ptr [EBX]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.ebx)));
                        cpu.set_st(0, v);
                    }
                    // 00d998a3: FADD float ptr [ESI]
                    {
                        let v = x87_add(cpu.st(0), x87_m32(e.mem.u32(cpu.esi)));
                        cpu.set_st(0, v);
                    }
                    // 00d998a5: FSTP float ptr [ESP + 0x4c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x4c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d998a9: FLD float ptr [0x010718c0]
                    cpu.st_push(x87_load32(e.mem.u32(0x010718c0)));
                    // 00d998af: MOVSS XMM1,dword ptr [ESP + 0x4c]
                    cpu.x[1] = scalar_vec(e, cpu.esp.wrapping_add(0x4c));
                    // 00d998b5: SHUFPS XMM1,XMM1,0x0
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[1], 0);
                    // 00d998b9: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d998bd: MOVSS XMM3,dword ptr [ESP + 0x24]
                    cpu.x[3] = scalar_vec(e, cpu.esp.wrapping_add(0x24));
                    // 00d998c3: SHUFPS XMM3,XMM3,0x0
                    cpu.x[3] = shufps(cpu.x[3], cpu.x[3], 0);
                    // 00d998c7: ADDPS XMM1,XMM3
                    cpu.x[1] = addps(cpu.x[1], cpu.x[3]);
                    // 00d998ca: CMPLEPS XMM4,XMM1
                    cpu.x[4] = cmpleps(cpu.x[4], cpu.x[1]);
                    // 00d998ce: MOVSS XMM1,dword ptr [ESP + 0x10]
                    cpu.x[1] = scalar_vec(e, cpu.esp.wrapping_add(0x10));
                    // 00d998d4: SHUFPS XMM1,XMM1,0x0
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[1], 0);
                    // 00d998d8: ANDPS XMM4,XMM0
                    cpu.x[4] = andps(cpu.x[4], cpu.x[0]);
                    // 00d998db: SUBPS XMM1,XMM3
                    cpu.x[1] = subps(cpu.x[1], cpu.x[3]);
                    // 00d998de: CMPLEPS XMM1,XMM5
                    cpu.x[1] = cmpleps(cpu.x[1], cpu.x[5]);
                    // 00d998e2: ANDPS XMM1,XMM0
                    cpu.x[1] = andps(cpu.x[1], cpu.x[0]);
                    // 00d998e5: MOVMSKPS EAX,XMM4
                    cpu.eax = movmskps(cpu.x[4]);
                    // 00d998e8: MOVAPS xmmword ptr [ESP + 0xf0],XMM4
                    store_vec(e, cpu.esp.wrapping_add(0xf0), cpu.x[4]);
                    // 00d998f0: MOVAPS xmmword ptr [ESP + 0x110],XMM1
                    store_vec(e, cpu.esp.wrapping_add(0x110), cpu.x[1]);
                    // 00d998f8: MOVMSKPS ECX,XMM1
                    cpu.ecx = movmskps(cpu.x[1]);
                    // 00d998fb: TEST EAX,EAX
                    cpu.test32(cpu.eax, cpu.eax);
                    // 00d998fd: JZ 0x00d99973
                    if cpu.zf {
                        pc = 0x00d99973;
                        continue 'run;
                    }
                    // 00d998ff: MOV EBX,dword ptr [ESP + 0x28]
                    cpu.ebx = e.mem.u32(cpu.esp.wrapping_add(0x28));
                    // 00d99903: TEST ECX,ECX
                    cpu.test32(cpu.ecx, cpu.ecx);
                    // 00d99905: JZ 0x00d9997f
                    if cpu.zf {
                        pc = 0x00d9997f;
                        continue 'run;
                    }
                    // 00d99907: MOV EDX,dword ptr [ESP + 0x2c]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0x2c));
                    // 00d9990b: MOV EAX,dword ptr [ESP + 0x110]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x110));
                    // 00d99912: MOV ECX,dword ptr [ESP + 0x114]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x114));
                    // 00d99919: MOV dword ptr [EDI + 0x30],EAX
                    e.mem.set_u32(cpu.edi.wrapping_add(0x30), cpu.eax);
                    // 00d9991c: MOV EAX,dword ptr [ESP + 0x11c]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x11c));
                    // 00d99923: MOV dword ptr [EDI + 0x34],ECX
                    e.mem.set_u32(cpu.edi.wrapping_add(0x34), cpu.ecx);
                    // 00d99926: MOV ECX,dword ptr [ESP + 0xf0]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0xf0));
                    // 00d9992d: MOV dword ptr [EDI],EDX
                    e.mem.set_u32(cpu.edi, cpu.edx);
                    // 00d9992f: MOV EDX,dword ptr [ESP + 0x118]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0x118));
                    // 00d99936: MOV dword ptr [EDI + 0x38],EDX
                    e.mem.set_u32(cpu.edi.wrapping_add(0x38), cpu.edx);
                    // 00d99939: MOV EDX,dword ptr [ESP + 0xf4]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0xf4));
                    // 00d99940: MOV dword ptr [EDI + 0x3c],EAX
                    e.mem.set_u32(cpu.edi.wrapping_add(0x3c), cpu.eax);
                    // 00d99943: MOV EAX,dword ptr [ESP + 0xf8]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0xf8));
                    // 00d9994a: MOV dword ptr [ESP + 0x30],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x30), cpu.ecx);
                    // 00d9994e: MOV ECX,dword ptr [ESP + 0xfc]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0xfc));
                    // 00d99955: MOVAPS xmmword ptr [EDI + 0x10],XMM7
                    store_vec(e, cpu.edi.wrapping_add(0x10), cpu.x[7]);
                    // 00d99959: MOVAPS xmmword ptr [EDI + 0x20],XMM6
                    store_vec(e, cpu.edi.wrapping_add(0x20), cpu.x[6]);
                    // 00d9995d: MOV dword ptr [ESP + 0x34],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x34), cpu.edx);
                    // 00d99961: MOV dword ptr [ESP + 0x38],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x38), cpu.eax);
                    // 00d99965: MOV dword ptr [ESP + 0x3c],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x3c), cpu.ecx);
                    // 00d99969: MOVAPS XMM0,xmmword ptr [ESP + 0x30]
                    cpu.x[0] = load_vec(e, cpu.esp.wrapping_add(0x30));
                    // 00d9996e: ADD EDI,0x40
                    cpu.edi = cpu.add32(cpu.edi, 0x40);
                    // 00d99971: JMP 0x00d9997f
                    pc = 0x00d9997f;
                    continue 'run;
                }
                0x00d99973 => {
                    // 00d99973: TEST ECX,ECX
                    cpu.test32(cpu.ecx, cpu.ecx);
                    // 00d99975: JZ 0x00d99ea8
                    if cpu.zf {
                        pc = 0x00d99ea8;
                        continue 'run;
                    }
                    // 00d9997b: MOV EBX,dword ptr [ESP + 0x2c]
                    cpu.ebx = e.mem.u32(cpu.esp.wrapping_add(0x2c));
                    pc = 0x00d9997f;
                    continue 'run;
                }
                0x00d9997f => {
                    // 00d9997f: MOV ESI,dword ptr [EBX]
                    cpu.esi = e.mem.u32(cpu.ebx);
                    // 00d99981: MOV EDX,dword ptr [EBX + 0x4]
                    cpu.edx = e.mem.u32(cpu.ebx.wrapping_add(0x4));
                    // 00d99984: MOV dword ptr [ESP + 0x54],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x54), cpu.edx);
                    // 00d99988: TEST ESI,ESI
                    cpu.test32(cpu.esi, cpu.esi);
                    // 00d9998a: JS 0x00d99827
                    if cpu.sf {
                        pc = 0x00d99827;
                        continue 'run;
                    }
                    // 00d99990: MOVAPS XMM5,xmmword ptr [ESP + 0x90]
                    cpu.x[5] = load_vec(e, cpu.esp.wrapping_add(0x90));
                    pc = 0x00d99998;
                    continue 'run;
                }
                0x00d99998 => {
                    // 00d99998: MOV ECX,ESI
                    cpu.ecx = cpu.esi;
                    // 00d9999a: XOR ECX,0x7fffffff
                    cpu.ecx = cpu.xor32(cpu.ecx, 0x7fffffff);
                    // 00d999a0: JZ 0x00d99eb0
                    if cpu.zf {
                        pc = 0x00d99eb0;
                        continue 'run;
                    }
                    // 00d999a6: MOV EAX,dword ptr [ESP + 0x54]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x54));
                    // 00d999aa: TEST EAX,EAX
                    cpu.test32(cpu.eax, cpu.eax);
                    // 00d999ac: JLE 0x00d99eb0
                    if cpu.zf || cpu.sf != cpu.of {
                        pc = 0x00d99eb0;
                        continue 'run;
                    }
                    // 00d999b2: MOV EDX,dword ptr [ESP + 0xbc]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0xbc));
                    // 00d999b9: FST float ptr [ESP + 0x100]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x100), x87_store32(cpu.st(0)));
                    // 00d999c0: FST float ptr [ESP + 0x104]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x104), x87_store32(cpu.st(0)));
                    // 00d999c7: LEA EBX,[ESP + 0x2a0]
                    cpu.ebx = cpu.esp.wrapping_add(0x2a0);
                    // 00d999ce: SUB EBX,dword ptr [EBP + 0x10]
                    cpu.ebx = cpu.sub32(cpu.ebx, e.mem.u32(cpu.ebp.wrapping_add(0x10)));
                    // 00d999d1: FST float ptr [ESP + 0x108]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x108), x87_store32(cpu.st(0)));
                    // 00d999d8: AND ESI,0x7fffffff
                    cpu.esi = cpu.and32(cpu.esi, 0x7fffffff);
                    // 00d999de: FST float ptr [ESP + 0x10c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x10c), x87_store32(cpu.st(0)));
                    // 00d999e5: FST float ptr [ESP + 0x120]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x120), x87_store32(cpu.st(0)));
                    // 00d999ec: SHL ESI,0x4
                    cpu.esi = cpu.shl32(cpu.esi, 0x4);
                    // 00d999ef: FST float ptr [ESP + 0x124]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x124), x87_store32(cpu.st(0)));
                    // 00d999f6: PXOR XMM0,XMM0
                    cpu.x[0] = [0.0; 4];
                    // 00d999fa: FST float ptr [ESP + 0x128]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x128), x87_store32(cpu.st(0)));
                    // 00d99a01: MOVDQA xmmword ptr [ESP + 0x1f0],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0x1f0), cpu.x[0]);
                    // 00d99a0a: FST float ptr [ESP + 0x12c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x12c), x87_store32(cpu.st(0)));
                    // 00d99a11: MOV dword ptr [ESP + 0x10],EBX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x10), cpu.ebx);
                    // 00d99a15: LEA ESI,[ESI + EDX*0x1 + 0xe]
                    cpu.esi = cpu.esi.wrapping_add(cpu.edx).wrapping_add(0xe);
                    // 00d99a19: MOV dword ptr [ESP + 0x28],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x28), cpu.eax);
                    // 00d99a1d: JMP 0x00d99bdc
                    pc = 0x00d99bdc;
                    continue 'run;
                }
                0x00d99a22 => {
                    // 00d99a22: MOV EAX,dword ptr [ESP + 0x54]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x54));
                    // 00d99a26: MOVZX ECX,AX
                    cpu.ecx = cpu.eax & 0xffff;
                    // 00d99a29: MOV dword ptr [ESP + 0x10],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x10), cpu.ecx);
                    // 00d99a2d: SHR EAX,0x10
                    cpu.eax = cpu.shr32(cpu.eax, 0x10);
                    // 00d99a30: SHL ESI,0x4
                    cpu.esi = cpu.shl32(cpu.esi, 0x4);
                    // 00d99a33: FILD dword ptr [ESP + 0x10]
                    cpu.st_push(f64::from(e.mem.i32(cpu.esp.wrapping_add(0x10))));
                    // 00d99a37: MOVAPS XMM3,xmmword ptr [ESP + ESI*0x1 + 0x270]
                    cpu.x[3] = load_vec(e, cpu.esp.wrapping_add(cpu.esi).wrapping_add(0x270));
                    // 00d99a3f: MOVAPS XMM4,xmmword ptr [ESP + ESI*0x1 + 0x2d0]
                    cpu.x[4] = load_vec(e, cpu.esp.wrapping_add(cpu.esi).wrapping_add(0x2d0));
                    // 00d99a47: MOV dword ptr [ESP + 0x10],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x10), cpu.eax);
                    // 00d99a4b: FSTP float ptr [ESP + 0x4c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x4c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d99a4f: FILD dword ptr [ESP + 0x10]
                    cpu.st_push(f64::from(e.mem.i32(cpu.esp.wrapping_add(0x10))));
                    // 00d99a53: MOVSS XMM1,dword ptr [ESP + 0x4c]
                    cpu.x[1] = scalar_vec(e, cpu.esp.wrapping_add(0x4c));
                    // 00d99a59: SHUFPS XMM1,XMM1,0x0
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[1], 0);
                    // 00d99a5d: MULPS XMM1,XMM3
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[3]);
                    // 00d99a60: ADDPS XMM1,XMM4
                    cpu.x[1] = addps(cpu.x[1], cpu.x[4]);
                    // 00d99a63: FSTP float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x10), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00d99a67: MOVAPS XMM5,XMM1
                    cpu.x[5] = cpu.x[1];
                    // 00d99a6a: MOVSS XMM1,dword ptr [ESP + 0x10]
                    cpu.x[1] = scalar_vec(e, cpu.esp.wrapping_add(0x10));
                    // 00d99a70: SHUFPS XMM1,XMM1,0x0
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[1], 0);
                    // 00d99a74: MULPS XMM1,XMM3
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[3]);
                    // 00d99a77: ADDPS XMM1,XMM4
                    cpu.x[1] = addps(cpu.x[1], cpu.x[4]);
                    // 00d99a7a: MOVAPS XMM4,xmmword ptr [ESP + ESI*0x1 + 0x1a0]
                    cpu.x[4] = load_vec(e, cpu.esp.wrapping_add(cpu.esi).wrapping_add(0x1a0));
                    // 00d99a82: MOVAPS XMM3,XMM1
                    cpu.x[3] = cpu.x[1];
                    // 00d99a85: MOVAPS XMM1,xmmword ptr [ESP + 0xa0]
                    cpu.x[1] = load_vec(e, cpu.esp.wrapping_add(0xa0));
                    // 00d99a8d: CMPLEPS XMM1,XMM4
                    cpu.x[1] = cmpleps(cpu.x[1], cpu.x[4]);
                    // 00d99a91: ANDPS XMM1,XMM0
                    cpu.x[1] = andps(cpu.x[1], cpu.x[0]);
                    // 00d99a94: MOVMSKPS EDX,XMM1
                    cpu.edx = movmskps(cpu.x[1]);
                    // 00d99a97: MOVAPS xmmword ptr [ESP + 0x1d0],XMM5
                    store_vec(e, cpu.esp.wrapping_add(0x1d0), cpu.x[5]);
                    // 00d99a9f: MOVAPS xmmword ptr [ESP + 0x190],XMM3
                    store_vec(e, cpu.esp.wrapping_add(0x190), cpu.x[3]);
                    // 00d99aa7: TEST EDX,EDX
                    cpu.test32(cpu.edx, cpu.edx);
                    // 00d99aa9: JZ 0x00d99ab3
                    if cpu.zf {
                        pc = 0x00d99ab3;
                        continue 'run;
                    }
                    // 00d99aab: MOV EAX,EBX
                    cpu.eax = cpu.ebx;
                    // 00d99aad: MOV EBX,dword ptr [ESP + 0x2c]
                    cpu.ebx = e.mem.u32(cpu.esp.wrapping_add(0x2c));
                    // 00d99ab1: JMP 0x00d99ab7
                    pc = 0x00d99ab7;
                    continue 'run;
                }
                0x00d99ab3 => {
                    // 00d99ab3: MOV EAX,dword ptr [ESP + 0x2c]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x2c));
                    pc = 0x00d99ab7;
                    continue 'run;
                }
                0x00d99ab7 => {
                    // 00d99ab7: MOVAPS XMM4,XMM1
                    cpu.x[4] = cpu.x[1];
                    // 00d99aba: ANDNPS XMM4,XMM3
                    cpu.x[4] = andnps(cpu.x[4], cpu.x[3]);
                    // 00d99abd: MOVAPS XMM3,XMM1
                    cpu.x[3] = cpu.x[1];
                    // 00d99ac0: ANDPS XMM3,XMM5
                    cpu.x[3] = andps(cpu.x[3], cpu.x[5]);
                    // 00d99ac3: ORPS XMM4,XMM3
                    cpu.x[4] = orps(cpu.x[4], cpu.x[3]);
                    // 00d99ac6: MOVAPS XMM3,XMM1
                    cpu.x[3] = cpu.x[1];
                    // 00d99ac9: ANDNPS XMM1,xmmword ptr [ESP + 0x1d0]
                    cpu.x[1] = andnps(cpu.x[1], load_vec(e, cpu.esp.wrapping_add(0x1d0)));
                    // 00d99ad1: ANDPS XMM3,xmmword ptr [ESP + 0x190]
                    cpu.x[3] = andps(cpu.x[3], load_vec(e, cpu.esp.wrapping_add(0x190)));
                    // 00d99ad9: ORPS XMM3,XMM1
                    cpu.x[3] = orps(cpu.x[3], cpu.x[1]);
                    // 00d99adc: MOVAPS XMM1,XMM7
                    cpu.x[1] = cpu.x[7];
                    // 00d99adf: CMPLEPS XMM1,XMM3
                    cpu.x[1] = cmpleps(cpu.x[1], cpu.x[3]);
                    // 00d99ae3: ANDPS XMM1,XMM0
                    cpu.x[1] = andps(cpu.x[1], cpu.x[0]);
                    // 00d99ae6: MOVAPS XMM5,XMM4
                    cpu.x[5] = cpu.x[4];
                    // 00d99ae9: CMPLEPS XMM5,XMM2
                    cpu.x[5] = cmpleps(cpu.x[5], cpu.x[2]);
                    // 00d99aed: MOVMSKPS ECX,XMM1
                    cpu.ecx = movmskps(cpu.x[1]);
                    // 00d99af0: ANDPS XMM5,XMM0
                    cpu.x[5] = andps(cpu.x[5], cpu.x[0]);
                    // 00d99af3: MOVAPS xmmword ptr [ESP + 0x130],XMM1
                    store_vec(e, cpu.esp.wrapping_add(0x130), cpu.x[1]);
                    // 00d99afb: MOVAPS xmmword ptr [ESP + 0x140],XMM5
                    store_vec(e, cpu.esp.wrapping_add(0x140), cpu.x[5]);
                    // 00d99b03: TEST ECX,ECX
                    cpu.test32(cpu.ecx, cpu.ecx);
                    // 00d99b05: JZ 0x00d99bbd
                    if cpu.zf {
                        pc = 0x00d99bbd;
                        continue 'run;
                    }
                    // 00d99b0b: MOVMSKPS ECX,XMM5
                    cpu.ecx = movmskps(cpu.x[5]);
                    // 00d99b0e: TEST ECX,ECX
                    cpu.test32(cpu.ecx, cpu.ecx);
                    // 00d99b10: JZ 0x00d99bbb
                    if cpu.zf {
                        pc = 0x00d99bbb;
                        continue 'run;
                    }
                    // 00d99b16: MOV EDX,dword ptr [ESP + 0x140]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0x140));
                    // 00d99b1d: MOV ECX,dword ptr [ESP + 0x144]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x144));
                    // 00d99b24: MOVAPS XMM5,xmmword ptr [ESP + 0x90]
                    cpu.x[5] = load_vec(e, cpu.esp.wrapping_add(0x90));
                    // 00d99b2c: MOV dword ptr [EDI + 0x30],EDX
                    e.mem.set_u32(cpu.edi.wrapping_add(0x30), cpu.edx);
                    // 00d99b2f: MOV EDX,dword ptr [ESP + 0x148]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0x148));
                    // 00d99b36: MOV dword ptr [EDI + 0x34],ECX
                    e.mem.set_u32(cpu.edi.wrapping_add(0x34), cpu.ecx);
                    // 00d99b39: MOV ECX,dword ptr [ESP + 0x14c]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x14c));
                    // 00d99b40: MOV dword ptr [EDI + 0x38],EDX
                    e.mem.set_u32(cpu.edi.wrapping_add(0x38), cpu.edx);
                    // 00d99b43: MOV EDX,dword ptr [ESP + 0x130]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0x130));
                    // 00d99b4a: MOV dword ptr [EDI],EBX
                    e.mem.set_u32(cpu.edi, cpu.ebx);
                    // 00d99b4c: MOVAPS xmmword ptr [EDI + 0x20],XMM6
                    store_vec(e, cpu.edi.wrapping_add(0x20), cpu.x[6]);
                    // 00d99b50: MOV dword ptr [EDI + 0x3c],ECX
                    e.mem.set_u32(cpu.edi.wrapping_add(0x3c), cpu.ecx);
                    // 00d99b53: MOV ECX,dword ptr [ESP + 0x138]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x138));
                    // 00d99b5a: MOV EBX,EAX
                    cpu.ebx = cpu.eax;
                    // 00d99b5c: MOV EAX,dword ptr [ESP + 0x134]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x134));
                    // 00d99b63: MOVAPS XMM0,XMM7
                    cpu.x[0] = cpu.x[7];
                    // 00d99b66: MAXPS XMM0,XMM4
                    cpu.x[0] = maxps(cpu.x[0], cpu.x[4]);
                    // 00d99b69: MOVAPS xmmword ptr [EDI + 0x10],XMM0
                    store_vec(e, cpu.edi.wrapping_add(0x10), cpu.x[0]);
                    // 00d99b6d: MOVAPS XMM2,xmmword ptr [ESP + 0x170]
                    cpu.x[2] = load_vec(e, cpu.esp.wrapping_add(0x170));
                    // 00d99b75: MULPS XMM2,xmmword ptr [ESP + 0x90]
                    cpu.x[2] = mulps(cpu.x[2], load_vec(e, cpu.esp.wrapping_add(0x90)));
                    // 00d99b7d: MOV ESI,dword ptr [EBX]
                    cpu.esi = e.mem.u32(cpu.ebx);
                    // 00d99b7f: MOV dword ptr [ESP + 0x30],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x30), cpu.edx);
                    // 00d99b83: MOV EDX,dword ptr [ESP + 0x13c]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0x13c));
                    // 00d99b8a: MOV dword ptr [ESP + 0x34],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x34), cpu.eax);
                    // 00d99b8e: MOV EAX,dword ptr [EBX + 0x4]
                    cpu.eax = e.mem.u32(cpu.ebx.wrapping_add(0x4));
                    // 00d99b91: MINPS XMM3,XMM6
                    cpu.x[3] = minps(cpu.x[3], cpu.x[6]);
                    // 00d99b94: MOVAPS XMM6,XMM3
                    cpu.x[6] = cpu.x[3];
                    // 00d99b97: MOV dword ptr [ESP + 0x38],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x38), cpu.ecx);
                    // 00d99b9b: MOV dword ptr [ESP + 0x3c],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x3c), cpu.edx);
                    // 00d99b9f: MOVAPS XMM0,xmmword ptr [ESP + 0x30]
                    cpu.x[0] = load_vec(e, cpu.esp.wrapping_add(0x30));
                    // 00d99ba4: MINPS XMM2,XMM6
                    cpu.x[2] = minps(cpu.x[2], cpu.x[6]);
                    // 00d99ba7: ADD EDI,0x40
                    cpu.edi = cpu.add32(cpu.edi, 0x40);
                    // 00d99baa: MOVAPS xmmword ptr [ESP + 0xd0],XMM2
                    store_vec(e, cpu.esp.wrapping_add(0xd0), cpu.x[2]);
                    // 00d99bb2: MOV dword ptr [ESP + 0x54],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x54), cpu.eax);
                    // 00d99bb6: JMP 0x00d9981f
                    pc = 0x00d9981f;
                    continue 'run;
                }
                0x00d99bbb => {
                    // 00d99bbb: MOV EBX,EAX
                    cpu.ebx = cpu.eax;
                    pc = 0x00d99bbd;
                    continue 'run;
                }
                0x00d99bbd => {
                    // 00d99bbd: MOV EAX,dword ptr [EBX + 0x4]
                    cpu.eax = e.mem.u32(cpu.ebx.wrapping_add(0x4));
                    // 00d99bc0: MOV ESI,dword ptr [EBX]
                    cpu.esi = e.mem.u32(cpu.ebx);
                    // 00d99bc2: MOVAPS XMM5,xmmword ptr [ESP + 0x90]
                    cpu.x[5] = load_vec(e, cpu.esp.wrapping_add(0x90));
                    // 00d99bca: MOV dword ptr [ESP + 0x54],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x54), cpu.eax);
                    // 00d99bce: JMP 0x00d9981f
                    pc = 0x00d9981f;
                    continue 'run;
                }
                0x00d99bd3 => {
                    // 00d99bd3: MOVDQA XMM0,xmmword ptr [ESP + 0x1f0]
                    cpu.x[0] = load_vec(e, cpu.esp.wrapping_add(0x1f0));
                    pc = 0x00d99bdc;
                    continue 'run;
                }
                0x00d99bdc => {
                    // 00d99bdc: MOV AX,word ptr [ESI + -0x8]
                    cpu.eax = (cpu.eax & 0xffff_0000)
                        | (u32::from(e.mem.u16(cpu.esi.wrapping_sub(0x8))) & 0xffff);
                    // 00d99be0: MOV CX,word ptr [ESI]
                    cpu.ecx = (cpu.ecx & 0xffff_0000) | (u32::from(e.mem.u16(cpu.esi)) & 0xffff);
                    // 00d99be3: MOV EDX,dword ptr [ESI + -0xe]
                    cpu.edx = e.mem.u32(cpu.esi.wrapping_sub(0xe));
                    // 00d99be6: MOVAPS XMM3,xmmword ptr [ESP + 0x160]
                    cpu.x[3] = load_vec(e, cpu.esp.wrapping_add(0x160));
                    // 00d99bee: MULPS XMM3,xmmword ptr [0x011b6de0]
                    cpu.x[3] = mulps(cpu.x[3], load_vec(e, 0x011b6de0));
                    // 00d99bf5: MOVAPS XMM4,xmmword ptr [ESP + 0xc0]
                    cpu.x[4] = load_vec(e, cpu.esp.wrapping_add(0xc0));
                    // 00d99bfd: MOV dword ptr [ESP + 0x150],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x150), cpu.edx);
                    // 00d99c04: MOV EDX,dword ptr [ESI + -0x2]
                    cpu.edx = e.mem.u32(cpu.esi.wrapping_sub(0x2));
                    // 00d99c07: MOV word ptr [ESP + 0x6a],CX
                    e.mem
                        .set_u16(cpu.esp.wrapping_add(0x6a), (cpu.ecx & 0xffff) as u16);
                    // 00d99c0c: MOV ECX,dword ptr [ESI + -0x6]
                    cpu.ecx = e.mem.u32(cpu.esi.wrapping_sub(0x6));
                    // 00d99c0f: MOV dword ptr [ESP + 0x158],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x158), cpu.ecx);
                    // 00d99c16: MOV ECX,dword ptr [ESP + 0xe0]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0xe0));
                    // 00d99c1d: MOV dword ptr [ESP + 0x15c],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x15c), cpu.edx);
                    // 00d99c24: MOV EDX,dword ptr [ESP + 0xe4]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0xe4));
                    // 00d99c2b: MOV word ptr [ESP + 0x68],AX
                    e.mem
                        .set_u16(cpu.esp.wrapping_add(0x68), (cpu.eax & 0xffff) as u16);
                    // 00d99c30: MOV EAX,dword ptr [ESI + -0xa]
                    cpu.eax = e.mem.u32(cpu.esi.wrapping_sub(0xa));
                    // 00d99c33: MOV dword ptr [ESP + 0x154],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x154), cpu.eax);
                    // 00d99c3a: MOVDQA XMM2,xmmword ptr [ESP + 0x150]
                    cpu.x[2] = load_vec(e, cpu.esp.wrapping_add(0x150));
                    // 00d99c43: MOVDQA XMM1,XMM2
                    cpu.x[1] = cpu.x[2];
                    // 00d99c47: PUNPCKHWD XMM2,XMM0
                    cpu.x[2] = from_lane_bits(punpckhwd(lane_bits(cpu.x[2]), lane_bits(cpu.x[0])));
                    // 00d99c4b: PUNPCKLWD XMM1,XMM0
                    cpu.x[1] = from_lane_bits(punpcklwd(lane_bits(cpu.x[1]), lane_bits(cpu.x[0])));
                    // 00d99c4f: MOVDQA XMM0,XMM1
                    cpu.x[0] = cpu.x[1];
                    // 00d99c53: PSLLD XMM1,0x10
                    cpu.x[1] = from_lane_bits(pslld(lane_bits(cpu.x[1]), 16));
                    // 00d99c58: PSRLD XMM1,0x10
                    cpu.x[1] = from_lane_bits(psrld(lane_bits(cpu.x[1]), 16));
                    // 00d99c5d: PSRLD XMM0,0x10
                    cpu.x[0] = from_lane_bits(psrld(lane_bits(cpu.x[0]), 16));
                    // 00d99c62: CVTDQ2PS XMM1,XMM1
                    cpu.x[1] = cvtdq2ps(lane_bits(cpu.x[1]));
                    // 00d99c65: CVTDQ2PS XMM0,XMM0
                    cpu.x[0] = cvtdq2ps(lane_bits(cpu.x[0]));
                    // 00d99c68: MULPS XMM0,xmmword ptr [ESP + 0x100]
                    cpu.x[0] = mulps(cpu.x[0], load_vec(e, cpu.esp.wrapping_add(0x100)));
                    // 00d99c70: ADDPS XMM0,XMM1
                    cpu.x[0] = addps(cpu.x[0], cpu.x[1]);
                    // 00d99c73: MOVDQA XMM1,XMM2
                    cpu.x[1] = cpu.x[2];
                    // 00d99c77: PSLLD XMM2,0x10
                    cpu.x[2] = from_lane_bits(pslld(lane_bits(cpu.x[2]), 16));
                    // 00d99c7c: PSRLD XMM2,0x10
                    cpu.x[2] = from_lane_bits(psrld(lane_bits(cpu.x[2]), 16));
                    // 00d99c81: CVTDQ2PS XMM2,XMM2
                    cpu.x[2] = cvtdq2ps(lane_bits(cpu.x[2]));
                    // 00d99c84: PSRLD XMM1,0x10
                    cpu.x[1] = from_lane_bits(psrld(lane_bits(cpu.x[1]), 16));
                    // 00d99c89: CVTDQ2PS XMM1,XMM1
                    cpu.x[1] = cvtdq2ps(lane_bits(cpu.x[1]));
                    // 00d99c8c: MULPS XMM1,xmmword ptr [ESP + 0x120]
                    cpu.x[1] = mulps(cpu.x[1], load_vec(e, cpu.esp.wrapping_add(0x120)));
                    // 00d99c94: ADDPS XMM1,XMM2
                    cpu.x[1] = addps(cpu.x[1], cpu.x[2]);
                    // 00d99c97: MULPS XMM0,XMM3
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[3]);
                    // 00d99c9a: ADDPS XMM0,XMM4
                    cpu.x[0] = addps(cpu.x[0], cpu.x[4]);
                    // 00d99c9d: MOVAPS XMM2,XMM0
                    cpu.x[2] = cpu.x[0];
                    // 00d99ca0: SHUFPS XMM2,XMM0,0x0
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[0], 0);
                    // 00d99ca4: MOVAPS xmmword ptr [ESP + 0x210],XMM2
                    store_vec(e, cpu.esp.wrapping_add(0x210), cpu.x[2]);
                    // 00d99cac: MOV dword ptr [ESP + 0x80],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x80), cpu.ecx);
                    // 00d99cb3: MOV ECX,dword ptr [ESP + 0xe8]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0xe8));
                    // 00d99cba: MOV dword ptr [ESP + 0x84],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x84), cpu.edx);
                    // 00d99cc1: MOV EDX,dword ptr [ESP + 0xec]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0xec));
                    // 00d99cc8: MULPS XMM1,XMM3
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[3]);
                    // 00d99ccb: ADDPS XMM1,XMM4
                    cpu.x[1] = addps(cpu.x[1], cpu.x[4]);
                    // 00d99cce: MOVAPS XMM2,XMM1
                    cpu.x[2] = cpu.x[1];
                    // 00d99cd1: SHUFPS XMM2,XMM1,0x0
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[1], 0);
                    // 00d99cd5: MOVAPS xmmword ptr [ESP + 0x220],XMM2
                    store_vec(e, cpu.esp.wrapping_add(0x220), cpu.x[2]);
                    // 00d99cdd: MOV dword ptr [ESP + 0x88],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x88), cpu.ecx);
                    // 00d99ce4: MOV ECX,dword ptr [ESP + 0xd0]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0xd0));
                    // 00d99ceb: MOV dword ptr [ESP + 0x8c],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x8c), cpu.edx);
                    // 00d99cf2: MOV EDX,dword ptr [ESP + 0xd4]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0xd4));
                    // 00d99cf9: MOVAPS XMM2,XMM0
                    cpu.x[2] = cpu.x[0];
                    // 00d99cfc: SHUFPS XMM2,XMM0,0x55
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[0], 85);
                    // 00d99d00: MOVAPS xmmword ptr [ESP + 0x230],XMM2
                    store_vec(e, cpu.esp.wrapping_add(0x230), cpu.x[2]);
                    // 00d99d08: MOV dword ptr [ESP + 0x70],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x70), cpu.ecx);
                    // 00d99d0c: MOV ECX,dword ptr [ESP + 0xd8]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0xd8));
                    // 00d99d13: MOV dword ptr [ESP + 0x74],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x74), cpu.edx);
                    // 00d99d17: MOV EDX,dword ptr [ESP + 0xdc]
                    cpu.edx = e.mem.u32(cpu.esp.wrapping_add(0xdc));
                    // 00d99d1e: MOVAPS XMM2,XMM1
                    cpu.x[2] = cpu.x[1];
                    // 00d99d21: SHUFPS XMM0,XMM0,0xaa
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[0], 170);
                    // 00d99d25: SHUFPS XMM2,XMM1,0x55
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[1], 85);
                    // 00d99d29: MOVAPS xmmword ptr [ESP + 0x250],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0x250), cpu.x[0]);
                    // 00d99d31: MOVAPS XMM0,xmmword ptr [0x01268550]
                    cpu.x[0] = load_vec(e, 0x01268550);
                    // 00d99d38: SHUFPS XMM1,XMM1,0xaa
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[1], 170);
                    // 00d99d3c: MOV dword ptr [ESP + 0x78],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x78), cpu.ecx);
                    // 00d99d40: MOV ECX,dword ptr [EBP + 0x10]
                    cpu.ecx = e.mem.u32(cpu.ebp.wrapping_add(0x10));
                    // 00d99d43: MOV dword ptr [ESP + 0x7c],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x7c), cpu.edx);
                    // 00d99d47: MOVAPS xmmword ptr [ESP + 0x240],XMM2
                    store_vec(e, cpu.esp.wrapping_add(0x240), cpu.x[2]);
                    // 00d99d4f: MOVAPS xmmword ptr [ESP + 0x260],XMM1
                    store_vec(e, cpu.esp.wrapping_add(0x260), cpu.x[1]);
                    // 00d99d57: LEA EAX,[ESP + 0x210]
                    cpu.eax = cpu.esp.wrapping_add(0x210);
                    // 00d99d5e: MOVAPS xmmword ptr [ESP + 0x180],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0x180), cpu.x[0]);
                    // 00d99d66: MOV EDX,0x3
                    cpu.edx = 0x3;
                    pc = 0x00d99d6b;
                    continue 'run;
                }
                0x00d99d6b => {
                    // 00d99d6b: MOVAPS XMM4,xmmword ptr [EAX]
                    cpu.x[4] = load_vec(e, cpu.eax);
                    // 00d99d6e: MOVAPS XMM5,xmmword ptr [ECX]
                    cpu.x[5] = load_vec(e, cpu.ecx);
                    // 00d99d71: MOVAPS XMM7,xmmword ptr [EAX + 0x10]
                    cpu.x[7] = load_vec(e, cpu.eax.wrapping_add(0x10));
                    // 00d99d75: MOVAPS XMM6,xmmword ptr [EBX + ECX*0x1]
                    cpu.x[6] = load_vec(e, cpu.ebx.wrapping_add(cpu.ecx));
                    // 00d99d79: MOVAPS XMM1,xmmword ptr [ESP + 0x200]
                    cpu.x[1] = load_vec(e, cpu.esp.wrapping_add(0x200));
                    // 00d99d81: MOVAPS XMM2,XMM4
                    cpu.x[2] = cpu.x[4];
                    // 00d99d84: MOVAPS XMM3,XMM7
                    cpu.x[3] = cpu.x[7];
                    // 00d99d87: ADD EAX,0x10
                    cpu.eax = cpu.add32(cpu.eax, 0x10);
                    // 00d99d8a: CMPLEPS XMM4,XMM5
                    cpu.x[4] = cmpleps(cpu.x[4], cpu.x[5]);
                    // 00d99d8e: SUBPS XMM2,XMM5
                    cpu.x[2] = subps(cpu.x[2], cpu.x[5]);
                    // 00d99d91: MULPS XMM2,XMM6
                    cpu.x[2] = mulps(cpu.x[2], cpu.x[6]);
                    // 00d99d94: SUBPS XMM3,XMM5
                    cpu.x[3] = subps(cpu.x[3], cpu.x[5]);
                    // 00d99d97: MULPS XMM3,XMM6
                    cpu.x[3] = mulps(cpu.x[3], cpu.x[6]);
                    // 00d99d9a: MOVAPS XMM0,XMM1
                    cpu.x[0] = cpu.x[1];
                    // 00d99d9d: CMPLEPS XMM0,XMM6
                    cpu.x[0] = cmpleps(cpu.x[0], cpu.x[6]);
                    // 00d99da1: CMPEQPS XMM1,XMM6
                    cpu.x[1] = cmpeqps(cpu.x[1], cpu.x[6]);
                    // 00d99da5: MOVAPS XMM6,XMM5
                    cpu.x[6] = cpu.x[5];
                    // 00d99da8: CMPLEPS XMM6,XMM7
                    cpu.x[6] = cmpleps(cpu.x[6], cpu.x[7]);
                    // 00d99dac: ANDPS XMM6,XMM4
                    cpu.x[6] = andps(cpu.x[6], cpu.x[4]);
                    // 00d99daf: MOVAPS XMM4,xmmword ptr [ESP + 0x180]
                    cpu.x[4] = load_vec(e, cpu.esp.wrapping_add(0x180));
                    // 00d99db7: ANDPS XMM6,XMM1
                    cpu.x[6] = andps(cpu.x[6], cpu.x[1]);
                    // 00d99dba: MOVAPS XMM5,XMM1
                    cpu.x[5] = cpu.x[1];
                    // 00d99dbd: ANDNPS XMM5,XMM4
                    cpu.x[5] = andnps(cpu.x[5], cpu.x[4]);
                    // 00d99dc0: ORPS XMM6,XMM5
                    cpu.x[6] = orps(cpu.x[6], cpu.x[5]);
                    // 00d99dc3: ANDPS XMM6,XMM4
                    cpu.x[6] = andps(cpu.x[6], cpu.x[4]);
                    // 00d99dc6: MOVAPS XMM5,XMM6
                    cpu.x[5] = cpu.x[6];
                    // 00d99dc9: MOVAPS XMM4,XMM0
                    cpu.x[4] = cpu.x[0];
                    // 00d99dcc: MOVAPS XMM7,XMM0
                    cpu.x[7] = cpu.x[0];
                    // 00d99dcf: ANDPS XMM7,XMM2
                    cpu.x[7] = andps(cpu.x[7], cpu.x[2]);
                    // 00d99dd2: ANDNPS XMM4,XMM3
                    cpu.x[4] = andnps(cpu.x[4], cpu.x[3]);
                    // 00d99dd5: ORPS XMM4,XMM7
                    cpu.x[4] = orps(cpu.x[4], cpu.x[7]);
                    // 00d99dd8: MOVAPS XMM6,XMM1
                    cpu.x[6] = cpu.x[1];
                    // 00d99ddb: ANDNPS XMM6,XMM4
                    cpu.x[6] = andnps(cpu.x[6], cpu.x[4]);
                    // 00d99dde: MOVAPS XMM4,xmmword ptr [ESP + 0x80]
                    cpu.x[4] = load_vec(e, cpu.esp.wrapping_add(0x80));
                    // 00d99de6: MOVAPS XMM7,XMM4
                    cpu.x[7] = cpu.x[4];
                    // 00d99de9: ANDPS XMM7,XMM1
                    cpu.x[7] = andps(cpu.x[7], cpu.x[1]);
                    // 00d99dec: ORPS XMM6,XMM7
                    cpu.x[6] = orps(cpu.x[6], cpu.x[7]);
                    // 00d99def: MOVAPS XMM7,XMM0
                    cpu.x[7] = cpu.x[0];
                    // 00d99df2: ANDNPS XMM0,XMM2
                    cpu.x[0] = andnps(cpu.x[0], cpu.x[2]);
                    // 00d99df5: MAXPS XMM6,XMM4
                    cpu.x[6] = maxps(cpu.x[6], cpu.x[4]);
                    // 00d99df8: MOVAPS XMM4,XMM6
                    cpu.x[4] = cpu.x[6];
                    // 00d99dfb: ANDPS XMM7,XMM3
                    cpu.x[7] = andps(cpu.x[7], cpu.x[3]);
                    // 00d99dfe: ORPS XMM7,XMM0
                    cpu.x[7] = orps(cpu.x[7], cpu.x[0]);
                    // 00d99e01: MOVAPS XMM0,xmmword ptr [ESP + 0x70]
                    cpu.x[0] = load_vec(e, cpu.esp.wrapping_add(0x70));
                    // 00d99e06: MOVAPS XMM2,XMM0
                    cpu.x[2] = cpu.x[0];
                    // 00d99e09: MOVAPS XMM6,XMM1
                    cpu.x[6] = cpu.x[1];
                    // 00d99e0c: ANDNPS XMM6,XMM7
                    cpu.x[6] = andnps(cpu.x[6], cpu.x[7]);
                    // 00d99e0f: ANDPS XMM2,XMM1
                    cpu.x[2] = andps(cpu.x[2], cpu.x[1]);
                    // 00d99e12: ORPS XMM6,XMM2
                    cpu.x[6] = orps(cpu.x[6], cpu.x[2]);
                    // 00d99e15: ADD EAX,0x10
                    cpu.eax = cpu.add32(cpu.eax, 0x10);
                    // 00d99e18: MINPS XMM6,XMM0
                    cpu.x[6] = minps(cpu.x[6], cpu.x[0]);
                    // 00d99e1b: ADD ECX,0x10
                    cpu.ecx = cpu.add32(cpu.ecx, 0x10);
                    // 00d99e1e: SUB EDX,0x1
                    cpu.edx = cpu.sub32(cpu.edx, 0x1);
                    // 00d99e21: MOVAPS XMM0,XMM6
                    cpu.x[0] = cpu.x[6];
                    // 00d99e24: MOVAPS xmmword ptr [ESP + 0x180],XMM5
                    store_vec(e, cpu.esp.wrapping_add(0x180), cpu.x[5]);
                    // 00d99e2c: MOVAPS xmmword ptr [ESP + 0x80],XMM4
                    store_vec(e, cpu.esp.wrapping_add(0x80), cpu.x[4]);
                    // 00d99e34: MOVAPS xmmword ptr [ESP + 0x70],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0x70), cpu.x[0]);
                    // 00d99e39: JNZ 0x00d99d6b
                    if !cpu.zf {
                        pc = 0x00d99d6b;
                        continue 'run;
                    }
                    // 00d99e3f: CMPLTPS XMM4,XMM0
                    cpu.x[4] = cmpltps(cpu.x[4], cpu.x[0]);
                    // 00d99e43: ANDPS XMM4,XMM5
                    cpu.x[4] = andps(cpu.x[4], cpu.x[5]);
                    // 00d99e46: ANDPS XMM4,xmmword ptr [ESP + 0x30]
                    cpu.x[4] = andps(cpu.x[4], load_vec(e, cpu.esp.wrapping_add(0x30)));
                    // 00d99e4b: MOVMSKPS EAX,XMM4
                    cpu.eax = movmskps(cpu.x[4]);
                    // 00d99e4e: MOVAPS xmmword ptr [ESP + 0x1e0],XMM4
                    store_vec(e, cpu.esp.wrapping_add(0x1e0), cpu.x[4]);
                    // 00d99e56: TEST EAX,EAX
                    cpu.test32(cpu.eax, cpu.eax);
                    // 00d99e58: JZ 0x00d99e9a
                    if cpu.zf {
                        pc = 0x00d99e9a;
                        continue 'run;
                    }
                    // 00d99e5a: MOV EBX,dword ptr [EBP + 0x18]
                    cpu.ebx = e.mem.u32(cpu.ebp.wrapping_add(0x18));
                    // 00d99e5d: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00d99e5f: MOV ECX,dword ptr [ESP + 0x68]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x68));
                    // 00d99e63: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00d99e65: MOV EDX,dword ptr [EBX]
                    cpu.edx = e.mem.u32(cpu.ebx);
                    // 00d99e67: MOV EDX,dword ptr [EDX + 0x4]
                    cpu.edx = e.mem.u32(cpu.edx.wrapping_add(0x4));
                    // 00d99e6a: LEA EAX,[ESP + 0x1e0]
                    cpu.eax = cpu.esp.wrapping_add(0x1e0);
                    // 00d99e71: PUSH EAX
                    cpu.push(e, cpu.eax);
                    // 00d99e72: PUSH ECX
                    cpu.push(e, cpu.ecx);
                    // 00d99e73: MOV ECX,EBX
                    cpu.ecx = cpu.ebx;
                    // 00d99e75: CALL EDX
                    e.call(
                        cpu.edx,
                        &args![
                            cpu.ecx,
                            e.mem.u32(cpu.esp.wrapping_add(0)),
                            e.mem.u32(cpu.esp.wrapping_add(4))
                        ],
                    );
                    cpu.esp = cpu.esp.wrapping_add(8);
                    // 00d99e77: MOV EAX,dword ptr [EBX]
                    cpu.eax = e.mem.u32(cpu.ebx);
                    // 00d99e79: MOV EDX,dword ptr [EAX + 0x8]
                    cpu.edx = e.mem.u32(cpu.eax.wrapping_add(0x8));
                    // 00d99e7c: LEA ECX,[ESP + 0x170]
                    cpu.ecx = cpu.esp.wrapping_add(0x170);
                    // 00d99e83: PUSH ECX
                    cpu.push(e, cpu.ecx);
                    // 00d99e84: MOV ECX,EBX
                    cpu.ecx = cpu.ebx;
                    // 00d99e86: CALL EDX
                    e.call(cpu.edx, &args![cpu.ecx, e.mem.u32(cpu.esp.wrapping_add(0))]);
                    cpu.esp = cpu.esp.wrapping_add(4);
                    // 00d99e88: FLD float ptr [0x010cd644]
                    cpu.st_push(x87_load32(e.mem.u32(0x010cd644)));
                    // 00d99e8e: FLD double ptr [0x010217d0]
                    cpu.st_push(e.mem.f64(0x010217d0));
                    // 00d99e94: MOV EBX,dword ptr [ESP + 0x10]
                    cpu.ebx = e.mem.u32(cpu.esp.wrapping_add(0x10));
                    // 00d99e98: FXCH
                    cpu.st_exchange(1);
                    pc = 0x00d99e9a;
                    continue 'run;
                }
                0x00d99e9a => {
                    // 00d99e9a: ADD ESI,0x10
                    cpu.esi = cpu.add32(cpu.esi, 0x10);
                    // 00d99e9d: SUB dword ptr [ESP + 0x28],0x1
                    {
                        let at = cpu.esp.wrapping_add(0x28);
                        let v = cpu.sub32(e.mem.u32(at), 0x1);
                        e.mem.set_u32(at, v);
                    }
                    // 00d99ea2: JNZ 0x00d99bd3
                    if !cpu.zf {
                        pc = 0x00d99bd3;
                        continue 'run;
                    }
                    pc = 0x00d99ea8;
                    continue 'run;
                }
                0x00d99ea8 => {
                    // 00d99ea8: MOVAPS XMM5,xmmword ptr [ESP + 0x90]
                    cpu.x[5] = load_vec(e, cpu.esp.wrapping_add(0x90));
                    pc = 0x00d99eb0;
                    continue 'run;
                }
                0x00d99eb0 => {
                    // 00d99eb0: MOVAPS XMM1,xmmword ptr [ESP + 0x170]
                    cpu.x[1] = load_vec(e, cpu.esp.wrapping_add(0x170));
                    pc = 0x00d99eb8;
                    continue 'run;
                }
                0x00d99eb8 => {
                    // 00d99eb8: CMP EDI,dword ptr [ESP + 0x6c]
                    cpu.cmp32(cpu.edi, e.mem.u32(cpu.esp.wrapping_add(0x6c)));
                    // 00d99ebc: JZ 0x00d99f2a
                    if cpu.zf {
                        pc = 0x00d99f2a;
                        continue 'run;
                    }
                    // 00d99ebe: MOV EBX,dword ptr [EDI + -0x40]
                    cpu.ebx = e.mem.u32(cpu.edi.wrapping_sub(0x40));
                    // 00d99ec1: MOV EAX,dword ptr [EBX + 0x4]
                    cpu.eax = e.mem.u32(cpu.ebx.wrapping_add(0x4));
                    // 00d99ec4: MOV ECX,dword ptr [EDI + -0x10]
                    cpu.ecx = e.mem.u32(cpu.edi.wrapping_sub(0x10));
                    // 00d99ec7: MOV EDX,dword ptr [EDI + -0xc]
                    cpu.edx = e.mem.u32(cpu.edi.wrapping_sub(0xc));
                    // 00d99eca: MOVAPS XMM7,xmmword ptr [EDI + -0x30]
                    cpu.x[7] = load_vec(e, cpu.edi.wrapping_sub(0x30));
                    // 00d99ece: MOVAPS XMM6,xmmword ptr [EDI + -0x20]
                    cpu.x[6] = load_vec(e, cpu.edi.wrapping_sub(0x20));
                    // 00d99ed2: MOV ESI,dword ptr [EBX]
                    cpu.esi = e.mem.u32(cpu.ebx);
                    // 00d99ed4: SUB EDI,0x40
                    cpu.edi = cpu.sub32(cpu.edi, 0x40);
                    // 00d99ed7: MOV dword ptr [ESP + 0x30],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x30), cpu.ecx);
                    // 00d99edb: MOV ECX,dword ptr [EDI + 0x3c]
                    cpu.ecx = e.mem.u32(cpu.edi.wrapping_add(0x3c));
                    // 00d99ede: MOV dword ptr [ESP + 0x54],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x54), cpu.eax);
                    // 00d99ee2: MOV EAX,dword ptr [EDI + 0x38]
                    cpu.eax = e.mem.u32(cpu.edi.wrapping_add(0x38));
                    // 00d99ee5: MOV dword ptr [ESP + 0x34],EDX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x34), cpu.edx);
                    // 00d99ee9: MOVAPS XMM2,XMM1
                    cpu.x[2] = cpu.x[1];
                    // 00d99eec: MOV dword ptr [ESP + 0x38],EAX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x38), cpu.eax);
                    // 00d99ef0: MOV dword ptr [ESP + 0x3c],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x3c), cpu.ecx);
                    // 00d99ef4: MOVAPS XMM3,xmmword ptr [ESP + 0x30]
                    cpu.x[3] = load_vec(e, cpu.esp.wrapping_add(0x30));
                    // 00d99ef9: MULPS XMM2,XMM5
                    cpu.x[2] = mulps(cpu.x[2], cpu.x[5]);
                    // 00d99efc: MINPS XMM2,XMM6
                    cpu.x[2] = minps(cpu.x[2], cpu.x[6]);
                    // 00d99eff: MOVAPS XMM0,XMM7
                    cpu.x[0] = cpu.x[7];
                    // 00d99f02: CMPLTPS XMM0,XMM2
                    cpu.x[0] = cmpltps(cpu.x[0], cpu.x[2]);
                    // 00d99f06: ANDPS XMM0,XMM3
                    cpu.x[0] = andps(cpu.x[0], cpu.x[3]);
                    // 00d99f09: MOVMSKPS EDX,XMM0
                    cpu.edx = movmskps(cpu.x[0]);
                    // 00d99f0c: MOVAPS xmmword ptr [ESP + 0xe0],XMM7
                    store_vec(e, cpu.esp.wrapping_add(0xe0), cpu.x[7]);
                    // 00d99f14: MOVAPS xmmword ptr [ESP + 0xd0],XMM2
                    store_vec(e, cpu.esp.wrapping_add(0xd0), cpu.x[2]);
                    // 00d99f1c: MOVAPS xmmword ptr [ESP + 0x30],XMM0
                    store_vec(e, cpu.esp.wrapping_add(0x30), cpu.x[0]);
                    // 00d99f21: TEST EDX,EDX
                    cpu.test32(cpu.edx, cpu.edx);
                    // 00d99f23: JZ 0x00d99eb8
                    if cpu.zf {
                        pc = 0x00d99eb8;
                        continue 'run;
                    }
                    // 00d99f25: JMP 0x00d9981f
                    pc = 0x00d9981f;
                    continue 'run;
                }
                0x00d99f2a => {
                    // 00d99f2a: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00d99f2c: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00d99f2e: CALL 0x00c85750
                    cpu.eax = e.call(0x00c85750, &args![]).u32();
                    // 00d99f33: MOV ECX,dword ptr [ESP + 0x6c]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x6c));
                    // 00d99f37: POP EDI
                    cpu.edi = cpu.pop(e);
                    // 00d99f38: MOV dword ptr [EAX],ECX
                    e.mem.set_u32(cpu.eax, cpu.ecx);
                    // 00d99f3a: MOV EAX,dword ptr [EBP + 0x8]
                    cpu.eax = e.mem.u32(cpu.ebp.wrapping_add(0x8));
                    // 00d99f3d: POP ESI
                    cpu.esi = cpu.pop(e);
                    // 00d99f3e: MOV byte ptr [EAX],0x1
                    e.mem.set_u8(cpu.eax, 0x1);
                    // 00d99f41: POP EBX
                    cpu.ebx = cpu.pop(e);
                    // 00d99f42: MOV ESP,EBP
                    cpu.esp = cpu.ebp;
                    // 00d99f44: POP EBP
                    cpu.ebp = cpu.pop(e);
                    // 00d99f45: RET
                    break 'run;
                }
                other => panic!("jump to {other:#x} outside the recovered function"),
            }
        }
        cpu.eax
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d9_84b0, fn_00d984b0(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00d9_8630, fn_00d98630(u32, u32, u32, u32) -> u32),
        entry!(0x00d9_9fc0, fn_00d99fc0(u32, u32, u32, f32, f32, f32, u32)),
        entry!(0x00d9_a040, fn_00d9a040(Ptr) -> Ptr),
        entry!(0x00d9_a070, fn_00d9a070(Ptr, u32)),
        entry!(0x00d9_a220, fn_00d9a220(Ptr, u32) -> f32),
        entry!(0x00d9_a240, fn_00d9a240(Ptr, u32) -> f32),
        entry!(0x00d9_a260, fn_00d9a260(Ptr, u32) -> u32),
        entry!(0x00d9_a270, fn_00d9a270(Ptr, u32) -> u32),
        entry!(0x00d9_a2a0, fn_00d9a2a0(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00d9_a410, fn_00d9a410(Ptr, u32)),
        entry!(0x00d9_a480, fn_00d9a480(Ptr, u32) -> f32),
        entry!(0x00d9_a4a0, fn_00d9a4a0(Ptr, u32) -> f32),
        entry!(0x00d9_a4c0, fn_00d9a4c0(Ptr, u32) -> u32),
        entry!(0x00d9_a4d0, fn_00d9a4d0(Ptr, u32) -> u32),
        entry!(0x00d9_a4e0, fn_00d9a4e0(Ptr, u32) -> Ptr),
        entry!(
            0x00d9_86c0,
            fn_00d986c0(u32, u32, u32, u32, u32, u32, u32, f32, f32, f32, u32)
        ),
        entry!(0x00d9_8ca0, fn_00d98ca0(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00d9_9570, fn_00d99570(u32, u32, u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const BOUNDS_SLOT: u32 = 0x0900_001c;

    type Seen = Rc<RefCell<Vec<(Vec<u32>, [Vec4; 3])>>>;

    fn put(e: &mut Engine, address: u32, vector: Vec4) {
        store_vec(e, address, vector);
    }

    /// Everything a cast query needs: `(this, a1, a2, a4, a5)` with the constants mapped.
    fn cast_fixture(e: &mut Engine) -> (u32, u32, u32, u32, u32) {
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(PADDING, 0.25f32);
        put(e, BOX_SCALE, [2.0, 3.0, 4.0, 5.0]);
        // The shape object: slot 0x1c writes the bounds (min, max) into the buffer it is given.
        e.put_vtable(0x0900_0000, &[0; 8]);
        e.mem.set_u32(0x0900_0000 + 0x1c, BOUNDS_SLOT);
        e.register(BOUNDS_SLOT, |e, a| {
            store_vec(e, a[3], [-1.0, -2.0, -3.0, 0.0]);
            store_vec(e, a[3] + 0x10, [1.0, 2.0, 3.0, 0.0]);
            Ret::default()
        });
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, 0x0900_0000);
        e.mem.set_u32(object + 0xc, 0x7e7e);
        let transform = e.mem.alloc(0x40);
        put(e, transform + 0x30, [1.0, 2.0, 3.0, 0.0]);
        let node = e.mem.alloc(0x20);
        e.mem.set_u32(node, object);
        e.mem.set_u32(node + 8, transform);
        let ray = e.mem.alloc(0x20);
        put(e, ray, [4.0, 6.0, 3.0, 0.0]);
        e.mem.set_f32(ray + 0x10, 7.5);
        e.mem.set_f32(ray + 0x14, 0.5);
        let record = e.mem.alloc(0x50);
        for word in 0..0x14u32 {
            e.mem.set_u32(record + 4 * word, 0x100 + word);
        }
        put(e, record + 0x20, [11.0, 12.0, 13.0, 14.0]);
        put(e, record + 0x30, [21.0, 22.0, 23.0, 24.0]);
        put(e, record + 0x40, [31.0, 32.0, 33.0, 34.0]);
        let fraction = e.mem.alloc(8);
        e.mem.set_f32(fraction, 9.0);
        let this = e.mem.alloc(0xa0);
        (this, node, ray, record, fraction)
    }

    fn final_double(e: &mut Engine) -> Seen {
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        for address in [CAST_FIRST, CAST_SECOND] {
            let log = Rc::clone(&seen);
            e.register_double(address, move |e, a| {
                let boxes = [
                    load_vec(e, a[2]),
                    load_vec(e, a[2] + 0x10),
                    load_vec(e, a[2] + 0x20),
                ];
                log.borrow_mut().push((a.to_vec(), boxes));
                Ret {
                    eax: 0x5151,
                    ..Ret::default()
                }
            });
        }
        seen
    }

    #[test]
    fn the_first_cast_records_the_query_and_hands_the_box_to_00df2750() {
        let mut e = Engine::new();
        let (this, node, ray, record, fraction) = cast_fixture(&mut e);
        let seen = final_double(&mut e);
        let result = e
            .call(
                0x00d9_84b0,
                &args![this, 0x9000u32, node, ray, 0x33u32, record, fraction, 0x66u32, 0x77u32],
            )
            .u32();
        assert_eq!(result, 0x5151);
        assert_eq!(e.mem.u32(this + 0xc), 0x66);
        assert_eq!(e.mem.u32(this + 8), 0x33);
        assert_eq!(e.mem.u32(this + 0x10), 0x77);
        assert_eq!(e.mem.u32(this + 0x14), node);
        assert_eq!(e.mem.u32(this + 4), ray);
        assert_eq!(e.mem.u32(this + 0x18), 0x7e7e);
        for word in [0u32, 1, 3, 4, 5] {
            assert_eq!(e.mem.u32(this + 0x20 + 4 * word), 0x100 + word);
        }
        assert_eq!(load_vec(&e, this + 0x40), [11.0, 12.0, 13.0, 14.0]);
        assert_eq!(load_vec(&e, this + 0x50), [21.0, 22.0, 23.0, 24.0]);
        assert_eq!(load_vec(&e, this + 0x60), [31.0, 32.0, 33.0, 34.0]);
        assert_eq!(e.mem.u32(this + 0x88), fraction);
        // d = ray - translation = (3, 4, 0): length 5.
        assert_eq!(load_vec(&e, this + 0x70), [3.0, 4.0, 0.0, 0.0]);
        assert_eq!(e.mem.f32(this + 0x84), 5.0);
        assert_eq!(e.mem.f32(this + 0x28), 0.5);
        assert_eq!(e.mem.f32(this + 0x80), 7.5);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let (arguments, boxes) = &seen[0];
        assert_eq!(arguments[1], 0x9000);
        assert_eq!(arguments[3], this);
        assert_eq!(arguments[4], 9.0f32.to_bits());
        assert_eq!(boxes[0], [-1.5, -5.0, -10.5, 0.0]);
        assert_eq!(boxes[1], [1.5, -1.0, -10.5, 0.0]);
        assert_eq!(boxes[2], [0.5, 1.0, 1.5, 0.0]);
    }

    #[test]
    fn the_second_cast_shifts_the_fields_and_ends_in_00d1c2b0() {
        let mut e = Engine::new();
        let (this, node, ray, record, fraction) = cast_fixture(&mut e);
        let seen = final_double(&mut e);
        e.call(
            0x00d9_a2a0,
            &args![this, 0x9000u32, node, ray, 0x33u32, record, fraction, 0x66u32, 0x77u32],
        );
        assert_eq!(e.mem.u32(this + 0x10), 0x66);
        assert_eq!(e.mem.u32(this + 0xc), 0x33);
        assert_eq!(e.mem.u32(this + 0x14), 0x77);
        assert_eq!(e.mem.u32(this + 0x18), node);
        assert_eq!(e.mem.u32(this + 8), ray);
        assert_eq!(e.mem.u32(this + 0x1c), 0x7e7e);
        assert_eq!(e.mem.u32(this + 0x88), fraction);
        let seen = seen.borrow();
        let (arguments, boxes) = &seen[0];
        assert_eq!(arguments[0], 0x9000);
        assert_eq!(arguments[3], this + 4);
        assert_eq!(arguments[4], 7.5f32.to_bits());
        assert_eq!(arguments[5..], [0, 0]);
        assert_eq!(boxes[2], [0.5, 1.0, 1.5, 0.0]);
        // The bounds block is what the box starts from.
        assert_eq!(arguments[1] + 0x20, arguments[2]);
    }

    #[test]
    fn the_sign_test_wants_each_difference_all_negative_zero_or_positive_on_the_masked_lanes() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x30);
        let b = e.mem.alloc(0x30);
        let out = e.mem.alloc(8);
        let fill = |e: &mut Engine, base: u32, vectors: [Vec4; 3]| {
            for (i, v) in vectors.iter().enumerate() {
                store_vec(e, base + 16 * i as u32, *v);
            }
        };
        let zero = [0.0; 4];
        fill(&mut e, b, [zero; 3]);
        // All positive on lanes 0..3, all zero, all negative.
        fill(&mut e, a, [[1.0; 4], [0.0; 4], [-1.0; 4]]);
        assert_eq!(e.call(0x00d9_8630, &args![out, b, a, 0xfu32]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        // A mixed vector fails the full mask but passes a mask that sees only matching lanes.
        fill(&mut e, a, [[1.0, -1.0, 1.0, 1.0], [0.0; 4], [1.0; 4]]);
        e.call(0x00d9_8630, &args![out, b, a, 0xfu32]);
        assert_eq!(e.mem.u8(out), 0);
        e.call(0x00d9_8630, &args![out, b, a, 0xdu32]);
        assert_eq!(e.mem.u8(out), 1);
        // A NaN lane matches none of the three tests.
        fill(&mut e, a, [[1.0, f32::NAN, 1.0, 1.0], [0.0; 4], [1.0; 4]]);
        e.call(0x00d9_8630, &args![out, b, a, 0x2u32]);
        assert_eq!(e.mem.u8(out), 0);
        e.call(0x00d9_8630, &args![out, b, a, 0xdu32]);
        assert_eq!(e.mem.u8(out), 1);
        // The difference is a minus b.
        fill(&mut e, b, [[2.0; 4]; 3]);
        fill(&mut e, a, [[3.0; 4], [1.0; 4], [2.0; 4]]);
        e.call(0x00d9_8630, &args![out, b, a, 0xfu32]);
        assert_eq!(e.mem.u8(out), 1);
        fill(&mut e, a, [[3.0; 4], [1.0, 3.0, 3.0, 3.0], [2.0; 4]]);
        e.call(0x00d9_8630, &args![out, b, a, 0xfu32]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn the_constructor_sets_both_vtables_and_clears_the_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        for word in 0..12 {
            e.mem.set_u32(this + 4 * word, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00d9_a040, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_FIRST);
        assert_eq!(e.mem.u32(this + 4), VTABLE_SECOND);
        for offset in [8u32, 0xc, 0x10, 0x14, 0x18] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x1c), 1.0f32.to_bits());
        assert_eq!(e.mem.u32(this + 0x20), 0xffff_ffff);
    }

    /// A query for the ray tests: the shape object `[query]` with vtable slots `0x20` and `0x24`
    /// doubles, `[query + 8]` an identity-like transform that swaps nothing.
    fn ray_fixture(e: &mut Engine) -> (u32, u32, u32) {
        e.put_vtable(0x0900_0200, &[0; 10]);
        e.mem.set_u32(0x0900_0200 + 0x20, 0x0900_0220);
        e.mem.set_u32(0x0900_0200 + 0x24, 0x0900_0224);
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape, 0x0900_0200);
        let transform = e.mem.alloc(0x40);
        put(e, transform, [1.0, 0.0, 0.0, 0.0]);
        put(e, transform + 0x10, [0.0, 1.0, 0.0, 0.0]);
        put(e, transform + 0x20, [0.0, 0.0, 1.0, 0.0]);
        put(e, transform + 0x30, [1.0, 2.0, 3.0, 0.0]);
        let query = e.mem.alloc(0x20);
        e.mem.set_u32(query, shape);
        e.mem.set_u32(query + 8, transform);
        let ray = e.mem.alloc(0x40);
        put(e, ray, [5.0, 7.0, 9.0, 0.0]);
        put(e, ray + 0x10, [6.0, 8.0, 10.0, 0.0]);
        e.mem.set_u32(ray + 0x24, 0xabcd);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, ray);
        e.mem.set_u32(this + 0x10, 0x4321);
        (this, query, shape)
    }

    #[test]
    fn the_ray_query_with_a_collector_calls_slot_0x24_and_keeps_its_fraction() {
        let mut e = Engine::new();
        let (this, query, shape) = ray_fixture(&mut e);
        let collector = e.mem.alloc(0x10);
        e.mem.set_f32(collector + 4, 0.75);
        e.mem.set_u32(this + 0x18, collector);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x0900_0224, move |e, a| {
            let frame: Vec<u32> = (0..10).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut().push((a.to_vec(), frame));
            Ret::default()
        });
        e.call(0x00d9_a070, &args![this, query]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let (arguments, frame) = &seen[0];
        assert_eq!(
            (arguments[0], arguments[2], arguments[3]),
            (shape, query, collector)
        );
        // The ray taken into the shape's frame: (5, 7, 9) - (1, 2, 3) with the identity rotation.
        assert_eq!(f32::from_bits(frame[0]), 4.0);
        assert_eq!(f32::from_bits(frame[1]), 5.0);
        assert_eq!(f32::from_bits(frame[2]), 6.0);
        assert_eq!(f32::from_bits(frame[4]), 5.0);
        assert_eq!(f32::from_bits(frame[5]), 6.0);
        assert_eq!(f32::from_bits(frame[6]), 7.0);
        assert_eq!(frame[8], 0xabcd);
        assert_eq!(frame[9], 0x4321);
        assert_eq!(e.mem.f32(this + 0x1c), 0.75);
    }

    #[test]
    fn the_ray_query_without_a_collector_calls_slot_0x20_and_moves_the_hit() {
        let mut e = Engine::new();
        let (this, query, _) = ray_fixture(&mut e);
        let hit = e.mem.alloc(0x60);
        put(&mut e, hit, [1.0, 2.0, 3.0, 0.0]);
        e.mem.set_f32(hit + 0x10, 0.5);
        e.mem.set_u32(this + 0x14, hit);
        // The transform rotates by swapping x and y (columns (0,1,0), (1,0,0), (0,0,1)).
        let transform = e.mem.u32(query + 8);
        put(&mut e, transform, [0.0, 1.0, 0.0, 0.0]);
        put(&mut e, transform + 0x10, [1.0, 0.0, 0.0, 0.0]);
        let hit_flag = Rc::new(RefCell::new(1u8));
        let flag = Rc::clone(&hit_flag);
        e.register_double(0x0900_0220, move |e, a| {
            e.mem.set_u8(a[1], *flag.borrow());
            Ret::default()
        });
        e.call(0x00d9_a070, &args![this, query]);
        assert_eq!(e.mem.u32(hit + 0x50), query);
        assert_eq!(load_vec(&e, hit), [2.0, 1.0, 3.0, 0.0]);
        assert_eq!(e.mem.f32(this + 0x1c), 0.5);
        // No hit: nothing changes.
        *hit_flag.borrow_mut() = 0;
        put(&mut e, hit, [1.0, 2.0, 3.0, 0.0]);
        e.mem.set_u32(hit + 0x50, 0);
        e.call(0x00d9_a070, &args![this, query]);
        assert_eq!(e.mem.u32(hit + 0x50), 0);
        assert_eq!(load_vec(&e, hit), [1.0, 2.0, 3.0, 0.0]);
        // A null shape word ends it at once.
        e.mem.set_u32(query, 0);
        e.call(0x00d9_a070, &args![this, query]);
    }

    #[test]
    fn a_failing_pre_check_object_stops_the_ray_query() {
        let mut e = Engine::new();
        let (this, query, _) = ray_fixture(&mut e);
        e.put_vtable(0x0900_0300, &[0, 0x0900_0304]);
        let checker = e.mem.alloc(0x10);
        e.mem.set_u32(checker, 0x0900_0300);
        e.mem.set_u32(this + 0xc, checker);
        let answer = e.mem.alloc(8);
        e.register_double(0x0900_0304, move |_, _| Ret {
            eax: answer,
            ..Ret::default()
        });
        let calls = Rc::new(RefCell::new(0));
        let counter = Rc::clone(&calls);
        e.register_double(0x0900_0220, move |_, _| {
            *counter.borrow_mut() += 1;
            Ret::default()
        });
        let hit_holder = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x14, hit_holder);
        e.call(0x00d9_a070, &args![this, query]);
        assert_eq!(*calls.borrow(), 0);
        e.mem.set_u8(answer, 1);
        e.call(0x00d9_a070, &args![this, query]);
        assert_eq!(*calls.borrow(), 1);
    }

    #[test]
    fn the_float_getters_run_00d9a070_and_return_the_stored_fraction() {
        let mut e = Engine::new();
        let (this, query, _) = ray_fixture(&mut e);
        let hit_holder = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x14, hit_holder);
        e.register(0x0900_0220, |_, _| Ret::default());
        e.mem.set_f32(this + 0x1c, 0.125);
        let holder = e.mem.alloc(0x40);
        e.mem.set_u32(holder + 0x20, query);
        assert_eq!(e.call(0x00d9_a220, &args![this, holder]).f32(), 0.125);
        // The second getter works on `this - 4` and reads `this + 0x18`.
        let base = e.mem.alloc(0x60);
        let second = base + 4;
        e.mem.set_u32(base + 8, e.mem.u32(this + 8));
        e.mem.set_u32(base + 0x10, 0x4321);
        e.mem.set_u32(base + 0x14, hit_holder);
        e.mem.set_f32(second + 0x18, 0.375);
        let other = e.mem.alloc(0x40);
        e.mem.set_u32(other + 0xc, query);
        assert_eq!(e.call(0x00d9_a240, &args![second, other]).f32(), 0.375);
    }

    #[test]
    fn the_deleting_destructor_thunk_works_on_this_minus_4() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, 0x0900_0008);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let frees = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&frees);
        e.register_double(0x0900_0008, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let block = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d9_a260, &args![block + 4, 0u32]).u32(), block);
        assert_eq!(e.mem.u32(block), DESTROYED_VTABLE);
        assert_eq!(e.mem.u32(block + 4), VTABLE_FIRST_BASE);
        assert!(frees.borrow().is_empty());
        assert_eq!(e.call(0x00d9_a260, &args![block + 4, 3u32]).u32(), block);
        assert_eq!(*frees.borrow(), [vec![allocator, block, 0x20]]);
    }

    /// A dispatcher table: selector `m` at `D + 32 * (kind + 0xd) + shape type` and the function
    /// `0x0900_0400` at `D + 0x9ac + 0x14 * m`.
    fn dispatch_fixture(e: &mut Engine) -> (u32, u32, u32) {
        let dispatcher = e.mem.alloc(0xc00);
        e.mem.set_u8(dispatcher + 32 * (3 + 0xd) + 5, 2);
        e.mem.set_u32(dispatcher + 0x9ac + 2 * 0x14, 0x0900_0400);
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape + 0xc, 5);
        let arg = e.mem.alloc(0x10);
        e.mem.set_u32(arg, shape);
        // The pre-check object whose slot 4 answers a flag byte.
        e.put_vtable(0x0900_0300, &[0, 0x0900_0304]);
        let checker = e.mem.alloc(0x10);
        e.mem.set_u32(checker, 0x0900_0300);
        let flag = e.mem.alloc(8);
        e.mem.set_u8(flag, 1);
        e.register_double(0x0900_0304, move |_, _| Ret {
            eax: flag,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, checker);
        e.mem.set_u32(this + 0x10, 0x11);
        e.mem.set_u32(this + 0x14, 0x22);
        e.mem.set_u32(this + 0x18, 0x9999);
        e.mem.set_u32(this + 0x1c, 3);
        e.mem.set_u32(this + 0x20, dispatcher);
        (this, arg, flag)
    }

    #[test]
    fn the_dispatching_callback_calls_the_function_the_table_picks() {
        let mut e = Engine::new();
        let (this, arg, flag) = dispatch_fixture(&mut e);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x0900_0400, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00d9_a410, &args![this, arg]);
        assert_eq!(*seen.borrow(), [vec![0x9999, arg, this + 0x20, 0x11, 0x22]]);
        // The pre-check's byte of zero, the same object, or a null shape stop it.
        e.mem.set_u8(flag, 0);
        e.call(0x00d9_a410, &args![this, arg]);
        e.mem.set_u8(flag, 1);
        let held = e.mem.u32(this + 0x18);
        e.mem.set_u32(this + 0x18, arg);
        e.call(0x00d9_a410, &args![this, arg]);
        e.mem.set_u32(this + 0x18, held);
        e.mem.set_u32(arg, 0);
        e.call(0x00d9_a410, &args![this, arg]);
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn the_two_fraction_getters_around_the_dispatcher() {
        let mut e = Engine::new();
        let (this, arg, _) = dispatch_fixture(&mut e);
        e.register(0x0900_0400, |_, _| Ret::default());
        let fractions = e.mem.alloc(0x10);
        e.mem.set_f32(fractions + 4, 0.25);
        e.mem.set_u32(this + 0x10, fractions);
        let holder = e.mem.alloc(0x40);
        e.mem.set_u32(holder + 0x20, arg);
        assert_eq!(e.call(0x00d9_a4a0, &args![this, holder]).f32(), 0.25);
        // `00d9a480` runs `00d9a410` on `this - 4`: call it with `this + 4`.
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other + 0xc, arg);
        assert_eq!(e.call(0x00d9_a480, &args![this + 4, other]).f32(), 0.25);
    }

    #[test]
    fn the_element_address_helpers_scale_the_index() {
        let mut e = Engine::new();
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, 0x1000);
        assert_eq!(
            e.call(0x00d9_a4c0, &args![array, 3u32]).u32(),
            0x1000 + 3 * 0x30
        );
        assert_eq!(
            e.call(0x00d9_a4d0, &args![array, 0x1_0002u32]).u32(),
            0x1000 + 2 * 0x14
        );
        assert_eq!(
            e.call(0x00d9_a4d0, &args![array, 7u32]).u32(),
            0x1000 + 7 * 0x14
        );
    }

    #[test]
    fn the_record_copy_moves_two_vectors_a_word_and_two_halves() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0x30);
        for word in 0..10u32 {
            e.mem.set_u32(source + 4 * word, 0x4000_0000 + word);
        }
        let this = e.mem.alloc(0x40);
        for word in 0..16u32 {
            e.mem.set_u32(this + 4 * word, 0xeeee_eeee);
        }
        assert_eq!(e.call(0x00d9_a4e0, &args![this, source]).u32(), this);
        for word in 0..9u32 {
            assert_eq!(e.mem.u32(this + 4 * word), 0x4000_0000 + word);
        }
        assert_eq!(e.mem.u16(this + 0x24), 0x0009);
        assert_eq!(e.mem.u16(this + 0x26), 0x4000);
        assert_eq!(e.mem.u32(this + 0x28), 0xeeee_eeee);
    }
}

#[cfg(test)]
mod a09_tests_b {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ID_SLOT: u32 = 0x0900_0004;
    const BYTE_SLOT: u32 = 0x0900_0008;
    const LENGTH_SLOT: u32 = 0x0900_000c;

    type Ids = Rc<RefCell<Vec<u32>>>;

    /// The constants the traversal reads, with a unit box-word scale and a high weight of
    /// 65536 (the exe's high halves are always zero here).
    fn constants(e: &mut Engine) {
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(SIXTEEN_BIT_RANGE, 65535.0f64);
        e.set_global(MARGIN, 0.25f64);
        for (lane, mask) in [1u32, 2, 4, 8].iter().enumerate() {
            e.set_global(AXIS_MASKS + 4 * lane as u32, *mask);
        }
        e.set_global(HIGH_WEIGHT, 65536.0f32);
        e.set_global(ZERO_LANE_NEAR, 0xff7f_ffeeu32);
        e.set_global(ZERO_LANE_FAR, 0x7f7f_ffeeu32);
        for lane in 0..4u32 {
            e.set_global(BOX_WORD_SCALE + 4 * lane, 1.0f32);
            e.set_global(ONES + 4 * lane, 1.0f32);
        }
        // Maximum and minimum of two floats, as `00c98210` and `00c9e0f0` answer them.
        e.register(FLOAT_MAX, |_, a| {
            let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            Ret {
                st0: f64::from(if second < first { first } else { second }),
                ..Ret::default()
            }
        });
        e.register(FLOAT_MIN, |_, a| {
            let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            Ret {
                st0: f64::from(if second <= first { second } else { first }),
                ..Ret::default()
            }
        });
    }

    /// A collector object: slot `+4` records the id, slot `+8` counts, slot `+0xc` answers
    /// `length`.
    fn collector(e: &mut Engine, length: f32) -> (u32, Ids, Rc<RefCell<u32>>) {
        e.put_vtable(0x0900_0000, &[0, ID_SLOT, BYTE_SLOT, LENGTH_SLOT]);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x0900_0000);
        let ids: Ids = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&ids);
        e.register_double(ID_SLOT, move |_, a| {
            log.borrow_mut().push(a[2]);
            Ret::default()
        });
        let bytes = Rc::new(RefCell::new(0u32));
        let counter = Rc::clone(&bytes);
        e.register_double(BYTE_SLOT, move |_, _| {
            *counter.borrow_mut() += 1;
            Ret::default()
        });
        e.register_double(LENGTH_SLOT, move |_, _| Ret {
            st0: f64::from(length),
            ..Ret::default()
        });
        (object, ids, bytes)
    }

    fn put(e: &mut Engine, address: u32, vector: Vec4) {
        store_vec(e, address, vector);
    }

    /// A 16-byte entry: the box corners `min` and `max` as words 0..2 and 4..6, the id in words
    /// 3 and 7.
    fn entry(e: &mut Engine, address: u32, min: [u16; 3], max: [u16; 3], id: u32) {
        for lane in 0..3u32 {
            e.mem.set_u16(address + 2 * lane, min[lane as usize]);
            e.mem.set_u16(address + 8 + 2 * lane, max[lane as usize]);
        }
        e.mem.set_u16(address + 6, id as u16);
        e.mem.set_u16(address + 14, (id >> 16) as u16);
    }

    /// The tree parameters: scale `a3`, offset `a4`, origin `a5`, direction `a6`.
    fn rays(e: &mut Engine, origin: Vec4, direction: Vec4) -> (u32, u32, u32, u32) {
        let block = e.mem.alloc(0x40);
        put(e, block, [65535.0, 1.0, 1.0, 1.0]);
        put(e, block + 0x10, [0.0; 4]);
        put(e, block + 0x20, origin);
        put(e, block + 0x30, direction);
        (block, block + 0x10, block + 0x20, block + 0x30)
    }

    /// Two entries, ids `0xa` and `0xb`, with a box that holds the whole test ray.
    fn entries(e: &mut Engine) -> u32 {
        let base = e.mem.alloc(0x40);
        entry(e, base, [0, 0, 0], [1, 1000, 1000], 0xa);
        entry(e, base + 0x10, [0, 0, 0], [1, 1000, 1000], 0xb);
        base
    }

    fn run_traversal(
        e: &mut Engine,
        node: u32,
        a2: u32,
        tree: (u32, u32, u32, u32),
        interval: (f32, f32, f32),
        collector: u32,
    ) {
        e.call(
            0x00d9_86c0,
            &args![
                0u32, node, a2, tree.0, tree.1, tree.2, tree.3, interval.0, interval.1, interval.2,
                collector
            ],
        );
    }

    #[test]
    fn a_leaf_calls_the_collector_for_a_box_the_ray_crosses() {
        let mut e = Engine::new();
        constants(&mut e);
        let (object, ids, _) = collector(&mut e, 0.0);
        let base = e.mem.alloc(0x40);
        entry(&mut e, base, [0, 4, 4], [1, 8, 8], 0x1234_5678);
        let leaf = e.mem.alloc(8);
        e.mem.set_u32(leaf, 0);
        e.mem.set_u32(leaf + 4, 1);
        // The box is x in [0, 65535], y and z in [4, 8]; the ray runs along x at y = z = 6.
        let tree = rays(&mut e, [0.0, 6.0, 6.0, 0.0], [1.0, 0.0, 0.0, 0.0]);
        run_traversal(&mut e, leaf, base, tree, (0.0, 100.0, 1.0), object);
        assert_eq!(*ids.borrow(), [0x1234_5678]);
        // Outside in y: a flat lane that is not inside the box ends it.
        e.mem.set_f32(tree.2 + 4, 20.0);
        run_traversal(&mut e, leaf, base, tree, (0.0, 100.0, 1.0), object);
        assert_eq!(ids.borrow().len(), 1);
    }

    #[test]
    fn empty_leaves_and_the_end_marker_do_nothing() {
        let mut e = Engine::new();
        constants(&mut e);
        let (object, ids, _) = collector(&mut e, 0.0);
        let base = entries(&mut e);
        let tree = rays(&mut e, [0.0, 6.0, 6.0, 0.0], [1.0, 0.0, 0.0, 0.0]);
        let leaf = e.mem.alloc(8);
        e.mem.set_u32(leaf + 4, 0);
        run_traversal(&mut e, leaf, base, tree, (0.0, 100.0, 1.0), object);
        e.mem.set_u32(leaf, 0x7fff_ffff);
        e.mem.set_u32(leaf + 4, 5);
        run_traversal(&mut e, leaf, base, tree, (0.0, 100.0, 1.0), object);
        e.mem.set_u32(leaf, 0);
        e.mem.set_u32(leaf + 4, 0xffff_fffe);
        run_traversal(&mut e, leaf, base, tree, (0.0, 100.0, 1.0), object);
        assert!(ids.borrow().is_empty());
    }

    /// A root with the planes 10 and 20 on the axis 0 and two leaf children of one entry each.
    fn two_leaf_tree(e: &mut Engine) -> (u32, u32) {
        let base = entries(e);
        let nodes = e.mem.alloc(0x20);
        e.mem.set_u32(nodes, 0x8000_0008);
        e.mem.set_u32(nodes + 4, 10 | (20 << 16));
        // First child: leaf at entry 0; second child: leaf at entry 1; one entry each.
        e.mem.set_u32(nodes + 8, 0);
        e.mem.set_u32(nodes + 12, 1);
        e.mem.set_u32(nodes + 16, 1);
        e.mem.set_u32(nodes + 20, 1);
        (nodes, base)
    }

    #[test]
    fn a_positive_ray_visits_the_children_in_order_through_a_recursion() {
        let mut e = Engine::new();
        constants(&mut e);
        let (object, ids, bytes) = collector(&mut e, 1000.0);
        let (root, base) = two_leaf_tree(&mut e);
        let tree = rays(&mut e, [0.0, 6.0, 6.0, 0.0], [1.0, 0.0, 0.0, 0.0]);
        run_traversal(&mut e, root, base, tree, (0.0, 50.0, 1.0), object);
        assert_eq!(*ids.borrow(), [0xa, 0xb]);
        assert_eq!(*bytes.borrow(), 1);
    }

    #[test]
    fn a_negative_ray_visits_the_second_child_first() {
        let mut e = Engine::new();
        constants(&mut e);
        let (object, ids, _) = collector(&mut e, 2000.0);
        let (root, base) = two_leaf_tree(&mut e);
        let tree = rays(&mut e, [500.0, 6.0, 6.0, 0.0], [-1.0, 0.0, 0.0, 0.0]);
        run_traversal(&mut e, root, base, tree, (0.0, 1000.0, 1.0), object);
        assert_eq!(*ids.borrow(), [0xb, 0xa]);
    }

    #[test]
    fn an_interval_that_starts_late_or_ends_early_visits_one_child() {
        let mut e = Engine::new();
        constants(&mut e);
        let (object, ids, bytes) = collector(&mut e, 1000.0);
        let (root, base) = two_leaf_tree(&mut e);
        let tree = rays(&mut e, [0.0, 6.0, 6.0, 0.0], [1.0, 0.0, 0.0, 0.0]);
        // Start after the first range (20): only the second child.
        run_traversal(&mut e, root, base, tree, (30.0, 50.0, 1.0), object);
        assert_eq!(*ids.borrow(), [0xb]);
        // End before the first range (10): only the first child.
        ids.borrow_mut().clear();
        run_traversal(&mut e, root, base, tree, (0.0, 5.0, 1.0), object);
        assert_eq!(*ids.borrow(), [0xa]);
        assert_eq!(*bytes.borrow(), 0);
    }

    #[test]
    fn a_ray_along_the_axis_picks_the_children_by_the_origin() {
        let mut e = Engine::new();
        constants(&mut e);
        let (object, ids, _) = collector(&mut e, 1000.0);
        let (root, base) = two_leaf_tree(&mut e);
        // Direction zero on axis 0: the origin x decides (planes 10 and 20, margin 0.25).
        let tree = rays(&mut e, [5.0, 6.0, 6.0, 0.0], [0.0, 1.0, 0.0, 0.0]);
        run_traversal(&mut e, root, base, tree, (0.0, 50.0, 1.0), object);
        // Below the near plane: only the first child.
        assert_eq!(*ids.borrow(), [0xa]);
        ids.borrow_mut().clear();
        e.mem.set_f32(tree.2, 15.0);
        run_traversal(&mut e, root, base, tree, (0.0, 50.0, 1.0), object);
        assert_eq!(*ids.borrow(), [0xa, 0xb]);
        ids.borrow_mut().clear();
        e.mem.set_f32(tree.2, 30.0);
        run_traversal(&mut e, root, base, tree, (0.0, 50.0, 1.0), object);
        assert_eq!(*ids.borrow(), [0xb]);
    }

    #[test]
    fn the_wrapper_scans_with_the_bounds_of_its_object() {
        let mut e = Engine::new();
        constants(&mut e);
        let (object, ids, _) = collector(&mut e, 0.0);
        let base = e.mem.alloc(0x40);
        entry(&mut e, base, [0, 4, 4], [1, 8, 8], 0x77);
        let leaf = e.mem.alloc(8);
        e.mem.set_u32(leaf, 0);
        e.mem.set_u32(leaf + 4, 1);
        let p = e.mem.alloc(0x60);
        e.mem.set_u32(p, leaf);
        e.mem.set_u32(p + 0x1c, base);
        put(&mut e, p + 0x30, [0.0; 4]);
        put(&mut e, p + 0x40, [65535.0, 1.0, 1.0, 1.0]);
        let origin = e.mem.alloc(0x20);
        put(&mut e, origin, [0.0, 6.0, 6.0, 0.0]);
        let direction = e.mem.alloc(0x20);
        put(&mut e, direction, [1.0, 0.0, 0.0, 0.0]);
        e.call(
            0x00d9_9fc0,
            &args![p, origin, direction, 0.0f32, 100.0f32, 1.0f32, object],
        );
        assert_eq!(*ids.borrow(), [0x77]);
    }
}

#[cfg(test)]
mod a09_tests_c {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ID_SLOT: u32 = 0x0900_0004;
    const FINISH_SLOT: u32 = 0x0900_0008;
    const LENGTH_SLOT: u32 = 0x0900_000c;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    #[test]
    fn compares_set_the_flags_the_exe_tests() {
        let mut cpu = Cpu::default();
        cpu.cmp32(1, 2);
        assert!(!cpu.zf && cpu.sf && !cpu.of);
        cpu.cmp32(5, 5);
        assert!(cpu.zf && !cpu.sf && !cpu.of);
        // 0x80000000 - 1 overflows: the result is positive.
        cpu.cmp32(0x8000_0000, 1);
        assert!(!cpu.zf && !cpu.sf && cpu.of);
        // JG after a compare of two signed numbers: -1 is not greater than 0.
        cpu.cmp32(0xffff_ffff, 0);
        assert!(cpu.sf != cpu.of);
        cpu.cmp8(0x80, 0x01);
        assert!(!cpu.zf && !cpu.sf && cpu.of);
    }

    #[test]
    fn logic_and_arithmetic_set_the_flags_from_the_result() {
        let mut cpu = Cpu::default();
        assert_eq!(cpu.add32(0x7fff_ffff, 1), 0x8000_0000);
        assert!(cpu.sf && cpu.of && !cpu.zf);
        assert_eq!(cpu.sub32(3, 3), 0);
        assert!(cpu.zf && cpu.pf && !cpu.of);
        assert_eq!(cpu.and32(0xf0, 0x0f), 0);
        assert!(cpu.zf && !cpu.of);
        assert_eq!(cpu.xor32(0x7fff_ffff, 0x7fff_ffff), 0);
        assert!(cpu.zf);
        // The parity flag follows the low byte: 0x03 has two bits set, 0x07 three.
        cpu.test8(0x03, 0xff);
        assert!(cpu.pf && !cpu.zf);
        cpu.test8(0x07, 0xff);
        assert!(!cpu.pf);
        cpu.test8(0x12, 0x05);
        assert!(cpu.zf && cpu.pf);
        assert_eq!(cpu.shl32(0x8000_0001, 4), 0x10);
        assert_eq!(cpu.shr32(0x10, 4), 1);
        assert_eq!(cpu.shl32(7, 0), 7);
    }

    #[test]
    fn the_x87_stack_has_st0_on_top() {
        let mut cpu = Cpu::default();
        cpu.st_push(1.0);
        cpu.st_push(2.0);
        cpu.st_push(3.0);
        assert_eq!((cpu.st(0), cpu.st(1), cpu.st(2)), (3.0, 2.0, 1.0));
        cpu.st_exchange(2);
        assert_eq!((cpu.st(0), cpu.st(1), cpu.st(2)), (1.0, 2.0, 3.0));
        cpu.set_st(1, 9.0);
        cpu.st_pop();
        assert_eq!((cpu.st(0), cpu.st(1)), (9.0, 3.0));
    }

    /// The constants the two walks read (the exe's values).
    fn constants(e: &mut Engine) {
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(0x0101_1d78u32, 0.0f32);
        e.set_global(0x0101_7718u32, 3.0f32);
        e.set_global(0x0101_6248u32, 0.5f32);
        e.set_global(0x0101_2068u32, 1.0f32);
        e.set_global(0x0107_18c0u32, f32::from_bits(0x3727_c5ac));
        e.set_global(SIXTEEN_BIT_RANGE, 65535.0f64);
        e.set_global(0x0103_2980u32, f64::from_bits(0x3f1a_36e2_e000_0000));
        e.set_global(0x010c_d644u32, f32::from_bits(0x5a0e_1bca));
        e.set_global(0x0104_e148u32, 0xff7f_ffeeu32);
        e.set_global(0x0104_da48u32, 0x7f7f_ffeeu32);
        e.set_global(0x010d_4080u32, f32::from_bits(0x3780_0080));
        for (lane, mask) in [1u32, 2, 4, 8].iter().enumerate() {
            e.set_global(AXIS_MASKS + 4 * lane as u32, *mask);
        }
        for lane in 0..4u32 {
            e.set_global(0x010c_7170 + 4 * lane, 1.0f32);
            e.set_global(0x011b_6de0 + 4 * lane, f32::from_bits(0x3780_0080));
            e.set_global(0x010c_71c0 + 4 * lane, 3.0f32);
            e.set_global(0x010c_71d0 + 4 * lane, 0.5f32);
            e.set_global(
                0x010c_4c10 + 4 * lane,
                if lane < 3 { 0xffff_ffffu32 } else { 0 },
            );
        }
    }

    /// The memory router: its first word is the stack top, a region of 0x4000 bytes.
    fn router(e: &mut Engine) -> (u32, u32) {
        let region = e.mem.alloc(0x4000);
        let router = e.mem.alloc(8);
        e.mem.set_u32(router, region);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (router, region)
    }

    /// A collector: slot `+4` records `[4, first word, second word]`, slot `+8` records
    /// `[8, argument]`, slot `+0xc` answers `length`.
    fn collector(e: &mut Engine, length: f32) -> (u32, Log) {
        e.put_vtable(0x0900_0000, &[0, ID_SLOT, FINISH_SLOT, LENGTH_SLOT]);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x0900_0000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let hits = Rc::clone(&log);
        e.register_double(ID_SLOT, move |_, a| {
            hits.borrow_mut().push(vec![4, a[1], a[2]]);
            Ret::default()
        });
        let finishes = Rc::clone(&log);
        e.register_double(FINISH_SLOT, move |_, a| {
            finishes.borrow_mut().push(vec![8, a[1]]);
            Ret::default()
        });
        e.register_double(LENGTH_SLOT, move |_, _| Ret {
            st0: f64::from(length),
            ..Ret::default()
        });
        (object, log)
    }

    /// The ids the collector was told of: the single-ray walk passes `(result byte, id)`, the
    /// packet walk `(id, hit lanes)`, so the id is the word `word` of the call.
    fn hit_ids(log: &Log, word: usize) -> Vec<u32> {
        log.borrow()
            .iter()
            .filter(|c| c[0] == 4)
            .map(|c| c[word])
            .collect()
    }

    /// A 16-byte entry: the box corners `min` and `max` as words 0..2 and 4..6, the id in words
    /// 3 and 7.
    fn entry(e: &mut Engine, address: u32, min: [u16; 3], max: [u16; 3], id: u32) {
        for lane in 0..3u32 {
            e.mem.set_u16(address + 2 * lane, min[lane as usize]);
            e.mem.set_u16(address + 8 + 2 * lane, max[lane as usize]);
        }
        e.mem.set_u16(address + 6, id as u16);
        e.mem.set_u16(address + 14, (id >> 16) as u16);
    }

    /// The query block: root node, stack capacity, entries, the box `[0, 65535]` on every axis.
    fn query(e: &mut Engine, root: u32, entries: u32) -> u32 {
        let query = e.mem.alloc(0x60);
        e.mem.set_u32(query, root);
        e.mem.set_u32(query + 0xc, 8);
        e.mem.set_u32(query + 0x1c, entries);
        store_vec(e, query + 0x30, [0.0; 4]);
        store_vec(e, query + 0x40, [65535.0, 65535.0, 65535.0, 0.0]);
        query
    }

    /// A leaf with `count` entries from entry `first`.
    fn leaf(e: &mut Engine, first: u32, count: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, first);
        e.mem.set_u32(node + 4, count);
        node
    }

    /// One box `x in [0, 1]`, `y and z in [4, 8]` with the id `0x12345678`, as the only entry.
    fn one_box(e: &mut Engine) -> u32 {
        let base = e.mem.alloc(0x40);
        entry(e, base, [0, 4, 4], [1, 8, 8], 0x1234_5678);
        base
    }

    fn vector(e: &mut Engine, lanes: [f32; 4]) -> u32 {
        let at = e.mem.alloc(0x10);
        store_vec(e, at, lanes);
        at
    }

    struct Single {
        e: Engine,
        log: Log,
        router: u32,
        region: u32,
        result: u32,
        returned: u32,
    }

    /// Runs `00d98ca0` over a leaf with `count` entries of `one_box`, the ray from `from` to `to`.
    fn run_single(from: [f32; 4], to: [f32; 4], first: u32, count: u32, fill: u32) -> Single {
        let mut e = Engine::new();
        constants(&mut e);
        let (router, region) = router(&mut e);
        let (object, log) = collector(&mut e, 1000.0);
        let base = one_box(&mut e);
        let root = leaf(&mut e, first, count);
        let query = query(&mut e, root, base);
        let (from, to) = (vector(&mut e, from), vector(&mut e, to));
        let result = e.mem.alloc(0x20);
        let returned = run_00d98ca0(&mut e, [result, query, from, to, object], fill);
        Single {
            e,
            log,
            router,
            region,
            result,
            returned,
        }
    }

    #[test]
    fn a_ray_through_the_box_reaches_the_collector() {
        let run = run_single([-5.0, 6.0, 6.0, 0.0], [5.0, 6.0, 6.0, 0.0], 0, 1, 0);
        assert_eq!(run.returned, run.result);
        assert_eq!(hit_ids(&run.log, 2), [0x1234_5678]);
        assert_eq!(run.log.borrow().last(), Some(&vec![8, run.result]));
        // The router's stack top is put back.
        assert_eq!(run.e.mem.u32(run.router), run.region);
    }

    #[test]
    fn a_ray_beside_the_box_only_finishes() {
        let run = run_single([-5.0, 20.0, 6.0, 0.0], [5.0, 20.0, 6.0, 0.0], 0, 1, 0);
        assert_eq!(*run.log.borrow(), vec![vec![8, run.result]]);
        assert_eq!(run.e.mem.u32(run.router), run.region);
    }

    #[test]
    fn an_empty_leaf_and_the_end_marker_find_nothing() {
        let run = run_single([-5.0, 6.0, 6.0, 0.0], [5.0, 6.0, 6.0, 0.0], 0, 0, 0);
        assert_eq!(*run.log.borrow(), vec![vec![8, run.result]]);
        let run = run_single(
            [-5.0, 6.0, 6.0, 0.0],
            [5.0, 6.0, 6.0, 0.0],
            0x7fff_ffff,
            1,
            0,
        );
        assert_eq!(*run.log.borrow(), vec![vec![8, run.result]]);
    }

    #[test]
    fn the_walk_does_not_depend_on_what_the_frame_held() {
        for ray in [
            ([-5.0, 6.0, 6.0, 0.0], [5.0, 6.0, 6.0, 0.0]),
            ([-5.0, 20.0, 6.0, 0.0], [5.0, 20.0, 6.0, 0.0]),
            ([0.5, 6.0, 6.0, 0.0], [0.5, 6.0, 9.0, 0.0]),
        ] {
            let quiet = run_single(ray.0, ray.1, 0, 1, 0);
            for fill in [0xdead_beef, 0xffff_ffff, 0x3f80_0000] {
                let noisy = run_single(ray.0, ray.1, 0, 1, fill);
                let a: Vec<Vec<u32>> = quiet.log.borrow().iter().map(|c| c[..1].to_vec()).collect();
                let b: Vec<Vec<u32>> = noisy.log.borrow().iter().map(|c| c[..1].to_vec()).collect();
                assert_eq!(a, b, "{ray:?} {fill:#x}");
            }
        }
    }

    /// A root with the planes 10 and 20 on the axis 0 and two leaf children of one entry each.
    fn two_leaf_tree(e: &mut Engine) -> (u32, u32) {
        let base = e.mem.alloc(0x40);
        entry(e, base, [0, 0, 0], [1, 1000, 1000], 0xa);
        entry(e, base + 0x10, [0, 0, 0], [1, 1000, 1000], 0xb);
        let nodes = e.mem.alloc(0x20);
        e.mem.set_u32(nodes, 0x8000_0008);
        e.mem.set_u32(nodes + 4, 10 | (20 << 16));
        e.mem.set_u32(nodes + 8, 0);
        e.mem.set_u32(nodes + 12, 1);
        e.mem.set_u32(nodes + 16, 1);
        e.mem.set_u32(nodes + 20, 1);
        (nodes, base)
    }

    #[test]
    fn a_tree_with_two_leaves_is_walked_without_panicking() {
        let mut e = Engine::new();
        constants(&mut e);
        let (router, region) = router(&mut e);
        let (object, log) = collector(&mut e, 1000.0);
        let (root, base) = two_leaf_tree(&mut e);
        let query = query(&mut e, root, base);
        let (from, to) = (
            vector(&mut e, [0.0, 6.0, 6.0, 0.0]),
            vector(&mut e, [50.0, 6.0, 6.0, 0.0]),
        );
        let result = e.mem.alloc(0x20);
        run_00d98ca0(&mut e, [result, query, from, to, object], 0);
        // Whatever it found, it finished exactly once and gave the stack back.
        let finishes = log.borrow().iter().filter(|c| c[0] == 8).count();
        assert_eq!(finishes, 1);
        assert_eq!(e.mem.u32(router), region);
    }

    /// The ids and finishes the single-ray walk reports over the two-leaf tree, with a frame
    /// that starts as `fill`.
    fn two_leaf_walk(from: [f32; 4], to: [f32; 4], fill: u32) -> Vec<Vec<u32>> {
        let mut e = Engine::new();
        constants(&mut e);
        router(&mut e);
        let (object, log) = collector(&mut e, 1000.0);
        let (root, base) = two_leaf_tree(&mut e);
        let query = query(&mut e, root, base);
        let (from, to) = (vector(&mut e, from), vector(&mut e, to));
        let result = e.mem.alloc(0x20);
        run_00d98ca0(&mut e, [result, query, from, to, object], fill);
        let log = log.borrow().iter().map(|c| c[..1].to_vec()).collect();
        log
    }

    #[test]
    fn the_inner_node_walk_does_not_depend_on_what_the_frame_held() {
        for ray in [
            ([0.0, 6.0, 6.0, 0.0], [50.0, 6.0, 6.0, 0.0]),
            ([50.0, 6.0, 6.0, 0.0], [0.0, 6.0, 6.0, 0.0]),
            ([15.0, 6.0, 6.0, 0.0], [15.0, 6.0, 60.0, 0.0]),
            ([-5.0, 6.0, 6.0, 0.0], [5.0, 6.0, 6.0, 0.0]),
        ] {
            let quiet = two_leaf_walk(ray.0, ray.1, 0);
            for fill in [0xdead_beef, 0xffff_ffff, 0x7fc0_0000] {
                assert_eq!(
                    quiet,
                    two_leaf_walk(ray.0, ray.1, fill),
                    "{ray:?} {fill:#x}"
                );
            }
        }
    }

    /// A deterministic generator for the differential test.
    struct Lcg(u64);

    impl Lcg {
        fn next(&mut self) -> u32 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (self.0 >> 33) as u32
        }

        fn below(&mut self, limit: u32) -> u32 {
            self.next() % limit
        }
    }

    /// A tree of one inner node (a random axis and two random planes) over two leaves of four
    /// random boxes each; the boxes' ids are 1 to 8. Returns (root, entries).
    fn random_tree(e: &mut Engine, rng: &mut Lcg) -> (u32, u32) {
        let axis = rng.below(3) as usize;
        let low = 20000 + rng.below(10000);
        let high = low + rng.below(20000);
        let base = e.mem.alloc(0x100);
        for i in 0..8u32 {
            let mut min = [rng.below(50000), rng.below(50000), rng.below(50000)];
            let mut max = [0, 1, 2].map(|k| (min[k] + 100 + rng.below(15000)).min(65535));
            // The first four boxes lie below both planes on the split axis, the others above.
            if i < 4 {
                min[axis] = rng.below(low - 300);
                max[axis] = (min[axis] + 100 + rng.below(low - 200 - min[axis])).min(low - 100);
            } else {
                min[axis] = high + 200 + rng.below(65000 - high - 200);
                max[axis] = (min[axis] + 100 + rng.below(65535 - min[axis] - 100)).min(65535);
            }
            entry(
                e,
                base + 16 * i,
                min.map(|w| w as u16),
                max.map(|w| w as u16),
                i + 1,
            );
        }
        let nodes = e.mem.alloc(0x20);
        e.mem.set_u32(nodes, 0x8000_0008 | axis as u32);
        e.mem.set_u32(nodes + 4, low | (high << 16));
        for (i, first) in [0u32, 4].iter().enumerate() {
            e.mem.set_u32(nodes + 8 + 8 * i as u32, *first);
            e.mem.set_u32(nodes + 12 + 8 * i as u32, 4);
        }
        (nodes, base)
    }

    #[test]
    fn the_loop_walk_finds_what_the_recursive_walk_finds() {
        let mut rng = Lcg(0x1234_5678_9abc_def0);
        let mut compared = 0;
        let mut hit_something = 0;
        for _ in 0..300 {
            let mut e = Engine::new();
            constants(&mut e);
            e.set_global(MARGIN, 0.25f64);
            e.register(FLOAT_MAX, |_, a| {
                let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
                Ret {
                    st0: f64::from(if second < first { first } else { second }),
                    ..Ret::default()
                }
            });
            e.register(FLOAT_MIN, |_, a| {
                let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
                Ret {
                    st0: f64::from(if second <= first { second } else { first }),
                    ..Ret::default()
                }
            });
            router(&mut e);
            let (root, base) = random_tree(&mut e, &mut rng);
            let from = [0, 1, 2].map(|_| rng.below(70000) as f32 - 3000.0);
            let to = [0, 1, 2].map(|_| rng.below(70000) as f32 - 3000.0);
            let delta = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
            let length = (delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]).sqrt();
            let direction = [delta[0] / length, delta[1] / length, delta[2] / length, 0.0];
            let (object, log) = collector(&mut e, 1000.0);
            let query = query(&mut e, root, base);
            let (start, end) = (
                vector(&mut e, [from[0], from[1], from[2], 0.0]),
                vector(&mut e, [to[0], to[1], to[2], 0.0]),
            );
            let result = e.mem.alloc(0x20);
            run_00d98ca0(&mut e, [result, query, start, end, object], 0);
            let mut looped = hit_ids(&log, 2);
            log.borrow_mut().clear();
            let scale = vector(&mut e, [65535.0, 65535.0, 65535.0, 0.0]);
            let offset = vector(&mut e, [0.0; 4]);
            let origin = vector(&mut e, [from[0], from[1], from[2], 0.0]);
            let aim = vector(&mut e, direction);
            e.call(
                0x00d9_86c0,
                &args![
                    0u32, root, base, scale, offset, origin, aim, 0.0f32, length, length, object
                ],
            );
            let mut recursive = hit_ids(&log, 2);
            looped.sort_unstable();
            recursive.sort_unstable();
            looped.dedup();
            recursive.dedup();
            let truth: Vec<u32> = (0..8u32)
                .filter(|i| {
                    let w: Vec<f64> = (0..8)
                        .map(|k| f64::from(e.mem.u16(base + 16 * i + 2 * k)))
                        .collect();
                    let (mut t0, mut t1) = (0.0f64, f64::from(length));
                    for axis in 0..3 {
                        let (lo, hi) = (w[axis].min(w[4 + axis]), w[axis].max(w[4 + axis]));
                        let (o, d) = (f64::from(from[axis]), f64::from(direction[axis]));
                        if d == 0.0 {
                            if o < lo || o > hi {
                                return false;
                            }
                            continue;
                        }
                        let (a, b) = ((lo - o) / d, (hi - o) / d);
                        t0 = t0.max(a.min(b));
                        t1 = t1.min(a.max(b));
                    }
                    t0 <= t1
                })
                .map(|i| i + 1)
                .collect();
            assert_eq!(looped, truth, "{from:?} -> {to:?}");
            assert_eq!(recursive, truth, "{from:?} -> {to:?}");
            compared += 1;
            if !looped.is_empty() {
                hit_something += 1;
            }
        }
        assert_eq!(compared, 300);
        assert!(hit_something > 20, "only {hit_something} rays hit a box");
    }

    /// The distance parameters `[0, 1]` in which the segment `from -> to` is inside the box
    /// `[min, max]` (decoded words), by the slab method: whether it touches it.
    fn segment_touches(from: [f32; 3], to: [f32; 3], words: &[u16]) -> bool {
        let (mut t0, mut t1) = (0.0f64, 1.0f64);
        for axis in 0..3 {
            let (a, b) = (f64::from(words[axis]), f64::from(words[4 + axis]));
            let (lo, hi) = (a.min(b), a.max(b));
            let (o, d) = (
                f64::from(from[axis]),
                f64::from(to[axis]) - f64::from(from[axis]),
            );
            if d == 0.0 {
                if o < lo || o > hi {
                    return false;
                }
                continue;
            }
            let (x, y) = ((lo - o) / d, (hi - o) / d);
            t0 = t0.max(x.min(y));
            t1 = t1.min(x.max(y));
        }
        t0 <= t1
    }

    #[test]
    fn the_packet_walk_finds_what_each_of_its_rays_finds() {
        let mut rng = Lcg(0x0fed_cba9_8765_4321);
        let mut reported_pairs = 0;
        for _ in 0..150 {
            let mut e = Engine::new();
            constants(&mut e);
            let all = [f32::from_bits(u32::MAX); 4];
            store_vec(&mut e, 0x0126_8460, all);
            store_vec(&mut e, 0x0126_8550, all);
            router(&mut e);
            let (root, base) = random_tree(&mut e, &mut rng);
            // Four rays that move the same way (the packet walk is for coherent rays): a base
            // ray and three jittered copies whose direction signs agree with it.
            let base_from = [0, 1, 2].map(|_| rng.below(70000) as f32 - 3000.0);
            let base_to = [0, 1, 2].map(|_| rng.below(70000) as f32 - 3000.0);
            let signs = |from: [f32; 3], to: [f32; 3]| [0, 1, 2].map(|k| to[k] > from[k]);
            let mut rays = vec![(base_from, base_to)];
            while rays.len() < 4 {
                let jitter =
                    |rng: &mut Lcg, p: [f32; 3]| p.map(|v| v + rng.below(4000) as f32 - 2000.0);
                let (from, to) = (jitter(&mut rng, base_from), jitter(&mut rng, base_to));
                if signs(from, to) == signs(base_from, base_to) {
                    rays.push((from, to));
                }
            }
            let from_block = e.mem.alloc(0x30);
            let to_block = e.mem.alloc(0x30);
            for axis in 0..3usize {
                let from_lanes = [0, 1, 2, 3].map(|k| rays[k].0[axis]);
                let to_lanes = [0, 1, 2, 3].map(|k| rays[k].1[axis]);
                store_vec(&mut e, from_block + 0x10 * axis as u32, from_lanes);
                store_vec(&mut e, to_block + 0x10 * axis as u32, to_lanes);
            }
            // The collector records, for every call of slot 4, the id and the four lane masks.
            e.put_vtable(0x0900_0000, &[0, ID_SLOT, FINISH_SLOT, LENGTH_SLOT]);
            let object = e.mem.alloc(0x10);
            e.mem.set_u32(object, 0x0900_0000);
            let reports: Rc<RefCell<Vec<(u32, [u32; 4])>>> = Rc::new(RefCell::new(Vec::new()));
            let log = Rc::clone(&reports);
            e.register_double(ID_SLOT, move |e, a| {
                let masks = [0, 1, 2, 3].map(|k| e.mem.u32(a[2] + 4 * k));
                log.borrow_mut().push((a[1], masks));
                Ret::default()
            });
            e.register_double(FINISH_SLOT, |_, _| Ret::default());
            let query = query(&mut e, root, base);
            let done = e.mem.alloc(8);
            run_00d99570(&mut e, [done, query, from_block, to_block, object, 0], 0);
            let mut found = std::collections::BTreeSet::new();
            for (id, masks) in reports.borrow().iter() {
                for (ray, mask) in masks.iter().enumerate() {
                    if *mask != 0 {
                        found.insert((*id, ray));
                    }
                }
            }
            let mut truth = std::collections::BTreeSet::new();
            for i in 0..8u32 {
                let words: Vec<u16> = (0..8).map(|k| e.mem.u16(base + 16 * i + 2 * k)).collect();
                for (ray, (from, to)) in rays.iter().enumerate() {
                    if segment_touches(*from, *to, &words) {
                        truth.insert((i + 1, ray));
                    }
                }
            }
            assert_eq!(found, truth, "{rays:?}");
            reported_pairs += found.len();
        }
        assert!(reported_pairs > 20, "only {reported_pairs} hits");
    }

    /// Four copies of one ray in the packet layout: three vectors (x, y, z lanes) each.
    fn packet(e: &mut Engine, origin: [f32; 3]) -> u32 {
        let block = e.mem.alloc(0x30);
        for axis in 0..3u32 {
            store_vec(e, block + 0x10 * axis, [origin[axis as usize]; 4]);
        }
        block
    }

    struct Packet {
        e: Engine,
        log: Log,
        router: u32,
        region: u32,
        done: u32,
        returned: u32,
    }

    fn run_packet(from: [f32; 3], to: [f32; 3], fill: u32) -> Packet {
        let mut e = Engine::new();
        constants(&mut e);
        let (router, region) = router(&mut e);
        let (object, log) = collector(&mut e, 1000.0);
        let base = one_box(&mut e);
        let root = leaf(&mut e, 0, 1);
        let query = query(&mut e, root, base);
        let (from, to) = (packet(&mut e, from), packet(&mut e, to));
        let done = e.mem.alloc(8);
        // The lane masks the exe fills in at start-up: all four rays active.
        let all = [f32::from_bits(u32::MAX); 4];
        store_vec(&mut e, 0x0126_8460, all);
        store_vec(&mut e, 0x0126_8550, all);
        let returned = run_00d99570(&mut e, [done, query, from, to, object, 0], fill);
        Packet {
            e,
            log,
            router,
            region,
            done,
            returned,
        }
    }

    #[test]
    fn four_rays_through_the_box_reach_the_collector() {
        let run = run_packet([-5.0, 6.0, 6.0], [5.0, 6.0, 6.0], 0);
        assert_eq!(run.returned, run.done);
        assert_eq!(run.e.mem.u8(run.done), 1);
        assert_eq!(hit_ids(&run.log, 1), [0x1234_5678]);
        assert_eq!(run.e.mem.u32(run.router), run.region);
    }

    #[test]
    fn four_rays_beside_the_box_find_nothing() {
        let run = run_packet([-5.0, 20.0, 6.0], [5.0, 20.0, 6.0], 0);
        assert_eq!(run.e.mem.u8(run.done), 1);
        assert!(run.log.borrow().iter().all(|c| c[0] != 4));
        assert_eq!(run.e.mem.u32(run.router), run.region);
    }

    #[test]
    fn the_packet_walk_does_not_depend_on_what_the_frame_held() {
        for ray in [
            ([-5.0, 6.0, 6.0], [5.0, 6.0, 6.0]),
            ([-5.0, 20.0, 6.0], [5.0, 20.0, 6.0]),
        ] {
            let quiet = run_packet(ray.0, ray.1, 0);
            for fill in [0xdead_beef, 0xffff_ffff, 0x3f80_0000] {
                let noisy = run_packet(ray.0, ray.1, fill);
                let a: Vec<u32> = quiet.log.borrow().iter().map(|c| c[0]).collect();
                let b: Vec<u32> = noisy.log.borrow().iter().map(|c| c[0]).collect();
                assert_eq!(a, b, "{ray:?} {fill:#x}");
            }
        }
    }
}

#[cfg(test)]
mod gap1_tests {
    use super::*;

    #[test]
    fn fn_00d9a270_stores_the_vtables_and_frees_0x20_bytes() {
        let mut e = Engine::new();
        let mut slots = [0u32; 4];
        slots[2] = 0x0f00_0008;
        e.put_vtable(0x0f00_1000, &slots);
        e.register(0x0f00_0008, |_, _| Ret::default());
        let allocator = e.mem.alloc(4);
        e.mem.set_u32(allocator, 0x0f00_1000);
        let router = e.mem.alloc(0x14);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| router.into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d9_a270, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), DESTROYED_VTABLE);
        assert_eq!(e.mem.u32(this + 4), VTABLE_FIRST_BASE);
        assert_eq!(e.call(0x00d9_a270, &args![this, 1u32]).u32(), this);
        let freed: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x0f00_0008)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(freed, vec![vec![allocator, this, 0x20]]);
    }
}
