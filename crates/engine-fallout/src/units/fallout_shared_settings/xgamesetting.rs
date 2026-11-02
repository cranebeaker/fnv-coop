//! `fallout shared/settings/xgamesetting.cpp` (Xbox PDB source unit), subsystem `fallout shared/settings`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f8960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the class is unconfirmed). `__fastcall` getter: the 32-bit word at
/// `+0x108` of `this`.
pub fn fn_004f8960(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x108))
}

/// `0096ad30` (`thiscall`, `RET 4`: the new size): grows the array.
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
/// `0096ae90` (`thiscall`, `RET 8`: the index and the element): stores the
/// element.
const ARRAY_SET_AT: u32 = 0x0096_ae90;

// Translated from 00470000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<char *, NiTMallocInterface<char *>>::SetAtGrow` (engine map name),
/// `thiscall`, `RET 8` (index, element): when the index is not below the
/// 16-bit size at `+8`, grows the array to the index plus the 16-bit growth
/// step at `+0xe` (`0096ad30`); then stores the element (`0096ae90`). Answers
/// the index.
pub fn ni_t_array_set_at_grow(e: &mut Engine, this: Ptr, index: u32, element: u32) -> u32 {
    let size = u32::from(e.mem.u16(this.addr() + 8));
    if index >= size {
        let grow = u32::from(e.mem.u16(this.addr() + 0xe));
        e.call(ARRAY_SET_SIZE, &args![this, grow.wrapping_add(index)]);
    }
    e.call(ARRAY_SET_AT, &args![this, index, element]);
    index
}

/// `004f8ac0` (`thiscall`, `RET 4`): called with 0 before the file is kept.
const COLLECTION_RESET_004F8AC0: u32 = 0x004f_8ac0;

// Translated from 004f8a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: a file): runs `004f8ac0`
/// with 0, then keeps the file at `+0x108` (where `004f8960` reads it).
pub fn fn_004f8a60(e: &mut Engine, this: Ptr, file: u32) {
    e.call(COLLECTION_RESET_004F8AC0, &args![this, 0u32]);
    e.mem.set_u32(this.addr() + 0x108, file);
}

// Translated from 004f89a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GameSettingCollection::Close` (engine map name), `thiscall`, no stack
/// words: clears the file at `+0x108` and answers true.
pub fn game_setting_collection_close(e: &mut Engine, this: Ptr) -> bool {
    e.mem.set_u32(this.addr() + 0x108, 0);
    true
}

/// `00c33090` (`cdecl`, one word: the setting's name at `setting+8`): the
/// setting's type.
const SETTING_TYPE_OF_NAME: u32 = 0x00c3_3090;

// Translated from 004f8940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a setting, no stack words): the
/// setting's type, `00c33090` of the name word at `+8`.
pub fn fn_004f8940(e: &mut Engine, this: Ptr) -> u32 {
    let name = e.mem.u32(this.addr() + 8);
    e.call(SETTING_TYPE_OF_NAME, &args![name]).u32()
}

/// `TESFile` callees of `ReadSetting` (engine map names where it has them):
/// `004726f0` (`thiscall`, no stack words), `TESFile::GetTESChunk` (`004726b0`,
/// `thiscall`; the chunk's four characters), `TESFile::GetChunkData`
/// (`004727f0`, `thiscall`, `RET 4`: the destination), `00401660` (`thiscall`;
/// the chunk's size) and `TESFile::GetChunkData_ov3` (`00472890`, `thiscall`,
/// `RET 8`: the destination and 0).
const FILE_004726F0: u32 = 0x0047_26f0;
const FILE_GET_TES_CHUNK: u32 = 0x0047_26b0;
const FILE_GET_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_GET_CHUNK_DATA_OV3: u32 = 0x0047_2890;
/// `00c33170` (`thiscall` on the setting, `RET 4`: the text): stores a string
/// value.
const SETTING_SET_STRING: u32 = 0x00c3_3170;
/// The chunk type `DATA` as the file reads it (`0x41544144`).
const CHUNK_DATA: u32 = 0x4154_4144;

