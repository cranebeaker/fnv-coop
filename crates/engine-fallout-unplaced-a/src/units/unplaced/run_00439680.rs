//! `(unplaced)/run_00439680` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00439680 (decompiled, FalloutNV.exe 1.4.0.525)
/// A three-word setter (unnamed in the map; `thiscall`, `(&point)`): copies the three words
/// at `point` to `this + 0x18`.
pub fn fn_00439680(e: &mut Engine, this: Ptr, point: Ptr) {
    for word in 0..3u32 {
        let value = e.mem.u32(point.addr().wrapping_add(4 * word));
        e.mem
            .set_u32(this.addr().wrapping_add(0x18 + 4 * word), value);
    }
}

// Translated from 004396b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A nine-word setter (unnamed in the map; `thiscall`, `RET 4`): copies the nine words at
/// `source` (a 3x3 matrix by its size; `REP MOVSD`, first word first) to `this + 0x24`.
pub fn fn_004396b0(e: &mut Engine, this: Ptr, source: Ptr) {
    for word in 0..9u32 {
        let value = e.mem.u32(source.addr().wrapping_add(4 * word));
        e.mem
            .set_u32(this.addr().wrapping_add(0x24 + 4 * word), value);
    }
}

/// The allocator `00aa13e0` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x00aa_13e0;
/// `00430830` (`ECX` = the reference): the address of three `float`s (the half extents).
const EXTENTS_00430830: u32 = 0x0043_0830;
/// `00408840` (`effectsetting.cpp`; `cdecl`, one `float` word): a float in `ST0`.
const EXTENT_VALUE_00408840: u32 = 0x0040_8840;
/// The `double` threshold the three values are compared with (0.01 as a `float`).
const THRESHOLD: u32 = 0x0101_6408;
/// `BSMultiBoundAABB::BSMultiBoundAABB` (engine map, `00c37ec0`, `ECX`), 0x24 bytes.
const MULTI_BOUND_AABB: u32 = 0x00c3_7ec0;
/// `BSMultiBoundOBB::BSMultiBoundOBB` (engine map, `00c36310`, `ECX`), 0x48 bytes.
const MULTI_BOUND_OBB: u32 = 0x00c3_6310;
/// `BSMultiBoundRoom::BSMultiBoundRoom` (engine map, `00c39640`, `ECX`), 0xec bytes.
const MULTI_BOUND_ROOM: u32 = 0x00c3_9640;
/// `BSMultiBound::BSMultiBound` (engine map, `00c35e00`, `ECX`), 0x10 bytes.
const MULTI_BOUND: u32 = 0x00c3_5e00;
/// `TESObjectREFR::GetOrientation` (engine map, `0056fa00`, `ECX`, one stack word: the
/// 0x24-byte matrix it fills); answers a matrix address.
const GET_ORIENTATION: u32 = 0x0056_fa00;
/// `TESObjectREFR::GetMultiBoundHalfExtent` (engine map, `00569880`, `ECX`): a point
/// address.
const GET_HALF_EXTENT: u32 = 0x0056_9880;
/// `004addc0` (`ECX` = the multi-bound, one stack word: the shape).
const SET_SHAPE_004ADDC0: u32 = 0x004a_ddc0;
/// `00439920` (named `BSFaceGenNiNode::SetAnimationData` by folded code; `ECX` = the room,
/// one stack word: the multi-bound).
const SET_BOUND_00439920: u32 = 0x0043_9920;

/// Constructs an object of `size` bytes with `constructor`, or answers null when the
/// allocation fails.
fn construct(e: &mut Engine, size: u32, constructor: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![size]).u32();
    if block == 0 {
        0
    } else {
        e.call(constructor, &args![block]).u32()
    }
}

