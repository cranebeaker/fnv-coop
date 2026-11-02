//! `Havok SDK/hkpbvtreeagent3.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions
//! in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkBvTreeAgent3` (the Xbox PDB has no record of the Havok classes, so the field names below
//! are unconfirmed): a collision agent between a shape and a bounding-volume tree. The agent
//! holds the two collidables at `+0`/`+4` (each: the shape at `+0`, a motion state at `+8`),
//! the input at `+0xC`, a transform at `+0x20` and a velocity vector at `+0x70`. SSE code
//! computes lane by lane in `f32`; the x87 code computes in double precision and rounds at each
//! `float` store. The profiling records `process` writes use the host clock for `RDTSC`.

use super::hkpboxmotion::{add, cross, mul, rows_times, sub};
use super::hkpsphereshape::{
    load_bits, load_vec, maxps, minps, store_vec, zip_with, Vec4, PROFILE_SLOT, SIGN_MASK,
    TLS_GET_VALUE,
};
#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its first word is a stack-like cursor the agent bumps by
/// 0x4000 bytes for its hit buffer.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkpAgentMachineUtil::calc1nStatistics` (`00d97250`, cdecl, three words),
/// `hkTransform::setInverse` (`00cb21e0`, `this` the output, one word),
/// `hkAgent1nMachine_Process` (`00da4aa0`, cdecl, five words) and the agent registration
/// (`00cfbd60`, `this` the dispatcher, the table, -1 and 8).
const CALC_1N_STATISTICS: u32 = 0x00d9_7250;
const TRANSFORM_SET_INVERSE: u32 = 0x00cb_21e0;
const AGENT_1N_PROCESS: u32 = 0x00da_4aa0;
const REGISTER_AGENT: u32 = 0x00cf_bd60;
/// The profiling labels `process` writes: `"LtBvTree3"` and `"StQueryTree"` (one record of
/// 16 bytes), `"StNarrow"` and the shared end label `"Et"` (12 bytes each).
const PROFILE_BEGIN_TREE: u32 = 0x010c_e7ec;
const PROFILE_QUERY_TREE: u32 = 0x010c_e7e0;
const PROFILE_NARROW: u32 = 0x010c_e7d4;
const PROFILE_END: u32 = 0x0107_6f38;
/// The name `calcStatistics` gives the collector (`"hkBvTreeAgent3"`).
const AGENT_NAME: u32 = 0x010c_e7c4;
/// The `double` 0.5 and the two `float` constants (0.4 and -2.0) of the cached box growth.
const HALF_F64: u32 = 0x0101_1588;
const GROWTH_FACTOR: u32 = 0x0102_31a8;
const SWEEP_FACTOR: u32 = 0x0104_a494;
/// The agent function table `00d2d9d0` fills: the vtable-like word at `+0`, the process
/// function and the other entries, as set by the exe.
const TABLE_FIRST: u32 = 0x00da_6fa0;
const TABLE_SECOND: u32 = 0x00da_6dc0;
const TABLE_PROCESS: u32 = 0x00d2_d890;
const TABLE_ENTRY_18: u32 = 0x00d2_d3e0;
const TABLE_ENTRY_1C: u32 = 0x00d2_d430;
const TABLE_ENTRY_20: u32 = 0x00d2_d460;
const TABLE_ENTRY_24: u32 = 0x00d2_d480;

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

// Translated from 00d2d430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBvTreeAgent3::calcStatistics` (engine map): cdecl, four words (the first never read).
/// Gives the collector (the fourth word) the name `"hkBvTreeAgent3"` (slot `0x18`), calls
/// `calc1nStatistics` with the second and third words and the collector, then jumps to the
/// collector's slot `0x1C` with no arguments.
pub fn hk_bv_tree_agent3_calc_statistics(
    e: &mut Engine,
    _unused: u32,
    second: u32,
    third: u32,
    collector: Ptr,
) {
    e.vcall(collector.addr(), 0x18, &args![AGENT_NAME]);
    e.call(CALC_1N_STATISTICS, &args![second, third, collector]);
    e.vcall(collector.addr(), 0x1c, &args![]);
}

// Translated from 00d2d4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A jump through the object's first word (no engine-map name; cdecl, two words, the object
/// and an output): `this` is the pointer stored at `[object]`; its vtable's slot `0x10` is
/// called with the same two stack words. Returns what that returns.
pub fn fn_00d2d4b0(e: &mut Engine, object: Ptr, output: u32) -> u32 {
    let inner = e.mem.u32(object.addr());
    e.vcall(inner, 0x10, &args![object, output]).u32()
}

// Translated from 00d2d4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks a pointer out of the shape behind `[collidable]` by its type at `+0xC` (no
/// engine-map name; cdecl, one word): type 8 with the byte 1 at `+0x10` gives the word at
/// `+0x18`, type 10 the word at `+0x34`, everything else 0.
pub fn fn_00d2d4c0(e: &mut Engine, collidable: Ptr) -> u32 {
    let shape = e.mem.u32(collidable.addr());
    match e.mem.u32(shape + 0xc) {
        8 if e.mem.u8(shape + 0x10) == 1 => e.mem.u32(shape + 0x18),
        10 => e.mem.u32(shape + 0x34),
        _ => 0,
    }
}