// Translated from 004f8860 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f8860 - the /GS stack cookie (011c16bc, __security_check_cookie 00ec408c) is not translated; the `_alloca_probe_16` stack buffer is modelled as a stack block of the chunk's size
/// `GameSettingCollection::ReadSetting` (engine map name), `thiscall`, `RET 4`
/// (the setting). Without a file (`004f8960`) answers false. Otherwise, on the
/// file (read again): `004726f0`, then the current chunk must be `DATA`; by the
/// setting's type (`004f8940`): 3 and 5 read the chunk into the setting's value
/// at `+4` (`GetChunkData`), 6 reads it into a stack buffer of the chunk's size
/// (`00401660`, `GetChunkData_ov3` with 0) and stores that text in the setting
/// (`00c33170`). Answers the read's result, false for any other type or chunk.
pub fn game_setting_collection_read_setting(e: &mut Engine, this: Ptr, setting: Ptr) -> bool {
    if fn_004f8960(e, this) == 0 {
        return false;
    }
    let file = fn_004f8960(e, this);
    e.call(FILE_004726F0, &args![file]);
    if e.call(FILE_GET_TES_CHUNK, &args![file]).u32() != CHUNK_DATA {
        return false;
    }
    match fn_004f8940(e, setting) {
        3 | 5 => e
            .call(FILE_GET_CHUNK_DATA, &args![file, setting.addr() + 4])
            .bool(),
        6 => {
            let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
            e.with_stack(size, |e, buffer| {
                let read = e
                    .call(FILE_GET_CHUNK_DATA_OV3, &args![file, buffer, 0u32])
                    .bool();
                e.call(SETTING_SET_STRING, &args![setting, buffer]);
                read
            })
        }
        _ => false,
    }
}

/// `00464f30` (`cdecl`, two words: a setting name and 0; `tesdatahandler.cpp`).
const LOOKUP_00464F30: u32 = 0x0046_4f30;
/// `004f8a30` (`thiscall`, `RET 4`): the collection's setting for what
/// `00464f30` answered.
const COLLECTION_FIND_004F8A30: u32 = 0x004f_8a30;

// Translated from 004f89c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GameSettingCollection::ReadSetting_ov2` (engine map name), `thiscall`,
/// `RET 8` (a file and a name): keeps the file (`004f8a60`), looks the setting
/// up (`00464f30(name, 0)`, then `004f8a30`) and, when found, reads it through
/// the collection's vtable slot `+0x14`; then the slot `+0x1c` runs (no
/// arguments). Answers the read's byte (false when not found).
pub fn game_setting_collection_read_setting_ov2(
    e: &mut Engine,
    this: Ptr,
    file: u32,
    name: u32,
) -> bool {
    let mut read = false;
    fn_004f8a60(e, this, file);
    let key = e.call(LOOKUP_00464F30, &args![name, 0u32]).u32();
    let setting = e.call(COLLECTION_FIND_004F8A30, &args![this, key]).u32();
    if setting != 0 {
        read = e.vcall(this.addr(), 0x14, &args![setting]).bool();
    }
    e.vcall(this.addr(), 0x1c, &args![]);
    read
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f8960, fn_004f8960(Ptr) -> u32),
        entry!(0x0047_0000, ni_t_array_set_at_grow(Ptr, u32, u32) -> u32),
        entry!(0x004f_8a60, fn_004f8a60(Ptr, u32)),
        entry!(0x004f_89a0, game_setting_collection_close(Ptr) -> bool),
        entry!(0x004f_8940, fn_004f8940(Ptr) -> u32),
        entry!(0x004f_8860, game_setting_collection_read_setting(Ptr, Ptr) -> bool),
        entry!(0x004f_89c0, game_setting_collection_read_setting_ov2(Ptr, u32, u32) -> bool),
    ]
    .into_iter()
    .chain(funcs_n086())
    .collect()
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_004f8960_answers_the_word_at_offset_0x108() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x110);
        e.mem.set_u32(this + 0x104, 0x1111_1111);
        e.mem.set_u32(this + 0x108, 0xcafe_f00d);
        e.mem.set_u32(this + 0x10c, 0x2222_2222);
        assert_eq!(e.call(0x004f_8960, &args![this]).u32(), 0xcafe_f00d);
    }
}

