//! `fallout/ai/run_0096ad00` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0096ad30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the capacity of an array of pointers (the engine map gives it no name). The array object
/// (`ECX`) holds the pointer to the elements at `+4`, the capacity at `+8`, the count at `+0xa`
/// and the number of non-null elements at `+0xc`, all three counts being words. A capacity equal
/// to the current one does nothing. A smaller one first drops the elements from the new capacity
/// up to the count (clearing them and counting the non-null ones out) and lowers the count. A
/// new block is then taken from `0096afc0` (none for capacity 0), the elements below the count
/// are copied, the rest cleared, and the old block is given to `004ede70`.
pub fn fn_0096ad30(e: &mut Engine, this: Ptr, new_capacity: u32) {
    let base = this.addr();
    if new_capacity == u32::from(e.mem.u16(base + 8)) {
        return;
    }
    if new_capacity < u32::from(e.mem.u16(base + 0xa)) {
        let mut index = new_capacity as u16;
        while u32::from(index) < u32::from(e.mem.u16(base + 0xa)) {
            let elements = e.mem.u32(base + 4);
            if e.mem.u32(elements + u32::from(index) * 4) != 0 {
                e.mem.set_u32(elements + u32::from(index) * 4, 0);
                let count = e.mem.u16(base + 0xc);
                e.mem.set_u16(base + 0xc, count.wrapping_sub(1));
            }
            index = index.wrapping_add(1);
        }
        e.mem.set_u16(base + 0xa, new_capacity as u16);
    }
    let old = e.mem.u32(base + 4);
    e.mem.set_u16(base + 8, new_capacity as u16);
    if new_capacity > 0 {
        let capacity = u32::from(e.mem.u16(base + 8));
        let block = e.call(0x0096_afc0, &args![capacity]).u32();
        e.mem.set_u32(base + 4, block);
        let mut index = 0u16;
        while u32::from(index) < u32::from(e.mem.u16(base + 0xa)) {
            let value = e.mem.u32(old + u32::from(index) * 4);
            let elements = e.mem.u32(base + 4);
            e.mem.set_u32(elements + u32::from(index) * 4, value);
            index = index.wrapping_add(1);
        }
        index = e.mem.u16(base + 0xa);
        while u32::from(index) < u32::from(e.mem.u16(base + 8)) {
            let elements = e.mem.u32(base + 4);
            e.mem.set_u32(elements + u32::from(index) * 4, 0);
            index = index.wrapping_add(1);
        }
    } else {
        e.mem.set_u32(base + 4, 0);
    }
    e.call(0x004e_de70, &args![old]);
}

// Translated from 0096ae90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the pointer `*value` at `index` of the array `0096ad30` manages (the engine map gives
/// it no name). An index at or past the count raises the count to `index + 1` (as a word) and
/// counts a non-null pointer in; below the count the number of non-null elements follows the
/// element being replaced.
pub fn fn_0096ae90(e: &mut Engine, this: Ptr, index: u32, value: Ptr) {
    let base = this.addr();
    let pointer = e.mem.u32(value.addr());
    if index >= u32::from(e.mem.u16(base + 0xa)) {
        e.mem.set_u16(base + 0xa, index.wrapping_add(1) as u16);
        if pointer != 0 {
            let count = e.mem.u16(base + 0xc);
            e.mem.set_u16(base + 0xc, count.wrapping_add(1));
        }
    } else {
        let elements = e.mem.u32(base + 4);
        let current = e.mem.u32(elements + index.wrapping_mul(4));
        if pointer != 0 {
            if current == 0 {
                let count = e.mem.u16(base + 0xc);
                e.mem.set_u16(base + 0xc, count.wrapping_add(1));
            }
        } else if current != 0 {
            let count = e.mem.u16(base + 0xc);
            e.mem.set_u16(base + 0xc, count.wrapping_sub(1));
        }
    }
    let elements = e.mem.u32(base + 4);
    e.mem.set_u32(elements + index.wrapping_mul(4), pointer);
}

// Translated from 0096afe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the pointer array `0096ad30` manages (the engine map gives it no name). Stores
/// the vtable `0108c0c8` at `+0`, the low word of `capacity` at `+8` and the low word of
/// `second_word` at `+0xe`, clears the count (`+0xa`) and the non-null count (`+0xc`), and
/// takes the element block (`+4`) from `0096afc0` when the capacity word is above zero
/// (otherwise `+4` is 0). Parameters: two words, `RET 8`, `this` in `ECX`; both are pushed as
/// whole words and only their low 16 bits are read. Returns `this` (`EAX`).
pub fn fn_0096afe0(e: &mut Engine, this: Ptr, capacity: u32, second_word: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0108_c0c8);
    e.mem.set_u16(base + 8, capacity as u16);
    e.mem.set_u16(base + 0xe, second_word as u16);
    e.mem.set_u16(base + 0xa, 0);
    e.mem.set_u16(base + 0xc, 0);
    let words = u32::from(e.mem.u16(base + 8));
    if words > 0 {
        let block = e.call(0x0096_afc0, &args![words]).u32();
        e.mem.set_u32(base + 4, block);
    } else {
        e.mem.set_u32(base + 4, 0);
    }
    this
}

// Translated from 0096af30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A subclass constructor of the array `0096afe0` builds (the engine map gives it no name): runs
/// `0096afe0(this, capacity, second_word)` of this unit, then overwrites the vtable at `+0`
/// with `0108c0d0`. Returns `this`. Parameters: two words, `RET 8`, `this` in `ECX`.
pub fn fn_0096af30(e: &mut Engine, this: Ptr, capacity: u32, second_word: u32) -> Ptr {
    fn_0096afe0(e, this, capacity, second_word);
    e.mem.set_u32(this.addr(), 0x0108_c0d0);
    this
}

/// `005de0f0` (unnamed in the engine map): `thiscall` on the process array (`this + 4`),
/// `RET 4`; the first index of the list of process level `level`.
const LEVEL_BEGIN: u32 = 0x005d_e0f0;
/// `005be5c0` (unnamed in the engine map): `thiscall` on the process array, `RET 4`; the end
/// index of the list of process level `level`.
const LEVEL_END: u32 = 0x005b_e5c0;
/// `00968670` (unnamed in the engine map): `thiscall` on the process array, `RET 4`; the actor
/// at `index` (0 for an empty slot).
const ACTOR_AT: u32 = 0x0096_8670;
/// `008d8520` on an actor (`thiscall`, no stack words; the engine map names the folded body
/// `MiddleHighProcess::GetSavedAcquireObject`): the actor's process object.
const ACTOR_PROCESS: u32 = 0x008d_8520;
/// The player character (`PlayerCharacter *`).
const PLAYER: u32 = 0x011d_ea3c;
/// `0.0` (`double`).
const ZERO_01012060: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE_01012070: u32 = 0x0101_2070;
/// `-1.0` (`double`).
const MINUS_ONE_0101A6B0: u32 = 0x0101_a6b0;
/// The byte that, when set, skips the per-level updates (`011e0e34`).
const UPDATES_DISABLED_011E0E34: u32 = 0x011e_0e34;
/// The game calendar (`Calendar::GetHour`, `00867da0`, is called on it).
const CALENDAR_011DE7B8: u32 = 0x011d_e7b8;
/// The lock `0040fbf0` enters and `0040fba0` leaves around a process change.
const PROCESS_LOCK_011F11A0: u32 = 0x011f_11a0;

/// The x87 condition bits `FCOM` leaves in `AH` (after `FNSTSW AX`) for `st0` compared with
/// `st1`: C0 (0x01) for less, C3 (0x40) for equal, C3|C2|C0 (0x45) unordered, none for greater.
fn fpu_compare(st0: f64, st1: f64) -> u8 {
    if st0.is_nan() || st1.is_nan() {
        0x45
    } else if st0 < st1 {
        0x01
    } else if st0 == st1 {
        0x40
    } else {
        0
    }
}

/// The parity flag of `TEST AH,mask`: set when the masked byte has an even number of bits.
fn parity_even(value: u8) -> bool {
    value.count_ones() % 2 == 0
}

/// A `double` constant of the exe, as the `FCOMP qword` operand sees it.
fn double_at(e: &Engine, address: u32) -> f64 {
    f64::from_bits(e.mem.u64(address))
}

