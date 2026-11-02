//! `fallout shared/tesobjectbook.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESObjectBOOK` (vtable `0102a9c4`, RTTI `.?AVTESObjectBOOK@@`, form type
//! 0x19). Its components sit at the offsets the constructor `00514500`
//! gives them; the ones used here: `+0x30` (full name), `+0x3C` (model with
//! texture swaps), `+0x5C` (icon), `+0x68` (script), `+0x74`
//! (enchantment), `+0x94` (description), `+0x9C` (destructible object),
//! `+0xA4` (message icon), `+0xB4` (sounds); the book's data is 2 bytes at
//! `+0xC0` (flags, then the skill byte at `+0xC1`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESFile::GetTESForm` (Xbox PDB, `00472660`): the record's form type.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, `RET 4`).
const LOAD_FORM: u32 = 0x0048_5110;
/// Sets (`true`) or clears form flag 0x8 (`00484ab0`, `RET 4`).
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// Tests form flag 0x8 (`004013e0`).
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The size of the current chunk (`00401660`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`): buffer and size.
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (Xbox PDB, `004727f0`): one word.
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov2` (Xbox PDB, `00472840`): one 16-bit word.
const GET_CHUNK_DATA_U16: u32 = 0x0047_2840;
/// Moves to the next chunk (`004726f0`, `bool`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESForm::LoadData` (`004861f0`, `RET 0xC`): file, buffer, size.
const LOAD_DATA: u32 = 0x0048_61f0;
/// The setter the form-id chunks go to (`006ecd40`, `RET 4`; the engine map
/// names the folded body `CombatProcedureAttackMelee::Initialize`).
const SET_FORM: u32 = 0x006e_cd40;
/// `ActorValue::ToActorValue` (Xbox PDB, `0066ec10`, cdecl, two words).
const TO_ACTOR_VALUE: u32 = 0x0066_ec10;

/// Chunk signatures, as the words the exe compares.
const SIG_ANAM: u32 = 0x4d41_4e41;
const SIG_DATA: u32 = 0x4154_4144;
const SIG_DESC: u32 = 0x4353_4544;
const SIG_DEST: u32 = 0x5453_4544;
const SIG_DSTD: u32 = 0x4454_5344;
const SIG_EAMT: u32 = 0x544d_4145;
const SIG_EDID: u32 = 0x4449_4445;
const SIG_EITM: u32 = 0x4d54_4945;
const SIG_ENAM: u32 = 0x4d41_4e45;
const SIG_FULL: u32 = 0x4c4c_5546;
const SIG_ICON: u32 = 0x4e4f_4349;
const SIG_MICO: u32 = 0x4f43_494d;
const SIG_MODD: u32 = 0x4444_4f4d;
const SIG_MODL: u32 = 0x4c44_4f4d;
const SIG_MODS: u32 = 0x5344_4f4d;
const SIG_MODT: u32 = 0x5444_4f4d;
const SIG_MOSD: u32 = 0x4453_4f4d;
const SIG_OBND: u32 = 0x444e_424f;
const SIG_SCRI: u32 = 0x4952_4353;
const SIG_YNAM: u32 = 0x4d41_4e59;
const SIG_ZNAM: u32 = 0x4d41_4e5a;

/// A word read from the chunk into a cleared local.
fn read_word(e: &mut Engine, file: Ptr, local: Ptr) -> u32 {
    e.mem.set_u32(local.addr(), 0);
    e.call(GET_CHUNK_DATA, &args![file, local]);
    e.mem.u32(local.addr())
}

// Translated from 00514870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectBOOK::InitItem` (Xbox PDB): unless form flag 0x8 is set
/// (`004013e0`), the components' `InitItem` with the book (`0048cd90` on
/// `+0x68`, `00483210` on `+0x74`, `00478be0` on `+0x9C`,
/// `TESModelTextureSwap::InitItem` `0048aa80` on `+0x3C`, `0047b6a0` on
/// `+0xB4`); a skill byte (`+0xC1`, signed) other than -1 whose actor value
/// (`ActorValue::ToActorValue`, `0066ec10`, with 2 and the byte) is below
/// 0x20 or above 0x2E becomes 0xFF. Finally form flag 0x8 is set.
pub fn tes_object_book_init_item(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    for (callee, offset) in [
        (0x0048_cd90, 0x68),
        (0x0048_3210, 0x74),
        (0x0047_8be0, 0x9c),
        (0x0048_aa80, 0x3c),
        (0x0047_b6a0, 0xb4),
    ] {
        e.call(callee, &args![base + offset, this]);
    }
    if e.mem.i8(base + 0xc1) != -1 {
        let skill = e.mem.u8(base + 0xc1);
        let value = e.call(TO_ACTOR_VALUE, &args![2u32, u32::from(skill)]).i32();
        if !(0x20..=0x2e).contains(&value) {
            e.mem.set_u8(base + 0xc1, 0xff);
        }
    }
    e.call(SET_FORM_FLAG_8, &args![this, true]);
}

// Translated from 00514930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00514930 - the /GS cookie check (00ec408c) is not translated; the __alloca_probe_16 (00ec5ec0) buffers are with_stack blocks of the chunk size (their 16-byte alignment padding, and the stack growing with each such chunk, are not modelled)
/// `TESObjectBOOK`'s `Load` (`RET 4`): false unless the record is of form
/// type 0x19. Then `TESForm::LoadForm` (Xbox PDB, `00485110`), form flag
/// 0x8 cleared, and every chunk of `file` (a component address is null
/// when `this` is, as in the exe):
/// - `EDID` (vtable slot 0x134, at most 0x200 bytes), `OBND` (slot 0xE0);
/// - `FULL` (`00487050` on `+0x30`), `MODL`/`MODD`/`MOSD`/`MODT`
///   (`TESModel::LoadModelChunk`, `004892d0`, on `+0x3C`), `MODS` (the
///   chunk read whole into a stack buffer and unpacked by `0048a7b0` on
///   `+0x3C` with its size and the form version `00403570`), `ICON`
///   (`0048e3d0` on `+0x5C`), `MICO` (`0047a290` on `+0xA4`), `DESC`
///   (`00482d50` on `+0x94`), `DSTD`/`DEST` (`004781e0` on `+0x9C`);
/// - form ids (each read into a cleared local): `SCRI` (`006ecd40` and
///   `0048cd90` with the book on `+0x68`), `ENAM`/`EITM` (`006ecd40` on
///   `+0x74`), `YNAM` (`006ecd40` on `+0xB4`), `ZNAM` (`00403550` on
///   `+0xB4`); `ANAM`/`EAMT` are 16-bit words (`00472840`) given to
///   `00483170` on `+0x74`;
/// - `DATA`: 2 bytes into `+0xC0` (`TESForm::LoadData`, `004861f0`).
///
/// Returns true.
pub fn fn_00514930(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    let component = |offset: u32| if base != 0 { base + offset } else { 0 };
    if e.call(GET_TES_FORM, &args![file]).u8() != 0x19 {
        return false;
    }
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, false]);
    e.with_stack(4, |e, word_at| loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            SIG_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, 0x200u32]);
                    e.vcall(base, 0x134, &args![buffer]);
                });
            }
            SIG_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            SIG_FULL => {
                e.call(0x0048_7050, &args![component(0x30), file]);
            }
            SIG_MODL | SIG_MODD | SIG_MOSD | SIG_MODT => {
                e.call(0x0048_92d0, &args![component(0x3c), file]);
            }
            SIG_MODS => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, size]);
                    let version = e.call(0x0040_3570, &args![file]).u16();
                    e.call(
                        0x0048_a7b0,
                        &args![base + 0x3c, buffer, size, u32::from(version)],
                    );
                });
            }
            SIG_ICON => {
                e.call(0x0048_e3d0, &args![component(0x5c), file]);
            }
            SIG_MICO => {
                e.call(0x0047_a290, &args![component(0xa4), file]);
            }
            SIG_SCRI => {
                let script = read_word(e, file, word_at);
                e.call(SET_FORM, &args![base + 0x68, script]);
                e.call(0x0048_cd90, &args![base + 0x68, this]);
            }
            SIG_ENAM | SIG_EITM => {
                let enchantment = read_word(e, file, word_at);
                e.call(SET_FORM, &args![base + 0x74, enchantment]);
            }
            SIG_ANAM | SIG_EAMT => {
                e.mem.set_u16(word_at.addr(), 0);
                e.call(GET_CHUNK_DATA_U16, &args![file, word_at]);
                let amount = e.mem.u16(word_at.addr());
                e.call(0x0048_3170, &args![base + 0x74, u32::from(amount)]);
            }
            SIG_DESC => {
                e.call(0x0048_2d50, &args![component(0x94), file]);
            }
            SIG_DSTD | SIG_DEST => {
                e.call(0x0047_81e0, &args![component(0x9c), file]);
            }
            SIG_YNAM => {
                let sound = read_word(e, file, word_at);
                e.call(SET_FORM, &args![base + 0xb4, sound]);
            }
            SIG_ZNAM => {
                let sound = read_word(e, file, word_at);
                e.call(0x0040_3550, &args![base + 0xb4, sound]);
            }
            SIG_DATA => {
                e.call(LOAD_DATA, &args![this, file, base + 0xc0, 2u32]);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    });
    true
}

