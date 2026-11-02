//! `Havok SDK/hkpconvexlistagent3.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use crate::prelude::*;

/// `"hkConvexListAgent3"`, the object name of the statistics report.
const AGENT_NAME: u32 = 0x010c_eb2c;
/// `hkpAgentMachineUtil::calc1nStatistics` (`00d97250`, cdecl: the track, the dispatcher
/// pointer, the collector).
const CALC_1N_STATISTICS: u32 = 0x00d9_7250;
/// `hkAgent1nMachine_UpdateShapeCollectionFilter` (`00da48e0`, cdecl: the track, a
/// seven-word description).
const UPDATE_SHAPE_COLLECTION_FILTER: u32 = 0x00da_48e0;
/// The registration of an agent function table for a shape type pair (`00cfbd60`, thiscall
/// on the dispatcher: the table, the two shape types; `RET 0xc`).
const REGISTER_AGENT: u32 = 0x00cf_bd60;
/// The `TlsGetValue` import slot (`kernel32`) and the TLS index of the profiling record stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const PROFILE_SLOT: u32 = 0x0126_835c;
/// The profiling labels: `"LtCvxLst"`, `"StTim"`, `"StStream"` and the end label.
const LABEL_LT_CVX_LST: u32 = 0x010c_eb84;
const LABEL_ST_TIM: u32 = 0x010c_eb7c;
const LABEL_ST_STREAM: u32 = 0x010c_eb70;
const LABEL_END: u32 = 0x0107_6f38;
/// The virtual table the body descriptor on the stack is given.
const BODY_DESCRIPTOR_TABLE: u32 = 0x010c_eb44;
/// The float `0xfeffffee` stored as the query's time when nothing was hit.
const NO_HIT_TIME: u32 = 0x010c_eb6c;
/// The memory router (`00c85750`, no arguments): its first word is the cursor of a stack-like
/// allocation area.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `memcpy` (`00ec44d0`, cdecl: destination, source, bytes).
const MEMCPY: u32 = 0x00ec_44d0;
/// `hkTransform::setMulInverseMul` (`00cb22f0`, thiscall: the first and the second transform).
const SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// The initialiser of the local output object (`00d33670`, thiscall on the object: the
/// context) and its continuation after the machine was destroyed (`00d33600`, thiscall:
/// both bodies, the context, the pair's object).
const INIT_LOCAL_OBJECT: u32 = 0x00d3_3670;
const RESUME_LOCAL_OBJECT: u32 = 0x00d3_3600;
/// `hkpGsk::getClosestPoint` (`00dabca0`, cdecl: the body descriptor, the agent, the query,
/// a 16-byte result).
const GET_CLOSEST_POINT: u32 = 0x00da_bca0;
/// `hkAgent1nMachine_Destroy` (`00da42b0`, cdecl: the machine, the dispatcher, the pair's
/// object, the context's owner).
const DESTROY_MACHINE: u32 = 0x00da_42b0;
/// `hkPredGskAgent3::process` (`00d41a30`, cdecl: the record, the input, the agent, the
/// query, the context) and `hkPredGskAgent3::cleanup` (`00d41860`, cdecl: the input, the
/// agent, the pair's object, the context's owner).
const PREDICATE_PROCESS: u32 = 0x00d4_1a30;
const PREDICATE_CLEANUP: u32 = 0x00d4_1860;
/// The reset of the agent's machine words (`00da4430`, cdecl: the machine).
const RESET_MACHINE: u32 = 0x00da_4430;
/// `hkAgent1nMachine_Process` (`00da4aa0`, cdecl: the machine, the pair, the owner's
/// dispatcher words, the index list, the context).
const MACHINE_PROCESS: u32 = 0x00da_4aa0;

/// The agent functions the function table of [`fill_agent_table`] points to, by slot.
const TABLE_FUNCTIONS: [(u32, u32); 12] = [
    (0x00, 0x00d3_2b70),
    (0x2c, 0x00d3_2cd0),
    (0x28, 0x00d3_2ca0),
    (0x08, 0x00d3_2bd0),
    (0x0c, 0x00d3_2be0),
    (0x10, 0x00d3_2c00),
    (0x14, 0x00d3_2c20),
    (0x18, 0x00d3_2c40),
    (0x1c, 0x00d3_2ae0),
    (0x20, 0x00d3_2b20),
    (0x24, 0x00d3_2b40),
    (0x04, 0x00d3_2ba0),
];

// Translated from 00d32ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkConvexListAgent3::getAgent1nTrack`: cdecl with two stack words, the first unused;
/// answers the second plus `0x10` (the track of an agent entry).
pub fn get_agent_1n_track(_e: &mut Engine, _unused_0: u32, entry: u32) -> u32 {
    entry.wrapping_add(0x10)
}

// Translated from 00d32ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkConvexListAgent3::getAgent1nTrack_ov2`: the same as [`get_agent_1n_track`].
pub fn get_agent_1n_track_ov2(_e: &mut Engine, _unused_0: u32, entry: u32) -> u32 {
    entry.wrapping_add(0x10)
}

// Translated from 00d32ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkConvexListAgent3::calcStatistics`: cdecl with four stack words, the first unused. Unless
/// bit 5 of the entry's byte at `+0xb` is set, tells the collector (virtual slot `0x18`,
/// the object name), has `calc1nStatistics` (the entry's track at `+0x10`, the third word
/// `input`, the collector) report, and ends the object through the collector's slot `0x1c`
/// (a jump, so the collector's answer is left as it is). The Ghidra listing shows the entry
/// as the second argument of `calc1nStatistics`, but the disassembly passes the third word.
pub fn calc_statistics(e: &mut Engine, _unused_0: u32, entry: Ptr, input: u32, collector: Ptr) {
    if e.mem.u8(entry.addr() + 0xb) & 0x20 != 0 {
        return;
    }
    e.vcall(collector.addr(), 0x18, &args![AGENT_NAME]);
    e.call(
        CALC_1N_STATISTICS,
        &args![entry.addr() + 0x10, input, collector],
    );
    e.vcall(collector.addr(), 0x1c, &args![]);
}

// Translated from 00d32c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agent function slot `0x18` (name unknown): cdecl with seven stack words, the first unused.
/// Unless bit 5 of the entry's byte at `+0xb` is set, builds a description on the stack
/// (`input`, `owner`, an unwritten word, the answer of the object `*owner` for its virtual
/// slot `0x10`, then the last three words) and hands it with the entry's track to
/// `hkAgent1nMachine_UpdateShapeCollectionFilter`. The third word of the description is never
/// initialised by the exe and the callee does not read it; it is zero here.
pub fn update_shape_collection_filter(
    e: &mut Engine,
    _unused_0: u32,
    entry: Ptr,
    input: u32,
    owner: Ptr,
    word_5: u32,
    word_6: u32,
    word_7: u32,
) {
    if e.mem.u8(entry.addr() + 0xb) & 0x20 != 0 {
        return;
    }
    let object = e.mem.u32(owner.addr());
    let answer = e.vcall(object, 0x10, &args![]).u32();
    e.with_stack(0x20, |e, description| {
        let description = description.addr();
        let words = [input, owner.addr(), 0, answer, word_5, word_6, word_7];
        for (index, word) in words.into_iter().enumerate() {
            e.mem.set_u32(description + 4 * index as u32, word);
        }
        e.call(
            UPDATE_SHAPE_COLLECTION_FILTER,
            &args![entry.addr() + 0x10, description],
        );
    });
}

// Translated from 00d33140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the agent function table `table` (cdecl, one word): twelve function addresses
/// (slot `0` `00d32b70`, `4` `00d32ba0`, `8` `00d32bd0`, `0xc` `00d32be0`, `0x10`
/// `00d32c00`, `0x14` `00d32c20`, `0x18` `00d32c40`, `0x1c` `00d32ae0` (statistics), `0x20`
/// `00d32b20`, `0x24` `00d32b40`, `0x28` `00d32ca0`, `0x2c` `00d32cd0` (process)) and the byte
/// `1` at `+0x30`.
pub fn fill_agent_table(e: &mut Engine, table: Ptr) {
    let table = table.addr();
    for (offset, function) in TABLE_FUNCTIONS {
        e.mem.set_u32(table + offset, function);
    }
    e.mem.set_u8(table + 0x30, 1);
}

// Translated from 00d331a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the convex list agent (the table of [`fill_agent_table`]) with the dispatcher
/// `dispatcher` (cdecl, one word) for the shape type pairs `(0x12, 0x16)` and `(0x16, 0x16)`.
/// The table is built in a stack block of which the exe first zeroes the words at `+0x18` to
/// `+0x28` and the bytes at `+0x30` to `+0x32` (the table overwrites most of that); the
/// callee copies `0x40` bytes from it, the last twelve of which the exe never writes (zero
/// here, and not used by the callee). The names are by use: no source name is known.
pub fn register_convex_list_agent(e: &mut Engine, dispatcher: Ptr) {
    e.with_stack(0x50, |e, table| {
        let table_addr = table.addr();
        for offset in 0..3 {
            e.mem.set_u8(table_addr + 0x30 + offset, 0);
        }
        for offset in [0x18, 0x1c, 0x20, 0x24, 0x28] {
            e.mem.set_u32(table_addr + offset, 0);
        }
        fill_agent_table(e, table);
        e.call(REGISTER_AGENT, &args![dispatcher, table, 0x12u32, 0x16u32]);
        e.call(REGISTER_AGENT, &args![dispatcher, table, 0x16u32, 0x16u32]);
    });
}

// Translated from 00d33210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMapPointsToSubShapeContactMgr::cleanup`: tail-calls the wrapped manager (`this+0xc`,
/// its virtual slot `0x24`, no stack word). Answers what that answers.
pub fn cleanup(e: &mut Engine, this: Ptr) -> u32 {
    let inner = e.mem.u32(this.addr() + 0xc);
    e.vcall(inner, 0x24, &args![]).u32()
}

// Translated from 00d33220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMapPointsToSubShapeContactMgr::reserveContactPointsImpl`: tail-calls the wrapped manager
/// (`this+0xc`, virtual slot `0x10`) with the one stack word `count`; the number of words
/// is that of the contact manager implementations (`RET 4`).
pub fn reserve_contact_points(e: &mut Engine, this: Ptr, count: u32) -> u32 {
    let inner = e.mem.u32(this.addr() + 0xc);
    e.vcall(inner, 0x10, &args![count]).u32()
}

