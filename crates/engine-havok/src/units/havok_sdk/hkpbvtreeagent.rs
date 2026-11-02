//! `Havok SDK/hkpbvtreeagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue` import slot and the TLS index of Havok's monitor (timer) stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// The memory router (`00c85750`, no arguments): word +0x10 is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkTransform::setMulInverseMul` (Xbox PDB, `00cb22f0`, `this` = result, `(a, b)`).
const TRANSFORM_SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// A function (`004b4ab0`, `this` = a vector result, `(body motion, input + 0x50)`) that
/// produces the sweep displacement.
const SWEEP_DISPLACEMENT: u32 = 0x004b_4ab0;
/// Timer labels: `"LtBvTree"`, `"StQueryTree"`, `"StNarrowPhase"` and the end label.
const LABEL_BV_TREE: u32 = 0x010c_ea04;
const LABEL_QUERY_TREE: u32 = 0x010c_e7e0;
const LABEL_NARROW_PHASE: u32 = 0x010c_e9f4;
const LABEL_END: u32 = 0x0107_6f38;
/// `hkpNullAgent::getNullAgent` (`00d107a0`, no arguments).
const NULL_AGENT: u32 = 0x00d1_07a0;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkMemUtil::memMove` (`00d2cc70`, cdecl `(destination, source, bytes)`).
const MEM_MOVE: u32 = 0x00d2_cc70;
/// The unsigned word sort (`00d2da70`, cdecl, four words) and the out-of-memory state setter
/// (`00d88040`, cdecl, one word).
const QUICK_SORT: u32 = 0x00d2_da70;
const SET_OUT_OF_MEMORY_STATE: u32 = 0x00d8_8040;
/// `hkStatisticsCollector::getAllocatedSize` (`00d06f80`, cdecl `(block, bytes)`).
const STATISTICS_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// `hkpCollisionDispatcher` agent registration (`this` = dispatcher, `RET 0xC`): `(agent
/// function entry, type a, type b)`.
const REGISTER_AGENT: u32 = 0x00cf_bbe0;
/// The byte (`011b4284`) that turns on the cached box of the agent (`this + 0x20`).
const CACHED_BOX_FLAG: u32 = 0x011b_4284;
/// The byte (`01268ad4`) that selects the linear key merge over the sorted one.
const LINEAR_MERGE_FLAG: u32 = 0x0126_8ad4;
/// The `double` 0.5, the two `float` constants that grow the cached box and the 16-byte sign
/// mask.
const HALF_F64: u32 = 0x0101_1588;
const GROWTH_FACTOR: u32 = 0x0102_31a8;
const SWEEP_FACTOR: u32 = 0x0104_a494;
const SIGN_MASK: u32 = 0x010c_44d0;
/// The `float` that `00d2f980` broadcasts into the agent's two cached boxes, its vtable and
/// the two statistics labels (`"AgentPtrs"`, `"Agent"`).
const BOX_RESET_VALUE: u32 = 0x0104_da48;
const AGENT_VTABLE: u32 = 0x010c_e8f4;
const LABEL_AGENT_PTRS: u32 = 0x010c_e9ac;
const LABEL_AGENT: u32 = 0x010c_e9a4;

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the host
/// clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// The thread's monitor stream (`TlsGetValue` of the index at `0126835c`).
fn monitor_stream(e: &mut Engine) -> u32 {
    let index: u32 = e.global(MONITOR_SLOT);
    e.call(TLS_GET_VALUE, &args![index]).u32()
}

/// Appends a 12-byte timer record `(label, time stamp)` while the stream's cursor (+4) is
/// below its limit (+0xC).
fn monitor_record(e: &mut Engine, label: u32) {
    let stream = monitor_stream(e);
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0xc);
    }
}

/// Appends a 16-byte start record: `first` at +0, the time stamp at +4 and `second` at +0xC.
fn monitor_start_record(e: &mut Engine, first: u32, second: u32) {
    let stream = monitor_stream(e);
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, first);
        e.mem.set_u32(cursor + 0xc, second);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0x10);
    }
}

/// The four lanes of the vector at `addr`.
fn load4(e: &Engine, addr: u32) -> [f32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.f32(addr + offset))
}

/// `MINPS dest, src` per lane: `dest` when `dest < src`, otherwise `src`.
fn min_lane(dest: f32, src: f32) -> f32 {
    if dest < src {
        dest
    } else {
        src
    }
}

/// `MAXPS dest, src` per lane: `dest` when `dest > src`, otherwise `src`.
fn max_lane(dest: f32, src: f32) -> f32 {
    if dest > src {
        dest
    } else {
        src
    }
}

// Translated from 00d2f690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::calcAabbLinearCast` (Xbox PDB): asks the shape (`*first`, vtable slot
/// 0x1C) for the AABB of the cast, using the relative transform of the two bodies' motions
/// (`00cb22f0`) and the float at `input + 8`, writing it to the `aabb` (min at +0, max at
/// +0x10); then widens it by the sweep displacement `d` (`004b4ab0` with `input + 0x50`): the
/// minimum gains `min(0, d)` and the maximum `max(0, d)` in each of the four lanes.
pub fn hkp_bv_tree_agent_calc_aabb_linear_cast(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    aabb: Ptr,
) {
    let first_motion = e.mem.u32(first.addr() + 8);
    let second_motion = e.mem.u32(second.addr() + 8);
    let shape_owner = e.mem.u32(first.addr());
    e.with_stack(0x40, |e, transform| {
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![transform, second_motion, first_motion],
        );
        let distance = x87_quiet(e.mem.f32(input.addr() + 8));
        e.vcall(shape_owner, 0x1c, &args![transform, distance, aabb]);
    });
    e.with_stack(0x10, |e, displacement| {
        e.call(
            SWEEP_DISPLACEMENT,
            &args![displacement, second_motion, input.addr() + 0x50],
        );
        let moved = load4(e, displacement.addr());
        let lower = load4(e, aabb.addr());
        let upper = load4(e, aabb.addr() + 0x10);
        for lane in 0..4u32 {
            let d = moved[lane as usize];
            e.mem.set_f32(
                aabb.addr() + 4 * lane,
                min_lane(0.0, d) + lower[lane as usize],
            );
            e.mem.set_f32(
                aabb.addr() + 0x10 + 4 * lane,
                max_lane(0.0, d) + upper[lane as usize],
            );
        }
    });
}

// Translated from 00d30f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::staticLinearCast` (Xbox PDB): a timed cast of a body against a
/// bounding-volume tree. It computes the AABB of the cast, asks the tree shape (`*second`,
/// vtable slot 0x30) for the keys overlapping it into a stack array of 128 words, and for
/// each key gets the child shape (container slot 0x10 gives the container, whose slot 0x14
/// turns a key into a shape), builds a body record on the stack (the shape, the key, the
/// motion at `second + 8` and `second`) and calls the cast function found in the dispatcher
/// (`*input`) table at `0x9AC + 0x14 * type`, where the type is a byte of the shape's data at
/// `(*(*first + 0xC) + 0xD) * 0x20 + *input`. The array's heap storage, if it owns any, is
/// freed at the end. The parameter roles are described from their use.
pub fn hkp_bv_tree_agent_static_linear_cast(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    monitor_start_record(e, LABEL_BV_TREE, LABEL_QUERY_TREE);
    // The exe's stack frame: the AABB at +0x30, the key array at +0x50 (data, size,
    // capacity) with its inline storage at +0x5C, the body record at +0x20, and a scratch
    // buffer for child shapes at +0x260.
    e.with_stack(0x460, |e, frame| {
        let frame = frame.addr();
        hkp_bv_tree_agent_calc_aabb_linear_cast(e, first, second, input, Ptr::new(frame + 0x30));
        let shape_owner = e.mem.u32(second.addr());
        e.mem.set_u32(frame + 0x50, frame + 0x5c);
        e.mem.set_u32(frame + 0x54, 0);
        e.mem.set_u32(frame + 0x58, 0x8000_0080);
        e.vcall(shape_owner, 0x30, &args![frame + 0x30, frame + 0x50]);
        monitor_record(e, LABEL_NARROW_PHASE);
        let first_header = e.mem.u32(first.addr());
        let type_base = e.mem.u32(first_header + 0xc);
        let mut key_cursor = e.mem.u32(frame + 0x50);
        let key_end = key_cursor + e.mem.u32(frame + 0x54) * 4;
        e.mem.set_u32(frame + 0x28, e.mem.u32(second.addr() + 8));
        e.mem.set_u32(frame + 0x2c, second.addr());
        let container = e.vcall(shape_owner, 0x10, &args![]).u32();
        if key_cursor != key_end {
            let type_offset = type_base.wrapping_add(0xd) << 5;
            loop {
                let key = e.mem.u32(key_cursor);
                let shape = e.vcall(container, 0x14, &args![key, frame + 0x260]).u32();
                e.mem.set_u32(frame + 0x20, shape);
                e.mem.set_u32(frame + 0x24, e.mem.u32(key_cursor));
                let dispatcher = e.mem.u32(input.addr());
                let shape_data = e.mem.u32(shape + 0xc);
                let shape_type = e.mem.u8(shape_data
                    .wrapping_add(type_offset)
                    .wrapping_add(dispatcher));
                let function = e.mem.u32(dispatcher + 0x9ac + u32::from(shape_type) * 0x14);
                e.call(
                    function,
                    &args![first, frame + 0x20, input, cast_collector, start_collector],
                );
                key_cursor += 4;
                if key_cursor == key_end {
                    break;
                }
            }
        }
        monitor_record(e, LABEL_END);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(frame + 0x58);
        e.mem.set_u32(frame + 0x54, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(frame + 0x50);
            e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) * 4]);
        }
    });
}

// Translated from 00d315a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::linearCast` (Xbox PDB): forwards its five words to `staticLinearCast`
/// (`00d30f50`).
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hkp_bv_tree_agent_linear_cast(
    e: &mut Engine,
    _this: u32,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    hkp_bv_tree_agent_static_linear_cast(e, first, second, input, cast_collector, start_collector);
}

/// Four lanes of a vector register.
type Lanes = [f32; 4];

/// Stores the four lanes at `addr`.
fn store4(e: &mut Engine, addr: u32, lanes: Lanes) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// `ADDPS` per lane.
fn add4(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] + b[lane])
}

/// `SUBPS` per lane.
fn sub4(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] - b[lane])
}

/// `MULPS` per lane.
fn mul4(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] * b[lane])
}

/// `MAXPS dest, src` in all four lanes.
fn max4(dest: Lanes, src: Lanes) -> Lanes {
    std::array::from_fn(|lane| max_lane(dest[lane], src[lane]))
}

/// `MINPS dest, src` in all four lanes.
fn min4(dest: Lanes, src: Lanes) -> Lanes {
    std::array::from_fn(|lane| min_lane(dest[lane], src[lane]))
}

/// The compiler's matrix-times-vector sequence on the three rows of a motion state: lane `i`
/// (0 to 2) is `(row_i.y * v.y + v.x * row_i.x) + v.z * row_i.z`; lane 3 uses the third row's
/// `w`, `y` and `w` (`(row_2.w * v.y + v.x * row_2.y) + v.z * row_2.w`), as the shuffles of the
/// exe leave it.
fn rows_times(rows: [Lanes; 3], v: Lanes) -> Lanes {
    let [r0, r1, r2] = rows;
    [
        (r0[1] * v[1] + v[0] * r0[0]) + v[2] * r0[2],
        (r1[1] * v[1] + v[0] * r1[0]) + v[2] * r1[2],
        (r2[1] * v[1] + v[0] * r2[0]) + v[2] * r2[2],
        (r2[3] * v[1] + v[0] * r2[1]) + v[2] * r2[3],
    ]
}

/// The three rows (16-byte vectors) at the start of a motion state.
fn rows_of(e: &Engine, state: u32) -> [Lanes; 3] {
    [
        load4(e, state),
        load4(e, state + 0x10),
        load4(e, state + 0x20),
    ]
}

/// What the agent functions share: the two bodies, the dispatcher input, the shape container
/// of the second body and a scratch block of 0x220 bytes in the exe's frame: the filter's
/// answer byte at +0, the 16-byte body record `(shape, key, motion, second)` at +0x10 and the
/// buffer for one child shape at +0x20.
struct AgentContext {
    first: Ptr,
    second: Ptr,
    input: Ptr,
    container: u32,
    scratch: u32,
}

/// Gets the child shape for `key` from the container (slot 0x14 with the scratch buffer) and
/// writes the body record `(shape, key, second's motion, second)`; returns the shape.
fn child_record(e: &mut Engine, ctx: &AgentContext, key: u32) -> u32 {
    let shape = e
        .vcall(ctx.container, 0x14, &args![key, ctx.scratch + 0x20])
        .u32();
    let record = ctx.scratch + 0x10;
    e.mem.set_u32(record, shape);
    e.mem.set_u32(record + 4, key);
    e.mem.set_u32(record + 8, e.mem.u32(ctx.second.addr() + 8));
    e.mem.set_u32(record + 0xc, ctx.second.addr());
    shape
}

/// Asks the filter (the object at `*(input + 0xC) + 0xC`, vtable slot 4, with the scratch answer
/// byte, the input, both bodies, the container and the key) whether the pair collides; the
/// answer is the byte the function returns a pointer to.
fn filter_enabled(e: &mut Engine, ctx: &AgentContext, key: u32) -> bool {
    let filter = e.mem.u32(ctx.input.addr() + 0xc) + 0xc;
    let answer = e
        .vcall(
            filter,
            4,
            &args![
                ctx.scratch,
                ctx.input,
                ctx.first,
                ctx.second,
                ctx.container,
                key
            ],
        )
        .u32();
    e.mem.u8(answer) != 0
}

