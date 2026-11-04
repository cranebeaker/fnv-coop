//! `fallout shared/teslandtexture.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00540c50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00540c50 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESLandTexture::TESLandTexture` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x12 (0x28 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn teslandtexture_teslandtexture(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0102_e6c4);
    e.call(0x0096_a2d0, &args![base + 0x20]);
    e.call(0x004f_15a0, &args![this, 0x12]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    e.call(0x0054_0d80, &args![this]);
    this
}

// Translated from 00540ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00540d10` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00540ce0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0054_0d10, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

layout! {
    /// `TESLandTexture` (Xbox PDB, 0x38 bytes there): 0x28 bytes on PC
    /// (`CreateFormOfType` allocates that much), `TESForm` being 0x18.
    pub struct TESLandTexture: 0x28 {
        /// `pTextureSet` (Xbox PDB +0x28): `BGSTextureSet*`.
        0x18 pTextureSet: u32,
        /// `HavokData.cMaterialType` (Xbox PDB +0x2C).
        0x1C cMaterialType: u8,
        /// `HavokData.cFriction` (Xbox PDB +0x2D).
        0x1D cFriction: u8,
        /// `HavokData.cRestitution` (Xbox PDB +0x2E).
        0x1E cRestitution: u8,
        /// `cSpecularExponent` (Xbox PDB +0x2F).
        0x1F cSpecularExponent: u8,
        /// `TextureGrassList` (Xbox PDB +0x30): `BSSimpleList<TESGrass *>`.
        0x20 TextureGrassList: Inline<crate::types::BSSimpleList>,
    }
}

/// `TESLandTexture`'s vtable (stored by the constructor and the
/// destructor).
const LAND_TEXTURE_VTABLE: u32 = 0x0102_e6c4;
/// The list at `this + 0x20` (`00891170`, plain `RET`; the engine map
/// places it in `actor.cpp`, a folded body).
const GRASS_LIST: u32 = 0x0089_1170;
/// Empties a list (`00470470`, plain `RET`).
const LIST_REMOVE_ALL: u32 = 0x0047_0470;
/// A `BSSimpleList` node's empty test (`008256d0`, `bool`).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// The address of a node's item (`006815c0`).
const NODE_ITEM: u32 = 0x0068_15c0;
/// A node's next node (`00726070`).
const NODE_NEXT: u32 = 0x0072_6070;
/// Appends the item a pointer points at to a list (`00905820`, `RET 4`).
const LIST_ADD_TAIL: u32 = 0x0090_5820;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors: `.?AVTESForm@@`, the grass list's target type
/// (`0118a260`) and the texture set's (`01187788`), as the code passes them.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_GRASS: u32 = 0x0118_a260;
const TYPE_TEXTURE_SET: u32 = 0x0118_7788;
/// `TESForm`'s form id getter (`0084e3a0`).
const FORM_ID: u32 = 0x0084_e3a0;
/// Tests form flag 0x8 (`004013e0`).
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// Sets (`true`) or clears form flag 0x8 (`00484ab0`, `RET 4`).
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// `TESForm::GetFile` (Xbox PDB, `00484e60`, `RET 4`).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`, cdecl, two words).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// Looks up a form by id (`004839c0`, cdecl, one word).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// The warning printer (`005b5e40`, cdecl, variable arguments).
const WARNING: u32 = 0x005b_5e40;
/// "MASTERFILE: Could not find Grass (%08X) for LandTexture '%s' (%08X)."
const MISSING_GRASS_MESSAGE: u32 = 0x0102_e910;
/// "MASTERFILE: Could not find Texture Set (%08X) for LandTexture '%s' (%08X)."
const MISSING_TEXTURE_SET_MESSAGE: u32 = 0x0102_e8c0;
/// "MASTERFILE: TextureSet '%s' (%08X) missing diffuse texture (required
/// for landscape textures)."
const MISSING_DIFFUSE_MESSAGE: u32 = 0x0102_e860;
/// "MASTERFILE: TextureSet '%s' (%08X) missing normal texture (required for
/// landscape textures)."
const MISSING_NORMAL_MESSAGE: u32 = 0x0102_e800;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, `RET 4`).
const LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESForm` (Xbox PDB, `00472660`): the current record's form
/// type.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The size of the current chunk (`00401660`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`): buffer and size.
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (Xbox PDB, `004727f0`): one word, the buffer.
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// Moves to the next chunk (`004726f0`, `bool`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `BGSTextureSet::QueueTextureSet` (Xbox PDB, `005930e0`, `RET 0xC`).
const QUEUE_TEXTURE_SET: u32 = 0x0059_30e0;

/// Chunk signatures, as the words the exe compares.
const SIG_EDID: u32 = 0x4449_4445;
const SIG_OBND: u32 = 0x444e_424f;
const SIG_GNAM: u32 = 0x4d41_4e47;
const SIG_HNAM: u32 = 0x4d41_4e48;
const SIG_SNAM: u32 = 0x4d41_4e53;
const SIG_TNAM: u32 = 0x4d41_4e54;

