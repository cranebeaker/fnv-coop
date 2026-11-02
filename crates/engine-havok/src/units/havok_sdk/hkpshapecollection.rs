//! `Havok SDK/hkpshapecollection.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkpShapeCollection` embeds a shape container (an `hkpShapeContainer` interface with
//! its own vtable) at +0x10. The container's virtual slots used here: `+8` first key,
//! `+0xC` next key (`-1` ends the walk), `+0x14` child shape of a key (key, scratch buffer)
//! and, on the child shape, `+0xC` (the supporting projection) and `+0x20` (the ray cast).
//!
//! Havok's profiling records (`RDTSC` time stamps appended to the thread's monitor stream,
//! found with `TlsGetValue` of the index at `0126835c`) are written as the exe does; the
//! engine has no model of `RDTSC`, so the host clock stands in for the time stamps.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    load_vec, maxps, minps, store_bits, store_vec, zip_with,
};

/// `TlsGetValue` import slot and the TLS index of the Havok monitor stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// The end-of-timer label and the start labels of the two timed functions.
const LABEL_END: u32 = 0x010c_41f8;
const LABEL_SHAPE_COLLECT: u32 = 0x010c_e1f8;
const LABEL_MAXIMUM_PROJECTION: u32 = 0x010c_e228;
/// The `float` (about `-FLT_MAX`) a maximum starts from (`0104e148`).
const MINIMUM_FLOAT: u32 = 0x0104_e148;
/// The key that ends a walk of the container.
const NO_KEY: u32 = 0xffff_ffff;

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the host
/// clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte record `(label, time stamp)` to the thread's monitor stream while its
/// cursor (+4) is below its limit (+0xC).
fn profile_record(e: &mut Engine, label: u32) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

// Translated from 00d213f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method of the shape collection (no map name) that does nothing and takes two
/// arguments.
pub fn fn_00d213f0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) {}

