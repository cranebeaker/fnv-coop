//! `fallout/ai/dialoguepackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `DialoguePackage` fields on PC (the Xbox PDB's offsets less 0x10:
/// `TESPackage` is 0x80 bytes on PC): `handle` (`BSSoundHandle`) +0x80,
/// `ptopic` +0x8C, `flistentimer` +0x90, `pTalkingActor` +0x94,
/// `bWaitingforLipFile` +0x98, `pGenericSecondLoc` +0x9C, `bUseIdleManager`
/// +0xA0, `pConversation` +0xA4, `pCurrentItem` +0xA8, `pCurrentResponse`
/// +0xAC, `pActor` +0xB0, `pTargetActor` +0xB4, `pTalkingActivator` +0xB8,
/// `bActorHeadTrack` +0xBC, `bCreatedPack` +0xBD, `bContinueMoving` +0xBE,
/// `bTargetContinueMoving` +0xBF, `bTargetMoveTowardStarter` +0xC0,
/// `bDoNotControlTargetMovement` +0xC1, `bReachedAmbushLocation` +0xC2,
/// `iEmotion` +0xC4, `iEmotionValue` +0xC8, `bSoundFile` +0xCC.
const HANDLE: u32 = 0x80;
const TOPIC: u32 = 0x8c;
const LISTEN_TIMER: u32 = 0x90;
const TALKING_ACTOR: u32 = 0x94;
const WAITING_FOR_LIP_FILE: u32 = 0x98;
const GENERIC_SECOND_LOCATION: u32 = 0x9c;
const USE_IDLE_MANAGER: u32 = 0xa0;
const CONVERSATION: u32 = 0xa4;
const CURRENT_ITEM: u32 = 0xa8;
const CURRENT_RESPONSE: u32 = 0xac;
const ACTOR: u32 = 0xb0;
const TARGET_ACTOR: u32 = 0xb4;
const TALKING_ACTIVATOR: u32 = 0xb8;
const ACTOR_HEAD_TRACK: u32 = 0xbc;
const CREATED_PACK: u32 = 0xbd;
const CONTINUE_MOVING: u32 = 0xbe;
const TARGET_CONTINUE_MOVING: u32 = 0xbf;
const TARGET_MOVE_TOWARD_STARTER: u32 = 0xc0;
const DO_NOT_CONTROL_TARGET_MOVEMENT: u32 = 0xc1;
const REACHED_AMBUSH_LOCATION: u32 = 0xc2;
const EMOTION: u32 = 0xc4;
const EMOTION_VALUE: u32 = 0xc8;
const SOUND_FILE: u32 = 0xcc;

/// `DialoguePackage`'s vtable (RTTI `.?AVDialoguePackage@@`).
const DIALOGUE_PACKAGE_VTABLE: u32 = 0x0109_2f6c;
/// `TESPackage::TESPackage` (`006707c0`, `thiscall`).
const PACKAGE_CONSTRUCTOR: u32 = 0x0067_07c0;
/// The sound handle's constructor (`0041a250`, `thiscall`; engine map unit
/// `extradatalist.cpp`, a folded body).
const SOUND_HANDLE_CONSTRUCTOR: u32 = 0x0041_a250;
/// `Conversation::FirstItem` (`0083b9a0`, `thiscall`).
const CONVERSATION_FIRST_ITEM: u32 = 0x0083_b9a0;
/// The actor's vtable slot +0x200 the constructor calls with 0.
const ACTOR_SLOT_200: u32 = 0x200;
/// The player (`011dea3c`) and `PlayerCharacter::CheckForClosesConversation`
/// (`00967730`, `thiscall (player, actor, target, package)`).
const PLAYER: u32 = 0x011d_ea3c;
const CHECK_FOR_CLOSE_CONVERSATION: u32 = 0x0096_7730;
/// `TESPackage::SaveGame` (`006797c0`, one word),
/// `BGSSaveGameBuffer::SaveFormID_ov2` (`00865df0`, `(buffer, form, 0)`),
/// the buffer's data save (`00865e50`, `(buffer, data, size, 0)`),
/// `Conversation::SaveGame` (`0083c270`, one word),
/// `Conversation::GetDialogueItemIndex` (`0083ba50`, one word) and
/// `DialogueItem::GetDialogueResponseIndex` (`0083c900`, one word), both
/// answering a 16-bit index.
const PACKAGE_SAVE_GAME: u32 = 0x0067_97c0;
const SAVE_FORM_ID: u32 = 0x0086_5df0;
const SAVE_DATA: u32 = 0x0086_5e50;
const CONVERSATION_SAVE_GAME: u32 = 0x0083_c270;
const CONVERSATION_ITEM_INDEX: u32 = 0x0083_ba50;
const ITEM_RESPONSE_INDEX: u32 = 0x0083_c900;

fn field(this: Ptr, offset: u32) -> u32 {
    this.addr().wrapping_add(offset)
}

// Translated from 008d80e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialoguePackage::GetTargetOfConversation` (Xbox PDB): `pTargetActor`.
pub fn dialogue_package_get_target_of_conversation(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(field(this, TARGET_ACTOR)))
}

// Translated from 009ee040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialoguePackage::GetActorStartedConversation` (Xbox PDB): `pActor`.
pub fn dialogue_package_get_actor_started_conversation(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(field(this, ACTOR)))
}

// Translated from 009edd80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009edd80 - the exception-unwinding frame (FS:[0]) is not translated.
/// `DialoguePackage::DialoguePackage` (Xbox PDB): the `TESPackage`
/// constructor, the class's vtable, the sound handle's constructor, then
/// the conversation (with `Conversation::FirstItem` when there is one), no
/// item or response, listen timer 0, the actor and target (the actor's
/// vtable slot +0x200 with 0), no topic or talking actor, `bCreatedPack`
/// from its argument, the flags cleared but `bActorHeadTrack`, no generic
/// second location, the player's `CheckForClosesConversation` with actor,
/// target and the package, emotion 0 with value 50, the talking activator,
/// `bUseIdleManager` from its argument, and `bSoundFile` set. Answers
/// `this`. The exception-unwinding frame is not translated.
pub fn dialogue_package_dialogue_package(
    e: &mut Engine,
    this: Ptr,
    conversation: Ptr,
    actor: Ptr,
    target_actor: Ptr,
    talking_activator: Ptr,
    created_pack: u8,
    use_idle_manager: u8,
) -> Ptr {
    e.call(PACKAGE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), DIALOGUE_PACKAGE_VTABLE);
    e.call(SOUND_HANDLE_CONSTRUCTOR, &args![field(this, HANDLE)]);
    e.mem
        .set_u32(field(this, CONVERSATION), conversation.addr());
    if !conversation.is_null() {
        let conversation = e.mem.u32(field(this, CONVERSATION));
        e.call(CONVERSATION_FIRST_ITEM, &args![conversation]);
    }
    e.mem.set_u32(field(this, CURRENT_ITEM), 0);
    e.mem.set_u32(field(this, CURRENT_RESPONSE), 0);
    e.mem.set_u32(field(this, LISTEN_TIMER), x87_store32(0.0));
    e.mem.set_u32(field(this, ACTOR), actor.addr());
    e.mem
        .set_u32(field(this, TARGET_ACTOR), target_actor.addr());
    let started_by = e.mem.u32(field(this, ACTOR));
    e.vcall(started_by, ACTOR_SLOT_200, &args![0u32]);
    e.mem.set_u32(field(this, TOPIC), 0);
    e.mem.set_u32(field(this, TALKING_ACTOR), 0);
    e.mem.set_u8(field(this, CREATED_PACK), created_pack);
    e.mem.set_u8(field(this, WAITING_FOR_LIP_FILE), 0);
    e.mem.set_u8(field(this, CONTINUE_MOVING), 0);
    e.mem.set_u8(field(this, TARGET_CONTINUE_MOVING), 0);
    e.mem.set_u8(field(this, TARGET_MOVE_TOWARD_STARTER), 0);
    e.mem.set_u8(field(this, ACTOR_HEAD_TRACK), 1);
    e.mem.set_u8(field(this, DO_NOT_CONTROL_TARGET_MOVEMENT), 0);
    e.mem.set_u32(field(this, GENERIC_SECOND_LOCATION), 0);
    let player = e.mem.u32(PLAYER);
    e.call(
        CHECK_FOR_CLOSE_CONVERSATION,
        &args![player, actor, target_actor, this],
    );
    e.mem.set_u32(field(this, EMOTION), 0);
    e.mem.set_u32(field(this, EMOTION_VALUE), 0x32);
    e.mem
        .set_u32(field(this, TALKING_ACTIVATOR), talking_activator.addr());
    e.mem
        .set_u8(field(this, USE_IDLE_MANAGER), use_idle_manager);
    e.mem.set_u8(field(this, REACHED_AMBUSH_LOCATION), 0);
    e.mem.set_u8(field(this, SOUND_FILE), 1);
    this
}

// Translated from 009f01d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialoguePackage::SaveGame` (Xbox PDB): `TESPackage::SaveGame`, then the
/// talking actor, actor, target, talking activator, generic second location
/// and topic as form IDs, the listen timer, the flags, emotion and its
/// value, `bUseIdleManager`, `bSoundFile` and `bReachedAmbushLocation` as
/// data, a byte saying whether there is a conversation, and with one the
/// conversation itself and the 16-bit indexes of the current item and
/// response (-1 when unset).
pub fn dialogue_package_save_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(PACKAGE_SAVE_GAME, &args![this, buffer]);
    for offset in [
        TALKING_ACTOR,
        ACTOR,
        TARGET_ACTOR,
        TALKING_ACTIVATOR,
        GENERIC_SECOND_LOCATION,
        TOPIC,
    ] {
        let form = e.mem.u32(field(this, offset));
        e.call(SAVE_FORM_ID, &args![buffer, form, 0u32]);
    }
    for (offset, size) in [
        (LISTEN_TIMER, 4u32),
        (WAITING_FOR_LIP_FILE, 1),
        (ACTOR_HEAD_TRACK, 1),
        (CREATED_PACK, 1),
        (CONTINUE_MOVING, 1),
        (TARGET_CONTINUE_MOVING, 1),
        (TARGET_MOVE_TOWARD_STARTER, 1),
        (DO_NOT_CONTROL_TARGET_MOVEMENT, 1),
        (EMOTION, 4),
        (EMOTION_VALUE, 4),
        (USE_IDLE_MANAGER, 1),
        (SOUND_FILE, 1),
        (REACHED_AMBUSH_LOCATION, 1),
    ] {
        e.call(SAVE_DATA, &args![buffer, field(this, offset), size, 0u32]);
    }
    e.with_stack(0xc, |e, frame| {
        let has_conversation = frame.addr();
        let item_index = has_conversation + 4;
        let response_index = has_conversation + 8;
        let conversation = e.mem.u32(field(this, CONVERSATION));
        e.mem.set_u8(has_conversation, u8::from(conversation != 0));
        e.call(SAVE_DATA, &args![buffer, has_conversation, 1u32, 0u32]);
        if e.mem.u8(has_conversation) == 0 {
            return;
        }
        let conversation = e.mem.u32(field(this, CONVERSATION));
        e.call(CONVERSATION_SAVE_GAME, &args![conversation, buffer]);
        let item = e.mem.u32(field(this, CURRENT_ITEM));
        let index = if item != 0 {
            let conversation = e.mem.u32(field(this, CONVERSATION));
            e.call(CONVERSATION_ITEM_INDEX, &args![conversation, item])
                .u16()
        } else {
            0xffff
        };
        e.mem.set_u16(item_index, index);
        e.call(SAVE_DATA, &args![buffer, item_index, 2u32, 0u32]);
        let item = e.mem.u32(field(this, CURRENT_ITEM));
        let response = e.mem.u32(field(this, CURRENT_RESPONSE));
        let index = if item != 0 && response != 0 {
            e.call(ITEM_RESPONSE_INDEX, &args![item, response]).u16()
        } else {
            0xffff
        };
        e.mem.set_u16(response_index, index);
        e.call(SAVE_DATA, &args![buffer, response_index, 2u32, 0u32]);
    });
}

