//! `fallout shared/tempeffects/bstempeffectgeometrydecal.cpp` (Xbox PDB source unit), subsystem `fallout shared/tempeffects`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006838d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectGeometryDecal` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00688770` and frees 0x64 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_temp_effect_geometry_decal_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0068_8770, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x64u32]);
    }
    this
}

// Translated from 006885a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00a85580` and frees 0x34 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn ni_skin_instance_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x00a8_5580, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x34u32]);
    }
    this
}

// Translated from 006885e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00688610` and frees 0x40 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_dismember_skin_instance_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0068_8610, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x40u32]);
    }
    this
}

// Translated from 006838a0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d6b04 without reading the receiver. The map names this body `BSTempEffectGeometryDecal::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_006838a0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_6b04
}

// Translated from 00688590 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011f5b2c without reading the receiver. The map names this body `NiSkinInstance::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00688590(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011f_5b2c
}

// Translated from 006885d0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011f49d8 without reading the receiver. The map names this body `BSDismemberSkinInstance::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_006885d0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011f_49d8
}

/// The word getter `00559450` (`thiscall`: the word at `+0`).
const WORD_AT_0_00559450: u32 = 0x0055_9450;
/// The global `fn_00683a60` answers (`011f5b04`).
const GLOBAL_011F5B04: u32 = 0x011f_5b04;
/// The compiler's `eh vector destructor iterator` (`00ec5fce`, four words cleaned by the
/// callee: array, element size, count, destructor), `operator delete` (`00401030`, `cdecl`),
/// the partition destructor `00a81bd0` and `NiSkinInstance`'s destructor `00a85580`
/// (`thiscall`).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const OPERATOR_DELETE: u32 = 0x0040_1030;
const PARTITION_DESTRUCTOR_00A81BD0: u32 = 0x00a8_1bd0;
const NI_SKIN_INSTANCE_DESTRUCTOR: u32 = 0x00a8_5580;
/// `BSDismemberSkinInstance`'s vtable (`01069a84`, by its RTTI name).
const BS_DISMEMBER_SKIN_INSTANCE_VTABLE: u32 = 0x0106_9a84;

// Translated from 004b6190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3::Dot` (engine map; the body matches): `x*x' + y*y' + z*z'` in x87 (each product
/// of two `float`s, the sums in the exe's order), stored to a `float` local and loaded back
/// into `ST0`.
pub fn ni_point3_dot(e: &mut Engine, this: Ptr, other: u32) -> f32 {
    let a = this.addr();
    let x = x87_mul(x87_load32(e.mem.u32(a)), x87_m32(e.mem.u32(other)));
    let y = x87_mul(
        x87_load32(e.mem.u32(a.wrapping_add(4))),
        x87_m32(e.mem.u32(other.wrapping_add(4))),
    );
    let sum = x87_add(x, y);
    let z = x87_mul(
        x87_load32(e.mem.u32(a.wrapping_add(8))),
        x87_m32(e.mem.u32(other.wrapping_add(8))),
    );
    let sum = x87_add(sum, z);
    f32::from_bits(x87_store32(sum))
}

// Translated from 006838b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word at `+0x1c` (through the getter `00559450` on
/// `this + 0x1c`).
pub fn fn_006838b0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(WORD_AT_0_00559450, &args![this.addr().wrapping_add(0x1c)])
        .u32()
}

// Translated from 00683a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: answers the word at `011f5b04` (no arguments).
pub fn fn_00683a60(e: &mut Engine) -> u32 {
    e.mem.u32(GLOBAL_011F5B04)
}

// Translated from 00685e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinPartition::Partition::vector deleting destructor` (engine map, from the vtable):
/// with flag bit 1 the array before `this` (its count in the word at `this - 4`) is
/// destroyed element by element (`00ec5fce` with 0x2c-byte elements and `00a81bd0`) and,
/// with bit 0, the block from `this - 4` freed; answers `this - 4`. Otherwise the one
/// partition is destroyed (`00a81bd0`) and, with bit 0, freed; answers `this`.
pub fn ni_skin_partition_partition_vector_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> u32 {
    let a = this.addr();
    if flags & 2 != 0 {
        let count = e.mem.u32(a.wrapping_sub(4));
        e.call(
            VECTOR_DESTRUCT,
            &args![a, 0x2cu32, count, PARTITION_DESTRUCTOR_00A81BD0],
        );
        if flags & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![a.wrapping_sub(4)]);
        }
        return a.wrapping_sub(4);
    }
    e.call(PARTITION_DESTRUCTOR_00A81BD0, &args![a]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![a]);
    }
    a
}

// Translated from 00688430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies nine words (`REP MOVSD`) from the argument to
/// `this + 0x68`.
pub fn fn_00688430(e: &mut Engine, this: Ptr, source: u32) {
    for i in 0..9u32 {
        let word = e.mem.u32(source.wrapping_add(4 * i));
        e.mem.set_u32(this.addr().wrapping_add(0x68 + 4 * i), word);
    }
}

// Translated from 00688610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDismemberSkinInstance`'s destructor (unnamed in the engine map; its scalar deleting
/// destructor `006885e0` runs it): sets the vtable `01069a84`; with the array at `+0x38`,
/// frees it (`operator delete`) and clears it and the count at `+0x34`; then
/// `NiSkinInstance`'s destructor (`00a85580`).
pub fn bs_dismember_skin_instance_destructor(e: &mut Engine, this: Ptr) {
    let a = this.addr();
    e.mem.set_u32(a, BS_DISMEMBER_SKIN_INSTANCE_VTABLE);
    let array = e.mem.u32(a.wrapping_add(0x38));
    if array != 0 {
        e.call(OPERATOR_DELETE, &args![array]);
        e.mem.set_u32(a.wrapping_add(0x38), 0);
        e.mem.set_u32(a.wrapping_add(0x34), 0);
    }
    e.call(NI_SKIN_INSTANCE_DESTRUCTOR, &args![this]);
}

/// The word [`fn_00688460`] answers (`011c6228`; the decal code names its shapes with it
/// when the decal is not permanent).
const TRANSIENT_NAME_WORD: u32 = 0x011c_6228;

// Translated from 00688460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): answers the word at `011c6228`.
pub fn fn_00688460(e: &mut Engine) -> u32 {
    e.global::<u32>(TRANSIENT_NAME_WORD)
}

layout! {
    /// `BSTempEffectGeometryDecal` (Xbox PDB), 0x64 bytes on both builds (the
    /// scalar deleting destructor frees 0x64); the fields after the
    /// `BSTempEffect` base (+0x00..+0x18) the unit's code uses.
    pub struct BSTempEffectGeometryDecal: 0x64 {
        /// `spDecalGeometry` (Xbox PDB): `NiPointer<NiTriBasedGeom>`.
        0x1C spDecalGeometry: u32,
        /// `pOldIndex` (Xbox PDB): `u16*`.
        0x24 pOldIndex: u32,
        /// `bFinished` (Xbox PDB).
        0x28 bFinished: u8,
        /// `spTargetGeom` (Xbox PDB): `NiPointer<NiTriBasedGeom>`.
        0x2C spTargetGeom: u32,
        /// `spParent` (Xbox PDB): `NiPointer<NiNode>`.
        0x30 spParent: u32,
        /// `spDecalParent` (Xbox PDB): `NiPointer<NiNode>`.
        0x34 spDecalParent: u32,
        /// `fScale` (Xbox PDB).
        0x50 fScale: u32,
        /// `fRotation` (Xbox PDB).
        0x54 fRotation: u32,
        /// `pTextureSet` (Xbox PDB): `BGSTextureSet*`.
        0x58 pTextureSet: u32,
        /// `iNumVerts` (Xbox PDB).
        0x5C iNumVerts: u32,
        /// `iBodyParts` (Xbox PDB).
        0x60 iBodyParts: u32,
    }
}

/// `BSTempEffectGeometryDecal`'s vtable (RTTI `.?AVBSTempEffectGeometryDecal@@`).
const GEOMETRY_DECAL_VTABLE: u32 = 0x0106_99bc;
/// `NiSkinInstance`'s vtable (RTTI `.?AVNiSkinInstance@@`).
const NI_SKIN_INSTANCE_VTABLE: u32 = 0x0106_9b14;
/// The `BSTempEffect` constructor (`006827b0`, `thiscall`, `RET 8`: the
/// cell and the lifetime).
const TEMP_EFFECT_CONSTRUCTOR: u32 = 0x0068_27b0;
/// The `BSTempEffect` destructor body (`00682860`).
const TEMP_EFFECT_DESTRUCTOR: u32 = 0x0068_2860;
/// `BSTempEffect::Initialize` (`00682810`, engine map).
const TEMP_EFFECT_INITIALIZE: u32 = 0x0068_2810;
/// `BSTempEffect::Update` (`006828a0`, engine map; `RET 4`, a `float`).
const TEMP_EFFECT_UPDATE: u32 = 0x0068_28a0;
/// `NiObject`'s constructor body (`00a5d3a0`, no stack word).
const NI_OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
/// `00633c90` (`thiscall`, `RET 4`): constructs a smart pointer with an object.
const POINTER_CONSTRUCT_00633C90: u32 = 0x0063_3c90;
/// `0066b0d0` (`thiscall`, `RET 4`): assigns a smart pointer.
const POINTER_SET_0066B0D0: u32 = 0x0066_b0d0;
/// `0045cec0` (no stack word): a smart pointer's destructor.
const POINTER_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;
/// `006815c0` (no stack word): run on the point members before they are set.
const LOCAL_INIT_006815C0: u32 = 0x0068_15c0;
/// `009611e0` (no stack word): an object's parent node.
const PARENT_009611E0: u32 = 0x0096_11e0;
/// The node's slot (+0xE8) that detaches the child it is given.
const SLOT_DETACH_CHILD: u32 = 0xe8;
/// `004534f0` (`thiscall`, `RET 4`: an index): a check of the array index
/// that does nothing in this build.
const INDEX_CHECK_004534F0: u32 = 0x0045_34f0;
/// `00560d30` (`thiscall`, `RET 4`): the address of the colour's
/// component `i` (`this + 4*i`).
const COLOR_COMPONENT_00560D30: u32 = 0x0056_0d30;
/// `004a8ae0`, `0059bb30`, `007af430` (no stack word): the `u16` at +8, the
/// words at +0x24 and +0x20 of the object they are given.
const WORD_AT_8_004A8AE0: u32 = 0x004a_8ae0;
const WORD_AT_24_0059BB30: u32 = 0x0059_bb30;
const WORD_AT_20_007AF430: u32 = 0x007a_f430;
/// `00a855f0` (`thiscall`, `RET 0x20`): the eight-word set-up the
/// [`fn_00687d40`] forwarder hands its words to.
const SKIN_SETUP_00A855F0: u32 = 0x00a8_55f0;
/// `004a8a90` (no stack word): the object at +0x30 (through `00559450`).
const POINTER_AT_30_004A8A90: u32 = 0x004a_8a90;
/// `0043fad0` (no stack word): a test of the target geometry (non-zero to
/// initialize).
const GEOMETRY_TEST_0043FAD0: u32 = 0x0043_fad0;
/// `00726070` (no stack word): an answer of the node compared with 1.
const NODE_ANSWER_00726070: u32 = 0x0072_6070;
/// The skin object's slot (+0x8C) whose byte answer selects the skinned
/// build of the decal.
const SLOT_SKIN_TEST: u32 = 0x8c;
/// The effect's slot (+0x98) that gives the decal geometry `Update` fades.
const SLOT_DECAL_GEOMETRY: u32 = 0x98;
/// `NiAVObject::GetProperty` (`00a59d30`, engine map; `RET 4`).
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// `004ee4f0` (`thiscall`, `RET 4`): run on the parent with the effect.
const PARENT_NOTIFY_004EE4F0: u32 = 0x004e_e4f0;
/// `00453470` (no stack word): a test of the decal parent (zero detaches it).
const NODE_TEST_00453470: u32 = 0x0045_3470;
/// A count the update decrements when it detaches the decal parent.
const DECAL_COUNT_011C57F0: u32 = 0x011c_57f0;
/// The `TESSaveLoadGame` singleton pointer.
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `TESSaveLoadGame::UseSaveGameBlocks` (`00862110`, engine map).
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
/// `00408d60` (no stack word) on the object at `011de4e8`: the address of
/// a byte flag (the size and save reports).
const REPORT_FLAG_00408D60: u32 = 0x0040_8d60;
const REPORT_FLAG_OWNER: u32 = 0x011d_e4e8;
/// `TES::GetWorldSpace` (`004fd3e0`, engine map name; no stack word): here
/// the record of the form being saved (id, then flags at +5).
const SAVING_RECORD_004FD3E0: u32 = 0x004f_d3e0;
/// `004fd3c0` (no stack word): the record of the form being loaded (id,
/// flags at +5, version byte at +9).
const LOADING_RECORD_004FD3C0: u32 = 0x004f_d3c0;
/// `008df040` (no stack word): the current version byte.
const VERSION_008DF040: u32 = 0x008d_f040;
/// `00825c00` (no stack word): the save buffer's current position.
const BUFFER_POSITION_00825C00: u32 = 0x0082_5c00;
/// `008579b0` / `008579e0` (`thiscall`, `RET 8`: data, size): write / read bytes.
const SAVE_DATA_008579B0: u32 = 0x0085_79b0;
const LOAD_DATA_008579E0: u32 = 0x0085_79e0;
/// `004839c0` (cdecl, the id): the form of an id.
const LOOKUP_FORM_004839C0: u32 = 0x0048_39c0;
/// The form's text slot (+0x130) the reports print.
const SLOT_FORM_TEXT: u32 = 0x130;
/// `Error` (`0040fbe0`, engine map; cdecl, a format and its words).
const ERROR_REPORT: u32 = 0x0040_fbe0;
/// `005b5e40` (cdecl, a format and its words): prints a warning.
const PRINT_WARNING_005B5E40: u32 = 0x005b_5e40;
/// The source file name the reports give.
const SOURCE_FILE_NAME: u32 = 0x0106_9ba0;
/// `"GetSaveSize(): %-5i for form %08X %s with flags %08X ending at line %i in file %s"`.
pub(crate) const SIZE_REPORT_FORM: u32 = 0x0101_2cb0;
/// `"GetSaveSize(): %-5i ending at line %i in file %s"`.
pub(crate) const SIZE_REPORT: u32 = 0x0101_2c78;
/// `"SaveGame(): %-5i for form %08X %s with flags %08X ending at line %i in file %s"`.
pub(crate) const SAVE_REPORT_FORM: u32 = 0x0101_53a0;
/// `"SaveGame(): %-5i ending at line %i in file %s"`.
pub(crate) const SAVE_REPORT: u32 = 0x0101_536c;
/// `"SAVELOAD: Save Game Block in file %s on line %i is greater than maximum short size"`.
const BLOCK_TOO_LARGE: u32 = 0x0101_5318;
/// The load warnings: a wrong block header, an overrun and an underrun,
/// each with the loading form (`_FORM`) or with the current version.
const BAD_HEADER_FORM: u32 = 0x0101_5718;
const BAD_HEADER: u32 = 0x0101_56a8;
const OVERRUN_FORM: u32 = 0x0101_5588;
const UNDERRUN_FORM: u32 = 0x0101_5500;
const OVERRUN: u32 = 0x0101_54a0;
const UNDERRUN: u32 = 0x0101_5440;
/// `'KOLB'` as a word: the block header the save writes.
pub(crate) const BLOCK_TAG: u32 = 0x424c_4f4b;
/// The comparison constants of [`fn_00683a70`]: 0.0 and 1.0 (`double`) for
/// the first two coordinates, about 0.1 (`double`) for the third.
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
const DEPTH_LIMIT: u32 = 0x0101_ffa0;

// Translated from 00683900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00683900 - the compiler's exception frame (FS:[0] chain, the unwind states of the four smart pointers) is not translated
/// The `BSTempEffectGeometryDecal` constructor (unnamed in the engine map;
/// `thiscall`, 13 words, `RET 0x34`): the `BSTempEffect` constructor with
/// the cell and lifetime, the class's vtable, the four smart pointers
/// (+0x1C, +0x2C, +0x30, +0x34) constructed with 0, `006815c0` on the two
/// points (+0x38, +0x44), `bFinished` 0, `spDecalGeometry` set to 0,
/// `pOldIndex` 0, `spTargetGeom` set to `target`, `spParent` to its parent
/// (`009611e0`), the origin and vector words copied, `fScale` and
/// `fRotation` (`FLD`/`FSTP`), `iBodyParts`, `iNumVerts` 0 and
/// `pTextureSet` from the creation data's +0x30. `pDecalData` is not
/// written. Returns `this`.
pub fn fn_00683900(
    e: &mut Engine,
    this: Ptr<BSTempEffectGeometryDecal>,
    cell: u32,
    lifetime: f32,
    creation_data: u32,
    target: u32,
    origin: [u32; 3],
    vector: [u32; 3],
    scale: f32,
    rotation: f32,
    body_parts: u32,
) -> Ptr<BSTempEffectGeometryDecal> {
    let base = this.addr();
    e.call(
        TEMP_EFFECT_CONSTRUCTOR,
        &args![base, cell, x87_quiet_bits(lifetime.to_bits())],
    );
    e.mem.set_u32(base, GEOMETRY_DECAL_VTABLE);
    for offset in [0x1cu32, 0x2c, 0x30, 0x34] {
        e.call(POINTER_CONSTRUCT_00633C90, &args![base + offset, 0u32]);
    }
    e.call(LOCAL_INIT_006815C0, &args![base + 0x38]);
    e.call(LOCAL_INIT_006815C0, &args![base + 0x44]);
    e.set(this, BSTempEffectGeometryDecal::bFinished, 0);
    e.call(POINTER_SET_0066B0D0, &args![base + 0x1c, 0u32]);
    e.set(this, BSTempEffectGeometryDecal::pOldIndex, 0);
    e.call(POINTER_SET_0066B0D0, &args![base + 0x2c, target]);
    let parent = e.call(PARENT_009611E0, &args![target]).u32();
    e.call(POINTER_SET_0066B0D0, &args![base + 0x30, parent]);
    for (i, word) in (0u32..).zip(origin) {
        e.mem.set_u32(base + 0x38 + 4 * i, word);
    }
    for (i, word) in (0u32..).zip(vector) {
        e.mem.set_u32(base + 0x44 + 4 * i, word);
    }
    e.set(
        this,
        BSTempEffectGeometryDecal::fScale,
        x87_quiet_bits(scale.to_bits()),
    );
    e.set(
        this,
        BSTempEffectGeometryDecal::fRotation,
        x87_quiet_bits(rotation.to_bits()),
    );
    e.set(this, BSTempEffectGeometryDecal::iBodyParts, body_parts);
    e.set(this, BSTempEffectGeometryDecal::iNumVerts, 0);
    let texture_set = e.mem.u32(creation_data.wrapping_add(0x30));
    e.set(this, BSTempEffectGeometryDecal::pTextureSet, texture_set);
    this
}

/// The uniform form of [`fn_00683900`] (13 words after `this`).
fn fn_00683900_entry(e: &mut Engine, a: &[u32]) -> Ret {
    let r = fn_00683900(
        e,
        Ptr::new(a[0]),
        a[1],
        f32::from_bits(a[2]),
        a[3],
        a[4],
        [a[5], a[6], a[7]],
        [a[8], a[9], a[10]],
        f32::from_bits(a[11]),
        f32::from_bits(a[12]),
        a[13],
    );
    Ret {
        eax: r.addr(),
        ..Default::default()
    }
}

// Translated from 00683a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 0x18`; `this` is not read): whether triangle
/// `triangle` reaches the decal's unit square. Each of its three corners
/// is an index (`indices[triangle + i]` for strips, else
/// `indices[3*triangle + i]`, plus `base`, as a `u16`, then through `remap`
/// when it is given) into the 12-byte `points`. A corner's `x` below 0.0
/// sets bit 1, above 1.0 bit 2, else counts as inside; `y` likewise bits 4
/// and 8; `z` above about 0.1 sets bit 0x10 and also counts. Three counts
/// on one corner answer true at once; after the corners, true when (bits
/// 1 and 2, or 4 and 8) and 0x10 are set. Comparisons are ordered (NaN
/// sets nothing).
pub fn fn_00683a70(
    e: &mut Engine,
    _unused_this: Ptr,
    triangle: u32,
    base: u32,
    strips: u32,
    indices: u32,
    points: u32,
    remap: u32,
) -> bool {
    let zero: f64 = e.global(ZERO_DOUBLE);
    let one: f64 = e.global(ONE_DOUBLE);
    let depth: f64 = e.global(DEPTH_LIMIT);
    let mut flags: u32 = 0;
    for corner in 0..3u32 {
        let mut count: u32 = 0;
        let slot = if strips as u8 != 0 {
            triangle.wrapping_add(corner)
        } else {
            triangle.wrapping_mul(3).wrapping_add(corner)
        };
        let mut index = e
            .mem
            .u16(indices.wrapping_add(slot.wrapping_mul(2)))
            .wrapping_add(base as u16);
        if remap != 0 {
            index = e.mem.u16(remap.wrapping_add(u32::from(index) * 2));
        }
        let point = points.wrapping_add(u32::from(index) * 0xc);
        let x = x87_load32(e.mem.u32(point));
        if x < zero {
            flags |= 1;
        } else if x > one {
            flags |= 2;
        } else {
            count += 1;
        }
        let y = x87_load32(e.mem.u32(point + 4));
        if y < zero {
            flags |= 4;
        } else if y > one {
            flags |= 8;
        } else {
            count += 1;
        }
        let z = x87_load32(e.mem.u32(point + 8));
        if z > depth {
            flags |= 0x10;
            count += 1;
        }
        if count == 3 {
            return true;
        }
    }
    (flags & 3 == 3 || flags & 0xc == 0xc) && flags & 0x10 != 0
}

