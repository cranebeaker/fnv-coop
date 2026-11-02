//! `fallout shared/bgsencounterzone.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BGSEncounterZone` (form type `0x61`, record `ECZN`) is 0x30 bytes on the PC:
//! the 0x18-byte `TESForm`, then `ENCOUNTER_ZONE_DATA` (8 bytes, the record's
//! `DATA` chunk) and `ENCOUNTER_ZONE_GAME_DATA` (0x10 bytes, run-time only).
//! Its vtable is at `0102cbbc` (the RTTI locator before it names
//! `BGSEncounterZone`); this unit holds the destructor (slot `0x10`),
//! `InitializeData` (`0x14`), `Load` (`0x20`), `Save` (`0x2C`), `InitItem`
//! (`0x88`), `Copy` (`0x108`) and `Compare` (`0x10C`), plus the zone's own
//! accessors and `CreateNoZone`.
//!
//! Layouts and addresses are at the top; every function is translated from
//! the disassembly, and the three functions that run under a compiler
//! exception frame or the stack cookie carry a `Translation incomplete`
//! marker (the unwinding and the cookie check are not translated).

#[allow(unused_imports)]
use crate::prelude::*;

// ---- Layout --------------------------------------------------------------------------

layout! {
    /// `BGSEncounterZone` (Xbox PDB), 0x30 bytes on the PC (the Xbox build is
    /// 0x40: its `TESForm` is 0x10 bytes larger). Names are the Xbox PDB's;
    /// offsets are the PC code's.
    pub struct BGSEncounterZone: 0x30 {
        /// `Data.pZoneOwner` (Xbox PDB, `TESForm*`): the zone's owner. In
        /// the saved record it is the owner's form id; `InitItem` turns the
        /// id into the pointer.
        0x18 pZoneOwner: u32,
        /// `Data.iOwnerRank` (Xbox PDB, `char`).
        0x1C iOwnerRank: i8,
        /// `Data.cMinLevel` (Xbox PDB).
        0x1D cMinLevel: u8,
        /// `Data.cFlags` (Xbox PDB, `char`): bit 0 and bit 1 are tested by
        /// `fn_00526320` and `fn_00526250`.
        0x1E cFlags: i8,
        /// `GameData.iDetachTime` (Xbox PDB).
        0x20 iDetachTime: u32,
        /// `GameData.iAttachTime` (Xbox PDB).
        0x24 iAttachTime: u32,
        /// `GameData.iResetTime` (Xbox PDB).
        0x28 iResetTime: u32,
        /// `GameData.sZoneLevel` (Xbox PDB).
        0x2C sZoneLevel: u16,
    }
}

/// Size of the `DATA` chunk (`ENCOUNTER_ZONE_DATA`).
const DATA_SIZE: u32 = 8;
/// Size of `ENCOUNTER_ZONE_GAME_DATA`.
const GAME_DATA_SIZE: u32 = 0x10;
/// Offset of `ENCOUNTER_ZONE_DATA` in the zone.
const DATA_OFFSET: u32 = 0x18;
/// Offset of `ENCOUNTER_ZONE_GAME_DATA` in the zone.
const GAME_DATA_OFFSET: u32 = 0x20;

/// The four-character code of a chunk, as the file stores it.
const fn tag(name: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*name)
}

const CHUNK_DATA: u32 = tag(b"DATA");
const CHUNK_EDID: u32 = tag(b"EDID");
const CHUNK_OBND: u32 = tag(b"OBND");

/// The form type byte of an encounter zone record (`0x61`, set by the
/// constructor and checked by `Load`).
const FORM_TYPE_ENCOUNTER_ZONE: u32 = 0x61;
/// `BGSEncounterZone`'s vtable.
const VTABLE: u32 = 0x0102_cbbc;
/// The form id the "no zone" zone gets (`CreateNoZone`).
const NO_ZONE_FORM_ID: u32 = 0x1e;

// ---- Globals --------------------------------------------------------------------------

/// `BGSEncounterZone::pNoZone` (Xbox PDB static): the zone `CreateNoZone`
/// makes.
const NO_ZONE: u32 = 0x011c_9520;
/// The player character pointer.
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// The data handler (`TESDataHandler*`).
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// A game setting object whose value is read as a `u32` through `0043d4d0`
/// (`fn_00526100`). Its name is not confirmed.
const SETTING_RESET_TIME: u32 = 0x011c_a160;
/// A game setting object whose value is read as a `float` through
/// `00403e20` (`fn_00526190`). Its name is not confirmed.
const SETTING_LEVEL_SCALE: u32 = 0x011c_9550;
/// The object `fn_00526100`'s caller passes to `00867e30` (an object with a
/// `TESGlobal*` at `+0x10`; its identity is not confirmed).
const GAME_TIME_OBJECT: u32 = 0x011d_e7b8;

// ---- Calls outside this file ---------------------------------------------------------

