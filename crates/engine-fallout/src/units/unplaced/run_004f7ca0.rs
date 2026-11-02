//! `(unplaced)/run_004f7ca0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These functions work on one object whose first eight bytes are the head
//! node of a list of points (node: item at +0, next at +4), with a byte at
//! +0xC that says the list owns its items, the cached bounds at +0x10
//! (smallest of the points' first coordinate), +0x14 (smallest second),
//! +0x18 (largest first), +0x1C (largest second), and the point count at
//! +0x24. The two coordinate getters are `006a7f50` and `006b9130` (each a
//! float in ST0). Together they form a polygon: bounds, a point-inside test
//! and a test over every four consecutive points (`004f7310`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0059bb30` (named `D3DTexture_LockRect` in the engine map, a folded
/// body): the word at +0x24 of its `this` (the point count).
const COUNT_0059BB30: u32 = 0x0059_bb30;
/// `006815c0` (unnamed in the engine map): returns its `this` (a node; the
/// item is its first word).
const NODE_ITEM_006815C0: u32 = 0x0068_15c0;
/// `00726070` (unnamed in the engine map): the node's next (word at +4).
const NODE_FOLLOWING_00726070: u32 = 0x0072_6070;
/// `0063f7b0` (unnamed in the engine map): removes the head node's item
/// from the list.
const REMOVE_HEAD_0063F7B0: u32 = 0x0063_f7b0;
/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;
/// `006a7f50` (unnamed in the engine map): a point's first coordinate (ST0).
const POINT_FIRST_006A7F50: u32 = 0x006a_7f50;
/// `006b9130` (unnamed in the engine map): a point's second coordinate (ST0).
const POINT_SECOND_006B9130: u32 = 0x006b_9130;
/// `004f72a0` (unnamed in the engine map; cdecl, two points and a float):
/// the first coordinate where the edge between the two points meets the
/// given second coordinate (ST0); used by the inside test.
const EDGE_CROSSING_004F72A0: u32 = 0x004f_72a0;
/// `004f7310` (unnamed in the engine map; cdecl, four points): the test
/// run on each four consecutive points.
const FOUR_POINT_TEST_004F7310: u32 = 0x004f_7310;
/// `FLT_MAX` (float), the bound a cleared smallest coordinate starts at.
const FLOAT_MAX: u32 = 0x0101_6970;
/// `-FLT_MAX` (float), the bound a cleared largest coordinate starts at.
const FLOAT_LOWEST: u32 = 0x0101_5f5c;
/// `FLT_MAX` as a double: a cached smallest bound below it is valid.
const DOUBLE_FLOAT_MAX: u32 = 0x0102_31b0;
/// `-FLT_MAX` as a double: a cached largest bound above it is valid.
const DOUBLE_FLOAT_LOWEST: u32 = 0x0102_41b0;

fn count(e: &mut Engine, this: Ptr) -> u32 {
    e.call(COUNT_0059BB30, &args![this]).u32()
}

fn item_of(e: &mut Engine, node: u32) -> u32 {
    let item = e.call(NODE_ITEM_006815C0, &args![node]).u32();
    e.mem.u32(item)
}

fn next_of(e: &mut Engine, node: u32) -> u32 {
    e.call(NODE_FOLLOWING_00726070, &args![node]).u32()
}

/// A coordinate getter's ST0 result as `FSTP dword` leaves it, read back
/// with `FLD dword`.
fn coordinate(e: &mut Engine, getter: u32, point: u32) -> f64 {
    let value = e.call(getter, &args![point]).f64();
    x87_load32(x87_store32(value))
}

/// The shared body of the four bound getters: the cached bound at `offset`
/// when it is valid (below `FLT_MAX` for a smallest bound, above `-FLT_MAX`
/// for a largest one), else the extreme of `getter` over the list's items,
/// from `start` (the cache is not written). Each improvement calls the
/// getter a second time for the value it stores, as the exe does.
fn bound(e: &mut Engine, this: Ptr, offset: u32, getter: u32, smallest: bool) -> f64 {
    let (start, limit) = if smallest {
        (FLOAT_MAX, DOUBLE_FLOAT_MAX)
    } else {
        (FLOAT_LOWEST, DOUBLE_FLOAT_LOWEST)
    };
    let mut best = x87_store32(x87_load32(e.global::<u32>(start)));
    let cached = x87_load32(e.mem.u32(this.addr() + offset));
    let limit = e.global::<f64>(limit);
    let valid = if smallest {
        cached < limit
    } else {
        cached > limit
    };
    if valid {
        return cached;
    }
    let mut node = this.addr();
    while node != 0 && item_of(e, node) != 0 {
        let point = item_of(e, node);
        let value = e.call(getter, &args![point]).f64();
        let current = x87_load32(best);
        let better = if smallest {
            current > value
        } else {
            current < value
        };
        if better {
            let point = item_of(e, node);
            let value = e.call(getter, &args![point]).f64();
            best = x87_store32(value);
        }
        node = next_of(e, node);
    }
    x87_load32(best)
}