/// `ST0` of a call stored by `FSTP dword` and loaded again by `FLD dword`.
fn stored_float(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// Makes the 8-byte actor list the level updates collect actors in: `00401000(8)`, then the
/// constructor `0096a2d0` when the allocation succeeded (0 otherwise).
fn new_actor_list(e: &mut Engine) -> u32 {
    let block = e.call(0x0040_1000, &args![8u32]).u32();
    if block != 0 {
        e.call(0x0096_a2d0, &args![block]).u32()
    } else {
        0
    }
}

/// Appends the actor in the word at `slot` to `list` (`005ae3d0`, `RET 4`).
fn collect_actor(e: &mut Engine, list: u32, slot: u32) {
    e.call(0x005a_e3d0, &args![list, slot]);
}

/// Empties (`00470470`) and deletes (`004702f0(list, 1)`, when not null) an actor list.
fn free_actor_list(e: &mut Engine, list: u32) {
    e.call(0x0047_0470, &args![list]);
    if list != 0 {
        e.call(0x0047_02f0, &args![list, 1u32]);
    }
}

/// The actor of a list node: `006815c0(node)` gives the address of the node's element.
fn node_actor(e: &mut Engine, node: u32) -> u32 {
    let element = e.call(0x0068_15c0, &args![node]).u32();
    e.mem.u32(element)
}

/// Whether the list node is not null and holds an actor (`006815c0` gives its element).
fn node_has_actor(e: &mut Engine, node: u32) -> bool {
    if node == 0 {
        return false;
    }
    let element = e.call(0x0068_15c0, &args![node]).u32();
    e.mem.u32(element) != 0
}

/// The next node (`00726070`).
fn next_node(e: &mut Engine, node: u32) -> u32 {
    e.call(0x0072_6070, &args![node]).u32()
}

/// Under the process lock, deletes the actor's process object (its vtable slot 0 with 1) and
/// clears it (`MiddleHighProcess::SetSavedAcquireObject(actor, 0)`, `00407800`, the name the
/// engine map gives the folded body).
fn drop_process(e: &mut Engine, actor: u32) {
    e.call(0x0040_fbf0, &args![PROCESS_LOCK_011F11A0, 0u32]);
    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
    if process != 0 {
        e.vcall(process, 0, &args![1u32]);
    }
    e.call(0x0040_7800, &args![actor, 0u32]);
    e.call(0x0040_fba0, &args![PROCESS_LOCK_011F11A0]);
}

/// The tail the three level updates share: every actor collected in `list` is removed from the
/// process level `level` (`ProcessArray::RemoveActor(array, actor, level)`, `0096aaf0`); an
/// actor for which `00440da0` is true loses its process object; the list is then freed. With
/// `use_slot_2ec` (the level-1 update), an actor whose vtable slot `0x2ec` answers true instead
/// gets its own vtable slot `0x10` with 1 under the lock.
fn remove_collected(e: &mut Engine, this: Ptr, list: u32, level: u32, use_slot_2ec: bool) {
    let array = this.addr().wrapping_add(4);
    let mut node = list;
    while node_has_actor(e, node) {
        let actor = node_actor(e, node);
        e.call(0x0096_aaf0, &args![array, actor, level]);
        if use_slot_2ec && e.vcall(actor, 0x2ec, &args![]).bool() {
            e.call(0x0040_fbf0, &args![PROCESS_LOCK_011F11A0, 0u32]);
            if actor != 0 {
                e.vcall(actor, 0x10, &args![1u32]);
            }
            e.call(0x0040_fba0, &args![PROCESS_LOCK_011F11A0]);
        } else if e.call(0x0044_0da0, &args![actor]).bool() {
            drop_process(e, actor);
        }
        node = next_node(e, node);
    }
    free_actor_list(e, list);
}

/// The common update of an actor whose process says it may run (`process` vtable slot
/// `0x1ec`): the actor's vtable slot `0x25c` with `time` (copied as a `float`), its script
/// (`TESObjectREFR::RunScript`, `00565870`) when `Script::GetProcessScripts` (`005ac740`) is
/// true, and `004f8960` on the actor when its vtable slot `0x100` is true (the result only sets a
/// flag the exe never reads).
fn run_actor_update(e: &mut Engine, actor: u32, process: u32, time: f32) {
    if !e.vcall(process, 0x1ec, &args![]).bool() {
        return;
    }
    e.vcall(actor, 0x25c, &args![x87_quiet_bits(time.to_bits())]);
    if e.call(0x005a_c740, &args![]).bool() {
        e.call(0x0056_5870, &args![actor]);
    }
    let character = if actor != 0 && e.vcall(actor, 0x100, &args![]).bool() {
        actor
    } else {
        0
    };
    if character != 0 {
        e.call(0x004f_8960, &args![character]);
    }
}

// Translated from 0096ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the pointer array `0096afe0` builds (the engine map gives it no name): stores
/// the vtable `0108c0c8` and frees the element block (`+4`) through `004ede70`.
pub fn fn_0096ad00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_c0c8);
    let block = e.mem.u32(this.addr().wrapping_add(4));
    e.call(0x004e_de70, &args![block]);
}

// Translated from 0096b050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0096b050 - the C++ exception-unwinding frame (FS:[0]) that frees the actor list when its constructor throws is not translated
/// The update of the actors of process level 3 (the engine map gives it no name; `RET 8`:
/// `time` and `all`). It runs from the index saved at `011e0e30` (reset to the level's first
/// index when 0 or at or past the level's end) to the level's end, saving the next index after
/// each actor. Unless `all` (its low byte) is set, every fifth index checks the time budget: the
/// deadline is `QueryPerformanceCounter` (import `00fdf0a0`) plus `__alldiv(__allmul(budget,
/// frequency), 1000)` with the budget the word `0043d4d0(011e0e64)` points to and the frequency
/// at `011e0e20`; past it the loop stops. An actor with a process object, not `00576d30`, is
/// collected for removal when `00440da0` or `00440d80` is true or its process level
/// (`0045cd60`) is not 3; otherwise, unless updates are disabled (`011e0e34`), an actor without
/// `00437bb0` and one whose last update hour (`007df1f0`, stored as a `float`) is -1 get
/// `LowProcess::ComputeLastTimeProcessed` (`00907650`) on the process; the others update
/// (`run_actor_update`) when at least an hour has passed since (`Calendar::GetHour` minus that
/// hour, as a `float`, not below 1), the difference is negative, or `time` is not 0. The
/// collected actors leave level 3 (`remove_collected`).
pub fn fn_0096b050(e: &mut Engine, this: Ptr, time: f32, all: u32) {
    let list = new_actor_list(e);
    let deadline = e.with_stack(8, |e, counter| {
        e.call(0x00fd_f0a0, &args![counter]);
        let start = e.mem.u64(counter.addr());
        let budget_slot = e.call(0x0043_d4d0, &args![0x011e_0e64u32]).u32();
        let budget = e.mem.u32(budget_slot);
        let frequency_low = e.mem.u32(0x011e_0e20);
        let frequency_high = e.mem.u32(0x011e_0e24);
        let product = e
            .call(
                0x00ec_c300,
                &args![budget, 0u32, frequency_low, frequency_high],
            )
            .u64();
        let ticks = e
            .call(
                0x00ec_c250,
                &args![product as u32, (product >> 32) as u32, 1000u32, 0u32],
            )
            .u64();
        ticks.wrapping_add(start)
    });
    let array = this.addr().wrapping_add(4);
    let end = e.call(LEVEL_END, &args![array, 3u32]).u32();
    let saved = e.mem.u32(0x011e_0e30);
    if saved == 0 || saved >= end {
        let first = e.call(LEVEL_BEGIN, &args![array, 3u32]).u32();
        e.mem.set_u32(0x011e_0e30, first);
    }
    let mut limit = end;
    if limit > end || all as u8 != 0 {
        limit = end;
    }
    let hour = stored_float(e.call(0x0086_7da0, &args![CALENDAR_011DE7B8]).f64());
    let mut out_of_time = false;
    let mut index = e.mem.u32(0x011e_0e30);
    e.with_stack(0xc, |e, locals| {
        let actor_slot = locals.addr();
        let now = actor_slot + 4;
        while index < limit && !out_of_time {
            let actor = e.call(ACTOR_AT, &args![array, index]).u32();
            e.mem.set_u32(actor_slot, actor);
            if all as u8 == 0 && index % 5 == 0 {
                e.call(0x00fd_f0a0, &args![now]);
                if e.mem.u64(now) as i64 >= deadline as i64 {
                    out_of_time = true;
                }
            }
            if actor != 0 {
                level_3_actor(e, actor, actor_slot, list, hour, time);
            }
            e.mem.set_u32(0x011e_0e30, index.wrapping_add(1));
            index = index.wrapping_add(1);
        }
    });
    remove_collected(e, this, list, 3, false);
}

/// One actor of `0096b050`'s loop (the actor is also in the word at `actor_slot`).
fn level_3_actor(e: &mut Engine, actor: u32, actor_slot: u32, list: u32, hour: f64, time: f32) {
    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
    if process == 0 || e.call(0x0057_6d30, &args![actor]).bool() {
        return;
    }
    if e.call(0x0044_0da0, &args![actor]).bool()
        || e.call(0x0044_0d80, &args![actor]).bool()
        || e.call(0x0045_cd60, &args![process]).u32() != 3
    {
        collect_actor(e, list, actor_slot);
        return;
    }
    if e.mem.u8(UPDATES_DISABLED_011E0E34) != 0 {
        return;
    }
    if !e.call(0x0043_7bb0, &args![actor]).bool() {
        e.call(0x0090_7650, &args![process]);
        return;
    }
    let last = stored_float(e.call(0x007d_f1f0, &args![process]).f64());
    if !parity_even(fpu_compare(last, double_at(e, MINUS_ONE_0101A6B0)) & 0x44) {
        e.call(0x0090_7650, &args![process]);
        return;
    }
    let passed = stored_float(x87_sub(hour, last));
    let due = fpu_compare(passed, double_at(e, ONE_01012070)) & 0x01 == 0
        || !parity_even(fpu_compare(passed, double_at(e, ZERO_01012060)) & 0x05)
        || parity_even(fpu_compare(x87_load32(time.to_bits()), double_at(e, ZERO_01012060)) & 0x44);
    if due {
        run_actor_update(e, actor, process, time);
    }
}

// Translated from 0096b470 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0096b470 - the C++ exception-unwinding frame (FS:[0]) that frees the actor list when its constructor throws is not translated
/// The update of the actors of process level 2 (the engine map gives it no name; `RET 8`, the
/// second word is not read). For each actor of the level (not null, not `00576d30`): it is
/// collected for removal when `00440da0` or `00440d80` is true or
/// `MobileObject::GetCurrentProcessType` (`00931850`) is not 2. Otherwise, unless updates are
/// disabled (`011e0e34`): a character (vtable slot `0x100`) without `00437bb0` and without
/// `00493bb0` gets its process's vtable slot `0x28`; the others compare `Calendar::GetHour` with
/// their process's `007df1f0` hour (both stored as `float`s): -1 runs `0092f190` on the actor,
/// otherwise the actor updates (`run_actor_update`) when 0.3 hour (`010290d0`) has passed, the
/// hour is before the last one, or `time` is not 0. The collected actors leave level 2.
pub fn fn_0096b470(e: &mut Engine, this: Ptr, time: f32, _unused_2: u32) {
    let list = new_actor_list(e);
    let array = this.addr().wrapping_add(4);
    let begin = e.call(LEVEL_BEGIN, &args![array, 2u32]).u32();
    let end = e.call(LEVEL_END, &args![array, 2u32]).u32();
    e.with_stack(4, |e, actor_slot| {
        let actor_slot = actor_slot.addr();
        let mut index = begin;
        while index < end {
            let actor = e.call(ACTOR_AT, &args![array, index]).u32();
            e.mem.set_u32(actor_slot, actor);
            if actor != 0 && !e.call(0x0057_6d30, &args![actor]).bool() {
                level_2_actor(e, actor, actor_slot, list, time);
            }
            index = index.wrapping_add(1);
        }
    });
    remove_collected(e, this, list, 2, false);
}

