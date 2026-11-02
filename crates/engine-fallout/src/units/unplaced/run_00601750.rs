//! `(unplaced)/run_00601750` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These functions sit inside `fallout shared/tesnpc.cpp` (between its
//! constructor `00601170` and `TESNPC::Load` `00602190`) but the Xbox PDB
//! names no unit for them. They are `TESNPC` members: the layout is the one
//! of `units/fallout_shared/tesnpc.rs` (PC offsets), reused here.
//!
//! Notes for the next session:
//! - Translated in order: `00601750` `ClearData`, `00601800` face-gen
//!   coordinate pointer, `00601830` face randomizer, `00601c10`/`00602130`
//!   hair length setter/getter, `00601c30` `TESModel::GetModel` override,
//!   `00601c70` class setter, `00601ca0` `Save`. This unit's last function
//!   is `00602130`.
//! - `TESNPC::Save` writes its chunks with the `TESForm` chunk writers
//!   (`00485910` one 32-bit value, `00485950` one 16-bit value, `00485990`
//!   a block of bytes, `00485710` an array of 32-bit words); the chunk names
//!   are four ASCII letters read as a little-endian word.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesnpc::{
    COMPONENT_DESTRUCTIBLE, COMPONENT_FULL_NAME, COMPONENT_MODEL, COMPONENT_RACE,
    COMPONENT_SPELL_LIST, COMPONENT_TOUCH_SPELL, FACEGEN_COORDS_DIFFER, FACEGEN_COPY_COORD,
    FACEGEN_INIT_COORD, GET_SEX, GET_WORD_AT_4, NPC_GET_EYE_COLOR, NPC_GET_HEAD_PARTS,
    SAVE_LOAD_GAME, SAVE_LOAD_UNAVAILABLE, TESNPC, VECTOR_CONSTRUCTOR_ITERATOR,
    VSLOT_GET_COMBAT_STYLE,
};

/// `NiPointer<T>::operator=(T *)` (`0066b0d0`, thiscall, returns the
/// pointer it was called on) and the `NiPointer` assignment from another
/// `NiPointer` (`006e5cc0`).
const NI_POINTER_ASSIGN_RAW: u32 = 0x0066_b0d0;
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// Clears a `BSSimpleList` (`00470470`, thiscall).
const SIMPLE_LIST_CLEAR: u32 = 0x0047_0470;
/// Releases an object with the deleting destructor (`005d9ff0`, thiscall,
/// one flags word on the stack).
const DELETE_OBJECT: u32 = 0x005d_9ff0;
/// `FR2MatrixVTC` constructor and destructor (`00449610` / `00449680`) and
/// `_eh_vector_destructor_iterator_(array, size, count, destructor)`.
const MATRIX_CONSTRUCTOR: u32 = 0x0044_9610;
const MATRIX_DESTRUCTOR: u32 = 0x0044_9680;
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// `GetCoordinate(coord, row, column)` (`00652440`, cdecl) and
/// `SetCoordinate(coord, row, column, value)` (`00652470`, cdecl) on a
/// face-gen coordinate; the value is a `float` in `ST0` / on the stack.
const FACEGEN_GET_VALUE: u32 = 0x0065_2440;
const FACEGEN_SET_VALUE: u32 = 0x0065_2470;
/// `(scale, raceCoord, coord)` cdecl: scales the race coordinate into `coord`.
const FACEGEN_SCALE_RACE_COORD: u32 = 0x0065_2fc0;
/// `(raceCoord, coord, destination, flag)` cdecl: blends them.
const FACEGEN_BLEND_RACE_COORD: u32 = 0x0065_2e00;
/// `TESRace` face-gen coordinate of a sex (`005d9fb0`, thiscall, one word).
const RACE_FACE_COORD: u32 = 0x005d_9fb0;
/// `00403e20` on the static at `011cb7a0`: pointer to a `float` setting.
const SCALE_SETTING_GETTER: u32 = 0x0040_3e20;
const SCALE_SETTING: u32 = 0x011c_b7a0;
/// Random number in `[0, 1)` (`005c5420`, result in `ST0`).
const RANDOM_FRACTION: u32 = 0x005c_5420;
/// `max(a, b)` (`00404010`) and `min(a, b)` (`0040ebd0`) on `float`s,
/// cdecl, result in `ST0`.
const FLOAT_MAX: u32 = 0x0040_4010;
const FLOAT_MIN: u32 = 0x0040_ebd0;
/// Constants of the randomizer: limits of the first row (`float`s) and of
/// the second (`float`s), then the scale and offset the random fraction is
/// given (`double`s).
const ROW0_LOW: u32 = 0x0101_e580;
const ROW0_HIGH: u32 = 0x0102_243c;
const ROW1_LOW: u32 = 0x0104_a494;
const ROW1_HIGH: u32 = 0x0101_62c0;
const ROW0_RANDOM_SCALE: u32 = 0x0101_e2c0;
const ROW0_RANDOM_OFFSET: u32 = 0x0101_5a38;
const ROW1_RANDOM_SCALE: u32 = 0x0101_db80;
const ROW1_RANDOM_OFFSET: u32 = 0x0101_1590;
/// The form id of a form (`0084e3a0`, `+0xC`), and an empty-list test
/// (`008256d0`: both words of the node are zero).
const GET_FORM_ID: u32 = 0x0084_e3a0;
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// Address of the item of a list node (`006815c0`: the node itself).
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
/// The hair of the NPC (`00603b50`, `TESNPC` unit).
const NPC_GET_HAIR: u32 = 0x0060_3b50;
/// `TESForm` chunk writers (cdecl).
const ADD_CHUNK_VALUE: u32 = 0x0048_5910;
const ADD_CHUNK_SHORT: u32 = 0x0048_5950;
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
const ADD_CHUNK_ARRAY: u32 = 0x0048_5710;
/// `TESForm::StartForm` / `SaveData` / `CloseForm` (thiscall).
const START_FORM: u32 = 0x0048_55a0;
const SAVE_DATA: u32 = 0x0048_5e00;
const CLOSE_FORM: u32 = 0x0048_5680;
/// Component savers (thiscall on the component).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const MODEL_SAVE: u32 = 0x0048_91d0;
const ACTOR_BASE_DATA_SAVE: u32 = 0x0047_cac0;
const RACE_FORM_SAVE: u32 = 0x0048_b830;
const SPELL_LIST_SAVE: u32 = 0x0048_d330;
const TOUCH_SPELL_SAVE: u32 = 0x0047_bd70;
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
/// `bEndianSwapOnSave` test (`00401500`) and the byte-swap of a block
/// (`00483710`, does nothing on PC).
const NEEDS_ENDIAN_SWAP: u32 = 0x0040_1500;
const ENDIAN_SWAP_BLOCK: u32 = 0x0048_3710;
/// The game's `malloc` (`00401000`), `free` (`00401030`) and `memset`
/// (`00403d30`, cdecl: destination, value, size).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
const MEMSET: u32 = 0x0040_3d30;
/// `FR2MatrixVTC` accessors for one element: iterator constructor
/// (`0060b370(matrix, out, index)`), element count (`009611e0`) and
/// element pointer (`00629ab0(iterator, 0)`).
const MATRIX_ELEMENT_COUNT: u32 = 0x0096_11e0;
const MATRIX_ELEMENT_ITERATOR: u32 = 0x0060_b370;
const MATRIX_ELEMENT_POINTER: u32 = 0x0062_9ab0;

