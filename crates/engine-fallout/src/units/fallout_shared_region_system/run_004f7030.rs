//! `fallout shared/region system/run_004f7030` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006815c0` (`thiscall`): a no-op constructor step (answers `this`).
const TRIVIAL_CONSTRUCT: u32 = 0x0068_15c0;
/// `0052ce20` / `004f5d90` (`thiscall`, `RET 4`, a float): set a region
/// point's x / y.
const POINT_SET_X: u32 = 0x0052_ce20;
const POINT_SET_Y: u32 = 0x004f_5d90;
/// `006a7f50` / `006b9130` (`thiscall`): a point's x / y (x87 `ST0`).
const POINT_X: u32 = 0x006a_7f50;
const POINT_Y: u32 = 0x006b_9130;
/// The double constant the segment test compares with (`01012060`, 0.0 in
/// the exe).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `0096a2d0` (`thiscall`): a `BSSimpleList` node constructor.
const SIMPLE_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// The float constants `004f7680` copies (`01016970` into +0x10 and +0x14,
/// `01015f5c` into +0x18 and +0x1c).
const BOUND_MIN: u32 = 0x0101_6970;
const BOUND_MAX: u32 = 0x0101_5f5c;
/// `004f7dd0` (`thiscall`): empties the point list; `00401030` frees;
/// `0046ffb0` the `BSSimpleList` destructor.
const POINT_LIST_CLEAR: u32 = 0x004f_7dd0;
const FREE: u32 = 0x0040_1030;
const SIMPLE_LIST_DESTRUCT: u32 = 0x0046_ffb0;

/// The x87 condition bits (`AH` after `FNSTSW AX`: C0 0x01, C2 0x04, C3
/// 0x40) of comparing `st0` with `source`.
fn fcom_status(st0: f64, source: f64) -> u8 {
    if st0.is_nan() || source.is_nan() {
        0x45
    } else if st0 < source {
        0x01
    } else if st0 == source {
        0x40
    } else {
        0
    }
}

/// `TEST AH,mask` then `JP`: the parity flag of the masked bits (set for an
/// even number of ones).
fn parity(status: u8, mask: u8) -> bool {
    (status & mask).count_ones() % 2 == 0
}

// Translated from 004f7030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a region point from another's two floats
/// (`thiscall`, `RET 4`, the source's address): `006815c0`, then x
/// (`0052ce20`) and y (`004f5d90`) set from the source's +0 and +4 (each a
/// `FLD`/`FSTP` copy). Answers `this`.
pub fn fn_004f7030(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(TRIVIAL_CONSTRUCT, &args![this]);
    let x = x87_quiet_bits(e.mem.u32(source.addr()));
    e.call(POINT_SET_X, &args![this, x]);
    let y = x87_quiet_bits(e.mem.u32(source.addr().wrapping_add(4)));
    e.call(POINT_SET_Y, &args![this, y]);
    this
}

// Translated from 004f7070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a region point from x and y (`thiscall`,
/// `RET 8`): `006815c0`, then `0052ce20` and `004f5d90` with the two floats
/// (`FLD`/`FSTP` copies). Answers `this`.
pub fn fn_004f7070(e: &mut Engine, this: Ptr, x: f32, y: f32) -> Ptr {
    e.call(TRIVIAL_CONSTRUCT, &args![this]);
    e.call(POINT_SET_X, &args![this, x87_quiet_bits(x.to_bits())]);
    e.call(POINT_SET_Y, &args![this, x87_quiet_bits(y.to_bits())]);
    this
}

// Translated from 004f72a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: two points and a y): the x at which
/// the line through the points reaches `y`, from the second point:
/// `x1 + (x0 - x1) / (y0 - y1) * (y1 - y)` in the exe's x87 order (both
/// point coordinates of the second point stored as floats first). 0.0
/// (`FLDZ`) without either point.
pub fn fn_004f72a0(e: &mut Engine, first: Ptr, second: Ptr, y: f32) -> f32 {
    if first.is_null() || second.is_null() {
        return 0.0;
    }
    let x1 = x87_store32(e.call(POINT_X, &args![second]).f64());
    let y1 = x87_store32(e.call(POINT_Y, &args![second]).f64());
    let drop = x87_sub(x87_load32(y1), x87_m32(y.to_bits()));
    let dx = x87_sub(e.call(POINT_X, &args![first]).f64(), x87_m32(x1));
    let dy = x87_sub(e.call(POINT_Y, &args![first]).f64(), x87_m32(y1));
    let slope = x87_div(dx, dy);
    let offset = x87_mul(slope, drop);
    let x = x87_sub(x87_m32(x1), offset);
    f32::from_bits(x87_store32(x))
}

