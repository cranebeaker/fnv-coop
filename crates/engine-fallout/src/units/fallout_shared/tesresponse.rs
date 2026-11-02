//! `fallout shared/tesresponse.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! One dialogue response (`TESResponse`): its text, the copy of another
//! response, `Load` (the `TRDT` chunk) with the byte swap of its words, and
//! `InitItem` (resolving the topic, sound and idle form ids, with the
//! master-file warnings when they are missing).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `TESResponse` (Xbox PDB), 0x2c bytes (the PC code allocates the
    /// same and uses the same offsets). Its first 0x18 bytes are
    /// `RESPONSE_DATA Data`, whose fields are declared here.
    pub struct TesResponse: 0x2c {
        /// `Data.eEmotion` (Xbox PDB): `DIALOGUE_EMOTION`.
        0x00 eEmotion: u32,
        /// `Data.iEmotionValue` (Xbox PDB).
        0x04 iEmotionValue: u32,
        /// `Data.pConversationTopic` (Xbox PDB): `TESTopic*` (the form id
        /// until `InitItem`).
        0x08 pConversationTopic: u32,
        /// `Data.ucResponseID` (Xbox PDB).
        0x0c ucResponseID: u8,
        /// `Data.pVoiceSound` (Xbox PDB): `TESSound*` (the form id until
        /// `InitItem`).
        0x10 pVoiceSound: u32,
        /// `Data.bUseEmotion` (Xbox PDB).
        0x14 bUseEmotion: u8,
        /// `cResponseText` (Xbox PDB): `BSStringT<char>`, 8 bytes.
        0x18 cResponseText: u32,
        /// `pSpeakerIdle` (Xbox PDB): `TESIdleForm*` (the form id until
        /// `InitItem`).
        0x20 pSpeakerIdle: u32,
        /// `pListenerIdle` (Xbox PDB): `TESIdleForm*` (the form id until
        /// `InitItem`).
        0x24 pListenerIdle: u32,
        /// `pNext` (Xbox PDB): `TESResponse*`.
        0x28 pNext: u32,
    }
}

/// The size of `RESPONSE_DATA` (Xbox PDB), the bytes the copy moves.
const RESPONSE_DATA_SIZE: u32 = 0x18;
/// The empty string (`01011584`).
const EMPTY_STRING: u32 = 0x0101_1584;
/// `00559450` (`ECX` = object): the word at +0 (a string's characters).
const WORD_AT_ZERO: u32 = 0x0055_9450;
/// `006815c0` (`ECX` = object): returns `ECX`.
const SELF: u32 = 0x0068_15c0;
/// The game's `memcpy` wrapper (`00401460`, cdecl: destination, source,
/// count).
const MEMCPY: u32 = 0x0040_1460;
/// `004037f0` (`ECX` = string; characters, a word; `RET 8`): sets the
/// string.
const STRING_SET: u32 = 0x0040_37f0;
/// `004037b0` (`ECX` = string): the string object's constructor.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// `004037d0` (`ECX` = string): the string object's destructor.
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `007af430` (`ECX` = object): the word at +0x20.
const WORD_AT_0X20: u32 = 0x007a_f430;
/// `0050f9c0` (`ECX` = object, one word): stores it at +0x20.
const SET_WORD_AT_0X20: u32 = 0x0050_f9c0;
/// `0059bb30` (`ECX` = object): the word at +0x24.
const WORD_AT_0X24: u32 = 0x0059_bb30;
/// `007037c0` (`ECX` = object, one word): stores it at +0x24.
const SET_WORD_AT_0X24: u32 = 0x0070_37c0;
/// `TESFile::GetTESChunk` (`004726b0`, `ECX` = file): the chunk's tag.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `TESFile::GetChunkData_ov3` (`00472890`, `ECX` = file; buffer, size;
/// `RET 8`).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `00401680` (`ECX` = file): whether the file's words need swapping.
const FILE_NEEDS_SWAP: u32 = 0x0040_1680;
/// `00401080` (cdecl: the word's address, a byte): swaps a word's bytes.
const BYTE_SWAP_WORD: u32 = 0x0040_1080;
/// The `TRDT` chunk tag.
const CHUNK_TRDT: u32 = 0x5444_5254;
/// `TESForm::GetFile` (`00484e60`, `ECX` = form, one word: the index).
const GET_FILE: u32 = 0x0048_4e60;
/// The `GetFile` index word the exe passes (-1).
const ANY_FILE: u32 = 0xffff_ffff;
/// `TESForm::AddCompileIndex` (`00485d50`, cdecl: the form id's address,
/// the file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (cdecl: a form id): the form with that id, or 0.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `00401170` (`ECX` = form): the form type byte at +4, zero-extended.
const FORM_TYPE: u32 = 0x0040_1170;
/// The form type the topic must have (0x45).
const TOPIC_TYPE: u32 = 0x45;
/// The form type the sound must have (0xd).
const SOUND_TYPE: u32 = 0x0d;
/// The form type the idles must have (0x48).
const IDLE_TYPE: u32 = 0x48;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `TESForm` type descriptor (`01183028`).
const TES_FORM_TYPE: u32 = 0x0118_3028;
/// The `TESTopicInfo` type descriptor (`01184738`).
const TES_TOPIC_INFO_TYPE: u32 = 0x0118_4738;
/// `0061a310` (cdecl: an info; `TESTopic::GetOwnerTopic` in the engine
/// map): the form the warning names as the topic.
const OWNER_TOPIC: u32 = 0x0061_a310;
/// `00619d00` (`ECX` = the topic, one word: the info; `RET 4`): the form
/// the warning names as the quest.
const OWNER_QUEST: u32 = 0x0061_9d00;
/// `TESTopicInfo::GetInfoDisplayText` (`0061e2b0`, `ECX` = info; the
/// string, a byte; `RET 8`).
const GET_INFO_DISPLAY_TEXT: u32 = 0x0061_e2b0;
/// `0084e3a0` (`ECX` = form): the word at +0xc (the form id).
const FORM_ID: u32 = 0x0084_e3a0;
/// The virtual slot the warning takes the topic's name from.
const TOPIC_NAME_SLOT: u32 = 0x130;
/// The virtual slot the warning takes the quest's name from.
const QUEST_NAME_SLOT: u32 = 0x138;
/// `sprintf` (`00ec623a`, cdecl).
const SPRINTF: u32 = 0x00ec_623a;
/// `005b5e40` (cdecl: format, words): the warning output.
const WARNING: u32 = 0x005b_5e40;
/// "TopicInfo %08X \"%s\" in Quest %s (%08x) Topic %s (%08x)" (`0104cae0`).
const FORMAT_INFO: u32 = 0x0104_cae0;
/// "MASTERFILE: Unable to find TESTopic (%08X) in TESResponse init for
/// %s." (`0104ca98`).
const FORMAT_NO_TOPIC: u32 = 0x0104_ca98;
/// "MASTERFILE: Unable to find TESSound (%08X) in TESResponse init for
/// %s." (`0104ca50`).
const FORMAT_NO_SOUND: u32 = 0x0104_ca50;
/// "MASTERFILE: Could not find speaker idle (%08X) in TESResponse init for
/// %s." (`0104ca00`).
const FORMAT_NO_SPEAKER_IDLE: u32 = 0x0104_ca00;
/// "MASTERFILE: Could not find listener idle (%08X) in TESResponse init
/// for %s." (`0104c9b0`).
const FORMAT_NO_LISTENER_IDLE: u32 = 0x0104_c9b0;
/// The size of the stack buffers the descriptions go in.
const DESCRIPTION_SIZE: u32 = 0x10c;
/// The size of the stack string object (`BSStringT<char>`).
const STRING_SIZE: u32 = 8;
/// `TESResponse::iStandardEmotionValue` (`011cbdf4`, a game setting; its
/// initializer `00f54410` names it "iStandardEmotionValue").
const STANDARD_EMOTION_VALUE: u32 = 0x011c_bdf4;
/// `0043d4d0` (`ECX` = setting): the address of its value.
const SETTING_VALUE: u32 = 0x0043_d4d0;
/// `006ecd40` (`ECX` = object, one word): stores it at +4.
const SET_WORD_AT_4: u32 = 0x006e_cd40;