/// `TESForm::AddChange` virtual (slot `0x48`).
const VSLOT_ADD_CHANGE: u32 = 0x48;
/// Change flag of the NPC's class.
const CHANGE_CLASS: u32 = 0x400;

// Translated from 00601750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::ClearData` (virtual slot `0x18` of `TESForm`, Xbox PDB name):
/// unless the save-load object says otherwise, the skinned head becomes the
/// biped head (`NiPointer` assignment) and the body-modification texture is
/// released; the last race face number goes back to `0xff`, the head-part
/// list is emptied and the alternate face-gen coordinate is deleted.
pub fn tesnpc_clear_data(e: &mut Engine, this: Ptr<TESNPC>) {
    let base = this.addr();
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if !e.call(SAVE_LOAD_UNAVAILABLE, &args![save_load]).bool() {
        let head = e
            .call(NI_POINTER_ASSIGN_RAW, &args![base + 0x1c4, 0u32])
            .u32();
        e.call(NI_POINTER_ASSIGN, &args![base + 0x1c8, head]);
        e.call(NI_POINTER_ASSIGN_RAW, &args![base + 0x1cc, 0u32]);
    }
    e.set(this, TESNPC::sLastRaceFaceNum, 0xff);
    e.call(SIMPLE_LIST_CLEAR, &args![base + 0x1dc]);
    let alternate = e.get(this, TESNPC::pAlternateFaceOffsetCoord);
    if !alternate.is_null() {
        e.call(DELETE_OBJECT, &args![alternate, 3u32]);
    }
    e.set(this, TESNPC::pAlternateFaceOffsetCoord, Ptr::NULL);
}

// Translated from 00601800 (decompiled, FalloutNV.exe 1.4.0.525)
/// The face-gen coordinate of an NPC: `pAlternateFaceOffsetCoord` when it
/// is set, otherwise the `RaceFaceOffsetCoord` embedded at `+0x134`.
pub fn fn_00601800(e: &mut Engine, this: Ptr<TESNPC>) -> Ptr {
    let alternate = e.get(this, TESNPC::pAlternateFaceOffsetCoord);
    if alternate.is_null() {
        this.byte_add(0x134)
    } else {
        alternate
    }
}

/// `max(value, low)` then `min(.., high)` with the exe's float helpers.
fn clamp(e: &mut Engine, value: f32, low: f32, high: f32) -> f32 {
    let at_least = e.call(FLOAT_MAX, &args![value, low]).f32();
    e.call(FLOAT_MIN, &args![at_least, high]).f32()
}

/// The two values of a face-gen row: `(first, second)`.
fn face_row(e: &mut Engine, coord: Ptr, row: u32) -> (f32, f32) {
    let first = e.call(FACEGEN_GET_VALUE, &args![coord, row, 0u32]).f32();
    let second = e.call(FACEGEN_GET_VALUE, &args![coord, row, 1u32]).f32();
    (first, second)
}

/// Writes a pair of clamped values into a face-gen row.
fn set_face_row(e: &mut Engine, coord: Ptr, row: u32, pair: (f32, f32), low: f32, high: f32) {
    let first = clamp(e, pair.0, low, high);
    let second = clamp(e, pair.1, low, high);
    e.call(FACEGEN_SET_VALUE, &args![coord, row, 0u32, first]);
    e.call(FACEGEN_SET_VALUE, &args![coord, row, 1u32, second]);
}

