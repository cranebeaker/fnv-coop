//! `fallout shared/tesfaction.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `005a8080` (`ECX` = faction): the address of the rank list (+0x3C).
const RANK_LIST: u32 = 0x005a_8080;
/// `006815c0` (`ECX` = node): the address of the node's item.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`ECX` = node): the next node, or 0.
const NODE_NEXT: u32 = 0x0072_6070;
/// `008256d0` (`ECX` = node): true when the node holds no item and has no
/// next node (an empty list).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `00905820` (`ECX` = node, the address of an item): adds the item after
/// the node.
const LIST_ADD: u32 = 0x0090_5820;
/// `0084e3a0` (`ECX` = form): the form id.
const FORM_ID: u32 = 0x0084_e3a0;
/// The form's name virtual slot.
const NAME_SLOT: u32 = 0x130;
/// The `TESDataHandler` singleton pointer.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `"DEFAULT: (Faction Reaction Error) Faction '%s' (%08X) is a Friend or
/// Ally of Faction '%s' (%08X), but Faction '%s' (%08X) is not ..."`.
const REACTION_ERROR: u32 = 0x0104_9a18;

// Translated from 005fd3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The faction's modifier toward `form`: `0048bf50` on the
/// `TESReactionForm` at +0x24.
pub fn fn_005fd3b0(e: &mut Engine, this: Ptr, form: u32) -> u32 {
    e.call(0x0048_bf50, &args![this.addr() + 0x24, form]).u32()
}

// Translated from 005fd430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the faction's reaction type toward `form` (`0048c220` on the
/// `TESReactionForm` at +0x24 with `reaction`), then vtable slot 0x48
/// (`AddChange` in the Xbox PDB's `TESForm` vtable) with 4.
pub fn fn_005fd430(e: &mut Engine, this: Ptr, form: u32, reaction: u32) {
    e.call(0x0048_c220, &args![this.addr() + 0x24, form, reaction]);
    e.vcall(this.addr(), 0x48, &args![4u32]);
}

// Translated from 005fcf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFaction::InitItem` (Xbox PDB): unless form flag 0x8 is set
/// (`004013e0`), resolves the reactions (`TESReactionForm::InitItem`,
/// `0048bc30`, with this faction as owner), sets form flag 0x8
/// (`00484ab0`) and, with a reputation form id at +0x38, gives it the
/// compile index of the faction's file and stores
/// `TESDataHandler::GetReputation` (`00461630`, on the handler at
/// `011c3f2c`) for it.
pub fn tes_faction_init_item(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    e.call(0x0048_bc30, &args![base + 0x24, this]);
    e.call(0x0048_4ab0, &args![this, true]);
    if e.mem.u32(base + 0x38) == 0 {
        return;
    }
    let id = e.with_stack(4, |e, id_at| {
        let id = e.mem.u32(base + 0x38);
        e.mem.set_u32(id_at.addr(), id);
        let file = e.call(0x0048_4e60, &args![this, u32::MAX]).u32();
        e.call(0x0048_5d50, &args![id_at, file]);
        e.mem.u32(id_at.addr())
    });
    let handler: u32 = e.global(DATA_HANDLER);
    let reputation = e.call(0x0046_1630, &args![handler, id]).u32();
    e.mem.set_u32(base + 0x38, reputation);
}

// Translated from 005fd460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fd460 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// Adds a new rank at the end of the faction's rank list and returns how
/// many ranks there were before: a new 0x1C-byte rank (`00401000`,
/// constructed by `005fcf10`, null when the allocation fails); the list
/// is walked while the node is not null and not empty (`008256d0`),
/// counting; the rank goes after the last node walked (`00905820`), or,
/// with none, into the list through `005ae3d0` (both with the address of a
/// stack copy of the rank).
pub fn fn_005fd460(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(0x0040_1000, &args![0x1cu32]).u32();
    let rank = if block != 0 {
        e.call(0x005f_cf10, &args![block]).u32()
    } else {
        0
    };
    let mut count = 0u32;
    let mut node = e.call(RANK_LIST, &args![this]).u32();
    let mut last = 0u32;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        count = count.wrapping_add(1);
        last = node;
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.with_stack(4, |e, rank_at| {
        e.mem.set_u32(rank_at.addr(), rank);
        if last != 0 {
            e.call(LIST_ADD, &args![last, rank_at]);
        } else {
            let list = e.call(RANK_LIST, &args![this]).u32();
            e.call(0x005a_e3d0, &args![list, rank_at]);
        }
    });
    count
}

// Translated from 005fd690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the faction into the save-game buffer `buffer`: the form's own
/// part (`00484d60`), the reactions (`0048c9d0` on +0x24), the 4-byte
/// `FACTION_DATA` (+0x34) when bit 0x2 of the buffer's flags word is set,
/// and the two crime words (+0x44, +0x48) when bit 0x80000000 is set (each
/// through `00865e50` with 4 bytes and 0; the flags word is copied to a
/// stack word by `00428110` and tested by `004280f0`).
pub fn fn_005fd690(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    e.call(0x0048_4d60, &args![this, buffer]);
    e.call(0x0048_c9d0, &args![base + 0x24, buffer]);
    let flag_set = |e: &mut Engine, mask: u32| {
        e.with_stack(4, |e, flags_at| {
            let flags = e.call(0x0042_8110, &args![buffer, flags_at]).u32();
            e.call(0x0042_80f0, &args![flags, mask]).bool()
        })
    };
    if flag_set(e, 2) {
        e.call(0x0086_5e50, &args![buffer, base + 0x34, 4u32, 0u32]);
    }
    if flag_set(e, 0x8000_0000) {
        e.call(0x0086_5e50, &args![buffer, base + 0x44, 4u32, 0u32]);
        e.call(0x0086_5e50, &args![buffer, base + 0x48, 4u32, 0u32]);
    }
}

