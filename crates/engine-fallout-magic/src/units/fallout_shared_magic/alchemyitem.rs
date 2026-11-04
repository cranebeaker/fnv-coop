//! `fallout shared/magic/alchemyitem.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `AlchemyItem` is `MagicItemObject` (form base at +0x00, the `MagicItem`
//! name and effect list at +0x30 and +0x3c) plus the components listed in
//! `00402820`. The magic-item virtual functions of this unit receive the
//! `MagicItem` subobject, 0x30 bytes into the object, as `this`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesfile::{Form, TESFile};

layout! {
    /// `AlchemyItem` (Xbox PDB) on PC, 0xd8 bytes (the size `CreateFormOfType`
    /// allocates for form type 0x2f, see `00402820`). Only the members of its
    /// `AlchemyItemData` this unit reads are declared; the PC offsets are the
    /// PDB's minus 0x10 (the `TESForm` difference).
    pub struct AlchemyItem: 0xd8 {
        /// `data.iCostOverride` (Xbox PDB).
        0xb8 iCostOverride: i32,
        /// `data.pAddictionItem` (Xbox PDB): a `SpellItem *`.
        0xc0 pAddictionItem: Ptr,
        /// `data.pConsumptionSound` (Xbox PDB): a `TESSound *`.
        0xc8 pConsumptionSound: Ptr,
    }
}

/// The chunk tags `LoadMagicItemChunk` switches on, as the little-endian
/// words of their four characters.
const TAG_MODEL_MODL: u32 = 0x4c44_4f4d;
const TAG_MODEL_MODT: u32 = 0x5444_4f4d;
const TAG_MODEL_MODS: u32 = 0x5344_4f4d;
const TAG_MODEL_MODD: u32 = 0x4444_4f4d;
const TAG_MODEL_MOSD: u32 = 0x4453_4f4d;
const TAG_ICON: u32 = 0x4e4f_4349;
const TAG_MESSAGE_ICON: u32 = 0x4f43_494d;
const TAG_SCRIPT: u32 = 0x4952_4353;
const TAG_DESTRUCTIBLE_END: u32 = 0x5453_4544;
const TAG_DESTRUCTIBLE_STAGE: u32 = 0x4454_5344;
const TAG_EQUIP_TYPE: u32 = 0x5059_5445;
const TAG_PICKUP_SOUND: u32 = 0x4d41_4e59;
const TAG_PUTDOWN_SOUND: u32 = 0x4d41_4e5a;
const TAG_FORM_DATA: u32 = 0x4154_4144;

/// `TESBoundObject::TESBoundObject` (Xbox PDB name of `0050e820`).
const BOUND_OBJECT_CONSTRUCTOR: u32 = 0x0050_e820;
/// `00402c90`: runs the constructors of the `MagicItem` parts (a name
/// component at +0 and a list at +0xc, two vtables). The engine map calls it
/// `EffectItemList::EffectItemList`, but it is called on the +0x30
/// subobject and builds both parts.
const MAGIC_ITEM_CONSTRUCTOR: u32 = 0x0040_2c90;
/// `TESModelTextureSwap::Save` (Xbox PDB name of `0048a520`, `RET 0xc`):
/// three chunk tags.
const MODEL_TEXTURE_SWAP_SAVE: u32 = 0x0048_a520;
/// `TESTexture::Save` (Xbox PDB name of `0048e370`, `RET 4`): a chunk tag.
const TEXTURE_SAVE: u32 = 0x0048_e370;
/// `BGSMessageIcon::Save` (Xbox PDB name of `0047a240`, `RET 4`): a chunk tag.
const MESSAGE_ICON_SAVE: u32 = 0x0047_a240;
/// `TESScriptableForm::Save` (Xbox PDB name of `0048cca0`).
const SCRIPTABLE_FORM_SAVE: u32 = 0x0048_cca0;
/// `BGSDestructibleObjectForm::Save` (Xbox PDB name of `00477f20`).
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
/// `BGSPickupPutdownSounds::Save` (Xbox PDB name of `0047b560`).
const PICKUP_PUTDOWN_SAVE: u32 = 0x0047_b560;
/// `BGSEquipType::Save` (Xbox PDB name of `00479350`).
const EQUIP_TYPE_SAVE: u32 = 0x0047_9350;
/// `TESForm::SaveData` (Xbox PDB name of `00485e00`).
const FORM_SAVE_DATA: u32 = 0x0048_5e00;

/// `0084e3a0`: the word at `+0xc` of its object (`TESForm::iFormID`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `00874460` (`RET 4`): stores its argument at `+0xc0` (the setter that
/// goes with `004030b0`).
const SET_ADDICTION_ITEM: u32 = 0x0087_4460;
/// `009acce0`: the word at `+0xc8` (the getter that goes with `004030d0`).
const GET_CONSUMPTION_SOUND: u32 = 0x009a_cce0;
/// `00401500`: the byte at `011c54ba`, whether chunk data is written in the
/// other byte order (`bEndianSwapOnSave`, Xbox PDB static of `TESForm`).
const ENDIAN_SWAP_ON_SAVE: u32 = 0x0040_1500;
/// `00403720` (`run_00403720`): byte-swaps the four 32-bit members of the
/// 0x14-byte data block it is given.
const SWAP_DATA_ENDIAN: u32 = 0x0040_3720;
/// `TESForm::__AddChunkData` (Xbox PDB name of `00485990`, cdecl): the chunk
/// tag, the data and its size.
const ADD_CHUNK_DATA: u32 = 0x0048_5990;

/// `00401660`: the word at `+0x25c` of the `TESFile` (`m_actualChunkSize`).
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB name of `00472890`, `RET 8`): the
/// buffer and its size.
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (Xbox PDB name of `004727f0`, `RET 4`): the
/// address of a word that receives the chunk's word.
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// `TESModelTextureSwap::UnpackTextureSwapChunkData` (Xbox PDB name of
/// `0048a7b0`, `RET 0xc`): buffer, size and form version.
const UNPACK_TEXTURE_SWAP: u32 = 0x0048_a7b0;
/// `TESModel::LoadModelChunk` (Xbox PDB name of `004892d0`, cdecl):
/// (model, file).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESTexture::LoadTextureChunk` (Xbox PDB name of `0048e3d0`, cdecl):
/// (texture, file).
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
/// `0047a290` (`bgsmessageicon.cpp`, cdecl): loads the message icon chunk
/// into the component (component, file).
const LOAD_MESSAGE_ICON_CHUNK: u32 = 0x0047_a290;
/// `BGSDestructibleObjectForm::LoadChunk` (Xbox PDB name of `004781e0`,
/// cdecl): (component, file).
const LOAD_DESTRUCTIBLE_CHUNK: u32 = 0x0047_81e0;
/// `006ecd40` (`RET 4`; the engine map's name for it is wrong): stores a
/// form id in the component it is called on.
const STORE_FORM_ID: u32 = 0x006e_cd40;
/// `TESScriptableForm::InitItem` (Xbox PDB name of `0048cd90`, `RET 4`).
const SCRIPTABLE_FORM_INIT_ITEM: u32 = 0x0048_cd90;
/// `00403550` (`RET 4`): stores its argument at `+8`.
const SET_WORD_AT_8: u32 = 0x0040_3550;
/// `TESForm::LoadData` (Xbox PDB name of `004861f0`, `RET 0xc`).
const FORM_LOAD_DATA: u32 = 0x0048_61f0;

