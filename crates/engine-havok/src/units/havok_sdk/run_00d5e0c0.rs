//! `Havok SDK/run_00d5e0c0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions are virtual methods of `hkpConvexListShape` (the RTTI name
//! of the vtable at `010d10c4`; the class is not in the Xbox PDB, so the
//! field meanings below come from this code only): a convex shape holding a
//! list of child shapes (`children` +0x44, `child_count` +0x48) that it
//! forwards to. The children are re-read from the list at every step, as the
//! exe does; calls to them are virtual (the slot is the byte offset in the
//! vtable). SSE code computes lane by lane in `f32`.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// The convex list shape, as far as these functions use it.
    pub struct ConvexListShape: 0x50 {
        /// The vtable.
        0x00 vtable: u32,
        /// The radius added to the tolerance when the stored bounds are used
        /// (the convex shape radius field).
        0x10 radius: f32,
        /// When non-zero `getAabb` bounds the stored vectors at +0x20 and +0x30
        /// instead of asking the children.
        0x40 use_stored_bounds: u8,
        /// Pointer to the array of child shape pointers.
        0x44 children: u32,
        /// Number of children.
        0x48 child_count: i32,
    }
}

/// Offsets of the two stored vectors `getAabb` hands to the bounds helper.
const STORED_BOUNDS_A: u32 = 0x20;
const STORED_BOUNDS_B: u32 = 0x30;
/// The bounds helper `getAabb` calls when the stored bounds are used
/// (`00d26230`, cdecl, five words).
const STORED_BOUNDS_AABB: u32 = 0x00d2_6230;
/// `float` constant: the most negative value used (`0xff7fffee`), where the
/// supporting vertex search starts.
const SUPPORT_START: u32 = 0x0104_e148;

// Vtable slots of the child shapes (byte offsets).
const SLOT_GET_AABB: u32 = 0x1c;
const SLOT_VERTEX_COUNT: u32 = 0x2c;
const SLOT_COLLISION_SPHERES: u32 = 0x30;
const SLOT_FIRST_VERTEX: u32 = 0x34;
const SLOT_SUPPORTING_VERTEX: u32 = 0x38;
const SLOT_CONVERT_VERTEX_IDS: u32 = 0x3c;

type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// A 16-byte copy (`MOVAPS` load and store), bit for bit.
fn copy_vec(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

/// The child at `index` of the shape's list.
fn child(e: &Engine, this: Ptr<ConvexListShape>, index: u32) -> u32 {
    let list = e.get(this, ConvexListShape::children);
    e.mem.u32(list.wrapping_add(index.wrapping_mul(4)))
}

/// Tags the `w` lane of a vertex with the child it came from: the bits of `w`
/// that survive `0xc0ffffff`, plus the child index shifted left by 8, ORed with
/// `0x3f000000`.
fn tag_vertex(e: &mut Engine, vertex: u32, child_index: u32) {
    let w = e.mem.u32(vertex + 0xc);
    e.mem.set_u32(
        vertex + 0xc,
        ((w & 0xc0ff_ffff).wrapping_add(child_index << 8)) | 0x3f00_0000,
    );
}

// Translated from 00d5e0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot +0x10 (`getContainer`-like, fastcall): the address of the
/// field at +0x14 of the shape, or 0 for a null shape.
pub fn fn_00d5e0f0(_e: &mut Engine, this: Ptr<ConvexListShape>) -> u32 {
    if this.addr() == 0 {
        0
    } else {
        this.addr() + 0x14
    }
}

// Translated from 00d5e100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot +0x3c: converts `count` vertex ids (16-bit) to vertices in
/// `out`. The high byte of an id picks the child, the low byte is the id in
/// that child: the child converts it (its slot +0x3c, with a pointer to the
/// low byte as a 4-byte word, a count of 1 and the output vertex), then the
/// `w` lane of the vertex is tagged with the child index (see `tag_vertex`).
/// A count of 0 or less converts nothing.
pub fn fn_00d5e100(e: &mut Engine, this: Ptr<ConvexListShape>, ids: Ptr, count: i32, out: Ptr) {
    let mut id_address = ids.addr();
    let mut out_address = out.addr();
    let mut remaining = count;
    while remaining > 0 {
        let id = u32::from(e.mem.u16(id_address));
        let child_index = id >> 8;
        let low = id & 0xff;
        let target = child(e, this, child_index);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), low);
            e.vcall(
                target,
                SLOT_CONVERT_VERTEX_IDS,
                &args![slot, 1u32, out_address],
            );
        });
        tag_vertex(e, out_address, child_index);
        id_address = id_address.wrapping_add(2);
        out_address = out_address.wrapping_add(0x10);
        remaining -= 1;
    }
}

