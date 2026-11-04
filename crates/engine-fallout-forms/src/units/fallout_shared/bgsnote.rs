//! `fallout shared/bgsnote.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSNote` (Xbox PDB) on PC (`TESForm` is 0x10 smaller there, so every
    /// field sits 0x10 lower; the PC code reads these offsets), 0x80 bytes.
    pub struct BGSNote: 0x80 {
        /// The union `pNoteSound` / `pNoteText` / `pNoteImage` / `pNoteTopic`
        /// (Xbox PDB), chosen by `cType`.
        0x6C pNoteData: u32,
        /// `pSpeaker` (Xbox PDB): `TESActorBase *`.
        0x70 pSpeaker: u32,
        /// `OwnerQuests` (Xbox PDB): a `BSSimpleList<TESQuest *>`.
        0x74 OwnerQuests: Inline<crate::types::BSSimpleList>,
        /// `cType` (Xbox PDB).
        0x7C cType: u8,
        /// `bHasBeenRead` (Xbox PDB).
        0x7D bHasBeenRead: u8,
    }
}

/// `00401000` / `00401030`: allocate / free (`cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `004f29c0` (`thiscall`, `RET 4`): the deleting destructor of a type-2
/// note's data.
const NOTE_DATA_DELETE: u32 = 0x004f_29c0;
/// The constructors of a type-1 note's data (`00482c50`, 8 bytes) and a
/// type-2 note's (`0048e270`, 12 bytes).
const DESCRIPTION_CONSTRUCT: u32 = 0x0048_2c50;
const TEXTURE_CONSTRUCT: u32 = 0x0048_e270;
/// `004013e0` (`thiscall`): a `bool` that makes `InitItem` do nothing.
const FORM_SKIPS_INIT: u32 = 0x0040_13e0;
/// `00484ab0` (`thiscall`, `RET 4`).
const FORM_SET_FLAG: u32 = 0x0048_4ab0;
/// `TESForm::GetFile` (`00484e60`, `RET 4`).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (`00485d50`, `cdecl`: the id's address, the
/// file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`cdecl`, one word): the form of a form id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words), its source type
/// descriptor and the targets `InitItem` casts to (quest, sound, topic,
/// actor base by the warnings that follow each).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_QUEST_TYPE: u32 = 0x0118_6500;
const CAST_SOUND_TYPE: u32 = 0x0118_32fc;
const CAST_TOPIC_TYPE: u32 = 0x0118_4720;
const CAST_ACTOR_BASE_TYPE: u32 = 0x0118_46e8;
/// `0084e3a0`: the word at +0xc (`TESForm::iFormID`).
const FORM_ID: u32 = 0x0084_e3a0;
/// The form vtable slot (byte offset) whose answer the warnings print as
/// `%s`.
const NAME_SLOT: u32 = 0x130;
/// `005b5e40` (`cdecl`, variable): prints a warning; the four formats
/// "MASTERFILE: Unable to find owner quest / sound / topic / actor base
/// (%08X) for Note '%s' (%08X)." (`01046ac0`, `01046a7c`, `01046a3c`,
/// `010469f8`).
const PRINT_WARNING: u32 = 0x005b_5e40;
const MISSING_QUEST_FORMAT: u32 = 0x0104_6ac0;
const MISSING_SOUND_FORMAT: u32 = 0x0104_6a7c;
const MISSING_TOPIC_FORMAT: u32 = 0x0104_6a3c;
const MISSING_ACTOR_BASE_FORMAT: u32 = 0x0104_69f8;
/// List helpers (on a node): empty (`008256d0`), the item's address
/// (`006815c0`), next (`00726070`), remove the head (`0063f7b0`), set the
/// item from an address (`00726c60`, `RET 4`) and remove the following item
/// (`00905330`, `RET 4`, that item's address).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
const NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
const NODE_SET_ITEM: u32 = 0x0072_6c60;
const LIST_REMOVE_AFTER: u32 = 0x0090_5330;
/// `005e9130` (`RET 4`): sets a type-0 note's sound.
const NOTE_SET_SOUND: u32 = 0x005e_9130;
/// `BGSPickupPutdownSounds::InitItem` (`0047b6a0`, `RET 4`).
const SOUNDS_INIT_ITEM: u32 = 0x0047_b6a0;

// Translated from 005e8d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `cType`, sign-extended (`MOVSX`).
pub fn fn_005e8d40(e: &mut Engine, this: Ptr<BGSNote>) -> i32 {
    i32::from(e.get(this, BGSNote::cType) as i8)
}

/// The note data when the type is `note_type`, else null (the shape of
/// `005e9100`, `005e9160`, `005e91d0` and `005e9240`).
fn data_if_type(e: &mut Engine, this: Ptr<BGSNote>, note_type: i32) -> u32 {
    if fn_005e8d40(e, this) == note_type {
        e.get(this, BGSNote::pNoteData)
    } else {
        0
    }
}

// Translated from 005e9100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the note data of a type-0 note (its sound),
/// else null.
pub fn fn_005e9100(e: &mut Engine, this: Ptr<BGSNote>) -> u32 {
    data_if_type(e, this, 0)
}

// Translated from 005e9160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the note data of a type-2 note, else null.
pub fn fn_005e9160(e: &mut Engine, this: Ptr<BGSNote>) -> u32 {
    data_if_type(e, this, 2)
}

