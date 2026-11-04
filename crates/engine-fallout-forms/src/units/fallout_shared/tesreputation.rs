//! `fallout shared/tesreputation.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESFile` chunk reading (see `bgsaddonnode.rs`).
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const FILE_SWAPS_ENDIAN: u32 = 0x0040_1680;
/// `00484ab0`: the form flag setter.
const FORM_SET_FLAG: u32 = 0x0048_4ab0;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_ICON: u32 = 0x4e4f_4349;
const CHUNK_MICO: u32 = 0x4f43_494d;
const CHUNK_DATA: u32 = 0x4154_4144;

// Translated from 00616290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00616290 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESReputation::Load` (Xbox PDB): `TESForm::LoadForm`, `00484ab0(0)` and
/// each chunk: `EDID` (vtable +0x134), `OBND` (vtable +0xE0), `FULL`
/// (+0x18, `00487050`), `ICON` (+0x24, `TESTexture::LoadTextureChunk`),
/// `MICO` (+0x30, `0047a290`) and `DATA` (four bytes at +0x40,
/// `TESForm::LoadData`, swapped by `00503210` when the file asks). Returns
/// true.
pub fn tes_reputation_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    let or_null = |offset: u32| if this.is_null() { 0 } else { base + offset };
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_FULL => {
                e.call(0x0048_7050, &args![or_null(0x18), file]);
            }
            CHUNK_DATA => {
                e.call(0x0048_61f0, &args![this, file, base + 0x40, 4u32]);
                if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
                    e.call(0x0050_3210, &args![base + 0x40]);
                }
            }
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_ICON => {
                e.call(0x0048_e3d0, &args![or_null(0x24), file]);
            }
            CHUNK_MICO => {
                e.call(0x0047_a290, &args![or_null(0x30), file]);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// `TESForm::StartForm` (`004855a0`) and `TESForm::CloseForm` (`00485680`),
/// `thiscall` on the form, no stack words.
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
/// The component saves (engine map; `thiscall`): `TESFullName::Save`
/// (`00487010`), `TESTexture::Save` (`0048e370`, a tag) and
/// `BGSMessageIcon::Save` (`0047a240`, a tag).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const TEXTURE_SAVE: u32 = 0x0048_e370;
const MESSAGE_ICON_SAVE: u32 = 0x0047_a240;
/// `00401500`: the byte at `011c54ba` (whether saved words are
/// byte-swapped); `00503210` (`thiscall`): the data's byte swap.
const SAVE_SWAPS_BYTES: u32 = 0x0040_1500;
const SWAP_DATA: u32 = 0x0050_3210;
/// `TESForm::__SaveData` (`00485e20`, `thiscall`, `RET 8`: data, size).
const SAVE_DATA: u32 = 0x0048_5e20;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the type
/// descriptors `TESForm` (`01183028`) and `TESReputation` (`011864e4`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_REPUTATION: u32 = 0x0118_64e4;
/// `TESForm::CopyAllComponents` (`004851b0`, `RET 4`) and
/// `TESForm::CompareAllComponents` (`00485270`, `RET 4`).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `memcpy(dest, source, count)` (`00401460`, cdecl), `_memcmp`
/// (`00ec4835`, cdecl) and the data's address (`0087eaa0`: `this + 0x40`).
const MEMCPY: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;
const DATA_ADDRESS: u32 = 0x0087_eaa0;
/// The `TESForm` save-game members the reputation's extend (`thiscall`):
/// the size (`00484bf0`, `RET 4`), `TESForm::LoadGame` (`00484c50`,
/// `RET 8`), the save (`00484d60`, `RET 4`) and the load (`00484da0`,
/// `RET 4`); the buffers' raw save (`00865e50`, `RET 0xc`: address, size, 0)
/// and load (`00864980`, `RET 8`: address, size).
const FORM_SAVE_SIZE: u32 = 0x0048_4bf0;
const FORM_LOAD_GAME: u32 = 0x0048_4c50;
const FORM_SAVE_GAME: u32 = 0x0048_4d60;
const FORM_LOAD_GAME_BUFFER: u32 = 0x0048_4da0;
const BUFFER_SAVE_BYTES: u32 = 0x0086_5e50;
const BUFFER_LOAD_BYTES: u32 = 0x0086_4980;
/// The icon's text: its length (`0048cee0`) and its text (`00408da0`); the
/// empty string at `01011584`.
const STRING_LENGTH: u32 = 0x0048_cee0;
const STRING_TEXT: u32 = 0x0040_8da0;
const EMPTY_STRING: u32 = 0x0101_1584;
/// The reputation's level (`00616950`, `RET 4`: 1 for the positive scale,
/// 0 for the negative; a signed byte) and a string setting's text
/// (`00403df0`, `thiscall` on the setting).
const REPUTATION_LEVEL: u32 = 0x0061_6950;
const SETTING_STRING: u32 = 0x0040_3df0;
/// The sixteen title settings `GetReputationTitle` picks from, row by row
/// (the negative level picks the row, the positive one the column).
const REPUTATION_TITLES: [u32; 16] = [
    0x011c_bce4,
    0x011c_bd28,
    0x011c_bbb0,
    0x011c_bc08,
    0x011c_bd74,
    0x011c_bbf0,
    0x011c_bc60,
    0x011c_bb2c,
    0x011c_bc40,
    0x011c_ba88,
    0x011c_bb58,
    0x011c_bbd8,
    0x011c_bcfc,
    0x011c_bd84,
    0x011c_bb04,
    0x011c_ba0c,
];

// Translated from 00616470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESReputation::Save` (engine map; no stack words): `StartForm`, the
/// full name (+0x18), the icon (+0x24, `ICON`), the message icon (+0x30,
/// `MICO`), the 4 bytes of data at +0x40 (`__SaveData`; `00503210` before
/// and after when `00401500` says so), `CloseForm`.
pub fn tes_reputation_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![base + 0x18]);
    e.call(TEXTURE_SAVE, &args![base + 0x24, CHUNK_ICON]);
    e.call(MESSAGE_ICON_SAVE, &args![base + 0x30, CHUNK_MICO]);
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        e.call(SWAP_DATA, &args![base + 0x40]);
    }
    e.call(SAVE_DATA, &args![this, base + 0x40, 4u32]);
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        e.call(SWAP_DATA, &args![base + 0x40]);
    }
    e.call(CLOSE_FORM, &args![this]);
}

