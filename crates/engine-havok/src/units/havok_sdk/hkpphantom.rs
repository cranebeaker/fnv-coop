//! `Havok SDK/hkpphantom.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A phantom (`hkpPhantom`, 0xa4 bytes) has its world at +8, its collidable at +0x10 (the
//! broad-phase handle at +0x24 and the quantized bounding-volume data at +0x30..+0x4f),
//! `m_overlapListeners` (an `hkArray`: data +0x8c, size +0x90, capacity word +0x94) and
//! `m_phantomListeners` (data +0x98, size +0x9c, capacity word +0xa0). Removed listeners leave
//! null entries that `removeNullOverlapListeners` and its twin for the phantom listeners sweep
//! away afterwards.
//!
//! SSE code computes lane by lane in `f32`; floats copied through `FLD`/`FSTP` go through
//! [`x87_quiet`]; `FISTP` rounds to nearest (even) and answers `0x80000000` for a NaN or an
//! out-of-range value, assuming the default x87 control word.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkMemoryRouter` lookup (`00c85750`, no arguments): its first word is the top of the
/// router's temporary stack, its field `+0x10` the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The vectors the bounding data is clamped between (upper, lower), four lanes each.
const BOUNDING_UPPER_CLAMP: u32 = 0x011b_6d70;
const BOUNDING_LOWER_CLAMP: u32 = 0x0126_7e30;
/// `hkpBroadPhase::lockImplementation` and `::unlockImplementation` (Xbox PDB; `this` = the
/// broad phase).
const BROAD_PHASE_LOCK: u32 = 0x00d9_7050;
const BROAD_PHASE_UNLOCK: u32 = 0x00d9_7060;
/// Queues a world operation (`hkpworld.cpp`, no name in the engine map; `this` = the world,
/// one argument: the operation record).
const WORLD_QUEUE_OPERATION: u32 = 0x00c9_1160;
/// `hkpTypedBroadPhaseDispatcher::removeDuplicates` (Xbox PDB, cdecl `(added pairs, removed
/// pairs)`), `::removePairs` (`this` = dispatcher, `(pairs, count)`) and `::addPairs`
/// (`this` = dispatcher, `(pairs, count, filter)`).
const REMOVE_DUPLICATES: u32 = 0x00cf_7130;
const REMOVE_PAIRS: u32 = 0x00cf_70f0;
const ADD_PAIRS: u32 = 0x00cf_7080;
/// Two `hkpworld.cpp` functions the world runs when its lock count drops to zero (`this` = the
/// world, no arguments; no names in the engine map).
const WORLD_EXECUTE_PENDING_OPERATIONS: u32 = 0x00c9_1120;
const WORLD_EXECUTE_PENDING_BODY_OPERATIONS: u32 = 0x00c9_1140;
/// The operation record type `updateBroadPhase` queues while the world is locked.
const OPERATION_UPDATE_PHANTOM: u8 = 0x11;

/// Four `float` lanes.
type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

/// `MINPS dst, src` per lane: `dst` when it is smaller, else `src`.
fn minps(dst: f32, src: f32) -> f32 {
    if dst < src {
        dst
    } else {
        src
    }
}

/// `MAXPS dst, src` per lane: `dst` when it is larger, else `src`.
fn maxps(dst: f32, src: f32) -> f32 {
    if dst > src {
        dst
    } else {
        src
    }
}

/// `FISTP dword` of a `float` already on the x87 stack: nearest (even), `0x80000000` for a NaN
/// or a value outside the `int` range.
fn fistp(value: f32) -> i32 {
    crate::x87::fistp(f64::from(value))
}

/// The memory router's allocator (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Quantizes `offset + bounds` lane by lane: `(offset + bounds) * scale`, clamped to the two
/// clamp vectors, rounded with `FISTP`, and stored as four `int`s at `to`.
fn store_quantized(e: &mut Engine, to: u32, offset: Vec4, bounds: Vec4, scale: Vec4) {
    let upper = load_vec(e, BOUNDING_UPPER_CLAMP);
    let lower = load_vec(e, BOUNDING_LOWER_CLAMP);
    for lane in 0..4 {
        let scaled = (offset[lane] + bounds[lane]) * scale[lane];
        let clamped = maxps(minps(scaled, upper[lane]), lower[lane]);
        let quantized = fistp(x87_quiet(clamped));
        e.mem.set_i32(to + 4 * lane as u32, quantized);
    }
}

// Translated from 00d07950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantom::setBoundingVolumeData` (Xbox PDB): quantizes the AABB at `aabb` (minimum at +0,
/// maximum at +0x10) with the world's collision input (`world + 0x78`: offsets at +0x20 and
/// +0x30, scale at +0x40) into the phantom's bounding-volume data: the minimum's four `int`s
/// at +0x30, the maximum's at +0x40. Then clears the bytes +0x3c..+0x3f and +0x4c..+0x4e (the
/// fourth lanes keep only the top byte at +0x4f).
pub fn hkp_phantom_set_bounding_volume_data(e: &mut Engine, this: Ptr, aabb: Ptr) {
    let me = this.addr();
    let world = e.mem.u32(me + 8);
    let input = e.mem.u32(world + 0x78);
    let scale = load_vec(e, input + 0x40);
    let minimum_offset = load_vec(e, input + 0x20);
    let minimum = load_vec(e, aabb.addr());
    store_quantized(e, me + 0x30, minimum_offset, minimum, scale);
    let maximum_offset = load_vec(e, input + 0x30);
    let maximum = load_vec(e, aabb.addr() + 0x10);
    store_quantized(e, me + 0x40, maximum_offset, maximum, scale);
    for offset in [0x3f, 0x3c, 0x3d, 0x3e, 0x4c, 0x4d, 0x4e] {
        e.mem.set_u8(me + offset, 0);
    }
}

/// `hkArray::removeAt` as the code inlines it for an array of pointers: the size drops by one
/// and the words behind `index` move down one place (with `index == -1` the word before the
/// data is overwritten, as the code does).
fn remove_word_at(e: &mut Engine, array: u32, index: i32) {
    let size = e.mem.i32(array + 4).wrapping_sub(1);
    e.mem.set_i32(array + 4, size);
    let bytes = size.wrapping_sub(index).wrapping_mul(4);
    let mut slot = e
        .mem
        .u32(array)
        .wrapping_add((index as u32).wrapping_mul(4));
    if bytes > 0 {
        let words = ((bytes - 1) as u32 >> 2) + 1;
        for _ in 0..words {
            let next = e.mem.u32(slot.wrapping_add(4));
            e.mem.set_u32(slot, next);
            slot = slot.wrapping_add(4);
        }
    }
}

/// Removes every null entry of the pointer array whose data pointer is at `array` and size at
/// `array + 4`, last to first.
fn remove_null_entries(e: &mut Engine, array: u32) {
    let mut index = e.mem.i32(array + 4).wrapping_sub(1);
    while index >= 0 {
        let data = e.mem.u32(array);
        if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) == 0 {
            remove_word_at(e, array, index);
        }
        index -= 1;
    }
}

