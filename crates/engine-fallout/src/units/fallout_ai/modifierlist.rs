//! `fallout/ai/modifierlist.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_ai::run_00936ef0::ModifierList;

// Translated from 009373f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this`. The engine map has no name for it; the quest stage
/// item's `InitItem` (`0060fa10`) calls it.
pub fn fn_009373f0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr())
}

// Translated from 00937b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `009370b0` (Ghidra name
/// `ModifierList::DeleteAllModifiers`), which deletes every item of the list and the
/// fast-modifier collection at `+0xc`. The engine map names this address
/// `ModifierList::Revert` from the call graph only (row `cg`), which this translation does
/// not confirm, and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `009370b0`'s disassembly ends on
/// the store `MOV dword ptr [ECX+0xc],0x0` with no result (Ghidra types it void).
pub fn fn_00937b30(e: &mut Engine, this: Ptr) {
    e.call(0x0093_70b0, &args![this]);
}

/// `ModifierList::DeleteAllModifiers` (`009370b0`; `thiscall`, no stack words).
const DELETE_ALL_MODIFIERS: u32 = 0x0093_70b0;
/// `00401000` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE: u32 = 0x0040_1000;
/// `00937e10` (`thiscall`, `RET 4`, unit `modifierlist.cpp`, unnamed): constructs, in the
/// block it is given, a copy of the collection at its argument (the `pFastModifiers` word of
/// a `ModifierList`): a pointer array sized by the word at `0x011e0750`, each used slot a
/// copy made with `00936fd0`. Returns `this`.
const FAST_MODIFIERS_COPY: u32 = 0x0093_7e10;
/// `006815c0` (`thiscall`, plain `RET`): returns its `this` (the address of the list node's
/// item word).
const NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
/// `00936fd0` (`thiscall`, `RET 4`, unit `fallout/ai` run, translated in `run_00936ef0.rs`):
/// constructs, in the block it is given, a copy of the modifier at its argument.
const MODIFIER_COPY: u32 = 0x0093_6fd0;
/// `00937380` (`thiscall`, `RET 4`, unit `modifierlist.cpp`, unnamed): takes a modifier
/// into the list.
const ADD_MODIFIER: u32 = 0x0093_7380;
/// `00726070` (`thiscall`, plain `RET`): returns the word at `+4` of its `this` (the next
/// node of a `BSSimpleList`).
const NEXT_NODE: u32 = 0x0072_6070;

// Translated from 00937160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00937160 - the C++ exception unwinding states (FS:[0] frame, handler 00f177b6) are not translated
/// Unnamed in the engine map (unit `modifierlist.cpp`; `thiscall`, `RET 4`): makes the
/// `ModifierList` `this` a copy of `source` (a `ModifierList`, Xbox PDB layout: a
/// `BSSimpleList<Modifier *>` head, `pFastModifiers` at `+0xc`).
///
/// - Deletes everything `this` holds first (`009370b0`).
/// - A null `source` ends there.
/// - When `source`'s `pFastModifiers` is set, allocates 4 bytes (`00401000`) and, when the
///   block came back, builds the copy in it with `00937e10(block, source->pFastModifiers)`;
///   the result (0 for a failed allocation) becomes `this`'s `pFastModifiers`.
/// - Then walks the list nodes from `source` itself: the node's item word (the address
///   `006815c0` returns, so the node's first word) must be non-zero, otherwise the walk ends.
///   For each node, allocates 8 bytes, copies the item into it with `00936fd0` (0 for a failed
///   allocation) and hands the copy to `00937380` on `this`; the next node is `00726070`.
pub fn fn_00937160(e: &mut Engine, this: Ptr<ModifierList>, source: Ptr<ModifierList>) {
    e.call(DELETE_ALL_MODIFIERS, &args![this]);
    if source.addr() == 0 {
        return;
    }
    if e.get(source, ModifierList::pFastModifiers) != 0 {
        let block = e.call(ALLOCATE, &args![4u32]).u32();
        let collection = if block != 0 {
            let fast_modifiers = e.get(source, ModifierList::pFastModifiers);
            e.call(FAST_MODIFIERS_COPY, &args![block, fast_modifiers])
                .u32()
        } else {
            0
        };
        e.set(this, ModifierList::pFastModifiers, collection);
    }
    let mut node = source.addr();
    while node != 0 {
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        if e.mem.u32(item_address) == 0 {
            break;
        }
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let item = e.mem.u32(item_address);
        let block = e.call(ALLOCATE, &args![8u32]).u32();
        let modifier = if block != 0 {
            e.call(MODIFIER_COPY, &args![block, item]).u32()
        } else {
            0
        };
        e.call(ADD_MODIFIER, &args![this, modifier]);
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
}

/// `00401030` (`cdecl`, one word): frees a block.
const FREE: u32 = 0x0040_1030;
/// `0063f7b0` (`thiscall`): drops the head item of a `BSSimpleList` (the
/// second node's item moves into the head, the node is deleted; a list
/// without a second node gets a null head item).
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// `005ae3d0` (`thiscall`, `RET 4`): adds the item at the given address to a
/// `BSSimpleList`.
const LIST_ADD: u32 = 0x005a_e3d0;
/// `00905330` (`thiscall`, `RET 4`): removes the item at the given address
/// from a `BSSimpleList`.
const LIST_REMOVE: u32 = 0x0090_5330;
/// `00938060` (`thiscall`, `RET 4`, unit `modifierlist.cpp`): frees and
/// clears the fast collection's slot of an actor value.
const FAST_SLOT_REMOVE: u32 = 0x0093_8060;
/// `006b9130` (`thiscall`): a modifier's `float` value (+4), in ST0.
const MODIFIER_VALUE: u32 = 0x006b_9130;
/// `004f5d90` (`thiscall`, `RET 4`): stores a modifier's `float` value (+4).
const MODIFIER_SET_VALUE: u32 = 0x004f_5d90;
/// `0066e920` (`cdecl`, one word): the actor value's info record (0 outside
/// `0..0x4d`).
const ACTOR_VALUE_INFO: u32 = 0x0066_e920;
/// `00937fd0` (no arguments, unit `modifierlist.cpp`): builds the slot table
/// at `011e0618` and the slot count at `011e0750` once.
const BUILD_FAST_SLOTS: u32 = 0x0093_7fd0;
/// `BGSSaveGameBuffer::StartVariableSizedValue` (engine map), `00865f20`.
const SAVE_START_VARIABLE_SIZED: u32 = 0x0086_5f20;
/// `00865e50` (`thiscall`, `RET 0xc`): writes `size` bytes from an address
/// to the save buffer (third word 0 here).
const SAVE_WRITE: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveVariableSizedValue_ov2` (engine map), `00865ff0`.
const SAVE_END_VARIABLE_SIZED: u32 = 0x0086_5ff0;
/// The fast-modifier slot of each actor value (`int[0x4d]`, 0x4d for a value
/// without a slot), built by `00937fd0`.
const FAST_SLOT_INDEX: u32 = 0x011e_0618;
/// The number of fast-modifier slots (`011e0750`).
const FAST_SLOT_COUNT: u32 = 0x011e_0750;
/// `double` 0.0 (`01012060`).
const ZERO_DOUBLE: u32 = 0x0101_2060;

/// A `ModifierList` field or slot read: `ModifierList` +0x0C is
/// `pFastModifiers` (Xbox PDB), whose first word is the slot array.
fn fast_slots(e: &Engine, this: Ptr<ModifierList>) -> u32 {
    let collection = e.get(this, ModifierList::pFastModifiers);
    e.mem.u32(collection)
}

/// The slot of actor value `kind` (`FAST_SLOT_INDEX[kind]`, signed index).
fn fast_slot_index(e: &Engine, kind: i8) -> u32 {
    e.mem
        .u32(FAST_SLOT_INDEX.wrapping_add((i32::from(kind) as u32).wrapping_mul(4)))
}

/// `count * 4` as the exe's `MUL`/`SETO`/`NEG`/`OR` computes it: all ones
/// when the product overflows.
fn array_bytes(count: u32) -> u32 {
    let product = u64::from(count) * 4;
    if product >> 32 != 0 {
        0xffff_ffff
    } else {
        product as u32
    }
}

// Translated from 009370b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModifierList::DeleteAllModifiers` (engine map name): frees each item of
/// the list (`00401030`) and drops it from the head (`0063f7b0`) until the
/// head item is null; then, when `pFastModifiers` is set, deletes that
/// collection ([`fn_00937130`] with flag 1) and clears the pointer.
pub fn modifier_list_delete_all_modifiers(e: &mut Engine, this: Ptr<ModifierList>) {
    loop {
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![this]).u32();
        let item = e.mem.u32(item_address);
        if item == 0 {
            break;
        }
        e.call(FREE, &args![item]);
        e.call(LIST_REMOVE_HEAD, &args![this]);
    }
    if e.get(this, ModifierList::pFastModifiers) != 0 {
        let collection = e.get(this, ModifierList::pFastModifiers);
        if collection != 0 {
            fn_00937130(e, Ptr::new(collection), 1);
        }
        e.set(this, ModifierList::pFastModifiers, 0);
    }
}

