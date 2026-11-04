//! `fallout/dialogue/dialogueresponse.cpp` (Xbox PDB source unit), subsystem `fallout/dialogue`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0083d890 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `004048e0` of unit effectitem (the cached string
/// length, calculated when the field holds the sentinel 0xffff), and returns its result (a
/// `u32`). Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. The return type (`u32`) is that of `004048e0`'s
/// translated signature `fn_004048e0(Ptr) -> u32`.
pub fn fn_0083d890(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0040_48e0, &args![this]).u32()
}

layout! {
    /// `DialogueResponse` (Xbox PDB), 0x2c bytes (the PC code allocates 0x2c).
    pub struct DialogueResponse: 0x2c {
        /// `cText` (Xbox PDB): a `BSStringT<char>`.
        0x00 cText: Inline<crate::types::BSStringT>,
        /// `eEmotion` (Xbox PDB).
        0x08 eEmotion: u32,
        /// `uiEmotion` (Xbox PDB).
        0x0C uiEmotion: u32,
        /// `cVoice` (Xbox PDB): a `BSStringT<char>`.
        0x10 cVoice: Inline<crate::types::BSStringT>,
        /// `pSpeakerIdle` (Xbox PDB): `TESIdleForm*`.
        0x18 pSpeakerIdle: u32,
        /// `pListenIdle` (Xbox PDB): `TESIdleForm*`.
        0x1C pListenIdle: u32,
        /// `pVoiceSound` (Xbox PDB): `TESSound*`.
        0x20 pVoiceSound: u32,
        /// `bUseEmotion` (Xbox PDB).
        0x24 bUseEmotion: u8,
        /// `nResponseID` (Xbox PDB).
        0x28 nResponseID: u32,
    }
}

/// The `BSStringT<char>` constructor `004037b0`, `Set` from a C string `004037f0`
/// (`thiscall`, two words), `BSStringT<char>::Set` from a string `00438470` (`thiscall`,
/// one word) and the destructor `004037d0`.
const STRING_CONSTRUCTOR: u32 = 0x0040_37b0;
const STRING_SET_TEXT: u32 = 0x0040_37f0;
const STRING_SET: u32 = 0x0043_8470;
const STRING_DESTRUCTOR: u32 = 0x0040_37d0;
/// `TESResponse::GetResponseText` (engine map, `00616de0`, `thiscall`).
const GET_RESPONSE_TEXT: u32 = 0x0061_6de0;
/// `0083d8b0` (`thiscall`, one word: the string to fill): the text as shown.
const DISPLAY_STRING_0083D8B0: u32 = 0x0083_d8b0;
/// Getters of the response record (`thiscall`): `00559450` (+0, the emotion), `00726070`
/// (+4, the emotion value), `BGSSaveFormBuffer::GetForm` (`007af430`), `0059bb30`,
/// `BaseProcess::GetCurrentProcedureIndex` (`0044edb0`) and `00843150` (folded getters as
/// the engine map names them), `006174b0`.
const RECORD_WORD_0: u32 = 0x0055_9450;
const RECORD_WORD_4: u32 = 0x0072_6070;
const RECORD_SPEAKER_IDLE_007AF430: u32 = 0x007a_f430;
const RECORD_LISTEN_IDLE_0059BB30: u32 = 0x0059_bb30;
const RECORD_SOUND_0044EDB0: u32 = 0x0044_edb0;
const RECORD_USE_EMOTION_00843150: u32 = 0x0084_3150;
const RECORD_ID_006174B0: u32 = 0x0061_74b0;
/// `TESTopic::GetOwnerTopic` (engine map, `0061a310`, `cdecl`, one word).
const GET_OWNER_TOPIC: u32 = 0x0061_a310;
/// `00616f40` (`tesresponse.cpp`, `thiscall`, five words): fills the voice file name.
const VOICE_FILE_NAME_00616F40: u32 = 0x0061_6f40;
/// `BGSSaveGameBuffer::SaveString` (`00865e70`, two words), the raw write `00865e50`
/// (three words: address, size, 0) and `SaveFormID_ov2` (`00865df0`, two words).
const SAVE_STRING: u32 = 0x0086_5e70;
const SAVE_DATA: u32 = 0x0086_5e50;
const SAVE_FORM_ID: u32 = 0x0086_5df0;