// Translated from 005e91d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the note data of a type-1 note, else null.
pub fn fn_005e91d0(e: &mut Engine, this: Ptr<BGSNote>) -> u32 {
    data_if_type(e, this, 1)
}

// Translated from 005e9240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the note data of a type-3 note (its topic),
/// else null.
pub fn fn_005e9240(e: &mut Engine, this: Ptr<BGSNote>) -> u32 {
    data_if_type(e, this, 3)
}

// Translated from 005e92a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSNote::GetNoteSpeaker` (engine map name): `pSpeaker` of a type-3 note,
/// else null.
pub fn bgs_note_get_note_speaker(e: &mut Engine, this: Ptr<BGSNote>) -> u32 {
    if fn_005e8d40(e, this) == 3 {
        e.get(this, BGSNote::pSpeaker)
    } else {
        0
    }
}

// Translated from 005e9270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): on a type-3 note, sets the note data
/// (its topic); otherwise nothing.
pub fn fn_005e9270(e: &mut Engine, this: Ptr<BGSNote>, topic: u32) {
    if fn_005e8d40(e, this) == 3 {
        e.set(this, BGSNote::pNoteData, topic);
    }
}

// Translated from 005e92d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): on a type-3 note, sets `pSpeaker`;
/// otherwise nothing.
pub fn fn_005e92d0(e: &mut Engine, this: Ptr<BGSNote>, speaker: u32) {
    if fn_005e8d40(e, this) == 3 {
        e.set(this, BGSNote::pSpeaker, speaker);
    }
}

// Translated from 005e8f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the note's type (`RET 4`). Nothing when it
/// equals `cType` (sign-extended). Otherwise the old data is released (type
/// 1 freed, type 2 deleted with `004f29c0` when there is one; both cleared),
/// the type byte is stored, and a type-1 note gets a new 8-byte description
/// (`00482c50`), a type-2 note a new 12-byte texture (`0048e270`), null
/// when the allocation failed.
// Translation incomplete: 005e8f90 - the SEH frame (FS:[0], unwind of the two allocations) is not translated.
pub fn fn_005e8f90(e: &mut Engine, this: Ptr<BGSNote>, note_type: u32) {
    if note_type == i32::from(e.get(this, BGSNote::cType) as i8) as u32 {
        return;
    }
    match e.get(this, BGSNote::cType) {
        1 => {
            let data = e.get(this, BGSNote::pNoteData);
            e.call(FREE, &args![data]);
            e.set(this, BGSNote::pNoteData, 0);
        }
        2 => {
            let data = e.get(this, BGSNote::pNoteData);
            if data != 0 {
                e.call(NOTE_DATA_DELETE, &args![data, 1u32]);
            }
            e.set(this, BGSNote::pNoteData, 0);
        }
        _ => {}
    }
    e.set(this, BGSNote::cType, note_type as u8);
    let (size, construct) = match e.get(this, BGSNote::cType) {
        1 => (8u32, DESCRIPTION_CONSTRUCT),
        2 => (0xc, TEXTURE_CONSTRUCT),
        _ => return,
    };
    let block = e.call(ALLOCATE, &args![size]).u32();
    let data = if block != 0 {
        e.call(construct, &args![block]).u32()
    } else {
        0
    };
    e.set(this, BGSNote::pNoteData, data);
}

/// Resolves a form id with the note's file's compile index (`00484e60` with
/// -1, `00485d50` on a stack copy), looks it up (`004839c0`) and casts it to
/// `target` (`00ec43fb`). Answers the cast and the id as adjusted.
fn resolve(e: &mut Engine, this: Ptr<BGSNote>, id: u32, target: u32) -> (u32, u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), id);
        let file = e.call(GET_FILE, &args![this, 0xffff_ffffu32]).u32();
        e.call(ADD_COMPILE_INDEX, &args![local, file]);
        let id = e.mem.u32(local.addr());
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let cast = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, CAST_SOURCE_TYPE, target, 0u32],
            )
            .u32();
        (cast, id)
    })
}

/// The warning for an id that did not resolve: the format, the id, the
/// note's name (vtable `+0x130`) and its form id (`0084e3a0`, called first).
fn warn_missing(e: &mut Engine, this: Ptr<BGSNote>, format: u32, id: u32) {
    let form_id = e.call(FORM_ID, &args![this]).u32();
    let name = e.vcall(this.addr(), NAME_SLOT, &args![]).u32();
    e.call(PRINT_WARNING, &args![format, id, name, form_id]);
}