// Translated from 004f7dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties the point list. While the count
/// (`0059bb30`) is non-zero: takes the head item, removes it
/// (`0063f7b0`), decrements the count at +0x24, and frees the item when it
/// is non-null and the owning byte at +0xC is set. Then the count is
/// cleared and the bounds reset: +0x14 and +0x10 to `FLT_MAX`, +0x1C and
/// +0x18 to `-FLT_MAX` (copied through the FPU, in that order).
pub fn fn_004f7dd0(e: &mut Engine, this: Ptr) {
    while count(e, this) != 0 {
        let item = item_of(e, this.addr());
        e.call(REMOVE_HEAD_0063F7B0, &args![this]);
        let remaining = e.mem.u32(this.addr() + 0x24).wrapping_sub(1);
        e.mem.set_u32(this.addr() + 0x24, remaining);
        if item != 0 && e.mem.u8(this.addr() + 0xc) != 0 {
            e.call(DEALLOCATE, &args![item]);
        }
    }
    e.mem.set_u32(this.addr() + 0x24, 0);
    let max = x87_quiet_bits(e.global::<u32>(FLOAT_MAX));
    e.mem.set_u32(this.addr() + 0x14, max);
    let max = x87_quiet_bits(e.global::<u32>(FLOAT_MAX));
    e.mem.set_u32(this.addr() + 0x10, max);
    let lowest = x87_quiet_bits(e.global::<u32>(FLOAT_LOWEST));
    e.mem.set_u32(this.addr() + 0x1c, lowest);
    let lowest = x87_quiet_bits(e.global::<u32>(FLOAT_LOWEST));
    e.mem.set_u32(this.addr() + 0x18, lowest);
}

// Translated from 004f7e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the smallest first coordinate (ST0): the
/// cached +0x10 when below `FLT_MAX`, else the minimum of `006a7f50` over
/// the items (`FLT_MAX` for none).
pub fn fn_004f7e80(e: &mut Engine, this: Ptr) -> f64 {
    bound(e, this, 0x10, POINT_FIRST_006A7F50, true)
}

// Translated from 004f7f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the largest first coordinate (ST0): the
/// cached +0x18 when above `-FLT_MAX`, else the maximum of `006a7f50` over
/// the items (`-FLT_MAX` for none).
pub fn fn_004f7f10(e: &mut Engine, this: Ptr) -> f64 {
    bound(e, this, 0x18, POINT_FIRST_006A7F50, false)
}

// Translated from 004f7fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the smallest second coordinate (ST0): the
/// cached +0x14 when below `FLT_MAX`, else the minimum of `006b9130` over
/// the items.
pub fn fn_004f7fa0(e: &mut Engine, this: Ptr) -> f64 {
    bound(e, this, 0x14, POINT_SECOND_006B9130, true)
}

// Translated from 004f8030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the largest second coordinate (ST0): the
/// cached +0x1C when above `-FLT_MAX`, else the maximum of `006b9130` over
/// the items.
pub fn fn_004f8030(e: &mut Engine, this: Ptr) -> f64 {
    bound(e, this, 0x1c, POINT_SECOND_006B9130, false)
}

// Translated from 004f81e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): with `check_count` set
/// and fewer than 3 points, false; otherwise true exactly when the
/// four-point walk `004f8220` (with its byte argument 1) finds nothing.
pub fn fn_004f81e0(e: &mut Engine, this: Ptr, check_count: u8) -> bool {
    if check_count != 0 && count(e, this) < 3 {
        return false;
    }
    !fn_004f8220(e, this, 1)
}

// Translated from 004f8220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): false with fewer than 4
/// points. Otherwise, for each first node `a` with second node `b` (a node
/// that has a next), and each later pair `c`, `d` after `b` (`d` moving on
/// through the list; at the end of the list, with `wrap` set and `a` not
/// the head, `d` becomes the head once), `004f7310(a, b, c, d)` on the four
/// items (all four nodes non-null) answering true makes the result true.
/// After the inner walk, `a` moves to `b`. False when nothing answered.
pub fn fn_004f8220(e: &mut Engine, this: Ptr, wrap: u8) -> bool {
    if count(e, this) < 4 {
        return false;
    }
    let head = this.addr();
    let mut first = head;
    let mut second = next_of(e, first);
    let third = next_of(e, second);
    let _ = next_of(e, third);
    loop {
        if next_of(e, second) == 0 {
            return false;
        }
        let mut third = next_of(e, second);
        let mut fourth = if third != 0 { next_of(e, third) } else { 0 };
        loop {
            let mut last = false;
            if fourth == 0 {
                if wrap != 0 && first != head {
                    fourth = head;
                }
                last = true;
            }
            if first != 0 && second != 0 && third != 0 && fourth != 0 {
                let d = item_of(e, fourth);
                let c = item_of(e, third);
                let b = item_of(e, second);
                let a = item_of(e, first);
                if e.call(FOUR_POINT_TEST_004F7310, &args![a, b, c, d]).u8() != 0 {
                    return true;
                }
            }
            third = fourth;
            if fourth != 0 {
                fourth = next_of(e, fourth);
            }
            if last {
                break;
            }
        }
        first = second;
        second = next_of(e, second);
    }
}

