//! `fallout shared/testopicinfo.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A topic info's responses: the response list read back from the info's
//! record (`TESTopicInfo::GetResponseList`, kept in one shared list for the
//! last info asked without a list of its own), the list helpers (append,
//! clear, copy) and the info's display text (`GetInfoDisplayText`).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `TESResponse` (Xbox PDB), 0x2c bytes (the PC code allocates the
    /// same); the fields used here.
    pub struct TesResponse: 0x2c {
        /// `pNext` (Xbox PDB): `TESResponse*`.
        0x28 pNext: u32,
    }
}

/// The offset of `TESTopicInfo::iFileOffset` (Xbox PDB +0x5c; the PC code
/// reads +0x4c, `TESForm` being 0x10 smaller): the record's offset in its
/// file.
const INFO_FILE_OFFSET: u32 = 0x4c;
/// The shared response list (`011cc07c`, one word: the first response).
const SHARED_RESPONSES: u32 = 0x011c_c07c;
/// The info whose responses the shared list holds (`011cbf3c`).
const SHARED_RESPONSES_INFO: u32 = 0x011c_bf3c;
/// `0045cd60` (`ECX` = response): the word at +0x28 (`pNext`).
const RESPONSE_NEXT: u32 = 0x0045_cd60;
/// `00559450` (`ECX` = object): the word at +0 (a string's characters; a
/// response list's first response).
const WORD_AT_ZERO: u32 = 0x0055_9450;
/// `TESResponse::GetResponseText` (`00616de0`, `ECX` = response).
const GET_RESPONSE_TEXT: u32 = 0x0061_6de0;
/// `BSStringT<char>::operator+=` (`00404820`, `ECX` = string, one word: the
/// characters to append; `RET 4`).
const STRING_APPEND: u32 = 0x0040_4820;
/// `004037f0` (`ECX` = string; characters, a word; `RET 8`): sets the
/// string.
const STRING_SET: u32 = 0x0040_37f0;
/// `" | "` (`0104d7c8`), between the responses of the display text.
const RESPONSE_SEPARATOR: u32 = 0x0104_d7c8;
/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00401030` (cdecl: block): frees it.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The size of a `TESResponse`.
const RESPONSE_SIZE: u32 = 0x2c;
/// `00616ce0` (`ECX` = memory): constructs a response there, returns it.
const RESPONSE_CONSTRUCT: u32 = 0x0061_6ce0;
/// `00616dc0` (`ECX` = response): its destructor.
const RESPONSE_DESTRUCT: u32 = 0x0061_6dc0;
/// `006174d0` (`ECX` = response, one word: the response to copy).
const RESPONSE_COPY: u32 = 0x0061_74d0;
/// `TESResponse::Load` (`00617530`, `ECX` = response, one word: the file).
const RESPONSE_LOAD: u32 = 0x0061_7530;
/// `TESResponse::InitItem` (`00617580`, `ECX` = response, one word: the
/// info).
const RESPONSE_INIT_ITEM: u32 = 0x0061_7580;
/// `TESForm::GetFile` (`00484e60`, `ECX` = form, one word: an index).
const GET_FILE: u32 = 0x0048_4e60;
/// The `GetFile` index word the exe passes (-1).
const ANY_FILE: u32 = 0xffff_ffff;
/// `TESFile::GetThreadSafeFile` (`004739b0`, `ECX` = file).
const GET_THREAD_SAFE_FILE: u32 = 0x0047_39b0;
/// `TESFile::SetOffset` (`004723a0`, `ECX` = file, one word: the offset).
const SET_OFFSET: u32 = 0x0047_23a0;
/// `TESFile::GetTESForm` (`00472660`, `ECX` = file): the current record's
/// form type byte.
const GET_TES_FORM: u32 = 0x0047_2660;
/// The record form type the responses are read from (0x46).
const INFO_FORM_TYPE: u8 = 0x46;
/// `TESFile::GetTESChunk` (`004726b0`, `ECX` = file): the current chunk's
/// tag.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk; zero at the end.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData` (`004727f0`, `ECX` = file, one word: the
/// destination of one word).
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov3` (`00472890`, `ECX` = file; destination,
/// size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `TESForm::AddCompileIndex` (`00485d50`, cdecl: address of a form id,
/// file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (cdecl: form id): the form with that id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `00401170` (`ECX` = form): the byte at +4 (`TESForm` form type).
const FORM_TYPE: u32 = 0x0040_1170;
/// The form type an idle must have (0x48).
const IDLE_FORM_TYPE: u32 = 0x48;
/// `0084e3a0` (`ECX` = form): the word at +0xc (`TESForm` form id).
const FORM_ID: u32 = 0x0084_e3a0;
/// `005b5e40` (cdecl: format, arguments...): writes a warning line.
const LOG_WARNING: u32 = 0x005b_5e40;
/// `"DEFAULT: Unable to find idle (%08X) for info (%08X)."` (`0104d7cc`).
const FORMAT_NO_IDLE: u32 = 0x0104_d7cc;
/// `0050f9c0` (`ECX` = response, one word: the idle): the `SNAM` idle's
/// setter.
const SET_SNAM_IDLE: u32 = 0x0050_f9c0;
/// `007037c0` (`ECX` = response, one word: the idle): the `LNAM` idle's
/// setter.
const SET_LNAM_IDLE: u32 = 0x0070_37c0;
/// The chunk tags.
const CHUNK_TRDT: u32 = 0x5444_5254;
const CHUNK_NAM1: u32 = 0x314d_414e;
const CHUNK_SNAM: u32 = 0x4d41_4e53;
const CHUNK_LNAM: u32 = 0x4d41_4e4c;

// Translated from 0061cad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears a response list (`ECX`, the word holding the first response):
/// each response is taken off (its next, `0045cd60`, read first) and
/// deleted (`0061cb30` with 1), then the head is zeroed. The engine map
/// gives it no name.
pub fn fn_0061cad0(e: &mut Engine, this: Ptr) {
    let mut response = e.mem.u32(this.addr());
    while response != 0 {
        let next = e.call(RESPONSE_NEXT, &args![response]).u32();
        if response != 0 {
            fn_0061cb30(e, Ptr::new(response), 1);
        }
        response = next;
    }
    e.mem.set_u32(this.addr(), 0);
}

// Translated from 0061cb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The response's (`ECX`) scalar deleting destructor (one word: the flags;
/// `RET 4`): the destructor (`00616dc0`), then, with flag bit 0, the block
/// is freed (`00401030`). Returns the response. The engine map gives it no
/// name.
pub fn fn_0061cb30(e: &mut Engine, this: Ptr<TesResponse>, flags: u32) -> Ptr<TesResponse> {
    e.call(RESPONSE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0061cb60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061cb60 - the exception-unwinding frame (FS:[0]) that frees a response whose constructor throws is not translated.
/// Makes the response list (`ECX`) a copy of the chain starting at the
/// response given (one word; `RET 4`): the list is cleared (`0061cad0`),
/// then for each response of the chain (`0045cd60`) a new one is allocated
/// (0x2c bytes, `00401000`), constructed (`00616ce0`; 0 if the allocation
/// failed), copied from it (`006174d0`) and linked after the previous copy
/// (`0061cc40`), the first appended to the list (`0061cc60`). The engine
/// map gives it no name. The exception-unwinding frame is not translated.
pub fn fn_0061cb60(e: &mut Engine, this: Ptr, first: u32) {
    fn_0061cad0(e, this);
    let mut previous = 0u32;
    let mut source = first;
    while source != 0 {
        let memory = e.call(OPERATOR_NEW, &args![RESPONSE_SIZE]).u32();
        let copy = if memory != 0 {
            e.call(RESPONSE_CONSTRUCT, &args![memory]).u32()
        } else {
            0
        };
        e.call(RESPONSE_COPY, &args![copy, source]);
        if previous != 0 {
            fn_0061cc40(e, Ptr::new(previous), copy);
        } else {
            fn_0061cc60(e, this, copy);
        }
        previous = copy;
        source = e.call(RESPONSE_NEXT, &args![source]).u32();
    }
}

// Translated from 0061cc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the word given (one word; `RET 4`) as the response's (`ECX`)
/// next (`pNext`, +0x28). The engine map gives it no name.
pub fn fn_0061cc40(e: &mut Engine, this: Ptr<TesResponse>, next: u32) {
    e.set(this, TesResponse::pNext, next);
}

// Translated from 0061cc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a response (one word; `RET 4`) to the response list (`ECX`):
/// stored as the first response of an empty list, else handed to the last
/// response's link (`0061cc90` on the first). The engine map gives it no
/// name.
pub fn fn_0061cc60(e: &mut Engine, this: Ptr, response: u32) {
    let first = e.mem.u32(this.addr());
    if first == 0 {
        e.mem.set_u32(this.addr(), response);
    } else {
        fn_0061cc90(e, Ptr::new(first), response);
    }
}

// Translated from 0061cc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Links a response (one word; `RET 4`) after the last response of the
/// chain starting at `ECX` (followed through `0045cd60`): stored as its
/// `pNext` (+0x28). The engine map gives it no name.
pub fn fn_0061cc90(e: &mut Engine, this: Ptr<TesResponse>, response: u32) {
    let mut last = this.addr();
    while e.call(RESPONSE_NEXT, &args![last]).u32() != 0 {
        last = e.call(RESPONSE_NEXT, &args![last]).u32();
    }
    e.set(Ptr::<TesResponse>::new(last), TesResponse::pNext, response);
}

// Translated from 0061d230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the shared response list (`0061cad0` on `011cc07c`); `ECX` (an
/// info) is not read. The engine map gives it no name.
pub fn fn_0061d230(e: &mut Engine, _this: Ptr) {
    fn_0061cad0(e, Ptr::new(SHARED_RESPONSES));
}

// Translated from 0061e2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTopicInfo::GetInfoDisplayText` (Xbox PDB; `ECX` = info; the string,
/// a byte; `RET 8`): empties the string (`004037f0` with 0, 0), then
/// appends the text (`TESResponse::GetResponseText`) of each response of
/// the info's list (`GetResponseList` with no list of its own, its first
/// response from `00559450`), with `" | "` after each response that has a
/// next; only the first response when `first_only` is nonzero.
pub fn tes_topic_info_get_info_display_text(
    e: &mut Engine,
    this: Ptr,
    string: Ptr,
    first_only: u8,
) {
    e.call(STRING_SET, &args![string, 0u32, 0u32]);
    let list = tes_topic_info_get_response_list(e, this, Ptr::NULL);
    let mut response = e.call(WORD_AT_ZERO, &args![list]).u32();
    while response != 0 {
        let text = e.call(GET_RESPONSE_TEXT, &args![response]).u32();
        e.call(STRING_APPEND, &args![string, text]);
        if e.call(RESPONSE_NEXT, &args![response]).u32() != 0 {
            e.call(STRING_APPEND, &args![string, RESPONSE_SEPARATOR]);
        }
        if first_only != 0 {
            break;
        }
        response = e.call(RESPONSE_NEXT, &args![response]).u32();
    }
}

// Translated from 0061e780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061e780 - the exception-unwinding frame (FS:[0]) around the response allocation and the security-cookie check (00ec408c) are not translated.
/// `TESTopicInfo::GetResponseList` (Xbox PDB; `ECX` = info, one word: a
/// response list or 0; `RET 4`): the list the responses go in is the one
/// given, or the shared list (`011cc07c`) for 0, and is returned.
/// - For the info whose responses the shared list holds (`011cbf3c`): a
///   given list becomes a copy of the shared one (`0061cb60`); nothing is
///   read.
/// - Otherwise, without a list of its own, the info becomes the shared
///   list's info and the list is cleared (`0061d230`). The info's record
///   is read again: from its file (`GetFile(-1)`, then
///   `GetThreadSafeFile`) at its file offset (+0x4c; nothing with no file
///   or a zero offset, or when `SetOffset` fails, or the record is not of
///   form type 0x46), chunk by chunk (`004726f0`) until a zero tag:
///   `TRDT` starts a new response (0x2c bytes, `00616ce0`, `Load` from the
///   file, `InitItem` with the info, appended by `0061cc60`); `NAM1`
///   (with a response) sets its text from the chunk (read into a stack
///   buffer of the chunk's size, `00401660`, `0061eb40`); `SNAM` and
///   `LNAM` (with a response) read a form id, give it the file's compile
///   index and look it up; a form of type 0x48 is the idle, else 0, and a
///   nonzero id without an idle warns with the id and the info's id; the
///   idle goes to `0050f9c0` (`SNAM`) or `007037c0` (`LNAM`).
///
/// The stack buffer is the `_alloca` (`00ec5ec0`) the exe makes. The
/// exception-unwinding frame and the security-cookie check are not
/// translated.
pub fn tes_topic_info_get_response_list(e: &mut Engine, this: Ptr, list: Ptr) -> Ptr {
    let target = if list.is_null() {
        Ptr::new(SHARED_RESPONSES)
    } else {
        list
    };
    if this.addr() == e.mem.u32(SHARED_RESPONSES_INFO) {
        if list.is_null() {
            return target;
        }
        let first = e.call(WORD_AT_ZERO, &args![SHARED_RESPONSES]).u32();
        fn_0061cb60(e, list, first);
        return list;
    }
    if list.is_null() {
        e.mem.set_u32(SHARED_RESPONSES_INFO, this.addr());
        fn_0061d230(e, this);
    }
    let file = e.call(GET_FILE, &args![this, ANY_FILE]).u32();
    if file == 0 || e.mem.u32(this.addr() + INFO_FILE_OFFSET) == 0 {
        return target;
    }
    let file = e.call(GET_THREAD_SAFE_FILE, &args![file]).u32();
    let offset = e.mem.u32(this.addr() + INFO_FILE_OFFSET);
    if e.call(SET_OFFSET, &args![file, offset]).u8() == 0 {
        return target;
    }
    if e.call(GET_TES_FORM, &args![file]).u8() != INFO_FORM_TYPE {
        return target;
    }
    let mut response = 0u32;
    loop {
        let tag = e.call(GET_TES_CHUNK, &args![file]).u32();
        if tag == 0 {
            break;
        }
        match tag {
            CHUNK_TRDT => {
                let memory = e.call(OPERATOR_NEW, &args![RESPONSE_SIZE]).u32();
                response = if memory != 0 {
                    e.call(RESPONSE_CONSTRUCT, &args![memory]).u32()
                } else {
                    0
                };
                e.call(RESPONSE_LOAD, &args![response, file]);
                e.call(RESPONSE_INIT_ITEM, &args![response, this]);
                fn_0061cc60(e, target, response);
            }
            CHUNK_NAM1 if response != 0 => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, size]);
                    fn_0061eb40(e, Ptr::new(response), buffer);
                });
            }
            CHUNK_SNAM | CHUNK_LNAM if response != 0 => {
                let idle = read_idle(e, this, file);
                let setter = if tag == CHUNK_SNAM {
                    SET_SNAM_IDLE
                } else {
                    SET_LNAM_IDLE
                };
                e.call(setter, &args![response, idle]);
            }
            _ => {}
        }
        if e.call(NEXT_CHUNK, &args![file]).u8() == 0 {
            break;
        }
    }
    target
}

/// `GetResponseList`'s `SNAM`/`LNAM` read (the same code in both cases):
/// the chunk's form id (in a stack word, 0 first) gets the file's compile
/// index and is looked up; the form is the idle if its type is 0x48. A
/// nonzero id without an idle warns with the id and the info's id.
fn read_idle(e: &mut Engine, info: Ptr, file: u32) -> u32 {
    e.with_stack(4, |e, id_slot| {
        e.mem.set_u32(id_slot.addr(), 0);
        e.call(GET_CHUNK_WORD, &args![file, id_slot]);
        e.call(ADD_COMPILE_INDEX, &args![id_slot, file]);
        let mut idle = 0u32;
        let form = e.call(LOOKUP_FORM, &args![e.mem.u32(id_slot.addr())]).u32();
        if form != 0 && e.call(FORM_TYPE, &args![form]).u32() == IDLE_FORM_TYPE {
            idle = form;
        }
        let id = e.mem.u32(id_slot.addr());
        if id != 0 && idle == 0 {
            let info_id = e.call(FORM_ID, &args![info]).u32();
            e.call(LOG_WARNING, &args![FORMAT_NO_IDLE, id, info_id]);
        }
        idle
    })
}

// Translated from 0061eb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the response's (`ECX`) text (`cResponseText`, +0x18) to the
/// characters given (one word; `RET 4`): `004037f0` with the characters
/// and 0. The engine map gives it no name.
pub fn fn_0061eb40(e: &mut Engine, this: Ptr<TesResponse>, text: Ptr) {
    e.call(STRING_SET, &args![this.addr() + 0x18, text, 0u32]);
}

layout! {
    /// `TESTopicInfo` (Xbox PDB), 0x60 bytes on the Xbox; on PC every field
    /// sits 0x10 lower (`TESForm` is 0x18 bytes). The fields used here.
    pub struct TesTopicInfo: 0x50 {
        /// `iInfoIndex` (Xbox PDB +0x30).
        0x20 iInfoIndex: u16,
        /// `bSaidOnce` (Xbox PDB +0x32).
        0x22 bSaidOnce: u8,
        /// `m_Data` (Xbox PDB `TOPIC_INFO_DATA`, 4 bytes from +0x33): the
        /// first byte (the type the `DATA` chunk sets).
        0x23 m_Data: u8,
        /// `m_pConversationData` (Xbox PDB +0x48): `TESConversationData*`.
        0x38 m_pConversationData: u32,
        /// `pSpeaker` (Xbox PDB +0x4c).
        0x3c pSpeaker: u32,
        /// `pPerkSkillStat` (Xbox PDB +0x50).
        0x40 pPerkSkillStat: u32,
        /// `eDifficulty` (Xbox PDB +0x54).
        0x44 eDifficulty: u32,
        /// `pOwnerQuest` (Xbox PDB +0x58).
        0x48 pOwnerQuest: u32,
    }
}