// Translated from 00d07a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantom::removeNullOverlapListeners` (Xbox PDB): removes the null entries of
/// `m_overlapListeners` (data +0x8c, size +0x90), last to first.
pub fn hkp_phantom_remove_null_overlap_listeners(e: &mut Engine, this: Ptr) {
    remove_null_entries(e, this.addr() + 0x8c);
}

// Translated from 00d07af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Xbox name unknown): removes the null entries of `m_phantomListeners` (data +0x98,
/// size +0x9c), last to first.
pub fn fn_00d07af0(e: &mut Engine, this: Ptr) {
    remove_null_entries(e, this.addr() + 0x98);
}

// Translated from 00d07c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Xbox name unknown): removes `listener` from `m_phantomListeners`, shifting the
/// following entries down. A listener that is not in the list is treated as index -1: the size
/// still drops by one and the shift starts one word before the data.
pub fn fn_00d07c10(e: &mut Engine, this: Ptr, listener: u32) {
    let array = this.addr() + 0x98;
    let count = e.mem.i32(array + 4);
    let data = e.mem.u32(array);
    let mut index = -1;
    for candidate in 0..count.max(0) {
        if e.mem.u32(data + candidate as u32 * 4) == listener {
            index = candidate;
            break;
        }
    }
    remove_word_at(e, array, index);
}

/// Calls virtual slot `slot` of every non-null phantom listener (last to first, the list
/// re-read each time) with the phantom as the argument, then removes the null entries.
fn fire_phantom_listeners(e: &mut Engine, this: Ptr, slot: u32) {
    let me = this.addr();
    let mut index = e.mem.i32(me + 0x9c).wrapping_sub(1);
    while index >= 0 {
        let data = e.mem.u32(me + 0x98);
        let listener = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
        if listener != 0 {
            e.vcall(listener, slot, &args![this]);
        }
        index -= 1;
    }
    fn_00d07af0(e, this);
}

// Translated from 00d07b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantom::firePhantomRemoved` (Xbox PDB): calls slot `8` of each phantom listener with
/// the phantom, then removes the null listeners.
pub fn hkp_phantom_fire_phantom_removed(e: &mut Engine, this: Ptr) {
    fire_phantom_listeners(e, this, 8);
}

// Translated from 00d07b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantom::firePhantomAdded` (Xbox PDB): calls slot `4` of each phantom listener with the
/// phantom, then removes the null listeners.
pub fn hkp_phantom_fire_phantom_added(e: &mut Engine, this: Ptr) {
    fire_phantom_listeners(e, this, 4);
}

// Translated from 00d07da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Xbox name unknown): appends `listener` to `m_phantomListeners`, first growing the
/// array (`00c90940`) when its size equals its capacity.
pub fn fn_00d07da0(e: &mut Engine, this: Ptr, listener: u32) {
    let array = this.addr() + 0x98;
    let allocator = router_allocator(e);
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), listener);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Takes `count` 8-byte pair slots (rounded up to 128 bytes) off the router's temporary stack
/// and describes them as an array in the 16-byte block at `array`: data pointer, size 0,
/// capacity word `count | 0x80000000` (storage not owned), and a copy of the data pointer for
/// the release.
fn open_pair_array(e: &mut Engine, array: u32, count: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let top = e.mem.u32(router);
    let used = count.wrapping_mul(8).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, top.wrapping_add(used));
    e.mem.set_u32(array, top);
    e.mem.set_u32(array + 8, count | 0x8000_0000);
    e.mem.set_u32(array + 0xc, top);
}

/// Gives the pair array back: the router's stack top returns to the array's first block, and
/// storage the array owns (capacity word without bit 31) goes back to the heap allocator
/// (vtable slot 0x10, `(block, bytes)`).
fn close_pair_array(e: &mut Engine, array: u32) {
    let start = e.mem.u32(array + 0xc);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, start);
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(8)],
        );
    }
}

// Translated from 00d07ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantom::updateBroadPhase` (Xbox PDB): with a world, queues the update as operation
/// `0x11` (record: type byte, phantom, AABB) while the world's critical-operation lock counts
/// (+0x94 and +0x98) are not both zero. Otherwise takes the lock (+0x94 += 1), takes two pair
/// arrays of `world + 0x33c` slots off the router's temporary stack, locks the broad phase
/// (`world + 0x58`) when its field +0x10 is set, calls the broad phase's slot `0x28`
/// (`updateAabbs` in the Xbox PDB) with this phantom's handle (+0x24), `aabb`, 1 and the two
/// arrays, and, when either array has pairs, removes duplicates, removes the second array's
/// pairs and adds the first's (through the dispatcher at `world + 0x64`, with the collision
/// filter's `+8` part or null) and sweeps the null overlap listeners. Then unlocks the broad
/// phase, requantizes the bounding data (`hkp_phantom_set_bounding_volume_data`), tells the
/// kd-tree manager (`world + 0x5c`, slot `0x14`, argument 0), drops the lock and, at zero with
/// no blocked operations, runs the pending operations (`00c91120` when `world + 0x8c`,
/// `00c91140` when the queue count is 1 and `world + 0x90`), and gives the arrays back. The
/// operation record's unused bytes are zero (the code leaves stack garbage there).
pub fn hkp_phantom_update_broad_phase(e: &mut Engine, this: Ptr, aabb: Ptr) {
    // The exe's frame (`SUB ESP,0x30`, three pushes): 0x3c bytes, the queued operation at +0x10
    // (00d07ee6..00d07ef4; two pushes on that path), the handle slot at +0xc, the pair arrays at
    // +0x1c and +0x2c (00d07f17..00d07fc5).
    e.with_stack(0x3c, |e, frame| {
        let frame = frame.addr();
        let me = this.addr();
        let world = e.mem.u32(me + 8);
        if world == 0 {
            return;
        }
        if e.mem
            .u32(world + 0x98)
            .wrapping_add(e.mem.u32(world + 0x94))
            != 0
        {
            let record = frame + 0x10;
            e.mem.set_u8(record, OPERATION_UPDATE_PHANTOM);
            e.mem.set_u32(record + 4, me);
            e.mem.set_u32(record + 8, aabb.addr());
            e.call(WORLD_QUEUE_OPERATION, &args![world, record]);
            return;
        }
        let locks = e.mem.u32(world + 0x94);
        e.mem.set_u32(world + 0x94, locks.wrapping_add(1));
        // Locals: the two pair arrays (16 bytes each) and the slot holding the handle pointer.
        let first = frame + 0x2c;
        let second = frame + 0x1c;
        let handles = frame + 0xc;
        let update_size = e.mem.u32(e.mem.u32(me + 8) + 0x33c);
        open_pair_array(e, first, update_size);
        let update_size = e.mem.u32(e.mem.u32(me + 8) + 0x33c);
        open_pair_array(e, second, update_size);
        e.mem.set_u32(handles, me + 0x24);
        let broad_phase = e.mem.u32(e.mem.u32(me + 8) + 0x58);
        if e.mem.u32(broad_phase + 0x10) != 0 {
            e.call(BROAD_PHASE_LOCK, &args![broad_phase]);
        }
        let broad_phase = e.mem.u32(e.mem.u32(me + 8) + 0x58);
        e.vcall(
            broad_phase,
            0x28,
            &args![handles, aabb, 1u32, first, second],
        );
        if e.mem.u32(first + 4) != 0 || e.mem.u32(second + 4) != 0 {
            e.call(REMOVE_DUPLICATES, &args![first, second]);
            let world_now = e.mem.u32(me + 8);
            let dispatcher = e.mem.u32(world_now + 0x64);
            let removed_data = e.mem.u32(second);
            let removed_count = e.mem.u32(second + 4);
            e.call(
                REMOVE_PAIRS,
                &args![dispatcher, removed_data, removed_count],
            );
            let world_now = e.mem.u32(me + 8);
            let filter = e.mem.u32(world_now + 0x7c);
            let listener = if filter == 0 { 0 } else { filter + 8 };
            let dispatcher = e.mem.u32(world_now + 0x64);
            let added_data = e.mem.u32(first);
            let added_count = e.mem.u32(first + 4);
            e.call(
                ADD_PAIRS,
                &args![dispatcher, added_data, added_count, listener],
            );
            hkp_phantom_remove_null_overlap_listeners(e, this);
        }
        let broad_phase = e.mem.u32(e.mem.u32(me + 8) + 0x58);
        if e.mem.u32(broad_phase + 0x10) != 0 {
            e.call(BROAD_PHASE_UNLOCK, &args![broad_phase]);
        }
        hkp_phantom_set_bounding_volume_data(e, this, aabb);
        let world = e.mem.u32(me + 8);
        let tree = e.mem.u32(world + 0x5c);
        if tree != 0 {
            e.vcall(tree, 0x14, &args![0u32]);
        }
        let world = e.mem.u32(me + 8);
        let locks = e.mem.u32(world + 0x94).wrapping_sub(1);
        e.mem.set_u32(world + 0x94, locks);
        if locks == 0 && e.mem.u8(world + 0x9c) == 0 {
            if e.mem.u32(world + 0x8c) != 0 {
                e.call(WORLD_EXECUTE_PENDING_OPERATIONS, &args![world]);
            }
            if e.mem.u32(world + 0xa4) == 1 && e.mem.u32(world + 0x90) != 0 {
                e.call(WORLD_EXECUTE_PENDING_BODY_OPERATIONS, &args![world]);
            }
        }
        close_pair_array(e, second);
        e.mem.set_u32(second, 0);
        e.mem.set_u32(second + 8, 0x8000_0000);
        close_pair_array(e, first);
    })
}

