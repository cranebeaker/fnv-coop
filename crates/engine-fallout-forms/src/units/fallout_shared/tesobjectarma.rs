//! `fallout shared/tesobjectarma.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004013e0`: true makes `InitItem` do nothing; `00484ab0`: the form
/// flag setter.
const FORM_INIT_SKIP: u32 = 0x0040_13e0;
const FORM_SET_FLAG: u32 = 0x0048_4ab0;
/// The warning printer (`005b5e40`, cdecl, variable arguments).
const WARNING: u32 = 0x005b_5e40;
/// `"MASTERFILE: Armor '%s' needs to have biped slots selected in the
/// editor."`
const NO_BIPED_SLOTS_FORMAT: u32 = 0x0102_a4a0;

/// `TESFile` chunk reading (see `bgsaddonnode.rs`).
const FILE_FORM_TYPE: u32 = 0x0047_2660;
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const FILE_SWAPS_ENDIAN: u32 = 0x0040_1680;
const FILE_VERSION: u32 = 0x0040_3570;
/// `memset` (`00403d30`, cdecl, three words).
const MEMSET: u32 = 0x0040_3d30;

/// Component readers, cdecl `(component, file)`: `TESModel::LoadModelChunk`
/// (`004892d0`), `TESModel::LoadModelTextureChunk` (`004893e0`),
/// `TESModelTextureSwap::LoadModelTextureSwapChunk` (`0048a990`), the full
/// name (`00487050`) and `BGSDestructibleObjectForm::LoadChunk`
/// (`004781e0`).
const LOAD_MODEL: u32 = 0x0048_92d0;
const LOAD_MODEL_TEXTURE: u32 = 0x0048_93e0;
const LOAD_TEXTURE_SWAP: u32 = 0x0048_a990;
const LOAD_FULL_NAME: u32 = 0x0048_7050;
const LOAD_DESTRUCTIBLE: u32 = 0x0047_81e0;
/// Setters taking a path text (on the component): `00489100` (the icon
/// components) and `0047a380` (the message icon components).
const SET_ICON: u32 = 0x0048_9100;
const SET_MESSAGE_ICON: u32 = 0x0047_a380;
/// Swaps the words of the data at +0x178 (`00535aa0`).
const SWAP_DATA: u32 = 0x0053_5aa0;

/// The four model components (on PC): +0x7C, +0x9C, +0xBC, +0xDC (their
/// vtable +0x18 sets the model path); the destructible form at +0x14C.
const MODEL_1: u32 = 0x7c;
const MODEL_2: u32 = 0x9c;
const MODEL_3: u32 = 0xbc;
const MODEL_4: u32 = 0xdc;
const DESTRUCTIBLE: u32 = 0x14c;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_BMDT: u32 = 0x5444_4d42;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;
const CHUNK_MODD: u32 = 0x4444_4f4d;
const CHUNK_MOD2: u32 = 0x3244_4f4d;
const CHUNK_MO2T: u32 = 0x5432_4f4d;
const CHUNK_MO2S: u32 = 0x5332_4f4d;
const CHUNK_MOD3: u32 = 0x3344_4f4d;
const CHUNK_MO3T: u32 = 0x5433_4f4d;
const CHUNK_MO3S: u32 = 0x5333_4f4d;
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_MOD4: u32 = 0x3444_4f4d;
const CHUNK_MO4T: u32 = 0x5434_4f4d;
const CHUNK_MO4S: u32 = 0x5334_4f4d;
const CHUNK_ICON: u32 = 0x4e4f_4349;
const CHUNK_ICO2: u32 = 0x324f_4349;
const CHUNK_MICO: u32 = 0x4f43_494d;
const CHUNK_MIC2: u32 = 0x3243_494d;
const CHUNK_BMCT: u32 = 0x5443_4d42;
const CHUNK_DEST: u32 = 0x5453_4544;
const CHUNK_DSTD: u32 = 0x4454_5344;
const CHUNK_ETYP: u32 = 0x5059_5445;
const CHUNK_DNAM: u32 = 0x4d41_4e44;
const CHUNK_DATA: u32 = 0x4154_4144;