// Translated from 005fd850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFaction::CheckFactionReactions` (Xbox PDB, `cdecl`, no arguments):
/// for every faction of the data handler's list (`00871a30` on the handler
/// at `011c3f2c`) and every reaction of it whose form is a faction (form
/// type 8, `00401170`) with reaction type 3 or 2, checks that the other
/// faction's reaction type toward this one (`0048c1b0`) is 2 or 3; if not,
/// prints the "Faction Reaction Error" message (twelve names and form ids,
/// fetched in the exe's order) and makes the other faction's reaction type
/// 3 (`005fd430`).
pub fn tes_faction_check_faction_reactions(e: &mut Engine) {
    let handler: u32 = e.global(DATA_HANDLER);
    let mut node = e.call(0x0087_1a30, &args![handler]).u32();
    while node != 0 {
        let at = e.call(NODE_ITEM, &args![node]).u32();
        let faction = e.mem.u32(at);
        if faction != 0 {
            let mut reaction_node = e.call(0x0071_7e50, &args![faction + 0x24]).u32();
            while reaction_node != 0 {
                let at = e.call(NODE_ITEM, &args![reaction_node]).u32();
                let reaction = e.mem.u32(at);
                if reaction != 0 {
                    check_reaction(e, faction, reaction);
                }
                reaction_node = e.call(NODE_NEXT, &args![reaction_node]).u32();
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

/// One reaction of `CheckFactionReactions`.
fn check_reaction(e: &mut Engine, faction: u32, reaction: u32) {
    let other = e.mem.u32(reaction);
    if other == 0 || e.call(0x0040_1170, &args![other]).u32() != 8 {
        return;
    }
    let kind = e.mem.u32(reaction + 8);
    if kind != 3 && kind != 2 {
        return;
    }
    let back = e.call(0x0048_c1b0, &args![other + 0x24, faction]).u32();
    if back == 2 || back == 3 {
        return;
    }
    let id = |e: &mut Engine, form: u32| e.call(FORM_ID, &args![form]).u32();
    let name = |e: &mut Engine, form: u32| e.vcall(form, NAME_SLOT, &[]).u32();
    // The words in push order (last argument first).
    let pushed = [
        id(e, faction),
        name(e, faction),
        name(e, faction),
        name(e, other),
        id(e, faction),
        name(e, faction),
        id(e, other),
        name(e, other),
        id(e, other),
        name(e, other),
        id(e, faction),
        name(e, faction),
    ];
    let mut words = vec![REACTION_ERROR];
    words.extend(pushed.iter().rev());
    e.call(0x005b_5e40, &words);
    fn_005fd430(e, Ptr::new(other), faction, 3);
}

/// `__RTDynamicCast` (`00ec43fb`) from `TESForm` (`01183028`) to
/// `TESFaction` (`01184704`), the type descriptors' names read at +8.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_FORM: u32 = 0x0118_3028;
const TYPE_FACTION: u32 = 0x0118_4704;
/// `TESForm::CopyAllComponents` (`004851b0`), `TESForm::CompareAllComponents`
/// (`00485270`), the copy `00401460` and `memcmp` (`00ec4835`, both
/// `cdecl (a, b, size)`).
const FORM_COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const FORM_COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
const COPY_BYTES: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;
/// The faction getters of the other faction: its data (`006a9540`, the
/// address of the four bytes at +0x34) and its reputation (`009e32d0`, the
/// word at +0x38); the reputation setter `0040e500` (`thiscall`, one word)
/// and `005fc860` (`thiscall`, run on the copy before the ranks).
const FACTION_DATA: u32 = 0x006a_9540;
const FACTION_REPUTATION: u32 = 0x009e_32d0;
const FACTION_SET_REPUTATION: u32 = 0x0040_e500;
const FACTION_005FC860: u32 = 0x005f_c860;
/// The ranks: the allocator (`00401000`, `cdecl (size)`), the rank
/// constructor (`005fcf10`), a string's length (`004048e0`) and text
/// (`00559450`), `BSStringT::Set` (`004037f0`, `thiscall (string, text,
/// length)`) and `BSStringT::StrCmp` (`00408a80`, `thiscall (string, text,
/// flag)`).
const ALLOCATE: u32 = 0x0040_1000;
const RANK_CTOR: u32 = 0x005f_cf10;
const STRING_LENGTH: u32 = 0x0040_48e0;
const STRING_TEXT: u32 = 0x0055_9450;
const STRING_SET: u32 = 0x0040_37f0;
const STRING_COMPARE: u32 = 0x0040_8a80;
/// The reaction setters on the `TESReactionForm` at +0x24 (`0048bfc0`,
/// `0048c0b0`, `thiscall`, two words each).
const REACTION_0048BFC0: u32 = 0x0048_bfc0;
const REACTION_0048C0B0: u32 = 0x0048_c0b0;
/// The change-flag steps of the form base and of the reaction part at
/// +0x24: size (`00484bf0`, `0048c2e0`), save (`00484c20`, `0048c400`),
/// load (`00484c50`, `0048c640`, two words), `008d0600` and `0048c930`
/// (two words), the buffer loads (`00484da0`, `0048ca90`) and `004534f0`
/// with `0048cb40` (one word each); `TESForm::SaveGameDataOLD`
/// (`00484ce0`) and `TESForm::LoadGameDataOLD` (`00484d00`), `thiscall
/// (form, data, size)`.
const FORM_CHANGED_SIZE: u32 = 0x0048_4bf0;
const REACTION_CHANGED_SIZE: u32 = 0x0048_c2e0;
const FORM_SAVE_GAME: u32 = 0x0048_4c20;
const REACTION_SAVE_GAME: u32 = 0x0048_c400;
const FORM_LOAD_GAME: u32 = 0x0048_4c50;
const REACTION_LOAD_GAME: u32 = 0x0048_c640;
const FORM_008D0600: u32 = 0x008d_0600;
const REACTION_0048C930: u32 = 0x0048_c930;
const FORM_LOAD_GAME_BUFFER: u32 = 0x0048_4da0;
const REACTION_LOAD_GAME_BUFFER: u32 = 0x0048_ca90;
const FORM_004534F0: u32 = 0x0045_34f0;
const REACTION_0048CB40: u32 = 0x0048_cb40;
const FORM_SAVE_GAME_DATA: u32 = 0x0048_4ce0;
const FORM_LOAD_GAME_DATA: u32 = 0x0048_4d00;
/// The load-game buffer: the change flags object (`00428110`, one word: a
/// stack word), its variant `0042ce30` (one word), the flag test
/// (`004280f0`, one word) and the read (`00864980`, `thiscall (buffer,
/// data, size)`).
const BUFFER_FLAGS: u32 = 0x0042_8110;
const BUFFER_FLAGS_0042CE30: u32 = 0x0042_ce30;
const FLAGS_TEST: u32 = 0x0042_80f0;
const BUFFER_READ: u32 = 0x0086_4980;

/// `other` cast from `TESForm` to `TESFaction`.
fn cast_faction(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![other, 0u32, TYPE_FORM, TYPE_FACTION, 0u32],
    )
    .u32()
}

/// Whether two rank strings differ the way the comparison tests them: when
/// either is non-empty, a different length or a non-zero `StrCmp` (flag 0).
fn rank_strings_differ(e: &mut Engine, mine: u32, theirs: u32) -> bool {
    if e.call(STRING_LENGTH, &args![mine]).u32() == 0
        && e.call(STRING_LENGTH, &args![theirs]).u32() == 0
    {
        return false;
    }
    let length = e.call(STRING_LENGTH, &args![mine]).u32();
    if length != e.call(STRING_LENGTH, &args![theirs]).u32() {
        return true;
    }
    let text = e.call(STRING_TEXT, &args![theirs]).u32();
    e.call(STRING_COMPARE, &args![mine, text, 0u32]).u32() != 0
}

// Translated from 005fd000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; the comparison of a
/// `TESFaction`): true (different) when `other` is none, when
/// `CompareAllComponents` says so, when the four data bytes at +0x34 or
/// the reputation (+0x38) differ, or at the first difference between the
/// rank lists: a rank of this faction with no rank (or a null one) at the
/// same place in `other`, either of the two rank titles (+0, +8), or the
/// insignia (+0x10, its virtual slot +0xC); or when `other` has more ranks.
pub fn fn_005fd000(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let source = cast_faction(e, other);
    if source == 0 {
        return true;
    }
    let at = this.addr();
    if e.call(FORM_COMPARE_ALL_COMPONENTS, &args![this, source])
        .u8()
        != 0
    {
        return true;
    }
    let data = e.call(FACTION_DATA, &args![source]).u32();
    if e.call(MEMCMP, &args![at + 0x34, data, 4u32]).u32() != 0 {
        return true;
    }
    let reputation = e.call(FACTION_REPUTATION, &args![source]).u32();
    if e.mem.u32(at + 0x38) != reputation {
        return true;
    }
    let mut mine = e.call(RANK_LIST, &args![this]).u32();
    let mut theirs = e.call(RANK_LIST, &args![source]).u32();
    while mine != 0 && e.call(LIST_IS_EMPTY, &args![mine]).u8() == 0 {
        if theirs == 0 {
            return true;
        }
        let item = e.call(NODE_ITEM, &args![theirs]).u32();
        if e.mem.u32(item) == 0 {
            return true;
        }
        let item = e.call(NODE_ITEM, &args![mine]).u32();
        let my_rank = e.mem.u32(item);
        let item = e.call(NODE_ITEM, &args![theirs]).u32();
        let their_rank = e.mem.u32(item);
        if rank_strings_differ(e, my_rank, their_rank) {
            return true;
        }
        if rank_strings_differ(e, my_rank + 8, their_rank + 8) {
            return true;
        }
        if e.vcall(my_rank + 0x10, 0xc, &args![their_rank + 0x10]).u8() != 0 {
            return true;
        }
        mine = e.call(NODE_NEXT, &args![mine]).u32();
        theirs = e.call(NODE_NEXT, &args![theirs]).u32();
    }
    theirs != 0 && e.call(LIST_IS_EMPTY, &args![theirs]).u8() == 0
}

// Translated from 005fd210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fd210 - the C++ exception frame (FS:[0], state for the rank allocation) is not translated
/// Unnamed in the engine map (`thiscall`, one word; the copy of a
/// `TESFaction`): when `other` is one, `CopyAllComponents` from `other`
/// (the word as given), its four data bytes and reputation (`0040e500`),
/// `005fc860`, then for each rank of `other` (while the node is not null
/// and not empty) a new rank (0x1C bytes, `005fcf10`, or null) with both
/// titles (`BSStringT::Set` with length 0) and the insignia (virtual slot
/// +8) copied, added after this list's node (`00905820`), which moves on
/// when it has a next node.
pub fn fn_005fd210(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = cast_faction(e, other);
    if source == 0 {
        return;
    }
    let at = this.addr();
    e.call(FORM_COPY_ALL_COMPONENTS, &args![this, other]);
    let data = e.call(FACTION_DATA, &args![source]).u32();
    e.call(COPY_BYTES, &args![at + 0x34, data, 4u32]);
    let reputation = e.call(FACTION_REPUTATION, &args![source]).u32();
    e.call(FACTION_SET_REPUTATION, &args![this, reputation]);
    e.call(FACTION_005FC860, &args![this]);
    let mut mine = e.call(RANK_LIST, &args![this]).u32();
    let mut theirs = e.call(RANK_LIST, &args![source]).u32();
    while theirs != 0 && e.call(LIST_IS_EMPTY, &args![theirs]).u8() == 0 {
        let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
        let rank = if block != 0 {
            e.call(RANK_CTOR, &args![block]).u32()
        } else {
            0
        };
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), rank);
            let item = e.call(NODE_ITEM, &args![theirs]).u32();
            let their_rank = e.mem.u32(item);
            let text = e.call(STRING_TEXT, &args![their_rank]).u32();
            e.call(STRING_SET, &args![rank, text, 0u32]);
            let item = e.call(NODE_ITEM, &args![theirs]).u32();
            let their_rank = e.mem.u32(item);
            let text = e.call(STRING_TEXT, &args![their_rank + 8]).u32();
            e.call(STRING_SET, &args![rank.wrapping_add(8), text, 0u32]);
            let item = e.call(NODE_ITEM, &args![theirs]).u32();
            let their_rank = e.mem.u32(item);
            e.vcall(rank.wrapping_add(0x10), 8, &args![their_rank + 0x10]);
            e.call(LIST_ADD, &args![mine, slot]);
        });
        if e.call(NODE_NEXT, &args![mine]).u32() != 0 {
            mine = e.call(NODE_NEXT, &args![mine]).u32();
        }
        theirs = e.call(NODE_NEXT, &args![theirs]).u32();
    }
}