// Translated from 00d33230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMapPointsToSubShapeContactMgr::removeContactPointImpl`: tail-calls the wrapped manager
/// (`this+0xc`, virtual slot `0x14`) with the two stack words of the contact manager
/// implementations (`RET 8`).
pub fn remove_contact_point(e: &mut Engine, this: Ptr, point_id: u32, owner: u32) -> u32 {
    let inner = e.mem.u32(this.addr() + 0xc);
    e.vcall(inner, 0x14, &args![point_id, owner]).u32()
}

// Translated from 00d33240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Contact manager slot `0x18` (`processContactImpl` in the simple contact manager; no name
/// known here): tail-calls the wrapped manager (`this+0xc`, virtual slot `0x18`) with the four
/// stack words of the contact manager implementations (`RET 0x10`).
pub fn process_contact(
    e: &mut Engine,
    this: Ptr,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
) -> u32 {
    let inner = e.mem.u32(this.addr() + 0xc);
    e.vcall(inner, 0x18, &args![word_1, word_2, word_3, word_4])
        .u32()
}

// Translated from 00d33250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMapPointsToSubShapeContactMgr::removeToiImpl`: tail-calls the wrapped manager
/// (`this+0xc`, virtual slot `0x20`) with the two stack words of the contact manager
/// implementations (`RET 8`).
pub fn remove_toi(e: &mut Engine, this: Ptr, word_1: u32, word_2: u32) -> u32 {
    let inner = e.mem.u32(this.addr() + 0xc);
    e.vcall(inner, 0x20, &args![word_1, word_2]).u32()
}

// Translated from 00d33260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy of a record (thiscall, one stack word; name and class unknown): the five words at
/// `+0` to `+0x10`, then the `0x40` bytes from `+0x20` (read as sixteen words first, then
/// stored). The words at `+0x14` to `+0x1c` are not copied. Answers `this`.
pub fn copy_record_00d33260(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let (to, from) = (this.addr(), source.addr());
    for offset in [0, 4, 8, 0xc, 0x10] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    let staged: Vec<u32> = (0..16)
        .map(|word| e.mem.u32(from + 0x20 + 4 * word))
        .collect();
    for (word, value) in staged.into_iter().enumerate() {
        e.mem.set_u32(to + 0x20 + 4 * word as u32, value);
    }
    this
}

// Translated from 00d33330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy of a record (thiscall, one stack word; name and class unknown): the vectors at `+0`
/// and `+0x10`, the words at `+0x20`, `+0x24`, `+0x30`, `+0x34`, `+0x38`, `+0x3c`, then the
/// `0x30` bytes from `+0x40`. Answers `this`.
pub fn copy_record_00d33330(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let (to, from) = (this.addr(), source.addr());
    for offset in [0, 0x10] {
        for word in 0..4 {
            let value = e.mem.u32(from + offset + 4 * word);
            e.mem.set_u32(to + offset + 4 * word, value);
        }
    }
    for offset in [0x20, 0x24, 0x30, 0x34, 0x38, 0x3c] {
        let value = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, value);
    }
    for word in 0..12 {
        let value = e.mem.u32(from + 0x40 + 4 * word);
        e.mem.set_u32(to + 0x40 + 4 * word, value);
    }
    this
}

/// Replaces a body of the shape type `0x16` (a list shape) by a stand-in made of its child
/// shape: all the key bytes of its group in `info` (pairs from `info+1`; the group of the
/// first body starts at pair 0 and has `info[8]` pairs, that of the second starts at pair
/// `info[8]` and has `info[9]`) must be equal, else `None`; the child comes from the shape's
/// embedded container (at `+0x14`, virtual slot `0x14`: key, scratch buffer). The stand-in
/// at `stand_in` has the child, the key, the third word of the body and the body itself.
/// `first` selects the group. Bodies of other types are used as they are.
fn child_body(
    e: &mut Engine,
    body: u32,
    info: u32,
    first: bool,
    stand_in: u32,
    buffer: u32,
) -> Option<u32> {
    let shape = e.mem.u32(body);
    if e.mem.u32(shape + 0xc) != 0x16 {
        return Some(body);
    }
    let first_count = i32::from(e.mem.u8(info + 8));
    let (start, end) = if first {
        (1, first_count)
    } else {
        (first_count + 1, i32::from(e.mem.u8(info + 9)) + first_count)
    };
    let key_offset = if first { 0 } else { first_count as u32 };
    let key = u32::from(e.mem.u8(info + 1 + 2 * key_offset));
    for pair in start..end {
        if u32::from(e.mem.u8(info + 1 + 2 * pair as u32)) != key {
            return None;
        }
    }
    let child = e.vcall(shape + 0x14, 0x14, &args![key, buffer]).u32();
    let third = e.mem.u32(body + 8);
    e.mem.set_u32(stand_in, child);
    e.mem.set_u32(stand_in + 4, key);
    e.mem.set_u32(stand_in + 8, third);
    e.mem.set_u32(stand_in + 0xc, body);
    Some(stand_in)
}

// Translated from 00d33390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMapPointsToSubShapeContactMgr::addContactPointImpl`: thiscall, six stack words. A body
/// that is a list shape (type `0x16`) is replaced by a stand-in for the child shape named by
/// the key bytes of `info` (see [`child_body`]); when a group has differing keys the flag
/// byte at `this+0x10` is set and `0xffff` answered. Otherwise forwards both bodies and the
/// other words to the wrapped manager (`this+0xc`, virtual slot `0xc`) and answers its result.
pub fn add_contact_point(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    word_3: u32,
    word_4: u32,
    info: Ptr,
    word_6: u32,
) -> u32 {
    e.with_stack(0x240, |e, frame| {
        let frame = frame.addr();
        let (stand_in_b, stand_in_a, buffer) = (frame, frame + 0x10, frame + 0x20);
        let Some(a) = child_body(e, body_a.addr(), info.addr(), true, stand_in_a, buffer) else {
            e.mem.set_u8(this.addr() + 0x10, 1);
            return 0xffff;
        };
        let Some(b) = child_body(e, body_b.addr(), info.addr(), false, stand_in_b, buffer) else {
            e.mem.set_u8(this.addr() + 0x10, 1);
            return 0xffff;
        };
        let inner = e.mem.u32(this.addr() + 0xc);
        e.vcall(inner, 0xc, &args![a, b, word_3, word_4, info, word_6])
            .u32()
    })
}

// Translated from 00d334c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMapPointsToSubShapeContactMgr::addToiImpl`: thiscall, nine stack words. The same
/// stand-ins as [`add_contact_point`] (the flag byte at `this+0x10` is set and `1` answered
/// on differing keys), then forwards to the wrapped manager's virtual slot `0x1c`; the
/// float `time` is passed on through the x87 stack (a signalling NaN comes out quiet).
pub fn add_toi(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    word_3: u32,
    word_4: u32,
    time: u32,
    word_6: u32,
    info: Ptr,
    word_8: u32,
    word_9: u32,
) -> u32 {
    e.with_stack(0x240, |e, frame| {
        let frame = frame.addr();
        let (stand_in_b, stand_in_a, buffer) = (frame, frame + 0x10, frame + 0x20);
        let Some(a) = child_body(e, body_a.addr(), info.addr(), true, stand_in_a, buffer) else {
            e.mem.set_u8(this.addr() + 0x10, 1);
            return 1;
        };
        let Some(b) = child_body(e, body_b.addr(), info.addr(), false, stand_in_b, buffer) else {
            e.mem.set_u8(this.addr() + 0x10, 1);
            return 1;
        };
        let inner = e.mem.u32(this.addr() + 0xc);
        let time = x87_quiet_bits(time);
        e.vcall(
            inner,
            0x1c,
            &args![a, b, word_3, word_4, time, word_6, info, word_8, word_9],
        )
        .u32()
    })
}

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// `TlsGetValue` of the profiling index at `0126835c`: the thread's record stream.
fn profile_stream(e: &mut Engine) -> u32 {
    let slot: u32 = e.global(PROFILE_SLOT);
    e.call(TLS_GET_VALUE, &args![slot]).u32()
}

/// Appends a profiling record to `stream` while its cursor (`+4`) is below its limit (`+0xc`):
/// the label and the time stamp (`+4`), after the first label a second one at `+0xc` when
/// `second_label` is given; the record is `size` bytes.
fn append_record(e: &mut Engine, stream: u32, label: u32, second_label: Option<u32>, size: u32) {
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        if let Some(second) = second_label {
            e.mem.set_u32(cursor + 0xc, second);
        }
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + size);
    }
}

/// Where `process` leaves its loop.
enum Leave {
    /// The agent's counter ran out or no closest point was found.
    Counter,
    /// Nothing more to do for this call.
    Finish,
}