// Translated from 00512030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectARMA::InitItem` (Xbox PDB): unless `004013e0` answers true,
/// warns (with the name, vtable +0x130) when the biped slot word at +0x74
/// is 0, then `BGSDestructibleObjectForm::InitItem` (+0x14C),
/// `TESBipedModelForm::InitItem` (+0x70) and `00484ab0(1)`.
pub fn tes_object_arma_init_item(e: &mut Engine, this: Ptr) {
    if e.call(FORM_INIT_SKIP, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    if e.mem.u32(base + 0x74) == 0 {
        let name = e.vcall(base, 0x130, &args![]).u32();
        e.call(WARNING, &args![NO_BIPED_SLOTS_FORMAT, name]);
    }
    e.call(0x0047_8be0, &args![base + DESTRUCTIBLE, this]);
    e.call(0x0048_0750, &args![base + 0x70, this]);
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
}

/// A chunk whose text (on the stack, `__alloca_probe_16` `00ec5ec0`, read
/// with size word `limit`) goes to `f`.
fn with_chunk_text(e: &mut Engine, file: Ptr, limit: u32, f: impl FnOnce(&mut Engine, Ptr)) {
    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
    e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
        e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, limit]);
        f(e, text);
    });
}

// Translated from 005120a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005120a0 - the /GS stack cookie check (00ec408c) is not translated.
/// The armor addon's record reader (`RET 4`: the `TESFile`; the engine map
/// has no name): false unless the record's form type byte is 0x60;
/// otherwise `TESForm::LoadForm`, `00484ab0(0)` and each chunk: `EDID`,
/// `OBND`, `FULL`, the biped data `BMDT` (format versions below 11: four
/// bytes, swapped by `00512810`, the low half to +0x74 and the third byte
/// to +0x78; later: eight bytes at +0x74, swapped by `00503210`; then bit 4
/// of +0x78 cleared), the four models with their textures and swaps,
/// `ICON`/`ICO2`, `MICO`/`MIC2`, `BMCT` (vtable +0x18 of +0x134),
/// `DEST`/`DSTD`, `ETYP` (`006ecd40` on +0x154) and `DNAM` (0xC bytes at
/// +0x178, swapped by `00535aa0`; a `DATA` chunk only swaps them); at the
/// end `004f15a0(0x18)`. Returns true.
pub fn fn_005120a0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let form_type = e.call(FILE_FORM_TYPE, &args![file]).u8();
    if form_type != 0x60 {
        return false;
    }
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
            CHUNK_EDID => with_chunk_text(e, file, 0x200, |e, text| {
                e.vcall(base, 0x134, &args![text]);
            }),
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_FULL => {
                e.call(LOAD_FULL_NAME, &args![or_null(0x30), file]);
            }
            CHUNK_BMDT => {
                let version = e.call(FILE_VERSION, &args![file]).u16();
                if i32::from(version) < 11 {
                    let data = e.with_stack(4, |e, data| {
                        e.call(FILE_CHUNK_DATA_SIZED, &args![file, data, 4u32]);
                        if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
                            e.call(0x0051_2810, &args![data]);
                        }
                        e.mem.u32(data.addr())
                    });
                    e.mem.set_u8(base + 0x78, (data >> 16) as u8);
                    e.mem.set_u32(base + 0x74, data & 0xffff);
                } else {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, base + 0x74, 8u32]);
                    if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
                        e.call(0x0050_3210, &args![base + 0x74]);
                    }
                }
                let flags = e.mem.u8(base + 0x78);
                e.mem.set_u8(base + 0x78, flags & 0xef);
            }
            CHUNK_BMCT => with_chunk_text(e, file, 0, |e, text| {
                e.vcall(base + 0x134, 0x18, &args![text]);
            }),
            CHUNK_MODL => with_chunk_text(e, file, 0, |e, text| {
                e.vcall(base + MODEL_1, 0x18, &args![text]);
            }),
            CHUNK_MODT => {
                e.call(LOAD_MODEL_TEXTURE, &args![base + MODEL_1, file]);
            }
            CHUNK_MODS => {
                e.call(LOAD_TEXTURE_SWAP, &args![base + MODEL_1, file]);
            }
            CHUNK_MODD => {
                e.call(LOAD_MODEL, &args![base + MODEL_1, file]);
            }
            CHUNK_MOSD => {
                e.call(LOAD_MODEL, &args![base + MODEL_2, file]);
            }
            CHUNK_MOD2 => with_chunk_text(e, file, 0, |e, text| {
                e.vcall(base + MODEL_3, 0x18, &args![text]);
            }),
            CHUNK_MO2T => {
                e.call(LOAD_MODEL_TEXTURE, &args![base + MODEL_3, file]);
            }
            CHUNK_MO2S => {
                e.call(LOAD_TEXTURE_SWAP, &args![base + MODEL_3, file]);
            }
            CHUNK_ICON => with_chunk_text(e, file, 0, |e, text| {
                e.call(SET_ICON, &args![base + 0xfc, text]);
            }),
            CHUNK_MICO => with_chunk_text(e, file, 0, |e, text| {
                e.call(SET_MESSAGE_ICON, &args![base + 0x114, text]);
            }),
            CHUNK_MIC2 => with_chunk_text(e, file, 0, |e, text| {
                e.call(SET_MESSAGE_ICON, &args![base + 0x124, text]);
            }),
            CHUNK_MOD3 => with_chunk_text(e, file, 0, |e, text| {
                e.vcall(base + MODEL_2, 0x18, &args![text]);
            }),
            CHUNK_MO3T => {
                e.call(LOAD_MODEL_TEXTURE, &args![base + MODEL_2, file]);
            }
            CHUNK_MO3S => {
                e.call(LOAD_TEXTURE_SWAP, &args![base + MODEL_2, file]);
            }
            CHUNK_MOD4 => with_chunk_text(e, file, 0, |e, text| {
                e.vcall(base + MODEL_4, 0x18, &args![text]);
            }),
            CHUNK_MO4T => {
                e.call(LOAD_MODEL_TEXTURE, &args![base + MODEL_4, file]);
            }
            CHUNK_MO4S => {
                e.call(LOAD_TEXTURE_SWAP, &args![base + MODEL_4, file]);
            }
            CHUNK_ICO2 => with_chunk_text(e, file, 0, |e, text| {
                e.call(SET_ICON, &args![base + 0x108, text]);
            }),
            CHUNK_DEST | CHUNK_DSTD => {
                e.call(LOAD_DESTRUCTIBLE, &args![or_null(DESTRUCTIBLE), file]);
            }
            CHUNK_ETYP => {
                let id = e.with_stack(4, |e, slot| {
                    e.call(FILE_CHUNK_DATA, &args![file, slot]);
                    e.mem.u32(slot.addr())
                });
                e.call(0x006e_cd40, &args![base + 0x154, id]);
            }
            CHUNK_DNAM => {
                e.call(MEMSET, &args![base + 0x178, 0u32, 0xcu32]);
                e.call(FILE_CHUNK_DATA_SIZED, &args![file, base + 0x178, 0xcu32]);
                if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
                    e.call(SWAP_DATA, &args![base + 0x178]);
                }
            }
            _ => {}
        }
        if chunk == CHUNK_DATA && e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
            e.call(SWAP_DATA, &args![base + 0x178]);
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    e.call(0x004f_15a0, &args![this, 0x18u32]);
    true
}