// Translated from 005e8900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSNote::InitItem` (engine map name): unless `004013e0` answers true,
/// resolves the form ids read by the load. Each owner quest in
/// `OwnerQuests` is cast to a quest; a found one replaces the id in its node
/// (`00726c60`), a missing one is warned about and removed (after the
/// previous node, `00905330`, or as the head, `0063f7b0`). A type-0 note's
/// sound (`005e9130`), a type-3 note's topic (`005e9270`) and speaker
/// (`005e92d0`) are set to their resolved forms (null when missing, with a
/// warning). Then the pickup/putdown sounds' `InitItem` (`0047b6a0`) and
/// `00484ab0` with 1.
pub fn bgs_note_init_item(e: &mut Engine, this: Ptr<BGSNote>) {
    if e.call(FORM_SKIPS_INIT, &args![this]).bool() {
        return;
    }
    let mut node = this.addr().wrapping_add(0x74);
    let mut previous = 0;
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let id = e.mem.u32(item_address);
        let (quest, id) = resolve(e, this, id, CAST_QUEST_TYPE);
        if quest != 0 {
            e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), quest);
                e.call(NODE_SET_ITEM, &args![node, local]);
            });
            previous = node;
            node = e.call(NODE_NEXT, &args![node]).u32();
        } else {
            warn_missing(e, this, MISSING_QUEST_FORMAT, id);
            if previous != 0 {
                let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
                e.call(LIST_REMOVE_AFTER, &args![previous, item_address]);
                node = e.call(NODE_NEXT, &args![previous]).u32();
            } else {
                e.call(LIST_REMOVE_HEAD, &args![node]);
            }
        }
    }
    if fn_005e9100(e, this) != 0 {
        let id = fn_005e9100(e, this);
        let (sound, id) = resolve(e, this, id, CAST_SOUND_TYPE);
        e.call(NOTE_SET_SOUND, &args![this, sound]);
        if sound == 0 {
            warn_missing(e, this, MISSING_SOUND_FORMAT, id);
        }
    }
    if fn_005e9240(e, this) != 0 {
        let id = fn_005e9240(e, this);
        let (topic, id) = resolve(e, this, id, CAST_TOPIC_TYPE);
        fn_005e9270(e, this, topic);
        if topic == 0 {
            warn_missing(e, this, MISSING_TOPIC_FORMAT, id);
        }
    }
    if bgs_note_get_note_speaker(e, this) != 0 {
        let id = bgs_note_get_note_speaker(e, this);
        let (speaker, id) = resolve(e, this, id, CAST_ACTOR_BASE_TYPE);
        fn_005e92d0(e, this, speaker);
        if speaker == 0 {
            warn_missing(e, this, MISSING_ACTOR_BASE_FORMAT, id);
        }
    }
    e.call(
        SOUNDS_INIT_ITEM,
        &args![this.addr().wrapping_add(0x60), this],
    );
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
}

/// `BGSNote`'s RTTI type descriptor (`.?AVBGSNote@@`), the target of the
/// `CopyFrom`/`Compare` casts (source: `TESForm`, [`CAST_SOURCE_TYPE`]).
const TYPE_NOTE: u32 = 0x0118_640c;
/// `004fadc0`: the address of a note's owner-quest list (`ECX` = the note).
const OWNER_QUESTS_OF: u32 = 0x004f_adc0;
/// The player (`PlayerCharacter *`, `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;

// Translated from 005e8c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`), a copy from another form: nothing
/// unless `source` is a note (`__RTDynamicCast`). Copies the components
/// (`004851b0`), the type ([`fn_005e8f90`] with the source's
/// [`fn_005e8d40`]), the speaker ([`fn_005e92d0`]), the note data by this
/// note's (new) type (0 [`fn_005e9130`], 1 [`fn_005e9200`], 2
/// [`fn_005e9190`], 3 [`fn_005e9270`], each with the source's getter),
/// then clears `OwnerQuests` (`00470470`) and adds each of the source's
/// owner quests (`004fadc0`, nodes until empty) with `005ae3d0`, and
/// `bHasBeenRead`.
pub fn fn_005e8c00(e: &mut Engine, this: Ptr<BGSNote>, source: u32) {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, CAST_SOURCE_TYPE, TYPE_NOTE, 0u32],
        )
        .u32();
    if cast == 0 {
        return;
    }
    let other: Ptr<BGSNote> = Ptr::new(cast);
    e.call(0x0048_51b0, &args![this, source]);
    let note_type = fn_005e8d40(e, other);
    fn_005e8f90(e, this, note_type as u32);
    let speaker = bgs_note_get_note_speaker(e, other);
    fn_005e92d0(e, this, speaker);
    match fn_005e8d40(e, this) {
        0 => {
            let data = fn_005e9100(e, other);
            fn_005e9130(e, this, data);
        }
        1 => {
            let data = fn_005e91d0(e, other);
            fn_005e9200(e, this, data);
        }
        2 => {
            let data = fn_005e9160(e, other);
            fn_005e9190(e, this, data);
        }
        3 => {
            let data = fn_005e9240(e, other);
            fn_005e9270(e, this, data);
        }
        _ => {}
    }
    e.call(0x0047_0470, &args![this.addr() + 0x74]);
    let mut node = e.call(OWNER_QUESTS_OF, &args![other]).u32();
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let item = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        e.call(0x005a_e3d0, &args![this.addr() + 0x74, item]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let read = e.get(other, BGSNote::bHasBeenRead);
    e.set(this, BGSNote::bHasBeenRead, read);
}

