//! `fallout shared/bgsbodypartdata.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BGSBodyPartData` (form type `0x57`, record `BPTD`) is 0x74 bytes on the
//! PC (0x84 on the Xbox: its `TESForm` is 0x10 bytes larger). Its vtable is
//! at `01045504` (RTTI `BGSBodyPartData`); the `TESModel` component sits at
//! `+0x18` (vtable `010454dc`), the `BGSPreloadable` component at `+0x30`
//! (vtable `010454c4`), the 15 body part pointers (`pPartArray`) at `+0x34`
//! and the ragdoll (`pRagdollData`) at `+0x70`.
//!
//! Vtable slots checked in the exe: `0x10` deleting destructor
//! (`005e4b90`), `0x14` `InitializeData` (`005e4c40`), `0x18` `ClearData`
//! (`005e4c70`), `0x20` `Load` (`005e4d80`), `0x88` `InitItem`
//! (`005e4c90`); slot `0x10` of the `BGSPreloadable` vtable
//! (`QueueFiles` in the Xbox PDB) is `005e5590`.
//!
//! The unit also holds three small getters of `BGSBodyPart` (its
//! `PART_DATA` block starts at `+0x5C`, the same offset as in the Xbox PDB).
//! Functions that run under a compiler exception frame or the stack cookie
//! carry a `Translation incomplete` marker (the unwinding and the cookie
//! check are not translated).

#[allow(unused_imports)]
use crate::prelude::*;

// ---- Layout --------------------------------------------------------------------------

layout! {
    /// `BGSBodyPartData` (Xbox PDB), 0x74 bytes on the PC. Only the fields
    /// this unit uses by name are declared; the part array is indexed.
    pub struct BGSBodyPartData: 0x74 {
        /// `pRagdollData` (Xbox PDB, `BGSRagdoll*`; a form id until
        /// `InitItem` resolves it).
        0x70 pRagdollData: u32,
    }
}

/// `TESModel` component offset.
const MODEL: u32 = 0x18;
/// `pPartArray` (Xbox PDB `+0x44`): 15 `BGSBodyPart*`.
const PARTS: u32 = 0x34;
/// Number of body part slots.
const PART_COUNT: u32 = 15;

/// The three vtables the destructor puts back: the form's, the model
/// component's and the preloadable component's.
const VTABLE: u32 = 0x0104_5504;
const MODEL_VTABLE: u32 = 0x0104_54dc;
const PRELOADABLE_VTABLE: u32 = 0x0104_54c4;

/// The form type of a body part data record.
const FORM_TYPE_BODY_PART_DATA: u32 = 0x57;

/// The four-character code of a chunk, as the file stores it.
const fn tag(name: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*name)
}

const CHUNK_EDID: u32 = tag(b"EDID");
const CHUNK_OBND: u32 = tag(b"OBND");
const CHUNK_MODL: u32 = tag(b"MODL");
const CHUNK_MODT: u32 = tag(b"MODT");
const CHUNK_RAGA: u32 = tag(b"RAGA");
const CHUNK_BPTN: u32 = tag(b"BPTN");
const CHUNK_BPNN: u32 = tag(b"BPNN");
const CHUNK_PNAM: u32 = tag(b"PNAM");

// ---- Globals and strings -------------------------------------------------------------------

/// `BGSBodyPartData::pDefaultBodyPartData` and `pPlayerBodyPartData` (Xbox
/// PDB statics; the code writes these two words).
const DEFAULT_BODY_PART_DATA: u32 = 0x011c_afec;
const PLAYER_BODY_PART_DATA: u32 = 0x011c_aff0;
/// The data handler (`TESDataHandler*`).
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `"DefaultBodyPartData"` and `"PlayerBodyPartData"`.
const DEFAULT_EDITOR_ID: u32 = 0x0104_5814;
const PLAYER_EDITOR_ID: u32 = 0x0104_5800;
/// `"MASTERFILE: Could not find ragdoll (%08X) for bodypart data '%s' (%08X)."`
const MESSAGE_NO_RAGDOLL: u32 = 0x0104_5640;
/// `"MASTERFILE: Load failed for body part data '%s' (%08X)."`
const MESSAGE_LOAD_FAILED: u32 = 0x0104_5740;
/// `"MASTERFILE: Body part data for '%s' (%08X) advanced file too far.
/// Attempting recovery."`
const MESSAGE_TOO_FAR: u32 = 0x0104_56e8;
/// `"MASTERFILE: Warnings encountered loading part '%s' for body part data
/// '%s' (%08X)."`
const MESSAGE_PART_WARNINGS: u32 = 0x0104_5690;
/// `"MODELS: Invalid body part index %i in body part data %s (%08X)."`
const MESSAGE_BAD_INDEX: u32 = 0x0104_5778;
/// `"MODELS: Body part index %i already in use in body part data %s (%08X)."`
const MESSAGE_INDEX_IN_USE: u32 = 0x0104_57b8;
/// Two `NiRTTI` records `00653270` casts against in the limb functions (at
/// `011f4acc` and `011f49d8`; which classes they describe is not confirmed
/// here, the second is what `BSDismemberSkinInstance::SetBodyPartEnabled`
/// is then called on).
const RTTI_011F4ACC: u32 = 0x011f_4acc;
const RTTI_011F49D8: u32 = 0x011f_49d8;

/// Type descriptors (names read from the RTTI): `TESForm`, `BGSRagdoll` and
/// `BGSBodyPartData`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_BGS_RAGDOLL: u32 = 0x0118_614c;
const TYPE_BGS_BODY_PART_DATA: u32 = 0x0118_61bc;

// ---- Calls outside this file ---------------------------------------------------------

