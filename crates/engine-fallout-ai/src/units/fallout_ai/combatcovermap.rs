//! `fallout/ai/combat/combatcovermap.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VCoverReservation@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108D124: u32 = 0x0108_d124;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 00983700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VCoverReservation@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_00983700(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108D124);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `CombatCoverMap`'s vtable (`0108d0e4`, named by its RTTI locator).
const COMBAT_COVER_MAP_VTABLE: u32 = 0x0108_d0e4;
/// `NiTMap<unsigned int, ReservedCoverLocation*>`'s vtable (`0108d104`).
const NI_TMAP_VTABLE: u32 = 0x0108_d104;
/// `NiTMapBase<DFALL<NiTMapItem<unsigned int, ReservedCoverLocation*>>,
/// unsigned int, ReservedCoverLocation*>`'s vtable (`0108d138`).
const NI_TMAP_BASE_VTABLE: u32 = 0x0108_d138;
/// `NiTMapBase` fields (Xbox PDB names): `m_uiHashSize` +0x04,
/// `m_ppkHashTable` +0x08, the allocator's `m_uiCount` +0x0C.
const HASH_SIZE: u32 = 0x04;
const HASH_TABLE: u32 = 0x08;
const MAP_COUNT: u32 = 0x0c;
/// `CombatCoverMap`'s pointer member at +0x10.
const COVER_MAP_POINTER: u32 = 0x10;
/// The allocator (`00aa1070`, `cdecl(size)`) and its free (`00aa10f0`,
/// `cdecl(block)`).
const MAP_ALLOCATE: u32 = 0x00aa_1070;
const MAP_FREE: u32 = 0x00aa_10f0;
/// The game's `memset` wrapper (`00403d30`, `cdecl(dst, value, count)`).
const MEMSET: u32 = 0x0040_3d30;
/// `NiTMapBase::RemoveAll` of this instance (`00438af0`, `thiscall`).
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `GetFirstPos` (`004b9ba0`, `thiscall`) and `GetNext` (`006b7f20`,
/// `thiscall(position pointer, key pointer, value pointer)`).
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
const MAP_GET_NEXT: u32 = 0x006b_7f20;
/// The scalar deleting destructor of a reserved cover location (`00983250`,
/// `thiscall(flags)`).
const RESERVED_LOCATION_DELETE: u32 = 0x0098_3250;
/// `NiPointer` get (`00559450`) and destructor (`0045cec0`).
const POINTER_GET: u32 = 0x0055_9450;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// Returns the object a pointer's target refers to (`009611e0`,
/// `thiscall`).
const POINTER_TARGET_OWNER: u32 = 0x0096_11e0;
/// `operator delete` (`00401030`, `cdecl(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 00983100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatCoverMap::scalar deleting destructor` (Xbox PDB): the
/// destructor body (`00983130`), then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn combat_cover_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00983130(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00983130 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00983130 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatCoverMap` destructor body (by its vtable, `0108d0e4`): deletes
/// every reserved cover location the map holds (`GetFirstPos`/`GetNext`,
/// `00983250(1)` on each non-null value), empties the map (`00438af0`);
/// when the pointer at +0x10 holds an object whose `009611e0` is not null,
/// calls that owner's slot 0xE8 with the object; then destroys the pointer
/// (`0045cec0`) and the `NiTMap` base (`00983820`).
pub fn fn_00983130(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, COMBAT_COVER_MAP_VTABLE);
    // The game's locals: key at EBP-0x18, value at EBP-0x14, position at
    // EBP-0x10.
    e.with_stack(0xc, |e, locals| {
        let key = locals.addr();
        let value = key + 4;
        let position = key + 8;
        let first = e.call(MAP_FIRST_POSITION, &args![this]).u32();
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(key, 0);
            e.mem.set_u32(value, 0);
            e.call(MAP_GET_NEXT, &args![this, position, key, value]);
            let location = e.mem.u32(value);
            if location != 0 {
                e.call(RESERVED_LOCATION_DELETE, &args![location, 1u32]);
            }
        }
    });
    e.call(MAP_REMOVE_ALL, &args![this]);
    let pointer = base + COVER_MAP_POINTER;
    if e.call(POINTER_GET, &args![pointer]).u32() != 0 {
        let object = e.call(POINTER_GET, &args![pointer]).u32();
        if e.call(POINTER_TARGET_OWNER, &args![object]).u32() != 0 {
            let object = e.call(POINTER_GET, &args![pointer]).u32();
            let owner = e.call(POINTER_TARGET_OWNER, &args![object]).u32();
            let object = e.call(POINTER_GET, &args![pointer]).u32();
            e.vcall(owner, 0xe8, &args![object]);
        }
    }
    e.call(POINTER_DESTRUCT, &args![pointer]);
    fn_00983820(e, this);
}

// Translated from 00983670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int, ReservedCoverLocation*>` constructor (by the
/// vtable it sets, `0108d104`): the `NiTMapBase` part ([`fn_009837b0`]),
/// then the vtable. Returns `this`.
pub fn fn_00983670(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_009837b0(e, this, hash_size);
    e.mem.set_u32(this.addr(), NI_TMAP_VTABLE);
    this
}

// Translated from 00983720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `count` 8-byte elements at `index` from the
/// `BSSimpleArray<CoverReservation, 1024>` at `this` (buffer +0x04, size
/// +0x08; unnamed in the engine map): `n = 0042f5a0(size - index - count,
/// count)`, destroys the removed elements (`0072ba80`), moves the last `n`
/// elements into the gap (`00692480(gap, end - n, n)`) and lowers the size
/// by `count`.
pub fn fn_00983720(e: &mut Engine, this: Ptr, index: u32, count: u32) {
    let base = this.addr();
    let size = e.mem.u32(base + 8);
    let moved = e
        .call(
            0x0042_f5a0,
            &args![size.wrapping_sub(index).wrapping_sub(count), count],
        )
        .u32();
    let gap = e.mem.u32(base + 4).wrapping_add(index.wrapping_mul(8));
    e.call(0x0072_ba80, &args![this, gap, count]);
    let buffer = e.mem.u32(base + 4);
    let size = e.mem.u32(base + 8);
    let tail = buffer
        .wrapping_add(size.wrapping_mul(8))
        .wrapping_sub(moved.wrapping_mul(8));
    let gap = buffer.wrapping_add(index.wrapping_mul(8));
    e.call(0x0069_2480, &args![this, gap, tail, moved]);
    let size = e.mem.u32(base + 8).wrapping_sub(count);
    e.mem.set_u32(base + 8, size);
}

// Translated from 009837b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., unsigned int, ReservedCoverLocation*>` constructor (by
/// the vtable it sets, `0108d138`): `m_uiHashSize = hash_size`, count 0,
/// and a zeroed table of `hash_size` pointers (`00aa1070`, `00403d30`).
/// Returns `this`.
pub fn fn_009837b0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_TMAP_BASE_VTABLE);
    e.mem.set_u32(base + HASH_SIZE, hash_size);
    e.mem.set_u32(base + MAP_COUNT, 0);
    let bytes = e.mem.u32(base + HASH_SIZE) << 2;
    let table = e.call(MAP_ALLOCATE, &args![bytes]).u32();
    e.mem.set_u32(base + HASH_TABLE, table);
    let bytes = e.mem.u32(base + HASH_SIZE) << 2;
    let table = e.mem.u32(base + HASH_TABLE);
    e.call(MEMSET, &args![table, 0u32, bytes]);
    this
}

// Translated from 00983820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00983820 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `NiTMap<unsigned int, ReservedCoverLocation*>` destructor body: sets its
/// vtable, empties the map (`00438af0`) and destroys the `NiTMapBase` part
/// ([`fn_00983880`]).
pub fn fn_00983820(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_TMAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00983880(e, this);
}

