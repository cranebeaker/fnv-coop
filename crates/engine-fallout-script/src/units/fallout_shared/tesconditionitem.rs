//! `fallout shared/tesconditionitem.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! One condition item's record handling: `TESConditionItem::Load` (one
//! `CTDA` chunk), `Save` and `InitItem` (resolving the run-on reference and
//! the comparison global, and the warnings when they or the function are
//! bad), and the byte swap of an item's words (`00681170`).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `TESConditionItem` (Xbox PDB), 0x1c bytes; its one member is
    /// `CONDITION_ITEM_DATA Data` at +0, whose fields are declared here (the
    /// PC code uses the same offsets).
    pub struct TesConditionItem: 0x1c {
        /// `Data.iFlags` (Xbox PDB): the flag byte (bit 2: the comparison
        /// value is a global; bits 5-7: the comparison operator, as
        /// `00681530` stores it).
        0x00 iFlags: u8,
        /// `Data.pGlobal` (Xbox PDB): `TESGlobal*` (the form id until
        /// `InitItem`), when flag bit 2 is set.
        0x04 pGlobal: u32,
        /// `Data.fValue` (Xbox PDB): the comparison value otherwise (the
        /// same word as `pGlobal`).
        0x04 fValue: f32,
        /// `Data.FunctionData.iFunction` (Xbox PDB): the function number.
        0x08 iFunction: u16,
        /// `Data.FunctionData.pParam[0]` (Xbox PDB).
        0x0c pParam0: u32,
        /// `Data.FunctionData.pParam[1]` (Xbox PDB).
        0x10 pParam1: u32,
        /// `Data.eObject` (Xbox PDB): `CONDITIONITEMOBJECT` (0 self, 1
        /// target, 2 reference, 3 combat target, 4 linked reference).
        0x14 eObject: u32,
        /// `Data.pRunOnRef` (Xbox PDB): `TESObjectREFR*` (the form id until
        /// `InitItem`).
        0x18 pRunOnRef: u32,
    }
}