// Translated from 00540d10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00540d10 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESLandTexture`'s destructor body (called by `00540ce0`): stores the
/// vtable `0102e6c4`, empties the grass list (`00540dc0`), destroys the list
/// at `+0x20` (`0046ffb0`) and runs `TESForm::~TESForm` (Xbox PDB,
/// `00483630`).
pub fn fn_00540d10(e: &mut Engine, this: Ptr<TESLandTexture>) {
    e.mem.set_u32(this.addr(), LAND_TEXTURE_VTABLE);
    fn_00540dc0(e, this);
    e.call(0x0046_ffb0, &args![this.addr() + 0x20]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 00540d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start values of the constructor (`00540c50` calls it last): no
/// texture set, material type 2, friction, restitution and specular
/// exponent 30.
pub fn fn_00540d80(e: &mut Engine, this: Ptr<TESLandTexture>) {
    e.set(this, TESLandTexture::pTextureSet, 0);
    e.set(this, TESLandTexture::cMaterialType, 2);
    e.set(this, TESLandTexture::cFriction, 0x1e);
    e.set(this, TESLandTexture::cRestitution, 0x1e);
    e.set(this, TESLandTexture::cSpecularExponent, 0x1e);
}

// Translated from 00540dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the grass list: `00470470` on the list `00891170` gives.
pub fn fn_00540dc0(e: &mut Engine, this: Ptr<TESLandTexture>) {
    let list = e.call(GRASS_LIST, &args![this]).u32();
    e.call(LIST_REMOVE_ALL, &args![list]);
}

/// A form id in the slot at `id_at`, given the compile index of `this`'s
/// file and looked up, cast from `TESForm` to `target` (`__RTDynamicCast`).
fn resolve(e: &mut Engine, this: Ptr<TESLandTexture>, id_at: Ptr, target: u32) -> u32 {
    let file = e.call(GET_FILE, &args![this, u32::MAX]).u32();
    e.call(ADD_COMPILE_INDEX, &args![id_at, file]);
    let id = e.mem.u32(id_at.addr());
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
    )
    .u32()
}

// Translated from 00540fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESLandTexture::InitItem` (Xbox PDB): unless form flag 0x8 is set
/// (`004013e0`), resolves every grass of the grass list, which `Load` left
/// as form ids (`resolve`, cast to `0118a260`): a found one replaces the
/// item (`00726c60` with the address of the cast result); a missing one is
/// reported ("Could not find Grass", with the id, vtable slot 0x130's answer
/// and the form id) and its node removed: after an earlier node through
/// `00905330` on the earlier node (with the address of a copy of the id),
/// the walk going on from the earlier node's next; for the first node
/// through `0063f7b0` on the node, which is then walked again.
///
/// Then the texture set id at `+0x18`, when not zero, is resolved the same
/// way (cast to `01187788`) and stored back; missing, it is reported. A
/// texture set found has its component at `+0x30` asked through its vtable
/// slot 0x8C for texture 0 (diffuse) and 1 (normal), each missing one
/// reported with the set's slot 0x130 answer and form id, and is then
/// given to `00541250` with 0. Finally form flag 0x8 is set.
pub fn tes_land_texture_init_item(e: &mut Engine, this: Ptr<TESLandTexture>) {
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    e.with_stack(0x10, |e, locals| {
        // `[EBP-0x10]` (the id), `[EBP-0xC]` (the cast result), `[EBP-0x18]`
        // (the id's copy), `[EBP-0x14]` (the texture set id).
        let id_at = locals;
        let form_at = locals.byte_add(4);
        let copy_at = locals.byte_add(8);
        let set_id_at = locals.byte_add(12);
        let mut node = e.call(GRASS_LIST, &args![this]).u32();
        let mut previous = 0u32;
        while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
            let item_at = e.call(NODE_ITEM, &args![node]).u32();
            let item = e.mem.u32(item_at);
            e.mem.set_u32(id_at.addr(), item);
            let grass = resolve(e, this, id_at, TYPE_GRASS);
            e.mem.set_u32(form_at.addr(), grass);
            if grass == 0 {
                let form_id = e.call(FORM_ID, &args![this]).u32();
                let name = e.vcall(this.addr(), 0x130, &args![]).u32();
                let id = e.mem.u32(id_at.addr());
                e.call(WARNING, &args![MISSING_GRASS_MESSAGE, id, name, form_id]);
                if previous != 0 {
                    let id = e.mem.u32(id_at.addr());
                    e.mem.set_u32(copy_at.addr(), id);
                    e.call(0x0090_5330, &args![previous, copy_at]);
                    node = e.call(NODE_NEXT, &args![previous]).u32();
                } else {
                    e.call(0x0063_f7b0, &args![node]);
                }
            } else {
                e.call(0x0072_6c60, &args![node, form_at]);
                previous = node;
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
        }
        if e.get(this, TESLandTexture::pTextureSet) != 0 {
            let set_id = e.get(this, TESLandTexture::pTextureSet);
            e.mem.set_u32(set_id_at.addr(), set_id);
            let set = resolve(e, this, set_id_at, TYPE_TEXTURE_SET);
            e.set(this, TESLandTexture::pTextureSet, set);
            if e.get(this, TESLandTexture::pTextureSet) == 0 {
                let form_id = e.call(FORM_ID, &args![this]).u32();
                let name = e.vcall(this.addr(), 0x130, &args![]).u32();
                let id = e.mem.u32(set_id_at.addr());
                e.call(
                    WARNING,
                    &args![MISSING_TEXTURE_SET_MESSAGE, id, name, form_id],
                );
            }
            if e.get(this, TESLandTexture::pTextureSet) != 0 {
                for (texture, message) in
                    [(0u32, MISSING_DIFFUSE_MESSAGE), (1, MISSING_NORMAL_MESSAGE)]
                {
                    let set = e.get(this, TESLandTexture::pTextureSet);
                    let found = e.vcall(set + 0x30, 0x8c, &args![texture]).u32();
                    if found == 0 {
                        let set = e.get(this, TESLandTexture::pTextureSet);
                        let form_id = e.call(FORM_ID, &args![set]).u32();
                        let set = e.get(this, TESLandTexture::pTextureSet);
                        let name = e.vcall(set, 0x130, &args![]).u32();
                        e.call(WARNING, &args![message, name, form_id]);
                    }
                }
                let set = e.get(this, TESLandTexture::pTextureSet);
                fn_00541250(e, Ptr::new(set), false);
            }
        }
    });
    e.call(SET_FORM_FLAG_8, &args![this, true]);
}

// Translated from 00541250 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `teslandtexture.cpp`; `InitItem` calls
/// it on the texture set). Clears bit 0 of the word at `this + 0x84` when
/// `clear` is true, sets it otherwise; `RET 4`.
pub fn fn_00541250(e: &mut Engine, this: Ptr, clear: bool) {
    let flags = e.mem.u16(this.addr() + 0x84);
    let flags = if clear { flags & 0xfffe } else { flags | 1 };
    e.mem.set_u16(this.addr() + 0x84, flags);
}

// Translated from 005412a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005412a0 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// `TESLandTexture`'s `Load`: false unless the record is of form type 0x12
/// (`TESFile::GetTESForm`, `00472660`). Then `TESForm::LoadForm` (Xbox PDB,
/// `00485110`) and every chunk of `file`, without checking for a missing
/// chunk:
/// - `HNAM`: 3 bytes into `HavokData` (`+0x1C`);
/// - `EDID`: into a stack buffer of the chunk's size (`00401660`), read with
///   at most 0x200 bytes, given to vtable slot 0x134;
/// - `OBND`: vtable slot 0xE0 with the file;
/// - `GNAM`: a word (cleared first); when not zero, a copy of it is
///   appended to the grass list (`00905820` on the list `00891170` gives);
/// - `SNAM`: 1 byte into `cSpecularExponent` (`+0x1F`);
/// - `TNAM`: a word (cleared first) stored in `pTextureSet`.
///
/// After the last chunk form flag 0x8 is cleared (`00484ab0`); returns true.
pub fn fn_005412a0(e: &mut Engine, this: Ptr<TESLandTexture>, file: Ptr) -> bool {
    let form_type = e.call(GET_TES_FORM, &args![file]).u8();
    if form_type != 0x12 {
        return false;
    }
    e.call(LOAD_FORM, &args![this, file]);
    e.with_stack(12, |e, locals| {
        // `[EBP-0xC]`, `[EBP-0x10]` and `[EBP-0x18]`.
        let texture_set_at = locals;
        let grass_at = locals.byte_add(4);
        let copy_at = locals.byte_add(8);
        loop {
            let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
            match chunk {
                SIG_HNAM => {
                    let havok = this.addr() + 0x1c;
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, havok, 3u32]);
                }
                SIG_EDID => {
                    let size = e.call(CHUNK_SIZE, &args![file]).u32();
                    e.with_stack(size, |e, buffer| {
                        e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, 0x200u32]);
                        e.vcall(this.addr(), 0x134, &args![buffer]);
                    });
                }
                SIG_OBND => {
                    e.vcall(this.addr(), 0xe0, &args![file]);
                }
                SIG_GNAM => {
                    e.mem.set_u32(grass_at.addr(), 0);
                    e.call(GET_CHUNK_DATA, &args![file, grass_at]);
                    let grass = e.mem.u32(grass_at.addr());
                    if grass != 0 {
                        e.mem.set_u32(copy_at.addr(), grass);
                        let list = e.call(GRASS_LIST, &args![this]).u32();
                        e.call(LIST_ADD_TAIL, &args![list, copy_at]);
                    }
                }
                SIG_SNAM => {
                    let exponent = this.addr() + 0x1f;
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, exponent, 1u32]);
                }
                SIG_TNAM => {
                    e.mem.set_u32(texture_set_at.addr(), 0);
                    e.call(GET_CHUNK_DATA, &args![file, texture_set_at]);
                    let set = e.mem.u32(texture_set_at.addr());
                    e.set(this, TESLandTexture::pTextureSet, set);
                }
                _ => {}
            }
            if !e.call(NEXT_CHUNK, &args![file]).bool() {
                break;
            }
        }
    });
    e.call(SET_FORM_FLAG_8, &args![this, false]);
    true
}