#[cfg(test)]
mod tests_b008 {
    use super::*;

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap()[1..].to_vec()
    }

    #[test]
    fn ni_t_array_set_at_grow_grows_only_past_the_size() {
        let mut e = Engine::new();
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u16(this + 8, 4);
        e.mem.set_u16(this + 0xe, 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_0000, &args![this, 3u32, 0x55u32]).u32(), 3);
        assert_eq!(calls(&mut e), vec![(ARRAY_SET_AT, vec![this, 3, 0x55])]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_0000, &args![this, 4u32, 0x66u32]).u32(), 4);
        assert_eq!(
            calls(&mut e),
            vec![
                (ARRAY_SET_SIZE, vec![this, 7]),
                (ARRAY_SET_AT, vec![this, 4, 0x66])
            ]
        );
    }

    #[test]
    fn fn_004f8a60_resets_and_keeps_the_file() {
        let mut e = Engine::new();
        e.register(COLLECTION_RESET_004F8AC0, |_, _| Ret::default());
        let this = e.mem.alloc(0x110);
        e.call_log = Some(vec![]);
        e.call(0x004f_8a60, &args![this, 0x7000u32]);
        assert_eq!(
            calls(&mut e),
            vec![(COLLECTION_RESET_004F8AC0, vec![this, 0])]
        );
        assert_eq!(e.mem.u32(this + 0x108), 0x7000);
    }

    #[test]
    fn game_setting_collection_close_clears_the_file() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x110);
        e.mem.set_u32(this + 0x108, 0x7000);
        assert!(e.call(0x004f_89a0, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 0x108), 0);
    }

    #[test]
    fn fn_004f8940_types_the_name() {
        let mut e = Engine::new();
        e.register(SETTING_TYPE_OF_NAME, |_, a| (a[0] & 0xf).into_ret());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0x1236);
        assert_eq!(e.call(0x004f_8940, &args![this]).u32(), 6);
    }

    /// Runs `ReadSetting` with the file `file`, the chunk `chunk` and the
    /// setting type `kind`; answers the result and the calls.
    fn read(file: u32, chunk: u32, kind: u32) -> (bool, Vec<u32>, Vec<Vec<u32>>) {
        let mut e = Engine::new();
        e.register(FILE_004726F0, |_, _| Ret::default());
        e.register_double(FILE_GET_TES_CHUNK, move |_, _| chunk.into_ret());
        e.register_double(SETTING_TYPE_OF_NAME, move |_, _| kind.into_ret());
        e.register(FILE_GET_CHUNK_DATA, |_, _| 1u32.into_ret());
        e.register(FILE_CHUNK_SIZE, |_, _| 0x20u32.into_ret());
        e.register(FILE_GET_CHUNK_DATA_OV3, |e, a| {
            e.mem.set_cstr(a[1], b"text");
            1u32.into_ret()
        });
        e.register(SETTING_SET_STRING, |e, a| {
            assert_eq!(e.mem.cstr(a[1]), b"text".to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x110);
        e.mem.set_u32(this + 0x108, file);
        let setting = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        let result = e.call(0x004f_8860, &args![this, setting]).bool();
        let log = calls(&mut e);
        let names = log.iter().map(|c| c.0).collect();
        let words = log
            .into_iter()
            .filter(|c| c.0 == FILE_GET_CHUNK_DATA || c.0 == SETTING_SET_STRING)
            .map(|c| vec![c.1[0], c.1[1].wrapping_sub(setting)])
            .collect();
        (result, names, words)
    }

    #[test]
    fn game_setting_collection_read_setting_reads_by_type() {
        let (result, names, words) = read(0x5000, CHUNK_DATA, 3);
        assert!(result);
        assert_eq!(
            names,
            vec![
                FILE_004726F0,
                FILE_GET_TES_CHUNK,
                SETTING_TYPE_OF_NAME,
                FILE_GET_CHUNK_DATA
            ]
        );
        assert_eq!(words, vec![vec![0x5000, 4]]);
        let (result, _, _) = read(0x5000, CHUNK_DATA, 5);
        assert!(result);
        let (result, names, _) = read(0x5000, CHUNK_DATA, 6);
        assert!(result);
        assert_eq!(
            names[3..].to_vec(),
            vec![FILE_CHUNK_SIZE, FILE_GET_CHUNK_DATA_OV3, SETTING_SET_STRING]
        );
    }

    #[test]
    fn game_setting_collection_read_setting_refuses_the_rest() {
        let (result, names, _) = read(0, CHUNK_DATA, 3);
        assert!(!result);
        assert!(names.is_empty());
        let (result, names, _) = read(0x5000, 0x4c4c_554e, 3);
        assert!(!result);
        assert_eq!(names, vec![FILE_004726F0, FILE_GET_TES_CHUNK]);
        let (result, names, _) = read(0x5000, CHUNK_DATA, 4);
        assert!(!result);
        assert_eq!(names.len(), 3);
    }

    #[test]
    fn game_setting_collection_read_setting_ov2_reads_a_found_setting() {
        const READ_SLOT: u32 = 0x0123_9000;
        const DONE_SLOT: u32 = 0x0123_9010;
        let mut e = Engine::new();
        e.register(COLLECTION_RESET_004F8AC0, |_, _| Ret::default());
        e.register(LOOKUP_00464F30, |_, a| (a[0] + 1).into_ret());
        e.register(COLLECTION_FIND_004F8A30, |_, a| {
            (if a[1] == 0x101 { 0x4440u32 } else { 0 }).into_ret()
        });
        e.register(READ_SLOT, |_, _| 1u32.into_ret());
        e.register(DONE_SLOT, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x14, READ_SLOT);
        e.mem.set_u32(vtable + 0x1c, DONE_SLOT);
        let this = e.mem.alloc(0x110);
        e.mem.set_u32(this, vtable);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x004f_89c0, &args![this, 0x7000u32, 0x100u32])
            .bool());
        assert_eq!(
            calls(&mut e),
            vec![
                (COLLECTION_RESET_004F8AC0, vec![this, 0]),
                (LOOKUP_00464F30, vec![0x100, 0]),
                (COLLECTION_FIND_004F8A30, vec![this, 0x101]),
                (READ_SLOT, vec![this, 0x4440]),
                (DONE_SLOT, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x108), 0x7000);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x004f_89c0, &args![this, 0x7000u32, 0x200u32])
            .bool());
        let names: Vec<u32> = calls(&mut e).into_iter().map(|c| c.0).collect();
        assert!(!names.contains(&READ_SLOT));
        assert!(names.contains(&DONE_SLOT));
    }
}