/// The save/load manager (`TESSaveLoadGame*`, a pointer global) and its
/// calls the old-format save and load use: the buffer position
/// (`00825c00`), whether save-game blocks are used (`00862110`,
/// `TESSaveLoadGame::UseSaveGameBlocks`), the save version (`008df040`),
/// the raw write and read (`008579b0` / `008579e0`, thiscall(data,
/// size)) and the form being saved or loaded (`004fd3e0` / `004fd3c0`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const BUFFER_POSITION: u32 = 0x0082_5c00;
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
const SAVE_VERSION: u32 = 0x008d_f040;
const RAW_WRITE: u32 = 0x0085_79b0;
const RAW_READ: u32 = 0x0085_79e0;
const SAVED_FORM: u32 = 0x004f_d3e0;
const LOADED_FORM: u32 = 0x004f_d3c0;
/// The `bool` setting that turns on save-size logging (`011de4e8`, through
/// `00408d60`).
const SAVE_LOGGING_SETTING: u32 = 0x011d_e4e8;
const BOOL_SETTING_VALUE: u32 = 0x0040_8d60;
/// `TESForm::SaveGameDataOLD`, `LoadGameDataOLD`, `SaveNumericID` and
/// `LoadNumericID` (`00484ce0`, `00484d00`, `00484d20`, `00484d40`;
/// thiscall(data, size)).
const SAVE_DATA_OLD: u32 = 0x0048_4ce0;
const LOAD_DATA_OLD: u32 = 0x0048_4d00;
const SAVE_NUMERIC_ID: u32 = 0x0048_4d20;
const LOAD_NUMERIC_ID: u32 = 0x0048_4d40;
/// The engine's message and error prints (`005b5e40`, `0040fbe0`; cdecl
/// with a format).
const PRINT: u32 = 0x005b_5e40;
const ERROR: u32 = 0x0040_fbe0;
/// `"D:\_Fallout3\...\DialoguePackage.cpp"`.
const SOURCE_FILE: u32 = 0x0109_3108;
/// `004839c0` (cdecl(form id)): the form for a saved or loaded id.
const FORM_FOR_ID: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the RTTI type
/// descriptors `.?AVTESForm@@`, `.?AVActor@@` and `.?AVTESTopic@@`; the
/// descriptor `01199c3c` is `.?AVDialoguePackage@@`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_ACTOR: u32 = 0x0118_46d4;
const TYPE_TES_TOPIC: u32 = 0x0118_4720;
const TYPE_DIALOGUE_PACKAGE: u32 = 0x0119_9c3c;
/// `"KOLB"` read as a little-endian word: the save-game block header.
const BLOCK_HEADER: u32 = 0x424c_4f4b;
/// The game's allocator (`00401000`, cdecl(size)).
const ALLOCATE: u32 = 0x0040_1000;
/// `Conversation::Conversation` (`0083b830`), `GetDialogueItemByIndex`
/// (`0083bac0`, thiscall(index)), `DialogueItem::GetDialogueResponseByIndex`
/// (`0083c970`, thiscall(index)).
const CONVERSATION_CONSTRUCTOR: u32 = 0x0083_b830;
const ITEM_BY_INDEX: u32 = 0x0083_bac0;
const RESPONSE_BY_INDEX: u32 = 0x0083_c970;
/// `BGSLoadGameBuffer::LoadFormID_ov2` (`008648e0`, thiscall(address)),
/// `LoadFormID` (`008648a0`, thiscall) and the byte load (`00864980`,
/// thiscall(destination, size)).
const LOAD_FORM_ID_INTO: u32 = 0x0086_48e0;
const LOAD_FORM_ID: u32 = 0x0086_48a0;
const LOAD_DATA: u32 = 0x0086_4980;

/// `__RTDynamicCast(004839c0(id), 0, TESForm, to, 0)`.
fn cast_loaded(e: &mut Engine, id: u32, to: u32) -> u32 {
    let form = e.call(FORM_FOR_ID, &args![id]).u32();
    e.call(RT_DYNAMIC_CAST, &args![form, 0u32, TYPE_TES_FORM, to, 0u32])
        .u32()
}

/// The save/load manager.
fn save_load_game(e: &Engine) -> u32 {
    e.global::<u32>(SAVE_LOAD_GAME)
}

fn logging(e: &mut Engine) -> bool {
    let value = e
        .call(BOOL_SETTING_VALUE, &args![SAVE_LOGGING_SETTING])
        .u32();
    e.mem.u8(value) != 0
}

// Translated from 009ee060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The package's `00673290(reference)` (thiscall(`reference`), `RET 4`; no
/// Xbox PDB name), then a cast of `reference` to `DialoguePackage`
/// (`__RTDynamicCast`) whose result is not used.
pub fn fn_009ee060(e: &mut Engine, this: Ptr, reference: u32) {
    e.call(0x0067_3290, &args![this, reference]);
    e.call(
        RT_DYNAMIC_CAST,
        &args![reference, 0u32, TYPE_TES_FORM, TYPE_DIALOGUE_PACKAGE, 0u32],
    );
}

// Translated from 009ef510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bActorHeadTrack` (+0xBC; thiscall; no Xbox PDB name).
pub fn fn_009ef510(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(field(this, ACTOR_HEAD_TRACK))
}

// Translated from 009ef530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00934250` on the talking actor (+0x94) when there is one (thiscall; no
/// Xbox PDB name).
pub fn fn_009ef530(e: &mut Engine, this: Ptr) {
    let talking = e.mem.u32(field(this, TALKING_ACTOR));
    if talking != 0 {
        e.call(0x0093_4250, &args![talking]);
    }
}

/// The form-and-name line of the save-size and buffer logs: the form
/// (`*form` the id, `004839c0`), its name (virtual +0x130), the byte at +9
/// and the word at +5 of the form record.
fn logged_form(e: &mut Engine, record: u32) -> (u32, u32, u32, u32) {
    let id = e.mem.u32(record);
    let form = e.call(FORM_FOR_ID, &args![id]).u32();
    let name = e.vcall(form, 0x130, &args![]).u32();
    let id = e.mem.u32(record);
    let version = u32::from(e.mem.u8(record.wrapping_add(9)));
    let flags = e.mem.u32(record.wrapping_add(5));
    (id, name, version, flags)
}

// Translated from 009ef560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialoguePackage`'s old-format save size (thiscall, a 16-bit size; no
/// Xbox PDB name): the `TESPackage` part (`00678fd0`), 4 + 2 more with
/// save-game blocks, 4, 1 from version 0x6A, 0x10 + 2, and with a
/// conversation its size (`0083bb30`) + 4. With save-size logging the
/// difference is printed (`0040fbe0`, with the form being saved when there
/// is one). Every step is a 16-bit addition.
pub fn fn_009ef560(e: &mut Engine, this: Ptr) -> u16 {
    let mut size = 0u16;
    size = size.wrapping_add(e.call(0x0067_8fd0, &args![this]).u16());
    let start = size;
    let manager = save_load_game(e);
    if e.call(USE_SAVE_GAME_BLOCKS, &args![manager]).bool() {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    size = size.wrapping_add(4);
    let manager = save_load_game(e);
    if e.call(SAVE_VERSION, &args![manager]).u8() >= 0x6a {
        size = size.wrapping_add(1);
    }
    size = size.wrapping_add(0x10);
    size = size.wrapping_add(2);
    let conversation = e.mem.u32(field(this, CONVERSATION));
    if conversation != 0 {
        size = size.wrapping_add(e.call(0x0083_bb30, &args![conversation]).u16());
        size = size.wrapping_add(2);
        size = size.wrapping_add(2);
    }
    if logging(e) {
        let difference = (i32::from(size) - i32::from(start)) as u32;
        let manager = save_load_game(e);
        let record = e.call(SAVED_FORM, &args![manager]).u32();
        if record != 0 {
            let (id, name, _, flags) = logged_form(e, record);
            e.call(
                ERROR,
                &args![
                    0x0101_2cb0u32,
                    difference,
                    id,
                    name,
                    flags,
                    0x1f4u32,
                    SOURCE_FILE
                ],
            );
        } else {
            e.call(
                ERROR,
                &args![0x0101_2c78u32, difference, 0x1f4u32, SOURCE_FILE],
            );
        }
    }
    size
}

// Translated from 009ef6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialoguePackage`'s old-format save (thiscall; no Xbox PDB name): the
/// `TESPackage` part (`00679120`); with save-game blocks the `"KOLB"`
/// header and a 16-bit length placeholder (its address the buffer position
/// before it); the listen timer (4), `bWaitingforLipFile` from version
/// 0x6A; the talking actor, actor, target and topic as numeric ids (0 for
/// none, `0084e3a0`); the conversation's size (`0083bb30`, 0 without
/// one) and, when not 0, the conversation (`0083bc80`) with the 16-bit
/// indexes of the current item and response (-1 when unset). With
/// save-size logging the size is printed; with blocks the length is
/// written (a warning when it passes 0xFFFF).
pub fn fn_009ef6e0(e: &mut Engine, this: Ptr) {
    e.call(0x0067_9120, &args![this]);
    e.with_stack(0x48, |e, frame| {
        let ebp = frame.addr() + 0x48;
        let length = ebp - 0x18;
        e.mem.set_u16(length, 0);
        let mut block = 0u32;
        let manager = save_load_game(e);
        let mut start = e.call(BUFFER_POSITION, &args![manager]).u32();
        if logging(e) {
            let manager = save_load_game(e);
            start = e.call(BUFFER_POSITION, &args![manager]).u32();
        }
        let manager = save_load_game(e);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![manager]).bool() {
            let header = ebp - 0x24;
            e.mem.set_u32(header, BLOCK_HEADER);
            e.call(RAW_WRITE, &args![manager, header, 4u32]);
            let manager = save_load_game(e);
            block = e.call(BUFFER_POSITION, &args![manager]).u32();
            let manager = save_load_game(e);
            e.call(RAW_WRITE, &args![manager, length, 2u32]);
        }
        e.call(SAVE_DATA_OLD, &args![this, field(this, LISTEN_TIMER), 4u32]);
        let manager = save_load_game(e);
        if e.call(SAVE_VERSION, &args![manager]).u8() >= 0x6a {
            e.call(
                SAVE_DATA_OLD,
                &args![this, field(this, WAITING_FOR_LIP_FILE), 1u32],
            );
        }
        for (offset, slot) in [
            (TALKING_ACTOR, ebp - 4),
            (ACTOR, ebp - 0x14),
            (TARGET_ACTOR, ebp - 0x20),
            (TOPIC, ebp - 0xc),
        ] {
            e.mem.set_u32(slot, 0);
            let form = e.mem.u32(field(this, offset));
            if form != 0 {
                let id = e.call(0x0084_e3a0, &args![form]).u32();
                e.mem.set_u32(slot, id);
            }
            e.call(SAVE_NUMERIC_ID, &args![this, slot, 4u32]);
        }
        let count = ebp - 0x10;
        e.mem.set_u16(count, 0);
        let conversation = e.mem.u32(field(this, CONVERSATION));
        if conversation != 0 {
            let size = e.call(0x0083_bb30, &args![conversation]).u16();
            e.mem.set_u16(count, size);
        }
        let manager = save_load_game(e);
        e.call(RAW_WRITE, &args![manager, count, 2u32]);
        if e.mem.u16(count) != 0 {
            let conversation = e.mem.u32(field(this, CONVERSATION));
            e.call(0x0083_bc80, &args![conversation]);
            let item_index = ebp - 0x2c;
            e.mem.set_u16(item_index, 0xffff);
            let item = e.mem.u32(field(this, CURRENT_ITEM));
            if item != 0 {
                let conversation = e.mem.u32(field(this, CONVERSATION));
                let index = e
                    .call(CONVERSATION_ITEM_INDEX, &args![conversation, item])
                    .u16();
                e.mem.set_u16(item_index, index);
            }
            e.call(SAVE_DATA_OLD, &args![this, item_index, 2u32]);
            let response_index = ebp - 0x28;
            e.mem.set_u16(response_index, 0xffff);
            let item = e.mem.u32(field(this, CURRENT_ITEM));
            let response = e.mem.u32(field(this, CURRENT_RESPONSE));
            if item != 0 && response != 0 {
                let index = e.call(ITEM_RESPONSE_INDEX, &args![item, response]).u16();
                e.mem.set_u16(response_index, index);
            }
            e.call(SAVE_DATA_OLD, &args![this, response_index, 2u32]);
        }
        if logging(e) {
            let manager = save_load_game(e);
            let now = e.call(BUFFER_POSITION, &args![manager]).u32();
            let manager = save_load_game(e);
            let record = e.call(SAVED_FORM, &args![manager]).u32();
            let difference = now.wrapping_sub(start);
            if record != 0 {
                let (id, name, _, flags) = logged_form(e, record);
                e.call(
                    ERROR,
                    &args![
                        0x0101_53a0u32,
                        difference,
                        id,
                        name,
                        flags,
                        0x231u32,
                        SOURCE_FILE
                    ],
                );
            } else {
                e.call(
                    ERROR,
                    &args![0x0101_536cu32, difference, 0x231u32, SOURCE_FILE],
                );
            }
        }
        let manager = save_load_game(e);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![manager]).bool() {
            let manager = save_load_game(e);
            let now = e.call(BUFFER_POSITION, &args![manager]).u32();
            if now > block.wrapping_add(0xffff) {
                e.call(PRINT, &args![0x0101_5318u32, SOURCE_FILE, 0x231u32]);
            }
            e.mem.set_u16(block, now.wrapping_sub(block) as u16);
        }
    });
}