// Translated from 00541540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESLandTexture::QueueTexture` (Xbox PDB): with a texture set, queues it
/// (`BGSTextureSet::QueueTextureSet`, `005930e0`, with the two words and
/// 1); `RET 8`.
pub fn tes_land_texture_queue_texture(
    e: &mut Engine,
    this: Ptr<TESLandTexture>,
    first: u32,
    second: u32,
) {
    let set = e.get(this, TESLandTexture::pTextureSet);
    if set != 0 {
        e.call(QUEUE_TEXTURE_SET, &args![set, first, second, 1u32]);
    }
}

/// The `TESLandTexture` type descriptor (`01186350`, its name read at +8).
const TYPE_LAND_TEXTURE: u32 = 0x0118_6350;
/// `TESForm::CopyAllComponents` (`004851b0`), `TESForm::CompareAllComponents`
/// (`00485270`), the copy `00401460` (`cdecl (to, from, size)`), `memcmp`
/// (`00ec4835`) and the texture set getter `009611e0` (the word at +0x18).
const FORM_COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const FORM_COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
const COPY_BYTES: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;
const TEXTURE_SET_GETTER: u32 = 0x0096_11e0;
/// `TESForm::StartForm` (`004855a0`), `TESForm::CloseForm` (`00485680`),
/// the empty `00483710` (`thiscall`, runs on the form and, when saving
/// swaps bytes, on the Havok data), `TESForm::AddChunk` with a word
/// (`00485910`, `cdecl`) and with a byte (`004858f0`, `cdecl`),
/// `TESForm::__AddChunkData` (`00485990`, `cdecl`) and the byte
/// `00401500` answers (whether saving swaps bytes).
const FORM_START: u32 = 0x0048_55a0;
const FORM_CLOSE: u32 = 0x0048_5680;
const EMPTY_00483710: u32 = 0x0048_3710;
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
const ADD_CHUNK_BYTE: u32 = 0x0048_58f0;
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
const SAVE_SWAPS: u32 = 0x0040_1500;