// Translated from 00685d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, two words): `base` raised to `exponent`
/// ([`fn_00688f10`]; the `float` copied with `FLD`/`FSTP`).
pub fn fn_00685d20(e: &mut Engine, base: f32, exponent: i32) -> f32 {
    fn_00688f10(e, x87_quiet(base), exponent)
}

// Translated from 00685d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a four-`float` constructor storing 0.0 at +0xC, +8, +4, +0 (in
/// that order). Returns `this`.
pub fn fn_00685d40(e: &mut Engine, this: Ptr) -> Ptr {
    for offset in [0xcu32, 8, 4, 0] {
        e.mem.set_u32(this.addr() + offset, x87_store32(0.0));
    }
    this
}

/// The element `index` of an array whose data is at +0 and element size at
/// +4 (after the index check `004534f0`).
fn array_element(e: &mut Engine, array: Ptr, index: u32) -> u32 {
    e.call(INDEX_CHECK_004534F0, &args![array, index]);
    let size = e.mem.u32(array.addr() + 4);
    index
        .wrapping_mul(size)
        .wrapping_add(e.mem.u32(array.addr()))
}

// Translated from 00685d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 8`): fills the four components of `color`
/// (through `00560d30`) from the three `float`s of element `index`: the
/// first three are copied and the fourth is `1 - a - b - c` (x87, each
/// step stored to a `float`).
pub fn fn_00685d70(e: &mut Engine, this: Ptr, index: u32, color: u32) {
    let element = array_element(e, this, index);
    let value = x87_quiet_bits(e.mem.u32(element));
    let mut rest = x87_store32(x87_sub(1.0, x87_load32(value)));
    let slot = e.call(COLOR_COMPONENT_00560D30, &args![color, 0u32]).u32();
    e.mem.set_u32(slot, value);
    let value = x87_quiet_bits(e.mem.u32(element + 4));
    rest = x87_store32(x87_sub(x87_load32(rest), x87_m32(value)));
    let slot = e.call(COLOR_COMPONENT_00560D30, &args![color, 1u32]).u32();
    e.mem.set_u32(slot, value);
    let value = x87_quiet_bits(e.mem.u32(element + 8));
    rest = x87_store32(x87_sub(x87_load32(rest), x87_m32(value)));
    let slot = e.call(COLOR_COMPONENT_00560D30, &args![color, 2u32]).u32();
    e.mem.set_u32(slot, value);
    let slot = e.call(COLOR_COMPONENT_00560D30, &args![color, 3u32]).u32();
    e.mem.set_u32(slot, x87_quiet_bits(rest));
}

// Translated from 00685e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 8`): copies the first word of element `index`
/// to `out`.
pub fn fn_00685e20(e: &mut Engine, this: Ptr, index: u32, out: u32) {
    let element = array_element(e, this, index);
    let word = e.mem.u32(element);
    e.mem.set_u32(out, word);
}

// Translated from 00685e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 8`): stores `first` at +0xC and `second` at +8.
pub fn fn_00685e60(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.mem.set_u32(this.addr() + 0xc, first);
    e.mem.set_u32(this.addr() + 8, second);
}

// Translated from 00687d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, six words, `RET 0x18`): hands `00a855f0` the words
/// at +0x20 and +0x24 and the `u16` at +8 of `source` (`007af430`,
/// `0059bb30`, `004a8ae0`, in that argument order), then the five other
/// words.
pub fn fn_00687d40(e: &mut Engine, this: Ptr, source: u32, a: u32, b: u32, c: u32, d: u32, f: u32) {
    let count = e.call(WORD_AT_8_004A8AE0, &args![source]).u16();
    let second = e.call(WORD_AT_24_0059BB30, &args![source]).u32();
    let first = e.call(WORD_AT_20_007AF430, &args![source]).u32();
    e.call(
        SKIN_SETUP_00A855F0,
        &args![this, first, second, u32::from(count), a, b, c, d, f],
    );
}

// Translated from 00688470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSDismemberSkinInstance` constructor (unnamed in the engine map;
/// `thiscall`, five words, `RET 0x14`): the `NiSkinInstance` constructor
/// [`fn_006884c0`] with the first three words, the class's vtable, the
/// fifth word at +0x34, the fourth at +0x38, and 1 at +0x3C. Returns `this`.
pub fn fn_00688470(
    e: &mut Engine,
    this: Ptr,
    skin_data: u32,
    root: u32,
    bones: u32,
    partitions: u32,
    count: u32,
) -> Ptr {
    fn_006884c0(e, this, skin_data, root, bones);
    e.mem
        .set_u32(this.addr(), BS_DISMEMBER_SKIN_INSTANCE_VTABLE);
    e.mem.set_u32(this.addr() + 0x34, count);
    e.mem.set_u32(this.addr() + 0x38, partitions);
    e.mem.set_u8(this.addr() + 0x3c, 1);
    this
}

// Translated from 006884c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006884c0 - the compiler's exception frame (FS:[0] chain, the unwind states of the base and the first pointer) is not translated
/// The `NiSkinInstance` constructor (unnamed in the engine map; `thiscall`,
/// `RET 0xC`): `NiObject`'s constructor, the class's vtable, smart pointers
/// at +8 (with `skin_data`) and +0xC (with 0), `root` at +0x10, `bones` at
/// +0x14, -1 at +0x18 and 0 at +0x1C, +0x20, +0x24, +0x28 and +0x30.
/// Returns `this`.
pub fn fn_006884c0(e: &mut Engine, this: Ptr, skin_data: u32, root: u32, bones: u32) -> Ptr {
    let base = this.addr();
    e.call(NI_OBJECT_CONSTRUCTOR, &args![base]);
    e.mem.set_u32(base, NI_SKIN_INSTANCE_VTABLE);
    e.call(POINTER_CONSTRUCT_00633C90, &args![base + 8, skin_data]);
    e.call(POINTER_CONSTRUCT_00633C90, &args![base + 0xc, 0u32]);
    e.mem.set_u32(base + 0x10, root);
    e.mem.set_u32(base + 0x14, bones);
    e.mem.set_u32(base + 0x18, 0xffff_ffff);
    for offset in [0x1cu32, 0x20, 0x24, 0x28, 0x30] {
        e.mem.set_u32(base + offset, 0);
    }
    this
}

// Translated from 00688660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectGeometryDecal::Initialize` (engine map): nothing unless the
/// target geometry passes `0043fad0`. Unless the effect is already
/// initialized (`bInitialized`, +0x14), and when `00726070` answers above
/// 1 for both the parent and the target, the decal is built: skinned
/// (`00683c10`) when the target has a skin object ([`fn_00688730`]) whose
/// slot +0x8C answers non-zero, else `00685ef0`. Then
/// `BSTempEffect::Initialize`, and the target and parent pointers are
/// cleared.
pub fn bs_temp_effect_geometry_decal_initialize(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
    if e.call(GEOMETRY_TEST_0043FAD0, &args![target]).u32() == 0 {
        return;
    }
    if e.mem.u8(base + 0x14) == 0 {
        let parent = e.call(WORD_AT_0_00559450, &args![base + 0x30]).u32();
        if e.call(NODE_ANSWER_00726070, &args![parent]).u32() > 1 {
            let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
            if e.call(NODE_ANSWER_00726070, &args![target]).u32() > 1 {
                let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
                let skin = fn_00688730(e, Ptr::new(target));
                if skin != 0 && e.vcall(skin, SLOT_SKIN_TEST, &args![]).u8() != 0 {
                    fn_00683c10(e, this);
                } else {
                    fn_00685ef0(e, this);
                }
            }
        }
    }
    e.call(TEMP_EFFECT_INITIALIZE, &args![this]);
    e.call(POINTER_SET_0066B0D0, &args![base + 0x2c, 0u32]);
    e.call(POINTER_SET_0066B0D0, &args![base + 0x30, 0u32]);
}

// Translated from 00688730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the skin object of a geometry: when its pointer at +0xB8 holds
/// an object, that object's +0x30 object (`004a8a90`), else 0.
pub fn fn_00688730(e: &mut Engine, this: Ptr) -> u32 {
    if e.call(WORD_AT_0_00559450, &args![this.addr() + 0xb8]).u32() == 0 {
        return 0;
    }
    let held = e.call(WORD_AT_0_00559450, &args![this.addr() + 0xb8]).u32();
    e.call(POINTER_AT_30_004A8A90, &args![held]).u32()
}

// Translated from 00688770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00688770 - the compiler's exception frame (FS:[0] chain, the unwind states of the four smart pointers) is not translated
/// The `BSTempEffectGeometryDecal` destructor body (unnamed in the engine
/// map): the class's vtable; a decal geometry is detached from its parent
/// (slot +0xE8, when it has one) and the pointer cleared; `pOldIndex` is
/// freed when set; the target and parent pointers are cleared; the four
/// smart pointers destroyed (+0x34, +0x30, +0x2C, +0x1C); then the
/// `BSTempEffect` destructor body.
pub fn fn_00688770(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, GEOMETRY_DECAL_VTABLE);
    if e.call(WORD_AT_0_00559450, &args![base + 0x1c]).u32() != 0 {
        let geometry = e.call(WORD_AT_0_00559450, &args![base + 0x1c]).u32();
        if e.call(PARENT_009611E0, &args![geometry]).u32() != 0 {
            let geometry = e.call(WORD_AT_0_00559450, &args![base + 0x1c]).u32();
            let parent = e.call(PARENT_009611E0, &args![geometry]).u32();
            let geometry = e.call(WORD_AT_0_00559450, &args![base + 0x1c]).u32();
            e.vcall(parent, SLOT_DETACH_CHILD, &args![geometry]);
        }
        e.call(POINTER_SET_0066B0D0, &args![base + 0x1c, 0u32]);
    }
    let old_index = e.mem.u32(base + 0x24);
    if old_index != 0 {
        e.call(OPERATOR_DELETE, &args![old_index]);
    }
    e.call(POINTER_SET_0066B0D0, &args![base + 0x2c, 0u32]);
    e.call(POINTER_SET_0066B0D0, &args![base + 0x30, 0u32]);
    for offset in [0x34u32, 0x30, 0x2c, 0x1c] {
        e.call(POINTER_DESTRUCT_0045CEC0, &args![base + offset]);
    }
    e.call(TEMP_EFFECT_DESTRUCTOR, &args![this]);
}

// Translated from 006888a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectGeometryDecal::Update` (engine map; `RET 4`, the elapsed
/// time): false once `bFinished`. Otherwise `BSTempEffect::Update` gives
/// the answer; the decal geometry's (slot +0x98) property 3 gets
/// `fAge / fLifetime` at +0xBC. When the lifetime is below the age
/// (ordered), the decal geometry is detached from the decal parent, the
/// decal parent's parent (if any) gets `004ee4f0(this)`, and, when there
/// is such a parent and `00453470` answers 0 for the decal parent, the
/// decal parent is detached from it and the count `011c57f0` drops;
/// `bFinished` is set.
pub fn bs_temp_effect_geometry_decal_update(e: &mut Engine, this: Ptr, seconds: f32) -> bool {
    let base = this.addr();
    if e.mem.u8(base + 0x28) != 0 {
        return false;
    }
    let alive = e
        .call(
            TEMP_EFFECT_UPDATE,
            &args![this, x87_quiet_bits(seconds.to_bits())],
        )
        .bool();
    let geometry = e.vcall(base, SLOT_DECAL_GEOMETRY, &args![]).u32();
    let property = e.call(GET_PROPERTY, &args![geometry, 3u32]).u32();
    let age = e.mem.u32(base + 0x10);
    let lifetime = e.mem.u32(base + 8);
    e.mem.set_u32(
        property.wrapping_add(0xbc),
        x87_store32(x87_div(x87_load32(age), x87_m32(lifetime))),
    );
    let age = x87_load32(e.mem.u32(base + 0x10));
    let lifetime = x87_load32(e.mem.u32(base + 8));
    if !(lifetime < age) {
        return alive;
    }
    let mut parent = 0;
    if e.call(WORD_AT_0_00559450, &args![base + 0x34]).u32() != 0 {
        let decal_parent = e.call(WORD_AT_0_00559450, &args![base + 0x34]).u32();
        let geometry = e.call(WORD_AT_0_00559450, &args![base + 0x1c]).u32();
        e.vcall(decal_parent, SLOT_DETACH_CHILD, &args![geometry]);
        let decal_parent = e.call(WORD_AT_0_00559450, &args![base + 0x34]).u32();
        if e.call(PARENT_009611E0, &args![decal_parent]).u32() != 0 {
            let decal_parent = e.call(WORD_AT_0_00559450, &args![base + 0x34]).u32();
            parent = e.call(PARENT_009611E0, &args![decal_parent]).u32();
            e.call(PARENT_NOTIFY_004EE4F0, &args![parent, this]);
        }
    }
    if parent != 0 {
        let decal_parent = e.call(WORD_AT_0_00559450, &args![base + 0x34]).u32();
        if e.call(NODE_TEST_00453470, &args![decal_parent]).u32() == 0 {
            let decal_parent = e.call(WORD_AT_0_00559450, &args![base + 0x34]).u32();
            let grand_parent = e.call(PARENT_009611E0, &args![decal_parent]).u32();
            let decal_parent = e.call(WORD_AT_0_00559450, &args![base + 0x34]).u32();
            e.vcall(grand_parent, SLOT_DETACH_CHILD, &args![decal_parent]);
            let count = e.global::<u32>(DECAL_COUNT_011C57F0).wrapping_sub(1);
            e.set_global(DECAL_COUNT_011C57F0, count);
        }
    }
    e.mem.set_u8(base + 0x28, 1);
    alive
}

/// The report flag (`00408d60` on `011de4e8`, a byte at the address it
/// returns); shared with the other temp effects' save code.
pub(crate) fn report_flag(e: &mut Engine) -> bool {
    let at = e
        .call(REPORT_FLAG_00408D60, &args![REPORT_FLAG_OWNER])
        .u32();
    e.mem.u8(at) != 0
}

/// The save reports' tail (`Error`): with a record of the form being saved
/// (`004fd3e0`), its id, text, flags, the line and the file; without, only
/// the line and the file.
pub(crate) fn save_report(
    e: &mut Engine,
    with_form: u32,
    plain: u32,
    size: u32,
    line: u32,
    file: u32,
) {
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    let record = e.call(SAVING_RECORD_004FD3E0, &args![game]).u32();
    if record != 0 {
        let id = e.mem.u32(record);
        let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
        let flags = e.mem.u32(record + 5);
        let text = e.vcall(form, SLOT_FORM_TEXT, &args![]).u32();
        let id = e.mem.u32(record);
        e.call(
            ERROR_REPORT,
            &args![with_form, size, id, text, flags, line, file],
        );
    } else {
        e.call(ERROR_REPORT, &args![plain, size, line, file]);
    }
}

/// `TESSaveLoadGame::UseSaveGameBlocks` on the singleton.
pub(crate) fn uses_blocks(e: &mut Engine) -> bool {
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(USE_SAVE_GAME_BLOCKS, &args![game]).bool()
}

/// The save buffer's position (`00825c00` on the singleton).
pub(crate) fn buffer_position(e: &mut Engine) -> u32 {
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(BUFFER_POSITION_00825C00, &args![game]).u32()
}

/// `008579b0` / `008579e0` on the singleton.
pub(crate) fn save_bytes(e: &mut Engine, data: u32, size: u32) {
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(SAVE_DATA_008579B0, &args![game, data, size]);
}
pub(crate) fn load_bytes(e: &mut Engine, data: u32, size: u32) {
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(LOAD_DATA_008579E0, &args![game, data, size]);
}

/// The start of a saved block: the `'KOLB'` header (from `tag_local`) and
/// the 2-byte size placeholder (`size_local`); returns the placeholder's
/// buffer position.
pub(crate) fn save_block_begin(e: &mut Engine, tag_local: u32, size_local: u32) -> u32 {
    e.mem.set_u32(tag_local, BLOCK_TAG);
    save_bytes(e, tag_local, 4);
    let placeholder = buffer_position(e);
    save_bytes(e, size_local, 2);
    placeholder
}

/// The end of a saved block: a warning (`line`, `file`) when more than
/// 0xFFFF bytes follow the placeholder, then the placeholder becomes the
/// difference (as a `u16`).
pub(crate) fn save_block_end(e: &mut Engine, placeholder: u32, line: u32, file: u32) {
    let now = buffer_position(e);
    if now > placeholder.wrapping_add(0xffff) {
        e.call(PRINT_WARNING_005B5E40, &args![BLOCK_TOO_LARGE, file, line]);
    }
    e.mem
        .set_u16(placeholder, now.wrapping_sub(placeholder) as u16);
}

/// The wrong-header warning of a loaded block: with a record of the form
/// being loaded (`004fd3c0`), its id, text, version byte and flags;
/// without, the current version byte.
pub(crate) fn load_bad_header(e: &mut Engine, line: u32, file: u32) {
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    let record = e.call(LOADING_RECORD_004FD3C0, &args![game]).u32();
    if record != 0 {
        let id = e.mem.u32(record);
        let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
        let flags = e.mem.u32(record + 5);
        let version = u32::from(e.mem.u8(record + 9));
        let text = e.vcall(form, SLOT_FORM_TEXT, &args![]).u32();
        let id = e.mem.u32(record);
        e.call(
            PRINT_WARNING_005B5E40,
            &args![BAD_HEADER_FORM, file, line, id, text, version, flags],
        );
    } else {
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        let version = u32::from(e.call(VERSION_008DF040, &args![game]).u8());
        e.call(
            PRINT_WARNING_005B5E40,
            &args![BAD_HEADER, file, line, version],
        );
    }
}

/// The end of a loaded block: the position now against the kept `start`
/// plus the block's `size`; an overrun (now past the end) or underrun (now
/// short of it) warns with that many bytes. The record of the form being
/// loaded and its form are fetched before the comparison.
pub(crate) fn load_block_end(e: &mut Engine, start: u32, size: u16, line: u32, file: u32) {
    let now = buffer_position(e);
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    let record = e.call(LOADING_RECORD_004FD3C0, &args![game]).u32();
    let end = u32::from(size).wrapping_add(start);
    let (format, difference) = if now > end {
        (
            if record != 0 { OVERRUN_FORM } else { OVERRUN },
            now.wrapping_sub(end),
        )
    } else {
        (
            if record != 0 { UNDERRUN_FORM } else { UNDERRUN },
            end.wrapping_sub(now),
        )
    };
    if record != 0 {
        let id = e.mem.u32(record);
        let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
        if now != end {
            let flags = e.mem.u32(record + 5);
            let version = u32::from(e.mem.u8(record + 9));
            let text = e.vcall(form, SLOT_FORM_TEXT, &args![]).u32();
            let id = e.mem.u32(record);
            e.call(
                PRINT_WARNING_005B5E40,
                &args![format, difference, file, line, id, text, version, flags],
            );
        }
    } else if now != end {
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        let version = u32::from(e.call(VERSION_008DF040, &args![game]).u8());
        e.call(
            PRINT_WARNING_005B5E40,
            &args![format, difference, file, line, version],
        );
    }
}

// Translated from 00688a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectGeometryDecal::GetSaveSize` (engine map): 6 (a 4-byte
/// block header and a 2-byte block size) when the save uses blocks, else
/// 0, as a `u16`. With the report flag set it reports the size (`Error`,
/// line 0x72F) with the record of the form being saved when there is one.
pub fn bs_temp_effect_geometry_decal_get_save_size(e: &mut Engine, _this: Ptr) -> u16 {
    let mut size: u16 = 0;
    let start = size;
    if uses_blocks(e) {
        size += 4;
        size += 2;
    }
    if report_flag(e) {
        let difference = (i32::from(size) - i32::from(start)) as u32;
        save_report(
            e,
            SIZE_REPORT_FORM,
            SIZE_REPORT,
            difference,
            0x72f,
            SOURCE_FILE_NAME,
        );
    }
    size
}