// Translated from 0083d280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `DialogueResponse` constructor (unnamed in the engine map; `DialogueItem`'s
/// constructor calls it on 0x2c-byte blocks; `thiscall`, `RET 0x14`: quest, an unread
/// word, info, speaker, the response record). Constructs `cText` and `cVoice`. Without a
/// record it clears the idles, emotion, sound and id and sets `bUseEmotion`. With one:
/// `cText` is set from the record's text (`TESResponse::GetResponseText` into a stack
/// string, `0083d8b0` into a second one, `BSStringT::Set`), the emotion from its words +0
/// and +4, `cVoice` from `00616f40` (speaker, quest, the info's owner topic, info), and the
/// idles, sound, `bUseEmotion` and `nResponseID` from the record's getters. Returns `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 0083d280 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn dialogue_response_constructor(
    e: &mut Engine,
    this: Ptr<DialogueResponse>,
    quest: u32,
    _unused_2: u32,
    info: u32,
    speaker: u32,
    record: u32,
) -> Ptr<DialogueResponse> {
    let text = this.at(DialogueResponse::cText).addr();
    let voice = this.at(DialogueResponse::cVoice).addr();
    e.call(STRING_CONSTRUCTOR, &args![text]);
    e.call(STRING_CONSTRUCTOR, &args![voice]);
    if record == 0 {
        e.set(this, DialogueResponse::pSpeakerIdle, 0);
        e.set(this, DialogueResponse::pListenIdle, 0);
        e.set(this, DialogueResponse::eEmotion, 0);
        e.set(this, DialogueResponse::uiEmotion, 0);
        e.set(this, DialogueResponse::pVoiceSound, 0);
        e.set(this, DialogueResponse::bUseEmotion, 1);
        e.set(this, DialogueResponse::nResponseID, 0);
        return this;
    }
    e.with_stack(0x10, |e, locals| {
        let shown = locals.addr();
        let raw = shown + 8;
        e.call(STRING_CONSTRUCTOR, &args![raw]);
        let response_text = e.call(GET_RESPONSE_TEXT, &args![record]).u32();
        e.call(STRING_SET_TEXT, &args![raw, response_text, 0u32]);
        let shown = e.call(DISPLAY_STRING_0083D8B0, &args![raw, shown]).u32();
        e.call(STRING_SET, &args![text, shown]);
        e.call(STRING_DESTRUCTOR, &args![locals]);
        let emotion = e.call(RECORD_WORD_0, &args![record]).u32();
        e.set(this, DialogueResponse::eEmotion, emotion);
        let value = e.call(RECORD_WORD_4, &args![record]).u32();
        e.set(this, DialogueResponse::uiEmotion, value);
        let owner = e.call(GET_OWNER_TOPIC, &args![info]).u32();
        e.call(
            VOICE_FILE_NAME_00616F40,
            &args![record, speaker, quest, owner, info, voice],
        );
        let idle = e.call(RECORD_SPEAKER_IDLE_007AF430, &args![record]).u32();
        e.set(this, DialogueResponse::pSpeakerIdle, idle);
        let idle = e.call(RECORD_LISTEN_IDLE_0059BB30, &args![record]).u32();
        e.set(this, DialogueResponse::pListenIdle, idle);
        let sound = e.call(RECORD_SOUND_0044EDB0, &args![record]).u32();
        e.set(this, DialogueResponse::pVoiceSound, sound);
        let use_emotion = e.call(RECORD_USE_EMOTION_00843150, &args![record]).u8();
        e.set(this, DialogueResponse::bUseEmotion, use_emotion);
        let id = e.call(RECORD_ID_006174B0, &args![record]).u32();
        e.set(this, DialogueResponse::nResponseID, id);
        e.call(STRING_DESTRUCTOR, &args![raw]);
    });
    this
}

