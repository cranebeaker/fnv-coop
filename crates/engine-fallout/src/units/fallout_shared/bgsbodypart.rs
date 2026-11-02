//! `fallout shared/bgsbodypart.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BGSBodyPart` is one body part of a body part data form: node names, the
//! dismemberment models and the `PART_DATA` record (`BPND`). The limb
//! functions switch a part's partitions of a `BSDismemberSkinInstance` on
//! and off (the part's number and the number + 200, its cap).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSBodyPart` (Xbox PDB), 0xB0 bytes on both builds (`Load` reads the
    /// 0x54-byte `PART_DATA` to +0x5C): the vtable at +0 (slot 0
    /// `QueueFiles`), five `BSStringT` names, two models and the data.
    pub struct BGSBodyPart: 0xB0 {
        /// `NodeName` (Xbox PDB): `BSStringT<char>`, chunk `BPNN`.
        0x04 NodeName: Inline<crate::types::BSStringT>,
        /// `TargetName` (Xbox PDB): `BSStringT<char>`, chunk `BPNT`.
        0x0C TargetName: Inline<crate::types::BSStringT>,
        /// `IKStartNodeName` (Xbox PDB): `BSStringT<char>`, chunk `BPNI`.
        0x14 IKStartNodeName: Inline<crate::types::BSStringT>,
        /// `PartName` (Xbox PDB): `BSStringT<char>`, chunk `BPTN`.
        0x1C PartName: Inline<crate::types::BSStringT>,
        /// `GoreObjectName` (Xbox PDB): `BSStringT<char>`, chunk `NAM4`.
        /// Then `ExplosionSpecialDebris` (Xbox PDB, a `TESModel`) at +0x2C
        /// (chunks `NAM1`, `NAM5`) and `PSAModel` (a `TESModelPSA`) at +0x44
        /// (chunk `PNAM`), both 0x18 bytes.
        0x24 GoreObjectName: Inline<crate::types::BSStringT>,
        /// `Data` (Xbox PDB): `PART_DATA`, chunk `BPND`.
        0x5C Data: Inline<PartData>,
    }

    /// `PART_DATA` (Xbox PDB), 0x54 bytes on both builds; the fields the
    /// unit's functions use.
    pub struct PartData: 0x54 {
        /// `fDamageMult` (Xbox PDB).
        0x00 fDamageMult: f32,
        /// `pExplosionGenericDebris` (Xbox PDB): `BGSDebris*`.
        0x0C pExplosionGenericDebris: Ptr,
        /// `pExplosion` (Xbox PDB): `BGSExplosion*`.
        0x10 pExplosion: Ptr,
        /// `fTrackingMaxAngle` (Xbox PDB).
        0x14 fTrackingMaxAngle: f32,
        /// `fExplosionGenericDebrisScale` (Xbox PDB).
        0x18 fExplosionGenericDebrisScale: f32,
        /// `pDismemberGenericDebris` (Xbox PDB): `BGSDebris*`.
        0x20 pDismemberGenericDebris: Ptr,
        /// `pDismemberExplosion` (Xbox PDB): `BGSExplosion*`.
        0x24 pDismemberExplosion: Ptr,
        /// `fDismemberGenericDebrisScale` (Xbox PDB).
        0x28 fDismemberGenericDebrisScale: f32,
        /// `pDismemberImpactDataSet` (Xbox PDB): `BGSImpactDataSet*`.
        0x44 pDismemberImpactDataSet: Ptr,
        /// `pExplosionImpactDataSet` (Xbox PDB): `BGSImpactDataSet*`.
        0x48 pExplosionImpactDataSet: Ptr,
        /// `fExplosionSpecialDebrisScale` (Xbox PDB).
        0x50 fExplosionSpecialDebrisScale: f32,
    }
}

/// `BGSBodyPart`'s vtable.
const BODY_PART_VTABLE: u32 = 0x0104_5404;
/// Calls slot 4 of the two models (+0x2C, +0x44) (`(part)`).
const CLEAR_MODELS: u32 = 0x005e_3d50;
/// `TESModelPSA`'s destructor (`(model)`).
const MODEL_PSA_DESTRUCT: u32 = 0x0051_b950;
/// `TESModel`'s destructor (`(model)`).
const MODEL_DESTRUCT: u32 = 0x0048_9070;
/// `BSStringT<char>`'s destructor (`(string)`).
const STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `BSStringT<char>`'s setter `(string, text, 0)`.
const STRING_SET: u32 = 0x0040_37f0;
/// `operator delete` (cdecl, one argument).
const FREE: u32 = 0x0040_1030;
/// Byte-swaps the word at a pointer (cdecl `(pointer, flag)`; the flag
/// byte is not read).
const BYTE_SWAP: u32 = 0x0040_1080;