// Translated from 00983880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., unsigned int, ReservedCoverLocation*>` destructor body:
/// sets its vtable, empties the map (`00438af0`) and frees the table
/// (`00aa10f0`).
pub fn fn_00983880(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_TMAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.mem.u32(this.addr() + HASH_TABLE);
    e.call(MAP_FREE, &args![table]);
}

/// `operator new` (`00401000`, `cdecl(size)`).
const OPERATOR_NEW: u32 = 0x0040_1000;

// The reserved cover locations (0x1C bytes: a `BSSimpleArray` of 8-byte
// reservations at +0x00, a mask at +0x10, three bytes at +0x14..+0x16 and an
// `NiPointer` at +0x18), the map's reserve and release under its lock at
// +0x14, and the slot search `009827c0`. What the words mean is described
// by what the code does with them; their names are not confirmed.

/// The array's count (`0044ddc0`, `thiscall`) and element address
/// (`00844160`, `thiscall(index)`).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
const ARRAY_AT: u32 = 0x0084_4160;
/// The array's append (`006db840`, `thiscall(element)`) and its constructor
/// step (`006dc440`, `thiscall(word, word)`).
const ARRAY_APPEND: u32 = 0x006d_b840;
const ARRAY_INITIALIZE: u32 = 0x006d_c440;
/// Builds an 8-byte reservation (`004b0680`, `thiscall(first, second)`,
/// returns `this`).
const PAIR_CONSTRUCT: u32 = 0x004b_0680;
/// `00982b90` (`thiscall`): the slot mask of the object at +0x70 of the
/// second word; `00982f10` (`thiscall(object)`), called with it when the
/// byte at +0x15 is clear.
const SLOT_MASK: u32 = 0x0098_2b90;
const MASK_PREPARE: u32 = 0x0098_2f10;
/// `CombatGroup::IsTarget` (engine map; `009865b0`, `thiscall(word)`) on the
/// group `004fb070` gives, and the word at +0xBC (`0097ae90`).
const GROUP_IS_TARGET: u32 = 0x0098_65b0;
const OWNER_GROUP: u32 = 0x004f_b070;
const OWNER_WORD: u32 = 0x0097_ae90;
/// `NiPointer` constructor (`00633c90`, `thiscall(object)`).
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `008c7aa0` (`cdecl`, no words, `bool`), the global `004537b0` returns
/// (`011df1a8`) and its `0087ac60` (`thiscall(object)`).
const DESTROY_TEST: u32 = 0x008c_7aa0;
const QUEUE_GLOBAL: u32 = 0x0045_37b0;
const QUEUE_OBJECT: u32 = 0x0087_ac60;
/// The map's lookup (`00853130`, `thiscall(key, value out)`), insert
/// (`00844700`, `thiscall(key, value)`) and remove (`00405430`,
/// `thiscall(key)`).
const MAP_GET_AT: u32 = 0x0085_3130;
const MAP_SET_AT: u32 = 0x0084_4700;
const MAP_REMOVE_AT: u32 = 0x0040_5430;
/// The lock at +0x14 of the map: `BSReadWriteLock::LockForWrite` (engine
/// map; `00aa6060`) and its release (`00aa62e0`); the read pair `00aa5ff0`
/// and `00aa62c0`.
const LOCK_FOR_WRITE: u32 = 0x00aa_6060;
const UNLOCK_WRITE: u32 = 0x00aa_62e0;
const LOCK_FOR_READ: u32 = 0x00aa_5ff0;
const UNLOCK_READ: u32 = 0x00aa_62c0;
/// On the second word of the reserve calls: `00678ce0` (`thiscall`, byte)
/// and `0067fdb0` (`thiscall(byte)`).
const SECOND_FLAG: u32 = 0x0067_8ce0;
const SECOND_SET_RESERVED: u32 = 0x0067_fdb0;
/// The object at `011f1958` and its `009611e0` (`thiscall`): the cover map.
const COVER_MAP_OWNER: u32 = 0x011f_1958;
/// `005dc960` (`thiscall`, `bool`) on the reserve calls' word.
const RESERVE_TEST: u32 = 0x005d_c960;
/// The slot search's callees on its object (`this`): `00982630`
/// (`thiscall(word)`), `00982750` (`thiscall(float)`, byte), `00982b20`
/// (`cdecl(byte, byte)`, byte), `00982b50` (`thiscall`) and `00982be0`
/// (`thiscall`, `bool`) on the first word; `00982bc0` and `00843150` (bytes)
/// and `0044edb0` (the taken mask) on the location.
const SEARCH_RESET: u32 = 0x0098_2630;
const SEARCH_WIDTH: u32 = 0x0098_2750;
const SEARCH_FIT: u32 = 0x0098_2b20;
const SEARCH_FAILED: u32 = 0x0098_2b50;
const SEARCH_CENTERED: u32 = 0x0098_2be0;
const LOCATION_COUNT: u32 = 0x0098_2bc0;
const LOCATION_READY: u32 = 0x0084_3150;
const LOCATION_TAKEN: u32 = 0x0044_edb0;
/// `1.5` (`double`, `01016ff0`): the width is divided by it.
const WIDTH_DIVISOR: u32 = 0x0101_6ff0;

// Translated from 009827c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(word, unused, width, location, a,
/// b)`): picks a slot for the object at `this` (bytes: +0 ready, +1 count,
/// +2 width, +3 slot, +4 and +5 the last `a` and `b`). With a `location` it
/// takes the count (`00982bc0`), the ready byte (`00843150`) and the taken
/// mask (`0044edb0`) from it; without one, `00982630(word)` and no taken
/// bits. False unless ready and count are set. A zero width is first set
/// from `00982750(width)`; false when the width divided by 1.5 (truncated,
/// low byte) exceeds the count; the width becomes `00982b20(width, count)`.
/// A slot already chosen stays (true) when its mask (`00982b90`) misses the
/// taken bits and `b` is unchanged. Otherwise, with `00982be0(word)` true
/// and `a` clear, the one slot 0 (`b` set) or `count - width`; else, when
/// `count / width` is 2, the slots from 0 up (or from `count - width` down
/// when the byte at `word + 0x4F` is set), and for any other ratio the slots
/// outward from `count / 2 - width / 2`. The first slot whose shifted mask
/// misses the taken bits is kept at +3 (true); none: `00982b50` and false.
/// A zero width faults in the exe's `IDIV`; here it panics.
pub fn fn_009827c0(
    e: &mut Engine,
    this: Ptr,
    word: u32,
    _unused_2: u32,
    width: f32,
    location: u32,
    a: u8,
    b: u8,
) -> bool {
    let base = this.addr();
    let mut taken = 0u32;
    if location != 0 {
        let count = e.call(LOCATION_COUNT, &args![location]).u8();
        e.mem.set_u8(base + 1, count);
        let ready = e.call(LOCATION_READY, &args![location]).u8();
        e.mem.set_u8(base, ready);
        taken = e.call(LOCATION_TAKEN, &args![location]).u32();
    } else {
        e.call(SEARCH_RESET, &args![this, word]);
    }
    if e.mem.u8(base) == 0 || e.mem.u8(base + 1) == 0 {
        return false;
    }
    if e.mem.u8(base + 2) == 0 {
        let fitted = e.call(SEARCH_WIDTH, &args![this, x87_quiet(width)]).u8();
        e.mem.set_u8(base + 2, fitted);
    }
    let divisor: f64 = e.global(WIDTH_DIVISOR);
    let quotient = x87_div(f64::from(e.mem.u8(base + 2)), divisor);
    let needed = crate::x87::fistp_chop(quotient) as u8;
    if needed > e.mem.u8(base + 1) {
        return false;
    }
    let count = u32::from(e.mem.u8(base + 1));
    let current = u32::from(e.mem.u8(base + 2));
    let fitted = e.call(SEARCH_FIT, &args![current, count]).u8();
    e.mem.set_u8(base + 2, fitted);
    if e.mem.u8(base + 3) != 0 {
        e.mem.set_u8(base + 4, a);
        if e.call(SLOT_MASK, &args![this]).u32() & taken == 0 && e.mem.u8(base + 5) == b {
            return true;
        }
    }
    e.mem.set_u8(base + 4, a);
    e.mem.set_u8(base + 5, b);
    e.mem.set_u8(base + 3, 0);
    let mask = e.call(SLOT_MASK, &args![this]).u32();
    let free = |slot: i32| taken & mask.wrapping_shl(slot as u32) == 0;
    if e.call(SEARCH_CENTERED, &args![word]).bool() && e.mem.u8(base + 4) == 0 {
        let slot = if e.mem.u8(base + 5) != 0 {
            0
        } else {
            e.mem.u8(base + 1).wrapping_sub(e.mem.u8(base + 2))
        };
        if free(i32::from(slot)) {
            e.mem.set_u8(base + 3, slot);
            return true;
        }
    } else {
        let count = i32::from(e.mem.u8(base + 1));
        let width = i32::from(e.mem.u8(base + 2));
        let last = count - width;
        if count / width == 2 {
            if e.mem.u8(word + 0x4f) != 0 {
                let mut slot = last;
                while slot >= 0 {
                    if free(slot) {
                        e.mem.set_u8(base + 3, slot as u8);
                        return true;
                    }
                    slot -= 1;
                }
            } else {
                let mut slot = 0;
                while slot <= last {
                    if free(slot) {
                        e.mem.set_u8(base + 3, slot as u8);
                        return true;
                    }
                    slot += 1;
                }
            }
        } else {
            let center = count / 2 - width / 2;
            let mut distance = 0i32;
            let mut done = false;
            while !done {
                let mut side = 0;
                while side < i32::from(distance > 0) + 1 {
                    let slot = if side == 0 {
                        center - distance
                    } else {
                        center + distance
                    };
                    if slot < 0 || slot > last {
                        done = true;
                    } else if free(slot) {
                        e.mem.set_u8(base + 3, slot as u8);
                        return true;
                    }
                    side += 1;
                }
                distance += 1;
            }
        }
    }
    e.call(SEARCH_FAILED, &args![this]);
    false
}