// Translated from 004f7310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: four points): whether the segment
/// from the first to the second point meets the one from the third to the
/// fourth. False when a point is null. The bounding ranges are tested first
/// in x, then in y (each segment's extent from the sign of its difference,
/// the other's ends compared with the exe's `FCOMPP` tests, NaN passing);
/// then with `denominator = dy1 * dx2 - dx1 * dy2` (the differences stored
/// as floats, `dx1 = x1 - x0`, `dx2 = x2 - x3`, and the same for y) a zero
/// denominator answers true, and otherwise both numerators
/// (`dx1 * ey - dy1 * ex` and `dy2 * ex - dx2 * ey`, `ex = x0 - x2`,
/// `ey = y0 - y2`) must lie between 0 and the denominator on its side.
pub fn fn_004f7310(e: &mut Engine, p0: Ptr, p1: Ptr, p2: Ptr, p3: Ptr) -> bool {
    if p0.is_null() || p1.is_null() || p2.is_null() || p3.is_null() {
        return false;
    }
    let zero = f64::from_bits(e.mem.u64(ZERO_DOUBLE));
    // Ranges per axis: `getter` is the x or y getter.
    let mut differences = [0u32; 4];
    for (axis, getter) in [POINT_X, POINT_Y].into_iter().enumerate() {
        let a1 = e.call(getter, &args![p1]).f64();
        let a0 = e.call(getter, &args![p0]).f64();
        let d1 = x87_store32(x87_sub(a1, a0));
        let a2 = e.call(getter, &args![p2]).f64();
        let a3 = e.call(getter, &args![p3]).f64();
        let d2 = x87_store32(x87_sub(a2, a3));
        differences[axis * 2] = d1;
        differences[axis * 2 + 1] = d2;
        let (high, low) = if parity(fcom_status(x87_load32(d1), zero), 0x05) {
            let high = x87_store32(e.call(getter, &args![p1]).f64());
            let low = x87_store32(e.call(getter, &args![p0]).f64());
            (high, low)
        } else {
            let high = x87_store32(e.call(getter, &args![p0]).f64());
            let low = x87_store32(e.call(getter, &args![p1]).f64());
            (high, low)
        };
        let (first, second) = if fcom_status(x87_load32(d2), zero) & 0x41 == 0 {
            (p3, p2)
        } else {
            (p2, p3)
        };
        let end = e.call(getter, &args![first]).f64();
        if !parity(fcom_status(x87_load32(high), end), 0x05) {
            return false;
        }
        let end = e.call(getter, &args![second]).f64();
        if fcom_status(x87_load32(low), end) & 0x41 == 0 {
            return false;
        }
    }
    let [dx1, dx2, dy1, dy2] = differences;
    let x0 = e.call(POINT_X, &args![p0]).f64();
    let x2 = e.call(POINT_X, &args![p2]).f64();
    let ex = x87_store32(x87_sub(x0, x2));
    let y0 = e.call(POINT_Y, &args![p0]).f64();
    let y2 = e.call(POINT_Y, &args![p2]).f64();
    let ey = x87_store32(x87_sub(y0, y2));
    let product = |a: u32, b: u32| x87_mul(x87_load32(a), x87_m32(b));
    let denominator = x87_store32(x87_sub(product(dy1, dx2), product(dx1, dy2)));
    let first = x87_store32(x87_sub(product(dx1, ey), product(dy1, ex)));
    let second = x87_store32(x87_sub(product(dy2, ex), product(dx2, ey)));
    let denominator_value = x87_load32(denominator);
    let status = fcom_status(denominator_value, zero);
    if !parity(status, 0x44) {
        return true;
    }
    for numerator in [second, first] {
        let value = x87_load32(numerator);
        if status & 0x41 == 0 {
            if !parity(fcom_status(value, zero), 0x05) {
                return false;
            }
            if !parity(fcom_status(denominator_value, value), 0x05) {
                return false;
            }
        } else {
            if fcom_status(value, zero) & 0x41 == 0 {
                return false;
            }
            if fcom_status(denominator_value, value) & 0x41 == 0 {
                return false;
            }
        }
    }
    true
}