/// `TESFile::GetTESChunk` (Xbox PDB): the current chunk's tag.
const CHUNK_TAG: u32 = 0x0047_26b0;
/// The current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData` (Xbox PDB, `_ov3`): `(file, buffer, size)`.
const CHUNK_DATA: u32 = 0x0047_2890;
/// Moves to the next chunk; `false` at the end of the record.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// Whether the file's byte order is swapped (the byte at +0x299).
const IS_BYTE_SWAPPED: u32 = 0x0040_1680;
/// The file's form version (the 16-bit word at +0x254).
const FORM_VERSION: u32 = 0x0040_3570;
/// `TESModel::LoadModelChunk` (Xbox PDB, cdecl `(model, file)`).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `TESModel::LoadModelTextureChunk` (Xbox PDB, cdecl `(model, file)`).
const LOAD_MODEL_TEXTURE_CHUNK: u32 = 0x0048_93e0;
/// Prints a message (cdecl, the text).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// `"MODELS: Debris scale value of zero loaded.  Please set to non-zero
/// value in editor."`
const ZERO_DEBRIS_SCALE_MESSAGE: u32 = 0x0104_5430;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// The `float` a zero debris scale is replaced with.
const DEFAULT_DEBRIS_SCALE: u32 = 0x0101_e2bc;

/// The model's path (`0048cee0` on a `TESModel`).
const MODEL_PATH: u32 = 0x0048_cee0;
/// The model loader (the pointer at `011c3b3c`) and its queueing call
/// `(loader, model, a, b, 1, 1, 0, 0)`.
const MODEL_LOADER: u32 = 0x011c_3b3c;
const QUEUE_MODEL: u32 = 0x0044_3d30;

/// The node's child count (`(node)`).
const CHILD_COUNT: u32 = 0x0043_b480;
/// The node's child `index` (`(node, index)`).
const CHILD_AT: u32 = 0x0043_b4a0;
/// The first word of a geometry (its skin instance).
const SKIN_INSTANCE: u32 = 0x0043_fad0;
/// Cast to `BSDismemberSkinInstance` (cdecl `(rtti, object)`, null for a
/// null object).
const DYNAMIC_CAST: u32 = 0x0065_3270;
/// `BSDismemberSkinInstance`'s `NiRTTI`.
const DISMEMBER_SKIN_RTTI: u32 = 0x011f_49d8;

const CHUNK_BPTN: u32 = u32::from_le_bytes(*b"BPTN");
const CHUNK_PNAM: u32 = u32::from_le_bytes(*b"PNAM");
const CHUNK_BPNN: u32 = u32::from_le_bytes(*b"BPNN");
const CHUNK_BPNT: u32 = u32::from_le_bytes(*b"BPNT");
const CHUNK_BPNI: u32 = u32::from_le_bytes(*b"BPNI");
const CHUNK_BPND: u32 = u32::from_le_bytes(*b"BPND");
const CHUNK_NAM1: u32 = u32::from_le_bytes(*b"NAM1");
const CHUNK_NAM2: u32 = u32::from_le_bytes(*b"NAM2");
const CHUNK_NAM3: u32 = u32::from_le_bytes(*b"NAM3");
const CHUNK_NAM4: u32 = u32::from_le_bytes(*b"NAM4");
const CHUNK_NAM5: u32 = u32::from_le_bytes(*b"NAM5");

// Translated from 005e3c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBodyPart`'s destructor: its vtable, the models' slot 4
/// (`005e3d50`), then the members in reverse order: `PSAModel`,
/// `ExplosionSpecialDebris` and the five names.
// Translation incomplete: 005e3c60 - the C++ exception-unwinding frame (FS:[0]) that keeps destroying the remaining members when a destructor throws is not translated.
pub fn fn_005e3c60(e: &mut Engine, this: Ptr<BGSBodyPart>) {
    let base = this.addr();
    e.mem.set_u32(base, BODY_PART_VTABLE);
    e.call(CLEAR_MODELS, &args![this]);
    e.call(MODEL_PSA_DESTRUCT, &args![base + 0x44]);
    e.call(MODEL_DESTRUCT, &args![base + 0x2c]);
    for offset in [0x24, 0x1c, 0x14, 0xc, 4] {
        e.call(STRING_DESTRUCT, &args![base + offset]);
    }
}

// Translated from 005e3fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the `PART_DATA` floats (`00401080` on each, with a zero
/// byte): `fDamageMult`, `fTrackingMaxAngle`,
/// `fExplosionGenericDebrisScale`, `fDismemberGenericDebrisScale`, the six
/// floats of `kGoreTranslate` and `kGoreRotate` (+0x2C to +0x40) and
/// `fExplosionSpecialDebrisScale`.
pub fn fn_005e3fe0(e: &mut Engine, this: Ptr<PartData>) {
    for offset in [
        0, 0x14, 0x18, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x50,
    ] {
        e.call(BYTE_SWAP, &args![this.addr() + offset, 0u32]);
    }
}

