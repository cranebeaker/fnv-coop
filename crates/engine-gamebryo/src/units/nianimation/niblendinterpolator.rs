//! `NiAnimation/niblendinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBlendInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_BLEND_INTERPOLATOR_RTTI: u32 = 0x011f_36fc;

// Translated from 00a36960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_blend_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BLEND_INTERPOLATOR_RTTI)
}

// Translated from 00a37670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a37170`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x30 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_blend_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a3_7170;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x30u32]);
    }
    this
}

use crate::units::nimain::nimainsdm::{assign_ref_counted, interlocked_decrement};

/// `NiBlendInterpolator`'s vtable (written by its constructor and
/// destructor).
const NI_BLEND_INTERPOLATOR_VTABLE: u32 = 0x0109_697c;
/// `NiInterpolator::NiInterpolator` (`00a56ff0`, `thiscall`) and the
/// `NiInterpolator` destructor body (`00a57010`, reached by a tail jump).
const NI_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a5_6ff0;
const NI_INTERPOLATOR_DESTRUCTOR: u32 = 0x00a5_7010;
/// `NiInterpolator::LoadBinary` (`00a88a40`, `RET 4`).
const NI_INTERPOLATOR_LOAD_BINARY: u32 = 0x00a8_8a40;
/// `NiObject::ProcessClone` (`00a5d430`, `RET 4`).
const NI_OBJECT_PROCESS_CLONE: u32 = 0x00a5_d430;
/// `NiKeyBasedInterpolator::CopyMembers` (`00aa1000`, `RET 8`).
const KEY_BASED_COPY_MEMBERS: u32 = 0x00aa_1000;
/// The clone map lookup (`00a53c30`, `thiscall (map, key, &value)`,
/// answers whether the key was found).
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiBlendInterpolator::InterpArrayItem::LoadBinary` (`00a377b0`,
/// `thiscall (item, stream)`).
const ITEM_LOAD_BINARY: u32 = 0x00a3_77b0;
/// `NiStream::ResolveLinkID` (`00a64620`, `thiscall`).
const STREAM_RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// The memory manager's allocate and free (`00aa3e40`, `00aa4060`,
/// `thiscall` on the manager at `011f6238`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// The compiler's vector constructor iterator (`00401050`, `stdcall
/// (array, element size, count, constructor)`).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// `NiInterpolator::INVALID_FLOAT` (Xbox PDB static; `-FLT_MAX`).
const INVALID_FLOAT: u32 = 0x0109_696c;
/// `NiBlendInterpolator::ms_ucArrayGrowBy` (Xbox PDB static, a byte).
const ARRAY_GROW_BY: u32 = 0x011a_83ff;
/// `-FLT_MAX` (`double`, `010241b0`), the unset sum of weights.
const LOWEST: u32 = 0x0102_41b0;
/// `-1.0` (`float`, `01012054`).
const MINUS_ONE: u32 = 0x0101_2054;
/// The `float` `1.0`, as the exe's `FLD1` / `FSTP` stores it.
const ONE_BITS: u32 = 0x3f80_0000;

/// `m_uFlags` (Xbox PDB) +0xC and its bits: 1 manager controlled, 2 only
/// the highest weight, 4 weights out of date.
const FLAGS: u32 = 0xc;
const MANAGER_CONTROLLED: u8 = 1;
const ONLY_USE_HIGHEST_WEIGHT: u8 = 2;
const COMPUTE_NORMALIZED_WEIGHTS: u8 = 4;
/// `m_ucArraySize`, `m_ucInterpCount`, `m_ucSingleIdx` (Xbox PDB).
const ARRAY_SIZE: u32 = 0xd;
const INTERP_COUNT: u32 = 0xe;
const SINGLE_INDEX: u32 = 0xf;
/// `m_cHighPriority`, `m_cNextHighPriority` (Xbox PDB).
const HIGH_PRIORITY: u32 = 0x10;
const NEXT_HIGH_PRIORITY: u32 = 0x11;
/// `m_pkInterpArray`, `m_pkSingleInterpolator` (Xbox PDB).
const INTERP_ARRAY: u32 = 0x14;
const SINGLE_INTERPOLATOR: u32 = 0x18;
/// `m_fWeightThreshold`, `m_fSingleTime`, `m_fHighSumOfWeights`,
/// `m_fNextHighSumOfWeights`, `m_fHighEaseSpinner` (Xbox PDB).
const WEIGHT_THRESHOLD: u32 = 0x1c;
const SINGLE_TIME: u32 = 0x20;
const HIGH_SUM_OF_WEIGHTS: u32 = 0x24;
const NEXT_HIGH_SUM_OF_WEIGHTS: u32 = 0x28;
const HIGH_EASE_SPINNER: u32 = 0x2c;

/// `NiBlendInterpolator::InterpArrayItem` (Xbox PDB), 0x18 bytes:
/// `m_spInterpolator` +0, `m_fWeight` +4, `m_fNormalizedWeight` +8,
/// `m_cPriority` +0xC, `m_fEaseSpinner` +0x10, `m_fUpdateTime` +0x14.
const ITEM_SIZE: u32 = 0x18;
const ITEM_WEIGHT: u32 = 0x4;
const ITEM_NORMALIZED_WEIGHT: u32 = 0x8;
const ITEM_PRIORITY: u32 = 0xc;
const ITEM_EASE_SPINNER: u32 = 0x10;
const ITEM_UPDATE_TIME: u32 = 0x14;

/// The address of item `index` of the interpolator's array.
fn item(e: &Engine, this: Ptr, index: u32) -> u32 {
    e.mem
        .u32(this.addr().wrapping_add(INTERP_ARRAY))
        .wrapping_add(index.wrapping_mul(ITEM_SIZE))
}

fn byte(e: &Engine, this: Ptr, offset: u32) -> u8 {
    e.mem.u8(this.addr().wrapping_add(offset))
}

fn set_byte(e: &mut Engine, this: Ptr, offset: u32, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(offset), value);
}

/// `FLD dword` / `FSTP dword` of the float at `from` into `to`.
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

/// `FLD INVALID_FLOAT` / `FSTP` into `to`.
fn store_invalid(e: &mut Engine, to: u32) {
    copy_float(e, INVALID_FLOAT, to);
}

/// The compiler's `new InterpArrayItem[count]`: `count * 0x18 + 4` bytes
/// (all ones on overflow) from the memory manager, the count in the first
/// word and every item constructed (`00a370a0`) by the vector constructor
/// iterator. The array (after the count), or 0 when the allocation failed.
fn new_item_array(e: &mut Engine, count: u32) -> u32 {
    let size = count
        .checked_mul(ITEM_SIZE)
        .unwrap_or(u32::MAX)
        .checked_add(4)
        .unwrap_or(u32::MAX);
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32();
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    let array = block.wrapping_add(4);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![array, ITEM_SIZE, count, 0x00a3_70a0u32],
    );
    array
}

/// The compiler's `delete[]` of an item array (non-null): each item's
/// interpolator released, last first (`InterlockedDecrement` of its count
/// at +4; vtable slot +4 when it reaches 0), for as many items as the count
/// word says while the count minus one is not negative; then the block
/// freed through the memory manager.
fn delete_item_array(e: &mut Engine, array: u32) {
    let block = array.wrapping_sub(4);
    let count = e.mem.u32(block);
    let mut remaining = count.wrapping_sub(1) as i32;
    let mut at = array.wrapping_add(count.wrapping_mul(ITEM_SIZE));
    while remaining >= 0 {
        at = at.wrapping_sub(ITEM_SIZE);
        let interpolator = e.mem.u32(at);
        if interpolator != 0 && interlocked_decrement(e, interpolator.wrapping_add(4)) == 0 {
            e.vcall(interpolator, 4, &args![]);
        }
        remaining -= 1;
    }
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, block]);
}

// Translated from 00a370a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::InterpArrayItem`'s constructor (no Ghidra
/// function; the vector constructor iterator's callback): clears
/// `m_spInterpolator` and answers `this`.
pub fn fn_00a370a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    this
}

// Translated from 00a371f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `InterpArrayItem`'s assignment from `other`:
/// the reference-counted interpolator (released and counted as a smart
/// pointer), then the weight, normalized weight, priority, ease spinner and
/// update time. Answers `this`.
pub fn fn_00a371f0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let interpolator = e.mem.u32(other.addr());
    assign_ref_counted(e, this.addr(), interpolator);
    let (to, from) = (this.addr(), other.addr());
    copy_float(e, from + ITEM_WEIGHT, to + ITEM_WEIGHT);
    copy_float(
        e,
        from + ITEM_NORMALIZED_WEIGHT,
        to + ITEM_NORMALIZED_WEIGHT,
    );
    let priority = e.mem.u8(from + ITEM_PRIORITY);
    e.mem.set_u8(to + ITEM_PRIORITY, priority);
    copy_float(e, from + ITEM_EASE_SPINNER, to + ITEM_EASE_SPINNER);
    copy_float(e, from + ITEM_UPDATE_TIME, to + ITEM_UPDATE_TIME);
    this
}

// Translated from 00a370b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::NiBlendInterpolator` (Xbox PDB): the
/// `NiInterpolator` constructor, then the weight threshold, the four
/// cached sums and times `INVALID_FLOAT`, both priorities `0x80`, the
/// class's vtable, the array size, no interpolators (count 0, single index
/// `0xFF`, no array, no single interpolator), the manager-controlled flag
/// (`flags = manager_controlled != 0`), and with a non-zero size a new item
/// array of that size. Answers `this`.
pub fn ni_blend_interpolator_ni_blend_interpolator(
    e: &mut Engine,
    this: Ptr,
    manager_controlled: u8,
    weight_threshold: f32,
    array_size: u8,
) -> Ptr {
    e.call(NI_INTERPOLATOR_CONSTRUCTOR, &args![this]);
    let at = |offset: u32| this.addr().wrapping_add(offset);
    e.mem.set_u32(
        at(WEIGHT_THRESHOLD),
        x87_quiet_bits(weight_threshold.to_bits()),
    );
    store_invalid(e, at(SINGLE_TIME));
    store_invalid(e, at(HIGH_SUM_OF_WEIGHTS));
    set_byte(e, this, HIGH_PRIORITY, 0x80);
    set_byte(e, this, NEXT_HIGH_PRIORITY, 0x80);
    store_invalid(e, at(NEXT_HIGH_SUM_OF_WEIGHTS));
    e.mem.set_u32(this.addr(), NI_BLEND_INTERPOLATOR_VTABLE);
    store_invalid(e, at(HIGH_EASE_SPINNER));
    set_byte(e, this, ARRAY_SIZE, array_size);
    set_byte(e, this, INTERP_COUNT, 0);
    set_byte(e, this, SINGLE_INDEX, 0xff);
    e.mem.set_u32(at(INTERP_ARRAY), 0);
    e.mem.set_u32(at(SINGLE_INTERPOLATOR), 0);
    set_byte(e, this, FLAGS, u8::from(manager_controlled != 0));
    if array_size > 0 {
        let array = new_item_array(e, u32::from(array_size));
        e.mem.set_u32(at(INTERP_ARRAY), array);
    }
    this
}

// Translated from 00a37170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::~NiBlendInterpolator` (Xbox PDB): puts the class's
/// vtable back, deletes the item array when there is one (releasing each
/// item's interpolator), then runs the `NiInterpolator` destructor.
pub fn ni_blend_interpolator_destructor(e: &mut Engine, this: Ptr) {
    let array = e.mem.u32(this.addr().wrapping_add(INTERP_ARRAY));
    e.mem.set_u32(this.addr(), NI_BLEND_INTERPOLATOR_VTABLE);
    if array != 0 {
        delete_item_array(e, array);
    }
    e.call(NI_INTERPOLATOR_DESTRUCTOR, &args![this]);
}

// Translated from 00a368f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::GetHighestWeightedIndex` (Xbox PDB): with exactly
/// one interpolator its single index; otherwise the index of the first item
/// with the largest normalized weight above 0 (`0xFF` when none is above 0).
pub fn ni_blend_interpolator_get_highest_weighted_index(e: &mut Engine, this: Ptr) -> u8 {
    if byte(e, this, INTERP_COUNT) == 1 {
        return byte(e, this, SINGLE_INDEX);
    }
    let mut highest = 0.0f64;
    let mut index = 0xffu8;
    let size = byte(e, this, ARRAY_SIZE);
    let mut i = 0u8;
    while i < size {
        let weight = x87_load32(
            e.mem
                .u32(item(e, this, u32::from(i)) + ITEM_NORMALIZED_WEIGHT),
        );
        if highest < weight {
            highest = weight;
            index = i;
        }
        i = i.wrapping_add(1);
    }
    index
}

/// The priorities' update for an item of priority `priority` being added:
/// above the next-highest it becomes the highest (the old highest moving to
/// next-highest) or, below the highest, the next-highest.
fn raise_priorities(e: &mut Engine, this: Ptr, priority: i8) {
    if priority <= byte(e, this, NEXT_HIGH_PRIORITY) as i8 {
        return;
    }
    let high = byte(e, this, HIGH_PRIORITY) as i8;
    if priority > high {
        set_byte(e, this, NEXT_HIGH_PRIORITY, high as u8);
        set_byte(e, this, HIGH_PRIORITY, priority as u8);
    } else if priority != high {
        set_byte(e, this, NEXT_HIGH_PRIORITY, priority as u8);
    }
}

/// Marks the weights out of date and the three cached sums unset (the end
/// of `AddInterpInfo` and `RemoveInterpInfo`).
fn invalidate_weights(e: &mut Engine, this: Ptr) {
    let flags = byte(e, this, FLAGS) | COMPUTE_NORMALIZED_WEIGHTS;
    set_byte(e, this, FLAGS, flags);
    let at = |offset: u32| this.addr().wrapping_add(offset);
    store_invalid(e, at(HIGH_SUM_OF_WEIGHTS));
    store_invalid(e, at(NEXT_HIGH_SUM_OF_WEIGHTS));
    store_invalid(e, at(HIGH_EASE_SPINNER));
}