/// `TESTopicInfo`'s vtable (its RTTI name, read through the locator).
const TOPIC_INFO_VTABLE: u32 = 0x0104_d5b4;
/// `TESForm::TESForm` and `TESForm::~TESForm` (Xbox PDB names of `00483370`
/// and `00483630`; `ECX` = form).
const TES_FORM_CONSTRUCT: u32 = 0x0048_3370;
const TES_FORM_DESTRUCT: u32 = 0x0048_3630;
/// `00680890` and `006808b0` (`ECX` = the conditions at +0x18): construct
/// and destroy.
const CONDITIONS_CONSTRUCT: u32 = 0x0068_0890;
const CONDITIONS_DESTRUCT: u32 = 0x0068_08b0;
/// `004037b0` and `004037d0` (`ECX` = string): construct and destroy.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `0096a2d0` and `0046ffb0` (`ECX` = a `BSSimpleList`): construct and
/// destroy.
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// `0061c330` (`ECX` = conversation data): run first by its destructor.
const CONVERSATION_0061C330: u32 = 0x0061_c330;
/// `0061cdc0` (`ECX` = info, one word; `RET 4`).
const INFO_0061CDC0: u32 = 0x0061_cdc0;
/// `0061c210` (`ECX` = conversation data; the info, a word; `RET 8`).
const CONVERSATION_0061C210: u32 = 0x0061_c210;
/// The size of a `TESConversationData` (0x18).
const CONVERSATION_SIZE: u32 = 0x18;
/// `00403d30` (cdecl: destination, byte, count): fills memory.
const FILL_MEMORY: u32 = 0x0040_3d30;
/// `004f15a0` (`ECX` = form, the form type; `RET 4`).
const SET_FORM_TYPE: u32 = 0x004f_15a0;
/// `00409480` and `0087ce80` (`ECX` = info, one word; `RET 4`).
const INFO_SET_00409480: u32 = 0x0040_9480;
const INFO_SET_0087CE80: u32 = 0x0087_ce80;
/// `LowProcess::SetGenericLocation` (engine map name of `008d8040`; `ECX` =
/// info, one word; `RET 4`): sets the word at +0x44 (`eDifficulty`).
const SET_DIFFICULTY: u32 = 0x008d_8040;
/// The empty string (`01011584`).
const EMPTY_STRING: u32 = 0x0101_1584;
/// `004077c0` and `004013e0` (`ECX` = form): bytes.
const FORM_004077C0: u32 = 0x0040_77c0;
const FORM_004013E0: u32 = 0x0040_13e0;
/// `00440d80` (`ECX` = form): a byte.
const FORM_00440D80: u32 = 0x0044_0d80;
/// `00619a70` (`run_00619a70`; `ECX` = info): `iInfoIndex` (in `AX`).
const INFO_INDEX: u32 = 0x0061_9a70;
/// `00619b20` (`run_00619a70`; `ECX` = info): bit 2 of the byte at +0x25.
const INFO_00619B20: u32 = 0x0061_9b20;
/// `00619b40` (`ECX` = info): a byte.
const INFO_00619B40: u32 = 0x0061_9b40;
/// `TESTopic::GetOwnerTopic` (Xbox PDB name of `0061a310`; cdecl: info).
const GET_OWNER_TOPIC: u32 = 0x0061_a310;
/// `00619d00` (`ECX` = topic, the info; `RET 4`): the quest the topic
/// holds the info for.
const TOPIC_QUEST: u32 = 0x0061_9d00;
/// `00500940` (`ECX` = object): `this + 0x18` (an info's conditions; a
/// script's data header).
const AT_0X18: u32 = 0x0050_0940;
/// `00680fe0` (`ECX` = conditions): clears them.
const CONDITIONS_CLEAR: u32 = 0x0068_0fe0;
/// `TESCondition::Load` and `InitItem` (Xbox PDB names of `00680960` and
/// `00680a20`; `ECX` = conditions, one word; `RET 4`).
const CONDITION_LOAD: u32 = 0x0068_0960;
const CONDITION_INIT_ITEM: u32 = 0x0068_0a20;
/// `TESCondition::IsTrue_ov2` (Xbox PDB name of `00680c60`; `ECX` =
/// conditions, four words; `RET 0x10`): a byte.
const CONDITION_IS_TRUE: u32 = 0x0068_0c60;
/// `005f5760` (`ECX` = object): its conditions.
const CONDITIONS_OF_005F5760: u32 = 0x005f_5760;
/// `Script::Script`, `Script::CopyResultScript` and the script destructor
/// (Xbox PDB names of `005aa0f0`, `005aa420`; `005aa1a0`; `ECX` = script).
const SCRIPT_CONSTRUCT: u32 = 0x005a_a0f0;
const SCRIPT_COPY: u32 = 0x005a_a420;
const SCRIPT_DESTRUCT: u32 = 0x005a_a1a0;
/// The size the stack script takes (the exe reserves 0x5c bytes).
const STACK_SCRIPT_SIZE: u32 = 0x5c;
/// The result scripts (`011cbf60`, 0x54 bytes each; the first two are the
/// begin and end scripts every info shares).
const RESULT_SCRIPTS: u32 = 0x011c_bf60;
const RESULT_SCRIPT_SIZE: u32 = 0x54;
/// The info whose result scripts `RESULT_SCRIPTS` holds (`011cbf40`).
const RESULT_SCRIPTS_INFO: u32 = 0x011c_bf40;
/// `0061c360`'s target and the topic code's arrays.
const ARRAY_COUNT: u32 = 0x0084_e3a0;
/// `00619c50` (`ECX` = topic, the info; `RET 4`).
const TOPIC_00619C50: u32 = 0x0061_9c50;
/// `BaseProcess::GetActorPackageThatIsRunning` (engine map name of
/// `00717e50`; `ECX` = `00619c50`'s result): an array.
const ARRAY_OF_00717E50: u32 = 0x0071_7e50;
/// `00877a30` (`ECX` = array, an index; `RET 4`): the address of that
/// element.
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// `00486b80` (`ECX` = array, an index; `RET 4`): removes that element.
const ARRAY_REMOVE: u32 = 0x0048_6b80;
/// `0061bdc0` (`ECX` = array).
const ARRAY_0061BDC0: u32 = 0x0061_bdc0;
/// `00619fe0` (cdecl: the array).
const ARRAY_00619FE0: u32 = 0x0061_9fe0;
/// `00436aa0` (`ECX` = info): `this + 0x30`, the topic list.
const ADD_TOPICS: u32 = 0x0043_6aa0;
/// `008256d0` (`ECX` = list node): a byte; true at the end of the list.
const NODE_IS_END: u32 = 0x0082_56d0;
/// `0063f7b0` (`ECX` = list node).
const NODE_0063F7B0: u32 = 0x0063_f7b0;
/// `006815c0` (`ECX` = list node): the address of its item.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`ECX` = list node): the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `00905330` (`ECX` = node, the address of an item; `RET 4`).
const NODE_00905330: u32 = 0x0090_5330;
/// `00726c60` (`ECX` = node, the address of an item; `RET 4`): sets it.
const NODE_SET_ITEM: u32 = 0x0072_6c60;
/// `00905820` (`ECX` = list, the address of an item; `RET 4`): appends.
const LIST_APPEND: u32 = 0x0090_5820;
/// `sprintf` (`00ec623a`, cdecl).
const SPRINTF: u32 = 0x00ec_623a;
/// `"TopicInfo %08X \"%s\" in Quest %s (%08x) Topic %s (%08x)"`
/// (`0104cae0`) and `"TopicInfo %08X \"%s\""` (`0104d598`).
const FORMAT_INFO_IN_QUEST: u32 = 0x0104_cae0;
const FORMAT_INFO: u32 = 0x0104_d598;
/// The size of the description buffer `InitItem` writes (0x108 bytes) and
/// `TESConversationData::InitItem`'s (0x400 bytes).
const INFO_DESCRIPTION_SIZE: u32 = 0x108;
const CONVERSATION_DESCRIPTION_SIZE: u32 = 0x400;
/// The warnings: `"MASTERFILE: Unable to find topic (%08X) for %s"`
/// (`0104d6ec`) and the conversation data's 'Link To' (`0104d548`), 'Link
/// From' (`0104d4f8`) and 'Follow Up' (`0104d4a8`) ones.
const FORMAT_NO_TOPIC: u32 = 0x0104_d6ec;
const FORMAT_NO_LINK_TO: u32 = 0x0104_d548;
const FORMAT_NO_LINK_FROM: u32 = 0x0104_d4f8;
const FORMAT_NO_FOLLOW_UP: u32 = 0x0104_d4a8;
/// `"MASTERFILE: Unable to insert topic info (%08X) into topic '%s'
/// (%08X), quest (%08X)."` (`0104d770`).
const FORMAT_NOT_INSERTED: u32 = 0x0104_d770;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Type descriptors: `TESForm`, `TESTopic`, and those of the speaker
/// (`011846e8`) and of the two classes `pPerkSkillStat` may be (`011861dc`,
/// `0119a554`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_TOPIC: u32 = 0x0118_4720;
const TYPE_011846E8: u32 = 0x0118_46e8;
const TYPE_011861DC: u32 = 0x0118_61dc;
const TYPE_0119A554: u32 = 0x0119_a554;
/// The form types the conversation data's lists hold (0x45 topics, 0x46
/// infos).
const TOPIC_FORM_TYPE: u32 = 0x45;
const INFO_FORM_TYPE_WORD: u32 = 0x46;
/// `00639b40` (`ECX` = info): the word at +0x3c (`pSpeaker`).
const SPEAKER: u32 = 0x0063_9b40;
/// `009e32d0` (`ECX` = info): the word at +0x38 (`m_pConversationData`).
const CONVERSATION_DATA: u32 = 0x009e_32d0;
/// The conversation data's lists: `00413f40` (+8, 'Link To'), `006815c0`
/// (+0, 'Link From') and `00460140` (+0x10, 'Follow Up'); `ECX` = data.
const LINK_TO: u32 = 0x0041_3f40;
const LINK_FROM: u32 = 0x0068_15c0;
const FOLLOW_UP: u32 = 0x0046_0140;
/// `00484ab0` (`ECX` = form, a word; `RET 4`).
const FORM_SET_484AB0: u32 = 0x0048_4ab0;
/// `TESFile::GetOptimizedFile` (Xbox PDB name of `00471ca0`; `ECX` = file).
const GET_OPTIMIZED_FILE: u32 = 0x0047_1ca0;
/// `00467bb0` (`ECX` = file): the current record's offset.
const RECORD_OFFSET: u32 = 0x0046_7bb0;
/// `TESForm::LoadForm` (Xbox PDB name of `00485110`; `ECX` = form, file).
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// The topic `Load` inserts the info into (`011cbeb0`, a pointer global).
const CURRENT_TOPIC: u32 = 0x011c_beb0;
/// `TESTopic::InsertInfo` (Xbox PDB name of `00619820`; `ECX` = topic;
/// topic id, info, previous info id; `RET 0xc`).
const INSERT_INFO: u32 = 0x0061_9820;
/// The actor value holder `0043d4d0` reads (`011d0dc8`).
const VALUE_HOLDER: u32 = 0x011d_0dc8;
/// `0043d4d0` (`ECX` = `011d0dc8`): the address of a word.
const VALUE_0043D4D0: u32 = 0x0043_d4d0;
/// `00508d70` (`ECX` = info): a byte; false leaves the result scripts empty.
const INFO_00508D70: u32 = 0x0050_8d70;
/// `TESForm::SetTemporary` (Xbox PDB name of `00484490`; `ECX` = form).
const SET_TEMPORARY: u32 = 0x0048_4490;
/// `00401680` (`ECX` = file): whether the data read needs swapping.
const FILE_NEEDS_SWAP: u32 = 0x0040_1680;
/// `00414220` (`ECX` = script data header): its byte swap.
const HEADER_SWAP: u32 = 0x0041_4220;
/// `Script::SetCompileData` (Xbox PDB name of `005aae70`; `ECX` = script;
/// size, data; `RET 8`).
const SET_COMPILE_DATA: u32 = 0x005a_ae70;
/// `Script::InitItem` (Xbox PDB name of `005ab400`; `ECX` = script, the
/// owner; `RET 4`).
const SCRIPT_INIT_ITEM: u32 = 0x005a_b400;
/// `00500910` (`ECX` = memory): constructs a referenced-object entry.
const REFERENCED_OBJECT_CONSTRUCT: u32 = 0x0050_0910;
/// `005d43c0` (`ECX` = object): `this + 0x44` (a script's referenced
/// objects; a reference's extra data list).
const AT_0X44: u32 = 0x005d_43c0;
/// `005d43e0` (`ECX` = script): `this + 0x4c`, its variables.
const AT_0X4C: u32 = 0x005d_43e0;
/// `005e28b0` (`ECX` = memory): constructs a 0x20-byte script variable.
const VARIABLE_CONSTRUCT: u32 = 0x005e_28b0;
/// `ScriptVariable::Load` (Xbox PDB name of `005e2a20`; `ECX` = variable,
/// the file; `RET 4`).
const VARIABLE_LOAD: u32 = 0x005e_2a20;
/// The result scripts' variable being loaded (`011cc0a0`).
const RESULT_VARIABLE: u32 = 0x011c_c0a0;
/// The most a variable name is read (0x200 bytes).
const NAME_MAXIMUM: u32 = 0x200;
/// The player (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `009527b0` (`ECX` = player, a list; `RET 4`).
const PLAYER_009527B0: u32 = 0x0095_27b0;
/// `Script::GetProcessScripts` (Xbox PDB name of `005ac740`).
const PROCESS_SCRIPTS: u32 = 0x005a_c740;
/// `ExtraDataList::GetScriptLocals` (Xbox PDB name of `00418830`; `ECX` =
/// list).
const SCRIPT_LOCALS: u32 = 0x0041_8830;
/// `Script::Run` (Xbox PDB name of `005ac1e0`; `ECX` = script; four words;
/// `RET 0x10`).
const SCRIPT_RUN: u32 = 0x005a_c1e0;
/// `0061f220` and `0061aef0` (`ECX` = info; the second a byte).
const INFO_0061F220: u32 = 0x0061_f220;
const INFO_0061AEF0: u32 = 0x0061_aef0;
/// `00935920` (`ECX` = reference, the info; `RET 4`).
const REFERENCE_00935920: u32 = 0x0093_5920;
/// The chunk tags `Load` and `GetResultScript` read.
const CHUNK_CTDA: u32 = 0x4144_5443;
const CHUNK_PNAM: u32 = 0x4d41_4e50;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_NAME: u32 = 0x454d_414e;
const CHUNK_QSTI: u32 = 0x4954_5351;
const CHUNK_TCLT: u32 = 0x544c_4354;
const CHUNK_TCLF: u32 = 0x464c_4354;
const CHUNK_TCFU: u32 = 0x5546_4354;
const CHUNK_RNAM: u32 = 0x4d41_4e52;
const CHUNK_KNAM: u32 = 0x4d41_4e4b;
const CHUNK_ANAM: u32 = 0x4d41_4e41;
const CHUNK_DNAM: u32 = 0x4d41_4e44;
const CHUNK_SCHR: u32 = 0x5248_4353;
const CHUNK_SCDA: u32 = 0x4144_4353;
const CHUNK_SLSD: u32 = 0x4453_4c53;
const CHUNK_SCRO: u32 = 0x4f52_4353;
const CHUNK_SCVR: u32 = 0x5256_4353;
const CHUNK_SCRV: u32 = 0x5652_4353;
const CHUNK_NEXT: u32 = 0x5458_454e;

// Translated from 0061c190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061c190 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESConversationData`'s destructor body (its lists at +0, +8 and +0x10,
/// Xbox PDB): `0061c330`, then the lists at +0x10, +8 and +0
/// (`0046ffb0`). The engine map gives it no name.
pub fn fn_0061c190(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(CONVERSATION_0061C330, &args![this]);
    e.call(LIST_DESTRUCT, &args![base + 0x10]);
    e.call(LIST_DESTRUCT, &args![base + 0x8]);
    e.call(LIST_DESTRUCT, &args![this]);
}

/// The warning both `InitItem`s write for a form id not found: the info's
/// topic (`TESTopic::GetOwnerTopic`) and that topic's quest (`00619d00`),
/// its display text (`GetInfoDisplayText` with 1, into a stack string),
/// then a description into a stack buffer of `size` bytes:
/// `"TopicInfo %08X \"%s\" in Quest %s (%08x) Topic %s (%08x)"` with the
/// info's form id, the text, the quest's slot `0x138` and form id and the
/// topic's slot `0x130` and form id, or `"TopicInfo %08X \"%s\""` without
/// a topic or quest; then `format` with the id and the description, and the
/// stack string destroyed. The calls are in the exe's order.
fn warn_with_description(e: &mut Engine, info: Ptr, id: u32, format: u32, size: u32) {
    let topic = e.call(GET_OWNER_TOPIC, &args![info]).u32();
    let quest = if topic != 0 {
        e.call(TOPIC_QUEST, &args![topic, info]).u32()
    } else {
        0
    };
    e.with_stack(size, |e, buffer| {
        e.with_stack(8, |e, text| {
            e.call(STRING_CONSTRUCT, &args![text]);
            tes_topic_info_get_info_display_text(e, info, text, 1);
            if topic != 0 && quest != 0 {
                let topic_id = e.call(FORM_ID, &args![topic]).u32();
                let topic_name = e.vcall(topic, 0x130, &[]).u32();
                let quest_id = e.call(FORM_ID, &args![quest]).u32();
                let quest_name = e.vcall(quest, 0x138, &[]).u32();
                let characters = e.call(WORD_AT_ZERO, &args![text]).u32();
                let info_id = e.call(FORM_ID, &args![info]).u32();
                e.call(
                    SPRINTF,
                    &args![
                        buffer,
                        FORMAT_INFO_IN_QUEST,
                        info_id,
                        characters,
                        quest_name,
                        quest_id,
                        topic_name,
                        topic_id
                    ],
                );
            } else {
                let characters = e.call(WORD_AT_ZERO, &args![text]).u32();
                let info_id = e.call(FORM_ID, &args![info]).u32();
                e.call(SPRINTF, &args![buffer, FORMAT_INFO, info_id, characters]);
            }
            e.call(LOG_WARNING, &args![format, id, buffer]);
            e.call(STRING_DESTRUCT, &args![text]);
        });
    });
}

