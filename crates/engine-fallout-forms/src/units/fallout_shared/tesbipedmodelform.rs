//! `fallout shared/tesbipedmodelform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00401170` (unnamed in the engine map): `thiscall` getter, the form type byte at `this + 4`.
const FORM_TYPE_BYTE: u32 = 0x0040_1170;
/// `00726070` (unnamed in the engine map): `thiscall`; the dword at `this + 4` (a list node's
/// next).
const LIST_NEXT: u32 = 0x0072_6070;
/// `00500940` (unnamed in the engine map): `thiscall`; `this + 0x18` (the first node of a
/// container's list).
const LIST_FIRST_NODE_AT_18: u32 = 0x0050_0940;
/// `008256d0` (unnamed in the engine map): `thiscall` on a list node, whether the node is empty
/// (no item and no next).
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0`: returns `this` (the address of a node's item pointer).
const LIST_NODE_ITEM_SLOT: u32 = 0x0068_15c0;

// Translated from 00480db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::GetFormAsBipedModel` (Xbox PDB): `cdecl`, one stack word. For a non-null
/// form, answers its biped model part by the form's type byte (`00401170`): type 0x18 at
/// `form + 0x70`, type 0x1a at `form + 0x68`, type 0x60 at `form + 0x70`; null for any other type
/// or a null form. (The exe tests the form for null again inside each case; those tests cannot
/// fail and are kept.)
pub fn tes_biped_model_form_get_form_as_biped_model(e: &mut Engine, form: Ptr) -> Ptr {
    let mut result = 0;
    if !form.is_null() {
        let part_at = |offset: u32| {
            if form.is_null() {
                0
            } else {
                form.addr().wrapping_add(offset)
            }
        };
        match e.call(FORM_TYPE_BYTE, &args![form]).u32() {
            0x18 => result = part_at(0x70),
            0x1a => result = part_at(0x68),
            0x60 => result = part_at(0x70),
            _ => {}
        }
    }
    Ptr::new(result)
}

// Translated from 00480af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::FillsBipedSlot` (Xbox PDB): `thiscall`, `RET 0xc` (a slot number, a flag
/// byte, a container). False when the flag is set, the slot is 1 and bit 0 of the flags dword
/// at `this + 4` is set. Otherwise true when bit `slot & 31` of that dword is set; if it is
/// not and a container was given, walks the container's list (its first node comes from the
/// container's own list head `00726070`, then `00500940` on that) and asks every item that is a
/// biped model form (`GetFormAsBipedModel`) the same question, without a container, until one
/// answers true or the list ends.
pub fn tes_biped_model_form_fills_biped_slot(
    e: &mut Engine,
    this: Ptr,
    slot: u32,
    flag: u8,
    container: Ptr,
) -> bool {
    let flags = this.addr().wrapping_add(4);
    if flag != 0 && slot == 1 && e.mem.u32(flags) & 1 != 0 {
        return false;
    }
    let mut filled = (1u32.wrapping_shl(slot & 0x1f) & e.mem.u32(flags)) != 0;
    if !filled && !container.is_null() {
        let head = e.call(LIST_NEXT, &args![container]).u32();
        if head != 0 {
            let mut node = e.call(LIST_FIRST_NODE_AT_18, &args![head]).u32();
            while !filled && node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
                let item_slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
                let form = e.mem.u32(item_slot);
                let part = tes_biped_model_form_get_form_as_biped_model(e, Ptr::new(form));
                if !part.is_null() {
                    filled = tes_biped_model_form_fills_biped_slot(e, part, slot, flag, Ptr::NULL);
                }
                node = e.call(LIST_NEXT, &args![node]).u32();
            }
        }
    }
    filled
}

/// The components `InitializeDataComponent` and `ClearDataComponent` run,
/// in the exe's order: `bipedModel[0]`, `worldModel[0]`, `inventoryIcon[0]`,
/// `bipedModel[1]`, `worldModel[1]`, `inventoryIcon[1]`, `ConstraintTemplate`
/// (Xbox PDB names; the PC offsets are the same).
const DATA_COMPONENTS: [u32; 7] = [0x0c, 0x4c, 0x8c, 0x2c, 0x6c, 0x98, 0xc4];
/// Whether a model has a file name (`0048cee0`).
const MODEL_HAS_NAME: u32 = 0x0048_cee0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVTESForm@@` and the biped model form's
/// (`01183978`), as the code passes them.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_BIPED_MODEL_FORM: u32 = 0x0118_3978;
/// The model loader (a pointer at `011c3b3c`).
const MODEL_LOADER: u32 = 0x011c_3b3c;

// Translated from 004804d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::InitializeDataComponent` (Xbox PDB): clears the
/// biped model data (`+4` word, `+8` byte), then runs vtable slot 0 of
/// each data component (`DATA_COMPONENTS`).
pub fn tes_biped_model_form_initialize_data_component(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u8(this.addr() + 8, 0);
    for offset in DATA_COMPONENTS {
        e.vcall(this.addr() + offset, 0, &args![]);
    }
}

// Translated from 00480570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::ClearDataComponent` (Xbox PDB): vtable slot 4 of
/// each data component (`DATA_COMPONENTS`).
pub fn tes_biped_model_form_clear_data_component(e: &mut Engine, this: Ptr) {
    for offset in DATA_COMPONENTS {
        e.vcall(this.addr() + offset, 4, &args![]);
    }
}

// Translated from 00480750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::InitItem` (Xbox PDB): `TESModelTextureSwap::InitItem`
/// (Xbox PDB, `0048aa80`) with `form` on `bipedModel[0]` (`+0xC`),
/// `worldModel[0]` (`+0x4C`), `bipedModel[1]` (`+0x2C`) and `worldModel[1]`
/// (`+0x6C`); `RET 4`.
pub fn tes_biped_model_form_init_item(e: &mut Engine, this: Ptr, form: u32) {
    for offset in [0x0c, 0x4c, 0x2c, 0x6c] {
        e.call(0x0048_aa80, &args![this.addr() + offset, form]);
    }
}

