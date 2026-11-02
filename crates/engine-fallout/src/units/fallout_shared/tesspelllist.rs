//! `fallout shared/tesspelllist.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0048d150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers `this + 0xC` (`thiscall`, no arguments): the address of a member
/// at `+0xC`. The engine map gives this folded body no name; its many
/// callers use it on different classes (in `TESSpellList` that offset is
/// `leveledSpellList`, Xbox PDB).
pub fn fn_0048d150(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0xc))
}

// Translated from 0048d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0048d0f0` (Ghidra name
/// `TESSpellList::FreeSpellList`), which empties the spell lists. The Xbox PDB name
/// `TESSpellList::ClearDataComponent` is the class's vslot +0x004 (pdbdump `type ...
/// TESSpellList`; engine map row `vt 822ded00`), and returns nothing. Parameters: one word,
/// `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX` on.
/// `0048d0f0`'s disassembly ends on a loop exit without loading a result (Ghidra types it
/// void).
pub fn tes_spell_list_clear_data_component(e: &mut Engine, this: Ptr) {
    e.call(0x0048_d0f0, &args![this]);
}

/// `00717e50` (thiscall): returns `this + 4`, the spell list (`TESSpellList`
/// +0x04, the first node of its list of spells).
const SPELL_LIST: u32 = 0x0071_7e50;
/// `006815c0` (thiscall on a list node): the address of the node's item word.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (thiscall on a list node): the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `0063f7b0` (thiscall on a list node): removes the node's item (the next
/// item moves into the node).
const NODE_REMOVE: u32 = 0x0063_f7b0;
/// `005f65d0` (thiscall on a list, `RET 4`: item pointer): whether the list
/// holds the item.
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// `005ae3d0` (thiscall on a list, `RET 4`: item pointer): adds the item.
const LIST_ADD: u32 = 0x005a_e3d0;
/// `00905330` (thiscall on a list node, `RET 4`: item pointer): the call the
/// spell list's `InitItem` makes on the previous node when the last node's
/// form is missing (`highprocess.cpp` in the engine map).
const NODE_REMOVE_AFTER: u32 = 0x0090_5330;
/// `00726c60` (thiscall on a list node, `RET 4`: item pointer): stores the item.
const NODE_SET_ITEM: u32 = 0x0072_6c60;
/// The player singleton pointer (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `004181e0` (thiscall on the player): the address its actor base's spell
/// list compares with (the value at the player compared after `+0x7C`).
const PLAYER_BASE_FORM: u32 = 0x0041_81e0;
/// `PlayerCharacter::SpellLearned` (Xbox PDB, `0094c6c0`, thiscall, `RET 4`).
const PLAYER_SPELL_LEARNED: u32 = 0x0094_c6c0;
/// The `TES` singleton pointer (`011c3f2c`).
const TES: u32 = 0x011c_3f2c;
/// `004516b0` (thiscall on `TES`, `tes.cpp`): the flag that makes `AddSpell`
/// skip the cast and the player notification.
const TES_FLAG: u32 = 0x0045_16b0;
/// `TESSpellList::AddSpell_ov2` (Xbox PDB, `0048d210`, thiscall, `RET 4`):
/// adds a leveled spell.
const ADD_LEVELED_SPELL: u32 = 0x0048_d210;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors: `TESForm` (`01183028`), `SpellItem` (`01183060`),
/// `TESLevSpell` (`01187818`), `BaseFormComponent` (`01183040`),
/// `TESSpellList` (`01186ba4`), `TESActorBase` (`011846e8`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_SPELL_ITEM: u32 = 0x0118_3060;
const TYPE_TES_LEV_SPELL: u32 = 0x0118_7818;
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_TES_SPELL_LIST: u32 = 0x0118_6ba4;
const TYPE_TES_ACTOR_BASE: u32 = 0x0118_46e8;
/// `TESForm::GetFile` (Xbox PDB, `00484e60`, thiscall, `RET 4`).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`, cdecl: id pointer, file).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (cdecl, one word): the form with the form id given.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `005b5e40` (cdecl, variadic): prints a formatted warning.
const PRINT_WARNING: u32 = 0x005b_5e40;
/// "MASTERFILE: Unable to find Actor Effect (%08X) for unknown owner."
const TEXT_NO_OWNER: u32 = 0x0101_d648;
/// "MASTERFILE: Unable to find Actor Effect (%08X) for owner object (%08X)."
const TEXT_OWNER_ID: u32 = 0x0101_d690;
/// "MASTERFILE: Unable to find Actor Effect (%08X) for owner object \"%s\"."
const TEXT_OWNER_NAME: u32 = 0x0101_d6d8;

/// Removes the list's items one at a time while the node at `node` holds one
/// (the exe tests `node` for null again before each step; it does not change).
fn empty_list(e: &mut Engine, node: u32) {
    if node == 0 {
        return;
    }
    loop {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        e.call(NODE_REMOVE, &args![node]);
    }
}

// Translated from 0048d0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::FreeSpellList` (Xbox PDB): removes every spell from the
/// spell list (`this + 4`, through `00717e50`) and every leveled spell from
/// the list at `this + 0xC` (`0048d150`), one `0063f7b0` per item.
pub fn tes_spell_list_free_spell_list(e: &mut Engine, this: Ptr) {
    let spells = e.call(SPELL_LIST, &args![this]).u32();
    empty_list(e, spells);
    let leveled = fn_0048d150(e, this).addr();
    empty_list(e, leveled);
}

// Translated from 0048d170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::AddSpell` (Xbox PDB, `RET 4`: the spell): false for a null
/// spell or one the list already holds (`005f65d0`); otherwise adds it
/// (`005ae3d0`) and, when this list is the player's (`this` is the address
/// `004181e0` gives for the player plus 0x7C) and the `TES` flag `004516b0` is
/// clear, tells the player (`PlayerCharacter::SpellLearned`). Then true.
pub fn tes_spell_list_add_spell(e: &mut Engine, this: Ptr, spell: u32) -> bool {
    if spell == 0 {
        return false;
    }
    // The spell parameter's own stack word is what the list calls get and
    // what the player is told about afterwards.
    let added = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), spell);
        let list = e.call(SPELL_LIST, &args![this]).u32();
        if e.call(LIST_CONTAINS, &args![list, slot]).bool() {
            return None;
        }
        let list = e.call(SPELL_LIST, &args![this]).u32();
        e.call(LIST_ADD, &args![list, slot]);
        Some(e.mem.u32(slot.addr()))
    });
    let Some(spell) = added else {
        return false;
    };
    let player = e.global::<u32>(PLAYER);
    let base = e.call(PLAYER_BASE_FORM, &args![player]).u32();
    let players_list = if base != 0 { base + 0x7c } else { 0 };
    if this.addr() == players_list {
        let tes = e.global::<u32>(TES);
        if !e.call(TES_FLAG, &args![tes]).bool() {
            let player = e.global::<u32>(PLAYER);
            e.call(PLAYER_SPELL_LEARNED, &args![player, spell]);
        }
    }
    true
}

// Translated from 0048d290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::AddSpell_ov3` (Xbox PDB, `RET 4`: a form): when the `TES`
/// flag `004516b0` is set, `AddSpell` with the form as it is (its result
/// dropped) and true. Otherwise the form cast to `SpellItem` goes to
/// `AddSpell`, or, failing that, the form cast to `TESLevSpell` to
/// `AddSpell_ov2` (`0048d210`); false when it is neither.
pub fn tes_spell_list_add_spell_ov3(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let tes = e.global::<u32>(TES);
    if e.call(TES_FLAG, &args![tes]).bool() {
        tes_spell_list_add_spell(e, this, form);
        return true;
    }
    let spell = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_SPELL_ITEM, 0u32],
        )
        .u32();
    let leveled = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_LEV_SPELL, 0u32],
        )
        .u32();
    if spell != 0 {
        tes_spell_list_add_spell(e, this, spell)
    } else if leveled != 0 {
        e.call(ADD_LEVELED_SPELL, &args![this, leveled]).bool()
    } else {
        false
    }
}

/// The step `InitItem` takes on a node whose form is missing or not a spell:
/// the previous node's `00905330` (with the form id) and its next node when
/// this is the last node and a previous node exists; otherwise the node's
/// item is removed in place. Returns the node to look at next.
fn drop_node(e: &mut Engine, node: u32, previous: u32, id: u32) -> u32 {
    if e.call(NODE_NEXT, &args![node]).u32() == 0 && previous != 0 {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), id);
            e.call(NODE_REMOVE_AFTER, &args![previous, slot]);
        });
        e.call(NODE_NEXT, &args![previous]).u32()
    } else {
        e.call(NODE_REMOVE, &args![node]);
        node
    }
}