/// `TESForm::TESForm` (Xbox PDB), `thiscall`.
const FORM_CONSTRUCTOR: u32 = 0x0048_3370;
/// `TESForm::~TESForm` (Xbox PDB), `thiscall`.
const FORM_DESTRUCTOR: u32 = 0x0048_3630;
/// Stores the byte argument at `+0x04` of the form (`TESForm::cFormType` in
/// the Xbox PDB), `thiscall`, `RET 4` (no name in the engine map).
const FORM_SET_FORM_TYPE: u32 = 0x004f_15a0;
/// `memset` wrapper (`cdecl`: destination, byte, count; `00403d30`).
const MEMSET: u32 = 0x0040_3d30;
/// `memcpy` wrapper (`cdecl`: destination, source, count; `00401460`).
const MEMCPY: u32 = 0x0040_1460;
/// `memcmp` (`cdecl`: first, second, count; `_memcmp`, `00ec4835`).
const MEMCMP: u32 = 0x00ec_4835;
/// Allocation (`cdecl`, size; `00401000`) and its counterpart, the
/// release called by the deleting destructor (`cdecl`, pointer; `00401030`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `TESForm` "initialized" bit setter (bit 3 of `iFormFlags`), `thiscall`
/// (form, value), and its test (`AL`), `thiscall`.
const FORM_SET_FLAG_8: u32 = 0x0048_4ab0;
const FORM_FLAG_8_TEST: u32 = 0x0040_13e0;
/// `TESFile::GetTESForm` (Xbox PDB): the form type of the record being read.
const FILE_GET_FORM_TYPE: u32 = 0x0047_2660;
/// `TESFile::GetTESChunk` (Xbox PDB): the current chunk's code, 0 at the end.
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
/// Moves to the next chunk (`AL`: true while there is one), `thiscall`.
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESFile::GetChunkData_ov3` (Xbox PDB): `thiscall` (file, buffer, size).
const FILE_GET_CHUNK_BYTES: u32 = 0x0047_2890;
/// The size of the current chunk, `thiscall` (file; `+0x25C`).
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// Whether the file's data are byte-swapped (`AL`, `thiscall`; `+0x299`).
const FILE_SWAPS: u32 = 0x0040_1680;
/// `TESForm::LoadForm` (Xbox PDB), `thiscall` (form, file).
const FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESForm::StartForm` and `TESForm::CloseForm` (Xbox PDB), `thiscall`.
const FORM_START_FORM: u32 = 0x0048_55a0;
const FORM_CLOSE_FORM: u32 = 0x0048_5680;
/// `TESForm::__AddChunkData` (Xbox PDB), `cdecl` (chunk, data, size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// Save-side byte-swap switch (`cdecl`, no arguments, `AL`).
const SWAP_ON_SAVE: u32 = 0x0040_1500;
/// Swaps the byte order of the word its `thiscall` receiver points at
/// (`00503210`, which calls `00401080`).
const SWAP_WORD: u32 = 0x0050_3210;
/// `iFormID` getter (`thiscall`; the engine map names the folded body
/// `0084e3a0`).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// `TESForm::GetFile` (Xbox PDB), `thiscall` (form, `-1`).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB), `cdecl` (pointer to the form id,
/// file).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// Form lookup by id (`cdecl`, one word).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// Logging function (`cdecl`, format and arguments).
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// `TESForm::CopyAllComponents` and `TESForm::CompareAllComponents` (Xbox
/// PDB), `thiscall` (form, other form).
const FORM_COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const FORM_COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Type descriptors of `TESForm` and `BGSEncounterZone`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_BGS_ENCOUNTER_ZONE: u32 = 0x0118_40dc;
/// The float setting getter (`thiscall` on a setting; pointer to its value).
const SETTING_FLOAT_POINTER: u32 = 0x0040_3e20;
/// The `u32` setting getter (`thiscall` on a setting; pointer to its value).
const SETTING_VALUE_POINTER: u32 = 0x0043_d4d0;
/// The player's level (`thiscall` on the player, `AX`).
const PLAYER_LEVEL: u32 = 0x0087_f9f0;
/// `TESObjectCELL::GetDetachTime` (Xbox PDB), `thiscall` on a cell.
const CELL_GET_DETACH_TIME: u32 = 0x0054_6af0;
/// The game time in whole units (`thiscall` on `GAME_TIME_OBJECT`; the
/// value of the object's global times 24, truncated; identity unconfirmed).
const GET_GAME_TIME: u32 = 0x0086_7e30;
/// `TESDataHandler::AddFormToDataHandler` (Xbox PDB), `thiscall` (handler,
/// form).
const ADD_FORM_TO_DATA_HANDLER: u32 = 0x0046_03b0;

/// Virtual slot `0x48` of a form (`AddChange` in the Xbox PDB), called with
/// `0x80000000` after the zone changes.
const FORM_ADD_CHANGE_SLOT: u32 = 0x48;
const CHANGE_FLAG_GAME_DATA: u32 = 0x8000_0000;
/// Virtual slot `0xE0` of a form (`LoadObjectBound` in the Xbox PDB).
const FORM_LOAD_OBJECT_BOUND_SLOT: u32 = 0xe0;
/// Virtual slot `0x128` (`TESForm::SetFormID`, `00485c10`), `thiscall`
/// (id, flag).
const FORM_SET_FORM_ID_SLOT: u32 = 0x128;
/// Virtual slot `0x130` of a form: its object type name.
const FORM_GET_OBJECT_TYPE_NAME_SLOT: u32 = 0x130;
/// Virtual slot `0x134` of a form: the editor id setter `Load` calls for the
/// `EDID` chunk (`TESForm`'s own version returns 1).
const FORM_SET_EDITOR_ID_SLOT: u32 = 0x134;

/// `"Unable to find owner form (%08X) on encounter zone '%s' (%08X)."`
const OWNER_NOT_FOUND_MESSAGE: u32 = 0x0102_ccf8;
/// `"NoZoneZone"`.
const NO_ZONE_EDITOR_ID: u32 = 0x0102_cd38;

// ---- Constructor, destructor, defaults ------------------------------------------------

