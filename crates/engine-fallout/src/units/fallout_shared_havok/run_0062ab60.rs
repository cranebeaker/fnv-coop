//! `fallout shared/havok/run_0062ab60` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// `008256d0` (unnamed in the engine map): `thiscall` on a list node, plain `RET`; answers a byte
/// that ends the walk.
const NODE_IS_END: u32 = 0x0082_56d0;
/// `006815c0` (unnamed in the engine map): `thiscall`, plain `RET`; answers the address of the
/// held item.
const HELD_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`fallout/interface/menumanager.cpp` in the engine map): `thiscall` on a list
/// node, plain `RET`; the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// The list of records `0062a7e0` (`tesarrowtargetlistener.rs`) keeps.
const RECORD_LIST: u32 = 0x011c_c408;
/// `bhkCollisionObject::GetbhkCollisionObject` (Xbox PDB), `0043b610`: `cdecl`, one word.
const GET_COLLISION_OBJECT: u32 = 0x0043_b610;
/// `006fa820` (unnamed in the engine map): `thiscall`, plain `RET`; the rigid body of a
/// collision object.
const RIGID_BODY_OF: u32 = 0x006f_a820;
/// `0056ded0` (`fallout shared/tesobjectrefr.cpp` in the engine map): `thiscall`, plain `RET`;
/// constructs a transform.
const TRANSFORM_CONSTRUCT: u32 = 0x0056_ded0;
/// `0062aad0` (`tesarrowtargetlistener.rs`): no arguments read; answers `012683b0`.
const TRANSFORM_SOURCE: u32 = 0x0062_aad0;
/// `0062aae0` (`tesarrowtargetlistener.rs`): `thiscall`, `RET 8`; builds a transform.
const TRANSFORM_FROM: u32 = 0x0062_aae0;
/// `hkTransform::setInverse` (Xbox PDB), `00cb21e0`: `thiscall`, `RET 4`.
const TRANSFORM_SET_INVERSE: u32 = 0x00cb_21e0;
/// `hkTransform::setMul` (Xbox PDB), `00cb2240`: `thiscall`, `RET 8`.
const TRANSFORM_SET_MUL: u32 = 0x00cb_2240;
/// Vtable slot of the rigid body's transform getter.
const BODY_TRANSFORM_SLOT: u32 = 0xf4;
/// Vtable slot of the rigid body's centre of mass getter.
const BODY_CENTER_SLOT: u32 = 0xec;
/// `00436aa0` (`fallout shared/extradataobjects.cpp` in the engine map): `thiscall`, plain
/// `RET`.
const TRANSFORM_TRANSLATION: u32 = 0x0043_6aa0;
/// `004a3c90` (unnamed in the engine map): `thiscall`, `RET 4`; a vector copy.
const VECTOR_COPY: u32 = 0x004a_3c90;
/// `00403e20` (`fallout shared/magic/effectitem.cpp` in the engine map): `thiscall` on a setting,
/// plain `RET`; answers the address of its `float`.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The setting whose `float` weighs an arrow (`0062a7e0` uses the same one).
const ARROW_WEIGHT: u32 = 0x011c_f8b0;
/// `005dbf20` (`fallout shared/tesscriptfunctions.cpp` in the engine map): `thiscall`, `RET 4`,
/// a `float`; a vector of that `float`.
const VECTOR_SPLAT: u32 = 0x005d_bf20;
/// `00627920` (unnamed in the engine map): `thiscall`, `RET 4`; a vector multiply in place.
const VECTOR_MUL: u32 = 0x0062_7920;
/// `00458a10` (`fallout shared/tes.cpp` in the engine map): `thiscall`, `RET 8`; a vector
/// difference.
const VECTOR_SUB: u32 = 0x0045_8a10;
/// `bhkCharacterProxy::operator->` (engine map name of `004ae750`, possibly a folded body):
/// `thiscall`, plain `RET`.
const HELD_OBJECT: u32 = 0x004a_e750;
/// `hkpRigidBody::setCenterOfMassLocal` (Xbox PDB), `00ca85f0`: `thiscall`, `RET 4`.
const SET_CENTER_OF_MASS_LOCAL: u32 = 0x00ca_85f0;
/// The `float` `0062ab60` scales the moved offset by (`1.5` in the exe).
const OFFSET_SCALE: u32 = 0x0101_6088;
/// `00560d80` (`fallout shared/tesobjectrefr.cpp` in the engine map): `thiscall`, plain `RET`.
const BODY_WORD: u32 = 0x0056_0d80;
/// `00553f70` (`fallout shared/tesobjectcell.cpp` in the engine map): `thiscall`, `RET 4`.
const VECTOR_APPLY: u32 = 0x0055_3f70;
/// `00561690` (`fallout shared/tesobjectrefr.cpp` in the engine map): `thiscall`, `RET 4`.
const BODY_MOVE: u32 = 0x0056_1690;
/// `00401030` (unnamed in the engine map): `cdecl`, one word; the delete.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00905330` (`fallout/ai/highprocess.cpp` in the engine map): `thiscall` on the list, `RET 4`,
/// the address of the word holding the record; the removal.
const LIST_REMOVE: u32 = 0x0090_5330;
/// `00561240` (`fallout shared/tesobjectrefr.cpp` in the engine map): `thiscall`, `RET 4`.
const VECTOR_EXPAND: u32 = 0x0056_1240;
/// The `float` 0.5 of the normalizer (`01016248`).
const HALF: u32 = 0x0101_6248;
/// The `float` 3.0 of the normalizer (`01017718`).
const THREE: u32 = 0x0101_7718;
/// The `float` 0.0 of the normalizer (`01011d78`).
const ZERO: u32 = 0x0101_1d78;
/// `00559450` (unnamed in the engine map): `thiscall`, plain `RET`.
const OBJECT_OF: u32 = 0x0055_9450;
/// `004a3a40` (`fallout shared/bgsdecalmanager.cpp` in the engine map): `cdecl`, one word.
const REFERENCE_OF: u32 = 0x004a_3a40;
/// `00624020` (unnamed in the engine map): `thiscall`, plain `RET`.
const FILTER_HOLDER: u32 = 0x0062_4020;
/// `0043b540` (`fallout shared/modelloader.cpp` in the engine map): `thiscall`, plain `RET`.
const FILTER_WORD: u32 = 0x0043_b540;
/// `008c71b0` (engine map name `BGSChangeFlags::IsForcedChange`, a folded body: `thiscall`,
/// `RET 4`, a word; fills a 4-byte record).
const RECORD_FROM_WORD: u32 = 0x008c_71b0;
/// `0043b4d0` (unnamed in the engine map): `thiscall`, plain `RET`; the layer of a filter record.
const LAYER_OF: u32 = 0x0043_b4d0;
/// `00457880` (`fallout shared/tes.cpp` in the engine map): `thiscall` on the object at
/// `011dea10`, `RET 4`.
const LAYER_17_LOOKUP: u32 = 0x0045_7880;
/// The global holding the object `00457880` is called on.
const LAYER_17_OWNER: u32 = 0x011d_ea10;
/// `00c80020` (`BSHavok/bhkutilfunctions.obj` in the engine map): `cdecl`, one word.
const HAVOK_LOOKUP: u32 = 0x00c8_0020;
/// `006205a0` (`run_00620230.rs`): `fastcall`; the low five bits of the word at `+0x10`.
const LOW_BITS: u32 = 0x0062_05a0;
/// `004a8f20` (unnamed in the engine map): `cdecl`, two words.
const REFERENCE_LOOKUP: u32 = 0x004a_8f20;
/// `004ae6a0` (unnamed in the engine map): `thiscall`, plain `RET`.
const HAVOK_OBJECT: u32 = 0x004a_e6a0;
/// Vtable slot of the shape getter.
const SHAPE_SLOT: u32 = 0xcc;
/// Vtable slot of the shape's material of a key.
const MATERIAL_SLOT: u32 = 0xe0;
/// `0043b4f0` (`fallout shared/modelloader.cpp` in the engine map): `thiscall`, `RET 4`; fills a
/// 4-byte record.
const FILTER_RECORD: u32 = 0x0043_b4f0;
/// `00624070` (`run_00624070.rs`): `thiscall` on the record, plain `RET`; answers a byte.
const RECORD_TEST: u32 = 0x0062_4070;
/// `006240b0` (`run_00624070.rs`): `thiscall` on the record, plain `RET`; answers a kind.
const RECORD_KIND: u32 = 0x0062_40b0;
/// The `double` 0.0 `0062b310` compares the mean with.
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// The setting `0062b310` multiplies by for a mean of zero or less, and for the first group of
/// kinds.
const FIRST_FACTOR: u32 = 0x011d_0e64;
/// The setting `0062b310` multiplies by for the second group of kinds.
const SECOND_FACTOR: u32 = 0x011d_00cc;

