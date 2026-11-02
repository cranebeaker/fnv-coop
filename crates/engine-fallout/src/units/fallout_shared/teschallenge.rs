//! `fallout shared/teschallenge.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESForm::GetFile` (`00484e60`), `TESForm::AddCompileIndex` (`00485d50`,
/// cdecl), the form lookup by id (`004839c0`, cdecl) and the form id
/// (`0084e3a0`).
const FORM_GET_FILE: u32 = 0x0048_4e60;
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
const LOOKUP_FORM: u32 = 0x0048_39c0;
const FORM_ID: u32 = 0x0084_e3a0;
/// The warning printer (`005b5e40`, cdecl, variable arguments) and
/// `"MASTERFILE: Could not find Form (%08X) on challenge '%s' (%08X)."`
const WARNING: u32 = 0x005b_5e40;
const MISSING_FORM_FORMAT: u32 = 0x0104_8a58;

/// `TESFile` chunk reading (see `bgsaddonnode.rs`).
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const FILE_SWAPS_ENDIAN: u32 = 0x0040_1680;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_ICON: u32 = 0x4e4f_4349;
const CHUNK_MICO: u32 = 0x4f43_494d;
const CHUNK_SCRI: u32 = 0x4952_4353;
const CHUNK_DESC: u32 = 0x4353_4544;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_SNAM: u32 = 0x4d41_4e53;
const CHUNK_XNAM: u32 = 0x4d41_4e58;

/// The form id at `offset` looked up (adjusted to the challenge's file),
/// with a warning when nothing is found.
fn resolve_form(e: &mut Engine, this: Ptr, offset: u32) {
    let base = this.addr();
    let stored = e.mem.u32(base + offset);
    if stored == 0 {
        return;
    }
    let id = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), stored);
        let file = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
        e.call(FORM_ADD_COMPILE_INDEX, &args![slot, file]);
        e.mem.u32(slot.addr())
    });
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    e.mem.set_u32(base + offset, form);
    if e.mem.u32(base + offset) == 0 {
        let form_id = e.call(FORM_ID, &args![this]).u32();
        let name = e.vcall(base, 0x130, &args![]).u32();
        e.call(WARNING, &args![MISSING_FORM_FORMAT, id, name, form_id]);
    }
}

// Translated from 005f5080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESChallenge::InitItem` (Xbox PDB): unless `004013e0` answers true,
/// `TESScriptableForm::InitItem` (+0x2C), then the two form ids at +0x74
/// and +0x78 looked up (a warning with the id, the name and the form id
/// for each one not found) and `00484ab0(1)`.
pub fn tes_challenge_init_item(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    e.call(0x0048_cd90, &args![this.addr() + 0x2c, this]);
    resolve_form(e, this, 0x74);
    resolve_form(e, this, 0x78);
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

/// A form id chunk read into a zeroed local (`004727f0`).
fn read_form_id(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        e.call(FILE_CHUNK_DATA, &args![file, slot]);
        e.mem.u32(slot.addr())
    })
}