/// `hkAlgorithm::quickSortRecursive<unsigned int, less<unsigned int>>` core: Hoare
/// partition around the middle element (`(first + last) >> 1`, signed), recursing into the
/// left part and iterating on the right one.
fn quick_sort_u32(e: &mut Engine, array: u32, first: i32, last: i32, functor: u32) {
    let at = |index: i32| array.wrapping_add((index as u32).wrapping_mul(4));
    let mut first = first;
    loop {
        let mut low = first;
        let mut high = last;
        let pivot = e.mem.u32(at(low.wrapping_add(high) >> 1));
        loop {
            while e.mem.u32(at(low)) < pivot {
                low += 1;
            }
            while pivot < e.mem.u32(at(high)) {
                high -= 1;
            }
            if high < low {
                break;
            }
            if high != low {
                let at_low = e.mem.u32(at(low));
                let at_high = e.mem.u32(at(high));
                e.mem.set_u32(at(high), at_low);
                e.mem.set_u32(at(low), at_high);
            }
            high -= 1;
            low += 1;
            if low > high {
                break;
            }
        }
        if first < high {
            quick_sort_u32(e, array, first, high, functor);
        }
        if low < last {
            first = low;
        } else {
            return;
        }
    }
}

// Translated from 00d2da70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAlgorithm::quickSortRecursive<unsigned int, less<unsigned int>>` (engine map): cdecl;
/// the array, the first and last index (signed), and the empty comparison object (a word the
/// code never reads).
pub fn quick_sort_recursive_u32(e: &mut Engine, array: u32, first: i32, last: i32, functor: u32) {
    quick_sort_u32(e, array, first, last, functor);
}