// Translated from 00a36970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::AddInterpInfo` (Xbox PDB): puts `interpolator`
/// (reference counted) with `weight`, `priority` and `ease_spinner` into
/// the first free item (normalized weight 0, update time `INVALID_FLOAT`),
/// growing the array through `ReallocateArrays` (vtable +0xE8) when all are
/// used (`0xFF` when that fails); updates the priorities; with it the only
/// interpolator it becomes the single one, otherwise there is none (single
/// index `0xFF`, single time `INVALID_FLOAT`); marks the weights out of
/// date. Answers the item's index.
pub fn ni_blend_interpolator_add_interp_info(
    e: &mut Engine,
    this: Ptr,
    interpolator: Ptr,
    weight: f32,
    priority: i8,
    ease_spinner: f32,
) -> u8 {
    let size = byte(e, this, ARRAY_SIZE);
    let mut index = 0u8;
    while index < size {
        if e.mem.u32(item(e, this, u32::from(index))) == 0 {
            break;
        }
        index = index.wrapping_add(1);
    }
    if index == size && !e.vcall(this.addr(), 0xe8, &args![]).bool() {
        return 0xff;
    }
    let slot = item(e, this, u32::from(index));
    assign_ref_counted(e, slot, interpolator.addr());
    e.mem
        .set_u32(slot + ITEM_WEIGHT, x87_quiet_bits(weight.to_bits()));
    e.mem.set_u8(slot + ITEM_PRIORITY, priority as u8);
    e.mem.set_u32(slot + ITEM_NORMALIZED_WEIGHT, 0);
    e.mem.set_u32(
        slot + ITEM_EASE_SPINNER,
        x87_quiet_bits(ease_spinner.to_bits()),
    );
    store_invalid(e, slot + ITEM_UPDATE_TIME);
    raise_priorities(e, this, priority);
    let count = byte(e, this, INTERP_COUNT).wrapping_add(1);
    set_byte(e, this, INTERP_COUNT, count);
    if count == 1 {
        set_byte(e, this, SINGLE_INDEX, index);
        e.mem
            .set_u32(this.addr() + SINGLE_INTERPOLATOR, interpolator.addr());
    } else {
        set_byte(e, this, SINGLE_INDEX, 0xff);
        store_invalid(e, this.addr() + SINGLE_TIME);
        e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, 0);
    }
    invalidate_weights(e, this);
    index
}

