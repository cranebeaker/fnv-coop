//! `Havok SDK/hkplistagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00d34810` (cdecl, five stack words, no result): the body `00d36930` jumps to. The engine
/// map files it under `hkpconvexlistagent.cpp`.
const LINEAR_CAST_BODY: u32 = 0x00d3_4810;

// Translated from 00d36930 (decompiled, FalloutNV.exe 1.4.0.525)
/// A bare `JMP 00d34810`: the five stack words go to that function unchanged (cdecl, nothing
/// is returned).
pub fn fn_00d36930(e: &mut Engine, a: u32, b: u32, c: u32, d: u32, f: u32) {
    e.call(LINEAR_CAST_BODY, &args![a, b, c, d, f]);
}

// Translated from 00d36960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListAgent::linearCast` (Xbox PDB): a virtual method that does not use `this` and passes
/// its five stack words on to `fn_00d36930`.
pub fn hkp_list_agent_linear_cast(
    e: &mut Engine,
    _this: Ptr,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    f: u32,
) {
    fn_00d36930(e, a, b, c, d, f);
}

/// `00da42b0` (cdecl, `(array, word, word, owner)`): the cleanup of the agent's entries.
const CLEANUP_ENTRIES: u32 = 0x00da_42b0;
/// `00da3510` (cdecl, `(array, word)`).
const INVALIDATE_ENTRIES: u32 = 0x00da_3510;
/// `00da35f0` (cdecl, `(array, float, float, word)`).
const WARP_ENTRIES: u32 = 0x00da_35f0;
/// `00d97250` (cdecl, `(array, word, statistics collector)`).
const CALC_STATISTICS: u32 = 0x00d9_7250;
/// `00da48e0` (cdecl, `(array, record)`): walks the entries with a filter callback.
const UPDATE_FILTER_ENTRIES: u32 = 0x00da_48e0;
/// `hkTransform::setMulInverseMul` (`00cb22f0`, thiscall, `RET 8`): `this = inverse(a) * b`.
const SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// `00da4aa0` (cdecl, five words): processes the entries of an agent array.
const PROCESS_ENTRIES: u32 = 0x00da_4aa0;
/// `00da4430` (cdecl, `(array)`): the array setup the constructor ends with.
const SETUP_ENTRIES: u32 = 0x00da_4430;
/// `00d36920` and `00d36990` (cdecl, four words each): the bodies of `getClosestPoints` and
/// `getPenetrations`.
const CLOSEST_POINTS_BODY: u32 = 0x00d3_6920;
const PENETRATIONS_BODY: u32 = 0x00d3_6990;
/// `00cfbbe0` (thiscall, `RET 0xC`, `(record, type, type)`): registers an agent record with
/// the collision dispatcher for a pair of shape types.
const REGISTER_AGENT: u32 = 0x00cf_bbe0;
/// The thread's memory router (`00c85750`): word 0 is the cursor of its stack allocator,
/// word +0x10 the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `float` the flipping wrappers store beside the original collector (`0104da48`).
const WRAPPER_FLOAT: u32 = 0x0104_da48;
/// Sign mask of four lanes that flips a contact normal (`010c4c00`).
const NORMAL_SIGN_MASK: u32 = 0x010c_4c00;
/// Sign mask of four lanes (`010c44d0`).
const SIGN_MASK: u32 = 0x010c_44d0;
/// The vtable `0102e388` the destructor leaves the object with.
const BASE_VTABLE: u32 = 0x0102_e388;
/// Vtables of the wrappers the symmetric functions build on the stack (unconfirmed classes).
const CLOSEST_POINTS_WRAPPER_VTABLE: u32 = 0x010c_e7fc;
const PENETRATIONS_WRAPPER_VTABLE: u32 = 0x010c_e81c;
const LINEAR_CAST_WRAPPER_VTABLE: u32 = 0x010c_e80c;
/// The agent's own vtable written by the constructor.
const LIST_AGENT_VTABLE: u32 = 0x010c_ed34;

type Lanes = [f32; 4];

fn load(e: &Engine, addr: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(addr.wrapping_add(4 * lane as u32)))
}

fn store(e: &mut Engine, addr: u32, v: Lanes) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(addr.wrapping_add(4 * lane as u32), *value);
    }
}

fn sub(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] - b[lane])
}

fn mul(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] * b[lane])
}

fn add(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] + b[lane])
}

fn splat(value: f32) -> Lanes {
    [value; 4]
}

/// `XORPS` of the four words at `from` with the four words at `mask`, written to `to`.
fn xor_mask(e: &mut Engine, from: u32, mask: u32, to: u32) {
    let bits: [u32; 4] = std::array::from_fn(|lane| {
        let offset = 4 * lane as u32;
        e.mem.u32(from.wrapping_add(offset)) ^ e.mem.u32(mask.wrapping_add(offset))
    });
    for (lane, value) in bits.iter().enumerate() {
        e.mem.set_u32(to.wrapping_add(4 * lane as u32), *value);
    }
}

// Translated from 00d36820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListAgent::cleanup` (Xbox PDB): cleans up the entries of the agent array at +0x10
/// (`00da42b0` with the words at +0x0C and +0x08 and the owner) and then deletes the agent
/// itself through vtable slot 0 with flag 1.
pub fn hkp_list_agent_cleanup(e: &mut Engine, this: Ptr, owner: u32) {
    let second = e.mem.u32(this.addr() + 8);
    let first = e.mem.u32(this.addr() + 0xc);
    e.call(
        CLEANUP_ENTRIES,
        &args![this.addr() + 0x10, first, second, owner],
    );
    e.vcall(this.addr(), 0, &args![1u32]);
}

// Translated from 00d36850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListAgent::invalidateTim` (Xbox PDB): hands its word to `00da3510` with the agent
/// array at +0x10.
pub fn hkp_list_agent_invalidate_tim(e: &mut Engine, this: Ptr, word: u32) {
    e.call(INVALIDATE_ENTRIES, &args![this.addr() + 0x10, word]);
}

// Translated from 00d36870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListAgent::warpTime` (Xbox PDB): passes two floats (each through the x87 stack) and a
/// word to `00da35f0` with the agent array at +0x10.
pub fn hkp_list_agent_warp_time(e: &mut Engine, this: Ptr, from: f32, to: f32, word: u32) {
    e.call(
        WARP_ENTRIES,
        &args![this.addr() + 0x10, x87_quiet(from), x87_quiet(to), word],
    );
}

// Translated from 00d368a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListAgent::calcContentStatistics` (Xbox PDB): `00d97250` with the agent array at
/// +0x10, the word at +8 of `collector`, and `collector`. The second stack word is never read.
pub fn hkp_list_agent_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    _unused_1: u32,
) {
    let word = e.mem.u32(collector.addr() + 8);
    e.call(CALC_STATISTICS, &args![this.addr() + 0x10, word, collector]);
}

