//! `fallout shared/tesskill.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The chunk codes of a skill record (the four characters, first byte lowest).
const CHUNK_INDX: u32 = 0x5844_4e49;
const CHUNK_ICON: u32 = 0x4e4f_4349;
const CHUNK_ANAM: u32 = 0x4d41_4e41;
const CHUNK_JNAM: u32 = 0x4d41_4e4a;
const CHUNK_ENAM: u32 = 0x4d41_4e45;
const CHUNK_MNAM: u32 = 0x4d41_4e4d;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_DESC: u32 = 0x4353_4544;
const CHUNK_OBND: u32 = 0x444e_424f;
/// `TESForm::StartForm` (`004855a0`), `TESForm::CloseForm` (`00485680`), the one-word chunk
/// `00485910` (`cdecl`: chunk, word) and `TESForm::__SaveData` (`00485e20`, `RET 8`: data,
/// size), engine map names.
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
const ADD_WORD_CHUNK: u32 = 0x0048_5910;
const SAVE_DATA: u32 = 0x0048_5e20;
/// `TESDescription::Save` (`00482cb0`), `TESDescription::SaveDescriptionData` (`00482cd0`,
/// `cdecl`: description, chunk) and `TESDescription::LoadDescriptionData` (`00482d50`,
/// `cdecl`: description, file), engine map names.
const DESCRIPTION_SAVE: u32 = 0x0048_2cb0;
const SAVE_DESCRIPTION_DATA: u32 = 0x0048_2cd0;
const LOAD_DESCRIPTION_DATA: u32 = 0x0048_2d50;
/// `TESTexture::Save` (`0048e370`, `RET 4`: the chunk) and `TESTexture::LoadTextureChunk`
/// (`0048e3d0`, `cdecl`: texture, file).
const TEXTURE_SAVE: u32 = 0x0048_e370;
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
/// `00401500` (`cdecl`, no words): whether saved data has its byte order swapped;
/// `00401680` (`ECX` = file): whether the file's data needs its byte order swapped; the 32-bit
/// swap `00401080` (`cdecl`: address, flag byte).
const SWAPS_BYTE_ORDER: u32 = 0x0040_1500;
const FILE_NEEDS_SWAP: u32 = 0x0040_1680;
const SWAP_U32: u32 = 0x0040_1080;
/// The record reading of `TESFile`: `GetTESForm` (`00472660`), `TESForm::LoadForm`
/// (`00485110`, `RET 4`), `GetTESChunk` (`004726b0`), the chunk size `00401660`,
/// `GetChunkData_ov3` (`00472890`, `RET 8`: buffer, size), `TESForm::LoadData` (`004861f0`,
/// `RET 0xC`: file, data, size) and the step to the next chunk `004726f0`.
const GET_TES_FORM: u32 = 0x0047_2660;
const LOAD_FORM: u32 = 0x0048_5110;
const GET_TES_CHUNK: u32 = 0x0047_26b0;
const CHUNK_SIZE: u32 = 0x0040_1660;
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const LOAD_DATA: u32 = 0x0048_61f0;
const NEXT_CHUNK: u32 = 0x0047_26f0;

/// When saved data is byte-swapped (`00401500`): `006180a0` on the skill data (+0x2C).
fn swap_saved_data(e: &mut Engine, base: u32) {
    if e.call(SWAPS_BYTE_ORDER, &[]).bool() {
        fn_006180a0(e, Ptr::new(base.wrapping_add(0x2c)));
    }
}