// Translated from 00a36a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::RemoveInterpInfo` (Xbox PDB): releases item
/// `index`'s interpolator (cleared); when its priority was the highest or
/// next-highest, both are recomputed from the items still holding one
/// (from `0x80`); one fewer interpolator: with one left it becomes the
/// single one (for an array of two the other index, `index == 0`, otherwise
/// the first used item; the single index is left as it was when none is
/// found) with its update time as the single time; with none left the
/// priorities are `0x80`, the single index `0xFF`, no single interpolator,
/// single time `INVALID_FLOAT`. The weights are marked out of date.
pub fn ni_blend_interpolator_remove_interp_info(e: &mut Engine, this: Ptr, index: u8) {
    let slot = item(e, this, u32::from(index));
    let interpolator = e.mem.u32(slot);
    let priority = e.mem.u8(slot + ITEM_PRIORITY);
    if interpolator != 0 {
        if interlocked_decrement(e, interpolator.wrapping_add(4)) == 0 {
            e.vcall(interpolator, 4, &args![]);
        }
        e.mem.set_u32(slot, 0);
    }
    if priority == byte(e, this, HIGH_PRIORITY) || priority == byte(e, this, NEXT_HIGH_PRIORITY) {
        set_byte(e, this, NEXT_HIGH_PRIORITY, 0x80);
        set_byte(e, this, HIGH_PRIORITY, 0x80);
        let mut i = 0u8;
        while i < byte(e, this, ARRAY_SIZE) {
            let other = item(e, this, u32::from(i));
            if e.mem.u32(other) != 0 {
                let p = e.mem.u8(other + ITEM_PRIORITY) as i8;
                if p > byte(e, this, NEXT_HIGH_PRIORITY) as i8 {
                    let high = byte(e, this, HIGH_PRIORITY) as i8;
                    if p > high {
                        set_byte(e, this, NEXT_HIGH_PRIORITY, high as u8);
                        let p = e.mem.u8(other + ITEM_PRIORITY);
                        set_byte(e, this, HIGH_PRIORITY, p);
                    } else if p < high {
                        set_byte(e, this, NEXT_HIGH_PRIORITY, p as u8);
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    let count = byte(e, this, INTERP_COUNT).wrapping_add(0xff);
    set_byte(e, this, INTERP_COUNT, count);
    if count == 1 {
        let size = byte(e, this, ARRAY_SIZE);
        if size == 2 {
            set_byte(e, this, SINGLE_INDEX, u8::from(index == 0));
        } else {
            let mut i = 0u8;
            while i < size {
                if e.mem.u32(item(e, this, u32::from(i))) != 0 {
                    set_byte(e, this, SINGLE_INDEX, i);
                    break;
                }
                i = i.wrapping_add(1);
            }
        }
        let single = item(e, this, u32::from(byte(e, this, SINGLE_INDEX)));
        let interpolator = e.mem.u32(single);
        e.mem
            .set_u32(this.addr() + SINGLE_INTERPOLATOR, interpolator);
        copy_float(e, single + ITEM_UPDATE_TIME, this.addr() + SINGLE_TIME);
    } else if count == 0 {
        set_byte(e, this, NEXT_HIGH_PRIORITY, 0x80);
        set_byte(e, this, HIGH_PRIORITY, 0x80);
        set_byte(e, this, SINGLE_INDEX, 0xff);
        e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, 0);
        store_invalid(e, this.addr() + SINGLE_TIME);
    }
    invalidate_weights(e, this);
}

// Translated from 00a36bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ComputeNormalizedWeights`' case of exactly
/// two interpolators): takes the first two used items A and B (the exe
/// reads through null when there are fewer) and their weights times ease
/// spinners `wa`, `wb` (rounded to floats). Both 0: both normalized weights
/// 0. Otherwise by priority: A's higher with A's spinner 1 gives (1, 0),
/// else the weights `wa * ea` and `wb * (1 - ea)` normalized; B's higher
/// the same mirrored ((0, 1) for B's spinner 1, else `wa * (1 - eb)` and
/// `eb * wb`); equal priorities `wa` and `wb` normalized (each through the
/// reciprocal of the sum, rounded to a float). Then with "only the highest
/// weight" (flag 2) the larger gets 1 and the other 0 (A on a tie, B when
/// unordered); otherwise with a positive threshold a weight below it is
/// zeroed, and the other becomes 1 when either was.
pub fn fn_00a36bd0(e: &mut Engine, this: Ptr) {
    let size = byte(e, this, ARRAY_SIZE);
    let mut first = 0u32;
    let mut second = 0u32;
    let mut i = 0u8;
    while i < size {
        let candidate = item(e, this, u32::from(i));
        if e.mem.u32(candidate) != 0 {
            if first != 0 {
                second = candidate;
                break;
            }
            first = candidate;
        }
        i = i.wrapping_add(1);
    }
    let load = |e: &Engine, at: u32| x87_load32(e.mem.u32(at));
    let operand = |e: &Engine, at: u32| x87_m32(e.mem.u32(at));
    let wa = x87_store32(x87_mul(
        load(e, first + ITEM_WEIGHT),
        operand(e, first + ITEM_EASE_SPINNER),
    ));
    let wb = x87_store32(x87_mul(
        load(e, second + ITEM_WEIGHT),
        operand(e, second + ITEM_EASE_SPINNER),
    ));
    let (wa, wb) = (x87_load32(wa), x87_load32(wb));
    if wa == 0.0 && wb == 0.0 {
        e.mem.set_u32(first + ITEM_NORMALIZED_WEIGHT, 0);
        e.mem.set_u32(second + ITEM_NORMALIZED_WEIGHT, 0);
        return;
    }
    let set = |e: &mut Engine, a: u32, b: u32| {
        e.mem.set_u32(first + ITEM_NORMALIZED_WEIGHT, a);
        e.mem.set_u32(second + ITEM_NORMALIZED_WEIGHT, b);
    };
    let priority_a = e.mem.u8(first + ITEM_PRIORITY) as i8;
    let priority_b = e.mem.u8(second + ITEM_PRIORITY) as i8;
    let (a, b) = if priority_a > priority_b {
        if operand(e, first + ITEM_EASE_SPINNER) == 1.0 {
            set(e, ONE_BITS, 0);
            return;
        }
        let ease = load(e, first + ITEM_EASE_SPINNER);
        let one_minus = x87_load32(x87_store32(x87_sub(1.0, ease)));
        let ease = load(e, first + ITEM_EASE_SPINNER);
        let weighted_a = x87_mul(wa, ease);
        let weighted_b = x87_mul(wb, one_minus);
        let inverse = x87_load32(x87_store32(x87_div(1.0, x87_add(weighted_b, weighted_a))));
        (
            x87_store32(x87_mul(weighted_a, inverse)),
            x87_store32(x87_mul(weighted_b, inverse)),
        )
    } else if priority_a < priority_b {
        if operand(e, second + ITEM_EASE_SPINNER) == 1.0 {
            set(e, 0, ONE_BITS);
            return;
        }
        let ease = load(e, second + ITEM_EASE_SPINNER);
        let one_minus = x87_load32(x87_store32(x87_sub(1.0, ease)));
        let weighted_a = x87_mul(wa, one_minus);
        let ease = load(e, second + ITEM_EASE_SPINNER);
        let sum = x87_add(x87_mul(ease, wb), weighted_a);
        let inverse = x87_load32(x87_store32(x87_div(1.0, sum)));
        let a = x87_store32(x87_mul(weighted_a, inverse));
        let ease = load(e, second + ITEM_EASE_SPINNER);
        let weighted_b = x87_mul(wb, ease);
        (a, x87_store32(x87_mul(inverse, weighted_b)))
    } else {
        let inverse = x87_load32(x87_store32(x87_div(1.0, x87_add(wb, wa))));
        (
            x87_store32(x87_mul(wa, inverse)),
            x87_store32(x87_mul(wb, inverse)),
        )
    };
    e.mem.set_u32(first + ITEM_NORMALIZED_WEIGHT, a);
    e.mem.set_u32(second + ITEM_NORMALIZED_WEIGHT, b);
    if byte(e, this, FLAGS) & ONLY_USE_HIGHEST_WEIGHT != 0 {
        let a = load(e, first + ITEM_NORMALIZED_WEIGHT);
        let b = load(e, second + ITEM_NORMALIZED_WEIGHT);
        if b <= a {
            set(e, ONE_BITS, 0);
        } else {
            set(e, 0, ONE_BITS);
        }
        return;
    }
    let threshold = this.addr() + WEIGHT_THRESHOLD;
    if !(0.0 < operand(e, threshold)) {
        return;
    }
    let mut zeroed = false;
    if load(e, threshold) > load(e, first + ITEM_NORMALIZED_WEIGHT) {
        e.mem.set_u32(first + ITEM_NORMALIZED_WEIGHT, 0);
        zeroed = true;
    }
    if load(e, threshold) > load(e, second + ITEM_NORMALIZED_WEIGHT) {
        e.mem.set_u32(second + ITEM_NORMALIZED_WEIGHT, 0);
        e.mem.set_u32(second + ITEM_NORMALIZED_WEIGHT, ONE_BITS);
        return;
    }
    if zeroed {
        e.mem.set_u32(second + ITEM_NORMALIZED_WEIGHT, ONE_BITS);
    }
}

// Translated from 00a37260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::ComputeNormalizedWeights` (Xbox PDB): only when
/// the weights are out of date (flag 4, then cleared). One interpolator:
/// its normalized weight 1. Two: `fn_00a36bd0`. Otherwise, when the high
/// sum is unset (`-FLT_MAX`), the sums of `weight * ease spinner` of the
/// highest- and next-highest-priority items and the largest ease spinner
/// among the highest are recomputed (from 0); each used item's normalized
/// weight is then `weight * spinner * ease * r` (highest priority),
/// `weight * (1 - spinner) * ease * r` (next highest) or 0, with `r` the
/// reciprocal of `high * spinner + next * (1 - spinner)` (0 when that is
/// not positive). With a positive threshold, weights below it are zeroed
/// and the rest scaled to sum to 1 (by 0 when the sum is not positive;
/// untouched when it is exactly 1). With "only the highest weight" (flag 2)
/// the first largest weight becomes 1 and every other 0 (the exe writes
/// item `0xFF` for an empty array). The float roundings follow the exe's
/// stores.
pub fn ni_blend_interpolator_compute_normalized_weights(e: &mut Engine, this: Ptr) {
    let flags = byte(e, this, FLAGS);
    if flags & COMPUTE_NORMALIZED_WEIGHTS == 0 {
        return;
    }
    set_byte(e, this, FLAGS, flags & !COMPUTE_NORMALIZED_WEIGHTS);
    match byte(e, this, INTERP_COUNT) {
        1 => {
            let single = item(e, this, u32::from(byte(e, this, SINGLE_INDEX)));
            e.mem.set_u32(single + ITEM_NORMALIZED_WEIGHT, ONE_BITS);
            return;
        }
        2 => {
            fn_00a36bd0(e, this);
            return;
        }
        _ => {}
    }
    let at = |offset: u32| this.addr().wrapping_add(offset);
    let load = |e: &Engine, address: u32| x87_load32(e.mem.u32(address));
    let operand = |e: &Engine, address: u32| x87_m32(e.mem.u32(address));
    let lowest = f64::from_bits(e.global::<u64>(LOWEST));
    if load(e, at(HIGH_SUM_OF_WEIGHTS)) == lowest {
        e.mem.set_u32(at(HIGH_SUM_OF_WEIGHTS), 0);
        e.mem.set_u32(at(NEXT_HIGH_SUM_OF_WEIGHTS), 0);
        e.mem.set_u32(at(HIGH_EASE_SPINNER), 0);
        let mut i = 0u8;
        while i < byte(e, this, ARRAY_SIZE) {
            let entry = item(e, this, u32::from(i));
            if e.mem.u32(entry) != 0 {
                let weighted = x87_store32(x87_mul(
                    load(e, entry + ITEM_WEIGHT),
                    operand(e, entry + ITEM_EASE_SPINNER),
                ));
                let priority = e.mem.u8(entry + ITEM_PRIORITY);
                if priority == byte(e, this, HIGH_PRIORITY) {
                    let sum = x87_add(load(e, at(HIGH_SUM_OF_WEIGHTS)), x87_m32(weighted));
                    e.mem.set_u32(at(HIGH_SUM_OF_WEIGHTS), x87_store32(sum));
                    if load(e, at(HIGH_EASE_SPINNER)) < load(e, entry + ITEM_EASE_SPINNER) {
                        copy_float(e, entry + ITEM_EASE_SPINNER, at(HIGH_EASE_SPINNER));
                    }
                } else if priority == byte(e, this, NEXT_HIGH_PRIORITY) {
                    let sum = x87_add(
                        x87_load32(weighted),
                        operand(e, at(NEXT_HIGH_SUM_OF_WEIGHTS)),
                    );
                    e.mem
                        .set_u32(at(NEXT_HIGH_SUM_OF_WEIGHTS), x87_store32(sum));
                }
            }
            i = i.wrapping_add(1);
        }
    }
    let one_minus_spinner = x87_load32(x87_store32(x87_sub(1.0, load(e, at(HIGH_EASE_SPINNER)))));
    let next_part = x87_mul(load(e, at(NEXT_HIGH_SUM_OF_WEIGHTS)), one_minus_spinner);
    let high_part = x87_mul(
        load(e, at(HIGH_SUM_OF_WEIGHTS)),
        operand(e, at(HIGH_EASE_SPINNER)),
    );
    let total = x87_load32(x87_store32(x87_add(next_part, high_part)));
    let reciprocal = if total > 0.0 {
        x87_div(1.0, total)
    } else {
        0.0
    };
    let reciprocal = x87_load32(x87_store32(reciprocal));
    let mut i = 0u8;
    while i < byte(e, this, ARRAY_SIZE) {
        let entry = item(e, this, u32::from(i));
        if e.mem.u32(entry) != 0 {
            let priority = e.mem.u8(entry + ITEM_PRIORITY);
            if priority == byte(e, this, HIGH_PRIORITY) {
                let w = x87_mul(
                    load(e, entry + ITEM_WEIGHT),
                    operand(e, at(HIGH_EASE_SPINNER)),
                );
                let w = x87_mul(w, operand(e, entry + ITEM_EASE_SPINNER));
                let w = x87_mul(w, reciprocal);
                e.mem
                    .set_u32(entry + ITEM_NORMALIZED_WEIGHT, x87_store32(w));
            } else if priority == byte(e, this, NEXT_HIGH_PRIORITY) {
                let w = x87_mul(load(e, entry + ITEM_WEIGHT), one_minus_spinner);
                let w = x87_mul(w, operand(e, entry + ITEM_EASE_SPINNER));
                let w = x87_mul(w, reciprocal);
                e.mem
                    .set_u32(entry + ITEM_NORMALIZED_WEIGHT, x87_store32(w));
            } else {
                e.mem.set_u32(entry + ITEM_NORMALIZED_WEIGHT, 0);
            }
        }
        i = i.wrapping_add(1);
    }
    if 0.0 < operand(e, at(WEIGHT_THRESHOLD)) {
        let mut sum = 0u32;
        let mut i = 0u8;
        while i < byte(e, this, ARRAY_SIZE) {
            let entry = item(e, this, u32::from(i));
            if e.mem.u32(entry) != 0 && !(operand(e, entry + ITEM_NORMALIZED_WEIGHT) == 0.0) {
                if load(e, at(WEIGHT_THRESHOLD)) > load(e, entry + ITEM_NORMALIZED_WEIGHT) {
                    e.mem.set_u32(entry + ITEM_NORMALIZED_WEIGHT, 0);
                }
                let added = x87_add(load(e, entry + ITEM_NORMALIZED_WEIGHT), x87_m32(sum));
                sum = x87_store32(added);
            }
            i = i.wrapping_add(1);
        }
        let sum = x87_load32(sum);
        if sum != 1.0 {
            let factor = if sum > 0.0 {
                x87_store32(x87_div(1.0, sum))
            } else {
                0
            };
            let factor = x87_load32(factor);
            let mut i = 0u8;
            while i < byte(e, this, ARRAY_SIZE) {
                let entry = item(e, this, u32::from(i)) + ITEM_NORMALIZED_WEIGHT;
                if !(0.0 == operand(e, entry)) {
                    let scaled = x87_mul(load(e, entry), factor);
                    e.mem.set_u32(entry, x87_store32(scaled));
                }
                i = i.wrapping_add(1);
            }
        }
    }
    if byte(e, this, FLAGS) & ONLY_USE_HIGHEST_WEIGHT == 0 {
        return;
    }
    let mut highest = x87_load32(e.mem.u32(MINUS_ONE));
    let mut index = 0xffu8;
    let mut i = 0u8;
    while i < byte(e, this, ARRAY_SIZE) {
        let entry = item(e, this, u32::from(i)) + ITEM_NORMALIZED_WEIGHT;
        let weight = load(e, entry);
        if highest < weight {
            highest = weight;
            index = i;
        }
        e.mem.set_u32(entry, 0);
        i = i.wrapping_add(1);
    }
    let chosen = item(e, this, u32::from(index));
    e.mem.set_u32(chosen + ITEM_NORMALIZED_WEIGHT, ONE_BITS);
}

// Translated from 00a36e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::ProcessClone` (Xbox PDB): `NiObject::ProcessClone`,
/// looks up this object's clone in the cloning process's clone map
/// (`00a53c30` on the map at `cloning + 0`), runs `ProcessClone` (vtable
/// +0x68) of every item's interpolator, then sets the clone's single
/// interpolator to the clone of this one's (when the map has it) or this
/// one's own.
pub fn ni_blend_interpolator_process_clone(e: &mut Engine, this: Ptr, cloning: Ptr) {
    e.call(NI_OBJECT_PROCESS_CLONE, &args![this, cloning]);
    e.with_stack(4, |e, found| {
        e.mem.set_u32(found.addr(), 0);
        let map = e.mem.u32(cloning.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, this, found]);
        let clone = e.mem.u32(found.addr());
        let mut i = 0u8;
        while i < byte(e, this, ARRAY_SIZE) {
            let interpolator = e.mem.u32(item(e, this, u32::from(i)));
            if interpolator != 0 {
                e.vcall(interpolator, 0x68, &args![cloning]);
            }
            i = i.wrapping_add(1);
        }
        let single = e.mem.u32(this.addr() + SINGLE_INTERPOLATOR);
        if single == 0 {
            return;
        }
        let map = e.mem.u32(cloning.addr());
        let value = if e.call(CLONE_MAP_LOOKUP, &args![map, single, found]).bool() {
            e.mem.u32(found.addr())
        } else {
            e.mem.u32(this.addr() + SINGLE_INTERPOLATOR)
        };
        e.mem
            .set_u32(clone.wrapping_add(SINGLE_INTERPOLATOR), value);
    });
}

// Translated from 00a37510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NiBlendInterpolator`'s `CopyMembers`, the
/// target of `fn_00a40aa0`'s jump): `NiKeyBasedInterpolator::CopyMembers`,
/// then the flags and array size, a new item array of that size and the
/// weight threshold; unless the interpolator is manager controlled (flag
/// 1), every item is copied (its interpolator replaced by a clone made by
/// `CreateClone`, vtable +0x48, when it has one) and the interpolator
/// count, single index, single time and both priorities.
pub fn fn_00a37510(e: &mut Engine, this: Ptr, destination: Ptr, cloning: Ptr) {
    e.call(KEY_BASED_COPY_MEMBERS, &args![this, destination, cloning]);
    let flags = byte(e, this, FLAGS);
    set_byte(e, destination, FLAGS, flags);
    let size = byte(e, this, ARRAY_SIZE);
    set_byte(e, destination, ARRAY_SIZE, size);
    let size = byte(e, this, ARRAY_SIZE);
    let array = new_item_array(e, u32::from(size));
    e.mem.set_u32(destination.addr() + INTERP_ARRAY, array);
    copy_float(
        e,
        this.addr() + WEIGHT_THRESHOLD,
        destination.addr() + WEIGHT_THRESHOLD,
    );
    if byte(e, this, FLAGS) & MANAGER_CONTROLLED != 0 {
        return;
    }
    let mut i = 0u8;
    while i < byte(e, this, ARRAY_SIZE) {
        let from = item(e, this, u32::from(i));
        let to = item(e, destination, u32::from(i));
        let interpolator = e.mem.u32(from);
        if interpolator != 0 {
            let clone = e.vcall(interpolator, 0x48, &args![cloning]).u32();
            assign_ref_counted(e, to, clone);
        }
        copy_float(e, from + ITEM_WEIGHT, to + ITEM_WEIGHT);
        copy_float(
            e,
            from + ITEM_NORMALIZED_WEIGHT,
            to + ITEM_NORMALIZED_WEIGHT,
        );
        let priority = e.mem.u8(from + ITEM_PRIORITY);
        e.mem.set_u8(to + ITEM_PRIORITY, priority);
        copy_float(e, from + ITEM_EASE_SPINNER, to + ITEM_EASE_SPINNER);
        copy_float(e, from + ITEM_UPDATE_TIME, to + ITEM_UPDATE_TIME);
        i = i.wrapping_add(1);
    }
    for offset in [INTERP_COUNT, SINGLE_INDEX] {
        let value = byte(e, this, offset);
        set_byte(e, destination, offset, value);
    }
    copy_float(
        e,
        this.addr() + SINGLE_TIME,
        destination.addr() + SINGLE_TIME,
    );
    for offset in [HIGH_PRIORITY, NEXT_HIGH_PRIORITY] {
        let value = byte(e, this, offset);
        set_byte(e, destination, offset, value);
    }
}

// Translated from 00a37690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::ReallocateArrays` (Xbox PDB): `false` when the
/// grow step (`ms_ucArrayGrowBy`) is 0. Otherwise a new item array of
/// `size + step` items (a byte; at least 4), the old items assigned into it
/// (`fn_00a371f0`), the old array deleted, and the new array and size kept.
/// Answers `true`.
pub fn ni_blend_interpolator_reallocate_arrays(e: &mut Engine, this: Ptr) -> bool {
    let grow_by = e.mem.u8(ARRAY_GROW_BY);
    if grow_by == 0 {
        return false;
    }
    let mut size = byte(e, this, ARRAY_SIZE).wrapping_add(grow_by);
    if size < 4 {
        size = 4;
    }
    let array = new_item_array(e, u32::from(size));
    let mut i = 0u8;
    while i < byte(e, this, ARRAY_SIZE) {
        let offset = u32::from(i) * ITEM_SIZE;
        let from = item(e, this, u32::from(i));
        fn_00a371f0(e, Ptr::new(offset.wrapping_add(array)), Ptr::new(from));
        i = i.wrapping_add(1);
    }
    let old = e.mem.u32(this.addr() + INTERP_ARRAY);
    if old != 0 {
        delete_item_array(e, old);
    }
    e.mem.set_u32(this.addr() + INTERP_ARRAY, array);
    set_byte(e, this, ARRAY_SIZE, size);
    true
}

/// `NiStream::m_uiNifFileVersion` (Xbox PDB) +0xD8 and `m_pkIstr` +0x24C.
const STREAM_FILE_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
/// The file versions `LoadBinary` tells apart.
const VERSION_SINGLE_LINK: u32 = 0x0a01_006c;
const VERSION_BYTE_COUNTS: u32 = 0x0a01_006e;
const VERSION_FLAGS_FIRST: u32 = 0x0a01_0070;

/// One read through the stream's input `m_pfnRead` (`cdecl (istr, buffer,
/// size, &component size, 1)`, component size `width` written first), the
/// position (+4) advanced by the bytes read.
fn read(e: &mut Engine, stream: Ptr, buffer: u32, size: u32, component_size: u32, width: u32) {
    let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
    let function = e.mem.u32(input.wrapping_add(8));
    e.mem.set_u32(component_size, width);
    let got = e
        .call(function, &args![input, buffer, size, component_size, 1u32])
        .u32();
    let position = input.wrapping_add(4);
    let advanced = e.mem.u32(position).wrapping_add(got);
    e.mem.set_u32(position, advanced);
}

// Translated from 00a37980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::LoadBinary` (Xbox PDB): `NiInterpolator::LoadBinary`,
/// then by file version. From `0x0A010070`: the flags (1 byte, "weights out
/// of date" cleared), the array size (1 byte) and a new item array, the
/// threshold, and unless manager controlled the count, single index, both
/// priorities (1 byte each), single time, the three sums (4 bytes each),
/// every item (`InterpArrayItem::LoadBinary`) and the single interpolator's
/// link (`ResolveLinkID`). Before it: the array size (2 bytes, a second 2
/// discarded before `0x0A01006E`, else 1 byte) and the array, the items,
/// the manager-controlled and only-highest flags (1 byte each), the
/// threshold, the count and single index (2 bytes each before `0x0A01006E`,
/// else 1), from `0x0A01006C` the single link and time, and the priorities
/// (4-byte integers, `0x80000000` meaning `0x80`, before `0x0A01006E`; else 1
/// byte each). The buffers and component sizes are the stack words the exe
/// uses (the stream argument's word and three locals).
pub fn ni_blend_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
    let at = |offset: u32| this.addr().wrapping_add(offset);
    let version = |e: &Engine| e.mem.u32(stream.addr().wrapping_add(STREAM_FILE_VERSION));
    e.with_stack(16, |e, frame| {
        // The stream argument's stack word and the three locals.
        let argument = frame.addr();
        let first_local = argument + 4;
        let second_local = argument + 8;
        let third_local = argument + 0xc;
        e.mem.set_u32(argument, stream.addr());
        if version(e) >= VERSION_FLAGS_FIRST {
            read(e, stream, at(FLAGS), 1, argument, 1);
            let flags = byte(e, this, FLAGS) & !COMPUTE_NORMALIZED_WEIGHTS;
            set_byte(e, this, FLAGS, flags);
        }
        if version(e) < VERSION_BYTE_COUNTS {
            read(e, stream, argument, 2, first_local, 2);
            let size = e.mem.u8(argument);
            set_byte(e, this, ARRAY_SIZE, size);
            read(e, stream, second_local, 2, first_local, 2);
        } else {
            read(e, stream, at(ARRAY_SIZE), 1, argument, 1);
        }
        let size = byte(e, this, ARRAY_SIZE);
        let array = new_item_array(e, u32::from(size));
        if array != 0 {
            e.mem.set_u32(argument, array);
        }
        e.mem.set_u32(at(INTERP_ARRAY), array);
        let load_items = |e: &mut Engine| {
            e.mem.set_u8(argument, 0);
            while e.mem.u8(argument) < byte(e, this, ARRAY_SIZE) {
                let index = u32::from(e.mem.u8(argument));
                let entry = item(e, this, index);
                e.call(ITEM_LOAD_BINARY, &args![entry, stream]);
                let next = e.mem.u8(argument).wrapping_add(1);
                e.mem.set_u8(argument, next);
            }
        };
        if version(e) >= VERSION_FLAGS_FIRST {
            read(e, stream, at(WEIGHT_THRESHOLD), 4, argument, 4);
            if byte(e, this, FLAGS) & MANAGER_CONTROLLED != 0 {
                return;
            }
            for offset in [
                INTERP_COUNT,
                SINGLE_INDEX,
                HIGH_PRIORITY,
                NEXT_HIGH_PRIORITY,
            ] {
                read(e, stream, at(offset), 1, argument, 1);
            }
            for offset in [
                SINGLE_TIME,
                HIGH_SUM_OF_WEIGHTS,
                NEXT_HIGH_SUM_OF_WEIGHTS,
                HIGH_EASE_SPINNER,
            ] {
                read(e, stream, at(offset), 4, argument, 4);
            }
            load_items(e);
            let link = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
            e.mem.set_u32(at(SINGLE_INTERPOLATOR), link);
            return;
        }
        load_items(e);
        read(e, stream, third_local + 2, 1, argument, 1);
        let flags = if e.mem.u8(third_local + 2) != 0 {
            byte(e, this, FLAGS) | MANAGER_CONTROLLED
        } else {
            byte(e, this, FLAGS) & !MANAGER_CONTROLLED
        };
        set_byte(e, this, FLAGS, flags);
        read(e, stream, at(WEIGHT_THRESHOLD), 4, argument, 4);
        read(e, stream, third_local + 3, 1, argument, 1);
        let flags = if e.mem.u8(third_local + 3) != 0 {
            byte(e, this, FLAGS) | ONLY_USE_HIGHEST_WEIGHT
        } else {
            byte(e, this, FLAGS) & !ONLY_USE_HIGHEST_WEIGHT
        };
        set_byte(e, this, FLAGS, flags);
        if version(e) < VERSION_BYTE_COUNTS {
            read(e, stream, argument, 2, first_local, 2);
            let count = e.mem.u8(argument);
            set_byte(e, this, INTERP_COUNT, count);
            read(e, stream, second_local, 2, first_local, 2);
            let index = e.mem.u8(second_local);
            set_byte(e, this, SINGLE_INDEX, index);
        } else {
            read(e, stream, at(INTERP_COUNT), 1, argument, 1);
            read(e, stream, at(SINGLE_INDEX), 1, argument, 1);
        }
        if version(e) >= VERSION_SINGLE_LINK {
            let link = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
            e.mem.set_u32(at(SINGLE_INTERPOLATOR), link);
            read(e, stream, at(SINGLE_TIME), 4, argument, 4);
        }
        if version(e) < VERSION_BYTE_COUNTS {
            read(e, stream, first_local, 4, argument, 4);
            let high = e.mem.u32(first_local);
            let high = if high == 0x8000_0000 {
                0x80
            } else {
                high as u8
            };
            set_byte(e, this, HIGH_PRIORITY, high);
            read(e, stream, second_local, 4, argument, 4);
            let next = e.mem.u32(second_local);
            let next = if next == 0x8000_0000 {
                0x80
            } else {
                next as u8
            };
            set_byte(e, this, NEXT_HIGH_PRIORITY, next);
        } else {
            read(e, stream, at(HIGH_PRIORITY), 1, argument, 1);
            read(e, stream, at(NEXT_HIGH_PRIORITY), 1, argument, 1);
        }
    });
}

/// `NiStream::m_pkOstr` (Xbox PDB) +0x250: the stream being written.
const STREAM_OUTPUT: u32 = 0x250;
/// The viewer-string array's resize (`0096ad30`, `thiscall (array, new
/// size)`) and its store (`0096ae90`, `thiscall (array, index, &string)`).
const STRINGS_SET_SIZE: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

/// One read through `stream`'s input (`m_pkIstr`, its read function at +8,
/// `cdecl (istr, buffer, size, &component size, 1)`) with the component-size
/// word at `component_size` set to `size` first; the position (+4) advances
/// by the bytes read. The animation units' `LoadBinary` functions share it.
pub(crate) fn stream_read(
    e: &mut Engine,
    stream: Ptr,
    buffer: u32,
    size: u32,
    component_size: u32,
) {
    read(e, stream, buffer, size, component_size, size);
}

/// The write counterpart of [`stream_read`]: `stream`'s output (`m_pkOstr`
/// +0x250) and its write function (+0xC).
pub(crate) fn stream_write(
    e: &mut Engine,
    stream: Ptr,
    buffer: u32,
    size: u32,
    component_size: u32,
) {
    let output = e.mem.u32(stream.addr().wrapping_add(STREAM_OUTPUT));
    let function = e.mem.u32(output.wrapping_add(0xc));
    e.mem.set_u32(component_size, size);
    let written = e
        .call(function, &args![output, buffer, size, component_size, 1u32])
        .u32();
    let position = output.wrapping_add(4);
    let advanced = e.mem.u32(position).wrapping_add(written);
    e.mem.set_u32(position, advanced);
}

/// The inline append of a viewer string (`NiTPrimitiveArray<char*>`: size
/// `u16` +0xA, capacity +0x8, growth step +0xE): `text` goes into the stack
/// word at `slot`; when the size has reached the capacity the array is
/// resized to the step plus the size (`0096ad30`); then the string is stored
/// at the old size (`0096ae90`, given `slot`). The animation units'
/// `GetViewerStrings` functions share it.
pub(crate) fn append_viewer_string(e: &mut Engine, strings: Ptr, slot: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings.addr().wrapping_add(0xa)));
    e.mem.set_u32(slot, text);
    if size >= u32::from(e.mem.u16(strings.addr().wrapping_add(8))) {
        let step = u32::from(e.mem.u16(strings.addr().wrapping_add(0xe)));
        e.call(STRINGS_SET_SIZE, &args![strings, step.wrapping_add(size)]);
    }
    e.call(STRINGS_SET_AT, &args![strings, size, slot]);
}