// Translated from 004f7680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a point-list object's constructor
/// (`thiscall`, `RET 4`, a byte): the list (`0096a2d0`), the two corners
/// (`006815c0` at +0x10 and +0x18), the byte at +0xc, 0x400 at +0x20, 0 at
/// +0x24, the corner floats from `01016970` (+0x14, +0x10) and `01015f5c`
/// (+0x1c, +0x18) as `FLD`/`FSTP` copies, and no array (+8). Answers `this`.
// Translation incomplete: 004f7680 - the SEH frame (FS:[0], unwind of the list) is not translated.
pub fn fn_004f7680(e: &mut Engine, this: Ptr, kind: u8) -> Ptr {
    let base = this.addr();
    e.call(SIMPLE_LIST_CONSTRUCT, &args![this]);
    e.call(TRIVIAL_CONSTRUCT, &args![base.wrapping_add(0x10)]);
    e.call(TRIVIAL_CONSTRUCT, &args![base.wrapping_add(0x18)]);
    e.mem.set_u8(base.wrapping_add(0xc), kind);
    e.mem.set_u32(base.wrapping_add(0x20), 0x400);
    e.mem.set_u32(base.wrapping_add(0x24), 0);
    for (offset, constant) in [
        (0x14u32, BOUND_MIN),
        (0x10, BOUND_MIN),
        (0x1c, BOUND_MAX),
        (0x18, BOUND_MAX),
    ] {
        let value = x87_quiet_bits(e.global::<u32>(constant));
        e.mem.set_u32(base.wrapping_add(offset), value);
    }
    e.mem.set_u32(base.wrapping_add(8), 0);
    this
}

// Translated from 004f7740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: that object's destructor body: empties the
/// list (`004f7dd0`), frees the array at +8 when set (`00401030`), then
/// destroys the list (`0046ffb0`).
// Translation incomplete: 004f7740 - the SEH frame (FS:[0], unwind of the list) is not translated.
pub fn fn_004f7740(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(POINT_LIST_CLEAR, &args![this]);
    let array = e.mem.u32(base.wrapping_add(8));
    if array != 0 {
        e.call(FREE, &args![array]);
    }
    e.call(SIMPLE_LIST_DESTRUCT, &args![this]);
}

/// The float tolerance the point comparisons pass (`01017d00`).
const TOLERANCE: u32 = 0x0101_7d00;
/// The float `-1.0` answered without a second point (`01012054`).
const MINUS_ONE: u32 = 0x0101_2054;
/// `0040eb10` (cdecl: two floats and a tolerance): whether they are equal
/// within the tolerance.
const NEAR_EQUAL: u32 = 0x0040_eb10;
/// `004019d0` (cdecl: a float): a float routine whose result is in `ST0`
/// (the square root by its use for a distance; unconfirmed).
const FLOAT_ROUTINE_004019D0: u32 = 0x0040_19d0;
/// `TESRegionPoint::TESRegionPoint` (`004f6fc0`, `thiscall`).
const REGION_POINT_CONSTRUCT: u32 = 0x004f_6fc0;
/// The game's allocator (`00401000`, cdecl: size).
const ALLOCATE: u32 = 0x0040_1000;
/// `00485910` (cdecl: chunk, word): adds a one-word chunk.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// `TESForm::AddChunkArray_ov2` (`00485710`, cdecl: chunk, data, count).
const ADD_CHUNK_ARRAY: u32 = 0x0048_5710;
/// `008256d0` (`thiscall`): the end-of-list test on a node.
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `00726070` (`thiscall`): the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// The array constructor helper (`00401050`, `stdcall`: array, element
/// size, count, constructor).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
/// `00401460` (cdecl: destination, source, size): copies bytes.
const COPY_BYTES: u32 = 0x0040_1460;
/// `RPLI` and `RPLD`.
const CHUNK_RPLI: u32 = 0x494c_5052;
const CHUNK_RPLD: u32 = 0x444c_5052;