// Translated from 00937130 (decompiled, FalloutNV.exe 1.4.0.525)
/// The fast-modifier collection's deleting destructor (no Xbox PDB name):
/// [`fn_00937f40`], then, with bit 0 of `flags`, the collection's own block
/// freed (`00401030`). Returns `this`.
pub fn fn_00937130(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00937f40(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00937280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00937280 - the C++ exception unwinding states (FS:[0] frame, handler 00eed64b) are not translated
/// Puts `modifier` into the fast slot of actor value `kind` (no Xbox PDB
/// name): with neither a modifier nor a collection nothing happens; an
/// existing collection's slot is freed first (`00938060`). A modifier whose
/// value is not 0 (or NaN), or any modifier when the list allows zero
/// values (+8), is stored, the collection (4 bytes, [`fn_00937d70`]; null
/// when the allocation fails) made first when missing.
pub fn fn_00937280(e: &mut Engine, this: Ptr<ModifierList>, kind: i8, modifier: u32) {
    if modifier == 0 && e.get(this, ModifierList::pFastModifiers) == 0 {
        return;
    }
    if e.get(this, ModifierList::pFastModifiers) != 0 {
        let collection = e.get(this, ModifierList::pFastModifiers);
        e.call(FAST_SLOT_REMOVE, &args![collection, i32::from(kind)]);
    }
    if modifier == 0 {
        return;
    }
    let value = e.call(MODIFIER_VALUE, &args![modifier]).f64();
    // FCOMP 0.0; TEST AH,0x44; JP: not equal (or unordered) stores.
    let zero = e.mem.f64(ZERO_DOUBLE);
    if value == zero && e.get(this, ModifierList::bZeroValuesAllowed) == 0 {
        return;
    }
    if e.get(this, ModifierList::pFastModifiers) == 0 {
        let block = e.call(ALLOCATE, &args![4u32]).u32();
        let collection = if block == 0 {
            0
        } else {
            fn_00937d70(e, Ptr::new(block)).addr()
        };
        e.set(this, ModifierList::pFastModifiers, collection);
    }
    let slots = fast_slots(e, this);
    let index = fast_slot_index(e, kind);
    e.mem
        .set_u32(slots.wrapping_add(index.wrapping_mul(4)), modifier);
}

// Translated from 00937380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes `modifier` into the list (no Xbox PDB name): nothing for a null
/// one; an actor value whose info record (`0066e920`) has bit 0 of +0x44
/// set goes to its fast slot ([`fn_00937280`]), any other is added to the
/// list (`005ae3d0`, given the address of the argument).
pub fn fn_00937380(e: &mut Engine, this: Ptr<ModifierList>, modifier: u32) {
    if modifier == 0 {
        return;
    }
    let kind = fn_009373f0(e, Ptr::new(modifier)) as i8;
    let info = e.call(ACTOR_VALUE_INFO, &args![i32::from(kind)]).u32();
    // ActorValueInfo +0x44: flags; bit 0 marks a value kept in a fast slot.
    if info != 0 && e.mem.u32(info + 0x44) & 1 != 0 {
        fn_00937280(e, this, kind, modifier);
    } else {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), modifier);
            e.call(LIST_ADD, &args![this, slot]);
        });
    }
}

// Translated from 00937400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `modifier` (no Xbox PDB name): nothing for a null one; a
/// fast-slot actor value with a collection has its slot cleared
/// (`00938060`); otherwise the modifier is removed from the list
/// (`00905330`, given the address of the argument) and the argument word,
/// as it stands after that call, is freed (`00401030`).
pub fn fn_00937400(e: &mut Engine, this: Ptr<ModifierList>, modifier: u32) {
    if modifier == 0 {
        return;
    }
    let kind = fn_009373f0(e, Ptr::new(modifier)) as i8;
    let info = e.call(ACTOR_VALUE_INFO, &args![i32::from(kind)]).u32();
    if info != 0
        && e.mem.u32(info + 0x44) & 1 != 0
        && e.get(this, ModifierList::pFastModifiers) != 0
    {
        let kind = fn_009373f0(e, Ptr::new(modifier)) as i8;
        let collection = e.get(this, ModifierList::pFastModifiers);
        e.call(FAST_SLOT_REMOVE, &args![collection, i32::from(kind)]);
    } else {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), modifier);
            e.call(LIST_REMOVE, &args![this, slot]);
            let item = e.mem.u32(slot.addr());
            e.call(FREE, &args![item]);
        });
    }
}

/// Allocates an 8-byte modifier and constructs it ([`fn_009375b0`]) with
/// `kind` and the value bits; 0 when the allocation fails.
fn new_modifier(e: &mut Engine, kind: u8, value_bits: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![8u32]).u32();
    if block == 0 {
        0
    } else {
        fn_009375b0(e, Ptr::new(block), kind, f32::from_bits(value_bits)).addr()
    }
}

// Translated from 00937480 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00937480 - the C++ exception unwinding states (FS:[0] frame, handler 00f0b23b) are not translated
/// `ModifierList::AdjustModifier` (engine map name): without a modifier for
/// `kind`, a `delta` other than 0 (or NaN), or any delta when the list
/// allows zero values, makes a new modifier ([`fn_009375b0`], 8 bytes;
/// null when the allocation fails) taken in by [`fn_00937380`]. With one,
/// its value becomes [`fn_00937810`]`(value, delta, mode)`, and a modifier
/// whose value is then 0 is removed ([`fn_00937400`]) unless the list
/// allows zero values.
pub fn modifier_list_adjust_modifier(
    e: &mut Engine,
    this: Ptr<ModifierList>,
    kind: u8,
    delta: f32,
    mode: u32,
) {
    let delta_bits = x87_store32(x87_load32(delta.to_bits()));
    let zero = e.mem.f64(ZERO_DOUBLE);
    let modifier = fn_00937760(e, this, kind as i8);
    if modifier == 0 {
        // FCOMP 0.0; TEST AH,0x44; JP: not equal (or unordered) adds.
        if x87_load32(delta.to_bits()) != zero || e.get(this, ModifierList::bZeroValuesAllowed) != 0
        {
            let made = new_modifier(e, kind, delta_bits);
            fn_00937380(e, this, made);
        }
        return;
    }
    let value = e.call(MODIFIER_VALUE, &args![modifier]).f64();
    let adjusted = fn_00937810(
        e,
        f32::from_bits(x87_store32(value)),
        f32::from_bits(delta_bits),
        mode,
    );
    let stored = x87_store32(x87_load32(x87_store32(adjusted)));
    e.call(MODIFIER_SET_VALUE, &args![modifier, stored]);
    let value = e.call(MODIFIER_VALUE, &args![modifier]).f64();
    if value == zero && e.get(this, ModifierList::bZeroValuesAllowed) == 0 {
        fn_00937400(e, this, modifier);
    }
}

// Translated from 009375b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The modifier constructor (no Xbox PDB name): the actor value byte at +0,
/// the value stored by `004f5d90`. Returns `this`.
pub fn fn_009375b0(e: &mut Engine, this: Ptr, kind: u8, value: f32) -> Ptr {
    e.mem.set_u8(this.addr(), kind);
    let bits = x87_store32(x87_load32(value.to_bits()));
    e.call(MODIFIER_SET_VALUE, &args![this, bits]);
    this
}

// Translated from 009375e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009375e0 - the C++ exception unwinding states (FS:[0] frame, handler 00eeb0fb) are not translated
/// Sets the modifier of `kind` to `value` (no Xbox PDB name): without one, a
/// value other than 0 (or NaN), or any value when the list allows zero
/// values, makes a new modifier taken in by [`fn_00937380`]; with one, a
/// value of 0 removes it ([`fn_00937400`]) unless zero values are allowed,
/// and any other value is stored (`004f5d90`).
pub fn fn_009375e0(e: &mut Engine, this: Ptr<ModifierList>, kind: u8, value: f32) {
    let value_bits = x87_store32(x87_load32(value.to_bits()));
    let zero = e.mem.f64(ZERO_DOUBLE);
    let is_zero = x87_load32(value.to_bits()) == zero;
    let modifier = fn_00937760(e, this, kind as i8);
    if modifier == 0 {
        if !is_zero || e.get(this, ModifierList::bZeroValuesAllowed) != 0 {
            let made = new_modifier(e, kind, value_bits);
            fn_00937380(e, this, made);
        }
    } else if is_zero && e.get(this, ModifierList::bZeroValuesAllowed) == 0 {
        fn_00937400(e, this, modifier);
    } else {
        e.call(MODIFIER_SET_VALUE, &args![modifier, value_bits]);
    }
}

