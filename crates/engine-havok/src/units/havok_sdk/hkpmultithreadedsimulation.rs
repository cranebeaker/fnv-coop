//! `Havok SDK/hkpmultithreadedsimulation.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpsphereshape::{profile_record, PROFILE_END};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d04320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d0_2da0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d04320(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d0_2da0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// Address of element `index` (8 bytes each) of the array at `array`.
fn element_00d04200(array: u32, index: i32) -> u32 {
    array.wrapping_add((index as u32).wrapping_mul(8))
}

/// Swaps the two 8-byte elements.
fn swap_00d04200(e: &mut Engine, first: u32, second: u32) {
    let (a0, a1) = (e.mem.u32(first), e.mem.u32(first + 4));
    let (b0, b1) = (e.mem.u32(second), e.mem.u32(second + 4));
    e.mem.set_u32(first, b0);
    e.mem.set_u32(first + 4, b1);
    e.mem.set_u32(second, a0);
    e.mem.set_u32(second + 4, a1);
}

// Translated from 00d04200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAlgorithm::quickSortRecursive<hkpTypedBroadPhaseHandlePair, bool
/// (__cdecl*)(const hkpTypedBroadPhaseHandlePair&, const
/// hkpTypedBroadPhaseHandlePair&)>` (map name). cdecl, four words: sorts
/// elements `low..=high` of the array of 8-byte pairs at `array` with the
/// less-than function `less` (called by address as `less(&a, &b)`, result
/// in AL). The pivot is a copy of the middle element on the stack; the
/// partition scans, swaps and recursion are those of the code (recursion on
/// the left part, a loop on the right part).
pub fn hk_algorithm_quick_sort_recursive_hkp_typed_broad_phase_handle_pair(
    e: &mut Engine,
    array: Ptr,
    mut low: i32,
    high: i32,
    less: u32,
) {
    let array = array.addr();
    loop {
        let (mut i, mut j) = (low, high);
        let middle = element_00d04200(array, low.wrapping_add(high) >> 1);
        let (pivot_first, pivot_second) = (e.mem.u32(middle), e.mem.u32(middle + 4));
        e.with_stack(8, |e, pivot| {
            let pivot = pivot.addr();
            e.mem.set_u32(pivot, pivot_first);
            e.mem.set_u32(pivot + 4, pivot_second);
            loop {
                if e.call(less, &args![element_00d04200(array, i), pivot])
                    .bool()
                {
                    loop {
                        i = i.wrapping_add(1);
                        if !e
                            .call(less, &args![element_00d04200(array, i), pivot])
                            .bool()
                        {
                            break;
                        }
                    }
                }
                if e.call(less, &args![pivot, element_00d04200(array, j)])
                    .bool()
                {
                    loop {
                        j = j.wrapping_sub(1);
                        if !e
                            .call(less, &args![pivot, element_00d04200(array, j)])
                            .bool()
                        {
                            break;
                        }
                    }
                }
                if j < i {
                    break;
                }
                if j != i {
                    swap_00d04200(e, element_00d04200(array, i), element_00d04200(array, j));
                }
                j = j.wrapping_sub(1);
                i = i.wrapping_add(1);
                if i > j {
                    break;
                }
            }
        });
        if low < j {
            hk_algorithm_quick_sort_recursive_hkp_typed_broad_phase_handle_pair(
                e,
                Ptr::new(array),
                low,
                j,
                less,
            );
        }
        if i < high {
            low = i;
        } else {
            return;
        }
    }
}

/// `TlsGetValue` is `0x00fd_f044`; the critical section imports are `EnterCriticalSection`
/// (`0x00fd_f05c`), `DeleteCriticalSection` (`0x00fd_f058`) and `LeaveCriticalSection`
/// (`0x00fd_f100`).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `hkArrayUtil`'s growth step of a full array (`00c90940`, cdecl: allocator, array, element size).
const ARRAY_GROW: u32 = 0x00c9_0940;
/// `hkpWorldAgentUtil::addAgent` (`00cc0f40`, cdecl: collidable a, collidable b, dispatcher holder).
const ADD_AGENT: u32 = 0x00cc_0f40;
/// `hkAgentNnMachine_FindAgent` (`00cfdd70`, cdecl: collidable a, collidable b) and
/// `hkpWorldAgentUtil::removeAgent` (`00cc10b0`, cdecl: agent).
const FIND_AGENT: u32 = 0x00cf_dd70;
const REMOVE_AGENT: u32 = 0x00cc_10b0;
/// `hkpContinuousSimulation`'s constructor (`00d0d020`, `this` and the world) and destructor
/// (`00d0d0d0`, `this`).
const CONTINUOUS_CONSTRUCT: u32 = 0x00d0_d020;
const CONTINUOUS_DESTRUCT: u32 = 0x00d0_d0d0;
/// `hkCriticalSection`'s constructor (`00cf6da0`, `this` and the spin count), the
/// `hkpMultithreadConfig` one (`00c90f30`, `this`) and `hkpBroadPhase::enableMultiThreading`
/// (`00d97070`, `this` and the maximum number of pairs).
const CRITICAL_SECTION_CONSTRUCT: u32 = 0x00cf_6da0;
const MULTITHREAD_CONFIG_CONSTRUCT: u32 = 0x00c9_0f30;
const ENABLE_MULTITHREADING: u32 = 0x00d9_7070;
/// `operator delete` (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The text buffer constructor the error report uses (`00cb1600`, `this`, buffer, size), the
/// function that writes text into it (`00cc01c0`, `this`, text), its destructor (`00cc00e0`,
/// `this`) and the global error reporter (`01268360`) whose vtable slot `0xc` takes
/// `(severity, id, text, file, line)`.
const TEXT_BUFFER_CONSTRUCT: u32 = 0x00cb_1600;
const TEXT_BUFFER_WRITE: u32 = 0x00cc_01c0;
const TEXT_BUFFER_DESTRUCT: u32 = 0x00cc_00e0;
const ERROR_REPORTER: u32 = 0x0126_8360;
/// The profiling label `010ccc18` begins the Toi event allocation.
const TOI_LABEL: u32 = 0x010c_cc18;

/// The simulation (`hkpMultiThreadedSimulation`, derived from `hkpContinuousSimulation`):
/// the world at `+0xc`, the vtables the constructor sets (`010ccbc4` for the simulation, then
/// the three listener sub-objects at `+0x50`, `+0x58` and `+0x60`: `010ccb84`, `010ccb94`,
/// `010ccba4`), the in-step flag at `+0x68`, the pending add pairs at `+0x6c` and remove pairs
/// at `+0x90` (arrays of 8-byte pairs: data, size, capacity word), the critical sections
/// at `+0x78`, `+0x9c`, `+0x100` and `+0x140`; `+0xb4` holds the multithread config and
/// `+0x30`/`+0x34`/`+0x38` the Toi event array.
const SIM_WORLD: u32 = 0x0c;
const SIM_LISTENER_ENTITY: u32 = 0x50;
const SIM_LISTENER_BORDER: u32 = 0x58;
const SIM_LISTENER_PHANTOM: u32 = 0x60;
const SIM_IN_STEP: u32 = 0x68;
const SIM_ADD_PAIRS: u32 = 0x6c;
const SIM_REMOVE_PAIRS: u32 = 0x90;
const SIM_CONFIG: u32 = 0xb4;
const SIM_TOI_EVENTS: u32 = 0x30;
const SIM_TOI_COUNT: u32 = 0x34;
const SIM_TOI_CAPACITY: u32 = 0x38;
const CAPACITY_MASK: u32 = 0x3fff_ffff;
const DONT_DEALLOCATE_FLAG: u32 = 0x8000_0000;
const SIMULATION_VTABLE: u32 = 0x010c_cbc4;
const LISTENER_VTABLES: [u32; 3] = [0x010c_cb84, 0x010c_cb94, 0x010c_cba4];
const LISTENER_BASE_VTABLE: u32 = 0x010c_63bc;
/// The spin count of the simulation's critical sections (`0xfa0`).
const SPIN_COUNT: u32 = 4000;
/// The maximum number of broad phase pairs the constructor asks for (`0x2710`).
const MAX_PAIRS: u32 = 10000;

/// The sign-extended byte at `addr`, as the `MOVSX` instructions load it.
fn signed_byte(e: &Engine, addr: u32) -> u32 {
    i32::from(e.mem.i8(addr)) as u32
}

/// The linked collidable a broad phase handle points back to: `handle + (signed byte at
/// handle + 5)`.
fn handle_owner(e: &Engine, handle: u32) -> u32 {
    handle.wrapping_add(signed_byte(e, handle + 5))
}

/// The object (unconfirmed class, with the quality byte at `+0xe8`, the number at `+0xcc`
/// and vtable slots `0x1c`/`0x24` notified about pairs) of a handle: its owner plus the signed
/// byte at `owner + 0x10`.
fn handle_object(e: &Engine, handle: u32) -> u32 {
    let owner = handle_owner(e, handle);
    owner.wrapping_add(signed_byte(e, owner + 0x10))
}

/// What the phantom and border listeners do for a pair of broad phase handles: when the
/// first handle has type `kind` (the byte at `+4`) and (for `skip_if_both`) the second does
/// not, the first handle's object is called through vtable `slot` with the second's owner;
/// then, if the second (read again) has type `kind`, the second's object with the first's
/// owner.
fn notify_pair(e: &mut Engine, pair: u32, kind: u8, slot: u32, skip_if_both: bool) {
    let first = e.mem.u32(pair);
    if e.mem.u8(first + 4) == kind {
        let second = e.mem.u32(pair + 4);
        if skip_if_both && e.mem.u8(second + 4) == kind {
            return;
        }
        let target = handle_object(e, first);
        let argument = handle_owner(e, second);
        e.vcall(target, slot, &args![argument]);
    }
    let second = e.mem.u32(pair + 4);
    if e.mem.u8(second + 4) == kind {
        let first = e.mem.u32(pair);
        let target = handle_object(e, second);
        let argument = handle_owner(e, first);
        e.vcall(target, slot, &args![argument]);
    }
}

// Translated from 00d01fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::MtPhantomBroadPhaseListener::addCollisionPair` (engine map):
/// `ECX` (unused) and one stack word, the pair of two broad phase handle pointers. For each
/// handle of type 2 (byte at `+4`), in turn (the first one first), calls vtable slot `0x1c` of
/// that handle's object with the owner of the other handle (`MOVSX` of the bytes at `+5` and at
/// `owner + 0x10` give the offsets; the second call is a tail jump).
pub fn mt_phantom_broad_phase_listener_add_collision_pair(e: &mut Engine, _this: Ptr, pair: u32) {
    notify_pair(e, pair, 2, 0x1c, false);
}

// Translated from 00d02020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::MtPhantomBroadPhaseListener::removeCollisionPair` (engine map):
/// as [`mt_phantom_broad_phase_listener_add_collision_pair`] with vtable slot `0x24`.
pub fn mt_phantom_broad_phase_listener_remove_collision_pair(
    e: &mut Engine,
    _this: Ptr,
    pair: u32,
) {
    notify_pair(e, pair, 2, 0x24, false);
}

// Translated from 00d02080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::MtBroadPhaseBorderListener::addCollisionPair` (engine map):
/// `ECX` (unused) and the pair. As the phantom listener's, for handle type 3, except that a
/// pair whose first handle has type 3 and whose second does too is ignored.
pub fn mt_broad_phase_border_listener_add_collision_pair(e: &mut Engine, _this: Ptr, pair: u32) {
    notify_pair(e, pair, 3, 0x1c, true);
}

// Translated from 00d020f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::MtBroadPhaseBorderListener::removeCollisionPair` (engine map):
/// as [`mt_broad_phase_border_listener_add_collision_pair`] with vtable slot `0x24`.
pub fn mt_broad_phase_border_listener_remove_collision_pair(e: &mut Engine, _this: Ptr, pair: u32) {
    notify_pair(e, pair, 3, 0x24, true);
}

/// The number at `+0xd4` of a handle's object (signed).
fn handle_order(e: &Engine, handle: u32) -> i32 {
    e.mem.i32(handle_object(e, handle) + 0xd4)
}

// Translated from 00d02160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The less-than function `sortPairList` gives the quick sort (cdecl, two pair pointers; no
/// map name): compares the number at `+0xd4` of the objects (see `handle_object`) of the
/// pairs' first handles, signed, and when equal those of the second handles.
pub fn fn_00d02160(e: &mut Engine, left: u32, right: u32) -> bool {
    let (left_first, right_first) = (e.mem.u32(left), e.mem.u32(right));
    let (a, b) = (handle_order(e, left_first), handle_order(e, right_first));
    if a < b {
        return true;
    }
    if a != b {
        return false;
    }
    let (left_second, right_second) = (e.mem.u32(left + 4), e.mem.u32(right + 4));
    handle_order(e, left_second) < handle_order(e, right_second)
}

// Translated from 00d021e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::getMultithreadConfig` (engine map): `ECX` = simulation and one
/// stack word, `out`; stores the word at `this + 0xb4` there.
pub fn get_multithread_config(e: &mut Engine, this: Ptr, out: u32) {
    let config = e.mem.u32(this.addr() + SIM_CONFIG);
    e.mem.set_u32(out, config);
}

// Translated from 00d021f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::setMultithreadConfig` (engine map): `ECX` = simulation and two
/// stack words (`RET 8`), the first the address of the new config word, which is stored at
/// `this + 0xb4`; the second is not used.
pub fn set_multithread_config(e: &mut Engine, this: Ptr, config: u32, _unused: u32) {
    let value = e.mem.u32(config);
    e.mem.set_u32(this.addr() + SIM_CONFIG, value);
}

// Translated from 00d02200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::assertThereIsNoCollisionInformationForAgent` (engine map):
/// `ECX` and one stack word, neither used.
pub fn assert_there_is_no_collision_information_for_agent(
    _e: &mut Engine,
    _this: Ptr,
    _agent: u32,
) {
}

// Translated from 00d02210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::assertThereIsNoCollisionInformationForEntities` (engine map):
/// `ECX` and three stack words (`RET 0xc`), none used.
pub fn assert_there_is_no_collision_information_for_entities(
    _e: &mut Engine,
    _this: Ptr,
    _first: u32,
    _second: u32,
    _third: u32,
) {
}

// Translated from 00d02250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `sortPairList` (engine map): cdecl, one word, the address of a list `{data, size}` of 8-byte
/// pairs of handles. First puts every pair in order (swaps the two handles when the number at
/// `+0x14` of the first one's owner is greater, signed, than the second's), then, with more than
/// one pair, sorts the list with the quick sort `00d04200` on `fn_00d02160`.
pub fn sort_pair_list(e: &mut Engine, list: u32) {
    let data = e.mem.u32(list);
    let size = e.mem.u32(list + 4);
    let end = data.wrapping_add(size.wrapping_mul(8));
    let mut pair = data;
    while pair < end {
        let (first, second) = (e.mem.u32(pair), e.mem.u32(pair + 4));
        let first_number = e.mem.i32(handle_owner(e, first) + 0x14);
        let second_number = e.mem.i32(handle_owner(e, second) + 0x14);
        if first_number > second_number {
            e.mem.set_u32(pair, second);
            e.mem.set_u32(pair + 4, first);
        }
        pair = pair.wrapping_add(8);
    }
    let count = e.mem.i32(list + 4);
    if count > 1 {
        let data = e.mem.u32(list);
        e.call(
            0x00d0_4200,
            &args![data, 0i32, count.wrapping_sub(1), 0x00d0_2160u32],
        );
    }
}

/// Adds the pair to the array at `array` (data, size, capacity word) of 8-byte elements:
/// when the size equals the capacity (flags masked off) the array grows through `00c90940`
/// with the memory router's allocator; the element is stored if its address is not 0; the size
/// grows by one.
fn push_pair(e: &mut Engine, array: u32, pair: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 8u32]);
    }
    let slot = e
        .mem
        .u32(array)
        .wrapping_add(e.mem.u32(array + 4).wrapping_mul(8));
    if slot != 0 {
        let (first, second) = (e.mem.u32(pair), e.mem.u32(pair + 4));
        e.mem.set_u32(slot, first);
        e.mem.set_u32(slot + 4, second);
    }
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// The test both entity listeners start with: the simulation is in its step (byte at `+0x68`),
/// neither object has the quality byte `5` (at `+0xe8`) and the numbers at `+0xcc` differ.
fn pair_is_deferred(e: &Engine, simulation: u32, first_object: u32, second_object: u32) -> bool {
    e.mem.u8(simulation + SIM_IN_STEP) != 0
        && e.mem.u8(first_object + 0xe8) != 5
        && e.mem.u8(second_object + 0xe8) != 5
        && e.mem.u32(first_object + 0xcc) != e.mem.u32(second_object + 0xcc)
}

/// The two owners of the pair and whether it is to be deferred (see `pair_is_deferred`).
fn classify_pair(e: &Engine, simulation: u32, pair: u32) -> (u32, u32, bool) {
    let (first, second) = (e.mem.u32(pair), e.mem.u32(pair + 4));
    let (a, b) = (handle_owner(e, first), handle_owner(e, second));
    let first_object = a.wrapping_add(signed_byte(e, a + 0x10));
    let second_object = b.wrapping_add(signed_byte(e, b + 0x10));
    (
        a,
        b,
        pair_is_deferred(e, simulation, first_object, second_object),
    )
}

// Translated from 00d026e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::MtEntityEntityBroadPhaseListener::addCollisionPair` (engine map):
/// `ECX` = listener, whose `+4` is the simulation, and one stack word, the pair of handles.
/// While the simulation is in its step (byte `+0x68`) and neither object has quality byte 5
/// and their numbers at `+0xcc` differ, the pair is appended to the simulation's pending add
/// list (`+0x6c`). Otherwise, if the byte at `[table] + 0x1bb0 + kind a * 10 + kind b` (kinds
/// are the signed bytes at `+0x1a` of the two owners; `table` is `[[world + 0x78]]`) is not 0,
/// `[holder + 0x14]` is set to the word at `[table] + 0x1c30 + (byte << 6)` (the byte
/// sign-extended) and the agent is added through `00cc0f40` with both owners and `holder`
/// (`[world + 0x78]`).
pub fn mt_entity_entity_broad_phase_listener_add_collision_pair(
    e: &mut Engine,
    this: Ptr,
    pair: u32,
) {
    let simulation = e.mem.u32(this.addr() + 4);
    let (a, b, deferred) = classify_pair(e, simulation, pair);
    if deferred {
        push_pair(e, simulation + SIM_ADD_PAIRS, pair);
        return;
    }
    let world = e.mem.u32(simulation + SIM_WORLD);
    let holder = e.mem.u32(world + 0x78);
    let table = e.mem.u32(holder);
    let kinds = signed_byte(e, a + 0x1a).wrapping_mul(10);
    let at = table
        .wrapping_add(signed_byte(e, b + 0x1a))
        .wrapping_add(kinds)
        .wrapping_add(0x1bb0);
    let index = e.mem.u8(at);
    if index != 0 {
        let record_at = table
            .wrapping_add((i32::from(index as i8) as u32) << 6)
            .wrapping_add(0x1c30);
        let record = e.mem.u32(record_at);
        e.mem.set_u32(holder + 0x14, record);
        e.call(ADD_AGENT, &args![a, b, holder]);
    }
}

// Translated from 00d027c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::MtEntityEntityBroadPhaseListener::removeCollisionPair` (engine
/// map): as the add function, but the deferred pair goes to the simulation's pending remove
/// list (`+0x90`); otherwise the agent between the two owners is looked up with `00cfdd70` and,
/// if there is one, removed with `00cc10b0`.
pub fn mt_entity_entity_broad_phase_listener_remove_collision_pair(
    e: &mut Engine,
    this: Ptr,
    pair: u32,
) {
    let simulation = e.mem.u32(this.addr() + 4);
    let (a, b, deferred) = classify_pair(e, simulation, pair);
    if deferred {
        push_pair(e, simulation + SIM_REMOVE_PAIRS, pair);
        return;
    }
    let agent = e.call(FIND_AGENT, &args![a, b]).u32();
    if agent != 0 {
        e.call(REMOVE_AGENT, &args![agent]);
    }
}

// Translated from 00d02c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::hkpMultiThreadedSimulation` (engine map): `ECX` = simulation and
/// one stack word, the world. Runs the `hkpContinuousSimulation` constructor (`00d0d020`), sets
/// the vtables (`010ccbc4` at `+0`; `010ccb84` at `+0x50`, `010ccb94` at `+0x58`, `010ccba4`
/// at `+0x60`), `+0x54`, `+0x5c` and `+0x64` = 0 and then `+0x54` = the simulation and `+0x5c`
/// and `+0x64` = its `+0x140`, empty arrays at `+0x6c` and `+0x90` (capacity word
/// `0x80000000`), critical sections (`00cf6da0`, spin count 4000) at `+0x78`, `+0x9c`, `+0x100`
/// and `+0x140`, the config (`00c90f30`) at `+0xb4`, `+0xc0` and the byte at `+0x68` = 0,
/// enables multithreading on the world's broad phase (`[[this + 0xc] + 0x58]`, 10000), and
/// hooks the three listener sub-objects into the world's listener table (`[world + 0x64]`:
/// `+0x24` = `+0x50`; `+0x28`, `+0x44`, `+0x48` = `+0x58`; `+0x2c`, `+0x4c`, `+0x64`, `+0x68`,
/// `+0x6c` = `+0x60`). Returns `this`.
pub fn hkp_multi_threaded_simulation_construct(e: &mut Engine, this: Ptr, world: u32) -> Ptr {
    let me = this.addr();
    e.call(CONTINUOUS_CONSTRUCT, &args![me, world]);
    e.mem.set_u32(me, SIMULATION_VTABLE);
    for (offset, vtable) in [
        SIM_LISTENER_ENTITY,
        SIM_LISTENER_BORDER,
        SIM_LISTENER_PHANTOM,
    ]
    .into_iter()
    .zip(LISTENER_VTABLES)
    {
        e.mem.set_u32(me + offset, vtable);
        e.mem.set_u32(me + offset + 4, 0);
    }
    let border = me + SIM_LISTENER_BORDER;
    let phantom = me + SIM_LISTENER_PHANTOM;
    e.mem.set_u32(me + SIM_ADD_PAIRS, 0);
    e.mem.set_u32(me + SIM_ADD_PAIRS + 4, 0);
    e.mem.set_u32(me + SIM_ADD_PAIRS + 8, DONT_DEALLOCATE_FLAG);
    e.call(CRITICAL_SECTION_CONSTRUCT, &args![me + 0x78, SPIN_COUNT]);
    e.mem.set_u32(me + SIM_REMOVE_PAIRS, 0);
    e.mem.set_u32(me + SIM_REMOVE_PAIRS + 4, 0);
    e.mem
        .set_u32(me + SIM_REMOVE_PAIRS + 8, DONT_DEALLOCATE_FLAG);
    e.call(CRITICAL_SECTION_CONSTRUCT, &args![me + 0x9c, SPIN_COUNT]);
    e.call(MULTITHREAD_CONFIG_CONSTRUCT, &args![me + SIM_CONFIG]);
    e.call(CRITICAL_SECTION_CONSTRUCT, &args![me + 0x100, SPIN_COUNT]);
    e.call(CRITICAL_SECTION_CONSTRUCT, &args![me + 0x140, SPIN_COUNT]);
    e.mem.set_u32(me + 0xc0, 0);
    e.mem.set_u8(me + SIM_IN_STEP, 0);
    let own_world = e.mem.u32(me + SIM_WORLD);
    let broad_phase = e.mem.u32(own_world + 0x58);
    e.call(ENABLE_MULTITHREADING, &args![broad_phase, MAX_PAIRS]);
    e.mem.set_u32(me + 0x54, me);
    e.mem.set_u32(me + 0x5c, me + 0x140);
    let table = e.mem.u32(world + 0x64);
    e.mem.set_u32(table + 0x24, me + SIM_LISTENER_ENTITY);
    for offset in [0x44, 0x28, 0x48] {
        e.mem.set_u32(table + offset, border);
    }
    e.mem.set_u32(me + 0x64, me + 0x140);
    for offset in [0x2c, 0x64, 0x4c, 0x68, 0x6c] {
        e.mem.set_u32(table + offset, phantom);
    }
    this
}

/// Empties the array `{data, size, capacity word}` at `array` the way the destructor does: the
/// size becomes 0, the storage is returned to the allocator (slot `0x10`: data, capacity * 8)
/// unless bit 31 of the capacity word is set, and data and the capacity word are reset.
fn release_pairs(e: &mut Engine, array: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & DONT_DEALLOCATE_FLAG == 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK) << 3],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, DONT_DEALLOCATE_FLAG);
}

// Translated from 00d02da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::~hkpMultiThreadedSimulation` (engine map): `ECX` = simulation,
/// no stack words. Sets the vtable `010ccbc4`, deletes the critical sections at `+0x140`,
/// `+0x100` and `+0x9c`, empties the remove pair list (`+0x90`), deletes the one at `+0x78`,
/// empties the add pair list (`+0x6c`), sets the three listener vtables to `010c63bc` and
/// finishes with the `hkpContinuousSimulation` destructor (`00d0d0d0`, a tail jump).
pub fn hkp_multi_threaded_simulation_destruct(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, SIMULATION_VTABLE);
    e.call(DELETE_CRITICAL_SECTION, &args![me + 0x140]);
    e.call(DELETE_CRITICAL_SECTION, &args![me + 0x100]);
    e.call(DELETE_CRITICAL_SECTION, &args![me + 0x9c]);
    release_pairs(e, me + SIM_REMOVE_PAIRS);
    e.call(DELETE_CRITICAL_SECTION, &args![me + 0x78]);
    release_pairs(e, me + SIM_ADD_PAIRS);
    for offset in [
        SIM_LISTENER_PHANTOM,
        SIM_LISTENER_BORDER,
        SIM_LISTENER_ENTITY,
    ] {
        e.mem.set_u32(me + offset, LISTENER_BASE_VTABLE);
    }
    e.call(CONTINUOUS_DESTRUCT, &args![me]);
}