// Translated from 00514f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the book's data (`this + 0xC0`).
pub fn fn_00514f30(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0xc0)
}

/// `TESForm::StartForm` (`004855a0`) and `TESForm::CloseForm` (`00485680`),
/// `thiscall` on the form, no stack words.
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
/// The component saves (engine map; `thiscall`): `TESFullName::Save`
/// (`00487010`), `TESModelTextureSwap::Save` (`0048a520`, three chunk
/// tags), `TESTexture::Save` (`0048e370`, a tag), `BGSMessageIcon::Save`
/// (`0047a240`, a tag), `TESScriptableForm::Save` (`0048cca0`),
/// `TESEnchantableForm::Save` (`0047bd70`), `TESDescription::Save`
/// (`00482cb0`), `BGSDestructibleObjectForm::Save` (`00477f20`) and
/// `BGSPickupPutdownSounds::Save` (`0047b560`).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const MODEL_TEXTURE_SWAP_SAVE: u32 = 0x0048_a520;
const TEXTURE_SAVE: u32 = 0x0048_e370;
const MESSAGE_ICON_SAVE: u32 = 0x0047_a240;
const SCRIPTABLE_FORM_SAVE: u32 = 0x0048_cca0;
const ENCHANTABLE_SAVE: u32 = 0x0047_bd70;
const DESCRIPTION_SAVE: u32 = 0x0048_2cb0;
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
const PICKUP_PUTDOWN_SAVE: u32 = 0x0047_b560;
/// `00401500`: the byte at `011c54ba` (whether saved words are
/// byte-swapped); `00483710` (`thiscall` on the data): its byte swap, whose
/// body does nothing.
const SAVE_SWAPS_BYTES: u32 = 0x0040_1500;
const SWAP_BOOK_DATA: u32 = 0x0048_3710;
/// `TESForm::__SaveData` (`00485e20`, `thiscall`, `RET 8`: data, size).
const SAVE_DATA: u32 = 0x0048_5e20;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the type
/// descriptors `TESForm` (`01183028`), `TESObjectBOOK` (`0118a650`),
/// `TESObjectREFR` (`011841cc`) and `Actor` (`011846d4`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_BOOK: u32 = 0x0118_a650;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_ACTOR: u32 = 0x0118_46d4;
/// `TESForm::CopyAllComponents` (`004851b0`, `RET 4`) and
/// `TESForm::CompareAllComponents` (`00485270`, `RET 4`).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `_memcmp` (`00ec4835`, cdecl).
const MEMCMP: u32 = 0x00ec_4835;
/// `0050f670` (`thiscall`, `RET 0x14`; engine map unit `tesobject.cpp`).
const OBJECT_ACTIVATE_0050F670: u32 = 0x0050_f670;
/// `Read`'s callees: `00493bb0` (`thiscall` on the actor),
/// `Interface::IsInMenuMode` (`00702360`), the message box `007052f0`
/// (cdecl, six words), a string setting's text (`00403df0`, `thiscall` on
/// the setting) and a float setting's address (`00403e20`), `0047f060`
/// (cdecl, the actor value), `BGSEntryPoint::HandleEntryPoint` (`005e58f0`,
/// cdecl, three words), `MiscStatManager::Increment` (`004d5c60`, cdecl),
/// `_ftol2` (`00ec62c0`, the value in `ST0`),
/// `ActorValue::GetActorValueName` (`0066e950`, cdecl), the `BSStringT`
/// constructor (`004037b0`), format (`00406f60`, cdecl), text (`00559450`)
/// and destructor (`004037d0`).
const ACTOR_CHECK_00493BB0: u32 = 0x0049_3bb0;
const IS_IN_MENU_MODE: u32 = 0x0070_2360;
const SHOW_MESSAGE: u32 = 0x0070_52f0;
const SETTING_STRING: u32 = 0x0040_3df0;
const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;
const SKILL_CHECK_0047F060: u32 = 0x0047_f060;
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
const FTOL2: u32 = 0x00ec_62c0;
const ACTOR_VALUE_NAME: u32 = 0x0066_e950;
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_FORMAT: u32 = 0x0040_6f60;
const STRING_TEXT: u32 = 0x0055_9450;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// The player (the word at `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// The settings `Read` uses: the message when the book can't be read now
/// (string, `011d4ccc`), the skill-increase message format (string,
/// `011d49c0`) and the skill points a book gives (float, `011c9284`).
const CANNOT_READ_SETTING: u32 = 0x011d_4ccc;
const SKILL_MESSAGE_SETTING: u32 = 0x011d_49c0;
const BOOK_SKILL_POINTS_SETTING: u32 = 0x011c_9284;
/// The float (2.0) at `010162c0` both messages pass.
const MESSAGE_FLOAT: u32 = 0x0101_62c0;