/// Byte-swaps a 16-bit word in place (`00407a90`, cdecl(address, byte)).
const SWAP_WORD16: u32 = 0x0040_7a90;
/// `TESForm::StartForm` (`004855a0`), `TESForm::CloseForm` (`00485680`) and
/// `TESForm::SaveData` (`00485e00`), `thiscall` on the form, no stack words.
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
const FORM_SAVE_DATA: u32 = 0x0048_5e00;
/// The component saves (engine map; `thiscall`): `TESFullName::Save`
/// (`00487010`), `TESBipedModelForm::Save` (`00480610`) and
/// `BGSEquipType::Save` (`00479350`).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const BIPED_MODEL_SAVE: u32 = 0x0048_0610;
const EQUIP_TYPE_SAVE: u32 = 0x0047_9350;
/// `00401500`: the byte at `011c54ba` (whether saved words are byte-swapped).
const SAVE_SWAPS_BYTES: u32 = 0x0040_1500;
/// `TESForm::__AddChunkData` (`00485990`, cdecl(chunk, data, size)).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;

// Translated from 00512810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the 16-bit word at `this` (`00407a90` with a zero byte;
/// engine map: no name; no stack words).
pub fn fn_00512810(e: &mut Engine, this: Ptr) {
    e.call(SWAP_WORD16, &args![this, 0u32]);
}

