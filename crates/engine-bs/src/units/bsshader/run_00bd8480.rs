//! `BSShader/run_00bd8480` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Most functions here work on one unnamed object (no engine-map name; its constructor is
//! `00bd8ae0`). The offsets the code uses:
//!
//! - +0x00 a reference-counted geometry (`NiPointer`), made by `00c1a480` or `00c1a660`;
//! - +0x04 a reference-counted `BSMultiBoundNode`;
//! - +0x08 the reference-counted source object the constructor is given;
//! - +0x0C a reference-counted shader property (made by `00b60cb0`);
//! - +0x10 the entry capacity (`u16`), +0x12 the entry count (`u16`);
//! - +0x14 the entries, 16 bytes each (three floats and a fourth word), +0x18 one word per
//!   entry;
//! - +0x1C a member constructed by `00b600e0` (0x20 bytes are copied over it by `00bd86a0`),
//!   +0x38..+0x3A three flag bytes inside it;
//! - +0x3C, +0x40 two words from the constructor, +0x44 a flag byte.
//!
//! The global word at `011f9ef8` counts the entries in use over all such objects.

#[allow(unused_imports)]
use crate::prelude::*;

/// The count word whose `count * 4` bytes `00bd9120` allocates.
const SLOT_COUNT: u32 = 0x011b_f558;
/// The memory manager object the code passes as `this` to its allocator.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The memory manager's allocation method (thiscall, one word: the size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;

/// The memory manager's deallocation (`00aa4060`, `thiscall`, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The allocator `00aa13e0` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x00aa_13e0;
/// The sized free `00aa1460` (`cdecl`, two words: the block and its size).
const FREE_SIZED: u32 = 0x00aa_1460;
/// `InterlockedDecrement` (KERNEL32 import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `InterlockedIncrement` (KERNEL32 import slot).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;
/// `memset` (`00ec61c0`, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;
/// `_CIsqrt` (`00ec6040`): the square root of ST0.
const SQRT: u32 = 0x00ec_6040;
/// The destructor body `00bd74a0` (`bscubemapcamera.cpp`, `thiscall`).
const DESTRUCTOR_00BD74A0: u32 = 0x00bd_74a0;
/// The global count of entries in use.
const ENTRIES_IN_USE: u32 = 0x011f_9ef8;
/// The `float` an empty entry's third component is set to (`0104eb20`).
const EMPTY_ENTRY_Z: u32 = 0x0104_eb20;
/// The `float` the bound search starts from (`010bc3ec`).
const BOUND_START: u32 = 0x010b_c3ec;
/// `0.5` (`double`, `01011588`).
const HALF: u32 = 0x0101_1588;
/// The class record `00bd8750` checks the geometry against (`012024c0`).
const GEOMETRY_RTTI_012024C0: u32 = 0x0120_24c0;
/// The argument `00653290` is called with (`012024b8`).
const LOOKUP_ARGUMENT_012024B8: u32 = 0x0120_24b8;
/// `00653290` (`thiscall`, one word): `this` or null.
const LOOKUP_00653290: u32 = 0x0065_3290;
/// `NiAVObject::AttachProperty` (engine map, `00439410`, `thiscall`, one word).
const ATTACH_PROPERTY: u32 = 0x0043_9410;
/// The member constructor `00b600e0` (`thiscall`).
const MEMBER_CONSTRUCTOR_00B600E0: u32 = 0x00b6_00e0;
/// The geometry constructors `00c1a480` and `00c1a660` (`thiscall`, one word).
const GEOMETRY_CONSTRUCTOR_00C1A480: u32 = 0x00c1_a480;
const GEOMETRY_CONSTRUCTOR_00C1A660: u32 = 0x00c1_a660;
/// The shader property constructor `00b60cb0` (`thiscall`).
const PROPERTY_CONSTRUCTOR_00B60CB0: u32 = 0x00b6_0cb0;
/// `NiAdditionalGeometryData::NiAdditionalGeometryData_ov2` (`thiscall`, two words).
const ADDITIONAL_DATA_CONSTRUCTOR: u32 = 0x00ba_63e0;
/// `NiAdditionalGeometryData::SetDataBlockCount` (`thiscall`, one word).
const SET_DATA_BLOCK_COUNT: u32 = 0x00a7_2ca0;
/// `NiAdditionalGeometryData::SetDataBlock` (`thiscall`, four words).
const SET_DATA_BLOCK: u32 = 0x00a7_2bb0;
/// `NiAdditionalGeometryData::SetDataStream` (`thiscall`, seven words).
const SET_DATA_STREAM: u32 = 0x00a7_26a0;
/// `00a67260` (`nigeometrydata.cpp`, `thiscall`, one word).
const SET_ADDITIONAL_DATA_00A67260: u32 = 0x00a6_7260;
/// `BSShaderManager::PrepareObject` (`cdecl`, three words).
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
/// `BSMultiBoundNode::BSMultiBoundNode` (`thiscall`).
const MULTI_BOUND_NODE_CONSTRUCTOR: u32 = 0x00c4_6970;
/// `BSMultiBound::BSMultiBound` (`thiscall`).
const MULTI_BOUND_CONSTRUCTOR: u32 = 0x00c3_5e00;
/// `BSMultiBoundAABB::BSMultiBoundAABB` (`thiscall`).
const MULTI_BOUND_AABB_CONSTRUCTOR: u32 = 0x00c3_7ec0;
/// The renderer setting word at `011f91c0` and the byte at `011f941e` that choose which of
/// the source's two property words (+0x18 or +0x1c) is attached.
const RENDERER_LEVEL_011F91C0: u32 = 0x011f_91c0;
const RENDERER_FLAG_011F941E: u32 = 0x011f_941e;

// Translated from 00bd9120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a one-word object (unnamed in the engine map; the
/// `BSShaderAccumulator` and batch renderer constructors keep it as their
/// instance renderer): allocates `count * 4` bytes (`0xFFFFFFFF` when the
/// product overflows), the count being the word at `011bf558`, and stores
/// the block in the object's only word. Returns `this`.
pub fn fn_00bd9120(e: &mut Engine, this: Ptr) -> Ptr {
    let count = e.mem.u32(SLOT_COUNT);
    let size = count.checked_mul(4).unwrap_or(u32::MAX);
    let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, size]).u32();
    e.mem.set_u32(this.addr(), block);
    this
}