/// `NiGetViewerString` (`00a7d330`, `cdecl`: a text) and
/// `NiGetViewerString_ov8` (`00a7d510`, `cdecl`: a text and a word): a new
/// viewer string.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_OV8: u32 = 0x00a7_d510;
/// `0096a610` (`thiscall`, `RET 4`: the address of a string pointer): adds
/// the string to the viewer string list.
const STRINGS_ADD_0096A610: u32 = 0x0096_a610;
/// `SettingCollectionMap<Setting>::GetViewerStrings` (`00404fd0`, engine
/// map; `thiscall`, `RET 4`).
const SETTING_MAP_GET_VIEWER_STRINGS: u32 = 0x0040_4fd0;
/// The texts `"GameSettings"` (`010241d0`) and `"Total GameSettings"`
/// (`010241bc`).
const TEXT_GAME_SETTINGS: u32 = 0x0102_41d0;
const TEXT_TOTAL_GAME_SETTINGS: u32 = 0x0102_41bc;

// Translated from 004f8800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GameSettingCollection::GetViewerStrings` (engine map name; `thiscall`,
/// `RET 4`: the string list): adds `"GameSettings"`, the map's own viewer
/// strings (`00404fd0`), then `"Total GameSettings"` with the word that
/// answers, each new string added through a stack word holding it
/// (`0096a610`). Answers that word.
pub fn game_setting_collection_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) -> u32 {
    e.with_stack(8, |e, locals| {
        let title = e
            .call(NI_GET_VIEWER_STRING, &args![TEXT_GAME_SETTINGS])
            .u32();
        e.mem.set_u32(locals.addr(), title);
        e.call(STRINGS_ADD_0096A610, &args![strings, locals]);
        let total = e
            .call(SETTING_MAP_GET_VIEWER_STRINGS, &args![this, strings])
            .u32();
        let line = e
            .call(
                NI_GET_VIEWER_STRING_OV8,
                &args![TEXT_TOTAL_GAME_SETTINGS, total],
            )
            .u32();
        e.mem.set_u32(locals.addr() + 4, line);
        e.call(STRINGS_ADD_0096A610, &args![strings, locals.addr() + 4]);
        total
    })
}

// Translated from 004f8980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; the stack word is not
/// read): whether the file at `+0x108` (where `004f8a60` keeps it) is set.
pub fn fn_004f8980(e: &mut Engine, this: Ptr, _unused_1: u32) -> bool {
    e.mem.u32(this.addr().wrapping_add(0x108)) != 0
}

/// This unit's n086 translations, by exe address.
fn funcs_n086() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004f_8800,
            game_setting_collection_get_viewer_strings(Ptr, u32) -> u32
        ),
        entry!(0x004f_8980, fn_004f8980(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod n086_tests {
    use super::*;

    #[test]
    fn get_viewer_strings_adds_the_title_and_the_total() {
        let mut e = Engine::new();
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(NI_GET_VIEWER_STRING_OV8, |_, a| (a[0] + a[1]).into_ret());
        e.register(SETTING_MAP_GET_VIEWER_STRINGS, |_, _| 0x30u32.into_ret());
        e.register_double(STRINGS_ADD_0096A610, |e, a| {
            // Record the string the word holds in the list's first word.
            let string = e.mem.u32(a[1]);
            let n = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4 + n * 4, string);
            e.mem.set_u32(a[0], n + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let total = e.call(0x004f_8800, &args![0x7000u32, strings]).u32();
        assert_eq!(total, 0x30);
        assert_eq!(e.mem.u32(strings), 2);
        assert_eq!(e.mem.u32(strings + 4), TEXT_GAME_SETTINGS + 1);
        assert_eq!(e.mem.u32(strings + 8), TEXT_TOTAL_GAME_SETTINGS + 0x30);
        let map: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == SETTING_MAP_GET_VIEWER_STRINGS)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(map, vec![vec![0x7000, strings]]);
    }

    #[test]
    fn fn_004f8980_tests_the_file_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x110);
        assert!(!e.call(0x004f_8980, &args![this, 5u32]).bool());
        e.mem.set_u32(this + 0x108, 0x40);
        assert!(e.call(0x004f_8980, &args![this, 5u32]).bool());
    }
}
