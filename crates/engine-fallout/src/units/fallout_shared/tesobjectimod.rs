//! `fallout shared/tesobjectimod.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004013e0`: true makes `InitItem` do nothing; `00484ab0`: the form
/// flag setter.
const FORM_INIT_SKIP: u32 = 0x0040_13e0;
const FORM_SET_FLAG: u32 = 0x0048_4ab0;

/// `TESFile` chunk reading (see `bgsaddonnode.rs`).
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const FILE_VERSION: u32 = 0x0040_3570;

/// The components (PC offsets): full name +0x30, model +0x3C, icon +0x5C,
/// script +0x68, description +0x74, destructible +0x8C, message icon
/// +0x94, pickup/putdown sounds +0xA4.
const FULL_NAME: u32 = 0x30;
const MODEL: u32 = 0x3c;
const ICON: u32 = 0x5c;
const SCRIPT: u32 = 0x68;
const DESCRIPTION: u32 = 0x74;
const DESTRUCTIBLE: u32 = 0x8c;
const MESSAGE_ICON: u32 = 0x94;
const SOUNDS: u32 = 0xa4;
/// A form id setter on a component (`006ecd40`).
const SET_FORM_ID: u32 = 0x006e_cd40;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;
const CHUNK_MODD: u32 = 0x4444_4f4d;
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_ICON: u32 = 0x4e4f_4349;
const CHUNK_MICO: u32 = 0x4f43_494d;
const CHUNK_SCRI: u32 = 0x4952_4353;
const CHUNK_DESC: u32 = 0x4353_4544;
const CHUNK_DEST: u32 = 0x5453_4544;
const CHUNK_DSTD: u32 = 0x4454_5344;
const CHUNK_YNAM: u32 = 0x4d41_4e59;
const CHUNK_ZNAM: u32 = 0x4d41_4e5a;
const CHUNK_DATA: u32 = 0x4154_4144;

// Translated from 00519fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectIMOD::InitItem` (Xbox PDB): unless `004013e0` answers true,
/// `TESScriptableForm::InitItem` (+0x68), `BGSDestructibleObjectForm::
/// InitItem` (+0x8C), `TESModelTextureSwap::InitItem` (+0x3C),
/// `BGSPickupPutdownSounds::InitItem` (+0xA4), each with the form, then
/// `00484ab0(1)`.
pub fn tes_object_imod_init_item(e: &mut Engine, this: Ptr) {
    if e.call(FORM_INIT_SKIP, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    e.call(0x0048_cd90, &args![base + SCRIPT, this]);
    e.call(0x0047_8be0, &args![base + DESTRUCTIBLE, this]);
    e.call(0x0048_aa80, &args![base + MODEL, this]);
    e.call(0x0047_b6a0, &args![base + SOUNDS, this]);
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
}

/// A form id chunk read into a zeroed local (`004727f0`).
fn read_form_id(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        e.call(FILE_CHUNK_DATA, &args![file, slot]);
        e.mem.u32(slot.addr())
    })
}

