//! `fallout/misc/saveload/bgssaveloadglobaldata.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0084d030 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A folded float getter: `FLD dword [this+0xc]`, the value in `ST0`.
/// `Script::Run` (`005ac1e0`) calls it on the timer object at `011f6394`
/// for the frame's elapsed seconds. The `FLD` quiets a signaling NaN.
pub fn fn_0084d030(e: &mut Engine, this: Ptr) -> f64 {
    e.mem.f32(this.addr() + 0xc) as f64
}

/// `SaveGameWarning` (`00845cc0`, engine map; `cdecl`, format and
/// arguments).
const SAVE_GAME_WARNING: u32 = 0x0084_5cc0;
/// "SAVELOAD: Unknown global data type %i in SaveGlobalData()".
const UNKNOWN_TYPE_WARNING: u32 = 0x0107_f5a4;
/// The `TES` singleton pointer.
const TES: u32 = 0x011d_ea10;
/// The player singleton pointer (`PlayerCharacter*`, as the code passes it
/// to `TESObjectREFR::GetWorldSpace`).
const PLAYER: u32 = 0x011d_ea3c;
/// The `TESDataHandler` singleton pointer.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The `ProcessLists` object (passed by address).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The `CombatManager` singleton pointer.
const COMBAT_MANAGER: u32 = 0x011f_1958;
/// The data-type handlers of [`bgs_save_load_global_data_save_global_data`].
const MISC_STATS_SAVE_004D5FA0: u32 = 0x004d_5fa0;
const TES_SAVE_GAME: u32 = 0x0045_9490;
const PROCESS_LISTS_SAVE_GAME: u32 = 0x0097_5840;
const COMBAT_MANAGER_SAVE_GAME: u32 = 0x0099_32d0;
const INTERFACE_SAVE_007066D0: u32 = 0x0070_66d0;
const SKY_GET_INSTANCE: u32 = 0x0046_dd00;
const SKY_SAVE_GAME: u32 = 0x0063_eb70;
const ACTOR_CAUSE_SAVE_LIST: u32 = 0x0066_e6d0;
const FALLOUT_RADIO_SAVE_GAME: u32 = 0x0083_66e0;
const FALLOUT_AUDIO_SAVE_GAME: u32 = 0x0082_ec40;
const PROCESS_LISTS_SAVE_004534F0: u32 = 0x0045_34f0;
/// `00865e50` (`thiscall` on the save buffer: data, size, a word passed as
/// 0): writes raw bytes.
const BUFFER_SAVE_DATA: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID` (`00865df0`, engine map `_ov2`;
/// `thiscall`: form, a word passed as 0).
const BUFFER_SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `BGSSaveGameBuffer::StartVariableSizedValue` (`00865f20`, engine map;
/// `thiscall`): the position to fill in later.
const BUFFER_START_VARIABLE_SIZED: u32 = 0x0086_5f20;
/// `BGSSaveGameBuffer::SaveVariableSizedValue` (`00865ff0`, engine map
/// `_ov2`; `thiscall`: value, position).
const BUFFER_SAVE_VARIABLE_SIZED: u32 = 0x0086_5ff0;
/// `TES::GetWorldSpace` (`004fd3e0`, engine map; `thiscall`).
const TES_GET_WORLD_SPACE: u32 = 0x004f_d3e0;
/// `0059bb30` (`thiscall`): the word at +0x24 (engine map name
/// `D3DTexture_LockRect`, a folded body).
const WORD_24_0059BB30: u32 = 0x0059_bb30;
/// `0045cd60` (`thiscall`): the word at +0x28.
const WORD_28_0045CD60: u32 = 0x0045_cd60;
/// `TESObjectREFR::GetWorldSpace` (`00575d70`, engine map; `thiscall`).
const REFR_GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// `008d6f30` (`thiscall`, no Xbox PDB name): the player's other form
/// saved when it has no worldspace.
const PLAYER_FORM_008D6F30: u32 = 0x008d_6f30;
/// `00436aa0` (`thiscall`): the address of three words to save.
const POSITION_00436AA0: u32 = 0x0043_6aa0;
/// `0078d6b0` (`cdecl`, the buffer; engine map unit `loadingmenu.cpp`).
const LOADING_MENU_SAVE_0078D6B0: u32 = 0x0078_d6b0;
/// `00461190` (`thiscall`): the data handler's global list (first node).
const DATA_HANDLER_GLOBALS_00461190: u32 = 0x0046_1190;
/// `006815c0` (`thiscall`): returns `this`; here a node's item word.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`thiscall`): the word at +4; here the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `00549580` (`thiscall`, a `bool` in AL): globals for which it answers
/// true are skipped.
const GLOBAL_SKIP_00549580: u32 = 0x0054_9580;
/// `TESGlobal`'s value getter (`00526ac0`; result in ST0).
const GLOBAL_GET_VALUE: u32 = 0x0052_6ac0;
/// `0043b5d0` (`thiscall` on `TES`): a list (first node) of entries whose
/// objects [`fn_0084cf30`] saves.
const TES_LIST_0043B5D0: u32 = 0x0043_b5d0;
/// `00559450` (`thiscall`): the object a smart-pointer member holds.
const POINTER_GET_00559450: u32 = 0x0055_9450;
/// `00529990` (`cdecl`, no words): `FLD dword [011c96fc]`.
const GLOBAL_VALUE_00529990: u32 = 0x0052_9990;
/// `0047c860` (`thiscall`, result in ST0; engine map name
/// `TESActorBaseData::GetKarma`, a folded body).
const FLOAT_GETTER_0047C860: u32 = 0x0047_c860;
/// `00441110` (`thiscall`): a word saved as a form id.
const WORD_GETTER_00441110: u32 = 0x0044_1110;

// Translated from 0084bcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGlobalData::SaveGlobalData` (Xbox PDB, matched by the engine
/// map; `cdecl (type, buffer)`): saves one global data type into the save
/// buffer and answers true; an unknown type is reported (`SaveGameWarning`)
/// and answers false. The types (the exe's jump table at `0084be10` for
/// 0..=11, and 1000): 0 `004d5fa0`, 1 [`bgs_save_load_global_data_save_location_data`],
/// 2 `TES::SaveGame`, 3 [`bgs_save_load_global_data_save_globals`], 4
/// `ProcessLists::SaveGame`, 5 `CombatManager::SaveGame`, 6 `007066d0`, 7
/// [`fn_0084cf30`], 8 `Sky::SaveGame` on `Sky::GetInstance()`, 9
/// `ActorCause::SaveActorCausesList`, 10 `FalloutRadio::SaveGame`, 11
/// `FalloutAudio::SaveGame`, 1000 `004534f0` on the process lists.
pub fn bgs_save_load_global_data_save_global_data(e: &mut Engine, kind: u32, buffer: u32) -> bool {
    match kind {
        0 => {
            e.call(MISC_STATS_SAVE_004D5FA0, &args![buffer]);
        }
        1 => bgs_save_load_global_data_save_location_data(e, buffer),
        2 => {
            let tes: u32 = e.global(TES);
            e.call(TES_SAVE_GAME, &args![tes, buffer]);
        }
        3 => bgs_save_load_global_data_save_globals(e, buffer),
        4 => {
            e.call(PROCESS_LISTS_SAVE_GAME, &args![PROCESS_LISTS, buffer]);
        }
        5 => {
            let manager: u32 = e.global(COMBAT_MANAGER);
            e.call(COMBAT_MANAGER_SAVE_GAME, &args![manager, buffer]);
        }
        6 => {
            e.call(INTERFACE_SAVE_007066D0, &args![buffer]);
        }
        7 => fn_0084cf30(e, buffer),
        8 => {
            let sky = e.call(SKY_GET_INSTANCE, &args![]).u32();
            e.call(SKY_SAVE_GAME, &args![sky, buffer]);
        }
        9 => {
            e.call(ACTOR_CAUSE_SAVE_LIST, &args![buffer]);
        }
        10 => {
            e.call(FALLOUT_RADIO_SAVE_GAME, &args![buffer]);
        }
        11 => {
            e.call(FALLOUT_AUDIO_SAVE_GAME, &args![buffer]);
        }
        1000 => {
            e.call(PROCESS_LISTS_SAVE_004534F0, &args![PROCESS_LISTS, buffer]);
        }
        _ => {
            e.call(SAVE_GAME_WARNING, &args![UNKNOWN_TYPE_WARNING, kind]);
            return false;
        }
    }
    true
}

// Translated from 0084c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGlobalData::SaveLocationData` (Xbox PDB, matched by the
/// engine map; `cdecl (buffer)`): saves the data handler's word at +0x208
/// ([`fn_0084c580`]), the current worldspace (`TES::GetWorldSpace`) as a
/// form id, the `TES` words at +0x24 and +0x28, the player's worldspace (or,
/// without one, `008d6f30`'s form) as a form id and the three words at
/// `00436aa0`'s address, then runs `0078d6b0(buffer)`.
pub fn bgs_save_load_global_data_save_location_data(e: &mut Engine, buffer: u32) {
    // The exe's locals from EBP-0x1C: +4, +8 and +0xC are single words,
    // +0x10 three words.
    e.with_stack(0x1c, |e, locals| {
        let base = locals.addr();
        let handler: u32 = e.global(DATA_HANDLER);
        let word = fn_0084c580(e, Ptr::new(handler));
        e.mem.set_u32(base + 8, word);
        e.call(BUFFER_SAVE_DATA, &args![buffer, base + 8, 4u32, 0u32]);
        let tes: u32 = e.global(TES);
        let world = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
        e.call(BUFFER_SAVE_FORM_ID, &args![buffer, world, 0u32]);
        let tes: u32 = e.global(TES);
        let first = e.call(WORD_24_0059BB30, &args![tes]).u32();
        e.mem.set_u32(base + 0xc, first);
        let tes: u32 = e.global(TES);
        let second = e.call(WORD_28_0045CD60, &args![tes]).u32();
        e.mem.set_u32(base + 4, second);
        e.call(BUFFER_SAVE_DATA, &args![buffer, base + 0xc, 4u32, 0u32]);
        e.call(BUFFER_SAVE_DATA, &args![buffer, base + 4, 4u32, 0u32]);
        let player: u32 = e.global(PLAYER);
        let mut world = e.call(REFR_GET_WORLD_SPACE, &args![player]).u32();
        if world == 0 {
            let player: u32 = e.global(PLAYER);
            world = e.call(PLAYER_FORM_008D6F30, &args![player]).u32();
        }
        e.call(BUFFER_SAVE_FORM_ID, &args![buffer, world, 0u32]);
        let player: u32 = e.global(PLAYER);
        let source = e.call(POSITION_00436AA0, &args![player]).u32();
        for i in 0..3 {
            let value = e.mem.u32(source + 4 * i);
            e.mem.set_u32(base + 0x10 + 4 * i, value);
        }
        e.call(BUFFER_SAVE_DATA, &args![buffer, base + 0x10, 0xcu32, 0u32]);
        e.call(LOADING_MENU_SAVE_0078D6B0, &args![buffer]);
    });
}

// Translated from 0084c580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x208 (`thiscall`; no Xbox PDB name; called on the data
/// handler).
pub fn fn_0084c580(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x208)
}