// Translated from 0048d400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::InitItem` (Xbox PDB, `RET 4`: the owning form or null):
/// turns the spell list's form ids into forms. For each node while it holds
/// an id: the id gets the owner's file compile index (`GetFile(-1)`; file 0
/// without an owner) and is looked up (`004839c0`).
/// - Not found: the node is dropped (see `drop_node`) and a `MASTERFILE`
///   warning printed (`005b5e40`): with the owner's name (vtable slot 0x130)
///   when `00474cb0` says the owner has one, with the owner's id (`0084e3a0`)
///   otherwise, or for an unknown owner.
/// - A `SpellItem`: stored in the node (`00726c60`); when this list belongs
///   to an actor base that is the player's (`004181e0`), the player learns
///   it. The node becomes the previous node; the next node follows.
/// - Otherwise: the node is dropped and a `TESLevSpell` is added through
///   `AddSpell_ov2`.
pub fn tes_spell_list_init_item(e: &mut Engine, this: Ptr, owner: u32) {
    let mut previous = 0u32;
    let mut node = e.call(SPELL_LIST, &args![this]).u32();
    let file = if owner != 0 {
        e.call(FORM_GET_FILE, &args![owner, 0xffff_ffffu32]).u32()
    } else {
        0
    };
    while node != 0 {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let (id, form) = e.with_stack(4, |e, slot| {
            let id = e.mem.u32(item);
            e.mem.set_u32(slot.addr(), id);
            e.call(FORM_ADD_COMPILE_INDEX, &args![slot, file]);
            let id = e.mem.u32(slot.addr());
            (id, e.call(LOOKUP_FORM, &args![id]).u32())
        });
        if form == 0 {
            node = drop_node(e, node, previous, id);
            if owner != 0 {
                if e.call(0x0047_4cb0, &args![owner]).u32() != 0 {
                    let name = e.vcall(owner, 0x130, &[]).u32();
                    e.call(PRINT_WARNING, &args![TEXT_OWNER_NAME, id, name]);
                } else {
                    let owner_id = e.call(0x0084_e3a0, &args![owner]).u32();
                    e.call(PRINT_WARNING, &args![TEXT_OWNER_ID, id, owner_id]);
                }
            } else {
                e.call(PRINT_WARNING, &args![TEXT_NO_OWNER, id]);
            }
            continue;
        }
        let spell = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_SPELL_ITEM, 0u32],
            )
            .u32();
        let leveled = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_LEV_SPELL, 0u32],
            )
            .u32();
        if spell != 0 {
            // The cast result's stack word goes to the node and, read again,
            // to the player.
            let spell = e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), spell);
                e.call(NODE_SET_ITEM, &args![node, slot]);
                e.mem.u32(slot.addr())
            });
            let actor_base = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![this, 0u32, TYPE_TES_SPELL_LIST, TYPE_TES_ACTOR_BASE, 0u32],
                )
                .u32();
            if actor_base != 0 {
                let player = e.global::<u32>(PLAYER);
                if actor_base == e.call(PLAYER_BASE_FORM, &args![player]).u32() {
                    let player = e.global::<u32>(PLAYER);
                    e.call(PLAYER_SPELL_LEARNED, &args![player, spell]);
                }
            }
            previous = node;
            node = e.call(NODE_NEXT, &args![node]).u32();
        } else {
            node = drop_node(e, node, previous, id);
            if leveled != 0 {
                e.call(ADD_LEVELED_SPELL, &args![this, leveled]);
            }
        }
    }
}

/// Adds every nonzero item of the list starting at `from` to the list `to`.
fn copy_items(e: &mut Engine, mut from: u32, to: u32) {
    while from != 0 {
        let item = e.call(NODE_ITEM, &args![from]).u32();
        let value = e.mem.u32(item);
        if value != 0 {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), value);
                e.call(LIST_ADD, &args![to, slot]);
            });
        }
        from = e.call(NODE_NEXT, &args![from]).u32();
    }
}

// Translated from 0048d630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::CopyComponent` (Xbox PDB, `RET 4`: the other component):
/// when the other component is a `TESSpellList` (`__RTDynamicCast` from
/// `BaseFormComponent`), empties this one (`FreeSpellList`) and adds the
/// other's spells, then its leveled spells, in order (`005ae3d0`).
pub fn tes_spell_list_copy_component(e: &mut Engine, this: Ptr, other: u32) {
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_SPELL_LIST,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return;
    }
    tes_spell_list_free_spell_list(e, this);
    let from = e.call(SPELL_LIST, &args![source]).u32();
    let to = e.call(SPELL_LIST, &args![this]).u32();
    copy_items(e, from, to);
    let from = fn_0048d150(e, Ptr::new(source)).addr();
    let to = fn_0048d150(e, this).addr();
    copy_items(e, from, to);
}

/// Writes the form ids of the nonzero items of the list starting at `node`
/// as one variable-sized value of the save buffer.
fn save_list(e: &mut Engine, buffer: u32, mut node: u32) {
    let mut count = 0u32;
    let start = e.call(0x0086_5f20, &args![buffer]).u32();
    while node != 0 {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let value = e.mem.u32(item);
        if value != 0 {
            e.call(0x0086_5df0, &args![buffer, value, 0u32]);
            count += 1;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(0x0086_5ff0, &args![buffer, count, start]);
}

// Translated from 0048e040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the spell lists to the save buffer (`RET 4`: the buffer) when its
/// flag 0x10 is set (`00428110` copies the buffer's word at +0x17 to a local,
/// `004280f0` tests the bit): the spells (`this + 4`), then the leveled spells
/// (`this + 0xC`), each as a count-prefixed run of form ids
/// (`BGSSaveGameBuffer::StartVariableSizedValue`, `SaveFormID_ov2` with 0,
/// `SaveVariableSizedValue_ov2`).
pub fn fn_0048e040(e: &mut Engine, this: Ptr, buffer: u32) {
    let saves = e.with_stack(4, |e, local| {
        let flags = e.call(0x0042_8110, &args![buffer, local]).u32();
        e.call(0x0042_80f0, &args![flags, 0x10u32]).bool()
    });
    if !saves {
        return;
    }
    save_list(e, buffer, this.addr() + 4);
    save_list(e, buffer, this.addr() + 0xc);
}

// Translated from 0048d210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::AddSpell_ov2` (Xbox PDB, `RET 4`: a leveled spell): false
/// for a null spell or when this list is the player's (`004181e0` on the
/// player plus 0x7C, or 0 when that gives null); false when the leveled list
/// (`0048d150`) already holds it (`005f65d0`); otherwise adds it there
/// (`005ae3d0`) and true. The list calls get the parameter's own stack word.
pub fn tes_spell_list_add_spell_ov2(e: &mut Engine, this: Ptr, spell: u32) -> bool {
    if spell == 0 {
        return false;
    }
    let player = e.global::<u32>(PLAYER);
    let base = e.call(PLAYER_BASE_FORM, &args![player]).u32();
    let players_list = if base != 0 {
        base.wrapping_add(0x7c)
    } else {
        0
    };
    if this.addr() == players_list {
        return false;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), spell);
        let list = fn_0048d150(e, this).addr();
        if e.call(LIST_CONTAINS, &args![list, slot]).bool() {
            return false;
        }
        let list = fn_0048d150(e, this).addr();
        e.call(LIST_ADD, &args![list, slot]);
        true
    })
}

/// `00440d80` (thiscall on a form): whether its flags word (+8) has bit 0x20.
const FORM_FLAG_20: u32 = 0x0044_0d80;
/// `0084e3a0` (thiscall on a form): its form id (`TESForm` +0x0C).
const FORM_ID: u32 = 0x0084_e3a0;
/// `00485910` (cdecl, two words: a chunk tag and a form id; `tesform.cpp` in
/// the engine map): what `Save` calls for each listed form without flag 0x20.
const SAVE_FORM_ID_CHUNK: u32 = 0x0048_5910;
/// The chunk tag `Save` passes: the bytes `SPLO`.
const CHUNK_SPLO: u32 = 0x4f4c_5053;