// Translated from 00525c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEncounterZone::BGSEncounterZone` (Xbox PDB): constructs the `TESForm`
/// base, installs the encounter zone vtable, sets the form type to `0x61` and
/// clears the zone's data.
// Translation incomplete: 00525c40 - the C++ exception unwinding frame (FS:[0] handler chain that destroys the TESForm base if a later step throws) is not translated.
pub fn bgs_encounter_zone_bgs_encounter_zone(
    e: &mut Engine,
    this: Ptr<BGSEncounterZone>,
) -> Ptr<BGSEncounterZone> {
    e.call(FORM_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(FORM_SET_FORM_TYPE, &args![this, FORM_TYPE_ENCOUNTER_ZONE]);
    fn_00525d00(e, this);
    this
}

// Translated from 00525cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (vtable slot `0x10`, `__vecDelDtor` in the
/// Xbox PDB): runs the destructor body and, when bit 0 of `flags` is set,
/// frees the object. Returns `this`.
pub fn fn_00525cb0(
    e: &mut Engine,
    this: Ptr<BGSEncounterZone>,
    flags: u32,
) -> Ptr<BGSEncounterZone> {
    fn_00525ce0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00525ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body: puts the encounter zone vtable back and runs the
/// `TESForm` destructor.
pub fn fn_00525ce0(e: &mut Engine, this: Ptr<BGSEncounterZone>) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(FORM_DESTRUCTOR, &args![this]);
}

// Translated from 00525d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x14` (`InitializeData` in the Xbox PDB): zeroes the 8-byte
/// `Data` block and the 0x10-byte `GameData` block.
pub fn fn_00525d00(e: &mut Engine, this: Ptr<BGSEncounterZone>) {
    e.call(MEMSET, &args![this.addr() + DATA_OFFSET, 0u32, DATA_SIZE]);
    e.call(
        MEMSET,
        &args![this.addr() + GAME_DATA_OFFSET, 0u32, GAME_DATA_SIZE],
    );
}

// ---- Load, Save -----------------------------------------------------------------------

// Translated from 00525d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x20` (`Load` in the Xbox PDB): reads an encounter zone
/// record (form type `0x61`) from `file` and returns whether it did. Starts
/// with the form header (`TESForm::LoadForm`) and clears bit 3 of the form
/// flags, then loops over the chunks: `DATA` is read into the 8-byte data
/// block (its owner word is byte-swapped when the file is), `EDID` goes to the
/// virtual editor id setter (slot `0x134`), `OBND` to the object bound loader
/// (slot `0xE0`); other chunks are skipped.
///
/// Not translated: the stack-protector cookie of the compiled function. The
/// `alloca` buffer of the `EDID` chunk is a heap block here.
// Translation incomplete: 00525d40 - the compiler's security cookie check (`__security_check_cookie`, 00ec408c) is not translated.
pub fn fn_00525d40(e: &mut Engine, this: Ptr<BGSEncounterZone>, file: Ptr) -> bool {
    if e.call(FILE_GET_FORM_TYPE, &args![file]).u8() as u32 != FORM_TYPE_ENCOUNTER_ZONE {
        return false;
    }
    e.call(FORM_LOAD_FORM, &args![this, file]);
    e.call(FORM_SET_FLAG_8, &args![this, 0u32]);
    loop {
        let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_DATA => {
                e.call(
                    FILE_GET_CHUNK_BYTES,
                    &args![file, this.addr() + DATA_OFFSET, DATA_SIZE],
                );
                if e.call(FILE_SWAPS, &args![file]).bool() {
                    e.call(SWAP_WORD, &args![this.addr() + DATA_OFFSET]);
                }
            }
            CHUNK_EDID => {
                // The game takes a stack buffer as large as the chunk and reads
                // at most 0x200 bytes into it.
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                let buffer = e.mem.alloc(size.max(1));
                e.call(FILE_GET_CHUNK_BYTES, &args![file, buffer, 0x200u32]);
                e.vcall(this.addr(), FORM_SET_EDITOR_ID_SLOT, &args![buffer]);
                e.mem.free(buffer);
            }
            CHUNK_OBND => {
                e.vcall(this.addr(), FORM_LOAD_OBJECT_BOUND_SLOT, &args![file]);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 00525e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x2C` (`Save` in the Xbox PDB): writes the record. The
/// owner pointer is replaced by its form id while the 8-byte `DATA` chunk is
/// written (byte-swapped around the write when the save swaps), then restored.
pub fn fn_00525e70(e: &mut Engine, this: Ptr<BGSEncounterZone>) {
    e.call(FORM_START_FORM, &args![this]);
    let owner = e.get(this, BGSEncounterZone::pZoneOwner);
    if owner != 0 {
        let form_id = e.call(GET_FORM_ID, &args![owner]).u32();
        e.set(this, BGSEncounterZone::pZoneOwner, form_id);
    }
    if e.call(SWAP_ON_SAVE, &[]).bool() {
        e.call(SWAP_WORD, &args![this.addr() + DATA_OFFSET]);
    }
    e.call(
        ADD_CHUNK_DATA,
        &args![CHUNK_DATA, this.addr() + DATA_OFFSET, DATA_SIZE],
    );
    if e.call(SWAP_ON_SAVE, &[]).bool() {
        e.call(SWAP_WORD, &args![this.addr() + DATA_OFFSET]);
    }
    e.set(this, BGSEncounterZone::pZoneOwner, owner);
    e.call(FORM_CLOSE_FORM, &args![this]);
}

// ---- InitItem, Copy, Compare ----------------------------------------------------------

// Translated from 00525f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEncounterZone::InitItem` (Xbox PDB), vtable slot `0x88`: unless bit 3
/// of the form flags is set, turns the owner's form id into the form (moved
/// to the running game's load order with the zone's file); an owner that does
/// not resolve is logged and left as null. Then marks the form initialized.
pub fn bgs_encounter_zone_init_item(e: &mut Engine, this: Ptr<BGSEncounterZone>) {
    if e.call(FORM_FLAG_8_TEST, &args![this]).bool() {
        return;
    }
    let owner = e.get(this, BGSEncounterZone::pZoneOwner);
    if owner != 0 {
        let file = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
        let id = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), owner);
            e.call(FORM_ADD_COMPILE_INDEX, &args![slot, file]);
            e.mem.u32(slot.addr())
        });
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        if form == 0 {
            let form_id = e.call(GET_FORM_ID, &args![this]).u32();
            let name = e
                .vcall(this.addr(), FORM_GET_OBJECT_TYPE_NAME_SLOT, &[])
                .u32();
            e.call(
                LOG_MESSAGE,
                &args![OWNER_NOT_FOUND_MESSAGE, id, name, form_id],
            );
        }
        e.set(this, BGSEncounterZone::pZoneOwner, form);
    }
    e.call(FORM_SET_FLAG_8, &args![this, 1u32]);
}

/// `__RTDynamicCast` of `other` to `BGSEncounterZone`.
fn cast_to_encounter_zone(e: &mut Engine, other: u32) -> Ptr<BGSEncounterZone> {
    e.call(
        RT_DYNAMIC_CAST,
        &args![other, 0u32, TYPE_TES_FORM, TYPE_BGS_ENCOUNTER_ZONE, 0u32],
    )
    .ptr()
}

// Translated from 00525fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x108` (`Copy` in the Xbox PDB): when `other` is an encounter
/// zone, copies its components (`TESForm::CopyAllComponents`) and its 8-byte
/// `Data` block.
pub fn fn_00525fa0(e: &mut Engine, this: Ptr<BGSEncounterZone>, other: u32) {
    let zone = cast_to_encounter_zone(e, other);
    if zone.is_null() {
        return;
    }
    e.call(FORM_COPY_ALL_COMPONENTS, &args![this, zone]);
    e.call(
        MEMCPY,
        &args![
            this.addr() + DATA_OFFSET,
            zone.addr() + DATA_OFFSET,
            DATA_SIZE
        ],
    );
}

// Translated from 00526000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x10C` (`Compare` in the Xbox PDB): true when `other` is not
/// an encounter zone, or its components or its 8-byte `Data` block differ.
pub fn fn_00526000(e: &mut Engine, this: Ptr<BGSEncounterZone>, other: u32) -> bool {
    let zone = cast_to_encounter_zone(e, other);
    if zone.is_null() {
        return true;
    }
    if e.call(FORM_COMPARE_ALL_COMPONENTS, &args![this, zone])
        .bool()
    {
        return true;
    }
    e.call(
        MEMCMP,
        &args![
            this.addr() + DATA_OFFSET,
            zone.addr() + DATA_OFFSET,
            DATA_SIZE
        ],
    )
    .i32()
        != 0
}

// ---- Game data accessors --------------------------------------------------------------

// Translated from 00526070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `GameData.iDetachTime` and reports the change (`AddChange`,
/// virtual slot `0x48`, with `0x80000000`).
pub fn fn_00526070(e: &mut Engine, this: Ptr<BGSEncounterZone>, time: u32) {
    e.set(this, BGSEncounterZone::iDetachTime, time);
    e.vcall(
        this.addr(),
        FORM_ADD_CHANGE_SLOT,
        &args![CHANGE_FLAG_GAME_DATA],
    );
}

// Translated from 005260a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `GameData.iAttachTime`. When the zone has a detach time and the
/// attach time is later than it by more than the reset-time setting
/// (`fn_00526100`), `iResetTime` takes the attach time. Reports the change
/// (`AddChange`, slot `0x48`, `0x80000000`).
pub fn fn_005260a0(e: &mut Engine, this: Ptr<BGSEncounterZone>, time: u32) {
    e.set(this, BGSEncounterZone::iAttachTime, time);
    let detach = e.get(this, BGSEncounterZone::iDetachTime);
    if detach != 0 {
        let attach = e.get(this, BGSEncounterZone::iAttachTime);
        let elapsed = attach.wrapping_sub(detach);
        if elapsed > fn_00526100(e) {
            e.set(this, BGSEncounterZone::iResetTime, time);
        }
    }
    e.vcall(
        this.addr(),
        FORM_ADD_CHANGE_SLOT,
        &args![CHANGE_FLAG_GAME_DATA],
    );
}