// Translated from 00d5e180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot +0x34: the first vertex of the first child, by a tail jump to
/// its slot +0x34 with `out`.
pub fn fn_00d5e180(e: &mut Engine, this: Ptr<ConvexListShape>, out: Ptr) {
    let first = child(e, this, 0);
    e.vcall(first, SLOT_FIRST_VERTEX, &args![out]);
}

// Translated from 00d5e190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot +0x2c: the sum over the children of their slot +0x2c result
/// (a count of vertices or spheres).
pub fn fn_00d5e190(e: &mut Engine, this: Ptr<ConvexListShape>) -> i32 {
    let mut total = 0i32;
    let mut index = 0i32;
    while index < e.get(this, ConvexListShape::child_count) {
        let target = child(e, this, index as u32);
        let n = e.vcall(target, SLOT_VERTEX_COUNT, &[]).u32() as i32;
        index += 1;
        total = total.wrapping_add(n);
    }
    total
}

// Translated from 00d5e1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot +0x30: collects the children's collision spheres into `out`,
/// one child after the other: each child writes to its slot +0x30 (with the
/// next free position), then its slot +0x2c gives the number written; the
/// position moves on by that many 16-byte entries. Returns `out`.
pub fn fn_00d5e1c0(e: &mut Engine, this: Ptr<ConvexListShape>, out: Ptr) -> Ptr {
    let mut destination = out.addr();
    let mut index = 0i32;
    while index < e.get(this, ConvexListShape::child_count) {
        let target = child(e, this, index as u32);
        e.vcall(target, SLOT_COLLISION_SPHERES, &args![destination]);
        let n = e.vcall(target, SLOT_VERTEX_COUNT, &[]).u32();
        index += 1;
        destination = destination.wrapping_add(n << 4);
    }
    out
}

// Translated from 00d5e300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot +0x38: the supporting vertex of the list. Every child gives
/// its supporting vertex for `direction` (its slot +0x38, into a scratch
/// vector); the one with the largest dot product `z + (y + x)` (strictly above
/// the best so far, which starts at the float at `0104e148`; NaN never wins)
/// is copied to `out`. The `w` lane of `out` is tagged with the winning child
/// index (also when there is no child or none wins: index 0).
pub fn fn_00d5e300(e: &mut Engine, this: Ptr<ConvexListShape>, direction: Ptr, out: Ptr) {
    let mut best = x87_quiet(e.global::<f32>(SUPPORT_START));
    let mut best_index = 0u32;
    let mut index = 0i32;
    while index < e.get(this, ConvexListShape::child_count) {
        let target = child(e, this, index as u32);
        e.with_stack(0x10, |e, scratch| {
            e.vcall(target, SLOT_SUPPORTING_VERTEX, &args![direction, scratch]);
            let vertex = load_vec(e, scratch.addr());
            let direction_lanes = load_vec(e, direction.addr());
            let product: Vec4 = std::array::from_fn(|lane| direction_lanes[lane] * vertex[lane]);
            let dot = product[2] + (product[1] + product[0]);
            if best < dot {
                best = x87_quiet(dot);
                copy_vec(e, scratch.addr(), out.addr());
                best_index = index as u32;
            }
        });
        index += 1;
    }
    tag_vertex(e, out.addr(), best_index);
}