// Translated from 00616c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the bytes of the response's (`ECX`) data words at +0, +4, +8 and
/// +0x10 (`00401080`, each with a zero byte). The engine map gives it no
/// name.
pub fn fn_00616c70(e: &mut Engine, this: Ptr<TesResponse>) {
    for offset in [0x00, 0x04, 0x08, 0x10] {
        e.call(BYTE_SWAP_WORD, &args![this.addr() + offset, 0u32]);
    }
}

// Translated from 00616de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESResponse::GetResponseText` (Xbox PDB; `ECX` = response): the
/// characters of the response text (+0x18, `00559450`), or the empty
/// string (`01011584`) when they are 0.
pub fn tes_response_get_response_text(e: &mut Engine, this: Ptr<TesResponse>) -> u32 {
    let text = this.addr() + 0x18;
    if e.call(WORD_AT_ZERO, &args![text]).u32() == 0 {
        EMPTY_STRING
    } else {
        e.call(WORD_AT_ZERO, &args![text]).u32()
    }
}

// Translated from 006174d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the response (`ECX`) a copy of the one given (one word; `RET 4`):
/// the 0x18 data bytes (`memcpy` from `006815c0` of the source), the text
/// (`004037f0` with the source's `GetResponseText` and 0), and the words
/// at +0x20 and +0x24 (the idles; `007af430`/`0050f9c0` and
/// `0059bb30`/`007037c0`). The engine map gives it no name.
pub fn fn_006174d0(e: &mut Engine, this: Ptr<TesResponse>, source: Ptr<TesResponse>) {
    let data = e.call(SELF, &args![source]).u32();
    e.call(MEMCPY, &args![this, data, RESPONSE_DATA_SIZE]);
    let text = tes_response_get_response_text(e, source);
    e.call(STRING_SET, &args![this.addr() + 0x18, text, 0u32]);
    let speaker = e.call(WORD_AT_0X20, &args![source]).u32();
    e.call(SET_WORD_AT_0X20, &args![this, speaker]);
    let listener = e.call(WORD_AT_0X24, &args![source]).u32();
    e.call(SET_WORD_AT_0X24, &args![this, listener]);
}

// Translated from 00617530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESResponse::Load` (Xbox PDB; `ECX` = response, one word: the file;
/// `RET 4`): with a file whose current chunk is `TRDT`, reads 0x18 bytes
/// into the response's data (`GetChunkData_ov3`) and swaps its words
/// (`00616c70`) when the file says so (`00401680`).
pub fn tes_response_load(e: &mut Engine, this: Ptr<TesResponse>, file: Ptr) {
    if file.is_null() {
        return;
    }
    if e.call(GET_TES_CHUNK, &args![file]).u32() != CHUNK_TRDT {
        return;
    }
    e.call(GET_CHUNK_DATA, &args![file, this, RESPONSE_DATA_SIZE]);
    if e.call(FILE_NEEDS_SWAP, &args![file]).u8() != 0 {
        fn_00616c70(e, this);
    }
}

/// `InitItem`'s resolution of one form id field (+`offset`): the id gets
/// the owner's file's compile index (`GetFile(-1)`, `AddCompileIndex` on a
/// stack copy), and the field becomes the form with that id when it has
/// `form_type`, else 0. Returns the id after `AddCompileIndex` (what the
/// warning prints) and whether the field is now 0.
fn resolve_field(
    e: &mut Engine,
    this: Ptr<TesResponse>,
    offset: u32,
    owner: Ptr,
    form_type: u32,
) -> (u32, bool) {
    let field = this.addr() + offset;
    let id = e.mem.u32(field);
    let id = e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), id);
        let file = e.call(GET_FILE, &args![owner, ANY_FILE]).u32();
        e.call(ADD_COMPILE_INDEX, &args![local, file]);
        e.mem.u32(local.addr())
    });
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    if form != 0 && e.call(FORM_TYPE, &args![form]).u32() == form_type {
        e.mem.set_u32(field, form);
    } else {
        e.mem.set_u32(field, 0);
    }
    (id, e.mem.u32(field) == 0)
}

/// `InitItem`'s warning for a missing form: the owner cast to
/// `TESTopicInfo` (`__RTDynamicCast`), its topic (`0061a310`) and, with a
/// topic, quest (`00619d00`); its display text (first response only) in a
/// stack string; with `describe`, the description (`sprintf` of the info's
/// id, the text, the quest's name (slot 0x138) and id, the topic's name
/// (slot 0x130) and id; the empty string and 0 for a missing one) goes to
/// the warning with the id; without, the warning gets a stack buffer
/// nothing wrote (the exe's listener-idle warning passes an unwritten
/// buffer). The string is destroyed after.
fn warn_missing(e: &mut Engine, owner: Ptr, format: u32, id: u32, describe: bool) {
    let info = e
        .call(
            DYNAMIC_CAST,
            &args![owner, 0u32, TES_FORM_TYPE, TES_TOPIC_INFO_TYPE, 0u32],
        )
        .u32();
    let topic = e.call(OWNER_TOPIC, &args![info]).u32();
    let quest = if topic != 0 {
        e.call(OWNER_QUEST, &args![topic, info]).u32()
    } else {
        0
    };
    e.with_stack(STRING_SIZE, |e, string| {
        e.call(STRING_CONSTRUCT, &args![string]);
        e.call(GET_INFO_DISPLAY_TEXT, &args![info, string, 1u32]);
        e.with_stack(DESCRIPTION_SIZE, |e, buffer| {
            if describe {
                let topic_id = if topic != 0 {
                    e.call(FORM_ID, &args![topic]).u32()
                } else {
                    0
                };
                let topic_name = if topic != 0 {
                    e.vcall(topic, TOPIC_NAME_SLOT, &args![]).u32()
                } else {
                    EMPTY_STRING
                };
                let quest_id = if quest != 0 {
                    e.call(FORM_ID, &args![quest]).u32()
                } else {
                    0
                };
                let quest_name = if quest != 0 {
                    e.vcall(quest, QUEST_NAME_SLOT, &args![]).u32()
                } else {
                    EMPTY_STRING
                };
                let text = e.call(WORD_AT_ZERO, &args![string]).u32();
                let info_id = e.call(FORM_ID, &args![info]).u32();
                e.call(
                    SPRINTF,
                    &args![
                        buffer,
                        FORMAT_INFO,
                        info_id,
                        text,
                        quest_name,
                        quest_id,
                        topic_name,
                        topic_id
                    ],
                );
            }
            e.call(WARNING, &args![format, id, buffer]);
        });
        e.call(STRING_DESTRUCT, &args![string]);
    });
}