// Translated from 0083d690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DialogueResponse::SaveGame` (engine map; `thiscall`, `RET 4`: the save buffer): saves
/// `cText` and `cVoice` (their character pointers, `SaveString` with 0), `eEmotion` and
/// `uiEmotion` (4 bytes each), `bUseEmotion` (1 byte), and the form ids of the two idles
/// and the voice sound.
pub fn dialogue_response_save_game(e: &mut Engine, this: Ptr<DialogueResponse>, buffer: u32) {
    for string in [0u32, 0x10] {
        let chars = e.call(RECORD_WORD_0, &args![this.addr() + string]).u32();
        e.call(SAVE_STRING, &args![buffer, chars, 0u32]);
    }
    for (offset, size) in [(8u32, 4u32), (0xc, 4), (0x24, 1)] {
        e.call(SAVE_DATA, &args![buffer, this.addr() + offset, size, 0u32]);
    }
    for field in [
        DialogueResponse::pSpeakerIdle,
        DialogueResponse::pListenIdle,
        DialogueResponse::pVoiceSound,
    ] {
        let form = e.get(this, field);
        e.call(SAVE_FORM_ID, &args![buffer, form, 0u32]);
    }
}

/// The save/load singleton holder (`011de45c`) and its `thiscall` write (`008579b0`) and read
/// (`008579e0`), both (address, size).
const SAVE_LOAD_GAME_011DE45C: u32 = 0x011d_e45c;
const SAVE_WRITE_008579B0: u32 = 0x0085_79b0;
const LOAD_READ_008579E0: u32 = 0x0085_79e0;
/// `memset` (`00403d30`, `cdecl`: address, value, size).
const MEMSET_00403D30: u32 = 0x0040_3d30;
/// The empty C string (`01011584`).
const EMPTY_STRING_01011584: u32 = 0x0101_1584;
/// `BGSLoadGameBuffer` readers (`thiscall` on the buffer): `LoadString` (`008649a0`, one word:
/// where the characters go), the raw read `00864980` (address, size) and `LoadFormID`
/// (`008648a0`).
const LOAD_STRING: u32 = 0x0086_49a0;
const LOAD_DATA: u32 = 0x0086_4980;
const LOAD_FORM_ID: u32 = 0x0086_48a0;
/// `004839c0` (`cdecl`, one word) and `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, 0,
/// source type, target type, 0) with the source type `01183028` and the targets of the idles
/// (`01186a18`) and the voice sound (`011832fc`).
const CAST_SOURCE_004839C0: u32 = 0x0048_39c0;
const DYNAMIC_CAST_00EC43FB: u32 = 0x00ec_43fb;
const FORM_TYPE_01183028: u32 = 0x0118_3028;
const IDLE_TYPE_01186A18: u32 = 0x0118_6a18;
const SOUND_TYPE_011832FC: u32 = 0x0118_32fc;
/// The security cookie (`011c16bc`) and `__security_check_cookie` (`00ec408c`, fastcall).
const SECURITY_COOKIE_011C16BC: u32 = 0x011c_16bc;
const SECURITY_CHECK_COOKIE: u32 = 0x00ec_408c;