// Translated from 00480bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00480bd0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESBipedModelForm::AddToBiped` (Xbox PDB), `RET 0xC`: `biped`, the
/// `sex` (1 falls back to 0 when `bipedModel[1]` has no file name,
/// `0048cee0`) and a third word. Gives `004abad0` (on `biped`) the form
/// (`this` cast from `01183978` to `TESForm`), `bipedModel[sex]`
/// (`this + 0xC + sex * 0x20`) and the third word. Then, when the
/// constraint template (`+0xC4`) has a file name, the model it names
/// (`00480ce0`) is looked up (`ModelLoader::FindModel`, `004472a0`, on the
/// loader at `011c3b3c`, into a stack `NiPointer` made by `00633c90` with
/// 0 and released by `0040c110`); a model found (`00559450`) has its
/// object (`0043b230`) applied (`Actor::ApplyConstrTemplate`, `008bbc40`)
/// to what `0043f010` answers for `biped`.
pub fn tes_biped_model_form_add_to_biped(
    e: &mut Engine,
    this: Ptr,
    biped: u32,
    sex: u32,
    third: u32,
) {
    let base = this.addr();
    let mut sex = sex;
    if sex == 1 && e.call(MODEL_HAS_NAME, &args![base + 0x2c]).u32() == 0 {
        sex = 0;
    }
    let model = base.wrapping_add(sex.wrapping_shl(5)).wrapping_add(0xc);
    let form = e
        .call(
            RT_DYNAMIC_CAST,
            &args![this, 0u32, TYPE_BIPED_MODEL_FORM, TYPE_TES_FORM, 0u32],
        )
        .u32();
    e.call(0x004a_bad0, &args![biped, form, model, third]);
    if e.call(MODEL_HAS_NAME, &args![base + 0xc4]).u32() == 0 {
        return;
    }
    let name = tes_biped_model_form_constraint_template_name(e, this);
    e.with_stack(4, |e, pointer| {
        e.call(0x0063_3c90, &args![pointer, 0u32]);
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(0x0044_72a0, &args![loader, name, pointer]);
        if e.call(0x0055_9450, &args![pointer]).u32() != 0 {
            let found = e.call(0x0055_9450, &args![pointer]).u32();
            let object = e.call(0x0043_b230, &args![found]).u32();
            let actor = e.call(0x0043_f010, &args![biped]).u32();
            e.call(0x008b_bc40, &args![actor, object]);
        }
        e.call(0x0040_c110, &args![pointer]);
    });
}

// Translated from 00480ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constraint template's model name: vtable slot 0x14 of the
/// `TESModelRDT` at `+0xC4`.
pub fn tes_biped_model_form_constraint_template_name(e: &mut Engine, this: Ptr) -> u32 {
    e.vcall(this.addr() + 0xc4, 0x14, &args![]).u32()
}

// Translated from 00480d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::IsPowerArmor` (Xbox PDB): bit 0x20 of the biped
/// model flags byte (`+8`).
pub fn tes_biped_model_form_is_power_armor(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 8) & 0x20 != 0
}

// Translated from 00480d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 4 of the biped model flags byte (`+8`).
pub fn fn_00480d30(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 8) & 4 != 0
}

// Translated from 00480d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` true) or clears bit 0x10 of the biped model flags byte
/// (`+8`); `RET 4`.
pub fn fn_00480d50(e: &mut Engine, this: Ptr, set: bool) {
    let flags = e.mem.u8(this.addr() + 8);
    let flags = if set { flags | 0x10 } else { flags & 0xef };
    e.mem.set_u8(this.addr() + 8, flags);
}

// Translated from 00480d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::GetPlayable` (Xbox PDB): true unless bit 0x40 of the
/// biped model flags byte (`+8`) is set.
pub fn tes_biped_model_form_get_playable(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 8) & 0x40 == 0
}

// Translated from 00481010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::GetWorldTESModel` (Xbox PDB), cdecl with two words:
/// `form` cast from `TESForm` to the biped model form (`01183978`); null
/// when that fails; `worldModel[0]` (`+0x4C`) without a reference;
/// otherwise the world model for the sex of the reference's owner
/// (`TESObjectREFR::GetOwner`, `00567790`): `TESActorBase::GetSex`
/// (`005f0cc0`) when the owner has form type 0x2A (`00401170`), else 0,
/// through `GetWorldTESModel_ov2`.
pub fn tes_biped_model_form_get_world_tes_model(e: &mut Engine, form: u32, reference: u32) -> u32 {
    let biped = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_BIPED_MODEL_FORM, 0u32],
        )
        .u32();
    if biped == 0 {
        return 0;
    }
    if reference == 0 {
        return biped + 0x4c;
    }
    let owner = e.call(0x0056_7790, &args![reference]).u32();
    let mut sex = 0;
    if owner != 0 && e.call(FORM_TYPE_BYTE, &args![owner]).u32() == 0x2a {
        sex = e.call(0x005f_0cc0, &args![owner]).u32();
    }
    tes_biped_model_form_get_world_tes_model_ov2(e, Ptr::new(biped), sex)
}

// Translated from 00481110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::GetWorldTESModel_ov2` (Xbox PDB): `worldModel[sex]`
/// (`this + 0x4C + sex * 0x20`), sex 1 falling back to 0 when that model
/// has no file name (`0048cee0`); `RET 4`.
pub fn tes_biped_model_form_get_world_tes_model_ov2(e: &mut Engine, this: Ptr, sex: u32) -> u32 {
    let mut sex = sex;
    let model = this
        .addr()
        .wrapping_add(sex.wrapping_shl(5))
        .wrapping_add(0x4c);
    if e.call(MODEL_HAS_NAME, &args![model]).u32() == 0 && sex == 1 {
        sex = 0;
    }
    this.addr()
        .wrapping_add(sex.wrapping_shl(5))
        .wrapping_add(0x4c)
}

// Translated from 00481150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::GetBipedTESModel_ov2` (engine map): `bipedModel[sex]`
/// (`this + 0xC + sex * 0x20`), sex 1 falling back to 0 when that model has no file name
/// (`0048cee0`); `RET 4`.
pub fn tes_biped_model_form_get_biped_tes_model_ov2(e: &mut Engine, this: Ptr, sex: u32) -> u32 {
    let mut sex = sex;
    let model = this
        .addr()
        .wrapping_add(sex.wrapping_shl(5))
        .wrapping_add(0xc);
    if e.call(MODEL_HAS_NAME, &args![model]).u32() == 0 && sex == 1 {
        sex = 0;
    }
    this.addr()
        .wrapping_add(sex.wrapping_shl(5))
        .wrapping_add(0xc)
}