// Translated from 004396e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004396e0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eebc6f, state variable around each constructor) and the frame's cookie word are not translated.
/// Unnamed in the engine map (`cdecl`, one word: a reference; called by
/// `GetMultiBoundRoom` at `00569a22`): null for a null reference. Otherwise picks the
/// shape: when any of the three floats at `00430830`'s address, through `00408840`, is not
/// below the `double` at `01016408` (0.01; NaN counts as not below), a `BSMultiBoundOBB`
/// (0x48 bytes) given the reference's orientation ([`fn_004396b0`] with
/// `TESObjectREFR::GetOrientation`'s matrix) and half extent ([`fn_00439680`]); else a
/// `BSMultiBoundAABB` (0x24 bytes) given the half extent. The shape's vtable slot +0xb8
/// gets the reference's slot +0x1f4 word. Then a `BSMultiBoundRoom` (0xec bytes) and a
/// `BSMultiBound` (0x10 bytes) are made, the shape set on the bound (`004addc0`) and the
/// bound on the room (`00439920`). Answers the room.
pub fn fn_004396e0(e: &mut Engine, reference: Ptr) -> u32 {
    if reference.is_null() {
        return 0;
    }
    let threshold: f64 = e.global(THRESHOLD);
    let mut oriented = false;
    for index in 0..3u32 {
        let extents = e.call(EXTENTS_00430830, &args![reference]).u32();
        let bits = x87_quiet_bits(e.mem.u32(extents + 4 * index));
        let value = e.call(EXTENT_VALUE_00408840, &args![bits]).f64();
        // `FCOMP`, `TEST AH,5`, `JP`: taken unless the value is below the threshold.
        #[allow(clippy::neg_cmp_op_on_partial_ord)]
        if !(value < threshold) {
            oriented = true;
            break;
        }
    }
    let shape = if oriented {
        let shape = construct(e, 0x48, MULTI_BOUND_OBB);
        // The matrix buffer is a local of the frame, alive until the function returns.
        e.with_stack(0x24, |e, buffer| {
            let matrix = e.call(GET_ORIENTATION, &args![reference, buffer]).u32();
            fn_004396b0(e, Ptr::new(shape), Ptr::new(matrix));
        });
        let extent = e.call(GET_HALF_EXTENT, &args![reference]).u32();
        fn_00439680(e, Ptr::new(shape), Ptr::new(extent));
        shape
    } else {
        let shape = construct(e, 0x24, MULTI_BOUND_AABB);
        let extent = e.call(GET_HALF_EXTENT, &args![reference]).u32();
        fn_00439680(e, Ptr::new(shape), Ptr::new(extent));
        shape
    };
    let word = e.vcall(reference.addr(), 0x1f4, &args![]).u32();
    e.vcall(shape, 0xb8, &args![word]);
    let room = construct(e, 0xec, MULTI_BOUND_ROOM);
    let bound = construct(e, 0x10, MULTI_BOUND);
    e.call(SET_SHAPE_004ADDC0, &args![bound, shape]);
    e.call(SET_BOUND_00439920, &args![room, bound]);
    room
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0043_96e0, fn_004396e0(Ptr) -> u32),
        entry!(0x00439680, fn_00439680(Ptr, Ptr)),
        entry!(0x0043_96b0, fn_004396b0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00439680_copies_three_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        let point = e.mem.alloc(12);
        for word in 0..3u32 {
            e.mem.set_u32(point + 4 * word, 0x10 + word);
        }
        e.call(0x00439680, &args![this, point]);
        assert_eq!(
            [0u32, 4, 8].map(|o| e.mem.u32(this + 0x18 + o)),
            [0x10, 0x11, 0x12]
        );
    }

    #[test]
    fn fn_004396b0_copies_nine_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        let source = e.mem.alloc(0x24);
        for word in 0..9u32 {
            e.mem.set_u32(source + 4 * word, 0x20 + word);
        }
        e.call(0x0043_96b0, &args![this, source]);
        let copied: Vec<u32> = (0..9u32).map(|w| e.mem.u32(this + 0x24 + 4 * w)).collect();
        assert_eq!(copied, (0x20..0x29).collect::<Vec<u32>>());
        assert_eq!(e.mem.u32(this + 0x20), 0);
    }
}