// Translated from 005fd3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words): `0048bfc0` on the
/// `TESReactionForm` at +0x24 with both words, then virtual slot +0x48
/// with 4.
pub fn fn_005fd3d0(e: &mut Engine, this: Ptr, form: u32, value: u32) {
    e.call(REACTION_0048BFC0, &args![this.addr() + 0x24, form, value]);
    e.vcall(this.addr(), 0x48, &args![4u32]);
}

// Translated from 005fd400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words): `0048c0b0` on the
/// `TESReactionForm` at +0x24 with both words, then virtual slot +0x48
/// with 4.
pub fn fn_005fd400(e: &mut Engine, this: Ptr, form: u32, value: u32) {
    e.call(REACTION_0048C0B0, &args![this.addr() + 0x24, form, value]);
    e.vcall(this.addr(), 0x48, &args![4u32]);
}

// Translated from 005fd550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: change flags): the
/// size of the changed data, the form base's (`00484bf0`) plus the
/// reaction part's (`0048c2e0`) plus 4 with flag 2, in 16 bits.
pub fn fn_005fd550(e: &mut Engine, this: Ptr, flags: u32) -> u16 {
    let mut size = e.call(FORM_CHANGED_SIZE, &args![this, flags]).u16();
    let reactions = e
        .call(REACTION_CHANGED_SIZE, &args![this.addr() + 0x24, flags])
        .u16();
    size = size.wrapping_add(reactions);
    if flags & 2 != 0 {
        size = size.wrapping_add(4);
    }
    size
}