/// The walk `TESConversationData::InitItem` makes over each of its three
/// lists (and `TESTopicInfo::InitItem` over its topic list, with a cast):
/// each node's form id gets the file's compile index and is looked up
/// (`resolve`); a found form replaces the id in the node (`00726c60`) and
/// the walk moves on; a missing one is taken out (`0063f7b0` on the node,
/// or, at the end of the list after a found node, `00905330` on that node
/// and the walk goes on from its next) and warned about (`warn`).
fn resolve_list(
    e: &mut Engine,
    first: u32,
    file: u32,
    mut resolve: impl FnMut(&mut Engine, u32) -> u32,
    mut warn: impl FnMut(&mut Engine, u32),
) {
    let mut previous = 0u32;
    let mut node = first;
    loop {
        if node == 0 {
            break;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let raw = e.mem.u32(item);
        let (id, found) = e.with_stack(8, |e, slots| {
            e.mem.set_u32(slots.addr(), raw);
            e.call(ADD_COMPILE_INDEX, &args![slots, file]);
            let id = e.mem.u32(slots.addr());
            let found = resolve(e, id);
            e.mem.set_u32(slots.addr() + 4, found);
            if found != 0 {
                e.call(NODE_SET_ITEM, &args![node, slots.addr() + 4]);
            }
            (id, found)
        });
        if found == 0 {
            if e.call(NODE_NEXT, &args![node]).u32() != 0 || previous == 0 {
                e.call(NODE_0063F7B0, &args![node]);
            } else {
                e.with_stack(4, |e, copy| {
                    e.mem.set_u32(copy.addr(), id);
                    e.call(NODE_00905330, &args![previous, copy]);
                });
                node = e.call(NODE_NEXT, &args![previous]).u32();
            }
            warn(e, id);
        } else {
            previous = node;
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
}

// Translated from 0061c360 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061c360 - the exception-unwinding frame (FS:[0]) and the security-cookie check (00ec408c) are not translated.
/// `TESConversationData::InitItem` (Xbox PDB; the info; `RET 4`): resolves
/// the form ids of its 'Link To' (+8) and 'Link From' (+0) lists to topics
/// (form type 0x45) and of its 'Follow Up' list (+0x10) to infos (0x46),
/// with the info's file (`GetFile(-1)`; none for a null info). Each id not
/// found is taken out of its list and warned about with the info's
/// description (when the info is of form type 0x46).
pub fn tes_conversation_data_init_item(e: &mut Engine, this: Ptr, info: Ptr) {
    let file = if info.is_null() {
        0
    } else {
        e.call(GET_FILE, &args![info, ANY_FILE]).u32()
    };
    let lists = [
        (LINK_TO, TOPIC_FORM_TYPE, FORMAT_NO_LINK_TO),
        (LINK_FROM, TOPIC_FORM_TYPE, FORMAT_NO_LINK_FROM),
        (FOLLOW_UP, INFO_FORM_TYPE_WORD, FORMAT_NO_FOLLOW_UP),
    ];
    for (getter, form_type, format) in lists {
        let first = e.call(getter, &args![this]).u32();
        resolve_list(
            e,
            first,
            file,
            |e, id| {
                let form = e.call(LOOKUP_FORM, &args![id]).u32();
                if form != 0 && e.call(FORM_TYPE, &args![form]).u32() == form_type {
                    form
                } else {
                    0
                }
            },
            |e, id| {
                let owner = if !info.is_null()
                    && e.call(FORM_TYPE, &args![info]).u32() == INFO_FORM_TYPE_WORD
                {
                    info
                } else {
                    Ptr::NULL
                };
                warn_with_description(e, owner, id, format, CONVERSATION_DESCRIPTION_SIZE);
            },
        );
    }
}

// Translated from 0061ccd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061ccd0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs a `TESTopicInfo` (the vtable's RTTI name; one word, `RET 4`):
/// `TESForm::TESForm`, the vtable, the conditions (+0x18), the prompt
/// string (+0x28) and the topic list (+0x30) constructed, the data set to
/// its start values (`0061cf40`), the first data byte (+0x23) set to the
/// word's low byte, and for a word of 1 a new conversation data (0x18 bytes,
/// `0061ce40`; 0 when the allocation fails) at +0x38 and `0061cdc0` with 1.
/// Returns `this`. The engine map gives it no name.
pub fn fn_0061ccd0(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    let base = this.addr();
    e.call(TES_FORM_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, TOPIC_INFO_VTABLE);
    e.call(CONDITIONS_CONSTRUCT, &args![base + 0x18]);
    e.call(STRING_CONSTRUCT, &args![base + 0x28]);
    e.call(LIST_CONSTRUCT, &args![base + 0x30]);
    fn_0061cf40(e, this);
    e.mem.set_u8(base + 0x23, kind as u8);
    if kind == 1 {
        let block = e.call(OPERATOR_NEW, &args![CONVERSATION_SIZE]).u32();
        let data = if block != 0 {
            fn_0061ce40(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.mem.set_u32(base + 0x38, data);
        e.call(INFO_0061CDC0, &args![this, 1u32]);
    }
    this
}

// Translated from 0061ce10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape: the destructor body `0061ceb0`, then
/// `this` freed (`00401030`) when bit 0 of the flags is set; `RET 4`.
/// Returns `this`. The engine map gives it no name.
pub fn fn_0061ce10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0061ceb0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0061ce40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061ce40 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs a `TESConversationData` (its three lists at +0, +8 and
/// +0x10, Xbox PDB; `0096a2d0` each). Returns `this`. The engine map gives
/// it no name.
pub fn fn_0061ce40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(LIST_CONSTRUCT, &args![this]);
    e.call(LIST_CONSTRUCT, &args![base + 0x8]);
    e.call(LIST_CONSTRUCT, &args![base + 0x10]);
    this
}

// Translated from 0061ceb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061ceb0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESTopicInfo`'s destructor body (the engine map names it
/// `TESTopicInfo::_scalar_deleting_destructor_`, but it frees nothing):
/// the vtable, `ClearData` (`0061cff0`), then the topic list (+0x30), the
/// prompt (+0x28), the conditions (+0x18) and the `TESForm` base
/// destroyed.
pub fn fn_0061ceb0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, TOPIC_INFO_VTABLE);
    tes_topic_info_clear_data(e, this);
    e.call(LIST_DESTRUCT, &args![base + 0x30]);
    e.call(STRING_DESTRUCT, &args![base + 0x28]);
    e.call(CONDITIONS_DESTRUCT, &args![base + 0x18]);
    e.call(TES_FORM_DESTRUCT, &args![this]);
}

// Translated from 0061cf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The info's start values: the four data bytes (+0x23) cleared, the file
/// offset (+0x4c) 0, `bSaidOnce` 0, `iInfoIndex` 0xffff, form type 0x46
/// (`004f15a0`), the conversation data (+0x38) and +0x48 0, the prompt set
/// empty (`0061cfd0`), `00409480` with 0, `0087ce80` with 0 twice and the
/// difficulty set to 1 (`008d8040`). The engine map gives it no name.
pub fn fn_0061cf40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(FILL_MEMORY, &args![base + 0x23, 0u32, 4u32]);
    e.mem.set_u32(base + INFO_FILE_OFFSET, 0);
    e.mem.set_u8(base + 0x22, 0);
    e.mem.set_u16(base + 0x20, 0xffff);
    e.call(SET_FORM_TYPE, &args![this, 0x46u32]);
    e.mem.set_u32(base + 0x38, 0);
    e.mem.set_u32(base + 0x48, 0);
    fn_0061cfd0(e, this, EMPTY_STRING);
    e.call(INFO_SET_00409480, &args![this, 0u32]);
    e.call(INFO_SET_0087CE80, &args![this, 0u32]);
    e.call(INFO_SET_0087CE80, &args![this, 0u32]);
    e.call(SET_DIFFICULTY, &args![this, 1u32]);
}

// Translated from 0061cfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the prompt string (+0x28) to the characters given (`004037f0` with
/// 0; `RET 4`). The engine map gives it no name.
pub fn fn_0061cfd0(e: &mut Engine, this: Ptr, text: u32) {
    e.call(STRING_SET, &args![this.addr() + 0x28, text, 0u32]);
}

// Translated from 0061cff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061cff0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESTopicInfo::ClearData` (Xbox PDB). Unless `004077c0` reports true:
/// an info with an index (`iInfoIndex` not 0xffff) and a topic
/// (`TESTopic::GetOwnerTopic`) leaves it (`0061d140`); the conditions are
/// cleared (`00680fe0`), the shared responses (`0061d230`) and the topic
/// list (`0061d250`) emptied, and both shared result scripts (`011cbf60`,
/// `011cbfb4`) copied from a new stack script (`Script::CopyResultScript`)
/// that is then destroyed. Then the result scripts' info (`011cbf40`) and
/// the shared responses' info (`011cbf3c`) are cleared, and the
/// conversation data (+0x38), when there is one, is deleted (`0061d110`
/// with 1); the field keeps its value.
pub fn tes_topic_info_clear_data(e: &mut Engine, this: Ptr) {
    if !e.call(FORM_004077C0, &args![this]).bool() {
        if e.call(INFO_INDEX, &args![this]).u16() != 0xffff {
            let topic = e.call(GET_OWNER_TOPIC, &args![this]).u32();
            if topic != 0 {
                fn_0061d140(e, this, Ptr::new(topic));
            }
        }
        let conditions = e.call(AT_0X18, &args![this]).u32();
        e.call(CONDITIONS_CLEAR, &args![conditions]);
        fn_0061d230(e, this);
        fn_0061d250(e, this);
        e.with_stack(STACK_SCRIPT_SIZE, |e, script| {
            e.call(SCRIPT_CONSTRUCT, &args![script]);
            e.call(SCRIPT_COPY, &args![RESULT_SCRIPTS, script]);
            e.call(
                SCRIPT_COPY,
                &args![RESULT_SCRIPTS + RESULT_SCRIPT_SIZE, script],
            );
            e.call(SCRIPT_DESTRUCT, &args![script]);
        });
    }
    e.mem.set_u32(SHARED_RESPONSES_INFO, 0);
    e.mem.set_u32(RESULT_SCRIPTS_INFO, 0);
    let data = e.mem.u32(this.addr() + 0x38);
    if data != 0 {
        fn_0061d110(e, Ptr::new(data), 1);
    }
}

// Translated from 0061d110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The conversation data's scalar deleting destructor shape: `0061c190`,
/// then `this` freed (`00401030`) when bit 0 of the flags is set; `RET 4`.
/// Returns `this`. The engine map gives it no name.
pub fn fn_0061d110(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0061c190(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0061d140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the info out of its topic (`thiscall`, the topic; `RET 4`): with
/// a topic, an index (`iInfoIndex` not 0xffff) and `00619c50`'s object for
/// the info, the array of that object (`00717e50`) loses the info: at its
/// index when the element there is the info, otherwise at the first index
/// (counted signed up to the array's count, `0084e3a0`) whose element is;
/// then `0061bdc0` and `00619fe0` on the array. The engine map gives it no
/// name.
pub fn fn_0061d140(e: &mut Engine, this: Ptr, topic: Ptr) {
    if topic.is_null() {
        return;
    }
    if e.call(INFO_INDEX, &args![this]).u16() == 0xffff {
        return;
    }
    let holder = e.call(TOPIC_00619C50, &args![topic, this]).u32();
    if holder == 0 {
        return;
    }
    let array = e.call(ARRAY_OF_00717E50, &args![holder]).u32();
    let index = u32::from(e.call(INFO_INDEX, &args![this]).u16());
    let element = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
    if e.mem.u32(element) == this.addr() {
        let index = u32::from(e.call(INFO_INDEX, &args![this]).u16());
        e.call(ARRAY_REMOVE, &args![array, index]);
    } else {
        let count = e.call(ARRAY_COUNT, &args![array]).u32() as i32;
        let mut index = 0i32;
        while index < count {
            let element = e.call(ARRAY_ELEMENT, &args![array, index as u32]).u32();
            if e.mem.u32(element) == this.addr() {
                e.call(ARRAY_REMOVE, &args![array, index as u32]);
                break;
            }
            index += 1;
        }
    }
    e.call(ARRAY_0061BDC0, &args![array]);
    e.call(ARRAY_00619FE0, &args![array]);
}

// Translated from 0061d250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the topic list (+0x30, `00436aa0`): `0063f7b0` on its first
/// node until `008256d0` reports the end (nothing for a null list). The
/// engine map gives it no name.
pub fn fn_0061d250(e: &mut Engine, this: Ptr) {
    let list = e.call(ADD_TOPICS, &args![this]).u32();
    if list == 0 {
        return;
    }
    loop {
        if e.call(NODE_IS_END, &args![list]).bool() {
            break;
        }
        e.call(NODE_0063F7B0, &args![list]);
    }
}

// Translated from 0061d290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061d290 - the exception-unwinding frame (FS:[0]) and the security-cookie check (00ec408c) are not translated.
/// `TESTopicInfo::InitItem` (Xbox PDB): skipped when `004013e0` reports
/// true. Resolves the topic list's form ids to topics (with the info's
/// file, `GetFile(-1)`, cast to `TESTopic`), each one not found taken out
/// with a warning describing the info; the speaker's (`00639b40`) id, when
/// nonzero, resolved and cast (`011846e8`) into `00409480`; the conditions
/// initialised; the conversation data's links (`0061c360`) when it has
/// some; each shared response initialised (`TESResponse::InitItem`); the
/// word at +0x40, when nonzero, resolved and cast to the class at
/// `011861dc` or else the one at `0119a554`; then `00484ab0` with 1.
pub fn tes_topic_info_init_item(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.call(FORM_004013E0, &args![this]).bool() {
        return;
    }
    let file = e.call(GET_FILE, &args![this, ANY_FILE]).u32();
    let first = e.call(ADD_TOPICS, &args![this]).u32();
    resolve_list(
        e,
        first,
        file,
        |e, id| {
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            e.call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_TOPIC, 0u32],
            )
            .u32()
        },
        |e, id| warn_with_description(e, this, id, FORMAT_NO_TOPIC, INFO_DESCRIPTION_SIZE),
    );
    let speaker = e.call(SPEAKER, &args![this]).u32();
    if speaker != 0 {
        let cast = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), speaker);
            let file = e.call(GET_FILE, &args![this, ANY_FILE]).u32();
            e.call(ADD_COMPILE_INDEX, &args![slot, file]);
            let id = e.mem.u32(slot.addr());
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            e.call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_011846E8, 0u32],
            )
            .u32()
        });
        e.call(INFO_SET_00409480, &args![this, cast]);
    }
    if e.call(AT_0X18, &args![this]).u32() != 0 {
        let conditions = e.call(AT_0X18, &args![this]).u32();
        e.call(CONDITION_INIT_ITEM, &args![conditions, this]);
    }
    if e.call(CONVERSATION_DATA, &args![this]).u32() != 0 {
        let data = e.call(CONVERSATION_DATA, &args![this]).u32();
        tes_conversation_data_init_item(e, Ptr::new(data), this);
    }
    let mut response = e.call(WORD_AT_ZERO, &args![SHARED_RESPONSES]).u32();
    while response != 0 {
        e.call(RESPONSE_INIT_ITEM, &args![response, this]);
        response = e.call(RESPONSE_NEXT, &args![response]).u32();
    }
    if e.mem.u32(base + 0x40) != 0 {
        let raw = e.mem.u32(base + 0x40);
        let form = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), raw);
            let file = e.call(GET_FILE, &args![this, ANY_FILE]).u32();
            e.call(ADD_COMPILE_INDEX, &args![slot, file]);
            let id = e.mem.u32(slot.addr());
            e.call(LOOKUP_FORM, &args![id]).u32()
        });
        let first = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_011861DC, 0u32],
            )
            .u32();
        let value = if first != 0 {
            first
        } else {
            e.call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_0119A554, 0u32],
            )
            .u32()
        };
        e.mem.set_u32(base + 0x40, value);
    }
    e.call(FORM_SET_484AB0, &args![this, 1u32]);
}

/// Reads one chunk word into a fresh stack word (zeroed first when the exe
/// zeroes it) and returns it.
fn chunk_word(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        e.call(GET_CHUNK_WORD, &args![file, word]);
        e.mem.u32(word.addr())
    })
}

/// `Load`'s conversation chunks (`TCLT`, `TCLF`, `TCFU`): the conversation
/// data is made when missing (`0061e180` with 0), then the chunk's word is
/// appended (`00905820`, through a stack copy) to the list `getter` gives
/// for the data.
fn load_conversation_link(e: &mut Engine, this: Ptr, file: Ptr, getter: u32) {
    if e.call(CONVERSATION_DATA, &args![this]).u32() == 0 {
        fn_0061e180(e, this, 0);
    }
    let value = chunk_word(e, file);
    e.with_stack(4, |e, copy| {
        e.mem.set_u32(copy.addr(), value);
        let data = e.call(CONVERSATION_DATA, &args![this]).u32();
        let target = e.call(getter, &args![data]).u32();
        e.call(LIST_APPEND, &args![target, copy]);
    });
}

// Translated from 0061dbd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061dbd0 - the security-cookie check (00ec408c) is not translated.
/// `TESTopicInfo::Load` (Xbox PDB; the file; `RET 4`): false for a record
/// not of form type 0x46. Outside an optimized file the record's offset
/// (`00467bb0`) is kept at +0x4c. `TESForm::LoadForm`, then chunk by chunk
/// until a zero tag or no next chunk:
/// - `CTDA`: `TESCondition::Load`; `DATA`: 4 bytes into +0x23, then slot
///   `0xc4` with 1 for a first byte of 8 or more (signed) and `0061e180`
///   with 0 for a first byte of 1;
/// - `NAME`: a topic id appended to the topic list; `QSTI`: the topic id
///   (with the file's compile index); `PNAM`: the previous info's id (with
///   the compile index when nonzero; -1 before);
/// - `TCLT`, `TCLF`, `TCFU`: a topic or info id appended to the
///   conversation data's 'Link To' (`00413f40`), 'Link From' (`006815c0`)
///   or 'Follow Up' (`00460140`) list, the data made first when missing;
/// - `RNAM`: the prompt (0x100 bytes into a stack buffer, `0061cfd0`);
///   `KNAM` into +0x40, `ANAM` into +0x3c; `DNAM`: a difficulty below 6
///   is set (`008d8040`), and a difficulty of 0 becomes 1.
///
/// After the chunks a first data byte of 3 becomes 0, and unless
/// `00440d80` reports true the info is inserted into the topic `011cbeb0`
/// (`TESTopic::InsertInfo` with the topic id and the previous id), with a
/// warning when that fails. Returns true.
pub fn tes_topic_info_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u8() != INFO_FORM_TYPE {
        return false;
    }
    if !e.call(GET_OPTIMIZED_FILE, &args![file]).bool() {
        let offset = e.call(RECORD_OFFSET, &args![file]).u32();
        e.mem.set_u32(base + INFO_FILE_OFFSET, offset);
    }
    e.with_stack(8, |e, ids| {
        let previous = ids;
        let topic = ids.addr() + 4;
        e.mem.set_u32(previous.addr(), 0xffff_ffff);
        e.mem.set_u32(topic, 0);
        e.call(TES_FORM_LOAD_FORM, &args![this, file]);
        loop {
            let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
            if chunk == 0 {
                break;
            }
            match chunk {
                CHUNK_CTDA => {
                    let conditions = e.call(AT_0X18, &args![this]).u32();
                    e.call(CONDITION_LOAD, &args![conditions, file]);
                }
                CHUNK_PNAM => {
                    e.call(GET_CHUNK_WORD, &args![file, previous]);
                    if e.mem.u32(previous.addr()) != 0 {
                        e.call(ADD_COMPILE_INDEX, &args![previous, file]);
                    }
                }
                CHUNK_DATA => {
                    e.call(GET_CHUNK_DATA, &args![file, base + 0x23, 4u32]);
                    if i32::from(e.mem.u8(base + 0x23) as i8) >= 8 {
                        e.vcall(base, 0xc4, &args![1u32]);
                    }
                    if e.mem.u8(base + 0x23) as i8 == 1 {
                        fn_0061e180(e, this, 0);
                    }
                }
                CHUNK_NAME => {
                    let value = chunk_word(e, file);
                    e.with_stack(4, |e, copy| {
                        e.mem.set_u32(copy.addr(), value);
                        let list = e.call(ADD_TOPICS, &args![this]).u32();
                        e.call(LIST_APPEND, &args![list, copy]);
                    });
                }
                CHUNK_QSTI => {
                    e.call(GET_CHUNK_WORD, &args![file, topic]);
                    e.call(ADD_COMPILE_INDEX, &args![topic, file]);
                }
                CHUNK_TCLT => load_conversation_link(e, this, file, LINK_TO),
                CHUNK_TCLF => load_conversation_link(e, this, file, LINK_FROM),
                CHUNK_TCFU => load_conversation_link(e, this, file, FOLLOW_UP),
                CHUNK_RNAM => {
                    e.with_stack(0x100, |e, buffer| {
                        e.call(GET_CHUNK_DATA, &args![file, buffer, 0x100u32]);
                        fn_0061cfd0(e, this, buffer.addr());
                    });
                }
                CHUNK_KNAM => {
                    let value = chunk_word(e, file);
                    e.mem.set_u32(base + 0x40, value);
                }
                CHUNK_ANAM => {
                    let value = chunk_word(e, file);
                    e.mem.set_u32(base + 0x3c, value);
                }
                CHUNK_DNAM => {
                    let difficulty = e.with_stack(4, |e, word| {
                        e.call(GET_CHUNK_WORD, &args![file, word]);
                        e.mem.u32(word.addr())
                    });
                    if difficulty < 6 {
                        e.call(SET_DIFFICULTY, &args![this, difficulty]);
                    }
                    if e.mem.u32(base + 0x44) == 0 {
                        e.mem.set_u32(base + 0x44, 1);
                    }
                }
                _ => {}
            }
            if !e.call(NEXT_CHUNK, &args![file]).bool() {
                break;
            }
        }
        if e.mem.u8(base + 0x23) as i8 == 3 {
            e.mem.set_u8(base + 0x23, 0);
        }
        if !e.call(FORM_00440D80, &args![this]).bool() {
            let current = e.mem.u32(CURRENT_TOPIC);
            let topic_id = e.mem.u32(topic);
            let previous_id = e.mem.u32(previous.addr());
            if !e
                .call(INSERT_INFO, &args![current, topic_id, this, previous_id])
                .bool()
            {
                let current = e.mem.u32(CURRENT_TOPIC);
                let current_id = e.call(FORM_ID, &args![current]).u32();
                let current = e.mem.u32(CURRENT_TOPIC);
                let name = e.vcall(current, 0x130, &[]).u32();
                let info_id = e.call(FORM_ID, &args![this]).u32();
                e.call(
                    LOG_WARNING,
                    &args![FORMAT_NOT_INSERTED, info_id, name, current_id, topic_id],
                );
            }
        }
    });
    true
}