// Translated from 00617580 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00617580 - the exception-unwinding frame (FS:[0]) that destroys the stack strings and the security-cookie check (00ec408c) are not translated.
/// `TESResponse::InitItem` (Xbox PDB; `ECX` = response, one word: the
/// owning form, the info; `RET 4`). Each nonzero form id field is resolved
/// with the owner's file's compile index: the topic (+8, form type 0x45),
/// the sound (+0x10, 0xd), the speaker idle (+0x20, 0x48) and the listener
/// idle (+0x24, 0x48); a field left 0 warns ("MASTERFILE: ...", with the
/// id) with a description of the info, its quest and topic (none for the
/// listener idle, which passes an unwritten buffer). Then, with a nonzero
/// emotion (+0, `00559450`), the emotion value (+4, `006ecd40`) becomes
/// `iStandardEmotionValue` (`011cbdf4`, through `0043d4d0`). The
/// exception-unwinding frame and the security-cookie check are not
/// translated.
pub fn tes_response_init_item(e: &mut Engine, this: Ptr<TesResponse>, owner: Ptr) {
    let fields = [
        (0x08, TOPIC_TYPE, FORMAT_NO_TOPIC, true),
        (0x10, SOUND_TYPE, FORMAT_NO_SOUND, true),
        (0x20, IDLE_TYPE, FORMAT_NO_SPEAKER_IDLE, true),
        (0x24, IDLE_TYPE, FORMAT_NO_LISTENER_IDLE, false),
    ];
    for (offset, form_type, format, describe) in fields {
        if e.mem.u32(this.addr() + offset) == 0 {
            continue;
        }
        let (id, missing) = resolve_field(e, this, offset, owner, form_type);
        if missing {
            warn_missing(e, owner, format, id, describe);
        }
    }
    if e.call(WORD_AT_ZERO, &args![this]).u32() != 0 {
        let value = e.call(SETTING_VALUE, &args![STANDARD_EMOTION_VALUE]).u32();
        let value = e.mem.u32(value);
        e.call(SET_WORD_AT_4, &args![this, value]);
    }
}

/// `00406d50`: appends a string to a buffer (`cdecl`: buffer, size, text).
const STRING_APPEND: u32 = 0x0040_6d50;
/// `00438390` (`ECX` = string, one word): sets the string from a C string.
const STRING_SET_TEXT: u32 = 0x0043_8390;
/// `"\\"` and `"."`.
const TEXT_BACKSLASH: u32 = 0x0101_3444;
const TEXT_DOT: u32 = 0x0104_4c28;
/// The path buffer's size (`0x104`, as passed to the appends).
const PATH_SIZE: u32 = 0x104;
/// `004046f0`: copies a C string (`cdecl`: destination, source).
const STRING_COPY: u32 = 0x0040_46f0;
/// `_strlen` (`00ec6130`, `cdecl`).
const STRLEN: u32 = 0x00ec_6130;
/// `00406d00`: formats into a buffer (`cdecl`: buffer, size, format, ...).
const FORMAT_INTO: u32 = 0x0040_6d00;
/// `"%s_%s"`.
const FORMAT_QUEST_TOPIC: u32 = 0x0104_c988;
/// `00406f60`: formats into a string (`cdecl`: string, format, ...).
const STRING_FORMAT: u32 = 0x0040_6f60;
/// `"%s_%08X_%u"`.
const FORMAT_AUDIO_FILENAME: u32 = 0x0104_c9a0;
/// `"New Response"`.
const TEXT_NEW_RESPONSE: u32 = 0x0104_c990;
/// `"Data\\Sound\\Voice"`.
const TEXT_VOICE_FOLDER: u32 = 0x0104_c974;
/// `00639b40` (`ECX` = info): an object whose `+0x30` component's slot
/// `0x68` gives a voice type, or 0.
const INFO_SPEAKER_00639B40: u32 = 0x0063_9b40;
/// `009185e0` (`ECX` = form): the voice type of a form of type `0x16`.
const FORM_VOICE_009185E0: u32 = 0x0091_85e0;
/// `TESRaceForm::GetFormRace` (engine map), `cdecl`, one word.
const GET_FORM_RACE: u32 = 0x0061_7270;
/// `00604780` (`ECX` = race, one word): the race's voice type.
const RACE_VOICE_00604780: u32 = 0x0060_4780;
/// `00891170` (`ECX` = file): the file's name.
const FILE_NAME_00891170: u32 = 0x0089_1170;
/// The object `00403df0` reads the file extension from.
const VOICE_EXTENSION_011CBE14: u32 = 0x011c_be14;
/// `00403df0` (`ECX` = [`VOICE_EXTENSION_011CBE14`]): its text.
const SETTING_TEXT: u32 = 0x0040_3df0;
/// The size of `006172c0`'s two name buffers (`EBP-0x218`, `EBP-0x110`).
const NAME_BUFFER_SIZE: u32 = 0x108;

// Translated from 00616e20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00616e20 - the /GS cookie check (00ec408c) is not translated
/// No map name: joins `folder\file\voice\name.extension` in a 0x104-byte
/// buffer (`00406d50` for each part) and sets `result` to it. Always true.
/// `this` is kept but not read.
pub fn fn_00616e20(
    e: &mut Engine,
    _this: Ptr<TesResponse>,
    result: Ptr,
    folder: u32,
    file: u32,
    voice: u32,
    name: u32,
    extension: u32,
) -> bool {
    e.with_stack(0x108, |e, buffer| {
        let buffer = buffer.addr();
        e.mem.set_u8(buffer, 0);
        for part in [
            folder,
            TEXT_BACKSLASH,
            file,
            TEXT_BACKSLASH,
            voice,
            TEXT_BACKSLASH,
            name,
            TEXT_DOT,
            extension,
        ] {
            e.call(STRING_APPEND, &args![buffer, PATH_SIZE, part]);
        }
        e.call(STRING_SET_TEXT, &args![result, buffer]);
    });
    true
}