/// `other` as a `TESReputation` (`__RTDynamicCast` from `TESForm`), or 0.
fn as_reputation(e: &mut Engine, other: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![other, 0u32, TYPE_TES_FORM, TYPE_TES_REPUTATION, 0u32],
    )
    .u32()
}

// Translated from 00616500 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reputation's copy (engine map: no name; `RET 4`): when `other` is a
/// `TESReputation`, copies the floats at +0x48 and +0x44 (`FLD`/`FSTP`), the
/// 4 bytes of data at +0x40 (`memcpy`), then all components
/// (`TESForm::CopyAllComponents` with `other`).
pub fn fn_00616500(e: &mut Engine, this: Ptr, other: u32) {
    let source = as_reputation(e, other);
    if source == 0 {
        return;
    }
    let value = x87_quiet_bits(e.mem.u32(source + 0x48));
    e.mem.set_u32(this.addr() + 0x48, value);
    let value = x87_quiet_bits(e.mem.u32(source + 0x44));
    e.mem.set_u32(this.addr() + 0x44, value);
    e.call(MEMCPY, &args![this.addr() + 0x40, source + 0x40, 4u32]);
    e.call(COPY_ALL_COMPONENTS, &args![this, other]);
}

// Translated from 00616570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reputation's compare (engine map: no name; `RET 4`): true when
/// `other` is no `TESReputation`, when the 4 bytes of data at +0x40 differ
/// (`_memcmp` with `0087eaa0`), or when `TESForm::CompareAllComponents`
/// (with the cast form) says so.
pub fn fn_00616570(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let source = as_reputation(e, other);
    if source == 0 {
        return true;
    }
    let data = e.call(DATA_ADDRESS, &args![source]).u32();
    if e.call(MEMCMP, &args![this.addr() + 0x40, data, 4u32]).u32() != 0 {
        return true;
    }
    e.call(COMPARE_ALL_COMPONENTS, &args![this, source]).u32() as u8 != 0
}

