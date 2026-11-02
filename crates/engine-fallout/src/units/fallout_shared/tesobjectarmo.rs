//! `fallout shared/tesobjectarmo.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESObjectARMO` (vtable `0102a62c`, RTTI `.?AVTESObjectARMO@@`, form type
//! 0x18). Its components sit at the offsets the constructor `005129f0`
//! gives them; the ones used here: `TESFullName` `+0x30`,
//! `TESScriptableForm` `+0x3C`, `TESEnchantableForm` `+0x48`,
//! `TESBipedModelForm` `+0x70` (flags word `+0x74`, flags byte `+0x78`,
//! models `+0x7C`, `+0x9C`, `+0xBC`, `+0xDC`, icons `+0xFC`, `+0x108`,
//! message icons `+0x114`, `+0x124`, constraint template `+0x134`),
//! `BGSDestructibleObjectForm` `+0x14C`, and the words at `+0x154`,
//! `+0x15C`, `+0x164`, `+0x16C`. The armor's data is at `+0x178` (0xC
//! bytes), the list or template at `+0x188` and its flag byte at `+0x18C`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESFile::GetTESForm` (Xbox PDB, `00472660`): the record's form type.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, `RET 4`).
const LOAD_FORM: u32 = 0x0048_5110;
/// Sets (`true`) or clears form flag 0x8 (`00484ab0`, `RET 4`).
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// Tests form flag 0x8 (`004013e0`).
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The size of the current chunk (`00401660`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`): buffer and size.
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (Xbox PDB, `004727f0`): one word.
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov2` (Xbox PDB, `00472840`): one 16-bit word.
const GET_CHUNK_DATA_U16: u32 = 0x0047_2840;
/// Moves to the next chunk (`004726f0`, `bool`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// Whether the file's data needs its byte order swapped (`00401680`).
const SWAP_BYTES: u32 = 0x0040_1680;
/// The file's form version (`00403570`, a 16-bit word).
const FORM_VERSION: u32 = 0x0040_3570;
/// `TESForm::LoadData` (`004861f0`, `RET 0xC`): file, buffer, size.
const LOAD_DATA: u32 = 0x0048_61f0;
/// A `memset` wrapper (`00403d30`, cdecl; the engine map's `std::_Fiopen`
/// name is a folded body's).
const MEMSET: u32 = 0x0040_3d30;
/// The setter the form-id chunks go to (`006ecd40`, `RET 4`; the engine map
/// names the folded body `CombatProcedureAttackMelee::Initialize`).
const SET_FORM: u32 = 0x006e_cd40;
/// The `MemoryManager` allocation wrapper (`00401000`, cdecl).
const ALLOCATE: u32 = 0x0040_1000;
/// A `BSSimpleList` node's item address (`006815c0`).
const NODE_ITEM: u32 = 0x0068_15c0;
/// A node's next node (`00726070`).
const NODE_NEXT: u32 = 0x0072_6070;
/// Looks up a form by id (`004839c0`, cdecl, one word).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// The warning printer (`005b5e40`, cdecl, variable arguments).
const WARNING: u32 = 0x005b_5e40;
/// "MASTERFILE: Armor '%s' needs to have biped slots selected in the
/// editor."
const NO_BIPED_SLOTS_MESSAGE: u32 = 0x0102_a4a0;
/// The data's byte-order swap (`00535aa0`, on `+0x178`).
const SWAP_DATA: u32 = 0x0053_5aa0;

/// Chunk signatures, as the words the exe compares.
const SIG_ANAM: u32 = 0x4d41_4e41;
const SIG_BIPL: u32 = 0x4c50_4942;
const SIG_BMCT: u32 = 0x5443_4d42;
const SIG_BMDT: u32 = 0x5444_4d42;
const SIG_BNAM: u32 = 0x4d41_4e42;
const SIG_DATA: u32 = 0x4154_4144;
const SIG_DEST: u32 = 0x5453_4544;
const SIG_DNAM: u32 = 0x4d41_4e44;
const SIG_DSTD: u32 = 0x4454_5344;
const SIG_EAMT: u32 = 0x544d_4145;
const SIG_EDID: u32 = 0x4449_4445;
const SIG_EITM: u32 = 0x4d54_4945;
const SIG_ENAM: u32 = 0x4d41_4e45;
const SIG_ETYP: u32 = 0x5059_5445;
const SIG_FULL: u32 = 0x4c4c_5546;
const SIG_ICO2: u32 = 0x324f_4349;
const SIG_ICON: u32 = 0x4e4f_4349;
const SIG_MIC2: u32 = 0x3243_494d;
const SIG_MICO: u32 = 0x4f43_494d;
const SIG_MO2S: u32 = 0x5332_4f4d;
const SIG_MO2T: u32 = 0x5432_4f4d;
const SIG_MO3S: u32 = 0x5333_4f4d;
const SIG_MO3T: u32 = 0x5433_4f4d;
const SIG_MO4S: u32 = 0x5334_4f4d;
const SIG_MO4T: u32 = 0x5434_4f4d;
const SIG_MOD2: u32 = 0x3244_4f4d;
const SIG_MOD3: u32 = 0x3344_4f4d;
const SIG_MOD4: u32 = 0x3444_4f4d;
const SIG_MODD: u32 = 0x4444_4f4d;
const SIG_MODL: u32 = 0x4c44_4f4d;
const SIG_MODS: u32 = 0x5344_4f4d;
const SIG_MODT: u32 = 0x5444_4f4d;
const SIG_MOSD: u32 = 0x4453_4f4d;
const SIG_OBND: u32 = 0x444e_424f;
const SIG_REPL: u32 = 0x4c50_4552;
const SIG_SCRI: u32 = 0x4952_4353;
const SIG_SNAM: u32 = 0x4d41_4e53;
const SIG_TNAM: u32 = 0x4d41_4e54;
const SIG_YNAM: u32 = 0x4d41_4e59;
const SIG_ZNAM: u32 = 0x4d41_4e5a;

/// A word read from the chunk into a cleared local.
fn read_word(e: &mut Engine, file: Ptr, local: Ptr) -> u32 {
    e.mem.set_u32(local.addr(), 0);
    e.call(GET_CHUNK_DATA, &args![file, local]);
    e.mem.u32(local.addr())
}

/// The whole chunk read into a stack buffer of its size (`__alloca_probe_16`,
/// `00ec5ec0`; read with size 0), then given to `use_buffer`.
fn read_buffer(e: &mut Engine, file: Ptr, use_buffer: impl FnOnce(&mut Engine, Ptr)) {
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    e.with_stack(size, |e, buffer| {
        e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, 0u32]);
        use_buffer(e, buffer);
    });
}