/// `00401500`: whether saved data is byte-swapped (`AL`); `00503210`
/// (`ECX`): swaps the biped model data's words.
const SAVE_SWAPS_ENDIAN: u32 = 0x0040_1500;
const SWAP_BIPED_DATA: u32 = 0x0050_3210;
/// `TESForm::__AddChunkData` (Xbox PDB, `00485990`, cdecl: code, data,
/// size), `TESModelTextureSwap::Save` (`0048a520`, `RET 0xC`),
/// `TESModel::SaveFlags` (`00489240`, `RET 4`), `TESTexture::Save`
/// (`0048e370`, `RET 4`), `BGSMessageIcon::Save` (`0047a240`, `RET 4`) and
/// `TESModel::Save` (`004891d0`, `RET 8`), by the decompiler's names.
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
const TEXTURE_SWAP_SAVE: u32 = 0x0048_a520;
const MODEL_SAVE_FLAGS: u32 = 0x0048_9240;
const TEXTURE_SAVE: u32 = 0x0048_e370;
const MESSAGE_ICON_SAVE: u32 = 0x0047_a240;
const MODEL_SAVE: u32 = 0x0048_91d0;
/// `memcpy` (`00401460`) and `_memcmp` (`00ec4835`), cdecl, three words.
const MEMCPY: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;
/// `.?AVBaseFormComponent@@`, the type the component copy and compare cast
/// from.
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
/// The components the copy and the compare visit, in the exe's order:
/// `bipedModel[0]`, `worldModel[0]`, `inventoryIcon[0]`, `messageIcon[0]`,
/// `bipedModel[1]`, `worldModel[1]`, `inventoryIcon[1]`, `messageIcon[1]`,
/// `ConstraintTemplate` (Xbox PDB names).
const COPY_COMPONENTS: [u32; 9] = [0x0c, 0x4c, 0x8c, 0xa4, 0x2c, 0x6c, 0x98, 0xb4, 0xc4];
/// `TESObjectREFR::GetOwner` (`00567790`) and `TESActorBase::GetSex`
/// (`005f0cc0`).
const GET_OWNER: u32 = 0x0056_7790;
const GET_SEX: u32 = 0x005f_0cc0;

/// The sex of `reference`'s owner as the getters below pick it: 0 unless
/// the owner (`TESObjectREFR::GetOwner`) has form type 0x2A, then
/// `TESActorBase::GetSex`.
fn owner_sex(e: &mut Engine, reference: u32) -> u32 {
    let owner = e.call(GET_OWNER, &args![reference]).u32();
    if owner != 0 && e.call(FORM_TYPE_BYTE, &args![owner]).u32() == 0x2a {
        e.call(GET_SEX, &args![owner]).u32()
    } else {
        0
    }
}

/// `form` cast from `TESForm` to the biped model form (`01183978`).
fn as_biped_model_form(e: &mut Engine, form: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_BIPED_MODEL_FORM, 0u32],
    )
    .u32()
}

// Translated from 00480610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::Save` (Xbox PDB; `ECX`): the biped model data
/// (`BMDT`, 8 bytes at +4, swapped before and after when saving swaps),
/// then each component: `MODL`/`MODT`/`MODS` and `MODD` for
/// `bipedModel[0]`, `MOD2`/`MO2T`/`MO2S` for `worldModel[0]`, `ICON` and
/// `MICO` for the icons, `MOD3`/`MO3T`/`MO3S` and `MOSD` for
/// `bipedModel[1]`, `MOD4`/`MO4T`/`MO4S` for `worldModel[1]`, `ICO2` and
/// `MIC2`, and `BMCT` (with 0) for the constraint template.
pub fn tes_biped_model_form_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.call(SAVE_SWAPS_ENDIAN, &args![]).bool() {
        e.call(SWAP_BIPED_DATA, &args![base + 4]);
    }
    e.call(ADD_CHUNK_DATA, &args![0x5444_4d42u32, base + 4, 8u32]);
    if e.call(SAVE_SWAPS_ENDIAN, &args![]).bool() {
        e.call(SWAP_BIPED_DATA, &args![base + 4]);
    }
    let swap = |e: &mut Engine, at: u32, codes: [u32; 3]| {
        e.call(
            TEXTURE_SWAP_SAVE,
            &args![base + at, codes[0], codes[1], codes[2]],
        );
    };
    swap(e, 0x0c, [0x4c44_4f4d, 0x5444_4f4d, 0x5344_4f4d]);
    e.call(MODEL_SAVE_FLAGS, &args![base + 0x0c, 0x4444_4f4du32]);
    swap(e, 0x4c, [0x3244_4f4d, 0x5432_4f4d, 0x5332_4f4d]);
    e.call(TEXTURE_SAVE, &args![base + 0x8c, 0x4e4f_4349u32]);
    e.call(MESSAGE_ICON_SAVE, &args![base + 0xa4, 0x4f43_494du32]);
    swap(e, 0x2c, [0x3344_4f4d, 0x5433_4f4d, 0x5333_4f4d]);
    e.call(MODEL_SAVE_FLAGS, &args![base + 0x2c, 0x4453_4f4du32]);
    swap(e, 0x6c, [0x3444_4f4d, 0x5434_4f4d, 0x5334_4f4d]);
    e.call(TEXTURE_SAVE, &args![base + 0x98, 0x324f_4349u32]);
    e.call(MESSAGE_ICON_SAVE, &args![base + 0xb4, 0x3243_494du32]);
    e.call(MODEL_SAVE, &args![base + 0xc4, 0x5443_4d42u32, 0u32]);
}