/// Calls the agent create function the dispatcher (`*input`) holds at `0x9A0 + 0x14 * kind`,
/// `kind` being the byte at `(first's shape word +0xC) * 0x20 + table + shape data (+0xC)`
/// where the table is at `+0x5A0` of the dispatcher when `input + 0x14` is non-zero and at
/// `+0x1A0` otherwise. It receives `(first, body record, input, owner word)`.
fn create_from_dispatcher(e: &mut Engine, ctx: &AgentContext, shape: u32, owner_word: u32) -> u32 {
    let input = ctx.input.addr();
    let dispatcher = e.mem.u32(input);
    let first_shape_word = e.mem.u32(e.mem.u32(ctx.first.addr()) + 0xc);
    let table = dispatcher
        + if e.mem.u32(input + 0x14) != 0 {
            0x5a0
        } else {
            0x1a0
        };
    let shape_data = e.mem.u32(shape + 0xc);
    let kind = e.mem.u8((first_shape_word << 5)
        .wrapping_add(table)
        .wrapping_add(shape_data));
    let function = e.mem.u32(dispatcher + 0x9a0 + u32::from(kind) * 0x14);
    e.call(
        function,
        &args![ctx.first, ctx.scratch + 0x10, ctx.input, owner_word],
    )
    .u32()
}

/// The agent for one key: the child record, then the filter; the create function when the
/// filter allows the pair, the null agent otherwise.
fn make_agent(e: &mut Engine, ctx: &AgentContext, key: u32, owner_word: u32) -> u32 {
    let shape = child_record(e, ctx, key);
    if filter_enabled(e, ctx, key) {
        create_from_dispatcher(e, ctx, shape, owner_word)
    } else {
        e.call(NULL_AGENT, &args![]).u32()
    }
}

/// Frees the heap storage of a key array (data +0, size +4, capacity +8) the exe built on its
/// stack: the size is cleared, and the data freed through the allocator's slot 0x10 (4 bytes a
/// key) when the capacity word's top bit says the storage is on the heap.
fn free_key_array(e: &mut Engine, keys: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(keys + 8);
    e.mem.set_u32(keys + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(keys);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
}

/// The first half of the linear merge of the agent entries (12 bytes each: key, a word, agent)
/// of `this` (array at `+0xC`) against the sorted keys: an entry whose key is not found in the
/// keys has its agent destroyed (slot 0x1C with `destroy_word`, no null test) and is removed
/// by shifting the later entries down a word at a time.
fn remove_unmatched_agents(e: &mut Engine, this: u32, destroy_word: u32, keys: u32) {
    let keys_begin = e.mem.u32(keys);
    let keys_end = keys_begin.wrapping_add(e.mem.u32(keys + 4).wrapping_mul(4));
    let mut entry = e.mem.u32(this + 0xc);
    let mut entries_end = entry.wrapping_add(e.mem.u32(this + 0x10).wrapping_mul(12));
    let mut cursor = keys_begin;
    while entry != entries_end {
        if cursor != keys_end && e.mem.u32(entry) == e.mem.u32(cursor) {
            cursor += 4;
        } else {
            let wanted = e.mem.u32(entry);
            cursor = keys_begin;
            let mut found = false;
            while cursor != keys_end {
                if e.mem.u32(cursor) == wanted {
                    cursor += 4;
                    found = true;
                    break;
                }
                cursor += 4;
            }
            if !found {
                let agent = e.mem.u32(entry + 8);
                e.vcall(agent, 0x1c, &args![destroy_word]);
                let data = e.mem.u32(this + 0xc);
                let count = e.mem.u32(this + 0x10).wrapping_sub(1);
                e.mem.set_u32(this + 0x10, count);
                let index = (entry.wrapping_sub(data) as i32).wrapping_div(12);
                let bytes = count.wrapping_sub(index as u32).wrapping_mul(12) as i32;
                let destination = data.wrapping_add((index as u32).wrapping_mul(12));
                if bytes > 0 {
                    let words = ((bytes as u32 - 1) >> 2) + 1;
                    for word in 0..words {
                        let value = e.mem.u32(destination + 12 + 4 * word);
                        e.mem.set_u32(destination + 4 * word, value);
                    }
                }
                entry = entry.wrapping_sub(12);
                entries_end = entries_end.wrapping_sub(12);
            }
        }
        entry = entry.wrapping_add(12);
    }
}

/// The second half of the linear merge: when the entry count differs from the key count, each
/// key without an entry at its position gets one inserted there (the array grows through
/// `_reserve` when its capacity is short, the tail moves up by `memMove`) holding the key and
/// the agent made for it.
fn insert_missing_agents(
    e: &mut Engine,
    this: u32,
    ctx: &AgentContext,
    owner_word: u32,
    keys: u32,
) {
    let keys_begin = e.mem.u32(keys);
    let key_count = e.mem.u32(keys + 4);
    let count = e.mem.u32(this + 0x10);
    if key_count == count {
        return;
    }
    let mut slot = e.mem.u32(this + 0xc);
    let mut entries_end = slot.wrapping_add(count.wrapping_mul(12));
    let keys_end = keys_begin.wrapping_add(key_count.wrapping_mul(4));
    let mut key_cursor = keys_begin;
    if key_cursor == keys_end {
        return;
    }
    loop {
        if slot == entries_end || e.mem.u32(slot) != e.mem.u32(key_cursor) {
            let index = (key_cursor.wrapping_sub(keys_begin) as i32 >> 2) as u32;
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let old_count = e.mem.u32(this + 0x10);
            let new_count = old_count.wrapping_add(1);
            let moved = old_count.wrapping_sub(index);
            let capacity = e.mem.u32(this + 0x14) & 0x3fff_ffff;
            if (capacity as i32) < (new_count as i32) {
                let doubled = capacity.wrapping_add(capacity);
                let wanted = if (new_count as i32) < (doubled as i32) {
                    doubled
                } else {
                    new_count
                };
                e.call(ARRAY_RESERVE, &args![allocator, this + 0xc, wanted, 12u32]);
            }
            let source = e.mem.u32(this + 0xc).wrapping_add(index.wrapping_mul(12));
            e.call(
                MEM_MOVE,
                &args![source.wrapping_add(12), source, moved.wrapping_mul(12)],
            );
            e.mem.set_u32(this + 0x10, new_count);
            slot = e.mem.u32(this + 0xc).wrapping_add(index.wrapping_mul(12));
            let key = e.mem.u32(key_cursor);
            let agent = make_agent(e, ctx, key, owner_word);
            e.mem.set_u32(slot + 8, agent);
            e.mem.set_u32(slot, e.mem.u32(key_cursor));
            entries_end = e
                .mem
                .u32(this + 0xc)
                .wrapping_add(e.mem.u32(this + 0x10).wrapping_mul(12));
        }
        key_cursor += 4;
        slot = slot.wrapping_add(12);
        if key_cursor == keys_end {
            break;
        }
    }
}

/// The merge of the agent entries against the sorted keys when the linear merge flag is off:
/// the keys are sorted (`quickSort` when there is more than one), then walked together with the
/// entries into a temporary array of entries taken from the memory router's stack (a key
/// equal to an entry's keeps the entry, a smaller key gets a new agent, a larger one destroys
/// the entry's agent when it has one; keys left over get new agents, entries left over are
/// destroyed), and the temporary array is copied back into the entry array of `this`.
fn merge_sorted_agents(
    e: &mut Engine,
    this: u32,
    ctx: &AgentContext,
    destroy_word: u32,
    owner_word: u32,
    keys: u32,
) {
    let key_count = e.mem.u32(keys + 4);
    if (key_count as i32) > 1 {
        // The fourth word of the sort is the stack residue of a byte store (an empty
        // comparison object); the sort never reads it, so 0 is passed.
        let keys_data = e.mem.u32(keys);
        e.call(
            QUICK_SORT,
            &args![keys_data, 0u32, key_count.wrapping_sub(1), 0u32],
        );
    }
    let key_count = e.mem.u32(keys + 4);
    let entries_begin = e.mem.u32(this + 0xc);
    let entries_end = entries_begin.wrapping_add(e.mem.u32(this + 0x10).wrapping_mul(12));
    let keys_begin = e.mem.u32(keys);
    let keys_end = keys_begin.wrapping_add(key_count.wrapping_mul(4));
    // The temporary array: data, size and capacity words.
    e.with_stack(12, |e, temp| {
        let temp = temp.addr();
        e.mem.set_u32(temp + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let bump = e.mem.u32(router);
        e.mem.set_u32(
            router,
            (key_count.wrapping_mul(12).wrapping_add(0x7f) & 0xffff_ff80).wrapping_add(bump),
        );
        e.mem.set_u32(temp + 8, key_count | 0x8000_0000);
        let key_count = e.mem.u32(keys + 4);
        e.mem.set_u32(temp, bump);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(temp + 8) & 0x3fff_ffff;
        if (capacity as i32) < (key_count as i32) {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if (key_count as i32) < (doubled as i32) {
                doubled
            } else {
                key_count
            };
            e.call(ARRAY_RESERVE, &args![allocator, temp, wanted, 12u32]);
        }
        e.mem.set_u32(temp + 4, key_count);
        let mut out = e.mem.u32(temp);
        let mut old = entries_begin;
        let mut key_cursor = keys_begin;
        if old != entries_end {
            loop {
                if key_cursor == keys_end {
                    // The keys are used up: the remaining entries' agents are destroyed.
                    while old != entries_end {
                        let agent = e.mem.u32(old + 8);
                        if agent != 0 {
                            e.vcall(agent, 0x1c, &args![destroy_word]);
                        }
                        old = old.wrapping_add(12);
                    }
                    break;
                }
                let key = e.mem.u32(key_cursor);
                let old_key = e.mem.u32(old);
                if key == old_key {
                    e.mem.set_u32(out, old_key);
                    e.mem.set_u32(out + 4, e.mem.u32(old + 4));
                    e.mem.set_u32(out + 8, e.mem.u32(old + 8));
                    out = out.wrapping_add(12);
                    old = old.wrapping_add(12);
                    key_cursor += 4;
                } else if key < old_key {
                    let agent = make_agent(e, ctx, key, owner_word);
                    e.mem.set_u32(out + 8, agent);
                    e.mem.set_u32(out, e.mem.u32(key_cursor));
                    out = out.wrapping_add(12);
                    key_cursor += 4;
                } else {
                    let agent = e.mem.u32(old + 8);
                    if agent != 0 {
                        e.vcall(agent, 0x1c, &args![destroy_word]);
                    }
                    old = old.wrapping_add(12);
                }
                if old == entries_end {
                    break;
                }
            }
        }
        while key_cursor != keys_end {
            let key = e.mem.u32(key_cursor);
            let agent = make_agent(e, ctx, key, owner_word);
            e.mem.set_u32(out + 8, agent);
            e.mem.set_u32(out, e.mem.u32(key_cursor));
            key_cursor += 4;
            out = out.wrapping_add(12);
        }

        // Copy the temporary array back into the agent entries.
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let new_count = e.mem.u32(temp + 4);
        let old_count = e.mem.u32(this + 0x10);
        let kept = if (new_count as i32) > (old_count as i32) {
            old_count
        } else {
            new_count
        };
        let capacity = e.mem.u32(this + 0x14) & 0x3fff_ffff;
        if (capacity as i32) < (new_count as i32) {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if (new_count as i32) >= (doubled as i32) {
                new_count
            } else {
                doubled
            };
            e.call(ARRAY_RESERVE, &args![allocator, this + 0xc, wanted, 12u32]);
        }
        let data = e.mem.u32(this + 0xc);
        let temp_data = e.mem.u32(temp);
        if (kept as i32) > 0 {
            for index in 0..kept {
                for word in 0..3u32 {
                    let value = e.mem.u32(temp_data.wrapping_add(index * 12 + 4 * word));
                    e.mem
                        .set_u32(data.wrapping_add(index * 12 + 4 * word), value);
                }
            }
        }
        let mut source = e.mem.u32(temp).wrapping_add(kept.wrapping_mul(12));
        let mut destination = e.mem.u32(this + 0xc).wrapping_add(kept.wrapping_mul(12));
        let mut remaining = (new_count as i32).wrapping_sub(kept as i32);
        while remaining > 0 {
            // The compiler's placement-new test of the destination.
            if destination != 0 {
                for word in 0..3u32 {
                    let value = e.mem.u32(source + 4 * word);
                    e.mem.set_u32(destination + 4 * word, value);
                }
            }
            source = source.wrapping_add(12);
            destination = destination.wrapping_add(12);
            remaining -= 1;
        }
        e.mem.set_u32(this + 0x10, new_count);
        if bump == e.mem.u32(temp) {
            e.mem.set_u32(temp + 4, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, bump);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(temp + 8);
        e.mem.set_u32(temp + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(temp);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(12)],
            );
        }
    });
}

/// Brings the agent entries of `this` in line with the sorted-or-not keys the second body's
/// shape returned: the linear merge (removal, then insertion) when the byte at `01268ad4` is
/// set, the sorted merge otherwise. `owner_word` is the word at `this + 8`, read before.
fn reconcile_agents(e: &mut Engine, this: u32, ctx: &AgentContext, destroy_word: u32, keys: u32) {
    let owner_word = e.mem.u32(this + 8);
    if e.mem.u8(LINEAR_MERGE_FLAG) != 0 {
        remove_unmatched_agents(e, this, destroy_word, keys);
        insert_missing_agents(e, this, ctx, owner_word, keys);
    } else {
        merge_sorted_agents(e, this, ctx, destroy_word, owner_word, keys);
    }
}

// Translated from 00d2f550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::cleanup` (Xbox PDB): calls slot 0x1C of every child agent in the entry
/// array (12 bytes each, array at `+0xC`, agent at `+8`) that has one with the argument, then
/// destroys the agent itself (slot 0 with the delete flag 1).
pub fn hkp_bv_tree_agent_cleanup(e: &mut Engine, this: Ptr, param: u32) {
    let mut entry = e.mem.u32(this.addr() + 0xc);
    let end = entry.wrapping_add(e.mem.u32(this.addr() + 0x10).wrapping_mul(12));
    while entry != end {
        let agent = e.mem.u32(entry + 8);
        if agent != 0 {
            e.vcall(agent, 0x1c, &args![param]);
        }
        entry = entry.wrapping_add(12);
    }
    e.vcall(this.addr(), 0, &args![1u32]);
}

// Translated from 00d2f5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::invalidateTim` (Xbox PDB): calls slot 0x24 of every child agent in the
/// entry array that has one with the argument.
pub fn hkp_bv_tree_agent_invalidate_tim(e: &mut Engine, this: Ptr, param: u32) {
    let mut entry = e.mem.u32(this.addr() + 0xc);
    let end = entry.wrapping_add(e.mem.u32(this.addr() + 0x10).wrapping_mul(12));
    while entry != end {
        let agent = e.mem.u32(entry + 8);
        if agent != 0 {
            e.vcall(agent, 0x24, &args![param]);
        }
        entry = entry.wrapping_add(12);
    }
}

// Translated from 00d2f5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::warpTime` (Xbox PDB): calls slot 0x28 of every child agent in the entry
/// array that has one with the two times (floats, copied through `FLD`/`FSTP`) and the third
/// word.
pub fn hkp_bv_tree_agent_warp_time(
    e: &mut Engine,
    this: Ptr,
    old_time: f32,
    new_time: f32,
    third: u32,
) {
    let mut entry = e.mem.u32(this.addr() + 0xc);
    let end = entry.wrapping_add(e.mem.u32(this.addr() + 0x10).wrapping_mul(12));
    while entry != end {
        let agent = e.mem.u32(entry + 8);
        if agent != 0 {
            e.vcall(
                agent,
                0x28,
                &args![x87_quiet(old_time), x87_quiet(new_time), third],
            );
        }
        entry = entry.wrapping_add(12);
    }
}

// Translated from 00d2f710 (decompiled, FalloutNV.exe 1.4.0.525)
/// The AABB of the first body's shape against the second's (the map gives the function no
/// name; cdecl, four words): the relative transform of the two bodies' motions (`00cb22f0`
/// with the second's and the first's), then slot 0x1C of `*first` with the transform, the float
/// at `input + 8` and the `aabb` to fill. It is the first half of `00d2f690` without the
/// widening by the sweep.
pub fn fn_00d2f710(e: &mut Engine, first: Ptr, second: Ptr, input: Ptr, aabb: Ptr) {
    let first_motion = e.mem.u32(first.addr() + 8);
    let second_motion = e.mem.u32(second.addr() + 8);
    e.with_stack(0x40, |e, transform| {
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![transform, second_motion, first_motion],
        );
        let shape_owner = e.mem.u32(first.addr());
        let distance = x87_quiet(e.mem.f32(input.addr() + 8));
        e.vcall(shape_owner, 0x1c, &args![transform, distance, aabb]);
    });
}