/// `NiPointer` release: when `object` is not null its count (+4) goes down
/// (`InterlockedDecrement`) and at zero it deletes itself (slot +4).
fn release(e: &mut Engine, object: u32) {
    if object != 0
        && e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// `NiPointer` assignment into the word at `slot`: nothing when it already holds `object`;
/// otherwise the old one is released, `object` stored, and its count raised
/// (`InterlockedIncrement`), skipped for null when `check_null` (the code tests the new
/// pointer at some sites and not at others).
fn assign(e: &mut Engine, slot: u32, object: u32, check_null: bool) {
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    release(e, old);
    e.mem.set_u32(slot, object);
    if !check_null || object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

/// `NiPointer` release that clears the word at `slot` afterwards.
fn release_and_clear(e: &mut Engine, slot: u32) {
    let old = e.mem.u32(slot);
    if old != 0 {
        release(e, old);
        e.mem.set_u32(slot, 0);
    }
}

/// `00aa13e0(size)`, then `construct(block, words..)` when the block is not null.
fn new_object(e: &mut Engine, size: u32, construct: u32, words: &[u32]) -> u32 {
    let block = e.call(ALLOCATE, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    let mut all = vec![block];
    all.extend_from_slice(words);
    e.call(construct, &all).u32()
}

// Translated from 00bd8480 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor (vtable `010bc470` slot 0; unnamed in the engine map;
/// `thiscall`, one word): runs the destructor body `00bd74a0` and frees 0x140 bytes when
/// bit 0 of the flags is set. Returns `this`.
pub fn fn_00bd8480(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_00BD74A0, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0x140u32]);
    }
    this
}

// Translated from 00bd8550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends an entry (`thiscall`, two words: the address of a 16-byte entry and a word):
/// when the count (+0x12) is below the capacity (+0x10) and the entry is not null, copies
/// it to slot `count` of the entries (+0x14), stores the word in slot `count` of the words
/// (+0x18), raises the count and returns 1 in `AL`. Otherwise returns `AL` 0 with the
/// count's high byte left in `AH` (the code only clears `AL`).
pub fn fn_00bd8550(e: &mut Engine, this: Ptr, entry: u32, word: u32) -> u32 {
    let count = e.mem.u16(this.addr() + 0x12);
    if count == e.mem.u16(this.addr() + 0x10) || entry == 0 {
        return u32::from(count) & 0xff00;
    }
    let slot = (u32::from(count) << 4).wrapping_add(e.mem.u32(this.addr() + 0x14));
    for i in 0..4 {
        let value = e.mem.u32(entry + 4 * i);
        e.mem.set_u32(slot + 4 * i, value);
    }
    let count = e.mem.u16(this.addr() + 0x12);
    let words = e.mem.u32(this.addr() + 0x18);
    e.mem
        .set_u32(words.wrapping_add(u32::from(count) * 4), word);
    e.mem.set_u16(this.addr() + 0x12, count.wrapping_add(1));
    1
}

// Translated from 00bd85b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes entry `index` (`thiscall`, one word; only its low 16 bits are used): when it is
/// below the count, the last entry and its word are moved into its place (unless it is the
/// last), the last entry is reset to (0, 0, the float at `0104eb20`, 0) and its word to 0,
/// the count goes down and so does the global count at `011f9ef8`. Returns the count
/// (`u16`).
pub fn fn_00bd85b0(e: &mut Engine, this: Ptr, index: u32) -> u16 {
    let base = this.addr();
    let count = e.mem.u16(base + 0x12);
    let index = index as u16;
    if index < count {
        let index = u32::from(index);
        if index != u32::from(count) - 1 {
            let entries = e.mem.u32(base + 0x14);
            let last = (u32::from(count) << 4)
                .wrapping_add(entries)
                .wrapping_sub(0x10);
            let slot = (index << 4).wrapping_add(entries);
            for i in 0..4 {
                let value = e.mem.u32(last + 4 * i);
                e.mem.set_u32(slot + 4 * i, value);
            }
            let count = u32::from(e.mem.u16(base + 0x12));
            let words = e.mem.u32(base + 0x18);
            let value = e.mem.u32(words.wrapping_add(count * 4).wrapping_sub(4));
            e.mem.set_u32(words.wrapping_add(index * 4), value);
        }
        // FLDZ; FST, FST; FLD [0104eb20]; FSTP; FSTP: the constant comes back quiet.
        let z = x87_quiet_bits(e.mem.u32(EMPTY_ENTRY_Z));
        for (offset, value) in [(0x10u32, 0u32), (0xc, 0), (8, z), (4, 0)] {
            let count = u32::from(e.mem.u16(base + 0x12));
            let entries = e.mem.u32(base + 0x14);
            e.mem.set_u32(
                (count << 4).wrapping_add(entries).wrapping_sub(offset),
                value,
            );
        }
        let count = u32::from(e.mem.u16(base + 0x12));
        let words = e.mem.u32(base + 0x18);
        e.mem
            .set_u32(words.wrapping_add(count * 4).wrapping_sub(4), 0);
        let count = e.mem.u16(base + 0x12);
        e.mem.set_u16(base + 0x12, count.wrapping_add(0xffff));
        let in_use = e.mem.u32(ENTRIES_IN_USE);
        e.mem.set_u32(ENTRIES_IN_USE, in_use.wrapping_sub(1));
    }
    e.mem.u16(base + 0x12)
}

// Translated from 00bd8670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes every entry whose word is `word` (`thiscall`, one word), walking from the last
/// index down to 0 ([`fn_00bd85b0`] for each match). Returns the count (`u16`).
pub fn fn_00bd8670(e: &mut Engine, this: Ptr, word: u32) -> u16 {
    let mut index = i32::from(e.mem.u16(this.addr() + 0x12)) - 1;
    while index >= 0 {
        let words = e.mem.u32(this.addr() + 0x18);
        if e.mem.u32(words.wrapping_add(index as u32 * 4)) == word {
            fn_00bd85b0(e, this, index as u32);
        }
        index -= 1;
    }
    e.mem.u16(this.addr() + 0x12)
}

/// Sets or clears `bit` of the word at `object + 0x24` as `on` says, clearing the word at
/// `object + 0x38` when the bit changes.
fn set_property_bit(e: &mut Engine, object: u32, bit: u32, on: bool) {
    let flags = e.mem.u32(object + 0x24);
    if (flags & bit != 0) != on {
        e.mem.set_u32(object + 0x38, 0);
    }
    let flags = e.mem.u32(object + 0x24);
    e.mem
        .set_u32(object + 0x24, if on { flags | bit } else { flags & !bit });
}

// Translated from 00bd86a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes new settings (`thiscall`, one word: the address of 0x20 bytes or null): copies
/// them over the member at +0x1c (`memcpy`) when given, then mirrors its three flag bytes
/// (+0x38, +0x39, +0x3a) into bits 0x400, 0x800 and 0x1000 of the word at +0x24 of the
/// object at +0x0c (the shader property), clearing that object's word +0x38 for each bit
/// that changes.
pub fn fn_00bd86a0(e: &mut Engine, this: Ptr, settings: u32) {
    let base = this.addr();
    if settings != 0 {
        e.call(MEMCPY, &args![base + 0x1c, settings, 0x20u32]);
    }
    for (flag, bit) in [(0x38u32, 0x400u32), (0x39, 0x800), (0x3a, 0x1000)] {
        let property = e.mem.u32(base + 0xc);
        let on = e.mem.u8(base + flag) != 0;
        set_property_bit(e, property, bit, on);
    }
}