// Translated from 00616f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: [`tes_response_get_audio_full_path`] for the form at
/// `speaker + 0x20` (`007af430`) when `speaker` is set and that form's
/// type (`00401170`) is `0x16` or `0x2a`..`0x2b`; false otherwise.
pub fn fn_00616f40(
    e: &mut Engine,
    this: Ptr<TesResponse>,
    speaker: Ptr,
    quest: u32,
    topic: u32,
    info: u32,
    result: Ptr,
) -> bool {
    if speaker.is_null() {
        return false;
    }
    let form = e.call(WORD_AT_0X20, &args![speaker]).u32();
    let kind = e.call(FORM_TYPE, &args![form]).i32();
    if kind == 0x16 || (kind > 0x29 && kind <= 0x2b) {
        let form = e.call(WORD_AT_0X20, &args![speaker]).ptr();
        return tes_response_get_audio_full_path(e, this, form, quest, topic, info, result);
    }
    false
}

// Translated from 00616fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00616fa0 - the C++ exception frame (FS:[0]) is not translated
/// `TESResponse::GetAudioFullPath` (engine map): the response's voice file
/// path into `result`. The file name comes from
/// [`tes_response_get_audio_filename`]; the voice type from the info's
/// speaker (`00639b40`, its `+0x30` slot `0x68`), else from the form: its
/// `+0x30` slot `0x68` for types `0x2a` and `0x2b`, `009185e0` for `0x16`,
/// its race's (`00604780(race, 0)`) otherwise. False when the info has no
/// file, there is no voice type (or race), the voice type has no name
/// (slot `0x130`) or the file no name (`00891170`); true once the path is
/// built ([`fn_00616e20`]).
pub fn tes_response_get_audio_full_path(
    e: &mut Engine,
    this: Ptr<TesResponse>,
    form: Ptr,
    quest: u32,
    topic: u32,
    info: u32,
    result: Ptr,
) -> bool {
    e.with_stack(0x10, |e, strings| {
        let full = strings.addr() + 8;
        let filename = strings.addr();
        e.call(STRING_CONSTRUCT, &args![full]);
        e.call(STRING_CONSTRUCT, &args![filename]);
        let done = |e: &mut Engine, value: bool| {
            e.call(STRING_DESTRUCT, &args![filename]);
            e.call(STRING_DESTRUCT, &args![full]);
            value
        };
        tes_response_get_audio_filename(e, this, quest, topic, info, Ptr::new(filename));
        let mut voice = 0u32;
        let file = e.call(GET_FILE, &args![info, 0u32]).u32();
        if file == 0 {
            return done(e, false);
        }
        let mut file_name = 0u32;
        if info != 0 && e.call(INFO_SPEAKER_00639B40, &args![info]).u32() != 0 {
            let speaker = e.call(INFO_SPEAKER_00639B40, &args![info]).u32();
            voice = e.vcall(speaker + 0x30, 0x68, &args![]).u32();
        }
        if voice == 0 {
            let form = form.addr();
            if e.call(FORM_TYPE, &args![form]).u32() == 0x2a
                || e.call(FORM_TYPE, &args![form]).u32() == 0x2b
            {
                voice = e.vcall(form + 0x30, 0x68, &args![]).u32();
            } else if e.call(FORM_TYPE, &args![form]).u32() == 0x16 {
                voice = e.call(FORM_VOICE_009185E0, &args![form]).u32();
            } else {
                let race = e.call(GET_FORM_RACE, &args![form]).u32();
                if race == 0 {
                    return done(e, false);
                }
                voice = e.call(RACE_VOICE_00604780, &args![race, 0u32]).u32();
            }
        }
        if voice == 0 {
            return done(e, false);
        }
        let voice_name = e.vcall(voice, TOPIC_NAME_SLOT, &args![]).u32();
        if voice_name == 0 {
            return done(e, false);
        }
        if file != 0 {
            file_name = e.call(FILE_NAME_00891170, &args![file]).u32();
            if file_name == 0 {
                return done(e, false);
            }
        }
        let extension = e.call(SETTING_TEXT, &args![VOICE_EXTENSION_011CBE14]).u32();
        let name = e.call(WORD_AT_ZERO, &args![filename]).u32();
        fn_00616e20(
            e,
            this,
            result,
            TEXT_VOICE_FOLDER,
            file_name,
            voice_name,
            name,
            extension,
        );
        done(e, true)
    })
}

// Translated from 006172c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006172c0 - the /GS cookie check (00ec408c) is not translated
/// No map name, `cdecl`: writes `"<quest name>_<topic name>"` (quest slot
/// `0x138`, topic slot `0x130`, each copied into a 0x108-byte buffer) into
/// `buffer` (`size` bytes). Nothing without a buffer or a size; an empty
/// string without a quest or a topic. When the two names are longer than
/// 0x19 together, the topic name is cut at `0xf + 10 - quest length`, or,
/// for a quest name longer than 10, the quest name at 10 and the topic name
/// at 0xf.
pub fn fn_006172c0(e: &mut Engine, quest: u32, topic: u32, buffer: Ptr, size: u32) {
    if buffer.is_null() || size == 0 {
        return;
    }
    e.mem.set_u8(buffer.addr(), 0);
    if quest == 0 || topic == 0 {
        return;
    }
    e.with_stack(2 * NAME_BUFFER_SIZE, |e, names| {
        let quest_name = names.addr();
        let topic_name = quest_name + NAME_BUFFER_SIZE;
        let text = e.vcall(quest, QUEST_NAME_SLOT, &args![]).u32();
        e.call(STRING_COPY, &args![quest_name, text]);
        let text = e.vcall(topic, TOPIC_NAME_SLOT, &args![]).u32();
        e.call(STRING_COPY, &args![topic_name, text]);
        let quest_length = e.call(STRLEN, &args![quest_name]).u32();
        let topic_length = e.call(STRLEN, &args![topic_name]).u32();
        if quest_length.wrapping_add(topic_length) > 0x19 {
            let quest_limit = 0xau32;
            let mut topic_limit = 0xfu32;
            if quest_length > quest_limit {
                e.mem.set_u8(quest_name + quest_limit, 0);
            } else {
                topic_limit = quest_limit
                    .wrapping_sub(quest_length)
                    .wrapping_add(topic_limit);
            }
            e.mem.set_u8(topic_name.wrapping_add(topic_limit), 0);
        }
        e.call(
            FORMAT_INTO,
            &args![buffer, size, FORMAT_QUEST_TOPIC, quest_name, topic_name],
        );
    });
}