// Translated from 005e8d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`), whether `other` differs: true
/// when it is not a note, its components differ (`00485270`), its type
/// differs, its data differs by this note's type (0: the sound; 1 and 2:
/// only one has data, or this one's slot 3 (+0xC) with the other's is
/// true; 3: the topic, then the speaker), its owner-quest count differs
/// (`005ae380` of `004fadc0`, this note's first) or one of this note's
/// owner quests is not in its list (`005f65d0` with a stack copy).
pub fn fn_005e8d60(e: &mut Engine, this: Ptr<BGSNote>, other: u32) -> bool {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, CAST_SOURCE_TYPE, TYPE_NOTE, 0u32],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    let theirs: Ptr<BGSNote> = Ptr::new(cast);
    if e.call(0x0048_5270, &args![this, cast]).bool() {
        return true;
    }
    let their_type = i32::from(e.get(theirs, BGSNote::cType) as i8);
    let my_type = i32::from(e.get(this, BGSNote::cType) as i8);
    if their_type != my_type {
        return true;
    }
    match my_type {
        0 => {
            if fn_005e9100(e, this) != fn_005e9100(e, theirs) {
                return true;
            }
        }
        1 | 2 => {
            let getter: fn(&mut Engine, Ptr<BGSNote>) -> u32 = if my_type == 1 {
                fn_005e91d0
            } else {
                fn_005e9160
            };
            let mine = getter(e, this);
            let other_data = getter(e, theirs);
            if mine != 0 && other_data != 0 {
                if e.vcall(mine, 0xc, &args![other_data]).bool() {
                    return true;
                }
            } else if mine != 0 || other_data != 0 {
                return true;
            }
        }
        3 => {
            if fn_005e9240(e, this) != fn_005e9240(e, theirs) {
                return true;
            }
            if bgs_note_get_note_speaker(e, this) != bgs_note_get_note_speaker(e, theirs) {
                return true;
            }
        }
        _ => {}
    }
    let list = e.call(OWNER_QUESTS_OF, &args![this]).u32();
    let my_count = e.call(0x005a_e380, &args![list]).u32();
    let list = e.call(OWNER_QUESTS_OF, &args![theirs]).u32();
    if my_count != e.call(0x005a_e380, &args![list]).u32() {
        return true;
    }
    let mut node = e.call(OWNER_QUESTS_OF, &args![this]).u32();
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let item = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let quest = e.mem.u32(item);
        let found = e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), quest);
            let list = e.call(OWNER_QUESTS_OF, &args![theirs]).u32();
            e.call(0x005f_65d0, &args![list, local]).bool()
        });
        if !found {
            return true;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 005e9130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): on a type-0 note, sets the note
/// data (its sound); otherwise nothing.
pub fn fn_005e9130(e: &mut Engine, this: Ptr<BGSNote>, sound: u32) {
    if fn_005e8d40(e, this) == 0 {
        e.set(this, BGSNote::pNoteData, sound);
    }
}

// Translated from 005e9190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): on a type-2 note, passes `value`
/// to the note data's slot 2 (+0x8).
pub fn fn_005e9190(e: &mut Engine, this: Ptr<BGSNote>, value: u32) {
    if fn_005e8d40(e, this) == 2 {
        let data = e.get(this, BGSNote::pNoteData);
        e.vcall(data, 0x8, &args![value]);
    }
}

// Translated from 005e9200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): on a type-1 note, passes `value`
/// to the note data's slot 2 (+0x8).
pub fn fn_005e9200(e: &mut Engine, this: Ptr<BGSNote>, value: u32) {
    if fn_005e8d40(e, this) == 1 {
        let data = e.get(this, BGSNote::pNoteData);
        e.vcall(data, 0x8, &args![value]);
    }
}

// Translated from 005e9300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`), sets `bHasBeenRead`: nothing when
/// it already equals `read`; otherwise stores it and calls the form's slot
/// 18 (+0x48) when set or slot 19 (+0x4C) when cleared, with `0x80000000`.
pub fn fn_005e9300(e: &mut Engine, this: Ptr<BGSNote>, read: u8) {
    if e.get(this, BGSNote::bHasBeenRead) == read {
        return;
    }
    e.set(this, BGSNote::bHasBeenRead, read);
    let slot = if e.get(this, BGSNote::bHasBeenRead) != 0 {
        0x48
    } else {
        0x4c
    };
    e.vcall(this.addr(), slot, &args![0x8000_0000u32]);
}