// Translated from 00bd8750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fits the bound to the entries (`thiscall`). Starting from the float at `010bc3ec` (0
/// with no entries) as the minimum and its negation as the maximum, takes per axis the
/// smallest and largest of the entries' first three floats (a value replaces the minimum
/// unless it is greater, the maximum unless it is smaller: a NaN replaces both). With the
/// source object (+0x08) and the geometry (+0x00) present it then computes the centre
/// (minimum plus half the extent, `0.5` from `01011588`) and the radius (the square root
/// of the squared distance from the centre to the maximum corner, plus the source's float
/// +0x28); a geometry of the class `012024c0` (its `GetRTTI` chain), or otherwise the
/// result of `00653290(geometry, 012024b8)`, gets the bound (slot +0xfc, centre and radius
/// by value) and the entry count (slot +0x100). The node's (+0x04) bound object (+0xac,
/// its +0x0c) gets the centre (slot +0xb8, its address) and the half extents at +0x18..
/// +0x20, read back after the call.
///
/// x87: each instruction is one helper call, in the exe's operand order.
pub fn fn_00bd8750(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let count = e.mem.u16(base + 0x12);
    let start = if count == 0 {
        0.0
    } else {
        x87_load32(e.mem.u32(BOUND_START))
    };
    let start_bits = x87_store32(start);
    let minimum = f64::from(f32::from_bits(start_bits));
    // FCHS, FSTP, FLD.
    let maximum = x87_load32(x87_store32(-minimum));
    let mut min = [minimum; 3];
    let mut max = [maximum; 3];
    let mut entry = e.mem.u32(base + 0x14);
    for _ in 0..count {
        for axis in 0..3 {
            let value = x87_load32(x87_store32(x87_load32(e.mem.u32(entry + 4 * axis as u32))));
            if !(value > min[axis]) {
                min[axis] = value;
            }
            if !(value < max[axis]) {
                max[axis] = value;
            }
        }
        entry = entry.wrapping_add(0x10);
    }

    let source = e.mem.u32(base + 8);
    if source == 0 {
        return;
    }
    let geometry = e.mem.u32(base);
    if geometry == 0 {
        return;
    }
    // Extents, stored as floats.
    let extent: Vec<u32> = (0..3)
        .map(|axis| x87_store32(x87_sub(max[axis], min[axis])))
        .collect();
    let half = x87_load64(e.mem.u64(HALF));
    // FMUL ST(1),ST(0) (DC C9), FMUL ST(0),ST(1) (D8 C9), FMUL dword.
    let half_extent = [
        x87_store32(x87_mul(x87_load32(extent[0]), half)),
        x87_store32(x87_mul(x87_load32(extent[1]), half)),
        x87_store32(x87_mul(half, x87_m32(extent[2]))),
    ];
    let center: Vec<u32> = (0..3)
        .map(|axis| x87_store32(x87_add(min[axis], x87_load32(half_extent[axis]))))
        .collect();
    let offset: Vec<f64> = (0..3)
        .map(|axis| {
            if axis < 2 {
                x87_load32(x87_store32(x87_sub(max[axis], x87_load32(center[axis]))))
            } else {
                x87_load32(x87_store32(x87_sub(max[axis], x87_m32(center[axis]))))
            }
        })
        .collect();
    // FLD y, x, z; FLD ST1; FMULP ST2; FLD ST2; FMULP ST3; FXCH; FADDP ST2; FMUL ST0; FADDP.
    let xx = x87_mul(offset[0], offset[0]);
    let yy = x87_mul(offset[1], offset[1]);
    let sum = x87_add(yy, xx);
    let zz = x87_mul(offset[2], offset[2]);
    let squared = x87_load32(x87_store32(x87_add(sum, zz)));
    let root = e.call(SQRT, &args![squared]).f64();
    let radius = x87_load32(x87_store32(root));
    let radius = x87_store32(x87_add(radius, x87_m32(e.mem.u32(source + 0x28))));

    let mut rtti = e.vcall(geometry, 8, &args![]).u32();
    let mut of_class = false;
    while rtti != 0 {
        if rtti == GEOMETRY_RTTI_012024C0 {
            of_class = true;
            break;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    let mut target = if of_class { geometry } else { 0 };
    if target == 0 {
        let geometry = e.mem.u32(base);
        if geometry != 0 {
            target = e
                .call(LOOKUP_00653290, &args![geometry, LOOKUP_ARGUMENT_012024B8])
                .u32();
        }
    }
    if target != 0 {
        e.vcall(
            target,
            0xfc,
            &args![center[0], center[1], center[2], radius],
        );
        let count = u32::from(e.mem.u16(base + 0x12));
        e.vcall(target, 0x100, &args![count]);
    }

    let node = e.mem.u32(base + 4);
    let bound = e.mem.u32(node + 0xac);
    let object = e.mem.u32(bound + 0xc);
    e.with_stack(0xc, |e, point| {
        for axis in 0..3 {
            e.mem
                .set_u32(point.addr() + 4 * axis, center[axis as usize]);
        }
        e.vcall(object, 0xb8, &args![point]);
        for axis in 0..3u32 {
            let value = x87_store32(x87_sub(
                max[axis as usize],
                x87_m32(e.mem.u32(point.addr() + 4 * axis)),
            ));
            e.mem.set_u32(object + 0x18 + 4 * axis, value);
        }
    });
}

// Translated from 00bd8a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the entries (`thiscall`): clears the words (+0x18) and the entries (+0x14) for
/// the whole capacity (`memset`), sets every entry's third float to the float at
/// `0104eb20`, takes the count off the global count at `011f9ef8`, sets the count to 0 and
/// refits the bound ([`fn_00bd8750`], a tail call).
pub fn fn_00bd8a70(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let capacity = u32::from(e.mem.u16(base + 0x10));
    let words = e.mem.u32(base + 0x18);
    e.call(MEMSET, &args![words, 0u32, capacity * 4]);
    let capacity = u32::from(e.mem.u16(base + 0x10));
    let entries = e.mem.u32(base + 0x14);
    e.call(MEMSET, &args![entries, 0u32, capacity << 4]);
    if e.mem.u16(base + 0x10) != 0 {
        // FLD [0104eb20] once, FST into each entry: the constant comes back quiet.
        let z = x87_quiet_bits(e.mem.u32(EMPTY_ENTRY_Z));
        let mut index = 0u32;
        let mut offset = 0u32;
        loop {
            let entries = e.mem.u32(base + 0x14);
            e.mem
                .set_u32(entries.wrapping_add(offset).wrapping_add(8), z);
            index += 1;
            offset = offset.wrapping_add(0x10);
            if index as i32 >= i32::from(e.mem.u16(base + 0x10)) {
                break;
            }
        }
    }
    let count = u32::from(e.mem.u16(base + 0x12));
    let in_use = e.mem.u32(ENTRIES_IN_USE);
    e.mem.set_u32(ENTRIES_IN_USE, in_use.wrapping_sub(count));
    e.mem.set_u16(base + 0x12, 0);
    fn_00bd8750(e, this);
}

// Translated from 00bd8ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (`thiscall`, `RET 0x10`: the source object, two words, a flag byte). Clears
/// the four pointers, constructs the member at +0x1c (`00b600e0`), keeps the flag (+0x44),
/// the capacity (the source's `u16` +0x2c), the source (+0x08, counted) and the two words
/// (+0x3c, +0x40); allocates the words (`capacity * 4`) and entries (`capacity * 16`) from
/// the memory manager and empties them ([`fn_00bd8a70`]). Then it releases the geometry
/// slot and makes the geometry (0xd8 bytes: `00c1a480` when the source's byte +0x30 is 1,
/// else `00c1a660`, given the source's +0x08), attaches to it the source's property +0x18
/// (renderer level at `011f91c0` at least 2 and the byte at `011f941e` set) or +0x1c, and
/// the property +0x20, when not null; makes the shader property (0x98 bytes, `00b60cb0`),
/// gives it the source's +0x14 (counted, its +0x84) and `this` (+0x7c), sets its bits 4
/// (word +0x20) and 0x2000 (word +0x24) from the flag and the source's byte +0x31 (clearing
/// its +0x38 when one changes), attaches it, sets its +0x58 to 2 and keeps it (+0x0c). An
/// `NiAdditionalGeometryData` (0x2c bytes) for `source[+0x10] * capacity` floats gets one
/// data block (the source's +0x0c) and one stream, and is given to the geometry's data
/// (`00a67260`); `BSShaderManager::PrepareObject` prepares the geometry. Finally a
/// `BSMultiBoundNode` (0xb4 bytes, +0x04) gets a `BSMultiBound` (0x10) holding a
/// `BSMultiBoundAABB` (0x24, at its +0x0c) as its +0xac, and the geometry as a child (slot
/// +0xdc with 1). A failed allocation leaves its object null. Returns `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00bd8ae0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00bd8ae0(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    first_word: u32,
    second_word: u32,
    flag: u8,
) -> Ptr {
    let base = this.addr();
    for offset in [0u32, 4, 8, 0xc] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(MEMBER_CONSTRUCTOR_00B600E0, &args![base + 0x1c]);
    e.mem.set_u8(base + 0x44, flag);
    let capacity = e.mem.u16(source + 0x2c);
    e.mem.set_u16(base + 0x10, capacity);
    assign(e, base + 8, source, false);
    e.mem.set_u32(base + 0x3c, first_word);
    e.mem.set_u32(base + 0x40, second_word);
    let capacity = u32::from(e.mem.u16(base + 0x10));
    let size = capacity.checked_mul(4).unwrap_or(u32::MAX);
    let words = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    e.mem.set_u32(base + 0x18, words);
    let capacity = u32::from(e.mem.u16(base + 0x10));
    let size = capacity.checked_mul(0x10).unwrap_or(u32::MAX);
    let entries = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    e.mem.set_u32(base + 0x14, entries);
    e.mem.set_u16(base + 0x12, 0);
    fn_00bd8a70(e, this);
    release_and_clear(e, base);

    let data = e.mem.u32(source + 8);
    let constructor = if e.mem.u8(source + 0x30) == 1 {
        GEOMETRY_CONSTRUCTOR_00C1A480
    } else {
        GEOMETRY_CONSTRUCTOR_00C1A660
    };
    let geometry = new_object(e, 0xd8, constructor, &[data]);
    assign(e, base, geometry, true);

    let property =
        if e.mem.i32(RENDERER_LEVEL_011F91C0) >= 2 && e.mem.u8(RENDERER_FLAG_011F941E) != 0 {
            e.mem.u32(source + 0x18)
        } else {
            e.mem.u32(source + 0x1c)
        };
    if property != 0 {
        let geometry = e.mem.u32(base);
        e.call(ATTACH_PROPERTY, &args![geometry, property]);
    }
    let property = e.mem.u32(source + 0x20);
    if property != 0 {
        let geometry = e.mem.u32(base);
        e.call(ATTACH_PROPERTY, &args![geometry, property]);
    }

    let shader = new_object(e, 0x98, PROPERTY_CONSTRUCTOR_00B60CB0, &[]);
    let held = e.mem.u32(shader.wrapping_add(0x84));
    if held != e.mem.u32(source + 0x14) {
        release(e, held);
        let texture = e.mem.u32(source + 0x14);
        e.mem.set_u32(shader.wrapping_add(0x84), texture);
        if texture != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![texture + 4]);
        }
    }
    e.mem.set_u32(shader.wrapping_add(0x7c), base);
    let flags = e.mem.u32(shader.wrapping_add(0x20));
    let on = flag != 0;
    if (flags & 4 != 0) != on {
        e.mem.set_u32(shader.wrapping_add(0x38), 0);
    }
    let flags = e.mem.u32(shader.wrapping_add(0x20));
    e.mem.set_u32(
        shader.wrapping_add(0x20),
        if on { flags | 4 } else { flags & !4 },
    );
    let on = e.mem.u8(source + 0x31) != 0;
    set_property_bit(e, shader, 0x2000, on);
    let geometry = e.mem.u32(base);
    e.call(ATTACH_PROPERTY, &args![geometry, shader]);
    e.mem.set_u32(shader.wrapping_add(0x58), 2);
    assign(e, base + 0xc, shader, false);

    let floats = e
        .mem
        .u32(source + 0x10)
        .wrapping_mul(e.mem.u32(source + 0x2c));
    let additional = new_object(e, 0x2c, ADDITIONAL_DATA_CONSTRUCTOR, &[floats, 1]);
    e.call(SET_DATA_BLOCK_COUNT, &args![additional, 1u32]);
    let block = e.mem.u32(source + 0xc);
    e.call(
        SET_DATA_BLOCK,
        &args![additional, 0u32, block, floats.wrapping_mul(4), 0u32],
    );
    e.call(
        SET_DATA_STREAM,
        &args![additional, 0u32, 0u32, 0u32, 1u32, floats, 4u32, 4u32],
    );
    let geometry = e.mem.u32(base);
    let geometry_data = e.mem.u32(geometry + 0xb8);
    e.call(
        SET_ADDITIONAL_DATA_00A67260,
        &args![geometry_data, additional],
    );
    let geometry = e.mem.u32(base);
    e.call(PREPARE_OBJECT, &args![geometry, 0u32, 0u32]);

    let node = new_object(e, 0xb4, MULTI_BOUND_NODE_CONSTRUCTOR, &[]);
    assign(e, base + 4, node, true);
    let multi_bound = new_object(e, 0x10, MULTI_BOUND_CONSTRUCTOR, &[]);
    let aabb = new_object(e, 0x24, MULTI_BOUND_AABB_CONSTRUCTOR, &[]);
    assign(e, multi_bound.wrapping_add(0xc), aabb, true);
    let node = e.mem.u32(base + 4);
    assign(e, node.wrapping_add(0xac), multi_bound, true);
    let node = e.mem.u32(base + 4);
    let geometry = e.mem.u32(base);
    e.vcall(node, 0xdc, &args![geometry, 1u32]);
    this
}