/// `operator new` (`cdecl`, size; `00401000`) and the release the deleting
/// destructors call (`cdecl`, pointer; `00401030`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `memset` (`cdecl`: destination, value, count; `00403d30`).
const MEMSET: u32 = 0x0040_3d30;
/// `TESModel` destructor body (`thiscall`, `00489070`).
const MODEL_DESTRUCTOR: u32 = 0x0048_9070;
/// `TESForm::~TESForm` (Xbox PDB), `thiscall`.
const FORM_DESTRUCTOR: u32 = 0x0048_3630;
/// The `BGSBodyPart` scalar deleting destructor (`BGSBodyPart::~BGSBodyPart`
/// in the engine map), `thiscall` (part, flags).
const BODY_PART_DELETE: u32 = 0x005e_50c0;
/// `BGSBodyPart` constructor (`thiscall`, `005e3af0`).
const BODY_PART_CONSTRUCTOR: u32 = 0x005e_3af0;
/// `BGSBodyPart::Load` (Xbox PDB), `thiscall` (part, file), `AL`.
const BODY_PART_LOAD: u32 = 0x005e_4100;
/// The body part's `InitItem` (`thiscall` (part, body part data); no name in
/// the engine map).
const BODY_PART_INIT_ITEM: u32 = 0x005e_3d80;
/// `thiscall` on a body part: the address of a string the warning message
/// prints as the part's name (`006838b0`).
const BODY_PART_NAME: u32 = 0x0068_38b0;
/// `thiscall` on a body part: the node name the limb functions look up
/// (`007fa950`).
const BODY_PART_NODE_NAME: u32 = 0x007f_a950;
/// Bit 3 of the form flags (`AL`), `thiscall` (`004013e0`), and its setter
/// (`thiscall` (form, value), `00484ab0`).
const FORM_FLAG_8_TEST: u32 = 0x0040_13e0;
const FORM_SET_FLAG_8: u32 = 0x0048_4ab0;
/// `TESForm::GetFile` (Xbox PDB), `thiscall` (form, `-1`).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB), `cdecl` (pointer to the id, file).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// Form lookup by id (`cdecl`, one word; `004839c0`).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `iFormID` getter (`thiscall`; folded body `0084e3a0`).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// The message function the loaders call (`cdecl`, format and words; its
/// body returns 0 in this build; `005b5e40`).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// Form virtual slots: the name string the messages print (`0x130`), the
/// editor id setter `Load` calls for `EDID` (`0x134`), `LoadObjectBound`
/// (`0xE0`, Xbox PDB), slot `0x128` (two words; the Xbox PDB has `SetFormID`
/// one slot higher, so its identity is not confirmed) and slot `0xC8`
/// (`SetAltered` in the Xbox PDB, one word).
const FORM_GET_EDITOR_ID_SLOT: u32 = 0x130;
const FORM_SET_EDITOR_ID_SLOT: u32 = 0x134;
const FORM_LOAD_OBJECT_BOUND_SLOT: u32 = 0xe0;
const FORM_SLOT_128: u32 = 0x128;
const FORM_SET_ALTERED_SLOT: u32 = 0xc8;
/// `TESForm::LoadForm` (Xbox PDB), `thiscall` (form, file).
const FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESForm` (Xbox PDB): the form type of the record being read.
const FILE_GET_FORM_TYPE: u32 = 0x0047_2660;
/// `TESFile::GetTESChunk` (Xbox PDB): the current chunk's code, 0 at the end.
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
/// Moves to the next chunk (`AL`: true while there is one), `thiscall`.
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESFile::GetChunkData_ov3` (Xbox PDB): `thiscall` (file, buffer, size).
const FILE_GET_CHUNK_BYTES: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (Xbox PDB): `thiscall` (file, pointer to a word).
const FILE_GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// The size of the current chunk, `thiscall` (file).
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESModel::LoadModelChunk` (Xbox PDB), `cdecl` (model, file).
const MODEL_LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// A per-thread warning counter (`cdecl`, no arguments: the word at `+0x2B8`
/// of the thread's TLS block; `0046e8a0`).
const THREAD_WARNING_COUNT: u32 = 0x0046_e8a0;
/// `TESDataHandler::AddFormToDataHandler` (Xbox PDB), `thiscall` (handler,
/// form).
const ADD_FORM_TO_DATA_HANDLER: u32 = 0x0046_03b0;
/// Appends the word at the pointer to a list (`thiscall` (list, pointer),
/// `005ae3d0`).
const LIST_APPEND: u32 = 0x005a_e3d0;
/// Finds a node by name under a root (`cdecl` (root, name), `004aae30`).
const FIND_NODE: u32 = 0x004a_ae30;
/// The parent of a node (`thiscall`, the word at `+0x18`; `009611e0`).
const NODE_PARENT: u32 = 0x0096_11e0;
/// `NiRTTI` cast (`cdecl` (rtti, object), null for a null object;
/// `00653270`).
const RTTI_CAST: u32 = 0x0065_3270;
/// `thiscall` on an object: a pointer derived from it (`0043fad0`).
const OBJECT_0043FAD0: u32 = 0x0043_fad0;
/// The `index`-th child getters of the limb functions (`thiscall`
/// (container, index), `0043b4a0` for hiding, `0045bc00` for showing) and the
/// child count (`thiscall`, `0043b480`).
const CHILD_AT_FOR_HIDE: u32 = 0x0043_b4a0;
const CHILD_AT_FOR_SHOW: u32 = 0x0045_bc00;
const CHILD_COUNT: u32 = 0x0043_b480;
/// `BSDismemberSkinInstance::SetBodyPartEnabled` (Xbox PDB), `thiscall`
/// (instance, part index as a 16-bit value, enabled byte).
const SET_BODY_PART_ENABLED: u32 = 0x005e_4810;
/// Virtual slot `0x0C` of the object the limb functions are given: a
/// container of children (zero for none).
const OBJECT_CHILDREN_SLOT: u32 = 0x0c;
/// `BGSBodyPart` virtual slot 0 (`QueueFiles` in the Xbox PDB).
const BODY_PART_QUEUE_FILES_SLOT: u32 = 0x00;

/// A component pointer, or null when the object itself is null.
fn component(object: u32, offset: u32) -> u32 {
    if object == 0 {
        0
    } else {
        object + offset
    }
}

/// The body part pointer in slot `index`.
fn part_slot(this: Ptr, index: u32) -> u32 {
    this.addr() + PARTS + index * 4
}

/// The editor id and form id of the form, as the messages print them.
fn name_and_id(e: &mut Engine, this: Ptr) -> (u32, u32) {
    let id = e.call(GET_FORM_ID, &args![this]).u32();
    let name = e.vcall(this.addr(), FORM_GET_EDITOR_ID_SLOT, &[]).u32();
    (name, id)
}

/// The resolution `InitItem` does for a form id: move the id to the running
/// load order (the form's file first), look it up and cast it to `target`.
/// Returns the moved id and the cast form.
fn resolve_form(e: &mut Engine, this: Ptr, id: u32, target: u32) -> (u32, u32) {
    let file = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
    let id = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), id);
        e.call(FORM_ADD_COMPILE_INDEX, &args![slot, file]);
        e.mem.u32(slot.addr())
    });
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    let cast = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
        )
        .u32();
    (id, cast)
}

// ---- Constructor, destructor, data ------------------------------------------------------

// Translated from 005e4af0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e4af0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x57 (0x74 bytes); `ECX` is the object.
/// The engine map names it `BGSBodyPartData::BGSBodyPartData` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0104_5504`) is `BGSBodyPartData`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005e4af0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0048_8f50, &args![base + 0x18]);
    e.call(0x004f_9d90, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0104_5504);
    e.mem.set_u32(base + 0x18, 0x0104_54dc);
    e.mem.set_u32(base + 0x30, 0x0104_54c4);
    e.call(0x004f_15a0, &args![this, 0x57u32]);
    e.call(0x005e_4c40, &args![this]);
    this
}

// Translated from 005e4b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (vtable slot `0x10`): runs the destructor body
/// (`005e4bc0`) and, when bit 0 of `flags` is set, frees the object. Returns
/// `this`.
pub fn fn_005e4b90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_005e4bc0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 005e4bc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e4bc0 - the C++ exception unwinding frame (FS:[0] state table that tracks which members are destroyed) is not translated.
/// The destructor body: puts the three vtables back, clears the data
/// (`005e4c70`, which deletes the body parts), destroys the model component
/// and runs `TESForm::~TESForm`.
pub fn fn_005e4bc0(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, VTABLE);
    e.mem.set_u32(object + MODEL, MODEL_VTABLE);
    e.mem.set_u32(object + 0x30, PRELOADABLE_VTABLE);
    fn_005e4c70(e, this);
    e.call(MODEL_DESTRUCTOR, &args![object + MODEL]);
    e.call(FORM_DESTRUCTOR, &args![this]);
}

// Translated from 005e4c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x14` (`InitializeData` in the Xbox PDB): zeroes the 15 body
/// part pointers (`memset` of 0x3C bytes) and the ragdoll.
pub fn fn_005e4c40(e: &mut Engine, this: Ptr) {
    e.call(MEMSET, &args![this.addr() + PARTS, 0u32, 0x3cu32]);
    e.set(
        this.cast::<BGSBodyPartData>(),
        BGSBodyPartData::pRagdollData,
        0,
    );
}

// Translated from 005e4c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x18` (`ClearData` in the Xbox PDB): deletes the body parts
/// (`005e51b0`).
pub fn fn_005e4c70(e: &mut Engine, this: Ptr) {
    fn_005e51b0(e, this);
}