// Translated from 00688b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectGeometryDecal::SaveGame` (engine map): with the save using
/// blocks, writes the block header `'KOLB'` and a 2-byte size placeholder
/// (its buffer position kept); with the report flag set, reports the bytes
/// written since the start (line 0x783); with blocks, when more than
/// 0xFFFF bytes follow the placeholder it warns, and the placeholder
/// becomes the difference (as a `u16`). The decal writes nothing else.
pub fn bs_temp_effect_geometry_decal_save_game(e: &mut Engine, _this: Ptr) {
    e.with_stack(8, |e, locals| {
        // The 2-byte size (EBP-8) and the header word (EBP-0x10).
        let size_local = locals.addr();
        let tag_local = locals.addr() + 4;
        let mut placeholder: u32 = 0;
        let mut start = buffer_position(e);
        if report_flag(e) {
            start = buffer_position(e);
        }
        if uses_blocks(e) {
            placeholder = save_block_begin(e, tag_local, size_local);
        }
        if report_flag(e) {
            let end = buffer_position(e);
            save_report(
                e,
                SAVE_REPORT_FORM,
                SAVE_REPORT,
                end.wrapping_sub(start),
                0x783,
                SOURCE_FILE_NAME,
            );
        }
        if uses_blocks(e) {
            save_block_end(e, placeholder, 0x783, SOURCE_FILE_NAME);
        }
    });
}

// Translated from 00688c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectGeometryDecal::LoadGame` (engine map): with the save using
/// blocks, reads the 4-byte block header (warning, line 0x78C, when it is
/// not `'KOLB'`), keeps the buffer position and reads the 2-byte block
/// size; with blocks, compares the position now with the kept position
/// plus the size and warns of an overrun (now past the end) or an underrun
/// (now short of it) by that many bytes (line 0x838). The record of the
/// form being loaded (`004fd3c0`) is looked up anew for each part, as the
/// exe does. Always true.
pub fn bs_temp_effect_geometry_decal_load_game(e: &mut Engine, _this: Ptr) -> bool {
    e.with_stack(8, |e, locals| {
        let size_local = locals.addr();
        let tag_local = locals.addr() + 4;
        let mut start: u32 = 0;
        if uses_blocks(e) {
            load_bytes(e, tag_local, 4);
            if e.mem.u32(tag_local) != BLOCK_TAG {
                load_bad_header(e, 0x78c, SOURCE_FILE_NAME);
            }
            start = buffer_position(e);
            load_bytes(e, size_local, 2);
        }
        if uses_blocks(e) {
            let size = e.mem.u16(size_local);
            load_block_end(e, start, size, 0x838, SOURCE_FILE_NAME);
        }
    });
    true
}

/// The calls [`fn_00687d90`] makes, by address (none is in this unit but
/// [`fn_00688470`] and [`fn_00688430`]).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const MEMORY_ALLOCATE: u32 = 0x0040_1000;
const NI_TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4410;
const SET_NAME_00A5B950: u32 = 0x00a5_b950;
const SHADER_PROPERTY_CONSTRUCT_00BB4080: u32 = 0x00bb_4080;
const ATTACH_PROPERTY_00439410: u32 = 0x0043_9410;
const PROPERTY_SET_FLOAT_004EE4B0: u32 = 0x004e_e4b0;
const TEXTURE_SET_SLOT_90: u32 = 0x90;
const IS_KIND_0043B300: u32 = 0x0043_b300;
const KIND_011F444C: u32 = 0x011f_444c;
const SKIN_DATA_OF_005585E0: u32 = 0x0055_85e0;
const BONE_COUNT_008041A0: u32 = 0x0080_41a0;
const ROOT_0048D150: u32 = 0x0048_d150;
const SKIN_DATA_CONSTRUCT_00A854C0: u32 = 0x00a8_54c0;
const BONES_00825C00: u32 = 0x0082_5c00;
const LIST_COUNT_0044DDC0: u32 = 0x0044_ddc0;
const SKIN_ROOT_0044EDB0: u32 = 0x0044_edb0;
const SET_PARTITION_004ADDC0: u32 = 0x004a_ddc0;
const SET_SKIN_INSTANCE_004ADDE0: u32 = 0x004a_dde0;
const PARTITION_CONSTRUCT_00A81F90: u32 = 0x00a8_1f90;
const PARTITION_MAKE_00A84980: u32 = 0x00a8_4980;
const GEOMETRY_DATA_005495F0: u32 = 0x0054_95f0;
const SET_TRANSLATE_00440460: u32 = 0x0044_0460;
const SET_ROTATE_0043FA80: u32 = 0x0043_fa80;
const SET_SCALE_00440490: u32 = 0x0044_0490;
const TRANSLATION_0043C490: u32 = 0x0043_c490;
const ROTATION_006A9540: u32 = 0x006a_9540;
const SCALE_009CDAE0: u32 = 0x009c_dae0;
const WORLD_POSITION_0045BB80: u32 = 0x0045_bb80;
const SET_WORLD_POSITION_004F00E0: u32 = 0x004f_00e0;
const WORLD_TRANSFORM_00461130: u32 = 0x0046_1130;
const VALUE_008D01E0: u32 = 0x008d_01e0;
const SET_FLOAT_006427A0: u32 = 0x0064_27a0;
const SHAPE_SLOT_BC: u32 = 0xbc;
const UPDATE_PROPERTIES_00A5A040: u32 = 0x00a5_a040;
const PREPARE_OBJECT: u32 = 0x00b5_7e30;

// Translated from 00687d90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00687d90 - the compiler's exception frame (FS:[0] chain, the unwind states of the allocations and the texture pointer) is not translated
/// Unnamed (`thiscall`, nine words, `RET 0x24`): builds the skinned decal
/// geometry. A new `NiTriShape` (0xC4 bytes; `vertex_count`, `vertices`,
/// `normals`, four zeros, `index_count / 3` triangles, `triangles`) named
/// with the word `00688460` answers, gets a new shader property (0xD4
/// bytes, `00bb4080`; `004ee4b0(1.0)`) whose +0x7C pointer takes the
/// texture set's (+0x58) texture (slot +0x90 of its +0x30 object with 0)
/// when `0043b300` accepts it. The source's skin (`0043fad0`) gives the
/// bone count (`008041a0` of its `005585e0` object); new skin data (0x48
/// bytes, `00a854c0` with the count, `bone_data`, `0048d150` of that
/// object and the vertices) and a copy of the bone array (`00825c00`). The
/// partitions are the `list`'s count, or one per set bit 1..14 of
/// `iBodyParts` (+0x60), each `{1, 1, part}` with the part from `parts`
/// when given, else the bit number. A `BSDismemberSkinInstance` (0x40
/// bytes) is made of them; with `list` it takes the list as its partition,
/// otherwise a new skin partition (0x10 bytes) made from the shape's data
/// (`00a84980`, 0x12, 4, 1); when that fails both are deleted (slot 0 with
/// 1) and the shape keeps no skin. The shape copies the target's
/// translation, rotation, scale, world position, world transform and the
/// `008d01e0` value, runs slot +0xBC, updates its properties and is
/// prepared (`BSShaderManager::PrepareObject` with 0, 0); it becomes
/// `spDecalGeometry` and `pGeometry` the target.
#[allow(clippy::too_many_arguments)]
pub fn fn_00687d90(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    vertex_count: u16,
    index_count: u16,
    vertices: u32,
    normals: u32,
    triangles: u32,
    bone_data: u32,
    list: u32,
    parts: u32,
) {
    let base = this.addr();
    // The frame: the texture pointer (EBP-0x6C) and the bit list (EBP-0x50, 15 words).
    e.with_stack(0x40, |e, bits| {
        e.with_stack(4, |e, texture_pointer| {
            let block = e.call(NI_ALLOCATE, &args![0xc4u32]).u32();
            let shape = if block != 0 {
                e.call(
                    NI_TRI_SHAPE_CONSTRUCT,
                    &args![
                        block,
                        u32::from(vertex_count),
                        vertices,
                        normals,
                        0u32,
                        0u32,
                        0u32,
                        0u32,
                        u32::from(index_count) / 3,
                        triangles
                    ],
                )
                .u32()
            } else {
                0
            };
            let name = fn_00688460(e);
            e.call(SET_NAME_00A5B950, &args![shape, name]);
            let block = e.call(NI_ALLOCATE, &args![0xd4u32]).u32();
            let property = if block != 0 {
                e.call(SHADER_PROPERTY_CONSTRUCT_00BB4080, &args![block])
                    .u32()
            } else {
                0
            };
            e.call(ATTACH_PROPERTY_00439410, &args![shape, property]);
            e.call(
                PROPERTY_SET_FLOAT_004EE4B0,
                &args![property, x87_store32(1.0)],
            );
            e.call(POINTER_CONSTRUCT_00633C90, &args![texture_pointer, 0u32]);
            let texture_set = e.mem.u32(base + 0x58);
            e.vcall(
                texture_set.wrapping_add(0x30),
                TEXTURE_SET_SLOT_90,
                &args![0u32, texture_pointer],
            );
            let texture = e.call(WORD_AT_0_00559450, &args![texture_pointer]).u32();
            if e.call(IS_KIND_0043B300, &args![KIND_011F444C, texture])
                .u8()
                != 0
            {
                e.call(POINTER_SET_0066B0D0, &args![property + 0x7c, texture]);
            }
            let skin = e.call(GEOMETRY_TEST_0043FAD0, &args![source]).u32();
            let skin_object = e.call(SKIN_DATA_OF_005585E0, &args![skin]).u32();
            let bone_count = e.call(BONE_COUNT_008041A0, &args![skin_object]).u32();
            let block = e.call(NI_ALLOCATE, &args![0x48u32]).u32();
            let skin_data = if block != 0 {
                let skin_object = e.call(SKIN_DATA_OF_005585E0, &args![skin]).u32();
                let root = e.call(ROOT_0048D150, &args![skin_object]).u32();
                e.call(
                    SKIN_DATA_CONSTRUCT_00A854C0,
                    &args![block, bone_count, bone_data, root, vertices],
                )
                .u32()
            } else {
                0
            };
            let bones_source = e.call(BONES_00825C00, &args![skin]).u32();
            let bones = e
                .call(
                    MEMORY_ALLOCATE,
                    &args![bone_count.checked_mul(4).unwrap_or(u32::MAX)],
                )
                .u32();
            let mut i: i32 = 0;
            while i < bone_count as i32 {
                let word = e.mem.u32(bones_source.wrapping_add((i as u32) * 4));
                e.mem.set_u32(bones.wrapping_add((i as u32) * 4), word);
                i += 1;
            }
            let mut count: u32 = 0;
            if list != 0 {
                count = e.call(LIST_COUNT_0044DDC0, &args![list]).u32();
            } else {
                let mut bit: u32 = 2;
                for number in 0..0xfu32 {
                    if e.mem.u32(base + 0x60) & bit != 0 {
                        e.mem
                            .set_u32(bits.addr().wrapping_add(count.wrapping_mul(4)), number);
                        count = count.wrapping_add(1);
                    }
                    bit <<= 1;
                }
            }
            let partitions = e
                .call(
                    MEMORY_ALLOCATE,
                    &args![count.checked_mul(4).unwrap_or(u32::MAX)],
                )
                .u32();
            let mut i: i32 = 0;
            while i < count as i32 {
                let entry = partitions.wrapping_add((i as u32) * 4);
                e.mem.set_u8(entry, 1);
                e.mem.set_u8(entry + 1, 1);
                let part = if parts != 0 {
                    e.mem.u16(parts.wrapping_add((i as u32) * 2))
                } else {
                    e.mem.u16(bits.addr().wrapping_add((i as u32) * 4))
                };
                e.mem.set_u16(entry + 2, part);
                i += 1;
            }
            let block = e.call(NI_ALLOCATE, &args![0x40u32]).u32();
            let instance = if block != 0 {
                let root = e.call(SKIN_ROOT_0044EDB0, &args![skin]).u32();
                fn_00688470(
                    e,
                    Ptr::new(block),
                    skin_data,
                    root,
                    bones,
                    partitions,
                    count,
                )
                .addr()
            } else {
                0
            };
            if list != 0 {
                e.call(SET_PARTITION_004ADDC0, &args![instance, list]);
                e.call(SET_SKIN_INSTANCE_004ADDE0, &args![shape, instance]);
            } else {
                let block = e.call(NI_ALLOCATE, &args![0x10u32]).u32();
                let partition = if block != 0 {
                    e.call(PARTITION_CONSTRUCT_00A81F90, &args![block]).u32()
                } else {
                    0
                };
                let data = e.call(GEOMETRY_DATA_005495F0, &args![shape]).u32();
                if e.call(
                    PARTITION_MAKE_00A84980,
                    &args![partition, data, skin_data, 0x12u32, 4u32, 1u32, 0u32, 0u32, 0u32],
                )
                .u8()
                    != 0
                {
                    e.call(SET_PARTITION_004ADDC0, &args![instance, partition]);
                    e.call(SET_SKIN_INSTANCE_004ADDE0, &args![shape, instance]);
                } else {
                    if partition != 0 {
                        e.vcall(partition, 0, &args![1u32]);
                    }
                    if instance != 0 {
                        e.vcall(instance, 0, &args![1u32]);
                    }
                }
            }
            let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
            let translation = e.call(TRANSLATION_0043C490, &args![target]).u32();
            e.call(SET_TRANSLATE_00440460, &args![shape, translation]);
            let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
            let rotation = e.call(ROTATION_006A9540, &args![target]).u32();
            e.call(SET_ROTATE_0043FA80, &args![shape, rotation]);
            let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
            let scale = e.call(SCALE_009CDAE0, &args![target]).f64();
            e.call(SET_SCALE_00440490, &args![shape, x87_store32(scale)]);
            let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
            let position = e.call(WORLD_POSITION_0045BB80, &args![target]).u32();
            e.call(SET_WORLD_POSITION_004F00E0, &args![shape, position]);
            let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
            let transform = e.call(WORLD_TRANSFORM_00461130, &args![target]).u32();
            fn_00688430(e, Ptr::new(shape), transform);
            let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
            let value = e.call(VALUE_008D01E0, &args![target]).f64();
            e.call(SET_FLOAT_006427A0, &args![shape, x87_store32(value)]);
            e.vcall(shape, SHAPE_SLOT_BC, &args![]);
            e.call(UPDATE_PROPERTIES_00A5A040, &args![shape]);
            e.call(PREPARE_OBJECT, &args![shape, 0u32, 0u32]);
            e.call(POINTER_SET_0066B0D0, &args![base + 0x1c, shape]);
            let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
            e.mem.set_u32(base + 0x20, target);
            e.call(POINTER_DESTRUCT_0045CEC0, &args![texture_pointer]);
        });
    });
}

/// The calls of the decal builds ([`fn_00685ef0`], [`fn_00683c10`]) by address.
const IS_KIND_0045BAD0: u32 = 0x0045_bad0;
const DISMEMBER_SKIN_INSTANCE_RTTI: u32 = 0x011f_49d8;
const FIRST_CONTROLLER_0043B230: u32 = 0x0043_b230;
const MATRIX_INVERSE_004B45B0: u32 = 0x004b_45b0;
const MATRIX_TIMES_POINT_004B4500: u32 = 0x004b_4500;
const MATRIX_ANGLE_004A0C90: u32 = 0x004a_0c90;
const ROTATION_Z_TO_VECTOR: u32 = 0x00c4_b600;
const MATRIX_MULTIPLY_0043F8D0: u32 = 0x0043_f8d0;
const SET_TRANSLATION_004578C0: u32 = 0x0045_78c0;
const BONE_DATA_008D6F30: u32 = 0x008d_6f30;
const PARTITION_FLAGS_009E32D0: u32 = 0x009e_32d0;
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
const FILL_MEMORY_00403D30: u32 = 0x0040_3d30;
const EH_VECTOR_CONSTRUCT_00EC782F: u32 = 0x00ec_782f;
const VECTOR_CONSTRUCT_00401050: u32 = 0x0040_1050;
const PARTITION_CONSTRUCT_00A81BA0: u32 = 0x00a8_1ba0;
const PARTITION_DESTRUCT_00A81BD0: u32 = 0x00a8_1bd0;
const BONE_CONSTRUCT_00A84C90: u32 = 0x00a8_4c90;
const WEIGHT_CONSTRUCT_00A84C60: u32 = 0x00a8_4c60;
const POINT_TIMES_FLOAT_00439180: u32 = 0x0043_9180;
const TRANSFORM_VERTICES: u32 = 0x00a5_82f0;
const POINT_ZERO: u32 = 0x011f_426c;
const HALF_DOUBLE: u32 = 0x0101_1588;
const TWO_FIFTY_SIX: u32 = 0x0102_31d8;
const MAX_00404010: u32 = 0x0040_4010;
const ABS_00408840: u32 = 0x0040_8840;
const NI_ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const COPY_MEMORY_00401460: u32 = 0x0040_1460;
const DEBUG_BREAK: u32 = 0x00fd_f0c8;
const VERTEX_COUNT_004A8AE0: u32 = 0x004a_8ae0;

/// `MemoryManager` allocation of `count * size` bytes with the compiler's
/// overflow check (0xFFFFFFFF on overflow).
fn allocate_array(e: &mut Engine, count: u32, size: u32) -> u32 {
    let bytes = count.checked_mul(size).unwrap_or(u32::MAX);
    e.call(MEMORY_ALLOCATE, &args![bytes]).u32()
}

/// An array of `count` 12-byte points, each run through `006815c0`
/// (`00401050`), or null when the allocation fails.
fn allocate_points(e: &mut Engine, count: u32) -> u32 {
    let block = allocate_array(e, count, 0xc);
    if block == 0 {
        return 0;
    }
    e.call(
        VECTOR_CONSTRUCT_00401050,
        &args![block, 0xcu32, count, LOCAL_INIT_006815C0],
    );
    block
}

/// An array of `count` 8-byte bone weights (`00a84c60` on each through
/// `00401050`), or null when the allocation fails.
fn allocate_weights(e: &mut Engine, count: u32) -> u32 {
    let block = allocate_array(e, count, 8);
    if block == 0 {
        return 0;
    }
    e.call(
        VECTOR_CONSTRUCT_00401050,
        &args![block, 8u32, count, WEIGHT_CONSTRUCT_00A84C60],
    );
    block
}

/// A partition's body part number as the builds reduce it: above 1000 the
/// thousands, above 100 the part minus hundreds until at most 100.
fn reduce_body_part(mut part: u16) -> u16 {
    if part > 1000 {
        part /= 1000;
    } else if part > 100 {
        while part > 100 {
            part -= 100;
        }
    }
    part
}