// Translated from 00512e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectARMO::InitItem` (Xbox PDB): unless form flag 0x8 is set
/// (`004013e0`): an armor without biped slots (the word at `+0x74`) is
/// reported with its name (vtable slot 0x130); then the components'
/// `InitItem` with the armor (`0048cd90` on `+0x3C`, `00483210` on
/// `+0x48`, `00478be0` on `+0x14C`, `TESBipedModelForm::InitItem`
/// `00480750` on `+0x70`, `0047b8e0` on `+0x15C`, `00474da0` on `+0x164`,
/// `0047b6a0` on `+0x16C`). Without the flag byte at `+0x18C`, a word at
/// `+0x188` that is not zero is looked up as a form id (`004839c0`) and
/// replaced; with it, every entry of the list at `+0x188` whose item is
/// not null (the walk stops at the first node with a null item) has the
/// form id at its `+0` looked up and replaced when not zero. Finally form
/// flag 0x8 is set.
pub fn tes_object_armo_init_item(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    if e.mem.u32(base + 0x74) == 0 {
        let name = e.vcall(base, 0x130, &args![]).u32();
        e.call(WARNING, &args![NO_BIPED_SLOTS_MESSAGE, name]);
    }
    for (callee, offset) in [
        (0x0048_cd90, 0x3c),
        (0x0048_3210, 0x48),
        (0x0047_8be0, 0x14c),
        (0x0048_0750, 0x70),
        (0x0047_b8e0, 0x15c),
        (0x0047_4da0, 0x164),
        (0x0047_b6a0, 0x16c),
    ] {
        e.call(callee, &args![base + offset, this]);
    }
    let has_list = e.mem.u8(base + 0x18c) != 0;
    if !has_list && e.mem.u32(base + 0x188) != 0 {
        let id = e.mem.u32(base + 0x188);
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        e.mem.set_u32(base + 0x188, form);
    } else if has_list && e.mem.u32(base + 0x188) != 0 {
        let mut node = e.mem.u32(base + 0x188);
        while node != 0 {
            let item_at = e.call(NODE_ITEM, &args![node]).u32();
            if e.mem.u32(item_at) == 0 {
                break;
            }
            let item_at = e.call(NODE_ITEM, &args![node]).u32();
            let entry = e.mem.u32(item_at);
            node = e.call(NODE_NEXT, &args![node]).u32();
            if e.mem.u32(entry) != 0 {
                let id = e.mem.u32(entry);
                let form = e.call(LOOKUP_FORM, &args![id]).u32();
                e.mem.set_u32(entry, form);
            }
        }
    }
    e.call(SET_FORM_FLAG_8, &args![this, true]);
}

