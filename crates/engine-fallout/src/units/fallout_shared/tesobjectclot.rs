//! `fallout shared/tesobjectclot.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESFile::GetTESForm` (`00472660`): the record's form type.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (`00485110`, `RET 4`).
const LOAD_FORM: u32 = 0x0048_5110;
/// Sets (`1`) or clears (`0`) form flag 0x8 (`00484ab0`, `RET 4`).
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// Tests form flag 0x8 (`004013e0`).
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// `TESFile::GetTESChunk` (`004726b0`).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The size of the current chunk (`00401660`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (`00472890`): buffer and size.
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (`004727f0`): one word.
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov2` (`00472840`): one 16-bit word.
const GET_CHUNK_DATA_U16: u32 = 0x0047_2840;
/// Moves to the next chunk (`004726f0`, `bool`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// Whether the file's data needs its byte order swapped (`00401680`).
const SWAP_BYTES: u32 = 0x0040_1680;
/// The file's form version (`00403570`, a 16-bit word).
const FORM_VERSION: u32 = 0x0040_3570;
/// `TESForm::LoadData` (`004861f0`, `RET 0xC`): file, buffer, size.
const LOAD_DATA: u32 = 0x0048_61f0;
/// The setter the form-id chunks go to (`006ecd40`, `RET 4`: the word at
/// +4).
const SET_FORM: u32 = 0x006e_cd40;
/// The components' loaders and setters (`thiscall` unless noted):
/// `TESFullName`'s chunk load (`00487050`, cdecl(component, file)),
/// `TESScriptableForm::InitItem` (`0048cd90`, `RET 4`),
/// `TESEnchantableForm`'s amount (`00483170`, `RET 4`), the biped flags
/// swaps (`00512810` on 4 bytes, `00503210` on 8), `TESModel`'s
/// `LoadModelTextureChunk` (`004893e0`, cdecl(model, file)),
/// `TESModelTextureSwap::LoadModelTextureSwapChunk` (`0048a990`, cdecl),
/// `TESModel::LoadModelChunk` (`004892d0`, cdecl),
/// `TESSoundFile::SetSoundFile` (`00489100`, `RET 4`; the icons), the
/// message icons' setter (`0047a380`, `RET 4`) and
/// `BGSDestructibleObjectForm::LoadChunk` (`004781e0`, cdecl(component,
/// file)).
const FULL_NAME_LOAD: u32 = 0x0048_7050;
const SCRIPTABLE_INIT_ITEM: u32 = 0x0048_cd90;
const ENCHANTABLE_SET_AMOUNT: u32 = 0x0048_3170;
const SWAP_FLAGS_SHORT: u32 = 0x0051_2810;
const SWAP_FLAGS_LONG: u32 = 0x0050_3210;
const MODEL_LOAD_TEXTURES: u32 = 0x0048_93e0;
const MODEL_LOAD_TEXTURE_SWAPS: u32 = 0x0048_a990;
const MODEL_LOAD_CHUNK: u32 = 0x0048_92d0;
const SET_ICON: u32 = 0x0048_9100;
const SET_MESSAGE_ICON: u32 = 0x0047_a380;
const DESTRUCTIBLE_LOAD_CHUNK: u32 = 0x0047_81e0;
/// `InitItem`'s callees: `BGSDestructibleObjectForm::InitItem`
/// (`00478be0`), `TESBipedModelForm::InitItem` (`00480750`),
/// `TESEnchantableForm::InitItem` (`00483210`), each `RET 4` with the form.
const ENCHANTABLE_INIT_ITEM: u32 = 0x0048_3210;
const DESTRUCTIBLE_INIT_ITEM: u32 = 0x0047_8be0;
const BIPED_MODEL_INIT_ITEM: u32 = 0x0048_0750;
/// The warning printer (`005b5e40`, cdecl, variable arguments) and
/// "MASTERFILE: Clothing '%s' needs to have biped slots selected in the
/// editor."
const WARNING: u32 = 0x005b_5e40;
const NO_BIPED_SLOTS_MESSAGE: u32 = 0x0102_ad90;
/// The component saves (`thiscall`, no stack words): `TESFullName::Save`
/// (`00487010`), `TESScriptableForm::Save` (`0048cca0`),
/// `TESEnchantableForm::Save` (`0047bd70`), `TESBipedModelForm::Save`
/// (`00480610`), `BGSDestructibleObjectForm::Save` (`00477f20`),
/// `BGSEquipType::Save` (`00479350`); `TESForm::StartForm` (`004855a0`),
/// `TESForm::SaveData` (`00485e00`), `TESForm::CloseForm` (`00485680`).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const SCRIPTABLE_FORM_SAVE: u32 = 0x0048_cca0;
const ENCHANTABLE_SAVE: u32 = 0x0047_bd70;
const BIPED_MODEL_SAVE: u32 = 0x0048_0610;
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
const EQUIP_TYPE_SAVE: u32 = 0x0047_9350;
const START_FORM: u32 = 0x0048_55a0;
const FORM_SAVE_DATA: u32 = 0x0048_5e00;
const CLOSE_FORM: u32 = 0x0048_5680;

/// Chunk signatures, as the words the exe compares.
const SIG_ANAM: u32 = u32::from_le_bytes(*b"ANAM");
const SIG_BMCT: u32 = u32::from_le_bytes(*b"BMCT");
const SIG_BMDT: u32 = u32::from_le_bytes(*b"BMDT");
const SIG_DATA: u32 = u32::from_le_bytes(*b"DATA");
const SIG_DEST: u32 = u32::from_le_bytes(*b"DEST");
const SIG_DSTD: u32 = u32::from_le_bytes(*b"DSTD");
const SIG_EAMT: u32 = u32::from_le_bytes(*b"EAMT");
const SIG_EDID: u32 = u32::from_le_bytes(*b"EDID");
const SIG_EITM: u32 = u32::from_le_bytes(*b"EITM");
const SIG_ENAM: u32 = u32::from_le_bytes(*b"ENAM");
const SIG_ETYP: u32 = u32::from_le_bytes(*b"ETYP");
const SIG_FULL: u32 = u32::from_le_bytes(*b"FULL");
const SIG_ICO2: u32 = u32::from_le_bytes(*b"ICO2");
const SIG_ICON: u32 = u32::from_le_bytes(*b"ICON");
const SIG_MIC2: u32 = u32::from_le_bytes(*b"MIC2");
const SIG_MICO: u32 = u32::from_le_bytes(*b"MICO");
const SIG_MO2S: u32 = u32::from_le_bytes(*b"MO2S");
const SIG_MO2T: u32 = u32::from_le_bytes(*b"MO2T");
const SIG_MO3S: u32 = u32::from_le_bytes(*b"MO3S");
const SIG_MO3T: u32 = u32::from_le_bytes(*b"MO3T");
const SIG_MO4S: u32 = u32::from_le_bytes(*b"MO4S");
const SIG_MO4T: u32 = u32::from_le_bytes(*b"MO4T");
const SIG_MOD2: u32 = u32::from_le_bytes(*b"MOD2");
const SIG_MOD3: u32 = u32::from_le_bytes(*b"MOD3");
const SIG_MOD4: u32 = u32::from_le_bytes(*b"MOD4");
const SIG_MODD: u32 = u32::from_le_bytes(*b"MODD");
const SIG_MODL: u32 = u32::from_le_bytes(*b"MODL");
const SIG_MODS: u32 = u32::from_le_bytes(*b"MODS");
const SIG_MODT: u32 = u32::from_le_bytes(*b"MODT");
const SIG_MOSD: u32 = u32::from_le_bytes(*b"MOSD");
const SIG_OBND: u32 = u32::from_le_bytes(*b"OBND");
const SIG_SCRI: u32 = u32::from_le_bytes(*b"SCRI");

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

// Translated from 005156d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCLOT::InitItem` (engine map; no stack words): unless form flag
/// 0x8 is set (`004013e0`): clothing without biped slots (the word at
/// `+0x6c`) is reported with its name (vtable slot 0x130); then the
/// components' `InitItem` with the clothing (`0048cd90` on `+0x3c`,
/// `00483210` on `+0x48`, `00478be0` on `+0x144`, `00480750` on `+0x68`);
/// finally form flag 0x8 is set.
pub fn tes_object_clot_init_item(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    if e.mem.u32(base + 0x6c) == 0 {
        let name = e.vcall(base, 0x130, &[]).u32();
        e.call(WARNING, &args![NO_BIPED_SLOTS_MESSAGE, name]);
    }
    for (callee, offset) in [
        (SCRIPTABLE_INIT_ITEM, 0x3cu32),
        (ENCHANTABLE_INIT_ITEM, 0x48),
        (DESTRUCTIBLE_INIT_ITEM, 0x144),
        (BIPED_MODEL_INIT_ITEM, 0x68),
    ] {
        e.call(callee, &args![base + offset, this]);
    }
    e.call(SET_FORM_FLAG_8, &args![this, 1u32]);
}