// Translated from 004f70b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether the two
/// points' x and then y are equal within the tolerance at `01017d00`
/// (`0040eb10` with this point's coordinate, the other's, the tolerance;
/// each stored as a float first).
pub fn fn_004f70b0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    for getter in [POINT_X, POINT_Y] {
        let tolerance = x87_quiet_bits(e.global::<u32>(TOLERANCE));
        let theirs = x87_store32(e.call(getter, &args![other]).f64());
        let mine = x87_store32(e.call(getter, &args![this]).f64());
        if !e.call(NEAR_EQUAL, &args![mine, theirs, tolerance]).bool() {
            return false;
        }
    }
    true
}

// Translated from 004f7130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): copies this point's x
/// and y (each stored as a float) into `target` (`0052ce20`, `004f5d90`)
/// and answers it. Without a target a new 8-byte point is made
/// (`00401000`, `TESRegionPoint::TESRegionPoint`; null when the allocation
/// fails).
// Translation incomplete: 004f7130 - the C++ exception frame (FS:[0] chain, unwind state 0 that frees the new point) and the /GS cookie word are not translated.
pub fn fn_004f7130(e: &mut Engine, this: Ptr, target: u32) -> u32 {
    let target = if target != 0 {
        target
    } else {
        let block = e.call(ALLOCATE, &args![8u32]).u32();
        if block == 0 {
            0
        } else {
            e.call(REGION_POINT_CONSTRUCT, &args![block]).u32()
        }
    };
    let x = x87_store32(e.call(POINT_X, &args![this]).f64());
    e.call(POINT_SET_X, &args![target, x]);
    let y = x87_store32(e.call(POINT_Y, &args![this]).f64());
    e.call(POINT_SET_Y, &args![target, y]);
    target
}

// Translated from 004f71f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the float `-1.0` at
/// `01012054` without a second point; otherwise `004019d0` of the squared
/// distance (`004f7230`, passed as a float), its `ST0` result.
pub fn fn_004f71f0(e: &mut Engine, this: Ptr, other: Ptr) -> f64 {
    if other.is_null() {
        return x87_load32(e.global::<u32>(MINUS_ONE));
    }
    let squared = fn_004f7230(e, this, other);
    let pushed = x87_store32(f64::from(squared));
    e.call(FLOAT_ROUTINE_004019D0, &args![pushed]).f64()
}

// Translated from 004f7230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the squared distance
/// between the points, `dx * dx + dy * dy` with `dx = x - other.x` and
/// `dy = y - other.y` stored as floats, the sum stored as a float. The
/// float `-1.0` at `01012054` without a second point.
pub fn fn_004f7230(e: &mut Engine, this: Ptr, other: Ptr) -> f32 {
    if other.is_null() {
        return f32::from_bits(x87_quiet_bits(e.global::<u32>(MINUS_ONE)));
    }
    let mine = e.call(POINT_X, &args![this]).f64();
    let theirs = e.call(POINT_X, &args![other]).f64();
    let dx = x87_store32(x87_sub(mine, theirs));
    let mine = e.call(POINT_Y, &args![this]).f64();
    let theirs = e.call(POINT_Y, &args![other]).f64();
    let dy = x87_store32(x87_sub(mine, theirs));
    let square_x = x87_mul(x87_load32(dx), x87_m32(dx));
    let square_y = x87_mul(x87_load32(dy), x87_m32(dy));
    let sum = x87_store32(x87_add(square_x, square_y));
    f32::from_bits(x87_quiet_bits(sum))
}

// Translated from 004f7620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: a point, a float):
/// false without a point; otherwise whether `value` equals the float at
/// +8 within the tolerance (`0040eb10`) and the point matches this one
/// (`004f70b0` on the point with this one).
pub fn fn_004f7620(e: &mut Engine, this: Ptr, other: Ptr, value: f32) -> bool {
    if other.is_null() {
        return false;
    }
    let tolerance = x87_quiet_bits(e.global::<u32>(TOLERANCE));
    let own = x87_quiet_bits(e.mem.u32(this.addr() + 8));
    let value = x87_quiet_bits(value.to_bits());
    if !e.call(NEAR_EQUAL, &args![value, own, tolerance]).bool() {
        return false;
    }
    fn_004f70b0(e, other, this)
}