/// One actor of `0096b470`'s loop.
fn level_2_actor(e: &mut Engine, actor: u32, actor_slot: u32, list: u32, time: f32) {
    let character = if e.vcall(actor, 0x100, &args![]).bool() {
        actor
    } else {
        0
    };
    if e.call(0x0044_0da0, &args![actor]).bool()
        || e.call(0x0044_0d80, &args![actor]).bool()
        || e.call(0x0093_1850, &args![actor]).u32() != 2
    {
        collect_actor(e, list, actor_slot);
        return;
    }
    if e.mem.u8(UPDATES_DISABLED_011E0E34) != 0 {
        return;
    }
    if !e.call(0x0043_7bb0, &args![actor]).bool()
        && character != 0
        && !e.call(0x0049_3bb0, &args![character]).bool()
    {
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        e.vcall(process, 0x28, &args![]);
        return;
    }
    timed_update(e, actor, time, 0x0102_90d0);
}

/// The hour test of the level 1 and level 2 updates: `last` is the process's `007df1f0` hour
/// and `hour` `Calendar::GetHour` (both stored as `float`s); -1 runs `0092f190` on the actor;
/// otherwise the actor updates (`run_actor_update`, with a fresh `008d8520` process) when `hour
/// - last` (unrounded) is not below the `double` at `interval`, `hour < last`, or `time` is not 0.
fn timed_update(e: &mut Engine, actor: u32, time: f32, interval: u32) {
    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
    let last = stored_float(e.call(0x007d_f1f0, &args![process]).f64());
    let hour = stored_float(e.call(0x0086_7da0, &args![CALENDAR_011DE7B8]).f64());
    timed_update_with(e, actor, time, interval, last, hour);
}

/// The hour test of `timed_update` once both hours are read.
fn timed_update_with(e: &mut Engine, actor: u32, time: f32, interval: u32, last: f64, hour: f64) {
    if !parity_even(fpu_compare(last, double_at(e, MINUS_ONE_0101A6B0)) & 0x44) {
        e.call(0x0092_f190, &args![actor]);
        return;
    }
    let due = fpu_compare(x87_sub(hour, last), double_at(e, interval)) & 0x01 == 0
        || !parity_even(fpu_compare(hour, last) & 0x05)
        || parity_even(fpu_compare(x87_load32(time.to_bits()), double_at(e, ZERO_01012060)) & 0x44);
    if due {
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        run_actor_update(e, actor, process, time);
    }
}

// Translated from 0096b810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0096b810 - the C++ exception-unwinding frame (FS:[0]) that frees the actor list when its constructor throws is not translated
/// The update of the actors of process level 1 (the engine map gives it no name; `RET 8`, the
/// second word is not read). For each actor of the level (not null, not `00576d30`): it is
/// collected for removal when `00440da0` or `00440d80` is true or its process type
/// (`00931850`) is not 1. Otherwise, unless updates are disabled (`011e0e34`), the process's
/// `007df1f0` hour and `Calendar::GetHour` are read, the actor's vtable slot `0x100` is asked
/// (its answer is not used), and the hour test runs with 0.15 hour (`01070c80`): -1 runs
/// `0092f190`, otherwise the actor updates when due. The collected actors leave level 1; one
/// whose vtable slot `0x2ec` is true gets its slot `0x10` with 1 under the lock instead of
/// losing its process.
pub fn fn_0096b810(e: &mut Engine, this: Ptr, time: f32, _unused_2: u32) {
    let list = new_actor_list(e);
    let array = this.addr().wrapping_add(4);
    let begin = e.call(LEVEL_BEGIN, &args![array, 1u32]).u32();
    let end = e.call(LEVEL_END, &args![array, 1u32]).u32();
    e.with_stack(4, |e, actor_slot| {
        let actor_slot = actor_slot.addr();
        let mut index = begin;
        while index < end {
            let actor = e.call(ACTOR_AT, &args![array, index]).u32();
            e.mem.set_u32(actor_slot, actor);
            if actor != 0 && !e.call(0x0057_6d30, &args![actor]).bool() {
                if e.call(0x0044_0da0, &args![actor]).bool()
                    || e.call(0x0044_0d80, &args![actor]).bool()
                    || e.call(0x0093_1850, &args![actor]).u32() != 1
                {
                    collect_actor(e, list, actor_slot);
                } else if e.mem.u8(UPDATES_DISABLED_011E0E34) == 0 {
                    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
                    let last = stored_float(e.call(0x007d_f1f0, &args![process]).f64());
                    let hour = stored_float(e.call(0x0086_7da0, &args![CALENDAR_011DE7B8]).f64());
                    e.vcall(actor, 0x100, &args![]);
                    timed_update_with(e, actor, time, 0x0107_0c80, last, hour);
                }
            }
            index = index.wrapping_add(1);
        }
    });
    remove_collected(e, this, list, 1, true);
}