/// Reads the current chunk into a new buffer of its size (the game's
/// `__alloca_probe_16` stack buffer; a heap block here, kept in `buffers`
/// until `Load` returns).
fn read_chunk(e: &mut Engine, file: Ptr, buffers: &mut Vec<u32>) -> u32 {
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    let buffer = e.mem.alloc(size);
    buffers.push(buffer);
    e.call(CHUNK_DATA, &args![file, buffer, 0u32]);
    buffer
}

/// Reads the current chunk and sets the string at `target` to it.
fn read_string_chunk(e: &mut Engine, file: Ptr, target: u32, buffers: &mut Vec<u32>) {
    let buffer = read_chunk(e, file, buffers);
    e.call(STRING_SET, &args![target, buffer, 0u32]);
}

fn next_chunk(e: &mut Engine, file: Ptr) -> bool {
    e.call(NEXT_CHUNK, &args![file]).bool()
}

fn chunk_tag(e: &mut Engine, file: Ptr) -> u32 {
    e.call(CHUNK_TAG, &args![file]).u32()
}

fn form_version(e: &mut Engine, file: Ptr) -> u32 {
    e.call(FORM_VERSION, &args![file]).u32() & 0xffff
}

// Translated from 005e4100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBodyPart::Load` (Xbox PDB): reads a body part's chunks from `file`
/// (false for a null file). In order: optional `BPTN` (`PartName`) and
/// `PNAM` (`PSAModel`, `LoadModelChunk`), then `BPNN` (`NodeName`) and
/// `BPNT` (`TargetName`), optional `BPNI` (`IKStartNodeName`), then `BPND`:
/// 0x54 bytes into `Data`, byte-swapped ([`fn_005e3fe0`]) before and after
/// the read when the file is swapped. A zero (ordered equal to 0.0)
/// `fExplosionSpecialDebrisScale`, `fDismemberGenericDebrisScale` or
/// `fExplosionGenericDebrisScale` becomes the float at `0101e2bc`, and any
/// such replacement prints a message. Then `NAM1` (a non-empty chunk is
/// the `ExplosionSpecialDebris` model's path, through its slot 0x18); for
/// form versions below 3 the `NAM2` and `NAM3` chunks are read and
/// dropped; then `NAM4` (`GoreObjectName`) and `NAM5` (the model's texture
/// chunk). A required chunk that is missing, or a record that ends before
/// it, returns false, except that a missing `NAM2`, `NAM3` or `NAM4`
/// returns whether the form version is below 2, and a missing `NAM5` (or
/// the end after `NAM4`) returns true. Every chunk read moves to the next
/// one; reaching the end after `BPTN`, `PNAM`, `BPNN`, `BPNT`, `BPNI` or
/// `BPND` returns false.
// Translation incomplete: 005e4100 - the /GS stack-cookie check (00ec408c) is not translated; the __alloca_probe_16 stack buffers are heap blocks freed on return.
pub fn bgs_body_part_load(e: &mut Engine, this: Ptr<BGSBodyPart>, file: Ptr) -> bool {
    if file.is_null() {
        return false;
    }
    let mut buffers = Vec::new();
    let result = load_chunks(e, this.addr(), file, &mut buffers);
    for buffer in buffers {
        e.mem.free(buffer);
    }
    result
}