/// The three words of a 12-byte point.
fn copy_point(e: &mut Engine, from: u32, to: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 00685ef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00685ef0 - the compiler's exception frame (FS:[0] chain, the unwind states of the array allocations) is not translated
/// Unnamed (the dismembered decal's build, `thiscall`): nothing without
/// `iBodyParts` (+0x60), without a `BSDismemberSkinInstance` skin on the
/// target (`0045bad0` with its RTTI `011f49d8`), without bone data
/// (`008d6f30`, its +0x44) or when no partition is both enabled and of a
/// body part in `iBodyParts` (bit `part + 1`, the part reduced below 100).
/// Otherwise the target's vertices are deformed by the skin
/// ([`fn_00687d40`]) and projected into the decal's space (through the
/// target's inverse world rotation, the decal's rotation and its vector's
/// rotation): each vertex gets `(x / fScale + 0.5, y / fScale + 0.5,
/// (1 - max(0, normal . vector))^10 * (1 - |z| / 256))`. Every triangle
/// of each enabled partition that [`fn_00683a70`] keeps is copied, with
/// new vertex numbers (the global remap and each partition's own), its
/// bone weights and indices; strips alternate their winding. The kept
/// partitions become a new skin partition (`00a81f90`, [`fn_00685e60`]),
/// and, with kept vertices, the decal geometry is built
/// ([`fn_00687d90`]) from the kept positions, the projected values, the
/// triangles and the bones' weights of the kept vertices, and
/// `iNumVerts` (+0x5C) is their count; without kept vertices the bones'
/// weight arrays are freed. The work arrays are freed.
pub fn fn_00685ef0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x60) == 0 {
        return;
    }
    // The frame below EBP: `local(n)` is EBP - n.
    e.with_stack(0x3c0, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x3c0 - offset;
        let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
        let skin = e.call(GEOMETRY_TEST_0043FAD0, &args![target]).u32();
        if e.call(IS_KIND_0045BAD0, &args![DISMEMBER_SKIN_INSTANCE_RTTI, skin])
            .u8()
            == 0
        {
            return;
        }
        let dismember = skin;
        let controller = e.call(FIRST_CONTROLLER_0043B230, &args![skin]).u32();
        let part_count = e.call(LIST_COUNT_0044DDC0, &args![controller]).u32();
        // The decal's projection: m1 (EBP-0x54) from the vector's rotation
        // and the decal's rotation, the target's inverse rotation
        // (EBP-0x17C), the vector in the target's space (EBP-0xC4), the
        // target's transform with the origin as translation (EBP-0x144).
        let projection = local(0x54);
        e.call(LOCAL_INIT_006815C0, &args![projection]);
        let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
        let world = e.call(WORLD_TRANSFORM_00461130, &args![target]).u32();
        for i in 0..9 {
            let word = e.mem.u32(world + 4 * i);
            e.mem.set_u32(local(0xfc) + 4 * i, word);
        }
        e.call(MATRIX_INVERSE_004B45B0, &args![local(0xfc), local(0x17c)]);
        let vector = local(0xc4);
        e.call(LOCAL_INIT_006815C0, &args![vector]);
        let moved = e
            .call(
                MATRIX_TIMES_POINT_004B4500,
                &args![local(0x17c), local(0x264), base + 0x44],
            )
            .u32();
        copy_point(e, moved, vector);
        let spin = local(0x80);
        e.call(LOCAL_INIT_006815C0, &args![spin]);
        let rotation = x87_quiet_bits(e.mem.u32(base + 0x54));
        e.call(MATRIX_ANGLE_004A0C90, &args![spin, rotation]);
        let facing = e
            .call(
                ROTATION_Z_TO_VECTOR,
                &args![
                    local(0x288),
                    e.mem.u32(base + 0x44),
                    e.mem.u32(base + 0x48),
                    e.mem.u32(base + 0x4c)
                ],
            )
            .u32();
        let product = e
            .call(MATRIX_MULTIPLY_0043F8D0, &args![spin, local(0x2ac), facing])
            .u32();
        for i in 0..9 {
            let word = e.mem.u32(product + 4 * i);
            e.mem.set_u32(projection + 4 * i, word);
        }
        let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
        let world = e.call(WORLD_TRANSFORM_00461130, &args![target]).u32();
        let transform = local(0x144);
        for i in 0..13 {
            let word = e.mem.u32(world + 4 * i);
            e.mem.set_u32(transform + 4 * i, word);
        }
        e.call(
            SET_TRANSLATION_004578C0,
            &args![transform + 0x24, base + 0x38],
        );
        let skin_object = e.call(SKIN_DATA_OF_005585E0, &args![skin]).u32();
        let source_bones = e.call(BONE_DATA_008D6F30, &args![skin_object]).u32();
        if source_bones == 0 || e.mem.u32(source_bones + 0x44) == 0 {
            return;
        }
        let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
        let data = e.call(GEOMETRY_DATA_005495F0, &args![target]).u32();
        let vertex_count = u32::from(e.call(VERTEX_COUNT_004A8AE0, &args![data]).u16());
        let mut kept_triangles: u16 = 0;
        let mut any = false;
        let mut used: u32 = 0;
        let controller = e.call(FIRST_CONTROLLER_0043B230, &args![skin]).u32();
        let parts = e.call(FORM_ID_0084E3A0, &args![controller]).u32();
        // The exe computes the first partition's strip flag here; it is
        // set again for each partition before it is read.
        let _ = e.mem.u16(parts + 0x22);
        let mut index: i32 = 0;
        while index < part_count as i32 {
            let i = index as u32;
            let flags = e.call(PARTITION_FLAGS_009E32D0, &args![dismember]).u32();
            let enabled = e.mem.u8(flags + i * 4);
            let flags = e.call(PARTITION_FLAGS_009E32D0, &args![dismember]).u32();
            let part = reduce_body_part(e.mem.u16(flags + i * 4 + 2));
            if enabled != 0
                && (1u32 << ((u32::from(part) + 1) & 0x1f)) & e.mem.u32(base + 0x60) != 0
            {
                any = true;
                kept_triangles = kept_triangles.wrapping_add(e.mem.u16(parts + i * 0x2c + 0x1e));
                used += 1;
            }
            index += 1;
        }
        if !any {
            return;
        }
        let index_total = u32::from(kept_triangles) * 3;
        let out_triangles = allocate_array(e, index_total, 2);
        let remap = allocate_array(e, vertex_count, 4);
        let part_triangles = allocate_array(e, index_total, 2);
        let part_vertex_map = allocate_array(e, vertex_count, 2);
        let part_remap = allocate_array(e, vertex_count, 4);
        let part_limit = part_count;
        let part_list = allocate_array(e, part_count, 2);
        let weights = allocate_array(e, vertex_count << 2, 4);
        let bone_indices = e.call(MEMORY_ALLOCATE, &args![vertex_count << 2]).u32();
        e.call(
            FILL_MEMORY_00403D30,
            &args![remap, 0xffff_ffffu32, vertex_count << 2],
        );
        // The new partitions: a count word, then `used` 0x2C-byte partitions.
        let bytes = used
            .checked_mul(0x2c)
            .unwrap_or(u32::MAX)
            .checked_add(4)
            .unwrap_or(u32::MAX);
        let block = e.call(MEMORY_ALLOCATE, &args![bytes]).u32();
        let partitions = if block != 0 {
            e.mem.set_u32(block, used);
            e.call(
                EH_VECTOR_CONSTRUCT_00EC782F,
                &args![
                    block + 4,
                    0x2cu32,
                    used,
                    PARTITION_CONSTRUCT_00A81BA0,
                    PARTITION_DESTRUCT_00A81BD0
                ],
            );
            block + 4
        } else {
            0
        };
        let projected = allocate_points(e, vertex_count);
        let positions = allocate_points(e, vertex_count);
        let normals = allocate_points(e, vertex_count);
        let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
        let data = e.call(GEOMETRY_DATA_005495F0, &args![target]).u32();
        fn_00687d40(e, Ptr::new(skin), data, positions, normals, 0, 0, 0xc);
        let skin_object = e.call(SKIN_DATA_OF_005585E0, &args![skin]).u32();
        let bone_data = e.call(BONE_DATA_008D6F30, &args![skin_object]).u32();
        let bone_count = e.call(BONE_COUNT_008041A0, &args![skin_object]).u32();
        let block = allocate_array(e, bone_count, 0x4c);
        let new_bones = if block != 0 {
            e.call(
                VECTOR_CONSTRUCT_00401050,
                &args![block, 0x4cu32, bone_count, BONE_CONSTRUCT_00A84C90],
            );
            block
        } else {
            0
        };
        let mut bone: i32 = 0;
        while bone < bone_count as i32 {
            let b = bone as u32;
            let to = new_bones.wrapping_add(b * 0x4c);
            for i in 0..13 {
                let word = e.mem.u32(bone_data.wrapping_add(b * 0x4c) + 4 * i);
                e.mem.set_u32(to + 4 * i, word);
            }
            let array = if b == 0 {
                allocate_weights(e, vertex_count << 2)
            } else {
                allocate_weights(e, vertex_count)
            };
            e.mem.set_u32(to + 0x44, array);
            bone += 1;
        }
        let local_vector = local(0xa4);
        copy_point(e, vector, local_vector);
        let point = local(0x28);
        let normal = local(0x8c);
        e.call(LOCAL_INIT_006815C0, &args![point]);
        e.call(LOCAL_INIT_006815C0, &args![normal]);
        let outer = local(0x1bc);
        let inner = local(0x1b0);
        let half: f64 = e.global(HALF_DOUBLE);
        let two_fifty_six: f64 = e.global(TWO_FIFTY_SIX);
        let mut vertex: i32 = 0;
        while vertex < vertex_count as i32 {
            let v = vertex as u32;
            copy_point(e, POINT_ZERO, outer);
            copy_point(e, positions.wrapping_add(v * 0xc), point);
            copy_point(e, normals.wrapping_add(v * 0xc), normal);
            copy_point(e, POINT_ZERO, inner);
            let scale = x87_quiet_bits(e.mem.u32(transform + 0x30));
            e.call(POINT_TIMES_FLOAT_00439180, &args![point, scale]);
            e.call(
                TRANSFORM_VERTICES,
                &args![transform, transform + 0x24, 1u32, point, inner],
            );
            e.call(
                TRANSFORM_VERTICES,
                &args![projection, POINT_ZERO, 1u32, inner, outer],
            );
            let to = projected.wrapping_add(v * 0xc);
            for k in 0..2 {
                let value = x87_add(
                    x87_div(
                        x87_load32(e.mem.u32(outer + 4 * k)),
                        x87_m32(e.mem.u32(base + 0x50)),
                    ),
                    half,
                );
                e.mem.set_u32(to + 4 * k, x87_store32(value));
            }
            let dot = ni_point3_dot(e, Ptr::new(local_vector), normal);
            let larger = e
                .call(MAX_00404010, &args![x87_store32(0.0), dot.to_bits()])
                .f64();
            let facing = x87_store32(x87_sub(1.0, larger));
            let power = f64::from(fn_00685d20(e, f32::from_bits(facing), 10));
            let depth = x87_quiet_bits(e.mem.u32(outer + 8));
            let magnitude = e.call(ABS_00408840, &args![depth]).f64();
            let fade = x87_sub(1.0, x87_div(magnitude, two_fifty_six));
            e.mem.set_u32(to + 8, x87_store32(x87_mul(fade, power)));
            vertex += 1;
        }
        let mut out_count: u16 = 0;
        let mut new_count: u16 = 0;
        used = 0;
        let mut index: i32 = 0;
        while index < part_count as i32 {
            let i = index as u32;
            let flags = e.call(PARTITION_FLAGS_009E32D0, &args![dismember]).u32();
            let enabled = e.mem.u8(flags + i * 4);
            let flags = e.call(PARTITION_FLAGS_009E32D0, &args![dismember]).u32();
            let part = reduce_body_part(e.mem.u16(flags + i * 4 + 2));
            if enabled == 0
                || (1u32 << ((u32::from(part) + 1) & 0x1f)) & e.mem.u32(base + 0x60) == 0
            {
                index += 1;
                continue;
            }
            let source = parts + i * 0x2c;
            let triangle_count = e.mem.u16(source + 0x1e);
            let triangles = e.mem.u32(source + 0x14);
            let vertex_map = e.mem.u32(source + 0xc);
            let strips = e.mem.u16(source + 0x22) != 0;
            let mut part_index_count: u16 = 0;
            let mut part_vertex_count: u16 = 0;
            e.call(
                FILL_MEMORY_00403D30,
                &args![part_remap, 0xffff_ffffu32, vertex_count << 2],
            );
            e.call(
                FILL_MEMORY_00403D30,
                &args![weights, 0u32, (vertex_count << 2) << 2],
            );
            e.call(
                FILL_MEMORY_00403D30,
                &args![bone_indices, 0u32, vertex_count << 2],
            );
            let mut flip = false;
            let mut position: u32 = 0;
            let mut triangle: i32 = 0;
            while triangle < i32::from(triangle_count) {
                let ok = true;
                let corner = |e: &mut Engine, k: u32| {
                    let slot = e.mem.u16(triangles.wrapping_add((position + k) * 2));
                    e.mem.u16(vertex_map.wrapping_add(u32::from(slot) * 2))
                };
                let a = corner(e, 0);
                let b = corner(e, 1);
                let c = corner(e, 2);
                if a != b && a != c && b != c {
                    let hit = fn_00683a70(
                        e,
                        this,
                        triangle as u32,
                        0,
                        u32::from(strips),
                        triangles,
                        projected,
                        vertex_map,
                    );
                    if hit && ok {
                        let mut k: i32 = 0;
                        while k < 3 {
                            let slot = e.mem.u16(triangles.wrapping_add((position + k as u32) * 2));
                            let global =
                                u32::from(e.mem.u16(vertex_map.wrapping_add(u32::from(slot) * 2)));
                            let remapped = e.mem.u32(remap + global * 4);
                            if remapped == 0xffff_ffff {
                                e.mem.set_u32(remap + global * 4, u32::from(new_count));
                                e.mem
                                    .set_u16(out_triangles + u32::from(out_count) * 2, new_count);
                                out_count = out_count.wrapping_add(1);
                                new_count = new_count.wrapping_add(1);
                            } else {
                                let word = e.mem.u16(remap + global * 4);
                                e.mem
                                    .set_u16(out_triangles + u32::from(out_count) * 2, word);
                                out_count = out_count.wrapping_add(1);
                            }
                            if e.mem.u32(part_remap + global * 4) == 0xffff_ffff {
                                let per_vertex = u32::from(e.mem.u16(source + 0x24));
                                let mut w: i32 = 0;
                                while w < per_vertex as i32 {
                                    let per_vertex = u32::from(e.mem.u16(source + 0x24));
                                    let from = u32::from(slot)
                                        .wrapping_mul(per_vertex)
                                        .wrapping_add(w as u32);
                                    let to = u32::from(part_vertex_count)
                                        .wrapping_mul(per_vertex)
                                        .wrapping_add(w as u32);
                                    let weight = x87_quiet_bits(
                                        e.mem.u32(e.mem.u32(source + 8).wrapping_add(from * 4)),
                                    );
                                    e.mem.set_u32(weights.wrapping_add(to * 4), weight);
                                    let bone_index =
                                        e.mem.u8(e.mem.u32(source + 0x10).wrapping_add(from));
                                    e.mem.set_u8(bone_indices.wrapping_add(to), bone_index);
                                    w += 1;
                                }
                                e.mem
                                    .set_u32(part_remap + global * 4, u32::from(part_vertex_count));
                                e.mem.set_u16(
                                    part_triangles + u32::from(part_index_count) * 2,
                                    part_vertex_count,
                                );
                                part_index_count = part_index_count.wrapping_add(1);
                                let word = e.mem.u16(remap + global * 4);
                                e.mem.set_u16(
                                    part_vertex_map + u32::from(part_vertex_count) * 2,
                                    word,
                                );
                                part_vertex_count = part_vertex_count.wrapping_add(1);
                            } else {
                                let word = e.mem.u16(part_remap + global * 4);
                                e.mem.set_u16(
                                    part_triangles + u32::from(part_index_count) * 2,
                                    word,
                                );
                                part_index_count = part_index_count.wrapping_add(1);
                            }
                            k += 1;
                        }
                        if strips && flip {
                            for (array, count) in [
                                (out_triangles, out_count),
                                (part_triangles, part_index_count),
                            ] {
                                let last = array + u32::from(count) * 2;
                                let first = e.mem.u16(last - 6);
                                let third = e.mem.u16(last - 2);
                                e.mem.set_u16(last - 6, third);
                                e.mem.set_u16(last - 2, first);
                            }
                        }
                    }
                }
                if strips {
                    flip = !flip;
                    position += 1;
                } else {
                    position += 3;
                }
                triangle += 1;
            }
            if part_vertex_count > 0 {
                let to = partitions.wrapping_add(used * 0x2c);
                e.mem.set_u16(to + 0x1e, part_index_count / 3);
                let block = e
                    .call(
                        NI_ALLOCATE_00AA1070,
                        &args![u32::from(part_index_count) << 1],
                    )
                    .u32();
                e.mem.set_u32(to + 0x14, block);
                e.call(
                    COPY_MEMORY_00401460,
                    &args![block, part_triangles, u32::from(part_index_count) << 1],
                );
                e.mem.set_u16(to + 0x1c, part_vertex_count);
                let block = e
                    .call(
                        NI_ALLOCATE_00AA1070,
                        &args![u32::from(part_vertex_count) << 1],
                    )
                    .u32();
                e.mem.set_u32(to + 0xc, block);
                e.call(
                    COPY_MEMORY_00401460,
                    &args![block, part_vertex_map, u32::from(part_vertex_count) << 1],
                );
                let bones_used = e.mem.u16(source + 0x20);
                e.mem.set_u16(to + 0x20, bones_used);
                let bones_used = u32::from(e.mem.u16(source + 0x20)) << 1;
                let block = e.call(NI_ALLOCATE_00AA1070, &args![bones_used]).u32();
                e.mem.set_u32(to + 4, block);
                let bones_used = u32::from(e.mem.u16(source + 0x20)) << 1;
                let bone_list = e.mem.u32(source + 4);
                e.call(COPY_MEMORY_00401460, &args![block, bone_list, bones_used]);
                let per_vertex = e.mem.u16(source + 0x24);
                e.mem.set_u16(to + 0x24, per_vertex);
                let total =
                    u32::from(e.mem.u16(to + 0x24)).wrapping_mul(u32::from(part_vertex_count));
                let block = e.call(NI_ALLOCATE_00AA1070, &args![total << 2]).u32();
                e.mem.set_u32(to + 8, block);
                let block = e.call(NI_ALLOCATE_00AA1070, &args![total]).u32();
                e.mem.set_u32(to + 0x10, block);
                let size = (u32::from(e.mem.u16(to + 0x24)) << 2)
                    .wrapping_mul(u32::from(e.mem.u16(to + 0x1c)));
                let weights_to = e.mem.u32(to + 8);
                e.call(COPY_MEMORY_00401460, &args![weights_to, weights, size]);
                let size =
                    u32::from(e.mem.u16(to + 0x24)).wrapping_mul(u32::from(e.mem.u16(to + 0x1c)));
                let indices_to = e.mem.u32(to + 0x10);
                e.call(COPY_MEMORY_00401460, &args![indices_to, bone_indices, size]);
                if used >= part_limit {
                    e.call(DEBUG_BREAK, &args![]);
                }
                e.mem.set_u16(part_list + used * 2, part);
                used += 1;
            }
            index += 1;
        }
        let mut partition = 0;
        if used as i32 > 0 {
            let block = e.call(NI_ALLOCATE, &args![0x10u32]).u32();
            partition = if block != 0 {
                e.call(PARTITION_CONSTRUCT_00A81F90, &args![block]).u32()
            } else {
                0
            };
            fn_00685e60(e, Ptr::new(partition), partitions, used);
        }
        if new_count > 0 {
            let triangles_copy = allocate_array(e, u32::from(out_count), 2);
            e.call(
                COPY_MEMORY_00401460,
                &args![triangles_copy, out_triangles, u32::from(out_count) << 1],
            );
            let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
            let data = e.call(GEOMETRY_DATA_005495F0, &args![target]).u32();
            let source_positions = e.call(WORD_AT_20_007AF430, &args![data]).u32();
            let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
            let data = e.call(GEOMETRY_DATA_005495F0, &args![target]).u32();
            // The exe also takes the source's +0x24 array and never reads it.
            e.call(WORD_AT_24_0059BB30, &args![data]);
            let new_positions = allocate_points(e, u32::from(new_count));
            let new_projected = allocate_points(e, u32::from(new_count));
            let mut v: i32 = 0;
            while v < vertex_count as i32 {
                let remapped = e.mem.u32(remap + (v as u32) * 4);
                if remapped != 0xffff_ffff {
                    copy_point(
                        e,
                        source_positions.wrapping_add((v as u32) * 0xc),
                        new_positions.wrapping_add(remapped.wrapping_mul(0xc)),
                    );
                    copy_point(
                        e,
                        projected.wrapping_add((v as u32) * 0xc),
                        new_projected.wrapping_add(remapped.wrapping_mul(0xc)),
                    );
                }
                v += 1;
            }
            let mut bone: i32 = 0;
            while bone < bone_count as i32 {
                let b = bone as u32;
                let from = bone_data.wrapping_add(b * 0x4c);
                let to = new_bones.wrapping_add(b * 0x4c);
                let mut w: i32 = 0;
                while w < i32::from(e.mem.u16(from + 0x48)) {
                    let entry = e.mem.u32(from + 0x44).wrapping_add((w as u32) * 8);
                    let vertex = u32::from(e.mem.u16(entry));
                    if e.mem.u32(remap + vertex * 4) != 0xffff_ffff {
                        let slot = e
                            .mem
                            .u32(to + 0x44)
                            .wrapping_add(u32::from(e.mem.u16(to + 0x48)) * 8);
                        let word = e.mem.u16(remap + vertex * 4);
                        e.mem.set_u16(slot, word);
                        let entry = e.mem.u32(from + 0x44).wrapping_add((w as u32) * 8);
                        let slot = e
                            .mem
                            .u32(to + 0x44)
                            .wrapping_add(u32::from(e.mem.u16(to + 0x48)) * 8);
                        let weight = x87_quiet_bits(e.mem.u32(entry + 4));
                        e.mem.set_u32(slot + 4, weight);
                        let count = e.mem.u16(to + 0x48).wrapping_add(1);
                        e.mem.set_u16(to + 0x48, count);
                    }
                    w += 1;
                }
                let old = e.mem.u32(to + 0x44);
                let count = u32::from(e.mem.u16(to + 0x48));
                let array = allocate_weights(e, count);
                e.mem
                    .set_u32(new_bones.wrapping_add(b * 0x4c) + 0x44, array);
                let size = u32::from(e.mem.u16(to + 0x48)) << 3;
                let array = e.mem.u32(new_bones.wrapping_add(b * 0x4c) + 0x44);
                e.call(COPY_MEMORY_00401460, &args![array, old, size]);
                e.call(OPERATOR_DELETE, &args![old]);
                bone += 1;
            }
            let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
            let (list, list_parts) = if partition != 0 {
                (partition, part_list)
            } else {
                (0, 0)
            };
            fn_00687d90(
                e,
                this,
                target,
                new_count,
                out_count,
                new_positions,
                new_projected,
                triangles_copy,
                new_bones,
                list,
                list_parts,
            );
            e.mem.set_u32(base + 0x5c, u32::from(new_count));
        } else {
            let mut bone: i32 = 0;
            while bone < bone_count as i32 {
                let slot = new_bones.wrapping_add((bone as u32) * 0x4c) + 0x44;
                let array = e.mem.u32(slot);
                e.call(OPERATOR_DELETE, &args![array]);
                e.mem.set_u32(slot, 0);
                bone += 1;
            }
            e.call(OPERATOR_DELETE, &args![new_bones]);
        }
        for block in [
            positions,
            normals,
            out_triangles,
            part_triangles,
            part_vertex_map,
            remap,
            part_remap,
            weights,
            bone_indices,
            projected,
            part_list,
        ] {
            e.call(OPERATOR_DELETE, &args![block]);
        }
    });
}