// Translated from 00bd8f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor (`thiscall`): detaches the geometry from the node (the node's slot +0xe8
/// with the geometry) and the node from its parent (`m_pkParent`, +0x18: its slot +0xe8
/// with the node), releases and clears the geometry, node, shader property and source,
/// empties the entries ([`fn_00bd8a70`]), frees the entries (when not null) and the words
/// through the memory manager and clears them, then runs the four `NiPointer` member
/// destructors (shader property, source, node, geometry; all null by now).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00bd8f90 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00bd8f90(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let node = e.mem.u32(base + 4);
    let geometry = e.mem.u32(base);
    e.vcall(node, 0xe8, &args![geometry]);
    let node = e.mem.u32(base + 4);
    let parent = e.mem.u32(node + 0x18);
    if parent != 0 {
        e.vcall(parent, 0xe8, &args![node]);
    }
    for offset in [0u32, 4, 0xc, 8] {
        release_and_clear(e, base + offset);
    }
    fn_00bd8a70(e, this);
    let entries = e.mem.u32(base + 0x14);
    if entries != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, entries]);
        e.mem.set_u32(base + 0x14, 0);
    }
    let words = e.mem.u32(base + 0x18);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, words]);
    e.mem.set_u32(base + 0x18, 0);
    for offset in [0xcu32, 8, 4, 0] {
        let object = e.mem.u32(base + offset);
        release(e, object);
    }
}