/// `004f77c0`'s array: `elements` 8-byte points (`00401000` with
/// `elements * 8`, `0xffffffff` when that overflows, then the array
/// constructor with `006815c0`); null when the allocation fails.
fn allocate_points(e: &mut Engine, elements: u32) -> u32 {
    let wide = u64::from(elements) * 8;
    let size = if wide > u64::from(u32::MAX) {
        u32::MAX
    } else {
        wide as u32
    };
    let block = e.call(ALLOCATE, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        VECTOR_CONSTRUCT,
        &args![block, 8u32, elements, TRIVIAL_CONSTRUCT],
    );
    block
}

// Translated from 004f77c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): saves a point list: the `RPLI`
/// chunk with the word at +0x20 (`00485910`), then, unless the list (from
/// `this` as its first node) is empty (`008256d0`), its points' two
/// floats gathered into an array (0x2000 points allocated for a capacity
/// of 0x400; when full the capacity grows by 0x400, a new array of eight
/// times the capacity is made, the points copied over with `00401460` and
/// the old array freed), written as the `RPLD` chunk (`00485710` with
/// twice the point count, when there are points) and freed.
// Translation incomplete: 004f77c0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the arrays) and the /GS cookie word are not translated.
pub fn fn_004f77c0(e: &mut Engine, this: Ptr) {
    let index = e.mem.u32(this.addr() + 0x20);
    e.call(ADD_CHUNK_WORD, &args![CHUNK_RPLI, index]);
    let mut node = this.addr();
    if e.call(NODE_IS_EMPTY, &args![node]).bool() {
        return;
    }
    let mut count = 0u32;
    let mut grows = 0u32;
    let mut capacity = 0x400u32;
    let mut array = allocate_points(e, capacity << 3);
    while node != 0 {
        if e.call(NODE_IS_EMPTY, &args![node]).bool() {
            break;
        }
        let item_address = e.call(TRIVIAL_CONSTRUCT, &args![node]).u32();
        let item = e.mem.u32(item_address);
        let point = e.call(TRIVIAL_CONSTRUCT, &args![item]).u32();
        let x = e.mem.u32(point);
        let y = e.mem.u32(point.wrapping_add(4));
        let slot = array.wrapping_add(count.wrapping_mul(8));
        e.mem.set_u32(slot, x);
        e.mem.set_u32(slot.wrapping_add(4), y);
        count = count.wrapping_add(1);
        if count >= capacity {
            grows = grows.wrapping_add(1);
            capacity = grows.wrapping_add(1) << 10;
            let old = array;
            array = allocate_points(e, capacity << 3);
            e.call(COPY_BYTES, &args![array, old, count << 3]);
            e.call(FREE, &args![old]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    if count != 0 {
        e.call(ADD_CHUNK_ARRAY, &args![CHUNK_RPLD, array, count << 1]);
    }
    e.call(FREE, &args![array]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_70b0, fn_004f70b0(Ptr, Ptr) -> bool),
        entry!(0x004f_7130, fn_004f7130(Ptr, u32) -> u32),
        entry!(0x004f_71f0, fn_004f71f0(Ptr, Ptr) -> f64),
        entry!(0x004f_7230, fn_004f7230(Ptr, Ptr) -> f32),
        entry!(0x004f_7620, fn_004f7620(Ptr, Ptr, f32) -> bool),
        entry!(0x004f_77c0, fn_004f77c0(Ptr)),
        entry!(0x004f_7030, fn_004f7030(Ptr, Ptr) -> Ptr),
        entry!(0x004f_7070, fn_004f7070(Ptr, f32, f32) -> Ptr),
        entry!(0x004f_72a0, fn_004f72a0(Ptr, Ptr, f32) -> f32),
        entry!(0x004f_7310, fn_004f7310(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x004f_7680, fn_004f7680(Ptr, u8) -> Ptr),
        entry!(0x004f_7740, fn_004f7740(Ptr)),
    ]
}

#[cfg(test)]
mod b025_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x100);
        e.register(POINT_X, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0])),
            ..Ret::default()
        });
        e.register(POINT_Y, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 4)),
            ..Ret::default()
        });
        e.register(TRIVIAL_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(POINT_SET_X, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINT_SET_Y, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e
    }

    fn point(e: &mut Engine, x: f32, y: f32) -> u32 {
        let p = e.mem.alloc(8);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        p
    }

    #[test]
    fn region_points_from_floats() {
        let mut e = engine();
        let source = point(&mut e, 1.5, -2.0);
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x004f_7030, &args![this, source]).u32(), this);
        assert_eq!((e.mem.f32(this), e.mem.f32(this + 4)), (1.5, -2.0));
        // A signalling NaN is quieted by the FLD/FSTP copy.
        e.mem.set_u32(source, 0x7f80_0001);
        e.call(0x004f_7030, &args![this, source]);
        assert_eq!(e.mem.u32(this), 0x7fc0_0001);
        assert_eq!(
            e.call(0x004f_7070, &args![this, 3.0f32, 4.0f32]).u32(),
            this
        );
        assert_eq!((e.mem.f32(this), e.mem.f32(this + 4)), (3.0, 4.0));
    }

    #[test]
    fn fn_004f72a0_interpolates_x_at_y() {
        let mut e = engine();
        let a = point(&mut e, 0.0, 0.0);
        let b = point(&mut e, 10.0, 20.0);
        // From b down the line to y = 5: x = 10 + (0 - 10) / (0 - 20) * (20 - 5).
        let r = e.call(0x004f_72a0, &args![a, b, 5.0f32]).f64();
        assert_eq!(r, 2.5);
        assert_eq!(e.call(0x004f_72a0, &args![0u32, b, 5.0f32]).f64(), 0.0);
        assert_eq!(e.call(0x004f_72a0, &args![a, 0u32, 5.0f32]).f64(), 0.0);
    }

    fn crosses(e: &mut Engine, s: [(f32, f32); 4]) -> bool {
        let p: Vec<u32> = s.iter().map(|(x, y)| point(e, *x, *y)).collect();
        e.call(0x004f_7310, &args![p[0], p[1], p[2], p[3]]).bool()
    }

    #[test]
    fn fn_004f7310_tests_segment_intersection() {
        let mut e = engine();
        // Crossing diagonals.
        assert!(crosses(
            &mut e,
            [(0.0, 0.0), (2.0, 2.0), (0.0, 2.0), (2.0, 0.0)]
        ));
        // The same with the segments' ends swapped.
        assert!(crosses(
            &mut e,
            [(2.0, 2.0), (0.0, 0.0), (2.0, 0.0), (0.0, 2.0)]
        ));
        // Disjoint in x.
        assert!(!crosses(
            &mut e,
            [(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (3.0, 1.0)]
        ));
        // Disjoint in y.
        assert!(!crosses(
            &mut e,
            [(0.0, 0.0), (1.0, 1.0), (0.0, 2.0), (1.0, 3.0)]
        ));
        // Boxes overlap but the segments miss.
        assert!(!crosses(
            &mut e,
            [(0.0, 0.0), (2.0, 2.0), (1.5, 0.0), (3.0, 1.0)]
        ));
        // Parallel (zero denominator) with overlapping boxes: true.
        assert!(crosses(
            &mut e,
            [(0.0, 0.0), (2.0, 2.0), (1.0, 1.0), (3.0, 3.0)]
        ));
        // Touching at an end.
        assert!(crosses(
            &mut e,
            [(0.0, 0.0), (1.0, 1.0), (1.0, 1.0), (2.0, 0.0)]
        ));
        // A null point.
        let p = point(&mut e, 0.0, 0.0);
        assert!(!e.call(0x004f_7310, &args![p, p, p, 0u32]).bool());
    }

    #[test]
    fn point_list_object_construct_and_destroy() {
        let mut e = engine();
        e.map(0x0101_5000, 0x2000);
        e.set_global::<f32>(BOUND_MIN, 3.0e38);
        e.set_global::<f32>(BOUND_MAX, -3.0e38);
        e.register(SIMPLE_LIST_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x28);
        e.mem.write(this, &[0xee; 0x28]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_7680, &args![this, 7u32]).u32(), this);
        assert_eq!(
            calls_to(&e, TRIVIAL_CONSTRUCT),
            vec![vec![this + 0x10], vec![this + 0x18]]
        );
        assert_eq!(e.mem.u8(this + 0xc), 7);
        assert_eq!(e.mem.u32(this + 0x20), 0x400);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(
            [e.mem.f32(this + 0x10), e.mem.f32(this + 0x14)],
            [3.0e38; 2]
        );
        assert_eq!(
            [e.mem.f32(this + 0x18), e.mem.f32(this + 0x1c)],
            [-3.0e38; 2]
        );
        for addr in [POINT_LIST_CLEAR, FREE, SIMPLE_LIST_DESTRUCT] {
            e.register(addr, |_, _| Ret::default());
        }
        e.mem.set_u32(this + 8, 0x888);
        e.call_log = Some(vec![]);
        e.call(0x004f_7740, &args![this]);
        let order: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (POINT_LIST_CLEAR, vec![this]),
                (FREE, vec![0x888]),
                (SIMPLE_LIST_DESTRUCT, vec![this]),
            ]
        );
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_7740, &args![this]);
        assert!(calls_to(&e, FREE).is_empty());
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Points are two floats; the tolerance is 0.01 and `-1.0` is at
    /// `01012054`; `0040eb10` compares within the tolerance.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x100);
        e.map(0x0101_7000, 0x1000);
        e.set_global::<f32>(TOLERANCE, 0.01);
        e.set_global::<f32>(MINUS_ONE, -1.0);
        e.register(POINT_X, |e, a| f64::from(e.mem.f32(a[0])).into_ret());
        e.register(POINT_Y, |e, a| f64::from(e.mem.f32(a[0] + 4)).into_ret());
        e.register(TRIVIAL_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(POINT_SET_X, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINT_SET_Y, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(NEAR_EQUAL, |_, a| {
            let (x, y, t) = (
                f32::from_bits(a[0]),
                f32::from_bits(a[1]),
                f32::from_bits(a[2]),
            );
            u32::from((x - y).abs() <= t).into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn point(e: &mut Engine, x: f32, y: f32) -> u32 {
        let p = e.mem.alloc(0xc);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        p
    }

    #[test]
    fn fn_004f70b0_compares_both_coordinates() {
        let mut e = engine();
        let a = point(&mut e, 1.0, 2.0);
        let b = point(&mut e, 1.005, 2.0);
        let c = point(&mut e, 1.0, 2.5);
        let d = point(&mut e, 1.5, 2.0);
        assert!(e.call(0x004f_70b0, &args![a, b]).bool());
        assert!(!e.call(0x004f_70b0, &args![a, c]).bool());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_70b0, &args![a, d]).bool());
        assert_eq!(
            calls_to(&e, NEAR_EQUAL),
            vec![vec![1.0f32.to_bits(), 1.5f32.to_bits(), 0.01f32.to_bits()]]
        );
        assert!(calls_to(&e, POINT_Y).is_empty());
    }

    #[test]
    fn fn_004f7130_copies_into_a_target_or_a_new_point() {
        let mut e = engine();
        let a = point(&mut e, 1.5, -2.5);
        let target = point(&mut e, 0.0, 0.0);
        assert_eq!(e.call(0x004f_7130, &args![a, target]).u32(), target);
        assert_eq!((e.mem.f32(target), e.mem.f32(target + 4)), (1.5, -2.5));
        let made = e.mem.alloc(8);
        e.register_double(ALLOCATE, move |_, _| made.into_ret());
        e.register(REGION_POINT_CONSTRUCT, |_, a| a[0].into_ret());
        assert_eq!(e.call(0x004f_7130, &args![a, 0u32]).u32(), made);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![8]]);
        assert_eq!(e.mem.f32(made + 4), -2.5);
    }

    #[test]
    fn fn_004f7230_and_004f71f0_measure_the_distance() {
        let mut e = engine();
        e.register(FLOAT_ROUTINE_004019D0, |_, a| {
            f64::from(f32::from_bits(a[0])).sqrt().into_ret()
        });
        let a = point(&mut e, 1.0, 2.0);
        let b = point(&mut e, 4.0, 6.0);
        assert_eq!(e.call(0x004f_7230, &args![a, b]).f32(), 25.0);
        assert_eq!(e.call(0x004f_7230, &args![a, 0u32]).f32(), -1.0);
        assert_eq!(e.call(0x004f_71f0, &args![a, b]).f64(), 5.0);
        assert_eq!(e.call(0x004f_71f0, &args![a, 0u32]).f64(), -1.0);
        assert_eq!(
            calls_to(&e, FLOAT_ROUTINE_004019D0),
            vec![vec![25.0f32.to_bits()]]
        );
    }

    #[test]
    fn fn_004f7620_checks_the_value_then_the_point() {
        let mut e = engine();
        let this = point(&mut e, 1.0, 2.0);
        e.mem.set_f32(this + 8, 7.0);
        let same = point(&mut e, 1.0, 2.0);
        let other = point(&mut e, 3.0, 2.0);
        assert!(!e.call(0x004f_7620, &args![this, 0u32, 7.0f32]).bool());
        assert!(e.call(0x004f_7620, &args![this, same, 7.0f32]).bool());
        assert!(!e.call(0x004f_7620, &args![this, same, 8.0f32]).bool());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_7620, &args![this, other, 7.0f32]).bool());
        let compared = calls_to(&e, NEAR_EQUAL);
        assert_eq!(
            compared[0],
            vec![7.0f32.to_bits(), 7.0f32.to_bits(), 0.01f32.to_bits()]
        );
        // The point check runs on the other point with this one.
        assert_eq!(compared[1][0], 3.0f32.to_bits());
    }

    type Chunks = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn save(points: usize) -> (Engine, Chunks, Vec<Vec<u32>>) {
        let mut e = engine();
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0].min(0x20_0000)).into_ret());
        e.register(COPY_BYTES, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(FREE, |_, _| Ret::default());
        e.register(NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let chunks: Chunks = Rc::default();
        let log = chunks.clone();
        e.register_double(ADD_CHUNK_WORD, move |_, a| {
            log.borrow_mut().push((a[0], vec![a[1]]));
            Ret::default()
        });
        let log = chunks.clone();
        e.register_double(ADD_CHUNK_ARRAY, move |e, a| {
            let words = (0..a[2]).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut().push((a[0], words));
            Ret::default()
        });
        // The list object: its first node at +0 (item, next), +0x20 the
        // word saved in `RPLI`.
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x20, 0x42);
        let mut node = this;
        for i in 0..points {
            let p = point(&mut e, i as f32, -(i as f32));
            e.mem.set_u32(node, p);
            if i + 1 < points {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        e.call_log = Some(vec![]);
        e.call(0x004f_77c0, &args![this]);
        let frees = calls_to(&e, FREE);
        (e, chunks, frees)
    }

    #[test]
    fn fn_004f77c0_saves_the_points() {
        let (e, chunks, frees) = save(2);
        let chunks = chunks.borrow();
        assert_eq!(chunks[0], (CHUNK_RPLI, vec![0x42]));
        assert_eq!(
            chunks[1],
            (
                CHUNK_RPLD,
                vec![
                    0.0f32.to_bits(),
                    (-0.0f32).to_bits(),
                    1.0f32.to_bits(),
                    (-1.0f32).to_bits()
                ]
            )
        );
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x10000]]);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT)[0][1..],
            [8, 0x2000, TRIVIAL_CONSTRUCT]
        );
        assert_eq!(frees.len(), 1);
    }

    #[test]
    fn fn_004f77c0_without_points_writes_only_the_index() {
        let (e, chunks, _) = save(0);
        assert_eq!(*chunks.borrow(), vec![(CHUNK_RPLI, vec![0x42])]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
    }

    #[test]
    fn fn_004f77c0_grows_the_array() {
        let (e, chunks, frees) = save(0x401);
        let chunks = chunks.borrow();
        assert_eq!(chunks[1].1.len(), 0x802);
        assert_eq!(chunks[1].1[0x800], 1024.0f32.to_bits());
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x10000], vec![0x20000]]);
        assert_eq!(calls_to(&e, COPY_BYTES)[0][2], 0x2000);
        assert_eq!(frees.len(), 2);
    }
}