// Translated from 00d368c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListAgent::updateShapeCollectionFilter` (Xbox PDB): builds a 0x20-byte record on the
/// stack, `{a, b, (unwritten), result of the vtable slot 0x10 of the object b points at, c,
/// word at this+8, d, (unwritten)}`, and walks the agent array at +0x10 with it
/// (`00da48e0`). The callee copies the record as a block; its filter callback `00da3f90`
/// reads only the words at +0, +4, +0xC and +0x10.
pub fn hkp_list_agent_update_shape_collection_filter(
    e: &mut Engine,
    this: Ptr,
    a: u32,
    b: Ptr,
    c: u32,
    d: u32,
) {
    e.with_stack(0x20, |e, record| {
        let at = record.addr();
        let word = e.mem.u32(this.addr() + 8);
        e.mem.set_u32(at + 0x10, c);
        e.mem.set_u32(at, a);
        e.mem.set_u32(at + 0x18, d);
        e.mem.set_u32(at + 4, b.addr());
        e.mem.set_u32(at + 0x14, word);
        let object = e.mem.u32(b.addr());
        let result = e.vcall(object, 0x10, &args![]).u32();
        e.mem.set_u32(at + 0xc, result);
        e.call(UPDATE_FILTER_ENTRIES, &args![this.addr() + 0x10, record]);
    });
}

// Translated from 00d36940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListAgent::getClosestPoints` (Xbox PDB): does not use `this`; passes its four stack
/// words unchanged to `00d36920`.
pub fn hkp_list_agent_get_closest_points(
    e: &mut Engine,
    _this: Ptr,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
) {
    e.call(CLOSEST_POINTS_BODY, &args![a, b, c, d]);
}

// Translated from 00d369a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListAgent::processCollision` (Xbox PDB). `a` is an object whose +8 is the first body;
/// `pair` points at `{manager, _, second body}`. Builds the process input on the stack:
/// `{a, pair, 0, input, the word at this+8}`, at +0x20 the transform
/// `setMulInverseMul(first body, second body)` and at +0x70 the vector
/// `(first body's +0x40 - +0x50) * f_first + (second body's +0x50 - +0x40) * f_second`
/// with `f = input's +0x58 times the body's +0x5C` (rounded to float) and, in lane 3, the
/// sum of `+0x9C * f * +0xA0` over both bodies. Then takes room for `manager+0x1C` words on
/// the router's stack allocator (rounded up to `0x80` bytes), lists in it the indices below
/// `manager+0x1C` that are set in the bit field at `manager+0x50` (all above index 255),
/// ends the list with `-1`, runs `00da4aa0(this+0x10, record, manager+0x10, list, output)`
/// and gives the room back. The stack words of the record the exe never writes (+0x14..+0x1F,
/// +0x60..+0x6F) are zero here.
pub fn hkp_list_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    a: u32,
    pair: Ptr,
    input: Ptr,
    output: u32,
) {
    let manager = e.mem.u32(pair.addr());
    let second = e.mem.u32(pair.addr() + 8);
    let this_word = e.mem.u32(this.addr() + 8);
    let factor = x87_quiet(e.mem.f32(input.addr() + 0x58));
    let first = e.mem.u32(a + 8);
    e.with_stack(0x80, |e, record| {
        let at = record.addr();
        e.mem.set_u32(at, a);
        e.mem.set_u32(at + 4, pair.addr());
        e.mem.set_u32(at + 8, 0);
        e.mem.set_u32(at + 0xc, input.addr());
        e.mem.set_u32(at + 0x10, this_word);
        let first_lower = load(e, first + 0x40);
        let first_upper = load(e, first + 0x50);
        let second_lower = load(e, second + 0x40);
        let second_upper = load(e, second + 0x50);
        let first_scale = ((factor as f64) * (e.mem.f32(first + 0x5c) as f64)) as f32;
        let second_scale = ((factor as f64) * (e.mem.f32(second + 0x5c) as f64)) as f32;
        let from_first = mul(sub(first_lower, first_upper), splat(first_scale));
        let from_second = mul(splat(second_scale), sub(second_upper, second_lower));
        let mut velocity = add(from_second, from_first);
        let second_term = ((e.mem.f32(second + 0x9c) as f64) * (second_scale as f64))
            * (e.mem.f32(second + 0xa0) as f64);
        let first_term = ((e.mem.f32(first + 0x9c) as f64) * (first_scale as f64))
            * (e.mem.f32(first + 0xa0) as f64);
        velocity[3] = (second_term + first_term) as f32;
        store(e, at + 0x70, velocity);
        e.call(SET_MUL_INVERSE_MUL, &args![at + 0x20, first, second]);
        let count = e.mem.u32(manager + 0x1c);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let list = e.mem.u32(router);
        let room = count.wrapping_mul(4).wrapping_add(0x83) & 0xffff_ff80;
        e.mem.set_u32(router, list.wrapping_add(room));
        let mut written = 0u32;
        let mut mask = 1u32;
        let mut index = 0u32;
        if (count as i32) > 0 {
            loop {
                let wanted =
                    index >= 0x100 || e.mem.u32(manager + 0x50 + (index >> 5) * 4) & mask != 0;
                if wanted {
                    e.mem.set_u32(list + written * 4, index);
                    written += 1;
                }
                mask = mask.rotate_left(1);
                index += 1;
                if (index as i32) >= (count as i32) {
                    break;
                }
            }
        }
        e.mem.set_u32(list + written * 4, 0xffff_ffff);
        e.call(
            PROCESS_ENTRIES,
            &args![this.addr() + 0x10, record, manager + 0x10, list, output],
        );
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, list);
    });
}

// Translated from 00d36b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListAgent::getPenetrations` (Xbox PDB): does not use `this`; passes its four stack
/// words unchanged to `00d36990`.
pub fn hkp_list_agent_get_penetrations(e: &mut Engine, _this: Ptr, a: u32, b: u32, c: u32, d: u32) {
    e.call(PENETRATIONS_BODY, &args![a, b, c, d]);
}

// Translated from 00d36b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor (no name in the engine map; four stack words, the first two never read):
/// the word at +6 is 1, +8 the fourth word, the vtable `010ced34`, the array at +0x10 empty
/// without owned storage, +0x0C the word `third` points at; then `00da4430` on the array.
/// Returns `this`.
pub fn fn_00d36b20(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    third: Ptr,
    fourth: u32,
) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, fourth);
    e.mem.set_u32(me, LIST_AGENT_VTABLE);
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me + 0x14, 0);
    e.mem.set_u32(me + 0x18, 0x8000_0000);
    let word = e.mem.u32(third.addr());
    e.mem.set_u32(me + 0xc, word);
    e.call(SETUP_ENTRIES, &args![me + 0x10]);
    this
}