// Translated from 00d2f760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::updateShapeCollectionFilter` (Xbox PDB): for every entry of the agent array
/// (key at +0, agent at +8) it takes the child shape for the key from the second body's
/// shape container (slot 0x10 gives the container, its slot 0x14 the shape), builds the body
/// record `(shape, key, second's motion, second)` and asks the input's filter whether the pair
/// collides. When it does and the entry holds the null agent (`00d107a0`), the dispatcher's
/// create function makes one (`kind` as in `create_from_dispatcher`, called with `(first,
/// record, input, this + 8)`); when it does and the entry holds a real agent, that agent's slot
/// 0x20 is called with `(first, record, input, destroy_word)`. When the filter refuses and the
/// entry holds a real agent, its slot 0x1C is called with `destroy_word` and the entry is set
/// to the null agent. The parameter roles are described from their use.
pub fn hkp_bv_tree_agent_update_shape_collection_filter(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    destroy_word: u32,
) {
    let this = this.addr();
    let second_owner = e.mem.u32(second.addr());
    let container = e.vcall(second_owner, 0x10, &args![]).u32();
    if (e.mem.u32(this + 0x10) as i32) <= 0 {
        return;
    }
    e.with_stack(0x220, |e, scratch| {
        let ctx = AgentContext {
            first,
            second,
            input,
            container,
            scratch: scratch.addr(),
        };
        let mut index = 0i32;
        let mut offset = 0u32;
        loop {
            let key = e.mem.u32(e.mem.u32(this + 0xc).wrapping_add(offset));
            let shape = child_record(e, &ctx, key);
            let key = e.mem.u32(e.mem.u32(this + 0xc).wrapping_add(offset));
            let enabled = filter_enabled(e, &ctx, key);
            let entry = e.mem.u32(this + 0xc).wrapping_add(offset);
            let null_agent = e.call(NULL_AGENT, &args![]).u32();
            if enabled {
                if e.mem.u32(entry + 8) == null_agent {
                    let owner_word = e.mem.u32(this + 8);
                    let agent = create_from_dispatcher(e, &ctx, shape, owner_word);
                    let entries = e.mem.u32(this + 0xc);
                    e.mem.set_u32(entries.wrapping_add(offset) + 8, agent);
                } else {
                    let entries = e.mem.u32(this + 0xc);
                    let agent = e.mem.u32(entries.wrapping_add(offset) + 8);
                    e.vcall(
                        agent,
                        0x20,
                        &args![first, scratch.addr() + 0x10, input, destroy_word],
                    );
                }
            } else if e.mem.u32(entry + 8) != null_agent {
                let entries = e.mem.u32(this + 0xc);
                let agent = e.mem.u32(entries.wrapping_add(offset) + 8);
                e.vcall(agent, 0x1c, &args![destroy_word]);
                let entry = e.mem.u32(this + 0xc).wrapping_add(offset);
                let null_agent = e.call(NULL_AGENT, &args![]).u32();
                e.mem.set_u32(entry + 8, null_agent);
            }
            index += 1;
            offset = offset.wrapping_add(12);
            if index >= e.mem.u32(this + 0x10) as i32 {
                break;
            }
        }
    });
}

// Translated from 00d2f8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::calcContentStatistics` (Xbox PDB): reports the agent array to the
/// collector (slot 0x14 with 3, `"AgentPtrs"`, the data, the used bytes, and the allocated size
/// from `00d06f80`) when the array's storage is on the heap (capacity word non-negative) or the
/// collector's byte at +0x10 is set, then reports every child agent that exists (collector slot
/// 0xC with `"Agent"`, the agent, 0). The second stack word is never read.
pub fn hkp_bv_tree_agent_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    _unused_2: u32,
) {
    let this = this.addr();
    let capacity = e.mem.u32(this + 0x14);
    if (capacity as i32) >= 0 || e.mem.u8(collector.addr() + 0x10) != 0 {
        let data = e.mem.u32(this + 0xc);
        let allocated = e
            .call(
                STATISTICS_ALLOCATED_SIZE,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(12)],
            )
            .u32();
        let data = e.mem.u32(this + 0xc);
        let used = e.mem.u32(this + 0x10).wrapping_mul(12);
        e.vcall(
            collector.addr(),
            0x14,
            &args![3u32, LABEL_AGENT_PTRS, data, used, allocated],
        );
    }
    let mut entry = e.mem.u32(this + 0xc);
    let end = entry.wrapping_add(e.mem.u32(this + 0x10).wrapping_mul(12));
    while entry != end {
        let agent = e.mem.u32(entry + 8);
        if agent != 0 {
            e.vcall(collector.addr(), 0xc, &args![LABEL_AGENT, agent, 0u32]);
        }
        entry = entry.wrapping_add(12);
    }
}

// Translated from 00d2f980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises the fields of an `hkpBvTreeAgent` (vtable `010ce8f4`, `.?AVhkpBvTreeAgent@@` by
/// its RTTI; the map gives the function no name): the half-word at +6 set to 1, the word at +8
/// to `owner`, the vtable, the entry array at +0xC emptied (data 0, size 0, capacity word
/// `0x80000000`) and the float at `0104da48` broadcast into the two 16-byte cached boxes at
/// +0x20 and +0x30. Returns `this` (the exe leaves it in `EAX`).
pub fn fn_00d2f980(e: &mut Engine, this: Ptr, owner: u32) -> Ptr {
    let this_addr = this.addr();
    e.mem.set_u16(this_addr + 6, 1);
    e.mem.set_u32(this_addr + 8, owner);
    e.mem.set_u32(this_addr, AGENT_VTABLE);
    e.mem.set_u32(this_addr + 0xc, 0);
    e.mem.set_u32(this_addr + 0x10, 0);
    e.mem.set_u32(this_addr + 0x14, 0x8000_0000);
    let value = e.mem.u32(BOX_RESET_VALUE);
    for lane in 0..4 {
        e.mem.set_u32(this_addr + 0x30 + 4 * lane, value);
    }
    for lane in 0..4 {
        e.mem.set_u32(this_addr + 0x20 + 4 * lane, value);
    }
    this
}

// Translated from 00d2fac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::prepareCollisionPartners` (Xbox PDB): brings the agent's entry array in
/// line with the shape keys overlapping the first body's box. It computes the box of the first
/// body's shape in the second's frame (relative transform `00cb22f0`, then slot 0x1C of `*first`
/// with a tolerance of `(input[2] * 0.5 + input[2])` and the box); when the cached box flag
/// (`011b4284`) is set and the cached box (`this + 0x20`, `+0x30`) already holds the new box in
/// the first three lanes the function ends, otherwise the new box becomes the cached one. The
/// second body's shape (slot 0x30) then fills a stack array of keys (inline storage for 0x80);
/// the memory router's allocator (slot 0x24) is asked whether `(keys - entries) << 7` bytes are
/// available, and if not the out-of-memory state is set (`00d88040`) and nothing more is done.
/// Otherwise the entries are merged with the keys (`reconcile_agents`: removing agents of
/// vanished keys with `destroy_word`, making agents for new ones). The array's heap storage is
/// freed at the end. The parameter roles are described from their use.
pub fn hkp_bv_tree_agent_prepare_collision_partners(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    destroy_word: u32,
) {
    let this = this.addr();
    let first_motion = e.mem.u32(first.addr() + 8);
    let second_motion = e.mem.u32(second.addr() + 8);
    e.with_stack(0x4d0, |e, frame| {
        let frame = frame.addr();
        let transform = frame + 0x490;
        let aabb = frame + 0x40;
        let keys = frame + 0x280;
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![transform, second_motion, first_motion],
        );
        let half: f64 = e.global(HALF_F64);
        let distance = e.mem.f32(input.addr() + 8);
        let scaled = (f64::from(x87_quiet(distance)) * half) as f32;
        let tolerance = (f64::from(scaled) + f64::from(distance)) as f32;
        let first_owner = e.mem.u32(first.addr());
        e.vcall(
            first_owner,
            0x1c,
            &args![transform, x87_quiet(tolerance), aabb],
        );
        if e.mem.u8(CACHED_BOX_FLAG) != 0 {
            let lower = load4(e, aabb);
            let upper = load4(e, aabb + 0x10);
            let cached_lower = load4(e, this + 0x20);
            let cached_upper = load4(e, this + 0x30);
            let inside = (0..3)
                .all(|lane| upper[lane] <= cached_upper[lane] && cached_lower[lane] <= lower[lane]);
            if inside {
                return;
            }
            store4(e, this + 0x20, lower);
            store4(e, this + 0x30, upper);
        }
        let second_owner = e.mem.u32(second.addr());
        e.mem.set_u32(keys, keys + 0xc);
        e.mem.set_u32(keys + 4, 0);
        e.mem.set_u32(keys + 8, 0x8000_0080);
        e.vcall(second_owner, 0x30, &args![aabb, keys]);
        let entry_count = e.mem.u32(this + 0x10);
        let key_count = e.mem.u32(keys + 4);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let available = e
            .vcall(
                allocator,
                0x24,
                &args![key_count.wrapping_sub(entry_count) << 7],
            )
            .bool();
        if !available {
            e.call(SET_OUT_OF_MEMORY_STATE, &args![1u32]);
        } else {
            let container = e.vcall(second_owner, 0x10, &args![]).u32();
            e.with_stack(0x220, |e, scratch| {
                let ctx = AgentContext {
                    first,
                    second,
                    input,
                    container,
                    scratch: scratch.addr(),
                };
                reconcile_agents(e, this, &ctx, destroy_word, keys);
            });
        }
        free_key_array(e, keys);
    });
}