// Translated from 00512fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00512fd0 - the exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated; the __alloca_probe_16 (00ec5ec0) buffers are with_stack blocks of the chunk size (their 16-byte alignment padding, and the stack growing with each such chunk, are not modelled)
/// `TESObjectARMO`'s `Load` (`RET 4`): false unless the record is of form
/// type 0x18. Then `TESForm::LoadForm` (Xbox PDB, `00485110`), form flag
/// 0x8 cleared, and every chunk of `file`:
/// - `EDID` (vtable slot 0x134, read with at most 0x200 bytes), `OBND`
///   (slot 0xE0), `FULL` (`00487050` on `+0x30`, null when `this` is),
///   `DSTD`/`DEST` (`004781e0` on `+0x14C`, the same);
/// - form ids (each read into a cleared local) given to `006ecd40`: `SCRI`
///   (`+0x3C`, then `0048cd90` with the armor), `EITM` and `ENAM` (`+0x48`),
///   `REPL` (`+0x15C`), `BIPL` (`+0x164`), `ETYP` (`+0x154`), `YNAM`
///   (`+0x16C`); `ZNAM` goes to `00403550` on `+0x16C`; `ANAM` and `EAMT`
///   are 16-bit words (`00472840`) given to `00483170` on `+0x48`;
/// - the four models (`+0x7C`, `+0xBC`, `+0x9C`, `+0xDC`): `MODL`, `MOD2`,
///   `MOD3`, `MOD4` read whole and given to the model's vtable slot 0x18;
///   `MODT`/`MO2T`/`MO3T`/`MO4T` to `004893e0`; `MODS`/`MO2S`/`MO3S`/`MO4S`
///   to `0048a990`; `MODD` (`+0x7C`) and `MOSD` (`+0x9C`) to
///   `TESModel::LoadModelChunk` (`004892d0`); `BMCT` (`+0x134`, slot 0x18);
///   `ICON`/`ICO2` to `00489100` on `+0xFC`/`+0x108`; `MICO`/`MIC2` to
///   `0047a380` on `+0x114`/`+0x124`;
/// - `BMDT`: before form version 0xB, 4 bytes (swapped by `00512810` when
///   the file says so, `00401680`) whose first 16 bits become the slots
///   word `+0x74` and whose third byte the flags byte `+0x78`; from version
///   0xB on, 8 bytes straight into `+0x74` (swapped by `00503210`); then bit
///   0x10 of `+0x78` is cleared;
/// - `DATA`: when the chunk is 2 bytes longer than `00486620` says, 2
///   zeroed bytes are read (`TESForm::LoadData`, `004861f0`), swapped by
///   `00512810` when needed, copied to `+0x178` (`00401460`) and the byte
///   at `+0x180` cleared; otherwise `004861f0` with no buffer. Either way
///   the data at `+0x178` is swapped (`00535aa0`) when needed;
/// - `DNAM`: 0xC bytes into `+0x178` after clearing them, swapped by
///   `00535aa0` when needed;
/// - `BNAM`: the flag byte `+0x18C` (1 when the word is not zero);
///   `TNAM`: without that flag, a word stored at `+0x188`;
/// - `SNAM`: with the flag, a new 0xC-byte entry (`00513d60`) loaded by
///   `004861f0` (and `00483710` on it when swapping) is appended
///   (`005ae3d0`) to the list at `+0x188`, which is made first (8 bytes,
///   `0096a2d0`) when missing.
///
/// `CNAM` and other chunks are skipped. Returns true.
pub fn fn_00512fd0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u8() != 0x18 {
        return false;
    }
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, false]);
    e.with_stack(8, |e, locals| {
        // One word for the chunk words (each cleared before its read), and
        // `[EBP-0x28]` (`BMDT`, not cleared), `[EBP-0x74]` (`DATA`) and
        // `[EBP-0x6C]` (`SNAM`) in the second.
        let word_at = locals;
        let small_at = locals.byte_add(4);
        loop {
            let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
            if chunk == 0 {
                break;
            }
            match chunk {
                SIG_SNAM => {
                    if e.mem.u8(base + 0x18c) != 0 {
                        if e.mem.u32(base + 0x188) == 0 {
                            let block = e.call(ALLOCATE, &args![8u32]).u32();
                            let list = if block != 0 {
                                e.call(0x0096_a2d0, &args![block]).u32()
                            } else {
                                0
                            };
                            e.mem.set_u32(base + 0x188, list);
                        }
                        let block = e.call(ALLOCATE, &args![0xcu32]).u32();
                        let entry = if block != 0 {
                            fn_00513d60(e, Ptr::new(block)).addr()
                        } else {
                            0
                        };
                        e.mem.set_u32(small_at.addr(), entry);
                        e.call(LOAD_DATA, &args![this, file, entry, 0xcu32]);
                        if e.call(SWAP_BYTES, &args![file]).bool() {
                            let entry = e.mem.u32(small_at.addr());
                            e.call(0x0048_3710, &args![entry]);
                        }
                        let list = e.mem.u32(base + 0x188);
                        e.call(0x005a_e3d0, &args![list, small_at]);
                    }
                }
                SIG_DSTD | SIG_DEST => {
                    let component = if base != 0 { base + 0x14c } else { 0 };
                    e.call(0x0047_81e0, &args![component, file]);
                }
                SIG_DATA => {
                    let expected = e.call(0x0048_6620, &args![this]).u32();
                    let size = e.call(CHUNK_SIZE, &args![file]).u32();
                    if size.wrapping_sub(expected) == 2 {
                        e.call(MEMSET, &args![small_at, 0u32, 2u32]);
                        e.call(LOAD_DATA, &args![this, file, small_at, 2u32]);
                        if e.call(SWAP_BYTES, &args![file]).bool() {
                            e.call(0x0051_2810, &args![small_at]);
                        }
                        e.call(0x0040_1460, &args![base + 0x178, small_at, 2u32]);
                        e.mem.set_u8(base + 0x180, 0);
                    } else {
                        e.call(LOAD_DATA, &args![this, file, 0u32, 0u32]);
                    }
                }
                SIG_ICO2 => read_buffer(e, file, |e, buffer| {
                    e.call(0x0048_9100, &args![base + 0x108, buffer]);
                }),
                SIG_ICON => read_buffer(e, file, |e, buffer| {
                    e.call(0x0048_9100, &args![base + 0xfc, buffer]);
                }),
                SIG_MIC2 => read_buffer(e, file, |e, buffer| {
                    e.call(0x0047_a380, &args![base + 0x124, buffer]);
                }),
                SIG_MICO => read_buffer(e, file, |e, buffer| {
                    e.call(0x0047_a380, &args![base + 0x114, buffer]);
                }),
                SIG_MODL | SIG_MOD2 | SIG_MOD3 | SIG_MOD4 | SIG_BMCT => {
                    let model = match chunk {
                        SIG_MODL => 0x7c,
                        SIG_MOD2 => 0xbc,
                        SIG_MOD3 => 0x9c,
                        SIG_MOD4 => 0xdc,
                        _ => 0x134,
                    };
                    read_buffer(e, file, |e, buffer| {
                        e.vcall(base + model, 0x18, &args![buffer]);
                    });
                }
                SIG_MODT | SIG_MO2T | SIG_MO3T | SIG_MO4T => {
                    let model = match chunk {
                        SIG_MODT => 0x7c,
                        SIG_MO2T => 0xbc,
                        SIG_MO3T => 0x9c,
                        _ => 0xdc,
                    };
                    e.call(0x0048_93e0, &args![base + model, file]);
                }
                SIG_MODS | SIG_MO2S | SIG_MO3S | SIG_MO4S => {
                    let model = match chunk {
                        SIG_MODS => 0x7c,
                        SIG_MO2S => 0xbc,
                        SIG_MO3S => 0x9c,
                        _ => 0xdc,
                    };
                    e.call(0x0048_a990, &args![base + model, file]);
                }
                SIG_MODD => {
                    e.call(0x0048_92d0, &args![base + 0x7c, file]);
                }
                SIG_MOSD => {
                    e.call(0x0048_92d0, &args![base + 0x9c, file]);
                }
                SIG_OBND => {
                    e.vcall(base, 0xe0, &args![file]);
                }
                SIG_EDID => {
                    let size = e.call(CHUNK_SIZE, &args![file]).u32();
                    e.with_stack(size, |e, buffer| {
                        e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, 0x200u32]);
                        e.vcall(base, 0x134, &args![buffer]);
                    });
                }
                SIG_FULL => {
                    let component = if base != 0 { base + 0x30 } else { 0 };
                    e.call(0x0048_7050, &args![component, file]);
                }
                SIG_SCRI => {
                    let script = read_word(e, file, word_at);
                    e.call(SET_FORM, &args![base + 0x3c, script]);
                    e.call(0x0048_cd90, &args![base + 0x3c, this]);
                }
                SIG_EITM | SIG_ENAM | SIG_REPL | SIG_BIPL | SIG_ETYP | SIG_YNAM => {
                    let component = match chunk {
                        SIG_EITM | SIG_ENAM => 0x48,
                        SIG_REPL => 0x15c,
                        SIG_BIPL => 0x164,
                        SIG_ETYP => 0x154,
                        _ => 0x16c,
                    };
                    let form = read_word(e, file, word_at);
                    e.call(SET_FORM, &args![base + component, form]);
                }
                SIG_ZNAM => {
                    let sound = read_word(e, file, word_at);
                    e.call(0x0040_3550, &args![base + 0x16c, sound]);
                }
                SIG_ANAM | SIG_EAMT => {
                    e.mem.set_u16(word_at.addr(), 0);
                    e.call(GET_CHUNK_DATA_U16, &args![file, word_at]);
                    let amount = e.mem.u16(word_at.addr());
                    e.call(0x0048_3170, &args![base + 0x48, u32::from(amount)]);
                }
                SIG_BNAM => {
                    let has_list = read_word(e, file, word_at) != 0;
                    e.mem.set_u8(base + 0x18c, u8::from(has_list));
                }
                SIG_TNAM => {
                    if e.mem.u8(base + 0x18c) == 0 {
                        let template = read_word(e, file, word_at);
                        e.mem.set_u32(base + 0x188, template);
                    }
                }
                SIG_DNAM => {
                    e.call(MEMSET, &args![base + 0x178, 0u32, 0xcu32]);
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, base + 0x178, 0xcu32]);
                    if e.call(SWAP_BYTES, &args![file]).bool() {
                        e.call(SWAP_DATA, &args![base + 0x178]);
                    }
                }
                SIG_BMDT => {
                    let version = e.call(FORM_VERSION, &args![file]).u16();
                    if version < 0xb {
                        e.call(GET_CHUNK_DATA_SIZED, &args![file, small_at, 4u32]);
                        if e.call(SWAP_BYTES, &args![file]).bool() {
                            e.call(0x0051_2810, &args![small_at]);
                        }
                        let flags = e.mem.u8(small_at.addr() + 2);
                        e.mem.set_u8(base + 0x78, flags);
                        let slots = e.mem.u16(small_at.addr());
                        e.mem.set_u32(base + 0x74, u32::from(slots));
                    } else {
                        e.call(GET_CHUNK_DATA_SIZED, &args![file, base + 0x74, 8u32]);
                        if e.call(SWAP_BYTES, &args![file]).bool() {
                            e.call(0x0050_3210, &args![base + 0x74]);
                        }
                    }
                    let flags = e.mem.u8(base + 0x78);
                    e.mem.set_u8(base + 0x78, flags & 0xef);
                }
                _ => {}
            }
            if chunk == SIG_DATA && e.call(SWAP_BYTES, &args![file]).bool() {
                e.call(SWAP_DATA, &args![base + 0x178]);
            }
            if !e.call(NEXT_CHUNK, &args![file]).bool() {
                break;
            }
        }
    });
    true
}