// Translated from 006165f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reputation's save-game size (engine map: no name; `RET 4`): the
/// form's (`00484bf0` with `flags`) plus 4 and 4, in 16 bits.
pub fn fn_006165f0(e: &mut Engine, this: Ptr, flags: u32) -> u16 {
    let mut size: u16 = 0;
    let form = e.call(FORM_SAVE_SIZE, &args![this, flags]).u32() as u16;
    size = size.wrapping_add(form);
    size = size.wrapping_add(4);
    size.wrapping_add(4)
}

// Translated from 00616640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `TESForm::LoadGame` (engine map: no name; `RET 8`) and
/// answers what it leaves in `EAX`.
pub fn fn_00616640(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    e.call(FORM_LOAD_GAME, &args![this, first, second]).u32()
}

// Translated from 00616660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reputation's save into a save-game buffer (engine map: no name;
/// `RET 4`): the form's (`00484d60`), then the floats at +0x44 and +0x48
/// (4 bytes each, `00865e50` with 0).
pub fn fn_00616660(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(FORM_SAVE_GAME, &args![this, buffer]);
    for offset in [0x44u32, 0x48] {
        e.call(
            BUFFER_SAVE_BYTES,
            &args![buffer, this.addr() + offset, 4u32, 0u32],
        );
    }
}

// Translated from 006166a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reputation's load from a load-game buffer (engine map: no name;
/// `RET 4`): the form's (`00484da0`), then the floats at +0x44 and +0x48
/// (4 bytes each, `00864980`).
pub fn fn_006166a0(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(FORM_LOAD_GAME_BUFFER, &args![this, buffer]);
    for offset in [0x44u32, 0x48] {
        e.call(
            BUFFER_LOAD_BYTES,
            &args![buffer, this.addr() + offset, 4u32],
        );
    }
}

// Translated from 006166e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reputation's icon name (engine map: no name; no stack words): the
/// text of the icon (+0x24, `00408da0`) when it is not empty (`0048cee0`),
/// else the empty string (`01011584`).
pub fn fn_006166e0(e: &mut Engine, this: Ptr) -> u32 {
    let icon = this.addr() + 0x24;
    if e.call(STRING_LENGTH, &args![icon]).u32() != 0 {
        e.call(STRING_TEXT, &args![icon]).u32()
    } else {
        EMPTY_STRING
    }
}