// Translated from 005fd5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: change flags): saves
/// the form base (`00484c20`) and the reaction part (`0048c400`), then
/// with flag 2 the four bytes at +0x34 (`TESForm::SaveGameDataOLD`).
pub fn fn_005fd5c0(e: &mut Engine, this: Ptr, flags: u32) {
    e.call(FORM_SAVE_GAME, &args![this, flags]);
    e.call(REACTION_SAVE_GAME, &args![this.addr() + 0x24, flags]);
    if flags & 2 != 0 {
        e.call(FORM_SAVE_GAME_DATA, &args![this, this.addr() + 0x34, 4u32]);
    }
}

// Translated from 005fd610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: change flags and a
/// second word): loads the form base (`TESForm::LoadGame`) and the
/// reaction part (`0048c640`), then with flag 2 the four bytes at +0x34
/// (`TESForm::LoadGameDataOLD`).
pub fn fn_005fd610(e: &mut Engine, this: Ptr, flags: u32, second: u32) {
    e.call(FORM_LOAD_GAME, &args![this, flags, second]);
    e.call(
        REACTION_LOAD_GAME,
        &args![this.addr() + 0x24, flags, second],
    );
    if flags & 2 != 0 {
        e.call(FORM_LOAD_GAME_DATA, &args![this, this.addr() + 0x34, 4u32]);
    }
}

// Translated from 005fd660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words): `008d0600` on the
/// form, then `0048c930` on the reaction part, both with the two words.
pub fn fn_005fd660(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(FORM_008D0600, &args![this, first, second]);
    e.call(REACTION_0048C930, &args![this.addr() + 0x24, first, second]);
}

// Translated from 005fd730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: the load-game
/// buffer): loads the form base (`00484da0`) and the reaction part
/// (`0048ca90`); when the change flags have 2 (`00428110`, `004280f0`),
/// four bytes into +0x34; when they have 0x80000000, four bytes each into
/// +0x44 and +0x48 (`00864980`).
pub fn fn_005fd730(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let at = this.addr();
    e.call(FORM_LOAD_GAME_BUFFER, &args![this, buffer]);
    e.call(REACTION_LOAD_GAME_BUFFER, &args![at + 0x24, buffer]);
    let has = |e: &mut Engine, flag: u32| {
        e.with_stack(4, |e, scratch| {
            let flags = e.call(BUFFER_FLAGS, &args![buffer, scratch]).u32();
            e.call(FLAGS_TEST, &args![flags, flag]).u8()
        })
    };
    if has(e, 2) != 0 {
        e.call(BUFFER_READ, &args![buffer, at + 0x34, 4u32]);
    }
    if has(e, 0x8000_0000) != 0 {
        e.call(BUFFER_READ, &args![buffer, at + 0x44, 4u32]);
        e.call(BUFFER_READ, &args![buffer, at + 0x48, 4u32]);
    }
}

// Translated from 005fd7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): `004534f0` on the
/// form, then `0048cb40` on the reaction part, both with the word.
pub fn fn_005fd7d0(e: &mut Engine, this: Ptr, value: u32) {
    e.call(FORM_004534F0, &args![this, value]);
    e.call(REACTION_0048CB40, &args![this.addr() + 0x24, value]);
}