// Translated from 005e9360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSNote::Activate` (engine map), `RET 0x14`: false unless `activator`
/// is the player (`011dea3c`); it is first cast from `TESObjectREFR` to
/// `Actor` (`__RTDynamicCast`, result unused). When the reference has an
/// owner (`00567790`) the player is not one of (`005785e0` with 1), the
/// player gets the karma of the setting at `011cde20` (`00403e20`, the
/// float truncated by `_ftol2`, `0094fd30`) when the owner is of the kind
/// `00401170` answers 8 for and `0047d7c0` is false, or otherwise when the
/// alignment (`0047e040`) of its slot 3 (+0xC) value 0x17 of the object at
/// +0x100 is neither 2 nor 4. Then the player adds the note (`00966a70`,
/// with 1), the reference is disabled (`00574400`) and it answers true.
/// The last three words are not read.
#[allow(clippy::too_many_arguments)]
pub fn bgs_note_activate(
    e: &mut Engine,
    this: Ptr<BGSNote>,
    reference: u32,
    activator: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> bool {
    e.call(
        DYNAMIC_CAST,
        &args![activator, 0u32, 0x0118_41ccu32, 0x0118_46d4u32, 0u32],
    );
    let mut rewards = false;
    if activator != e.mem.u32(PLAYER) {
        return false;
    }
    let owner = e.call(0x0056_7790, &args![reference]).u32();
    if owner != 0 {
        let player = e.mem.u32(PLAYER);
        if !e.call(0x0057_85e0, &args![reference, player, 1u32]).bool() {
            if e.call(0x0040_1170, &args![owner]).u32() == 8 {
                if !e.call(0x0047_d7c0, &args![owner]).bool() {
                    rewards = true;
                }
            } else if owner != 0 {
                let value = e
                    .vcall(owner.wrapping_add(0x100), 0xc, &args![0x17u32])
                    .f64();
                let alignment = e.call(0x0047_e040, &args![x87_store32(value)]).u32();
                if alignment != 2 && alignment != 4 {
                    rewards = true;
                }
            }
        }
    }
    if rewards {
        let setting = e.call(0x0040_3e20, &args![0x011c_de20u32]).u32();
        let karma = x87_load32(e.mem.u32(setting));
        let whole = e.call(0x00ec_62c0, &args![karma]).u32();
        let player = e.mem.u32(PLAYER);
        e.call(0x0094_fd30, &args![player, whole]);
    }
    let player = e.mem.u32(PLAYER);
    e.call(0x0096_6a70, &args![player, this, 1u32]);
    e.call(0x0057_4400, &args![reference]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_8d40, fn_005e8d40(Ptr<BGSNote>) -> i32),
        entry!(0x005e_9100, fn_005e9100(Ptr<BGSNote>) -> u32),
        entry!(0x005e_9160, fn_005e9160(Ptr<BGSNote>) -> u32),
        entry!(0x005e_91d0, fn_005e91d0(Ptr<BGSNote>) -> u32),
        entry!(0x005e_9240, fn_005e9240(Ptr<BGSNote>) -> u32),
        entry!(0x005e_92a0, bgs_note_get_note_speaker(Ptr<BGSNote>) -> u32),
        entry!(0x005e_9270, fn_005e9270(Ptr<BGSNote>, u32)),
        entry!(0x005e_92d0, fn_005e92d0(Ptr<BGSNote>, u32)),
        entry!(0x005e_8f90, fn_005e8f90(Ptr<BGSNote>, u32)),
        entry!(0x005e_8900, bgs_note_init_item(Ptr<BGSNote>)),
        entry!(0x005e_8c00, fn_005e8c00(Ptr<BGSNote>, u32)),
        entry!(0x005e_8d60, fn_005e8d60(Ptr<BGSNote>, u32) -> bool),
        entry!(0x005e_9130, fn_005e9130(Ptr<BGSNote>, u32)),
        entry!(0x005e_9190, fn_005e9190(Ptr<BGSNote>, u32)),
        entry!(0x005e_9200, fn_005e9200(Ptr<BGSNote>, u32)),
        entry!(0x005e_9300, fn_005e9300(Ptr<BGSNote>, u8)),
        entry!(
            0x005e_9360,
            bgs_note_activate(Ptr<BGSNote>, u32, u32, u32, u32, u32) -> bool
        ),
    ]
}

#[cfg(test)]
mod b025_tests {
    use super::*;

    const FORM_VTABLE: u32 = 0x00ab_0000;
    const NAME_FN: u32 = 0x00ab_1000;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn note(e: &mut Engine, note_type: u8, data: u32, speaker: u32) -> Ptr<BGSNote> {
        let n = e.new_object::<BGSNote>();
        e.mem.set_u32(n.addr(), FORM_VTABLE);
        e.set(n, BGSNote::cType, note_type);
        e.set(n, BGSNote::pNoteData, data);
        e.set(n, BGSNote::pSpeaker, speaker);
        n
    }

    #[test]
    fn typed_getters_and_setters() {
        let mut e = Engine::new();
        let getters = [0x005e_9100u32, 0x005e_91d0, 0x005e_9160, 0x005e_9240];
        for t in 0..4u8 {
            let n = note(&mut e, t, 0x50, 0x60);
            assert_eq!(e.call(0x005e_8d40, &args![n]).i32(), i32::from(t));
            for (i, g) in getters.iter().enumerate() {
                let expected = if i == usize::from(t) { 0x50 } else { 0 };
                assert_eq!(e.call(*g, &args![n]).u32(), expected);
            }
            assert_eq!(
                e.call(0x005e_92a0, &args![n]).u32(),
                if t == 3 { 0x60 } else { 0 }
            );
            e.call(0x005e_9270, &args![n, 0x77u32]);
            e.call(0x005e_92d0, &args![n, 0x88u32]);
            let set = t == 3;
            assert_eq!(e.get(n, BGSNote::pNoteData), if set { 0x77 } else { 0x50 });
            assert_eq!(e.get(n, BGSNote::pSpeaker), if set { 0x88 } else { 0x60 });
        }
        let n = note(&mut e, 0xff, 0x50, 0);
        assert_eq!(e.call(0x005e_8d40, &args![n]).i32(), -1);
    }