// Translated from 009376e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModifierList::GetModifierItem` (engine map name): the value of the
/// modifier of `kind` (stored to a `float` local), 0 without one; `*found`
/// says whether there was one. In ST0.
pub fn modifier_list_get_modifier_item(
    e: &mut Engine,
    this: Ptr<ModifierList>,
    kind: u8,
    found: Ptr,
) -> f64 {
    let modifier = fn_00937760(e, this, kind as i8);
    e.mem.set_u8(found.addr(), (modifier != 0) as u8);
    let bits = if modifier != 0 {
        x87_store32(e.call(MODIFIER_VALUE, &args![modifier]).f64())
    } else {
        0
    };
    x87_load32(bits)
}

// Translated from 00937730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModifierList::GetModifier` (engine map name): [`modifier_list_get_modifier_item`]
/// with a local `found` byte (cleared first); its ST0 result is this
/// function's.
pub fn modifier_list_get_modifier(e: &mut Engine, this: Ptr<ModifierList>, kind: u8) -> f64 {
    e.with_stack(4, |e, found| {
        e.mem.set_u8(found.addr(), 0);
        modifier_list_get_modifier_item(e, this, kind, found)
    })
}

// Translated from 00937760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the modifier of actor value `kind` (no Xbox PDB name): with a fast
/// collection and a slot below 0x4d, the slot's word; otherwise the first
/// list item whose actor value byte ([`fn_009373f0`]) is `kind`, the walk
/// ending at a null head item or once one is found. 0 when none.
pub fn fn_00937760(e: &mut Engine, this: Ptr<ModifierList>, kind: i8) -> u32 {
    let mut found = 0u32;
    if e.get(this, ModifierList::pFastModifiers) != 0 && (fast_slot_index(e, kind) as i32) < 0x4d {
        let slots = fast_slots(e, this);
        let index = fast_slot_index(e, kind);
        return e.mem.u32(slots.wrapping_add(index.wrapping_mul(4)));
    }
    let mut node = this.addr();
    while node != 0 {
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        if e.mem.u32(item_address) == 0 || found != 0 {
            break;
        }
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let item = e.mem.u32(item_address);
        if item != 0 && fn_009373f0(e, Ptr::new(item)) as i8 == kind {
            found = item;
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    found
}

// Translated from 00937810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `value + delta` stored to a `float` (`cdecl`; no Xbox PDB name), then
/// clamped by `mode`: 0 keeps it at most 0 (a positive sum becomes 0), 1
/// at least 0 (a negative sum becomes 0), any other mode leaves it; NaN
/// is kept. In ST0.
pub fn fn_00937810(_e: &mut Engine, value: f32, delta: f32, mode: u32) -> f64 {
    let mut sum = x87_store32(x87_add(
        x87_load32(value.to_bits()),
        x87_m32(delta.to_bits()),
    ));
    match mode {
        0 => {
            // FCOMP 0.0; TEST AH,0x41; JNZ: only "above" clears.
            if x87_load32(sum) > 0.0 {
                sum = x87_store32(0.0);
            }
        }
        1 => {
            // FCOMP 0.0; TEST AH,0x5; JP: only "below" clears.
            if x87_load32(sum) < 0.0 {
                sum = x87_store32(0.0);
            }
        }
        _ => {}
    }
    x87_load32(sum)
}

// Translated from 00937b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModifierList::SaveGame_ov2` (engine map name): in a variable-sized
/// value of the save buffer (`00865f20` ... `00865ff0` with the count),
/// each modifier of the list and then of the fast slots (`011e0750` of
/// them) as its actor value byte and its 4-byte value (`00865e50`).
pub fn modifier_list_save_game_ov2(e: &mut Engine, this: Ptr<ModifierList>, buffer: Ptr) {
    let mut count = 0u32;
    let token = e.call(SAVE_START_VARIABLE_SIZED, &args![buffer]).u32();
    let mut node = this.addr();
    while node != 0 {
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let item = e.mem.u32(item_address);
        if item != 0 {
            e.call(SAVE_WRITE, &args![buffer, item, 1u32, 0u32]);
            e.call(SAVE_WRITE, &args![buffer, item + 4, 4u32, 0u32]);
            count = count.wrapping_add(1);
        }
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
    if e.get(this, ModifierList::pFastModifiers) != 0 {
        let mut i = 0u32;
        while i < e.mem.u32(FAST_SLOT_COUNT) {
            let slot = |e: &Engine| {
                e.mem
                    .u32(fast_slots(e, this).wrapping_add(i.wrapping_mul(4)))
            };
            if slot(e) != 0 {
                let item = slot(e);
                e.call(SAVE_WRITE, &args![buffer, item, 1u32, 0u32]);
                let item = slot(e);
                e.call(SAVE_WRITE, &args![buffer, item.wrapping_add(4), 4u32, 0u32]);
                count = count.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    }
    e.call(SAVE_END_VARIABLE_SIZED, &args![buffer, count, token]);
}

// Translated from 00937d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The fast-modifier collection constructor (no Xbox PDB name): builds the
/// slot table (`00937fd0`), then with no slots the array pointer is 0;
/// otherwise an array of `011e0750` words is allocated (`00401000`; size
/// all ones on overflow) and every word cleared (no null check, as in the
/// exe). Returns `this`.
pub fn fn_00937d70(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BUILD_FAST_SLOTS, &args![]);
    if e.mem.u32(FAST_SLOT_COUNT) == 0 {
        e.mem.set_u32(this.addr(), 0);
        return this;
    }
    let count = e.mem.u32(FAST_SLOT_COUNT);
    let block = if count == 0 {
        0
    } else {
        e.call(ALLOCATE, &args![array_bytes(count)]).u32()
    };
    e.mem.set_u32(this.addr(), block);
    let mut i = 0i32;
    while i < e.mem.u32(FAST_SLOT_COUNT) as i32 {
        let array = e.mem.u32(this.addr());
        e.mem
            .set_u32(array.wrapping_add((i as u32).wrapping_mul(4)), 0);
        i += 1;
    }
    this
}

// Translated from 00937e10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00937e10 - the C++ exception unwinding states (FS:[0] frame, handler 00f0814b) are not translated
/// The fast-modifier collection's copy constructor (no Xbox PDB name):
/// a source without an array gives a null array; otherwise a new array of
/// `011e0750` words (null for no slots) whose used slots hold copies of the
/// source's modifiers (8 bytes each, `00936fd0`; null when the allocation
/// fails) and the rest 0. Returns `this`.
pub fn fn_00937e10(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    if e.mem.u32(other.addr()) == 0 {
        e.mem.set_u32(this.addr(), 0);
        return this;
    }
    let count = e.mem.u32(FAST_SLOT_COUNT);
    let block = if count == 0 {
        0
    } else {
        e.call(ALLOCATE, &args![array_bytes(count)]).u32()
    };
    e.mem.set_u32(this.addr(), block);
    let mut i = 0i32;
    while i < e.mem.u32(FAST_SLOT_COUNT) as i32 {
        let at = (i as u32).wrapping_mul(4);
        let source = e.mem.u32(e.mem.u32(other.addr()).wrapping_add(at));
        let copy = if source != 0 {
            let block = e.call(ALLOCATE, &args![8u32]).u32();
            if block == 0 {
                0
            } else {
                let source = e.mem.u32(e.mem.u32(other.addr()).wrapping_add(at));
                e.call(MODIFIER_COPY, &args![block, source]).u32()
            }
        } else {
            0
        };
        let array = e.mem.u32(this.addr());
        e.mem.set_u32(array.wrapping_add(at), copy);
        i += 1;
    }
    this
}

// Translated from 00937f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the fast-modifier collection (no Xbox PDB name): with slots and
/// an array, frees every non-null slot's modifier and then the array
/// (`00401030`), and clears the array pointer.
pub fn fn_00937f40(e: &mut Engine, this: Ptr) {
    if e.mem.u32(FAST_SLOT_COUNT) == 0 || e.mem.u32(this.addr()) == 0 {
        return;
    }
    let mut i = 0i32;
    while i < e.mem.u32(FAST_SLOT_COUNT) as i32 {
        let at = e
            .mem
            .u32(this.addr())
            .wrapping_add((i as u32).wrapping_mul(4));
        let modifier = e.mem.u32(at);
        if modifier != 0 {
            e.call(FREE, &args![modifier]);
        }
        i += 1;
    }
    let array = e.mem.u32(this.addr());
    e.call(FREE, &args![array]);
    e.mem.set_u32(this.addr(), 0);
}

/// The save/load game object (`TESSaveLoadGame*`, `011de45c`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `008579b0` and `008579e0` (unnamed, engine map unit `tessaveloadgame.cpp`; `thiscall` on
/// the save/load game object, `RET 8`: address, size): write and read that many bytes.
const SAVE_LOAD_WRITE: u32 = 0x0085_79b0;
const SAVE_LOAD_READ: u32 = 0x0085_79e0;
/// `00825c00` (unnamed, `thiscall` on the save/load game object, no stack words): the address
/// the count is written back to at the end of [`modifier_list_save_game`].
const SAVE_LOAD_COUNT_SLOT: u32 = 0x0082_5c00;
/// `005ae380` (`thiscall`, no stack words; the engine map's name `VATS::GetCount` is folded
/// code): the number of items in the list.
const LIST_COUNT: u32 = 0x005a_e380;
/// `BGSLoadGameBuffer` raw load (`00864980`, `thiscall`, `RET 8`: data, size) and
/// `BGSLoadGameBuffer::LoadVariableSizedValue` (engine map, `00864a60`, no stack words).
const LOAD_BYTES: u32 = 0x0086_4980;
const LOAD_VARIABLE_SIZE: u32 = 0x0086_4a60;
/// The fast slot of an actor value without one, and the number of actor values.
const ACTOR_VALUE_COUNT: i32 = 0x4d;

// Translated from 00937870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes one modifier (unnamed in the engine map; `thiscall`, one stack word, `RET 4`; `this`
/// is not read): `false` for a null modifier; otherwise its actor value byte
/// ([`fn_009373f0`]) and its `float` value (`006b9130`, stored as a `float`) go through the
/// save/load game object (`008579b0`, 1 and 4 bytes, from locals), and the answer is `true`.
pub fn fn_00937870(e: &mut Engine, _unused_0: Ptr<ModifierList>, modifier: u32) -> bool {
    if modifier == 0 {
        return false;
    }
    e.with_stack(8, |e, local| {
        let kind_at = local.addr() + 3;
        let value_at = local.addr() + 4;
        let kind = fn_009373f0(e, Ptr::new(modifier));
        e.mem.set_u8(kind_at, kind);
        let value = e.call(MODIFIER_VALUE, &args![modifier]).f64();
        e.mem.set_u32(value_at, x87_store32(value));
        let save = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_LOAD_WRITE, &args![save, kind_at, 1u32]);
        let save = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_LOAD_WRITE, &args![save, value_at, 4u32]);
    });
    true
}

// Translated from 009378d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModifierList::SaveGame` (engine map; `thiscall`, no stack words): through the save/load
/// game object, a 16-bit count written as 0 first (`008579b0`, after `00825c00` gave the
/// address the final count goes to); then every list item (walking the nodes from `this`
/// while the node's item is set) written by [`fn_00937870`], counted when it answers true;
/// then, with `pFastModifiers`, each of the `011e0750` slots written and counted whatever
/// the answer. The count is stored as 16 bits at `00825c00`'s address.
pub fn modifier_list_save_game(e: &mut Engine, this: Ptr<ModifierList>) {
    e.with_stack(4, |e, count_local| {
        let mut count: u16 = 0;
        let save = e.global::<u32>(SAVE_LOAD_GAME);
        let count_slot = e.call(SAVE_LOAD_COUNT_SLOT, &args![save]).u32();
        e.mem.set_u16(count_local.addr(), count);
        let save = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_LOAD_WRITE, &args![save, count_local, 2u32]);
        let mut node = this.addr();
        while node != 0 {
            let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
            if e.mem.u32(item_address) == 0 {
                break;
            }
            let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
            let item = e.mem.u32(item_address);
            if fn_00937870(e, this, item) {
                count = count.wrapping_add(1);
            }
            node = e.call(NEXT_NODE, &args![node]).u32();
        }
        let slots = if e.get(this, ModifierList::pFastModifiers) != 0 {
            e.mem.u32(FAST_SLOT_COUNT)
        } else {
            0
        };
        let mut index = 0i32;
        while index < slots as i32 {
            let item = e
                .mem
                .u32(fast_slots(e, this).wrapping_add((index as u32).wrapping_mul(4)));
            fn_00937870(e, this, item);
            count = count.wrapping_add(1);
            index += 1;
        }
        e.mem.set_u16(count_slot, count);
    });
}

// Translated from 009379c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009379c0 - the C++ exception unwinding states (FS:[0] frame, handler 00f0d3ab) are not translated
/// `ModifierList::LoadGame` (engine map; `thiscall`, no stack words): through the save/load
/// game object (`008579e0`), a 16-bit count, then for each modifier its actor value byte and
/// its `float` value; each becomes a new 8-byte modifier ([`fn_009375b0`]; 0 when the
/// allocation fails) taken in by [`fn_00937380`].
pub fn modifier_list_load_game(e: &mut Engine, this: Ptr<ModifierList>) {
    e.with_stack(8, |e, local| {
        let count_at = local.addr();
        let kind_at = local.addr() + 3;
        let value_at = local.addr() + 4;
        let save = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_LOAD_READ, &args![save, count_at, 2u32]);
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(count_at)) {
            let save = e.global::<u32>(SAVE_LOAD_GAME);
            e.call(SAVE_LOAD_READ, &args![save, kind_at, 1u32]);
            let save = e.global::<u32>(SAVE_LOAD_GAME);
            e.call(SAVE_LOAD_READ, &args![save, value_at, 4u32]);
            let kind = e.mem.u8(kind_at);
            let value = x87_quiet_bits(e.mem.u32(value_at));
            let modifier = new_modifier(e, kind, value);
            fn_00937380(e, this, modifier);
            index += 1;
        }
    });
}