/// `hkpWorldObject::~hkpWorldObject` (`00c9d3b0`, `this`) and
/// `hkpWorldObject::calcContentStatistics` (`00cd2b00`, `this`, collector, class record).
const WORLD_OBJECT_DESTRUCT: u32 = 0x00c9_d3b0;
const WORLD_OBJECT_CALC_CONTENT_STATISTICS: u32 = 0x00cd_2b00;
/// `hkStatisticsCollector::getAllocatedSize` (`00d06f80`, cdecl `(pointer, bytes)`).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// The class record `calcContentStatistics` hands on (`01268808`) and the vtable
/// `~hkpPhantom` leaves in the object (`010c7214`, the deleting destructor's table).
const PHANTOM_CLASS_RECORD: u32 = 0x0126_8808;
const PHANTOM_VTABLE: u32 = 0x010c_7214;
/// The statistics names of the two listener arrays: `"OvrlapLisPtr"` and `"PhantmLisPtr"`.
const NAME_OVERLAP_LISTENERS: u32 = 0x010c_cf68;
const NAME_PHANTOM_LISTENERS: u32 = 0x010c_cf58;

/// Calls virtual slot `slot` of every non-null phantom listener (last to first, the list
/// re-read each time) with the phantom as the argument.
fn call_phantom_listeners(e: &mut Engine, this: Ptr, slot: u32) {
    let me = this.addr();
    let mut index = e.mem.i32(me + 0x9c).wrapping_sub(1);
    while index >= 0 {
        let data = e.mem.u32(me + 0x98);
        let listener = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
        if listener != 0 {
            e.vcall(listener, slot, &args![this]);
        }
        index -= 1;
    }
}

/// Returns the storage of the pointer array at `array` to the memory router's allocator (slot
/// `0x10`, `(data, capacity * 4)`) when its capacity word is not negative, after setting its
/// size to 0, and leaves it empty with null data and the capacity word `0x80000000`.
fn release_pointer_array(e: &mut Engine, array: u32) {
    let allocator = router_allocator(e);
    e.mem.set_u32(array + 4, 0);
    let capacity = e.mem.u32(array + 8);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// The statistics record for one listener array: when the capacity word's top bit is clear or
/// the collector's byte at `+0x10` is set, the collector (slot `0x14`) gets `(3, name, data,
/// size * 4, allocated size)` where the allocated size is
/// `getAllocatedSize(data, capacity * 4)`.
fn report_pointer_array(e: &mut Engine, collector: u32, array: u32, name: u32) {
    let capacity = e.mem.u32(array + 8);
    if (capacity as i32) >= 0 || e.mem.u8(collector + 0x10) != 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        let allocated = e.call(GET_ALLOCATED_SIZE, &args![data, bytes]).u32();
        let data = e.mem.u32(array);
        let used = e.mem.u32(array + 4).wrapping_mul(4);
        e.vcall(collector, 0x14, &args![3u32, name, data, used, allocated]);
    }
}

// Translated from 00d07910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Xbox name unknown; the destructor runs it first): calls virtual slot `0x10` of
/// every non-null phantom listener (`m_phantomListeners`, last to first) with the phantom,
/// without sweeping the null entries afterwards.
pub fn fn_00d07910(e: &mut Engine, this: Ptr) {
    call_phantom_listeners(e, this, 0x10);
}

// Translated from 00d07bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantom::firePhantomShapeSet` (engine map): calls slot `0xc` of each phantom listener
/// with the phantom, then removes the null listeners (the exe jumps to `00d07af0` for that).
pub fn hkp_phantom_fire_phantom_shape_set(e: &mut Engine, this: Ptr) {
    call_phantom_listeners(e, this, 0xc);
    fn_00d07af0(e, this);
}

// Translated from 00d07c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantom::removePhantomOverlapListener` (engine map): removes `listener` from
/// `m_overlapListeners` (data +0x8c, size +0x90), shifting the following entries down. A
/// listener that is not in the list is treated as index -1, as `fn_00d07c10` does for the
/// phantom listeners.
pub fn hkp_phantom_remove_phantom_overlap_listener(e: &mut Engine, this: Ptr, listener: u32) {
    let array = this.addr() + 0x8c;
    let count = e.mem.i32(array + 4);
    let data = e.mem.u32(array);
    let mut index = -1;
    for candidate in 0..count.max(0) {
        if e.mem.u32(data + candidate as u32 * 4) == listener {
            index = candidate;
            break;
        }
    }
    remove_word_at(e, array, index);
}