// Translated from 0096bbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): for the process level `00726070(visitor)` names, each
/// actor of the level (from `005de0f0` to `005be5c0`, both asked once) that is not null and not
/// `00576d30` is passed to the visitor's vtable slot 0.
pub fn fn_0096bbc0(e: &mut Engine, this: Ptr, visitor: u32) {
    let array = this.addr().wrapping_add(4);
    let level = e.call(0x0072_6070, &args![visitor]).u32();
    let end = e.call(LEVEL_END, &args![array, level]).u32();
    let level = e.call(0x0072_6070, &args![visitor]).u32();
    let mut index = e.call(LEVEL_BEGIN, &args![array, level]).u32();
    while index < end {
        let actor = e.call(ACTOR_AT, &args![array, index]).u32();
        if actor != 0 && !e.call(0x0057_6d30, &args![actor]).bool() {
            e.vcall(visitor, 0, &args![actor]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 0096bcd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0096bcd0 - the C++ exception-unwinding frame (FS:[0]) that ends the profiling scope and frees the follower lists on a throw is not translated
/// The update of the actors of process level 0 (the engine map gives it no name; `RET 8`, the
/// second word is not read). A profiling scope (`00404eb0(scope, 0x2a, 1, "...ProcessLists.cpp",
/// 0x235)` ... `00404ee0`) surrounds it; `008c8bd0` runs first and the character count at
/// `+0x103b0` is cleared. Each actor (not null, not `00576d30`) with a process object, process
/// type 0 and neither `00440d80` nor `00440da0`, when updates are not disabled (`011e0e34`):
/// a character (vtable slot `0x100`, counted at `+0x103b0`) for which `008b0520` is true is
/// handed to its process's vtable slot `0x24` (with 0) and `008c8fd0(process, 1)`; every other
/// actor gets its vtable slot `0x25c` with `time`. A character, while the player sleeps or rests
/// (`PlayerCharacter::IsSleepingorResting`), restores health and fatigue
/// (`Actor::RestoreHealth`/`RestoreFatigue` with the `float` at `010162c0`). For an actor that was
/// not handed over, its followers (the list at `+0xc` of `00422700(ExtraDataList)`) are copied
/// into a new list (`00905820`); each with a process that is not the player keeps following when
/// its package (`Actor::GetPackageSetAsPcurrent`, replaced by `ExtraDataList::GetPackageExtra`
/// when there is one) is of type 1 or 7 (`0041ca90`), its process being told `GetHour - 1.0`
/// (`00693d50`) when its process type is not 0; the others are collected and removed
/// (`ExtraDataList::RemoveFollower`). Both lists are freed and, when `008256d0` on the
/// follower list is true, `ExtraDataList::RemoveFollowerExtra` runs. An actor that fails the
/// first tests is added to the temporary change list (`0096e870`) when `00440da0` is true.
/// Finally `00977130(this, 0084d030(011f6394))`. The two counters the exe keeps on its stack
/// are never read.
pub fn fn_0096bcd0(e: &mut Engine, this: Ptr, time: f32, _unused_2: u32) {
    e.with_stack(8, |e, locals| {
        let scope = locals.addr();
        let actor_slot = scope + 4;
        e.call(
            0x0040_4eb0,
            &args![scope, 0x2au32, 1u32, 0x0108_c11cu32, 0x235u32],
        );
        e.call(0x008c_8bd0, &args![]);
        let base = this.addr();
        e.mem.set_u32(base.wrapping_add(0x103b0), 0);
        e.mem.set_u32(actor_slot, 0);
        let array = base.wrapping_add(4);
        let mut index = e.call(LEVEL_BEGIN, &args![array, 0u32]).u32();
        while index < e.call(LEVEL_END, &args![array, 0u32]).u32() {
            let actor = e.call(ACTOR_AT, &args![array, index]).u32();
            e.mem.set_u32(actor_slot, actor);
            if actor != 0 && !e.call(0x0057_6d30, &args![actor]).bool() {
                level_0_actor(e, this, actor, time);
            }
            index = index.wrapping_add(1);
        }
        let seconds = e.call(0x0084_d030, &args![0x011f_6394u32]).f64();
        e.call(0x0097_7130, &args![this, x87_store32(seconds)]);
        e.call(0x0040_4ee0, &args![scope]);
    });
}

/// One actor of `0096bcd0`'s loop.
fn level_0_actor(e: &mut Engine, this: Ptr, actor: u32, time: f32) {
    let eligible = !e.call(0x0044_0d80, &args![actor]).bool()
        && !e.call(0x0044_0da0, &args![actor]).bool()
        && e.call(ACTOR_PROCESS, &args![actor]).u32() != 0
        && e.call(0x0093_1850, &args![actor]).u32() == 0
        && e.mem.u8(UPDATES_DISABLED_011E0E34) == 0;
    if !eligible {
        if e.call(0x0044_0da0, &args![actor]).bool() {
            e.call(0x0096_e870, &args![this, actor]);
        }
        return;
    }
    let base = this.addr();
    let mut character = 0u32;
    let mut handle_followers = true;
    if actor != 0 && e.vcall(actor, 0x100, &args![]).bool() {
        character = actor;
        let count = e.mem.u32(base.wrapping_add(0x103b0));
        e.mem
            .set_u32(base.wrapping_add(0x103b0), count.wrapping_add(1));
    }
    if character != 0 && e.call(0x008b_0520, &args![character]).bool() {
        let process = e.call(ACTOR_PROCESS, &args![character]).u32();
        e.vcall(process, 0x24, &args![character, 0u32]);
        handle_followers = false;
        let process = e.call(ACTOR_PROCESS, &args![character]).u32();
        e.call(0x008c_8fd0, &args![process, 1u32]);
    } else {
        e.vcall(actor, 0x25c, &args![x87_quiet_bits(time.to_bits())]);
    }
    if character != 0 {
        let player = e.mem.u32(PLAYER);
        if e.call(0x0094_df60, &args![player]).bool() {
            let amount = x87_quiet_bits(e.mem.u32(0x0101_62c0));
            e.call(0x0088_b510, &args![character, amount]);
            let amount = x87_quiet_bits(e.mem.u32(0x0101_62c0));
            e.call(0x0088_b5a0, &args![character, amount]);
        }
    }
    if handle_followers {
        level_0_followers(e, actor, time);
    }
}

/// The follower part of `0096bcd0` for `actor`.
fn level_0_followers(e: &mut Engine, actor: u32, time: f32) {
    let followers = if e.call(0x005d_43c0, &args![actor]).u32() != 0 {
        let extra = e.call(0x005d_43c0, &args![actor]).u32();
        e.call(0x0042_2700, &args![extra]).u32()
    } else {
        0
    };
    if followers == 0 {
        return;
    }
    let removed = new_actor_list(e);
    let copy = new_actor_list(e);
    let mut node = e.mem.u32(followers.wrapping_add(0xc));
    while node_has_actor(e, node) {
        let element = e.call(0x0068_15c0, &args![node]).u32();
        e.call(0x0090_5820, &args![copy, element]);
        node = next_node(e, node);
    }
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let mut node = copy;
        while node_has_actor(e, node) {
            let follower = node_actor(e, node);
            e.mem.set_u32(slot, follower);
            if follower != 0
                && follower != e.mem.u32(PLAYER)
                && e.call(ACTOR_PROCESS, &args![follower]).u32() != 0
            {
                let mut package = e.call(0x0088_1510, &args![follower]).u32();
                let extra = e.call(0x005d_43c0, &args![follower]).u32();
                let package_extra = e.call(0x0041_cb10, &args![extra]).u32();
                if package_extra != 0 {
                    package = package_extra;
                }
                let follows = package != 0
                    && (e.call(0x0041_ca90, &args![package]).u32() == 1
                        || e.call(0x0041_ca90, &args![package]).u32() == 7);
                if !follows {
                    collect_actor(e, removed, slot);
                } else if e.call(0x0093_1850, &args![follower]).u32() != 0 {
                    // The exe also computes `time`, or the `float` at `01084838` when `time` is
                    // not above 0, into a local it never reads.
                    let mut unused = x87_quiet_bits(time.to_bits());
                    if !parity_even(
                        fpu_compare(x87_load32(unused), double_at(e, ZERO_01012060)) & 0x41,
                    ) {
                        unused = x87_quiet_bits(e.mem.u32(0x0108_4838));
                    }
                    let _ = unused;
                    let hour = e.call(0x0086_7da0, &args![CALENDAR_011DE7B8]).f64();
                    let earlier = x87_store32(x87_sub(hour, double_at(e, ONE_01012070)));
                    let process = e.call(ACTOR_PROCESS, &args![follower]).u32();
                    e.call(0x0069_3d50, &args![process, earlier]);
                }
            }
            node = next_node(e, node);
        }
    });
    let mut node = removed;
    while node_has_actor(e, node) {
        let follower = node_actor(e, node);
        let extra = e.call(0x005d_43c0, &args![actor]).u32();
        e.call(0x0042_2690, &args![extra, follower]);
        node = next_node(e, node);
    }
    free_actor_list(e, removed);
    free_actor_list(e, copy);
    let list = e.mem.u32(followers.wrapping_add(0xc));
    if e.call(0x0082_56d0, &args![list]).bool() {
        let extra = e.call(0x005d_43c0, &args![actor]).u32();
        e.call(0x0042_2720, &args![extra]);
    }
}

// Translated from 0096c240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`, the word is not read): for each actor of process level
/// 0 (the end asked again each step) that is not null and for which `00576d30`, `00440d80` and
/// `00440da0` are all false: when its vtable slot `0x224` is true it gets slot `0x314`;
/// otherwise when slot `0x220` is true, `009ae6e0` runs on it.
pub fn fn_0096c240(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let array = this.addr().wrapping_add(4);
    let mut index = e.call(LEVEL_BEGIN, &args![array, 0u32]).u32();
    while index < e.call(LEVEL_END, &args![array, 0u32]).u32() {
        let actor = e.call(ACTOR_AT, &args![array, index]).u32();
        if actor != 0
            && !e.call(0x0057_6d30, &args![actor]).bool()
            && !e.call(0x0044_0d80, &args![actor]).bool()
            && !e.call(0x0044_0da0, &args![actor]).bool()
        {
            if e.vcall(actor, 0x224, &args![]).bool() {
                e.vcall(actor, 0x314, &args![]);
            } else if e.vcall(actor, 0x220, &args![]).bool() {
                e.call(0x009a_e6e0, &args![actor]);
            }
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 0096c330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the player's perception pass over the actors of process level 0.
/// The spread `0040ebd0(n * 0084d030(011f6394), 5.0)` (`n` the level's end index, the product
/// stored as a `float`, 5.0 the `float` at `0101712c`) is computed first; with no actor
/// `008c8bb0(player, 0)` runs. For each character (vtable slot `0x100`) with the byte at
/// `+0xbc` set and a vtable slot `0x1d0` object, its distance to the player
/// (`TESObjectREFR::GetDistanceFromReference(player, 0, 0)`) is read; the player itself and the
/// actors for which `0047c850(011de45c)` is true are skipped. The character's vtable slot
/// `0x2b4` value below 0 (and neither `00437bd0` nor slot `0x22c` with 0 true) starts a check
/// when the fader alpha (`FaderManager::GetFaderAlpha(011d8804, 1)`) is not above 0: slot
/// `0x2b0` gets `RandomFloat(0, spread)`; unless the vtable slot `0x20` of `004181e0(actor) +
/// 0x30` is true and when the process's slot `0x70` accepts the actor, the detection level
/// (`Actor::GetDetectionLevelAgainstActor`, `008a0d10`) and hostility (`GetShouldAttackActor`,
/// `IsInCombatWithActor`, or a reaction of 0/1 with `00705cf0(004181e0(actor) + 0x90)` and not
/// `ShouldSkipFallOutBehavior(6)`) decide: a close actor (distance below the setting
/// `011ccf94`) is inserted near the player (`0096e3c0(011e0e80, actor)`); a level above the
/// setting `011d13c8` reports it to the player (`00966f20(player, actor, hostile, 0)`), else a
/// noticed one (`00966f20(.., 1)`) or the perceived actor is removed
/// (`PlayerCharacter::RemovePerceivedActor`). Any other value removes a negative one and then
/// runs the character's slot `0x2b8`. Before the test, the first index also runs
/// `008c8bb0(player, 0)`.
pub fn fn_0096c330(e: &mut Engine, this: Ptr) {
    let array = this.addr().wrapping_add(4);
    let count = e.call(LEVEL_END, &args![array, 0u32]).u32();
    let scale = e.call(0x0084_d030, &args![0x011f_6394u32]).f64();
    let spread = x87_store32(x87_mul(scale, f64::from(count)));
    let cap = x87_quiet_bits(e.mem.u32(0x0101_712c));
    let spread = x87_store32(
        e.call(0x0040_ebd0, &args![x87_quiet_bits(spread), cap])
            .f64(),
    );
    if count == 0 {
        let player = e.mem.u32(PLAYER);
        e.call(0x008c_8bb0, &args![player, 0u32]);
    }
    let mut index = e.call(LEVEL_BEGIN, &args![array, 0u32]).u32();
    while index < e.call(LEVEL_END, &args![array, 0u32]).u32() {
        let actor = e.call(ACTOR_AT, &args![array, index]).u32();
        if actor != 0 && e.vcall(actor, 0x100, &args![]).bool() {
            perceive_actor(e, actor, index, spread);
        }
        index = index.wrapping_add(1);
    }
}

/// One character of `0096c330`'s loop.
fn perceive_actor(e: &mut Engine, actor: u32, index: u32, spread: u32) {
    if e.mem.u8(actor.wrapping_add(0xbc)) == 0 {
        return;
    }
    if e.vcall(actor, 0x1d0, &args![]).u32() == 0 {
        return;
    }
    let player = e.mem.u32(PLAYER);
    let distance = stored_float(e.call(0x0057_23b0, &args![actor, player, 0u32, 0u32]).f64());
    if actor == e.mem.u32(PLAYER) {
        return;
    }
    let base_data = e.mem.u32(0x011d_e45c);
    if e.call(0x0047_c850, &args![base_data]).bool() {
        return;
    }
    let value = stored_float(e.vcall(actor, 0x2b4, &args![]).f64());
    if index == 0 {
        let player = e.mem.u32(PLAYER);
        e.call(0x008c_8bb0, &args![player, 0u32]);
    }
    let zero = double_at(e, ZERO_01012060);
    let negative = !parity_even(fpu_compare(value, zero) & 0x05);
    if negative
        && !e.call(0x0043_7bd0, &args![actor]).bool()
        && !e.vcall(actor, 0x22c, &args![0u32]).bool()
    {
        let fader = e.mem.u32(0x011d_8804);
        let alpha = e.call(0x0070_14e0, &args![fader, 1u32]).f64();
        if parity_even(fpu_compare(alpha, zero) & 0x41) {
            return;
        }
        let random = x87_store32(
            e.call(0x0047_6b70, &args![0.0f32, x87_quiet_bits(spread)])
                .f64(),
        );
        e.vcall(actor, 0x2b0, &args![x87_quiet_bits(random)]);
        let data = e.call(0x0041_81e0, &args![actor]).u32().wrapping_add(0x30);
        if e.vcall(data, 0x20, &args![]).bool() {
            return;
        }
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        if !e.vcall(process, 0x70, &args![actor]).bool() {
            return;
        }
        perceive_detect(e, actor, distance);
        return;
    }
    if negative {
        let player = e.mem.u32(PLAYER);
        e.call(0x0096_7350, &args![player, actor]);
    }
    e.vcall(actor, 0x2b8, &args![]);
}

/// The detection part of `0096c330` for a character that passed the checks.
fn perceive_detect(e: &mut Engine, actor: u32, distance: f64) {
    e.with_stack(8, |e, locals| {
        let base = locals.addr();
        let noticed = base;
        let unseen = base + 1;
        let reaction = base + 4;
        e.mem.set_u8(noticed, 0);
        let crouching = e.call(0x0049_3bb0, &args![actor]).u8();
        e.mem.set_u8(unseen, 0);
        let player = e.mem.u32(PLAYER);
        let level = e
            .call(
                0x008a_0d10,
                &args![
                    player,
                    1u32,
                    actor,
                    noticed,
                    u32::from(crouching),
                    0u32,
                    0u32,
                    unseen
                ],
            )
            .i32();
        e.mem.set_u32(reaction, 0);
        let player = e.mem.u32(PLAYER);
        let mut hostile = e
            .call(0x008b_06d0, &args![actor, player, 0u32, reaction, 0u32])
            .bool();
        if !hostile {
            let player = e.mem.u32(PLAYER);
            hostile = e.call(0x008b_c700, &args![actor, player]).bool();
        }
        if !hostile {
            let kind = e.mem.u32(reaction);
            if kind == 0 || kind == 1 {
                let data = e.call(0x0041_81e0, &args![actor]).u32().wrapping_add(0x90);
                if e.call(0x0070_5cf0, &args![data]).bool()
                    && !e.call(0x008a_78f0, &args![actor, 6u32]).bool()
                {
                    hostile = true;
                }
            }
        }
        if actor != 0 {
            let setting = e.call(0x0040_3e20, &args![0x011c_cf94u32]).u32();
            let close = x87_load32(e.mem.u32(setting));
            if fpu_compare(close, distance) & 0x41 == 0 {
                e.call(0x0096_e3c0, &args![0x011e_0e80u32, actor]);
            }
        }
        let setting = e.call(0x0040_3e20, &args![0x011d_13c8u32]).u32();
        let threshold = x87_load32(e.mem.u32(setting));
        let player = e.mem.u32(PLAYER);
        if !parity_even(fpu_compare(threshold, f64::from(level)) & 0x05) {
            e.call(0x0096_6f20, &args![player, actor, u32::from(hostile), 0u32]);
        } else if e.mem.u8(unseen) != 0 || e.mem.u8(noticed) != 0 {
            e.call(0x0096_6f20, &args![player, actor, u32::from(hostile), 1u32]);
        } else {
            e.call(0x0096_7350, &args![player, actor]);
        }
    });
}

// Translated from 0096c710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `008f63e0(process, actor)` for the player (when it has a process
/// object, `008d8520`) and then for every character (vtable slot `0x100`) of process level 0.
pub fn fn_0096c710(e: &mut Engine, this: Ptr) {
    let player = e.mem.u32(PLAYER);
    if e.call(ACTOR_PROCESS, &args![player]).u32() != 0 {
        let player = e.mem.u32(PLAYER);
        let process = e.call(ACTOR_PROCESS, &args![player]).u32();
        e.call(0x008f_63e0, &args![process, player]);
    }
    let array = this.addr().wrapping_add(4);
    let mut index = e.call(LEVEL_BEGIN, &args![array, 0u32]).u32();
    while index < e.call(LEVEL_END, &args![array, 0u32]).u32() {
        let actor = e.call(ACTOR_AT, &args![array, index]).u32();
        if actor != 0 && e.vcall(actor, 0x100, &args![]).bool() {
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            e.call(0x008f_63e0, &args![process, actor]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 0096c7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: every character (vtable slot `0x100`) of process level 0 with
/// the byte at `+0xbc` set gets its vtable slot `0x2f8` with 0.0.
pub fn fn_0096c7c0(e: &mut Engine, this: Ptr) {
    let array = this.addr().wrapping_add(4);
    let mut index = e.call(LEVEL_BEGIN, &args![array, 0u32]).u32();
    while index < e.call(LEVEL_END, &args![array, 0u32]).u32() {
        let actor = e.call(ACTOR_AT, &args![array, index]).u32();
        if actor != 0
            && e.vcall(actor, 0x100, &args![]).bool()
            && e.mem.u8(actor.wrapping_add(0xbc)) != 0
        {
            e.vcall(actor, 0x2f8, &args![0.0f32]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 0096af60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pointer array's scalar deleting destructor: `fn_0096ad00`, then
/// `operator delete(this)` (`00401030`) when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_0096af60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0096ad00(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0096af90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor around `0096a930` (`run_0096a850.rs`):
/// that body, then `operator delete(this)` (`00401030`) when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_0096af90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0096_a930, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0096bc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counts the actors of process level 0 (the list object at +4, from
/// `005de0f0(0)` to `005be5c0(0)`, each from `00968670`) whose slot 0x100
/// answers true; empty slots are skipped.
pub fn fn_0096bc50(e: &mut Engine, this: Ptr) -> u32 {
    let list = this.addr() + 4;
    let mut count = 0u32;
    let mut index = e.call(LEVEL_BEGIN, &args![list, 0u32]).u32();
    while index < e.call(LEVEL_END, &args![list, 0u32]).u32() {
        let actor = e.call(ACTOR_AT, &args![list, index]).u32();
        if actor != 0 && e.vcall(actor, 0x100, &[]).bool() {
            count = count.wrapping_add(1);
        }
        index += 1;
    }
    count
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0096_af60, fn_0096af60(Ptr, u32) -> Ptr),
        entry!(0x0096_af90, fn_0096af90(Ptr, u32) -> Ptr),
        entry!(0x0096_bc50, fn_0096bc50(Ptr) -> u32),
        entry!(0x0096_ad30, fn_0096ad30(Ptr, u32)),
        entry!(0x0096_ae90, fn_0096ae90(Ptr, u32, Ptr)),
        entry!(0x0096_af30, fn_0096af30(Ptr, u32, u32) -> Ptr),
        entry!(0x0096_afe0, fn_0096afe0(Ptr, u32, u32) -> Ptr),
        entry!(0x0096_ad00, fn_0096ad00(Ptr)),
        entry!(0x0096_b050, fn_0096b050(Ptr, f32, u32)),
        entry!(0x0096_b470, fn_0096b470(Ptr, f32, u32)),
        entry!(0x0096_b810, fn_0096b810(Ptr, f32, u32)),
        entry!(0x0096_bbc0, fn_0096bbc0(Ptr, u32)),
        entry!(0x0096_bcd0, fn_0096bcd0(Ptr, f32, u32)),
        entry!(0x0096_c240, fn_0096c240(Ptr, u32)),
        entry!(0x0096_c330, fn_0096c330(Ptr)),
        entry!(0x0096_c710, fn_0096c710(Ptr)),
        entry!(0x0096_c7c0, fn_0096c7c0(Ptr)),
    ]
}

#[cfg(test)]
mod movement_path_tests {
    use super::*;

    /// An array object with `capacity` slots holding `elements`, `count` of them in use.
    fn array(e: &mut Engine, capacity: u16, elements: &[u32]) -> (u32, u32) {
        let this = e.mem.alloc(0x10);
        let block = e.mem.alloc(u32::from(capacity.max(1)) * 4);
        for (index, value) in elements.iter().enumerate() {
            e.mem.set_u32(block + index as u32 * 4, *value);
        }
        e.mem.set_u32(this + 4, block);
        e.mem.set_u16(this + 8, capacity);
        e.mem.set_u16(this + 0xa, elements.len() as u16);
        let used = elements.iter().filter(|value| **value != 0).count();
        e.mem.set_u16(this + 0xc, used as u16);
        (this, block)
    }

    fn doubles(e: &mut Engine) {
        e.register_double(0x0096_afc0, |e, _| {
            let block = e.mem.alloc(0x40);
            e.mem.write(block, &[0xcd; 0x40]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(0x004e_de70, |_, _| Ret::default());
    }

    #[test]
    fn resize_to_the_same_capacity_does_nothing() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (this, block) = array(&mut e, 4, &[1, 2]);
        e.call_log = Some(vec![]);
        e.call(0x0096_ad30, &[this, 4]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this + 4), block);
    }

    #[test]
    fn growing_copies_the_elements_and_clears_the_rest() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (this, block) = array(&mut e, 2, &[7, 0]);
        e.call_log = Some(vec![]);
        e.call(0x0096_ad30, &[this, 4]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0096_afc0, vec![4]));
        assert_eq!(log[2], (0x004e_de70, vec![block]));
        let new = e.mem.u32(this + 4);
        assert_ne!(new, block);
        assert_eq!(e.mem.u16(this + 8), 4);
        assert_eq!(e.mem.u16(this + 0xa), 2);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        let words: Vec<u32> = (0..4).map(|index| e.mem.u32(new + index * 4)).collect();
        assert_eq!(words, [7, 0, 0, 0]);
    }

    #[test]
    fn shrinking_drops_the_elements_past_the_new_capacity() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (this, block) = array(&mut e, 4, &[1, 0, 3, 4]);
        e.call(0x0096_ad30, &[this, 2]);
        assert_eq!(e.mem.u16(this + 8), 2);
        assert_eq!(e.mem.u16(this + 0xa), 2);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!(e.mem.u32(block + 12), 0);
        let new = e.mem.u32(this + 4);
        assert_eq!((e.mem.u32(new), e.mem.u32(new + 4)), (1, 0));
    }

    #[test]
    fn resizing_to_zero_frees_the_block() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (this, block) = array(&mut e, 2, &[5, 6]);
        e.call_log = Some(vec![]);
        e.call(0x0096_ad30, &[this, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x004e_de70, vec![block])]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 8), 0);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
    }

    #[test]
    fn setting_an_element_keeps_the_non_null_count() {
        let mut e = Engine::new();
        let (this, block) = array(&mut e, 4, &[0, 9]);
        let value = e.mem.alloc(4);
        // Replacing a null with a pointer.
        e.mem.set_u32(value, 0x1234);
        e.call(0x0096_ae90, &[this, 0, value]);
        assert_eq!((e.mem.u32(block), e.mem.u16(this + 0xc)), (0x1234, 2));
        // Clearing a pointer.
        e.mem.set_u32(value, 0);
        e.call(0x0096_ae90, &[this, 1, value]);
        assert_eq!((e.mem.u32(block + 4), e.mem.u16(this + 0xc)), (0, 1));
        // Clearing a null changes nothing.
        e.call(0x0096_ae90, &[this, 1, value]);
        assert_eq!(e.mem.u16(this + 0xc), 1);
        // Replacing a pointer with another.
        e.mem.set_u32(value, 0x77);
        e.call(0x0096_ae90, &[this, 0, value]);
        assert_eq!((e.mem.u32(block), e.mem.u16(this + 0xc)), (0x77, 1));
    }

    #[test]
    fn setting_past_the_count_extends_it() {
        let mut e = Engine::new();
        let (this, block) = array(&mut e, 4, &[3]);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0x55);
        e.call(0x0096_ae90, &[this, 2, value]);
        assert_eq!(e.mem.u16(this + 0xa), 3);
        assert_eq!(e.mem.u16(this + 0xc), 2);
        assert_eq!(e.mem.u32(block + 8), 0x55);
        e.mem.set_u32(value, 0);
        e.call(0x0096_ae90, &[this, 3, value]);
        assert_eq!(e.mem.u16(this + 0xa), 4);
        assert_eq!(e.mem.u16(this + 0xc), 2);
    }
}

#[cfg(test)]
mod process_list_tests {
    use super::*;

    fn block_double(e: &mut Engine) {
        e.register(0x0096_afc0, |_, _| Ret {
            eax: 0x0009_0000,
            ..Ret::default()
        });
    }

    #[test]
    fn constructor_takes_the_element_block_for_a_positive_capacity() {
        let mut e = Engine::new();
        block_double(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xee; 0x10]);
        e.call_log = Some(vec![]);
        // The high halves of the pushed words are not read.
        let result = e.call(0x0096_afe0, &[this, 0xabcd_0007, 0x1234_0001]).u32();
        assert_eq!(result, this);
        let log = e.call_log.take().unwrap();
        let allocations: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0096_afc0).collect();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0].1, vec![7]);
        assert_eq!(e.mem.u32(this), 0x0108_c0c8);
        assert_eq!(e.mem.u32(this + 4), 0x0009_0000);
        assert_eq!(e.mem.u16(this + 8), 7);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(e.mem.u16(this + 0xe), 1);
    }

    #[test]
    fn constructor_without_capacity_allocates_nothing() {
        let mut e = Engine::new();
        block_double(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xee; 0x10]);
        e.call_log = Some(vec![]);
        e.call(0x0096_afe0, &[this, 0, 1]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != 0x0096_afc0));
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 0xe), 1);
    }

    #[test]
    fn subclass_constructor_replaces_the_vtable_and_returns_this() {
        let mut e = Engine::new();
        block_double(&mut e);
        let this = e.mem.alloc(0x10);
        let result = e.call(0x0096_af30, &[this, 0, 1]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), 0x0108_c0d0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 8), 0);
        assert_eq!(e.mem.u16(this + 0xe), 1);
    }
}