// Translated from 004f8360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether `point` lies
/// inside the polygon. False for a null point, fewer than 3 points, or a
/// point outside the bounds (first coordinate below `004f7e80` or above
/// `004f7f10`, second below `004f7fa0` or above `004f8030`). Otherwise each
/// edge (each node and its next, the last closing to the head) is a
/// candidate when the point's first coordinate is below both ends' first
/// coordinates, or below one of them and below the edge's crossing
/// (`004f72a0` at the point's second coordinate). A candidate counts when
/// the point's second coordinate is below the first end's and the second
/// end's is not above it, or is not below the first end's (ordered) and the
/// second end's is above it. An odd count is inside.
pub fn fn_004f8360(e: &mut Engine, this: Ptr, point: u32) -> bool {
    if point == 0 || count(e, this) < 3 {
        return false;
    }
    let px = coordinate(e, POINT_FIRST_006A7F50, point);
    let py_bits = x87_store32(e.call(POINT_SECOND_006B9130, &args![point]).f64());
    let py = x87_load32(py_bits);
    if px < fn_004f7e80(e, this) {
        return false;
    }
    if px > fn_004f7f10(e, this) {
        return false;
    }
    if py < fn_004f7fa0(e, this) {
        return false;
    }
    if py > fn_004f8030(e, this) {
        return false;
    }
    let head = this.addr();
    let mut crossings: i32 = 0;
    let mut node = head;
    loop {
        let mut next = next_of(e, node);
        if next == 0 {
            next = head;
        }
        let a = item_of(e, node);
        let b = item_of(e, next);
        let ax = coordinate(e, POINT_FIRST_006A7F50, a);
        let bx = coordinate(e, POINT_FIRST_006A7F50, b);
        let hit = if px < ax && px < bx {
            true
        } else if px < ax || px < bx {
            let pushed = x87_store32(x87_load32(py_bits));
            let crossing = e.call(EDGE_CROSSING_004F72A0, &args![a, b, pushed]).f64();
            px < crossing
        } else {
            false
        };
        if hit {
            let ay = coordinate(e, POINT_SECOND_006B9130, a);
            let by = coordinate(e, POINT_SECOND_006B9130, b);
            if (py < ay && by <= py) || (py >= ay && by > py) {
                crossings = crossings.wrapping_add(1);
            }
        }
        node = next;
        if node == head {
            break;
        }
    }
    crossings % 2 != 0
}

/// `004f7230` (`ECX` = a point, one stack word: another point): a distance measure
/// between them (ST0; the squared distance by its use against a squared radius).
const POINT_DISTANCE_004F7230: u32 = 0x004f_7230;
/// `004f71f0` (`ECX` = a point, one stack word: another point): the edge measure
/// [`fn_004f8560`] divides by (ST0).
const EDGE_MEASURE_004F71F0: u32 = 0x004f_71f0;
/// `005ae3d0` (`ECX` = the head node, one stack word: the address of the item word):
/// stores an item into the list.
const NODE_STORE_005AE3D0: u32 = 0x005a_e3d0;
/// The allocator (`00401000`, `cdecl`, one word).
const ALLOCATE: u32 = 0x0040_1000;
/// `004f7680` (`ECX`, one byte word: the owning flag): the polygon constructor (0x28 bytes).
const POLYGON_CONSTRUCT_004F7680: u32 = 0x004f_7680;
/// `004f7130` (`ECX` = a point, one stack word): a copy of the point (or, with a
/// destination, the point copied into it).
const POINT_COPY_004F7130: u32 = 0x004f_7130;
/// `TESRegionPoint::TESRegionPoint` (engine map, `004f6fc0`, `ECX`).
const REGION_POINT_CONSTRUCT: u32 = 0x004f_6fc0;
/// `004f7620` (`ECX` = the cached point, two stack words: a point and a `float` radius),
/// `AL`: whether the cache is for them.
const CACHE_MATCHES_004F7620: u32 = 0x004f_7620;
/// `004019d0` (`cdecl`, one `float` word): the square root (ST0).
const SQUARE_ROOT_004019D0: u32 = 0x0040_19d0;
/// `1.0`, `2.0` and `0.0` (`double`).
const DOUBLE_ONE: u32 = 0x0101_2070;
const DOUBLE_TWO: u32 = 0x0101_1590;
const DOUBLE_ZERO: u32 = 0x0101_2060;

// Translated from 004f7ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `point` to the polygon (unnamed in the engine map; `ECX` and two stack words,
/// `RET 8`; called by [`fn_004f80c0`]): false for a null point, or when the list has a
/// first item and `004f7230(first item, point)` is below `distance * distance` (x87;
/// NaN not below). Otherwise stores it (`005ae3d0` with the argument's slot); when the
/// four-point walk [`fn_004f8220`] (with 0) then finds a crossing, removes it again
/// (`0063f7b0`) and answers false. Else counts it at +0x24, widens the cached bounds at
/// +0x10/+0x14 (smallest) and +0x18/+0x1c (largest) with its coordinates (each stored
/// from a second call of the getter), and answers true.
pub fn fn_004f7ca0(e: &mut Engine, this: Ptr, point: u32, distance: u32) -> bool {
    if point == 0 {
        return false;
    }
    if item_of(e, this.addr()) != 0 {
        let first = item_of(e, this.addr());
        let measure = e.call(POINT_DISTANCE_004F7230, &args![first, point]).f64();
        let squared = x87_mul(x87_load32(distance), x87_m32(distance));
        // `FCOMPP`, `TEST AH,0x41`, `JNZ`: false only when the square is above.
        if squared > measure {
            return false;
        }
    }
    let stored = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), point);
        e.call(NODE_STORE_005AE3D0, &args![this, slot]);
        e.mem.u32(slot.addr())
    });
    if fn_004f8220(e, this, 0) {
        e.call(REMOVE_HEAD_0063F7B0, &args![this]);
        return false;
    }
    let base = this.addr();
    let points = e.mem.u32(base + 0x24).wrapping_add(1);
    e.mem.set_u32(base + 0x24, points);
    for (offset, getter, smallest) in [
        (0x10u32, POINT_FIRST_006A7F50, true),
        (0x14, POINT_SECOND_006B9130, true),
        (0x18, POINT_FIRST_006A7F50, false),
        (0x1c, POINT_SECOND_006B9130, false),
    ] {
        let value = e.call(getter, &args![stored]).f64();
        let cached = x87_load32(e.mem.u32(base + offset));
        // Smallest: `JNZ` on C0|C3 (kept unless the cache is above); largest: `JP` on
        // C0|C2 (kept unless the cache is below). NaN keeps the cache either way.
        let replace = if smallest {
            cached > value
        } else {
            cached < value
        };
        if replace {
            let value = e.call(getter, &args![stored]).f64();
            e.mem.set_u32(base + offset, x87_store32(value));
        }
    }
    true
}