// Translated from 00617400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00617400 - the /GS cookie check (00ec408c) is not translated
/// `TESResponse::GetAudioFilename` (engine map): with a quest, a topic and
/// an info, sets `result` to `"<quest>_<topic>_<info id & 0xffffff as
/// %08X>_<response number>"` ([`fn_006172c0`] into a 0x104-byte buffer),
/// or to `"New Response"` when the response number is 0.
pub fn tes_response_get_audio_filename(
    e: &mut Engine,
    this: Ptr<TesResponse>,
    quest: u32,
    topic: u32,
    info: u32,
    result: Ptr,
) {
    if quest == 0 || topic == 0 || info == 0 {
        return;
    }
    let number = u32::from(fn_006174b0(e, this));
    if number == 0 {
        e.call(STRING_SET, &args![result, TEXT_NEW_RESPONSE, 0u32]);
        return;
    }
    e.with_stack(0x10c, |e, buffer| {
        fn_006172c0(e, quest, topic, buffer, PATH_SIZE);
        let id = e.call(FORM_ID, &args![info]).u32() & 0x00ff_ffff;
        e.call(
            STRING_FORMAT,
            &args![result, FORMAT_AUDIO_FILENAME, buffer, id, number],
        );
    });
}

// Translated from 006174b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the response's number, the byte `Data.ucResponseID` (+0xc).
pub fn fn_006174b0(e: &mut Engine, this: Ptr<TesResponse>) -> u8 {
    e.get(this, TesResponse::ucResponseID)
}

/// The type descriptor of `TESRaceForm` (`01186bc0`) and the word at +4
/// (`00726070`, `thiscall`): the race form's race.
const TES_RACE_FORM_TYPE: u32 = 0x0118_6bc0;
const WORD_AT_4: u32 = 0x0072_6070;