// Translated from 00514de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectBOOK::Save` (engine map; no stack words): `StartForm`, the
/// full name (+0x30), the model with texture swaps (+0x3c; `MODL`, `MODT`,
/// `MODS`), the icon (+0x5c, `ICON`), the message icon (+0xa4, `MICO`),
/// script (+0x68), enchantment (+0x74), description (+0x94), destructible
/// data (+0x9c), pickup/putdown sounds (+0xb4), the 2 bytes of data at
/// +0xc0 (`__SaveData`; `00483710` before and after when `00401500` says
/// so), `CloseForm`.
pub fn tes_object_book_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![base + 0x30]);
    e.call(
        MODEL_TEXTURE_SWAP_SAVE,
        &args![base + 0x3c, SIG_MODL, SIG_MODT, SIG_MODS],
    );
    e.call(TEXTURE_SAVE, &args![base + 0x5c, SIG_ICON]);
    e.call(MESSAGE_ICON_SAVE, &args![base + 0xa4, SIG_MICO]);
    for (offset, save) in [
        (0x68u32, SCRIPTABLE_FORM_SAVE),
        (0x74, ENCHANTABLE_SAVE),
        (0x94, DESCRIPTION_SAVE),
        (0x9c, DESTRUCTIBLE_SAVE),
        (0xb4, PICKUP_PUTDOWN_SAVE),
    ] {
        e.call(save, &args![base + offset]);
    }
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        e.call(SWAP_BOOK_DATA, &args![base + 0xc0]);
    }
    e.call(SAVE_DATA, &args![this, base + 0xc0, 2u32]);
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        e.call(SWAP_BOOK_DATA, &args![base + 0xc0]);
    }
    e.call(CLOSE_FORM, &args![this]);
}