/// `NiInterpolator`'s virtual slots (Xbox PDB order, the offsets the PC code
/// uses): `IsEqual` +0x5C, `RegisterStreamables` +0x54, `GetViewerStrings`
/// +0x60, `Collapse` +0xBC, `GuaranteeTimeRange` +0xC4.
const SLOT_IS_EQUAL: u32 = 0x5c;
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_GET_VIEWER_STRINGS: u32 = 0x60;
const SLOT_COLLAPSE: u32 = 0xbc;
const SLOT_GUARANTEE_TIME_RANGE: u32 = 0xc4;
/// `NiStream`'s `SaveLinkID` slot (+0x30, one word: the object).
const STREAM_SLOT_SAVE_LINK_ID: u32 = 0x30;

// Translated from 00a36dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::Collapse` (Xbox PDB): with exactly one interpolator
/// a tail call of the single interpolator's `Collapse` (vtable +0xBC);
/// otherwise `Collapse` of every item's interpolator.
pub fn ni_blend_interpolator_collapse(e: &mut Engine, this: Ptr) {
    if byte(e, this, INTERP_COUNT) == 1 {
        let single = e.mem.u32(this.addr() + SINGLE_INTERPOLATOR);
        e.vcall(single, SLOT_COLLAPSE, &args![]);
        return;
    }
    let mut i = 0u8;
    while i < byte(e, this, ARRAY_SIZE) {
        let interpolator = e.mem.u32(item(e, this, u32::from(i)));
        if interpolator != 0 {
            e.vcall(interpolator, SLOT_COLLAPSE, &args![]);
        }
        i = i.wrapping_add(1);
    }
}

// Translated from 00a36e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::GuaranteeTimeRange` (Xbox PDB): passes both times
/// (each through `FLD`/`FSTP`) to the single interpolator's
/// `GuaranteeTimeRange` (vtable +0xC4) with exactly one interpolator,
/// otherwise to that of every item's interpolator.
pub fn ni_blend_interpolator_guarantee_time_range(e: &mut Engine, this: Ptr, start: f32, end: f32) {
    let start = x87_quiet_bits(start.to_bits());
    let end = x87_quiet_bits(end.to_bits());
    if byte(e, this, INTERP_COUNT) == 1 {
        let single = e.mem.u32(this.addr() + SINGLE_INTERPOLATOR);
        e.vcall(single, SLOT_GUARANTEE_TIME_RANGE, &args![start, end]);
        return;
    }
    let mut i = 0u8;
    while i < byte(e, this, ARRAY_SIZE) {
        let interpolator = e.mem.u32(item(e, this, u32::from(i)));
        if interpolator != 0 {
            e.vcall(interpolator, SLOT_GUARANTEE_TIME_RANGE, &args![start, end]);
        }
        i = i.wrapping_add(1);
    }
}

// Translated from 00a36f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, an `InterpArrayItem` compared with
/// `other`): both interpolators null, or both set and this one's `IsEqual`
/// (vtable +0x5C) true of the other's; then the weights, the normalized
/// weights, the priorities and the ease spinners equal (the floats compared
/// as the x87 does: a NaN is never equal).
pub fn fn_00a36f10(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let (this, other) = (this.addr(), other.addr());
    let mine = e.mem.u32(this);
    let theirs = e.mem.u32(other);
    if mine == 0 {
        if theirs != 0 {
            return false;
        }
    } else if theirs == 0 || !e.vcall(mine, SLOT_IS_EQUAL, &args![theirs]).bool() {
        return false;
    }
    let equal = |e: &Engine, offset: u32| {
        x87_load32(e.mem.u32(this + offset)) == x87_load32(e.mem.u32(other + offset))
    };
    if !equal(e, ITEM_WEIGHT) || !equal(e, ITEM_NORMALIZED_WEIGHT) {
        return false;
    }
    if e.mem.u8(this + ITEM_PRIORITY) != e.mem.u8(other + ITEM_PRIORITY) {
        return false;
    }
    equal(e, ITEM_EASE_SPINNER)
}

// Translated from 00a36f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::RegisterStreamables` (Xbox PDB): the base
/// `RegisterStreamables` (`00a81fe0`); false when it is false. Unless the
/// interpolator is manager controlled (flag 1), every item's interpolator
/// registers itself (vtable +0x54). Answers true.
pub fn ni_blend_interpolator_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if !e.call(0x00a8_1fe0, &args![this, stream]).bool() {
        return false;
    }
    if byte(e, this, FLAGS) & MANAGER_CONTROLLED == 0 {
        let mut i = 0u8;
        while i < byte(e, this, ARRAY_SIZE) {
            let interpolator = e.mem.u32(item(e, this, u32::from(i)));
            if interpolator != 0 {
                e.vcall(interpolator, SLOT_REGISTER_STREAMABLES, &args![stream]);
            }
            i = i.wrapping_add(1);
        }
    }
    true
}

// Translated from 00a36fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::IsEqual` (Xbox PDB): the base `IsEqual`
/// (`00ba87d0`), then the same array size and every item equal
/// (`fn_00a36f10`), the same flags, threshold (x87 compare), interpolator
/// count and single index; when this one has a single interpolator its
/// `IsEqual` (vtable +0x5C) of the other's must be true, and the two single
/// interpolators must be the same object; then the single times (x87
/// compare) and both priorities equal.
pub fn ni_blend_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(0x00ba_87d0, &args![this, other]).bool() {
        return false;
    }
    let size = byte(e, this, ARRAY_SIZE);
    if size != byte(e, other, ARRAY_SIZE) {
        return false;
    }
    if size > 0 {
        let mut i = 0u8;
        loop {
            let offset = u32::from(i) * ITEM_SIZE;
            let theirs = e.mem.u32(other.addr() + INTERP_ARRAY).wrapping_add(offset);
            let mine = e.mem.u32(this.addr() + INTERP_ARRAY).wrapping_add(offset);
            if !fn_00a36f10(e, Ptr::new(mine), Ptr::new(theirs)) {
                return false;
            }
            i = i.wrapping_add(1);
            if i >= byte(e, this, ARRAY_SIZE) {
                break;
            }
        }
    }
    let float = |e: &Engine, p: Ptr, offset: u32| x87_load32(e.mem.u32(p.addr() + offset));
    if byte(e, this, FLAGS) != byte(e, other, FLAGS)
        || float(e, this, WEIGHT_THRESHOLD) != float(e, other, WEIGHT_THRESHOLD)
        || byte(e, this, INTERP_COUNT) != byte(e, other, INTERP_COUNT)
        || byte(e, this, SINGLE_INDEX) != byte(e, other, SINGLE_INDEX)
    {
        return false;
    }
    let single = e.mem.u32(this.addr() + SINGLE_INTERPOLATOR);
    if single != 0 {
        let theirs = e.mem.u32(other.addr() + SINGLE_INTERPOLATOR);
        if !e.vcall(single, SLOT_IS_EQUAL, &args![theirs]).bool() {
            return false;
        }
    }
    if e.mem.u32(this.addr() + SINGLE_INTERPOLATOR) != e.mem.u32(other.addr() + SINGLE_INTERPOLATOR)
        || float(e, this, SINGLE_TIME) != float(e, other, SINGLE_TIME)
    {
        return false;
    }
    byte(e, this, HIGH_PRIORITY) == byte(e, other, HIGH_PRIORITY)
        && byte(e, this, NEXT_HIGH_PRIORITY) == byte(e, other, NEXT_HIGH_PRIORITY)
}

// Translated from 00a377b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::InterpArrayItem::LoadBinary` (Xbox PDB): the
/// interpolator from `ResolveLinkID` (assigned as a smart pointer), then the
/// weight and normalized weight (4 bytes each), the priority (a 4-byte
/// integer before file version `0x0A01006E`, `0x80000000` meaning `0x80`,
/// otherwise 1 byte) and the ease spinner (4 bytes). The component sizes go
/// into the stream argument's stack word; the 4-byte priority is read into
/// the local the exe keeps there (which held `this`).
pub fn ni_blend_interpolator_interp_array_item_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let link = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
    assign_ref_counted(e, this.addr(), link);
    e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let argument = local + 4;
        e.mem.set_u32(local, this.addr());
        e.mem.set_u32(argument, stream.addr());
        let at = |offset: u32| this.addr().wrapping_add(offset);
        stream_read(e, stream, at(ITEM_WEIGHT), 4, argument);
        stream_read(e, stream, at(ITEM_NORMALIZED_WEIGHT), 4, argument);
        if e.mem.u32(stream.addr().wrapping_add(STREAM_FILE_VERSION)) < VERSION_BYTE_COUNTS {
            stream_read(e, stream, local, 4, argument);
            let priority = e.mem.u32(local);
            let priority = if priority == 0x8000_0000 {
                0x80
            } else {
                priority as u8
            };
            e.mem.set_u8(at(ITEM_PRIORITY), priority);
        } else {
            stream_read(e, stream, at(ITEM_PRIORITY), 1, argument);
        }
        stream_read(e, stream, at(ITEM_EASE_SPINNER), 4, argument);
    });
}

// Translated from 00a378d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`InterpArrayItem::SaveBinary`, `thiscall (item,
/// stream)`): the interpolator's link (`SaveLinkID`, the stream's vtable
/// +0x30), then the weight, the normalized weight (4 bytes each), the
/// priority (1 byte) and the ease spinner (4 bytes); the component sizes go
/// into the stream argument's stack word.
pub fn fn_00a378d0(e: &mut Engine, this: Ptr, stream: Ptr) {
    let interpolator = e.mem.u32(this.addr());
    e.vcall(
        stream.addr(),
        STREAM_SLOT_SAVE_LINK_ID,
        &args![interpolator],
    );
    e.with_stack(4, |e, argument| {
        let argument = argument.addr();
        e.mem.set_u32(argument, stream.addr());
        let at = |offset: u32| this.addr().wrapping_add(offset);
        stream_write(e, stream, at(ITEM_WEIGHT), 4, argument);
        stream_write(e, stream, at(ITEM_NORMALIZED_WEIGHT), 4, argument);
        stream_write(e, stream, at(ITEM_PRIORITY), 1, argument);
        stream_write(e, stream, at(ITEM_EASE_SPINNER), 4, argument);
    });
}