// Translated from 0083d1e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083d1e0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `DialogueResponse::DialogueResponse` (engine map; `thiscall`): constructs `cText` and
/// `cVoice` (`004037b0`) and clears the two idles, `eEmotion`, `uiEmotion`, the voice sound and
/// `nResponseID` (`bUseEmotion` is left as it is). Returns `this`.
pub fn dialogue_response_default_constructor(
    e: &mut Engine,
    this: Ptr<DialogueResponse>,
) -> Ptr<DialogueResponse> {
    e.call(STRING_CONSTRUCTOR, &args![this]);
    e.call(STRING_CONSTRUCTOR, &args![this.addr() + 0x10]);
    e.set(this, DialogueResponse::pSpeakerIdle, 0);
    e.set(this, DialogueResponse::pListenIdle, 0);
    e.set(this, DialogueResponse::eEmotion, 0);
    e.set(this, DialogueResponse::uiEmotion, 0);
    e.set(this, DialogueResponse::pVoiceSound, 0);
    e.set(this, DialogueResponse::nResponseID, 0);
    this
}

// Translated from 0083d420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): the response's size in the older save format as a 16-bit value: the
/// low bytes of the two string lengths ([`fn_0083d890`] on `cText` and `cVoice`) plus one
/// each, plus 4 and 4 (`AX`).
pub fn fn_0083d420(e: &mut Engine, this: Ptr<DialogueResponse>) -> u16 {
    let mut size = 0u16;
    for string in [0u32, 0x10] {
        let length = fn_0083d890(e, Ptr::new(this.addr() + string)) as u8;
        size = size.wrapping_add(u16::from(length)).wrapping_add(1);
    }
    size.wrapping_add(4).wrapping_add(4)
}

// Translated from 0083d490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): saves the response the older way through the `TESSaveLoadGame` at
/// `011de45c`: for `cText` and `cVoice` a length byte (the low byte of [`fn_0083d890`]) and,
/// when not 0, that many characters (`00559450`, the string's characters); then `eEmotion` and
/// `uiEmotion` (4 bytes each).
pub fn fn_0083d490(e: &mut Engine, this: Ptr<DialogueResponse>) {
    e.with_stack(4, |e, length_at| {
        for string in [0u32, 0x10] {
            let string = this.addr() + string;
            let length = fn_0083d890(e, Ptr::new(string)) as u8;
            e.mem.set_u8(length_at.addr(), length);
            let saver = e.global::<u32>(SAVE_LOAD_GAME_011DE45C);
            e.call(SAVE_WRITE_008579B0, &args![saver, length_at, 1u32]);
            let length = e.mem.u8(length_at.addr());
            if length != 0 {
                let chars = e.call(RECORD_WORD_0, &args![string]).u32();
                let saver = e.global::<u32>(SAVE_LOAD_GAME_011DE45C);
                e.call(SAVE_WRITE_008579B0, &args![saver, chars, u32::from(length)]);
            }
        }
    });
    for offset in [8u32, 0xc] {
        let saver = e.global::<u32>(SAVE_LOAD_GAME_011DE45C);
        e.call(
            SAVE_WRITE_008579B0,
            &args![saver, this.addr() + offset, 4u32],
        );
    }
}

// Translated from 0083d550 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083d550 - ends in the /GS cookie check (00ec408c), translated as a call
/// Unnamed (`thiscall`): loads the response the older way through the `TESSaveLoadGame` at
/// `011de45c`: for `cText` and `cVoice` a length byte and, when not 0, that many characters into
/// a zeroed 0x104-byte stack buffer (`memset`), the string set from it (`004037f0` with 0); a 0
/// length sets the empty string (`01011584`). Then `eEmotion` and `uiEmotion` (4 bytes each).
pub fn fn_0083d550(e: &mut Engine, this: Ptr<DialogueResponse>) {
    let cookie = e.global::<u32>(SECURITY_COOKIE_011C16BC);
    e.with_stack(0x210, |e, frame| {
        let lengths = frame.addr();
        let buffers = [frame.addr() + 4, frame.addr() + 0x108];
        for (k, string) in [0u32, 0x10].into_iter().enumerate() {
            let length_at = lengths + k as u32;
            let loader = e.global::<u32>(SAVE_LOAD_GAME_011DE45C);
            e.call(LOAD_READ_008579E0, &args![loader, length_at, 1u32]);
            let length = e.mem.u8(length_at);
            let target = this.addr() + string;
            if length != 0 {
                let buffer = buffers[k];
                e.call(MEMSET_00403D30, &args![buffer, 0u32, 0x104u32]);
                let length = e.mem.u8(length_at);
                let loader = e.global::<u32>(SAVE_LOAD_GAME_011DE45C);
                e.call(
                    LOAD_READ_008579E0,
                    &args![loader, buffer, u32::from(length)],
                );
                e.call(STRING_SET_TEXT, &args![target, buffer, 0u32]);
            } else {
                e.call(STRING_SET_TEXT, &args![target, EMPTY_STRING_01011584, 0u32]);
            }
        }
    });
    for offset in [8u32, 0xc] {
        let loader = e.global::<u32>(SAVE_LOAD_GAME_011DE45C);
        e.call(
            LOAD_READ_008579E0,
            &args![loader, this.addr() + offset, 4u32],
        );
    }
    e.call(SECURITY_CHECK_COOKIE, &args![cookie]);
}