// Translated from 00982d10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00982d10 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The reserved cover location's destructor body (unnamed in the engine
/// map; by its body): when the pointer at +0x18 holds an object whose
/// `009611e0` is set, hands the object to that owner's slot 0xE8, or (when
/// `008c7aa0` holds) to `0087ac60` on the global `004537b0` returns; then
/// destroys the pointer (`0045cec0`) and the array ([`fn_00983700`]).
pub fn fn_00982d10(e: &mut Engine, this: Ptr) {
    let pointer = this.addr() + 0x18;
    if e.call(POINTER_GET, &args![pointer]).u32() != 0 {
        let object = e.call(POINTER_GET, &args![pointer]).u32();
        if e.call(POINTER_TARGET_OWNER, &args![object]).u32() != 0 {
            if !e.call(DESTROY_TEST, &[]).bool() {
                let object = e.call(POINTER_GET, &args![pointer]).u32();
                let owner = e.call(POINTER_TARGET_OWNER, &args![object]).u32();
                let object = e.call(POINTER_GET, &args![pointer]).u32();
                e.vcall(owner, 0xe8, &args![object]);
            } else {
                let object = e.call(POINTER_GET, &args![pointer]).u32();
                let queue = e.call(QUEUE_GLOBAL, &[]).u32();
                e.call(QUEUE_OBJECT, &args![queue, object]);
            }
        }
    }
    e.call(POINTER_DESTRUCT, &args![pointer]);
    fn_00983700(e, this);
}

// Translated from 00982df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(owner)`): true for a null `owner`;
/// false when a reservation of the location already belongs to it, or when
/// the group of `owner` (`004fb070`) has a reservation's owner word
/// (`0097ae90`) as a target; true otherwise.
pub fn fn_00982df0(e: &mut Engine, this: Ptr, owner: u32) -> bool {
    if owner == 0 {
        return true;
    }
    let mut index = 0u32;
    while index < e.call(ARRAY_COUNT, &args![this]).u32() {
        let entry = e.call(ARRAY_AT, &args![this, index]).u32();
        let holder = e.mem.u32(entry);
        if holder == owner {
            return false;
        }
        let word = e.call(OWNER_WORD, &args![holder]).u32();
        let group = e.call(OWNER_GROUP, &args![owner]).u32();
        if e.call(GROUP_IS_TARGET, &args![group, word]).bool() {
            return false;
        }
        index += 1;
    }
    true
}

/// The index of the reservation whose first word is `owner`, if any.
fn reservation_of(e: &mut Engine, this: Ptr, owner: u32) -> Option<u32> {
    let mut index = 0u32;
    while index < e.call(ARRAY_COUNT, &args![this]).u32() {
        let entry = e.call(ARRAY_AT, &args![this, index]).u32();
        if e.mem.u32(entry) == owner {
            return Some(index);
        }
        index += 1;
    }
    None
}

/// The slot mask of `second` (`00982f10` first when the byte at +0x15 is
/// clear).
fn second_mask(e: &mut Engine, this: Ptr, second: u32) -> u32 {
    if e.mem.u8(this.addr() + 0x15) == 0 {
        e.call(MASK_PREPARE, &args![this, second + 0x70]);
    }
    e.call(SLOT_MASK, &args![second + 0x70]).u32()
}

/// Appends the reservation `(owner, second)` (built on the stack by
/// `004b0680`).
fn append_reservation(e: &mut Engine, this: Ptr, owner: u32, second: u32) {
    e.with_stack(8, |e, pair| {
        let pair = e.call(PAIR_CONSTRUCT, &args![pair, owner, second]).u32();
        e.call(ARRAY_APPEND, &args![this, pair]);
    });
}

// Translated from 00982e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(owner, second)`): nothing when
/// `owner` already holds a reservation; otherwise ORs the slot mask of
/// `second` into +0x10 and appends `(owner, second)`.
pub fn fn_00982e70(e: &mut Engine, this: Ptr, owner: u32, second: u32) {
    if reservation_of(e, this, owner).is_some() {
        return;
    }
    let mask = second_mask(e, this, second);
    let bits = e.mem.u32(this.addr() + 0x10) | mask;
    e.mem.set_u32(this.addr() + 0x10, bits);
    append_reservation(e, this, owner, second);
}

// Translated from 00982f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(owner, second)`): as
/// [`fn_00982e70`], but false when `owner` already holds a reservation or
/// the slot mask of `second` overlaps +0x10; true once added.
pub fn fn_00982f40(e: &mut Engine, this: Ptr, owner: u32, second: u32) -> bool {
    if reservation_of(e, this, owner).is_some() {
        return false;
    }
    let mask = second_mask(e, this, second);
    if e.mem.u32(this.addr() + 0x10) & mask != 0 {
        return false;
    }
    let bits = e.mem.u32(this.addr() + 0x10) | mask;
    e.mem.set_u32(this.addr() + 0x10, bits);
    append_reservation(e, this, owner, second);
    true
}

// Translated from 00982ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(owner, second)`): for the
/// reservation of `owner`, true when it is the only one (the location can
/// go); otherwise clears the slot mask of `second` from +0x10, removes it
/// ([`fn_00983720`]) and returns false. False when `owner` holds none. The
/// count is read once, before the loop.
pub fn fn_00982ff0(e: &mut Engine, this: Ptr, owner: u32, second: u32) -> bool {
    let count = e.call(ARRAY_COUNT, &args![this]).u32();
    for index in 0..count {
        let entry = e.call(ARRAY_AT, &args![this, index]).u32();
        if e.mem.u32(entry) != owner {
            continue;
        }
        if count == 1 {
            return true;
        }
        let mask = e.call(SLOT_MASK, &args![second + 0x70]).u32();
        let bits = !mask & e.mem.u32(this.addr() + 0x10);
        e.mem.set_u32(this.addr() + 0x10, bits);
        fn_00983720(e, this, index, 1);
        return false;
    }
    false
}