// Translated from 00d04300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::MtEntityEntityBroadPhaseListener::_scalar_deleting_destructor_`
/// (engine map): `ECX` = listener and one stack word, `flags`. Sets the vtable `010c63bc`; with
/// bit 0 of `flags` set it also deletes the object (`operator delete`, `00401030`). Returns
/// `this`.
pub fn mt_entity_entity_broad_phase_listener_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), LISTENER_BASE_VTABLE);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00d02e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::addToiEventWithCriticalSectionLock` (engine map): `ECX` =
/// simulation and three stack words (`result`, `entry`, `section`). The simulation's Toi event
/// array (`+0x30`: data, `+0x34`: size, `+0x38`: capacity word; elements of 0x70 bytes) must
/// have room (size different from the capacity with the flags masked off). If not, an error
/// is reported: a text buffer is built (`00cb1600`, buffer of 0x200 bytes), the message
/// "Toi event queue full ..." (`010ccc70`) is written into it (`00cc01c0`), the global error
/// reporter (`01268360`) is called through its vtable slot `0xc` with `(1, 0xf0323454, text,
/// file name 010ccc30, line 0x3f7)` and the buffer is destroyed (`00cc00e0`).
///
/// Otherwise the profiling label `010ccc18` is recorded, `section` is entered, the size grows
/// by one (the new element's `+0x18` and `+0x1c` floats are set to 0 if its address is not 0),
/// the section is left, `010c41f8` is recorded and the element is filled from `result` (the
/// output of a process collision) and `entry`: `+0` = the `float` at `result + 0x3030`, `+4` =
/// the one at `+0x3034` (both copied through the x87 stack), `+8` = the byte at
/// `[[world + 0x80] + (signed byte at entry + 0xc << 6) + 0x1c34]`, `+0xc` and `+0x10` = the
/// objects of the handles at `entry + 0x10` and `entry + 0x14` (handle plus the signed byte at
/// `+0x10`), `+0x14` = the word at `entry + 8`, `+0x18..0x2c` = the words at
/// `result + 0x3050..0x3064`, `+0x2c..0x48` = the seven words at `result + 0x3064`,
/// `+0x50` and `+0x60` = the 16 bytes at `result + 0x3010` and `result + 0x3020`.
pub fn add_toi_event_with_critical_section_lock(
    e: &mut Engine,
    this: Ptr,
    result: u32,
    entry: u32,
    section: u32,
) {
    let me = this.addr();
    let capacity = e.mem.u32(me + SIM_TOI_CAPACITY) & CAPACITY_MASK;
    if e.mem.u32(me + SIM_TOI_COUNT) == capacity {
        report_toi_queue_full(e);
        return;
    }
    profile_record(e, TOI_LABEL);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let index = e.mem.u32(me + SIM_TOI_COUNT);
    let data = e.mem.u32(me + SIM_TOI_EVENTS);
    e.mem.set_u32(me + SIM_TOI_COUNT, index.wrapping_add(1));
    let offset = index.wrapping_mul(0x70);
    let slot = data.wrapping_add(offset);
    if slot != 0 {
        e.mem.set_u32(slot + 0x18, 0);
        e.mem.set_u32(slot + 0x1c, 0);
    }
    let element = e.mem.u32(me + SIM_TOI_EVENTS).wrapping_add(offset);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    profile_record(e, PROFILE_END);
    fill_toi_event(e, me, element, result, entry);
}

/// Fills a Toi event element (see [`add_toi_event_with_critical_section_lock`]).
fn fill_toi_event(e: &mut Engine, simulation: u32, element: u32, result: u32, entry: u32) {
    let time = x87_quiet_bits(e.mem.u32(result + 0x3030));
    e.mem.set_u32(element, time);
    let world = e.mem.u32(simulation + SIM_WORLD);
    let table = e.mem.u32(world + 0x80);
    let kind = signed_byte(e, entry + 0xc);
    let quality = e
        .mem
        .u8((kind << 6).wrapping_add(table).wrapping_add(0x1c34));
    e.mem.set_u8(element + 8, quality);
    let fraction = x87_quiet_bits(e.mem.u32(result + 0x3034));
    e.mem.set_u32(element + 4, fraction);
    for (from, to) in [(0x3010, 0x50), (0x3020, 0x60)] {
        let bytes = e.mem.bytes(result + from, 16);
        e.mem.write(element + to, &bytes);
    }
    for (handle_offset, to) in [(0x10, 0x0c), (0x14, 0x10)] {
        let handle = e.mem.u32(entry + handle_offset);
        let object = handle.wrapping_add(signed_byte(e, handle + 0x10));
        e.mem.set_u32(element + to, object);
    }
    for (from, to) in [
        (0x3050, 0x18),
        (0x3054, 0x1c),
        (0x3058, 0x20),
        (0x305c, 0x24),
        (0x3060, 0x28),
    ] {
        let word = e.mem.u32(result + from);
        e.mem.set_u32(element + to, word);
    }
    let word = e.mem.u32(entry + 8);
    e.mem.set_u32(element + 0x14, word);
    for word in 0..7 {
        let value = e.mem.u32(result + 0x3064 + 4 * word);
        e.mem.set_u32(element + 0x2c + 4 * word, value);
    }
}

/// The error report of a full Toi event queue (see [`add_toi_event_with_critical_section_lock`]).
fn report_toi_queue_full(e: &mut Engine) {
    e.with_stack(0x214, |e, frame| {
        let object = frame.addr();
        let buffer = object + 0xc;
        e.call(TEXT_BUFFER_CONSTRUCT, &args![object, buffer, 0x200u32]);
        e.call(TEXT_BUFFER_WRITE, &args![object, 0x010c_cc70u32]);
        let reporter = e.global::<u32>(ERROR_REPORTER);
        e.vcall(
            reporter,
            0xc,
            &args![1u32, 0xf032_3454u32, buffer, 0x010c_cc30u32, 0x3f7u32],
        );
        e.call(TEXT_BUFFER_DESTRUCT, &args![object]);
    });
}

/// The job queue of the simulation while a step is running (`+0xc0`; 0 outside one).
const SIM_JOB_QUEUE: u32 = 0xc0;
/// `hkJobQueue::addJob` (`00d04b90`, `this` = the queue, `(job, 1)`) and `processAllJobs`
/// (`00d051b0`, `this` = the queue); `00d04a80` (`this` = the queue, `(0 or 1, count)`) prepares
/// the queue for a number of jobs.
const JOB_QUEUE_ADD_JOB: u32 = 0x00d0_4b90;
const JOB_QUEUE_PROCESS_ALL: u32 = 0x00d0_51b0;
const JOB_QUEUE_PREPARE: u32 = 0x00d0_4a80;
/// `hkpAgentSectorHeader::allocate` (`00d96530`, cdecl: `(sector count, sector size)`).
const SECTOR_HEADER_ALLOCATE: u32 = 0x00d9_6530;
/// `hkCpuPostCollideJob` (`00d931a0`, cdecl: `(job state, 0, job)`).
const POST_COLLIDE_JOB: u32 = 0x00d9_31a0;
/// `hkpConstraintQueryIn::set` (`00cfa6e0`, `this` = the query at `state + 0x10`, then
/// `(world + 0x1e0, world + 0x1d0, [world + 0x1b8])`) and the 0x74-byte copy `00cfa7d0`
/// (`this` = the copy at `state + 0x90`, `source`).
const CONSTRAINT_QUERY_SET: u32 = 0x00cf_a6e0;
const COPY_INTEGRATION_INFO: u32 = 0x00cf_a7d0;
/// `hkpContinuousSimulation::processAgentNnEntries` (`00d0c9c0`, translated as taking
/// `(this, entries, count, input, owner, use_toi_quality)`; the callers leave their
/// `this` in `ECX`).
const CONTINUOUS_PROCESS_ENTRIES: u32 = 0x00d0_c9c0;
/// The vtable word the post collide job state keeps at `+0x11c`.
const POST_COLLIDE_STATE_TABLE: u32 = 0x0126_9210;
/// The global the exe compares with 1 to stop a step early (`01269200`).
const STOP_FLAG: u32 = 0x0126_9200;

/// The four words of a step info `(start, end, delta, 1 / delta)` the exe builds from two
/// stored times: the times pass through the x87 stack, the delta is the single precision
/// difference `end - start` and the inverse is 0 for a delta that compares equal to 0 (the
/// `FUCOM` test; a NaN delta is divided) and `1 / delta` otherwise.
fn step_info(start: u32, end: u32) -> [u32; 4] {
    let (start, end) = (x87_quiet_bits(start), x87_quiet_bits(end));
    let delta = (x87_load32(end) - x87_load32(start)) as f32;
    let inverse = if delta == 0.0 {
        0.0
    } else {
        (1.0 / f64::from(delta)) as f32
    };
    [start, end, delta.to_bits(), inverse.to_bits()]
}

/// Adds `amount` to the word at `addr`.
fn add_to_word(e: &mut Engine, addr: u32, amount: i32) {
    let value = e.mem.u32(addr).wrapping_add(amount as u32);
    e.mem.set_u32(addr, value);
}

/// What the exe does after taking one off the word at `world + 0x94` (a nesting count of the
/// world; the purpose of the two callees is unconfirmed): when it reaches 0 and the byte at
/// `+0x9c` is 0, `00c91120` runs if the word at `+0x8c` is not 0 and then `00c91140` if the
/// word at `+0xa4` is 1 and the one at `+0x90` is not 0 (both with the world as `this`).
fn leave_world_scope(e: &mut Engine, world: u32) {
    add_to_word(e, world + 0x94, -1);
    if e.mem.u32(world + 0x94) != 0 || e.mem.u8(world + 0x9c) != 0 {
        return;
    }
    if e.mem.u32(world + 0x8c) != 0 {
        e.call(0x00c9_1120, &args![world]);
    }
    if e.mem.u32(world + 0xa4) == 1 && e.mem.u32(world + 0x90) != 0 {
        e.call(0x00c9_1140, &args![world]);
    }
}

/// Builds the post collide job state `state` (0x120 bytes of the caller's stack) and runs the
/// post collide job on it, the way the three narrow phase functions do: the zeroed words at
/// `+0x94` and `+0xf4`, the world's integration info copied to `+0x90`, the constraint query
/// at `+0x10`, the simulation at `+0x110`, the step info at `+0x114`, the word at `[[world +
/// 0x78] + 8]` (through the x87 stack) at `+0x118`, the table `01269210` at `+0x11c` and the
/// world at `+0`; then `00d931a0(state, 0, job)` and the release of the block at `+0xf4` (the
/// allocator's slot 8 with 0x200 bytes) when the job left one there.
fn run_post_collide_job(e: &mut Engine, simulation: u32, state: u32, job: u32) {
    let world = e.mem.u32(simulation + SIM_WORLD);
    e.mem.set_u32(state + 0x114, world + 0x1d0);
    e.mem.set_u32(state + 0x94, 0);
    e.mem.set_u32(state + 0xf4, 0);
    let integrator = e.mem.u32(world + 0x78);
    e.call(COPY_INTEGRATION_INFO, &args![state + 0x90, integrator]);
    let query = e.mem.u32(world + 0x1b8);
    e.call(
        CONSTRAINT_QUERY_SET,
        &args![state + 0x10, world + 0x1e0, world + 0x1d0, query],
    );
    e.mem.set_u32(state + 0x110, simulation);
    let world = e.mem.u32(simulation + SIM_WORLD);
    let integrator = e.mem.u32(world + 0x78);
    let time = x87_quiet_bits(e.mem.u32(integrator + 8));
    e.mem.set_u32(state + 0x11c, POST_COLLIDE_STATE_TABLE);
    e.mem.set_u32(state + 0x118, time);
    e.mem.set_u32(state, world);
    e.call(POST_COLLIDE_JOB, &args![state, 0u32, job]);
    let block = e.mem.u32(state + 0xf4);
    if block != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![block, 0x200u32]);
    }
}

// Translated from 00d022b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::stepBeginSt` (Ghidra name): `ECX` = simulation, then `queue` (the
/// job queue the step uses) and the step's time `delta_time` (a float passed as its bit
/// pattern) on the stack. Between the profiling records `"LtPhysics"`/`"StInit"` and `010c41f8`
/// (the final one is `01076f38`): sets the in-step byte, takes one on the world's word at
/// `+0x94`, stores the delta time at `+0x1c`, and builds the step info `(start, start + delta,
/// delta, inverse)` with `start` the float at `+0x18` (see `step_info`), which goes to the world
/// at `+0x1d0` and to the integration data at `[world + 0x78] + 0x50`. The object at
/// `[world + 0x4c]` is told through its slot `0x14` `(world, info)`, `00cb55d0(world)` runs and
/// `00c92b50` (the memory the integration needs, `this` = world) fills a word that the object
/// at `[router + 0x18]` (slot `0x20`) is asked about. When it answers 0, the memory watchdog
/// (`00c93570`) is asked: without one the byte at `world + 0x54` stops the exe with an
/// `INT3` (panic here) or else the word at `+0x28` is set to 1 and 1 is returned at once;
/// with one the world scope is left (`leave_world_scope`), `00d01bc0(world)` runs and the scope
/// is entered again. Then `00d892d0` runs, the step info is written to the world again, the
/// world's `+0x2ec` (`[+0x300] * delta`) and `+0x2f0` (`[+0x2f4] * inverse`) are computed
/// through the x87 stack, the vectors at `+0x1f0` and `+0x200` are the vector at `+0x10`
/// multiplied by `+0x2ec` and by the delta, the word behind `[world + 0x1b8]` is cleared and the
/// integration state at `[world + 0x74]` is set up (the info pointer at `+0x114`, the copy
/// `00cfa7d0` at `+0x90`, the constraint query `00cfa6e0` at `+0x10`, the simulation at `+0x110`,
/// the float at `[[world + 0x78] + 8]` at `+0x118`, the table `01269210` at `+0x11c`, the
/// world at `+0`). With `count = [world + 0x2c]` above 0: when the byte at `world + 0xb0` is set
/// the record `"StActions"` is written and `00cf8b00` runs on the simulation with the word at
/// `world + 0x98` taken down around it, then the queue is prepared twice (`00d04a80` with `(0,
/// count)` and `(1, count)`) and a job (bytes 0, 0, 2, halfwords `0x30` and `0xffff`, `[world
/// + 0x74]` and `count` at `+0x1c` and `+0x20`) is added. The three bytes at `world + 0x305` to
/// `+0x307` are stepped (a counter whose bits 0 to 2 toggle bits of the first, and a flip of
/// the second every 16 steps), `world + 0x2c` and `+0x38` are copied to `+0xb8` and `+0xbc`,
/// `00c910b0` (`checkDeterminism`) runs on the world, and 0 is returned.
pub fn step_begin_st(e: &mut Engine, this: Ptr, queue: u32, delta_time: u32) -> u32 {
    let me = this.addr();
    super::hkpbvagent::profile_start_record(e, 0x010c_cbb0, 0x010c_c5bc);
    e.mem.set_u8(me + SIM_IN_STEP, 1);
    let world = e.mem.u32(me + SIM_WORLD);
    add_to_word(e, world + 0x94, 1);
    let step = x87_quiet_bits(delta_time);
    e.mem.set_u32(me + 0x1c, step);
    let start = e.mem.u32(me + 0x18);
    let end = ((f64::from(f32::from_bits(step)) + x87_load32(start)) as f32).to_bits();
    let info = step_info(start, end);
    e.with_stack(0x20, |e, frame| {
        let frame = frame.addr();
        let (info_at, usage_at) = (frame, frame + 0x10);
        let world = e.mem.u32(me + SIM_WORLD);
        for (lane, bits) in info.into_iter().enumerate() {
            e.mem.set_u32(world + 0x1d0 + 4 * lane as u32, bits);
        }
        let world = e.mem.u32(me + SIM_WORLD);
        let integrator = e.mem.u32(world + 0x78);
        for (lane, bits) in info.into_iter().enumerate() {
            e.mem.set_u32(integrator + 0x50 + 4 * lane as u32, bits);
        }
        for (lane, bits) in info.into_iter().enumerate() {
            e.mem.set_u32(info_at + 4 * lane as u32, bits);
        }
        let world = e.mem.u32(me + SIM_WORLD);
        let owner = e.mem.u32(world + 0x4c);
        e.vcall(owner, 0x14, &args![world, info_at]);
        let world = e.mem.u32(me + SIM_WORLD);
        e.call(0x00cb_55d0, &args![world]);
        let world = e.mem.u32(me + SIM_WORLD);
        e.call(0x00c9_2b50, &args![world, usage_at]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let guard = e.mem.u32(router + 0x18);
        let usage = e.mem.u32(usage_at);
        let enough = e.vcall(guard, 0x20, &args![usage]).u32() as u8 != 0;
        if !enough {
            let world = e.mem.u32(me + SIM_WORLD);
            let watchdog = e.call(0x00c9_3570, &args![world]).u32();
            if watchdog == 0 {
                let world = e.mem.u32(me + SIM_WORLD);
                if e.mem.u8(world + 0x54) != 0 {
                    panic!(
                        "INT3 at 00d026c3 (no memory watchdog and the world byte at +0x54 is set)"
                    );
                }
                e.mem.set_u32(me + 0x28, 1);
                return 1;
            }
            let world = e.mem.u32(me + SIM_WORLD);
            leave_world_scope(e, world);
            let world = e.mem.u32(me + SIM_WORLD);
            e.call(0x00d0_1bc0, &args![world]);
            let world = e.mem.u32(me + SIM_WORLD);
            add_to_word(e, world + 0x94, 1);
        }
        e.call(0x00d8_92d0, &args![]);
        let world = e.mem.u32(me + SIM_WORLD);
        for (lane, bits) in info.into_iter().enumerate() {
            e.mem.set_u32(world + 0x1d0 + 4 * lane as u32, bits);
        }
        let world = e.mem.u32(me + SIM_WORLD);
        let scale = (x87_load32(e.mem.u32(world + 0x300)) * x87_load32(info[2])) as f32;
        e.mem.set_f32(world + 0x2ec, scale);
        let ticks = f64::from(e.mem.i32(world + 0x2f4));
        e.mem
            .set_f32(world + 0x2f0, (ticks * x87_load32(info[3])) as f32);
        let from_scale = e.mem.f32(world + 0x2ec);
        let gravity = super::hkpsphereshape::load_vec(e, world + 0x10);
        super::hkpsphereshape::store_vec(
            e,
            world + 0x1f0,
            std::array::from_fn(|lane| from_scale * gravity[lane]),
        );
        let delta = f32::from_bits(x87_quiet_bits(info[2]));
        let gravity = super::hkpsphereshape::load_vec(e, world + 0x10);
        super::hkpsphereshape::store_vec(
            e,
            world + 0x200,
            std::array::from_fn(|lane| delta * gravity[lane]),
        );
        let world = e.mem.u32(me + SIM_WORLD);
        let query = e.mem.u32(world + 0x1b8);
        e.mem.set_u32(query, 0);
        let world = e.mem.u32(me + SIM_WORLD);
        let state = e.mem.u32(world + 0x74);
        e.mem.set_u32(state + 0x114, world + 0x1d0);
        let integrator = e.mem.u32(world + 0x78);
        e.call(COPY_INTEGRATION_INFO, &args![state + 0x90, integrator]);
        let query = e.mem.u32(world + 0x1b8);
        e.call(
            CONSTRAINT_QUERY_SET,
            &args![state + 0x10, world + 0x1e0, world + 0x1d0, query],
        );
        e.mem.set_u32(state + 0x110, me);
        let world = e.mem.u32(me + SIM_WORLD);
        let integrator = e.mem.u32(world + 0x78);
        let time = x87_quiet_bits(e.mem.u32(integrator + 8));
        e.mem.set_u32(state + 0x11c, POST_COLLIDE_STATE_TABLE);
        e.mem.set_u32(state + 0x118, time);
        let world = e.mem.u32(me + SIM_WORLD);
        e.mem.set_u32(state, world);
        let world = e.mem.u32(me + SIM_WORLD);
        let count = e.mem.i32(world + 0x2c);
        if count > 0 {
            if e.mem.u8(world + 0xb0) != 0 {
                profile_record(e, 0x010c_c5b0);
                let world = e.mem.u32(me + SIM_WORLD);
                add_to_word(e, world + 0x98, -1);
                e.call(0x00cf_8b00, &args![me]);
                let world = e.mem.u32(me + SIM_WORLD);
                add_to_word(e, world + 0x98, 1);
            }
            e.with_stack(0x30, |e, job| {
                let job = job.addr();
                let world = e.mem.u32(me + SIM_WORLD);
                e.mem.set_u8(job, 0);
                e.mem.set_u8(job + 1, 0);
                e.mem.set_u8(job + 2, 2);
                e.mem.set_u16(job + 4, 0x30);
                e.mem.set_u16(job + 6, 0xffff);
                e.mem.set_u16(job + 0x10, 0);
                e.mem.set_u32(job + 0x14, 0);
                e.mem.set_u32(job + 0x18, 0);
                e.mem.set_u32(job + 0x20, count as u32);
                let state = e.mem.u32(world + 0x74);
                e.mem.set_u32(job + 0x1c, state);
                e.call(JOB_QUEUE_PREPARE, &args![queue, 0u32, count as u32]);
                e.call(JOB_QUEUE_PREPARE, &args![queue, 1u32, count as u32]);
                e.call(JOB_QUEUE_ADD_JOB, &args![queue, job, 1u32]);
            });
        }
        let world = e.mem.u32(me + SIM_WORLD);
        let counter = e.mem.u8(world + 0x307).wrapping_add(1);
        e.mem.set_u8(world + 0x307, counter);
        if counter.wrapping_sub(4) & 7 == 0 {
            let flags = e.mem.u8(world + 0x305) ^ 1;
            e.mem.set_u8(world + 0x305, flags);
        }
        if counter & 7 == 0 {
            let flags = e.mem.u8(world + 0x305) ^ 2;
            e.mem.set_u8(world + 0x305, flags);
        }
        if counter & 0xf == 0 {
            let flip = 1u8.wrapping_sub(e.mem.u8(world + 0x306));
            e.mem.set_u8(world + 0x307, 0);
            e.mem.set_u8(world + 0x306, flip);
        }
        let world = e.mem.u32(me + SIM_WORLD);
        let actions = e.mem.u32(world + 0x2c);
        e.mem.set_u32(me + 0xb8, actions);
        let last = e.mem.u32(world + 0x38);
        e.mem.set_u32(me + 0xbc, last);
        e.call(0x00c9_10b0, &args![world]);
        profile_record(e, 0x0107_6f38);
        0
    })
}

// Translated from 00d03030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::processAgentNnEntries` (Ghidra name): `ECX` = simulation and
/// five stack words: `entries` (an array of pointers), `count`, `input` (the collision input,
/// its words `+0x50..+0x5c` are copied), `owner` (its halfword at `+0x20` is copied) and a
/// byte `use_toi_quality`. Without a job queue (`+0xc0`) or with a `count` that is not above
/// the world's limit (`[world + 0xec]`, signed) the entries go to
/// `hkpContinuousSimulation::processAgentNnEntries` (`00d0c9c0`) unchanged. Otherwise one
/// narrow phase job is built on the stack (bytes `0xf`, 1 and 1, halfwords `0x50` and `0xffff`,
/// the world's word `+0x74`, the entries, the count and the sector size `[world + 0xe8]` as
/// halfwords, the input words and the byte). With `count` above 0 a sector header for
/// `(count - 1) / size + 1` sectors is allocated (`00d96530`), the job gets it, the owner's
/// halfword and the type byte 2 and is added to the queue (`00d04b90`, with 1). All jobs are
/// run (`00d051b0`) and, with `count` above 0, a post collide job (bytes `0xe`, 0, 2, halfwords
/// `0x30` and `0xffff`, the owner, the job's words `+0x18`, `+0x1c`, `+0x20` and halfword
/// `+0x10`) is run on the state `run_post_collide_job` builds. The exe's last step gives the
/// entries array back to the allocator (`release_block` with `count * 4` bytes) unless bit 31
/// of its capacity word is set; the word is made from `count | 0x80000000`, so in the exe this
/// release can never run (it is translated and tested through `release_block` alone).
pub fn process_agent_nn_entries(
    e: &mut Engine,
    this: Ptr,
    entries: u32,
    count: i32,
    input: u32,
    owner: u32,
    use_toi_quality: u32,
) {
    let me = this.addr();
    let queue = e.mem.u32(me + SIM_JOB_QUEUE);
    let world = e.mem.u32(me + SIM_WORLD);
    if queue == 0 || count <= e.mem.i32(world + 0xec) {
        e.call(
            CONTINUOUS_PROCESS_ENTRIES,
            &args![me, entries, count, input, owner, use_toi_quality],
        );
        return;
    }
    e.with_stack(0x1a0, |e, frame| {
        let job = frame.addr();
        let post = job + 0x50;
        let state = job + 0x80;
        let sector_size = e.mem.i32(world + 0xe8);
        e.mem.set_u16(job + 4, 0x50);
        e.mem.set_u16(job + 6, 0xffff);
        e.mem.set_u32(job + 0x14, 0);
        e.mem.set_u32(job + 0x18, 0);
        for lane in 0..4 {
            let word = e.mem.u32(input + 0x50 + 4 * lane);
            e.mem.set_u32(job + 0x30 + 4 * lane, word);
        }
        e.mem.set_u16(job + 0x24, 0);
        e.mem.set_u32(job + 0x20, 0);
        e.mem.set_u16(job + 0x10, 0);
        e.mem.set_u8(job + 0x40, use_toi_quality as u8);
        e.mem.set_u8(job, 0xf);
        e.mem.set_u8(job + 2, 1);
        e.mem.set_u32(job + 0x2c, entries);
        e.mem.set_u16(job + 0x28, count as u16);
        e.mem.set_u16(job + 0x2a, sector_size as u16);
        e.mem.set_u8(job + 1, 1);
        let world_state = e.mem.u32(world + 0x74);
        e.mem.set_u32(job + 0x1c, world_state);
        if count > 0 {
            let sectors = (count - 1) / sector_size + 1;
            let header = e
                .call(SECTOR_HEADER_ALLOCATE, &args![sectors, sector_size])
                .u32();
            e.mem.set_u32(job + 0x20, header);
            let flags = e.mem.u16(owner + 0x20);
            e.mem.set_u16(job + 0x10, flags);
            e.mem.set_u8(job + 2, 2);
            let queue = e.mem.u32(me + SIM_JOB_QUEUE);
            e.call(JOB_QUEUE_ADD_JOB, &args![queue, job, 1u32]);
        }
        let queue = e.mem.u32(me + SIM_JOB_QUEUE);
        e.call(JOB_QUEUE_PROCESS_ALL, &args![queue]);
        if count > 0 {
            e.mem.set_u16(post + 6, 0xffff);
            e.mem.set_u16(post + 4, 0x30);
            let word = e.mem.u32(job + 0x18);
            e.mem.set_u32(post + 0x18, word);
            let word = e.mem.u32(job + 0x1c);
            e.mem.set_u32(post + 0x1c, word);
            e.mem.set_u8(post + 1, 0);
            let flags = e.mem.u16(job + 0x10);
            e.mem.set_u16(post + 0x10, flags);
            let word = e.mem.u32(job + 0x20);
            e.mem.set_u32(post + 0x20, word);
            e.mem.set_u32(post + 0x14, owner);
            e.mem.set_u8(post, 0xe);
            e.mem.set_u8(post + 2, 2);
            run_post_collide_job(e, me, state, post);
        }
        release_block(e, entries, count as u32 | DONT_DEALLOCATE_FLAG, 4);
    });
}

// Translated from 00d02930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::processAgentNnEntriesFromMultipleIslands` (Ghidra name): `ECX` =
/// simulation and six stack words: `base` (the entries array), `total` (the number of entries),
/// `records` (an array of `0x18`-byte records: the island at `+0`, the number of entries at
/// `+4`, the index of the first at `+8`, the job pointer is stored at `+0x10`), `record_count`,
/// `input` (as for [`process_agent_nn_entries`]) and `use_toi_quality`. Without a job queue or
/// with a `total` not above the world's limit (`[world + 0xec]`) every record goes to
/// `hkpContinuousSimulation::processAgentNnEntries` (`00d0c9c0`) as `(base + first * 4,
/// number, input, island, use_toi_quality)`. Otherwise `((record_count * 0x50 + 0x7f) & !0x7f)`
/// bytes are taken off the router's temporary stack (word `+0`) for one `0x50`-byte job per
/// record. For each record whose island is not the world's word `+0x20` (the last one that is
/// is remembered) a narrow phase job is made as in [`process_agent_nn_entries`] (with the
/// record's entries and number) and, with entries, given a sector header, added to the queue
/// and stored in the record; all jobs run (`00d051b0`); each record that has a job gets a
/// post collide job (`run_post_collide_job`), and the remembered record, if any, is run
/// directly through `00d0c9c0` (its fifth word is 0 here; the exe leaves the upper bits of
/// whatever `ECX` held, which the callee does not read). The temporary stack is given back.
pub fn process_agent_nn_entries_from_multiple_islands(
    e: &mut Engine,
    this: Ptr,
    base: u32,
    total: i32,
    records: u32,
    record_count: i32,
    input: u32,
    use_toi_quality: u32,
) {
    let me = this.addr();
    let queue = e.mem.u32(me + SIM_JOB_QUEUE);
    let world = e.mem.u32(me + SIM_WORLD);
    if queue == 0 || total <= e.mem.i32(world + 0xec) {
        for index in 0..record_count.max(0) as u32 {
            let record = records + index * 0x18;
            let first = e.mem.u32(record + 8);
            let entries = base.wrapping_add(first.wrapping_mul(4));
            let (count, island) = (e.mem.u32(record + 4), e.mem.u32(record));
            e.call(
                CONTINUOUS_PROCESS_ENTRIES,
                &args![me, entries, count, input, island, use_toi_quality],
            );
        }
        return;
    }
    let sector_size = e.mem.i32(world + 0xe8);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let top = e.mem.u32(router);
    let rounded = (record_count as u32).wrapping_mul(0x50).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, top.wrapping_add(rounded));
    let mut remembered = 0u32;
    for index in 0..record_count.max(0) as u32 {
        let record = records + index * 0x18;
        let world = e.mem.u32(me + SIM_WORLD);
        if e.mem.u32(record) == e.mem.u32(world + 0x20) {
            remembered = record;
            continue;
        }
        let first = e.mem.u32(record + 8);
        let count = e.mem.i32(record + 4);
        let job = top.wrapping_add(index.wrapping_mul(0x50));
        if job != 0 {
            e.mem.set_u8(job, 0xf);
            e.mem.set_u8(job + 1, 1);
            e.mem.set_u8(job + 2, 1);
            e.mem.set_u16(job + 4, 0x50);
            e.mem.set_u16(job + 6, 0xffff);
            e.mem.set_u32(job + 0x14, 0);
            e.mem.set_u32(job + 0x18, 0);
            for lane in 0..4 {
                let word = e.mem.u32(input + 0x50 + 4 * lane);
                e.mem.set_u32(job + 0x30 + 4 * lane, word);
            }
            e.mem
                .set_u32(job + 0x2c, base.wrapping_add(first.wrapping_mul(4)));
            e.mem.set_u16(job + 0x10, 0);
            e.mem.set_u16(job + 0x24, 0);
            e.mem.set_u32(job + 0x20, 0);
            e.mem.set_u16(job + 0x28, count as u16);
            e.mem.set_u16(job + 0x2a, sector_size as u16);
            e.mem.set_u8(job + 0x40, use_toi_quality as u8);
        }
        let world = e.mem.u32(me + SIM_WORLD);
        let world_state = e.mem.u32(world + 0x74);
        e.mem.set_u32(job.wrapping_add(0x1c), world_state);
        if count > 0 {
            let sectors = (count - 1) / sector_size + 1;
            let header = e
                .call(SECTOR_HEADER_ALLOCATE, &args![sectors, sector_size])
                .u32();
            e.mem.set_u32(job + 0x20, header);
            e.mem.set_u8(job + 2, 2);
            let island = e.mem.u32(record);
            let flags = e.mem.u16(island + 0x20);
            e.mem.set_u16(job + 0x10, flags);
            let queue = e.mem.u32(me + SIM_JOB_QUEUE);
            e.call(JOB_QUEUE_ADD_JOB, &args![queue, job, 1u32]);
            e.mem.set_u32(record + 0x10, job);
        }
    }
    let queue = e.mem.u32(me + SIM_JOB_QUEUE);
    e.call(JOB_QUEUE_PROCESS_ALL, &args![queue]);
    e.with_stack(0x150, |e, frame| {
        let post = frame.addr();
        let state = post + 0x30;
        for index in 0..record_count.max(0) as u32 {
            let record = records + index * 0x18;
            let job = e.mem.u32(record + 0x10);
            if job == 0 {
                continue;
            }
            e.mem.set_u8(post, 0xe);
            e.mem.set_u8(post + 1, 0);
            e.mem.set_u8(post + 2, 2);
            e.mem.set_u16(post + 6, 0xffff);
            e.mem.set_u16(post + 4, 0x30);
            let flags = e.mem.u16(job + 0x10);
            e.mem.set_u16(post + 0x10, flags);
            let word = e.mem.u32(job + 0x18);
            e.mem.set_u32(post + 0x18, word);
            let word = e.mem.u32(job + 0x1c);
            e.mem.set_u32(post + 0x1c, word);
            let word = e.mem.u32(job + 0x20);
            e.mem.set_u32(post + 0x20, word);
            let island = e.mem.u32(record);
            e.mem.set_u32(post + 0x14, island);
            run_post_collide_job(e, me, state, post);
        }
    });
    if remembered != 0 {
        let first = e.mem.u32(remembered + 8);
        let entries = base.wrapping_add(first.wrapping_mul(4));
        let (count, island) = (e.mem.u32(remembered + 4), e.mem.u32(remembered));
        e.call(
            CONTINUOUS_PROCESS_ENTRIES,
            &args![me, entries, count, input, island, 0u32],
        );
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, top);
}