/// `Save`'s step over one list: each nonzero item without form flag 0x20 is
/// written as an `SPLO` chunk with its form id.
fn save_list_ids(e: &mut Engine, mut node: u32) {
    while node != 0 {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let form = e.mem.u32(item);
        if form == 0 {
            break;
        }
        if !e.call(FORM_FLAG_20, &args![form]).bool() {
            let id = e.call(FORM_ID, &args![form]).u32();
            e.call(SAVE_FORM_ID_CHUNK, &args![CHUNK_SPLO, id]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 0048d330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::Save` (Xbox PDB; `thiscall`, plain `RET`): writes the form
/// ids of the spells (`this + 4`, through `00717e50`), then of the leveled
/// spells (`0048d150`), as `SPLO` chunks (`00485910`), stopping each list at
/// its first empty item and skipping forms whose flag 0x20 is set
/// (`00440d80`).
pub fn tes_spell_list_save(e: &mut Engine, this: Ptr) {
    let spells = e.call(SPELL_LIST, &args![this]).u32();
    save_list_ids(e, spells);
    let leveled = fn_0048d150(e, this).addr();
    save_list_ids(e, leveled);
}

/// `VATS::GetCount` in the engine map (`005ae380`, thiscall on a list; a
/// folded body): the list's item count.
const LIST_COUNT: u32 = 0x005a_e380;

// Translated from 0048d700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::CompareComponent` (Xbox PDB, `RET 4`: the other
/// component): true ("different") unless the other component is a
/// `TESSpellList` (`__RTDynamicCast` from `BaseFormComponent`) that holds
/// every nonzero spell of this list (`0048d830`) and every nonzero leveled
/// spell (`0048d880`), with item counts (`005ae380`) equal to the numbers of
/// nonzero items here. Empty items do not stop the walks.
pub fn tes_spell_list_compare_component(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_SPELL_LIST,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return true;
    }
    let mut count = 0u32;
    let mut node = e.call(SPELL_LIST, &args![this]).u32();
    while node != 0 {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let spell = e.mem.u32(item);
        if spell != 0 {
            if !fn_0048d830(e, Ptr::new(source), spell) {
                return true;
            }
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let list = e.call(SPELL_LIST, &args![source]).u32();
    if e.call(LIST_COUNT, &args![list]).u32() != count {
        return true;
    }
    let mut count = 0u32;
    let mut node = fn_0048d150(e, this).addr();
    while node != 0 {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) != 0 {
            let item = e.call(NODE_ITEM, &args![node]).u32();
            let spell = e.mem.u32(item);
            if !fn_0048d880(e, Ptr::new(source), spell) {
                return true;
            }
            count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let list = fn_0048d150(e, Ptr::new(source)).addr();
    e.call(LIST_COUNT, &args![list]).u32() != count
}

/// Whether the list starting at `node` holds `item`; the walk does not stop
/// at an empty item.
fn list_holds(e: &mut Engine, mut node: u32, item: u32) -> bool {
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == item {
            return true;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 0048d830 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`RET 4`: a spell): whether the spell list (`this + 4`,
/// through `00717e50`) holds the spell, walking every node.
pub fn fn_0048d830(e: &mut Engine, this: Ptr, spell: u32) -> bool {
    let node = e.call(SPELL_LIST, &args![this]).u32();
    list_holds(e, node, spell)
}

// Translated from 0048d880 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`RET 4`: a leveled spell): whether the leveled spell list
/// (`0048d150`) holds it, walking every node.
pub fn fn_0048d880(e: &mut Engine, this: Ptr, spell: u32) -> bool {
    let node = fn_0048d150(e, this).addr();
    list_holds(e, node, spell)
}

// Translated from 0048d8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`RET 4`: a spell): hands the spell parameter's stack word to
/// `00905330` on the spell list (`this + 4`, through `00717e50`), the call
/// actors make to remove a spell.
pub fn fn_0048d8d0(e: &mut Engine, this: Ptr, spell: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), spell);
        let list = e.call(SPELL_LIST, &args![this]).u32();
        e.call(NODE_REMOVE_AFTER, &args![list, slot]);
    });
}

/// The save/load game object the old save functions use (the pointer at
/// `011de45c`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `TESSaveLoadGame::UseSaveGameBlocks` (engine map, `00862110`, thiscall):
/// whether the save version (`008df040`) is 0x1F to 0x59.
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
/// `00408d60` (thiscall on a setting): the address of its value.
const SETTING_VALUE_POINTER: u32 = 0x0040_8d60;
/// A setting (`011de4e8`) whose value byte turns on the save-size reports.
const SAVE_REPORT_SETTING: u32 = 0x011d_e4e8;
/// `004fd3e0` (thiscall): the word at +0x88 of the save/load game object,
/// the form being written (or null).
const SAVING_FORM: u32 = 0x004f_d3e0;
/// `004fd3c0` (thiscall): the word at +0x84 of the save/load game object,
/// the form being read (or null).
const LOADING_FORM: u32 = 0x004f_d3c0;
/// `00825c00` (thiscall): the word at +0x14 of the save/load game object,
/// its buffer position.
const BUFFER_POSITION: u32 = 0x0082_5c00;
/// `008df040` (thiscall): the byte at +0x80 of the save/load game object,
/// the save version.
const SAVE_VERSION: u32 = 0x008d_f040;
/// `008579b0` (thiscall, `RET 8`: data, size): writes bytes.
const WRITE_BYTES: u32 = 0x0085_79b0;
/// `008579e0` (thiscall, `RET 8`: data, size): reads bytes.
const READ_BYTES: u32 = 0x0085_79e0;
/// `TESSaveLoadGame::SaveNumericID` (engine map, `00857a10`, `RET 8`).
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
/// `TESSaveLoadGame::LoadNumericID` (engine map, `00857aa0`, `RET 8`).
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
/// `Error` (`0040fbe0`, engine map; cdecl, a format and its words).
const ERROR: u32 = 0x0040_fbe0;
/// The block marker the old saves write before a block: the bytes `KOLB`.
const BLOCK_MARKER: u32 = 0x424c_4f4b;
/// The source file name the reports pass:
/// "D:\_Fallout3\Platforms\Common\Code\Fallout Shared\TESSpellList.cpp".
const SOURCE_FILE: u32 = 0x0101_d720;
/// "GetSaveSize(): %-5i for form %08X %s with flags %08X ending at line %i in file %s"
const TEXT_SIZE_FOR_FORM: u32 = 0x0101_2cb0;
/// "GetSaveSize(): %-5i ending at line %i in file %s"
const TEXT_SIZE: u32 = 0x0101_2c78;
/// "SaveGame(): %-5i for form %08X %s with flags %08X ending at line %i in file %s"
const TEXT_SAVED_FOR_FORM: u32 = 0x0101_53a0;
/// "SaveGame(): %-5i ending at line %i in file %s"
const TEXT_SAVED: u32 = 0x0101_536c;
/// "SAVELOAD: Save Game Block in file %s on line %i is greater than maximum short size"
const TEXT_BLOCK_TOO_LARGE: u32 = 0x0101_5318;
/// "SAVELOAD: (LoadGame Buffer error) Block Header is incorrect in file %s on
/// line %i.  Currently loading form is %08X %s with version %i and flags %08X"
const TEXT_BAD_HEADER_FOR_FORM: u32 = 0x0101_5718;
/// "SAVELOAD: (LoadGame Buffer error) Block Header is incorrect in file %s on
/// line %i.  Current version is %i"
const TEXT_BAD_HEADER: u32 = 0x0101_56a8;
/// "MASTERFILE: Could not find spell %08X"
const TEXT_NO_SPELL: u32 = 0x0101_d764;
/// "SAVELOAD: LoadGame Buffer overrun of %i bytes in file %s on line %i.
/// Currently loading form is %08X %s with version %i and flags %08X"
const TEXT_OVERRUN_FOR_FORM: u32 = 0x0101_5588;
/// The same report for an underrun.
const TEXT_UNDERRUN_FOR_FORM: u32 = 0x0101_5500;
/// "SAVELOAD: LoadGame Buffer overrun of %i bytes in file %s on line %i.
/// Current version is %i"
const TEXT_OVERRUN: u32 = 0x0101_54a0;
/// The same report for an underrun.
const TEXT_UNDERRUN: u32 = 0x0101_5440;
/// `00470470` (thiscall on a list): empties it.
const LIST_CLEAR: u32 = 0x0047_0470;
/// `BGSLoadGameBuffer::LoadVariableSizedValue` (engine map, `00864a60`).
const LOAD_VARIABLE_SIZED_VALUE: u32 = 0x0086_4a60;
/// `BGSLoadGameBuffer::LoadFormID` (engine map, `008648a0`).
const LOAD_FORM_ID: u32 = 0x0086_48a0;

fn save_load_game(e: &mut Engine) -> u32 {
    e.global::<u32>(SAVE_LOAD_GAME)
}

fn save_reports_on(e: &mut Engine) -> bool {
    let value = e
        .call(SETTING_VALUE_POINTER, &args![SAVE_REPORT_SETTING])
        .u32();
    e.mem.u8(value) != 0
}

/// The save-size report the save functions print through `Error` when the
/// report setting is on: with the form being written (its id, its name from
/// vtable slot 0x130 of the form its id looks up, and its word at +5), or
/// without one. `size` is the first word after the format.
fn report_size(e: &mut Engine, size: u32, line: u32, with_form: u32, without_form: u32) {
    let slg = save_load_game(e);
    let current = e.call(SAVING_FORM, &args![slg]).u32();
    if current != 0 {
        let id = e.mem.u32(current);
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let flags = e.mem.u32(current.wrapping_add(5));
        let name = e.vcall(form, 0x130, &[]).u32();
        let id = e.mem.u32(current);
        e.call(
            ERROR,
            &args![with_form, size, id, name, flags, line, SOURCE_FILE],
        );
    } else {
        e.call(ERROR, &args![without_form, size, line, SOURCE_FILE]);
    }
}

// Translated from 0048d8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`RET 4`: the save flags): the size the spell lists take in
/// an old save, 0 unless flag 0x10 is set: 4 + 2 when the save uses blocks
/// (`00862110`), 2 for the count and 4 per item of either list (`005ae380`
/// on `this + 4` and `this + 0xC`), all in 16 bits. With the report setting
/// on, prints the size through `Error` (line 0x51B), with the form being
/// written when there is one.
pub fn fn_0048d8f0(e: &mut Engine, this: Ptr, flags: u32) -> u16 {
    let mut size = 0u16;
    if flags & 0x10 == 0 {
        return size;
    }
    let start = size;
    let slg = save_load_game(e);
    if e.call(USE_SAVE_GAME_BLOCKS, &args![slg]).bool() {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    size = size.wrapping_add(2);
    let spells = e
        .call(LIST_COUNT, &args![this.addr().wrapping_add(4)])
        .u32();
    size = size.wrapping_add((spells as u16).wrapping_mul(4));
    let leveled = e
        .call(LIST_COUNT, &args![this.addr().wrapping_add(0xc)])
        .u32();
    size = size.wrapping_add((leveled as u16).wrapping_mul(4));
    if save_reports_on(e) {
        let reported = u32::from(size).wrapping_sub(u32::from(start));
        report_size(e, reported, 0x51b, TEXT_SIZE_FOR_FORM, TEXT_SIZE);
    }
    size
}

/// `SaveGame`'s step over one list: each nonzero item's form id is written
/// (`SaveNumericID`, 4 bytes) and counted.
fn save_game_list(e: &mut Engine, mut node: u32, id_slot: u32, count: &mut u16) {
    while node != 0 {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let form = e.mem.u32(item);
        if form != 0 {
            let id = e.call(FORM_ID, &args![form]).u32();
            e.mem.set_u32(id_slot, id);
            let slg = save_load_game(e);
            e.call(SAVE_NUMERIC_ID, &args![slg, id_slot, 4u32]);
            *count = count.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 0048da20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::SaveGame` (Xbox PDB, `RET 4`: the save flags): with flag
/// 0x10, writes the spell lists to an old save: when the save uses blocks,
/// the `KOLB` marker and a 16-bit block size placeholder; a 16-bit count
/// (patched in place at the end) and the form ids of the nonzero spells and
/// leveled spells. With the report setting on it prints the bytes written
/// (line 0x54F); with blocks it patches the block size, first warning
/// (`005b5e40`) when the block runs past 0xFFFF bytes.
pub fn tes_spell_list_save_game(e: &mut Engine, this: Ptr, flags: u32) {
    if flags & 0x10 == 0 {
        return;
    }
    // Locals: +0 block size placeholder (u16), +4 marker, +8 count (u16),
    // +0xC form id.
    e.with_stack(0x10, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u16(frame, 0);
        let mut block = 0u32;
        let slg = save_load_game(e);
        let mut start = e.call(BUFFER_POSITION, &args![slg]).u32();
        if save_reports_on(e) {
            let slg = save_load_game(e);
            start = e.call(BUFFER_POSITION, &args![slg]).u32();
        }
        let slg = save_load_game(e);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![slg]).bool() {
            e.mem.set_u32(frame + 4, BLOCK_MARKER);
            let slg = save_load_game(e);
            e.call(WRITE_BYTES, &args![slg, frame + 4, 4u32]);
            let slg = save_load_game(e);
            block = e.call(BUFFER_POSITION, &args![slg]).u32();
            let slg = save_load_game(e);
            e.call(WRITE_BYTES, &args![slg, frame, 2u32]);
        }
        let mut count = 0u16;
        e.mem.set_u16(frame + 8, count);
        let slg = save_load_game(e);
        let count_at = e.call(BUFFER_POSITION, &args![slg]).u32();
        let slg = save_load_game(e);
        e.call(WRITE_BYTES, &args![slg, frame + 8, 2u32]);
        let spells = e.call(SPELL_LIST, &args![this]).u32();
        save_game_list(e, spells, frame + 0xc, &mut count);
        let leveled = fn_0048d150(e, this).addr();
        save_game_list(e, leveled, frame + 0xc, &mut count);
        e.mem.set_u16(count_at, count);
        if save_reports_on(e) {
            let slg = save_load_game(e);
            let now = e.call(BUFFER_POSITION, &args![slg]).u32();
            report_size(
                e,
                now.wrapping_sub(start),
                0x54f,
                TEXT_SAVED_FOR_FORM,
                TEXT_SAVED,
            );
        }
        let slg = save_load_game(e);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![slg]).bool() {
            let slg = save_load_game(e);
            let end = e.call(BUFFER_POSITION, &args![slg]).u32();
            if end > block.wrapping_add(0xffff) {
                e.call(
                    PRINT_WARNING,
                    &args![TEXT_BLOCK_TOO_LARGE, SOURCE_FILE, 0x54fu32],
                );
            }
            e.mem.set_u16(block, end.wrapping_sub(block) as u16);
        }
    });
}

/// The block-check report `LoadGame` prints through `005b5e40` with the form
/// being read: the format, `amount` (when the report has one), the file and
/// line, the form's id, its name (vtable slot 0x130 of the form its id looks
/// up), its byte at +9 and its word at +5. `amount` is `None` for the block
/// header report.
fn report_loading_form(
    e: &mut Engine,
    current: u32,
    form: u32,
    text: u32,
    amount: Option<u32>,
    line: u32,
) {
    let flags = e.mem.u32(current.wrapping_add(5));
    let version = u32::from(e.mem.u8(current.wrapping_add(9)));
    let name = e.vcall(form, 0x130, &[]).u32();
    let id = e.mem.u32(current);
    match amount {
        Some(amount) => e.call(
            PRINT_WARNING,
            &args![text, amount, SOURCE_FILE, line, id, name, version, flags],
        ),
        None => e.call(
            PRINT_WARNING,
            &args![text, SOURCE_FILE, line, id, name, version, flags],
        ),
    };
}

/// The save version as `005b5e40`'s reports pass it (zero-extended).
fn save_version(e: &mut Engine) -> u32 {
    let slg = save_load_game(e);
    u32::from(e.call(SAVE_VERSION, &args![slg]).u8())
}

// Translated from 0048dca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::LoadGame` (Xbox PDB, `RET 8`: the save flags and a word it
/// does not read): with flag 0x10, reads the spell lists back from an old
/// save. When the save uses blocks it reads the block marker (reporting a
/// wrong one, line 0x55B, with the form being read or the save version) and
/// the 16-bit block size, noting the position after the marker. Both lists
/// are emptied (`00470470`); a 16-bit count of form ids follows
/// (`LoadNumericID`), each looked up (`004839c0`) and cast: a `SpellItem` is
/// added to the spells; a `TESLevSpell` to the leveled spells unless, in a
/// save older than version 0x47, they already hold it; neither is reported
/// ("MASTERFILE: Could not find spell"). With blocks, a position past or
/// short of the block's end is reported (line 0x57F).
pub fn tes_spell_list_load_game(e: &mut Engine, this: Ptr, flags: u32, _unused_2: u32) {
    if flags & 0x10 == 0 {
        return;
    }
    // Locals: +0 block size (u16), +4 marker, +8 count (u16), +0xC form id,
    // +0x10 spell, +0x14 leveled spell.
    e.with_stack(0x18, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u16(frame, 0);
        let mut block = 0u32;
        let slg = save_load_game(e);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![slg]).bool() {
            let slg = save_load_game(e);
            e.call(READ_BYTES, &args![slg, frame + 4, 4u32]);
            if e.mem.u32(frame + 4) != BLOCK_MARKER {
                let slg = save_load_game(e);
                let current = e.call(LOADING_FORM, &args![slg]).u32();
                if current != 0 {
                    let id = e.mem.u32(current);
                    let form = e.call(LOOKUP_FORM, &args![id]).u32();
                    report_loading_form(e, current, form, TEXT_BAD_HEADER_FOR_FORM, None, 0x55b);
                } else {
                    let version = save_version(e);
                    e.call(
                        PRINT_WARNING,
                        &args![TEXT_BAD_HEADER, SOURCE_FILE, 0x55bu32, version],
                    );
                }
            }
            let slg = save_load_game(e);
            block = e.call(BUFFER_POSITION, &args![slg]).u32();
            let slg = save_load_game(e);
            e.call(READ_BYTES, &args![slg, frame, 2u32]);
        }
        e.call(LIST_CLEAR, &args![this.addr().wrapping_add(4)]);
        e.call(LIST_CLEAR, &args![this.addr().wrapping_add(0xc)]);
        let slg = save_load_game(e);
        e.call(READ_BYTES, &args![slg, frame + 8, 2u32]);
        let mut index = 0i32;
        while index < i32::from(e.mem.u16(frame + 8)) {
            let slg = save_load_game(e);
            e.call(LOAD_NUMERIC_ID, &args![slg, frame + 0xc, 4u32]);
            let id = e.mem.u32(frame + 0xc);
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let spell = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_SPELL_ITEM, 0u32],
                )
                .u32();
            e.mem.set_u32(frame + 0x10, spell);
            let id = e.mem.u32(frame + 0xc);
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let leveled = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_LEV_SPELL, 0u32],
                )
                .u32();
            e.mem.set_u32(frame + 0x14, leveled);
            if spell != 0 {
                e.call(LIST_ADD, &args![this.addr().wrapping_add(4), frame + 0x10]);
            } else if leveled != 0 {
                let mut add = true;
                if save_version(e) < 0x47
                    && e.call(
                        LIST_CONTAINS,
                        &args![this.addr().wrapping_add(0xc), frame + 0x14],
                    )
                    .bool()
                {
                    add = false;
                }
                if add {
                    e.call(
                        LIST_ADD,
                        &args![this.addr().wrapping_add(0xc), frame + 0x14],
                    );
                }
            } else {
                let id = e.mem.u32(frame + 0xc);
                e.call(PRINT_WARNING, &args![TEXT_NO_SPELL, id]);
            }
            index += 1;
        }
        let slg = save_load_game(e);
        if !e.call(USE_SAVE_GAME_BLOCKS, &args![slg]).bool() {
            return;
        }
        let slg = save_load_game(e);
        let now = e.call(BUFFER_POSITION, &args![slg]).u32();
        let slg = save_load_game(e);
        let current = e.call(LOADING_FORM, &args![slg]).u32();
        let expected = |e: &mut Engine| u32::from(e.mem.u16(frame)).wrapping_add(block);
        if current != 0 {
            let id = e.mem.u32(current);
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            if now > expected(e) {
                let amount = now.wrapping_sub(expected(e));
                report_loading_form(e, current, form, TEXT_OVERRUN_FOR_FORM, Some(amount), 0x57f);
            } else if now < expected(e) {
                let amount = expected(e).wrapping_sub(now);
                report_loading_form(
                    e,
                    current,
                    form,
                    TEXT_UNDERRUN_FOR_FORM,
                    Some(amount),
                    0x57f,
                );
            }
        } else if now > expected(e) {
            let version = save_version(e);
            let amount = now.wrapping_sub(expected(e));
            e.call(
                PRINT_WARNING,
                &args![TEXT_OVERRUN, amount, SOURCE_FILE, 0x57fu32, version],
            );
        } else if now < expected(e) {
            let version = save_version(e);
            let amount = expected(e).wrapping_sub(now);
            e.call(
                PRINT_WARNING,
                &args![TEXT_UNDERRUN, amount, SOURCE_FILE, 0x57fu32, version],
            );
        }
    });
}

// Translated from 0048e140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSpellList::LoadGame_ov2` (Xbox PDB, `RET 4`: the load buffer): when
/// the buffer's flag 0x10 is set (`00428110` copies its word to a local,
/// `004280f0` tests the bit), empties both lists (`00470470`) and reads two
/// runs of form ids, each preceded by its count
/// (`BGSLoadGameBuffer::LoadVariableSizedValue`): every form id read
/// (`LoadFormID`) is looked up (`004839c0`) and, cast to `SpellItem` (first
/// run) or `TESLevSpell` (second run), added to the spells or the leveled
/// spells when the cast succeeds.
pub fn tes_spell_list_load_game_ov2(e: &mut Engine, this: Ptr, buffer: u32) {
    let loads = e.with_stack(4, |e, local| {
        let flags = e.call(0x0042_8110, &args![buffer, local]).u32();
        e.call(0x0042_80f0, &args![flags, 0x10u32]).bool()
    });
    if !loads {
        return;
    }
    e.call(LIST_CLEAR, &args![this.addr().wrapping_add(4)]);
    e.call(LIST_CLEAR, &args![this.addr().wrapping_add(0xc)]);
    for (list, target) in [(4u32, TYPE_SPELL_ITEM), (0xc, TYPE_TES_LEV_SPELL)] {
        let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
        let mut index = 0u32;
        while index < count {
            e.with_stack(4, |e, slot| {
                let id = e.call(LOAD_FORM_ID, &args![buffer]).u32();
                let form = e.call(LOOKUP_FORM, &args![id]).u32();
                let cast = e
                    .call(
                        RT_DYNAMIC_CAST,
                        &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
                    )
                    .u32();
                e.mem.set_u32(slot.addr(), cast);
                if cast != 0 {
                    e.call(LIST_ADD, &args![this.addr().wrapping_add(list), slot]);
                }
            });
            index += 1;
        }
    }
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_d0d0, tes_spell_list_clear_data_component(Ptr)),
        entry!(0x0048d150, fn_0048d150(Ptr) -> Ptr),
        entry!(0x0048_d0f0, tes_spell_list_free_spell_list(Ptr)),
        entry!(0x0048_d170, tes_spell_list_add_spell(Ptr, u32) -> bool),
        entry!(0x0048_d290, tes_spell_list_add_spell_ov3(Ptr, u32) -> bool),
        entry!(0x0048_d400, tes_spell_list_init_item(Ptr, u32)),
        entry!(0x0048_d630, tes_spell_list_copy_component(Ptr, u32)),
        entry!(0x0048_e040, fn_0048e040(Ptr, u32)),
        entry!(0x0048_d210, tes_spell_list_add_spell_ov2(Ptr, u32) -> bool),
        entry!(0x0048_d330, tes_spell_list_save(Ptr)),
        entry!(0x0048_d700, tes_spell_list_compare_component(Ptr, u32) -> bool),
        entry!(0x0048_d830, fn_0048d830(Ptr, u32) -> bool),
        entry!(0x0048_d880, fn_0048d880(Ptr, u32) -> bool),
        entry!(0x0048_d8d0, fn_0048d8d0(Ptr, u32)),
        entry!(0x0048_d8f0, fn_0048d8f0(Ptr, u32) -> u16),
        entry!(0x0048_da20, tes_spell_list_save_game(Ptr, u32)),
        entry!(0x0048_dca0, tes_spell_list_load_game(Ptr, u32, u32)),
        entry!(0x0048_e140, tes_spell_list_load_game_ov2(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_member_at_c() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x0048_d150, &args![0x0000_5000u32]).u32(),
            0x0000_500c
        );
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
    fn fwd_a03_0048d0d0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0048_d0f0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_d0d0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0048_d0f0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod b024_tests {
    use crate::prelude::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// List doubles: a node is two words (item, next).
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(0x0071_7e50, |_, a| ret(a[0] + 4));
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0063_f7b0, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let (item, after) = (e.mem.u32(next), e.mem.u32(next + 4));
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
        e.register(0x005f_65d0, |e, a| {
            let want = e.mem.u32(a[1]);
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == want {
                    return ret(1);
                }
                node = e.mem.u32(node + 4);
            }
            ret(0)
        });
        e.register(0x005a_e3d0, |e, a| {
            let item = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) == 0 {
                e.mem.set_u32(a[0], item);
                return Ret::default();
            }
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let new = e.mem.alloc(8);
            e.mem.set_u32(new, item);
            e.mem.set_u32(node + 4, new);
            Ret::default()
        });
        e.register(0x0090_5330, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(0x0072_6c60, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e
    }

    /// Fills the inline list head at `head` with `items` (extra nodes allocated).
    fn fill(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    fn items(e: &Engine, head: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut node = head;
        while node != 0 && e.mem.u32(node) != 0 {
            out.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        out
    }

    #[test]
    fn at_0048d0f0_empties_both_lists() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x14);
        fill(&mut e, this + 4, &[1, 2, 3]);
        fill(&mut e, this + 0xc, &[4, 5]);
        e.call_log = Some(vec![]);
        e.call(0x0048_d0f0, &[this]);
        let log = e.call_log.take().unwrap();
        assert!(items(&e, this + 4).is_empty());
        assert!(items(&e, this + 0xc).is_empty());
        assert_eq!(only(&log, 0x0063_f7b0).len(), 5);
    }

    fn player_engine(player_list_owner: bool, tes_flag: u32) -> (Engine, u32) {
        let mut e = list_engine();
        let this = e.mem.alloc(0x14);
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global::<u32>(0x011d_ea3c, 0x0055_0000);
        e.set_global::<u32>(0x011c_3f2c, 0x0066_0000);
        let base = if player_list_owner {
            this - 0x7c
        } else {
            0x0077_0000
        };
        e.register_double(0x0041_81e0, move |_, _| ret(base));
        e.register_double(0x0045_16b0, move |_, _| ret(tes_flag));
        e.register(0x0094_c6c0, ok);
        (e, this)
    }

    #[test]
    fn at_0048d170_adds_new_spells_and_tells_the_player() {
        let (mut e, this) = player_engine(true, 0);
        fill(&mut e, this + 4, &[7]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0048_d170, &[this, 0]).bool());
        assert!(!e.call(0x0048_d170, &[this, 7]).bool());
        assert!(e.call(0x0048_d170, &[this, 9]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(items(&e, this + 4), vec![7, 9]);
        assert_eq!(only(&log, 0x0094_c6c0), vec![vec![0x0055_0000, 9]]);

        // The TES flag set: added, not told.
        let (mut e, this) = player_engine(true, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_d170, &[this, 9]).bool());
        assert!(only(&e.call_log.take().unwrap(), 0x0094_c6c0).is_empty());

        // Not the player's list.
        let (mut e, this) = player_engine(false, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_d170, &[this, 9]).bool());
        assert!(only(&e.call_log.take().unwrap(), 0x0094_c6c0).is_empty());
        assert_eq!(items(&e, this + 4), vec![9]);

        // A player with no base form: the list compared with is 0.
        let (mut e, this) = player_engine(false, 0);
        e.register(0x0041_81e0, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_d170, &[this, 9]).bool());
        assert!(only(&e.call_log.take().unwrap(), 0x0094_c6c0).is_empty());
    }

    /// Casts by a table: form 0x100 is a spell (0x101), 0x200 a leveled
    /// spell (0x201); the spell list 0 is cast to an actor base `base`.
    fn casts(e: &mut Engine) {
        e.register(0x00ec_43fb, |_, a| {
            ret(match (a[0], a[3]) {
                (0x100, 0x0118_3060) => 0x101,
                (0x200, 0x0118_7818) => 0x201,
                (0x300, 0x0118_46e8) => 0x301,
                _ => 0,
            })
        });
    }

    #[test]
    fn at_0048d290_casts_the_form() {
        let (mut e, this) = player_engine(false, 0);
        casts(&mut e);
        e.register(0x0048_d210, |_, _| ret(0x55));
        assert!(e.call(0x0048_d290, &[this, 0x100]).bool());
        assert_eq!(items(&e, this + 4), vec![0x101]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_d290, &[this, 0x200]).bool());
        assert!(!e.call(0x0048_d290, &[this, 0x999]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x0048_d210), vec![vec![this, 0x201]]);
        assert_eq!(
            only(&log, 0x00ec_43fb)[0],
            vec![0x200, 0, 0x0118_3028, 0x0118_3060, 0]
        );
        // The spell was already there: AddSpell's false comes back.
        assert!(!e.call(0x0048_d290, &[this, 0x100]).bool());

        // The TES flag: the form goes to AddSpell as it is; true.
        let (mut e, this) = player_engine(false, 1);
        casts(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_d290, &[this, 0x999]).bool());
        assert!(e.call(0x0048_d290, &[this, 0x999]).bool());
        assert!(only(&e.call_log.take().unwrap(), 0x00ec_43fb).is_empty());
        assert_eq!(items(&e, this + 4), vec![0x999]);
    }

    fn init_engine() -> (Engine, u32) {
        let (mut e, this) = player_engine(false, 0);
        e.register(0x0048_4e60, |_, _| ret(0x5000));
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        // Ids 0x0100_0001 (a spell), 0x0100_0002 (a leveled spell); others missing.
        e.register(0x0048_39c0, |_, a| {
            ret(match a[0] {
                0x0100_0001 => 0x100,
                0x0100_0002 => 0x200,
                _ => 0,
            })
        });
        casts(&mut e);
        e.register(0x0048_d210, ok);
        e.register(0x005b_5e40, ok);
        e.register(0x0047_4cb0, |_, a| ret(u32::from(a[0] == 0x0088_0000)));
        e.register(0x0084_e3a0, |_, _| ret(0x0bad));
        (e, this)
    }

    #[test]
    fn at_0048d400_resolves_the_spells() {
        let (mut e, this) = init_engine();
        // spell, missing, leveled, spell, missing (last).
        fill(&mut e, this + 4, &[1, 3, 2, 1, 4]);
        e.call_log = Some(vec![]);
        e.call(0x0048_d400, &[this, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(items(&e, this + 4), vec![0x101, 0x101]);
        assert!(only(&log, 0x0048_4e60).is_empty());
        assert_eq!(only(&log, 0x0048_5d50)[0][1], 0);
        assert_eq!(only(&log, 0x0048_d210), vec![vec![this, 0x201]]);
        // The middle missing one is removed in place, the last one through the
        // previous node.
        assert_eq!(only(&log, 0x0063_f7b0).len(), 2);
        assert_eq!(only(&log, 0x0090_5330).len(), 1);
        assert_eq!(
            only(&log, 0x005b_5e40),
            vec![
                vec![0x0101_d648, 0x0100_0003],
                vec![0x0101_d648, 0x0100_0004]
            ]
        );
        assert!(only(&log, 0x0094_c6c0).is_empty());
    }

    #[test]
    fn at_0048d400_warns_with_the_owner_and_tells_the_player() {
        // An owner with a name (slot 0x130) and one without.
        let (mut e, this) = init_engine();
        e.put_vtable(0x0090_0000, &[0x00a0_0000; 0x50]);
        e.mem.set_u32(0x0090_0130, 0x00a0_0130);
        e.register(0x00a0_0130, |_, _| ret(0x0012_3456));
        e.map(0x0088_0000, 0x1000);
        e.mem.set_u32(0x0088_0000, 0x0090_0000);
        fill(&mut e, this + 4, &[5, 1]);
        e.call_log = Some(vec![]);
        e.call(0x0048_d400, &[this, 0x0088_0000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, 0x0048_4e60),
            vec![vec![0x0088_0000, 0xffff_ffff]]
        );
        assert_eq!(only(&log, 0x0048_5d50)[0][1], 0x5000);
        assert_eq!(
            only(&log, 0x005b_5e40),
            vec![vec![0x0101_d6d8, 0x0100_0005, 0x0012_3456]]
        );
        assert_eq!(items(&e, this + 4), vec![0x101]);

        let (mut e, this) = init_engine();
        e.map(0x0089_0000, 0x1000);
        fill(&mut e, this + 4, &[6]);
        e.call_log = Some(vec![]);
        e.call(0x0048_d400, &[this, 0x0089_0000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, 0x005b_5e40),
            vec![vec![0x0101_d690, 0x0100_0006, 0x0bad]]
        );

        // The list belongs to the player's actor base: the player learns it.
        let (mut e, _) = init_engine();
        let list = 0x300;
        e.map(0, 0x1000);
        e.register(0x0041_81e0, |_, _| ret(0x301));
        fill(&mut e, list + 4, &[1]);
        e.call_log = Some(vec![]);
        e.call(0x0048_d400, &[list, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x0094_c6c0), vec![vec![0x0055_0000, 0x101]]);
    }

    #[test]
    fn at_0048d630_copies_both_lists() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x14);
        let other = e.mem.alloc(0x14);
        fill(&mut e, this + 4, &[1, 2]);
        fill(&mut e, other + 4, &[5, 0, 6]);
        fill(&mut e, other + 0xc, &[7]);
        e.register_double(0x00ec_43fb, move |_, a| {
            ret(if a[0] == other { other } else { 0 })
        });
        e.call(0x0048_d630, &[this, 0x1234]);
        assert_eq!(items(&e, this + 4), vec![1, 2]);
        e.call_log = Some(vec![]);
        e.call(0x0048_d630, &[this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, 0x00ec_43fb),
            vec![vec![other, 0, 0x0118_3040, 0x0118_6ba4, 0]]
        );
        assert_eq!(items(&e, this + 4), vec![5, 6]);
        assert_eq!(items(&e, this + 0xc), vec![7]);
    }

    #[test]
    fn at_0048e040_saves_both_lists_when_flag_10_is_set() {
        for (flag, saves) in [(0x10u32, true), (0xef, false)] {
            let mut e = list_engine();
            let this = e.mem.alloc(0x14);
            fill(&mut e, this + 4, &[5, 0, 6]);
            fill(&mut e, this + 0xc, &[7]);
            e.register(0x0042_8110, |e, a| {
                e.mem.set_u32(a[1], 0x77);
                ret(a[1])
            });
            e.register_double(0x0042_80f0, move |e, a| {
                assert_eq!(e.mem.u32(a[0]), 0x77);
                ret(u32::from(a[1] & flag != 0))
            });
            let mut start = 0;
            e.register_double(0x0086_5f20, move |_, _| {
                start += 0x10;
                ret(start)
            });
            e.register(0x0086_5df0, ok);
            e.register(0x0086_5ff0, ok);
            e.call_log = Some(vec![]);
            e.call(0x0048_e040, &[this, 0x4000]);
            let log = e.call_log.take().unwrap();
            assert_eq!(only(&log, 0x0042_80f0)[0][1], 0x10);
            if saves {
                assert_eq!(
                    only(&log, 0x0086_5df0),
                    vec![vec![0x4000, 5, 0], vec![0x4000, 6, 0], vec![0x4000, 7, 0]]
                );
                assert_eq!(
                    only(&log, 0x0086_5ff0),
                    vec![vec![0x4000, 2, 0x10], vec![0x4000, 1, 0x20]]
                );
            } else {
                assert!(only(&log, 0x0086_5f20).is_empty());
            }
        }
    }
}