// Translated from 0051a030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051a030 - the /GS stack cookie check (00ec408c) is not translated.
/// The item mod's record reader (`RET 4`: the `TESFile`; the engine map
/// has no name; the form type is not checked): `TESForm::LoadForm`,
/// `00484ab0(0)` and each chunk to its component: `EDID`, `OBND`, `FULL`,
/// `MODL`/`MODT`/`MODD`/`MOSD` (`TESModel::LoadModelChunk`), `MODS` (the
/// texture swaps), `ICON`, `MICO`, `SCRI` (the script's id, then
/// `TESScriptableForm::InitItem`), `DESC`, `DEST`/`DSTD`, `YNAM`/`ZNAM`
/// (the two sounds' ids, `006ecd40` and `00403550` on +0xA4) and `DATA`
/// (`TESForm::LoadData` with no buffer and size 0). Returns true.
pub fn fn_0051a030(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
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
            CHUNK_MODL | CHUNK_MODD | CHUNK_MOSD | CHUNK_MODT => {
                e.call(0x0048_92d0, &args![or_null(MODEL), file]);
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_DATA => {
                e.call(0x0048_61f0, &args![this, file, 0u32, 0u32]);
            }
            CHUNK_DESC => {
                e.call(0x0048_2d50, &args![or_null(DESCRIPTION), file]);
            }
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            CHUNK_DEST | CHUNK_DSTD => {
                e.call(0x0047_81e0, &args![or_null(DESTRUCTIBLE), file]);
            }
            CHUNK_SCRI => {
                let id = read_form_id(e, file);
                e.call(SET_FORM_ID, &args![base + SCRIPT, id]);
                e.call(0x0048_cd90, &args![base + SCRIPT, this]);
            }
            CHUNK_MICO => {
                e.call(0x0047_a290, &args![or_null(MESSAGE_ICON), file]);
            }
            CHUNK_ZNAM => {
                let id = read_form_id(e, file);
                e.call(0x0040_3550, &args![base + SOUNDS, id]);
            }
            CHUNK_FULL => {
                e.call(0x0048_7050, &args![or_null(FULL_NAME), file]);
            }
            CHUNK_YNAM => {
                let id = read_form_id(e, file);
                e.call(SET_FORM_ID, &args![base + SOUNDS, id]);
            }
            CHUNK_ICON => {
                e.call(0x0048_e3d0, &args![or_null(ICON), file]);
            }
            CHUNK_MODS => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, data| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, data, size]);
                    let version = e.call(FILE_VERSION, &args![file]).u16();
                    e.call(
                        0x0048_a7b0,
                        &args![base + MODEL, data, size, u32::from(version)],
                    );
                });
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// `TESForm::StartForm` (`004855a0`), `TESForm::SaveData` (`00485e00`) and
/// `TESForm::CloseForm` (`00485680`), `thiscall` on the form, no stack words.
const START_FORM: u32 = 0x0048_55a0;
const FORM_SAVE_DATA: u32 = 0x0048_5e00;
const CLOSE_FORM: u32 = 0x0048_5680;
/// The component saves (engine map; `thiscall`): `TESFullName::Save`
/// (`00487010`), `TESModelTextureSwap::Save` (`0048a520`, three chunk
/// tags), `TESTexture::Save` (`0048e370`, a tag), `BGSMessageIcon::Save`
/// (`0047a240`, a tag), `TESScriptableForm::Save` (`0048cca0`),
/// `BGSDestructibleObjectForm::Save` (`00477f20`) and
/// `BGSPickupPutdownSounds::Save` (`0047b560`).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const MODEL_TEXTURE_SWAP_SAVE: u32 = 0x0048_a520;
const TEXTURE_SAVE: u32 = 0x0048_e370;
const MESSAGE_ICON_SAVE: u32 = 0x0047_a240;
const SCRIPTABLE_FORM_SAVE: u32 = 0x0048_cca0;
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
const PICKUP_PUTDOWN_SAVE: u32 = 0x0047_b560;
/// `TESDescription::Save` (`00482cb0`, `thiscall`).
const DESCRIPTION_SAVE: u32 = 0x0048_2cb0;