// Translated from 00a37ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::SaveBinary` (Xbox PDB): the base `SaveBinary`
/// (`00a724d0`), then the flags and array size (1 byte each) and the
/// threshold (4 bytes); unless manager controlled (flag 1) the count, single
/// index and both priorities (1 byte each), the single time and the three
/// sums (4 bytes each), every item (`fn_00a378d0`) and the single
/// interpolator's link (`SaveLinkID`, the stream's vtable +0x30). The
/// component sizes go into the stream argument's stack word.
pub fn ni_blend_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(0x00a7_24d0, &args![this, stream]);
    let at = |offset: u32| this.addr().wrapping_add(offset);
    e.with_stack(4, |e, argument| {
        let argument = argument.addr();
        e.mem.set_u32(argument, stream.addr());
        stream_write(e, stream, at(FLAGS), 1, argument);
        stream_write(e, stream, at(ARRAY_SIZE), 1, argument);
        stream_write(e, stream, at(WEIGHT_THRESHOLD), 4, argument);
        if byte(e, this, FLAGS) & MANAGER_CONTROLLED != 0 {
            return;
        }
        for offset in [
            INTERP_COUNT,
            SINGLE_INDEX,
            HIGH_PRIORITY,
            NEXT_HIGH_PRIORITY,
        ] {
            stream_write(e, stream, at(offset), 1, argument);
        }
        for offset in [
            SINGLE_TIME,
            HIGH_SUM_OF_WEIGHTS,
            NEXT_HIGH_SUM_OF_WEIGHTS,
            HIGH_EASE_SPINNER,
        ] {
            stream_write(e, stream, at(offset), 4, argument);
        }
        let mut i = 0u8;
        while i < byte(e, this, ARRAY_SIZE) {
            let entry = item(e, this, u32::from(i));
            fn_00a378d0(e, Ptr::new(entry), stream);
            i = i.wrapping_add(1);
        }
        let single = e.mem.u32(at(SINGLE_INTERPOLATOR));
        e.vcall(stream.addr(), STREAM_SLOT_SAVE_LINK_ID, &args![single]);
    });
}

/// `NiInterpolator::GetViewerStrings` (`00a570b0`, `thiscall (this,
/// strings)`) and the viewer-string makers (`cdecl`): `NiGetViewerString`
/// (`00a7d330`, the class name), the byte form (`00a7d410`, `(name,
/// value)`), the bool form (`00a7d370`), the float form (`00a7d550`), the
/// signed form (`00a7d4d0`) and the string form (`00a7d5e0`).
const NI_INTERPOLATOR_GET_VIEWER_STRINGS: u32 = 0x00a5_70b0;
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_BYTE: u32 = 0x00a7_d410;
const VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const VIEWER_STRING_SIGNED: u32 = 0x00a7_d4d0;
const VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
/// The two sequence lookups `GetViewerStrings` tries (`00a2e570`,
/// `thiscall (holder, interpolator, 1)`, on the objects held at `011f36f4`
/// and `011f36f8`).
const SEQUENCE_LOOKUP: u32 = 0x00a2_e570;
const SEQUENCE_HOLDER_FIRST: u32 = 0x011f_36f4;
const SEQUENCE_HOLDER_SECOND: u32 = 0x011f_36f8;