// Translated from 00d2d4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBvTreeAgent3::calcAabbAndQueryTree` (engine map): cdecl, eight words: the two
/// collidables (`entity_a`, `entity_b`: shape at `+0`, motion state at `+8`), the inverse
/// transform, the velocity vector (`motion state B` rotation rows `+0`, `+0x10`, `+0x20`
/// carry it into B's frame), the input, an optional cached box (two vectors, or 0), the hit
/// buffer and its length. Computes the box of A's shape (its slot `0x1C` with the inverse
/// transform, a tolerance and a 32-byte output) in B's frame, widened by the swept motion when
/// `[[input + 0x60] + 0x10]` is not 0; returns -1 when the cached box already contains it;
/// otherwise grows the cached box (by 0.4 of the extents and -2 times the motion) and asks B's
/// shape (slot `0x34`) for the tree leaves in the box, falling back to the input owner's
/// function (slot 8 of the sub-object at `[input + 0xC] + 0xC`) when the buffer overflows, sorts
/// the hits as unsigned integers and ends the list with `0xffffffff`. Returns the number of
/// hits.
pub fn hk_bv_tree_agent3_calc_aabb_and_query_tree(
    e: &mut Engine,
    entity_a: Ptr,
    entity_b: Ptr,
    transform: u32,
    velocity: Ptr,
    input: Ptr,
    cached: Ptr,
    hits: u32,
    max_hits: i32,
) -> i32 {
    let a_state = e.mem.u32(entity_a.addr() + 8);
    let b_state = e.mem.u32(entity_b.addr() + 8);
    let rows = [
        load_vec(e, b_state),
        load_vec(e, b_state + 0x10),
        load_vec(e, b_state + 0x20),
    ];
    let carried = rows_times(load_vec(e, velocity.addr()), rows[0], rows[1], rows[2]);
    let input_address = input.addr();
    let sweeping = e.mem.u32(e.mem.u32(input_address + 0x60) + 0x10) != 0;
    let half: f64 = e.global(HALF_F64);
    let shape_a = e.mem.u32(entity_a.addr());
    e.with_stack(0x20, |e, bounds| {
        let bounds = bounds.addr();
        let (lower, upper, extents, motion) = if sweeping {
            let radius = x87_quiet(e.mem.f32(b_state + 0x9c));
            let mass = e.mem.f32(b_state + 0xa0);
            let own_term = (f64::from(radius) * f64::from(radius) * f64::from(mass)) as f32;
            let other_radius = x87_quiet(e.mem.f32(a_state + 0x9c));
            let other_mass = e.mem.f32(a_state + 0xa0);
            let combined =
                ((f64::from(other_radius) + f64::from(radius)) * f64::from(other_mass)) as f32;
            let margin = f64::from(x87_quiet(e.mem.f32(input_address + 8))) * half;
            let tolerance = ((f64::from(combined) + f64::from(own_term)) + margin) as f32;
            e.vcall(shape_a, 0x1c, &args![transform, tolerance, bounds]);
            let margin = f64::from(x87_quiet(e.mem.f32(input_address + 8))) * half;
            let growth =
                ((margin + f64::from(e.mem.f32(a_state + 0xa0))) + f64::from(own_term)) as f32;
            let difference = sub(load_vec(e, a_state + 0x50), load_vec(e, b_state + 0x30));
            let local = rows_times(difference, rows[0], rows[1], rows[2]);
            let box_lower = load_vec(e, bounds);
            let box_upper = load_vec(e, bounds + 0x10);
            let spread = [growth; 4];
            let widened_lower = zip_with(box_lower, sub(local, spread), maxps);
            let widened_upper = zip_with(box_upper, add(local, spread), minps);
            store_vec(e, bounds, widened_lower);
            store_vec(e, bounds + 0x10, widened_upper);
            let box_extents = sub(widened_upper, widened_lower);
            let total = if 0.0 < f64::from(radius) {
                let scale = (f64::from(x87_quiet(e.mem.f32(b_state + 0x5c)))
                    * f64::from(e.mem.f32(input_address + 0x58)))
                    as f32;
                let offset = sub(local, load_vec(e, b_state + 0x80));
                let spin = mul(cross(offset, load_vec(e, b_state + 0x90)), [scale; 4]);
                add(spin, carried)
            } else {
                carried
            };
            let zero = [0.0; 4];
            let below = zip_with(zero, total, minps);
            let above = zip_with(zero, total, maxps);
            let lower = add(below, widened_lower);
            let upper = add(above, widened_upper);
            store_vec(e, bounds, lower);
            store_vec(e, bounds + 0x10, upper);
            (lower, upper, box_extents, total)
        } else {
            let margin = f64::from(x87_quiet(e.mem.f32(input_address + 8))) * half;
            e.vcall(shape_a, 0x1c, &args![transform, margin as f32, bounds]);
            let lower = load_vec(e, bounds);
            let upper = load_vec(e, bounds + 0x10);
            (lower, upper, sub(upper, lower), carried)
        };
        if cached.addr() != 0 {
            let cached_lower = load_vec(e, cached.addr());
            let cached_upper = load_vec(e, cached.addr() + 0x10);
            let inside = (0..3)
                .all(|lane| cached_lower[lane] <= lower[lane] && upper[lane] <= cached_upper[lane]);
            if inside {
                return -1;
            }
            let margin = (f64::from(x87_quiet(e.mem.f32(input_address + 8))) * half) as f32;
            let growth_factor = x87_quiet(e.global::<f32>(GROWTH_FACTOR));
            let sweep_factor = x87_quiet(e.global::<f32>(SWEEP_FACTOR));
            let margin_vector = [margin; 4];
            let lowered = sub(lower, margin_vector);
            let raised = add(margin_vector, upper);
            let zero = [0.0; 4];
            let forward = mul([sweep_factor; 4], zip_with(zero, motion, maxps));
            let grown = mul([growth_factor; 4], extents);
            let sign = load_bits(e, SIGN_MASK);
            let negated: Vec4 =
                std::array::from_fn(|lane| f32::from_bits(grown[lane].to_bits() ^ sign[lane]));
            let backward = mul([sweep_factor; 4], zip_with(zero, motion, minps));
            let new_lower = add(zip_with(forward, negated, maxps), lowered);
            let new_upper = add(zip_with(backward, grown, minps), raised);
            store_vec(e, bounds, new_lower);
            store_vec(e, bounds + 0x10, new_upper);
            store_vec(e, cached.addr(), new_lower);
            store_vec(e, cached.addr() + 0x10, new_upper);
        }
        let shape_b = e.mem.u32(entity_b.addr());
        let mut count = e.vcall(shape_b, 0x34, &args![bounds, hits, max_hits]).u32() as i32;
        if count >= max_hits {
            let owner = e.mem.u32(input_address + 0xc);
            let shape_b = e.mem.u32(entity_b.addr());
            count = e
                .vcall(
                    owner + 0xc,
                    8,
                    &args![input, entity_a, entity_b, shape_b, bounds, hits, max_hits],
                )
                .u32() as i32;
            if count >= max_hits - 1 {
                count = max_hits - 1;
            }
        }
        if count > 1 {
            quick_sort_u32(e, hits, 0, count - 1, 0);
        }
        e.mem.set_u32(
            hits.wrapping_add((count as u32).wrapping_mul(4)),
            0xffff_ffff,
        );
        count
    })
}