// Translated from 00d36bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills a function record (cdecl, one pointer): four function addresses (`00d36b70`,
/// `00d34790`, `00d347d0`, `00d34810`) and two flag bytes, 0 and 1. The engine map has no name
/// for it.
pub fn fn_00d36bb0(e: &mut Engine, record: Ptr) {
    let at = record.addr();
    e.mem.set_u32(at, 0x00d3_6b70);
    e.mem.set_u32(at + 4, 0x00d3_4790);
    e.mem.set_u32(at + 8, 0x00d3_47d0);
    e.mem.set_u32(at + 0xc, 0x00d3_4810);
    e.mem.set_u8(at + 0x10, 0);
    e.mem.set_u8(at + 0x11, 1);
}

// Translated from 00d36c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills a function record (cdecl, one pointer): four function addresses (`00d36be0`,
/// `00d43dc0`, `00d43960`, `00d43b90`) and two flag bytes, both 1. The engine map has no name
/// for it.
pub fn fn_00d36c30(e: &mut Engine, record: Ptr) {
    let at = record.addr();
    e.mem.set_u32(at, 0x00d3_6be0);
    e.mem.set_u32(at + 4, 0x00d4_3dc0);
    e.mem.set_u32(at + 8, 0x00d4_3960);
    e.mem.set_u32(at + 0xc, 0x00d4_3b90);
    e.mem.set_u8(at + 0x10, 1);
    e.mem.set_u8(at + 0x11, 1);
}

// Translated from 00d36c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the list agent with the collision dispatcher (cdecl, one pointer; no name in the
/// engine map): with one 0x14-byte record on the stack, filled by `fn_00d36c30` (its two flag
/// bytes cleared first) and registered for the shape types `(9, -1)`, then refilled by
/// `fn_00d36bb0` (flag bytes cleared first) and registered for `(-1, 9)`; the dispatcher call
/// is `00cfbbe0(record, first type, second type)`.
pub fn fn_00d36c60(e: &mut Engine, dispatcher: Ptr) {
    e.with_stack(0x14, |e, record| {
        e.mem.set_u8(record.addr() + 0x10, 0);
        e.mem.set_u8(record.addr() + 0x11, 0);
        fn_00d36c30(e, record);
        e.call(
            REGISTER_AGENT,
            &args![dispatcher, record, 9u32, 0xffff_ffffu32],
        );
        e.mem.set_u8(record.addr() + 0x10, 0);
        e.mem.set_u8(record.addr() + 0x11, 0);
        fn_00d36bb0(e, record);
        e.call(
            REGISTER_AGENT,
            &args![dispatcher, record, 0xffff_ffffu32, 9u32],
        );
    });
}

// Translated from 00d36cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpListAgent>::updateShapeCollectionFilter` (Xbox PDB):
/// `hkpListAgent::updateShapeCollectionFilter` with its first two stack words swapped.
pub fn hkp_symmetric_agent_linear_cast_hkp_list_agent_update_shape_collection_filter(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) {
    hkp_list_agent_update_shape_collection_filter(e, this, second, Ptr::new(first), third, fourth);
}

// Translated from 00d36ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpListAgent>::getClosestPoints` (Xbox PDB): calls
/// `hkpListAgent::getClosestPoints` with its first two words swapped and, as the fourth, a
/// 12-byte wrapper on the stack `{vtable 010ce7fc, the float at 0104da48, the original
/// fourth word}`.
pub fn hkp_symmetric_agent_linear_cast_hkp_list_agent_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) {
    let value = x87_quiet(e.global::<f32>(WRAPPER_FLOAT));
    e.with_stack(0xc, |e, wrapper| {
        let at = wrapper.addr();
        e.mem.set_u32(at + 8, fourth);
        e.mem.set_f32(at + 4, value);
        e.mem.set_u32(at, CLOSEST_POINTS_WRAPPER_VTABLE);
        hkp_list_agent_get_closest_points(e, this, second, first, third, at);
    });
}

// Translated from 00d36d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpListAgent>::getPenetrations` (Xbox PDB): calls
/// `hkpListAgent::getPenetrations` with its first two words swapped and, as the fourth, a
/// 12-byte wrapper on the stack `{vtable 010ce81c, byte 0 at +4, the original fourth word at
/// +8}` (the three padding bytes after the flag are not written).
pub fn hkp_symmetric_agent_linear_cast_hkp_list_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) {
    e.with_stack(0xc, |e, wrapper| {
        let at = wrapper.addr();
        e.mem.set_u32(at + 8, fourth);
        e.mem.set_u8(at + 4, 0);
        e.mem.set_u32(at, PENETRATIONS_WRAPPER_VTABLE);
        hkp_list_agent_get_penetrations(e, this, second, first, third, at);
    });
}

// Translated from 00d36d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpListAgent>::processCollision` (Xbox PDB): runs
/// `hkpListAgent::processCollision` with its first two words swapped, then turns the contact
/// points it appended to `output` around (the word at `output` +0 is the write cursor, the
/// entries are 0x30 bytes, between the old and the new cursor): `position += normal *
/// normal.w` and the normal's sign bits flipped with the mask `010c4c00`. If the float at
/// `output+0x3030` changed (or is NaN) the result record is flipped too: the vector at
/// +0x3020 negated, the nibbles of the byte at +0x304A swapped, the bytes at +0x3048 and
/// +0x3049 exchanged, the word at +0x3040 exchanged with the word at index `b49 + b48 - 1`
/// (from +0x3040, before the exchange of the bytes) and, if `b48 & b49 == 2`, the words at
/// +0x3042 and +0x3044 exchanged.
pub fn hkp_symmetric_agent_linear_cast_hkp_list_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    output: Ptr,
) {
    let out = output.addr();
    let mut entry = e.mem.u32(out);
    let before = x87_quiet(e.mem.f32(out + 0x3030));
    hkp_list_agent_process_collision(e, this, second, Ptr::new(first), Ptr::new(third), out);
    if entry < e.mem.u32(out) {
        loop {
            let normal = load(e, entry + 0x10);
            let moved = add(mul(splat(normal[3]), normal), load(e, entry));
            store(e, entry, moved);
            xor_mask(e, entry + 0x10, NORMAL_SIGN_MASK, entry + 0x10);
            entry = entry.wrapping_add(0x30);
            if entry >= e.mem.u32(out) {
                break;
            }
        }
    }
    let after = x87_quiet(e.mem.f32(out + 0x3030));
    if after != before {
        xor_mask(e, out + 0x3020, NORMAL_SIGN_MASK, out + 0x3020);
        let nibbles = e.mem.u8(out + 0x304a);
        e.mem
            .set_u8(out + 0x304a, (nibbles >> 4) | ((nibbles & 0xf) << 4));
        let second_byte = e.mem.u8(out + 0x3049);
        let first_byte = e.mem.u8(out + 0x3048);
        e.mem.set_u8(out + 0x3049, first_byte);
        e.mem.set_u8(out + 0x3048, second_byte);
        let kept = e.mem.u16(out + 0x3040);
        let index = (second_byte as u32)
            .wrapping_add(first_byte as u32)
            .wrapping_sub(1);
        let other = out.wrapping_add(0x3040).wrapping_add(index.wrapping_mul(2));
        let moved = e.mem.u16(other);
        e.mem.set_u16(out + 0x3040, moved);
        e.mem.set_u16(other, kept);
        if e.mem.u8(out + 0x3048) & e.mem.u8(out + 0x3049) == 2 {
            let low = e.mem.u16(out + 0x3042);
            let high = e.mem.u16(out + 0x3044);
            e.mem.set_u16(out + 0x3042, high);
            e.mem.set_u16(out + 0x3044, low);
        }
    }
}