// Translated from 00d5e3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot +0x1c: the bounding box of the list under `transform`, grown
/// by `tolerance`, into `out` (`m_min` at +0, `m_max` at +0x10).
///
/// With the stored-bounds byte set, the bounds helper `00d26230` gets the
/// transform, the vectors at +0x20 and +0x30, `radius + tolerance` (rounded
/// to a float) and `out`. Otherwise the first child writes its box straight
/// to `out` (its slot +0x1c) and every further child writes into a scratch
/// box that is merged in with `MINPS` / `MAXPS` (the new value wins ties and
/// NaN is taken from the second operand).
pub fn fn_00d5e3c0(
    e: &mut Engine,
    this: Ptr<ConvexListShape>,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    if e.get(this, ConvexListShape::use_stored_bounds) != 0 {
        let radius = e.get(this, ConvexListShape::radius);
        let grown = x87_quiet((radius as f64 + tolerance as f64) as f32);
        e.call(
            STORED_BOUNDS_AABB,
            &args![
                transform,
                this.addr() + STORED_BOUNDS_A,
                this.addr() + STORED_BOUNDS_B,
                grown,
                out
            ],
        );
        return;
    }
    let first = child(e, this, 0);
    let quiet_tolerance = x87_quiet(tolerance);
    e.vcall(
        first,
        SLOT_GET_AABB,
        &args![transform, quiet_tolerance, out],
    );
    let mut index = 1i32;
    while e.get(this, ConvexListShape::child_count) > index {
        let target = child(e, this, index as u32);
        e.with_stack(0x20, |e, scratch| {
            e.vcall(
                target,
                SLOT_GET_AABB,
                &args![transform, quiet_tolerance, scratch],
            );
            let low = load_vec(e, out.addr());
            let scratch_low = load_vec(e, scratch.addr());
            let merged_low: Vec4 = std::array::from_fn(|lane| {
                if low[lane] < scratch_low[lane] {
                    low[lane]
                } else {
                    scratch_low[lane]
                }
            });
            store_vec(e, out.addr(), merged_low);
            let high = load_vec(e, out.addr() + 0x10);
            let scratch_high = load_vec(e, scratch.addr() + 0x10);
            let merged_high: Vec4 = std::array::from_fn(|lane| {
                if high[lane] > scratch_high[lane] {
                    high[lane]
                } else {
                    scratch_high[lane]
                }
            });
            store_vec(e, out.addr() + 0x10, merged_high);
        });
        index += 1;
    }
}

/// The memory-free `getAabb` argument, an identity transform (`01268370`), and the floats the
/// bounds start from: `+3.4e38` (`0104da48`) and `-3.4e38` (`0104e148`).
const IDENTITY_TRANSFORM: u32 = 0x0126_8370;
const LARGEST_FLOAT: u32 = 0x0104_da48;
const SMALLEST_FLOAT: u32 = 0x0104_e148;
/// The vector of four halves (`010c71d0`).
const HALVES: u32 = 0x010c_71d0;

// Translated from 00d5e0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, no stack words): answers the byte at `+0x40`.
pub fn fn_00d5e0c0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x40)
}