// Translated from 00d32cd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d32cd0 - when the agent entry has bit 5 set on entry, the first
// pass hands the callee a body descriptor whose second word keeps the stack's old low half
// (the exe never writes it on that path); zero is used here.
/// `hkConvexListAgent3::process`: cdecl with five words (`pair`, `input`, `agent`, `query`,
/// `context`). Profiling records frame the work (`"LtCvxLst"`/`"StTim"`, one `"StStream"` per
/// pass, the end label). Unless bit 5 of the agent's flag byte (`+0xb`) is set on entry, a
/// pass counts down the agent's 16-bit counter (`+0xc`): while it is not negative the loop
/// is left; otherwise it is reset to `0x19`, the relative transform of the pair's bodies is
/// computed and `getClosestPoint` is asked; with no point the loop is left, with one the
/// 1-n machine of the agent (`+0x10`) is destroyed, bit 5 set and the counter word cleared.
/// Then (in every pass) a body descriptor and a copy of the pair's record are built on the
/// stack and handed to `hkPredGskAgent3::process` (`00d41a30`). If its descriptor byte is
/// zero the pass ends the call when the agent's byte `+0xe` is zero or bit 6 of the flag
/// byte is set, or when the smaller of the two bodies' root floats (at `+0x4c` of the top of
/// each parent chain) doubled and negated does not exceed the query's float at `+0xc`.
/// Otherwise the results are published in the context (the changed float at `+0xc0c`
/// pulls the contact data, the vtable call `0x20` of the pair's object and a twelve-word
/// block, the first word, the `0x1008` bytes to the buffer at `+0xc20` when it is set), the
/// predicate agent is cleaned up (`00d41860`), bit 5 cleared, the machine words at `+0x10`
/// reset and the counter set to `0x19`; the loop then runs again. The loop's exit through
/// the counter writes the no-hit float `0xfeffffee` to the query's `+0xc`, runs
/// `hkAgent1nMachine_Process` over the identity index list of the owner's `0x48` entries (taken
/// from the memory router and released again) and answers `agent + 0x1c`; the other exit
/// answers what the predicate agent answered. Bit 6 is set before the end record. Answers the
/// value rounded up to 16.
pub fn process(e: &mut Engine, pair: Ptr, input: u32, agent: Ptr, query: Ptr, context: Ptr) -> u32 {
    e.with_stack(0x1144, |e, frame| {
        // The exe's `ESP` once the prologue is done lies 12 bytes below the frame.
        let s = frame.addr().wrapping_sub(0xc);
        let (pair, agent, query, context) =
            (pair.addr(), agent.addr(), query.addr(), context.addr());
        let stream = profile_stream(e);
        append_record(e, stream, LABEL_LT_CVX_LST, Some(LABEL_ST_TIM), 0x10);
        let second_body = e.mem.u32(pair + 4);
        let owner = e.mem.u32(second_body);
        e.mem.set_u32(s + 0x24, owner);
        e.call(INIT_LOCAL_OBJECT, &args![s + 0xd0, context]);
        let mut entered_flagged = e.mem.u8(agent + 0xb) & 0x20 != 0;
        let leave = 'run: loop {
            if !entered_flagged {
                let stream = profile_stream(e);
                e.mem.set_u32(s + 0x28, stream);
                append_record(e, stream, LABEL_ST_STREAM, None, 0xc);
                let counter = e.mem.u16(agent + 0xc);
                e.mem.set_u16(agent + 0xc, counter.wrapping_sub(1));
                if (counter as i16) >= 0 {
                    break 'run Leave::Counter;
                }
                e.mem.set_u16(agent + 0xc, 0x19);
                let first_body = e.mem.u32(pair);
                let second_body = e.mem.u32(pair + 4);
                let first_transform = e.mem.u32(first_body + 8);
                let second_transform = e.mem.u32(second_body + 8);
                e.call(
                    SET_MUL_INVERSE_MUL,
                    &args![s + 0x40, first_transform, second_transform],
                );
                let first_body = e.mem.u32(pair);
                let second_body = e.mem.u32(pair + 4);
                let first_word = e.mem.u32(first_body);
                e.mem.set_u32(s + 0x34, first_word);
                let second_word = e.mem.u32(second_body);
                e.mem.set_u32(s + 0x38, second_word);
                let third = e.mem.u32(pair + 0xc);
                e.mem.set_u32(s + 0x2c, s + 0x40);
                let transform = e.mem.u32(first_body + 8);
                e.mem.set_u32(s + 0x30, transform);
                let reading = x87_quiet_bits(e.mem.u32(third + 8));
                e.mem.set_u32(s + 0x3c, reading);
                let found = e
                    .call(GET_CLOSEST_POINT, &args![s + 0x2c, agent, query, s + 0xc0])
                    .u32();
                if found == 0 {
                    break 'run Leave::Counter;
                }
                let machine_owner = e.mem.u32(context + 4);
                let object = e.mem.u32(pair + 0x10);
                let third = e.mem.u32(pair + 0xc);
                let dispatcher = e.mem.u32(third);
                e.call(
                    DESTROY_MACHINE,
                    &args![agent + 0x10, dispatcher, object, machine_owner],
                );
                let flags = e.mem.u8(agent + 0xb) | 0x20;
                e.mem.set_u8(agent + 0xb, flags);
                e.mem.set_u32(agent + 0xc, 0);
                let first_body = e.mem.u32(pair);
                let second_body = e.mem.u32(pair + 4);
                let object = e.mem.u32(pair + 0x10);
                e.call(
                    RESUME_LOCAL_OBJECT,
                    &args![s + 0xd0, first_body, second_body, context, object],
                );
            }
            entered_flagged = false;
            // The body descriptor and the copy of the pair's record.
            e.mem.set_u16(s + 0x32, 1);
            e.mem.set_u32(s + 0x34, 2);
            e.mem.set_u32(s + 0x2c, BODY_DESCRIPTOR_TABLE);
            let object = e.mem.u32(pair + 0x10);
            e.mem.set_u32(s + 0x38, object);
            e.mem.set_u8(s + 0x3c, 0);
            copy_record_00d33260(e, Ptr::new(s + 0x40), Ptr::new(pair));
            for (from, to) in [
                (0x60, 0xa0),
                (0x70, 0xb0),
                (0x74, 0xb4),
                (0x78, 0xb8),
                (0x7c, 0xbc),
            ] {
                let word = e.mem.u32(pair + from);
                e.mem.set_u32(s + to, word);
            }
            e.mem.set_u32(s + 0x50, s + 0x2c);
            let answer = e
                .call(
                    PREDICATE_PROCESS,
                    &args![s + 0x40, input, agent, query, context],
                )
                .u32();
            e.mem.set_u32(s + 0x28, answer);
            let word = e.mem.u32(s + 0x38);
            e.mem.set_u32(s + 0x50, word);
            if e.mem.u8(s + 0x3c) == 0 {
                if e.mem.u8(agent + 0xe) == 0 || e.mem.u8(agent + 0xb) & 0x40 != 0 {
                    break 'run Leave::Finish;
                }
                // The smaller of the two parent chains' root floats, doubled and negated.
                let mut root = e.mem.u32(pair + 4);
                loop {
                    let parent = e.mem.u32(root + 0xc);
                    if parent == 0 {
                        break;
                    }
                    root = parent;
                }
                let second_float = f32::from_bits(x87_quiet_bits(e.mem.u32(root + 0x4c)));
                let mut root = e.mem.u32(pair);
                loop {
                    let parent = e.mem.u32(root + 0xc);
                    if parent == 0 {
                        break;
                    }
                    root = parent;
                }
                let first_float = f32::from_bits(x87_quiet_bits(e.mem.u32(root + 0x4c)));
                let smaller = if second_float > first_float {
                    first_float
                } else {
                    second_float
                };
                let doubled = (f64::from(smaller) + f64::from(smaller)) as f32;
                let limit = f32::from_bits(e.mem.u32(query + 0xc));
                // `FCOMPP` then a test of the C0 and C3 bits: only a strictly greater,
                // ordered value goes on.
                let goes_on = -doubled > limit;
                if !goes_on {
                    break 'run Leave::Finish;
                }
            }
            // Publish the results in the context when the contact float changed.
            let current = f32::from_bits(e.mem.u32(context + 0x3030));
            let found = f32::from_bits(e.mem.u32(s + 0x1100));
            if found != current {
                let object = e.mem.u32(pair + 0x10);
                let machine_owner = e.mem.u32(context + 4);
                e.vcall(object, 0x20, &args![machine_owner, context + 0x3050]);
                for (from, to) in [
                    (0x1100, 0x3030),
                    (0x1104, 0x3034),
                    (0x1110, 0x3040),
                    (0x1114, 0x3044),
                    (0x1118, 0x3048),
                    (0x111c, 0x304c),
                ] {
                    let word = e.mem.u32(s + from);
                    e.mem.set_u32(context + to, word);
                }
                for (from, to) in [(0x10e0, 0x3010), (0x10f0, 0x3020)] {
                    for lane in 0..4 {
                        let word = e.mem.u32(s + from + 4 * lane);
                        e.mem.set_u32(context + to + 4 * lane, word);
                    }
                }
                for word in 0..12 {
                    let value = e.mem.u32(s + 0x1120 + 4 * word);
                    e.mem.set_u32(context + 0x3050 + 4 * word, value);
                }
            }
            let word = e.mem.u32(s + 0xd0);
            e.mem.set_u32(context, word);
            let buffer = e.mem.u32(context + 0x3080);
            if buffer != 0 {
                e.call(MEMCPY, &args![buffer, s + 0xd4, 0x1008u32]);
            }
            let machine_owner = e.mem.u32(context + 4);
            let object = e.mem.u32(pair + 0x10);
            e.call(
                PREDICATE_CLEANUP,
                &args![input, agent, object, machine_owner],
            );
            let flags = e.mem.u8(agent + 0xb) & 0xdf;
            e.mem.set_u8(agent + 0xb, flags);
            if agent.wrapping_add(0xc) != 0 {
                e.mem.set_u32(agent + 0x10, 0);
                e.mem.set_u32(agent + 0x14, 0);
                e.mem.set_u32(agent + 0x18, 0x8000_0000);
            }
            e.call(RESET_MACHINE, &args![agent + 0x10]);
            e.mem.set_u16(agent + 0xc, 0x19);
        };
        if matches!(leave, Leave::Counter) {
            let no_hit = x87_quiet_bits(e.mem.u32(NO_HIT_TIME));
            e.mem.set_u32(query + 0xc, no_hit);
            let owner = e.mem.u32(s + 0x24);
            let entries = e.mem.u32(owner + 0x48);
            e.mem.set_u32(s + 0x18, entries);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let entries = e.mem.u32(s + 0x18);
            let list = e.mem.u32(router);
            let size = entries.wrapping_mul(4).wrapping_add(0x83) & 0xffff_ff80;
            e.mem.set_u32(router, size.wrapping_add(list));
            let entries = e.mem.u32(s + 0x18);
            e.mem.set_u32(s + 0x1c, list);
            let mut index = 0u32;
            while (index as i32) < (entries as i32) {
                e.mem.set_u32(list + 4 * index, index);
                index += 1;
            }
            e.mem.set_u32(list + entries.wrapping_mul(4), 0xffff_ffff);
            let machine_arguments = e.mem.u32(s + 0x24).wrapping_add(0x14);
            e.call(
                MACHINE_PROCESS,
                &args![agent + 0x10, pair, machine_arguments, list, context],
            );
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let list = e.mem.u32(s + 0x1c);
            e.mem.set_u32(router, list);
            e.mem.set_u32(s + 0x28, agent + 0x1c);
        }
        let flags = e.mem.u8(agent + 0xb) | 0x40;
        e.mem.set_u8(agent + 0xb, flags);
        let stream = profile_stream(e);
        append_record(e, stream, LABEL_END, None, 0xc);
        e.mem.u32(s + 0x28).wrapping_add(0xf) & 0xffff_fff0
    })
}