#[cfg(test)]
mod b035_tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// Vtable slot `slot` of every test object calls `SLOT_BASE + slot`.
    const SLOT_BASE: u32 = 0x7100_0000;

    type Answers = Rc<RefCell<HashMap<(u32, u32), Ret>>>;

    /// A world of test objects: every object shares one vtable whose slots call doubles that
    /// answer from `answers` by (object, slot), 0 by default.
    struct World {
        vtable: u32,
        answers: Answers,
    }

    impl World {
        fn new(e: &mut Engine, slots: &[u32]) -> World {
            let vtable = e.mem.alloc(0x400);
            for slot in (0..0x400).step_by(4) {
                e.mem.set_u32(vtable + slot, SLOT_BASE + slot);
            }
            let answers: Answers = Default::default();
            for slot in slots {
                let slot = *slot;
                let table = answers.clone();
                e.register_double(SLOT_BASE + slot, move |_, a| {
                    table
                        .borrow()
                        .get(&(a[0], slot))
                        .copied()
                        .unwrap_or_default()
                });
            }
            World { vtable, answers }
        }

        fn object(&self, e: &mut Engine) -> u32 {
            let object = e.mem.alloc(0x100);
            e.mem.set_u32(object, self.vtable);
            // The object at +0x80 stands for its process.
            e.mem.set_u32(object + 0x80, self.vtable);
            object
        }

        fn answer(&self, object: u32, slot: u32, value: u32) {
            self.answers.borrow_mut().insert(
                (object, slot),
                Ret {
                    eax: value,
                    ..Ret::default()
                },
            );
        }

        fn answer_float(&self, object: u32, slot: u32, value: f64) {
            self.answers.borrow_mut().insert(
                (object, slot),
                Ret {
                    st0: value,
                    ..Ret::default()
                },
            );
        }
    }

    type Flags = Rc<RefCell<HashMap<(u32, u32), u32>>>;

    /// Doubles the functions in `addresses`: each answers the value set for (address, first
    /// word), 0 by default.
    fn by_first_word(e: &mut Engine, addresses: &[u32]) -> Flags {
        let flags: Flags = Default::default();
        for address in addresses {
            let address = *address;
            let table = flags.clone();
            e.register_double(address, move |_, a| {
                let first = a.first().copied().unwrap_or(0);
                table
                    .borrow()
                    .get(&(address, first))
                    .copied()
                    .unwrap_or(0)
                    .into_ret()
            });
        }
        flags
    }

    /// The process array: level `level` holds `actors` (indices `10..10 + n`).
    fn levels(e: &mut Engine, actors: Vec<u32>) {
        let count = actors.len() as u32;
        e.register(LEVEL_BEGIN, |_, _| 10u32.into_ret());
        e.register_double(LEVEL_END, move |_, _| (10 + count).into_ret());
        e.register_double(ACTOR_AT, move |_, a| {
            actors[(a[1] - 10) as usize].into_ret()
        });
    }

    fn globals(e: &mut Engine) {
        for page in [
            0x011d_e000u32,
            0x011e_0000,
            0x0101_2000,
            0x0101_a000,
            0x0102_9000,
            0x0107_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_u64(ZERO_01012060, 0f64.to_bits());
        e.mem.set_u64(ONE_01012070, 1f64.to_bits());
        e.mem.set_u64(MINUS_ONE_0101A6B0, (-1f64).to_bits());
        e.mem.set_u64(0x0102_90d0, f64::from(0.3f32).to_bits());
        e.mem.set_u64(0x0107_0c80, f64::from(0.15f32).to_bits());
    }

    #[test]
    fn fn_0096ad00_stores_the_vtable_and_frees_the_block() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x004e_de70]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0096_ad00, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), 0x0108_c0c8);
        assert_eq!(calls(&log, 0x004e_de70), vec![vec![0x5000]]);
    }

    #[test]
    fn fn_0096bbc0_visits_the_live_actors_of_the_visitor_level() {
        let mut e = Engine::new();
        let world = World::new(&mut e, &[0]);
        let actors = vec![world.object(&mut e), 0, world.object(&mut e)];
        levels(&mut e, actors.clone());
        let visitor = world.object(&mut e);
        e.register(0x0072_6070, |_, _| 3u32.into_ret());
        let flags = by_first_word(&mut e, &[0x0057_6d30]);
        flags.borrow_mut().insert((0x0057_6d30, actors[2]), 1);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0096_bbc0, &args![this, visitor]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, LEVEL_END), vec![vec![this + 4, 3]]);
        assert_eq!(calls(&log, LEVEL_BEGIN), vec![vec![this + 4, 3]]);
        assert_eq!(calls(&log, SLOT_BASE), vec![vec![visitor, actors[0]]]);
    }

    #[test]
    fn fn_0096c240_runs_the_slot_or_the_explosion_step() {
        let mut e = Engine::new();
        let world = World::new(&mut e, &[0x224, 0x220, 0x314]);
        let actors: Vec<u32> = (0..3).map(|_| world.object(&mut e)).collect();
        levels(&mut e, actors.clone());
        let flags = by_first_word(&mut e, &[0x0057_6d30, 0x0044_0d80, 0x0044_0da0]);
        flags.borrow_mut().insert((0x0044_0d80, actors[2]), 1);
        world.answer(actors[0], 0x224, 1);
        world.answer(actors[1], 0x220, 1);
        world.answer(actors[2], 0x224, 1);
        quiet(&mut e, &[0x009a_e6e0]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0096_c240, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SLOT_BASE + 0x314), vec![vec![actors[0]]]);
        assert_eq!(calls(&log, 0x009a_e6e0), vec![vec![actors[1]]]);
    }

    #[test]
    fn fn_0096c710_updates_the_player_and_the_characters() {
        let mut e = Engine::new();
        globals(&mut e);
        let world = World::new(&mut e, &[0x100]);
        let actors: Vec<u32> = (0..2).map(|_| world.object(&mut e)).collect();
        levels(&mut e, actors.clone());
        world.answer(actors[1], 0x100, 1);
        e.mem.set_u32(PLAYER, 0x900);
        e.register(ACTOR_PROCESS, |_, a| (a[0] + 0x80).into_ret());
        quiet(&mut e, &[0x008f_63e0]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0096_c710, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, 0x008f_63e0),
            vec![vec![0x980, 0x900], vec![actors[1] + 0x80, actors[1]]]
        );
    }

    #[test]
    fn fn_0096c7c0_resets_the_flagged_characters() {
        let mut e = Engine::new();
        let world = World::new(&mut e, &[0x100, 0x2f8]);
        let actors: Vec<u32> = (0..3).map(|_| world.object(&mut e)).collect();
        levels(&mut e, actors.clone());
        for actor in &actors {
            world.answer(*actor, 0x100, 1);
        }
        e.mem.set_u8(actors[0] + 0xbc, 1);
        e.mem.set_u8(actors[2] + 0xbc, 1);
        world.answer(actors[2], 0x100, 0);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0096_c7c0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SLOT_BASE + 0x2f8), vec![vec![actors[0], 0]]);
    }

    /// Doubles for the removal tail and the list the level updates collect actors in: the
    /// list's nodes are the words of a block, `006815c0` gives the node itself.
    fn removal_doubles(e: &mut Engine) -> u32 {
        let list = e.mem.alloc(0x40);
        e.register_double(0x0040_1000, move |_, _| list.into_ret());
        e.register(0x0096_a2d0, |_, a| a[0].into_ret());
        e.register(0x005a_e3d0, |e, a| {
            // Appends at the first empty word.
            let mut at = a[0];
            while e.mem.u32(at) != 0 {
                at += 4;
            }
            let actor = e.mem.u32(a[1]);
            e.mem.set_u32(at, actor);
            Ret::default()
        });
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0072_6070, |_, a| (a[0] + 4).into_ret());
        quiet(
            e,
            &[
                0x0096_aaf0,
                0x0047_0470,
                0x0047_02f0,
                0x0040_fbf0,
                0x0040_fba0,
                0x0040_7800,
            ],
        );
        list
    }

    fn time_doubles(e: &mut Engine, hour: f64) {
        e.register_double(0x0086_7da0, move |_, _| Ret {
            st0: hour,
            ..Ret::default()
        });
    }

    #[test]
    fn fn_0096b810_updates_level_1_and_removes_the_others() {
        let mut e = Engine::new();
        globals(&mut e);
        let world = World::new(&mut e, &[0x100, 0x1ec, 0x25c, 0x2ec, 0x10]);
        let actors: Vec<u32> = (0..4).map(|_| world.object(&mut e)).collect();
        levels(&mut e, actors.clone());
        let list = removal_doubles(&mut e);
        time_doubles(&mut e, 10.0);
        let flags = by_first_word(
            &mut e,
            &[0x0057_6d30, 0x0044_0da0, 0x0044_0d80, 0x0093_1850],
        );
        for actor in &actors[..3] {
            flags.borrow_mut().insert((0x0093_1850, *actor), 1);
        }
        // Actor 3 is of process type 2: collected, its slot 0x2ec says true.
        flags.borrow_mut().insert((0x0093_1850, actors[3]), 2);
        world.answer(actors[3], 0x2ec, 1);
        // The process of an actor is the actor plus 1; its last hour by process.
        e.register(ACTOR_PROCESS, |_, a| (a[0] + 0x80).into_ret());
        let last: HashMap<u32, f64> = [
            (actors[0] + 0x80, -1.0),
            (actors[1] + 0x80, 9.95),
            (actors[2] + 0x80, 9.5),
        ]
        .into_iter()
        .collect();
        e.register_double(0x007d_f1f0, move |_, a| Ret {
            st0: last[&a[0]],
            ..Ret::default()
        });
        for process in [actors[1] + 0x80, actors[2] + 0x80] {
            world.answer(process, 0x1ec, 1);
        }
        quiet(&mut e, &[0x0092_f190, 0x005a_c740]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0096_b810, &args![this, 0.0f32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0092_f190), vec![vec![actors[0]]]);
        // Only actor 2 is due (0.5 hour passed, 0.05 is not enough).
        assert_eq!(calls(&log, SLOT_BASE + 0x25c), vec![vec![actors[2], 0]]);
        assert_eq!(calls(&log, 0x0096_aaf0), vec![vec![this + 4, actors[3], 1]]);
        assert_eq!(calls(&log, SLOT_BASE + 0x10), vec![vec![actors[3], 1]]);
        assert_eq!(e.mem.u32(list), actors[3]);
        assert_eq!(calls(&log, 0x0047_02f0), vec![vec![list, 1]]);
        // A time step makes every one due.
        e.call_log = Some(vec![]);
        e.mem.set_u32(list, 0);
        e.call(0x0096_b810, &args![this, 0.25f32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SLOT_BASE + 0x25c).len(), 2);
    }

    #[test]
    fn fn_0096b470_hands_resting_characters_to_their_process() {
        let mut e = Engine::new();
        globals(&mut e);
        let world = World::new(&mut e, &[0x100, 0x1ec, 0x25c, 0x28]);
        let actors: Vec<u32> = (0..3).map(|_| world.object(&mut e)).collect();
        levels(&mut e, actors.clone());
        removal_doubles(&mut e);
        time_doubles(&mut e, 5.0);
        let flags = by_first_word(
            &mut e,
            &[
                0x0057_6d30,
                0x0044_0da0,
                0x0044_0d80,
                0x0093_1850,
                0x0043_7bb0,
                0x0049_3bb0,
            ],
        );
        for actor in &actors {
            flags.borrow_mut().insert((0x0093_1850, *actor), 2);
        }
        // Actor 0: a character without 00437bb0 and 00493bb0: slot 0x28 of its process.
        world.answer(actors[0], 0x100, 1);
        // Actor 1: 00437bb0 true, its hour test is due (last update at 4.0).
        flags.borrow_mut().insert((0x0043_7bb0, actors[1]), 1);
        // Actor 2: removed (00440da0 true) and loses its process.
        flags.borrow_mut().insert((0x0044_0da0, actors[2]), 1);
        e.register(ACTOR_PROCESS, |_, a| (a[0] + 0x80).into_ret());
        e.register(0x007d_f1f0, |_, _| Ret {
            st0: 4.0,
            ..Ret::default()
        });
        world.answer(actors[1] + 0x80, 0x1ec, 1);
        world.answer(actors[2] + 0x80, 0, 0);
        quiet(&mut e, &[0x005a_c740, SLOT_BASE]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0096_b470, &args![this, 0.0f32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SLOT_BASE + 0x28), vec![vec![actors[0] + 0x80]]);
        assert_eq!(calls(&log, SLOT_BASE + 0x25c), vec![vec![actors[1], 0]]);
        assert_eq!(calls(&log, 0x0096_aaf0), vec![vec![this + 4, actors[2], 2]]);
        assert_eq!(calls(&log, SLOT_BASE), vec![vec![actors[2] + 0x80, 1]]);
        assert_eq!(calls(&log, 0x0040_7800), vec![vec![actors[2], 0]]);
    }

    #[test]
    fn fn_0096b050_resumes_at_the_saved_index_and_saves_the_next() {
        let mut e = Engine::new();
        globals(&mut e);
        let world = World::new(&mut e, &[0x100, 0x1ec, 0x25c]);
        let actors: Vec<u32> = (0..3).map(|_| world.object(&mut e)).collect();
        levels(&mut e, actors.clone());
        removal_doubles(&mut e);
        time_doubles(&mut e, 5.0);
        // The counter reads 100, except the third read (the first check of the second run),
        // 200: past that run's deadline 100 + 50.
        let reads = Rc::new(RefCell::new(0u32));
        let counter = reads.clone();
        e.register_double(0x00fd_f0a0, move |e, a| {
            let value = if *counter.borrow() == 2 { 200u64 } else { 100 };
            *counter.borrow_mut() += 1;
            e.mem.set_u64(a[0], value);
            1u32.into_ret()
        });
        let budget = e.mem.alloc(4);
        e.mem.set_u32(budget, 50);
        e.register_double(0x0043_d4d0, move |_, _| budget.into_ret());
        e.mem.set_u32(0x011e_0e20, 1000);
        e.register(0x00ec_c300, |_, a| {
            (u64::from(a[0]) * u64::from(a[2])).into_ret()
        });
        e.register(0x00ec_c250, |_, a| {
            ((u64::from(a[0]) | u64::from(a[1]) << 32) / u64::from(a[2])).into_ret()
        });
        let flags = by_first_word(
            &mut e,
            &[
                0x0057_6d30,
                0x0044_0da0,
                0x0044_0d80,
                0x0045_cd60,
                0x0043_7bb0,
            ],
        );
        e.register(ACTOR_PROCESS, |_, a| (a[0] + 0x80).into_ret());
        for actor in &actors {
            flags.borrow_mut().insert((0x0045_cd60, actor + 0x80), 3);
            flags.borrow_mut().insert((0x0043_7bb0, *actor), 1);
            world.answer(actor + 0x80, 0x1ec, 1);
        }
        e.register(0x007d_f1f0, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        quiet(&mut e, &[0x005a_c740, 0x0090_7650]);
        // Start at the saved index 11 (inside the level).
        e.mem.set_u32(0x011e_0e30, 11);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0096_b050, &args![this, 0.0f32, 0u32]);
        let log = e.call_log.take().unwrap();
        // Index 11 runs; index 12 is not a multiple of 5 and runs too; then the level ends.
        assert_eq!(
            calls(&log, SLOT_BASE + 0x25c),
            vec![vec![actors[1], 0], vec![actors[2], 0]]
        );
        assert_eq!(e.mem.u32(0x011e_0e30), 13);
        // At the end the saved index restarts at the level's first index; index 10 checks the
        // clock (200 is past 150) and the loop stops after it.
        e.call_log = Some(vec![]);
        e.call(0x0096_b050, &args![this, 0.0f32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SLOT_BASE + 0x25c), vec![vec![actors[0], 0]]);
        assert_eq!(e.mem.u32(0x011e_0e30), 11);
        // With `all` the clock is not read.
        e.mem.set_u32(0x011e_0e30, 0);
        let before = *reads.borrow();
        e.call(0x0096_b050, &args![this, 0.0f32, 1u32]);
        assert_eq!(*reads.borrow(), before + 1);
        assert_eq!(e.mem.u32(0x011e_0e30), 13);
    }

    #[test]
    fn fn_0096bcd0_updates_level_0_and_its_followers() {
        let mut e = Engine::new();
        globals(&mut e);
        e.map(0x0101_6000, 0x1000);
        e.mem.set_u32(0x0101_62c0, 2.0f32.to_bits());
        e.map(0x0108_4000, 0x1000);
        let world = World::new(&mut e, &[0x100, 0x25c, 0x24]);
        let actors: Vec<u32> = (0..3).map(|_| world.object(&mut e)).collect();
        levels(&mut e, actors.clone());
        removal_doubles(&mut e);
        time_doubles(&mut e, 5.0);
        quiet(
            &mut e,
            &[
                0x0040_4eb0,
                0x0040_4ee0,
                0x008c_8bd0,
                0x008c_8fd0,
                0x0088_b510,
                0x0088_b5a0,
                0x0096_e870,
                0x0097_7130,
                0x0069_3d50,
                0x0042_2690,
                0x0042_2720,
            ],
        );
        e.register(0x0084_d030, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        let flags = by_first_word(
            &mut e,
            &[
                0x0057_6d30,
                0x0044_0da0,
                0x0044_0d80,
                0x0093_1850,
                0x008b_0520,
                0x0094_df60,
                0x0041_ca90,
                0x0082_56d0,
            ],
        );
        e.register(ACTOR_PROCESS, |_, a| (a[0] + 0x80).into_ret());
        e.mem.set_u32(PLAYER, 0x900);
        flags.borrow_mut().insert((0x0094_df60, 0x900), 1);
        // Actor 0: a character handed to its process. Actor 1: an updated non-character with
        // two followers, one following (package type 7), one not. Actor 2: not eligible.
        world.answer(actors[0], 0x100, 1);
        flags.borrow_mut().insert((0x008b_0520, actors[0]), 1);
        flags.borrow_mut().insert((0x0044_0d80, actors[2]), 1);
        flags.borrow_mut().insert((0x0044_0da0, actors[2]), 1);
        let follower_a = world.object(&mut e);
        let follower_b = world.object(&mut e);
        flags.borrow_mut().insert((0x0093_1850, follower_a), 1);
        e.register(0x005d_43c0, |_, a| (a[0] + 2).into_ret());
        let followers = e.mem.alloc(0x10);
        let nodes = e.mem.alloc(0x10);
        e.mem.set_u32(nodes, follower_a);
        e.mem.set_u32(nodes + 4, follower_b);
        e.mem.set_u32(followers + 0xc, nodes);
        e.register_double(0x0042_2700, move |_, _| followers.into_ret());
        let copy = e.mem.alloc(0x10);
        e.mem.set_u32(copy, follower_a);
        e.mem.set_u32(copy + 4, follower_b);
        // The second list allocated is the copy; 00905820 is doubled as a no-op.
        let blocks = Rc::new(RefCell::new(vec![copy, e.mem.alloc(0x40)]));
        let next = blocks.clone();
        e.register_double(0x0040_1000, move |_, _| {
            next.borrow_mut().pop().unwrap().into_ret()
        });
        quiet(&mut e, &[0x0090_5820]);
        e.register(0x0088_1510, |_, a| (a[0] + 3).into_ret());
        e.register(0x0041_cb10, |_, _| 0u32.into_ret());
        flags.borrow_mut().insert((0x0041_ca90, follower_a + 3), 7);
        flags.borrow_mut().insert((0x0041_ca90, follower_b + 3), 4);
        flags.borrow_mut().insert((0x0082_56d0, nodes), 1);
        let this = e.mem.alloc(0x103c0);
        e.call_log = Some(vec![]);
        e.call(0x0096_bcd0, &args![this, 0.5f32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, SLOT_BASE + 0x24),
            vec![vec![actors[0] + 0x80, actors[0], 0]]
        );
        assert_eq!(calls(&log, 0x008c_8fd0), vec![vec![actors[0] + 0x80, 1]]);
        assert_eq!(
            calls(&log, 0x0088_b510),
            vec![vec![actors[0], 2.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&log, SLOT_BASE + 0x25c),
            vec![vec![actors[1], 0.5f32.to_bits()]]
        );
        assert_eq!(e.mem.u32(this + 0x103b0), 1);
        assert_eq!(
            calls(&log, 0x0069_3d50),
            vec![vec![follower_a + 0x80, 4.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&log, 0x0042_2690),
            vec![vec![actors[1] + 2, follower_b]]
        );
        assert_eq!(calls(&log, 0x0042_2720), vec![vec![actors[1] + 2]]);
        assert_eq!(calls(&log, 0x0096_e870), vec![vec![this, actors[2]]]);
        assert_eq!(calls(&log, 0x0097_7130), vec![vec![this, 0.5f32.to_bits()]]);
        let scope = calls(&log, 0x0040_4eb0)[0][0];
        assert_eq!(calls(&log, 0x0040_4ee0), vec![vec![scope]]);
    }

    #[test]
    fn fn_0096c330_perceives_and_reports_characters() {
        let mut e = Engine::new();
        globals(&mut e);
        e.map(0x0101_7000, 0x1000);
        e.mem.set_u32(0x0101_712c, 5.0f32.to_bits());
        e.map(0x011c_c000, 0x2000);
        e.map(0x011d_8000, 0x1000);
        let world = World::new(
            &mut e,
            &[0x100, 0x1d0, 0x2b4, 0x22c, 0x2b0, 0x20, 0x70, 0x2b8],
        );
        let actors: Vec<u32> = (0..2).map(|_| world.object(&mut e)).collect();
        levels(&mut e, actors.clone());
        e.mem.set_u32(PLAYER, 0x900);
        for actor in &actors {
            world.answer(*actor, 0x100, 1);
            world.answer(*actor, 0x1d0, 1);
            e.mem.set_u8(actor + 0xbc, 1);
        }
        // Actor 0 has a negative value and is checked; actor 1 a positive one.
        world.answer_float(actors[0], 0x2b4, -1.0);
        world.answer_float(actors[1], 0x2b4, 1.0);
        e.register(0x0084_d030, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.register(0x0040_ebd0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        e.register(0x0057_23b0, |_, _| Ret {
            st0: 100.0,
            ..Ret::default()
        });
        quiet(
            &mut e,
            &[0x0047_c850, 0x008c_8bb0, 0x0043_7bd0, 0x0070_14e0],
        );
        e.register(0x0047_6b70, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[1])),
            ..Ret::default()
        });
        let data = world.object(&mut e);
        e.register_double(0x0041_81e0, move |_, _| (data - 0x30).into_ret());
        let process = world.object(&mut e);
        e.register_double(ACTOR_PROCESS, move |_, _| process.into_ret());
        world.answer(process, 0x70, 1);
        e.register(0x0049_3bb0, |_, _| 0u32.into_ret());
        e.register(0x008a_0d10, |_, _| 3u32.into_ret());
        e.register(0x008b_06d0, |_, _| 1u32.into_ret());
        let settings = e.mem.alloc(8);
        e.mem.set_u32(settings, 150.0f32.to_bits());
        e.mem.set_u32(settings + 4, 2.0f32.to_bits());
        e.register_double(0x0040_3e20, move |_, a| {
            if a[0] == 0x011c_cf94 {
                settings
            } else {
                settings + 4
            }
            .into_ret()
        });
        quiet(&mut e, &[0x0096_e3c0, 0x0096_6f20, 0x0096_7350]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0096_c330, &args![this]);
        let log = e.call_log.take().unwrap();
        // The spread is min(12 * 0.5, 5.0) = 5.0 (12 is the level's end index), given to slot
        // 0x2b0.
        assert_eq!(
            calls(&log, SLOT_BASE + 0x2b0),
            vec![vec![actors[0], 5.0f32.to_bits()]]
        );
        assert_eq!(calls(&log, 0x0096_e3c0), vec![vec![0x011e_0e80, actors[0]]]);
        assert_eq!(calls(&log, 0x0096_6f20), vec![vec![0x900, actors[0], 1, 0]]);
        assert_eq!(calls(&log, SLOT_BASE + 0x2b8), vec![vec![actors[1]]]);
        assert!(calls(&log, 0x0096_7350).is_empty());
        // The level does not start at index 0 and is not empty.
        assert!(calls(&log, 0x008c_8bb0).is_empty());
    }
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn the_deleting_destructors_free_on_bit_0() {
        let mut e = Engine::new();
        e.register(0x004e_de70, |_, _| Ret::default());
        e.register(0x0096_a930, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0096_af60, &[this, 1]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0108_c0c8);
        assert_eq!(calls_to(&e, 0x0040_1030), vec![vec![this]]);
        assert_eq!(e.call(0x0096_af90, &[this, 0]).u32(), this);
        assert_eq!(calls_to(&e, 0x0096_a930), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x0040_1030).len(), 1);
        e.call(0x0096_af90, &[this, 1]);
        assert_eq!(calls_to(&e, 0x0040_1030).len(), 2);
    }

    #[test]
    fn fn_0096bc50_counts_the_accepting_actors() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.register(LEVEL_BEGIN, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(LEVEL_END, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        let actors: Vec<u32> = (0..5)
            .map(|i| {
                let actor = e.mem.alloc(8);
                let table = e.mem.alloc(0x104);
                e.mem.set_u32(table + 0x100, 0x7100_0000 + i);
                e.mem.set_u32(actor, table);
                actor
            })
            .collect();
        let list = actors.clone();
        e.register_double(ACTOR_AT, move |_, a| Ret {
            eax: if a[1] == 2 { 0 } else { list[a[1] as usize] },
            ..Ret::default()
        });
        for i in 0..5 {
            e.register(0x7100_0000 + i, |_, _| Ret::default());
        }
        e.register(0x7100_0001, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(0x7100_0004, |_, _| Ret {
            eax: 0x101,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0096_bc50, &[this]).u32(), 2);
        assert_eq!(calls_to(&e, LEVEL_BEGIN), vec![vec![this + 4, 0]]);
        assert_eq!(calls_to(&e, ACTOR_AT).len(), 4);
    }
}