// Translated from 00601830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Randomizes the face of the NPC (the character-creation "Randomize"
/// button): builds a new face-gen coordinate from the race's scaled by a
/// setting, then replaces the first values of the two rows by random ones
/// (row 0 in `[15, 65)`, row 1 in `[-2, 2)`) unless the matching flag
/// (`keep_row0`, `keep_row1`) is set; the second value of the row moves by
/// the same amount and both are clamped. Unless `keep_hair_length` is set
/// the hair length becomes a random fraction. The result is blended with the
/// race coordinate and, when it differs from the NPC's, copied into it and
/// the body-modification texture is released.
///
/// The compiler's exception-unwinding frame of the two stack coordinates
/// (four `FR2MatrixVTC` each) is not translated.
// Translation incomplete: 00601830 - C++ exception unwinding frame (FS:[0]) not translated
pub fn fn_00601830(
    e: &mut Engine,
    this: Ptr<TESNPC>,
    keep_row0: u8,
    keep_row1: u8,
    keep_hair_length: u8,
) {
    e.with_stack(0x80, |e, random_coord| {
        e.with_stack(0x80, |e, blended| {
            e.call(
                VECTOR_CONSTRUCTOR_ITERATOR,
                &args![
                    random_coord,
                    0x20u32,
                    4u32,
                    MATRIX_CONSTRUCTOR,
                    MATRIX_DESTRUCTOR
                ],
            );
            e.call(
                VECTOR_CONSTRUCTOR_ITERATOR,
                &args![
                    blended,
                    0x20u32,
                    4u32,
                    MATRIX_CONSTRUCTOR,
                    MATRIX_DESTRUCTOR
                ],
            );
            e.call(FACEGEN_INIT_COORD, &args![blended]);

            let race = race_coordinate(e, this);
            let scale_pointer = e.call(SCALE_SETTING_GETTER, &args![SCALE_SETTING]).u32();
            let scale = e.mem.f32(scale_pointer);
            e.call(FACEGEN_SCALE_RACE_COORD, &args![scale, race, random_coord]);

            // Row 0.
            let (first, second) = face_row(e, random_coord, 0);
            let new_first = if keep_row0 != 0 {
                first
            } else {
                let fraction = e.call(RANDOM_FRACTION, &args![]).f64();
                let scale: f64 = e.global(ROW0_RANDOM_SCALE);
                let offset: f64 = e.global(ROW0_RANDOM_OFFSET);
                (fraction * scale + offset) as f32
            };
            let new_second = (second as f64 - (first as f64 - new_first as f64)) as f32;
            let low = e.global::<f32>(ROW0_LOW);
            let high = e.global::<f32>(ROW0_HIGH);
            set_face_row(e, random_coord, 0, (new_first, new_second), low, high);

            // Row 1.
            let (first, second) = face_row(e, random_coord, 1);
            let (new_first, new_second) = if keep_row1 != 0 {
                (first, second)
            } else {
                let fraction = e.call(RANDOM_FRACTION, &args![]).f64();
                let scale: f64 = e.global(ROW1_RANDOM_SCALE);
                let offset: f64 = e.global(ROW1_RANDOM_OFFSET);
                let new_first = (fraction * scale - offset) as f32;
                (
                    new_first,
                    (second as f64 - (first as f64 - new_first as f64)) as f32,
                )
            };
            let low = e.global::<f32>(ROW1_LOW);
            let high = e.global::<f32>(ROW1_HIGH);
            set_face_row(e, random_coord, 1, (new_first, new_second), low, high);

            if keep_hair_length == 0 {
                let fraction = e.call(RANDOM_FRACTION, &args![]).f64();
                fn_00601c10(e, this, fraction as f32);
            }

            let race = race_coordinate(e, this);
            e.call(
                FACEGEN_BLEND_RACE_COORD,
                &args![race, random_coord, blended, 0u32],
            );
            let current = fn_00601800(e, this);
            if e.call(FACEGEN_COORDS_DIFFER, &args![blended, current])
                .bool()
            {
                let current = fn_00601800(e, this);
                e.call(FACEGEN_COPY_COORD, &args![blended, current, 0u32, 0u32]);
                e.call(NI_POINTER_ASSIGN_RAW, &args![this.addr() + 0x1cc, 0u32]);
            }

            e.call(
                VECTOR_DESTRUCTOR_ITERATOR,
                &args![blended, 0x20u32, 4u32, MATRIX_DESTRUCTOR],
            );
            e.call(
                VECTOR_DESTRUCTOR_ITERATOR,
                &args![random_coord, 0x20u32, 4u32, MATRIX_DESTRUCTOR],
            );
        })
    })
}

/// The face-gen coordinate of the NPC's race for its sex.
fn race_coordinate(e: &mut Engine, this: Ptr<TESNPC>) -> u32 {
    let sex = e.call(GET_SEX, &args![this]).u32();
    let race = e
        .call(GET_WORD_AT_4, &args![this.addr() + COMPONENT_RACE])
        .u32();
    e.call(RACE_FACE_COORD, &args![race, sex]).u32()
}