// Translated from 00937ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModifierList::GetSaveSize` (engine map; `thiscall`, no stack words; a 16-bit answer in
/// `AX`): 5 bytes a modifier (the exe tests that local constant against 0, answering 0) times
/// the list's item count (`005ae380`) plus 2, plus 5 for each of the `011e0750` fast slots
/// when `pFastModifiers` is set; 16-bit arithmetic.
pub fn modifier_list_get_save_size(e: &mut Engine, this: Ptr<ModifierList>) -> u16 {
    let mut size: u16 = 0;
    let per_modifier: u16 = 5;
    if per_modifier == 0 {
        return 0;
    }
    let items = e.call(LIST_COUNT, &args![this]).u32();
    size = (u32::from(size)
        .wrapping_add(u32::from(per_modifier).wrapping_mul(items))
        .wrapping_add(2)) as u16;
    let slots = if e.get(this, ModifierList::pFastModifiers) != 0 {
        e.mem.u32(FAST_SLOT_COUNT)
    } else {
        0
    };
    (u32::from(size).wrapping_add(u32::from(per_modifier).wrapping_mul(slots))) as u16
}

// Translated from 00937d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The modifier default constructor (unnamed in the engine map; `thiscall`, no stack words):
/// the actor value byte 0 and the value 0.0 (`FLDZ`). Returns `this`.
pub fn fn_00937d30(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, x87_store32(0.0));
    this
}

