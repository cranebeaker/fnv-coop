//! `Havok SDK/hkpcpuagentsectorjob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::profile_record;

// Translated from 00d93310 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `ECX` = object, nothing on the stack. Sets the words at `+0`,
/// `+4` and `+8` to 0x10, 0 and 0x100. Returns `this`.
pub fn fn_00d93310(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, 0x10);
    e.mem.set_u32(me + 4, 0);
    e.mem.set_u32(me + 8, 0x100);
    this
}

// Translated from 00d93330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDeferredConstraintOwner::addConstraintToCriticalLockedIsland` (Xbox PDB): `ECX` =
/// object, one stack word. Adds 1 to the word at `+0x18` and stores the stack word at `+0x1C`.
pub fn hkp_deferred_constraint_owner_add_constraint_to_critical_locked_island(
    e: &mut Engine,
    this: Ptr,
    constraint: u32,
) {
    let me = this.addr();
    let count = e.mem.u32(me + 0x18).wrapping_add(1);
    e.mem.set_u32(me + 0x18, count);
    e.mem.set_u32(me + 0x1c, constraint);
}

// Translated from 00d93340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDeferredConstraintOwner::removeConstraintFromCriticalLockedIsland` (Xbox PDB): `ECX` =
/// object, one stack word. Subtracts 1 from the word at `+0x18` and stores the stack word at
/// `+0x1C`.
pub fn hkp_deferred_constraint_owner_remove_constraint_from_critical_locked_island(
    e: &mut Engine,
    this: Ptr,
    constraint: u32,
) {
    let me = this.addr();
    let count = e.mem.u32(me + 0x18).wrapping_sub(1);
    e.mem.set_u32(me + 0x18, count);
    e.mem.set_u32(me + 0x1c, constraint);
}

// Translated from 00d93350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDeferredConstraintOwner::addCallbackRequest` (Xbox PDB): `ECX` = object, two stack
/// words (a constraint, a request mask). When the constraint's word at `+0x30` is not 0 it is
/// an object whose byte at `+0x12` gets the low byte of the mask or-ed in; otherwise the mask
/// is or-ed into the word at `+0x20` of the owner.
pub fn hkp_deferred_constraint_owner_add_callback_request(
    e: &mut Engine,
    this: Ptr,
    constraint: Ptr,
    request: u32,
) {
    let target = e.mem.u32(constraint.addr() + 0x30);
    if target != 0 {
        let byte = e.mem.u8(target + 0x12) | request as u8;
        e.mem.set_u8(target + 0x12, byte);
    } else {
        let mask = e.mem.u32(this.addr() + 0x20) | request;
        e.mem.set_u32(this.addr() + 0x20, mask);
    }
}

/// `hkpProcessCollisionInput` copy (`00cfa7d0`, `ECX` = destination, one stack word: the source).
const COPY_COLLISION_INPUT: u32 = 0x00cf_a7d0;
/// `hkAgentNnMachine_ProcessAgent` (Xbox PDB name per the engine map, `00cfedd0`, cdecl `(agent
/// entry, input, output cell, agent)`).
const PROCESS_AGENT: u32 = 0x00cf_edd0;
/// `hkpMultiThreadedSimulation::addToiEventWithCriticalSectionLock` (`00d02e70`, `this` = the
/// world, three stack words: the output cell, the agent entry, the address `world + 0x100`).
const ADD_TOI_EVENT: u32 = 0x00d0_2e70;
/// `hkpWorld::lockIslandForConstraintUpdate` and `hkpWorld::unlockIslandForConstraintUpdate`
/// (`00c911d0`, `00c91210`: `ECX` = the object, one stack word).
pub(crate) const LOCK_ISLAND: u32 = 0x00c9_11d0;
pub(crate) const UNLOCK_ISLAND: u32 = 0x00c9_1210;
/// Runs a buffer of deferred constraint commands (`00df25f0`, cdecl `(object, commands, byte
/// count)`; a command is 16 bytes whose first byte is 0 for an add and 1 for a remove).
pub(crate) const RUN_COMMANDS: u32 = 0x00df_25f0;
/// The call the job ends with (`00d04e80`, `ECX` = the second argument of the job function, three
/// stack words: the job twice and 0), in the file `hkjobqueue.cpp` (no name in the engine map).
pub(crate) const JOB_DONE: u32 = 0x00d0_4e80;
/// The label that closes the work of a CPU job (`01076f38`) and the one that opens the integrate
/// jobs' start records (`"LtIntegrate"`, `010cc5c4`).
pub(crate) const LABEL_JOB_END: u32 = 0x0107_6f38;
pub(crate) const LABEL_LT_INTEGRATE: u32 = 0x010c_c5c4;
/// The memory router (`00c85750`); its `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The vtable of `hkpDeferredConstraintOwner` (its slots `3`, `4` and `5` are `00d93330`,
/// `00d93340` and `00d93350`) and the vtable the destructor leaves in the object.
const DEFERRED_OWNER_VTABLE: u32 = 0x010d_3df8;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The flag that stops the jobs early when it is 1 (`01269200`).
const STOP_FLAG: u32 = 0x0126_9200;
/// `3.40282e+38` as a `float` and as a `double` (`0104da48`, `0104e138`).
const FLOAT_MAX: u32 = 0x0104_da48;
const DOUBLE_MAX: u32 = 0x0104_e138;
/// Profiling labels `"TtNarrowPhase"`, `"EtNarrowPhase"`, `"TtNarrowPhaseTOI"` and
/// `"EtNarrowPhaseTOI"`.
const LABEL_TT: u32 = 0x010c_c54c;
const LABEL_ET: u32 = 0x010d_3e28;
const LABEL_TT_TOI: u32 = 0x010c_d440;
const LABEL_ET_TOI: u32 = 0x010d_3e14;