// Translated from 004807a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::CopyComponent` (Xbox PDB; `RET 4`: the source
/// component): when the source casts from `BaseFormComponent` to the biped
/// model form, the 8 bytes of biped model data and each component's copy
/// (its vtable slot +8 with the source's component).
pub fn tes_biped_model_form_copy_component(e: &mut Engine, this: Ptr, source: u32) {
    let other = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                source,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_BIPED_MODEL_FORM,
                0u32
            ],
        )
        .u32();
    if other == 0 {
        return;
    }
    e.call(MEMCPY, &args![this.addr() + 4, other + 4, 8u32]);
    for offset in COPY_COMPONENTS {
        e.vcall(this.addr() + offset, 8, &args![other + offset]);
    }
}

// Translated from 004808f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::CompareComponent` (Xbox PDB; `RET 4`: the other
/// component): true (different) unless the other casts to the biped model
/// form, the 8 bytes of biped model data match and no component's compare
/// (its vtable slot +0xC with the other's component) answers true.
pub fn tes_biped_model_form_compare_component(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let form = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_BIPED_MODEL_FORM,
                0u32
            ],
        )
        .u32();
    if form == 0 {
        return true;
    }
    if e.call(MEMCMP, &args![this.addr() + 4, form + 4, 8u32])
        .u32()
        != 0
    {
        return true;
    }
    for offset in COPY_COMPONENTS {
        if e.vcall(this.addr() + offset, 0xc, &args![form + offset])
            .bool()
        {
            return true;
        }
    }
    false
}

// Translated from 00480ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two biped model forms share a biped slot (`RET 4`: the other;
/// the engine map has no name): the AND of their slot words (+4) is not 0.
pub fn fn_00480ad0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.mem.u32(this.addr() + 4) & e.mem.u32(other.addr() + 4) != 0
}

// Translated from 00480e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::GetIcon` (Xbox PDB; cdecl: a form and a reference):
/// 0 when the form is no biped model form; without a reference
/// `worldModel[0]`'s vtable slot +0x14 (its file name); otherwise
/// [`tes_biped_model_form_get_icon_ov2`] for the sex of the reference's
/// owner. The function ends in those calls, so their `EAX` is the answer.
pub fn tes_biped_model_form_get_icon(e: &mut Engine, form: u32, reference: u32) -> u32 {
    let biped = as_biped_model_form(e, form);
    if biped == 0 {
        return 0;
    }
    if reference == 0 {
        return e.vcall(biped + 0x4c, 0x14, &args![]).u32();
    }
    let sex = owner_sex(e, reference);
    tes_biped_model_form_get_icon_ov2(e, Ptr::new(biped), sex)
}

// Translated from 00480ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The inventory icon's text (cdecl: a form and a reference; the engine map
/// has no name): 0 when the form is no biped model form; without a
/// reference `00408da0` on `inventoryIcon[0]` (+0x8C); otherwise `00481230`
/// for the sex of the reference's owner. The callee's `EAX` is the answer.
pub fn fn_00480ef0(e: &mut Engine, form: u32, reference: u32) -> u32 {
    let biped = as_biped_model_form(e, form);
    if biped == 0 {
        return 0;
    }
    if reference == 0 {
        return e.call(0x0040_8da0, &args![biped + 0x8c]).u32();
    }
    let sex = owner_sex(e, reference);
    e.call(0x0048_1230, &args![biped, sex]).u32()
}

// Translated from 00480f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The message icon's answer (cdecl: a form and a reference; the engine
/// map has no name): 0 when the form is no biped model form; without a
/// reference `0047a300` on `messageIcon[0]` (+0xA4); otherwise `00481280`
/// for the sex of the reference's owner. The callee's `EAX` is the answer.
pub fn fn_00480f80(e: &mut Engine, form: u32, reference: u32) -> u32 {
    let biped = as_biped_model_form(e, form);
    if biped == 0 {
        return 0;
    }
    if reference == 0 {
        return e.call(0x0047_a300, &args![biped + 0xa4]).u32();
    }
    let sex = owner_sex(e, reference);
    e.call(0x0048_1280, &args![biped, sex]).u32()
}

// Translated from 00481090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::GetBipedTESModel` (Xbox PDB; cdecl: a form and a
/// reference): null when the form is no biped model form; without a
/// reference `bipedModel[0]` (+0xC); otherwise
/// [`tes_biped_model_form_get_biped_tes_model_ov2`] for the sex of the
/// reference's owner.
pub fn tes_biped_model_form_get_biped_tes_model(e: &mut Engine, form: u32, reference: u32) -> u32 {
    let biped = as_biped_model_form(e, form);
    if biped == 0 {
        return 0;
    }
    if reference == 0 {
        return biped + 0xc;
    }
    let sex = owner_sex(e, reference);
    tes_biped_model_form_get_biped_tes_model_ov2(e, Ptr::new(biped), sex)
}

// Translated from 00481190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::GetIcon_ov2` (Xbox PDB; `RET 4`: the sex):
/// `worldModel[sex]`'s vtable slot +0x14 (sex 1 falling back to 0 when
/// that model has no file name, `0048cee0`); the slot's `EAX` is the
/// answer.
pub fn tes_biped_model_form_get_icon_ov2(e: &mut Engine, this: Ptr, sex: u32) -> u32 {
    let mut sex = sex;
    let model = this
        .addr()
        .wrapping_add(sex.wrapping_shl(5))
        .wrapping_add(0x4c);
    if e.call(MODEL_HAS_NAME, &args![model]).u32() == 0 && sex == 1 {
        sex = 0;
    }
    let model = this
        .addr()
        .wrapping_add(sex.wrapping_shl(5))
        .wrapping_add(0x4c);
    e.vcall(model, 0x14, &args![]).u32()
}