    #[test]
    fn fn_005e8f90_swaps_the_data_for_the_new_type() {
        let mut e = Engine::new();
        e.register(FREE, |_, _| Ret::default());
        e.register(NOTE_DATA_DELETE, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(DESCRIPTION_CONSTRUCT, |_, a| (a[0] + 1).into_ret());
        e.register(TEXTURE_CONSTRUCT, |_, a| (a[0] + 2).into_ret());
        // The same type: nothing (0xffffffff matches type 0xff sign-extended).
        let n = note(&mut e, 0xff, 0x50, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_8f90, &args![n, 0xffff_ffffu32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // 1 -> 2: the description is freed, a texture made.
        let n = note(&mut e, 1, 0x50, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_8f90, &args![n, 2u32]);
        assert_eq!(calls_to(&e, FREE), vec![vec![0x50]]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0xc]]);
        let block = calls_to(&e, TEXTURE_CONSTRUCT)[0][0];
        assert_eq!(e.get(n, BGSNote::pNoteData), block + 2);
        assert_eq!(e.get(n, BGSNote::cType), 2);
        // 2 -> 1: the texture is deleted, a description made.
        e.call_log = Some(vec![]);
        e.call(0x005e_8f90, &args![n, 1u32]);
        assert_eq!(calls_to(&e, NOTE_DATA_DELETE), vec![vec![block + 2, 1]]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![8]]);
        // 1 -> 3: freed, nothing made; the data stays cleared.
        e.call_log = Some(vec![]);
        e.call(0x005e_8f90, &args![n, 3u32]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert_eq!(e.get(n, BGSNote::pNoteData), 0);
        assert_eq!(e.get(n, BGSNote::cType), 3);
        // A failed allocation leaves null.
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x005e_8f90, &args![n, 1u32]);
        assert_eq!(e.get(n, BGSNote::pNoteData), 0);
    }