/// The 0x3284-byte frame `hkCpuAgentSectorJob` takes through `__alloca_probe` (`00ec8660`)
/// holds, around the processed collision input `L` (a copy of `owner + 0x90`):
/// the deferred constraint owner (`hkpDeferredConstraintOwner`) at `L - 0x140`, with its four
/// counters at `+8`, `+0xC`, `+0x10`, `+0x14`, its lock count at `+0x18`, the last constraint
/// at `+0x1C` and the request mask at `+0x20`; the command buffer at `L - 0x110` (offset of the
/// commands `+0`, size in bytes `+4`, capacity `+8`, the commands after the offset); the
/// words `L - 0x154` and `L - 0x14C` that the first word of a command is built in; the fields
/// `L + 0x14` (a word of the pair information), `L + 0x50`..`L + 0x5C` (four words of the job),
/// `L + 0x60` (the pair information), `L + 0x64` (a block to free, 0), `L + 0x80` (the output
/// cursor) with `L + 0x84` (the owner), the output buffer at `L + 0x90`, `L + 0x30B0` (a float,
/// `3.40282e+38` when no time of impact was found) and `L + 0x3100`.
const OWNER_AT: i32 = -0x140;
const BUFFER_AT: i32 = -0x110;
const COMMAND_TAG_ADD_AT: i32 = -0x14c;
const COMMAND_TAG_REMOVE_AT: i32 = -0x154;
const INPUT_VALUE: u32 = 0x14;
const INPUT_JOB_WORDS: u32 = 0x50;
const INPUT_PAIR_INFO: u32 = 0x60;
const INPUT_SCRATCH: u32 = 0x64;
const INPUT_OUTPUT_CURSOR: u32 = 0x80;
const INPUT_OWNER: u32 = 0x84;
const INPUT_OUTPUT_BUFFER: u32 = 0x90;
const INPUT_MIN_TOI: u32 = 0x30b0;
const INPUT_ZEROED_WORDS: u32 = 0x30d0;
const INPUT_CLEARED_WORD: u32 = 0x3100;
/// The size of the frame block and where `L` sits in it.
const FRAME_SIZE: u32 = 0x3300;
const FRAME_INPUT: u32 = 0x180;

/// The address `offset` bytes from `base`.
fn at(base: u32, offset: i32) -> u32 {
    base.wrapping_add(offset as u32)
}

// Translated from 00d93380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuAgentSectorJob` (Xbox PDB name per the engine map): cdecl, three stack words: the
/// owner, the word that becomes `ECX` of the final call, and the job. The frame is a 0x3284-byte
/// block taken through `__alloca_probe` (`00ec8660`). It copies the owner's collision input
/// (`+0x90`) and puts the job's four words (`+0x30`..`+0x3C`) in it, builds a
/// `hkpDeferredConstraintOwner` and a command buffer, then:
///
/// - if the job's type byte is 0xD, walks the agent entries of the world's sector chunks
///   (`job + 0x14`: chunk table `+0x5C`, chunk count `+0x60`, last chunk size `+0x58`; a chunk
///   is 0x200 bytes of 0x80-byte entries, starting with the chunk number `job + 0x40`) and
///   processes each; then either hands the counters and commands to the job's result block
///   (`job + 0x20`, entry `job + 0x24`) or, with no result block, adds them to the world's
///   counters under `lockIslandForConstraintUpdate` and runs the commands;
/// - otherwise walks `job + 0x28` entries of the list at `job + 0x2C` from the position
///   `u16(job + 0x2A) * u16(job + 0x24)`, using the second pair information array when
///   `job + 0x40` is not 0, and always hands the counters and commands to the result block.
///
/// An entry is processed by `hkAgentNnMachine_ProcessAgent`, the agent's virtual function at
/// `+0x18` when the output cursor moved, and `addToiEventWithCriticalSectionLock` when a time
/// of impact was found; a deferred constraint command pending in the owner becomes a
/// 16-byte command in the buffer. The stop flag `01269200` ends the walk early. The first word
/// of a command keeps its upper three bytes from the frame (padding, never read). Profile
/// records `TtNarrowPhase`/`EtNarrowPhase` (or the `TOI` pair) bracket the work. Returns what
/// `00d04e80` returns.
pub fn hk_cpu_agent_sector_job(e: &mut Engine, owner: Ptr, second: u32, job: Ptr) -> u32 {
    e.with_stack(FRAME_SIZE, |e, frame| {
        let input = frame.addr() + FRAME_INPUT;
        let owner = owner.addr();
        let job = job.addr();
        for word in 0..0x14 {
            let value = e.mem.u32(owner + 0x90 + 4 * word);
            e.mem.set_u32(input + 4 * word, value);
        }
        e.mem.set_u32(input + INPUT_SCRATCH, 0);
        e.call(COPY_COLLISION_INPUT, &args![input, owner + 0x90]);
        for word in 0..4 {
            let value = e.mem.u32(job + 0x30 + 4 * word);
            e.mem.set_u32(input + INPUT_JOB_WORDS + 4 * word, value);
        }
        let deferred = at(input, OWNER_AT);
        e.mem.set_u16(deferred + 6, 1);
        e.mem.set_u32(deferred, DEFERRED_OWNER_VTABLE);
        fn_00d93310(e, Ptr::new(at(input, BUFFER_AT)));
        e.mem
            .set_u32(input + INPUT_MIN_TOI, x87_quiet_bits(e.mem.u32(FLOAT_MAX)));
        e.mem.set_u32(input + INPUT_ZEROED_WORDS, 0);
        e.mem.set_u32(input + INPUT_ZEROED_WORDS + 4, 0);
        for offset in [8, 0xc, 0x10, 0x14, 0x18, 0x20] {
            e.mem.set_u32(deferred + offset, 0);
        }
        e.mem.set_u32(input + INPUT_OWNER, deferred);
        if e.mem.u8(job) == 0xd {
            narrow_phase_sectors(e, owner, job, input);
        } else {
            narrow_phase_list(e, owner, job, input);
        }
        let result = e.call(JOB_DONE, &args![second, job, job, 0u32]).u32();
        e.mem.set_u32(deferred, BASE_VTABLE);
        let scratch = e.mem.u32(input + INPUT_SCRATCH);
        if scratch != 0 {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.vcall(allocator, 8, &args![scratch, 0x200u32]);
        }
        result
    })
}