// Translated from 00512840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectARMA::Save` (engine map; no stack words): `StartForm`, the
/// full name (+0x30), the biped model (+0x70), the equip type (+0x154),
/// `TESForm::SaveData`, the 0xc bytes at +0x178 as chunk `DNAM`
/// (byte-swapped before and back after, `00535aa0`, when `00401500` says
/// so), `CloseForm`.
pub fn tes_object_arma_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![base + 0x30]);
    e.call(BIPED_MODEL_SAVE, &args![base + 0x70]);
    e.call(EQUIP_TYPE_SAVE, &args![base + 0x154]);
    e.call(FORM_SAVE_DATA, &args![this]);
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        e.call(SWAP_DATA, &args![base + 0x178]);
    }
    e.call(ADD_CHUNK_DATA, &args![CHUNK_DNAM, base + 0x178, 0xcu32]);
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        e.call(SWAP_DATA, &args![base + 0x178]);
    }
    e.call(CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_2030, tes_object_arma_init_item(Ptr)),
        entry!(0x0051_20a0, fn_005120a0(Ptr, Ptr) -> bool),
        entry!(0x0051_2810, fn_00512810(Ptr)),
        entry!(0x0051_2840, tes_object_arma_save(Ptr)),
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

    /// An addon whose own vtable (+0x130, +0x134, +0xE0) and the vtable
    /// +0x18 of each model component and of +0x134 are doubles (one shared
    /// component vtable: the calls tell the components apart by `this`).
    fn addon(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x190);
        let vtable = e.mem.alloc(0x140);
        for slot in [0x130u32, 0x134, 0xe0] {
            e.mem.set_u32(vtable + slot, 0x7000_0000 + slot);
        }
        e.mem.set_u32(this, vtable);
        let component = e.mem.alloc(0x20);
        e.mem.set_u32(component + 0x18, 0x7100_0018);
        for offset in [MODEL_1, MODEL_2, MODEL_3, MODEL_4, 0x134] {
            e.mem.set_u32(this + offset, component);
        }
        e.register(0x7000_0130, |_, _| ret(0x5555));
        quiet(e, &[0x7000_0134, 0x7000_00e0, 0x7100_0018]);
        this
    }

    #[test]
    fn init_item_warns_without_biped_slots() {
        for (slots, warned) in [(0u32, true), (4, false)] {
            let mut e = Engine::new();
            let this = addon(&mut e);
            e.mem.set_u32(this + 0x74, slots);
            e.register(FORM_INIT_SKIP, |_, _| ret(0));
            quiet(&mut e, &[WARNING, 0x0047_8be0, 0x0048_0750, FORM_SET_FLAG]);
            e.call_log = Some(vec![]);
            e.call(0x0051_2030, &args![this]);
            assert_eq!(
                calls(&e, WARNING),
                if warned {
                    vec![vec![NO_BIPED_SLOTS_FORMAT, 0x5555]]
                } else {
                    vec![]
                }
            );
            assert_eq!(calls(&e, 0x0047_8be0), vec![vec![this + 0x14c, this]]);
            assert_eq!(calls(&e, 0x0048_0750), vec![vec![this + 0x70, this]]);
            assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 1]]);
        }
        let mut e = Engine::new();
        let this = addon(&mut e);
        e.register(FORM_INIT_SKIP, |_, _| ret(1));
        quiet(&mut e, &[FORM_SET_FLAG]);
        e.call_log = Some(vec![]);
        e.call(0x0051_2030, &args![this]);
        assert!(calls(&e, FORM_SET_FLAG).is_empty());
    }

    /// A file whose chunks are `chunks` (type, first data word).
    fn file(
        e: &mut Engine,
        form_type: u32,
        chunks: Vec<(u32, u32)>,
        version: u32,
        swap: bool,
    ) -> u32 {
        let file = e.mem.alloc(0x10);
        let chunks = Rc::new(RefCell::new((chunks, 0usize)));
        e.register_double(FILE_FORM_TYPE, move |_, _| ret(form_type));
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
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_DATA, move |e, a| {
            let c = c.borrow();
            e.mem.set_u32(a[1], c.0[c.1].1);
            Ret::default()
        });
        let c = chunks;
        e.register_double(FILE_CHUNK_DATA_SIZED, move |e, a| {
            let c = c.borrow();
            e.mem.set_u32(a[1], c.0[c.1].1);
            Ret::default()
        });
        e.register(FILE_CHUNK_SIZE, |_, _| ret(8));
        e.register_double(FILE_VERSION, move |_, _| ret(version));
        if swap {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(1));
        } else {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(0));
        }
        quiet(
            e,
            &[
                0x0048_5110,
                FORM_SET_FLAG,
                MEMSET,
                LOAD_MODEL,
                LOAD_MODEL_TEXTURE,
                LOAD_TEXTURE_SWAP,
                LOAD_FULL_NAME,
                LOAD_DESTRUCTIBLE,
                SET_ICON,
                SET_MESSAGE_ICON,
                SWAP_DATA,
                0x0051_2810,
                0x0050_3210,
                0x006e_cd40,
                0x004f_15a0,
            ],
        );
        file
    }

    #[test]
    fn load_sends_each_chunk_to_its_component() {
        let mut e = Engine::new();
        let this = addon(&mut e);
        let chunks = vec![
            (CHUNK_EDID, 0),
            (CHUNK_OBND, 0),
            (CHUNK_FULL, 0),
            (CHUNK_MODL, 0),
            (CHUNK_MODT, 0),
            (CHUNK_MODS, 0),
            (CHUNK_MODD, 0),
            (CHUNK_MOD2, 0),
            (CHUNK_MO2T, 0),
            (CHUNK_MO2S, 0),
            (CHUNK_MOD3, 0),
            (CHUNK_MO3T, 0),
            (CHUNK_MO3S, 0),
            (CHUNK_MOSD, 0),
            (CHUNK_MOD4, 0),
            (CHUNK_MO4T, 0),
            (CHUNK_MO4S, 0),
            (CHUNK_ICON, 0),
            (CHUNK_ICO2, 0),
            (CHUNK_MICO, 0),
            (CHUNK_MIC2, 0),
            (CHUNK_BMCT, 0),
            (CHUNK_DEST, 0),
            (CHUNK_DSTD, 0),
            (CHUNK_ETYP, 0x1234),
            (CHUNK_DNAM, 0),
            (CHUNK_DATA, 0),
        ];
        let f = file(&mut e, 0x60, chunks, 11, true);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0051_20a0, &args![this, f]).bool());
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, f]]);
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, f]]);
        assert_eq!(calls(&e, LOAD_FULL_NAME), vec![vec![this + 0x30, f]]);
        let paths: Vec<u32> = calls(&e, 0x7100_0018).iter().map(|c| c[0]).collect();
        assert_eq!(
            paths,
            vec![
                this + 0x7c,
                this + 0xbc,
                this + 0x9c,
                this + 0xdc,
                this + 0x134
            ]
        );
        let textures: Vec<u32> = calls(&e, LOAD_MODEL_TEXTURE).iter().map(|c| c[0]).collect();
        assert_eq!(
            textures,
            vec![this + 0x7c, this + 0xbc, this + 0x9c, this + 0xdc]
        );
        let swaps: Vec<u32> = calls(&e, LOAD_TEXTURE_SWAP).iter().map(|c| c[0]).collect();
        assert_eq!(
            swaps,
            vec![this + 0x7c, this + 0xbc, this + 0x9c, this + 0xdc]
        );
        assert_eq!(
            calls(&e, LOAD_MODEL),
            vec![vec![this + 0x7c, f], vec![this + 0x9c, f]]
        );
        let icons: Vec<u32> = calls(&e, SET_ICON).iter().map(|c| c[0]).collect();
        assert_eq!(icons, vec![this + 0xfc, this + 0x108]);
        let message_icons: Vec<u32> = calls(&e, SET_MESSAGE_ICON).iter().map(|c| c[0]).collect();
        assert_eq!(message_icons, vec![this + 0x114, this + 0x124]);
        assert_eq!(calls(&e, LOAD_DESTRUCTIBLE), vec![vec![this + 0x14c, f]; 2]);
        assert_eq!(calls(&e, 0x006e_cd40), vec![vec![this + 0x154, 0x1234]]);
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x178, 0, 0xc]]);
        // DNAM and DATA both swap the data.
        assert_eq!(calls(&e, SWAP_DATA), vec![vec![this + 0x178]; 2]);
        let sized = calls(&e, FILE_CHUNK_DATA_SIZED);
        assert_eq!(sized[0][2], 0x200);
        assert!(sized[1..sized.len() - 1].iter().all(|c| c[2] == 0));
        assert_eq!(calls(&e, 0x004f_15a0), vec![vec![this, 0x18]]);
    }

    #[test]
    fn load_reads_both_biped_data_formats() {
        // Format 10: four bytes, low half and third byte; bit 4 cleared.
        let mut e = Engine::new();
        let this = addon(&mut e);
        e.mem.set_u32(this + 0x74, 0xdead_beef);
        let f = file(&mut e, 0x60, vec![(CHUNK_BMDT, 0x0031_0204)], 10, true);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0051_20a0, &args![this, f]).bool());
        assert_eq!(e.mem.u32(this + 0x74), 0x0204);
        assert_eq!(e.mem.u8(this + 0x78), 0x21);
        assert_eq!(calls(&e, 0x0051_2810).len(), 1);
        assert!(calls(&e, 0x0050_3210).is_empty());

        // Format 11: eight bytes in place.
        let mut e = Engine::new();
        let this = addon(&mut e);
        e.mem.set_u8(this + 0x78, 0xff);
        let f = file(&mut e, 0x60, vec![(CHUNK_BMDT, 0x0000_0402)], 11, true);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0051_20a0, &args![this, f]).bool());
        assert_eq!(e.mem.u32(this + 0x74), 0x0402);
        assert_eq!(e.mem.u8(this + 0x78), 0xef);
        assert_eq!(calls(&e, 0x0050_3210), vec![vec![this + 0x74]]);

        // Another record type: false at once.
        let mut e = Engine::new();
        let this = addon(&mut e);
        let f = file(&mut e, 0x61, vec![(CHUNK_BMDT, 0)], 11, false);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0051_20a0, &args![this, f]).bool());
        assert!(calls(&e, 0x004f_15a0).is_empty());
    }
}