/// `hkpLinkedCollidable::getCollisionEntriesSorted` (`00d08260`, `this` = the linked collidable
/// at `entity + 0x10`, one stack word: the array to fill, an `hkArray` of pairs of words) and
/// the functions of the hash maps (`hkMapBase<unsigned long, unsigned long>`, `this` = the map)
/// the narrow phase functions use: `getWithDefault` (`00cb1930`, `(key, default)`), `insert`
/// (`00cb1870`, `(allocator, key, value)`), `reserve` (`00cb1fe0`, `(allocator, 0)`),
/// `clearAndDeallocate` (`00cb1a90`, `(allocator)`) and `00cb1a60` (no stack words; it is
/// unconfirmed what it does).
const GET_COLLISION_ENTRIES_SORTED: u32 = 0x00d0_8260;
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
const MAP_INSERT: u32 = 0x00cb_1870;
const MAP_RESERVE: u32 = 0x00cb_1fe0;
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
const MAP_CLEAR_UNCONFIRMED: u32 = 0x00cb_1a60;
/// `hkReferencedObject::addReference` (`00c90510`) and the call the narrow phase makes on each
/// island entity at the end of its scan (`00c905b0`, a release; unconfirmed).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const RELEASE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkString::memSet` (`00cb1280`, cdecl `(destination, value, size)`).
const MEM_SET: u32 = 0x00cb_1280;
/// Gives a block of `capacity & 0x3fffffff` elements of `element_size` bytes at `data` back to
/// the heap allocator (slot `0x10`, `(data, bytes)`) unless bit 31 of the capacity word
/// `capacity` is set. The allocator is fetched from the router first, as the exe does.
fn release_block(e: &mut Engine, data: u32, capacity: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    if capacity & DONT_DEALLOCATE_FLAG == 0 {
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK).wrapping_mul(element_size)],
        );
    }
}

/// Gives the storage of an `hkArray` (data, size, capacity word at `array`) back to the heap
/// allocator (slot `0x10`, `(data, capacity * element size)`) unless bit 31 of its capacity word
/// is set, and leaves it empty (size 0, data null, capacity word `0x80000000`).
fn empty_array(e: &mut Engine, array: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & DONT_DEALLOCATE_FLAG == 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK).wrapping_mul(element_size)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, DONT_DEALLOCATE_FLAG);
}

/// Finds the record of `island` (`0x18` bytes, the island at `+0`) in the array of records at
/// `groups` or appends a zeroed one with the island set (growing the array with `00c90940`
/// first when it is full) and returns its address.
fn island_group(e: &mut Engine, groups: u32, island: u32) -> u32 {
    let size = e.mem.i32(groups + 4);
    for index in 0..size.max(0) as u32 {
        let record = e.mem.u32(groups) + index * 0x18;
        if island == e.mem.u32(record) {
            return record;
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    if e.mem.u32(groups + 4) == e.mem.u32(groups + 8) & CAPACITY_MASK {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, groups, 0x18u32]);
    }
    let size = e.mem.u32(groups + 4);
    let record = e.mem.u32(groups).wrapping_add(size.wrapping_mul(0x18));
    if record != 0 {
        for word in 0..6 {
            e.mem.set_u32(record + 4 * word, 0);
        }
    }
    e.mem.set_u32(groups + 4, size.wrapping_add(1));
    let record = e.mem.u32(groups).wrapping_add(size.wrapping_mul(0x18));
    if record != 0 {
        for word in 0..6 {
            e.mem.set_u32(record + 4 * word, 0);
        }
    }
    e.mem.set_u32(record, island);
    record
}

// Translated from 00d03970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::collideEntitiesOfOneIslandNarrowPhaseContinuous_toiOnly` (Ghidra
/// name): `ECX` = simulation and four stack words: `entities` (an array of pointers), `count`,
/// `input` and `references` (a hash map). Without a job queue (`+0xc0`) the work is left to
/// `00d0db40` with `(entities, count, input, this + 0x3c)` (the exe jumps there). Otherwise the
/// island of the first entity (`[entity + 0xcc]`) gives the number of bytes (its word `+0x4c`)
/// of a flag block taken off the router's temporary stack and cleared (`00cb1280`), and a
/// list of entries gets `0x1000` bytes of it (capacity `0x3e8`). Every entity sets the flag
/// indexed by its halfword at `+0xa8` and has its sorted collision entries
/// (`00d08260` on `entity + 0x10`) scanned; an entry is skipped when the object of its second
/// word (that plus the signed byte at `+0x10`) is in the same island and flagged. Entries
/// whose type byte (`+0xc`, signed, times `0x40`, in the dispatcher table at `[world + 0x80]`)
/// has a word at `+0x1c30` that is not 0 and a byte at `+0x1c34` that is 0 are appended to the
/// list; all others make the entity be looked up in `references` by its word at `+0xd4`
/// (`00cb1930`) and, if it is not there, referenced (`00c90510`) and stored (`00cb1870` with
/// the router's allocator). With entries in the list [`process_agent_nn_entries`] runs with
/// `(list, count, input, island, 0)`; the arrays and the flag block are given back. The exe's
/// last release, of the flag block (`release_block` with one byte per element), is skipped when
/// bit 31 of its capacity word is set; the word is made from the block size `| 0x80000000`, so
/// in the exe this release can never run (it is translated and tested through `release_block`
/// alone).
pub fn collide_entities_of_one_island_narrow_phase_continuous_toi_only(
    e: &mut Engine,
    this: Ptr,
    entities: u32,
    count: i32,
    input: u32,
    references: u32,
) {
    let me = this.addr();
    if e.mem.u32(me + SIM_JOB_QUEUE) == 0 {
        e.call(0x00d0_db40, &args![me, entities, count, input, me + 0x3c]);
        return;
    }
    let first = e.mem.u32(entities);
    let island = e.mem.u32(first + 0xcc);
    let size = e.mem.u32(island + 0x4c);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let flags = e.mem.u32(router);
    e.mem.set_u32(
        router,
        flags.wrapping_add(size.wrapping_add(0x7f) & 0xffff_ff80),
    );
    e.call(MEM_SET, &args![flags, 0u32, size]);
    e.with_stack(0x20, |e, frame| {
        let frame = frame.addr();
        let (collisions, list) = (frame, frame + 0x10);
        e.mem.set_u32(list, 0);
        e.mem.set_u32(list + 4, 0);
        e.mem.set_u32(list + 8, DONT_DEALLOCATE_FLAG);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let list_top = e.mem.u32(router);
        e.mem.set_u32(router, list_top.wrapping_add(0x1000));
        e.mem.set_u32(list, list_top);
        e.mem.set_u32(list + 8, 0x8000_03e8);
        e.mem.set_u32(collisions, 0);
        e.mem.set_u32(collisions + 4, 0);
        e.mem.set_u32(collisions + 8, DONT_DEALLOCATE_FLAG);
        for index in 0..count.max(0) as u32 {
            let entity = e.mem.u32(entities + index * 4);
            let flag_at = flags.wrapping_add(u32::from(e.mem.u16(entity + 0xa8)));
            e.mem.set_u8(flag_at, 1);
            e.call(
                GET_COLLISION_ENTRIES_SORTED,
                &args![entity + 0x10, collisions],
            );
            let mut position = 0i32;
            while position < e.mem.i32(collisions + 4) {
                let data = e.mem.u32(collisions);
                let entry = e.mem.u32(data + position as u32 * 8);
                let other = e.mem.u32(data + position as u32 * 8 + 4);
                let other = other.wrapping_add(signed_byte(e, other + 0x10));
                position += 1;
                if e.mem.u32(other + 0xcc) == island
                    && e.mem
                        .u8(flags.wrapping_add(u32::from(e.mem.u16(other + 0xa8))))
                        != 0
                {
                    continue;
                }
                let world = e.mem.u32(me + SIM_WORLD);
                let table = e.mem.u32(world + 0x80);
                let kind = signed_byte(e, entry + 0xc)
                    .wrapping_shl(6)
                    .wrapping_add(table);
                if e.mem.u32(kind + 0x1c30) != 0 && e.mem.u8(kind + 0x1c34) == 0 {
                    let router = e.call(MEMORY_ROUTER, &args![]).u32();
                    if e.mem.u32(list + 4) == e.mem.u32(list + 8) & CAPACITY_MASK {
                        let allocator = e.mem.u32(router + 0x10);
                        e.call(ARRAY_GROW, &args![allocator, list, 4u32]);
                    }
                    let used = e.mem.u32(list + 4);
                    let data = e.mem.u32(list);
                    e.mem
                        .set_u32(data.wrapping_add(used.wrapping_mul(4)), entry);
                    e.mem.set_u32(list + 4, used.wrapping_add(1));
                } else {
                    let key = e.mem.u32(entity + 0xd4);
                    let present = e
                        .call(MAP_GET_WITH_DEFAULT, &args![references, key, 0u32])
                        .u32();
                    if present == 0 {
                        e.call(ADD_REFERENCE, &args![entity]);
                        let key = e.mem.u32(entity + 0xd4);
                        let router = e.call(MEMORY_ROUTER, &args![]).u32();
                        let allocator = e.mem.u32(router + 0x10);
                        e.call(MAP_INSERT, &args![references, allocator, key, entity]);
                    }
                }
            }
        }
        let used = e.mem.u32(list + 4);
        if used != 0 {
            let data = e.mem.u32(list);
            process_agent_nn_entries(e, this, data, used as i32, input, island, 0);
        }
        empty_array(e, collisions, 8);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        if e.mem.u32(list) == list_top {
            e.mem.set_u32(list + 4, 0);
        }
        e.mem.set_u32(router, list_top);
        empty_array(e, list, 4);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, flags);
        release_block(e, flags, size | DONT_DEALLOCATE_FLAG, 1);
    });
}

// Translated from 00d03c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::collideEntitiesNeedingPsiCollisionDetectionNarrowPhase_toiOnly`
/// (Ghidra name): `ECX` = simulation and two stack words: the first is not used, the second is
/// `entities`, a hash map from keys to entity pointers (data `+0`, 8-byte slots with key
/// `0xffffffff` for an empty one, highest slot index `+8`). Between the profiling records
/// `"TtRecollide PSI"` and `010c41f8`: takes one on the world's word `+0x94` and makes a local
/// map (reserved with 0 through the router's allocator). For every entity in `entities` its
/// sorted collision entries (`00d08260` on `entity + 0x10`) are scanned; an entry that is not
/// in the local map (`00cb1930`) and whose type has no word at `+0x1c30` in the dispatcher
/// table `[world + 0x80]` or has a byte at `+0x1c34` set while the float at `entity + 0x14c`
/// is 0 is put in it (value 1, `00cb1870`), appended to a list of `(entry, island)` pairs
/// (island: the word `+0xcc` of the entity, or for a type byte 5 at `+0xe8` that of the object
/// found by XOR-ing the objects of the entry's words `+0x10` and `+0x14` with the entity), and
/// counted in the `0x18`-byte record of that island (`island_group`: island `+0`, count `+4`).
/// The entity is released (`00c905b0`) after its scan. The map `entities` is then cleaned up
/// (`00cb1a60`), each record gets the start `+8` and the cursor `+0xc` of its slice of one
/// block (4 bytes per entry, taken off the router's temporary stack), every pair stores its
/// entry at its island's cursor, and
/// [`process_agent_nn_entries_from_multiple_islands`] runs on the block with `[world + 0x78]`
/// as the input and 0 as the last word. The world's counter is taken down as at the start
/// (`leave_world_scope`), `010c41f8` is recorded, the temporary stack and the arrays are given
/// back and the local map is cleared (`00cb1a90`, then the bare `00a29680`).
pub fn collide_entities_needing_psi_collision_detection_narrow_phase_toi_only(
    e: &mut Engine,
    this: Ptr,
    _unused: u32,
    entities: u32,
) {
    let me = this.addr();
    profile_record(e, 0x010c_cd50);
    let world = e.mem.u32(me + SIM_WORLD);
    add_to_word(e, world + 0x94, 1);
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        let (map, pairs, groups, collisions) = (frame, frame + 0xc, frame + 0x18, frame + 0x24);
        e.mem.set_u32(map, 0);
        e.mem.set_u32(map + 4, 0);
        e.mem.set_u32(map + 8, 0xffff_ffff);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
        for array in [pairs, groups] {
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, DONT_DEALLOCATE_FLAG);
        }
        let mut total = 0u32;
        let mut slot = 0i32;
        let skip_empty = |e: &Engine, slot: &mut i32| {
            if *slot <= e.mem.i32(entities + 8) {
                loop {
                    let data = e.mem.u32(entities);
                    if e.mem.u32(data + *slot as u32 * 8) != 0xffff_ffff {
                        break;
                    }
                    *slot += 1;
                    if *slot > e.mem.i32(entities + 8) {
                        break;
                    }
                }
            }
        };
        skip_empty(e, &mut slot);
        while slot <= e.mem.i32(entities + 8) {
            let data = e.mem.u32(entities);
            let entity = e.mem.u32(data + slot as u32 * 8 + 4);
            e.mem.set_u32(collisions, 0);
            e.mem.set_u32(collisions + 4, 0);
            e.mem.set_u32(collisions + 8, DONT_DEALLOCATE_FLAG);
            e.call(
                GET_COLLISION_ENTRIES_SORTED,
                &args![entity + 0x10, collisions],
            );
            let count = e.mem.i32(collisions + 4);
            for position in 0..count.max(0) as u32 {
                let entry = e.mem.u32(e.mem.u32(collisions) + position * 8);
                let present = e.call(MAP_GET_WITH_DEFAULT, &args![map, entry, 0u32]).u32();
                if present != 0 {
                    continue;
                }
                let world = e.mem.u32(me + SIM_WORLD);
                let table = e.mem.u32(world + 0x80);
                let kind = signed_byte(e, entry + 0xc)
                    .wrapping_shl(6)
                    .wrapping_add(table);
                let unlimited = e.mem.u32(kind + 0x1c30) == 0;
                let flagged = e.mem.u8(kind + 0x1c34) != 0;
                let at_rest = e.mem.f32(entity + 0x14c) == 0.0;
                if !(unlimited || (flagged && at_rest)) {
                    continue;
                }
                let mut island = e.mem.u32(entity + 0xcc);
                if e.mem.u8(entity + 0xe8) == 5 {
                    let second = e.mem.u32(entry + 0x14);
                    let second = second.wrapping_add(signed_byte(e, second + 0x10));
                    let first = e.mem.u32(entry + 0x10);
                    let first = first.wrapping_add(signed_byte(e, first + 0x10));
                    island = e.mem.u32((second ^ first ^ entity).wrapping_add(0xcc));
                }
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let allocator = e.mem.u32(router + 0x10);
                e.call(MAP_INSERT, &args![map, allocator, entry, 1u32]);
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                if e.mem.u32(pairs + 4) == e.mem.u32(pairs + 8) & CAPACITY_MASK {
                    let allocator = e.mem.u32(router + 0x10);
                    e.call(ARRAY_GROW, &args![allocator, pairs, 8u32]);
                }
                let used = e.mem.u32(pairs + 4);
                let at = e.mem.u32(pairs).wrapping_add(used.wrapping_mul(8));
                e.mem.set_u32(pairs + 4, used.wrapping_add(1));
                e.mem.set_u32(at, entry);
                e.mem.set_u32(at + 4, island);
                let group = island_group(e, groups, island);
                let members = e.mem.u32(group + 4).wrapping_add(1);
                e.mem.set_u32(group + 4, members);
                total = total.wrapping_add(1);
            }
            e.call(RELEASE_REFERENCE, &args![entity]);
            slot += 1;
            skip_empty(e, &mut slot);
            empty_array(e, collisions, 8);
        }
        e.call(MAP_CLEAR_UNCONFIRMED, &args![entities]);
        let mut cursor = 0u32;
        for index in 0..e.mem.i32(groups + 4).max(0) as u32 {
            let group = e.mem.u32(groups) + index * 0x18;
            e.mem.set_u32(group + 8, cursor);
            e.mem.set_u32(group + 0xc, cursor);
            cursor = cursor.wrapping_add(e.mem.u32(group + 4));
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let block = e.mem.u32(router);
        let rounded = total.wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80;
        e.mem.set_u32(router, block.wrapping_add(rounded));
        for index in 0..e.mem.i32(pairs + 4).max(0) as u32 {
            let at = e.mem.u32(pairs) + index * 8;
            let (entry, island) = (e.mem.u32(at), e.mem.u32(at + 4));
            let group = island_group(e, groups, island);
            let place = e.mem.u32(group + 0xc);
            e.mem.set_u32(group + 0xc, place.wrapping_add(1));
            e.mem
                .set_u32(block.wrapping_add(place.wrapping_mul(4)), entry);
        }
        let world = e.mem.u32(me + SIM_WORLD);
        let integrator = e.mem.u32(world + 0x78);
        let record_data = e.mem.u32(groups);
        let record_count = e.mem.i32(groups + 4);
        process_agent_nn_entries_from_multiple_islands(
            e,
            this,
            block,
            total as i32,
            record_data,
            record_count,
            integrator,
            0,
        );
        let world = e.mem.u32(me + SIM_WORLD);
        leave_world_scope(e, world);
        profile_record(e, PROFILE_END);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, block);
        empty_array(e, groups, 0x18);
        empty_array(e, pairs, 8);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
        e.call(0x00a2_9680, &args![map]);
    });
}