// Translated from 0084cc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGlobalData::SaveGlobals` (Xbox PDB, matched by the engine
/// map; `cdecl (buffer)`): a variable-sized count followed by, for each
/// global of the data handler's list that `00549580` does not skip, its form
/// id and its value as a `float`.
pub fn bgs_save_load_global_data_save_globals(e: &mut Engine, buffer: u32) {
    let handler: u32 = e.global(DATA_HANDLER);
    let mut node = e.call(DATA_HANDLER_GLOBALS_00461190, &args![handler]).u32();
    let mut count = 0u32;
    let start = e.call(BUFFER_START_VARIABLE_SIZED, &args![buffer]).u32();
    e.with_stack(4, |e, value| {
        while node != 0 {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let global = e.mem.u32(slot);
            if global != 0 && !e.call(GLOBAL_SKIP_00549580, &args![global]).bool() {
                e.call(BUFFER_SAVE_FORM_ID, &args![buffer, global, 0u32]);
                let v = e.call(GLOBAL_GET_VALUE, &args![global]).f64();
                e.mem.set_u32(value.addr(), x87_store32(v));
                e.call(BUFFER_SAVE_DATA, &args![buffer, value, 4u32, 0u32]);
                count = count.wrapping_add(1);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
    e.call(BUFFER_SAVE_VARIABLE_SIZED, &args![buffer, count, start]);
}

// Translated from 0084cf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves global data type 7 (`cdecl (buffer)`; no Xbox PDB name): a
/// variable-sized count followed by, for each entry of `TES`'s `0043b5d0`
/// list whose object (`00559450` on the entry) exists, lacks bit 0 of +0x18
/// ([`fn_0084d050`]) and has a vtable +0x94 object, that object's `float`
/// at +0xC ([`fn_0084d030`]), the `float` at `011c96fc` less `0047c860` of
/// it, and `00441110` of it as a form id.
pub fn fn_0084cf30(e: &mut Engine, buffer: u32) {
    let mut count = 0u32;
    let start = e.call(BUFFER_START_VARIABLE_SIZED, &args![buffer]).u32();
    let tes: u32 = e.global(TES);
    let mut node = e.call(TES_LIST_0043B5D0, &args![tes]).u32();
    // The exe's locals at EBP-0x1C (+0) and EBP-0x18 (+4).
    e.with_stack(8, |e, locals| {
        let base = locals.addr();
        while node != 0 {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let object = e.call(POINTER_GET_00559450, &args![slot]).u32();
            if object != 0 && !fn_0084d050(e, Ptr::new(object)) {
                let inner = e.vcall(object, 0x94, &args![]).u32();
                if inner != 0 {
                    let first = fn_0084d030(e, Ptr::new(inner));
                    e.mem.set_u32(base + 4, x87_store32(first));
                    e.call(BUFFER_SAVE_DATA, &args![buffer, base + 4, 4u32, 0u32]);
                    let now = e.call(GLOBAL_VALUE_00529990, &args![]).f64();
                    let then = e.call(FLOAT_GETTER_0047C860, &args![inner]).f64();
                    e.mem.set_u32(base, x87_store32(x87_sub(now, then)));
                    e.call(BUFFER_SAVE_DATA, &args![buffer, base, 4u32, 0u32]);
                    let form = e.call(WORD_GETTER_00441110, &args![inner]).u32();
                    e.call(BUFFER_SAVE_FORM_ID, &args![buffer, form, 0u32]);
                    count = count.wrapping_add(1);
                }
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
    e.call(BUFFER_SAVE_VARIABLE_SIZED, &args![buffer, count, start]);
}

// Translated from 0084d050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the word at +0x18 is set (`thiscall`; no Xbox PDB
/// name).
pub fn fn_0084d050(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x18) & 1 != 0
}

// Translated from 0084d310 (decompiled, FalloutNV.exe 1.4.0.525)
/// A hash map's set (`thiscall (this; key, value byte)`; no Xbox PDB name;
/// the map's vtable gives +0x04 the bucket of a key, +0x08 whether two keys
/// are equal, +0x0C node set-up, +0x14 a new node; buckets at +8, count at
/// +0xC; nodes hold the next node, the key at +4 and the byte at +8): an
/// existing key's byte is replaced, else a new node is put at the head of
/// the key's bucket and the count grows.
pub fn fn_0084d310(e: &mut Engine, this: Ptr, key: u32, value: u8) {
    let map = this.addr();
    let bucket = e.vcall(map, 4, &args![key]).u32();
    let buckets = e.mem.u32(map + 8);
    let mut node = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
    while node != 0 {
        let node_key = e.mem.u32(node + 4);
        if e.vcall(map, 8, &args![key, node_key]).bool() {
            e.mem.set_u8(node + 8, value);
            return;
        }
        node = e.mem.u32(node);
    }
    let node = e.vcall(map, 0x14, &args![]).u32();
    e.vcall(map, 0xc, &args![node, key, u32::from(value)]);
    let slot = e.mem.u32(map + 8).wrapping_add(bucket.wrapping_mul(4));
    let head = e.mem.u32(slot);
    e.mem.set_u32(node, head);
    let slot = e.mem.u32(map + 8).wrapping_add(bucket.wrapping_mul(4));
    e.mem.set_u32(slot, node);
    let count = e.mem.u32(map + 0xc);
    e.mem.set_u32(map + 0xc, count.wrapping_add(1));
}

/// `0084c2a0` (this unit; `cdecl`: first type, end type, flag): reverts the global data
/// types from `first` below `end` (`RevertGlobalDataRange` with a stack load buffer).
const REVERT_GLOBAL_DATA_TYPES: u32 = 0x0084_c2a0;

// Translated from 0084c270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): reverts the global data types 0 to 0xB
/// and 0x3E8 (`0084c2a0(0, 0xC, flag)`, then `0084c2a0(1000, 1001, flag)`), the flag
/// passed zero-extended from its low byte.
pub fn fn_0084c270(e: &mut Engine, flag: u8) {
    e.call(
        REVERT_GLOBAL_DATA_TYPES,
        &args![0u32, 0xcu32, u32::from(flag)],
    );
    e.call(
        REVERT_GLOBAL_DATA_TYPES,
        &args![0x3e8u32, 0x3e9u32, u32::from(flag)],
    );
}

// --- Loading, initialising, finishing and reverting the global data types ---

/// The load handlers of [`bgs_save_load_global_data_load_global_data`] (the exe's jump table
/// at `0084bf80` for 0..=11, and 1000), all with the buffer: `MiscStatManager::LoadGame`
/// (`004d6020`, `cdecl`), `TES::LoadGame` (`004595e0`, on `TES`), `ProcessLists::LoadGame`
/// (`00975930`, on the process lists), `CombatManager::LoadGame` (`00993440`, on the manager),
/// `Interface::LoadGame` (`00706860`, `cdecl`), `Sky::LoadGame` (`0063ecb0`, on
/// `Sky::GetInstance()`), `ActorCause::LoadActorCausesList` (`0066e750`),
/// `FalloutRadio::LoadGame` (`008368b0`), `FalloutAudio::LoadGame` (`0082ec60`) (`cdecl`).
const MISC_STATS_LOAD_GAME: u32 = 0x004d_6020;
const TES_LOAD_GAME: u32 = 0x0045_95e0;
const PROCESS_LISTS_LOAD_GAME: u32 = 0x0097_5930;
const COMBAT_MANAGER_LOAD_GAME: u32 = 0x0099_3440;
const INTERFACE_LOAD_GAME: u32 = 0x0070_6860;
const SKY_LOAD_GAME: u32 = 0x0063_ecb0;
const ACTOR_CAUSE_LOAD_LIST: u32 = 0x0066_e750;
const FALLOUT_RADIO_LOAD_GAME: u32 = 0x0083_68b0;
const FALLOUT_AUDIO_LOAD_GAME: u32 = 0x0082_ec60;
/// `"SAVELOAD: Unknown global data type %i in LoadGlobalData()"`, `"DEFAULT: Unknown global
/// data type %i in InitGlobalData()"`, and the finish and revert warnings (`0107f658`,
/// `0107f698`).
const UNKNOWN_LOAD_TYPE: u32 = 0x0107_f5e0;
const UNKNOWN_INIT_TYPE: u32 = 0x0107_f61c;
const UNKNOWN_FINISH_TYPE: u32 = 0x0107_f658;
const UNKNOWN_REVERT_TYPE: u32 = 0x0107_f698;
/// The init handlers (jump table `0084c0e0`): `00975af0` (on the process lists),
/// `CombatManager::InitLoadGame` (`00993600`), `ActorCause::InitLoadActorCausesList`
/// (`0066e810`, `cdecl`), `FalloutRadio::InitLoadGame` (`00836af0`, `cdecl`).
const PROCESS_LISTS_INIT_00975AF0: u32 = 0x0097_5af0;
const COMBAT_MANAGER_INIT_LOAD_GAME: u32 = 0x0099_3600;
const ACTOR_CAUSE_INIT_LOAD_LIST: u32 = 0x0066_e810;
const FALLOUT_RADIO_INIT_LOAD_GAME: u32 = 0x0083_6af0;
/// The finish handlers (jump table `0084c240`): `00459750` (on `TES`), `00993790` (on the
/// combat manager), `Interface::FinishLoadGame` (`00706a90`, `cdecl`),
/// `FalloutRadio::FinishLoadGame` (`00836e20`, `cdecl`).
const TES_FINISH_00459750: u32 = 0x0045_9750;
const COMBAT_MANAGER_FINISH_00993790: u32 = 0x0099_3790;
const INTERFACE_FINISH_LOAD_GAME: u32 = 0x0070_6a90;
const FALLOUT_RADIO_FINISH_LOAD_GAME: u32 = 0x0083_6e20;
/// The revert handlers (jump table `0084c454`): `004d60c0` (`cdecl`), `00459840` (on `TES`),
/// `00975b60` (on the process lists), `00993800` (on the combat manager), `00706ae0`
/// (`cdecl`), `0066e910` (`cdecl`), `FalloutRadio::Revert` (`00836ef0`, `cdecl`), `0040fbe0`
/// (`cdecl`; engine map name `Error`, a folded body) and, for 1000, `00975c50` (on the
/// process lists).
const MISC_STATS_REVERT_004D60C0: u32 = 0x004d_60c0;
const TES_REVERT_00459840: u32 = 0x0045_9840;
const PROCESS_LISTS_REVERT_00975B60: u32 = 0x0097_5b60;
const COMBAT_MANAGER_REVERT_00993800: u32 = 0x0099_3800;
const INTERFACE_REVERT_00706AE0: u32 = 0x0070_6ae0;
const ACTOR_CAUSE_REVERT_0066E910: u32 = 0x0066_e910;
const FALLOUT_RADIO_REVERT: u32 = 0x0083_6ef0;
const REVERT_0040FBE0: u32 = 0x0040_fbe0;
const PROCESS_LISTS_REVERT_00975C50: u32 = 0x0097_5c50;
/// `BGSLoadGameBuffer::BGSLoadGameBuffer` (engine map, `008646b0`) and its destructor
/// (`008646f0`) on a 0x10-byte local.
const LOAD_BUFFER_CONSTRUCT: u32 = 0x0086_46b0;
const LOAD_BUFFER_DESTRUCT: u32 = 0x0086_46f0;
/// The load buffer's reads (`thiscall` on the buffer): `BGSLoadGameBuffer::LoadFormID`
/// (`008648a0`), `00864980` (destination, byte count), `LoadVariableSizedValue` (`00864a60`),
/// and the buffer's vtable slot 0 (the save's version byte).
const BUFFER_LOAD_FORM_ID: u32 = 0x0086_48a0;
const BUFFER_LOAD_DATA: u32 = 0x0086_4980;
const BUFFER_LOAD_VARIABLE_SIZED: u32 = 0x0086_4a60;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, offset, source type, target type, by
/// reference) and the type descriptors passed to it: the source (`01183028`) and the targets
/// of the globals (`01183ad4`), the image-space modifiers (`011862b8`), the world spaces
/// (`01183fd0`) and the references (`011841cc`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_GLOBAL_TYPE: u32 = 0x0118_3ad4;
const CAST_IMAGE_SPACE_MODIFIER_TYPE: u32 = 0x0118_62b8;
const CAST_WORLD_SPACE_TYPE: u32 = 0x0118_3fd0;
const CAST_REFERENCE_TYPE: u32 = 0x0118_41cc;
/// `004839c0` (`cdecl`, a form id): the loaded form with that id, or 0.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// The `BGSSaveLoadGame` singleton pointer and the object `0084ced0` and `LoadGlobals` add
/// forms to (`007af430` on it; the engine map's `BGSSaveFormBuffer::GetForm` is a folded
/// name), `BGSReconstructFormsInAllFilesMap::AddForm` (`00843630`; `thiscall`: form, word).
const SAVE_LOAD_GAME: u32 = 0x011d_df38;
const RECONSTRUCT_MAP_007AF430: u32 = 0x007a_f430;
const RECONSTRUCT_ADD_FORM: u32 = 0x0084_3630;
/// The global map's vtable-driven removal (`00405430`, `thiscall`: key; engine map name
/// `NiTMapBase<..CombatThreat *>::RemoveAt`, the same body), its iteration (`004b9ba0` first
/// position, `0096a400` next: position, key and value addresses) and `00438af0` (emptying).
const MAP_REMOVE_AT: u32 = 0x0040_5430;
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
const GLOBAL_MAP_NEXT: u32 = 0x0096_a400;
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `0046dce0` (`thiscall` on a global, a `float`): sets its value.
const GLOBAL_SET_VALUE: u32 = 0x0046_dce0;
/// The `NiTMapBase<.., TESGlobal *, bool>` and `NiTMap<TESGlobal *, bool>` vtables (RTTI).
const GLOBAL_MAP_BASE_VTABLE: u32 = 0x0107_f734;
const GLOBAL_MAP_VTABLE: u32 = 0x0107_f714;
/// The allocator (`00aa1070`, `cdecl`, byte count), `memset` (`00403d30`) and the free
/// (`00aa10f0`, `cdecl`).
const ALLOCATE: u32 = 0x00aa_1070;
const MEMSET: u32 = 0x0040_3d30;
const FREE: u32 = 0x00aa_10f0;
/// `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `ImageSpaceModifierInstanceForm::Trigger` (engine map, `005299a0`; `cdecl`: modifier, a
/// `float`, 0), `006694c0` (`thiscall` on the instance, a `float`) and the float it is given
/// less (`011c3c08`).
const IMAGE_SPACE_TRIGGER: u32 = 0x0052_99a0;
const INSTANCE_SET_006694C0: u32 = 0x0066_94c0;
const EFFECT_TIME: u32 = 0x011c_3c08;
/// `RevertEffects`' callees: `004e0870`, `004defb0` (`cdecl`), `00c50930` on `0045a190`'s
/// object, the `BSSimpleList` node removal `BSSimpleList<..>::Remove` (`00631620`: the
/// smart pointer's address), the smart pointer's constructor (`00633c90`, the object) and
/// destructor (`0045cec0`), `0063f790` (a `float`).
const REVERT_004E0870: u32 = 0x004e_0870;
const REVERT_004DEFB0: u32 = 0x004d_efb0;
const OBJECT_0045A190: u32 = 0x0045_a190;
const REVERT_00C50930: u32 = 0x00c5_0930;
const LIST_REMOVE: u32 = 0x0063_1620;
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
const EFFECT_SET_0063F790: u32 = 0x0063_f790;
/// `vprintf`-like output (`00ecbe60`, `cdecl`: format, argument list).
const VPRINTF_00ECBE60: u32 = 0x00ec_be60;

// Translated from 0084be40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGlobalData::LoadGlobalData` (Xbox PDB; `cdecl (type, buffer)`): loads one
/// global data type and answers true; an unknown type is reported (`SaveGameWarning`) and
/// answers false. The types (jump table `0084bf80` for 0..=11, and 1000): 0
/// `MiscStatManager::LoadGame`, 1 [`fn_0084c5a0`], 2 `TES::LoadGame`, 3
/// [`bgs_save_load_global_data_load_globals`], 4 `ProcessLists::LoadGame`, 5
/// `CombatManager::LoadGame`, 6 `Interface::LoadGame`, 7
/// [`bgs_save_load_global_data_load_effects`], 8 `Sky::LoadGame` on `Sky::GetInstance()`, 9
/// `ActorCause::LoadActorCausesList`, 10 `FalloutRadio::LoadGame`, 11
/// `FalloutAudio::LoadGame`, 1000 `004534f0` on the process lists.
pub fn bgs_save_load_global_data_load_global_data(e: &mut Engine, kind: u32, buffer: u32) -> bool {
    match kind {
        0 => {
            e.call(MISC_STATS_LOAD_GAME, &args![buffer]);
        }
        1 => fn_0084c5a0(e, buffer),
        2 => {
            let tes: u32 = e.global(TES);
            e.call(TES_LOAD_GAME, &args![tes, buffer]);
        }
        3 => bgs_save_load_global_data_load_globals(e, buffer),
        4 => {
            e.call(PROCESS_LISTS_LOAD_GAME, &args![PROCESS_LISTS, buffer]);
        }
        5 => {
            let manager: u32 = e.global(COMBAT_MANAGER);
            e.call(COMBAT_MANAGER_LOAD_GAME, &args![manager, buffer]);
        }
        6 => {
            e.call(INTERFACE_LOAD_GAME, &args![buffer]);
        }
        7 => bgs_save_load_global_data_load_effects(e, buffer),
        8 => {
            let sky = e.call(SKY_GET_INSTANCE, &args![]).u32();
            e.call(SKY_LOAD_GAME, &args![sky, buffer]);
        }
        9 => {
            e.call(ACTOR_CAUSE_LOAD_LIST, &args![buffer]);
        }
        10 => {
            e.call(FALLOUT_RADIO_LOAD_GAME, &args![buffer]);
        }
        11 => {
            e.call(FALLOUT_AUDIO_LOAD_GAME, &args![buffer]);
        }
        1000 => {
            e.call(PROCESS_LISTS_SAVE_004534F0, &args![PROCESS_LISTS, buffer]);
        }
        _ => {
            e.call(SAVE_GAME_WARNING, &args![UNKNOWN_LOAD_TYPE, kind]);
            return false;
        }
    }
    true
}

/// Runs `per_type(type, &buffer)` for each type from `first` below `end` with a
/// `BGSLoadGameBuffer` local (`008646b0`, destroyed by `008646f0`).
fn for_each_type(e: &mut Engine, first: u32, end: u32, per_type: impl Fn(&mut Engine, u32, u32)) {
    // The exe's buffer is the 0x10 bytes at EBP-0x1C.
    e.with_stack(0x10, |e, buffer| {
        e.call(LOAD_BUFFER_CONSTRUCT, &args![buffer]);
        let mut kind = first;
        while kind < end {
            per_type(e, kind, buffer.addr());
            kind = kind.wrapping_add(1);
        }
        e.call(LOAD_BUFFER_DESTRUCT, &args![buffer]);
    });
}

// Translated from 0084bfb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0084bfb0 - the exception-unwinding frame (FS:[0]) that destroys the load buffer is not translated.
/// Initialises the global data types from `first` below `end` (unnamed in the engine map;
/// `cdecl`): [`fn_0084c030`] on each with a `BGSLoadGameBuffer` local.
pub fn fn_0084bfb0(e: &mut Engine, first: u32, end: u32) {
    for_each_type(e, first, end, |e, kind, buffer| {
        fn_0084c030(e, kind, buffer);
    });
}

// Translated from 0084c030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises one global data type before a load (`cdecl (type, buffer)`; the engine map's
/// `BGSSaveLoadGlobalData::InitGlobalDataRange` belongs to the loop [`fn_0084bfb0`]; the
/// warning says `InitGlobalData()`). Jump table `0084c0e0`: 4 `00975af0` on the process
/// lists, 5 `CombatManager::InitLoadGame`, 9 `ActorCause::InitLoadActorCausesList`, 10
/// `FalloutRadio::InitLoadGame` answer true; 0..=3, 6..=8, 11 and 1000 answer false; other
/// types are reported and answer false.
pub fn fn_0084c030(e: &mut Engine, kind: u32, buffer: u32) -> bool {
    match kind {
        4 => {
            e.call(PROCESS_LISTS_INIT_00975AF0, &args![PROCESS_LISTS, buffer]);
            true
        }
        5 => {
            let manager: u32 = e.global(COMBAT_MANAGER);
            e.call(COMBAT_MANAGER_INIT_LOAD_GAME, &args![manager, buffer]);
            true
        }
        9 => {
            e.call(ACTOR_CAUSE_INIT_LOAD_LIST, &args![buffer]);
            true
        }
        10 => {
            e.call(FALLOUT_RADIO_INIT_LOAD_GAME, &args![buffer]);
            true
        }
        0..=3 | 6..=8 | 11 | 1000 => false,
        _ => {
            e.call(SAVE_GAME_WARNING, &args![UNKNOWN_INIT_TYPE, kind]);
            false
        }
    }
}

// Translated from 0084c110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0084c110 - the exception-unwinding frame (FS:[0]) that destroys the load buffer is not translated.
/// Finishes loading the global data types from `first` below `end` (unnamed in the engine
/// map; `cdecl`): [`fn_0084c190`] on each with a `BGSLoadGameBuffer` local.
pub fn fn_0084c110(e: &mut Engine, first: u32, end: u32) {
    for_each_type(e, first, end, |e, kind, buffer| {
        fn_0084c190(e, kind, buffer);
    });
}

// Translated from 0084c190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finishes loading one global data type (`cdecl (type, buffer)`; the engine map's
/// `BGSSaveLoadGlobalData::FinishLoadGlobalDataRange` belongs to the loop [`fn_0084c110`]).
/// Jump table `0084c240`: 2 `00459750` on `TES`, 5 `00993790` on the combat manager, 6
/// `Interface::FinishLoadGame`, 10 `FalloutRadio::FinishLoadGame` answer true; 0, 1, 3, 4,
/// 7..=9, 11 and 1000 answer false; other types are reported and answer false.
pub fn fn_0084c190(e: &mut Engine, kind: u32, buffer: u32) -> bool {
    match kind {
        2 => {
            let tes: u32 = e.global(TES);
            e.call(TES_FINISH_00459750, &args![tes, buffer]);
            true
        }
        5 => {
            let manager: u32 = e.global(COMBAT_MANAGER);
            e.call(COMBAT_MANAGER_FINISH_00993790, &args![manager, buffer]);
            true
        }
        6 => {
            e.call(INTERFACE_FINISH_LOAD_GAME, &args![buffer]);
            true
        }
        10 => {
            e.call(FALLOUT_RADIO_FINISH_LOAD_GAME, &args![buffer]);
            true
        }
        0 | 1 | 3 | 4 | 7..=9 | 11 | 1000 => false,
        _ => {
            e.call(SAVE_GAME_WARNING, &args![UNKNOWN_FINISH_TYPE, kind]);
            false
        }
    }
}

// Translated from 0084c2a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0084c2a0 - the exception-unwinding frame (FS:[0]) that destroys the load buffer is not translated.
/// Reverts the global data types from `first` below `end` (unnamed in the engine map; `cdecl`:
/// first, end, a flag byte): [`fn_0084c330`] on each with a `BGSLoadGameBuffer` local and the
/// flag zero-extended.
pub fn fn_0084c2a0(e: &mut Engine, first: u32, end: u32, flag: u8) {
    for_each_type(e, first, end, |e, kind, buffer| {
        fn_0084c330(e, kind, buffer, flag);
    });
}

// Translated from 0084c330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reverts one global data type (`cdecl`: type, buffer, a flag byte; the engine map's
/// `BGSSaveLoadGlobalData::RevertGlobalDataRange` belongs to the loop [`fn_0084c2a0`]). Jump
/// table `0084c454`: 0 `004d60c0`, 2 `00459840` on `TES`, 3 [`fn_0084ced0`] only with the
/// flag, 4 `00975b60` on the process lists, 5 `00993800` on the combat manager, 6 `00706ae0`,
/// 7 [`bgs_save_load_global_data_revert_effects`], 9 `0066e910`, 10 `FalloutRadio::Revert`,
/// 11 `0040fbe0`, 1000 `00975c50` on the process lists answer true; 1, 8 and type 3 without
/// the flag answer false; other types are reported and answer false.
pub fn fn_0084c330(e: &mut Engine, kind: u32, buffer: u32, flag: u8) -> bool {
    match kind {
        0 => {
            e.call(MISC_STATS_REVERT_004D60C0, &args![buffer]);
            true
        }
        2 => {
            let tes: u32 = e.global(TES);
            e.call(TES_REVERT_00459840, &args![tes, buffer]);
            true
        }
        3 => {
            if flag != 0 {
                fn_0084ced0(e);
                true
            } else {
                false
            }
        }
        4 => {
            e.call(PROCESS_LISTS_REVERT_00975B60, &args![PROCESS_LISTS, buffer]);
            true
        }
        5 => {
            let manager: u32 = e.global(COMBAT_MANAGER);
            e.call(COMBAT_MANAGER_REVERT_00993800, &args![manager, buffer]);
            true
        }
        6 => {
            e.call(INTERFACE_REVERT_00706AE0, &args![buffer]);
            true
        }
        7 => {
            bgs_save_load_global_data_revert_effects(e);
            true
        }
        9 => {
            e.call(ACTOR_CAUSE_REVERT_0066E910, &args![buffer]);
            true
        }
        10 => {
            e.call(FALLOUT_RADIO_REVERT, &args![buffer]);
            true
        }
        11 => {
            e.call(REVERT_0040FBE0, &args![buffer]);
            true
        }
        1000 => {
            e.call(PROCESS_LISTS_REVERT_00975C50, &args![PROCESS_LISTS, buffer]);
            true
        }
        1 | 8 => false,
        _ => {
            e.call(SAVE_GAME_WARNING, &args![UNKNOWN_REVERT_TYPE, kind]);
            false
        }
    }
}

/// `0084c5a0`'s callees. `0078d770` (`cdecl`, the buffer; loading-menu unit),
/// `TES::SetWorldSpace` (`00458200`), `007037c0` and `0061cc40` (`thiscall` on `TES`, a word
/// each; the engine map's `BGSMenuPacker::RecomputePacking` for the first is a folded name),
/// `00430830` (`thiscall` on a reference: the address of three words passed after the
/// position), `PlayerCharacter::PositionPlayer` (`0093c200`) and `PositionPlayerExterior`
/// (`0093cce0`) (`thiscall`: position, three words, cell or world space, a word),
/// `PlayerCharacter::CenterOnCell` (`0093db60`: a word, the cell),
/// `TESWorldSpace::GetHasBorderRegion` (`00586260`), `00408d60` on `011e08e0` (the address of
/// a byte), `00588d10` (`thiscall` on the world space, the position's address; a `bool`),
/// `TES::UpdateCurrentGridCell` (`00452580`: a point's address, 0), `00457d70` (on `TES`: 1,
/// the world space, 0), `00461c20` (on the data handler: x, y, world space, a word; the cell),
/// `TESObjectCELL::Get3D` (`00545cb0`), `TESObjectCELL::RemoveReference` (`0054ca90`),
/// `00454450` (on `TES`, the point's address).
const LOADING_MENU_LOAD_0078D770: u32 = 0x0078_d770;
const TES_SET_WORLD_SPACE: u32 = 0x0045_8200;
const TES_WORD_007037C0: u32 = 0x0070_37c0;
const TES_WORD_0061CC40: u32 = 0x0061_cc40;
const REFR_ROTATION_00430830: u32 = 0x0043_0830;
const POSITION_PLAYER: u32 = 0x0093_c200;
const POSITION_PLAYER_EXTERIOR: u32 = 0x0093_cce0;
const CENTER_ON_CELL: u32 = 0x0093_db60;
const WORLD_HAS_BORDER_REGION: u32 = 0x0058_6260;
const BORDER_OBJECT: u32 = 0x011e_08e0;
const BORDER_BYTE_00408D60: u32 = 0x0040_8d60;
const WORLD_TEST_POSITION_00588D10: u32 = 0x0058_8d10;
const TES_UPDATE_CURRENT_GRID_CELL: u32 = 0x0045_2580;
const TES_00457D70: u32 = 0x0045_7d70;
const HANDLER_CELL_AT_00461C20: u32 = 0x0046_1c20;
const CELL_GET_3D: u32 = 0x0054_5cb0;
const CELL_REMOVE_REFERENCE: u32 = 0x0054_ca90;
const TES_00454450: u32 = 0x0045_4450;
/// `006815c0` (`thiscall`): answers `this` (the constructor of a 12-byte local here).
const RETURNS_THIS_006815C0: u32 = 0x0068_15c0;
/// `00406d90` (`cdecl`, a `float`): the integer the cell coordinate is made from (`SAR 12`).
const FLOAT_TO_INT_00406D90: u32 = 0x0040_6d90;
/// `00401170` (`thiscall` on a form): its type byte.
const FORM_TYPE: u32 = 0x0040_1170;
/// `0084e3a0` (`thiscall` on a form): its id.
const FORM_ID: u32 = 0x0084_e3a0;
/// The fallback of `0084c5a0`: the data handler's quest list (`00455600`), the quest's member
/// at +0x30 read by `00408da0` (its text; the engine map's `MapMarkerData::GetLocationName` is a
/// folded name) and `0048cee0` (its length), `strstr` (`00ec7750`), `"Quest %s (%08x) compared
/// to Patch04Debug\n"`, `"Patch04Debug"`, the player's word at +0xE4C, `0095f530` (on the
/// player: 0, 0xFF), the text of the setting at `011de25c` (`00403df0`), `strtol` (`00ec8d08`,
/// engine map `strtoxl`), the world space `000da726` and its cells: `005875a0`
/// (`TESWorldSpace::GetCellFromCellCoord`), `00585b30` (`TESWorldSpace::LoadCell`) (x, y each),
/// `TESDataHandler::NewCell` (`00461330`: three words, the world space).
const HANDLER_QUESTS_00455600: u32 = 0x0045_5600;
const STRING_TEXT_00408DA0: u32 = 0x0040_8da0;
const STRING_LENGTH_0048CEE0: u32 = 0x0048_cee0;
const STRSTR: u32 = 0x00ec_7750;
const QUEST_DEBUG_FORMAT: u32 = 0x0107_f6e4;
const PATCH_DEBUG_TEXT: u32 = 0x0107_f6d4;
const PLAYER_QUEST_WORD: u32 = 0xe4c;
const PLAYER_0095F530: u32 = 0x0095_f530;
const START_REFERENCE_SETTING: u32 = 0x011d_e25c;
const SETTING_TEXT_00403DF0: u32 = 0x0040_3df0;
const STRTOL: u32 = 0x00ec_8d08;
const DEFAULT_WORLD_SPACE_ID: u32 = 0x000d_a726;
const WORLD_CELL_AT: u32 = 0x0058_75a0;
const WORLD_LOAD_CELL: u32 = 0x0058_5b30;
const HANDLER_NEW_CELL: u32 = 0x0046_1330;
/// The end of `0084c5a0`: `004539a0` (on `TES`: 1, 0), `GarbageCollector::ClearAll`
/// (`00868d70`, `cdecl`, 0) and `00c459d0` (`cdecl`, 0).
const TES_004539A0: u32 = 0x0045_39a0;
const GARBAGE_CLEAR_ALL: u32 = 0x0086_8d70;
const FINISH_00C459D0: u32 = 0x00c4_59d0;

/// `PlayerCharacter::PositionPlayer` (or `PositionPlayerExterior`, `function`) on the player
/// with the three position words at `position`, the three words at `00430830`'s address of
/// `source`, the cell or world space and `last`.
fn position_player_at(
    e: &mut Engine,
    function: u32,
    source: u32,
    position: [u32; 3],
    place: u32,
    last: u32,
) {
    let other = e.call(REFR_ROTATION_00430830, &args![source]).u32();
    let r = [e.mem.u32(other), e.mem.u32(other + 4), e.mem.u32(other + 8)];
    let player: u32 = e.global(PLAYER);
    e.call(
        function,
        &args![
            player,
            position[0],
            position[1],
            position[2],
            r[0],
            r[1],
            r[2],
            place,
            last
        ],
    );
}

// Translated from 0084c5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads global data type 1, the location (`cdecl (buffer)`; unnamed in the engine map; the
/// counterpart of `SaveLocationData`): a word, the world space (a form id cast to
/// `TESWorldSpace`), two words x and y, the location's form id and three position words are
/// read, and `0078d770(buffer)` runs. A `CELL` location (type 0x39) positions the player in it
/// (unless already there), sets the world space and hands x and y to `007037c0` and
/// `0061cc40`. A `WRLD` location (type 0x41) sets the world space and, unless its border
/// region refuses the position, updates the grid cell from (x, y) * 4096, runs `00457d70`, and
/// finds the cell at the position's coordinates (`00461c20`, loading it after removing the
/// player from its cell and `00454450` when it has no 3D) to position the player in, with the
/// save-load flag 0x20 set meanwhile. Otherwise (no location, another type, no cell, a refused
/// position) the flag 0x40 is set, the quest whose text contains `"Patch04Debug"` (each one
/// printed) is put at the player's +0xE4C, and the player is placed at the reference named by
/// the setting at `011de25c` (in its world space or cell), else at a cell of world space
/// `000da726` (coordinates (-18, 0), then (0, 0), else a new cell). In the end `004539a0(1,
/// 0)`, `GarbageCollector::ClearAll(0)` and `00c459d0(0)` run.
pub fn fn_0084c5a0(e: &mut Engine, buffer: u32) {
    // The exe's locals from EBP-0x80.
    e.with_stack(0x80, |e, frame| {
        let ebp = frame.addr() + 0x80;
        let first_word = ebp - 0x18;
        let x_at = ebp - 0x14;
        let y_at = ebp - 0x1c;
        let position = ebp - 0xc;
        let point = ebp - 0x44;
        e.call(BUFFER_LOAD_DATA, &args![buffer, first_word, 4u32]);
        let id = e.call(BUFFER_LOAD_FORM_ID, &args![buffer]).u32();
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let world = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, CAST_SOURCE_TYPE, CAST_WORLD_SPACE_TYPE, 0u32],
            )
            .u32();
        e.call(BUFFER_LOAD_DATA, &args![buffer, x_at, 4u32]);
        e.call(BUFFER_LOAD_DATA, &args![buffer, y_at, 4u32]);
        let id = e.call(BUFFER_LOAD_FORM_ID, &args![buffer]).u32();
        let location = e.call(LOOKUP_FORM, &args![id]).u32();
        e.call(RETURNS_THIS_006815C0, &args![position]);
        e.call(BUFFER_LOAD_DATA, &args![buffer, position, 0xcu32]);
        e.call(LOADING_MENU_LOAD_0078D770, &args![buffer]);
        let saved_position = |e: &Engine| {
            [
                e.mem.u32(position),
                e.mem.u32(position + 4),
                e.mem.u32(position + 8),
            ]
        };
        let mut placed = true;
        if location == 0 {
            placed = false;
        } else if e.call(FORM_TYPE, &args![location]).u32() == 0x39 {
            let player: u32 = e.global(PLAYER);
            if e.call(PLAYER_FORM_008D6F30, &args![player]).u32() != location {
                let player: u32 = e.global(PLAYER);
                let p = saved_position(e);
                position_player_at(e, POSITION_PLAYER, player, p, location, 0);
            }
            if world != 0 {
                let tes: u32 = e.global(TES);
                e.call(TES_SET_WORLD_SPACE, &args![tes, world]);
            }
            let tes: u32 = e.global(TES);
            let x = e.mem.u32(x_at);
            e.call(TES_WORD_007037C0, &args![tes, x]);
            let tes: u32 = e.global(TES);
            let y = e.mem.u32(y_at);
            e.call(TES_WORD_0061CC40, &args![tes, y]);
        } else if e.call(FORM_TYPE, &args![location]).u32() == 0x41 {
            if world != 0 {
                let tes: u32 = e.global(TES);
                e.call(TES_SET_WORLD_SPACE, &args![tes, world]);
            }
            let space = location;
            if e.call(WORLD_HAS_BORDER_REGION, &args![space]).bool() {
                let byte_at = e.call(BORDER_BYTE_00408D60, &args![BORDER_OBJECT]).u32();
                if e.mem.u8(byte_at) != 0
                    && !e
                        .call(WORLD_TEST_POSITION_00588D10, &args![space, position])
                        .bool()
                {
                    placed = false;
                }
            }
            if placed {
                e.call(RETURNS_THIS_006815C0, &args![point]);
                let x = (e.mem.u32(x_at) << 12) as i32;
                e.mem.set_u32(point, x87_store32(f64::from(x)));
                let y = (e.mem.u32(y_at) << 12) as i32;
                e.mem.set_u32(point + 4, x87_store32(f64::from(y)));
                e.mem.set_u32(point + 8, 0);
                let tes: u32 = e.global(TES);
                e.call(TES_UPDATE_CURRENT_GRID_CELL, &args![tes, point, 0u32]);
                let tes: u32 = e.global(TES);
                e.call(TES_00457D70, &args![tes, 1u32, location, 0u32]);
                let x_bits = x87_quiet_bits(e.mem.u32(position));
                let cell_x = (e.call(FLOAT_TO_INT_00406D90, &args![x_bits]).i32() >> 12) as u32;
                let y_bits = x87_quiet_bits(e.mem.u32(position + 4));
                let cell_y = (e.call(FLOAT_TO_INT_00406D90, &args![y_bits]).i32() >> 12) as u32;
                let handler: u32 = e.global(DATA_HANDLER);
                let mut cell = e
                    .call(
                        HANDLER_CELL_AT_00461C20,
                        &args![handler, cell_x, cell_y, space, 0u32],
                    )
                    .u32();
                if cell == 0 || e.call(CELL_GET_3D, &args![cell]).u32() == 0 {
                    let player: u32 = e.global(PLAYER);
                    if e.call(PLAYER_FORM_008D6F30, &args![player]).u32() != 0 {
                        let player: u32 = e.global(PLAYER);
                        let parent = e.call(PLAYER_FORM_008D6F30, &args![player]).u32();
                        e.call(CELL_REMOVE_REFERENCE, &args![parent, player]);
                    }
                    let tes: u32 = e.global(TES);
                    e.call(TES_00454450, &args![tes, point]);
                    let handler: u32 = e.global(DATA_HANDLER);
                    cell = e
                        .call(
                            HANDLER_CELL_AT_00461C20,
                            &args![handler, cell_x, cell_y, space, 1u32],
                        )
                        .u32();
                }
                if cell != 0 {
                    let save_load: u32 = e.global(SAVE_LOAD_GAME);
                    fn_0084cbf0(e, Ptr::new(save_load), 1);
                    let player: u32 = e.global(PLAYER);
                    let p = saved_position(e);
                    position_player_at(e, POSITION_PLAYER, player, p, cell, 0);
                    let save_load: u32 = e.global(SAVE_LOAD_GAME);
                    fn_0084cbf0(e, Ptr::new(save_load), 0);
                } else {
                    placed = false;
                }
            }
        } else {
            placed = false;
        }
        if !placed {
            let save_load: u32 = e.global(SAVE_LOAD_GAME);
            fn_0084cc40(e, Ptr::new(save_load), 1);
            let handler: u32 = e.global(DATA_HANDLER);
            let mut node = e.call(HANDLER_QUESTS_00455600, &args![handler]).u32();
            while node != 0 {
                let slot = e.call(NODE_ITEM, &args![node]).u32();
                if e.mem.u32(slot) == 0 {
                    break;
                }
                let slot = e.call(NODE_ITEM, &args![node]).u32();
                let id = e.call(FORM_ID, &args![e.mem.u32(slot)]).u32();
                let slot = e.call(NODE_ITEM, &args![node]).u32();
                let text = e
                    .call(STRING_TEXT_00408DA0, &args![e.mem.u32(slot) + 0x30])
                    .u32();
                fn_0084cbd0(e, QUEST_DEBUG_FORMAT, &[text, id]);
                let slot = e.call(NODE_ITEM, &args![node]).u32();
                if e.call(STRING_LENGTH_0048CEE0, &args![e.mem.u32(slot) + 0x30])
                    .u32()
                    > 8
                {
                    let slot = e.call(NODE_ITEM, &args![node]).u32();
                    let text = e
                        .call(STRING_TEXT_00408DA0, &args![e.mem.u32(slot) + 0x30])
                        .u32();
                    if e.call(STRSTR, &args![text, PATCH_DEBUG_TEXT]).u32() != 0 {
                        let slot = e.call(NODE_ITEM, &args![node]).u32();
                        let player: u32 = e.global(PLAYER);
                        let quest = e.mem.u32(slot);
                        e.mem.set_u32(player + PLAYER_QUEST_WORD, quest);
                        break;
                    }
                }
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
            let player: u32 = e.global(PLAYER);
            e.call(PLAYER_0095F530, &args![player, 0u32, 0xffu32]);
            let text = e
                .call(SETTING_TEXT_00403DF0, &args![START_REFERENCE_SETTING])
                .u32();
            let id = e.call(STRTOL, &args![text, 0u32, 0x10u32]).u32();
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let reference = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, CAST_SOURCE_TYPE, CAST_REFERENCE_TYPE, 0u32],
                )
                .u32();
            if reference != 0 {
                let source = e.vcall(reference, 0x1f4, &args![]).u32();
                let p = [
                    e.mem.u32(source),
                    e.mem.u32(source + 4),
                    e.mem.u32(source + 8),
                ];
                let other = e.call(REFR_ROTATION_00430830, &args![reference]).u32();
                let r = [e.mem.u32(other), e.mem.u32(other + 4), e.mem.u32(other + 8)];
                let cell = e.call(PLAYER_FORM_008D6F30, &args![reference]).u32();
                let space = e.call(REFR_GET_WORLD_SPACE, &args![reference]).u32();
                placed = true;
                if space != 0 {
                    let player: u32 = e.global(PLAYER);
                    e.call(
                        POSITION_PLAYER_EXTERIOR,
                        &args![player, p[0], p[1], p[2], r[0], r[1], r[2], space, 1u32],
                    );
                } else if cell != 0 {
                    let player: u32 = e.global(PLAYER);
                    e.call(
                        POSITION_PLAYER,
                        &args![player, p[0], p[1], p[2], r[0], r[1], r[2], cell, 1u32],
                    );
                } else {
                    placed = false;
                }
            }
            if !placed {
                let form = e.call(LOOKUP_FORM, &args![DEFAULT_WORLD_SPACE_ID]).u32();
                let space = e
                    .call(
                        DYNAMIC_CAST,
                        &args![form, 0u32, CAST_SOURCE_TYPE, CAST_WORLD_SPACE_TYPE, 0u32],
                    )
                    .u32();
                if space != 0 {
                    let minus_18 = (-0x12i32) as u32;
                    let mut cell = e.call(WORLD_CELL_AT, &args![space, minus_18, 0u32]).u32();
                    if cell == 0 {
                        cell = e.call(WORLD_LOAD_CELL, &args![space, minus_18, 0u32]).u32();
                    }
                    if cell == 0 {
                        cell = e.call(WORLD_CELL_AT, &args![space, 0u32, 0u32]).u32();
                    }
                    if cell == 0 {
                        cell = e.call(WORLD_LOAD_CELL, &args![space, 0u32, 0u32]).u32();
                    }
                    if cell == 0 {
                        let handler: u32 = e.global(DATA_HANDLER);
                        cell = e
                            .call(HANDLER_NEW_CELL, &args![handler, 0u32, 0u32, 0u32, space])
                            .u32();
                    }
                    if cell != 0 {
                        let player: u32 = e.global(PLAYER);
                        e.call(CENTER_ON_CELL, &args![player, 0u32, cell]);
                    }
                }
            }
        }
        let tes: u32 = e.global(TES);
        e.call(TES_004539A0, &args![tes, 1u32, 0u32]);
        e.call(GARBAGE_CLEAR_ALL, &args![0u32]);
        e.call(FINISH_00C459D0, &args![0u32]);
    });
}

// Translated from 0084cbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A print (unnamed in the engine map; variadic `cdecl`: a format and its values):
/// `00ecbe60(format, the values' list)`, whose `EAX` it passes on.
pub fn fn_0084cbd0(e: &mut Engine, format: u32, values: &[u32]) -> u32 {
    let size = (4 * values.len() as u32).max(4);
    e.with_stack(size, |e, list| {
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u32(list.addr() + 4 * i as u32, *value);
        }
        e.call(VPRINTF_00ECBE60, &args![format, list]).u32()
    })
}

/// [`fn_0084cbd0`] in the uniform form: the format, then any number of values.
fn fn_0084cbd0_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(!words.is_empty(), "0084cbd0 needs its format word");
    Ret {
        eax: fn_0084cbd0(e, words[0], &words[1..]),
        ..Ret::default()
    }
}

// Translated from 0084cbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` non-zero) or clears bit 0x20 of the word at +0x244 (unnamed in the engine map;
/// called on the `BGSSaveLoadGame` singleton; `RET 4`).
pub fn fn_0084cbf0(e: &mut Engine, this: Ptr, set: u8) {
    let flags = e.mem.u32(this.addr() + 0x244);
    let flags = if set != 0 {
        flags | 0x20
    } else {
        flags & 0xffff_ffdf
    };
    e.mem.set_u32(this.addr() + 0x244, flags);
}

// Translated from 0084cc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` non-zero) or clears bit 0x40 of the word at +0x244 (unnamed in the engine map;
/// called on the `BGSSaveLoadGame` singleton; `RET 4`).
pub fn fn_0084cc40(e: &mut Engine, this: Ptr, set: u8) {
    let flags = e.mem.u32(this.addr() + 0x244);
    let flags = if set != 0 {
        flags | 0x40
    } else {
        flags & 0xffff_ffbf
    };
    e.mem.set_u32(this.addr() + 0x244, flags);
}

// Translated from 0084cd40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0084cd40 - the exception-unwinding frame (FS:[0]) that destroys the global map is not translated.
/// `BGSSaveLoadGlobalData::LoadGlobals` (Xbox PDB; `cdecl (buffer)`): every global of the data
/// handler's list goes into a local `NiTMap<TESGlobal *, bool>` (`0084d240` with 0x25 buckets,
/// [`fn_0084d310`] with 1); then for each of the variable-sized count of saved globals (a form id
/// cast to `TESGlobal`, a `float` read after it) the value is set (`0046dce0`) unless
/// `00549580` answers true, and the global leaves the map (`00405430`). The globals left in the
/// map that `00549580` does not skip are added to the reconstruct map
/// (`BGSReconstructFormsInAllFilesMap::AddForm(global, 0)`); the map is destroyed (`0084d3f0`).
pub fn bgs_save_load_global_data_load_globals(e: &mut Engine, buffer: u32) {
    let save_load: u32 = e.global(SAVE_LOAD_GAME);
    let reconstruct = e.call(RECONSTRUCT_MAP_007AF430, &args![save_load]).u32();
    // The exe's locals from EBP-0x48: the map at EBP-0x20, the value at EBP-0x38, the
    // position at EBP-0x10, the key at EBP-0x44 and the byte at EBP-0x3D.
    e.with_stack(0x48, |e, frame| {
        let ebp = frame.addr() + 0x48;
        let map = ebp - 0x20;
        let value_at = ebp - 0x38;
        let position_at = ebp - 0x10;
        let key_at = ebp - 0x44;
        let byte_at = ebp - 0x3d;
        fn_0084d240(e, Ptr::new(map), 0x25);
        let handler: u32 = e.global(DATA_HANDLER);
        let mut node = e.call(DATA_HANDLER_GLOBALS_00461190, &args![handler]).u32();
        while node != 0 {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let global = e.mem.u32(slot);
            if global != 0 {
                fn_0084d310(e, Ptr::new(map), global, 1);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        let count = e.call(BUFFER_LOAD_VARIABLE_SIZED, &args![buffer]).u32();
        let mut index = 0u32;
        while index < count {
            let id = e.call(BUFFER_LOAD_FORM_ID, &args![buffer]).u32();
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let global = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, CAST_SOURCE_TYPE, CAST_GLOBAL_TYPE, 0u32],
                )
                .u32();
            e.mem.set_u32(value_at, 0);
            e.call(BUFFER_LOAD_DATA, &args![buffer, value_at, 4u32]);
            if global != 0 {
                if !e.call(GLOBAL_SKIP_00549580, &args![global]).bool() {
                    let value = x87_quiet_bits(e.mem.u32(value_at));
                    e.call(GLOBAL_SET_VALUE, &args![global, value]);
                }
                e.call(MAP_REMOVE_AT, &args![map, global]);
            }
            index = index.wrapping_add(1);
        }
        let position = e.call(MAP_FIRST_POSITION, &args![map]).u32();
        e.mem.set_u32(position_at, position);
        while e.mem.u32(position_at) != 0 {
            e.mem.set_u32(key_at, 0);
            e.mem.set_u8(byte_at, 0);
            e.call(GLOBAL_MAP_NEXT, &args![map, position_at, key_at, byte_at]);
            let global = e.mem.u32(key_at);
            if !e.call(GLOBAL_SKIP_00549580, &args![global]).bool() {
                let global = e.mem.u32(key_at);
                e.call(RECONSTRUCT_ADD_FORM, &args![reconstruct, global, 0u32]);
            }
        }
        fn_0084d3f0(e, Ptr::new(map));
    });
}

// Translated from 0084ced0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reverting global data type 3 (unnamed in the engine map; `cdecl`, no words): every global of
/// the data handler's list is added to the reconstruct map (`007af430` of `*011ddf38`) with
/// `BGSReconstructFormsInAllFilesMap::AddForm(global, 0)`.
pub fn fn_0084ced0(e: &mut Engine) {
    let save_load: u32 = e.global(SAVE_LOAD_GAME);
    let reconstruct = e.call(RECONSTRUCT_MAP_007AF430, &args![save_load]).u32();
    let handler: u32 = e.global(DATA_HANDLER);
    let mut node = e.call(DATA_HANDLER_GLOBALS_00461190, &args![handler]).u32();
    while node != 0 {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let global = e.mem.u32(slot);
        e.call(RECONSTRUCT_ADD_FORM, &args![reconstruct, global, 0u32]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 0084d070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadGlobalData::LoadEffects` (Xbox PDB; `cdecl (buffer)`): for each of the
/// variable-sized count of saved effects, a `float` (0 first), a second `float` (0 unless the
/// buffer's version, vtable slot 0, is at least 14), and a form id cast to
/// `ImageSpaceModifierInstanceForm`'s modifier type; a found modifier is triggered
/// (`ImageSpaceModifierInstanceForm::Trigger(modifier, first, 0)`) and a new instance gets
/// `006694c0(*011c3c08 - second)` (x87, stored as a `float`).
pub fn bgs_save_load_global_data_load_effects(e: &mut Engine, buffer: u32) {
    let count = e.call(BUFFER_LOAD_VARIABLE_SIZED, &args![buffer]).u32();
    // The exe's locals from EBP-0x1C: the first float at EBP-0xC, the second at EBP-0x14.
    e.with_stack(0x1c, |e, frame| {
        let ebp = frame.addr() + 0x1c;
        let first_at = ebp - 0xc;
        let second_at = ebp - 0x14;
        let mut index = 0u32;
        while index < count {
            e.mem.set_u32(first_at, 0);
            e.call(BUFFER_LOAD_DATA, &args![buffer, first_at, 4u32]);
            e.mem.set_u32(second_at, 0);
            let version = e.vcall(buffer, 0, &args![]).u8();
            if version >= 0xe {
                e.call(BUFFER_LOAD_DATA, &args![buffer, second_at, 4u32]);
            }
            let id = e.call(BUFFER_LOAD_FORM_ID, &args![buffer]).u32();
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let modifier = e
                .call(
                    DYNAMIC_CAST,
                    &args![
                        form,
                        0u32,
                        CAST_SOURCE_TYPE,
                        CAST_IMAGE_SPACE_MODIFIER_TYPE,
                        0u32
                    ],
                )
                .u32();
            if modifier != 0 {
                let first = x87_quiet_bits(e.mem.u32(first_at));
                let instance = e
                    .call(IMAGE_SPACE_TRIGGER, &args![modifier, first, 0u32])
                    .u32();
                if instance != 0 {
                    let now = x87_load32(e.global::<u32>(EFFECT_TIME));
                    let age = x87_store32(x87_sub(now, x87_m32(e.mem.u32(second_at))));
                    e.call(INSTANCE_SET_006694C0, &args![instance, x87_quiet_bits(age)]);
                }
            }
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 0084d160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0084d160 - the exception-unwinding frame (FS:[0]) that destroys the smart pointer local is not translated.
/// `BGSSaveLoadGlobalData::RevertEffects` (Xbox PDB; `cdecl`, no words): `004e0870`,
/// `004defb0`, `00c50930` on `0045a190`'s object; then over `TES`'s `0043b5d0` list, an entry
/// whose object (`00559450`) lacks bit 0 of +0x18 ([`fn_0084d050`]) is removed from the list
/// (`BSSimpleList<..>::Remove` with a smart pointer to it, `00633c90` / `0045cec0`) and the walk
/// goes on at the same node; an entry with the bit set gets `0063f790(0.0)` and the walk
/// steps to the next node.
pub fn bgs_save_load_global_data_revert_effects(e: &mut Engine) {
    e.call(REVERT_004E0870, &args![]);
    e.call(REVERT_004DEFB0, &args![]);
    let object = e.call(OBJECT_0045A190, &args![]).u32();
    e.call(REVERT_00C50930, &args![object]);
    let tes: u32 = e.global(TES);
    let mut node = e.call(TES_LIST_0043B5D0, &args![tes]).u32();
    // The exe's smart pointer local at EBP-0x18.
    e.with_stack(4, |e, pointer| {
        while node != 0 {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let object = e.call(POINTER_GET_00559450, &args![slot]).u32();
            if object != 0 && !fn_0084d050(e, Ptr::new(object)) {
                e.call(POINTER_CONSTRUCT, &args![pointer, object]);
                e.call(LIST_REMOVE, &args![node, pointer]);
                e.call(POINTER_DESTRUCT, &args![pointer]);
                continue;
            }
            if object != 0 {
                e.call(EFFECT_SET_0063F790, &args![object, 0u32]);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
}

// Translated from 0084d240 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMap<TESGlobal *, bool>` constructor (unnamed in the engine map; `RET 4`):
/// `0084d2a0(count)`, then the vtable. Answers `this`.
pub fn fn_0084d240(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    fn_0084d2a0(e, this, bucket_count);
    e.mem.set_u32(this.addr(), GLOBAL_MAP_VTABLE);
    this
}

// Translated from 0084d270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESGlobal *,bool>::_scalar_deleting_destructor_` (Xbox PDB): `0084d3f0`, then frees
/// `this` (`00401030`) with bit 0 of `flags`. Answers `this`.
pub fn ni_t_map_global_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0084d3f0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0084d2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<.., TESGlobal *, bool>` constructor (unnamed in the engine map; `RET 4`):
/// the vtable, the bucket count at +4, 0 at +0xC, a bucket table of `4 * count` bytes
/// (`00aa1070`) at +8, zeroed (`00403d30`). Answers `this`.
pub fn fn_0084d2a0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, GLOBAL_MAP_BASE_VTABLE);
    e.mem.set_u32(t + 4, bucket_count);
    e.mem.set_u32(t + 0xc, 0);
    let bytes = e.mem.u32(t + 4) << 2;
    let table = e.call(ALLOCATE, &args![bytes]).u32();
    e.mem.set_u32(t + 8, table);
    let bytes = e.mem.u32(t + 4) << 2;
    let table = e.mem.u32(t + 8);
    e.call(MEMSET, &args![table, 0u32, bytes]);
    this
}

// Translated from 0084d3f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0084d3f0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The `NiTMap<TESGlobal *, bool>` destructor (unnamed in the engine map): its vtable, the map
/// emptied (`00438af0`), then the base destructor [`fn_0084d450`].
pub fn fn_0084d3f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), GLOBAL_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_0084d450(e, this);
}

// Translated from 0084d450 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<.., TESGlobal *, bool>` destructor (unnamed in the engine map): its vtable,
/// the map emptied (`00438af0`), the bucket table (+8) freed (`00aa10f0`).
pub fn fn_0084d450(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), GLOBAL_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.mem.u32(this.addr() + 8);
    e.call(FREE, &args![table]);
}

// Translated from 0084d480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..TESGlobal *,bool>::_scalar_deleting_destructor_` (Xbox PDB): `0084d450`, then
/// frees `this` (`00401030`) with bit 0 of `flags`. Answers `this`.
pub fn ni_t_map_base_global_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0084d450(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0084_c270, fn_0084c270(u8)),
        entry!(0x0084d030, fn_0084d030(Ptr) -> f64),
        entry!(
            0x0084_bcd0,
            bgs_save_load_global_data_save_global_data(u32, u32) -> bool
        ),
        entry!(
            0x0084_c490,
            bgs_save_load_global_data_save_location_data(u32)
        ),
        entry!(0x0084_c580, fn_0084c580(Ptr) -> u32),
        entry!(0x0084_cc90, bgs_save_load_global_data_save_globals(u32)),
        entry!(0x0084_cf30, fn_0084cf30(u32)),
        entry!(0x0084_d050, fn_0084d050(Ptr) -> bool),
        entry!(0x0084_d310, fn_0084d310(Ptr, u32, u8)),
        entry!(0x0084_be40, bgs_save_load_global_data_load_global_data(u32, u32) -> bool),
        entry!(0x0084_bfb0, fn_0084bfb0(u32, u32)),
        entry!(0x0084_c030, fn_0084c030(u32, u32) -> bool),
        entry!(0x0084_c110, fn_0084c110(u32, u32)),
        entry!(0x0084_c190, fn_0084c190(u32, u32) -> bool),
        entry!(0x0084_c2a0, fn_0084c2a0(u32, u32, u8)),
        entry!(0x0084_c330, fn_0084c330(u32, u32, u8) -> bool),
        entry!(0x0084_c5a0, fn_0084c5a0(u32)),
        (0x0084_cbd0, fn_0084cbd0_entry as AbiFn),
        entry!(0x0084_cbf0, fn_0084cbf0(Ptr, u8)),
        entry!(0x0084_cc40, fn_0084cc40(Ptr, u8)),
        entry!(0x0084_cd40, bgs_save_load_global_data_load_globals(u32)),
        entry!(0x0084_ced0, fn_0084ced0()),
        entry!(0x0084_d070, bgs_save_load_global_data_load_effects(u32)),
        entry!(0x0084_d160, bgs_save_load_global_data_revert_effects()),
        entry!(0x0084_d240, fn_0084d240(Ptr, u32) -> Ptr),
        entry!(0x0084_d270, ni_t_map_global_bool_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0084_d2a0, fn_0084d2a0(Ptr, u32) -> Ptr),
        entry!(0x0084_d3f0, fn_0084d3f0(Ptr)),
        entry!(0x0084_d450, fn_0084d450(Ptr)),
        entry!(0x0084_d480, ni_t_map_base_global_bool_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_float_at_0xc_into_st0() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_f32(object + 8, 7.0);
        for value in [0.016_666_668f32, -1.0, 0.0] {
            e.mem.set_f32(object + 0xc, value);
            assert_eq!(e.call(0x0084d030, &[object]).f64(), value as f64);
        }
    }
}

#[cfg(test)]
mod save_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Saved = Rc<RefCell<Vec<Vec<u32>>>>;

    const BUFFER: u32 = 0x0000_6000;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// Doubles for the save buffer: raw data is recorded as words, form ids
    /// as `[0xF0F0, form]`, the variable-sized value as `[count, start]`.
    fn buffer_doubles(e: &mut Engine) -> Saved {
        let saved: Saved = Rc::new(RefCell::new(vec![]));
        let record = saved.clone();
        e.register_double(BUFFER_SAVE_DATA, move |e, a| {
            assert_eq!((a[0], a[3]), (BUFFER, 0));
            let words = (0..a[2] / 4).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            record.borrow_mut().push(words);
            Ret::default()
        });
        let record = saved.clone();
        e.register_double(BUFFER_SAVE_FORM_ID, move |_, a| {
            assert_eq!((a[0], a[2]), (BUFFER, 0));
            record.borrow_mut().push(vec![0xf0f0, a[1]]);
            Ret::default()
        });
        e.register(BUFFER_START_VARIABLE_SIZED, |_, _| word(0x77));
        let record = saved.clone();
        e.register_double(BUFFER_SAVE_VARIABLE_SIZED, move |_, a| {
            record.borrow_mut().push(vec![a[1], a[2]]);
            Ret::default()
        });
        e.register(NODE_ITEM, |_, a| word(a[0]));
        e.register(NODE_NEXT, |e, a| word(e.mem.u32(a[0] + 4)));
        saved
    }

    /// A list of nodes (item, next) holding `items`.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    fn singletons(e: &mut Engine) {
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x011f_1000, 0x1000);
        e.set_global(TES, 0x0000_5100u32);
        e.set_global(PLAYER, 0x0000_5200u32);
        e.set_global(COMBAT_MANAGER, 0x0000_5300u32);
    }

    fn location_doubles(e: &mut Engine) {
        let handler = e.mem.alloc(0x210);
        e.mem.set_u32(handler + 0x208, 0x1208);
        e.set_global(DATA_HANDLER, handler);
        e.register(TES_GET_WORLD_SPACE, |_, a| word(a[0] + 1));
        e.register(WORD_24_0059BB30, |_, _| word(0x24));
        e.register(WORD_28_0045CD60, |_, _| word(0x28));
        e.register(REFR_GET_WORLD_SPACE, |_, _| word(0));
        e.register(PLAYER_FORM_008D6F30, |_, a| word(a[0] + 2));
        let position = e.mem.alloc(0xc);
        for i in 0..3 {
            e.mem.set_u32(position + 4 * i, 0x100 + i);
        }
        e.register_double(POSITION_00436AA0, move |_, _| word(position));
        e.register(LOADING_MENU_SAVE_0078D6B0, |_, _| Ret::default());
    }

    #[test]
    fn save_location_data_saves_each_value_in_order() {
        let mut e = Engine::new();
        singletons(&mut e);
        let saved = buffer_doubles(&mut e);
        location_doubles(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0084_c490, &args![BUFFER]);
        assert_eq!(
            *saved.borrow(),
            vec![
                vec![0x1208],
                vec![0xf0f0, 0x5101],
                vec![0x24],
                vec![0x28],
                vec![0xf0f0, 0x5202],
                vec![0x100, 0x101, 0x102],
            ]
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.last().unwrap(),
            &(LOADING_MENU_SAVE_0078D6B0, vec![BUFFER])
        );

        // With a player worldspace that one is saved.
        e.register(REFR_GET_WORLD_SPACE, |_, _| word(0x9999));
        saved.borrow_mut().clear();
        e.call(0x0084_c490, &args![BUFFER]);
        assert_eq!(saved.borrow()[4], vec![0xf0f0, 0x9999]);
    }

    #[test]
    fn fn_0084c580_reads_the_word_at_0x208() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x210);
        e.mem.set_u32(this + 0x208, 0x55);
        assert_eq!(e.call(0x0084_c580, &args![this]).u32(), 0x55);
    }

    #[test]
    fn save_globals_saves_the_unskipped_globals_and_their_count() {
        let mut e = Engine::new();
        singletons(&mut e);
        let saved = buffer_doubles(&mut e);
        let globals = list(&mut e, &[0x4000, 0, 0x4100, 0x4200]);
        let handler = e.mem.alloc(0x10);
        e.set_global(DATA_HANDLER, handler);
        e.register_double(DATA_HANDLER_GLOBALS_00461190, move |_, a| {
            assert_eq!(a[0], handler);
            word(globals)
        });
        e.register(GLOBAL_SKIP_00549580, |_, a| word(u32::from(a[0] == 0x4100)));
        e.register(GLOBAL_GET_VALUE, |_, a| st0(f64::from(a[0] >> 8)));
        e.call(0x0084_cc90, &args![BUFFER]);
        assert_eq!(
            *saved.borrow(),
            vec![
                vec![0xf0f0, 0x4000],
                vec![64.0f32.to_bits()],
                vec![0xf0f0, 0x4200],
                vec![66.0f32.to_bits()],
                vec![2, 0x77],
            ]
        );
    }

    #[test]
    fn fn_0084cf30_saves_each_usable_entry() {
        let mut e = Engine::new();
        singletons(&mut e);
        let saved = buffer_doubles(&mut e);
        // Three entries: no object, an object with bit 0 of +0x18, a usable
        // object whose +0x94 object exists.
        let flagged = e.mem.alloc(0x20);
        e.mem.set_u32(flagged + 0x18, 1);
        let usable = e.mem.alloc(0x20);
        let inner = e.mem.alloc(0x10);
        e.mem.set_f32(inner + 0xc, 1.5);
        let mut slots = vec![0u32; 0x98 / 4];
        slots[0x94 / 4] = 0x0001_0010;
        e.put_vtable(0x0002_0000, &slots);
        e.mem.set_u32(usable, 0x0002_0000);
        e.register_double(0x0001_0010, move |_, _| word(inner));
        let entries = list(&mut e, &[0x10, 0x20, 0x30]);
        e.register_double(TES_LIST_0043B5D0, move |_, a| {
            assert_eq!(a[0], 0x5100);
            word(entries)
        });
        e.register_double(POINTER_GET_00559450, move |e, a| {
            word(match e.mem.u32(a[0]) {
                0x20 => flagged,
                0x30 => usable,
                _ => 0,
            })
        });
        e.register(GLOBAL_VALUE_00529990, |_, _| st0(10.0));
        e.register(FLOAT_GETTER_0047C860, |_, _| st0(2.5));
        e.register_double(WORD_GETTER_00441110, move |_, a| {
            assert_eq!(a[0], inner);
            word(0x3333)
        });
        e.call(0x0084_cf30, &args![BUFFER]);
        assert_eq!(
            *saved.borrow(),
            vec![
                vec![1.5f32.to_bits()],
                vec![7.5f32.to_bits()],
                vec![0xf0f0, 0x3333],
                vec![1, 0x77],
            ]
        );
    }

    #[test]
    fn fn_0084d050_tests_bit_0_of_0x18() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, 2);
        assert!(!e.call(0x0084_d050, &args![this]).bool());
        e.mem.set_u32(this + 0x18, 3);
        assert!(e.call(0x0084_d050, &args![this]).bool());
    }

    #[test]
    fn save_global_data_dispatches_each_type() {
        let mut e = Engine::new();
        singletons(&mut e);
        buffer_doubles(&mut e);
        location_doubles(&mut e);
        e.register(DATA_HANDLER_GLOBALS_00461190, |_, _| word(0));
        e.register(TES_LIST_0043B5D0, |_, _| word(0));
        let handlers = [
            MISC_STATS_SAVE_004D5FA0,
            TES_SAVE_GAME,
            PROCESS_LISTS_SAVE_GAME,
            COMBAT_MANAGER_SAVE_GAME,
            INTERFACE_SAVE_007066D0,
            SKY_SAVE_GAME,
            ACTOR_CAUSE_SAVE_LIST,
            FALLOUT_RADIO_SAVE_GAME,
            FALLOUT_AUDIO_SAVE_GAME,
            PROCESS_LISTS_SAVE_004534F0,
            SAVE_GAME_WARNING,
        ];
        for handler in handlers {
            e.register(handler, |_, _| Ret::default());
        }
        e.register(SKY_GET_INSTANCE, |_, _| word(0x5400));
        let expected: [(u32, u32, Vec<u32>); 13] = [
            (0, MISC_STATS_SAVE_004D5FA0, vec![BUFFER]),
            (1, LOADING_MENU_SAVE_0078D6B0, vec![BUFFER]),
            (2, TES_SAVE_GAME, vec![0x5100, BUFFER]),
            (3, BUFFER_SAVE_VARIABLE_SIZED, vec![BUFFER, 0, 0x77]),
            (4, PROCESS_LISTS_SAVE_GAME, vec![PROCESS_LISTS, BUFFER]),
            (5, COMBAT_MANAGER_SAVE_GAME, vec![0x5300, BUFFER]),
            (6, INTERFACE_SAVE_007066D0, vec![BUFFER]),
            (7, BUFFER_SAVE_VARIABLE_SIZED, vec![BUFFER, 0, 0x77]),
            (8, SKY_SAVE_GAME, vec![0x5400, BUFFER]),
            (9, ACTOR_CAUSE_SAVE_LIST, vec![BUFFER]),
            (10, FALLOUT_RADIO_SAVE_GAME, vec![BUFFER]),
            (11, FALLOUT_AUDIO_SAVE_GAME, vec![BUFFER]),
            (
                1000,
                PROCESS_LISTS_SAVE_004534F0,
                vec![PROCESS_LISTS, BUFFER],
            ),
        ];
        for (kind, handler, words) in expected {
            e.call_log = Some(vec![]);
            assert!(e.call(0x0084_bcd0, &args![kind, BUFFER]).bool());
            let log = e.call_log.take().unwrap();
            assert_eq!(log.last().unwrap(), &(handler, words), "type {kind}");
        }
        for kind in [12u32, 999, 1001] {
            e.call_log = Some(vec![]);
            assert!(!e.call(0x0084_bcd0, &args![kind, BUFFER]).bool());
            let log = e.call_log.take().unwrap();
            assert_eq!(
                log.last().unwrap(),
                &(SAVE_GAME_WARNING, vec![UNKNOWN_TYPE_WARNING, kind])
            );
        }
    }

    #[test]
    fn fn_0084d310_replaces_or_inserts() {
        let mut e = Engine::new();
        let map = e.mem.alloc(0x10);
        let buckets = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, buckets);
        e.put_vtable(
            0x0002_0000,
            &[0, 0x0001_0004, 0x0001_0008, 0x0001_000c, 0, 0x0001_0014],
        );
        e.mem.set_u32(map, 0x0002_0000);
        e.register(0x0001_0004, |_, a| word(a[1] % 4));
        e.register(0x0001_0008, |_, a| word(u32::from(a[1] == a[2])));
        e.register(0x0001_0014, |e, _| word(e.mem.alloc(0xc)));
        e.register(0x0001_000c, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u8(a[1] + 8, a[3] as u8);
            Ret::default()
        });
        e.call(0x0084_d310, &args![map, 5u32, 7u32]);
        e.call(0x0084_d310, &args![map, 9u32, 8u32]);
        let head = e.mem.u32(buckets + 4);
        assert_eq!(e.mem.u32(head + 4), 9, "the newest node heads the bucket");
        let second = e.mem.u32(head);
        assert_eq!((e.mem.u32(second + 4), e.mem.u8(second + 8)), (5, 7));
        assert_eq!(e.mem.u32(map + 0xc), 2);
        e.call(0x0084_d310, &args![map, 5u32, 0x109u32]);
        assert_eq!(e.mem.u8(second + 8), 9);
        assert_eq!(e.mem.u32(map + 0xc), 2);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_0084c270_reverts_both_ranges() {
        let mut e = Engine::new();
        e.register(REVERT_GLOBAL_DATA_TYPES, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0084_c270, &args![0xff01u32]);
        let log = e.call_log.take().unwrap();
        let ranges: Vec<Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == REVERT_GLOBAL_DATA_TYPES)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(ranges, vec![vec![0, 0xc, 1], vec![0x3e8, 0x3e9, 1]]);
    }
}