// Translated from 004f80c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f80c0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f174bb, state variable around the constructor) and the frame's cookie word are not translated.
/// Copies the polygon into `target` (unnamed in the engine map; `ECX` and one stack word,
/// `RET 4`; called by `004f10d0`), or into a new one (0x28 bytes, `004f7680` with this
/// one's owning byte at +0xc; null when the allocation fails) when `target` is null. Each
/// node's item is added with [`fn_004f7ca0`] and distance 0.0: as it is when the target
/// does not own its items, else as a copy (`004f7130` with 0), freed (`00401030`) when the
/// add fails. Answers the target.
pub fn fn_004f80c0(e: &mut Engine, this: Ptr, target: u32) -> u32 {
    let target = if target != 0 {
        target
    } else {
        let block = e.call(ALLOCATE, &args![0x28u32]).u32();
        if block == 0 {
            0
        } else {
            let owns = u32::from(e.mem.u8(this.addr() + 0xc));
            e.call(POLYGON_CONSTRUCT_004F7680, &args![block, owns])
                .u32()
        }
    };
    let mut node = this.addr();
    while node != 0 {
        if e.mem.u8(target + 0xc) != 0 {
            let item = item_of(e, node);
            let copy = e.call(POINT_COPY_004F7130, &args![item, 0u32]).u32();
            if !fn_004f7ca0(e, Ptr::new(target), copy, 0.0f32.to_bits()) {
                e.call(DEALLOCATE, &args![copy]);
            }
        } else {
            let item = item_of(e, node);
            fn_004f7ca0(e, Ptr::new(target), item, 0.0f32.to_bits());
        }
        node = next_of(e, node);
    }
    target
}

// Translated from 004f8560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f8560 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef32cb, state variable around the cache point's constructor) and the frame's cookie word are not translated.
/// A scale for `point` and `radius` inside the polygon (unnamed in the engine map; `ECX`
/// and two stack words, `RET 8`; called by `GetScaleBasedOnLocation`), in ST0. A cached
/// answer at +8 that `004f7620` accepts for them is its `float` at +0xc. 0.0 for a null
/// point or one outside ([`fn_004f8360`]); 1.0 for a radius below 1.0. Otherwise the
/// smallest, over the edges (each node with its next, the last closing to the head), of
/// the squared distance from the point to the edge: with `a` and `b` the measures from the
/// point to the two ends (`004f7230`), `c` the edge's (`004f71f0`) and `d` the ends'
/// (`004f7230`), `t = (d - b + a) / (c * 2)`: `b` when `c <= t`, else `a` when `t <= 0`,
/// else `a - t * t` (every step x87 in the exe's order, rounded to `float` at each store).
/// The square root of that (`004019d0`) divided by the radius when the radius is above it
/// (or unordered), else 1.0, is the answer: stored with the radius in a new cache point
/// (0x10 bytes, [`fn_004f87e0`], the old one freed; the point copied in with `004f7130`).
pub fn fn_004f8560(e: &mut Engine, this: Ptr, point: u32, radius: u32) -> f64 {
    let base = this.addr();
    let cache = e.mem.u32(base + 8);
    if cache != 0 {
        let matches = e
            .call(
                CACHE_MATCHES_004F7620,
                &args![cache, point, x87_quiet_bits(radius)],
            )
            .u8();
        if matches != 0 {
            let cache = e.mem.u32(base + 8);
            return x87_load32(e.mem.u32(cache + 0xc));
        }
    }
    let inside = fn_004f8360(e, this, point);
    if point == 0 || !inside {
        return 0.0;
    }
    let one: f64 = e.global(DOUBLE_ONE);
    if x87_load32(radius) < one && inside {
        return 1.0;
    }
    let mut best = x87_store32(x87_load32(e.global::<u32>(FLOAT_MAX)));
    let two: f64 = e.global(DOUBLE_TWO);
    let zero: f64 = e.global(DOUBLE_ZERO);
    let mut node = base;
    let mut done = false;
    loop {
        let mut next = next_of(e, node);
        if next == 0 {
            next = base;
            done = true;
        }
        let start = item_of(e, node);
        let a = x87_store32(e.call(POINT_DISTANCE_004F7230, &args![point, start]).f64());
        let end = item_of(e, next);
        let b = x87_store32(e.call(POINT_DISTANCE_004F7230, &args![point, end]).f64());
        let end = item_of(e, next);
        let start = item_of(e, node);
        let c = x87_store32(e.call(EDGE_MEASURE_004F71F0, &args![start, end]).f64());
        let end = item_of(e, next);
        let start = item_of(e, node);
        let d = x87_store32(e.call(POINT_DISTANCE_004F7230, &args![start, end]).f64());
        let t = x87_store32(x87_add(x87_sub(x87_load32(d), x87_m32(b)), x87_m32(a)));
        let doubled = x87_mul(x87_load32(c), two);
        let t = x87_store32(x87_div(x87_m32(t), doubled));
        let (t_value, c_value) = (x87_load32(t), x87_load32(c));
        // `FCOMPP` (ST0 = c), `TEST AH,0x41`, `JP`: `b` unless c is above t or unordered.
        let distance = if c_value <= t_value {
            x87_quiet_bits(b)
        } else if x87_load32(t) <= zero {
            x87_quiet_bits(a)
        } else {
            x87_store32(x87_sub(x87_m32(a), x87_mul(x87_load32(t), x87_m32(t))))
        };
        if x87_load32(best) > x87_load32(distance) {
            best = x87_quiet_bits(distance);
        }
        node = next;
        if done {
            break;
        }
    }
    let root = e
        .call(SQUARE_ROOT_004019D0, &args![x87_quiet_bits(best)])
        .f64();
    let best = x87_store32(root);
    let scale = if x87_load32(radius) <= x87_load32(best) {
        1.0f32.to_bits()
    } else {
        x87_store32(x87_div(x87_load32(best), x87_m32(radius)))
    };
    let cache = e.mem.u32(base + 8);
    if cache != 0 {
        e.call(DEALLOCATE, &args![cache]);
    }
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    let cache = if block == 0 {
        0
    } else {
        fn_004f87e0(e, Ptr::new(block)).addr()
    };
    e.mem.set_u32(base + 8, cache);
    let cache = e.mem.u32(base + 8);
    e.call(POINT_COPY_004F7130, &args![point, cache]);
    let cache = e.mem.u32(base + 8);
    e.mem.set_u32(cache + 8, x87_quiet_bits(radius));
    let cache = e.mem.u32(base + 8);
    e.mem.set_u32(cache + 0xc, x87_quiet_bits(scale));
    x87_load32(scale)
}