// Translated from 00513d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears a 0xC-byte entry (the ones `Load` appends for `SNAM`): the words
/// at `+0` and `+8` and the byte at `+4`. Returns `this`.
pub fn fn_00513d60(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u8(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 8, 0);
    this
}

/// `TESForm::StartForm` (`004855a0`), `TESForm::CloseForm` (`00485680`) and
/// `TESForm::SaveData` (`00485e00`), `thiscall` on the form, no stack words.
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
const FORM_SAVE_DATA: u32 = 0x0048_5e00;
/// The component saves (engine map; `thiscall`, no stack words):
/// `TESFullName::Save` (`00487010`), `TESScriptableForm::Save` (`0048cca0`),
/// `TESEnchantableForm::Save` (`0047bd70`), `TESBipedModelForm::Save`
/// (`00480610`), `BGSDestructibleObjectForm::Save` (`00477f20`),
/// `BGSRepairItemList::Save` (`0047b8a0`), `BGSBipedModelList::Save`
/// (`00474d60`), `BGSEquipType::Save` (`00479350`) and
/// `BGSPickupPutdownSounds::Save` (`0047b560`).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const SCRIPTABLE_FORM_SAVE: u32 = 0x0048_cca0;
const ENCHANTABLE_SAVE: u32 = 0x0047_bd70;
const BIPED_MODEL_SAVE: u32 = 0x0048_0610;
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
const REPAIR_LIST_SAVE: u32 = 0x0047_b8a0;
const BIPED_MODEL_LIST_SAVE: u32 = 0x0047_4d60;
const EQUIP_TYPE_SAVE: u32 = 0x0047_9350;
const PICKUP_PUTDOWN_SAVE: u32 = 0x0047_b560;
/// `00401500`: the byte at `011c54ba` (whether saved words are byte-swapped).
const SAVE_SWAPS_BYTES: u32 = 0x0040_1500;
/// `TESForm::__AddChunkData` (`00485990`, cdecl(chunk, data, size)) and
/// `00485910` (cdecl(chunk, word)): adds a one-word chunk.
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// The form-id getter (`0084e3a0`, `thiscall`: the word at +0xc).
const FORM_ID: u32 = 0x0084_e3a0;
/// The 0xc-byte sound record `Save` writes as `SNAM`: its constructor
/// (`00513d60`, `thiscall`: clears its word +0, byte +4 and word +8) and
/// its byte swap (`00483710`, `thiscall`; its body does nothing).
const SOUND_RECORD_CONSTRUCT: u32 = 0x0051_3d60;
const SOUND_RECORD_SWAP: u32 = 0x0048_3710;