// Translated from 004811e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::GetBipedModel` (Xbox PDB; `RET 4`: the sex):
/// `bipedModel[sex]`'s vtable slot +0x14 (sex 1 falling back to 0 when that
/// model has no file name, `0048cee0`); the slot's `EAX` is the answer.
pub fn tes_biped_model_form_get_biped_model(e: &mut Engine, this: Ptr, sex: u32) -> u32 {
    let mut sex = sex;
    let model = this
        .addr()
        .wrapping_add(sex.wrapping_shl(5))
        .wrapping_add(0xc);
    if e.call(MODEL_HAS_NAME, &args![model]).u32() == 0 && sex == 1 {
        sex = 0;
    }
    let model = this
        .addr()
        .wrapping_add(sex.wrapping_shl(5))
        .wrapping_add(0xc);
    e.vcall(model, 0x14, &args![]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_0610, tes_biped_model_form_save(Ptr)),
        entry!(0x0048_07a0, tes_biped_model_form_copy_component(Ptr, u32)),
        entry!(
            0x0048_08f0,
            tes_biped_model_form_compare_component(Ptr, u32) -> bool
        ),
        entry!(0x0048_0ad0, fn_00480ad0(Ptr, Ptr) -> bool),
        entry!(0x0048_0e60, tes_biped_model_form_get_icon(u32, u32) -> u32),
        entry!(0x0048_0ef0, fn_00480ef0(u32, u32) -> u32),
        entry!(0x0048_0f80, fn_00480f80(u32, u32) -> u32),
        entry!(
            0x0048_1090,
            tes_biped_model_form_get_biped_tes_model(u32, u32) -> u32
        ),
        entry!(0x0048_1190, tes_biped_model_form_get_icon_ov2(Ptr, u32) -> u32),
        entry!(0x0048_11e0, tes_biped_model_form_get_biped_model(Ptr, u32) -> u32),
        entry!(0x0048_1150, tes_biped_model_form_get_biped_tes_model_ov2(Ptr, u32) -> u32),
        entry!(0x0048_0db0, tes_biped_model_form_get_form_as_biped_model(Ptr) -> Ptr),
        entry!(0x0048_0af0, tes_biped_model_form_fills_biped_slot(Ptr, u32, u8, Ptr) -> bool),
        entry!(
            0x0048_04d0,
            tes_biped_model_form_initialize_data_component(Ptr)
        ),
        entry!(0x0048_0570, tes_biped_model_form_clear_data_component(Ptr)),
        entry!(0x0048_0750, tes_biped_model_form_init_item(Ptr, u32)),
        entry!(
            0x0048_0bd0,
            tes_biped_model_form_add_to_biped(Ptr, u32, u32, u32)
        ),
        entry!(0x0048_0ce0, tes_biped_model_form_constraint_template_name(Ptr) -> u32),
        entry!(0x0048_0d10, tes_biped_model_form_is_power_armor(Ptr) -> bool),
        entry!(0x0048_0d30, fn_00480d30(Ptr) -> bool),
        entry!(0x0048_0d50, fn_00480d50(Ptr, bool)),
        entry!(0x0048_0d90, tes_biped_model_form_get_playable(Ptr) -> bool),
        entry!(0x0048_1010, tes_biped_model_form_get_world_tes_model(u32, u32) -> u32),
        entry!(0x0048_1110, tes_biped_model_form_get_world_tes_model_ov2(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod b023_tests {
    use super::*;

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

    /// A form whose components share one vtable: slot 0 is `0x7000_0000`,
    /// slot 4 `0x7000_0004`, slot 0x14 `0x7000_0014`.
    fn form(e: &mut Engine) -> u32 {
        let mut slots = vec![0u32; 6];
        slots[0] = 0x7000_0000;
        slots[1] = 0x7000_0004;
        slots[5] = 0x7000_0014;
        e.put_vtable(0x0600_0000, &slots);
        let form = e.mem.alloc(0xdc);
        for offset in DATA_COMPONENTS {
            e.mem.set_u32(form + offset, 0x0600_0000);
        }
        form
    }

    #[test]
    fn data_components_are_initialized_and_cleared_in_order() {
        let mut e = Engine::new();
        e.register(0x7000_0000, |_, _| Ret::default());
        e.register(0x7000_0004, |_, _| Ret::default());
        let f = form(&mut e);
        e.mem.set_u32(f + 4, 0xffff_ffff);
        e.mem.set_u8(f + 8, 0xff);
        e.mem.set_u8(f + 9, 0xee);
        e.call_log = Some(vec![]);
        e.call(0x0048_04d0, &[f]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = calls_to(&log, 0x7000_0000)
            .iter()
            .map(|w| w[0] - f)
            .collect();
        assert_eq!(order, DATA_COMPONENTS.to_vec());
        assert_eq!(e.mem.u32(f + 4), 0);
        assert_eq!(e.mem.u8(f + 8), 0);
        assert_eq!(e.mem.u8(f + 9), 0xee);
        e.call_log = Some(vec![]);
        e.call(0x0048_0570, &[f]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = calls_to(&log, 0x7000_0004)
            .iter()
            .map(|w| w[0] - f)
            .collect();
        assert_eq!(order, DATA_COMPONENTS.to_vec());
    }

    #[test]
    fn init_item_and_flag_bits() {
        let mut e = Engine::new();
        e.register(0x0048_aa80, |_, _| Ret::default());
        let f = e.mem.alloc(0xdc);
        e.call_log = Some(vec![]);
        e.call(0x0048_0750, &[f, 0x99]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0048_aa80),
            vec![
                vec![f + 0xc, 0x99],
                vec![f + 0x4c, 0x99],
                vec![f + 0x2c, 0x99],
                vec![f + 0x6c, 0x99]
            ]
        );
        for (flags, power, four, playable) in [
            (0u8, false, false, true),
            (0x20, true, false, true),
            (0x04, false, true, true),
            (0x40, false, false, false),
        ] {
            e.mem.set_u8(f + 8, flags);
            assert_eq!(e.call(0x0048_0d10, &[f]).bool(), power);
            assert_eq!(e.call(0x0048_0d30, &[f]).bool(), four);
            assert_eq!(e.call(0x0048_0d90, &[f]).bool(), playable);
        }
        e.mem.set_u8(f + 8, 0x81);
        e.call(0x0048_0d50, &[f, 0x100]);
        assert_eq!(e.mem.u8(f + 8), 0x81);
        e.call(0x0048_0d50, &[f, 1]);
        assert_eq!(e.mem.u8(f + 8), 0x91);
        e.call(0x0048_0d50, &[f, 0]);
        assert_eq!(e.mem.u8(f + 8), 0x81);
    }

    #[test]
    fn world_models_by_sex() {
        let mut e = Engine::new();
        // Model at +0x6c has no name; any other has one.
        e.register_double(MODEL_HAS_NAME, |_, a| ret(u32::from(a[0] & 0xff != 0x6c)));
        let f = 0x0010_0000u32;
        assert_eq!(e.call(0x0048_1110, &[f, 0]).u32(), f + 0x4c);
        assert_eq!(e.call(0x0048_1110, &[f, 1]).u32(), f + 0x4c);
        assert_eq!(e.call(0x0048_1110, &[f, 2]).u32(), f + 0x8c);
        let g = 0x0010_0100u32 - 0x4c + 0x20;
        // A sex-1 model with a name stays.
        assert_eq!(e.call(0x0048_1110, &[g, 1]).u32(), g + 0x6c);

        e.register(RT_DYNAMIC_CAST, |_, a| {
            ret(if a[0] == 0x55 { 0 } else { a[0] })
        });
        e.register(0x0056_7790, |_, a| ret(a[0] + 1));
        e.register(FORM_TYPE_BYTE, |_, a| {
            ret(if a[0] == 0x301 { 0x2a } else { 0x2b })
        });
        e.register(0x005f_0cc0, |_, _| ret(2));
        assert_eq!(e.call(0x0048_1010, &[0x55, 0]).u32(), 0);
        assert_eq!(e.call(0x0048_1010, &[f, 0]).u32(), f + 0x4c);
        // An NPC owner (type 0x2a) gives its sex.
        assert_eq!(e.call(0x0048_1010, &[f, 0x300]).u32(), f + 0x8c);
        // Another owner, or none, gives sex 0.
        assert_eq!(e.call(0x0048_1010, &[f, 0x400]).u32(), f + 0x4c);
        e.register(0x0056_7790, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_1010, &[f, 0x300]).u32(), f + 0x4c);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FORM_TYPE_BYTE).is_empty());
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![f, 0, TYPE_TES_FORM, TYPE_BIPED_MODEL_FORM, 0]]
        );
    }

    #[test]
    fn add_to_biped_and_the_constraint_template() {
        let mut e = Engine::new();
        let f = form(&mut e);
        e.register(0x7000_0014, |_, _| ret(0x0123_0000));
        // Female model (+0x2c) has no name; the template (+0xc4) has one.
        e.register_double(MODEL_HAS_NAME, move |_, a| ret(u32::from(a[0] != f + 0x2c)));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0] + 1));
        for a in [0x004a_bad0, 0x0063_3c90, 0x0040_c110, 0x008b_bc40] {
            e.register(a, |_, _| Ret::default());
        }
        e.map(MODEL_LOADER & !0xfff, 0x1000);
        e.set_global(MODEL_LOADER, 0x0777_0000u32);
        e.register(0x0044_72a0, |e, a| {
            e.mem.set_u32(a[2], 0x0888_0000);
            Ret::default()
        });
        e.register(0x0055_9450, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0043_b230, |_, a| ret(a[0] + 2));
        e.register(0x0043_f010, |_, a| ret(a[0] + 3));
        e.call_log = Some(vec![]);
        e.call(0x0048_0bd0, &[f, 0x0500_0000, 1, 0x42]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![f, 0, TYPE_BIPED_MODEL_FORM, TYPE_TES_FORM, 0]]
        );
        assert_eq!(
            calls_to(&log, 0x004a_bad0),
            vec![vec![0x0500_0000, f + 1, f + 0xc, 0x42]]
        );
        let pointer = calls_to(&log, 0x0063_3c90)[0][0];
        assert_eq!(calls_to(&log, 0x0063_3c90)[0][1], 0);
        assert_eq!(
            calls_to(&log, 0x0044_72a0),
            vec![vec![0x0777_0000, 0x0123_0000, pointer]]
        );
        assert_eq!(
            calls_to(&log, 0x008b_bc40),
            vec![vec![0x0500_0003, 0x0888_0002]]
        );
        assert_eq!(calls_to(&log, 0x0040_c110), vec![vec![pointer]]);

        // Sex 2 is used as given; no model found: nothing applied; no
        // template name: no lookup.
        e.register(0x0044_72a0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_0bd0, &[f, 0x0500_0000, 2, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x004a_bad0)[0][2], f + 0x4c);
        assert!(calls_to(&log, 0x008b_bc40).is_empty());
        assert_eq!(calls_to(&log, 0x0040_c110).len(), 1);
        e.register_double(MODEL_HAS_NAME, move |_, a| ret(u32::from(a[0] != f + 0xc4)));
        e.call_log = Some(vec![]);
        e.call(0x0048_0bd0, &[f, 0x0500_0000, 1, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x004a_bad0)[0][2], f + 0x2c);
        assert!(calls_to(&log, 0x0063_3c90).is_empty());
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(FORM_TYPE_BYTE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_FIRST_NODE_AT_18, |_, a| (a[0] + 0x18).into_ret());
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_NODE_ITEM_SLOT, |_, a| a[0].into_ret());
        e
    }

    fn form_of_type(e: &mut Engine, kind: u32) -> u32 {
        let form = e.mem.alloc(0x80);
        e.mem.set_u32(form + 4, kind);
        form
    }

    #[test]
    fn at_00480db0_answers_the_part_each_type_selects() {
        let mut e = rig();
        for (kind, offset) in [(0x18u32, 0x70u32), (0x1a, 0x68), (0x60, 0x70)] {
            let form = form_of_type(&mut e, kind);
            assert_eq!(
                e.call(0x0048_0db0, &args![form]).u32(),
                form + offset,
                "{kind:#x}"
            );
        }
        for kind in [0u32, 0x17, 0x19, 0x61, 0xff] {
            let form = form_of_type(&mut e, kind);
            assert_eq!(e.call(0x0048_0db0, &args![form]).u32(), 0, "{kind:#x}");
        }
        assert_eq!(e.call(0x0048_0db0, &args![0u32]).u32(), 0);
        assert_eq!(calls(&e, FORM_TYPE_BYTE).len(), 8);
    }

    #[test]
    fn at_00480af0_tests_the_slot_bit() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0b1010);
        assert!(e.call(0x0048_0af0, &args![this, 1u32, 0u8, 0u32]).bool());
        assert!(!e.call(0x0048_0af0, &args![this, 2u32, 0u8, 0u32]).bool());
        assert!(e.call(0x0048_0af0, &args![this, 3u32, 0u8, 0u32]).bool());
        // The shift count is taken modulo 32.
        assert!(e.call(0x0048_0af0, &args![this, 33u32, 0u8, 0u32]).bool());
        assert!(!e.call(0x0048_0af0, &args![this, 34u32, 0u8, 0u32]).bool());
    }

    #[test]
    fn at_00480af0_the_flag_with_slot_1_and_bit_0_means_false() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0b11);
        assert!(!e.call(0x0048_0af0, &args![this, 1u32, 1u8, 0u32]).bool());
        // Without the flag the slot-1 bit is true as usual.
        assert!(e.call(0x0048_0af0, &args![this, 1u32, 0u8, 0u32]).bool());
        // Another slot is unaffected by the flag.
        assert!(e.call(0x0048_0af0, &args![this, 0u32, 1u8, 0u32]).bool());
        // Slot 1 with bit 0 clear: the slot bit decides.
        e.mem.set_u32(this + 4, 0b10);
        assert!(e.call(0x0048_0af0, &args![this, 1u32, 1u8, 0u32]).bool());
    }

    /// A container whose list head sits at `container + 4` (`00726070`), its nodes from
    /// `head + 0x18`: (item, next).
    fn container_of(e: &mut Engine, forms: &[u32]) -> u32 {
        let container = e.mem.alloc(0x10);
        let head = e.mem.alloc(0x40);
        e.mem.set_u32(container + 4, head);
        let mut next = 0;
        let mut nodes = Vec::new();
        for form in forms.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *form);
            e.mem.set_u32(node + 4, next);
            next = node;
            nodes.push(node);
        }
        // The first node is the head's list head: `00500940` answers `head + 0x18`, so the
        // first node is stored there.
        let first = nodes.last().copied().unwrap_or(0);
        if first != 0 {
            e.mem.set_u32(head + 0x18, e.mem.u32(first));
            e.mem.set_u32(head + 0x1c, e.mem.u32(first + 4));
        }
        container
    }

    #[test]
    fn at_00480af0_asks_the_container_s_forms_when_the_slot_is_not_set_here() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0);
        let unrelated = form_of_type(&mut e, 0x19);
        let biped_form = form_of_type(&mut e, 0x18);
        // The biped part of the form (at +0x70) has the slot bit set.
        e.mem.set_u32(biped_form + 0x70 + 4, 0b100);
        let container = container_of(&mut e, &[unrelated, biped_form]);
        assert!(e
            .call(0x0048_0af0, &args![this, 2u32, 0u8, container])
            .bool());
        let types = calls(&e, FORM_TYPE_BYTE);
        assert_eq!(types, vec![vec![unrelated], vec![biped_form]]);
    }

    #[test]
    fn at_00480af0_answers_false_when_no_form_of_the_container_fills_the_slot() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        let biped_form = form_of_type(&mut e, 0x18);
        let container = container_of(&mut e, &[biped_form]);
        assert!(!e
            .call(0x0048_0af0, &args![this, 2u32, 0u8, container])
            .bool());
        // An empty container (no list head) ends the search at once.
        let bare = e.mem.alloc(0x10);
        assert!(!e.call(0x0048_0af0, &args![this, 2u32, 0u8, bare]).bool());
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn get_biped_tes_model_falls_back_to_the_male_model() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        // The female model (+0x2C) has a name only when its word +4 is set.
        e.register(MODEL_HAS_NAME, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let at = |e: &mut Engine, sex: u32| e.call(0x0048_1150, &args![this, sex]).u32();
        assert_eq!(at(&mut e, 0), this + 0xc);
        assert_eq!(at(&mut e, 1), this + 0xc);
        e.mem.set_u32(this + 0x2c + 4, 1);
        assert_eq!(at(&mut e, 1), this + 0x2c);
        // Other indices are not checked: 2 without a name stays 2.
        assert_eq!(at(&mut e, 2), this + 0x4c);
    }
}