/// The rule for the end of a chunk: the last chunk (number `world + 0x60`) holds `world + 0x58`
/// bytes and the others 0x200.
fn chunk_bytes(e: &Engine, world: u32, chunk: u32) -> u32 {
    if chunk == e.mem.u32(world + 0x60) {
        e.mem.u32(world + 0x58)
    } else {
        0x200
    }
}

/// The type-0xD branch of `hkCpuAgentSectorJob`: the entries of the chunks starting at
/// `job + 0x40`.
fn narrow_phase_sectors(e: &mut Engine, owner: u32, job: u32, input: u32) {
    let world = e.mem.u32(job + 0x14);
    profile_record(e, LABEL_TT);
    let mut remaining = i32::from(e.mem.u16(job + 0x28)) - 1;
    let first = e.mem.u32(job + 0x40);
    let mut chunk_offset = first.wrapping_mul(4);
    let mut entry = e
        .mem
        .u32(chunk_offset.wrapping_add(e.mem.u32(world + 0x5c)));
    let mut chunk = first.wrapping_add(1);
    let mut end = entry.wrapping_add(chunk_bytes(e, world, chunk));
    // The next entry and the one after it; 0 when they are past the end of the chunk.
    let mut next = entry.wrapping_add(0x80);
    let mut after = next.wrapping_add(0x80);
    if next >= end {
        next = 0;
        after = 0;
    } else if after >= end {
        after = 0;
    }
    loop {
        let class = i32::from(e.mem.u8(entry + 0xc) as i8) << 6;
        let info = e
            .mem
            .u32(input)
            .wrapping_add(class as u32)
            .wrapping_add(0x1c20);
        e.mem.set_u32(input + INPUT_PAIR_INFO, info);
        let value = e.mem.u32(info + 0x10);
        e.mem.set_u32(input + INPUT_VALUE, value);
        process_entry(e, owner, input, entry, true);
        let following = after;
        if e.mem.u32(STOP_FLAG) == 1 {
            break;
        }
        if after != 0 {
            after = after.wrapping_add(0x80);
            if after >= end {
                chunk_offset = chunk_offset.wrapping_add(4);
                chunk = chunk.wrapping_add(1);
                remaining -= 1;
                if remaining < 0 {
                    after = 0;
                } else {
                    after = e
                        .mem
                        .u32(e.mem.u32(world + 0x5c).wrapping_add(chunk_offset));
                    end = after.wrapping_add(chunk_bytes(e, world, chunk));
                }
            }
        }
        let current = next;
        entry = next;
        next = following;
        if current == 0 {
            break;
        }
    }
    if e.mem.u32(job + 0x20) == 0 {
        let deferred = at(input, OWNER_AT);
        let buffer = at(input, BUFFER_AT);
        let counters = [
            e.mem.u32(deferred + 8),
            e.mem.u32(deferred + 0xc),
            e.mem.u32(deferred + 0x10),
            e.mem.u32(deferred + 0x14),
        ];
        let size = e.mem.u32(buffer + 4);
        if counters[3] | counters[2] | counters[1] | counters[0] != 0 || size != 0 {
            let lock_object = e.mem.u32(owner);
            e.call(LOCK_ISLAND, &args![lock_object, world]);
            let widest = (e.mem.i32(world + 8)).max(counters[0] as i32);
            e.mem.set_u32(world + 8, widest as u32);
            for (offset, counter) in [(0xc, counters[1]), (0x10, counters[2]), (0x14, counters[3])]
            {
                let total = e.mem.u32(world + offset).wrapping_add(counter);
                e.mem.set_u32(world + offset, total);
            }
            if size != 0 {
                let commands = buffer.wrapping_add(e.mem.u32(buffer));
                e.call(RUN_COMMANDS, &args![lock_object, commands, size]);
            }
            e.call(UNLOCK_ISLAND, &args![lock_object, world]);
        }
    } else {
        store_results(e, job, input);
    }
    profile_record(e, LABEL_ET);
}