// Translated from 00d5e0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, no stack words; a vtable slot): answers 0.
pub fn fn_00d5e0d0(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

// Translated from 00d5e0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 4`, the stack word is never read; a vtable slot): answers 0.
pub fn fn_00d5e0e0(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u32 {
    0
}

/// `MINPS dst, src` on one lane: `dst` when it is below `src`, else `src`.
fn minps_00d5e210(dst: f32, src: f32) -> f32 {
    if dst < src {
        dst
    } else {
        src
    }
}

/// `MAXPS dst, src` on one lane: `dst` when it is above `src`, else `src`.
fn maxps_00d5e210(dst: f32, src: f32) -> f32 {
    if dst > src {
        dst
    } else {
        src
    }
}

// Translated from 00d5e210 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 4`: one stack word of which the low byte `flag` is used): stores
/// `flag` at `+0x40`; when it is not 0, the bounds of the children are collected. The pointers of
/// the array at `+0x44` (count at `+0x48`) are asked, one by one, for their box through slot
/// `0x1c` (arguments: the identity transform at `01268370`, the float `0.0`, the address of a
/// 0x20-byte area in the frame whose first vector is the minimum and `+0x10` the maximum). The
/// lane-wise minimum (starting from `+3.4e38`, `MINPS` with the accumulator first) and maximum
/// (from `-3.4e38`, `MAXPS`) of all of them end as `+0x30` = `(max + min) * 0.5` and `+0x20` =
/// `(max - min) * 0.5` (the vector `010c71d0`, four halves). With no children the extreme
/// constants give `0` and `3.4e38` in the two vectors.
pub fn fn_00d5e210(e: &mut Engine, this: Ptr, flag: u8) {
    let me = this.addr();
    e.mem.set_u8(me + 0x40, flag);
    if flag == 0 {
        return;
    }
    let mut minimum = [e.mem.f32(LARGEST_FLOAT); 4];
    let mut maximum = [e.mem.f32(SMALLEST_FLOAT); 4];
    if e.mem.i32(me + 0x48) > 0 {
        let mut index = 0i32;
        loop {
            let child = e
                .mem
                .u32(e.mem.u32(me + 0x44).wrapping_add(index as u32 * 4));
            e.with_stack(0x20, |e, area| {
                e.vcall(child, 0x1c, &args![IDENTITY_TRANSFORM, 0.0f32, area]);
                let low = [0u32, 1, 2, 3].map(|lane| e.mem.f32(area.addr() + lane * 4));
                let high = [0u32, 1, 2, 3].map(|lane| e.mem.f32(area.addr() + 0x10 + lane * 4));
                for lane in 0..4 {
                    minimum[lane] = minps_00d5e210(minimum[lane], low[lane]);
                    maximum[lane] = maxps_00d5e210(maximum[lane], high[lane]);
                }
            });
            index += 1;
            if index >= e.mem.i32(me + 0x48) {
                break;
            }
        }
    }
    let halves = [0u32, 1, 2, 3].map(|lane| e.mem.f32(HALVES + lane * 4));
    for lane in 0..4u32 {
        let index = lane as usize;
        let center = (maximum[index] + minimum[index]) * halves[index];
        e.mem.set_f32(me + 0x30 + lane * 4, center);
    }
    for lane in 0..4u32 {
        let index = lane as usize;
        let extent = (maximum[index] - minimum[index]) * halves[index];
        e.mem.set_f32(me + 0x20 + lane * 4, extent);
    }
}

// Translated from 00d5e2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, no stack words; a vtable slot): answers the word at `+0x34`.
pub fn fn_00d5e2d0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x34)
}

// Translated from 00d5e2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 4`: one stack word `key`; a vtable slot): the next key,
/// `key + 1`, or `0xffffffff` when that is not below (signed) the word at `+0x34`.
pub fn fn_00d5e2e0(e: &mut Engine, this: Ptr, key: i32) -> i32 {
    let next = key.wrapping_add(1);
    if next < e.mem.i32(this.addr() + 0x34) {
        next
    } else {
        -1
    }
}