// Translated from 00937c60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00937c60 - the C++ exception unwinding states (FS:[0] frame, handler 00f01e5b) are not translated
/// `ModifierList::LoadGame_ov2` (engine map; `thiscall`, one stack word: the load buffer,
/// `RET 4`): the count (`LoadVariableSizedValue`), then for each modifier a new 8-byte one
/// ([`fn_00937d30`]; 0 when the allocation fails, which the exe then reads into) whose
/// actor value byte and 4-byte value are read from the buffer (`00864980`), taken in by
/// [`fn_00937380`].
pub fn modifier_list_load_game_ov2(e: &mut Engine, this: Ptr<ModifierList>, buffer: u32) {
    let count = e.call(LOAD_VARIABLE_SIZE, &args![buffer]).u32();
    let mut index = 0u32;
    while index < count {
        let block = e.call(ALLOCATE, &args![8u32]).u32();
        let modifier = if block != 0 {
            fn_00937d30(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.call(LOAD_BYTES, &args![buffer, modifier, 1u32]);
        e.call(LOAD_BYTES, &args![buffer, modifier.wrapping_add(4), 4u32]);
        fn_00937380(e, this, modifier);
        index += 1;
    }
}

// Translated from 00938060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees and clears the fast slot of an actor value (unnamed in the engine map; `thiscall` on
/// the fast-modifier collection, one stack word, `RET 4`): for `kind` in `0..0x4d` (signed),
/// with an array, a slot (`011e0618[kind]` below 0x4d, signed) and a modifier in it: the
/// modifier is freed (`00401030`) and the slot cleared.
pub fn fn_00938060(e: &mut Engine, this: Ptr, kind: i32) {
    if !(0..ACTOR_VALUE_COUNT).contains(&kind) {
        return;
    }
    if e.mem.u32(this.addr()) == 0 {
        return;
    }
    let index_at = FAST_SLOT_INDEX.wrapping_add((kind as u32).wrapping_mul(4));
    if e.mem.i32(index_at) >= ACTOR_VALUE_COUNT {
        return;
    }
    let slot_at = |e: &Engine| {
        let index = e.mem.u32(index_at);
        e.mem.u32(this.addr()).wrapping_add(index.wrapping_mul(4))
    };
    if e.mem.u32(slot_at(e)) == 0 {
        return;
    }
    let modifier = e.mem.u32(slot_at(e));
    e.call(FREE, &args![modifier]);
    let slot = slot_at(e);
    e.mem.set_u32(slot, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0093_7870, fn_00937870(Ptr<ModifierList>, u32) -> bool),
        entry!(0x0093_78d0, modifier_list_save_game(Ptr<ModifierList>)),
        entry!(0x0093_79c0, modifier_list_load_game(Ptr<ModifierList>)),
        entry!(
            0x0093_7ab0,
            modifier_list_get_save_size(Ptr<ModifierList>) -> u16
        ),
        entry!(
            0x0093_7c60,
            modifier_list_load_game_ov2(Ptr<ModifierList>, u32)
        ),
        entry!(0x0093_7d30, fn_00937d30(Ptr) -> Ptr),
        entry!(0x0093_8060, fn_00938060(Ptr, i32)),
        entry!(
            0x0093_7160,
            fn_00937160(Ptr<ModifierList>, Ptr<ModifierList>)
        ),
        entry!(0x0093_7b30, fn_00937b30(Ptr)),
        entry!(0x009373f0, fn_009373f0(Ptr) -> u8),
        entry!(
            0x009370b0,
            modifier_list_delete_all_modifiers(Ptr<ModifierList>)
        ),
        entry!(0x00937130, fn_00937130(Ptr, u32) -> Ptr),
        entry!(0x00937280, fn_00937280(Ptr<ModifierList>, i8, u32)),
        entry!(0x00937380, fn_00937380(Ptr<ModifierList>, u32)),
        entry!(0x00937400, fn_00937400(Ptr<ModifierList>, u32)),
        entry!(
            0x00937480,
            modifier_list_adjust_modifier(Ptr<ModifierList>, u8, f32, u32)
        ),
        entry!(0x009375b0, fn_009375b0(Ptr, u8, f32) -> Ptr),
        entry!(0x009375e0, fn_009375e0(Ptr<ModifierList>, u8, f32)),
        entry!(
            0x009376e0,
            modifier_list_get_modifier_item(Ptr<ModifierList>, u8, Ptr) -> f64
        ),
        entry!(
            0x00937730,
            modifier_list_get_modifier(Ptr<ModifierList>, u8) -> f64
        ),
        entry!(0x00937760, fn_00937760(Ptr<ModifierList>, i8) -> u32),
        entry!(0x00937810, fn_00937810(f32, f32, u32) -> f64),
        entry!(
            0x00937b50,
            modifier_list_save_game_ov2(Ptr<ModifierList>, Ptr)
        ),
        entry!(0x00937d70, fn_00937d70(Ptr) -> Ptr),
        entry!(0x00937e10, fn_00937e10(Ptr, Ptr) -> Ptr),
        entry!(0x00937f40, fn_00937f40(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_first_byte() {
        let mut e = Engine::new();
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x1122_33a5);
        assert_eq!(e.call(0x0093_73f0, &[object]).u32() & 0xff, 0xa5);
    }
}

#[cfg(test)]
mod thunk_tests {
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
    fn fwd_a03_00937b30_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0093_70b0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0093_7b30, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0093_70b0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod copy_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const FIXED_BLOCK: u32 = 0x0e00_0000;

    /// Doubles for every callee outside this file. `allocation` is what `00401000` answers
    /// (0 for a failure); `00937e10` answers its block, `00936fd0` its block plus 1 and
    /// the next node of `00726070` comes from the word at `+4`. `00401000` answers
    /// `FIXED_BLOCK` and then `FIXED_BLOCK + 0x10` per call when `allocation` is non-zero.
    fn fixture(allocation: u32) -> (Engine, Log) {
        let mut e = Engine::new();
        let log: Log = Log::default();
        let calls = std::rc::Rc::new(std::cell::Cell::new(0u32));
        for address in [
            DELETE_ALL_MODIFIERS,
            ALLOCATE,
            FAST_MODIFIERS_COPY,
            NODE_ITEM_ADDRESS,
            MODIFIER_COPY,
            ADD_MODIFIER,
            NEXT_NODE,
        ] {
            let log = log.clone();
            let calls = calls.clone();
            e.register_double(address, move |e, a| {
                log.borrow_mut().push((address, a.to_vec()));
                let eax = match address {
                    ALLOCATE if allocation == 0 => 0,
                    ALLOCATE => {
                        calls.set(calls.get() + 1);
                        FIXED_BLOCK + 0x10 * calls.get()
                    }
                    FAST_MODIFIERS_COPY => a[0],
                    MODIFIER_COPY => a[0] + 1,
                    NODE_ITEM_ADDRESS => a[0],
                    NEXT_NODE => e.mem.u32(a[0] + 4),
                    _ => 0,
                };
                Ret {
                    eax,
                    ..Ret::default()
                }
            });
        }
        (e, log)
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn a_null_source_only_deletes_what_this_holds() {
        let (mut e, log) = fixture(1);
        let this = e.mem.alloc(0x10);
        e.call(0x0093_7160, &args![this, 0u32]);
        assert_eq!(*log.borrow(), [(DELETE_ALL_MODIFIERS, vec![this])]);
    }

    #[test]
    fn copies_the_fast_modifiers_and_every_item() {
        let (mut e, log) = fixture(1);
        let this = e.mem.alloc(0x10);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0xbbb0);
        e.mem.set_u32(second + 4, 0);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0xaaa0);
        e.mem.set_u32(source + 4, second);
        e.mem.set_u32(source + 0xc, 0x5000);
        e.call(0x0093_7160, &args![this, source]);
        assert_eq!(calls_to(&log, DELETE_ALL_MODIFIERS), [vec![this]]);
        assert_eq!(calls_to(&log, ALLOCATE), [vec![4], vec![8], vec![8]]);
        assert_eq!(
            calls_to(&log, FAST_MODIFIERS_COPY),
            [vec![FIXED_BLOCK + 0x10, 0x5000]]
        );
        assert_eq!(e.mem.u32(this + 0xc), FIXED_BLOCK + 0x10);
        assert_eq!(
            calls_to(&log, MODIFIER_COPY),
            [
                vec![FIXED_BLOCK + 0x20, 0xaaa0],
                vec![FIXED_BLOCK + 0x30, 0xbbb0]
            ]
        );
        assert_eq!(
            calls_to(&log, ADD_MODIFIER),
            [
                vec![this, FIXED_BLOCK + 0x21],
                vec![this, FIXED_BLOCK + 0x31]
            ]
        );
        assert_eq!(calls_to(&log, NEXT_NODE), [vec![source], vec![second]]);
    }

    #[test]
    fn without_fast_modifiers_only_the_items_are_copied() {
        let (mut e, log) = fixture(1);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0x7777);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0xccc0);
        e.call(0x0093_7160, &args![this, source]);
        assert_eq!(calls_to(&log, ALLOCATE), [vec![8]]);
        assert!(calls_to(&log, FAST_MODIFIERS_COPY).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0x7777);
        assert_eq!(calls_to(&log, ADD_MODIFIER).len(), 1);
    }

    #[test]
    fn an_empty_head_ends_the_walk() {
        let (mut e, log) = fixture(1);
        let this = e.mem.alloc(0x10);
        let source = e.mem.alloc(0x10);
        e.call(0x0093_7160, &args![this, source]);
        assert_eq!(calls_to(&log, NODE_ITEM_ADDRESS), [vec![source]]);
        assert!(calls_to(&log, ALLOCATE).is_empty());
        assert!(calls_to(&log, ADD_MODIFIER).is_empty());
    }

    #[test]
    fn a_failed_allocation_hands_over_a_null_copy() {
        let (mut e, log) = fixture(0);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0x7777);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0xccc0);
        e.mem.set_u32(source + 0xc, 0x5000);
        e.call(0x0093_7160, &args![this, source]);
        assert!(calls_to(&log, FAST_MODIFIERS_COPY).is_empty());
        assert!(calls_to(&log, MODIFIER_COPY).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls_to(&log, ADD_MODIFIER), [vec![this, 0]]);
    }
}