// Translated from 00617fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSkill::Save` (engine map; `ECX`, plain `RET`): writes the skill record: `INDX` (the
/// word at +0x2C), the description (+0x18), the icon (`ICON`, +0x20), the 0x14-byte data at
/// +0x2C (`TESForm::__SaveData`, byte-swapped by `006180a0` before and after when `00401500`
/// says so), and the four descriptions `ANAM` (+0x40), `JNAM` (+0x48), `ENAM` (+0x50) and
/// `MNAM` (+0x58).
pub fn tes_skill_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    let index = e.mem.u32(base.wrapping_add(0x2c));
    e.call(ADD_WORD_CHUNK, &args![CHUNK_INDX, index]);
    e.call(DESCRIPTION_SAVE, &args![base.wrapping_add(0x18)]);
    e.call(TEXTURE_SAVE, &args![base.wrapping_add(0x20), CHUNK_ICON]);
    swap_saved_data(e, base);
    e.call(SAVE_DATA, &args![this, base.wrapping_add(0x2c), 0x14u32]);
    swap_saved_data(e, base);
    for (offset, chunk) in [
        (0x40u32, CHUNK_ANAM),
        (0x48, CHUNK_JNAM),
        (0x50, CHUNK_ENAM),
        (0x58, CHUNK_MNAM),
    ] {
        e.call(
            SAVE_DESCRIPTION_DATA,
            &args![base.wrapping_add(offset), chunk],
        );
    }
    e.call(CLOSE_FORM, &args![this]);
}

// Translated from 006180a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX` = the 0x14-byte skill data): swaps the byte order of its five
/// words (`00401080` with flag 0).
pub fn fn_006180a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(SWAP_U32, &args![base, 0u32]);
    e.call(SWAP_U32, &args![base.wrapping_add(4), 0u32]);
    e.call(SWAP_U32, &args![base.wrapping_add(8), 0u32]);
    let mut index = 0i32;
    while index < 2 {
        let at = base
            .wrapping_add((index as u32).wrapping_mul(4))
            .wrapping_add(0xc);
        e.call(SWAP_U32, &args![at, 0u32]);
        index += 1;
    }
}

/// A component address the loader takes only from a non-null form (`this + offset`, or 0).
fn component(this: u32, offset: u32) -> u32 {
    if this != 0 {
        this.wrapping_add(offset)
    } else {
        0
    }
}