/// The offset of `Data.FunctionData.pParam` (Xbox PDB) in the item.
const PARAMS_OFFSET: u32 = 0x0c;
/// The chunk tag `CTDA` (one condition item).
const CHUNK_CTDA: u32 = 0x4144_5443;
/// `TESFile::GetTESChunk` (`004726b0`, `ECX` = file): the current chunk's tag.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `TESFile::GetChunkData_ov3` (`00472890`, `ECX` = file; destination, size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `00401680` (`ECX` = file): the file's byte at +0x299, nonzero when its
/// data is stored the other way round.
const FILE_IS_SWAPPED: u32 = 0x0040_1680;
/// `00401500` (no arguments): the byte at `011c54ba`, nonzero when saved
/// data is stored the other way round.
const IS_SWAPPED: u32 = 0x0040_1500;
/// `00600ab0` (`ECX` = item, one word; `RET 4`): stores the word at +0x14
/// (`eObject`).
const SET_OBJECT: u32 = 0x0060_0ab0;
/// `00401080` (cdecl: address, a byte passed as a word): swaps the bytes of
/// the word at the address.
const SWAP_WORD: u32 = 0x0040_1080;
/// `00681110` (`ECX` = the item's function data at +8): swaps the function
/// data (translated in `run_00680fe0.rs`).
const SWAP_FUNCTION_DATA: u32 = 0x0068_1110;
/// `00401460` (cdecl: destination, source, size): a copy.
const COPY: u32 = 0x0040_1460;
/// `00413f40` (`ECX` = item): the address of the item's function data
/// (+8).
const FUNCTION_DATA: u32 = 0x0041_3f40;
/// `00682270` (cdecl: function data): the function's parameter count.
const PARAM_COUNT: u32 = 0x0068_2270;
/// `TESParameters::GetParamIsTESForm` (`006822a0`, cdecl: function data,
/// index).
const PARAM_IS_FORM: u32 = 0x0068_22a0;
/// `00681fb0` (`TESParameters::InitItem` in the map; cdecl: owner, function
/// data).
const PARAMS_INIT_ITEM: u32 = 0x0068_1fb0;
/// `006814e0` (`ECX` = item): whether flag bit 2 is set (the comparison
/// value is a global).
const USES_GLOBAL: u32 = 0x0068_14e0;
/// `0084e3a0` (`ECX` = form): the word at +0xc (`TESForm` form id).
const FORM_ID: u32 = 0x0084_e3a0;
/// `TESForm::__AddChunkData` (`00485990`, cdecl: tag, data, size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `TESForm::GetFile` (`00484e60`, `ECX` = form, one word: an index).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (`00485d50`, cdecl: address of a form id,
/// file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (cdecl: form id): the form with that id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, vfptr offset, source type,
/// target type, reference flag).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `TESForm` type descriptor (`01183028`), the cast's source type.
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// The cast target for the run-on reference (`011841cc`).
const TYPE_RUN_ON_REF: u32 = 0x0118_41cc;
/// The cast target for the comparison global (`01183ad4`).
const TYPE_GLOBAL: u32 = 0x0118_3ad4;
/// The cast target for an owner of form type 0x46 (`01184738`).
const TYPE_OWNER_0X46: u32 = 0x0118_4738;
/// `005b5e40` (cdecl: format, arguments...): writes a warning line.
const LOG_WARNING: u32 = 0x005b_5e40;
/// `00440e30` (`ECX` = form): the word at `01187004 + 0xc *` form type (a
/// string per form type; the first `%s` of the
/// owner warnings).
const FORM_STRING: u32 = 0x0044_0e30;
/// The owner's virtual slot 0x130: a string (the second `%s`).
const SLOT_NAME_0X130: u32 = 0x130;
/// The quest's virtual slot 0x138: a string (the quest `%s`).
const SLOT_NAME_0X138: u32 = 0x138;
/// `00401170` (`ECX` = form): the byte at +4 (`TESForm` form type).
const FORM_TYPE: u32 = 0x0040_1170;
/// The owner form type that gets the topic-info description.
const OWNER_TYPE_0X46: u32 = 0x46;
/// `TESTopic::GetOwnerTopic` (`0061a310`, cdecl: the topic info).
const GET_OWNER_TOPIC: u32 = 0x0061_a310;
/// `00619d00` (`ECX` = topic, one word: the topic info; `RET 4`): the quest
/// the warning names.
const TOPIC_QUEST: u32 = 0x0061_9d00;
/// The `BSString` constructor (`004037b0`, `ECX` = string).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// The `BSString` destructor (`004037d0`, `ECX` = string).
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `TESTopicInfo::GetInfoDisplayText` (`0061e2b0`, `ECX` = info; string, a
/// word; `RET 8`).
const GET_INFO_DISPLAY_TEXT: u32 = 0x0061_e2b0;
/// `00559450` (`ECX` = string): the string's characters.
const STRING_CHARS: u32 = 0x0055_9450;
/// `sprintf` (`00ec623a`, cdecl).
const SPRINTF: u32 = 0x00ec_623a;
/// `_snprintf` (`00eca38d`, cdecl: buffer, size, format, ...).
const SNPRINTF: u32 = 0x00ec_a38d;
/// The empty string at `01011584` (for a missing topic or quest).
const EMPTY_STRING: u32 = 0x0101_1584;
/// `"TopicInfo %08X \"%s\" in Quest %s (%08x) Topic %s (%08x)"` (`0104cae0`).
const FORMAT_TOPIC_INFO: u32 = 0x0104_cae0;
/// `"Form %s %s (%08X)"` (`01069440`).
const FORMAT_FORM: u32 = 0x0106_9440;
/// The missing run-on reference warning with an owner (`010694b0`).
const FORMAT_NO_REFERENCE_OWNER: u32 = 0x0106_94b0;
/// The missing run-on reference warning without an owner (`01069458`).
const FORMAT_NO_REFERENCE: u32 = 0x0106_9458;
/// The "is not a condition function" warning (`010693d8`).
const FORMAT_NOT_A_CONDITION: u32 = 0x0106_93d8;
/// The missing comparison global warning (`01069378`).
const FORMAT_NO_GLOBAL: u32 = 0x0106_9378;
/// The function table (`01190910`, 0x28 bytes per function number): +0 the
/// name, +0x20 a word that is zero for a function that is not a condition
/// function.
const FUNCTION_TABLE: u32 = 0x0119_0910;
const FUNCTION_ENTRY_SIZE: u32 = 0x28;
const FUNCTION_ENTRY_CONDITION: u32 = 0x20;
/// `005eb680` (`ECX` = item): the function number at +8, zero-extended.
const FUNCTION_NUMBER: u32 = 0x005e_b680;
/// The `GetFile` index word the exe passes (-1).
const ANY_FILE: u32 = 0xffff_ffff;
/// `00891170` (`ECX` = file): the address of the file's +0x20 (the
/// `%s` after "in file").
const FILE_NAME: u32 = 0x0089_1170;
/// The size of the warning buffers.
const DESCRIPTION_SIZE: u32 = 0x410;

// Translated from 00681170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the byte order of an item (`ECX`, no stack words): the words at
/// +4, +0x18 and +0x14 (`00401080` with a zero byte), then the function
/// data at +8 (`00681110`).
pub fn fn_00681170(e: &mut Engine, this: Ptr<TesConditionItem>) {
    e.call(SWAP_WORD, &args![this.addr() + 0x04, 0u32]);
    e.call(SWAP_WORD, &args![this.addr() + 0x18, 0u32]);
    e.call(SWAP_WORD, &args![this.addr() + 0x14, 0u32]);
    e.call(SWAP_FUNCTION_DATA, &args![this.addr() + 0x08]);
}

// Translated from 00681840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESConditionItem::Save` (Xbox PDB): writes the item (`ECX`, no stack
/// words) as a `CTDA` chunk. A copy of the 0x1c bytes is saved, with every
/// nonzero parameter the function declares a form (`006822a0`) replaced by
/// its form id, the global (when flag bit 2 is set) and the run-on
/// reference too; the copy is byte-swapped around `__AddChunkData` when
/// `00401500` says so.
///
/// The copy sits in the game's frame below the function data pointer and
/// the parameter count, as in the exe, so a count above 2 reaches them.
pub fn tes_condition_item_save(e: &mut Engine, this: Ptr<TesConditionItem>) {
    // Frame: the copy (0x1c bytes), the function data word, the count.
    e.with_stack(0x24, |e, copy| {
        e.call(COPY, &args![copy, this, 0x1cu32]);
        let function = e.call(FUNCTION_DATA, &args![this]).u32();
        e.mem.set_u32(copy.addr() + 0x1c, function);
        let count = e.call(PARAM_COUNT, &args![function]).u32();
        e.mem.set_u32(copy.addr() + 0x20, count);
        let mut index: i32 = 0;
        while index < e.mem.u32(copy.addr() + 0x20) as i32 {
            let is_form = e
                .call(PARAM_IS_FORM, &args![e.mem.u32(copy.addr() + 0x1c), index])
                .u8();
            if is_form != 0 {
                let slot = copy
                    .addr()
                    .wrapping_add(PARAMS_OFFSET)
                    .wrapping_add((index as u32).wrapping_mul(4));
                let param = e.mem.u32(slot);
                if param != 0 {
                    let id = e.call(FORM_ID, &args![param]).u32();
                    e.mem.set_u32(slot, id);
                }
            }
            index += 1;
        }
        if e.call(USES_GLOBAL, &args![this]).u8() != 0 {
            let global = e.get(this, TesConditionItem::pGlobal);
            if global != 0 {
                let id = e.call(FORM_ID, &args![global]).u32();
                e.mem.set_u32(copy.addr() + 0x04, id);
            }
        }
        let run_on = e.get(this, TesConditionItem::pRunOnRef);
        if run_on != 0 {
            let id = e.call(FORM_ID, &args![run_on]).u32();
            e.mem.set_u32(copy.addr() + 0x18, id);
        }
        if e.call(IS_SWAPPED, &args![]).u8() != 0 {
            fn_00681170(e, Ptr::new(copy.addr()));
        }
        e.call(ADD_CHUNK_DATA, &args![CHUNK_CTDA, copy, 0x1cu32]);
        if e.call(IS_SWAPPED, &args![]).u8() != 0 {
            fn_00681170(e, Ptr::new(copy.addr()));
        }
    });
}

// Translated from 00681950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESConditionItem::Load` (Xbox PDB): with a `file` (`RET 4`) whose
/// current chunk is `CTDA`, reads the chunk's 0x1c bytes into the item
/// (`ECX`) and swaps them when the file is stored the other way round.
/// Then, with or without a file, flag bit 1 becomes `eObject` 1 and flag
/// bit 3 `eObject` 2 (`00600ab0`), each bit cleared.
pub fn tes_condition_item_load(e: &mut Engine, this: Ptr<TesConditionItem>, file: Ptr) {
    if !file.is_null() && e.call(GET_TES_CHUNK, &args![file]).u32() == CHUNK_CTDA {
        e.call(GET_CHUNK_DATA, &args![file, this, 0x1cu32]);
        if e.call(FILE_IS_SWAPPED, &args![file]).u8() != 0 {
            fn_00681170(e, this);
        }
    }
    if e.get(this, TesConditionItem::iFlags) & 0x02 != 0 {
        e.call(SET_OBJECT, &args![this, 1u32]);
        let flags = e.get(this, TesConditionItem::iFlags);
        e.set(this, TesConditionItem::iFlags, flags & 0xfd);
    }
    if e.get(this, TesConditionItem::iFlags) & 0x08 != 0 {
        e.call(SET_OBJECT, &args![this, 2u32]);
        let flags = e.get(this, TesConditionItem::iFlags);
        e.set(this, TesConditionItem::iFlags, flags & 0xf7);
    }
}

/// The owner description both warnings of `InitItem` build into `buffer`
/// (the exe's two copies of this code differ only in `sprintf` against
/// `_snprintf` with 0x410).
fn describe_owner(e: &mut Engine, owner: u32, buffer: Ptr, bounded: bool) {
    let print = |e: &mut Engine, rest: &[u32]| {
        let mut words = vec![buffer.addr()];
        if bounded {
            words.push(DESCRIPTION_SIZE);
        }
        words.extend_from_slice(rest);
        e.call(if bounded { SNPRINTF } else { SPRINTF }, &words);
    };
    if e.call(FORM_TYPE, &args![owner]).u32() == OWNER_TYPE_0X46 {
        let info = e
            .call(
                DYNAMIC_CAST,
                &args![owner, 0u32, TYPE_TES_FORM, TYPE_OWNER_0X46, 0u32],
            )
            .u32();
        let topic = e.call(GET_OWNER_TOPIC, &args![info]).u32();
        let quest = if topic != 0 {
            e.call(TOPIC_QUEST, &args![topic, info]).u32()
        } else {
            0
        };
        e.with_stack(8, |e, text| {
            e.call(STRING_CONSTRUCT, &args![text]);
            e.call(GET_INFO_DISPLAY_TEXT, &args![info, text, 0u32]);
            let topic_id = if topic != 0 {
                e.call(FORM_ID, &args![topic]).u32()
            } else {
                0
            };
            let topic_name = if topic != 0 {
                e.vcall(topic, SLOT_NAME_0X130, &args![]).u32()
            } else {
                EMPTY_STRING
            };
            let quest_id = if quest != 0 {
                e.call(FORM_ID, &args![quest]).u32()
            } else {
                0
            };
            let quest_name = if quest != 0 {
                e.vcall(quest, SLOT_NAME_0X138, &args![]).u32()
            } else {
                EMPTY_STRING
            };
            let chars = e.call(STRING_CHARS, &args![text]).u32();
            let info_id = e.call(FORM_ID, &args![info]).u32();
            print(
                e,
                &[
                    FORMAT_TOPIC_INFO,
                    info_id,
                    chars,
                    quest_name,
                    quest_id,
                    topic_name,
                    topic_id,
                ],
            );
            e.call(STRING_DESTRUCT, &args![text]);
        });
    } else {
        let id = e.call(FORM_ID, &args![owner]).u32();
        let name = e.vcall(owner, SLOT_NAME_0X130, &args![]).u32();
        let string = e.call(FORM_STRING, &args![owner]).u32();
        print(e, &[FORMAT_FORM, string, name, id]);
    }
}

// Translated from 006819e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006819e0 - the exception-unwinding frame (FS:[0]) for the topic-info display string and the security-cookie check (00ec408c) are not translated.
/// `TESConditionItem::InitItem` (Xbox PDB): resolves the item (`ECX`) after
/// loading, for `owner` (the one stack word, `RET 4`). The parameters go
/// through `00681fb0`; a run-on reference id is made file-relative
/// (`AddCompileIndex` with the owner's file), looked up and cast, and a
/// warning is written when nothing is found (naming the owner, or
/// "UNKNOWN owner" without one). When the function table marks the item's
/// function as no condition function, a warning names the owner, its file
/// and the function. With flag bit 2 set, the comparison global's id is
/// resolved the same way, with a warning when it is missing.
pub fn tes_condition_item_init_item(e: &mut Engine, this: Ptr<TesConditionItem>, owner: u32) {
    let function = e.call(FUNCTION_DATA, &args![this]).u32();
    e.call(PARAMS_INIT_ITEM, &args![owner, function]);
    let run_on = e.get(this, TesConditionItem::pRunOnRef);
    if run_on != 0 {
        // The form id is a local of the game's stack, passed by address.
        e.with_stack(4, |e, id_slot| {
            e.mem.set_u32(id_slot.addr(), run_on);
            let file = e.call(GET_FILE, &args![owner, ANY_FILE]).u32();
            e.call(ADD_COMPILE_INDEX, &args![id_slot, file]);
            let form = e.call(LOOKUP_FORM, &args![e.mem.u32(id_slot.addr())]).u32();
            let cast = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_RUN_ON_REF, 0u32],
                )
                .u32();
            e.set(this, TesConditionItem::pRunOnRef, cast);
            if e.get(this, TesConditionItem::pRunOnRef) == 0 {
                let id = e.mem.u32(id_slot.addr());
                if owner != 0 {
                    let owner_id = e.call(FORM_ID, &args![owner]).u32();
                    let name = e.vcall(owner, SLOT_NAME_0X130, &args![]).u32();
                    let string = e.call(FORM_STRING, &args![owner]).u32();
                    e.call(
                        LOG_WARNING,
                        &args![FORMAT_NO_REFERENCE_OWNER, id, string, name, owner_id],
                    );
                } else {
                    e.call(LOG_WARNING, &args![FORMAT_NO_REFERENCE, id]);
                }
            }
        });
    }
    let number = e.call(FUNCTION_NUMBER, &args![this]).u32();
    let entry = FUNCTION_TABLE.wrapping_add(number.wrapping_mul(FUNCTION_ENTRY_SIZE));
    if e.mem.u32(entry.wrapping_add(FUNCTION_ENTRY_CONDITION)) == 0 {
        e.with_stack(DESCRIPTION_SIZE, |e, buffer| {
            describe_owner(e, owner, buffer, false);
            let number = e.call(FUNCTION_NUMBER, &args![this]).u32();
            let entry = FUNCTION_TABLE.wrapping_add(number.wrapping_mul(FUNCTION_ENTRY_SIZE));
            let function_name = e.mem.u32(entry);
            let file = e.call(GET_FILE, &args![owner, ANY_FILE]).u32();
            let file_name = e.call(FILE_NAME, &args![file]).u32();
            e.call(
                LOG_WARNING,
                &args![FORMAT_NOT_A_CONDITION, buffer, file_name, function_name],
            );
        });
    }
    if e.call(USES_GLOBAL, &args![this]).u8() != 0 && e.get(this, TesConditionItem::pGlobal) != 0 {
        e.with_stack(4, |e, id_slot| {
            let global = e.get(this, TesConditionItem::pGlobal);
            e.mem.set_u32(id_slot.addr(), global);
            let file = e.call(GET_FILE, &args![owner, ANY_FILE]).u32();
            e.call(ADD_COMPILE_INDEX, &args![id_slot, file]);
            let form = e.call(LOOKUP_FORM, &args![e.mem.u32(id_slot.addr())]).u32();
            let cast = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_GLOBAL, 0u32],
                )
                .u32();
            e.set(this, TesConditionItem::pGlobal, cast);
            if e.get(this, TesConditionItem::pGlobal) == 0 {
                e.with_stack(DESCRIPTION_SIZE, |e, buffer| {
                    describe_owner(e, owner, buffer, true);
                    let id = e.mem.u32(id_slot.addr());
                    e.call(LOG_WARNING, &args![FORMAT_NO_GLOBAL, id, buffer]);
                });
            }
        });
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00681170, fn_00681170(Ptr<TesConditionItem>)),
        entry!(0x00681840, tes_condition_item_save(Ptr<TesConditionItem>)),
        entry!(
            0x00681950,
            tes_condition_item_load(Ptr<TesConditionItem>, Ptr)
        ),
        entry!(
            0x006819e0,
            tes_condition_item_init_item(Ptr<TesConditionItem>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// The virtual slots of the test forms (slot 0x130 and 0x138 doubles).
    const NAME_0X130: u32 = 0x7fff_0130;
    const NAME_0X138: u32 = 0x7fff_0138;

    /// An engine with every callee outside this file doubled: form ids are
    /// the word at +0xc, form types the byte at +4, the file's current
    /// chunk is its first word and its swap byte its second.
    fn setup() -> Engine {
        let mut e = Engine::new();
        for addr in [
            SWAP_WORD,
            SWAP_FUNCTION_DATA,
            GET_CHUNK_DATA,
            SET_OBJECT,
            COPY,
            ADD_CHUNK_DATA,
            PARAMS_INIT_ITEM,
            ADD_COMPILE_INDEX,
            LOG_WARNING,
            STRING_CONSTRUCT,
            STRING_DESTRUCT,
            GET_INFO_DISPLAY_TEXT,
            SPRINTF,
            SNPRINTF,
        ] {
            e.register(addr, |_, _| ret(0));
        }
        e.register(GET_TES_CHUNK, |e, a| ret(e.mem.u32(a[0])));
        e.register(FILE_IS_SWAPPED, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(IS_SWAPPED, |_, _| ret(0));
        e.register(FUNCTION_DATA, |_, a| ret(a[0] + 8));
        e.register(FUNCTION_NUMBER, |e, a| ret(e.mem.u16(a[0] + 8) as u32));
        e.register(USES_GLOBAL, |e, a| ret((e.mem.u8(a[0]) & 4 != 0) as u32));
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(FORM_TYPE, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register(FORM_STRING, |_, a| ret(a[0] + 0x1000));
        e.register(NAME_0X130, |_, a| ret(a[0] + 0x130));
        e.register(NAME_0X138, |_, a| ret(a[0] + 0x138));
        e.register(GET_FILE, |_, _| ret(0x5000));
        e.register(FILE_NAME, |_, a| ret(a[0] + 0x20));
        e.register(STRING_CHARS, |_, a| ret(a[0] + 1));
        // Ids 0x10..0x20 are found (the form is id + 0x100), others not.
        e.register(LOOKUP_FORM, |_, a| {
            ret(if (0x10..0x20).contains(&a[0]) {
                a[0] + 0x100
            } else {
                0
            })
        });
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.call_log = Some(vec![]);
        e
    }

    /// A form with an id, a form type and the two name slots.
    fn form(e: &mut Engine, id: u32, kind: u8) -> u32 {
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0x130, NAME_0X130);
        e.mem.set_u32(vtable + 0x138, NAME_0X138);
        let p = e.mem.alloc(0x20);
        e.mem.set_u32(p, vtable);
        e.mem.set_u8(p + 4, kind);
        e.mem.set_u32(p + 0xc, id);
        p
    }

    fn item(e: &mut Engine) -> Ptr<TesConditionItem> {
        Ptr::new(e.mem.alloc(0x1c))
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log: &Log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Marks function `number` of the function table as a condition
    /// function or not, with a name word.
    fn function_entry(e: &mut Engine, number: u32, condition: bool) {
        let entry = FUNCTION_TABLE + number * FUNCTION_ENTRY_SIZE;
        e.map(entry, FUNCTION_ENTRY_SIZE);
        e.mem.set_u32(entry, 0xabc0 + number);
        e.mem
            .set_u32(entry + FUNCTION_ENTRY_CONDITION, condition as u32);
    }

    /// A `COPY` double that copies.
    fn real_copy(e: &mut Engine) {
        e.register(COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
    }

    #[test]
    fn the_byte_swap_swaps_three_words_and_the_function_data() {
        let mut e = setup();
        let it = item(&mut e);
        e.call(0x0068_1170, &args![it]);
        let a = it.addr();
        assert_eq!(
            calls_to(&e, SWAP_WORD),
            vec![vec![a + 4, 0], vec![a + 0x18, 0], vec![a + 0x14, 0]]
        );
        assert_eq!(calls_to(&e, SWAP_FUNCTION_DATA), vec![vec![a + 8]]);
    }

    #[test]
    fn load_reads_a_ctda_chunk_and_turns_flags_into_the_object() {
        let mut e = setup();
        let it = item(&mut e);
        let file = e.mem.alloc(8);
        e.mem.set_u32(file, CHUNK_CTDA);
        e.mem.set_u32(file + 4, 1);
        e.mem.set_u8(it.addr(), 0x0a | 0x40);
        e.call(0x0068_1950, &args![it, file]);
        assert_eq!(
            calls_to(&e, GET_CHUNK_DATA),
            vec![vec![file, it.addr(), 0x1c]]
        );
        assert_eq!(calls_to(&e, SWAP_FUNCTION_DATA).len(), 1);
        assert_eq!(
            calls_to(&e, SET_OBJECT),
            vec![vec![it.addr(), 1], vec![it.addr(), 2]]
        );
        assert_eq!(e.mem.u8(it.addr()), 0x40);
    }

    #[test]
    fn load_without_the_chunk_or_a_file_only_converts_flags() {
        let mut e = setup();
        let it = item(&mut e);
        let file = e.mem.alloc(8);
        e.mem.set_u32(file, 0x4d41_4e4e);
        e.mem.set_u8(it.addr(), 0x08);
        e.call(0x0068_1950, &args![it, file]);
        assert!(calls_to(&e, GET_CHUNK_DATA).is_empty());
        assert_eq!(calls_to(&e, SET_OBJECT), vec![vec![it.addr(), 2]]);
        assert_eq!(e.mem.u8(it.addr()), 0);
        // An unswapped file, and no file: nothing swapped, no flags left.
        e.call_log = Some(vec![]);
        e.mem.set_u32(file, CHUNK_CTDA);
        e.call(0x0068_1950, &args![it, file]);
        e.call(0x0068_1950, &args![it, 0u32]);
        assert_eq!(calls_to(&e, GET_CHUNK_DATA).len(), 1);
        assert!(calls_to(&e, SWAP_WORD).is_empty());
        assert!(calls_to(&e, SET_OBJECT).is_empty());
        assert_eq!(calls_to(&e, GET_TES_CHUNK).len(), 1);
    }

    #[test]
    fn save_writes_a_copy_with_form_ids() {
        let mut e = setup();
        real_copy(&mut e);
        // Two parameters, the first a form.
        e.register(PARAM_COUNT, |_, _| ret(2));
        e.register(PARAM_IS_FORM, |_, a| ret((a[1] == 0) as u32));
        e.register(IS_SWAPPED, |_, _| ret(1));
        let written = Rc::new(RefCell::new(Vec::new()));
        let w = written.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            *w.borrow_mut() = e.mem.bytes(a[1], a[2]);
            ret(0)
        });
        let it = item(&mut e);
        let param = form(&mut e, 0x11, 0);
        let global = form(&mut e, 0x22, 6);
        let run_on = form(&mut e, 0x33, 0);
        e.mem.set_u8(it.addr(), 0x04);
        e.mem.set_u32(it.addr() + 4, global);
        e.mem.set_u32(it.addr() + 0xc, param);
        e.mem.set_u32(it.addr() + 0x10, 0x9999);
        e.mem.set_u32(it.addr() + 0x18, run_on);
        e.call(0x0068_1840, &args![it]);
        let w = written.borrow();
        let word = |o: usize| u32::from_le_bytes(w[o..o + 4].try_into().unwrap());
        assert_eq!(word(4), 0x22);
        assert_eq!(word(0xc), 0x11);
        assert_eq!(word(0x10), 0x9999);
        assert_eq!(word(0x18), 0x33);
        // The chunk and the swaps around it, on the copy, not the item.
        let chunk = calls_to(&e, ADD_CHUNK_DATA);
        assert_eq!(chunk.len(), 1);
        assert_eq!((chunk[0][0], chunk[0][2]), (CHUNK_CTDA, 0x1c));
        assert_eq!(
            calls_to(&e, SWAP_FUNCTION_DATA),
            vec![vec![chunk[0][1] + 8], vec![chunk[0][1] + 8]]
        );
        assert_eq!(e.mem.u32(it.addr() + 0xc), param);
        assert_eq!(calls_to(&e, PARAM_IS_FORM).len(), 2);
    }

    #[test]
    fn save_keeps_a_constant_value_and_null_forms() {
        let mut e = setup();
        real_copy(&mut e);
        e.register(PARAM_COUNT, |_, _| ret(1));
        e.register(PARAM_IS_FORM, |_, _| ret(1));
        let it = item(&mut e);
        e.mem.set_u32(it.addr() + 4, 0x3f80_0000);
        e.call(0x0068_1840, &args![it]);
        // No form ids asked: the value is no global, the parameter and the
        // run-on reference are null; no swap.
        assert!(calls_to(&e, FORM_ID).is_empty());
        assert!(calls_to(&e, SWAP_WORD).is_empty());
        assert_eq!(calls_to(&e, ADD_CHUNK_DATA).len(), 1);
    }

    #[test]
    fn init_item_resolves_the_reference_and_the_global() {
        let mut e = setup();
        function_entry(&mut e, 5, true);
        let owner = form(&mut e, 0x77, 0x47);
        let it = item(&mut e);
        e.mem.set_u8(it.addr(), 0x04);
        e.mem.set_u32(it.addr() + 4, 0x12);
        e.mem.set_u16(it.addr() + 8, 5);
        e.mem.set_u32(it.addr() + 0x18, 0x13);
        e.call(0x0068_19e0, &args![it, owner]);
        assert_eq!(
            calls_to(&e, PARAMS_INIT_ITEM),
            vec![vec![owner, it.addr() + 8]]
        );
        assert_eq!(e.mem.u32(it.addr() + 0x18), 0x113);
        assert_eq!(e.mem.u32(it.addr() + 4), 0x112);
        let casts = calls_to(&e, DYNAMIC_CAST);
        assert_eq!(casts[0], vec![0x113, 0, TYPE_TES_FORM, TYPE_RUN_ON_REF, 0]);
        assert_eq!(casts[1], vec![0x112, 0, TYPE_TES_FORM, TYPE_GLOBAL, 0]);
        assert_eq!(calls_to(&e, ADD_COMPILE_INDEX)[0][1], 0x5000);
        assert!(calls_to(&e, LOG_WARNING).is_empty());
    }

    #[test]
    fn init_item_warns_about_a_missing_reference() {
        let mut e = setup();
        function_entry(&mut e, 0, true);
        let owner = form(&mut e, 0x77, 0x47);
        let it = item(&mut e);
        e.mem.set_u32(it.addr() + 0x18, 0x55);
        e.call(0x0068_19e0, &args![it, owner]);
        assert_eq!(
            calls_to(&e, LOG_WARNING),
            vec![vec![
                FORMAT_NO_REFERENCE_OWNER,
                0x55,
                owner + 0x1000,
                owner + 0x130,
                0x77
            ]]
        );
        e.call_log = Some(vec![]);
        e.mem.set_u32(it.addr() + 0x18, 0x56);
        e.call(0x0068_19e0, &args![it, 0u32]);
        assert_eq!(
            calls_to(&e, LOG_WARNING),
            vec![vec![FORMAT_NO_REFERENCE, 0x56]]
        );
    }

    #[test]
    fn init_item_warns_about_a_function_that_is_no_condition() {
        let mut e = setup();
        function_entry(&mut e, 7, false);
        let owner = form(&mut e, 0x77, 0x47);
        let it = item(&mut e);
        e.mem.set_u16(it.addr() + 8, 7);
        e.call(0x0068_19e0, &args![it, owner]);
        let printed = calls_to(&e, SPRINTF);
        assert_eq!(printed.len(), 1);
        let buffer = printed[0][0];
        assert_eq!(
            printed[0][1..],
            [FORMAT_FORM, owner + 0x1000, owner + 0x130, 0x77]
        );
        assert_eq!(
            calls_to(&e, LOG_WARNING),
            vec![vec![FORMAT_NOT_A_CONDITION, buffer, 0x5020, 0xabc7]]
        );
        assert_eq!(calls_to(&e, GET_FILE), vec![vec![owner, ANY_FILE]]);
    }

    #[test]
    fn init_item_describes_a_topic_info_owner() {
        let mut e = setup();
        function_entry(&mut e, 7, true);
        let owner = form(&mut e, 0x77, 0x46);
        let topic = form(&mut e, 0x88, 0);
        let quest = form(&mut e, 0x99, 0);
        e.register_double(GET_OWNER_TOPIC, move |_, _| ret(topic));
        e.register_double(TOPIC_QUEST, move |_, _| ret(quest));
        let it = item(&mut e);
        e.mem.set_u8(it.addr(), 0x04);
        e.mem.set_u32(it.addr() + 4, 0x44);
        e.mem.set_u16(it.addr() + 8, 7);
        e.call(0x0068_19e0, &args![it, owner]);
        let printed = calls_to(&e, SNPRINTF);
        assert_eq!(printed.len(), 1);
        let buffer = printed[0][0];
        let text = calls_to(&e, STRING_CONSTRUCT)[0][0];
        assert_eq!(
            printed[0][1..],
            [
                DESCRIPTION_SIZE,
                FORMAT_TOPIC_INFO,
                0x77,
                text + 1,
                quest + 0x138,
                0x99,
                topic + 0x130,
                0x88
            ]
        );
        assert_eq!(calls_to(&e, TOPIC_QUEST), vec![vec![topic, owner]]);
        assert_eq!(
            calls_to(&e, GET_INFO_DISPLAY_TEXT),
            vec![vec![owner, text, 0]]
        );
        assert_eq!(calls_to(&e, STRING_DESTRUCT), vec![vec![text]]);
        assert_eq!(
            calls_to(&e, LOG_WARNING),
            vec![vec![FORMAT_NO_GLOBAL, 0x44, buffer]]
        );
        assert!(calls_to(&e, SPRINTF).is_empty());
    }

    #[test]
    fn a_topic_info_without_a_topic_names_empty_strings() {
        let mut e = setup();
        function_entry(&mut e, 7, false);
        let owner = form(&mut e, 0x77, 0x46);
        e.register(GET_OWNER_TOPIC, |_, _| ret(0));
        e.register(TOPIC_QUEST, |_, _| ret(0xdead));
        let it = item(&mut e);
        e.mem.set_u16(it.addr() + 8, 7);
        e.call(0x0068_19e0, &args![it, owner]);
        let printed = calls_to(&e, SPRINTF);
        let text = calls_to(&e, STRING_CONSTRUCT)[0][0];
        assert_eq!(
            printed[0][1..],
            [
                FORMAT_TOPIC_INFO,
                0x77,
                text + 1,
                EMPTY_STRING,
                0,
                EMPTY_STRING,
                0
            ]
        );
        assert!(calls_to(&e, TOPIC_QUEST).is_empty());
    }
}