/// The vector at `address` as four lanes.
fn load4(e: &Engine, address: u32) -> [f32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.f32(address.wrapping_add(offset)))
}

fn store4(e: &mut Engine, address: u32, lanes: [f32; 4]) {
    for (i, lane) in lanes.iter().enumerate() {
        e.mem
            .set_u32(address.wrapping_add(4 * i as u32), lane.to_bits());
    }
}

// Translated from 0062b0a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062b0a0 - /GS cookie check (00ec408c) not translated
/// Unnamed in the engine map. `thiscall`, `RET 8`. The three-lane dot product of `this` and
/// `other` (`MULPS`, `SHUFPS`/`ADDSS`): `[z + (y + x), z, z, z]` of the lane products, copied
/// into `out` (`004a3c90`). Answers `out`.
pub fn fn_0062b0a0(e: &mut Engine, this: Ptr, out: u32, other: u32) -> u32 {
    let a = load4(e, this.addr());
    let b = load4(e, other);
    let p = [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]];
    let first = p[1] + p[0];
    let sum = [p[2] + first, p[2], p[2], p[2]];
    e.with_stack(0x10, |e, local| {
        store4(e, local.addr(), sum);
        e.call(VECTOR_COPY, &args![out, local]);
    });
    out
}

// Translated from 0062af00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062af00 - /GS cookie check (00ec408c) not translated
/// Unnamed in the engine map. `thiscall`, `RET 4`. The inverse length of `this`: the squared
/// length (`0062b0a0` with itself, read through `006815c0`), its `RSQRTSS` estimate refined once
/// (`(0.5 * r) * (3 - (s * r) * r)` with the constants at `01016248`, `01017718`), 0 when the
/// square equals the constant at `01011d78` (`CMPEQSS`/`ANDNPS`); lanes 1 to 3 are 0. Copied
/// into `out` (`004a3c90`). Answers `out`.
pub fn fn_0062af00(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    let half: f32 = e.global(HALF);
    let three: f32 = e.global(THREE);
    let zero: f32 = e.global(ZERO);
    e.with_stack(0x20, |e, frame| {
        let dot = frame.addr();
        let result = frame.addr().wrapping_add(0x10);
        let answer = fn_0062b0a0(e, this, dot, this.addr());
        let held = e.call(HELD_ITEM, &args![answer]).u32();
        let square = e.mem.f32(held);
        let estimate = rsqrt_estimate(square);
        let scaled = square * estimate;
        let step = three - scaled * estimate;
        let value = (half * estimate) * step;
        let lane = if square == zero { 0.0 } else { value };
        store4(e, result, [lane, 0.0, 0.0, 0.0]);
        e.call(VECTOR_COPY, &args![out, result]);
    });
    out
}