/// `__RTDynamicCast(004839c0(form), 0, 01183028, target, 0)`.
fn cast_form(e: &mut Engine, form: u32, target: u32) -> u32 {
    let object = e.call(CAST_SOURCE_004839C0, &args![form]).u32();
    e.call(
        DYNAMIC_CAST_00EC43FB,
        &args![object, 0u32, FORM_TYPE_01183028, target, 0u32],
    )
    .u32()
}

// Translated from 0083d740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083d740 - ends in the /GS cookie check (00ec408c), translated as a call
/// `DialogueResponse::LoadGame` (engine map; `thiscall`, `RET 4`: the load buffer): `cText` and
/// `cVoice` from `LoadString` (into one 0x3ec-byte stack buffer, then `004037f0` with 0),
/// `eEmotion` and `uiEmotion` (4 bytes each) and `bUseEmotion` (1 byte), then the two idles and
/// the voice sound from `LoadFormID`, each cast to its type.
pub fn dialogue_response_load_game(e: &mut Engine, this: Ptr<DialogueResponse>, buffer: u32) {
    let cookie = e.global::<u32>(SECURITY_COOKIE_011C16BC);
    let base = this.addr();
    e.with_stack(0x3ec, |e, text| {
        for string in [0u32, 0x10] {
            e.call(LOAD_STRING, &args![buffer, text]);
            e.call(STRING_SET_TEXT, &args![base + string, text, 0u32]);
        }
    });
    for (offset, size) in [(8u32, 4u32), (0xc, 4), (0x24, 1)] {
        e.call(LOAD_DATA, &args![buffer, base + offset, size]);
    }
    for (field, target) in [
        (DialogueResponse::pSpeakerIdle, IDLE_TYPE_01186A18),
        (DialogueResponse::pListenIdle, IDLE_TYPE_01186A18),
        (DialogueResponse::pVoiceSound, SOUND_TYPE_011832FC),
    ] {
        let form = e.call(LOAD_FORM_ID, &args![buffer]).u32();
        let cast = cast_form(e, form, target);
        e.set(this, field, cast);
    }
    e.call(SECURITY_CHECK_COOKIE, &args![cookie]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0083_d890, fn_0083d890(Ptr) -> u32),
        entry!(
            0x0083_d1e0,
            dialogue_response_default_constructor(Ptr<DialogueResponse>) -> Ptr<DialogueResponse>
        ),
        entry!(0x0083_d420, fn_0083d420(Ptr<DialogueResponse>) -> u16),
        entry!(0x0083_d490, fn_0083d490(Ptr<DialogueResponse>)),
        entry!(0x0083_d550, fn_0083d550(Ptr<DialogueResponse>)),
        entry!(
            0x0083_d740,
            dialogue_response_load_game(Ptr<DialogueResponse>, u32)
        ),
        entry!(
            0x0083_d280,
            dialogue_response_constructor(
                Ptr<DialogueResponse>,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> Ptr<DialogueResponse>
        ),
        entry!(
            0x0083_d690,
            dialogue_response_save_game(Ptr<DialogueResponse>, u32)
        ),
    ]
}