// Translated from 005f51a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f51a0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESChallenge::Load` (Xbox PDB): `TESForm::LoadForm`, `00484ab0(0)` and
/// each chunk: `EDID`, `OBND`, `FULL` (+0x18), `ICON` (+0x38), `MICO`
/// (+0x44), `SCRI` (the script's id on +0x2C, then
/// `TESScriptableForm::InitItem`), `DESC` (+0x24), `DATA` (0x18 bytes at
/// +0x54, swapped by `005f5480`), `SNAM` (`004febb0`) and `XNAM`
/// (`00442a80`) with their form ids. Returns true.
pub fn tes_challenge_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
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
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            CHUNK_DATA => {
                e.call(0x0048_61f0, &args![this, file, base + 0x54, 0x18u32]);
                if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
                    e.call(0x005f_5480, &args![base + 0x54]);
                }
            }
            CHUNK_DESC => {
                e.call(0x0048_2d50, &args![or_null(0x24), file]);
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_SCRI => {
                let id = read_form_id(e, file);
                e.call(0x006e_cd40, &args![base + 0x2c, id]);
                e.call(0x0048_cd90, &args![base + 0x2c, this]);
            }
            CHUNK_ICON => {
                e.call(0x0048_e3d0, &args![or_null(0x38), file]);
            }
            CHUNK_SNAM => {
                let id = read_form_id(e, file);
                e.call(0x004f_ebb0, &args![this, id]);
            }
            CHUNK_XNAM => {
                let id = read_form_id(e, file);
                e.call(0x0044_2a80, &args![this, id]);
            }
            CHUNK_MICO => {
                e.call(0x0047_a290, &args![or_null(0x44), file]);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// The swap helpers (`cdecl (address, flag)`, given 0 here): a word
/// (`00401080`) and a 16-bit word (`00407a90`).
const SWAP_WORD: u32 = 0x0040_1080;
const SWAP_SHORT: u32 = 0x0040_7a90;
/// The savers `TESChallenge::Save` calls: `TESForm::StartForm`
/// (`004855a0`), `TESFullName::Save` (`00487010`), `TESTexture::Save`
/// (`0048e370`, one chunk word), `BGSMessageIcon::Save` (`0047a240`, one
/// chunk word), `TESScriptableForm::Save` (`0048cca0`),
/// `TESDescription::Save` (`00482cb0`), `TESForm::__SaveData` (`00485e20`,
/// `thiscall (form, data, size)`), `TESForm::AddChunk` with a word
/// (`00485910`, `cdecl`) and `TESForm::CloseForm` (`00485680`); the byte
/// `00401500` answers (whether saving swaps bytes).
const FORM_START: u32 = 0x0048_55a0;
const SAVE_FULL_NAME: u32 = 0x0048_7010;
const SAVE_TEXTURE: u32 = 0x0048_e370;
const SAVE_MESSAGE_ICON: u32 = 0x0047_a240;
const SAVE_SCRIPTABLE: u32 = 0x0048_cca0;
const SAVE_DESCRIPTION: u32 = 0x0048_2cb0;
const FORM_SAVE_DATA: u32 = 0x0048_5e20;
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
const FORM_CLOSE: u32 = 0x0048_5680;
const SAVE_SWAPS: u32 = 0x0040_1500;

// Translated from 005f5480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`; the byte swap of the challenge's
/// 0x18-byte data at +0x54 of a `TESChallenge`): the words at +0, +4,
/// +0xC (`00401080`) and the 16-bit words at +8, +0x10, +0x12, +0x14
/// (`00407a90`), in the exe's order.
pub fn fn_005f5480(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    for (helper, offset) in [
        (SWAP_WORD, 0u32),
        (SWAP_WORD, 4),
        (SWAP_SHORT, 8),
        (SWAP_WORD, 0xc),
        (SWAP_SHORT, 0x10),
        (SWAP_SHORT, 0x12),
        (SWAP_SHORT, 0x14),
    ] {
        e.call(helper, &args![at + offset, 0u32]);
    }
}

// Translated from 005f5540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESChallenge::Save` (Xbox PDB): starts the form, saves the full name
/// (+0x18), the icon (+0x38, `ICON`), the message icon (+0x44, `MICO`),
/// the script (+0x2C) and the description (+0x24), the 0x18-byte data at
/// +0x54 (`__SaveData`, swapped by [`fn_005f5480`] before and after when
/// saving swaps bytes), the forms at +0x74 and +0x78 (their form ids as
/// `SNAM` and `XNAM`) and closes the form.
pub fn tes_challenge_save(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.call(FORM_START, &args![this]);
    e.call(SAVE_FULL_NAME, &args![at + 0x18]);
    e.call(SAVE_TEXTURE, &args![at + 0x38, CHUNK_ICON]);
    e.call(SAVE_MESSAGE_ICON, &args![at + 0x44, CHUNK_MICO]);
    e.call(SAVE_SCRIPTABLE, &args![at + 0x2c]);
    e.call(SAVE_DESCRIPTION, &args![at + 0x24]);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        fn_005f5480(e, Ptr::new(at + 0x54));
    }
    e.call(FORM_SAVE_DATA, &args![this, at + 0x54, 0x18u32]);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        fn_005f5480(e, Ptr::new(at + 0x54));
    }
    for (offset, chunk) in [(0x74u32, CHUNK_SNAM), (0x78, CHUNK_XNAM)] {
        let form = e.mem.u32(at + offset);
        if form != 0 {
            let id = e.call(FORM_ID, &args![form]).u32();
            e.call(ADD_CHUNK_WORD, &args![chunk, id]);
        }
    }
    e.call(FORM_CLOSE, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_5080, tes_challenge_init_item(Ptr)),
        entry!(0x005f_51a0, tes_challenge_load(Ptr, Ptr) -> bool),
        entry!(0x005f_5480, fn_005f5480(Ptr)),
        entry!(0x005f_5540, tes_challenge_save(Ptr)),
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

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
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

    fn challenge(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x7c);
        let vtable = e.mem.alloc(0x140);
        for slot in [0x130u32, 0x134, 0xe0] {
            e.mem.set_u32(vtable + slot, 0x7000_0000 + slot);
        }
        e.mem.set_u32(this, vtable);
        e.register(0x7000_0130, |_, _| ret(0x5555));
        quiet(e, &[0x7000_0134, 0x7000_00e0]);
        this
    }

    #[test]
    fn init_item_looks_up_both_forms() {
        let mut e = Engine::new();
        let this = challenge(&mut e);
        e.mem.set_u32(this + 0x74, 0x11);
        e.mem.set_u32(this + 0x78, 0x22);
        e.register(0x0040_13e0, |_, _| ret(0));
        e.register(FORM_GET_FILE, |_, _| ret(0x4400));
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| {
            ret(if a[0] & 0xff == 0x22 { 0x9000 } else { 0 })
        });
        e.register(FORM_ID, |_, _| ret(0x0012_3456));
        quiet(&mut e, &[WARNING, 0x0048_cd90, 0x0048_4ab0]);
        e.call_log = Some(vec![]);
        e.call(0x005f_5080, &args![this]);
        assert_eq!(calls(&e, 0x0048_cd90), vec![vec![this + 0x2c, this]]);
        assert_eq!(e.mem.u32(this + 0x74), 0);
        assert_eq!(e.mem.u32(this + 0x78), 0x9000);
        assert_eq!(
            calls(&e, WARNING),
            vec![vec![MISSING_FORM_FORMAT, 0x0100_0011, 0x5555, 0x0012_3456]]
        );
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![this, 1]]);

        e.register(0x0040_13e0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x005f_5080, &args![this]);
        assert!(calls(&e, 0x0048_cd90).is_empty());
    }

    #[test]
    fn load_sends_each_chunk_to_its_component() {
        let mut e = Engine::new();
        let this = challenge(&mut e);
        let file = e.mem.alloc(0x10);
        let chunks = Rc::new(RefCell::new((
            vec![
                (CHUNK_EDID, 0u32),
                (CHUNK_OBND, 0),
                (CHUNK_FULL, 0),
                (CHUNK_ICON, 0),
                (CHUNK_MICO, 0),
                (CHUNK_SCRI, 0x11),
                (CHUNK_DESC, 0),
                (CHUNK_DATA, 0),
                (CHUNK_SNAM, 0x22),
                (CHUNK_XNAM, 0x33),
            ],
            0usize,
        )));
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_TYPE, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).map_or(0, |x| x.0))
        });
        let c = chunks.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        let c = chunks;
        e.register_double(FILE_CHUNK_DATA, move |e, a| {
            let c = c.borrow();
            e.mem.set_u32(a[1], c.0[c.1].1);
            Ret::default()
        });
        e.register(FILE_CHUNK_SIZE, |_, _| ret(8));
        e.register(FILE_SWAPS_ENDIAN, |_, _| ret(1));
        quiet(
            &mut e,
            &[
                FILE_CHUNK_DATA_SIZED,
                0x0048_5110,
                0x0048_4ab0,
                0x0048_7050,
                0x0048_e3d0,
                0x0047_a290,
                0x006e_cd40,
                0x0048_cd90,
                0x0048_2d50,
                0x0048_61f0,
                0x005f_5480,
                0x004f_ebb0,
                0x0044_2a80,
            ],
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x005f_51a0, &args![this, file]).bool());
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_7050), vec![vec![this + 0x18, file]]);
        assert_eq!(calls(&e, 0x0048_e3d0), vec![vec![this + 0x38, file]]);
        assert_eq!(calls(&e, 0x0047_a290), vec![vec![this + 0x44, file]]);
        assert_eq!(calls(&e, 0x006e_cd40), vec![vec![this + 0x2c, 0x11]]);
        assert_eq!(calls(&e, 0x0048_cd90), vec![vec![this + 0x2c, this]]);
        assert_eq!(calls(&e, 0x0048_2d50), vec![vec![this + 0x24, file]]);
        assert_eq!(
            calls(&e, 0x0048_61f0),
            vec![vec![this, file, this + 0x54, 0x18]]
        );
        assert_eq!(calls(&e, 0x005f_5480), vec![vec![this + 0x54]]);
        assert_eq!(calls(&e, 0x004f_ebb0), vec![vec![this, 0x22]]);
        assert_eq!(calls(&e, 0x0044_2a80), vec![vec![this, 0x33]]);
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x7300_0000, 4);
        for addr in [
            SWAP_WORD,
            SWAP_SHORT,
            FORM_START,
            SAVE_FULL_NAME,
            SAVE_TEXTURE,
            SAVE_MESSAGE_ICON,
            SAVE_SCRIPTABLE,
            SAVE_DESCRIPTION,
            FORM_SAVE_DATA,
            ADD_CHUNK_WORD,
            FORM_CLOSE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(SAVE_SWAPS, |e, _| e.mem.u32(0x7300_0000).into_ret());
        e.register(FORM_ID, |_, a| (a[0] + 1).into_ret());
        e
    }

    #[test]
    fn endian_swaps_words_and_shorts() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x005f_5480, &args![0x1000u32]);
        let order: Vec<(u32, u32)> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .skip(1)
            .map(|c| (c.0, c.1[0]))
            .collect();
        assert_eq!(
            order,
            vec![
                (SWAP_WORD, 0x1000),
                (SWAP_WORD, 0x1004),
                (SWAP_SHORT, 0x1008),
                (SWAP_WORD, 0x100c),
                (SWAP_SHORT, 0x1010),
                (SWAP_SHORT, 0x1012),
                (SWAP_SHORT, 0x1014)
            ]
        );
    }

    #[test]
    fn save_writes_every_part() {
        let mut e = engine();
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x74, 0x5000);
        e.mem.set_u32(0x7300_0000, 1);
        e.call_log = Some(vec![]);
        e.call(0x005f_5540, &args![this]);
        assert_eq!(calls(&e, SAVE_TEXTURE), vec![vec![this + 0x38, CHUNK_ICON]]);
        assert_eq!(
            calls(&e, SAVE_MESSAGE_ICON),
            vec![vec![this + 0x44, CHUNK_MICO]]
        );
        assert_eq!(calls(&e, SAVE_SCRIPTABLE), vec![vec![this + 0x2c]]);
        assert_eq!(calls(&e, SAVE_DESCRIPTION), vec![vec![this + 0x24]]);
        assert_eq!(
            calls(&e, FORM_SAVE_DATA),
            vec![vec![this, this + 0x54, 0x18]]
        );
        assert_eq!(calls(&e, SWAP_WORD).len(), 6);
        assert_eq!(calls(&e, ADD_CHUNK_WORD), vec![vec![CHUNK_SNAM, 0x5001]]);
        assert_eq!(calls(&e, FORM_CLOSE), vec![vec![this]]);
        e.mem.set_u32(0x7300_0000, 0);
        e.mem.set_u32(this + 0x74, 0);
        e.mem.set_u32(this + 0x78, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x005f_5540, &args![this]);
        assert!(calls(&e, SWAP_WORD).is_empty());
        assert_eq!(calls(&e, ADD_CHUNK_WORD), vec![vec![CHUNK_XNAM, 0x6001]]);
    }
}