// Translated from 0062aea0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062aea0 - /GS cookie check (00ec408c) not translated
/// Unnamed in the engine map. `fastcall`: normalizes the vector `this`. The inverse length
/// (`0062af00`, `RET 4`, into a stack vector) goes through `00561240` (`RET 4`) on that vector
/// with a second stack vector (the word the exe pushed before `0062af00` and left on the stack),
/// and `this` becomes that second vector times `this`, lane by lane (`MULPS`).
pub fn fn_0062aea0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x20, |e, frame| {
        let length = frame.addr();
        let expanded = frame.addr().wrapping_add(0x10);
        let answer = fn_0062af00(e, this, length);
        e.call(VECTOR_EXPAND, &args![answer, expanded]);
        let a = load4(e, expanded);
        let b = load4(e, this.addr());
        store4(
            e,
            this.addr(),
            [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]],
        );
    });
}

// Translated from 0062ab60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062ab60 - /GS cookie check (00ec408c) not translated
/// Unnamed in the engine map. `cdecl`, two words (`source`, `object`): undoes `0062a7e0` for an
/// arrow. Finds the record of `object` in the list `011cc408` (its word at `+4`); with one and a
/// rigid body (`0043b610`, `006fa820`), moves the body's centre of mass back: the source's
/// translation relative to the body (as in `0062a7e0`) weighted by the arrow weight (`00403e20`
/// on `011cf8b0`) taken from the centre weighted by the record's mass (`00458a10`), divided by
/// `mass - weight` (`setCenterOfMassLocal`). The same difference, normalized (`0062aea0`) and
/// scaled by the `float` at `01016088`, goes through `00553f70` (with what `00560d80` answers)
/// to `00561690` on the body. The record's mass drops by the weight and its count by one; at
/// zero or below it is deleted (`00401030`) and removed from the list (`00905330`).
pub fn fn_0062ab60(e: &mut Engine, source: u32, object: u32) {
    // The frame below `EBP`: 0x1d0 bytes, a local at `EBP - x` is at `frame + 0x1d0 - x`.
    e.with_stack(0x1d0, |e, frame| {
        let at = |x: u32| frame.addr().wrapping_add(0x1d0).wrapping_sub(x);
        let record_word = at(0x8);
        e.mem.set_u32(record_word, 0);
        if e.call(NODE_IS_END, &args![RECORD_LIST]).u8() == 0 {
            let mut node = RECORD_LIST;
            while node != 0
                && e.call(NODE_IS_END, &args![node]).u8() == 0
                && e.mem.u32(record_word) == 0
            {
                let item = e.call(HELD_ITEM, &args![node]).u32();
                let record = e.mem.u32(item);
                if record != 0 && e.mem.u32(record.wrapping_add(4)) == object {
                    e.mem.set_u32(record_word, record);
                }
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
        }
        if e.mem.u32(record_word) == 0 {
            return;
        }
        let collision = if object == 0 {
            0
        } else {
            e.call(GET_COLLISION_OBJECT, &args![object]).u32()
        };
        let body = if collision == 0 {
            0
        } else {
            e.call(RIGID_BODY_OF, &args![collision]).u32()
        };
        if body == 0 {
            return;
        }
        let record = e.mem.u32(record_word);
        let mass = x87_quiet_bits(e.mem.u32(record.wrapping_add(8)));
        let body_transform = at(0x70);
        let inverse = at(0xb0);
        let source_transform = at(0xf0);
        let relative = at(0x130);
        let translation = at(0x140);
        let weight_vector = at(0x150);
        let center = at(0x160);
        let mass_vector = at(0x170);
        let difference = at(0x180);
        let scale_vector = at(0x1a0);
        let offset = at(0x1b0);
        let offset_scale = at(0x1c0);
        e.call(TRANSFORM_CONSTRUCT, &args![body_transform]);
        e.call(TRANSFORM_CONSTRUCT, &args![inverse]);
        let rotation = e.call(TRANSFORM_SOURCE, &args![]).u32();
        e.call(TRANSFORM_FROM, &args![source_transform, rotation, source]);
        e.call(TRANSFORM_CONSTRUCT, &args![relative]);
        e.vcall(body, BODY_TRANSFORM_SLOT, &args![body_transform]);
        e.call(TRANSFORM_SET_INVERSE, &args![inverse, body_transform]);
        e.call(
            TRANSFORM_SET_MUL,
            &args![relative, inverse, source_transform],
        );
        let moved = e.call(TRANSFORM_TRANSLATION, &args![relative]).u32();
        e.call(VECTOR_COPY, &args![translation, moved]);
        let weight = e.call(SETTING_VALUE, &args![ARROW_WEIGHT]).u32();
        let weight_bits = x87_quiet_bits(e.mem.u32(weight));
        e.call(VECTOR_SPLAT, &args![weight_vector, weight_bits]);
        e.call(VECTOR_MUL, &args![translation, weight_vector]);
        e.call(HELD_ITEM, &args![center]);
        e.vcall(body, BODY_CENTER_SLOT, &args![center]);
        e.call(VECTOR_SPLAT, &args![mass_vector, mass]);
        e.call(VECTOR_MUL, &args![center, mass_vector]);
        e.call(HELD_ITEM, &args![difference]);
        e.call(VECTOR_SUB, &args![difference, center, translation]);
        let weight = e.call(SETTING_VALUE, &args![ARROW_WEIGHT]).u32();
        let remaining = x87_sub(x87_load32(mass), x87_m32(e.mem.u32(weight)));
        let inverse_remaining = x87_store32(x87_div(1.0, remaining));
        e.call(
            VECTOR_SPLAT,
            &args![scale_vector, x87_quiet_bits(inverse_remaining)],
        );
        e.call(VECTOR_MUL, &args![difference, scale_vector]);
        let havok_body = e.call(HELD_OBJECT, &args![body]).u32();
        e.call(SET_CENTER_OF_MASS_LOCAL, &args![havok_body, difference]);
        e.call(HELD_ITEM, &args![offset]);
        e.call(VECTOR_SUB, &args![offset, center, translation]);
        fn_0062aea0(e, Ptr::new(offset));
        let scale = x87_quiet_bits(e.mem.u32(OFFSET_SCALE));
        e.call(VECTOR_SPLAT, &args![offset_scale, scale]);
        e.call(VECTOR_MUL, &args![offset, offset_scale]);
        let word = e.call(BODY_WORD, &args![body]).u32();
        e.call(VECTOR_APPLY, &args![offset, word]);
        e.call(BODY_MOVE, &args![body, offset]);
        let weight = e.call(SETTING_VALUE, &args![ARROW_WEIGHT]).u32();
        let remaining = x87_sub(x87_load32(mass), x87_m32(e.mem.u32(weight)));
        let record = e.mem.u32(record_word);
        e.mem
            .set_u32(record.wrapping_add(8), x87_store32(remaining));
        let count = e.mem.u32(record.wrapping_add(0xc)).wrapping_sub(1);
        e.mem.set_u32(record.wrapping_add(0xc), count);
        if (e.mem.u32(record.wrapping_add(0xc)) as i32) <= 0 {
            e.call(OPERATOR_DELETE, &args![record]);
            e.call(LIST_REMOVE, &args![RECORD_LIST, record_word]);
        }
    });
}

// Translated from 0062b150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `cdecl`, two words. With a reference for `object` (`00559450`,
/// `004a3a40`): `004a8f20` with the reference's low five bits (`006205a0`) and 0x23. Without:
/// builds a filter record from `object` (`00624020`, `0043b540`, `008c71b0`) and, for layer 0x11
/// (`0043b4d0`), answers `00457880(other)` on the object at `011dea10`, otherwise
/// `00c80020(object)`.
pub fn fn_0062b150(e: &mut Engine, object: u32, other: u32) -> u32 {
    let inner = e.call(OBJECT_OF, &args![object]).u32();
    let reference = e.call(REFERENCE_OF, &args![inner]).u32();
    if reference != 0 {
        let bits = e.call(LOW_BITS, &args![reference]).u32();
        return e.call(REFERENCE_LOOKUP, &args![bits, 0x23u32]).u32();
    }
    let holder = e.call(FILTER_HOLDER, &args![object]).u32();
    let word = e.call(FILTER_WORD, &args![holder]).u32();
    e.with_stack(4, |e, record| {
        e.call(RECORD_FROM_WORD, &args![record, word]);
        if e.call(LAYER_OF, &args![record]).u32() == 0x11 {
            let owner = e.mem.u32(LAYER_17_OWNER);
            e.call(LAYER_17_LOOKUP, &args![owner, other]).u32()
        } else {
            e.call(HAVOK_LOOKUP, &args![object]).u32()
        }
    })
}

// Translated from 0062b1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `cdecl`, one word: the `float` (`00403e20`) of the setting for a
/// material number, in `ST0`: 0 to 9 and 0x18, 0x19 each have their own setting (the compiler's
/// byte map and jump table at `0062b2f0` / `0062b2bc`), everything else answers 0.
pub fn fn_0062b1e0(e: &mut Engine, material: u32) -> f64 {
    let setting = match material {
        0 => 0x011d_06d0,
        1 => 0x011d_0e40,
        2 => 0x011d_0c54,
        3 => 0x011d_0bf4,
        4 => 0x011d_0910,
        5 => 0x011d_0670,
        6 => 0x011d_10e0,
        7 => 0x011d_0298,
        8 => 0x011d_141c,
        9 => 0x011d_1224,
        0x18 => 0x011c_ffac,
        0x19 => 0x011d_088c,
        _ => return 0.0,
    };
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_load32(e.mem.u32(value))
}

// Translated from 0062b480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: the first key of the shape container at `+0x10` of the
/// Havok object (`004ae750`; its vtable slot `+8`).
pub fn fn_0062b480(e: &mut Engine, this: Ptr) -> u32 {
    let held = e.call(HELD_OBJECT, &args![this]).u32();
    e.vcall(held.wrapping_add(0x10), 8, &args![]).u32()
}

// Translated from 0062b4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`: the key after `key` of the same container
/// (its vtable slot `+0xc`).
pub fn fn_0062b4b0(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let held = e.call(HELD_OBJECT, &args![this]).u32();
    e.vcall(held.wrapping_add(0x10), 0xc, &args![key]).u32()
}

// Translated from 0062b310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `cdecl`, one word; a `float` in `ST0`. The mean material value
/// (`0062b1e0`) of the shape of `object`'s Havok object (`004ae6a0`, vtable `+0xcc`): over its
/// keys (`0062b480`, `0062b4b0` until -1; material by vtable `+0xe0`), summed as `float`s and
/// divided by the count (`FILD`, `FDIVR`); without a shape, the value of `object`'s own low five
/// bits (`006205a0`). Times a factor from the filter record (`0043b4f0`): 1 when `00624070`
/// answers 0; else the setting `011d0e64` for a mean not above 0 (or NaN); else by the record's
/// kind (`006240b0`, the byte map at `0062b464`): kinds 1-5, 8, 11, 14 the setting `011d0e64`,
/// kinds 6, 7, 9, 10, 12, 13, 15, 16, 22 the setting `011d00cc`, kinds 17-21 zero, others 1.
pub fn fn_0062b310(e: &mut Engine, object: u32) -> f64 {
    let havok = e.call(HAVOK_OBJECT, &args![object]).u32();
    let shape = e.vcall(havok, SHAPE_SLOT, &args![]).u32();
    let mean = if shape != 0 {
        let mut sum = 0.0f32.to_bits();
        let mut count = 0i32;
        let mut key = fn_0062b480(e, Ptr::new(shape));
        while key != u32::MAX {
            let material = e.vcall(shape, MATERIAL_SLOT, &args![key]).u32();
            let value = fn_0062b1e0(e, material);
            sum = x87_store32(x87_add(value, x87_m32(sum)));
            count = count.wrapping_add(1);
            key = fn_0062b4b0(e, Ptr::new(shape), key);
        }
        x87_store32(x87_div(x87_m32(sum), f64::from(count)))
    } else {
        let bits = e.call(LOW_BITS, &args![havok]).u32();
        x87_store32(fn_0062b1e0(e, bits))
    };
    let mut factor = 1.0f32.to_bits();
    e.with_stack(4, |e, record| {
        e.call(FILTER_RECORD, &args![object, record]);
        if e.call(RECORD_TEST, &args![record]).u8() != 0 {
            let zero: f64 = e.global(DOUBLE_ZERO);
            // `FCOMP`, `TEST AH,0x41`: below, equal or unordered takes the first setting.
            let above = x87_load32(mean) > zero;
            let setting = if above {
                let kind = e.call(RECORD_KIND, &args![record]).u32();
                match kind.wrapping_sub(1) {
                    0..=4 | 7 | 10 | 13 => Some(FIRST_FACTOR),
                    5 | 6 | 8 | 9 | 11 | 12 | 14 | 15 | 21 => Some(SECOND_FACTOR),
                    16..=20 => {
                        factor = 0.0f32.to_bits();
                        None
                    }
                    _ => None,
                }
            } else {
                Some(FIRST_FACTOR)
            };
            if let Some(setting) = setting {
                let value = e.call(SETTING_VALUE, &args![setting]).u32();
                factor = x87_quiet_bits(e.mem.u32(value));
            }
        }
    });
    let product = x87_store32(x87_mul(x87_load32(mean), x87_m32(factor)));
    x87_load32(product)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_ab60, fn_0062ab60(u32, u32)),
        entry!(0x0062_aea0, fn_0062aea0(Ptr)),
        entry!(0x0062_af00, fn_0062af00(Ptr, u32) -> u32),
        entry!(0x0062_b0a0, fn_0062b0a0(Ptr, u32, u32) -> u32),
        entry!(0x0062_b150, fn_0062b150(u32, u32) -> u32),
        entry!(0x0062_b1e0, fn_0062b1e0(u32) -> f64),
        entry!(0x0062_b310, fn_0062b310(u32) -> f64),
        entry!(0x0062_b480, fn_0062b480(Ptr) -> u32),
        entry!(0x0062_b4b0, fn_0062b4b0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(0x0101_1000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_6000, 0x2000);
        e.set_global(HALF, 0.5f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(ZERO, 0.0f32);
        e.set_global(OFFSET_SCALE, 1.5f32);
        e.set_global(DOUBLE_ZERO, 0.0f64);
        // A real vector copy, so the results can be read.
        e.register(VECTOR_COPY, |e, a| {
            for i in 0..4 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, word);
            }
            Ret::default()
        });
        e.register(HELD_ITEM, |_, a| a[0].into_ret());
        e
    }

    fn vector(e: &mut Engine, lanes: [f32; 4]) -> u32 {
        let p = e.mem.alloc(0x10);
        store4(e, p, lanes);
        p
    }

    #[test]
    fn at_0062b0a0_answers_the_three_lane_dot_product() {
        let mut e = engine();
        let a = vector(&mut e, [1.0, 2.0, 3.0, 100.0]);
        let b = vector(&mut e, [4.0, 5.0, 6.0, 100.0]);
        let out = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0062_b0a0, &args![a, out, b]).u32(), out);
        assert_eq!(load4(&e, out), [32.0, 18.0, 18.0, 18.0]);
    }

    #[test]
    fn at_0062af00_answers_the_inverse_length_or_zero() {
        let mut e = engine();
        let a = vector(&mut e, [3.0, 4.0, 0.0, 9.0]);
        let out = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0062_af00, &args![a, out]).u32(), out);
        let lanes = load4(&e, out);
        // One refinement step of the hardware estimate: within a few ULP of 0.2.
        assert!((lanes[0] - 0.2).abs() < 1e-6, "{}", lanes[0]);
        assert_eq!(&lanes[1..], &[0.0, 0.0, 0.0]);
        let zero = vector(&mut e, [0.0, 0.0, 0.0, 9.0]);
        e.call(0x0062_af00, &args![zero, out]);
        assert_eq!(load4(&e, out), [0.0; 4]);
    }

    #[test]
    fn at_0062aea0_scales_this_by_the_expanded_inverse_length() {
        let mut e = engine();
        e.register(VECTOR_EXPAND, |e, a| {
            let lane = e.mem.u32(a[0]);
            for i in 0..4 {
                e.mem.set_u32(a[1] + 4 * i, lane);
            }
            Ret::default()
        });
        let v = vector(&mut e, [0.0, 0.0, 2.0, 1.0]);
        e.call(0x0062_aea0, &args![v]);
        let lanes = load4(&e, v);
        assert!((lanes[2] - 1.0).abs() < 1e-6, "{}", lanes[2]);
        assert!((lanes[3] - 0.5).abs() < 1e-6, "{}", lanes[3]);
        assert_eq!(calls(&e, VECTOR_EXPAND).len(), 1);
    }

    #[test]
    fn at_0062b1e0_answers_the_setting_of_each_material() {
        let mut e = engine();
        e.register(SETTING_VALUE, |_, a| a[0].into_ret());
        e.map(0x011c_f000, 0x3000);
        e.set_global(0x011c_ffacu32, 2.5f32);
        e.set_global(0x011d_06d0u32, 1.5f32);
        e.set_global(0x011d_1224u32, 0x7f80_0001u32);
        assert_eq!(e.call(0x0062_b1e0, &args![0x18u32]).f64(), 2.5);
        assert_eq!(e.call(0x0062_b1e0, &args![0u32]).f64(), 1.5);
        let quiet = e.call(0x0062_b1e0, &args![9u32]).f64();
        assert_eq!(x87_store32(quiet), 0x7fc0_0001);
        assert_eq!(e.call(0x0062_b1e0, &args![0xau32]).f64(), 0.0);
        assert_eq!(e.call(0x0062_b1e0, &args![0x1au32]).f64(), 0.0);
        assert_eq!(calls(&e, SETTING_VALUE).len(), 3);
    }

    #[test]
    fn at_0062b480_and_0062b4b0_walk_the_container_keys() {
        let mut e = engine();
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, 0x0700_0008);
        e.mem.set_u32(vtable + 0xc, 0x0700_000c);
        e.register(0x0700_0008, |_, _| 5u32.into_ret());
        e.register(0x0700_000c, |_, a| (a[1] + 1).into_ret());
        let held = e.mem.alloc(0x20);
        e.mem.set_u32(held + 0x10, vtable);
        e.register_double(HELD_OBJECT, move |_, _| held.into_ret());
        assert_eq!(e.call(0x0062_b480, &args![0x4000u32]).u32(), 5);
        assert_eq!(e.call(0x0062_b4b0, &args![0x4000u32, 7u32]).u32(), 8);
        assert_eq!(calls(&e, 0x0700_000c), vec![vec![held + 0x10, 7]]);
    }

    #[test]
    fn at_0062b150_takes_the_reference_route() {
        let mut e = engine();
        e.register(OBJECT_OF, |_, _| 0x5000u32.into_ret());
        e.register(REFERENCE_OF, |_, _| 0x6000u32.into_ret());
        e.register(LOW_BITS, |_, _| 7u32.into_ret());
        e.register(REFERENCE_LOOKUP, |_, _| 0x77u32.into_ret());
        assert_eq!(e.call(0x0062_b150, &args![0x4000u32, 0x9u32]).u32(), 0x77);
        assert_eq!(calls(&e, REFERENCE_LOOKUP), vec![vec![7, 0x23]]);
        assert_eq!(calls(&e, LOW_BITS), vec![vec![0x6000]]);
    }

    #[test]
    fn at_0062b150_without_a_reference_picks_by_layer() {
        let mut e = engine();
        e.map(0x011d_e000, 0x1000);
        e.set_global(LAYER_17_OWNER, 0x8800u32);
        e.register(OBJECT_OF, |_, _| 0x5000u32.into_ret());
        e.register(REFERENCE_OF, |_, _| 0u32.into_ret());
        e.register(FILTER_HOLDER, |_, _| 0x5100u32.into_ret());
        e.register(FILTER_WORD, |_, _| 0x11u32.into_ret());
        e.register(RECORD_FROM_WORD, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(LAYER_OF, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LAYER_17_LOOKUP, |_, _| 0x17u32.into_ret());
        e.register(HAVOK_LOOKUP, |_, _| 0x99u32.into_ret());
        assert_eq!(e.call(0x0062_b150, &args![0x4000u32, 0x9u32]).u32(), 0x17);
        assert_eq!(calls(&e, LAYER_17_LOOKUP), vec![vec![0x8800, 0x9]]);
        e.register(FILTER_WORD, |_, _| 0x12u32.into_ret());
        assert_eq!(e.call(0x0062_b150, &args![0x4000u32, 0x9u32]).u32(), 0x99);
        assert_eq!(calls(&e, HAVOK_LOOKUP), vec![vec![0x4000]]);
    }

    /// A Havok object at `0x4000` whose shape answers keys 0, 1, 2 with materials 0, 1, 0x18,
    /// settings 1.0, 2.0, 6.0; the factor settings are 0.5 and 0.25.
    fn mean_doubles(e: &mut Engine, shape: bool, test: u32, kind: u32) {
        e.map(0x011c_f000, 0x3000);
        e.set_global(0x011d_06d0u32, 1.0f32);
        e.set_global(0x011d_0e40u32, 2.0f32);
        e.set_global(0x011c_ffacu32, 6.0f32);
        e.set_global(FIRST_FACTOR, 0.5f32);
        e.set_global(SECOND_FACTOR, 0.25f32);
        e.register(SETTING_VALUE, |_, a| a[0].into_ret());
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + SHAPE_SLOT, 0x0700_00cc);
        e.mem.set_u32(vtable + MATERIAL_SLOT, 0x0700_00e0);
        e.mem.set_u32(vtable + 8, 0x0700_0008);
        e.mem.set_u32(vtable + 0xc, 0x0700_000c);
        e.map(0x4000, 0x100);
        e.mem.set_u32(0x4000, vtable);
        e.mem.set_u32(0x4010, vtable);
        let shape_word = if shape { 0x4000u32 } else { 0 };
        e.register(HAVOK_OBJECT, |_, _| 0x4000u32.into_ret());
        e.register_double(0x0700_00cc, move |_, _| shape_word.into_ret());
        e.register(0x0700_00e0, |_, a| {
            [0u32, 1, 0x18][a[1] as usize].into_ret()
        });
        e.register(HELD_OBJECT, |_, _| 0x4000u32.into_ret());
        e.register(0x0700_0008, |_, _| 0u32.into_ret());
        e.register(0x0700_000c, |_, a| {
            if a[1] == 2 { u32::MAX } else { a[1] + 1 }.into_ret()
        });
        e.register(LOW_BITS, |_, _| 1u32.into_ret());
        e.register(FILTER_RECORD, |_, _| Ret::default());
        e.register_double(RECORD_TEST, move |_, _| test.into_ret());
        e.register_double(RECORD_KIND, move |_, _| kind.into_ret());
    }

    #[test]
    fn at_0062b310_averages_the_shape_materials() {
        let mut e = engine();
        mean_doubles(&mut e, true, 0, 0);
        assert_eq!(e.call(0x0062_b310, &args![0x9000u32]).f64(), 3.0);
        assert_eq!(calls(&e, FILTER_RECORD)[0][0], 0x9000);
        assert!(calls(&e, RECORD_KIND).is_empty());
    }

    #[test]
    fn at_0062b310_without_a_shape_takes_the_low_bits_material() {
        let mut e = engine();
        mean_doubles(&mut e, false, 0, 0);
        assert_eq!(e.call(0x0062_b310, &args![0x9000u32]).f64(), 2.0);
        assert_eq!(calls(&e, LOW_BITS), vec![vec![0x4000]]);
    }

    #[test]
    fn at_0062b310_scales_by_the_kind() {
        for (kind, expected) in [
            (1u32, 1.5f64),
            (8, 1.5),
            (6, 0.75),
            (22, 0.75),
            (17, 0.0),
            (21, 0.0),
            (0, 3.0),
            (23, 3.0),
        ] {
            let mut e = engine();
            mean_doubles(&mut e, true, 1, kind);
            assert_eq!(
                e.call(0x0062_b310, &args![0x9000u32]).f64(),
                expected,
                "kind {kind}"
            );
        }
    }

    #[test]
    fn at_0062b310_takes_the_first_factor_for_a_mean_not_above_zero() {
        let mut e = engine();
        mean_doubles(&mut e, false, 1, 6);
        e.register(LOW_BITS, |_, _| 0xau32.into_ret());
        assert_eq!(e.call(0x0062_b310, &args![0x9000u32]).f64(), 0.0);
        assert!(calls(&e, RECORD_KIND).is_empty());
        assert_eq!(calls(&e, SETTING_VALUE), vec![vec![FIRST_FACTOR]]);
    }

    fn removal_doubles(e: &mut Engine, record: u32) {
        e.register(GET_COLLISION_OBJECT, |_, _| 0x5000u32.into_ret());
        e.register(RIGID_BODY_OF, |_, _| 0x6000u32.into_ret());
        e.map(0x6000, 0x100);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + BODY_TRANSFORM_SLOT, 0x0700_00f4);
        e.mem.set_u32(vtable + BODY_CENTER_SLOT, 0x0700_00ec);
        e.mem.set_u32(0x6000, vtable);
        for addr in [
            0x0700_00f4,
            0x0700_00ec,
            TRANSFORM_CONSTRUCT,
            TRANSFORM_FROM,
            TRANSFORM_SET_INVERSE,
            TRANSFORM_SET_MUL,
            VECTOR_SPLAT,
            VECTOR_MUL,
            VECTOR_SUB,
            SET_CENTER_OF_MASS_LOCAL,
            VECTOR_EXPAND,
            VECTOR_APPLY,
            BODY_MOVE,
            OPERATOR_DELETE,
            LIST_REMOVE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(TRANSFORM_SOURCE, |_, _| 0x0126_83b0u32.into_ret());
        e.register(TRANSFORM_TRANSLATION, |_, _| 0x7700u32.into_ret());
        e.map(0x7700, 0x10);
        e.register(HELD_OBJECT, |_, _| 0x6600u32.into_ret());
        e.register(BODY_WORD, |_, _| 0x6700u32.into_ret());
        let weight = e.mem.alloc(4);
        e.mem.set_u32(weight, 2.0f32.to_bits());
        e.register_double(SETTING_VALUE, move |_, _| weight.into_ret());
        // A list of one node holding `record`.
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, record);
        e.register(NODE_IS_END, |_, a| (a[0] != RECORD_LIST).into_ret());
        e.register(NODE_NEXT, |_, _| 0x400u32.into_ret());
        e.register_double(HELD_ITEM, move |_, a| {
            if a[0] == RECORD_LIST { cell } else { a[0] }.into_ret()
        });
    }

    #[test]
    fn at_0062ab60_does_nothing_for_an_unknown_object() {
        let mut e = engine();
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record + 4, 0x1111);
        removal_doubles(&mut e, record);
        e.call(0x0062_ab60, &args![0x1234u32, 0x8800u32]);
        assert!(calls(&e, GET_COLLISION_OBJECT).is_empty());
    }

    #[test]
    fn at_0062ab60_moves_the_center_back_and_drops_the_last_arrow() {
        let mut e = engine();
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record, 0x6000);
        e.mem.set_u32(record + 4, 0x8800);
        e.mem.set_u32(record + 8, 10.0f32.to_bits());
        e.mem.set_u32(record + 0xc, 1);
        removal_doubles(&mut e, record);
        e.call(0x0062_ab60, &args![0x1234u32, 0x8800u32]);
        assert_eq!(calls(&e, GET_COLLISION_OBJECT), vec![vec![0x8800]]);
        assert_eq!(calls(&e, TRANSFORM_FROM)[0][1..], [0x0126_83b0, 0x1234]);
        let splats: Vec<u32> = calls(&e, VECTOR_SPLAT).iter().map(|a| a[1]).collect();
        assert_eq!(
            splats,
            vec![
                2.0f32.to_bits(),
                10.0f32.to_bits(),
                0.125f32.to_bits(),
                1.5f32.to_bits()
            ]
        );
        assert_eq!(calls(&e, SET_CENTER_OF_MASS_LOCAL)[0][0], 0x6600);
        let offset = calls(&e, BODY_MOVE)[0][1];
        assert_eq!(calls(&e, BODY_MOVE), vec![vec![0x6000, offset]]);
        assert_eq!(calls(&e, VECTOR_APPLY), vec![vec![offset, 0x6700]]);
        assert_eq!(f32::from_bits(e.mem.u32(record + 8)), 8.0);
        assert_eq!(e.mem.u32(record + 0xc), 0);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![record]]);
        assert_eq!(calls(&e, LIST_REMOVE).len(), 1);
        assert_eq!(calls(&e, LIST_REMOVE)[0][0], RECORD_LIST);
    }

    #[test]
    fn at_0062ab60_keeps_a_record_with_arrows_left() {
        let mut e = engine();
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record + 4, 0x8800);
        e.mem.set_u32(record + 8, 10.0f32.to_bits());
        e.mem.set_u32(record + 0xc, 3);
        removal_doubles(&mut e, record);
        e.call(0x0062_ab60, &args![0x1234u32, 0x8800u32]);
        assert_eq!(e.mem.u32(record + 0xc), 2);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert!(calls(&e, LIST_REMOVE).is_empty());
    }
}