// Translated from 00526100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `u32` value of the game setting at `011ca160` (the time the zone
/// needs to reset; the setting's name is not confirmed).
pub fn fn_00526100(e: &mut Engine) -> u32 {
    let value = e
        .call(SETTING_VALUE_POINTER, &args![SETTING_RESET_TIME])
        .u32();
    e.mem.u32(value)
}

// Translated from 00526120 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the zone has a reset time (`iResetTime != 0`) and `cell`'s
/// detach time (`TESObjectCELL::GetDetachTime`) is earlier than it.
pub fn fn_00526120(e: &mut Engine, this: Ptr<BGSEncounterZone>, cell: u32) -> bool {
    let reset = e.get(this, BGSEncounterZone::iResetTime);
    if reset == 0 {
        return false;
    }
    let detach = e.call(CELL_GET_DETACH_TIME, &args![cell]).u32();
    detach < reset
}

// Translated from 00526160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `GameData.sZoneLevel` and reports the change (`AddChange`, slot
/// `0x48`, `0x80000000`).
pub fn fn_00526160(e: &mut Engine, this: Ptr<BGSEncounterZone>, level: u16) {
    e.set(this, BGSEncounterZone::sZoneLevel, level);
    e.vcall(
        this.addr(),
        FORM_ADD_CHANGE_SLOT,
        &args![CHANGE_FLAG_GAME_DATA],
    );
}

// Translated from 00526190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The level the zone gives its creatures: the player's level, raised to
/// the zone's minimum level when the player is below it, unless bit 1 of
/// the flags is set (then the player's level is kept). A player at or above
/// the minimum gets
/// `minimum + (level - minimum) * setting(011c9550)`, truncated to 16 bits.
pub fn fn_00526190(e: &mut Engine, this: Ptr<BGSEncounterZone>) -> u16 {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let mut level = e.call(PLAYER_LEVEL, &args![player]).u16();
    let minimum = e.get(this, BGSEncounterZone::cMinLevel);
    if (level as i32) < minimum as i32 {
        if !fn_00526250(e, this) {
            level = e.get(this, BGSEncounterZone::cMinLevel) as u16;
        }
    } else {
        let minimum = e.get(this, BGSEncounterZone::cMinLevel);
        let difference = level as i32 - e.get(this, BGSEncounterZone::cMinLevel) as i32;
        let scale_address = e
            .call(SETTING_FLOAT_POINTER, &args![SETTING_LEVEL_SCALE])
            .u32();
        let scale = e.mem.f32(scale_address);
        // x87: the product and sum are computed in extended precision (here
        // `f64`) and truncated to an integer; the low 16 bits are returned.
        level = (scale as f64 * difference as f64 + minimum as f64) as i32 as u16;
    }
    level
}

// Translated from 00526250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 1 of the zone's flags (`Data.cFlags & 2`).
pub fn fn_00526250(e: &mut Engine, this: Ptr<BGSEncounterZone>) -> bool {
    e.get(this, BGSEncounterZone::cFlags) & 2 != 0
}

// Translated from 00526270 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the zone has no level yet (`sZoneLevel == 0`), sets it to the level
/// `fn_00526190` works out.
pub fn fn_00526270(e: &mut Engine, this: Ptr<BGSEncounterZone>) {
    if e.get(this, BGSEncounterZone::sZoneLevel) == 0 {
        let level = fn_00526190(e, this);
        fn_00526160(e, this, level);
    }
}

// Translated from 005262a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the zone is ready to be used again. True for the "no zone" zone
/// (`pNoZone`) and for a zone without a level; false when flag bit 0 is
/// set, or the zone was never detached (`iDetachTime == 0`), or the game time
/// has not passed the detach time by more than the reset-time setting
/// (`fn_00526100`).
pub fn fn_005262a0(e: &mut Engine, this: Ptr<BGSEncounterZone>) -> bool {
    let mut ready = true;
    if this.addr() != e.global::<u32>(NO_ZONE) && e.get(this, BGSEncounterZone::sZoneLevel) != 0 {
        if fn_00526320(e, this) {
            ready = false;
        } else {
            let detach = e.get(this, BGSEncounterZone::iDetachTime);
            if detach != 0 {
                let now = e.call(GET_GAME_TIME, &args![GAME_TIME_OBJECT]).u32();
                if now.wrapping_sub(detach) <= fn_00526100(e) {
                    ready = false;
                }
            } else {
                ready = false;
            }
        }
    }
    ready
}

// Translated from 00526320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0 of the zone's flags (`Data.cFlags & 1`).
pub fn fn_00526320(e: &mut Engine, this: Ptr<BGSEncounterZone>) -> bool {
    e.get(this, BGSEncounterZone::cFlags) & 1 != 0
}

// ---- CreateNoZone ---------------------------------------------------------------------