    fn init_engine() -> Engine {
        let mut e = Engine::new();
        e.register(FORM_SKIPS_INIT, |_, _| 0u32.into_ret());
        e.register(GET_FILE, |_, _| 0x7000u32.into_ret());
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let v = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], v | 0x0100_0000);
            Ret::default()
        });
        // Ids with bit 4 set are found (as 0x00f0_0000 + id).
        e.register(LOOKUP_FORM, |_, a| {
            if a[0] & 0x10 != 0 {
                (0x00f0_0000 + (a[0] & 0xff)).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(FORM_ID, |_, _| 0x0012_3456u32.into_ret());
        e.register(NAME_FN, |_, _| 0x0055_0000u32.into_ret());
        e.register(NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_SET_ITEM, |e, a| {
            let v = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], v);
            Ret::default()
        });
        // Removing after a node unlinks its next node.
        e.register(LIST_REMOVE_AFTER, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        // Removing the head pulls the second node's item and link up.
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        for addr in [
            PRINT_WARNING,
            NOTE_SET_SOUND,
            SOUNDS_INIT_ITEM,
            FORM_SET_FLAG,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let mut slots = vec![0u32; 0x4d];
        slots[0x130 / 4] = NAME_FN;
        e.put_vtable(FORM_VTABLE, &slots);
        e
    }

    /// Fills the note's quest list with `ids` (head node inside the note).
    fn quests(e: &mut Engine, n: Ptr<BGSNote>, ids: &[u32]) {
        let mut node = n.addr() + 0x74;
        for (i, id) in ids.iter().enumerate() {
            e.mem.set_u32(node, *id);
            if i + 1 < ids.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    #[test]
    fn init_item_resolves_quests_and_typed_forms() {
        let mut e = init_engine();
        // A missing head quest, a found one, a missing one after it.
        let n = note(&mut e, 0, 0x13, 0);
        quests(&mut e, n, &[0x01, 0x12, 0x03]);
        e.call_log = Some(vec![]);
        e.call(0x005e_8900, &args![n]);
        let a = n.addr();
        assert_eq!(e.mem.u32(a + 0x74), 0x00f0_0012);
        assert_eq!(e.mem.u32(a + 0x78), 0);
        assert_eq!(calls_to(&e, LIST_REMOVE_HEAD), vec![vec![a + 0x74]]);
        assert_eq!(calls_to(&e, LIST_REMOVE_AFTER).len(), 1);
        assert_eq!(calls_to(&e, NOTE_SET_SOUND), vec![vec![a, 0x00f0_0013]]);
        assert_eq!(
            calls_to(&e, PRINT_WARNING),
            vec![
                vec![MISSING_QUEST_FORMAT, 0x0100_0001, 0x0055_0000, 0x0012_3456],
                vec![MISSING_QUEST_FORMAT, 0x0100_0003, 0x0055_0000, 0x0012_3456],
            ]
        );
        assert_eq!(calls_to(&e, SOUNDS_INIT_ITEM), vec![vec![a + 0x60, a]]);
        assert_eq!(calls_to(&e, FORM_SET_FLAG), vec![vec![a, 1]]);
        // Type 3: a missing topic and a found speaker.
        let n = note(&mut e, 3, 0x04, 0x15);
        e.call_log = Some(vec![]);
        e.call(0x005e_8900, &args![n]);
        assert_eq!(e.get(n, BGSNote::pNoteData), 0);
        assert_eq!(e.get(n, BGSNote::pSpeaker), 0x00f0_0015);
        assert_eq!(
            calls_to(&e, PRINT_WARNING),
            vec![vec![
                MISSING_TOPIC_FORMAT,
                0x0100_0004,
                0x0055_0000,
                0x0012_3456
            ]]
        );
        assert_eq!(calls_to(&e, DYNAMIC_CAST)[0][3], CAST_TOPIC_TYPE);
        assert_eq!(calls_to(&e, DYNAMIC_CAST)[1][3], CAST_ACTOR_BASE_TYPE);
        // A missing speaker and a missing sound warn too.
        let n = note(&mut e, 3, 0, 0x05);
        e.call_log = Some(vec![]);
        e.call(0x005e_8900, &args![n]);
        assert_eq!(calls_to(&e, PRINT_WARNING)[0][0], MISSING_ACTOR_BASE_FORMAT);
        let n = note(&mut e, 0, 0x06, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_8900, &args![n]);
        assert_eq!(calls_to(&e, PRINT_WARNING)[0][0], MISSING_SOUND_FORMAT);
        assert_eq!(calls_to(&e, NOTE_SET_SOUND), vec![vec![n.addr(), 0]]);
        // Skipped entirely when 004013e0 says so.
        e.register(FORM_SKIPS_INIT, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x005e_8900, &args![n]);
        assert!(calls_to(&e, SOUNDS_INIT_ITEM).is_empty());
    }
}

#[cfg(test)]
mod n036_tests {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for callee in callees {
            e.register(*callee, |_, _| Ret::default());
        }
    }

    /// A cast that answers the object when its word +0x10 is 1.
    fn cast_engine() -> Engine {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |e, a| {
            (if e.mem.u32(a[0] + 0x10) == 1 { a[0] } else { 0 }).into_ret()
        });
        e
    }

    fn note(e: &mut Engine, note_type: u8, data: u32) -> Ptr<BGSNote> {
        let n = e.new_object::<BGSNote>();
        e.mem.set_u32(n.addr() + 0x10, 1);
        e.set(n, BGSNote::cType, note_type);
        e.set(n, BGSNote::pNoteData, data);
        n
    }

    #[test]
    fn copy_from_copies_type_data_quests_and_read_flag() {
        let mut e = cast_engine();
        quiet(&mut e, &[0x0048_51b0, 0x0047_0470, 0x005a_e3d0]);
        e.register(OWNER_QUESTS_OF, |_, a| (a[0] + 0x74).into_ret());
        e.register(NODE_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let source = note(&mut e, 3, 0x7700);
        e.set(source, BGSNote::pSpeaker, 0x7800);
        e.set(source, BGSNote::bHasBeenRead, 1);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x9002);
        e.mem.set_u32(source.addr() + 0x74, 0x9001);
        e.mem.set_u32(source.addr() + 0x78, second);
        let this = note(&mut e, 3, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_8c00, &args![this, source]);
        assert_eq!(e.get(this, BGSNote::pNoteData), 0x7700);
        assert_eq!(e.get(this, BGSNote::pSpeaker), 0x7800);
        assert_eq!(e.get(this, BGSNote::bHasBeenRead), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x005a_e3d0),
            vec![
                vec![this.addr() + 0x74, source.addr() + 0x74],
                vec![this.addr() + 0x74, second]
            ]
        );
        assert_eq!(
            calls_to(&log, 0x0048_51b0),
            vec![vec![this.addr(), source.addr()]]
        );

        // A type-0 source sets the sound; a non-note does nothing.
        let source = note(&mut e, 0, 0x5500);
        let this = note(&mut e, 0, 0);
        e.call(0x005e_8c00, &args![this, source]);
        assert_eq!(e.get(this, BGSNote::pNoteData), 0x5500);
        e.mem.set_u32(source.addr() + 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_8c00, &args![this, source]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn compare_checks_each_part() {
        let mut e = cast_engine();
        e.register(0x0048_5270, |e, a| {
            (e.mem.u32(a[0] + 0x20) != e.mem.u32(a[1] + 0x20)).into_ret()
        });
        e.register(OWNER_QUESTS_OF, |_, a| (a[0] + 0x74).into_ret());
        e.register(0x005a_e380, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_IS_EMPTY, |e, a| (e.mem.u32(a[0]) == 0).into_ret());
        e.register(NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |_, _| 0u32.into_ret());
        e.register(0x005f_65d0, |e, a| {
            (e.mem.u32(a[0]) == e.mem.u32(a[1])).into_ret()
        });
        e.put_vtable(0x0204_0000, &[0, 0, 0, 0x0090_000c]);
        e.register(0x0090_000c, |_, a| (a[1] == 0x6666).into_ret());
        let data = e.mem.alloc(4);
        e.mem.set_u32(data, 0x0204_0000);
        let a = note(&mut e, 1, data);
        let b = note(&mut e, 1, 0x1111);
        let compare = |e: &mut Engine| e.call(0x005e_8d60, &args![a, b]).bool();
        assert!(!compare(&mut e));
        e.set(b, BGSNote::pNoteData, 0x6666);
        assert!(compare(&mut e));
        e.set(b, BGSNote::pNoteData, 0);
        assert!(compare(&mut e));
        e.set(b, BGSNote::pNoteData, 0x1111);
        // Owner quests: same first quest, then another.
        e.mem.set_u32(a.addr() + 0x74, 0x9001);
        e.mem.set_u32(b.addr() + 0x74, 0x9001);
        assert!(!compare(&mut e));
        e.mem.set_u32(b.addr() + 0x74, 0x9002);
        assert!(compare(&mut e));
        e.mem.set_u32(b.addr() + 0x74, 0x9001);
        e.mem.set_u32(b.addr() + 0x78, 3);
        assert!(compare(&mut e));
        e.mem.set_u32(b.addr() + 0x78, 0);
        // Type 3: topic then speaker.
        e.set(a, BGSNote::cType, 3);
        assert!(compare(&mut e));
        e.set(b, BGSNote::cType, 3);
        e.set(b, BGSNote::pNoteData, data);
        assert!(!compare(&mut e));
        e.set(b, BGSNote::pSpeaker, 4);
        assert!(compare(&mut e));
        e.set(b, BGSNote::pSpeaker, 0);
        e.mem.set_u32(b.addr() + 0x20, 1);
        assert!(compare(&mut e));
        e.mem.set_u32(b.addr() + 0x10, 0);
        assert!(compare(&mut e));
    }

    #[test]
    fn data_setters_act_on_their_type() {
        let mut e = Engine::new();
        e.put_vtable(0x0204_0000, &[0, 0, 0x0090_0008]);
        e.register(0x0090_0008, |_, _| Ret::default());
        let data = e.mem.alloc(4);
        e.mem.set_u32(data, 0x0204_0000);
        let n = note(&mut e, 0, 0);
        e.call(0x005e_9130, &args![n, 0x55u32]);
        assert_eq!(e.get(n, BGSNote::pNoteData), 0x55);
        e.set(n, BGSNote::cType, 1);
        e.call(0x005e_9130, &args![n, 0x66u32]);
        assert_eq!(e.get(n, BGSNote::pNoteData), 0x55);
        e.set(n, BGSNote::pNoteData, data);
        e.call_log = Some(vec![]);
        e.call(0x005e_9200, &args![n, 7u32]);
        e.call(0x005e_9190, &args![n, 8u32]);
        e.set(n, BGSNote::cType, 2);
        e.call(0x005e_9190, &args![n, 9u32]);
        e.call(0x005e_9200, &args![n, 10u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0090_0008),
            vec![vec![data, 7], vec![data, 9]]
        );
    }

    #[test]
    fn set_read_calls_the_flag_slots_on_change() {
        let mut e = Engine::new();
        e.put_vtable(0x0204_0000, &[0; 0x20]);
        e.mem.set_u32(0x0204_0048, 0x0090_0048);
        e.mem.set_u32(0x0204_004c, 0x0090_004c);
        quiet(&mut e, &[0x0090_0048, 0x0090_004c]);
        let n = note(&mut e, 0, 0);
        e.mem.set_u32(n.addr(), 0x0204_0000);
        e.call_log = Some(vec![]);
        e.call(0x005e_9300, &args![n, 0u32]);
        e.call(0x005e_9300, &args![n, 0x101u32]);
        assert_eq!(e.get(n, BGSNote::bHasBeenRead), 1);
        e.call(0x005e_9300, &args![n, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0090_0048),
            vec![vec![n.addr(), 0x8000_0000]]
        );
        assert_eq!(
            calls_to(&log, 0x0090_004c),
            vec![vec![n.addr(), 0x8000_0000]]
        );
    }

    #[test]
    fn activate_adds_the_note_and_rewards_karma_on_theft() {
        let run = |owner_kind: u32, flag: bool, alignment: u32, owner: u32| {
            let mut e = Engine::new();
            e.map(0x011d_e000, 0x1000);
            e.map(0x011c_d000, 0x1000);
            e.mem.set_u32(PLAYER, 0x0400_0000);
            quiet(
                &mut e,
                &[DYNAMIC_CAST, 0x0094_fd30, 0x0096_6a70, 0x0057_4400],
            );
            e.register_double(0x0056_7790, move |_, _| owner.into_ret());
            e.register(0x0057_85e0, |_, _| false.into_ret());
            e.register_double(0x0040_1170, move |_, _| owner_kind.into_ret());
            e.register_double(0x0047_d7c0, move |_, _| flag.into_ret());
            e.register_double(0x0047_e040, move |_, _| alignment.into_ret());
            e.register(0x0040_3e20, |_, a| (a[0] + 4).into_ret());
            e.mem.set_f32(0x011c_de24, -2.5);
            e.register(0x00ec_62c0, |_, a| {
                let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
                (value as i32).into_ret()
            });
            if owner != 0 {
                e.map(owner, 0x1000);
                e.put_vtable(0x0205_0000, &[0, 0, 0, 0x0090_000c]);
                e.mem.set_u32(owner + 0x100, 0x0205_0000);
                e.register(0x0090_000c, |_, _| 1.0f64.into_ret());
            }
            let n = e.new_object::<BGSNote>();
            e.call_log = Some(vec![]);
            let result = e
                .call(
                    0x005e_9360,
                    &args![n, 0x5000u32, 0x0400_0000u32, 0u32, 0u32, 0u32],
                )
                .bool();
            assert!(result);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, 0x0096_6a70),
                vec![vec![0x0400_0000, n.addr(), 1]]
            );
            assert_eq!(calls_to(&log, 0x0057_4400), vec![vec![0x5000]]);
            calls_to(&log, 0x0094_fd30)
        };
        let reward = vec![vec![0x0400_0000, (-2i32) as u32]];
        assert!(run(8, false, 0, 0).is_empty());
        assert_eq!(run(8, false, 0, 0x0600_0000), reward);
        assert!(run(8, true, 0, 0x0600_0000).is_empty());
        assert_eq!(run(3, false, 1, 0x0600_0000), reward);
        assert!(run(3, false, 2, 0x0600_0000).is_empty());
        assert!(run(3, false, 4, 0x0600_0000).is_empty());

        // Not the player: false, nothing added.
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u32(PLAYER, 0x0400_0000);
        quiet(&mut e, &[DYNAMIC_CAST]);
        let n = e.new_object::<BGSNote>();
        assert!(!e
            .call(
                0x005e_9360,
                &args![n, 0x5000u32, 0x0400_0004u32, 0u32, 0u32, 0u32]
            )
            .bool());
    }
}