// Translated from 00617270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRaceForm::GetFormRace` (engine map name; cdecl, the form): the race
/// of `form` when it is a `TESRaceForm` (`__RTDynamicCast` from `TESForm`;
/// the word `00726070` reads at +4), else 0.
pub fn tes_race_form_get_form_race(e: &mut Engine, form: u32) -> u32 {
    let race_form = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TES_FORM_TYPE, TES_RACE_FORM_TYPE, 0u32],
        )
        .u32();
    if race_form != 0 {
        e.call(WORD_AT_4, &args![race_form]).u32()
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00616c70, fn_00616c70(Ptr<TesResponse>)),
        entry!(
            0x00616de0,
            tes_response_get_response_text(Ptr<TesResponse>) -> u32
        ),
        entry!(0x006174d0, fn_006174d0(Ptr<TesResponse>, Ptr<TesResponse>)),
        entry!(0x00617530, tes_response_load(Ptr<TesResponse>, Ptr)),
        entry!(0x00617580, tes_response_init_item(Ptr<TesResponse>, Ptr)),
        entry!(
            0x0061_6e20,
            fn_00616e20(Ptr<TesResponse>, Ptr, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x0061_6f40,
            fn_00616f40(Ptr<TesResponse>, Ptr, u32, u32, u32, Ptr) -> bool
        ),
        entry!(
            0x0061_6fa0,
            tes_response_get_audio_full_path(Ptr<TesResponse>, Ptr, u32, u32, u32, Ptr) -> bool
        ),
        entry!(0x0061_72c0, fn_006172c0(u32, u32, Ptr, u32)),
        entry!(
            0x0061_7400,
            tes_response_get_audio_filename(Ptr<TesResponse>, u32, u32, u32, Ptr)
        ),
        entry!(0x0061_74b0, fn_006174b0(Ptr<TesResponse>) -> u8),
        entry!(0x0061_7270, tes_race_form_get_form_race(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn response(e: &mut Engine) -> u32 {
        e.mem.alloc(0x2c)
    }

    #[test]
    fn the_data_words_are_swapped() {
        let mut e = Engine::new();
        e.register(BYTE_SWAP_WORD, |_, _| ret(0));
        e.call_log = Some(vec![]);
        let p = response(&mut e);
        e.call(0x0061_6c70, &args![p]);
        assert_eq!(
            calls(&e, BYTE_SWAP_WORD),
            vec![
                vec![p, 0],
                vec![p + 4, 0],
                vec![p + 8, 0],
                vec![p + 0x10, 0]
            ]
        );
    }

    #[test]
    fn the_text_is_the_string_or_empty() {
        let mut e = Engine::new();
        e.register(WORD_AT_ZERO, |e, a| ret(e.mem.u32(a[0])));
        let p = response(&mut e);
        e.mem.set_u32(p + 0x18, 0);
        assert_eq!(e.call(0x0061_6de0, &args![p]).u32(), EMPTY_STRING);
        e.mem.set_u32(p + 0x18, 0x1234);
        assert_eq!(e.call(0x0061_6de0, &args![p]).u32(), 0x1234);
    }

    #[test]
    fn the_copy_moves_data_text_and_idles() {
        let mut e = Engine::new();
        e.register(SELF, |_, a| ret(a[0]));
        e.register(MEMCPY, |_, a| ret(a[0]));
        e.register(WORD_AT_ZERO, |e, a| ret(e.mem.u32(a[0])));
        e.register(STRING_SET, |_, _| ret(1));
        e.register(WORD_AT_0X20, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(WORD_AT_0X24, |e, a| ret(e.mem.u32(a[0] + 0x24)));
        e.register(SET_WORD_AT_0X20, |_, _| ret(0));
        e.register(SET_WORD_AT_0X24, |_, _| ret(0));
        e.call_log = Some(vec![]);
        let this = response(&mut e);
        let source = response(&mut e);
        e.mem.set_u32(source + 0x18, 0);
        e.mem.set_u32(source + 0x20, 0x111);
        e.mem.set_u32(source + 0x24, 0x222);
        e.call(0x0061_74d0, &args![this, source]);
        assert_eq!(calls(&e, MEMCPY), vec![vec![this, source, 0x18]]);
        assert_eq!(
            calls(&e, STRING_SET),
            vec![vec![this + 0x18, EMPTY_STRING, 0]]
        );
        assert_eq!(calls(&e, SET_WORD_AT_0X20), vec![vec![this, 0x111]]);
        assert_eq!(calls(&e, SET_WORD_AT_0X24), vec![vec![this, 0x222]]);
    }

    #[test]
    fn load_reads_trdt_and_swaps_when_asked() {
        let mut e = Engine::new();
        e.map(0x0b00_0000, 0x10);
        e.register(GET_TES_CHUNK, |e, _| ret(e.mem.u32(0x0b00_0000)));
        e.register(GET_CHUNK_DATA, |_, _| ret(1));
        e.register(FILE_NEEDS_SWAP, |e, _| ret(e.mem.u32(0x0b00_0004)));
        e.register(BYTE_SWAP_WORD, |_, _| ret(0));
        e.call_log = Some(vec![]);
        let p = response(&mut e);
        let file = e.mem.alloc(4);
        // No file: nothing.
        e.call(0x0061_7530, &args![p, 0u32]);
        assert!(calls(&e, GET_TES_CHUNK).is_empty());
        // Another chunk: nothing read.
        e.mem.set_u32(0x0b00_0000, 0x4d414e31);
        e.call(0x0061_7530, &args![p, file]);
        assert!(calls(&e, GET_CHUNK_DATA).is_empty());
        // TRDT without swapping.
        e.mem.set_u32(0x0b00_0000, CHUNK_TRDT);
        e.call(0x0061_7530, &args![p, file]);
        assert_eq!(calls(&e, GET_CHUNK_DATA), vec![vec![file, p, 0x18]]);
        assert!(calls(&e, BYTE_SWAP_WORD).is_empty());
        // TRDT with swapping.
        e.mem.set_u32(0x0b00_0004, 1);
        e.call(0x0061_7530, &args![p, file]);
        assert_eq!(calls(&e, BYTE_SWAP_WORD).len(), 4);
    }

    /// Doubles for `InitItem`: forms 0x5000_0000 + type found by id
    /// `0x100 + type` (the compile index adds 0x0100_0000 to the id).
    fn init_engine() -> Engine {
        let mut e = Engine::new();
        e.register(GET_FILE, |_, _| ret(0x6000));
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            ret(0)
        });
        e.register(LOOKUP_FORM, |_, a| {
            let id = a[0] & 0x00ff_ffff;
            if (0x100..0x200).contains(&id) {
                ret(0x5000_0000 + (id - 0x100))
            } else {
                ret(0)
            }
        });
        e.register(FORM_TYPE, |_, a| ret(a[0] - 0x5000_0000));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0] + 1));
        e.register(OWNER_TOPIC, |_, a| {
            ret(if a[0] == 0x9001 { 0x9100 } else { 0 })
        });
        e.register(OWNER_QUEST, |_, _| ret(0x9200));
        e.register(STRING_CONSTRUCT, |_, a| ret(a[0]));
        e.register(STRING_DESTRUCT, |_, _| ret(0));
        e.register(GET_INFO_DISPLAY_TEXT, |e, a| {
            e.mem.set_u32(a[1], 0x7777);
            ret(0)
        });
        e.register(WORD_AT_ZERO, |e, a| ret(e.mem.u32(a[0])));
        e.register(FORM_ID, |_, a| ret(a[0] + 0x10));
        e.register(SPRINTF, |_, _| ret(0));
        e.register(WARNING, |_, _| ret(0));
        e.register(SETTING_VALUE, |_, a| ret(a[0] + 4));
        e.register(SET_WORD_AT_4, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            ret(0)
        });
        e.map(0x9100, 0x400);
        e.mem.set_u32(0x9100, 0x9180);
        e.mem.set_u32(0x9180 + TOPIC_NAME_SLOT, 0x00ab_0001);
        e.mem.set_u32(0x9200, 0x9280);
        e.mem.set_u32(0x9280 + QUEST_NAME_SLOT, 0x00ab_0002);
        e.register(0x00ab_0001, |_, _| ret(0x5151));
        e.register(0x00ab_0002, |_, _| ret(0x5252));
        e.map(STANDARD_EMOTION_VALUE, 8);
        e.mem.set_u32(STANDARD_EMOTION_VALUE + 4, 100);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn init_item_resolves_forms_of_the_right_type() {
        let mut e = init_engine();
        let p = response(&mut e);
        e.mem.set_u32(p, 0);
        e.mem.set_u32(p + 4, 50);
        e.mem.set_u32(p + 8, 0x100 + TOPIC_TYPE);
        e.mem.set_u32(p + 0x10, 0x100 + SOUND_TYPE);
        e.mem.set_u32(p + 0x20, 0x100 + IDLE_TYPE);
        e.mem.set_u32(p + 0x24, 0);
        e.call(0x0061_7580, &args![p, 0x9000u32]);
        assert_eq!(e.mem.u32(p + 8), 0x5000_0000 + TOPIC_TYPE);
        assert_eq!(e.mem.u32(p + 0x10), 0x5000_0000 + SOUND_TYPE);
        assert_eq!(e.mem.u32(p + 0x20), 0x5000_0000 + IDLE_TYPE);
        assert_eq!(e.mem.u32(p + 0x24), 0);
        assert!(calls(&e, WARNING).is_empty());
        // A neutral emotion keeps its value.
        assert_eq!(e.mem.u32(p + 4), 50);
        assert_eq!(calls(&e, LOOKUP_FORM).len(), 3);
        assert_eq!(calls(&e, GET_FILE), vec![vec![0x9000, ANY_FILE]; 3]);
    }

    #[test]
    fn init_item_warns_for_missing_forms_and_sets_the_emotion_value() {
        let mut e = init_engine();
        let p = response(&mut e);
        e.mem.set_u32(p, 3);
        e.mem.set_u32(p + 4, 50);
        // Wrong type for the topic; not found for the others.
        e.mem.set_u32(p + 8, 0x100 + SOUND_TYPE);
        e.mem.set_u32(p + 0x10, 0x300);
        e.mem.set_u32(p + 0x20, 0x100 + TOPIC_TYPE);
        e.mem.set_u32(p + 0x24, 0x301);
        // The info 0x9001 has a topic and quest.
        e.call(0x0061_7580, &args![p, 0x9000u32]);
        for offset in [8, 0x10, 0x20, 0x24] {
            assert_eq!(e.mem.u32(p + offset), 0, "{offset:#x}");
        }
        let warnings = calls(&e, WARNING);
        assert_eq!(warnings.len(), 4);
        let formats: Vec<_> = warnings.iter().map(|w| (w[0], w[1])).collect();
        assert_eq!(
            formats,
            vec![
                (FORMAT_NO_TOPIC, 0x0100_0100 + SOUND_TYPE),
                (FORMAT_NO_SOUND, 0x0100_0300),
                (FORMAT_NO_SPEAKER_IDLE, 0x0100_0100 + TOPIC_TYPE),
                (FORMAT_NO_LISTENER_IDLE, 0x0100_0301),
            ]
        );
        // Three descriptions, none for the listener idle.
        let sprintfs = calls(&e, SPRINTF);
        assert_eq!(sprintfs.len(), 3);
        assert_eq!(
            sprintfs[0][1..].to_vec(),
            vec![FORMAT_INFO, 0x9011, 0x7777, 0x5252, 0x9210, 0x5151, 0x9110]
        );
        assert_eq!(warnings[0][2], sprintfs[0][0]);
        assert_eq!(
            calls(&e, GET_INFO_DISPLAY_TEXT).len(),
            4,
            "the listener idle reads the text too"
        );
        assert_eq!(calls(&e, STRING_DESTRUCT).len(), 4);
        assert_eq!(calls(&e, OWNER_QUEST)[0], vec![0x9100, 0x9001]);
        // A nonzero emotion takes iStandardEmotionValue.
        assert_eq!(e.mem.u32(p + 4), 100);
        assert_eq!(calls(&e, SET_WORD_AT_4), vec![vec![p, 100]]);
    }

    #[test]
    fn init_item_describes_an_info_without_topic() {
        let mut e = init_engine();
        let p = response(&mut e);
        e.mem.set_u32(p, 0);
        e.mem.set_u32(p + 8, 0);
        e.mem.set_u32(p + 0x10, 0x300);
        e.mem.set_u32(p + 0x20, 0);
        e.mem.set_u32(p + 0x24, 0);
        // The info 0x9101 has no topic: no quest either.
        e.call(0x0061_7580, &args![p, 0x9100u32]);
        assert!(calls(&e, OWNER_QUEST).is_empty());
        let sprintfs = calls(&e, SPRINTF);
        assert_eq!(
            sprintfs[0][1..].to_vec(),
            vec![
                FORMAT_INFO,
                0x9111,
                0x7777,
                EMPTY_STRING,
                0,
                EMPTY_STRING,
                0
            ]
        );
        assert!(calls(&e, SET_WORD_AT_4).is_empty());
    }
}