#[cfg(test)]
mod response_tests {
    use super::*;

    fn called(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == target)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [
            STRING_CONSTRUCTOR,
            STRING_SET_TEXT,
            STRING_SET,
            STRING_DESTRUCTOR,
            VOICE_FILE_NAME_00616F40,
            SAVE_STRING,
            SAVE_DATA,
            SAVE_FORM_ID,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(GET_RESPONSE_TEXT, |_, _| 0x7e47u32.into_ret());
        e.register(DISPLAY_STRING_0083D8B0, |_, a| a[1].into_ret());
        e.register(RECORD_WORD_0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(RECORD_WORD_4, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(RECORD_SPEAKER_IDLE_007AF430, |_, _| 0x1801u32.into_ret());
        e.register(RECORD_LISTEN_IDLE_0059BB30, |_, _| 0x1c01u32.into_ret());
        e.register(RECORD_SOUND_0044EDB0, |_, _| 0x2001u32.into_ret());
        e.register(RECORD_USE_EMOTION_00843150, |_, _| 0x100u32.into_ret());
        e.register(RECORD_ID_006174B0, |_, _| 0x2801u32.into_ret());
        e.register(GET_OWNER_TOPIC, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn constructor_without_a_record_clears_the_fields() {
        let mut e = engine();
        let this = e.mem.alloc(0x2c);
        e.mem.write(this, &[0xee; 0x2c]);
        assert_eq!(e.call(0x0083_d280, &[this, 1, 2, 3, 4, 0]).u32(), this);
        assert_eq!(
            called(&e, STRING_CONSTRUCTOR),
            vec![vec![this], vec![this + 0x10]]
        );
        for offset in [8u32, 0xc, 0x18, 0x1c, 0x20, 0x28] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert!(called(&e, GET_RESPONSE_TEXT).is_empty());
    }

    #[test]
    fn constructor_reads_the_record() {
        let mut e = engine();
        let record = e.mem.alloc(8);
        e.mem.set_u32(record, 5);
        e.mem.set_u32(record + 4, 50);
        let this = e.mem.alloc(0x2c);
        e.call(0x0083_d280, &[this, 0x51, 0x52, 0x53, 0x54, record]);
        let set_text = called(&e, STRING_SET_TEXT);
        assert_eq!(set_text[0][1..], [0x7e47, 0]);
        let raw = set_text[0][0];
        let shown = called(&e, DISPLAY_STRING_0083D8B0);
        assert_eq!(shown[0][0], raw);
        assert_eq!(called(&e, STRING_SET), vec![vec![this, shown[0][1]]]);
        assert_eq!(
            called(&e, STRING_DESTRUCTOR),
            vec![vec![shown[0][1]], vec![raw]]
        );
        assert_eq!(
            called(&e, VOICE_FILE_NAME_00616F40),
            vec![vec![record, 0x54, 0x51, 0x54, 0x53, this + 0x10]]
        );
        assert_eq!(called(&e, GET_OWNER_TOPIC), vec![vec![0x53]]);
        assert_eq!(e.mem.u32(this + 8), 5);
        assert_eq!(e.mem.u32(this + 0xc), 50);
        assert_eq!(e.mem.u32(this + 0x18), 0x1801);
        assert_eq!(e.mem.u32(this + 0x1c), 0x1c01);
        assert_eq!(e.mem.u32(this + 0x20), 0x2001);
        assert_eq!(e.mem.u8(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0x2801);
    }

    #[test]
    fn save_game_writes_strings_emotion_and_forms() {
        let mut e = engine();
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this, 0x7e47);
        e.mem.set_u32(this + 0x10, 0x5010);
        e.mem.set_u32(this + 0x18, 0x18);
        e.mem.set_u32(this + 0x1c, 0x1c);
        e.mem.set_u32(this + 0x20, 0x20);
        e.call(0x0083_d690, &[this, 0xb0]);
        assert_eq!(
            called(&e, SAVE_STRING),
            vec![vec![0xb0, 0x7e47, 0], vec![0xb0, 0x5010, 0]]
        );
        assert_eq!(
            called(&e, SAVE_DATA),
            vec![
                vec![0xb0, this + 8, 4, 0],
                vec![0xb0, this + 0xc, 4, 0],
                vec![0xb0, this + 0x24, 1, 0]
            ]
        );
        let forms: Vec<u32> = called(&e, SAVE_FORM_ID).iter().map(|w| w[1]).collect();
        assert_eq!(forms, vec![0x18, 0x1c, 0x20]);
    }
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_0083d890_returns_the_callees_word() {
        let mut e = Engine::new();
        e.register(0x0040_48e0, |_, _| Ret {
            eax: 0x1234_5678,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0083_d890, &args![0x0000_5000u32]).u32(),
            0x1234_5678
        );
        assert_eq!(calls_to(&e, 0x0040_48e0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod n089_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// The string length `004048e0` answers the word at the string's `+4` (upper bytes set).
    fn length_double(e: &mut Engine) {
        e.register(0x0040_48e0, |e, a| {
            (0xab00 | e.mem.u32(a[0] + 4)).into_ret()
        });
    }

    #[test]
    fn n089_0083d1e0_constructs_an_empty_response() {
        let mut e = Engine::new();
        e.register(STRING_CONSTRUCTOR, quiet);
        let this = e.mem.alloc(0x2c);
        e.mem.write(this, &[0xee; 0x2c]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_d1e0, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&e, STRING_CONSTRUCTOR),
            vec![vec![this], vec![this + 0x10]]
        );
        for offset in [8u32, 0xc, 0x18, 0x1c, 0x20, 0x28] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u8(this + 0x24), 0xee, "bUseEmotion untouched");
    }

    #[test]
    fn n089_0083d420_sizes_the_response() {
        let mut e = Engine::new();
        length_double(&mut e);
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + 4, 5);
        e.mem.set_u32(this + 0x14, 0x07);
        // Only the low byte of each length counts (0xab05 -> 5).
        assert_eq!(e.call(0x0083_d420, &args![this]).u16(), 5 + 1 + 7 + 1 + 8);
    }

    #[test]
    fn n089_0083d490_saves_the_response_the_old_way() {
        let mut e = Engine::new();
        length_double(&mut e);
        e.map(SAVE_LOAD_GAME_011DE45C, 4);
        e.set_global(SAVE_LOAD_GAME_011DE45C, 0x5a5a_0000u32);
        e.register(RECORD_WORD_0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SAVE_WRITE_008579B0, quiet);
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this, 0xc4a5);
        e.mem.set_u32(this + 4, 3);
        e.mem.set_u32(this + 0x14, 0);
        e.call_log = Some(vec![]);
        e.call(0x0083_d490, &args![this]);
        let writes = calls_to(&e, SAVE_WRITE_008579B0);
        assert_eq!(writes.len(), 5);
        assert!(writes.iter().all(|w| w[0] == 0x5a5a_0000));
        assert_eq!(writes[0][2], 1);
        assert_eq!(writes[1][1..], [0xc4a5, 3]);
        assert_eq!(writes[2][2], 1, "the voice's length, 0: no characters");
        assert_eq!(writes[3][1..], [this + 8, 4]);
        assert_eq!(writes[4][1..], [this + 0xc, 4]);
        assert_eq!(calls_to(&e, RECORD_WORD_0), vec![vec![this]]);
    }

    #[test]
    fn n089_0083d550_loads_the_response_the_old_way() {
        let mut e = Engine::new();
        e.map(SAVE_LOAD_GAME_011DE45C, 4);
        e.map(SECURITY_COOKIE_011C16BC, 4);
        e.set_global(SECURITY_COOKIE_011C16BC, 0xc00c_1e00u32);
        e.register(MEMSET_00403D30, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            Ret::default()
        });
        e.register(STRING_SET_TEXT, quiet);
        e.register(SECURITY_CHECK_COOKIE, quiet);
        // A length byte of 2 for the text and 0 for the voice; the text reads "hi".
        e.register(LOAD_READ_008579E0, |e, a| {
            let reads = e.mem.u32(0x0bad_0000);
            e.mem.set_u32(0x0bad_0000, reads + 1);
            match reads {
                0 => e.mem.set_u8(a[1], 2),
                1 => e.mem.write(a[1], b"hi"),
                2 => e.mem.set_u8(a[1], 0),
                _ => e.mem.set_u32(a[1], 0x40 + reads),
            }
            Ret::default()
        });
        e.map(0x0bad_0000, 4);
        let this = e.mem.alloc(0x2c);
        e.call_log = Some(vec![]);
        e.call(0x0083_d550, &args![this]);
        let sets = calls_to(&e, STRING_SET_TEXT);
        assert_eq!(sets.len(), 2);
        assert_eq!(sets[0][0], this);
        assert_eq!(e.mem.cstr(sets[0][1]), b"hi".to_vec());
        assert_eq!(sets[1], vec![this + 0x10, EMPTY_STRING_01011584, 0]);
        assert_eq!(
            calls_to(&e, MEMSET_00403D30),
            vec![vec![sets[0][1], 0, 0x104]]
        );
        assert_eq!(e.mem.u32(this + 8), 0x43);
        assert_eq!(e.mem.u32(this + 0xc), 0x44);
        assert_eq!(calls_to(&e, SECURITY_CHECK_COOKIE), vec![vec![0xc00c_1e00]]);
    }

    #[test]
    fn n089_0083d740_loads_the_response() {
        let mut e = Engine::new();
        e.map(SECURITY_COOKIE_011C16BC, 4);
        e.register(LOAD_STRING, quiet);
        e.register(STRING_SET_TEXT, quiet);
        e.register(LOAD_DATA, quiet);
        e.register(SECURITY_CHECK_COOKIE, quiet);
        e.register(LOAD_FORM_ID, |_, _| 0x700u32.into_ret());
        e.register(CAST_SOURCE_004839C0, |_, a| (a[0] + 0x10).into_ret());
        e.register(DYNAMIC_CAST_00EC43FB, |_, a| (a[0] + a[3]).into_ret());
        let this = e.mem.alloc(0x2c);
        e.call_log = Some(vec![]);
        e.call(0x0083_d740, &args![this, 0xb0ffu32]);
        let loads = calls_to(&e, LOAD_STRING);
        assert_eq!(loads, vec![vec![0xb0ff, loads[0][1]]; 2]);
        assert_eq!(
            calls_to(&e, STRING_SET_TEXT),
            vec![
                vec![this, loads[0][1], 0],
                vec![this + 0x10, loads[0][1], 0]
            ]
        );
        assert_eq!(
            calls_to(&e, LOAD_DATA),
            vec![
                vec![0xb0ff, this + 8, 4],
                vec![0xb0ff, this + 0xc, 4],
                vec![0xb0ff, this + 0x24, 1]
            ]
        );
        assert_eq!(e.mem.u32(this + 0x18), 0x710 + IDLE_TYPE_01186A18);
        assert_eq!(e.mem.u32(this + 0x1c), 0x710 + IDLE_TYPE_01186A18);
        assert_eq!(e.mem.u32(this + 0x20), 0x710 + SOUND_TYPE_011832FC);
        assert_eq!(calls_to(&e, SECURITY_CHECK_COOKIE).len(), 1);
    }
}