// Translated from 0061e180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061e180 - the exception-unwinding frame (FS:[0]) around the allocations is not translated.
/// Sets up the conversation data (+0x38; one word, `RET 4`). With a
/// nonzero word: the data is made when missing (0x18 bytes, `0061ce40`;
/// 0 when the allocation fails) and `0061c210` runs on it with the info
/// and the word. With 0: an existing data is deleted (`0061d110` with 1)
/// and a new one made. The engine map gives it no name.
pub fn fn_0061e180(e: &mut Engine, this: Ptr, value: u32) {
    let base = this.addr();
    let make = |e: &mut Engine| {
        let block = e.call(OPERATOR_NEW, &args![CONVERSATION_SIZE]).u32();
        let data = if block != 0 {
            fn_0061ce40(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.mem.set_u32(base + 0x38, data);
    };
    if value != 0 {
        if e.mem.u32(base + 0x38) == 0 {
            make(e);
        }
        let data = e.mem.u32(base + 0x38);
        e.call(CONVERSATION_0061C210, &args![data, this, value]);
        return;
    }
    if e.mem.u32(base + 0x38) != 0 {
        let data = e.mem.u32(base + 0x38);
        if data != 0 {
            fn_0061d110(e, Ptr::new(data), 1);
        }
        e.mem.set_u32(base + 0x38, 0);
    }
    make(e);
}

// Translated from 0061e600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the info may be said (`thiscall`, four words: a word, a second
/// object, a word, the speaker; `RET 0x10`; the engine map gives it no
/// name). False when `00440d80` reports true, or `00619b20` and `00619b40`
/// both do. The kind `0061e720` gives decides next: with a speaker that is
/// an actor (its slot `0x100`) whose value (the slot `+8` of the vtable at
/// its +0xa4, with 9) is at most the word `0043d4d0` reads (on `011d0dc8`,
/// signed), kind 3 is false; otherwise kind 1 is false. Then, with a second
/// object, its conditions (`005f5760`) must hold
/// (`TESCondition::IsTrue_ov2` with the third word, the speaker, the first
/// word and 0), and the result is whether the info's own conditions (+0x18)
/// hold the same way.
pub fn fn_0061e600(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    speaker: Ptr,
) -> u8 {
    if e.call(FORM_00440D80, &args![this]).bool() {
        return 0;
    }
    if e.call(INFO_00619B20, &args![this]).bool() && e.call(INFO_00619B40, &args![this]).bool() {
        return 0;
    }
    let kind = fn_0061e720(e, this);
    let mut actor = 0u32;
    if !speaker.is_null() && e.vcall(speaker.addr(), 0x100, &[]).bool() {
        actor = speaker.addr();
    }
    let mut low_value = false;
    if actor != 0 {
        let value = e.vcall(actor + 0xa4, 0x8, &args![9u32]).u32() as i32;
        let limit = e.call(VALUE_0043D4D0, &args![VALUE_HOLDER]).u32();
        if value <= e.mem.u32(limit) as i32 {
            low_value = true;
        }
    }
    if low_value {
        if kind == 3 {
            return 0;
        }
    } else if kind == 1 {
        return 0;
    }
    let mut passes = 1u8;
    if second != 0 {
        let conditions = e.call(CONDITIONS_OF_005F5760, &args![second]).u32();
        passes = e
            .call(
                CONDITION_IS_TRUE,
                &args![conditions, third, speaker, first, 0u32],
            )
            .u8();
    }
    if passes == 0 {
        return 0;
    }
    let conditions = e.call(AT_0X18, &args![this]).u32();
    e.call(
        CONDITION_IS_TRUE,
        &args![conditions, third, speaker, first, 0u32],
    )
    .u8()
}

// Translated from 0061e720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The info's kind from the byte at +0x26: 1 with bit 4 set, else 3 with
/// bit 5 set, else 2. The engine map gives it no name.
pub fn fn_0061e720(e: &mut Engine, this: Ptr) -> u32 {
    let flags = e.mem.u8(this.addr() + 0x26);
    if flags & 0x10 != 0 {
        return 1;
    }
    if flags & 0x20 != 0 {
        3
    } else {
        2
    }
}

/// `GetResultScript`'s referenced-object entry (`SCRO` stores the word at
/// +8, `SCRV` at +0xc) appended to `script`'s referenced objects.
fn result_referenced_object(e: &mut Engine, script: u32, file: u32, offset: u32) {
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let entry = if block != 0 {
        e.call(REFERENCED_OBJECT_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.with_stack(8, |e, slots| {
        e.mem.set_u32(slots.addr(), entry);
        e.call(GET_CHUNK_WORD, &args![file, slots.addr() + 4]);
        let value = e.mem.u32(slots.addr() + 4);
        e.mem.set_u32(entry.wrapping_add(offset), value);
        let list = e.call(AT_0X44, &args![script]).u32();
        e.call(LIST_APPEND, &args![list, slots]);
    });
}

// Translated from 0061eb60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061eb60 - the exception-unwinding frame (FS:[0]) around the allocations and the security-cookie check (00ec408c) are not translated.
/// `TESTopicInfo::GetResultScript` (Xbox PDB; an index; `RET 4`): the
/// shared result script at that index (`011cbf60` + 0x54 * index), read
/// again for an info other than the one they hold (`011cbf40`): both are
/// emptied (copied from a new stack script, made temporary, the byte at
/// +0x10 of their header cleared); when `00508d70` reports true they get
/// `00484ab0` with 0 and the info's record is read again (its file through
/// `GetThreadSafeFile`, at the offset at +0x4c, as a record of form type
/// 0x46; nothing more without one): `NEXT` moves to the next script, and
/// `SCHR`, `SCDA`, `SCRO`, `SCRV`, `SLSD` and `SCVR` load the current one as
/// `PackageEventAction::Load` does (variables through `011cc0a0`). Then
/// both are initialised (`Script::InitItem` with the info). The stack script
/// is destroyed on every path.
pub fn tes_topic_info_get_result_script(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let result = index
        .wrapping_mul(RESULT_SCRIPT_SIZE)
        .wrapping_add(RESULT_SCRIPTS);
    if this.addr() == e.mem.u32(RESULT_SCRIPTS_INFO) {
        return result;
    }
    e.mem.set_u32(RESULT_SCRIPTS_INFO, this.addr());
    e.with_stack(STACK_SCRIPT_SIZE, |e, empty| {
        e.call(SCRIPT_CONSTRUCT, &args![empty]);
        for script in [RESULT_SCRIPTS, RESULT_SCRIPTS + RESULT_SCRIPT_SIZE] {
            e.call(SCRIPT_COPY, &args![script, empty]);
            e.call(SET_TEMPORARY, &args![script]);
            let header = e.call(AT_0X18, &args![script]).u32();
            e.mem.set_u8(header + 0x10, 0);
        }
        read_result_scripts(e, this);
        e.call(SCRIPT_DESTRUCT, &args![empty]);
    });
    result
}

/// The part of `GetResultScript` after the scripts are emptied, up to the
/// stack script's destruction.
fn read_result_scripts(e: &mut Engine, this: Ptr) {
    if !e.call(INFO_00508D70, &args![this]).bool() {
        return;
    }
    e.call(FORM_SET_484AB0, &args![RESULT_SCRIPTS, 0u32]);
    e.call(
        FORM_SET_484AB0,
        &args![RESULT_SCRIPTS + RESULT_SCRIPT_SIZE, 0u32],
    );
    let file = e.call(GET_FILE, &args![this, ANY_FILE]).u32();
    if file == 0 || e.mem.u32(this.addr() + INFO_FILE_OFFSET) == 0 {
        return;
    }
    let file = e.call(GET_THREAD_SAFE_FILE, &args![file]).u32();
    let offset = e.mem.u32(this.addr() + INFO_FILE_OFFSET);
    if !e.call(SET_OFFSET, &args![file, offset]).bool() {
        return;
    }
    if e.call(GET_TES_FORM, &args![file]).u8() != INFO_FORM_TYPE {
        return;
    }
    let mut current = 0u32;
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        let script = current
            .wrapping_mul(RESULT_SCRIPT_SIZE)
            .wrapping_add(RESULT_SCRIPTS);
        match chunk {
            CHUNK_NEXT => current = current.wrapping_add(1),
            CHUNK_SLSD | CHUNK_SCVR => {
                if e.call(GET_TES_CHUNK, &args![file]).u32() == CHUNK_SLSD {
                    let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
                    let variable = if block != 0 {
                        e.call(VARIABLE_CONSTRUCT, &args![block]).u32()
                    } else {
                        0
                    };
                    e.mem.set_u32(RESULT_VARIABLE, variable);
                    let variable = e.mem.u32(RESULT_VARIABLE);
                    e.call(VARIABLE_LOAD, &args![variable, file]);
                    let list = e.call(AT_0X4C, &args![script]).u32();
                    e.call(LIST_APPEND, &args![list, RESULT_VARIABLE]);
                } else if e.mem.u32(RESULT_VARIABLE) != 0 {
                    let size = e.call(CHUNK_SIZE, &args![file]).u32();
                    e.with_stack(size.wrapping_add(15) & !15, |e, buffer| {
                        e.call(GET_CHUNK_DATA, &args![file, buffer, NAME_MAXIMUM]);
                        let variable = e.mem.u32(RESULT_VARIABLE);
                        e.call(STRING_SET, &args![variable + 0x18, buffer, 0u32]);
                    });
                }
            }
            CHUNK_SCHR => {
                let header = e.call(AT_0X18, &args![script]).u32();
                e.call(GET_CHUNK_DATA, &args![file, header, 0u32]);
                if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
                    let header = e.call(AT_0X18, &args![script]).u32();
                    e.call(HEADER_SWAP, &args![header]);
                }
            }
            CHUNK_SCDA => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size.wrapping_add(15) & !15, |e, buffer| {
                    e.call(FILL_MEMORY, &args![buffer, 0u32, size]);
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0u32]);
                    e.call(SET_COMPILE_DATA, &args![script, size, buffer]);
                });
            }
            CHUNK_SCRO => result_referenced_object(e, script, file, 0x8),
            CHUNK_SCRV => result_referenced_object(e, script, file, 0xc),
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    e.call(SCRIPT_INIT_ITEM, &args![RESULT_SCRIPTS, this]);
    e.call(
        SCRIPT_INIT_ITEM,
        &args![RESULT_SCRIPTS + RESULT_SCRIPT_SIZE, this],
    );
}

// Translated from 0061f150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTopicInfo::AddTopicList` (Xbox PDB): the topic list (+0x30) handed to
/// the player (`009527b0` on `011dea3c`).
pub fn tes_topic_info_add_topic_list(e: &mut Engine, this: Ptr) {
    let list = e.call(ADD_TOPICS, &args![this]).u32();
    let player = e.mem.u32(PLAYER);
    e.call(PLAYER_009527B0, &args![player, list]);
}

// Translated from 0061f170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTopicInfo::RunResult` (Xbox PDB; an index and a reference; `RET 8`):
/// nothing without a reference. The result script at the index
/// (`GetResultScript`) runs on the reference (`Script::Run` with the
/// reference's script locals, 0 and 1) when scripts are processed
/// (`Script::GetProcessScripts`) and its data header's word at +8 is not 0;
/// then `0061f220`, and `00935920` on the reference with the info when
/// `0061aef0` and the reference's slot `0xfc` report true.
pub fn tes_topic_info_run_result(e: &mut Engine, this: Ptr, index: u32, reference: Ptr) {
    if reference.is_null() {
        return;
    }
    let script = tes_topic_info_get_result_script(e, this, index);
    if e.call(PROCESS_SCRIPTS, &[]).bool() && script != 0 {
        let header = e.call(AT_0X18, &args![script]).u32();
        if e.mem.u32(header + 8) != 0 {
            let list = e.call(AT_0X44, &args![reference]).u32();
            let locals = e.call(SCRIPT_LOCALS, &args![list]).u32();
            e.call(SCRIPT_RUN, &args![script, reference, locals, 0u32, 1u32]);
        }
    }
    e.call(INFO_0061F220, &args![this]);
    if e.call(INFO_0061AEF0, &args![this]).bool() && e.vcall(reference.addr(), 0xfc, &[]).bool() {
        e.call(REFERENCE_00935920, &args![reference, this]);
    }
}

/// RTTI type descriptor `.?AVTESTopicInfo@@` (read from the exe).
const TYPE_TES_TOPIC_INFO: u32 = 0x0118_4738;
/// The word at `01187020` a condition object's first word (its vtable) must equal, and the
/// word at `01187344` a condition's function word (+8) is compared with by `0061d8c0`.
const CONDITION_VTABLE_WORD: u32 = 0x0118_7020;
const CONDITION_FUNCTION_WORD: u32 = 0x0118_7344;
/// `00484020` (`thiscall`, `RET 4`; `tesform.cpp`): the form's own answer for other forms;
/// `00484730` (`RET 4`), `00485be0` (`RET 4`), `TESForm::LoadGame` (`00484c50`, `RET 8`),
/// `004534f0` (`RET 4`) and `00484da0` (`RET 4`): the form base methods the info overrides
/// call.
const FORM_ANSWER_484020: u32 = 0x0048_4020;
const FORM_SET_484730: u32 = 0x0048_4730;
const FORM_TEST_485BE0: u32 = 0x0048_5be0;
const FORM_LOAD_GAME: u32 = 0x0048_4c50;
const FORM_4534F0: u32 = 0x0045_34f0;
const FORM_484DA0: u32 = 0x0048_4da0;
/// The save buffer's flag words: `00428110` (load) and `0042ce30` (save) copy the buffer's
/// word to a local (`RET 4`), `004280f0` (`RET 4`) tests a flag in it.
const BUFFER_FLAGS_LOAD: u32 = 0x0042_8110;
const BUFFER_FLAGS_SAVE: u32 = 0x0042_ce30;
const FLAGS_TEST: u32 = 0x0042_80f0;
/// `00680a70` (`thiscall` on the conditions, `RET 8`: source, owner): copies conditions.
const CONDITIONS_COPY: u32 = 0x0068_0a70;
/// `00401280` and `00440e30` (`thiscall` on the form): the two words `GetFormDetailedString`
/// prints first (`00440e30` with the type name in its format, `00401280` the editor id).
const FORM_TEXT_AT_401280: u32 = 0x0040_1280;
const FORM_TEXT_AT_440E30: u32 = 0x0044_0e30;
/// The string printf (`00406f60`, `cdecl`: string, format, ...).
const STRING_PRINTF: u32 = 0x0040_6f60;
/// The formats `GetFormDetailedString` uses.
const FORMAT_DETAILED: u32 = 0x0104_d71c;
const FORMAT_TOPIC: u32 = 0x0104_d75c;
const FORMAT_QUEST: u32 = 0x0104_d748;
const FORMAT_TEXT: u32 = 0x0104_d738;
/// `LowProcess::GetGenericLocation` (`008041a0`), `LowProcess::GetSecondGenericLocation`
/// (`006733e0`) and `SetSecondGenericLocation` (`00673400`, `RET 4`), engine map names; the
/// set of the first is `SET_DIFFICULTY` (`008d8040`).
const GET_GENERIC_LOCATION: u32 = 0x0080_41a0;
const GET_SECOND_GENERIC_LOCATION: u32 = 0x0067_33e0;
const SET_SECOND_GENERIC_LOCATION: u32 = 0x0067_3400;
/// `004610d0` (`thiscall` on an info): the object whose first word `0061cfd0` takes.
const INFO_TEXT_HOLDER: u32 = 0x0046_10d0;
/// `0061f340` (`thiscall`, `RET 4`): sets the info's index (16 bits, zero-extended).
const SET_INFO_INDEX: u32 = 0x0061_f340;
/// The form types `0061e330` gives to `0087ce80` instead of copying (0x56, 0x59).
const FORM_TYPE_56: u32 = 0x56;
const FORM_TYPE_59: u32 = 0x59;
/// `TESForm::CopyAllComponents` (engine map, `004851b0`, `RET 4`).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;

/// Appends each item of the list from `node` (until an empty item) to `list` (`00905820`
/// with the item's stack word).
fn append_items(e: &mut Engine, mut node: u32, list_of: impl Fn(&mut Engine) -> u32, slot: u32) {
    e.mem.set_u32(slot, 0);
    while node != 0 {
        let at = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(at) == 0 {
            break;
        }
        let at = e.call(NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(at);
        e.mem.set_u32(slot, item);
        node = e.call(NODE_NEXT, &args![node]).u32();
        let list = list_of(e);
        e.call(LIST_APPEND, &args![list, slot]);
    }
}

// Translated from 0061c210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX` = conversation data, `RET 8`: the info (not read) and other
/// conversation data): with other data, empties this one (`0061c330`) and appends the other's
/// link-to (`00413f40`), link-from (`006815c0`) and follow-up (`00460140`) topics, each list
/// until an empty item (the follow-ups stop the whole function there).
pub fn fn_0061c210(e: &mut Engine, this: Ptr, _unused_1: u32, other: u32) {
    if other == 0 {
        return;
    }
    e.call(CONVERSATION_0061C330, &args![this]);
    e.with_stack(8, |e, locals| {
        let first = locals.addr();
        let second = first + 4;
        e.mem.set_u32(second, 0);
        let node = e.call(LINK_TO, &args![other]).u32();
        append_items(e, node, |e| e.call(LINK_TO, &args![this]).u32(), first);
        let node = e.call(LINK_FROM, &args![other]).u32();
        append_items(e, node, |e| e.call(LINK_FROM, &args![this]).u32(), first);
        let mut node = e.call(FOLLOW_UP, &args![other]).u32();
        while node != 0 {
            let at = e.call(NODE_ITEM, &args![node]).u32();
            let item = e.mem.u32(at);
            e.mem.set_u32(second, item);
            if item == 0 {
                return;
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
            let list = e.call(FOLLOW_UP, &args![this]).u32();
            e.call(LIST_APPEND, &args![list, second]);
        }
    });
}

// Translated from 0061cdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: a byte): sets bit 2 of the byte at +0x25 when the byte is
/// set, clears it otherwise.
pub fn fn_0061cdc0(e: &mut Engine, this: Ptr, set: u8) {
    let at = this.addr().wrapping_add(0x25);
    let flags = e.mem.u8(at);
    let flags = if set != 0 { flags | 2 } else { flags & 0xfd };
    e.mem.set_u8(at, flags);
}

/// The owner topic of an info (`TESTopic::GetOwnerTopic`, `cdecl`).
fn owner_topic(e: &mut Engine, info: u32) -> u32 {
    e.call(GET_OWNER_TOPIC, &args![info]).u32()
}

// Translated from 0061d6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: a byte): when `004077c0` is false, the byte is set and
/// `004013e0` is true, tells the owner topic (vtable slot 0xC8 with 1); then the form's
/// `00484730` with the byte.
pub fn fn_0061d6a0(e: &mut Engine, this: Ptr, flag: u8) {
    if !e.call(FORM_004077C0, &args![this]).bool()
        && flag != 0
        && e.call(FORM_004013E0, &args![this]).bool()
    {
        let topic = owner_topic(e, this.addr());
        if topic != 0 {
            e.vcall(topic, 0xc8, &args![1u32]);
        }
    }
    e.call(FORM_SET_484730, &args![this, u32::from(flag)]);
}

// Translated from 0061d710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: a form): for a topic (type 0x45), vtable slot 0x3C of this
/// info's owner topic for it (when both exist and differ); for an info (type 0x46), the same
/// between the two infos' owner topics (asked of this one's); otherwise `00484020`. False when
/// a cast or owner is missing.
pub fn fn_0061d710(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let kind = e.call(FORM_TYPE, &args![form]).u32();
    if kind == 0x45 {
        let topic = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_TOPIC, 0u32],
            )
            .u32();
        if topic == 0 {
            return false;
        }
        let owner = owner_topic(e, this.addr());
        if owner != 0 && owner != topic {
            return e.vcall(owner, 0x3c, &args![topic]).bool();
        }
        false
    } else if kind == 0x46 {
        let info = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_TOPIC_INFO, 0u32],
            )
            .u32();
        if info == 0 {
            return false;
        }
        let mine = owner_topic(e, this.addr());
        let theirs = owner_topic(e, info);
        if theirs != 0 && mine != 0 && theirs != mine {
            return e.vcall(mine, 0x3c, &args![theirs]).bool();
        }
        false
    } else {
        e.call(FORM_ANSWER_484020, &args![this, form]).bool()
    }
}

// Translated from 0061d820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 0xC`: a condition, two bytes): false unless the condition's
/// first word is the word at `01187020` and this info has an owner topic; with the first
/// byte set the topic's vtable slot 0x110 answers (the condition and both bytes); when that
/// is false and the condition's word at +0xC is 7, `00485be0` with its word at +8 can make it
/// true.
pub fn fn_0061d820(e: &mut Engine, this: Ptr, condition: u32, check: u8, extra: u8) -> bool {
    if condition == 0 || e.mem.u32(condition) != e.global::<u32>(CONDITION_VTABLE_WORD) {
        return false;
    }
    let topic = owner_topic(e, this.addr());
    if topic == 0 {
        return false;
    }
    let mut result = false;
    if check != 0 {
        result = e
            .vcall(
                topic,
                0x110,
                &args![condition, u32::from(check), u32::from(extra)],
            )
            .bool();
    }
    if !result && e.mem.u32(condition.wrapping_add(0xc)) == 7 {
        let id = e.mem.u32(condition.wrapping_add(8));
        if e.call(FORM_TEST_485BE0, &args![topic, id]).bool() {
            result = true;
        }
    }
    result
}