// Translated from 00d302d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::processCollision` (Xbox PDB): a timed step that refreshes the agent's
/// entries and then lets each child agent process its pair. After the start record (labels
/// `"LtBvTree"`, `"StQueryTree"`) it computes, in the second body's frame, the relative
/// transform (`00cb22f0`), the sweep of the two motion states over the step `input[0x58]` and
/// the box of the first body's shape (slot 0x1C of `*first`, with a tolerance that is wider
/// when `[*(input + 0x60) + 0x10]` is non-zero); when the cached box flag (`011b4284`) is set
/// and the cached box (`this + 0x20`, `+0x30`) holds the box in the first three lanes the query
/// is skipped, otherwise the box is grown (by 0.4 of its extents and -2 times the sweep,
/// constants read from the exe's data) and stored as the cached one, the second body's shape
/// (slot 0x30) fills a stack array of keys and the entries are brought in line with them
/// (`reconcile_agents`, with the word at `output + 4` for destroyed agents). Then, with the
/// label `"StNarrowPhase"`, slot 0x18 of every entry's agent is called with `(first, body
/// record, input, output)`, and the end label is recorded.
pub fn hkp_bv_tree_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    output: Ptr,
) {
    monitor_start_record(e, LABEL_BV_TREE, LABEL_QUERY_TREE);
    let this = this.addr();
    let input_address = input.addr();
    let destroy_word = e.mem.u32(output.addr() + 4);
    e.with_stack(0x700, |e, frame| {
        let frame = frame.addr();
        let keys = frame + 0x2e0;
        let transform = frame + 0x80;
        let aabb = frame + 0x60;
        e.mem.set_u32(keys, frame + 0x2ec);
        e.mem.set_u32(keys + 4, 0);
        e.mem.set_u32(keys + 8, 0x8000_0080);
        let state_a = e.mem.u32(first.addr() + 8);
        let state_b = e.mem.u32(second.addr() + 8);
        e.call(
            TRANSFORM_SET_MUL_INVERSE_MUL,
            &args![transform, state_b, state_a],
        );
        let step = x87_quiet(e.mem.f32(input_address + 0x58));
        let a_scale = (f64::from(x87_quiet(e.mem.f32(state_a + 0x5c))) * f64::from(step)) as f32;
        let b_scale = (f64::from(step) * f64::from(e.mem.f32(state_b + 0x5c))) as f32;
        let a_offset = sub4(load4(e, state_a + 0x40), load4(e, state_a + 0x50));
        let b_offset = sub4(load4(e, state_b + 0x50), load4(e, state_b + 0x40));
        let mut moved = add4(mul4(a_offset, [a_scale; 4]), mul4(b_offset, [b_scale; 4]));
        moved[3] = (f64::from(x87_quiet(e.mem.f32(state_b + 0x9c)))
            * f64::from(b_scale)
            * f64::from(e.mem.f32(state_b + 0xa0))
            + f64::from(x87_quiet(e.mem.f32(state_a + 0x9c)))
                * f64::from(a_scale)
                * f64::from(e.mem.f32(state_a + 0xa0))) as f32;
        let cached = if e.mem.u8(CACHED_BOX_FLAG) != 0 {
            Some(this + 0x20)
        } else {
            None
        };
        let rows = rows_of(e, state_b);
        let carried = rows_times(rows, moved);
        let first_owner = e.mem.u32(first.addr());
        let half: f64 = e.global(HALF_F64);
        let sweeping = e.mem.u32(e.mem.u32(input_address + 0x60) + 0x10) != 0;
        let (lower, upper, extents, motion) = if sweeping {
            let radius = x87_quiet(e.mem.f32(state_b + 0x9c));
            let own_term = (f64::from(radius)
                * f64::from(radius)
                * f64::from(e.mem.f32(state_b + 0xa0))) as f32;
            let combined = ((f64::from(x87_quiet(e.mem.f32(state_a + 0x9c)))
                + f64::from(e.mem.f32(state_b + 0x9c)))
                * f64::from(e.mem.f32(state_a + 0xa0))) as f32;
            let margin = f64::from(x87_quiet(e.mem.f32(input_address + 8))) * half;
            let tolerance = ((f64::from(combined) + f64::from(own_term)) + margin) as f32;
            e.vcall(
                first_owner,
                0x1c,
                &args![transform, x87_quiet(tolerance), aabb],
            );
            let margin = f64::from(x87_quiet(e.mem.f32(input_address + 8))) * half;
            let growth =
                ((margin + f64::from(e.mem.f32(state_a + 0xa0))) + f64::from(own_term)) as f32;
            let difference = sub4(load4(e, state_a + 0x50), load4(e, state_b + 0x30));
            let local = rows_times(rows, difference);
            let box_lower = load4(e, aabb);
            let box_upper = load4(e, aabb + 0x10);
            let spread = [growth; 4];
            let widened_lower = max4(box_lower, sub4(local, spread));
            let widened_upper = min4(box_upper, add4(local, spread));
            store4(e, aabb, widened_lower);
            store4(e, aabb + 0x10, widened_upper);
            let box_extents = sub4(widened_upper, widened_lower);
            let total = if 0.0 < e.mem.f32(state_b + 0x9c) {
                let scale = (f64::from(x87_quiet(e.mem.f32(state_b + 0x5c)))
                    * f64::from(e.mem.f32(input_address + 0x58)))
                    as f32;
                let offset = sub4(local, load4(e, state_b + 0x80));
                let spin = load4(e, state_b + 0x90);
                let turned = [
                    spin[2] * offset[1] - spin[1] * offset[2],
                    spin[0] * offset[2] - spin[2] * offset[0],
                    spin[1] * offset[0] - spin[0] * offset[1],
                    spin[3] * offset[3] - spin[3] * offset[3],
                ];
                add4(mul4(turned, [scale; 4]), carried)
            } else {
                carried
            };
            let zero = [0.0; 4];
            let lower = add4(min4(zero, total), widened_lower);
            let upper = add4(max4(zero, total), widened_upper);
            store4(e, aabb, lower);
            store4(e, aabb + 0x10, upper);
            (lower, upper, box_extents, total)
        } else {
            let margin = f64::from(x87_quiet(e.mem.f32(input_address + 8))) * half;
            e.vcall(
                first_owner,
                0x1c,
                &args![transform, x87_quiet(margin as f32), aabb],
            );
            let lower = load4(e, aabb);
            let upper = load4(e, aabb + 0x10);
            (lower, upper, sub4(upper, lower), carried)
        };

        let mut query = true;
        if let Some(cached) = cached {
            let cached_lower = load4(e, cached);
            let cached_upper = load4(e, cached + 0x10);
            let inside = (0..3)
                .all(|lane| cached_lower[lane] <= lower[lane] && upper[lane] <= cached_upper[lane]);
            if inside {
                query = false;
            } else {
                let margin = (f64::from(x87_quiet(e.mem.f32(input_address + 8))) * half) as f32;
                let growth_factor = x87_quiet(e.global::<f32>(GROWTH_FACTOR));
                let sweep_factor = x87_quiet(e.global::<f32>(SWEEP_FACTOR));
                let lowered = sub4(lower, [margin; 4]);
                let raised = add4([margin; 4], upper);
                let zero = [0.0; 4];
                let forward = mul4([sweep_factor; 4], max4(zero, motion));
                let grown = mul4([growth_factor; 4], extents);
                let sign: [u32; 4] =
                    std::array::from_fn(|lane| e.mem.u32(SIGN_MASK + 4 * lane as u32));
                let negated: Lanes =
                    std::array::from_fn(|lane| f32::from_bits(grown[lane].to_bits() ^ sign[lane]));
                let backward = mul4([sweep_factor; 4], min4(zero, motion));
                let new_lower = add4(max4(forward, negated), lowered);
                let new_upper = add4(min4(backward, grown), raised);
                store4(e, aabb, new_lower);
                store4(e, aabb + 0x10, new_upper);
                store4(e, cached, new_lower);
                store4(e, cached + 0x10, new_upper);
            }
        }
        let second_owner = e.mem.u32(second.addr());
        if query {
            e.vcall(second_owner, 0x30, &args![aabb, keys]);
            let container = e.vcall(second_owner, 0x10, &args![]).u32();
            e.with_stack(0x220, |e, scratch| {
                let ctx = AgentContext {
                    first,
                    second,
                    input,
                    container,
                    scratch: scratch.addr(),
                };
                reconcile_agents(e, this, &ctx, destroy_word, keys);
            });
        }
        free_key_array(e, keys);

        // The narrow phase: every entry's agent processes its pair.
        let entries = e.mem.u32(this + 0xc);
        let entries_end = entries.wrapping_add(e.mem.u32(this + 0x10).wrapping_mul(12));
        let record = frame + 0x48;
        e.mem.set_u32(record + 8, state_b);
        e.mem.set_u32(record + 0xc, second.addr());
        monitor_record(e, LABEL_NARROW_PHASE);
        let container = e.vcall(second_owner, 0x10, &args![]).u32();
        let mut entry = entries;
        while entry != entries_end {
            let key = e.mem.u32(entry);
            let shape = e.vcall(container, 0x14, &args![key, frame + 0x4f0]).u32();
            e.mem.set_u32(record, shape);
            e.mem.set_u32(record + 4, e.mem.u32(entry));
            let agent = e.mem.u32(entry + 8);
            e.vcall(agent, 0x18, &args![first, record, input, output]);
            entry = entry.wrapping_add(12);
        }
        monitor_record(e, LABEL_END);
    });
}

// Translated from 00d31110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::staticGetClosestPoints` (Xbox PDB; cdecl, four words): a timed query of the
/// closest points of a body against a bounding-volume tree. After the start record it builds
/// the box (`00d2f710`), asks the tree shape (`*second`, slot 0x30) for the keys overlapping it
/// into a stack array, and for each key whose filter answer (the object at `*(input + 0xC) + 0xC`,
/// slot 4) allows it takes the child shape (slot 0x14 of the container from slot 0x10) and calls
/// the function the dispatcher (`*input`) holds at `0x9A8 + 0x14 * kind` with `(first, body
/// record, input, collector)`; `kind` is the byte at `((first's shape word at +0xC) + 0xD) * 0x20
/// + shape data (+0xC) + dispatcher`. The array's heap storage is freed at the end.
pub fn hkp_bv_tree_agent_static_get_closest_points(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    collector: u32,
) {
    static_tree_query(e, first, second, input, collector, 0x9a8, false);
}

// Translated from 00d312f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::staticGetPenetrations` (Xbox PDB; cdecl, four words): as
/// `staticGetClosestPoints` (`00d31110`) with the function at `0x9A4 + 0x14 * kind`, and the loop
/// ends as soon as the byte at `collector + 4` is non-zero after a call.
pub fn hkp_bv_tree_agent_static_get_penetrations(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    static_tree_query(e, first, second, input, collector.addr(), 0x9a4, true);
}

/// The body of `staticGetClosestPoints` and `staticGetPenetrations`, which differ in the
/// dispatcher table offset and in the stop test on the collector's byte at +4.
fn static_tree_query(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    collector: u32,
    table_offset: u32,
    stop_on_flag: bool,
) {
    monitor_start_record(e, LABEL_BV_TREE, LABEL_QUERY_TREE);
    // The exe's frame: the AABB at +0x30, the key array at +0x50 (data, size, capacity) with
    // its inline storage at +0x5C, the body record at +0x20, the filter's answer byte at +0x1B
    // and a scratch buffer for child shapes at +0x260.
    e.with_stack(0x460, |e, frame| {
        let frame = frame.addr();
        fn_00d2f710(e, first, second, input, Ptr::new(frame + 0x30));
        let second_owner = e.mem.u32(second.addr());
        e.mem.set_u32(frame + 0x50, frame + 0x5c);
        e.mem.set_u32(frame + 0x54, 0);
        e.mem.set_u32(frame + 0x58, 0x8000_0080);
        e.vcall(second_owner, 0x30, &args![frame + 0x30, frame + 0x50]);
        monitor_record(e, LABEL_NARROW_PHASE);
        let first_shape_word = e.mem.u32(e.mem.u32(first.addr()) + 0xc);
        let mut cursor = e.mem.u32(frame + 0x50);
        let end = cursor.wrapping_add(e.mem.u32(frame + 0x54).wrapping_mul(4));
        e.mem.set_u32(frame + 0x28, e.mem.u32(second.addr() + 8));
        e.mem.set_u32(frame + 0x2c, second.addr());
        let container = e.vcall(second_owner, 0x10, &args![]).u32();
        while cursor != end {
            let key = e.mem.u32(cursor);
            let filter = e.mem.u32(input.addr() + 0xc) + 0xc;
            let answer = e
                .vcall(
                    filter,
                    4,
                    &args![frame + 0x1b, input, first, second, container, key],
                )
                .u32();
            if e.mem.u8(answer) != 0 {
                let shape = e.vcall(container, 0x14, &args![key, frame + 0x260]).u32();
                e.mem.set_u32(frame + 0x20, shape);
                e.mem.set_u32(frame + 0x24, e.mem.u32(cursor));
                let dispatcher = e.mem.u32(input.addr());
                let kind = e.mem.u8((first_shape_word.wrapping_add(0xd) << 5)
                    .wrapping_add(e.mem.u32(shape + 0xc))
                    .wrapping_add(dispatcher));
                let function = e
                    .mem
                    .u32(dispatcher + table_offset + u32::from(kind) * 0x14);
                e.call(function, &args![first, frame + 0x20, input, collector]);
                if stop_on_flag && e.mem.u8(collector + 4) != 0 {
                    break;
                }
            }
            cursor = cursor.wrapping_add(4);
        }
        monitor_record(e, LABEL_END);
        free_key_array(e, frame + 0x50);
    });
}

// Translated from 00d314e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the tree agent's functions with a dispatcher (the map gives the function no name;
/// cdecl, one word): three entries through `00cfbbe0` (`this` = the dispatcher, `(entry, type
/// a, type b)`), all built in the same 20-byte block on the stack. The first is the symmetric
/// one (create function `00d2fa00`, the flip functions `00d2e970` and `00d2e9b0`, the symmetric
/// cast `00d2e9f0`; both flag bytes 1) for the types (8, -1); the second and third are direct
/// (create functions `00d2f9d0` and `00d2fa40`, the static queries `00d312f0`, `00d31110`,
/// `00d30f50`; flag bytes 0 and 1) for the types (-1, 8) and (8, 8). The block's bytes +0x12
/// and +0x13 are not set by the exe (the callee copies the word they are in); here they are
/// zero.
pub fn fn_00d314e0(e: &mut Engine, dispatcher: Ptr) {
    e.with_stack(0x14, |e, entry| {
        let entry = entry.addr();
        for (words, flag_a, type_a, type_b) in [
            (
                [0x00d2_fa00, 0x00d2_e970, 0x00d2_e9b0, 0x00d2_e9f0],
                1u8,
                8u32,
                0xffff_ffff_u32,
            ),
            (
                [0x00d2_f9d0, 0x00d3_12f0, 0x00d3_1110, 0x00d3_0f50],
                0,
                0xffff_ffff,
                8,
            ),
            (
                [0x00d2_fa40, 0x00d3_12f0, 0x00d3_1110, 0x00d3_0f50],
                0,
                8,
                8,
            ),
        ] {
            for (index, word) in words.into_iter().enumerate() {
                e.mem.set_u32(entry + 4 * index as u32, word);
            }
            e.mem.set_u8(entry + 0x10, flag_a);
            e.mem.set_u8(entry + 0x11, 1);
            e.call(REGISTER_AGENT, &args![dispatcher, entry, type_a, type_b]);
        }
    });
}