/// The calls only [`fn_00683c10`] makes, by address.
const STREAM_CONSTRUCT_00534550: u32 = 0x0053_4550;
const STREAM_VALUE_00534570: u32 = 0x0053_4570;
const SKIN_PARTITION_DATA_004A8A90: u32 = 0x004a_8a90;
const STREAM_VERTEX_COUNT_0047D370: u32 = 0x0047_d370;
const DATA_LOCK_00A67360: u32 = 0x00a6_7360;
const DATA_UNLOCK_00A673D0: u32 = 0x00a6_73d0;
const DATA_STREAM_00A67410: u32 = 0x00a6_7410;
const DATA_STREAM_00A675B0: u32 = 0x00a6_75b0;
const DATA_STREAM_00A67720: u32 = 0x00a6_7720;
const DATA_STREAM_00A674B0: u32 = 0x00a6_74b0;
const DATA_STREAM_00A67530: u32 = 0x00a6_7530;
const SKIN_DEFORM_00A85510: u32 = 0x00a8_5510;
const WEIGHT_CONSTRUCT_00685D40: u32 = 0x0068_5d40;
const SMALL_WEIGHT: u32 = 0x0103_0f28;

// Translated from 00683c10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00683c10 - the compiler's exception frame (FS:[0] chain, the unwind states of the array allocations) is not translated
/// Unnamed (the skinned decal's build from the geometry's data streams,
/// `thiscall`): as [`fn_00685ef0`], but the target's vertex data comes
/// from its streams: the data is locked (`00a67360(0)`, unlocked at the
/// end when that answered non-zero), five stream readers are set up
/// (`00a67410`, `00a675b0`, `00a67720(0)`, `00a674b0`, `00a67530`) and the
/// skin deforms through them (`00a85510`). Each vertex also gets its four
/// bone weights (`00685d70`; the fourth `1 - a - b - c`, 0 below about
/// 0.0001). Nothing is done without bone data. The partitions index the
/// vertices directly from a running base (each partition's vertex count,
/// +0x1C, added after it, enabled or not) and the first partition's strip
/// flag serves all of them. A kept vertex adds itself, with each non-zero
/// weight, to the new bone (the partition's bone list entry of its bone
/// index byte, read from the stream `00685e20`), and to the partition's
/// weights and bone indices. The kept positions come from the first
/// stream (`00534570`).
pub fn fn_00683c10(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x320, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x320 - offset;
        let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
        let skin = e.call(GEOMETRY_TEST_0043FAD0, &args![target]).u32();
        if e.call(IS_KIND_0045BAD0, &args![DISMEMBER_SKIN_INSTANCE_RTTI, skin])
            .u8()
            == 0
        {
            return;
        }
        let dismember = skin;
        let controller = e.call(FIRST_CONTROLLER_0043B230, &args![skin]).u32();
        let part_count = e.call(LIST_COUNT_0044DDC0, &args![controller]).u32();
        let projection = local(0x84);
        e.call(LOCAL_INIT_006815C0, &args![projection]);
        let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
        let world = e.call(WORLD_TRANSFORM_00461130, &args![target]).u32();
        for i in 0..9 {
            let word = e.mem.u32(world + 4 * i);
            e.mem.set_u32(local(0x138) + 4 * i, word);
        }
        e.call(MATRIX_INVERSE_004B45B0, &args![local(0x138), local(0x1bc)]);
        let vector = local(0x108);
        e.call(LOCAL_INIT_006815C0, &args![vector]);
        let moved = e
            .call(
                MATRIX_TIMES_POINT_004B4500,
                &args![local(0x1bc), local(0x2bc), base + 0x44],
            )
            .u32();
        copy_point(e, moved, vector);
        let spin = local(0xbc);
        e.call(LOCAL_INIT_006815C0, &args![spin]);
        let rotation = x87_quiet_bits(e.mem.u32(base + 0x54));
        e.call(MATRIX_ANGLE_004A0C90, &args![spin, rotation]);
        let facing = e
            .call(
                ROTATION_Z_TO_VECTOR,
                &args![
                    local(0x2e0),
                    e.mem.u32(base + 0x44),
                    e.mem.u32(base + 0x48),
                    e.mem.u32(base + 0x4c)
                ],
            )
            .u32();
        let product = e
            .call(MATRIX_MULTIPLY_0043F8D0, &args![spin, local(0x304), facing])
            .u32();
        for i in 0..9 {
            let word = e.mem.u32(product + 4 * i);
            e.mem.set_u32(projection + 4 * i, word);
        }
        let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
        let world = e.call(WORLD_TRANSFORM_00461130, &args![target]).u32();
        let transform = local(0x184);
        for i in 0..13 {
            let word = e.mem.u32(world + 4 * i);
            e.mem.set_u32(transform + 4 * i, word);
        }
        e.call(
            SET_TRANSLATION_004578C0,
            &args![transform + 0x24, base + 0x38],
        );
        // The stream readers (12 bytes each).
        let positions_stream = local(0x2c);
        let normals_stream = local(0x44);
        let third_stream = local(0x98);
        let weights_stream = local(0x60);
        let bones_stream = local(0x20);
        for stream in [
            positions_stream,
            normals_stream,
            third_stream,
            weights_stream,
            bones_stream,
        ] {
            e.call(STREAM_CONSTRUCT_00534550, &args![stream]);
        }
        let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
        let data = e.call(GEOMETRY_DATA_005495F0, &args![target]).u32();
        let partition_data = e.call(SKIN_PARTITION_DATA_004A8A90, &args![data]).u32();
        let skin_object = e.call(SKIN_DATA_OF_005585E0, &args![skin]).u32();
        if e.call(BONE_DATA_008D6F30, &args![skin_object]).u32() == 0 {
            return;
        }
        let vertex_count = u32::from(
            e.call(STREAM_VERTEX_COUNT_0047D370, &args![partition_data])
                .u16(),
        );
        let geometry_data = |e: &mut Engine| {
            let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
            e.call(GEOMETRY_DATA_005495F0, &args![target]).u32()
        };
        let data = geometry_data(e);
        let locked = e.call(DATA_LOCK_00A67360, &args![data, 0u32]).u8();
        let data = geometry_data(e);
        e.call(DATA_STREAM_00A67410, &args![data, positions_stream]);
        let data = geometry_data(e);
        e.call(DATA_STREAM_00A675B0, &args![data, normals_stream]);
        let data = geometry_data(e);
        e.call(DATA_STREAM_00A67720, &args![data, 0u32, third_stream]);
        let data = geometry_data(e);
        e.call(DATA_STREAM_00A674B0, &args![data, weights_stream]);
        let data = geometry_data(e);
        e.call(DATA_STREAM_00A67530, &args![data, bones_stream]);
        let mut kept_triangles: u16 = 0;
        let mut any = false;
        let mut used: u32 = 0;
        let controller = e.call(FIRST_CONTROLLER_0043B230, &args![skin]).u32();
        let parts = e.call(FORM_ID_0084E3A0, &args![controller]).u32();
        let strips = e.mem.u16(parts + 0x22) != 0;
        let mut index: i32 = 0;
        while index < part_count as i32 {
            let i = index as u32;
            let flags = e.call(PARTITION_FLAGS_009E32D0, &args![dismember]).u32();
            let enabled = e.mem.u8(flags + i * 4);
            let flags = e.call(PARTITION_FLAGS_009E32D0, &args![dismember]).u32();
            let part = reduce_body_part(e.mem.u16(flags + i * 4 + 2));
            if enabled != 0
                && (1u32 << ((u32::from(part) + 1) & 0x1f)) & e.mem.u32(base + 0x60) != 0
            {
                any = true;
                kept_triangles = kept_triangles.wrapping_add(e.mem.u16(parts + i * 0x2c + 0x1e));
                used += 1;
            }
            index += 1;
        }
        if !any {
            return;
        }
        let index_total = u32::from(kept_triangles) * 3;
        let out_triangles = allocate_array(e, index_total, 2);
        let remap = allocate_array(e, vertex_count, 4);
        let mut new_count: u16 = 0;
        let part_triangles = allocate_array(e, index_total, 2);
        let part_vertex_map = allocate_array(e, vertex_count, 2);
        let part_remap = allocate_array(e, vertex_count, 4);
        let part_limit = used;
        let part_list = allocate_array(e, used, 2);
        let weights = allocate_array(e, vertex_count << 2, 4);
        let bone_indices = e.call(MEMORY_ALLOCATE, &args![vertex_count << 2]).u32();
        e.call(
            FILL_MEMORY_00403D30,
            &args![remap, 0xffff_ffffu32, vertex_count << 2],
        );
        let bytes = used
            .checked_mul(0x2c)
            .unwrap_or(u32::MAX)
            .checked_add(4)
            .unwrap_or(u32::MAX);
        let block = e.call(MEMORY_ALLOCATE, &args![bytes]).u32();
        let partitions = if block != 0 {
            e.mem.set_u32(block, used);
            e.call(
                EH_VECTOR_CONSTRUCT_00EC782F,
                &args![
                    block + 4,
                    0x2cu32,
                    used,
                    PARTITION_CONSTRUCT_00A81BA0,
                    PARTITION_DESTRUCT_00A81BD0
                ],
            );
            block + 4
        } else {
            0
        };
        let projected = allocate_points(e, vertex_count);
        let block = allocate_array(e, vertex_count, 0x10);
        let vertex_weights = if block != 0 {
            e.call(
                VECTOR_CONSTRUCT_00401050,
                &args![block, 0x10u32, vertex_count, WEIGHT_CONSTRUCT_00685D40],
            );
            block
        } else {
            0
        };
        let positions = allocate_points(e, vertex_count);
        let normals = allocate_points(e, vertex_count);
        e.call(
            SKIN_DEFORM_00A85510,
            &args![
                skin,
                positions_stream,
                weights_stream,
                bones_stream,
                normals_stream,
                vertex_count,
                positions,
                normals,
                0xcu32
            ],
        );
        let skin_object = e.call(SKIN_DATA_OF_005585E0, &args![skin]).u32();
        let bone_data = e.call(BONE_DATA_008D6F30, &args![skin_object]).u32();
        let bone_count = e.call(BONE_COUNT_008041A0, &args![skin_object]).u32();
        let block = allocate_array(e, bone_count, 0x4c);
        let new_bones = if block != 0 {
            e.call(
                VECTOR_CONSTRUCT_00401050,
                &args![block, 0x4cu32, bone_count, BONE_CONSTRUCT_00A84C90],
            );
            block
        } else {
            0
        };
        let mut bone: i32 = 0;
        while bone < bone_count as i32 {
            let b = bone as u32;
            let to = new_bones.wrapping_add(b * 0x4c);
            for i in 0..13 {
                let word = e.mem.u32(bone_data.wrapping_add(b * 0x4c) + 4 * i);
                e.mem.set_u32(to + 4 * i, word);
            }
            let array = if b == 0 {
                allocate_weights(e, vertex_count << 2)
            } else {
                allocate_weights(e, vertex_count)
            };
            e.mem.set_u32(to + 0x44, array);
            bone += 1;
        }
        let local_vector = local(0xe4);
        copy_point(e, vector, local_vector);
        let point = local(0x50);
        let normal = local(0xcc);
        e.call(LOCAL_INIT_006815C0, &args![point]);
        e.call(LOCAL_INIT_006815C0, &args![normal]);
        let outer = local(0x1fc);
        let inner = local(0x1f0);
        let half: f64 = e.global(HALF_DOUBLE);
        let two_fifty_six: f64 = e.global(TWO_FIFTY_SIX);
        let small: f64 = e.global(SMALL_WEIGHT);
        let mut vertex: i32 = 0;
        while vertex < vertex_count as i32 {
            let v = vertex as u32;
            copy_point(e, POINT_ZERO, outer);
            copy_point(e, positions.wrapping_add(v * 0xc), point);
            copy_point(e, normals.wrapping_add(v * 0xc), normal);
            let color = vertex_weights.wrapping_add(v * 0x10);
            fn_00685d70(e, Ptr::new(weights_stream), v, color);
            let sum = x87_add(
                x87_add(x87_load32(e.mem.u32(color)), x87_m32(e.mem.u32(color + 4))),
                x87_m32(e.mem.u32(color + 8)),
            );
            let sum = x87_store32(sum);
            e.mem
                .set_u32(color + 0xc, x87_store32(x87_sub(1.0, x87_load32(sum))));
            if x87_load32(e.mem.u32(color + 0xc)) < small {
                e.mem.set_u32(color + 0xc, x87_store32(0.0));
            }
            let scale = x87_quiet_bits(e.mem.u32(transform + 0x30));
            e.call(POINT_TIMES_FLOAT_00439180, &args![point, scale]);
            copy_point(e, POINT_ZERO, inner);
            e.call(
                TRANSFORM_VERTICES,
                &args![transform, transform + 0x24, 1u32, point, inner],
            );
            e.call(
                TRANSFORM_VERTICES,
                &args![projection, POINT_ZERO, 1u32, inner, outer],
            );
            let to = projected.wrapping_add(v * 0xc);
            for k in 0..2 {
                let value = x87_add(
                    x87_div(
                        x87_load32(e.mem.u32(outer + 4 * k)),
                        x87_m32(e.mem.u32(base + 0x50)),
                    ),
                    half,
                );
                e.mem.set_u32(to + 4 * k, x87_store32(value));
            }
            let dot = ni_point3_dot(e, Ptr::new(local_vector), normal);
            let larger = e
                .call(MAX_00404010, &args![x87_store32(0.0), dot.to_bits()])
                .f64();
            let facing = x87_store32(x87_sub(1.0, larger));
            let power = f64::from(fn_00685d20(e, f32::from_bits(facing), 10));
            let depth = x87_quiet_bits(e.mem.u32(outer + 8));
            let magnitude = e.call(ABS_00408840, &args![depth]).f64();
            let fade = x87_sub(1.0, x87_div(magnitude, two_fifty_six));
            e.mem.set_u32(to + 8, x87_store32(x87_mul(fade, power)));
            vertex += 1;
        }
        let mut out_count: u16 = 0;
        let mut vertex_base: u32 = 0;
        used = 0;
        let index_bytes = local(0x248);
        let part_bytes = local(0x258);
        let mut index: i32 = 0;
        while index < part_count as i32 {
            let i = index as u32;
            let flags = e.call(PARTITION_FLAGS_009E32D0, &args![dismember]).u32();
            let enabled = e.mem.u8(flags + i * 4);
            let flags = e.call(PARTITION_FLAGS_009E32D0, &args![dismember]).u32();
            let part = reduce_body_part(e.mem.u16(flags + i * 4 + 2));
            let source = parts + i * 0x2c;
            if enabled != 0
                && (1u32 << ((u32::from(part) + 1) & 0x1f)) & e.mem.u32(base + 0x60) != 0
            {
                // The exe also reads the partition's +0x28 word here and never uses it.
                let _ = e.mem.u32(source + 0x28);
                let triangle_count = e.mem.u16(source + 0x1e);
                let triangles = e.mem.u32(source + 0x14);
                let mut part_index_count: u16 = 0;
                let mut part_vertex_count: u16 = 0;
                e.call(
                    FILL_MEMORY_00403D30,
                    &args![part_remap, 0xffff_ffffu32, vertex_count << 2],
                );
                e.call(
                    FILL_MEMORY_00403D30,
                    &args![weights, 0u32, (vertex_count << 2) << 2],
                );
                e.call(
                    FILL_MEMORY_00403D30,
                    &args![bone_indices, 0u32, vertex_count << 2],
                );
                let mut flip = false;
                let mut triangle: i32 = 0;
                while triangle < i32::from(triangle_count) {
                    let t = triangle as u32;
                    let ok = true;
                    let slot_of = |k: u32| {
                        if strips {
                            t.wrapping_add(k)
                        } else {
                            t.wrapping_mul(3).wrapping_add(k)
                        }
                    };
                    let corner = |e: &mut Engine, k: u32| {
                        e.mem
                            .u16(triangles.wrapping_add(slot_of(k) * 2))
                            .wrapping_add(vertex_base as u16)
                    };
                    let a = corner(e, 0);
                    let b = corner(e, 1);
                    let c = corner(e, 2);
                    if a == b || a == c || b == c {
                        if strips {
                            flip = !flip;
                        }
                        triangle += 1;
                        continue;
                    }
                    let hit = fn_00683a70(
                        e,
                        this,
                        t,
                        vertex_base,
                        u32::from(strips),
                        triangles,
                        projected,
                        0,
                    );
                    if hit && ok {
                        let mut k: i32 = 0;
                        while k < 3 {
                            let slot = e.mem.u16(triangles.wrapping_add(slot_of(k as u32) * 2));
                            let global = u32::from(slot.wrapping_add(vertex_base as u16));
                            if e.mem.u32(remap + global * 4) == 0xffff_ffff {
                                e.mem.set_u32(remap + global * 4, u32::from(new_count));
                                e.mem
                                    .set_u16(out_triangles + u32::from(out_count) * 2, new_count);
                                out_count = out_count.wrapping_add(1);
                                fn_00685e20(e, Ptr::new(bones_stream), global, index_bytes);
                                let mut w: i32 = 0;
                                while w < 4 {
                                    let weight_at = vertex_weights
                                        .wrapping_add(global * 0x10)
                                        .wrapping_add((w as u32) * 4);
                                    if x87_load32(e.mem.u32(weight_at)) != 0.0 {
                                        let byte = u32::from(e.mem.u8(index_bytes + 3 - w as u32));
                                        let bone_list = e.mem.u32(source + 4);
                                        let bone_number =
                                            e.mem.u16(bone_list.wrapping_add(byte * 2));
                                        let to =
                                            new_bones.wrapping_add(u32::from(bone_number) * 0x4c);
                                        let slot = e
                                            .mem
                                            .u32(to + 0x44)
                                            .wrapping_add(u32::from(e.mem.u16(to + 0x48)) * 8);
                                        e.mem.set_u16(slot, new_count);
                                        let weight = x87_quiet_bits(e.mem.u32(weight_at));
                                        let slot = e
                                            .mem
                                            .u32(to + 0x44)
                                            .wrapping_add(u32::from(e.mem.u16(to + 0x48)) * 8);
                                        e.mem.set_u32(slot + 4, weight);
                                        let count = e.mem.u16(to + 0x48).wrapping_add(1);
                                        e.mem.set_u16(to + 0x48, count);
                                    }
                                    w += 1;
                                }
                                new_count = new_count.wrapping_add(1);
                            } else {
                                let word = e.mem.u16(remap + global * 4);
                                e.mem
                                    .set_u16(out_triangles + u32::from(out_count) * 2, word);
                                out_count = out_count.wrapping_add(1);
                            }
                            if e.mem.u32(part_remap + global * 4) == 0xffff_ffff {
                                fn_00685e20(e, Ptr::new(bones_stream), global, part_bytes);
                                let mut w: i32 = 0;
                                while w < 4 {
                                    let weight_at = vertex_weights
                                        .wrapping_add(global * 0x10)
                                        .wrapping_add((w as u32) * 4);
                                    if x87_load32(e.mem.u32(weight_at)) != 0.0 {
                                        let byte = e.mem.u8(part_bytes + 3 - w as u32);
                                        let at = (w as u32)
                                            .wrapping_add(u32::from(part_vertex_count) * 4);
                                        let weight = x87_quiet_bits(e.mem.u32(weight_at));
                                        e.mem.set_u32(weights.wrapping_add(at * 4), weight);
                                        e.mem.set_u8(bone_indices.wrapping_add(at), byte);
                                    }
                                    w += 1;
                                }
                                e.mem
                                    .set_u32(part_remap + global * 4, u32::from(part_vertex_count));
                                e.mem.set_u16(
                                    part_triangles + u32::from(part_index_count) * 2,
                                    part_vertex_count,
                                );
                                part_index_count = part_index_count.wrapping_add(1);
                                let word = e.mem.u16(remap + global * 4);
                                e.mem.set_u16(
                                    part_vertex_map + u32::from(part_vertex_count) * 2,
                                    word,
                                );
                                part_vertex_count = part_vertex_count.wrapping_add(1);
                            } else {
                                let word = e.mem.u16(part_remap + global * 4);
                                e.mem.set_u16(
                                    part_triangles + u32::from(part_index_count) * 2,
                                    word,
                                );
                                part_index_count = part_index_count.wrapping_add(1);
                            }
                            k += 1;
                        }
                        if strips && flip {
                            for (array, count) in [
                                (out_triangles, out_count),
                                (part_triangles, part_index_count),
                            ] {
                                let last = array + u32::from(count) * 2;
                                let first = e.mem.u16(last - 6);
                                let third = e.mem.u16(last - 2);
                                e.mem.set_u16(last - 6, third);
                                e.mem.set_u16(last - 2, first);
                            }
                        }
                    }
                    if strips {
                        flip = !flip;
                    }
                    triangle += 1;
                }
                if part_vertex_count > 0 {
                    let to = partitions.wrapping_add(used * 0x2c);
                    e.mem.set_u16(to + 0x1e, part_index_count / 3);
                    let block = e
                        .call(
                            NI_ALLOCATE_00AA1070,
                            &args![u32::from(part_index_count) << 1],
                        )
                        .u32();
                    e.mem.set_u32(to + 0x14, block);
                    e.call(
                        COPY_MEMORY_00401460,
                        &args![block, part_triangles, u32::from(part_index_count) << 1],
                    );
                    e.mem.set_u16(to + 0x1c, part_vertex_count);
                    let block = e
                        .call(
                            NI_ALLOCATE_00AA1070,
                            &args![u32::from(part_vertex_count) << 1],
                        )
                        .u32();
                    e.mem.set_u32(to + 0xc, block);
                    e.call(
                        COPY_MEMORY_00401460,
                        &args![block, part_vertex_map, u32::from(part_vertex_count) << 1],
                    );
                    let bones_used = e.mem.u16(source + 0x20);
                    e.mem.set_u16(to + 0x20, bones_used);
                    let bones_used = u32::from(e.mem.u16(source + 0x20)) << 1;
                    let block = e.call(NI_ALLOCATE_00AA1070, &args![bones_used]).u32();
                    e.mem.set_u32(to + 4, block);
                    let bones_used = u32::from(e.mem.u16(source + 0x20)) << 1;
                    let bone_list = e.mem.u32(source + 4);
                    e.call(COPY_MEMORY_00401460, &args![block, bone_list, bones_used]);
                    let per_vertex = e.mem.u16(source + 0x24);
                    e.mem.set_u16(to + 0x24, per_vertex);
                    let total =
                        u32::from(e.mem.u16(to + 0x24)).wrapping_mul(u32::from(part_vertex_count));
                    let block = e.call(NI_ALLOCATE_00AA1070, &args![total << 2]).u32();
                    e.mem.set_u32(to + 8, block);
                    let block = e.call(NI_ALLOCATE_00AA1070, &args![total]).u32();
                    e.mem.set_u32(to + 0x10, block);
                    let size = (u32::from(e.mem.u16(to + 0x24)) << 2)
                        .wrapping_mul(u32::from(e.mem.u16(to + 0x1c)));
                    let weights_to = e.mem.u32(to + 8);
                    e.call(COPY_MEMORY_00401460, &args![weights_to, weights, size]);
                    let size = u32::from(e.mem.u16(to + 0x24))
                        .wrapping_mul(u32::from(e.mem.u16(to + 0x1c)));
                    let indices_to = e.mem.u32(to + 0x10);
                    e.call(COPY_MEMORY_00401460, &args![indices_to, bone_indices, size]);
                    if used >= part_limit {
                        e.call(DEBUG_BREAK, &args![]);
                    }
                    e.mem.set_u16(part_list + used * 2, part);
                    used += 1;
                }
            }
            vertex_base = u32::from(e.mem.u16(source + 0x1c)).wrapping_add(vertex_base);
            index += 1;
        }
        e.call(OPERATOR_DELETE, &args![vertex_weights]);
        let mut partition = 0;
        if used as i32 > 0 {
            let block = e.call(NI_ALLOCATE, &args![0x10u32]).u32();
            partition = if block != 0 {
                e.call(PARTITION_CONSTRUCT_00A81F90, &args![block]).u32()
            } else {
                0
            };
            fn_00685e60(e, Ptr::new(partition), partitions, used);
        }
        if new_count > 0 {
            let triangles_copy = allocate_array(e, u32::from(out_count), 2);
            e.call(
                COPY_MEMORY_00401460,
                &args![triangles_copy, out_triangles, u32::from(out_count) << 1],
            );
            let new_positions = allocate_points(e, u32::from(new_count));
            let new_projected = allocate_points(e, u32::from(new_count));
            let position = local(0x294);
            let second = local(0x2a0);
            let mut v: i32 = 0;
            while v < vertex_count as i32 {
                e.call(LOCAL_INIT_006815C0, &args![position]);
                e.call(LOCAL_INIT_006815C0, &args![second]);
                let remapped = e.mem.u32(remap + (v as u32) * 4);
                if remapped != 0xffff_ffff {
                    e.call(
                        STREAM_VALUE_00534570,
                        &args![positions_stream, v as u32, position],
                    );
                    e.call(
                        STREAM_VALUE_00534570,
                        &args![normals_stream, v as u32, second],
                    );
                    let remapped = e.mem.u32(remap + (v as u32) * 4);
                    copy_point(
                        e,
                        position,
                        new_positions.wrapping_add(remapped.wrapping_mul(0xc)),
                    );
                    copy_point(
                        e,
                        projected.wrapping_add((v as u32) * 0xc),
                        new_projected.wrapping_add(remapped.wrapping_mul(0xc)),
                    );
                }
                v += 1;
            }
            let mut bone: i32 = 0;
            while bone < bone_count as i32 {
                let to = new_bones.wrapping_add((bone as u32) * 0x4c);
                let old = e.mem.u32(to + 0x44);
                let count = u32::from(e.mem.u16(to + 0x48));
                let array = allocate_weights(e, count);
                e.mem.set_u32(to + 0x44, array);
                let size = u32::from(e.mem.u16(to + 0x48)) << 3;
                let array = e.mem.u32(to + 0x44);
                e.call(COPY_MEMORY_00401460, &args![array, old, size]);
                e.call(OPERATOR_DELETE, &args![old]);
                bone += 1;
            }
            let target = e.call(WORD_AT_0_00559450, &args![base + 0x2c]).u32();
            let (list, list_parts) = if partition != 0 {
                (partition, part_list)
            } else {
                (0, 0)
            };
            fn_00687d90(
                e,
                this,
                target,
                new_count,
                out_count,
                new_positions,
                new_projected,
                triangles_copy,
                new_bones,
                list,
                list_parts,
            );
            e.mem.set_u32(base + 0x5c, u32::from(new_count));
        } else {
            let mut bone: i32 = 0;
            while bone < bone_count as i32 {
                let slot = new_bones.wrapping_add((bone as u32) * 0x4c) + 0x44;
                let array = e.mem.u32(slot);
                e.call(OPERATOR_DELETE, &args![array]);
                e.mem.set_u32(slot, 0);
                bone += 1;
            }
            e.call(OPERATOR_DELETE, &args![new_bones]);
        }
        for block in [
            positions,
            normals,
            out_triangles,
            remap,
            projected,
            part_triangles,
            part_vertex_map,
            part_remap,
            weights,
            bone_indices,
            part_list,
        ] {
            e.call(OPERATOR_DELETE, &args![block]);
        }
        if locked != 0 {
            let data = geometry_data(e);
            e.call(DATA_UNLOCK_00A673D0, &args![data]);
        }
    });
}