// Translated from 00601c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fHairLength` (Xbox PDB field).
pub fn fn_00601c10(e: &mut Engine, this: Ptr<TESNPC>, length: f32) {
    e.set(this, TESNPC::fHairLength, x87_quiet(length));
}

// Translated from 00601c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC`'s `TESModel::GetModel` override (slot `0x14` of the `TESModel`
/// component at `+0xDC`, Xbox PDB name of the slot): the female model path
/// string (`011d2718`) when the NPC is female, otherwise the male one
/// (`011d3e44`); the string's characters (`00403df0`) are returned.
pub fn fn_00601c30(e: &mut Engine, this: Ptr) -> Ptr {
    let npc = this.addr().wrapping_sub(COMPONENT_MODEL);
    let sex = e.call(GET_SEX, &args![npc]).i32();
    let string = if sex == 1 {
        0x011d_2718u32
    } else {
        0x011d_3e44
    };
    e.call(0x0040_3df0, &args![string]).ptr()
}

// Translated from 00601c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `pCl` (the NPC's class) and records the change (`AddChange` virtual,
/// slot `0x48`, with `0x400`).
pub fn fn_00601c70(e: &mut Engine, this: Ptr<TESNPC>, class: Ptr) {
    e.set(this, TESNPC::pCl, class);
    e.vcall(this.addr(), VSLOT_ADD_CHANGE, &args![CHANGE_CLASS]);
}

/// Writes one chunk made of a float/int array built from a face-gen matrix:
/// every element of the matrix at `matrix`, as 32-bit words.
fn save_matrix_chunk(e: &mut Engine, matrix: Ptr, chunk: u32) {
    let count = e.call(MATRIX_ELEMENT_COUNT, &args![matrix]).u32();
    if count == 0 {
        return;
    }
    let size = count << 2;
    let buffer = e.call(ALLOCATE, &args![size]).u32();
    e.call(MEMSET, &args![buffer, 0u32, size]);
    e.with_stack(8, |e, iterator| {
        for index in 0..count {
            let it = e
                .call(MATRIX_ELEMENT_ITERATOR, &args![matrix, iterator, index])
                .u32();
            let element = e.call(MATRIX_ELEMENT_POINTER, &args![it, 0u32]).u32();
            let value = e.mem.f32(element);
            e.mem.set_f32(buffer + index * 4, value);
        }
    });
    e.call(ADD_CHUNK_ARRAY, &args![chunk, buffer, count]);
    e.call(FREE, &args![buffer]);
}

// Translated from 00601ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESNPC::Save` (virtual slot `0x2c` of `TESForm`, Xbox PDB name): starts
/// the form, saves the components (full name, model with `MODL`/`MODT`,
/// actor base data, race form, spell list, touch spell, destructible), then
/// writes the chunks `CNAM` (class), `DNAM`/NPC data block (`0x1c` bytes, the
/// two byte-swap calls around it do nothing on PC), `PNAM` (head parts),
/// `HNAM` and `LNAM` (hair and its length), `ENAM` (eyes), `HCLR`, `ZNAM`
/// (combat style), `NAM4`, the face-gen arrays `FGGS`, `FGGA`, `FGTS` (one
/// 32-bit float per element of the matrices at `+0x134`, `+0x154`,
/// `+0x174`), `NAM5` (last race face number), `NAM6`/`NAM7` (height and
/// weight), and closes the form.
pub fn tesnpc_save(e: &mut Engine, this: Ptr<TESNPC>) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![base + COMPONENT_FULL_NAME]);
    e.call(
        MODEL_SAVE,
        &args![base + COMPONENT_MODEL, 0x4c44_4f4du32, 0x5444_4f4du32],
    );
    e.call(ACTOR_BASE_DATA_SAVE, &args![base + 0x30]);
    e.call(RACE_FORM_SAVE, &args![base + COMPONENT_RACE]);
    e.call(SPELL_LIST_SAVE, &args![base + COMPONENT_SPELL_LIST]);
    e.call(TOUCH_SPELL_SAVE, &args![base + COMPONENT_TOUCH_SPELL]);
    e.call(DESTRUCTIBLE_SAVE, &args![base + COMPONENT_DESTRUCTIBLE]);
    let class = e.get(this, TESNPC::pCl);
    if !class.is_null() {
        let form_id = e.call(GET_FORM_ID, &args![class]).u32();
        e.call(ADD_CHUNK_VALUE, &args![0x4d41_4e43u32, form_id]);
    }
    e.call(SAVE_DATA, &args![this]);
    if e.call(NEEDS_ENDIAN_SWAP, &args![]).bool() {
        e.call(ENDIAN_SWAP_BLOCK, &args![base + 0x114]);
    }
    e.call(
        ADD_CHUNK_DATA,
        &args![0x4d41_4e44u32, base + 0x114, 0x1cu32],
    );
    if e.call(NEEDS_ENDIAN_SWAP, &args![]).bool() {
        e.call(ENDIAN_SWAP_BLOCK, &args![base + 0x114]);
    }

    let mut node = e.call(NPC_GET_HEAD_PARTS, &args![this]).u32();
    while node != 0 {
        if e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
            break;
        }
        let item = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let head_part = e.mem.u32(item);
        let form_id = e.call(GET_FORM_ID, &args![head_part]).u32();
        e.call(ADD_CHUNK_VALUE, &args![0x4d41_4e50u32, form_id]);
        node = e.call(GET_WORD_AT_4, &args![node]).u32();
    }

    let hair = e.call(NPC_GET_HAIR, &args![this]).u32();
    if hair != 0 {
        let hair = e.call(NPC_GET_HAIR, &args![this]).u32();
        let form_id = e.call(GET_FORM_ID, &args![hair]).u32();
        e.call(ADD_CHUNK_VALUE, &args![0x4d41_4e48u32, form_id]);
        let length = fn_00602130(e, this);
        e.call(ADD_CHUNK_VALUE, &args![0x4d41_4e4cu32, length]);
    }

    let eyes = e.call(NPC_GET_EYE_COLOR, &args![this]).u32();
    if eyes != 0 {
        let eyes = e.call(NPC_GET_EYE_COLOR, &args![this]).u32();
        let form_id = e.call(GET_FORM_ID, &args![eyes]).u32();
        e.call(ADD_CHUNK_VALUE, &args![0x4d41_4e45u32, form_id]);
    }

    let hair_color = e.get(this, TESNPC::iHairColor);
    e.call(ADD_CHUNK_VALUE, &args![0x524c_4348u32, hair_color]);

    let combat_style = e.vcall(base, VSLOT_GET_COMBAT_STYLE, &args![]).u32();
    if combat_style != 0 {
        let combat_style = e.vcall(base, VSLOT_GET_COMBAT_STYLE, &args![]).u32();
        let form_id = e.call(GET_FORM_ID, &args![combat_style]).u32();
        e.call(ADD_CHUNK_VALUE, &args![0x4d41_4e5au32, form_id]);
    }

    let blood_impact = e.get(this, TESNPC::eBloodImpactMaterial);
    e.call(ADD_CHUNK_VALUE, &args![0x344d_414eu32, blood_impact]);

    save_matrix_chunk(e, this.byte_add(0x134), 0x5347_4746);
    save_matrix_chunk(e, this.byte_add(0x154), 0x4147_4746);
    save_matrix_chunk(e, this.byte_add(0x174), 0x5354_4746);

    let last_face = e.get(this, TESNPC::sLastRaceFaceNum);
    e.call(ADD_CHUNK_SHORT, &args![0x354d_414eu32, last_face as u32]);
    let height = e.get(this, TESNPC::fHeight);
    e.call(ADD_CHUNK_VALUE, &args![0x364d_414eu32, height]);
    let weight = e.get(this, TESNPC::fWeight);
    e.call(ADD_CHUNK_VALUE, &args![0x374d_414eu32, weight]);
    e.call(CLOSE_FORM, &args![this]);
}