// Translated from 00d2d890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBvTreeAgent3::process` (engine map): cdecl, five words (the agent, one never read, the
/// result block, one never read, the last passed on). Between profiling records
/// (`"LtBvTree3"` with `"StQueryTree"` in one 16-byte record, `"StNarrow"`, `"Et"`) it takes
/// 0x4000 bytes off the memory router's cursor for a hit buffer, makes the inverse of the
/// transform at `agent + 0x20` on the stack, queries the tree (`calcAabbAndQueryTree` with
/// the agent's two collidables, that transform, the velocity at `agent + 0x70`, the input at
/// `agent + 0xC`, no cache, the buffer and 0x1000 hits), gets `fn_00d2d4b0` of the second
/// collidable, runs the 1:N machine (`00da4aa0`: result block, agent, that value, the buffer
/// or 0 when the query returned a negative count, the last word) and puts the cursor back.
/// Returns the result block plus 0x10.
pub fn hk_bv_tree_agent3_process(
    e: &mut Engine,
    agent: Ptr,
    _unused_1: u32,
    result_block: u32,
    _unused_2: u32,
    last: u32,
) -> u32 {
    let agent = agent.addr();
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, PROFILE_BEGIN_TREE);
        e.mem.set_u32(cursor + 0xc, PROFILE_QUERY_TREE);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0x10);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let buffer = e.mem.u32(router);
    e.mem.set_u32(router, buffer.wrapping_add(0x4000));
    let (count, hit_source) = e.with_stack(0x40 + 0x210, |e, locals| {
        let transform = locals.addr();
        let tree_boxes = locals.addr() + 0x40;
        e.call(TRANSFORM_SET_INVERSE, &args![transform, agent + 0x20]);
        let first = e.mem.u32(agent);
        let second = e.mem.u32(agent + 4);
        let input = e.mem.u32(agent + 0xc);
        let count = hk_bv_tree_agent3_calc_aabb_and_query_tree(
            e,
            Ptr::new(first),
            Ptr::new(second),
            transform,
            Ptr::new(agent + 0x70),
            Ptr::new(input),
            Ptr::new(0),
            buffer,
            0x1000,
        );
        let source = fn_00d2d4b0(e, Ptr::new(second), tree_boxes);
        (count, source)
    });
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, PROFILE_NARROW);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0xc);
    }
    let passed = if count < 0 { 0 } else { buffer };
    e.call(
        AGENT_1N_PROCESS,
        &args![result_block, agent, hit_source, passed, last],
    );
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, PROFILE_END);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0xc);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, buffer);
    result_block.wrapping_add(0x10)
}

// Translated from 00d2d9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills an agent function table (no engine-map name; cdecl, one word): `+0` and `+4` the two
/// shared entries `00da6fa0` and `00da6dc0`, `+8` to `+0x14` and `+0x28` zero, `+0x18` to
/// `+0x24` the four agent functions, `+0x2C` the `process` function and the byte 1 at `+0x30`.
pub fn fn_00d2d9d0(e: &mut Engine, table: Ptr) {
    let table = table.addr();
    e.mem.set_u32(table, TABLE_FIRST);
    e.mem.set_u32(table + 0x2c, TABLE_PROCESS);
    e.mem.set_u32(table + 0x28, 0);
    for offset in [0x8, 0xc, 0x10, 0x14] {
        e.mem.set_u32(table + offset, 0);
    }
    e.mem.set_u32(table + 0x18, TABLE_ENTRY_18);
    e.mem.set_u32(table + 0x1c, TABLE_ENTRY_1C);
    e.mem.set_u32(table + 0x20, TABLE_ENTRY_20);
    e.mem.set_u32(table + 0x24, TABLE_ENTRY_24);
    e.mem.set_u32(table + 4, TABLE_SECOND);
    e.mem.set_u8(table + 0x30, 1);
}

// Translated from 00d2da20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the agent with the dispatcher (no engine-map name; cdecl, one word: the
/// dispatcher): builds the function table on the stack (`00d2d9d0` after clearing the bytes
/// `+0x30` to `+0x32` and the words `+0x18` to `+0x28`) and calls `00cfbd60` on the dispatcher
/// with the table, -1 and 8.
pub fn fn_00d2da20(e: &mut Engine, dispatcher: u32) {
    e.with_stack(0x40, |e, table| {
        for offset in [0x30, 0x31, 0x32] {
            e.mem.set_u8(table.addr() + offset, 0);
        }
        for offset in [0x18, 0x1c, 0x20, 0x24, 0x28] {
            e.mem.set_u32(table.addr() + offset, 0);
        }
        fn_00d2d9d0(e, table);
        e.call(REGISTER_AGENT, &args![dispatcher, table, u32::MAX, 8u32]);
    });
}