// Translated from 00688f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, a `float` and an `int`): `base` raised to the integer
/// `exponent` by squaring (each product stored to a `float`), its
/// reciprocal for a negative exponent (through a `double` local), returned
/// as a `float` in ST0.
pub fn fn_00688f10(_e: &mut Engine, base: f32, exponent: i32) -> f32 {
    let mut remaining = if exponent >= 0 {
        exponent as u32
    } else {
        exponent.wrapping_neg() as u32
    };
    let mut result = x87_store32(1.0);
    let mut square = base.to_bits();
    loop {
        if remaining & 1 != 0 {
            result = x87_store32(x87_mul(x87_load32(result), x87_m32(square)));
        }
        remaining >>= 1;
        if remaining == 0 {
            break;
        }
        square = x87_store32(x87_mul(x87_load32(square), x87_m32(square)));
    }
    let value = if exponent < 0 {
        x87_div(1.0, x87_load32(result))
    } else {
        x87_load32(result)
    };
    f32::from_bits(x87_store32(value))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x0068_3900, fn_00683900_entry as AbiFn),
        entry!(
            0x0068_3a70,
            fn_00683a70(Ptr, u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(0x0068_5d20, fn_00685d20(f32, i32) -> f32),
        entry!(0x0068_5d40, fn_00685d40(Ptr) -> Ptr),
        entry!(0x0068_5d70, fn_00685d70(Ptr, u32, u32)),
        entry!(0x0068_5e20, fn_00685e20(Ptr, u32, u32)),
        entry!(0x0068_5e60, fn_00685e60(Ptr, u32, u32)),
        entry!(0x0068_7d40, fn_00687d40(Ptr, u32, u32, u32, u32, u32, u32)),
        entry!(
            0x0068_8470,
            fn_00688470(Ptr, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x0068_84c0, fn_006884c0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0068_8660, bs_temp_effect_geometry_decal_initialize(Ptr)),
        entry!(0x0068_8730, fn_00688730(Ptr) -> u32),
        entry!(0x0068_8770, fn_00688770(Ptr)),
        entry!(
            0x0068_88a0,
            bs_temp_effect_geometry_decal_update(Ptr, f32) -> bool
        ),
        entry!(
            0x0068_8a10,
            bs_temp_effect_geometry_decal_get_save_size(Ptr) -> u16
        ),
        entry!(0x0068_8b00, bs_temp_effect_geometry_decal_save_game(Ptr)),
        entry!(
            0x0068_8c90,
            bs_temp_effect_geometry_decal_load_game(Ptr) -> bool
        ),
        entry!(0x0068_8f10, fn_00688f10(f32, i32) -> f32),
        entry!(0x0068_5ef0, fn_00685ef0(Ptr)),
        entry!(0x0068_3c10, fn_00683c10(Ptr)),
        entry!(
            0x0068_7d90,
            fn_00687d90(Ptr, u32, u16, u16, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x004b_6190, ni_point3_dot(Ptr, u32) -> f32),
        entry!(0x0068_38b0, fn_006838b0(Ptr) -> u32),
        entry!(0x0068_8460, fn_00688460() -> u32),
        entry!(0x0068_3a60, fn_00683a60() -> u32),
        entry!(
            0x0068_5e80,
            ni_skin_partition_partition_vector_deleting_destructor(Ptr, u32) -> u32
        ),
        entry!(0x0068_8430, fn_00688430(Ptr, u32)),
        entry!(0x0068_8610, bs_dismember_skin_instance_destructor(Ptr)),
        entry!(0x0068_38a0, fn_006838a0(Ptr) -> u32),
        entry!(0x0068_8590, fn_00688590(Ptr) -> u32),
        entry!(0x0068_85d0, fn_006885d0(Ptr) -> u32),
        entry!(0x0068_38d0, bs_temp_effect_geometry_decal_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0068_85a0, ni_skin_instance_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0068_85e0, bs_dismember_skin_instance_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_temp_effect_geometry_decal_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0068_38d0, 0x0068_8770, 0x64);
    }

    #[test]
    fn ni_skin_instance_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0068_85a0, 0x00a8_5580, 0x34);
    }

    #[test]
    fn bs_dismember_skin_instance_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0068_85e0, 0x0068_8610, 0x40);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_006838a0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0068_38a0, &args![0x1000u32]).u32(), 0x011d_6b04);
        assert_eq!(e.call(0x0068_38a0, &args![0u32]).u32(), 0x011d_6b04);
    }

    #[test]
    fn fn_00688590_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0068_8590, &args![0x1000u32]).u32(), 0x011f_5b2c);
        assert_eq!(e.call(0x0068_8590, &args![0u32]).u32(), 0x011f_5b2c);
    }

    #[test]
    fn fn_006885d0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0068_85d0, &args![0x1000u32]).u32(), 0x011f_49d8);
        assert_eq!(e.call(0x0068_85d0, &args![0u32]).u32(), 0x011f_49d8);
    }
}

#[cfg(test)]
mod b060_tests {
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

    fn point(e: &mut Engine, x: f32, y: f32, z: f32) -> u32 {
        let p = e.mem.alloc(12);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        e.mem.set_f32(p + 8, z);
        p
    }

    #[test]
    fn dot_product_rounds_to_float() {
        let mut e = Engine::new();
        let a = point(&mut e, 1.5, -2.0, 0.25);
        let b = point(&mut e, 4.0, 3.0, 8.0);
        let r = e.call(0x004b_6190, &args![a, b]).st0;
        assert_eq!(r, 2.0);
        let c = point(&mut e, 0.1, 0.2, 0.3);
        let r = e.call(0x004b_6190, &args![c, c]).st0;
        let (x, y, z) = (f64::from(0.1f32), f64::from(0.2f32), f64::from(0.3f32));
        let expected = ((x * x + y * y) + z * z) as f32;
        assert_eq!(r, f64::from(expected));
    }

    #[test]
    fn word_getters() {
        let mut e = Engine::new();
        e.register(WORD_AT_0_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x1c, 0x1234);
        assert_eq!(e.call(0x0068_38b0, &args![this]).u32(), 0x1234);
        e.map(0x011f_5000, 0x1000);
        e.set_global(GLOBAL_011F5B04, 0x5678u32);
        assert_eq!(e.call(0x0068_3a60, &[]).u32(), 0x5678);
    }

    #[test]
    fn partition_vector_deleting_destructor_handles_arrays_and_single_ones() {
        let mut e = Engine::new();
        for addr in [
            VECTOR_DESTRUCT,
            OPERATOR_DELETE,
            PARTITION_DESTRUCTOR_00A81BD0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let block = e.mem.alloc(0x60);
        e.mem.set_u32(block, 2);
        let array = block + 4;
        assert_eq!(e.call(0x0068_5e80, &args![array, 3u32]).u32(), block);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![vec![array, 0x2c, 2, PARTITION_DESTRUCTOR_00A81BD0]]
        );
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![block]]);
        assert_eq!(e.call(0x0068_5e80, &args![array, 2u32]).u32(), block);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
        assert_eq!(e.call(0x0068_5e80, &args![array, 1u32]).u32(), array);
        assert_eq!(calls(&e, PARTITION_DESTRUCTOR_00A81BD0), vec![vec![array]]);
        assert_eq!(calls(&e, OPERATOR_DELETE)[1], vec![array]);
        e.call(0x0068_5e80, &args![array, 0u32]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 2);
    }

    #[test]
    fn fn_00688430_copies_nine_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x90);
        let source = e.mem.alloc(0x28);
        for i in 0..10 {
            e.mem.set_u32(source + 4 * i, 0x100 + i);
        }
        e.call(0x0068_8430, &args![this, source]);
        assert_eq!(e.mem.u32(this + 0x68), 0x100);
        assert_eq!(e.mem.u32(this + 0x88), 0x108);
        assert_eq!(e.mem.u32(this + 0x8c), 0);
    }

    #[test]
    fn dismember_skin_instance_destructor_frees_the_array() {
        let mut e = Engine::new();
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(NI_SKIN_INSTANCE_DESTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x34, 3);
        e.mem.set_u32(this + 0x38, 0x9000);
        e.call(0x0068_8610, &args![this]);
        assert_eq!(e.mem.u32(this), BS_DISMEMBER_SKIN_INSTANCE_VTABLE);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x9000]]);
        assert_eq!((e.mem.u32(this + 0x34), e.mem.u32(this + 0x38)), (0, 0));
        e.call(0x0068_8610, &args![this]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1);
        assert_eq!(calls(&e, NI_SKIN_INSTANCE_DESTRUCTOR), vec![vec![this]; 2]);
    }

    #[test]
    fn fn_00688460_answers_the_word_at_011c6228() {
        let mut e = Engine::new();
        e.map(TRANSIENT_NAME_WORD, 4);
        e.set_global(TRANSIENT_NAME_WORD, 0x0abc_def0u32);
        assert_eq!(e.call(0x0068_8460, &args![]).u32(), 0x0abc_def0);
    }
}