/// `004013e0`: `iFormFlags & 8` of the form (`SETNZ`).
const FORM_FLAG_8: u32 = 0x0040_13e0;
/// `BGSDestructibleObjectForm::InitItem` (Xbox PDB name of `00478be0`,
/// `RET 4`).
const DESTRUCTIBLE_INIT_ITEM: u32 = 0x0047_8be0;
/// `TESModelTextureSwap::InitItem` (Xbox PDB name of `0048aa80`, `RET 4`).
const MODEL_TEXTURE_SWAP_INIT_ITEM: u32 = 0x0048_aa80;
/// `MagicItemObject::InitItem` (Xbox PDB name of `0040c9d0`).
const MAGIC_ITEM_OBJECT_INIT_ITEM: u32 = 0x0040_c9d0;
/// `BGSPickupPutdownSounds::InitItem` (Xbox PDB name of `0047b6a0`,
/// `RET 4`).
const PICKUP_PUTDOWN_INIT_ITEM: u32 = 0x0047_b6a0;
/// `TESForm::GetFile` (Xbox PDB name of `00484e60`, `RET 4`).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB name of `00485d50`, cdecl): the
/// address of the form id and the file; rewrites the id with the file's
/// load order.
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (cdecl): looks a form up by its id, 0 when missing.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `005b5e40` (cdecl, `printf`-style): writes a line to the log.
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// `TESForm::GetFormByEditorID` (Xbox PDB name of `00483a00`, cdecl).
const GET_FORM_BY_EDITOR_ID: u32 = 0x0048_3a00;
/// `00500940`: the address `+0x18` of its object (the form list's items).
const FORM_LIST_ITEMS: u32 = 0x0050_0940;
/// `005f65d0` (`RET 4`): whether the list contains the form id the
/// argument points to.
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// `004029d0` (`RET 4`): whether `iFlags` (the byte at `+0xbc`) has any bit
/// of the argument set.
const HAS_FLAG: u32 = 0x0040_29d0;
/// `EffectItemList::GetCost` (Xbox PDB name of `004063e0`, `RET 4`);
/// returns the cost in `ST0`.
const EFFECT_LIST_GET_COST: u32 = 0x0040_63e0;
/// `__memcmp` (`00ec4835`, cdecl).
const MEMCMP: u32 = 0x00ec_4835;
/// `memcpy` (`00401460`, cdecl).
const MEMCPY: u32 = 0x0040_1460;

/// The type descriptors the casts use (their decorated names, read from the
/// exe: `.?AVTESForm@@`, `.?AVSpellItem@@`, `.?AVTESSound@@`,
/// `.?AVMagicItem@@`, `.?AVAlchemyItem@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_SPELL_ITEM: u32 = 0x0118_3060;
const TYPE_TES_SOUND: u32 = 0x0118_32fc;
const TYPE_MAGIC_ITEM: u32 = 0x0118_3140;
const TYPE_ALCHEMY_ITEM: u32 = 0x0118_30cc;

/// The log lines `InitItem` writes when a reference does not resolve.
const MISSING_ADDICTION_ITEM: u32 = 0x0101_1dd0;
const MISSING_CONSUMPTION_SOUND: u32 = 0x0101_1d80;
/// `"WaterLIST"`, the editor id `IsWater` looks up.
const WATER_LIST_EDITOR_ID: u32 = 0x0101_1e20;

// Translated from 00402b40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00402b40 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `MagicItemObject::MagicItemObject` (Xbox PDB): runs the form base
/// constructor (`0050e820`) and the `MagicItem` parts constructor
/// (`00402c90` on the +0x30 subobject), then stores the three vtables
/// (object, +0x30 and +0x3c). Returns `this`. The exception-unwinding frame
/// is not translated.
pub fn magic_item_object_magic_item_object(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BOUND_OBJECT_CONSTRUCTOR, &args![this]);
    e.call(MAGIC_ITEM_CONSTRUCTOR, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0101_1b4c);
    e.mem.set_u32(base + 0x30, 0x0101_1b04);
    e.mem.set_u32(base + 0x3c, 0x0101_1aec);
    this
}

// Translated from 00402f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AlchemyItem::SaveMagicItemComponents` (Xbox PDB): slot 0x38 of the
/// `MagicItem` vtable. `this` is the `MagicItem` subobject (object + 0x30).
/// Saves the model with its texture-swap tags (`MODL`, `MODT`, `MODS`), the
/// icon (`ICON`), the message icon (`MICO`), the script, the destructible
/// data, the pickup and putdown sounds, the equip type, and finally the
/// form's own data (`TESForm::SaveData` on the object).
pub fn alchemy_item_save_magic_item_components(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(
        MODEL_TEXTURE_SWAP_SAVE,
        &args![base + 0x1c, TAG_MODEL_MODL, TAG_MODEL_MODT, TAG_MODEL_MODS],
    );
    e.call(TEXTURE_SAVE, &args![base + 0x3c, TAG_ICON]);
    e.call(MESSAGE_ICON_SAVE, &args![base + 0x48, TAG_MESSAGE_ICON]);
    e.call(SCRIPTABLE_FORM_SAVE, &args![base + 0x58]);
    e.call(DESTRUCTIBLE_SAVE, &args![base + 0x74]);
    e.call(PICKUP_PUTDOWN_SAVE, &args![base + 0x7c]);
    e.call(EQUIP_TYPE_SAVE, &args![base + 0x6c]);
    e.call(FORM_SAVE_DATA, &args![base.wrapping_sub(0x30)]);
}

// Translated from 00402f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot 0x3c of the `MagicItem` vtable (`SaveMagicItemData` in the Xbox PDB;
/// the engine map has no name for the override). `this` is the `MagicItem`
/// subobject (object + 0x30). Writes the 0x14-byte data block (+0xb8 of the
/// object) as a chunk whose tag is the object's virtual `GetChunkID` (slot
/// 0x24). Before writing, the addiction item and the consumption sound
/// pointers inside the block are replaced by their form ids (0 for none),
/// and when the save is byte-swapped (`00401500`) the block's words are
/// swapped (`00403720`) for the write and swapped back after; the pointers
/// are restored at the end.
pub fn fn_00402f90(e: &mut Engine, this: Ptr) {
    let object = Ptr::<AlchemyItem>::new(this.addr().wrapping_sub(0x30));
    let data = this.addr() + 0x88;
    let addiction_item = fn_004030b0(e, object);
    let addiction_id = if fn_004030b0(e, object).is_null() {
        0
    } else {
        let item = fn_004030b0(e, object);
        e.call(FORM_ID, &args![item]).u32()
    };
    e.call(SET_ADDICTION_ITEM, &args![object, addiction_id]);
    let consumption_sound = e.call(GET_CONSUMPTION_SOUND, &args![object]).u32();
    let sound_id = if e.call(GET_CONSUMPTION_SOUND, &args![object]).u32() == 0 {
        0
    } else {
        let sound = e.call(GET_CONSUMPTION_SOUND, &args![object]).u32();
        e.call(FORM_ID, &args![sound]).u32()
    };
    fn_004030d0(e, object, Ptr::new(sound_id));
    if e.call(ENDIAN_SWAP_ON_SAVE, &[]).bool() {
        e.call(SWAP_DATA_ENDIAN, &args![data]);
    }
    let chunk_id = e.vcall(this.addr(), 0x24, &[]).u32();
    e.call(ADD_CHUNK_DATA, &args![chunk_id, data, 0x14u32]);
    if e.call(ENDIAN_SWAP_ON_SAVE, &[]).bool() {
        e.call(SWAP_DATA_ENDIAN, &args![data]);
    }
    e.call(SET_ADDICTION_ITEM, &args![object, addiction_item]);
    fn_004030d0(e, object, Ptr::new(consumption_sound));
}