#[cfg(test)]
mod room_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// A reference whose three extents are `values`; `00408840` answers its float.
    fn engine(values: [f32; 3]) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.set_global(THRESHOLD, f64::from(0.01f32));
        let reference = e.mem.alloc(0x40);
        e.mem.set_u32(reference, 0x0070_0000);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(reference + 0x10 + 4 * i as u32, v.to_bits());
        }
        e.register(EXTENTS_00430830, |_, a| (a[0] + 0x10).into_ret());
        e.register(EXTENT_VALUE_00408840, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        let blocks = std::rc::Rc::new(std::cell::RefCell::new(0x0080_0000u32));
        e.register_double(ALLOCATE, move |e, a| {
            let mut next = blocks.borrow_mut();
            let block = *next;
            e.map(block, a[0]);
            *next += 0x1000;
            block.into_ret()
        });
        for ctor in [
            MULTI_BOUND_AABB,
            MULTI_BOUND_OBB,
            MULTI_BOUND_ROOM,
            MULTI_BOUND,
        ] {
            e.register(ctor, |e, a| {
                e.mem.set_u32(a[0], 0x0071_0000);
                a[0].into_ret()
            });
        }
        let point = e.mem.alloc(12);
        for i in 0..3 {
            e.mem.set_u32(point + 4 * i, 0x100 + i);
        }
        e.register_double(GET_HALF_EXTENT, move |_, _| point.into_ret());
        e.register(GET_ORIENTATION, |e, a| {
            for i in 0..9 {
                e.mem.set_u32(a[1] + 4 * i, 0x200 + i);
            }
            a[1].into_ret()
        });
        let mut slots = vec![0u32; 0x7e];
        slots[0x1f4 / 4] = 0x0060_01f4;
        e.put_vtable(0x0070_0000, &slots);
        let mut slots = vec![0u32; 0x2f];
        slots[0xb8 / 4] = 0x0060_00b8;
        e.put_vtable(0x0071_0000, &slots);
        e.register(0x0060_01f4, |_, _| 0x4444u32.into_ret());
        e.register(0x0060_00b8, |_, _| Ret::default());
        e.register(SET_SHAPE_004ADDC0, |_, _| Ret::default());
        e.register(SET_BOUND_00439920, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, reference)
    }

    fn calls(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn flat_extents_make_an_aabb_room() {
        let (mut e, reference) = engine([0.001, -0.5, 0.0]);
        let room = e.call(0x0043_96e0, &args![reference]).u32();
        let log = e.call_log.take().unwrap();
        // Allocation order: shape, room, bound.
        assert_eq!(room, 0x0080_1000);
        assert_eq!(calls(&log, MULTI_BOUND_AABB), vec![vec![0x0080_0000]]);
        assert!(calls(&log, MULTI_BOUND_OBB).is_empty());
        assert_eq!(calls(&log, EXTENT_VALUE_00408840).len(), 3);
        assert_eq!(e.mem.u32(0x0080_0000 + 0x18), 0x100);
        assert_eq!(e.mem.u32(0x0080_0000 + 0x20), 0x102);
        assert_eq!(calls(&log, 0x0060_00b8), vec![vec![0x0080_0000, 0x4444]]);
        assert_eq!(
            calls(&log, SET_SHAPE_004ADDC0),
            vec![vec![0x0080_2000, 0x0080_0000]]
        );
        assert_eq!(
            calls(&log, SET_BOUND_00439920),
            vec![vec![0x0080_1000, 0x0080_2000]]
        );
    }

    #[test]
    fn a_large_or_nan_extent_makes_an_obb_room() {
        for values in [[0.0, 0.5, 0.0], [f32::NAN, 0.0, 0.0]] {
            let (mut e, reference) = engine(values);
            e.call(0x0043_96e0, &args![reference]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, MULTI_BOUND_OBB), vec![vec![0x0080_0000]]);
            assert_eq!(e.mem.u32(0x0080_0000 + 0x24), 0x200);
            assert_eq!(e.mem.u32(0x0080_0000 + 0x44), 0x208);
            assert_eq!(e.mem.u32(0x0080_0000 + 0x1c), 0x101);
        }
    }

    #[test]
    fn a_null_reference_answers_null() {
        let (mut e, _) = engine([0.0; 3]);
        assert_eq!(e.call(0x0043_96e0, &args![0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