// Translated from 0061d8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 8`: an output condition, a condition): clears the output's first
/// word; for a condition whose word at +0xC is 0 and whose function word (+8) is the word at
/// `01187344`, when this info has an owner topic, fills the output with the word at
/// `01187020`, kind 7 (+0xC), the topic's form id (+8) and zeros at +4 and +0x10.
pub fn fn_0061d8c0(e: &mut Engine, this: Ptr, out: u32, condition: u32) {
    if out == 0 {
        return;
    }
    e.mem.set_u32(out, 0);
    if condition == 0 || e.mem.u32(condition.wrapping_add(0xc)) != 0 {
        return;
    }
    if e.mem.u32(condition.wrapping_add(8)) != e.global::<u32>(CONDITION_FUNCTION_WORD) {
        return;
    }
    let topic = owner_topic(e, this.addr());
    if topic == 0 {
        return;
    }
    let first = e.global::<u32>(CONDITION_VTABLE_WORD);
    e.mem.set_u32(out, first);
    e.mem.set_u32(out.wrapping_add(0xc), 7);
    let id = e.call(FORM_ID, &args![topic]).u32();
    e.mem.set_u32(out.wrapping_add(8), id);
    e.mem.set_u32(out.wrapping_add(4), 0);
    e.mem.set_u32(out.wrapping_add(0x10), 0);
}

// Translated from 0061d950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061d950 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESTopicInfo::GetFormDetailedString` (engine map, `RET 4`: a string): prints into the
/// string (`00406f60`, "%s Form '%s' (%08X): %s%s%s") the words of `00440e30` and `00401280`,
/// the form id, then the quest part ("Quest '%s' (%08X)": the owner topic's quest `00619d00`
/// when `004013e0` says so, its name from slot 0x138), the topic part (", Topic '%s' (%08X)":
/// the owner topic's name from slot 0x130) and the text part (", Text: \"%s\"": the first
/// response's text), each built with `sprintf` into a stack buffer that stays empty without
/// its object. The stack cookie check is not translated.
pub fn tes_topic_info_get_form_detailed_string(e: &mut Engine, this: Ptr, string: u32) {
    e.with_stack(0x318, |e, buffers| {
        let quest_part = buffers.addr();
        let topic_part = quest_part + 0x108;
        let text_part = quest_part + 0x210;
        e.mem.set_u8(topic_part, 0);
        let topic = owner_topic(e, this.addr());
        if topic != 0 {
            let id = e.call(FORM_ID, &args![topic]).u32();
            let name = e.vcall(topic, 0x130, &[]).u32();
            e.call(SPRINTF, &args![topic_part, FORMAT_TOPIC, name, id]);
        }
        e.mem.set_u8(quest_part, 0);
        if topic != 0 && e.call(FORM_004013E0, &args![this]).bool() {
            let quest = e.call(TOPIC_QUEST, &args![topic, this]).u32();
            if quest != 0 {
                let id = e.call(FORM_ID, &args![quest]).u32();
                let name = e.vcall(quest, 0x138, &[]).u32();
                e.call(SPRINTF, &args![quest_part, FORMAT_QUEST, name, id]);
            }
        }
        e.mem.set_u8(text_part, 0);
        let list = tes_topic_info_get_response_list(e, this, Ptr::NULL).addr();
        let response = e.call(WORD_AT_ZERO, &args![list]).u32();
        if response != 0 {
            let text = e.call(GET_RESPONSE_TEXT, &args![response]).u32();
            e.call(SPRINTF, &args![text_part, FORMAT_TEXT, text]);
        }
        let id = e.call(FORM_ID, &args![this]).u32();
        let editor = e.call(FORM_TEXT_AT_401280, &args![this]).u32();
        let kind = e.call(FORM_TEXT_AT_440E30, &args![this]).u32();
        e.call(
            STRING_PRINTF,
            &args![
                string,
                FORMAT_DETAILED,
                kind,
                editor,
                id,
                quest_part,
                topic_part,
                text_part
            ],
        );
    });
}

// Translated from 0061dad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 8`: flags and a word): `TESForm::LoadGame` with both, then the
/// byte at +0x22 set when flag 0x80000000 is.
pub fn fn_0061dad0(e: &mut Engine, this: Ptr, flags: u32, word: u32) {
    e.call(FORM_LOAD_GAME, &args![this, flags, word]);
    if flags & 0x8000_0000 != 0 {
        e.mem.set_u8(this.addr().wrapping_add(0x22), 1);
    }
}

// Translated from 0061db00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: flags): `004534f0` with them, then the byte at +0x22 cleared
/// when flag 0x80000000 is set.
pub fn fn_0061db00(e: &mut Engine, this: Ptr, flags: u32) {
    e.call(FORM_4534F0, &args![this, flags]);
    if flags & 0x8000_0000 != 0 {
        e.mem.set_u8(this.addr().wrapping_add(0x22), 0);
    }
}

/// Whether the save buffer's flag word (copied to a stack word by `copy`) has 0x80000000
/// (`004280f0`).
fn buffer_has_flag(e: &mut Engine, buffer: u32, copy: u32) -> bool {
    e.with_stack(4, |e, local| {
        let flags = e.call(copy, &args![buffer, local]).u32();
        e.call(FLAGS_TEST, &args![flags, 0x8000_0000u32]).bool()
    })
}

// Translated from 0061db30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: a load buffer): `00484da0` with it, then the byte at +0x22
/// set when the buffer's flags (`00428110`) have 0x80000000.
pub fn fn_0061db30(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(FORM_484DA0, &args![this, buffer]);
    if buffer_has_flag(e, buffer, BUFFER_FLAGS_LOAD) {
        e.mem.set_u8(this.addr().wrapping_add(0x22), 1);
    }
}

// Translated from 0061db80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: a save buffer): `004534f0` with it, then the byte at +0x22
/// cleared when the buffer's flags (`0042ce30`) have 0x80000000.
pub fn fn_0061db80(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(FORM_4534F0, &args![this, buffer]);
    if buffer_has_flag(e, buffer, BUFFER_FLAGS_SAVE) {
        e.mem.set_u8(this.addr().wrapping_add(0x22), 0);
    }
}

// Translated from 0061e0f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061e0f0 - the FS:[0] exception frame (destroying the stack conditions on unwinding) is not translated.
/// Unnamed in the map (`RET 4`: conditions or null): copies the conditions into this info's
/// (+0x18, `00680a70` with this info as owner); without any, copies an empty conditions object
/// built on the stack (`00680890`, then `006808b0`). The exception frame is not translated.
pub fn fn_0061e0f0(e: &mut Engine, this: Ptr, conditions: u32) {
    let target = this.addr().wrapping_add(0x18);
    if conditions != 0 {
        e.call(CONDITIONS_COPY, &args![target, conditions, this]);
        return;
    }
    e.with_stack(8, |e, empty| {
        e.call(CONDITIONS_CONSTRUCT, &args![empty]);
        e.call(CONDITIONS_COPY, &args![target, empty, this]);
        e.call(CONDITIONS_DESTRUCT, &args![empty]);
    });
}

// Translated from 0061e330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0061e330 - the FS:[0] exception frame (deleting a new response on unwinding) is not translated.
/// Unnamed in the map (`RET 4`: a form; the info's copy by its body): when the form is a
/// `TESTopicInfo`, copies all components and its word at +0x23 (`0061e5e0`), rebuilds the
/// added topics (`0061d250`, then each of the source's `00436aa0` topics appended until an
/// empty item), empties the responses (`0061d230`) and copies each source response (`0045cd60`
/// walk) into a new one (`00401000`, `00616ce0`, `006174d0`, added by `0061cc60`), the text
/// (`004610d0`, `0061cfd0`), the +0x40 form (types 0x56 and 0x59 and null go through
/// `0087ce80`, others are copied), the two generic locations, the speaker (`00639b40`,
/// `00409480`), both result scripts (`Script::CopyResultScript`), the index (`00619a70`,
/// `0061f340`), the conditions (`0061e0f0` with the source's `00500940`), the conversation
/// data (`009e32d0`, `0061e180`) and the flag `004013e0` (`00484ab0`). The exception frame is
/// not translated.
pub fn fn_0061e330(e: &mut Engine, this: Ptr, form: u32) {
    let base = this.addr();
    let source = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_TOPIC_INFO, 0u32],
        )
        .u32();
    if source == 0 {
        return;
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, form]);
    let at = fn_0061e5e0(e, Ptr::new(source)).addr();
    let word = e.mem.u32(at);
    e.mem.set_u32(base.wrapping_add(0x23), word);
    fn_0061d250(e, this);
    e.with_stack(4, |e, slot| {
        let node = e.call(ADD_TOPICS, &args![source]).u32();
        append_items(
            e,
            node,
            |e| e.call(ADD_TOPICS, &args![this]).u32(),
            slot.addr(),
        );
    });
    fn_0061d230(e, this);
    let list = tes_topic_info_get_response_list(e, Ptr::new(source), Ptr::NULL).addr();
    let mut response = e.call(WORD_AT_ZERO, &args![list]).u32();
    while response != 0 {
        let block = e.call(OPERATOR_NEW, &args![RESPONSE_SIZE]).u32();
        let copy = if block != 0 {
            e.call(RESPONSE_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.call(RESPONSE_COPY, &args![copy, response]);
        let list = tes_topic_info_get_response_list(e, this, Ptr::NULL).addr();
        fn_0061cc60(e, Ptr::new(list), copy);
        response = e.call(RESPONSE_NEXT, &args![response]).u32();
    }
    let holder = e.call(INFO_TEXT_HOLDER, &args![source]).u32();
    let text = e.call(WORD_AT_ZERO, &args![holder]).u32();
    fn_0061cfd0(e, this, text);
    let other = e.mem.u32(source.wrapping_add(0x40));
    if other == 0 {
        e.call(INFO_SET_0087CE80, &args![this, 0u32]);
    } else {
        let kind = e.call(FORM_TYPE, &args![other]).u32();
        if kind == FORM_TYPE_56 || kind == FORM_TYPE_59 {
            let other = e.mem.u32(source.wrapping_add(0x40));
            e.call(INFO_SET_0087CE80, &args![this, other]);
        } else {
            let other = e.mem.u32(source.wrapping_add(0x40));
            e.mem.set_u32(base.wrapping_add(0x40), other);
        }
    }
    let location = e.call(GET_GENERIC_LOCATION, &args![source]).u32();
    e.call(SET_DIFFICULTY, &args![this, location]);
    let speaker = e.call(SPEAKER, &args![source]).u32();
    e.call(INFO_SET_00409480, &args![this, speaker]);
    for index in 0..2u32 {
        let from = tes_topic_info_get_result_script(e, Ptr::new(source), index);
        let to = tes_topic_info_get_result_script(e, this, index);
        e.call(SCRIPT_COPY, &args![to, from]);
    }
    let info_index = u32::from(e.call(INFO_INDEX, &args![source]).u16());
    e.call(SET_INFO_INDEX, &args![this, info_index]);
    let conditions = e.call(AT_0X18, &args![source]).u32();
    fn_0061e0f0(e, this, conditions);
    if e.call(CONVERSATION_DATA, &args![source]).u32() != 0 {
        let data = e.call(CONVERSATION_DATA, &args![source]).u32();
        fn_0061e180(e, this, data);
    }
    let location = e.call(GET_SECOND_GENERIC_LOCATION, &args![source]).u32();
    e.call(SET_SECOND_GENERIC_LOCATION, &args![this, location]);
    let flag = u32::from(e.call(FORM_004013E0, &args![source]).u8());
    e.call(FORM_SET_484AB0, &args![this, flag]);
}

// Translated from 0061e5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX`): `this + 0x23`.
pub fn fn_0061e5e0(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x23))
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_cad0, fn_0061cad0(Ptr)),
        entry!(
            0x0061_cb30,
            fn_0061cb30(Ptr<TesResponse>, u32) -> Ptr<TesResponse>
        ),
        entry!(0x0061_cb60, fn_0061cb60(Ptr, u32)),
        entry!(0x0061_cc40, fn_0061cc40(Ptr<TesResponse>, u32)),
        entry!(0x0061_cc60, fn_0061cc60(Ptr, u32)),
        entry!(0x0061_cc90, fn_0061cc90(Ptr<TesResponse>, u32)),
        entry!(0x0061_d230, fn_0061d230(Ptr)),
        entry!(
            0x0061_e2b0,
            tes_topic_info_get_info_display_text(Ptr, Ptr, u8)
        ),
        entry!(0x0061_e780, tes_topic_info_get_response_list(Ptr, Ptr) -> Ptr),
        entry!(0x0061_eb40, fn_0061eb40(Ptr<TesResponse>, Ptr)),
        entry!(0x0061_c190, fn_0061c190(Ptr)),
        entry!(0x0061_c360, tes_conversation_data_init_item(Ptr, Ptr)),
        entry!(0x0061_ccd0, fn_0061ccd0(Ptr, u32) -> Ptr),
        entry!(0x0061_ce10, fn_0061ce10(Ptr, u32) -> Ptr),
        entry!(0x0061_ce40, fn_0061ce40(Ptr) -> Ptr),
        entry!(0x0061_ceb0, fn_0061ceb0(Ptr)),
        entry!(0x0061_cf40, fn_0061cf40(Ptr)),
        entry!(0x0061_cfd0, fn_0061cfd0(Ptr, u32)),
        entry!(0x0061_cff0, tes_topic_info_clear_data(Ptr)),
        entry!(0x0061_d110, fn_0061d110(Ptr, u32) -> Ptr),
        entry!(0x0061_d140, fn_0061d140(Ptr, Ptr)),
        entry!(0x0061_d250, fn_0061d250(Ptr)),
        entry!(0x0061_d290, tes_topic_info_init_item(Ptr)),
        entry!(0x0061_dbd0, tes_topic_info_load(Ptr, Ptr) -> bool),
        entry!(0x0061_e180, fn_0061e180(Ptr, u32)),
        entry!(0x0061_e600, fn_0061e600(Ptr, u32, u32, u32, Ptr) -> u8),
        entry!(0x0061_e720, fn_0061e720(Ptr) -> u32),
        entry!(0x0061_eb60, tes_topic_info_get_result_script(Ptr, u32) -> u32),
        entry!(0x0061_f150, tes_topic_info_add_topic_list(Ptr)),
        entry!(0x0061_f170, tes_topic_info_run_result(Ptr, u32, Ptr)),
        entry!(0x0061_c210, fn_0061c210(Ptr, u32, u32)),
        entry!(0x0061_cdc0, fn_0061cdc0(Ptr, u8)),
        entry!(0x0061_d6a0, fn_0061d6a0(Ptr, u8)),
        entry!(0x0061_d710, fn_0061d710(Ptr, u32) -> bool),
        entry!(0x0061_d820, fn_0061d820(Ptr, u32, u8, u8) -> bool),
        entry!(0x0061_d8c0, fn_0061d8c0(Ptr, u32, u32)),
        entry!(
            0x0061_d950,
            tes_topic_info_get_form_detailed_string(Ptr, u32)
        ),
        entry!(0x0061_dad0, fn_0061dad0(Ptr, u32, u32)),
        entry!(0x0061_db00, fn_0061db00(Ptr, u32)),
        entry!(0x0061_db30, fn_0061db30(Ptr, u32)),
        entry!(0x0061_db80, fn_0061db80(Ptr, u32)),
        entry!(0x0061_e0f0, fn_0061e0f0(Ptr, u32)),
        entry!(0x0061_e330, fn_0061e330(Ptr, u32)),
        entry!(0x0061_e5e0, fn_0061e5e0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Doubles that behave as the exe's bodies where the code follows
    /// links (`pNext` at +0x28, the word at +0), the rest record only.
    fn setup() -> Engine {
        let mut e = Engine::new();
        e.register(RESPONSE_NEXT, |e, a| ret(e.mem.u32(a[0] + 0x28)));
        e.register(WORD_AT_ZERO, |e, a| ret(e.mem.u32(a[0])));
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(RESPONSE_CONSTRUCT, |_, a| ret(a[0]));
        for addr in [
            OPERATOR_DELETE,
            RESPONSE_DESTRUCT,
            RESPONSE_COPY,
            RESPONSE_LOAD,
            RESPONSE_INIT_ITEM,
            STRING_SET,
            STRING_APPEND,
            ADD_COMPILE_INDEX,
            LOG_WARNING,
            SET_SNAM_IDLE,
            SET_LNAM_IDLE,
            GET_CHUNK_DATA,
        ] {
            e.register(addr, |_, _| ret(0));
        }
        e.register(GET_RESPONSE_TEXT, |_, a| ret(a[0] + 0x18));
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(FORM_TYPE, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.map(SHARED_RESPONSES_INFO, 4);
        e.map(SHARED_RESPONSES, 4);
        e
    }

    fn chain(e: &mut Engine, n: usize) -> Vec<u32> {
        let r: Vec<u32> = (0..n).map(|_| e.mem.alloc(RESPONSE_SIZE)).collect();
        for w in r.windows(2) {
            e.mem.set_u32(w[0] + 0x28, w[1]);
        }
        r
    }

    #[test]
    fn list_append_link_and_clear() {
        let mut e = setup();
        let head: Ptr = Ptr::new(e.mem.alloc(4));
        let r = chain(&mut e, 2);
        let extra = e.mem.alloc(RESPONSE_SIZE);
        e.call(0x0061_cc60, &args![head, r[0]]);
        assert_eq!(e.mem.u32(head.addr()), r[0]);
        e.mem.set_u32(r[1] + 0x28, 0);
        e.call(0x0061_cc60, &args![head, extra]);
        assert_eq!(e.mem.u32(r[1] + 0x28), extra);
        e.call(0x0061_cc40, &args![r[0], 0x1234u32]);
        assert_eq!(e.mem.u32(r[0] + 0x28), 0x1234);
        e.mem.set_u32(r[0] + 0x28, r[1]);

        e.call_log = Some(vec![]);
        e.call(0x0061_cad0, &args![head]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(head.addr()), 0);
        assert_eq!(
            calls(&log, RESPONSE_DESTRUCT),
            [vec![r[0]], vec![r[1]], vec![extra]]
        );
        assert_eq!(calls(&log, OPERATOR_DELETE).len(), 3);

        // The shared list, through 0061d230.
        e.mem.set_u32(SHARED_RESPONSES, r[0]);
        e.call(0x0061_d230, &args![0u32]);
        assert_eq!(e.mem.u32(SHARED_RESPONSES), 0);
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_zero() {
        let mut e = setup();
        let r = e.mem.alloc(RESPONSE_SIZE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_cb30, &args![r, 0u32]).u32(), r);
        assert_eq!(e.call(0x0061_cb30, &args![r, 3u32]).u32(), r);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, RESPONSE_DESTRUCT).len(), 2);
        assert_eq!(calls(&log, OPERATOR_DELETE), [vec![r]]);
    }

    #[test]
    fn copy_builds_a_new_chain() {
        let mut e = setup();
        let head: Ptr = Ptr::new(e.mem.alloc(4));
        let old = chain(&mut e, 1);
        e.mem.set_u32(head.addr(), old[0]);
        let source = chain(&mut e, 3);
        e.call_log = Some(vec![]);
        e.call(0x0061_cb60, &args![head, source[0]]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, RESPONSE_DESTRUCT), [vec![old[0]]]);
        let copies = calls(&log, RESPONSE_COPY);
        assert_eq!(copies.len(), 3);
        for (copy, src) in copies.iter().zip(&source) {
            assert_eq!(copy[1], *src);
        }
        assert_eq!(e.mem.u32(head.addr()), copies[0][0]);
        assert_eq!(e.mem.u32(copies[0][0] + 0x28), copies[1][0]);
        assert_eq!(e.mem.u32(copies[1][0] + 0x28), copies[2][0]);
    }

    #[test]
    fn response_text_is_set_from_the_chunk() {
        let mut e = setup();
        let r = e.mem.alloc(RESPONSE_SIZE);
        e.call_log = Some(vec![]);
        e.call(0x0061_eb40, &args![r, 0x5555u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, STRING_SET), [vec![r + 0x18, 0x5555, 0]]);
    }

    #[test]
    fn display_text_joins_the_shared_lists_responses() {
        let mut e = setup();
        let info = e.mem.alloc(0x60);
        let r = chain(&mut e, 2);
        e.mem.set_u32(SHARED_RESPONSES_INFO, info);
        e.mem.set_u32(SHARED_RESPONSES, r[0]);
        let string = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0061_e2b0, &args![info, string, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, STRING_SET), [vec![string, 0, 0]]);
        assert_eq!(
            calls(&log, STRING_APPEND),
            [
                vec![string, r[0] + 0x18],
                vec![string, RESPONSE_SEPARATOR],
                vec![string, r[1] + 0x18],
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x0061_e2b0, &args![info, string, 0x101u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, STRING_APPEND),
            [vec![string, r[0] + 0x18], vec![string, RESPONSE_SEPARATOR]]
        );
    }

    #[test]
    fn response_list_of_the_shared_info_is_copied() {
        let mut e = setup();
        let info = e.mem.alloc(0x60);
        let r = chain(&mut e, 1);
        e.mem.set_u32(SHARED_RESPONSES_INFO, info);
        e.mem.set_u32(SHARED_RESPONSES, r[0]);
        assert_eq!(
            e.call(0x0061_e780, &args![info, 0u32]).u32(),
            SHARED_RESPONSES
        );
        let list = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_e780, &args![info, list]).u32(), list);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, RESPONSE_COPY)[0][1], r[0]);
        assert!(calls(&log, GET_FILE).is_empty());
    }

    /// A record of chunks: (tag, word) pairs read in order by the doubles.
    fn file_with_chunks(e: &mut Engine, chunks: &[(u32, u32)], form_type: u8) -> u32 {
        let file = e.mem.alloc(0x18 + 8 * chunks.len() as u32);
        e.mem.set_u32(file, 0);
        e.mem.set_u8(file + 4, form_type);
        for (i, (tag, word)) in chunks.iter().enumerate() {
            e.mem.set_u32(file + 0x10 + 8 * i as u32, *tag);
            e.mem.set_u32(file + 0x14 + 8 * i as u32, *word);
        }
        e.register(GET_TES_FORM, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register(GET_TES_CHUNK, |e, a| {
            let i = e.mem.u32(a[0]);
            ret(e.mem.u32(a[0] + 0x10 + 8 * i))
        });
        e.register(NEXT_CHUNK, |e, a| {
            let i = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], i);
            ret((e.mem.u32(a[0] + 0x10 + 8 * i) != 0) as u32)
        });
        e.register(CHUNK_SIZE, |e, a| {
            let i = e.mem.u32(a[0]);
            ret(e.mem.u32(a[0] + 0x14 + 8 * i))
        });
        e.register(GET_CHUNK_WORD, |e, a| {
            let file = a[0];
            let i = e.mem.u32(file);
            let word = e.mem.u32(file + 0x14 + 8 * i);
            e.mem.set_u32(a[1], word);
            ret(1)
        });
        file
    }

    #[test]
    fn response_list_reads_the_record() {
        let mut e = setup();
        // Idle form 0x77 has type 0x48; id 0x66 is another type; 0x55 is
        // missing.
        let idle = e.mem.alloc(0x10);
        e.mem.set_u8(idle + 4, 0x48);
        let other = e.mem.alloc(0x10);
        e.mem.set_u8(other + 4, 0x2a);
        e.register_double(LOOKUP_FORM, move |_, a| {
            ret(match a[0] {
                0x77 => idle,
                0x66 => other,
                _ => 0,
            })
        });
        let file = file_with_chunks(
            &mut e,
            &[
                (CHUNK_NAM1, 3), // before any response: ignored
                (CHUNK_TRDT, 0),
                (CHUNK_NAM1, 5),
                (CHUNK_SNAM, 0x77),
                (CHUNK_LNAM, 0x55),
                (CHUNK_TRDT, 0),
                (CHUNK_SNAM, 0x66),
                (CHUNK_LNAM, 0),
            ],
            0x46,
        );
        e.register_double(GET_FILE, move |_, _| ret(file));
        e.register(GET_THREAD_SAFE_FILE, |_, a| ret(a[0]));
        e.register(SET_OFFSET, |_, _| ret(1));
        let info = e.mem.alloc(0x60);
        e.mem.set_u32(info + 0xc, 0x0001_2345);
        e.mem.set_u32(info + INFO_FILE_OFFSET, 0x400);
        e.mem.set_u32(SHARED_RESPONSES_INFO, 0x9999);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0061_e780, &args![info, 0u32]).u32(),
            SHARED_RESPONSES
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(SHARED_RESPONSES_INFO), info);
        assert_eq!(calls(&log, SET_OFFSET), [vec![file, 0x400]]);
        let loads = calls(&log, RESPONSE_LOAD);
        assert_eq!(loads.len(), 2);
        let (first, second) = (loads[0][0], loads[1][0]);
        assert_eq!(loads[0][1], file);
        assert_eq!(
            calls(&log, RESPONSE_INIT_ITEM),
            [vec![first, info], vec![second, info]]
        );
        assert_eq!(e.mem.u32(SHARED_RESPONSES), first);
        assert_eq!(e.mem.u32(first + 0x28), second);
        let data = calls(&log, GET_CHUNK_DATA);
        assert_eq!(data.len(), 1);
        assert_eq!(data[0][2], 5);
        let sets = calls(&log, STRING_SET);
        assert_eq!(sets, [vec![first + 0x18, data[0][1], 0]]);
        assert_eq!(
            calls(&log, SET_SNAM_IDLE),
            [vec![first, idle], vec![second, 0]]
        );
        assert_eq!(
            calls(&log, SET_LNAM_IDLE),
            [vec![first, 0], vec![second, 0]]
        );
        assert_eq!(
            calls(&log, LOG_WARNING),
            [
                vec![FORMAT_NO_IDLE, 0x55, 0x0001_2345],
                vec![FORMAT_NO_IDLE, 0x66, 0x0001_2345],
            ]
        );
    }

    #[test]
    fn response_list_stops_without_a_readable_record() {
        let mut e = setup();
        let file = file_with_chunks(&mut e, &[(CHUNK_TRDT, 0)], 0x45);
        e.register_double(GET_FILE, move |_, _| ret(file));
        e.register(GET_THREAD_SAFE_FILE, |_, a| ret(a[0]));
        e.register(SET_OFFSET, |_, _| ret(1));
        let info = e.mem.alloc(0x60);
        let list = e.mem.alloc(4);
        // Not an info record.
        e.mem.set_u32(info + INFO_FILE_OFFSET, 0x400);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_e780, &args![info, list]).u32(), list);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, RESPONSE_LOAD).is_empty());
        // With a list of its own, the shared list is left alone.
        assert_eq!(e.mem.u32(SHARED_RESPONSES_INFO), 0);
        // No file offset: the file is not touched.
        e.mem.set_u32(info + INFO_FILE_OFFSET, 0);
        e.call_log = Some(vec![]);
        e.call(0x0061_e780, &args![info, list]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, GET_THREAD_SAFE_FILE).is_empty());
        // SetOffset fails.
        e.mem.set_u32(info + INFO_FILE_OFFSET, 0x400);
        e.register(SET_OFFSET, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0061_e780, &args![info, list]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, GET_TES_FORM).is_empty());
    }
}