/// `other` cast from `TESForm` to `TESLandTexture`.
fn cast_land_texture(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![other, 0u32, TYPE_TES_FORM, TYPE_LAND_TEXTURE, 0u32],
    )
    .u32()
}

// Translated from 00540de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; the copy of a
/// `TESLandTexture`): when `other` is one, empties the grass list, copies
/// all components, the three Havok bytes (+0x1C), the specular exponent
/// (+0x1F) and the texture set (`009611e0`), then appends each grass of
/// `other`'s list (from its first node, while a node is not empty) to the
/// list at +0x20.
pub fn fn_00540de0(e: &mut Engine, this: Ptr<TESLandTexture>, other: Ptr) {
    let source = cast_land_texture(e, other);
    if source == 0 {
        return;
    }
    let at = this.addr();
    let list = e.call(GRASS_LIST, &args![this]).u32();
    e.call(LIST_REMOVE_ALL, &args![list]);
    e.call(FORM_COPY_ALL_COMPONENTS, &args![this, source]);
    e.call(COPY_BYTES, &args![at + 0x1c, source + 0x1c, 3u32]);
    let exponent = e.mem.u8(source + 0x1f);
    e.mem.set_u8(at + 0x1f, exponent);
    let set = e.call(TEXTURE_SET_GETTER, &args![source]).u32();
    e.set(this, TESLandTexture::pTextureSet, set);
    let mut node = e.call(GRASS_LIST, &args![source]).u32();
    while node != 0 {
        if e.call(NODE_IS_EMPTY, &args![node]).u8() != 0 {
            break;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let grass = e.mem.u32(item);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), grass);
            e.call(LIST_ADD_TAIL, &args![at + 0x20, slot]);
        });
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 00540ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; the comparison of a
/// `TESLandTexture`): true (different) when `other` is none, when
/// `CompareAllComponents` says so, when the three Havok bytes, the
/// specular exponent or the texture set (`009611e0`) differ, when the two
/// grass lists hold a different grass at the same place, or when one list
/// is longer (walked by node, empty nodes included).
pub fn fn_00540ec0(e: &mut Engine, this: Ptr<TESLandTexture>, other: Ptr) -> bool {
    let source = cast_land_texture(e, other);
    if source == 0 {
        return true;
    }
    let at = this.addr();
    if e.call(FORM_COMPARE_ALL_COMPONENTS, &args![this, source])
        .u8()
        != 0
    {
        return true;
    }
    if e.call(MEMCMP, &args![at + 0x1c, source + 0x1c, 3u32]).u32() != 0 {
        return true;
    }
    if e.mem.u8(at + 0x1f) != e.mem.u8(source + 0x1f) {
        return true;
    }
    let set = e.call(TEXTURE_SET_GETTER, &args![source]).u32();
    if e.get(this, TESLandTexture::pTextureSet) != set {
        return true;
    }
    let mut mine = e.call(GRASS_LIST, &args![this]).u32();
    let mut theirs = e.call(GRASS_LIST, &args![source]).u32();
    while mine != 0 && theirs != 0 {
        let my_item = e.call(NODE_ITEM, &args![mine]).u32();
        let their_item = e.call(NODE_ITEM, &args![theirs]).u32();
        if e.mem.u32(my_item) != e.mem.u32(their_item) {
            return true;
        }
        theirs = e.call(NODE_NEXT, &args![theirs]).u32();
        mine = e.call(NODE_NEXT, &args![mine]).u32();
    }
    (mine == 0 && theirs != 0) || (theirs == 0 && mine != 0)
}