// Translated from 00d03290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiThreadedSimulation::finishMtStep` (Ghidra name): `ECX` = simulation and two stack
/// words, `queue` (the job queue) and `pool` (the thread pool object, or 0). The exe's stack probe
/// (`00ec8660`, `0x30b4` bytes) is not modelled. Between the profiling records `"TtPhysics"` and
/// the ones named below: `00c91180` runs on the world; with a queue and a pool the queue's state
/// is remembered (`00d04440`), set to 1 and 2 (`00d049d0`, `00d04450`) and the pool is told
/// through its slot `0xc` `(queue, 0xb)`. The islands the world keeps from the index `+0xb8`
/// on (array `[world + 0x28]`, count `+0x2c`) are sorted (`00dd7d00`, comparison `00d02220`)
/// when there are two or more and numbered by their position (halfword `+0x20`);
/// `00c910b0` checks determinism. When the add pairs (`+0x6c`, count `+0x70`) and the remove
/// pairs (`+0x90`, count `+0x94`) are not both empty: the records `"LtInterIsland"` /
/// `"Stduplicates"`, `00cf7130` (duplicates between the two lists), `"StsortPairs"` and
/// [`sort_pair_list`] on both, `"StaddAgt"`; then every add pair (broad phase handles, whose
/// collidables are the handle plus the signed byte at `+5` and whose objects add the signed byte
/// at the collidable's `+0x10`) merges the two islands when they differ (`00cb4c60`), looks up
/// the agent type byte in the dispatcher table of `[world + 0x78]` at `0x1bb0 + type_b + 10 *
/// type_a` (types are the signed bytes at `+0x1a`) and, when it is not 0, adds the agent
/// (`00cc0f40`; the table's creation word `0x1c30 + 0x40 * type` goes to the state at `+0x14`)
/// and runs it (`00cfedd0` over an output block of `0x3090` bytes prepared with the maximum
/// time `0104da48`, the owner island at `+4` and the quality block `0x1c20 + 0x40 * quality`
/// at the state's `+0x60`; the manager's slot `0x18` is called when the output no longer points
/// at its own buffer); a Toi event is added ([`add_toi_event_with_critical_section_lock`], section
/// `this + 0x100`) when the output's time is not the double `0104e138`. The loop stops when the
/// global `01269200` is 1; the add pairs are cleared. `"StremoveAgt"`: every remove pair finds
/// its agent (`00cfdd70`) and removes it with its Toi events (`00cc1160`); the list is cleared;
/// `01076f38`. The in-step byte is cleared, the world's first `min(count, 0x80)` constraint
/// violations (`[[world + 0x1b8]]` is the count, the pointers follow) are reported
/// (`00d00aa0`), the time at `+0x18` grows by the delta at `+0x1c`, the world scope is left
/// (`leave_world_scope`). When the global `01269200` is 1 the result is 2, the time goes back by
/// the delta, `00c91190` runs and `010c41f8` is recorded. Otherwise, when the word at `world +
/// 0x180` is not 0 the post collide callbacks run (`"TtPostCollideCB"`, a step info from the
/// time minus delta to the time, `00d014f0(world, info)`, `010c41f8`); `00c910b0`, `010c41f8`, the
/// queue stored at `+0xc0` while the byte at `world + 0xe4` is set, `00c91190`, the simulation's
/// slot `0x18`, `+0xc0` cleared and result 3 if the global is 1 by now. With a pool (`"TtWaitForWorkerThreads"`)
/// the queue is set to 0 and to the remembered state and the pool's slot `0x10` runs
/// (`010c41f8`). The result goes to `+0x28`; when it is not 0 and a memory watchdog exists
/// (`00c93570`) `00d01df0(world)` runs. Returns the result.
pub fn finish_mt_step(e: &mut Engine, this: Ptr, queue: u32, pool: u32) -> u32 {
    let me = this.addr();
    profile_record(e, 0x010c_c568);
    let world = e.mem.u32(me + SIM_WORLD);
    e.call(0x00c9_1180, &args![world]);
    let mut remembered = 0u32;
    if queue != 0 && pool != 0 {
        remembered = e.call(0x00d0_4440, &args![queue]).u32();
        e.call(0x00d0_49d0, &args![queue, 1u32]);
        e.call(0x00d0_4450, &args![queue, 2u32]);
        e.vcall(pool, 0xc, &args![queue, 0xbu32]);
    }
    let world = e.mem.u32(me + SIM_WORLD);
    let start = e.mem.i32(me + 0xb8);
    let count = e.mem.i32(world + 0x2c).wrapping_sub(start);
    if count > 1 {
        let islands = e.mem.u32(world + 0x28);
        e.call(
            0x00dd_7d00,
            &args![
                islands.wrapping_add((start as u32).wrapping_mul(4)),
                0u32,
                (count - 1) as u32,
                0x00d0_2220u32
            ],
        );
    }
    let mut index = e.mem.i32(me + 0xb8);
    while index < e.mem.i32(e.mem.u32(me + SIM_WORLD) + 0x2c) {
        let world = e.mem.u32(me + SIM_WORLD);
        let island = e.mem.u32(e.mem.u32(world + 0x28) + index as u32 * 4);
        e.mem.set_u16(island + 0x20, index as u16);
        index += 1;
    }
    let world = e.mem.u32(me + SIM_WORLD);
    e.call(0x00c9_10b0, &args![world]);
    let pairs = e
        .mem
        .i32(me + SIM_REMOVE_PAIRS + 4)
        .wrapping_add(e.mem.i32(me + SIM_ADD_PAIRS + 4));
    if pairs != 0 {
        finish_pairs(e, this);
    }
    e.mem.set_u8(me + SIM_IN_STEP, 0);
    let world = e.mem.u32(me + SIM_WORLD);
    let violations = e.mem.u32(world + 0x1b8);
    let mut reported = e.mem.i32(violations);
    if reported >= 0x80 {
        reported = 0x80;
    }
    for index in 0..reported.max(0) as u32 {
        let world = e.mem.u32(me + SIM_WORLD);
        let violation = e.mem.u32(e.mem.u32(world + 0x1b8) + index * 4 + 4);
        e.call(0x00d0_0aa0, &args![world, violation]);
    }
    let time = x87_load32(e.mem.u32(me + 0x1c)) + x87_load32(e.mem.u32(me + 0x18));
    e.mem.set_f32(me + 0x18, time as f32);
    let world = e.mem.u32(me + SIM_WORLD);
    leave_world_scope(e, world);
    let world = e.mem.u32(me + SIM_WORLD);
    let mut result = 0u32;
    if e.global::<u32>(STOP_FLAG) == 1 {
        result = 2;
        let time = x87_load32(e.mem.u32(me + 0x18)) - x87_load32(e.mem.u32(me + 0x1c));
        e.mem.set_f32(me + 0x18, time as f32);
        e.call(0x00c9_1190, &args![world]);
        profile_record(e, PROFILE_END);
    } else {
        if e.mem.u32(world + 0x180) != 0 {
            profile_record(e, 0x010c_c530);
            let start = ((x87_load32(e.mem.u32(me + 0x18)) - x87_load32(e.mem.u32(me + 0x1c)))
                as f32)
                .to_bits();
            let info = step_info(start, e.mem.u32(me + 0x18));
            e.with_stack(0x10, |e, info_at| {
                let info_at = info_at.addr();
                for (lane, bits) in info.into_iter().enumerate() {
                    e.mem.set_u32(info_at + 4 * lane as u32, bits);
                }
                let world = e.mem.u32(me + SIM_WORLD);
                e.call(0x00d0_14f0, &args![world, info_at]);
            });
            profile_record(e, PROFILE_END);
        }
        let world = e.mem.u32(me + SIM_WORLD);
        e.call(0x00c9_10b0, &args![world]);
        profile_record(e, PROFILE_END);
        let world = e.mem.u32(me + SIM_WORLD);
        if e.mem.u8(world + 0xe4) != 0 {
            e.mem.set_u32(me + SIM_JOB_QUEUE, queue);
        }
        e.call(0x00c9_1190, &args![world]);
        e.vcall(me, 0x18, &args![]);
        e.mem.set_u32(me + SIM_JOB_QUEUE, 0);
        if e.global::<u32>(STOP_FLAG) == 1 {
            result = 3;
        }
    }
    if pool != 0 {
        profile_record(e, 0x010c_ccf4);
        e.call(0x00d0_49d0, &args![queue, 0u32]);
        e.call(0x00d0_4450, &args![queue, remembered]);
        e.vcall(pool, 0x10, &args![]);
        profile_record(e, PROFILE_END);
    }
    e.mem.set_u32(me + 0x28, result);
    if result != 0 {
        let world = e.mem.u32(me + SIM_WORLD);
        if e.call(0x00c9_3570, &args![world]).u32() != 0 {
            let world = e.mem.u32(me + SIM_WORLD);
            e.call(0x00d0_1df0, &args![world]);
        }
    }
    e.mem.u32(me + 0x28)
}

/// The block of [`finish_mt_step`] that runs when there are pending add or remove pairs.
fn finish_pairs(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    super::hkpbvagent::profile_start_record(e, 0x010c_cd40, 0x010c_cd30);
    e.call(
        0x00cf_7130,
        &args![me + SIM_ADD_PAIRS, me + SIM_REMOVE_PAIRS],
    );
    profile_record(e, 0x010c_cd24);
    sort_pair_list(e, me + SIM_ADD_PAIRS);
    sort_pair_list(e, me + SIM_REMOVE_PAIRS);
    profile_record(e, 0x010c_cd18);
    e.with_stack(0x30a0, |e, out| {
        let out = out.addr();
        let buffer = out + 0x10;
        let threshold = x87_quiet_bits(e.mem.u32(MAX_TIME_FLOAT));
        e.mem.set_u32(out + 0x3030, threshold);
        e.mem.set_u32(out + 0x3050, 0);
        e.mem.set_u32(out + 0x3054, 0);
        e.mem.set_u32(out + 4, 0);
        let mut index = 0u32;
        if e.mem.i32(me + SIM_ADD_PAIRS + 4) > 0 {
            loop {
                let pair = e.mem.u32(me + SIM_ADD_PAIRS) + index * 8;
                let (first, second) = (e.mem.u32(pair), e.mem.u32(pair + 4));
                let collidable_b = second.wrapping_add(signed_byte(e, second + 5));
                let collidable_a = first.wrapping_add(signed_byte(e, first + 5));
                let object_b = collidable_b.wrapping_add(signed_byte(e, collidable_b + 0x10));
                let object_a = collidable_a.wrapping_add(signed_byte(e, collidable_a + 0x10));
                let island_b = e.mem.u32(object_b + 0xcc);
                let mut island_a = e.mem.u32(object_a + 0xcc);
                if island_a != island_b {
                    let world = e.mem.u32(me + SIM_WORLD);
                    e.call(0x00cb_4c60, &args![world, island_a, island_b]);
                    island_a = e.mem.u32(object_a + 0xcc);
                }
                e.mem.set_u32(out + 4, island_a);
                let world = e.mem.u32(me + SIM_WORLD);
                let holder = e.mem.u32(world + 0x78);
                let dispatcher = e.mem.u32(holder);
                let agent_type = e.mem.u8(signed_byte(e, collidable_b + 0x1a)
                    .wrapping_add(dispatcher)
                    .wrapping_add(signed_byte(e, collidable_a + 0x1a).wrapping_mul(10))
                    .wrapping_add(0x1bb0));
                if agent_type != 0 {
                    let creation = signed_byte_of(agent_type)
                        .wrapping_shl(6)
                        .wrapping_add(dispatcher)
                        .wrapping_add(0x1c30);
                    let creation = e.mem.u32(creation);
                    e.mem.set_u32(holder + 0x14, creation);
                    let agent = e
                        .call(ADD_AGENT, &args![collidable_a, collidable_b, holder])
                        .u32();
                    if agent != 0 {
                        run_new_agent(e, this, out, buffer, agent);
                    }
                }
                if e.global::<u32>(STOP_FLAG) == 1 {
                    break;
                }
                index += 1;
                if index as i32 >= e.mem.i32(me + SIM_ADD_PAIRS + 4) {
                    break;
                }
            }
        }
        e.mem.set_u32(me + SIM_ADD_PAIRS + 4, 0);
    });
    profile_record(e, 0x010c_cd0c);
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(me + SIM_REMOVE_PAIRS + 4) {
        let pair = e.mem.u32(me + SIM_REMOVE_PAIRS) + index * 8;
        let (first, second) = (e.mem.u32(pair), e.mem.u32(pair + 4));
        let collidable_a = first.wrapping_add(signed_byte(e, first + 5));
        let collidable_b = second.wrapping_add(signed_byte(e, second + 5));
        let agent = e.call(FIND_AGENT, &args![collidable_a, collidable_b]).u32();
        if agent != 0 {
            e.call(0x00cc_1160, &args![agent]);
        }
        index += 1;
    }
    e.mem.set_u32(me + SIM_REMOVE_PAIRS + 4, 0);
    profile_record(e, 0x0107_6f38);
}

/// The float `0104da48` (the maximum time, `3.40282e+38`) and the double `0104e138` the Toi
/// event check compares the output's time with.
const MAX_TIME_FLOAT: u32 = 0x0104_da48;
const MAX_TIME_DOUBLE: u32 = 0x0104_e138;

/// The sign extension of a byte.
fn signed_byte_of(byte: u8) -> u32 {
    i32::from(byte as i8) as u32
}

/// Runs the agent `agent` just added by [`finish_mt_step`] over the output block `out` (whose
/// buffer starts at `buffer`) and queues a Toi event when it found one.
fn run_new_agent(e: &mut Engine, this: Ptr, out: u32, buffer: u32, agent: u32) {
    let me = this.addr();
    let world = e.mem.u32(me + SIM_WORLD);
    let holder = e.mem.u32(world + 0x78);
    let first = e.mem.u32(agent + 0x10);
    let quality = signed_byte(e, agent + 0xc);
    let second = e.mem.u32(agent + 0x14);
    let dispatcher = e.mem.u32(holder);
    let block = quality
        .wrapping_shl(6)
        .wrapping_add(dispatcher)
        .wrapping_add(0x1c20);
    e.mem.set_u32(holder + 0x60, block);
    let word = e.mem.u32(block + 0x10);
    e.mem.set_u32(holder + 0x14, word);
    let threshold = x87_quiet_bits(e.mem.u32(MAX_TIME_FLOAT));
    e.mem.set_u32(out + 0x3030, threshold);
    e.mem.set_u32(out, buffer);
    e.mem.set_u32(out + 0x3080, 0);
    let manager = e.mem.u32(agent + 8);
    e.call(0x00cf_edd0, &args![agent, holder, out, manager]);
    if e.mem.u32(out) != buffer {
        let manager = e.mem.u32(agent + 8);
        e.vcall(manager, 0x18, &args![first, second, holder, out]);
    }
    let time = f64::from(f32::from_bits(x87_quiet_bits(e.mem.u32(out + 0x3030))));
    if time != e.mem.f64(MAX_TIME_DOUBLE) {
        add_toi_event_with_critical_section_lock(e, this, out, agent, me + 0x100);
    }
}