// Translated from 00515760 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00515760 - the /GS cookie check (00ec408c) is not translated; the __alloca_probe_16 (00ec5ec0) buffers are with_stack blocks of the chunk size (their 16-byte alignment padding, and the stack growing with each such chunk, are not modelled)
/// `TESObjectCLOT`'s `Load` (engine map: no name; `RET 4`): false unless
/// the record is of form type 0x1a. Then `TESForm::LoadForm`, form flag 0x8
/// cleared, and every chunk of `file`:
/// - `EDID` (vtable slot 0x134, read with at most 0x200 bytes), `OBND`
///   (slot 0xe0), `FULL` (`00487050` on `+0x30`, null when `this` is),
///   `DSTD`/`DEST` (`004781e0` on `+0x144`, the same);
/// - form ids (each read into a cleared local) given to `006ecd40`: `SCRI`
///   (`+0x3c`, then `0048cd90` with the clothing), `EITM` and `ENAM`
///   (`+0x48`), `ETYP` (`+0x14c`); `ANAM` and `EAMT` are 16-bit words given
///   to `00483170` on `+0x48`;
/// - the four models (`+0x74`, `+0xb4`, `+0x94`, `+0xd4`): `MODL`, `MOD2`,
///   `MOD3`, `MOD4` read whole and given to the model's vtable slot 0x18;
///   `MODT`/`MO2T`/`MO3T`/`MO4T` to `004893e0`; `MODS`/`MO2S`/`MO3S`/`MO4S`
///   to `0048a990`; `MODD` (`+0x74`) and `MOSD` (`+0x94`) to
///   `TESModel::LoadModelChunk`; `BMCT` (`+0x12c`, slot 0x18);
///   `ICON`/`ICO2` to `00489100` on `+0xf4`/`+0x100`; `MICO`/`MIC2` to
///   `0047a380` on `+0x10c`/`+0x11c`;
/// - `BMDT`: before form version 0xb, 4 bytes (swapped by `00512810` when
///   the file says so) whose first 16 bits become the slots word `+0x6c`
///   and whose third byte the flags byte `+0x70`; from version 0xb on, 8
///   bytes straight into `+0x6c` (swapped by `00503210`); then bit 0x10 of
///   `+0x70` is cleared;
/// - `DATA`: `TESForm::LoadData` with no buffer.
///
/// Other chunks are skipped. Returns true.
pub fn fn_00515760(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(GET_TES_FORM, &args![file]).u8() != 0x1a {
        return false;
    }
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, 0u32]);
    e.with_stack(8, |e, locals| {
        // One word for the chunk words (each cleared before its read), and
        // `[EBP-0x1c]` (`BMDT`, not cleared) in the second.
        let word_at = locals;
        let small_at = locals.byte_add(4);
        loop {
            let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
            if chunk == 0 {
                break;
            }
            match chunk {
                SIG_EDID => {
                    let size = e.call(CHUNK_SIZE, &args![file]).u32();
                    e.with_stack(size, |e, buffer| {
                        e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, 0x200u32]);
                        e.vcall(base, 0x134, &args![buffer]);
                    });
                }
                SIG_OBND => {
                    e.vcall(base, 0xe0, &args![file]);
                }
                SIG_FULL => {
                    let component = if base != 0 { base + 0x30 } else { 0 };
                    e.call(FULL_NAME_LOAD, &args![component, file]);
                }
                SIG_SCRI => {
                    let script = read_word(e, file, word_at);
                    e.call(SET_FORM, &args![base + 0x3c, script]);
                    e.call(SCRIPTABLE_INIT_ITEM, &args![base + 0x3c, this]);
                }
                SIG_EITM | SIG_ENAM | SIG_ETYP => {
                    let component = if chunk == SIG_ETYP { 0x14c } else { 0x48 };
                    let form = read_word(e, file, word_at);
                    e.call(SET_FORM, &args![base + component, form]);
                }
                SIG_ANAM | SIG_EAMT => {
                    e.mem.set_u16(word_at.addr(), 0);
                    e.call(GET_CHUNK_DATA_U16, &args![file, word_at]);
                    let amount = e.mem.u16(word_at.addr());
                    e.call(
                        ENCHANTABLE_SET_AMOUNT,
                        &args![base + 0x48, u32::from(amount)],
                    );
                }
                SIG_BMDT => {
                    let version = e.call(FORM_VERSION, &args![file]).u16();
                    if version < 0xb {
                        e.call(GET_CHUNK_DATA_SIZED, &args![file, small_at, 4u32]);
                        if e.call(SWAP_BYTES, &args![file]).bool() {
                            e.call(SWAP_FLAGS_SHORT, &args![small_at]);
                        }
                        let flags = e.mem.u8(small_at.addr() + 2);
                        e.mem.set_u8(base + 0x70, flags);
                        let slots = e.mem.u16(small_at.addr());
                        e.mem.set_u32(base + 0x6c, u32::from(slots));
                    } else {
                        e.call(GET_CHUNK_DATA_SIZED, &args![file, base + 0x6c, 8u32]);
                        if e.call(SWAP_BYTES, &args![file]).bool() {
                            e.call(SWAP_FLAGS_LONG, &args![base + 0x6c]);
                        }
                    }
                    let flags = e.mem.u8(base + 0x70);
                    e.mem.set_u8(base + 0x70, flags & 0xef);
                }
                SIG_MODL | SIG_MOD2 | SIG_MOD3 | SIG_MOD4 | SIG_BMCT => {
                    let model = match chunk {
                        SIG_MODL => 0x74,
                        SIG_MOD2 => 0xb4,
                        SIG_MOD3 => 0x94,
                        SIG_MOD4 => 0xd4,
                        _ => 0x12c,
                    };
                    read_buffer(e, file, |e, buffer| {
                        e.vcall(base + model, 0x18, &args![buffer]);
                    });
                }
                SIG_MODT | SIG_MO2T | SIG_MO3T | SIG_MO4T => {
                    let model = match chunk {
                        SIG_MODT => 0x74,
                        SIG_MO2T => 0xb4,
                        SIG_MO3T => 0x94,
                        _ => 0xd4,
                    };
                    e.call(MODEL_LOAD_TEXTURES, &args![base + model, file]);
                }
                SIG_MODS | SIG_MO2S | SIG_MO3S | SIG_MO4S => {
                    let model = match chunk {
                        SIG_MODS => 0x74,
                        SIG_MO2S => 0xb4,
                        SIG_MO3S => 0x94,
                        _ => 0xd4,
                    };
                    e.call(MODEL_LOAD_TEXTURE_SWAPS, &args![base + model, file]);
                }
                SIG_MODD => {
                    e.call(MODEL_LOAD_CHUNK, &args![base + 0x74, file]);
                }
                SIG_MOSD => {
                    e.call(MODEL_LOAD_CHUNK, &args![base + 0x94, file]);
                }
                SIG_ICON | SIG_ICO2 => {
                    let icon = if chunk == SIG_ICON { 0xf4 } else { 0x100 };
                    read_buffer(e, file, |e, buffer| {
                        e.call(SET_ICON, &args![base + icon, buffer]);
                    });
                }
                SIG_MICO | SIG_MIC2 => {
                    let icon = if chunk == SIG_MICO { 0x10c } else { 0x11c };
                    read_buffer(e, file, |e, buffer| {
                        e.call(SET_MESSAGE_ICON, &args![base + icon, buffer]);
                    });
                }
                SIG_DSTD | SIG_DEST => {
                    let component = if base != 0 { base + 0x144 } else { 0 };
                    e.call(DESTRUCTIBLE_LOAD_CHUNK, &args![component, file]);
                }
                SIG_DATA => {
                    e.call(LOAD_DATA, &args![this, file, 0u32, 0u32]);
                }
                _ => {}
            }
            if !e.call(NEXT_CHUNK, &args![file]).bool() {
                break;
            }
        }
    });
    true
}