#[cfg(test)]
mod n045_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// List doubles: a node is two words (item, next); a list's count walks
    /// its nonzero items.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(SPELL_LIST, |_, a| ret(a[0] + 4));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_CONTAINS, |e, a| {
            let want = e.mem.u32(a[1]);
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == want {
                    return ret(1);
                }
                node = e.mem.u32(node + 4);
            }
            ret(0)
        });
        e.register(LIST_ADD, |e, a| {
            let item = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) == 0 {
                e.mem.set_u32(a[0], item);
                return Ret::default();
            }
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let new = e.mem.alloc(8);
            e.mem.set_u32(new, item);
            e.mem.set_u32(node + 4, new);
            Ret::default()
        });
        e.register(LIST_COUNT, |e, a| {
            let mut node = a[0];
            let mut n = 0;
            while node != 0 {
                if e.mem.u32(node) != 0 {
                    n += 1;
                }
                node = e.mem.u32(node + 4);
            }
            ret(n)
        });
        e.register(LIST_CLEAR, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e
    }

    fn fill(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    fn items(e: &Engine, head: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut node = head;
        while node != 0 {
            if e.mem.u32(node) != 0 {
                out.push(e.mem.u32(node));
            }
            node = e.mem.u32(node + 4);
        }
        out
    }

    /// A player whose base form gives `base` (0x011dea3c mapped).
    fn with_player(e: &mut Engine, base: u32) {
        e.map(0x011d_e000, 0x1000);
        e.set_global::<u32>(PLAYER, 0x7700);
        e.register_double(PLAYER_BASE_FORM, move |_, _| ret(base));
    }

    #[test]
    fn add_spell_ov2_adds_a_leveled_spell_once_and_never_to_the_players_list() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x14);
        with_player(&mut e, 0x9000);
        assert!(!e.call(0x0048_d210, &[this, 0]).bool());
        assert!(e.call(0x0048_d210, &[this, 0x55]).bool());
        assert!(!e.call(0x0048_d210, &[this, 0x55]).bool());
        assert_eq!(items(&e, this + 0xc), vec![0x55]);
        let mut e = list_engine();
        let base = e.mem.alloc(0x100);
        with_player(&mut e, base);
        assert!(!e.call(0x0048_d210, &[base + 0x7c, 0x55]).bool());
        assert_eq!(e.mem.u32(base + 0x7c + 0xc), 0);
    }

    #[test]
    fn save_writes_splo_chunks_until_an_empty_item() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x14);
        let forms: Vec<u32> = (0..4).map(|_| e.mem.alloc(0x10)).collect();
        for (i, f) in forms.iter().enumerate() {
            e.mem.set_u32(f + 0xc, 0x100 + i as u32);
        }
        e.mem.set_u32(forms[1] + 8, 0x20);
        fill(&mut e, this + 4, &[forms[0], forms[1], 0, forms[2]]);
        fill(&mut e, this + 0xc, &[forms[3]]);
        e.register(FORM_FLAG_20, |e, a| {
            ret(u32::from(e.mem.u32(a[0] + 8) & 0x20 != 0))
        });
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(SAVE_FORM_ID_CHUNK, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_d330, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, SAVE_FORM_ID_CHUNK),
            vec![vec![CHUNK_SPLO, 0x100], vec![CHUNK_SPLO, 0x103]]
        );
    }

    #[test]
    fn compare_component_and_the_two_membership_tests() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x14);
        let other = e.mem.alloc(0x14);
        fill(&mut e, this + 4, &[1, 0, 2]);
        fill(&mut e, this + 0xc, &[7]);
        fill(&mut e, other + 4, &[2, 1]);
        fill(&mut e, other + 0xc, &[7]);
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        assert!(e.call(0x0048_d830, &[other, 1]).bool());
        assert!(!e.call(0x0048_d830, &[other, 7]).bool());
        assert!(e.call(0x0048_d880, &[other, 7]).bool());
        assert!(!e.call(0x0048_d880, &[other, 1]).bool());
        assert!(!e.call(0x0048_d700, &[this, other]).bool());
        assert!(e.call(0x0048_d700, &[this, 0]).bool());
        // A spell the other lacks, then an extra leveled spell there.
        fill(&mut e, this + 4, &[1, 0, 3]);
        assert!(e.call(0x0048_d700, &[this, other]).bool());
        fill(&mut e, this + 4, &[1, 0, 2]);
        fill(&mut e, other + 0xc, &[7, 8]);
        assert!(e.call(0x0048_d700, &[this, other]).bool());
    }

    #[test]
    fn fn_0048d8d0_hands_the_spells_word_to_00905330() {
        let mut e = list_engine();
        let seen = std::rc::Rc::new(std::cell::Cell::new(0));
        let s = seen.clone();
        e.register_double(NODE_REMOVE_AFTER, move |e, a| {
            assert_eq!(a[0], 0x5004);
            s.set(e.mem.u32(a[1]));
            Ret::default()
        });
        e.call(0x0048_d8d0, &[0x5000, 0x77]);
        assert_eq!(seen.get(), 0x77);
    }

    fn save_engine(blocks: bool, report: bool, current: u32) -> Engine {
        let mut e = list_engine();
        e.map(0x011d_e000, 0x1000);
        let slg = e.mem.alloc(0x100);
        e.set_global::<u32>(SAVE_LOAD_GAME, slg);
        e.register_double(USE_SAVE_GAME_BLOCKS, move |_, _| ret(u32::from(blocks)));
        let value = e.mem.alloc(4);
        e.mem.set_u8(value, u8::from(report));
        e.register_double(SETTING_VALUE_POINTER, move |_, a| {
            assert_eq!(a[0], SAVE_REPORT_SETTING);
            ret(value)
        });
        e.register_double(SAVING_FORM, move |_, _| ret(current));
        e.register_double(LOADING_FORM, move |_, _| ret(current));
        e.register(ERROR, |_, _| Ret::default());
        e.register(PRINT_WARNING, |_, _| Ret::default());
        e
    }

    /// A form being written or read: id 0x1234, word 0xAABBCCDD at +5, byte
    /// 0x11 at +9; the looked-up form's slot 0x130 gives 0x6000.
    fn current_form(e: &mut Engine) -> u32 {
        let current = e.mem.alloc(0x10);
        e.mem.set_u32(current, 0x1234);
        e.mem.set_u32(current + 5, 0xaabb_ccdd);
        e.mem.set_u8(current + 9, 0x11);
        let form = e.mem.alloc(8);
        let vt = e.mem.alloc(0x140);
        e.mem.set_u32(form, vt);
        e.mem.set_u32(vt + 0x130, 0x0050_0000);
        e.register(0x0050_0000, |_, _| ret(0x6000));
        e.register_double(LOOKUP_FORM, move |_, a| {
            assert_eq!(a[0], 0x1234);
            ret(form)
        });
        current
    }

    #[test]
    fn fn_0048d8f0_sizes_the_lists_and_reports() {
        let mut e = save_engine(true, false, 0);
        let this = e.mem.alloc(0x14);
        fill(&mut e, this + 4, &[1, 2]);
        fill(&mut e, this + 0xc, &[3]);
        assert_eq!(e.call(0x0048_d8f0, &[this, 0]).u16(), 0);
        assert_eq!(e.call(0x0048_d8f0, &[this, 0x10]).u16(), 6 + 2 + 12);
        let mut e = save_engine(false, true, 0);
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_d8f0, &[this, 0x10]).u16(), 2);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, ERROR),
            vec![vec![TEXT_SIZE, 2, 0x51b, SOURCE_FILE]]
        );
        let mut e = save_engine(false, true, 0);
        let this = e.mem.alloc(0x14);
        let current = current_form(&mut e);
        e.register_double(SAVING_FORM, move |_, _| ret(current));
        e.call_log = Some(vec![]);
        e.call(0x0048_d8f0, &[this, 0x10]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, ERROR),
            vec![vec![
                TEXT_SIZE_FOR_FORM,
                2,
                0x1234,
                0x6000,
                0xaabb_ccdd,
                0x51b,
                SOURCE_FILE
            ]]
        );
    }

    /// Save buffer doubles: writes append to a byte buffer at `out`, whose
    /// position is kept at slg+0x14.
    fn writer(e: &mut Engine, out: u32) {
        let slg = e.global::<u32>(SAVE_LOAD_GAME);
        e.mem.set_u32(slg + 0x14, out);
        e.register(BUFFER_POSITION, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        fn write(e: &mut Engine, a: &[u32]) -> Ret {
            let pos = e.mem.u32(a[0] + 0x14);
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(pos, &bytes);
            e.mem.set_u32(a[0] + 0x14, pos + a[2]);
            Ret::default()
        }
        e.register(WRITE_BYTES, write);
        e.register(SAVE_NUMERIC_ID, write);
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
    }

    #[test]
    fn save_game_writes_a_block_with_the_count_and_ids() {
        let mut e = save_engine(true, true, 0);
        let out = e.mem.alloc(0x40);
        writer(&mut e, out);
        let this = e.mem.alloc(0x14);
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        e.mem.set_u32(a + 0xc, 0x0a0a);
        e.mem.set_u32(b + 0xc, 0x0b0b);
        fill(&mut e, this + 4, &[a, 0]);
        fill(&mut e, this + 0xc, &[b]);
        e.call(0x0048_da20, &[this, 0]);
        assert_eq!(e.mem.u32(out), 0);
        e.call_log = Some(vec![]);
        e.call(0x0048_da20, &[this, 0x10]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(out), BLOCK_MARKER);
        assert_eq!(e.mem.u16(out + 4), 12);
        assert_eq!(e.mem.u16(out + 6), 2);
        assert_eq!(e.mem.u32(out + 8), 0x0a0a);
        assert_eq!(e.mem.u32(out + 12), 0x0b0b);
        assert_eq!(
            only(&log, ERROR),
            vec![vec![TEXT_SAVED, 16, 0x54f, SOURCE_FILE]]
        );
        assert!(only(&log, PRINT_WARNING).is_empty());
    }

    #[test]
    fn save_game_warns_about_a_block_over_0xffff_bytes() {
        let mut e = save_engine(true, false, 0);
        let out = e.mem.alloc(0x20);
        writer(&mut e, out);
        e.register(SAVE_NUMERIC_ID, |e, a| {
            let pos = e.mem.u32(a[0] + 0x14);
            e.mem.set_u32(a[0] + 0x14, pos + 0x10000);
            Ret::default()
        });
        let this = e.mem.alloc(0x14);
        let a = e.mem.alloc(0x10);
        fill(&mut e, this + 4, &[a]);
        e.call_log = Some(vec![]);
        e.call(0x0048_da20, &[this, 0x10]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, PRINT_WARNING),
            vec![vec![TEXT_BLOCK_TOO_LARGE, SOURCE_FILE, 0x54f]]
        );
        assert_eq!(e.mem.u16(out + 4), 4);
    }

    /// Load doubles: reads take bytes from `input` (position at slg+0x14).
    fn reader(e: &mut Engine, input: u32, version: u8) {
        let slg = e.global::<u32>(SAVE_LOAD_GAME);
        e.mem.set_u32(slg + 0x14, input);
        e.mem.set_u8(slg + 0x80, version);
        e.register(BUFFER_POSITION, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        e.register(SAVE_VERSION, |e, a| ret(u32::from(e.mem.u8(a[0] + 0x80))));
        fn read(e: &mut Engine, a: &[u32]) -> Ret {
            let pos = e.mem.u32(a[0] + 0x14);
            let bytes = e.mem.bytes(pos, a[2]);
            e.mem.write(a[1], &bytes);
            e.mem.set_u32(a[0] + 0x14, pos + a[2]);
            Ret::default()
        }
        e.register(READ_BYTES, read);
        e.register(LOAD_NUMERIC_ID, read);
        // Ids below 0x100 are spells, 0x100.. leveled spells, 0x200 missing.
        e.register(LOOKUP_FORM, |_, a| ret(a[0]));
        e.register(RT_DYNAMIC_CAST, |_, a| {
            let id = a[0];
            let spell = id != 0 && id < 0x100;
            let leveled = (0x100..0x200).contains(&id);
            match a[3] {
                TYPE_SPELL_ITEM if spell => ret(id),
                TYPE_TES_LEV_SPELL if leveled => ret(id),
                _ => ret(0),
            }
        });
    }

    #[test]
    fn load_game_reads_the_ids_back() {
        let mut e = save_engine(true, false, 0);
        let input = e.mem.alloc(0x40);
        e.mem.set_u32(input, BLOCK_MARKER);
        e.mem.set_u16(input + 4, 20);
        e.mem.set_u16(input + 6, 4);
        for (i, id) in [5u32, 0x150, 0x200, 0x160].iter().enumerate() {
            e.mem.set_u32(input + 8 + 4 * i as u32, *id);
        }
        reader(&mut e, input, 0x40);
        let this = e.mem.alloc(0x14);
        fill(&mut e, this + 4, &[9]);
        e.call(0x0048_dca0, &[this, 0, 0]);
        assert_eq!(e.mem.u32(this + 4), 9);
        e.call_log = Some(vec![]);
        e.call(0x0048_dca0, &[this, 0x10, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(items(&e, this + 4), vec![5]);
        assert_eq!(items(&e, this + 0xc), vec![0x150, 0x160]);
        assert_eq!(only(&log, PRINT_WARNING), vec![vec![TEXT_NO_SPELL, 0x200]]);
    }

    #[test]
    fn load_game_skips_held_leveled_spells_in_old_saves_and_reports_bad_blocks() {
        let mut e = save_engine(true, false, 0);
        let input = e.mem.alloc(0x40);
        e.mem.set_u32(input, 0x1111);
        e.mem.set_u16(input + 4, 2);
        e.mem.set_u16(input + 6, 2);
        e.mem.set_u32(input + 8, 0x150);
        e.mem.set_u32(input + 12, 0x150);
        reader(&mut e, input, 0x46);
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(0x0048_dca0, &[this, 0x10, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(items(&e, this + 0xc), vec![0x150]);
        assert_eq!(
            only(&log, PRINT_WARNING),
            vec![
                vec![TEXT_BAD_HEADER, SOURCE_FILE, 0x55b, 0x46],
                vec![TEXT_OVERRUN, 10, SOURCE_FILE, 0x57f, 0x46],
            ]
        );
    }

    #[test]
    fn load_game_reports_with_the_form_being_read() {
        let mut e = save_engine(true, false, 0);
        let input = e.mem.alloc(0x40);
        e.mem.set_u32(input, 0x1111);
        e.mem.set_u16(input + 4, 0x20);
        e.mem.set_u16(input + 6, 0);
        reader(&mut e, input, 0x50);
        let current = current_form(&mut e);
        e.register_double(LOADING_FORM, move |_, _| ret(current));
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(0x0048_dca0, &[this, 0x10, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, PRINT_WARNING),
            vec![
                vec![
                    TEXT_BAD_HEADER_FOR_FORM,
                    SOURCE_FILE,
                    0x55b,
                    0x1234,
                    0x6000,
                    0x11,
                    0xaabb_ccdd
                ],
                vec![
                    TEXT_UNDERRUN_FOR_FORM,
                    0x1c,
                    SOURCE_FILE,
                    0x57f,
                    0x1234,
                    0x6000,
                    0x11,
                    0xaabb_ccdd
                ],
            ]
        );
    }

    #[test]
    fn load_game_ov2_reads_two_runs_of_form_ids() {
        let mut e = list_engine();
        e.register(0x0042_8110, |_, _| ret(0x10));
        e.register(0x0042_80f0, |_, a| ret(u32::from(a[0] & a[1] != 0)));
        let mut counts = vec![2u32, 1].into_iter();
        e.register_double(LOAD_VARIABLE_SIZED_VALUE, move |_, _| {
            ret(counts.next().unwrap())
        });
        let mut ids = vec![5u32, 0x150, 0x160].into_iter();
        e.register_double(LOAD_FORM_ID, move |_, _| ret(ids.next().unwrap()));
        e.register(LOOKUP_FORM, |_, a| ret(a[0]));
        e.register(RT_DYNAMIC_CAST, |_, a| match a[3] {
            TYPE_SPELL_ITEM if a[0] < 0x100 => ret(a[0]),
            TYPE_TES_LEV_SPELL if a[0] >= 0x100 => ret(a[0]),
            _ => ret(0),
        });
        let this = e.mem.alloc(0x14);
        fill(&mut e, this + 4, &[9]);
        e.call(0x0048_e140, &[this, 0x4000]);
        assert_eq!(items(&e, this + 4), vec![5]);
        assert_eq!(items(&e, this + 0xc), vec![0x160]);
        e.register(0x0042_8110, |_, _| ret(0));
        e.call(0x0048_e140, &[this, 0x4000]);
        assert_eq!(items(&e, this + 4), vec![5]);
    }
}