/// The body of [`bgs_body_part_load`] after the null check.
fn load_chunks(e: &mut Engine, base: u32, file: Ptr, buffers: &mut Vec<u32>) -> bool {
    if chunk_tag(e, file) == CHUNK_BPTN {
        read_string_chunk(e, file, base + 0x1c, buffers);
        if !next_chunk(e, file) {
            return false;
        }
    }
    if chunk_tag(e, file) == CHUNK_PNAM {
        e.call(LOAD_MODEL_CHUNK, &args![base + 0x44, file]);
        if !next_chunk(e, file) {
            return false;
        }
    }
    if chunk_tag(e, file) != CHUNK_BPNN {
        return false;
    }
    read_string_chunk(e, file, base + 4, buffers);
    if !next_chunk(e, file) {
        return false;
    }
    if chunk_tag(e, file) != CHUNK_BPNT {
        return false;
    }
    read_string_chunk(e, file, base + 0xc, buffers);
    if !next_chunk(e, file) {
        return false;
    }
    if chunk_tag(e, file) == CHUNK_BPNI {
        read_string_chunk(e, file, base + 0x14, buffers);
        if !next_chunk(e, file) {
            return false;
        }
    }
    if chunk_tag(e, file) != CHUNK_BPND {
        return false;
    }
    let data: Ptr<PartData> = Ptr::new(base + 0x5c);
    if e.call(IS_BYTE_SWAPPED, &args![file]).bool() {
        fn_005e3fe0(e, data);
    }
    e.call(CHUNK_DATA, &args![file, data, 0x54u32]);
    if e.call(IS_BYTE_SWAPPED, &args![file]).bool() {
        fn_005e3fe0(e, data);
    }
    let mut replaced = false;
    for field in [
        PartData::fExplosionSpecialDebrisScale,
        PartData::fDismemberGenericDebrisScale,
        PartData::fExplosionGenericDebrisScale,
    ] {
        let zero: f64 = e.global(ZERO);
        // FCOMP; TEST AH,0x44; JP: only an ordered equal (C3 alone) replaces.
        if e.get(data, field) as f64 == zero {
            let scale: f32 = e.global(DEFAULT_DEBRIS_SCALE);
            e.set(data, field, x87_quiet(scale));
            replaced = true;
        }
    }
    if replaced {
        e.call(PRINT_MESSAGE, &args![ZERO_DEBRIS_SCALE_MESSAGE]);
    }
    if !next_chunk(e, file) {
        return false;
    }
    if chunk_tag(e, file) != CHUNK_NAM1 {
        return false;
    }
    if e.call(CHUNK_SIZE, &args![file]).u32() != 0 {
        let buffer = read_chunk(e, file, buffers);
        e.vcall(base + 0x2c, 0x18, &args![buffer]);
    }
    if form_version(e, file) < 3 {
        if !next_chunk(e, file) || chunk_tag(e, file) != CHUNK_NAM2 {
            return form_version(e, file) < 2;
        }
        if e.call(CHUNK_SIZE, &args![file]).u32() != 0 {
            read_chunk(e, file, buffers);
        }
        if !next_chunk(e, file) || chunk_tag(e, file) != CHUNK_NAM3 {
            return form_version(e, file) < 2;
        }
        if e.call(CHUNK_SIZE, &args![file]).u32() != 0 {
            read_chunk(e, file, buffers);
        }
    }
    if !next_chunk(e, file) || chunk_tag(e, file) != CHUNK_NAM4 {
        return form_version(e, file) < 2;
    }
    if e.call(CHUNK_SIZE, &args![file]).u32() != 0 {
        read_string_chunk(e, file, base + 0x24, buffers);
    }
    if !next_chunk(e, file) || chunk_tag(e, file) != CHUNK_NAM5 {
        return true;
    }
    if e.call(CHUNK_SIZE, &args![file]).u32() != 0 {
        e.call(LOAD_MODEL_TEXTURE_CHUNK, &args![base + 0x2c, file]);
    }
    true
}

// Translated from 005e45e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBodyPart::QueueFiles` (Xbox PDB): queues the
/// `ExplosionSpecialDebris` model with the model loader (`00443d30` on the
/// loader at `011c3b3c`, `(model, a, b, 1, 1, 0, 0)`) when it has a path,
/// then passes `(a, b)` to slot 0x10 of the embedded object at +0x18 of
/// the generic debris forms and impact data sets and at +0x64 of the
/// explosions of `Data`, each when set.
pub fn bgs_body_part_queue_files(e: &mut Engine, this: Ptr<BGSBodyPart>, a: u32, b: u32) {
    let base = this.addr();
    if e.call(MODEL_PATH, &args![base + 0x2c]).u32() != 0 {
        let loader: u32 = e.global(MODEL_LOADER);
        e.call(
            QUEUE_MODEL,
            &args![loader, base + 0x2c, a, b, 1u32, 1u32, 0u32, 0u32],
        );
    }
    let data: Ptr<PartData> = Ptr::new(base + 0x5c);
    for (field, offset) in [
        (PartData::pExplosionGenericDebris, 0x18),
        (PartData::pExplosion, 0x64),
        (PartData::pDismemberGenericDebris, 0x18),
        (PartData::pDismemberExplosion, 0x64),
        (PartData::pDismemberImpactDataSet, 0x18),
        (PartData::pExplosionImpactDataSet, 0x18),
    ] {
        let form = e.get(data, field);
        if !form.is_null() {
            e.vcall(form.addr() + offset, 0x10, &args![a, b]);
        }
    }
}