// Translated from 004f87e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cache point's constructor (unnamed in the engine map; `ECX`, plain `RET`; run by
/// [`fn_004f8560`]): `TESRegionPoint::TESRegionPoint` on `this`, then answers `this`.
pub fn fn_004f87e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(REGION_POINT_CONSTRUCT, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_7ca0, fn_004f7ca0(Ptr, u32, u32) -> bool),
        entry!(0x004f_80c0, fn_004f80c0(Ptr, u32) -> u32),
        entry!(0x004f_8560, fn_004f8560(Ptr, u32, u32) -> f64),
        entry!(0x004f_87e0, fn_004f87e0(Ptr) -> Ptr),
        entry!(0x004f_7dd0, fn_004f7dd0(Ptr)),
        entry!(0x004f_7e80, fn_004f7e80(Ptr) -> f64),
        entry!(0x004f_7f10, fn_004f7f10(Ptr) -> f64),
        entry!(0x004f_7fa0, fn_004f7fa0(Ptr) -> f64),
        entry!(0x004f_8030, fn_004f8030(Ptr) -> f64),
        entry!(0x004f_81e0, fn_004f81e0(Ptr, u8) -> bool),
        entry!(0x004f_8220, fn_004f8220(Ptr, u8) -> bool),
        entry!(0x004f_8360, fn_004f8360(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine with the four constants and the list and point doubles.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_5000, 0x1_0000);
        e.set_global::<f32>(FLOAT_MAX, f32::MAX);
        e.set_global::<f32>(FLOAT_LOWEST, f32::MIN);
        e.set_global::<f64>(DOUBLE_FLOAT_MAX, f64::from(f32::MAX));
        e.set_global::<f64>(DOUBLE_FLOAT_LOWEST, f64::from(f32::MIN));
        e.register(COUNT_0059BB30, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        e.register(NODE_ITEM_006815C0, |_, a| a[0].into_ret());
        e.register(NODE_FOLLOWING_00726070, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(POINT_FIRST_006A7F50, |e, a| e.mem.f32(a[0]).into_ret());
        e.register(POINT_SECOND_006B9130, |e, a| e.mem.f32(a[0] + 4).into_ret());
        e
    }

    /// A polygon object over `points` (the head node is the object
    /// itself), bounds cleared, count set.
    fn polygon(e: &mut Engine, points: &[(f32, f32)]) -> u32 {
        let this = e.mem.alloc(0x28);
        let mut node = this;
        for (i, (x, y)) in points.iter().enumerate() {
            let point = e.mem.alloc(8);
            e.mem.set_f32(point, *x);
            e.mem.set_f32(point + 4, *y);
            e.mem.set_u32(node, point);
            if i + 1 < points.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        e.mem.set_f32(this + 0x10, f32::MAX);
        e.mem.set_f32(this + 0x14, f32::MAX);
        e.mem.set_f32(this + 0x18, f32::MIN);
        e.mem.set_f32(this + 0x1c, f32::MIN);
        e.mem.set_u32(this + 0x24, points.len() as u32);
        this
    }

    #[test]
    fn at_004f7dd0_empties_the_list_and_resets_the_bounds() {
        let mut e = engine();
        e.register(REMOVE_HEAD_0063F7B0, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let (item, after) = if next != 0 {
                (e.mem.u32(next), e.mem.u32(next + 4))
            } else {
                (0, 0)
            };
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(DEALLOCATE, |_, _| Ret::default());
        let this = polygon(&mut e, &[(1.0, 2.0), (3.0, 4.0)]);
        let first = e.mem.u32(this);
        e.mem.set_f32(this + 0x10, 5.0);
        e.mem.set_u8(this + 0xc, 1);
        e.call_log = Some(vec![]);
        e.call(0x004f_7dd0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.f32(this + 0x10), f32::MAX);
        assert_eq!(e.mem.f32(this + 0x14), f32::MAX);
        assert_eq!(e.mem.f32(this + 0x18), f32::MIN);
        assert_eq!(e.mem.f32(this + 0x1c), f32::MIN);
        let freed: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == DEALLOCATE)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(freed.len(), 2);
        assert_eq!(freed[0], first);

        // Not owning: nothing freed.
        let this = polygon(&mut e, &[(1.0, 2.0)]);
        e.call_log = Some(vec![]);
        e.call(0x004f_7dd0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != DEALLOCATE));
    }

    #[test]
    fn at_004f7e80_family_returns_cache_or_extremes() {
        let mut e = engine();
        let this = polygon(&mut e, &[(3.0, -2.0), (-1.5, 7.0), (2.0, 0.5)]);
        assert_eq!(e.call(0x004f_7e80, &args![this]).f64(), -1.5);
        assert_eq!(e.call(0x004f_7f10, &args![this]).f64(), 3.0);
        assert_eq!(e.call(0x004f_7fa0, &args![this]).f64(), -2.0);
        assert_eq!(e.call(0x004f_8030, &args![this]).f64(), 7.0);
        // Valid caches win.
        e.mem.set_f32(this + 0x10, 10.0);
        e.mem.set_f32(this + 0x18, -10.0);
        e.mem.set_f32(this + 0x14, 11.0);
        e.mem.set_f32(this + 0x1c, -11.0);
        assert_eq!(e.call(0x004f_7e80, &args![this]).f64(), 10.0);
        assert_eq!(e.call(0x004f_7f10, &args![this]).f64(), -10.0);
        assert_eq!(e.call(0x004f_7fa0, &args![this]).f64(), 11.0);
        assert_eq!(e.call(0x004f_8030, &args![this]).f64(), -11.0);
        // A NaN cache is not valid: the walk runs.
        e.mem.set_u32(this + 0x10, 0x7fc0_0000);
        assert_eq!(e.call(0x004f_7e80, &args![this]).f64(), -1.5);
        // An empty list gives the start values.
        let empty = polygon(&mut e, &[]);
        assert_eq!(
            e.call(0x004f_7e80, &args![empty]).f64(),
            f64::from(f32::MAX)
        );
        assert_eq!(
            e.call(0x004f_8030, &args![empty]).f64(),
            f64::from(f32::MIN)
        );
    }

    #[test]
    fn at_004f7e80_calls_the_getter_again_for_each_improvement() {
        let mut e = engine();
        let this = polygon(&mut e, &[(3.0, 0.0), (1.0, 0.0), (2.0, 0.0)]);
        e.call_log = Some(vec![]);
        e.call(0x004f_7e80, &args![this]);
        let log = e.call_log.take().unwrap();
        // Improvements at 3.0 and 1.0: 3 reads + 2 rereads.
        assert_eq!(
            log.iter().filter(|c| c.0 == POINT_FIRST_006A7F50).count(),
            5
        );
    }

    #[test]
    fn at_004f8220_needs_four_points_and_tests_each_window() {
        let mut e = engine();
        e.register(FOUR_POINT_TEST_004F7310, |_, _| Ret::default());
        let small = polygon(&mut e, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_8220, &args![small, 1u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != FOUR_POINT_TEST_004F7310));

        let this = polygon(&mut e, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        let items: Vec<u32> = {
            let mut v = vec![];
            let mut node = this;
            while node != 0 {
                v.push(e.mem.u32(node));
                node = e.mem.u32(node + 4);
            }
            v
        };
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_8220, &args![this, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        let windows: Vec<Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == FOUR_POINT_TEST_004F7310)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(windows, vec![vec![items[0], items[1], items[2], items[3]]]);

        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_8220, &args![this, 1u32]).bool());
        let log = e.call_log.take().unwrap();
        let windows: Vec<Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == FOUR_POINT_TEST_004F7310)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            windows,
            vec![
                vec![items[0], items[1], items[2], items[3]],
                vec![items[1], items[2], items[3], items[0]],
            ]
        );
    }

    #[test]
    fn at_004f8220_returns_true_on_the_first_hit() {
        let mut e = engine();
        e.register(FOUR_POINT_TEST_004F7310, |_, _| 1u32.into_ret());
        let this = polygon(&mut e, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_8220, &args![this, 1u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter()
                .filter(|c| c.0 == FOUR_POINT_TEST_004F7310)
                .count(),
            1
        );
    }

    #[test]
    fn at_004f81e0_checks_the_count_then_inverts_the_walk() {
        let mut e = engine();
        e.register(FOUR_POINT_TEST_004F7310, |_, _| Ret::default());
        let three = polygon(&mut e, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]);
        assert!(e.call(0x004f_81e0, &args![three, 1u32]).bool());
        let two = polygon(&mut e, &[(0.0, 0.0), (1.0, 0.0)]);
        assert!(!e.call(0x004f_81e0, &args![two, 1u32]).bool());
        assert!(e.call(0x004f_81e0, &args![two, 0u32]).bool());
        e.register(FOUR_POINT_TEST_004F7310, |_, _| 1u32.into_ret());
        let four = polygon(&mut e, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        assert!(!e.call(0x004f_81e0, &args![four, 0u32]).bool());
    }

    fn point(e: &mut Engine, x: f32, y: f32) -> u32 {
        let p = e.mem.alloc(8);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        p
    }

    #[test]
    fn at_004f8360_tests_points_against_a_square() {
        let mut e = engine();
        // The crossing of the edge a-b at second coordinate y.
        e.register(EDGE_CROSSING_004F72A0, |e, a| {
            let (ax, ay) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4));
            let (bx, by) = (e.mem.f32(a[1]), e.mem.f32(a[1] + 4));
            let y = f32::from_bits(a[2]);
            (ax + (y - ay) * (bx - ax) / (by - ay)).into_ret()
        });
        let this = polygon(&mut e, &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]);
        let inside = point(&mut e, 1.0, 2.0);
        assert!(e.call(0x004f_8360, &args![this, inside]).bool());
        let outside = point(&mut e, 5.0, 2.0);
        assert!(!e.call(0x004f_8360, &args![this, outside]).bool());
        let below = point(&mut e, 1.0, -1.0);
        assert!(!e.call(0x004f_8360, &args![this, below]).bool());
        assert!(!e.call(0x004f_8360, &args![this, 0u32]).bool());
        let two = polygon(&mut e, &[(0.0, 0.0), (4.0, 0.0)]);
        assert!(!e.call(0x004f_8360, &args![two, inside]).bool());
    }

    #[test]
    fn at_004f8360_asks_for_the_crossing_of_a_slanted_edge() {
        let mut e = engine();
        e.register(EDGE_CROSSING_004F72A0, |e, a| {
            let (ax, ay) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4));
            let (bx, by) = (e.mem.f32(a[1]), e.mem.f32(a[1] + 4));
            let y = f32::from_bits(a[2]);
            (ax + (y - ay) * (bx - ax) / (by - ay)).into_ret()
        });
        // A triangle; the point is right of the slanted edge's crossing.
        let this = polygon(&mut e, &[(0.0, 0.0), (4.0, 0.0), (0.0, 4.0)]);
        let inside = point(&mut e, 1.0, 1.0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_8360, &args![this, inside]).bool());
        let log = e.call_log.take().unwrap();
        let crossings: Vec<_> = log
            .iter()
            .filter(|c| c.0 == EDGE_CROSSING_004F72A0)
            .collect();
        assert!(!crossings.is_empty());
        assert_eq!(crossings[0].1[2], 1.0f32.to_bits());
        let outside = point(&mut e, 3.0, 3.0);
        assert!(!e.call(0x004f_8360, &args![this, outside]).bool());
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1_4000);
        e.set_global::<f32>(FLOAT_MAX, f32::MAX);
        e.set_global::<f32>(FLOAT_LOWEST, f32::MIN);
        e.set_global::<f64>(DOUBLE_FLOAT_MAX, f64::from(f32::MAX));
        e.set_global::<f64>(DOUBLE_FLOAT_LOWEST, f64::from(f32::MIN));
        e.set_global::<f64>(DOUBLE_ONE, 1.0);
        e.set_global::<f64>(DOUBLE_TWO, 2.0);
        e.set_global::<f64>(DOUBLE_ZERO, 0.0);
        e.register(COUNT_0059BB30, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        e.register(NODE_ITEM_006815C0, |_, a| a[0].into_ret());
        e.register(NODE_FOLLOWING_00726070, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(POINT_FIRST_006A7F50, |e, a| e.mem.f32(a[0]).into_ret());
        e.register(POINT_SECOND_006B9130, |e, a| e.mem.f32(a[0] + 4).into_ret());
        // Squared distance and length between two points.
        e.register(POINT_DISTANCE_004F7230, |e, a| {
            let (dx, dy) = (
                e.mem.f32(a[0]) - e.mem.f32(a[1]),
                e.mem.f32(a[0] + 4) - e.mem.f32(a[1] + 4),
            );
            (dx * dx + dy * dy).into_ret()
        });
        e.register(EDGE_MEASURE_004F71F0, |e, a| {
            let (dx, dy) = (
                e.mem.f32(a[0]) - e.mem.f32(a[1]),
                e.mem.f32(a[0] + 4) - e.mem.f32(a[1] + 4),
            );
            (dx * dx + dy * dy).sqrt().into_ret()
        });
        e.register(EDGE_CROSSING_004F72A0, |e, a| {
            let (ax, ay) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4));
            let (bx, by) = (e.mem.f32(a[1]), e.mem.f32(a[1] + 4));
            let y = f32::from_bits(a[2]);
            (ax + (y - ay) * (bx - ax) / (by - ay)).into_ret()
        });
        // Storing into an empty head fills it; later stores are only logged.
        e.register(NODE_STORE_005AE3D0, |e, a| {
            if e.mem.u32(a[0]) == 0 {
                let item = e.mem.u32(a[1]);
                e.mem.set_u32(a[0], item);
            }
            Ret::default()
        });
        e
    }

    fn polygon(e: &mut Engine, points: &[(f32, f32)]) -> u32 {
        let this = e.mem.alloc(0x28);
        let mut node = this;
        for (i, (x, y)) in points.iter().enumerate() {
            let point = point(e, *x, *y);
            e.mem.set_u32(node, point);
            if i + 1 < points.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        e.mem.set_f32(this + 0x10, f32::MAX);
        e.mem.set_f32(this + 0x14, f32::MAX);
        e.mem.set_f32(this + 0x18, f32::MIN);
        e.mem.set_f32(this + 0x1c, f32::MIN);
        e.mem.set_u32(this + 0x24, points.len() as u32);
        this
    }

    fn point(e: &mut Engine, x: f32, y: f32) -> u32 {
        let p = e.mem.alloc(8);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        p
    }

    #[test]
    fn fn_004f7ca0_adds_a_point_and_widens_the_bounds() {
        let mut e = engine();
        let this = polygon(&mut e, &[]);
        let p = point(&mut e, 3.0, 4.0);
        assert!(e
            .call(0x004f_7ca0, &args![this, p, 0.0f32.to_bits()])
            .bool());
        assert_eq!(e.mem.u32(this), p);
        assert_eq!(e.mem.u32(this + 0x24), 1);
        assert_eq!(e.mem.f32(this + 0x10), 3.0);
        assert_eq!(e.mem.f32(this + 0x14), 4.0);
        assert_eq!(e.mem.f32(this + 0x18), 3.0);
        assert_eq!(e.mem.f32(this + 0x1c), 4.0);
        // Too close to the first point: refused.
        let near = point(&mut e, 3.0, 5.0);
        assert!(!e
            .call(0x004f_7ca0, &args![this, near, 2.0f32.to_bits()])
            .bool());
        assert!(!e.call(0x004f_7ca0, &args![this, 0u32, 0u32]).bool());
        assert_eq!(e.mem.u32(this + 0x24), 1);
    }

    #[test]
    fn fn_004f7ca0_takes_a_crossing_point_out_again() {
        let mut e = engine();
        e.register(FOUR_POINT_TEST_004F7310, |_, _| 1u32.into_ret());
        e.register(REMOVE_HEAD_0063F7B0, |_, _| Ret::default());
        let this = polygon(&mut e, &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]);
        let p = point(&mut e, 9.0, 9.0);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x004f_7ca0, &args![this, p, 0.0f32.to_bits()])
            .bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, REMOVE_HEAD_0063F7B0), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 0x24), 4);
    }

    #[test]
    fn fn_004f80c0_adds_every_point_to_the_target() {
        let mut e = engine();
        let source = polygon(&mut e, &[(1.0, 1.0), (2.0, 2.0)]);
        let target = polygon(&mut e, &[]);
        assert_eq!(e.call(0x004f_80c0, &args![source, target]).u32(), target);
        assert_eq!(e.mem.u32(target + 0x24), 2);
        assert_eq!(e.mem.f32(target + 0x18), 2.0);
        // An owning target adds copies and frees a copy it refuses.
        let owning = polygon(&mut e, &[]);
        e.mem.set_u8(owning + 0xc, 1);
        e.register(POINT_COPY_004F7130, |_, _| 0u32.into_ret());
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_80c0, &args![source, owning]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, POINT_COPY_004F7130).len(), 2);
        assert_eq!(calls(&log, DEALLOCATE), vec![vec![0], vec![0]]);
        // Without a target, a new polygon is made with the source's owning byte.
        let fresh = polygon(&mut e, &[]);
        e.register(ALLOCATE, |_, _| 0x5000u32.into_ret());
        e.register_double(POLYGON_CONSTRUCT_004F7680, move |_, _| fresh.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_80c0, &args![source, 0u32]).u32(), fresh);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, POLYGON_CONSTRUCT_004F7680),
            vec![vec![0x5000, 0]]
        );
        assert_eq!(e.mem.u32(fresh + 0x24), 2);
    }

    #[test]
    fn fn_004f8560_scales_by_the_distance_to_the_nearest_edge() {
        let mut e = engine();
        e.register(SQUARE_ROOT_004019D0, |_, a| {
            f32::from_bits(a[0]).sqrt().into_ret()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(REGION_POINT_CONSTRUCT, |_, _| Ret::default());
        e.register(POINT_COPY_004F7130, |_, _| Ret::default());
        let this = polygon(&mut e, &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]);
        let p = point(&mut e, 1.0, 2.0);
        // The nearest edge is 1.0 away: 1.0 / 2.0.
        let scale = e.call(0x004f_8560, &args![this, p, 2.0f32.to_bits()]).f64();
        assert_eq!(scale, 0.5);
        let cache = e.mem.u32(this + 8);
        assert_eq!(e.mem.f32(cache + 8), 2.0);
        assert_eq!(e.mem.f32(cache + 0xc), 0.5);
        // A radius within the distance scales by 1.0.
        e.mem.set_u32(this + 8, 0);
        let scale = e.call(0x004f_8560, &args![this, p, 1.0f32.to_bits()]).f64();
        assert_eq!(scale, 1.0);
        // A matching cache answers its value.
        e.register(CACHE_MATCHES_004F7620, |_, _| 1u32.into_ret());
        let cache = e.mem.u32(this + 8);
        e.mem.set_f32(cache + 0xc, 0.25);
        assert_eq!(
            e.call(0x004f_8560, &args![this, p, 2.0f32.to_bits()]).f64(),
            0.25
        );
    }

    #[test]
    fn fn_004f8560_answers_zero_outside_and_one_for_small_radii() {
        let mut e = engine();
        let this = polygon(&mut e, &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]);
        let outside = point(&mut e, 5.0, 2.0);
        assert_eq!(
            e.call(0x004f_8560, &args![this, outside, 2.0f32.to_bits()])
                .f64(),
            0.0
        );
        assert_eq!(
            e.call(0x004f_8560, &args![this, 0u32, 2.0f32.to_bits()])
                .f64(),
            0.0
        );
        let inside = point(&mut e, 1.0, 2.0);
        assert_eq!(
            e.call(0x004f_8560, &args![this, inside, 0.5f32.to_bits()])
                .f64(),
            1.0
        );
    }

    #[test]
    fn fn_004f87e0_constructs_the_region_point() {
        let mut e = engine();
        e.register(REGION_POINT_CONSTRUCT, |_, _| 0x55u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_87e0, &args![0x9000u32]).u32(), 0x9000);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, REGION_POINT_CONSTRUCT), vec![vec![0x9000]]);
    }
}