// Translated from 00602130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `fHairLength` (Xbox PDB field) in `ST0`.
pub fn fn_00602130(e: &mut Engine, this: Ptr<TESNPC>) -> f32 {
    e.get(this, TESNPC::fHairLength)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00601750, tesnpc_clear_data(Ptr<TESNPC>)),
        entry!(0x00601800, fn_00601800(Ptr<TESNPC>) -> Ptr),
        entry!(0x00601830, fn_00601830(Ptr<TESNPC>, u8, u8, u8)),
        entry!(0x00601c10, fn_00601c10(Ptr<TESNPC>, f32)),
        entry!(0x00601c30, fn_00601c30(Ptr) -> Ptr),
        entry!(0x00601c70, fn_00601c70(Ptr<TESNPC>, Ptr)),
        entry!(0x00601ca0, tesnpc_save(Ptr<TESNPC>)),
        entry!(0x00602130, fn_00602130(Ptr<TESNPC>) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine where every address in `addrs` is a double returning zero.
    fn quiet(addrs: &[u32]) -> Engine {
        let mut e = Engine::new();
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
        e
    }

    fn npc(e: &mut Engine) -> Ptr<TESNPC> {
        e.new_object::<TESNPC>()
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Doubles that record every call (address and words) in the log.
    fn recording(e: &mut Engine, addrs: &[u32]) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for a in addrs {
            let a = *a;
            let log = log.clone();
            e.register_double(a, move |_, w| {
                log.borrow_mut().push((a, w.to_vec()));
                Ret::default()
            });
        }
        log
    }

    fn returning(value: u32) -> impl FnMut(&mut Engine, &[u32]) -> Ret {
        move |_, _| Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn clear_data_releases_heads_and_the_alternate_coordinate() {
        let mut e = quiet(&[SAVE_LOAD_UNAVAILABLE]);
        e.map(SAVE_LOAD_GAME, 4);
        let log = recording(
            &mut e,
            &[
                NI_POINTER_ASSIGN_RAW,
                NI_POINTER_ASSIGN,
                SIMPLE_LIST_CLEAR,
                DELETE_OBJECT,
            ],
        );
        let this = npc(&mut e);
        let base = this.addr();
        e.set(this, TESNPC::sLastRaceFaceNum, 7);
        e.set(this, TESNPC::pAlternateFaceOffsetCoord, Ptr::new(0x5000));
        tesnpc_clear_data(&mut e, this);
        assert_eq!(e.get(this, TESNPC::sLastRaceFaceNum), 0xff);
        assert!(e.get(this, TESNPC::pAlternateFaceOffsetCoord).is_null());
        assert_eq!(
            calls_to(&log, NI_POINTER_ASSIGN_RAW),
            vec![vec![base + 0x1c4, 0], vec![base + 0x1cc, 0]]
        );
        assert_eq!(calls_to(&log, NI_POINTER_ASSIGN).len(), 1);
        assert_eq!(calls_to(&log, SIMPLE_LIST_CLEAR), vec![vec![base + 0x1dc]]);
        assert_eq!(calls_to(&log, DELETE_OBJECT), vec![vec![0x5000, 3]]);
    }

    #[test]
    fn clear_data_skips_the_heads_when_the_save_load_object_says_so() {
        let mut e = quiet(&[]);
        e.register_double(SAVE_LOAD_UNAVAILABLE, returning(1));
        e.map(SAVE_LOAD_GAME, 4);
        let log = recording(
            &mut e,
            &[NI_POINTER_ASSIGN_RAW, SIMPLE_LIST_CLEAR, DELETE_OBJECT],
        );
        let this = npc(&mut e);
        tesnpc_clear_data(&mut e, this);
        assert!(calls_to(&log, NI_POINTER_ASSIGN_RAW).is_empty());
        // A null alternate coordinate is not deleted.
        assert!(calls_to(&log, DELETE_OBJECT).is_empty());
        assert_eq!(calls_to(&log, SIMPLE_LIST_CLEAR).len(), 1);
    }

    #[test]
    fn face_coordinate_pointer_prefers_the_alternate() {
        let mut e = Engine::new();
        let this = npc(&mut e);
        assert_eq!(fn_00601800(&mut e, this), this.byte_add(0x134));
        e.set(this, TESNPC::pAlternateFaceOffsetCoord, Ptr::new(0x7000));
        assert_eq!(e.call(0x0060_1800, &args![this]).u32(), 0x7000);
    }

    /// Doubles for the randomizer: the coordinate being built holds
    /// `values[row][column]`; sets and the later steps are recorded.
    fn randomizer_engine(values: [[f32; 2]; 2], fraction: f64) -> (Engine, Log) {
        let mut e = quiet(&[
            VECTOR_CONSTRUCTOR_ITERATOR,
            VECTOR_DESTRUCTOR_ITERATOR,
            FACEGEN_INIT_COORD,
            GET_SEX,
            GET_WORD_AT_4,
            RACE_FACE_COORD,
            FACEGEN_SCALE_RACE_COORD,
        ]);
        e.map(SCALE_SETTING, 8);
        e.register(SCALE_SETTING_GETTER, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        for global in [
            ROW0_LOW,
            ROW0_HIGH,
            ROW1_LOW,
            ROW1_HIGH,
            ROW0_RANDOM_SCALE,
            ROW0_RANDOM_OFFSET,
            ROW1_RANDOM_SCALE,
            ROW1_RANDOM_OFFSET,
        ] {
            e.map(global, 8);
        }
        e.set_global(ROW0_LOW, 15.0f32);
        e.set_global(ROW0_HIGH, 65.0f32);
        e.set_global(ROW1_LOW, -2.0f32);
        e.set_global(ROW1_HIGH, 2.0f32);
        e.set_global(ROW0_RANDOM_SCALE, 50.0f64);
        e.set_global(ROW0_RANDOM_OFFSET, 15.0f64);
        e.set_global(ROW1_RANDOM_SCALE, 4.0f64);
        e.set_global(ROW1_RANDOM_OFFSET, 2.0f64);
        e.register_double(FACEGEN_GET_VALUE, move |_, a| Ret {
            st0: values[a[1] as usize][a[2] as usize] as f64,
            ..Ret::default()
        });
        e.register_double(RANDOM_FRACTION, move |_, _| Ret {
            st0: fraction,
            ..Ret::default()
        });
        e.register(FLOAT_MAX, |_, a| Ret {
            st0: f32::from_bits(a[0]).max(f32::from_bits(a[1])) as f64,
            ..Ret::default()
        });
        e.register(FLOAT_MIN, |_, a| Ret {
            st0: f32::from_bits(a[0]).min(f32::from_bits(a[1])) as f64,
            ..Ret::default()
        });
        let log = recording(
            &mut e,
            &[
                FACEGEN_SET_VALUE,
                FACEGEN_BLEND_RACE_COORD,
                FACEGEN_COORDS_DIFFER,
                FACEGEN_COPY_COORD,
                NI_POINTER_ASSIGN_RAW,
            ],
        );
        (e, log)
    }

    fn set_values(log: &Log) -> Vec<(u32, u32, f32)> {
        calls_to(log, FACEGEN_SET_VALUE)
            .iter()
            .map(|w| (w[1], w[2], f32::from_bits(w[3])))
            .collect()
    }

    #[test]
    fn randomizer_keeping_everything_only_clamps() {
        let (mut e, log) = randomizer_engine([[30.0, 90.0], [1.0, -5.0]], 0.5);
        let this = npc(&mut e);
        e.set(this, TESNPC::fHairLength, 3.0);
        fn_00601830(&mut e, this, 1, 1, 1);
        assert_eq!(
            set_values(&log),
            vec![(0, 0, 30.0), (0, 1, 65.0), (1, 0, 1.0), (1, 1, -2.0)]
        );
        // Nothing random was drawn: the hair length is untouched.
        assert_eq!(e.get(this, TESNPC::fHairLength), 3.0);
        // No difference with the NPC's own coordinate: no copy.
        assert!(calls_to(&log, FACEGEN_COPY_COORD).is_empty());
    }

    #[test]
    fn randomizer_draws_new_values_and_shifts_the_second_of_the_row() {
        let (mut e, log) = randomizer_engine([[30.0, 40.0], [1.0, 1.5]], 0.5);
        e.register_double(FACEGEN_COORDS_DIFFER, returning(1));
        let this = npc(&mut e);
        fn_00601830(&mut e, this, 0, 0, 0);
        // Row 0: 0.5 * 50 + 15 = 40; second = 40 - (30 - 40) = 50.
        // Row 1: 0.5 * 4 - 2 = 0; second = 1.5 - (1 - 0) = 0.5.
        assert_eq!(
            set_values(&log),
            vec![(0, 0, 40.0), (0, 1, 50.0), (1, 0, 0.0), (1, 1, 0.5)]
        );
        assert_eq!(e.get(this, TESNPC::fHairLength), 0.5);
        // The difference copies the result into the NPC's coordinate (the
        // embedded one) and releases the body texture.
        let copies = calls_to(&log, FACEGEN_COPY_COORD);
        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0][1], this.addr() + 0x134);
        assert_eq!(
            calls_to(&log, NI_POINTER_ASSIGN_RAW),
            vec![vec![this.addr() + 0x1cc, 0]]
        );
    }

    #[test]
    fn hair_length_accessors() {
        let mut e = Engine::new();
        let this = npc(&mut e);
        fn_00601c10(&mut e, this, 2.5);
        assert_eq!(e.get(this, TESNPC::fHairLength), 2.5);
        assert_eq!(e.call(0x0060_2130, &args![this]).f32(), 2.5);
    }

    #[test]
    fn model_name_follows_the_sex() {
        let mut e = quiet(&[]);
        e.register(GET_SEX, |_, a| Ret {
            eax: (a[0] == 0x1000 - COMPONENT_MODEL) as u32,
            ..Ret::default()
        });
        e.register(0x0040_3df0, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        let female = e.call(0x0060_1c30, &args![0x1000u32]).u32();
        assert_eq!(female, 0x011d_2718 + 0x10);
        let male = e.call(0x0060_1c30, &args![0x2000u32]).u32();
        assert_eq!(male, 0x011d_3e44 + 0x10);
    }

    #[test]
    fn set_class_records_the_change() {
        let mut e = quiet(&[]);
        let log = recording(&mut e, &[0x0060_c000]);
        let this = npc(&mut e);
        let mut slots = vec![0u32; 0x20];
        slots[VSLOT_ADD_CHANGE as usize / 4] = 0x0060_c000;
        e.put_vtable(0x0a00_0000, &slots);
        e.mem.set_u32(this.addr(), 0x0a00_0000);
        fn_00601c70(&mut e, this, Ptr::new(0x4242));
        assert_eq!(e.get(this, TESNPC::pCl), Ptr::new(0x4242));
        assert_eq!(calls_to(&log, 0x0060_c000), vec![vec![this.addr(), 0x400]]);
    }

    /// Everything `Save` calls, recorded and answered with zero except the
    /// small accessors.
    fn save_engine() -> (Engine, Log) {
        let mut e = quiet(&[]);
        e.register(GET_FORM_ID, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        e.register(LIST_NODE_ITEM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(GET_WORD_AT_4, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(FREE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        for quiet_callee in [
            MEMSET,
            MATRIX_ELEMENT_COUNT,
            NPC_GET_HEAD_PARTS,
            NPC_GET_HAIR,
            NPC_GET_EYE_COLOR,
            LIST_NODE_IS_EMPTY,
        ] {
            e.register(quiet_callee, |_, _| Ret::default());
        }
        let log = recording(
            &mut e,
            &[
                START_FORM,
                FULL_NAME_SAVE,
                MODEL_SAVE,
                ACTOR_BASE_DATA_SAVE,
                RACE_FORM_SAVE,
                SPELL_LIST_SAVE,
                TOUCH_SPELL_SAVE,
                DESTRUCTIBLE_SAVE,
                SAVE_DATA,
                NEEDS_ENDIAN_SWAP,
                ENDIAN_SWAP_BLOCK,
                ADD_CHUNK_VALUE,
                ADD_CHUNK_SHORT,
                ADD_CHUNK_DATA,
                ADD_CHUNK_ARRAY,
                CLOSE_FORM,
                COMBAT_STYLE_GETTER,
            ],
        );
        (e, log)
    }

    /// An address standing in for the combat style getter of the vtable.
    const COMBAT_STYLE_GETTER: u32 = 0x0060_c100;

    fn with_combat_style_slot(e: &mut Engine, this: Ptr<TESNPC>) {
        let mut slots = vec![0u32; 0x80];
        slots[VSLOT_GET_COMBAT_STYLE as usize / 4] = COMBAT_STYLE_GETTER;
        e.put_vtable(0x0a00_0000, &slots);
        e.mem.set_u32(this.addr(), 0x0a00_0000);
    }

    fn chunks(log: &Log) -> Vec<(u32, Vec<u32>)> {
        log.borrow()
            .iter()
            .filter(|(a, _)| {
                [
                    ADD_CHUNK_VALUE,
                    ADD_CHUNK_SHORT,
                    ADD_CHUNK_DATA,
                    ADD_CHUNK_ARRAY,
                ]
                .contains(a)
            })
            .map(|(a, w)| (*a, w.clone()))
            .collect()
    }

    #[test]
    fn save_of_a_bare_npc_writes_the_fixed_chunks() {
        let (mut e, log) = save_engine();
        let this = npc(&mut e);
        with_combat_style_slot(&mut e, this);
        e.set(this, TESNPC::iHairColor, 0x11);
        e.set(this, TESNPC::eBloodImpactMaterial, 6);
        e.set(this, TESNPC::sLastRaceFaceNum, 0x1ff);
        e.set(this, TESNPC::fHeight, 1.5);
        e.set(this, TESNPC::fWeight, 0.5);
        tesnpc_save(&mut e, this);
        let base = this.addr();
        assert_eq!(
            chunks(&log),
            vec![
                (ADD_CHUNK_DATA, vec![0x4d41_4e44, base + 0x114, 0x1c]),
                (ADD_CHUNK_VALUE, vec![0x524c_4348, 0x11]),
                (ADD_CHUNK_VALUE, vec![0x344d_414e, 6]),
                (ADD_CHUNK_SHORT, vec![0x354d_414e, 0x1ff]),
                (ADD_CHUNK_VALUE, vec![0x364d_414e, 1.5f32.to_bits()]),
                (ADD_CHUNK_VALUE, vec![0x374d_414e, 0.5f32.to_bits()]),
            ]
        );
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(order[0], START_FORM);
        assert_eq!(*order.last().unwrap(), CLOSE_FORM);
        assert_eq!(
            calls_to(&log, MODEL_SAVE),
            vec![vec![base + 0xdc, 0x4c44_4f4d, 0x5444_4f4d]]
        );
        assert_eq!(
            calls_to(&log, ACTOR_BASE_DATA_SAVE),
            vec![vec![base + 0x30]]
        );
        // No byte swapping on PC.
        assert!(calls_to(&log, ENDIAN_SWAP_BLOCK).is_empty());
    }

    #[test]
    fn save_writes_class_head_parts_hair_eyes_and_combat_style() {
        let (mut e, log) = save_engine();
        let this = npc(&mut e);
        with_combat_style_slot(&mut e, this);
        // Forms: the id is at +0xC.
        let form = |e: &mut Engine, id: u32| {
            let f = e.mem.alloc(0x10);
            e.mem.set_u32(f + 0xc, id);
            f
        };
        let class = form(&mut e, 0xc1);
        let hair = form(&mut e, 0xa1);
        let eyes = form(&mut e, 0xe1);
        let part = form(&mut e, 0xb1);
        // A one-node list: the node's first word is the item (a head part).
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, part);
        e.set(this, TESNPC::pCl, Ptr::new(class));
        e.register_double(NPC_GET_HEAD_PARTS, returning(node));
        e.register_double(NPC_GET_HAIR, returning(hair));
        e.register_double(NPC_GET_EYE_COLOR, returning(eyes));
        e.register_double(COMBAT_STYLE_GETTER, returning(class));
        e.register_double(NEEDS_ENDIAN_SWAP, returning(1));
        e.set(this, TESNPC::fHairLength, 4.0);
        tesnpc_save(&mut e, this);
        let written: Vec<Vec<u32>> = chunks(&log).into_iter().map(|(_, w)| w).collect();
        assert_eq!(written[0], vec![0x4d41_4e43, 0xc1]);
        assert_eq!(written[1][0], 0x4d41_4e44);
        assert_eq!(written[2], vec![0x4d41_4e50, 0xb1]);
        assert_eq!(written[3], vec![0x4d41_4e48, 0xa1]);
        assert_eq!(written[4], vec![0x4d41_4e4c, 4.0f32.to_bits()]);
        assert_eq!(written[5], vec![0x4d41_4e45, 0xe1]);
        assert_eq!(written[6][0], 0x524c_4348);
        assert_eq!(written[7], vec![0x4d41_4e5a, 0xc1]);
        // The block is byte-swapped before and after it is written.
        assert_eq!(calls_to(&log, ENDIAN_SWAP_BLOCK).len(), 2);
    }

    #[test]
    fn save_writes_the_face_gen_arrays() {
        let (mut e, _log) = save_engine();
        let this = npc(&mut e);
        with_combat_style_slot(&mut e, this);
        let base = this.addr();
        // Only the matrix at +0x154 has elements (two).
        e.register_double(MATRIX_ELEMENT_COUNT, move |_, a| Ret {
            eax: if a[0] == base + 0x154 { 2 } else { 0 },
            ..Ret::default()
        });
        e.register(MATRIX_ELEMENT_ITERATOR, |_, a| Ret {
            eax: a[2] + 1,
            ..Ret::default()
        });
        let values = e.mem.alloc(8);
        e.mem.set_f32(values, 1.25);
        e.mem.set_f32(values + 4, -3.5);
        e.register_double(MATRIX_ELEMENT_POINTER, move |_, a| Ret {
            eax: values + (a[0] - 1) * 4,
            ..Ret::default()
        });
        let written: Rc<RefCell<Vec<(u32, f32)>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = written.clone();
        e.register_double(ADD_CHUNK_ARRAY, move |e, a| {
            for i in 0..a[2] {
                sink.borrow_mut().push((a[0], e.mem.f32(a[1] + 4 * i)));
            }
            Ret::default()
        });
        tesnpc_save(&mut e, this);
        assert_eq!(
            *written.borrow(),
            vec![(0x4147_4746, 1.25), (0x4147_4746, -3.5)]
        );
    }
}
