//! `Havok SDK/hkpshapecollectionagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpworld::{profile_record, PROFILE_END};

/// The profiling label `"TtShapeCollection"`.
const LABEL_SHAPE_COLLECTION: u32 = 0x010c_f300;

/// Size of the buffer a shape collection builds a child shape in (0x200 bytes on the stack).
const CHILD_SHAPE_BUFFER: u32 = 0x200;

/// Offset of the table of linear-cast functions inside the dispatcher object, and the size of
/// one table entry (five words).
const DISPATCH_TABLE: u32 = 0x9ac;
const DISPATCH_ENTRY: u32 = 0x14;

// Translated from 00d43aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollectionAgent::linearCast` (Xbox PDB), between profiling records
/// `"TtShapeCollection"` and the end record. `entry` (a collision entry: object at +0, a word
/// at +8) provides the shape collection by its virtual method at slot 0x10. For every
/// `(key, agent)` pair of the array at `this + 0xc` (count at `this + 0x10`), first to last,
/// the collection builds the child shape for `key` in a 0x200-byte local buffer (slot 0x14:
/// `(key, buffer)`), and the pair's agent gets its virtual method at slot 0x14 called with a
/// 16-byte record (child shape, `key`, the word at `entry + 8`, `entry`) and the four remaining
/// words.
pub fn hkp_shape_collection_agent_linear_cast(
    e: &mut Engine,
    this: Ptr,
    entry: Ptr,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
) {
    profile_record(e, LABEL_SHAPE_COLLECTION);
    let owner = e.mem.u32(entry.addr());
    let collection = e.vcall(owner, 0x10, &args![]).u32();
    let entry_word = e.mem.u32(entry.addr() + 8);
    let mut remaining = e.mem.i32(this.addr() + 0x10).wrapping_sub(1);
    let mut pair = e.mem.u32(this.addr() + 0xc);
    e.with_stack(0x20 + CHILD_SHAPE_BUFFER, |e, frame| {
        let record = frame.addr();
        let buffer = frame.addr() + 0x20;
        while remaining >= 0 {
            let key = e.mem.u32(pair);
            let child = e.vcall(collection, 0x14, &args![key, buffer]).u32();
            e.mem.set_u32(record, child);
            e.mem.set_u32(record + 4, key);
            e.mem.set_u32(record + 8, entry_word);
            e.mem.set_u32(record + 12, entry.addr());
            let agent = e.mem.u32(pair + 4);
            e.vcall(agent, 0x14, &args![record, a, b, c, d]);
            pair = pair.wrapping_add(8);
            remaining -= 1;
        }
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00d43b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, five stack words, no result; the engine map files it under
/// `hkpshapecollectionagent.cpp`). Between profiling records `"TtShapeCollection"` and the
/// end record, walks the keys of the shape collection that `first` provides (virtual method at
/// slot 0x10 of the object at `first + 0`) with its slots 8 (first key) and 0xC (next key) until
/// -1. For each key the filter object embedded at `[third + 0xc] + 0xc` is asked (its slot 4,
/// `(&answer, third, second, first, collection, key)`, answering through the returned pointer)
/// whether the pair is enabled; if it is, the collection builds the child shape for the key in
/// a local buffer (slot 0x14), and the function found in the dispatcher table
/// `[third] + 0x9ac + 0x14 * index` is called (cdecl) with a 16-byte record (child shape, key,
/// the word at `first + 8`, `first`) and `second`, `third`, `fourth`, `fifth`. The table index
/// is the byte at `[third] + 0x20 * (child shape's byte-word at +0xc + 0xd) + [[second] + 0xc]`.
pub fn fn_00d43b90(e: &mut Engine, first: Ptr, second: Ptr, third: Ptr, fourth: u32, fifth: u32) {
    profile_record(e, LABEL_SHAPE_COLLECTION);
    let owner = e.mem.u32(first.addr());
    let collection = e.vcall(owner, 0x10, &args![]).u32();
    let entry_word = e.mem.u32(first.addr() + 8);
    let second_object = e.mem.u32(second.addr());
    let selector = e.mem.u32(second_object + 0xc);
    e.with_stack(0x20 + 0x10 + CHILD_SHAPE_BUFFER, |e, frame| {
        let answer = frame.addr();
        let record = frame.addr() + 0x10;
        let buffer = frame.addr() + 0x20;
        let mut key = e.vcall(collection, 8, &args![]).u32();
        while key != u32::MAX {
            let filter_owner = e.mem.u32(third.addr() + 0xc);
            let filter = filter_owner + 0xc;
            let returned = e
                .vcall(
                    filter,
                    4,
                    &args![answer, third, second, first, collection, key],
                )
                .u32();
            if e.mem.u8(returned) != 0 {
                let child = e.vcall(collection, 0x14, &args![key, buffer]).u32();
                e.mem.set_u32(record, child);
                e.mem.set_u32(record + 4, key);
                e.mem.set_u32(record + 8, entry_word);
                e.mem.set_u32(record + 12, first.addr());
                let table = e.mem.u32(third.addr());
                let type_word = e.mem.u32(child.wrapping_add(0xc));
                let index_address = type_word
                    .wrapping_add(0xd)
                    .wrapping_shl(5)
                    .wrapping_add(table)
                    .wrapping_add(selector);
                let index = u32::from(e.mem.u8(index_address));
                let function = e
                    .mem
                    .u32(table + DISPATCH_TABLE + index.wrapping_mul(DISPATCH_ENTRY));
                e.call(function, &args![record, second, third, fourth, fifth]);
            }
            key = e.vcall(collection, 0xc, &args![key]).u32();
        }
    });
    profile_record(e, PROFILE_END);
}

/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The array grower (`00c90940`, cdecl `(allocator, array, element size)`).
const RESERVE_MORE: u32 = 0x00c9_0940;
/// The key that ends a walk of the shape collection.
const NO_KEY: u32 = 0xffff_ffff;
/// Offset of the dispatcher tables of `updateShapeCollectionFilter`, and of its two index
/// byte tables (chosen by the word at `input + 0x14`).
const UPDATE_DISPATCH_TABLE: u32 = 0x9a0;
const INDEX_TABLE_WITH_FLAG: u32 = 0x5a0;
const INDEX_TABLE_WITHOUT_FLAG: u32 = 0x1a0;
/// The tables of the static closest point and penetration walks.
const CLOSEST_POINTS_TABLE: u32 = 0x9a8;
const PENETRATIONS_TABLE: u32 = 0x9a4;

// Translated from 00d36920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollectionAgent::staticGetClosestPoints` (Xbox PDB), cdecl with four words: a
/// jump to the list agent's static function `00d347d0` with the same stack, so the four words
/// are handed on unchanged.
pub fn hkp_shape_collection_agent_static_get_closest_points(
    e: &mut Engine,
    body_a: u32,
    body_b: u32,
    input: u32,
    collector: u32,
) {
    e.call(0x00d3_47d0, &args![body_a, body_b, input, collector]);
}

// Translated from 00d36990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollectionAgent::staticGetPenetrations` (Xbox PDB), cdecl with four words: a jump
/// to the list agent's static function `00d34790` with the same stack, so the four words are
/// handed on unchanged.
pub fn hkp_shape_collection_agent_static_get_penetrations(
    e: &mut Engine,
    body_a: u32,
    body_b: u32,
    input: u32,
    collector: u32,
) {
    e.call(0x00d3_4790, &args![body_a, body_b, input, collector]);
}

// Translated from 00d436f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollectionAgent::cleanup` (Xbox PDB), thiscall with one stack word (`RET 4`): the
/// word is handed to the slot `0x1C` of every agent of the pair array (`this + 0xC`, stride 8,
/// the agent is the second word of a pair, count at `this + 0x10`), then the object destroys
/// itself with its slot 0 (flag 1).
pub fn hkp_shape_collection_agent_cleanup(e: &mut Engine, this: Ptr, constraint_owner: u32) {
    let this = this.addr();
    let mut index: u32 = 0;
    if e.mem.i32(this + 0x10) > 0 {
        loop {
            let pairs = e.mem.u32(this + 0xc);
            let agent = e
                .mem
                .u32(pairs.wrapping_add(index.wrapping_mul(8)).wrapping_add(4));
            e.vcall(agent, 0x1c, &args![constraint_owner]);
            index += 1;
            if index as i32 >= e.mem.i32(this + 0x10) {
                break;
            }
        }
    }
    e.vcall(this, 0, &args![1u32]);
}

// Translated from 00d43730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollectionAgent::invalidateTim` (Xbox PDB), thiscall with one stack word
/// (`RET 4`): the word is handed to the slot `0x24` of every agent of the pair array.
pub fn hkp_shape_collection_agent_invalidate_tim(e: &mut Engine, this: Ptr, input: u32) {
    let this = this.addr();
    let mut index: u32 = 0;
    if e.mem.i32(this + 0x10) > 0 {
        loop {
            let pairs = e.mem.u32(this + 0xc);
            let agent = e
                .mem
                .u32(pairs.wrapping_add(index.wrapping_mul(8)).wrapping_add(4));
            e.vcall(agent, 0x24, &args![input]);
            index += 1;
            if index as i32 >= e.mem.i32(this + 0x10) {
                break;
            }
        }
    }
}

// Translated from 00d43760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollectionAgent::warpTime` (Xbox PDB), thiscall with three stack words (`RET 0xC`)
/// `(old time, new time, input)`: the two times (`float`s that go through the x87 stack) and
/// the input are handed to the slot `0x28` of every agent of the pair array.
pub fn hkp_shape_collection_agent_warp_time(
    e: &mut Engine,
    this: Ptr,
    old_time: f32,
    new_time: f32,
    input: u32,
) {
    let this = this.addr();
    let mut index: u32 = 0;
    if e.mem.i32(this + 0x10) > 0 {
        loop {
            let pairs = e.mem.u32(this + 0xc);
            let agent = e
                .mem
                .u32(pairs.wrapping_add(index.wrapping_mul(8)).wrapping_add(4));
            e.vcall(
                agent,
                0x28,
                &args![
                    x87_quiet_bits(old_time.to_bits()),
                    x87_quiet_bits(new_time.to_bits()),
                    input
                ],
            );
            index += 1;
            if index as i32 >= e.mem.i32(this + 0x10) {
                break;
            }
        }
    }
}

/// The walk `processCollision` and `getClosestPoints` share: between the profiling records,
/// for every `(key, agent)` pair from the first to the last (the count is taken once) the
/// collection of `body_a` builds the child shape of the key in a local buffer, and the agent's
/// virtual function at `slot` runs with a 16-byte record (child shape, key, the word at
/// `body_a + 8`, `body_a`) and the three words.
fn walk_pairs(e: &mut Engine, this: Ptr, body_a: Ptr, slot: u32, words: [u32; 3]) {
    profile_record(e, LABEL_SHAPE_COLLECTION);
    let owner = e.mem.u32(body_a.addr());
    let collection = e.vcall(owner, 0x10, &args![]).u32();
    let entry_word = e.mem.u32(body_a.addr() + 8);
    let mut remaining = e.mem.i32(this.addr() + 0x10).wrapping_sub(1);
    let mut pair = e.mem.u32(this.addr() + 0xc);
    e.with_stack(0x20 + CHILD_SHAPE_BUFFER, |e, frame| {
        let record = frame.addr();
        let buffer = frame.addr() + 0x20;
        e.mem.set_u32(record + 8, entry_word);
        e.mem.set_u32(record + 12, body_a.addr());
        while remaining >= 0 {
            let key = e.mem.u32(pair);
            let child = e.vcall(collection, 0x14, &args![key, buffer]).u32();
            e.mem.set_u32(record, child);
            e.mem.set_u32(record + 4, key);
            let agent = e.mem.u32(pair + 4);
            e.vcall(agent, slot, &args![record, words[0], words[1], words[2]]);
            pair = pair.wrapping_add(8);
            remaining -= 1;
        }
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00d437a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollectionAgent::processCollision` (Xbox PDB), thiscall with four stack words
/// (`RET 0x10`) `(body_a, body_b, input, output)`: between profiling records
/// `"TtShapeCollection"` and the end record, walks the pair array from the first pair to the
/// last; the child shape of each key is built by the collection of `body_a` (slot `0x10` of
/// its object, then slot `0x14` with `(key, buffer)`) and the pair's agent gets its slot `0x18`
/// called with a 16-byte record (child shape, key, the word at `body_a + 8`, `body_a`) and the
/// three other words.
pub fn hkp_shape_collection_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: u32,
    input: u32,
    output: u32,
) {
    walk_pairs(e, this, body_a, 0x18, [body_b, input, output]);
}

// Translated from 00d43880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollectionAgent::getClosestPoints` (Xbox PDB), thiscall with four stack words
/// (`RET 0x10`) `(body_a, body_b, input, output)`: the same walk as `processCollision`, but
/// with the agents' slot `0x10`.
pub fn hkp_shape_collection_agent_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: u32,
    input: u32,
    output: u32,
) {
    walk_pairs(e, this, body_a, 0x10, [body_b, input, output]);
}

/// The static walk `00d43960` and `00d43dc0` share (cdecl, four words `(first, second, third,
/// fourth)`): between the profiling records, walks the keys of the collection of `first` and,
/// for each key the filter at `[third + 0xc] + 0xc` accepts, calls (cdecl) the function at
/// `table_offset + 0x14 * index` of the dispatcher `[third]` with a record (child shape, key,
/// the word at `first + 8`, `first`), `second`, `third` and `fourth`; the index is the byte at
/// `[third] + 0x20 * (type + 0xd) + [[second] + 0xc]` (`type` is the word at `+0xC` of the
/// child shape). With `stop_when_flagged` the walk ends after a call when the byte at
/// `fourth + 4` is set.
fn walk_static(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    third: Ptr,
    fourth: u32,
    table_offset: u32,
    stop_when_flagged: bool,
) {
    profile_record(e, LABEL_SHAPE_COLLECTION);
    let owner = e.mem.u32(first.addr());
    let collection = e.vcall(owner, 0x10, &args![]).u32();
    let entry_word = e.mem.u32(first.addr() + 8);
    let second_object = e.mem.u32(second.addr());
    let selector = e.mem.u32(second_object + 0xc);
    e.with_stack(0x20 + 0x10 + CHILD_SHAPE_BUFFER, |e, frame| {
        let answer = frame.addr();
        let record = frame.addr() + 0x10;
        let buffer = frame.addr() + 0x20;
        e.mem.set_u32(record + 8, entry_word);
        e.mem.set_u32(record + 12, first.addr());
        let mut key = e.vcall(collection, 8, &args![]).u32();
        while key != NO_KEY {
            let filter = e.mem.u32(third.addr() + 0xc) + 0xc;
            let returned = e
                .vcall(
                    filter,
                    4,
                    &args![answer, third, second, first, collection, key],
                )
                .u32();
            if e.mem.u8(returned) != 0 {
                let child = e.vcall(collection, 0x14, &args![key, buffer]).u32();
                e.mem.set_u32(record, child);
                e.mem.set_u32(record + 4, key);
                let table = e.mem.u32(third.addr());
                let type_word = e.mem.u32(child.wrapping_add(0xc));
                let index_address = type_word
                    .wrapping_add(0xd)
                    .wrapping_shl(5)
                    .wrapping_add(table)
                    .wrapping_add(selector);
                let index = u32::from(e.mem.u8(index_address));
                let function = e
                    .mem
                    .u32(table + table_offset + index.wrapping_mul(DISPATCH_ENTRY));
                e.call(function, &args![record, second, third, fourth]);
                if stop_when_flagged && e.mem.u8(fourth + 4) != 0 {
                    break;
                }
            }
            key = e.vcall(collection, 0xc, &args![key]).u32();
        }
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00d43960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, four stack words, no result; the static closest points walk of the shape
/// collection agent, reached through `staticGetClosestPoints`). See `walk_static`: the table
/// is at `[third] + 0x9a8` and the walk never stops early.
pub fn fn_00d43960(e: &mut Engine, first: Ptr, second: Ptr, third: Ptr, fourth: u32) {
    walk_static(e, first, second, third, fourth, CLOSEST_POINTS_TABLE, false);
}

// Translated from 00d43dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, four stack words, no result; the static penetrations walk of the shape
/// collection agent, reached through `staticGetPenetrations`). See `walk_static`: the table is
/// at `[third] + 0x9a4` and the walk ends after the first call that leaves the byte at
/// `fourth + 4` set.
pub fn fn_00d43dc0(e: &mut Engine, first: Ptr, second: Ptr, third: Ptr, fourth: u32) {
    walk_static(e, first, second, third, fourth, PENETRATIONS_TABLE, true);
}

// Translated from 00d43cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollectionAgent::getPenetrations` (Xbox PDB), thiscall with four stack words
/// (`RET 0x10`) `(body_a, body_b, input, output)`: like `processCollision` but with the
/// agents' slot `0xC`, and the walk ends after the first agent that leaves the byte at
/// `output + 4` set.
pub fn hkp_shape_collection_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: u32,
    input: u32,
    output: u32,
) {
    profile_record(e, LABEL_SHAPE_COLLECTION);
    let owner = e.mem.u32(body_a.addr());
    let collection = e.vcall(owner, 0x10, &args![]).u32();
    let entry_word = e.mem.u32(body_a.addr() + 8);
    let mut remaining = e.mem.i32(this.addr() + 0x10).wrapping_sub(1);
    let mut pair = e.mem.u32(this.addr() + 0xc);
    e.with_stack(0x20 + CHILD_SHAPE_BUFFER, |e, frame| {
        let record = frame.addr();
        let buffer = frame.addr() + 0x20;
        e.mem.set_u32(record + 8, entry_word);
        e.mem.set_u32(record + 12, body_a.addr());
        while remaining >= 0 {
            let key = e.mem.u32(pair);
            let child = e.vcall(collection, 0x14, &args![key, buffer]).u32();
            e.mem.set_u32(record + 4, key);
            e.mem.set_u32(record, child);
            let agent = e.mem.u32(pair + 4);
            e.vcall(agent, 0xc, &args![record, body_b, input, output]);
            if e.mem.u8(output + 4) != 0 {
                break;
            }
            pair = pair.wrapping_add(8);
            remaining -= 1;
        }
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00d43f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollectionAgent::updateShapeCollectionFilter` (Xbox PDB), thiscall with four
/// stack words (`RET 0x10`) `(body_a, body_b, input, constraint_owner)`. For every key of the
/// collection of `body_a` (slot `0x10` of its object; slots 8 and `0xC` walk the keys, slot
/// `0x14` builds a child shape in a local buffer) it looks the key up in the pair array
/// (`this + 0xC`, stride 8, count at `this + 0x10`, capacity word at `this + 0x14`) and asks
/// the filter of `input` (`[input + 0xC] + 0xC`, slot 4, `(&answer, input, body_b, body_a,
/// collection, key)`, answering through the returned pointer):
///
/// * Rejected and present: the agent's slot `0x1C` runs with `constraint_owner`, the count
///   drops and the last pair moves into the freed place (unless it was the last).
/// * Accepted and absent: the pair array grows by one (`00c90940` when full), the key is
///   appended and the agent comes from the dispatcher function at `[input] + 0x9a0 + 0x14 *
///   index` called (cdecl) with `(record, body_b, input, the word at this + 8)`; the answer is
///   the pair's agent. The index is the byte at `type * 0x20 + [input] + base + [[body_b] +
///   0xC]`, where `type` is the child shape's word at `+0xC` and `base` is `0x5a0` when the
///   word at `input + 0x14` is nonzero and `0x1a0` otherwise.
/// * Accepted and present: the agent's slot `0x20` runs with `(record, body_b, input,
///   constraint_owner)`.
///
/// The record is (child shape, key, the word at `body_a + 8`, `body_a`).
pub fn hkp_shape_collection_agent_update_shape_collection_filter(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    constraint_owner: u32,
) {
    let this = this.addr();
    let (body_a, body_b, input) = (body_a.addr(), body_b.addr(), input.addr());
    let owner = e.mem.u32(body_a);
    let collection = e.vcall(owner, 0x10, &args![]).u32();
    let entry_word = e.mem.u32(body_a + 8);
    let mut key = e.vcall(collection, 8, &args![]).u32();
    e.with_stack(0x20 + 0x10 + CHILD_SHAPE_BUFFER, |e, frame| {
        let answer = frame.addr();
        let record = frame.addr() + 0x10;
        let buffer = frame.addr() + 0x20;
        e.mem.set_u32(record + 8, entry_word);
        e.mem.set_u32(record + 12, body_a);
        while key != NO_KEY {
            // The slot of the key in the pair array, or -1.
            let count = e.mem.i32(this + 0x10);
            let mut slot: i32 = -1;
            if count > 0 {
                let mut pair = e.mem.u32(this + 0xc);
                let mut index: i32 = 0;
                loop {
                    if e.mem.u32(pair) == key {
                        slot = index;
                        break;
                    }
                    index += 1;
                    pair = pair.wrapping_add(8);
                    if index >= count {
                        break;
                    }
                }
            }
            let filter = e.mem.u32(input + 0xc) + 0xc;
            let returned = e
                .vcall(
                    filter,
                    4,
                    &args![answer, input, body_b, body_a, collection, key],
                )
                .u32();
            if e.mem.u8(returned) == 0 {
                if slot != -1 {
                    let pairs = e.mem.u32(this + 0xc);
                    let agent = e.mem.u32(
                        pairs
                            .wrapping_add((slot as u32).wrapping_mul(8))
                            .wrapping_add(4),
                    );
                    e.vcall(agent, 0x1c, &args![constraint_owner]);
                    let remaining = e.mem.i32(this + 0x10).wrapping_sub(1);
                    e.mem.set_u32(this + 0x10, remaining as u32);
                    if remaining != slot {
                        let pairs = e.mem.u32(this + 0xc);
                        let destination = pairs.wrapping_add((slot as u32).wrapping_mul(8));
                        let source = pairs.wrapping_add((remaining as u32).wrapping_mul(8));
                        for word in 0..2 {
                            let value = e.mem.u32(source + 4 * word);
                            e.mem.set_u32(destination + 4 * word, value);
                        }
                    }
                }
            } else {
                let child = e.vcall(collection, 0x14, &args![key, buffer]).u32();
                e.mem.set_u32(record, child);
                e.mem.set_u32(record + 4, key);
                if slot == -1 {
                    let array = this + 0xc;
                    let router = e.call(MEMORY_ROUTER, &args![]).u32();
                    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
                        let allocator = e.mem.u32(router + 0x10);
                        e.call(RESERVE_MORE, &args![allocator, array, 8u32]);
                    }
                    let size = e.mem.u32(array + 4);
                    let new_pair = e.mem.u32(array).wrapping_add(size.wrapping_mul(8));
                    e.mem.set_u32(array + 4, size.wrapping_add(1));
                    e.mem.set_u32(new_pair, key);
                    let selector = e.mem.u32(e.mem.u32(body_b) + 0xc);
                    let table = e.mem.u32(input);
                    let base = if e.mem.u32(input + 0x14) == 0 {
                        table + INDEX_TABLE_WITHOUT_FLAG
                    } else {
                        table + INDEX_TABLE_WITH_FLAG
                    };
                    let type_word = e.mem.u32(child.wrapping_add(0xc));
                    let index_address = type_word
                        .wrapping_shl(5)
                        .wrapping_add(base)
                        .wrapping_add(selector);
                    let index = u32::from(e.mem.u8(index_address));
                    let function = e
                        .mem
                        .u32(table + UPDATE_DISPATCH_TABLE + index.wrapping_mul(DISPATCH_ENTRY));
                    let manager = e.mem.u32(this + 8);
                    let agent = e
                        .call(function, &args![record, body_b, input, manager])
                        .u32();
                    e.mem.set_u32(new_pair + 4, agent);
                } else {
                    let pairs = e.mem.u32(this + 0xc);
                    let agent = e.mem.u32(
                        pairs
                            .wrapping_add((slot as u32).wrapping_mul(8))
                            .wrapping_add(4),
                    );
                    e.vcall(agent, 0x20, &args![record, body_b, input, constraint_owner]);
                }
            }
            key = e.vcall(collection, 0xc, &args![key]).u32();
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d3_6920,
            hkp_shape_collection_agent_static_get_closest_points(u32, u32, u32, u32)
        ),
        entry!(
            0x00d3_6990,
            hkp_shape_collection_agent_static_get_penetrations(u32, u32, u32, u32)
        ),
        entry!(0x00d4_36f0, hkp_shape_collection_agent_cleanup(Ptr, u32)),
        entry!(
            0x00d4_3730,
            hkp_shape_collection_agent_invalidate_tim(Ptr, u32)
        ),
        entry!(
            0x00d4_3760,
            hkp_shape_collection_agent_warp_time(Ptr, f32, f32, u32)
        ),
        entry!(
            0x00d4_37a0,
            hkp_shape_collection_agent_process_collision(Ptr, Ptr, u32, u32, u32)
        ),
        entry!(
            0x00d4_3880,
            hkp_shape_collection_agent_get_closest_points(Ptr, Ptr, u32, u32, u32)
        ),
        entry!(0x00d4_3960, fn_00d43960(Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00d4_3cd0,
            hkp_shape_collection_agent_get_penetrations(Ptr, Ptr, u32, u32, u32)
        ),
        entry!(0x00d4_3dc0, fn_00d43dc0(Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00d4_3f10,
            hkp_shape_collection_agent_update_shape_collection_filter(Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00d43aa0,
            hkp_shape_collection_agent_linear_cast(Ptr, Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00d43b90, fn_00d43b90(Ptr, Ptr, Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::hkpworld::profiling_support::{labels, profiling_engine};
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const OWNER_SLOT_10: u32 = 0x0900_0110;
    const COLLECTION_FIRST_KEY: u32 = 0x0900_0108;
    const COLLECTION_NEXT_KEY: u32 = 0x0900_010c;
    const COLLECTION_CHILD: u32 = 0x0900_0114;
    const AGENT_LINEAR_CAST: u32 = 0x0900_0214;
    const FILTER_SLOT_4: u32 = 0x0900_0304;
    const DISPATCHED: u32 = 0x0900_0400;

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    /// An object with the given vtable slots.
    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x40);
        for &(slot, target) in slots {
            e.mem.set_u32(table + slot, target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        object
    }

    /// A collection of the keys `keys` (the next-key double walks them, ending with -1).
    fn collection(e: &mut Engine, keys: &'static [u32], log: &Log) -> u32 {
        e.register_double(COLLECTION_FIRST_KEY, move |_, _| Ret {
            eax: keys.first().copied().unwrap_or(u32::MAX),
            ..Ret::default()
        });
        e.register_double(COLLECTION_NEXT_KEY, move |_, a| {
            let at = keys.iter().position(|k| *k == a[1]).unwrap();
            Ret {
                eax: keys.get(at + 1).copied().unwrap_or(u32::MAX),
                ..Ret::default()
            }
        });
        let sink = log.clone();
        e.register_double(COLLECTION_CHILD, move |_, a| {
            sink.borrow_mut().push(("child", vec![a[1], a[2]]));
            // The child shape is a block whose +0xc word is the key (the type).
            Ret {
                eax: a[2] + 0x100,
                ..Ret::default()
            }
        });
        object(
            e,
            8,
            &[
                (8, COLLECTION_FIRST_KEY),
                (0xc, COLLECTION_NEXT_KEY),
                (0x14, COLLECTION_CHILD),
            ],
        )
    }

    fn entry_with(e: &mut Engine, collection: u32) -> Ptr {
        let owner = object(e, 8, &[(0x10, OWNER_SLOT_10)]);
        e.register_double(OWNER_SLOT_10, move |_, _| Ret {
            eax: collection,
            ..Ret::default()
        });
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry, owner);
        e.mem.set_u32(entry + 8, 0xe8e8);
        Ptr::new(entry)
    }

    #[test]
    fn linear_cast_visits_every_pair_inside_profiling_records() {
        let (mut e, stream, start) = profiling_engine(4);
        let log: Log = Rc::new(RefCell::new(vec![]));
        let collection = collection(&mut e, &[], &log);
        let entry = entry_with(&mut e, collection);
        let sink = log.clone();
        e.register_double(AGENT_LINEAR_CAST, move |e, a| {
            let record = (0..4).map(|i| e.mem.u32(a[1] + 4 * i)).collect::<Vec<_>>();
            let mut words = vec![a[0]];
            words.extend(record);
            words.extend(&a[2..]);
            sink.borrow_mut().push(("agent", words));
            Ret::default()
        });
        let agent_a = object(&mut e, 8, &[(0x14, AGENT_LINEAR_CAST)]);
        let agent_b = object(&mut e, 8, &[(0x14, AGENT_LINEAR_CAST)]);
        let this = e.mem.alloc(0x20);
        let pairs = e.mem.alloc(16);
        for (i, (key, agent)) in [(5u32, agent_a), (9, agent_b)].into_iter().enumerate() {
            e.mem.set_u32(pairs + 8 * i as u32, key);
            e.mem.set_u32(pairs + 8 * i as u32 + 4, agent);
        }
        e.mem.set_u32(this + 0xc, pairs);
        e.mem.set_i32(this + 0x10, 2);
        e.call(0x00d43aa0, &args![this, entry, 1u32, 2u32, 3u32, 4u32]);
        let log = log.borrow();
        let agents: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(kind, _)| *kind == "agent")
            .map(|(_, words)| words)
            .collect();
        assert_eq!(agents.len(), 2);
        // Agent, record (child, key, entry word, entry), then the four words.
        assert_eq!(agents[0][0], agent_a);
        assert_eq!(agents[0][2], 5);
        assert_eq!(agents[0][3..], [0xe8e8, entry.addr(), 1, 2, 3, 4]);
        assert_eq!(agents[1][0], agent_b);
        assert_eq!(agents[1][2], 9);
        // The child shapes were built in a buffer, in pair order.
        let children: Vec<u32> = log
            .iter()
            .filter(|(kind, _)| *kind == "child")
            .map(|(_, words)| words[0])
            .collect();
        assert_eq!(children, vec![5, 9]);
        assert_eq!(
            labels(&e, stream, start),
            vec![LABEL_SHAPE_COLLECTION, PROFILE_END]
        );
    }

    #[test]
    fn linear_cast_with_no_pairs_only_writes_the_records() {
        let (mut e, stream, start) = profiling_engine(4);
        let log: Log = Rc::new(RefCell::new(vec![]));
        let collection = collection(&mut e, &[], &log);
        let entry = entry_with(&mut e, collection);
        let this = e.mem.alloc(0x20);
        e.mem.set_i32(this + 0x10, 0);
        e.call(0x00d43aa0, &args![this, entry, 1u32, 2u32, 3u32, 4u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(
            labels(&e, stream, start),
            vec![LABEL_SHAPE_COLLECTION, PROFILE_END]
        );
    }

    /// Scene for `fn_00d43b90`: keys 4, 6 and 8; the filter enables 4 and 8 only. `second`'s
    /// first word points to an object whose +0xc word is the selector 2; the dispatcher object
    /// (`third`'s first word) holds a table of index bytes and one function per index.
    #[test]
    fn the_filtered_keys_are_dispatched_through_the_table() {
        let (mut e, stream, start) = profiling_engine(4);
        let log: Log = Rc::new(RefCell::new(vec![]));
        let collection = collection(&mut e, &[4, 6, 8], &log);
        let first = entry_with(&mut e, collection);

        // `second`: [second] -> object with selector 2 at +0xc.
        let second_object = e.mem.alloc(0x20);
        e.mem.set_u32(second_object + 0xc, 2);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, second_object);

        // The dispatcher (`[third]`): index byte for type t at table + 0x20 * (t + 0xd) + 2.
        let dispatcher = e.mem.alloc(0x1000);
        // The child shape of key k is at buffer + 0x100; its +0xc word is set below in the
        // child double through the buffer, so give the type word its value by key.
        e.register_double(COLLECTION_CHILD, move |e, a| {
            let child = a[2] + 0x100;
            e.mem.set_u32(child + 0xc, a[1] - 4);
            Ret {
                eax: child,
                ..Ret::default()
            }
        });
        // Key 4 has type 0 and key 8 has type 4: index bytes 1 and 2.
        for (type_word, index) in [(0u32, 1u8), (4, 2)] {
            e.mem
                .set_u8(dispatcher + 0x20 * (type_word + 0xd) + 2, index);
        }
        // The functions of index 1 and 2.
        for (index, target) in [(1u32, DISPATCHED), (2, DISPATCHED + 4)] {
            e.mem
                .set_u32(dispatcher + DISPATCH_TABLE + 0x14 * index, target);
        }
        let seen = Rc::new(RefCell::new(vec![]));
        for target in [DISPATCHED, DISPATCHED + 4] {
            let sink = seen.clone();
            e.register_double(target, move |e, a| {
                let record: Vec<u32> = (0..4).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
                sink.borrow_mut().push((target, record, a[1..].to_vec()));
                Ret::default()
            });
        }
        let filter_owner = e.mem.alloc(0x20);
        let filter_table = e.mem.alloc(0x20);
        e.mem.set_u32(filter_table + 4, FILTER_SLOT_4);
        e.mem.set_u32(filter_owner + 0xc, filter_table);
        let third = e.mem.alloc(0x20);
        e.mem.set_u32(third, dispatcher);
        e.mem.set_u32(third + 0xc, filter_owner);
        let sink = log.clone();
        e.register_double(FILTER_SLOT_4, move |e, a| {
            // The answer goes through the hidden pointer, which is returned.
            let enabled = a[6] != 6;
            e.mem.set_u8(a[1], u8::from(enabled));
            sink.borrow_mut().push(("filter", a.to_vec()));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.call(0x00d43b90, &args![first, second, third, 0xaau32, 0xbbu32]);

        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        // Key 4: the record is (child, key, entry word, first); the stack words follow.
        let child_of = |record: &Vec<u32>| record[0];
        assert_eq!(seen[0].0, DISPATCHED);
        assert_eq!(seen[0].1[1..], [4, 0xe8e8, first.addr()]);
        assert_eq!(seen[0].2, vec![second, third, 0xaa, 0xbb]);
        assert_eq!(seen[1].0, DISPATCHED + 4);
        assert_eq!(seen[1].1[1..], [8, 0xe8e8, first.addr()]);
        assert_ne!(child_of(&seen[0].1), 0);
        // The filter saw every key with (collection, key) last.
        let log = log.borrow();
        let asked: Vec<u32> = log
            .iter()
            .filter(|(kind, _)| *kind == "filter")
            .map(|(_, words)| words[6])
            .collect();
        assert_eq!(asked, vec![4, 6, 8]);
        assert_eq!(
            labels(&e, stream, start),
            vec![LABEL_SHAPE_COLLECTION, PROFILE_END]
        );
    }

    const AGENT_SLOT_C: u32 = 0x0900_0212;
    const AGENT_SLOT_10: u32 = 0x0900_0210;
    const AGENT_SLOT_18: u32 = 0x0900_0218;
    const AGENT_SLOT_1C: u32 = 0x0900_021c;
    const AGENT_SLOT_20: u32 = 0x0900_0220;
    const AGENT_SLOT_24: u32 = 0x0900_0224;
    const AGENT_SLOT_28: u32 = 0x0900_0228;
    const DESTRUCTOR: u32 = 0x0900_0230;
    const UPDATE_DISPATCHED: u32 = 0x0900_0500;

    /// The words logged under `kind`.
    fn logged(log: &Log, kind: &str) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(logged_kind, _)| *logged_kind == kind)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A shape collection agent: pair array of `(key, agent)` with room for `capacity` pairs.
    fn agent_with_pairs(e: &mut Engine, pairs: &[(u32, u32)], capacity: u32) -> u32 {
        let this = e.mem.alloc(0x40);
        let data = e.mem.alloc(8 * capacity.max(1));
        for (index, (key, agent)) in pairs.iter().enumerate() {
            e.mem.set_u32(data + 8 * index as u32, *key);
            e.mem.set_u32(data + 8 * index as u32 + 4, *agent);
        }
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x10, pairs.len() as u32);
        e.mem.set_u32(this + 0x14, capacity);
        this
    }

    /// `count` agents whose virtual function at `slot` is the logging double `double`; returns
    /// the agent holding them and the agents.
    fn agents_at(e: &mut Engine, log: &Log, slot: u32, double: u32, count: u32) -> (u32, Vec<u32>) {
        let sink = log.clone();
        e.register_double(double, move |_, a| {
            sink.borrow_mut().push(("agent", a.to_vec()));
            Ret::default()
        });
        let agents: Vec<u32> = (0..count)
            .map(|_| object(e, 8, &[(slot, double)]))
            .collect();
        let pairs: Vec<(u32, u32)> = agents
            .iter()
            .enumerate()
            .map(|(index, agent)| (10 + index as u32, *agent))
            .collect();
        (agent_with_pairs(e, &pairs, count), agents)
    }

    #[test]
    fn the_static_thunks_hand_their_words_on_00d36920_00d36990() {
        for (thunk, target) in [(0x00d3_6920u32, 0x00d3_47d0u32), (0x00d3_6990, 0x00d3_4790)] {
            let mut e = Engine::new();
            let log: Log = Rc::new(RefCell::new(vec![]));
            let sink = log.clone();
            e.register_double(target, move |_, a| {
                sink.borrow_mut().push(("target", a.to_vec()));
                Ret::default()
            });
            e.call(thunk, &args![1u32, 2u32, 3u32, 4u32]);
            assert_eq!(logged(&log, "target"), vec![vec![1, 2, 3, 4]]);
        }
    }

    #[test]
    fn cleanup_00d436f0_cleans_every_agent_and_destroys_itself() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(vec![]));
        let (this, agents) = agents_at(&mut e, &log, 0x1c, AGENT_SLOT_1C, 2);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table, DESTRUCTOR);
        e.mem.set_u32(this, table);
        let sink = log.clone();
        e.register_double(DESTRUCTOR, move |_, a| {
            sink.borrow_mut().push(("destructor", a.to_vec()));
            Ret::default()
        });
        e.call(0x00d4_36f0, &args![this, 0x55u32]);
        assert_eq!(
            logged(&log, "agent"),
            vec![vec![agents[0], 0x55], vec![agents[1], 0x55]]
        );
        assert_eq!(logged(&log, "destructor"), vec![vec![this, 1]]);
        // The agent before the destructor, in the order of the log.
        assert_eq!(log.borrow().last().unwrap().0, "destructor");
    }

    #[test]
    fn cleanup_00d436f0_without_agents_only_destroys_itself() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(vec![]));
        let this = agent_with_pairs(&mut e, &[], 0);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table, DESTRUCTOR);
        e.mem.set_u32(this, table);
        let sink = log.clone();
        e.register_double(DESTRUCTOR, move |_, a| {
            sink.borrow_mut().push(("destructor", a.to_vec()));
            Ret::default()
        });
        e.call(0x00d4_36f0, &args![this, 0x55u32]);
        assert_eq!(*log.borrow(), vec![("destructor", vec![this, 1])]);
    }

    #[test]
    fn invalidate_tim_00d43730_hands_the_input_to_every_agent() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(vec![]));
        let (this, agents) = agents_at(&mut e, &log, 0x24, AGENT_SLOT_24, 3);
        e.call(0x00d4_3730, &args![this, 0x66u32]);
        assert_eq!(
            logged(&log, "agent"),
            agents
                .iter()
                .map(|agent| vec![*agent, 0x66])
                .collect::<Vec<_>>()
        );
        let empty = agent_with_pairs(&mut e, &[], 0);
        e.call(0x00d4_3730, &args![empty, 0x66u32]);
        assert_eq!(logged(&log, "agent").len(), 3);
    }

    #[test]
    fn warp_time_00d43760_hands_both_times_through_the_x87_stack() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(vec![]));
        let (this, agents) = agents_at(&mut e, &log, 0x28, AGENT_SLOT_28, 2);
        // The old time is a signalling NaN: the x87 stack quiets it.
        e.call(
            0x00d4_3760,
            &args![this, 0x7f80_0001u32, 1.5f32.to_bits(), 0x99u32],
        );
        let expected: Vec<Vec<u32>> = agents
            .iter()
            .map(|agent| vec![*agent, 0x7fc0_0001, 1.5f32.to_bits(), 0x99])
            .collect();
        assert_eq!(logged(&log, "agent"), expected);
    }

    /// Walks two pairs `(5, agent_a)` and `(9, agent_b)` of an agent whose collection has no
    /// keys; the agents' function at `slot` logs the record and the words.
    fn check_walk(address: u32, slot: u32, double: u32) {
        let (mut e, stream, start) = profiling_engine(4);
        let log: Log = Rc::new(RefCell::new(vec![]));
        let collection = collection(&mut e, &[], &log);
        let entry = entry_with(&mut e, collection);
        let sink = log.clone();
        e.register_double(double, move |e, a| {
            let record = (0..4).map(|i| e.mem.u32(a[1] + 4 * i)).collect::<Vec<_>>();
            let mut words = vec![a[0]];
            words.extend(record);
            words.extend(&a[2..]);
            sink.borrow_mut().push(("agent", words));
            Ret::default()
        });
        let agent_a = object(&mut e, 8, &[(slot, double)]);
        let agent_b = object(&mut e, 8, &[(slot, double)]);
        let this = agent_with_pairs(&mut e, &[(5, agent_a), (9, agent_b)], 2);
        e.call(address, &args![this, entry, 1u32, 2u32, 3u32]);
        let agents = logged(&log, "agent");
        assert_eq!(agents.len(), 2);
        // The agent, the record (child, key, entry word, entry), then the three words.
        assert_eq!(agents[0][0], agent_a);
        assert_eq!(agents[0][2], 5);
        assert_eq!(agents[0][3..], [0xe8e8, entry.addr(), 1, 2, 3]);
        assert_eq!(agents[1][0], agent_b);
        assert_eq!(agents[1][2], 9);
        assert_eq!(
            logged(&log, "child"),
            vec![vec![5, agents[0][1] - 0x100], vec![9, agents[1][1] - 0x100]]
        );
        assert_eq!(
            labels(&e, stream, start),
            vec![LABEL_SHAPE_COLLECTION, PROFILE_END]
        );
    }

    #[test]
    fn process_collision_00d437a0_calls_slot_0x18_of_every_agent() {
        check_walk(0x00d4_37a0, 0x18, AGENT_SLOT_18);
    }

    #[test]
    fn get_closest_points_00d43880_calls_slot_0x10_of_every_agent() {
        check_walk(0x00d4_3880, 0x10, AGENT_SLOT_10);
    }

    #[test]
    fn the_empty_walks_only_write_the_records_00d437a0_00d43880_00d43cd0() {
        for address in [0x00d4_37a0u32, 0x00d4_3880, 0x00d4_3cd0] {
            let (mut e, stream, start) = profiling_engine(4);
            let log: Log = Rc::new(RefCell::new(vec![]));
            let collection = collection(&mut e, &[], &log);
            let entry = entry_with(&mut e, collection);
            let this = agent_with_pairs(&mut e, &[], 0);
            let output = e.mem.alloc(8);
            e.call(address, &args![this, entry, 1u32, 2u32, output]);
            assert!(log.borrow().is_empty());
            assert_eq!(
                labels(&e, stream, start),
                vec![LABEL_SHAPE_COLLECTION, PROFILE_END]
            );
        }
    }

    #[test]
    fn get_penetrations_00d43cd0_stops_after_the_agent_that_sets_the_flag() {
        let (mut e, stream, start) = profiling_engine(4);
        let log: Log = Rc::new(RefCell::new(vec![]));
        let collection = collection(&mut e, &[], &log);
        let entry = entry_with(&mut e, collection);
        let agent_a = object(&mut e, 8, &[(0xc, AGENT_SLOT_C)]);
        let agent_b = object(&mut e, 8, &[(0xc, AGENT_SLOT_C)]);
        let agent_c = object(&mut e, 8, &[(0xc, AGENT_SLOT_C)]);
        let sink = log.clone();
        e.register_double(AGENT_SLOT_C, move |e, a| {
            let record = (0..4).map(|i| e.mem.u32(a[1] + 4 * i)).collect::<Vec<_>>();
            let mut words = vec![a[0]];
            words.extend(record);
            words.extend(&a[2..]);
            sink.borrow_mut().push(("agent", words));
            // The second agent finds a penetration.
            if a[0] == agent_b {
                e.mem.set_u8(a[4] + 4, 1);
            }
            Ret::default()
        });
        let this = agent_with_pairs(&mut e, &[(5, agent_a), (6, agent_b), (7, agent_c)], 3);
        let output = e.mem.alloc(8);
        e.call(0x00d4_3cd0, &args![this, entry, 1u32, 2u32, output]);
        let agents = logged(&log, "agent");
        assert_eq!(
            agents.iter().map(|call| call[0]).collect::<Vec<_>>(),
            vec![agent_a, agent_b]
        );
        assert_eq!(agents[0][3..], [0xe8e8, entry.addr(), 1, 2, output]);
        assert_eq!(agents[1][2], 6);
        assert_eq!(
            labels(&e, stream, start),
            vec![LABEL_SHAPE_COLLECTION, PROFILE_END]
        );
        // Without the flag, every agent runs.
        e.mem.set_u8(output + 4, 0);
        log.borrow_mut().clear();
        let this = agent_with_pairs(&mut e, &[(5, agent_a), (7, agent_c)], 2);
        e.call(0x00d4_3cd0, &args![this, entry, 1u32, 2u32, output]);
        assert_eq!(logged(&log, "agent").len(), 2);
    }

    /// The scene of the static walks: keys 4, 6 and 8 of the collection of `first`; the filter
    /// enables 4 and 8. The dispatcher keeps its index bytes (type `t` at `0x20 * (t + 0xd) +
    /// 2`: key 4 has type 0 and index 1, key 8 type 4 and index 2) and the functions of the
    /// indexes at `table_offset + 0x14 * index`. Returns the three objects.
    fn static_scene(e: &mut Engine, log: &Log, table_offset: u32) -> (Ptr, u32, u32) {
        let collection = collection(e, &[4, 6, 8], log);
        let first = entry_with(e, collection);
        let second_object = e.mem.alloc(0x20);
        e.mem.set_u32(second_object + 0xc, 2);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, second_object);
        let dispatcher = e.mem.alloc(0x1000);
        e.register_double(COLLECTION_CHILD, move |e, a| {
            let child = a[2] + 0x100;
            e.mem.set_u32(child + 0xc, a[1] - 4);
            Ret {
                eax: child,
                ..Ret::default()
            }
        });
        for (type_word, index) in [(0u32, 1u8), (4, 2)] {
            e.mem
                .set_u8(dispatcher + 0x20 * (type_word + 0xd) + 2, index);
        }
        for (index, target) in [(1u32, DISPATCHED), (2, DISPATCHED + 4)] {
            e.mem
                .set_u32(dispatcher + table_offset + 0x14 * index, target);
        }
        for target in [DISPATCHED, DISPATCHED + 4] {
            let sink = log.clone();
            e.register_double(target, move |e, a| {
                let mut words = vec![target];
                words.extend((0..4).map(|i| e.mem.u32(a[0] + 4 * i)));
                words.extend(&a[1..]);
                sink.borrow_mut().push(("dispatched", words));
                Ret::default()
            });
        }
        let filter_owner = e.mem.alloc(0x20);
        let filter_table = e.mem.alloc(0x20);
        e.mem.set_u32(filter_table + 4, FILTER_SLOT_4);
        e.mem.set_u32(filter_owner + 0xc, filter_table);
        let third = e.mem.alloc(0x20);
        e.mem.set_u32(third, dispatcher);
        e.mem.set_u32(third + 0xc, filter_owner);
        let sink = log.clone();
        e.register_double(FILTER_SLOT_4, move |e, a| {
            e.mem.set_u8(a[1], u8::from(a[6] != 6));
            sink.borrow_mut().push(("filter", a.to_vec()));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        (first, second, third)
    }

    #[test]
    fn the_static_closest_points_walk_dispatches_through_the_table_at_0x9a8() {
        let (mut e, stream, start) = profiling_engine(4);
        let log: Log = Rc::new(RefCell::new(vec![]));
        let (first, second, third) = static_scene(&mut e, &log, 0x9a8);
        // The flag byte at +4 of the fourth word does not stop this walk.
        let fourth = e.mem.alloc(8);
        e.mem.set_u8(fourth + 4, 1);
        e.call(0x00d4_3960, &args![first, second, third, fourth]);
        let dispatched = logged(&log, "dispatched");
        assert_eq!(dispatched.len(), 2);
        assert_eq!(dispatched[0][0], DISPATCHED);
        assert_eq!(dispatched[0][2..5], [4, 0xe8e8, first.addr()]);
        assert_eq!(dispatched[0][5..], [second, third, fourth]);
        assert_eq!(dispatched[1][0], DISPATCHED + 4);
        assert_eq!(dispatched[1][2], 8);
        let asked: Vec<u32> = logged(&log, "filter").iter().map(|call| call[6]).collect();
        assert_eq!(asked, vec![4, 6, 8]);
        assert_eq!(
            labels(&e, stream, start),
            vec![LABEL_SHAPE_COLLECTION, PROFILE_END]
        );
    }

    #[test]
    fn the_static_penetrations_walk_stops_when_the_flag_is_set_00d43dc0() {
        // Flag set before the walk: it ends after the first dispatched call.
        let (mut e, stream, start) = profiling_engine(4);
        let log: Log = Rc::new(RefCell::new(vec![]));
        let (first, second, third) = static_scene(&mut e, &log, 0x9a4);
        let fourth = e.mem.alloc(8);
        e.mem.set_u8(fourth + 4, 1);
        e.call(0x00d4_3dc0, &args![first, second, third, fourth]);
        let dispatched = logged(&log, "dispatched");
        assert_eq!(dispatched.len(), 1);
        assert_eq!(dispatched[0][0], DISPATCHED);
        assert_eq!(dispatched[0][2..5], [4, 0xe8e8, first.addr()]);
        assert_eq!(dispatched[0][5..], [second, third, fourth]);
        let asked: Vec<u32> = logged(&log, "filter").iter().map(|call| call[6]).collect();
        assert_eq!(asked, vec![4]);
        assert_eq!(
            labels(&e, stream, start),
            vec![LABEL_SHAPE_COLLECTION, PROFILE_END]
        );
        // Flag clear: every enabled key is dispatched.
        let (mut e, _, _) = profiling_engine(4);
        let log: Log = Rc::new(RefCell::new(vec![]));
        let (first, second, third) = static_scene(&mut e, &log, 0x9a4);
        let fourth = e.mem.alloc(8);
        e.call(0x00d4_3dc0, &args![first, second, third, fourth]);
        assert_eq!(logged(&log, "dispatched").len(), 2);
        let asked: Vec<u32> = logged(&log, "filter").iter().map(|call| call[6]).collect();
        assert_eq!(asked, vec![4, 6, 8]);
    }

    /// The scene of `updateShapeCollectionFilter`: keys 4, 6 and 8 (types 0, 2 and 4) of the
    /// collection of `body_a`; the filter enables the keys in `enabled`; `flag` is the word at
    /// `input + 0x14`. The dispatcher's index bytes are at the table the flag selects (the other
    /// table holds the index 3 as a decoy); the index of a key's type `t` is `t / 2 + 1` and its
    /// function answers the agent `0x6000 + index`. The pair array is `pairs` with room for
    /// `capacity` pairs and the word `0x7788` at `this + 8`; the memory router and the array
    /// grower are doubles. Returns `(this, body_a, body_b, input)`.
    fn update_scene(
        e: &mut Engine,
        log: &Log,
        keys: &'static [u32],
        flag: u32,
        enabled: &'static [u32],
        pairs: &[(u32, u32)],
        capacity: u32,
    ) -> (u32, Ptr, u32, u32) {
        let collection = collection(e, keys, log);
        let body_a = entry_with(e, collection);
        let selector_object = e.mem.alloc(0x20);
        e.mem.set_u32(selector_object + 0xc, 2);
        let body_b = e.mem.alloc(8);
        e.mem.set_u32(body_b, selector_object);
        e.register_double(COLLECTION_CHILD, move |e, a| {
            let child = a[2] + 0x100;
            e.mem.set_u32(child + 0xc, a[1] - 4);
            Ret {
                eax: child,
                ..Ret::default()
            }
        });
        let dispatcher = e.mem.alloc(0x1000);
        let (used, decoy) = if flag == 0 {
            (0x1a0, 0x5a0)
        } else {
            (0x5a0, 0x1a0)
        };
        for type_word in [0u32, 2, 4] {
            e.mem.set_u8(
                dispatcher + used + (type_word << 5) + 2,
                (type_word / 2 + 1) as u8,
            );
            e.mem.set_u8(dispatcher + decoy + (type_word << 5) + 2, 3);
        }
        for index in 1..=3u32 {
            let target = UPDATE_DISPATCHED + 4 * index;
            e.mem.set_u32(dispatcher + 0x9a0 + 0x14 * index, target);
            let sink = log.clone();
            e.register_double(target, move |e, a| {
                let mut words = vec![target];
                words.extend((0..4).map(|i| e.mem.u32(a[0] + 4 * i)));
                words.extend(&a[1..]);
                sink.borrow_mut().push(("dispatched", words));
                Ret {
                    eax: 0x6000 + index,
                    ..Ret::default()
                }
            });
        }
        let filter_owner = e.mem.alloc(0x20);
        let filter_table = e.mem.alloc(0x20);
        e.mem.set_u32(filter_table + 4, FILTER_SLOT_4);
        e.mem.set_u32(filter_owner + 0xc, filter_table);
        let input = e.mem.alloc(0x30);
        e.mem.set_u32(input, dispatcher);
        e.mem.set_u32(input + 0xc, filter_owner);
        e.mem.set_u32(input + 0x14, flag);
        let sink = log.clone();
        e.register_double(FILTER_SLOT_4, move |e, a| {
            e.mem.set_u8(a[1], u8::from(enabled.contains(&a[6])));
            sink.borrow_mut().push(("filter", a.to_vec()));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0xa110);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let grow_log = log.clone();
        e.register_double(RESERVE_MORE, move |e, a| {
            grow_log.borrow_mut().push(("grow", a.to_vec()));
            let data = e.mem.alloc(0x40);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        let this = agent_with_pairs(e, pairs, capacity);
        e.mem.set_u32(this + 8, 0x7788);
        (this, body_a, body_b, input)
    }

    /// An agent whose slots `0x1C` and `0x20` log as "cleanup" and "update".
    fn logging_agent(e: &mut Engine, log: &Log) -> u32 {
        for (kind, double) in [("cleanup", AGENT_SLOT_1C), ("update", AGENT_SLOT_20)] {
            let sink = log.clone();
            e.register_double(double, move |e, a| {
                let mut words = vec![a[0]];
                if kind == "update" {
                    words.extend((0..4).map(|i| e.mem.u32(a[1] + 4 * i)));
                    words.extend(&a[2..]);
                } else {
                    words.extend(&a[1..]);
                }
                sink.borrow_mut().push((kind, words));
                Ret::default()
            });
        }
        object(e, 8, &[(0x1c, AGENT_SLOT_1C), (0x20, AGENT_SLOT_20)])
    }

    #[test]
    fn update_filter_00d43f10_adds_the_pairs_of_accepted_keys_through_the_dispatcher() {
        for flag in [0u32, 1] {
            let mut e = Engine::new();
            let log: Log = Rc::new(RefCell::new(vec![]));
            let (this, body_a, body_b, input) =
                update_scene(&mut e, &log, &[4, 6, 8], flag, &[4, 8], &[], 0);
            e.call(0x00d4_3f10, &args![this, body_a, body_b, input, 0x99u32]);
            // The array grew once, by one element of 8 bytes.
            assert_eq!(logged(&log, "grow"), vec![vec![0xa110, this + 0xc, 8]]);
            let pairs = e.mem.u32(this + 0xc);
            assert_eq!(e.mem.u32(this + 0x10), 2);
            assert_eq!(
                [
                    e.mem.u32(pairs),
                    e.mem.u32(pairs + 4),
                    e.mem.u32(pairs + 8),
                    e.mem.u32(pairs + 12)
                ],
                [4, 0x6001, 8, 0x6003]
            );
            let dispatched = logged(&log, "dispatched");
            assert_eq!(dispatched.len(), 2);
            assert_eq!(dispatched[0][0], UPDATE_DISPATCHED + 4);
            assert_eq!(dispatched[0][2..5], [4, 0xe8e8, body_a.addr()]);
            assert_eq!(dispatched[0][5..], [body_b, input, 0x7788]);
            assert_eq!(dispatched[1][0], UPDATE_DISPATCHED + 12);
            assert_eq!(dispatched[1][2], 8);
            let asked: Vec<u32> = logged(&log, "filter").iter().map(|call| call[6]).collect();
            assert_eq!(asked, vec![4, 6, 8]);
        }
    }

    #[test]
    fn update_filter_00d43f10_removes_rejected_pairs_and_updates_accepted_ones() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(vec![]));
        let (agent_x, agent_y, agent_z) = (
            logging_agent(&mut e, &log),
            logging_agent(&mut e, &log),
            logging_agent(&mut e, &log),
        );
        // Only key 8 stays enabled; 4 and 6 are present and get dropped.
        let (this, body_a, body_b, input) = update_scene(
            &mut e,
            &log,
            &[4, 6, 8],
            0,
            &[8],
            &[(4, agent_x), (8, agent_y), (6, agent_z)],
            4,
        );
        e.call(0x00d4_3f10, &args![this, body_a, body_b, input, 0x99u32]);
        assert_eq!(
            logged(&log, "cleanup"),
            vec![vec![agent_x, 0x99], vec![agent_z, 0x99]]
        );
        // Key 8 updates its agent with (record, body_b, input, owner).
        let updates = logged(&log, "update");
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0][0], agent_y);
        assert_eq!(updates[0][2], 8);
        assert_eq!(
            updates[0][3..],
            [0xe8e8, body_a.addr(), body_b, input, 0x99]
        );
        assert!(logged(&log, "grow").is_empty());
        assert!(logged(&log, "dispatched").is_empty());
        // The last pair moved into each freed place: what is left is (8, agent_y).
        assert_eq!(e.mem.u32(this + 0x10), 1);
        let pairs = e.mem.u32(this + 0xc);
        assert_eq!([e.mem.u32(pairs), e.mem.u32(pairs + 4)], [8, agent_y]);
    }

    #[test]
    fn update_filter_00d43f10_drops_the_last_pair_without_moving_anything() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(vec![]));
        let (agent_x, agent_z) = (logging_agent(&mut e, &log), logging_agent(&mut e, &log));
        // Nothing is enabled: both present pairs go. The second is removed in place (it is the
        // last one by then).
        let (this, body_a, body_b, input) = update_scene(
            &mut e,
            &log,
            &[4, 6, 8],
            1,
            &[],
            &[(4, agent_x), (6, agent_z)],
            2,
        );
        e.call(0x00d4_3f10, &args![this, body_a, body_b, input, 0x77u32]);
        assert_eq!(
            logged(&log, "cleanup"),
            vec![vec![agent_x, 0x77], vec![agent_z, 0x77]]
        );
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert!(logged(&log, "update").is_empty());
        assert!(logged(&log, "dispatched").is_empty());
    }

    #[test]
    fn update_filter_00d43f10_with_no_keys_does_nothing() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(vec![]));
        let (this, body_a, body_b, input) = update_scene(&mut e, &log, &[], 0, &[], &[], 0);
        e.call(0x00d4_3f10, &args![this, body_a, body_b, input, 0x77u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }
}