// Translated from 00541440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESLandTexture::Save` (Xbox PDB): starts the form, runs the empty
/// `00483710` on it, with a texture set adds its form id as `TNAM`, adds
/// the three Havok bytes (+0x1C) as `HNAM` (the empty `00483710` run on
/// them before and after when saving swaps bytes), the specular exponent
/// as `SNAM` and each grass's form id as `GNAM` (from the first node,
/// while a node is not empty); closes the form.
pub fn tes_land_texture_save(e: &mut Engine, this: Ptr<TESLandTexture>) {
    let at = this.addr();
    e.call(FORM_START, &args![this]);
    e.call(EMPTY_00483710, &args![this]);
    let set = e.get(this, TESLandTexture::pTextureSet);
    if set != 0 {
        let id = e.call(FORM_ID, &args![set]).u32();
        e.call(ADD_CHUNK_WORD, &args![0x4d41_4e54u32, id]);
    }
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        e.call(EMPTY_00483710, &args![at + 0x1c]);
    }
    e.call(ADD_CHUNK_DATA, &args![0x4d41_4e48u32, at + 0x1c, 3u32]);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        e.call(EMPTY_00483710, &args![at + 0x1c]);
    }
    let exponent = u32::from(e.mem.u8(at + 0x1f));
    e.call(ADD_CHUNK_BYTE, &args![0x4d41_4e53u32, exponent]);
    let mut node = e.call(GRASS_LIST, &args![this]).u32();
    while node != 0 {
        if e.call(NODE_IS_EMPTY, &args![node]).u8() != 0 {
            break;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let grass = e.mem.u32(item);
        let id = e.call(FORM_ID, &args![grass]).u32();
        e.call(ADD_CHUNK_WORD, &args![0x4d41_4e47u32, id]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(FORM_CLOSE, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0054_0c50, teslandtexture_teslandtexture(Ptr) -> Ptr),
        entry!(0x0054_0ce0, fn_00540ce0(Ptr, u32) -> Ptr),
        entry!(0x0054_0d10, fn_00540d10(Ptr<TESLandTexture>)),
        entry!(0x0054_0d80, fn_00540d80(Ptr<TESLandTexture>)),
        entry!(0x0054_0dc0, fn_00540dc0(Ptr<TESLandTexture>)),
        entry!(0x0054_0fe0, tes_land_texture_init_item(Ptr<TESLandTexture>)),
        entry!(0x0054_1250, fn_00541250(Ptr, bool)),
        entry!(0x0054_12a0, fn_005412a0(Ptr<TESLandTexture>, Ptr) -> bool),
        entry!(
            0x0054_1540,
            tes_land_texture_queue_texture(Ptr<TESLandTexture>, u32, u32)
        ),
        entry!(0x0054_0de0, fn_00540de0(Ptr<TESLandTexture>, Ptr)),
        entry!(0x0054_0ec0, fn_00540ec0(Ptr<TESLandTexture>, Ptr) -> bool),
        entry!(0x0054_1440, tes_land_texture_save(Ptr<TESLandTexture>)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn teslandtexture_teslandtexture_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0054_0c50,
            size: 0x28,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0096_a2d0, &[A::T(0x20)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x12)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
                (0x0054_0d80, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_e6c4)],
        });
    }
    #[test]
    fn at_00540ce0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0054_0d10, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0054_0ce0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0054_0ce0)
                .collect();
            let mut expected = vec![(0x0054_0d10, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    mod b023 {
        use super::super::*;
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
        fn destructor_body_restores_the_vtable_and_tears_down() {
            let mut e = Engine::new();
            quiet(&mut e, &[LIST_REMOVE_ALL, 0x0046_ffb0, 0x0048_3630]);
            e.register(GRASS_LIST, |_, a| ret(a[0] + 0x20));
            let this: Ptr<TESLandTexture> = e.new_object();
            e.call_log = Some(vec![]);
            e.call(0x0054_0d10, &args![this]);
            let log: Vec<_> = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|(a, _)| *a != 0x0054_0d10)
                .collect();
            let t = this.addr();
            assert_eq!(
                log,
                vec![
                    (GRASS_LIST, vec![t]),
                    (LIST_REMOVE_ALL, vec![t + 0x20]),
                    (0x0046_ffb0, vec![t + 0x20]),
                    (0x0048_3630, vec![t]),
                ]
            );
            assert_eq!(e.mem.u32(t), LAND_TEXTURE_VTABLE);
        }

        #[test]
        fn start_values() {
            let mut e = Engine::new();
            let this: Ptr<TESLandTexture> = e.new_object();
            e.mem.write(this.addr(), &[0xcd; 0x28]);
            e.call(0x0054_0d80, &args![this]);
            assert_eq!(e.mem.u32(this.addr() + 0x18), 0);
            assert_eq!(
                e.mem.bytes(this.addr() + 0x1c, 4),
                vec![2, 0x1e, 0x1e, 0x1e]
            );
            assert_eq!(e.mem.u32(this.addr() + 0x20), 0xcdcd_cdcd);
        }

        #[test]
        fn flag_bit_zero_of_the_word_at_0x84() {
            let mut e = Engine::new();
            let set = e.mem.alloc(0x88);
            e.mem.set_u16(set + 0x84, 0x8001);
            e.call(0x0054_1250, &[set, 0x100]);
            // Only the low byte counts: 0x100 is false and sets the bit.
            assert_eq!(e.mem.u16(set + 0x84), 0x8001);
            e.call(0x0054_1250, &[set, 1]);
            assert_eq!(e.mem.u16(set + 0x84), 0x8000);
            e.call(0x0054_1250, &[set, 0]);
            assert_eq!(e.mem.u16(set + 0x84), 0x8001);
        }

        #[test]
        fn queue_texture_needs_a_texture_set() {
            let mut e = Engine::new();
            quiet(&mut e, &[QUEUE_TEXTURE_SET]);
            let this: Ptr<TESLandTexture> = e.new_object();
            e.call_log = Some(vec![]);
            e.call(0x0054_1540, &args![this, 7u32, 8u32]);
            assert!(calls_to(e.call_log.as_ref().unwrap(), QUEUE_TEXTURE_SET).is_empty());
            e.set(this, TESLandTexture::pTextureSet, 0x0123_4560);
            e.call(0x0054_1540, &args![this, 7u32, 8u32]);
            assert_eq!(
                calls_to(e.call_log.as_ref().unwrap(), QUEUE_TEXTURE_SET),
                vec![vec![0x0123_4560, 7, 8, 1]]
            );
        }

        #[test]
        fn load_refuses_other_form_types_and_reads_chunks() {
            let mut e = Engine::new();
            e.register(GET_TES_FORM, |_, _| ret(0x0113));
            quiet(&mut e, &[LOAD_FORM, SET_FORM_FLAG_8]);
            let this: Ptr<TESLandTexture> = e.new_object();
            let file = 0x0500_0000u32;
            e.call_log = Some(vec![]);
            assert!(!e.call(0x0054_12a0, &[this.addr(), file]).bool());
            assert!(calls_to(e.call_log.as_ref().unwrap(), LOAD_FORM).is_empty());

            e.register(GET_TES_FORM, |_, _| ret(0x12));
            let chunks = Rc::new(RefCell::new(vec![
                SIG_TNAM, SIG_SNAM, SIG_GNAM, SIG_GNAM, 0, SIG_OBND, SIG_EDID, SIG_HNAM,
            ]));
            let c = chunks.clone();
            e.register_double(GET_TES_CHUNK, move |_, _| {
                ret(c.borrow_mut().pop().unwrap())
            });
            let left = chunks.clone();
            e.register_double(NEXT_CHUNK, move |_, _| {
                ret(u32::from(!left.borrow().is_empty()))
            });
            e.register(CHUNK_SIZE, |_, _| ret(5));
            e.register(GET_CHUNK_DATA_SIZED, |_, _| Ret::default());
            let n = Rc::new(RefCell::new(0usize));
            let nn = n.clone();
            e.register_double(GET_CHUNK_DATA, move |e, a| {
                assert_eq!(e.mem.u32(a[1]), 0, "cleared before the read");
                let mut k = nn.borrow_mut();
                // GNAM 0, GNAM 0x22, TNAM 0x33.
                e.mem.set_u32(a[1], [0, 0x22, 0x33][*k]);
                *k += 1;
                Ret::default()
            });
            e.register(GRASS_LIST, |_, a| ret(a[0] + 0x20));
            let added = Rc::new(RefCell::new(vec![]));
            let ad = added.clone();
            e.register_double(LIST_ADD_TAIL, move |e, a| {
                ad.borrow_mut().push((a[0], e.mem.u32(a[1])));
                Ret::default()
            });
            quiet(&mut e, &[0x7000_0000, 0x7000_0004]);
            let mut slots = vec![0u32; 0x138 / 4];
            slots[0x134 / 4] = 0x7000_0000;
            slots[0xe0 / 4] = 0x7000_0004;
            e.put_vtable(0x0600_0000, &slots);
            e.mem.set_u32(this.addr(), 0x0600_0000);
            e.call_log = Some(vec![]);
            assert!(e.call(0x0054_12a0, &[this.addr(), file]).bool());
            let log = e.call_log.take().unwrap();
            let t = this.addr();
            assert_eq!(calls_to(&log, LOAD_FORM), vec![vec![t, file]]);
            let sized = calls_to(&log, GET_CHUNK_DATA_SIZED);
            assert_eq!(sized[0], vec![file, t + 0x1c, 3]);
            assert_eq!(sized[1][2], 0x200);
            assert_eq!(sized[2], vec![file, t + 0x1f, 1]);
            assert_eq!(calls_to(&log, 0x7000_0000).len(), 1);
            assert_eq!(calls_to(&log, 0x7000_0004), vec![vec![t, file]]);
            assert_eq!(*added.borrow(), vec![(t + 0x20, 0x22)]);
            assert_eq!(e.get(this, TESLandTexture::pTextureSet), 0x33);
            assert_eq!(calls_to(&log, SET_FORM_FLAG_8), vec![vec![t, 0]]);
        }

        #[test]
        fn init_item_resolves_grass_and_texture_set() {
            const FOUND: u32 = 0x0700_0000;
            let mut e = Engine::new();
            e.register(FORM_FLAG_8_SET, |_, _| ret(1));
            quiet(&mut e, &[SET_FORM_FLAG_8]);
            let this: Ptr<TESLandTexture> = e.new_object();
            let t = this.addr();
            e.call_log = Some(vec![]);
            e.call(0x0054_0fe0, &args![this]);
            assert!(calls_to(e.call_log.as_ref().unwrap(), SET_FORM_FLAG_8).is_empty());

            // A grass list of three nodes (item, next) with ids 9, 1, 2;
            // 9 and 2 are missing.
            e.register(FORM_FLAG_8_SET, |_, _| ret(0));
            let nodes = e.mem.alloc(0x20);
            e.mem.set_u32(nodes, 9);
            e.mem.set_u32(nodes + 4, nodes + 8);
            e.mem.set_u32(nodes + 8, 1);
            e.mem.set_u32(nodes + 12, nodes + 16);
            e.mem.set_u32(nodes + 16, 2);
            e.mem.set_u32(nodes + 20, 0);
            e.register_double(GRASS_LIST, move |_, _| ret(nodes));
            e.register(NODE_IS_EMPTY, |_, _| ret(0));
            e.register(NODE_ITEM, |_, a| ret(a[0]));
            e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
            e.register(GET_FILE, |_, _| ret(0x0400_0000));
            e.register(ADD_COMPILE_INDEX, |e, a| {
                assert_eq!(a[1], 0x0400_0000);
                let id = e.mem.u32(a[0]);
                e.mem.set_u32(a[0], id + 0x100);
                Ret::default()
            });
            e.register(LOOKUP_FORM, |_, a| {
                ret(match a[0] {
                    0x101 | 0x145 => a[0] + FOUND,
                    _ => 0,
                })
            });
            e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
            e.register(FORM_ID, |_, a| ret(a[0] ^ 0xf000_0000));
            e.register(0x7000_0000, |_, a| ret(a[0] + 1));
            // The first node removed: it takes the second's item and next.
            e.register(0x0063_f7b0, |e, a| {
                let next = e.mem.u32(a[0] + 4);
                let (item, after) = (e.mem.u32(next), e.mem.u32(next + 4));
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
                Ret::default()
            });
            e.register(0x0090_5330, |e, a| {
                let next = e.mem.u32(a[0] + 4);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0] + 4, after);
                Ret::default()
            });
            e.register(0x0072_6c60, |e, a| {
                let form = e.mem.u32(a[1]);
                e.mem.set_u32(a[0], form);
                Ret::default()
            });
            quiet(&mut e, &[WARNING]);
            let mut slots = vec![0u32; 0x134 / 4];
            slots[0x130 / 4] = 0x7000_0000;
            e.put_vtable(0x0600_0000, &slots);
            e.mem.set_u32(t, 0x0600_0000);
            // The texture set id 0x45 resolves to a set whose component at
            // +0x30 has a diffuse texture but no normal one.
            let set = FOUND + 0x145;
            e.mem.map(FOUND, 0x1000);
            e.mem.set_u32(set, 0x0600_0000);
            let mut component = vec![0u32; 0x90 / 4];
            component[0x8c / 4] = 0x7000_0008;
            e.put_vtable(0x0610_0000, &component);
            e.mem.set_u32(set + 0x30, 0x0610_0000);
            e.register(0x7000_0008, |_, a| ret(u32::from(a[1] == 0)));
            e.set(this, TESLandTexture::pTextureSet, 0x45);
            e.call_log = Some(vec![]);
            e.call(0x0054_0fe0, &args![this]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, WARNING),
                vec![
                    vec![MISSING_GRASS_MESSAGE, 0x109, t + 1, t ^ 0xf000_0000],
                    vec![MISSING_GRASS_MESSAGE, 0x102, t + 1, t ^ 0xf000_0000],
                    vec![MISSING_NORMAL_MESSAGE, set + 1, set ^ 0xf000_0000],
                ]
            );
            assert_eq!(
                calls_to(&log, 0x7000_0008),
                vec![vec![set + 0x30, 0], vec![set + 0x30, 1]]
            );
            // The first node took id 1 (resolved), the third was unlinked.
            assert_eq!(e.mem.u32(nodes), FOUND + 0x101);
            assert_eq!(e.mem.u32(nodes + 4), 0);
            assert_eq!(e.get(this, TESLandTexture::pTextureSet), set);
            assert_eq!(e.mem.u16(set + 0x84) & 1, 1);
            assert_eq!(calls_to(&log, SET_FORM_FLAG_8), vec![vec![t, 1]]);
            assert_eq!(
                calls_to(&log, RT_DYNAMIC_CAST).last().unwrap(),
                &vec![set, 0, TYPE_TES_FORM, TYPE_TEXTURE_SET, 0]
            );
            assert_eq!(
                calls_to(&log, RT_DYNAMIC_CAST)[0],
                vec![0, 0, TYPE_TES_FORM, TYPE_GRASS, 0]
            );

            // A texture set that is not found is reported and stays null.
            e.register(GRASS_LIST, |_, _| ret(0));
            e.set(this, TESLandTexture::pTextureSet, 0x46);
            e.call_log = Some(vec![]);
            e.call(0x0054_0fe0, &args![this]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, WARNING),
                vec![vec![
                    MISSING_TEXTURE_SET_MESSAGE,
                    0x146,
                    t + 1,
                    t ^ 0xf000_0000
                ]]
            );
            assert_eq!(e.get(this, TESLandTexture::pTextureSet), 0);
            assert!(calls_to(&log, 0x7000_0008).is_empty());
        }
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    const STATE: u32 = 0x7300_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// List nodes are 8 bytes (item, next) as the doubles read them; a
    /// node whose item is 0 is empty.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(STATE, 0x40);
        e.register(GRASS_LIST, |_, a| (a[0] + 0x20).into_ret());
        e.register(NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(TEXTURE_SET_GETTER, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(FORM_ID, |_, a| (a[0] + 1).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        for addr in [
            LIST_REMOVE_ALL,
            FORM_COPY_ALL_COMPONENTS,
            COPY_BYTES,
            LIST_ADD_TAIL,
            FORM_START,
            FORM_CLOSE,
            EMPTY_00483710,
            ADD_CHUNK_WORD,
            ADD_CHUNK_BYTE,
            ADD_CHUNK_DATA,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    /// A land texture whose list holds `grasses` (the first node inline at
    /// +0x20, the others allocated).
    fn texture(e: &mut Engine, grasses: &[u32]) -> u32 {
        let this = e.mem.alloc(0x28);
        let mut node = this + 0x20;
        for (index, grass) in grasses.iter().enumerate() {
            e.mem.set_u32(node, *grass);
            if index + 1 < grasses.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        this
    }

    #[test]
    fn copy_takes_the_data_and_the_grasses() {
        let mut e = engine();
        e.register_double(LIST_ADD_TAIL, |e, a| {
            let grass = e.mem.u32(a[1]);
            let n = e.mem.u32(STATE);
            e.mem.set_u32(STATE + 4 + 4 * n, grass);
            e.mem.set_u32(STATE, n + 1);
            Ret::default()
        });
        let this = texture(&mut e, &[]);
        let other = texture(&mut e, &[0x100, 0x200, 0x300]);
        e.mem.set_u8(other + 0x1f, 9);
        e.mem.set_u32(other + 0x18, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0054_0de0, &args![this, other]);
        assert_eq!(calls(&e, LIST_REMOVE_ALL), vec![vec![this + 0x20]]);
        assert_eq!(calls(&e, FORM_COPY_ALL_COMPONENTS), vec![vec![this, other]]);
        assert_eq!(
            calls(&e, COPY_BYTES),
            vec![vec![this + 0x1c, other + 0x1c, 3]]
        );
        assert_eq!(e.mem.u8(this + 0x1f), 9);
        assert_eq!(e.mem.u32(this + 0x18), 0x5000);
        assert_eq!(e.mem.u32(STATE), 3);
        assert_eq!(
            (
                e.mem.u32(STATE + 4),
                e.mem.u32(STATE + 8),
                e.mem.u32(STATE + 0xc)
            ),
            (0x100, 0x200, 0x300)
        );
        assert!(calls(&e, LIST_ADD_TAIL).iter().all(|c| c[0] == this + 0x20));
        // Not a land texture.
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0054_0de0, &args![this, other]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn compare_branches() {
        let mut e = engine();
        e.register(FORM_COMPARE_ALL_COMPONENTS, |e, _| {
            e.mem.u32(STATE).into_ret()
        });
        e.register(MEMCMP, |e, _| e.mem.u32(STATE + 4).into_ret());
        let this = texture(&mut e, &[0x100, 0x200]);
        let same = texture(&mut e, &[0x100, 0x200]);
        assert!(!e.call(0x0054_0ec0, &args![this, same]).bool());
        let other = texture(&mut e, &[0x100, 0x300]);
        assert!(e.call(0x0054_0ec0, &args![this, other]).bool());
        let longer = texture(&mut e, &[0x100, 0x200, 0x300]);
        assert!(e.call(0x0054_0ec0, &args![this, longer]).bool());
        assert!(e.call(0x0054_0ec0, &args![longer, this]).bool());
        e.mem.set_u8(same + 0x1f, 1);
        assert!(e.call(0x0054_0ec0, &args![this, same]).bool());
        e.mem.set_u8(same + 0x1f, 0);
        e.mem.set_u32(same + 0x18, 1);
        assert!(e.call(0x0054_0ec0, &args![this, same]).bool());
        e.mem.set_u32(same + 0x18, 0);
        e.mem.set_u32(STATE + 4, 1);
        assert!(e.call(0x0054_0ec0, &args![this, same]).bool());
        e.mem.set_u32(STATE + 4, 0);
        e.mem.set_u32(STATE, 1);
        assert!(e.call(0x0054_0ec0, &args![this, same]).bool());
        e.mem.set_u32(STATE, 0);
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0054_0ec0, &args![this, same]).bool());
    }

    #[test]
    fn save_writes_the_chunks() {
        let mut e = engine();
        e.register(SAVE_SWAPS, |e, _| e.mem.u32(STATE).into_ret());
        let this = texture(&mut e, &[0x100, 0x200]);
        e.mem.set_u32(this + 0x18, 0x5000);
        e.mem.set_u8(this + 0x1f, 0x40);
        e.mem.set_u32(STATE, 1);
        e.call_log = Some(vec![]);
        e.call(0x0054_1440, &args![this]);
        assert_eq!(
            calls(&e, ADD_CHUNK_WORD),
            vec![
                vec![0x4d41_4e54, 0x5001],
                vec![0x4d41_4e47, 0x101],
                vec![0x4d41_4e47, 0x201]
            ]
        );
        assert_eq!(
            calls(&e, ADD_CHUNK_DATA),
            vec![vec![0x4d41_4e48, this + 0x1c, 3]]
        );
        assert_eq!(calls(&e, ADD_CHUNK_BYTE), vec![vec![0x4d41_4e53, 0x40]]);
        assert_eq!(
            calls(&e, EMPTY_00483710),
            vec![vec![this], vec![this + 0x1c], vec![this + 0x1c]]
        );
        assert_eq!(calls(&e, FORM_CLOSE), vec![vec![this]]);
        // No texture set, no swapping, an empty list.
        let empty = texture(&mut e, &[]);
        e.mem.set_u32(STATE, 0);
        e.call_log = Some(vec![]);
        e.call(0x0054_1440, &args![empty]);
        assert!(calls(&e, ADD_CHUNK_WORD).is_empty());
        assert_eq!(calls(&e, EMPTY_00483710), vec![vec![empty]]);
    }
}