// Translated from 00618130 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00618130 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESSkill::Load` (engine map, `RET 4`: the file): false unless the record is of form type
/// 0xF. Loads the form (`TESForm::LoadForm`) and reads each chunk: `EDID` into a stack
/// buffer of the chunk's size (`GetChunkData_ov3` with 0x200) given to vtable slot 0x134,
/// `OBND` through slot 0xE0, `DATA` (0x14 bytes at +0x2C, `TESForm::LoadData`, byte-swapped by
/// `006180a0` when the file needs it), `DESC` (+0x18), `ICON` (+0x20) and the descriptions
/// `ANAM`, `JNAM`, `ENAM`, `MNAM` (+0x40, +0x48, +0x50, +0x58); others are skipped. True. The
/// buffer `_alloca_probe_16` (`00ec5ec0`) takes from the stack is a stack block of the
/// chunk's size rounded up to 16 bytes; the stack cookie check is not translated.
pub fn tes_skill_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u32() != 0xf {
        return false;
    }
    e.call(LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_ANAM | CHUNK_JNAM | CHUNK_ENAM | CHUNK_MNAM => {
                let offset = match chunk {
                    CHUNK_ANAM => 0x40,
                    CHUNK_JNAM => 0x48,
                    CHUNK_ENAM => 0x50,
                    _ => 0x58,
                };
                e.call(
                    LOAD_DESCRIPTION_DATA,
                    &args![base.wrapping_add(offset), file],
                );
            }
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, 0x200u32]);
                    e.vcall(base, 0x134, &args![buffer]);
                });
            }
            CHUNK_DATA => {
                e.call(
                    LOAD_DATA,
                    &args![this, file, base.wrapping_add(0x2c), 0x14u32],
                );
            }
            CHUNK_DESC => {
                e.call(LOAD_DESCRIPTION_DATA, &args![component(base, 0x18), file]);
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_ICON => {
                e.call(LOAD_TEXTURE_CHUNK, &args![component(base, 0x20), file]);
            }
            _ => {}
        }
        if chunk == CHUNK_DATA && e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
            fn_006180a0(e, Ptr::new(base.wrapping_add(0x2c)));
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_7fc0, tes_skill_save(Ptr)),
        entry!(0x0061_80a0, fn_006180a0(Ptr)),
        entry!(0x0061_8130, tes_skill_load(Ptr, Ptr) -> bool),
    ]
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

    #[test]
    fn save_writes_the_skill_record() {
        let mut e = Engine::new();
        for a in [
            START_FORM,
            CLOSE_FORM,
            ADD_WORD_CHUNK,
            SAVE_DATA,
            DESCRIPTION_SAVE,
            SAVE_DESCRIPTION_DATA,
            TEXTURE_SAVE,
            SWAP_U32,
        ] {
            e.register(a, ok);
        }
        e.register(SWAPS_BYTE_ORDER, |_, _| ret(1));
        let s = e.mem.alloc(0x60);
        e.mem.set_u32(s + 0x2c, 9);
        e.call_log = Some(vec![]);
        e.call(0x0061_7fc0, &[s]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, ADD_WORD_CHUNK), vec![vec![CHUNK_INDX, 9]]);
        assert_eq!(only(&log, TEXTURE_SAVE), vec![vec![s + 0x20, CHUNK_ICON]]);
        assert_eq!(only(&log, SAVE_DATA), vec![vec![s, s + 0x2c, 0x14]]);
        assert_eq!(only(&log, SWAP_U32).len(), 10);
        assert_eq!(
            only(&log, SAVE_DESCRIPTION_DATA),
            vec![
                vec![s + 0x40, CHUNK_ANAM],
                vec![s + 0x48, CHUNK_JNAM],
                vec![s + 0x50, CHUNK_ENAM],
                vec![s + 0x58, CHUNK_MNAM]
            ]
        );
    }

    #[test]
    fn data_swap_covers_five_words() {
        let mut e = Engine::new();
        e.register(SWAP_U32, ok);
        e.call_log = Some(vec![]);
        e.call(0x0061_80a0, &[0x1000]);
        let log = e.call_log.take().unwrap();
        let at: Vec<u32> = only(&log, SWAP_U32).iter().map(|w| w[0]).collect();
        assert_eq!(at, vec![0x1000, 0x1004, 0x1008, 0x100c, 0x1010]);
    }

    #[test]
    fn load_reads_each_chunk() {
        let mut e = Engine::new();
        let chunks = vec![
            CHUNK_EDID, CHUNK_DATA, CHUNK_DESC, CHUNK_ICON, CHUNK_ANAM, CHUNK_MNAM, CHUNK_OBND,
            0x1234,
        ];
        let mut remaining = chunks.clone().into_iter();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(remaining.next().unwrap_or(0))
        });
        e.register(GET_TES_FORM, |_, _| ret(0xf));
        e.register(CHUNK_SIZE, |_, _| ret(5));
        for a in [
            LOAD_FORM,
            GET_CHUNK_DATA_SIZED,
            LOAD_DATA,
            LOAD_DESCRIPTION_DATA,
            LOAD_TEXTURE_CHUNK,
            SWAP_U32,
        ] {
            e.register(a, ok);
        }
        e.register(FILE_NEEDS_SWAP, |_, _| ret(1));
        e.register(NEXT_CHUNK, |_, _| ret(1));
        e.register(0x0050_0000, ok);
        let s = e.mem.alloc(0x60);
        let vt = e.mem.alloc(0x140);
        e.mem.set_u32(s, vt);
        e.mem.set_u32(vt + 0x134, 0x0050_0000);
        e.mem.set_u32(vt + 0xe0, 0x0050_0000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0061_8130, &[s, 0x7000]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, LOAD_FORM), vec![vec![s, 0x7000]]);
        assert_eq!(only(&log, GET_CHUNK_DATA_SIZED)[0][2], 0x200);
        assert_eq!(only(&log, LOAD_DATA), vec![vec![s, 0x7000, s + 0x2c, 0x14]]);
        assert_eq!(only(&log, SWAP_U32).len(), 5);
        assert_eq!(
            only(&log, LOAD_DESCRIPTION_DATA),
            vec![
                vec![s + 0x18, 0x7000],
                vec![s + 0x40, 0x7000],
                vec![s + 0x58, 0x7000]
            ]
        );
        assert_eq!(only(&log, LOAD_TEXTURE_CHUNK), vec![vec![s + 0x20, 0x7000]]);
        assert_eq!(only(&log, 0x0050_0000).len(), 2);
        e.register(GET_TES_FORM, |_, _| ret(0x10));
        assert!(!e.call(0x0061_8130, &[s, 0x7000]).bool());
    }
}