// Translated from 00d21400 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method of the shape collection (no map name) that does nothing and takes one
/// argument.
pub fn fn_00d21400(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00d21450 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method of the shape collection (no map name; the timer record it writes is
/// labelled with the string `TtrcShpCollect` at `010ce1f8`): casts a ray through every child
/// the filter lets through and reports whether one hit. Bumps `output + 0x40`, then walks
/// the container's keys (`+8` first, `+0xC` next); for each key, when `input + 0x24` holds
/// a filter object, its slot 0 `(out hkBool, input, this, container, key)` must answer true;
/// then the child shape (`+0x14` of the container, with a scratch buffer) is asked slot
/// `0x20` `(out hit flag, input, output)` and a hit remembers the key. Afterwards the
/// counter is dropped again and the last hit key is stored at `output + 0x20 + 4 *
/// counter`. Writes `key != -1` to the byte `out` and returns `out`.
pub fn fn_00d21450(e: &mut Engine, this: Ptr, out: Ptr, input: Ptr, output: Ptr) -> Ptr {
    profile_record(e, LABEL_SHAPE_COLLECT);
    let depth = e.mem.u32(output.addr() + 0x40).wrapping_add(1);
    e.mem.set_u32(output.addr() + 0x40, depth);
    let container = this.addr() + 0x10;
    let filter = e.mem.u32(input.addr() + 0x24);
    let hit_key = e.with_stack(0x240, |e, base| {
        // the function's stack frame: the hit flag byte, the remembered key, the filter's
        // answer byte and the child shape's scratch buffer
        let hit_flag = base.addr() + 0x13;
        let remembered = base.addr() + 0x14;
        let filter_answer = base.addr() + 0x1b;
        let buffer = base.addr() + 0x20;
        e.mem.set_u32(remembered, NO_KEY);
        let mut key = e.vcall(container, 8, &args![]).u32();
        while key != NO_KEY {
            let accepted = if filter == 0 {
                true
            } else {
                let answer = e
                    .vcall(
                        filter,
                        0,
                        &args![filter_answer, input, this, container, key],
                    )
                    .u32();
                e.mem.u8(answer) != 0
            };
            if accepted {
                let child = e.vcall(container, 0x14, &args![key, buffer]).u32();
                e.vcall(child, 0x20, &args![hit_flag, input, output]);
                if e.mem.u8(hit_flag) != 0 {
                    e.mem.set_u32(remembered, key);
                }
            }
            key = e.vcall(container, 0xc, &args![key]).u32();
        }
        e.mem.u32(remembered)
    });
    let depth = e.mem.u32(output.addr() + 0x40).wrapping_sub(1);
    e.mem.set_u32(output.addr() + 0x40, depth);
    if hit_key != NO_KEY {
        e.mem
            .set_u32(output.addr() + 0x20 + depth.wrapping_mul(4), hit_key);
    }
    profile_record(e, LABEL_END);
    e.mem.set_u8(out.addr(), u8::from(hit_key != NO_KEY));
    out
}

// Translated from 00d216e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollection::getMaximumProjection` (Xbox PDB): the largest of the children's
/// projections on `direction` (child slot `0xC`, result in `ST0`), starting from the
/// `float` at `0104e148`; a child replaces the maximum unless its value is below it
/// (so a NaN replaces it). Timed with the profiling records.
pub fn hkp_shape_collection_get_maximum_projection(
    e: &mut Engine,
    this: Ptr,
    direction: Ptr,
) -> f32 {
    profile_record(e, LABEL_MAXIMUM_PROJECTION);
    let mut maximum = x87_quiet(e.global::<f32>(MINIMUM_FLOAT));
    let container = this.addr() + 0x10;
    e.with_stack(0x210, |e, buffer| {
        let mut key = e.vcall(container, 8, &args![]).u32();
        while key != NO_KEY {
            let child = e.vcall(container, 0x14, &args![key, buffer]).u32();
            let projection = e.vcall(child, 0xc, &args![direction]).f32();
            if !(projection < maximum) {
                maximum = x87_quiet(projection);
            }
            key = e.vcall(container, 0xc, &args![key]).u32();
        }
    });
    profile_record(e, LABEL_END);
    maximum
}

/// The label of the timer record `getAabb` starts with (the string `TthkpShapeCollection::getAabb`).
const LABEL_GET_AABB: u32 = 0x010c_e208;
/// The `float` (about `FLT_MAX`) a minimum starts from (`0104da48`).
const MAXIMUM_FLOAT: u32 = 0x0104_da48;
/// `hkReferencedObject::calcContentStatistics` (`00c90360`, thiscall on the object: collector,
/// the class record).
const BASE_CALC_CONTENT_STATISTICS: u32 = 0x00c9_0360;
/// The class record the shape collection hands its base report.
const CONTENT_CLASS_RECORD: u32 = 0x0126_8910;
/// The vtable of the shape collection, the vtable of its embedded container, and the vtable
/// the constructor stores into the container word first.
const COLLECTION_VTABLE: u32 = 0x010c_c364;
const CONTAINER_VTABLE_FINAL: u32 = 0x010c_c344;
const CONTAINER_VTABLE_FIRST: u32 = 0x010c_6fb0;

// Translated from 00d213e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollection::calcContentStatistics` (Xbox PDB), thiscall with two stack words
/// (`RET 8`): the second word is replaced by the class record `01268910` and the call jumps to
/// the base report (`00c90360`) with the same stack, so the base report is the function's own
/// answer.
pub fn hkp_shape_collection_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    _unused_2: u32,
) {
    e.call(
        BASE_CALC_CONTENT_STATISTICS,
        &args![this, collector, CONTENT_CLASS_RECORD],
    );
}

// Translated from 00d21410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollection::hkpShapeCollection` (Xbox PDB), thiscall with two stack words
/// (`RET 8`): the reference count word at `+6` is 1, the user data at `+8` is 0, the shape type
/// at `+0xC` is the first word, the byte at `+0x14` is 0 and the byte at `+0x15` is the low
/// byte of the second word. The vtable word at `+0x10` is stored twice (first with
/// `010c6fb0`, then with the container vtable `010cc344`) and the object vtable is
/// `010cc364`. Answers `this`.
pub fn hkp_shape_collection_constructor(
    e: &mut Engine,
    this: Ptr,
    shape_type: u32,
    collection_type: u8,
) -> Ptr {
    let at = this.addr();
    e.mem.set_u16(at + 6, 1);
    e.mem.set_u32(at + 8, 0);
    e.mem.set_u32(at + 0xc, shape_type);
    e.mem.set_u32(at + 0x10, CONTAINER_VTABLE_FIRST);
    e.mem.set_u32(at, COLLECTION_VTABLE);
    e.mem.set_u32(at + 0x10, CONTAINER_VTABLE_FINAL);
    e.mem.set_u8(at + 0x14, 0);
    e.mem.set_u8(at + 0x15, collection_type);
    this
}

// Translated from 00d215d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollection::getAabbImpl` (Xbox PDB), thiscall with three stack words (`RET 0xC`)
/// `(transform, tolerance, out)`. Writes a timer record (`TthkpShapeCollection::getAabb`), sets
/// the box `out` to the empty one (the minimum lanes to the `float` at `0104da48`, the maximum
/// lanes to the one at `0104e148`) and walks the container at `this + 0x10` (slot 8 first key,
/// slot `0xC` next key until `-1`, slot `0x14` child shape of a key with a scratch buffer).
/// Each child is asked slot `0x1C` for its box `(transform, tolerance, &box)` and the result is
/// merged with `MINPS`/`MAXPS` (the box so far is the destination operand), lane by lane. The
/// tolerance goes through the x87 stack (`FLD`/`FSTP`). The walk ends with the end record.
pub fn hkp_shape_collection_get_aabb_impl(
    e: &mut Engine,
    this: Ptr,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    let container = this.addr() + 0x10;
    let transform = transform.addr();
    let out = out.addr();
    profile_record(e, LABEL_GET_AABB);
    let start_min: u32 = e.global(MAXIMUM_FLOAT);
    store_bits(e, out, [start_min; 4]);
    let start_max: u32 = e.global(MINIMUM_FLOAT);
    store_bits(e, out + 0x10, [start_max; 4]);
    let tolerance = x87_quiet_bits(tolerance.to_bits());
    // The exe keeps the box of a child (0x20 bytes) and the scratch buffer on its stack.
    e.with_stack(0x240, |e, frame| {
        let child_box = frame.addr() + 0x10;
        let buffer = frame.addr() + 0x30;
        let mut key = e.vcall(container, 8, &args![]).u32();
        while key != NO_KEY {
            let child = e.vcall(container, 0x14, &args![key, buffer]).u32();
            e.vcall(child, 0x1c, &args![transform, tolerance, child_box]);
            let minimum = zip_with(load_vec(e, out), load_vec(e, child_box), minps);
            store_vec(e, out, minimum);
            let maximum = zip_with(
                load_vec(e, out + 0x10),
                load_vec(e, child_box + 0x10),
                maxps,
            );
            store_vec(e, out + 0x10, maximum);
            key = e.vcall(container, 0xc, &args![key]).u32();
        }
    });
    profile_record(e, LABEL_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d2_13e0,
            hkp_shape_collection_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(0x00d2_1410, hkp_shape_collection_constructor(Ptr, u32, u8) -> Ptr),
        entry!(0x00d2_13f0, fn_00d213f0(Ptr, u32, u32)),
        entry!(0x00d2_1400, fn_00d21400(Ptr, u32)),
        entry!(0x00d2_1450, fn_00d21450(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(
            0x00d2_15d0,
            hkp_shape_collection_get_aabb_impl(Ptr, Ptr, f32, Ptr)
        ),
        entry!(
            0x00d2_16e0,
            hkp_shape_collection_get_maximum_projection(Ptr, Ptr) -> f32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, log_double, new_log, Log};
    use std::cell::RefCell;
    use std::rc::Rc;

    const CONTAINER_VTABLE: u32 = 0x0a00_0000;
    const FIRST_KEY: u32 = 0x0a10_0001;
    const NEXT_KEY: u32 = 0x0a10_0002;
    const GET_CHILD: u32 = 0x0a10_0003;
    const CHILD_VTABLE: u32 = 0x0a00_1000;
    const CHILD_PROJECTION: u32 = 0x0a10_0011;
    const CHILD_CAST: u32 = 0x0a10_0012;
    const FILTER_VTABLE: u32 = 0x0a00_2000;
    const FILTER_SLOT: u32 = 0x0a10_0021;

    /// A collection of `children` keys `0..children` (keys are walked in order) with
    /// the profiling stream in place. Child `k` is one shared child object; the doubles
    /// below look at the key.
    fn rig(e: &mut Engine, children: u32) -> (u32, u32) {
        e.map(0x0104_e000, 0x1000);
        e.map(0x0126_8000, 0x1000);
        let mut slots = vec![0u32; 8];
        slots[8 / 4] = FIRST_KEY;
        slots[0xc / 4] = NEXT_KEY;
        slots[0x14 / 4] = GET_CHILD;
        e.put_vtable(CONTAINER_VTABLE, &slots);
        let mut child_slots = vec![0u32; 0x10];
        child_slots[0xc / 4] = CHILD_PROJECTION;
        child_slots[0x20 / 4] = CHILD_CAST;
        e.put_vtable(CHILD_VTABLE, &child_slots);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x10, CONTAINER_VTABLE);
        let child = e.mem.alloc(0x10);
        e.mem.set_u32(child, CHILD_VTABLE);
        e.register_double(FIRST_KEY, move |_, _| Ret {
            eax: if children == 0 { NO_KEY } else { 0 },
            ..Ret::default()
        });
        // the next key needs the child count: keep it in a global of the double
        e.register_double(NEXT_KEY, move |_, a| Ret {
            eax: if a[1] + 1 < children {
                a[1] + 1
            } else {
                NO_KEY
            },
            ..Ret::default()
        });
        e.register_double(GET_CHILD, move |_, _| Ret {
            eax: child,
            ..Ret::default()
        });
        // profiling stream: cursor at +4, limit at +0xC
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0xe0);
        e.mem.set_u32(MONITOR_SLOT, 3);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (this, stream)
    }

    #[test]
    fn empty_methods_take_their_arguments_and_return() {
        let mut e = Engine::new();
        e.call(0x00d2_13f0, &args![1u32, 2u32, 3u32]);
        e.call(0x00d2_1400, &args![1u32, 2u32]);
    }

    #[test]
    fn maximum_projection_keeps_the_largest_and_records_the_timer() {
        let mut e = Engine::new();
        let (this, stream) = rig(&mut e, 3);
        e.mem.set_f32(MINIMUM_FLOAT, -3.4e38);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        let values = [2.0f32, 5.0, 3.0];
        let next = Rc::new(RefCell::new(0usize));
        e.register_double(CHILD_PROJECTION, move |_, a| {
            log.borrow_mut().push(a[1]);
            let mut n = next.borrow_mut();
            let value = values[*n];
            *n += 1;
            Ret {
                st0: f64::from(value),
                ..Ret::default()
            }
        });
        let direction = e.mem.alloc(0x10);
        let got = e.call(0x00d2_16e0, &args![this, direction]).f32();
        assert_eq!(got, 5.0);
        assert_eq!(*seen.borrow(), vec![direction; 3]);
        // two records: the start label and the end label
        assert_eq!(e.mem.u32(stream + 4), stream + 0x20 + 24);
        assert_eq!(e.mem.u32(stream + 0x20), LABEL_MAXIMUM_PROJECTION);
        assert_eq!(e.mem.u32(stream + 0x2c), LABEL_END);
    }

    #[test]
    fn maximum_projection_without_children_is_the_starting_float() {
        let mut e = Engine::new();
        let (this, _) = rig(&mut e, 0);
        e.mem.set_f32(MINIMUM_FLOAT, -1.5);
        let direction = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00d2_16e0, &args![this, direction]).f32(), -1.5);
    }

    #[test]
    fn maximum_projection_lets_a_nan_replace_the_maximum() {
        let mut e = Engine::new();
        let (this, _) = rig(&mut e, 2);
        e.mem.set_f32(MINIMUM_FLOAT, 0.0);
        let next = Rc::new(RefCell::new(0usize));
        e.register_double(CHILD_PROJECTION, move |_, _| {
            let mut n = next.borrow_mut();
            *n += 1;
            Ret {
                st0: if *n == 1 { 4.0 } else { f64::NAN },
                ..Ret::default()
            }
        });
        let direction = e.mem.alloc(0x10);
        assert!(e.call(0x00d2_16e0, &args![this, direction]).f32().is_nan());
    }

    #[test]
    fn walk_without_a_filter_remembers_the_last_hitting_key() {
        let mut e = Engine::new();
        let (this, stream) = rig(&mut e, 3);
        let input = e.mem.alloc(0x40);
        let output = e.mem.alloc(0x80);
        e.mem.set_u32(output + 0x40, 5);
        let casts = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&casts);
        let keys_seen = Rc::new(RefCell::new(0u32));
        e.register_double(CHILD_CAST, move |e, a| {
            // a = [child, hit flag, input, output]; the counter is already bumped
            log.borrow_mut().push((a[2], a[3], e.mem.u32(a[3] + 0x40)));
            let mut n = keys_seen.borrow_mut();
            // children 0 and 1 hit, child 2 does not
            e.mem.set_u8(a[1], u8::from(*n < 2));
            *n += 1;
            Ret::default()
        });
        let out = e.mem.alloc(4);
        let got = e.call(0x00d2_1450, &args![this, out, input, output]).u32();
        assert_eq!(got, out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(
            *casts.borrow(),
            vec![(input, output, 6), (input, output, 6), (input, output, 6)]
        );
        // the counter is back at 5 and the last hit key (1) sits at output + 0x20 + 4 * 5
        assert_eq!(e.mem.u32(output + 0x40), 5);
        assert_eq!(e.mem.u32(output + 0x20 + 20), 1);
        assert_eq!(e.mem.u32(stream + 0x20), LABEL_SHAPE_COLLECT);
        assert_eq!(e.mem.u32(stream + 0x2c), LABEL_END);
    }

    #[test]
    fn walk_with_a_filter_casts_only_through_accepted_keys() {
        let mut e = Engine::new();
        let (this, _) = rig(&mut e, 3);
        let mut slots = vec![0u32; 2];
        slots[0] = FILTER_SLOT;
        e.put_vtable(FILTER_VTABLE, &slots);
        let filter = e.mem.alloc(8);
        e.mem.set_u32(filter, FILTER_VTABLE);
        let input = e.mem.alloc(0x40);
        e.mem.set_u32(input + 0x24, filter);
        let output = e.mem.alloc(0x80);
        let asked = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&asked);
        e.register_double(FILTER_SLOT, move |e, a| {
            // a = [filter, out, input, this, container, key]
            log.borrow_mut().push(a.to_vec());
            e.mem.set_u8(a[1], u8::from(a[5] == 1));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let cast_keys = Rc::new(RefCell::new(0u32));
        let count = Rc::clone(&cast_keys);
        e.register_double(CHILD_CAST, move |e, a| {
            *count.borrow_mut() += 1;
            e.mem.set_u8(a[1], 0);
            Ret::default()
        });
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 9);
        e.call(0x00d2_1450, &args![this, out, input, output]);
        let asked = asked.borrow();
        assert_eq!(asked.len(), 3);
        assert_eq!(asked[0][2..], [input, this, this + 0x10, 0]);
        assert_eq!(*cast_keys.borrow(), 1);
        // no child hit: the result byte is 0 and nothing is stored in the output
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(e.mem.u32(output + 0x20), 0);
        assert_eq!(e.mem.u32(output + 0x40), 0);
    }

    const AABB_CONTAINER_VTABLE: u32 = 0x0a00_3000;
    const AABB_FIRST_KEY: u32 = 0x0a10_0031;
    const AABB_NEXT_KEY: u32 = 0x0a10_0032;
    const AABB_GET_CHILD: u32 = 0x0a10_0033;
    const AABB_CHILD_VTABLE: u32 = 0x0a00_4000;
    const AABB_CHILD_SLOT: u32 = 0x0a10_0041;

    /// A collection whose container walks the keys `0..boxes.len()`; child `k` answers the box
    /// `boxes[k]` (min lanes, then max lanes). The profiling stream is in place; returns the
    /// collection and the stream.
    fn aabb_rig(e: &mut Engine, log: &Log, boxes: &[[f32; 8]]) -> (u32, u32) {
        for page in [0x0104_d000, 0x0104_e000, 0x0126_8000] {
            e.map(page, 0x1000);
        }
        e.set_global(MAXIMUM_FLOAT, 0x7f7f_ffeeu32);
        e.set_global(MINIMUM_FLOAT, 0xff7f_ffeeu32);
        e.put_vtable(
            AABB_CONTAINER_VTABLE,
            &[0, 0, AABB_FIRST_KEY, AABB_NEXT_KEY, 0, AABB_GET_CHILD],
        );
        let mut child_slots = vec![0u32; 8];
        child_slots[0x1c / 4] = AABB_CHILD_SLOT;
        e.put_vtable(AABB_CHILD_VTABLE, &child_slots);
        let shapes: Vec<u32> = boxes
            .iter()
            .map(|_| {
                let shape = e.mem.alloc(0x10);
                e.mem.set_u32(shape, AABB_CHILD_VTABLE);
                shape
            })
            .collect();
        let count = boxes.len() as u32;
        e.register_double(AABB_FIRST_KEY, move |_, _| Ret {
            eax: if count == 0 { NO_KEY } else { 0 },
            ..Ret::default()
        });
        e.register_double(AABB_NEXT_KEY, move |_, a| Ret {
            eax: if a[1] + 1 < count { a[1] + 1 } else { NO_KEY },
            ..Ret::default()
        });
        let child_log = Rc::clone(log);
        let child_shapes = shapes.clone();
        e.register_double(AABB_GET_CHILD, move |_, a| {
            child_log.borrow_mut().push((AABB_GET_CHILD, a.to_vec()));
            Ret {
                eax: child_shapes[a[1] as usize],
                ..Ret::default()
            }
        });
        let box_log = Rc::clone(log);
        let table = boxes.to_vec();
        e.register_double(AABB_CHILD_SLOT, move |e, a| {
            box_log.borrow_mut().push((AABB_CHILD_SLOT, a.to_vec()));
            let which = shapes.iter().position(|shape| *shape == a[0]).unwrap();
            for (lane, value) in table[which].iter().enumerate() {
                e.mem.set_f32(a[3] + 4 * lane as u32, *value);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x10, AABB_CONTAINER_VTABLE);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0xe0);
        e.mem.set_u32(MONITOR_SLOT, 3);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (this, stream)
    }

    #[test]
    fn calc_content_statistics_00d213e0_hands_the_class_record_to_the_base_report() {
        let mut e = Engine::new();
        let log = new_log();
        log_double(&mut e, &log, BASE_CALC_CONTENT_STATISTICS, 0);
        e.call(0x00d2_13e0, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        assert_eq!(
            calls(&log, BASE_CALC_CONTENT_STATISTICS),
            vec![vec![0x1000, 0x2000, 0x0126_8910]]
        );
    }

    #[test]
    fn constructor_00d21410_sets_the_fields_and_both_vtable_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        for offset in 0..0x20 {
            e.mem.set_u8(this + offset, 0xff);
        }
        let got = e
            .call(0x00d2_1410, &args![this, 0x1cu32, 0x1234_5602u32])
            .u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), 0x010c_c364);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u16(this + 4), 0xffff);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x1c);
        assert_eq!(e.mem.u32(this + 0x10), 0x010c_c344);
        assert_eq!(e.mem.u8(this + 0x14), 0);
        assert_eq!(e.mem.u8(this + 0x15), 2);
        assert_eq!(e.mem.u16(this + 0x16), 0xffff);
    }

    #[test]
    fn get_aabb_00d215d0_merges_the_boxes_of_the_children() {
        let mut e = Engine::new();
        let log = new_log();
        let boxes = [
            [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            [0.0, 3.0, 2.0, 9.0, 4.0, 7.0, 6.0, 1.0],
        ];
        let (this, stream) = aabb_rig(&mut e, &log, &boxes);
        let out = e.mem.alloc(0x20);
        // A signalling NaN tolerance goes through the x87 stack and comes out quiet.
        e.call(0x00d2_15d0, &args![this, 0x7000u32, 0x7f80_0001u32, out]);
        let minimum: Vec<f32> = (0..4).map(|lane| e.mem.f32(out + 4 * lane)).collect();
        let maximum: Vec<f32> = (0..4)
            .map(|lane| e.mem.f32(out + 0x10 + 4 * lane))
            .collect();
        assert_eq!(minimum, vec![0.0, 2.0, 2.0, 4.0]);
        assert_eq!(maximum, vec![5.0, 7.0, 7.0, 8.0]);
        let children = calls(&log, AABB_GET_CHILD);
        assert_eq!(
            children
                .iter()
                .map(|call| (call[0], call[1]))
                .collect::<Vec<_>>(),
            vec![(this + 0x10, 0), (this + 0x10, 1)]
        );
        let asked = calls(&log, AABB_CHILD_SLOT);
        assert_eq!(asked.len(), 2);
        for call in &asked {
            assert_eq!(call[1..3], [0x7000, 0x7fc0_0001]);
        }
        assert_eq!(e.mem.u32(stream + 4), stream + 0x20 + 24);
        assert_eq!(e.mem.u32(stream + 0x20), LABEL_GET_AABB);
        assert_eq!(e.mem.u32(stream + 0x2c), LABEL_END);
    }

    #[test]
    fn get_aabb_00d215d0_takes_a_nan_lane_from_the_child_and_keeps_the_empty_box_without_children()
    {
        let mut e = Engine::new();
        let log = new_log();
        let nan = f32::NAN;
        let (this, _) = aabb_rig(&mut e, &log, &[[nan, 1.0, 1.0, 1.0, nan, 2.0, 2.0, 2.0]]);
        let out = e.mem.alloc(0x20);
        e.call(0x00d2_15d0, &args![this, 0u32, 0.5f32, out]);
        assert!(e.mem.f32(out).is_nan());
        assert!(e.mem.f32(out + 0x10).is_nan());
        assert_eq!(e.mem.f32(out + 4), 1.0);
        assert_eq!(e.mem.f32(out + 0x14), 2.0);
        // No child: the box stays the empty one, a splat of the two start floats.
        let mut e = Engine::new();
        let log = new_log();
        let (this, _) = aabb_rig(&mut e, &log, &[]);
        let out = e.mem.alloc(0x20);
        e.call(0x00d2_15d0, &args![this, 0u32, 0.5f32, out]);
        for lane in 0..4 {
            assert_eq!(e.mem.u32(out + 4 * lane), 0x7f7f_ffee);
            assert_eq!(e.mem.u32(out + 0x10 + 4 * lane), 0xff7f_ffee);
        }
        assert!(calls(&log, AABB_GET_CHILD).is_empty());
    }
}