/// The buffer check at the end of the old-format load: an overrun or
/// underrun against `start + length` is printed (with the form being
/// loaded when there is one).
fn check_load_block(e: &mut Engine, start: u32, length: u16) {
    let manager = save_load_game(e);
    let now = e.call(BUFFER_POSITION, &args![manager]).u32();
    let manager = save_load_game(e);
    let record = e.call(LOADED_FORM, &args![manager]).u32();
    let end = u32::from(length).wrapping_add(start);
    if record != 0 {
        let id = e.mem.u32(record);
        let form = e.call(FORM_FOR_ID, &args![id]).u32();
        let (format, amount) = if now > end {
            (0x0101_5588u32, now.wrapping_sub(end))
        } else if now < end {
            (0x0101_5500, end.wrapping_sub(now))
        } else {
            return;
        };
        let flags = e.mem.u32(record.wrapping_add(5));
        let version = u32::from(e.mem.u8(record.wrapping_add(9)));
        let name = e.vcall(form, 0x130, &args![]).u32();
        let id = e.mem.u32(record);
        e.call(
            PRINT,
            &args![
                format,
                amount,
                SOURCE_FILE,
                0x288u32,
                id,
                name,
                version,
                flags
            ],
        );
    } else {
        let (format, amount) = if now > end {
            (0x0101_54a0u32, now.wrapping_sub(end))
        } else if now < end {
            (0x0101_5440, end.wrapping_sub(now))
        } else {
            return;
        };
        let manager = save_load_game(e);
        let version = u32::from(e.call(SAVE_VERSION, &args![manager]).u8());
        e.call(
            PRINT,
            &args![format, amount, SOURCE_FILE, 0x288u32, version],
        );
    }
}

// Translated from 009efa50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009efa50 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `DialoguePackage::LoadGame` (Xbox PDB; the old format; thiscall): the
/// `TESPackage` part (`00679340`); with save-game blocks the header (a
/// wrong one printed with the form being loaded, or the version) and the
/// block length; the listen timer, `bWaitingforLipFile` from version 0x6A,
/// the talking actor, actor and target ids as they are, the topic cast
/// from its id (`TESTopic`); from version 0x20 the conversation (when its
/// saved size is not 0: 0xC bytes, `Conversation::Conversation`,
/// `0083bec0`) with the current item and response by index (-1: none);
/// before version 0x20 the conversation word and the two indexes as they
/// are (cleared without a conversation). With blocks the buffer position
/// is checked against the block's end.
pub fn dialogue_package_load_game(e: &mut Engine, this: Ptr) {
    e.call(0x0067_9340, &args![this]);
    e.with_stack(0x68, |e, frame| {
        let ebp = frame.addr() + 0x68;
        let length = ebp - 0x1c;
        e.mem.set_u16(length, 0);
        let mut start = 0u32;
        let manager = save_load_game(e);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![manager]).bool() {
            let header = ebp - 0x28;
            let manager = save_load_game(e);
            e.call(RAW_READ, &args![manager, header, 4u32]);
            if e.mem.u32(header) != BLOCK_HEADER {
                let manager = save_load_game(e);
                let record = e.call(LOADED_FORM, &args![manager]).u32();
                if record != 0 {
                    let (_, name, version, flags) = logged_form(e, record);
                    let id = e.mem.u32(record);
                    e.call(
                        PRINT,
                        &args![
                            0x0101_5718u32,
                            SOURCE_FILE,
                            0x239u32,
                            id,
                            name,
                            version,
                            flags
                        ],
                    );
                } else {
                    let manager = save_load_game(e);
                    let version = u32::from(e.call(SAVE_VERSION, &args![manager]).u8());
                    e.call(
                        PRINT,
                        &args![0x0101_56a8u32, SOURCE_FILE, 0x239u32, version],
                    );
                }
            }
            let manager = save_load_game(e);
            start = e.call(BUFFER_POSITION, &args![manager]).u32();
            let manager = save_load_game(e);
            e.call(RAW_READ, &args![manager, length, 2u32]);
        }
        e.call(LOAD_DATA_OLD, &args![this, field(this, LISTEN_TIMER), 4u32]);
        let manager = save_load_game(e);
        if e.call(SAVE_VERSION, &args![manager]).u8() >= 0x6a {
            e.call(
                LOAD_DATA_OLD,
                &args![this, field(this, WAITING_FOR_LIP_FILE), 1u32],
            );
        }
        for (slot, offset) in [
            (ebp - 0x10, TALKING_ACTOR),
            (ebp - 0x18, ACTOR),
            (ebp - 0x24, TARGET_ACTOR),
        ] {
            e.call(LOAD_NUMERIC_ID, &args![this, slot, 4u32]);
            let id = e.mem.u32(slot);
            e.mem.set_u32(field(this, offset), id);
        }
        let topic = ebp - 0x14;
        e.call(LOAD_NUMERIC_ID, &args![this, topic, 4u32]);
        let id = e.mem.u32(topic);
        let topic = cast_loaded(e, id, TYPE_TES_TOPIC);
        e.mem.set_u32(field(this, TOPIC), topic);
        let manager = save_load_game(e);
        if e.call(SAVE_VERSION, &args![manager]).u8() >= 0x20 {
            let size = ebp - 0x34;
            e.call(LOAD_DATA_OLD, &args![this, size, 2u32]);
            if e.mem.u16(size) != 0 {
                let block = e.call(ALLOCATE, &args![0xcu32]).u32();
                let conversation = if block != 0 {
                    e.call(CONVERSATION_CONSTRUCTOR, &args![block]).u32()
                } else {
                    0
                };
                e.mem.set_u32(field(this, CONVERSATION), conversation);
                let conversation = e.mem.u32(field(this, CONVERSATION));
                e.call(0x0083_bec0, &args![conversation]);
                load_indexes(e, this, ebp - 0x3c, ebp - 0x38);
            }
        }
        let manager = save_load_game(e);
        if e.call(SAVE_VERSION, &args![manager]).u8() < 0x20 {
            let word = ebp - 0x40;
            e.call(LOAD_DATA_OLD, &args![this, word, 4u32]);
            let conversation = e.mem.u32(word);
            e.mem.set_u32(field(this, CONVERSATION), conversation);
            if conversation != 0 {
                for (slot, offset) in [(ebp - 0x48, CURRENT_ITEM), (ebp - 0x44, CURRENT_RESPONSE)] {
                    e.call(LOAD_DATA_OLD, &args![this, slot, 2u32]);
                    let index = e.mem.u16(slot) as i16;
                    e.mem.set_u32(field(this, offset), i32::from(index) as u32);
                }
            } else {
                e.mem.set_u32(field(this, CURRENT_ITEM), 0);
                e.mem.set_u32(field(this, CURRENT_RESPONSE), 0);
            }
        }
        let manager = save_load_game(e);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![manager]).bool() {
            let length = e.mem.u16(length);
            check_load_block(e, start, length);
        }
    });
}

/// `LoadGame`'s item and response indexes (`TESForm::LoadGameDataOLD`
/// into the frame slots): the item by index (`0083bac0`, none for -1);
/// the response index stored as read (sign-extended), then the response by
/// index (`0083c970`) with an item and an index other than -1, else none.
fn load_indexes(e: &mut Engine, this: Ptr, item_slot: u32, response_slot: u32) {
    e.call(LOAD_DATA_OLD, &args![this, item_slot, 2u32]);
    let item_index = e.mem.u16(item_slot);
    if item_index as i16 != -1 {
        let conversation = e.mem.u32(field(this, CONVERSATION));
        let item = e
            .call(ITEM_BY_INDEX, &args![conversation, u32::from(item_index)])
            .u32();
        e.mem.set_u32(field(this, CURRENT_ITEM), item);
    } else {
        e.mem.set_u32(field(this, CURRENT_ITEM), 0);
    }
    e.call(LOAD_DATA_OLD, &args![this, response_slot, 2u32]);
    let response_index = e.mem.u16(response_slot);
    e.mem.set_u32(
        field(this, CURRENT_RESPONSE),
        i32::from(response_index as i16) as u32,
    );
    let item = e.mem.u32(field(this, CURRENT_ITEM));
    if item != 0 && response_index as i16 != -1 {
        let response = e
            .call(RESPONSE_BY_INDEX, &args![item, u32::from(response_index)])
            .u32();
        e.mem.set_u32(field(this, CURRENT_RESPONSE), response);
    } else {
        e.mem.set_u32(field(this, CURRENT_RESPONSE), 0);
    }
}

// Translated from 009eff60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009eff60 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `DialoguePackage::InitLoadGame` (Xbox PDB): the `TESPackage` part
/// (`006796c0`); the talking actor, actor and target ids resolved and cast
/// to `Actor` (`004839c0`, `__RTDynamicCast`); from version 0x20 the
/// conversation's `0083c220`; before it a conversation (the saved word not
/// 0) is rebuilt (0xC bytes, `0083b850(actor, target, topic)`) with the
/// saved item and response indexes resolved (-1: none) and made current
/// (`0083c8a0` on the conversation and on the item).
pub fn dialogue_package_init_load_game(e: &mut Engine, this: Ptr) {
    e.call(0x0067_96c0, &args![this]);
    for offset in [TALKING_ACTOR, ACTOR, TARGET_ACTOR] {
        let id = e.mem.u32(field(this, offset));
        let actor = cast_loaded(e, id, TYPE_ACTOR);
        e.mem.set_u32(field(this, offset), actor);
    }
    let manager = save_load_game(e);
    if e.call(SAVE_VERSION, &args![manager]).u8() >= 0x20 {
        let conversation = e.mem.u32(field(this, CONVERSATION));
        if conversation != 0 {
            e.call(0x0083_c220, &args![conversation]);
        }
    }
    let manager = save_load_game(e);
    if e.call(SAVE_VERSION, &args![manager]).u8() >= 0x20 {
        return;
    }
    let saved = e.mem.u32(field(this, CONVERSATION));
    let item_index = e.mem.u16(field(this, CURRENT_ITEM));
    let response_index = e.mem.u16(field(this, CURRENT_RESPONSE));
    if saved == 0 {
        return;
    }
    let block = e.call(ALLOCATE, &args![0xcu32]).u32();
    let conversation = if block != 0 {
        let actor = e.mem.u32(field(this, ACTOR));
        let target = e.mem.u32(field(this, TARGET_ACTOR));
        let topic = e.mem.u32(field(this, TOPIC));
        e.call(0x0083_b850, &args![block, actor, target, topic])
            .u32()
    } else {
        0
    };
    e.mem.set_u32(field(this, CONVERSATION), conversation);
    if item_index as i16 != -1 {
        let conversation = e.mem.u32(field(this, CONVERSATION));
        let item = e
            .call(ITEM_BY_INDEX, &args![conversation, u32::from(item_index)])
            .u32();
        e.mem.set_u32(field(this, CURRENT_ITEM), item);
    } else {
        e.mem.set_u32(field(this, CURRENT_ITEM), 0);
    }
    let item = e.mem.u32(field(this, CURRENT_ITEM));
    if item != 0 && response_index as i16 != -1 {
        let response = e
            .call(RESPONSE_BY_INDEX, &args![item, u32::from(response_index)])
            .u32();
        e.mem.set_u32(field(this, CURRENT_RESPONSE), response);
    } else {
        e.mem.set_u32(field(this, CURRENT_RESPONSE), 0);
    }
    let conversation = e.mem.u32(field(this, CONVERSATION));
    let item = e.mem.u32(field(this, CURRENT_ITEM));
    e.call(0x0083_c8a0, &args![conversation, item]);
    let item = e.mem.u32(field(this, CURRENT_ITEM));
    let response = e.mem.u32(field(this, CURRENT_RESPONSE));
    e.call(0x0083_c8a0, &args![item, response]);
}