// Translated from 00616710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESReputation::GetReputationTitle` (engine map; no stack words): the
/// text (`00403df0`) of the title setting at row `00616950(0)` and column
/// `00616950(1)` (signed bytes) of a 4 by 4 table the game builds on its
/// stack. The frame is a block here laid out as the exe's (`this`, the two
/// level bytes at +6 and +7, the table at +8), so an index outside the
/// table reads that frame (or past it) as the exe does.
pub fn tes_reputation_get_reputation_title(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(0x48, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, this.addr());
        let table = base + 8;
        for (i, setting) in REPUTATION_TITLES.iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *setting);
        }
        e.mem.set_u8(base + 6, 0);
        e.mem.set_u8(base + 7, 0);
        let positive = e.call(REPUTATION_LEVEL, &args![this, 1u32]).u32() as u8;
        e.mem.set_u8(base + 6, positive);
        let negative = e.call(REPUTATION_LEVEL, &args![this, 0u32]).u32() as u8;
        e.mem.set_u8(base + 7, negative);
        let row = (i32::from(e.mem.i8(base + 7)) << 4) as u32;
        let column = (i32::from(e.mem.i8(base + 6)) << 2) as u32;
        let setting = e.mem.u32(table.wrapping_add(row).wrapping_add(column));
        e.call(SETTING_STRING, &args![setting]).u32()
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_6290, tes_reputation_load(Ptr, Ptr) -> bool),
        entry!(0x0061_6470, tes_reputation_save(Ptr)),
        entry!(0x0061_6500, fn_00616500(Ptr, u32)),
        entry!(0x0061_6570, fn_00616570(Ptr, u32) -> bool),
        entry!(0x0061_65f0, fn_006165f0(Ptr, u32) -> u16),
        entry!(0x0061_6640, fn_00616640(Ptr, u32, u32) -> u32),
        entry!(0x0061_6660, fn_00616660(Ptr, u32)),
        entry!(0x0061_66a0, fn_006166a0(Ptr, u32)),
        entry!(0x0061_66e0, fn_006166e0(Ptr) -> u32),
        entry!(0x0061_6710, tes_reputation_get_reputation_title(Ptr) -> u32),
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

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn load(swap: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0xe0, 0x7000_00e0);
        e.mem.set_u32(vtable + 0x134, 0x7000_0134);
        e.mem.set_u32(this, vtable);
        let chunks = Rc::new(RefCell::new((
            vec![
                CHUNK_EDID, CHUNK_OBND, CHUNK_FULL, CHUNK_ICON, CHUNK_MICO, CHUNK_DATA,
            ],
            0usize,
        )));
        let file = e.mem.alloc(0x10);
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_TYPE, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).copied().unwrap_or(0))
        });
        let c = chunks;
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        e.register(FILE_CHUNK_SIZE, |_, _| ret(8));
        if swap {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(1));
        } else {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(0));
        }
        for addr in [
            0x7000_00e0u32,
            0x7000_0134,
            FILE_CHUNK_DATA_SIZED,
            0x0048_5110,
            FORM_SET_FLAG,
            0x0048_7050,
            0x0048_e3d0,
            0x0047_a290,
            0x0048_61f0,
            0x0050_3210,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        assert!(e.call(0x0061_6290, &args![this, file]).bool());
        (e, this, file)
    }

    #[test]
    fn load_sends_each_chunk_to_its_component() {
        let (e, this, file) = load(true);
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 0]]);
        assert_eq!(calls(&e, FILE_CHUNK_DATA_SIZED)[0][2], 0x200);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_7050), vec![vec![this + 0x18, file]]);
        assert_eq!(calls(&e, 0x0048_e3d0), vec![vec![this + 0x24, file]]);
        assert_eq!(calls(&e, 0x0047_a290), vec![vec![this + 0x30, file]]);
        assert_eq!(
            calls(&e, 0x0048_61f0),
            vec![vec![this, file, this + 0x40, 4]]
        );
        assert_eq!(calls(&e, 0x0050_3210), vec![vec![this + 0x40]]);

        let (e, _, _) = load(false);
        assert!(calls(&e, 0x0050_3210).is_empty());
    }
}

#[cfg(test)]
mod save_title_tests {
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