#[cfg(test)]
mod tests_n073 {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Default::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn noop(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// Smart pointers as plain words: `00559450` reads, `0066b0d0` writes.
    fn pointer_doubles(e: &mut Engine) {
        e.register(WORD_AT_0_00559450, |e, a| ret(e.mem.u32(a[0])));
        e.register(POINTER_SET_0066B0D0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
    }

    #[test]
    fn constructor_stores_its_words() {
        let mut e = Engine::new();
        pointer_doubles(&mut e);
        noop(
            &mut e,
            &[
                TEMP_EFFECT_CONSTRUCTOR,
                POINTER_CONSTRUCT_00633C90,
                LOCAL_INIT_006815C0,
            ],
        );
        e.register(PARENT_009611E0, |_, a| ret(a[0] + 0x100));
        let this = e.mem.alloc(0x64);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data + 0x30, 0x7e57);
        e.mem.set_u32(this + 0x18, 0xdddd);
        e.call_log = Some(vec![]);
        let words = args![
            this,
            0xce11u32,
            0.5f32.to_bits(),
            data,
            0x3000u32,
            1u32,
            2u32,
            3u32,
            4u32,
            5u32,
            6u32,
            2.0f32.to_bits(),
            0.25f32.to_bits(),
            0x9u32
        ];
        assert_eq!(e.call(0x0068_3900, &words).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), GEOMETRY_DECAL_VTABLE);
        assert_eq!(
            calls_to(&log, TEMP_EFFECT_CONSTRUCTOR),
            vec![vec![this, 0xce11, 0.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, POINTER_CONSTRUCT_00633C90).len(), 4);
        assert_eq!(e.mem.u32(this + 0x2c), 0x3000);
        assert_eq!(e.mem.u32(this + 0x30), 0x3100);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(this + 0x38 + 4 * i)).collect();
        assert_eq!(words, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(e.mem.f32(this + 0x50), 2.0);
        assert_eq!(e.mem.f32(this + 0x54), 0.25);
        assert_eq!(e.mem.u32(this + 0x58), 0x7e57);
        assert_eq!(e.mem.u32(this + 0x60), 9);
        assert_eq!(e.mem.u32(this + 0x18), 0xdddd);
    }

    fn box_globals(e: &mut Engine) {
        e.map(ZERO_DOUBLE & !0xfff, 0x1000);
        e.map(DEPTH_LIMIT & !0xfff, 0x1000);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(DEPTH_LIMIT, 0.1f64);
    }

    #[test]
    fn triangle_reach_test() {
        let mut e = Engine::new();
        box_globals(&mut e);
        let points = e.mem.alloc(0xc * 8);
        let set = |e: &mut Engine, i: u32, p: [f32; 3]| {
            for (k, v) in (0u32..).zip(p) {
                e.mem.set_f32(points + 0xc * i + 4 * k, v);
            }
        };
        // Corners 0..2 inside, 3..5 left and right of the box and deep.
        set(&mut e, 0, [0.5, 0.5, 0.0]);
        set(&mut e, 1, [0.2, 0.3, 0.0]);
        set(&mut e, 2, [0.7, 0.1, 0.0]);
        set(&mut e, 3, [-1.0, 0.5, 0.5]);
        set(&mut e, 4, [2.0, 0.5, 0.0]);
        set(&mut e, 5, [-1.0, 0.5, 0.0]);
        let indices = e.mem.alloc(2 * 8);
        for (i, v) in [0u16, 1, 2, 3, 4, 5].iter().enumerate() {
            e.mem.set_u16(indices + 2 * i as u32, *v);
        }
        let outside = |e: &mut Engine, triangle: u32, base: u32, strips: u32, remap: u32| {
            e.call(
                0x0068_3a70,
                &args![0u32, triangle, base, strips, indices, points, remap],
            )
            .bool()
        };
        // Triangle 0 (a whole corner inside: two counts plus none) is not
        // decided by a corner; no flags: false.
        assert!(!outside(&mut e, 0, 0, 0, 0));
        // Triangle 1 (corners 3, 4, 5): bits 1 and 2 and 0x10: true.
        assert!(outside(&mut e, 1, 0, 0, 0));
        // As strips from triangle 3 (corners 3, 4, 5) too.
        assert!(outside(&mut e, 3, 0, 1, 0));
        // Base 3 moves triangle 0 onto corners 3..5.
        assert!(outside(&mut e, 0, 3, 0, 0));
        // A remap sending everything to corner 4 (right, shallow): false.
        let remap = e.mem.alloc(2 * 8);
        for i in 0..8 {
            e.mem.set_u16(remap + 2 * i, 4);
        }
        assert!(!outside(&mut e, 1, 0, 0, remap));
        // A corner with x, y inside and z deep counts three: true at once.
        set(&mut e, 0, [0.5, 0.5, 0.5]);
        assert!(outside(&mut e, 0, 0, 0, 0));
    }

    #[test]
    fn power_by_squaring() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x0068_8f10, &args![2.0f32.to_bits(), 10u32]).f32(),
            1024.0
        );
        assert_eq!(
            e.call(0x0068_8f10, &args![2.0f32.to_bits(), 0u32]).f32(),
            1.0
        );
        assert_eq!(
            e.call(0x0068_8f10, &args![2.0f32.to_bits(), (-2i32) as u32])
                .f32(),
            0.25
        );
        assert_eq!(
            e.call(0x0068_5d20, &args![3.0f32.to_bits(), 3u32]).f32(),
            27.0
        );
    }

    #[test]
    fn small_helpers() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x1234);
        assert_eq!(e.call(0x0068_5d40, &args![this]).u32(), this);
        assert_eq!(e.mem.bytes(this, 16), vec![0; 16]);
        e.call(0x0068_5e60, &args![this, 7u32, 8u32]);
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u32(this + 8)), (7, 8));
        // An array of 12-byte elements.
        noop(&mut e, &[INDEX_CHECK_004534F0]);
        e.register(COLOR_COMPONENT_00560D30, |_, a| ret(a[0] + 4 * a[1]));
        let data = e.mem.alloc(0x24);
        e.mem.set_f32(data + 0xc, 0.25);
        e.mem.set_f32(data + 0x10, 0.5);
        e.mem.set_f32(data + 0x14, 0.125);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 0xc);
        let color = e.mem.alloc(0x10);
        e.call(0x0068_5d70, &args![array, 1u32, color]);
        let c: Vec<f32> = (0..4).map(|i| e.mem.f32(color + 4 * i)).collect();
        assert_eq!(c, vec![0.25, 0.5, 0.125, 0.125]);
        let out = e.mem.alloc(4);
        e.call(0x0068_5e20, &args![array, 1u32, out]);
        assert_eq!(e.mem.f32(out), 0.25);
    }

    #[test]
    fn fn_00687d40_hands_its_words_on() {
        let mut e = Engine::new();
        e.register(SKIN_SETUP_00A855F0, |_, _| Ret::default());
        e.register(WORD_AT_8_004A8AE0, |_, _| ret(0xffff_0005));
        e.register(WORD_AT_24_0059BB30, |_, _| ret(0x24));
        e.register(WORD_AT_20_007AF430, |_, _| ret(0x20));
        e.call_log = Some(vec![]);
        e.call(
            0x0068_7d40,
            &args![0x100u32, 0x200u32, 1u32, 2u32, 3u32, 4u32, 5u32],
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SKIN_SETUP_00A855F0),
            vec![vec![0x100, 0x20, 0x24, 5, 1, 2, 3, 4, 5]]
        );
        assert_eq!(calls_to(&log, WORD_AT_8_004A8AE0), vec![vec![0x200]]);
    }

    #[test]
    fn skin_instance_constructors() {
        let mut e = Engine::new();
        noop(&mut e, &[NI_OBJECT_CONSTRUCTOR]);
        e.register(POINTER_CONSTRUCT_00633C90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x2c, 0x2c2c);
        for offset in [0x1cu32, 0x20, 0x24, 0x28, 0x30] {
            e.mem.set_u32(this + offset, 0xffff);
        }
        assert_eq!(
            e.call(
                0x0068_8470,
                &args![this, 0xd0u32, 0x10u32, 0x14u32, 0x38u32, 0x34u32]
            )
            .u32(),
            this
        );
        assert_eq!(e.mem.u32(this), BS_DISMEMBER_SKIN_INSTANCE_VTABLE);
        let words: Vec<u32> = (0..0x10).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(
            words[1..],
            [
                0,
                0xd0,
                0,
                0x10,
                0x14,
                0xffff_ffff,
                0,
                0,
                0,
                0,
                0x2c2c,
                0,
                0x34,
                0x38,
                1
            ]
        );
        assert_eq!(
            e.call(0x0068_84c0, &args![this, 1u32, 2u32, 3u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), NI_SKIN_INSTANCE_VTABLE);
    }

    #[test]
    fn initialize_picks_the_build() {
        let mut e = Engine::new();
        pointer_doubles(&mut e);
        noop(&mut e, &[TEMP_EFFECT_INITIALIZE]);
        // Both builds stop at their skin test (0045bad0 answers 0); only the
        // unskinned one (00685ef0) returns before it without iBodyParts.
        e.register(IS_KIND_0045BAD0, |_, _| ret(0));
        e.register(GEOMETRY_TEST_0043FAD0, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(NODE_ANSWER_00726070, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(POINTER_AT_30_004A8A90, |e, a| ret(e.mem.u32(a[0] + 0x30)));
        let target = e.mem.alloc(0xc0);
        let parent = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x64);
        e.mem.set_u32(this + 0x60, 1);
        let reset = |e: &mut Engine| {
            e.mem.set_u32(this + 0x2c, target);
            e.mem.set_u32(this + 0x30, parent);
        };
        // The target fails its test: nothing at all.
        reset(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0068_8660, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
        assert_eq!(e.mem.u32(this + 0x2c), target);
        // Both answers above 1, no skin object: the unskinned build.
        e.mem.set_u32(target + 4, 1);
        e.mem.set_u32(target + 8, 2);
        e.mem.set_u32(parent + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x0068_8660, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, IS_KIND_0045BAD0),
            vec![vec![DISMEMBER_SKIN_INSTANCE_RTTI, 1]]
        );
        assert_eq!(calls_to(&log, TEMP_EFFECT_INITIALIZE), vec![vec![this]]);
        assert_eq!((e.mem.u32(this + 0x2c), e.mem.u32(this + 0x30)), (0, 0));
        // A skin object: its answer 0x100 has a zero low byte (unskinned), 1 skinned.
        let holder = e.mem.alloc(0x40);
        let skin = e.mem.alloc(4);
        e.mem.set_u32(target + 0xb8, holder);
        e.mem.set_u32(holder + 0x30, skin);
        let mut slots = vec![0u32; 0x24];
        slots[0x8c / 4] = 0x0150_108c;
        e.put_vtable(0x0150_0000, &slots);
        e.mem.set_u32(skin, 0x0150_0000);
        e.register(0x0150_108c, |_, _| ret(0x100));
        e.mem.set_u32(this + 0x60, 0);
        reset(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0068_8660, &args![this]);
        assert!(calls_to(&e.call_log.take().unwrap(), IS_KIND_0045BAD0).is_empty());
        e.register(0x0150_108c, |_, _| ret(1));
        reset(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0068_8660, &args![this]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), IS_KIND_0045BAD0),
            vec![vec![DISMEMBER_SKIN_INSTANCE_RTTI, 1]]
        );
        e.mem.set_u32(this + 0x60, 1);
        // Already initialized or a small answer: no build.
        e.mem.set_u8(this + 0x14, 1);
        reset(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0068_8660, &args![this]);
        assert!(calls_to(&e.call_log.take().unwrap(), IS_KIND_0045BAD0).is_empty());
        e.mem.set_u8(this + 0x14, 0);
        e.mem.set_u32(parent + 8, 1);
        reset(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0068_8660, &args![this]);
        assert!(calls_to(&e.call_log.take().unwrap(), IS_KIND_0045BAD0).is_empty());
    }

    #[test]
    fn destructor_detaches_and_frees() {
        let mut e = Engine::new();
        pointer_doubles(&mut e);
        noop(
            &mut e,
            &[
                POINTER_DESTRUCT_0045CEC0,
                TEMP_EFFECT_DESTRUCTOR,
                OPERATOR_DELETE,
            ],
        );
        let parent = e.mem.alloc(4);
        let mut slots = vec![0u32; 0x3b];
        slots[0xe8 / 4] = 0x0150_10e8;
        e.put_vtable(0x0150_0000, &slots);
        e.mem.set_u32(parent, 0x0150_0000);
        e.register(0x0150_10e8, |_, _| Ret::default());
        let geometry = e.mem.alloc(0x10);
        e.mem.set_u32(geometry + 0xc, parent);
        e.register(PARENT_009611E0, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        let this = e.mem.alloc(0x64);
        e.mem.set_u32(this + 0x1c, geometry);
        e.mem.set_u32(this + 0x24, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x0068_8770, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), GEOMETRY_DECAL_VTABLE);
        assert_eq!(calls_to(&log, 0x0150_10e8), vec![vec![parent, geometry]]);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![0x4444]]);
        assert_eq!(
            calls_to(&log, POINTER_DESTRUCT_0045CEC0),
            vec![
                vec![this + 0x34],
                vec![this + 0x30],
                vec![this + 0x2c],
                vec![this + 0x1c]
            ]
        );
        assert_eq!(calls_to(&log, TEMP_EFFECT_DESTRUCTOR), vec![vec![this]]);
        // Nothing held: no detach, no free.
        e.call_log = Some(vec![]);
        e.call(0x0068_8770, &args![this]);
        e.mem.set_u32(this + 0x24, 0);
        e.call(0x0068_8770, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0150_10e8).is_empty());
        assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), 1);
    }

    #[test]
    fn update_fades_then_finishes() {
        let mut e = Engine::new();
        pointer_doubles(&mut e);
        e.register(TEMP_EFFECT_UPDATE, |e, a| {
            let age = e.mem.f32(a[0] + 0x10) + f32::from_bits(a[1]);
            e.mem.set_f32(a[0] + 0x10, age);
            ret(1)
        });
        let property = e.mem.alloc(0xc0);
        e.register_double(GET_PROPERTY, move |_, _| ret(property));
        let mut slots = vec![0u32; 0x3b];
        slots[0x98 / 4] = 0x0150_1098;
        slots[0xe8 / 4] = 0x0150_10e8;
        e.put_vtable(0x0150_0000, &slots);
        e.register(0x0150_1098, |_, _| ret(0x9898));
        e.register(0x0150_10e8, |_, _| Ret::default());
        e.register(PARENT_009611E0, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(PARENT_NOTIFY_004EE4F0, |_, _| Ret::default());
        e.register(NODE_TEST_00453470, |_, _| ret(0));
        e.map(DECAL_COUNT_011C57F0 & !0xfff, 0x1000);
        e.set_global(DECAL_COUNT_011C57F0, 5u32);
        let this = e.mem.alloc(0x64);
        e.mem.set_u32(this, 0x0150_0000);
        e.mem.set_f32(this + 8, 2.0);
        let decal_parent = e.mem.alloc(0x10);
        e.mem.set_u32(decal_parent, 0x0150_0000);
        let grand_parent = e.mem.alloc(0x10);
        e.mem.set_u32(grand_parent, 0x0150_0000);
        e.mem.set_u32(decal_parent + 0xc, grand_parent);
        e.mem.set_u32(this + 0x34, decal_parent);
        e.mem.set_u32(this + 0x1c, 0x1c1c);
        // Age 1 of 2: half faded, not finished.
        e.call_log = Some(vec![]);
        assert!(e.call(0x0068_88a0, &args![this, 1.0f32.to_bits()]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.f32(property + 0xbc), 0.5);
        assert_eq!(calls_to(&log, GET_PROPERTY), vec![vec![0x9898, 3]]);
        assert!(calls_to(&log, 0x0150_10e8).is_empty());
        // Age 3 of 2: detached and finished.
        e.call_log = Some(vec![]);
        assert!(e.call(0x0068_88a0, &args![this, 2.0f32.to_bits()]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0150_10e8),
            vec![vec![decal_parent, 0x1c1c], vec![grand_parent, decal_parent]]
        );
        assert_eq!(
            calls_to(&log, PARENT_NOTIFY_004EE4F0),
            vec![vec![grand_parent, this]]
        );
        assert_eq!(e.global::<u32>(DECAL_COUNT_011C57F0), 4);
        assert_eq!(e.mem.u8(this + 0x28), 1);
        // Finished: false without anything else.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0068_88a0, &args![this, 1.0f32.to_bits()]).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    fn save_globals(e: &mut Engine, blocks: u32, report: u8) -> u32 {
        e.map(SAVE_LOAD_GAME & !0xfff, 0x1000);
        e.set_global(SAVE_LOAD_GAME, 0x5a5au32);
        e.register_double(USE_SAVE_GAME_BLOCKS, move |_, _| ret(blocks));
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, report);
        e.register_double(REPORT_FLAG_00408D60, move |_, _| ret(flag));
        noop(e, &[ERROR_REPORT, PRINT_WARNING_005B5E40]);
        flag
    }

    #[test]
    fn save_size_and_its_reports() {
        let mut e = Engine::new();
        save_globals(&mut e, 1, 0);
        assert_eq!(e.call(0x0068_8a10, &args![0u32]).u16(), 6);
        let mut e = Engine::new();
        save_globals(&mut e, 0, 1);
        e.register(SAVING_RECORD_004FD3E0, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_8a10, &args![0u32]).u16(), 0);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), ERROR_REPORT),
            vec![vec![SIZE_REPORT, 0, 0x72f, SOURCE_FILE_NAME]]
        );
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record, 0x77);
        e.mem.set_u32(record + 5, 0xf1a9);
        e.register_double(SAVING_RECORD_004FD3E0, move |_, _| ret(record));
        let form = e.mem.alloc(4);
        let mut slots = vec![0u32; 0x4d];
        slots[0x130 / 4] = 0x0150_1130;
        e.put_vtable(0x0150_0000, &slots);
        e.mem.set_u32(form, 0x0150_0000);
        e.register(0x0150_1130, |_, _| ret(0x7e47));
        e.register_double(LOOKUP_FORM_004839C0, move |_, _| ret(form));
        e.call_log = Some(vec![]);
        e.call(0x0068_8a10, &args![0u32]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), ERROR_REPORT),
            vec![vec![
                SIZE_REPORT_FORM,
                0,
                0x77,
                0x7e47,
                0xf1a9,
                0x72f,
                SOURCE_FILE_NAME
            ]]
        );
    }

    #[test]
    fn save_game_writes_the_block_header_and_size() {
        let mut e = Engine::new();
        save_globals(&mut e, 1, 1);
        e.register(SAVING_RECORD_004FD3E0, |_, _| ret(0));
        // The buffer: a position that each write advances.
        let buffer = e.mem.alloc(0x20);
        let position = std::rc::Rc::new(std::cell::Cell::new(buffer));
        let p = position.clone();
        e.register_double(BUFFER_POSITION_00825C00, move |_, _| ret(p.get()));
        let p = position.clone();
        let written = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let w = written.clone();
        e.register_double(SAVE_DATA_008579B0, move |e, a| {
            w.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            p.set(p.get() + a[2]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0068_8b00, &args![0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(*written.borrow(), vec![b"KOLB".to_vec(), vec![0, 0]]);
        assert_eq!(
            calls_to(&log, ERROR_REPORT),
            vec![vec![SAVE_REPORT, 6, 0x783, SOURCE_FILE_NAME]]
        );
        // The placeholder at buffer + 4 holds the bytes after it: 2.
        assert_eq!(e.mem.u16(buffer + 4), 2);
        assert!(calls_to(&log, PRINT_WARNING_005B5E40).is_empty());
    }

    #[test]
    fn load_game_checks_the_block() {
        let mut e = Engine::new();
        save_globals(&mut e, 1, 0);
        e.register(LOADING_RECORD_004FD3C0, |_, _| ret(0));
        e.register(VERSION_008DF040, |_, _| ret(0x1e));
        let position = std::rc::Rc::new(std::cell::Cell::new(0x1000u32));
        let p = position.clone();
        e.register_double(BUFFER_POSITION_00825C00, move |_, _| ret(p.get()));
        let p = position.clone();
        let header = std::rc::Rc::new(std::cell::Cell::new(BLOCK_TAG));
        let h = header.clone();
        e.register_double(LOAD_DATA_008579E0, move |e, a| {
            if a[2] == 4 {
                e.mem.set_u32(a[1], h.get());
            } else {
                e.mem.set_u16(a[1], 8);
            }
            p.set(p.get() + a[2]);
            Ret::default()
        });
        // Position after the header 0x1004, the block ends at 0x100c; now 0x1006: underrun of 6.
        e.call_log = Some(vec![]);
        assert!(e.call(0x0068_8c90, &args![0u32]).bool());
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), PRINT_WARNING_005B5E40),
            vec![vec![UNDERRUN, 6, SOURCE_FILE_NAME, 0x838, 0x1e]]
        );
        // A wrong header, and the position moved past the end: overrun.
        header.set(0x1234);
        position.set(0x1000);
        e.register_double(BUFFER_POSITION_00825C00, {
            let p = position.clone();
            let calls = std::rc::Rc::new(std::cell::Cell::new(0));
            move |_, _| {
                calls.set(calls.get() + 1);
                ret(if calls.get() == 1 {
                    p.get()
                } else {
                    p.get() + 0x10
                })
            }
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0068_8c90, &args![0u32]).bool());
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), PRINT_WARNING_005B5E40),
            vec![
                vec![BAD_HEADER, SOURCE_FILE_NAME, 0x78c, 0x1e],
                vec![OVERRUN, 0xa, SOURCE_FILE_NAME, 0x838, 0x1e]
            ]
        );
        // With a loading record and an exact block: the form is still looked up, no warning.
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record, 0x77);
        e.register_double(LOADING_RECORD_004FD3C0, move |_, _| ret(record));
        e.register(LOOKUP_FORM_004839C0, |_, _| ret(0x4000));
        header.set(BLOCK_TAG);
        let p = position.clone();
        e.register_double(BUFFER_POSITION_00825C00, move |_, _| ret(p.get()));
        position.set(0x1000);
        e.register_double(LOAD_DATA_008579E0, {
            let p = position.clone();
            move |e, a| {
                if a[2] == 4 {
                    e.mem.set_u32(a[1], BLOCK_TAG);
                } else {
                    e.mem.set_u16(a[1], 2);
                }
                p.set(p.get() + a[2]);
                Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x0068_8c90, &args![0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOOKUP_FORM_004839C0), vec![vec![0x77]]);
    }

    #[test]
    fn skin_object_of_a_geometry() {
        let mut e = Engine::new();
        pointer_doubles(&mut e);
        e.register(POINTER_AT_30_004A8A90, |e, a| ret(e.mem.u32(a[0] + 0x30)));
        let geometry = e.mem.alloc(0xc0);
        assert_eq!(e.call(0x0068_8730, &args![geometry]).u32(), 0);
        let holder = e.mem.alloc(0x40);
        e.mem.set_u32(holder + 0x30, 0x5151);
        e.mem.set_u32(geometry + 0xb8, holder);
        assert_eq!(e.call(0x0068_8730, &args![geometry]).u32(), 0x5151);
    }
}