// Translated from 00d2d460 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: three stack words (cdecl, nothing popped), the first never read
/// (`_unused_0`). Calls `00da3510` with the second and third words and returns its answer (`EAX`).
/// Ghidra made no function here; the address is an immediate operand at 00d2da00.
pub fn fn_00d2d460(e: &mut Engine, _unused_0: u32, a: u32, b: u32) -> u32 {
    e.call(0x00da_3510, &args![a, b]).u32()
}
// Translated from 00d2d480 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: five stack words (cdecl, nothing popped), the first never read
/// (`_unused_0`). Calls `00da35f0` with the second word, the two floats (each passed through the
/// x87 stack: a signalling NaN is quieted) and the last word, and returns its answer (`EAX`).
/// Ghidra made no function here; the address is an immediate operand at 00d2da07.
pub fn fn_00d2d480(e: &mut Engine, _unused_0: u32, a: u32, first: f32, second: f32, b: u32) -> u32 {
    e.call(
        0x00da_35f0,
        &args![a, x87_quiet(first), x87_quiet(second), b],
    )
    .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d2_d430,
            hk_bv_tree_agent3_calc_statistics(u32, u32, u32, Ptr)
        ),
        entry!(0x00d2_d4b0, fn_00d2d4b0(Ptr, u32) -> u32),
        entry!(0x00d2_d4c0, fn_00d2d4c0(Ptr) -> u32),
        entry!(
            0x00d2_d4f0,
            hk_bv_tree_agent3_calc_aabb_and_query_tree(Ptr, Ptr, u32, Ptr, Ptr, Ptr, u32, i32) -> i32
        ),
        entry!(0x00d2_d890, hk_bv_tree_agent3_process(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00d2_d9d0, fn_00d2d9d0(Ptr)),
        entry!(0x00d2_da20, fn_00d2da20(u32)),
        entry!(0x00d2_da70, quick_sort_recursive_u32(u32, i32, i32, u32)),
        entry!(0x00d2_d460, fn_00d2d460(u32, u32, u32) -> u32),
        entry!(0x00d2_d480, fn_00d2d480(u32, u32, f32, f32, u32) -> u32),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const SLOT_AABB: u32 = 0x0900_0501;
    const SLOT_QUERY: u32 = 0x0900_0502;
    const SLOT_FALLBACK: u32 = 0x0900_0503;
    const SLOT_TREE: u32 = 0x0900_0504;
    const SLOT_NAME: u32 = 0x0900_0505;
    const SLOT_DONE: u32 = 0x0900_0506;

    struct Scene {
        e: Engine,
        entity_a: u32,
        entity_b: u32,
        input: u32,
        a_state: u32,
        b_state: u32,
        velocity: u32,
        hits: u32,
        calls: Log,
        seen: Rc<RefCell<Vec<f32>>>,
    }

    fn put_vec(e: &mut Engine, addr: u32, v: Vec4) {
        store_vec(e, addr, v);
    }

    fn vtable_with(e: &mut Engine, entries: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x40);
        for (offset, target) in entries {
            e.mem.set_u32(table + offset, *target);
        }
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, table);
        object
    }

    /// Two collidables whose shapes are doubles (A: box slot `0x1C` writing the box
    /// -1..1, -2..2, -3..3; B: query slot `0x34` listing `listed` and reporting `reported`,
    /// tree slot `0x10`), identity rotation rows in B's motion state, an input with the margin
    /// 2 (half of it is 1) and a sweep flag, and the exe constants the function reads.
    fn scene(sweeping: bool, listed: Vec<u32>, reported: i32) -> Scene {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(HALF_F64, 0.5f64);
        e.set_global(GROWTH_FACTOR, 0.4f32);
        e.set_global(SWEEP_FACTOR, -2.0f32);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        let calls: Log = Rc::default();
        let seen: Rc<RefCell<Vec<f32>>> = Rc::default();
        let sink = Rc::clone(&calls);
        e.register_double(SLOT_AABB, move |e, a| {
            sink.borrow_mut().push((SLOT_AABB, a.to_vec()));
            put_vec(e, a[3], [-1.0, -2.0, -3.0, 0.0]);
            put_vec(e, a[3] + 0x10, [1.0, 2.0, 3.0, 0.0]);
            Ret::default()
        });
        let record = Rc::clone(&seen);
        e.register_double(SLOT_QUERY, move |e, a| {
            let bounds = a[1];
            let mut values = load_vec(e, bounds).to_vec();
            values.extend(load_vec(e, bounds + 0x10));
            *record.borrow_mut() = values;
            for (index, hit) in listed.iter().enumerate() {
                e.mem.set_u32(a[2] + 4 * index as u32, *hit);
            }
            Ret {
                eax: reported as u32,
                ..Ret::default()
            }
        });
        e.register_double(SLOT_FALLBACK, move |e, a| {
            for (index, hit) in [30u32, 10, 20].iter().enumerate() {
                e.mem.set_u32(a[6] + 4 * index as u32, *hit);
            }
            Ret {
                eax: 10,
                ..Ret::default()
            }
        });
        e.register_double(SLOT_TREE, move |_, _| Ret {
            eax: 0x777,
            ..Ret::default()
        });
        let shape_a = vtable_with(&mut e, &[(0x1c, SLOT_AABB)]);
        let shape_b = vtable_with(&mut e, &[(0x34, SLOT_QUERY), (0x10, SLOT_TREE)]);
        let a_state = e.mem.alloc(0xc0);
        let b_state = e.mem.alloc(0xc0);
        put_vec(&mut e, b_state, [1.0, 0.0, 0.0, 0.0]);
        put_vec(&mut e, b_state + 0x10, [0.0, 1.0, 0.0, 0.0]);
        put_vec(&mut e, b_state + 0x20, [0.0, 0.0, 1.0, 0.0]);
        let entity_a = e.mem.alloc(0x10);
        e.mem.set_u32(entity_a, shape_a);
        e.mem.set_u32(entity_a + 8, a_state);
        let entity_b = e.mem.alloc(0x10);
        e.mem.set_u32(entity_b, shape_b);
        e.mem.set_u32(entity_b + 8, b_state);
        let flag = e.mem.alloc(0x20);
        e.mem.set_u32(flag + 0x10, u32::from(sweeping));
        let owner = e.mem.alloc(0x20);
        let fallback_table = vtable_with(&mut e, &[(8, SLOT_FALLBACK)]);
        e.mem.set_u32(owner + 0xc, e.mem.u32(fallback_table));
        let input = e.mem.alloc(0x80);
        e.mem.set_f32(input + 8, 2.0);
        e.mem.set_f32(input + 0x58, 3.0);
        e.mem.set_u32(input + 0xc, owner);
        e.mem.set_u32(input + 0x60, flag);
        let velocity = e.mem.alloc(0x10);
        put_vec(&mut e, velocity, [0.5, -1.0, 0.0, 0.0]);
        let hits = e.mem.alloc(0x40);
        Scene {
            e,
            entity_a,
            entity_b,
            input,
            a_state,
            b_state,
            velocity,
            hits,
            calls,
            seen,
        }
    }

    fn run(s: &mut Scene, cached: u32, max_hits: i32) -> i32 {
        hk_bv_tree_agent3_calc_aabb_and_query_tree(
            &mut s.e,
            Ptr::new(s.entity_a),
            Ptr::new(s.entity_b),
            0x1234,
            Ptr::new(s.velocity),
            Ptr::new(s.input),
            Ptr::new(cached),
            s.hits,
            max_hits,
        )
    }

    #[test]
    fn unswept_query_passes_the_tolerance_and_sorts_the_hits() {
        let mut s = scene(false, vec![9, 5], 2);
        assert_eq!(run(&mut s, 0, 8), 2);
        let calls = s.calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[1], 0x1234);
        assert_eq!(calls[0].1[2], 1.0f32.to_bits());
        assert_eq!(
            *s.seen.borrow(),
            vec![-1.0, -2.0, -3.0, 0.0, 1.0, 2.0, 3.0, 0.0]
        );
        assert_eq!(s.e.mem.u32(s.hits), 5);
        assert_eq!(s.e.mem.u32(s.hits + 4), 9);
        assert_eq!(s.e.mem.u32(s.hits + 8), 0xffff_ffff);
    }

    #[test]
    fn a_single_hit_is_not_sorted_but_still_terminated() {
        let mut s = scene(false, vec![7], 1);
        assert_eq!(run(&mut s, 0, 8), 1);
        assert_eq!(s.e.mem.u32(s.hits), 7);
        assert_eq!(s.e.mem.u32(s.hits + 4), 0xffff_ffff);
    }

    #[test]
    fn a_cached_box_that_contains_the_query_ends_with_minus_one() {
        let mut s = scene(false, vec![1], 1);
        let cached = s.e.mem.alloc(0x20);
        put_vec(&mut s.e, cached, [-5.0, -5.0, -5.0, 0.0]);
        put_vec(&mut s.e, cached + 0x10, [5.0, 5.0, 5.0, 0.0]);
        assert_eq!(run(&mut s, cached, 8), -1);
        assert!(s.seen.borrow().is_empty());
        assert_eq!(s.e.mem.u32(s.hits), 0);
    }

    #[test]
    fn a_smaller_cached_box_is_grown_and_queried() {
        let mut s = scene(false, vec![], 0);
        let cached = s.e.mem.alloc(0x20);
        put_vec(&mut s.e, cached, [0.0; 4]);
        put_vec(&mut s.e, cached + 0x10, [0.5, 0.5, 0.5, 0.0]);
        assert_eq!(run(&mut s, cached, 8), 0);
        let lower = [-(0.4f32 * 2.0) + -2.0, -3.0, -4.0];
        let upper = [2.0, 0.4f32 * 4.0 + 3.0, 4.0];
        let seen = s.seen.borrow();
        assert_eq!(seen[..3], lower);
        assert_eq!(seen[4..7], upper);
        assert_eq!(load_vec(&s.e, cached)[..3], lower);
        assert_eq!(load_vec(&s.e, cached + 0x10)[..3], upper);
        assert_eq!(s.e.mem.u32(s.hits), 0xffff_ffff);
    }

    #[test]
    fn a_swept_query_widens_the_box_by_the_motion() {
        let mut s = scene(true, vec![], 0);
        put_vec(&mut s.e, s.a_state + 0x50, [1.0, 1.0, 1.0, 0.0]);
        s.e.mem.set_f32(s.a_state + 0xa0, 1.0);
        assert_eq!(run(&mut s, 0, 8), 0);
        let calls = s.calls.borrow();
        assert_eq!(calls[0].1[2], 1.0f32.to_bits());
        let seen = s.seen.borrow();
        assert_eq!(seen[..3], [-1.0, -2.0, -1.0]);
        assert_eq!(seen[4..7], [1.5, 2.0, 3.0]);
    }

    #[test]
    fn a_spinning_body_adds_the_cross_term_to_the_motion() {
        let mut s = scene(true, vec![], 0);
        put_vec(&mut s.e, s.a_state + 0x50, [1.0, 1.0, 1.0, 0.0]);
        s.e.mem.set_f32(s.a_state + 0xa0, 1.0);
        s.e.mem.set_f32(s.b_state + 0x9c, 1.0);
        s.e.mem.set_f32(s.b_state + 0x5c, 2.0);
        put_vec(&mut s.e, s.b_state + 0x80, [1.0, 0.0, 0.0, 0.0]);
        put_vec(&mut s.e, s.b_state + 0x90, [0.0, 0.0, 1.0, 1.0]);
        assert_eq!(run(&mut s, 0, 8), 0);
        let calls = s.calls.borrow();
        assert_eq!(calls[0].1[2], 2.0f32.to_bits());
        let seen = s.seen.borrow();
        assert_eq!(seen[..3], [-1.0, -2.0, -1.0]);
        assert_eq!(seen[4..7], [7.5, 2.0, 3.0]);
    }

    #[test]
    fn a_full_hit_buffer_falls_back_to_the_owner_and_keeps_one_slot() {
        let mut s = scene(false, vec![1, 2, 3, 4], 4);
        assert_eq!(run(&mut s, 0, 4), 3);
        assert_eq!(s.e.mem.u32(s.hits), 10);
        assert_eq!(s.e.mem.u32(s.hits + 4), 20);
        assert_eq!(s.e.mem.u32(s.hits + 8), 30);
        assert_eq!(s.e.mem.u32(s.hits + 12), 0xffff_ffff);
    }

    #[test]
    fn quick_sort_orders_unsigned_values() {
        let mut e = Engine::new();
        let values = [
            5u32,
            3,
            9,
            1,
            7,
            3,
            0xffff_fff0,
            2,
            8,
            8,
            0,
            4,
            6,
            11,
            10,
            12,
            15,
            14,
            13,
        ];
        let array = e.mem.alloc(0x100);
        for (index, value) in values.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *value);
        }
        quick_sort_recursive_u32(&mut e, array, 0, values.len() as i32 - 1, 0);
        let mut expected = values;
        expected.sort_unstable();
        for (index, value) in expected.iter().enumerate() {
            assert_eq!(e.mem.u32(array + 4 * index as u32), *value);
        }
    }

    #[test]
    fn calc_statistics_names_the_collector_between_two_calls() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let sink = Rc::clone(&log);
        e.register_double(SLOT_NAME, move |_, a| {
            sink.borrow_mut().push((0x18, a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SLOT_DONE, move |_, a| {
            sink.borrow_mut().push((0x1c, a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(CALC_1N_STATISTICS, move |_, a| {
            sink.borrow_mut().push((CALC_1N_STATISTICS, a.to_vec()));
            Ret::default()
        });
        let collector = vtable_with(&mut e, &[(0x18, SLOT_NAME), (0x1c, SLOT_DONE)]);
        hk_bv_tree_agent3_calc_statistics(&mut e, 1, 2, 3, Ptr::new(collector));
        assert_eq!(
            *log.borrow(),
            vec![
                (0x18, vec![collector, AGENT_NAME]),
                (CALC_1N_STATISTICS, vec![2, 3, collector]),
                (0x1c, vec![collector]),
            ]
        );
    }

    #[test]
    fn the_tree_jump_goes_through_the_first_word_of_the_object() {
        let mut e = Engine::new();
        let seen: Log = Rc::default();
        let sink = Rc::clone(&seen);
        e.register_double(SLOT_TREE, move |_, a| {
            sink.borrow_mut().push((0, a.to_vec()));
            Ret {
                eax: 0x55,
                ..Ret::default()
            }
        });
        let inner = vtable_with(&mut e, &[(0x10, SLOT_TREE)]);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, inner);
        assert_eq!(fn_00d2d4b0(&mut e, Ptr::new(object), 0x99), 0x55);
        assert_eq!(*seen.borrow(), vec![(0, vec![inner, object, 0x99])]);
    }

    #[test]
    fn the_shape_pick_follows_the_shape_type() {
        let mut e = Engine::new();
        let shape = e.mem.alloc(0x40);
        let collidable = e.mem.alloc(0x10);
        e.mem.set_u32(collidable, shape);
        e.mem.set_u32(shape + 0x18, 0xaa);
        e.mem.set_u32(shape + 0x34, 0xbb);
        e.mem.set_u32(shape + 0xc, 8);
        e.mem.set_u8(shape + 0x10, 1);
        assert_eq!(fn_00d2d4c0(&mut e, Ptr::new(collidable)), 0xaa);
        e.mem.set_u8(shape + 0x10, 0);
        assert_eq!(fn_00d2d4c0(&mut e, Ptr::new(collidable)), 0);
        e.mem.set_u32(shape + 0xc, 10);
        assert_eq!(fn_00d2d4c0(&mut e, Ptr::new(collidable)), 0xbb);
        e.mem.set_u32(shape + 0xc, 3);
        assert_eq!(fn_00d2d4c0(&mut e, Ptr::new(collidable)), 0);
    }

    #[test]
    fn the_table_writer_fills_every_entry() {
        let mut e = Engine::new();
        let table = e.mem.alloc(0x40);
        for offset in (0..0x34).step_by(4) {
            e.mem.set_u32(table + offset, 0xdead_beef);
        }
        fn_00d2d9d0(&mut e, Ptr::new(table));
        let words: Vec<u32> = (0..0xcu32).map(|n| e.mem.u32(table + 4 * n)).collect();
        assert_eq!(
            words,
            vec![
                TABLE_FIRST,
                TABLE_SECOND,
                0,
                0,
                0,
                0,
                TABLE_ENTRY_18,
                TABLE_ENTRY_1C,
                TABLE_ENTRY_20,
                TABLE_ENTRY_24,
                0,
                TABLE_PROCESS
            ]
        );
        assert_eq!(e.mem.u8(table + 0x30), 1);
    }

    #[test]
    fn the_registration_builds_the_table_and_hands_it_to_the_dispatcher() {
        let mut e = Engine::new();
        let seen: Log = Rc::default();
        let sink = Rc::clone(&seen);
        e.register_double(REGISTER_AGENT, move |e, a| {
            let table = a[1];
            sink.borrow_mut().push((
                0,
                vec![
                    a[0],
                    a[2],
                    a[3],
                    e.mem.u32(table),
                    e.mem.u32(table + 0x2c),
                    e.mem.u32(table + 0x1c),
                    u32::from(e.mem.u8(table + 0x30)),
                ],
            ));
            Ret::default()
        });
        fn_00d2da20(&mut e, 0x4242);
        assert_eq!(
            *seen.borrow(),
            vec![(
                0,
                vec![
                    0x4242,
                    u32::MAX,
                    8,
                    TABLE_FIRST,
                    TABLE_PROCESS,
                    TABLE_ENTRY_1C,
                    1
                ]
            )]
        );
    }

    #[test]
    fn process_brackets_the_narrow_phase_with_profile_records_and_the_router() {
        let mut s = scene(false, vec![4, 2], 2);
        let records = s.e.mem.alloc(0x30);
        let stream = s.e.mem.alloc(0x10);
        s.e.mem.set_u32(stream + 4, records);
        s.e.mem.set_u32(stream + 0xc, records + 0x30);
        s.e.set_global(PROFILE_SLOT, 5u32);
        s.e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        let router = s.e.mem.alloc(0x10);
        let buffer = s.e.mem.alloc(0x20);
        s.e.mem.set_u32(router, buffer);
        s.e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let log: Log = Rc::default();
        let sink = Rc::clone(&log);
        s.e.register_double(TRANSFORM_SET_INVERSE, move |_, a| {
            sink.borrow_mut().push((TRANSFORM_SET_INVERSE, a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        s.e.register_double(AGENT_1N_PROCESS, move |e, a| {
            let mut words = a[..5].to_vec();
            words.push(e.mem.u32(router));
            sink.borrow_mut().push((AGENT_1N_PROCESS, words));
            Ret::default()
        });
        let agent = s.e.mem.alloc(0x100);
        s.e.mem.set_u32(agent, s.entity_a);
        s.e.mem.set_u32(agent + 4, s.entity_b);
        s.e.mem.set_u32(agent + 0xc, s.input);
        put_vec(&mut s.e, agent + 0x70, [0.0; 4]);
        let result = hk_bv_tree_agent3_process(&mut s.e, Ptr::new(agent), 0, 0x6000, 0, 0x7000);
        assert_eq!(result, 0x6010);
        assert_eq!(s.e.mem.u32(router), buffer);
        let log = log.borrow();
        assert_eq!(log[0].0, TRANSFORM_SET_INVERSE);
        assert_eq!(log[0].1[1], agent + 0x20);
        assert_eq!(
            log[1],
            (
                AGENT_1N_PROCESS,
                vec![0x6000, agent, 0x777, buffer, 0x7000, buffer + 0x4000]
            )
        );
        assert_eq!(s.e.mem.u32(records), PROFILE_BEGIN_TREE);
        assert_eq!(s.e.mem.u32(records + 0xc), PROFILE_QUERY_TREE);
        assert_eq!(s.e.mem.u32(records + 0x10), PROFILE_NARROW);
        assert_eq!(s.e.mem.u32(records + 0x1c), PROFILE_END);
        assert_eq!(s.e.mem.u32(stream + 4), records + 0x28);
        assert_eq!(s.e.mem.u32(buffer), 2);
        assert_eq!(s.e.mem.u32(buffer + 8), 0xffff_ffff);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    #[test]
    fn fn_00d2d460_forwards_the_second_and_third_words() {
        let mut e = Engine::new();
        e.register_double(0x00da_3510, |_, a| {
            assert_eq!(a, [0x22, 0x33]);
            Ret {
                eax: 0x44,
                ..Ret::default()
            }
        });
        assert_eq!(
            e.call(0x00d2_d460, &args![0x11u32, 0x22u32, 0x33u32]).u32(),
            0x44
        );
    }
    #[test]
    fn fn_00d2d480_forwards_the_words_and_quiets_the_floats() {
        let mut e = Engine::new();
        e.register_double(0x00da_35f0, |_, a| {
            assert_eq!(a, [0x22, 0x3fc0_0000, 0x7fc0_0001, 0x55]);
            Ret {
                eax: 0x66,
                ..Ret::default()
            }
        });
        let result = e
            .call(
                0x00d2_d480,
                &args![
                    0x11u32,
                    0x22u32,
                    1.5f32,
                    f32::from_bits(0x7f80_0001),
                    0x55u32
                ],
            )
            .u32();
        assert_eq!(result, 0x66);
    }
}