// Translated from 00983250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reserved cover location's scalar deleting destructor (unnamed in the
/// engine map): [`fn_00982d10`], then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00983250(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00982d10(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// The location of `second`'s key (+0x68) in the map, made (0x1C bytes,
/// [`fn_00983380`], its +0x16 byte from `00678ce0`) and inserted when
/// missing. A failed allocation leaves null, which the exe then writes
/// through.
fn location_for(e: &mut Engine, this: Ptr, second: u32) -> u32 {
    e.with_stack(4, |e, out| {
        let key = e.mem.u32(second + 0x68);
        if !e.call(MAP_GET_AT, &args![this, key, out]).bool() {
            let block = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
            let location = if block != 0 {
                fn_00983380(e, Ptr::new(block)).addr()
            } else {
                0
            };
            e.mem.set_u32(out.addr(), location);
            let flag = e.call(SECOND_FLAG, &args![second]).u8();
            fn_00983400(e, Ptr::new(location), flag);
            let key = e.mem.u32(second + 0x68);
            e.call(MAP_SET_AT, &args![this, key, location]);
        }
        e.mem.u32(out.addr())
    })
}

// Translated from 00983280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00983280 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Unnamed in the engine map (`thiscall(owner, second)`; the cover map's
/// reserve): under the write lock at +0x14, finds or makes the location of
/// `second`'s key, adds `(owner, second)` to it ([`fn_00982e70`]) and marks
/// `second` reserved (`0067fdb0(1)`).
pub fn fn_00983280(e: &mut Engine, this: Ptr, owner: u32, second: u32) {
    e.call(LOCK_FOR_WRITE, &args![this.addr() + 0x14]);
    let location = location_for(e, this, second);
    fn_00982e70(e, Ptr::new(location), owner, second);
    e.call(SECOND_SET_RESERVED, &args![second, 1u32]);
    e.call(UNLOCK_WRITE, &args![this.addr() + 0x14]);
}

// Translated from 00983380 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00983380 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The reserved cover location's constructor (unnamed in the engine map):
/// the array ([`fn_009836d0`]), a zero mask at +0x10, zero bytes at
/// +0x14..+0x16 and a null `NiPointer` at +0x18 (`00633c90(0)`). Returns
/// `this`.
pub fn fn_00983380(e: &mut Engine, this: Ptr) -> Ptr {
    fn_009836d0(e, this);
    let base = this.addr();
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u8(base + 0x14, 0);
    e.mem.set_u8(base + 0x15, 0);
    e.mem.set_u8(base + 0x16, 0);
    e.call(POINTER_CONSTRUCT, &args![base + 0x18, 0u32]);
    this
}

// Translated from 00983400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(byte)`): stores `value` at +0x16.
pub fn fn_00983400(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x16, value);
}

// Translated from 00983420 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00983420 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Unnamed in the engine map (`thiscall(owner, second)`; the cover map's
/// checked reserve): as [`fn_00983280`] with [`fn_00982f40`]; `second` is
/// marked with its result, which is returned.
pub fn fn_00983420(e: &mut Engine, this: Ptr, owner: u32, second: u32) -> bool {
    e.call(LOCK_FOR_WRITE, &args![this.addr() + 0x14]);
    let location = location_for(e, this, second);
    let added = fn_00982f40(e, Ptr::new(location), owner, second);
    e.call(SECOND_SET_RESERVED, &args![second, u32::from(added)]);
    e.call(UNLOCK_WRITE, &args![this.addr() + 0x14]);
    added
}

// Translated from 00983520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(owner, second)`; the cover map's
/// release): nothing for a zero key at `second` +0x68. Under the write lock,
/// when the map holds the key's location: if releasing `owner`
/// ([`fn_00982ff0`]) empties it, deletes the location
/// ([`fn_00983250`] with 1) and removes the key (`00405430`); then marks
/// `second` unreserved (`0067fdb0(0)`).
pub fn fn_00983520(e: &mut Engine, this: Ptr, owner: u32, second: u32) {
    if e.mem.u32(second + 0x68) == 0 {
        return;
    }
    e.call(LOCK_FOR_WRITE, &args![this.addr() + 0x14]);
    e.with_stack(4, |e, out| {
        let key = e.mem.u32(second + 0x68);
        if e.call(MAP_GET_AT, &args![this, key, out]).bool() {
            let location = e.mem.u32(out.addr());
            if fn_00982ff0(e, Ptr::new(location), owner, second) {
                let location = e.mem.u32(out.addr());
                if location != 0 {
                    fn_00983250(e, Ptr::new(location), 1);
                }
                let key = e.mem.u32(second + 0x68);
                e.call(MAP_REMOVE_AT, &args![this, key]);
            }
            e.call(SECOND_SET_RESERVED, &args![second, 0u32]);
        }
    });
    e.call(UNLOCK_WRITE, &args![this.addr() + 0x14]);
}

// Translated from 009835d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(key, word, owner, width, search, a,
/// b)`): under the read lock at +0x14, false when the map's location for
/// `key` refuses `owner` ([`fn_00982df0`]); otherwise the slot search
/// [`fn_009827c0`] on `search` with `(word, owner, width, location, a, b)`
/// (a null location when the key is missing).
pub fn fn_009835d0(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    word: u32,
    owner: u32,
    width: f32,
    search: u32,
    a: u8,
    b: u8,
) -> bool {
    e.call(LOCK_FOR_READ, &args![this.addr() + 0x14]);
    let result = e.with_stack(4, |e, out| {
        let mut result = true;
        if e.call(MAP_GET_AT, &args![this, key, out]).bool() {
            let location = e.mem.u32(out.addr());
            if !fn_00982df0(e, Ptr::new(location), owner) {
                result = false;
            }
        }
        if result {
            let location = e.mem.u32(out.addr());
            result = fn_009827c0(
                e,
                Ptr::new(search),
                word,
                owner,
                x87_quiet(width),
                location,
                a,
                b,
            );
        }
        result
    });
    e.call(UNLOCK_READ, &args![this.addr() + 0x14]);
    result
}