// Translated from 00d07cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantom::calcContentStatistics` (engine map): `ECX` = phantom and two stack words, the
/// collector and a word the function does not read. Runs
/// `hkpWorldObject::calcContentStatistics` (`00cd2b00`) with `(this, collector, 01268808)`, then
/// reports `m_overlapListeners` (name `"OvrlapLisPtr"`) and `m_phantomListeners` (name
/// `"PhantmLisPtr"`) to the collector, each as `report_pointer_array` describes.
pub fn hkp_phantom_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: u32,
    _unused: u32,
) {
    e.call(
        WORLD_OBJECT_CALC_CONTENT_STATISTICS,
        &args![this, collector, PHANTOM_CLASS_RECORD],
    );
    report_pointer_array(e, collector, this.addr() + 0x8c, NAME_OVERLAP_LISTENERS);
    report_pointer_array(e, collector, this.addr() + 0x98, NAME_PHANTOM_LISTENERS);
}

// Translated from 00d07de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantom::addPhantomOverlapListener` (engine map): appends `listener` to
/// `m_overlapListeners`, first growing the array (`00c90940`) when its size equals its
/// capacity.
pub fn hkp_phantom_add_phantom_overlap_listener(e: &mut Engine, this: Ptr, listener: u32) {
    let array = this.addr() + 0x8c;
    let allocator = router_allocator(e);
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), listener);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00d07e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantom::deallocateInternalArrays` (engine map, `__fastcall`): releases the storage of
/// `m_overlapListeners` when it is empty and then that of `m_phantomListeners` when it is empty
/// (see `release_pointer_array`); a list with entries is left alone.
pub fn hkp_phantom_deallocate_internal_arrays(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    if e.mem.u32(me + 0x90) == 0 {
        release_pointer_array(e, me + 0x8c);
    }
    if e.mem.u32(me + 0x9c) == 0 {
        release_pointer_array(e, me + 0x98);
    }
}

// Translated from 00d08130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantom::~hkpPhantom` (engine map, `__fastcall`): sets the vtable `010c7214`, tells the
/// phantom listeners (`fn_00d07910`), releases `m_phantomListeners` and then
/// `m_overlapListeners` (see `release_pointer_array`) and ends in the base destructor
/// (`hkpWorldObject::~hkpWorldObject`, `00c9d3b0`).
pub fn hkp_phantom_destruct(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, PHANTOM_VTABLE);
    fn_00d07910(e, this);
    release_pointer_array(e, me + 0x98);
    release_pointer_array(e, me + 0x8c);
    e.call(WORLD_OBJECT_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_7950, hkp_phantom_set_bounding_volume_data(Ptr, Ptr)),
        entry!(0x00d0_7a90, hkp_phantom_remove_null_overlap_listeners(Ptr)),
        entry!(0x00d0_7af0, fn_00d07af0(Ptr)),
        entry!(0x00d0_7c10, fn_00d07c10(Ptr, u32)),
        entry!(0x00d0_7b50, hkp_phantom_fire_phantom_removed(Ptr)),
        entry!(0x00d0_7b90, hkp_phantom_fire_phantom_added(Ptr)),
        entry!(0x00d0_7da0, fn_00d07da0(Ptr, u32)),
        entry!(0x00d0_7ec0, hkp_phantom_update_broad_phase(Ptr, Ptr)),
        entry!(0x00d0_7910, fn_00d07910(Ptr)),
        entry!(0x00d0_7bd0, hkp_phantom_fire_phantom_shape_set(Ptr)),
        entry!(
            0x00d0_7c80,
            hkp_phantom_remove_phantom_overlap_listener(Ptr, u32)
        ),
        entry!(
            0x00d0_7cf0,
            hkp_phantom_calc_content_statistics(Ptr, u32, u32)
        ),
        entry!(
            0x00d0_7de0,
            hkp_phantom_add_phantom_overlap_listener(Ptr, u32)
        ),
        entry!(0x00d0_7e20, hkp_phantom_deallocate_internal_arrays(Ptr)),
        entry!(0x00d0_8130, hkp_phantom_destruct(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const LISTENER_SLOT: u32 = 0x0900_0010;
    const ALLOCATOR_FREE_SLOT: u32 = 0x0900_0011;
    const BROAD_PHASE_UPDATE: u32 = 0x0900_0012;
    const TREE_SLOT: u32 = 0x0900_0013;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn new_log() -> CallLog {
        Rc::new(RefCell::new(Vec::new()))
    }

    /// Registers a double at `address` that records its argument words and answers `result`.
    fn record(e: &mut Engine, log: &CallLog, address: u32, result: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: result,
                ..Ret::default()
            }
        });
    }

    fn calls_to(log: &CallLog, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn set_vec(e: &mut Engine, addr: u32, v: Vec4) {
        for (lane, value) in v.into_iter().enumerate() {
            e.mem.set_f32(addr + 4 * lane as u32, value);
        }
    }

    /// A phantom whose world's collision input quantizes with offset 1 and scale 2.
    fn quantizing_phantom(e: &mut Engine) -> (u32, u32) {
        let phantom = e.mem.alloc(0xa4);
        let world = e.mem.alloc(0x400);
        let input = e.mem.alloc(0x60);
        e.mem.set_u32(phantom + 8, world);
        e.mem.set_u32(world + 0x78, input);
        set_vec(e, input + 0x20, [1.0; 4]);
        set_vec(e, input + 0x30, [0.0; 4]);
        set_vec(e, input + 0x40, [2.0; 4]);
        set_vec(e, BOUNDING_UPPER_CLAMP, [1.0e9; 4]);
        set_vec(e, BOUNDING_LOWER_CLAMP, [0.0; 4]);
        (phantom, world)
    }

    #[test]
    fn bounding_volume_data_is_quantized_clamped_and_rounded() {
        let mut e = engine();
        let (phantom, _) = quantizing_phantom(&mut e);
        let aabb = e.mem.alloc(0x20);
        // (1.25 + 1) * 2 = 4.5 rounds to even 4; 3.5 * 2 = 7; negative clamps to 0; the fourth
        // lane is wiped by the byte clears.
        set_vec(&mut e, aabb, [1.25, 2.5, -10.0, 150_994_944.0]);
        set_vec(&mut e, aabb + 0x10, [3.0, 2.5, 1.0e12, 150_994_976.0]);
        e.mem.set_u32(phantom + 0x4c, 0xffff_ffff);
        e.call(0x00d0_7950, &args![phantom, aabb]);
        assert_eq!(e.mem.i32(phantom + 0x30), 4);
        assert_eq!(e.mem.i32(phantom + 0x34), 7);
        assert_eq!(e.mem.i32(phantom + 0x38), 0);
        assert_eq!(e.mem.u32(phantom + 0x3c), 0);
        // The maximum has offset 0 and scale 2: 6, 5, the clamp 1e9 (rounded as a float), and
        // the high byte of the fourth lane (301989952 = 0x12000040) survives.
        assert_eq!(e.mem.i32(phantom + 0x40), 6);
        assert_eq!(e.mem.i32(phantom + 0x44), 5);
        assert_eq!(e.mem.i32(phantom + 0x48), 1.0e9f32 as i32);
        assert_eq!(e.mem.u32(phantom + 0x4c), 0x1200_0000);
    }

    #[test]
    fn bounding_volume_data_of_a_nan_is_the_integer_indefinite() {
        let mut e = engine();
        let (phantom, _) = quantizing_phantom(&mut e);
        // A NaN fails both compares of MINPS/MAXPS and takes the clamp constants instead.
        let aabb = e.mem.alloc(0x20);
        set_vec(&mut e, aabb, [f32::NAN, 0.0, 0.0, 0.0]);
        set_vec(&mut e, aabb + 0x10, [0.0; 4]);
        e.call(0x00d0_7950, &args![phantom, aabb]);
        assert_eq!(e.mem.i32(phantom + 0x30), 1.0e9f32 as i32);
        assert_eq!(fistp(f32::NAN), i32::MIN);
        assert_eq!(fistp(3.0e9), i32::MIN);
        assert_eq!(fistp(-2.5), -2);
    }

    fn make_list(e: &mut Engine, phantom: u32, offset: u32, entries: &[u32]) -> u32 {
        let data = e.mem.alloc(4 * entries.len().max(1) as u32 + 8);
        for (index, entry) in entries.iter().enumerate() {
            e.mem.set_u32(data + 4 + 4 * index as u32, *entry);
        }
        // The data starts one word into the block so that a write before it is harmless.
        e.mem.set_u32(phantom + offset, data + 4);
        e.mem.set_i32(phantom + offset + 4, entries.len() as i32);
        e.mem.set_u32(phantom + offset + 8, entries.len() as u32);
        data
    }

    fn read_list(e: &Engine, phantom: u32, offset: u32) -> Vec<u32> {
        let data = e.mem.u32(phantom + offset);
        (0..e.mem.u32(phantom + offset + 4))
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    #[test]
    fn null_overlap_listeners_are_swept_out() {
        let mut e = engine();
        let phantom = e.mem.alloc(0xa4);
        make_list(&mut e, phantom, 0x8c, &[1, 0, 2, 0, 0, 3]);
        e.call(0x00d0_7a90, &args![phantom]);
        assert_eq!(read_list(&e, phantom, 0x8c), vec![1, 2, 3]);
        // An empty list stays empty.
        make_list(&mut e, phantom, 0x8c, &[]);
        e.call(0x00d0_7a90, &args![phantom]);
        assert!(read_list(&e, phantom, 0x8c).is_empty());
    }

    #[test]
    fn null_phantom_listeners_are_swept_out() {
        let mut e = engine();
        let phantom = e.mem.alloc(0xa4);
        make_list(&mut e, phantom, 0x98, &[0, 5, 0]);
        e.call(0x00d0_7af0, &args![phantom]);
        assert_eq!(read_list(&e, phantom, 0x98), vec![5]);
    }

    #[test]
    fn remove_listener_shifts_the_rest_down() {
        let mut e = engine();
        let phantom = e.mem.alloc(0xa4);
        make_list(&mut e, phantom, 0x98, &[7, 8, 9]);
        e.call(0x00d0_7c10, &args![phantom, 8u32]);
        assert_eq!(read_list(&e, phantom, 0x98), vec![7, 9]);
        e.call(0x00d0_7c10, &args![phantom, 7u32]);
        assert_eq!(read_list(&e, phantom, 0x98), vec![9]);
    }

    #[test]
    fn remove_listener_that_is_absent_acts_as_index_minus_one() {
        let mut e = engine();
        let phantom = e.mem.alloc(0xa4);
        let block = make_list(&mut e, phantom, 0x98, &[7, 9]);
        e.call(0x00d0_7c10, &args![phantom, 5u32]);
        // The size dropped by one and the data moved down one word, starting before it.
        assert_eq!(e.mem.i32(phantom + 0x9c), 1);
        assert_eq!(e.mem.u32(block), 7);
        assert_eq!(e.mem.u32(block + 4), 9);
    }

    fn listener_object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, LISTENER_SLOT);
        e.mem.set_u32(vtable + 8, LISTENER_SLOT + 0x100);
        let listener = e.mem.alloc(8);
        e.mem.set_u32(listener, vtable);
        listener
    }

    #[test]
    fn firing_calls_each_listener_last_to_first_and_sweeps_nulls() {
        let mut e = engine();
        let log = new_log();
        record(&mut e, &log, LISTENER_SLOT, 0);
        record(&mut e, &log, LISTENER_SLOT + 0x100, 0);
        let first = listener_object(&mut e);
        let second = listener_object(&mut e);
        let phantom = e.mem.alloc(0xa4);
        make_list(&mut e, phantom, 0x98, &[first, 0, second]);
        e.call(0x00d0_7b90, &args![phantom]);
        assert_eq!(
            log.borrow()
                .iter()
                .map(|(_, w)| w.clone())
                .collect::<Vec<_>>(),
            vec![vec![second, phantom], vec![first, phantom]]
        );
        assert_eq!(calls_to(&log, LISTENER_SLOT + 0x100).len(), 0);
        assert_eq!(read_list(&e, phantom, 0x98), vec![first, second]);
        log.borrow_mut().clear();
        e.call(0x00d0_7b50, &args![phantom]);
        assert_eq!(calls_to(&log, LISTENER_SLOT + 0x100).len(), 2);
        assert_eq!(calls_to(&log, LISTENER_SLOT).len(), 0);
    }

    #[test]
    fn add_listener_grows_a_full_array_first() {
        let mut e = engine();
        let log = new_log();
        let allocator = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let phantom = e.mem.alloc(0xa4);
        make_list(&mut e, phantom, 0x98, &[4, 5]);
        // Capacity 2 = size 2: the array grows through the helper.
        let grow_log = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            grow_log.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let room = e.mem.alloc(32);
            for index in 0..2 {
                let old = e.mem.u32(e.mem.u32(a[1]) + 4 * index);
                e.mem.set_u32(room + 4 * index, old);
            }
            e.mem.set_u32(a[1], room);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        e.call(0x00d0_7da0, &args![phantom, 6u32]);
        assert_eq!(
            calls_to(&log, ARRAY_RESERVE_MORE),
            vec![vec![allocator, phantom + 0x98, 4]]
        );
        assert_eq!(read_list(&e, phantom, 0x98), vec![4, 5, 6]);
        // With room left, no growth.
        e.call(0x00d0_7da0, &args![phantom, 7u32]);
        assert_eq!(calls_to(&log, ARRAY_RESERVE_MORE).len(), 1);
        assert_eq!(read_list(&e, phantom, 0x98), vec![4, 5, 6, 7]);
    }

    struct BroadPhaseScene {
        phantom: u32,
        world: u32,
        router: u32,
        top: u32,
        aabb: u32,
        dispatcher: u32,
        filter: u32,
    }

    /// A phantom in a world with a broad phase (needing locks), a dispatcher, a collision
    /// filter, a kd-tree manager and a router whose stack top is `top`.
    fn broad_phase_scene(e: &mut Engine, log: &CallLog) -> BroadPhaseScene {
        let (phantom, world) = quantizing_phantom(e);
        let aabb = e.mem.alloc(0x20);
        set_vec(e, aabb, [1.0, 1.0, 1.0, 1.0]);
        set_vec(e, aabb + 0x10, [2.0, 2.0, 2.0, 2.0]);
        e.mem.set_u32(world + 0x33c, 4);
        let broad_phase = e.mem.alloc(0x20);
        let broad_phase_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(broad_phase_vtable + 0x28, BROAD_PHASE_UPDATE);
        e.mem.set_u32(broad_phase, broad_phase_vtable);
        e.mem.set_u32(broad_phase + 0x10, 1);
        e.mem.set_u32(world + 0x58, broad_phase);
        let tree = e.mem.alloc(0x10);
        let tree_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(tree_vtable + 0x14, TREE_SLOT);
        e.mem.set_u32(tree, tree_vtable);
        e.mem.set_u32(world + 0x5c, tree);
        let dispatcher = e.mem.alloc(0x10);
        e.mem.set_u32(world + 0x64, dispatcher);
        let filter = e.mem.alloc(0x20);
        e.mem.set_u32(world + 0x7c, filter);
        // The router: stack top, then (at +0x10) an allocator that records its frees.
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, ALLOCATOR_FREE_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        let top = e.mem.alloc(0x400);
        e.mem.set_u32(router, top);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        record(e, log, ALLOCATOR_FREE_SLOT, 0);
        record(e, log, BROAD_PHASE_LOCK, 0);
        record(e, log, BROAD_PHASE_UNLOCK, 0);
        record(e, log, TREE_SLOT, 0);
        record(e, log, REMOVE_DUPLICATES, 0);
        record(e, log, REMOVE_PAIRS, 0);
        record(e, log, ADD_PAIRS, 0);
        record(e, log, WORLD_EXECUTE_PENDING_OPERATIONS, 0);
        record(e, log, WORLD_EXECUTE_PENDING_BODY_OPERATIONS, 0);
        BroadPhaseScene {
            phantom,
            world,
            router,
            top,
            aabb,
            dispatcher,
            filter,
        }
    }

    #[test]
    fn update_broad_phase_without_a_world_does_nothing() {
        let mut e = engine();
        let phantom = e.mem.alloc(0xa4);
        let aabb = e.mem.alloc(0x20);
        e.call(0x00d0_7ec0, &args![phantom, aabb]);
        assert_eq!(e.mem.u32(phantom + 0x30), 0);
    }

    #[test]
    fn update_broad_phase_queues_the_operation_while_the_world_is_locked() {
        let mut e = engine();
        let log = new_log();
        let scene = broad_phase_scene(&mut e, &log);
        let seen = Rc::clone(&log);
        e.register_double(WORLD_QUEUE_OPERATION, move |e, a| {
            seen.borrow_mut().push((
                WORLD_QUEUE_OPERATION,
                vec![
                    a[0],
                    u32::from(e.mem.u8(a[1])),
                    e.mem.u32(a[1] + 4),
                    e.mem.u32(a[1] + 8),
                ],
            ));
            Ret::default()
        });
        e.mem.set_u32(scene.world + 0x98, 1);
        e.call(0x00d0_7ec0, &args![scene.phantom, scene.aabb]);
        assert_eq!(
            calls_to(&log, WORLD_QUEUE_OPERATION),
            vec![vec![scene.world, 0x11, scene.phantom, scene.aabb]]
        );
        assert!(calls_to(&log, BROAD_PHASE_LOCK).is_empty());
        assert_eq!(e.mem.u32(scene.world + 0x94), 0);
        assert_eq!(e.mem.u32(scene.phantom + 0x30), 0);
    }

    #[test]
    fn update_broad_phase_keeps_its_record_and_arrays_in_the_exe_frame() {
        let mut e = engine();
        let log = new_log();
        let scene = broad_phase_scene(&mut e, &log);
        let queued = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&queued);
        e.register_double(WORLD_QUEUE_OPERATION, move |_, a| {
            sink.borrow_mut().push(a[1]);
            Ret::default()
        });
        e.mem.set_u32(scene.world + 0x98, 1);
        let frame = e.next_stack_local(0x3c);
        e.call(0x00d0_7ec0, &args![scene.phantom, scene.aabb]);
        assert_eq!(*queued.borrow(), vec![frame + 0x10]);
        e.mem.set_u32(scene.world + 0x98, 0);
        let update_log = Rc::clone(&log);
        e.register_double(BROAD_PHASE_UPDATE, move |_, a| {
            update_log
                .borrow_mut()
                .push((BROAD_PHASE_UPDATE, a.to_vec()));
            Ret::default()
        });
        let frame = e.next_stack_local(0x3c);
        e.call(0x00d0_7ec0, &args![scene.phantom, scene.aabb]);
        let update = calls_to(&log, BROAD_PHASE_UPDATE);
        // The handle slot at +0xc, the added pairs at +0x2c, the removed ones at +0x1c.
        assert_eq!(update[0][1], frame + 0xc);
        assert_eq!(update[0][4..6], [frame + 0x2c, frame + 0x1c]);
    }

    #[test]
    fn update_broad_phase_applies_the_pairs_and_requantizes() {
        let mut e = engine();
        let log = new_log();
        let scene = broad_phase_scene(&mut e, &log);
        // The broad phase finds one added and one removed pair.
        let update_log = Rc::clone(&log);
        e.register_double(BROAD_PHASE_UPDATE, move |e, a| {
            update_log.borrow_mut().push((
                BROAD_PHASE_UPDATE,
                vec![a[0], e.mem.u32(a[1]), a[2], a[3], a[4], a[5]],
            ));
            e.mem.set_u32(a[4] + 4, 1);
            e.mem.set_u32(a[5] + 4, 1);
            Ret::default()
        });
        e.mem.set_u32(scene.world + 0x8c, 1);
        e.mem.set_u32(scene.world + 0xa4, 1);
        e.mem.set_u32(scene.world + 0x90, 1);
        // One null overlap listener to be swept after the pairs.
        make_list(&mut e, scene.phantom, 0x8c, &[0, 9]);
        e.call(0x00d0_7ec0, &args![scene.phantom, scene.aabb]);
        let update = calls_to(&log, BROAD_PHASE_UPDATE);
        assert_eq!(update.len(), 1);
        let broad_phase = e.mem.u32(scene.world + 0x58);
        let (first, second) = (update[0][4], update[0][5]);
        assert_eq!(
            update[0][..4],
            [broad_phase, scene.phantom + 0x24, scene.aabb, 1]
        );
        // The arrays sit one after the other on the router's temporary stack: 4 slots round up
        // to 0x80 bytes each; the second array's data is the data pointer saved in the first.
        assert_eq!(e.mem.u32(scene.router), scene.top);
        assert_eq!(calls_to(&log, REMOVE_DUPLICATES), vec![vec![first, second]]);
        assert_eq!(
            calls_to(&log, REMOVE_PAIRS),
            vec![vec![scene.dispatcher, scene.top + 0x80, 1]]
        );
        assert_eq!(
            calls_to(&log, ADD_PAIRS),
            vec![vec![scene.dispatcher, scene.top, 1, scene.filter + 8]]
        );
        assert_eq!(read_list(&e, scene.phantom, 0x8c), vec![9]);
        assert_eq!(calls_to(&log, BROAD_PHASE_LOCK).len(), 1);
        assert_eq!(calls_to(&log, BROAD_PHASE_UNLOCK).len(), 1);
        assert_eq!(calls_to(&log, TREE_SLOT).len(), 1);
        assert_eq!(calls_to(&log, TREE_SLOT)[0][1], 0);
        // Bounding data: (1 + 1) * 2 = 4.
        assert_eq!(e.mem.i32(scene.phantom + 0x30), 4);
        // The lock was taken and dropped again and the pending operations ran.
        assert_eq!(e.mem.u32(scene.world + 0x94), 0);
        assert_eq!(calls_to(&log, WORLD_EXECUTE_PENDING_OPERATIONS).len(), 1);
        assert_eq!(
            calls_to(&log, WORLD_EXECUTE_PENDING_BODY_OPERATIONS).len(),
            1
        );
        // Arrays on the stack (capacity flag set) are not freed.
        assert!(calls_to(&log, ALLOCATOR_FREE_SLOT).is_empty());
    }

    #[test]
    fn update_broad_phase_without_pairs_skips_the_pair_work() {
        let mut e = engine();
        let log = new_log();
        let scene = broad_phase_scene(&mut e, &log);
        // The broad phase grows the second array onto the heap; its storage goes back.
        let block = e.mem.alloc(64);
        e.register_double(BROAD_PHASE_UPDATE, move |e, a| {
            e.mem.set_u32(a[5], block);
            e.mem.set_u32(a[5] + 8, 8);
            Ret::default()
        });
        e.mem.set_u32(scene.world + 0x7c, 0);
        e.mem.set_u8(scene.world + 0x9c, 1);
        e.mem.set_u32(scene.world + 0x8c, 1);
        e.mem.set_u32(scene.world + 0x5c, 0);
        e.call(0x00d0_7ec0, &args![scene.phantom, scene.aabb]);
        assert!(calls_to(&log, REMOVE_DUPLICATES).is_empty());
        assert!(calls_to(&log, ADD_PAIRS).is_empty());
        assert!(calls_to(&log, TREE_SLOT).is_empty());
        // The world blocks pending operations.
        assert!(calls_to(&log, WORLD_EXECUTE_PENDING_OPERATIONS).is_empty());
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE_SLOT),
            vec![vec![e.mem.u32(scene.router + 0x10), block, 64]]
        );
        assert_eq!(e.mem.u32(scene.router), scene.top);
    }

    /// A memory router whose allocator records its frees (slot `0x10`); returns the allocator.
    fn heap_router(e: &mut Engine, log: &CallLog) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, ALLOCATOR_FREE_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        record(e, log, ALLOCATOR_FREE_SLOT, 0);
        allocator
    }

    /// A listener whose virtual function at vtable offset `offset` is `handler`.
    fn listener_with_slot(e: &mut Engine, offset: u32, handler: u32) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + offset, handler);
        let listener = e.mem.alloc(8);
        e.mem.set_u32(listener, vtable);
        listener
    }

    #[test]
    fn the_unnamed_listener_call_does_not_sweep_nulls() {
        let mut e = engine();
        let log = new_log();
        record(&mut e, &log, LISTENER_SLOT + 0x100, 0);
        let first = listener_with_slot(&mut e, 0x10, LISTENER_SLOT + 0x100);
        let phantom = e.mem.alloc(0xa4);
        make_list(&mut e, phantom, 0x98, &[0, first]);
        e.call(0x00d0_7910, &args![phantom]);
        assert_eq!(
            calls_to(&log, LISTENER_SLOT + 0x100),
            vec![vec![first, phantom]]
        );
        assert_eq!(read_list(&e, phantom, 0x98), vec![0, first]);
    }

    #[test]
    fn fire_phantom_shape_set_calls_slot_c_and_sweeps_nulls() {
        let mut e = engine();
        let log = new_log();
        let listener = listener_with_slot(&mut e, 0xc, LISTENER_SLOT + 0x200);
        record(&mut e, &log, LISTENER_SLOT + 0x200, 0);
        let phantom = e.mem.alloc(0xa4);
        make_list(&mut e, phantom, 0x98, &[0, listener, 0]);
        e.call(0x00d0_7bd0, &args![phantom]);
        assert_eq!(
            calls_to(&log, LISTENER_SLOT + 0x200),
            vec![vec![listener, phantom]]
        );
        assert_eq!(read_list(&e, phantom, 0x98), vec![listener]);
    }

    #[test]
    fn remove_overlap_listener_shifts_the_rest_down() {
        let mut e = engine();
        let phantom = e.mem.alloc(0xa4);
        make_list(&mut e, phantom, 0x8c, &[7, 8, 9]);
        e.call(0x00d0_7c80, &args![phantom, 8u32]);
        assert_eq!(read_list(&e, phantom, 0x8c), vec![7, 9]);
        e.call(0x00d0_7c80, &args![phantom, 9u32]);
        assert_eq!(read_list(&e, phantom, 0x8c), vec![7]);
        // The phantom listeners are not touched.
        assert_eq!(e.mem.u32(phantom + 0x9c), 0);
    }

    #[test]
    fn remove_overlap_listener_that_is_absent_acts_as_index_minus_one() {
        let mut e = engine();
        let phantom = e.mem.alloc(0xa4);
        let block = make_list(&mut e, phantom, 0x8c, &[7, 9]);
        e.call(0x00d0_7c80, &args![phantom, 5u32]);
        assert_eq!(e.mem.i32(phantom + 0x90), 1);
        assert_eq!(e.mem.u32(block), 7);
        assert_eq!(e.mem.u32(block + 4), 9);
    }

    #[test]
    fn add_overlap_listener_grows_a_full_array_first() {
        let mut e = engine();
        let log = new_log();
        let allocator = heap_router(&mut e, &log);
        let phantom = e.mem.alloc(0xa4);
        make_list(&mut e, phantom, 0x8c, &[4, 5]);
        let grow_log = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            grow_log.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let room = e.mem.alloc(32);
            for index in 0..2 {
                let old = e.mem.u32(e.mem.u32(a[1]) + 4 * index);
                e.mem.set_u32(room + 4 * index, old);
            }
            e.mem.set_u32(a[1], room);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        e.call(0x00d0_7de0, &args![phantom, 6u32]);
        assert_eq!(
            calls_to(&log, ARRAY_RESERVE_MORE),
            vec![vec![allocator, phantom + 0x8c, 4]]
        );
        assert_eq!(read_list(&e, phantom, 0x8c), vec![4, 5, 6]);
        e.call(0x00d0_7de0, &args![phantom, 7u32]);
        assert_eq!(calls_to(&log, ARRAY_RESERVE_MORE).len(), 1);
        assert_eq!(read_list(&e, phantom, 0x8c), vec![4, 5, 6, 7]);
        assert_eq!(e.mem.u32(phantom + 0x9c), 0);
    }

    #[test]
    fn deallocate_internal_arrays_releases_only_the_empty_lists() {
        let mut e = engine();
        let log = new_log();
        let allocator = heap_router(&mut e, &log);
        let phantom = e.mem.alloc(0xa4);
        let overlap_block = make_list(&mut e, phantom, 0x8c, &[]);
        e.mem.set_u32(phantom + 0x94, 4);
        make_list(&mut e, phantom, 0x98, &[1, 2]);
        e.call(0x00d0_7e20, &args![phantom]);
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE_SLOT),
            vec![vec![allocator, overlap_block + 4, 16]]
        );
        assert_eq!(e.mem.u32(phantom + 0x8c), 0);
        assert_eq!(e.mem.u32(phantom + 0x90), 0);
        assert_eq!(e.mem.u32(phantom + 0x94), 0x8000_0000);
        assert_eq!(read_list(&e, phantom, 0x98), vec![1, 2]);
        // Now the other list is empty and its storage is not owned: nothing is freed, but the
        // array is reset.
        e.mem.set_u32(phantom + 0x9c, 0);
        e.mem.set_u32(phantom + 0xa0, 0x8000_0002);
        e.call(0x00d0_7e20, &args![phantom]);
        assert_eq!(calls_to(&log, ALLOCATOR_FREE_SLOT).len(), 1);
        assert_eq!(e.mem.u32(phantom + 0x98), 0);
        assert_eq!(e.mem.u32(phantom + 0xa0), 0x8000_0000);
    }

    #[test]
    fn destructor_tells_the_listeners_and_frees_both_lists() {
        let mut e = engine();
        let log = new_log();
        let allocator = heap_router(&mut e, &log);
        record(&mut e, &log, LISTENER_SLOT + 0x100, 0);
        record(&mut e, &log, WORLD_OBJECT_DESTRUCT, 0);
        let listener = listener_with_slot(&mut e, 0x10, LISTENER_SLOT + 0x100);
        let phantom = e.mem.alloc(0xa4);
        let overlap_block = make_list(&mut e, phantom, 0x8c, &[3]);
        let phantom_block = make_list(&mut e, phantom, 0x98, &[listener, 0]);
        e.call(0x00d0_8130, &args![phantom]);
        let order: Vec<(u32, Vec<u32>)> = log.borrow().clone();
        assert_eq!(
            order,
            vec![
                (LISTENER_SLOT + 0x100, vec![listener, phantom]),
                (ALLOCATOR_FREE_SLOT, vec![allocator, phantom_block + 4, 8]),
                (ALLOCATOR_FREE_SLOT, vec![allocator, overlap_block + 4, 4]),
                (WORLD_OBJECT_DESTRUCT, vec![phantom]),
            ]
        );
        assert_eq!(e.mem.u32(phantom), PHANTOM_VTABLE);
        for offset in [0x8c, 0x98] {
            assert_eq!(e.mem.u32(phantom + offset), 0);
            assert_eq!(e.mem.u32(phantom + offset + 4), 0);
            assert_eq!(e.mem.u32(phantom + offset + 8), 0x8000_0000);
        }
    }

    /// A collector whose slot `0x14` and byte `+0x10` are set up for the statistics tests.
    fn statistics_collector(e: &mut Engine, log: &CallLog, borrowed_too: bool) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x14, LISTENER_SLOT + 0x300);
        let collector = e.mem.alloc(0x20);
        e.mem.set_u32(collector, vtable);
        e.mem.set_u8(collector + 0x10, u8::from(borrowed_too));
        record(e, log, LISTENER_SLOT + 0x300, 0);
        record(e, log, WORLD_OBJECT_CALC_CONTENT_STATISTICS, 0);
        record(e, log, GET_ALLOCATED_SIZE, 0x99);
        collector
    }

    #[test]
    fn calc_content_statistics_reports_both_listener_arrays() {
        let mut e = engine();
        let log = new_log();
        let collector = statistics_collector(&mut e, &log, false);
        let phantom = e.mem.alloc(0xa4);
        let overlap_block = make_list(&mut e, phantom, 0x8c, &[3, 4]);
        e.mem.set_u32(phantom + 0x94, 6);
        let phantom_block = make_list(&mut e, phantom, 0x98, &[5]);
        e.call(0x00d0_7cf0, &args![phantom, collector, 0u32]);
        assert_eq!(
            log.borrow().iter().map(|(at, _)| *at).collect::<Vec<_>>(),
            vec![
                WORLD_OBJECT_CALC_CONTENT_STATISTICS,
                GET_ALLOCATED_SIZE,
                LISTENER_SLOT + 0x300,
                GET_ALLOCATED_SIZE,
                LISTENER_SLOT + 0x300,
            ]
        );
        assert_eq!(
            calls_to(&log, WORLD_OBJECT_CALC_CONTENT_STATISTICS),
            vec![vec![phantom, collector, PHANTOM_CLASS_RECORD]]
        );
        assert_eq!(
            calls_to(&log, GET_ALLOCATED_SIZE),
            vec![vec![overlap_block + 4, 24], vec![phantom_block + 4, 4]]
        );
        assert_eq!(
            calls_to(&log, LISTENER_SLOT + 0x300),
            vec![
                vec![
                    collector,
                    3,
                    NAME_OVERLAP_LISTENERS,
                    overlap_block + 4,
                    8,
                    0x99
                ],
                vec![
                    collector,
                    3,
                    NAME_PHANTOM_LISTENERS,
                    phantom_block + 4,
                    4,
                    0x99
                ],
            ]
        );
    }

    #[test]
    fn calc_content_statistics_skips_borrowed_arrays_unless_asked() {
        let mut e = engine();
        let log = new_log();
        let collector = statistics_collector(&mut e, &log, false);
        let phantom = e.mem.alloc(0xa4);
        e.mem.set_u32(phantom + 0x94, 0x8000_0000);
        e.mem.set_u32(phantom + 0xa0, 0x8000_0003);
        e.call(0x00d0_7cf0, &args![phantom, collector, 0u32]);
        assert!(calls_to(&log, LISTENER_SLOT + 0x300).is_empty());
        assert!(calls_to(&log, GET_ALLOCATED_SIZE).is_empty());
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00d0_7cf0, &args![phantom, collector, 0u32]);
        assert_eq!(
            calls_to(&log, GET_ALLOCATED_SIZE),
            vec![vec![0, 0], vec![0, 12]]
        );
        assert_eq!(calls_to(&log, LISTENER_SLOT + 0x300).len(), 2);
        // Only the first array is borrowed: the second is reported without being asked.
        e.mem.set_u8(collector + 0x10, 0);
        e.mem.set_u32(phantom + 0xa0, 3);
        log.borrow_mut().clear();
        e.call(0x00d0_7cf0, &args![phantom, collector, 0u32]);
        let reported = calls_to(&log, LISTENER_SLOT + 0x300);
        assert_eq!(reported.len(), 1);
        assert_eq!(reported[0][2], NAME_PHANTOM_LISTENERS);
    }
}