// ---- InitItem, Load --------------------------------------------------------------------

// Translated from 005e4c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBodyPartData::InitItem` (Xbox PDB), vtable slot `0x88`: unless bit 3
/// of the form flags is set, runs each body part's `InitItem` (`005e3d80`,
/// with this form), turns the ragdoll's form id into the form (cast to
/// `BGSRagdoll`; a miss leaves null and logs the id) and marks the
/// form initialized.
pub fn bgs_body_part_data_init_item(e: &mut Engine, this: Ptr) {
    if e.call(FORM_FLAG_8_TEST, &args![this]).bool() {
        return;
    }
    for index in 0..PART_COUNT {
        let part = e.mem.u32(part_slot(this, index));
        if part != 0 {
            e.call(BODY_PART_INIT_ITEM, &args![part, this]);
        }
    }
    let data = this.cast::<BGSBodyPartData>();
    let ragdoll = e.get(data, BGSBodyPartData::pRagdollData);
    if ragdoll != 0 {
        let (id, cast) = resolve_form(e, this, ragdoll, TYPE_BGS_RAGDOLL);
        e.set(data, BGSBodyPartData::pRagdollData, cast);
        if e.get(data, BGSBodyPartData::pRagdollData) == 0 {
            let (name, form_id) = name_and_id(e, this);
            e.call(PRINT_MESSAGE, &args![MESSAGE_NO_RAGDOLL, id, name, form_id]);
        }
    }
    e.call(FORM_SET_FLAG_8, &args![this, 1u32]);
}

// Translated from 005e4d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBodyPartData::Load` (Xbox PDB), vtable slot `0x20`: reads a body part
/// data record (form type `0x57`, compared as the whole returned word) from
/// `file`. Reads the form header (`TESForm::LoadForm`) and loops over the
/// chunks: `EDID` (editor id, virtual slot `0x134`), `OBND` (slot `0xE0`),
/// `MODL`/`MODT` (the model), `RAGA` (the ragdoll's form id) and `BPTN`,
/// `BPNN` or `PNAM`, which start a body part: a new `BGSBodyPart` (0xB0
/// bytes) reads itself (`BGSBodyPart::Load`). When that fails, the part is
/// deleted, a message is logged and the load answers false. Otherwise a
/// following `BPNN`, `RAGA`, `PNAM` or `BPTN` chunk means the part read past
/// its end: a message is logged and that chunk is handled next without
/// advancing; a change of the thread's warning counter during the part's load
/// logs a message; and the part is added (`AddBodyPart`). Unknown chunks are
/// skipped. Returns true at the end of the record.
///
/// Not translated: the stack-protector cookie and the exception frame. The
/// `alloca` buffer for `EDID` is a heap block here.
// Translation incomplete: 005e4d80 - the C++ exception unwinding frame (FS:[0], frees the new body part if its constructor throws) and the security cookie check (`__security_check_cookie`, 00ec408c) are not translated.
pub fn bgs_body_part_data_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let object = this.addr();
    if e.call(FILE_GET_FORM_TYPE, &args![file]).u32() != FORM_TYPE_BODY_PART_DATA {
        return false;
    }
    e.call(FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        let mut recovering = false;
        match chunk {
            CHUNK_PNAM | CHUNK_BPNN | CHUNK_BPTN => {
                let memory = e.call(OPERATOR_NEW, &args![0xb0u32]).u32();
                let part = if memory != 0 {
                    e.call(BODY_PART_CONSTRUCTOR, &args![memory]).u32()
                } else {
                    0
                };
                let warnings = e.call(THREAD_WARNING_COUNT, &[]).u32();
                if !e.call(BODY_PART_LOAD, &args![part, file]).bool() {
                    let (name, id) = name_and_id(e, this);
                    e.call(PRINT_MESSAGE, &args![MESSAGE_LOAD_FAILED, name, id]);
                    if part != 0 {
                        e.call(BODY_PART_DELETE, &args![part, 1u32]);
                    }
                    return false;
                }
                let next = e.call(FILE_GET_CHUNK, &args![file]).u32();
                if matches!(next, CHUNK_BPNN | CHUNK_RAGA | CHUNK_PNAM | CHUNK_BPTN) {
                    let (name, id) = name_and_id(e, this);
                    e.call(PRINT_MESSAGE, &args![MESSAGE_TOO_FAR, name, id]);
                    recovering = true;
                }
                if e.call(THREAD_WARNING_COUNT, &[]).u32() != warnings {
                    let (name, id) = name_and_id(e, this);
                    let part_name = e.call(BODY_PART_NAME, &args![part]).u32();
                    e.call(
                        PRINT_MESSAGE,
                        &args![MESSAGE_PART_WARNINGS, part_name, name, id],
                    );
                }
                bgs_body_part_data_add_body_part(e, this, Ptr::new(part));
            }
            CHUNK_OBND => {
                e.vcall(object, FORM_LOAD_OBJECT_BOUND_SLOT, &args![file]);
            }
            CHUNK_RAGA => {
                let ragdoll = e.with_stack(4, |e, slot| {
                    e.call(FILE_GET_CHUNK_WORD, &args![file, slot]);
                    e.mem.u32(slot.addr())
                });
                e.set(
                    this.cast::<BGSBodyPartData>(),
                    BGSBodyPartData::pRagdollData,
                    ragdoll,
                );
            }
            CHUNK_EDID => {
                // The game takes a stack buffer as large as the chunk and reads
                // at most 0x200 bytes into it.
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                let buffer = e.mem.alloc(size.max(1));
                e.call(FILE_GET_CHUNK_BYTES, &args![file, buffer, 0x200u32]);
                e.vcall(object, FORM_SET_EDITOR_ID_SLOT, &args![buffer]);
                e.mem.free(buffer);
            }
            CHUNK_MODL | CHUNK_MODT => {
                let model = component(object, MODEL);
                e.call(MODEL_LOAD_MODEL_CHUNK, &args![model, file]);
            }
            _ => {}
        }
        if !recovering && !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// ---- Body parts -----------------------------------------------------------------------

// Translated from 005e50f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBodyPartData::GetBodyPart` (Xbox PDB): the body part in slot `index`
/// (0 to 14), null for any other index.
pub fn bgs_body_part_data_get_body_part(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    if index > -1 && index < PART_COUNT as i32 {
        e.mem.u32(part_slot(this, index as u32))
    } else {
        0
    }
}

// Translated from 005e5130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBodyPartData::GetBodyPart_ov2` (Xbox PDB): the first body part whose
/// signed byte at `+0x63` (`PART_DATA::cActorValue` in the Xbox PDB) equals
/// `actor_value`, or null.
pub fn bgs_body_part_data_get_body_part_ov2(e: &mut Engine, this: Ptr, actor_value: i32) -> u32 {
    for index in 0..PART_COUNT {
        let part = e.mem.u32(part_slot(this, index));
        if part != 0 && fn_005e5190(e, Ptr::new(part)) as i8 as i32 == actor_value {
            return part;
        }
    }
    0
}

// Translated from 005e5190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x63` of a `BGSBodyPart` (`PART_DATA::cActorValue` in the
/// Xbox PDB; the rest of `EAX` is the object pointer's upper bytes, which
/// callers do not read).
pub fn fn_005e5190(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x63)
}

// Translated from 005e51b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes every body part (the part's deleting destructor `005e50c0` with 1)
/// and clears the 15 slots.
pub fn fn_005e51b0(e: &mut Engine, this: Ptr) {
    for index in 0..PART_COUNT {
        let part = e.mem.u32(part_slot(this, index));
        if part != 0 {
            e.call(BODY_PART_DELETE, &args![part, 1u32]);
        }
        e.mem.set_u32(part_slot(this, index), 0);
    }
}