// Translated from 009836a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int, ReservedCoverLocation*>`'s scalar deleting
/// destructor (Xbox PDB): [`fn_00983820`], then `operator delete` when bit 0
/// of `flags` is set. Returns `this`.
pub fn ni_t_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00983820(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009836d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<CoverReservation, 1024>` constructor (unnamed in the
/// engine map; by the vtable it sets, `0108d124`): the vtable, then
/// `006dc440(0, 0)`. Returns `this`.
pub fn fn_009836d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108D124);
    e.call(ARRAY_INITIALIZE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 009838b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CoverReservation, 1024>`'s scalar deleting destructor
/// (Xbox PDB): [`fn_00983700`], then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn bs_simple_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00983700(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009838e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., unsigned int, ReservedCoverLocation*>`'s scalar
/// deleting destructor (Xbox PDB): [`fn_00983880`], then `operator delete`
/// when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00983880(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// The cover map (`009611e0` on the object at `011f1958`).
fn cover_map(e: &mut Engine) -> Ptr {
    let owner: u32 = e.global(COVER_MAP_OWNER);
    e.call(POINTER_TARGET_OWNER, &args![owner]).ptr()
}

// Translated from 009a0510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatCoverMap::ReserveCoverLocation` (Xbox PDB; `thiscall(second)`):
/// for a non-null `second` that fails `005dc960`, the cover map's reserve
/// ([`fn_00983280`]) with the word at +0x1C4 as the owner.
pub fn combat_cover_map_reserve_cover_location(e: &mut Engine, this: Ptr, second: u32) {
    if second == 0 || e.call(RESERVE_TEST, &args![second]).bool() {
        return;
    }
    let owner = e.mem.u32(this.addr() + 0x1c4);
    let map = cover_map(e);
    fn_00983280(e, map, owner, second);
}

// Translated from 009a0560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(second)`): true for a null `second`
/// or one `005dc960` accepts; otherwise the cover map's checked reserve
/// ([`fn_00983420`]) with the word at +0x1C4.
pub fn fn_009a0560(e: &mut Engine, this: Ptr, second: u32) -> bool {
    if second == 0 || e.call(RESERVE_TEST, &args![second]).bool() {
        return true;
    }
    let owner = e.mem.u32(this.addr() + 0x1c4);
    let map = cover_map(e);
    fn_00983420(e, map, owner, second)
}

// Translated from 009a05b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatCoverMap::UnreserveCoverLocation` (Xbox PDB; `thiscall(second)`):
/// for a non-null `second` that passes `005dc960`, the cover map's release
/// ([`fn_00983520`]) with the word at +0x1C4.
pub fn combat_cover_map_unreserve_cover_location(e: &mut Engine, this: Ptr, second: u32) {
    if second == 0 || !e.call(RESERVE_TEST, &args![second]).bool() {
        return;
    }
    let owner = e.mem.u32(this.addr() + 0x1c4);
    let map = cover_map(e);
    fn_00983520(e, map, owner, second);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0098_3700, fn_00983700(Ptr)),
        entry!(
            0x0098_3100,
            combat_cover_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0098_3130, fn_00983130(Ptr)),
        entry!(0x0098_3670, fn_00983670(Ptr, u32) -> Ptr),
        entry!(0x0098_3720, fn_00983720(Ptr, u32, u32)),
        entry!(0x0098_37b0, fn_009837b0(Ptr, u32) -> Ptr),
        entry!(0x0098_3820, fn_00983820(Ptr)),
        entry!(0x0098_3880, fn_00983880(Ptr)),
        entry!(0x0098_27c0, fn_009827c0(Ptr, u32, u32, f32, u32, u8, u8) -> bool),
        entry!(0x0098_2d10, fn_00982d10(Ptr)),
        entry!(0x0098_2df0, fn_00982df0(Ptr, u32) -> bool),
        entry!(0x0098_2e70, fn_00982e70(Ptr, u32, u32)),
        entry!(0x0098_2f40, fn_00982f40(Ptr, u32, u32) -> bool),
        entry!(0x0098_2ff0, fn_00982ff0(Ptr, u32, u32) -> bool),
        entry!(0x0098_3250, fn_00983250(Ptr, u32) -> Ptr),
        entry!(0x0098_3280, fn_00983280(Ptr, u32, u32)),
        entry!(0x0098_3380, fn_00983380(Ptr) -> Ptr),
        entry!(0x0098_3400, fn_00983400(Ptr, u8)),
        entry!(0x0098_3420, fn_00983420(Ptr, u32, u32) -> bool),
        entry!(0x0098_3520, fn_00983520(Ptr, u32, u32)),
        entry!(0x0098_35d0, fn_009835d0(Ptr, u32, u32, u32, f32, u32, u8, u8) -> bool),
        entry!(0x0098_36a0, ni_t_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0098_36d0, fn_009836d0(Ptr) -> Ptr),
        entry!(0x0098_38b0, bs_simple_array_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0098_38e0, ni_t_map_base_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x009a_0510,
            combat_cover_map_reserve_cover_location(Ptr, u32)
        ),
        entry!(0x009a_0560, fn_009a0560(Ptr, u32) -> bool),
        entry!(
            0x009a_05b0,
            combat_cover_map_unreserve_cover_location(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_00983700_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0098_3700, ARRAY_VTABLE_0108D124, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod cover_map_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

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
        for addr in [
            MAP_REMOVE_ALL,
            MAP_FREE,
            MEMSET,
            RESERVED_LOCATION_DELETE,
            POINTER_DESTRUCT,
            OPERATOR_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MAP_ALLOCATE, |_, a| (0x0800_0000 + a[0]).into_ret());
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn map_constructors_set_the_fields_and_vtables() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + MAP_COUNT, 9);
        let back = e.call(0x0098_3670, &args![this, 0x25u32]).u32();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this), NI_TMAP_VTABLE);
        assert_eq!(e.mem.u32(this + HASH_SIZE), 0x25);
        assert_eq!(e.mem.u32(this + MAP_COUNT), 0);
        assert_eq!(e.mem.u32(this + HASH_TABLE), 0x0800_0094);
        assert_eq!(calls(&e, MAP_ALLOCATE), vec![vec![0x94]]);
        assert_eq!(calls(&e, MEMSET), vec![vec![0x0800_0094, 0, 0x94]]);
        e.call(0x0098_37b0, &args![this, 2u32]);
        assert_eq!(e.mem.u32(this), NI_TMAP_BASE_VTABLE);
    }

    #[test]
    fn map_destructors_empty_and_free() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + HASH_TABLE, 0x4321);
        e.call(0x0098_3820, &args![this]);
        assert_eq!(e.mem.u32(this), NI_TMAP_BASE_VTABLE);
        assert_eq!(calls(&e, MAP_REMOVE_ALL), vec![vec![this], vec![this]]);
        assert_eq!(calls(&e, MAP_FREE), vec![vec![0x4321]]);
        e.call_log = Some(vec![]);
        e.call(0x0098_3880, &args![this]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL), vec![vec![this]]);
    }

    #[test]
    fn cover_map_destructor_deletes_locations_and_notifies_the_owner() {
        let mut e = engine();
        // Two positions: the first yields a location, the second a null one.
        let values = Rc::new(RefCell::new(vec![0x6000u32, 0]));
        e.register(MAP_FIRST_POSITION, |_, _| 1u32.into_ret());
        let next = values.clone();
        e.register_double(MAP_GET_NEXT, move |e, a| {
            let value = next.borrow_mut().remove(0);
            e.mem.set_u32(a[3], value);
            let left = u32::from(!next.borrow().is_empty());
            e.mem.set_u32(a[1], left);
            Ret::default()
        });
        let owner = e.mem.alloc(4);
        let vtable = e.mem.alloc(0xf0);
        e.mem.set_u32(owner, vtable);
        e.mem.set_u32(vtable + 0xe8, 0x0066_0000);
        e.register(0x0066_0000, |_, _| Ret::default());
        e.register_double(POINTER_TARGET_OWNER, move |_, _| owner.into_ret());
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + COVER_MAP_POINTER, 0x7777);
        e.call(0x0098_3100, &args![this, 1u32]);
        assert_eq!(calls(&e, RESERVED_LOCATION_DELETE), vec![vec![0x6000, 1]]);
        assert_eq!(calls(&e, MAP_GET_NEXT).len(), 2);
        assert_eq!(calls(&e, 0x0066_0000), vec![vec![owner, 0x7777]]);
        assert_eq!(calls(&e, POINTER_DESTRUCT), vec![vec![this + 0x10]]);
        assert_eq!(e.mem.u32(this), NI_TMAP_BASE_VTABLE);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn cover_map_destructor_without_a_pointer_or_owner() {
        let mut e = engine();
        e.register(MAP_FIRST_POSITION, |_, _| 0u32.into_ret());
        e.register(POINTER_TARGET_OWNER, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x18);
        e.call(0x0098_3100, &args![this, 0u32]);
        assert!(calls(&e, MAP_GET_NEXT).is_empty());
        assert!(calls(&e, POINTER_TARGET_OWNER).is_empty());
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.mem.set_u32(this + COVER_MAP_POINTER, 0x7777);
        e.call_log = Some(vec![]);
        e.call(0x0098_3130, &args![this]);
        assert_eq!(calls(&e, POINTER_TARGET_OWNER), vec![vec![0x7777]]);
    }

    #[test]
    fn remove_at_destroys_and_moves_the_tail() {
        let mut e = engine();
        e.register(0x0042_f5a0, |_, a| a[0].min(a[1]).into_ret());
        e.register(0x0072_ba80, |_, _| Ret::default());
        e.register(0x0069_2480, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x5000);
        e.mem.set_u32(this + 8, 10);
        e.call(0x0098_3720, &args![this, 2u32, 3u32]);
        assert_eq!(calls(&e, 0x0042_f5a0), vec![vec![5, 3]]);
        assert_eq!(calls(&e, 0x0072_ba80), vec![vec![this, 0x5010, 3]]);
        assert_eq!(
            calls(&e, 0x0069_2480),
            vec![vec![this, 0x5010, 0x5050 - 0x18, 3]]
        );
        assert_eq!(e.mem.u32(this + 8), 7);
    }
}