#[cfg(test)]
mod b019_tests {
    use super::*;

    /// The actor value whose info record marks it as a fast-slot value.
    const FAST_KIND: u8 = 5;
    /// Its slot.
    const FAST_SLOT: u32 = 1;

    /// Doubles for every callee outside this file; the list helpers work on
    /// real nodes (item at +0, next node at +4), modifiers keep their value
    /// at +4, `00401000` allocates from the engine heap, and only
    /// `FAST_KIND` has an info record with bit 0 of +0x44.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x011e_0000, 0x1000);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(FAST_SLOT_COUNT, 2u32);
        for kind in 0..0x4du32 {
            e.set_global(FAST_SLOT_INDEX + 4 * kind, 0x4du32);
        }
        e.set_global(FAST_SLOT_INDEX + 4 * u32::from(FAST_KIND), FAST_SLOT);
        let info = e.mem.alloc(0x48);
        e.mem.set_u32(info + 0x44, 1);
        e.register_double(ACTOR_VALUE_INFO, move |_, a| Ret {
            eax: if a[0] == u32::from(FAST_KIND) {
                info
            } else {
                0
            },
            ..Ret::default()
        });
        e.register(NODE_ITEM_ADDRESS, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(NEXT_NODE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MODIFIER_VALUE, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 4)),
            ..Ret::default()
        });
        e.register(MODIFIER_SET_VALUE, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(MODIFIER_COPY, |e, a| {
            let (kind, value) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            e.mem.set_u32(a[0], kind);
            e.mem.set_u32(a[0] + 4, value);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        for address in [
            FREE,
            LIST_REMOVE_HEAD,
            LIST_ADD,
            LIST_REMOVE,
            FAST_SLOT_REMOVE,
            BUILD_FAST_SLOTS,
            SAVE_START_VARIABLE_SIZED,
            SAVE_WRITE,
            SAVE_END_VARIABLE_SIZED,
        ] {
            e.register(address, |_, _| Ret {
                eax: 0x7700,
                ..Ret::default()
            });
        }
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn modifier(e: &mut Engine, kind: u8, value: f32) -> u32 {
        let m = e.mem.alloc(8);
        e.mem.set_u8(m, kind);
        e.mem.set_f32(m + 4, value);
        m
    }

    /// A list holding `items` (head node at `this`), no fast collection.
    fn list(e: &mut Engine, items: &[u32]) -> Ptr<ModifierList> {
        let this: Ptr<ModifierList> = e.new_object();
        let mut node = this.addr();
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        this
    }

    /// A fast collection for `this` with `slots` as its array.
    fn collection(e: &mut Engine, this: Ptr<ModifierList>, slots: [u32; 2]) -> (u32, u32) {
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, slots[0]);
        e.mem.set_u32(array + 4, slots[1]);
        let collection = e.mem.alloc(4);
        e.mem.set_u32(collection, array);
        e.set(this, ModifierList::pFastModifiers, collection);
        (collection, array)
    }

    #[test]
    fn delete_all_frees_the_items_and_the_fast_collection() {
        let mut e = engine();
        let item = modifier(&mut e, 2, 1.0);
        let this = list(&mut e, &[item]);
        // Dropping the head leaves a null item.
        e.register(LIST_REMOVE_HEAD, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        let fast = modifier(&mut e, FAST_KIND, 2.0);
        let (collection, array) = collection(&mut e, this, [0, fast]);
        e.call(0x0093_70b0, &args![this]);
        assert_eq!(
            calls(&e, FREE),
            vec![vec![item], vec![fast], vec![array], vec![collection]]
        );
        assert_eq!(calls(&e, LIST_REMOVE_HEAD), vec![vec![this.addr()]]);
        assert_eq!(e.get(this, ModifierList::pFastModifiers), 0);
    }

    #[test]
    fn collection_destructor_frees_itself_only_with_bit_0() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x0093_7130, &args![this, 2u32]).u32(), this);
        assert!(calls(&e, FREE).is_empty());
        e.call(0x0093_7130, &args![this, 1u32]);
        assert_eq!(calls(&e, FREE), vec![vec![this]]);
    }

    #[test]
    fn fast_slot_store_follows_value_and_zero_rule() {
        let mut e = engine();
        let this = list(&mut e, &[]);
        // Neither a modifier nor a collection: nothing.
        e.call(0x0093_7280, &args![this, FAST_KIND, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        // A zero value without zero values allowed: nothing stored.
        let zero = modifier(&mut e, FAST_KIND, 0.0);
        e.call(0x0093_7280, &args![this, FAST_KIND, zero]);
        assert!(calls(&e, ALLOCATE).is_empty());
        // Allowed: the collection is made and the slot set.
        e.set(this, ModifierList::bZeroValuesAllowed, 1);
        e.call(0x0093_7280, &args![this, FAST_KIND, zero]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![4], vec![8]]);
        assert_eq!(calls(&e, BUILD_FAST_SLOTS).len(), 1);
        let slots = fast_slots(&e, this);
        assert_eq!(e.mem.u32(slots + 4 * FAST_SLOT), zero);
        // With a collection the slot is cleared first (signed kind).
        let other = modifier(&mut e, FAST_KIND, 3.0);
        e.call(0x0093_7280, &args![this, FAST_KIND, other]);
        let collection = e.get(this, ModifierList::pFastModifiers);
        assert_eq!(
            calls(&e, FAST_SLOT_REMOVE),
            vec![vec![collection, u32::from(FAST_KIND)]]
        );
        assert_eq!(e.mem.u32(slots + 4 * FAST_SLOT), other);
    }

    #[test]
    fn add_sends_fast_values_to_their_slot_and_others_to_the_list() {
        let mut e = engine();
        let this = list(&mut e, &[]);
        let plain = modifier(&mut e, 2, 1.0);
        e.register_double(LIST_ADD, move |e, a| {
            // The list gets the address of a word holding the modifier.
            assert_eq!(e.mem.u32(a[1]), plain);
            Ret::default()
        });
        e.call(0x0093_7380, &args![this, 0u32]);
        assert!(calls(&e, ACTOR_VALUE_INFO).is_empty());
        e.call(0x0093_7380, &args![this, plain]);
        assert_eq!(calls(&e, LIST_ADD).len(), 1);
        assert_eq!(calls(&e, ACTOR_VALUE_INFO), vec![vec![2]]);
        let fast = modifier(&mut e, FAST_KIND, 1.0);
        e.call(0x0093_7380, &args![this, fast]);
        assert_eq!(calls(&e, LIST_ADD).len(), 1);
        let slots = fast_slots(&e, this);
        assert_eq!(e.mem.u32(slots + 4 * FAST_SLOT), fast);
    }

    #[test]
    fn remove_clears_a_fast_slot_or_frees_the_list_item() {
        let mut e = engine();
        let this = list(&mut e, &[]);
        let fast = modifier(&mut e, FAST_KIND, 1.0);
        // A fast value without a collection goes through the list.
        e.register(LIST_REMOVE, |e, a| {
            e.mem.set_u32(a[1], 0x4444);
            Ret::default()
        });
        e.call(0x0093_7400, &args![this, fast]);
        assert_eq!(calls(&e, LIST_REMOVE)[0][0], this.addr());
        assert_eq!(calls(&e, FREE), vec![vec![0x4444]]);
        let (collection, _) = collection(&mut e, this, [0, fast]);
        e.call(0x0093_7400, &args![this, fast]);
        assert_eq!(
            calls(&e, FAST_SLOT_REMOVE),
            vec![vec![collection, u32::from(FAST_KIND)]]
        );
        assert_eq!(calls(&e, FREE).len(), 1);
    }

    #[test]
    fn adjust_adds_changes_and_removes() {
        let mut e = engine();
        let this = list(&mut e, &[]);
        // No modifier and a zero delta: nothing.
        e.call(0x0093_7480, &args![this, 2u32, 0.0f32, 2u32]);
        assert!(calls(&e, ALLOCATE).is_empty());
        // A new modifier goes to the list.
        e.call(0x0093_7480, &args![this, 2u32, 2.5f32, 2u32]);
        let added = calls(&e, LIST_ADD);
        assert_eq!(added.len(), 1);
        let made = calls(&e, ALLOCATE);
        assert_eq!(made, vec![vec![8]]);
        // An existing modifier is adjusted and, at 0, removed.
        let item = modifier(&mut e, 3, 1.5);
        let this = list(&mut e, &[item]);
        e.call(0x0093_7480, &args![this, 3u32, 1.0f32, 2u32]);
        assert_eq!(e.mem.f32(item + 4), 2.5);
        e.call(0x0093_7480, &args![this, 3u32, -2.5f32, 2u32]);
        assert_eq!(e.mem.f32(item + 4), 0.0);
        assert_eq!(calls(&e, LIST_REMOVE).len(), 1);
        // Mode 1 keeps the sum at least 0.
        let item = modifier(&mut e, 4, 1.0);
        let this = list(&mut e, &[item]);
        e.set(this, ModifierList::bZeroValuesAllowed, 1);
        e.call(0x0093_7480, &args![this, 4u32, -3.0f32, 1u32]);
        assert_eq!(e.mem.f32(item + 4), 0.0);
        assert_eq!(calls(&e, LIST_REMOVE).len(), 1);
    }

    #[test]
    fn modifier_constructor_stores_kind_and_value() {
        let mut e = engine();
        let m = e.mem.alloc(8);
        assert_eq!(e.call(0x0093_75b0, &args![m, 7u32, 1.25f32]).u32(), m);
        assert_eq!(e.mem.u8(m), 7);
        assert_eq!(
            calls(&e, MODIFIER_SET_VALUE),
            vec![vec![m, 1.25f32.to_bits()]]
        );
    }

    #[test]
    fn set_adds_stores_or_removes() {
        let mut e = engine();
        let item = modifier(&mut e, 3, 1.5);
        let this = list(&mut e, &[item]);
        e.call(0x0093_75e0, &args![this, 3u32, 4.0f32]);
        assert_eq!(e.mem.f32(item + 4), 4.0);
        e.call(0x0093_75e0, &args![this, 3u32, 0.0f32]);
        assert_eq!(calls(&e, LIST_REMOVE).len(), 1);
        e.call(0x0093_75e0, &args![this, 9u32, 0.0f32]);
        assert!(calls(&e, ALLOCATE).is_empty());
        e.call(0x0093_75e0, &args![this, 9u32, f32::NAN]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![8]]);
        assert_eq!(calls(&e, LIST_ADD).len(), 1);
    }

    #[test]
    fn get_modifier_reads_the_value_or_zero() {
        let mut e = engine();
        let item = modifier(&mut e, 3, 1.5);
        let this = list(&mut e, &[item]);
        let found = e.mem.alloc(4);
        let v = e.call(0x0093_76e0, &args![this, 3u32, found]).f64();
        assert_eq!((v, e.mem.u8(found)), (1.5, 1));
        let v = e.call(0x0093_76e0, &args![this, 4u32, found]).f64();
        assert_eq!((v, e.mem.u8(found)), (0.0, 0));
        assert_eq!(e.call(0x0093_7730, &args![this, 3u32]).f64(), 1.5);
    }

    #[test]
    fn find_uses_the_fast_slot_or_walks_the_list() {
        let mut e = engine();
        let a = modifier(&mut e, 3, 1.0);
        let b = modifier(&mut e, 4, 2.0);
        let c = modifier(&mut e, 4, 3.0);
        let this = list(&mut e, &[a, b, c]);
        assert_eq!(e.call(0x0093_7760, &args![this, 4u32]).u32(), b);
        assert_eq!(e.call(0x0093_7760, &args![this, 6u32]).u32(), 0);
        // The walk stops once found: no NEXT_NODE call after b's node.
        e.call_log = Some(vec![]);
        e.call(0x0093_7760, &args![this, 3u32]);
        assert_eq!(calls(&e, NEXT_NODE).len(), 1);
        let fast = modifier(&mut e, FAST_KIND, 1.0);
        collection(&mut e, this, [0, fast]);
        assert_eq!(
            e.call(0x0093_7760, &args![this, u32::from(FAST_KIND)])
                .u32(),
            fast
        );
        // A value without a slot (0x4d) still walks the list.
        assert_eq!(e.call(0x0093_7760, &args![this, 4u32]).u32(), b);
    }

    #[test]
    fn add_and_clamp_by_mode() {
        let mut e = engine();
        let f = |e: &mut Engine, v: f32, d: f32, m: u32| e.call(0x0093_7810, &args![v, d, m]).f64();
        assert_eq!(f(&mut e, 1.0, 2.0, 0), 0.0);
        assert_eq!(f(&mut e, -1.0, -2.0, 0), -3.0);
        assert_eq!(f(&mut e, -1.0, -2.0, 1), 0.0);
        assert_eq!(f(&mut e, 1.0, 2.0, 1), 3.0);
        assert_eq!(f(&mut e, 1.0, 2.0, 7), 3.0);
        assert!(f(&mut e, f32::NAN, 2.0, 0).is_nan());
    }

    #[test]
    fn save_writes_list_then_fast_modifiers_and_the_count() {
        let mut e = engine();
        let a = modifier(&mut e, 3, 1.0);
        let this = list(&mut e, &[a]);
        let fast = modifier(&mut e, FAST_KIND, 2.0);
        collection(&mut e, this, [0, fast]);
        let buffer = 0x9900u32;
        e.call(0x0093_7b50, &args![this, buffer]);
        assert_eq!(
            calls(&e, SAVE_WRITE),
            vec![
                vec![buffer, a, 1, 0],
                vec![buffer, a + 4, 4, 0],
                vec![buffer, fast, 1, 0],
                vec![buffer, fast + 4, 4, 0],
            ]
        );
        assert_eq!(
            calls(&e, SAVE_END_VARIABLE_SIZED),
            vec![vec![buffer, 2, 0x7700]]
        );
    }

    #[test]
    fn collection_constructor_clears_one_word_per_slot() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        e.call(0x0093_7d70, &args![this]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![8]]);
        let array = e.mem.u32(this);
        assert_eq!((e.mem.u32(array), e.mem.u32(array + 4)), (0, 0));
        e.set_global(FAST_SLOT_COUNT, 0u32);
        e.call(0x0093_7d70, &args![this]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(array_bytes(0x4000_0000), 0xffff_ffff);
    }

    #[test]
    fn collection_copy_copies_each_used_slot() {
        let mut e = engine();
        let fast = modifier(&mut e, FAST_KIND, 2.0);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array + 4, fast);
        let other = e.mem.alloc(4);
        e.mem.set_u32(other, array);
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x0093_7e10, &args![this, other]).u32(), this);
        let copy = e.mem.u32(this);
        assert_eq!(e.mem.u32(copy), 0);
        let copied = e.mem.u32(copy + 4);
        assert_ne!(copied, fast);
        assert_eq!(e.mem.f32(copied + 4), 2.0);
        e.mem.set_u32(other, 0);
        e.call(0x0093_7e10, &args![this, other]);
        assert_eq!(e.mem.u32(this), 0);
    }

    #[test]
    fn collection_clear_frees_slots_then_the_array() {
        let mut e = engine();
        let array = e.mem.alloc(8);
        e.mem.set_u32(array + 4, 0x1234);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, array);
        e.call(0x0093_7f40, &args![this]);
        assert_eq!(calls(&e, FREE), vec![vec![0x1234], vec![array]]);
        assert_eq!(e.mem.u32(this), 0);
        e.call(0x0093_7f40, &args![this]);
        assert_eq!(calls(&e, FREE).len(), 2);
    }
}