// Translated from 00513d90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00513d90 - the SNAM record's padding bytes 5..7, which the exe writes with the chunk from its uninitialised stack, are 0 here.
/// `TESObjectARMO::Save` (engine map; no stack words): `StartForm`, the
/// full name (+0x30), script (+0x3c), enchantment (+0x48), biped model
/// (+0x70), destructible data (+0x14c), repair list (+0x15c), biped model
/// list (+0x164), equip type (+0x154) and pickup/putdown sounds (+0x16c),
/// `TESForm::SaveData`, the 0xc bytes at +0x178 as `DNAM` (byte-swapped
/// before and back after, `00535aa0`, when `00401500` says so), then `BNAM`
/// (1 when the byte at +0x18c is set, else 0). Without that byte, the form
/// at +0x188 by id as `TNAM`; with it, +0x188 is a list of sound entries:
/// for each node until the first null item, an entry with a form gets a
/// record (`00513d60`) of the form's id, the entry's byte +4 and word +8,
/// written as `SNAM` (0xc bytes, `00483710` before and after when
/// `00401500` says so). `CloseForm`. The record the game keeps on its stack
/// is a zeroed block here, so its three padding bytes (5..7, written with
/// the chunk) are 0 where the exe writes whatever its stack held.
pub fn tes_object_armo_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    for (offset, save) in [
        (0x30u32, FULL_NAME_SAVE),
        (0x3c, SCRIPTABLE_FORM_SAVE),
        (0x48, ENCHANTABLE_SAVE),
        (0x70, BIPED_MODEL_SAVE),
        (0x14c, DESTRUCTIBLE_SAVE),
        (0x15c, REPAIR_LIST_SAVE),
        (0x164, BIPED_MODEL_LIST_SAVE),
        (0x154, EQUIP_TYPE_SAVE),
        (0x16c, PICKUP_PUTDOWN_SAVE),
    ] {
        e.call(save, &args![base + offset]);
    }
    e.call(FORM_SAVE_DATA, &args![this]);
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        e.call(SWAP_DATA, &args![base + 0x178]);
    }
    e.call(ADD_CHUNK_DATA, &args![SIG_DNAM, base + 0x178, 0xcu32]);
    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
        e.call(SWAP_DATA, &args![base + 0x178]);
    }
    let is_list = u32::from(e.mem.u8(base + 0x18c) != 0);
    e.call(ADD_CHUNK_WORD, &args![SIG_BNAM, is_list]);
    if e.mem.u8(base + 0x18c) == 0 {
        let form = e.mem.u32(base + 0x188);
        if form != 0 {
            let id = e.call(FORM_ID, &args![form]).u32();
            e.call(ADD_CHUNK_WORD, &args![SIG_TNAM, id]);
        }
    } else if e.mem.u32(base + 0x188) != 0 {
        let mut node = e.mem.u32(base + 0x188);
        e.with_stack(0xc, |e, record| {
            while node != 0 {
                let at = e.call(NODE_ITEM, &args![node]).u32();
                if e.mem.u32(at) == 0 {
                    break;
                }
                let at = e.call(NODE_ITEM, &args![node]).u32();
                let entry = e.mem.u32(at);
                node = e.call(NODE_NEXT, &args![node]).u32();
                if entry != 0 && e.mem.u32(entry) != 0 {
                    e.call(SOUND_RECORD_CONSTRUCT, &args![record]);
                    let form = e.mem.u32(entry);
                    let id = e.call(FORM_ID, &args![form]).u32();
                    e.mem.set_u32(record.addr(), id);
                    let byte = e.mem.u8(entry + 4);
                    e.mem.set_u8(record.addr() + 4, byte);
                    let word = e.mem.u32(entry + 8);
                    e.mem.set_u32(record.addr() + 8, word);
                    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
                        e.call(SOUND_RECORD_SWAP, &args![record]);
                    }
                    e.call(ADD_CHUNK_DATA, &args![SIG_SNAM, record, 0xcu32]);
                    if e.call(SAVE_SWAPS_BYTES, &[]).u32() as u8 != 0 {
                        e.call(SOUND_RECORD_SWAP, &args![record]);
                    }
                }
            }
        });
    }
    e.call(CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_2e60, tes_object_armo_init_item(Ptr)),
        entry!(0x0051_2fd0, fn_00512fd0(Ptr, Ptr) -> bool),
        entry!(0x0051_3d60, fn_00513d60(Ptr) -> Ptr),
        entry!(0x0051_3d90, tes_object_armo_save(Ptr)),
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

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    #[test]
    fn entry_is_cleared() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0xc);
        e.mem.write(p, &[0xcd; 0xc]);
        assert_eq!(e.call(0x0051_3d60, &[p]).u32(), p);
        assert_eq!(
            e.mem.bytes(p, 0xc),
            vec![0, 0, 0, 0, 0, 0xcd, 0xcd, 0xcd, 0, 0, 0, 0]
        );
    }

    const COMPONENT_INITS: [u32; 7] = [
        0x0048_cd90,
        0x0048_3210,
        0x0047_8be0,
        0x0048_0750,
        0x0047_b8e0,
        0x0047_4da0,
        0x0047_b6a0,
    ];

    #[test]
    fn init_item_resolves_the_template_or_the_list() {
        let mut e = Engine::new();
        e.register(FORM_FLAG_8_SET, |_, _| ret(1));
        let this = e.mem.alloc(0x190);
        e.call_log = Some(vec![]);
        e.call(0x0051_2e60, &[this]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);

        e.register(FORM_FLAG_8_SET, |_, _| ret(0));
        quiet(&mut e, &COMPONENT_INITS);
        quiet(&mut e, &[WARNING, SET_FORM_FLAG_8]);
        e.register(0x7000_0000, |_, _| ret(0x0102_0304));
        let mut slots = vec![0u32; 0x134 / 4];
        slots[0x130 / 4] = 0x7000_0000;
        e.put_vtable(0x0600_0000, &slots);
        e.mem.set_u32(this, 0x0600_0000);
        e.register(LOOKUP_FORM, |_, a| ret(a[0] + 0x1000));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        // No biped slots: reported. A template id: looked up.
        e.mem.set_u32(this + 0x188, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x0051_2e60, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, WARNING),
            vec![vec![NO_BIPED_SLOTS_MESSAGE, 0x0102_0304]]
        );
        let inits: Vec<u32> = log
            .iter()
            .filter(|(a, _)| COMPONENT_INITS.contains(a))
            .map(|(_, w)| {
                assert_eq!(w[1], this);
                w[0] - this
            })
            .collect();
        assert_eq!(inits, vec![0x3c, 0x48, 0x14c, 0x70, 0x15c, 0x164, 0x16c]);
        assert_eq!(e.mem.u32(this + 0x188), 0x1042);
        assert_eq!(calls_to(&log, SET_FORM_FLAG_8), vec![vec![this, 1]]);

        // A list: entries with ids 5, 0 and 7; a node with a null item
        // stops the walk before the last entry (id 9).
        e.mem.set_u32(this + 0x74, 1);
        e.mem.set_u8(this + 0x18c, 1);
        let entries: Vec<u32> = [5u32, 0, 7, 9]
            .iter()
            .map(|id| {
                let p = e.mem.alloc(0xc);
                e.mem.set_u32(p, *id);
                p
            })
            .collect();
        let nodes = e.mem.alloc(0x28);
        let items = [entries[0], entries[1], entries[2], 0, entries[3]];
        for (i, item) in items.iter().enumerate() {
            let node = nodes + 8 * i as u32;
            e.mem.set_u32(node, *item);
            let next = if i + 1 < items.len() { node + 8 } else { 0 };
            e.mem.set_u32(node + 4, next);
        }
        e.mem.set_u32(this + 0x188, nodes);
        e.call_log = Some(vec![]);
        e.call(0x0051_2e60, &[this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, WARNING).is_empty());
        assert_eq!(calls_to(&log, LOOKUP_FORM), vec![vec![5], vec![7]]);
        assert_eq!(e.mem.u32(entries[0]), 0x1005);
        assert_eq!(e.mem.u32(entries[1]), 0);
        assert_eq!(e.mem.u32(entries[2]), 0x1007);
        assert_eq!(e.mem.u32(entries[3]), 9);
        assert_eq!(e.mem.u32(this + 0x188), nodes);
    }

    /// Doubles for `Load`: chunks popped from the end of `chunks`, word
    /// reads giving 0x500 + the read's number, chunk sizes of 8.
    fn load_doubles(e: &mut Engine, chunks: Vec<u32>, swap: bool) {
        let chunks = Rc::new(RefCell::new(chunks));
        let c = chunks.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(c.borrow_mut().pop().unwrap_or(0))
        });
        let c = chunks.clone();
        e.register_double(NEXT_CHUNK, move |_, _| {
            ret(u32::from(!c.borrow().is_empty()))
        });
        e.register(GET_TES_FORM, |_, _| ret(0x18));
        e.register(CHUNK_SIZE, |_, _| ret(8));
        let n = Rc::new(RefCell::new(0x500u32));
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            let mut k = n.borrow_mut();
            *k += 1;
            e.mem.set_u32(a[1], *k);
            Ret::default()
        });
        e.register(GET_CHUNK_DATA_U16, |e, a| {
            assert_eq!(e.mem.u16(a[1]), 0);
            e.mem.set_u16(a[1], 0x77);
            Ret::default()
        });
        e.register(GET_CHUNK_DATA_SIZED, |e, a| {
            if a[2] == 4 {
                e.mem.set_u32(a[1], 0x00ab_1234);
            }
            Ret::default()
        });
        e.register_double(SWAP_BYTES, move |_, _| ret(u32::from(swap)));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x0096_a2d0, |_, a| ret(a[0]));
        e.register(LOAD_DATA, |e, a| {
            if a[3] == 0xc {
                e.mem.set_u32(a[2], 0x99);
            }
            Ret::default()
        });
        quiet(
            e,
            &[
                LOAD_FORM,
                SET_FORM_FLAG_8,
                0x7000_0000,
                0x7000_0004,
                0x7000_0018,
                0x0047_81e0,
                0x0048_7050,
                0x0048_9100,
                0x0047_a380,
                0x0048_93e0,
                0x0048_a990,
                0x0048_92d0,
                SET_FORM,
                0x0048_cd90,
                0x0040_3550,
                0x0048_3170,
                MEMSET,
                SWAP_DATA,
                0x0051_2810,
                0x0050_3210,
                0x0048_3710,
                0x005a_e3d0,
                0x0040_1460,
            ],
        );
    }

    /// An armor whose vtable has `+0x134` at `0x7000_0000` and `+0xE0` at
    /// `0x7000_0004`, and whose models (`+0x7C`, `+0x9C`, `+0xBC`, `+0xDC`,
    /// `+0x134`) share a vtable with slot 0x18 at `0x7000_0018`.
    fn armor(e: &mut Engine) -> u32 {
        let mut slots = vec![0u32; 0x138 / 4];
        slots[0x134 / 4] = 0x7000_0000;
        slots[0xe0 / 4] = 0x7000_0004;
        e.put_vtable(0x0600_0000, &slots);
        e.put_vtable(0x0610_0000, &[0, 0, 0, 0, 0, 0, 0x7000_0018]);
        let this = e.mem.alloc(0x190);
        e.mem.set_u32(this, 0x0600_0000);
        for model in [0x7c, 0x9c, 0xbc, 0xdc, 0x134] {
            e.mem.set_u32(this + model, 0x0610_0000);
        }
        this
    }

    #[test]
    fn load_refuses_other_records_and_reads_the_model_chunks() {
        let mut e = Engine::new();
        e.register(GET_TES_FORM, |_, _| ret(0x19));
        let this = armor(&mut e);
        let file = 0x0500_0000u32;
        assert!(!e.call(0x0051_2fd0, &[this, file]).bool());

        let mut chunks = vec![
            SIG_EDID,
            SIG_OBND,
            SIG_FULL,
            SIG_DSTD,
            SIG_DEST,
            SIG_MODL,
            SIG_MOD2,
            SIG_MOD3,
            SIG_MOD4,
            SIG_BMCT,
            SIG_MODT,
            SIG_MO2T,
            SIG_MO3T,
            SIG_MO4T,
            SIG_MODS,
            SIG_MO2S,
            SIG_MO3S,
            SIG_MO4S,
            SIG_MODD,
            SIG_MOSD,
            SIG_ICON,
            SIG_ICO2,
            SIG_MICO,
            SIG_MIC2,
            0x4d41_4e43,
        ];
        chunks.reverse();
        load_doubles(&mut e, chunks, false);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0051_2fd0, &[this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOAD_FORM), vec![vec![this, file]]);
        assert_eq!(calls_to(&log, SET_FORM_FLAG_8), vec![vec![this, 0]]);
        assert_eq!(calls_to(&log, GET_CHUNK_DATA_SIZED)[0][2], 0x200);
        assert_eq!(calls_to(&log, 0x7000_0004), vec![vec![this, file]]);
        assert_eq!(calls_to(&log, 0x0048_7050), vec![vec![this + 0x30, file]]);
        assert_eq!(
            calls_to(&log, 0x0047_81e0),
            vec![vec![this + 0x14c, file]; 2]
        );
        let slot18: Vec<u32> = calls_to(&log, 0x7000_0018)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(slot18, vec![0x7c, 0xbc, 0x9c, 0xdc, 0x134]);
        let textures: Vec<u32> = calls_to(&log, 0x0048_93e0)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(textures, vec![0x7c, 0xbc, 0x9c, 0xdc]);
        let swaps: Vec<u32> = calls_to(&log, 0x0048_a990)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(swaps, vec![0x7c, 0xbc, 0x9c, 0xdc]);
        let chunks: Vec<u32> = calls_to(&log, 0x0048_92d0)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(chunks, vec![0x7c, 0x9c]);
        let icons: Vec<u32> = calls_to(&log, 0x0048_9100)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(icons, vec![0xfc, 0x108]);
        let message: Vec<u32> = calls_to(&log, 0x0047_a380)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(message, vec![0x114, 0x124]);
        // Whole-chunk reads use size 0.
        let sized = calls_to(&log, GET_CHUNK_DATA_SIZED);
        assert_eq!(sized.len(), 10);
        assert!(sized[1..].iter().all(|w| w[2] == 0));
    }

    #[test]
    fn load_reads_the_word_and_data_chunks() {
        let mut e = Engine::new();
        let this = armor(&mut e);
        let file = 0x0500_0000u32;
        let mut chunks = vec![
            SIG_SCRI, SIG_EITM, SIG_ENAM, SIG_REPL, SIG_BIPL, SIG_ETYP, SIG_YNAM, SIG_ZNAM,
            SIG_ANAM, SIG_EAMT, SIG_TNAM, SIG_DNAM, SIG_BMDT, SIG_DATA, SIG_SNAM,
        ];
        chunks.reverse();
        load_doubles(&mut e, chunks, true);
        e.register(FORM_VERSION, |_, _| ret(0xffff_000a));
        e.register(0x0048_6620, |_, _| ret(6));
        e.mem.set_u8(this + 0x78, 0xff);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0051_2fd0, &[this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SET_FORM),
            vec![
                vec![this + 0x3c, 0x501],
                vec![this + 0x48, 0x502],
                vec![this + 0x48, 0x503],
                vec![this + 0x15c, 0x504],
                vec![this + 0x164, 0x505],
                vec![this + 0x154, 0x506],
                vec![this + 0x16c, 0x507],
            ]
        );
        assert_eq!(calls_to(&log, 0x0048_cd90), vec![vec![this + 0x3c, this]]);
        assert_eq!(calls_to(&log, 0x0040_3550), vec![vec![this + 0x16c, 0x508]]);
        assert_eq!(
            calls_to(&log, 0x0048_3170),
            vec![vec![this + 0x48, 0x77]; 2]
        );
        assert_eq!(e.mem.u32(this + 0x188), 0x509);
        // DNAM: cleared, read, swapped.
        assert_eq!(calls_to(&log, MEMSET)[0], vec![this + 0x178, 0, 0xc]);
        assert!(calls_to(&log, GET_CHUNK_DATA_SIZED).contains(&vec![file, this + 0x178, 0xc]));
        // BMDT before version 0xB: 4 bytes, swapped, split.
        assert_eq!(e.mem.u32(this + 0x74), 0x1234);
        assert_eq!(e.mem.u8(this + 0x78), 0xab & 0xef);
        // DATA 2 bytes longer than 6: read, swapped, copied; then the
        // whole data is swapped again (DNAM's swap is the first).
        let data = calls_to(&log, LOAD_DATA);
        assert_eq!((data[0][0], data[0][1], data[0][3]), (this, file, 2));
        assert_eq!(
            calls_to(&log, 0x0040_1460),
            vec![vec![this + 0x178, data[0][2], 2]]
        );
        assert_eq!(calls_to(&log, 0x0051_2810).len(), 2);
        assert_eq!(calls_to(&log, SWAP_DATA), vec![vec![this + 0x178]; 2]);
        // SNAM without the flag: nothing.
        assert_eq!(data.len(), 1);
        assert!(calls_to(&log, 0x005a_e3d0).is_empty());
    }

    #[test]
    fn load_builds_the_list_and_reads_new_data() {
        let mut e = Engine::new();
        let this = armor(&mut e);
        let file = 0x0500_0000u32;
        let mut chunks = vec![SIG_BNAM, SIG_TNAM, SIG_SNAM, SIG_SNAM, SIG_BMDT, SIG_DATA];
        chunks.reverse();
        load_doubles(&mut e, chunks, false);
        e.register(FORM_VERSION, |_, _| ret(0xb));
        e.register(0x0048_6620, |_, _| ret(7));
        let added = Rc::new(RefCell::new(vec![]));
        let a = added.clone();
        e.register_double(0x005a_e3d0, move |e, w| {
            a.borrow_mut().push((w[0], e.mem.u32(w[1])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0051_2fd0, &[this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u8(this + 0x18c), 1);
        // TNAM with the flag set: not read.
        assert_eq!(calls_to(&log, GET_CHUNK_DATA).len(), 1);
        let list = e.mem.u32(this + 0x188);
        assert_eq!(calls_to(&log, 0x0096_a2d0), vec![vec![list]]);
        let added = added.borrow();
        assert_eq!(added.len(), 2);
        assert_eq!(added[0].0, list);
        assert_eq!(e.mem.u32(added[0].1), 0x99);
        assert_eq!(
            calls_to(&log, ALLOCATE),
            vec![vec![8], vec![0xc], vec![0xc]]
        );
        // BMDT from version 0xB: 8 bytes straight in.
        assert!(calls_to(&log, GET_CHUNK_DATA_SIZED).contains(&vec![file, this + 0x74, 8]));
        // DATA not 2 bytes longer: no buffer.
        assert!(calls_to(&log, LOAD_DATA).contains(&vec![this, file, 0, 0]));
        assert!(calls_to(&log, SWAP_DATA).is_empty());
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

    const SAVES: [u32; 9] = [
        FULL_NAME_SAVE,
        SCRIPTABLE_FORM_SAVE,
        ENCHANTABLE_SAVE,
        BIPED_MODEL_SAVE,
        DESTRUCTIBLE_SAVE,
        REPAIR_LIST_SAVE,
        BIPED_MODEL_LIST_SAVE,
        EQUIP_TYPE_SAVE,
        PICKUP_PUTDOWN_SAVE,
    ];

    fn engine(swaps: u32) -> Engine {
        let mut e = Engine::new();
        for a in SAVES.into_iter().chain([
            START_FORM,
            CLOSE_FORM,
            FORM_SAVE_DATA,
            SWAP_DATA,
            ADD_CHUNK_WORD,
            SOUND_RECORD_SWAP,
        ]) {
            e.register(a, |_, _| Ret::default());
        }
        e.register_double(SAVE_SWAPS_BYTES, move |_, _| swaps.into_ret());
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(SOUND_RECORD_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u8(a[0] + 4, 0);
            e.mem.set_u32(a[0] + 8, 0);
            a[0].into_ret()
        });
        e
    }

    fn form(e: &mut Engine, id: u32) -> u32 {
        let form = e.mem.alloc(0x18);
        e.mem.set_u32(form + 0xc, id);
        form
    }

    #[test]
    fn save_writes_components_dnam_bnam_and_tnam() {
        let mut e = engine(0);
        e.register(ADD_CHUNK_DATA, |_, _| Ret::default());
        let this = e.mem.alloc(0x190);
        let template = form(&mut e, 0x0abc);
        e.mem.set_u32(this + 0x188, template);
        e.call_log = Some(vec![]);
        e.call(0x0051_3d90, &args![this]);
        let log = e.call_log.take().unwrap();
        let offsets = [0x30, 0x3c, 0x48, 0x70, 0x14c, 0x15c, 0x164, 0x154, 0x16c];
        for (save, offset) in SAVES.iter().zip(offsets) {
            assert_eq!(only(&log, *save), vec![vec![this + offset]]);
        }
        assert_eq!(only(&log, FORM_SAVE_DATA), vec![vec![this]]);
        assert_eq!(
            only(&log, ADD_CHUNK_DATA),
            vec![vec![SIG_DNAM, this + 0x178, 0xc]]
        );
        assert_eq!(
            only(&log, ADD_CHUNK_WORD),
            vec![vec![SIG_BNAM, 0], vec![SIG_TNAM, 0x0abc]]
        );
        assert!(only(&log, SWAP_DATA).is_empty());
        assert_eq!(only(&log, CLOSE_FORM), vec![vec![this]]);
    }

    #[test]
    fn save_writes_one_snam_per_sound_entry() {
        let mut e = engine(1);
        let seen = e.mem.alloc(0x40);
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            if a[0] == SIG_SNAM {
                let n = e.mem.u32(seen + 0x3c);
                for i in 0..3 {
                    let word = e.mem.u32(a[1] + 4 * i);
                    e.mem.set_u32(seen + 0xc * n + 4 * i, word);
                }
                e.mem.set_u32(seen + 0x3c, n + 1);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x190);
        e.mem.set_u8(this + 0x18c, 2);
        let entry = |e: &mut Engine, form: u32, byte: u8, word: u32| {
            let p = e.mem.alloc(0xc);
            e.mem.set_u32(p, form);
            e.mem.set_u8(p + 4, byte);
            e.mem.set_u32(p + 8, word);
            p
        };
        let first_form = form(&mut e, 0x111);
        let second_form = form(&mut e, 0x222);
        let a = entry(&mut e, first_form, 7, 0x70);
        let empty = entry(&mut e, 0, 1, 1);
        let b = entry(&mut e, second_form, 8, 0x80);
        let after_end = entry(&mut e, first_form, 9, 0x90);
        let mut nodes = vec![];
        for item in [a, empty, b, 0, after_end] {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, item);
            nodes.push(node);
        }
        for pair in nodes.windows(2) {
            e.mem.set_u32(pair[0] + 4, pair[1]);
        }
        e.mem.set_u32(this + 0x188, nodes[0]);
        e.call_log = Some(vec![]);
        e.call(0x0051_3d90, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(seen + 0x3c), 2);
        let records: Vec<u32> = (0..6).map(|i| e.mem.u32(seen + 4 * i)).collect();
        assert_eq!(records, vec![0x111, 7, 0x70, 0x222, 8, 0x80]);
        assert_eq!(only(&log, ADD_CHUNK_WORD), vec![vec![SIG_BNAM, 1]]);
        assert_eq!(only(&log, SWAP_DATA).len(), 2);
        assert_eq!(only(&log, SOUND_RECORD_SWAP).len(), 4);
        assert_eq!(only(&log, SOUND_RECORD_CONSTRUCT).len(), 2);
    }
}