/// [`bgs_body_part_hide_limb`] and [`bgs_body_part_unhide_limb`]: for a
/// node (slot 0x0C non-null) every child in turn, the count re-read each
/// time; for a geometry (slot 0x18 non-null) whose skin instance is a
/// `BSDismemberSkinInstance`, partition `part` set to `part_enabled` and
/// partition `part + 200` to `cap_enabled`.
fn set_limb(e: &mut Engine, part: u32, object: Ptr, part_enabled: u8, cap_enabled: u8) {
    if object.is_null() {
        return;
    }
    if e.vcall(object.addr(), 0xc, &args![]).u32() != 0 {
        let node = e.vcall(object.addr(), 0xc, &args![]).u32();
        let mut index = 0u32;
        while index < e.call(CHILD_COUNT, &args![node]).u32() {
            let child = e.call(CHILD_AT, &args![node, index]).u32();
            set_limb(e, part, Ptr::new(child), part_enabled, cap_enabled);
            index += 1;
        }
    } else if e.vcall(object.addr(), 0x18, &args![]).u32() != 0 {
        let geometry = e.vcall(object.addr(), 0x18, &args![]).u32();
        let skin = e.call(SKIN_INSTANCE, &args![geometry]).u32();
        let dismember = e
            .call(DYNAMIC_CAST, &args![DISMEMBER_SKIN_RTTI, skin])
            .u32();
        if dismember != 0 {
            let dismember = Ptr::new(dismember);
            bs_dismember_skin_instance_set_body_part_enabled(
                e,
                dismember,
                part as u16,
                part_enabled,
            );
            bs_dismember_skin_instance_set_body_part_enabled(
                e,
                dismember,
                part.wrapping_add(200) as u16,
                cap_enabled,
            );
        }
    }
}

// Translated from 005e4730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBodyPart::HideLimb` (Xbox PDB, cdecl `(part, object)`): through
/// `object` and its descendants, disables partition `part` and enables its
/// cap `part + 200` on every dismember skin instance.
pub fn bgs_body_part_hide_limb(e: &mut Engine, part: u32, object: Ptr) {
    set_limb(e, part, object, 0, 1);
}

// Translated from 005e4810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance::SetBodyPartEnabled` (Xbox PDB): sets
/// `bEnabled` of every partition whose `usBodyPartNum` is `part`, and
/// recomputes `bVisiblePartitions` as the OR of all `bEnabled` bytes.
pub fn bs_dismember_skin_instance_set_body_part_enabled(
    e: &mut Engine,
    this: Ptr,
    part: u16,
    enabled: u8,
) {
    let base = this.addr();
    // BSDismemberSkinInstance::bVisiblePartitions (Xbox PDB) +0x3C
    e.mem.set_u8(base + 0x3c, 0);
    let mut index = 0u32;
    // uiPartitionDataArraySize (Xbox PDB) +0x34
    while index < e.mem.u32(base + 0x34) {
        // pPartitionDataArray (Xbox PDB) +0x38: 4-byte PartitionData
        // (bEnabled +0, usBodyPartNum +2).
        let partition = e.mem.u32(base + 0x38).wrapping_add(index.wrapping_mul(4));
        if e.mem.u16(partition + 2) == part {
            e.mem.set_u8(partition, enabled);
        }
        let partition = e.mem.u32(base + 0x38).wrapping_add(index.wrapping_mul(4));
        let visible = e.mem.u8(base + 0x3c) | e.mem.u8(partition);
        e.mem.set_u8(base + 0x3c, visible);
        index += 1;
    }
}

// Translated from 005e4890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBodyPart::UnhideLimb` (Xbox PDB, cdecl `(part, object)`): through
/// `object` and its descendants, enables partition `part` and disables its
/// cap `part + 200` on every dismember skin instance.
pub fn bgs_body_part_unhide_limb(e: &mut Engine, part: u32, object: Ptr) {
    set_limb(e, part, object, 1, 0);
}