// Translated from 00d02220 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a "less than" comparison for a sort, cdecl, three stack words
/// (nothing popped): `out`, `a` and `b`. Writes at `out` the byte 1 when the word at +0xd4 of the
/// object that the word at +0 of the object at `a + 0x48` points to is below (unsigned) the same
/// word for `b`, else 0. Returns `out`. Ghidra made no function here; the address is pushed at 00d0332e.
pub fn fn_00d02220(e: &mut Engine, out: u32, a: u32, b: u32) -> u32 {
    let a_block = e.mem.u32(a + 0x48);
    let a_object = e.mem.u32(a_block);
    let b_block = e.mem.u32(b + 0x48);
    let b_object = e.mem.u32(b_block);
    let less = e.mem.u32(a_object + 0xd4) < e.mem.u32(b_object + 0xd4);
    e.mem.set_u8(out, u8::from(less));
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d04200,
            hk_algorithm_quick_sort_recursive_hkp_typed_broad_phase_handle_pair(Ptr, i32, i32, u32)
        ),
        entry!(0x00d0_4320, deleting_destructor_00d04320(Ptr, u8) -> Ptr),
        entry!(
            0x00d01fc0,
            mt_phantom_broad_phase_listener_add_collision_pair(Ptr, u32)
        ),
        entry!(
            0x00d02020,
            mt_phantom_broad_phase_listener_remove_collision_pair(Ptr, u32)
        ),
        entry!(
            0x00d02080,
            mt_broad_phase_border_listener_add_collision_pair(Ptr, u32)
        ),
        entry!(
            0x00d020f0,
            mt_broad_phase_border_listener_remove_collision_pair(Ptr, u32)
        ),
        entry!(0x00d02160, fn_00d02160(u32, u32) -> bool),
        entry!(0x00d021e0, get_multithread_config(Ptr, u32)),
        entry!(0x00d021f0, set_multithread_config(Ptr, u32, u32)),
        entry!(
            0x00d02200,
            assert_there_is_no_collision_information_for_agent(Ptr, u32)
        ),
        entry!(
            0x00d02210,
            assert_there_is_no_collision_information_for_entities(Ptr, u32, u32, u32)
        ),
        entry!(0x00d02250, sort_pair_list(u32)),
        entry!(
            0x00d026e0,
            mt_entity_entity_broad_phase_listener_add_collision_pair(Ptr, u32)
        ),
        entry!(
            0x00d027c0,
            mt_entity_entity_broad_phase_listener_remove_collision_pair(Ptr, u32)
        ),
        entry!(
            0x00d02c80,
            hkp_multi_threaded_simulation_construct(Ptr, u32) -> Ptr
        ),
        entry!(0x00d02da0, hkp_multi_threaded_simulation_destruct(Ptr)),
        entry!(
            0x00d02e70,
            add_toi_event_with_critical_section_lock(Ptr, u32, u32, u32)
        ),
        entry!(
            0x00d04300,
            mt_entity_entity_broad_phase_listener_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
        entry!(0x00d022b0, step_begin_st(Ptr, u32, u32) -> u32),
        entry!(
            0x00d02930,
            process_agent_nn_entries_from_multiple_islands(Ptr, u32, i32, u32, i32, u32, u32)
        ),
        entry!(
            0x00d03030,
            process_agent_nn_entries(Ptr, u32, i32, u32, u32, u32)
        ),
        entry!(0x00d03290, finish_mt_step(Ptr, u32, u32) -> u32),
        entry!(
            0x00d03970,
            collide_entities_of_one_island_narrow_phase_continuous_toi_only(
                Ptr, u32, i32, u32, u32
            )
        ),
        entry!(
            0x00d03c20,
            collide_entities_needing_psi_collision_detection_narrow_phase_toi_only(Ptr, u32, u32)
        ),
        entry!(0x00d0_2220, fn_00d02220(u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::super::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d04320_frees_only_when_bit_zero_is_set() {
        check(0x00d0_4320, 0x00d0_2da0);
    }

    const LESS_00D04200: u32 = 0x0090_5000;

    /// A less-than double on the first word of two pairs, counting calls.
    fn sorted_00d04200(keys: &[u32]) -> (Vec<(u32, u32)>, usize) {
        let mut e = Engine::new();
        let calls = Rc::new(RefCell::new(0usize));
        let counter = Rc::clone(&calls);
        e.register_double(LESS_00D04200, move |e, a| {
            *counter.borrow_mut() += 1;
            Ret {
                eax: (e.mem.u32(a[0]) < e.mem.u32(a[1])) as u32,
                ..Ret::default()
            }
        });
        let array = e.mem.alloc(8 * keys.len() as u32);
        for (i, key) in keys.iter().enumerate() {
            e.mem.set_u32(array + 8 * i as u32, *key);
            e.mem.set_u32(array + 8 * i as u32 + 4, 1000 + i as u32);
        }
        e.call(
            0x00d0_4200,
            &args![
                Ptr::<()>::new(array),
                0i32,
                keys.len() as i32 - 1,
                LESS_00D04200
            ],
        );
        let sorted = (0..keys.len() as u32)
            .map(|i| (e.mem.u32(array + 8 * i), e.mem.u32(array + 8 * i + 4)))
            .collect();
        let count = *calls.borrow();
        (sorted, count)
    }

    #[test]
    fn hk_algorithm_quick_sort_recursive_sorts_the_pairs_by_the_comparator() {
        let (sorted, calls) = sorted_00d04200(&[5, 3, 9, 1, 7, 3, 8, 2]);
        let keys: Vec<u32> = sorted.iter().map(|pair| pair.0).collect();
        assert_eq!(keys, vec![1, 2, 3, 3, 5, 7, 8, 9]);
        assert!(calls > 0);
        // Each pair kept its second word with its key.
        let second_of_nine = sorted.iter().find(|pair| pair.0 == 9).unwrap().1;
        assert_eq!(second_of_nine, 1002);
        let (single, calls) = sorted_00d04200(&[4]);
        assert_eq!(single, vec![(4, 1000)]);
        assert!(calls > 0);
    }

    type Log = Vec<(u32, Vec<u32>)>;

    const FAKE_NOTIFY_ADD: u32 = 0x7000_0010;
    const FAKE_NOTIFY_REMOVE: u32 = 0x7000_0014;
    const FAKE_FREE: u32 = 0x7000_0020;
    const ALLOCATOR: u32 = 0x0031_0000;
    const ALLOCATOR_VTABLE: u32 = 0x0031_0100;
    const ROUTER: u32 = 0x0031_0200;

    /// The addresses `calls_to` filters the call log of a test by.
    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    /// A broad phase handle of `kind` at `base + 0x80` whose owner is `base + 0x40` (offset
    /// `-0x40` in the handle's byte at `+5`) and whose object is `owner + 0x20` (the owner's
    /// byte at `+0x10`) with the given vtable; returns `(handle, owner, object)`.
    fn make_handle(e: &mut Engine, kind: u8, vtable: u32) -> (u32, u32, u32) {
        let base = e.mem.alloc(0x200);
        let (handle, owner, object) = (base + 0x80, base + 0x40, base + 0x60);
        e.mem.set_u8(handle + 4, kind);
        e.mem.set_i8(handle + 5, -0x40);
        e.mem.set_i8(owner + 0x10, 0x20);
        e.mem.set_u32(object, vtable);
        (handle, owner, object)
    }

    /// Sets up the router, an allocator whose slot `0x10` is a double that logs, and the
    /// pair-growing function as a double that gives the array room for 4 pairs.
    fn memory(e: &mut Engine) {
        e.map(ROUTER, 0x20);
        e.mem.set_u32(ROUTER + 0x10, ALLOCATOR);
        e.map(ALLOCATOR, 8);
        e.mem.set_u32(ALLOCATOR, ALLOCATOR_VTABLE);
        e.put_vtable(ALLOCATOR_VTABLE, &[0, 0, 0, 0, FAKE_FREE]);
        e.register(MEMORY_ROUTER, |_, _| Ret {
            eax: ROUTER,
            ..Ret::default()
        });
        e.register(FAKE_FREE, |_, _| Ret::default());
    }

    /// Runs the listener function at `address` on a pair of handles of the given kinds, whose
    /// objects' vtables send slots `0x1c` and `0x24` to doubles; returns the call log and the
    /// `(owner, object)` of both handles.
    fn run_pair_listener(address: u32, kinds: (u8, u8)) -> (Log, [(u32, u32); 2]) {
        let mut e = Engine::new();
        let vtable = 0x0030_0000;
        let mut slots = [0u32; 10];
        slots[7] = FAKE_NOTIFY_ADD;
        slots[9] = FAKE_NOTIFY_REMOVE;
        e.put_vtable(vtable, &slots);
        e.register(FAKE_NOTIFY_ADD, |_, _| Ret::default());
        e.register(FAKE_NOTIFY_REMOVE, |_, _| Ret::default());
        let first = make_handle(&mut e, kinds.0, vtable);
        let second = make_handle(&mut e, kinds.1, vtable);
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, first.0);
        e.mem.set_u32(pair + 4, second.0);
        e.call_log = Some(vec![]);
        e.call(address, &args![0u32, pair]);
        let log = e.call_log.take().unwrap();
        (log, [(first.1, first.2), (second.1, second.2)])
    }

    #[test]
    fn the_phantom_listener_notifies_each_phantom_handle_of_the_other_s_owner() {
        for (address, fake) in [
            (0x00d0_1fc0u32, FAKE_NOTIFY_ADD),
            (0x00d0_2020, FAKE_NOTIFY_REMOVE),
        ] {
            let (log, [first, second]) = run_pair_listener(address, (2, 2));
            assert_eq!(
                calls_to(&log, fake),
                vec![
                    args![first.1, second.0].to_vec(),
                    args![second.1, first.0].to_vec()
                ]
            );
            let (log, [first, second]) = run_pair_listener(address, (2, 1));
            assert_eq!(
                calls_to(&log, fake),
                vec![args![first.1, second.0].to_vec()]
            );
            let (log, [first, second]) = run_pair_listener(address, (1, 2));
            assert_eq!(
                calls_to(&log, fake),
                vec![args![second.1, first.0].to_vec()]
            );
            let (log, _) = run_pair_listener(address, (1, 3));
            assert!(calls_to(&log, fake).is_empty());
        }
    }

    #[test]
    fn the_border_listener_ignores_two_border_handles() {
        for (address, fake) in [
            (0x00d0_2080u32, FAKE_NOTIFY_ADD),
            (0x00d0_20f0, FAKE_NOTIFY_REMOVE),
        ] {
            let (log, _) = run_pair_listener(address, (3, 3));
            assert!(calls_to(&log, fake).is_empty());
            let (log, [first, second]) = run_pair_listener(address, (3, 2));
            assert_eq!(
                calls_to(&log, fake),
                vec![args![first.1, second.0].to_vec()]
            );
            let (log, [first, second]) = run_pair_listener(address, (2, 3));
            assert_eq!(
                calls_to(&log, fake),
                vec![args![second.1, first.0].to_vec()]
            );
            let (log, _) = run_pair_listener(address, (2, 2));
            assert!(calls_to(&log, fake).is_empty());
        }
    }

    /// Four handles whose objects' numbers at `+0xd4` and owners' numbers at `+0x14` are given.
    fn numbered_handle(e: &mut Engine, object_number: i32, owner_number: i32) -> u32 {
        let (handle, owner, object) = make_handle(e, 0, 0);
        e.mem.set_i32(object + 0xd4, object_number);
        e.mem.set_i32(owner + 0x14, owner_number);
        handle
    }

    fn pair_of(e: &mut Engine, first: u32, second: u32) -> u32 {
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, first);
        e.mem.set_u32(pair + 4, second);
        pair
    }

    #[test]
    fn the_sort_comparison_orders_by_the_first_number_then_the_second() {
        let mut e = Engine::new();
        let handles: Vec<u32> = [-1, 1, 2, 2, 3, 4]
            .into_iter()
            .map(|number| numbered_handle(&mut e, number, 0))
            .collect();
        let mut less = |a: (usize, usize), b: (usize, usize)| {
            let left = pair_of(&mut e, handles[a.0], handles[a.1]);
            let right = pair_of(&mut e, handles[b.0], handles[b.1]);
            e.call(0x00d0_2160, &args![left, right]).u32()
        };
        assert_eq!(less((0, 4), (1, 2)), 1);
        assert_eq!(less((1, 2), (0, 4)), 0);
        assert_eq!(less((2, 4), (3, 5)), 1);
        assert_eq!(less((3, 5), (2, 4)), 0);
        assert_eq!(less((2, 4), (3, 4)), 0);
    }

    #[test]
    fn sorting_a_pair_list_orders_each_pair_and_then_the_list() {
        let mut e = Engine::new();
        let numbers = [(3, 1), (5, 2), (4, 4), (2, 9)];
        let list = e.mem.alloc(8);
        let data = e.mem.alloc(8 * 4);
        e.mem.set_u32(list, data);
        e.mem.set_u32(list + 4, 4);
        for (index, (a, b)) in numbers.into_iter().enumerate() {
            let (first, second) = (numbered_handle(&mut e, a, a), numbered_handle(&mut e, b, b));
            e.mem.set_u32(data + 8 * index as u32, first);
            e.mem.set_u32(data + 8 * index as u32 + 4, second);
        }
        e.call(0x00d0_2250, &args![list]);
        let sorted: Vec<(i32, i32)> = (0..4)
            .map(|index| {
                let first = e.mem.u32(data + 8 * index);
                let second = e.mem.u32(data + 8 * index + 4);
                (
                    e.mem.i32(handle_owner(&e, first) + 0x14),
                    e.mem.i32(handle_owner(&e, second) + 0x14),
                )
            })
            .collect();
        assert_eq!(sorted, vec![(1, 3), (2, 5), (2, 9), (4, 4)]);
    }

    #[test]
    fn sorting_calls_the_quick_sort_only_with_more_than_one_pair() {
        let mut e = Engine::new();
        e.register(0x00d0_4200, |_, _| Ret::default());
        let list = e.mem.alloc(8);
        let data = e.mem.alloc(16);
        e.mem.set_u32(list, data);
        let (a, b) = (numbered_handle(&mut e, 0, 1), numbered_handle(&mut e, 0, 2));
        e.mem.set_u32(data, a);
        e.mem.set_u32(data + 4, b);
        e.mem.set_u32(list + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00d0_2250, &args![list]);
        let log: Log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x00d0_4200).is_empty());
        e.mem.set_u32(data + 8, b);
        e.mem.set_u32(data + 12, a);
        e.mem.set_u32(list + 4, 2);
        e.call_log = Some(vec![]);
        e.call(0x00d0_2250, &args![list]);
        let log: Log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x00d0_4200),
            vec![args![data, 0i32, 1i32, 0x00d0_2160u32].to_vec()]
        );
    }

    #[test]
    fn the_multithread_config_is_copied_in_and_out() {
        let mut e = Engine::new();
        let sim = e.mem.alloc(0x100);
        let config = e.mem.alloc(4);
        e.mem.set_u32(config, 0x1234_5678);
        e.call(0x00d0_21f0, &args![sim, config, 0xdeadu32]);
        assert_eq!(e.mem.u32(sim + SIM_CONFIG), 0x1234_5678);
        let out = e.mem.alloc(4);
        e.mem.set_u32(sim + SIM_CONFIG, 0x9abc_def0);
        e.call(0x00d0_21e0, &args![sim, out]);
        assert_eq!(e.mem.u32(out), 0x9abc_def0);
        e.call_log = Some(vec![]);
        e.call(0x00d0_2200, &args![sim, 1u32]);
        e.call(0x00d0_2210, &args![sim, 1u32, 2u32, 3u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    /// A world for the entity listener: the dispatcher holder at `world + 0x78` points at a
    /// table where the agent type of kinds `(1, 2)` is `agent_type`, whose record is `0xfeed`.
    struct ListenerWorld {
        e: Engine,
        listener: u32,
        simulation: u32,
        holder: u32,
        pair: u32,
        owners: (u32, u32),
    }

    fn listener_world(in_step: bool, agent_type: u8) -> ListenerWorld {
        let mut e = Engine::new();
        memory(&mut e);
        let world = e.mem.alloc(0x100);
        let holder = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x2000);
        e.mem.set_u32(world + 0x78, holder);
        e.mem.set_u32(holder, table);
        let simulation = e.mem.alloc(0x100);
        e.mem.set_u32(simulation + SIM_WORLD, world);
        e.mem.set_u8(simulation + SIM_IN_STEP, u8::from(in_step));
        let listener = e.mem.alloc(0x10);
        e.mem.set_u32(listener + 4, simulation);
        let first = make_handle(&mut e, 0, 0);
        let second = make_handle(&mut e, 0, 0);
        e.mem.set_i8(first.1 + 0x1a, 1);
        e.mem.set_i8(second.1 + 0x1a, 2);
        e.mem.set_u8(table + 0x1bb0 + 10 + 2, agent_type);
        e.mem.set_u32(table + 0x1c30 + 64 * 2, 0xfeed);
        e.mem.set_u32(table + 0x1c30 - 64, 0xbeef);
        e.mem.set_u32(first.2 + 0xcc, 7);
        e.mem.set_u32(second.2 + 0xcc, 8);
        let pair = pair_of(&mut e, first.0, second.0);
        ListenerWorld {
            e,
            listener,
            simulation,
            holder,
            pair,
            owners: (first.1, second.1),
        }
    }

    #[test]
    fn the_entity_listener_adds_an_agent_for_the_pair_of_types() {
        let mut w = listener_world(false, 2);
        w.e.register(ADD_AGENT, |_, _| Ret::default());
        w.e.call_log = Some(vec![]);
        w.e.call(0x00d0_26e0, &args![w.listener, w.pair]);
        let log: Log = w.e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ADD_AGENT),
            vec![args![w.owners.0, w.owners.1, w.holder].to_vec()]
        );
        assert_eq!(w.e.mem.u32(w.holder + 0x14), 0xfeed);
    }

    #[test]
    fn the_entity_listener_sign_extends_the_agent_type_and_skips_type_zero() {
        let mut w = listener_world(false, 0xff);
        w.e.register(ADD_AGENT, |_, _| Ret::default());
        w.e.call(0x00d0_26e0, &args![w.listener, w.pair]);
        assert_eq!(w.e.mem.u32(w.holder + 0x14), 0xbeef);
        let mut w = listener_world(false, 0);
        w.e.register(ADD_AGENT, |_, _| Ret::default());
        w.e.call_log = Some(vec![]);
        w.e.call(0x00d0_26e0, &args![w.listener, w.pair]);
        let log: Log = w.e.call_log.take().unwrap();
        assert!(calls_to(&log, ADD_AGENT).is_empty());
        assert_eq!(w.e.mem.u32(w.holder + 0x14), 0);
    }

    /// Makes the growth double give the list room for 4 pairs.
    fn growth_double(w: &mut ListenerWorld) {
        w.e.register_double(ARRAY_GROW, |e, a| {
            let storage = e.mem.alloc(8 * 4);
            e.mem.set_u32(a[1], storage);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
    }

    #[test]
    fn a_deferred_pair_is_queued_and_the_list_grows_when_full() {
        let mut w = listener_world(true, 2);
        growth_double(&mut w);
        let list = w.simulation + SIM_ADD_PAIRS;
        w.e.mem.set_u32(list + 8, DONT_DEALLOCATE_FLAG);
        w.e.call_log = Some(vec![]);
        w.e.call(0x00d0_26e0, &args![w.listener, w.pair]);
        let log: Log = w.e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ARRAY_GROW),
            vec![args![ALLOCATOR, list, 8u32].to_vec()]
        );
        assert_eq!(w.e.mem.u32(list + 4), 1);
        let data = w.e.mem.u32(list);
        assert_eq!(w.e.mem.u32(data), w.e.mem.u32(w.pair));
        assert_eq!(w.e.mem.u32(data + 4), w.e.mem.u32(w.pair + 4));
        // The second pair needs no growth.
        w.e.call_log = Some(vec![]);
        w.e.call(0x00d0_26e0, &args![w.listener, w.pair]);
        let log: Log = w.e.call_log.take().unwrap();
        assert!(calls_to(&log, ARRAY_GROW).is_empty());
        assert_eq!(w.e.mem.u32(list + 4), 2);
        assert_eq!(w.e.mem.u32(data + 8), w.e.mem.u32(w.pair));
        assert!(calls_to(&log, ADD_AGENT).is_empty());
    }

    #[test]
    fn a_deferred_pair_with_no_storage_is_counted_but_not_stored() {
        let mut w = listener_world(true, 2);
        w.e.register(ARRAY_GROW, |_, _| Ret::default());
        let list = w.simulation + SIM_ADD_PAIRS;
        w.e.mem.set_u32(list + 8, 0x4000_0000);
        w.e.call(0x00d0_26e0, &args![w.listener, w.pair]);
        assert_eq!(w.e.mem.u32(list), 0);
        assert_eq!(w.e.mem.u32(list + 4), 1);
    }

    #[test]
    fn pairs_are_not_deferred_for_quality_five_or_equal_numbers() {
        let mut w = listener_world(true, 2);
        w.e.register(ADD_AGENT, |_, _| Ret::default());
        // Equal numbers at +0xcc.
        let first_object = handle_object(&w.e, w.e.mem.u32(w.pair));
        w.e.mem.set_u32(first_object + 0xcc, 8);
        w.e.call_log = Some(vec![]);
        w.e.call(0x00d0_26e0, &args![w.listener, w.pair]);
        let log: Log = w.e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ADD_AGENT).len(), 1);
        assert_eq!(w.e.mem.u32(w.simulation + SIM_ADD_PAIRS + 4), 0);
        // Quality byte 5 on the second object.
        w.e.mem.set_u32(first_object + 0xcc, 7);
        let second_object = handle_object(&w.e, w.e.mem.u32(w.pair + 4));
        w.e.mem.set_u8(second_object + 0xe8, 5);
        w.e.call_log = Some(vec![]);
        w.e.call(0x00d0_26e0, &args![w.listener, w.pair]);
        let log: Log = w.e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ADD_AGENT).len(), 1);
        assert_eq!(w.e.mem.u32(w.simulation + SIM_ADD_PAIRS + 4), 0);
    }

    #[test]
    fn the_remove_listener_finds_and_removes_the_agent_or_queues_the_pair() {
        let mut w = listener_world(false, 2);
        w.e.register(FIND_AGENT, |_, _| Ret {
            eax: 0x5150,
            ..Ret::default()
        });
        w.e.register(REMOVE_AGENT, |_, _| Ret::default());
        w.e.call_log = Some(vec![]);
        w.e.call(0x00d0_27c0, &args![w.listener, w.pair]);
        let log: Log = w.e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FIND_AGENT),
            vec![args![w.owners.0, w.owners.1].to_vec()]
        );
        assert_eq!(
            calls_to(&log, REMOVE_AGENT),
            vec![args![0x5150u32].to_vec()]
        );
        w.e.register(FIND_AGENT, |_, _| Ret::default());
        w.e.call_log = Some(vec![]);
        w.e.call(0x00d0_27c0, &args![w.listener, w.pair]);
        let log: Log = w.e.call_log.take().unwrap();
        assert!(calls_to(&log, REMOVE_AGENT).is_empty());

        let mut w = listener_world(true, 2);
        growth_double(&mut w);
        w.e.mem
            .set_u32(w.simulation + SIM_REMOVE_PAIRS + 8, DONT_DEALLOCATE_FLAG);
        w.e.call(0x00d0_27c0, &args![w.listener, w.pair]);
        assert_eq!(w.e.mem.u32(w.simulation + SIM_REMOVE_PAIRS + 4), 1);
        assert_eq!(w.e.mem.u32(w.simulation + SIM_ADD_PAIRS + 4), 0);
        let data = w.e.mem.u32(w.simulation + SIM_REMOVE_PAIRS);
        assert_eq!(w.e.mem.u32(data), w.e.mem.u32(w.pair));
    }

    #[test]
    fn the_listener_deleting_destructor_deletes_only_when_asked() {
        let mut e = Engine::new();
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let listener = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d0_4300, &args![listener, 0xfeu32]).u32(),
            listener
        );
        assert_eq!(e.mem.u32(listener), LISTENER_BASE_VTABLE);
        let log: Log = e.call_log.take().unwrap();
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d0_4300, &args![listener, 1u32]).u32(), listener);
        let log: Log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, OPERATOR_DELETE),
            vec![args![listener].to_vec()]
        );
    }

    #[test]
    fn the_constructor_wires_the_listeners_into_the_world() {
        let mut e = Engine::new();
        let world = e.mem.alloc(0x100);
        let table = e.mem.alloc(0x100);
        let broad_phase = e.mem.alloc(0x10);
        e.mem.set_u32(world + 0x64, table);
        e.mem.set_u32(world + 0x58, broad_phase);
        e.register_double(CONTINUOUS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + SIM_WORLD, a[1]);
            Ret::default()
        });
        for address in [
            CRITICAL_SECTION_CONSTRUCT,
            MULTITHREAD_CONFIG_CONSTRUCT,
            ENABLE_MULTITHREADING,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let sim = e.mem.alloc(0x180);
        e.mem.set_u8(sim + SIM_IN_STEP, 1);
        e.call_log = Some(vec![]);
        let got = e.call(0x00d0_2c80, &args![sim, world]).u32();
        let log: Log = e.call_log.take().unwrap();
        assert_eq!(got, sim);
        let order: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (CONTINUOUS_CONSTRUCT, args![sim, world].to_vec()),
                (
                    CRITICAL_SECTION_CONSTRUCT,
                    args![sim + 0x78, 4000u32].to_vec()
                ),
                (
                    CRITICAL_SECTION_CONSTRUCT,
                    args![sim + 0x9c, 4000u32].to_vec()
                ),
                (MULTITHREAD_CONFIG_CONSTRUCT, args![sim + 0xb4].to_vec()),
                (
                    CRITICAL_SECTION_CONSTRUCT,
                    args![sim + 0x100, 4000u32].to_vec()
                ),
                (
                    CRITICAL_SECTION_CONSTRUCT,
                    args![sim + 0x140, 4000u32].to_vec()
                ),
                (ENABLE_MULTITHREADING, args![broad_phase, 10000u32].to_vec()),
            ]
        );
        assert_eq!(e.mem.u32(sim), SIMULATION_VTABLE);
        assert_eq!(e.mem.u32(sim + 0x50), 0x010c_cb84);
        assert_eq!(e.mem.u32(sim + 0x58), 0x010c_cb94);
        assert_eq!(e.mem.u32(sim + 0x60), 0x010c_cba4);
        assert_eq!(e.mem.u32(sim + 0x54), sim);
        assert_eq!(e.mem.u32(sim + 0x5c), sim + 0x140);
        assert_eq!(e.mem.u32(sim + 0x64), sim + 0x140);
        assert_eq!(e.mem.u8(sim + SIM_IN_STEP), 0);
        for list in [SIM_ADD_PAIRS, SIM_REMOVE_PAIRS] {
            assert_eq!(e.mem.u32(sim + list), 0);
            assert_eq!(e.mem.u32(sim + list + 4), 0);
            assert_eq!(e.mem.u32(sim + list + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(table + 0x24), sim + 0x50);
        for offset in [0x28, 0x44, 0x48] {
            assert_eq!(e.mem.u32(table + offset), sim + 0x58);
        }
        for offset in [0x2c, 0x4c, 0x64, 0x68, 0x6c] {
            assert_eq!(e.mem.u32(table + offset), sim + 0x60);
        }
    }

    #[test]
    fn the_destructor_empties_the_lists_and_deletes_the_critical_sections() {
        let mut e = Engine::new();
        memory(&mut e);
        for address in [DELETE_CRITICAL_SECTION, CONTINUOUS_DESTRUCT] {
            e.register(address, |_, _| Ret::default());
        }
        let sim = e.mem.alloc(0x180);
        // The remove list owns its storage (4 pairs); the add list does not.
        e.mem.set_u32(sim + SIM_REMOVE_PAIRS, 0x1111);
        e.mem.set_u32(sim + SIM_REMOVE_PAIRS + 4, 3);
        e.mem.set_u32(sim + SIM_REMOVE_PAIRS + 8, 0x4000_0004);
        e.mem.set_u32(sim + SIM_ADD_PAIRS, 0x2222);
        e.mem.set_u32(sim + SIM_ADD_PAIRS + 4, 2);
        e.mem.set_u32(sim + SIM_ADD_PAIRS + 8, 0x8000_0008);
        e.call_log = Some(vec![]);
        e.call(0x00d0_2da0, &args![sim]);
        let log: Log = e.call_log.take().unwrap();
        let order: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .skip(1)
            .filter(|call| call.0 != MEMORY_ROUTER)
            .collect();
        assert_eq!(
            order,
            vec![
                (DELETE_CRITICAL_SECTION, args![sim + 0x140].to_vec()),
                (DELETE_CRITICAL_SECTION, args![sim + 0x100].to_vec()),
                (DELETE_CRITICAL_SECTION, args![sim + 0x9c].to_vec()),
                (FAKE_FREE, args![ALLOCATOR, 0x1111u32, 4u32 * 8].to_vec()),
                (DELETE_CRITICAL_SECTION, args![sim + 0x78].to_vec()),
                (CONTINUOUS_DESTRUCT, args![sim].to_vec()),
            ]
        );
        for list in [SIM_REMOVE_PAIRS, SIM_ADD_PAIRS] {
            assert_eq!(e.mem.u32(sim + list), 0);
            assert_eq!(e.mem.u32(sim + list + 4), 0);
            assert_eq!(e.mem.u32(sim + list + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(sim), SIMULATION_VTABLE);
        for offset in [0x50, 0x58, 0x60] {
            assert_eq!(e.mem.u32(sim + offset), LISTENER_BASE_VTABLE);
        }
    }

    /// A simulation with room for 2 Toi events (one used), a world with a quality table, an
    /// entry and a result with recognisable words.
    struct ToiWorld {
        e: Engine,
        simulation: u32,
        events: u32,
        result: u32,
        entry: u32,
        stream: u32,
    }

    fn toi_world(used: u32) -> ToiWorld {
        let mut e = Engine::new();
        e.map(PROFILE_SLOT & !0xfff, 0x1000);
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        let world = e.mem.alloc(0x100);
        let table = e.mem.alloc(0x2000);
        e.mem.set_u32(world + 0x80, table);
        e.mem.set_u8(table + 0x1c34 + 3 * 64, 0x5a);
        let simulation = e.mem.alloc(0x100);
        e.mem.set_u32(simulation + SIM_WORLD, world);
        let events = e.mem.alloc(0x70 * 2);
        e.mem.set_u32(simulation + SIM_TOI_EVENTS, events);
        e.mem.set_u32(simulation + SIM_TOI_COUNT, used);
        e.mem.set_u32(simulation + SIM_TOI_CAPACITY, 0x4000_0002);
        let result = e.mem.alloc(0x3100);
        for word in 0..0x3c {
            e.mem.set_u32(result + 0x3010 + 4 * word, 0x1000 + word);
        }
        e.mem.set_u32(result + 0x3030, 0x7f80_0001);
        e.mem.set_u32(result + 0x3034, 0x3f80_0000);
        let first = make_handle(&mut e, 0, 0);
        let second = make_handle(&mut e, 0, 0);
        let entry = e.mem.alloc(0x20);
        e.mem.set_i8(entry + 0xc, 3);
        e.mem.set_u32(entry + 0x10, first.0);
        e.mem.set_u32(entry + 0x14, second.0);
        e.mem.set_u32(entry + 8, 0x7777);
        ToiWorld {
            e,
            simulation,
            events,
            result,
            entry,
            stream,
        }
    }

    #[test]
    fn a_toi_event_is_filled_from_the_result_and_the_entry() {
        let mut w = toi_world(1);
        let section = 0x00ab_cdefu32;
        let (first, second) = (w.e.mem.u32(w.entry + 0x10), w.e.mem.u32(w.entry + 0x14));
        w.e.mem.set_i8(first + 0x10, -0x10);
        w.e.mem.set_i8(second + 0x10, 0x18);
        w.e.call_log = Some(vec![]);
        w.e.call(
            0x00d0_2e70,
            &args![w.simulation, w.result, w.entry, section],
        );
        let log: Log = w.e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ENTER_CRITICAL_SECTION),
            vec![args![section].to_vec()]
        );
        assert_eq!(
            calls_to(&log, LEAVE_CRITICAL_SECTION),
            vec![args![section].to_vec()]
        );
        assert_eq!(w.e.mem.u32(w.simulation + SIM_TOI_COUNT), 2);
        let event = w.events + 0x70;
        // The signalling NaN time comes out quiet; the fraction is copied.
        assert_eq!(w.e.mem.u32(event), 0x7fc0_0001);
        assert_eq!(w.e.mem.u32(event + 4), 0x3f80_0000);
        assert_eq!(w.e.mem.u8(event + 8), 0x5a);
        // Each object is the handle plus the signed byte at its +0x10.
        assert_eq!(w.e.mem.u32(event + 0xc), first - 0x10);
        assert_eq!(w.e.mem.u32(event + 0x10), second + 0x18);
        assert_eq!(w.e.mem.u32(event + 0x14), 0x7777);
        let at = |offset: u32| w.e.mem.u32(w.result + offset);
        assert_eq!(w.e.mem.u32(event + 0x18), at(0x3050));
        assert_eq!(w.e.mem.u32(event + 0x1c), at(0x3054));
        assert_eq!(w.e.mem.u32(event + 0x20), at(0x3058));
        assert_eq!(w.e.mem.u32(event + 0x24), at(0x305c));
        assert_eq!(w.e.mem.u32(event + 0x28), at(0x3060));
        for word in 0..7 {
            assert_eq!(w.e.mem.u32(event + 0x2c + 4 * word), at(0x3064 + 4 * word));
        }
        assert_eq!(
            w.e.mem.bytes(event + 0x50, 0x10),
            w.e.mem.bytes(w.result + 0x3010, 0x10)
        );
        assert_eq!(
            w.e.mem.bytes(event + 0x60, 0x10),
            w.e.mem.bytes(w.result + 0x3020, 0x10)
        );
        // The profile stream got the two labels.
        assert_eq!(w.e.mem.u32(w.stream + 0x20), TOI_LABEL);
        assert_eq!(w.e.mem.u32(w.stream + 0x2c), PROFILE_END);
        // The first event was left alone.
        assert_eq!(w.e.mem.u32(w.events), 0);
    }

    #[test]
    fn a_full_toi_queue_reports_an_error() {
        let mut w = toi_world(2);
        w.e.map(ERROR_REPORTER & !0xfff, 0x1000);
        let reporter = w.e.mem.alloc(8);
        let reporter_vtable = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(reporter, reporter_vtable);
        w.e.mem.set_u32(reporter_vtable + 0xc, 0x7000_0030);
        w.e.mem.set_u32(ERROR_REPORTER, reporter);
        for address in [
            TEXT_BUFFER_CONSTRUCT,
            TEXT_BUFFER_WRITE,
            TEXT_BUFFER_DESTRUCT,
            0x7000_0030,
        ] {
            w.e.register(address, |_, _| Ret::default());
        }
        w.e.call_log = Some(vec![]);
        w.e.call(
            0x00d0_2e70,
            &args![w.simulation, w.result, w.entry, 0x1234u32],
        );
        let log: Log = w.e.call_log.take().unwrap();
        let construct = calls_to(&log, TEXT_BUFFER_CONSTRUCT);
        assert_eq!(construct.len(), 1);
        let (object, buffer) = (construct[0][0], construct[0][1]);
        assert_eq!(buffer, object + 0xc);
        assert_eq!(construct[0][2], 0x200);
        assert_eq!(
            calls_to(&log, TEXT_BUFFER_WRITE),
            vec![args![object, 0x010c_cc70u32].to_vec()]
        );
        assert_eq!(
            calls_to(&log, 0x7000_0030),
            vec![args![
                reporter,
                1u32,
                0xf032_3454u32,
                buffer,
                0x010c_cc30u32,
                0x3f7u32
            ]
            .to_vec()]
        );
        assert_eq!(
            calls_to(&log, TEXT_BUFFER_DESTRUCT),
            vec![args![object].to_vec()]
        );
        assert!(calls_to(&log, ENTER_CRITICAL_SECTION).is_empty());
        assert_eq!(w.e.mem.u32(w.simulation + SIM_TOI_COUNT), 2);
    }
}