/// The other branch of `hkCpuAgentSectorJob`: `job + 0x28` entries from the list at `job + 0x2C`.
fn narrow_phase_list(e: &mut Engine, owner: u32, job: u32, input: u32) {
    profile_record(e, LABEL_TT_TOI);
    let count = i32::from(e.mem.u16(job + 0x28));
    if count != 0 {
        let mut offset = u32::from(e.mem.u16(job + 0x2a))
            .wrapping_mul(u32::from(e.mem.u16(job + 0x24)))
            .wrapping_mul(4);
        let mut done = 0;
        loop {
            let list = e.mem.u32(job + 0x2c);
            let entry = e.mem.u32(offset.wrapping_add(list));
            let class = i32::from(e.mem.u8(entry + 0xc) as i8) << 6;
            let dispatcher = e.mem.u32(input);
            if e.mem.u8(job + 0x40) != 0 {
                e.mem
                    .set_u32(input + INPUT_PAIR_INFO, dispatcher.wrapping_add(0x1de0));
                let value = e
                    .mem
                    .u32(dispatcher.wrapping_add(class as u32).wrapping_add(0x1c30));
                e.mem.set_u32(input + INPUT_VALUE, value);
            } else {
                let info = dispatcher.wrapping_add(class as u32).wrapping_add(0x1c20);
                e.mem.set_u32(input + INPUT_PAIR_INFO, info);
                let value = e.mem.u32(info + 0x10);
                e.mem.set_u32(input + INPUT_VALUE, value);
            }
            process_entry(e, owner, input, entry, false);
            if e.mem.u32(STOP_FLAG) == 1 {
                break;
            }
            offset = offset.wrapping_add(4);
            done += 1;
            if done >= i32::from(e.mem.u16(job + 0x28)) {
                break;
            }
        }
    }
    store_results(e, job, input);
    profile_record(e, LABEL_ET_TOI);
}

/// Processes one agent entry: runs `hkAgentNnMachine_ProcessAgent` into the output buffer, lets
/// the agent's virtual function at `+0x18` finish when the output cursor moved, reports a time
/// of impact, turns a pending deferred constraint command of the owner into a command in the
/// buffer and clears the owner's lock count. With `early` set the entry's two words at `+0x10`,
/// `+0x14` and the world pointer at `owner + 0x110` are read before the agent runs (the sector
/// walk), otherwise after (the list walk).
fn process_entry(e: &mut Engine, owner: u32, input: u32, entry: u32, early: bool) {
    let read = |e: &Engine| {
        (
            e.mem.u32(entry + 0x10),
            e.mem.u32(entry + 0x14),
            e.mem.u32(owner + 0x110),
        )
    };
    let early_words = early.then(|| read(e));
    e.mem
        .set_u32(input + INPUT_MIN_TOI, x87_quiet_bits(e.mem.u32(FLOAT_MAX)));
    e.mem
        .set_u32(input + INPUT_OUTPUT_CURSOR, input + INPUT_OUTPUT_BUFFER);
    e.mem.set_u32(input + INPUT_CLEARED_WORD, 0);
    let agent = e.mem.u32(entry + 8);
    e.call(
        PROCESS_AGENT,
        &args![entry, input, input + INPUT_OUTPUT_CURSOR, agent],
    );
    let (first, second, world) = early_words.unwrap_or_else(|| read(e));
    if e.mem.u32(input + INPUT_OUTPUT_CURSOR) != input + INPUT_OUTPUT_BUFFER {
        let agent = e.mem.u32(entry + 8);
        e.vcall(
            agent,
            0x18,
            &args![first, second, input, input + INPUT_OUTPUT_CURSOR],
        );
    }
    let min_toi = f64::from(x87_quiet(e.mem.f32(input + INPUT_MIN_TOI)));
    if min_toi != e.global::<f64>(DOUBLE_MAX) {
        // 00d935be: `00d02e70` (RET 0xc) reads ECX, the world (set at 00d935aa).
        e.call(
            ADD_TOI_EVENT,
            &args![world, input + INPUT_OUTPUT_CURSOR, entry, world + 0x100],
        );
    }
    let deferred = at(input, OWNER_AT);
    let buffer = at(input, BUFFER_AT);
    let lock_count = e.mem.i32(deferred + 0x18);
    if lock_count == 0 {
        e.mem.set_u32(deferred + 0x20, 0);
    } else {
        let slot = buffer
            .wrapping_add(e.mem.u32(buffer))
            .wrapping_add(e.mem.u32(buffer + 4));
        if lock_count > 0 {
            e.mem.set_u8(at(input, COMMAND_TAG_ADD_AT), 0);
            let tag = e.mem.u32(at(input, COMMAND_TAG_ADD_AT));
            e.mem.set_u32(slot, tag);
            let constraint = e.mem.u32(deferred + 0x1c);
            e.mem.set_u32(slot + 4, constraint);
            let request = e.mem.u32(deferred + 0x20);
            e.mem.set_u32(slot + 8, request);
            e.mem.set_u32(deferred + 0x20, 0);
        } else {
            e.mem.set_u8(at(input, COMMAND_TAG_REMOVE_AT), 1);
            let tag = e.mem.u32(at(input, COMMAND_TAG_REMOVE_AT));
            e.mem.set_u32(slot, tag);
            let constraint = e.mem.u32(deferred + 0x1c);
            e.mem.set_u32(slot + 4, constraint);
        }
        e.mem
            .set_u32(buffer + 4, e.mem.u32(buffer + 4).wrapping_add(0x10));
    }
    e.mem.set_u32(deferred + 0x18, 0);
}