// Translated from 004030b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `+0xc0` (`AlchemyItem::pAddictionItem`, Xbox PDB).
/// This body is shared by identical-code folding: callers in other classes
/// reach the same address for their own word at `+0xc0`.
pub fn fn_004030b0(e: &mut Engine, this: Ptr<AlchemyItem>) -> Ptr {
    e.get(this, AlchemyItem::pAddictionItem)
}

// Translated from 004030d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at `+0xc8` (`AlchemyItem::pConsumptionSound`, Xbox PDB).
/// Shared by identical-code folding like `004030b0`. `RET 4`.
pub fn fn_004030d0(e: &mut Engine, this: Ptr<AlchemyItem>, value: Ptr) {
    e.set(this, AlchemyItem::pConsumptionSound, value);
}

/// Looks `id` up as a form, casts it to `target` and returns the id as
/// `AddCompileIndex` left it with the cast form (0 when the id is not found):
/// the sequence `InitItem` runs for both of its references.
fn resolve_reference(e: &mut Engine, this: Ptr<AlchemyItem>, id: Ptr, target: u32) -> (u32, Ptr) {
    let file = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), id.addr());
        e.call(ADD_COMPILE_INDEX, &args![local, file]);
        let compiled = e.mem.u32(local.addr());
        let form = e.call(LOOKUP_FORM, &args![compiled]).u32();
        let cast = e.call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
        );
        (compiled, cast.ptr())
    })
}

/// Writes the log line `InitItem` gives for a reference that did not
/// resolve: the format, the id, the object's type name (virtual slot 0x130)
/// and the object's form id.
fn log_missing_reference(e: &mut Engine, this: Ptr<AlchemyItem>, format: u32, id: u32) {
    let form_id = e.call(FORM_ID, &args![this]).u32();
    let name = e.vcall(this.addr(), 0x130, &[]).u32();
    e.call(LOG_MESSAGE, &args![format, id, name, form_id]);
}

// Translated from 004030f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AlchemyItem::InitItem` (Xbox PDB): does nothing when the form's flag 8
/// is set. Otherwise initializes the components (script, destructible data,
/// model texture swap, the magic item, pickup and putdown sounds), then
/// resolves the two form ids stored in the data block: the addiction item
/// (a `SpellItem`) and the consumption sound (a `TESSound`). Each id gets
/// the file's load-order index, is looked up and cast; the result is stored
/// back (`00874460`, `004030d0`). A reference that is set but does not
/// resolve writes a "MASTERFILE: Failed to find ..." line to the log.
pub fn alchemy_item_init_item(e: &mut Engine, this: Ptr<AlchemyItem>) {
    if e.call(FORM_FLAG_8, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    e.call(SCRIPTABLE_FORM_INIT_ITEM, &args![base + 0x88, this]);
    e.call(DESTRUCTIBLE_INIT_ITEM, &args![base + 0xa4, this]);
    e.call(MODEL_TEXTURE_SWAP_INIT_ITEM, &args![base + 0x4c, this]);
    e.call(MAGIC_ITEM_OBJECT_INIT_ITEM, &args![this]);
    e.call(PICKUP_PUTDOWN_INIT_ITEM, &args![base + 0xac, this]);
    let addiction = fn_004030b0(e, this);
    if !addiction.is_null() {
        let (id, cast) = resolve_reference(e, this, addiction, TYPE_SPELL_ITEM);
        e.call(SET_ADDICTION_ITEM, &args![this, cast]);
        if fn_004030b0(e, this).is_null() {
            log_missing_reference(e, this, MISSING_ADDICTION_ITEM, id);
        }
    }
    let sound = Ptr::new(e.call(GET_CONSUMPTION_SOUND, &args![this]).u32());
    if !sound.is_null() {
        let (id, cast) = resolve_reference(e, this, sound, TYPE_TES_SOUND);
        fn_004030d0(e, this, cast);
        if e.call(GET_CONSUMPTION_SOUND, &args![this]).u32() == 0 {
            log_missing_reference(e, this, MISSING_CONSUMPTION_SOUND, id);
        }
    }
}

/// Reads the chunk's one word with `TESFile::GetChunkData` into a local the
/// game keeps on its stack, which starts as 0.
fn read_chunk_word(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), 0);
        e.call(GET_CHUNK_DATA, &args![file, local]);
        e.mem.u32(local.addr())
    })
}

// Translated from 00403280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00403280 - the /GS stack-cookie check (00ec408c) is not translated.
/// Slot 0x40 of the `MagicItem` vtable (`LoadMagicItemChunk` in the Xbox
/// PDB; the engine map has no name for the override). `this` is the
/// `MagicItem` subobject (object + 0x30), `file` the `TESFile` and `chunk`
/// the chunk tag (`RET 8`). Hands the chunk to the component it belongs to:
/// the model (`MODL`, `MODT`, `MODD`, `MOSD`; the component address
/// is 0 when the object address is 0), the texture-swap list (`MODS`: the
/// chunk is read whole into a buffer on the stack and unpacked with the
/// file's form version), `ICON`, `MICO`, the script (`SCRI`), the
/// destructible data (`DEST`, `DSTD`), the equip type (`ETYP`), the pickup
/// sound (`YNAM`), the putdown sound (`ZNAM`) and the form's own `DATA`.
/// Other tags are ignored. The stack-cookie check is not translated.
pub fn fn_00403280(e: &mut Engine, this: Ptr, file: Ptr<TESFile>, chunk: u32) {
    let base = this.addr();
    let object = base.wrapping_sub(0x30);
    let component = |offset: u32| {
        if object == 0 {
            0
        } else {
            base.wrapping_add(offset)
        }
    };
    match chunk {
        TAG_MODEL_MODD | TAG_MODEL_MOSD | TAG_MODEL_MODL | TAG_MODEL_MODT => {
            e.call(LOAD_MODEL_CHUNK, &args![component(0x1c), file]);
        }
        TAG_MODEL_MODS => {
            let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
            e.with_stack(size.wrapping_add(15) & !15, |e, buffer| {
                e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, size]);
                let version = u32::from(fn_00403570(e, file));
                e.call(
                    UNPACK_TEXTURE_SWAP,
                    &args![base + 0x1c, buffer, size, version],
                );
            });
        }
        TAG_ICON => {
            e.call(LOAD_TEXTURE_CHUNK, &args![component(0x3c), file]);
        }
        TAG_MESSAGE_ICON => {
            e.call(LOAD_MESSAGE_ICON_CHUNK, &args![component(0x48), file]);
        }
        TAG_SCRIPT => {
            let form_id = read_chunk_word(e, file.cast());
            e.call(STORE_FORM_ID, &args![base + 0x58, form_id]);
            e.call(SCRIPTABLE_FORM_INIT_ITEM, &args![base + 0x58, object]);
        }
        TAG_DESTRUCTIBLE_END | TAG_DESTRUCTIBLE_STAGE => {
            e.call(LOAD_DESTRUCTIBLE_CHUNK, &args![component(0x74), file]);
        }
        TAG_EQUIP_TYPE => {
            let value = read_chunk_word(e, file.cast());
            e.call(STORE_FORM_ID, &args![base + 0x6c, value]);
        }
        TAG_PICKUP_SOUND => {
            let value = read_chunk_word(e, file.cast());
            e.call(STORE_FORM_ID, &args![base + 0x7c, value]);
        }
        TAG_PUTDOWN_SOUND => {
            let value = read_chunk_word(e, file.cast());
            e.call(SET_WORD_AT_8, &args![base + 0x7c, value]);
        }
        TAG_FORM_DATA => {
            e.call(FORM_LOAD_DATA, &args![object, file, 0u32, 0u32]);
        }
        _ => {}
    }
}