// Translated from 0051a420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectIMOD::Save` (engine map; no stack words): `StartForm`, the full
/// name (+0x30), the model with texture swaps (+0x3c; `MODL`, `MODT`,
/// `MODS`), the icon (+0x5c, `ICON`), the message icon (+0x94, `MICO`),
/// script (+0x68), description (+0x74), destructible data (+0x8c),
/// pickup/putdown sounds (+0xa4), `TESForm::SaveData`, `CloseForm`.
pub fn tes_object_imod_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![base + FULL_NAME]);
    e.call(
        MODEL_TEXTURE_SWAP_SAVE,
        &args![base + MODEL, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS],
    );
    e.call(TEXTURE_SAVE, &args![base + ICON, CHUNK_ICON]);
    e.call(MESSAGE_ICON_SAVE, &args![base + MESSAGE_ICON, CHUNK_MICO]);
    e.call(SCRIPTABLE_FORM_SAVE, &args![base + SCRIPT]);
    e.call(DESCRIPTION_SAVE, &args![base + DESCRIPTION]);
    e.call(DESTRUCTIBLE_SAVE, &args![base + DESTRUCTIBLE]);
    e.call(PICKUP_PUTDOWN_SAVE, &args![base + SOUNDS]);
    e.call(FORM_SAVE_DATA, &args![this]);
    e.call(CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_9fc0, tes_object_imod_init_item(Ptr)),
        entry!(0x0051_a030, fn_0051a030(Ptr, Ptr) -> bool),
        entry!(0x0051_a420, tes_object_imod_save(Ptr)),
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

    #[test]
    fn init_item_initializes_the_components() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc0);
        e.register(FORM_INIT_SKIP, |_, _| ret(0));
        quiet(
            &mut e,
            &[
                0x0048_cd90,
                0x0047_8be0,
                0x0048_aa80,
                0x0047_b6a0,
                FORM_SET_FLAG,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0051_9fc0, &args![this]);
        let order: Vec<(u32, Vec<u32>)> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x0051_9fc0 && *a != FORM_INIT_SKIP)
            .collect();
        assert_eq!(
            order,
            vec![
                (0x0048_cd90, vec![this + 0x68, this]),
                (0x0047_8be0, vec![this + 0x8c, this]),
                (0x0048_aa80, vec![this + 0x3c, this]),
                (0x0047_b6a0, vec![this + 0xa4, this]),
                (FORM_SET_FLAG, vec![this, 1]),
            ]
        );
        e.register(FORM_INIT_SKIP, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0051_9fc0, &args![this]);
        assert!(calls(&e, FORM_SET_FLAG).is_empty());
    }

    #[test]
    fn load_sends_each_chunk_to_its_component() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc0);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0xe0, 0x7000_00e0);
        e.mem.set_u32(vtable + 0x134, 0x7000_0134);
        e.mem.set_u32(this, vtable);
        let chunks = Rc::new(RefCell::new((
            vec![
                (CHUNK_EDID, 0u32),
                (CHUNK_OBND, 0),
                (CHUNK_FULL, 0),
                (CHUNK_MODL, 0),
                (CHUNK_MODT, 0),
                (CHUNK_MODD, 0),
                (CHUNK_MOSD, 0),
                (CHUNK_MODS, 0),
                (CHUNK_ICON, 0),
                (CHUNK_MICO, 0),
                (CHUNK_SCRI, 0x11),
                (CHUNK_DESC, 0),
                (CHUNK_DEST, 0),
                (CHUNK_DSTD, 0),
                (CHUNK_YNAM, 0x22),
                (CHUNK_ZNAM, 0x33),
                (CHUNK_DATA, 0),
            ],
            0usize,
        )));
        let file = e.mem.alloc(0x10);
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
        e.register(FILE_VERSION, |_, _| ret(0x1_0005));
        quiet(
            &mut e,
            &[
                0x7000_00e0,
                0x7000_0134,
                FILE_CHUNK_DATA_SIZED,
                0x0048_5110,
                FORM_SET_FLAG,
                0x0048_7050,
                0x0048_92d0,
                0x0048_a7b0,
                0x0048_e3d0,
                0x0047_a290,
                SET_FORM_ID,
                0x0048_cd90,
                0x0048_2d50,
                0x0047_81e0,
                0x0040_3550,
                0x0048_61f0,
            ],
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0051_a030, &args![this, file]).bool());
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_7050), vec![vec![this + 0x30, file]]);
        assert_eq!(calls(&e, 0x0048_92d0), vec![vec![this + 0x3c, file]; 4]);
        let swaps = calls(&e, 0x0048_a7b0);
        assert_eq!((swaps[0][0], swaps[0][2], swaps[0][3]), (this + 0x3c, 8, 5));
        assert_eq!(calls(&e, 0x0048_e3d0), vec![vec![this + 0x5c, file]]);
        assert_eq!(calls(&e, 0x0047_a290), vec![vec![this + 0x94, file]]);
        assert_eq!(
            calls(&e, SET_FORM_ID),
            vec![vec![this + 0x68, 0x11], vec![this + 0xa4, 0x22]]
        );
        assert_eq!(calls(&e, 0x0048_cd90), vec![vec![this + 0x68, this]]);
        assert_eq!(calls(&e, 0x0048_2d50), vec![vec![this + 0x74, file]]);
        assert_eq!(calls(&e, 0x0047_81e0), vec![vec![this + 0x8c, file]; 2]);
        assert_eq!(calls(&e, 0x0040_3550), vec![vec![this + 0xa4, 0x33]]);
        assert_eq!(calls(&e, 0x0048_61f0), vec![vec![this, file, 0, 0]]);
    }
}

#[cfg(test)]
mod save_tests {
    use super::*;

    #[test]
    fn save_writes_each_component_in_order() {
        let mut e = Engine::new();
        for a in [
            START_FORM,
            FORM_SAVE_DATA,
            CLOSE_FORM,
            FULL_NAME_SAVE,
            MODEL_TEXTURE_SWAP_SAVE,
            TEXTURE_SAVE,
            MESSAGE_ICON_SAVE,
            SCRIPTABLE_FORM_SAVE,
            DESCRIPTION_SAVE,
            DESTRUCTIBLE_SAVE,
            PICKUP_PUTDOWN_SAVE,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0051_a420, &args![0x4000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (START_FORM, vec![0x4000]),
                (FULL_NAME_SAVE, vec![0x4030]),
                (
                    MODEL_TEXTURE_SWAP_SAVE,
                    vec![0x403c, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS]
                ),
                (TEXTURE_SAVE, vec![0x405c, CHUNK_ICON]),
                (MESSAGE_ICON_SAVE, vec![0x4094, CHUNK_MICO]),
                (SCRIPTABLE_FORM_SAVE, vec![0x4068]),
                (DESCRIPTION_SAVE, vec![0x4074]),
                (DESTRUCTIBLE_SAVE, vec![0x408c]),
                (PICKUP_PUTDOWN_SAVE, vec![0x40a4]),
                (FORM_SAVE_DATA, vec![0x4000]),
                (CLOSE_FORM, vec![0x4000])
            ]
        );
    }
}