/// Hands the owner's four counters and the command buffer to the job's result block, the
/// pointer at `job + 0x20 + 0xC + 4 * i16(job + 0x24)`: the counters go to its first four
/// words and the commands to `block + 0x10 + [block + 0x10]`, with their byte count at `+0x14`.
fn store_results(e: &mut Engine, job: u32, input: u32) {
    let deferred = at(input, OWNER_AT);
    let buffer = at(input, BUFFER_AT);
    let index = i32::from(e.mem.u16(job + 0x24) as i16);
    let table = e.mem.u32(job + 0x20);
    let block = e
        .mem
        .u32(table.wrapping_add(0xc).wrapping_add((index * 4) as u32));
    for word in 0..4 {
        let counter = e.mem.u32(deferred + 8 + 4 * word);
        e.mem.set_u32(block + 4 * word, counter);
    }
    let size = e.mem.u32(buffer + 4);
    if size != 0 {
        let mut to = block
            .wrapping_add(0x10)
            .wrapping_add(e.mem.u32(block + 0x10));
        let mut from = buffer.wrapping_add(e.mem.u32(buffer));
        for _ in 0..(size as i32 >> 4).max(0) {
            for word in 0..4 {
                let value = e.mem.u32(from + 4 * word);
                e.mem.set_u32(to + 4 * word, value);
            }
            to = to.wrapping_add(0x10);
            from = from.wrapping_add(0x10);
        }
        e.mem.set_u32(block + 0x14, size);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d93310, fn_00d93310(Ptr) -> Ptr),
        entry!(0x00d93380, hk_cpu_agent_sector_job(Ptr, u32, Ptr) -> u32),
        entry!(
            0x00d93330,
            hkp_deferred_constraint_owner_add_constraint_to_critical_locked_island(Ptr, u32)
        ),
        entry!(
            0x00d93340,
            hkp_deferred_constraint_owner_remove_constraint_from_critical_locked_island(Ptr, u32)
        ),
        entry!(
            0x00d93350,
            hkp_deferred_constraint_owner_add_callback_request(Ptr, Ptr, u32)
        ),
    ]
}