// Translated from 00d36e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgent<hkpListAgent>::linearCast` (Xbox PDB), five stack words: calls
/// `hkpListAgent::linearCast` with the first two words swapped and, in place of the third, a
/// copy of its 0x70-byte structure whose vector at +0x50 has its sign bits flipped
/// (`010c44d0`), as the fourth a 0x24-byte wrapper `{vtable 010ce80c, the float at 0104da48,
/// (8 bytes not written), the third's vector at +0x50, the fourth word}` and, when the fifth
/// is not 0, the same wrapper holding the fifth word (otherwise 0). The wrappers live on the
/// stack.
pub fn hkp_symmetric_agent_hkp_list_agent_linear_cast(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: Ptr,
    fourth: u32,
    fifth: u32,
) {
    let value = x87_quiet(e.global::<f32>(WRAPPER_FLOAT));
    e.with_stack(0xd8, |e, frame| {
        let at = frame.addr();
        let (wrapper, other, copy) = (at + 8, at + 0x38, at + 0x68);
        e.mem.set_f32(at + 0xc, value);
        for word in 0..0x1c {
            let source = e.mem.u32(third.addr() + 4 * word);
            e.mem.set_u32(copy + 4 * word, source);
        }
        for lane in 0..4 {
            let source = e.mem.u32(third.addr() + 0x50 + 4 * lane);
            e.mem.set_u32(at + 0x18 + 4 * lane, source);
        }
        e.mem.set_u32(at + 0x28, fourth);
        xor_mask(e, third.addr() + 0x50, SIGN_MASK, copy + 0x50);
        e.mem.set_u32(wrapper, LINEAR_CAST_WRAPPER_VTABLE);
        let extra = if fifth != 0 {
            e.mem.set_u32(other, LINEAR_CAST_WRAPPER_VTABLE);
            e.mem.set_f32(other + 4, value);
            for lane in 0..4 {
                let source = e.mem.u32(third.addr() + 0x50 + 4 * lane);
                e.mem.set_u32(other + 0x10 + 4 * lane, source);
            }
            e.mem.set_u32(other + 0x20, fifth);
            other
        } else {
            0
        };
        hkp_list_agent_linear_cast(e, this, second, first, copy, wrapper, extra);
    });
}