#[cfg(test)]
mod save_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn swap_passes_this_and_a_zero_byte() {
        let mut e = Engine::new();
        e.register(SWAP_WORD16, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0051_2810, &args![0x4400u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, SWAP_WORD16), vec![vec![0x4400, 0]]);
    }

    #[test]
    fn save_writes_components_and_dnam() {
        for swaps in [0u32, 1] {
            let mut e = Engine::new();
            for a in [
                START_FORM,
                CLOSE_FORM,
                FORM_SAVE_DATA,
                FULL_NAME_SAVE,
                BIPED_MODEL_SAVE,
                EQUIP_TYPE_SAVE,
                ADD_CHUNK_DATA,
                SWAP_DATA,
            ] {
                e.register(a, |_, _| Ret::default());
            }
            e.register_double(SAVE_SWAPS_BYTES, move |_, _| swaps.into_ret());
            let this = e.mem.alloc(0x190);
            e.call_log = Some(vec![]);
            e.call(0x0051_2840, &args![this]);
            let log = e.call_log.take().unwrap();
            assert_eq!(only(&log, FULL_NAME_SAVE), vec![vec![this + 0x30]]);
            assert_eq!(only(&log, BIPED_MODEL_SAVE), vec![vec![this + 0x70]]);
            assert_eq!(only(&log, EQUIP_TYPE_SAVE), vec![vec![this + 0x154]]);
            assert_eq!(only(&log, FORM_SAVE_DATA), vec![vec![this]]);
            assert_eq!(
                only(&log, ADD_CHUNK_DATA),
                vec![vec![CHUNK_DNAM, this + 0x178, 0xc]]
            );
            assert_eq!(only(&log, SWAP_DATA).len(), 2 * swaps as usize);
            assert_eq!(only(&log, CLOSE_FORM), vec![vec![this]]);
        }
    }
}