#[cfg(test)]
mod n085_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    /// A biped model form whose components share one vtable: slot +8 and
    /// +0xC log, +0x14 answers the component's address plus 1.
    fn form(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0xd0);
        let vtable = e.mem.alloc(0x18);
        e.mem.set_u32(vtable + 8, 0x7800_0008);
        e.mem.set_u32(vtable + 0xc, 0x7800_000c);
        e.mem.set_u32(vtable + 0x14, 0x7800_0014);
        for offset in COPY_COMPONENTS {
            e.mem.set_u32(this + offset, vtable);
        }
        quiet(e, &[0x7800_0008]);
        e.register(0x7800_000c, |_, _| Ret::default());
        e.register(0x7800_0014, |_, a| (a[0] + 1).into_ret());
        this
    }

    #[test]
    fn save_writes_every_chunk_in_order() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xd0);
        e.register(SAVE_SWAPS_ENDIAN, |_, _| 1u32.into_ret());
        quiet(
            &mut e,
            &[
                SWAP_BIPED_DATA,
                ADD_CHUNK_DATA,
                TEXTURE_SWAP_SAVE,
                MODEL_SAVE_FLAGS,
                TEXTURE_SAVE,
                MESSAGE_ICON_SAVE,
                MODEL_SAVE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0048_0610, &args![this]);
        assert_eq!(calls(&e, SWAP_BIPED_DATA), vec![vec![this + 4]; 2]);
        assert_eq!(
            calls(&e, ADD_CHUNK_DATA),
            vec![vec![0x5444_4d42, this + 4, 8]]
        );
        let swaps = calls(&e, TEXTURE_SWAP_SAVE);
        assert_eq!(
            swaps.iter().map(|a| a[0] - this).collect::<Vec<_>>(),
            vec![0x0c, 0x4c, 0x2c, 0x6c]
        );
        assert_eq!(swaps[2][1..], [0x3344_4f4d, 0x5433_4f4d, 0x5333_4f4d]);
        assert_eq!(
            calls(&e, MODEL_SAVE_FLAGS),
            vec![
                vec![this + 0x0c, 0x4444_4f4d],
                vec![this + 0x2c, 0x4453_4f4d]
            ]
        );
        assert_eq!(
            calls(&e, TEXTURE_SAVE),
            vec![
                vec![this + 0x8c, 0x4e4f_4349],
                vec![this + 0x98, 0x324f_4349]
            ]
        );
        assert_eq!(
            calls(&e, MESSAGE_ICON_SAVE),
            vec![
                vec![this + 0xa4, 0x4f43_494d],
                vec![this + 0xb4, 0x3243_494d]
            ]
        );
        assert_eq!(
            calls(&e, MODEL_SAVE),
            vec![vec![this + 0xc4, 0x5443_4d42, 0]]
        );
    }

    #[test]
    fn copy_and_compare_visit_every_component() {
        let mut e = Engine::new();
        let this = form(&mut e);
        let other = 0x9000u32;
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        quiet(&mut e, &[MEMCPY]);
        e.register(MEMCMP, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_07a0, &args![this, other]);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![
                other,
                0,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_BIPED_MODEL_FORM,
                0
            ]]
        );
        assert_eq!(calls(&e, MEMCPY), vec![vec![this + 4, other + 4, 8]]);
        let copies = calls(&e, 0x7800_0008);
        assert_eq!(copies.len(), 9);
        assert_eq!(copies[3], vec![this + 0xa4, other + 0xa4]);
        // All the same: not different.
        assert!(!e.call(0x0048_08f0, &args![this, other]).bool());
        assert_eq!(calls(&e, 0x7800_000c).len(), 9);
        // The fourth component differs: the rest are not asked.
        e.register_double(0x7800_000c, |_, a| {
            u32::from(a[1] == 0x9000 + 0xa4).into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_08f0, &args![this, other]).bool());
        assert_eq!(calls(&e, 0x7800_000c).len(), 4);
        // Different data, or no biped model form: different.
        e.register(MEMCMP, |_, _| 1u32.into_ret());
        assert!(e.call(0x0048_08f0, &args![this, other]).bool());
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        assert!(e.call(0x0048_08f0, &args![this, other]).bool());
        e.call_log = Some(vec![]);
        e.call(0x0048_07a0, &args![this, other]);
        assert!(calls(&e, MEMCPY).is_empty());
    }

    #[test]
    fn shared_slots_test_the_slot_words() {
        let mut e = Engine::new();
        let (a, b) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(a + 4, 0b0110);
        e.mem.set_u32(b + 4, 0b1001);
        assert!(!e.call(0x0048_0ad0, &args![a, b]).bool());
        e.mem.set_u32(b + 4, 0b0100);
        assert!(e.call(0x0048_0ad0, &args![a, b]).bool());
    }

    /// Doubles for the getters: the cast answers its first word; the owner
    /// of reference 0x5000 is 0x5100 (form type 0x2A, sex 1).
    fn getters(e: &mut Engine, has_name: bool) {
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(GET_OWNER, |_, a| {
            (if a[0] == 0x5000 { 0x5100u32 } else { 0 }).into_ret()
        });
        e.register(FORM_TYPE_BYTE, |_, _| 0x2au32.into_ret());
        e.register(GET_SEX, |_, _| 1u32.into_ret());
        e.register_double(MODEL_HAS_NAME, move |_, _| u32::from(has_name).into_ret());
        e.register(0x0040_8da0, |_, a| (a[0] | 0x8000_0000).into_ret());
        e.register(0x0048_1230, |_, a| (a[0] + a[1]).into_ret());
        e.register(0x0047_a300, |_, a| (a[0] | 0x4000_0000).into_ret());
        e.register(0x0048_1280, |_, a| (a[0] + 2 * a[1]).into_ret());
    }

    #[test]
    fn getters_pick_the_component_by_the_owner_sex() {
        let mut e = Engine::new();
        let this = form(&mut e);
        getters(&mut e, true);
        // Icons: worldModel[0] +0x14 without a reference, [1] for sex 1.
        assert_eq!(
            e.call(0x0048_0e60, &args![this, 0u32]).u32(),
            this + 0x4c + 1
        );
        assert_eq!(
            e.call(0x0048_0e60, &args![this, 0x5000u32]).u32(),
            this + 0x6c + 1
        );
        // An owner of another form type: sex 0.
        e.register(FORM_TYPE_BYTE, |_, _| 0x12au32.into_ret());
        assert_eq!(
            e.call(0x0048_0e60, &args![this, 0x5000u32]).u32(),
            this + 0x4c + 1
        );
        e.register(FORM_TYPE_BYTE, |_, _| 0x2au32.into_ret());
        assert_eq!(
            e.call(0x0048_0ef0, &args![this, 0u32]).u32(),
            (this + 0x8c) | 0x8000_0000
        );
        assert_eq!(e.call(0x0048_0ef0, &args![this, 0x5000u32]).u32(), this + 1);
        assert_eq!(
            e.call(0x0048_0f80, &args![this, 0u32]).u32(),
            (this + 0xa4) | 0x4000_0000
        );
        assert_eq!(e.call(0x0048_0f80, &args![this, 0x5000u32]).u32(), this + 2);
        assert_eq!(e.call(0x0048_1090, &args![this, 0u32]).u32(), this + 0xc);
        assert_eq!(
            e.call(0x0048_1090, &args![this, 0x5000u32]).u32(),
            this + 0x2c
        );
        // No biped model form: 0 for all.
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        for addr in [0x0048_0e60u32, 0x0048_0ef0, 0x0048_0f80, 0x0048_1090] {
            assert_eq!(e.call(addr, &args![this, 0x5000u32]).u32(), 0);
        }
    }

    #[test]
    fn sexed_models_fall_back_to_the_first_without_a_name() {
        for (has_name, offset) in [(true, 0x20u32), (false, 0)] {
            let mut e = Engine::new();
            let this = form(&mut e);
            getters(&mut e, has_name);
            assert_eq!(
                e.call(0x0048_1190, &args![this, 1u32]).u32(),
                this + 0x4c + offset + 1
            );
            assert_eq!(
                e.call(0x0048_11e0, &args![this, 1u32]).u32(),
                this + 0xc + offset + 1
            );
            // Sex 0 stays 0.
            assert_eq!(
                e.call(0x0048_11e0, &args![this, 0u32]).u32(),
                this + 0xc + 1
            );
        }
    }
}