#[cfg(test)]
mod step_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use super::super::hkpsphereshape::{store_vec, PROFILE_SLOT, TLS_GET_VALUE};

    type Log = Vec<(u32, Vec<u32>)>;
    type Snapshots = Rc<RefCell<Vec<Vec<u8>>>>;

    const FREE_BLOCK: u32 = 0x0900_0801;
    const FREE_ARRAY: u32 = 0x0900_0802;
    const STEP_NOTIFY: u32 = 0x0900_0803;
    const MEMORY_GUARD: u32 = 0x0900_0804;
    const SIMULATION_SLOT: u32 = 0x0900_0805;
    const MANAGER_SLOT: u32 = 0x0900_0806;
    const POOL_SLOT_C: u32 = 0x0900_0807;
    const POOL_SLOT_10: u32 = 0x0900_0808;

    struct Rig {
        e: Engine,
        sim: u32,
        world: u32,
        router: u32,
        allocator: u32,
        stack_top: u32,
        stream: u32,
    }

    /// An engine with the exe's data range, a world and simulation (`+0xc` of the simulation is
    /// the world), a memory router (temporary stack, allocator whose slots 8 and `0x10` are
    /// doubles) and a profiling stream whose records start at `stream + 0x20`.
    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_BLOCK);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let stack_top = e.mem.alloc(0x4000);
        let router = e.mem.alloc(0x40);
        e.mem.set_u32(router, stack_top);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        plain(&mut e, &[FREE_BLOCK, FREE_ARRAY]);
        let stream = e.mem.alloc(0x200);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x1e0);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        let world = e.mem.alloc(0x400);
        let sim = e.mem.alloc(0x200);
        e.mem.set_u32(sim + SIM_WORLD, world);
        Rig {
            e,
            sim,
            world,
            router,
            allocator,
            stack_top,
            stream,
        }
    }

    /// Doubles that do nothing and return 0.
    fn plain(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register_double(*address, |_, _| Ret::default());
        }
    }

    /// A double that returns `value`.
    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// A double that stores `len` bytes at argument `arg` each time it runs.
    fn snapshots(e: &mut Engine, address: u32, arg: usize, len: u32) -> Snapshots {
        let seen: Snapshots = Rc::default();
        let sink = Rc::clone(&seen);
        e.register_double(address, move |e, a| {
            sink.borrow_mut().push(e.mem.bytes(a[arg], len));
            Ret::default()
        });
        seen
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    /// The addresses of the calls to `wanted`, in order.
    fn order(log: &Log, wanted: &[u32]) -> Vec<u32> {
        log.iter()
            .map(|call| call.0)
            .filter(|address| wanted.contains(address))
            .collect()
    }

    fn word(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap())
    }

    fn half(bytes: &[u8], at: usize) -> u16 {
        u16::from_le_bytes(bytes[at..at + 2].try_into().unwrap())
    }

    fn floats(values: [f32; 4]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    /// The first word (the label) of each profiling record from `stream + 0x20` on, given the
    /// sizes of the records (12 bytes, or 16 for a two-label record).
    fn labels(r: &Rig, sizes: &[u32]) -> Vec<u32> {
        let mut at = r.stream + 0x20;
        sizes
            .iter()
            .map(|size| {
                let label = r.e.mem.u32(at);
                at += size;
                label
            })
            .collect()
    }

    #[test]
    fn the_step_info_is_start_end_delta_and_inverse() {
        let bits = f32::to_bits;
        assert_eq!(
            step_info(bits(1.0), bits(3.0)),
            [bits(1.0), bits(3.0), bits(2.0), bits(0.5)]
        );
        // Equal times: the delta is 0 and so is the inverse (not infinity).
        assert_eq!(
            step_info(bits(2.0), bits(2.0)),
            [bits(2.0), bits(2.0), 0, 0]
        );
        // A negative zero delta compares equal to 0 as well.
        assert_eq!(step_info(bits(0.0), bits(-0.0))[3], 0);
        // A signalling NaN start comes out quiet, the delta and its inverse are NaNs.
        let info = step_info(0x7f80_0001, bits(1.0));
        assert_eq!(info[0], 0x7fc0_0001);
        assert!(f32::from_bits(info[2]).is_nan() && f32::from_bits(info[3]).is_nan());
    }

    struct Step {
        r: Rig,
        queue: u32,
        integrator: u32,
        state: u32,
        info: Snapshots,
        jobs: Snapshots,
        actions_seen: Rc<Cell<u32>>,
    }

    /// A world ready for `stepBeginSt`: start time 1.5, memory guard answering 1, watchdog
    /// answering the word at `world + 0x3f0`, three actions with the profiling byte set.
    fn step_rig() -> Step {
        let mut r = rig();
        let (sim, world) = (r.sim, r.world);
        let integrator = r.e.mem.alloc(0x80);
        let state = r.e.mem.alloc(0x130);
        let queue = r.e.mem.alloc(0x10);
        let owner_vtable = r.e.mem.alloc(0x20);
        r.e.mem.set_u32(owner_vtable + 0x14, STEP_NOTIFY);
        let owner = r.e.mem.alloc(8);
        r.e.mem.set_u32(owner, owner_vtable);
        let guard_vtable = r.e.mem.alloc(0x40);
        r.e.mem.set_u32(guard_vtable + 0x20, MEMORY_GUARD);
        let guard = r.e.mem.alloc(0x10);
        r.e.mem.set_u32(guard, guard_vtable);
        r.e.mem.set_u32(guard + 8, 1);
        r.e.mem.set_u32(r.router + 0x18, guard);
        let query = r.e.mem.alloc(8);
        r.e.mem.set_u32(query, 0x55);
        let e = &mut r.e;
        e.mem.set_u32(world + 0x4c, owner);
        e.mem.set_u32(world + 0x74, state);
        e.mem.set_u32(world + 0x78, integrator);
        e.mem.set_u32(world + 0x1b8, query);
        e.mem.set_f32(sim + 0x18, 1.5);
        e.mem.set_f32(world + 0x300, 2.0);
        e.mem.set_i32(world + 0x2f4, 3);
        store_vec(e, world + 0x10, [1.0, 2.0, 3.0, 4.0]);
        e.mem.set_u32(integrator, 0x99);
        e.mem.set_f32(integrator + 8, 0.25);
        e.mem.set_u32(world + 0x2c, 3);
        e.mem.set_u32(world + 0x38, 0x38);
        e.mem.set_u8(world + 0xb0, 1);
        e.mem.set_u32(world + 0x98, 5);
        let info = snapshots(e, STEP_NOTIFY, 2, 16);
        let jobs = snapshots(e, JOB_QUEUE_ADD_JOB, 1, 0x24);
        e.register_double(0x00c9_2b50, |e, a| {
            e.mem.set_u32(a[1], 0x1234);
            Ret::default()
        });
        e.register_double(MEMORY_GUARD, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        e.register_double(0x00c9_3570, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x3f0),
            ..Ret::default()
        });
        let actions_seen = Rc::new(Cell::new(0));
        let seen = Rc::clone(&actions_seen);
        e.register_double(0x00cf_8b00, move |e, _| {
            seen.set(e.mem.u32(world + 0x98));
            Ret::default()
        });
        plain(
            e,
            &[
                0x00cb_55d0,
                0x00d8_92d0,
                CONSTRAINT_QUERY_SET,
                JOB_QUEUE_PREPARE,
                0x00c9_10b0,
                0x00d0_1bc0,
                0x00c9_1120,
                0x00c9_1140,
            ],
        );
        Step {
            r,
            queue,
            integrator,
            state,
            info,
            jobs,
            actions_seen,
        }
    }

    #[test]
    fn a_step_starts_with_the_step_info_the_scale_vectors_and_a_job() {
        let mut s = step_rig();
        let (sim, world, queue) = (s.r.sim, s.r.world, s.queue);
        s.r.e.call_log = Some(vec![]);
        let got =
            s.r.e
                .call(0x00d0_22b0, &args![sim, queue, 0.5f32.to_bits()])
                .u32();
        let log: Log = s.r.e.call_log.take().unwrap();
        let e = &s.r.e;
        assert_eq!(got, 0);
        assert_eq!(e.mem.u8(sim + SIM_IN_STEP), 1);
        assert_eq!(e.mem.f32(sim + 0x1c), 0.5);
        let info = [1.5f32, 2.0, 0.5, 2.0];
        for (lane, value) in info.into_iter().enumerate() {
            let at = 4 * lane as u32;
            assert_eq!(e.mem.f32(world + 0x1d0 + at), value);
            assert_eq!(e.mem.f32(s.integrator + 0x50 + at), value);
        }
        assert_eq!(s.info.borrow()[0], floats(info));
        assert_eq!(e.mem.f32(world + 0x2ec), 1.0);
        assert_eq!(e.mem.f32(world + 0x2f0), 6.0);
        assert_eq!(
            super::super::hkpsphereshape::load_vec(e, world + 0x1f0),
            [1.0, 2.0, 3.0, 4.0]
        );
        assert_eq!(
            super::super::hkpsphereshape::load_vec(e, world + 0x200),
            [0.5, 1.0, 1.5, 2.0]
        );
        // The word the query pointer points at is cleared and the state is set up.
        assert_eq!(e.mem.u32(e.mem.u32(world + 0x1b8)), 0);
        let state = s.state;
        assert_eq!(e.mem.u32(state + 0x114), world + 0x1d0);
        assert_eq!(e.mem.u32(state + 0x90), 0x99);
        assert_eq!(e.mem.u32(state + 0x110), sim);
        assert_eq!(e.mem.u32(state + 0x118), 0.25f32.to_bits());
        assert_eq!(e.mem.u32(state + 0x11c), 0x0126_9210);
        assert_eq!(e.mem.u32(state), world);
        assert_eq!(
            calls_to(&log, CONSTRAINT_QUERY_SET),
            vec![args![
                state + 0x10,
                world + 0x1e0,
                world + 0x1d0,
                e.mem.u32(world + 0x1b8)
            ]
            .to_vec()]
        );
        // The memory the integration needs is what the guard was asked about.
        assert_eq!(calls_to(&log, MEMORY_GUARD).len(), 1);
        assert_eq!(calls_to(&log, MEMORY_GUARD)[0][1], 0x1234);
        // The actions step runs with the word at +0x98 taken down.
        assert_eq!(s.actions_seen.get(), 4);
        assert_eq!(e.mem.u32(world + 0x98), 5);
        assert_eq!(calls_to(&log, 0x00cf_8b00), vec![args![sim].to_vec()]);
        assert_eq!(
            calls_to(&log, JOB_QUEUE_PREPARE),
            vec![
                args![queue, 0u32, 3u32].to_vec(),
                args![queue, 1u32, 3u32].to_vec()
            ]
        );
        let job = &s.jobs.borrow()[0];
        assert_eq!(job[..3], [0, 0, 2]);
        assert_eq!(
            (half(job, 4), half(job, 6), half(job, 0x10)),
            (0x30, 0xffff, 0)
        );
        assert_eq!(
            (
                word(job, 0x14),
                word(job, 0x18),
                word(job, 0x1c),
                word(job, 0x20)
            ),
            (0, 0, state, 3)
        );
        assert_eq!(calls_to(&log, JOB_QUEUE_ADD_JOB)[0][0], queue);
        assert_eq!(calls_to(&log, JOB_QUEUE_ADD_JOB)[0][2], 1);
        // The counters, the copies and the profiling records.
        assert_eq!(e.mem.u8(world + 0x307), 1);
        assert_eq!(e.mem.u8(world + 0x305), 0);
        assert_eq!(e.mem.u32(sim + 0xb8), 3);
        assert_eq!(e.mem.u32(sim + 0xbc), 0x38);
        assert_eq!(
            order(
                &log,
                &[
                    STEP_NOTIFY,
                    0x00cb_55d0,
                    0x00c9_2b50,
                    MEMORY_GUARD,
                    0x00d8_92d0,
                    0x00cf_a7d0,
                    CONSTRAINT_QUERY_SET,
                    0x00cf_8b00,
                    JOB_QUEUE_PREPARE,
                    JOB_QUEUE_ADD_JOB,
                    0x00c9_10b0
                ]
            ),
            vec![
                STEP_NOTIFY,
                0x00cb_55d0,
                0x00c9_2b50,
                MEMORY_GUARD,
                0x00d8_92d0,
                0x00cf_a7d0,
                CONSTRAINT_QUERY_SET,
                0x00cf_8b00,
                JOB_QUEUE_PREPARE,
                JOB_QUEUE_PREPARE,
                JOB_QUEUE_ADD_JOB,
                0x00c9_10b0
            ]
        );
        let stream = s.r.stream;
        assert_eq!(e.mem.u32(stream + 0x20), 0x010c_cbb0);
        assert_eq!(e.mem.u32(stream + 0x2c), 0x010c_c5bc);
        assert_eq!(e.mem.u32(stream + 0x30), 0x010c_c5b0);
        assert_eq!(e.mem.u32(stream + 0x3c), 0x0107_6f38);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x48);
    }

    #[test]
    fn the_step_counters_toggle_their_bits() {
        let mut s = step_rig();
        let (sim, world, queue) = (s.r.sim, s.r.world, s.queue);
        // (counter, flags, flip) before the step and after it.
        let cases: [((u8, u8, u8), (u8, u8, u8)); 5] = [
            ((3, 0, 0), (4, 1, 0)),
            ((7, 0, 0), (8, 2, 0)),
            ((11, 0, 0), (12, 1, 0)),
            ((15, 3, 0), (0, 1, 1)),
            ((31, 3, 1), (0, 1, 0)),
        ];
        for (before, expected) in cases {
            let cursor = s.r.stream + 0x20;
            s.r.e.mem.set_u32(s.r.stream + 4, cursor);
            s.r.e.mem.set_u8(world + 0x307, before.0);
            s.r.e.mem.set_u8(world + 0x305, before.1);
            s.r.e.mem.set_u8(world + 0x306, before.2);
            s.r.e
                .call(0x00d0_22b0, &args![sim, queue, 0.5f32.to_bits()]);
            let e = &s.r.e;
            assert_eq!(
                (
                    e.mem.u8(world + 0x307),
                    e.mem.u8(world + 0x305),
                    e.mem.u8(world + 0x306)
                ),
                expected,
                "counter {}",
                before.0
            );
        }
    }

    #[test]
    fn without_enough_memory_and_a_watchdog_the_step_fails_at_once() {
        let mut s = step_rig();
        let (sim, world, queue) = (s.r.sim, s.r.world, s.queue);
        let guard = s.r.e.mem.u32(s.r.router + 0x18);
        s.r.e.mem.set_u32(guard + 8, 0);
        s.r.e.call_log = Some(vec![]);
        let got =
            s.r.e
                .call(0x00d0_22b0, &args![sim, queue, 0.5f32.to_bits()])
                .u32();
        let log: Log = s.r.e.call_log.take().unwrap();
        assert_eq!(got, 1);
        assert_eq!(s.r.e.mem.u32(sim + 0x28), 1);
        assert!(order(&log, &[0x00d8_92d0, 0x00c9_10b0]).is_empty());
        // Only the first record was written.
        assert_eq!(s.r.e.mem.u32(s.r.stream + 4), s.r.stream + 0x30);
        assert_eq!(s.r.e.mem.u32(world + 0x94), 1);
    }

    #[test]
    #[should_panic(expected = "INT3 at 00d026c3")]
    fn a_world_that_traps_stops_when_there_is_no_watchdog() {
        let mut s = step_rig();
        let (sim, world, queue) = (s.r.sim, s.r.world, s.queue);
        let guard = s.r.e.mem.u32(s.r.router + 0x18);
        s.r.e.mem.set_u32(guard + 8, 0);
        s.r.e.mem.set_u8(world + 0x54, 1);
        s.r.e
            .call(0x00d0_22b0, &args![sim, queue, 0.5f32.to_bits()]);
    }

    #[test]
    fn with_a_watchdog_the_world_scope_is_left_and_entered_again() {
        let mut s = step_rig();
        let (sim, world, queue) = (s.r.sim, s.r.world, s.queue);
        let guard = s.r.e.mem.u32(s.r.router + 0x18);
        s.r.e.mem.set_u32(guard + 8, 0);
        s.r.e.mem.set_u32(world + 0x3f0, 1);
        s.r.e.mem.set_u32(world + 0x8c, 1);
        s.r.e.mem.set_u32(world + 0xa4, 1);
        s.r.e.mem.set_u32(world + 0x90, 1);
        s.r.e.call_log = Some(vec![]);
        let got =
            s.r.e
                .call(0x00d0_22b0, &args![sim, queue, 0.5f32.to_bits()])
                .u32();
        let log: Log = s.r.e.call_log.take().unwrap();
        assert_eq!(got, 0);
        assert_eq!(
            order(&log, &[0x00c9_1120, 0x00c9_1140, 0x00d0_1bc0, 0x00d8_92d0]),
            vec![0x00c9_1120, 0x00c9_1140, 0x00d0_1bc0, 0x00d8_92d0]
        );
        assert_eq!(calls_to(&log, 0x00d0_1bc0), vec![args![world].to_vec()]);
        assert_eq!(calls_to(&log, 0x00c9_1120), vec![args![world].to_vec()]);
        assert_eq!(s.r.e.mem.u32(world + 0x94), 1);
        // Another watcher keeps the scope: nothing runs when the word does not reach 0.
        s.r.e.mem.set_u32(world + 0x94, 4);
        s.r.e.call_log = Some(vec![]);
        s.r.e
            .call(0x00d0_22b0, &args![sim, queue, 0.5f32.to_bits()]);
        let log: Log = s.r.e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x00c9_1120).is_empty());
        assert_eq!(s.r.e.mem.u32(world + 0x94), 5);
        // A set byte at +0x9c keeps it too.
        s.r.e.mem.set_u32(world + 0x94, 0);
        s.r.e.mem.set_u8(world + 0x9c, 1);
        s.r.e.call_log = Some(vec![]);
        s.r.e
            .call(0x00d0_22b0, &args![sim, queue, 0.5f32.to_bits()]);
        let log: Log = s.r.e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x00c9_1120).is_empty());
    }

    /// A world for the narrow phase job functions: limit 4 entries, sector size 8, state word
    /// `0x4321`, an integration block whose first word is `0x1357` and whose float at `+8` is
    /// 0.25, a collision input whose words at `+0x50` are 1 to 4 and an owner with `0x1234` at
    /// `+0x20`.
    struct Jobs {
        r: Rig,
        queue: u32,
        input: u32,
        owner: u32,
        jobs: Snapshots,
        posts: Snapshots,
        states: Snapshots,
    }

    fn jobs_rig() -> Jobs {
        let mut r = rig();
        let (sim, world) = (r.sim, r.world);
        let queue = r.e.mem.alloc(0x10);
        let integrator = r.e.mem.alloc(0x80);
        let input = r.e.mem.alloc(0x70);
        let owner = r.e.mem.alloc(0x40);
        let query = r.e.mem.alloc(8);
        let e = &mut r.e;
        e.mem.set_u32(sim + SIM_JOB_QUEUE, queue);
        e.mem.set_u32(world + 0xec, 4);
        e.mem.set_u32(world + 0xe8, 8);
        e.mem.set_u32(world + 0x74, 0x4321);
        e.mem.set_u32(world + 0x78, integrator);
        e.mem.set_u32(world + 0x1b8, query);
        e.mem.set_u32(integrator, 0x1357);
        e.mem.set_f32(integrator + 8, 0.25);
        for lane in 0..4 {
            e.mem.set_u32(input + 0x50 + 4 * lane, 1 + lane);
        }
        e.mem.set_u16(owner + 0x20, 0x1234);
        answer(e, SECTOR_HEADER_ALLOCATE, 0xaaaa_0000);
        let jobs = snapshots(e, JOB_QUEUE_ADD_JOB, 1, 0x44);
        let posts: Snapshots = Rc::default();
        let states: Snapshots = Rc::default();
        let (post_sink, state_sink) = (Rc::clone(&posts), Rc::clone(&states));
        e.register_double(POST_COLLIDE_JOB, move |e, a| {
            state_sink.borrow_mut().push(e.mem.bytes(a[0], 0x120));
            post_sink.borrow_mut().push(e.mem.bytes(a[2], 0x30));
            e.mem.set_u32(a[0] + 0xf4, 0xbbbb_0000);
            Ret::default()
        });
        plain(
            e,
            &[
                JOB_QUEUE_PROCESS_ALL,
                CONSTRAINT_QUERY_SET,
                CONTINUOUS_PROCESS_ENTRIES,
            ],
        );
        Jobs {
            r,
            queue,
            input,
            owner,
            jobs,
            posts,
            states,
        }
    }

    #[test]
    fn many_entries_become_a_job_and_a_post_collide_job() {
        let mut j = jobs_rig();
        let (sim, world, queue, input, owner) = (j.r.sim, j.r.world, j.queue, j.input, j.owner);
        j.r.e.call_log = Some(vec![]);
        j.r.e.call(
            0x00d0_3030,
            &args![sim, 0x1000_0000u32, 20u32, input, owner, 0x1ffu32],
        );
        let log: Log = j.r.e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SECTOR_HEADER_ALLOCATE),
            vec![args![3u32, 8u32].to_vec()]
        );
        assert_eq!(calls_to(&log, JOB_QUEUE_ADD_JOB)[0][0], queue);
        assert_eq!(calls_to(&log, JOB_QUEUE_ADD_JOB)[0][2], 1);
        assert_eq!(
            calls_to(&log, JOB_QUEUE_PROCESS_ALL),
            vec![args![queue].to_vec()]
        );
        let job = &j.jobs.borrow()[0];
        assert_eq!(job[..3], [0xf, 1, 2]);
        assert_eq!(
            (half(job, 4), half(job, 6), half(job, 0x10)),
            (0x50, 0xffff, 0x1234)
        );
        assert_eq!(
            (
                word(job, 0x14),
                word(job, 0x18),
                word(job, 0x1c),
                word(job, 0x20)
            ),
            (0, 0, 0x4321, 0xaaaa_0000)
        );
        assert_eq!(
            (
                half(job, 0x24),
                half(job, 0x28),
                half(job, 0x2a),
                word(job, 0x2c)
            ),
            (0, 20, 8, 0x1000_0000)
        );
        assert_eq!(
            [
                word(job, 0x30),
                word(job, 0x34),
                word(job, 0x38),
                word(job, 0x3c)
            ],
            [1, 2, 3, 4]
        );
        assert_eq!(job[0x40], 0xff);
        let post = &j.posts.borrow()[0];
        assert_eq!(post[..3], [0xe, 0, 2]);
        assert_eq!(
            (half(post, 4), half(post, 6), half(post, 0x10)),
            (0x30, 0xffff, 0x1234)
        );
        assert_eq!(
            (
                word(post, 0x14),
                word(post, 0x18),
                word(post, 0x1c),
                word(post, 0x20)
            ),
            (owner, 0, 0x4321, 0xaaaa_0000)
        );
        let state = &j.states.borrow()[0];
        assert_eq!(word(state, 0), world);
        assert_eq!(word(state, 0x90), 0x1357);
        assert_eq!((word(state, 0x94), word(state, 0xf4)), (0, 0));
        assert_eq!(word(state, 0x110), sim);
        assert_eq!(word(state, 0x114), world + 0x1d0);
        assert_eq!(word(state, 0x118), 0.25f32.to_bits());
        assert_eq!(word(state, 0x11c), 0x0126_9210);
        // The block the post collide job left behind is given back.
        assert_eq!(
            calls_to(&log, FREE_BLOCK),
            vec![args![j.r.allocator, 0xbbbb_0000u32, 0x200u32].to_vec()]
        );
        assert_eq!(
            order(
                &log,
                &[
                    SECTOR_HEADER_ALLOCATE,
                    JOB_QUEUE_ADD_JOB,
                    JOB_QUEUE_PROCESS_ALL,
                    0x00cf_a7d0,
                    CONSTRAINT_QUERY_SET,
                    POST_COLLIDE_JOB,
                    FREE_BLOCK
                ]
            ),
            vec![
                SECTOR_HEADER_ALLOCATE,
                JOB_QUEUE_ADD_JOB,
                JOB_QUEUE_PROCESS_ALL,
                0x00cf_a7d0,
                CONSTRAINT_QUERY_SET,
                POST_COLLIDE_JOB,
                FREE_BLOCK
            ]
        );
    }

    #[test]
    fn few_entries_or_no_queue_go_straight_to_the_continuous_function() {
        let mut j = jobs_rig();
        let (sim, input, owner) = (j.r.sim, j.input, j.owner);
        // The count equals the limit: not above it.
        j.r.e.call_log = Some(vec![]);
        j.r.e.call(
            0x00d0_3030,
            &args![sim, 0x1000_0000u32, 4u32, input, owner, 0x1ffu32],
        );
        let log: Log = j.r.e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, CONTINUOUS_PROCESS_ENTRIES),
            vec![args![sim, 0x1000_0000u32, 4u32, input, owner, 0x1ffu32].to_vec()]
        );
        assert!(calls_to(&log, JOB_QUEUE_ADD_JOB).is_empty());
        // A negative count is below the limit too (signed compare).
        j.r.e.call_log = Some(vec![]);
        j.r.e.call(
            0x00d0_3030,
            &args![sim, 0x1000_0000u32, -1i32 as u32, input, owner, 0u32],
        );
        let log: Log = j.r.e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CONTINUOUS_PROCESS_ENTRIES).len(), 1);
        // Without a queue the limit is not even read.
        j.r.e.mem.set_u32(sim + SIM_JOB_QUEUE, 0);
        j.r.e.call_log = Some(vec![]);
        j.r.e.call(
            0x00d0_3030,
            &args![sim, 0x1000_0000u32, 100u32, input, owner, 0u32],
        );
        let log: Log = j.r.e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CONTINUOUS_PROCESS_ENTRIES).len(), 1);
        assert!(calls_to(&log, JOB_QUEUE_PROCESS_ALL).is_empty());
    }

    /// Three records `{island, count, first}` for `processAgentNnEntriesFromMultipleIslands`:
    /// the first and the third with islands of their own (halfwords `0x5555` and `0x6666` at
    /// `+0x20`), the second with the world's island `0x22`.
    fn island_records(j: &mut Jobs) -> (u32, [u32; 2]) {
        let e = &mut j.r.e;
        let first = e.mem.alloc(0x40);
        e.mem.set_u16(first + 0x20, 0x5555);
        let third = e.mem.alloc(0x40);
        e.mem.set_u16(third + 0x20, 0x6666);
        let records = e.mem.alloc(0x60);
        for (index, (island, count, start)) in [(first, 3u32, 0u32), (0x22, 4, 3), (third, 0, 7)]
            .into_iter()
            .enumerate()
        {
            let at = records + 0x18 * index as u32;
            e.mem.set_u32(at, island);
            e.mem.set_u32(at + 4, count);
            e.mem.set_u32(at + 8, start);
        }
        e.mem.set_u32(j.r.world + 0x20, 0x22);
        (records, [first, third])
    }

    #[test]
    fn every_island_gets_a_job_except_the_worlds_own_which_runs_directly() {
        let mut j = jobs_rig();
        let (sim, world, queue, input) = (j.r.sim, j.r.world, j.queue, j.input);
        let (records, [first, _]) = island_records(&mut j);
        j.r.e.mem.set_u32(world + 0xec, 2);
        j.r.e.mem.set_u32(world + 0xe8, 2);
        let top = j.r.stack_top;
        let base = 0x2000_0000u32;
        j.r.e.call_log = Some(vec![]);
        j.r.e.call(
            0x00d0_2930,
            &args![sim, base, 10u32, records, 3u32, input, 0x1ffu32],
        );
        let log: Log = j.r.e.call_log.take().unwrap();
        let e = &j.r.e;
        // Room for three jobs was taken and given back.
        assert_eq!(e.mem.u32(j.r.router), top);
        // The first record: (3 - 1) / 2 + 1 sectors of 2.
        assert_eq!(
            calls_to(&log, SECTOR_HEADER_ALLOCATE),
            vec![args![2u32, 2u32].to_vec()]
        );
        assert_eq!(
            calls_to(&log, JOB_QUEUE_ADD_JOB),
            vec![args![queue, top, 1u32].to_vec()]
        );
        assert_eq!(e.mem.u32(records + 0x10), top);
        assert_eq!(e.mem.u32(records + 0x18 + 0x10), 0);
        assert_eq!(e.mem.u32(records + 0x30 + 0x10), 0);
        let job = e.mem.bytes(top, 0x44);
        assert_eq!(job[..3], [0xf, 1, 2]);
        assert_eq!(
            (half(&job, 4), half(&job, 6), half(&job, 0x10)),
            (0x50, 0xffff, 0x5555)
        );
        assert_eq!(
            (
                word(&job, 0x14),
                word(&job, 0x18),
                word(&job, 0x1c),
                word(&job, 0x20)
            ),
            (0, 0, 0x4321, 0xaaaa_0000)
        );
        assert_eq!(
            (
                half(&job, 0x24),
                half(&job, 0x28),
                half(&job, 0x2a),
                word(&job, 0x2c)
            ),
            (0, 3, 2, base)
        );
        assert_eq!(
            [
                word(&job, 0x30),
                word(&job, 0x34),
                word(&job, 0x38),
                word(&job, 0x3c)
            ],
            [1, 2, 3, 4]
        );
        assert_eq!(job[0x40], 0xff);
        // The third record has no entries: its job is prepared but never added.
        let empty = e.mem.bytes(top + 0xa0, 0x44);
        assert_eq!(empty[..3], [0xf, 1, 1]);
        assert_eq!(word(&empty, 0x1c), 0x4321);
        assert_eq!(half(&empty, 0x28), 0);
        assert_eq!(word(&empty, 0x2c), base + 28);
        // One post collide job, for the first record's island.
        let posts = j.posts.borrow();
        assert_eq!(posts.len(), 1);
        let post = &posts[0];
        assert_eq!(post[..3], [0xe, 0, 2]);
        assert_eq!(half(post, 0x10), 0x5555);
        assert_eq!(
            (
                word(post, 0x14),
                word(post, 0x18),
                word(post, 0x1c),
                word(post, 0x20)
            ),
            (first, 0, 0x4321, 0xaaaa_0000)
        );
        assert_eq!(word(&j.states.borrow()[0], 0), world);
        // The world's own island runs directly, with the low byte of the last word clear.
        assert_eq!(
            calls_to(&log, CONTINUOUS_PROCESS_ENTRIES),
            vec![args![sim, base + 12, 4u32, input, 0x22u32, 0u32].to_vec()]
        );
        assert_eq!(
            order(
                &log,
                &[
                    JOB_QUEUE_ADD_JOB,
                    JOB_QUEUE_PROCESS_ALL,
                    POST_COLLIDE_JOB,
                    CONTINUOUS_PROCESS_ENTRIES
                ]
            ),
            vec![
                JOB_QUEUE_ADD_JOB,
                JOB_QUEUE_PROCESS_ALL,
                POST_COLLIDE_JOB,
                CONTINUOUS_PROCESS_ENTRIES
            ]
        );
    }

    #[test]
    fn few_entries_in_many_islands_run_one_island_at_a_time() {
        let mut j = jobs_rig();
        let (sim, input) = (j.r.sim, j.input);
        let (records, _) = island_records(&mut j);
        let base = 0x2000_0000u32;
        j.r.e.call_log = Some(vec![]);
        j.r.e.call(
            0x00d0_2930,
            &args![sim, base, 4u32, records, 3u32, input, 0x77u32],
        );
        let log: Log = j.r.e.call_log.take().unwrap();
        let first = j.r.e.mem.u32(records);
        let third = j.r.e.mem.u32(records + 0x30);
        assert_eq!(
            calls_to(&log, CONTINUOUS_PROCESS_ENTRIES),
            vec![
                args![sim, base, 3u32, input, first, 0x77u32].to_vec(),
                args![sim, base + 12, 4u32, input, 0x22u32, 0x77u32].to_vec(),
                args![sim, base + 28, 0u32, input, third, 0x77u32].to_vec(),
            ]
        );
        assert!(calls_to(&log, JOB_QUEUE_ADD_JOB).is_empty());
        // The temporary stack was not touched.
        assert_eq!(j.r.e.mem.u32(j.r.router), j.r.stack_top);
    }

    #[test]
    fn one_island_without_a_queue_is_left_to_the_continuous_function() {
        let mut r = rig();
        let sim = r.sim;
        let e = &mut r.e;
        plain(e, &[0x00d0_db40]);
        e.call_log = Some(vec![]);
        e.call(
            0x00d0_3970,
            &args![sim, 0x1111u32, 3u32, 0x2222u32, 0x3333u32],
        );
        let log: Log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x00d0_db40),
            vec![args![sim, 0x1111u32, 3u32, 0x2222u32, sim + 0x3c].to_vec()]
        );
    }

    #[test]
    fn one_island_collects_list_entries_and_references_the_others() {
        let mut r = rig();
        let (sim, world) = (r.sim, r.world);
        let (allocator, top) = (r.allocator, r.stack_top);
        let e = &mut r.e;
        let queue = e.mem.alloc(0x10);
        e.mem.set_u32(sim + SIM_JOB_QUEUE, queue);
        e.mem.set_u32(world + 0xec, 5);
        let island = e.mem.alloc(0x80);
        e.mem.set_u32(island + 0x4c, 0x10);
        // Type 0 entries have a creation word and no flag byte (the list); type 1 entries have
        // no creation word (the map).
        let table = e.mem.alloc(0x2000);
        e.mem.set_u32(world + 0x80, table);
        e.mem.set_u32(table + 0x1c30, 1);
        let thing = |e: &mut Engine, island: u32, flag: u16, key: u32| {
            let thing = e.mem.alloc(0x200);
            e.mem.set_u32(thing + 0xcc, island);
            e.mem.set_u16(thing + 0xa8, flag);
            e.mem.set_u32(thing + 0xd4, key);
            thing
        };
        let (first, second) = (thing(e, island, 1, 0x41), thing(e, island, 2, 0x42));
        let same_island = thing(e, island, 3, 0);
        let other_island = thing(e, 0x999, 4, 0);
        let entry = |e: &mut Engine, kind: u8| {
            let entry = e.mem.alloc(0x40);
            e.mem.set_u8(entry + 0xc, kind);
            entry
        };
        let (a1, a2, a3, a4) = (entry(e, 0), entry(e, 1), entry(e, 0), entry(e, 1));
        let blocks: Vec<u32> = [
            [a1, same_island, a2, other_island],
            [a3, first, a4, same_island],
        ]
        .iter()
        .map(|words| {
            let block = e.mem.alloc(0x20);
            for (index, word) in words.iter().enumerate() {
                e.mem.set_u32(block + 4 * index as u32, *word);
            }
            block
        })
        .collect();
        let used = Rc::new(Cell::new(0usize));
        let fills = blocks.clone();
        e.register_double(GET_COLLISION_ENTRIES_SORTED, move |e, a| {
            let block = fills[used.get()];
            used.set(used.get() + 1);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 4, 2);
            e.mem.set_u32(a[1] + 8, 2);
            Ret::default()
        });
        e.register_double(MAP_GET_WITH_DEFAULT, |_, a| Ret {
            eax: if a[1] == 0x42 { 7 } else { 0 },
            ..Ret::default()
        });
        plain(
            e,
            &[
                MEM_SET,
                ADD_REFERENCE,
                MAP_INSERT,
                CONTINUOUS_PROCESS_ENTRIES,
            ],
        );
        let entities = e.mem.alloc(8);
        e.mem.set_u32(entities, first);
        e.mem.set_u32(entities + 4, second);
        let references = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(
            0x00d0_3970,
            &args![sim, entities, 2u32, 0x7777_0000u32, references],
        );
        let log: Log = e.call_log.take().unwrap();
        // The flag block (16 bytes, rounded to 128) comes first, the list of entries after it.
        assert_eq!(
            calls_to(&log, MEM_SET),
            vec![args![top, 0u32, 0x10u32].to_vec()]
        );
        assert_eq!(e.mem.u8(top + 1), 1);
        assert_eq!(e.mem.u8(top + 2), 1);
        assert_eq!(e.mem.u8(top + 3), 0);
        let fills = calls_to(&log, GET_COLLISION_ENTRIES_SORTED);
        assert_eq!(fills.len(), 2);
        assert_eq!((fills[0][0], fills[1][0]), (first + 0x10, second + 0x10));
        assert_eq!(
            calls_to(&log, MAP_GET_WITH_DEFAULT),
            vec![
                args![references, 0x41u32, 0u32].to_vec(),
                args![references, 0x42u32, 0u32].to_vec()
            ]
        );
        assert_eq!(calls_to(&log, ADD_REFERENCE), vec![args![first].to_vec()]);
        assert_eq!(
            calls_to(&log, MAP_INSERT),
            vec![args![references, allocator, 0x41u32, first].to_vec()]
        );
        let list = top + 0x80;
        assert_eq!(e.mem.u32(list), a1);
        assert_eq!(
            calls_to(&log, CONTINUOUS_PROCESS_ENTRIES),
            vec![args![sim, list, 1u32, 0x7777_0000u32, island, 0u32].to_vec()]
        );
        // The array the last scan left is given back (2 pairs of words); the list and the flags
        // are on the temporary stack.
        assert_eq!(
            calls_to(&log, FREE_ARRAY),
            vec![args![allocator, blocks[1], 16u32].to_vec()]
        );
        assert_eq!(e.mem.u32(r.router), top);
    }

    struct Psi {
        r: Rig,
        entities: u32,
        items: [u32; 5],
        grown: Rc<RefCell<Vec<u32>>>,
        blocks: [u32; 2],
    }

    /// A hash map `{slots, size, highest slot}` of four slots with the entities `first` and
    /// `second` in slots 1 and 3 (key `0xffffffff` marks an empty one), where `first` has the
    /// entries `p1` to `p4` and `second` the entry `p5` (at rest, one with a
    /// type 5 entity whose island comes from the exclusive or of three objects).
    fn psi_rig() -> Psi {
        let mut r = rig();
        let world = r.world;
        let e = &mut r.e;
        e.map(0x0400_0000, 0x1000);
        e.map(0x0410_0000, 0x1000);
        e.map(0x0420_0000, 0x1000);
        e.map(0x0430_0000, 0x1000);
        e.mem.set_u32(0x0430_0000 + 0xcc, 0x200);
        let table = e.mem.alloc(0x2000);
        e.mem.set_u32(world + 0x80, table);
        // Type 0: no word (unlimited); type 1: word and flag; type 2: word without a flag.
        e.mem.set_u32(table + 0x1c30 + 0x40, 1);
        e.mem.set_u8(table + 0x1c34 + 0x40, 1);
        e.mem.set_u32(table + 0x1c30 + 0x80, 1);
        let first = e.mem.alloc(0x200);
        e.mem.set_u32(first + 0xcc, 0x100);
        let second = 0x0400_0000u32;
        e.mem.set_u8(second + 0xe8, 5);
        e.mem.set_u32(second + 0xcc, 0x999);
        let entry = |e: &mut Engine, kind: u8| {
            let entry = e.mem.alloc(0x40);
            e.mem.set_u8(entry + 0xc, kind);
            entry
        };
        let items = [
            entry(e, 0),
            entry(e, 1),
            entry(e, 2),
            entry(e, 0),
            entry(e, 0),
        ];
        e.mem.set_u32(items[4] + 0x14, 0x0410_0000);
        e.mem.set_u32(items[4] + 0x10, 0x0420_0000);
        let blocks = [e.mem.alloc(0x40), e.mem.alloc(0x40)];
        for (index, item) in items[..4].iter().enumerate() {
            e.mem.set_u32(blocks[0] + 8 * index as u32, *item);
        }
        e.mem.set_u32(blocks[1], items[4]);
        let slots = e.mem.alloc(0x40);
        for slot in 0..4 {
            e.mem.set_u32(slots + 8 * slot, 0xffff_ffff);
        }
        e.mem.set_u32(slots + 8, 5);
        e.mem.set_u32(slots + 12, first);
        e.mem.set_u32(slots + 24, 6);
        e.mem.set_u32(slots + 28, second);
        let entities = e.mem.alloc(0x10);
        e.mem.set_u32(entities, slots);
        e.mem.set_u32(entities + 4, 2);
        e.mem.set_u32(entities + 8, 3);
        let used = Rc::new(Cell::new(0usize));
        let fills = [(blocks[0], 4u32), (blocks[1], 1)];
        e.register_double(GET_COLLISION_ENTRIES_SORTED, move |e, a| {
            let (block, count) = fills[used.get()];
            used.set(used.get() + 1);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 4, count);
            e.mem.set_u32(a[1] + 8, count);
            Ret::default()
        });
        let present = items[3];
        e.register_double(MAP_GET_WITH_DEFAULT, move |_, a| Ret {
            eax: u32::from(a[1] == present),
            ..Ret::default()
        });
        let grown: Rc<RefCell<Vec<u32>>> = Rc::default();
        let sink = Rc::clone(&grown);
        e.register_double(ARRAY_GROW, move |e, a| {
            let data = e.mem.alloc(a[2] * 4);
            sink.borrow_mut().push(data);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        plain(
            e,
            &[
                MAP_RESERVE,
                MAP_INSERT,
                MAP_CLEAR_UNCONFIRMED,
                MAP_CLEAR_AND_DEALLOCATE,
                RELEASE_REFERENCE,
                0x00a2_9680,
                CONTINUOUS_PROCESS_ENTRIES,
                0x00c9_1120,
                0x00c9_1140,
            ],
        );
        Psi {
            r,
            entities,
            items,
            grown,
            blocks,
        }
    }

    #[test]
    fn recollision_groups_the_new_entries_by_island_and_runs_them() {
        let mut p = psi_rig();
        let (sim, world, top, allocator) = (p.r.sim, p.r.world, p.r.stack_top, p.r.allocator);
        let first = p.r.e.mem.u32(p.r.e.mem.u32(p.entities) + 12);
        let second = 0x0400_0000u32;
        let integrator = p.r.e.mem.alloc(0x80);
        p.r.e.mem.set_u32(world + 0x78, integrator);
        p.r.e.mem.set_u32(world + 0x8c, 1);
        p.r.e.mem.set_u32(world + 0xa4, 1);
        p.r.e.mem.set_u32(world + 0x90, 1);
        p.r.e.call_log = Some(vec![]);
        p.r.e.call(0x00d0_3c20, &args![sim, 0u32, p.entities]);
        let log: Log = p.r.e.call_log.take().unwrap();
        let e = &p.r.e;
        let [p1, p2, p3, p4, p5] = p.items;
        // The local map is reserved, searched with every entry, filled with the accepted ones.
        let reserve = calls_to(&log, MAP_RESERVE);
        assert_eq!(reserve.len(), 1);
        let map = reserve[0][0];
        assert_eq!(reserve[0][1..], [allocator, 0]);
        assert_eq!(
            calls_to(&log, MAP_GET_WITH_DEFAULT)
                .iter()
                .map(|call| (call[0], call[1], call[2]))
                .collect::<Vec<_>>(),
            [p1, p2, p3, p4, p5].map(|entry| (map, entry, 0)).to_vec()
        );
        assert_eq!(
            calls_to(&log, MAP_INSERT),
            [p1, p2, p5]
                .map(|entry| args![map, allocator, entry, 1u32].to_vec())
                .to_vec()
        );
        assert_eq!(
            calls_to(&log, RELEASE_REFERENCE),
            vec![args![first].to_vec(), args![second].to_vec()]
        );
        assert_eq!(
            calls_to(&log, MAP_CLEAR_UNCONFIRMED),
            vec![args![p.entities].to_vec()]
        );
        assert_eq!(
            calls_to(&log, MAP_CLEAR_AND_DEALLOCATE),
            vec![args![map, allocator].to_vec()]
        );
        assert_eq!(calls_to(&log, 0x00a2_9680), vec![args![map].to_vec()]);
        // The block holds the entries by island: 0x100 (two) and the one found through the
        // exclusive or for the type 5 entity (0x200).
        assert_eq!(
            [e.mem.u32(top), e.mem.u32(top + 4), e.mem.u32(top + 8)],
            [p1, p2, p5]
        );
        let integrator = e.mem.u32(world + 0x78);
        assert_eq!(
            calls_to(&log, CONTINUOUS_PROCESS_ENTRIES),
            vec![
                args![sim, top, 2u32, integrator, 0x100u32, 0u32].to_vec(),
                args![sim, top + 8, 1u32, integrator, 0x200u32, 0u32].to_vec()
            ]
        );
        assert_eq!(e.mem.u32(p.r.router), top);
        // The world's counter is back at 0 and its two callees ran.
        assert_eq!(e.mem.u32(world + 0x94), 0);
        assert_eq!(calls_to(&log, 0x00c9_1120), vec![args![world].to_vec()]);
        assert_eq!(calls_to(&log, 0x00c9_1140), vec![args![world].to_vec()]);
        // Both arrays grew once (pairs of words first) and every array is given back.
        let grown = p.grown.borrow();
        assert_eq!(grown.len(), 2);
        assert_eq!(
            calls_to(&log, FREE_ARRAY),
            vec![
                args![allocator, p.blocks[0], 32u32].to_vec(),
                args![allocator, p.blocks[1], 8u32].to_vec(),
                args![allocator, grown[1], 0x60u32].to_vec(),
                args![allocator, grown[0], 32u32].to_vec(),
            ]
        );
        assert_eq!(labels(&p.r, &[12, 12]), vec![0x010c_cd50, PROFILE_END]);
    }

    struct Finish {
        r: Rig,
        queue: u32,
        islands: [u32; 3],
        violations: u32,
        slot_queue: Rc<Cell<u32>>,
    }

    /// A simulation three of whose islands the world has (the first is not new: the simulation's
    /// `+0xb8` is 1), time 1.5 with a delta of 0.5, a world scope count of 1, a violation list
    /// with no violations and doubles for everything `finishMtStep` calls. The simulation's
    /// slot `0x18` records the job queue it finds at `+0xc0`.
    fn finish_rig() -> Finish {
        let mut r = rig();
        let (sim, world) = (r.sim, r.world);
        let e = &mut r.e;
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x18, SIMULATION_SLOT);
        e.mem.set_u32(sim, vtable);
        let array = e.mem.alloc(0x10);
        let islands = [e.mem.alloc(0x40), e.mem.alloc(0x40), e.mem.alloc(0x40)];
        for (index, island) in islands.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *island);
        }
        e.mem.set_u16(islands[0] + 0x20, 0x7777);
        e.mem.set_u32(world + 0x28, array);
        e.mem.set_u32(world + 0x2c, 3);
        e.mem.set_u32(sim + 0xb8, 1);
        let violations = e.mem.alloc(0x400);
        e.mem.set_u32(world + 0x1b8, violations);
        e.mem.set_u32(world + 0x94, 1);
        e.mem.set_f32(sim + 0x18, 1.5);
        e.mem.set_f32(sim + 0x1c, 0.5);
        e.mem.set_u8(sim + SIM_IN_STEP, 1);
        let queue = e.mem.alloc(0x10);
        let slot_queue = Rc::new(Cell::new(0xdead));
        let seen = Rc::clone(&slot_queue);
        e.register_double(SIMULATION_SLOT, move |e, a| {
            seen.set(e.mem.u32(a[0] + SIM_JOB_QUEUE));
            Ret::default()
        });
        e.register_double(0x00c9_3570, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x3f0),
            ..Ret::default()
        });
        answer(e, 0x00d0_4440, 0x77);
        plain(
            e,
            &[
                0x00c9_1180,
                0x00d0_49d0,
                0x00d0_4450,
                0x00dd_7d00,
                0x00c9_10b0,
                0x00d0_0aa0,
                0x00c9_1190,
                0x00d0_14f0,
                0x00cf_7130,
                0x00cb_4c60,
                0x00cc_1160,
                0x00d0_1df0,
                0x00c9_1120,
                0x00c9_1140,
                ENTER_CRITICAL_SECTION,
                LEAVE_CRITICAL_SECTION,
            ],
        );
        Finish {
            r,
            queue,
            islands,
            violations,
            slot_queue,
        }
    }

    /// Runs `finishMtStep` with the rig's queue and `pool`; returns the result and the call log.
    fn finish(f: &mut Finish, pool: u32) -> (u32, Log) {
        let (sim, queue) = (f.r.sim, f.queue);
        f.r.e.call_log = Some(vec![]);
        let got = f.r.e.call(0x00d0_3290, &args![sim, queue, pool]).u32();
        (got, f.r.e.call_log.take().unwrap())
    }

    /// A thread pool object whose slots `0xc` and `0x10` are doubles.
    fn pool_object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, POOL_SLOT_C);
        e.mem.set_u32(vtable + 0x10, POOL_SLOT_10);
        plain(e, &[POOL_SLOT_C, POOL_SLOT_10]);
        let pool = e.mem.alloc(8);
        e.mem.set_u32(pool, vtable);
        pool
    }

    #[test]
    fn a_finished_step_sorts_and_numbers_the_new_islands() {
        let mut f = finish_rig();
        let (sim, world) = (f.r.sim, f.r.world);
        let (got, log) = finish(&mut f, 0);
        let e = &f.r.e;
        assert_eq!(got, 0);
        let array = e.mem.u32(world + 0x28);
        assert_eq!(
            calls_to(&log, 0x00dd_7d00),
            vec![args![array + 4, 0u32, 1u32, 0x00d0_2220u32].to_vec()]
        );
        assert_eq!(
            f.islands.map(|island| e.mem.u16(island + 0x20)),
            [0x7777, 1, 2]
        );
        assert_eq!(e.mem.u8(sim + SIM_IN_STEP), 0);
        assert_eq!(e.mem.f32(sim + 0x18), 2.0);
        assert_eq!(e.mem.u32(sim + 0x28), 0);
        assert_eq!(e.mem.u32(world + 0x94), 0);
        assert_eq!(calls_to(&log, 0x00c9_1180), vec![args![world].to_vec()]);
        assert_eq!(calls_to(&log, 0x00c9_1190), vec![args![world].to_vec()]);
        assert_eq!(calls_to(&log, SIMULATION_SLOT), vec![args![sim].to_vec()]);
        assert_eq!(f.slot_queue.get(), 0);
        assert_eq!(
            order(
                &log,
                &[
                    0x00c9_1180,
                    0x00dd_7d00,
                    0x00c9_10b0,
                    0x00c9_1190,
                    SIMULATION_SLOT
                ]
            ),
            vec![
                0x00c9_1180,
                0x00dd_7d00,
                0x00c9_10b0,
                0x00c9_10b0,
                0x00c9_1190,
                SIMULATION_SLOT
            ]
        );
        // No queue operations without a pool, no pairs and no post collide callbacks.
        assert!(order(&log, &[0x00d0_4440, 0x00d0_49d0, 0x00d0_14f0, 0x00cf_7130]).is_empty());
        assert_eq!(labels(&f.r, &[12, 12]), vec![0x010c_c568, PROFILE_END]);
    }

    #[test]
    fn a_single_new_island_is_numbered_without_a_sort() {
        let mut f = finish_rig();
        let sim = f.r.sim;
        f.r.e.mem.set_u32(sim + 0xb8, 2);
        let (_, log) = finish(&mut f, 0);
        assert!(calls_to(&log, 0x00dd_7d00).is_empty());
        assert_eq!(
            f.islands.map(|island| f.r.e.mem.u16(island + 0x20)),
            [0x7777, 0, 2]
        );
    }

    #[test]
    fn at_most_128_constraint_violations_are_reported() {
        let mut f = finish_rig();
        let (world, violations) = (f.r.world, f.violations);
        f.r.e.mem.set_u32(violations, 2);
        f.r.e.mem.set_u32(violations + 4, 0xa1);
        f.r.e.mem.set_u32(violations + 8, 0xa2);
        let (_, log) = finish(&mut f, 0);
        assert_eq!(
            calls_to(&log, 0x00d0_0aa0),
            vec![
                args![world, 0xa1u32].to_vec(),
                args![world, 0xa2u32].to_vec()
            ]
        );
        f.r.e.mem.set_u32(violations, 0x90);
        for index in 0..0x90 {
            f.r.e
                .mem
                .set_u32(violations + 4 + 4 * index, 0x1000 + index);
        }
        let (_, log) = finish(&mut f, 0);
        let reported = calls_to(&log, 0x00d0_0aa0);
        assert_eq!(reported.len(), 0x80);
        assert_eq!(reported[0], args![world, 0x1000u32].to_vec());
        assert_eq!(reported[0x7f], args![world, 0x107fu32].to_vec());
        // Exactly 0x80 is reported in full, and a negative count not at all.
        f.r.e.mem.set_u32(violations, 0x80);
        assert_eq!(calls_to(&finish(&mut f, 0).1, 0x00d0_0aa0).len(), 0x80);
        f.r.e.mem.set_i32(violations, -3);
        assert!(calls_to(&finish(&mut f, 0).1, 0x00d0_0aa0).is_empty());
    }

    #[test]
    fn a_stopped_step_gives_the_time_back_and_returns_2() {
        let mut f = finish_rig();
        let (sim, world) = (f.r.sim, f.r.world);
        f.r.e.set_global(STOP_FLAG, 1u32);
        f.r.e.mem.set_u32(world + 0x3f0, 1);
        let (got, log) = finish(&mut f, 0);
        let e = &f.r.e;
        assert_eq!(got, 2);
        assert_eq!(e.mem.u32(sim + 0x28), 2);
        assert_eq!(e.mem.f32(sim + 0x18), 1.5);
        assert_eq!(f.slot_queue.get(), 0xdead);
        assert_eq!(calls_to(&log, 0x00c9_1190), vec![args![world].to_vec()]);
        assert_eq!(calls_to(&log, 0x00c9_10b0).len(), 1);
        assert_eq!(calls_to(&log, 0x00d0_1df0), vec![args![world].to_vec()]);
        assert_eq!(labels(&f.r, &[12, 12]), vec![0x010c_c568, PROFILE_END]);
    }

    #[test]
    fn the_post_collide_callbacks_get_the_info_of_the_step() {
        let mut f = finish_rig();
        let (sim, world, queue) = (f.r.sim, f.r.world, f.queue);
        let pool = pool_object(&mut f.r.e);
        f.r.e.mem.set_u32(world + 0x180, 1);
        f.r.e.mem.set_u8(world + 0xe4, 1);
        let seen = snapshots(&mut f.r.e, 0x00d0_14f0, 1, 16);
        let (got, log) = finish(&mut f, pool);
        let e = &f.r.e;
        assert_eq!(got, 0);
        assert_eq!(seen.borrow()[0], floats([1.5, 2.0, 0.5, 2.0]));
        assert_eq!(calls_to(&log, 0x00d0_14f0)[0][0], world);
        assert_eq!(f.slot_queue.get(), queue);
        assert_eq!(e.mem.u32(sim + SIM_JOB_QUEUE), 0);
        assert_eq!(
            order(
                &log,
                &[
                    0x00c9_1180,
                    0x00d0_4440,
                    0x00d0_49d0,
                    0x00d0_4450,
                    POOL_SLOT_C,
                    0x00dd_7d00,
                    0x00d0_14f0,
                    0x00c9_1190,
                    SIMULATION_SLOT,
                    POOL_SLOT_10
                ]
            ),
            vec![
                0x00c9_1180,
                0x00d0_4440,
                0x00d0_49d0,
                0x00d0_4450,
                POOL_SLOT_C,
                0x00dd_7d00,
                0x00d0_14f0,
                0x00c9_1190,
                SIMULATION_SLOT,
                0x00d0_49d0,
                0x00d0_4450,
                POOL_SLOT_10
            ]
        );
        assert_eq!(
            calls_to(&log, 0x00d0_49d0),
            vec![args![queue, 1u32].to_vec(), args![queue, 0u32].to_vec()]
        );
        assert_eq!(
            calls_to(&log, 0x00d0_4450),
            vec![args![queue, 2u32].to_vec(), args![queue, 0x77u32].to_vec()]
        );
        assert_eq!(
            calls_to(&log, POOL_SLOT_C),
            vec![args![pool, queue, 0xbu32].to_vec()]
        );
        assert_eq!(calls_to(&log, POOL_SLOT_10), vec![args![pool].to_vec()]);
        assert_eq!(
            labels(&f.r, &[12; 6]),
            vec![
                0x010c_c568,
                0x010c_c530,
                PROFILE_END,
                PROFILE_END,
                0x010c_ccf4,
                PROFILE_END
            ]
        );
    }

    #[test]
    fn a_pool_without_a_queue_is_not_prepared_but_still_released() {
        let mut f = finish_rig();
        let sim = f.r.sim;
        let pool = pool_object(&mut f.r.e);
        f.r.e.call_log = Some(vec![]);
        let got = f.r.e.call(0x00d0_3290, &args![sim, 0u32, pool]).u32();
        let log: Log = f.r.e.call_log.take().unwrap();
        assert_eq!(got, 0);
        assert!(order(&log, &[0x00d0_4440, POOL_SLOT_C]).is_empty());
        assert_eq!(
            calls_to(&log, 0x00d0_4450),
            vec![args![0u32, 0u32].to_vec()]
        );
        assert_eq!(calls_to(&log, POOL_SLOT_10), vec![args![pool].to_vec()]);
    }

    #[test]
    fn a_stop_during_the_simulation_slot_makes_the_result_3() {
        let mut f = finish_rig();
        let (sim, world) = (f.r.sim, f.r.world);
        f.r.e.register_double(SIMULATION_SLOT, |e, _| {
            e.set_global(STOP_FLAG, 1u32);
            Ret::default()
        });
        let (got, log) = finish(&mut f, 0);
        assert_eq!(got, 3);
        assert_eq!(f.r.e.mem.u32(sim + 0x28), 3);
        assert_eq!(f.r.e.mem.f32(sim + 0x18), 2.0);
        // Without a memory watchdog there is nothing more to do.
        assert!(calls_to(&log, 0x00d0_1df0).is_empty());
        f.r.e.set_global(STOP_FLAG, 0u32);
        f.r.e.mem.set_u32(world + 0x3f0, 1);
        f.r.e.mem.set_u32(world + 0x94, 1);
        let (got, log) = finish(&mut f, 0);
        assert_eq!(got, 3);
        assert_eq!(calls_to(&log, 0x00d0_1df0), vec![args![world].to_vec()]);
    }

    #[test]
    fn leaving_the_last_world_scope_runs_its_callbacks() {
        let mut f = finish_rig();
        let world = f.r.world;
        f.r.e.mem.set_u32(world + 0x8c, 1);
        f.r.e.mem.set_u32(world + 0xa4, 1);
        f.r.e.mem.set_u32(world + 0x90, 1);
        let (_, log) = finish(&mut f, 0);
        assert_eq!(calls_to(&log, 0x00c9_1120), vec![args![world].to_vec()]);
        assert_eq!(calls_to(&log, 0x00c9_1140), vec![args![world].to_vec()]);
        // A scope that is still held by someone else runs nothing.
        f.r.e.mem.set_u32(world + 0x94, 2);
        let (_, log) = finish(&mut f, 0);
        assert!(order(&log, &[0x00c9_1120, 0x00c9_1140]).is_empty());
        assert_eq!(f.r.e.mem.u32(world + 0x94), 1);
    }

    /// Broad phase handle `(handle, collidable, object)` for the pair lists: the handle's
    /// signed byte at `+5` leads to the collidable (whose number at `+0x14`, type byte at `+0x1a`
    /// and signed byte at `+0x10` lead to the object), the object has the island at `+0xcc`.
    fn pair_handle(e: &mut Engine, number: i32, kind: u8, island: u32) -> (u32, u32, u32) {
        let base = e.mem.alloc(0x200);
        let (handle, collidable, object) = (base + 0x80, base + 0x40, base + 0x60);
        e.mem.set_i8(handle + 5, -0x40);
        e.mem.set_i8(collidable + 0x10, 0x20);
        e.mem.set_i32(collidable + 0x14, number);
        e.mem.set_u8(collidable + 0x1a, kind);
        e.mem.set_u32(object + 0xcc, island);
        (handle, collidable, object)
    }

    /// Fills the simulation's list at `list` (`+0x6c` for the added pairs, `+0x90` for the removed
    /// ones) with the given pairs.
    fn set_pairs(e: &mut Engine, list: u32, pairs: &[(u32, u32)]) {
        let data = e.mem.alloc(8 * pairs.len() as u32 + 8);
        for (index, (first, second)) in pairs.iter().enumerate() {
            e.mem.set_u32(data + 8 * index as u32, *first);
            e.mem.set_u32(data + 8 * index as u32 + 4, *second);
        }
        e.mem.set_u32(list, data);
        e.mem.set_u32(list + 4, pairs.len() as u32);
    }

    struct PairWorld {
        holder: u32,
        dispatcher: u32,
        agent: u32,
        manager: u32,
        events: u32,
        // The handles of the pair to add: `a` has the lower number.
        a: (u32, u32, u32),
        b: (u32, u32, u32),
        ran: Snapshots,
        holder_words: Rc<RefCell<Vec<u32>>>,
    }

    /// Pending pairs for `finishMtStep`: one pair to add (given in the wrong order: the first
    /// handle has the higher number) whose collidables have the types 1 and 2 (the agent type
    /// byte 3 is at `dispatcher + 0x1bb0 + 2 + 10`, its creation word at `+ 0x1c30 + 3 * 0x40`)
    /// and whose islands differ, one pair to remove and one to remove whose agent is not found,
    /// an agent from the add double, a manager object with a slot `0x18` double, a Toi event
    /// array with room for 2, and doubles for the agent run (leaving the output at its buffer,
    /// the time at the maximum).
    fn pair_world(f: &mut Finish) -> PairWorld {
        let (sim, world) = (f.r.sim, f.r.world);
        let e = &mut f.r.e;
        let holder = e.mem.alloc(0x100);
        let dispatcher = e.mem.alloc(0x2000);
        e.mem.set_u32(holder, dispatcher);
        e.mem.set_u32(world + 0x78, holder);
        e.mem.set_u32(world + 0x80, dispatcher);
        e.mem.set_u8(dispatcher + 0x1bb0 + 2 + 10, 3);
        e.mem.set_u32(dispatcher + 0x1c30 + 3 * 0x40, 0xc0de);
        e.mem.set_u32(dispatcher + 0x1c20 + 0x40 + 0x10, 0xfeed);
        e.mem.set_u8(dispatcher + 0x1c34 + 0x40, 7);
        e.mem.set_u32(MAX_TIME_FLOAT, f32::MAX.to_bits());
        e.mem.set_f64(MAX_TIME_DOUBLE, f64::from(f32::MAX));
        let a = pair_handle(e, 1, 1, 0x100);
        let b = pair_handle(e, 2, 2, 0x200);
        set_pairs(e, sim + SIM_ADD_PAIRS, &[(b.0, a.0)]);
        let c = pair_handle(e, 5, 0, 0x300);
        let d = pair_handle(e, 6, 0, 0x300);
        let g = pair_handle(e, 7, 0, 0x300);
        set_pairs(e, sim + SIM_REMOVE_PAIRS, &[(c.0, d.0), (d.0, g.0)]);
        let manager_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(manager_vtable + 0x18, MANAGER_SLOT);
        let manager = e.mem.alloc(8);
        e.mem.set_u32(manager, manager_vtable);
        plain(e, &[MANAGER_SLOT]);
        let agent = e.mem.alloc(0x40);
        e.mem.set_u32(agent + 8, manager);
        e.mem.set_u8(agent + 0xc, 1);
        e.mem.set_u32(agent + 0x10, a.1);
        e.mem.set_u32(agent + 0x14, b.1);
        let events = e.mem.alloc(0xe0);
        e.mem.set_u32(sim + SIM_TOI_EVENTS, events);
        e.mem.set_u32(sim + SIM_TOI_CAPACITY, 2);
        let holder_words: Rc<RefCell<Vec<u32>>> = Rc::default();
        let sink = Rc::clone(&holder_words);
        e.register_double(ADD_AGENT, move |e, a| {
            sink.borrow_mut().push(e.mem.u32(a[2] + 0x14));
            Ret {
                eax: agent,
                ..Ret::default()
            }
        });
        let merged = a.2;
        e.register_double(0x00cb_4c60, move |e, _| {
            e.mem.set_u32(merged + 0xcc, 0x500);
            Ret::default()
        });
        let removed = d.1;
        let found = e.mem.alloc(8);
        e.register_double(FIND_AGENT, move |_, a| Ret {
            eax: if a[0] == removed { 0 } else { found },
            ..Ret::default()
        });
        let ran: Snapshots = Rc::default();
        let sink = Rc::clone(&ran);
        e.register_double(0x00cf_edd0, move |e, a| {
            let mut seen = e.mem.bytes(a[2], 0x30);
            seen.extend(e.mem.bytes(a[2] + 0x3030, 4));
            seen.extend(e.mem.bytes(a[2] + 0x3080, 4));
            seen.extend(e.mem.bytes(a[1] + 0x14, 4));
            seen.extend(e.mem.bytes(a[1] + 0x60, 4));
            sink.borrow_mut().push(seen);
            Ret::default()
        });
        PairWorld {
            holder,
            dispatcher,
            agent,
            manager,
            events,
            a,
            b,
            ran,
            holder_words,
        }
    }

    #[test]
    fn pending_pairs_are_sorted_merged_added_and_run() {
        let mut f = finish_rig();
        let (sim, world) = (f.r.sim, f.r.world);
        let p = pair_world(&mut f);
        let (got, log) = finish(&mut f, 0);
        let e = &f.r.e;
        assert_eq!(got, 0);
        // The duplicates between the lists, then the sort put the handles in order.
        assert_eq!(
            calls_to(&log, 0x00cf_7130),
            vec![args![sim + SIM_ADD_PAIRS, sim + SIM_REMOVE_PAIRS].to_vec()]
        );
        // The islands differ: they are merged, and the merged island is the owner.
        assert_eq!(
            calls_to(&log, 0x00cb_4c60),
            vec![args![world, 0x100u32, 0x200u32].to_vec()]
        );
        // The agent is added with the collidables in order, the creation word first.
        assert_eq!(
            calls_to(&log, ADD_AGENT),
            vec![args![p.a.1, p.b.1, p.holder].to_vec()]
        );
        assert_eq!(*p.holder_words.borrow(), vec![0xc0de]);
        assert_eq!(e.mem.u32(p.holder + 0x60), p.dispatcher + 0x1c60);
        assert_eq!(e.mem.u32(p.holder + 0x14), 0xfeed);
        let ran = calls_to(&log, 0x00cf_edd0);
        assert_eq!(ran.len(), 1);
        assert_eq!(
            (ran[0][0], ran[0][1], ran[0][3]),
            (p.agent, p.holder, p.manager)
        );
        let seen = &p.ran.borrow()[0];
        // The output block: the buffer pointer, the island, the maximum time, a clear count.
        assert_eq!(word(seen, 4), 0x500);
        assert_eq!(word(seen, 0), ran[0][2] + 0x10);
        assert_eq!(word(seen, 0x30), f32::MAX.to_bits());
        assert_eq!(word(seen, 0x34), 0);
        assert_eq!(word(seen, 0x38), 0xfeed);
        assert_eq!(word(seen, 0x3c), p.dispatcher + 0x1c60);
        // The output still points to its buffer and its time is the maximum: no manager call
        // and no Toi event.
        assert!(calls_to(&log, MANAGER_SLOT).is_empty());
        assert_eq!(e.mem.u32(sim + SIM_TOI_COUNT), 0);
        // The pairs to remove: both looked up, the one found removed.
        assert_eq!(calls_to(&log, FIND_AGENT).len(), 2);
        assert_eq!(calls_to(&log, 0x00cc_1160).len(), 1);
        assert_eq!(
            (
                e.mem.u32(sim + SIM_ADD_PAIRS + 4),
                e.mem.u32(sim + SIM_REMOVE_PAIRS + 4)
            ),
            (0, 0)
        );
        assert_eq!(e.mem.u32(p.events), 0);
        assert_eq!(
            labels(&f.r, &[12, 16, 12, 12, 12, 12, 12]),
            vec![
                0x010c_c568,
                0x010c_cd40,
                0x010c_cd24,
                0x010c_cd18,
                0x010c_cd0c,
                0x0107_6f38,
                PROFILE_END
            ]
        );
        assert_eq!(e.mem.u32(f.r.stream + 0x2c + 12), 0x010c_cd30);
    }

    #[test]
    fn a_new_agent_that_found_a_toi_is_reported_to_its_manager_and_queued() {
        let mut f = finish_rig();
        let sim = f.r.sim;
        let p = pair_world(&mut f);
        let toi = 0.5f32.to_bits();
        f.r.e.register_double(0x00cf_edd0, move |e, a| {
            // The agent wrote its pair somewhere else and found a collision at time 0.5.
            e.mem.set_u32(a[2], a[2] + 0x2000);
            e.mem.set_u32(a[2] + 0x3030, toi);
            Ret::default()
        });
        let (_, log) = finish(&mut f, 0);
        let e = &f.r.e;
        let out = calls_to(&log, 0x00cf_edd0)[0][2];
        assert_eq!(
            calls_to(&log, MANAGER_SLOT),
            vec![args![p.manager, p.a.1, p.b.1, p.holder, out].to_vec()]
        );
        assert_eq!(
            order(
                &log,
                &[
                    0x00cf_edd0,
                    MANAGER_SLOT,
                    ENTER_CRITICAL_SECTION,
                    LEAVE_CRITICAL_SECTION
                ]
            ),
            vec![
                0x00cf_edd0,
                MANAGER_SLOT,
                ENTER_CRITICAL_SECTION,
                LEAVE_CRITICAL_SECTION
            ]
        );
        assert_eq!(
            calls_to(&log, ENTER_CRITICAL_SECTION),
            vec![args![sim + 0x100].to_vec()]
        );
        assert_eq!(e.mem.u32(sim + SIM_TOI_COUNT), 1);
        assert_eq!(e.mem.u32(p.events), toi);
        assert_eq!(e.mem.u8(p.events + 8), 7);
        assert_eq!(e.mem.u32(p.events + 0xc), p.a.2);
        assert_eq!(e.mem.u32(p.events + 0x10), p.b.2);
    }

    #[test]
    fn an_agent_that_was_not_added_is_not_run() {
        let mut f = finish_rig();
        let p = pair_world(&mut f);
        f.r.e.register_double(ADD_AGENT, |_, _| Ret::default());
        let (_, log) = finish(&mut f, 0);
        assert!(calls_to(&log, 0x00cf_edd0).is_empty());
        assert_eq!(calls_to(&log, ADD_AGENT).len(), 1);
        assert_eq!(p.ran.borrow().len(), 0);
    }

    #[test]
    fn pairs_without_an_agent_type_or_with_equal_islands_only_skip_the_merge() {
        let mut f = finish_rig();
        let sim = f.r.sim;
        let p = pair_world(&mut f);
        f.r.e.mem.set_u8(p.dispatcher + 0x1bb0 + 2 + 10, 0);
        f.r.e.mem.set_u32(p.b.2 + 0xcc, 0x100);
        let (_, log) = finish(&mut f, 0);
        assert!(order(&log, &[0x00cb_4c60, ADD_AGENT, 0x00cf_edd0]).is_empty());
        assert_eq!(f.r.e.mem.u32(sim + SIM_ADD_PAIRS + 4), 0);
        assert_eq!(f.r.e.mem.u32(p.holder + 0x14), 0);
    }

    #[test]
    fn a_stop_ends_the_adding_of_pairs_early() {
        let mut f = finish_rig();
        let sim = f.r.sim;
        let p = pair_world(&mut f);
        let second = pair_handle(&mut f.r.e, 3, 1, 0x700);
        let third = pair_handle(&mut f.r.e, 4, 2, 0x800);
        set_pairs(
            &mut f.r.e,
            sim + SIM_ADD_PAIRS,
            &[(p.a.0, p.b.0), (second.0, third.0)],
        );
        f.r.e.set_global(STOP_FLAG, 1u32);
        let (got, log) = finish(&mut f, 0);
        assert_eq!(got, 2);
        assert_eq!(calls_to(&log, 0x00cb_4c60).len(), 1);
        assert_eq!(calls_to(&log, ADD_AGENT).len(), 1);
        assert_eq!(f.r.e.mem.u32(sim + SIM_ADD_PAIRS + 4), 0);
    }

    #[test]
    fn the_entries_array_of_the_job_function_is_released_without_bit_31() {
        // `00d03030` makes its capacity word from `count | 0x80000000`, so the exe never gets
        // here; the release itself takes 4 bytes per element and skips a word with bit 31 set.
        let mut r = rig();
        let (allocator, entries) = (r.allocator, 0x1000_0000u32);
        r.e.call_log = Some(vec![]);
        release_block(&mut r.e, entries, 20, 4);
        let log: Log = r.e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FREE_ARRAY),
            vec![args![allocator, entries, 80u32].to_vec()]
        );
        r.e.call_log = Some(vec![]);
        release_block(&mut r.e, entries, 20 | DONT_DEALLOCATE_FLAG, 4);
        let log: Log = r.e.call_log.take().unwrap();
        assert!(calls_to(&log, FREE_ARRAY).is_empty());
    }

    #[test]
    fn the_flag_block_of_the_one_island_function_is_released_without_bit_31() {
        // `00d03970` makes its capacity word from `size | 0x80000000`, so the exe never gets
        // here; the release takes one byte per element and masks off the two flag bits.
        let mut r = rig();
        let (allocator, flags) = (r.allocator, 0x1000_0000u32);
        r.e.call_log = Some(vec![]);
        release_block(&mut r.e, flags, 0x4000_0010, 1);
        let log: Log = r.e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FREE_ARRAY),
            vec![args![allocator, flags, 0x10u32].to_vec()]
        );
        r.e.call_log = Some(vec![]);
        release_block(&mut r.e, flags, 0x10 | DONT_DEALLOCATE_FLAG, 1);
        let log: Log = r.e.call_log.take().unwrap();
        assert!(calls_to(&log, FREE_ARRAY).is_empty());
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    /// An object whose key sits at +0xd4 of the object behind two levels of pointers at +0x48.
    fn keyed_object(e: &mut Engine, key: u32) -> u32 {
        let holder = e.mem.alloc(0x60);
        let block = e.mem.alloc(0x10);
        let object = e.mem.alloc(0xe0);
        e.mem.set_u32(holder + 0x48, block);
        e.mem.set_u32(block, object);
        e.mem.set_u32(object + 0xd4, key);
        holder
    }

    /// The byte at `out` is 1 only when the first key is below the second, unsigned.
    #[test]
    fn fn_00d02220_writes_whether_the_first_key_is_below_the_second() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        for (a, b, expected) in [
            (1, 2, 1),
            (2, 1, 0),
            (2, 2, 0),
            (0xffff_ffff, 1, 0),
            (1, 0xffff_ffff, 1),
        ] {
            let first = keyed_object(&mut e, a);
            let second = keyed_object(&mut e, b);
            e.mem.set_u8(out, 0x55);
            assert_eq!(e.call(0x00d0_2220, &args![out, first, second]).u32(), out);
            assert_eq!(e.mem.u8(out), expected, "{a:#x} against {b:#x}");
            assert_eq!(e.mem.u8(out + 1), 0);
        }
    }
}