    #[test]
    fn save_writes_components_and_data() {
        let mut e = Engine::new();
        for a in [
            START_FORM,
            CLOSE_FORM,
            FULL_NAME_SAVE,
            TEXTURE_SAVE,
            MESSAGE_ICON_SAVE,
            SWAP_DATA,
            SAVE_DATA,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(SAVE_SWAPS_BYTES, |_, _| ret(1));
        let (_, log) = run(&mut e, 0x0061_6470, &args![0x4000u32]);
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (START_FORM, vec![0x4000]),
                (FULL_NAME_SAVE, vec![0x4018]),
                (TEXTURE_SAVE, vec![0x4024, CHUNK_ICON]),
                (MESSAGE_ICON_SAVE, vec![0x4030, CHUNK_MICO]),
                (SAVE_SWAPS_BYTES, vec![]),
                (SWAP_DATA, vec![0x4040]),
                (SAVE_DATA, vec![0x4000, 0x4040, 4]),
                (SAVE_SWAPS_BYTES, vec![]),
                (SWAP_DATA, vec![0x4040]),
                (CLOSE_FORM, vec![0x4000])
            ]
        );
    }

    #[test]
    fn copy_and_compare() {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(0x100));
        e.register(DATA_ADDRESS, |_, a| ret(a[0] + 0x40));
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, b);
            }
            Ret::default()
        });
        e.register(MEMCMP, |e, a| {
            let differ = (0..a[2]).any(|i| e.mem.u8(a[0] + i) != e.mem.u8(a[1] + i));
            ret(u32::from(differ))
        });
        let this = e.mem.alloc(0x50);
        let other = e.mem.alloc(0x50);
        e.mem.set_u32(other + 0x40, 0x0102_0304);
        e.mem.set_u32(other + 0x44, 0x7f80_0001);
        e.mem.set_u32(other + 0x48, 2.0f32.to_bits());
        assert!(e.call(0x0061_6570, &args![this, other]).bool());
        let (_, log) = run(&mut e, 0x0061_6500, &args![this, other]);
        assert_eq!(only(&log, COPY_ALL_COMPONENTS), vec![vec![this, other]]);
        assert_eq!(e.mem.u32(this + 0x40), 0x0102_0304);
        assert_eq!(e.mem.u32(this + 0x44), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x48), 2.0f32.to_bits());
        assert!(!e.call(0x0061_6570, &args![this, other]).bool());
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(1));
        assert!(e.call(0x0061_6570, &args![this, other]).bool());
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(e.call(0x0061_6570, &args![this, other]).bool());
        let fresh = e.mem.alloc(0x50);
        e.call(0x0061_6500, &args![fresh, other]);
        assert_eq!(e.mem.u32(fresh + 0x40), 0);
    }

    #[test]
    fn save_game_size_load_and_save() {
        let mut e = Engine::new();
        e.register(FORM_SAVE_SIZE, |_, a| ret(0x1_fff0 + a[1]));
        assert_eq!(
            e.call(0x0061_65f0, &args![0x4000u32, 4u32]).u32() & 0xffff,
            0xfffc
        );
        e.register(FORM_LOAD_GAME, |_, a| ret(a[1] + a[2]));
        assert_eq!(e.call(0x0061_6640, &args![0x4000u32, 2u32, 3u32]).u32(), 5);
        for a in [
            FORM_SAVE_GAME,
            FORM_LOAD_GAME_BUFFER,
            BUFFER_SAVE_BYTES,
            BUFFER_LOAD_BYTES,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        let (_, log) = run(&mut e, 0x0061_6660, &args![0x4000u32, 0x9000u32]);
        assert_eq!(only(&log, FORM_SAVE_GAME), vec![vec![0x4000, 0x9000]]);
        assert_eq!(
            only(&log, BUFFER_SAVE_BYTES),
            vec![vec![0x9000, 0x4044, 4, 0], vec![0x9000, 0x4048, 4, 0]]
        );
        let (_, log) = run(&mut e, 0x0061_66a0, &args![0x4000u32, 0x9000u32]);
        assert_eq!(
            only(&log, FORM_LOAD_GAME_BUFFER),
            vec![vec![0x4000, 0x9000]]
        );
        assert_eq!(
            only(&log, BUFFER_LOAD_BYTES),
            vec![vec![0x9000, 0x4044, 4], vec![0x9000, 0x4048, 4]]
        );
    }

    #[test]
    fn icon_name_and_title() {
        let mut e = Engine::new();
        e.register_double(STRING_LENGTH, |_, a| ret(u32::from(a[0] == 0x4024)));
        e.register(STRING_TEXT, |_, a| ret(a[0] + 0x100));
        assert_eq!(e.call(0x0061_66e0, &args![0x4000u32]).u32(), 0x4124);
        assert_eq!(e.call(0x0061_66e0, &args![0x5000u32]).u32(), EMPTY_STRING);
        e.register(REPUTATION_LEVEL, |_, a| {
            ret(if a[1] == 1 { 0x102 } else { 0x203 })
        });
        e.register(SETTING_STRING, |_, a| ret(a[0] + 1));
        let (r, log) = run(&mut e, 0x0061_6710, &args![0x4000u32]);
        assert_eq!(r.eax, REPUTATION_TITLES[3 * 4 + 2] + 1);
        assert_eq!(
            only(&log, REPUTATION_LEVEL),
            vec![vec![0x4000, 1], vec![0x4000, 0]]
        );
    }
}