// Translated from 00d5e2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 8`: two stack words `index` and one never read; a vtable slot):
/// answers the word `index` of the array whose data pointer is at `+0x30`.
pub fn fn_00d5e2f0(e: &mut Engine, this: Ptr, index: u32, _unused_2: u32) -> u32 {
    let data = e.mem.u32(this.addr() + 0x30);
    e.mem.u32(data.wrapping_add(index.wrapping_mul(4)))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d5e0f0, fn_00d5e0f0(Ptr<ConvexListShape>) -> u32),
        entry!(0x00d5e100, fn_00d5e100(Ptr<ConvexListShape>, Ptr, i32, Ptr)),
        entry!(0x00d5e180, fn_00d5e180(Ptr<ConvexListShape>, Ptr)),
        entry!(0x00d5e190, fn_00d5e190(Ptr<ConvexListShape>) -> i32),
        entry!(0x00d5e1c0, fn_00d5e1c0(Ptr<ConvexListShape>, Ptr) -> Ptr),
        entry!(0x00d5e300, fn_00d5e300(Ptr<ConvexListShape>, Ptr, Ptr)),
        entry!(0x00d5e3c0, fn_00d5e3c0(Ptr<ConvexListShape>, Ptr, f32, Ptr)),
        entry!(0x00d5_e0c0, fn_00d5e0c0(Ptr) -> u8),
        entry!(0x00d5_e0d0, fn_00d5e0d0(Ptr) -> u32),
        entry!(0x00d5_e0e0, fn_00d5e0e0(Ptr, u32) -> u32),
        entry!(0x00d5_e210, fn_00d5e210(Ptr, u8)),
        entry!(0x00d5_e2d0, fn_00d5e2d0(Ptr) -> u32),
        entry!(0x00d5_e2e0, fn_00d5e2e0(Ptr, i32) -> i32),
        entry!(0x00d5_e2f0, fn_00d5e2f0(Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(SUPPORT_START, f32::from_bits(0xff7f_ffee));
        e
    }

    /// A child shape whose vtable slots point at the given function addresses
    /// (slot offset, address); returns the child.
    fn child_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A list shape over the given children.
    fn list_of(e: &mut Engine, children: &[u32]) -> Ptr<ConvexListShape> {
        let this = e.new_object::<ConvexListShape>();
        let array = e.mem.alloc(4 * children.len().max(1) as u32);
        for (i, c) in children.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *c);
        }
        e.set(this, ConvexListShape::children, array);
        e.set(this, ConvexListShape::child_count, children.len() as i32);
        this
    }

    #[test]
    fn slot_10_gives_the_field_address_or_null() {
        let mut e = engine();
        assert_eq!(e.call(0x00d5_e0f0, &args![0u32]).u32(), 0);
        assert_eq!(e.call(0x00d5_e0f0, &args![0x1000u32]).u32(), 0x1014);
    }

    /// What the conversion doubles saw: child, low byte, output address.
    type Seen = std::rc::Rc<std::cell::RefCell<Vec<(u32, u32, u32)>>>;

    #[test]
    fn convert_vertex_ids_asks_the_child_named_by_the_high_byte() {
        let mut e = engine();
        let conv0 = 0x00bb_0000u32;
        let conv1 = 0x00bb_0001u32;
        let child0 = child_with(&mut e, &[(SLOT_CONVERT_VERTEX_IDS, conv0)]);
        let child1 = child_with(&mut e, &[(SLOT_CONVERT_VERTEX_IDS, conv1)]);
        let this = list_of(&mut e, &[child0, child1]);
        let seen: Seen = Default::default();
        for (address, owner) in [(conv0, child0), (conv1, child1)] {
            let log = seen.clone();
            e.register_double(address, move |e, a| {
                assert_eq!(a[0], owner);
                assert_eq!(a[2], 1);
                let low = e.mem.u32(a[1]);
                log.borrow_mut().push((owner, low, a[3]));
                // The child leaves its own bits in w; x carries the low byte.
                e.mem.set_f32(a[3], low as f32);
                e.mem.set_u32(a[3] + 0xc, 0xffff_ffff);
                Ret::default()
            });
        }
        let ids = e.mem.alloc(8);
        e.mem.set_u16(ids, 0x0107);
        e.mem.set_u16(ids + 2, 0x0003);
        let out = e.mem.alloc(0x20);
        e.call(0x00d5_e100, &args![this, ids, 2i32, out]);
        assert_eq!(
            *seen.borrow(),
            vec![(child1, 7, out), (child0, 3, out + 0x10)]
        );
        assert_eq!(e.mem.f32(out), 7.0);
        // Child 1: the child's bits kept by 0xc0ffffff, plus 1 << 8, ORed with 0x3f000000.
        let first = 0xc0ff_ffffu32.wrapping_add(0x100) | 0x3f00_0000;
        assert_eq!(e.mem.u32(out + 0xc), first);
        let second = 0xc0ff_ffffu32 | 0x3f00_0000;
        assert_eq!(e.mem.u32(out + 0x1c), second);
        // A count of 0 or less converts nothing.
        seen.borrow_mut().clear();
        e.call(0x00d5_e100, &args![this, ids, 0i32, out]);
        e.call(0x00d5_e100, &args![this, ids, -2i32, out]);
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn first_vertex_is_the_first_childs() {
        let mut e = engine();
        let handler = 0x00bb_0010u32;
        let child0 = child_with(&mut e, &[(SLOT_FIRST_VERTEX, handler)]);
        let this = list_of(&mut e, &[child0]);
        e.register_double(handler, move |_, a| {
            assert_eq!(a, [child0, 0x4242]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00d5_e180, &args![this, 0x4242u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|(a, _)| *a == handler).count(), 1);
    }

    /// Registers a child answering slot +0x2c with `n` and recording slot +0x30.
    fn counting_child(e: &mut Engine, n: u32, tag: u32) -> u32 {
        let count_fn = 0x00bc_0000 + tag * 2;
        let spheres_fn = count_fn + 1;
        e.register_double(count_fn, move |_, _| Ret {
            eax: n,
            ..Ret::default()
        });
        e.register_double(spheres_fn, move |e, a| {
            // Marks where this child was told to write.
            e.mem.set_u32(a[1], tag);
            Ret::default()
        });
        child_with(
            e,
            &[
                (SLOT_VERTEX_COUNT, count_fn),
                (SLOT_COLLISION_SPHERES, spheres_fn),
            ],
        )
    }

    #[test]
    fn slot_2c_sums_the_children() {
        let mut e = engine();
        let a = counting_child(&mut e, 3, 1);
        let b = counting_child(&mut e, 4, 2);
        let this = list_of(&mut e, &[a, b]);
        assert_eq!(e.call(0x00d5_e190, &args![this]).u32(), 7);
        let none = list_of(&mut e, &[]);
        assert_eq!(e.call(0x00d5_e190, &args![none]).u32(), 0);
    }

    #[test]
    fn collision_spheres_follow_each_other() {
        let mut e = engine();
        let a = counting_child(&mut e, 2, 1);
        let b = counting_child(&mut e, 1, 2);
        let this = list_of(&mut e, &[a, b]);
        let out = e.mem.alloc(0x40);
        let returned = e.call(0x00d5_e1c0, &args![this, out]).u32();
        assert_eq!(returned, out);
        // The first child wrote at the start, the second after its two entries.
        assert_eq!(e.mem.u32(out), 1);
        assert_eq!(e.mem.u32(out + 0x20), 2);
        let none = list_of(&mut e, &[]);
        assert_eq!(e.call(0x00d5_e1c0, &args![none, out]).u32(), out);
    }

    /// A child whose supporting vertex is always `vertex`.
    fn vertex_child(e: &mut Engine, vertex: Vec4, tag: u32) -> u32 {
        let handler = 0x00bd_0000 + tag;
        e.register_double(handler, move |e, a| {
            store_vec(e, a[2], vertex);
            Ret::default()
        });
        child_with(e, &[(SLOT_SUPPORTING_VERTEX, handler)])
    }

    #[test]
    fn supporting_vertex_takes_the_best_child_and_tags_its_index() {
        let mut e = engine();
        let a = vertex_child(&mut e, [1.0, 0.0, 0.0, 0.0], 1);
        let b = vertex_child(&mut e, [0.0, 3.0, 1.0, 0.0], 2);
        let c = vertex_child(&mut e, [0.0, 3.0, 1.0, 0.0], 3);
        let this = list_of(&mut e, &[a, b, c]);
        let direction = e.mem.alloc(16);
        store_vec(&mut e, direction, [1.0, 1.0, 1.0, 0.0]);
        let out = e.mem.alloc(16);
        e.mem.set_u32(out + 0xc, 0xffff_ffff);
        e.call(0x00d5_e300, &args![this, direction, out]);
        // Child 1 (dot 4) beats child 0 (dot 1); child 2 ties and loses.
        assert_eq!(load_vec(&e, out)[..3], [0.0, 3.0, 1.0]);
        // The copied vertex has w = 0.0 (bits 0), which is tagged with index 1.
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0100);
    }

    #[test]
    fn supporting_vertex_without_children_only_tags_index_zero() {
        let mut e = engine();
        let this = list_of(&mut e, &[]);
        let direction = e.mem.alloc(16);
        let out = e.mem.alloc(16);
        store_vec(&mut e, out, [5.0, 6.0, 7.0, 0.0]);
        e.mem.set_u32(out + 0xc, 0x1234_5678);
        e.call(0x00d5_e300, &args![this, direction, out]);
        assert_eq!(load_vec(&e, out)[..3], [5.0, 6.0, 7.0]);
        assert_eq!(
            e.mem.u32(out + 0xc),
            (0x1234_5678 & 0xc0ff_ffff) | 0x3f00_0000
        );
    }

    #[test]
    fn aabb_with_stored_bounds_calls_the_helper() {
        let mut e = engine();
        let this = list_of(&mut e, &[]);
        e.set(this, ConvexListShape::use_stored_bounds, 1);
        e.set(this, ConvexListShape::radius, 0.5);
        e.register(STORED_BOUNDS_AABB, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d5_e3c0, &args![this, 0x1111u32, 0.25f32, 0x2222u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == STORED_BOUNDS_AABB)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(
            calls[0].1,
            vec![
                0x1111,
                this.addr() + 0x20,
                this.addr() + 0x30,
                0.75f32.to_bits(),
                0x2222
            ]
        );
    }

    /// A child whose box is `low` .. `high`.
    fn box_child(e: &mut Engine, low: Vec4, high: Vec4, tag: u32) -> u32 {
        let handler = 0x00be_0000 + tag;
        e.register_double(handler, move |e, a| {
            assert_eq!(a[2], 0.5f32.to_bits());
            store_vec(e, a[3], low);
            store_vec(e, a[3] + 0x10, high);
            Ret::default()
        });
        child_with(e, &[(SLOT_GET_AABB, handler)])
    }

    #[test]
    fn aabb_merges_the_childrens_boxes() {
        let mut e = engine();
        let a = box_child(&mut e, [0.0, -1.0, 2.0, 0.0], [1.0, 1.0, 3.0, 0.0], 1);
        let b = box_child(&mut e, [-2.0, 0.0, 2.5, 0.0], [0.5, 4.0, 2.5, 0.0], 2);
        let this = list_of(&mut e, &[a, b]);
        let out = e.mem.alloc(0x20);
        e.call(0x00d5_e3c0, &args![this, 0x1000u32, 0.5f32, out]);
        assert_eq!(load_vec(&e, out), [-2.0, -1.0, 2.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x10), [1.0, 4.0, 3.0, 0.0]);
    }

    #[test]
    fn aabb_of_a_single_child_is_its_box() {
        let mut e = engine();
        let a = box_child(&mut e, [1.0, 2.0, 3.0, 0.0], [4.0, 5.0, 6.0, 0.0], 3);
        let this = list_of(&mut e, &[a]);
        let out = e.mem.alloc(0x20);
        e.call(0x00d5_e3c0, &args![this, 0x1000u32, 0.5f32, out]);
        assert_eq!(load_vec(&e, out), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x10), [4.0, 5.0, 6.0, 0.0]);
    }
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn the_small_functions_read_their_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x40, 0x1234_5678);
        e.mem.set_u32(this + 0x34, 3);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data + 8, 0x77);
        e.mem.set_u32(this + 0x30, data);
        assert_eq!(e.call(0x00d5_e0c0, &args![this]).u8(), 0x78);
        assert_eq!(e.call(0x00d5_e0d0, &args![this]).u32(), 0);
        assert_eq!(e.call(0x00d5_e0e0, &args![this, 9u32]).u32(), 0);
        assert_eq!(e.call(0x00d5_e2d0, &args![this]).u32(), 3);
        assert_eq!(e.call(0x00d5_e2f0, &args![this, 2u32, 99u32]).u32(), 0x77);
    }

    #[test]
    fn fn_00d5e2e0_answers_the_next_key_or_minus_one() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x34, 3);
        assert_eq!(e.call(0x00d5_e2e0, &args![this, 0i32]).i32(), 1);
        assert_eq!(e.call(0x00d5_e2e0, &args![this, 1i32]).i32(), 2);
        assert_eq!(e.call(0x00d5_e2e0, &args![this, 2i32]).i32(), -1);
        assert_eq!(e.call(0x00d5_e2e0, &args![this, -1i32]).i32(), 0);
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(LARGEST_FLOAT, 0x7f7f_ffee);
        e.mem.set_u32(SMALLEST_FLOAT, 0xff7f_ffee);
        for lane in 0..4 {
            e.mem.set_f32(HALVES + lane * 4, 0.5);
        }
        e
    }

    #[test]
    fn fn_00d5e210_collects_the_bounds_of_the_children() {
        let mut e = engine();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x1c, 0x0900_0121);
        // Children whose boxes are written by the double: the pointer picks the box.
        e.register(0x0900_0121, |e, a| {
            let boxes = [
                ([0.0f32, 1.0, 2.0, 3.0], [4.0f32, 5.0, 6.0, 7.0]),
                ([-2.0, 3.0, 1.0, 9.0], [1.0, 8.0, 3.0, 10.0]),
            ];
            assert_eq!(a[1], IDENTITY_TRANSFORM);
            assert_eq!(a[2], 0.0f32.to_bits());
            let index = e.mem.u32(a[0] + 4) as usize;
            for lane in 0..4 {
                e.mem.set_f32(a[3] + lane as u32 * 4, boxes[index].0[lane]);
                e.mem
                    .set_f32(a[3] + 0x10 + lane as u32 * 4, boxes[index].1[lane]);
            }
            Ret::default()
        });
        let children = e.mem.alloc(8);
        for index in 0..2u32 {
            let child = e.mem.alloc(0x10);
            e.mem.set_u32(child, vtable);
            e.mem.set_u32(child + 4, index);
            e.mem.set_u32(children + index * 4, child);
        }
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x44, children);
        e.mem.set_u32(this + 0x48, 2);
        e.call(0x00d5_e210, &args![this, 0x1_0001u32]);
        assert_eq!(e.mem.u8(this + 0x40), 1);
        // Minimum (-2, 1, 1, 3), maximum (4, 8, 6, 10).
        let lanes = |e: &Engine, at: u32| [0u32, 1, 2, 3].map(|lane| e.mem.f32(at + lane * 4));
        assert_eq!(lanes(&e, this + 0x30), [1.0, 4.5, 3.5, 6.5]);
        assert_eq!(lanes(&e, this + 0x20), [3.0, 3.5, 2.5, 3.5]);
    }

    #[test]
    fn fn_00d5e210_without_children_or_flag() {
        let mut e = engine();
        let this = e.mem.alloc(0x80);
        e.mem.set_f32(this + 0x30, 5.0);
        e.call(0x00d5_e210, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 0x40), 0);
        assert_eq!(e.mem.f32(this + 0x30), 5.0);
        // No children: the extreme constants, (max + min) * 0.5 = 0 and (max - min) * 0.5 = inf.
        e.call(0x00d5_e210, &args![this, 1u32]);
        assert_eq!(e.mem.f32(this + 0x30), 0.0);
        assert!(e.mem.f32(this + 0x20).is_infinite());
    }
}