// Translated from 00d36f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListAgent::_scalar_deleting_destructor_` (Xbox PDB): empties the array at +0x10 (the
/// size at +0x14 cleared, its storage freed through the router's allocator slot `0x10` when
/// its capacity word's top bit is clear, 4 bytes per element), points the object at the base
/// vtable `0102e388` and, when bit 0 of `flags` is set, frees the object itself through
/// allocator slot 8 with the 16-bit size at +4. Returns `this`.
pub fn hkp_list_agent_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let me = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + 0x18);
    e.mem.set_u32(me + 0x14, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0x10);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 2]);
    }
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me + 0x18, 0x8000_0000);
    e.mem.set_u32(me, BASE_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(me + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// Allocates `size` bytes the way the agent create functions do: the thread's memory router
/// (`00c85750`) has the allocator at `+0x10`; its virtual slot `4` hands out the block, whose
/// size goes into the 16-bit word at `+4`.
fn allocate_agent_block(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block.wrapping_add(4), size as u16);
    block
}

// Translated from 00d36b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `body_a`, `body_b`, `input`, `contact_mgr`): a
/// create function of the list agent (code at `00d36bb4` stores its address in an agent
/// record). Allocates the 0x1c-byte agent (`allocate_agent_block`) and runs the constructor
/// `00d36b20` on it with the four words in this order. Answers what the constructor answers,
/// the agent.
pub fn fn_00d36b70(e: &mut Engine, body_a: u32, body_b: u32, input: Ptr, contact_mgr: u32) -> u32 {
    let agent = allocate_agent_block(e, 0x1c);
    fn_00d36b20(e, Ptr::new(agent), body_a, body_b, input, contact_mgr).addr()
}

// Translated from 00d36be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `body_a`, `body_b`, `input`, `contact_mgr`): the
/// create function of the flipped list agent (code at `00d36c36` stores its address in an agent
/// record). Allocates the 0x1c-byte agent (`allocate_agent_block`), runs the constructor
/// `00d36b20` with the first two words exchanged (`body_b`, `body_a`, `input`, `contact_mgr`) and
/// stores the vtable `010ced70`. Answers the agent.
pub fn fn_00d36be0(e: &mut Engine, body_a: u32, body_b: u32, input: Ptr, contact_mgr: u32) -> u32 {
    let agent = allocate_agent_block(e, 0x1c);
    fn_00d36b20(e, Ptr::new(agent), body_b, body_a, input, contact_mgr);
    e.mem.set_u32(agent, 0x010c_ed70);
    agent
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d36b70, fn_00d36b70(u32, u32, Ptr, u32) -> u32),
        entry!(0x00d36be0, fn_00d36be0(u32, u32, Ptr, u32) -> u32),
        entry!(0x00d36820, hkp_list_agent_cleanup(Ptr, u32)),
        entry!(0x00d36850, hkp_list_agent_invalidate_tim(Ptr, u32)),
        entry!(0x00d36870, hkp_list_agent_warp_time(Ptr, f32, f32, u32)),
        entry!(
            0x00d368a0,
            hkp_list_agent_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(
            0x00d368c0,
            hkp_list_agent_update_shape_collection_filter(Ptr, u32, Ptr, u32, u32)
        ),
        entry!(
            0x00d36940,
            hkp_list_agent_get_closest_points(Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00d369a0,
            hkp_list_agent_process_collision(Ptr, u32, Ptr, Ptr, u32)
        ),
        entry!(
            0x00d36b00,
            hkp_list_agent_get_penetrations(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00d36b20, fn_00d36b20(Ptr, u32, u32, Ptr, u32) -> Ptr),
        entry!(0x00d36bb0, fn_00d36bb0(Ptr)),
        entry!(0x00d36c30, fn_00d36c30(Ptr)),
        entry!(0x00d36c60, fn_00d36c60(Ptr)),
        entry!(
            0x00d36cc0,
            hkp_symmetric_agent_linear_cast_hkp_list_agent_update_shape_collection_filter(
                Ptr, u32, u32, u32, u32
            )
        ),
        entry!(
            0x00d36ce0,
            hkp_symmetric_agent_linear_cast_hkp_list_agent_get_closest_points(
                Ptr, u32, u32, u32, u32
            )
        ),
        entry!(
            0x00d36d20,
            hkp_symmetric_agent_linear_cast_hkp_list_agent_get_penetrations(
                Ptr, u32, u32, u32, u32
            )
        ),
        entry!(
            0x00d36d60,
            hkp_symmetric_agent_linear_cast_hkp_list_agent_process_collision(
                Ptr, u32, u32, u32, Ptr
            )
        ),
        entry!(
            0x00d36e80,
            hkp_symmetric_agent_hkp_list_agent_linear_cast(Ptr, u32, u32, Ptr, u32, u32)
        ),
        entry!(
            0x00d36f50,
            hkp_list_agent_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
        entry!(0x00d36930, fn_00d36930(u32, u32, u32, u32, u32)),
        entry!(
            0x00d36960,
            hkp_list_agent_linear_cast(Ptr, u32, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The memory router (`00c85750`) answers an object whose `+0x10` is an allocator; the
    /// allocator's slot 4 hands out `block` and records the size it was asked for. The array
    /// set-up the constructor runs (`00da4430`) is a no-op double.
    fn install_allocator(e: &mut Engine, block: u32) -> std::rc::Rc<std::cell::RefCell<Vec<u32>>> {
        let sizes = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = std::rc::Rc::clone(&sizes);
        e.map(block, 0x100);
        e.put_vtable(0x0900_0000, &[0, 0x0900_0100]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x0900_0000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x0900_0100, move |_, a| {
            seen.borrow_mut().push(a[1]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(SETUP_ENTRIES, |_, _| Ret::default());
        sizes
    }

    #[test]
    fn fn_00d36b70_allocates_the_agent_and_runs_the_constructor() {
        let mut e = Engine::new();
        let sizes = install_allocator(&mut e, 0x0800_0000);
        let input = e.mem.alloc(0x20);
        e.mem.set_u32(input, 0x77);
        let agent = e
            .call(0x00d3_6b70, &args![1u32, 2u32, input, 0x88u32])
            .u32();
        assert_eq!(agent, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x1c]);
        assert_eq!(e.mem.u16(agent + 4), 0x1c);
        assert_eq!(e.mem.u32(agent), LIST_AGENT_VTABLE);
        assert_eq!(e.mem.u32(agent + 8), 0x88);
        assert_eq!(e.mem.u32(agent + 0xc), 0x77);
    }

    #[test]
    fn fn_00d36be0_makes_the_flipped_agent() {
        let mut e = Engine::new();
        let sizes = install_allocator(&mut e, 0x0800_0000);
        let input = e.mem.alloc(0x20);
        e.mem.set_u32(input, 0x77);
        let agent = e
            .call(0x00d3_6be0, &args![1u32, 2u32, input, 0x88u32])
            .u32();
        assert_eq!(agent, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x1c]);
        assert_eq!(e.mem.u32(agent), 0x010c_ed70);
        assert_eq!(e.mem.u32(agent + 8), 0x88);
        assert_eq!(e.mem.u32(agent + 0xc), 0x77);
    }

    fn engine() -> (Engine, Rc<RefCell<Vec<Vec<u32>>>>) {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(vec![]));
        let sink = seen.clone();
        e.register_double(LINEAR_CAST_BODY, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        (e, seen)
    }

    #[test]
    fn the_jump_passes_all_five_words_on() {
        let (mut e, seen) = engine();
        e.call(0x00d36930, &args![1u32, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(*seen.borrow(), vec![vec![1, 2, 3, 4, 5]]);
    }

    #[test]
    fn linear_cast_forwards_its_stack_words_and_drops_this() {
        let (mut e, seen) = engine();
        e.call(0x00d36960, &args![0x99u32, 1u32, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(*seen.borrow(), vec![vec![1, 2, 3, 4, 5]]);
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    const SLOT_DOUBLE: u32 = 0x0900_0011;
    const DELETE_DOUBLE: u32 = 0x0900_0012;

    fn record(e: &mut Engine, addr: u32) -> Calls {
        let calls: Calls = Rc::default();
        let sink = Rc::clone(&calls);
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        calls
    }

    fn words(e: &Engine, addr: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(addr + 4 * i)).collect()
    }

    /// An object whose vtable has `target` in the slot at offset `slot`.
    fn object_with_slot(e: &mut Engine, slot: u32, target: u32) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + slot, target);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    fn router_with_cursor(e: &mut Engine, cursor: u32) -> u32 {
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, cursor);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        router
    }

    #[test]
    fn cleanup_passes_the_array_and_deletes_itself() {
        let mut e = Engine::new();
        let cleanup = record(&mut e, CLEANUP_ENTRIES);
        let delete = record(&mut e, SLOT_DOUBLE);
        let this = object_with_slot(&mut e, 0, SLOT_DOUBLE);
        e.mem.set_u32(this + 8, 0x22);
        e.mem.set_u32(this + 0xc, 0x11);
        e.call(0x00d36820, &args![this, 0x99u32]);
        assert_eq!(*cleanup.borrow(), vec![vec![this + 0x10, 0x11, 0x22, 0x99]]);
        assert_eq!(*delete.borrow(), vec![vec![this, 1]]);
    }

    #[test]
    fn invalidate_tim_and_calc_content_statistics_pass_the_array() {
        let mut e = Engine::new();
        let invalidate = record(&mut e, INVALIDATE_ENTRIES);
        let statistics = record(&mut e, CALC_STATISTICS);
        e.call(0x00d36850, &args![0x1000u32, 7u32]);
        assert_eq!(*invalidate.borrow(), vec![vec![0x1010, 7]]);
        let collector = e.mem.alloc(0x10);
        e.mem.set_u32(collector + 8, 0x44);
        e.call(0x00d368a0, &args![0x2000u32, collector, 0x77u32]);
        assert_eq!(*statistics.borrow(), vec![vec![0x2010, 0x44, collector]]);
    }

    #[test]
    fn warp_time_quiets_both_floats_and_keeps_the_word() {
        let mut e = Engine::new();
        let warp = record(&mut e, WARP_ENTRIES);
        e.call(
            0x00d36870,
            &args![0x3000u32, 0x7fa0_0000u32, 0x3f80_0000u32, 9u32],
        );
        assert_eq!(
            *warp.borrow(),
            vec![vec![0x3010, 0x7fe0_0000, 0x3f80_0000, 9]]
        );
    }

    #[test]
    fn update_shape_collection_filter_builds_the_record_for_the_array_walk() {
        let mut e = Engine::new();
        let seen: Calls = Rc::default();
        let sink = Rc::clone(&seen);
        e.register_double(UPDATE_FILTER_ENTRIES, move |e, a| {
            let mut row = vec![a[0]];
            row.extend(words(e, a[1], 8));
            sink.borrow_mut().push(row);
            Ret::default()
        });
        e.register_double(SLOT_DOUBLE, |_, _| Ret {
            eax: 0x777,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0x55);
        let object = object_with_slot(&mut e, 0x10, SLOT_DOUBLE);
        let b = e.mem.alloc(8);
        e.mem.set_u32(b, object);
        e.call(0x00d368c0, &args![this, 1u32, b, 3u32, 4u32]);
        let row = seen.borrow()[0].clone();
        assert_eq!(row[0], this + 0x10);
        // Words 2 and 8 of the record are never written by the exe.
        assert_eq!([row[1], row[2], row[4], row[5]], [1, b, 0x777, 3]);
        assert_eq!([row[6], row[7]], [0x55, 4]);
        // The swapped caller passes its words the other way around.
        seen.borrow_mut().clear();
        e.call(0x00d36cc0, &args![this, b, 8u32, 3u32, 4u32]);
        let row = seen.borrow()[0].clone();
        assert_eq!([row[1], row[2]], [8, b]);
    }

    #[test]
    fn closest_points_and_penetrations_forward_their_four_words() {
        let mut e = Engine::new();
        let closest = record(&mut e, CLOSEST_POINTS_BODY);
        let penetrations = record(&mut e, PENETRATIONS_BODY);
        e.call(0x00d36940, &args![0x99u32, 1u32, 2u32, 3u32, 4u32]);
        e.call(0x00d36b00, &args![0x99u32, 5u32, 6u32, 7u32, 8u32]);
        assert_eq!(*closest.borrow(), vec![vec![1, 2, 3, 4]]);
        assert_eq!(*penetrations.borrow(), vec![vec![5, 6, 7, 8]]);
    }

    struct Collision {
        e: Engine,
        list: u32,
        router: u32,
        this: u32,
        a: u32,
        pair: u32,
        input: u32,
        manager: u32,
        first: u32,
        second: u32,
    }

    fn collision(count: u32) -> Collision {
        let mut e = Engine::new();
        let list = e.mem.alloc(0x600);
        let router = router_with_cursor(&mut e, list);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0x55);
        let first = e.mem.alloc(0xb0);
        let second = e.mem.alloc(0xb0);
        store(&mut e, first + 0x40, [1.0, 2.0, 3.0, 4.0]);
        store(&mut e, first + 0x50, [0.5, 1.0, 1.5, 0.25]);
        store(&mut e, second + 0x40, [1.0; 4]);
        store(&mut e, second + 0x50, [2.0, 4.0, 6.0, 0.5]);
        e.mem.set_f32(first + 0x9c, 2.0);
        e.mem.set_f32(first + 0xa0, 6.0);
        e.mem.set_f32(second + 0x9c, 3.0);
        e.mem.set_f32(second + 0xa0, 4.0);
        let a = e.mem.alloc(0x10);
        e.mem.set_u32(a + 8, first);
        let manager = e.mem.alloc(0x100);
        e.mem.set_u32(manager + 0x1c, count);
        let pair = e.mem.alloc(0x10);
        e.mem.set_u32(pair, manager);
        e.mem.set_u32(pair + 8, second);
        let input = e.mem.alloc(0x60);
        e.mem.set_f32(input + 0x58, 2.0);
        Collision {
            e,
            list,
            router,
            this,
            a,
            pair,
            input,
            manager,
            first,
            second,
        }
    }

    /// Records the arguments of `00da4aa0`, the 32 words of its record, the router's cursor and
    /// the first three words of the list while it runs.
    fn capture_process(c: &mut Collision) -> Calls {
        let seen: Calls = Rc::default();
        let sink = Rc::clone(&seen);
        let router = c.router;
        c.e.register_double(PROCESS_ENTRIES, move |e, a| {
            let mut row = a.to_vec();
            row.extend(words(e, a[1], 0x20));
            row.push(e.mem.u32(router));
            row.extend(words(e, a[3], 3));
            sink.borrow_mut().push(row);
            Ret::default()
        });
        seen
    }

    #[test]
    fn process_collision_builds_the_record_and_lists_the_selected_indices() {
        let mut c = collision(3);
        c.e.mem.set_u32(c.manager + 0x50, 0b101);
        let multiplied = record(&mut c.e, SET_MUL_INVERSE_MUL);
        let seen = capture_process(&mut c);
        c.e.call(0x00d369a0, &args![c.this, c.a, c.pair, c.input, 0x123u32]);
        let rows = seen.borrow();
        assert_eq!(rows.len(), 1);
        let row = &rows[0];
        assert_eq!(multiplied.borrow()[0][1..], [c.first, c.second]);
        assert_eq!(row[0], c.this + 0x10);
        assert_eq!(row[2], c.manager + 0x10);
        assert_eq!(row[3], c.list);
        assert_eq!(row[4], 0x123);
        assert_eq!(multiplied.borrow()[0][0], row[1] + 0x20);
        // The record: a, pair, 0, input, this+8.
        assert_eq!(row[5..10], [c.a, c.pair, 0, c.input, 0x55]);
        // Vector at +0x70: second scale 1.0 * (hi - lo) + (lo - hi) * first scale 0.5; lane 3
        // is the sum of the +0x9C * scale * +0xA0 terms (12 + 6).
        let vector: Vec<f32> = row[5 + 28..5 + 32]
            .iter()
            .map(|b| f32::from_bits(*b))
            .collect();
        assert_eq!(vector, vec![1.25, 3.5, 5.75, 18.0]);
        // The router gave out 0x80 bytes and the list holds indices 0 and 2, then -1.
        assert_eq!(row[37], c.list + 0x80);
        assert_eq!(row[38..41], [0, 2, 0xffff_ffff]);
        assert_eq!(c.e.mem.u32(c.router), c.list);
    }

    #[test]
    fn process_collision_always_lists_the_indices_from_0x100() {
        let mut c = collision(0x101);
        c.e.mem.set_u32(c.manager + 0x50 + 7 * 4, 0x8000_0000);
        let seen = capture_process(&mut c);
        c.e.call(0x00d369a0, &args![c.this, c.a, c.pair, c.input, 0u32]);
        let rows = seen.borrow();
        assert_eq!(rows[0][37], c.list + 0x480);
        assert_eq!(rows[0][38..41], [0xff, 0x100, 0xffff_ffff]);
        assert_eq!(c.e.mem.u32(c.router), c.list);
    }

    #[test]
    fn symmetric_process_collision_turns_the_new_contacts_around() {
        let mut c = collision(0);
        c.e.map(0x0100_0000, 0x0030_0000);
        for (lane, bits) in [0x8000_0000u32, 0x8000_0000, 0x8000_0000, 0]
            .iter()
            .enumerate()
        {
            c.e.mem.set_u32(NORMAL_SIGN_MASK + 4 * lane as u32, *bits);
        }
        let out = c.e.mem.alloc(0x3100);
        let new_float = Rc::new(Cell::new(2.0f32));
        let value = Rc::clone(&new_float);
        c.e.register_double(PROCESS_ENTRIES, move |e, a| {
            let cursor = e.mem.u32(a[4]);
            store(e, cursor, [1.0, 2.0, 3.0, 0.0]);
            store(e, cursor + 0x10, [0.0, 0.0, 1.0, 2.0]);
            store(e, cursor + 0x30, [5.0, 5.0, 5.0, 1.0]);
            store(e, cursor + 0x40, [1.0, 0.0, 0.0, 0.5]);
            e.mem.set_u32(a[4], cursor + 0x60);
            e.mem.set_f32(a[4] + 0x3030, value.get());
            Ret::default()
        });
        let prepare = |c: &mut Collision, b48: u8, b49: u8| {
            c.e.mem.set_u32(out, out + 0x100);
            c.e.mem.set_f32(out + 0x3030, 1.0);
            store(&mut c.e, out + 0x3020, [1.0, 2.0, 3.0, 4.0]);
            for (i, v) in [0x1111u16, 0x2222, 0x3333, 0x4444].iter().enumerate() {
                c.e.mem.set_u16(out + 0x3040 + 2 * i as u32, *v);
            }
            c.e.mem.set_u8(out + 0x3048, b48);
            c.e.mem.set_u8(out + 0x3049, b49);
            c.e.mem.set_u8(out + 0x304a, 0x12);
        };
        let run = |c: &mut Collision| {
            c.e.call(0x00d36d60, &args![c.this, c.pair, c.a, c.input, out]);
        };
        // The new contacts are moved along their normals, whatever the float does.
        prepare(&mut c, 2, 2);
        new_float.set(1.0);
        run(&mut c);
        let entry = out + 0x100;
        let bits = |c: &Collision, at: u32| words(&c.e, at, 4);
        assert_eq!(
            bits(&c, entry),
            [1.0f32, 2.0, 5.0, 4.0].map(f32::to_bits).to_vec()
        );
        assert_eq!(
            bits(&c, entry + 0x10),
            [0x8000_0000, 0x8000_0000, 0xbf80_0000, 0x4000_0000]
        );
        assert_eq!(
            bits(&c, entry + 0x30),
            [5.5f32, 5.0, 5.0, 1.25].map(f32::to_bits).to_vec()
        );
        assert_eq!(
            bits(&c, entry + 0x40),
            [0xbf80_0000, 0x8000_0000, 0x8000_0000, 0x3f00_0000]
        );
        // The float did not change: the result record stays as it was.
        assert_eq!(
            bits(&c, out + 0x3020),
            [1.0f32, 2.0, 3.0, 4.0].map(f32::to_bits).to_vec()
        );
        assert_eq!(c.e.mem.u8(out + 0x304a), 0x12);
        assert_eq!(c.e.mem.u16(out + 0x3040), 0x1111);
        // The float changed: the record is flipped; b48 = b49 = 2 also exchanges words 1 and 2.
        prepare(&mut c, 2, 2);
        new_float.set(2.0);
        run(&mut c);
        assert_eq!(
            bits(&c, out + 0x3020),
            [-1.0f32, -2.0, -3.0, 4.0].map(f32::to_bits).to_vec()
        );
        assert_eq!(c.e.mem.u8(out + 0x304a), 0x21);
        let halves: Vec<u16> = (0..4).map(|i| c.e.mem.u16(out + 0x3040 + 2 * i)).collect();
        assert_eq!(halves, vec![0x4444, 0x3333, 0x2222, 0x1111]);
        // b48 = 1, b49 = 2: the bytes are exchanged and word 0 trades places with word 2.
        prepare(&mut c, 1, 2);
        new_float.set(f32::NAN);
        run(&mut c);
        assert_eq!([c.e.mem.u8(out + 0x3048), c.e.mem.u8(out + 0x3049)], [2, 1]);
        let halves: Vec<u16> = (0..4).map(|i| c.e.mem.u16(out + 0x3040 + 2 * i)).collect();
        assert_eq!(halves, vec![0x3333, 0x2222, 0x1111, 0x4444]);
    }

    #[test]
    fn the_constructor_empties_the_array_and_sets_it_up() {
        let mut e = Engine::new();
        let setup = record(&mut e, SETUP_ENTRIES);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0x5555);
        e.mem.set_u32(this + 0x14, 0x5555);
        let third = e.mem.alloc(8);
        e.mem.set_u32(third, 0x1234);
        let result = e.call(0x00d36b20, &args![this, 0u32, 0u32, third, 0x66u32]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0x66);
        assert_eq!(e.mem.u32(this), LIST_AGENT_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0x1234);
        assert_eq!(words(&e, this + 0x10, 3), [0, 0, 0x8000_0000]);
        assert_eq!(*setup.borrow(), vec![vec![this + 0x10]]);
    }

    #[test]
    fn the_registration_fills_two_records_and_registers_both() {
        let mut e = Engine::new();
        let seen: Calls = Rc::default();
        let sink = Rc::clone(&seen);
        e.register_double(REGISTER_AGENT, move |e, a| {
            let mut row = a.to_vec();
            row.extend(words(e, a[1], 4));
            row.push(u32::from(e.mem.u8(a[1] + 0x10)));
            row.push(u32::from(e.mem.u8(a[1] + 0x11)));
            sink.borrow_mut().push(row);
            Ret::default()
        });
        e.call(0x00d36c60, &args![0x4000u32]);
        let rows = seen.borrow();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0][0], 0x4000);
        assert_eq!(rows[0][1], rows[1][1]);
        assert_eq!(
            rows[0][2..],
            [
                9,
                0xffff_ffff,
                0x00d3_6be0,
                0x00d4_3dc0,
                0x00d4_3960,
                0x00d4_3b90,
                1,
                1
            ]
        );
        assert_eq!(
            rows[1][2..],
            [
                0xffff_ffff,
                9,
                0x00d3_6b70,
                0x00d3_4790,
                0x00d3_47d0,
                0x00d3_4810,
                0,
                1
            ]
        );
        // The two fillers on their own.
        let record_at = e.mem.alloc(0x20);
        e.call(0x00d36bb0, &args![record_at]);
        assert_eq!(
            words(&e, record_at, 4),
            [0x00d3_6b70, 0x00d3_4790, 0x00d3_47d0, 0x00d3_4810]
        );
        e.call(0x00d36c30, &args![record_at]);
        assert_eq!(
            words(&e, record_at, 4),
            [0x00d3_6be0, 0x00d4_3dc0, 0x00d4_3960, 0x00d4_3b90]
        );
        assert_eq!(
            [e.mem.u8(record_at + 0x10), e.mem.u8(record_at + 0x11)],
            [1, 1]
        );
    }

    #[test]
    fn the_symmetric_wrappers_swap_the_first_two_words() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(WRAPPER_FLOAT, 0x7fa0_0000);
        let seen: Calls = Rc::default();
        let sink = Rc::clone(&seen);
        e.register_double(CLOSEST_POINTS_BODY, move |e, a| {
            let mut row = a.to_vec();
            row.extend(words(e, a[3], 3));
            sink.borrow_mut().push(row);
            Ret::default()
        });
        let sink = Rc::clone(&seen);
        e.register_double(PENETRATIONS_BODY, move |e, a| {
            let mut row = a.to_vec();
            row.extend(words(e, a[3], 3));
            sink.borrow_mut().push(row);
            Ret::default()
        });
        e.call(0x00d36ce0, &args![0x9000u32, 1u32, 2u32, 3u32, 0x44u32]);
        e.call(0x00d36d20, &args![0x9000u32, 5u32, 6u32, 7u32, 0x88u32]);
        let rows = seen.borrow();
        // The wrapper is on the stack; its address is the fourth word.
        assert_eq!(rows[0][..3], [2, 1, 3]);
        assert_eq!(rows[0][4..], [0x010c_e7fc, 0x7fe0_0000, 0x44]);
        assert_eq!(rows[1][..3], [6, 5, 7]);
        assert_eq!([rows[1][4], rows[1][5], rows[1][6]], [0x010c_e81c, 0, 0x88]);
    }

    #[test]
    fn the_symmetric_linear_cast_builds_the_wrappers_and_the_flipped_copy() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(WRAPPER_FLOAT, 0x7fa0_0000);
        for (lane, bits) in [0x8000_0000u32, 0, 0x8000_0000, 0].iter().enumerate() {
            e.mem.set_u32(SIGN_MASK + 4 * lane as u32, *bits);
        }
        let seen: Calls = Rc::default();
        let sink = Rc::clone(&seen);
        e.register_double(LINEAR_CAST_BODY, move |e, a| {
            let mut row = a.to_vec();
            row.extend(words(e, a[2], 0x1c));
            row.extend(words(e, a[3], 2));
            row.extend(words(e, a[3] + 0x10, 5));
            if a[4] != 0 {
                row.extend(words(e, a[4], 2));
                row.extend(words(e, a[4] + 0x10, 5));
            }
            sink.borrow_mut().push(row);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let third = e.mem.alloc(0x80);
        for word in 0..0x1c {
            e.mem.set_u32(third + 4 * word, 0x100 + word);
        }
        e.call(0x00d36e80, &args![this, 1u32, 2u32, third, 0x44u32, 0u32]);
        e.call(
            0x00d36e80,
            &args![this, 1u32, 2u32, third, 0x44u32, 0x66u32],
        );
        let rows = seen.borrow();
        let vector = [0x114u32 ^ 0x8000_0000, 0x115, 0x116 ^ 0x8000_0000, 0x117];
        for (row, other) in [(&rows[0], false), (&rows[1], true)] {
            assert_eq!(row[0], 2);
            assert_eq!(row[1], 1);
            // The copy of the third structure, with its vector at +0x50 flipped.
            let copy = &row[5..5 + 0x1c];
            assert_eq!(
                copy[..0x14],
                (0..0x14).map(|w| 0x100 + w).collect::<Vec<u32>>()[..]
            );
            assert_eq!(copy[0x14..0x18], vector);
            assert_eq!(copy[0x18..], [0x118, 0x119, 0x11a, 0x11b]);
            // The wrapper: vtable, quieted float, the unflipped vector, the fourth word.
            let wrapper = &row[5 + 0x1c..];
            assert_eq!(wrapper[..2], [0x010c_e80c, 0x7fe0_0000]);
            assert_eq!(wrapper[2..6], [0x114, 0x115, 0x116, 0x117]);
            assert_eq!(wrapper[6], 0x44);
            assert_eq!(wrapper.len(), if other { 14 } else { 7 });
            if other {
                assert_eq!(wrapper[7..9], [0x010c_e80c, 0x7fe0_0000]);
                assert_eq!(wrapper[9..13], [0x114, 0x115, 0x116, 0x117]);
                assert_eq!(wrapper[13], 0x66);
            }
        }
    }

    #[test]
    fn the_destructor_frees_owned_storage_and_the_object_on_request() {
        let mut e = Engine::new();
        let frees = record(&mut e, SLOT_DOUBLE);
        let deletes = record(&mut e, DELETE_DOUBLE);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, SLOT_DOUBLE);
        e.mem.set_u32(vtable + 8, DELETE_DOUBLE);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = router_with_cursor(&mut e, 0);
        e.mem.set_u32(router + 0x10, allocator);
        let this = e.mem.alloc(0x20);
        e.mem.set_u16(this + 4, 0x30);
        e.mem.set_u32(this + 0x10, 0x1111);
        e.mem.set_u32(this + 0x14, 3);
        e.mem.set_u32(this + 0x18, 5);
        let result = e.call(0x00d36f50, &args![this, 3u32]);
        assert_eq!(result.u32(), this);
        assert_eq!(*frees.borrow(), vec![vec![allocator, 0x1111, 20]]);
        assert_eq!(*deletes.borrow(), vec![vec![allocator, this, 0x30]]);
        assert_eq!(words(&e, this + 0x10, 3), [0, 0, 0x8000_0000]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        // Storage that is not owned is left alone, and without the flag nothing is deleted.
        frees.borrow_mut().clear();
        deletes.borrow_mut().clear();
        e.mem.set_u32(this + 0x10, 0x2222);
        e.mem.set_u32(this + 0x18, 0x8000_0004);
        e.call(0x00d36f50, &args![this, 0u32]);
        assert!(frees.borrow().is_empty());
        assert!(deletes.borrow().is_empty());
        assert_eq!(words(&e, this + 0x10, 3), [0, 0, 0x8000_0000]);
    }
}