// Translated from 00526340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSEncounterZone::CreateNoZone` (Xbox PDB): once, makes the "no zone"
/// encounter zone: allocates and constructs a zone (0x30 bytes), stores it in
/// `pNoZone`, gives it form id `0x1e` (virtual slot `0x128`, called with the
/// id and 1) and the editor id `"NoZoneZone"` (slot `0x134`), and adds it to
/// the data handler.
// Translation incomplete: 00526340 - the C++ exception unwinding frame (FS:[0] handler chain; it frees the allocation if the constructor throws) is not translated.
pub fn bgs_encounter_zone_create_no_zone(e: &mut Engine) {
    if e.global::<u32>(NO_ZONE) != 0 {
        return;
    }
    let memory = e
        .call(OPERATOR_NEW, &args![BGSEncounterZone::SIZE])
        .ptr::<BGSEncounterZone>();
    let zone = if memory.is_null() {
        memory
    } else {
        bgs_encounter_zone_bgs_encounter_zone(e, memory)
    };
    e.set_global(NO_ZONE, zone.addr());
    let zone = e.global::<u32>(NO_ZONE);
    e.vcall(zone, FORM_SET_FORM_ID_SLOT, &args![NO_ZONE_FORM_ID, 1u32]);
    let zone = e.global::<u32>(NO_ZONE);
    e.vcall(zone, FORM_SET_EDITOR_ID_SLOT, &args![NO_ZONE_EDITOR_ID]);
    let zone = e.global::<u32>(NO_ZONE);
    let handler = e.global::<u32>(DATA_HANDLER);
    e.call(ADD_FORM_TO_DATA_HANDLER, &args![handler, zone]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00525c40,
            bgs_encounter_zone_bgs_encounter_zone(Ptr<BGSEncounterZone>) -> Ptr<BGSEncounterZone>
        ),
        entry!(
            0x00525cb0,
            fn_00525cb0(Ptr<BGSEncounterZone>, u32) -> Ptr<BGSEncounterZone>
        ),
        entry!(0x00525ce0, fn_00525ce0(Ptr<BGSEncounterZone>)),
        entry!(0x00525d00, fn_00525d00(Ptr<BGSEncounterZone>)),
        entry!(0x00525d40, fn_00525d40(Ptr<BGSEncounterZone>, Ptr) -> bool),
        entry!(0x00525e70, fn_00525e70(Ptr<BGSEncounterZone>)),
        entry!(
            0x00525f00,
            bgs_encounter_zone_init_item(Ptr<BGSEncounterZone>)
        ),
        entry!(0x00525fa0, fn_00525fa0(Ptr<BGSEncounterZone>, u32)),
        entry!(0x00526000, fn_00526000(Ptr<BGSEncounterZone>, u32) -> bool),
        entry!(0x00526070, fn_00526070(Ptr<BGSEncounterZone>, u32)),
        entry!(0x005260a0, fn_005260a0(Ptr<BGSEncounterZone>, u32)),
        entry!(0x00526100, fn_00526100() -> u32),
        entry!(0x00526120, fn_00526120(Ptr<BGSEncounterZone>, u32) -> bool),
        entry!(0x00526160, fn_00526160(Ptr<BGSEncounterZone>, u16)),
        entry!(0x00526190, fn_00526190(Ptr<BGSEncounterZone>) -> u16),
        entry!(0x00526250, fn_00526250(Ptr<BGSEncounterZone>) -> bool),
        entry!(0x00526270, fn_00526270(Ptr<BGSEncounterZone>)),
        entry!(0x005262a0, fn_005262a0(Ptr<BGSEncounterZone>) -> bool),
        entry!(0x00526320, fn_00526320(Ptr<BGSEncounterZone>) -> bool),
        entry!(0x00526340, bgs_encounter_zone_create_no_zone()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    fn zone(e: &mut Engine) -> Ptr<BGSEncounterZone> {
        e.new_object()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn returning(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    fn recording(e: &mut Engine, addr: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        log
    }

    /// A recording double that returns `value`.
    fn recording_returning(e: &mut Engine, addr: u32, value: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        log
    }

    fn give_vtable(e: &mut Engine, object: u32, slots: &[(u32, u32)]) {
        let vtable = e.mem.alloc(0x800);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        e.mem.set_u32(object, vtable);
    }

    fn map_pages(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.map(addr & !0xfff, 0x1000);
        }
    }

    /// Gives the zone a vtable whose `AddChange` slot is a recording double.
    fn with_add_change(e: &mut Engine, z: Ptr<BGSEncounterZone>) -> Log {
        give_vtable(e, z.addr(), &[(FORM_ADD_CHANGE_SLOT, 0x00dd_0048)]);
        recording(e, 0x00dd_0048)
    }

    #[test]
    fn constructor_sets_vtable_form_type_and_clears_data() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        quiet(&mut e, &[FORM_CONSTRUCTOR]);
        let types = recording(&mut e, FORM_SET_FORM_TYPE);
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.mem.set_u32(z.addr() + 0x18, 0xffff_ffff);
        e.mem.set_u32(z.addr() + 0x2c, 0xffff_ffff);
        let result = e.call(0x0052_5c40, &args![z]).u32();
        assert_eq!(result, z.addr());
        assert_eq!(e.mem.u32(z.addr()), VTABLE);
        assert_eq!(types.borrow().as_slice(), &[vec![z.addr(), 0x61]]);
        assert_eq!(e.mem.u32(z.addr() + 0x18), 0);
        assert_eq!(e.mem.u32(z.addr() + 0x1c), 0);
        assert_eq!(e.mem.u32(z.addr() + 0x20), 0);
        assert_eq!(e.mem.u32(z.addr() + 0x2c), 0);
    }

    #[test]
    fn deleting_destructor_frees_only_when_asked() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        let base = recording(&mut e, FORM_DESTRUCTOR);
        let free = recording(&mut e, OPERATOR_DELETE);
        let result = e.call(0x0052_5cb0, &args![z, 0u32]).u32();
        assert_eq!(result, z.addr());
        assert_eq!(e.mem.u32(z.addr()), VTABLE);
        assert_eq!(base.borrow().as_slice(), &[vec![z.addr()]]);
        assert!(free.borrow().is_empty());
        e.call(0x0052_5cb0, &args![z, 1u32]);
        assert_eq!(free.borrow().as_slice(), &[vec![z.addr()]]);
        // Bits other than bit 0 do not free.
        e.call(0x0052_5cb0, &args![z, 2u32]);
        assert_eq!(free.borrow().len(), 1);
    }

    #[test]
    fn destructor_body_restores_vtable_and_calls_the_form_destructor() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        let base = recording(&mut e, FORM_DESTRUCTOR);
        e.mem.set_u32(z.addr(), 0x1234);
        e.call(0x0052_5ce0, &args![z]);
        assert_eq!(e.mem.u32(z.addr()), VTABLE);
        assert_eq!(base.borrow().as_slice(), &[vec![z.addr()]]);
    }

    #[test]
    fn initialize_data_zeroes_both_blocks() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        let calls = recording(&mut e, MEMSET);
        e.call(0x0052_5d00, &args![z]);
        assert_eq!(
            calls.borrow().as_slice(),
            &[vec![z.addr() + 0x18, 0, 8], vec![z.addr() + 0x20, 0, 0x10]]
        );
    }

    /// Doubles for the file functions `Load` calls: a record of the given
    /// form type with the given chunks (code, bytes).
    fn load_doubles(e: &mut Engine, form_type: u32, swaps: bool, chunks: Vec<(u32, Vec<u8>)>) {
        returning(e, FILE_GET_FORM_TYPE, form_type);
        quiet(e, &[FORM_LOAD_FORM, FORM_SET_FLAG_8]);
        returning(e, FILE_SWAPS, swaps as u32);
        let position = Rc::new(RefCell::new(0usize));
        let chunks = Rc::new(chunks);
        let (p, c) = (position.clone(), chunks.clone());
        e.register_double(FILE_GET_CHUNK, move |_, _| Ret {
            eax: c.get(*p.borrow()).map(|chunk| chunk.0).unwrap_or(0),
            ..Ret::default()
        });
        let (p, c) = (position.clone(), chunks.clone());
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            *p.borrow_mut() += 1;
            Ret {
                eax: (*p.borrow() < c.len()) as u32,
                ..Ret::default()
            }
        });
        let (p, c) = (position.clone(), chunks.clone());
        e.register_double(FILE_CHUNK_SIZE, move |_, _| Ret {
            eax: c[*p.borrow()].1.len() as u32,
            ..Ret::default()
        });
        let (p, c) = (position, chunks);
        e.register_double(FILE_GET_CHUNK_BYTES, move |e, a| {
            let bytes = &c[*p.borrow()].1;
            let count = (a[2] as usize).min(bytes.len());
            for (i, byte) in bytes.iter().take(count).enumerate() {
                e.mem.set_u8(a[1] + i as u32, *byte);
            }
            Ret::default()
        });
    }

    #[test]
    fn load_rejects_other_record_types() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        load_doubles(&mut e, 0x60, false, vec![]);
        let load_form = recording(&mut e, FORM_LOAD_FORM);
        assert!(!e.call(0x0052_5d40, &args![z, 0x5000u32]).bool());
        assert!(load_form.borrow().is_empty());
    }

    #[test]
    fn load_reads_data_edid_and_bound_chunks() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        let editor_id = b"MyZone\0".to_vec();
        load_doubles(
            &mut e,
            0x61,
            true,
            vec![
                (CHUNK_DATA, vec![1, 2, 3, 4, 5, 6, 7, 8]),
                (CHUNK_EDID, editor_id),
                (CHUNK_OBND, vec![]),
                (tag(b"XXXX"), vec![9]),
            ],
        );
        let flags = recording(&mut e, FORM_SET_FLAG_8);
        let swapped = recording(&mut e, SWAP_WORD);
        give_vtable(
            &mut e,
            z.addr(),
            &[
                (FORM_SET_EDITOR_ID_SLOT, 0x00dd_0134),
                (FORM_LOAD_OBJECT_BOUND_SLOT, 0x00dd_00e0),
            ],
        );
        let names = Rc::new(RefCell::new(Vec::<Vec<u8>>::new()));
        let sink = names.clone();
        e.register_double(0x00dd_0134, move |e, a| {
            let mut text = Vec::new();
            let mut at = a[1];
            while e.mem.u8(at) != 0 {
                text.push(e.mem.u8(at));
                at += 1;
            }
            sink.borrow_mut().push(text);
            Ret::default()
        });
        let bound = recording(&mut e, 0x00dd_00e0);
        assert!(e.call(0x0052_5d40, &args![z, 0x5000u32]).bool());
        assert_eq!(e.mem.u32(z.addr() + 0x18), 0x0403_0201);
        assert_eq!(e.mem.u32(z.addr() + 0x1c), 0x0807_0605);
        // The file swaps, so the owner word is swapped once.
        assert_eq!(swapped.borrow().as_slice(), &[vec![z.addr() + 0x18]]);
        assert_eq!(names.borrow().as_slice(), &[b"MyZone".to_vec()]);
        assert_eq!(bound.borrow().as_slice(), &[vec![z.addr(), 0x5000]]);
        // The "initialized" bit is cleared before the chunks.
        assert_eq!(flags.borrow().as_slice(), &[vec![z.addr(), 0]]);
    }

    #[test]
    fn load_does_not_swap_when_the_file_does_not() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        load_doubles(&mut e, 0x61, false, vec![(CHUNK_DATA, vec![0; 8])]);
        let swapped = recording(&mut e, SWAP_WORD);
        assert!(e.call(0x0052_5d40, &args![z, 0x5000u32]).bool());
        assert!(swapped.borrow().is_empty());
    }

    #[test]
    fn load_of_a_record_without_chunks_succeeds() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        load_doubles(&mut e, 0x61, false, vec![]);
        assert!(e.call(0x0052_5d40, &args![z, 0x5000u32]).bool());
    }

    #[test]
    fn save_writes_the_owner_id_and_restores_the_pointer() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        quiet(&mut e, &[FORM_START_FORM, FORM_CLOSE_FORM]);
        e.register(GET_FORM_ID, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        returning(&mut e, SWAP_ON_SAVE, 1);
        let swaps = recording(&mut e, SWAP_WORD);
        let seen = Rc::new(RefCell::new(Vec::<(u32, u32, u32, u32)>::new()));
        let sink = seen.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            sink.borrow_mut().push((a[0], a[1], a[2], e.mem.u32(a[1])));
            Ret::default()
        });
        e.mem.set_u32(z.addr() + 0x18, 0x7000);
        e.call(0x0052_5e70, &args![z]);
        // While the chunk is written the word holds the owner's form id.
        assert_eq!(
            seen.borrow().as_slice(),
            &[(CHUNK_DATA, z.addr() + 0x18, 8, 0x7100)]
        );
        assert_eq!(e.mem.u32(z.addr() + 0x18), 0x7000);
        assert_eq!(swaps.borrow().len(), 2);
    }

    #[test]
    fn save_without_owner_or_swap() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        quiet(&mut e, &[FORM_START_FORM, FORM_CLOSE_FORM]);
        returning(&mut e, SWAP_ON_SAVE, 0);
        let ids = recording(&mut e, GET_FORM_ID);
        let swaps = recording(&mut e, SWAP_WORD);
        let chunks = recording(&mut e, ADD_CHUNK_DATA);
        e.call(0x0052_5e70, &args![z]);
        assert!(ids.borrow().is_empty());
        assert!(swaps.borrow().is_empty());
        assert_eq!(chunks.borrow().len(), 1);
    }

    /// Doubles for `InitItem`.
    fn init_item_doubles(e: &mut Engine, z: Ptr<BGSEncounterZone>) {
        returning(e, FORM_FLAG_8_TEST, 0);
        returning(e, FORM_GET_FILE, 0x7000);
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| Ret {
            eax: if a[0] & 0xff == 0xff {
                0
            } else {
                a[0] + 0x1000
            },
            ..Ret::default()
        });
        e.register(GET_FORM_ID, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        give_vtable(
            e,
            z.addr(),
            &[(FORM_GET_OBJECT_TYPE_NAME_SLOT, 0x00dd_0130)],
        );
        returning(e, 0x00dd_0130, 0xbeef);
    }

    #[test]
    fn init_item_does_nothing_when_flag_eight_is_set() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        returning(&mut e, FORM_FLAG_8_TEST, 1);
        let flags = recording(&mut e, FORM_SET_FLAG_8);
        e.mem.set_u32(z.addr() + 0x18, 5);
        e.call(0x0052_5f00, &args![z]);
        assert_eq!(e.mem.u32(z.addr() + 0x18), 5);
        assert!(flags.borrow().is_empty());
    }

    #[test]
    fn init_item_resolves_the_owner() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        init_item_doubles(&mut e, z);
        let flags = recording(&mut e, FORM_SET_FLAG_8);
        let log = recording(&mut e, LOG_MESSAGE);
        e.mem.set_u32(z.addr() + 0x18, 5);
        e.call(0x0052_5f00, &args![z]);
        assert_eq!(e.mem.u32(z.addr() + 0x18), 0x0100_0005 + 0x1000);
        assert!(log.borrow().is_empty());
        assert_eq!(flags.borrow().as_slice(), &[vec![z.addr(), 1]]);
    }

    #[test]
    fn init_item_logs_an_owner_that_does_not_resolve() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        init_item_doubles(&mut e, z);
        let log = recording(&mut e, LOG_MESSAGE);
        e.mem.set_u32(z.addr() + 0xc, 0x1234);
        e.mem.set_u32(z.addr() + 0x18, 0xff);
        e.call(0x0052_5f00, &args![z]);
        assert_eq!(e.mem.u32(z.addr() + 0x18), 0);
        assert_eq!(
            log.borrow().as_slice(),
            &[vec![OWNER_NOT_FOUND_MESSAGE, 0x0100_00ff, 0xbeef, 0x1234]]
        );
    }

    #[test]
    fn init_item_without_owner_only_marks_the_form() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        init_item_doubles(&mut e, z);
        let lookups = recording(&mut e, LOOKUP_FORM);
        let flags = recording(&mut e, FORM_SET_FLAG_8);
        e.call(0x0052_5f00, &args![z]);
        assert!(lookups.borrow().is_empty());
        assert_eq!(flags.borrow().len(), 1);
    }

    fn cast_double(e: &mut Engine, result: u32) -> Log {
        recording_returning(e, RT_DYNAMIC_CAST, result)
    }

    #[test]
    fn copy_copies_components_and_data_of_an_encounter_zone() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        let other = zone(&mut e);
        let cast = cast_double(&mut e, other.addr());
        let components = recording(&mut e, FORM_COPY_ALL_COMPONENTS);
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        for i in 0..8 {
            e.mem.set_u8(other.addr() + 0x18 + i, 0x10 + i as u8);
        }
        e.call(0x0052_5fa0, &args![z, 0x9000u32]);
        assert_eq!(
            cast.borrow().as_slice(),
            &[vec![0x9000, 0, TYPE_TES_FORM, TYPE_BGS_ENCOUNTER_ZONE, 0]]
        );
        assert_eq!(
            components.borrow().as_slice(),
            &[vec![z.addr(), other.addr()]]
        );
        assert_eq!(e.mem.u8(z.addr() + 0x18), 0x10);
        assert_eq!(e.mem.u8(z.addr() + 0x1f), 0x17);
    }

    #[test]
    fn copy_ignores_other_forms() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        cast_double(&mut e, 0);
        let components = recording(&mut e, FORM_COPY_ALL_COMPONENTS);
        let copy = recording(&mut e, MEMCPY);
        e.call(0x0052_5fa0, &args![z, 0x9000u32]);
        assert!(components.borrow().is_empty());
        assert!(copy.borrow().is_empty());
    }

    #[test]
    fn compare_reports_other_types_components_and_data() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        let other = zone(&mut e);
        // Not an encounter zone: different.
        cast_double(&mut e, 0);
        assert!(e.call(0x0052_6000, &args![z, 0x9000u32]).bool());
        // Components differ: different, data not compared.
        cast_double(&mut e, other.addr());
        returning(&mut e, FORM_COMPARE_ALL_COMPONENTS, 1);
        let memcmp = recording(&mut e, MEMCMP);
        assert!(e.call(0x0052_6000, &args![z, 0x9000u32]).bool());
        assert!(memcmp.borrow().is_empty());
        // Same components: the data block decides.
        returning(&mut e, FORM_COMPARE_ALL_COMPONENTS, 0);
        returning(&mut e, MEMCMP, 0);
        assert!(!e.call(0x0052_6000, &args![z, 0x9000u32]).bool());
        returning(&mut e, MEMCMP, 0xffff_ffff);
        assert!(e.call(0x0052_6000, &args![z, 0x9000u32]).bool());
    }

    #[test]
    fn setters_store_and_report_the_change() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        let changes = with_add_change(&mut e, z);
        e.call(0x0052_6070, &args![z, 1234u32]);
        assert_eq!(e.mem.u32(z.addr() + 0x20), 1234);
        e.call(0x0052_6160, &args![z, 7u16]);
        assert_eq!(e.mem.u16(z.addr() + 0x2c), 7);
        assert_eq!(
            changes.borrow().as_slice(),
            &[vec![z.addr(), 0x8000_0000], vec![z.addr(), 0x8000_0000]]
        );
    }

    /// The reset-time setting reads 100.
    fn reset_time_setting(e: &mut Engine, value: u32) {
        map_pages(e, &[0x6000]);
        e.mem.set_u32(0x6000, value);
        returning(e, SETTING_VALUE_POINTER, 0x6000);
    }

    #[test]
    fn setting_the_attach_time_sets_the_reset_time_after_a_long_absence() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        let changes = with_add_change(&mut e, z);
        reset_time_setting(&mut e, 100);
        // No detach time: only the attach time is stored.
        e.call(0x0052_60a0, &args![z, 500u32]);
        assert_eq!(e.mem.u32(z.addr() + 0x24), 500);
        assert_eq!(e.mem.u32(z.addr() + 0x28), 0);
        // Detached 50 ago: not long enough.
        e.mem.set_u32(z.addr() + 0x20, 450);
        e.call(0x0052_60a0, &args![z, 500u32]);
        assert_eq!(e.mem.u32(z.addr() + 0x28), 0);
        // Exactly the setting: still not more than it.
        e.mem.set_u32(z.addr() + 0x20, 400);
        e.call(0x0052_60a0, &args![z, 500u32]);
        assert_eq!(e.mem.u32(z.addr() + 0x28), 0);
        // More than the setting: the reset time is set.
        e.mem.set_u32(z.addr() + 0x20, 399);
        e.call(0x0052_60a0, &args![z, 500u32]);
        assert_eq!(e.mem.u32(z.addr() + 0x28), 500);
        assert_eq!(changes.borrow().len(), 4);
    }

    #[test]
    fn reset_time_setting_is_read_through_its_pointer() {
        let mut e = Engine::new();
        let calls = {
            let seen = Rc::new(RefCell::new(Vec::<u32>::new()));
            let sink = seen.clone();
            map_pages(&mut e, &[0x6000]);
            e.mem.set_u32(0x6000, 77);
            e.register_double(SETTING_VALUE_POINTER, move |_, a| {
                sink.borrow_mut().push(a[0]);
                Ret {
                    eax: 0x6000,
                    ..Ret::default()
                }
            });
            seen
        };
        assert_eq!(e.call(0x0052_6100, &[]).u32(), 77);
        assert_eq!(calls.borrow().as_slice(), &[SETTING_RESET_TIME]);
    }

    #[test]
    fn reset_check_compares_the_cell_detach_time() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        returning(&mut e, CELL_GET_DETACH_TIME, 10);
        // No reset time: false without asking the cell.
        let cell = recording_returning(&mut e, CELL_GET_DETACH_TIME, 10);
        assert!(!e.call(0x0052_6120, &args![z, 0x8000u32]).bool());
        assert!(cell.borrow().is_empty());
        e.mem.set_u32(z.addr() + 0x28, 11);
        assert!(e.call(0x0052_6120, &args![z, 0x8000u32]).bool());
        assert_eq!(cell.borrow().as_slice(), &[vec![0x8000]]);
        e.mem.set_u32(z.addr() + 0x28, 10);
        assert!(!e.call(0x0052_6120, &args![z, 0x8000u32]).bool());
    }

    #[test]
    fn flag_tests_read_bits_zero_and_one() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        e.mem.set_u8(z.addr() + 0x1e, 1);
        assert!(e.call(0x0052_6320, &args![z]).bool());
        assert!(!e.call(0x0052_6250, &args![z]).bool());
        e.mem.set_u8(z.addr() + 0x1e, 2);
        assert!(!e.call(0x0052_6320, &args![z]).bool());
        assert!(e.call(0x0052_6250, &args![z]).bool());
        // The flags are a signed byte: 0x80 has neither bit.
        e.mem.set_u8(z.addr() + 0x1e, 0x80);
        assert!(!e.call(0x0052_6320, &args![z]).bool());
        assert!(!e.call(0x0052_6250, &args![z]).bool());
    }

    /// Doubles for the zone level: the player is at `player_level`, the
    /// level scale setting is `scale`.
    fn level_doubles(e: &mut Engine, player_level: u16, scale: f32) {
        map_pages(e, &[0x6000, PLAYER_CHARACTER]);
        e.set_global(PLAYER_CHARACTER, 0x4444u32);
        returning(e, PLAYER_LEVEL, player_level as u32);
        e.mem.set_f32(0x6004, scale);
        returning(e, SETTING_FLOAT_POINTER, 0x6004);
    }

    #[test]
    fn zone_level_below_the_minimum_is_raised_to_it() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        level_doubles(&mut e, 3, 0.5);
        e.mem.set_u8(z.addr() + 0x1d, 10);
        assert_eq!(e.call(0x0052_6190, &args![z]).u16(), 10);
        // With flag bit 1 the player's level stays.
        e.mem.set_u8(z.addr() + 0x1e, 2);
        assert_eq!(e.call(0x0052_6190, &args![z]).u16(), 3);
    }

    #[test]
    fn zone_level_above_the_minimum_is_scaled() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        level_doubles(&mut e, 20, 0.5);
        e.mem.set_u8(z.addr() + 0x1d, 10);
        // 10 + (20 - 10) * 0.5
        assert_eq!(e.call(0x0052_6190, &args![z]).u16(), 15);
        // Equal to the minimum: the minimum.
        level_doubles(&mut e, 10, 0.5);
        assert_eq!(e.call(0x0052_6190, &args![z]).u16(), 10);
        // The result is truncated: 10 + 5 * 0.5 = 12.5 -> 12.
        level_doubles(&mut e, 15, 0.5);
        assert_eq!(e.call(0x0052_6190, &args![z]).u16(), 12);
    }

    #[test]
    fn zone_level_is_set_only_once() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        let changes = with_add_change(&mut e, z);
        level_doubles(&mut e, 20, 1.0);
        e.mem.set_u8(z.addr() + 0x1d, 10);
        e.call(0x0052_6270, &args![z]);
        assert_eq!(e.mem.u16(z.addr() + 0x2c), 20);
        assert_eq!(changes.borrow().len(), 1);
        // Already set: nothing changes.
        level_doubles(&mut e, 30, 1.0);
        e.call(0x0052_6270, &args![z]);
        assert_eq!(e.mem.u16(z.addr() + 0x2c), 20);
        assert_eq!(changes.borrow().len(), 1);
    }

    /// Doubles for the readiness check: game time `now`, reset setting 100.
    fn ready_doubles(e: &mut Engine, now: u32) {
        map_pages(e, &[NO_ZONE]);
        e.set_global(NO_ZONE, 0u32);
        reset_time_setting(e, 100);
        returning(e, GET_GAME_TIME, now);
    }

    #[test]
    fn readiness_of_the_no_zone_zone_and_of_a_zone_without_level() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        ready_doubles(&mut e, 0);
        e.set_global(NO_ZONE, z.addr());
        e.mem.set_u16(z.addr() + 0x2c, 5);
        e.mem.set_u8(z.addr() + 0x1e, 1);
        assert!(e.call(0x0052_62a0, &args![z]).bool());
        // A zone without a level is always ready.
        e.set_global(NO_ZONE, 0u32);
        e.mem.set_u16(z.addr() + 0x2c, 0);
        assert!(e.call(0x0052_62a0, &args![z]).bool());
    }

    #[test]
    fn readiness_depends_on_the_flag_the_detach_time_and_the_game_time() {
        let mut e = Engine::new();
        let z = zone(&mut e);
        ready_doubles(&mut e, 1000);
        e.mem.set_u16(z.addr() + 0x2c, 5);
        // Flag bit 0: never ready.
        e.mem.set_u8(z.addr() + 0x1e, 1);
        e.mem.set_u32(z.addr() + 0x20, 1);
        assert!(!e.call(0x0052_62a0, &args![z]).bool());
        e.mem.set_u8(z.addr() + 0x1e, 0);
        // Never detached: not ready.
        e.mem.set_u32(z.addr() + 0x20, 0);
        assert!(!e.call(0x0052_62a0, &args![z]).bool());
        // Detached 100 ago: not more than the setting.
        e.mem.set_u32(z.addr() + 0x20, 900);
        assert!(!e.call(0x0052_62a0, &args![z]).bool());
        // Detached 101 ago.
        e.mem.set_u32(z.addr() + 0x20, 899);
        assert!(e.call(0x0052_62a0, &args![z]).bool());
    }

    #[test]
    fn create_no_zone_makes_the_zone_once() {
        let mut e = Engine::new();
        map_pages(&mut e, &[NO_ZONE, DATA_HANDLER, VTABLE]);
        e.set_global(NO_ZONE, 0u32);
        e.set_global(DATA_HANDLER, 0x7777u32);
        let memory = e.mem.alloc(0x30);
        let sizes = recording_returning(&mut e, OPERATOR_NEW, memory);
        quiet(&mut e, &[FORM_CONSTRUCTOR, FORM_SET_FORM_TYPE, MEMSET]);
        let handler = recording(&mut e, ADD_FORM_TO_DATA_HANDLER);
        let ids = recording(&mut e, 0x00dd_0128);
        let editor = recording(&mut e, 0x00dd_0134);
        // The vtable the constructor installs, with the two slots used.
        e.mem.set_u32(VTABLE + FORM_SET_FORM_ID_SLOT, 0x00dd_0128);
        e.mem.set_u32(VTABLE + FORM_SET_EDITOR_ID_SLOT, 0x00dd_0134);
        e.call(0x0052_6340, &[]);
        assert_eq!(sizes.borrow().as_slice(), &[vec![0x30]]);
        assert_eq!(e.global::<u32>(NO_ZONE), memory);
        assert_eq!(e.mem.u32(memory), VTABLE);
        assert_eq!(ids.borrow().as_slice(), &[vec![memory, 0x1e, 1]]);
        assert_eq!(
            editor.borrow().as_slice(),
            &[vec![memory, NO_ZONE_EDITOR_ID]]
        );
        assert_eq!(handler.borrow().as_slice(), &[vec![0x7777, memory]]);
        // A second call does nothing.
        e.call(0x0052_6340, &[]);
        assert_eq!(sizes.borrow().len(), 1);
    }
}