/// `hkAgent1nMachine_InvalidateTim` (Ghidra's name; `00da3510`, cdecl: the track, one word).
const INVALIDATE_TIM: u32 = 0x00da_3510;
/// `hkAgent1nMachine_WarpTime` (Ghidra's name; `00da35f0`, cdecl: the track, two floats as
/// stack words, one word).
const WARP_TIME: u32 = 0x00da_35f0;
/// `hkPredGskAgent3::create` (Ghidra's name; `00d418d0`, cdecl: three words, answers a pointer).
const PREDICATE_CREATE: u32 = 0x00d4_18d0;
/// `00d418b0` (Ghidra made no function of it; cdecl, four words, which it hands unchanged to
/// [`PREDICATE_CLEANUP`]).
const PREDICATE_DESTROY: u32 = 0x00d4_18b0;
/// The predicate agent's functions for the agent table's slots `0xc`, `0x10` and `0x14`
/// (`00d417b0`, `00d417f0`, `00d41830`; cdecl, three words: the first never read, the entry, a
/// 16-bit id; `00d41830` has no Ghidra function).
const PREDICATE_SLOT_0C: u32 = 0x00d4_17b0;
const PREDICATE_SLOT_10: u32 = 0x00d4_17f0;
const PREDICATE_SLOT_14: u32 = 0x00d4_1830;
/// `hkPredGskAgent3::sepNormal` (Ghidra's name; `00d41930`, cdecl, four words).
const PREDICATE_SEP_NORMAL: u32 = 0x00d4_1930;
/// The 16 bytes (`MOVAPS`) that slot `0x28` copies to its answer buffer when the agent has
/// been cleaned up, and the float it stores over the last four bytes is [`NO_HIT_TIME`].
const NO_HIT_ANSWER: u32 = 0x0126_8370;

// Translated from 00d32b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agent function slot `0x20` (name unknown): cdecl with three stack words, the first unused.
/// Unless bit 5 of the entry's byte at `+0xb` is set, hands the entry's track (`entry + 0x10`)
/// and the third word to `hkAgent1nMachine_InvalidateTim` (`00da3510`). Answers what `EAX` holds
/// at the end: the entry when the bit was set, otherwise the callee's answer.
pub fn fn_00d32b20(e: &mut Engine, _unused_0: u32, entry: Ptr, word_3: u32) -> u32 {
    if e.mem.u8(entry.addr() + 0xb) & 0x20 != 0 {
        return entry.addr();
    }
    e.call(INVALIDATE_TIM, &args![entry.addr() + 0x10, word_3])
        .u32()
}

// Translated from 00d32b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agent function slot `0x24` (name unknown): cdecl with five stack words, the first unused.
/// Unless bit 5 of the entry's byte at `+0xb` is set, passes the entry's track, the two float
/// words (copied through the x87 stack, so a signalling NaN comes out quiet) and the fifth word
/// to `hkAgent1nMachine_WarpTime` (`00da35f0`). Answers what `EAX` holds at the end: the entry
/// when the bit was set, otherwise the callee's answer.
pub fn fn_00d32b40(
    e: &mut Engine,
    _unused_0: u32,
    entry: Ptr,
    time_1: u32,
    time_2: u32,
    word_5: u32,
) -> u32 {
    if e.mem.u8(entry.addr() + 0xb) & 0x20 != 0 {
        return entry.addr();
    }
    let first = x87_quiet_bits(time_1);
    let second = x87_quiet_bits(time_2);
    e.call(
        WARP_TIME,
        &args![entry.addr() + 0x10, first, second, word_5],
    )
    .u32()
}

// Translated from 00d32b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agent function slot `0` (the create function; name unknown): cdecl with three stack words,
/// `input`, `word_2` and `entry`. Has `hkPredGskAgent3::create` (`00d418d0`) work on all three,
/// then sets bit 5 and clears bit 6 of the entry's byte at `+0xb` (read after the call, which
/// changes that byte too). Answers what the callee answers.
pub fn fn_00d32b70(e: &mut Engine, input: u32, word_2: u32, entry: Ptr) -> u32 {
    let result = e
        .call(PREDICATE_CREATE, &args![input, word_2, entry.addr()])
        .u32();
    let flags = e.mem.u8(entry.addr() + 0xb);
    e.mem.set_u8(entry.addr() + 0xb, (flags & 0xbf) | 0x20);
    result
}

// Translated from 00d32ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agent function slot `4` (name unknown): cdecl with five stack words, the first unused by
/// the second branch. When bit 5 of the entry's byte at `+0xb` is set, jumps to `00d418b0` with
/// the first four words. Otherwise calls `hkAgent1nMachine_Destroy` (`00da42b0`, cdecl: the
/// machine, the dispatcher, the pair's object, the context's owner) with the entry's track
/// (`entry + 0x10`), the fifth word, the third and the fourth. Answers what the callee answers.
pub fn fn_00d32ba0(
    e: &mut Engine,
    word_1: u32,
    entry: Ptr,
    word_3: u32,
    word_4: u32,
    word_5: u32,
) -> u32 {
    if e.mem.u8(entry.addr() + 0xb) & 0x20 != 0 {
        return e
            .call(
                PREDICATE_DESTROY,
                &args![word_1, entry.addr(), word_3, word_4],
            )
            .u32();
    }
    e.call(
        DESTROY_MACHINE,
        &args![entry.addr() + 0x10, word_5, word_3, word_4],
    )
    .u32()
}

// Translated from 00d32bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agent function slot `8` (name unknown): cdecl with four stack words, all handed on by a
/// jump to `hkPredGskAgent3::cleanup` (`00d41860`); answers what that answers.
pub fn fn_00d32bd0(e: &mut Engine, word_1: u32, word_2: u32, word_3: u32, word_4: u32) -> u32 {
    e.call(PREDICATE_CLEANUP, &args![word_1, word_2, word_3, word_4])
        .u32()
}

// Translated from 00d32be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agent function slot `0xc` (name unknown): cdecl with three stack words, the first and the
/// third handed on. When bit 5 of the entry's byte at `+0xb` is set, jumps to `00d417b0` with
/// the three words and answers what it answers; otherwise answers the entry (the value `EAX`
/// holds).
pub fn fn_00d32be0(e: &mut Engine, word_1: u32, entry: Ptr, word_3: u32) -> u32 {
    if e.mem.u8(entry.addr() + 0xb) & 0x20 == 0 {
        return entry.addr();
    }
    e.call(PREDICATE_SLOT_0C, &args![word_1, entry.addr(), word_3])
        .u32()
}

// Translated from 00d32c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agent function slot `0x10` (name unknown): as [`fn_00d32be0`] with the jump to `00d417f0`.
pub fn fn_00d32c00(e: &mut Engine, word_1: u32, entry: Ptr, word_3: u32) -> u32 {
    if e.mem.u8(entry.addr() + 0xb) & 0x20 == 0 {
        return entry.addr();
    }
    e.call(PREDICATE_SLOT_10, &args![word_1, entry.addr(), word_3])
        .u32()
}

// Translated from 00d32c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agent function slot `0x14` (name unknown): as [`fn_00d32be0`] with the jump to `00d41830`.
pub fn fn_00d32c20(e: &mut Engine, word_1: u32, entry: Ptr, word_3: u32) -> u32 {
    if e.mem.u8(entry.addr() + 0xb) & 0x20 == 0 {
        return entry.addr();
    }
    e.call(PREDICATE_SLOT_14, &args![word_1, entry.addr(), word_3])
        .u32()
}