#[cfg(test)]
mod reservation_tests {
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
        e.map(0x0101_6000, 0x1000);
        e.set_global(WIDTH_DIVISOR, 1.5f64);
        e.call_log = Some(vec![]);
        e
    }

    /// The slot search's object: bytes ready, count, width, slot, a, b; the
    /// mask `00982b90` returns at +8. The location answers count (+0), ready
    /// (+1) and its taken mask (+4).
    fn search(e: &mut Engine, count: u8, width: u8, mask: u32) -> u32 {
        let this = e.mem.alloc(0x10);
        e.mem.set_u8(this, 1);
        e.mem.set_u8(this + 1, count);
        e.mem.set_u8(this + 2, width);
        e.mem.set_u32(this + 8, mask);
        e.register(SEARCH_RESET, |_, _| Ret::default());
        e.register(SEARCH_WIDTH, |_, _| 2u8.into_ret());
        e.register(SEARCH_FIT, |_, a| (a[0] as u8).into_ret());
        e.register(SLOT_MASK, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(SEARCH_FAILED, |_, _| Ret::default());
        e.register(SEARCH_CENTERED, |e, a| e.mem.u8(a[0]).into_ret());
        e.register(LOCATION_COUNT, |e, a| e.mem.u8(a[0]).into_ret());
        e.register(LOCATION_READY, |e, a| e.mem.u8(a[0] + 1).into_ret());
        e.register(LOCATION_TAKEN, |e, a| e.mem.u32(a[0] + 4).into_ret());
        this
    }

    fn location(e: &mut Engine, count: u8, taken: u32) -> u32 {
        let location = e.mem.alloc(8);
        e.mem.set_u8(location, count);
        e.mem.set_u8(location + 1, 1);
        e.mem.set_u32(location + 4, taken);
        location
    }

    fn run(e: &mut Engine, this: u32, word: u32, location: u32, a: u8, b: u8) -> bool {
        e.call(
            0x0098_27c0,
            &args![this, word, 0u32, 3.0f32, location, a, b],
        )
        .bool()
    }

    #[test]
    fn slot_search_needs_a_ready_location_and_a_fitting_width() {
        let mut e = engine();
        let word = e.mem.alloc(0x50);
        let this = search(&mut e, 0, 0, 1);
        e.mem.set_u8(this, 0);
        // No location: 00982630 and nothing ready.
        assert!(!run(&mut e, this, word, 0, 0, 0));
        assert_eq!(calls(&e, SEARCH_RESET), vec![vec![this, word]]);
        // A location gives count 3; a zero width comes from 00982750 (2).
        let place = location(&mut e, 3, 0);
        e.mem.set_u8(word, 1);
        assert!(run(&mut e, this, word, place, 0, 0));
        assert_eq!(calls(&e, SEARCH_WIDTH), vec![vec![this, 3.0f32.to_bits()]]);
        assert_eq!(calls(&e, SEARCH_FIT), vec![vec![2, 3]]);
        // Centered: slot count - width.
        assert_eq!(e.mem.u8(this + 3), 1);
        // Width 6 needs 4 places: refused.
        e.mem.set_u8(this + 2, 6);
        assert!(!run(&mut e, this, word, place, 0, 0));
        // Width 4 (4 / 1.5 truncates to 2) fits a count of 3.
        e.mem.set_u8(this + 2, 4);
        e.mem.set_u8(this + 3, 0);
        assert!(run(&mut e, this, word, place, 0, 1));
        assert_eq!(e.mem.u8(this + 3), 0);
        assert_eq!(e.mem.u8(this + 5), 1);
    }

    #[test]
    fn a_kept_slot_stays_while_free() {
        let mut e = engine();
        let word = e.mem.alloc(0x50);
        let this = search(&mut e, 8, 2, 0b10);
        let place = location(&mut e, 8, 0b1);
        e.mem.set_u8(this + 3, 5);
        e.mem.set_u8(this + 5, 7);
        assert!(run(&mut e, this, word, place, 9, 7));
        assert_eq!(e.mem.u8(this + 3), 5);
        assert_eq!(e.mem.u8(this + 4), 9);
        assert_eq!(calls(&e, SEARCH_CENTERED).len(), 0);
        // A changed `b` searches again (here: centered, slot 0 for b set).
        e.mem.set_u8(word, 1);
        assert!(run(&mut e, this, word, place, 0, 1));
        assert_eq!(e.mem.u8(this + 3), 0);
        // Centered but taken: 00982b50 and false.
        e.mem.set_u32(place + 4, 0b10);
        assert!(!run(&mut e, this, word, place, 0, 1));
        assert_eq!(calls(&e, SEARCH_FAILED), vec![vec![this]]);
    }

    #[test]
    fn half_width_counts_scan_from_one_end() {
        let mut e = engine();
        let word = e.mem.alloc(0x50);
        let this = search(&mut e, 6, 3, 0b1);
        // Taken: slots 0 and 3 (count 6, width 3: slots 0..=3).
        let place = location(&mut e, 6, 0b1001);
        assert!(run(&mut e, this, word, place, 1, 0));
        assert_eq!(e.mem.u8(this + 3), 1);
        e.mem.set_u8(word + 0x4f, 1);
        assert!(run(&mut e, this, word, place, 1, 0));
        assert_eq!(e.mem.u8(this + 3), 2);
        e.mem.set_u32(place + 4, 0b1111);
        assert!(!run(&mut e, this, word, place, 1, 0));
    }

    #[test]
    fn other_counts_scan_outward_from_the_middle() {
        let mut e = engine();
        let word = e.mem.alloc(0x50);
        // Count 7, width 2: center 3 - 1 = 2, slots 0..=5.
        let this = search(&mut e, 7, 2, 0b1);
        let place = location(&mut e, 7, 0b0110);
        assert!(run(&mut e, this, word, place, 1, 0));
        assert_eq!(e.mem.u8(this + 3), 3);
        e.mem.set_u32(place + 4, 0b1111);
        assert!(run(&mut e, this, word, place, 1, 0));
        assert_eq!(e.mem.u8(this + 3), 4);
        e.mem.set_u32(place + 4, 0b0011_1111);
        assert!(!run(&mut e, this, word, place, 1, 0));
    }

    #[test]
    fn location_destructor_hands_the_pointer_on() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, 0x7000);
        let owner = e.mem.alloc(8);
        e.put_vtable(0x0200_0000, &[0; 0x3b]);
        e.mem.set_u32(0x0200_00e8, 0x0300_00e8);
        e.mem.set_u32(owner, 0x0200_0000);
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register_double(POINTER_TARGET_OWNER, move |_, _| owner.into_ret());
        e.map(0x011d_f000, 0x1000);
        e.register(DESTROY_TEST, |e, _| e.mem.u8(0x011d_f000).into_ret());
        e.register(QUEUE_GLOBAL, |_, _| 0x6000u32.into_ret());
        for addr in [
            0x0300_00e8,
            QUEUE_OBJECT,
            POINTER_DESTRUCT,
            ARRAY_RELEASE_008454F0,
            OPERATOR_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        assert_eq!(e.call(0x0098_3250, &args![this, 0u32]).u32(), this);
        assert_eq!(calls(&e, 0x0300_00e8), vec![vec![owner, 0x7000]]);
        assert_eq!(calls(&e, POINTER_DESTRUCT), vec![vec![this + 0x18]]);
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE_0108D124);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.mem.set_u8(0x011d_f000, 1);
        e.call(0x0098_3250, &args![this, 1u32]);
        assert_eq!(calls(&e, QUEUE_OBJECT), vec![vec![0x6000, 0x7000]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
        // A null pointer: only the destructors.
        e.call_log = Some(vec![]);
        e.mem.set_u32(this + 0x18, 0);
        e.call(0x0098_2d10, &args![this]);
        assert!(calls(&e, POINTER_TARGET_OWNER).is_empty());
        assert_eq!(calls(&e, POINTER_DESTRUCT).len(), 1);
    }

    /// A location whose array holds the reservations `owners` (8-byte
    /// elements; count `0044ddc0`, element `00844160`), at +0 its buffer
    /// address and +4 the count.
    fn reservations(e: &mut Engine, owners: &[u32]) -> u32 {
        let this = e.mem.alloc(0x20);
        let buffer = e.mem.alloc(0x80);
        e.mem.set_u32(this + 4, buffer);
        e.mem.set_u32(this + 8, owners.len() as u32);
        for (i, owner) in owners.iter().enumerate() {
            e.mem.set_u32(buffer + 8 * i as u32, *owner);
        }
        e.register(ARRAY_COUNT, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ARRAY_AT, |e, a| (e.mem.u32(a[0] + 4) + 8 * a[1]).into_ret());
        e.register(ARRAY_APPEND, |e, a| {
            let count = e.mem.u32(a[0] + 8);
            let slot = e.mem.u32(a[0] + 4) + 8 * count;
            let pair = e.mem.u64(a[1]);
            e.mem.set_u64(slot, pair);
            e.mem.set_u32(a[0] + 8, count + 1);
            Ret::default()
        });
        e.register(PAIR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        e.register(SLOT_MASK, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(MASK_PREPARE, |_, _| Ret::default());
        this
    }

    #[test]
    fn owner_check_refuses_holders_and_targets() {
        let mut e = engine();
        let this = reservations(&mut e, &[0x100, 0x200]);
        e.register(OWNER_WORD, |_, a| (a[0] + 1).into_ret());
        e.register(OWNER_GROUP, |_, a| a[0].into_ret());
        e.register(GROUP_IS_TARGET, |_, a| {
            (a[0] == 0x900 && a[1] == 0x201).into_ret()
        });
        assert!(e.call(0x0098_2df0, &args![this, 0u32]).bool());
        assert!(!e.call(0x0098_2df0, &args![this, 0x200u32]).bool());
        assert!(!e.call(0x0098_2df0, &args![this, 0x900u32]).bool());
        assert!(e.call(0x0098_2df0, &args![this, 0x800u32]).bool());
        assert_eq!(
            calls(&e, GROUP_IS_TARGET)[1..3],
            [vec![0x900, 0x101], vec![0x900, 0x201]]
        );
    }

    #[test]
    fn adding_a_reservation_merges_its_mask() {
        let mut e = engine();
        let this = reservations(&mut e, &[0x100]);
        let second = e.mem.alloc(0x80);
        e.mem.set_u32(second + 0x70, 0b100);
        e.mem.set_u32(this + 0x10, 0b001);
        e.call(0x0098_2e70, &args![this, 0x100u32, second]);
        assert_eq!(e.mem.u32(this + 8), 1);
        e.call(0x0098_2e70, &args![this, 0x300u32, second]);
        assert_eq!(e.mem.u32(this + 8), 2);
        assert_eq!(e.mem.u32(this + 0x10), 0b101);
        assert_eq!(calls(&e, MASK_PREPARE), vec![vec![this, second + 0x70]]);
        let buffer = e.mem.u32(this + 4);
        assert_eq!(
            (e.mem.u32(buffer + 8), e.mem.u32(buffer + 12)),
            (0x300, second)
        );
        // The checked add refuses holders and overlapping masks.
        assert!(!e.call(0x0098_2f40, &args![this, 0x300u32, second]).bool());
        assert!(!e.call(0x0098_2f40, &args![this, 0x400u32, second]).bool());
        e.mem.set_u8(this + 0x15, 1);
        e.mem.set_u32(second + 0x70, 0b10);
        assert!(e.call(0x0098_2f40, &args![this, 0x400u32, second]).bool());
        assert_eq!(e.mem.u32(this + 0x10), 0b111);
        assert_eq!(e.mem.u32(this + 8), 3);
        assert_eq!(calls(&e, MASK_PREPARE).len(), 2);
    }

    #[test]
    fn releasing_removes_the_reservation_unless_it_is_the_last() {
        let mut e = engine();
        let this = reservations(&mut e, &[0x100, 0x200]);
        let second = e.mem.alloc(0x80);
        e.mem.set_u32(second + 0x70, 0b10);
        e.mem.set_u32(this + 0x10, 0b11);
        e.register(0x0042_f5a0, |_, a| a[0].min(a[1]).into_ret());
        e.register(0x0072_ba80, |_, _| Ret::default());
        e.register(0x0069_2480, |_, _| Ret::default());
        assert!(!e.call(0x0098_2ff0, &args![this, 0x300u32, second]).bool());
        assert!(!e.call(0x0098_2ff0, &args![this, 0x200u32, second]).bool());
        assert_eq!(e.mem.u32(this + 0x10), 0b01);
        assert_eq!(e.mem.u32(this + 8), 1);
        assert_eq!(calls(&e, 0x0072_ba80)[0][1], e.mem.u32(this + 4) + 8);
        assert!(e.call(0x0098_2ff0, &args![this, 0x100u32, second]).bool());
        assert_eq!(e.mem.u32(this + 8), 1);
    }

    /// The cover map (its lock at +0x14) with a lookup that finds the
    /// location stored at +0x40 for the key stored at +0x44.
    fn cover_map_engine() -> (Engine, u32, u32) {
        let mut e = engine();
        let map = e.mem.alloc(0x50);
        let second = e.mem.alloc(0x80);
        e.mem.set_u32(second + 0x68, 0x55);
        e.register(MAP_GET_AT, |e, a| {
            let found = e.mem.u32(a[0] + 0x44) == a[1] && a[1] != 0;
            if found {
                let location = e.mem.u32(a[0] + 0x40);
                e.mem.set_u32(a[2], location);
            }
            found.into_ret()
        });
        e.register(MAP_SET_AT, |e, a| {
            e.mem.set_u32(a[0] + 0x44, a[1]);
            e.mem.set_u32(a[0] + 0x40, a[2]);
            Ret::default()
        });
        e.register(MAP_REMOVE_AT, |e, a| {
            e.mem.set_u32(a[0] + 0x44, 0);
            Ret::default()
        });
        for addr in [
            LOCK_FOR_WRITE,
            UNLOCK_WRITE,
            LOCK_FOR_READ,
            UNLOCK_READ,
            SECOND_SET_RESERVED,
            POINTER_CONSTRUCT,
            ARRAY_INITIALIZE,
            POINTER_DESTRUCT,
            ARRAY_RELEASE_008454F0,
            OPERATOR_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SECOND_FLAG, |_, _| 3u8.into_ret());
        (e, map, second)
    }

    #[test]
    fn reserve_makes_the_location_once() {
        let (mut e, map, second) = cover_map_engine();
        reservations(&mut e, &[]);
        e.register(ARRAY_INITIALIZE, |e, a| {
            let buffer = e.mem.alloc(0x40);
            e.mem.set_u32(a[0] + 4, buffer);
            Ret::default()
        });
        e.call(0x0098_3280, &args![map, 0x100u32, second]);
        let location = e.mem.u32(map + 0x40);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x1c]]);
        assert_eq!(e.mem.u32(location), ARRAY_VTABLE_0108D124);
        assert_eq!(e.mem.u8(location + 0x16), 3);
        assert_eq!(calls(&e, ARRAY_INITIALIZE), vec![vec![location, 0, 0]]);
        assert_eq!(calls(&e, POINTER_CONSTRUCT), vec![vec![location + 0x18, 0]]);
        assert_eq!(calls(&e, MAP_SET_AT), vec![vec![map, 0x55, location]]);
        assert_eq!(calls(&e, SECOND_SET_RESERVED), vec![vec![second, 1]]);
        assert_eq!(calls(&e, LOCK_FOR_WRITE), vec![vec![map + 0x14]]);
        assert_eq!(calls(&e, UNLOCK_WRITE), vec![vec![map + 0x14]]);
        assert_eq!(calls(&e, ARRAY_APPEND).len(), 1);
        // The checked reserve finds it and refuses the same owner.
        assert!(!e.call(0x0098_3420, &args![map, 0x100u32, second]).bool());
        assert_eq!(calls(&e, OPERATOR_NEW).len(), 1);
        assert_eq!(calls(&e, SECOND_SET_RESERVED)[1], vec![second, 0]);
        e.mem.set_u32(second + 0x70, 0b1000);
        assert!(e.call(0x0098_3420, &args![map, 0x200u32, second]).bool());
        assert_eq!(calls(&e, SECOND_SET_RESERVED)[2], vec![second, 1]);
    }

    #[test]
    fn release_deletes_an_emptied_location() {
        let (mut e, map, second) = cover_map_engine();
        let location = reservations(&mut e, &[0x100]);
        e.mem.set_u32(map + 0x40, location);
        e.mem.set_u32(map + 0x44, 0x55);
        e.call(0x0098_3520, &args![map, 0x100u32, second]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![location]]);
        assert_eq!(calls(&e, MAP_REMOVE_AT), vec![vec![map, 0x55]]);
        assert_eq!(calls(&e, SECOND_SET_RESERVED), vec![vec![second, 0]]);
        // Missing key: only the lock.
        e.call_log = Some(vec![]);
        e.call(0x0098_3520, &args![map, 0x100u32, second]);
        assert!(calls(&e, SECOND_SET_RESERVED).is_empty());
        assert_eq!(calls(&e, UNLOCK_WRITE).len(), 1);
        // A zero key: nothing at all.
        e.call_log = Some(vec![]);
        e.mem.set_u32(second + 0x68, 0);
        e.call(0x0098_3520, &args![map, 0x100u32, second]);
        assert!(calls(&e, LOCK_FOR_WRITE).is_empty());
    }

    #[test]
    fn find_slot_checks_the_owner_first() {
        let (mut e, map, _) = cover_map_engine();
        let location = reservations(&mut e, &[0x100]);
        e.mem.set_u32(map + 0x40, location);
        e.mem.set_u32(map + 0x44, 0x55);
        e.register(OWNER_WORD, |_, _| 0u32.into_ret());
        e.register(OWNER_GROUP, |_, _| 0u32.into_ret());
        e.register(GROUP_IS_TARGET, |_, _| false.into_ret());
        let word = e.mem.alloc(0x50);
        let finder = search(&mut e, 0, 0, 1);
        e.mem.set_u8(finder, 0);
        let find = |e: &mut Engine, key: u32, owner: u32| {
            e.call(
                0x0098_35d0,
                &args![map, key, word, owner, 1.0f32, finder, 1u32, 0u32],
            )
            .bool()
        };
        assert!(!find(&mut e, 0x55, 0x100));
        assert!(calls(&e, SEARCH_RESET).is_empty());
        assert!(calls(&e, LOCATION_COUNT).is_empty());
        // Another owner: the search runs on the location (count 0: false).
        e.register(LOCATION_COUNT, |_, _| 0u8.into_ret());
        e.register(LOCATION_READY, |_, _| 1u8.into_ret());
        e.register(LOCATION_TAKEN, |_, _| 0u32.into_ret());
        assert!(!find(&mut e, 0x55, 0x200));
        assert_eq!(calls(&e, LOCATION_COUNT), vec![vec![location]]);
        // A missing key searches without a location.
        assert!(!find(&mut e, 0x66, 0x200));
        assert_eq!(calls(&e, SEARCH_RESET), vec![vec![finder, word]]);
        assert_eq!(calls(&e, LOCK_FOR_READ).len(), 3);
        assert_eq!(calls(&e, UNLOCK_READ).len(), 3);
    }

    #[test]
    fn constructors_and_deleting_destructors() {
        let (mut e, _, _) = cover_map_engine();
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(MAP_FREE, |_, _| Ret::default());
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block + 0x10, 9);
        e.mem.set_u8(block + 0x15, 9);
        assert_eq!(e.call(0x0098_3380, &args![block]).u32(), block);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        assert_eq!(e.mem.u8(block + 0x15), 0);
        e.call(0x0098_3400, &args![block, 4u32]);
        assert_eq!(e.mem.u8(block + 0x16), 4);
        assert_eq!(e.call(0x0098_36d0, &args![block]).u32(), block);
        for (addr, vtable) in [
            (0x0098_36a0, NI_TMAP_BASE_VTABLE),
            (0x0098_38b0, ARRAY_VTABLE_0108D124),
            (0x0098_38e0, NI_TMAP_BASE_VTABLE),
        ] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![block, 1u32]).u32(), block);
            assert_eq!(e.mem.u32(block), vtable);
            assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![block]]);
            e.call(addr, &args![block, 0u32]);
            assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
        }
    }

    #[test]
    fn controller_reserve_calls_go_to_the_cover_map() {
        let (mut e, _, second) = cover_map_engine();
        e.map(0x011f_1000, 0x1000);
        e.set_global(COVER_MAP_OWNER, 0x4400u32);
        let map = e.mem.alloc(0x50);
        e.register_double(POINTER_TARGET_OWNER, move |_, _| map.into_ret());
        e.register(RESERVE_TEST, |e, a| e.mem.u8(a[0] + 0x7c).into_ret());
        // The map holds the key's location, already reserved by 0x77.
        let location = reservations(&mut e, &[0x77]);
        e.mem.set_u32(map + 0x40, location);
        e.mem.set_u32(map + 0x44, 0x55);
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this + 0x1c4, 0x77);
        e.call(0x009a_0510, &args![this, 0u32]);
        assert!(e.call(0x009a_0560, &args![this, 0u32]).bool());
        e.call(0x009a_05b0, &args![this, 0u32]);
        e.call(0x009a_05b0, &args![this, second]);
        assert!(calls(&e, POINTER_TARGET_OWNER).is_empty());
        // Reserve: the owner word is already there, so only the mark.
        e.call(0x009a_0510, &args![this, second]);
        assert_eq!(calls(&e, POINTER_TARGET_OWNER), vec![vec![0x4400]]);
        assert_eq!(calls(&e, LOCK_FOR_WRITE), vec![vec![map + 0x14]]);
        assert_eq!(calls(&e, SECOND_SET_RESERVED), vec![vec![second, 1]]);
        // The checked reserve refuses the same owner.
        assert!(!e.call(0x009a_0560, &args![this, second]).bool());
        assert_eq!(calls(&e, SECOND_SET_RESERVED)[1], vec![second, 0]);
        // Once `005dc960` holds: 0560 is true at once and 05b0 releases.
        e.mem.set_u8(second + 0x7c, 1);
        assert!(e.call(0x009a_0560, &args![this, second]).bool());
        e.call(0x009a_0510, &args![this, second]);
        assert_eq!(calls(&e, LOCK_FOR_WRITE).len(), 2);
        e.call(0x009a_05b0, &args![this, second]);
        assert_eq!(calls(&e, MAP_REMOVE_AT), vec![vec![map, 0x55]]);
    }
}