#[cfg(test)]
mod b040_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let block = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(block, s);
        block
    }

    /// C string doubles: copy, append, length; the formatter joins its
    /// string arguments with `_` (enough to see what was passed).
    fn c_string_doubles(e: &mut Engine) {
        e.register(STRING_COPY, |e, a| {
            let s = e.mem.cstr(a[1]);
            e.mem.set_cstr(a[0], &s);
            Ret::default()
        });
        e.register(STRING_APPEND, |e, a| {
            let mut s = e.mem.cstr(a[0]);
            s.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &s);
            Ret::default()
        });
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(FORMAT_INTO, |e, a| {
            let mut s = e.mem.cstr(a[3]);
            s.push(b'_');
            s.extend(e.mem.cstr(a[4]));
            e.mem.set_cstr(a[0], &s);
            Ret::default()
        });
    }

    fn named(e: &mut Engine, slot: u32, name: &[u8]) -> u32 {
        let object = e.mem.alloc(4);
        let table = e.mem.alloc(0x140);
        e.mem.set_u32(object, table);
        let name = text(e, name);
        let target = 0x7400_0000 + object;
        e.mem.set_u32(table + slot, target);
        e.register_double(target, move |_, _| name.into_ret());
        object
    }

    #[test]
    fn quest_topic_name_is_joined_and_cut() {
        let mut e = Engine::new();
        c_string_doubles(&mut e);
        let buffer = e.mem.alloc(0x104);
        for (quest, topic, expected) in [
            (&b"VMS"[..], &b"Greeting"[..], &b"VMS_Greeting"[..]),
            (
                b"Quest",
                b"ABCDEFGHIJKLMNOPQRSTUVW",
                b"Quest_ABCDEFGHIJKLMNOPQRST",
            ),
            (
                b"LongQuestNameHere",
                b"ABCDEFGHIJKLMNOPQRSTUVW",
                b"LongQuestN_ABCDEFGHIJKLMNO",
            ),
        ] {
            let q = named(&mut e, QUEST_NAME_SLOT, quest);
            let t = named(&mut e, TOPIC_NAME_SLOT, topic);
            e.call(0x0061_72c0, &args![q, t, buffer, 0x104u32]);
            assert_eq!(e.mem.cstr(buffer), expected);
        }
        // No quest: an empty string; no buffer or size: untouched.
        e.mem.set_cstr(buffer, b"x");
        e.call(0x0061_72c0, &args![0u32, 1u32, buffer, 0x104u32]);
        assert_eq!(e.mem.cstr(buffer), b"");
        e.mem.set_cstr(buffer, b"x");
        e.call(0x0061_72c0, &args![1u32, 1u32, buffer, 0u32]);
        assert_eq!(e.mem.cstr(buffer), b"x");
    }

    #[test]
    fn response_number_and_join() {
        let mut e = Engine::new();
        let this = e.new_object::<TesResponse>();
        e.set(this, TesResponse::ucResponseID, 7);
        assert_eq!(e.call(0x0061_74b0, &args![this]).u8(), 7);

        c_string_doubles(&mut e);
        e.map(0x0101_3000, 0x1000);
        e.mem.set_cstr(TEXT_BACKSLASH, b"\\");
        e.map(0x0104_4000, 0x1000);
        e.mem.set_cstr(TEXT_DOT, b".");
        e.register(STRING_SET_TEXT, |e, a| {
            let s = e.mem.cstr(a[1]);
            let copy = e.mem.alloc(s.len() as u32 + 1);
            e.mem.set_cstr(copy, &s);
            e.mem.set_u32(a[0], copy);
            Ret::default()
        });
        let parts: Vec<u32> = [&b"Data"[..], b"F.esm", b"Voice", b"Name", b"ogg"]
            .iter()
            .map(|s| text(&mut e, s))
            .collect();
        let result = e.mem.alloc(8);
        let ok = e.call(
            0x0061_6e20,
            &args![this, result, parts[0], parts[1], parts[2], parts[3], parts[4]],
        );
        assert!(ok.bool());
        assert_eq!(
            e.mem.cstr(e.mem.u32(result)),
            b"Data\\F.esm\\Voice\\Name.ogg"
        );
    }

    #[test]
    fn audio_filename_formats_or_names_a_new_response() {
        let mut e = Engine::new();
        c_string_doubles(&mut e);
        let this = e.new_object::<TesResponse>();
        e.register(STRING_SET, |_, _| Ret::default());
        e.register(STRING_FORMAT, |_, _| Ret::default());
        e.register(FORM_ID, |_, _| 0x1234_5678u32.into_ret());
        let q = named(&mut e, QUEST_NAME_SLOT, b"Q");
        let t = named(&mut e, TOPIC_NAME_SLOT, b"T");
        e.call_log = Some(vec![]);
        e.call(0x0061_7400, &args![this, q, t, 0x99u32, 0x5000u32]);
        assert_eq!(
            calls_to(&e, STRING_SET),
            vec![vec![0x5000, TEXT_NEW_RESPONSE, 0]]
        );
        e.set(this, TesResponse::ucResponseID, 3);
        e.call_log = Some(vec![]);
        e.call(0x0061_7400, &args![this, q, t, 0x99u32, 0x5000u32]);
        let format = calls_to(&e, STRING_FORMAT);
        assert_eq!(format.len(), 1);
        assert_eq!(format[0][..2], [0x5000, FORMAT_AUDIO_FILENAME]);
        assert_eq!(e.mem.cstr(format[0][2]), b"Q_T");
        assert_eq!(format[0][3..], [0x0034_5678, 3]);
        e.call_log = Some(vec![]);
        e.call(0x0061_7400, &args![this, q, t, 0u32, 0x5000u32]);
        assert!(calls_to(&e, FORM_ID).is_empty());
        assert!(calls_to(&e, STRING_SET).is_empty());
    }

    struct Path {
        this: Ptr<TesResponse>,
        form: u32,
        voice: u32,
    }

    fn path_engine(e: &mut Engine, form_type: u32) -> Path {
        let this = e.new_object::<TesResponse>();
        e.set(this, TesResponse::ucResponseID, 0);
        for addr in [
            STRING_CONSTRUCT,
            STRING_DESTRUCT,
            STRING_SET,
            STRING_SET_TEXT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(STRING_APPEND, |_, _| Ret::default());
        e.register(WORD_AT_ZERO, |_, _| 0x5a5au32.into_ret());
        e.register(GET_FILE, |_, _| 0x6600u32.into_ret());
        e.register(INFO_SPEAKER_00639B40, |_, _| 0u32.into_ret());
        e.register_double(FORM_TYPE, move |_, _| form_type.into_ret());
        let voice = named(e, TOPIC_NAME_SLOT, b"MaleAdult");
        let form = e.mem.alloc(0x40);
        let component = e.mem.alloc(4);
        let table = e.mem.alloc(0x70);
        e.mem.set_u32(component, table);
        e.mem.set_u32(form + 0x30, table);
        e.mem.set_u32(table + 0x68, 0x7500_0068);
        e.register_double(0x7500_0068, move |_, _| voice.into_ret());
        e.register_double(FORM_VOICE_009185E0, move |_, _| voice.into_ret());
        e.register(GET_FORM_RACE, |_, _| 0x7700u32.into_ret());
        e.register_double(RACE_VOICE_00604780, move |_, _| voice.into_ret());
        e.register(FILE_NAME_00891170, |_, _| 0x6611u32.into_ret());
        e.register(SETTING_TEXT, |_, _| 0x6622u32.into_ret());
        Path { this, form, voice }
    }

    fn joined(e: &Engine) -> Vec<Vec<u32>> {
        calls_to(e, STRING_SET_TEXT)
    }

    #[test]
    fn audio_full_path_by_form_type() {
        for form_type in [0x2au32, 0x2b, 0x16, 0x30] {
            let mut e = Engine::new();
            let p = path_engine(&mut e, form_type);
            e.call_log = Some(vec![]);
            let ok = e
                .call(
                    0x0061_6fa0,
                    &args![p.this, p.form, 0u32, 0u32, 0x4400u32, 0x5000u32],
                )
                .bool();
            assert!(ok, "type {form_type:x}");
            let appends = calls_to(&e, STRING_APPEND);
            let parts: Vec<u32> = appends.iter().map(|c| c[2]).collect();
            let voice_name = parts[4];
            assert_eq!(e.mem.cstr(voice_name), b"MaleAdult");
            assert_eq!(
                parts,
                vec![
                    TEXT_VOICE_FOLDER,
                    TEXT_BACKSLASH,
                    0x6611,
                    TEXT_BACKSLASH,
                    voice_name,
                    TEXT_BACKSLASH,
                    0x5a5a,
                    TEXT_DOT,
                    0x6622
                ]
            );
            assert_eq!(joined(&e)[0][0], 0x5000);
            assert_eq!(calls_to(&e, STRING_DESTRUCT).len(), 2);
            assert_eq!(calls_to(&e, GET_FILE), vec![vec![0x4400, 0]]);
            let race = calls_to(&e, RACE_VOICE_00604780);
            assert_eq!(race.len(), usize::from(form_type == 0x30));
            let _ = p.voice;
        }
    }

    #[test]
    fn audio_full_path_failures() {
        let mut e = Engine::new();
        let p = path_engine(&mut e, 0x30);
        let call = |e: &mut Engine| {
            e.call_log = Some(vec![]);
            e.call(
                0x0061_6fa0,
                &args![p.this, p.form, 0u32, 0u32, 0x4400u32, 0x5000u32],
            )
            .bool()
        };
        e.register(GET_FORM_RACE, |_, _| 0u32.into_ret());
        assert!(!call(&mut e));
        assert_eq!(calls_to(&e, STRING_DESTRUCT).len(), 2);
        e.register(GET_FORM_RACE, |_, _| 0x7700u32.into_ret());
        e.register(FILE_NAME_00891170, |_, _| 0u32.into_ret());
        assert!(!call(&mut e));
        assert!(joined(&e).is_empty());
        e.register(GET_FILE, |_, _| 0u32.into_ret());
        assert!(!call(&mut e));
        assert!(calls_to(&e, GET_FORM_RACE).is_empty());
        // The info's speaker gives the voice type first.
        e.register(GET_FILE, |_, _| 0x6600u32.into_ret());
        e.register(FILE_NAME_00891170, |_, _| 0x6611u32.into_ret());
        let speaker = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x70);
        e.mem.set_u32(speaker + 0x30, table);
        e.mem.set_u32(table + 0x68, 0x7500_0068);
        e.register_double(INFO_SPEAKER_00639B40, move |_, _| speaker.into_ret());
        assert!(call(&mut e));
        assert!(calls_to(&e, FORM_TYPE).is_empty());
        assert_eq!(calls_to(&e, 0x7500_0068), vec![vec![speaker + 0x30]]);
    }

    #[test]
    fn audio_full_path_for_a_speaker() {
        let mut e = Engine::new();
        let p = path_engine(&mut e, 0x2b);
        let form = p.form;
        e.register_double(WORD_AT_0X20, move |_, _| form.into_ret());
        let speaker = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        let ok = e.call(
            0x0061_6f40,
            &args![p.this, speaker, 1u32, 2u32, 0x4400u32, 0x5000u32],
        );
        assert!(ok.bool());
        assert_eq!(calls_to(&e, WORD_AT_0X20).len(), 2);
        assert_eq!(calls_to(&e, FORM_ID).len(), 0);
        for (form_type, expected) in [(0x16u32, true), (0x29, false), (0x2c, false)] {
            e.register_double(FORM_TYPE, move |_, _| form_type.into_ret());
            let ok = e.call(
                0x0061_6f40,
                &args![p.this, speaker, 1u32, 2u32, 0x4400u32, 0x5000u32],
            );
            assert_eq!(ok.bool(), expected);
        }
        let ok = e.call(
            0x0061_6f40,
            &args![p.this, 0u32, 1u32, 2u32, 0x4400u32, 0x5000u32],
        );
        assert!(!ok.bool());
    }
}

#[cfg(test)]
mod form_race_tests {
    use super::*;

    #[test]
    fn form_race_is_the_race_forms_word_or_zero() {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| {
            (if a[3] == TES_RACE_FORM_TYPE { a[0] } else { 0 }).into_ret()
        });
        e.register(WORD_AT_4, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let form = e.mem.alloc(8);
        e.mem.set_u32(form + 4, 0x6000);
        assert_eq!(e.call(0x0061_7270, &args![form]).u32(), 0x6000);
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_7270, &args![form]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != WORD_AT_4));
    }
}