/// What the tests of the CPU job functions share: the record stream and a double for the call
/// every job ends with.
#[cfg(test)]
pub(crate) mod test_support {
    use super::JOB_DONE;
    use crate::prelude::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, monitor, new_log, Log,
    };

    /// The calls the doubles saw and the record stream the job writes to.
    pub(crate) struct JobRig {
        pub(crate) log: Log,
        stream: u32,
        records: u32,
    }

    /// An engine with the record stream and a logging double for the final call, which answers
    /// 0x55.
    pub(crate) fn job_rig() -> (Engine, JobRig) {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, JOB_DONE, 0x55);
        let (stream, records) = monitor(&mut e, 0x200);
        (
            e,
            JobRig {
                log,
                stream,
                records,
            },
        )
    }

    impl JobRig {
        /// The labels of the records written so far. With `start` the first record is the
        /// 16-byte start record and gives two labels (at `+0` and `+0xC`); the others are 12
        /// bytes.
        pub(crate) fn labels(&self, e: &Engine, start: bool) -> Vec<u32> {
            let end = e.mem.u32(self.stream + 4);
            let mut labels = Vec::new();
            let mut at = self.records;
            if start {
                labels.push(e.mem.u32(at));
                labels.push(e.mem.u32(at + 0xc));
                at += 0x10;
            }
            while at < end {
                labels.push(e.mem.u32(at));
                at += 12;
            }
            labels
        }

        /// The labels of the records written so far when every record is 12 bytes except the
        /// ones that start with `wide`, which are 16 bytes and give two labels (at `+0` and
        /// `+0xC`).
        pub(crate) fn labels_with_wide(&self, e: &Engine, wide: u32) -> Vec<u32> {
            let end = e.mem.u32(self.stream + 4);
            let mut labels = Vec::new();
            let mut at = self.records;
            while at < end {
                let label = e.mem.u32(at);
                labels.push(label);
                if label == wide {
                    labels.push(e.mem.u32(at + 0xc));
                    at += 4;
                }
                at += 12;
            }
            labels
        }

        /// Checks that the final call ran once, with the second word as `ECX` and the job twice,
        /// and that the job function returned its answer.
        pub(crate) fn assert_finished(&self, returned: u32, second: u32, job: u32) {
            assert_eq!(returned, 0x55);
            assert_eq!(calls(&self.log, JOB_DONE), vec![vec![second, job, job, 0]]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, monitor, new_log, profile_labels, Log,
    };
    use std::rc::Rc;

    #[test]
    fn fn_00d93310_sets_the_three_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00d9_3310, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x10);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0x100);
    }

    #[test]
    fn locking_a_constraint_counts_up_and_unlocking_counts_down() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        e.call(0x00d9_3330, &args![this, 0xc1u32]);
        e.call(0x00d9_3330, &args![this, 0xc2u32]);
        assert_eq!(e.mem.u32(this + 0x18), 2);
        assert_eq!(e.mem.u32(this + 0x1c), 0xc2);
        e.call(0x00d9_3340, &args![this, 0xc3u32]);
        assert_eq!(e.mem.u32(this + 0x18), 1);
        assert_eq!(e.mem.u32(this + 0x1c), 0xc3);
        e.call(0x00d9_3340, &args![this, 0xc4u32]);
        e.call(0x00d9_3340, &args![this, 0xc5u32]);
        assert_eq!(e.mem.u32(this + 0x18), 0xffff_ffff);
    }

    #[test]
    fn callback_request_goes_to_the_constraint_object_or_the_owner() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        let target = e.mem.alloc(0x20);
        let constraint = e.mem.alloc(0x40);
        e.mem.set_u32(constraint + 0x30, target);
        e.mem.set_u8(target + 0x12, 0x01);
        e.call(0x00d9_3350, &args![this, constraint, 0x1234_5606u32]);
        assert_eq!(e.mem.u8(target + 0x12), 0x07);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        e.mem.set_u32(constraint + 0x30, 0);
        e.mem.set_u32(this + 0x20, 0x100);
        e.call(0x00d9_3350, &args![this, constraint, 0x1234_5606u32]);
        assert_eq!(e.mem.u32(this + 0x20), 0x1234_5706);
        assert_eq!(e.mem.u8(target + 0x12), 0x07);
    }

    /// The agents' vtable doubles live at `AGENT + slot` and the allocator's at
    /// `ALLOCATOR + slot`.
    const AGENT: u32 = 0x0900_0000;
    const ALLOCATOR: u32 = 0x0910_0000;
    /// The agent double's script bits, kept in the word at `entry + 0x1c`: move the output
    /// cursor, report a time of impact, make an add command pending, make a remove command
    /// pending, leave a scratch block to free, and count 5, 2, 3, 4 in the owner's counters.
    const MOVE_CURSOR: u32 = 1;
    const FIND_TOI: u32 = 2;
    const PEND_ADD: u32 = 4;
    const PEND_REMOVE: u32 = 8;
    const LEAVE_SCRATCH: u32 = 0x10;
    const COUNT: u32 = 0x20;

    struct Rig {
        log: Log,
        owner: u32,
        world_toi: u32,
        dispatcher: u32,
        agent: u32,
        allocator: u32,
    }

    /// Doubles for everything the job calls, an owner whose collision input points to a
    /// dispatcher (the pair information of class 0 at `+0x1c20` has the value 0xAA at `+0x10`,
    /// class 1 at `+0x1c60` has 0xBB) and
    /// the record stream.
    fn rig(e: &mut Engine) -> Rig {
        let log = new_log();
        for address in [
            COPY_COLLISION_INPUT,
            ADD_TOI_EVENT,
            LOCK_ISLAND,
            UNLOCK_ISLAND,
            RUN_COMMANDS,
        ] {
            log_double(e, &log, address, 0);
        }
        log_double(e, &log, JOB_DONE, 0x77);
        let allocator = logging_object(e, &log, ALLOCATOR);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let agent = logging_object(e, &log, AGENT);
        let sink = Rc::clone(&log);
        e.register_double(PROCESS_AGENT, move |e, a| {
            let input = a[1];
            let mut words = a.to_vec();
            words.extend([
                e.mem.u32(input + 0x14),
                e.mem.u32(input + 0x60),
                e.mem.u32(input + 0x50),
            ]);
            sink.borrow_mut().push((PROCESS_AGENT, words));
            let script = e.mem.u32(a[0] + 0x1c);
            let owner = input - 0x140;
            if script & MOVE_CURSOR != 0 {
                e.mem.set_u32(a[2], input + 0xa0);
            }
            if script & FIND_TOI != 0 {
                e.mem.set_f32(input + 0x30b0, 0.5);
            }
            if script & PEND_ADD != 0 {
                e.mem.set_u32(owner + 0x18, 1);
                e.mem.set_u32(owner + 0x1c, 0xc1);
                e.mem.set_u32(owner + 0x20, 0x55);
            }
            if script & PEND_REMOVE != 0 {
                e.mem.set_u32(owner + 0x18, u32::MAX);
                e.mem.set_u32(owner + 0x1c, 0xc2);
            }
            if script & LEAVE_SCRATCH != 0 {
                e.mem.set_u32(input + 0x64, 0x5000);
            }
            if script & COUNT != 0 {
                for (offset, value) in [(8, 5), (0xc, 2), (0x10, 3), (0x14, 4)] {
                    e.mem.set_u32(owner + offset, value);
                }
            }
            Ret::default()
        });
        let dispatcher = e.mem.alloc(0x2000);
        for (offset, value) in [(0x1c30, 0xaa), (0x1c70, 0xbb)] {
            e.mem.set_u32(dispatcher + offset, value);
        }
        let world_toi = e.mem.alloc(0x200);
        let owner = e.mem.alloc(0x200);
        e.mem.set_u32(owner, 0x1234);
        for word in 0..0x14 {
            e.mem.set_u32(owner + 0x90 + 4 * word, 0x100 + word);
        }
        e.mem.set_u32(owner + 0x90, dispatcher);
        e.mem.set_u32(owner + 0x110, world_toi);
        e.set_global(FLOAT_MAX, f32::from_bits(0x7f7f_ffee));
        e.set_global(DOUBLE_MAX, f64::from(f32::from_bits(0x7f7f_ffee)));
        Rig {
            log,
            owner,
            world_toi,
            dispatcher,
            agent,
            allocator,
        }
    }

    /// A 0x80-byte entry at `at`: agent `rig.agent`, class `class`, the words `at + 1` and
    /// `at + 2` in `+0x10`, `+0x14`, and the double's script.
    fn entry(e: &mut Engine, rig: &Rig, at: u32, class: u8, script: u32) {
        e.mem.set_u32(at + 8, rig.agent);
        e.mem.set_u8(at + 0xc, class);
        e.mem.set_u32(at + 0x10, at + 1);
        e.mem.set_u32(at + 0x14, at + 2);
        e.mem.set_u32(at + 0x1c, script);
    }

    /// A world whose chunk table lists `chunks`, whose last chunk is number `last` and holds
    /// `last_size` bytes, and whose counters are 3, 10, 20, 30.
    fn chunk_world(e: &mut Engine, chunks: &[u32], last_size: u32, last: u32) -> u32 {
        let world = e.mem.alloc(0x80);
        let table = e.mem.alloc(4 * chunks.len() as u32);
        for (slot, chunk) in chunks.iter().enumerate() {
            e.mem.set_u32(table + 4 * slot as u32, *chunk);
        }
        e.mem.set_u32(world + 0x5c, table);
        e.mem.set_u32(world + 0x58, last_size);
        e.mem.set_u32(world + 0x60, last);
        for (offset, value) in [(8, 3), (0xc, 10), (0x10, 20), (0x14, 30)] {
            e.mem.set_u32(world + offset, value);
        }
        world
    }

    /// A job of the given type with `count` chunks or list entries, four job words
    /// 0x31..0x34 and a result table whose block number `index` is `result` (none when
    /// `result` is 0).
    fn job(e: &mut Engine, kind: u8, world: u32, count: u16, index: i16, result: u32) -> u32 {
        let job = e.mem.alloc(0x50);
        e.mem.set_u8(job, kind);
        e.mem.set_u32(job + 0x14, world);
        if result != 0 {
            let table = e.mem.alloc(0x40);
            e.mem.set_u32(table + 0xc + 4 * (index as u32), result);
            e.mem.set_u32(job + 0x20, table);
        }
        e.mem.set_u16(job + 0x24, index as u16);
        e.mem.set_u16(job + 0x28, count);
        for word in 0..4 {
            e.mem.set_u32(job + 0x30 + 4 * word, 0x31 + word);
        }
        job
    }

    /// The first words of the calls the agent double saw: the entries, in order.
    fn processed(log: &Log) -> Vec<u32> {
        calls(log, PROCESS_AGENT)
            .iter()
            .map(|words| words[0])
            .collect()
    }

    #[test]
    fn sector_walk_processes_each_entry_and_hands_the_results_over() {
        let mut e = engine();
        let rig = rig(&mut e);
        let (stream, records) = monitor(&mut e, 0x100);
        let chunk = e.mem.alloc(0x200);
        entry(
            &mut e,
            &rig,
            chunk,
            0,
            MOVE_CURSOR | FIND_TOI | PEND_ADD | COUNT | LEAVE_SCRATCH,
        );
        entry(&mut e, &rig, chunk + 0x80, 1, PEND_REMOVE);
        let world = chunk_world(&mut e, &[chunk], 0x100, 1);
        let result = e.mem.alloc(0x80);
        e.mem.set_u32(result + 0x10, 8);
        let job = job(&mut e, 0xd, world, 1, 0, result);
        let returned = e.call(0x00d9_3380, &args![rig.owner, 0x4444u32, job]).u32();
        assert_eq!(returned, 0x77);
        assert_eq!(calls(&rig.log, JOB_DONE), vec![vec![0x4444, job, job, 0]]);
        let seen = calls(&rig.log, PROCESS_AGENT);
        let input = seen[0][1];
        // The input is a copy of the owner's, which is given to the copy function too.
        assert_eq!(
            calls(&rig.log, COPY_COLLISION_INPUT),
            vec![vec![input, rig.owner + 0x90]]
        );
        let cursor = input + 0x80;
        let class_0 = rig.dispatcher + 0x1c20;
        let class_1 = rig.dispatcher + 0x1c60;
        assert_eq!(
            seen,
            vec![
                vec![chunk, input, cursor, rig.agent, 0xaa, class_0, 0x31],
                vec![chunk + 0x80, input, cursor, rig.agent, 0xbb, class_1, 0x31],
            ]
        );
        // The first agent moved the cursor: its virtual function runs and a time of impact
        // is reported.
        assert_eq!(
            calls(&rig.log, AGENT + 0x18),
            vec![vec![rig.agent, chunk + 1, chunk + 2, input, cursor]]
        );
        assert_eq!(
            calls(&rig.log, ADD_TOI_EVENT),
            vec![vec![rig.world_toi, cursor, chunk, rig.world_toi + 0x100]]
        );
        // The counters and the two commands (add, then remove) are in the result block.
        let counters: Vec<u32> = (0..4).map(|word| e.mem.u32(result + 4 * word)).collect();
        assert_eq!(counters, vec![5, 2, 3, 4]);
        assert_eq!(e.mem.u32(result + 0x14), 0x20);
        let commands: Vec<u32> = (0..8)
            .map(|word| e.mem.u32(result + 0x18 + 4 * word))
            .collect();
        assert_eq!(commands, vec![0, 0xc1, 0x55, 0, 1, 0xc2, 0, 0]);
        // The scratch block the agent left is freed through the allocator.
        assert_eq!(
            calls(&rig.log, ALLOCATOR + 8),
            vec![vec![rig.allocator, 0x5000, 0x200]]
        );
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![LABEL_TT, LABEL_ET]
        );
    }

    #[test]
    fn without_a_result_block_the_world_takes_the_counters_under_the_lock() {
        let mut e = engine();
        let rig = rig(&mut e);
        monitor(&mut e, 0x100);
        let chunk = e.mem.alloc(0x200);
        entry(&mut e, &rig, chunk, 0, COUNT | PEND_ADD);
        let world = chunk_world(&mut e, &[chunk], 0x80, 1);
        let job_one = job(&mut e, 0xd, world, 1, 0, 0);
        e.call(0x00d9_3380, &args![rig.owner, 0u32, job_one]);
        let input = calls(&rig.log, PROCESS_AGENT)[0][1];
        let commands = input - 0x110 + 0x10;
        let sequence: Vec<u32> = rig
            .log
            .borrow()
            .iter()
            .map(|(address, _)| *address)
            .filter(|address| [LOCK_ISLAND, RUN_COMMANDS, UNLOCK_ISLAND].contains(address))
            .collect();
        assert_eq!(sequence, vec![LOCK_ISLAND, RUN_COMMANDS, UNLOCK_ISLAND]);
        assert_eq!(calls(&rig.log, LOCK_ISLAND), vec![vec![0x1234, world]]);
        assert_eq!(
            calls(&rig.log, RUN_COMMANDS),
            vec![vec![0x1234, commands, 0x10]]
        );
        assert_eq!(calls(&rig.log, UNLOCK_ISLAND), vec![vec![0x1234, world]]);
        // The maximum of 3 and 5, and the sums 10 + 2, 20 + 3, 30 + 4.
        let totals: Vec<u32> = [8, 0xc, 0x10, 0x14]
            .iter()
            .map(|offset| e.mem.u32(world + offset))
            .collect();
        assert_eq!(totals, vec![5, 12, 23, 34]);
        // Nothing counted and nothing queued: no lock at all.
        rig.log.borrow_mut().clear();
        entry(&mut e, &rig, chunk, 0, 0);
        let job_two = job(&mut e, 0xd, world, 1, 0, 0);
        e.call(0x00d9_3380, &args![rig.owner, 0u32, job_two]);
        assert!(calls(&rig.log, LOCK_ISLAND).is_empty());
        assert!(calls(&rig.log, UNLOCK_ISLAND).is_empty());
        assert!(calls(&rig.log, RUN_COMMANDS).is_empty());
    }

    #[test]
    fn sector_walk_follows_the_chunks_and_stops_on_the_flag() {
        let mut e = engine();
        let rig = rig(&mut e);
        monitor(&mut e, 0x100);
        let first = e.mem.alloc(0x200);
        let second = e.mem.alloc(0x80);
        for slot in 0..4 {
            entry(&mut e, &rig, first + 0x80 * slot, 0, 0);
        }
        entry(&mut e, &rig, second, 0, 0);
        // Chunk number 2 is the last one and holds a single entry.
        let world = chunk_world(&mut e, &[first, second], 0x80, 2);
        let result = e.mem.alloc(0x80);
        let walk = job(&mut e, 0xd, world, 2, 0, result);
        e.call(0x00d9_3380, &args![rig.owner, 0u32, walk]);
        assert_eq!(
            processed(&rig.log),
            vec![first, first + 0x80, first + 0x100, first + 0x180, second]
        );
        // The stop flag ends the walk after the first entry.
        rig.log.borrow_mut().clear();
        e.set_global(STOP_FLAG, 1u32);
        e.call(0x00d9_3380, &args![rig.owner, 0u32, walk]);
        assert_eq!(processed(&rig.log), vec![first]);
    }

    #[test]
    fn list_walk_picks_the_pair_information_array_and_always_reports() {
        let mut e = engine();
        let rig = rig(&mut e);
        let (stream, records) = monitor(&mut e, 0x100);
        let entries = e.mem.alloc(0x100);
        entry(
            &mut e,
            &rig,
            entries,
            0,
            FIND_TOI | MOVE_CURSOR | PEND_ADD | COUNT,
        );
        entry(&mut e, &rig, entries + 0x80, 1, 0);
        // The list holds a spare word, then the two entries: it starts at index 1 * 1.
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list + 4, entries);
        e.mem.set_u32(list + 8, entries + 0x80);
        let result = e.mem.alloc(0x80);
        e.mem.set_u32(result + 0x10, 8);
        let walk = job(&mut e, 0x5, 0, 2, 1, result);
        e.mem.set_u16(walk + 0x2a, 1);
        e.mem.set_u32(walk + 0x2c, list);
        e.call(0x00d9_3380, &args![rig.owner, 0u32, walk]);
        let seen = calls(&rig.log, PROCESS_AGENT);
        let input = seen[0][1];
        let cursor = input + 0x80;
        assert_eq!(seen[0][4..], [0xaa, rig.dispatcher + 0x1c20, 0x31]);
        assert_eq!(seen[1][4..], [0xbb, rig.dispatcher + 0x1c60, 0x31]);
        assert_eq!(
            calls(&rig.log, ADD_TOI_EVENT),
            vec![vec![rig.world_toi, cursor, entries, rig.world_toi + 0x100]]
        );
        assert_eq!(e.mem.u32(result), 5);
        assert_eq!(e.mem.u32(result + 0x14), 0x10);
        assert_eq!(e.mem.u32(result + 0x18 + 4), 0xc1);
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![LABEL_TT_TOI, LABEL_ET_TOI]
        );
        // With the flag byte set, the pair information is the one at `+0x1de0`.
        rig.log.borrow_mut().clear();
        e.mem.set_u8(walk + 0x40, 1);
        e.call(0x00d9_3380, &args![rig.owner, 0u32, walk]);
        let seen = calls(&rig.log, PROCESS_AGENT);
        assert_eq!(seen[0][4..], [0xaa, rig.dispatcher + 0x1de0, 0x31]);
        assert_eq!(seen[1][4..], [0xbb, rig.dispatcher + 0x1de0, 0x31]);
    }
}