// Translated from 00d315d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::getClosestPoints` (Xbox PDB): hands its four stack words on to the static
/// query `00d31110`; `this` is not used.
pub fn hkp_bv_tree_agent_get_closest_points(
    e: &mut Engine,
    _unused_this: Ptr,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    collector: u32,
) {
    hkp_bv_tree_agent_static_get_closest_points(e, first, second, input, collector);
}

// Translated from 00d315f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvTreeAgent::getPenetrations` (Xbox PDB): hands its four stack words on to the static
/// query `00d312f0`; `this` is not used.
pub fn hkp_bv_tree_agent_get_penetrations(
    e: &mut Engine,
    _unused_this: Ptr,
    first: Ptr,
    second: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    hkp_bv_tree_agent_static_get_penetrations(e, first, second, input, collector);
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

// Translated from 00d2f9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words of which only the last, `owner`, is read): a
/// create function of the bounding-volume tree pair (code at `00d2f30f` stores its address in an
/// agent record). Allocates the 0x40-byte agent (`allocate_agent_block`) and runs the
/// `hkpBvTreeAgent` constructor `00d2f980` on it with `owner`. Answers the agent.
pub fn fn_00d2f9d0(
    e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    owner: u32,
) -> u32 {
    let agent = allocate_agent_block(e, 0x40);
    fn_00d2f980(e, Ptr::new(agent), owner).addr()
}

// Translated from 00d2fa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words of which only the last, `owner`, is read): the
/// create function of the flipped bounding-volume tree agent (code at `00d2f2d6` stores its
/// address in an agent record). Allocates the 0x40-byte agent (`allocate_agent_block`), runs the
/// constructor `00d2f980` with `owner` and stores the vtable `010ce9bc`. Answers the agent.
pub fn fn_00d2fa00(
    e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    owner: u32,
) -> u32 {
    let agent = allocate_agent_block(e, 0x40);
    fn_00d2f980(e, Ptr::new(agent), owner);
    e.mem.set_u32(agent, 0x010c_e9bc);
    agent
}

// Translated from 00d2fa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `body_a`, `body_b`, `_unused_2`, `owner`; code at
/// `00d31567` stores its address in an agent record): chooses between the two create functions
/// by the float at `+0xa0` of the object each body's word at `+8` points to. When `body_b`'s is
/// above `body_a`'s (`FCOMPP`, a NaN counts as not above) it makes the agent as `00d2f9d0` does;
/// otherwise as `00d2fa00` does (the flipped agent, vtable `010ce9bc`). Answers the agent.
pub fn fn_00d2fa40(e: &mut Engine, body_a: Ptr, body_b: Ptr, _unused_2: u32, owner: u32) -> u32 {
    let motion_a = e.mem.u32(body_a.addr().wrapping_add(8));
    let first = x87_load32(e.mem.u32(motion_a.wrapping_add(0xa0)));
    let motion_b = e.mem.u32(body_b.addr().wrapping_add(8));
    let second = x87_load32(e.mem.u32(motion_b.wrapping_add(0xa0)));
    if second > first {
        let agent = allocate_agent_block(e, 0x40);
        fn_00d2f980(e, Ptr::new(agent), owner).addr()
    } else {
        let agent = allocate_agent_block(e, 0x40);
        fn_00d2f980(e, Ptr::new(agent), owner);
        e.mem.set_u32(agent, 0x010c_e9bc);
        agent
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2f9d0, fn_00d2f9d0(u32, u32, u32, u32) -> u32),
        entry!(0x00d2fa00, fn_00d2fa00(u32, u32, u32, u32) -> u32),
        entry!(0x00d2fa40, fn_00d2fa40(Ptr, Ptr, u32, u32) -> u32),
        entry!(
            0x00d2_f690,
            hkp_bv_tree_agent_calc_aabb_linear_cast(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d3_0f50,
            hkp_bv_tree_agent_static_linear_cast(Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00d3_15a0,
            hkp_bv_tree_agent_linear_cast(u32, Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(0x00d2_f550, hkp_bv_tree_agent_cleanup(Ptr, u32)),
        entry!(0x00d2_f5a0, hkp_bv_tree_agent_invalidate_tim(Ptr, u32)),
        entry!(0x00d2_f5e0, hkp_bv_tree_agent_warp_time(Ptr, f32, f32, u32)),
        entry!(0x00d2_f710, fn_00d2f710(Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00d2_f760,
            hkp_bv_tree_agent_update_shape_collection_filter(Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00d2_f8f0,
            hkp_bv_tree_agent_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(0x00d2_f980, fn_00d2f980(Ptr, u32) -> Ptr),
        entry!(
            0x00d2_fac0,
            hkp_bv_tree_agent_prepare_collision_partners(Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00d3_02d0,
            hkp_bv_tree_agent_process_collision(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d3_1110,
            hkp_bv_tree_agent_static_get_closest_points(Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00d3_12f0,
            hkp_bv_tree_agent_static_get_penetrations(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d3_14e0, fn_00d314e0(Ptr)),
        entry!(
            0x00d3_15d0,
            hkp_bv_tree_agent_get_closest_points(Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00d3_15f0,
            hkp_bv_tree_agent_get_penetrations(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A double at `address` that logs its arguments and answers `eax`.
    fn log_double(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An object whose vtable has the given `(slot, double address)` entries.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for &(slot, target) in slots {
            e.mem.set_u32(vtable + slot, target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The monitor stream: TLS index 3, a cursor at `records` and room for `room` bytes.
    fn monitor(e: &mut Engine, room: u32) -> (u32, u32) {
        e.set_global(MONITOR_SLOT, 3u32);
        let stream = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + room);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (stream, records)
    }

    #[test]
    fn calc_aabb_widens_the_box_by_the_sweep() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, TRANSFORM_SET_MUL_INVERSE_MUL, 0);
        e.register_double(0x0900_001c, |e, a| {
            // `this`, transform, float, aabb.
            assert_eq!(f32::from_bits(a[2]), 2.5);
            for lane in 0..4 {
                e.mem.set_f32(a[3] + 4 * lane, 1.0);
                e.mem.set_f32(a[3] + 0x10 + 4 * lane, 5.0);
            }
            Ret::default()
        });
        e.register_double(SWEEP_DISPLACEMENT, |e, a| {
            for (lane, v) in [1.0f32, -2.0, 0.0, -3.0].iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * lane as u32, *v);
            }
            Ret::default()
        });
        let owner = object(&mut e, &[(0x1c, 0x0900_001c)]);
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first, owner);
        e.mem.set_u32(first + 8, 0x111);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second + 8, 0x222);
        let input = e.mem.alloc(0x100);
        e.mem.set_f32(input + 8, 2.5);
        let aabb = e.mem.alloc(0x20);
        e.call(0x00d2_f690, &args![first, second, input, aabb]);
        let lower: Vec<f32> = (0..4).map(|i| e.mem.f32(aabb + 4 * i)).collect();
        let upper: Vec<f32> = (0..4).map(|i| e.mem.f32(aabb + 0x10 + 4 * i)).collect();
        assert_eq!(lower, vec![1.0, -1.0, 1.0, -2.0]);
        assert_eq!(upper, vec![6.0, 5.0, 5.0, 5.0]);
        let transform = &calls(&log, TRANSFORM_SET_MUL_INVERSE_MUL)[0];
        assert_eq!(transform[1..], [0x222, 0x111]);
    }

    #[test]
    fn static_linear_cast_calls_the_cast_function_of_each_overlapping_child() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let (stream, records) = monitor(&mut e, 0x100);
        // The geometry calls are not the subject here.
        log_double(&mut e, &log, TRANSFORM_SET_MUL_INVERSE_MUL, 0);
        log_double(&mut e, &log, SWEEP_DISPLACEMENT, 0);
        log_double(&mut e, &log, 0x0900_001c, 0);
        // The tree shape: slot 0x30 reports two keys in heap storage, slot 0x10 answers
        // the container, whose slot 0x14 answers a shape per key.
        let heap = e.mem.alloc(0x10);
        e.register_double(0x0900_0030, move |e, a| {
            e.mem.set_u32(heap, 7);
            e.mem.set_u32(heap + 4, 9);
            e.mem.set_u32(a[2], heap);
            e.mem.set_u32(a[2] + 4, 2);
            e.mem.set_u32(a[2] + 8, 4);
            Ret::default()
        });
        let shape_a = e.mem.alloc(0x20);
        let shape_b = e.mem.alloc(0x20);
        let first_header = object(&mut e, &[(0x1c, 0x0900_001c)]);
        let offset = 0xdu32 << 5;
        e.mem.set_u32(shape_a + 0xc, 0x10);
        e.mem.set_u32(shape_b + 0xc, 0x20);
        let dispatcher = e.mem.alloc(0xa80);
        let cast_one = 0x0900_0101;
        let cast_zero = 0x0900_0100;
        e.mem.set_u32(dispatcher + 0x9ac, cast_zero);
        e.mem.set_u32(dispatcher + 0x9ac + 0x14, cast_one);
        // The type byte of the first child is 1; the second child's byte stays 0.
        e.mem.set_u8(dispatcher + offset + 0x10, 1);
        log_double(&mut e, &log, cast_zero, 0);
        log_double(&mut e, &log, cast_one, 0);
        let container = object(&mut e, &[(0x14, 0x0900_0014)]);
        e.register_double(0x0900_0014, move |_, a| Ret {
            eax: if a[1] == 7 { shape_a } else { shape_b },
            ..Ret::default()
        });
        e.register_double(0x0900_0010, move |_, _| Ret {
            eax: container,
            ..Ret::default()
        });
        let owner = object(
            &mut e,
            &[
                (0x30, 0x0900_0030),
                (0x10, 0x0900_0010),
                (0x1c, 0x0900_001c),
            ],
        );
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first, first_header);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second, owner);
        e.mem.set_u32(second + 8, 0x222);
        let input = e.mem.alloc(0x100);
        e.mem.set_u32(input, dispatcher);
        // The heap array is released with its capacity in bytes.
        let router = e.mem.alloc(0x20);
        let allocator = object(&mut e, &[(0x10, 0x0900_0210)]);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        log_double(&mut e, &log, 0x0900_0210, 0);
        e.call(0x00d3_0f50, &args![first, second, input, 0xaau32, 0xbbu32]);
        let zero = calls(&log, cast_zero);
        let one = calls(&log, cast_one);
        assert_eq!((zero.len(), one.len()), (1, 1));
        assert_eq!(one[0][0], first);
        assert_eq!(one[0][2..], [input, 0xaa, 0xbb]);
        // The body record: shape, key, motion of the second body, the second body.
        let record = one[0][1];
        assert_eq!(e.mem.u32(record + 8), 0x222);
        assert_eq!(e.mem.u32(record + 0xc), second);
        assert_eq!(calls(&log, 0x0900_0210), vec![vec![allocator, heap, 16]]);
        // Three timer records were appended (16, 12 and 12 bytes).
        assert_eq!(e.mem.u32(stream + 4), records + 0x10 + 0xc + 0xc);
        assert_eq!(e.mem.u32(records), LABEL_BV_TREE);
        assert_eq!(e.mem.u32(records + 0xc), LABEL_QUERY_TREE);
        assert_eq!(e.mem.u32(records + 0x10), LABEL_NARROW_PHASE);
        assert_eq!(e.mem.u32(records + 0x1c), LABEL_END);
    }

    #[test]
    fn linear_cast_forwards_to_the_static_cast() {
        let mut e = test_engine();
        // A stream whose cursor is at its limit: no records are written.
        let (stream, records) = monitor(&mut e, 0);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, TRANSFORM_SET_MUL_INVERSE_MUL, 0);
        log_double(&mut e, &log, SWEEP_DISPLACEMENT, 0);
        log_double(&mut e, &log, 0x0900_001c, 0);
        log_double(&mut e, &log, 0x0900_0030, 0);
        log_double(&mut e, &log, 0x0900_0010, 0);
        let router = e.mem.alloc(0x20);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let owner = object(
            &mut e,
            &[
                (0x30, 0x0900_0030),
                (0x10, 0x0900_0010),
                (0x1c, 0x0900_001c),
            ],
        );
        let header = object(&mut e, &[(0x1c, 0x0900_001c)]);
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first, header);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second, owner);
        let input = e.mem.alloc(0x100);
        // The allocator the array would be freed through (never needed: inline storage).
        e.call(
            0x00d3_15a0,
            &args![VCALL_THIS, first, second, input, 1u32, 2u32],
        );
        assert_eq!(calls(&log, 0x0900_0030).len(), 1);
        assert_eq!(calls(&log, 0x0900_0010).len(), 1);
        assert_eq!(e.mem.u32(stream + 4), records);
    }
}

#[cfg(test)]
mod havok_agent_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The memory router (`00c85750`) answers an object whose `+0x10` is an allocator; the
    /// allocator's slot 4 hands out `block` and records the size it was asked for. The float the
    /// constructor reads (`0104da48`) is set to 2.5.
    fn install_allocator(e: &mut Engine, block: u32) -> std::rc::Rc<std::cell::RefCell<Vec<u32>>> {
        let sizes = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = std::rc::Rc::clone(&sizes);
        e.map(block, 0x100);
        e.map(0x0104_d000, 0x1000);
        e.mem.set_f32(0x0104_da48, 2.5);
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
        sizes
    }

    /// A body whose word at `+8` points to a motion with the float `value` at `+0xa0`.
    fn body_with_float_00d2fa40(e: &mut Engine, value: f32) -> u32 {
        let body = e.mem.alloc(0x10);
        let motion = e.mem.alloc(0x100);
        e.mem.set_f32(motion + 0xa0, value);
        e.mem.set_u32(body + 8, motion);
        body
    }

    #[test]
    fn fn_00d2f9d0_allocates_the_agent_and_runs_the_constructor_with_the_owner() {
        let mut e = Engine::new();
        let sizes = install_allocator(&mut e, 0x0800_0000);
        let agent = e.call(0x00d2_f9d0, &args![1u32, 2u32, 3u32, 0x88u32]).u32();
        assert_eq!(agent, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x40]);
        assert_eq!(e.mem.u16(agent + 4), 0x40);
        assert_eq!(e.mem.u32(agent + 8), 0x88);
        assert_eq!(e.mem.u32(agent), AGENT_VTABLE);
    }

    #[test]
    fn fn_00d2fa00_makes_the_flipped_agent() {
        let mut e = Engine::new();
        let sizes = install_allocator(&mut e, 0x0800_0000);
        let agent = e.call(0x00d2_fa00, &args![1u32, 2u32, 3u32, 0x88u32]).u32();
        assert_eq!(agent, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x40]);
        assert_eq!(e.mem.u32(agent + 8), 0x88);
        assert_eq!(e.mem.u32(agent), 0x010c_e9bc);
    }

    #[test]
    fn fn_00d2fa40_makes_the_plain_agent_when_the_second_float_is_larger() {
        let mut e = Engine::new();
        install_allocator(&mut e, 0x0800_0000);
        let a = body_with_float_00d2fa40(&mut e, 1.0);
        let b = body_with_float_00d2fa40(&mut e, 2.0);
        let agent = e.call(0x00d2_fa40, &args![a, b, 0u32, 0x88u32]).u32();
        assert_eq!(agent, 0x0800_0000);
        assert_eq!(e.mem.u32(agent), AGENT_VTABLE);
        assert_eq!(e.mem.u32(agent + 8), 0x88);
    }

    #[test]
    fn fn_00d2fa40_makes_the_flipped_agent_when_the_second_float_is_not_larger() {
        for (first, second) in [(2.0, 1.0), (2.0, 2.0), (f32::NAN, 1.0), (1.0, f32::NAN)] {
            let mut e = Engine::new();
            install_allocator(&mut e, 0x0800_0000);
            let a = body_with_float_00d2fa40(&mut e, first);
            let b = body_with_float_00d2fa40(&mut e, second);
            let agent = e.call(0x00d2_fa40, &args![a, b, 0u32, 0x88u32]).u32();
            assert_eq!(agent, 0x0800_0000);
            assert_eq!(e.mem.u32(agent), 0x010c_e9bc);
            assert_eq!(e.mem.u32(agent + 8), 0x88);
        }
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// The null agent of `00d107a0`.
    const NULL_AGENT_ADDRESS: u32 = 0x0126_87fc;
    /// The agent functions: slot -> double address `0x0900_1000 + slot`.
    const AGENT_SLOTS: [(u32, u32); 6] = [
        (0, 0x0900_1000),
        (0x18, 0x0900_1018),
        (0x1c, 0x0900_101c),
        (0x20, 0x0900_1020),
        (0x24, 0x0900_1024),
        (0x28, 0x0900_1028),
    ];
    const CREATE_A: u32 = 0x0900_0aaa;
    const CREATE_B: u32 = 0x0900_0bbb;
    const PENETRATIONS_FUNCTION: u32 = 0x0900_0ccc;
    const CLOSEST_FUNCTION: u32 = 0x0900_0ddd;
    const FREE_BLOCK: u32 = 0x0900_2010;
    const AVAILABLE: u32 = 0x0900_2024;
    const FILTER: u32 = 0x0900_0104;
    const CONTAINER_SHAPE: u32 = 0x0900_0114;
    const CONTAINER: u32 = 0x0900_0110;
    const BOX_QUERY: u32 = 0x0900_011c;
    const KEY_QUERY: u32 = 0x0900_0130;

    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.mem.set_f64(HALF_F64, 0.5);
        (e, Rc::new(RefCell::new(Vec::new())))
    }

    /// A double at `address` that logs its words and answers `eax`.
    fn log_double(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An object whose vtable has the given `(slot, double address)` entries.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for &(slot, target) in slots {
            e.mem.set_u32(vtable + slot, target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The agent slot doubles. Slots 0x18 and 0x20 also log the four words of the body record
    /// (their third word) after their own words.
    fn agent_doubles(e: &mut Engine, log: &Log) {
        for (slot, address) in AGENT_SLOTS {
            if slot == 0x18 || slot == 0x20 {
                let log = Rc::clone(log);
                e.register_double(address, move |e, a| {
                    let mut words = a.to_vec();
                    words.extend((0..4).map(|word| e.mem.u32(a[2] + 4 * word)));
                    log.borrow_mut().push((address, words));
                    Ret::default()
                });
            } else {
                log_double(e, log, address, 0);
            }
        }
    }

    fn new_agent(e: &mut Engine) -> u32 {
        object(e, &AGENT_SLOTS)
    }

    fn add_entry(e: &mut Engine, this: u32, key: u32, agent: u32) {
        let count = e.mem.u32(this + 0x10);
        assert!(count < e.mem.u32(this + 0x14));
        let entry = e.mem.u32(this + 0xc) + 12 * count;
        e.mem.set_u32(entry, key);
        e.mem.set_u32(entry + 8, agent);
        e.mem.set_u32(this + 0x10, count + 1);
    }

    /// A tree agent with the owner word 0x4242, a vtable of the agent slots and an entry array
    /// of `capacity` holding the given `(key, agent)` entries.
    fn tree_agent(e: &mut Engine, entries: &[(u32, u32)], capacity: u32) -> u32 {
        let this = object(e, &AGENT_SLOTS);
        e.mem.set_u32(this + 8, 0x4242);
        let data = e.mem.alloc(capacity * 12);
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x14, capacity);
        for &(key, agent) in entries {
            add_entry(e, this, key, agent);
        }
        this
    }

    /// The `(key, agent)` entries of a tree agent.
    fn entries_of(e: &Engine, this: u32) -> Vec<(u32, u32)> {
        let data = e.mem.u32(this + 0xc);
        (0..e.mem.u32(this + 0x10))
            .map(|index| {
                (
                    e.mem.u32(data + 12 * index),
                    e.mem.u32(data + 12 * index + 8),
                )
            })
            .collect()
    }

    /// Sets the cached box of an agent to `lower..upper` in all lanes.
    fn set_cached_box(e: &mut Engine, this: u32, lower: f32, upper: f32) {
        for lane in 0..4 {
            e.mem.set_f32(this + 0x20 + 4 * lane, lower);
            e.mem.set_f32(this + 0x30 + 4 * lane, upper);
        }
    }

    /// The memory router: an arena of 0x1000 bytes as its stack and an allocator whose slot
    /// 0x10 logs frees and whose slot 0x24 answers `available`; returns `(router, arena)`.
    fn router(e: &mut Engine, log: &Log, available: bool) -> (u32, u32) {
        log_double(e, log, FREE_BLOCK, 0);
        log_double(e, log, AVAILABLE, u32::from(available));
        let allocator = object(e, &[(0x10, FREE_BLOCK), (0x24, AVAILABLE)]);
        let arena = e.mem.alloc(0x1000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, arena);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (router, arena)
    }

    /// Doubles for `_reserve` (grows the array into a fresh block, keeping its elements) and
    /// `memMove` (an overlap-safe copy), `_reserve` logged.
    fn array_doubles(e: &mut Engine, log: &Log) {
        let sink = Rc::clone(log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let (array, count, size) = (a[1], a[2], a[3]);
            if e.mem.u32(array + 8) & 0x3fff_ffff < count {
                let fresh = e.mem.alloc(count * size);
                let old = e.mem.u32(array);
                for offset in 0..e.mem.u32(array + 4) * size {
                    let byte = e.mem.u8(old + offset);
                    e.mem.set_u8(fresh + offset, byte);
                }
                e.mem.set_u32(array, fresh);
                e.mem.set_u32(array + 8, count);
            }
            Ret::default()
        });
        e.register_double(MEM_MOVE, |e, a| {
            let bytes: Vec<u8> = (0..a[2]).map(|offset| e.mem.u8(a[1] + offset)).collect();
            for (offset, byte) in bytes.into_iter().enumerate() {
                e.mem.set_u8(a[0] + offset as u32, byte);
            }
            Ret::default()
        });
    }

    /// The monitor stream with room for 0x100 bytes; returns the address of its records.
    fn monitor(e: &mut Engine) -> u32 {
        e.set_global(MONITOR_SLOT, 3u32);
        let stream = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x100);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        records
    }

    /// Two bodies, a tree shape and a dispatcher input, with their doubles.
    struct Tree {
        first: u32,
        second: u32,
        input: u32,
        shape: u32,
        state_a: u32,
        state_b: u32,
        keys: Rc<RefCell<Vec<u32>>>,
    }

    /// The doubles: the tree shape's slot 0x30 (writes the keys, logs the box words after its
    /// own), slot 0x10 (the container), the container's slot 0x14 (a shape whose data word is
    /// 7), the first shape's slot 0x1C (writes the box -1..1, logs its words) and the input's
    /// filter (slot 4, enabled for the keys in `enabled`). The dispatcher's byte for the shape
    /// data 7 is 1 in the table at +0x1A0 and 2 in the table at +0x5A0 (so also 1 for the
    /// static queries, whose first shape word is 0 and 0xD * 0x20 = 0x1A0). The input's
    /// distance is 2.0 and its step 1.0.
    fn tree(e: &mut Engine, log: &Log, enabled: &[u32]) -> Tree {
        let keys: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
        let fill = Rc::clone(&keys);
        let sink = Rc::clone(log);
        e.register_double(KEY_QUERY, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..8).map(|lane| e.mem.u32(a[1] + 4 * lane)));
            sink.borrow_mut().push((KEY_QUERY, words));
            let data = e.mem.u32(a[2]);
            let keys = fill.borrow();
            for (index, key) in keys.iter().enumerate() {
                e.mem.set_u32(data + 4 * index as u32, *key);
            }
            e.mem.set_u32(a[2] + 4, keys.len() as u32);
            Ret::default()
        });
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape + 0xc, 7);
        let container = object(e, &[(0x14, CONTAINER_SHAPE)]);
        log_double(e, log, CONTAINER_SHAPE, shape);
        log_double(e, log, CONTAINER, container);
        let owner = object(e, &[(0x10, CONTAINER), (0x30, KEY_QUERY)]);
        let second = e.mem.alloc(0x20);
        let state_b = e.mem.alloc(0x100);
        e.mem.set_u32(second, owner);
        e.mem.set_u32(second + 8, state_b);

        let sink = Rc::clone(log);
        e.register_double(BOX_QUERY, move |e, a| {
            sink.borrow_mut().push((BOX_QUERY, a.to_vec()));
            for lane in 0..4 {
                e.mem.set_f32(a[3] + 4 * lane, -1.0);
                e.mem.set_f32(a[3] + 0x10 + 4 * lane, 1.0);
            }
            Ret::default()
        });
        let first_owner = object(e, &[(0x1c, BOX_QUERY)]);
        let first = e.mem.alloc(0x20);
        let state_a = e.mem.alloc(0x100);
        e.mem.set_u32(first, first_owner);
        e.mem.set_u32(first + 8, state_a);

        let yes = e.mem.alloc(8);
        e.mem.set_u8(yes, 1);
        let no = e.mem.alloc(8);
        let allowed = enabled.to_vec();
        let sink = Rc::clone(log);
        e.register_double(FILTER, move |_, a| {
            sink.borrow_mut().push((FILTER, a.to_vec()));
            Ret {
                eax: if allowed.contains(&a[6]) { yes } else { no },
                ..Ret::default()
            }
        });
        let filter_table = e.mem.alloc(0x20);
        e.mem.set_u32(filter_table + 4, FILTER);
        let filter_holder = e.mem.alloc(0x20);
        e.mem.set_u32(filter_holder + 0xc, filter_table);

        let dispatcher = e.mem.alloc(0xc00);
        e.mem.set_u8(dispatcher + 0x1a0 + 7, 1);
        e.mem.set_u8(dispatcher + 0x5a0 + 7, 2);
        e.mem.set_u32(dispatcher + 0x9b4, CREATE_A);
        e.mem.set_u32(dispatcher + 0x9c8, CREATE_B);
        e.mem.set_u32(dispatcher + 0x9b8, PENETRATIONS_FUNCTION);
        e.mem.set_u32(dispatcher + 0x9bc, CLOSEST_FUNCTION);
        for address in [CREATE_A, CREATE_B] {
            let sink = Rc::clone(log);
            e.register_double(address, move |e, a| {
                let mut words = a.to_vec();
                words.extend((0..4).map(|word| e.mem.u32(a[1] + 4 * word)));
                sink.borrow_mut().push((address, words));
                let created = object(e, &AGENT_SLOTS);
                Ret {
                    eax: created,
                    ..Ret::default()
                }
            });
        }
        for address in [PENETRATIONS_FUNCTION, CLOSEST_FUNCTION] {
            let sink = Rc::clone(log);
            e.register_double(address, move |e, a| {
                let mut words = a.to_vec();
                words.extend((0..4).map(|word| e.mem.u32(a[1] + 4 * word)));
                sink.borrow_mut().push((address, words));
                Ret::default()
            });
        }
        let input = e.mem.alloc(0x80);
        let sweep = e.mem.alloc(0x20);
        e.mem.set_u32(input, dispatcher);
        e.mem.set_u32(input + 0xc, filter_holder);
        e.mem.set_u32(input + 0x60, sweep);
        e.mem.set_f32(input + 8, 2.0);
        e.mem.set_f32(input + 0x58, 1.0);
        log_double(e, log, TRANSFORM_SET_MUL_INVERSE_MUL, 0);
        Tree {
            first,
            second,
            input,
            shape,
            state_a,
            state_b,
            keys,
        }
    }

    /// Everything the merge and process tests need: an agent with room for 3 entries (none
    /// yet), the tree, the memory router, the monitor and an output block whose second word is
    /// 0x55 (the destroy word).
    struct Rig {
        e: Engine,
        log: Log,
        tree: Tree,
        this: u32,
        output: u32,
        records: u32,
        router: u32,
        arena: u32,
    }

    fn rig(linear: bool, keys: &[u32], enabled: &[u32]) -> Rig {
        let (mut e, log) = engine();
        agent_doubles(&mut e, &log);
        let (router, arena) = router(&mut e, &log, true);
        array_doubles(&mut e, &log);
        let tree = tree(&mut e, &log, enabled);
        *tree.keys.borrow_mut() = keys.to_vec();
        e.mem.set_u8(LINEAR_MERGE_FLAG, u8::from(linear));
        let records = monitor(&mut e);
        let this = tree_agent(&mut e, &[], 3);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 4, 0x55);
        Rig {
            e,
            log,
            tree,
            this,
            output,
            records,
            router,
            arena,
        }
    }

    #[test]
    fn cleanup_calls_the_children_then_the_destructor() {
        let (mut e, log) = engine();
        agent_doubles(&mut e, &log);
        let a = new_agent(&mut e);
        let b = new_agent(&mut e);
        let this = tree_agent(&mut e, &[(1, a), (2, 0), (3, b)], 4);
        e.call(0x00d2_f550, &args![this, 9u32]);
        assert_eq!(calls(&log, 0x0900_101c), vec![vec![a, 9], vec![b, 9]]);
        assert_eq!(calls(&log, 0x0900_1000), vec![vec![this, 1]]);
        // The destructor is the last call.
        assert_eq!(log.borrow().last().unwrap().0, 0x0900_1000);
    }

    #[test]
    fn invalidate_tim_calls_slot_0x24_of_the_existing_children() {
        let (mut e, log) = engine();
        agent_doubles(&mut e, &log);
        let a = new_agent(&mut e);
        let this = tree_agent(&mut e, &[(1, 0), (2, a)], 4);
        e.call(0x00d2_f5a0, &args![this, 5u32]);
        assert_eq!(calls(&log, 0x0900_1024), vec![vec![a, 5]]);
        assert_eq!(log.borrow().len(), 1);
    }

    #[test]
    fn warp_time_passes_the_times_through_the_x87_copy() {
        let (mut e, log) = engine();
        agent_doubles(&mut e, &log);
        let a = new_agent(&mut e);
        let this = tree_agent(&mut e, &[(1, a), (2, 0)], 4);
        let signalling = f32::from_bits(0x7fa0_0000);
        e.call(0x00d2_f5e0, &args![this, signalling, 1.5f32, 7u32]);
        assert_eq!(
            calls(&log, 0x0900_1028),
            vec![vec![a, 0x7fe0_0000, 1.5f32.to_bits(), 7]]
        );
    }

    #[test]
    fn the_box_function_asks_the_first_shape_with_the_relative_transform() {
        let (mut e, log) = engine();
        let tree = tree(&mut e, &log, &[]);
        let aabb = e.mem.alloc(0x20);
        e.call(
            0x00d2_f710,
            &args![tree.first, tree.second, tree.input, aabb],
        );
        let transform = calls(&log, TRANSFORM_SET_MUL_INVERSE_MUL);
        assert_eq!(transform[0][1..], [tree.state_b, tree.state_a]);
        let query = &calls(&log, BOX_QUERY)[0];
        assert_eq!(query[1], transform[0][0]);
        assert_eq!(query[2], 2.0f32.to_bits());
        assert_eq!(query[3], aabb);
        assert_eq!(e.mem.f32(aabb), -1.0);
        assert_eq!(e.mem.f32(aabb + 0x10), 1.0);
    }

    #[test]
    fn update_filter_creates_updates_and_destroys_by_the_filter_answer() {
        let (mut e, log) = engine();
        agent_doubles(&mut e, &log);
        let tree = tree(&mut e, &log, &[10, 11]);
        let real_a = new_agent(&mut e);
        let real_b = new_agent(&mut e);
        let this = tree_agent(
            &mut e,
            &[
                (10, NULL_AGENT_ADDRESS),
                (11, real_a),
                (12, real_b),
                (13, NULL_AGENT_ADDRESS),
            ],
            4,
        );
        e.call(
            0x00d2_f760,
            &args![this, tree.first, tree.second, tree.input, 0x99u32],
        );
        // Key 10: enabled with the null agent, so the dispatcher's create function (table
        // +0x1A0, byte 1) runs with (first, record, input, owner word).
        let created = calls(&log, CREATE_A);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][0], tree.first);
        assert_eq!(created[0][2..4], [tree.input, 0x4242]);
        assert_eq!(created[0][4..], [tree.shape, 10, tree.state_b, tree.second]);
        // Key 11: enabled with a real agent, so its slot 0x20 runs.
        let updated = calls(&log, 0x0900_1020);
        assert_eq!(updated.len(), 1);
        assert_eq!(updated[0][..2], [real_a, tree.first]);
        assert_eq!(updated[0][3..5], [tree.input, 0x99]);
        assert_eq!(updated[0][5..], [tree.shape, 11, tree.state_b, tree.second]);
        // Key 12: refused with a real agent: slot 0x1C, then the null agent.
        assert_eq!(calls(&log, 0x0900_101c), vec![vec![real_b, 0x99]]);
        let entries = entries_of(&e, this);
        assert_eq!(
            entries.iter().map(|entry| entry.0).collect::<Vec<_>>(),
            vec![10, 11, 12, 13]
        );
        assert_ne!(entries[0].1, NULL_AGENT_ADDRESS);
        assert_eq!(entries[1].1, real_a);
        assert_eq!(entries[2].1, NULL_AGENT_ADDRESS);
        // Key 13: refused with the null agent: nothing.
        assert_eq!(entries[3].1, NULL_AGENT_ADDRESS);
        assert_eq!(calls(&log, FILTER).len(), 4);
    }

    #[test]
    fn update_filter_uses_the_other_table_when_the_input_flag_is_set() {
        let (mut e, log) = engine();
        let tree = tree(&mut e, &log, &[10]);
        e.mem.set_u32(tree.input + 0x14, 1);
        let this = tree_agent(&mut e, &[(10, NULL_AGENT_ADDRESS)], 1);
        e.call(
            0x00d2_f760,
            &args![this, tree.first, tree.second, tree.input, 0u32],
        );
        assert_eq!(calls(&log, CREATE_B).len(), 1);
        assert!(calls(&log, CREATE_A).is_empty());
    }

    #[test]
    fn update_filter_with_no_entries_only_asks_for_the_container() {
        let (mut e, log) = engine();
        let tree = tree(&mut e, &log, &[]);
        let this = tree_agent(&mut e, &[], 1);
        e.call(
            0x00d2_f760,
            &args![this, tree.first, tree.second, tree.input, 0u32],
        );
        assert_eq!(calls(&log, CONTAINER).len(), 1);
        assert!(calls(&log, CONTAINER_SHAPE).is_empty());
    }

    #[test]
    fn content_statistics_reports_the_array_and_the_agents() {
        let (mut e, log) = engine();
        log_double(&mut e, &log, STATISTICS_ALLOCATED_SIZE, 0x77);
        log_double(&mut e, &log, 0x0900_0c0c, 0);
        log_double(&mut e, &log, 0x0900_0c14, 0);
        let collector = object(&mut e, &[(0xc, 0x0900_0c0c), (0x14, 0x0900_0c14)]);
        let agent = new_agent(&mut e);
        let this = tree_agent(&mut e, &[(1, agent), (2, 0)], 4);
        let data = e.mem.u32(this + 0xc);
        // Heap storage (capacity word non-negative): the array is reported.
        e.call(0x00d2_f8f0, &args![this, collector, 0u32]);
        assert_eq!(
            calls(&log, STATISTICS_ALLOCATED_SIZE),
            vec![vec![data, 4 * 12]]
        );
        assert_eq!(
            calls(&log, 0x0900_0c14),
            vec![vec![collector, 3, LABEL_AGENT_PTRS, data, 2 * 12, 0x77]]
        );
        assert_eq!(
            calls(&log, 0x0900_0c0c),
            vec![vec![collector, LABEL_AGENT, agent, 0]]
        );
        // Inline storage (capacity word negative): reported only with the collector's byte.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x14, 0x8000_0004);
        e.call(0x00d2_f8f0, &args![this, collector, 0u32]);
        assert!(calls(&log, 0x0900_0c14).is_empty());
        assert_eq!(calls(&log, 0x0900_0c0c).len(), 1);
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00d2_f8f0, &args![this, collector, 0u32]);
        assert_eq!(
            calls(&log, STATISTICS_ALLOCATED_SIZE)[0],
            vec![data, 4 * 12]
        );
        assert_eq!(calls(&log, 0x0900_0c14).len(), 1);
    }

    #[test]
    fn the_initialiser_sets_the_fields_and_broadcasts_the_float() {
        let (mut e, _log) = engine();
        e.mem.set_f32(BOX_RESET_VALUE, 3.0e38);
        let this = e.mem.alloc(0x80);
        let result = e.call(0x00d2_f980, &args![this, 0x1234u32]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0x1234);
        assert_eq!(e.mem.u32(this), AGENT_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
        for lane in 0..4 {
            assert_eq!(e.mem.f32(this + 0x20 + 4 * lane), 3.0e38);
            assert_eq!(e.mem.f32(this + 0x30 + 4 * lane), 3.0e38);
        }
    }

    #[test]
    fn prepare_partners_stops_when_the_cached_box_holds_the_new_one() {
        let mut r = rig(true, &[1], &[1]);
        r.e.mem.set_u8(CACHED_BOX_FLAG, 1);
        set_cached_box(&mut r.e, r.this, -5.0, 5.0);
        let t = &r.tree;
        r.e.call(
            0x00d2_fac0,
            &args![r.this, t.first, t.second, t.input, 0u32],
        );
        // The tolerance is 2.0 * 0.5 + 2.0 = 3.0.
        assert_eq!(calls(&r.log, BOX_QUERY)[0][2], 3.0f32.to_bits());
        assert!(calls(&r.log, KEY_QUERY).is_empty());
        assert!(calls(&r.log, AVAILABLE).is_empty());
        assert_eq!(r.e.mem.f32(r.this + 0x20), -5.0);
    }

    #[test]
    fn prepare_partners_stores_a_box_the_cache_does_not_hold() {
        let mut r = rig(true, &[], &[]);
        r.e.mem.set_u8(CACHED_BOX_FLAG, 1);
        // The cached box holds the new one except for the upper bound of lane 1.
        set_cached_box(&mut r.e, r.this, -5.0, 5.0);
        r.e.mem.set_f32(r.this + 0x34, 0.5);
        let t = &r.tree;
        r.e.call(
            0x00d2_fac0,
            &args![r.this, t.first, t.second, t.input, 0u32],
        );
        assert_eq!(r.e.mem.f32(r.this + 0x20), -1.0);
        assert_eq!(r.e.mem.f32(r.this + 0x34), 1.0);
        assert_eq!(calls(&r.log, KEY_QUERY).len(), 1);
        // No keys and no entries: no bytes are asked for.
        let asked = calls(&r.log, AVAILABLE);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][1], 0);
    }

    #[test]
    fn prepare_partners_gives_up_when_the_allocator_is_short() {
        let (mut e, log) = engine();
        agent_doubles(&mut e, &log);
        router(&mut e, &log, false);
        array_doubles(&mut e, &log);
        let tree = tree(&mut e, &log, &[1]);
        *tree.keys.borrow_mut() = vec![1, 2];
        log_double(&mut e, &log, SET_OUT_OF_MEMORY_STATE, 0);
        let this = tree_agent(&mut e, &[], 2);
        e.call(
            0x00d2_fac0,
            &args![this, tree.first, tree.second, tree.input, 0u32],
        );
        // Two keys, no entries: 2 << 7 bytes were asked for.
        assert_eq!(calls(&log, AVAILABLE)[0][1], 2 << 7);
        assert_eq!(calls(&log, SET_OUT_OF_MEMORY_STATE), vec![vec![1]]);
        assert!(calls(&log, CONTAINER).is_empty());
        assert!(entries_of(&e, this).is_empty());
    }

    #[test]
    fn prepare_partners_merges_linearly_when_the_flag_is_set() {
        let mut r = rig(true, &[2, 3, 4, 5], &[2, 3, 4]);
        let first_agent = new_agent(&mut r.e);
        let second_agent = new_agent(&mut r.e);
        let third_agent = new_agent(&mut r.e);
        for (key, agent) in [(1, first_agent), (2, second_agent), (3, third_agent)] {
            add_entry(&mut r.e, r.this, key, agent);
        }
        let t = &r.tree;
        r.e.call(
            0x00d2_fac0,
            &args![r.this, t.first, t.second, t.input, 0x66u32],
        );
        // The entry of key 1 had no key: its agent was destroyed with the word.
        assert_eq!(calls(&r.log, 0x0900_101c), vec![vec![first_agent, 0x66]]);
        // Four keys against three entries: (4 - 3) << 7 bytes were asked for.
        assert_eq!(calls(&r.log, AVAILABLE)[0][1], 1 << 7);
        // Keys 4 and 5 got agents (5 is refused by the filter, so it holds the null agent) and
        // the array grew once, to twice its capacity of 3.
        let entries = entries_of(&r.e, r.this);
        assert_eq!(
            entries.iter().map(|entry| entry.0).collect::<Vec<_>>(),
            vec![2, 3, 4, 5]
        );
        assert_eq!(entries[0].1, second_agent);
        assert_eq!(entries[1].1, third_agent);
        assert_ne!(entries[2].1, NULL_AGENT_ADDRESS);
        assert_eq!(entries[3].1, NULL_AGENT_ADDRESS);
        let created = calls(&r.log, CREATE_A);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][0], t.first);
        assert_eq!(created[0][2..4], [t.input, 0x4242]);
        assert_eq!(created[0][4..], [t.shape, 4, t.state_b, t.second]);
        let reserves = calls(&r.log, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 1);
        assert_eq!(reserves[0][1..], [r.this + 0xc, 6, 12]);
        assert_eq!(r.e.mem.u32(r.this + 0x14) & 0x3fff_ffff, 6);
    }

    #[test]
    fn prepare_partners_merges_sorted_keys_when_the_flag_is_clear() {
        // Unsorted keys: the sort puts them in order; key 5 is refused by the filter.
        let mut r = rig(false, &[5, 3, 2, 4], &[2, 3, 4]);
        let first_agent = new_agent(&mut r.e);
        let second_agent = new_agent(&mut r.e);
        let third_agent = new_agent(&mut r.e);
        for (key, agent) in [(1, first_agent), (2, second_agent), (3, third_agent)] {
            add_entry(&mut r.e, r.this, key, agent);
        }
        r.e.mem.set_u32(r.e.mem.u32(r.this + 0xc) + 4, 0x1111);
        let t = &r.tree;
        r.e.call(
            0x00d2_fac0,
            &args![r.this, t.first, t.second, t.input, 0x66u32],
        );
        assert_eq!(calls(&r.log, 0x0900_101c), vec![vec![first_agent, 0x66]]);
        let entries = entries_of(&r.e, r.this);
        assert_eq!(
            entries.iter().map(|entry| entry.0).collect::<Vec<_>>(),
            vec![2, 3, 4, 5]
        );
        assert_eq!(entries[0].1, second_agent);
        assert_eq!(entries[1].1, third_agent);
        assert_ne!(entries[2].1, NULL_AGENT_ADDRESS);
        assert_eq!(entries[3].1, NULL_AGENT_ADDRESS);
        // The kept entries are copied whole (the middle word too): the first entry's 0x1111
        // went with its removal.
        let data = r.e.mem.u32(r.this + 0xc);
        assert_eq!(r.e.mem.u32(data + 4), 0);
        assert_eq!(calls(&r.log, CREATE_A).len(), 1);
        // The router's stack is back where it was.
        assert_eq!(r.e.mem.u32(r.router), r.arena);
        assert_eq!(r.e.mem.u32(r.this + 0x10), 4);
    }

    #[test]
    fn prepare_partners_merge_destroys_the_entries_left_over_after_the_keys() {
        // Entries 1, 2, 3 against the single key 1: the agents of 2 and 3 are destroyed.
        let mut r = rig(false, &[1], &[1]);
        let agents: Vec<u32> = (0..3).map(|_| new_agent(&mut r.e)).collect();
        for (key, agent) in [(1, agents[0]), (2, agents[1]), (3, agents[2])] {
            add_entry(&mut r.e, r.this, key, agent);
        }
        let t = &r.tree;
        r.e.call(
            0x00d2_fac0,
            &args![r.this, t.first, t.second, t.input, 0x66u32],
        );
        assert_eq!(
            calls(&r.log, 0x0900_101c),
            vec![vec![agents[1], 0x66], vec![agents[2], 0x66]]
        );
        assert_eq!(entries_of(&r.e, r.this), vec![(1, agents[0])]);
    }

    #[test]
    fn process_collision_runs_the_children_between_the_labels() {
        let mut r = rig(false, &[2], &[2]);
        let existing = new_agent(&mut r.e);
        add_entry(&mut r.e, r.this, 2, existing);
        let t = &r.tree;
        r.e.call(
            0x00d3_02d0,
            &args![r.this, t.first, t.second, t.input, r.output],
        );
        // Non-sweeping: the tolerance is the input's 2.0 times 0.5.
        let boxed = &calls(&r.log, BOX_QUERY)[0];
        assert_eq!(boxed[2], 1.0f32.to_bits());
        let relative = &calls(&r.log, TRANSFORM_SET_MUL_INVERSE_MUL)[0];
        assert_eq!(relative[1..], [t.state_b, t.state_a]);
        // The tree shape was asked with the box and no key was new.
        let query = &calls(&r.log, KEY_QUERY)[0];
        assert_eq!(f32::from_bits(query[3]), -1.0);
        assert_eq!(f32::from_bits(query[7]), 1.0);
        assert!(calls(&r.log, CREATE_A).is_empty());
        // The narrow phase: slot 0x18 of the child with (first, record, input, output).
        let narrow = calls(&r.log, 0x0900_1018);
        assert_eq!(narrow.len(), 1);
        assert_eq!(narrow[0][..2], [existing, t.first]);
        assert_eq!(narrow[0][3..5], [t.input, r.output]);
        assert_eq!(narrow[0][5..], [t.shape, 2, t.state_b, t.second]);
        // Start record (two labels), narrow phase label, end label.
        assert_eq!(r.e.mem.u32(r.records), LABEL_BV_TREE);
        assert_eq!(r.e.mem.u32(r.records + 0xc), LABEL_QUERY_TREE);
        assert_eq!(r.e.mem.u32(r.records + 0x10), LABEL_NARROW_PHASE);
        assert_eq!(r.e.mem.u32(r.records + 0x1c), LABEL_END);
    }

    #[test]
    fn process_collision_skips_the_query_when_the_cached_box_holds_the_box() {
        let mut r = rig(false, &[3], &[3]);
        r.e.mem.set_u8(CACHED_BOX_FLAG, 1);
        set_cached_box(&mut r.e, r.this, -5.0, 5.0);
        let t = &r.tree;
        r.e.call(
            0x00d3_02d0,
            &args![r.this, t.first, t.second, t.input, r.output],
        );
        assert!(calls(&r.log, KEY_QUERY).is_empty());
        assert!(entries_of(&r.e, r.this).is_empty());
        assert_eq!(r.e.mem.f32(r.this + 0x20), -5.0);
    }

    /// Sets body A to move (3, -3, 0) over the unit step with scale 1 and body B's rows to the
    /// identity: the motion in B's frame is (3, -3, 0, 0).
    fn move_body_a(e: &mut Engine, tree: &Tree) {
        e.mem.set_f32(tree.state_a + 0x5c, 1.0);
        e.mem.set_f32(tree.state_a + 0x40, 3.0);
        e.mem.set_f32(tree.state_a + 0x44, -3.0);
        e.mem.set_f32(tree.state_b, 1.0);
        e.mem.set_f32(tree.state_b + 0x14, 1.0);
        e.mem.set_f32(tree.state_b + 0x28, 1.0);
    }

    #[test]
    fn process_collision_grows_the_cached_box_along_the_motion() {
        let mut r = rig(false, &[], &[]);
        r.e.mem.set_u8(CACHED_BOX_FLAG, 1);
        r.e.mem.set_f32(GROWTH_FACTOR, 0.4);
        r.e.mem.set_f32(SWEEP_FACTOR, -0.1);
        for lane in 0..4 {
            r.e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        move_body_a(&mut r.e, &r.tree);
        let t = &r.tree;
        r.e.call(
            0x00d3_02d0,
            &args![r.this, t.first, t.second, t.input, r.output],
        );
        // The box -1..1 (extents 2) is lowered and raised by the margin 1.0, then widened:
        // lower = max(sweep * max(0, motion), -(0.4 * 2)) - 2, upper = min(sweep * min(0, motion),
        // 0.4 * 2) + 2.
        let growth = 0.4f32 * 2.0;
        let lower = [(-0.1f32 * 3.0).max(-growth) + -2.0, -2.0, -2.0, -2.0];
        let upper = [2.0, (-0.1f32 * -3.0).min(growth) + 2.0, 2.0, 2.0];
        let query = &calls(&r.log, KEY_QUERY)[0];
        for (lane, (low, high)) in lower.iter().zip(&upper).enumerate() {
            assert_eq!(f32::from_bits(query[3 + lane]), *low);
            assert_eq!(f32::from_bits(query[7 + lane]), *high);
            assert_eq!(r.e.mem.f32(r.this + 0x20 + 4 * lane as u32), *low);
            assert_eq!(r.e.mem.f32(r.this + 0x30 + 4 * lane as u32), *high);
        }
    }

    #[test]
    fn process_collision_sweeps_the_box_when_the_input_asks_for_it() {
        let mut r = rig(false, &[], &[]);
        let t = &r.tree;
        r.e.mem.set_u32(r.e.mem.u32(t.input + 0x60) + 0x10, 1);
        r.e.mem.set_f32(t.input + 8, 4.0);
        r.e.mem.set_f32(t.state_b + 0x9c, 2.0);
        r.e.mem.set_f32(t.state_b + 0xa0, 0.5);
        r.e.mem.set_f32(t.state_a + 0x9c, 1.0);
        r.e.mem.set_f32(t.state_a + 0xa0, 3.0);
        move_body_a(&mut r.e, t);
        r.e.call(
            0x00d3_02d0,
            &args![r.this, t.first, t.second, t.input, r.output],
        );
        // The tolerance is (1 + 2) * 3 + (2 * 2 * 0.5) + 4 * 0.5 = 13.
        assert_eq!(calls(&r.log, BOX_QUERY)[0][2], 13.0f32.to_bits());
        // The box -1..1 widened by the motion (3, -3, 0, 0): min(0, motion) below, max above.
        let query = &calls(&r.log, KEY_QUERY)[0];
        let lower = [-1.0f32, -4.0, -1.0, -1.0];
        let upper = [4.0f32, 1.0, 1.0, 1.0];
        for (lane, (low, high)) in lower.iter().zip(&upper).enumerate() {
            assert_eq!(f32::from_bits(query[3 + lane]), *low);
            assert_eq!(f32::from_bits(query[7 + lane]), *high);
        }
    }

    #[test]
    fn static_closest_points_calls_the_dispatcher_function_for_the_allowed_keys() {
        let mut r = rig(false, &[10, 11], &[10]);
        let t = &r.tree;
        r.e.call(0x00d3_1110, &args![t.first, t.second, t.input, 0x5150u32]);
        let called = calls(&r.log, CLOSEST_FUNCTION);
        assert_eq!(called.len(), 1);
        assert_eq!(called[0][0], t.first);
        assert_eq!(called[0][2..4], [t.input, 0x5150]);
        assert_eq!(called[0][4..], [t.shape, 10, t.state_b, t.second]);
        assert!(calls(&r.log, PENETRATIONS_FUNCTION).is_empty());
        assert_eq!(calls(&r.log, FILTER).len(), 2);
        assert_eq!(r.e.mem.u32(r.records), LABEL_BV_TREE);
        assert_eq!(r.e.mem.u32(r.records + 0xc), LABEL_QUERY_TREE);
        assert_eq!(r.e.mem.u32(r.records + 0x10), LABEL_NARROW_PHASE);
        assert_eq!(r.e.mem.u32(r.records + 0x1c), LABEL_END);
    }

    #[test]
    fn static_penetrations_stop_when_the_collector_flag_is_set() {
        let mut r = rig(false, &[10, 11, 12], &[10, 11, 12]);
        let collector = r.e.mem.alloc(0x10);
        let t = &r.tree;
        r.e.call(0x00d3_12f0, &args![t.first, t.second, t.input, collector]);
        assert_eq!(calls(&r.log, PENETRATIONS_FUNCTION).len(), 3);
        assert!(calls(&r.log, CLOSEST_FUNCTION).is_empty());
        r.log.borrow_mut().clear();
        r.e.mem.set_u8(collector + 4, 1);
        r.e.call(0x00d3_12f0, &args![t.first, t.second, t.input, collector]);
        assert_eq!(calls(&r.log, PENETRATIONS_FUNCTION).len(), 1);
    }

    #[test]
    fn the_member_queries_forward_to_the_static_ones() {
        let mut r = rig(false, &[10], &[10]);
        let collector = r.e.mem.alloc(0x10);
        let t = &r.tree;
        r.e.call(
            0x00d3_15d0,
            &args![0u32, t.first, t.second, t.input, collector],
        );
        assert_eq!(calls(&r.log, CLOSEST_FUNCTION).len(), 1);
        r.e.call(
            0x00d3_15f0,
            &args![0u32, t.first, t.second, t.input, collector],
        );
        assert_eq!(calls(&r.log, PENETRATIONS_FUNCTION).len(), 1);
    }

    #[test]
    fn registration_adds_the_three_tree_agent_entries() {
        let (mut e, log) = engine();
        let sink = Rc::clone(&log);
        e.register_double(REGISTER_AGENT, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..5).map(|word| e.mem.u32(a[1] + 4 * word)));
            sink.borrow_mut().push((REGISTER_AGENT, words));
            Ret::default()
        });
        e.call(0x00d3_14e0, &args![0xd15cu32]);
        let registered = calls(&log, REGISTER_AGENT);
        assert_eq!(registered.len(), 3);
        for call in &registered {
            assert_eq!(call[0], 0xd15c);
        }
        assert_eq!(registered[0][2..4], [8, 0xffff_ffff]);
        assert_eq!(
            registered[0][4..],
            [0x00d2_fa00, 0x00d2_e970, 0x00d2_e9b0, 0x00d2_e9f0, 0x0101]
        );
        assert_eq!(registered[1][2..4], [0xffff_ffff, 8]);
        assert_eq!(
            registered[1][4..],
            [0x00d2_f9d0, 0x00d3_12f0, 0x00d3_1110, 0x00d3_0f50, 0x0100]
        );
        assert_eq!(registered[2][2..4], [8, 8]);
        assert_eq!(
            registered[2][4..],
            [0x00d2_fa40, 0x00d3_12f0, 0x00d3_1110, 0x00d3_0f50, 0x0100]
        );
    }
}