// Translated from 005e5220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBodyPartData::AddBodyPart` (Xbox PDB): puts `part` in the slot its
/// type byte names (`005e5300`). An index of 15 or more is logged and the
/// part is not stored; a slot already in use is logged and its part deleted
/// before the new one is stored. A null part does nothing.
pub fn bgs_body_part_data_add_body_part(e: &mut Engine, this: Ptr, part: Ptr) {
    if part.is_null() {
        return;
    }
    let index = fn_005e5300(e, part) as i32;
    if index >= 0 && index < PART_COUNT as i32 {
        let slot = part_slot(this, index as u32);
        if e.mem.u32(slot) != 0 {
            let (name, id) = name_and_id(e, this);
            e.call(PRINT_MESSAGE, &args![MESSAGE_INDEX_IN_USE, index, name, id]);
            let old = e.mem.u32(slot);
            if old != 0 {
                e.call(BODY_PART_DELETE, &args![old, 1u32]);
            }
        }
        e.mem.set_u32(slot, part.addr());
    } else {
        let (name, id) = name_and_id(e, this);
        e.call(PRINT_MESSAGE, &args![MESSAGE_BAD_INDEX, index, name, id]);
    }
}

// Translated from 005e5300 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x61` of a `BGSBodyPart` (`PART_DATA::cType` in the Xbox
/// PDB), zero-extended.
pub fn fn_005e5300(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u8(this.addr() + 0x61) as u32
}

// Translated from 005e5320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends to `list` (`005ae3d0`, given the address of a local holding the
/// part) every body part with flag `2` (`005e5380`). A null list does
/// nothing.
pub fn fn_005e5320(e: &mut Engine, this: Ptr, list: u32) {
    if list == 0 {
        return;
    }
    for index in 0..PART_COUNT {
        let part = e.mem.u32(part_slot(this, index));
        if part == 0 {
            continue;
        }
        if fn_005e5380(e, Ptr::new(part)) {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), part);
                e.call(LIST_APPEND, &args![list, slot]);
            });
        }
    }
}

// Translated from 005e5380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit `2` of the byte at `+0x60` of a `BGSBodyPart` (`PART_DATA::cFlags` in
/// the Xbox PDB).
pub fn fn_005e5380(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x60) & 2 != 0
}

// Translated from 005e53a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBodyPartData::GenerateDefaultBodyPartData` (Xbox PDB): looks up forms
/// `0x1D` and `0x1C` as body part data into `pDefaultBodyPartData` and
/// `pPlayerBodyPartData`. A missing one is made (`new` of 0x74 bytes and the
/// constructor `005e4af0`, null when the allocation fails), then virtual
/// slot `0x128` gets (`id`, 1) and slot `0x134` the editor id
/// `"DefaultBodyPartData"` / `"PlayerBodyPartData"`; the form is added to
/// the data handler and slot `0xC8` gets 0. A newly made player one is
/// not kept in `pPlayerBodyPartData` (the static is cleared again).
// Translation incomplete: 005e53a0 - the C++ exception unwinding frame (FS:[0], frees the new form if its constructor throws) is not translated.
pub fn bgs_body_part_data_generate_default_body_part_data(e: &mut Engine) {
    for (id, global, editor_id) in [
        (0x1du32, DEFAULT_BODY_PART_DATA, DEFAULT_EDITOR_ID),
        (0x1c, PLAYER_BODY_PART_DATA, PLAYER_EDITOR_ID),
    ] {
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let cast = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_BGS_BODY_PART_DATA, 0u32],
            )
            .u32();
        e.set_global(global, cast);
        if e.global::<u32>(global) != 0 {
            continue;
        }
        let memory = e.call(OPERATOR_NEW, &args![0x74u32]).u32();
        let made = if memory != 0 {
            fn_005e4af0(e, Ptr::new(memory)).addr()
        } else {
            0
        };
        e.set_global(global, made);
        let data = e.global::<u32>(global);
        e.vcall(data, FORM_SLOT_128, &args![id, 1u32]);
        let data = e.global::<u32>(global);
        e.vcall(data, FORM_SET_EDITOR_ID_SLOT, &args![editor_id]);
        let handler = e.global::<u32>(DATA_HANDLER);
        let data = e.global::<u32>(global);
        e.call(ADD_FORM_TO_DATA_HANDLER, &args![handler, data]);
        let data = e.global::<u32>(global);
        e.vcall(data, FORM_SET_ALTERED_SLOT, &args![0u32]);
        if global == PLAYER_BODY_PART_DATA {
            e.set_global(PLAYER_BODY_PART_DATA, 0u32);
        }
    }
}

// Translated from 005e5590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `0x10` of the `BGSPreloadable` component's vtable as body part data
/// installs it (`QueueFiles` in the Xbox PDB); `this` is the component at
/// `+0x30`. Calls virtual slot 0 (`BGSBodyPart::QueueFiles` in the Xbox PDB)
/// of every body part with the two arguments.
pub fn fn_005e5590(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    for index in 0..PART_COUNT {
        let part = e.mem.u32(this.addr() + 4 + index * 4);
        if part != 0 {
            e.vcall(part, BODY_PART_QUEUE_FILES_SLOT, &args![first, second]);
        }
    }
}

/// The node walk shared by the two limb functions: for each body part other
/// than `skip_index`, finds its node under `root` (`004aae30` with the part's
/// node name) and walks up the parents (`009611e0`). A walk that reaches
/// `root` first leaves the part alone; one that reaches `ancestor` calls
/// `apply` with the part index.
fn for_parts_below(
    e: &mut Engine,
    this: Ptr,
    ancestor: u32,
    root: u32,
    skip_index: u32,
    mut apply: impl FnMut(&mut Engine, u32),
) {
    for index in 0..PART_COUNT {
        if index == skip_index {
            continue;
        }
        let part = e.mem.u32(part_slot(this, index));
        if part == 0 {
            continue;
        }
        let name = e.call(BODY_PART_NODE_NAME, &args![part]).u32();
        let mut node = e.call(FIND_NODE, &args![root, name]).u32();
        if node == 0 {
            continue;
        }
        loop {
            if node == root {
                break;
            }
            if node == ancestor {
                apply(e, index);
                break;
            }
            node = e.call(NODE_PARENT, &args![node]).u32();
        }
    }
}

/// The skin instances of the limb functions: `object` itself when it has no
/// children (virtual slot `0x0C` answers 0), else each child from `child_at`
/// in turn; each is cast with `RTTI_011F4ACC`, passed through `0043fad0` and
/// cast with `RTTI_011F49D8`, and a non-null result goes to `apply`.
fn for_skin_instances(
    e: &mut Engine,
    object: u32,
    children: u32,
    child_at: u32,
    mut apply: impl FnMut(&mut Engine, u32),
) {
    let mut child = 0u32;
    loop {
        let geometry = if children != 0 {
            let item = e.call(child_at, &args![children, child]).u32();
            child = child.wrapping_add(1);
            e.call(RTTI_CAST, &args![RTTI_011F4ACC, item]).u32()
        } else {
            e.call(RTTI_CAST, &args![RTTI_011F4ACC, object]).u32()
        };
        let inner = if geometry != 0 {
            e.call(OBJECT_0043FAD0, &args![geometry]).u32()
        } else {
            0
        };
        let skin = e.call(RTTI_CAST, &args![RTTI_011F49D8, inner]).u32();
        if skin != 0 {
            apply(e, skin);
        }
        if children == 0 || child >= e.call(CHILD_COUNT, &args![children]).u32() {
            break;
        }
    }
}