// Translated from 00515f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCLOT::Save` (engine map; no stack words): `StartForm`, the
/// full name (+0x30), script (+0x3c), enchantment (+0x48), biped model
/// (+0x68), destructible data (+0x144), equip type (+0x14c),
/// `TESForm::SaveData`, `CloseForm`.
pub fn tes_object_clot_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    for (offset, save) in [
        (0x30u32, FULL_NAME_SAVE),
        (0x3c, SCRIPTABLE_FORM_SAVE),
        (0x48, ENCHANTABLE_SAVE),
        (0x68, BIPED_MODEL_SAVE),
        (0x144, DESTRUCTIBLE_SAVE),
        (0x14c, EQUIP_TYPE_SAVE),
    ] {
        e.call(save, &args![base + offset]);
    }
    e.call(FORM_SAVE_DATA, &args![this]);
    e.call(CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_56d0, tes_object_clot_init_item(Ptr)),
        entry!(0x0051_5760, fn_00515760(Ptr, Ptr) -> bool),
        entry!(0x0051_5f40, tes_object_clot_save(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::fallout_shared::tesmodel::load_test::{
        self, calls_to, run, stub, tag, CHUNK_VALUE, FILE,
    };

    type Loaded = (Engine, u32, bool, load_test::Calls);

    const LOAD_CALLEES: [u32; 15] = [
        LOAD_FORM,
        SET_FORM_FLAG_8,
        FULL_NAME_LOAD,
        SET_FORM,
        SCRIPTABLE_INIT_ITEM,
        ENCHANTABLE_SET_AMOUNT,
        SWAP_FLAGS_SHORT,
        SWAP_FLAGS_LONG,
        MODEL_LOAD_TEXTURES,
        MODEL_LOAD_TEXTURE_SWAPS,
        MODEL_LOAD_CHUNK,
        SET_ICON,
        SET_MESSAGE_ICON,
        DESTRUCTIBLE_LOAD_CHUNK,
        LOAD_DATA,
    ];

    /// A clothing object of 0x160 bytes whose vtable slots `+0xe0`, `+0x134`
    /// and whose models' and template's slot `+0x18` are doubles.
    fn clothing(e: &mut Engine) -> u32 {
        let this = load_test::object(
            e,
            0x160,
            0x0900_0000,
            &[(0xe0, 0x0090_0500), (0x134, 0x0090_0600)],
        );
        e.map(0x0900_1000, 0x100);
        e.mem.set_u32(0x0900_1018, 0x0090_0700);
        e.register(0x0090_0700, |_, _| Ret::default());
        for model in [0x74, 0x94, 0xb4, 0xd4, 0x12c] {
            e.mem.set_u32(this + model, 0x0900_1000);
        }
        this
    }

    fn load_with(chunks: &[u32], form_type: u32, swap: bool, version: u32) -> Loaded {
        let mut e = load_test::engine(form_type, chunks, swap);
        e.register_double(FORM_VERSION, move |_, _| version.into_ret());
        stub(&mut e, &LOAD_CALLEES);
        e.register(GET_CHUNK_DATA_U16, |e, a| {
            e.mem.set_u16(a[1], 0x0777);
            Ret::default()
        });
        let this = clothing(&mut e);
        let (ret, calls) = run(&mut e, 0x0051_5760, &[this, FILE]);
        (e, this, ret.bool(), calls)
    }

    fn load(chunks: &[u32]) -> Loaded {
        load_with(chunks, 0x1a, false, 0x1234)
    }

    #[test]
    fn load_needs_the_clothing_form_type() {
        let (_, _, answer, calls) = load_with(&[tag(b"DATA")], 0x18, false, 0x1234);
        assert!(!answer);
        assert!(calls.is_empty());
        let (_, this, answer, calls) = load(&[]);
        assert!(answer);
        assert_eq!(
            calls,
            [
                (LOAD_FORM, vec![this, FILE]),
                (SET_FORM_FLAG_8, vec![this, 0])
            ]
        );
    }

    #[test]
    fn load_reads_the_name_and_form_chunks() {
        let (_, this, _, calls) = load(&[tag(b"EDID"), tag(b"OBND"), tag(b"FULL")]);
        let read = calls_to(&calls, GET_CHUNK_DATA_SIZED);
        assert_eq!((read[0][0], read[0][2]), (FILE, 0x200));
        assert_eq!(calls_to(&calls, 0x0090_0600), [vec![this, read[0][1]]]);
        assert_eq!(calls_to(&calls, 0x0090_0500), [vec![this, FILE]]);
        assert_eq!(calls_to(&calls, FULL_NAME_LOAD), [vec![this + 0x30, FILE]]);

        let (_, this, _, calls) = load(&[
            tag(b"SCRI"),
            tag(b"EITM"),
            tag(b"ENAM"),
            tag(b"ETYP"),
            tag(b"ANAM"),
            tag(b"EAMT"),
        ]);
        assert_eq!(
            calls_to(&calls, SET_FORM),
            [
                vec![this + 0x3c, CHUNK_VALUE],
                vec![this + 0x48, CHUNK_VALUE],
                vec![this + 0x48, CHUNK_VALUE],
                vec![this + 0x14c, CHUNK_VALUE]
            ]
        );
        assert_eq!(
            calls_to(&calls, SCRIPTABLE_INIT_ITEM),
            [vec![this + 0x3c, this]]
        );
        assert_eq!(
            calls_to(&calls, ENCHANTABLE_SET_AMOUNT),
            [vec![this + 0x48, 0x777], vec![this + 0x48, 0x777]]
        );
    }

    #[test]
    fn load_sends_model_chunks_to_their_models() {
        let (_, this, _, calls) = load(&[
            tag(b"MODL"),
            tag(b"MOD2"),
            tag(b"MOD3"),
            tag(b"MOD4"),
            tag(b"BMCT"),
        ]);
        let models: Vec<u32> = calls_to(&calls, 0x0090_0700)
            .iter()
            .map(|c| c[0] - this)
            .collect();
        assert_eq!(models, [0x74, 0xb4, 0x94, 0xd4, 0x12c]);
        let (_, this, _, calls) = load(&[
            tag(b"MODT"),
            tag(b"MO2T"),
            tag(b"MO3T"),
            tag(b"MO4T"),
            tag(b"MODS"),
            tag(b"MO2S"),
            tag(b"MO3S"),
            tag(b"MO4S"),
            tag(b"MODD"),
            tag(b"MOSD"),
        ]);
        let offsets = |callee: u32| -> Vec<u32> {
            calls_to(&calls, callee)
                .iter()
                .map(|c| c[0] - this)
                .collect()
        };
        assert_eq!(offsets(MODEL_LOAD_TEXTURES), [0x74, 0xb4, 0x94, 0xd4]);
        assert_eq!(offsets(MODEL_LOAD_TEXTURE_SWAPS), [0x74, 0xb4, 0x94, 0xd4]);
        assert_eq!(offsets(MODEL_LOAD_CHUNK), [0x74, 0x94]);
    }

    #[test]
    fn load_sets_icons_destruction_and_data() {
        let (_, this, _, calls) = load(&[
            tag(b"ICON"),
            tag(b"ICO2"),
            tag(b"MICO"),
            tag(b"MIC2"),
            tag(b"DSTD"),
            tag(b"DEST"),
            tag(b"DATA"),
            tag(b"CNAM"),
        ]);
        let icons: Vec<u32> = calls_to(&calls, SET_ICON)
            .iter()
            .map(|c| c[0] - this)
            .collect();
        assert_eq!(icons, [0xf4, 0x100]);
        let icons: Vec<u32> = calls_to(&calls, SET_MESSAGE_ICON)
            .iter()
            .map(|c| c[0] - this)
            .collect();
        assert_eq!(icons, [0x10c, 0x11c]);
        assert_eq!(
            calls_to(&calls, DESTRUCTIBLE_LOAD_CHUNK),
            [vec![this + 0x144, FILE], vec![this + 0x144, FILE]]
        );
        assert_eq!(calls_to(&calls, LOAD_DATA), [vec![this, FILE, 0, 0]]);
        let read = calls_to(&calls, GET_CHUNK_DATA_SIZED);
        assert!(read.iter().all(|r| r[2] == 0));
    }

    #[test]
    fn load_reads_biped_data_by_version() {
        let (e, this, _, calls) = load_with(&[tag(b"BMDT")], 0x1a, true, 0x000a);
        assert_eq!(e.mem.u32(this + 0x6c), 0x5a5a);
        assert_eq!(e.mem.u8(this + 0x70), 0x5a & 0xef);
        let read = calls_to(&calls, GET_CHUNK_DATA_SIZED);
        assert_eq!(read[0][2], 4);
        assert_eq!(calls_to(&calls, SWAP_FLAGS_SHORT), [vec![read[0][1]]]);
        let (e, this, _, calls) = load_with(&[tag(b"BMDT")], 0x1a, true, 0x000b);
        assert_eq!(e.mem.u32(this + 0x6c), 0x5a5a_5a5a);
        assert_eq!(e.mem.u8(this + 0x70), 0x5a & 0xef);
        assert_eq!(
            calls_to(&calls, GET_CHUNK_DATA_SIZED),
            [vec![FILE, this + 0x6c, 8]]
        );
        assert_eq!(calls_to(&calls, SWAP_FLAGS_LONG), [vec![this + 0x6c]]);
        let (_, _, _, calls) = load_with(&[tag(b"BMDT")], 0x1a, false, 0x000a);
        assert!(calls_to(&calls, SWAP_FLAGS_SHORT).is_empty());
    }

    #[test]
    fn init_item_reports_missing_slots_and_inits_components() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                SCRIPTABLE_INIT_ITEM,
                ENCHANTABLE_INIT_ITEM,
                DESTRUCTIBLE_INIT_ITEM,
                BIPED_MODEL_INIT_ITEM,
                SET_FORM_FLAG_8,
                WARNING,
            ],
        );
        e.register(FORM_FLAG_8_SET, |e, a| {
            u32::from(e.mem.u8(a[0] + 8) & 8 != 0).into_ret()
        });
        let this = load_test::object(&mut e, 0x160, 0x0900_2000, &[(0x130, 0x0090_0800)]);
        e.register(0x0090_0800, |_, _| 0x6666u32.into_ret());
        let (_, calls) = run(&mut e, 0x0051_56d0, &[this]);
        assert_eq!(
            calls_to(&calls, WARNING),
            [vec![NO_BIPED_SLOTS_MESSAGE, 0x6666]]
        );
        for (callee, offset) in [
            (SCRIPTABLE_INIT_ITEM, 0x3c),
            (ENCHANTABLE_INIT_ITEM, 0x48),
            (DESTRUCTIBLE_INIT_ITEM, 0x144),
            (BIPED_MODEL_INIT_ITEM, 0x68),
        ] {
            assert_eq!(calls_to(&calls, callee), [vec![this + offset, this]]);
        }
        assert_eq!(calls_to(&calls, SET_FORM_FLAG_8), [vec![this, 1]]);
        e.mem.set_u32(this + 0x6c, 1);
        let (_, calls) = run(&mut e, 0x0051_56d0, &[this]);
        assert!(calls_to(&calls, WARNING).is_empty());
        e.mem.set_u8(this + 8, 8);
        let (_, calls) = run(&mut e, 0x0051_56d0, &[this]);
        assert_eq!(calls.len(), 1);
    }

    #[test]
    fn save_writes_the_components() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                START_FORM,
                FULL_NAME_SAVE,
                SCRIPTABLE_FORM_SAVE,
                ENCHANTABLE_SAVE,
                BIPED_MODEL_SAVE,
                DESTRUCTIBLE_SAVE,
                EQUIP_TYPE_SAVE,
                FORM_SAVE_DATA,
                CLOSE_FORM,
            ],
        );
        let (_, calls) = run(&mut e, 0x0051_5f40, &[0x4000]);
        assert_eq!(
            calls,
            [
                (START_FORM, vec![0x4000]),
                (FULL_NAME_SAVE, vec![0x4030]),
                (SCRIPTABLE_FORM_SAVE, vec![0x403c]),
                (ENCHANTABLE_SAVE, vec![0x4048]),
                (BIPED_MODEL_SAVE, vec![0x4068]),
                (DESTRUCTIBLE_SAVE, vec![0x4144]),
                (EQUIP_TYPE_SAVE, vec![0x414c]),
                (FORM_SAVE_DATA, vec![0x4000]),
                (CLOSE_FORM, vec![0x4000])
            ]
        );
    }
}