#[cfg(test)]
mod n027_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// The save/load game object at `0x5000` (`011de45c`) and the fast slot globals mapped;
    /// the writer double records the bytes it is given (`(size, value)`).
    fn save_engine() -> (Engine, std::rc::Rc<std::cell::RefCell<Vec<(u32, u32)>>>) {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x3000);
        e.set_global(SAVE_LOAD_GAME, 0x5000u32);
        let written = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let record = written.clone();
        e.register_double(SAVE_LOAD_WRITE, move |e, a| {
            let value = match a[2] {
                1 => u32::from(e.mem.u8(a[1])),
                2 => u32::from(e.mem.u16(a[1])),
                _ => e.mem.u32(a[1]),
            };
            record.borrow_mut().push((a[2], value));
            Ret::default()
        });
        e.register(MODIFIER_VALUE, |e, a| {
            f64::from(e.mem.f32(a[0] + 4)).into_ret()
        });
        e.register(NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(NEXT_NODE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.call_log = Some(vec![]);
        (e, written)
    }

    fn modifier(e: &mut Engine, kind: u8, value: f32) -> u32 {
        let block = e.mem.alloc(8);
        e.mem.set_u8(block, kind);
        e.mem.set_f32(block + 4, value);
        block
    }

    #[test]
    fn fn_00937870_writes_kind_and_value() {
        let (mut e, written) = save_engine();
        let item = modifier(&mut e, 0x12, 2.5);
        assert!(e.call(0x0093_7870, &args![0u32, item]).bool());
        assert_eq!(*written.borrow(), vec![(1, 0x12), (4, 2.5f32.to_bits())]);
        assert_eq!(calls(&e, SAVE_LOAD_WRITE)[0][0], 0x5000);
        assert!(!e.call(0x0093_7870, &args![0u32, 0u32]).bool());
        assert_eq!(written.borrow().len(), 2);
    }

    #[test]
    fn save_game_writes_the_list_and_fast_slots_and_the_count() {
        let (mut e, written) = save_engine();
        let count_slot = e.mem.alloc(4);
        e.register_double(SAVE_LOAD_COUNT_SLOT, move |_, _| count_slot.into_ret());
        // A list of two items (head node is the list itself), then a node with no item.
        let list: Ptr<ModifierList> = e.new_object();
        let second = e.mem.alloc(8);
        let third = e.mem.alloc(8);
        let a = modifier(&mut e, 1, 1.0);
        let b = modifier(&mut e, 2, 2.0);
        e.mem.set_u32(list.addr(), a);
        e.mem.set_u32(list.addr() + 4, second);
        e.mem.set_u32(second, b);
        e.mem.set_u32(second + 4, third);
        // Two fast slots: one used, one empty (written and counted all the same).
        e.set_global(FAST_SLOT_COUNT, 2u32);
        let collection = e.mem.alloc(4);
        let slots = e.mem.alloc(8);
        e.mem.set_u32(collection, slots);
        let c = modifier(&mut e, 3, 3.0);
        e.mem.set_u32(slots, c);
        e.set(list, ModifierList::pFastModifiers, collection);
        e.call(0x0093_78d0, &args![list]);
        assert_eq!(
            *written.borrow(),
            vec![
                (2, 0),
                (1, 1),
                (4, 1.0f32.to_bits()),
                (1, 2),
                (4, 2.0f32.to_bits()),
                (1, 3),
                (4, 3.0f32.to_bits())
            ]
        );
        assert_eq!(e.mem.u16(count_slot), 4);
        // Without the collection only the list counts.
        e.set(list, ModifierList::pFastModifiers, 0);
        e.call(0x0093_78d0, &args![list]);
        assert_eq!(e.mem.u16(count_slot), 2);
    }

    #[test]
    fn load_game_reads_each_modifier_into_the_list() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x3000);
        e.set_global(SAVE_LOAD_GAME, 0x5000u32);
        // The stream: count 2, then (kind, value) pairs.
        let stream = [2u32, 7, 1.5f32.to_bits(), 9, 4.0f32.to_bits()];
        let mut next = 0usize;
        e.register_double(SAVE_LOAD_READ, move |e, a| {
            let value = stream[next];
            next += 1;
            match a[2] {
                1 => e.mem.set_u8(a[1], value as u8),
                2 => e.mem.set_u16(a[1], value as u16),
                _ => e.mem.set_u32(a[1], value),
            }
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MODIFIER_SET_VALUE, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        // Taken in: through the list (no actor value info).
        e.register(ACTOR_VALUE_INFO, |_, _| 0u32.into_ret());
        e.register(LIST_ADD, |e, a| {
            let modifier = e.mem.u32(a[1]);
            let count = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 0x10 + 4 * count, modifier);
            e.mem.set_u32(a[0] + 8, count + 1);
            Ret::default()
        });
        let list = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0093_79c0, &args![list]);
        assert_eq!(e.mem.u32(list + 8), 2);
        let first = e.mem.u32(list + 0x10);
        let second = e.mem.u32(list + 0x14);
        assert_eq!((e.mem.u8(first), e.mem.f32(first + 4)), (7, 1.5));
        assert_eq!((e.mem.u8(second), e.mem.f32(second + 4)), (9, 4.0));
        assert_eq!(calls(&e, SAVE_LOAD_READ).len(), 5);
        assert_eq!(calls(&e, SAVE_LOAD_READ)[0][0], 0x5000);
    }

    #[test]
    fn get_save_size_counts_five_bytes_a_modifier() {
        let mut e = Engine::new();
        e.map(0x011e_0000, 0x1000);
        e.set_global(FAST_SLOT_COUNT, 3u32);
        e.register(LIST_COUNT, |_, _| 4u32.into_ret());
        let list: Ptr<ModifierList> = e.new_object();
        assert_eq!(e.call(0x0093_7ab0, &args![list]).u16(), 22);
        e.set(list, ModifierList::pFastModifiers, 0x100);
        assert_eq!(e.call(0x0093_7ab0, &args![list]).u16(), 37);
        // 16-bit arithmetic.
        e.register(LIST_COUNT, |_, _| 0x3334u32.into_ret());
        assert_eq!(
            e.call(0x0093_7ab0, &args![list]).u16(),
            ((5 * 0x3334 + 2 + 15) & 0xffff) as u16
        );
    }

    #[test]
    fn fn_00937d30_clears_kind_and_value() {
        let mut e = Engine::new();
        let block = modifier(&mut e, 5, 9.0);
        e.mem.set_u8(block + 1, 0x66);
        assert_eq!(e.call(0x0093_7d30, &args![block]).u32(), block);
        assert_eq!(e.mem.u8(block), 0);
        assert_eq!(e.mem.u8(block + 1), 0x66);
        assert_eq!(e.mem.u32(block + 4), 0);
    }

    #[test]
    fn load_game_ov2_reads_into_new_modifiers() {
        let mut e = Engine::new();
        e.register(LOAD_VARIABLE_SIZE, |_, _| 2u32.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LOAD_BYTES, |e, a| {
            if a[2] == 1 {
                e.mem.set_u8(a[1], 4);
            } else {
                e.mem.set_f32(a[1], 0.5);
            }
            Ret::default()
        });
        e.register(ACTOR_VALUE_INFO, |_, _| 0u32.into_ret());
        e.register(LIST_ADD, nothing);
        let list = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0093_7c60, &args![list, 0x6000u32]);
        let loads = calls(&e, LOAD_BYTES);
        assert_eq!(loads.len(), 4);
        assert_eq!(loads[0][0], 0x6000);
        assert_eq!(loads[0][2], 1);
        assert_eq!(loads[1][1], loads[0][1] + 4);
        assert_eq!(loads[1][2], 4);
        let added = calls(&e, LIST_ADD);
        assert_eq!(added.len(), 2);
        assert_eq!(e.mem.u8(loads[2][1]), 4);
        assert_eq!(e.mem.f32(loads[3][1]), 0.5);
    }

    #[test]
    fn fn_00938060_frees_and_clears_the_slot() {
        let mut e = Engine::new();
        e.map(0x011e_0000, 0x1000);
        // Actor value 3 uses slot 1; 4 has no slot (0x4d).
        e.mem.set_u32(FAST_SLOT_INDEX + 4 * 3, 1);
        e.mem.set_u32(FAST_SLOT_INDEX + 4 * 4, 0x4d);
        e.register(FREE, nothing);
        let collection = e.mem.alloc(4);
        let slots = e.mem.alloc(8);
        e.mem.set_u32(collection, slots);
        e.mem.set_u32(slots + 4, 0x7777);
        e.call_log = Some(vec![]);
        for kind in [-1i32, 0x4d, 4] {
            e.call(0x0093_8060, &args![collection, kind]);
        }
        // Slot 0 (actor value 0) is empty.
        e.call(0x0093_8060, &args![collection, 0i32]);
        assert!(calls(&e, FREE).is_empty());
        e.call(0x0093_8060, &args![collection, 3i32]);
        assert_eq!(calls(&e, FREE), vec![vec![0x7777]]);
        assert_eq!(e.mem.u32(slots + 4), 0);
        // No array.
        e.mem.set_u32(collection, 0);
        e.call(0x0093_8060, &args![collection, 3i32]);
        assert_eq!(calls(&e, FREE).len(), 1);
    }
}