#[cfg(test)]
mod b027_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn slot(table: u32, offset: u32) -> u32 {
        table + 0x10_0000 + offset
    }

    /// A vtable at `table` with 0x80 slots, each a double returning 0.
    fn vtable(e: &mut Engine, table: u32) {
        let slots: Vec<u32> = (0..0x80).map(|i| slot(table, 4 * i)).collect();
        for s in &slots {
            e.register(*s, |_, _| ret(0));
        }
        e.put_vtable(table, &slots);
    }

    /// An info (0x50 bytes) with the vtable at `0f00_0000`.
    fn new_info(e: &mut Engine) -> u32 {
        let info = e.mem.alloc(0x50);
        e.mem.set_u32(info, 0x0f00_0000);
        info
    }

    /// The doubles every test here shares: zero doubles for `zero`, the
    /// pointer globals mapped, the shared responses empty and the getters
    /// that return a field address or value doing so.
    fn engine(zero: &[u32]) -> Engine {
        let mut e = Engine::new();
        for addr in zero {
            e.register(*addr, |_, _| ret(0));
        }
        vtable(&mut e, 0x0f00_0000);
        for global in [
            SHARED_RESPONSES,
            SHARED_RESPONSES_INFO,
            RESULT_SCRIPTS_INFO,
            RESULT_VARIABLE,
            PLAYER,
            CURRENT_TOPIC,
        ] {
            e.map(global, 4);
        }
        e.map(RESULT_SCRIPTS, 2 * RESULT_SCRIPT_SIZE);
        e.register(WORD_AT_ZERO, |e, a| ret(e.mem.u32(a[0])));
        e.register(AT_0X18, |_, a| ret(a[0] + 0x18));
        e.register(ADD_TOPICS, |_, a| ret(a[0] + 0x30));
        e.register(AT_0X44, |_, a| ret(a[0] + 0x44));
        e.register(AT_0X4C, |_, a| ret(a[0] + 0x4c));
        e.register(CONVERSATION_DATA, |e, a| ret(e.mem.u32(a[0] + 0x38)));
        e.register(SPEAKER, |e, a| ret(e.mem.u32(a[0] + 0x3c)));
        e.register(INFO_INDEX, |e, a| ret(u32::from(e.mem.u16(a[0] + 0x20))));
        e
    }

    #[test]
    fn conversation_data_lifetime() {
        let mut e = engine(&[
            LIST_CONSTRUCT,
            LIST_DESTRUCT,
            CONVERSATION_0061C330,
            OPERATOR_DELETE,
        ]);
        let (result, log) = run(&mut e, 0x0061_ce40, &[0x4000]);
        assert_eq!(result.u32(), 0x4000);
        assert_eq!(
            log,
            vec![
                (LIST_CONSTRUCT, vec![0x4000]),
                (LIST_CONSTRUCT, vec![0x4008]),
                (LIST_CONSTRUCT, vec![0x4010]),
            ]
        );
        let (_, log) = run(&mut e, 0x0061_c190, &[0x4000]);
        assert_eq!(
            log,
            vec![
                (CONVERSATION_0061C330, vec![0x4000]),
                (LIST_DESTRUCT, vec![0x4010]),
                (LIST_DESTRUCT, vec![0x4008]),
                (LIST_DESTRUCT, vec![0x4000]),
            ]
        );
        let (result, log) = run(&mut e, 0x0061_d110, &[0x4000, 2]);
        assert_eq!(result.u32(), 0x4000);
        assert!(calls(&log, OPERATOR_DELETE).is_empty());
        let (_, log) = run(&mut e, 0x0061_d110, &[0x4000, 1]);
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![0x4000]]);
    }

    const CONSTRUCT_CALLEES: [u32; 12] = [
        TES_FORM_CONSTRUCT,
        CONDITIONS_CONSTRUCT,
        STRING_CONSTRUCT,
        LIST_CONSTRUCT,
        FILL_MEMORY,
        SET_FORM_TYPE,
        STRING_SET,
        INFO_SET_00409480,
        INFO_SET_0087CE80,
        SET_DIFFICULTY,
        INFO_0061CDC0,
        OPERATOR_NEW,
    ];

    #[test]
    fn constructor_sets_the_start_values() {
        let mut e = engine(&CONSTRUCT_CALLEES);
        let info = e.mem.alloc(0x50);
        for i in 0..0x14 {
            e.mem.set_u32(info + 4 * i, 0xffff_ffff);
        }
        let (result, log) = run(&mut e, 0x0061_ccd0, &[info, 0x102]);
        assert_eq!(result.u32(), info);
        assert_eq!(e.mem.u32(info), TOPIC_INFO_VTABLE);
        assert_eq!(e.mem.u8(info + 0x23), 2);
        assert_eq!(e.mem.u16(info + 0x20), 0xffff);
        assert_eq!(e.mem.u8(info + 0x22), 0);
        assert_eq!(e.mem.u32(info + 0x38), 0);
        assert_eq!(e.mem.u32(info + 0x48), 0);
        assert_eq!(e.mem.u32(info + 0x4c), 0);
        assert_eq!(calls(&log, CONDITIONS_CONSTRUCT), vec![vec![info + 0x18]]);
        assert_eq!(calls(&log, STRING_CONSTRUCT), vec![vec![info + 0x28]]);
        assert_eq!(calls(&log, LIST_CONSTRUCT), vec![vec![info + 0x30]]);
        assert_eq!(calls(&log, FILL_MEMORY), vec![vec![info + 0x23, 0, 4]]);
        assert_eq!(calls(&log, SET_FORM_TYPE), vec![vec![info, 0x46]]);
        assert_eq!(
            calls(&log, STRING_SET),
            vec![vec![info + 0x28, EMPTY_STRING, 0]]
        );
        assert_eq!(calls(&log, INFO_SET_0087CE80), vec![vec![info, 0]; 2]);
        assert_eq!(calls(&log, SET_DIFFICULTY), vec![vec![info, 1]]);
        assert!(calls(&log, OPERATOR_NEW).is_empty());
        // Kind 1 gets conversation data.
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        let (_, log) = run(&mut e, 0x0061_ccd0, &[info, 1]);
        let data = e.mem.u32(info + 0x38);
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![CONVERSATION_SIZE]]);
        assert_eq!(
            calls(&log, LIST_CONSTRUCT)[1..],
            [vec![data], vec![data + 8], vec![data + 0x10]]
        );
        assert_eq!(calls(&log, INFO_0061CDC0), vec![vec![info, 1]]);
        // A failed allocation leaves no data.
        e.register(OPERATOR_NEW, |_, _| ret(0));
        run(&mut e, 0x0061_ccd0, &[info, 1]);
        assert_eq!(e.mem.u32(info + 0x38), 0);
    }

    const CLEAR_CALLEES: [u32; 18] = [
        FORM_004077C0,
        GET_OWNER_TOPIC,
        CONDITIONS_CLEAR,
        SCRIPT_CONSTRUCT,
        SCRIPT_COPY,
        SCRIPT_DESTRUCT,
        NODE_IS_END,
        NODE_0063F7B0,
        TOPIC_00619C50,
        CONVERSATION_0061C330,
        LIST_DESTRUCT,
        OPERATOR_DELETE,
        STRING_DESTRUCT,
        CONDITIONS_DESTRUCT,
        TES_FORM_DESTRUCT,
        RESPONSE_NEXT,
        ARRAY_0061BDC0,
        ARRAY_00619FE0,
    ];

    #[test]
    fn clear_data_and_destructors() {
        let mut e = engine(&CLEAR_CALLEES);
        e.register(NODE_IS_END, |_, _| ret(1));
        let info = new_info(&mut e);
        e.mem.set_u16(info + 0x20, 4);
        e.register(GET_OWNER_TOPIC, |_, _| ret(0x5000));
        e.mem.set_u32(info + 0x38, 0x6000);
        e.mem.set_u32(SHARED_RESPONSES_INFO, 0x77);
        e.mem.set_u32(RESULT_SCRIPTS_INFO, 0x88);
        let (_, log) = run(&mut e, 0x0061_cff0, &[info]);
        assert_eq!(calls(&log, TOPIC_00619C50), vec![vec![0x5000, info]]);
        assert_eq!(calls(&log, CONDITIONS_CLEAR), vec![vec![info + 0x18]]);
        assert_eq!(calls(&log, NODE_IS_END), vec![vec![info + 0x30]]);
        let script = calls(&log, SCRIPT_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&log, SCRIPT_COPY),
            vec![vec![RESULT_SCRIPTS, script], vec![0x011c_bfb4, script]]
        );
        assert_eq!(calls(&log, SCRIPT_DESTRUCT), vec![vec![script]]);
        assert_eq!(e.mem.u32(SHARED_RESPONSES_INFO), 0);
        assert_eq!(e.mem.u32(RESULT_SCRIPTS_INFO), 0);
        assert_eq!(calls(&log, CONVERSATION_0061C330), vec![vec![0x6000]]);
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![0x6000]]);
        assert_eq!(e.mem.u32(info + 0x38), 0x6000);
        // Being deleted already (`004077c0`): only the globals and the data.
        e.register(FORM_004077C0, |_, _| ret(1));
        e.mem.set_u32(info + 0x38, 0);
        let (_, log) = run(&mut e, 0x0061_cff0, &[info]);
        assert_eq!(log, vec![(FORM_004077C0, vec![info])]);
        // The destructor body and the deleting destructor.
        let (_, log) = run(&mut e, 0x0061_ce10, &[info, 1]);
        assert_eq!(e.mem.u32(info), TOPIC_INFO_VTABLE);
        let tail: Log = log
            .into_iter()
            .filter(|(a, _)| *a != FORM_004077C0)
            .collect();
        assert_eq!(
            tail,
            vec![
                (LIST_DESTRUCT, vec![info + 0x30]),
                (STRING_DESTRUCT, vec![info + 0x28]),
                (CONDITIONS_DESTRUCT, vec![info + 0x18]),
                (TES_FORM_DESTRUCT, vec![info]),
                (OPERATOR_DELETE, vec![info]),
            ]
        );
        let (result, log) = run(&mut e, 0x0061_ce10, &[info, 0]);
        assert_eq!(result.u32(), info);
        assert!(calls(&log, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn leaving_the_topic_removes_the_info_from_the_array() {
        let mut e = engine(&[ARRAY_REMOVE, ARRAY_0061BDC0, ARRAY_00619FE0]);
        let info = new_info(&mut e);
        let elements = e.mem.alloc(0x10);
        e.mem.set_u32(elements + 4, 0x1234);
        e.mem.set_u32(elements + 8, info);
        e.register(TOPIC_00619C50, |_, _| ret(0x6000));
        e.register(ARRAY_OF_00717E50, |_, _| ret(0x7000));
        e.register(ARRAY_COUNT, |_, _| ret(4));
        e.register_double(ARRAY_ELEMENT, move |_, a| ret(elements + 4 * a[1]));
        // At its own index.
        e.mem.set_u16(info + 0x20, 2);
        let (_, log) = run(&mut e, 0x0061_d140, &[info, 0x5000]);
        assert_eq!(calls(&log, ARRAY_REMOVE), vec![vec![0x7000, 2]]);
        assert_eq!(calls(&log, ARRAY_0061BDC0), vec![vec![0x7000]]);
        assert_eq!(calls(&log, ARRAY_00619FE0), vec![vec![0x7000]]);
        // Elsewhere: found by the search.
        e.mem.set_u16(info + 0x20, 1);
        let (_, log) = run(&mut e, 0x0061_d140, &[info, 0x5000]);
        assert_eq!(calls(&log, ARRAY_REMOVE), vec![vec![0x7000, 2]]);
        assert_eq!(calls(&log, ARRAY_ELEMENT).len(), 4);
        // Not there at all.
        e.mem.set_u32(elements + 8, 0);
        let (_, log) = run(&mut e, 0x0061_d140, &[info, 0x5000]);
        assert!(calls(&log, ARRAY_REMOVE).is_empty());
        assert_eq!(calls(&log, ARRAY_00619FE0), vec![vec![0x7000]]);
        // No topic, no index or no holder: nothing.
        let (_, log) = run(&mut e, 0x0061_d140, &[info, 0]);
        assert!(log.is_empty());
        e.register(TOPIC_00619C50, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0061_d140, &[info, 0x5000]);
        assert_eq!(log.len(), 2);
        e.mem.set_u16(info + 0x20, 0xffff);
        let (_, log) = run(&mut e, 0x0061_d140, &[info, 0x5000]);
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn topic_list_is_emptied() {
        let mut e = engine(&[]);
        let left = Rc::new(Cell::new(2u32));
        let seen = left.clone();
        e.register_double(NODE_IS_END, move |_, _| ret(u32::from(seen.get() == 0)));
        e.register_double(NODE_0063F7B0, move |_, _| {
            left.set(left.get() - 1);
            ret(0)
        });
        let (_, log) = run(&mut e, 0x0061_d250, &[0x4000]);
        assert_eq!(calls(&log, NODE_0063F7B0), vec![vec![0x4030]; 2]);
        e.register(ADD_TOPICS, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0061_d250, &[0x4000]);
        assert_eq!(log, vec![(ADD_TOPICS, vec![0x4000])]);
    }

    /// Doubles for the resolution walks: each node is a block holding the
    /// address of its item word at +0 and its next node at +4.
    fn walk_doubles(e: &mut Engine) {
        // `006815c0` gives the node itself (its item is its first word);
        // the same address serves as the conversation data's 'Link From'.
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(NODE_SET_ITEM, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            ret(0)
        });
        // Taking out a node moves the next node's item into it (or empties
        // it at the end); `00905330` unlinks the node after the one given.
        e.register(NODE_0063F7B0, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let moved = e.mem.u32(next);
                e.mem.set_u32(a[0], moved);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0] + 4, after);
            }
            ret(0)
        });
        e.register(NODE_00905330, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            ret(0)
        });
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            ret(0)
        });
    }

    /// A list of nodes (item word, next node) holding `ids`; returns the
    /// first node and the nodes.
    fn node_list(e: &mut Engine, ids: &[u32]) -> (u32, Vec<u32>) {
        let mut next = 0;
        let mut nodes = vec![];
        for id in ids.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *id);
            e.mem.set_u32(node + 4, next);
            next = node;
            nodes.push(node);
        }
        nodes.reverse();
        (next, nodes)
    }

    #[test]
    fn conversation_links_are_resolved() {
        let mut e = engine(&[
            GET_FILE,
            NODE_00905330,
            LOG_WARNING,
            STRING_CONSTRUCT,
            STRING_DESTRUCT,
            STRING_SET,
            SPRINTF,
            GET_OWNER_TOPIC,
        ]);
        walk_doubles(&mut e);
        // Forms are their ids; ids ending in 5 are topics, in 6 infos.
        e.register(LOOKUP_FORM, |_, a| ret(a[0]));
        e.register(FORM_TYPE, |_, a| ret(0x40 + (a[0] & 0xf)));
        e.register(FORM_ID, |_, _| ret(0x00ab_cdef));
        // An empty 'Link From' list: the data's first word is 0.
        let data = e.mem.alloc(0x18);
        let (link_to, link_to_nodes) = node_list(&mut e, &[0x26, 0x15]);
        let (follow_up, follow_up_nodes) = node_list(&mut e, &[0x36]);
        e.register_double(LINK_TO, move |_, _| ret(link_to));
        e.register_double(FOLLOW_UP, move |_, _| ret(follow_up));
        let info = new_info(&mut e);
        e.mem.set_u32(SHARED_RESPONSES_INFO, info);
        let (_, log) = run(&mut e, 0x0061_c360, &[data, info]);
        // The info id first in the 'Link To' list is not a topic: taken out
        // (the next item moves up into its node) and warned about; the info
        // is not of type 0x46 (an address), so the description has no topic.
        assert_eq!(e.mem.u32(link_to_nodes[0]), 0x0100_0015);
        assert_eq!(e.mem.u32(link_to_nodes[0] + 4), 0);
        assert_eq!(calls(&log, GET_OWNER_TOPIC), vec![vec![0]]);
        let warnings = calls(&log, LOG_WARNING);
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0][..2], [FORMAT_NO_LINK_TO, 0x0100_0026]);
        assert_eq!(calls(&log, SPRINTF)[0][1..], [FORMAT_INFO, 0x00ab_cdef, 0]);
        assert_eq!(e.mem.u32(follow_up_nodes[0]), 0x0100_0036);
        assert!(calls(&log, NODE_00905330).is_empty());
    }

    #[test]
    fn info_init_item_resolves_its_links() {
        let zero = [
            FORM_004013E0,
            NODE_00905330,
            LOG_WARNING,
            STRING_CONSTRUCT,
            STRING_DESTRUCT,
            STRING_SET,
            SPRINTF,
            INFO_SET_00409480,
            CONDITION_INIT_ITEM,
            RESPONSE_INIT_ITEM,
            RESPONSE_NEXT,
            FORM_SET_484AB0,
            LINK_TO,
            LINK_FROM,
            FOLLOW_UP,
        ];
        let mut e = engine(&zero);
        walk_doubles(&mut e);
        e.register(GET_FILE, |_, _| ret(0x7100));
        e.register(LOOKUP_FORM, |_, a| ret(a[0] + 0x1000));
        // Topic ids end in 4; the cast to the second class of +0x40 works
        // for ids ending in 9.
        e.register(DYNAMIC_CAST, |_, a| {
            let wanted = match a[3] {
                TYPE_TES_TOPIC => (a[0] & 0xf) == 4,
                TYPE_011846E8 => true,
                TYPE_011861DC => false,
                _ => (a[0] & 0xf) == 9,
            };
            ret(if wanted { a[0] + 0x2000 } else { 0 })
        });
        e.register(GET_OWNER_TOPIC, |_, _| ret(0x5000));
        e.register(TOPIC_QUEST, |_, _| ret(0x5100));
        e.register(FORM_ID, |_, a| ret(a[0] + 1));
        let topic_vtable = 0x0f40_0000;
        vtable(&mut e, topic_vtable);
        e.map(0x5000, 4);
        e.map(0x5100, 4);
        e.mem.set_u32(0x5000, topic_vtable);
        e.mem.set_u32(0x5100, topic_vtable);
        e.register(slot(topic_vtable, 0x130), |_, _| ret(0x6130));
        e.register(slot(topic_vtable, 0x138), |_, _| ret(0x6138));
        let info = new_info(&mut e);
        e.mem.set_u32(SHARED_RESPONSES_INFO, info);
        let (first, nodes) = node_list(&mut e, &[0x14, 0x27]);
        e.register_double(ADD_TOPICS, move |_, _| ret(first));
        e.mem.set_u32(info + 0x3c, 0x33);
        e.mem.set_u32(info + 0x40, 0x49);
        let data = e.mem.alloc(0x18);
        e.mem.set_u32(info + 0x38, data);
        let (_, log) = run(&mut e, 0x0061_d290, &[info]);
        assert_eq!(e.mem.u32(nodes[0]), 0x0100_3014);
        // The second, missing and last after a found node, goes through
        // `00905330` on the found node.
        let moved = calls(&log, NODE_00905330);
        assert_eq!(moved.len(), 1);
        assert_eq!(moved[0][0], first);
        let warnings = calls(&log, LOG_WARNING);
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0][..2], [FORMAT_NO_TOPIC, 0x0100_0027]);
        assert_eq!(
            calls(&log, SPRINTF)[0][1..],
            [
                FORMAT_INFO_IN_QUEST,
                info + 1,
                0,
                0x6138,
                0x5101,
                0x6130,
                0x5001
            ]
        );
        assert_eq!(
            calls(&log, INFO_SET_00409480),
            vec![vec![info, 0x0100_3033]]
        );
        assert_eq!(
            calls(&log, CONDITION_INIT_ITEM),
            vec![vec![info + 0x18, info]]
        );
        assert_eq!(calls(&log, LINK_TO), vec![vec![data]]);
        assert_eq!(e.mem.u32(info + 0x40), 0x0100_3049);
        assert_eq!(calls(&log, FORM_SET_484AB0), vec![vec![info, 1]]);
        // Already initialised: nothing.
        e.register(FORM_004013E0, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0061_d290, &[info]);
        assert_eq!(log, vec![(FORM_004013E0, vec![info])]);
    }

    /// A `Load` engine over `chunks`; each chunk word read is 0x100 plus the
    /// chunk's position.
    fn load_engine(chunks: Vec<u32>) -> Engine {
        let mut e = engine(&[
            TES_FORM_LOAD_FORM,
            CONDITION_LOAD,
            ADD_COMPILE_INDEX,
            LIST_APPEND,
            STRING_SET,
            SET_DIFFICULTY,
            FORM_00440D80,
            LOG_WARNING,
            LIST_CONSTRUCT,
            INFO_0061CDC0,
        ]);
        e.register(GET_TES_FORM, |_, _| ret(0x46));
        e.register(GET_OPTIMIZED_FILE, |_, _| ret(0));
        e.register(RECORD_OFFSET, |_, _| ret(0x1234));
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(LINK_TO, |_, a| ret(a[0] + 8));
        e.register(FOLLOW_UP, |_, a| ret(a[0] + 0x10));
        e.register(LINK_FROM, |_, a| ret(a[0]));
        e.register(INSERT_INFO, |_, _| ret(1));
        let position = Rc::new(Cell::new(0usize));
        let (list, at) = (chunks.clone(), position.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(list.get(at.get()).copied().unwrap_or(0))
        });
        let total = chunks.len();
        let next = position.clone();
        e.register_double(NEXT_CHUNK, move |_, _| {
            next.set(next.get() + 1);
            ret(u32::from(next.get() < total))
        });
        e.register_double(GET_CHUNK_WORD, move |e, a| {
            e.mem.set_u32(a[1], 0x100 + position.get() as u32);
            ret(0)
        });
        e.register(GET_CHUNK_DATA, |e, a| {
            e.mem.set_u8(a[1], 9);
            ret(0)
        });
        e
    }

    #[test]
    fn load_reads_each_chunk() {
        let chunks = vec![
            CHUNK_CTDA, CHUNK_PNAM, CHUNK_DATA, CHUNK_NAME, CHUNK_QSTI, CHUNK_TCLT, CHUNK_TCLF,
            CHUNK_TCFU, CHUNK_RNAM, CHUNK_KNAM, CHUNK_ANAM, CHUNK_DNAM,
        ];
        let mut e = load_engine(chunks);
        e.mem.set_u32(CURRENT_TOPIC, 0x5000);
        let info = new_info(&mut e);
        let (result, log) = run(&mut e, 0x0061_dbd0, &[info, 0x7000]);
        assert!(result.bool());
        assert_eq!(e.mem.u32(info + 0x4c), 0x1234);
        assert_eq!(calls(&log, CONDITION_LOAD), vec![vec![info + 0x18, 0x7000]]);
        // `DATA` read 9 into the first data byte: slot 0xc4 with 1.
        assert_eq!(calls(&log, slot(0x0f00_0000, 0xc4)), vec![vec![info, 1]]);
        // The conversation data was made once, for `TCLT`.
        let data = e.mem.u32(info + 0x38);
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![CONVERSATION_SIZE]]);
        let appends = calls(&log, LIST_APPEND);
        assert_eq!(appends.len(), 4);
        assert_eq!(
            appends.iter().map(|a| a[0]).collect::<Vec<_>>(),
            vec![info + 0x30, data + 8, data, data + 0x10]
        );
        assert_eq!(calls(&log, STRING_SET)[0][0], info + 0x28);
        assert_eq!(e.mem.u32(info + 0x40), 0x109);
        assert_eq!(e.mem.u32(info + 0x3c), 0x10a);
        // `DNAM` read 0x10b, not below 6: the difficulty is not set, and 0
        // becomes 1.
        assert!(calls(&log, SET_DIFFICULTY).is_empty());
        assert_eq!(e.mem.u32(info + 0x44), 1);
        let compile = calls(&log, ADD_COMPILE_INDEX);
        assert_eq!(compile.len(), 2);
        assert_eq!(
            calls(&log, INSERT_INFO),
            vec![vec![0x5000, 0x104, info, 0x101]]
        );
    }

    #[test]
    fn load_refuses_other_records_and_warns_when_not_inserted() {
        let mut e = load_engine(vec![CHUNK_DNAM]);
        e.register(GET_TES_FORM, |_, _| ret(0x0146));
        let info = new_info(&mut e);
        let (result, log) = run(&mut e, 0x0061_dbd0, &[info, 0x7000]);
        assert!(result.bool());
        assert_eq!(calls(&log, TES_FORM_LOAD_FORM).len(), 1);
        e.register(GET_TES_FORM, |_, _| ret(0x45));
        let (result, log) = run(&mut e, 0x0061_dbd0, &[info, 0x7000]);
        assert!(!result.bool());
        assert_eq!(log.len(), 1);
        let mut e = load_engine(vec![CHUNK_DNAM, CHUNK_DATA]);
        let info = new_info(&mut e);
        e.register(GET_CHUNK_WORD, |e, a| {
            e.mem.set_u32(a[1], 3);
            ret(0)
        });
        e.register(GET_CHUNK_DATA, |e, a| {
            e.mem.set_u8(a[1], 3);
            ret(0)
        });
        e.register(INSERT_INFO, |_, _| ret(0x100));
        e.register(FORM_ID, |_, a| ret(a[0] + 1));
        e.register(GET_OPTIMIZED_FILE, |_, _| ret(1));
        let topic = info_with_name(&mut e);
        e.mem.set_u32(CURRENT_TOPIC, topic);
        let (_, log) = run(&mut e, 0x0061_dbd0, &[info, 0x7000]);
        assert_eq!(calls(&log, SET_DIFFICULTY), vec![vec![info, 3]]);
        // A first data byte of 3 becomes 0.
        assert_eq!(e.mem.u8(info + 0x23), 0);
        assert_eq!(e.mem.u32(info + 0x4c), 0);
        assert_eq!(
            calls(&log, LOG_WARNING),
            vec![vec![FORMAT_NOT_INSERTED, info + 1, 0x6130, topic + 1, 0]]
        );
    }

    /// An object whose slot 0x130 gives 0x6130.
    fn info_with_name(e: &mut Engine) -> u32 {
        vtable(e, 0x0f40_0000);
        e.register(slot(0x0f40_0000, 0x130), |_, _| ret(0x6130));
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0f40_0000);
        object
    }

    #[test]
    fn conversation_data_setup() {
        let mut e = engine(&[
            CONVERSATION_0061C210,
            LIST_CONSTRUCT,
            LIST_DESTRUCT,
            CONVERSATION_0061C330,
            OPERATOR_DELETE,
        ]);
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        let info = new_info(&mut e);
        let (_, log) = run(&mut e, 0x0061_e180, &[info, 5]);
        let data = e.mem.u32(info + 0x38);
        assert_ne!(data, 0);
        assert_eq!(
            calls(&log, CONVERSATION_0061C210),
            vec![vec![data, info, 5]]
        );
        let (_, log) = run(&mut e, 0x0061_e180, &[info, 6]);
        assert!(calls(&log, OPERATOR_NEW).is_empty());
        let (_, log) = run(&mut e, 0x0061_e180, &[info, 0]);
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![data]]);
        assert_eq!(calls(&log, OPERATOR_NEW).len(), 1);
        assert_ne!(e.mem.u32(info + 0x38), data);
    }

    #[test]
    fn kind_and_say_checks() {
        let mut e = engine(&[
            FORM_00440D80,
            INFO_00619B20,
            INFO_00619B40,
            CONDITIONS_OF_005F5760,
        ]);
        let info = new_info(&mut e);
        assert_eq!(e.call(0x0061_e720, &args![info]).u32(), 2);
        e.mem.set_u8(info + 0x26, 0x30);
        assert_eq!(e.call(0x0061_e720, &args![info]).u32(), 1);
        e.mem.set_u8(info + 0x26, 0x20);
        assert_eq!(e.call(0x0061_e720, &args![info]).u32(), 3);
        // Kind 3 with no actor: say it if the conditions hold.
        e.register(CONDITION_IS_TRUE, |_, a| ret(u32::from(a[0] != 0x6000)));
        let (result, log) = run(&mut e, 0x0061_e600, &[info, 1, 0, 3, 0]);
        assert_eq!(result.u32(), 1);
        assert_eq!(
            calls(&log, CONDITION_IS_TRUE),
            vec![vec![info + 0x18, 3, 0, 1, 0]]
        );
        // A second object's failing conditions stop it.
        e.register(CONDITIONS_OF_005F5760, |_, _| ret(0x6000));
        let (result, _) = run(&mut e, 0x0061_e600, &[info, 1, 0x5000, 3, 0]);
        assert_eq!(result.u32(), 0);
        // An actor whose value is at most the limit: kind 3 is refused.
        let actor = e.mem.alloc(0xa8);
        e.mem.set_u32(actor, 0x0f00_0000);
        e.register(slot(0x0f00_0000, 0x100), |_, _| ret(1));
        vtable(&mut e, 0x0f60_0000);
        e.mem.set_u32(actor + 0xa4, 0x0f60_0000);
        e.register(slot(0x0f60_0000, 8), |_, _| ret(0xffff_fffe));
        let limit = e.mem.alloc(4);
        e.register_double(VALUE_0043D4D0, move |_, _| ret(limit));
        let (result, log) = run(&mut e, 0x0061_e600, &[info, 1, 0, 3, actor]);
        assert_eq!(result.u32(), 0);
        assert_eq!(
            calls(&log, slot(0x0f60_0000, 8)),
            vec![vec![actor + 0xa4, 9]]
        );
        assert_eq!(calls(&log, VALUE_0043D4D0), vec![vec![VALUE_HOLDER]]);
        // Kind 1 is refused unless the value is at most the limit.
        e.mem.set_u8(info + 0x26, 0x10);
        let (result, _) = run(&mut e, 0x0061_e600, &[info, 1, 0, 3, actor]);
        assert_eq!(result.u32(), 1);
        e.mem.set_u32(limit, 0x8000_0000);
        let (result, _) = run(&mut e, 0x0061_e600, &[info, 1, 0, 3, actor]);
        assert_eq!(result.u32(), 0);
        // `00440d80`, or `00619b20` and `00619b40` both: never.
        e.register(INFO_00619B20, |_, _| ret(1));
        e.register(INFO_00619B40, |_, _| ret(1));
        assert_eq!(
            e.call(0x0061_e600, &args![info, 1u32, 0u32, 3u32, 0u32])
                .u32(),
            0
        );
        e.register(INFO_00619B40, |_, _| ret(0));
        e.register(FORM_00440D80, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0061_e600, &[info, 1, 0, 3, 0]);
        assert_eq!(log, vec![(FORM_00440D80, vec![info])]);
    }

    const RESULT_CALLEES: [u32; 18] = [
        SCRIPT_CONSTRUCT,
        SCRIPT_COPY,
        SET_TEMPORARY,
        SCRIPT_DESTRUCT,
        FORM_SET_484AB0,
        GET_CHUNK_DATA,
        HEADER_SWAP,
        FILL_MEMORY,
        SET_COMPILE_DATA,
        LIST_APPEND,
        VARIABLE_LOAD,
        STRING_SET,
        SCRIPT_INIT_ITEM,
        REFERENCED_OBJECT_CONSTRUCT,
        VARIABLE_CONSTRUCT,
        GET_THREAD_SAFE_FILE,
        SET_OFFSET,
        GET_TES_FORM,
    ];

    #[test]
    fn result_scripts_are_read_once_per_info() {
        let mut e = engine(&RESULT_CALLEES);
        let info = new_info(&mut e);
        e.mem.set_u32(RESULT_SCRIPTS_INFO, info);
        let (result, log) = run(&mut e, 0x0061_eb60, &[info, 1]);
        assert_eq!(result.u32(), 0x011c_bfb4);
        assert!(log.is_empty());
        // Another info: the scripts are emptied; `00508d70` false stops.
        e.mem.set_u32(RESULT_SCRIPTS_INFO, 0);
        e.register(INFO_00508D70, |_, _| ret(0));
        let (result, log) = run(&mut e, 0x0061_eb60, &[info, 0]);
        assert_eq!(result.u32(), RESULT_SCRIPTS);
        assert_eq!(e.mem.u32(RESULT_SCRIPTS_INFO), info);
        let empty = calls(&log, SCRIPT_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&log, SCRIPT_COPY),
            vec![vec![RESULT_SCRIPTS, empty], vec![0x011c_bfb4, empty]]
        );
        assert_eq!(calls(&log, SCRIPT_DESTRUCT), vec![vec![empty]]);
        assert!(calls(&log, FORM_SET_484AB0).is_empty());
    }

    #[test]
    fn result_scripts_are_loaded_from_the_record() {
        let mut e = engine(&RESULT_CALLEES);
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(REFERENCED_OBJECT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(VARIABLE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(INFO_00508D70, |_, _| ret(1));
        e.register(GET_FILE, |_, _| ret(0x7000));
        e.register(GET_THREAD_SAFE_FILE, |_, _| ret(0x7100));
        e.register(SET_OFFSET, |_, _| ret(1));
        e.register(GET_TES_FORM, |_, _| ret(0x46));
        e.register(CHUNK_SIZE, |_, _| ret(0x13));
        e.register(FILE_NEEDS_SWAP, |_, _| ret(1));
        e.register(GET_CHUNK_WORD, |e, a| {
            e.mem.set_u32(a[1], 0x99);
            ret(0)
        });
        let chunks = vec![
            CHUNK_SCHR, CHUNK_SCDA, CHUNK_SCRO, CHUNK_NEXT, CHUNK_SCRV, CHUNK_SLSD, CHUNK_SCVR,
        ];
        let position = Rc::new(Cell::new(0usize));
        let (list, at) = (chunks.clone(), position.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(list.get(at.get()).copied().unwrap_or(0))
        });
        let total = chunks.len();
        e.register_double(NEXT_CHUNK, move |_, _| {
            position.set(position.get() + 1);
            ret(u32::from(position.get() < total))
        });
        let info = new_info(&mut e);
        e.mem.set_u32(info + 0x4c, 0x2222);
        let (_, log) = run(&mut e, 0x0061_eb60, &[info, 0]);
        let second = RESULT_SCRIPTS + RESULT_SCRIPT_SIZE;
        assert_eq!(
            calls(&log, FORM_SET_484AB0),
            vec![vec![RESULT_SCRIPTS, 0], vec![second, 0]]
        );
        assert_eq!(calls(&log, SET_OFFSET), vec![vec![0x7100, 0x2222]]);
        assert_eq!(calls(&log, HEADER_SWAP), vec![vec![RESULT_SCRIPTS + 0x18]]);
        let data = calls(&log, GET_CHUNK_DATA);
        assert_eq!(data[0], vec![0x7100, RESULT_SCRIPTS + 0x18, 0]);
        assert_eq!(
            calls(&log, SET_COMPILE_DATA),
            vec![vec![RESULT_SCRIPTS, 0x13, data[1][1]]]
        );
        // `SCRO` before `NEXT` goes to the first script, the rest to the
        // second.
        let appends = calls(&log, LIST_APPEND);
        assert_eq!(appends[0][0], RESULT_SCRIPTS + 0x44);
        assert_eq!(appends[1][0], second + 0x44);
        assert_eq!(appends[2], vec![second + 0x4c, RESULT_VARIABLE]);
        let variable = e.mem.u32(RESULT_VARIABLE);
        assert_eq!(calls(&log, VARIABLE_LOAD), vec![vec![variable, 0x7100]]);
        assert_eq!(
            calls(&log, STRING_SET),
            vec![vec![variable + 0x18, data[2][1], 0]]
        );
        assert_eq!(
            calls(&log, SCRIPT_INIT_ITEM),
            vec![vec![RESULT_SCRIPTS, info], vec![second, info]]
        );
        let entries = calls(&log, REFERENCED_OBJECT_CONSTRUCT);
        assert_eq!(e.mem.u32(entries[0][0] + 8), 0x99);
        assert_eq!(e.mem.u32(entries[1][0] + 0xc), 0x99);
    }

    #[test]
    fn topic_list_goes_to_the_player_and_results_run() {
        let mut e = engine(&[
            PLAYER_009527B0,
            PROCESS_SCRIPTS,
            SCRIPT_LOCALS,
            SCRIPT_RUN,
            INFO_0061F220,
            INFO_0061AEF0,
            REFERENCE_00935920,
        ]);
        e.mem.set_u32(PLAYER, 0x9000);
        let (_, log) = run(&mut e, 0x0061_f150, &[0x4000]);
        assert_eq!(calls(&log, PLAYER_009527B0), vec![vec![0x9000, 0x4030]]);
        let info = new_info(&mut e);
        e.mem.set_u32(RESULT_SCRIPTS_INFO, info);
        let reference = e.mem.alloc(0x50);
        e.mem.set_u32(reference, 0x0f00_0000);
        let (_, log) = run(&mut e, 0x0061_f170, &[info, 1, 0]);
        assert!(log.is_empty());
        let (_, log) = run(&mut e, 0x0061_f170, &[info, 1, reference]);
        assert!(calls(&log, SCRIPT_RUN).is_empty());
        assert_eq!(calls(&log, INFO_0061F220), vec![vec![info]]);
        e.register(PROCESS_SCRIPTS, |_, _| ret(1));
        e.register(SCRIPT_LOCALS, |_, _| ret(0x6600));
        e.mem.set_u32(0x011c_bfb4 + 0x18 + 8, 1);
        e.register(INFO_0061AEF0, |_, _| ret(1));
        e.register(slot(0x0f00_0000, 0xfc), |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0061_f170, &[info, 1, reference]);
        assert_eq!(calls(&log, SCRIPT_LOCALS), vec![vec![reference + 0x44]]);
        assert_eq!(
            calls(&log, SCRIPT_RUN),
            vec![vec![0x011c_bfb4, reference, 0x6600, 0, 1]]
        );
        assert_eq!(calls(&log, REFERENCE_00935920), vec![vec![reference, info]]);
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

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object whose vtable maps `slot` to `target`.
    fn object(e: &mut Engine, slot: u32, target: u32) -> u32 {
        let o = e.mem.alloc(0x60);
        let vt = e.mem.alloc(0x140);
        e.mem.set_u32(o, vt);
        e.mem.set_u32(vt + slot, target);
        o
    }

    fn lists(e: &mut Engine) {
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_APPEND, |e, a| {
            let item = e.mem.u32(a[1]);
            let n = e.mem.u32(a[0] + 0x20);
            e.mem.set_u32(a[0] + 0x24 + 4 * n, item);
            e.mem.set_u32(a[0] + 0x20, n + 1);
            Ret::default()
        });
    }

    #[test]
    fn fn_0061c210_copies_the_three_topic_lists() {
        let mut e = Engine::new();
        lists(&mut e);
        e.register(CONVERSATION_0061C330, ok);
        // The data's lists (item, next): link-from at +0 (its accessor is the node item
        // accessor itself), link-to at +0x100, follow-up at +0x200 here.
        e.register(LINK_TO, |_, a| ret(a[0] + 0x100));
        e.register(FOLLOW_UP, |_, a| ret(a[0] + 0x200));
        let other = e.mem.alloc(0x300);
        let tail = e.mem.alloc(8);
        e.mem.set_u32(other + 0x100, 0x11);
        e.mem.set_u32(other + 0x104, tail);
        e.mem.set_u32(tail, 0x12);
        e.mem.set_u32(other, 0x21);
        e.mem.set_u32(other + 0x200, 0x31);
        let this = e.mem.alloc(0x300);
        e.call(0x0061_c210, &[this, 0x9999, 0]);
        assert_eq!(e.mem.u32(this + 0x120), 0);
        e.call_log = Some(vec![]);
        e.call(0x0061_c210, &[this, 0x9999, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, CONVERSATION_0061C330), vec![vec![this]]);
        // The append double counts at +0x20 of each list and stores from +0x24.
        assert_eq!(e.mem.u32(this + 0x120), 2);
        assert_eq!(
            (e.mem.u32(this + 0x124), e.mem.u32(this + 0x128)),
            (0x11, 0x12)
        );
        assert_eq!((e.mem.u32(this + 0x20), e.mem.u32(this + 0x24)), (1, 0x21));
        assert_eq!(e.mem.u32(this + 0x224), 0x31);
    }

    #[test]
    fn fn_0061cdc0_sets_and_clears_bit_2() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        e.mem.set_u8(this + 0x25, 0x81);
        e.call(0x0061_cdc0, &[this, 1]);
        assert_eq!(e.mem.u8(this + 0x25), 0x83);
        e.call(0x0061_cdc0, &[this, 0x100]);
        assert_eq!(e.mem.u8(this + 0x25), 0x81);
    }

    #[test]
    fn fn_0061d6a0_tells_the_owner_topic() {
        let mut e = Engine::new();
        e.register(FORM_004077C0, |_, _| ret(0));
        e.register(FORM_004013E0, |_, _| ret(1));
        e.register(FORM_SET_484730, ok);
        e.register(0x0050_0000, ok);
        let topic = object(&mut e, 0xc8, 0x0050_0000);
        e.register_double(GET_OWNER_TOPIC, move |_, _| ret(topic));
        e.call_log = Some(vec![]);
        e.call(0x0061_d6a0, &[0x5000, 1]);
        e.call(0x0061_d6a0, &[0x5000, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x0050_0000), vec![vec![topic, 1]]);
        assert_eq!(
            only(&log, FORM_SET_484730),
            vec![vec![0x5000, 1], vec![0x5000, 0]]
        );
    }

    #[test]
    fn fn_0061d710_compares_owner_topics() {
        let mut e = Engine::new();
        e.register(FORM_TYPE, |e, a| ret(u32::from(e.mem.u8(a[0] + 4))));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(0x0050_0000, |_, a| ret(u32::from(a[1] != 0)));
        let mine = object(&mut e, 0x3c, 0x0050_0000);
        let theirs = object(&mut e, 0x3c, 0x0050_0000);
        let info = e.mem.alloc(0x20);
        e.mem.set_u8(info + 4, 0x46);
        e.register_double(GET_OWNER_TOPIC, move |_, a| {
            ret(if a[0] == info { theirs } else { mine })
        });
        e.mem.set_u8(theirs + 4, 0x45);
        e.mem.set_u8(mine + 4, 0x45);
        assert!(e.call(0x0061_d710, &[0x5000, theirs]).bool());
        assert!(!e.call(0x0061_d710, &[0x5000, mine]).bool());
        assert!(e.call(0x0061_d710, &[0x5000, info]).bool());
        e.register(FORM_ANSWER_484020, |_, _| ret(1));
        let other = e.mem.alloc(0x20);
        e.mem.set_u8(other + 4, 0x10);
        assert!(e.call(0x0061_d710, &[0x5000, other]).bool());
    }

    #[test]
    fn fn_0061d820_and_fn_0061d8c0_handle_topic_conditions() {
        let mut e = Engine::new();
        e.map(0x0118_7000, 0x1000);
        e.set_global::<u32>(CONDITION_VTABLE_WORD, 0x0300);
        e.set_global::<u32>(CONDITION_FUNCTION_WORD, 0x0400);
        e.register(0x0050_0000, |_, a| ret(u32::from(a[3] == 9)));
        let topic = object(&mut e, 0x110, 0x0050_0000);
        e.register_double(GET_OWNER_TOPIC, move |_, _| ret(topic));
        e.register(FORM_TEST_485BE0, |_, a| ret(u32::from(a[1] == 0x77)));
        let condition = e.mem.alloc(0x14);
        assert!(!e.call(0x0061_d820, &[0x5000, condition, 1, 9]).bool());
        e.mem.set_u32(condition, 0x0300);
        assert!(e.call(0x0061_d820, &[0x5000, condition, 1, 9]).bool());
        assert!(!e.call(0x0061_d820, &[0x5000, condition, 1, 2]).bool());
        e.mem.set_u32(condition + 0xc, 7);
        e.mem.set_u32(condition + 8, 0x77);
        assert!(e.call(0x0061_d820, &[0x5000, condition, 0, 0]).bool());
        // 0061d8c0 fills a kind-7 condition with the topic's id.
        e.register(FORM_ID, |_, _| ret(0x1234));
        let out = e.mem.alloc(0x14);
        e.mem.set_u32(out, 0xffff);
        let source = e.mem.alloc(0x14);
        e.mem.set_u32(source + 8, 0x0401);
        e.call(0x0061_d8c0, &[0x5000, out, source]);
        assert_eq!(e.mem.u32(out), 0);
        e.mem.set_u32(source + 8, 0x0400);
        e.call(0x0061_d8c0, &[0x5000, out, source]);
        assert_eq!(e.mem.u32(out), 0x0300);
        assert_eq!((e.mem.u32(out + 8), e.mem.u32(out + 0xc)), (0x1234, 7));
        e.call(0x0061_d8c0, &[0x5000, 0, source]);
    }

    #[test]
    fn get_form_detailed_string_prints_its_parts() {
        let mut e = Engine::new();
        e.map(0x011c_b000, 0x2000);
        e.register(0x0050_0000, |_, _| ret(0x7100));
        e.register(0x0050_0010, |_, _| ret(0x7200));
        let topic = object(&mut e, 0x130, 0x0050_0000);
        let quest = object(&mut e, 0x138, 0x0050_0010);
        e.register_double(GET_OWNER_TOPIC, move |_, _| ret(topic));
        e.register(FORM_004013E0, |_, _| ret(1));
        e.register_double(TOPIC_QUEST, move |_, _| ret(quest));
        e.register(FORM_ID, |_, a| ret(a[0] & 0xff));
        e.register(SPRINTF, ok);
        e.register(STRING_PRINTF, ok);
        e.register(FORM_TEXT_AT_401280, |_, _| ret(0x401));
        e.register(FORM_TEXT_AT_440E30, |_, _| ret(0x440));
        let info = e.mem.alloc(0x60);
        e.mem.set_u32(SHARED_RESPONSES_INFO, info);
        e.mem.set_u32(SHARED_RESPONSES, 0);
        e.register(WORD_AT_ZERO, |e, a| ret(e.mem.u32(a[0])));
        e.call_log = Some(vec![]);
        e.call(0x0061_d950, &[info, 0x9000]);
        let log = e.call_log.take().unwrap();
        let s = only(&log, SPRINTF);
        assert_eq!(s.len(), 2);
        assert_eq!(&s[0][1..], &[FORMAT_TOPIC, 0x7100, topic & 0xff]);
        assert_eq!(&s[1][1..], &[FORMAT_QUEST, 0x7200, quest & 0xff]);
        let p = only(&log, STRING_PRINTF);
        assert_eq!(
            &p[0][..5],
            &[0x9000, FORMAT_DETAILED, 0x440, 0x401, info & 0xff]
        );
        assert_eq!(p[0][6], s[0][0]);
        assert_eq!(p[0][5], s[1][0]);
    }

    #[test]
    fn save_and_load_overrides_keep_flag_0x22() {
        let mut e = Engine::new();
        e.register(FORM_LOAD_GAME, ok);
        e.register(FORM_4534F0, ok);
        e.register(FORM_484DA0, ok);
        e.register(BUFFER_FLAGS_LOAD, |_, _| ret(0x8000_0000));
        e.register(BUFFER_FLAGS_SAVE, |_, _| ret(0x8000_0000));
        e.register(FLAGS_TEST, |_, a| ret(u32::from(a[0] & a[1] != 0)));
        let this = e.mem.alloc(0x30);
        e.call(0x0061_dad0, &[this, 1, 2]);
        assert_eq!(e.mem.u8(this + 0x22), 0);
        e.call(0x0061_dad0, &[this, 0x8000_0000, 2]);
        assert_eq!(e.mem.u8(this + 0x22), 1);
        e.call(0x0061_db00, &[this, 0x8000_0000]);
        assert_eq!(e.mem.u8(this + 0x22), 0);
        e.call(0x0061_db30, &[this, 0x7000]);
        assert_eq!(e.mem.u8(this + 0x22), 1);
        e.call(0x0061_db80, &[this, 0x7000]);
        assert_eq!(e.mem.u8(this + 0x22), 0);
        e.register(BUFFER_FLAGS_LOAD, |_, _| ret(0));
        e.call(0x0061_db30, &[this, 0x7000]);
        assert_eq!(e.mem.u8(this + 0x22), 0);
    }

    #[test]
    fn fn_0061e0f0_copies_given_or_empty_conditions() {
        let mut e = Engine::new();
        e.register(CONDITIONS_COPY, ok);
        e.register(CONDITIONS_CONSTRUCT, ok);
        e.register(CONDITIONS_DESTRUCT, ok);
        e.call_log = Some(vec![]);
        e.call(0x0061_e0f0, &[0x5000, 0x6000]);
        e.call(0x0061_e0f0, &[0x5000, 0]);
        let log = e.call_log.take().unwrap();
        let copies = only(&log, CONDITIONS_COPY);
        assert_eq!(copies[0], vec![0x5018, 0x6000, 0x5000]);
        let empty = only(&log, CONDITIONS_CONSTRUCT)[0][0];
        assert_eq!(copies[1], vec![0x5018, empty, 0x5000]);
        assert_eq!(only(&log, CONDITIONS_DESTRUCT), vec![vec![empty]]);
        assert_eq!(e.call(0x0061_e5e0, &[0x5000]).u32(), 0x5023);
    }

    #[test]
    fn fn_0061e330_copies_an_info() {
        let mut e = Engine::new();
        e.map(0x011c_b000, 0x2000);
        lists(&mut e);
        let this = e.mem.alloc(0x60);
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        for a in [
            COPY_ALL_COMPONENTS,
            NODE_0063F7B0,
            STRING_SET,
            INFO_SET_0087CE80,
            SET_DIFFICULTY,
            INFO_SET_00409480,
            SCRIPT_COPY,
            SET_INFO_INDEX,
            CONDITIONS_COPY,
            CONVERSATION_0061C210,
            SET_SECOND_GENERIC_LOCATION,
            FORM_SET_484AB0,
        ] {
            e.register(a, ok);
        }
        e.register(ADD_TOPICS, |_, a| ret(a[0] + 0x30));
        e.register(NODE_IS_END, |_, _| ret(1));
        e.register(WORD_AT_ZERO, |e, a| ret(e.mem.u32(a[0])));
        e.register(INFO_TEXT_HOLDER, |_, a| ret(a[0] + 0x50));
        e.register(GET_GENERIC_LOCATION, |_, _| ret(0x31));
        e.register(GET_SECOND_GENERIC_LOCATION, |_, _| ret(0x32));
        e.register(SPEAKER, |_, _| ret(0x33));
        e.register(INFO_INDEX, |_, _| ret(0xffff_0007));
        e.register(AT_0X18, |_, _| ret(0x6000));
        e.register(CONVERSATION_DATA, |_, _| ret(0x6100));
        e.register(FORM_004013E0, |_, _| ret(0x101));
        e.register(FORM_TYPE, |e, a| ret(u32::from(e.mem.u8(a[0] + 4))));
        e.mem.set_u32(SHARED_RESPONSES_INFO, this);
        e.mem.set_u32(SHARED_RESPONSES, 0);
        e.mem.set_u32(RESULT_SCRIPTS_INFO, this);
        e.mem.set_u32(this + 0x38, 0x6200);
        e.mem.set_u32(this + 0x50, 0x6300);
        e.mem.set_u32(this + 0x23, 0x0102_0304);
        let other = e.mem.alloc(0x10);
        e.mem.set_u8(other + 4, 0x56);
        e.mem.set_u32(this + 0x40, other);
        e.call(0x0061_e330, &[this, 0]);
        e.call_log = Some(vec![]);
        e.call(0x0061_e330, &[this, this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, COPY_ALL_COMPONENTS), vec![vec![this, this]]);
        assert_eq!(e.mem.u32(this + 0x23), 0x0102_0304);
        assert_eq!(only(&log, STRING_SET), vec![vec![this + 0x28, 0x6300, 0]]);
        assert_eq!(only(&log, INFO_SET_0087CE80), vec![vec![this, other]]);
        assert_eq!(only(&log, SET_DIFFICULTY), vec![vec![this, 0x31]]);
        assert_eq!(only(&log, INFO_SET_00409480), vec![vec![this, 0x33]]);
        assert_eq!(
            only(&log, SCRIPT_COPY),
            vec![
                vec![RESULT_SCRIPTS, RESULT_SCRIPTS],
                vec![
                    RESULT_SCRIPTS + RESULT_SCRIPT_SIZE,
                    RESULT_SCRIPTS + RESULT_SCRIPT_SIZE
                ]
            ]
        );
        assert_eq!(only(&log, SET_INFO_INDEX), vec![vec![this, 7]]);
        assert_eq!(
            only(&log, CONDITIONS_COPY),
            vec![vec![this + 0x18, 0x6000, this]]
        );
        assert_eq!(
            only(&log, CONVERSATION_0061C210),
            vec![vec![0x6200, this, 0x6100]]
        );
        assert_eq!(
            only(&log, SET_SECOND_GENERIC_LOCATION),
            vec![vec![this, 0x32]]
        );
        assert_eq!(only(&log, FORM_SET_484AB0), vec![vec![this, 1]]);
        // A form of another type at +0x40 is copied as it is.
        e.mem.set_u8(other + 4, 0x10);
        e.mem.set_u32(this + 0x40, other);
        e.call(0x0061_e330, &[this, this]);
        assert_eq!(e.mem.u32(this + 0x40), other);
    }
}