// Translated from 00bd9150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the block in the object's only word through the memory manager (`thiscall`; the
/// destructor of the one-word object [`fn_00bd9120`] constructs).
pub fn fn_00bd9150(e: &mut Engine, this: Ptr) {
    let block = e.mem.u32(this.addr());
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

/// `BSCubeMapCamera`'s two render functions (`00bd77c0`, thiscall, no stack words, and
/// `00bd7bb0`, thiscall, `RET 4`; unit `bscubemapcamera.cpp` by the engine map).
const CUBE_MAP_CAMERA_00BD77C0: u32 = 0x00bd_77c0;
const CUBE_MAP_CAMERA_00BD7BB0: u32 = 0x00bd_7bb0;

// Translated from 00bd84b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCubeMapCamera::Click` (slot +0xdc of the vtable at `010bc470`, which `BSCubeMapCamera`'s
/// constructor `00bd8290` installs; the Xbox PDB's name for that slot; thiscall, `RET 4`).
/// By `eCubeMapType` (+0x114 on PC; the compiler's jump table at `00bd8540`): type 0 runs
/// `00bd77c0`, type 3 `00bd7bb0` with the word, types 1, 2 and any other nothing. Then
/// `spRenderedCubeMap` (+0x130) and the six `spRenderedTexture` pointers (+0x118) are released
/// and cleared.
pub fn bs_cube_map_camera_click(e: &mut Engine, this: Ptr, word: u32) {
    let base = this.addr();
    match e.mem.u32(base + 0x114) {
        0 => {
            e.call(CUBE_MAP_CAMERA_00BD77C0, &args![this]);
        }
        3 => {
            e.call(CUBE_MAP_CAMERA_00BD7BB0, &args![this, word]);
        }
        _ => {}
    }
    release_and_clear(e, base + 0x130);
    for index in 0..6u32 {
        release_and_clear(e, base + 0x118 + 4 * index);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bd_84b0, bs_cube_map_camera_click(Ptr, u32)),
        entry!(0x00bd_9120, fn_00bd9120(Ptr) -> Ptr),
        entry!(0x00bd_8480, fn_00bd8480(Ptr, u32) -> Ptr),
        entry!(0x00bd_8550, fn_00bd8550(Ptr, u32, u32) -> u32),
        entry!(0x00bd_85b0, fn_00bd85b0(Ptr, u32) -> u16),
        entry!(0x00bd_8670, fn_00bd8670(Ptr, u32) -> u16),
        entry!(0x00bd_86a0, fn_00bd86a0(Ptr, u32)),
        entry!(0x00bd_8750, fn_00bd8750(Ptr)),
        entry!(0x00bd_8a70, fn_00bd8a70(Ptr)),
        entry!(0x00bd_8ae0, fn_00bd8ae0(Ptr, u32, u32, u32, u8) -> Ptr),
        entry!(0x00bd_8f90, fn_00bd8f90(Ptr)),
        entry!(0x00bd_9150, fn_00bd9150(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_allocates_four_bytes_per_slot_and_saturates() {
        for (count, size) in [(5u32, 20u32), (0x4000_0001, u32::MAX)] {
            let mut e = Engine::new();
            e.map(0x011b_f000, 0x1000);
            e.mem.set_u32(SLOT_COUNT, count);
            e.register(MEMORY_ALLOCATE, |_, a| {
                0x0070_0000u32.wrapping_add(a[1]).into_ret()
            });
            let this = e.mem.alloc(4);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00bd_9120, &[this]).u32(), this);
            assert_eq!(e.mem.u32(this), 0x0070_0000u32.wrapping_add(size));
            let log = e.call_log.unwrap();
            let allocations: Vec<_> = log.iter().filter(|(a, _)| *a == MEMORY_ALLOCATE).collect();
            assert_eq!(allocations[0].1, vec![MEMORY_MANAGER, size]);
        }
    }
}

#[cfg(test)]
mod entry_tests {
    use super::*;

    const Z: f32 = -10.0;

    /// The exe constants, the global count, interlocked doubles that really count, and the
    /// memory and CRT callees.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_1000, 0x0104_e000, 0x010b_c000, 0x011f_9000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(EMPTY_ENTRY_Z, Z);
        e.mem.set_f32(BOUND_START, 5.0e9);
        e.mem.set_f64(HALF, 0.5);
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(SQRT, |_, a| f64::take(a, &mut 0).sqrt().into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e
    }

    /// An object with `capacity` entries (none in use) and no pointers, so the bound fit
    /// stops after the min/max scan.
    fn object(e: &mut Engine, capacity: u16) -> u32 {
        let this = e.mem.alloc(0x48);
        e.mem.set_u16(this + 0x10, capacity);
        let entries = e.mem.alloc(u32::from(capacity) * 16 + 16);
        let words = e.mem.alloc(u32::from(capacity) * 4 + 4);
        e.mem.set_u32(this + 0x14, entries);
        e.mem.set_u32(this + 0x18, words);
        this
    }

    fn entry_at(e: &Engine, this: u32, index: u32) -> [u32; 4] {
        let entries = e.mem.u32(this + 0x14);
        [0, 4, 8, 12].map(|i| e.mem.u32(entries + index * 16 + i))
    }

    #[test]
    fn deleting_destructor_frees_0x140_bytes_with_bit_0() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR_00BD74A0, |_, _| Ret::default());
        e.register(FREE_SIZED, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(0x00bd_8480, &[0x1000, flags]).u32(), 0x1000);
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter()
                .filter(|(a, _)| *a == DESTRUCTOR_00BD74A0)
                .count(),
            4
        );
        let frees: Vec<_> = log.iter().filter(|(a, _)| *a == FREE_SIZED).collect();
        assert_eq!(frees.len(), 2);
        assert_eq!(frees[0].1, vec![0x1000, 0x140]);
    }

    #[test]
    fn append_copies_the_entry_and_word_until_full() {
        let mut e = engine();
        let this = object(&mut e, 2);
        let source = e.mem.alloc(16);
        for (i, v) in [1u32, 2, 3, 4].iter().enumerate() {
            e.mem.set_u32(source + 4 * i as u32, *v);
        }
        assert_eq!(e.call(0x00bd_8550, &[this, source, 0x77]).u32(), 1);
        assert_eq!(e.call(0x00bd_8550, &[this, source, 0x88]).u32(), 1);
        assert_eq!(entry_at(&e, this, 1), [1, 2, 3, 4]);
        let words = e.mem.u32(this + 0x18);
        assert_eq!([e.mem.u32(words), e.mem.u32(words + 4)], [0x77, 0x88]);
        assert_eq!(e.mem.u16(this + 0x12), 2);
        // Full: AL 0, the count's high byte in AH.
        e.mem.set_u16(this + 0x10, 0x0102);
        e.mem.set_u16(this + 0x12, 0x0102);
        assert_eq!(e.call(0x00bd_8550, &[this, source, 0x99]).u32(), 0x0100);
        // A null entry is refused too.
        e.mem.set_u16(this + 0x12, 0);
        assert_eq!(e.call(0x00bd_8550, &[this, 0, 0x99]).u32(), 0);
    }

    #[test]
    fn remove_moves_the_last_entry_down_and_resets_it() {
        let mut e = engine();
        let this = object(&mut e, 4);
        let entries = e.mem.u32(this + 0x14);
        let words = e.mem.u32(this + 0x18);
        for i in 0..3u32 {
            for j in 0..4u32 {
                e.mem.set_u32(entries + i * 16 + j * 4, 0x100 * (i + 1) + j);
            }
            e.mem.set_u32(words + i * 4, 0x10 + i);
        }
        e.mem.set_u16(this + 0x12, 3);
        e.mem.set_u32(ENTRIES_IN_USE, 9);
        // The index's high word is ignored.
        assert_eq!(e.call(0x00bd_85b0, &[this, 0x0001_0000]).u32(), 2);
        assert_eq!(entry_at(&e, this, 0), [0x300, 0x301, 0x302, 0x303]);
        assert_eq!(entry_at(&e, this, 2), [0, 0, Z.to_bits(), 0]);
        assert_eq!(e.mem.u32(words), 0x12);
        assert_eq!(e.mem.u32(words + 8), 0);
        assert_eq!(e.mem.u32(ENTRIES_IN_USE), 8);
        // The last one is only reset.
        assert_eq!(e.call(0x00bd_85b0, &[this, 1]).u32(), 1);
        assert_eq!(entry_at(&e, this, 1), [0, 0, Z.to_bits(), 0]);
        assert_eq!(e.mem.u32(words), 0x12);
        // Out of range: nothing.
        assert_eq!(e.call(0x00bd_85b0, &[this, 1]).u32(), 1);
        assert_eq!(e.mem.u32(ENTRIES_IN_USE), 7);
    }

    #[test]
    fn remove_word_removes_every_match_from_the_top() {
        let mut e = engine();
        let this = object(&mut e, 4);
        let words = e.mem.u32(this + 0x18);
        for (i, w) in [5u32, 6, 5, 5].iter().enumerate() {
            e.mem.set_u32(words + 4 * i as u32, *w);
        }
        e.mem.set_u16(this + 0x12, 4);
        e.mem.set_u32(ENTRIES_IN_USE, 4);
        assert_eq!(e.call(0x00bd_8670, &[this, 5]).u32(), 1);
        assert_eq!(e.mem.u32(words), 6);
        assert_eq!(e.mem.u32(ENTRIES_IN_USE), 1);
        // Empty: nothing.
        let this = object(&mut e, 4);
        assert_eq!(e.call(0x00bd_8670, &[this, 5]).u32(), 0);
    }

    #[test]
    fn settings_copy_and_mirror_three_bits() {
        let mut e = engine();
        let this = object(&mut e, 1);
        let property = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, property);
        e.mem.set_u32(property + 0x24, 0x800 | 0x1000);
        e.mem.set_u32(property + 0x38, 7);
        let settings = e.mem.alloc(0x20);
        e.mem.set_u8(settings + 0x1c, 1);
        e.mem.set_u8(settings + 0x1d, 1);
        e.mem.set_u8(settings + 0x1e, 0);
        e.mem.set_u32(settings, 0xabcd);
        e.call(0x00bd_86a0, &[this, settings]);
        assert_eq!(e.mem.u32(this + 0x1c), 0xabcd);
        assert_eq!(e.mem.u32(property + 0x24), 0x400 | 0x800);
        assert_eq!(e.mem.u32(property + 0x38), 0);
        // Nothing changes: +0x38 kept, no copy.
        e.mem.set_u32(property + 0x38, 9);
        e.call_log = Some(vec![]);
        e.call(0x00bd_86a0, &[this, 0]);
        assert_eq!(e.mem.u32(property + 0x38), 9);
        assert!(!e.call_log.take().unwrap().iter().any(|(a, _)| *a == MEMCPY));
    }

    /// Geometry, node chain and source for the bound fit, with recorded vcalls.
    struct Bound {
        e: Engine,
        this: u32,
        object: u32,
        target: u32,
    }

    const BOUND_SLOT: u32 = 0x0000_60fc;
    const COUNT_SLOT: u32 = 0x0000_6100;
    const RTTI_SLOT: u32 = 0x0000_6008;
    const CENTER_SLOT: u32 = 0x0000_60b8;

    fn bound(rtti_chain: &[u32]) -> Bound {
        let mut e = engine();
        let this = object(&mut e, 4);
        let entries = e.mem.u32(this + 0x14);
        let points = [[1.0f32, -2.0, 3.0], [5.0, 2.0, -1.0]];
        for (i, p) in points.iter().enumerate() {
            for (j, v) in p.iter().enumerate() {
                e.mem.set_f32(entries + 16 * i as u32 + 4 * j as u32, *v);
            }
        }
        e.mem.set_u16(this + 0x12, 2);
        let source = e.mem.alloc(0x40);
        e.mem.set_f32(source + 0x28, 0.5);
        e.mem.set_u32(this + 8, source);
        let vtable = e.mem.alloc(0x104);
        e.mem.set_u32(vtable + 8, RTTI_SLOT);
        e.mem.set_u32(vtable + 0xfc, BOUND_SLOT);
        e.mem.set_u32(vtable + 0x100, COUNT_SLOT);
        e.mem.set_u32(vtable + 0xb8, CENTER_SLOT);
        let geometry = e.mem.alloc(8);
        e.mem.set_u32(geometry, vtable);
        e.mem.set_u32(this, geometry);
        // The RTTI chain: each record's +4 is the next.
        let mut next = 0u32;
        for &name in rtti_chain.iter().rev() {
            let record = if name == GEOMETRY_RTTI_012024C0 {
                e.map(0x0120_2000, 0x1000);
                name
            } else {
                e.mem.alloc(8)
            };
            e.mem.set_u32(record + 4, next);
            next = record;
        }
        let first = next;
        e.register_double(RTTI_SLOT, move |_, _| first.into_ret());
        e.register(BOUND_SLOT, |_, _| Ret::default());
        e.register(COUNT_SLOT, |_, _| Ret::default());
        e.register(CENTER_SLOT, |e, a| {
            // The callee may change the point; the code reads it back.
            let x = e.mem.f32(a[1]);
            e.mem.set_f32(a[1], x + 1.0);
            Ret::default()
        });
        let node = e.mem.alloc(0xb0);
        let multi_bound = e.mem.alloc(0x10);
        let object = e.mem.alloc(0x24);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(multi_bound + 0xc, object);
        e.mem.set_u32(node + 0xac, multi_bound);
        e.mem.set_u32(this + 4, node);
        e.call_log = Some(vec![]);
        Bound {
            e,
            this,
            object,
            target: geometry,
        }
    }

    #[test]
    fn bound_fit_gives_centre_radius_and_half_extents() {
        let mut b = bound(&[0x1, GEOMETRY_RTTI_012024C0]);
        b.e.call(0x00bd_8750, &[b.this]);
        let log = b.e.call_log.take().unwrap();
        // Minimum (1, -2, -1), maximum (5, 2, 3): centre (3, 0, 1), corner offset (2, 2, 2).
        let center = [3.0f32, 0.0, 1.0].map(f32::to_bits);
        let radius = (12.0f64.sqrt() as f32 + 0.5).to_bits();
        let bound_call = log.iter().find(|(a, _)| *a == BOUND_SLOT).unwrap();
        assert_eq!(
            bound_call.1,
            vec![b.target, center[0], center[1], center[2], radius]
        );
        let count_call = log.iter().find(|(a, _)| *a == COUNT_SLOT).unwrap();
        assert_eq!(count_call.1, vec![b.target, 2]);
        let center_call = log.iter().find(|(a, _)| *a == CENTER_SLOT).unwrap();
        assert_eq!(center_call.1[0], b.object);
        // Half extents against the centre read back after the call (x became 4).
        assert_eq!(b.e.mem.f32(b.object + 0x18), 1.0);
        assert_eq!(b.e.mem.f32(b.object + 0x1c), 2.0);
        assert_eq!(b.e.mem.f32(b.object + 0x20), 2.0);
        assert!(!log.iter().any(|(a, _)| *a == LOOKUP_00653290));
    }

    #[test]
    fn bound_fit_falls_back_to_the_lookup() {
        // Not of the class; the lookup answers null: no bound call, the centre still goes.
        let mut b = bound(&[0x1]);
        b.e.register(LOOKUP_00653290, |_, _| 0u32.into_ret());
        b.e.call(0x00bd_8750, &[b.this]);
        let log = b.e.call_log.take().unwrap();
        let lookup = log.iter().find(|(a, _)| *a == LOOKUP_00653290).unwrap();
        assert_eq!(lookup.1, vec![b.target, LOOKUP_ARGUMENT_012024B8]);
        assert!(!log.iter().any(|(a, _)| *a == BOUND_SLOT));
        assert!(log.iter().any(|(a, _)| *a == CENTER_SLOT));
    }

    #[test]
    fn bound_fit_stops_without_source_and_scans_nan() {
        let mut e = engine();
        let this = object(&mut e, 2);
        let entries = e.mem.u32(this + 0x14);
        e.mem.set_u32(entries, 0x7fc0_0000);
        e.mem.set_u16(this + 0x12, 1);
        e.call_log = Some(vec![]);
        e.call(0x00bd_8750, &[this]);
        // Only the top-level call: nothing else is reached.
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn empty_clears_everything_and_refits() {
        let mut e = engine();
        let this = object(&mut e, 3);
        let entries = e.mem.u32(this + 0x14);
        e.mem.set_u32(entries + 4, 0x55);
        let words = e.mem.u32(this + 0x18);
        e.mem.set_u32(words + 8, 0x66);
        e.mem.set_u16(this + 0x12, 2);
        e.mem.set_u32(ENTRIES_IN_USE, 5);
        e.call_log = Some(vec![]);
        e.call(0x00bd_8a70, &[this]);
        for i in 0..3 {
            assert_eq!(entry_at(&e, this, i), [0, 0, Z.to_bits(), 0]);
        }
        assert_eq!(e.mem.u32(words + 8), 0);
        assert_eq!(e.mem.u16(this + 0x12), 0);
        assert_eq!(e.mem.u32(ENTRIES_IN_USE), 3);
        let log = e.call_log.take().unwrap();
        let sets: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == MEMSET)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(sets, vec![vec![words, 0, 12], vec![entries, 0, 48]]);
    }

    /// Constructor callees: every allocation hands out a fresh zeroed block, every
    /// constructor returns its block.
    fn constructor_engine() -> Engine {
        let mut e = engine();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1].max(4)).into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        for address in [
            GEOMETRY_CONSTRUCTOR_00C1A480,
            GEOMETRY_CONSTRUCTOR_00C1A660,
            PROPERTY_CONSTRUCTOR_00B60CB0,
            ADDITIONAL_DATA_CONSTRUCTOR,
            MULTI_BOUND_NODE_CONSTRUCTOR,
            MULTI_BOUND_CONSTRUCTOR,
            MULTI_BOUND_AABB_CONSTRUCTOR,
        ] {
            e.register(address, |_, a| a[0].into_ret());
        }
        for address in [
            MEMBER_CONSTRUCTOR_00B600E0,
            ATTACH_PROPERTY,
            SET_DATA_BLOCK_COUNT,
            SET_DATA_BLOCK,
            SET_DATA_STREAM,
            SET_ADDITIONAL_DATA_00A67260,
            PREPARE_OBJECT,
            0x0000_70dc,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn constructor_builds_geometry_property_data_and_node() {
        for (byte30, level, flag) in [(1u8, 2i32, 1u8), (0, 1, 0)] {
            let mut e = constructor_engine();
            e.mem.set_i32(RENDERER_LEVEL_011F91C0, level);
            e.mem.set_u8(RENDERER_FLAG_011F941E, 1);
            let source = e.mem.alloc(0x34);
            e.mem.set_u16(source + 0x2c, 3);
            e.mem.set_u32(source + 8, 0x808);
            e.mem.set_u32(source + 0xc, 0xc0c);
            e.mem.set_u32(source + 0x10, 2);
            e.mem.set_u32(source + 0x18, 0x1818);
            e.mem.set_u32(source + 0x1c, 0x1c1c);
            e.mem.set_u8(source + 0x30, byte30);
            e.mem.set_u8(source + 0x31, 1);
            let texture = e.mem.alloc(8);
            e.mem.set_u32(source + 0x14, texture);
            let this = e.mem.alloc(0x48);
            e.call_log = Some(vec![]);
            // The allocator gives the 0xb4-byte node a vtable whose slot +0xdc is doubled.
            e.register(ALLOCATE, |e, a| {
                let block = e.mem.alloc(a[0]);
                if a[0] == 0xb4 {
                    let vtable = e.mem.alloc(0xe0);
                    e.mem.set_u32(vtable + 0xdc, 0x0000_70dc);
                    e.mem.set_u32(block, vtable);
                }
                block.into_ret()
            });
            assert_eq!(
                e.call(
                    0x00bd_8ae0,
                    &[this, source, 0x3c3c, 0x4040, u32::from(flag)]
                )
                .u32(),
                this
            );
            let log = e.call_log.take().unwrap();
            let geometry = e.mem.u32(this);
            let node = e.mem.u32(this + 4);
            let shader = e.mem.u32(this + 0xc);
            assert_eq!(e.mem.u32(this + 8), source);
            assert_eq!(e.mem.u32(source + 4), 1);
            assert_eq!(e.mem.u16(this + 0x10), 3);
            assert_eq!(e.mem.u32(this + 0x3c), 0x3c3c);
            assert_eq!(e.mem.u32(this + 0x40), 0x4040);
            assert_eq!(e.mem.u8(this + 0x44), flag);
            let made = if byte30 == 1 {
                GEOMETRY_CONSTRUCTOR_00C1A480
            } else {
                GEOMETRY_CONSTRUCTOR_00C1A660
            };
            assert_eq!(
                log.iter().find(|(a, _)| *a == made).unwrap().1,
                vec![geometry, 0x808]
            );
            assert_eq!(e.mem.u32(geometry + 4), 1);
            let attached: Vec<u32> = log
                .iter()
                .filter(|(a, _)| *a == ATTACH_PROPERTY)
                .map(|(_, w)| w[1])
                .collect();
            let first = if level >= 2 { 0x1818 } else { 0x1c1c };
            assert_eq!(attached, vec![first, shader]);
            assert_eq!(e.mem.u32(shader + 0x84), texture);
            assert_eq!(e.mem.u32(texture + 4), 1);
            assert_eq!(e.mem.u32(shader + 0x7c), this);
            assert_eq!(e.mem.u32(shader + 0x20) & 4, if flag != 0 { 4 } else { 0 });
            assert_eq!(e.mem.u32(shader + 0x24) & 0x2000, 0x2000);
            assert_eq!(e.mem.u32(shader + 0x58), 2);
            assert_eq!(e.mem.u32(shader + 4), 1);
            let data = log
                .iter()
                .find(|(a, _)| *a == ADDITIONAL_DATA_CONSTRUCTOR)
                .unwrap();
            assert_eq!(data.1[1..], [6, 1]);
            let block = log.iter().find(|(a, _)| *a == SET_DATA_BLOCK).unwrap();
            assert_eq!(block.1[1..], [0, 0xc0c, 24, 0]);
            let stream = log.iter().find(|(a, _)| *a == SET_DATA_STREAM).unwrap();
            assert_eq!(stream.1[1..], [0, 0, 0, 1, 6, 4, 4]);
            let prepare = log.iter().find(|(a, _)| *a == PREPARE_OBJECT).unwrap();
            assert_eq!(prepare.1, vec![geometry, 0, 0]);
            let multi_bound = e.mem.u32(node + 0xac);
            assert_ne!(multi_bound, 0);
            assert_ne!(e.mem.u32(multi_bound + 0xc), 0);
            let child = log.iter().find(|(a, _)| *a == 0x0000_70dc).unwrap();
            assert_eq!(child.1, vec![node, geometry, 1]);
            // Entries emptied: three, the third float -10.
            assert_eq!(entry_at(&e, this, 2), [0, 0, Z.to_bits(), 0]);
        }
    }

    #[test]
    fn destructor_detaches_releases_and_frees() {
        let mut e = engine();
        let this = object(&mut e, 1);
        let entries = e.mem.u32(this + 0x14);
        let words = e.mem.u32(this + 0x18);
        let detach = 0x0000_70e8u32;
        let delete = 0x0000_7004u32;
        e.register(detach, |_, _| Ret::default());
        e.register(delete, |_, _| Ret::default());
        let vtable = e.mem.alloc(0xec);
        e.mem.set_u32(vtable + 0xe8, detach);
        e.mem.set_u32(vtable + 4, delete);
        let mut objects = vec![];
        for offset in [0u32, 4, 8, 0xc] {
            let object = e.mem.alloc(0x20);
            e.mem.set_u32(object, vtable);
            e.mem.set_u32(object + 4, 1);
            e.mem.set_u32(this + offset, object);
            objects.push(object);
        }
        let parent = e.mem.alloc(8);
        e.mem.set_u32(parent, vtable);
        e.mem.set_u32(objects[1] + 0x18, parent);
        e.call_log = Some(vec![]);
        e.call(0x00bd_8f90, &[this]);
        let log = e.call_log.take().unwrap();
        let detaches: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == detach)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(
            detaches,
            vec![vec![objects[1], objects[0]], vec![parent, objects[1]]]
        );
        let deletes: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == delete)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(
            deletes,
            vec![objects[0], objects[1], objects[3], objects[2]]
        );
        let frees: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == MEMORY_DEALLOCATE)
            .map(|(_, w)| w[1])
            .collect();
        assert_eq!(frees, vec![entries, words]);
        for offset in [0u32, 4, 8, 0xc, 0x14, 0x18] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }

    #[test]
    fn one_word_destructor_frees_its_block() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x00bd_9150, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MEMORY_DEALLOCATE, vec![MEMORY_MANAGER, 0x1234]));
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    #[test]
    fn click_renders_by_type_and_releases_the_textures() {
        const DELETE: u32 = 0x03f3_0000;
        let mut e = Engine::new();
        for address in [CUBE_MAP_CAMERA_00BD77C0, CUBE_MAP_CAMERA_00BD7BB0, DELETE] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE);
        let this = e.mem.alloc(0x170);
        let mut textures = vec![];
        for offset in [0x130u32, 0x118, 0x120, 0x12c] {
            let texture = e.mem.alloc(8);
            e.mem.set_u32(texture, vtable);
            // The first texture is held elsewhere too and survives.
            e.mem
                .set_u32(texture + 4, if offset == 0x130 { 2 } else { 1 });
            e.mem.set_u32(this + offset, texture);
            textures.push(texture);
        }
        for (kind, zero_calls, three_calls) in
            [(0u32, 1usize, 0usize), (3, 0, 1), (1, 0, 0), (7, 0, 0)]
        {
            e.mem.set_u32(this + 0x114, kind);
            e.call_log = Some(vec![]);
            e.call(0x00bd_84b0, &args![this, 0x55u32]);
            let log = e.call_log.take().unwrap();
            let count = |addr: u32| log.iter().filter(|c| c.0 == addr).count();
            assert_eq!(count(CUBE_MAP_CAMERA_00BD77C0), zero_calls);
            assert_eq!(count(CUBE_MAP_CAMERA_00BD7BB0), three_calls);
            if kind == 3 {
                let call = log
                    .iter()
                    .find(|c| c.0 == CUBE_MAP_CAMERA_00BD7BB0)
                    .unwrap();
                assert_eq!(call.1, vec![this, 0x55]);
            }
            if kind == 0 {
                // The first run released all four: three deleted, the shared one kept.
                let deleted: Vec<u32> = log
                    .iter()
                    .filter(|c| c.0 == DELETE)
                    .map(|c| c.1[0])
                    .collect();
                assert_eq!(deleted, textures[1..].to_vec());
                assert_eq!(e.mem.u32(textures[0] + 4), 1);
            }
            for offset in [0x130u32, 0x118, 0x11c, 0x120, 0x124, 0x128, 0x12c] {
                assert_eq!(e.mem.u32(this + offset), 0);
            }
        }
    }
}