/// `other` as a `TESObjectBOOK` (`__RTDynamicCast` from `TESForm`), or 0.
fn as_book(e: &mut Engine, other: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![other, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_BOOK, 0u32],
    )
    .u32()
}

// Translated from 00514ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The book's copy (engine map: no name; `RET 4`): when `other` is a
/// `TESObjectBOOK`, copies all components (`TESForm::CopyAllComponents` with
/// `other`) and the 16-bit data word at +0xc0.
pub fn fn_00514ed0(e: &mut Engine, this: Ptr, other: u32) {
    let source = as_book(e, other);
    if source == 0 {
        return;
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, other]);
    let data = fn_00514f30(e, Ptr::new(source));
    let word = e.mem.u16(data);
    e.mem.set_u16(this.addr() + 0xc0, word);
}

// Translated from 00514f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The book's compare (engine map: no name; `RET 4`): true when `other` is
/// no `TESObjectBOOK`, when `TESForm::CompareAllComponents` (with the cast
/// form) says they differ, or when the 2 bytes of data at +0xc0 differ
/// (`_memcmp`).
pub fn fn_00514f50(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let source = as_book(e, other);
    if source == 0 {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, source]).u32() as u8 != 0 {
        return true;
    }
    let data = fn_00514f30(e, Ptr::new(source));
    e.call(MEMCMP, &args![this.addr() + 0xc0, data, 2u32]).u32() != 0
}

// Translated from 00514fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The skill a book teaches (engine map: no name; no stack words): -1 when
/// the signed byte at +0xc1 is -1, else `ActorValue::ToActorValue(2, byte)`.
pub fn fn_00514fd0(e: &mut Engine, this: Ptr) -> u32 {
    if e.mem.i8(this.addr() + 0xc1) == -1 {
        return 0xffff_ffff;
    }
    let skill = u32::from(e.mem.u8(this.addr() + 0xc1));
    e.call(TO_ACTOR_VALUE, &args![2u32, skill]).u32()
}

// Translated from 00515010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its five words to `0050f670` on `this` (the third as a byte,
/// zero-extended) and answers what it answers (engine map: no name;
/// `RET 0x14`).
pub fn fn_00515010(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u8,
    fourth: u32,
    fifth: u32,
) -> u32 {
    e.call(
        OBJECT_ACTIVATE_0050F670,
        &args![this, first, second, u32::from(third), fourth, fifth],
    )
    .u32()
}

/// The message box `007052f0` with `text`, `second`, 0, 0, the float at
/// `010162c0` (`FLD`/`FSTP` copy) and 0.
fn show_message(e: &mut Engine, text: u32, second: u32) {
    let float = x87_quiet_bits(e.global::<u32>(MESSAGE_FLOAT));
    e.call(SHOW_MESSAGE, &args![text, second, 0u32, 0u32, float, 0u32]);
}