// Translated from 009f0460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f0460 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `DialoguePackage::LoadGame_ov2` (Xbox PDB; thiscall(`buffer`), `RET 4`):
/// what [`dialogue_package_save_game`] writes, in its order: the
/// `TESPackage` part (`006798c0`), the talking actor, actor, target,
/// talking activator and generic second location ids (`008648e0`), the
/// topic (`008648a0`, cast to `TESTopic`), the listen timer, the flags,
/// emotion and value, `bUseIdleManager`, `bSoundFile`,
/// `bReachedAmbushLocation` from version 3 (the buffer's virtual +0), and
/// with the conversation byte a conversation (0xC bytes, `0083b830`,
/// `0083c330(buffer)`) with the current item and response by index (left
/// as they are for -1).
pub fn dialogue_package_load_game_ov2(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(0x0067_98c0, &args![this, buffer]);
    for offset in [
        TALKING_ACTOR,
        ACTOR,
        TARGET_ACTOR,
        TALKING_ACTIVATOR,
        GENERIC_SECOND_LOCATION,
    ] {
        e.call(LOAD_FORM_ID_INTO, &args![buffer, field(this, offset)]);
    }
    let id = e.call(LOAD_FORM_ID, &args![buffer]).u32();
    let topic = cast_loaded(e, id, TYPE_TES_TOPIC);
    e.mem.set_u32(field(this, TOPIC), topic);
    for (offset, size) in [
        (LISTEN_TIMER, 4u32),
        (WAITING_FOR_LIP_FILE, 1),
        (ACTOR_HEAD_TRACK, 1),
        (CREATED_PACK, 1),
        (CONTINUE_MOVING, 1),
        (TARGET_CONTINUE_MOVING, 1),
        (TARGET_MOVE_TOWARD_STARTER, 1),
        (DO_NOT_CONTROL_TARGET_MOVEMENT, 1),
        (EMOTION, 4),
        (EMOTION_VALUE, 4),
        (USE_IDLE_MANAGER, 1),
        (SOUND_FILE, 1),
    ] {
        e.call(LOAD_DATA, &args![buffer, field(this, offset), size]);
    }
    if i32::from(e.vcall(buffer.addr(), 0, &args![]).u8()) >= 3 {
        e.call(
            LOAD_DATA,
            &args![buffer, field(this, REACHED_AMBUSH_LOCATION), 1u32],
        );
    }
    e.with_stack(0xc, |e, frame| {
        let flag = frame.addr();
        e.mem.set_u8(flag, 0);
        e.call(LOAD_DATA, &args![buffer, flag, 1u32]);
        if e.mem.u8(flag) == 0 {
            return;
        }
        let block = e.call(ALLOCATE, &args![0xcu32]).u32();
        let conversation = if block != 0 {
            e.call(CONVERSATION_CONSTRUCTOR, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(field(this, CONVERSATION), conversation);
        let conversation = e.mem.u32(field(this, CONVERSATION));
        e.call(0x0083_c330, &args![conversation, buffer]);
        let item_slot = frame.addr() + 4;
        e.mem.set_u16(item_slot, 0xffff);
        e.call(LOAD_DATA, &args![buffer, item_slot, 2u32]);
        let index = e.mem.u16(item_slot);
        if index as i16 != -1 {
            let conversation = e.mem.u32(field(this, CONVERSATION));
            let item = e
                .call(ITEM_BY_INDEX, &args![conversation, u32::from(index)])
                .u32();
            e.mem.set_u32(field(this, CURRENT_ITEM), item);
        }
        let response_slot = frame.addr() + 8;
        e.mem.set_u16(response_slot, 0xffff);
        e.call(LOAD_DATA, &args![buffer, response_slot, 2u32]);
        let index = e.mem.u16(response_slot);
        let item = e.mem.u32(field(this, CURRENT_ITEM));
        if item != 0 && index as i16 != -1 {
            let response = e
                .call(RESPONSE_BY_INDEX, &args![item, u32::from(index)])
                .u32();
            e.mem.set_u32(field(this, CURRENT_RESPONSE), response);
        }
    });
}

/// `Conversation::FirstItem`/`NextItem` (`0083b9a0`/`0083b9f0`),
/// `DialogueItem::FirstResponse`/`NextResponse`/`GetCurrentResponse`
/// (`0083c7b0`/`0083c7e0`/`0083c820`; `GetCurrentResponse` also answers a
/// conversation's current item) and `DialogueItem::RunResult` (`0083c850`,
/// thiscall(byte)).
const CONVERSATION_NEXT_ITEM: u32 = 0x0083_b9f0;
const ITEM_FIRST_RESPONSE: u32 = 0x0083_c7b0;
const ITEM_NEXT_RESPONSE: u32 = 0x0083_c7e0;
const GET_CURRENT: u32 = 0x0083_c820;
const ITEM_RUN_RESULT: u32 = 0x0083_c850;
/// `MiddleHighProcess::GetSavedAcquireObject` per the engine map
/// (`008d8520`): the word at +0x68 of an actor (its process).
const GET_PROCESS: u32 = 0x008d_8520;
/// `MobileObject::StopCurrentDialogue` (`00934250`, engine map).
const STOP_CURRENT_DIALOGUE: u32 = 0x0093_4250;
/// The response's speaker (`009611e0`, thiscall on the dialogue item).
const ITEM_SPEAKER: u32 = 0x0096_11e0;
/// `TESPackage::GetDialogueDoNotControlTarget` (`006727b0`, engine map).
const DO_NOT_CONTROL_TARGET: u32 = 0x0067_27b0;
/// `BSAudioManager::QInstance` (`00ad9060`) and its
/// `SetCompletionCallback` (`00adbfd0`, thiscall(handle id, callback,
/// form id)); the two callbacks the package uses (`00935cc0`, `00936ac0`).
const AUDIO_MANAGER: u32 = 0x00ad_9060;
const SET_COMPLETION_CALLBACK: u32 = 0x00ad_bfd0;
const LIP_FILE_CALLBACK: u32 = 0x0093_5cc0;
const DIALOGUE_DONE_CALLBACK: u32 = 0x0093_6ac0;
/// `BSStringT<char>` constructor (`004037b0`), constructor from a C string
/// (`004047f0`), `Set` (`00438470`) and destructor (`004037d0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_FROM: u32 = 0x0040_47f0;
const STRING_SET: u32 = 0x0043_8470;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `00460140` (thiscall): the response's text (a string object; its
/// pointer through `00559450`).
const RESPONSE_TEXT: u32 = 0x0046_0140;
const WORD_AT: u32 = 0x0055_9450;
/// `BGSSaveFormBuffer::GetForm` per the engine map (`007af430`): the
/// response's sound form; on the talking activator its reference.
const FORM_OF: u32 = 0x007a_f430;
/// The sound handle's assignment (`00418900`, thiscall(source)) and
/// release (`00483710`).
const HANDLE_ASSIGN: u32 = 0x0041_8900;
const HANDLE_RELEASE: u32 = 0x0048_3710;
/// The `bool` settings `Speak` reads (`00408d60`): `011e05ec` (lip sync),
/// `011d8928` (subtitles for everyone).
const LIP_SYNC_SETTING: u32 = 0x011e_05ec;
const ALL_SUBTITLES_SETTING: u32 = 0x011d_8928;
/// The byte at `011f122d` that also shows subtitles.
const SHOW_SUBTITLES: u32 = 0x011f_122d;
/// `float`s the listen timer is set to: 50.0 (`0101b268`), 2.0
/// (`010162c0`) and 10.0 (`01017b78`).
const LISTEN_LONG: u32 = 0x0101_b268;
const LISTEN_SHORT: u32 = 0x0101_62c0;
const LISTEN_DEFAULT: u32 = 0x0101_7b78;
/// `"AI: No responses found for conversation Topic Info..."`.
const NO_RESPONSES_MESSAGE: u32 = 0x0109_30cc;

/// `vcall(speaker, 0x284, ...)`, the actor's say call: the response's
/// values in the exe's evaluation order (`00441110`, `009611e0`,
/// `004048e0(006815c0)`, `0084e3a0`, `0044ddc0`), then the string, the
/// package's sound handle and `who` with the five trailing words. Answers
/// ST0.
fn say(e: &mut Engine, this: Ptr, speaker: u32, string: u32, who: u32, tail: [u32; 5]) -> f64 {
    let response = e.mem.u32(field(this, CURRENT_RESPONSE));
    let a = e.call(0x0044_1110, &args![response]).u32();
    let response = e.mem.u32(field(this, CURRENT_RESPONSE));
    let b = e.call(ITEM_SPEAKER, &args![response]).u32();
    let response = e.mem.u32(field(this, CURRENT_RESPONSE));
    let own = e.call(0x0068_15c0, &args![response]).u32();
    let c = e.call(0x0040_48e0, &args![own]).u32();
    let response = e.mem.u32(field(this, CURRENT_RESPONSE));
    let id = e.call(0x0084_e3a0, &args![response]).u32();
    let response = e.mem.u32(field(this, CURRENT_RESPONSE));
    let kind = e.call(0x0044_ddc0, &args![response]).u32();
    let text = e.call(WORD_AT, &args![string]).u32();
    e.vcall(
        speaker,
        0x284,
        &args![
            text,
            field(this, HANDLE),
            kind,
            id,
            c,
            b,
            a,
            who,
            tail[0],
            tail[1],
            tail[2],
            tail[3],
            tail[4]
        ],
    )
    .f64()
}

/// `008a5cf0(00543c30(response))` on `speaker` (the response's byte).
fn set_speaking(e: &mut Engine, this: Ptr, speaker: u32) {
    let response = e.mem.u32(field(this, CURRENT_RESPONSE));
    let byte = e.call(0x0054_3c30, &args![response]).u8();
    e.call(0x008a_5cf0, &args![speaker, u32::from(byte)]);
}

/// The audio manager's completion callback `callback` for the package's
/// sound handle, with `form`'s id.
fn completion_callback(e: &mut Engine, this: Ptr, callback: u32, form: u32) {
    let manager = e.call(AUDIO_MANAGER, &[]).u32();
    let id = e.call(0x0084_e3a0, &args![form]).u32();
    let handle = e.call(WORD_AT, &args![field(this, HANDLE)]).u32();
    e.call(
        SET_COMPLETION_CALLBACK,
        &args![manager, handle, callback, id],
    );
}

/// The talking activator's line (when its reference, `007af430`, is the
/// speaker): it becomes the talking actor and says the response to the
/// target (or to the starter when it is the target).
fn activator_says(e: &mut Engine, this: Ptr, speaker: u32, string: u32) -> Option<f64> {
    let activator = e.mem.u32(field(this, TALKING_ACTIVATOR));
    let reference = e.call(FORM_OF, &args![activator]).u32();
    if e.call(0x0051_6bf0, &args![reference]).u32() != speaker {
        return None;
    }
    let mut who = e.mem.u32(field(this, TARGET_ACTOR));
    if e.mem.u32(field(this, TARGET_ACTOR)) == e.mem.u32(field(this, TALKING_ACTIVATOR)) {
        who = e.mem.u32(field(this, ACTOR));
    }
    let activator = e.mem.u32(field(this, TALKING_ACTIVATOR));
    e.mem.set_u32(field(this, TALKING_ACTOR), activator);
    set_speaking(e, this, activator);
    Some(say(e, this, activator, string, who, [0, 0, 0, 1, 1]))
}

/// The process's virtual `slot` (no words) answers true.
fn process_says(e: &mut Engine, actor: u32, slot: u32) -> bool {
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    e.vcall(process, slot, &args![]).bool()
}

// Translated from 009ee0a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ee0a0 - C++ exception unwinding (FS:[0] frame state) and the /GS security-cookie check (00ec408c) are not translated.
/// `DialoguePackage::Speak` (Xbox PDB; thiscall(`speak`, a byte), `RET 4`).
/// Nothing while the talking actor still talks (its byte +0x80 clear) or
/// the actor's process is busy (virtual +0x11C). Unless waiting for a lip
/// file it advances the conversation: the first item and response (the
/// head tracking through the target's process, virtual +0x644(1) and
/// +0x634(actor), unless the package does not control the target), or the
/// next response; the next item when the actor (virtual +0x2E8, +0x22C(0))
/// has no response left. Without an item both actors get virtual +0x200(1)
/// (the target unless `GetDialogueDoNotControlTarget`); an item without a
/// response is reported. With a response and `speak`:
///
/// - an actor speaker (virtual +0x100) stops the current talking actor,
///   starts the response's sound (`00933150`, with the lip callback) when
///   the package plays sound files, then, with lip sync
///   (`011e05ec`) and an idle process (virtual +0x37C, +0x36C, +0x378),
///   sets the emotion (`0044ddc0`, `0084e3a0`, `00705b10`), runs the
///   item's result, says the line (virtual +0x284) with the talking
///   activator's line too, and waits for the lip file or sets the
///   completion callback and the listen timer (50); otherwise says the
///   line without lip sync, the process's virtual +0x380(0) and +0x374(0),
///   and the listen timer 2. Then the actor's and target's processes look
///   at each other (virtual +0x44).
/// - another speaker plays the response's text as a sound file (copied to
///   a 0x200-byte buffer, `00406d30`) through the talking activator or
///   `BSAudio` (`00ad7480` with flags 0x106, positioned at the speaker
///   (`00ad8be0`, `00ad8b60`, `00ad8f20`) and played (`00ad8870(1, 0)`).
///
/// The player's dialogue (or the byte at `011f122d`) shows the subtitle
/// (`Interface::ShowSubtitle`) when every subtitle is on (`011d8928`) or
/// the speaker always shows them (`Actor::AlwaysShowSubtitles`); a speaker
/// other than the target sets an expired listen timer to 10.
pub fn dialogue_package_speak(e: &mut Engine, this: Ptr, speak: u32) {
    let speak = speak as u8;
    let talking = e.mem.u32(field(this, TALKING_ACTOR));
    if talking != 0 && e.mem.u8(talking.wrapping_add(0x80)) == 0 {
        return;
    }
    let actor = e.mem.u32(field(this, ACTOR));
    if process_says(e, actor, 0x11c) {
        return;
    }
    // The exe also sets a local "started" byte here that nothing reads.
    if e.mem.u8(field(this, WAITING_FOR_LIP_FILE)) == 0 && advance(e, this) {
        return;
    }
    if e.mem.u32(field(this, CURRENT_ITEM)) == 0 {
        let actor = e.mem.u32(field(this, ACTOR));
        e.vcall(actor, 0x200, &args![1u32]);
        if !e.call(DO_NOT_CONTROL_TARGET, &args![this]).bool() {
            let target = e.mem.u32(field(this, TARGET_ACTOR));
            e.vcall(target, 0x200, &args![1u32]);
        }
        return;
    }
    if e.mem.u32(field(this, CURRENT_RESPONSE)) == 0 {
        let item = e.mem.u32(field(this, CURRENT_ITEM));
        e.call(ITEM_FIRST_RESPONSE, &args![item]);
        let item = e.mem.u32(field(this, CURRENT_ITEM));
        let response = e.call(GET_CURRENT, &args![item]).u32();
        e.mem.set_u32(field(this, CURRENT_RESPONSE), response);
        if e.mem.u32(field(this, CURRENT_RESPONSE)) == 0 {
            let item = e.mem.u32(field(this, CURRENT_ITEM));
            let info = e.call(0x0084_e3a0, &args![item]).u32();
            let id = e.call(0x0084_e3a0, &args![info]).u32();
            e.call(0x005b_5e40, &args![NO_RESPONSES_MESSAGE, id]);
        }
    }
    if e.mem.u32(field(this, CURRENT_RESPONSE)) == 0 {
        return;
    }
    let item = e.mem.u32(field(this, CURRENT_ITEM));
    let speaker = e.call(ITEM_SPEAKER, &args![item]).u32();
    if e.vcall(speaker, 0x100, &args![]).bool() {
        let item = e.mem.u32(field(this, CURRENT_ITEM));
        let speaker = e.call(ITEM_SPEAKER, &args![item]).u32();
        let mut unanswered = true;
        if speaker != 0 && speak != 0 {
            e.with_stack(0x40, |e, frame| {
                let ebp = frame.addr() + 0x40;
                unanswered = actor_speaks(e, this, speaker, ebp);
            });
        }
        if unanswered {
            look_at_each_other(e, this);
        }
    } else if speaker != 0 && speak != 0 {
        e.with_stack(0x2a0, |e, frame| {
            let ebp = frame.addr() + 0x2a0;
            other_speaks(e, this, speaker, ebp);
        });
    }
}

/// The conversation step of [`dialogue_package_speak`] (not waiting for a
/// lip file).
fn advance(e: &mut Engine, this: Ptr) -> bool {
    if e.mem.u32(field(this, CURRENT_ITEM)) == 0 {
        if e.mem.u32(field(this, CONVERSATION)) == 0 {
            let actor = e.mem.u32(field(this, ACTOR));
            e.vcall(actor, 0x200, &args![1u32]);
            let target = e.mem.u32(field(this, TARGET_ACTOR));
            e.vcall(target, 0x200, &args![1u32]);
            // The exe leaves `Speak` here.
            return true;
        }
        if fn_009ef510(e, this) != 0 && e.mem.u8(field(this, DO_NOT_CONTROL_TARGET_MOVEMENT)) == 0 {
            let target = e.mem.u32(field(this, TARGET_ACTOR));
            let process = e.call(GET_PROCESS, &args![target]).u32();
            e.vcall(process, 0x644, &args![1u32]);
            let target = e.mem.u32(field(this, TARGET_ACTOR));
            let process = e.call(GET_PROCESS, &args![target]).u32();
            let actor = e.mem.u32(field(this, ACTOR));
            e.vcall(process, 0x634, &args![actor]);
        }
        let conversation = e.mem.u32(field(this, CONVERSATION));
        e.call(CONVERSATION_FIRST_ITEM, &args![conversation]);
        let conversation = e.mem.u32(field(this, CONVERSATION));
        let item = e.call(GET_CURRENT, &args![conversation]).u32();
        e.mem.set_u32(field(this, CURRENT_ITEM), item);
        if e.mem.u32(field(this, CURRENT_ITEM)) != 0 {
            let item = e.mem.u32(field(this, CURRENT_ITEM));
            e.call(ITEM_FIRST_RESPONSE, &args![item]);
            let item = e.mem.u32(field(this, CURRENT_ITEM));
            let response = e.call(GET_CURRENT, &args![item]).u32();
            e.mem.set_u32(field(this, CURRENT_RESPONSE), response);
        } else {
            let actor = e.mem.u32(field(this, ACTOR));
            e.vcall(actor, 0x200, &args![1u32]);
            if !e.call(DO_NOT_CONTROL_TARGET, &args![this]).bool() {
                let target = e.mem.u32(field(this, TARGET_ACTOR));
                e.vcall(target, 0x200, &args![1u32]);
            }
        }
    } else if e.mem.u32(field(this, CURRENT_RESPONSE)) != 0 {
        let item = e.mem.u32(field(this, CURRENT_ITEM));
        e.call(ITEM_NEXT_RESPONSE, &args![item]);
        let item = e.mem.u32(field(this, CURRENT_ITEM));
        let response = e.call(GET_CURRENT, &args![item]).u32();
        e.mem.set_u32(field(this, CURRENT_RESPONSE), response);
    }
    let actor = e.mem.u32(field(this, ACTOR));
    if actor == 0 {
        return false;
    }
    let busy = e.vcall(actor, 0x2e8, &args![]).bool() || {
        let actor = e.mem.u32(field(this, ACTOR));
        e.vcall(actor, 0x22c, &args![0u32]).bool()
    };
    if !busy {
        if e.mem.u32(field(this, CURRENT_RESPONSE)) == 0 {
            let conversation = e.mem.u32(field(this, CONVERSATION));
            e.call(CONVERSATION_NEXT_ITEM, &args![conversation]);
            let conversation = e.mem.u32(field(this, CONVERSATION));
            let item = e.call(GET_CURRENT, &args![conversation]).u32();
            e.mem.set_u32(field(this, CURRENT_ITEM), item);
        }
    } else if e.mem.u32(field(this, CURRENT_RESPONSE)) == 0 {
        e.mem.set_u32(field(this, CURRENT_ITEM), 0);
    }
    false
}

/// The actor speaker's line on its frame (`ebp`); answers whether the
/// actors should still look at each other (the exe's `[ebp-0xE]`).
fn actor_speaks(e: &mut Engine, this: Ptr, speaker: u32, ebp: u32) -> bool {
    let string = ebp - 0x20;
    e.call(STRING_CONSTRUCT, &args![string]);
    let talking = e.mem.u32(field(this, TALKING_ACTOR));
    if talking != 0 {
        e.call(STOP_CURRENT_DIALOGUE, &args![talking]);
        e.mem.set_u32(field(this, TALKING_ACTOR), 0);
    }
    let mut speak_now = true;
    let response = e.mem.u32(field(this, CURRENT_RESPONSE));
    if e.call(FORM_OF, &args![response]).u32() != 0 && e.mem.u8(field(this, SOUND_FILE)) != 0 {
        let response = e.mem.u32(field(this, CURRENT_RESPONSE));
        let sound = e.call(FORM_OF, &args![response]).u32();
        let id = e.call(0x0084_e3a0, &args![sound]).u32();
        let handle = ebp - 0x38;
        e.call(
            0x0093_3150,
            &args![speaker, handle, id, 0u32, 0x4000_0102u32, 1u32],
        );
        let manager = e.call(AUDIO_MANAGER, &[]).u32();
        let actor = e.mem.u32(field(this, ACTOR));
        let actor_id = e.call(0x0084_e3a0, &args![actor]).u32();
        let handle_id = e.call(WORD_AT, &args![handle]).u32();
        e.call(
            SET_COMPLETION_CALLBACK,
            &args![manager, handle_id, LIP_FILE_CALLBACK, actor_id],
        );
        e.mem.set_u8(field(this, SOUND_FILE), 0);
        speak_now = false;
        e.call(HANDLE_RELEASE, &args![handle]);
    }
    let mut unanswered = true;
    if speak_now && e.mem.u8(field(this, SOUND_FILE)) != 0 {
        let response = e.mem.u32(field(this, CURRENT_RESPONSE));
        let text = e.call(RESPONSE_TEXT, &args![response]).u32();
        e.call(STRING_SET, &args![string, text]);
        let lip_synced = {
            let process = e.call(GET_PROCESS, &args![speaker]).u32();
            e.vcall(process, 0x37c, &args![]).u32() == 0
        } && !process_says(e, speaker, 0x36c)
            && {
                let value = e.call(0x0040_8d60, &args![LIP_SYNC_SETTING]).u32();
                e.mem.u8(value) != 0
            }
            && !process_says(e, speaker, 0x378);
        if lip_synced {
            speak_with_lip_sync(e, this, speaker, string);
            e.call(STRING_DESTRUCT, &args![string]);
            // The exe leaves `Speak` here.
            return false;
        }
        if process_says(e, speaker, 0x36c) && !process_says(e, speaker, 0x378) {
            e.call(STRING_DESTRUCT, &args![string]);
            return false;
        }
        let value = e.call(0x0040_8d60, &args![LIP_SYNC_SETTING]).u32();
        if e.mem.u8(value) == 0 && e.mem.u32(field(this, CURRENT_ITEM)) != 0 {
            let item = e.mem.u32(field(this, CURRENT_ITEM));
            e.call(ITEM_RUN_RESULT, &args![item, 0u32]);
        }
        e.mem.set_u32(field(this, TALKING_ACTOR), speaker);
        set_speaking(e, this, speaker);
        let use_idle = u32::from(e.mem.u8(field(this, USE_IDLE_MANAGER)));
        let target = e.mem.u32(field(this, TARGET_ACTOR));
        say(e, this, speaker, string, target, [1, 0, 0, 1, use_idle]);
        // The exe tests the talking activator twice here.
        if e.mem.u32(field(this, TALKING_ACTIVATOR)) != 0 {
            let activator = e.mem.u32(field(this, TALKING_ACTIVATOR));
            if activator != 0 {
                activator_says(e, this, speaker, string);
            }
        }
        let actor = e.mem.u32(field(this, ACTOR));
        completion_callback(e, this, DIALOGUE_DONE_CALLBACK, actor);
        e.mem.set_u8(speaker.wrapping_add(0x80), 0);
        unanswered = false;
        e.mem.set_u32(field(this, TALKING_ACTOR), speaker);
        let process = e.call(GET_PROCESS, &args![speaker]).u32();
        e.vcall(process, 0x380, &args![0u32]);
        e.mem.set_u8(field(this, WAITING_FOR_LIP_FILE), 0);
        let process = e.call(GET_PROCESS, &args![speaker]).u32();
        e.vcall(process, 0x374, &args![0u32]);
        if e.mem.u32(field(this, ACTOR)) != e.mem.u32(field(this, TARGET_ACTOR)) {
            let timer = x87_quiet_bits(e.global::<u32>(LISTEN_SHORT));
            e.mem.set_u32(field(this, LISTEN_TIMER), timer);
        }
        let talking = e.mem.u32(field(this, TALKING_ACTOR));
        e.vcall(talking, 0x1e4, &args![]);
        let target = e.mem.u32(field(this, TARGET_ACTOR));
        e.vcall(target, 0x1e4, &args![]);
    }
    e.call(STRING_DESTRUCT, &args![string]);
    unanswered
}

/// The lip-synced line of [`actor_speaks`].
fn speak_with_lip_sync(e: &mut Engine, this: Ptr, speaker: u32, string: u32) {
    let response = e.mem.u32(field(this, CURRENT_RESPONSE));
    let emotion = e.call(0x0044_ddc0, &args![response]).u32();
    e.mem.set_u32(field(this, EMOTION), emotion);
    let response = e.mem.u32(field(this, CURRENT_RESPONSE));
    let value = e.call(0x0084_e3a0, &args![response]).u32();
    e.mem.set_u32(field(this, EMOTION_VALUE), value);
    let emotion = e.mem.u32(field(this, EMOTION));
    e.call(0x0070_5b10, &args![this, emotion]);
    if e.mem.u32(field(this, CURRENT_ITEM)) != 0 {
        let item = e.mem.u32(field(this, CURRENT_ITEM));
        e.call(ITEM_RUN_RESULT, &args![item, 0u32]);
    }
    e.mem.set_u32(field(this, TALKING_ACTOR), speaker);
    set_speaking(e, this, speaker);
    let use_idle = u32::from(e.mem.u8(field(this, USE_IDLE_MANAGER)));
    let target = e.mem.u32(field(this, TARGET_ACTOR));
    // The duration (ST0) is stored in a local nothing reads.
    say(e, this, speaker, string, target, [1, 0, 1, 1, use_idle]);
    if e.mem.u32(field(this, TALKING_ACTIVATOR)) != 0 {
        activator_says(e, this, speaker, string);
    }
    if process_says(e, speaker, 0x36c) {
        e.mem.set_u8(field(this, WAITING_FOR_LIP_FILE), 1);
    } else {
        let actor = e.mem.u32(field(this, ACTOR));
        completion_callback(e, this, DIALOGUE_DONE_CALLBACK, actor);
        if e.mem.u32(field(this, ACTOR)) != e.mem.u32(field(this, TARGET_ACTOR)) {
            let timer = x87_quiet_bits(e.global::<u32>(LISTEN_LONG));
            e.mem.set_u32(field(this, LISTEN_TIMER), timer);
        }
        e.mem.set_u8(speaker.wrapping_add(0x80), 0);
    }
}

/// The actor's and the target's processes' virtual +0x44 toward each
/// other (`(actor, 0, 2, 0, 1, 1)`), the target's only when the talking
/// actor is someone else with a process.
fn look_at_each_other(e: &mut Engine, this: Ptr) {
    let actor = e.mem.u32(field(this, ACTOR));
    if e.call(GET_PROCESS, &args![actor]).u32() != 0 {
        let actor = e.mem.u32(field(this, ACTOR));
        let process = e.call(GET_PROCESS, &args![actor]).u32();
        let actor = e.mem.u32(field(this, ACTOR));
        e.vcall(process, 0x44, &args![actor, 0u32, 2u32, 0u32, 1u32, 1u32]);
    }
    let talking = e.mem.u32(field(this, TALKING_ACTOR));
    if e.mem.u32(field(this, ACTOR)) != talking
        && talking != 0
        && e.call(GET_PROCESS, &args![talking]).u32() != 0
    {
        let target = e.mem.u32(field(this, TARGET_ACTOR));
        let process = e.call(GET_PROCESS, &args![target]).u32();
        let target = e.mem.u32(field(this, TARGET_ACTOR));
        e.vcall(process, 0x44, &args![target, 0u32, 2u32, 0u32, 1u32, 1u32]);
    }
}

/// The non-actor speaker's line on its frame (`ebp`).
fn other_speaks(e: &mut Engine, this: Ptr, speaker: u32, ebp: u32) {
    let string = ebp - 0x26c;
    let text_string = ebp - 0x274;
    let file = ebp - 0x264;
    e.call(STRING_CONSTRUCT, &args![string]);
    let response = e.mem.u32(field(this, CURRENT_RESPONSE));
    let text = e.call(RESPONSE_TEXT, &args![response]).u32();
    e.call(STRING_FROM, &args![text_string, text]);
    let response = e.mem.u32(field(this, CURRENT_RESPONSE));
    let text = e.call(RESPONSE_TEXT, &args![response]).u32();
    if e.call(WORD_AT, &args![text]).u32() != 0 {
        let response = e.mem.u32(field(this, CURRENT_RESPONSE));
        let text = e.call(RESPONSE_TEXT, &args![response]).u32();
        let chars = e.call(WORD_AT, &args![text]).u32();
        e.call(0x0040_6d30, &args![file, 0x200u32, chars]);
        let talking = e.mem.u32(field(this, TALKING_ACTOR));
        if talking != 0 {
            e.call(STOP_CURRENT_DIALOGUE, &args![talking]);
            e.mem.set_u32(field(this, TALKING_ACTOR), 0);
        }
        let response = e.mem.u32(field(this, CURRENT_RESPONSE));
        let text = e.call(RESPONSE_TEXT, &args![response]).u32();
        e.call(STRING_SET, &args![string, text]);
        if e.mem.u32(field(this, TALKING_ACTIVATOR)) != 0 {
            let activator = e.mem.u32(field(this, TALKING_ACTIVATOR));
            // The activator's reference is fetched into a local nothing reads.
            e.call(FORM_OF, &args![activator]);
            set_speaking(e, this, activator);
            let activator = e.mem.u32(field(this, TALKING_ACTIVATOR));
            e.mem.set_u32(field(this, TALKING_ACTOR), activator);
            let target = e.mem.u32(field(this, TARGET_ACTOR));
            say(e, this, activator, string, target, [0, 0, 0, 1, 1]);
            let activator = e.mem.u32(field(this, TALKING_ACTIVATOR));
            completion_callback(e, this, DIALOGUE_DONE_CALLBACK, activator);
            let activator = e.mem.u32(field(this, TALKING_ACTIVATOR));
            e.mem.set_u8(activator.wrapping_add(0x80), 0);
        } else {
            // The exe builds the flags as `0x104 | 2`.
            let flags = 0x104u32 | 2;
            let handle = ebp - 0x298;
            let audio = e.call(0x0045_3a70, &[]).u32();
            let made = e
                .call(0x00ad_7480, &args![audio, handle, file, flags, 0u32])
                .u32();
            e.call(HANDLE_ASSIGN, &args![field(this, HANDLE), made]);
            e.call(HANDLE_RELEASE, &args![handle]);
            let position = e.vcall(speaker, 0x1f4, &args![]).u32();
            let point = [
                e.mem.u32(position),
                e.mem.u32(position.wrapping_add(4)),
                e.mem.u32(position.wrapping_add(8)),
            ];
            // `BSSoundHandle::SetMinMax` (engine map) gets `005790b0`'s
            // `float` first and `005790d0`'s second; `005790d0` runs first.
            let second = x87_store32(e.call(0x0057_90d0, &[]).f64());
            let first = x87_store32(e.call(0x0057_90b0, &[]).f64());
            e.call(0x00ad_8be0, &args![field(this, HANDLE), first, second]);
            e.call(
                0x00ad_8b60,
                &args![
                    field(this, HANDLE),
                    x87_quiet_bits(point[0]),
                    x87_quiet_bits(point[1]),
                    x87_quiet_bits(point[2])
                ],
            );
            let root = e.vcall(speaker, 0x1d0, &args![]).u32();
            e.call(0x00ad_8f20, &args![field(this, HANDLE), root]);
            e.call(0x00ad_8870, &args![field(this, HANDLE), 1u32, 0u32]);
        }
    }
    subtitle(e, this, speaker);
    let timer = e.mem.u32(field(this, LISTEN_TIMER));
    // `FCOMP` with the `double` 0, `TEST AH,0x41`/`JP`: an ordered "at
    // most" passes.
    if x87_load32(timer) <= e.global::<f64>(0x0101_2060)
        && e.mem.u32(field(this, TARGET_ACTOR)) != speaker
    {
        let timer = x87_quiet_bits(e.global::<u32>(LISTEN_DEFAULT));
        e.mem.set_u32(field(this, LISTEN_TIMER), timer);
    }
    e.call(STRING_DESTRUCT, &args![text_string]);
    e.call(STRING_DESTRUCT, &args![string]);
}

/// The subtitle of [`other_speaks`]: for the player's dialogue package
/// (the player's +0x208) or with the byte at `011f122d`, when the
/// conversation's current item has a current response with text.
fn subtitle(e: &mut Engine, this: Ptr, speaker: u32) {
    let player = e.global::<u32>(PLAYER);
    let players = e.mem.u32(player.wrapping_add(0x208)) == this.addr();
    if !players && e.mem.u8(SHOW_SUBTITLES) == 0 {
        return;
    }
    let text = |e: &mut Engine, this: Ptr, depth: u32| {
        let conversation = e.mem.u32(field(this, CONVERSATION));
        let mut value = e.call(GET_CURRENT, &args![conversation]).u32();
        for _ in 1..depth {
            value = e.call(GET_CURRENT, &args![value]).u32();
        }
        value
    };
    if text(e, this, 1) == 0 || text(e, this, 2) == 0 {
        return;
    }
    let response = text(e, this, 2);
    let own = e.call(0x0068_15c0, &args![response]).u32();
    if e.call(WORD_AT, &args![own]).u32() == 0 {
        return;
    }
    let value = e.call(0x0040_8d60, &args![ALL_SUBTITLES_SETTING]).u32();
    let shown = e.mem.u8(value) != 0
        || (e.vcall(speaker, 0x100, &args![]).bool()
            && e.call(0x008c_1bc0, &args![speaker]).bool());
    if !shown {
        return;
    }
    let target = e.mem.u32(field(this, TARGET_ACTOR));
    let position = e.vcall(speaker, 0x1f4, &args![]).u32();
    let point = [
        e.mem.u32(position),
        e.mem.u32(position.wrapping_add(4)),
        e.mem.u32(position.wrapping_add(8)),
    ];
    e.with_stack(0xc, |e, copy| {
        e.call(HANDLE_ASSIGN, &args![copy, field(this, HANDLE)]);
        let handle = [
            e.mem.u32(copy.addr()),
            e.mem.u32(copy.addr() + 4),
            e.mem.u32(copy.addr() + 8),
        ];
        let response = text(e, this, 2);
        let own = e.call(0x0068_15c0, &args![response]).u32();
        let chars = e.call(WORD_AT, &args![own]).u32();
        e.call(
            0x0070_5210,
            &args![
                chars, handle[0], handle[1], handle[2], point[0], point[1], point[2], target, 0u32
            ],
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d80e0, dialogue_package_get_target_of_conversation(Ptr) -> Ptr),
        entry!(0x009ee040, dialogue_package_get_actor_started_conversation(Ptr) -> Ptr),
        entry!(
            0x009edd80,
            dialogue_package_dialogue_package(Ptr, Ptr, Ptr, Ptr, Ptr, u8, u8) -> Ptr
        ),
        entry!(0x009f01d0, dialogue_package_save_game(Ptr, Ptr)),
        entry!(0x009e_e060, fn_009ee060(Ptr, u32)),
        entry!(0x009e_e0a0, dialogue_package_speak(Ptr, u32)),
        entry!(0x009e_f510, fn_009ef510(Ptr) -> u8),
        entry!(0x009e_f530, fn_009ef530(Ptr)),
        entry!(0x009e_f560, fn_009ef560(Ptr) -> u16),
        entry!(0x009e_f6e0, fn_009ef6e0(Ptr)),
        entry!(0x009e_fa50, dialogue_package_load_game(Ptr)),
        entry!(0x009e_ff60, dialogue_package_init_load_game(Ptr)),
        entry!(0x009f_0460, dialogue_package_load_game_ov2(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn getters() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xd0);
        e.mem.set_u32(this + 0xb0, 0x1111);
        e.mem.set_u32(this + 0xb4, 0x2222);
        assert_eq!(e.call(0x008d80e0, &args![this]).u32(), 0x2222);
        assert_eq!(e.call(0x009ee040, &args![this]).u32(), 0x1111);
    }

    #[test]
    fn constructor_sets_the_package_up() {
        const ACTOR_DOUBLE: u32 = 0x7200_0200;
        for with_conversation in [true, false] {
            let mut e = Engine::new();
            e.register(PACKAGE_CONSTRUCTOR, |_, _| Ret::default());
            e.register(SOUND_HANDLE_CONSTRUCTOR, |_, _| Ret::default());
            e.register(CONVERSATION_FIRST_ITEM, |_, _| Ret::default());
            e.register(CHECK_FOR_CLOSE_CONVERSATION, |_, _| Ret::default());
            e.register(ACTOR_DOUBLE, |_, _| Ret::default());
            e.put_vtable(0x7000_0000, &[0; 0x204 / 4]);
            e.mem.set_u32(0x7000_0000 + 0x200, ACTOR_DOUBLE);
            e.map(0x011d_e000, 0x1000);
            e.mem.set_u32(PLAYER, 0x9999);
            let actor = e.mem.alloc(4);
            e.mem.set_u32(actor, 0x7000_0000);
            let this = e.mem.alloc(0xd0);
            let conversation = if with_conversation { 0x5555u32 } else { 0 };
            e.call_log = Some(vec![]);
            let r = e
                .call(
                    0x009edd80,
                    &args![
                        this,
                        conversation,
                        actor,
                        0x3333u32,
                        0x4444u32,
                        1u32,
                        0x101u32
                    ],
                )
                .u32();
            assert_eq!(r, this);
            assert_eq!(e.mem.u32(this), DIALOGUE_PACKAGE_VTABLE);
            assert_eq!(calls(&e, SOUND_HANDLE_CONSTRUCTOR), vec![vec![this + 0x80]]);
            assert_eq!(
                calls(&e, CONVERSATION_FIRST_ITEM).len(),
                usize::from(with_conversation)
            );
            assert_eq!(calls(&e, ACTOR_DOUBLE), vec![vec![actor, 0]]);
            assert_eq!(
                calls(&e, CHECK_FOR_CLOSE_CONVERSATION),
                vec![vec![0x9999, actor, 0x3333, this]]
            );
            assert_eq!(e.mem.u32(this + 0xa4), conversation);
            assert_eq!(e.mem.u32(this + 0xb4), 0x3333);
            assert_eq!(e.mem.u32(this + 0xb8), 0x4444);
            assert_eq!(e.mem.u8(this + 0xbd), 1);
            assert_eq!(e.mem.u8(this + 0xa0), 1);
            assert_eq!(e.mem.u8(this + 0xbc), 1);
            assert_eq!(e.mem.u8(this + 0xcc), 1);
            assert_eq!(e.mem.u32(this + 0xc8), 50);
        }
    }

    #[test]
    fn save_game_writes_everything_and_the_conversation_indexes() {
        for (conversation, item, response) in [(0u32, 0u32, 0u32), (0x50, 0x60, 0x70), (0x50, 0, 0)]
        {
            let mut e = Engine::new();
            e.register(PACKAGE_SAVE_GAME, |_, _| Ret::default());
            e.register(SAVE_FORM_ID, |_, _| Ret::default());
            e.map(0x7300_0000, 0x10);
            e.register(SAVE_DATA, |e, a| {
                if a[2] == 2 {
                    let n = e.mem.u32(0x7300_0000);
                    e.mem.set_u16(0x7300_0004 + 2 * n, e.mem.u16(a[1]));
                    e.mem.set_u32(0x7300_0000, n + 1);
                }
                Ret::default()
            });
            e.register(CONVERSATION_SAVE_GAME, |_, _| Ret::default());
            e.register(CONVERSATION_ITEM_INDEX, |_, _| 3u32.into_ret());
            e.register(ITEM_RESPONSE_INDEX, |_, _| 0xffff_0004u32.into_ret());
            let this = e.mem.alloc(0xd0);
            e.mem.set_u32(this + 0xa4, conversation);
            e.mem.set_u32(this + 0xa8, item);
            e.mem.set_u32(this + 0xac, response);
            e.mem.set_u32(this + 0x94, 0x1001);
            e.call_log = Some(vec![]);
            e.call(0x009f01d0, &args![this, 0x8000u32]);
            let forms = calls(&e, SAVE_FORM_ID);
            assert_eq!(forms.len(), 6);
            assert_eq!(forms[0], vec![0x8000, 0x1001, 0]);
            let data = calls(&e, SAVE_DATA);
            assert_eq!(data[0], vec![0x8000, this + 0x90, 4, 0]);
            if conversation == 0 {
                assert_eq!(data.len(), 14);
                assert!(calls(&e, CONVERSATION_SAVE_GAME).is_empty());
            } else {
                assert_eq!(data.len(), 16);
                assert_eq!(calls(&e, CONVERSATION_SAVE_GAME), vec![vec![0x50, 0x8000]]);
                let saved = (e.mem.u16(0x7300_0004), e.mem.u16(0x7300_0006));
                if item != 0 {
                    assert_eq!(saved, (3, 4));
                } else {
                    assert_eq!(saved, (0xffff, 0xffff));
                }
            }
        }
    }
}

#[cfg(test)]
mod n026_tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];
    type Logged = Vec<(u32, Vec<u32>)>;

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn word(value: u32) -> Ret {
        value.into_ret()
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// An object whose vtable slots answer the given results (each slot a
    /// double at `vtable + 0x10_0000 + slot`).
    fn object(e: &mut Engine, vtable: u32, slots: &[(u32, Ret)]) -> u32 {
        let size = slots.iter().map(|(s, _)| *s).max().unwrap_or(0) / 4 + 1;
        let mut words = vec![0u32; size as usize];
        for (slot, ret) in slots {
            let target = vtable + 0x10_0000 + slot;
            words[*slot as usize / 4] = target;
            let ret = *ret;
            e.register_double(target, move |_, _| ret);
        }
        e.put_vtable(vtable, &words);
        let object = e.mem.alloc(0x300);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn small_accessors() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0xd0));
        e.mem.set_u8(this.addr() + 0xbc, 7);
        assert_eq!(e.call(0x009e_f510, &args![this]).u8(), 7);
        quiet(
            &mut e,
            &[STOP_CURRENT_DIALOGUE, 0x0067_3290, RT_DYNAMIC_CAST],
        );
        e.call_log = Some(vec![]);
        e.call(0x009e_f530, &args![this]);
        e.mem.set_u32(this.addr() + 0x94, 0x55);
        e.call(0x009e_f530, &args![this]);
        e.call(0x009e_e060, &args![this, 0x66u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, STOP_CURRENT_DIALOGUE), vec![vec![0x55]]);
        assert_eq!(calls(&log, 0x0067_3290), vec![vec![this.addr(), 0x66]]);
        assert_eq!(
            calls(&log, RT_DYNAMIC_CAST),
            vec![vec![0x66, 0, TYPE_TES_FORM, TYPE_DIALOGUE_PACKAGE, 0]]
        );
    }

    /// The save/load manager with `blocks`, version `version`, logging
    /// `logging` and a raw buffer position that counts up 6 per write.
    fn manager(e: &mut Engine, blocks: u32, version: u32, logging: u32, form: u32) -> u32 {
        e.map(0x011d_e000, 0x1000);
        e.set_global(SAVE_LOAD_GAME, 0x0b00_0000u32);
        e.register_double(USE_SAVE_GAME_BLOCKS, move |_, _| word(blocks));
        e.register_double(SAVE_VERSION, move |_, _| word(version));
        e.mem.set_u8(SAVE_LOGGING_SETTING + 4, logging as u8);
        e.register(BOOL_SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register_double(SAVED_FORM, move |_, _| word(form));
        e.register_double(LOADED_FORM, move |_, _| word(form));
        let position = e.mem.alloc(0x100);
        let cursor = std::rc::Rc::new(std::cell::Cell::new(position));
        let at = cursor.clone();
        e.register_double(BUFFER_POSITION, move |_, _| word(at.get()));
        let step = cursor;
        e.register_double(RAW_WRITE, move |_, a| {
            step.set(step.get() + a[2]);
            Ret::default()
        });
        quiet(e, &[ERROR, PRINT]);
        position
    }

    #[test]
    fn save_size_counts_the_parts() {
        let mut e = Engine::new();
        manager(&mut e, 1, 0x6a, 1, 0);
        let this: Ptr = Ptr::new(e.mem.alloc(0xd0));
        e.register(0x0067_8fd0, |_, _| word(0xfff0));
        e.call_log = Some(vec![]);
        // 0xfff0 + 6 + 4 + 1 + 0x12 = 0x1_000d, 16-bit: 0x000d.
        assert_eq!(e.call(0x009e_f560, &args![this]).u16(), 0x000d);
        e.mem.set_u32(this.addr() + 0xa4, 0x99);
        e.register(0x0083_bb30, |_, _| word(0x10));
        assert_eq!(e.call(0x009e_f560, &args![this]).u16(), 0x0021);
        let log = e.call_log.take().unwrap();
        let difference = (0x000d_i32 - 0xfff0) as u32;
        assert_eq!(
            calls(&log, ERROR)[0],
            vec![0x0101_2c78, difference, 0x1f4, SOURCE_FILE]
        );
        // With the form being saved logged.
        let mut e = Engine::new();
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record, 0x1234);
        e.mem.set_u32(record + 5, 0x77);
        manager(&mut e, 0, 0x10, 1, record);
        e.register(0x0067_8fd0, |_, _| word(2));
        let form = object(&mut e, 0x0a00_0000, &[(0x130, word(0x0abc))]);
        e.register_double(FORM_FOR_ID, move |_, _| word(form));
        let this: Ptr = Ptr::new(e.mem.alloc(0xd0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009e_f560, &args![this]).u16(), 0x18);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, ERROR)[0],
            vec![0x0101_2cb0, 0x16, 0x1234, 0x0abc, 0x77, 0x1f4, SOURCE_FILE]
        );
    }

    #[test]
    fn old_save_writes_ids_and_the_block_length() {
        let mut e = Engine::new();
        let base = manager(&mut e, 1, 0x6a, 0, 0);
        let this: Ptr = Ptr::new(e.mem.alloc(0xd0));
        e.mem.set_u32(this.addr() + 0x94, 0x0c00_0000);
        e.mem.set_u32(this.addr() + 0xb4, 0x0c10_0000);
        e.mem.set_u32(this.addr() + 0xa4, 0x0c20_0000);
        e.mem.set_u32(this.addr() + 0xa8, 0x0c30_0000);
        quiet(&mut e, &[0x0067_9120, 0x0083_bc80]);
        e.register(0x0084_e3a0, |_, a| (a[0] >> 16).into_ret());
        e.register(0x0083_bb30, |_, _| word(3));
        e.register(CONVERSATION_ITEM_INDEX, |_, _| word(0xab));
        // Each old-format write moves the buffer as well.
        let saved = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let into = saved.clone();
        e.register_double(SAVE_DATA_OLD, move |e, a| {
            into.borrow_mut().push((a[1], a[2], e.mem.u32(a[1])));
            Ret::default()
        });
        let ids = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let id_list = ids.clone();
        e.register_double(SAVE_NUMERIC_ID, move |e, a| {
            id_list.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009e_f6e0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(ids.borrow().clone(), vec![0x0c00, 0, 0x0c10, 0]);
        let saved = saved.borrow().clone();
        assert_eq!(saved[0].0, this.addr() + 0x90);
        assert_eq!(saved[1].0, this.addr() + 0x98);
        assert_eq!(saved[2].2 & 0xffff, 0xab);
        assert_eq!(saved[3].2 & 0xffff, 0xffff);
        assert_eq!(calls(&log, 0x0083_bc80), vec![vec![0x0c20_0000]]);
        // The header (4), the length (2) and the size (2) went through the
        // raw writer; the length at the block start is the 2 bytes after it.
        let writes = calls(&log, RAW_WRITE);
        assert_eq!(writes.len(), 3);
        assert_eq!(e.mem.u16(base + 4), 4);
    }

    #[test]
    fn old_load_reads_by_version() {
        for version in [0x10u32, 0x6a] {
            let mut e = Engine::new();
            manager(&mut e, 1, version, 0, 0);
            let this: Ptr = Ptr::new(e.mem.alloc(0xd0));
            quiet(&mut e, &[0x0067_9340, 0x0083_bec0]);
            e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register(CONVERSATION_CONSTRUCTOR, |_, a| a[0].into_ret());
            e.register(RAW_READ, |e, a| {
                if a[2] == 4 {
                    e.mem.set_u32(a[1], 0x1111);
                }
                Ret::default()
            });
            e.register(LOAD_NUMERIC_ID, |e, a| {
                e.mem.set_u32(a[1], 0x0d00);
                Ret::default()
            });
            e.register(LOAD_DATA_OLD, |e, a| {
                match a[2] {
                    4 => e.mem.set_u32(a[1], 0x0e00_0000),
                    2 => e.mem.set_u16(a[1], 0xffff),
                    _ => {}
                }
                Ret::default()
            });
            e.register(FORM_FOR_ID, |_, a| (a[0] + 1).into_ret());
            e.register(RT_DYNAMIC_CAST, |_, a| {
                assert_eq!(a[3], TYPE_TES_TOPIC);
                (a[0] + 1).into_ret()
            });
            e.call_log = Some(vec![]);
            e.call(0x009e_fa50, &args![this]);
            let log = e.call_log.take().unwrap();
            // A wrong block header is reported with the version.
            assert_eq!(
                calls(&log, PRINT)[0][..3],
                [0x0101_56a8, SOURCE_FILE, 0x239]
            );
            assert_eq!(e.mem.u32(this.addr() + 0x8c), 0x0d02);
            assert_eq!(e.mem.u32(this.addr() + 0xb0), 0x0d00);
            if version < 0x20 {
                // The old conversation word and the indexes as read.
                assert_eq!(e.mem.u32(this.addr() + 0xa4), 0x0e00_0000);
                assert_eq!(e.mem.u32(this.addr() + 0xa8), u32::MAX);
            } else {
                // A saved size of 0xFFFF builds the conversation; index -1:
                // no item and no response.
                assert_eq!(e.mem.u32(this.addr() + 0xa8), 0);
                assert_eq!(e.mem.u32(this.addr() + 0xac), 0);
                assert_eq!(calls(&log, CONVERSATION_CONSTRUCTOR).len(), 1);
            }
        }
    }

    #[test]
    fn init_load_rebuilds_an_old_conversation() {
        let mut e = Engine::new();
        manager(&mut e, 0, 0x10, 0, 0);
        let this: Ptr = Ptr::new(e.mem.alloc(0xd0));
        e.mem.set_u32(this.addr() + 0x94, 0x11);
        e.mem.set_u32(this.addr() + 0xb0, 0x12);
        e.mem.set_u32(this.addr() + 0xb4, 0x13);
        e.mem.set_u32(this.addr() + 0x8c, 0x14);
        e.mem.set_u32(this.addr() + 0xa4, 0x99);
        e.mem.set_u32(this.addr() + 0xa8, 2);
        e.mem.set_u32(this.addr() + 0xac, 0xffff);
        quiet(&mut e, &[0x0067_96c0, 0x0083_c8a0]);
        e.register(FORM_FOR_ID, |_, a| (a[0] + 0x100).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| (a[0] + 0x1000).into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0083_b850, |_, a| a[0].into_ret());
        e.register(ITEM_BY_INDEX, |_, a| (0x5000 + a[1]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009e_ff60, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this.addr() + 0x94), 0x1111);
        let built = calls(&log, 0x0083_b850);
        assert_eq!(built[0][1..], [0x1112, 0x1113, 0x14]);
        assert_eq!(e.mem.u32(this.addr() + 0xa8), 0x5002);
        assert_eq!(e.mem.u32(this.addr() + 0xac), 0);
        assert_eq!(
            calls(&log, 0x0083_c8a0),
            vec![vec![built[0][0], 0x5002], vec![0x5002, 0]]
        );
    }

    #[test]
    fn new_load_restores_the_conversation() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0xd0));
        let buffer = object(&mut e, 0x0a10_0000, &[(0, word(3))]);
        quiet(&mut e, &[0x0067_98c0, LOAD_FORM_ID_INTO, 0x0083_c330]);
        e.register(LOAD_FORM_ID, |_, _| word(0x21));
        e.register(FORM_FOR_ID, |_, a| (a[0] + 1).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| (a[0] + 1).into_ret());
        e.register(LOAD_DATA, |e, a| {
            match a[2] {
                1 => e.mem.set_u8(a[1], 1),
                2 => e.mem.set_u16(a[1], 4),
                _ => {}
            }
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONVERSATION_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(ITEM_BY_INDEX, |_, a| (0x5000 + a[1]).into_ret());
        e.register(RESPONSE_BY_INDEX, |_, a| (0x6000 + a[1]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009f_0460, &args![this, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this.addr() + 0x8c), 0x23);
        assert_eq!(e.mem.u8(this.addr() + 0xc2), 1);
        let loaded: Vec<_> = calls(&log, LOAD_FORM_ID_INTO)
            .iter()
            .map(|w| w[1] - this.addr())
            .collect();
        assert_eq!(loaded, [0x94, 0xb0, 0xb4, 0xb8, 0x9c]);
        assert_eq!(e.mem.u32(this.addr() + 0xa8), 0x5004);
        assert_eq!(e.mem.u32(this.addr() + 0xac), 0x6004);
        let conversation = e.mem.u32(this.addr() + 0xa4);
        assert_eq!(calls(&log, 0x0083_c330), vec![vec![conversation, buffer]]);
    }

    const CONVERSATION_OBJECT: u32 = 0x0c00_0000;
    const ITEM: u32 = 0x0c10_0000;
    const PROCESS_VTABLE: u32 = 0x0a20_0000;

    struct Talk {
        lip_sync: u32,
        speaker_is_actor: bool,
        talking_done: u32,
        busy: u32,
    }

    /// A package whose conversation has one item with one response; the
    /// actor (`+0xB0`) and target (`+0xB4`) with a shared process.
    fn talk(world: Talk) -> (Engine, Ptr, u32, u32, u32, Logged) {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x10000);
        e.mem.set_f32(LISTEN_LONG, 50.0);
        e.mem.set_f32(LISTEN_SHORT, 2.0);
        e.mem.set_f32(LISTEN_DEFAULT, 10.0);
        e.set_global(0x0101_2060, 0.0f64);
        e.map(0x011d_0000, 0x10000);
        e.map(0x011e_0000, 0x1000);
        e.map(0x011f_1000, 0x1000);
        e.mem.set_u8(LIP_SYNC_SETTING + 4, world.lip_sync as u8);
        e.mem.set_u8(ALL_SUBTITLES_SETTING + 4, 1);
        e.register(0x0040_8d60, |_, a| (a[0] + 4).into_ret());
        let player = e.mem.alloc(0x300);
        e.set_global(PLAYER, player);

        let busy = world.busy;
        let process = object(
            &mut e,
            PROCESS_VTABLE,
            &[
                (0x11c, word(busy)),
                (0x644, word(0)),
                (0x634, word(0)),
                (0x37c, word(0)),
                (0x36c, word(0)),
                (0x378, word(0)),
                (0x380, word(0)),
                (0x374, word(0)),
                (0x44, word(0)),
            ],
        );
        e.register_double(GET_PROCESS, move |_, _| word(process));
        let position = e.mem.alloc(0xc);
        e.mem.set_f32(position, 1.0);
        let actor_slots = [
            (0x100, word(u32::from(world.speaker_is_actor))),
            (0x200, word(0)),
            (0x2e8, word(0)),
            (0x22c, word(0)),
            (0x1e4, word(0)),
            (0x284, st0(3.0)),
            (0x1f4, word(position)),
            (0x1d0, word(0x0d00_0000)),
        ];
        let actor = object(&mut e, 0x0a30_0000, &actor_slots);
        e.mem.set_u8(actor + 0x80, 1);
        let target = object(&mut e, 0x0a50_0000, &actor_slots);
        let this: Ptr = Ptr::new(e.mem.alloc(0xd0));
        e.mem.set_u32(player + 0x208, this.addr());
        e.mem.set_u32(this.addr() + 0xb0, actor);
        e.mem.set_u32(this.addr() + 0xb4, target);
        e.mem.set_u32(this.addr() + 0xa4, CONVERSATION_OBJECT);
        e.mem.set_u8(this.addr() + 0xbc, 1);
        e.mem.set_u8(this.addr() + 0xcc, 1);
        if world.talking_done != 2 {
            let talking = e.mem.alloc(0x100);
            e.mem.set_u8(talking + 0x80, world.talking_done as u8);
            e.mem.set_u32(this.addr() + 0x94, talking);
        }
        let response = e.mem.alloc(0x10);
        e.mem.set_u32(response, 0x0e00_0000);
        let current = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let started = current.clone();
        e.register_double(CONVERSATION_FIRST_ITEM, move |_, _| {
            started.set(1);
            Ret::default()
        });
        let state = current;
        e.register_double(GET_CURRENT, move |_, a| {
            let value = match a[0] {
                CONVERSATION_OBJECT if state.get() != 0 => ITEM,
                ITEM => response,
                _ => 0,
            };
            word(value)
        });
        e.register_double(ITEM_SPEAKER, move |_, _| word(actor));
        quiet(
            &mut e,
            &[
                ITEM_FIRST_RESPONSE,
                STOP_CURRENT_DIALOGUE,
                STRING_CONSTRUCT,
                STRING_SET,
                STRING_DESTRUCT,
                STRING_FROM,
                ITEM_RUN_RESULT,
                0x0070_5b10,
                0x008a_5cf0,
                SET_COMPLETION_CALLBACK,
                0x0040_6d30,
                0x0041_8900,
                HANDLE_RELEASE,
                0x00ad_8be0,
                0x00ad_8b60,
                0x00ad_8f20,
                0x00ad_8870,
                0x0070_5210,
            ],
        );
        e.register(FORM_OF, |_, _| word(0));
        e.register(0x0044_1110, |_, _| word(0x41));
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0040_48e0, |_, _| word(0x43));
        e.register(0x0084_e3a0, |_, a| (a[0] >> 20).into_ret());
        e.register(0x0044_ddc0, |_, _| word(0x45));
        e.register(WORD_AT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0054_3c30, |_, _| word(1));
        e.register(AUDIO_MANAGER, |_, _| word(0x0f00_0000));
        e.register(RESPONSE_TEXT, |_, a| a[0].into_ret());
        e.register(0x0045_3a70, |_, _| word(0x0f10_0000));
        e.register(0x00ad_7480, |_, a| a[1].into_ret());
        e.register(0x0057_90d0, |_, _| st0(100.0));
        e.register(0x0057_90b0, |_, _| st0(5.0));
        e.register(0x008c_1bc0, |_, _| word(1));
        e.register(DO_NOT_CONTROL_TARGET, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x009e_e0a0, &args![this, 0x0101u32]);
        let log = e.call_log.take().unwrap();
        (e, this, actor, target, process, log)
    }

    #[test]
    fn speaking_with_lip_sync() {
        let (e, this, actor, target, _, log) = talk(Talk {
            lip_sync: 1,
            speaker_is_actor: true,
            talking_done: 2,
            busy: 0,
        });
        // The first item and response, the target turned toward the actor.
        assert_eq!(calls(&log, PROCESS_VTABLE + 0x10_0000 + 0x634)[0][1], actor);
        assert_eq!(e.mem.u32(this.addr() + 0xa8), ITEM);
        let said = calls(&log, 0x0a30_0000 + 0x10_0000 + 0x284);
        assert_eq!(said.len(), 1);
        assert_eq!(said[0][2], this.addr() + 0x80);
        let response = e.mem.u32(this.addr() + 0xac);
        assert_eq!(said[0][3..8], [0x45, response >> 20, 0x43, actor, 0x41]);
        assert_eq!(said[0][8..], [target, 1, 0, 1, 1, 0]);
        assert_eq!(calls(&log, ITEM_RUN_RESULT), vec![vec![ITEM, 0]]);
        assert_eq!(e.mem.u32(this.addr() + 0xc4), 0x45);
        let callback = calls(&log, SET_COMPLETION_CALLBACK);
        assert_eq!(callback[0][2..], [DIALOGUE_DONE_CALLBACK, actor >> 20]);
        assert_eq!(e.mem.f32(this.addr() + 0x90), 50.0);
        assert_eq!(e.mem.u8(actor + 0x80), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x94), actor);
        // No look toward each other after a line.
        assert!(calls(&log, PROCESS_VTABLE + 0x10_0000 + 0x44).is_empty());
    }

    #[test]
    fn speaking_without_lip_sync() {
        let (e, this, _, target, _, log) = talk(Talk {
            lip_sync: 0,
            speaker_is_actor: true,
            talking_done: 1,
            busy: 0,
        });
        let said = calls(&log, 0x0a30_0000 + 0x10_0000 + 0x284);
        assert_eq!(said[0][8..], [target, 1, 0, 0, 1, 0]);
        assert_eq!(calls(&log, ITEM_RUN_RESULT).len(), 1);
        assert_eq!(calls(&log, PROCESS_VTABLE + 0x10_0000 + 0x380)[0][1], 0);
        assert_eq!(calls(&log, PROCESS_VTABLE + 0x10_0000 + 0x374)[0][1], 0);
        assert_eq!(e.mem.f32(this.addr() + 0x90), 2.0);
        // The previous talking actor is stopped first.
        assert_eq!(calls(&log, STOP_CURRENT_DIALOGUE).len(), 1);
    }

    #[test]
    fn another_speaker_plays_the_text() {
        let (e, this, _, target, _, log) = talk(Talk {
            lip_sync: 0,
            speaker_is_actor: false,
            talking_done: 2,
            busy: 0,
        });
        assert!(calls(&log, 0x0a30_0000 + 0x10_0000 + 0x284).is_empty());
        let played = calls(&log, 0x00ad_7480);
        assert_eq!(played[0][3..], [0x106, 0]);
        assert_eq!(
            calls(&log, 0x00ad_8be0)[0][1..],
            [5.0f32.to_bits(), 100.0f32.to_bits()]
        );
        assert_eq!(calls(&log, 0x00ad_8b60)[0][1], 1.0f32.to_bits());
        assert_eq!(calls(&log, 0x00ad_8f20)[0][1], 0x0d00_0000);
        assert_eq!(calls(&log, 0x00ad_8870)[0][1..], [1, 0]);
        let subtitle = calls(&log, 0x0070_5210);
        assert_eq!(subtitle.len(), 1);
        assert_eq!(subtitle[0][0], 0x0e00_0000);
        assert_eq!(subtitle[0][4], 1.0f32.to_bits());
        assert_eq!(subtitle[0][7..], [target, 0]);
        assert_eq!(e.mem.f32(this.addr() + 0x90), 10.0);
    }

    #[test]
    fn nothing_to_say() {
        // The talking actor is still talking.
        let (_, _, _, _, _, log) = talk(Talk {
            lip_sync: 1,
            speaker_is_actor: true,
            talking_done: 0,
            busy: 0,
        });
        assert_eq!(log.len(), 1);
        // The actor's process is busy.
        let (_, _, _, _, _, log) = talk(Talk {
            lip_sync: 1,
            speaker_is_actor: true,
            talking_done: 2,
            busy: 1,
        });
        assert!(calls(&log, CONVERSATION_FIRST_ITEM).is_empty());
    }
}