#[cfg(test)]
mod tests_n073_skinned {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Default::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The doubles of [`fn_00687d90`]; returns the decal, the shape and the bones.
    pub(super) fn setup(e: &mut Engine, partition_ok: u32) -> (u32, u32, u32) {
        e.register(WORD_AT_0_00559450, |e, a| ret(e.mem.u32(a[0])));
        e.register(POINTER_SET_0066B0D0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_CONSTRUCT_00633C90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MEMORY_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        let shape = e.mem.alloc(0xd0);
        let mut slots = vec![0u32; 0x30];
        slots[0xbc / 4] = 0x0150_10bc;
        e.put_vtable(0x0150_0000, &slots);
        e.mem.set_u32(shape, 0x0150_0000);
        e.register(0x0150_10bc, |_, _| Ret::default());
        e.register_double(NI_TRI_SHAPE_CONSTRUCT, move |_, _| ret(shape));
        e.map(TRANSIENT_NAME_WORD & !0xfff, 0x1000);
        e.set_global(TRANSIENT_NAME_WORD, 0x4a4eu32);
        e.register(SHADER_PROPERTY_CONSTRUCT_00BB4080, |_, a| ret(a[0]));
        // The texture set's +0x30 object: slot 0x90 stores a texture.
        let mut slots = vec![0u32; 0x25];
        slots[0x90 / 4] = 0x0150_1090;
        e.put_vtable(0x0150_0100, &slots);
        e.register(0x0150_1090, |e, a| {
            e.mem.set_u32(a[2], 0x7e7);
            Ret::default()
        });
        e.register(IS_KIND_0043B300, |_, _| ret(1));
        e.register(GEOMETRY_TEST_0043FAD0, |_, _| ret(0x5c1));
        e.register(SKIN_DATA_OF_005585E0, |_, _| ret(0x5d0));
        e.register(BONE_COUNT_008041A0, |_, _| ret(2));
        e.register(ROOT_0048D150, |_, _| ret(0x2007));
        e.register(SKIN_DATA_CONSTRUCT_00A854C0, |_, a| ret(a[0]));
        let bones_source = e.mem.alloc(8);
        e.mem.set_u32(bones_source, 0xb0);
        e.mem.set_u32(bones_source + 4, 0xb1);
        e.register_double(BONES_00825C00, move |_, _| ret(bones_source));
        e.register(SKIN_ROOT_0044EDB0, |_, _| ret(0x2004));
        for address in [
            NI_OBJECT_CONSTRUCTOR,
            SET_NAME_00A5B950,
            ATTACH_PROPERTY_00439410,
            PROPERTY_SET_FLOAT_004EE4B0,
            SET_PARTITION_004ADDC0,
            SET_SKIN_INSTANCE_004ADDE0,
            SET_TRANSLATE_00440460,
            SET_ROTATE_0043FA80,
            SET_SCALE_00440490,
            SET_WORLD_POSITION_004F00E0,
            SET_FLOAT_006427A0,
            UPDATE_PROPERTIES_00A5A040,
            PREPARE_OBJECT,
            POINTER_DESTRUCT_0045CEC0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(PARTITION_CONSTRUCT_00A81F90, |_, a| ret(a[0]));
        e.register(GEOMETRY_DATA_005495F0, |_, _| ret(0xda7a));
        e.register_double(PARTITION_MAKE_00A84980, move |_, _| ret(partition_ok));
        e.register(TRANSLATION_0043C490, |_, _| ret(0x7a));
        e.register(ROTATION_006A9540, |_, _| ret(0x70));
        e.register(SCALE_009CDAE0, |_, _| Ret {
            st0: 2.0,
            ..Default::default()
        });
        e.register(WORLD_POSITION_0045BB80, |_, _| ret(0x9050));
        let transform = e.mem.alloc(0x24);
        e.register_double(WORLD_TRANSFORM_00461130, move |_, _| ret(transform));
        e.register(VALUE_008D01E0, |_, _| Ret {
            st0: 0.5,
            ..Default::default()
        });
        let this = e.mem.alloc(0x64);
        let texture_set = e.mem.alloc(0x40);
        e.mem.set_u32(texture_set + 0x30, 0x0150_0100);
        e.mem.set_u32(this + 0x58, texture_set);
        e.mem.set_u32(this + 0x2c, 0x7a67);
        e.mem.set_u32(this + 0x60, 0b1010);
        (this, shape, bones_source)
    }

    #[test]
    fn builds_the_shape_with_body_part_partitions() {
        let mut e = Engine::new();
        let (this, shape, _) = setup(&mut e, 1);
        e.call_log = Some(vec![]);
        e.call(
            0x0068_7d90,
            &args![
                this,
                0x50u32,
                0x10007u32,
                0x1_0009u32,
                0x100u32,
                0x200u32,
                0x300u32,
                0x400u32,
                0u32,
                0u32
            ],
        );
        let log = e.call_log.take().unwrap();
        let shape_call = calls_to(&log, NI_TRI_SHAPE_CONSTRUCT);
        assert_eq!(shape_call[0][1..], [7, 0x100, 0x200, 0, 0, 0, 0, 3, 0x300]);
        assert_eq!(calls_to(&log, SET_NAME_00A5B950), vec![vec![shape, 0x4a4e]]);
        let property = calls_to(&log, SHADER_PROPERTY_CONSTRUCT_00BB4080)[0][0];
        assert_eq!(e.mem.u32(property + 0x7c), 0x7e7);
        assert_eq!(
            calls_to(&log, SKIN_DATA_CONSTRUCT_00A854C0)[0][1..],
            [2, 0x400, 0x2007, 0x100]
        );
        // iBodyParts 0b1010: bits 1 and 3 give parts 0 and 2.
        let instance = calls_to(&log, SET_SKIN_INSTANCE_004ADDE0)[0][1];
        assert_eq!(e.mem.u32(instance + 0x34), 2);
        let partitions = e.mem.u32(instance + 0x38);
        assert_eq!(e.mem.bytes(partitions, 8), vec![1, 1, 0, 0, 1, 1, 2, 0]);
        let bones = e.mem.u32(instance + 0x14);
        assert_eq!((e.mem.u32(bones), e.mem.u32(bones + 4)), (0xb0, 0xb1));
        assert_eq!(e.mem.u32(instance + 0x10), 0x2004);
        assert_eq!(
            calls_to(&log, PARTITION_MAKE_00A84980)[0][1..],
            [
                0xda7a,
                calls_to(&log, SKIN_DATA_CONSTRUCT_00A854C0)[0][0],
                0x12,
                4,
                1,
                0,
                0,
                0
            ]
        );
        assert_eq!(
            calls_to(&log, SET_SCALE_00440490),
            vec![vec![shape, 2.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, SET_FLOAT_006427A0),
            vec![vec![shape, 0.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, PREPARE_OBJECT), vec![vec![shape, 0, 0]]);
        assert_eq!(e.mem.u32(this + 0x1c), shape);
        assert_eq!(e.mem.u32(this + 0x20), 0x7a67);
    }

    #[test]
    fn a_given_list_and_parts_or_a_failed_partition() {
        let mut e = Engine::new();
        let (this, _, _) = setup(&mut e, 0);
        e.register(LIST_COUNT_0044DDC0, |_, _| ret(1));
        let parts = e.mem.alloc(2);
        e.mem.set_u16(parts, 0x33);
        e.call_log = Some(vec![]);
        e.call(
            0x0068_7d90,
            &args![
                this, 0x50u32, 7u32, 9u32, 0x100u32, 0x200u32, 0x300u32, 0x400u32, 0x1157u32, parts
            ],
        );
        let log = e.call_log.take().unwrap();
        let instance = calls_to(&log, SET_SKIN_INSTANCE_004ADDE0)[0][1];
        assert_eq!(
            calls_to(&log, SET_PARTITION_004ADDC0),
            vec![vec![instance, 0x1157]]
        );
        assert_eq!(
            e.mem.bytes(e.mem.u32(instance + 0x38), 4),
            vec![1, 1, 0x33, 0]
        );
        assert!(calls_to(&log, PARTITION_MAKE_00A84980).is_empty());
        // No list and a failed partition: both are deleted, no skin.
        let mut e = Engine::new();
        let (this, _, _) = setup(&mut e, 0);
        e.put_vtable(0x0150_0400, &[0x0150_1400]);
        e.register(0x0150_1400, |_, _| Ret::default());
        e.register(PARTITION_CONSTRUCT_00A81F90, |e, a| {
            e.mem.set_u32(a[0], 0x0150_0400);
            ret(a[0])
        });
        e.put_vtable(
            BS_DISMEMBER_SKIN_INSTANCE_VTABLE,
            &[BS_DISMEMBER_PLACEHOLDER],
        );
        e.register(BS_DISMEMBER_PLACEHOLDER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(
            0x0068_7d90,
            &args![this, 0x50u32, 7u32, 9u32, 0x100u32, 0x200u32, 0x300u32, 0x400u32, 0u32, 0u32],
        );
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SET_SKIN_INSTANCE_004ADDE0).is_empty());
        assert_eq!(calls_to(&log, 0x0150_1400).len(), 1);
        assert_eq!(calls_to(&log, BS_DISMEMBER_PLACEHOLDER)[0][1], 1);
        assert_eq!(e.mem.u32(this + 0x1c), shape_of(&log));
    }

    /// The deleting destructor double put in the instance's vtable (the
    /// game's table, mapped by the test).
    const BS_DISMEMBER_PLACEHOLDER: u32 = 0x0150_1500;

    fn shape_of(log: &[(u32, Vec<u32>)]) -> u32 {
        calls_to(log, PREPARE_OBJECT)[0][0]
    }
}

#[cfg(test)]
mod tests_n073_dismembered {
    use super::*;

    fn identity(e: &mut Engine, at: u32) {
        for i in 0..9 {
            e.mem
                .set_f32(at + 4 * i, if i % 4 == 0 { 1.0 } else { 0.0 });
        }
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Default::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let block = e.mem.alloc(4 * values.len() as u32);
        for (i, v) in (0u32..).zip(values) {
            e.mem.set_f32(block + 4 * i, *v);
        }
        block
    }

    fn words16(e: &mut Engine, values: &[u16]) -> u32 {
        let block = e.mem.alloc(2 * values.len() as u32 + 2);
        for (i, v) in (0u32..).zip(values) {
            e.mem.set_u16(block + 2 * i, *v);
        }
        block
    }

    /// A one-partition, one-triangle, one-bone model whose triangle faces
    /// away from the decal's vector (so it is kept).
    pub(super) fn model(e: &mut Engine) -> (u32, u32) {
        let (this, _, _) = super::tests_n073_skinned::setup(e, 1);
        e.mem.set_u32(this + 0x60, 0b10);
        e.mem.set_f32(this + 0x44, 0.0);
        e.mem.set_f32(this + 0x48, 0.0);
        e.mem.set_f32(this + 0x4c, 1.0);
        e.mem.set_f32(this + 0x50, 1.0);
        for page in [
            HALF_DOUBLE,
            ZERO_DOUBLE,
            DEPTH_LIMIT,
            TWO_FIFTY_SIX,
            POINT_ZERO,
        ] {
            e.map(page & !0xfff, 0x1000);
        }
        e.set_global(HALF_DOUBLE, 0.5f64);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(DEPTH_LIMIT, 0.1f64);
        e.set_global(TWO_FIFTY_SIX, 256.0f64);
        e.register(IS_KIND_0045BAD0, |_, _| ret(1));
        e.register(FIRST_CONTROLLER_0043B230, |_, _| ret(0xc0c0));
        e.register(LIST_COUNT_0044DDC0, |_, _| ret(1));
        // The source partition.
        let parts = e.mem.alloc(0x2c);
        let triangles = words16(e, &[0, 1, 2]);
        let vertex_map = words16(e, &[0, 1, 2]);
        let weights = floats(e, &[1.0, 1.0, 1.0]);
        let indices = e.mem.alloc(4);
        let bone_list = words16(e, &[0]);
        e.mem.set_u32(parts + 4, bone_list);
        e.mem.set_u32(parts + 8, weights);
        e.mem.set_u32(parts + 0xc, vertex_map);
        e.mem.set_u32(parts + 0x10, indices);
        e.mem.set_u32(parts + 0x14, triangles);
        e.mem.set_u16(parts + 0x1e, 1);
        e.mem.set_u16(parts + 0x20, 1);
        e.mem.set_u16(parts + 0x22, 0);
        e.mem.set_u16(parts + 0x24, 1);
        e.register_double(FORM_ID_0084E3A0, move |_, _| ret(parts));
        let flags = e.mem.alloc(4);
        e.mem.set_u8(flags, 1);
        e.register_double(PARTITION_FLAGS_009E32D0, move |_, _| ret(flags));
        e.register(LOCAL_INIT_006815C0, |_, a| ret(a[0]));
        // Identity transforms and rotations.
        let transform = floats(
            e,
            &[
                1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );
        e.register_double(WORLD_TRANSFORM_00461130, move |_, _| ret(transform));
        e.register(MATRIX_INVERSE_004B45B0, |e, a| {
            identity(e, a[1]);
            Ret::default()
        });
        e.register(MATRIX_TIMES_POINT_004B4500, |e, a| {
            copy_point(e, a[2], a[1]);
            ret(a[1])
        });
        e.register(MATRIX_ANGLE_004A0C90, |_, _| Ret::default());
        e.register(ROTATION_Z_TO_VECTOR, |e, a| {
            identity(e, a[0]);
            ret(a[0])
        });
        e.register(MATRIX_MULTIPLY_0043F8D0, |e, a| {
            identity(e, a[1]);
            ret(a[1])
        });
        e.register(SET_TRANSLATION_004578C0, |e, a| {
            copy_point(e, a[1], a[0]);
            Ret::default()
        });
        e.register(POINT_TIMES_FLOAT_00439180, |_, _| Ret::default());
        e.register(TRANSFORM_VERTICES, |e, a| {
            copy_point(e, a[3], a[4]);
            Ret::default()
        });
        e.register(MAX_00404010, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))),
            ..Default::default()
        });
        e.register(ABS_00408840, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).abs()),
            ..Default::default()
        });
        // One bone with the three vertices.
        let bone_data = e.mem.alloc(0x4c);
        let bone_weights = e.mem.alloc(0x18);
        for i in 0..3 {
            e.mem.set_u16(bone_weights + 8 * i, i as u16);
            e.mem.set_f32(bone_weights + 8 * i + 4, 1.0);
        }
        e.mem.set_u32(bone_data + 0x44, bone_weights);
        e.mem.set_u16(bone_data + 0x48, 3);
        e.register_double(BONE_DATA_008D6F30, move |_, _| ret(bone_data));
        e.register(BONE_COUNT_008041A0, |_, _| ret(1));
        e.register(VERTEX_COUNT_004A8AE0, |_, _| ret(3));
        let source_positions = floats(e, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        e.register_double(WORD_AT_20_007AF430, move |_, _| ret(source_positions));
        e.register(WORD_AT_24_0059BB30, |_, _| ret(0));
        // The deformed vertices in the decal's square, normals away from the vector.
        e.register(SKIN_SETUP_00A855F0, |e, a| {
            let points = [0.25f32, 0.25, 0.0, 0.5, 0.25, 0.0, 0.25, 0.5, 0.0];
            for (i, v) in (0u32..).zip(points) {
                e.mem.set_f32(a[4] + 4 * i, v);
            }
            for i in 0..3 {
                e.mem.set_f32(a[5] + 0xc * i + 8, -1.0);
            }
            Ret::default()
        });
        e.register(FILL_MEMORY_00403D30, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(EH_VECTOR_CONSTRUCT_00EC782F, |_, _| Ret::default());
        e.register(VECTOR_CONSTRUCT_00401050, |_, _| Ret::default());
        e.register(NI_ALLOCATE_00AA1070, |e, a| ret(e.mem.alloc(a[0].max(1))));
        e.register(COPY_MEMORY_00401460, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        (this, parts)
    }

    #[test]
    fn builds_the_kept_partition_and_geometry() {
        let mut e = Engine::new();
        let (this, _) = model(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0068_5ef0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x5c), 3);
        let shape = calls_to(&log, NI_TRI_SHAPE_CONSTRUCT);
        assert_eq!(shape.len(), 1);
        assert_eq!((shape[0][1], shape[0][8]), (3, 1));
        // The kept vertices: the source positions, and the projected values
        // (x + 0.5, y + 0.5, facing 1 and no depth: 1).
        let positions = shape[0][2];
        assert_eq!(e.mem.f32(positions + 0x18), 7.0);
        let projected = shape[0][3];
        assert_eq!(
            (0..3)
                .map(|i| e.mem.f32(projected + 4 * i))
                .collect::<Vec<_>>(),
            vec![0.75, 0.75, 1.0]
        );
        let triangles = shape[0][9];
        assert_eq!(
            (0..3)
                .map(|i| e.mem.u16(triangles + 2 * i))
                .collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
        // The new partition holds one 0x2C-byte partition of the triangle.
        let instance = calls_to(&log, SET_SKIN_INSTANCE_004ADDE0)[0][1];
        let list = calls_to(&log, SET_PARTITION_004ADDC0)[0][1];
        let partitions = e.mem.u32(list + 0xc);
        assert_eq!(e.mem.u32(list + 8), 1);
        assert_eq!(e.mem.u16(partitions + 0x1e), 1);
        assert_eq!(e.mem.u16(partitions + 0x1c), 3);
        assert_eq!(e.mem.u16(partitions + 0x24), 1);
        assert_eq!(e.mem.f32(e.mem.u32(partitions + 8) + 8), 1.0);
        // The instance's partitions: {1, 1, body part 0}.
        assert_eq!(e.mem.bytes(e.mem.u32(instance + 0x38), 4), vec![1, 1, 0, 0]);
        assert!(calls_to(&log, DEBUG_BREAK).is_empty());
    }

    #[test]
    fn stops_early_without_body_parts_skin_or_an_enabled_part() {
        let mut e = Engine::new();
        let (this, _) = model(&mut e);
        e.mem.set_u32(this + 0x60, 0);
        e.call_log = Some(vec![]);
        e.call(0x0068_5ef0, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u32(this + 0x60, 0b100);
        e.call_log = Some(vec![]);
        e.call(0x0068_5ef0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MEMORY_ALLOCATE).is_empty());
        e.mem.set_u32(this + 0x60, 0b10);
        e.register(IS_KIND_0045BAD0, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0068_5ef0, &args![this]);
        assert!(calls_to(&e.call_log.take().unwrap(), FIRST_CONTROLLER_0043B230).is_empty());
    }

    #[test]
    fn a_triangle_off_the_square_keeps_nothing() {
        let mut e = Engine::new();
        let (this, _) = model(&mut e);
        // The normals face the vector: no fade, so no corner counts its depth.
        e.register(SKIN_SETUP_00A855F0, |e, a| {
            let points = [2.25f32, 2.25, 0.0, 2.5, 2.25, 0.0, 2.25, 2.5, 0.0];
            for (i, v) in (0u32..).zip(points) {
                e.mem.set_f32(a[4] + 4 * i, v);
            }
            for i in 0..3 {
                e.mem.set_f32(a[5] + 0xc * i + 8, 1.0);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0068_5ef0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, NI_TRI_SHAPE_CONSTRUCT).is_empty());
        assert_eq!(e.mem.u32(this + 0x5c), 0);
    }
}

#[cfg(test)]
mod tests_n073_streams {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Default::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The dismembered model, read through streams: the weights stream holds
    /// (0.5, 0.25, 0.25) per vertex, the bone index stream zeros.
    fn stream_model(e: &mut Engine) -> u32 {
        let (this, parts) = super::tests_n073_dismembered::model(e);
        e.mem.set_u16(parts + 0x1c, 3);
        e.map(SMALL_WEIGHT & !0xfff, 0x1000);
        e.set_global(SMALL_WEIGHT, 0.0001f64);
        e.register(SKIN_PARTITION_DATA_004A8A90, |_, _| ret(0x9a9a));
        e.register(STREAM_VERTEX_COUNT_0047D370, |_, _| ret(3));
        e.register(STREAM_CONSTRUCT_00534550, |_, _| Ret::default());
        e.register(DATA_LOCK_00A67360, |_, _| ret(1));
        e.register(DATA_UNLOCK_00A673D0, |_, _| Ret::default());
        for address in [DATA_STREAM_00A67410, DATA_STREAM_00A675B0] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(DATA_STREAM_00A67720, |_, _| Ret::default());
        let weights = e.mem.alloc(0x24);
        for v in 0..3 {
            e.mem.set_f32(weights + 0xc * v, 0.5);
            e.mem.set_f32(weights + 0xc * v + 4, 0.25);
            e.mem.set_f32(weights + 0xc * v + 8, 0.25);
        }
        e.register_double(DATA_STREAM_00A674B0, move |e, a| {
            e.mem.set_u32(a[1], weights);
            e.mem.set_u32(a[1] + 4, 0xc);
            Ret::default()
        });
        let bones = e.mem.alloc(0xc);
        e.register_double(DATA_STREAM_00A67530, move |e, a| {
            e.mem.set_u32(a[1], bones);
            e.mem.set_u32(a[1] + 4, 4);
            Ret::default()
        });
        e.register(INDEX_CHECK_004534F0, |_, _| Ret::default());
        e.register(COLOR_COMPONENT_00560D30, |_, a| ret(a[0] + 4 * a[1]));
        e.register(SKIN_DEFORM_00A85510, |e, a| {
            let points = [0.25f32, 0.25, 0.0, 0.5, 0.25, 0.0, 0.25, 0.5, 0.0];
            for (i, v) in (0u32..).zip(points) {
                e.mem.set_f32(a[6] + 4 * i, v);
            }
            for i in 0..3 {
                e.mem.set_f32(a[7] + 0xc * i + 8, -1.0);
            }
            Ret::default()
        });
        e.register(STREAM_VALUE_00534570, |e, a| {
            e.mem.set_f32(a[2], 10.0 + a[1] as f32);
            Ret::default()
        });
        this
    }

    #[test]
    fn builds_from_the_streams_and_adds_the_bone_weights() {
        let mut e = Engine::new();
        let this = stream_model(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0068_3c10, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x5c), 3);
        assert_eq!(calls_to(&log, DATA_LOCK_00A67360), vec![vec![0xda7a, 0]]);
        assert_eq!(calls_to(&log, DATA_UNLOCK_00A673D0), vec![vec![0xda7a]]);
        let shape = calls_to(&log, NI_TRI_SHAPE_CONSTRUCT);
        assert_eq!((shape[0][1], shape[0][8]), (3, 1));
        // The kept positions come from the first stream.
        let positions = shape[0][2];
        assert_eq!(e.mem.f32(positions + 0x18), 12.0);
        // Bone 0 gathers each vertex's three non-zero weights.
        let bones = calls_to(&log, SKIN_DATA_CONSTRUCT_00A854C0)[0][2];
        assert_eq!(e.mem.u16(bones + 0x48), 9);
        let entries = e.mem.u32(bones + 0x44);
        assert_eq!((e.mem.u16(entries), e.mem.f32(entries + 4)), (0, 0.5));
        assert_eq!(
            (e.mem.u16(entries + 0x40), e.mem.f32(entries + 0x44)),
            (2, 0.25)
        );
        // The partition keeps the weights (four per vertex, the fourth 0).
        let list = calls_to(&log, SET_PARTITION_004ADDC0)[0][1];
        let partition = e.mem.u32(list + 0xc);
        assert_eq!(e.mem.u16(partition + 0x1c), 3);
        let weights = e.mem.u32(partition + 8);
        assert_eq!(
            (0..4)
                .map(|i| e.mem.f32(weights + 4 * i))
                .collect::<Vec<_>>(),
            vec![0.5, 0.25, 0.25, 0.0]
        );
    }

    #[test]
    fn stops_without_dismember_skin_or_bone_data() {
        let mut e = Engine::new();
        let this = stream_model(&mut e);
        e.register(BONE_DATA_008D6F30, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0068_3c10, &args![this]);
        assert!(calls_to(&e.call_log.take().unwrap(), DATA_LOCK_00A67360).is_empty());
        e.register(IS_KIND_0045BAD0, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0068_3c10, &args![this]);
        assert!(calls_to(&e.call_log.take().unwrap(), FIRST_CONTROLLER_0043B230).is_empty());
    }
}