// Translated from 00515040 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00515040 - the FS:[0] exception frame (it destroys the message string if a step throws) is not translated.
/// `TESObjectBOOK::Read` (engine map; `RET 4`, the reader as a reference):
/// casts the reader to `Actor` (no null check before its vtable slot
/// `+0x360`); when that slot answers false, false. When `00493bb0` answers
/// true and the interface is not in menu mode, shows the "cannot read now"
/// message (`011d4ccc`) and answers false. Otherwise, when the book's skill
/// (`fn_00514fd0`) passes `0047f060` and the player's value of it (the
/// player's `+0xa4` part, vtable slot `+0x1c`) is below 200: takes the
/// skill points setting (`011c9284`), lets entry point 0xb adjust them
/// (`BGSEntryPoint::HandleEntryPoint(0xb, player, &points)`), raises the
/// skill (player vtable slot `+0x3a4` with the skill, the points and 0),
/// counts misc stat 0xf, formats the message (`011d49c0`, the skill's name,
/// the points through `_ftol2`) into a string, shows it and answers true.
/// Else false. The string and the points the game keeps on its stack are a
/// block here.
pub fn tes_object_book_read(e: &mut Engine, this: Ptr, reader: u32) -> bool {
    let actor = e
        .call(
            RT_DYNAMIC_CAST,
            &args![reader, 0u32, TYPE_TES_OBJECT_REFR, TYPE_ACTOR, 0u32],
        )
        .u32();
    if e.vcall(actor, 0x360, &[]).u32() as u8 == 0 {
        return false;
    }
    if e.call(ACTOR_CHECK_00493BB0, &args![actor]).u32() as u8 != 0
        && e.call(IS_IN_MENU_MODE, &[]).u32() as u8 == 0
    {
        let text = e.call(SETTING_STRING, &args![CANNOT_READ_SETTING]).u32();
        show_message(e, text, 0);
        return false;
    }
    let skill = fn_00514fd0(e, this);
    if e.call(SKILL_CHECK_0047F060, &args![skill]).u32() as u8 == 0 {
        return false;
    }
    let player = e.global::<u32>(PLAYER);
    let current = e.vcall(player + 0xa4, 0x1c, &args![skill]).i32();
    if current >= 0xc8 {
        return false;
    }
    e.with_stack(0xc, |e, locals| {
        let points_at = locals.addr();
        let message = locals.addr() + 4;
        let setting = e
            .call(SETTING_FLOAT_ADDRESS, &args![BOOK_SKILL_POINTS_SETTING])
            .u32();
        let points = x87_quiet_bits(e.mem.u32(setting));
        e.mem.set_u32(points_at, points);
        e.call(STRING_CONSTRUCT, &args![message]);
        let player = e.global::<u32>(PLAYER);
        e.call(HANDLE_ENTRY_POINT, &args![0xbu32, player, points_at]);
        let points = x87_quiet_bits(e.mem.u32(points_at));
        let player = e.global::<u32>(PLAYER);
        e.vcall(player, 0x3a4, &args![skill, points, 0u32]);
        e.call(MISC_STAT_INCREMENT, &args![0xfu32]);
        let value = x87_load32(e.mem.u32(points_at));
        let whole = e.call(FTOL2, &args![value]).u32();
        let name = e.call(ACTOR_VALUE_NAME, &args![skill]).u32();
        let format = e.call(SETTING_STRING, &args![SKILL_MESSAGE_SETTING]).u32();
        e.call(STRING_FORMAT, &args![message, format, name, whole]);
        let text = e.call(STRING_TEXT, &args![message]).u32();
        show_message(e, text, 1);
        e.call(STRING_DESTRUCT, &args![message]);
    });
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_4870, tes_object_book_init_item(Ptr)),
        entry!(0x0051_4930, fn_00514930(Ptr, Ptr) -> bool),
        entry!(0x0051_4f30, fn_00514f30(Ptr) -> u32),
        entry!(0x0051_4de0, tes_object_book_save(Ptr)),
        entry!(0x0051_4ed0, fn_00514ed0(Ptr, u32)),
        entry!(0x0051_4f50, fn_00514f50(Ptr, u32) -> bool),
        entry!(0x0051_4fd0, fn_00514fd0(Ptr) -> u32),
        entry!(
            0x0051_5010,
            fn_00515010(Ptr, u32, u32, u8, u32, u32) -> u32
        ),
        entry!(0x0051_5040, tes_object_book_read(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    #[test]
    fn data_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0051_4f30, &[0x1000]).u32(), 0x10c0);
        assert_eq!(e.call(0x0051_4f30, &[0xffff_ff80]).u32(), 0x40);
    }

    #[test]
    fn init_item_checks_the_skill() {
        let mut e = Engine::new();
        e.register(FORM_FLAG_8_SET, |_, _| ret(1));
        let this = e.mem.alloc(0xc4);
        e.call_log = Some(vec![]);
        e.call(0x0051_4870, &[this]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);

        e.register(FORM_FLAG_8_SET, |_, _| ret(0));
        let inits = [
            0x0048_cd90,
            0x0048_3210,
            0x0047_8be0,
            0x0048_aa80,
            0x0047_b6a0,
        ];
        quiet(&mut e, &inits);
        quiet(&mut e, &[SET_FORM_FLAG_8]);
        // The actor value is the byte itself.
        e.register(TO_ACTOR_VALUE, |_, a| {
            assert_eq!(a[0], 2);
            ret(a[1])
        });
        for (skill, kept) in [
            (0x20u8, true),
            (0x2e, true),
            (0x1f, false),
            (0x2f, false),
            (0xff, true),
        ] {
            e.mem.set_u8(this + 0xc1, skill);
            e.call_log = Some(vec![]);
            e.call(0x0051_4870, &[this]);
            let log = e.call_log.take().unwrap();
            let expected = if kept { skill } else { 0xff };
            assert_eq!(e.mem.u8(this + 0xc1), expected, "{skill:#x}");
            let order: Vec<u32> = log
                .iter()
                .filter(|(a, _)| inits.contains(a))
                .map(|(_, w)| w[0] - this)
                .collect();
            assert_eq!(order, vec![0x68, 0x74, 0x9c, 0x3c, 0xb4]);
            assert_eq!(
                calls_to(&log, TO_ACTOR_VALUE).len(),
                usize::from(skill != 0xff)
            );
            assert_eq!(calls_to(&log, SET_FORM_FLAG_8), vec![vec![this, 1]]);
        }
    }

    #[test]
    fn load_reads_every_chunk_kind() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc4);
        let file = 0x0500_0000u32;
        e.register(GET_TES_FORM, |_, _| ret(0x18));
        assert!(!e.call(0x0051_4930, &[this, file]).bool());

        e.register(GET_TES_FORM, |_, _| ret(0x19));
        let mut chunks = vec![
            SIG_EDID, SIG_OBND, SIG_FULL, SIG_MODL, SIG_MODD, SIG_MOSD, SIG_MODT, SIG_MODS,
            SIG_ICON, SIG_MICO, SIG_SCRI, SIG_ENAM, SIG_EITM, SIG_ANAM, SIG_EAMT, SIG_DESC,
            SIG_DSTD, SIG_DEST, SIG_YNAM, SIG_ZNAM, SIG_DATA, 0x1234,
        ];
        chunks.reverse();
        let chunks = Rc::new(RefCell::new(chunks));
        let c = chunks.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(c.borrow_mut().pop().unwrap_or(0))
        });
        let c = chunks.clone();
        e.register_double(NEXT_CHUNK, move |_, _| {
            ret(u32::from(!c.borrow().is_empty()))
        });
        e.register(CHUNK_SIZE, |_, _| ret(8));
        let n = Rc::new(RefCell::new(0x500u32));
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            let mut k = n.borrow_mut();
            *k += 1;
            e.mem.set_u32(a[1], *k);
            Ret::default()
        });
        e.register(GET_CHUNK_DATA_U16, |e, a| {
            assert_eq!(e.mem.u16(a[1]), 0);
            e.mem.set_u16(a[1], 0x66);
            Ret::default()
        });
        e.register(0x0040_3570, |_, _| ret(0xffff_000f));
        quiet(
            &mut e,
            &[
                LOAD_FORM,
                SET_FORM_FLAG_8,
                GET_CHUNK_DATA_SIZED,
                0x7000_0000,
                0x7000_0004,
                0x0048_7050,
                0x0048_92d0,
                0x0048_a7b0,
                0x0048_e3d0,
                0x0047_a290,
                SET_FORM,
                0x0048_cd90,
                0x0048_3170,
                0x0048_2d50,
                0x0047_81e0,
                0x0040_3550,
                LOAD_DATA,
            ],
        );
        let mut slots = vec![0u32; 0x138 / 4];
        slots[0x134 / 4] = 0x7000_0000;
        slots[0xe0 / 4] = 0x7000_0004;
        e.put_vtable(0x0600_0000, &slots);
        e.mem.set_u32(this, 0x0600_0000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0051_4930, &[this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOAD_FORM), vec![vec![this, file]]);
        assert_eq!(calls_to(&log, SET_FORM_FLAG_8), vec![vec![this, 0]]);
        let sized = calls_to(&log, GET_CHUNK_DATA_SIZED);
        assert_eq!((sized[0][2], sized[1][2]), (0x200, 8));
        assert_eq!(calls_to(&log, 0x7000_0000).len(), 1);
        assert_eq!(calls_to(&log, 0x7000_0004), vec![vec![this, file]]);
        assert_eq!(calls_to(&log, 0x0048_7050), vec![vec![this + 0x30, file]]);
        assert_eq!(
            calls_to(&log, 0x0048_92d0),
            vec![vec![this + 0x3c, file]; 4]
        );
        assert_eq!(
            calls_to(&log, 0x0048_a7b0),
            vec![vec![this + 0x3c, sized[1][1], 8, 0xf]]
        );
        assert_eq!(calls_to(&log, 0x0048_e3d0), vec![vec![this + 0x5c, file]]);
        assert_eq!(calls_to(&log, 0x0047_a290), vec![vec![this + 0xa4, file]]);
        assert_eq!(
            calls_to(&log, SET_FORM),
            vec![
                vec![this + 0x68, 0x501],
                vec![this + 0x74, 0x502],
                vec![this + 0x74, 0x503],
                vec![this + 0xb4, 0x504],
            ]
        );
        assert_eq!(calls_to(&log, 0x0048_cd90), vec![vec![this + 0x68, this]]);
        assert_eq!(
            calls_to(&log, 0x0048_3170),
            vec![vec![this + 0x74, 0x66]; 2]
        );
        assert_eq!(calls_to(&log, 0x0048_2d50), vec![vec![this + 0x94, file]]);
        assert_eq!(
            calls_to(&log, 0x0047_81e0),
            vec![vec![this + 0x9c, file]; 2]
        );
        assert_eq!(calls_to(&log, 0x0040_3550), vec![vec![this + 0xb4, 0x505]]);
        assert_eq!(
            calls_to(&log, LOAD_DATA),
            vec![vec![this, file, this + 0xc0, 2]]
        );
    }
}