// Translated from 00a38080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::GetViewerStrings` (Xbox PDB): `NiInterpolator`'s
/// strings, then the class name, `m_ucArraySize`, `ms_ucArrayGrowBy`,
/// `ManagerControlled`, `m_fWeightThreshold`, `m_cHighPriority`,
/// `m_cNextHighPriority`, `m_fHighSumOfWeights`, `m_fNextHighSumOfWeights`,
/// `m_bOnlyUseHighestWeight`, `m_ucInterpCount` and `m_ucSingleIdx`. Then,
/// for every item whose interpolator a sequence is found for (the first
/// holder's lookup, else the second holder's; skipped when the first holder
/// is null, or the second when the first lookup fails), the sequence's name
/// (+8), the item's priority, weight, normalized weight and ease spinner, and
/// the interpolator's own strings (vtable +0x60). The two bool strings pass
/// the stack word whose low byte the exe sets (its upper bytes are what the
/// word held: the previous string's address); the loop's strings go through
/// the exe's local (which held `this`), the others through the argument's
/// word, which also holds the loop counter.
pub fn ni_blend_interpolator_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(NI_INTERPOLATOR_GET_VIEWER_STRINGS, &args![this, strings]);
    let at = |offset: u32| this.addr().wrapping_add(offset);
    e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let argument = local + 4;
        e.mem.set_u32(local, this.addr());
        e.mem.set_u32(argument, strings.addr());
        let class_name = e.mem.u32(NI_BLEND_INTERPOLATOR_RTTI);
        let text = e.call(VIEWER_STRING, &args![class_name]).u32();
        append_viewer_string(e, strings, argument, text);
        let size = u32::from(byte(e, this, ARRAY_SIZE));
        let text = e
            .call(VIEWER_STRING_BYTE, &args![0x0109_6b68u32, size])
            .u32();
        append_viewer_string(e, strings, argument, text);
        let grow_by = u32::from(e.mem.u8(ARRAY_GROW_BY));
        let text = e
            .call(VIEWER_STRING_BYTE, &args![0x0109_6b54u32, grow_by])
            .u32();
        append_viewer_string(e, strings, argument, text);
        e.mem.set_u8(argument, byte(e, this, FLAGS) & 1);
        let word = e.mem.u32(argument);
        let text = e
            .call(VIEWER_STRING_BOOL, &args![0x0109_6b40u32, word])
            .u32();
        append_viewer_string(e, strings, argument, text);
        let threshold = x87_quiet_bits(e.mem.u32(at(WEIGHT_THRESHOLD)));
        let text = e
            .call(VIEWER_STRING_FLOAT, &args![0x0109_6b2cu32, threshold])
            .u32();
        append_viewer_string(e, strings, argument, text);
        let high = i32::from(byte(e, this, HIGH_PRIORITY) as i8);
        let text = e
            .call(VIEWER_STRING_SIGNED, &args![0x0109_6b1cu32, high])
            .u32();
        append_viewer_string(e, strings, argument, text);
        let next = i32::from(byte(e, this, NEXT_HIGH_PRIORITY) as i8);
        let text = e
            .call(VIEWER_STRING_SIGNED, &args![0x0109_6b08u32, next])
            .u32();
        append_viewer_string(e, strings, argument, text);
        let high_sum = x87_quiet_bits(e.mem.u32(at(HIGH_SUM_OF_WEIGHTS)));
        let text = e
            .call(VIEWER_STRING_FLOAT, &args![0x0109_6af4u32, high_sum])
            .u32();
        append_viewer_string(e, strings, argument, text);
        let next_sum = x87_quiet_bits(e.mem.u32(at(NEXT_HIGH_SUM_OF_WEIGHTS)));
        let text = e
            .call(VIEWER_STRING_FLOAT, &args![0x0109_6adcu32, next_sum])
            .u32();
        append_viewer_string(e, strings, argument, text);
        e.mem.set_u8(argument, (byte(e, this, FLAGS) >> 1) & 1);
        let word = e.mem.u32(argument);
        let text = e
            .call(VIEWER_STRING_BOOL, &args![0x0109_6ac4u32, word])
            .u32();
        append_viewer_string(e, strings, argument, text);
        let count = u32::from(byte(e, this, INTERP_COUNT));
        let text = e
            .call(VIEWER_STRING_BYTE, &args![0x0109_6ab4u32, count])
            .u32();
        append_viewer_string(e, strings, argument, text);
        let single = u32::from(byte(e, this, SINGLE_INDEX));
        let text = e
            .call(VIEWER_STRING_BYTE, &args![0x0109_6aa4u32, single])
            .u32();
        append_viewer_string(e, strings, argument, text);
        e.mem.set_u32(argument, 0);
        if byte(e, this, ARRAY_SIZE) == 0 {
            return;
        }
        let mut offset = 0u32;
        loop {
            let entry = |e: &Engine| e.mem.u32(at(INTERP_ARRAY)).wrapping_add(offset);
            let interpolator = e.mem.u32(entry(e));
            let first = e.mem.u32(SEQUENCE_HOLDER_FIRST);
            if interpolator != 0 && first != 0 {
                let mut sequence = e
                    .call(SEQUENCE_LOOKUP, &args![first, interpolator, 1u32])
                    .u32();
                if sequence == 0 {
                    let second = e.mem.u32(SEQUENCE_HOLDER_SECOND);
                    if second != 0 {
                        let interpolator = e.mem.u32(entry(e));
                        sequence = e
                            .call(SEQUENCE_LOOKUP, &args![second, interpolator, 1u32])
                            .u32();
                    }
                }
                if sequence != 0 {
                    let name = e.mem.u32(sequence.wrapping_add(8));
                    let text = e
                        .call(VIEWER_STRING_TEXT, &args![0x0109_5dbcu32, name])
                        .u32();
                    append_viewer_string(e, strings, local, text);
                    let priority = u32::from(e.mem.u8(entry(e) + ITEM_PRIORITY));
                    let text = e
                        .call(VIEWER_STRING_BYTE, &args![0x0109_6a98u32, priority])
                        .u32();
                    append_viewer_string(e, strings, local, text);
                    for (name, field) in [
                        (0x0109_6a8cu32, ITEM_WEIGHT),
                        (0x0109_6a78, ITEM_NORMALIZED_WEIGHT),
                        (0x0109_6a68, ITEM_EASE_SPINNER),
                    ] {
                        let value = x87_quiet_bits(e.mem.u32(entry(e) + field));
                        let text = e.call(VIEWER_STRING_FLOAT, &args![name, value]).u32();
                        append_viewer_string(e, strings, local, text);
                    }
                    let interpolator = e.mem.u32(entry(e));
                    e.vcall(interpolator, SLOT_GET_VIEWER_STRINGS, &args![strings]);
                }
            }
            let counter = e.mem.u32(argument).wrapping_add(1);
            offset = offset.wrapping_add(ITEM_SIZE);
            e.mem.set_u32(argument, counter);
            if counter >= u32::from(byte(e, this, ARRAY_SIZE)) {
                break;
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a36960, ni_blend_interpolator_get_rtti() -> Ptr),
        entry!(0x00a37670, ni_blend_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a370a0, fn_00a370a0(Ptr) -> Ptr),
        entry!(0x00a371f0, fn_00a371f0(Ptr, Ptr) -> Ptr),
        entry!(0x00a370b0, ni_blend_interpolator_ni_blend_interpolator(Ptr, u8, f32, u8) -> Ptr),
        entry!(0x00a37170, ni_blend_interpolator_destructor(Ptr)),
        entry!(0x00a368f0, ni_blend_interpolator_get_highest_weighted_index(Ptr) -> u8),
        entry!(
            0x00a36970,
            ni_blend_interpolator_add_interp_info(Ptr, Ptr, f32, i8, f32) -> u8
        ),
        entry!(
            0x00a36a90,
            ni_blend_interpolator_remove_interp_info(Ptr, u8)
        ),
        entry!(0x00a36bd0, fn_00a36bd0(Ptr)),
        entry!(
            0x00a37260,
            ni_blend_interpolator_compute_normalized_weights(Ptr)
        ),
        entry!(0x00a36e80, ni_blend_interpolator_process_clone(Ptr, Ptr)),
        entry!(0x00a37510, fn_00a37510(Ptr, Ptr, Ptr)),
        entry!(0x00a37690, ni_blend_interpolator_reallocate_arrays(Ptr) -> bool),
        entry!(0x00a37980, ni_blend_interpolator_load_binary(Ptr, Ptr)),
        entry!(0x00a3_6dc0, ni_blend_interpolator_collapse(Ptr)),
        entry!(
            0x00a3_6e10,
            ni_blend_interpolator_guarantee_time_range(Ptr, f32, f32)
        ),
        entry!(0x00a3_6f10, fn_00a36f10(Ptr, Ptr) -> bool),
        entry!(
            0x00a3_6f80,
            ni_blend_interpolator_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a3_6fd0, ni_blend_interpolator_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a3_77b0,
            ni_blend_interpolator_interp_array_item_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a3_78d0, fn_00a378d0(Ptr, Ptr)),
        entry!(0x00a3_7ec0, ni_blend_interpolator_save_binary(Ptr, Ptr)),
        entry!(
            0x00a3_8080,
            ni_blend_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_blend_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a36960, &args![]).u32(),
            NI_BLEND_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_blend_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a3_7170, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a37670, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a37670, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x30]]);
        assert_eq!(calls_to(&e, 0x00a3_7170), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b014_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A vtable for counted test objects: slot +4 (delete) logs `this`,
    /// +0x48 (`CreateClone`) answers `this + 0x100`, +0x68 (`ProcessClone`)
    /// and +0xE8 (`ReallocateArrays`, answers false) are doubles too.
    const VTABLE: u32 = 0x7000_0000;
    const DELETE: u32 = 0x7200_0004;
    const CREATE_CLONE: u32 = 0x7200_0048;
    const CHILD_PROCESS_CLONE: u32 = 0x7200_0068;
    const REALLOCATE: u32 = 0x7200_00e8;

    type Log = Rc<RefCell<Vec<u32>>>;

    /// The engine every test uses: counting interlocked imports, the
    /// memory manager's allocate/free and the vector constructor iterator
    /// as doubles (the iterator runs `00a370a0` on every element), the
    /// constants mapped, and the test vtable.
    fn engine() -> (Engine, Log) {
        let mut e = engine_with_counting();
        e.map(0x0109_6000, 0x1000);
        e.mem.set_f32(INVALID_FLOAT, f32::MIN);
        e.map(0x0102_4000, 0x1000);
        e.set_global(LOWEST, -3.4028234663852886e38f64);
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f32(MINUS_ONE, -1.0);
        e.map(0x011a_8000, 0x1000);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |e, a| {
            for i in 0..a[2] {
                e.call(0x00a3_70a0, &args![a[0] + i * a[1]]);
            }
            Ret::default()
        });
        let mut slots = vec![0u32; 0xec / 4];
        slots[1] = DELETE;
        slots[0x48 / 4] = CREATE_CLONE;
        slots[0x68 / 4] = CHILD_PROCESS_CLONE;
        slots[0xe8 / 4] = REALLOCATE;
        e.put_vtable(VTABLE, &slots);
        let deleted: Log = Rc::default();
        let log = deleted.clone();
        e.register_double(DELETE, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        e.register(CREATE_CLONE, |_, a| (a[0] + 0x100).into_ret());
        e.register(CHILD_PROCESS_CLONE, |_, _| Ret::default());
        e.register(REALLOCATE, |_, _| false.into_ret());
        (e, deleted)
    }

    /// A counted object (count `count`) with the test vtable.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object, VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// A blend interpolator with an item array of `size` items (count word
    /// in front), the test vtable.
    fn blend(e: &mut Engine, size: u8) -> Ptr {
        let this = Ptr::new(e.mem.alloc(0x30));
        e.mem.set_u32(this.addr(), VTABLE);
        let block = e.mem.alloc(4 + ITEM_SIZE * u32::from(size));
        e.mem.set_u32(block, u32::from(size));
        e.mem.set_u32(this.addr() + INTERP_ARRAY, block + 4);
        set_byte(e, this, ARRAY_SIZE, size);
        set_byte(e, this, HIGH_PRIORITY, 0x80);
        set_byte(e, this, NEXT_HIGH_PRIORITY, 0x80);
        set_byte(e, this, SINGLE_INDEX, 0xff);
        this
    }

    /// Fills item `index`.
    fn put_item(
        e: &mut Engine,
        this: Ptr,
        index: u32,
        interp: u32,
        weight: f32,
        priority: i8,
        ease: f32,
    ) {
        let at = item(e, this, index);
        e.mem.set_u32(at, interp);
        e.mem.set_f32(at + ITEM_WEIGHT, weight);
        e.mem.set_u8(at + ITEM_PRIORITY, priority as u8);
        e.mem.set_f32(at + ITEM_EASE_SPINNER, ease);
    }

    fn normalized(e: &Engine, this: Ptr, index: u32) -> f32 {
        e.mem.f32(item(e, this, index) + ITEM_NORMALIZED_WEIGHT)
    }

    #[test]
    fn item_constructor_clears_the_interpolator() {
        let (mut e, _) = engine();
        let at = e.mem.alloc(ITEM_SIZE);
        e.mem.set_u32(at, 0x1234);
        assert_eq!(e.call(0x00a370a0, &args![at]).u32(), at);
        assert_eq!(e.mem.u32(at), 0);
    }

    #[test]
    fn item_assignment_counts_the_interpolator_and_copies_the_rest() {
        let (mut e, deleted) = engine();
        let old = counted(&mut e, 1);
        let new = counted(&mut e, 1);
        let to = e.mem.alloc(ITEM_SIZE);
        let from = e.mem.alloc(ITEM_SIZE);
        e.mem.set_u32(to, old);
        e.mem.set_u32(from, new);
        for (offset, value) in [(4u32, 1.5f32), (8, 2.5), (0x10, 3.5), (0x14, 4.5)] {
            e.mem.set_f32(from + offset, value);
        }
        e.mem.set_u8(from + 0xc, 7);
        assert_eq!(e.call(0x00a371f0, &args![to, from]).u32(), to);
        assert_eq!(e.mem.u32(to), new);
        assert_eq!(e.mem.u32(new + 4), 2);
        assert_eq!(*deleted.borrow(), [old]);
        assert_eq!(e.mem.f32(to + 0x14), 4.5);
        assert_eq!(e.mem.u8(to + 0xc), 7);
    }

    #[test]
    fn constructor_sets_the_defaults_and_allocates_the_array() {
        for size in [0u8, 3] {
            let (mut e, _) = engine();
            e.register(NI_INTERPOLATOR_CONSTRUCTOR, |_, _| Ret::default());
            let this = Ptr::new(e.mem.alloc(0x30));
            e.call_log = Some(vec![]);
            let r = e
                .call(0x00a370b0, &args![this, 5u32, 0.25f32, u32::from(size)])
                .u32();
            assert_eq!(r, this.addr());
            assert_eq!(e.mem.u32(this.addr()), NI_BLEND_INTERPOLATOR_VTABLE);
            assert_eq!(e.mem.f32(this.addr() + WEIGHT_THRESHOLD), 0.25);
            for offset in [
                SINGLE_TIME,
                HIGH_SUM_OF_WEIGHTS,
                NEXT_HIGH_SUM_OF_WEIGHTS,
                HIGH_EASE_SPINNER,
            ] {
                assert_eq!(e.mem.f32(this.addr() + offset), f32::MIN);
            }
            assert_eq!(byte(&e, this, FLAGS), 1);
            assert_eq!(byte(&e, this, ARRAY_SIZE), size);
            assert_eq!(byte(&e, this, SINGLE_INDEX), 0xff);
            assert_eq!(byte(&e, this, HIGH_PRIORITY), 0x80);
            let array = e.mem.u32(this.addr() + INTERP_ARRAY);
            if size == 0 {
                assert_eq!(array, 0);
                assert!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
            } else {
                assert_eq!(e.mem.u32(array - 4), 3);
                assert_eq!(
                    calls_to(&e, MEMORY_MANAGER_ALLOCATE),
                    vec![vec![MEMORY_MANAGER, 3 * 0x18 + 4]]
                );
                assert_eq!(
                    calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
                    vec![vec![array, 0x18, 3, 0x00a3_70a0]]
                );
            }
        }
    }

    #[test]
    fn destructor_releases_every_item_and_frees_the_array() {
        let (mut e, deleted) = engine();
        e.register(NI_INTERPOLATOR_DESTRUCTOR, |_, _| Ret::default());
        let this = blend(&mut e, 3);
        let kept = counted(&mut e, 2);
        let dropped = counted(&mut e, 1);
        put_item(&mut e, this, 0, kept, 1.0, 0, 1.0);
        put_item(&mut e, this, 2, dropped, 1.0, 0, 1.0);
        let array = e.mem.u32(this.addr() + INTERP_ARRAY);
        e.call_log = Some(vec![]);
        e.call(0x00a37170, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NI_BLEND_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(kept + 4), 1);
        assert_eq!(*deleted.borrow(), [dropped]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, array - 4]]
        );
        assert_eq!(
            calls_to(&e, NI_INTERPOLATOR_DESTRUCTOR),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn highest_weighted_index() {
        let (mut e, _) = engine();
        let this = blend(&mut e, 3);
        set_byte(&mut e, this, INTERP_COUNT, 1);
        set_byte(&mut e, this, SINGLE_INDEX, 2);
        assert_eq!(e.call(0x00a368f0, &args![this]).u8(), 2);
        set_byte(&mut e, this, INTERP_COUNT, 3);
        assert_eq!(e.call(0x00a368f0, &args![this]).u8(), 0xff);
        for (i, w) in [0.25f32, 0.5, 0.5].iter().enumerate() {
            e.mem
                .set_f32(item(&e, this, i as u32) + ITEM_NORMALIZED_WEIGHT, *w);
        }
        assert_eq!(e.call(0x00a368f0, &args![this]).u8(), 1);
    }

    #[test]
    fn add_interp_info_fills_the_first_free_item() {
        let (mut e, _) = engine();
        let this = blend(&mut e, 2);
        let first = counted(&mut e, 0);
        let second = counted(&mut e, 0);
        let r = e
            .call(0x00a36970, &args![this, first, 0.5f32, 3u32, 0.75f32])
            .u8();
        assert_eq!(r, 0);
        assert_eq!(e.mem.u32(first + 4), 1);
        let at = item(&e, this, 0);
        assert_eq!(e.mem.f32(at + ITEM_WEIGHT), 0.5);
        assert_eq!(e.mem.f32(at + ITEM_EASE_SPINNER), 0.75);
        assert_eq!(e.mem.f32(at + ITEM_UPDATE_TIME), f32::MIN);
        assert_eq!(byte(&e, this, HIGH_PRIORITY), 3);
        assert_eq!(byte(&e, this, NEXT_HIGH_PRIORITY), 0x80);
        assert_eq!(byte(&e, this, INTERP_COUNT), 1);
        assert_eq!(byte(&e, this, SINGLE_INDEX), 0);
        assert_eq!(e.mem.u32(this.addr() + SINGLE_INTERPOLATOR), first);
        assert_eq!(byte(&e, this, FLAGS) & 4, 4);

        let r = e
            .call(0x00a36970, &args![this, second, 1.0f32, 5u32, 1.0f32])
            .u8();
        assert_eq!(r, 1);
        assert_eq!(byte(&e, this, HIGH_PRIORITY), 5);
        assert_eq!(byte(&e, this, NEXT_HIGH_PRIORITY), 3);
        assert_eq!(byte(&e, this, SINGLE_INDEX), 0xff);
        assert_eq!(e.mem.u32(this.addr() + SINGLE_INTERPOLATOR), 0);
        assert_eq!(e.mem.f32(this.addr() + SINGLE_TIME), f32::MIN);

        // Full and the reallocation refuses: 0xFF, nothing changes.
        let r = e
            .call(0x00a36970, &args![this, second, 1.0f32, 5u32, 1.0f32])
            .u8();
        assert_eq!(r, 0xff);
        assert_eq!(byte(&e, this, INTERP_COUNT), 2);
    }

    #[test]
    fn remove_interp_info_recomputes_priorities_and_the_single_item() {
        let (mut e, deleted) = engine();
        let this = blend(&mut e, 3);
        let a = counted(&mut e, 1);
        let b = counted(&mut e, 2);
        put_item(&mut e, this, 0, a, 1.0, 5, 1.0);
        put_item(&mut e, this, 2, b, 1.0, 2, 1.0);
        e.mem.set_f32(item(&e, this, 2) + ITEM_UPDATE_TIME, 9.0);
        set_byte(&mut e, this, HIGH_PRIORITY, 5);
        set_byte(&mut e, this, NEXT_HIGH_PRIORITY, 2);
        set_byte(&mut e, this, INTERP_COUNT, 2);
        e.call(0x00a36a90, &args![this, 0u32]);
        assert_eq!(*deleted.borrow(), [a]);
        assert_eq!(e.mem.u32(item(&e, this, 0)), 0);
        assert_eq!(byte(&e, this, HIGH_PRIORITY), 2);
        assert_eq!(byte(&e, this, NEXT_HIGH_PRIORITY), 0x80);
        assert_eq!(byte(&e, this, INTERP_COUNT), 1);
        assert_eq!(byte(&e, this, SINGLE_INDEX), 2);
        assert_eq!(e.mem.u32(this.addr() + SINGLE_INTERPOLATOR), b);
        assert_eq!(e.mem.f32(this.addr() + SINGLE_TIME), 9.0);
        // The last one: nothing left.
        e.call(0x00a36a90, &args![this, 2u32]);
        assert_eq!(byte(&e, this, INTERP_COUNT), 0);
        assert_eq!(byte(&e, this, SINGLE_INDEX), 0xff);
        assert_eq!(byte(&e, this, HIGH_PRIORITY), 0x80);
        assert_eq!(e.mem.u32(this.addr() + SINGLE_INTERPOLATOR), 0);
        assert_eq!(e.mem.f32(this.addr() + SINGLE_TIME), f32::MIN);
        assert_eq!(e.mem.f32(this.addr() + HIGH_SUM_OF_WEIGHTS), f32::MIN);
    }

    #[test]
    fn remove_from_two_items_takes_the_other_index() {
        let (mut e, _) = engine();
        let this = blend(&mut e, 2);
        let a = counted(&mut e, 2);
        let b = counted(&mut e, 2);
        put_item(&mut e, this, 0, a, 1.0, 0, 1.0);
        put_item(&mut e, this, 1, b, 1.0, 0, 1.0);
        set_byte(&mut e, this, INTERP_COUNT, 2);
        e.call(0x00a36a90, &args![this, 1u32]);
        assert_eq!(byte(&e, this, SINGLE_INDEX), 0);
        assert_eq!(e.mem.u32(this.addr() + SINGLE_INTERPOLATOR), a);
    }

    fn two(e: &mut Engine, (wa, pa, ea): (f32, i8, f32), (wb, pb, eb): (f32, i8, f32)) -> Ptr {
        let this = blend(e, 3);
        put_item(e, this, 0, 1, wa, pa, ea);
        put_item(e, this, 2, 1, wb, pb, eb);
        this
    }

    #[test]
    fn two_items_by_priority() {
        let (mut e, _) = engine();
        // Equal priorities: proportional.
        let this = two(&mut e, (1.0, 1, 1.0), (3.0, 1, 1.0));
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.25, 0.75)
        );
        // A higher with spinner 1: (1, 0); B higher with spinner 1: (0, 1).
        let this = two(&mut e, (1.0, 2, 1.0), (3.0, 1, 1.0));
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (1.0, 0.0)
        );
        let this = two(&mut e, (1.0, 1, 1.0), (3.0, 2, 1.0));
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.0, 1.0)
        );
        // A higher with spinner 0.5: 0.25 against 0.5.
        let this = two(&mut e, (1.0, 2, 0.5), (1.0, 1, 1.0));
        e.call(0x00a36bd0, &args![this]);
        let (a, b) = (normalized(&e, this, 0), normalized(&e, this, 2));
        assert!(
            (a - 1.0 / 3.0).abs() < 1e-6 && (b - 2.0 / 3.0).abs() < 1e-6,
            "{a} {b}"
        );
        // B higher with spinner 0.5: A 1 * 0.5 against B (1 * 0.5) * 0.5.
        let this = two(&mut e, (1.0, 1, 1.0), (1.0, 2, 0.5));
        e.call(0x00a36bd0, &args![this]);
        let (a, b) = (normalized(&e, this, 0), normalized(&e, this, 2));
        assert!(
            (a - 2.0 / 3.0).abs() < 1e-6 && (b - 1.0 / 3.0).abs() < 1e-6,
            "{a} {b}"
        );
        // Both zero.
        let this = two(&mut e, (0.0, 1, 1.0), (0.0, 1, 1.0));
        e.mem
            .set_f32(item(&e, this, 0) + ITEM_NORMALIZED_WEIGHT, 7.0);
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.0, 0.0)
        );
    }

    #[test]
    fn two_items_highest_only_and_threshold() {
        let (mut e, _) = engine();
        let this = two(&mut e, (1.0, 1, 1.0), (3.0, 1, 1.0));
        set_byte(&mut e, this, FLAGS, ONLY_USE_HIGHEST_WEIGHT);
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.0, 1.0)
        );
        let this = two(&mut e, (1.0, 1, 1.0), (1.0, 1, 1.0));
        set_byte(&mut e, this, FLAGS, ONLY_USE_HIGHEST_WEIGHT);
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (1.0, 0.0)
        );
        // Threshold 0.3: A's 0.25 is dropped and B becomes 1.
        let this = two(&mut e, (1.0, 1, 1.0), (3.0, 1, 1.0));
        e.mem.set_f32(this.addr() + WEIGHT_THRESHOLD, 0.3);
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.0, 1.0)
        );
        // Threshold 0.8: B's 0.75 also: B is zeroed, then set to 1.
        let this = two(&mut e, (1.0, 1, 1.0), (3.0, 1, 1.0));
        e.mem.set_f32(this.addr() + WEIGHT_THRESHOLD, 0.8);
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.0, 1.0)
        );
        // Threshold 0.1: nothing changes.
        let this = two(&mut e, (1.0, 1, 1.0), (3.0, 1, 1.0));
        e.mem.set_f32(this.addr() + WEIGHT_THRESHOLD, 0.1);
        e.call(0x00a36bd0, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.25, 0.75)
        );
    }

    fn three(e: &mut Engine) -> Ptr {
        let this = blend(e, 4);
        put_item(e, this, 0, 1, 1.0, 4, 1.0);
        put_item(e, this, 1, 1, 1.0, 4, 1.0);
        put_item(e, this, 3, 1, 2.0, 4, 1.0);
        set_byte(e, this, HIGH_PRIORITY, 4);
        set_byte(e, this, INTERP_COUNT, 3);
        set_byte(e, this, FLAGS, COMPUTE_NORMALIZED_WEIGHTS);
        for offset in [
            HIGH_SUM_OF_WEIGHTS,
            NEXT_HIGH_SUM_OF_WEIGHTS,
            HIGH_EASE_SPINNER,
        ] {
            e.mem.set_f32(this.addr() + offset, f32::MIN);
        }
        this
    }

    #[test]
    fn compute_normalized_weights_for_many() {
        let (mut e, _) = engine();
        let this = three(&mut e);
        e.call(0x00a37260, &args![this]);
        assert_eq!(byte(&e, this, FLAGS) & 4, 0);
        assert_eq!(e.mem.f32(this.addr() + HIGH_SUM_OF_WEIGHTS), 4.0);
        assert_eq!(e.mem.f32(this.addr() + HIGH_EASE_SPINNER), 1.0);
        let weights: Vec<f32> = [0, 1, 3].iter().map(|i| normalized(&e, this, *i)).collect();
        assert_eq!(weights, [0.25, 0.25, 0.5]);
        // Not out of date: nothing happens.
        e.mem
            .set_f32(item(&e, this, 0) + ITEM_NORMALIZED_WEIGHT, 9.0);
        e.call(0x00a37260, &args![this]);
        assert_eq!(normalized(&e, this, 0), 9.0);

        // A threshold of 0.3 drops the quarters; the half is scaled to 1.
        let this = three(&mut e);
        e.mem.set_f32(this.addr() + WEIGHT_THRESHOLD, 0.3);
        e.call(0x00a37260, &args![this]);
        let weights: Vec<f32> = [0, 1, 3].iter().map(|i| normalized(&e, this, *i)).collect();
        assert_eq!(weights, [0.0, 0.0, 1.0]);

        // Only the highest weight.
        let this = three(&mut e);
        set_byte(
            &mut e,
            this,
            FLAGS,
            COMPUTE_NORMALIZED_WEIGHTS | ONLY_USE_HIGHEST_WEIGHT,
        );
        e.call(0x00a37260, &args![this]);
        let weights: Vec<f32> = [0, 1, 2, 3]
            .iter()
            .map(|i| normalized(&e, this, *i))
            .collect();
        assert_eq!(weights, [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn compute_normalized_weights_for_one_or_two() {
        let (mut e, _) = engine();
        let this = blend(&mut e, 2);
        set_byte(&mut e, this, INTERP_COUNT, 1);
        set_byte(&mut e, this, SINGLE_INDEX, 1);
        set_byte(&mut e, this, FLAGS, COMPUTE_NORMALIZED_WEIGHTS);
        e.call(0x00a37260, &args![this]);
        assert_eq!(normalized(&e, this, 1), 1.0);
        let this = two(&mut e, (1.0, 1, 1.0), (3.0, 1, 1.0));
        set_byte(&mut e, this, INTERP_COUNT, 2);
        set_byte(&mut e, this, FLAGS, COMPUTE_NORMALIZED_WEIGHTS);
        e.call(0x00a37260, &args![this]);
        assert_eq!(
            (normalized(&e, this, 0), normalized(&e, this, 2)),
            (0.25, 0.75)
        );
    }

    #[test]
    fn process_clone_maps_the_single_interpolator() {
        const MAP: u32 = 0x7300_0000;
        for found in [true, false] {
            let (mut e, _) = engine();
            e.register(NI_OBJECT_PROCESS_CLONE, |_, _| Ret::default());
            let this = blend(&mut e, 2);
            let child = counted(&mut e, 1);
            put_item(&mut e, this, 1, child, 1.0, 0, 1.0);
            e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, child);
            let clone = e.mem.alloc(0x30);
            e.map(MAP, 0x10);
            e.mem.set_u32(MAP + 4, clone);
            e.mem.set_u32(MAP + 8, u32::from(found));
            e.register(CLONE_MAP_LOOKUP, |e, a| {
                // The object itself maps to the clone; its child to child + 1.
                if a[1] == e.mem.u32(MAP + 0xc) {
                    e.mem.set_u32(a[2], e.mem.u32(MAP + 4));
                    true.into_ret()
                } else {
                    e.mem.set_u32(a[2], a[1] + 1);
                    (e.mem.u32(MAP + 8) != 0).into_ret()
                }
            });
            e.mem.set_u32(MAP + 0xc, this.addr());
            let cloning = e.mem.alloc(4);
            e.mem.set_u32(cloning, MAP);
            e.call_log = Some(vec![]);
            e.call(0x00a36e80, &args![this, cloning]);
            assert_eq!(
                calls_to(&e, CHILD_PROCESS_CLONE),
                vec![vec![child, cloning]]
            );
            let expected = if found { child + 1 } else { child };
            assert_eq!(e.mem.u32(clone + SINGLE_INTERPOLATOR), expected);
        }
    }

    #[test]
    fn copy_members_clones_the_items_unless_manager_controlled() {
        for managed in [false, true] {
            let (mut e, _) = engine();
            e.register(KEY_BASED_COPY_MEMBERS, |_, _| Ret::default());
            let this = blend(&mut e, 2);
            let child = counted(&mut e, 1);
            put_item(&mut e, this, 0, child, 0.5, 3, 0.75);
            set_byte(&mut e, this, FLAGS, u8::from(managed));
            set_byte(&mut e, this, INTERP_COUNT, 1);
            set_byte(&mut e, this, SINGLE_INDEX, 0);
            set_byte(&mut e, this, HIGH_PRIORITY, 3);
            e.mem.set_f32(this.addr() + WEIGHT_THRESHOLD, 0.125);
            e.mem.set_f32(this.addr() + SINGLE_TIME, 6.0);
            let destination = Ptr::new(e.mem.alloc(0x30));
            e.call(0x00a37510, &args![this, destination, 0x5000u32]);
            assert_eq!(byte(&e, destination, ARRAY_SIZE), 2);
            assert_eq!(byte(&e, destination, FLAGS), u8::from(managed));
            assert_eq!(e.mem.f32(destination.addr() + WEIGHT_THRESHOLD), 0.125);
            let copied = item(&e, destination, 0);
            if managed {
                assert_eq!(e.mem.u32(copied), 0);
                assert_eq!(byte(&e, destination, INTERP_COUNT), 0);
            } else {
                assert_eq!(e.mem.u32(copied), child + 0x100);
                assert_eq!(e.mem.f32(copied + ITEM_EASE_SPINNER), 0.75);
                assert_eq!(e.mem.u8(copied + ITEM_PRIORITY), 3);
                assert_eq!(byte(&e, destination, INTERP_COUNT), 1);
                assert_eq!(byte(&e, destination, HIGH_PRIORITY), 3);
                assert_eq!(e.mem.f32(destination.addr() + SINGLE_TIME), 6.0);
            }
        }
    }

    #[test]
    fn reallocate_arrays_grows_by_the_step_to_at_least_4() {
        let (mut e, _) = engine();
        let this = blend(&mut e, 1);
        let child = counted(&mut e, 1);
        put_item(&mut e, this, 0, child, 0.5, 3, 0.75);
        let old = e.mem.u32(this.addr() + INTERP_ARRAY);
        assert!(!e.call(0x00a37690, &args![this]).bool());
        e.mem.set_u8(ARRAY_GROW_BY, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a37690, &args![this]).bool());
        assert_eq!(byte(&e, this, ARRAY_SIZE), 4);
        let new = e.mem.u32(this.addr() + INTERP_ARRAY);
        assert_eq!(e.mem.u32(new - 4), 4);
        assert_eq!(e.mem.u32(new), child);
        assert_eq!(e.mem.f32(new + ITEM_EASE_SPINNER), 0.75);
        assert_eq!(e.mem.u32(child + 4), 1);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, old - 4]]
        );
        // A bigger array grows by the step.
        set_byte(&mut e, this, ARRAY_SIZE, 4);
        e.mem.set_u8(ARRAY_GROW_BY, 3);
        e.call(0x00a37690, &args![this]);
        assert_eq!(byte(&e, this, ARRAY_SIZE), 7);
    }

    /// A stream whose reads answer successive byte strings, logging
    /// (size, component size) of each.
    fn stream(
        e: &mut Engine,
        version: u32,
        chunks: Vec<Vec<u8>>,
    ) -> (u32, Rc<RefCell<Vec<(u32, u32)>>>) {
        const FAKE_READ: u32 = 0x7100_0000;
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, FAKE_READ);
        e.mem.set_u32(stream + 0xd8, version);
        e.mem.set_u32(stream + 0x24c, input);
        let log: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        let seen = log.clone();
        let mut next = chunks.into_iter();
        e.register_double(FAKE_READ, move |e, a| {
            seen.borrow_mut().push((a[2], e.mem.u32(a[3])));
            let chunk = next.next().unwrap();
            assert_eq!(chunk.len() as u32, a[2]);
            e.mem.write(a[1], &chunk);
            (chunk.len() as u32).into_ret()
        });
        (stream, log)
    }

    #[test]
    fn load_binary_current_format() {
        let (mut e, _) = engine();
        e.register(NI_INTERPOLATOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(ITEM_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_RESOLVE_LINK_ID, |_, _| 0x4444u32.into_ret());
        let f = |v: f32| v.to_le_bytes().to_vec();
        let (stream, log) = stream(
            &mut e,
            0x0a01_0070,
            vec![
                vec![0x06],
                vec![2],
                f(0.5),
                vec![1],
                vec![0],
                vec![7],
                vec![6],
                f(1.5),
                f(2.5),
                f(3.5),
                f(4.5),
            ],
        );
        let this = Ptr::new(e.mem.alloc(0x30));
        e.call_log = Some(vec![]);
        e.call(0x00a37980, &args![this, stream]);
        assert_eq!(byte(&e, this, FLAGS), 2);
        assert_eq!(byte(&e, this, ARRAY_SIZE), 2);
        assert_eq!(e.mem.f32(this.addr() + WEIGHT_THRESHOLD), 0.5);
        assert_eq!(byte(&e, this, INTERP_COUNT), 1);
        assert_eq!(byte(&e, this, HIGH_PRIORITY), 7);
        assert_eq!(byte(&e, this, NEXT_HIGH_PRIORITY), 6);
        assert_eq!(e.mem.f32(this.addr() + HIGH_EASE_SPINNER), 4.5);
        assert_eq!(e.mem.u32(this.addr() + SINGLE_INTERPOLATOR), 0x4444);
        let array = e.mem.u32(this.addr() + INTERP_ARRAY);
        assert_eq!(
            calls_to(&e, ITEM_LOAD_BINARY),
            vec![vec![array, stream], vec![array + 0x18, stream]]
        );
        let sizes: Vec<u32> = log.borrow().iter().map(|r| r.0).collect();
        assert_eq!(sizes, [1, 1, 4, 1, 1, 1, 1, 4, 4, 4, 4]);
        assert!(log.borrow().iter().all(|r| r.0 == r.1));
    }

    #[test]
    fn load_binary_old_format() {
        let (mut e, _) = engine();
        e.register(NI_INTERPOLATOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(ITEM_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_RESOLVE_LINK_ID, |_, _| 0x4444u32.into_ret());
        let f = |v: f32| v.to_le_bytes().to_vec();
        let (stream, log) = stream(
            &mut e,
            0x0a01_006c,
            vec![
                vec![1, 0],
                vec![9, 9],
                vec![1],
                f(0.25),
                vec![0],
                vec![1, 0],
                vec![0, 0],
                f(8.0),
                vec![0, 0, 0, 0x80],
                vec![5, 0, 0, 0],
            ],
        );
        let this = Ptr::new(e.mem.alloc(0x30));
        set_byte(&mut e, this, FLAGS, ONLY_USE_HIGHEST_WEIGHT);
        e.call(0x00a37980, &args![this, stream]);
        assert_eq!(byte(&e, this, ARRAY_SIZE), 1);
        assert_eq!(byte(&e, this, FLAGS), MANAGER_CONTROLLED);
        assert_eq!(e.mem.f32(this.addr() + WEIGHT_THRESHOLD), 0.25);
        assert_eq!(byte(&e, this, INTERP_COUNT), 1);
        assert_eq!(byte(&e, this, SINGLE_INDEX), 0);
        assert_eq!(e.mem.u32(this.addr() + SINGLE_INTERPOLATOR), 0x4444);
        assert_eq!(e.mem.f32(this.addr() + SINGLE_TIME), 8.0);
        assert_eq!(byte(&e, this, HIGH_PRIORITY), 0x80);
        assert_eq!(byte(&e, this, NEXT_HIGH_PRIORITY), 5);
        let sizes: Vec<(u32, u32)> = log.borrow().clone();
        assert_eq!(
            sizes,
            [
                (2, 2),
                (2, 2),
                (1, 1),
                (4, 4),
                (1, 1),
                (2, 2),
                (2, 2),
                (4, 4),
                (4, 4),
                (4, 4)
            ]
        );
    }
}

/// Test doubles the animation units' newer tests share: test streams, the
/// viewer-string array and its makers, and the call log filter.
#[cfg(test)]
pub(crate) mod a07_test_support {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    /// Bytes or words recorded by a double.
    pub(crate) type Bytes = Rc<RefCell<Vec<u8>>>;
    pub(crate) type Words = Rc<RefCell<Vec<u32>>>;

    /// The read and write functions of the test streams.
    const READ: u32 = 0x7310_0000;
    const WRITE: u32 = 0x7310_0010;

    /// The argument words of every logged call to `addr`.
    pub(crate) fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .map(|log| {
                log.iter()
                    .filter(|(a, _)| *a == addr)
                    .map(|(_, words)| words.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// A test `NiStream` of file version `version`: its input reads the
    /// bytes of `input` in order and its output appends what is written to
    /// the returned bytes; both check that the component-size word holds the
    /// size and answer the size.
    pub(crate) fn stream(e: &mut Engine, version: u32, input: &[u8]) -> (Ptr, Bytes) {
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_FILE_VERSION, version);
        let istr = e.mem.alloc(0x10);
        let ostr = e.mem.alloc(0x10);
        e.mem.set_u32(istr + 8, READ);
        e.mem.set_u32(ostr + 0xc, WRITE);
        e.mem.set_u32(stream + STREAM_INPUT, istr);
        e.mem.set_u32(stream + STREAM_OUTPUT, ostr);
        let queue: Rc<RefCell<VecDeque<u8>>> =
            Rc::new(RefCell::new(input.iter().copied().collect()));
        e.register_double(READ, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2], "component size");
            for i in 0..a[2] {
                let value = queue.borrow_mut().pop_front().expect("stream input");
                e.mem.set_u8(a[1] + i, value);
            }
            a[2].into_ret()
        });
        let written: Bytes = Rc::default();
        let log = written.clone();
        e.register_double(WRITE, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2], "component size");
            for i in 0..a[2] {
                log.borrow_mut().push(e.mem.u8(a[1] + i));
            }
            a[2].into_ret()
        });
        (Ptr::new(stream), written)
    }

    /// A viewer-string array (growth step 4) whose store records each stored
    /// string word; every viewer-string maker answers its first word (the
    /// name's address, or the class name for `NiGetViewerString`).
    pub(crate) fn viewer_strings(e: &mut Engine) -> (Ptr, Words) {
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 4);
        for maker in [
            VIEWER_STRING,
            VIEWER_STRING_BYTE,
            VIEWER_STRING_BOOL,
            VIEWER_STRING_FLOAT,
            VIEWER_STRING_SIGNED,
            VIEWER_STRING_TEXT,
        ] {
            e.register(maker, |_, a| a[0].into_ret());
        }
        e.register(STRINGS_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        let stored: Words = Rc::default();
        let log = stored.clone();
        e.register_double(STRINGS_SET_AT, move |e, a| {
            log.borrow_mut().push(e.mem.u32(a[2]));
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        (Ptr::new(strings), stored)
    }
}

#[cfg(test)]
mod a07_tests {
    use super::a07_test_support::{calls, stream, viewer_strings};
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::engine_with_counting;

    /// A vtable at 0x7400_0000 whose slots are doubles at `0x7500_0000 +
    /// slot`: `IsEqual` answers whether the other object's word +8 equals its
    /// own, the rest answer 1.
    const CHILD_VTABLE: u32 = 0x7400_0000;
    fn child_slot(slot: u32) -> u32 {
        0x7500_0000 + slot
    }

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        let slots: Vec<u32> = (0..0x40).map(|i| child_slot(4 * i)).collect();
        e.put_vtable(CHILD_VTABLE, &slots);
        for i in 0..0x40 {
            e.register(child_slot(4 * i), |_, _| 1u32.into_ret());
        }
        e.register(child_slot(SLOT_IS_EQUAL), |e, a| {
            (e.mem.u32(a[0] + 8) == e.mem.u32(a[1] + 8)).into_ret()
        });
        e.map(0x011a_8000, 0x1000);
        e.map(0x011f_3000, 0x1000);
        e
    }

    fn child(e: &mut Engine, key: u32) -> u32 {
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, CHILD_VTABLE);
        e.mem.set_u32(object + 4, 1);
        e.mem.set_u32(object + 8, key);
        object
    }

    /// A blend interpolator with `size` items.
    fn blend(e: &mut Engine, size: u8) -> Ptr {
        let this = Ptr::new(e.mem.alloc(0x30));
        let array = e.mem.alloc(4 + ITEM_SIZE * u32::from(size));
        e.mem.set_u32(this.addr() + INTERP_ARRAY, array + 4);
        set_byte(e, this, ARRAY_SIZE, size);
        this
    }

    #[test]
    fn collapse_runs_the_single_or_every_interpolator() {
        let mut e = engine();
        let this = blend(&mut e, 3);
        let single = child(&mut e, 1);
        set_byte(&mut e, this, INTERP_COUNT, 1);
        e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, single);
        e.call_log = Some(vec![]);
        e.call(0x00a3_6dc0, &args![this]);
        assert_eq!(calls(&e, child_slot(SLOT_COLLAPSE)), vec![vec![single]]);
        let (a, b) = (child(&mut e, 2), child(&mut e, 3));
        e.mem.set_u32(item(&e, this, 0), a);
        e.mem.set_u32(item(&e, this, 2), b);
        set_byte(&mut e, this, INTERP_COUNT, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a3_6dc0, &args![this]);
        assert_eq!(calls(&e, child_slot(SLOT_COLLAPSE)), vec![vec![a], vec![b]]);
    }

    #[test]
    fn guarantee_time_range_passes_quieted_times_on() {
        let mut e = engine();
        let this = blend(&mut e, 2);
        let single = child(&mut e, 1);
        set_byte(&mut e, this, INTERP_COUNT, 1);
        e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, single);
        e.call_log = Some(vec![]);
        e.call(0x00a3_6e10, &args![this, 0x7fa0_0000u32, 2.0f32]);
        let slot = child_slot(SLOT_GUARANTEE_TIME_RANGE);
        assert_eq!(
            calls(&e, slot),
            vec![vec![single, 0x7fe0_0000, 2.0f32.to_bits()]]
        );
        let a = child(&mut e, 2);
        e.mem.set_u32(item(&e, this, 1), a);
        set_byte(&mut e, this, INTERP_COUNT, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a3_6e10, &args![this, 1.0f32, 3.0f32]);
        assert_eq!(
            calls(&e, slot),
            vec![vec![a, 1.0f32.to_bits(), 3.0f32.to_bits()]]
        );
    }

    fn put(e: &mut Engine, at: u32, interpolator: u32, values: [f32; 3], priority: u8) {
        e.mem.set_u32(at, interpolator);
        e.mem.set_f32(at + ITEM_WEIGHT, values[0]);
        e.mem.set_f32(at + ITEM_NORMALIZED_WEIGHT, values[1]);
        e.mem.set_u8(at + ITEM_PRIORITY, priority);
        e.mem.set_f32(at + ITEM_EASE_SPINNER, values[2]);
    }

    #[test]
    fn item_equality_compares_interpolators_and_fields() {
        let mut e = engine();
        let (a, b) = (e.mem.alloc(ITEM_SIZE), e.mem.alloc(ITEM_SIZE));
        put(&mut e, a, 0, [1.0, 0.5, 1.0], 3);
        put(&mut e, b, 0, [1.0, 0.5, 1.0], 3);
        let equal = |e: &mut Engine| e.call(0x00a3_6f10, &args![a, b]).bool();
        assert!(equal(&mut e));
        let (x, y, z) = (child(&mut e, 7), child(&mut e, 7), child(&mut e, 8));
        e.mem.set_u32(a, x);
        assert!(!equal(&mut e), "only one interpolator");
        e.mem.set_u32(b, y);
        assert!(equal(&mut e), "equal interpolators");
        e.mem.set_u32(b, z);
        assert!(!equal(&mut e), "IsEqual false");
        e.mem.set_u32(b, y);
        e.mem.set_u8(b + ITEM_PRIORITY, 4);
        assert!(!equal(&mut e), "priority");
        e.mem.set_u8(b + ITEM_PRIORITY, 3);
        e.mem.set_u32(a + ITEM_EASE_SPINNER, f32::NAN.to_bits());
        e.mem.set_u32(b + ITEM_EASE_SPINNER, f32::NAN.to_bits());
        assert!(!equal(&mut e), "a NaN is never equal");
    }

    #[test]
    fn register_streamables_skips_items_when_manager_controlled() {
        let mut e = engine();
        e.register(0x00a8_1fe0, |e, a| (e.mem.u32(a[0] + 0x2c) == 0).into_ret());
        let this = blend(&mut e, 2);
        let a = child(&mut e, 1);
        e.mem.set_u32(item(&e, this, 1), a);
        let stream = 0x1234u32;
        let slot = child_slot(SLOT_REGISTER_STREAMABLES);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a3_6f80, &args![this, stream]).bool());
        assert_eq!(calls(&e, slot), vec![vec![a, stream]]);
        set_byte(&mut e, this, FLAGS, MANAGER_CONTROLLED);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a3_6f80, &args![this, stream]).bool());
        assert!(calls(&e, slot).is_empty());
        e.mem.set_u32(this.addr() + 0x2c, 1);
        assert!(
            !e.call(0x00a3_6f80, &args![this, stream]).bool(),
            "base false"
        );
    }

    #[test]
    fn is_equal_needs_equal_items_fields_and_the_same_single_interpolator() {
        let mut e = engine();
        e.register(0x00ba_87d0, |_, _| 1u32.into_ret());
        let (this, other) = (blend(&mut e, 1), blend(&mut e, 1));
        let single = child(&mut e, 5);
        for p in [this, other] {
            let at = item(&e, p, 0);
            put(&mut e, at, 0, [1.0, 1.0, 1.0], 2);
            e.mem.set_u32(p.addr() + SINGLE_INTERPOLATOR, single);
            e.mem.set_f32(p.addr() + SINGLE_TIME, 4.0);
            e.mem.set_f32(p.addr() + WEIGHT_THRESHOLD, 0.5);
            set_byte(&mut e, p, HIGH_PRIORITY, 2);
        }
        let equal = |e: &mut Engine| e.call(0x00a3_6fd0, &args![this, other]).bool();
        assert!(equal(&mut e));
        let twin = child(&mut e, 5);
        e.mem.set_u32(other.addr() + SINGLE_INTERPOLATOR, twin);
        assert!(!equal(&mut e), "IsEqual is not enough: the same object");
        e.mem.set_u32(other.addr() + SINGLE_INTERPOLATOR, single);
        set_byte(&mut e, other, NEXT_HIGH_PRIORITY, 1);
        assert!(!equal(&mut e), "priority");
        set_byte(&mut e, other, NEXT_HIGH_PRIORITY, 0);
        let weight = item(&e, other, 0) + ITEM_WEIGHT;
        e.mem.set_f32(weight, 2.0);
        assert!(!equal(&mut e), "item");
        e.mem.set_f32(weight, 1.0);
        set_byte(&mut e, other, ARRAY_SIZE, 2);
        assert!(!equal(&mut e), "array size");
        set_byte(&mut e, other, ARRAY_SIZE, 1);
        e.register(0x00ba_87d0, |_, _| 0u32.into_ret());
        assert!(!equal(&mut e), "base");
    }

    #[test]
    fn item_load_binary_reads_old_and_new_priorities() {
        let mut e = engine();
        e.register(STREAM_RESOLVE_LINK_ID, |_, _| 0u32.into_ret());
        let mut bytes = vec![];
        bytes.extend(2.0f32.to_le_bytes());
        bytes.extend(0.5f32.to_le_bytes());
        bytes.extend(0x8000_0000u32.to_le_bytes());
        bytes.extend(1.5f32.to_le_bytes());
        let (old, _) = stream(&mut e, 0x0a01_006d, &bytes);
        let at = e.mem.alloc(ITEM_SIZE);
        e.call(0x00a3_77b0, &args![at, old]);
        assert_eq!(e.mem.f32(at + ITEM_WEIGHT), 2.0);
        assert_eq!(e.mem.f32(at + ITEM_NORMALIZED_WEIGHT), 0.5);
        assert_eq!(e.mem.u8(at + ITEM_PRIORITY), 0x80);
        assert_eq!(e.mem.f32(at + ITEM_EASE_SPINNER), 1.5);
        let mut bytes = vec![];
        bytes.extend(1.0f32.to_le_bytes());
        bytes.extend(1.0f32.to_le_bytes());
        bytes.push(7);
        bytes.extend(1.0f32.to_le_bytes());
        let link = child(&mut e, 1);
        e.register_double(STREAM_RESOLVE_LINK_ID, move |_, _| link.into_ret());
        let (new, _) = stream(&mut e, 0x0a01_006e, &bytes);
        e.call(0x00a3_77b0, &args![at, new]);
        assert_eq!(e.mem.u8(at + ITEM_PRIORITY), 7);
        assert_eq!(e.mem.u32(at), link);
        assert_eq!(e.mem.u32(link + 4), 2, "counted");
    }

    #[test]
    fn item_save_binary_saves_the_link_and_the_fields() {
        let mut e = engine();
        let (s, written) = stream(&mut e, 0x1400_0000, &[]);
        e.mem.set_u32(s.addr(), CHILD_VTABLE);
        let at = e.mem.alloc(ITEM_SIZE);
        put(&mut e, at, 0x4444, [1.0, 0.25, 2.0], 9);
        e.call_log = Some(vec![]);
        e.call(0x00a3_78d0, &args![at, s]);
        let save_link = child_slot(STREAM_SLOT_SAVE_LINK_ID);
        assert_eq!(calls(&e, save_link), vec![vec![s.addr(), 0x4444]]);
        let mut expected = vec![];
        expected.extend(1.0f32.to_le_bytes());
        expected.extend(0.25f32.to_le_bytes());
        expected.push(9);
        expected.extend(2.0f32.to_le_bytes());
        assert_eq!(*written.borrow(), expected);
    }

    #[test]
    fn save_binary_writes_everything_unless_manager_controlled() {
        let mut e = engine();
        e.register(0x00a7_24d0, |_, _| Ret::default());
        let (s, written) = stream(&mut e, 0x1400_0000, &[]);
        e.mem.set_u32(s.addr(), CHILD_VTABLE);
        let this = blend(&mut e, 1);
        set_byte(&mut e, this, FLAGS, MANAGER_CONTROLLED);
        e.mem.set_f32(this.addr() + WEIGHT_THRESHOLD, 0.5);
        e.call(0x00a3_7ec0, &args![this, s]);
        let mut expected = vec![MANAGER_CONTROLLED, 1];
        expected.extend(0.5f32.to_le_bytes());
        assert_eq!(*written.borrow(), expected);
        written.borrow_mut().clear();
        set_byte(&mut e, this, FLAGS, 0);
        set_byte(&mut e, this, INTERP_COUNT, 1);
        e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x00a3_7ec0, &args![this, s]);
        // Flags, size, threshold, four bytes, four floats, one item (13).
        assert_eq!(written.borrow().len(), 2 + 4 + 4 + 16 + 13);
        let links = calls(&e, child_slot(STREAM_SLOT_SAVE_LINK_ID));
        assert_eq!(links, vec![vec![s.addr(), 0], vec![s.addr(), 0x5555]]);
    }

    #[test]
    fn get_viewer_strings_lists_the_members_and_the_sequenced_items() {
        let mut e = engine();
        e.register(NI_INTERPOLATOR_GET_VIEWER_STRINGS, |_, _| Ret::default());
        let (strings, stored) = viewer_strings(&mut e);
        e.mem.set_u32(NI_BLEND_INTERPOLATOR_RTTI, 0x0109_6000);
        let this = blend(&mut e, 2);
        // Flags: manager controlled and "only the highest weight".
        set_byte(&mut e, this, FLAGS, 3);
        set_byte(&mut e, this, HIGH_PRIORITY, 0xfe);
        let sequence = e.mem.alloc(0xc);
        e.mem.set_u32(sequence + 8, 0x0abc);
        let a = child(&mut e, 1);
        let at = item(&e, this, 1);
        put(&mut e, at, a, [1.0, 0.5, 0.25], 6);
        e.mem.set_u32(SEQUENCE_HOLDER_FIRST, 0x99);
        e.mem.set_u32(SEQUENCE_HOLDER_SECOND, 0x98);
        // The first holder knows no sequence, the second does.
        e.register_double(SEQUENCE_LOOKUP, move |_, a| {
            let found = if a[0] == 0x98 { sequence } else { 0 };
            found.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a3_8080, &args![this, strings]);
        let names = stored.borrow().clone();
        assert_eq!(
            names,
            [
                0x0109_6000,
                0x0109_6b68,
                0x0109_6b54,
                0x0109_6b40,
                0x0109_6b2c,
                0x0109_6b1c,
                0x0109_6b08,
                0x0109_6af4,
                0x0109_6adc,
                0x0109_6ac4,
                0x0109_6ab4,
                0x0109_6aa4,
                0x0109_5dbc,
                0x0109_6a98,
                0x0109_6a8c,
                0x0109_6a78,
                0x0109_6a68,
            ]
        );
        let bools = calls(&e, VIEWER_STRING_BOOL);
        // The low byte is the flag; the upper bytes are the previous string's.
        assert_eq!(bools[0][1], (0x0109_6b54 & !0xff) | 1);
        assert_eq!(bools[1][1], (0x0109_6adc & !0xff) | 1);
        assert_eq!(calls(&e, VIEWER_STRING_SIGNED)[0][1], (-2i32) as u32);
        assert_eq!(
            calls(&e, VIEWER_STRING_TEXT),
            vec![vec![0x0109_5dbc, 0x0abc]]
        );
        assert_eq!(
            calls(&e, child_slot(SLOT_GET_VIEWER_STRINGS)),
            vec![vec![a, strings.addr()]]
        );
        assert_eq!(
            calls(&e, SEQUENCE_LOOKUP),
            vec![vec![0x99, a, 1], vec![0x98, a, 1]]
        );
    }
}