#[cfg(test)]
mod n088_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const BUFFER: u32 = 0x0000_6000;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for a in addresses {
            e.register(*a, |_, _| Ret::default());
        }
    }

    /// The singletons' pages and pointers.
    fn singletons(e: &mut Engine) {
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_d000, 0x2000);
        e.map(0x011f_1000, 0x1000);
        e.set_global(TES, 0x0000_5100u32);
        e.set_global(PLAYER, 0x0000_5200u32);
        e.set_global(COMBAT_MANAGER, 0x0000_5300u32);
        e.set_global(DATA_HANDLER, 0x0000_5400u32);
        let save_load = e.mem.alloc(0x248);
        e.set_global(SAVE_LOAD_GAME, save_load);
    }

    #[test]
    fn load_global_data_dispatches_each_type() {
        let mut e = Engine::new();
        singletons(&mut e);
        let handlers = [
            MISC_STATS_LOAD_GAME,
            TES_LOAD_GAME,
            PROCESS_LISTS_LOAD_GAME,
            COMBAT_MANAGER_LOAD_GAME,
            INTERFACE_LOAD_GAME,
            SKY_LOAD_GAME,
            ACTOR_CAUSE_LOAD_LIST,
            FALLOUT_RADIO_LOAD_GAME,
            FALLOUT_AUDIO_LOAD_GAME,
            PROCESS_LISTS_SAVE_004534F0,
            SAVE_GAME_WARNING,
            BUFFER_LOAD_DATA,
            LOADING_MENU_LOAD_0078D770,
            HANDLER_QUESTS_00455600,
            PLAYER_0095F530,
            SETTING_TEXT_00403DF0,
            STRTOL,
            LOOKUP_FORM,
            DYNAMIC_CAST,
            BUFFER_LOAD_FORM_ID,
            RETURNS_THIS_006815C0,
            TES_004539A0,
            GARBAGE_CLEAR_ALL,
            FINISH_00C459D0,
            RECONSTRUCT_MAP_007AF430,
            DATA_HANDLER_GLOBALS_00461190,
            ALLOCATE,
            MEMSET,
            BUFFER_LOAD_VARIABLE_SIZED,
            MAP_FIRST_POSITION,
            MAP_REMOVE_ALL,
            FREE,
        ];
        quiet(&mut e, &handlers);
        e.register(SKY_GET_INSTANCE, |_, _| ret(0x5500));
        let expected: [(u32, u32, Vec<u32>); 13] = [
            (0, MISC_STATS_LOAD_GAME, vec![BUFFER]),
            (1, FINISH_00C459D0, vec![0]),
            (2, TES_LOAD_GAME, vec![0x5100, BUFFER]),
            (3, FREE, vec![0]),
            (4, PROCESS_LISTS_LOAD_GAME, vec![PROCESS_LISTS, BUFFER]),
            (5, COMBAT_MANAGER_LOAD_GAME, vec![0x5300, BUFFER]),
            (6, INTERFACE_LOAD_GAME, vec![BUFFER]),
            (7, BUFFER_LOAD_VARIABLE_SIZED, vec![BUFFER]),
            (8, SKY_LOAD_GAME, vec![0x5500, BUFFER]),
            (9, ACTOR_CAUSE_LOAD_LIST, vec![BUFFER]),
            (10, FALLOUT_RADIO_LOAD_GAME, vec![BUFFER]),
            (11, FALLOUT_AUDIO_LOAD_GAME, vec![BUFFER]),
            (
                1000,
                PROCESS_LISTS_SAVE_004534F0,
                vec![PROCESS_LISTS, BUFFER],
            ),
        ];
        for (kind, handler, words) in expected {
            e.call_log = Some(vec![]);
            assert!(e.call(0x0084_be40, &[kind, BUFFER]).bool(), "type {kind}");
            let log = e.call_log.take().unwrap();
            assert_eq!(log.last().unwrap(), &(handler, words), "type {kind}");
        }
        for kind in [12u32, 999, 1001] {
            e.call_log = Some(vec![]);
            assert!(!e.call(0x0084_be40, &[kind, BUFFER]).bool());
            assert_eq!(
                calls(&e, SAVE_GAME_WARNING),
                vec![vec![UNKNOWN_LOAD_TYPE, kind]]
            );
        }
    }

    /// Runs `function` for `kinds` and gives each answer with the calls after the top one.
    fn answers(
        e: &mut Engine,
        function: u32,
        kinds: &[u32],
        extra: &[u32],
    ) -> Vec<(bool, Vec<(u32, Vec<u32>)>)> {
        kinds
            .iter()
            .map(|kind| {
                e.call_log = Some(vec![]);
                let mut words = vec![*kind, BUFFER];
                words.extend_from_slice(extra);
                let answer = e.call(function, &words).bool();
                let mut log = e.call_log.take().unwrap();
                log.remove(0);
                (answer, log)
            })
            .collect()
    }

    #[test]
    fn init_and_finish_per_type() {
        let mut e = Engine::new();
        singletons(&mut e);
        quiet(
            &mut e,
            &[
                PROCESS_LISTS_INIT_00975AF0,
                COMBAT_MANAGER_INIT_LOAD_GAME,
                ACTOR_CAUSE_INIT_LOAD_LIST,
                FALLOUT_RADIO_INIT_LOAD_GAME,
                TES_FINISH_00459750,
                COMBAT_MANAGER_FINISH_00993790,
                INTERFACE_FINISH_LOAD_GAME,
                FALLOUT_RADIO_FINISH_LOAD_GAME,
                SAVE_GAME_WARNING,
            ],
        );
        let kinds = [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1000, 12];
        let init = answers(&mut e, 0x0084_c030, &kinds, &[]);
        let handled: Vec<(u32, Vec<(u32, Vec<u32>)>)> = kinds
            .iter()
            .zip(init.iter())
            .filter(|(_, (answer, _))| *answer)
            .map(|(kind, (_, log))| (*kind, log.clone()))
            .collect();
        assert_eq!(
            handled,
            vec![
                (
                    4,
                    vec![(PROCESS_LISTS_INIT_00975AF0, vec![PROCESS_LISTS, BUFFER])]
                ),
                (
                    5,
                    vec![(COMBAT_MANAGER_INIT_LOAD_GAME, vec![0x5300, BUFFER])]
                ),
                (9, vec![(ACTOR_CAUSE_INIT_LOAD_LIST, vec![BUFFER])]),
                (10, vec![(FALLOUT_RADIO_INIT_LOAD_GAME, vec![BUFFER])]),
            ]
        );
        assert!(init[..13]
            .iter()
            .all(|(answer, log)| *answer || log.is_empty()));
        assert_eq!(
            init[13].1,
            vec![(SAVE_GAME_WARNING, vec![UNKNOWN_INIT_TYPE, 12])]
        );
        let finish = answers(&mut e, 0x0084_c190, &kinds, &[]);
        let handled: Vec<(u32, Vec<(u32, Vec<u32>)>)> = kinds
            .iter()
            .zip(finish.iter())
            .filter(|(_, (answer, _))| *answer)
            .map(|(kind, (_, log))| (*kind, log.clone()))
            .collect();
        assert_eq!(
            handled,
            vec![
                (2, vec![(TES_FINISH_00459750, vec![0x5100, BUFFER])]),
                (
                    5,
                    vec![(COMBAT_MANAGER_FINISH_00993790, vec![0x5300, BUFFER])]
                ),
                (6, vec![(INTERFACE_FINISH_LOAD_GAME, vec![BUFFER])]),
                (10, vec![(FALLOUT_RADIO_FINISH_LOAD_GAME, vec![BUFFER])]),
            ]
        );
        assert!(finish[..13]
            .iter()
            .all(|(answer, log)| *answer || log.is_empty()));
        assert_eq!(
            finish[13].1,
            vec![(SAVE_GAME_WARNING, vec![UNKNOWN_FINISH_TYPE, 12])]
        );
    }

    #[test]
    fn revert_per_type() {
        let mut e = Engine::new();
        singletons(&mut e);
        quiet(
            &mut e,
            &[
                MISC_STATS_REVERT_004D60C0,
                TES_REVERT_00459840,
                PROCESS_LISTS_REVERT_00975B60,
                COMBAT_MANAGER_REVERT_00993800,
                INTERFACE_REVERT_00706AE0,
                ACTOR_CAUSE_REVERT_0066E910,
                FALLOUT_RADIO_REVERT,
                REVERT_0040FBE0,
                PROCESS_LISTS_REVERT_00975C50,
                SAVE_GAME_WARNING,
                REVERT_004E0870,
                REVERT_004DEFB0,
                OBJECT_0045A190,
                REVERT_00C50930,
                TES_LIST_0043B5D0,
                RECONSTRUCT_MAP_007AF430,
                DATA_HANDLER_GLOBALS_00461190,
            ],
        );
        let kinds = [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1000, 12];
        let revert = answers(&mut e, 0x0084_c330, &kinds, &[0x100]);
        let first: Vec<(u32, bool, Option<u32>)> = kinds
            .iter()
            .zip(revert.iter())
            .map(|(kind, (answer, log))| (*kind, *answer, log.first().map(|c| c.0)))
            .collect();
        assert_eq!(
            first,
            vec![
                (0, true, Some(MISC_STATS_REVERT_004D60C0)),
                (1, false, None),
                (2, true, Some(TES_REVERT_00459840)),
                (3, false, None),
                (4, true, Some(PROCESS_LISTS_REVERT_00975B60)),
                (5, true, Some(COMBAT_MANAGER_REVERT_00993800)),
                (6, true, Some(INTERFACE_REVERT_00706AE0)),
                (7, true, Some(REVERT_004E0870)),
                (8, false, None),
                (9, true, Some(ACTOR_CAUSE_REVERT_0066E910)),
                (10, true, Some(FALLOUT_RADIO_REVERT)),
                (11, true, Some(REVERT_0040FBE0)),
                (1000, true, Some(PROCESS_LISTS_REVERT_00975C50)),
                (12, false, Some(SAVE_GAME_WARNING)),
            ]
        );
        assert_eq!(revert[2].1[0].1, vec![0x5100, BUFFER]);
        assert_eq!(revert[12].1[0].1, vec![PROCESS_LISTS, BUFFER]);
        // Type 3 with the flag: the globals go to the reconstruct map.
        let with_flag = answers(&mut e, 0x0084_c330, &[3], &[1]);
        assert!(with_flag[0].0);
        assert_eq!(with_flag[0].1[0].0, RECONSTRUCT_MAP_007AF430);
    }

    #[test]
    fn the_range_loops_run_each_type_with_a_buffer() {
        let mut e = Engine::new();
        singletons(&mut e);
        quiet(
            &mut e,
            &[
                LOAD_BUFFER_CONSTRUCT,
                LOAD_BUFFER_DESTRUCT,
                SAVE_GAME_WARNING,
                PROCESS_LISTS_INIT_00975AF0,
                COMBAT_MANAGER_INIT_LOAD_GAME,
                COMBAT_MANAGER_FINISH_00993790,
                INTERFACE_FINISH_LOAD_GAME,
                PROCESS_LISTS_REVERT_00975B60,
                COMBAT_MANAGER_REVERT_00993800,
            ],
        );
        for (address, handlers) in [
            (
                0x0084_bfb0u32,
                [PROCESS_LISTS_INIT_00975AF0, COMBAT_MANAGER_INIT_LOAD_GAME],
            ),
            (
                0x0084_c110,
                [COMBAT_MANAGER_FINISH_00993790, INTERFACE_FINISH_LOAD_GAME],
            ),
        ] {
            e.call_log = Some(vec![]);
            e.call(address, &[4, 7]);
            let made = calls(&e, LOAD_BUFFER_CONSTRUCT);
            assert_eq!(made.len(), 1);
            let buffer = made[0][0];
            let handled: Vec<u32> = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(a, _)| handlers.contains(a))
                .map(|(a, w)| {
                    assert_eq!(*w.last().unwrap(), buffer);
                    *a
                })
                .collect();
            assert_eq!(handled.len(), 2, "{address:08x}: types 4 and 5");
            assert_eq!(calls(&e, LOAD_BUFFER_DESTRUCT), vec![vec![buffer]]);
        }
        e.call_log = Some(vec![]);
        e.call(0x0084_c2a0, &[4, 6, 0x0201]);
        assert_eq!(calls(&e, PROCESS_LISTS_REVERT_00975B60).len(), 1);
        assert_eq!(calls(&e, COMBAT_MANAGER_REVERT_00993800).len(), 1);
        // An empty range makes and destroys the buffer only.
        e.call_log = Some(vec![]);
        e.call(0x0084_bfb0, &[5, 5]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
    }

    /// Doubles for `0084c5a0`: the buffer's reads give `words` in order (raw data word by word),
    /// form ids map to `forms` by `LOOKUP_FORM` and casts pass the form on.
    fn location_rig(e: &mut Engine, words: Vec<u32>, ids: Vec<u32>) {
        singletons(e);
        let words = Rc::new(RefCell::new(words));
        e.register_double(BUFFER_LOAD_DATA, move |e, a| {
            for i in 0..a[2] / 4 {
                let w = words.borrow_mut().remove(0);
                e.mem.set_u32(a[1] + 4 * i, w);
            }
            Ret::default()
        });
        let ids = Rc::new(RefCell::new(ids));
        e.register_double(BUFFER_LOAD_FORM_ID, move |_, _| {
            ret(ids.borrow_mut().remove(0))
        });
        e.register(LOOKUP_FORM, |_, a| ret(a[0]));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(RETURNS_THIS_006815C0, |_, a| ret(a[0]));
        e.register(FORM_TYPE, |e, a| ret(u32::from(e.mem.u8(a[0] + 4))));
        e.register(FLOAT_TO_INT_00406D90, |_, a| {
            ret(f32::from_bits(a[0]) as i32 as u32)
        });
        let rotation = e.mem.alloc(12);
        e.mem.set_u32(rotation, 0x11);
        e.mem.set_u32(rotation + 4, 0x22);
        e.mem.set_u32(rotation + 8, 0x33);
        e.register_double(REFR_ROTATION_00430830, move |_, _| ret(rotation));
        quiet(
            e,
            &[
                LOADING_MENU_LOAD_0078D770,
                POSITION_PLAYER,
                POSITION_PLAYER_EXTERIOR,
                CENTER_ON_CELL,
                TES_SET_WORLD_SPACE,
                TES_WORD_007037C0,
                TES_WORD_0061CC40,
                TES_UPDATE_CURRENT_GRID_CELL,
                TES_00457D70,
                CELL_REMOVE_REFERENCE,
                TES_00454450,
                TES_004539A0,
                GARBAGE_CLEAR_ALL,
                FINISH_00C459D0,
                PLAYER_0095F530,
                VPRINTF_00ECBE60,
            ],
        );
    }

    fn typed(e: &mut Engine, kind: u8) -> u32 {
        let f = e.mem.alloc(0x40);
        e.mem.set_u8(f + 4, kind);
        f
    }

    #[test]
    fn load_location_into_a_cell() {
        let mut e = Engine::new();
        let cell = typed(&mut e, 0x39);
        location_rig(&mut e, vec![7, 3, 4, 10, 20, 30], vec![0x7000, cell]);
        e.register(PLAYER_FORM_008D6F30, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0084_c5a0, &[BUFFER]);
        assert_eq!(
            calls(&e, POSITION_PLAYER),
            vec![vec![0x5200, 10, 20, 30, 0x11, 0x22, 0x33, cell, 0]]
        );
        assert_eq!(calls(&e, TES_SET_WORLD_SPACE), vec![vec![0x5100, 0x7000]]);
        assert_eq!(calls(&e, TES_WORD_007037C0), vec![vec![0x5100, 3]]);
        assert_eq!(calls(&e, TES_WORD_0061CC40), vec![vec![0x5100, 4]]);
        assert_eq!(calls(&e, TES_004539A0), vec![vec![0x5100, 1, 0]]);
        assert!(calls(&e, HANDLER_QUESTS_00455600).is_empty());
        // Already in the cell, without a world space: no positioning.
        location_rig(&mut e, vec![7, 3, 4, 10, 20, 30], vec![0, cell]);
        e.register_double(PLAYER_FORM_008D6F30, move |_, _| ret(cell));
        e.call_log = Some(vec![]);
        e.call(0x0084_c5a0, &[BUFFER]);
        assert!(calls(&e, POSITION_PLAYER).is_empty());
        assert!(calls(&e, TES_SET_WORLD_SPACE).is_empty());
    }

    #[test]
    fn load_location_into_a_world_space() {
        let mut e = Engine::new();
        let space = typed(&mut e, 0x41);
        let position = [8192.0f32.to_bits(), (-4096.0f32).to_bits(), 5];
        let mut words = vec![7, 3, 0xffff_ffff];
        words.extend_from_slice(&position);
        location_rig(&mut e, words.clone(), vec![space, space]);
        e.register(WORLD_HAS_BORDER_REGION, |_, _| ret(0));
        e.register(HANDLER_CELL_AT_00461C20, |_, a| {
            ret(if a[4] == 1 { 0x0c00_0000 } else { 0 })
        });
        e.register(PLAYER_FORM_008D6F30, |_, _| ret(0x0c10_0000));
        let save_load: u32 = e.global(SAVE_LOAD_GAME);
        e.register_double(POSITION_PLAYER, move |e, _| {
            assert_eq!(e.mem.u32(save_load + 0x244) & 0x20, 0x20);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0084_c5a0, &[BUFFER]);
        let grid = calls(&e, TES_UPDATE_CURRENT_GRID_CELL);
        assert_eq!(grid.len(), 1);
        assert_eq!(calls(&e, TES_00457D70), vec![vec![0x5100, 1, space, 0]]);
        assert_eq!(
            calls(&e, HANDLER_CELL_AT_00461C20),
            vec![
                vec![0x5400, 2, u32::MAX, space, 0],
                vec![0x5400, 2, u32::MAX, space, 1]
            ]
        );
        assert_eq!(
            calls(&e, CELL_REMOVE_REFERENCE),
            vec![vec![0x0c10_0000, 0x5200]]
        );
        assert_eq!(calls(&e, TES_00454450), vec![vec![0x5100, grid[0][1]]]);
        assert_eq!(
            calls(&e, POSITION_PLAYER),
            vec![vec![
                0x5200,
                position[0],
                position[1],
                5,
                0x11,
                0x22,
                0x33,
                0x0c00_0000,
                0
            ]]
        );
        assert_eq!(e.mem.u32(save_load + 0x244) & 0x20, 0);
        // The grid point: (3 << 12, -1 << 12, 0) as floats.
        location_rig(&mut e, words, vec![space, space]);
        e.register_double(TES_UPDATE_CURRENT_GRID_CELL, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 12288.0f32.to_bits());
            assert_eq!(e.mem.u32(a[1] + 4), (-4096.0f32).to_bits());
            assert_eq!(e.mem.u32(a[1] + 8), 0);
            Ret::default()
        });
        e.register(WORLD_HAS_BORDER_REGION, |_, _| ret(0));
        e.register(HANDLER_CELL_AT_00461C20, |_, _| ret(0x0c00_0000));
        e.register(CELL_GET_3D, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0084_c5a0, &[BUFFER]);
        assert!(calls(&e, TES_00454450).is_empty());
        assert_eq!(calls(&e, POSITION_PLAYER).len(), 1);
    }

    #[test]
    fn load_location_falls_back_to_the_start_reference_or_a_default_cell() {
        let mut e = Engine::new();
        // A refused border position.
        let space = typed(&mut e, 0x41);
        location_rig(&mut e, vec![7, 3, 4, 10, 20, 30], vec![0, space]);
        e.register(WORLD_HAS_BORDER_REGION, |_, _| ret(1));
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        e.register_double(BORDER_BYTE_00408D60, move |_, _| ret(flag));
        e.register(WORLD_TEST_POSITION_00588D10, |_, _| ret(0));
        // Two quests: the second's text contains the debug text.
        let quests: Vec<u32> = (0..2).map(|_| e.mem.alloc(0x40)).collect();
        let node_b = e.mem.alloc(8);
        let node_a = e.mem.alloc(8);
        e.mem.set_u32(node_a, quests[0]);
        e.mem.set_u32(node_a + 4, node_b);
        e.mem.set_u32(node_b, quests[1]);
        e.register_double(HANDLER_QUESTS_00455600, move |_, _| ret(node_a));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(FORM_ID, |_, a| ret(a[0] + 1));
        e.register(STRING_TEXT_00408DA0, |_, a| ret(a[0]));
        e.register(STRING_LENGTH_0048CEE0, |_, _| ret(12));
        let second = quests[1];
        e.register_double(STRSTR, move |_, a| ret(u32::from(a[0] == second + 0x30)));
        e.register(SETTING_TEXT_00403DF0, |_, _| ret(0x0e00_0000));
        e.register(STRTOL, |_, _| ret(0));
        e.register(LOOKUP_FORM, |_, a| {
            ret(if a[0] == DEFAULT_WORLD_SPACE_ID {
                0x0d00_0000
            } else {
                a[0]
            })
        });
        e.register(WORLD_CELL_AT, |_, _| ret(0));
        e.register(WORLD_LOAD_CELL, |_, a| {
            ret(if a[1] == 0 { 0x0d10_0000 } else { 0 })
        });
        let player = e.mem.alloc(0xe50);
        e.set_global(PLAYER, player);
        let printed: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(vec![]));
        let p = printed.clone();
        e.register_double(VPRINTF_00ECBE60, move |e, a| {
            p.borrow_mut()
                .push(vec![a[0], e.mem.u32(a[1]), e.mem.u32(a[1] + 4)]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0084_c5a0, &[BUFFER]);
        let save_load: u32 = e.global(SAVE_LOAD_GAME);
        assert_eq!(e.mem.u32(save_load + 0x244) & 0x40, 0x40);
        assert_eq!(e.mem.u32(player + 0xe4c), second);
        assert_eq!(
            *printed.borrow(),
            vec![
                vec![QUEST_DEBUG_FORMAT, quests[0] + 0x30, quests[0] + 1],
                vec![QUEST_DEBUG_FORMAT, second + 0x30, second + 1]
            ]
        );
        assert_eq!(calls(&e, STRTOL), vec![vec![0x0e00_0000, 0, 0x10]]);
        assert_eq!(
            calls(&e, WORLD_CELL_AT),
            vec![
                vec![0x0d00_0000, (-18i32) as u32, 0],
                vec![0x0d00_0000, 0, 0]
            ]
        );
        assert_eq!(
            calls(&e, CENTER_ON_CELL),
            vec![vec![player, 0, 0x0d10_0000]]
        );
        assert!(calls(&e, HANDLER_NEW_CELL).is_empty());
        assert_eq!(calls(&e, PLAYER_0095F530), vec![vec![player, 0, 0xff]]);
        // No location: the start reference, exterior or interior.
        let reference = e.mem.alloc(0x40);
        e.put_vtable(0x0710_0000, &{
            let mut s = vec![0u32; 0x80];
            s[0x1f4 / 4] = 0x7100_01f4;
            s
        });
        e.mem.set_u32(reference, 0x0710_0000);
        let place = e.mem.alloc(12);
        for i in 0..3 {
            e.mem.set_u32(place + 4 * i, 0x40 + i);
        }
        e.register_double(0x7100_01f4, move |_, _| ret(place));
        for (world, cell, function, last) in [
            (
                0x0a00_0000u32,
                0x0b00_0000u32,
                POSITION_PLAYER_EXTERIOR,
                0x0a00_0000u32,
            ),
            (0, 0x0b00_0000, POSITION_PLAYER, 0x0b00_0000),
        ] {
            location_rig(&mut e, vec![7, 3, 4, 10, 20, 30], vec![0, 0]);
            e.register(HANDLER_QUESTS_00455600, |_, _| ret(0));
            e.register(SETTING_TEXT_00403DF0, |_, _| ret(0x0e00_0000));
            e.register_double(STRTOL, move |_, _| ret(reference));
            e.register_double(REFR_GET_WORLD_SPACE, move |_, _| ret(world));
            e.register_double(PLAYER_FORM_008D6F30, move |_, _| ret(cell));
            e.call_log = Some(vec![]);
            e.call(0x0084_c5a0, &[BUFFER]);
            assert_eq!(
                calls(&e, function),
                vec![vec![0x5200, 0x40, 0x41, 0x42, 0x11, 0x22, 0x33, last, 1]]
            );
            assert!(calls(&e, CENTER_ON_CELL).is_empty());
        }
        // Neither: the default world space; no cell at all makes a new one.
        location_rig(&mut e, vec![7, 3, 4, 10, 20, 30], vec![0, 0]);
        e.register(HANDLER_QUESTS_00455600, |_, _| ret(0));
        e.register(SETTING_TEXT_00403DF0, |_, _| ret(0x0e00_0000));
        e.register(STRTOL, |_, _| ret(0));
        e.register(LOOKUP_FORM, |_, a| {
            ret(if a[0] == DEFAULT_WORLD_SPACE_ID {
                0x0d00_0000
            } else {
                0
            })
        });
        e.register(WORLD_CELL_AT, |_, _| ret(0));
        e.register(WORLD_LOAD_CELL, |_, _| ret(0));
        e.register(HANDLER_NEW_CELL, |_, _| ret(0x0d20_0000));
        e.call_log = Some(vec![]);
        e.call(0x0084_c5a0, &[BUFFER]);
        assert_eq!(
            calls(&e, HANDLER_NEW_CELL),
            vec![vec![0x5400, 0, 0, 0, 0x0d00_0000]]
        );
        assert_eq!(
            calls(&e, CENTER_ON_CELL),
            vec![vec![0x5200, 0, 0x0d20_0000]]
        );
    }

    #[test]
    fn the_print_and_the_save_load_flags() {
        let mut e = Engine::new();
        e.register(VPRINTF_00ECBE60, |e, a| {
            ret(e.mem.u32(a[1]) + e.mem.u32(a[1] + 4))
        });
        assert_eq!(e.call(0x0084_cbd0, &[0x0107_0000, 5, 6]).u32(), 11);
        e.register(VPRINTF_00ECBE60, |_, _| ret(3));
        assert_eq!(e.call(0x0084_cbd0, &[0x0107_0000]).u32(), 3);
        let this = e.mem.alloc(0x248);
        e.mem.set_u32(this + 0x244, 0x101);
        e.call(0x0084_cbf0, &[this, 0x100]);
        assert_eq!(e.mem.u32(this + 0x244), 0x101);
        e.call(0x0084_cbf0, &[this, 1]);
        assert_eq!(e.mem.u32(this + 0x244), 0x121);
        e.call(0x0084_cc40, &[this, 1]);
        assert_eq!(e.mem.u32(this + 0x244), 0x161);
        e.call(0x0084_cbf0, &[this, 0]);
        e.call(0x0084_cc40, &[this, 0]);
        assert_eq!(e.mem.u32(this + 0x244), 0x101);
    }

    /// A list of nodes (item, next) holding `items`.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    #[test]
    fn load_globals_and_add_the_rest_to_the_reconstruct_map() {
        let mut e = Engine::new();
        singletons(&mut e);
        let globals = list(&mut e, &[0x4000, 0, 0x4100, 0x4200]);
        e.register_double(DATA_HANDLER_GLOBALS_00461190, move |_, _| ret(globals));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(RECONSTRUCT_MAP_007AF430, |_, _| ret(0x0a00_0000));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        quiet(
            &mut e,
            &[
                MEMSET,
                MAP_REMOVE_ALL,
                FREE,
                RECONSTRUCT_ADD_FORM,
                GLOBAL_SET_VALUE,
            ],
        );
        // The map's own vtable slots (the bucket, the key test, the node set-up, a new node)
        // run the real 0084d310; the set-up records the keys in order.
        e.put_vtable(
            GLOBAL_MAP_VTABLE,
            &[0, 0x7300_0004, 0x7300_0008, 0x7300_000c, 0, 0x7300_0014],
        );
        e.register(0x7300_0004, |_, a| ret(a[1] % 0x25));
        e.register(0x7300_0008, |_, a| ret(u32::from(a[1] == a[2])));
        e.register(0x7300_0014, |e, _| ret(e.mem.alloc(0xc)));
        let keys: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
        let k = keys.clone();
        e.register_double(0x7300_000c, move |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u8(a[1] + 8, a[3] as u8);
            k.borrow_mut().push(a[2]);
            Ret::default()
        });
        let k = keys.clone();
        e.register_double(MAP_REMOVE_AT, move |_, a| {
            k.borrow_mut().retain(|x| *x != a[1]);
            Ret::default()
        });
        let k = keys.clone();
        e.register_double(MAP_FIRST_POSITION, move |_, _| {
            ret(u32::from(!k.borrow().is_empty()))
        });
        let k = keys.clone();
        e.register_double(GLOBAL_MAP_NEXT, move |e, a| {
            let position = e.mem.u32(a[1]) as usize;
            let key = k.borrow()[position - 1];
            e.mem.set_u32(a[2], key);
            let next = if position < k.borrow().len() {
                position + 1
            } else {
                0
            };
            e.mem.set_u32(a[1], next as u32);
            Ret::default()
        });
        e.register(BUFFER_LOAD_VARIABLE_SIZED, |_, _| ret(3));
        let ids = Rc::new(RefCell::new(vec![0x4100u32, 0x4000, 0x9999]));
        e.register_double(BUFFER_LOAD_FORM_ID, move |_, _| {
            ret(ids.borrow_mut().remove(0))
        });
        e.register(LOOKUP_FORM, |_, a| ret(a[0]));
        e.register(DYNAMIC_CAST, |_, a| {
            ret(if a[0] == 0x9999 { 0 } else { a[0] })
        });
        e.register(BUFFER_LOAD_DATA, |e, a| {
            e.mem.set_u32(a[1], 2.5f32.to_bits());
            Ret::default()
        });
        e.register(GLOBAL_SKIP_00549580, |_, a| ret(u32::from(a[0] == 0x4000)));
        e.call_log = Some(vec![]);
        e.call(0x0084_cd40, &[BUFFER]);
        // The map was made with 0x25 buckets and destroyed.
        let made = calls(&e, ALLOCATE);
        assert_eq!(made, vec![vec![0x94]]);
        assert_eq!(calls(&e, FREE).len(), 1);
        assert_eq!(
            calls(&e, GLOBAL_SET_VALUE),
            vec![vec![0x4100, 2.5f32.to_bits()]]
        );
        assert_eq!(calls(&e, MAP_REMOVE_AT).len(), 2);
        assert_eq!(*keys.borrow(), vec![0x4200]);
        assert_eq!(
            calls(&e, RECONSTRUCT_ADD_FORM),
            vec![vec![0x0a00_0000, 0x4200, 0]]
        );
        // 0084ced0: every global of the list, nulls included.
        e.call_log = Some(vec![]);
        e.call(0x0084_ced0, &[]);
        let added: Vec<u32> = calls(&e, RECONSTRUCT_ADD_FORM)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(added, vec![0x4000, 0, 0x4100, 0x4200]);
    }

    #[test]
    fn load_effects_triggers_the_modifiers() {
        let mut e = Engine::new();
        singletons(&mut e);
        e.set_global(EFFECT_TIME, 10.0f32);
        let buffer = e.mem.alloc(4);
        e.put_vtable(0x0710_0000, &[0x7100_0000]);
        e.mem.set_u32(buffer, 0x0710_0000);
        e.register(BUFFER_LOAD_VARIABLE_SIZED, |_, _| ret(3));
        let ids = Rc::new(RefCell::new(vec![0x10u32, 0x20, 0]));
        e.register_double(BUFFER_LOAD_FORM_ID, move |_, _| {
            ret(ids.borrow_mut().remove(0))
        });
        e.register(LOOKUP_FORM, |_, a| ret(a[0]));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(IMAGE_SPACE_TRIGGER, |_, a| {
            ret(if a[0] == 0x20 { 0x0c00_0000 } else { 0 })
        });
        quiet(&mut e, &[INSTANCE_SET_006694C0]);
        // The version byte: 14 (as 0x10E) for the first entry, then 0.
        let mut versions = vec![0x10eu32];
        e.register_double(0x7100_0000, move |_, _| {
            ret(if versions.is_empty() {
                0
            } else {
                versions.remove(0)
            })
        });
        let mut floats = vec![1.0f32, 2.0, 0.5, 0.0];
        e.register_double(BUFFER_LOAD_DATA, move |e, a| {
            let value = if floats.is_empty() {
                0.0
            } else {
                floats.remove(0)
            };
            e.mem.set_u32(a[1], value.to_bits());
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0084_d070, &[buffer]);
        // Entry 1 (version 14): first 1.0, second 2.0, modifier 0x10 (no instance).
        // Entry 2 (version 0): first 0.5 only, modifier 0x20, instance: 10 - 0.
        // Entry 3: id 0, nothing.
        assert_eq!(
            calls(&e, IMAGE_SPACE_TRIGGER),
            vec![
                vec![0x10, 1.0f32.to_bits(), 0],
                vec![0x20, 0.5f32.to_bits(), 0]
            ]
        );
        assert_eq!(
            calls(&e, INSTANCE_SET_006694C0),
            vec![vec![0x0c00_0000, 10.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, BUFFER_LOAD_DATA).len(), 4);
    }

    #[test]
    fn revert_effects_removes_the_unflagged_entries() {
        let mut e = Engine::new();
        singletons(&mut e);
        quiet(
            &mut e,
            &[
                REVERT_004E0870,
                REVERT_004DEFB0,
                REVERT_00C50930,
                POINTER_CONSTRUCT,
                POINTER_DESTRUCT,
                EFFECT_SET_0063F790,
            ],
        );
        e.register(OBJECT_0045A190, |_, _| ret(0x0c00_0000));
        let flagged = e.mem.alloc(0x20);
        e.mem.set_u32(flagged + 0x18, 1);
        let plain = e.mem.alloc(0x20);
        let entries = list(&mut e, &[0, plain, flagged]);
        e.register_double(TES_LIST_0043B5D0, move |_, _| ret(entries));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(POINTER_GET_00559450, |e, a| ret(e.mem.u32(a[0])));
        // The removal copies the next node into this one, as a list's head removal does.
        e.register(LIST_REMOVE, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let item = e.mem.u32(next);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0084_d160, &[]);
        assert_eq!(calls(&e, REVERT_00C50930), vec![vec![0x0c00_0000]]);
        let removed = calls(&e, LIST_REMOVE);
        assert_eq!(removed.len(), 1);
        assert_eq!(
            calls(&e, POINTER_CONSTRUCT),
            vec![vec![removed[0][1], plain]]
        );
        assert_eq!(calls(&e, EFFECT_SET_0063F790), vec![vec![flagged, 0]]);
    }

    #[test]
    fn the_global_map() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        quiet(&mut e, &[MEMSET, MAP_REMOVE_ALL, FREE, OPERATOR_DELETE]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0084_d240, &[this, 0x25]).u32(), this);
        assert_eq!(e.mem.u32(this), GLOBAL_MAP_VTABLE);
        assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 0xc)), (0x25, 0));
        let table = e.mem.u32(this + 8);
        assert_eq!(calls(&e, MEMSET), vec![vec![table, 0, 0x94]]);
        let other = e.mem.alloc(0x10);
        e.call(0x0084_d2a0, &[other, 2]);
        assert_eq!(e.mem.u32(other), GLOBAL_MAP_BASE_VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x0084_d3f0, &[this]);
        assert_eq!(e.mem.u32(this), GLOBAL_MAP_BASE_VTABLE);
        assert_eq!(calls(&e, MAP_REMOVE_ALL), vec![vec![this]; 2]);
        assert_eq!(calls(&e, FREE), vec![vec![table]]);
        for address in [0x0084_d270u32, 0x0084_d480] {
            for (flags, frees) in [(0u32, 0usize), (1, 1)] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(address, &[this, flags]).u32(), this);
                assert_eq!(calls(&e, FREE).len(), 1);
                assert_eq!(calls(&e, OPERATOR_DELETE).len(), frees);
            }
        }
        e.call_log = Some(vec![]);
        e.call(0x0084_d450, &[this]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL).len(), 1);
    }
}