// Translated from 005fd800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: the load-game
/// buffer): `004534f0` on the form, then, when the buffer's change flags
/// (`0042ce30` with a stack word, `004280f0`) have 0x80000000, clears the
/// words at +0x44 and +0x48.
pub fn fn_005fd800(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(FORM_004534F0, &args![this, buffer]);
    let changed = e.with_stack(4, |e, scratch| {
        let flags = e.call(BUFFER_FLAGS_0042CE30, &args![buffer, scratch]).u32();
        e.call(FLAGS_TEST, &args![flags, 0x8000_0000u32]).u8()
    });
    if changed != 0 {
        e.mem.set_u32(this.addr() + 0x44, 0);
        e.mem.set_u32(this.addr() + 0x48, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_d3b0, fn_005fd3b0(Ptr, u32) -> u32),
        entry!(0x005f_d430, fn_005fd430(Ptr, u32, u32)),
        entry!(0x005f_cf80, tes_faction_init_item(Ptr)),
        entry!(0x005f_d460, fn_005fd460(Ptr) -> u32),
        entry!(0x005f_d690, fn_005fd690(Ptr, Ptr)),
        entry!(0x005f_d850, tes_faction_check_faction_reactions()),
        entry!(0x005f_d000, fn_005fd000(Ptr, Ptr) -> bool),
        entry!(0x005f_d210, fn_005fd210(Ptr, Ptr)),
        entry!(0x005f_d3d0, fn_005fd3d0(Ptr, u32, u32)),
        entry!(0x005f_d400, fn_005fd400(Ptr, u32, u32)),
        entry!(0x005f_d550, fn_005fd550(Ptr, u32) -> u16),
        entry!(0x005f_d5c0, fn_005fd5c0(Ptr, u32)),
        entry!(0x005f_d610, fn_005fd610(Ptr, u32, u32)),
        entry!(0x005f_d660, fn_005fd660(Ptr, u32, u32)),
        entry!(0x005f_d730, fn_005fd730(Ptr, Ptr)),
        entry!(0x005f_d7d0, fn_005fd7d0(Ptr, u32)),
        entry!(0x005f_d800, fn_005fd800(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != skip)
            .collect()
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    #[test]
    fn b020_005fd3b0_asks_the_reaction_form() {
        let mut e = Engine::new();
        e.register(0x0048_bf50, |_, a| ret(a[0] + a[1]));
        assert_eq!(e.call(0x005f_d3b0, &args![0x1000u32, 5u32]).u32(), 0x1029);
    }

    #[test]
    fn b020_005fd430_sets_and_marks_changed() {
        let mut e = Engine::new();
        nop(&mut e, &[0x0048_c220, 0x0000_1048]);
        e.put_vtable(0x0150_0000, &[0; 0x20]);
        e.mem.set_u32(0x0150_0048, 0x0000_1048);
        let p = e.mem.alloc(0x4c);
        e.mem.set_u32(p, 0x0150_0000);
        e.call_log = Some(vec![]);
        e.call(0x005f_d430, &args![p, 0x77u32, 3u32]);
        assert_eq!(
            calls(&mut e, 0x005f_d430),
            vec![
                (0x0048_c220, vec![p + 0x24, 0x77, 3]),
                (0x0000_1048, vec![p, 4])
            ]
        );
    }

    #[test]
    fn b020_005fcf80_resolves_reactions_and_reputation() {
        let mut e = Engine::new();
        e.register(0x0040_13e0, |_, _| ret(0));
        nop(&mut e, &[0x0048_bc30, 0x0048_4ab0]);
        e.register(0x0048_4e60, |_, _| ret(0x77));
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0046_1630, |_, a| ret(a[1] + a[0]));
        e.map(DATA_HANDLER, 4);
        e.set_global::<u32>(DATA_HANDLER, 0x10);
        let p = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        e.call(0x005f_cf80, &args![p]);
        assert_eq!(
            calls(&mut e, 0x005f_cf80),
            vec![
                (0x0040_13e0, vec![p]),
                (0x0048_bc30, vec![p + 0x24, p]),
                (0x0048_4ab0, vec![p, 1]),
            ]
        );
        e.mem.set_u32(p + 0x38, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x005f_cf80, &args![p]);
        assert_eq!(calls_to(&e, 0x0048_4e60), vec![vec![p, u32::MAX]]);
        assert_eq!(calls_to(&e, 0x0046_1630), vec![vec![0x10, 0x0100_0042]]);
        assert_eq!(e.mem.u32(p + 0x38), 0x0100_0052);
        e.register(0x0040_13e0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x005f_cf80, &args![p]);
        assert_eq!(calls(&mut e, 0x005f_cf80).len(), 1);
    }

    /// Nodes are (item, next) pairs.
    fn chain_engine() -> Engine {
        let mut e = Engine::new();
        e.register(RANK_LIST, |_, a| ret(a[0] + 0x3c));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        nop(&mut e, &[LIST_ADD, 0x005a_e3d0]);
        e
    }

    #[test]
    fn b020_005fd460_appends_a_rank() {
        let mut e = chain_engine();
        e.register(0x0040_1000, |_, _| ret(0x0160_0000));
        e.register(0x005f_cf10, |_, a| ret(a[0]));
        let p = e.mem.alloc(0x4c);
        // An empty list: the rank goes in through 005ae3d0.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_d460, &args![p]).u32(), 0);
        let inserted = calls_to(&e, 0x005a_e3d0);
        assert_eq!(inserted.len(), 1);
        assert_eq!(inserted[0][0], p + 0x3c);
        assert_eq!(e.mem.u32(inserted[0][1]), 0x0160_0000);
        // Two ranks: after the second node.
        let second = e.mem.alloc(8);
        e.mem.set_u32(p + 0x3c, 0x11);
        e.mem.set_u32(p + 0x40, second);
        e.mem.set_u32(second, 0x22);
        e.register(0x0040_1000, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_d460, &args![p]).u32(), 2);
        let added = calls_to(&e, LIST_ADD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], second);
        assert_eq!(e.mem.u32(added[0][1]), 0);
        assert!(calls_to(&e, 0x005f_cf10).is_empty());
    }

    #[test]
    fn b020_005fd690_saves_by_flags() {
        let mut e = Engine::new();
        nop(&mut e, &[0x0048_4d60, 0x0048_c9d0, 0x0086_5e50]);
        e.register(0x0042_8110, |e, a| {
            let flags = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], flags);
            ret(a[1])
        });
        e.register(0x0042_80f0, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) & a[1] != 0))
        });
        let p = e.mem.alloc(0x4c);
        let buffer = e.mem.alloc(0x40);
        for (flags, saved) in [
            (0u32, vec![]),
            (2, vec![0x34]),
            (0x8000_0000, vec![0x44, 0x48]),
            (0x8000_0002, vec![0x34, 0x44, 0x48]),
        ] {
            e.mem.set_u32(buffer + 0x17, flags);
            e.call_log = Some(vec![]);
            e.call(0x005f_d690, &args![p, buffer]);
            assert_eq!(calls_to(&e, 0x0048_4d60), vec![vec![p, buffer]]);
            assert_eq!(calls_to(&e, 0x0048_c9d0), vec![vec![p + 0x24, buffer]]);
            let want: Vec<Vec<u32>> = saved.iter().map(|o| vec![buffer, p + o, 4, 0]).collect();
            assert_eq!(calls_to(&e, 0x0086_5e50), want);
        }
    }

    #[test]
    fn b020_005fd850_fixes_one_sided_alliances() {
        let mut e = Engine::new();
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0071_7e50, |_, a| ret(a[0] + 4));
        e.register(0x0040_1170, |e, a| ret(u32::from(e.mem.u8(a[0] + 4))));
        e.register(FORM_ID, |_, a| ret(a[0] | 0x0900_0000));
        e.register(0x0000_1130, |_, a| ret(a[0] | 0x0500_0000));
        nop(&mut e, &[0x005b_5e40, 0x0048_c220, 0x0000_1048]);
        e.put_vtable(0x0150_0000, &[0; 0x50]);
        e.mem.set_u32(0x0150_0130, 0x0000_1130);
        e.mem.set_u32(0x0150_0048, 0x0000_1048);
        // Factions a (ally of b, type 3) and b (reaction 0 toward a); c is
        // not a faction (type 9); d is an ally of a with an answer of 2.
        let faction = |e: &mut Engine, kind: u8| {
            let f = e.mem.alloc(0x4c);
            e.mem.set_u32(f, 0x0150_0000);
            e.mem.set_u8(f + 4, kind);
            f
        };
        let a = faction(&mut e, 8);
        let b = faction(&mut e, 8);
        let c = faction(&mut e, 9);
        let d = faction(&mut e, 8);
        let reaction = |e: &mut Engine, form: u32, kind: u32| {
            let r = e.mem.alloc(0xc);
            e.mem.set_u32(r, form);
            e.mem.set_u32(r + 8, kind);
            r
        };
        let r1 = reaction(&mut e, b, 3);
        let r2 = reaction(&mut e, c, 3);
        let r3 = reaction(&mut e, d, 2);
        // a's reaction list at a + 0x28: r1, r2, r3, then a null item.
        e.mem.set_u32(a + 0x28, r1);
        let n2 = e.mem.alloc(8);
        let n3 = e.mem.alloc(8);
        let n4 = e.mem.alloc(8);
        e.mem.set_u32(a + 0x2c, n2);
        e.mem.set_u32(n2, r2);
        e.mem.set_u32(n2 + 4, n3);
        e.mem.set_u32(n3, r3);
        e.mem.set_u32(n3 + 4, n4);
        e.register_double(0x0048_c1b0, move |_, args| {
            ret(if args[0] == d + 0x24 { 2 } else { 0 })
        });
        // The handler's faction list: a null item, then a.
        e.map(DATA_HANDLER, 4);
        let handler = e.mem.alloc(0x100);
        e.set_global::<u32>(DATA_HANDLER, handler);
        e.register(0x0087_1a30, |_, h| ret(h[0] + 0x88));
        let second = e.mem.alloc(8);
        e.mem.set_u32(handler + 0x8c, second);
        e.mem.set_u32(second, a);
        e.call_log = Some(vec![]);
        e.call(0x005f_d850, &[]);
        let printed = calls_to(&e, 0x005b_5e40);
        let na = a | 0x0500_0000;
        let ia = a | 0x0900_0000;
        let nb = b | 0x0500_0000;
        let ib = b | 0x0900_0000;
        assert_eq!(
            printed,
            vec![vec![
                REACTION_ERROR,
                na,
                ia,
                nb,
                ib,
                nb,
                ib,
                na,
                ia,
                nb,
                na,
                na,
                ia
            ]]
        );
        assert_eq!(calls_to(&e, 0x0048_c220), vec![vec![b + 0x24, a, 3]]);
        assert_eq!(calls_to(&e, 0x0000_1048), vec![vec![b, 4]]);
        assert_eq!(
            calls_to(&e, 0x0048_c1b0),
            vec![vec![b + 0x24, a], vec![d + 0x24, a]]
        );
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    const STATE: u32 = 0x7300_0000;
    const INSIGNIA_COPY: u32 = 0x7100_0008;
    const INSIGNIA_COMPARE: u32 = 0x7100_000c;
    const SLOT_48: u32 = 0x7100_0048;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    /// Ranks are 0x1C bytes: two strings (+0, +8; their length is the word
    /// at +4 of each) and an insignia (+0x10) whose vtable answers through
    /// the two slots. List nodes are (item, next); the rank list of a
    /// faction starts at +0x3C.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(STATE, 0x40);
        e.register(RANK_LIST, |_, a| (a[0] + 0x3c).into_ret());
        e.register(LIST_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(STRING_LENGTH, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(STRING_TEXT, |_, a| (a[0] + 0x100).into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(FACTION_DATA, |_, a| (a[0] + 0x34).into_ret());
        e.register(FACTION_REPUTATION, |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        e
    }

    fn rank(e: &mut Engine, first: u32, second: u32) -> u32 {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, INSIGNIA_COPY);
        e.mem.set_u32(vtable + 0xc, INSIGNIA_COMPARE);
        let rank = e.mem.alloc(0x1c);
        e.mem.set_u32(rank + 4, first);
        e.mem.set_u32(rank + 0xc, second);
        e.mem.set_u32(rank + 0x10, vtable);
        rank
    }

    fn faction(e: &mut Engine, ranks: &[u32]) -> u32 {
        let this = e.mem.alloc(0x50);
        let mut node = this + 0x3c;
        for (index, rank) in ranks.iter().enumerate() {
            e.mem.set_u32(node, *rank);
            if index + 1 < ranks.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        this
    }

    #[test]
    fn compare_walks_both_rank_lists() {
        let mut e = engine();
        e.register(FORM_COMPARE_ALL_COMPONENTS, |e, _| {
            e.mem.u32(STATE).into_ret()
        });
        e.register(MEMCMP, |e, _| e.mem.u32(STATE + 4).into_ret());
        e.register(STRING_COMPARE, |e, _| e.mem.u32(STATE + 8).into_ret());
        e.register(INSIGNIA_COMPARE, |e, _| e.mem.u32(STATE + 0xc).into_ret());
        let a = rank(&mut e, 3, 0);
        let b = rank(&mut e, 3, 0);
        let this = faction(&mut e, &[a]);
        let other = faction(&mut e, &[b]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005f_d000, &args![this, other]).bool());
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, TYPE_FORM, TYPE_FACTION, 0]]
        );
        assert_eq!(calls(&e, MEMCMP), vec![vec![this + 0x34, other + 0x34, 4]]);
        // The first titles are compared, the empty second ones are not.
        assert_eq!(calls(&e, STRING_COMPARE), vec![vec![a, b + 0x100, 0]]);
        assert_eq!(calls(&e, INSIGNIA_COMPARE), vec![vec![a + 0x10, b + 0x10]]);
        for at in [STATE, STATE + 4, STATE + 8, STATE + 0xc] {
            e.mem.set_u32(at, 1);
            assert!(e.call(0x005f_d000, &args![this, other]).bool());
            e.mem.set_u32(at, 0);
        }
        e.mem.set_u32(other + 0x38, 9);
        assert!(e.call(0x005f_d000, &args![this, other]).bool());
        e.mem.set_u32(other + 0x38, 0);
        // A different title length; a missing rank; an extra rank.
        let c = rank(&mut e, 4, 0);
        let shorter = faction(&mut e, &[c]);
        assert!(e.call(0x005f_d000, &args![this, shorter]).bool());
        let empty = faction(&mut e, &[]);
        assert!(e.call(0x005f_d000, &args![this, empty]).bool());
        let longer = faction(&mut e, &[b, b]);
        assert!(e.call(0x005f_d000, &args![this, longer]).bool());
        assert!(!e.call(0x005f_d000, &args![empty, empty]).bool());
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x005f_d000, &args![this, other]).bool());
    }

    #[test]
    fn copy_builds_new_ranks() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                FORM_COPY_ALL_COMPONENTS,
                COPY_BYTES,
                FACTION_SET_REPUTATION,
                FACTION_005FC860,
                STRING_SET,
                INSIGNIA_COPY,
            ],
        );
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(RANK_CTOR, |e, a| {
            let vtable = e.mem.alloc(0x10);
            e.mem.set_u32(vtable + 8, INSIGNIA_COPY);
            e.mem.set_u32(a[0] + 0x10, vtable);
            a[0].into_ret()
        });
        e.register_double(LIST_ADD, |e, a| {
            let item = e.mem.u32(a[1]);
            let n = e.mem.u32(STATE);
            e.mem.set_u32(STATE + 4 + 4 * n, item);
            e.mem.set_u32(STATE, n + 1);
            Ret::default()
        });
        let a = rank(&mut e, 1, 1);
        let b = rank(&mut e, 2, 2);
        let this = faction(&mut e, &[]);
        let other = faction(&mut e, &[a, b]);
        e.mem.set_u32(other + 0x38, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x005f_d210, &args![this, other]);
        assert_eq!(calls(&e, FORM_COPY_ALL_COMPONENTS), vec![vec![this, other]]);
        assert_eq!(
            calls(&e, COPY_BYTES),
            vec![vec![this + 0x34, other + 0x34, 4]]
        );
        assert_eq!(calls(&e, FACTION_SET_REPUTATION), vec![vec![this, 0x77]]);
        assert_eq!(e.mem.u32(STATE), 2);
        let first = e.mem.u32(STATE + 4);
        assert_eq!(
            calls(&e, STRING_SET)[..2],
            [vec![first, a + 0x100, 0], vec![first + 8, a + 0x108, 0]]
        );
        assert_eq!(calls(&e, INSIGNIA_COPY)[0], vec![first + 0x10, a + 0x10]);
        assert!(calls(&e, LIST_ADD).iter().all(|c| c[0] == this + 0x3c));
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x005f_d210, &args![this, other]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn reaction_setters() {
        let mut e = engine();
        quiet(&mut e, &[REACTION_0048BFC0, REACTION_0048C0B0, SLOT_48]);
        let vtable = e.mem.alloc(0x50);
        e.mem.set_u32(vtable + 0x48, SLOT_48);
        let this = faction(&mut e, &[]);
        e.mem.set_u32(this, vtable);
        e.call_log = Some(vec![]);
        e.call(0x005f_d3d0, &args![this, 5u32, 6u32]);
        e.call(0x005f_d400, &args![this, 7u32, 8u32]);
        assert_eq!(calls(&e, REACTION_0048BFC0), vec![vec![this + 0x24, 5, 6]]);
        assert_eq!(calls(&e, REACTION_0048C0B0), vec![vec![this + 0x24, 7, 8]]);
        assert_eq!(calls(&e, SLOT_48), vec![vec![this, 4], vec![this, 4]]);
    }

    #[test]
    fn change_steps() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                FORM_SAVE_GAME,
                REACTION_SAVE_GAME,
                FORM_LOAD_GAME,
                REACTION_LOAD_GAME,
                FORM_008D0600,
                REACTION_0048C930,
                FORM_004534F0,
                REACTION_0048CB40,
                FORM_SAVE_GAME_DATA,
                FORM_LOAD_GAME_DATA,
            ],
        );
        e.register(FORM_CHANGED_SIZE, |_, _| 0x1_0010u32.into_ret());
        e.register(REACTION_CHANGED_SIZE, |_, _| 0xfff0u32.into_ret());
        // 0x10 + 0xfff0 wraps to 0 in 16 bits.
        assert_eq!(e.call(0x005f_d550, &args![0x1000u32, 2u32]).u16(), 4);
        assert_eq!(e.call(0x005f_d550, &args![0x1000u32, 1u32]).u16(), 0);
        e.call_log = Some(vec![]);
        e.call(0x005f_d5c0, &args![0x1000u32, 2u32]);
        e.call(0x005f_d610, &args![0x1000u32, 2u32, 9u32]);
        e.call(0x005f_d660, &args![0x1000u32, 3u32, 4u32]);
        e.call(0x005f_d7d0, &args![0x1000u32, 5u32]);
        assert_eq!(calls(&e, FORM_SAVE_GAME), vec![vec![0x1000, 2]]);
        assert_eq!(calls(&e, REACTION_SAVE_GAME), vec![vec![0x1024, 2]]);
        assert_eq!(
            calls(&e, FORM_SAVE_GAME_DATA),
            vec![vec![0x1000, 0x1034, 4]]
        );
        assert_eq!(calls(&e, FORM_LOAD_GAME), vec![vec![0x1000, 2, 9]]);
        assert_eq!(calls(&e, REACTION_LOAD_GAME), vec![vec![0x1024, 2, 9]]);
        assert_eq!(
            calls(&e, FORM_LOAD_GAME_DATA),
            vec![vec![0x1000, 0x1034, 4]]
        );
        assert_eq!(calls(&e, FORM_008D0600), vec![vec![0x1000, 3, 4]]);
        assert_eq!(calls(&e, REACTION_0048C930), vec![vec![0x1024, 3, 4]]);
        assert_eq!(calls(&e, FORM_004534F0), vec![vec![0x1000, 5]]);
        assert_eq!(calls(&e, REACTION_0048CB40), vec![vec![0x1024, 5]]);
        e.call_log = Some(vec![]);
        e.call(0x005f_d5c0, &args![0x1000u32, 1u32]);
        e.call(0x005f_d610, &args![0x1000u32, 1u32, 9u32]);
        assert!(calls(&e, FORM_SAVE_GAME_DATA).is_empty());
        assert!(calls(&e, FORM_LOAD_GAME_DATA).is_empty());
    }

    #[test]
    fn buffer_loads() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                FORM_LOAD_GAME_BUFFER,
                REACTION_LOAD_GAME_BUFFER,
                BUFFER_READ,
                FORM_004534F0,
            ],
        );
        e.register(BUFFER_FLAGS, |_, a| (a[0] + 0x40).into_ret());
        e.register(BUFFER_FLAGS_0042CE30, |_, a| (a[0] + 0x80).into_ret());
        // The flag test passes for the flags in the state word.
        e.register(FLAGS_TEST, |e, a| {
            u32::from(a[1] & e.mem.u32(STATE) != 0).into_ret()
        });
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(STATE, 0x8000_0002);
        e.call_log = Some(vec![]);
        e.call(0x005f_d730, &args![this, 0x2000u32]);
        assert_eq!(calls(&e, FORM_LOAD_GAME_BUFFER), vec![vec![this, 0x2000]]);
        assert_eq!(
            calls(&e, REACTION_LOAD_GAME_BUFFER),
            vec![vec![this + 0x24, 0x2000]]
        );
        assert_eq!(
            calls(&e, FLAGS_TEST),
            vec![vec![0x2040, 2], vec![0x2040, 0x8000_0000]]
        );
        assert_eq!(
            calls(&e, BUFFER_READ),
            vec![
                vec![0x2000, this + 0x34, 4],
                vec![0x2000, this + 0x44, 4],
                vec![0x2000, this + 0x48, 4]
            ]
        );
        e.mem.set_u32(STATE, 2);
        e.call_log = Some(vec![]);
        e.call(0x005f_d730, &args![this, 0x2000u32]);
        assert_eq!(calls(&e, BUFFER_READ).len(), 1);
        // 005fd800: clears +0x44 and +0x48 only with 0x80000000.
        e.mem.set_u32(this + 0x44, 1);
        e.mem.set_u32(this + 0x48, 1);
        e.call(0x005f_d800, &args![this, 0x2000u32]);
        assert_eq!((e.mem.u32(this + 0x44), e.mem.u32(this + 0x48)), (1, 1));
        e.mem.set_u32(STATE, 0x8000_0000);
        e.call_log = Some(vec![]);
        e.call(0x005f_d800, &args![this, 0x2000u32]);
        assert_eq!((e.mem.u32(this + 0x44), e.mem.u32(this + 0x48)), (0, 0));
        assert_eq!(calls(&e, FORM_004534F0), vec![vec![this, 0x2000]]);
        assert_eq!(calls(&e, FLAGS_TEST), vec![vec![0x2080, 0x8000_0000]]);
    }
}