// Translated from 005e55e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hides the limbs below `ancestor`: for every body part except
/// `skip_index` whose node (found under `root`) has `ancestor` above it
/// before `root`, disables that part index
/// (`BSDismemberSkinInstance::SetBodyPartEnabled` with 0) on every skin
/// instance of `object` (its children from `0043b4a0`, or `object` itself
/// when it has none). The counterpart of `ShowChildLimbs` (`005e5750`).
pub fn fn_005e55e0(
    e: &mut Engine,
    this: Ptr,
    ancestor: u32,
    root: u32,
    skip_index: u32,
    object: u32,
) {
    let children = e.vcall(object, OBJECT_CHILDREN_SLOT, &[]).u32();
    for_parts_below(e, this, ancestor, root, skip_index, |e, index| {
        for_skin_instances(e, object, children, CHILD_AT_FOR_HIDE, |e, skin| {
            e.call(SET_BODY_PART_ENABLED, &args![skin, index & 0xffff, 0u32]);
        });
    });
}

// Translated from 005e5750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBodyPartData::ShowChildLimbs` (Xbox PDB): as `005e55e0`, but enables
/// the part index (`SetBodyPartEnabled` with 1) and takes the children of
/// `object` from `0045bc00`. The game finds the index with a loop over all
/// 15 indices that matches only the part's own, so one call is made per skin
/// instance.
pub fn bgs_body_part_data_show_child_limbs(
    e: &mut Engine,
    this: Ptr,
    ancestor: u32,
    root: u32,
    skip_index: u32,
    object: u32,
) {
    let children = e.vcall(object, OBJECT_CHILDREN_SLOT, &[]).u32();
    for_parts_below(e, this, ancestor, root, skip_index, |e, index| {
        for_skin_instances(e, object, children, CHILD_AT_FOR_SHOW, |e, skin| {
            for other in 0..PART_COUNT {
                if other == index {
                    e.call(SET_BODY_PART_ENABLED, &args![skin, index & 0xffff, 1u32]);
                }
            }
        });
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_4af0, fn_005e4af0(Ptr) -> Ptr),
        entry!(0x005e_4b90, fn_005e4b90(Ptr, u32) -> Ptr),
        entry!(0x005e_4bc0, fn_005e4bc0(Ptr)),
        entry!(0x005e_4c40, fn_005e4c40(Ptr)),
        entry!(0x005e_4c70, fn_005e4c70(Ptr)),
        entry!(0x005e_4c90, bgs_body_part_data_init_item(Ptr)),
        entry!(0x005e_4d80, bgs_body_part_data_load(Ptr, Ptr) -> bool),
        entry!(0x005e_50f0, bgs_body_part_data_get_body_part(Ptr, i32) -> u32),
        entry!(0x005e_5130, bgs_body_part_data_get_body_part_ov2(Ptr, i32) -> u32),
        entry!(0x005e_5190, fn_005e5190(Ptr) -> u8),
        entry!(0x005e_51b0, fn_005e51b0(Ptr)),
        entry!(0x005e_5220, bgs_body_part_data_add_body_part(Ptr, Ptr)),
        entry!(0x005e_5300, fn_005e5300(Ptr) -> u32),
        entry!(0x005e_5320, fn_005e5320(Ptr, u32)),
        entry!(0x005e_5380, fn_005e5380(Ptr) -> bool),
        entry!(
            0x005e_53a0,
            bgs_body_part_data_generate_default_body_part_data()
        ),
        entry!(0x005e_5590, fn_005e5590(Ptr, u32, u32)),
        entry!(0x005e_55e0, fn_005e55e0(Ptr, u32, u32, u32, u32)),
        entry!(
            0x005e_5750,
            bgs_body_part_data_show_child_limbs(Ptr, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Vec<(u32, Vec<u32>)>;

    fn body_part_data(e: &mut Engine) -> Ptr {
        e.new_object::<BGSBodyPartData>().cast()
    }

    /// A zeroed body part (0xB0 bytes) with type byte `index`.
    fn body_part(e: &mut Engine, index: u8) -> u32 {
        let part = e.mem.alloc(0xb0);
        e.mem.set_u8(part + 0x61, index);
        part
    }

    fn returning(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            returning(e, addr, 0);
        }
    }

    fn give_vtable(e: &mut Engine, object: u32, slots: &[(u32, u32)]) {
        let vtable = e.mem.alloc(0x200);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        e.mem.set_u32(object, vtable);
    }

    /// The logged calls to `addr`, without the address.
    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The double standing in for the form's virtual slot `0x130`.
    const EDITOR_ID_TARGET: u32 = 0x00dd_0130;

    /// Gives the form a vtable whose slot `0x130` answers `0xABCDEF`, and an
    /// id getter answering `form | 0x01000000`.
    fn message_doubles(e: &mut Engine, this: Ptr) {
        give_vtable(
            e,
            this.addr(),
            &[
                (0x130, EDITOR_ID_TARGET),
                (0x134, 0x00dd_0134),
                (0xe0, 0x00dd_00e0),
                (0x128, 0x00dd_0128),
                (0xc8, 0x00dd_00c8),
            ],
        );
        returning(e, EDITOR_ID_TARGET, 0x00ab_cdef);
        e.register(GET_FORM_ID, |_, a| Ret {
            eax: a[0] | 0x0100_0000,
            ..Ret::default()
        });
        quiet(
            e,
            &[
                PRINT_MESSAGE,
                BODY_PART_DELETE,
                0x00dd_0134,
                0x00dd_00e0,
                0x00dd_0128,
                0x00dd_00c8,
            ],
        );
    }

    #[test]
    fn fn_005e4af0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_4af0,
            size: 0x74,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x18)]),
                (0x004f_9d90, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x57)]),
                (0x005e_4c40, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_5504),
                (0x18, 4, 0x0104_54dc),
                (0x30, 4, 0x0104_54c4),
            ],
        });
    }

    #[test]
    fn destructor_body_deletes_the_parts_and_destroys_the_bases() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        let a = d.addr();
        quiet(
            &mut e,
            &[BODY_PART_DELETE, MODEL_DESTRUCTOR, FORM_DESTRUCTOR],
        );
        e.mem.set_u32(a + 0x34 + 4 * 3, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x005e_4bc0, &args![d]);
        assert_eq!(e.mem.u32(a), VTABLE);
        assert_eq!(e.mem.u32(a + 0x18), MODEL_VTABLE);
        assert_eq!(e.mem.u32(a + 0x30), PRELOADABLE_VTABLE);
        assert_eq!(e.mem.u32(a + 0x34 + 4 * 3), 0);
        let log: Calls = e.call_log.take().unwrap()[1..].to_vec();
        assert_eq!(
            log,
            vec![
                (BODY_PART_DELETE, vec![0x5000, 1]),
                (MODEL_DESTRUCTOR, vec![a + 0x18]),
                (FORM_DESTRUCTOR, vec![a]),
            ]
        );
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        quiet(
            &mut e,
            &[MODEL_DESTRUCTOR, FORM_DESTRUCTOR, OPERATOR_DELETE],
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_4b90, &args![d, 0u32]).u32(), d.addr());
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, FORM_DESTRUCTOR).len(), 1);
        e.call(0x005e_4b90, &args![d, 3u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![d.addr()]]);
    }

    #[test]
    fn initialize_data_zeroes_the_parts_and_the_ragdoll() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        quiet(&mut e, &[MEMSET]);
        e.mem.set_u32(d.addr() + 0x70, 7);
        e.call_log = Some(vec![]);
        e.call(0x005e_4c40, &args![d]);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![d.addr() + 0x34, 0, 0x3c]]);
        assert_eq!(e.mem.u32(d.addr() + 0x70), 0);
    }

    #[test]
    fn clear_data_deletes_the_parts() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        quiet(&mut e, &[BODY_PART_DELETE]);
        e.mem.set_u32(d.addr() + 0x34, 0x5000);
        e.mem.set_u32(d.addr() + 0x34 + 4 * 14, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x005e_4c70, &args![d]);
        assert_eq!(
            calls_to(&e, BODY_PART_DELETE),
            vec![vec![0x5000, 1], vec![0x6000, 1]]
        );
        assert_eq!(e.mem.u32(d.addr() + 0x34), 0);
        assert_eq!(e.mem.u32(d.addr() + 0x34 + 4 * 14), 0);
    }

    #[test]
    fn delete_parts_clears_every_slot() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        quiet(&mut e, &[BODY_PART_DELETE]);
        for index in 0..15 {
            e.mem.set_u32(d.addr() + 0x34 + 4 * index, 0x5000 + index);
        }
        e.mem.set_u32(d.addr() + 0x34 + 4 * 2, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_51b0, &args![d]);
        assert_eq!(calls_to(&e, BODY_PART_DELETE).len(), 14);
        for index in 0..15 {
            assert_eq!(e.mem.u32(d.addr() + 0x34 + 4 * index), 0);
        }
    }

    /// Doubles for `InitItem`: ids resolve to `id + 0x1000` unless their low
    /// byte is `0xFF`.
    fn init_item_doubles(e: &mut Engine, this: Ptr, initialized: bool) {
        message_doubles(e, this);
        returning(e, FORM_FLAG_8_TEST, initialized as u32);
        returning(e, FORM_GET_FILE, 0x7000);
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0200_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| Ret {
            eax: if a[0] & 0xff == 0xff {
                0
            } else {
                a[0] + 0x1000
            },
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        quiet(e, &[BODY_PART_INIT_ITEM, FORM_SET_FLAG_8]);
    }

    #[test]
    fn init_item_initializes_the_parts_and_resolves_the_ragdoll() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        let a = d.addr();
        init_item_doubles(&mut e, d, false);
        e.mem.set_u32(a + 0x34 + 4, 0x5000);
        e.mem.set_u32(a + 0x34 + 4 * 9, 0x6000);
        e.mem.set_u32(a + 0x70, 0x40);
        e.call_log = Some(vec![]);
        e.call(0x005e_4c90, &args![d]);
        assert_eq!(
            calls_to(&e, BODY_PART_INIT_ITEM),
            vec![vec![0x5000, a], vec![0x6000, a]]
        );
        assert_eq!(e.mem.u32(a + 0x70), 0x0200_1040);
        assert_eq!(calls_to(&e, RT_DYNAMIC_CAST)[0][3], TYPE_BGS_RAGDOLL);
        assert!(calls_to(&e, PRINT_MESSAGE).is_empty());
        assert_eq!(calls_to(&e, FORM_SET_FLAG_8), vec![vec![a, 1]]);

        // A ragdoll that does not resolve is logged.
        e.mem.set_u32(a + 0x70, 0x4ff);
        e.call_log = Some(vec![]);
        e.call(0x005e_4c90, &args![d]);
        assert_eq!(e.mem.u32(a + 0x70), 0);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![
                MESSAGE_NO_RAGDOLL,
                0x0200_04ff,
                0x00ab_cdef,
                a | 0x0100_0000
            ]]
        );

        // Already initialized: nothing.
        init_item_doubles(&mut e, d, true);
        e.call_log = Some(vec![]);
        e.call(0x005e_4c90, &args![d]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    /// A file double that hands out `chunks`; the position is shared so the
    /// body part load double can move it (as the part's own loader reads its
    /// chunks). Word chunks read `0x500 + position`.
    fn file_doubles(e: &mut Engine, form_type: u32, chunks: Vec<u32>) -> Rc<RefCell<usize>> {
        let position = Rc::new(RefCell::new(0usize));
        returning(e, FILE_GET_FORM_TYPE, form_type);
        let (p, list) = (position.clone(), chunks.clone());
        e.register_double(FILE_GET_CHUNK, move |_, _| Ret {
            eax: list.get(*p.borrow()).copied().unwrap_or(0),
            ..Ret::default()
        });
        let (p, count) = (position.clone(), chunks.len());
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            *p.borrow_mut() += 1;
            Ret {
                eax: (*p.borrow() < count) as u32,
                ..Ret::default()
            }
        });
        let p = position.clone();
        e.register_double(FILE_GET_CHUNK_WORD, move |e, a| {
            e.mem.set_u32(a[1], 0x500 + *p.borrow() as u32);
            Ret::default()
        });
        returning(e, FILE_CHUNK_SIZE, 8);
        quiet(
            e,
            &[FORM_LOAD_FORM, FILE_GET_CHUNK_BYTES, MODEL_LOAD_MODEL_CHUNK],
        );
        position
    }

    /// Doubles for the body parts `Load` makes: `new` hands out parts with
    /// type bytes 0, 1, 2, ...; each part's load moves the file one chunk on
    /// and answers `loads[n]`; the second load raises the warning counter.
    fn part_doubles(e: &mut Engine, position: Rc<RefCell<usize>>, loads: Vec<bool>) {
        let made = Rc::new(RefCell::new(0u8));
        e.register_double(OPERATOR_NEW, move |e, _| {
            let index = *made.borrow();
            *made.borrow_mut() += 1;
            Ret {
                eax: body_part(e, index),
                ..Ret::default()
            }
        });
        e.register(BODY_PART_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let warnings = Rc::new(RefCell::new(0u32));
        let count = warnings.clone();
        e.register_double(THREAD_WARNING_COUNT, move |_, _| Ret {
            eax: *count.borrow(),
            ..Ret::default()
        });
        let loaded = Rc::new(RefCell::new(0usize));
        e.register_double(BODY_PART_LOAD, move |_, _| {
            let n = *loaded.borrow();
            *loaded.borrow_mut() += 1;
            *position.borrow_mut() += 1;
            if n == 1 {
                *warnings.borrow_mut() += 1;
            }
            Ret {
                eax: loads[n] as u32,
                ..Ret::default()
            }
        });
        returning(e, BODY_PART_NAME, 0x00bb_0000);
    }

    #[test]
    fn load_reads_the_chunks_and_the_body_parts() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        let a = d.addr();
        message_doubles(&mut e, d);
        let position = file_doubles(
            &mut e,
            0x57,
            vec![
                CHUNK_EDID,
                CHUNK_OBND,
                CHUNK_RAGA,
                CHUNK_MODL,
                CHUNK_BPTN,
                tag(b"NAM5"),
                CHUNK_BPNN,
                CHUNK_BPTN,
                tag(b"NAM5"),
            ],
        );
        part_doubles(&mut e, position, vec![true, true, true]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_4d80, &args![d, 0x9000u32]).bool());
        assert_eq!(calls_to(&e, FORM_LOAD_FORM), vec![vec![a, 0x9000]]);
        assert_eq!(calls_to(&e, 0x00dd_0134).len(), 1);
        assert_eq!(calls_to(&e, FILE_GET_CHUNK_BYTES)[0][2], 0x200);
        assert_eq!(calls_to(&e, 0x00dd_00e0), vec![vec![a, 0x9000]]);
        assert_eq!(e.mem.u32(a + 0x70), 0x502);
        assert_eq!(
            calls_to(&e, MODEL_LOAD_MODEL_CHUNK),
            vec![vec![a + 0x18, 0x9000]]
        );
        // Three parts, stored by their type bytes.
        let parts: Vec<u32> = (0..3).map(|i| e.mem.u32(a + 0x34 + 4 * i)).collect();
        assert!(parts.iter().all(|&p| p != 0));
        for (i, &p) in parts.iter().enumerate() {
            assert_eq!(e.mem.u8(p + 0x61) as usize, i);
        }
        // The second part ran past its end (the next chunk was `BPTN`) and
        // raised the warning counter.
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![
                vec![MESSAGE_TOO_FAR, 0x00ab_cdef, a | 0x0100_0000],
                vec![
                    MESSAGE_PART_WARNINGS,
                    0x00bb_0000,
                    0x00ab_cdef,
                    a | 0x0100_0000
                ],
            ]
        );
        assert_eq!(calls_to(&e, BODY_PART_NAME), vec![vec![parts[1]]]);
    }

    #[test]
    fn load_fails_when_a_part_fails() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        let a = d.addr();
        message_doubles(&mut e, d);
        let position = file_doubles(&mut e, 0x57, vec![CHUNK_PNAM, tag(b"NAM5")]);
        part_doubles(&mut e, position, vec![false]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005e_4d80, &args![d, 0x9000u32]).bool());
        let part = calls_to(&e, BODY_PART_LOAD)[0][0];
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![MESSAGE_LOAD_FAILED, 0x00ab_cdef, a | 0x0100_0000]]
        );
        assert_eq!(calls_to(&e, BODY_PART_DELETE), vec![vec![part, 1]]);
        assert_eq!(e.mem.u32(a + 0x34), 0);
    }

    #[test]
    fn load_refuses_another_form_type() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        // The game compares the whole word, so `0x157` is not `0x57`.
        file_doubles(&mut e, 0x157, vec![]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005e_4d80, &args![d, 0x9000u32]).bool());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn get_body_part_checks_the_index() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        e.mem.set_u32(d.addr() + 0x34, 0x5000);
        e.mem.set_u32(d.addr() + 0x34 + 4 * 14, 0x6000);
        assert_eq!(e.call(0x005e_50f0, &args![d, 0i32]).u32(), 0x5000);
        assert_eq!(e.call(0x005e_50f0, &args![d, 14i32]).u32(), 0x6000);
        assert_eq!(e.call(0x005e_50f0, &args![d, 15i32]).u32(), 0);
        assert_eq!(e.call(0x005e_50f0, &args![d, -1i32]).u32(), 0);
    }

    #[test]
    fn get_body_part_by_actor_value() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        let first = body_part(&mut e, 0);
        let second = body_part(&mut e, 1);
        e.mem.set_u8(first + 0x63, 0x05);
        e.mem.set_u8(second + 0x63, 0xff);
        e.mem.set_u32(d.addr() + 0x34 + 4, first);
        e.mem.set_u32(d.addr() + 0x34 + 4 * 7, second);
        assert_eq!(e.call(0x005e_5130, &args![d, 5i32]).u32(), first);
        assert_eq!(e.call(0x005e_5130, &args![d, -1i32]).u32(), second);
        assert_eq!(e.call(0x005e_5130, &args![d, 0xffi32]).u32(), 0);
        assert_eq!(e.call(0x005e_5130, &args![d, 0i32]).u32(), 0);
    }

    #[test]
    fn body_part_byte_getters() {
        let mut e = Engine::new();
        let part = body_part(&mut e, 0x0e);
        e.mem.set_u8(part + 0x63, 0x9a);
        e.mem.set_u8(part + 0x60, 0x02);
        assert_eq!(e.call(0x005e_5190, &args![part]).u8(), 0x9a);
        assert_eq!(e.call(0x005e_5300, &args![part]).u32(), 0x0e);
        assert!(e.call(0x005e_5380, &args![part]).bool());
        e.mem.set_u8(part + 0x60, 0xfd);
        assert!(!e.call(0x005e_5380, &args![part]).bool());
    }

    #[test]
    fn type_getter_zero_extends() {
        let mut e = Engine::new();
        let part = body_part(&mut e, 0xf0);
        assert_eq!(e.call(0x005e_5300, &args![part]).u32(), 0xf0);
    }

    #[test]
    fn flag_2_getter() {
        let mut e = Engine::new();
        let part = body_part(&mut e, 0);
        assert!(!e.call(0x005e_5380, &args![part]).bool());
        e.mem.set_u8(part + 0x60, 0x03);
        assert!(e.call(0x005e_5380, &args![part]).bool());
    }

    #[test]
    fn actor_value_getter() {
        let mut e = Engine::new();
        let part = body_part(&mut e, 0);
        e.mem.set_u8(part + 0x63, 0x11);
        assert_eq!(e.call(0x005e_5190, &args![part]).u8(), 0x11);
    }

    #[test]
    fn add_body_part_stores_logs_and_replaces() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        let a = d.addr();
        message_doubles(&mut e, d);
        let first = body_part(&mut e, 4);
        let second = body_part(&mut e, 4);
        let bad = body_part(&mut e, 15);
        e.call_log = Some(vec![]);
        e.call(0x005e_5220, &args![d, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.call(0x005e_5220, &args![d, first]);
        assert_eq!(e.mem.u32(a + 0x34 + 16), first);
        assert!(calls_to(&e, PRINT_MESSAGE).is_empty());
        e.call(0x005e_5220, &args![d, second]);
        assert_eq!(e.mem.u32(a + 0x34 + 16), second);
        assert_eq!(calls_to(&e, BODY_PART_DELETE), vec![vec![first, 1]]);
        e.call(0x005e_5220, &args![d, bad]);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![
                vec![MESSAGE_INDEX_IN_USE, 4, 0x00ab_cdef, a | 0x0100_0000],
                vec![MESSAGE_BAD_INDEX, 15, 0x00ab_cdef, a | 0x0100_0000],
            ]
        );
    }

    #[test]
    fn flagged_parts_are_appended_to_the_list() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        let flagged = body_part(&mut e, 0);
        let plain = body_part(&mut e, 1);
        e.mem.set_u8(flagged + 0x60, 2);
        e.mem.set_u32(d.addr() + 0x34, plain);
        e.mem.set_u32(d.addr() + 0x34 + 4 * 6, flagged);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(LIST_APPEND, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call(0x005e_5320, &args![d, 0u32]);
        assert!(seen.borrow().is_empty());
        e.call(0x005e_5320, &args![d, 0x7000u32]);
        assert_eq!(seen.borrow().as_slice(), &[(0x7000, flagged)]);
    }

    #[test]
    fn generate_default_body_part_data_makes_the_missing_forms() {
        let mut e = Engine::new();
        e.map(DEFAULT_BODY_PART_DATA & !0xfff, 0x1000);
        e.map(DATA_HANDLER & !0xfff, 0x1000);
        e.set_global(DATA_HANDLER, 0x3333u32);
        let existing = e.mem.alloc(0x74);
        // Form 0x1D exists; form 0x1C does not.
        e.register_double(LOOKUP_FORM, move |_, a| Ret {
            eax: if a[0] == 0x1d { existing } else { 0 },
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let made = e.mem.alloc(0x74);
        returning(&mut e, OPERATOR_NEW, made);
        quiet(
            &mut e,
            &[
                0x0048_3370,
                0x0048_8f50,
                0x004f_9d90,
                0x004f_15a0,
                0x005e_4c40,
                ADD_FORM_TO_DATA_HANDLER,
            ],
        );
        // The constructor stores the game's vtable; replace it after with one
        // of doubles by standing in for the last constructor step.
        e.register_double(0x005e_4c40, |e, a| {
            let vtable = e.mem.alloc(0x200);
            e.mem.set_u32(vtable + 0x128, 0x00dd_0128);
            e.mem.set_u32(vtable + 0x134, 0x00dd_0134);
            e.mem.set_u32(vtable + 0xc8, 0x00dd_00c8);
            e.mem.set_u32(a[0], vtable);
            Ret::default()
        });
        quiet(&mut e, &[0x00dd_0128, 0x00dd_0134, 0x00dd_00c8]);
        e.call_log = Some(vec![]);
        e.call(0x005e_53a0, &[]);
        assert_eq!(e.global::<u32>(DEFAULT_BODY_PART_DATA), existing);
        assert_eq!(e.global::<u32>(PLAYER_BODY_PART_DATA), 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x74]]);
        assert_eq!(calls_to(&e, 0x00dd_0128), vec![vec![made, 0x1c, 1]]);
        assert_eq!(
            calls_to(&e, 0x00dd_0134),
            vec![vec![made, PLAYER_EDITOR_ID]]
        );
        assert_eq!(
            calls_to(&e, ADD_FORM_TO_DATA_HANDLER),
            vec![vec![0x3333, made]]
        );
        assert_eq!(calls_to(&e, 0x00dd_00c8), vec![vec![made, 0]]);
        let casts: Vec<u32> = calls_to(&e, RT_DYNAMIC_CAST).iter().map(|c| c[3]).collect();
        assert_eq!(casts, vec![TYPE_BGS_BODY_PART_DATA; 2]);
    }

    #[test]
    fn generate_default_body_part_data_keeps_a_made_default_one() {
        let mut e = Engine::new();
        e.map(DEFAULT_BODY_PART_DATA & !0xfff, 0x1000);
        e.map(DATA_HANDLER & !0xfff, 0x1000);
        let existing = e.mem.alloc(0x74);
        // Form 0x1C exists; form 0x1D does not.
        e.register_double(LOOKUP_FORM, move |_, a| Ret {
            eax: if a[0] == 0x1c { existing } else { 0 },
            ..Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let made = e.mem.alloc(0x74);
        returning(&mut e, OPERATOR_NEW, made);
        quiet(
            &mut e,
            &[
                0x0048_3370,
                0x0048_8f50,
                0x004f_9d90,
                0x004f_15a0,
                ADD_FORM_TO_DATA_HANDLER,
                0x00dd_0128,
                0x00dd_0134,
                0x00dd_00c8,
            ],
        );
        e.register_double(0x005e_4c40, |e, a| {
            let vtable = e.mem.alloc(0x200);
            e.mem.set_u32(vtable + 0x128, 0x00dd_0128);
            e.mem.set_u32(vtable + 0x134, 0x00dd_0134);
            e.mem.set_u32(vtable + 0xc8, 0x00dd_00c8);
            e.mem.set_u32(a[0], vtable);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x005e_53a0, &[]);
        assert_eq!(e.global::<u32>(DEFAULT_BODY_PART_DATA), made);
        assert_eq!(e.global::<u32>(PLAYER_BODY_PART_DATA), existing);
        assert_eq!(
            calls_to(&e, 0x00dd_0134),
            vec![vec![made, DEFAULT_EDITOR_ID]]
        );
    }

    #[test]
    fn queue_files_calls_every_part() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        let first = body_part(&mut e, 0);
        let second = body_part(&mut e, 1);
        give_vtable(&mut e, first, &[(0, 0x00dd_0000)]);
        give_vtable(&mut e, second, &[(0, 0x00dd_0000)]);
        quiet(&mut e, &[0x00dd_0000]);
        e.mem.set_u32(d.addr() + 0x34 + 4 * 2, first);
        e.mem.set_u32(d.addr() + 0x34 + 4 * 14, second);
        e.call_log = Some(vec![]);
        e.call(0x005e_5590, &args![d.addr() + 0x30, 7u32, 8u32]);
        assert_eq!(
            calls_to(&e, 0x00dd_0000),
            vec![vec![first, 7, 8], vec![second, 7, 8]]
        );
    }

    /// Doubles for the limb functions. Parts 0, 1 and 2 have node names
    /// `0x100 + index`; their nodes are `0x2000 + index`, whose parents are:
    /// node 0 -> root, node 1 -> `0x3000` (the ancestor) and node 2 ->
    /// `0x3001` -> `0x3000`. The object has two children (`0x10`, `0x11`),
    /// both casting to geometry and skin instances `0x4000 + child`.
    fn limb_doubles(e: &mut Engine, this: Ptr, children: u32) -> u32 {
        for index in 0..3u32 {
            let part = body_part(e, index as u8);
            e.mem.set_u32(this.addr() + 0x34 + 4 * index, part);
            e.mem.set_u32(part + 0x80, 0x100 + index);
        }
        e.register(BODY_PART_NODE_NAME, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x80),
            ..Ret::default()
        });
        e.register(FIND_NODE, |_, a| Ret {
            eax: if a[0] == 0x9000 { a[1] + 0x1f00 } else { 0 },
            ..Ret::default()
        });
        e.register(NODE_PARENT, |_, a| Ret {
            eax: match a[0] {
                0x2000 => 0x9000,
                0x2001 => 0x3000,
                0x2002 => 0x3001,
                0x3001 => 0x3000,
                _ => 0,
            },
            ..Ret::default()
        });
        let object = e.mem.alloc(0x20);
        give_vtable(e, object, &[(0x0c, 0x00dd_000c)]);
        returning(e, 0x00dd_000c, children);
        e.register(CHILD_AT_FOR_HIDE, |_, a| Ret {
            eax: 0x10 + a[1],
            ..Ret::default()
        });
        e.register(CHILD_AT_FOR_SHOW, |_, a| Ret {
            eax: 0x10 + a[1],
            ..Ret::default()
        });
        returning(e, CHILD_COUNT, 2);
        e.register(RTTI_CAST, |_, a| Ret {
            eax: if a[1] == 0 { 0 } else { a[1] },
            ..Ret::default()
        });
        e.register(OBJECT_0043FAD0, |_, a| Ret {
            eax: 0x4000 + a[0],
            ..Ret::default()
        });
        quiet(e, &[SET_BODY_PART_ENABLED]);
        object
    }

    #[test]
    fn hide_disables_the_parts_below_the_ancestor() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        let object = limb_doubles(&mut e, d, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x005e_55e0, &args![d, 0x3000u32, 0x9000u32, 99u32, object]);
        assert_eq!(
            calls_to(&e, SET_BODY_PART_ENABLED),
            vec![
                vec![0x4010, 1, 0],
                vec![0x4011, 1, 0],
                vec![0x4010, 2, 0],
                vec![0x4011, 2, 0],
            ]
        );
        assert_eq!(calls_to(&e, CHILD_AT_FOR_HIDE)[0], vec![0x55, 0]);

        // The skipped index is left alone; without children the object
        // itself is the skin instance.
        returning(&mut e, 0x00dd_000c, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_55e0, &args![d, 0x3000u32, 0x9000u32, 2u32, object]);
        assert_eq!(
            calls_to(&e, SET_BODY_PART_ENABLED),
            vec![vec![0x4000 + object, 1, 0]]
        );
    }

    #[test]
    fn show_child_limbs_enables_the_parts_below_the_ancestor() {
        let mut e = Engine::new();
        let d = body_part_data(&mut e);
        let object = limb_doubles(&mut e, d, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x005e_5750, &args![d, 0x3000u32, 0x9000u32, 1u32, object]);
        assert_eq!(
            calls_to(&e, SET_BODY_PART_ENABLED),
            vec![vec![0x4010, 2, 1], vec![0x4011, 2, 1]]
        );
        assert_eq!(calls_to(&e, CHILD_AT_FOR_SHOW).len(), 2);
        assert!(calls_to(&e, CHILD_AT_FOR_HIDE).is_empty());
    }
}