// Translated from 00403570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the form version of the record the file is reading: the 16-bit
/// word at `+0x254`, `m_currentform.sFormVersion` (Xbox PDB layout of
/// `TESFile`). The engine map has no name for it.
pub fn fn_00403570(e: &mut Engine, this: Ptr<TESFile>) -> u16 {
    e.get(this.at(TESFile::m_currentform), Form::sFormVersion)
}

// Translated from 00403590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot 0x30 of the `MagicItem` vtable (`CompareMagicItemData` in the Xbox
/// PDB; no map name for the override). `this` is the `MagicItem` subobject.
/// Returns 1 when `other` is not an `AlchemyItem` (the cast from `MagicItem`
/// fails) or when its 0x14-byte data block (virtual `GetData`, slot 0x28)
/// differs from this object's block at `this + 0x88`; 0 when they are
/// equal. `RET 4`.
pub fn fn_00403590(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let cast = e
        .call(
            RT_DYNAMIC_CAST,
            &args![other, 0u32, TYPE_MAGIC_ITEM, TYPE_ALCHEMY_ITEM, 0u32],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    let other_data = e.vcall(cast + 0x30, 0x28, &[]).u32();
    e.call(MEMCMP, &args![this.addr() + 0x88, other_data, 0x14u32])
        .i32()
        != 0
}

// Translated from 00403600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot 0x34 of the `MagicItem` vtable (`CopyMagicItemData` in the Xbox PDB;
/// no map name for the override). `this` is the `MagicItem` subobject. When
/// `other` casts to an `AlchemyItem`, copies its 0x14-byte data block
/// (virtual `GetData`, slot 0x28, asked of `other` first and then of this
/// object) over this object's block. `RET 4`.
pub fn fn_00403600(e: &mut Engine, this: Ptr, other: Ptr) {
    let cast = e
        .call(
            RT_DYNAMIC_CAST,
            &args![other, 0u32, TYPE_MAGIC_ITEM, TYPE_ALCHEMY_ITEM, 0u32],
        )
        .u32();
    if cast != 0 {
        let source = e.vcall(cast + 0x30, 0x28, &[]).u32();
        let destination = e.vcall(this.addr(), 0x28, &[]).u32();
        e.call(MEMCPY, &args![destination, source, 0x14u32]);
    }
}

// Translated from 00403660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The override of slot 8 of the `EffectItemList` vtable (`GetCost` in the
/// Xbox PDB; no map name). `this` is the `EffectItemList` subobject (object
/// plus 0x3c). When the data flag 1 is set (`004029d0` on the object) the cost
/// is the data block's `iCostOverride` converted to a float, otherwise the
/// effect list's own `GetCost` (`004063e0`, asked with 0). The result is in
/// `ST0`. The one stack argument (`RET 4`) is never read.
pub fn fn_00403660(e: &mut Engine, this: Ptr, _unused_1: u32) -> f64 {
    let object = Ptr::<AlchemyItem>::new(this.addr().wrapping_sub(0x3c));
    if e.call(HAS_FLAG, &args![object, 1u32]).bool() {
        f64::from(fn_004036b0(e, object))
    } else {
        e.call(EFFECT_LIST_GET_COST, &args![this, 0u32]).f64()
    }
}

// Translated from 004036b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `+0xb8` (`AlchemyItem` data `iCostOverride`, Xbox
/// PDB). Shared by identical-code folding: other classes' callers reach the
/// same address for their own word at `+0xb8`.
pub fn fn_004036b0(e: &mut Engine, this: Ptr<AlchemyItem>) -> i32 {
    e.get(this, AlchemyItem::iCostOverride)
}

// Translated from 004036d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AlchemyItem::IsWater` (Xbox PDB): whether the item is in the form list
/// whose editor id is `"WaterLIST"`. Looks the list up
/// (`GetFormByEditorID`); when it exists, asks whether its items
/// (`00500940`, the list at the form's `+0x18`) contain `this`
/// (`005f65d0`, called with the address of a stack word that holds `this`).
pub fn alchemy_item_is_water(e: &mut Engine, this: Ptr<AlchemyItem>) -> bool {
    let list_form = e
        .call(GET_FORM_BY_EDITOR_ID, &args![WATER_LIST_EDITOR_ID])
        .u32();
    if list_form == 0 {
        return false;
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), this.addr());
        let items = e.call(FORM_LIST_ITEMS, &args![list_form]).u32();
        e.call(LIST_CONTAINS, &args![items, local]).bool()
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_2b40, magic_item_object_magic_item_object(Ptr) -> Ptr),
        entry!(0x0040_2f10, alchemy_item_save_magic_item_components(Ptr)),
        entry!(0x0040_2f90, fn_00402f90(Ptr)),
        entry!(0x0040_30b0, fn_004030b0(Ptr<AlchemyItem>) -> Ptr),
        entry!(0x0040_30d0, fn_004030d0(Ptr<AlchemyItem>, Ptr)),
        entry!(0x0040_30f0, alchemy_item_init_item(Ptr<AlchemyItem>)),
        entry!(0x0040_3280, fn_00403280(Ptr, Ptr<TESFile>, u32)),
        entry!(0x0040_3570, fn_00403570(Ptr<TESFile>) -> u16),
        entry!(0x0040_3590, fn_00403590(Ptr, Ptr) -> bool),
        entry!(0x0040_3600, fn_00403600(Ptr, Ptr)),
        entry!(0x0040_3660, fn_00403660(Ptr, u32) -> f64),
        entry!(0x0040_36b0, fn_004036b0(Ptr<AlchemyItem>) -> i32),
        entry!(0x0040_36d0, alchemy_item_is_water(Ptr<AlchemyItem>) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// A recorded call: the callee and its argument words.
    type Call = (u32, Vec<u32>);

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// Makes `object` answer virtual calls: its vtable (at `vtable`) has
    /// `target` in the slot at byte offset `slot`.
    fn set_vtable_slot(e: &mut Engine, object: u32, vtable: u32, slot: u32, target: u32) {
        e.mem.map(vtable, 0x200);
        e.mem.set_u32(vtable + slot, target);
        e.mem.set_u32(object, vtable);
    }

    /// A stand-in form whose id (`+0xc`) is `id`.
    fn form_with_id(e: &mut Engine, id: u32) -> u32 {
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(form + 0xc, id);
        form
    }

    /// Starts recording the calls.
    fn record(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// The recorded calls after the top-level one.
    fn recorded(e: &mut Engine) -> Vec<Call> {
        e.call_log.take().unwrap()[1..].to_vec()
    }

    #[test]
    fn constructor_builds_the_magic_item_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_2b40,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (BOUND_OBJECT_CONSTRUCTOR, &[A::T(0x0)]),
                (MAGIC_ITEM_CONSTRUCTOR, &[A::T(0x30)]),
            ],
            stores: &[
                (0x0, 4, 0x0101_1b4c),
                (0x30, 4, 0x0101_1b04),
                (0x3c, 4, 0x0101_1aec),
            ],
        });
    }

    #[test]
    fn save_components_saves_every_component_then_the_form_data() {
        let mut e = Engine::new();
        for callee in [
            MODEL_TEXTURE_SWAP_SAVE,
            TEXTURE_SAVE,
            MESSAGE_ICON_SAVE,
            SCRIPTABLE_FORM_SAVE,
            DESTRUCTIBLE_SAVE,
            PICKUP_PUTDOWN_SAVE,
            EQUIP_TYPE_SAVE,
            FORM_SAVE_DATA,
        ] {
            e.register(callee, nothing);
        }
        record(&mut e);
        e.call(0x0040_2f10, &args![0x1030u32]);
        assert_eq!(
            recorded(&mut e),
            vec![
                (
                    MODEL_TEXTURE_SWAP_SAVE,
                    vec![0x104c, 0x4c44_4f4d, 0x5444_4f4d, 0x5344_4f4d]
                ),
                (TEXTURE_SAVE, vec![0x106c, 0x4e4f_4349]),
                (MESSAGE_ICON_SAVE, vec![0x1078, 0x4f43_494d]),
                (SCRIPTABLE_FORM_SAVE, vec![0x1088]),
                (DESTRUCTIBLE_SAVE, vec![0x10a4]),
                (PICKUP_PUTDOWN_SAVE, vec![0x10ac]),
                (EQUIP_TYPE_SAVE, vec![0x109c]),
                (FORM_SAVE_DATA, vec![0x1000]),
            ]
        );
    }

    /// Stand-ins for the getters and setters of the save and init code,
    /// acting on the object words (+0xc form id, +0xc0, +0xc8).
    fn word_accessors(e: &mut Engine) {
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(SET_ADDICTION_ITEM, |e, a| {
            e.mem.set_u32(a[0] + 0xc0, a[1]);
            Ret::default()
        });
        e.register(GET_CONSUMPTION_SOUND, |e, a| {
            e.mem.u32(a[0] + 0xc8).into_ret()
        });
    }

    /// An object with a data block of cost 5, flags 3 and the two given
    /// references; its `GetChunkID` answers 0x4d41_4c41. Returns the object
    /// and the `MagicItem` subobject.
    fn save_data_item(e: &mut Engine, spell: u32, sound: u32) -> (u32, u32) {
        let object = e.mem.alloc(0xd8);
        let this = object + 0x30;
        e.mem.set_i32(object + 0xb8, 5);
        e.mem.set_u8(object + 0xbc, 3);
        e.mem.set_u32(object + 0xc0, spell);
        e.mem.set_u32(object + 0xc4, 0x3f80_0000);
        e.mem.set_u32(object + 0xc8, sound);
        set_vtable_slot(e, this, 0x0200_0000, 0x24, 0x0200_1000);
        e.register(0x0200_1000, |_, _| 0x4d41_4c41u32.into_ret());
        word_accessors(e);
        e.map(0x011c_54ba, 1);
        e.register(ENDIAN_SWAP_ON_SAVE, |e, _| e.mem.u8(0x011c_54ba).into_ret());
        e.register(SWAP_DATA_ENDIAN, nothing);
        (object, this)
    }

    #[test]
    fn save_data_writes_form_ids_and_restores_the_pointers() {
        let mut e = Engine::new();
        let spell = form_with_id(&mut e, 0x0100_0aaa);
        let sound = form_with_id(&mut e, 0x0100_0bbb);
        let (object, this) = save_data_item(&mut e, spell, sound);
        let written = Rc::new(RefCell::new(Vec::new()));
        let seen = written.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            *seen.borrow_mut() = e.mem.bytes(a[1], 0x14);
            Ret::default()
        });
        record(&mut e);
        e.call(0x0040_2f90, &args![this]);
        assert_eq!(
            recorded(&mut e),
            vec![
                (FORM_ID, vec![spell]),
                (SET_ADDICTION_ITEM, vec![object, 0x0100_0aaa]),
                (GET_CONSUMPTION_SOUND, vec![object]),
                (GET_CONSUMPTION_SOUND, vec![object]),
                (GET_CONSUMPTION_SOUND, vec![object]),
                (FORM_ID, vec![sound]),
                (ENDIAN_SWAP_ON_SAVE, vec![]),
                (0x0200_1000, vec![this]),
                (ADD_CHUNK_DATA, vec![0x4d41_4c41, this + 0x88, 0x14]),
                (ENDIAN_SWAP_ON_SAVE, vec![]),
                (SET_ADDICTION_ITEM, vec![object, spell]),
            ]
        );
        // The chunk holds the ids where the pointers are.
        let mut want = vec![5, 0, 0, 0, 3, 0, 0, 0];
        want.extend_from_slice(&0x0100_0aaau32.to_le_bytes());
        want.extend_from_slice(&0x3f80_0000u32.to_le_bytes());
        want.extend_from_slice(&0x0100_0bbbu32.to_le_bytes());
        assert_eq!(*written.borrow(), want);
        assert_eq!(e.mem.u32(object + 0xc0), spell);
        assert_eq!(e.mem.u32(object + 0xc8), sound);
    }

    #[test]
    fn save_data_swaps_the_block_around_the_write_when_the_save_is_swapped() {
        let mut e = Engine::new();
        let (object, this) = save_data_item(&mut e, 0, 0);
        e.mem.set_u8(0x011c_54ba, 1);
        e.register(ADD_CHUNK_DATA, nothing);
        record(&mut e);
        e.call(0x0040_2f90, &args![this]);
        // No references: no ids are looked up, 0 is stored in their place.
        assert_eq!(
            recorded(&mut e),
            vec![
                (SET_ADDICTION_ITEM, vec![object, 0]),
                (GET_CONSUMPTION_SOUND, vec![object]),
                (GET_CONSUMPTION_SOUND, vec![object]),
                (ENDIAN_SWAP_ON_SAVE, vec![]),
                (SWAP_DATA_ENDIAN, vec![this + 0x88]),
                (0x0200_1000, vec![this]),
                (ADD_CHUNK_DATA, vec![0x4d41_4c41, this + 0x88, 0x14]),
                (ENDIAN_SWAP_ON_SAVE, vec![]),
                (SWAP_DATA_ENDIAN, vec![this + 0x88]),
                (SET_ADDICTION_ITEM, vec![object, 0]),
            ]
        );
        assert_eq!(e.mem.u32(object + 0xc0), 0);
        assert_eq!(e.mem.u32(object + 0xc8), 0);
    }

    #[test]
    fn folded_word_accessors_read_and_write_their_offsets() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0xd8);
        e.mem.set_u32(object + 0xc0, 0x1234);
        e.mem.set_i32(object + 0xb8, -7);
        assert_eq!(e.call(0x0040_30b0, &args![object]).u32(), 0x1234);
        assert_eq!(e.call(0x0040_36b0, &args![object]).i32(), -7);
        e.call(0x0040_30d0, &args![object, 0x5678u32]);
        assert_eq!(e.mem.u32(object + 0xc8), 0x5678);
        assert_eq!(e.mem.u32(object + 0xc0), 0x1234, "only +0xc8 is written");
    }

    /// The doubles `InitItem` needs: the form flags come from the object, the
    /// file index is 0x5000, `AddCompileIndex` sets bit 25 of the id, the
    /// lookup flips bit 31 and the cast succeeds unless `fail_for` names its
    /// target type. Returns the object, whose type name (slot 0x130) is 0x7777.
    fn init_item_setup(e: &mut Engine, fail_for: Rc<Cell<u32>>) -> u32 {
        let object = e.mem.alloc(0xd8);
        e.mem.set_u32(object + 0xc, 0x0100_0123);
        set_vtable_slot(e, object, 0x0200_0000, 0x130, 0x0200_1000);
        e.register(0x0200_1000, |_, _| 0x7777u32.into_ret());
        e.register(FORM_FLAG_8, |e, a| {
            (e.mem.u32(a[0] + 8) & 8 != 0).into_ret()
        });
        for callee in [
            SCRIPTABLE_FORM_INIT_ITEM,
            DESTRUCTIBLE_INIT_ITEM,
            MODEL_TEXTURE_SWAP_INIT_ITEM,
            MAGIC_ITEM_OBJECT_INIT_ITEM,
            PICKUP_PUTDOWN_INIT_ITEM,
            LOG_MESSAGE,
        ] {
            e.register(callee, nothing);
        }
        e.register(FORM_GET_FILE, |_, _| 0x5000u32.into_ret());
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0200_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| (a[0] ^ 0x8000_0000).into_ret());
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            if a[3] == fail_for.get() {
                0u32.into_ret()
            } else {
                a[0].into_ret()
            }
        });
        word_accessors(e);
        object
    }

    /// The log without the address of the stack local `AddCompileIndex` gets.
    fn without_local(log: Vec<Call>) -> Vec<Call> {
        log.into_iter()
            .map(|(callee, words)| {
                if callee == ADD_COMPILE_INDEX {
                    (callee, vec![0, words[1]])
                } else {
                    (callee, words)
                }
            })
            .collect()
    }

    #[test]
    fn init_item_does_nothing_when_form_flag_8_is_set() {
        let mut e = Engine::new();
        let object = init_item_setup(&mut e, Rc::new(Cell::new(0)));
        e.mem.set_u32(object + 8, 8);
        e.mem.set_u32(object + 0xc0, 0x0100_0aaa);
        record(&mut e);
        e.call(0x0040_30f0, &args![object]);
        assert_eq!(recorded(&mut e), vec![(FORM_FLAG_8, vec![object])]);
        assert_eq!(e.mem.u32(object + 0xc0), 0x0100_0aaa);
    }

    #[test]
    fn init_item_resolves_both_references() {
        let mut e = Engine::new();
        let object = init_item_setup(&mut e, Rc::new(Cell::new(0)));
        e.mem.set_u32(object + 0xc0, 0x0100_0aaa);
        e.mem.set_u32(object + 0xc8, 0x0100_0bbb);
        record(&mut e);
        e.call(0x0040_30f0, &args![object]);
        let log = without_local(recorded(&mut e));
        assert_eq!(
            log,
            vec![
                (FORM_FLAG_8, vec![object]),
                (SCRIPTABLE_FORM_INIT_ITEM, vec![object + 0x88, object]),
                (DESTRUCTIBLE_INIT_ITEM, vec![object + 0xa4, object]),
                (MODEL_TEXTURE_SWAP_INIT_ITEM, vec![object + 0x4c, object]),
                (MAGIC_ITEM_OBJECT_INIT_ITEM, vec![object]),
                (PICKUP_PUTDOWN_INIT_ITEM, vec![object + 0xac, object]),
                (FORM_GET_FILE, vec![object, 0xffff_ffff]),
                (ADD_COMPILE_INDEX, vec![0, 0x5000]),
                (LOOKUP_FORM, vec![0x0300_0aaa]),
                (
                    RT_DYNAMIC_CAST,
                    vec![0x8300_0aaa, 0, TYPE_TES_FORM, TYPE_SPELL_ITEM, 0]
                ),
                (SET_ADDICTION_ITEM, vec![object, 0x8300_0aaa]),
                (GET_CONSUMPTION_SOUND, vec![object]),
                (FORM_GET_FILE, vec![object, 0xffff_ffff]),
                (ADD_COMPILE_INDEX, vec![0, 0x5000]),
                (LOOKUP_FORM, vec![0x0300_0bbb]),
                (
                    RT_DYNAMIC_CAST,
                    vec![0x8300_0bbb, 0, TYPE_TES_FORM, TYPE_TES_SOUND, 0]
                ),
                (GET_CONSUMPTION_SOUND, vec![object]),
            ]
        );
        assert_eq!(e.mem.u32(object + 0xc0), 0x8300_0aaa);
        assert_eq!(e.mem.u32(object + 0xc8), 0x8300_0bbb);
    }

    #[test]
    fn init_item_logs_an_addiction_item_that_does_not_resolve() {
        let mut e = Engine::new();
        let fail = Rc::new(Cell::new(TYPE_SPELL_ITEM));
        let object = init_item_setup(&mut e, fail);
        e.mem.set_u32(object + 0xc0, 0x0100_0aaa);
        record(&mut e);
        e.call(0x0040_30f0, &args![object]);
        let log = without_local(recorded(&mut e));
        assert_eq!(e.mem.u32(object + 0xc0), 0, "the failed cast is stored");
        let tail = &log[log.len() - 4..];
        assert_eq!(
            tail,
            [
                (FORM_ID, vec![object]),
                (0x0200_1000, vec![object]),
                (
                    LOG_MESSAGE,
                    vec![MISSING_ADDICTION_ITEM, 0x0300_0aaa, 0x7777, 0x0100_0123]
                ),
                (GET_CONSUMPTION_SOUND, vec![object]),
            ]
        );
        // No consumption sound was set, so nothing else is looked up.
        assert_eq!(log.iter().filter(|c| c.0 == LOOKUP_FORM).count(), 1);
    }

    #[test]
    fn init_item_logs_a_consumption_sound_that_does_not_resolve() {
        let mut e = Engine::new();
        let fail = Rc::new(Cell::new(TYPE_TES_SOUND));
        let object = init_item_setup(&mut e, fail);
        e.mem.set_u32(object + 0xc8, 0x0100_0bbb);
        record(&mut e);
        e.call(0x0040_30f0, &args![object]);
        let log = without_local(recorded(&mut e));
        assert_eq!(e.mem.u32(object + 0xc8), 0);
        assert_eq!(
            log.last().unwrap(),
            &(
                LOG_MESSAGE,
                vec![MISSING_CONSUMPTION_SOUND, 0x0300_0bbb, 0x7777, 0x0100_0123]
            )
        );
        assert_eq!(log.iter().filter(|c| c.0 == LOOKUP_FORM).count(), 1);
    }

    #[test]
    fn init_item_with_no_references_only_initializes_the_components() {
        let mut e = Engine::new();
        let object = init_item_setup(&mut e, Rc::new(Cell::new(0)));
        record(&mut e);
        e.call(0x0040_30f0, &args![object]);
        let log = recorded(&mut e);
        assert_eq!(log.len(), 7, "flag, five components, sound read: {log:x?}");
        assert!(log
            .iter()
            .all(|c| c.0 != FORM_GET_FILE && c.0 != LOG_MESSAGE));
    }

    /// A file object whose current record has the given form version.
    fn file_with_version(e: &mut Engine, version: u16) -> u32 {
        let file: Ptr<TESFile> = e.new_object();
        e.set(file.at(TESFile::m_currentform), Form::sFormVersion, version);
        file.addr()
    }

    #[test]
    fn form_version_is_the_current_forms_version() {
        let mut e = Engine::new();
        let file = file_with_version(&mut e, 0xbeef);
        assert_eq!(e.call(0x0040_3570, &args![file]).u16(), 0xbeef);
    }

    fn chunk_setup(e: &mut Engine) {
        for callee in [
            LOAD_MODEL_CHUNK,
            LOAD_TEXTURE_CHUNK,
            LOAD_MESSAGE_ICON_CHUNK,
            LOAD_DESTRUCTIBLE_CHUNK,
            STORE_FORM_ID,
            SCRIPTABLE_FORM_INIT_ITEM,
            SET_WORD_AT_8,
            FORM_LOAD_DATA,
        ] {
            e.register(callee, nothing);
        }
        // The chunk word is 0x0100_0042, written where the caller says.
        e.register(GET_CHUNK_DATA, |e, a| {
            e.mem.set_u32(a[1], 0x0100_0042);
            1u32.into_ret()
        });
    }

    fn load_chunk(e: &mut Engine, this: u32, file: u32, tag: u32) -> Vec<Call> {
        record(e);
        e.call(0x0040_3280, &args![this, file, tag]);
        recorded(e)
    }

    /// The calls of a one-word chunk after its read, which must come first.
    fn after_read(file: u32, mut calls: Vec<Call>) -> Vec<Call> {
        assert_eq!((calls[0].0, calls[0].1[0]), (GET_CHUNK_DATA, file));
        calls.remove(0);
        calls
    }

    #[test]
    fn chunks_for_the_model_texture_icon_and_destructible_parts_go_to_their_loaders() {
        let mut e = Engine::new();
        chunk_setup(&mut e);
        let file = file_with_version(&mut e, 0);
        let this = 0x2000u32;
        for tag in [
            TAG_MODEL_MODL,
            TAG_MODEL_MODT,
            TAG_MODEL_MODD,
            TAG_MODEL_MOSD,
        ] {
            assert_eq!(
                load_chunk(&mut e, this, file, tag),
                vec![(LOAD_MODEL_CHUNK, vec![this + 0x1c, file])]
            );
        }
        assert_eq!(
            load_chunk(&mut e, this, file, TAG_ICON),
            vec![(LOAD_TEXTURE_CHUNK, vec![this + 0x3c, file])]
        );
        assert_eq!(
            load_chunk(&mut e, this, file, TAG_MESSAGE_ICON),
            vec![(LOAD_MESSAGE_ICON_CHUNK, vec![this + 0x48, file])]
        );
        for tag in [TAG_DESTRUCTIBLE_END, TAG_DESTRUCTIBLE_STAGE] {
            assert_eq!(
                load_chunk(&mut e, this, file, tag),
                vec![(LOAD_DESTRUCTIBLE_CHUNK, vec![this + 0x74, file])]
            );
        }
    }

    #[test]
    fn a_null_object_gives_null_component_addresses() {
        let mut e = Engine::new();
        chunk_setup(&mut e);
        let file = file_with_version(&mut e, 0);
        // `this` = 0x30 is the subobject of a null object.
        assert_eq!(
            load_chunk(&mut e, 0x30, file, TAG_MODEL_MODL),
            vec![(LOAD_MODEL_CHUNK, vec![0, file])]
        );
        assert_eq!(
            load_chunk(&mut e, 0x30, file, TAG_ICON),
            vec![(LOAD_TEXTURE_CHUNK, vec![0, file])]
        );
        assert_eq!(
            load_chunk(&mut e, 0x30, file, TAG_MESSAGE_ICON),
            vec![(LOAD_MESSAGE_ICON_CHUNK, vec![0, file])]
        );
        assert_eq!(
            load_chunk(&mut e, 0x30, file, TAG_DESTRUCTIBLE_END),
            vec![(LOAD_DESTRUCTIBLE_CHUNK, vec![0, file])]
        );
        // The form data chunk uses the adjusted address as it is.
        assert_eq!(
            load_chunk(&mut e, 0x30, file, TAG_FORM_DATA),
            vec![(FORM_LOAD_DATA, vec![0, file, 0, 0])]
        );
    }

    #[test]
    fn chunks_with_one_word_store_it_in_their_component() {
        let mut e = Engine::new();
        chunk_setup(&mut e);
        let file = file_with_version(&mut e, 0);
        let this = 0x2000u32;
        let log = load_chunk(&mut e, this, file, TAG_SCRIPT);
        assert_eq!(
            after_read(file, log),
            vec![
                (STORE_FORM_ID, vec![this + 0x58, 0x0100_0042]),
                (SCRIPTABLE_FORM_INIT_ITEM, vec![this + 0x58, this - 0x30]),
            ]
        );
        let log = load_chunk(&mut e, this, file, TAG_EQUIP_TYPE);
        assert_eq!(
            after_read(file, log),
            vec![(STORE_FORM_ID, vec![this + 0x6c, 0x0100_0042])]
        );
        let log = load_chunk(&mut e, this, file, TAG_PICKUP_SOUND);
        assert_eq!(
            after_read(file, log),
            vec![(STORE_FORM_ID, vec![this + 0x7c, 0x0100_0042])]
        );
        let log = load_chunk(&mut e, this, file, TAG_PUTDOWN_SOUND);
        assert_eq!(
            after_read(file, log),
            vec![(SET_WORD_AT_8, vec![this + 0x7c, 0x0100_0042])]
        );
    }

    #[test]
    fn the_form_data_chunk_goes_to_the_form_and_unknown_tags_are_ignored() {
        let mut e = Engine::new();
        chunk_setup(&mut e);
        let file = file_with_version(&mut e, 0);
        assert_eq!(
            load_chunk(&mut e, 0x2000, file, TAG_FORM_DATA),
            vec![(FORM_LOAD_DATA, vec![0x1fd0, file, 0, 0])]
        );
        assert_eq!(load_chunk(&mut e, 0x2000, file, 0x1234_5678), vec![]);
    }

    #[test]
    fn the_texture_swap_chunk_is_read_whole_and_unpacked_with_the_form_version() {
        let mut e = Engine::new();
        chunk_setup(&mut e);
        let file = file_with_version(&mut e, 4);
        e.register(FILE_CHUNK_SIZE, |_, _| 0x21u32.into_ret());
        e.register(GET_CHUNK_DATA_SIZED, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, i as u8 + 1);
            }
            1u32.into_ret()
        });
        let unpacked = Rc::new(RefCell::new(Vec::new()));
        let seen = unpacked.clone();
        e.register_double(UNPACK_TEXTURE_SWAP, move |e, a| {
            *seen.borrow_mut() = e.mem.bytes(a[1], a[2]);
            Ret::default()
        });
        let this = 0x2000u32;
        let log = load_chunk(&mut e, this, file, TAG_MODEL_MODS);
        assert_eq!(log[0], (FILE_CHUNK_SIZE, vec![file]));
        let buffer = log[1].1[1];
        assert_eq!(log[1], (GET_CHUNK_DATA_SIZED, vec![file, buffer, 0x21]));
        assert_eq!(
            log[2],
            (UNPACK_TEXTURE_SWAP, vec![this + 0x1c, buffer, 0x21, 4])
        );
        assert_eq!(log.len(), 3);
        assert_eq!(*unpacked.borrow(), (1..=0x21).collect::<Vec<u8>>());
    }

    /// A `MagicItem` subobject whose `GetData` (slot 0x28) answers the
    /// address of its own data block at +0x88; returns the subobject.
    fn magic_item_with_data(e: &mut Engine, block: [u8; 0x14]) -> u32 {
        let object = e.mem.alloc(0x100);
        let this = object + 0x30;
        set_vtable_slot(e, this, 0x0200_0000, 0x28, 0x0200_1000);
        e.register(0x0200_1000, |_, a| (a[0] + 0x88).into_ret());
        e.mem.write(this + 0x88, &block);
        this
    }

    #[test]
    fn compare_differs_when_the_other_is_not_an_alchemy_item() {
        let mut e = Engine::new();
        let this = magic_item_with_data(&mut e, [1; 0x14]);
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        record(&mut e);
        assert!(e.call(0x0040_3590, &args![this, 0x7000u32]).bool());
        assert_eq!(
            recorded(&mut e),
            vec![(
                RT_DYNAMIC_CAST,
                vec![0x7000, 0, TYPE_MAGIC_ITEM, TYPE_ALCHEMY_ITEM, 0]
            )]
        );
    }

    #[test]
    fn compare_checks_the_whole_data_block() {
        let mut e = Engine::new();
        let this = magic_item_with_data(&mut e, [7; 0x14]);
        let mut other_block = [7; 0x14];
        let other = magic_item_with_data(&mut e, other_block);
        // The cast of the MagicItem pointer is the object address.
        e.register_double(RT_DYNAMIC_CAST, move |_, _| (other - 0x30).into_ret());
        assert!(!e.call(0x0040_3590, &args![this, other]).bool());
        // A difference in the last byte is found.
        other_block[0x13] = 8;
        e.mem.write(other + 0x88, &other_block);
        assert!(e.call(0x0040_3590, &args![this, other]).bool());
    }

    #[test]
    fn copy_moves_the_block_only_for_an_alchemy_item() {
        let mut e = Engine::new();
        let this = magic_item_with_data(&mut e, [1; 0x14]);
        let other = magic_item_with_data(&mut e, [9; 0x14]);
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call(0x0040_3600, &args![this, other]);
        assert_eq!(e.mem.bytes(this + 0x88, 0x14), vec![1; 0x14]);
        e.register_double(RT_DYNAMIC_CAST, move |_, _| (other - 0x30).into_ret());
        record(&mut e);
        e.call(0x0040_3600, &args![this, other]);
        assert_eq!(e.mem.bytes(this + 0x88, 0x14), vec![9; 0x14]);
        // The other object is asked for its block first, then this one.
        let asked: Vec<Call> = recorded(&mut e)
            .into_iter()
            .filter(|c| c.0 == 0x0200_1000)
            .collect();
        assert_eq!(
            asked,
            vec![(0x0200_1000, vec![other]), (0x0200_1000, vec![this])]
        );
    }

    /// An `AlchemyItem` whose list (+0x3c) is what `GetCost` gets.
    fn cost_setup(e: &mut Engine, flag_set: bool, cost_override: i32) -> u32 {
        let object = e.mem.alloc(0xd8);
        e.mem.set_i32(object + 0xb8, cost_override);
        e.register_double(HAS_FLAG, move |_, _| flag_set.into_ret());
        e.register(EFFECT_LIST_GET_COST, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        object
    }

    #[test]
    fn cost_is_the_override_when_the_flag_is_set() {
        let mut e = Engine::new();
        let object = cost_setup(&mut e, true, -7);
        record(&mut e);
        let cost = e.call(0x0040_3660, &args![object + 0x3c, 99u32]).f64();
        assert_eq!(cost, -7.0);
        assert_eq!(recorded(&mut e), vec![(HAS_FLAG, vec![object, 1])]);
    }

    #[test]
    fn cost_is_the_effect_list_cost_otherwise() {
        let mut e = Engine::new();
        let object = cost_setup(&mut e, false, 1000);
        record(&mut e);
        let cost = e.call(0x0040_3660, &args![object + 0x3c, 0u32]).f64();
        assert_eq!(cost, 2.5);
        assert_eq!(
            recorded(&mut e),
            vec![
                (HAS_FLAG, vec![object, 1]),
                (EFFECT_LIST_GET_COST, vec![object + 0x3c, 0]),
            ]
        );
    }

    #[test]
    fn cost_passes_the_effect_list_value_through_unchanged() {
        let mut e = Engine::new();
        let object = cost_setup(&mut e, false, 0);
        e.register(EFFECT_LIST_GET_COST, |_, _| Ret {
            st0: f64::from_bits(0x7ff0_0000_0000_0001),
            ..Ret::default()
        });
        let cost = e.call(0x0040_3660, &args![object + 0x3c, 0u32]).f64();
        assert_eq!(cost.to_bits(), 0x7ff0_0000_0000_0001);
    }

    #[test]
    fn is_water_looks_for_the_item_in_the_water_list() {
        let mut e = Engine::new();
        e.register(GET_FORM_BY_EDITOR_ID, |_, a| {
            assert_eq!(a, [WATER_LIST_EDITOR_ID]);
            0x4000u32.into_ret()
        });
        e.register(FORM_LIST_ITEMS, |_, a| (a[0] + 0x18).into_ret());
        // The list contains the item stored in the word it is asked about.
        e.register(LIST_CONTAINS, |e, a| {
            assert_eq!(a[0], 0x4018);
            (e.mem.u32(a[1]) == 0x9000).into_ret()
        });
        assert!(e.call(0x0040_36d0, &args![0x9000u32]).bool());
        assert!(!e.call(0x0040_36d0, &args![0x9100u32]).bool());
    }

    #[test]
    fn is_water_is_false_without_the_list() {
        let mut e = Engine::new();
        e.register(GET_FORM_BY_EDITOR_ID, |_, _| 0u32.into_ret());
        record(&mut e);
        assert!(!e.call(0x0040_36d0, &args![0x9000u32]).bool());
        assert_eq!(
            recorded(&mut e),
            vec![(GET_FORM_BY_EDITOR_ID, vec![WATER_LIST_EDITOR_ID])]
        );
    }
}