// Translated from 005e50c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBodyPart::~BGSBodyPart` (Xbox PDB name; the body is the scalar
/// deleting destructor): the destructor ([`fn_005e3c60`]), then `operator
/// delete` when bit 0 of `flags` is set. Returns `this`.
pub fn bgs_body_part_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BGSBodyPart>,
    flags: u32,
) -> Ptr<BGSBodyPart> {
    fn_005e3c60(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e3c60, fn_005e3c60(Ptr<BGSBodyPart>)),
        entry!(0x005e3fe0, fn_005e3fe0(Ptr<PartData>)),
        entry!(
            0x005e4100,
            bgs_body_part_load(Ptr<BGSBodyPart>, Ptr) -> bool
        ),
        entry!(
            0x005e45e0,
            bgs_body_part_queue_files(Ptr<BGSBodyPart>, u32, u32)
        ),
        entry!(0x005e4730, bgs_body_part_hide_limb(u32, Ptr)),
        entry!(
            0x005e4810,
            bs_dismember_skin_instance_set_body_part_enabled(Ptr, u16, u8)
        ),
        entry!(0x005e4890, bgs_body_part_unhide_limb(u32, Ptr)),
        entry!(
            0x005e50c0,
            bgs_body_part_scalar_deleting_destructor(Ptr<BGSBodyPart>, u32) -> Ptr<BGSBodyPart>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn nothing(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn destructors_tear_down_in_reverse() {
        let mut e = Engine::new();
        nothing(
            &mut e,
            &[
                CLEAR_MODELS,
                MODEL_PSA_DESTRUCT,
                MODEL_DESTRUCT,
                STRING_DESTRUCT,
                FREE,
            ],
        );
        let part = e.new_object::<BGSBodyPart>();
        let p = part.addr();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_50c0, &[p, 1]).u32(), p);
        assert_eq!(e.mem.u32(p), BODY_PART_VTABLE);
        let calls: Vec<_> = e.call_log.clone().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            calls,
            vec![
                (CLEAR_MODELS, vec![p]),
                (MODEL_PSA_DESTRUCT, vec![p + 0x44]),
                (MODEL_DESTRUCT, vec![p + 0x2c]),
                (STRING_DESTRUCT, vec![p + 0x24]),
                (STRING_DESTRUCT, vec![p + 0x1c]),
                (STRING_DESTRUCT, vec![p + 0x14]),
                (STRING_DESTRUCT, vec![p + 0xc]),
                (STRING_DESTRUCT, vec![p + 4]),
                (FREE, vec![p]),
            ]
        );
        e.call(0x005e_50c0, &[p, 0]);
        assert_eq!(calls_to(&e, FREE).len(), 1);
    }

    #[test]
    fn part_data_swap_covers_the_floats() {
        let mut e = Engine::new();
        nothing(&mut e, &[BYTE_SWAP]);
        let data = e.mem.alloc(0x54);
        e.call_log = Some(vec![]);
        e.call(0x005e_3fe0, &[data]);
        let offsets: Vec<u32> = calls_to(&e, BYTE_SWAP)
            .iter()
            .map(|a| a[0] - data)
            .collect();
        assert_eq!(
            offsets,
            [0, 0x14, 0x18, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x50]
        );
        assert!(calls_to(&e, BYTE_SWAP).iter().all(|a| a[1] == 0));
    }

    /// A scripted record: the chunk tags in order, each chunk's size, the
    /// form version and the byte-order flag.
    struct Record {
        chunks: Vec<(u32, u32)>,
        position: usize,
    }

    fn record_file(
        e: &mut Engine,
        chunks: &[(&[u8; 4], u32)],
        version: u32,
        swapped: bool,
    ) -> Rc<RefCell<Record>> {
        let record = Rc::new(RefCell::new(Record {
            chunks: chunks
                .iter()
                .map(|(tag, size)| (u32::from_le_bytes(**tag), *size))
                .collect(),
            position: 0,
        }));
        let r = record.clone();
        e.register_double(CHUNK_TAG, move |_, _| {
            let r = r.borrow();
            r.chunks.get(r.position).map_or(0, |c| c.0).into_ret()
        });
        let r = record.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            let r = r.borrow();
            r.chunks.get(r.position).map_or(0, |c| c.1).into_ret()
        });
        let r = record.clone();
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut r = r.borrow_mut();
            r.position += 1;
            (r.position < r.chunks.len()).into_ret()
        });
        e.register_double(FORM_VERSION, move |_, _| version.into_ret());
        e.register_double(IS_BYTE_SWAPPED, move |_, _| swapped.into_ret());
        nothing(
            e,
            &[
                CHUNK_DATA,
                STRING_SET,
                LOAD_MODEL_CHUNK,
                LOAD_MODEL_TEXTURE_CHUNK,
                PRINT_MESSAGE,
                BYTE_SWAP,
            ],
        );
        record
    }

    fn load_world() -> (Engine, Ptr<BGSBodyPart>) {
        let mut e = Engine::new();
        e.map(ZERO, 8);
        e.set_global(ZERO, 0.0f64);
        e.map(DEFAULT_DEBRIS_SCALE, 4);
        e.set_global(DEFAULT_DEBRIS_SCALE, 0.1f32);
        let part = e.new_object::<BGSBodyPart>();
        // ExplosionSpecialDebris's vtable with slot 0x18.
        e.put_vtable(0x0150_0000, &[0, 0, 0, 0, 0, 0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        e.mem.set_u32(part.addr() + 0x2c, 0x0150_0000);
        (e, part)
    }

    fn full_record() -> Vec<(&'static [u8; 4], u32)> {
        vec![
            (b"BPTN", 5),
            (b"PNAM", 0),
            (b"BPNN", 6),
            (b"BPNT", 7),
            (b"BPNI", 8),
            (b"BPND", 0x54),
            (b"NAM1", 9),
            (b"NAM2", 4),
            (b"NAM3", 0),
            (b"NAM4", 10),
            (b"NAM5", 12),
        ]
    }

    #[test]
    fn load_reads_every_chunk_of_an_old_record() {
        let (mut e, part) = load_world();
        let p = part.addr();
        let file = e.mem.alloc(0x300);
        let record = record_file(&mut e, &full_record(), 2, true);
        e.mem.set_f32(p + 0x5c + 0x18, 2.0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_4100, &[p, file]).bool());
        assert_eq!(record.borrow().position, 10);
        let targets: Vec<u32> = calls_to(&e, STRING_SET).iter().map(|a| a[0] - p).collect();
        assert_eq!(targets, [0x1c, 4, 0xc, 0x14, 0x24]);
        assert_eq!(calls_to(&e, LOAD_MODEL_CHUNK), vec![vec![p + 0x44, file]]);
        assert_eq!(
            calls_to(&e, LOAD_MODEL_TEXTURE_CHUNK),
            vec![vec![p + 0x2c, file]]
        );
        assert_eq!(calls_to(&e, 0x0150_1000).len(), 1);
        // Swapped before and after the BPND read; reads of BPTN, BPNN,
        // BPNT, BPNI, BPND, NAM1, NAM2 and NAM4 (NAM3 is empty).
        assert_eq!(calls_to(&e, BYTE_SWAP).len(), 22);
        let reads = calls_to(&e, CHUNK_DATA);
        assert_eq!(reads.len(), 8);
        assert!(reads.contains(&vec![file, p + 0x5c, 0x54]));
        // Two zero scales replaced, the non-zero one kept, one message.
        assert_eq!(e.mem.f32(p + 0x5c + 0x50), 0.1);
        assert_eq!(e.mem.f32(p + 0x5c + 0x28), 0.1);
        assert_eq!(e.mem.f32(p + 0x5c + 0x18), 2.0);
        assert_eq!(
            calls_to(&e, PRINT_MESSAGE),
            vec![vec![ZERO_DEBRIS_SCALE_MESSAGE]]
        );
    }

    #[test]
    fn load_of_a_new_record_skips_nam2_and_nam3() {
        let (mut e, part) = load_world();
        let p = part.addr();
        for offset in [0x18, 0x28, 0x50] {
            e.mem.set_f32(p + 0x5c + offset, 1.0);
        }
        let file = e.mem.alloc(0x300);
        let chunks = [
            (b"BPNN", 6),
            (b"BPNT", 7),
            (b"BPND", 0x54),
            (b"NAM1", 0),
            (b"NAM4", 0),
        ];
        record_file(&mut e, &chunks, 3, false);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_4100, &[p, file]).bool());
        assert!(calls_to(&e, BYTE_SWAP).is_empty());
        assert!(calls_to(&e, PRINT_MESSAGE).is_empty());
        assert!(calls_to(&e, 0x0150_1000).is_empty());
        let targets: Vec<u32> = calls_to(&e, STRING_SET).iter().map(|a| a[0] - p).collect();
        assert_eq!(targets, [4, 0xc]);
    }

    #[test]
    fn load_failures() {
        type Case<'a> = (&'a [(&'a [u8; 4], u32)], u32, bool);
        let cases: [Case; 6] = [
            (&[(b"BPNT", 1)], 3, false),
            (&[(b"BPNN", 1)], 3, false),
            (&[(b"BPNN", 1), (b"BPNT", 1), (b"XXXX", 1)], 3, false),
            // A missing NAM2 returns whether the version is below 2.
            (
                &[
                    (b"BPNN", 1),
                    (b"BPNT", 1),
                    (b"BPND", 0x54),
                    (b"NAM1", 0),
                    (b"XXXX", 0),
                ],
                1,
                true,
            ),
            (
                &[
                    (b"BPNN", 1),
                    (b"BPNT", 1),
                    (b"BPND", 0x54),
                    (b"NAM1", 0),
                    (b"XXXX", 0),
                ],
                2,
                false,
            ),
            (
                &[(b"BPNN", 1), (b"BPNT", 1), (b"BPND", 0x54), (b"NAM2", 0)],
                3,
                false,
            ),
        ];
        for (chunks, version, expected) in cases {
            let (mut e, part) = load_world();
            let file = e.mem.alloc(0x300);
            record_file(&mut e, chunks, version, false);
            assert_eq!(
                e.call(0x005e_4100, &[part.addr(), file]).bool(),
                expected,
                "{chunks:?}"
            );
        }
        let (mut e, part) = load_world();
        assert!(!e.call(0x005e_4100, &[part.addr(), 0]).bool());
    }

    #[test]
    fn queue_files_reaches_the_model_and_every_set_form() {
        let mut e = Engine::new();
        nothing(&mut e, &[QUEUE_MODEL]);
        e.register(MODEL_PATH, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.map(MODEL_LOADER, 4);
        e.set_global(MODEL_LOADER, 0x0900_0000u32);
        e.put_vtable(0x0150_0000, &[0, 0, 0, 0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let part = e.new_object::<BGSBodyPart>();
        let p = part.addr();
        e.mem.set_u32(p + 0x2c + 4, 0x0123);
        let debris = e.mem.alloc(0x80);
        let explosion = e.mem.alloc(0x80);
        e.mem.set_u32(debris + 0x18, 0x0150_0000);
        e.mem.set_u32(explosion + 0x64, 0x0150_0000);
        e.mem.set_u32(p + 0x5c + 0x0c, debris);
        e.mem.set_u32(p + 0x5c + 0x24, explosion);
        e.mem.set_u32(p + 0x5c + 0x48, debris);
        e.call_log = Some(vec![]);
        e.call(0x005e_45e0, &[p, 7, 8]);
        assert_eq!(
            calls_to(&e, QUEUE_MODEL),
            vec![vec![0x0900_0000, p + 0x2c, 7, 8, 1, 1, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, 0x0150_1000),
            vec![
                vec![debris + 0x18, 7, 8],
                vec![explosion + 0x64, 7, 8],
                vec![debris + 0x18, 7, 8],
            ]
        );
        e.mem.set_u32(p + 0x2c + 4, 0);
        e.call(0x005e_45e0, &[p, 7, 8]);
        assert_eq!(calls_to(&e, QUEUE_MODEL).len(), 1);
    }

    /// A dismember skin instance with partitions of the given body parts.
    fn skin(e: &mut Engine, parts: &[(u16, u8)]) -> u32 {
        let skin = e.mem.alloc(0x40);
        let array = e.mem.alloc(4 * parts.len() as u32 + 4);
        for (i, &(part, enabled)) in parts.iter().enumerate() {
            e.mem.set_u8(array + 4 * i as u32, enabled);
            e.mem.set_u16(array + 4 * i as u32 + 2, part);
        }
        e.mem.set_u32(skin + 0x34, parts.len() as u32);
        e.mem.set_u32(skin + 0x38, array);
        e.mem.set_u8(skin + 0x3c, 0x55);
        skin
    }

    #[test]
    fn set_body_part_enabled_updates_matches_and_visibility() {
        let mut e = Engine::new();
        let s = skin(&mut e, &[(3, 1), (203, 0), (3, 1)]);
        e.call(0x005e_4810, &[s, 3, 0]);
        let array = e.mem.u32(s + 0x38);
        assert_eq!(e.mem.u8(array), 0);
        assert_eq!(e.mem.u8(array + 8), 0);
        assert_eq!(e.mem.u8(s + 0x3c), 0);
        e.call(0x005e_4810, &[s, 0x1_00cb, 1]);
        assert_eq!(e.mem.u8(array + 4), 1);
        assert_eq!(e.mem.u8(s + 0x3c), 1);
        let empty = skin(&mut e, &[]);
        e.call(0x005e_4810, &[empty, 3, 1]);
        assert_eq!(e.mem.u8(empty + 0x3c), 0);
    }

    /// A node (slot 0x0C returns itself) with two geometry children (slot
    /// 0x18 returns themselves) whose skins are `skins`.
    fn limb_tree(e: &mut Engine, skins: [u32; 2]) -> u32 {
        e.put_vtable(0x0150_0000, &[0, 0, 0, 0x0150_1000, 0, 0, 0x0150_1100]);
        e.register(0x0150_1000, |_, a| a[0].into_ret());
        e.register(0x0150_1100, |_, _| 0u32.into_ret());
        e.put_vtable(0x0150_0100, &[0, 0, 0, 0x0150_1100, 0, 0, 0x0150_1200]);
        e.register(0x0150_1200, |_, a| a[0].into_ret());
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node, 0x0150_0000);
        let children = e.mem.alloc(8);
        for (i, skin) in skins.into_iter().enumerate() {
            let child = e.mem.alloc(8);
            e.mem.set_u32(child, 0x0150_0100);
            e.mem.set_u32(child + 4, skin);
            e.mem.set_u32(children + 4 * i as u32, child);
        }
        e.mem.set_u32(node + 4, children);
        e.register(CHILD_COUNT, |_, _| 2u32.into_ret());
        e.register(CHILD_AT, |e, a| {
            e.mem.u32(e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        e.register(SKIN_INSTANCE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[1].into_ret());
        node
    }

    #[test]
    fn hide_and_unhide_switch_part_and_cap_in_every_child() {
        let mut e = Engine::new();
        let first = skin(&mut e, &[(5, 1), (205, 0)]);
        let tree = limb_tree(&mut e, [first, 0]);
        e.call_log = Some(vec![]);
        e.call(0x005e_4730, &[5, tree]);
        let array = e.mem.u32(first + 0x38);
        assert_eq!((e.mem.u8(array), e.mem.u8(array + 4)), (0, 1));
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            vec![
                vec![DISMEMBER_SKIN_RTTI, first],
                vec![DISMEMBER_SKIN_RTTI, 0]
            ]
        );
        assert_eq!(calls_to(&e, CHILD_COUNT).len(), 3);
        e.call(0x005e_4890, &[5, tree]);
        assert_eq!((e.mem.u8(array), e.mem.u8(array + 4)), (1, 0));
        e.call(0x005e_4730, &[5, 0]);
    }
}