#[cfg(test)]
mod save_read_tests {
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

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    #[test]
    fn save_writes_components_and_data() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                START_FORM,
                CLOSE_FORM,
                FULL_NAME_SAVE,
                MODEL_TEXTURE_SWAP_SAVE,
                TEXTURE_SAVE,
                MESSAGE_ICON_SAVE,
                SCRIPTABLE_FORM_SAVE,
                ENCHANTABLE_SAVE,
                DESCRIPTION_SAVE,
                DESTRUCTIBLE_SAVE,
                PICKUP_PUTDOWN_SAVE,
                SWAP_BOOK_DATA,
                SAVE_DATA,
            ],
        );
        e.register(SAVE_SWAPS_BYTES, |_, _| ret(1));
        let this = e.mem.alloc(0xc4);
        let (_, log) = run(&mut e, 0x0051_4de0, &args![this]);
        assert_eq!(
            only(&log, MODEL_TEXTURE_SWAP_SAVE),
            vec![vec![this + 0x3c, SIG_MODL, SIG_MODT, SIG_MODS]]
        );
        assert_eq!(only(&log, TEXTURE_SAVE), vec![vec![this + 0x5c, SIG_ICON]]);
        assert_eq!(
            only(&log, MESSAGE_ICON_SAVE),
            vec![vec![this + 0xa4, SIG_MICO]]
        );
        for (save, offset) in [
            (FULL_NAME_SAVE, 0x30),
            (SCRIPTABLE_FORM_SAVE, 0x68),
            (ENCHANTABLE_SAVE, 0x74),
            (DESCRIPTION_SAVE, 0x94),
            (DESTRUCTIBLE_SAVE, 0x9c),
            (PICKUP_PUTDOWN_SAVE, 0xb4),
        ] {
            assert_eq!(only(&log, save), vec![vec![this + offset]]);
        }
        assert_eq!(only(&log, SAVE_DATA), vec![vec![this, this + 0xc0, 2]]);
        assert_eq!(only(&log, SWAP_BOOK_DATA).len(), 2);
        assert_eq!(only(&log, CLOSE_FORM), vec![vec![this]]);
    }

    #[test]
    fn copy_and_compare_use_the_data_word() {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(0x100));
        e.register(MEMCMP, |e, a| {
            let differ = (0..a[2]).any(|i| e.mem.u8(a[0] + i) != e.mem.u8(a[1] + i));
            ret(u32::from(differ))
        });
        let this = e.mem.alloc(0xc4);
        let other = e.mem.alloc(0xc4);
        e.mem.set_u32(other + 0xc0, 0xaabb_1234);
        e.mem.set_u32(this + 0xc0, 0xccdd_0000);
        assert!(e.call(0x0051_4f50, &args![this, other]).bool());
        let (_, log) = run(&mut e, 0x0051_4ed0, &args![this, other]);
        assert_eq!(only(&log, COPY_ALL_COMPONENTS), vec![vec![this, other]]);
        assert_eq!(e.mem.u32(this + 0xc0), 0xccdd_1234);
        assert!(!e.call(0x0051_4f50, &args![this, other]).bool());
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(1));
        assert!(e.call(0x0051_4f50, &args![this, other]).bool());
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(e.call(0x0051_4f50, &args![this, other]).bool());
        e.mem.set_u16(other + 0xc0, 0x5555);
        e.call(0x0051_4ed0, &args![this, other]);
        assert_eq!(e.mem.u16(this + 0xc0), 0x1234);
    }

    #[test]
    fn skill_is_minus_one_or_the_actor_value() {
        let mut e = Engine::new();
        e.register(TO_ACTOR_VALUE, |_, a| ret(a[0] * 0x100 + a[1]));
        let this = e.mem.alloc(0xc4);
        e.mem.set_u8(this + 0xc1, 0xff);
        assert_eq!(e.call(0x0051_4fd0, &args![this]).u32(), 0xffff_ffff);
        e.mem.set_u8(this + 0xc1, 0x8e);
        assert_eq!(e.call(0x0051_4fd0, &args![this]).u32(), 0x28e);
    }

    #[test]
    fn forwarder_passes_five_words_with_a_byte() {
        let mut e = Engine::new();
        e.register(OBJECT_ACTIVATE_0050F670, |_, a| ret(a[3] + a[5]));
        let (r, log) = run(
            &mut e,
            0x0051_5010,
            &args![0x500u32, 1u32, 2u32, 0x1ffu32, 4u32, 5u32],
        );
        assert_eq!(
            only(&log, OBJECT_ACTIVATE_0050F670),
            vec![vec![0x500, 1, 2, 0xff, 4, 5]]
        );
        assert_eq!(r.eax, 0xff + 5);
    }

    /// The reader actor: vtable slot `+0x360` answers `can_read`; the
    /// player's `+0xa4` part answers `skill_value` from slot `+0x1c`.
    fn read_engine(can_read: u32, busy: u32, skill_value: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        let actor = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x400);
        e.mem.set_u32(actor, vtable);
        e.mem.set_u32(vtable + 0x360, 0x0f46_0360);
        e.register_double(0x0f46_0360, move |_, _| ret(can_read));
        e.register_double(ACTOR_CHECK_00493BB0, move |_, _| ret(busy));
        e.register(IS_IN_MENU_MODE, |_, _| ret(0));
        e.register(SETTING_STRING, |_, a| ret(a[0] + 1));
        e.register(SHOW_MESSAGE, |_, _| Ret::default());
        e.register(TO_ACTOR_VALUE, |_, a| ret(a[1]));
        e.register(SKILL_CHECK_0047F060, |_, a| ret(u32::from(a[0] == 0x2b)));
        e.map(PLAYER & !0xfff, 0x1000);
        let player = e.mem.alloc(0x100);
        e.set_global::<u32>(PLAYER, player);
        let player_vtable = e.mem.alloc(0x400);
        e.mem.set_u32(player, player_vtable);
        e.mem.set_u32(player_vtable + 0x3a4, 0x0f46_03a4);
        e.register(0x0f46_03a4, |_, _| Ret::default());
        let part_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(player + 0xa4, part_vtable);
        e.mem.set_u32(part_vtable + 0x1c, 0x0f46_001c);
        e.register_double(0x0f46_001c, move |_, _| ret(skill_value));
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, 1.0f32.to_bits());
        e.register_double(SETTING_FLOAT_ADDRESS, move |_, _| ret(setting));
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            let v = f32::from_bits(e.mem.u32(a[2])) * 2.0;
            e.mem.set_u32(a[2], v.to_bits());
            Ret::default()
        });
        e.register(MISC_STAT_INCREMENT, |_, _| Ret::default());
        e.register(FTOL2, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            ret(value as i32 as u32)
        });
        e.register(ACTOR_VALUE_NAME, |_, a| ret(0x4000 + a[0]));
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_FORMAT, |_, _| Ret::default());
        e.register(STRING_TEXT, |_, a| ret(a[0] + 0x10));
        e.register(STRING_DESTRUCT, |_, _| Ret::default());
        e.map(MESSAGE_FLOAT & !0xfff, 0x1000);
        e.set_global::<u32>(MESSAGE_FLOAT, 2.0f32.to_bits());
        let book = e.mem.alloc(0xc4);
        e.mem.set_u8(book + 0xc1, 0x2b);
        (e, book, actor)
    }

    #[test]
    fn read_raises_the_skill_and_shows_the_message() {
        let (mut e, book, actor) = read_engine(1, 0, 50);
        let (r, log) = run(&mut e, 0x0051_5040, &args![book, actor]);
        assert!(r.eax & 0xff != 0);
        assert_eq!(
            only(&log, HANDLE_ENTRY_POINT)[0][..2],
            [0xb, e.global::<u32>(PLAYER)]
        );
        assert_eq!(only(&log, 0x0f46_03a4)[0][1..], [0x2b, 2.0f32.to_bits(), 0]);
        assert_eq!(only(&log, MISC_STAT_INCREMENT), vec![vec![0xf]]);
        let format = only(&log, STRING_FORMAT);
        assert_eq!(format[0][1..], [SKILL_MESSAGE_SETTING + 1, 0x402b, 2]);
        let message = format[0][0];
        assert_eq!(
            only(&log, SHOW_MESSAGE),
            vec![vec![message + 0x10, 1, 0, 0, 2.0f32.to_bits(), 0]]
        );
        assert_eq!(only(&log, STRING_DESTRUCT), vec![vec![message]]);
    }

    #[test]
    fn read_stops_early_in_each_case() {
        let (mut e, book, actor) = read_engine(0x100, 0, 50);
        let (r, log) = run(&mut e, 0x0051_5040, &args![book, actor]);
        assert_eq!(r.eax & 0xff, 0);
        assert!(only(&log, SHOW_MESSAGE).is_empty());
        let (mut e, book, actor) = read_engine(1, 1, 50);
        let (r, log) = run(&mut e, 0x0051_5040, &args![book, actor]);
        assert_eq!(r.eax & 0xff, 0);
        assert_eq!(
            only(&log, SHOW_MESSAGE),
            vec![vec![CANNOT_READ_SETTING + 1, 0, 0, 0, 2.0f32.to_bits(), 0]]
        );
        let (mut e, book, actor) = read_engine(1, 0, 200);
        let (r, log) = run(&mut e, 0x0051_5040, &args![book, actor]);
        assert_eq!(r.eax & 0xff, 0);
        assert!(only(&log, HANDLE_ENTRY_POINT).is_empty());
        let (mut e, book, actor) = read_engine(1, 0, 50);
        e.mem.set_u8(book + 0xc1, 0x2c);
        let (r, log) = run(&mut e, 0x0051_5040, &args![book, actor]);
        assert_eq!(r.eax & 0xff, 0);
        assert!(only(&log, 0x0f46_001c).is_empty());
    }
}