// Translated from 00d32ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Agent function slot `0x28` (name unknown): cdecl with four stack words. When bit 5 of the
/// byte at `+0xb` of the entry (the third word) is set, jumps to `hkPredGskAgent3::sepNormal`
/// (`00d41930`) with the four words and answers what it answers. Otherwise fills the answer
/// buffer (the fourth word): 16 bytes copied from `0126_8370`, then the float at `010ceb6c`
/// (copied through the x87 stack, so a signalling NaN comes out quiet) over its last four
/// bytes; answers the buffer's address.
pub fn fn_00d32ca0(e: &mut Engine, word_1: u32, word_2: u32, entry: Ptr, answer: Ptr) -> u32 {
    if e.mem.u8(entry.addr() + 0xb) & 0x20 != 0 {
        return e
            .call(
                PREDICATE_SEP_NORMAL,
                &args![word_1, word_2, entry.addr(), answer.addr()],
            )
            .u32();
    }
    let time = x87_quiet_bits(e.mem.u32(NO_HIT_TIME));
    for lane in 0..4u32 {
        let word = e.mem.u32(NO_HIT_ANSWER + 4 * lane);
        e.mem.set_u32(answer.addr() + 4 * lane, word);
    }
    e.mem.set_u32(answer.addr() + 0xc, time);
    answer.addr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d3_2ac0, get_agent_1n_track(u32, u32) -> u32),
        entry!(0x00d3_2ad0, get_agent_1n_track_ov2(u32, u32) -> u32),
        entry!(0x00d3_2ae0, calc_statistics(u32, Ptr, u32, Ptr)),
        entry!(
            0x00d3_2c40,
            update_shape_collection_filter(u32, Ptr, u32, Ptr, u32, u32, u32)
        ),
        entry!(0x00d3_2cd0, process(Ptr, u32, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d3_3140, fill_agent_table(Ptr)),
        entry!(0x00d3_31a0, register_convex_list_agent(Ptr)),
        entry!(0x00d3_3210, cleanup(Ptr) -> u32),
        entry!(0x00d3_3220, reserve_contact_points(Ptr, u32) -> u32),
        entry!(0x00d3_3230, remove_contact_point(Ptr, u32, u32) -> u32),
        entry!(0x00d3_3240, process_contact(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00d3_3250, remove_toi(Ptr, u32, u32) -> u32),
        entry!(0x00d3_3260, copy_record_00d33260(Ptr, Ptr) -> Ptr),
        entry!(0x00d3_3330, copy_record_00d33330(Ptr, Ptr) -> Ptr),
        entry!(
            0x00d3_3390,
            add_contact_point(Ptr, Ptr, Ptr, u32, u32, Ptr, u32) -> u32
        ),
        entry!(
            0x00d3_34c0,
            add_toi(Ptr, Ptr, Ptr, u32, u32, u32, u32, Ptr, u32, u32) -> u32
        ),
        entry!(0x00d32b20, fn_00d32b20(u32, Ptr, u32) -> u32),
        entry!(0x00d32b40, fn_00d32b40(u32, Ptr, u32, u32, u32) -> u32),
        entry!(0x00d32b70, fn_00d32b70(u32, u32, Ptr) -> u32),
        entry!(0x00d32ba0, fn_00d32ba0(u32, Ptr, u32, u32, u32) -> u32),
        entry!(0x00d32bd0, fn_00d32bd0(u32, u32, u32, u32) -> u32),
        entry!(0x00d32be0, fn_00d32be0(u32, Ptr, u32) -> u32),
        entry!(0x00d32c00, fn_00d32c00(u32, Ptr, u32) -> u32),
        entry!(0x00d32c20, fn_00d32c20(u32, Ptr, u32) -> u32),
        entry!(0x00d32ca0, fn_00d32ca0(u32, u32, Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles at the addresses that log their arguments and answer the given value.
    fn log_doubles(e: &mut Engine, doubles: &[(u32, u32)]) -> Calls {
        let log: Calls = Rc::new(RefCell::new(Vec::new()));
        for &(address, answer) in doubles {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax: answer,
                    ..Ret::default()
                }
            });
        }
        log
    }

    /// An object whose vtable holds `(slot offset, target)`.
    fn object_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x40);
        for &(offset, target) in slots {
            e.mem.set_u32(table + offset, target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, table);
        object
    }

    #[test]
    fn the_track_getters_answer_the_second_word_plus_0x10() {
        let mut e = Engine::new();
        for address in [0x00d3_2ac0, 0x00d3_2ad0] {
            assert_eq!(e.call(address, &args![1u32, 0x5000u32]).u32(), 0x5010);
        }
    }

    #[test]
    fn calc_statistics_brackets_the_report_with_the_collector_calls() {
        let mut e = Engine::new();
        let log = log_doubles(
            &mut e,
            &[(0x0900_0018, 0), (0x0900_001c, 0), (CALC_1N_STATISTICS, 0)],
        );
        let collector = object_with(&mut e, &[(0x18, 0x0900_0018), (0x1c, 0x0900_001c)]);
        let entry = e.mem.alloc(0x40);
        e.call(0x00d3_2ae0, &args![0u32, entry, 0x7777u32, collector]);
        assert_eq!(
            *log.borrow(),
            vec![
                (0x0900_0018, vec![collector, AGENT_NAME]),
                (CALC_1N_STATISTICS, vec![entry + 0x10, 0x7777, collector]),
                (0x0900_001c, vec![collector]),
            ]
        );
        log.borrow_mut().clear();
        e.mem.set_u8(entry + 0xb, 0x20);
        e.call(0x00d3_2ae0, &args![0u32, entry, 0x7777u32, collector]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn the_filter_update_hands_over_the_description_words() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(UPDATE_SHAPE_COLLECTION_FILTER, move |e, a| {
            let words: Vec<u32> = (0..7).map(|word| e.mem.u32(a[1] + 4 * word)).collect();
            sink.borrow_mut().push((a[0], words));
            Ret::default()
        });
        let log = log_doubles(&mut e, &[(0x0900_0010, 0xabcd)]);
        let object = object_with(&mut e, &[(0x10, 0x0900_0010)]);
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, object);
        let entry = e.mem.alloc(0x40);
        e.call(
            0x00d3_2c40,
            &args![0u32, entry, 0x11u32, owner, 5u32, 6u32, 7u32],
        );
        assert_eq!(
            *seen.borrow(),
            vec![(entry + 0x10, vec![0x11, owner, 0, 0xabcd, 5, 6, 7])]
        );
        assert_eq!(*log.borrow(), vec![(0x0900_0010, vec![object])]);
        e.mem.set_u8(entry + 0xb, 0x20);
        e.call(
            0x00d3_2c40,
            &args![0u32, entry, 0x11u32, owner, 5u32, 6u32, 7u32],
        );
        assert_eq!(seen.borrow().len(), 1);
    }

    /// The function table the exe builds: `(offset, function)` pairs.
    const TABLE: [(u32, u32); 12] = [
        (0x00, 0x00d3_2b70),
        (0x04, 0x00d3_2ba0),
        (0x08, 0x00d3_2bd0),
        (0x0c, 0x00d3_2be0),
        (0x10, 0x00d3_2c00),
        (0x14, 0x00d3_2c20),
        (0x18, 0x00d3_2c40),
        (0x1c, 0x00d3_2ae0),
        (0x20, 0x00d3_2b20),
        (0x24, 0x00d3_2b40),
        (0x28, 0x00d3_2ca0),
        (0x2c, 0x00d3_2cd0),
    ];

    #[test]
    fn the_agent_table_holds_the_twelve_functions_and_the_flag() {
        let mut e = Engine::new();
        let table = e.mem.alloc(0x40);
        e.call(0x00d3_3140, &args![table]);
        for (offset, function) in TABLE {
            assert_eq!(e.mem.u32(table + offset), function, "{offset:#x}");
        }
        assert_eq!(e.mem.u8(table + 0x30), 1);
    }

    #[test]
    fn the_registration_installs_the_table_for_two_type_pairs() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(REGISTER_AGENT, move |e, a| {
            let table: Vec<u32> = (0..13).map(|word| e.mem.u32(a[1] + 4 * word)).collect();
            sink.borrow_mut().push((a[0], a[2], a[3], table));
            Ret::default()
        });
        let dispatcher = e.mem.alloc(0x20);
        e.call(0x00d3_31a0, &args![dispatcher]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0].0, dispatcher);
        assert_eq!((seen[0].1, seen[0].2), (0x12, 0x16));
        assert_eq!((seen[1].1, seen[1].2), (0x16, 0x16));
        assert_eq!(seen[0].3, seen[1].3);
        for (offset, function) in TABLE {
            assert_eq!(seen[0].3[offset as usize / 4], function);
        }
        // The flag byte at +0x30 is 1, the bytes after it are 0.
        assert_eq!(seen[0].3[12], 1);
    }

    #[test]
    fn the_contact_manager_forwarders_call_the_wrapped_manager_slots() {
        let mut e = Engine::new();
        let log = log_doubles(
            &mut e,
            &[
                (0x0900_0010, 11),
                (0x0900_0014, 12),
                (0x0900_0018, 13),
                (0x0900_0020, 14),
                (0x0900_0024, 15),
            ],
        );
        let inner = object_with(
            &mut e,
            &[
                (0x10, 0x0900_0010),
                (0x14, 0x0900_0014),
                (0x18, 0x0900_0018),
                (0x20, 0x0900_0020),
                (0x24, 0x0900_0024),
            ],
        );
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, inner);
        assert_eq!(e.call(0x00d3_3220, &args![this, 7u32]).u32(), 11);
        assert_eq!(e.call(0x00d3_3230, &args![this, 8u32, 9u32]).u32(), 12);
        assert_eq!(
            e.call(0x00d3_3240, &args![this, 1u32, 2u32, 3u32, 4u32])
                .u32(),
            13
        );
        assert_eq!(e.call(0x00d3_3250, &args![this, 5u32, 6u32]).u32(), 14);
        assert_eq!(e.call(0x00d3_3210, &args![this]).u32(), 15);
        assert_eq!(
            *log.borrow(),
            vec![
                (0x0900_0010, vec![inner, 7]),
                (0x0900_0014, vec![inner, 8, 9]),
                (0x0900_0018, vec![inner, 1, 2, 3, 4]),
                (0x0900_0020, vec![inner, 5, 6]),
                (0x0900_0024, vec![inner]),
            ]
        );
    }

    /// Words `1000 + n` for `n` in `0..count` at a new block.
    fn numbered(e: &mut Engine, count: u32) -> u32 {
        let block = e.mem.alloc(4 * count);
        for word in 0..count {
            e.mem.set_u32(block + 4 * word, 1000 + word);
        }
        block
    }

    #[test]
    fn the_first_record_copy_skips_the_words_at_0x14_to_0x1c() {
        let mut e = Engine::new();
        let source = numbered(&mut e, 0x18);
        let target = e.mem.alloc(0x60);
        let back = e.call(0x00d3_3260, &args![target, source]).u32();
        assert_eq!(back, target);
        for word in 0..0x18 {
            let expected = if (5..8).contains(&word) {
                0
            } else {
                1000 + word
            };
            assert_eq!(e.mem.u32(target + 4 * word), expected, "word {word}");
        }
    }

    #[test]
    fn the_second_record_copy_skips_the_words_at_0x28_and_0x2c() {
        let mut e = Engine::new();
        let source = numbered(&mut e, 0x1c);
        let target = e.mem.alloc(0x70);
        let back = e.call(0x00d3_3330, &args![target, source]).u32();
        assert_eq!(back, target);
        for word in 0..0x1c {
            let expected = if (10..12).contains(&word) {
                0
            } else {
                1000 + word
            };
            assert_eq!(e.mem.u32(target + 4 * word), expected, "word {word}");
        }
    }

    const CHILD_FETCH: u32 = 0x0900_0100;
    const INNER_ADD: u32 = 0x0900_0200;
    const INNER_TOI: u32 = 0x0900_0300;

    /// A body of a plain shape (type 5) with the third word `0x300 + tag`.
    fn plain_body(e: &mut Engine, tag: u32) -> u32 {
        let shape = e.mem.alloc(0x40);
        e.mem.set_u32(shape + 0xc, 5);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        e.mem.set_u32(body + 8, 0x300 + tag);
        body
    }

    /// A body of a list shape (type 0x16) whose container is at `shape + 0x14`; returns
    /// `(body, shape)`.
    fn list_body(e: &mut Engine, tag: u32) -> (u32, u32) {
        let shape = e.mem.alloc(0x40);
        e.mem.set_u32(shape + 0xc, 0x16);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x14, CHILD_FETCH);
        e.mem.set_u32(shape + 0x14, table);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        e.mem.set_u32(body + 8, 0x300 + tag);
        (body, shape)
    }

    /// The info block: `first` pairs of key `a`, `second` pairs of key `b`.
    fn info_block(e: &mut Engine, first: u8, a: u8, second: u8, b: u8) -> u32 {
        let info = e.mem.alloc(0x40);
        e.mem.set_u8(info + 8, first);
        e.mem.set_u8(info + 9, second);
        for pair in 0..u32::from(first) {
            e.mem.set_u8(info + 1 + 2 * pair, a);
        }
        for pair in 0..u32::from(second) {
            e.mem.set_u8(info + 1 + 2 * (u32::from(first) + pair), b);
        }
        info
    }

    /// What the wrapped manager saw: its arguments and the four words of both bodies.
    type Forwarded = Rc<RefCell<Vec<(Vec<u32>, Vec<u32>, Vec<u32>)>>>;

    /// A manager wrapping an object with a double at the slot `slot`, answering `0x4321`.
    fn manager(e: &mut Engine, slot: u32, target: u32) -> (u32, Forwarded) {
        let seen: Forwarded = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(target, move |e, a| {
            let first: Vec<u32> = (0..4).map(|word| e.mem.u32(a[1] + 4 * word)).collect();
            let second: Vec<u32> = (0..4).map(|word| e.mem.u32(a[2] + 4 * word)).collect();
            sink.borrow_mut().push((a.to_vec(), first, second));
            Ret {
                eax: 0x4321,
                ..Ret::default()
            }
        });
        let inner = object_with(e, &[(slot, target)]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, inner);
        (this, seen)
    }

    #[test]
    fn adding_a_contact_point_forwards_plain_bodies_unchanged() {
        let mut e = Engine::new();
        let (this, seen) = manager(&mut e, 0xc, INNER_ADD);
        let a = plain_body(&mut e, 1);
        let b = plain_body(&mut e, 2);
        let info = info_block(&mut e, 2, 4, 2, 9);
        let back = e
            .call(
                0x00d3_3390,
                &args![this, a, b, 0x10u32, 0x20u32, info, 0x30u32],
            )
            .u32();
        assert_eq!(back, 0x4321);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[1..], [a, b, 0x10, 0x20, info, 0x30]);
    }

    #[test]
    fn adding_a_contact_point_replaces_list_shape_bodies_by_their_child() {
        let mut e = Engine::new();
        let fetch = log_doubles(&mut e, &[(CHILD_FETCH, 0x5555)]);
        let (this, seen) = manager(&mut e, 0xc, INNER_ADD);
        let (a, shape_a) = list_body(&mut e, 1);
        let (b, shape_b) = list_body(&mut e, 2);
        // Two pairs of key 4 for the first body, two of key 9 for the second.
        let info = info_block(&mut e, 2, 4, 2, 9);
        let back = e
            .call(
                0x00d3_3390,
                &args![this, a, b, 0x10u32, 0x20u32, info, 0x30u32],
            )
            .u32();
        assert_eq!(back, 0x4321);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let (words, first, second) = &seen[0];
        assert_ne!(words[1], a);
        assert_ne!(words[2], b);
        assert_eq!(words[3..], [0x10, 0x20, info, 0x30]);
        assert_eq!(*first, vec![0x5555, 4, 0x301, a]);
        assert_eq!(*second, vec![0x5555, 9, 0x302, b]);
        let fetch = fetch.borrow();
        assert_eq!(fetch.len(), 2);
        assert_eq!(fetch[0].1[..2], [shape_a + 0x14, 4]);
        assert_eq!(fetch[1].1[..2], [shape_b + 0x14, 9]);
        // Both fetches share one scratch buffer.
        assert_eq!(fetch[0].1[2], fetch[1].1[2]);
    }

    #[test]
    fn adding_a_contact_point_with_unequal_keys_sets_the_flag_and_answers_0xffff() {
        for (second_group, fetches) in [(false, 0), (true, 1)] {
            let mut e = Engine::new();
            let fetch = log_doubles(&mut e, &[(CHILD_FETCH, 0x5555)]);
            let (this, seen) = manager(&mut e, 0xc, INNER_ADD);
            let (a, _) = list_body(&mut e, 1);
            let (b, _) = list_body(&mut e, 2);
            let info = info_block(&mut e, 2, 4, 2, 9);
            // Spoil the second pair of the first group, or the last pair of the second.
            let spoiled = if second_group { 1 + 2 * 3 } else { 1 + 2 };
            e.mem.set_u8(info + spoiled, 77);
            let back = e
                .call(0x00d3_3390, &args![this, a, b, 0u32, 0u32, info, 0u32])
                .u32();
            assert_eq!(back, 0xffff);
            assert_eq!(e.mem.u8(this + 0x10), 1);
            assert!(seen.borrow().is_empty());
            assert_eq!(fetch.borrow().len(), fetches);
        }
    }

    #[test]
    fn adding_a_toi_quiets_the_time_and_answers_1_on_unequal_keys() {
        let mut e = Engine::new();
        let fetch = log_doubles(&mut e, &[(CHILD_FETCH, 0x5555)]);
        let (this, seen) = manager(&mut e, 0x1c, INNER_TOI);
        let (a, _) = list_body(&mut e, 1);
        let b = plain_body(&mut e, 2);
        let info = info_block(&mut e, 2, 4, 1, 9);
        let back = e
            .call(
                0x00d3_34c0,
                &args![
                    this,
                    a,
                    b,
                    1u32,
                    2u32,
                    0x7f80_0001u32,
                    3u32,
                    info,
                    4u32,
                    5u32
                ],
            )
            .u32();
        assert_eq!(back, 0x4321);
        {
            let seen = seen.borrow();
            let (words, first, _) = &seen[0];
            assert_eq!(words[3..5], [1, 2]);
            assert_eq!(words[5], 0x7fc0_0001);
            assert_eq!(words[6..], [3, info, 4, 5]);
            assert_eq!(*first, vec![0x5555, 4, 0x301, a]);
        }
        assert_eq!(fetch.borrow().len(), 1);
        e.mem.set_u8(info + 3, 8);
        let back = e
            .call(
                0x00d3_34c0,
                &args![this, a, b, 1u32, 2u32, 0u32, 3u32, info, 4u32, 5u32],
            )
            .u32();
        assert_eq!(back, 1);
        assert_eq!(e.mem.u8(this + 0x10), 1);
        assert_eq!(seen.borrow().len(), 1);
    }

    /// Everything `process` is built around: the doubles log `(address, arguments)`.
    struct Rig {
        e: Engine,
        log: Calls,
        stream: u32,
        pair: u32,
        agent: u32,
        query: u32,
        context: u32,
        owner: u32,
        buffer: u32,
        arena: u32,
        router: u32,
        seen: Rc<RefCell<Vec<Vec<u32>>>>,
        index_list: Rc<RefCell<Vec<u32>>>,
    }

    /// The knobs of a run.
    struct Setup {
        agent_flags: u8,
        counter: u16,
        agent_e: u8,
        closest: u32,
        descriptor_byte: u8,
        first_root_float: f32,
        second_root_float: f32,
        query_time: f32,
    }

    fn setup() -> Setup {
        Setup {
            agent_flags: 0,
            counter: 0xffff,
            agent_e: 1,
            closest: 1,
            descriptor_byte: 1,
            first_root_float: 0.5,
            second_root_float: 1.0,
            query_time: -2.0,
        }
    }

    fn rig(setup: Setup) -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(NO_HIT_TIME, 0xfeff_ffee);
        let log: Calls = Rc::new(RefCell::new(Vec::new()));
        // The profile stream: room for four records of 0x10 bytes.
        let stream = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x40);
        e.set_global(PROFILE_SLOT, 9u32);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a, [9]);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        let arena = e.mem.alloc(0x100);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, arena);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let owner = e.mem.alloc(0x80);
        e.mem.set_u32(owner + 0x48, 3);
        let first_transform = e.mem.alloc(0x40);
        let second_transform = e.mem.alloc(0x40);
        let first_body = e.mem.alloc(0x60);
        e.mem.set_u32(first_body, 0xa0a0);
        e.mem.set_u32(first_body + 8, first_transform);
        e.mem.set_f32(first_body + 0x4c, setup.first_root_float);
        let second_body = e.mem.alloc(0x60);
        e.mem.set_u32(second_body, owner);
        e.mem.set_u32(second_body + 8, second_transform);
        e.mem.set_f32(second_body + 0x4c, setup.second_root_float);
        let third = e.mem.alloc(0x20);
        e.mem.set_u32(third, 0x3333);
        e.mem.set_f32(third + 8, 0.75);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x20, 0x0900_0020);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, table);
        let pair = e.mem.alloc(0x90);
        for (word, value) in (0..0x24u32).zip(2000u32..) {
            e.mem.set_u32(pair + 4 * word, value);
        }
        e.mem.set_u32(pair, first_body);
        e.mem.set_u32(pair + 4, second_body);
        e.mem.set_u32(pair + 0xc, third);
        e.mem.set_u32(pair + 0x10, object);
        let agent = e.mem.alloc(0x40);
        e.mem.set_u8(agent + 0xb, setup.agent_flags);
        e.mem.set_u16(agent + 0xc, setup.counter);
        e.mem.set_u8(agent + 0xe, setup.agent_e);
        let query = e.mem.alloc(0x20);
        e.mem.set_f32(query + 0xc, setup.query_time);
        let context = e.mem.alloc(0x3100);
        e.mem.set_u32(context + 4, 0x7777);
        let buffer = e.mem.alloc(0x1010);
        e.mem.set_u32(context + 0x3080, buffer);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let index_list = Rc::new(RefCell::new(Vec::new()));

        // Logging doubles without effects.
        for address in [
            SET_MUL_INVERSE_MUL,
            RESUME_LOCAL_OBJECT,
            DESTROY_MACHINE,
            PREDICATE_CLEANUP,
            RESET_MACHINE,
            0x0900_0020,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        // The local object's initialiser fills the fields the exe later reads back.
        let sink = Rc::clone(&log);
        e.register_double(INIT_LOCAL_OBJECT, move |e, a| {
            sink.borrow_mut().push((INIT_LOCAL_OBJECT, a.to_vec()));
            let object = a[0];
            e.mem.set_u32(object, 0x1357);
            e.mem.set_u32(object + 4, 0xaabb_ccdd);
            e.mem.set_u32(object + 0x1008, 0x1122_3344);
            for (offset, value) in [
                (0x1030, 5.0f32),
                (0x1034, 6.0),
                (0x1040, 7.0),
                (0x1044, 8.0),
                (0x1048, 9.0),
                (0x104c, 10.0),
            ] {
                e.mem.set_f32(object + offset, value);
            }
            for lane in 0..4u32 {
                e.mem
                    .set_f32(object + 0x1010 + 4 * lane, 11.0 + lane as f32);
                e.mem
                    .set_f32(object + 0x1020 + 4 * lane, 15.0 + lane as f32);
            }
            for word in 0..12u32 {
                e.mem.set_u32(object + 0x1050 + 4 * word, 100 + word);
            }
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(GET_CLOSEST_POINT, move |_, a| {
            sink.borrow_mut().push((GET_CLOSEST_POINT, a.to_vec()));
            Ret {
                eax: setup.closest,
                ..Ret::default()
            }
        });
        // The predicate agent: records what it was given and sets the descriptor byte.
        let (sink, snapshots) = (Rc::clone(&log), Rc::clone(&seen));
        e.register_double(PREDICATE_PROCESS, move |e, a| {
            sink.borrow_mut().push((PREDICATE_PROCESS, a.to_vec()));
            let record = a[0];
            let descriptor = record - 0x14;
            let mut words: Vec<u32> = (0..5).map(|w| e.mem.u32(descriptor + 4 * w)).collect();
            words.extend((0..32).map(|w| e.mem.u32(record + 4 * w)));
            snapshots.borrow_mut().push(words);
            e.mem.set_u8(descriptor + 0x10, setup.descriptor_byte);
            Ret {
                eax: 0x35,
                ..Ret::default()
            }
        });
        let (sink, list) = (Rc::clone(&log), Rc::clone(&index_list));
        e.register_double(MACHINE_PROCESS, move |e, a| {
            sink.borrow_mut().push((MACHINE_PROCESS, a.to_vec()));
            *list.borrow_mut() = (0..4).map(|w| e.mem.u32(a[3] + 4 * w)).collect();
            Ret::default()
        });
        Rig {
            e,
            log,
            stream,
            pair,
            agent,
            query,
            context,
            owner,
            buffer,
            arena,
            router,
            seen,
            index_list,
        }
    }

    impl Rig {
        fn run(&mut self) -> u32 {
            let (pair, agent, query, context) = (self.pair, self.agent, self.query, self.context);
            self.e
                .call(0x00d3_2cd0, &args![pair, 0x4444u32, agent, query, context])
                .u32()
        }

        fn calls(&self, address: u32) -> Vec<Vec<u32>> {
            self.log
                .borrow()
                .iter()
                .filter(|(called, _)| *called == address)
                .map(|(_, words)| words.clone())
                .collect()
        }

        /// The exe's stack pointer after the prologue, from the local object's address.
        fn frame(&self) -> u32 {
            self.calls(INIT_LOCAL_OBJECT)[0][0] - 0xd0
        }

        /// The addresses of the doubles called, in order.
        fn order(&self) -> Vec<u32> {
            self.log
                .borrow()
                .iter()
                .map(|(address, _)| *address)
                .collect()
        }
    }

    #[test]
    fn a_counter_that_is_not_negative_runs_the_machine_over_the_index_list() {
        let mut r = rig(Setup {
            counter: 3,
            ..setup()
        });
        let answer = r.run();
        // The counter was counted down, the no-hit time stored, the machine run over the
        // owner's three entries and the index list ended by -1.
        assert_eq!(r.e.mem.u16(r.agent + 0xc), 2);
        assert_eq!(r.e.mem.u32(r.query + 0xc), 0xfeff_ffee);
        let (agent, pair, owner, context) = (r.agent, r.pair, r.owner, r.context);
        assert_eq!(
            r.calls(MACHINE_PROCESS),
            vec![vec![agent + 0x10, pair, owner + 0x14, r.arena, context]]
        );
        assert_eq!(*r.index_list.borrow(), vec![0, 1, 2, 0xffff_ffff]);
        // The cursor is back, bit 6 is set, the answer is agent + 0x1c rounded up to 16.
        assert_eq!(r.e.mem.u32(r.router), r.arena);
        assert_eq!(r.e.mem.u8(r.agent + 0xb), 0x40);
        assert_eq!(answer, (agent + 0x1c + 0xf) & 0xffff_fff0);
        assert_eq!(r.order(), vec![INIT_LOCAL_OBJECT, MACHINE_PROCESS]);
        // Three profile records: the start (16 bytes), the pass and the end (12 bytes).
        let records = r.e.mem.u32(r.stream + 4);
        let first = records - 0x10 - 0xc - 0xc;
        assert_eq!(r.e.mem.u32(first), LABEL_LT_CVX_LST);
        assert_eq!(r.e.mem.u32(first + 0xc), LABEL_ST_TIM);
        assert_eq!(r.e.mem.u32(first + 0x10), LABEL_ST_STREAM);
        assert_eq!(r.e.mem.u32(first + 0x1c), LABEL_END);
    }

    #[test]
    fn a_negative_counter_without_a_closest_point_leaves_through_the_counter_exit() {
        let mut r = rig(Setup {
            closest: 0,
            ..setup()
        });
        r.run();
        let s = r.frame();
        assert_eq!(r.e.mem.u16(r.agent + 0xc), 0x19);
        let (first_body, second_body) = (r.e.mem.u32(r.pair), r.e.mem.u32(r.pair + 4));
        let transforms = vec![vec![
            s + 0x40,
            r.e.mem.u32(first_body + 8),
            r.e.mem.u32(second_body + 8),
        ]];
        assert_eq!(r.calls(SET_MUL_INVERSE_MUL), transforms);
        // The descriptor: the transform address, the bodies' words, the reading 0.75.
        let closest = r.calls(GET_CLOSEST_POINT);
        assert_eq!(closest, vec![vec![s + 0x2c, r.agent, r.query, s + 0xc0]]);
        assert_eq!(r.e.mem.u32(s + 0x2c), s + 0x40);
        assert_eq!(r.e.mem.u32(s + 0x30), r.e.mem.u32(first_body + 8));
        assert_eq!(r.e.mem.u32(s + 0x34), 0xa0a0);
        assert_eq!(r.e.mem.u32(s + 0x38), r.owner);
        assert_eq!(r.e.mem.f32(s + 0x3c), 0.75);
        // Nothing but the machine run follows.
        assert_eq!(
            r.order(),
            vec![
                INIT_LOCAL_OBJECT,
                SET_MUL_INVERSE_MUL,
                GET_CLOSEST_POINT,
                MACHINE_PROCESS
            ]
        );
        assert_eq!(r.e.mem.u32(r.query + 0xc), 0xfeff_ffee);
    }

    #[test]
    fn a_closest_point_destroys_the_machine_and_publishes_the_results() {
        let mut r = rig(setup());
        let answer = r.run();
        let s = r.frame();
        let (agent, pair, context) = (r.agent, r.pair, r.context);
        let third = r.e.mem.u32(pair + 0xc);
        let object = r.e.mem.u32(pair + 0x10);
        let (first_body, second_body) = (r.e.mem.u32(pair), r.e.mem.u32(pair + 4));
        assert_eq!(
            r.calls(DESTROY_MACHINE),
            vec![vec![agent + 0x10, 0x3333, object, 0x7777]]
        );
        assert_eq!(
            r.calls(RESUME_LOCAL_OBJECT),
            vec![vec![s + 0xd0, first_body, second_body, context, object]]
        );
        // The predicate agent got the record copy, the descriptor and the pair's words.
        let predicate = r.calls(PREDICATE_PROCESS);
        assert_eq!(
            predicate,
            vec![vec![s + 0x40, 0x4444, agent, r.query, context]]
        );
        let words = r.seen.borrow()[0].clone();
        assert_eq!(words[0], BODY_DESCRIPTOR_TABLE);
        assert_eq!(
            words[1],
            0x0001_0000 | (r.e.mem.u32(first_body + 8) & 0xffff)
        );
        assert_eq!(words[2], 2);
        assert_eq!(words[3], object);
        // The record: five words from the pair (the fifth replaced by the descriptor
        // address), a gap, sixteen words from +0x20.
        let record = &words[5..];
        assert_eq!(record[..4], [first_body, second_body, 2002, third]);
        assert_eq!(record[4], s + 0x2c);
        assert_eq!(record[8..24], (2008..2024).collect::<Vec<u32>>()[..]);
        assert_eq!(record[24], r.e.mem.u32(pair + 0x60));
        assert_eq!(record[28], r.e.mem.u32(pair + 0x70));
        // Results published: the vtable call, the floats, the vectors, the block, the
        // word and the buffer.
        assert_eq!(
            r.calls(0x0900_0020),
            vec![vec![object, 0x7777, context + 0x3050]]
        );
        assert_eq!(r.e.mem.f32(context + 0x3030), 5.0);
        assert_eq!(r.e.mem.f32(context + 0x3034), 6.0);
        assert_eq!(r.e.mem.f32(context + 0x3040), 7.0);
        assert_eq!(r.e.mem.f32(context + 0x3044), 8.0);
        assert_eq!(r.e.mem.f32(context + 0x3048), 9.0);
        assert_eq!(r.e.mem.f32(context + 0x304c), 10.0);
        assert_eq!(r.e.mem.f32(context + 0x3010), 11.0);
        assert_eq!(r.e.mem.f32(context + 0x301c), 14.0);
        assert_eq!(r.e.mem.f32(context + 0x3020), 15.0);
        assert_eq!(r.e.mem.f32(context + 0x302c), 18.0);
        assert_eq!(r.e.mem.u32(context + 0x3050), 100);
        assert_eq!(r.e.mem.u32(context + 0x3050 + 44), 111);
        assert_eq!(r.e.mem.u32(context), 0x1357);
        assert_eq!(r.e.mem.u32(r.buffer), 0xaabb_ccdd);
        assert_eq!(r.e.mem.u32(r.buffer + 0x1004), 0x1122_3344);
        // Cleanup, the machine reset, the counter at 0x19 and then the counter exit.
        assert_eq!(
            r.calls(PREDICATE_CLEANUP),
            vec![vec![0x4444, agent, object, 0x7777]]
        );
        assert_eq!(r.calls(RESET_MACHINE), vec![vec![agent + 0x10]]);
        assert_eq!(r.e.mem.u32(agent + 0x10), 0);
        assert_eq!(r.e.mem.u32(agent + 0x14), 0);
        assert_eq!(r.e.mem.u32(agent + 0x18), 0x8000_0000);
        assert_eq!(r.e.mem.u16(agent + 0xc), 0x18);
        assert_eq!(r.calls(MACHINE_PROCESS).len(), 1);
        assert_eq!(r.e.mem.u8(agent + 0xb), 0x40);
        assert_eq!(answer, (agent + 0x1c + 0xf) & 0xffff_fff0);
    }

    #[test]
    fn a_flagged_entry_skips_the_counter_and_ends_without_publishing_when_idle() {
        let mut r = rig(Setup {
            agent_flags: 0x20,
            agent_e: 0,
            descriptor_byte: 0,
            ..setup()
        });
        let answer = r.run();
        // No closest point query, no results, bit 6 set, bit 5 still set; the answer is the
        // predicate agent's 0x35 rounded up to 16.
        assert_eq!(r.order(), vec![INIT_LOCAL_OBJECT, PREDICATE_PROCESS]);
        assert_eq!(r.e.mem.u8(r.agent + 0xb), 0x60);
        assert_eq!(answer, 0x40);
        assert_eq!(r.e.mem.u32(r.context), 0);
        assert_eq!(r.e.mem.u32(r.query + 0xc), (-2.0f32).to_bits());
    }

    #[test]
    fn the_smaller_root_float_decides_whether_an_idle_pass_publishes() {
        // The roots are 0.5 (first body) and 1.0 (second): the smaller, doubled and negated,
        // is -1.0. A query float below it goes on, a float at or above it ends the call.
        for (query_time, goes_on) in [(-2.0f32, true), (-1.0, false), (0.5, false)] {
            let mut r = rig(Setup {
                agent_flags: 0x20,
                descriptor_byte: 0,
                query_time,
                ..setup()
            });
            r.run();
            assert_eq!(
                r.calls(PREDICATE_CLEANUP).len(),
                usize::from(goes_on),
                "{query_time}"
            );
        }
        // With bit 6 already set the pass ends at once, as it does for a NaN query float.
        let mut r = rig(Setup {
            agent_flags: 0x60,
            descriptor_byte: 0,
            ..setup()
        });
        r.run();
        assert!(r.calls(PREDICATE_CLEANUP).is_empty());
        let mut r = rig(Setup {
            agent_flags: 0x20,
            descriptor_byte: 0,
            query_time: f32::NAN,
            ..setup()
        });
        r.run();
        assert!(r.calls(PREDICATE_CLEANUP).is_empty());
    }

    #[test]
    fn an_unchanged_contact_float_publishes_nothing_but_the_word_and_buffer() {
        let mut r = rig(Setup {
            agent_flags: 0x20,
            ..setup()
        });
        r.e.mem.set_f32(r.context + 0x3030, 5.0);
        r.run();
        assert!(r.calls(0x0900_0020).is_empty());
        assert_eq!(r.e.mem.f32(r.context + 0x3034), 0.0);
        assert_eq!(r.e.mem.u32(r.context), 0x1357);
        assert_eq!(r.e.mem.u32(r.buffer), 0xaabb_ccdd);
        // A cleared buffer pointer skips the copy.
        let mut r = rig(Setup {
            agent_flags: 0x20,
            ..setup()
        });
        r.e.mem.set_u32(r.context + 0x3080, 0);
        r.run();
        assert_eq!(r.e.mem.u32(r.buffer), 0);
    }
}

#[cfg(test)]
mod tests_a07_gap {
    use super::*;

    /// The argument words of every call to `address` in the engine's call log (which is taken).
    fn calls_to(e: &mut Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words)
            .collect()
    }

    /// A block standing for an agent entry whose byte at `+0xb` is `flags`.
    fn entry_with(e: &mut Engine, flags: u8) -> u32 {
        let entry = e.mem.alloc(0x40);
        e.mem.set_u8(entry + 0xb, flags);
        entry
    }

    /// A double over `address` that answers `eax`.
    fn answering(e: &mut Engine, address: u32) {
        e.register(address, |_, _| Ret {
            eax: 0x99,
            ..Ret::default()
        });
    }

    #[test]
    fn fn_00d32b20_hands_the_track_to_the_invalidate_unless_the_entry_is_cleaned_up() {
        let mut e = Engine::new();
        answering(&mut e, INVALIDATE_TIM);
        let cleaned = entry_with(&mut e, 0x20);
        let live = entry_with(&mut e, 0);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d3_2b20, &args![1u32, cleaned, 5u32]).u32(),
            cleaned
        );
        assert_eq!(e.call(0x00d3_2b20, &args![1u32, live, 5u32]).u32(), 0x99);
        assert_eq!(calls_to(&mut e, INVALIDATE_TIM), vec![vec![live + 0x10, 5]]);
    }

    #[test]
    fn fn_00d32b40_passes_the_floats_through_the_x87_stack() {
        let mut e = Engine::new();
        answering(&mut e, WARP_TIME);
        let cleaned = entry_with(&mut e, 0x20);
        let live = entry_with(&mut e, 0);
        e.call_log = Some(vec![]);
        let args_for = |entry: u32| args![1u32, entry, 0x7f80_0001u32, 0x3fc0_0000u32, 9u32];
        assert_eq!(e.call(0x00d3_2b40, &args_for(cleaned)).u32(), cleaned);
        assert_eq!(e.call(0x00d3_2b40, &args_for(live)).u32(), 0x99);
        // The signalling NaN comes out quiet; the other float is unchanged.
        assert_eq!(
            calls_to(&mut e, WARP_TIME),
            vec![vec![live + 0x10, 0x7fc0_0001, 0x3fc0_0000, 9]]
        );
    }

    #[test]
    fn fn_00d32b70_creates_then_marks_the_entry() {
        let mut e = Engine::new();
        // The callee changes the entry's byte after it was handed over; the marking reads it
        // after the call.
        e.register_double(PREDICATE_CREATE, |e, a| {
            e.mem.set_u8(a[2] + 0xb, 0xd0);
            Ret {
                eax: 0x99,
                ..Ret::default()
            }
        });
        let entry = entry_with(&mut e, 0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d3_2b70, &args![3u32, 4u32, entry]).u32(), 0x99);
        assert_eq!(calls_to(&mut e, PREDICATE_CREATE), vec![vec![3, 4, entry]]);
        // (0xd0 & 0xbf) | 0x20
        assert_eq!(e.mem.u8(entry + 0xb), 0xb0);
    }

    #[test]
    fn fn_00d32ba0_destroys_the_machine_or_jumps_to_the_predicate_function() {
        let mut e = Engine::new();
        answering(&mut e, PREDICATE_DESTROY);
        answering(&mut e, DESTROY_MACHINE);
        let cleaned = entry_with(&mut e, 0x20);
        let live = entry_with(&mut e, 0);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d3_2ba0, &args![1u32, cleaned, 3u32, 4u32, 5u32])
                .u32(),
            0x99
        );
        assert_eq!(
            e.call(0x00d3_2ba0, &args![1u32, live, 3u32, 4u32, 5u32])
                .u32(),
            0x99
        );
        let log = e.call_log.take().unwrap();
        let only = |address: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|(called, _)| *called == address)
                .map(|(_, words)| words.clone())
                .collect()
        };
        assert_eq!(only(PREDICATE_DESTROY), vec![vec![1, cleaned, 3, 4]]);
        assert_eq!(only(DESTROY_MACHINE), vec![vec![live + 0x10, 5, 3, 4]]);
    }

    #[test]
    fn fn_00d32bd0_jumps_to_the_predicate_cleanup() {
        let mut e = Engine::new();
        answering(&mut e, PREDICATE_CLEANUP);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d3_2bd0, &args![1u32, 2u32, 3u32, 4u32]).u32(),
            0x99
        );
        assert_eq!(calls_to(&mut e, PREDICATE_CLEANUP), vec![vec![1, 2, 3, 4]]);
    }

    #[test]
    fn the_slot_0c_10_and_14_functions_jump_only_for_a_cleaned_up_entry() {
        let mut e = Engine::new();
        let cleaned = entry_with(&mut e, 0x20);
        let live = entry_with(&mut e, 0);
        for (function, target) in [
            (0x00d3_2be0u32, PREDICATE_SLOT_0C),
            (0x00d3_2c00, PREDICATE_SLOT_10),
            (0x00d3_2c20, PREDICATE_SLOT_14),
        ] {
            answering(&mut e, target);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(function, &args![1u32, cleaned, 3u32]).u32(), 0x99);
            assert_eq!(e.call(function, &args![1u32, live, 3u32]).u32(), live);
            assert_eq!(calls_to(&mut e, target), vec![vec![1, cleaned, 3]]);
        }
    }

    #[test]
    fn fn_00d32ca0_fills_the_answer_buffer_or_jumps_to_the_separating_normal() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        for lane in 0..4u32 {
            e.mem
                .set_u32(NO_HIT_ANSWER + 4 * lane, 0x1111_1111 * (lane + 1));
        }
        e.mem.set_u32(NO_HIT_TIME, 0x7f80_0001);
        answering(&mut e, PREDICATE_SEP_NORMAL);
        let cleaned = entry_with(&mut e, 0x20);
        let live = entry_with(&mut e, 0);
        let answer = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d3_2ca0, &args![1u32, 2u32, cleaned, answer])
                .u32(),
            0x99
        );
        assert_eq!(
            calls_to(&mut e, PREDICATE_SEP_NORMAL),
            vec![vec![1, 2, cleaned, answer]]
        );
        assert_eq!(e.mem.u32(answer), 0);
        assert_eq!(
            e.call(0x00d3_2ca0, &args![1u32, 2u32, live, answer]).u32(),
            answer
        );
        assert_eq!(e.mem.u32(answer), 0x1111_1111);
        assert_eq!(e.mem.u32(answer + 4), 0x2222_2222);
        assert_eq!(e.mem.u32(answer + 8), 0x3333_3333);
        // The fourth word is overwritten by the float copied through the x87 stack (quiet NaN).
        assert_eq!(e.mem.u32(answer + 0xc), 0x7fc0_0001);
    }
}
