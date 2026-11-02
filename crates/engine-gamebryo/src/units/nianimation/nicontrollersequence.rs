//! `NiAnimation/nicontrollersequence.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream, NiTPrimitiveSet};

/// `NiControllerSequence::ms_RTTI` (the class's `NiRTTI` record).
const NI_CONTROLLER_SEQUENCE_RTTI: u32 = 0x011f_36dc;

// Translated from 00a31730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_controller_sequence_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_CONTROLLER_SEQUENCE_RTTI)
}

// Translated from 00a35820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a35640`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x74 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_controller_sequence_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a3_5640;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x74u32]);
    }
    this
}

layout! {
    /// `NiControllerSequence` (Xbox PDB), 0x74 bytes on PC as on the Xbox, an `NiObject`
    /// derivative (it is not an `NiSequence`): the name, the interpolator and ID-tag arrays,
    /// the timing, the owner and state, and the animation notes.
    pub struct NiControllerSequence: 0x74 {
        /// `m_uiRefCount` (Xbox PDB).
        0x04 m_uiRefCount: u32,
        /// `m_kName` (Xbox PDB): an `NiFixedString` handle.
        0x08 m_kName: u32,
        /// `m_uiArraySize` (Xbox PDB).
        0x0C m_uiArraySize: u32,
        /// `m_uiArrayGrowBy` (Xbox PDB).
        0x10 m_uiArrayGrowBy: u32,
        /// `m_pkInterpArray` (Xbox PDB): `InterpArrayItem` (0x10 bytes each).
        0x14 m_pkInterpArray: Ptr,
        /// `m_pkIDTagArray` (Xbox PDB): `IDTag` (0x14 bytes each).
        0x18 m_pkIDTagArray: Ptr,
        /// `m_fSeqWeight` (Xbox PDB).
        0x1C m_fSeqWeight: f32,
        /// `m_spTextKeys` (Xbox PDB): `NiPointer<NiTextKeyExtraData>`.
        0x20 m_spTextKeys: Ptr,
        /// `m_eCycleType` (Xbox PDB).
        0x24 m_eCycleType: u32,
        /// `m_fFrequency` (Xbox PDB).
        0x28 m_fFrequency: f32,
        /// `m_fBeginKeyTime` (Xbox PDB).
        0x2C m_fBeginKeyTime: f32,
        /// `m_fEndKeyTime` (Xbox PDB).
        0x30 m_fEndKeyTime: f32,
        /// `m_fLastTime` (Xbox PDB).
        0x34 m_fLastTime: f32,
        /// `m_fWeightedLastTime` (Xbox PDB).
        0x38 m_fWeightedLastTime: f32,
        /// `m_fLastScaledTime` (Xbox PDB).
        0x3C m_fLastScaledTime: f32,
        /// `m_pkOwner` (Xbox PDB): `NiControllerManager *`.
        0x40 m_pkOwner: Ptr,
        /// `m_eState` (Xbox PDB): `AnimState`.
        0x44 m_eState: u32,
        /// `m_fOffset` (Xbox PDB).
        0x48 m_fOffset: f32,
        /// `m_fStartTime` (Xbox PDB).
        0x4C m_fStartTime: f32,
        /// `m_fEndTime` (Xbox PDB).
        0x50 m_fEndTime: f32,
        /// `m_fDestFrame` (Xbox PDB).
        0x54 m_fDestFrame: f32,
        /// `m_pkPartnerSequence` (Xbox PDB).
        0x58 m_pkPartnerSequence: Ptr,
        /// `m_kAccumRootName` (Xbox PDB): an `NiFixedString` handle.
        0x5C m_kAccumRootName: u32,
        /// `m_pkAccumRoot` (Xbox PDB): `NiAVObject *`.
        0x60 m_pkAccumRoot: Ptr,
        /// `m_spDeprecatedStringPalette` (Xbox PDB).
        0x64 m_spDeprecatedStringPalette: Ptr,
        /// `ssCurAnimNIdx` (Xbox PDB).
        0x68 ssCurAnimNIdx: i16,
        /// `spAnimNotesA` (Xbox PDB): `NiPointer<BSAnimNote> *`.
        0x6C spAnimNotesA: Ptr,
        /// `usNumNotes` (Xbox PDB).
        0x70 usNumNotes: u16,
        /// `bRemovableObjects` (Xbox PDB).
        0x72 bRemovableObjects: bool,
    }

    /// `NiControllerSequence::InterpArrayItem` (Xbox PDB), 0x10 bytes.
    pub struct InterpArrayItem: 0x10 {
        /// `m_spInterpolator` (Xbox PDB): `NiPointer<NiInterpolator>`.
        0x00 m_spInterpolator: Ptr,
        /// `m_spInterpCtlr` (Xbox PDB): `NiPointer<NiInterpController>`.
        0x04 m_spInterpCtlr: Ptr,
        /// `m_pkBlendInterp` (Xbox PDB): `NiBlendInterpolator *`.
        0x08 m_pkBlendInterp: Ptr,
        /// `m_ucBlendIdx` (Xbox PDB).
        0x0C m_ucBlendIdx: u8,
        /// `m_ucPriority` (Xbox PDB).
        0x0D m_ucPriority: u8,
    }

    /// `NiControllerSequence::IDTag` (Xbox PDB), 0x14 bytes: five `NiFixedString` handles.
    pub struct IDTag: 0x14 {
        /// `m_kAVObjectName` (Xbox PDB).
        0x00 m_kAVObjectName: u32,
        /// `m_kPropertyType` (Xbox PDB).
        0x04 m_kPropertyType: u32,
        /// `m_kCtlrType` (Xbox PDB).
        0x08 m_kCtlrType: u32,
        /// `m_kCtlrID` (Xbox PDB).
        0x0C m_kCtlrID: u32,
        /// `m_kInterpolatorID` (Xbox PDB).
        0x10 m_kInterpolatorID: u32,
    }
}

/// `NiControllerSequence`'s vtable, the vtable of the `NiTArray<NiPointer<NiInterpController>>` (`01096620`)
/// that `00a34450` destroys and the vtable of the `NiTObjectArray<NiPointer<NiInterpController>>` that
/// `00a352f0` makes on the stack (`01096688`); `00a47600` frees such an array's elements (`cdecl`,
/// `(elements)`).
const NI_CONTROLLER_SEQUENCE_VTABLE: u32 = 0x0109_6584;
const INTERP_CONTROLLER_ARRAY_VTABLE: u32 = 0x0109_6620;
const SEQUENCE_CONTROLLER_ARRAY_VTABLE: u32 = 0x0109_6688;
const POINTER_ARRAY_FREE: u32 = 0x00a4_7600;
/// The memory manager singleton, its `Allocate` (`this` = the manager, `(size)`) and `Deallocate`
/// (`(block)`); the game's `operator new` (`cdecl`, `(size)`), the table allocator (`cdecl`, `(size)`) and
/// its free (`cdecl`, `(block)`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const TABLE_ALLOCATE: u32 = 0x00aa_1070;
const TABLE_FREE: u32 = 0x00aa_10f0;
/// The C runtime: `_vector_constructor_iterator_` (`cdecl`, `(array, element size, count, constructor)`),
/// `atexit` (`cdecl`, `(function)`), `strcpy_s` (`cdecl`, `(destination, size, source)`), `strchr`
/// (`cdecl`, `(text, character)`), `sprintf` and `_CIfmod` (`00ec913a`: the dividend in `ST1`, the
/// divisor in `ST0`, here the two doubles in that order, the result in `ST0`). `InterlockedIncrement`
/// and `InterlockedDecrement` are import slots.
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
const ATEXIT: u32 = 0x00ec_658f;
const STRING_COPY_SECURE: u32 = 0x00ec_65a6;
const STRCHR: u32 = 0x00ec_7690;
const SPRINTF: u32 = 0x00ec_623a;
const CI_FMOD: u32 = 0x00ec_913a;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The word that holds the handle of the empty `NiFixedString`, the static `NiFixedString` at `011f49e0`
/// (set at start-up) and the constructor of a `NiFixedString` from characters (`cdecl`, `(chars)`,
/// answers the handle with a count taken).
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
const FIXED_STRING_011F49E0: u32 = 0x011f_49e0;
const FIXED_STRING_FROM_CHARS: u32 = 0x00a5_b690;
/// The element constructors `_vector_constructor_iterator_` runs: `InterpArrayItem` (`00a31240`),
/// `IDTag` (`00a31680`) and the one of the animation note pointer array (`00b54620`).
const INTERP_ITEM_CONSTRUCTOR: u32 = 0x00a3_1240;
const ID_TAG_CONSTRUCTOR: u32 = 0x00a3_1680;
const POINTER_CONSTRUCTOR: u32 = 0x00b5_4620;
/// `NiStricmp`, `NiStrnicmp` (`cdecl`) and `NiSprintf` (`cdecl`, `(destination, size, format, ...)`).
const NI_STRICMP: u32 = 0x00aa_1360;
const NI_STRNICMP: u32 = 0x00aa_1370;
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// The float constants: `-FLT_MAX` (float `01015f5c`, double `010241b0`), `FLT_MAX` (float `01096548`,
/// double `010231b0`) and the float `0.0001` (`010141cc`) that `00a350d0` raises a non-positive time to.
const NEGATIVE_FLT_MAX_FLOAT: u32 = 0x0101_5f5c;
const NEGATIVE_FLT_MAX_DOUBLE: u32 = 0x0102_41b0;
const FLT_MAX_FLOAT: u32 = 0x0109_6548;
const FLT_MAX_DOUBLE: u32 = 0x0102_31b0;
const SMALL_TIME_010141CC: u32 = 0x0101_41cc;
/// The `NiRTTI` records the code compares: `NiMultiTargetTransformController` (`011f36bc`),
/// `BSMultiTargetTreadTransfController` (`011f36e4`), `NiTransformController` (`011f36ec`),
/// `NiInterpController` (`011f36b4`), `NiBlendInterpolator` (`011f36fc`) and
/// `NiDefaultAVObjectPalette` (`011f4a04`).
const NI_MULTI_TARGET_TRANSFORM_CONTROLLER_RTTI: u32 = 0x011f_36bc;
const BS_MULTI_TARGET_TREAD_TRANSF_CONTROLLER_RTTI: u32 = 0x011f_36e4;
const NI_TRANSFORM_CONTROLLER_RTTI: u32 = 0x011f_36ec;
const NI_INTERP_CONTROLLER_RTTI: u32 = 0x011f_36b4;
const NI_BLEND_INTERPOLATOR_RTTI: u32 = 0x011f_36fc;
const NI_DEFAULT_AV_OBJECT_PALETTE_RTTI: u32 = 0x011f_4a04;
/// The character pointer and the length (`int`) that identify the morph keys' text: a key text is
/// a morph key when `NiStrnicmp(text, *011a8244, *011a8248)` is 0.
const MORPH_TAG_POINTER: u32 = 0x011a_8244;
const MORPH_TAG_LENGTH: u32 = 0x011a_8248;
/// The 16-bit "no index" word (`01096320`) and the byte (`01096968`) that is an item's invalid blend
/// index.
const INVALID_INDEX: u32 = 0x0109_6320;
const INVALID_BLEND_INDEX: u32 = 0x0109_6968;
/// Texts: `"PROP\n"`, `"PROP"`, `"%s\n%s%s"`, `"%s = %s"`, `"BSTreadTransfController"` and `"Bip01"` (`0101e460`).
const TEXT_PROP: u32 = 0x0109_6554;
const TEXT_PROP_MARKER: u32 = 0x0109_6614;
const TEXT_FORMAT_PROP: u32 = 0x0109_654c;
const TEXT_FORMAT_STATE: u32 = 0x0109_655c;
const TEXT_TREAD_TRANSF_CONTROLLER: u32 = 0x0109_6564;
const TEXT_BIP01: u32 = 0x0101_e460;
/// The state names `GetViewerString` prints, by state (`INACTIVE`, `ANIMATING`, `EASEIN`, `EASEOUT`,
/// `TRANSSOURCE`, `TRANSDEST`, `MORPHSOURCE`).
const STATE_NAMES: [u32; 7] = [
    0x0102_0fd4,
    0x0102_0fc8,
    0x0102_0fc0,
    0x0102_0fb8,
    0x0102_0fac,
    0x0102_0fa0,
    0x0102_0f94,
];
/// Function-local statics: the name handle of `"BSTreadTransfController"` (`011f36c4`, guard `011f36c8`,
/// destructor for `atexit` `00fdad60`), the one of `"Bip01"` (`011f36cc`, guard `011f36d0`, `00fdad80`) and
/// the null `NiPointer` that `00a32940` compares with (`011f36d4`, guard `011f36d8`, `00fdada0`).
const TREAD_NAME_STATIC: u32 = 0x011f_36c4;
const TREAD_NAME_GUARD: u32 = 0x011f_36c8;
const TREAD_NAME_ATEXIT: u32 = 0x00fd_ad60;
const BIP01_NAME_STATIC: u32 = 0x011f_36cc;
const BIP01_NAME_GUARD: u32 = 0x011f_36d0;
const BIP01_NAME_ATEXIT: u32 = 0x00fd_ad80;
const NULL_POINTER_STATIC: u32 = 0x011f_36d4;
const NULL_POINTER_GUARD: u32 = 0x011f_36d8;
const NULL_POINTER_ATEXIT: u32 = 0x00fd_ada0;
/// The controller functions called by address: `NiInterpController`'s constructor (`00a2f8c0`),
/// `NiMultiTargetTransformController`'s (`00a2feb0`, `(count)`) that makes its interpolators and
/// targets, `NiMultiTargetTransformController`'s vtable (`01096444`), `BSMultiTargetTreadTransfController`'s
/// constructor (`00a36070`, `(count, sum)`), its `FindIndex` (`00a35910`, `(object)`), `GetInterpolator`
/// (`00a358a0`, `(index)`), add (`00a35fe0`, `(object, counter, record)`) and set (`00a35f70`, `(index,
/// object, counter, record)`).
const INTERP_CONTROLLER_CONSTRUCT: u32 = 0x00a2_f8c0;
const MULTI_TARGET_SET_COUNT: u32 = 0x00a2_feb0;
const NI_MULTI_TARGET_TRANSFORM_CONTROLLER_VTABLE: u32 = 0x0109_6444;
const TREAD_CONTROLLER_CONSTRUCT: u32 = 0x00a3_6070;
const TREAD_CONTROLLER_FIND_INDEX: u32 = 0x00a3_5910;
const TREAD_CONTROLLER_GET_INTERPOLATOR: u32 = 0x00a3_58a0;
const TREAD_CONTROLLER_ADD: u32 = 0x00a3_5fe0;
const TREAD_CONTROLLER_SET: u32 = 0x00a3_5f70;
/// `NiTimeController::SetNext` (`00a67260`, `this` = a controller, `(next)`) and the `NiPointer`
/// assignment that goes through the reference-counting helpers (`0066b0d0`, `this` = the slot,
/// `(object)`).
const CONTROLLER_SET_NEXT: u32 = 0x00a6_7260;
const POINTER_SLOT_ASSIGN: u32 = 0x0066_b0d0;
/// `NiObjectNET`'s `GetController` (`this` = the object, `(rtti)`) and `RemoveController` (`(controller)`).
const OBJECT_NET_GET_CONTROLLER: u32 = 0x00a5_c570;
const OBJECT_NET_REMOVE_CONTROLLER: u32 = 0x00a5_c480;
/// `NiObject`'s functions: constructor, destructor body, the member copy (`this` = the source,
/// `(destination, process)`), `ProcessClone`, the clone helper (`this` = an object, `(process)`),
/// `RegisterStreamables`, `LoadBinary`, `IsEqual`, `GetViewerStrings`; and the empty function (`RET 4`) that
/// `NiObject::SaveBinary`, `LinkObject` and `PostLinkObject` are.
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
const OBJECT_PROCESS_CLONE: u32 = 0x00a5_d430;
const OBJECT_CLONE_NEW: u32 = 0x00a5_d3f0;
const OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
const OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
const OBJECT_STREAM_HOOK: u32 = 0x00a8_9af0;
/// The cloning process's map lookup (`this` = the map, `(key, &out)`, answers whether there is a
/// clone) and the two `NiPointer` assignments `ProcessClone` runs on an item: `00a5f860` from a pointer
/// (`this` = the slot, `(object)`) and `00c663d0` from a reference (`this` = the slot, `(&source slot)`).
const CLONE_MAP_GET_AT: u32 = 0x00a5_3c30;
const POINTER_ASSIGN: u32 = 0x00a5_f860;
const POINTER_ASSIGN_FROM_REFERENCE: u32 = 0x00c6_63d0;
/// `NiStream` (`this` = the stream): `ResolveLinkID` (reads a link id and answers its object),
/// `GetObjectFromLinkID` (answers the object of the next link id), `ReadLinkID` (reads a link id
/// into the list), `SaveFixedString` (`(&handle)`), `LoadFixedString` (`(&handle)`) and
/// `LoadCStringAsFixedString` (`(&handle)`).
const STREAM_RESOLVE_LINK_ID: u32 = 0x00a6_4620;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
const STREAM_SAVE_FIXED_STRING: u32 = 0x00a6_41f0;
const STREAM_LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
const STREAM_LOAD_C_STRING: u32 = 0x00a6_4ea0;
/// The converter of an old file's top object (`cdecl`, `(stream, index, &object, 0)`).
const STREAM_GET_OBJECT_AT: u32 = 0x00a4_7f40;
/// `NiGetViewerString` (`cdecl`): `(text)`, `_ov8` `(name, int)`, `_ov9` `(name, float)`, `_ov11` `(name,
/// text)`; and `NiTimeController::GetViewerString_ov2` (`cdecl`, `(name, cycle type)`).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_OV8: u32 = 0x00a7_d510;
const GET_VIEWER_STRING_OV9: u32 = 0x00a7_d550;
const GET_VIEWER_STRING_OV11: u32 = 0x00a7_d5e0;
const TIME_CONTROLLER_GET_VIEWER_STRING: u32 = 0x00a6_d190;
/// The viewer-string array's growth (`this` = the array, `(new capacity)`) and `SetAt` (`(index, &string)`).
const VIEWER_STRINGS_GROW: u32 = 0x0096_ad30;
const VIEWER_STRINGS_SET_AT: u32 = 0x0096_ae90;
/// Functions of the unit's neighbours: the owner's `00a2e420` (`this` = the manager, `(time)`);
/// the sequence functions this file does not translate: `00a30540` (`this` = a sequence), `00a30580`
/// (`this` = a sequence, `(partner)`, answers whether the partner chain accepts it), `00a30640` (an
/// `IDTag` reset), `00a30700` (an `InterpArrayItem`'s destructor), `00a30750` (an `IDTag`'s destructor) and
/// `00a316a0` (the constructor of an empty sequence); the animation note listener's `00a41410` (`this` =
/// the listener, `(note)`); `00653290`, a cast (`this` = an object, `(rtti)`, answers the object or 0);
/// the getter `00dbbc10` that answers the item count (`+0x14`) of a palette's map; `NiTSet::Realloc`
/// (`00a311e0`, `this` = the set, `(new size)`) and the set's `AddUnique` (`00e7ec00`, `this` = the set,
/// `(&element)`).
const MANAGER_DEACTIVATE_SOURCES: u32 = 0x00a2_e420;
const SEQUENCE_CLEAR_BLEND: u32 = 0x00a3_0540;
const SEQUENCE_PARTNER_CHAIN_CHECK: u32 = 0x00a3_0580;
const ID_TAG_RESET: u32 = 0x00a3_0640;
const INTERP_ITEM_DESTRUCT: u32 = 0x00a3_0700;
const ID_TAG_DESTRUCT: u32 = 0x00a3_0750;
const SEQUENCE_CONSTRUCT_EMPTY: u32 = 0x00a3_16a0;
const LISTENER_NOTIFY: u32 = 0x00a4_1410;
const OBJECT_CAST: u32 = 0x0065_3290;
const PALETTE_ITEM_COUNT: u32 = 0x00db_bc10;
const SET_REALLOC: u32 = 0x00a3_11e0;
/// Virtual slots. Of every object: `DeleteThis` (4), `GetRTTI` (8), `SaveLinkID` of the `NiStream` (0x30),
/// `CreateClone` (0x48), `RegisterStreamables` (0x54), `IsEqual` (0x5c), `ProcessClone` (0x68); of the
/// sequence: `Deactivate` (0x8c); of a palette: `GetAVObject` (0x8c); of a node: `SetSelectiveUpdateFlags`
/// (0xa0); of a controller: `SetTarget` (0x98), the interpolator count (0xb4), ID (0xb8), index for an ID
/// (0xbc), index for a tag (0xc0), `GetInterpolator` (0xc4), `SetInterpolator` (0xc8), `GetCtlrID` (0xd0),
/// `CreatePoseInterpolator` (0xd4) and `CreateBlendInterpolator` (0xdc); of an interpolator: the check
/// against a palette (0xcc); of a blend interpolator: `AddInterpInfo` (0xdc).
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_GET_RTTI: u32 = 0x08;
const SLOT_SAVE_LINK_ID: u32 = 0x30;
const SLOT_CREATE_CLONE: u32 = 0x48;
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_IS_EQUAL: u32 = 0x5c;
const SLOT_PROCESS_CLONE: u32 = 0x68;
const SLOT_DEACTIVATE: u32 = 0x8c;
const SLOT_GET_AV_OBJECT: u32 = 0x8c;
const SLOT_SET_TARGET: u32 = 0x98;
const SLOT_SELECTIVE_UPDATE_FLAGS: u32 = 0xa0;
const SLOT_GET_INTERPOLATOR_COUNT: u32 = 0xb4;
const SLOT_GET_INTERPOLATOR_ID: u32 = 0xb8;
const SLOT_GET_INTERPOLATOR_INDEX: u32 = 0xbc;
const SLOT_GET_INTERPOLATOR_INDEX_FX: u32 = 0xc0;
const SLOT_GET_INTERPOLATOR: u32 = 0xc4;
const SLOT_SET_INTERPOLATOR: u32 = 0xc8;
const SLOT_INTERPOLATOR_ACCEPTS_PALETTE: u32 = 0xcc;
const SLOT_GET_CTLR_ID: u32 = 0xd0;
const SLOT_CREATE_POSE_INTERPOLATOR: u32 = 0xd4;
const SLOT_CREATE_BLEND_INTERPOLATOR: u32 = 0xdc;
const SLOT_ADD_INTERP_INFO: u32 = 0xdc;
/// Sizes: an `InterpArrayItem` is 0x10 bytes, an `IDTag` 0x14 bytes.
const INTERP_ITEM_SIZE: u32 = 0x10;
const ID_TAG_SIZE: u32 = 0x14;

/// The `float` the exe copies with `FLD`/`FSTP` from the word at `source`: its bits, a signalling NaN quieted.
fn quiet_word(e: &Engine, source: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(source))
}

/// `FLD dword [source]`: the value the x87 stack holds after loading the float at `source`.
fn load_float(e: &Engine, source: u32) -> f64 {
    x87_load32(e.mem.u32(source))
}

/// A float memory operand (`FADD dword [source]` and the like) of the word at `source`.
fn float_operand(e: &Engine, source: u32) -> f64 {
    x87_m32(e.mem.u32(source))
}

/// `InterlockedIncrement` of an object's reference count at `+4`.
fn add_ref(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
}

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at `+4` and, when it
/// reaches zero, the object's `DeleteThis` (virtual slot 4).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// The inline `NiPointer::operator=` for the pointer stored at `slot`: when the new pointer
/// differs the old one (if set) is released, the new one is stored and (if set) counted.
fn assign_pointer(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old != new {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, new);
        if new != 0 {
            add_ref(e, new);
        }
    }
}

/// [`assign_pointer`] for a new pointer the exe reads again from `source` after the release of the
/// old one (an assignment from a reference).
fn assign_pointer_from(e: &mut Engine, slot: u32, source: u32) {
    let old = e.mem.u32(slot);
    if old != e.mem.u32(source) {
        if old != 0 {
            release(e, old);
        }
        let new = e.mem.u32(source);
        e.mem.set_u32(slot, new);
        if new != 0 {
            add_ref(e, new);
        }
    }
}

/// `InterlockedIncrement` of a pooled string's count (`handle - 8`) unless it is the empty string.
fn fixed_string_add_ref(e: &mut Engine, handle: u32) {
    if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_INCREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// `InterlockedDecrement` of a pooled string's count (`handle - 8`) unless it is the empty string.
fn fixed_string_release(e: &mut Engine, handle: u32) {
    if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// The inline `NiFixedString::operator=` for the handle stored at `slot`: when the new handle
/// differs it is counted first, then the old one is released and the new one stored.
fn assign_fixed_string(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old != new {
        fixed_string_add_ref(e, new);
        let old = e.mem.u32(slot);
        fixed_string_release(e, old);
        e.mem.set_u32(slot, new);
    }
}

/// The assignment the exe makes from a freshly made `NiFixedString` temporary whose handle is
/// `handle` (counted by its constructor): the same assignment as [`assign_fixed_string`] (the new handle
/// counted, the old one released) followed by the temporary's release.
fn assign_fixed_string_temporary(e: &mut Engine, slot: u32, handle: u32) {
    assign_fixed_string(e, slot, handle);
    fixed_string_release(e, handle);
}

/// The number of bytes in the NUL-terminated text at `text` (the inline `strlen` loop).
fn string_length(e: &Engine, text: u32) -> u32 {
    let mut length = 0u32;
    while e.mem.u8(text.wrapping_add(length)) != 0 {
        length = length.wrapping_add(1);
    }
    length
}

/// `strcmp(a, b) == 0` as the exe's inlined loop does it: compares the two strings byte by
/// byte, stopping at the first difference or at the common end.
fn same_text(e: &Engine, a: u32, b: u32) -> bool {
    let mut offset = 0u32;
    loop {
        let left = e.mem.u8(a.wrapping_add(offset));
        let right = e.mem.u8(b.wrapping_add(offset));
        if left != right {
            return false;
        }
        if left == 0 {
            return true;
        }
        offset = offset.wrapping_add(1);
    }
}

/// `NiStrnicmp(text, morph tag, its length) == 0`: whether `text` starts with the morph tag.
fn is_morph_text(e: &mut Engine, text: u32) -> bool {
    let tag = e.global::<u32>(MORPH_TAG_POINTER);
    let length = e.global::<u32>(MORPH_TAG_LENGTH);
    e.call(NI_STRNICMP, &args![text, tag, length]).u32() == 0
}

/// The handle of the `"BSTreadTransfController"` fixed string, made on first use (the guard word
/// `011f36c8` bit 0, the handle stored at `011f36c4`, its destructor given to `atexit`).
fn tread_controller_name(e: &mut Engine) -> u32 {
    if e.mem.u8(TREAD_NAME_GUARD) & 1 == 0 {
        let guard = e.mem.u32(TREAD_NAME_GUARD);
        e.mem.set_u32(TREAD_NAME_GUARD, guard | 1);
        let handle = e
            .call(
                FIXED_STRING_FROM_CHARS,
                &args![TEXT_TREAD_TRANSF_CONTROLLER],
            )
            .u32();
        e.mem.set_u32(TREAD_NAME_STATIC, handle);
        e.call(ATEXIT, &args![TREAD_NAME_ATEXIT]);
    }
    e.mem.u32(TREAD_NAME_STATIC)
}

/// The same for the handle of `"Bip01"` (`011f36cc`, guard `011f36d0`).
fn bip01_name(e: &mut Engine) -> u32 {
    if e.mem.u8(BIP01_NAME_GUARD) & 1 == 0 {
        let guard = e.mem.u32(BIP01_NAME_GUARD);
        e.mem.set_u32(BIP01_NAME_GUARD, guard | 1);
        let handle = e.call(FIXED_STRING_FROM_CHARS, &args![TEXT_BIP01]).u32();
        e.mem.set_u32(BIP01_NAME_STATIC, handle);
        e.call(ATEXIT, &args![BIP01_NAME_ATEXIT]);
    }
    e.mem.u32(BIP01_NAME_STATIC)
}

/// The size argument the inline allocation sequences compute for `count` elements of `element_size`
/// bytes plus the count word: all ones when the product overflows 32 bits or adding the 4 bytes of the
/// count word carries (`MUL`, `SETO`/`NEG`/`OR`, `ADD`, `SETC`/`NEG`/`OR`).
fn block_size(count: u32, element_size: u32) -> u32 {
    let product = u64::from(count) * u64::from(element_size);
    let low = product as u32;
    let size = if product >> 32 != 0 { u32::MAX } else { low };
    let (sum, carry) = size.overflowing_add(4);
    if carry {
        u32::MAX
    } else {
        sum
    }
}

/// The inline array maker of this unit: a block from the memory manager of `count` elements of
/// `element_size` bytes behind a count word, the count stored, `_vector_constructor_iterator_` run
/// with `constructor`. Answers the address of the first element, 0 when the allocation answers 0.
fn new_element_array(e: &mut Engine, count: u32, element_size: u32, constructor: u32) -> u32 {
    let size = block_size(count, element_size);
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![block + 4, element_size, count, constructor],
    );
    block + 4
}

/// The inline array maker for arrays of pointers that are cleared instead of constructed: a block
/// from the memory manager of `count` words behind a count word, the count stored and, for a
/// count that is at least 1, the words cleared. Answers the address of the first word, 0 when the
/// allocation answers 0.
fn new_pointer_array(e: &mut Engine, count: u32) -> u32 {
    let size = block_size(count, 4);
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    if count.wrapping_sub(1) as i32 >= 0 {
        for word in 0..count {
            e.mem.set_u32(block + 4 + 4 * word, 0);
        }
    }
    block + 4
}

/// One write through a stream's output `NiBinaryStream`: `function(stream, data, size, &sizes, 1)`
/// with the sizes word set to `size`, and the byte count it answers added to the position.
fn write_stream(e: &mut Engine, output: Ptr<NiBinaryStream>, data: u32, size: u32) {
    e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), size);
        let function = e.get(output, NiBinaryStream::m_pfnWrite);
        let done = e
            .call(function, &args![output, data, size, sizes, 1u32])
            .u32();
        let position = e.get(output, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            output,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
    });
}

/// The read counterpart of [`write_stream`] (`m_pfnRead`).
fn read_stream(e: &mut Engine, input: Ptr<NiBinaryStream>, data: u32, size: u32) {
    e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), size);
        let function = e.get(input, NiBinaryStream::m_pfnRead);
        let done = e
            .call(function, &args![input, data, size, sizes, 1u32])
            .u32();
        let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            input,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
    });
}

/// The stream's input `NiBinaryStream`.
fn input_of(e: &Engine, stream: Ptr<NiStream>) -> Ptr<NiBinaryStream> {
    Ptr::new(e.get(stream, NiStream::m_pkIstr))
}

/// The stream's output `NiBinaryStream`.
fn output_of(e: &Engine, stream: Ptr<NiStream>) -> Ptr<NiBinaryStream> {
    Ptr::new(e.get(stream, NiStream::m_pkOstr))
}

/// Appends `text` to the viewer-string array `strings` (the inline sequence of the
/// `GetViewerStrings` functions): when the 16-bit size (`+0xA`) has reached the capacity (`+8`) the
/// array is grown to the step (`+0xE`) plus the size first, then the string is stored at the old size.
fn append_viewer_string(e: &mut Engine, strings: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(VIEWER_STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(VIEWER_STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00a307c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::GetObjectNameAt` (Xbox PDB): with no `destination` nothing happens. When the
/// interpolator at `index` (`+0x14`, 0x10 bytes each) is null, writes an empty string. Otherwise takes
/// the tag at `index` (`+0x18`, 0x14 bytes each): with a property type (its second word) writes
/// `"<name>\nPROP\n<property type>"` through `NiSprintf` (`00a305b0`, size = the three lengths
/// plus 2), without one copies the object name with `strcpy_s` (`00ec65a6`, size = its length + 1).
pub fn ni_controller_sequence_get_object_name_at(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    index: u32,
    destination: u32,
) {
    if destination == 0 {
        return;
    }
    let interpolators = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
    if e.mem
        .u32(interpolators.wrapping_add(index.wrapping_mul(INTERP_ITEM_SIZE)))
        == 0
    {
        e.mem.set_u8(destination, 0);
        return;
    }
    let tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
    let tag = tags.wrapping_add(index.wrapping_mul(ID_TAG_SIZE));
    let name = e.mem.u32(tag);
    let name_size = string_length(e, name).wrapping_add(1);
    let property_type = e.mem.u32(tag + 4);
    if property_type == 0 {
        e.call(STRING_COPY_SECURE, &args![destination, name_size, name]);
        return;
    }
    let prefix_length = string_length(e, TEXT_PROP);
    let property_length = string_length(e, property_type);
    let size = property_length
        .wrapping_add(prefix_length)
        .wrapping_add(name_size)
        .wrapping_add(1);
    e.call(
        NI_SPRINTF,
        &args![
            destination,
            size,
            TEXT_FORMAT_PROP,
            name,
            TEXT_PROP,
            property_type
        ],
    );
}

// Translated from 00a30880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`): when the state (`+0x44`) is not 0 runs the sequence's
/// virtual slot 0x8c (`Deactivate`) with `(0.0, 0)`; then for each of the `+0xc` interpolator
/// items releases its interpolator controller (second word, cleared) and clears its blend
/// interpolator (third word).
pub fn fn_00a30880(e: &mut Engine, this: Ptr<NiControllerSequence>) {
    if e.get(this, NiControllerSequence::m_eState) != 0 {
        e.vcall(this.addr(), SLOT_DEACTIVATE, &args![0.0f32, 0u32]);
    }
    let mut index = 0u32;
    if e.get(this, NiControllerSequence::m_uiArraySize) != 0 {
        let mut offset = 0u32;
        loop {
            let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
            let slot = items.wrapping_add(offset).wrapping_add(4);
            let controller = e.mem.u32(slot);
            if controller != 0 {
                release(e, controller);
                e.mem.set_u32(slot, 0);
            }
            let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
            e.mem.set_u32(items.wrapping_add(offset).wrapping_add(8), 0);
            index = index.wrapping_add(1);
            offset = offset.wrapping_add(INTERP_ITEM_SIZE);
            if index >= e.get(this, NiControllerSequence::m_uiArraySize) {
                break;
            }
        }
    }
}

// Translated from 00a30900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): for each of the `+0xc` interpolator items with an
/// interpolator (first word) and a blend interpolator (third word), asks the blend interpolator's
/// virtual slot 0xdc (`AddInterpInfo`) with `(interpolator, weight, priority, 1.0)` and stores the
/// answered byte in the item's blend index (`+0xc`). The weight is the sequence's (`+0x1c`, copied
/// through the x87 stack); the priority is the item's byte at `+0xd`, or the sign-extended byte
/// `default_priority` when that is `0xff`.
pub fn fn_00a30900(e: &mut Engine, this: Ptr<NiControllerSequence>, default_priority: u32) {
    let mut index = 0u32;
    if e.get(this, NiControllerSequence::m_uiArraySize) != 0 {
        let mut offset = 0u32;
        loop {
            let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
            let item = items.wrapping_add(offset);
            let interpolator = e.mem.u32(item);
            let blend = e.mem.u32(item + 8);
            if interpolator != 0 && blend != 0 {
                let stored = e.mem.u8(item + 0xd);
                let priority = if stored == 0xff {
                    default_priority as u8 as i8 as i32 as u32
                } else {
                    u32::from(stored)
                };
                let weight = quiet_word(e, this.addr() + 0x1c);
                let index_byte = e
                    .vcall(
                        blend,
                        SLOT_ADD_INTERP_INFO,
                        &args![interpolator, weight, priority, 1.0f32],
                    )
                    .u8();
                e.mem.set_u8(item + 0xc, index_byte);
            }
            index = index.wrapping_add(1);
            offset = offset.wrapping_add(INTERP_ITEM_SIZE);
            if index >= e.get(this, NiControllerSequence::m_uiArraySize) {
                break;
            }
        }
    }
}

// Translated from 00a30970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::ComputeScaledTime` (Xbox PDB): the sequence's scaled time for `time`.
/// The elapsed time is `time` minus the last time (`+0x34`), or `time` itself with the weighted
/// last time (`+0x38`) taken as 0 when the last time is `-FLT_MAX`; the scaled time is the frequency
/// (`+0x28`) times that plus the weighted last time. For the cycle type 0 (`+0x24`, a loop) it is
/// wrapped into the range `[begin, end]` (`+0x2c`, `+0x30`): a zero range gives `begin`; when the range
/// equals the shifted time it gives `end`; otherwise it is the `_CIfmod` remainder of the shifted time
/// by the range plus `begin`, with the range added back when that is below `begin`. For every
/// cycle type the result is then clamped into `[begin, end]`. When `update` is set the weighted last
/// time, the last time and the last scaled time (`+0x38`, `+0x34`, `+0x3c`) are stored. Each store to
/// a float rounds the x87 value to a float; every operation is the x87's (NaNs and ordering as
/// its flag tests).
pub fn ni_controller_sequence_compute_scaled_time(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    time: f32,
    update: bool,
) -> f32 {
    let base = this.addr();
    let negative_max = e.global::<f64>(NEGATIVE_FLT_MAX_DOUBLE);
    let mut weighted = quiet_word(e, base + 0x38);
    let last_time = load_float(e, base + 0x34);
    let time_value = f64::from(x87_quiet(time));
    let elapsed = if last_time == negative_max {
        weighted = 0.0f32.to_bits();
        time_value
    } else {
        x87_sub(time_value, float_operand(e, base + 0x34))
    };
    let elapsed_bits = x87_store32(elapsed);
    let frequency = load_float(e, base + 0x28);
    let scaled = x87_add(x87_mul(frequency, x87_m32(elapsed_bits)), x87_m32(weighted));
    let scaled_bits = x87_store32(scaled);
    let scaled_value = x87_load32(scaled_bits);
    let begin_operand = float_operand(e, base + 0x2c);
    let mut candidate = scaled_bits;
    let mut candidate_value = x87_load32(candidate);
    if e.get(this, NiControllerSequence::m_eCycleType) == 0 {
        let range = x87_sub(load_float(e, base + 0x30), begin_operand);
        let range_bits = x87_store32(range);
        let range_value = x87_load32(range_bits);
        if range_value == 0.0 {
            candidate = x87_store32(load_float(e, base + 0x2c));
            candidate_value = x87_load32(candidate);
        } else {
            let shifted = x87_sub(scaled_value, begin_operand);
            let shifted_bits = x87_store32(shifted);
            let shifted_value = x87_load32(shifted_bits);
            if range_value == shifted_value {
                candidate = x87_store32(load_float(e, base + 0x30));
            } else {
                let remainder = e.call(CI_FMOD, &args![shifted_value, range_value]).f64();
                candidate = x87_store32(remainder);
                let added = x87_add(x87_load32(candidate), begin_operand);
                candidate = x87_store32(added);
            }
            candidate_value = x87_load32(candidate);
            let begin_value = load_float(e, base + 0x2c);
            if begin_value > candidate_value {
                candidate = x87_store32(x87_add(range_value, candidate_value));
                candidate_value = x87_load32(candidate);
            }
        }
    }
    let end_value = load_float(e, base + 0x30);
    let result = if end_value < candidate_value {
        x87_load32(x87_store32(end_value))
    } else {
        let begin_value = load_float(e, base + 0x2c);
        if begin_value > candidate_value {
            x87_load32(x87_store32(begin_value))
        } else {
            candidate_value
        }
    };
    if update {
        e.mem.set_u32(base + 0x38, x87_store32(scaled_value));
        let time_bits = x87_quiet_bits(time.to_bits());
        e.mem.set_u32(base + 0x34, time_bits);
        e.mem.set_u32(base + 0x3c, x87_store32(result));
    }
    f32::from_bits(x87_store32(result))
}

// Translated from 00a30ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether the morph keys of two sequences
/// correspond. False when either has no text keys (`+0x20`). Otherwise walks the other
/// sequence's keys (the count at `+0xc`, an array at `+0x10` of 8-byte entries, the text pointer at
/// +4): for each key whose text starts with the morph tag, this sequence must have a key with the
/// same text (a failed search, or this sequence having no keys, answers false); then the same the
/// other way round; true only when keys of the morph tag were found in the second pass too.
pub fn fn_00a30ab0(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    other: Ptr<NiControllerSequence>,
) -> bool {
    let text_keys = e.get(this, NiControllerSequence::m_spTextKeys).addr();
    if text_keys == 0 {
        return false;
    }
    let other_keys = e.get(other, NiControllerSequence::m_spTextKeys).addr();
    if other_keys == 0 {
        return false;
    }
    let keys = e.mem.u32(text_keys + 0x10);
    let count = e.mem.u32(text_keys + 0xc);
    let other_count = e.mem.u32(other_keys + 0xc);
    let other_array = e.mem.u32(other_keys + 0x10);
    let mut found = false;
    if other_count == 0 {
        return false;
    }
    let mut index = 0u32;
    loop {
        let text = e.mem.u32(
            other_array
                .wrapping_add(4)
                .wrapping_add(index.wrapping_mul(8)),
        );
        if is_morph_text(e, text) {
            found = true;
            if count == 0 {
                return false;
            }
            let mut position = 0u32;
            loop {
                let candidate = e
                    .mem
                    .u32(keys.wrapping_add(4).wrapping_add(position.wrapping_mul(8)));
                if same_text(e, text, candidate) {
                    break;
                }
                position = position.wrapping_add(1);
                if position >= count {
                    return false;
                }
            }
        }
        index = index.wrapping_add(1);
        if index >= other_count {
            break;
        }
    }
    if !found {
        return false;
    }
    found = false;
    if count == 0 {
        return false;
    }
    let mut index = 0u32;
    loop {
        let text = e
            .mem
            .u32(keys.wrapping_add(4).wrapping_add(index.wrapping_mul(8)));
        if is_morph_text(e, text) {
            found = true;
            let mut position = 0u32;
            loop {
                let candidate = e.mem.u32(
                    other_array
                        .wrapping_add(4)
                        .wrapping_add(position.wrapping_mul(8)),
                );
                if same_text(e, text, candidate) {
                    break;
                }
                position = position.wrapping_add(1);
                if position >= other_count {
                    return false;
                }
            }
        }
        index = index.wrapping_add(1);
        if index >= count {
            break;
        }
    }
    found
}

// Translated from 00a30c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): false for a null `other`. When `other` has a
/// partner sequence (`+0x58`) it must not be this sequence and `00a30580` (`this` = this sequence,
/// `(partner)`) must answer true. Then the answer is `00a30ab0` on `other`.
pub fn fn_00a30c80(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    other: Ptr<NiControllerSequence>,
) -> bool {
    if other.is_null() {
        return false;
    }
    let partner = e.get(other, NiControllerSequence::m_pkPartnerSequence);
    if !partner.is_null() {
        if partner.addr() == this.addr() {
            return false;
        }
        if !e
            .call(SEQUENCE_PARTNER_CHAIN_CHECK, &args![this, partner])
            .bool()
        {
            return false;
        }
    }
    fn_00a30ab0(e, this, other)
}

// Translated from 00a30cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; `this` and `other` are `InterpArrayItem`s):
/// whether the two items are equal: their interpolators (first word) are both null or both set and
/// accepted by the first one's `IsEqual` (virtual slot 0x5c), likewise their interpolator
/// controllers (second word), and their priority bytes (`+0xd`) are equal.
pub fn fn_00a30cc0(
    e: &mut Engine,
    this: Ptr<InterpArrayItem>,
    other: Ptr<InterpArrayItem>,
) -> bool {
    let ours = e.get(this, InterpArrayItem::m_spInterpolator).addr();
    let theirs = e.get(other, InterpArrayItem::m_spInterpolator).addr();
    if ours == 0 {
        if theirs != 0 {
            return false;
        }
    } else {
        if theirs == 0 {
            return false;
        }
        if !e.vcall(ours, SLOT_IS_EQUAL, &args![theirs]).bool() {
            return false;
        }
    }
    let ours = e.get(this, InterpArrayItem::m_spInterpCtlr).addr();
    let theirs = e.get(other, InterpArrayItem::m_spInterpCtlr).addr();
    if ours == 0 {
        if theirs != 0 {
            return false;
        }
    } else {
        if theirs == 0 {
            return false;
        }
        if !e.vcall(ours, SLOT_IS_EQUAL, &args![theirs]).bool() {
            return false;
        }
    }
    e.get(this, InterpArrayItem::m_ucPriority) == e.get(other, InterpArrayItem::m_ucPriority)
}

// Translated from 00a30d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`, the second word unused; `this` is an `IDTag`,
/// `palette` an object whose word at `+8` is the base of a text table): the five words of the tag are offsets into the
/// table, `-1` meaning none. Saves them, sets the five handles to the empty string, and for each
/// offset that is not `-1` makes the fixed string of the table text at that offset (`00a5b690`) and
/// stores it (the old handle released unless it is the empty string's). Finally the empty
/// handle the function kept is released (not, as it still is the empty string's).
pub fn fn_00a30d40(e: &mut Engine, this: Ptr<IDTag>, palette: Ptr, _unused_1: u32) {
    let mut offsets = [0u32; 5];
    for (index, offset) in offsets.iter_mut().enumerate() {
        *offset = e.mem.u32(this.addr() + 4 * index as u32);
    }
    let empty = e.global::<u32>(EMPTY_FIXED_STRING);
    for index in 0..5 {
        e.mem.set_u32(this.addr() + 4 * index, empty);
    }
    for (index, offset) in offsets.iter().enumerate() {
        if *offset != 0xffff_ffff {
            let slot = this.addr() + 4 * index as u32;
            let current = e.mem.u32(slot);
            let table = e.mem.u32(palette.addr() + 8);
            let text = table.wrapping_add(*offset);
            if current != text {
                let handle = e.call(FIXED_STRING_FROM_CHARS, &args![text]).u32();
                e.mem.set_u32(slot, handle);
                fixed_string_release(e, current);
            }
        }
    }
    fixed_string_release(e, empty);
}

// Translated from 00a30ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::RegisterStreamables` (Xbox PDB): false when `NiObject::RegisterStreamables`
/// (`00a5d2f0`) answers false. Otherwise for each of the `+0xc` items registers its interpolator and
/// interpolator controller (virtual slot 0x54, when set) and each of the tag's five strings (the
/// stream's virtual slot 0x20, with the address of the handle); then the text keys (`+0x20`, when
/// set), the name (`+8`) and the accum root name (`+0x5c`) strings, and each animation note
/// (`+0x6c`, 16-bit count `+0x70`) with its slot 0x54. Answers true.
pub fn ni_controller_sequence_register_streamables(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    stream: Ptr,
) -> bool {
    if !e
        .call(OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let mut index = 0u32;
    if e.get(this, NiControllerSequence::m_uiArraySize) != 0 {
        let mut interpolator_offset = 0u32;
        let mut tag_offset = 0u32;
        loop {
            let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
            let item = items.wrapping_add(interpolator_offset);
            let interpolator = e.mem.u32(item);
            if interpolator != 0 {
                e.vcall(interpolator, SLOT_REGISTER_STREAMABLES, &args![stream]);
            }
            let controller = e.mem.u32(item + 4);
            if controller != 0 {
                e.vcall(controller, SLOT_REGISTER_STREAMABLES, &args![stream]);
            }
            let tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
            let tag = tags.wrapping_add(tag_offset);
            for word in 0..5u32 {
                e.vcall(stream.addr(), 0x20, &args![tag + 4 * word]);
            }
            interpolator_offset = interpolator_offset.wrapping_add(INTERP_ITEM_SIZE);
            index = index.wrapping_add(1);
            tag_offset = tag_offset.wrapping_add(ID_TAG_SIZE);
            if index >= e.get(this, NiControllerSequence::m_uiArraySize) {
                break;
            }
        }
    }
    let text_keys = e.get(this, NiControllerSequence::m_spTextKeys).addr();
    if text_keys != 0 {
        e.vcall(text_keys, SLOT_REGISTER_STREAMABLES, &args![stream]);
    }
    e.vcall(stream.addr(), 0x20, &args![this.addr() + 8]);
    e.vcall(stream.addr(), 0x20, &args![this.addr() + 0x5c]);
    let mut note = 0u16;
    if 0 < e.get(this, NiControllerSequence::usNumNotes) {
        loop {
            let notes = e.get(this, NiControllerSequence::spAnimNotesA).addr();
            let note_object = e.mem.u32(notes.wrapping_add(u32::from(note) * 4));
            e.vcall(note_object, SLOT_REGISTER_STREAMABLES, &args![stream]);
            note = note.wrapping_add(1);
            if note >= e.get(this, NiControllerSequence::usNumNotes) {
                break;
            }
        }
    }
    true
}

// Translated from 00a30fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::IsEqual` (Xbox PDB): false unless `NiObject::IsEqual` (`00a5d470`) holds, the
/// names (`+8`, equal handles or `NiStricmp` equal), the item count (`+0xc`) and the growth (`+0x10`)
/// are equal, every item equals the other's (`00a30cc0`) with the five words of its tag equal,
/// the weights (`+0x1c`) are equal (a NaN is not), the text keys (`+0x20`) are both null or both set and
/// accepted by the first one's `IsEqual` (virtual slot 0x5c), the cycle types (`+0x24`) are equal and the
/// frequency, begin and end times (`+0x28`, `+0x2c`, `+0x30`) are equal as floats, the owners (`+0x40`)
/// are both null or both set and accepted by the first one's `IsEqual`; the answer is whether the accum
/// root names (`+0x5c`) are the same handle.
pub fn ni_controller_sequence_is_equal(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    other: Ptr<NiControllerSequence>,
) -> bool {
    if !e.call(OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let name = e.get(this, NiControllerSequence::m_kName);
    let other_name = e.get(other, NiControllerSequence::m_kName);
    if name != other_name && e.call(NI_STRICMP, &args![name, other_name]).u32() != 0 {
        return false;
    }
    let count = e.get(this, NiControllerSequence::m_uiArraySize);
    if count != e.get(other, NiControllerSequence::m_uiArraySize) {
        return false;
    }
    if e.get(this, NiControllerSequence::m_uiArrayGrowBy)
        != e.get(other, NiControllerSequence::m_uiArrayGrowBy)
    {
        return false;
    }
    let mut index = 0u32;
    if count != 0 {
        let mut item_offset = 0u32;
        let mut tag_offset = 0u32;
        loop {
            let other_items = e.get(other, NiControllerSequence::m_pkInterpArray).addr();
            let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
            if !fn_00a30cc0(
                e,
                Ptr::new(items.wrapping_add(item_offset)),
                Ptr::new(other_items.wrapping_add(item_offset)),
            ) {
                return false;
            }
            let tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
            let other_tags = e.get(other, NiControllerSequence::m_pkIDTagArray).addr();
            for word in 0..5u32 {
                let ours = e
                    .mem
                    .u32(tags.wrapping_add(tag_offset).wrapping_add(4 * word));
                let theirs = e
                    .mem
                    .u32(other_tags.wrapping_add(tag_offset).wrapping_add(4 * word));
                if ours != theirs {
                    return false;
                }
            }
            index = index.wrapping_add(1);
            item_offset = item_offset.wrapping_add(INTERP_ITEM_SIZE);
            tag_offset = tag_offset.wrapping_add(ID_TAG_SIZE);
            if index >= e.get(this, NiControllerSequence::m_uiArraySize) {
                break;
            }
        }
    }
    if load_float(e, this.addr() + 0x1c) != load_float(e, other.addr() + 0x1c) {
        return false;
    }
    let text_keys = e.get(this, NiControllerSequence::m_spTextKeys).addr();
    let other_text_keys = e.get(other, NiControllerSequence::m_spTextKeys).addr();
    if text_keys == 0 {
        if other_text_keys != 0 {
            return false;
        }
    } else {
        if other_text_keys == 0 {
            return false;
        }
        if !e
            .vcall(text_keys, SLOT_IS_EQUAL, &args![other_text_keys])
            .bool()
        {
            return false;
        }
    }
    if e.get(this, NiControllerSequence::m_eCycleType)
        != e.get(other, NiControllerSequence::m_eCycleType)
    {
        return false;
    }
    for field in [0x28u32, 0x2c, 0x30] {
        if load_float(e, this.addr() + field) != load_float(e, other.addr() + field) {
            return false;
        }
    }
    let owner = e.get(this, NiControllerSequence::m_pkOwner).addr();
    let other_owner = e.get(other, NiControllerSequence::m_pkOwner).addr();
    if owner == 0 {
        if other_owner != 0 {
            return false;
        }
    } else {
        if other_owner == 0 {
            return false;
        }
        if !e.vcall(owner, SLOT_IS_EQUAL, &args![other_owner]).bool() {
            return false;
        }
    }
    e.get(this, NiControllerSequence::m_kAccumRootName)
        == e.get(other, NiControllerSequence::m_kAccumRootName)
}

// Translated from 00a31140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::GetViewerString` (Xbox PDB; `cdecl`, `(name, state)`): allocates (the memory
/// manager's `Allocate`) the length of `name` plus 0x20 bytes, writes `"<name> = <state name>"` into
/// it with `sprintf` (`00ec623a`; the state name is the text for the state 0 to 6, a null pointer for
/// anything above) and answers the buffer.
pub fn ni_controller_sequence_get_viewer_string(e: &mut Engine, name: u32, state: u32) -> u32 {
    let length = string_length(e, name);
    let buffer = e
        .call(
            MEMORY_ALLOCATE,
            &args![MEMORY_MANAGER, length.wrapping_add(0x20)],
        )
        .u32();
    let state_name = STATE_NAMES.get(state as usize).copied().unwrap_or(0);
    e.call(SPRINTF, &args![buffer, TEXT_FORMAT_STATE, name, state_name]);
    buffer
}

// Translated from 00a31260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`; `this` is an `InterpArrayItem`): releases its interpolator
/// and interpolator controller (both cleared), clears the blend interpolator, sets the blend index to
/// the byte at `01096968` and the priority to 0xff.
pub fn fn_00a31260(e: &mut Engine, this: Ptr<InterpArrayItem>) {
    let interpolator = e.get(this, InterpArrayItem::m_spInterpolator).addr();
    if interpolator != 0 {
        release(e, interpolator);
        e.set(this, InterpArrayItem::m_spInterpolator, Ptr::NULL);
    }
    let controller = e.get(this, InterpArrayItem::m_spInterpCtlr).addr();
    if controller != 0 {
        release(e, controller);
        e.set(this, InterpArrayItem::m_spInterpCtlr, Ptr::NULL);
    }
    e.set(this, InterpArrayItem::m_pkBlendInterp, Ptr::NULL);
    let blend_index = e.mem.u8(INVALID_BLEND_INDEX);
    e.set(this, InterpArrayItem::m_ucBlendIdx, blend_index);
    e.set(this, InterpArrayItem::m_ucPriority, 0xff);
}

// Translated from 00a312d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x14`; `this` is an `IDTag`): `IDTag`'s
/// constructor from five fixed-string handles. Starts with the five handles empty (the word at
/// `0109b220` read for each), runs `00a30640` on it, and assigns the five handles that the pointers
/// point to as `NiFixedString::operator=` does (the new handle counted first, the old one
/// released). Answers `this`.
pub fn fn_00a312d0(
    e: &mut Engine,
    this: Ptr<IDTag>,
    first: Ptr,
    second: Ptr,
    third: Ptr,
    fourth: Ptr,
    fifth: Ptr,
) -> Ptr<IDTag> {
    for word in 0..5 {
        let empty = e.global::<u32>(EMPTY_FIXED_STRING);
        e.mem.set_u32(this.addr() + 4 * word, empty);
    }
    e.call(ID_TAG_RESET, &args![this]);
    for (word, source) in [first, second, third, fourth, fifth].iter().enumerate() {
        let handle = e.mem.u32(source.addr());
        assign_fixed_string(e, this.addr() + 4 * word as u32, handle);
    }
    this
}

// Translated from 00a31400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; `this` and `other` are `IDTag`s): the assignment:
/// each of the five handles takes the other's as `NiFixedString::operator=` does. Answers `this`.
pub fn fn_00a31400(e: &mut Engine, this: Ptr<IDTag>, other: Ptr<IDTag>) -> Ptr<IDTag> {
    for word in 0..5u32 {
        let handle = e.mem.u32(other.addr() + 4 * word);
        assign_fixed_string(e, this.addr() + 4 * word, handle);
    }
    this
}

// Translated from 00a31500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the vector deleting destructor of an array of
/// `InterpArrayItem`s (`this` = the first element, the count is the word before it). With bit 1 of
/// `flags` clear it destroys the one item at `this` with `00a30700` and, when bit 0 is set, frees `this`
/// with the memory manager's `Deallocate`; answers `this`. With bit 1 set it releases, from the last item
/// to the first, the interpolator controller (second word) then the interpolator (first word), frees the
/// block from the count word on when bit 0 is set, and answers that address.
pub fn fn_00a31500(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 2 == 0 {
        e.call(INTERP_ITEM_DESTRUCT, &args![this]);
        if flags & 1 != 0 {
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
        }
        return this;
    }
    let block = this.addr().wrapping_sub(4);
    let mut remaining = e.mem.u32(block);
    let mut end = remaining
        .wrapping_mul(INTERP_ITEM_SIZE)
        .wrapping_add(this.addr());
    while (remaining.wrapping_sub(1) as i32) >= 0 {
        remaining = remaining.wrapping_sub(1);
        let controller = e.mem.u32(end.wrapping_sub(0xc));
        end = end.wrapping_sub(INTERP_ITEM_SIZE);
        if controller != 0 {
            release(e, controller);
        }
        let interpolator = e.mem.u32(end);
        if interpolator != 0 {
            release(e, interpolator);
        }
    }
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
    }
    Ptr::new(block)
}

// Translated from 00a315b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the vector deleting destructor of an array of
/// `IDTag`s (`this` = the first element, the count is the word before it). With bit 1 of `flags`
/// clear it destroys the one tag at `this` with `00a30750` and, when bit 0 is set, frees `this`; answers
/// `this`. With bit 1 set it releases the five handles of each tag, from the last tag to the first
/// (the last handle first, an empty-string handle skipped), frees the block from the count word on
/// when bit 0 is set, and answers that address.
pub fn fn_00a315b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 2 == 0 {
        e.call(ID_TAG_DESTRUCT, &args![this]);
        if flags & 1 != 0 {
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
        }
        return this;
    }
    let block = this.addr().wrapping_sub(4);
    let mut remaining = e.mem.u32(block);
    let mut end = remaining
        .wrapping_mul(ID_TAG_SIZE)
        .wrapping_add(this.addr());
    while (remaining.wrapping_sub(1) as i32) >= 0 {
        remaining = remaining.wrapping_sub(1);
        end = end.wrapping_sub(ID_TAG_SIZE);
        for word in (0..5u32).rev() {
            let handle = e.mem.u32(end + 4 * word);
            fixed_string_release(e, handle);
        }
    }
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
    }
    Ptr::new(block)
}

// Translated from 00a31740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the vector deleting destructor of an array of
/// `NiPointer`s (`this` = the first element, the count is the word before it). With bit 1 of `flags`
/// clear it releases the one pointer at `this` and, when bit 0 is set, frees `this`; answers `this`.
/// With bit 1 set it releases every pointer, from the last to the first, frees the block from the
/// count word on when bit 0 is set, and answers that address.
pub fn fn_00a31740(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 2 == 0 {
        let object = e.mem.u32(this.addr());
        if object != 0 {
            release(e, object);
        }
        if flags & 1 != 0 {
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
        }
        return this;
    }
    let block = this.addr().wrapping_sub(4);
    let mut remaining = e.mem.u32(block);
    let mut end = this.addr().wrapping_add(remaining.wrapping_mul(4));
    while (remaining.wrapping_sub(1) as i32) >= 0 {
        remaining = remaining.wrapping_sub(1);
        end = end.wrapping_sub(4);
        let object = e.mem.u32(end);
        if object != 0 {
            release(e, object);
        }
    }
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
    }
    Ptr::new(block)
}

// Translated from 00a317e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): takes the interpolator of item `index` out of the
/// sequence into `destination` (a `NiPointer`). It is counted while the item is cleared with `00a31260`
/// and the tag at `index` is reset with `00a30640`; then `destination` is assigned it as the
/// `NiPointer` assignment does (the old object released when different, the new one counted) and the
/// temporary count is released.
pub fn fn_00a317e0(e: &mut Engine, this: Ptr<NiControllerSequence>, index: u32, destination: Ptr) {
    let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
    let interpolator = e
        .mem
        .u32(index.wrapping_mul(INTERP_ITEM_SIZE).wrapping_add(items));
    if interpolator != 0 {
        add_ref(e, interpolator);
    }
    let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
    fn_00a31260(
        e,
        Ptr::new(items.wrapping_add(index.wrapping_mul(INTERP_ITEM_SIZE))),
    );
    let tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
    e.call(
        ID_TAG_RESET,
        &args![tags.wrapping_add(index.wrapping_mul(ID_TAG_SIZE))],
    );
    assign_pointer(e, destination.addr(), interpolator);
    if interpolator != 0 {
        release(e, interpolator);
    }
}

// Translated from 00a31880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; `this` and `other` are `InterpArrayItem`s): the
/// assignment. The interpolator and interpolator controller (first and second word) are assigned as
/// the `NiPointer` assignment does (the old object released when different, the new one counted),
/// then the blend interpolator, blend index and priority are copied. Answers `this`.
pub fn fn_00a31880(
    e: &mut Engine,
    this: Ptr<InterpArrayItem>,
    other: Ptr<InterpArrayItem>,
) -> Ptr<InterpArrayItem> {
    assign_pointer_from(e, this.addr(), other.addr());
    assign_pointer_from(e, this.addr() + 4, other.addr() + 4);
    let blend = e.mem.u32(other.addr() + 8);
    e.mem.set_u32(this.addr() + 8, blend);
    let index = e.mem.u8(other.addr() + 0xc);
    e.mem.set_u8(this.addr() + 0xc, index);
    let priority = e.mem.u8(other.addr() + 0xd);
    e.mem.set_u8(this.addr() + 0xd, priority);
    this
}

/// Puts `controller` (just made for `target`) into the owner's controller chain, as the exe does
/// when the target is the owner's own target: `target`'s `RemoveController(controller)`, then
/// `controller`'s next controller (`+0x30`) becomes the owner's and the owner's next controller
/// becomes `controller` (both `NiPointer` assignments).
fn insert_controller_after_owner(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    target: u32,
    controller: u32,
) {
    e.call(OBJECT_NET_REMOVE_CONTROLLER, &args![target, controller]);
    let owner = e.get(this, NiControllerSequence::m_pkOwner).addr();
    let owner_next = e.mem.u32(owner + 0x30);
    assign_pointer(e, controller + 0x30, owner_next);
    let owner = e.get(this, NiControllerSequence::m_pkOwner).addr();
    assign_pointer(e, owner + 0x30, controller);
}

/// Whether `palette`'s class chain (its `GetRTTI`, virtual slot 8, and the parent links at `+4`)
/// contains `NiDefaultAVObjectPalette`.
fn is_default_palette(e: &mut Engine, palette: u32) -> bool {
    let mut rtti = e.vcall(palette, SLOT_GET_RTTI, &args![]).u32();
    while rtti != 0 {
        if rtti == NI_DEFAULT_AV_OBJECT_PALETTE_RTTI {
            return true;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    false
}

/// Whether the `NiFixedString` handle `handle` names an `NiTransformController` (its text has a `T` at
/// index 2 and an `f` at index 7: `NiTransf...`); false for a null handle.
fn names_transform_controller(e: &Engine, handle: u32) -> bool {
    handle != 0 && e.mem.u8(handle + 2) == b'T' && e.mem.u8(handle + 7) == b'f'
}

// Translated from 00a31910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (it is mapped as `IDTag::IsTreadController`, which its body is not;
/// `thiscall`, `RET 0xc`, the third word unused): makes the controllers the sequence's items need on
/// `target` and links the items to them, `palette` giving the objects. Answers whether a controller was
/// made. The target's `NiMultiTargetTransformController` and `BSMultiTargetTreadTransfController` are
/// looked up (and counted when there). The tags whose controller type (`+8`) is `BSTreadTransfController`
/// are counted and the 16-bit words at `+0x10` of their interpolators summed. Unless a transform
/// controller exists and either the target has a tread controller or there is no tread tag, the number of
/// transform targets is the palette's item count (`00dbbc10`) less that sum for a
/// `NiDefaultAVObjectPalette` and else the number of tags whose controller type text starts `NiTransf`.
/// With no transform targets and no tread tag nothing is made and the answer is false. A missing transform
/// controller (0x40 bytes, vtable `01096444`, made for that number of targets) and a missing tread
/// controller (0x44 bytes, `00a36070` with the tread count and sum) are made when needed; each gets
/// `SetTarget(target)` (virtual slot 0x98), the flag 0x20 and, when the target is the owner's target, is
/// put into the owner's controller chain. Then for each tag: a transform one whose object from the palette
/// (virtual slot 0x8c; not the accum root of a cumulative owner) is entered in the controller's target
/// table (where it is, or in the first empty slot) with the object's flag 0x80 set, its old transform
/// controller removed unless the object is named `"Bip01"` (the text at `0101e460`), and the item gets the
/// controller and that blend interpolator (flag 1 set); a tread one has each of its interpolator's 0x44-byte
/// records added to the tread controller (`00a35fe0`, or `00a35f70` when `00a35910` finds the object), then
/// the item gets the controller and the tread controller's next interpolator (`00a358a0`, flag 1 set).
pub fn fn_00a31910(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    target: Ptr,
    palette: Ptr,
    _unused_1: u32,
) -> bool {
    let target = target.addr();
    let palette = palette.addr();
    let mut created = false;
    let mut transform_controller = e
        .call(
            OBJECT_NET_GET_CONTROLLER,
            &args![target, NI_MULTI_TARGET_TRANSFORM_CONTROLLER_RTTI],
        )
        .u32();
    if transform_controller != 0 {
        add_ref(e, transform_controller);
    }
    let mut tread_controller = e
        .call(
            OBJECT_NET_GET_CONTROLLER,
            &args![target, BS_MULTI_TARGET_TREAD_TRANSF_CONTROLLER_RTTI],
        )
        .u32();
    if tread_controller != 0 {
        add_ref(e, tread_controller);
    }
    let mut tread_count = 0u32;
    let mut tread_sum = 0u16;
    let mut index = 0u32;
    if e.get(this, NiControllerSequence::m_uiArraySize) != 0 {
        loop {
            let tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
            let tag = tags.wrapping_add(index.wrapping_mul(ID_TAG_SIZE));
            let empty = e.global::<u32>(EMPTY_FIXED_STRING);
            if e.mem.u32(tag + 8) != empty {
                let name = tread_controller_name(e);
                if name == e.mem.u32(tag + 8) {
                    let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
                    let interpolator = e
                        .mem
                        .u32(items.wrapping_add(index.wrapping_mul(INTERP_ITEM_SIZE)));
                    let weight = e.mem.u16(interpolator + 0x10);
                    tread_sum = tread_sum.wrapping_add(weight);
                    tread_count = tread_count.wrapping_add(1);
                }
            }
            index = index.wrapping_add(1);
            if index >= e.get(this, NiControllerSequence::m_uiArraySize) {
                break;
            }
        }
    }
    let tread_sum_word = u32::from(tread_sum);
    let skip_creation =
        transform_controller != 0 && (tread_controller != 0 || tread_count as u16 == 0);
    if !skip_creation {
        let mut transform_targets = 0u32;
        let mut counted = false;
        if palette != 0 && is_default_palette(e, palette) {
            let items = e.call(PALETTE_ITEM_COUNT, &args![palette]).u32();
            transform_targets = u32::from(items.wrapping_sub(tread_sum_word) as u16);
            counted = true;
        }
        if !counted {
            let count = e.get(this, NiControllerSequence::m_uiArraySize);
            let tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
            for position in 0..count {
                let handle = e
                    .mem
                    .u32(tags.wrapping_add(position.wrapping_mul(ID_TAG_SIZE)) + 8);
                if handle != e.global::<u32>(EMPTY_FIXED_STRING)
                    && names_transform_controller(e, handle)
                {
                    transform_targets = transform_targets.wrapping_add(1);
                }
            }
        }
        let mut create_tread = false;
        if transform_targets as u16 == 0 {
            if tread_count as u16 == 0 {
                if tread_controller != 0 {
                    release(e, tread_controller);
                }
                if transform_controller != 0 {
                    release(e, transform_controller);
                }
                return false;
            }
            create_tread = true;
        } else {
            if transform_controller == 0 {
                let block = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
                if block != 0 {
                    e.call(INTERP_CONTROLLER_CONSTRUCT, &args![block]);
                    e.mem.set_u16(block + 0x3c, 0);
                    e.mem
                        .set_u32(block, NI_MULTI_TARGET_TRANSFORM_CONTROLLER_VTABLE);
                    e.mem.set_u32(block + 0x34, 0);
                    e.mem.set_u32(block + 0x38, 0);
                    e.call(MULTI_TARGET_SET_COUNT, &args![block, transform_targets]);
                    add_ref(e, block);
                    transform_controller = block;
                }
                e.vcall(transform_controller, SLOT_SET_TARGET, &args![target]);
                let flags = e.mem.u16(transform_controller + 8);
                e.mem.set_u16(transform_controller + 8, flags | 0x20);
                created = true;
                let owner = e.get(this, NiControllerSequence::m_pkOwner).addr();
                if target == e.mem.u32(owner + 0x2c) {
                    insert_controller_after_owner(e, this, target, transform_controller);
                }
            }
            if tread_count as u16 != 0 {
                create_tread = true;
            }
        }
        if create_tread && tread_controller == 0 {
            let block = e.call(OPERATOR_NEW, &args![0x44u32]).u32();
            if block != 0 {
                let made = e
                    .call(
                        TREAD_CONTROLLER_CONSTRUCT,
                        &args![block, tread_count, tread_sum_word],
                    )
                    .u32();
                if made != 0 {
                    add_ref(e, made);
                    tread_controller = made;
                }
            }
            e.vcall(tread_controller, SLOT_SET_TARGET, &args![target]);
            let flags = e.mem.u16(tread_controller + 8);
            e.mem.set_u16(tread_controller + 8, flags | 0x20);
            created = true;
            let owner = e.get(this, NiControllerSequence::m_pkOwner).addr();
            if target == e.mem.u32(owner + 0x2c) {
                insert_controller_after_owner(e, this, target, tread_controller);
            }
        }
    }
    let mut counter = 0u32;
    bip01_name(e);
    if e.get(this, NiControllerSequence::m_uiArraySize) != 0 {
        let mut position = 0u32;
        let mut item_offset = 0u32;
        let mut tag_offset = 0u32;
        loop {
            let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
            let item = items.wrapping_add(item_offset);
            let tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
            let tag = tags.wrapping_add(tag_offset);
            let empty = e.global::<u32>(EMPTY_FIXED_STRING);
            let handle = e.mem.u32(tag + 8);
            if handle != empty {
                if names_transform_controller(e, handle) {
                    let object = e.vcall(palette, SLOT_GET_AV_OBJECT, &args![tag]).u32();
                    if object != 0 {
                        let accum_root = e.get(this, NiControllerSequence::m_pkAccumRoot).addr();
                        let owner = e.get(this, NiControllerSequence::m_pkOwner).addr();
                        if accum_root != object || e.mem.u8(owner + 0x68) == 0 {
                            link_transform_item(e, transform_controller, object, item);
                        }
                    }
                } else {
                    let name = tread_controller_name(e);
                    if name == e.mem.u32(tag + 8) {
                        link_tread_item(e, tread_controller, palette, item, counter);
                        counter = counter.wrapping_add(1);
                    }
                }
            }
            item_offset = item_offset.wrapping_add(INTERP_ITEM_SIZE);
            tag_offset = tag_offset.wrapping_add(ID_TAG_SIZE);
            position = position.wrapping_add(1);
            if position >= e.get(this, NiControllerSequence::m_uiArraySize) {
                break;
            }
        }
    }
    if tread_controller != 0 {
        release(e, tread_controller);
    }
    if transform_controller != 0 {
        release(e, transform_controller);
    }
    created
}

/// The transform-controller case of [`fn_00a31910`]'s item loop: `object`'s own transform
/// controller (`GetController`, `011f36ec`) is looked up first; then `object` is searched for in the
/// controller's target table (`+0x38`, the 16-bit count at `+0x3c`) and, when it is not
/// there (or is found at the invalid index), the first empty slot is taken. A slot gets `object`, whose
/// flags (`+0x30`) get bit 0x80. With a valid slot the object's transform controller is removed from
/// it unless the object is named `"Bip01"` (the text at `0101e460`), the item's controller (`+4`) is
/// assigned `transform_controller` and its blend interpolator (`+8`) is the slot's 0x30-byte
/// interpolator of the controller's array (`+0x34`), flag 1 of its `+0xc` set.
fn link_transform_item(e: &mut Engine, transform_controller: u32, object: u32, item: u32) {
    let old_controller = e
        .call(
            OBJECT_NET_GET_CONTROLLER,
            &args![object, NI_TRANSFORM_CONTROLLER_RTTI],
        )
        .u32();
    let slot_count = e.mem.u16(transform_controller + 0x3c);
    let invalid = e.mem.u16(INVALID_INDEX);
    let mut slot: Option<u16> = None;
    let mut position = 0u16;
    if slot_count != 0 {
        let table = e.mem.u32(transform_controller + 0x38);
        loop {
            if e.mem.u32(table.wrapping_add(u32::from(position) * 4)) == object {
                if position != invalid {
                    e.mem
                        .set_u32(table.wrapping_add(u32::from(position) * 4), object);
                    let flags = e.mem.u32(object + 0x30);
                    e.mem.set_u32(object + 0x30, flags | 0x80);
                    slot = Some(position);
                }
                break;
            }
            position = position.wrapping_add(1);
            if position >= e.mem.u16(transform_controller + 0x3c) {
                break;
            }
        }
    }
    if slot.is_none() {
        if slot_count == 0 {
            return;
        }
        let table = e.mem.u32(transform_controller + 0x38);
        let mut position = 0u16;
        loop {
            if e.mem.u32(table.wrapping_add(u32::from(position) * 4)) == 0 {
                e.mem
                    .set_u32(table.wrapping_add(u32::from(position) * 4), object);
                let flags = e.mem.u32(object + 0x30);
                e.mem.set_u32(object + 0x30, flags | 0x80);
                if position == invalid {
                    return;
                }
                slot = Some(position);
                break;
            }
            position = position.wrapping_add(1);
            if position >= e.mem.u16(transform_controller + 0x3c) {
                return;
            }
        }
    }
    let slot = u32::from(slot.unwrap_or(0));
    let object_name = e.mem.u32(object + 8);
    if object_name != e.mem.u32(BIP01_NAME_STATIC) {
        e.call(OBJECT_NET_REMOVE_CONTROLLER, &args![object, old_controller]);
    }
    assign_pointer(e, item + 4, transform_controller);
    let interpolators = e.mem.u32(transform_controller + 0x34);
    let blend = slot.wrapping_mul(0x30).wrapping_add(interpolators);
    e.mem.set_u32(item + 8, blend);
    let flags = e.mem.u8(blend + 0xc);
    e.mem.set_u8(blend + 0xc, flags | 1);
}

/// The tread-controller case of [`fn_00a31910`]'s item loop for the interpolator at `item`: each of
/// its records (the count at `+0x10`, 0x44 bytes each from `+0xc`) is looked for in the tread controller
/// by the palette's object for the record's name (virtual slot 0x8c of `palette`, `record + 0x40`):
/// `00a35910` answers the index or the invalid index; the record is added (`00a35fe0`, `(object, counter,
/// record)`) or set (`00a35f70`, `(index, object, counter, record)`). Then the item's controller (`+4`)
/// is assigned the tread controller and its blend interpolator (`+8`) is the controller's interpolator
/// for `counter` (`00a358a0`), flag 1 of its `+0xc` set.
fn link_tread_item(e: &mut Engine, tread_controller: u32, palette: u32, item: u32, counter: u32) {
    let interpolator = e.mem.u32(item);
    let invalid = e.mem.u16(INVALID_INDEX);
    if e.mem.u32(interpolator + 0x10) != 0 {
        let mut record_offset = 0u32;
        let mut record_index = 0u32;
        loop {
            let records = e.mem.u32(interpolator + 0xc);
            let record = records.wrapping_add(record_offset);
            let object = e
                .vcall(palette, SLOT_GET_AV_OBJECT, &args![record + 0x40])
                .u32();
            let index = e
                .call(
                    TREAD_CONTROLLER_FIND_INDEX,
                    &args![tread_controller, object],
                )
                .u16();
            if index == invalid {
                e.call(
                    TREAD_CONTROLLER_ADD,
                    &args![tread_controller, object, counter, record],
                );
            } else {
                e.call(
                    TREAD_CONTROLLER_SET,
                    &args![tread_controller, index, object, counter, record],
                );
            }
            record_index = record_index.wrapping_add(1);
            record_offset = record_offset.wrapping_add(0x44);
            if record_index >= e.mem.u32(interpolator + 0x10) {
                break;
            }
        }
    }
    assign_pointer(e, item + 4, tread_controller);
    let blend = e
        .call(
            TREAD_CONTROLLER_GET_INTERPOLATOR,
            &args![tread_controller, counter],
        )
        .u32();
    e.mem.set_u32(item + 8, blend);
    let flags = e.mem.u8(blend + 0xc);
    e.mem.set_u8(blend + 0xc, flags | 1);
}

// Translated from 00a32010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::FindCorrespondingMorphFrame` (Xbox PDB; `thiscall`, `RET 8`): the time in this
/// sequence that matches `time` of `other`, by their morph text keys (each key an 8-byte entry,
/// the time first, the text at `+4`, in the object at `+0x20`: count `+0xc`, array `+0x10`).
/// `other`'s scaled time (`ComputeScaledTime`, without updating) is placed among its keys whose text starts
/// with the morph tag: `before` is the last such key with a time below it, `after` the first with a time
/// at or after it (a NaN counts as at or after), `first` and `last` the first and the last of them. Without
/// an `after` the time wraps: `after` is `first`, the time of `after` is moved by `other`'s length and the
/// wrap is noted; without a `before` it is `last`, moved back by the length. The ratio from `before` to `after`
/// is stored as a float. Then this sequence's keys are searched (`NiStricmp`) for the keys with the
/// same text as `before` and `after`; the two times are taken (0.0 for one that is not found), moved by this
/// sequence's length in the way the wrap needs (and when both are the same key, when the wrap is noted the second
/// is moved forward, otherwise the first back; for different keys the second is moved forward when it is below the
/// first) and the answer is the first plus the ratio times their difference, stored as a float,
/// then brought into this sequence's `[begin, end]` by adding or subtracting its length. Every
/// operation is the x87's (loads, stores to floats and the flag tests included).
pub fn ni_controller_sequence_find_corresponding_morph_frame(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    other: Ptr<NiControllerSequence>,
    time: f32,
) -> f32 {
    let base = this.addr();
    let other_base = other.addr();
    let scaled_bits = ni_controller_sequence_compute_scaled_time(e, other, time, false).to_bits();
    let scaled = x87_load32(scaled_bits);
    let other_keys = e.get(other, NiControllerSequence::m_spTextKeys).addr();
    let count = e.mem.u32(other_keys + 0xc);
    let array = e.mem.u32(other_keys + 0x10);
    let mut after = 0u32;
    let mut first = 0u32;
    let mut last = 0u32;
    let mut before = 0u32;
    for position in 0..count {
        let key = array.wrapping_add(position.wrapping_mul(8));
        let text = e.mem.u32(key + 4);
        if text != 0 && is_morph_text(e, text) {
            last = key;
            if first == 0 {
                first = key;
            }
            if scaled > x87_load32(e.mem.u32(key)) {
                before = key;
            } else if after == 0 {
                after = key;
            }
        }
    }
    let mut wrapped = false;
    let other_end = load_float(e, other_base + 0x30);
    let other_begin = x87_m32(e.mem.u32(other_base + 0x2c));
    let before_time_bits;
    if after == 0 {
        before_time_bits = quiet_word(e, before);
        wrapped = true;
        after = first;
    } else if before != 0 {
        before_time_bits = quiet_word(e, before);
    } else {
        before = last;
        let shifted = x87_add(
            x87_sub(
                load_float(e, other_base + 0x2c),
                x87_m32(e.mem.u32(other_base + 0x30)),
            ),
            x87_m32(e.mem.u32(last)),
        );
        before_time_bits = x87_store32(shifted);
    }
    let before_time = x87_load32(before_time_bits);
    let numerator = x87_sub(scaled, before_time);
    let denominator = if wrapped {
        let next_bits = x87_store32(x87_add(
            x87_sub(other_end, other_begin),
            x87_m32(e.mem.u32(first)),
        ));
        x87_sub(x87_m32(next_bits), before_time)
    } else {
        x87_sub(x87_m32(e.mem.u32(after)), before_time)
    };
    let ratio_bits = x87_store32(x87_div(numerator, denominator));
    let mut before_done = before == 0;
    let mut after_done = after == 0;
    let keys = e.get(this, NiControllerSequence::m_spTextKeys).addr();
    let count = e.mem.u32(keys + 0xc);
    let array = e.mem.u32(keys + 0x10);
    let mut found_before = 0u32;
    let mut found_after = 0u32;
    for position in 0..count {
        let key = array.wrapping_add(position.wrapping_mul(8));
        let text = e.mem.u32(key + 4);
        let search_after = if before_done {
            if after_done {
                break;
            }
            true
        } else {
            let before_text = e.mem.u32(before + 4);
            if e.call(NI_STRICMP, &args![text, before_text]).u32() == 0 {
                found_before = key;
                before_done = true;
            }
            !after_done
        };
        if search_after {
            let after_text = e.mem.u32(after + 4);
            if e.call(NI_STRICMP, &args![text, after_text]).u32() == 0 {
                found_after = key;
                after_done = true;
            }
        }
    }
    let length = x87_sub(load_float(e, base + 0x30), x87_m32(e.mem.u32(base + 0x2c)));
    let mut from_bits = 0.0f32.to_bits();
    let mut to_bits = 0.0f32.to_bits();
    if found_before == found_after {
        from_bits = quiet_word(e, found_before);
        to_bits = quiet_word(e, found_after);
        if wrapped {
            to_bits = x87_store32(x87_add(length, x87_m32(to_bits)));
        } else {
            from_bits = x87_store32(x87_sub(x87_load32(from_bits), length));
        }
    } else {
        if found_after != 0 {
            to_bits = quiet_word(e, found_after);
        }
        if found_before != 0 {
            from_bits = quiet_word(e, found_before);
        }
        let to_value = x87_load32(to_bits);
        if to_value < x87_load32(from_bits) {
            to_bits = x87_store32(x87_add(length, to_value));
        }
    }
    let from_value = x87_load32(from_bits);
    let difference = x87_sub(x87_load32(to_bits), from_value);
    let value_bits = x87_store32(x87_add(
        x87_mul(difference, x87_load32(ratio_bits)),
        from_value,
    ));
    let value = x87_load32(value_bits);
    let begin = load_float(e, base + 0x2c);
    let end = load_float(e, base + 0x30);
    let result_bits = if begin > value {
        x87_store32(x87_add(
            value,
            x87_sub(end, x87_m32(e.mem.u32(base + 0x2c))),
        ))
    } else if end < value {
        x87_store32(x87_sub(
            value,
            x87_sub(end, x87_m32(e.mem.u32(base + 0x2c))),
        ))
    } else {
        value_bits
    };
    f32::from_bits(result_bits)
}

// Translated from 00a32290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): for each of the `+0xc` items that has a blend
/// interpolator (third word) writes the three floats into it. When the blend interpolator's byte at
/// `+0xe` is 1 and the item's blend index (`+0xc`) is the blend interpolator's byte at `+0xf`, `third` is
/// stored at `+0x20` of the blend interpolator. Otherwise `first`, `second` and `third` go to the
/// words `+4`, `+0x10` and `+0x14` of the blend interpolator's array element (0x18 bytes each, from its
/// pointer at `+0x14`) for that index, bit 4 of its byte at `+0xc` is set and `-FLT_MAX` is stored at
/// `+0x24`, `+0x28` and `+0x2c`. Every float goes through the x87 stack (a signalling NaN is quieted).
pub fn fn_00a32290(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    first: f32,
    second: f32,
    third: f32,
) {
    let first = x87_quiet(first).to_bits();
    let second = x87_quiet(second).to_bits();
    let third = x87_quiet(third).to_bits();
    let mut index = 0u32;
    if e.get(this, NiControllerSequence::m_uiArraySize) == 0 {
        return;
    }
    let mut offset = 0u32;
    loop {
        let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
        let item = items.wrapping_add(offset);
        let blend = e.mem.u32(item + 8);
        if blend != 0 {
            let blend_index = e.mem.u8(item + 0xc);
            if e.mem.u8(blend + 0xe) == 1 && blend_index == e.mem.u8(blend + 0xf) {
                e.mem.set_u32(blend + 0x20, third);
            } else {
                let element = u32::from(blend_index).wrapping_mul(0x18);
                let array = e.mem.u32(blend + 0x14);
                e.mem.set_u32(element.wrapping_add(array) + 0x14, third);
                let array = e.mem.u32(blend + 0x14);
                e.mem.set_u32(element.wrapping_add(array) + 4, first);
                let array = e.mem.u32(blend + 0x14);
                e.mem.set_u32(element.wrapping_add(array) + 0x10, second);
                let flags = e.mem.u8(blend + 0xc);
                e.mem.set_u8(blend + 0xc, flags | 4);
                let negative_max = quiet_word(e, NEGATIVE_FLT_MAX_FLOAT);
                e.mem.set_u32(blend + 0x24, negative_max);
                e.mem.set_u32(blend + 0x28, negative_max);
                e.mem.set_u32(blend + 0x2c, negative_max);
            }
        }
        index = index.wrapping_add(1);
        offset = offset.wrapping_add(INTERP_ITEM_SIZE);
        if index >= e.get(this, NiControllerSequence::m_uiArraySize) {
            break;
        }
    }
}

// Translated from 00a32330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`): grows the item and ID-tag arrays by their growth
/// (`+0x10`). False when the growth is 0. Otherwise makes two arrays of the old size (`+0xc`)
/// plus the growth (`InterpArrayItem`s and `IDTag`s, constructed), assigns the old items and tags into
/// them (`00a31880`, `00a31400`), deletes the old arrays (`00a31500`, `00a315b0` with flags 3, when
/// set), stores the new size and arrays and answers true.
pub fn fn_00a32330(e: &mut Engine, this: Ptr<NiControllerSequence>) -> bool {
    let growth = e.get(this, NiControllerSequence::m_uiArrayGrowBy);
    if growth == 0 {
        return false;
    }
    let size = e
        .get(this, NiControllerSequence::m_uiArraySize)
        .wrapping_add(growth);
    let items = new_element_array(e, size, INTERP_ITEM_SIZE, INTERP_ITEM_CONSTRUCTOR);
    let tags = new_element_array(e, size, ID_TAG_SIZE, ID_TAG_CONSTRUCTOR);
    let mut index = 0u32;
    let mut item_offset = 0u32;
    let mut tag_offset = 0u32;
    while index < size {
        if index < e.get(this, NiControllerSequence::m_uiArraySize) {
            let old_items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
            fn_00a31880(
                e,
                Ptr::new(items.wrapping_add(item_offset)),
                Ptr::new(old_items.wrapping_add(item_offset)),
            );
            let old_tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
            fn_00a31400(
                e,
                Ptr::new(tags.wrapping_add(tag_offset)),
                Ptr::new(old_tags.wrapping_add(tag_offset)),
            );
        }
        index = index.wrapping_add(1);
        item_offset = item_offset.wrapping_add(INTERP_ITEM_SIZE);
        tag_offset = tag_offset.wrapping_add(ID_TAG_SIZE);
    }
    let old_items = e.get(this, NiControllerSequence::m_pkInterpArray);
    if !old_items.is_null() {
        fn_00a31500(e, old_items, 3);
    }
    let old_tags = e.get(this, NiControllerSequence::m_pkIDTagArray);
    if !old_tags.is_null() {
        fn_00a315b0(e, old_tags, 3);
    }
    e.set(this, NiControllerSequence::m_uiArraySize, size);
    e.set(this, NiControllerSequence::m_pkIDTagArray, Ptr::new(tags));
    e.set(this, NiControllerSequence::m_pkInterpArray, Ptr::new(items));
    true
}

// Translated from 00a32470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::ProcessClone` (Xbox PDB): runs `NiObject::ProcessClone` (`00a5d430`), then
/// fixes up the clone (the map of the cloning process, `process`' first word, gives the clone of an
/// object: `00a53c30` with `(object, &clone)`, answering whether there is one; the clone of this sequence
/// is looked up first). For each item with an interpolator: without an owner (`+0x40`) the interpolator's
/// and the interpolator controller's `ProcessClone` (virtual slot 0x68) run; with an owner, in the
/// state 0 (`+0x44`) only the interpolator's does, in another state the clone item's interpolator is set from the
/// clone of the interpolator (`00a5f860`) or, when it has none, from the original item (`00c663d0`);
/// then the clone item's controller is assigned the clone of the original's, or the original, and
/// its blend interpolator is stored as the clone of the original's, or the original. At the end the
/// clone's owner (`+0x40`), partner (`+0x58`) and accum root (`+0x60`) are stored the same way.
pub fn ni_controller_sequence_process_clone(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    process: Ptr,
) {
    e.call(OBJECT_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, local| {
        let map = e.mem.u32(process.addr());
        let lookup = |e: &mut Engine, key: u32| -> Option<u32> {
            e.call(CLONE_MAP_GET_AT, &args![map, key, local])
                .bool()
                .then(|| e.mem.u32(local.addr()))
        };
        lookup(e, this.addr());
        let clone = e.mem.u32(local.addr());
        let mut index = 0u32;
        let mut offset = 0u32;
        if e.get(this, NiControllerSequence::m_uiArraySize) != 0 {
            loop {
                let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
                let item = items.wrapping_add(offset);
                let interpolator = e.mem.u32(item);
                if interpolator != 0 {
                    if e.get(this, NiControllerSequence::m_pkOwner).is_null() {
                        e.vcall(interpolator, SLOT_PROCESS_CLONE, &args![process]);
                        let controller = e.mem.u32(items.wrapping_add(offset) + 4);
                        if controller != 0 {
                            e.vcall(controller, SLOT_PROCESS_CLONE, &args![process]);
                        }
                    } else {
                        let clone_items = e.mem.u32(clone + 0x14);
                        let clone_item = clone_items.wrapping_add(offset);
                        if e.get(this, NiControllerSequence::m_eState) == 0 {
                            e.vcall(interpolator, SLOT_PROCESS_CLONE, &args![process]);
                        } else if let Some(found) = lookup(e, interpolator) {
                            e.call(POINTER_ASSIGN, &args![clone_item, found]);
                        } else {
                            let source = e
                                .get(this, NiControllerSequence::m_pkInterpArray)
                                .addr()
                                .wrapping_add(offset);
                            e.call(POINTER_ASSIGN_FROM_REFERENCE, &args![clone_item, source]);
                        }
                        let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
                        let controller = e.mem.u32(items.wrapping_add(offset) + 4);
                        if let Some(found) = lookup(e, controller) {
                            let clone_items = e.mem.u32(clone + 0x14);
                            assign_pointer(e, clone_items.wrapping_add(offset) + 4, found);
                        } else {
                            let clone_items = e.mem.u32(clone + 0x14);
                            let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
                            assign_pointer_from(
                                e,
                                clone_items.wrapping_add(offset) + 4,
                                items.wrapping_add(offset) + 4,
                            );
                        }
                        let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
                        let blend = e.mem.u32(items.wrapping_add(offset) + 8);
                        let clone_items = e.mem.u32(clone + 0x14);
                        if let Some(found) = lookup(e, blend) {
                            e.mem.set_u32(clone_items.wrapping_add(offset) + 8, found);
                        } else {
                            let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
                            let blend = e.mem.u32(items.wrapping_add(offset) + 8);
                            e.mem.set_u32(clone_items.wrapping_add(offset) + 8, blend);
                        }
                    }
                }
                index = index.wrapping_add(1);
                offset = offset.wrapping_add(INTERP_ITEM_SIZE);
                if index >= e.get(this, NiControllerSequence::m_uiArraySize) {
                    break;
                }
            }
        }
        for field in [0x40u32, 0x58, 0x60] {
            let source = e.mem.u32(this.addr() + field);
            let value = lookup(e, source).unwrap_or(source);
            e.mem.set_u32(clone + field, value);
        }
    });
}

// Translated from 00a326e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::LinkObject` (Xbox PDB): runs the `NiObject` one (`00a89af0`), then takes the
/// next link ids of `stream` (`NiStream::GetObjectFromLinkID`, `00a63430`). For a NIF version below
/// `0x0a010068` the text keys (`+0x20`) and then the interpolator controller (`+4`) of each item
/// are assigned as `NiPointer`s. From that version on the text keys are assigned, the owner
/// (`+0x40`) is stored without counting it, for a BS stream version above 0x15 and below 0x1d one more link
/// is taken and the notes are cleared (`+0x6c` and the count `+0x70`), and for a BS stream version above 0x1c
/// each of the `+0x70` notes (`+0x6c`) is assigned.
pub fn ni_controller_sequence_link_object(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    stream: Ptr<NiStream>,
) {
    e.call(OBJECT_STREAM_HOOK, &args![this, stream]);
    if e.get(stream, NiStream::m_uiNifFileVersion) < 0x0a01_0068 {
        let keys = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        assign_pointer(e, this.addr() + 0x20, keys);
        let mut index = 0u32;
        if e.get(this, NiControllerSequence::m_uiArraySize) != 0 {
            let mut offset = 0u32;
            loop {
                let controller = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
                let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
                assign_pointer(e, items.wrapping_add(offset) + 4, controller);
                offset = offset.wrapping_add(INTERP_ITEM_SIZE);
                index = index.wrapping_add(1);
                if index >= e.get(this, NiControllerSequence::m_uiArraySize) {
                    break;
                }
            }
        }
        return;
    }
    let keys = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.addr() + 0x20, keys);
    let owner = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    e.set(this, NiControllerSequence::m_pkOwner, Ptr::new(owner));
    let version = e.get(stream, NiStream::m_BSStreamHeader);
    if version > 0x15 && version < 0x1d {
        e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]);
        e.set(this, NiControllerSequence::spAnimNotesA, Ptr::NULL);
        e.set(this, NiControllerSequence::usNumNotes, 0);
    }
    if e.get(stream, NiStream::m_BSStreamHeader) > 0x1c
        && e.get(this, NiControllerSequence::usNumNotes) != 0
    {
        let mut note = 0u16;
        loop {
            let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            let notes = e.get(this, NiControllerSequence::spAnimNotesA).addr();
            assign_pointer(e, notes.wrapping_add(u32::from(note) * 4), object);
            note = note.wrapping_add(1);
            if note >= e.get(this, NiControllerSequence::usNumNotes) {
                break;
            }
        }
    }
}

// Translated from 00a328b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::SetPhase` (Xbox PDB; `thiscall`, `RET 8`): the offset (`+0x48`) becomes the begin
/// time (`+0x2c`) plus `phase` (the sum rounded to a float on the x87). When `update_controllers`
/// is not 0 `phase` is also stored (`+0x10`) in the interpolator controller (second word) of each item
/// that has one and whose tag's controller type (`+8` of the tag) is neither the empty string nor
/// an `NiTransf...` type name (the `T` at index 2 and `f` at 7).
pub fn ni_controller_sequence_set_phase(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    phase: f32,
    update_controllers: u8,
) {
    let phase_value = x87_load32(phase.to_bits());
    let sum_bits = x87_store32(x87_add(load_float(e, this.addr() + 0x2c), phase_value));
    if update_controllers != 0 {
        let mut index = 0u32;
        if e.get(this, NiControllerSequence::m_uiArraySize) != 0 {
            let mut item_offset = 0u32;
            let mut tag_offset = 0u32;
            loop {
                let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
                let tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
                let controller = e.mem.u32(items.wrapping_add(item_offset) + 4);
                let tag = tags.wrapping_add(tag_offset);
                if controller != 0 {
                    let handle = e.mem.u32(tag + 8);
                    if handle != e.global::<u32>(EMPTY_FIXED_STRING)
                        && !names_transform_controller(e, handle)
                    {
                        e.mem.set_u32(controller + 0x10, x87_store32(phase_value));
                    }
                }
                index = index.wrapping_add(1);
                tag_offset = tag_offset.wrapping_add(ID_TAG_SIZE);
                item_offset = item_offset.wrapping_add(INTERP_ITEM_SIZE);
                if index >= e.get(this, NiControllerSequence::m_uiArraySize) {
                    break;
                }
            }
        }
    }
    e.mem.set_u32(this.addr() + 0x48, x87_quiet_bits(sum_bits));
}

// Translated from 00a32940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`; `this` is an array of `NiPointer`s: the element
/// pointer at `+4`, the 16-bit size at `+0xa`, the count of set elements at `+0xc`): `SetAt(index,
/// &element)`. Makes the function-local static null pointer (`011f36d4`) on first use. An `index`
/// at or beyond the size sets the size to `index + 1` and counts the element when it is set; below it
/// the count goes up when a set element replaces an unset one and down when an unset one replaces
/// a set one. Then the pointer is assigned as the `NiPointer` assignment does (the old one released
/// when different, the new one counted).
pub fn fn_00a32940(e: &mut Engine, this: Ptr<NiTArray>, index: u32, element: Ptr) {
    if e.mem.u8(NULL_POINTER_GUARD) & 1 == 0 {
        let guard = e.mem.u32(NULL_POINTER_GUARD);
        e.mem.set_u32(NULL_POINTER_GUARD, guard | 1);
        e.mem.set_u32(NULL_POINTER_STATIC, 0);
        e.call(ATEXIT, &args![NULL_POINTER_ATEXIT]);
    }
    let size = u32::from(e.get(this, NiTArray::m_usSize));
    let count = e.get(this, NiTArray::m_usESize);
    if index >= size {
        e.set(this, NiTArray::m_usSize, index.wrapping_add(1) as u16);
        if e.mem.u32(element.addr()) != e.mem.u32(NULL_POINTER_STATIC) {
            e.set(this, NiTArray::m_usESize, count.wrapping_add(1));
        }
    } else {
        let null = e.mem.u32(NULL_POINTER_STATIC);
        let base = e.get(this, NiTArray::m_pBase);
        let current = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        if e.mem.u32(element.addr()) == null {
            if current != null {
                e.set(this, NiTArray::m_usESize, count.wrapping_sub(1));
            }
        } else if current == null {
            e.set(this, NiTArray::m_usESize, count.wrapping_add(1));
        }
    }
    let base = e.get(this, NiTArray::m_pBase);
    assign_pointer_from(e, base.wrapping_add(index.wrapping_mul(4)), element.addr());
}

// Translated from 00a32a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): `NiControllerSequence`'s constructor from a
/// name (`name` points at the `NiFixedString` handle), the number of items and the growth. Runs the
/// `NiObject` constructor (`00a5d3a0`), stores the vtable (`01096584`) and the initial values (weight 1.0,
/// cycle type 0, frequency 1.0, begin `FLT_MAX`, end and every time `-FLT_MAX`, no owner, state 0, no partner
/// or accum root, an empty accum root name, curAnimNIdx 0xffff, no notes), takes the name as the
/// `NiFixedString` assignment does and, for a size above 0, makes the item array and the tag array of
/// that size (0 when an allocation answers 0). Answers `this`.
pub fn fn_00a32a10(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    name: Ptr,
    size: u32,
    grow_by: u32,
) -> Ptr<NiControllerSequence> {
    e.call(OBJECT_CONSTRUCT, &args![this]);
    let one = x87_quiet(1.0f32).to_bits();
    let base = this.addr();
    e.mem.set_u32(base, NI_CONTROLLER_SEQUENCE_VTABLE);
    let empty = e.global::<u32>(EMPTY_FIXED_STRING);
    e.mem.set_u32(base + 8, empty);
    e.mem.set_u32(base + 0x1c, one);
    e.mem.set_u32(base + 0xc, size);
    e.mem.set_u32(base + 0x10, grow_by);
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u32(base + 0x18, 0);
    e.mem.set_u32(base + 0x20, 0);
    e.mem.set_u32(base + 0x28, one);
    let positive_max = quiet_word(e, FLT_MAX_FLOAT);
    e.mem.set_u32(base + 0x24, 0);
    e.mem.set_u32(base + 0x2c, positive_max);
    e.mem.set_u32(base + 0x40, 0);
    let negative_max = quiet_word(e, NEGATIVE_FLT_MAX_FLOAT);
    e.mem.set_u32(base + 0x44, 0);
    for field in [0x30u32, 0x34] {
        e.mem.set_u32(base + field, negative_max);
    }
    e.mem.set_u32(base + 0x58, 0);
    for field in [0x38u32, 0x3c, 0x48, 0x4c, 0x50, 0x54] {
        e.mem.set_u32(base + field, negative_max);
    }
    let empty = e.global::<u32>(EMPTY_FIXED_STRING);
    e.mem.set_u32(base + 0x5c, empty);
    e.mem.set_u32(base + 0x60, 0);
    e.mem.set_u32(base + 0x64, 0);
    e.mem.set_u16(base + 0x68, 0xffff);
    e.mem.set_u32(base + 0x6c, 0);
    e.mem.set_u16(base + 0x70, 0);
    e.mem.set_u8(base + 0x72, 0);
    let handle = e.mem.u32(name.addr());
    assign_fixed_string(e, base + 8, handle);
    let size = e.mem.u32(base + 0xc);
    if size != 0 {
        let items = new_element_array(e, size, INTERP_ITEM_SIZE, INTERP_ITEM_CONSTRUCTOR);
        let size = e.mem.u32(base + 0xc);
        e.mem.set_u32(base + 0x14, items);
        let tags = new_element_array(e, size, ID_TAG_SIZE, ID_TAG_CONSTRUCTOR);
        e.mem.set_u32(base + 0x18, tags);
    }
    this
}

// Translated from 00a32b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): takes the interpolator at `index` out of the
/// sequence (`00a317e0`) into a temporary pointer and releases it, so that the item and tag are
/// cleared.
pub fn fn_00a32b80(e: &mut Engine, this: Ptr<NiControllerSequence>, index: u32) {
    e.with_stack(4, |e, local| {
        fn_00a317e0(e, this, index, local);
        let object = e.mem.u32(local.addr());
        if object != 0 {
            release(e, object);
        }
    });
}

// Translated from 00a32bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): puts `interpolator` into the first item without
/// one and answers its index, or 0xffffffff when every item is used and `00a32330` cannot grow the
/// arrays. The item's interpolator is assigned as the `NiPointer` assignment does, the tag at that index is
/// assigned `tag` (`00a31400`) and the item's priority (`+0xd`) is set to `priority`.
pub fn fn_00a32bc0(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    interpolator: Ptr,
    tag: Ptr<IDTag>,
    priority: u8,
) -> u32 {
    let count = e.get(this, NiControllerSequence::m_uiArraySize);
    let mut index = 0u32;
    if count != 0 {
        let mut item = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
        loop {
            if e.mem.u32(item) == 0 {
                break;
            }
            index = index.wrapping_add(1);
            item = item.wrapping_add(INTERP_ITEM_SIZE);
            if index >= count {
                break;
            }
        }
    }
    if index == count && !fn_00a32330(e, this) {
        return 0xffff_ffff;
    }
    let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
    let offset = index.wrapping_shl(4);
    assign_pointer(e, items.wrapping_add(offset), interpolator.addr());
    let tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
    fn_00a31400(
        e,
        Ptr::new(tags.wrapping_add(index.wrapping_mul(ID_TAG_SIZE))),
        tag,
    );
    let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
    e.mem.set_u8(offset.wrapping_add(items) + 0xd, priority);
    index
}

/// The per-item part of [`fn_00a32c70`] for the item whose interpolator accepted the palette and has no
/// blend interpolator yet, `tag` being its ID tag (the object name `+0`, property type `+4`, controller type
/// `+8`, controller ID `+0xc`, interpolator ID `+0x10`). Answers whether the "controller made" flag
/// got set. The object is the palette's (virtual slot 0x8c with the tag); without one a name that
/// starts with `##` marks the sequence as having removable objects (`+0x72`). With a property type the
/// object's property list (the head at `+0x24`, each node's next pointer first and the property at `+8`) is
/// searched for a property whose class name is it, and the rest uses that property. The controller whose
/// class name is the controller type and whose ID (virtual slot 0xd0) is the tag's (both null counts) is
/// looked for in the object's controller chain (`+0xc`, next at `+0x30`); with none the item's own
/// controller (`+4`) takes the object as its target (virtual slot 0x98) and, when the object is the owner's
/// target, is put at the head of the owner's chain (`RemoveController`, then the next-controller links by
/// `00a67260`). That controller's interpolator index for the interpolator ID (virtual slot 0xbc) is looked up;
/// unless it is the invalid index its interpolator (virtual slot 0xc4) is cast to `NiBlendInterpolator`
/// (`00653290`) or, without one, a blend interpolator is made (virtual slot 0xdc with `(index, 1, whether the
/// object is a cumulative owner's accum root, 0.0, 2)`) and set (virtual slot 0xc8); the controller gets flag
/// 0x20, the item's controller (`+4`, through `0066b0d0`) and blend interpolator (`+8`).
fn attach_item(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    palette: u32,
    item_offset: u32,
    tag: u32,
) -> bool {
    let mut made = false;
    let object = e.vcall(palette, SLOT_GET_AV_OBJECT, &args![tag]).u32();
    if object == 0 {
        let name = e.mem.u32(tag);
        if e.mem.u16(name) == 0x2323 {
            e.mem.set_u8(this.addr() + 0x72, 1);
        }
        return made;
    }
    let mut found = object;
    if e.mem.u32(tag + 4) != 0 {
        let mut node = e.mem.u32(object + 0x24);
        while node != 0 {
            let property = e.mem.u32(node + 8);
            let property_type = e.mem.u32(tag + 4);
            let next = e.mem.u32(node);
            let rtti = e.vcall(property, SLOT_GET_RTTI, &args![]).u32();
            let class_name = e.mem.u32(rtti);
            if same_text(e, class_name, property_type) {
                found = property;
                break;
            }
            node = next;
        }
    }
    let mut controller = e.mem.u32(found + 0xc);
    let mut matched = 0u32;
    while controller != 0 {
        let controller_type = e.mem.u32(tag + 8);
        let rtti = e.vcall(controller, SLOT_GET_RTTI, &args![]).u32();
        let class_name = e.mem.u32(rtti);
        if same_text(e, class_name, controller_type) {
            let id = e.vcall(controller, SLOT_GET_CTLR_ID, &args![]).u32();
            let tag_id = e.mem.u32(tag + 0xc);
            let equal = if id == 0 {
                tag_id == 0
            } else {
                tag_id != 0 && same_text(e, id, tag_id)
            };
            if equal {
                matched = controller;
                break;
            }
        }
        controller = e.mem.u32(controller + 0x30);
    }
    if matched == 0 {
        let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
        let own = e.mem.u32(items.wrapping_add(item_offset) + 4);
        if own == 0 {
            return made;
        }
        e.vcall(own, SLOT_SET_TARGET, &args![found]);
        made = true;
        let owner = e.get(this, NiControllerSequence::m_pkOwner).addr();
        if found == e.mem.u32(owner + 0x2c) {
            e.call(OBJECT_NET_REMOVE_CONTROLLER, &args![found, own]);
            let owner = e.get(this, NiControllerSequence::m_pkOwner).addr();
            let owner_next = e.mem.u32(owner + 0x30);
            e.call(CONTROLLER_SET_NEXT, &args![own, owner_next]);
            let owner = e.get(this, NiControllerSequence::m_pkOwner).addr();
            e.call(CONTROLLER_SET_NEXT, &args![owner, own]);
        }
        matched = own;
    }
    let controller = matched;
    let interpolator_id = e.mem.u32(tag + 0x10);
    let index = e
        .vcall(
            controller,
            SLOT_GET_INTERPOLATOR_INDEX,
            &args![interpolator_id],
        )
        .u16();
    if index == e.mem.u16(INVALID_INDEX) {
        return made;
    }
    let interpolator = e
        .vcall(controller, SLOT_GET_INTERPOLATOR, &args![index])
        .u32();
    let mut blend = if interpolator == 0 {
        0
    } else {
        e.call(
            OBJECT_CAST,
            &args![interpolator, NI_BLEND_INTERPOLATOR_RTTI],
        )
        .u32()
    };
    let owner = e.get(this, NiControllerSequence::m_pkOwner).addr();
    let accum_root = e.get(this, NiControllerSequence::m_pkAccumRoot).addr() == found
        && e.mem.u8(owner + 0x68) != 0;
    if blend == 0 {
        blend = e
            .vcall(
                controller,
                SLOT_CREATE_BLEND_INTERPOLATOR,
                &args![index, 1u32, u32::from(accum_root), 0u32, 2u32],
            )
            .u32();
        e.vcall(controller, SLOT_SET_INTERPOLATOR, &args![blend, index]);
        if blend == 0 {
            return made;
        }
    }
    let flags = e.mem.u16(controller + 8);
    e.mem.set_u16(controller + 8, flags | 0x20);
    let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
    e.call(
        POINTER_SLOT_ASSIGN,
        &args![items.wrapping_add(item_offset) + 4, controller],
    );
    let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
    e.mem.set_u32(items.wrapping_add(item_offset) + 8, blend);
    made
}

// Translated from 00a32c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): attaches the sequence to `target`, through the owner's
/// palette (`+0x78` of the owner). When the accum root name (`+0x5c`) is not empty the accum root (`+0x60`)
/// is the palette's object for it (virtual slot 0x8c). Runs `00a31910` for `target` and the palette;
/// then for each item whose interpolator accepts the palette (virtual slot 0xcc) and that has no blend
/// interpolator, `attach_item`. When a controller was made (by `00a31910` or by an item) the
/// owner's target gets virtual slot 0xa0 (`SetSelectiveUpdateFlags`-shaped: a byte out parameter, 1 and a
/// word whose low byte is the made flag and whose other bytes are `target`'s). Answers true.
pub fn fn_00a32c70(e: &mut Engine, this: Ptr<NiControllerSequence>, target: Ptr) -> bool {
    let owner = e.get(this, NiControllerSequence::m_pkOwner).addr();
    let palette = e.mem.u32(owner + 0x78);
    if e.get(this, NiControllerSequence::m_kAccumRootName) != e.global::<u32>(EMPTY_FIXED_STRING) {
        let object = e
            .vcall(palette, SLOT_GET_AV_OBJECT, &args![this.addr() + 0x5c])
            .u32();
        e.set(this, NiControllerSequence::m_pkAccumRoot, Ptr::new(object));
    }
    let mut created = fn_00a31910(e, this, target, Ptr::new(palette), 0);
    let mut index = 0u32;
    if e.get(this, NiControllerSequence::m_uiArraySize) != 0 {
        let mut item_offset = 0u32;
        let mut tag_offset = 0u32;
        loop {
            let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
            let interpolator = e.mem.u32(items.wrapping_add(item_offset));
            if interpolator != 0 {
                let tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
                let tag = tags.wrapping_add(tag_offset);
                if e.vcall(
                    interpolator,
                    SLOT_INTERPOLATOR_ACCEPTS_PALETTE,
                    &args![palette],
                )
                .bool()
                {
                    let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
                    if e.mem.u32(items.wrapping_add(item_offset) + 8) == 0
                        && attach_item(e, this, palette, item_offset, tag)
                    {
                        created = true;
                    }
                }
            }
            tag_offset = tag_offset.wrapping_add(ID_TAG_SIZE);
            index = index.wrapping_add(1);
            item_offset = item_offset.wrapping_add(INTERP_ITEM_SIZE);
            if index >= e.get(this, NiControllerSequence::m_uiArraySize) {
                break;
            }
        }
    }
    if created {
        let owner = e.get(this, NiControllerSequence::m_pkOwner).addr();
        let owner_target = e.mem.u32(owner + 0x2c);
        e.with_stack(8, |e, scratch| {
            let flag_slot = scratch.addr() + 4;
            e.mem.set_u32(flag_slot, (target.addr() & 0xffff_ff00) | 1);
            e.vcall(
                owner_target,
                SLOT_SELECTIVE_UPDATE_FLAGS,
                &args![scratch, 1u32, flag_slot],
            );
        });
    }
    true
}

// Translated from 00a32fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`; the second and third words are unused): copies
/// this sequence's interpolators into `destination` as pose interpolators. When `clear_destination`
/// is not 0 every item of `destination` is first taken out (`00a317e0` into a temporary pointer that is
/// released). The destination's cycle type (`+0x24`) is copied when this one's is 0 or 2, and its
/// frequency, begin and end times (`+0x28`, `+0x2c`, `+0x30`) are copied through the x87 stack. For each item
/// of this sequence with an interpolator and an interpolator controller, the controller's virtual slot 0xc0 is asked
/// for the index of this sequence's tag; unless it is the invalid index, the controller's virtual slot 0xd4 makes a
/// pose interpolator for it, which is added to `destination` (`00a32bc0`, with an `IDTag` built by
/// `00a312d0` from the static at `011f49e0` five times, and priority 0xff; the tag is then destroyed with
/// `00a30750`); the new item takes this item's blend interpolator (`+8`), its controller (`+4`,
/// assigned as `NiPointer`) and its priority (`+0xd`).
pub fn fn_00a32fa0(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    destination: Ptr<NiControllerSequence>,
    _unused_1: u32,
    _unused_2: u32,
    clear_destination: u8,
) {
    if clear_destination != 0 {
        let mut index = 0u32;
        if e.get(destination, NiControllerSequence::m_uiArraySize) != 0 {
            loop {
                e.with_stack(4, |e, local| {
                    fn_00a317e0(e, destination, index, local);
                    let object = e.mem.u32(local.addr());
                    if object != 0 {
                        release(e, object);
                    }
                });
                index = index.wrapping_add(1);
                if index >= e.get(destination, NiControllerSequence::m_uiArraySize) {
                    break;
                }
            }
        }
    }
    let cycle_type = e.get(this, NiControllerSequence::m_eCycleType);
    if cycle_type == 2 || cycle_type == 0 {
        e.set(destination, NiControllerSequence::m_eCycleType, cycle_type);
    }
    for field in [0x28u32, 0x2c, 0x30] {
        let value = quiet_word(e, this.addr() + field);
        e.mem.set_u32(destination.addr() + field, value);
    }
    let mut index = 0u32;
    if e.get(this, NiControllerSequence::m_uiArraySize) != 0 {
        let mut item_offset = 0u32;
        let mut tag_offset = 0u32;
        loop {
            let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
            let item = items.wrapping_add(item_offset);
            let controller = e.mem.u32(item + 4);
            if e.mem.u32(item) != 0 && controller != 0 {
                let tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
                let source_index = e
                    .vcall(
                        controller,
                        SLOT_GET_INTERPOLATOR_INDEX_FX,
                        &args![tags.wrapping_add(tag_offset)],
                    )
                    .u16();
                if source_index != e.mem.u16(INVALID_INDEX) {
                    let pose = e
                        .vcall(
                            controller,
                            SLOT_CREATE_POSE_INTERPOLATOR,
                            &args![source_index],
                        )
                        .u32();
                    let new_index = e.with_stack(ID_TAG_SIZE, |e, tag| {
                        fn_00a312d0(
                            e,
                            Ptr::new(tag.addr()),
                            Ptr::new(FIXED_STRING_011F49E0),
                            Ptr::new(FIXED_STRING_011F49E0),
                            Ptr::new(FIXED_STRING_011F49E0),
                            Ptr::new(FIXED_STRING_011F49E0),
                            Ptr::new(FIXED_STRING_011F49E0),
                        );
                        let new_index =
                            fn_00a32bc0(e, destination, Ptr::new(pose), Ptr::new(tag.addr()), 0xff);
                        e.call(ID_TAG_DESTRUCT, &args![tag]);
                        new_index
                    });
                    let new_items = e
                        .get(destination, NiControllerSequence::m_pkInterpArray)
                        .addr();
                    let new_item = new_index.wrapping_shl(4).wrapping_add(new_items);
                    let blend = e.mem.u32(item + 8);
                    e.mem.set_u32(new_item + 8, blend);
                    assign_pointer_from(e, new_item + 4, item + 4);
                    let priority = e.mem.u8(item + 0xd);
                    e.mem.set_u8(new_item + 0xd, priority);
                }
            }
            item_offset = item_offset.wrapping_add(INTERP_ITEM_SIZE);
            tag_offset = tag_offset.wrapping_add(ID_TAG_SIZE);
            index = index.wrapping_add(1);
            if index >= e.get(this, NiControllerSequence::m_uiArraySize) {
                break;
            }
        }
    }
}

/// The five handles of the temporary `IDTag` at `tag` released by the inline destructor of the
/// functions that build one on the stack (the last handle first, an empty-string handle skipped).
fn release_temporary_tag(e: &mut Engine, tag: u32) {
    for word in (0..5u32).rev() {
        let handle = e.mem.u32(tag + 4 * word);
        fixed_string_release(e, handle);
    }
}

/// The `NiFixedString` for the text at `text` (`00a5b690`), or the empty string's handle when it is null,
/// as the inline code of the functions that build a temporary tag does it.
fn fixed_string_or_empty(e: &mut Engine, text: u32) -> u32 {
    if text == 0 {
        e.global::<u32>(EMPTY_FIXED_STRING)
    } else {
        e.call(FIXED_STRING_FROM_CHARS, &args![text]).u32()
    }
}

// Translated from 00a33130 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a33130 - the /GS cookie check (00ec408c) is not modeled
/// Unnamed in the engine map (`thiscall`, `RET 8`): adds the interpolators of `controller` to this sequence
/// for the object `text` names: `"name"`, or `"name\nPROP\nproperty type"` as `GetObjectNameAt` makes
/// it (`_strcpy_s` into three 0x104-byte buffers and `_strchr` for the line ends). For each of the controller's
/// interpolators (the count by its virtual slot 0xb4, the loop as 16-bit numbers) an `IDTag` is built on
/// the stack (`00a312d0` from the static at `011f49e0`) with the name, the property type when the second line
/// is `PROP` (`01096614`), the controller's class name (`GetRTTI`, virtual slot 8, the name word of the `NiRTTI`),
/// its ID (virtual slot 0xd0) and the interpolator's ID (virtual slot 0xb8), each made a `NiFixedString`
/// (`00a5b690`, the empty string for a null text) and taken as `NiFixedString`'s assignment does; the
/// interpolator (virtual slot 0xc4) is added with priority 0xff (`00a32bc0`) and the new item's controller
/// (`+4`) is assigned `controller`. A full array that cannot grow ends it with false (after releasing the
/// tag's handles); true otherwise. Both ways release the temporary tag's five handles.
pub fn fn_00a33130(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    text: Ptr,
    controller: Ptr,
) -> bool {
    let controller = controller.addr();
    e.with_stack(0x104 * 3 + ID_TAG_SIZE, |e, block| {
        let first = block.addr();
        let second = first + 0x104;
        let third = second + 0x104;
        let tag = third + 0x104;
        e.call(STRING_COPY_SECURE, &args![first, 0x104u32, text]);
        e.mem.set_u8(second, 0);
        e.mem.set_u8(third, 0);
        let line_end = e.call(STRCHR, &args![first, 0x0au32]).u32();
        if line_end != 0 {
            e.call(STRING_COPY_SECURE, &args![second, 0x104u32, line_end + 1]);
            e.mem.set_u8(line_end, 0);
            let line_end = e.call(STRCHR, &args![second, 0x0au32]).u32();
            e.call(
                STRING_COPY_SECURE,
                &args![third, 0x104u32, line_end.wrapping_add(1)],
            );
            e.mem.set_u8(line_end, 0);
        }
        let mut index = 0u32;
        let count = e
            .vcall(controller, SLOT_GET_INTERPOLATOR_COUNT, &args![])
            .u16();
        if count == 0 {
            return true;
        }
        loop {
            fn_00a312d0(
                e,
                Ptr::new(tag),
                Ptr::new(FIXED_STRING_011F49E0),
                Ptr::new(FIXED_STRING_011F49E0),
                Ptr::new(FIXED_STRING_011F49E0),
                Ptr::new(FIXED_STRING_011F49E0),
                Ptr::new(FIXED_STRING_011F49E0),
            );
            let handle = e.call(FIXED_STRING_FROM_CHARS, &args![first]).u32();
            assign_fixed_string_temporary(e, tag, handle);
            if same_text(e, second, TEXT_PROP_MARKER) {
                let handle = e.call(FIXED_STRING_FROM_CHARS, &args![third]).u32();
                assign_fixed_string_temporary(e, tag + 4, handle);
            }
            let rtti = e.vcall(controller, SLOT_GET_RTTI, &args![]).u32();
            let class_name = e.mem.u32(rtti);
            let handle = fixed_string_or_empty(e, class_name);
            assign_fixed_string_temporary(e, tag + 8, handle);
            let id = e.vcall(controller, SLOT_GET_CTLR_ID, &args![]).u32();
            let handle = fixed_string_or_empty(e, id);
            assign_fixed_string_temporary(e, tag + 0xc, handle);
            let interpolator_id = e
                .vcall(controller, SLOT_GET_INTERPOLATOR_ID, &args![index])
                .u32();
            let handle = fixed_string_or_empty(e, interpolator_id);
            assign_fixed_string_temporary(e, tag + 0x10, handle);
            let interpolator = e
                .vcall(controller, SLOT_GET_INTERPOLATOR, &args![index])
                .u32();
            let added = fn_00a32bc0(e, this, Ptr::new(interpolator), Ptr::new(tag), 0xff);
            if added == 0xffff_ffff {
                release_temporary_tag(e, tag);
                return false;
            }
            let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
            assign_pointer(e, items.wrapping_add(added.wrapping_shl(4)) + 4, controller);
            release_temporary_tag(e, tag);
            index = index.wrapping_add(1);
            let count = e
                .vcall(controller, SLOT_GET_INTERPOLATOR_COUNT, &args![])
                .u16();
            if (index as u16) >= count {
                break;
            }
        }
        true
    })
}

// Translated from 00a33570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`; `this` is the source): copies this sequence into
/// `destination` for a clone made by `process`. Runs the `NiObject` member copy (`00a5d450`, `this` = the
/// source, `(destination, process)`), assigns the name (`NiFixedString` assignment), copies the size and
/// growth (`+0xc`, `+0x10`) and makes the destination's item and tag arrays of that size. For each item with an
/// interpolator: without an owner (`+0x40`) the interpolator (and, when it has one, the interpolator
/// controller) is cloned with virtual slot 0x48 and the clone assigned into the destination's item; with an
/// owner the interpolator is cloned by `00a5d3f0` and the item's blend index (`+0xc`) is copied. The priority
/// (`+0xd`) and the tag (`00a31400`) are copied for every item. Then the weight (`+0x1c`) is copied through
/// the x87 stack, the text keys (`+0x20`) assigned as a `NiPointer`, the cycle type (`+0x24`) copied and the
/// frequency, begin and end times (`+0x28`, `+0x2c`, `+0x30`) through the x87 stack, the accum root name
/// (`+0x5c`) assigned as a `NiFixedString`, curAnimNIdx (`+0x68`) and the note count (`+0x70`) copied; with
/// notes the destination gets an array of that many (cleared) pointers (`+0x6c`) to which each note is
/// assigned.
pub fn fn_00a33570(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    destination: Ptr<NiControllerSequence>,
    process: Ptr,
) {
    let base = this.addr();
    let dest = destination.addr();
    e.call(OBJECT_COPY_MEMBERS, &args![this, destination, process]);
    let name = e.mem.u32(base + 8);
    assign_fixed_string(e, dest + 8, name);
    let size = e.mem.u32(base + 0xc);
    e.mem.set_u32(dest + 0xc, size);
    let grow_by = e.mem.u32(base + 0x10);
    e.mem.set_u32(dest + 0x10, grow_by);
    let size = e.mem.u32(base + 0xc);
    let items = new_element_array(e, size, INTERP_ITEM_SIZE, INTERP_ITEM_CONSTRUCTOR);
    e.mem.set_u32(dest + 0x14, items);
    let size = e.mem.u32(base + 0xc);
    let tags = new_element_array(e, size, ID_TAG_SIZE, ID_TAG_CONSTRUCTOR);
    e.mem.set_u32(dest + 0x18, tags);
    let mut index = 0u32;
    if e.mem.u32(base + 0xc) != 0 {
        let mut item_offset = 0u32;
        let mut tag_offset = 0u32;
        loop {
            let items = e.mem.u32(base + 0x14);
            let item = items.wrapping_add(item_offset);
            let interpolator = e.mem.u32(item);
            if interpolator != 0 {
                if e.mem.u32(base + 0x40) == 0 {
                    let clone = e
                        .vcall(interpolator, SLOT_CREATE_CLONE, &args![process])
                        .u32();
                    let dest_items = e.mem.u32(dest + 0x14);
                    assign_pointer(e, dest_items.wrapping_add(item_offset), clone);
                    let controller = e
                        .mem
                        .u32(item_offset.wrapping_add(e.mem.u32(base + 0x14)) + 4);
                    if controller != 0 {
                        let clone = e
                            .vcall(controller, SLOT_CREATE_CLONE, &args![process])
                            .u32();
                        let dest_items = e.mem.u32(dest + 0x14);
                        assign_pointer(e, item_offset.wrapping_add(dest_items) + 4, clone);
                    }
                } else {
                    let clone = e
                        .call(OBJECT_CLONE_NEW, &args![interpolator, process])
                        .u32();
                    let dest_items = e.mem.u32(dest + 0x14);
                    assign_pointer(e, dest_items.wrapping_add(item_offset), clone);
                    let items = e.mem.u32(base + 0x14);
                    let blend_index = e.mem.u8(item_offset.wrapping_add(items) + 0xc);
                    let dest_items = e.mem.u32(dest + 0x14);
                    e.mem
                        .set_u8(item_offset.wrapping_add(dest_items) + 0xc, blend_index);
                }
            }
            let items = e.mem.u32(base + 0x14);
            let priority = e.mem.u8(item_offset.wrapping_add(items) + 0xd);
            let dest_items = e.mem.u32(dest + 0x14);
            e.mem
                .set_u8(item_offset.wrapping_add(dest_items) + 0xd, priority);
            let source_tags = e.mem.u32(base + 0x18);
            let dest_tags = e.mem.u32(dest + 0x18);
            fn_00a31400(
                e,
                Ptr::new(dest_tags.wrapping_add(tag_offset)),
                Ptr::new(source_tags.wrapping_add(tag_offset)),
            );
            index = index.wrapping_add(1);
            tag_offset = tag_offset.wrapping_add(ID_TAG_SIZE);
            item_offset = item_offset.wrapping_add(INTERP_ITEM_SIZE);
            if index >= e.mem.u32(base + 0xc) {
                break;
            }
        }
    }
    let weight = quiet_word(e, base + 0x1c);
    e.mem.set_u32(dest + 0x1c, weight);
    let text_keys = e.mem.u32(base + 0x20);
    assign_pointer(e, dest + 0x20, text_keys);
    let cycle_type = e.mem.u32(base + 0x24);
    e.mem.set_u32(dest + 0x24, cycle_type);
    for field in [0x28u32, 0x2c, 0x30] {
        let value = quiet_word(e, base + field);
        e.mem.set_u32(dest + field, value);
    }
    let root_name = e.mem.u32(base + 0x5c);
    assign_fixed_string(e, dest + 0x5c, root_name);
    let cur_index = e.mem.u16(base + 0x68);
    e.mem.set_u16(dest + 0x68, cur_index);
    let notes = e.mem.u16(base + 0x70);
    e.mem.set_u16(dest + 0x70, notes);
    if e.mem.u16(base + 0x70) != 0 {
        let count = u32::from(e.mem.u16(base + 0x70));
        let array = new_pointer_array(e, count);
        e.mem.set_u32(dest + 0x6c, array);
    }
    let mut note = 0u16;
    if e.mem.u16(base + 0x70) != 0 {
        loop {
            let dest_notes = e.mem.u32(dest + 0x6c);
            let source_notes = e.mem.u32(base + 0x6c);
            assign_pointer_from(
                e,
                dest_notes.wrapping_add(u32::from(note) * 4),
                source_notes.wrapping_add(u32::from(note) * 4),
            );
            note = note.wrapping_add(1);
            if note >= e.mem.u16(base + 0x70) {
                break;
            }
        }
    }
}

// Translated from 00a33950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; `this` is an `InterpArrayItem`): loads the item from
/// `stream`. The interpolator and the interpolator controller (first and second word) are
/// assigned the objects of the next two link ids (`NiStream::ResolveLinkID`, `00a64620`) as `NiPointer`s. For
/// a NIF version below `0x0a01006f` one more link id is read and then 2 bytes are read and dropped. For a
/// nonzero BS stream version the priority (`+0xd`) is read as one byte.
pub fn fn_00a33950(e: &mut Engine, this: Ptr<InterpArrayItem>, stream: Ptr<NiStream>) {
    let interpolator = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.addr(), interpolator);
    let controller = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.addr() + 4, controller);
    if e.get(stream, NiStream::m_uiNifFileVersion) < 0x0a01_006f {
        e.call(STREAM_RESOLVE_LINK_ID, &args![stream]);
        let input = input_of(e, stream);
        e.with_stack(4, |e, scratch| read_stream(e, input, scratch.addr(), 2));
    }
    if e.get(stream, NiStream::m_BSStreamHeader) != 0 {
        let input = input_of(e, stream);
        read_stream(e, input, this.addr() + 0xd, 1);
    }
}

// Translated from 00a33a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::IDTag::LoadBinary` (Xbox PDB; `thiscall`, `RET 8`): loads the five strings of the tag
/// from `stream`; `interpolator` is where the link id's object goes for the middle format. Below NIF version
/// `0x0a010071` each is `NiStream::LoadCStringAsFixedString` (`00a64ea0`, `(&handle)`). From
/// `0x14010001` on each is `NiStream::LoadFixedString` (`00a64df0`, `(&handle)`). In between the
/// next link id's object is stored through `interpolator` (`00a64620`), then five 4-byte values
/// are read, the first to the last, and stored as the five words of the tag.
pub fn ni_controller_sequence_id_tag_load_binary(
    e: &mut Engine,
    this: Ptr<IDTag>,
    stream: Ptr<NiStream>,
    interpolator: Ptr,
) {
    let version = e.get(stream, NiStream::m_uiNifFileVersion);
    if version < 0x0a01_0071 {
        for word in 0..5u32 {
            e.call(STREAM_LOAD_C_STRING, &args![stream, this.addr() + 4 * word]);
        }
        return;
    }
    if version < 0x1401_0001 {
        let object = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
        e.mem.set_u32(interpolator.addr(), object);
        e.with_stack(0x14, |e, values| {
            for word in 0..5u32 {
                let input = input_of(e, stream);
                read_stream(e, input, values.addr() + 4 * word, 4);
            }
            for word in 0..5u32 {
                let value = e.mem.u32(values.addr() + 4 * word);
                e.mem.set_u32(this.addr() + 4 * word, value);
            }
        });
        return;
    }
    for word in 0..5u32 {
        e.call(
            STREAM_LOAD_FIXED_STRING,
            &args![stream, this.addr() + 4 * word],
        );
    }
}

// Translated from 00a33bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::SaveBinary` (Xbox PDB): runs the `NiObject` one (`00a89af0`), then writes to
/// `stream`'s output (`+0x250`; each write through its write function at `+0xc`, which adds the byte count it
/// answers to the position): the name (`NiStream::SaveFixedString`, `00a641f0`), the size and the growth (4
/// bytes each) and, for each item, the link ids of its interpolator and interpolator controller (the stream's
/// virtual slot 0x30), the priority (1 byte) and the five strings of its tag; then the weight (4 bytes), the
/// text keys' link id, the cycle type, frequency, begin and end (4 bytes each), the owner's link id, the accum
/// root name, the note count (2 bytes) and each note's link id.
pub fn ni_controller_sequence_save_binary(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    stream: Ptr<NiStream>,
) {
    let base = this.addr();
    e.call(OBJECT_STREAM_HOOK, &args![this, stream]);
    e.call(STREAM_SAVE_FIXED_STRING, &args![stream, base + 8]);
    let output = output_of(e, stream);
    write_stream(e, output, base + 0xc, 4);
    let output = output_of(e, stream);
    write_stream(e, output, base + 0x10, 4);
    let mut index = 0u32;
    if e.mem.u32(base + 0xc) != 0 {
        let mut item_offset = 0u32;
        let mut tag_offset = 0u32;
        loop {
            let item = e.mem.u32(base + 0x14).wrapping_add(item_offset);
            let interpolator = e.mem.u32(item);
            e.vcall(stream.addr(), SLOT_SAVE_LINK_ID, &args![interpolator]);
            let controller = e.mem.u32(item + 4);
            e.vcall(stream.addr(), SLOT_SAVE_LINK_ID, &args![controller]);
            let output = output_of(e, stream);
            write_stream(e, output, item + 0xd, 1);
            let tag = e.mem.u32(base + 0x18).wrapping_add(tag_offset);
            for word in 0..5u32 {
                e.call(STREAM_SAVE_FIXED_STRING, &args![stream, tag + 4 * word]);
            }
            item_offset = item_offset.wrapping_add(INTERP_ITEM_SIZE);
            index = index.wrapping_add(1);
            tag_offset = tag_offset.wrapping_add(ID_TAG_SIZE);
            if index >= e.mem.u32(base + 0xc) {
                break;
            }
        }
    }
    let output = output_of(e, stream);
    write_stream(e, output, base + 0x1c, 4);
    let text_keys = e.mem.u32(base + 0x20);
    e.vcall(stream.addr(), SLOT_SAVE_LINK_ID, &args![text_keys]);
    for field in [0x24u32, 0x28, 0x2c, 0x30] {
        let output = output_of(e, stream);
        write_stream(e, output, base + field, 4);
    }
    let owner = e.mem.u32(base + 0x40);
    e.vcall(stream.addr(), SLOT_SAVE_LINK_ID, &args![owner]);
    e.call(STREAM_SAVE_FIXED_STRING, &args![stream, base + 0x5c]);
    let output = output_of(e, stream);
    write_stream(e, output, base + 0x70, 2);
    let mut note = 0u16;
    if e.mem.u16(base + 0x70) != 0 {
        loop {
            let notes = e.mem.u32(base + 0x6c);
            let object = e.mem.u32(notes.wrapping_add(u32::from(note) * 4));
            e.vcall(stream.addr(), SLOT_SAVE_LINK_ID, &args![object]);
            note = note.wrapping_add(1);
            if note >= e.mem.u16(base + 0x70) {
                break;
            }
        }
    }
}

// Translated from 00a33e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::LoadBinary` (Xbox PDB): runs `NiObject::LoadBinary` (`00a5d610`), then reads from
/// `stream`'s input (`+0x24c`; each read through its read function at `+8`, which adds the byte count it answers
/// to the position; `ReadLinkID` is `00a645b0`). Below NIF version `0x0a010068`: the name and the accum root
/// name as C strings (`00a64ea0`), a link id, the size (4 bytes), the growth set to 12, the arrays made, and for
/// each item a C string taken as the tag's object name and a link id. From that version on: the name (a C string
/// below `0x14010001`, `00a64df0` from then on), the size and growth (4 bytes each), the arrays made, each item
/// (`00a33950`) and tag (`00a33a50`), the weight, a link id, the cycle type, the frequency, a dropped 4 bytes
/// below `0x0a030001`, the begin and end, a dropped byte below `0x0a01006b`, a link id, the accum root name (as
/// for the name), the string palette (`+0x64`: the next link id's object for `0x0a010071` up to `0x14010001`,
/// otherwise released and cleared), for a BS stream version above 0x15 and below 0x1d a link id, and above 0x1c
/// the note count (2 bytes), an array of that many pointers (constructed by `00b54620`) and a link id each.
pub fn ni_controller_sequence_load_binary(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    stream: Ptr<NiStream>,
) {
    let base = this.addr();
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    let version = e.get(stream, NiStream::m_uiNifFileVersion);
    if version < 0x0a01_0068 {
        e.call(STREAM_LOAD_C_STRING, &args![stream, base + 8]);
        e.call(STREAM_LOAD_C_STRING, &args![stream, base + 0x5c]);
        e.call(STREAM_READ_LINK_ID, &args![stream]);
        let input = input_of(e, stream);
        read_stream(e, input, base + 0xc, 4);
        let size = e.mem.u32(base + 0xc);
        e.mem.set_u32(base + 0x10, 0xc);
        let items = new_element_array(e, size, INTERP_ITEM_SIZE, INTERP_ITEM_CONSTRUCTOR);
        let size = e.mem.u32(base + 0xc);
        e.mem.set_u32(base + 0x14, items);
        let tags = new_element_array(e, size, ID_TAG_SIZE, ID_TAG_CONSTRUCTOR);
        e.mem.set_u32(base + 0x18, tags);
        let mut index = 0u32;
        if e.mem.u32(base + 0xc) != 0 {
            e.with_stack(4, |e, local| loop {
                let empty = e.global::<u32>(EMPTY_FIXED_STRING);
                e.mem.set_u32(local.addr(), empty);
                e.call(STREAM_LOAD_C_STRING, &args![stream, local]);
                let tag = e
                    .mem
                    .u32(base + 0x18)
                    .wrapping_add(index.wrapping_mul(ID_TAG_SIZE));
                let handle = e.mem.u32(local.addr());
                assign_fixed_string(e, tag, handle);
                e.call(STREAM_READ_LINK_ID, &args![stream]);
                let handle = e.mem.u32(local.addr());
                fixed_string_release(e, handle);
                index = index.wrapping_add(1);
                if index >= e.mem.u32(base + 0xc) {
                    break;
                }
            });
        }
        return;
    }
    if version < 0x1401_0001 {
        e.call(STREAM_LOAD_C_STRING, &args![stream, base + 8]);
    } else {
        e.call(STREAM_LOAD_FIXED_STRING, &args![stream, base + 8]);
    }
    let input = input_of(e, stream);
    read_stream(e, input, base + 0xc, 4);
    let input = input_of(e, stream);
    read_stream(e, input, base + 0x10, 4);
    let size = e.mem.u32(base + 0xc);
    let items = new_element_array(e, size, INTERP_ITEM_SIZE, INTERP_ITEM_CONSTRUCTOR);
    let size = e.mem.u32(base + 0xc);
    e.mem.set_u32(base + 0x14, items);
    let tags = new_element_array(e, size, ID_TAG_SIZE, ID_TAG_CONSTRUCTOR);
    e.mem.set_u32(base + 0x18, tags);
    e.with_stack(8, |e, scratch| {
        let word = scratch.addr();
        let mut index = 0u32;
        if e.mem.u32(base + 0xc) != 0 {
            let mut item_offset = 0u32;
            let mut tag_offset = 0u32;
            loop {
                let item = e.mem.u32(base + 0x14).wrapping_add(item_offset);
                fn_00a33950(e, Ptr::new(item), stream);
                let tag = e.mem.u32(base + 0x18).wrapping_add(tag_offset);
                ni_controller_sequence_id_tag_load_binary(e, Ptr::new(tag), stream, scratch);
                item_offset = item_offset.wrapping_add(INTERP_ITEM_SIZE);
                tag_offset = tag_offset.wrapping_add(ID_TAG_SIZE);
                index = index.wrapping_add(1);
                if index >= e.mem.u32(base + 0xc) {
                    break;
                }
            }
        }
        let input = input_of(e, stream);
        read_stream(e, input, base + 0x1c, 4);
        e.call(STREAM_READ_LINK_ID, &args![stream]);
        let input = input_of(e, stream);
        read_stream(e, input, word, 4);
        let cycle_type = e.mem.u32(word);
        e.mem.set_u32(base + 0x24, cycle_type);
        let input = input_of(e, stream);
        read_stream(e, input, base + 0x28, 4);
        if e.get(stream, NiStream::m_uiNifFileVersion) < 0x0a03_0001 {
            let input = input_of(e, stream);
            read_stream(e, input, word, 4);
        }
        let input = input_of(e, stream);
        read_stream(e, input, base + 0x2c, 4);
        let input = input_of(e, stream);
        read_stream(e, input, base + 0x30, 4);
        if e.get(stream, NiStream::m_uiNifFileVersion) < 0x0a01_006b {
            let input = input_of(e, stream);
            read_stream(e, input, word, 1);
        }
    });
    e.call(STREAM_READ_LINK_ID, &args![stream]);
    if e.get(stream, NiStream::m_uiNifFileVersion) < 0x1401_0001 {
        e.call(STREAM_LOAD_C_STRING, &args![stream, base + 0x5c]);
    } else {
        e.call(STREAM_LOAD_FIXED_STRING, &args![stream, base + 0x5c]);
    }
    let version = e.get(stream, NiStream::m_uiNifFileVersion);
    if (0x0a01_0071..0x1401_0001).contains(&version) {
        let palette = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
        assign_pointer(e, base + 0x64, palette);
    } else {
        let palette = e.mem.u32(base + 0x64);
        if palette != 0 {
            release(e, palette);
            e.mem.set_u32(base + 0x64, 0);
        }
    }
    let stream_version = e.get(stream, NiStream::m_BSStreamHeader);
    if stream_version > 0x15 && stream_version < 0x1d {
        e.call(STREAM_READ_LINK_ID, &args![stream]);
    }
    if e.get(stream, NiStream::m_BSStreamHeader) > 0x1c {
        let input = input_of(e, stream);
        read_stream(e, input, base + 0x70, 2);
        let notes = u32::from(e.mem.u16(base + 0x70));
        if notes != 0 {
            let array = new_element_array(e, notes, 4, POINTER_CONSTRUCTOR);
            e.mem.set_u32(base + 0x6c, array);
        }
        let mut note = 0u16;
        if e.mem.u16(base + 0x70) != 0 {
            loop {
                e.call(STREAM_READ_LINK_ID, &args![stream]);
                note = note.wrapping_add(1);
                if note >= e.mem.u16(base + 0x70) {
                    break;
                }
            }
        }
    }
}

// Translated from 00a343c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): copies this sequence into `destination` over what it
/// has. Every item of `destination` is taken out (`00a317e0` into a temporary pointer that is released), its
/// item and tag arrays are deleted (`00a31500`, `00a315b0` with flags 3, when set; the pointers are not cleared), then
/// the member copy `00a33570` runs for `process`.
pub fn fn_00a343c0(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    destination: Ptr<NiControllerSequence>,
    process: Ptr,
) {
    let mut index = 0u32;
    if e.get(destination, NiControllerSequence::m_uiArraySize) != 0 {
        loop {
            e.with_stack(4, |e, local| {
                fn_00a317e0(e, destination, index, local);
                let object = e.mem.u32(local.addr());
                if object != 0 {
                    release(e, object);
                }
            });
            index = index.wrapping_add(1);
            if index >= e.get(destination, NiControllerSequence::m_uiArraySize) {
                break;
            }
        }
    }
    let items = e.get(destination, NiControllerSequence::m_pkInterpArray);
    if !items.is_null() {
        fn_00a31500(e, items, 3);
    }
    let tags = e.get(destination, NiControllerSequence::m_pkIDTagArray);
    if !tags.is_null() {
        fn_00a315b0(e, tags, 3);
    }
    fn_00a33570(e, this, destination, process);
}

// Translated from 00a34450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the scalar deleting destructor of a
/// `NiTArray<NiPointer<NiInterpController>>` (`this` = the array): stores its vtable (`01096620`), frees the
/// array of pointers at `+4` (`00a47600`, `cdecl`: releases each element from the last to the first and
/// deallocates the block), and frees `this` with the memory manager's `Deallocate` when bit 0 of `flags` is set.
/// Answers `this`.
pub fn fn_00a34450(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), INTERP_CONTROLLER_ARRAY_VTABLE);
    let elements = e.mem.u32(this.addr() + 4);
    e.call(POINTER_ARRAY_FREE, &args![elements]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a34480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): false unless `text` and `controller` are both set and
/// `controller`'s class chain (its `GetRTTI`, virtual slot 8, the parent links at `+4`) contains
/// `NiInterpController` (`011f36b4`). Then the controller's flags (`+8`) get this sequence's cycle type (the low
/// word, doubled) in bits 1 and 2 and lose bit 4, its frequency (`+0xc`) is this sequence's (`+0x28`,
/// through the x87 stack) and the answer is `00a33130` for `text` and the controller.
pub fn fn_00a34480(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    text: Ptr,
    controller: Ptr,
) -> bool {
    if text.is_null() || controller.is_null() {
        return false;
    }
    let mut rtti = e.vcall(controller.addr(), SLOT_GET_RTTI, &args![]).u32();
    while rtti != 0 {
        if rtti == NI_INTERP_CONTROLLER_RTTI {
            let cycle_type = e.mem.u16(this.addr() + 0x24);
            let flags = e.mem.u16(controller.addr() + 8);
            e.mem.set_u16(
                controller.addr() + 8,
                cycle_type.wrapping_add(cycle_type) | (flags & 0xfff9),
            );
            let frequency = quiet_word(e, this.addr() + 0x28);
            e.mem.set_u32(controller.addr() + 0xc, frequency);
            let flags = e.mem.u16(controller.addr() + 8);
            e.mem.set_u16(controller.addr() + 8, flags & 0xffef);
            return fn_00a33130(e, this, text, controller);
        }
        rtti = e.mem.u32(rtti + 4);
    }
    false
}

// Translated from 00a344f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::CreateClone` (Xbox PDB; `thiscall`, `RET 4`): makes a sequence (`operator new` of 0x74
/// bytes, constructed empty by `00a316a0`) and copies this one into it with `00a33570` for `process`. When the
/// allocation answers 0 the copy still runs, into address 0. Answers the new sequence.
pub fn ni_controller_sequence_create_clone(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    process: Ptr,
) -> Ptr<NiControllerSequence> {
    let block = e.call(OPERATOR_NEW, &args![0x74u32]).u32();
    let mut clone = 0u32;
    if block != 0 {
        clone = e.call(SEQUENCE_CONSTRUCT_EMPTY, &args![block]).u32();
    }
    fn_00a33570(e, this, Ptr::new(clone), process);
    Ptr::new(clone)
}

/// The names `GetViewerStrings` prints: `"State"`, `"ArraySize"`, `"Weight"`, `"CycleType"`, `"Freq"`,
/// `"Begin"`, `"End"` (the one string for the end time and for the end of the active range), `"Last"`,
/// `"Offset"`, `"Start"`, `"WeightLast"`, `"LastScaled"`, and the formats `"Target %d, "` and
/// `"%s, %s, %d"` of the per-item line.
const TEXT_STATE: u32 = 0x0104_4a7c;
const TEXT_ARRAY_SIZE: u32 = 0x0109_6678;
const TEXT_WEIGHT: u32 = 0x0105_ac84;
const TEXT_CYCLE_TYPE: u32 = 0x0109_666c;
const TEXT_FREQ: u32 = 0x0109_6664;
const TEXT_BEGIN: u32 = 0x0104_4c94;
const TEXT_END: u32 = 0x0104_4c90;
const TEXT_LAST: u32 = 0x0109_665c;
const TEXT_OFFSET: u32 = 0x0109_6654;
const TEXT_START: u32 = 0x0104_77f4;
const TEXT_WEIGHT_LAST: u32 = 0x0109_6648;
const TEXT_LAST_SCALED: u32 = 0x0109_663c;
const TEXT_FORMAT_TARGET: u32 = 0x0109_6630;
const TEXT_FORMAT_TARGET_LINE: u32 = 0x0109_6624;

// Translated from 00a34540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a34540 - the /GS cookie check (00ec408c) is not modeled
/// `NiControllerSequence::GetViewerStrings` (Xbox PDB; `thiscall`, `RET 4`): `NiObject`'s (`00a5d6e0`), then
/// appends to the viewer-string array `strings` (the 16-bit capacity `+8`, size `+0xa`, count `+0xc`, growth `+0xe`;
/// growing it with `0096ad30` and storing with `0096ae90`) the class name, `"State"` (this unit's
/// `GetViewerString` of the state), `"ArraySize"` (`NiGetViewerString_ov8`), then `NiGetViewerString_ov9` of the
/// weight (`+0x1c`), the cycle type (`NiTimeController::GetViewerString_ov2`, `00a6d190`), the frequency
/// (`+0x28`), begin (`+0x2c`), end (`+0x30`), last time (`+0x34`), offset (`+0x48`), start (`+0x4c`), end time (`+0x50`),
/// weighted last time (`+0x38`) and last scaled time (`+0x3c`), each float through the x87 stack. Then for each
/// item the line `NiGetViewerString_ov11` makes of `"Target %d, "` (`NiSprintf` into 0x80 bytes) and
/// `"%s, %s, %d"` (the tag's object name and controller type and the item's priority, `NiSprintf`
/// into 0x100 bytes), stored with the array's `SetAt` inlined (the size becomes the index plus one when the
/// index is at or past it, the count follows the string being set or not, the pointer is stored without
/// counting).
pub fn ni_controller_sequence_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    strings: Ptr,
) {
    let base = this.addr();
    let array = strings.addr();
    e.call(OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(NI_CONTROLLER_SEQUENCE_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
    append_viewer_string(e, array, text);
    let state = e.mem.u32(base + 0x44);
    let text = ni_controller_sequence_get_viewer_string(e, TEXT_STATE, state);
    append_viewer_string(e, array, text);
    let size = e.mem.u32(base + 0xc);
    let text = e
        .call(GET_VIEWER_STRING_OV8, &args![TEXT_ARRAY_SIZE, size])
        .u32();
    append_viewer_string(e, array, text);
    let weight = quiet_word(e, base + 0x1c);
    let text = e
        .call(GET_VIEWER_STRING_OV9, &args![TEXT_WEIGHT, weight])
        .u32();
    append_viewer_string(e, array, text);
    let cycle_type = e.mem.u32(base + 0x24);
    let text = e
        .call(
            TIME_CONTROLLER_GET_VIEWER_STRING,
            &args![TEXT_CYCLE_TYPE, cycle_type],
        )
        .u32();
    append_viewer_string(e, array, text);
    for (name, field) in [
        (TEXT_FREQ, 0x28u32),
        (TEXT_BEGIN, 0x2c),
        (TEXT_END, 0x30),
        (TEXT_LAST, 0x34),
        (TEXT_OFFSET, 0x48),
        (TEXT_START, 0x4c),
        (TEXT_END, 0x50),
        (TEXT_WEIGHT_LAST, 0x38),
        (TEXT_LAST_SCALED, 0x3c),
    ] {
        let value = quiet_word(e, base + field);
        let text = e.call(GET_VIEWER_STRING_OV9, &args![name, value]).u32();
        append_viewer_string(e, array, text);
    }
    let mut index = 0u32;
    if e.mem.u32(base + 0xc) != 0 {
        let mut item_offset = 0u32;
        let mut tag_offset = 0u32;
        e.with_stack(0x80 + 0x100, |e, buffers| {
            let short_buffer = buffers.addr();
            let long_buffer = short_buffer + 0x80;
            loop {
                let tag = e.mem.u32(base + 0x18).wrapping_add(tag_offset);
                let item = e.mem.u32(base + 0x14).wrapping_add(item_offset);
                e.call(
                    NI_SPRINTF,
                    &args![short_buffer, 0x80u32, TEXT_FORMAT_TARGET, index],
                );
                let priority = u32::from(e.mem.u8(item + 0xd));
                let object_name = e.mem.u32(tag);
                let controller_type = e.mem.u32(tag + 8);
                e.call(
                    NI_SPRINTF,
                    &args![
                        long_buffer,
                        0x100u32,
                        TEXT_FORMAT_TARGET_LINE,
                        object_name,
                        controller_type,
                        priority
                    ],
                );
                let text = e
                    .call(GET_VIEWER_STRING_OV11, &args![short_buffer, long_buffer])
                    .u32();
                let slot = u32::from(e.mem.u16(array + 0xa));
                if slot >= u32::from(e.mem.u16(array + 8)) {
                    let step = u32::from(e.mem.u16(array + 0xe));
                    e.call(VIEWER_STRINGS_GROW, &args![array, step + slot]);
                }
                let count = e.mem.u16(array + 0xc);
                let elements = e.mem.u32(array + 4);
                if slot >= u32::from(e.mem.u16(array + 0xa)) {
                    e.mem.set_u16(array + 0xa, slot.wrapping_add(1) as u16);
                    if text != 0 {
                        e.mem.set_u16(array + 0xc, count.wrapping_add(1));
                    }
                } else if text != 0 {
                    if e.mem.u32(elements.wrapping_add(slot * 4)) == 0 {
                        e.mem.set_u16(array + 0xc, count.wrapping_add(1));
                    }
                } else if e.mem.u32(elements.wrapping_add(slot * 4)) != 0 {
                    e.mem.set_u16(array + 0xc, count.wrapping_sub(1));
                }
                let elements = e.mem.u32(array + 4);
                e.mem.set_u32(elements.wrapping_add(slot * 4), text);
                tag_offset = tag_offset.wrapping_add(ID_TAG_SIZE);
                item_offset = item_offset.wrapping_add(INTERP_ITEM_SIZE);
                index = index.wrapping_add(1);
                if index >= e.mem.u32(base + 0xc) {
                    break;
                }
            }
        });
    }
}

// Translated from 00a349e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two stack words, no `this`): makes a `NiControllerSequence` from the
/// object `source` (its name handle at `+8`, a 16-bit count at `+0x16`, an array of texts at `+0x10`, an array of
/// controllers at `+0x20` and a text-key object at `+0x2c`) and assigns it to the `NiPointer` at `out`. The
/// sequence is made like `00a32a10` does (0x74 bytes, the name from the text, the count as size and growth) and
/// counted. For each index with both a text and a controller: for the first one the cycle type
/// (`+0x24`) is taken from bits 1 and 2 of the controller's flags (`+8`; only 0 and 2 are kept, 1 and 3 become 0) and
/// the frequency, begin and end (`+0x28`, `+0x2c`, `+0x30`) from the controller's `+0xc`, `+0x14` and `+0x18`
/// through the x87 stack; `00a34480` then adds the controller for the text. Then the text keys (`+0x20`) are
/// assigned as a `NiPointer`, the accum root name (`+0x5c`) is assigned the static name `011f49e0` as a
/// `NiFixedString`, `out` is assigned the sequence and the creation count is released.
pub fn fn_00a349e0(e: &mut Engine, source: Ptr, out: Ptr) {
    let source = source.addr();
    let block = e.call(OPERATOR_NEW, &args![0x74u32]).u32();
    let mut sequence = 0u32;
    if block != 0 {
        let name = e.mem.u32(source + 8);
        let count = u32::from(e.mem.u16(source + 0x16));
        let handle = fixed_string_or_empty(e, name);
        sequence = e.with_stack(4, |e, name_slot| {
            e.mem.set_u32(name_slot.addr(), handle);
            fn_00a32a10(e, Ptr::new(block), name_slot, count, count).addr()
        });
        if sequence != 0 {
            add_ref(e, sequence);
        }
        fixed_string_release(e, handle);
    }
    let this = Ptr::<NiControllerSequence>::new(sequence);
    let mut first = true;
    let mut index = 0u32;
    if e.mem.u16(source + 0x16) != 0 {
        loop {
            let texts = e.mem.u32(source + 0x10);
            let text = e.mem.u32(texts.wrapping_add(index * 4));
            let controllers = e.mem.u32(source + 0x20);
            let controller = e.mem.u32(controllers.wrapping_add(index * 4));
            if text != 0 && controller != 0 {
                if first {
                    let flags = u32::from(e.mem.u16(controller + 8));
                    let mut cycle_type = (flags >> 1) & 3;
                    if cycle_type != 2 && cycle_type != 0 {
                        cycle_type = 0;
                    }
                    e.mem.set_u32(sequence + 0x24, cycle_type);
                    let frequency = quiet_word(e, controller + 0xc);
                    e.mem.set_u32(sequence + 0x28, frequency);
                    first = false;
                    let begin = quiet_word(e, controller + 0x14);
                    e.mem.set_u32(sequence + 0x2c, begin);
                    let end = quiet_word(e, controller + 0x18);
                    e.mem.set_u32(sequence + 0x30, end);
                }
                fn_00a34480(e, this, Ptr::new(text), Ptr::new(controller));
            }
            index = index.wrapping_add(1);
            if index >= u32::from(e.mem.u16(source + 0x16)) {
                break;
            }
        }
    }
    let text_keys = e.mem.u32(source + 0x2c);
    assign_pointer(e, sequence + 0x20, text_keys);
    let root_name = e.mem.u32(FIXED_STRING_011F49E0);
    assign_fixed_string(e, sequence + 0x5c, root_name);
    assign_pointer(e, out.addr(), sequence);
    if sequence != 0 {
        release(e, sequence);
    }
}

/// Holds `time` (and its stack word `time_bits`) at the start time (`+0x4c`) when the start time is above it:
/// the exe's compare of the start time with the time (flags of `FCOMP`: only an ordered "above" moves it),
/// the start time loaded and stored back as the time float.
fn clamp_time_to_start(e: &Engine, base: u32, time: &mut f64, time_bits: &mut u32) {
    if load_float(e, base + 0x4c) > *time {
        *time_bits = quiet_word(e, base + 0x4c);
        *time = x87_load32(*time_bits);
    }
}

/// The ratio `Update` makes of `numerator` and the length of the active range: `numerator / (end - start)` with
/// the end time loaded and the start time (`+0x4c`) a memory operand, divided on the x87 stack, stored as a float.
fn ratio(e: &Engine, base: u32, numerator: f64) -> u32 {
    let start = x87_m32(e.mem.u32(base + 0x4c));
    let end = load_float(e, base + 0x50);
    x87_store32(x87_div(numerator, x87_sub(end, start)))
}

// Translated from 00a34ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::Update` (Xbox PDB; `thiscall`, `RET 8`): steps the sequence to `time` (the update
/// flag `update` is the byte of the second word). In the state 0 only curAnimNIdx (`+0x68`) is set to 0xffff.
/// An offset (`+0x48`) of `-FLT_MAX` becomes the negated time, a start time (`+0x4c`) of `-FLT_MAX` becomes the
/// time and the end time (`+0x50`) gains it. Two ratios start as 1.0: the weight ratio and the second ratio. State
/// 1 (animating) takes the last scaled time (`+0x3c`) as the start time. State 2 (ease in) and 5 (transition
/// destination) give the ratio (time - start) / (end - start) with the time held at the start time or above
/// (the first to the weight ratio, the second to the second ratio), and when the end time is not above the time
/// the state becomes 1, the start time 0.0 (and, for state 5, a destination frame (`+0x54`) other than `-FLT_MAX`
/// becomes the offset as frame minus time and is set back to `-FLT_MAX`). State 3 (ease out) and 4 (transition
/// source) give the ratio (end - time) / (end - start) (to the weight ratio, resp. the second ratio) and when the end time
/// is not above the time call virtual slot 0x8c (`Deactivate`) with `(0.0, 0)` resp. `(0.0, 1)`. State 6 (morph
/// source) takes the partner's (`+0x58`) `FindCorrespondingMorphFrame` of `time + offset` divided by the
/// partner's frequency as the second ratio, sets the partner's offset to that minus the time, the state to 4,
/// and goes on as state 4. With `update` set the time is then replaced by the destination frame when there is
/// one, otherwise it is the time plus the offset, or, with a partner, the morph frame of the partner's offset
/// time (after updating the partner to the time when its last time is not its offset plus the time) divided by
/// the frequency; the scaled time of that (`ComputeScaledTime` updating) and the weight times the second
/// ratio, the first ratio and the scaled time go to `00a32290`. Finally the notes (`+0x6c`, count `+0x70`)
/// whose time is below the new time and that come after curAnimNIdx are announced (`00a41410`, to the owner's
/// listener `+0x64`) and curAnimNIdx follows them; a last scaled time that went down restarts it at -1.
/// Every float operation is the x87's.
pub fn ni_controller_sequence_update(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    time: f32,
    update: u8,
) {
    let base = this.addr();
    let state = e.get(this, NiControllerSequence::m_eState);
    if state == 0 {
        e.set(this, NiControllerSequence::ssCurAnimNIdx, -1);
        return;
    }
    let mut time_bits = time.to_bits();
    let mut t = x87_load32(time_bits);
    let negative_max = e.global::<f64>(NEGATIVE_FLT_MAX_DOUBLE);
    if load_float(e, base + 0x48) == negative_max {
        let negated = f64::from_bits(t.to_bits() ^ (1 << 63));
        e.mem.set_u32(base + 0x48, x87_store32(negated));
    }
    if load_float(e, base + 0x4c) == negative_max {
        e.mem.set_u32(base + 0x4c, x87_store32(t));
        let end = x87_add(load_float(e, base + 0x50), t);
        e.mem.set_u32(base + 0x50, x87_store32(end));
    }
    let mut weight_ratio = x87_quiet(1.0f32).to_bits();
    let mut second_ratio = weight_ratio;
    let mut state = state;
    match state {
        1 => {
            let last_scaled = quiet_word(e, base + 0x3c);
            e.mem.set_u32(base + 0x4c, last_scaled);
        }
        2 | 5 => {
            if load_float(e, base + 0x50) > t {
                clamp_time_to_start(e, base, &mut t, &mut time_bits);
                let numerator = x87_sub(t, x87_m32(e.mem.u32(base + 0x4c)));
                let value = ratio(e, base, numerator);
                if state == 2 {
                    weight_ratio = value;
                } else {
                    second_ratio = value;
                }
            } else {
                if state == 5 && load_float(e, base + 0x54) != negative_max {
                    let offset = x87_sub(load_float(e, base + 0x54), t);
                    e.mem.set_u32(base + 0x48, x87_store32(offset));
                    let negative = quiet_word(e, NEGATIVE_FLT_MAX_FLOAT);
                    e.mem.set_u32(base + 0x54, negative);
                }
                e.set(this, NiControllerSequence::m_eState, 1);
                e.mem.set_u32(base + 0x4c, 0);
            }
        }
        3 | 4 | 6 => {
            if state == 6 {
                let partner = e.get(this, NiControllerSequence::m_pkPartnerSequence);
                let shifted = x87_add(t, x87_m32(e.mem.u32(base + 0x48)));
                let shifted_time = f32::from_bits(x87_store32(shifted));
                let frame = ni_controller_sequence_find_corresponding_morph_frame(
                    e,
                    Ptr::new(partner.addr()),
                    this,
                    shifted_time,
                );
                let frequency = x87_m32(e.mem.u32(partner.addr() + 0x28));
                let value_bits = x87_store32(x87_div(x87_load32(frame.to_bits()), frequency));
                second_ratio = value_bits;
                let offset = x87_sub(x87_load32(value_bits), t);
                e.mem.set_u32(partner.addr() + 0x48, x87_store32(offset));
                let was_inactive = e.get(this, NiControllerSequence::m_eState) == 0;
                e.set(this, NiControllerSequence::m_eState, 4);
                if was_inactive {
                    add_to_active_set(e, this);
                }
                state = 4;
            }
            if load_float(e, base + 0x50) > t {
                clamp_time_to_start(e, base, &mut t, &mut time_bits);
                let numerator = x87_sub(load_float(e, base + 0x50), t);
                let value = ratio(e, base, numerator);
                if state == 3 {
                    weight_ratio = value;
                } else {
                    second_ratio = value;
                }
            } else {
                e.vcall(base, SLOT_DEACTIVATE, &args![0u32, u32::from(state == 4)]);
                return;
            }
        }
        _ => {}
    }
    let last_scaled_before = quiet_word(e, base + 0x3c);
    if update == 0 {
        return;
    }
    let t = x87_load32(time_bits);
    let target_time = if load_float(e, base + 0x54) != negative_max {
        load_float(e, base + 0x54)
    } else {
        let partner = e.get(this, NiControllerSequence::m_pkPartnerSequence);
        if partner.is_null() {
            x87_add(t, x87_m32(e.mem.u32(base + 0x48)))
        } else {
            let partner_base = partner.addr();
            let behind = x87_add(x87_load32(e.mem.u32(partner_base + 0x48)), t);
            if load_float(e, partner_base + 0x34) != behind {
                ni_controller_sequence_update(
                    e,
                    Ptr::new(partner_base),
                    f32::from_bits(x87_store32(t)),
                    0,
                );
            }
            let t = x87_load32(time_bits);
            let partner_base = e
                .get(this, NiControllerSequence::m_pkPartnerSequence)
                .addr();
            let shifted = x87_add(t, x87_m32(e.mem.u32(partner_base + 0x48)));
            let shifted_time = f32::from_bits(x87_store32(shifted));
            let frame = ni_controller_sequence_find_corresponding_morph_frame(
                e,
                this,
                Ptr::new(partner_base),
                shifted_time,
            );
            x87_div(x87_load32(frame.to_bits()), x87_m32(e.mem.u32(base + 0x28)))
        }
    };
    let target_bits = x87_store32(target_time);
    let scaled =
        ni_controller_sequence_compute_scaled_time(e, this, f32::from_bits(target_bits), true);
    let first = x87_store32(x87_mul(load_float(e, base + 0x1c), x87_m32(second_ratio)));
    fn_00a32290(
        e,
        this,
        f32::from_bits(first),
        f32::from_bits(x87_quiet_bits(weight_ratio)),
        scaled,
    );
    if e.get(this, NiControllerSequence::spAnimNotesA).is_null() {
        return;
    }
    let before = x87_load32(last_scaled_before);
    if load_float(e, base + 0x3c) < before {
        e.set(this, NiControllerSequence::ssCurAnimNIdx, -1);
    }
    let mut index = e
        .get(this, NiControllerSequence::ssCurAnimNIdx)
        .wrapping_add(1) as u16;
    loop {
        let signed_index = i32::from(index as i16);
        if signed_index >= i32::from(e.get(this, NiControllerSequence::usNumNotes)) {
            break;
        }
        let notes = e.get(this, NiControllerSequence::spAnimNotesA).addr();
        let slot = notes.wrapping_add((signed_index as u32).wrapping_mul(4));
        let note = e.mem.u32(slot);
        let note_time = load_float(e, note + 0xc);
        let current = x87_load32(target_bits);
        if !(current > note_time) {
            break;
        }
        let owner = e.get(this, NiControllerSequence::m_pkOwner).addr();
        e.set(this, NiControllerSequence::ssCurAnimNIdx, index as i16);
        let note = e.mem.u32(slot);
        let listener = e.mem.u32(owner + 0x64);
        e.call(LISTENER_NOTIFY, &args![listener, note]);
        index = index.wrapping_add(1);
    }
}

// Translated from 00a34f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x18`): activates the sequence. False when the state (`+0x44`)
/// is not 0. The partner (`+0x58`) is cleared; with a `time_sync` sequence that has a partner other than this
/// sequence the answer is false unless `00a30580` accepts that partner, and with any `time_sync` it is false
/// unless `00a30ab0` accepts it; then it becomes the partner. The weight (`+0x1c`) is stored through the x87 stack and
/// `00a30900` runs with `priority`. A positive `ease_in` (a NaN is not) makes the state 5 (`transition` not 0) or 2
/// with the start time `-FLT_MAX` and the end time `ease_in`; otherwise the state is 1 and the start time 0.0. A
/// state that was 0 adds the sequence to the owner's active set (`00e7ec00` on the set at `+0x44` of the owner).
/// `start_over` not 0 sets the offset (`+0x48`) to `-FLT_MAX`. The last time (`+0x34`) is `-FLT_MAX`. True.
pub fn fn_00a34f20(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    priority: u32,
    start_over: u8,
    weight: f32,
    ease_in: f32,
    time_sync: Ptr<NiControllerSequence>,
    transition: u8,
) -> bool {
    let base = this.addr();
    if e.get(this, NiControllerSequence::m_eState) != 0 {
        return false;
    }
    e.set(this, NiControllerSequence::m_pkPartnerSequence, Ptr::NULL);
    if !time_sync.is_null() {
        let partner = e.get(time_sync, NiControllerSequence::m_pkPartnerSequence);
        if !partner.is_null() {
            if partner.addr() == base {
                return false;
            }
            if !e
                .call(SEQUENCE_PARTNER_CHAIN_CHECK, &args![this, partner])
                .bool()
            {
                return false;
            }
        }
        if !fn_00a30ab0(e, this, time_sync) {
            return false;
        }
        e.set(
            this,
            NiControllerSequence::m_pkPartnerSequence,
            Ptr::new(time_sync.addr()),
        );
    }
    e.mem.set_u32(base + 0x1c, x87_quiet_bits(weight.to_bits()));
    fn_00a30900(e, this, priority);
    let negative_max = quiet_word(e, NEGATIVE_FLT_MAX_FLOAT);
    if 0.0 < x87_load32(ease_in.to_bits()) {
        let old_state = e.get(this, NiControllerSequence::m_eState);
        e.set(
            this,
            NiControllerSequence::m_eState,
            if transition != 0 { 5 } else { 2 },
        );
        if old_state == 0 {
            add_to_active_set(e, this);
        }
        e.mem.set_u32(base + 0x4c, negative_max);
        e.mem
            .set_u32(base + 0x50, x87_quiet_bits(ease_in.to_bits()));
    } else {
        let old_state = e.get(this, NiControllerSequence::m_eState);
        e.set(this, NiControllerSequence::m_eState, 1);
        if old_state == 0 {
            add_to_active_set(e, this);
        }
        e.mem.set_u32(base + 0x4c, 0);
    }
    if start_over != 0 {
        e.mem.set_u32(base + 0x48, negative_max);
    }
    e.mem.set_u32(base + 0x34, negative_max);
    true
}

/// `NiTPrimitiveSet<NiControllerSequence *>::AddUnique` (`00e7ec00`) of this sequence into the active set of its
/// owner (the set at `+0x44` of the owner at `+0x40`), the element passed by address.
fn add_to_active_set(e: &mut Engine, this: Ptr<NiControllerSequence>) {
    let owner = e.get(this, NiControllerSequence::m_pkOwner).addr();
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), this.addr());
        ni_t_primitive_set_add_unique(e, Ptr::new(owner + 0x44), local);
    });
}

// Translated from 00a35030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::Deactivate` (Xbox PDB; `thiscall`, `RET 8`): false when the state (`+0x44`) is 0. With a
/// positive `time` (a NaN or 0 is not) the start time (`+0x4c`) becomes `-FLT_MAX`, the end time `time` and the state 4
/// for `transition` not 0 or 3 for 0; true. Otherwise (immediate) when the last time (`+0x34`) is not `-FLT_MAX`
/// the offset (`+0x48`) gains `weightedLast / frequency - lastTime` (every operation the x87's); a sequence
/// in the state 5 with an owner has the owner's `00a2e420` run with 0.0; the destination frame (`+0x54`)
/// becomes `-FLT_MAX`, the state and the partner (`+0x58`) 0, `00a30540` runs and the answer is true.
pub fn ni_controller_sequence_deactivate(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    time: f32,
    transition: u8,
) -> bool {
    let base = this.addr();
    let state = e.get(this, NiControllerSequence::m_eState);
    if state == 0 {
        return false;
    }
    let value = x87_load32(time.to_bits());
    if value > 0.0 {
        let negative_max = quiet_word(e, NEGATIVE_FLT_MAX_FLOAT);
        e.mem.set_u32(base + 0x4c, negative_max);
        e.mem.set_u32(base + 0x50, x87_store32(value));
        e.set(
            this,
            NiControllerSequence::m_eState,
            u32::from(transition != 0) + 3,
        );
        return true;
    }
    let negative_max = e.global::<f64>(NEGATIVE_FLT_MAX_DOUBLE);
    if load_float(e, base + 0x34) != negative_max {
        let offset = x87_add(
            x87_sub(
                x87_div(load_float(e, base + 0x38), x87_m32(e.mem.u32(base + 0x28))),
                x87_m32(e.mem.u32(base + 0x34)),
            ),
            x87_m32(e.mem.u32(base + 0x48)),
        );
        e.mem.set_u32(base + 0x48, x87_store32(offset));
    }
    if state == 5 {
        let owner = e.get(this, NiControllerSequence::m_pkOwner).addr();
        if owner != 0 {
            e.call(MANAGER_DEACTIVATE_SOURCES, &args![owner, 0u32]);
        }
    }
    let negative_max = quiet_word(e, NEGATIVE_FLT_MAX_FLOAT);
    e.mem.set_u32(base + 0x54, negative_max);
    e.set(this, NiControllerSequence::m_eState, 0);
    e.set(this, NiControllerSequence::m_pkPartnerSequence, Ptr::NULL);
    e.call(SEQUENCE_CLEAR_BLEND, &args![this]);
    true
}

// Translated from 00a350d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x1c`): cross-fades this sequence to `other`. Runs this sequence's
/// virtual slot 0x8c (`Deactivate`) with `(0.0, 1)`; a `duration` that is not positive (a NaN is kept) becomes
/// the float at `010141cc`. False unless both sequences are then in the state 0. Then this sequence takes
/// `weight` (`+0x1c`, through the x87 stack), no partner, `00a30900` with `priority`, the state 1 (adding
/// itself to the owner's active set), the start time 0.0 and the last time `-FLT_MAX`; `other` is activated with
/// `00a34f20` (`priority`, no start over, `other_weight`, `duration`, `time_sync`, transition set) and
/// false is the answer if that fails. Otherwise this sequence's `Deactivate(duration, 1)` runs, this
/// sequence's destination frame (`+0x54`) becomes its last scaled time (`+0x3c`) divided by its frequency
/// and `other`'s becomes `other_frame` divided by its frequency. True.
pub fn fn_00a350d0(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    other: Ptr<NiControllerSequence>,
    duration: f32,
    other_frame: f32,
    priority: u32,
    weight: f32,
    other_weight: f32,
    time_sync: Ptr<NiControllerSequence>,
) -> bool {
    let base = this.addr();
    e.vcall(base, SLOT_DEACTIVATE, &args![0u32, 1u32]);
    let mut duration = duration;
    if !(0.0 < x87_load32(duration.to_bits()) || duration.is_nan()) {
        duration = f32::from_bits(quiet_word(e, SMALL_TIME_010141CC));
    }
    if e.get(other, NiControllerSequence::m_eState) != 0
        || e.get(this, NiControllerSequence::m_eState) != 0
    {
        return false;
    }
    e.mem.set_u32(base + 0x1c, x87_quiet_bits(weight.to_bits()));
    e.set(this, NiControllerSequence::m_pkPartnerSequence, Ptr::NULL);
    fn_00a30900(e, this, priority);
    let old_state = e.get(this, NiControllerSequence::m_eState);
    e.set(this, NiControllerSequence::m_eState, 1);
    if old_state == 0 {
        add_to_active_set(e, this);
    }
    e.mem.set_u32(base + 0x4c, 0);
    let negative_max = quiet_word(e, NEGATIVE_FLT_MAX_FLOAT);
    e.mem.set_u32(base + 0x34, negative_max);
    if !fn_00a34f20(
        e,
        other,
        priority,
        0,
        x87_quiet(other_weight),
        x87_quiet(duration),
        time_sync,
        1,
    ) {
        return false;
    }
    e.vcall(
        base,
        SLOT_DEACTIVATE,
        &args![x87_quiet_bits(duration.to_bits()), 1u32],
    );
    let own_frame = x87_div(load_float(e, base + 0x3c), x87_m32(e.mem.u32(base + 0x28)));
    e.mem.set_u32(base + 0x54, x87_store32(own_frame));
    let other_frame = x87_div(
        x87_load32(other_frame.to_bits()),
        x87_m32(e.mem.u32(other.addr() + 0x28)),
    );
    e.mem.set_u32(other.addr() + 0x54, x87_store32(other_frame));
    true
}

// Translated from 00a351d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x14`): morphs this sequence into `other`. Runs this sequence's
/// virtual slot 0x8c (`Deactivate`) with `(0.0, 1)`, then activates this sequence with `00a34f20` (`priority`,
/// no start over, `weight`, `ease_in`, `other` as the time sync sequence, transition set). False if that fails
/// or `other` is not in the state 0. Otherwise `other` takes `other_weight` (`+0x1c`, through the x87 stack), no
/// partner, `00a30900` with `priority`, the state 5 (a positive `ease_in`: start time `-FLT_MAX`, end time
/// `ease_in`) or 1 (start time 0.0), adding itself to the owner's active set when it was 0, and the offset
/// (`+0x48`) and last time (`+0x34`) `-FLT_MAX`; this sequence goes to the state 6 (adding itself to its owner's active
/// set when it was 0). True.
pub fn fn_00a351d0(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    other: Ptr<NiControllerSequence>,
    ease_in: f32,
    priority: u32,
    weight: f32,
    other_weight: f32,
) -> bool {
    e.vcall(this.addr(), SLOT_DEACTIVATE, &args![0u32, 1u32]);
    if !fn_00a34f20(
        e,
        this,
        priority,
        0,
        x87_quiet(weight),
        x87_quiet(ease_in),
        other,
        1,
    ) {
        return false;
    }
    if e.get(other, NiControllerSequence::m_eState) != 0 {
        return false;
    }
    let other_base = other.addr();
    e.mem
        .set_u32(other_base + 0x1c, x87_quiet_bits(other_weight.to_bits()));
    e.set(other, NiControllerSequence::m_pkPartnerSequence, Ptr::NULL);
    fn_00a30900(e, other, priority);
    let negative_max = quiet_word(e, NEGATIVE_FLT_MAX_FLOAT);
    if 0.0 < x87_load32(ease_in.to_bits()) {
        let old_state = e.get(other, NiControllerSequence::m_eState);
        e.set(other, NiControllerSequence::m_eState, 5);
        if old_state == 0 {
            add_to_active_set(e, other);
        }
        e.mem.set_u32(other_base + 0x4c, negative_max);
        e.mem
            .set_u32(other_base + 0x50, x87_quiet_bits(ease_in.to_bits()));
    } else {
        let old_state = e.get(other, NiControllerSequence::m_eState);
        e.set(other, NiControllerSequence::m_eState, 1);
        if old_state == 0 {
            add_to_active_set(e, other);
        }
        e.mem.set_u32(other_base + 0x4c, 0);
    }
    e.mem.set_u32(other_base + 0x48, negative_max);
    e.mem.set_u32(other_base + 0x34, negative_max);
    let old_state = e.get(this, NiControllerSequence::m_eState);
    e.set(this, NiControllerSequence::m_eState, 6);
    if old_state == 0 {
        add_to_active_set(e, this);
    }
    true
}

// Translated from 00a352f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::PostLinkObject` (Xbox PDB): runs the `NiObject` one (`00a89af0`). A string palette
/// (`+0x64`) turns each tag's five offsets into strings (`00a30d40`) and is released and cleared. For a NIF version
/// below `0x0a010068` (the format that kept its controllers apart) the begin and end times (`+0x2c`, `+0x30`) start
/// as `FLT_MAX` and `-FLT_MAX` and become the lowest `+0x14` and highest `+0x18` of the items' controllers (strict
/// comparisons on the x87 flags; both left unchanged become 0.0). A temporary array of `count` pointers
/// (an `NiTArray` on the stack: vtable `01096688`, size 0, growth 1) takes each item's controller by `00a32940`
/// and a table of copies of the tag names (`00aa1070`, `strcpy_s`) is made, the tag is reset (`00a30640`) and the item
/// is cleared (interpolator and controller released, blend interpolator and blend index reset, priority 0xff).
/// Then for each stored controller in turn the first sets the cycle type (bits 1 and 2 of its flags `+8`, 1 and 3
/// becoming 0) and the frequency (`+0x28` from its `+0xc`, through the x87 stack), and `00a34480` adds it for the saved
/// name. The name copies are freed (`00aa10f0`), the pointer array is freed (`00a47600`) and the name table too.
pub fn ni_controller_sequence_post_link_object(
    e: &mut Engine,
    this: Ptr<NiControllerSequence>,
    stream: Ptr<NiStream>,
) {
    let base = this.addr();
    e.call(OBJECT_STREAM_HOOK, &args![this, stream]);
    let palette = e.mem.u32(base + 0x64);
    if palette != 0 {
        let mut index = 0u32;
        if e.mem.u32(base + 0xc) != 0 {
            let mut offset = 0u32;
            loop {
                let palette = e.mem.u32(base + 0x64);
                let tag = e.mem.u32(base + 0x18).wrapping_add(offset);
                fn_00a30d40(e, Ptr::new(tag), Ptr::new(palette), stream.addr());
                index = index.wrapping_add(1);
                offset = offset.wrapping_add(ID_TAG_SIZE);
                if index >= e.mem.u32(base + 0xc) {
                    break;
                }
            }
        }
        let palette = e.mem.u32(base + 0x64);
        if palette != 0 {
            release(e, palette);
            e.mem.set_u32(base + 0x64, 0);
        }
    }
    if e.get(stream, NiStream::m_uiNifFileVersion) >= 0x0a01_0068 {
        return;
    }
    let count = e.mem.u32(base + 0xc);
    let positive_max = quiet_word(e, FLT_MAX_FLOAT);
    e.mem.set_u32(base + 0x2c, positive_max);
    let negative_max = quiet_word(e, NEGATIVE_FLT_MAX_FLOAT);
    e.mem.set_u32(base + 0x30, negative_max);
    if count != 0 {
        let mut index = 0u32;
        let mut slot = e.mem.u32(base + 0x14).wrapping_add(4);
        loop {
            let controller = e.mem.u32(slot);
            let controller_begin = load_float(e, controller + 0x14);
            if load_float(e, base + 0x2c) > controller_begin {
                let value = quiet_word(e, controller + 0x14);
                e.mem.set_u32(base + 0x2c, value);
            }
            let controller_end = load_float(e, controller + 0x18);
            if load_float(e, base + 0x30) < controller_end {
                let value = quiet_word(e, controller + 0x18);
                e.mem.set_u32(base + 0x30, value);
            }
            index = index.wrapping_add(1);
            slot = slot.wrapping_add(INTERP_ITEM_SIZE);
            if index >= count {
                break;
            }
        }
    }
    if load_float(e, base + 0x2c) == e.global::<f64>(FLT_MAX_DOUBLE)
        && load_float(e, base + 0x30) == e.global::<f64>(NEGATIVE_FLT_MAX_DOUBLE)
    {
        e.mem.set_u32(base + 0x30, 0);
        e.mem.set_u32(base + 0x2c, 0);
    }
    let count16 = e.mem.u16(base + 0xc);
    let names = if count16 != 0 {
        e.call(TABLE_ALLOCATE, &args![u32::from(count16) * 4]).u32()
    } else {
        0
    };
    let mut name_count = 0u16;
    e.with_stack(0x10, |e, array| {
        let array_base = array.addr();
        e.mem.set_u16(array_base + 0xe, 1);
        e.mem.set_u16(array_base + 8, count16);
        e.mem.set_u16(array_base + 0xa, 0);
        e.mem.set_u16(array_base + 0xc, 0);
        let pointers = if count16 != 0 {
            new_pointer_array(e, u32::from(count16))
        } else {
            0
        };
        e.mem.set_u32(array_base + 4, pointers);
        e.mem.set_u32(array_base, SEQUENCE_CONTROLLER_ARRAY_VTABLE);
        let mut index = 0u32;
        if e.mem.u32(base + 0xc) != 0 {
            let mut item_offset = 0u32;
            let mut tag_offset = 0u32;
            loop {
                let source = e.mem.u32(e.mem.u32(base + 0x18).wrapping_add(tag_offset));
                let length = string_length(e, source).wrapping_add(1);
                let copy = e.call(TABLE_ALLOCATE, &args![length]).u32();
                e.call(STRING_COPY_SECURE, &args![copy, length, source]);
                if index >= u32::from(name_count) {
                    name_count = index.wrapping_add(1) as u16;
                }
                e.mem
                    .set_u32(names.wrapping_add(index.wrapping_mul(4)), copy);
                let tag = e.mem.u32(base + 0x18).wrapping_add(tag_offset);
                e.call(ID_TAG_RESET, &args![tag]);
                let items = e.mem.u32(base + 0x14);
                let controller = e.mem.u32(items.wrapping_add(item_offset) + 4);
                if controller != 0 {
                    add_ref(e, controller);
                }
                e.with_stack(4, |e, element| {
                    e.mem.set_u32(element.addr(), controller);
                    fn_00a32940(e, Ptr::new(array_base), index, element);
                });
                if controller != 0 {
                    release(e, controller);
                }
                let item = e.mem.u32(base + 0x14).wrapping_add(item_offset);
                let interpolator = e.mem.u32(item);
                if interpolator != 0 {
                    release(e, interpolator);
                    e.mem.set_u32(item, 0);
                }
                let controller = e.mem.u32(item + 4);
                if controller != 0 {
                    release(e, controller);
                    e.mem.set_u32(item + 4, 0);
                }
                tag_offset = tag_offset.wrapping_add(ID_TAG_SIZE);
                item_offset = item_offset.wrapping_add(INTERP_ITEM_SIZE);
                e.mem.set_u32(item + 8, 0);
                let blend_index = e.mem.u8(INVALID_BLEND_INDEX);
                index = index.wrapping_add(1);
                e.mem.set_u8(item + 0xc, blend_index);
                e.mem.set_u8(item + 0xd, 0xff);
                if index >= e.mem.u32(base + 0xc) {
                    break;
                }
            }
        }
        let stored = u32::from(e.mem.u16(array_base + 0xa));
        let mut position = 0u32;
        while position < stored {
            let controllers = e.mem.u32(array_base + 4);
            let controller = e.mem.u32(controllers.wrapping_add(position * 4));
            if position == 0 {
                let flags = u32::from(e.mem.u16(controller + 8));
                let cycle_type = (flags >> 1) & 3;
                e.mem.set_u32(base + 0x24, cycle_type);
                if cycle_type != 2 && cycle_type != 0 {
                    e.mem.set_u32(base + 0x24, position);
                }
                let frequency = quiet_word(e, controller + 0xc);
                e.mem.set_u32(base + 0x28, frequency);
            }
            let text = e.mem.u32(names.wrapping_add(position * 4));
            fn_00a34480(e, this, Ptr::new(text), Ptr::new(controller));
            position += 1;
        }
        for position in 0..u32::from(name_count) {
            let copy = e.mem.u32(names.wrapping_add(position * 4));
            e.call(TABLE_FREE, &args![copy]);
        }
        let controllers = e.mem.u32(array_base + 4);
        e.call(POINTER_ARRAY_FREE, &args![controllers]);
        e.call(TABLE_FREE, &args![names]);
    });
}

// Translated from 00a35640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerSequence::~NiControllerSequence` (Xbox PDB; `fastcall`): stores the vtable (`01096584`); a
/// state other than 0 runs `Deactivate(0.0, 0)` (`00a35030`); the item array, the tag array (flags 3 of
/// `00a31500`, `00a315b0`, when set) and the note array (`00a31740`, when it is set and the count `+0x70` is not 0)
/// are deleted; the string palette (`+0x64`), the text keys (`+0x20`) are released and the accum root name (`+0x5c`) and
/// name (`+8`) handles lose a count (unless they are the empty string); then `NiObject`'s destructor
/// body (`00a5d3d0`) is jumped to.
pub fn ni_controller_sequence_destructor(e: &mut Engine, this: Ptr<NiControllerSequence>) {
    let base = this.addr();
    e.mem.set_u32(base, NI_CONTROLLER_SEQUENCE_VTABLE);
    if e.get(this, NiControllerSequence::m_eState) != 0 {
        ni_controller_sequence_deactivate(e, this, 0.0, 0);
    }
    let items = e.get(this, NiControllerSequence::m_pkInterpArray);
    if !items.is_null() {
        fn_00a31500(e, items, 3);
    }
    let tags = e.get(this, NiControllerSequence::m_pkIDTagArray);
    if !tags.is_null() {
        fn_00a315b0(e, tags, 3);
    }
    if e.get(this, NiControllerSequence::usNumNotes) != 0 {
        let notes = e.get(this, NiControllerSequence::spAnimNotesA);
        if !notes.is_null() {
            fn_00a31740(e, notes, 3);
        }
    }
    let palette = e.mem.u32(base + 0x64);
    if palette != 0 {
        release(e, palette);
    }
    let root_name = e.mem.u32(base + 0x5c);
    fixed_string_release(e, root_name);
    let text_keys = e.mem.u32(base + 0x20);
    if text_keys != 0 {
        release(e, text_keys);
    }
    let name = e.mem.u32(base + 8);
    fixed_string_release(e, name);
    e.call(OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00a35700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three stack words, no `this`): `NiPointer`-assigns to `out` the
/// sequence stored as top object `index` of `stream` (the array at `+0x21c`: elements `+0x220`, size `+0x228`). An
/// index at or beyond the size releases and clears `out`. Below NIF version `0x05000000` the object is
/// fetched by `00a47f40` (`(stream, index, &object, 0)`), a sequence is made from it by `00a349e0`
/// and the fetched object released; from that version on the top object is cast to `NiControllerSequence`
/// (`00653290`, answering 0 for an empty slot) and taken through `0066b0d0`. `out` is then assigned the
/// result as the `NiPointer` assignment does and the temporary count released.
pub fn fn_00a35700(e: &mut Engine, stream: Ptr<NiStream>, index: u32, out: Ptr) {
    let stream_base = stream.addr();
    if index >= e.mem.u32(stream_base + 0x228) {
        let old = e.mem.u32(out.addr());
        if old != 0 {
            release(e, old);
            e.mem.set_u32(out.addr(), 0);
        }
        return;
    }
    let mut result = 0u32;
    e.with_stack(8, |e, scratch| {
        let result_slot = scratch.addr();
        let object_slot = result_slot + 4;
        e.mem.set_u32(result_slot, 0);
        if e.get(stream, NiStream::m_uiNifFileVersion) < 0x0500_0000 {
            e.mem.set_u32(object_slot, 0);
            e.call(
                STREAM_GET_OBJECT_AT,
                &args![stream, index, object_slot, 0u32],
            );
            let object = e.mem.u32(object_slot);
            fn_00a349e0(e, Ptr::new(object), Ptr::new(result_slot));
            let object = e.mem.u32(object_slot);
            if object != 0 {
                release(e, object);
            }
        } else {
            let objects = e.mem.u32(stream_base + 0x220);
            let top_object = e.mem.u32(objects.wrapping_add(index.wrapping_mul(4)));
            let sequence = if top_object == 0 {
                0
            } else {
                e.call(OBJECT_CAST, &args![top_object, NI_CONTROLLER_SEQUENCE_RTTI])
                    .u32()
            };
            e.call(POINTER_SLOT_ASSIGN, &args![result_slot, sequence]);
        }
        result = e.mem.u32(result_slot);
    });
    assign_pointer(e, out.addr(), result);
    if result != 0 {
        release(e, result);
    }
}

// Translated from 00e7ec00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveSet<NiControllerSequence *>::AddUnique` (`thiscall`, `RET 4`; the engine map names this address
/// `NiControllerSequence::SetState`, which is what called it): `this` is the set (elements `+0`, capacity `+4`, used `+8`),
/// `element` points at the pointer to add. Does nothing when the element is already in the set; otherwise,
/// with the set full, grows it to twice its capacity (to 1 when it has none; `NiTSet::Realloc`, `00a311e0`) and appends it.
pub fn ni_t_primitive_set_add_unique(e: &mut Engine, this: Ptr<NiTPrimitiveSet>, element: Ptr) {
    let used = e.get(this, NiTPrimitiveSet::m_uiUsed);
    let key = e.mem.u32(element.addr());
    let mut index = 0u32;
    if used != 0 {
        let mut slot = e.get(this, NiTPrimitiveSet::m_pBase);
        loop {
            if e.mem.u32(slot) == key {
                if index != 0xffff_ffff {
                    return;
                }
                break;
            }
            index = index.wrapping_add(1);
            slot = slot.wrapping_add(4);
            if index >= used {
                break;
            }
        }
    }
    let alloced = e.get(this, NiTPrimitiveSet::m_uiAlloced);
    if used == alloced {
        let new_size = if alloced != 0 {
            alloced.wrapping_add(alloced)
        } else {
            1
        };
        e.call(SET_REALLOC, &args![this, new_size]);
    }
    let used = e.get(this, NiTPrimitiveSet::m_uiUsed);
    let key = e.mem.u32(element.addr());
    let base = e.get(this, NiTPrimitiveSet::m_pBase);
    e.mem.set_u32(base.wrapping_add(used.wrapping_mul(4)), key);
    e.set(this, NiTPrimitiveSet::m_uiUsed, used.wrapping_add(1));
}

// Translated from 00a326c0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiControllerSequence`): allocates 0x74 bytes (`operator new`) and, when that
/// succeeds, runs the empty-sequence constructor `00a316a0` on them and answers its result;
/// null otherwise.
pub fn fn_00a326c0(e: &mut Engine) -> Ptr<NiControllerSequence> {
    let block = e.call(OPERATOR_NEW, &args![0x74u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(SEQUENCE_CONSTRUCT_EMPTY, &args![block]).ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3_26c0, fn_00a326c0() -> Ptr<NiControllerSequence>),
        entry!(
            0x00a307c0,
            ni_controller_sequence_get_object_name_at(Ptr<NiControllerSequence>, u32, u32)
        ),
        entry!(0x00a30880, fn_00a30880(Ptr<NiControllerSequence>)),
        entry!(0x00a30900, fn_00a30900(Ptr<NiControllerSequence>, u32)),
        entry!(
            0x00a30970,
            ni_controller_sequence_compute_scaled_time(Ptr<NiControllerSequence>, f32, bool) -> f32
        ),
        entry!(
            0x00a30ab0,
            fn_00a30ab0(Ptr<NiControllerSequence>, Ptr<NiControllerSequence>) -> bool
        ),
        entry!(
            0x00a30c80,
            fn_00a30c80(Ptr<NiControllerSequence>, Ptr<NiControllerSequence>) -> bool
        ),
        entry!(
            0x00a30cc0,
            fn_00a30cc0(Ptr<InterpArrayItem>, Ptr<InterpArrayItem>) -> bool
        ),
        entry!(0x00a30d40, fn_00a30d40(Ptr<IDTag>, Ptr, u32)),
        entry!(
            0x00a30ea0,
            ni_controller_sequence_register_streamables(Ptr<NiControllerSequence>, Ptr) -> bool
        ),
        entry!(
            0x00a30fb0,
            ni_controller_sequence_is_equal(
                Ptr<NiControllerSequence>,
                Ptr<NiControllerSequence>,
            ) -> bool
        ),
        entry!(0x00a31140, ni_controller_sequence_get_viewer_string(u32, u32) -> u32),
        entry!(0x00a31260, fn_00a31260(Ptr<InterpArrayItem>)),
        entry!(
            0x00a312d0,
            fn_00a312d0(Ptr<IDTag>, Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr<IDTag>
        ),
        entry!(
            0x00a31400,
            fn_00a31400(Ptr<IDTag>, Ptr<IDTag>) -> Ptr<IDTag>
        ),
        entry!(0x00a31500, fn_00a31500(Ptr, u32) -> Ptr),
        entry!(0x00a315b0, fn_00a315b0(Ptr, u32) -> Ptr),
        entry!(0x00a31730, ni_controller_sequence_get_rtti() -> Ptr),
        entry!(0x00a31740, fn_00a31740(Ptr, u32) -> Ptr),
        entry!(0x00a317e0, fn_00a317e0(Ptr<NiControllerSequence>, u32, Ptr)),
        entry!(
            0x00a31880,
            fn_00a31880(Ptr<InterpArrayItem>, Ptr<InterpArrayItem>) -> Ptr<InterpArrayItem>
        ),
        entry!(
            0x00a31910,
            fn_00a31910(Ptr<NiControllerSequence>, Ptr, Ptr, u32) -> bool
        ),
        entry!(
            0x00a32010,
            ni_controller_sequence_find_corresponding_morph_frame(
                Ptr<NiControllerSequence>,
                Ptr<NiControllerSequence>,
                f32,
            ) -> f32
        ),
        entry!(
            0x00a32290,
            fn_00a32290(Ptr<NiControllerSequence>, f32, f32, f32)
        ),
        entry!(0x00a32330, fn_00a32330(Ptr<NiControllerSequence>) -> bool),
        entry!(
            0x00a32470,
            ni_controller_sequence_process_clone(Ptr<NiControllerSequence>, Ptr)
        ),
        entry!(
            0x00a326e0,
            ni_controller_sequence_link_object(Ptr<NiControllerSequence>, Ptr<NiStream>)
        ),
        entry!(
            0x00a328b0,
            ni_controller_sequence_set_phase(Ptr<NiControllerSequence>, f32, u8)
        ),
        entry!(0x00a32940, fn_00a32940(Ptr<NiTArray>, u32, Ptr)),
        entry!(
            0x00a32a10,
            fn_00a32a10(Ptr<NiControllerSequence>, Ptr, u32, u32) -> Ptr<NiControllerSequence>
        ),
        entry!(0x00a32b80, fn_00a32b80(Ptr<NiControllerSequence>, u32)),
        entry!(
            0x00a32bc0,
            fn_00a32bc0(Ptr<NiControllerSequence>, Ptr, Ptr<IDTag>, u8) -> u32
        ),
        entry!(
            0x00a32c70,
            fn_00a32c70(Ptr<NiControllerSequence>, Ptr) -> bool
        ),
        entry!(
            0x00a32fa0,
            fn_00a32fa0(Ptr<NiControllerSequence>, Ptr<NiControllerSequence>, u32, u32, u8)
        ),
        entry!(
            0x00a33130,
            fn_00a33130(Ptr<NiControllerSequence>, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a33570,
            fn_00a33570(Ptr<NiControllerSequence>, Ptr<NiControllerSequence>, Ptr)
        ),
        entry!(0x00a33950, fn_00a33950(Ptr<InterpArrayItem>, Ptr<NiStream>)),
        entry!(
            0x00a33a50,
            ni_controller_sequence_id_tag_load_binary(Ptr<IDTag>, Ptr<NiStream>, Ptr)
        ),
        entry!(
            0x00a33bf0,
            ni_controller_sequence_save_binary(Ptr<NiControllerSequence>, Ptr<NiStream>)
        ),
        entry!(
            0x00a33e40,
            ni_controller_sequence_load_binary(Ptr<NiControllerSequence>, Ptr<NiStream>)
        ),
        entry!(
            0x00a343c0,
            fn_00a343c0(Ptr<NiControllerSequence>, Ptr<NiControllerSequence>, Ptr)
        ),
        entry!(0x00a34450, fn_00a34450(Ptr, u32) -> Ptr),
        entry!(
            0x00a34480,
            fn_00a34480(Ptr<NiControllerSequence>, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a344f0,
            ni_controller_sequence_create_clone(
                Ptr<NiControllerSequence>,
                Ptr,
            ) -> Ptr<NiControllerSequence>
        ),
        entry!(
            0x00a34540,
            ni_controller_sequence_get_viewer_strings(Ptr<NiControllerSequence>, Ptr)
        ),
        entry!(0x00a349e0, fn_00a349e0(Ptr, Ptr)),
        entry!(
            0x00a34ba0,
            ni_controller_sequence_update(Ptr<NiControllerSequence>, f32, u8)
        ),
        entry!(
            0x00a34f20,
            fn_00a34f20(
                Ptr<NiControllerSequence>,
                u32,
                u8,
                f32,
                f32,
                Ptr<NiControllerSequence>,
                u8,
            ) -> bool
        ),
        entry!(
            0x00a35030,
            ni_controller_sequence_deactivate(Ptr<NiControllerSequence>, f32, u8) -> bool
        ),
        entry!(
            0x00a350d0,
            fn_00a350d0(
                Ptr<NiControllerSequence>,
                Ptr<NiControllerSequence>,
                f32,
                f32,
                u32,
                f32,
                f32,
                Ptr<NiControllerSequence>,
            ) -> bool
        ),
        entry!(
            0x00a351d0,
            fn_00a351d0(
                Ptr<NiControllerSequence>,
                Ptr<NiControllerSequence>,
                f32,
                u32,
                f32,
                f32,
            ) -> bool
        ),
        entry!(
            0x00a352f0,
            ni_controller_sequence_post_link_object(Ptr<NiControllerSequence>, Ptr<NiStream>)
        ),
        entry!(
            0x00a35640,
            ni_controller_sequence_destructor(Ptr<NiControllerSequence>)
        ),
        entry!(0x00a35700, fn_00a35700(Ptr<NiStream>, u32, Ptr)),
        entry!(0x00a35820, ni_controller_sequence_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00e7ec00,
            ni_t_primitive_set_add_unique(Ptr<NiTPrimitiveSet>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Vec<Vec<u32>>;

    fn calls_to(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The vtable of the test objects: slot `n` is the double at `target(n)`.
    const OBJECT_VTABLE: u32 = 0x0300_0000;

    fn target(slot: u32) -> u32 {
        0x0300_1000 + slot
    }

    /// An engine with the test vtable (every slot up to 0xe0 points at `target(slot)`), counting
    /// `InterlockedIncrement`/`Decrement` doubles, a silent `DeleteThis` and a call log.
    fn setup() -> Engine {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x38).map(|slot| target(slot * 4)).collect();
        e.put_vtable(OBJECT_VTABLE, &slots);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(target(SLOT_DELETE_THIS), |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    /// Counting doubles that do not touch memory (for tests with fixed-string handles, which are not
    /// addresses in the test heap): the increment answers 2 and the decrement 1.
    fn silent_interlocked(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |_, _| 2u32.into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
    }

    /// The globals for fixed strings: the empty string's handle word (0x9000) and the static
    /// `NiFixedString` at `011f49e0` (handle 0x7800).
    fn map_fixed_strings(e: &mut Engine) {
        e.map(EMPTY_FIXED_STRING, 4);
        e.set_global(EMPTY_FIXED_STRING, 0x9000u32);
        e.map(FIXED_STRING_011F49E0, 4);
        e.set_global(FIXED_STRING_011F49E0, 0x7800u32);
    }

    /// A counted test object (vtable [`OBJECT_VTABLE`], count 1).
    fn object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, OBJECT_VTABLE);
        e.mem.set_u32(object + 4, 1);
        object
    }

    fn count_of(e: &Engine, object: u32) -> u32 {
        e.mem.u32(object + 4)
    }

    /// A sequence with the test vtable.
    fn sequence(e: &mut Engine) -> Ptr<NiControllerSequence> {
        let this = e.new_object::<NiControllerSequence>();
        e.mem.set_u32(this.addr(), OBJECT_VTABLE);
        this
    }

    /// Gives `this` `count` zeroed items and tags (size `count`) and answers their addresses.
    fn set_items(e: &mut Engine, this: Ptr<NiControllerSequence>, count: u32) -> (u32, u32) {
        let items = e.mem.alloc(count * INTERP_ITEM_SIZE + 4);
        let tags = e.mem.alloc(count * ID_TAG_SIZE + 4);
        e.set(this, NiControllerSequence::m_uiArraySize, count);
        e.set(this, NiControllerSequence::m_pkInterpArray, Ptr::new(items));
        e.set(this, NiControllerSequence::m_pkIDTagArray, Ptr::new(tags));
        (items, tags)
    }

    /// Writes `text` (NUL terminated) at `address`, mapping the memory first.
    fn put_text(e: &mut Engine, address: u32, text: &str) {
        e.map(address, text.len() as u32 + 1);
        for (offset, byte) in text.bytes().enumerate() {
            e.mem.set_u8(address + offset as u32, byte);
        }
    }

    /// Writes `text` into a new heap block and answers its address.
    fn string(e: &mut Engine, text: &str) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 1);
        for (offset, byte) in text.bytes().enumerate() {
            e.mem.set_u8(address + offset as u32, byte);
        }
        address
    }

    fn set_float(e: &mut Engine, address: u32, value: f32) {
        e.mem.set_u32(address, value.to_bits());
    }

    fn float_at(e: &Engine, address: u32) -> f32 {
        f32::from_bits(e.mem.u32(address))
    }

    /// A stream whose input and output streams are 0x10-byte objects with the given read and
    /// write function addresses, NIF version and BS stream version.
    fn stream_with(
        e: &mut Engine,
        read: u32,
        write: u32,
        version: u32,
        bs_version: u32,
    ) -> (Ptr<NiStream>, u32, u32) {
        let stream = e.new_object::<NiStream>();
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, read);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, write);
        e.set(stream, NiStream::m_pkIstr, input);
        e.set(stream, NiStream::m_pkOstr, output);
        e.set(stream, NiStream::m_uiNifFileVersion, version);
        e.set(stream, NiStream::m_BSStreamHeader, bs_version);
        e.put_vtable(0x0300_2000, &[0; 13]);
        e.mem.set_u32(stream.addr(), 0x0300_2000);
        e.mem.set_u32(0x0300_2030, target(0x30));
        (stream, input, output)
    }

    /// The `CI_FMOD` double: `fmod` of the two doubles.
    fn fmod_double(e: &mut Engine) {
        e.register(CI_FMOD, |_, a| {
            let dividend = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            let divisor = f64::from_bits(u64::from(a[2]) | u64::from(a[3]) << 32);
            Ret {
                st0: dividend % divisor,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn get_object_name_at_writes_nothing_without_a_destination() {
        let mut e = setup();
        let this = sequence(&mut e);
        set_items(&mut e, this, 1);
        e.call(0x00a307c0, &args![this, 0u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn get_object_name_at_writes_an_empty_string_for_an_empty_item() {
        let mut e = setup();
        let this = sequence(&mut e);
        set_items(&mut e, this, 2);
        let destination = e.mem.alloc(16);
        e.mem.set_u8(destination, b'x');
        e.call(0x00a307c0, &args![this, 1u32, destination]);
        assert_eq!(e.mem.u8(destination), 0);
    }

    #[test]
    fn get_object_name_at_copies_the_name_or_formats_it_with_the_property_type() {
        let mut e = setup();
        put_text(&mut e, TEXT_PROP, "PROP\n");
        e.register(STRING_COPY_SECURE, |_, _| Ret::default());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        let this = sequence(&mut e);
        let (items, tags) = set_items(&mut e, this, 2);
        let name = string(&mut e, "Bip01");
        let property = string(&mut e, "NiAlpha");
        e.mem.set_u32(items, 0x1234);
        e.mem.set_u32(items + INTERP_ITEM_SIZE, 0x1234);
        e.mem.set_u32(tags, name);
        e.mem.set_u32(tags + ID_TAG_SIZE, name);
        e.mem.set_u32(tags + ID_TAG_SIZE + 4, property);
        let destination = e.mem.alloc(64);
        e.call(0x00a307c0, &args![this, 0u32, destination]);
        assert_eq!(
            calls_to(&e, STRING_COPY_SECURE),
            vec![vec![destination, 6, name]]
        );
        e.call(0x00a307c0, &args![this, 1u32, destination]);
        // The size is the name and the property text plus "PROP\n" plus 2.
        assert_eq!(
            calls_to(&e, NI_SPRINTF),
            vec![vec![
                destination,
                5 + 5 + 7 + 2,
                TEXT_FORMAT_PROP,
                name,
                TEXT_PROP,
                property
            ]]
        );
    }

    #[test]
    fn fn_00a30880_deactivates_an_active_sequence_and_drops_the_controllers() {
        let mut e = setup();
        e.register(target(SLOT_DEACTIVATE), |_, _| Ret::default());
        let this = sequence(&mut e);
        let (items, _) = set_items(&mut e, this, 2);
        let controller = object(&mut e);
        e.mem.set_u32(items + 4, controller);
        e.mem.set_u32(items + 8, 0x4444);
        e.mem.set_u32(items + INTERP_ITEM_SIZE + 8, 0x5555);
        e.set(this, NiControllerSequence::m_eState, 2);
        e.call(0x00a30880, &args![this]);
        assert_eq!(
            calls_to(&e, target(SLOT_DEACTIVATE)),
            vec![vec![this.addr(), 0, 0]]
        );
        assert_eq!(count_of(&e, controller), 0);
        assert_eq!(
            calls_to(&e, target(SLOT_DELETE_THIS)),
            vec![vec![controller]]
        );
        assert_eq!(e.mem.u32(items + 4), 0);
        assert_eq!(e.mem.u32(items + 8), 0);
        assert_eq!(e.mem.u32(items + INTERP_ITEM_SIZE + 8), 0);
    }

    #[test]
    fn fn_00a30880_leaves_an_inactive_sequence_alone() {
        let mut e = setup();
        let this = sequence(&mut e);
        set_items(&mut e, this, 1);
        e.call(0x00a30880, &args![this]);
        assert!(calls_to(&e, target(SLOT_DEACTIVATE)).is_empty());
    }

    #[test]
    fn fn_00a30900_adds_every_interpolator_to_its_blend_interpolator() {
        let mut e = setup();
        e.register(target(SLOT_ADD_INTERP_INFO), |_, a| {
            (a[1] - 0x100 + 7).into_ret()
        });
        let this = sequence(&mut e);
        let (items, _) = set_items(&mut e, this, 3);
        let blend = object(&mut e);
        e.mem.set_u32(this.addr() + 0x1c, 0x7f80_0001);
        for (index, priority) in [(0u32, 3u8), (1, 0xff)] {
            let item = items + index * INTERP_ITEM_SIZE;
            e.mem.set_u32(item, 0x100 + index);
            e.mem.set_u32(item + 8, blend);
            e.mem.set_u8(item + 0xd, priority);
        }
        // The third item has an interpolator but no blend interpolator.
        e.mem.set_u32(items + 2 * INTERP_ITEM_SIZE, 0x102);
        e.call(0x00a30900, &args![this, 0x80u32]);
        assert_eq!(
            calls_to(&e, target(SLOT_ADD_INTERP_INFO)),
            vec![
                vec![blend, 0x100, 0x7fc0_0001, 3, 0x3f80_0000],
                vec![blend, 0x101, 0x7fc0_0001, 0xffff_ff80, 0x3f80_0000],
            ]
        );
        assert_eq!(e.mem.u8(items + 0xc), 7);
        assert_eq!(e.mem.u8(items + INTERP_ITEM_SIZE + 0xc), 8);
        assert_eq!(e.mem.u8(items + 2 * INTERP_ITEM_SIZE + 0xc), 0);
    }

    /// A sequence for the scaled time tests: frequency 2.0, begin 0.0, end 10.0.
    fn timed_sequence(e: &mut Engine, cycle_type: u32) -> Ptr<NiControllerSequence> {
        let this = sequence(e);
        e.map(NEGATIVE_FLT_MAX_DOUBLE, 8);
        e.mem.set_u32(
            NEGATIVE_FLT_MAX_DOUBLE,
            (-f64::from(f32::MAX)).to_bits() as u32,
        );
        e.mem.set_u32(
            NEGATIVE_FLT_MAX_DOUBLE + 4,
            ((-f64::from(f32::MAX)).to_bits() >> 32) as u32,
        );
        set_float(e, this.addr() + 0x28, 2.0);
        set_float(e, this.addr() + 0x2c, 0.0);
        set_float(e, this.addr() + 0x30, 10.0);
        set_float(e, this.addr() + 0x34, 1.0);
        set_float(e, this.addr() + 0x38, 3.0);
        e.set(this, NiControllerSequence::m_eCycleType, cycle_type);
        this
    }

    fn scaled(e: &mut Engine, this: Ptr<NiControllerSequence>, time: f32, update: bool) -> f32 {
        e.call(0x00a30970, &args![this, time.to_bits(), u32::from(update)])
            .f32()
    }

    #[test]
    fn compute_scaled_time_scales_the_elapsed_time_and_adds_the_weighted_last_time() {
        let mut e = setup();
        let this = timed_sequence(&mut e, 1);
        // 2.0 * (2.0 - 1.0) + 3.0
        assert_eq!(scaled(&mut e, this, 2.0, false), 5.0);
        assert_eq!(float_at(&e, this.addr() + 0x34), 1.0);
        assert_eq!(scaled(&mut e, this, 2.0, true), 5.0);
        assert_eq!(float_at(&e, this.addr() + 0x38), 5.0);
        assert_eq!(float_at(&e, this.addr() + 0x34), 2.0);
        assert_eq!(float_at(&e, this.addr() + 0x3c), 5.0);
    }

    #[test]
    fn compute_scaled_time_starts_from_zero_when_there_is_no_last_time() {
        let mut e = setup();
        let this = timed_sequence(&mut e, 1);
        set_float(&mut e, this.addr() + 0x34, f32::MIN);
        // The elapsed time is the time itself and the weighted last time counts as 0.
        assert_eq!(scaled(&mut e, this, 3.0, false), 6.0);
    }

    #[test]
    fn compute_scaled_time_clamps_into_begin_and_end_for_the_other_cycle_types() {
        let mut e = setup();
        let this = timed_sequence(&mut e, 1);
        assert_eq!(scaled(&mut e, this, 20.0, false), 10.0);
        assert_eq!(scaled(&mut e, this, -20.0, false), 0.0);
        let this = timed_sequence(&mut e, 2);
        assert_eq!(scaled(&mut e, this, 20.0, false), 10.0);
    }

    #[test]
    fn compute_scaled_time_wraps_a_looping_sequence() {
        let mut e = setup();
        fmod_double(&mut e);
        let this = timed_sequence(&mut e, 0);
        // 2.0 * (8.0 - 1.0) + 3.0 = 17.0, which wraps to 7.0.
        assert_eq!(scaled(&mut e, this, 8.0, false), 7.0);
        // 2.0 * (-4.0 - 1.0) + 3.0 = -7.0: the remainder is -7.0, and begin + range makes it 3.0.
        assert_eq!(scaled(&mut e, this, -4.0, false), 3.0);
        // A scaled time that is exactly the range stays at the end.
        assert_eq!(scaled(&mut e, this, 4.0, false), 9.0);
        assert_eq!(scaled(&mut e, this, 4.5, false), 10.0);
    }

    #[test]
    fn compute_scaled_time_of_a_zero_length_loop_is_the_begin_time() {
        let mut e = setup();
        let this = timed_sequence(&mut e, 0);
        set_float(&mut e, this.addr() + 0x2c, 4.0);
        set_float(&mut e, this.addr() + 0x30, 4.0);
        assert_eq!(scaled(&mut e, this, 8.0, false), 4.0);
    }

    #[test]
    fn compute_scaled_time_passes_a_nan_through_the_clamps() {
        let mut e = setup();
        let this = timed_sequence(&mut e, 1);
        let result = scaled(&mut e, this, f32::NAN, true);
        assert!(result.is_nan());
        assert!(float_at(&e, this.addr() + 0x3c).is_nan());
    }

    /// Gives `this` a text key object holding `texts` (8 bytes per key: the time, then the text).
    fn set_text_keys(e: &mut Engine, this: Ptr<NiControllerSequence>, texts: &[(f32, u32)]) -> u32 {
        let keys = e.mem.alloc(0x20);
        let array = e.mem.alloc(8 * texts.len() as u32 + 8);
        e.mem.set_u32(keys + 0xc, texts.len() as u32);
        e.mem.set_u32(keys + 0x10, array);
        for (index, (time, text)) in texts.iter().enumerate() {
            set_float(e, array + 8 * index as u32, *time);
            e.mem.set_u32(array + 8 * index as u32 + 4, *text);
        }
        e.set(this, NiControllerSequence::m_spTextKeys, Ptr::new(keys));
        keys
    }

    /// The `NiStrnicmp` and `NiStricmp` doubles: a case-insensitive prefix and full comparison.
    fn string_compare_doubles(e: &mut Engine) {
        e.map(MORPH_TAG_POINTER, 8);
        let morph = string(e, "morph:");
        e.set_global(MORPH_TAG_POINTER, morph);
        e.set_global(MORPH_TAG_LENGTH, 6u32);
        fn read(e: &Engine, address: u32) -> String {
            let mut text = String::new();
            let mut offset = 0;
            while e.mem.u8(address + offset) != 0 {
                text.push(char::from(e.mem.u8(address + offset)).to_ascii_lowercase());
                offset += 1;
            }
            text
        }
        e.register(NI_STRNICMP, |e, a| {
            let text = read(e, a[0]);
            let prefix = read(e, a[1]);
            u32::from(!text.starts_with(&prefix[..a[2] as usize])).into_ret()
        });
        e.register(NI_STRICMP, |e, a| {
            u32::from(read(e, a[0]) != read(e, a[1])).into_ret()
        });
    }

    #[test]
    fn fn_00a30ab0_needs_the_same_morph_keys_in_both_sequences() {
        let mut e = setup();
        string_compare_doubles(&mut e);
        let this = sequence(&mut e);
        let other = sequence(&mut e);
        let open = string(&mut e, "morph:open");
        let close = string(&mut e, "morph:close");
        let note = string(&mut e, "sound");
        let open_copy = string(&mut e, "morph:open");
        // Without text keys the answer is false.
        assert!(!e.call(0x00a30ab0, &args![this, other]).bool());
        set_text_keys(&mut e, this, &[(0.0, open), (1.0, note), (2.0, close)]);
        assert!(!e.call(0x00a30ab0, &args![this, other]).bool());
        set_text_keys(
            &mut e,
            other,
            &[(0.0, open_copy), (2.0, close), (3.0, note)],
        );
        assert!(e.call(0x00a30ab0, &args![this, other]).bool());
        // A morph key of the other sequence that this one lacks.
        let wide = string(&mut e, "morph:wide");
        set_text_keys(&mut e, other, &[(0.0, open), (2.0, close), (4.0, wide)]);
        assert!(!e.call(0x00a30ab0, &args![this, other]).bool());
        // A morph key of this sequence that the other lacks.
        set_text_keys(&mut e, other, &[(0.0, open), (2.0, note)]);
        assert!(!e.call(0x00a30ab0, &args![this, other]).bool());
        // No morph keys at all.
        set_text_keys(&mut e, other, &[(0.0, note)]);
        assert!(!e.call(0x00a30ab0, &args![this, other]).bool());
    }

    #[test]
    fn fn_00a30c80_checks_the_partner_chain_before_the_morph_keys() {
        let mut e = setup();
        string_compare_doubles(&mut e);
        let this = sequence(&mut e);
        let other = sequence(&mut e);
        let partner = sequence(&mut e);
        let open = string(&mut e, "morph:open");
        set_text_keys(&mut e, this, &[(0.0, open)]);
        set_text_keys(&mut e, other, &[(0.0, open)]);
        let accept = Rc::new(RefCell::new(true));
        let answer = accept.clone();
        e.register_double(SEQUENCE_PARTNER_CHAIN_CHECK, move |_, _| {
            u32::from(*answer.borrow()).into_ret()
        });
        assert!(!e.call(0x00a30c80, &args![this, 0u32]).bool());
        assert!(e.call(0x00a30c80, &args![this, other]).bool());
        e.set(
            other,
            NiControllerSequence::m_pkPartnerSequence,
            Ptr::new(partner.addr()),
        );
        assert!(e.call(0x00a30c80, &args![this, other]).bool());
        assert_eq!(
            calls_to(&e, SEQUENCE_PARTNER_CHAIN_CHECK),
            vec![vec![this.addr(), partner.addr()]]
        );
        *accept.borrow_mut() = false;
        assert!(!e.call(0x00a30c80, &args![this, other]).bool());
        // A partner that is this sequence is refused without asking.
        e.set(
            other,
            NiControllerSequence::m_pkPartnerSequence,
            Ptr::new(this.addr()),
        );
        assert!(!e.call(0x00a30c80, &args![this, other]).bool());
        assert_eq!(calls_to(&e, SEQUENCE_PARTNER_CHAIN_CHECK).len(), 2);
    }

    #[test]
    fn fn_00a30cc0_compares_two_items() {
        let mut e = setup();
        let first = e.mem.alloc(INTERP_ITEM_SIZE);
        let second = e.mem.alloc(INTERP_ITEM_SIZE);
        let equal = Rc::new(RefCell::new(true));
        let answer = equal.clone();
        e.register_double(target(SLOT_IS_EQUAL), move |_, _| {
            u32::from(*answer.borrow()).into_ret()
        });
        assert!(e.call(0x00a30cc0, &args![first, second]).bool());
        let interpolator = object(&mut e);
        let other_interpolator = object(&mut e);
        e.mem.set_u32(first, interpolator);
        assert!(!e.call(0x00a30cc0, &args![first, second]).bool());
        e.mem.set_u32(second, other_interpolator);
        assert!(e.call(0x00a30cc0, &args![first, second]).bool());
        assert_eq!(
            calls_to(&e, target(SLOT_IS_EQUAL)),
            vec![vec![interpolator, other_interpolator]; 1]
        );
        // The controllers go through the same test, the priorities are compared last.
        let controller = object(&mut e);
        let other_controller = object(&mut e);
        e.mem.set_u32(first + 4, controller);
        e.mem.set_u32(second + 4, other_controller);
        assert!(e.call(0x00a30cc0, &args![first, second]).bool());
        e.mem.set_u8(first + 0xd, 4);
        assert!(!e.call(0x00a30cc0, &args![first, second]).bool());
        e.mem.set_u8(first + 0xd, 0);
        *equal.borrow_mut() = false;
        assert!(!e.call(0x00a30cc0, &args![first, second]).bool());
        e.mem.set_u32(second, 0);
        e.mem.set_u32(first, 0);
        e.mem.set_u32(second + 4, 0);
        assert!(!e.call(0x00a30cc0, &args![first, second]).bool());
    }

    #[test]
    fn fn_00a30d40_turns_palette_offsets_into_fixed_strings() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        silent_interlocked(&mut e);
        e.register(FIXED_STRING_FROM_CHARS, |_, a| (a[0] + 0x10000).into_ret());
        let tag = e.mem.alloc(ID_TAG_SIZE);
        let palette = e.mem.alloc(16);
        e.mem.set_u32(palette + 8, 0x5000);
        for (word, value) in [4u32, 0xffff_ffff, 10, 0xffff_ffff, 0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(tag + 4 * word as u32, value);
        }
        e.call(0x00a30d40, &args![tag, palette, 0u32]);
        let handles: Vec<u32> = (0..5).map(|word| e.mem.u32(tag + 4 * word)).collect();
        assert_eq!(handles, vec![0x15004, 0x9000, 0x1500a, 0x9000, 0x15000]);
        // The three made strings are released once each when they replace the empty string's
        // handle, the empty handle the function kept is not released.
        assert_eq!(
            calls_to(&e, FIXED_STRING_FROM_CHARS),
            vec![vec![0x5004], vec![0x500a], vec![0x5000]]
        );
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn register_streamables_registers_the_objects_and_strings() {
        let mut e = setup();
        let mut stream_slots = [0u32; 13];
        stream_slots[8] = target(0x20);
        e.put_vtable(0x0300_2000, &stream_slots);
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream, 0x0300_2000);
        e.register(target(0x20), |_, _| Ret::default());
        e.register(target(SLOT_REGISTER_STREAMABLES), |_, _| Ret::default());
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        let this = sequence(&mut e);
        let (items, tags) = set_items(&mut e, this, 2);
        let interpolator = object(&mut e);
        let controller = object(&mut e);
        e.mem.set_u32(items, interpolator);
        e.mem.set_u32(items + 4, controller);
        let keys = object(&mut e);
        e.set(this, NiControllerSequence::m_spTextKeys, Ptr::new(keys));
        let notes = e.mem.alloc(8);
        let first_note = object(&mut e);
        let second_note = object(&mut e);
        e.mem.set_u32(notes, first_note);
        e.mem.set_u32(notes + 4, second_note);
        e.set(this, NiControllerSequence::spAnimNotesA, Ptr::new(notes));
        e.set(this, NiControllerSequence::usNumNotes, 2);
        assert!(e.call(0x00a30ea0, &args![this, stream]).bool());
        assert_eq!(
            calls_to(&e, target(SLOT_REGISTER_STREAMABLES)),
            vec![
                vec![interpolator, stream],
                vec![controller, stream],
                vec![keys, stream],
                vec![first_note, stream],
                vec![second_note, stream]
            ]
        );
        let mut strings: Calls = Vec::new();
        for tag in [tags, tags + ID_TAG_SIZE] {
            for word in 0..5 {
                strings.push(vec![stream, tag + 4 * word]);
            }
        }
        strings.push(vec![stream, this.addr() + 8]);
        strings.push(vec![stream, this.addr() + 0x5c]);
        assert_eq!(calls_to(&e, target(0x20)), strings);
    }

    #[test]
    fn register_streamables_stops_when_the_base_class_refuses() {
        let mut e = setup();
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| Ret::default());
        let this = sequence(&mut e);
        set_items(&mut e, this, 1);
        assert!(!e.call(0x00a30ea0, &args![this, 0x1000u32]).bool());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    /// Two sequences with the same content (one item with no interpolator, one tag with five
    /// equal handles) and the doubles `IsEqual` needs.
    fn equal_pair(e: &mut Engine) -> (Ptr<NiControllerSequence>, Ptr<NiControllerSequence>) {
        string_compare_doubles(e);
        e.register(OBJECT_IS_EQUAL, |_, _| 1u32.into_ret());
        let pair = (sequence(e), sequence(e));
        for this in [pair.0, pair.1] {
            set_items(e, this, 1);
            e.set(this, NiControllerSequence::m_uiArrayGrowBy, 4);
            e.mem.set_u32(this.addr() + 0x08, 0x6000);
            e.mem.set_u32(this.addr() + 0x5c, 0x6100);
            set_float(e, this.addr() + 0x1c, 1.0);
            set_float(e, this.addr() + 0x28, 2.0);
            set_float(e, this.addr() + 0x2c, 3.0);
            set_float(e, this.addr() + 0x30, 4.0);
            let tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
            for word in 0..5 {
                e.mem.set_u32(tags + 4 * word, 0x6200 + word);
            }
        }
        pair
    }

    #[test]
    fn is_equal_accepts_two_identical_sequences() {
        let mut e = setup();
        let (this, other) = equal_pair(&mut e);
        assert!(e.call(0x00a30fb0, &args![this, other]).bool());
        // Names with different handles are compared as text.
        let first = string(&mut e, "Walk");
        let second = string(&mut e, "wALK");
        e.mem.set_u32(this.addr() + 8, first);
        e.mem.set_u32(other.addr() + 8, second);
        assert!(e.call(0x00a30fb0, &args![this, other]).bool());
        assert_eq!(calls_to(&e, NI_STRICMP), vec![vec![first, second]]);
    }

    #[test]
    fn is_equal_rejects_every_difference() {
        type Change = fn(&mut Engine, Ptr<NiControllerSequence>, Ptr<NiControllerSequence>);
        let changes: Vec<Change> = vec![
            |e, this, other| {
                let name = string(e, "Walk");
                let other_name = string(e, "Run");
                e.mem.set_u32(this.addr() + 8, name);
                e.mem.set_u32(other.addr() + 8, other_name);
            },
            |e, _, other| e.mem.set_u32(other.addr() + 0xc, 2),
            |e, _, other| e.mem.set_u32(other.addr() + 0x10, 9),
            |e, _, other| {
                let items = e.get(other, NiControllerSequence::m_pkInterpArray).addr();
                e.mem.set_u8(items + 0xd, 1);
            },
            |e, _, other| {
                let tags = e.get(other, NiControllerSequence::m_pkIDTagArray).addr();
                e.mem.set_u32(tags + 8, 1);
            },
            |e, _, other| set_float(e, other.addr() + 0x1c, f32::NAN),
            |e, _, other| {
                let keys = object(e);
                e.mem.set_u32(other.addr() + 0x20, keys);
            },
            |e, _, other| e.mem.set_u32(other.addr() + 0x24, 1),
            |e, _, other| set_float(e, other.addr() + 0x28, 5.0),
            |e, _, other| set_float(e, other.addr() + 0x2c, 5.0),
            |e, _, other| set_float(e, other.addr() + 0x30, 5.0),
            |e, _, other| {
                let owner = object(e);
                e.mem.set_u32(other.addr() + 0x40, owner);
            },
            |e, _, other| e.mem.set_u32(other.addr() + 0x5c, 0x6101),
        ];
        for (index, change) in changes.iter().enumerate() {
            let mut e = setup();
            let (this, other) = equal_pair(&mut e);
            e.register(target(SLOT_IS_EQUAL), |_, _| 1u32.into_ret());
            assert!(
                e.call(0x00a30fb0, &args![this, other]).bool(),
                "the pair {index} is equal before the change"
            );
            change(&mut e, this, other);
            assert!(
                !e.call(0x00a30fb0, &args![this, other]).bool(),
                "change {index} not noticed"
            );
        }
    }

    #[test]
    fn is_equal_compares_text_keys_and_owners_with_their_own_is_equal() {
        let mut e = setup();
        let (this, other) = equal_pair(&mut e);
        let answer = Rc::new(RefCell::new(true));
        let seen = answer.clone();
        e.register_double(target(SLOT_IS_EQUAL), move |_, _| {
            u32::from(*seen.borrow()).into_ret()
        });
        let keys = object(&mut e);
        let other_keys = object(&mut e);
        let owner = object(&mut e);
        let other_owner = object(&mut e);
        e.mem.set_u32(this.addr() + 0x20, keys);
        e.mem.set_u32(other.addr() + 0x20, other_keys);
        e.mem.set_u32(this.addr() + 0x40, owner);
        e.mem.set_u32(other.addr() + 0x40, other_owner);
        assert!(e.call(0x00a30fb0, &args![this, other]).bool());
        assert_eq!(
            calls_to(&e, target(SLOT_IS_EQUAL)),
            vec![vec![keys, other_keys], vec![owner, other_owner]]
        );
        *answer.borrow_mut() = false;
        assert!(!e.call(0x00a30fb0, &args![this, other]).bool());
        // An owner that is missing from the first sequence only.
        *answer.borrow_mut() = true;
        e.mem.set_u32(this.addr() + 0x40, 0);
        assert!(!e.call(0x00a30fb0, &args![this, other]).bool());
        // Text keys that are missing from the first sequence only.
        e.mem.set_u32(this.addr() + 0x40, owner);
        e.mem.set_u32(this.addr() + 0x20, 0);
        assert!(!e.call(0x00a30fb0, &args![this, other]).bool());
    }

    #[test]
    fn get_viewer_string_prints_the_state_name() {
        let mut e = setup();
        e.register(MEMORY_ALLOCATE, |e, _| e.mem.alloc(0x40).into_ret());
        e.register(SPRINTF, |_, _| Ret::default());
        let name = string(&mut e, "Walk");
        let buffer = e.call(0x00a31140, &args![name, 2u32]).u32();
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 4 + 0x20]]
        );
        assert_eq!(
            calls_to(&e, SPRINTF),
            vec![vec![buffer, TEXT_FORMAT_STATE, name, STATE_NAMES[2]]]
        );
        // A state past the last name prints a null pointer.
        e.call(0x00a31140, &args![name, 7u32]);
        assert_eq!(calls_to(&e, SPRINTF)[1][3], 0);
    }

    #[test]
    fn fn_00a31260_resets_an_item() {
        let mut e = setup();
        e.map(INVALID_BLEND_INDEX, 1);
        e.mem.set_u8(INVALID_BLEND_INDEX, 0xfe);
        let item = e.mem.alloc(INTERP_ITEM_SIZE);
        let interpolator = object(&mut e);
        let controller = object(&mut e);
        e.mem.set_u32(item, interpolator);
        e.mem.set_u32(item + 4, controller);
        e.mem.set_u32(item + 8, 0x4444);
        e.mem.set_u8(item + 0xc, 3);
        e.mem.set_u8(item + 0xd, 1);
        e.call(0x00a31260, &args![item]);
        assert_eq!(e.mem.u32(item), 0);
        assert_eq!(e.mem.u32(item + 4), 0);
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(e.mem.u8(item + 0xc), 0xfe);
        assert_eq!(e.mem.u8(item + 0xd), 0xff);
        assert_eq!(count_of(&e, interpolator), 0);
        assert_eq!(count_of(&e, controller), 0);
        assert_eq!(
            calls_to(&e, target(SLOT_DELETE_THIS)),
            vec![vec![interpolator], vec![controller]]
        );
        // An item without objects: nothing is released.
        e.call(0x00a31260, &args![item]);
        assert_eq!(calls_to(&e, target(SLOT_DELETE_THIS)).len(), 2);
    }

    #[test]
    fn fn_00a312d0_builds_a_tag_from_five_handles() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        silent_interlocked(&mut e);
        e.register(ID_TAG_RESET, |_, _| Ret::default());
        let tag = e.mem.alloc(ID_TAG_SIZE);
        let sources: Vec<u32> = (0..5)
            .map(|word| {
                let source = e.mem.alloc(4);
                e.mem
                    .set_u32(source, if word == 3 { 0x9000 } else { 0x7100 + word });
                source
            })
            .collect();
        let result = e
            .call(
                0x00a312d0,
                &args![tag, sources[0], sources[1], sources[2], sources[3], sources[4]],
            )
            .u32();
        assert_eq!(result, tag);
        let handles: Vec<u32> = (0..5).map(|word| e.mem.u32(tag + 4 * word)).collect();
        assert_eq!(handles, vec![0x7100, 0x7101, 0x7102, 0x9000, 0x7104]);
        assert_eq!(calls_to(&e, ID_TAG_RESET), vec![vec![tag]]);
        // The new handles are counted (the empty string's is not); the empty handles they replace
        // are not released.
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT),
            vec![
                vec![0x7100 - 8],
                vec![0x7101 - 8],
                vec![0x7102 - 8],
                vec![0x7104 - 8]
            ]
        );
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn fn_00a31400_assigns_the_five_handles() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        silent_interlocked(&mut e);
        let tag = e.mem.alloc(ID_TAG_SIZE);
        let other = e.mem.alloc(ID_TAG_SIZE);
        for word in 0..5 {
            e.mem.set_u32(tag + 4 * word, 0x7200 + word);
            e.mem.set_u32(other + 4 * word, 0x7300 + word);
        }
        // The same handle in one word and the empty string in another.
        e.mem.set_u32(other + 4, 0x7201);
        e.mem.set_u32(other + 8, 0x9000);
        assert_eq!(e.call(0x00a31400, &args![tag, other]).u32(), tag);
        let handles: Vec<u32> = (0..5).map(|word| e.mem.u32(tag + 4 * word)).collect();
        assert_eq!(handles, vec![0x7300, 0x7201, 0x9000, 0x7303, 0x7304]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT),
            vec![vec![0x7300 - 8], vec![0x7303 - 8], vec![0x7304 - 8]]
        );
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![
                vec![0x7200 - 8],
                vec![0x7202 - 8],
                vec![0x7203 - 8],
                vec![0x7204 - 8]
            ]
        );
    }

    /// An array block of `count` elements of `size` bytes behind a count word; answers the address
    /// of the first element.
    fn element_block(e: &mut Engine, count: u32, size: u32) -> u32 {
        let block = e.mem.alloc(4 + count * size);
        e.mem.set_u32(block, count);
        block + 4
    }

    #[test]
    fn item_array_destructor_destroys_one_item_or_releases_the_whole_array() {
        let mut e = setup();
        e.register(INTERP_ITEM_DESTRUCT, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let single = e.mem.alloc(INTERP_ITEM_SIZE);
        assert_eq!(e.call(0x00a31500, &args![single, 1u32]).u32(), single);
        assert_eq!(calls_to(&e, INTERP_ITEM_DESTRUCT), vec![vec![single]]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, single]]
        );
        assert_eq!(e.call(0x00a31500, &args![single, 0u32]).u32(), single);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE).len(), 1);
        // The whole array: from the last item to the first, controller then interpolator.
        let array = element_block(&mut e, 2, INTERP_ITEM_SIZE);
        let objects: Vec<u32> = (0..4).map(|_| object(&mut e)).collect();
        e.mem.set_u32(array, objects[0]);
        e.mem.set_u32(array + 4, objects[1]);
        e.mem.set_u32(array + INTERP_ITEM_SIZE, objects[2]);
        e.mem.set_u32(array + INTERP_ITEM_SIZE + 4, objects[3]);
        assert_eq!(e.call(0x00a31500, &args![array, 3u32]).u32(), array - 4);
        assert_eq!(
            calls_to(&e, target(SLOT_DELETE_THIS)),
            vec![
                vec![objects[3]],
                vec![objects[2]],
                vec![objects[1]],
                vec![objects[0]]
            ]
        );
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE)[1],
            vec![MEMORY_MANAGER, array - 4]
        );
        // Bit 1 alone releases without freeing.
        let array = element_block(&mut e, 0, INTERP_ITEM_SIZE);
        assert_eq!(e.call(0x00a31500, &args![array, 2u32]).u32(), array - 4);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE).len(), 2);
    }

    #[test]
    fn tag_array_destructor_releases_the_handles_of_every_tag_from_the_last() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        silent_interlocked(&mut e);
        e.register(ID_TAG_DESTRUCT, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let single = e.mem.alloc(ID_TAG_SIZE);
        assert_eq!(e.call(0x00a315b0, &args![single, 1u32]).u32(), single);
        assert_eq!(calls_to(&e, ID_TAG_DESTRUCT), vec![vec![single]]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, single]]
        );
        let array = element_block(&mut e, 2, ID_TAG_SIZE);
        for word in 0..10 {
            e.mem.set_u32(
                array + 4 * word,
                if word == 6 { 0x9000 } else { 0x7400 + word },
            );
        }
        assert_eq!(e.call(0x00a315b0, &args![array, 3u32]).u32(), array - 4);
        let released: Vec<u32> = calls_to(&e, INTERLOCKED_DECREMENT)
            .iter()
            .map(|call| call[0] + 8 - 0x7400)
            .collect();
        // Tag 1 (words 5 to 9) from its last word, the empty word 6 skipped, then tag 0.
        assert_eq!(released, vec![9, 8, 7, 5, 4, 3, 2, 1, 0]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE)[1],
            vec![MEMORY_MANAGER, array - 4]
        );
    }

    #[test]
    fn pointer_array_destructor_releases_the_pointers() {
        let mut e = setup();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let single = e.mem.alloc(4);
        let held = object(&mut e);
        e.mem.set_u32(single, held);
        assert_eq!(e.call(0x00a31740, &args![single, 1u32]).u32(), single);
        assert_eq!(count_of(&e, held), 0);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, single]]
        );
        let array = element_block(&mut e, 3, 4);
        let objects: Vec<u32> = (0..2).map(|_| object(&mut e)).collect();
        e.mem.set_u32(array, objects[0]);
        e.mem.set_u32(array + 8, objects[1]);
        assert_eq!(e.call(0x00a31740, &args![array, 3u32]).u32(), array - 4);
        assert_eq!(
            calls_to(&e, target(SLOT_DELETE_THIS)),
            vec![vec![held], vec![objects[1]], vec![objects[0]]]
        );
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE)[1],
            vec![MEMORY_MANAGER, array - 4]
        );
    }

    #[test]
    fn fn_00a317e0_moves_an_interpolator_out_of_the_sequence() {
        let mut e = setup();
        e.map(INVALID_BLEND_INDEX, 1);
        e.register(ID_TAG_RESET, |_, _| Ret::default());
        let this = sequence(&mut e);
        let (items, tags) = set_items(&mut e, this, 2);
        let interpolator = object(&mut e);
        e.mem.set_u32(items + INTERP_ITEM_SIZE, interpolator);
        let destination = e.mem.alloc(4);
        e.call(0x00a317e0, &args![this, 1u32, destination]);
        assert_eq!(e.mem.u32(destination), interpolator);
        assert_eq!(e.mem.u32(items + INTERP_ITEM_SIZE), 0);
        assert_eq!(count_of(&e, interpolator), 1);
        assert_eq!(calls_to(&e, ID_TAG_RESET), vec![vec![tags + ID_TAG_SIZE]]);
        // An empty item leaves the destination empty.
        e.call(0x00a317e0, &args![this, 0u32, destination]);
        assert_eq!(e.mem.u32(destination), 0);
        assert_eq!(count_of(&e, interpolator), 0);
    }

    #[test]
    fn fn_00a31880_assigns_an_item() {
        let mut e = setup();
        let item = e.mem.alloc(INTERP_ITEM_SIZE);
        let other = e.mem.alloc(INTERP_ITEM_SIZE);
        let old = object(&mut e);
        let interpolator = object(&mut e);
        let controller = object(&mut e);
        e.mem.set_u32(item, old);
        e.mem.set_u32(other, interpolator);
        e.mem.set_u32(other + 4, controller);
        e.mem.set_u32(other + 8, 0x4444);
        e.mem.set_u8(other + 0xc, 5);
        e.mem.set_u8(other + 0xd, 6);
        assert_eq!(e.call(0x00a31880, &args![item, other]).u32(), item);
        assert_eq!(e.mem.u32(item), interpolator);
        assert_eq!(e.mem.u32(item + 4), controller);
        assert_eq!(e.mem.u32(item + 8), 0x4444);
        assert_eq!(e.mem.u8(item + 0xc), 5);
        assert_eq!(e.mem.u8(item + 0xd), 6);
        assert_eq!(count_of(&e, old), 0);
        assert_eq!(count_of(&e, interpolator), 2);
        assert_eq!(count_of(&e, controller), 2);
        // Assigning the same objects again changes no count.
        e.call(0x00a31880, &args![item, other]);
        assert_eq!(count_of(&e, interpolator), 2);
    }

    /// Gives the function-local statics of the unit a constructed state (guards set): the name
    /// of `BSTreadTransfController` (handle 0x7777), the one at `0101e460` (handle 0x7778) and the
    /// null pointer (0), and the invalid index word.
    fn map_statics(e: &mut Engine) {
        e.map(TREAD_NAME_STATIC, 0x18);
        e.mem.set_u32(TREAD_NAME_GUARD, 1);
        e.mem.set_u32(TREAD_NAME_STATIC, 0x7777);
        e.mem.set_u32(BIP01_NAME_GUARD, 1);
        e.mem.set_u32(BIP01_NAME_STATIC, 0x7778);
        e.mem.set_u32(NULL_POINTER_GUARD, 1);
        e.mem.set_u32(NULL_POINTER_STATIC, 0);
        e.map(INVALID_INDEX, 2);
        e.mem.set_u16(INVALID_INDEX, 0xffff);
    }

    /// Writes the object vtable at `address`.
    fn put_object_vtable(e: &mut Engine, address: u32) {
        let slots: Vec<u32> = (0..0x38).map(|slot| target(slot * 4)).collect();
        e.put_vtable(address, &slots);
    }

    /// The doubles of `00a31910` that are not the unit's own: `GetController` (answering what is in
    /// `controllers` for the two class records), the allocator and the controller constructors.
    fn controller_doubles(e: &mut Engine, transform: u32, tread: u32) {
        e.register_double(OBJECT_NET_GET_CONTROLLER, move |_, a| {
            match a[1] {
                NI_MULTI_TARGET_TRANSFORM_CONTROLLER_RTTI => transform,
                BS_MULTI_TARGET_TREAD_TRANSF_CONTROLLER_RTTI => tread,
                _ => 0,
            }
            .into_ret()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(INTERP_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        e.register(MULTI_TARGET_SET_COUNT, |e, a| {
            let interpolators = e.mem.alloc(a[1] * 0x30);
            let targets = e.mem.alloc(a[1] * 4);
            e.mem.set_u32(a[0] + 0x34, interpolators);
            e.mem.set_u32(a[0] + 0x38, targets);
            e.mem.set_u16(a[0] + 0x3c, a[1] as u16);
            Ret::default()
        });
        e.register(target(SLOT_SET_TARGET), |_, _| Ret::default());
        e.register(OBJECT_NET_REMOVE_CONTROLLER, |_, _| Ret::default());
        e.register(CONTROLLER_SET_NEXT, |_, _| Ret::default());
        put_object_vtable(e, NI_MULTI_TARGET_TRANSFORM_CONTROLLER_VTABLE);
    }

    /// A palette object (the test vtable): `GetRTTI` answers `rtti`, `GetAVObject` answers `found`.
    fn palette(e: &mut Engine, rtti: u32, found: u32) -> u32 {
        e.register_double(target(SLOT_GET_RTTI), move |_, _| rtti.into_ret());
        e.register_double(target(SLOT_GET_AV_OBJECT), move |_, _| found.into_ret());
        object(e)
    }

    #[test]
    fn fn_00a31910_makes_a_transform_controller_and_links_the_item() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        map_statics(&mut e);
        controller_doubles(&mut e, 0, 0);
        e.register(PALETTE_ITEM_COUNT, |_, _| 1u32.into_ret());
        let scene_object = e.mem.alloc(0x80);
        e.mem.set_u32(scene_object + 8, 0x5555);
        let palette = palette(&mut e, NI_DEFAULT_AV_OBJECT_PALETTE_RTTI, scene_object);
        let target_object = e.mem.alloc(0x80);
        let owner = e.mem.alloc(0x80);
        e.mem.set_u32(owner + 0x2c, target_object);
        let this = sequence(&mut e);
        e.mem.set_u32(this.addr() + 0x40, owner);
        let (items, tags) = set_items(&mut e, this, 1);
        let transform_name = string(&mut e, "NiTransformController");
        e.mem.set_u32(tags + 8, transform_name);
        assert!(e
            .call(0x00a31910, &args![this, target_object, palette, 0u32])
            .bool());
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x40]]);
        let controller = calls_to(&e, MULTI_TARGET_SET_COUNT)[0][0];
        assert_eq!(
            calls_to(&e, MULTI_TARGET_SET_COUNT),
            vec![vec![controller, 1]]
        );
        assert_eq!(
            e.mem.u32(controller),
            NI_MULTI_TARGET_TRANSFORM_CONTROLLER_VTABLE
        );
        assert_eq!(
            calls_to(&e, target(SLOT_SET_TARGET)),
            vec![vec![controller, target_object]]
        );
        assert_eq!(e.mem.u16(controller + 8), 0x20);
        // The target is the owner's: the controller is put into the owner's chain.
        assert_eq!(
            calls_to(&e, OBJECT_NET_REMOVE_CONTROLLER),
            vec![vec![target_object, controller], vec![scene_object, 0]]
        );
        assert_eq!(e.mem.u32(owner + 0x30), controller);
        // The item has the controller and the first blend interpolator, flagged.
        assert_eq!(e.mem.u32(items + 4), controller);
        let interpolators = e.mem.u32(controller + 0x34);
        assert_eq!(e.mem.u32(items + 8), interpolators);
        assert_eq!(e.mem.u8(interpolators + 0xc), 1);
        assert_eq!(e.mem.u32(e.mem.u32(controller + 0x38)), scene_object);
        assert_eq!(e.mem.u32(scene_object + 0x30), 0x80);
        // One count from the item, one from the owner.
        assert_eq!(count_of(&e, controller), 2);
    }

    #[test]
    fn fn_00a31910_does_nothing_when_no_tag_needs_a_controller() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        map_statics(&mut e);
        controller_doubles(&mut e, 0, 0);
        let this = sequence(&mut e);
        let (_, tags) = set_items(&mut e, this, 1);
        e.mem.set_u32(tags + 8, 0x9000);
        let owner = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 0x40, owner);
        let target_object = e.mem.alloc(0x80);
        // Without a palette the transform tags are counted from the tags: none.
        assert!(!e
            .call(0x00a31910, &args![this, target_object, 0u32, 0u32])
            .bool());
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn fn_00a31910_counts_transform_tags_when_the_palette_is_not_the_default_one() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        map_statics(&mut e);
        controller_doubles(&mut e, 0, 0);
        let end_of_chain = e.mem.alloc(8);
        let other_rtti = e.mem.alloc(8);
        e.mem.set_u32(other_rtti + 4, end_of_chain);
        let palette = palette(&mut e, other_rtti, 0);
        let this = sequence(&mut e);
        let (_, tags) = set_items(&mut e, this, 2);
        let transform_name = string(&mut e, "NiTransformController");
        e.mem.set_u32(tags + 8, transform_name);
        e.mem.set_u32(tags + ID_TAG_SIZE + 8, transform_name);
        let owner = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 0x40, owner);
        let target_object = e.mem.alloc(0x80);
        e.call(0x00a31910, &args![this, target_object, palette, 0u32]);
        let controller = calls_to(&e, MULTI_TARGET_SET_COUNT)[0][0];
        assert_eq!(
            calls_to(&e, MULTI_TARGET_SET_COUNT),
            vec![vec![controller, 2]]
        );
        assert!(calls_to(&e, PALETTE_ITEM_COUNT).is_empty());
    }

    #[test]
    fn fn_00a31910_links_an_existing_controller_and_the_tread_records() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        map_statics(&mut e);
        let transform = object(&mut e);
        let tread = object(&mut e);
        controller_doubles(&mut e, transform, tread);
        let scene_object = e.mem.alloc(0x80);
        e.mem.set_u32(scene_object + 8, 0x5555);
        let palette = palette(&mut e, NI_DEFAULT_AV_OBJECT_PALETTE_RTTI, scene_object);
        let this = sequence(&mut e);
        let (items, tags) = set_items(&mut e, this, 2);
        let owner = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 0x40, owner);
        // Item 0: a transform tag whose object is already in the controller's table at slot 1.
        let transform_name = string(&mut e, "NiTransformController");
        e.mem.set_u32(tags + 8, transform_name);
        e.mem.set_u16(transform + 0x3c, 2);
        let interpolators = e.mem.alloc(0x60);
        let table = e.mem.alloc(8);
        e.mem.set_u32(transform + 0x34, interpolators);
        e.mem.set_u32(transform + 0x38, table);
        e.mem.set_u32(table + 4, scene_object);
        // Item 1: a tread tag whose interpolator has two records.
        let tread_name = string(&mut e, "BSTreadTransfController");
        e.mem.set_u32(TREAD_NAME_STATIC, tread_name);
        e.mem.set_u32(tags + ID_TAG_SIZE + 8, tread_name);
        let interpolator = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x88);
        e.mem.set_u32(interpolator + 0xc, records);
        e.mem.set_u32(interpolator + 0x10, 2);
        e.mem.set_u32(items + INTERP_ITEM_SIZE, interpolator);
        let blend = e.mem.alloc(0x20);
        e.register(TREAD_CONTROLLER_FIND_INDEX, |_, _| 0xffffu32.into_ret());
        e.register(TREAD_CONTROLLER_ADD, |_, _| Ret::default());
        e.register(TREAD_CONTROLLER_SET, |_, _| Ret::default());
        e.register_double(TREAD_CONTROLLER_GET_INTERPOLATOR, move |_, _| {
            blend.into_ret()
        });
        let target_object = e.mem.alloc(0x80);
        assert!(!e
            .call(0x00a31910, &args![this, target_object, palette, 0u32])
            .bool());
        // Item 0 took the controller and the interpolator of slot 1 (0x30 bytes each).
        assert_eq!(e.mem.u32(items + 4), transform);
        assert_eq!(e.mem.u32(items + 8), interpolators + 0x30);
        assert_eq!(e.mem.u8(interpolators + 0x30 + 0xc), 1);
        assert_eq!(e.mem.u32(scene_object + 0x30), 0x80);
        // The object's name is not "Bip01" (the handle at 011f36cc): its transform controller was removed.
        assert_eq!(
            calls_to(&e, OBJECT_NET_REMOVE_CONTROLLER),
            vec![vec![scene_object, 0]]
        );
        // Item 1 took the tread controller and its first interpolator; both records were added.
        assert_eq!(e.mem.u32(items + INTERP_ITEM_SIZE + 4), tread);
        assert_eq!(e.mem.u32(items + INTERP_ITEM_SIZE + 8), blend);
        assert_eq!(e.mem.u8(blend + 0xc), 1);
        assert_eq!(
            calls_to(&e, TREAD_CONTROLLER_ADD),
            vec![
                vec![tread, scene_object, 0, records],
                vec![tread, scene_object, 0, records + 0x44]
            ]
        );
        assert_eq!(
            calls_to(&e, TREAD_CONTROLLER_GET_INTERPOLATOR),
            vec![vec![tread, 0]]
        );
        assert_eq!(
            calls_to(&e, target(SLOT_GET_AV_OBJECT)),
            vec![
                vec![palette, tags],
                vec![palette, records + 0x40],
                vec![palette, records + 0x44 + 0x40]
            ]
        );
    }

    #[test]
    fn fn_00a31910_sets_a_tread_record_that_the_controller_already_has() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        map_statics(&mut e);
        let transform = object(&mut e);
        let tread = object(&mut e);
        controller_doubles(&mut e, transform, tread);
        let scene_object = e.mem.alloc(0x80);
        let palette = palette(&mut e, NI_DEFAULT_AV_OBJECT_PALETTE_RTTI, scene_object);
        let this = sequence(&mut e);
        let (items, tags) = set_items(&mut e, this, 1);
        let owner = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 0x40, owner);
        let tread_name = string(&mut e, "BSTreadTransfController");
        e.mem.set_u32(TREAD_NAME_STATIC, tread_name);
        e.mem.set_u32(tags + 8, tread_name);
        let interpolator = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x44);
        e.mem.set_u32(interpolator + 0xc, records);
        e.mem.set_u32(interpolator + 0x10, 1);
        e.mem.set_u32(items, interpolator);
        let blend = e.mem.alloc(0x20);
        e.register(TREAD_CONTROLLER_FIND_INDEX, |_, _| 4u32.into_ret());
        e.register(TREAD_CONTROLLER_SET, |_, _| Ret::default());
        e.register_double(TREAD_CONTROLLER_GET_INTERPOLATOR, move |_, _| {
            blend.into_ret()
        });
        let target_object = e.mem.alloc(0x80);
        e.call(0x00a31910, &args![this, target_object, palette, 0u32]);
        assert_eq!(
            calls_to(&e, TREAD_CONTROLLER_SET),
            vec![vec![tread, 4, scene_object, 0, records]]
        );
        assert_eq!(
            calls_to(&e, TREAD_CONTROLLER_FIND_INDEX),
            vec![vec![tread, scene_object]]
        );
    }

    /// Maps the float constants the unit reads: `-FLT_MAX` (float) and `FLT_MAX` (float).
    fn map_float_constants(e: &mut Engine) {
        e.map(NEGATIVE_FLT_MAX_FLOAT, 4);
        set_float(e, NEGATIVE_FLT_MAX_FLOAT, -f32::MAX);
        e.map(FLT_MAX_FLOAT, 4);
        set_float(e, FLT_MAX_FLOAT, f32::MAX);
    }

    /// Two sequences for the morph frame tests: `other` runs 0 to 10 at the frequency 1.0 with the
    /// given morph keys, `this` runs 0 to 100.
    fn morph_pair(
        e: &mut Engine,
        this_keys: &[(f32, &str)],
        other_keys: &[(f32, &str)],
    ) -> (Ptr<NiControllerSequence>, Ptr<NiControllerSequence>) {
        string_compare_doubles(e);
        e.map(NEGATIVE_FLT_MAX_DOUBLE, 8);
        e.mem.set_u32(
            NEGATIVE_FLT_MAX_DOUBLE,
            (-f64::from(f32::MAX)).to_bits() as u32,
        );
        e.mem.set_u32(
            NEGATIVE_FLT_MAX_DOUBLE + 4,
            ((-f64::from(f32::MAX)).to_bits() >> 32) as u32,
        );
        let this = sequence(e);
        let other = sequence(e);
        for (sequence, end, keys) in [(this, 100.0, this_keys), (other, 10.0, other_keys)] {
            set_float(e, sequence.addr() + 0x28, 1.0);
            set_float(e, sequence.addr() + 0x2c, 0.0);
            set_float(e, sequence.addr() + 0x30, end);
            set_float(e, sequence.addr() + 0x34, 0.0);
            set_float(e, sequence.addr() + 0x38, 0.0);
            e.set(sequence, NiControllerSequence::m_eCycleType, 1);
            let texts: Vec<(f32, u32)> = keys
                .iter()
                .map(|(time, text)| (*time, string(e, text)))
                .collect();
            set_text_keys(e, sequence, &texts);
        }
        (this, other)
    }

    fn morph_frame(
        e: &mut Engine,
        this: Ptr<NiControllerSequence>,
        other: Ptr<NiControllerSequence>,
        time: f32,
    ) -> f32 {
        e.call(0x00a32010, &args![this, other, time.to_bits()])
            .f32()
    }

    /// `value` as the x87 gets it: the ratio stored as a float, the product and sum in double precision.
    fn morph_value(ratio: f64, from: f32, to: f32) -> f32 {
        let ratio = ratio as f32;
        ((f64::from(to) - f64::from(from)) * f64::from(ratio) + f64::from(from)) as f32
    }

    #[test]
    fn find_corresponding_morph_frame_interpolates_between_the_matching_keys() {
        let mut e = setup();
        let (this, other) = morph_pair(
            &mut e,
            &[(20.0, "morph:a"), (40.0, "morph:b")],
            &[(2.0, "morph:a"), (6.0, "morph:b")],
        );
        // Halfway from the key at 2 to the one at 6.
        assert_eq!(morph_frame(&mut e, this, other, 4.0), 30.0);
        assert_eq!(calls_to(&e, NI_STRICMP).len(), 3);
    }

    #[test]
    fn find_corresponding_morph_frame_wraps_past_the_last_key() {
        let mut e = setup();
        let (this, other) = morph_pair(
            &mut e,
            &[(20.0, "morph:a"), (40.0, "morph:b")],
            &[(2.0, "morph:a"), (6.0, "morph:b")],
        );
        // The next key is the first one moved by the length (12), the matching key of this
        // sequence (20) is moved by its length (100).
        let expected = morph_value(2.0 / 6.0, 40.0, 120.0);
        assert_eq!(morph_frame(&mut e, this, other, 8.0), expected);
    }

    #[test]
    fn find_corresponding_morph_frame_wraps_before_the_first_key_and_brings_the_result_into_range()
    {
        let mut e = setup();
        let (this, other) = morph_pair(
            &mut e,
            &[(20.0, "morph:a"), (40.0, "morph:b")],
            &[(2.0, "morph:a"), (6.0, "morph:b")],
        );
        // The previous key is the last one moved back by the length (-4); the answer 106.67 is
        // above the end time and is brought back by the length.
        let expected = morph_value(5.0 / 6.0, 40.0, 120.0);
        assert!(expected > 100.0);
        let expected = (f64::from(expected) - 100.0) as f32;
        assert_eq!(morph_frame(&mut e, this, other, 1.0), expected);
    }

    #[test]
    fn find_corresponding_morph_frame_moves_one_key_by_the_length_when_both_are_the_same() {
        let mut e = setup();
        let (this, other) = morph_pair(&mut e, &[(20.0, "morph:a")], &[(2.0, "morph:a")]);
        // One morph key: it is both neighbours, the wrap moves the second by the length (100).
        let expected = morph_value(3.0 / 10.0, 20.0, 120.0);
        assert_eq!(morph_frame(&mut e, this, other, 5.0), expected);
    }

    #[test]
    fn find_corresponding_morph_frame_skips_the_keys_that_are_not_morph_keys() {
        let mut e = setup();
        let (this, other) = morph_pair(
            &mut e,
            &[(20.0, "morph:a"), (30.0, "sound"), (40.0, "morph:b")],
            &[(2.0, "morph:a"), (4.0, "sound"), (6.0, "morph:b")],
        );
        // The key at 4 is a note, not a morph key: the neighbours of 4 are still the keys at 2 and 6.
        assert_eq!(morph_frame(&mut e, this, other, 4.0), 30.0);
    }

    #[test]
    fn fn_00a32290_stores_the_three_floats_in_the_blend_interpolators() {
        let mut e = setup();
        map_float_constants(&mut e);
        let this = sequence(&mut e);
        let (items, _) = set_items(&mut e, this, 3);
        // Item 0: blend index 1 of a blend interpolator with the elements at +0x14 (0x18 bytes each).
        let blend = e.mem.alloc(0x40);
        let elements = e.mem.alloc(0x30);
        e.mem.set_u32(blend + 0x14, elements);
        e.mem.set_u32(items + 8, blend);
        e.mem.set_u8(items + 0xc, 1);
        // Item 1: a blend interpolator in the single mode whose index is the item's.
        let single = e.mem.alloc(0x40);
        e.mem.set_u32(items + INTERP_ITEM_SIZE + 8, single);
        e.mem.set_u8(single + 0xe, 1);
        e.mem.set_u8(single + 0xf, 2);
        e.mem.set_u8(items + INTERP_ITEM_SIZE + 0xc, 2);
        // Item 2 has no blend interpolator.
        e.call(
            0x00a32290,
            &args![this, 1.5f32.to_bits(), 0x7f80_0001u32, 2.5f32.to_bits()],
        );
        let element = elements + 0x18;
        assert_eq!(e.mem.u32(element + 4), 1.5f32.to_bits());
        assert_eq!(e.mem.u32(element + 0x10), 0x7fc0_0001);
        assert_eq!(e.mem.u32(element + 0x14), 2.5f32.to_bits());
        assert_eq!(e.mem.u8(blend + 0xc), 4);
        for field in [0x24u32, 0x28, 0x2c] {
            assert_eq!(e.mem.u32(blend + field), (-f32::MAX).to_bits());
        }
        // The single mode takes only the last float.
        assert_eq!(e.mem.u32(single + 0x20), 2.5f32.to_bits());
        assert_eq!(e.mem.u8(single + 0xc), 0);
    }

    #[test]
    fn fn_00a32290_does_nothing_without_items() {
        let mut e = setup();
        let this = sequence(&mut e);
        e.call(0x00a32290, &args![this, 0u32, 0u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    /// Doubles for the allocation of element arrays: the memory manager's `Allocate` answers heap
    /// blocks (zeroed) and the vector constructor iterator does nothing.
    fn allocator_doubles(e: &mut Engine) {
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
    }

    #[test]
    fn fn_00a32330_grows_the_arrays_by_the_growth() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        silent_interlocked(&mut e);
        allocator_doubles(&mut e);
        let this = sequence(&mut e);
        e.set(this, NiControllerSequence::m_uiArrayGrowBy, 3);
        // Two items and tags in arrays with a count word, as the exe makes them.
        let items = element_block(&mut e, 2, INTERP_ITEM_SIZE);
        let tags = element_block(&mut e, 2, ID_TAG_SIZE);
        e.set(this, NiControllerSequence::m_uiArraySize, 2);
        e.set(this, NiControllerSequence::m_pkInterpArray, Ptr::new(items));
        e.set(this, NiControllerSequence::m_pkIDTagArray, Ptr::new(tags));
        e.mem.set_u32(items + INTERP_ITEM_SIZE, 0x4000);
        e.mem.set_u8(items + INTERP_ITEM_SIZE + 0xd, 6);
        e.mem.set_u32(tags + ID_TAG_SIZE, 0x7500);
        assert!(e.call(0x00a32330, &args![this]).bool());
        assert_eq!(e.get(this, NiControllerSequence::m_uiArraySize), 5);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![
                vec![MEMORY_MANAGER, 5 * INTERP_ITEM_SIZE + 4],
                vec![MEMORY_MANAGER, 5 * ID_TAG_SIZE + 4]
            ]
        );
        let new_items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
        let new_tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
        assert_ne!(new_items, items);
        assert_eq!(e.mem.u32(new_items - 4), 5);
        assert_eq!(e.mem.u32(new_tags - 4), 5);
        assert_eq!(e.mem.u32(new_items + INTERP_ITEM_SIZE), 0x4000);
        assert_eq!(e.mem.u8(new_items + INTERP_ITEM_SIZE + 0xd), 6);
        assert_eq!(e.mem.u32(new_tags + ID_TAG_SIZE), 0x7500);
        // The constructors ran over the whole new arrays; the old arrays were freed.
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![
                vec![new_items, INTERP_ITEM_SIZE, 5, INTERP_ITEM_CONSTRUCTOR],
                vec![new_tags, ID_TAG_SIZE, 5, ID_TAG_CONSTRUCTOR]
            ]
        );
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![
                vec![MEMORY_MANAGER, items - 4],
                vec![MEMORY_MANAGER, tags - 4]
            ]
        );
    }

    #[test]
    fn fn_00a32330_refuses_without_a_growth() {
        let mut e = setup();
        let this = sequence(&mut e);
        assert!(!e.call(0x00a32330, &args![this]).bool());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    /// A map double for `00a53c30`: answers the clone in `table` for a key and whether there is one.
    fn clone_map_double(e: &mut Engine, table: Vec<(u32, u32)>) {
        e.register_double(CLONE_MAP_GET_AT, move |e, a| {
            match table.iter().find(|(key, _)| *key == a[1]) {
                Some((_, clone)) => {
                    e.mem.set_u32(a[2], *clone);
                    1u32.into_ret()
                }
                None => 0u32.into_ret(),
            }
        });
    }

    #[test]
    fn process_clone_without_an_owner_clones_the_interpolators_and_controllers() {
        let mut e = setup();
        e.register(OBJECT_PROCESS_CLONE, |_, _| Ret::default());
        e.register(target(SLOT_PROCESS_CLONE), |_, _| Ret::default());
        let this = sequence(&mut e);
        let clone = sequence(&mut e);
        let (items, _) = set_items(&mut e, this, 2);
        let interpolator = object(&mut e);
        let controller = object(&mut e);
        let other_interpolator = object(&mut e);
        e.mem.set_u32(items, interpolator);
        e.mem.set_u32(items + 4, controller);
        e.mem.set_u32(items + INTERP_ITEM_SIZE, other_interpolator);
        let partner = object(&mut e);
        let partner_clone = object(&mut e);
        let root = object(&mut e);
        e.mem.set_u32(this.addr() + 0x58, partner);
        e.mem.set_u32(this.addr() + 0x60, root);
        e.mem.set_u32(this.addr() + 0x40, 0);
        clone_map_double(
            &mut e,
            vec![(this.addr(), clone.addr()), (partner, partner_clone)],
        );
        let process = e.mem.alloc(8);
        e.call(0x00a32470, &args![this, process]);
        assert_eq!(
            calls_to(&e, OBJECT_PROCESS_CLONE),
            vec![vec![this.addr(), process]]
        );
        assert_eq!(
            calls_to(&e, target(SLOT_PROCESS_CLONE)),
            vec![
                vec![interpolator, process],
                vec![controller, process],
                vec![other_interpolator, process]
            ]
        );
        // The owner is null (not in the map: the original is copied), the partner is the clone
        // of the partner and the accum root is not in the map.
        assert_eq!(e.mem.u32(clone.addr() + 0x40), 0);
        assert_eq!(e.mem.u32(clone.addr() + 0x58), partner_clone);
        assert_eq!(e.mem.u32(clone.addr() + 0x60), root);
    }

    #[test]
    fn process_clone_with_an_owner_points_the_clone_at_the_cloned_objects() {
        let mut e = setup();
        e.register(OBJECT_PROCESS_CLONE, |_, _| Ret::default());
        e.register(POINTER_ASSIGN, |_, _| Ret::default());
        e.register(POINTER_ASSIGN_FROM_REFERENCE, |_, _| Ret::default());
        e.register(target(SLOT_PROCESS_CLONE), |_, _| Ret::default());
        let this = sequence(&mut e);
        let clone = sequence(&mut e);
        let (items, _) = set_items(&mut e, this, 2);
        let (clone_items, _) = set_items(&mut e, clone, 2);
        let owner = object(&mut e);
        let owner_clone = object(&mut e);
        e.mem.set_u32(this.addr() + 0x40, owner);
        e.set(this, NiControllerSequence::m_eState, 1);
        let interpolator = object(&mut e);
        let interpolator_clone = object(&mut e);
        let controller = object(&mut e);
        let controller_clone = object(&mut e);
        let blend = object(&mut e);
        let blend_clone = object(&mut e);
        // Item 0 has clones for all three, item 1 for none.
        e.mem.set_u32(items, interpolator);
        e.mem.set_u32(items + 4, controller);
        e.mem.set_u32(items + 8, blend);
        let other_interpolator = object(&mut e);
        let other_controller = object(&mut e);
        e.mem.set_u32(items + INTERP_ITEM_SIZE, other_interpolator);
        e.mem
            .set_u32(items + INTERP_ITEM_SIZE + 4, other_controller);
        e.mem.set_u32(items + INTERP_ITEM_SIZE + 8, 0x4321);
        let stale = object(&mut e);
        let other_stale = object(&mut e);
        e.mem.set_u32(clone_items + 4, stale);
        e.mem
            .set_u32(clone_items + INTERP_ITEM_SIZE + 4, other_stale);
        clone_map_double(
            &mut e,
            vec![
                (this.addr(), clone.addr()),
                (interpolator, interpolator_clone),
                (controller, controller_clone),
                (blend, blend_clone),
                (owner, owner_clone),
            ],
        );
        let process = e.mem.alloc(8);
        e.call(0x00a32470, &args![this, process]);
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN),
            vec![vec![clone_items, interpolator_clone]]
        );
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN_FROM_REFERENCE),
            vec![vec![
                clone_items + INTERP_ITEM_SIZE,
                items + INTERP_ITEM_SIZE
            ]]
        );
        assert_eq!(e.mem.u32(clone_items + 4), controller_clone);
        assert_eq!(e.mem.u32(clone_items + 8), blend_clone);
        assert_eq!(
            e.mem.u32(clone_items + INTERP_ITEM_SIZE + 4),
            other_controller
        );
        assert_eq!(e.mem.u32(clone_items + INTERP_ITEM_SIZE + 8), 0x4321);
        assert_eq!(e.mem.u32(clone.addr() + 0x40), owner_clone);
        // The stale controller lost its count, the clones gained one.
        assert_eq!(count_of(&e, stale), 0);
        assert_eq!(count_of(&e, other_stale), 0);
        assert_eq!(count_of(&e, controller_clone), 2);
        assert_eq!(count_of(&e, other_controller), 2);
    }

    #[test]
    fn process_clone_of_an_inactive_sequence_with_an_owner_only_clones_the_interpolator() {
        let mut e = setup();
        e.register(OBJECT_PROCESS_CLONE, |_, _| Ret::default());
        e.register(target(SLOT_PROCESS_CLONE), |_, _| Ret::default());
        let this = sequence(&mut e);
        let clone = sequence(&mut e);
        let (items, _) = set_items(&mut e, this, 1);
        set_items(&mut e, clone, 1);
        let owner = object(&mut e);
        e.mem.set_u32(this.addr() + 0x40, owner);
        let interpolator = object(&mut e);
        e.mem.set_u32(items, interpolator);
        clone_map_double(&mut e, vec![(this.addr(), clone.addr())]);
        let process = e.mem.alloc(8);
        e.call(0x00a32470, &args![this, process]);
        assert_eq!(
            calls_to(&e, target(SLOT_PROCESS_CLONE)),
            vec![vec![interpolator, process]]
        );
    }

    #[test]
    fn link_object_resolves_the_link_ids_of_an_old_file() {
        let mut e = setup();
        e.register(OBJECT_STREAM_HOOK, |_, _| Ret::default());
        let links = Rc::new(RefCell::new(Vec::<u32>::new()));
        let queue = links.clone();
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            queue.borrow_mut().remove(0).into_ret()
        });
        let (stream, _, _) = stream_with(&mut e, 0, 0, 0x0a01_0067, 0);
        let this = sequence(&mut e);
        let (items, _) = set_items(&mut e, this, 2);
        let keys = object(&mut e);
        let first = object(&mut e);
        let second = object(&mut e);
        *links.borrow_mut() = vec![keys, first, second];
        e.call(0x00a326e0, &args![this, stream]);
        assert_eq!(e.mem.u32(this.addr() + 0x20), keys);
        assert_eq!(e.mem.u32(items + 4), first);
        assert_eq!(e.mem.u32(items + INTERP_ITEM_SIZE + 4), second);
        assert_eq!(count_of(&e, keys), 2);
        assert!(links.borrow().is_empty());
    }

    #[test]
    fn link_object_resolves_the_link_ids_of_a_newer_file() {
        let mut e = setup();
        e.register(OBJECT_STREAM_HOOK, |_, _| Ret::default());
        let links = Rc::new(RefCell::new(Vec::<u32>::new()));
        let queue = links.clone();
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            queue.borrow_mut().remove(0).into_ret()
        });
        let (stream, _, _) = stream_with(&mut e, 0, 0, 0x0a01_0068, 0x1e);
        let this = sequence(&mut e);
        let keys = object(&mut e);
        let owner = object(&mut e);
        let notes = e.mem.alloc(8);
        let first = object(&mut e);
        let second = object(&mut e);
        e.set(this, NiControllerSequence::spAnimNotesA, Ptr::new(notes));
        e.set(this, NiControllerSequence::usNumNotes, 2);
        *links.borrow_mut() = vec![keys, owner, first, second];
        e.call(0x00a326e0, &args![this, stream]);
        assert_eq!(e.mem.u32(this.addr() + 0x20), keys);
        // The owner is stored without counting it.
        assert_eq!(e.mem.u32(this.addr() + 0x40), owner);
        assert_eq!(count_of(&e, owner), 1);
        assert_eq!(e.mem.u32(notes), first);
        assert_eq!(e.mem.u32(notes + 4), second);
        assert_eq!(count_of(&e, second), 2);
    }

    #[test]
    fn link_object_drops_the_notes_of_a_stream_version_between_0x15_and_0x1d() {
        let mut e = setup();
        e.register(OBJECT_STREAM_HOOK, |_, _| Ret::default());
        e.register(STREAM_GET_OBJECT_FROM_LINK_ID, |_, _| Ret::default());
        let (stream, _, _) = stream_with(&mut e, 0, 0, 0x0a01_0068, 0x16);
        let this = sequence(&mut e);
        e.set(this, NiControllerSequence::spAnimNotesA, Ptr::new(0x1234));
        e.set(this, NiControllerSequence::usNumNotes, 3);
        e.call(0x00a326e0, &args![this, stream]);
        assert_eq!(calls_to(&e, STREAM_GET_OBJECT_FROM_LINK_ID).len(), 3);
        assert_eq!(e.get(this, NiControllerSequence::spAnimNotesA).addr(), 0);
        assert_eq!(e.get(this, NiControllerSequence::usNumNotes), 0);
    }

    #[test]
    fn set_phase_stores_the_offset_and_the_phase_in_the_controllers() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        let this = sequence(&mut e);
        let (items, tags) = set_items(&mut e, this, 4);
        set_float(&mut e, this.addr() + 0x2c, 1.5);
        let transform_name = string(&mut e, "NiTransformController");
        let other_name = string(&mut e, "NiAlphaController");
        let controllers: Vec<u32> = (0..4).map(|_| e.mem.alloc(0x20)).collect();
        for (index, controller) in controllers.iter().enumerate() {
            e.mem
                .set_u32(items + index as u32 * INTERP_ITEM_SIZE + 4, *controller);
        }
        // Item 0: a transform controller (skipped); 1: another type (written); 2: the empty
        // string (skipped); 3: no controller (skipped).
        e.mem.set_u32(tags + 8, transform_name);
        e.mem.set_u32(tags + ID_TAG_SIZE + 8, other_name);
        e.mem.set_u32(tags + 2 * ID_TAG_SIZE + 8, 0x9000);
        e.mem.set_u32(tags + 3 * ID_TAG_SIZE + 8, other_name);
        e.mem.set_u32(items + 3 * INTERP_ITEM_SIZE + 4, 0);
        e.call(0x00a328b0, &args![this, 2.0f32.to_bits(), 1u32]);
        assert_eq!(float_at(&e, this.addr() + 0x48), 3.5);
        assert_eq!(float_at(&e, controllers[1] + 0x10), 2.0);
        for index in [0, 2] {
            assert_eq!(e.mem.u32(controllers[index] + 0x10), 0);
        }
        // Without the flag only the offset changes.
        e.mem.set_u32(controllers[1] + 0x10, 0);
        e.call(0x00a328b0, &args![this, 4.0f32.to_bits(), 0u32]);
        assert_eq!(float_at(&e, this.addr() + 0x48), 5.5);
        assert_eq!(e.mem.u32(controllers[1] + 0x10), 0);
    }

    #[test]
    fn set_phase_passes_a_nan_through_quieted() {
        let mut e = setup();
        let this = sequence(&mut e);
        e.call(0x00a328b0, &args![this, 0x7f80_0001u32, 0u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x48), 0x7fc0_0001);
    }

    #[test]
    fn fn_00a32940_sets_an_element_and_keeps_the_counts() {
        let mut e = setup();
        map_statics(&mut e);
        e.register(ATEXIT, |_, _| Ret::default());
        e.mem.set_u32(NULL_POINTER_GUARD, 0);
        let array = e.mem.alloc(0x10);
        let storage = e.mem.alloc(0x20);
        e.mem.set_u32(array + 4, storage);
        e.mem.set_u16(array + 8, 8);
        let first = object(&mut e);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, first);
        e.call(0x00a32940, &args![array, 2u32, holder]);
        // The static was made on this first use.
        assert_eq!(calls_to(&e, ATEXIT), vec![vec![NULL_POINTER_ATEXIT]]);
        assert_eq!(e.mem.u32(NULL_POINTER_GUARD) & 1, 1);
        assert_eq!(e.mem.u16(array + 0xa), 3);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert_eq!(e.mem.u32(storage + 8), first);
        assert_eq!(count_of(&e, first), 2);
        // Replacing it with another object: the count stays, the old one is released.
        let second = object(&mut e);
        e.mem.set_u32(holder, second);
        e.call(0x00a32940, &args![array, 2u32, holder]);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert_eq!(count_of(&e, first), 1);
        assert_eq!(count_of(&e, second), 2);
        // Clearing it and clearing an unset element.
        e.mem.set_u32(holder, 0);
        e.call(0x00a32940, &args![array, 2u32, holder]);
        assert_eq!(e.mem.u16(array + 0xc), 0);
        e.call(0x00a32940, &args![array, 1u32, holder]);
        assert_eq!(e.mem.u16(array + 0xc), 0);
        // Filling an unset element below the size, and past the size.
        e.mem.set_u32(holder, first);
        e.call(0x00a32940, &args![array, 1u32, holder]);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        e.call(0x00a32940, &args![array, 6u32, holder]);
        assert_eq!(e.mem.u16(array + 0xa), 7);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        e.mem.set_u32(holder, 0);
        e.call(0x00a32940, &args![array, 7u32, holder]);
        assert_eq!(e.mem.u16(array + 0xa), 8);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        // The static is made only once.
        assert_eq!(calls_to(&e, ATEXIT).len(), 1);
    }

    #[test]
    fn fn_00a32a10_constructs_an_empty_sequence_with_arrays() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        map_float_constants(&mut e);
        silent_interlocked(&mut e);
        allocator_doubles(&mut e);
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x74);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x7600);
        assert_eq!(
            e.call(0x00a32a10, &args![this, name, 3u32, 4u32]).u32(),
            this
        );
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), NI_CONTROLLER_SEQUENCE_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x7600);
        assert_eq!(e.mem.u32(this + 0xc), 3);
        assert_eq!(e.mem.u32(this + 0x10), 4);
        assert_eq!(float_at(&e, this + 0x1c), 1.0);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(float_at(&e, this + 0x28), 1.0);
        assert_eq!(float_at(&e, this + 0x2c), f32::MAX);
        for field in [0x30u32, 0x34, 0x38, 0x3c, 0x48, 0x4c, 0x50, 0x54] {
            assert_eq!(float_at(&e, this + field), -f32::MAX, "field {field:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x40), 0);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(e.mem.u32(this + 0x58), 0);
        assert_eq!(e.mem.u32(this + 0x5c), 0x9000);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.u16(this + 0x68), 0xffff);
        assert_eq!(e.mem.u32(this + 0x6c), 0);
        assert_eq!(e.mem.u16(this + 0x70), 0);
        assert_eq!(e.mem.u8(this + 0x72), 0);
        let items = e.mem.u32(this + 0x14);
        let tags = e.mem.u32(this + 0x18);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![
                vec![items, INTERP_ITEM_SIZE, 3, INTERP_ITEM_CONSTRUCTOR],
                vec![tags, ID_TAG_SIZE, 3, ID_TAG_CONSTRUCTOR]
            ]
        );
        // The name was counted (it replaced the empty string's handle).
        assert_eq!(calls_to(&e, INTERLOCKED_INCREMENT), vec![vec![0x7600 - 8]]);
        // Without items no arrays are made.
        let empty = e.mem.alloc(0x74);
        e.call(0x00a32a10, &args![empty, name, 0u32, 0u32]);
        assert_eq!(e.mem.u32(empty + 0x14), 0);
        assert_eq!(e.mem.u32(empty + 0x18), 0);
    }

    #[test]
    fn fn_00a32b80_clears_an_item_and_releases_the_interpolator() {
        let mut e = setup();
        e.map(INVALID_BLEND_INDEX, 1);
        e.register(ID_TAG_RESET, |_, _| Ret::default());
        let this = sequence(&mut e);
        let (items, tags) = set_items(&mut e, this, 1);
        let interpolator = object(&mut e);
        e.mem.set_u32(items, interpolator);
        e.call(0x00a32b80, &args![this, 0u32]);
        assert_eq!(e.mem.u32(items), 0);
        assert_eq!(count_of(&e, interpolator), 0);
        assert_eq!(
            calls_to(&e, target(SLOT_DELETE_THIS)),
            vec![vec![interpolator]]
        );
        assert_eq!(calls_to(&e, ID_TAG_RESET), vec![vec![tags]]);
    }

    #[test]
    fn fn_00a32bc0_adds_an_interpolator_in_the_first_free_item() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        silent_interlocked(&mut e);
        let this = sequence(&mut e);
        let (items, tags) = set_items(&mut e, this, 3);
        e.mem.set_u32(items, 0x4000);
        let interpolator = object(&mut e);
        let tag = e.mem.alloc(ID_TAG_SIZE);
        for word in 0..5 {
            e.mem.set_u32(tag + 4 * word, 0x7700 + word);
        }
        assert_eq!(
            e.call(0x00a32bc0, &args![this, interpolator, tag, 9u32])
                .u32(),
            1
        );
        assert_eq!(e.mem.u32(items + INTERP_ITEM_SIZE), interpolator);
        assert_eq!(e.mem.u8(items + INTERP_ITEM_SIZE + 0xd), 9);
        assert_eq!(e.mem.u32(tags + ID_TAG_SIZE + 0x10), 0x7704);
        assert_eq!(e.mem.u32(tags + 0x10), 0);
    }

    #[test]
    fn fn_00a32bc0_grows_a_full_sequence_or_answers_minus_one() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        silent_interlocked(&mut e);
        allocator_doubles(&mut e);
        let this = sequence(&mut e);
        let items = element_block(&mut e, 1, INTERP_ITEM_SIZE);
        let tags = element_block(&mut e, 1, ID_TAG_SIZE);
        e.set(this, NiControllerSequence::m_uiArraySize, 1);
        e.set(this, NiControllerSequence::m_pkInterpArray, Ptr::new(items));
        e.set(this, NiControllerSequence::m_pkIDTagArray, Ptr::new(tags));
        e.mem.set_u32(items, 0x4000);
        let interpolator = object(&mut e);
        let tag = e.mem.alloc(ID_TAG_SIZE);
        // No growth: the sequence stays full.
        assert_eq!(
            e.call(0x00a32bc0, &args![this, interpolator, tag, 1u32])
                .u32(),
            0xffff_ffff
        );
        e.set(this, NiControllerSequence::m_uiArrayGrowBy, 2);
        assert_eq!(
            e.call(0x00a32bc0, &args![this, interpolator, tag, 1u32])
                .u32(),
            1
        );
        assert_eq!(e.get(this, NiControllerSequence::m_uiArraySize), 3);
        let new_items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
        assert_eq!(e.mem.u32(new_items), 0x4000);
        assert_eq!(e.mem.u32(new_items + INTERP_ITEM_SIZE), interpolator);
    }

    /// An attach fixture for `00a32c70`: a sequence with an owner whose palette (`+0x78`) is a test
    /// object, one item with an interpolator and its own controller, and one tag naming a class
    /// that is neither of the controllers the object has. `GetAVObject` of the palette answers `found`.
    struct Attach {
        this: Ptr<NiControllerSequence>,
        items: u32,
        tag: u32,
        found: u32,
        palette: u32,
        own: u32,
        target_object: u32,
    }

    fn attach_fixture(e: &mut Engine) -> Attach {
        map_fixed_strings(e);
        map_statics(e);
        controller_doubles(e, 0, 0);
        e.register(PALETTE_ITEM_COUNT, |_, _| 0u32.into_ret());
        e.register(target(SLOT_INTERPOLATOR_ACCEPTS_PALETTE), |_, _| {
            1u32.into_ret()
        });
        e.register(OBJECT_CAST, |_, a| a[0].into_ret());
        e.register(POINTER_SLOT_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let found = object(e);
        let palette = palette(e, NI_DEFAULT_AV_OBJECT_PALETTE_RTTI, found);
        let owner = e.mem.alloc(0x80);
        e.mem.set_u32(owner + 0x78, palette);
        let target_object = found;
        e.mem.set_u32(owner + 0x2c, target_object);
        let this = sequence(e);
        e.mem.set_u32(this.addr() + 0x40, owner);
        let (items, tags) = set_items(e, this, 1);
        let interpolator = object(e);
        let own = object(e);
        e.mem.set_u32(items, interpolator);
        e.mem.set_u32(items + 4, own);
        let alpha = string(e, "NiAlphaController");
        let name = string(e, "Spine");
        e.mem.set_u32(tags, name);
        e.mem.set_u32(tags + 8, alpha);
        e.mem.set_u32(tags + 0x10, 0x1111);
        e.mem.set_u32(this.addr() + 0x5c, 0x9000);
        Attach {
            this,
            items,
            tag: tags,
            found,
            palette,
            own,
            target_object,
        }
    }

    #[test]
    fn fn_00a32c70_gives_an_item_its_own_controller_and_a_new_blend_interpolator() {
        let mut e = setup();
        let fixture = attach_fixture(&mut e);
        let blend = object(&mut e);
        e.register(target(SLOT_SET_TARGET), |_, _| Ret::default());
        e.register(target(SLOT_GET_INTERPOLATOR_INDEX), |_, _| 4u32.into_ret());
        e.register(target(SLOT_GET_INTERPOLATOR), |_, _| 0u32.into_ret());
        e.register_double(target(SLOT_CREATE_BLEND_INTERPOLATOR), move |_, _| {
            blend.into_ret()
        });
        e.register(target(SLOT_SET_INTERPOLATOR), |_, _| Ret::default());
        let flags = Rc::new(RefCell::new(Vec::new()));
        let seen = flags.clone();
        e.register_double(target(SLOT_SELECTIVE_UPDATE_FLAGS), move |e, a| {
            seen.borrow_mut().push((a[2], e.mem.u32(a[3])));
            Ret::default()
        });
        assert!(e
            .call(0x00a32c70, &args![fixture.this, fixture.target_object])
            .bool());
        let own = fixture.own;
        assert_eq!(
            calls_to(&e, target(SLOT_SET_TARGET)),
            vec![vec![own, fixture.found]]
        );
        // The object is the owner's target: the own controller goes into the owner's chain.
        let owner = e.get(fixture.this, NiControllerSequence::m_pkOwner).addr();
        assert_eq!(
            calls_to(&e, OBJECT_NET_REMOVE_CONTROLLER),
            vec![vec![fixture.found, own]]
        );
        assert_eq!(
            calls_to(&e, CONTROLLER_SET_NEXT),
            vec![vec![own, 0], vec![owner, own]]
        );
        assert_eq!(
            calls_to(&e, target(SLOT_GET_INTERPOLATOR_INDEX)),
            vec![vec![own, 0x1111]]
        );
        assert_eq!(
            calls_to(&e, target(SLOT_CREATE_BLEND_INTERPOLATOR)),
            vec![vec![own, 4, 1, 0, 0, 2]]
        );
        assert_eq!(
            calls_to(&e, target(SLOT_SET_INTERPOLATOR)),
            vec![vec![own, blend, 4]]
        );
        assert_eq!(e.mem.u16(own + 8), 0x20);
        assert_eq!(e.mem.u32(fixture.items + 8), blend);
        // The owner's target is told that a controller was made.
        assert_eq!(
            flags.borrow().clone(),
            vec![(1, (fixture.target_object & 0xffff_ff00) | 1)]
        );
        // The accum root name is empty: no object was looked up for it.
        assert_eq!(calls_to(&e, target(SLOT_GET_AV_OBJECT)).len(), 1);
    }

    #[test]
    fn fn_00a32c70_uses_the_matching_controller_of_the_object() {
        let mut e = setup();
        let fixture = attach_fixture(&mut e);
        // The object has a controller whose class is the tag's, with an interpolator that is a
        // blend interpolator already.
        let controller = object(&mut e);
        e.mem.set_u32(fixture.found + 0xc, controller);
        let class_name = e.mem.u32(fixture.tag + 8);
        let record = e.mem.alloc(8);
        e.mem.set_u32(record, class_name);
        e.register_double(target(SLOT_GET_RTTI), move |_, a| {
            if a[0] == fixture.palette {
                NI_DEFAULT_AV_OBJECT_PALETTE_RTTI
            } else {
                record
            }
            .into_ret()
        });
        e.register(target(SLOT_GET_CTLR_ID), |_, _| 0u32.into_ret());
        let blend = object(&mut e);
        e.register(target(SLOT_GET_INTERPOLATOR_INDEX), |_, _| 2u32.into_ret());
        e.register_double(target(SLOT_GET_INTERPOLATOR), move |_, _| blend.into_ret());
        // The accum root name is set: the palette gives the accum root.
        e.mem.set_u32(fixture.this.addr() + 0x5c, 0x7000);
        assert!(e
            .call(0x00a32c70, &args![fixture.this, fixture.target_object])
            .bool());
        assert_eq!(
            e.get(fixture.this, NiControllerSequence::m_pkAccumRoot)
                .addr(),
            fixture.found
        );
        assert!(calls_to(&e, target(SLOT_SET_TARGET)).is_empty());
        assert!(calls_to(&e, target(SLOT_CREATE_BLEND_INTERPOLATOR)).is_empty());
        assert_eq!(e.mem.u32(fixture.items + 4), controller);
        assert_eq!(e.mem.u32(fixture.items + 8), blend);
        assert_eq!(e.mem.u16(controller + 8), 0x20);
        assert!(calls_to(&e, target(SLOT_SELECTIVE_UPDATE_FLAGS)).is_empty());
    }

    #[test]
    fn fn_00a32c70_marks_hidden_names_when_the_palette_has_no_object() {
        let mut e = setup();
        let fixture = attach_fixture(&mut e);
        e.register(target(SLOT_GET_AV_OBJECT), |_, _| 0u32.into_ret());
        let hidden = string(&mut e, "##Spine");
        e.mem.set_u32(fixture.tag, hidden);
        assert!(e
            .call(0x00a32c70, &args![fixture.this, fixture.target_object])
            .bool());
        assert_eq!(e.mem.u8(fixture.this.addr() + 0x72), 1);
        // A plain name leaves it alone.
        e.mem.set_u8(fixture.this.addr() + 0x72, 0);
        let plain = string(&mut e, "Spine");
        e.mem.set_u32(fixture.tag, plain);
        e.call(0x00a32c70, &args![fixture.this, fixture.target_object]);
        assert_eq!(e.mem.u8(fixture.this.addr() + 0x72), 0);
    }

    #[test]
    fn fn_00a32c70_looks_for_the_property_of_the_tag_in_the_property_list() {
        let mut e = setup();
        let fixture = attach_fixture(&mut e);
        // The tag names a property type: the object's list holds one of another class and then one of
        // that class, which becomes the object the controller is set on.
        let property_type = string(&mut e, "NiAlphaProperty");
        e.mem.set_u32(fixture.tag + 4, property_type);
        let other = object(&mut e);
        let wanted = object(&mut e);
        let other_name = string(&mut e, "NiTexturingProperty");
        let other_record = e.mem.alloc(8);
        e.mem.set_u32(other_record, other_name);
        let wanted_record = e.mem.alloc(8);
        e.mem.set_u32(wanted_record, property_type);
        let second_node = e.mem.alloc(0x10);
        e.mem.set_u32(second_node + 8, wanted);
        let first_node = e.mem.alloc(0x10);
        e.mem.set_u32(first_node, second_node);
        e.mem.set_u32(first_node + 8, other);
        e.mem.set_u32(fixture.found + 0x24, first_node);
        e.register_double(target(SLOT_GET_RTTI), move |_, a| {
            if a[0] == fixture.palette {
                NI_DEFAULT_AV_OBJECT_PALETTE_RTTI
            } else if a[0] == other {
                other_record
            } else {
                wanted_record
            }
            .into_ret()
        });
        e.register(target(SLOT_SET_TARGET), |_, _| Ret::default());
        e.register(target(SLOT_GET_INTERPOLATOR_INDEX), |_, _| {
            0xffffu32.into_ret()
        });
        e.register(target(SLOT_SELECTIVE_UPDATE_FLAGS), |_, _| Ret::default());
        e.call(0x00a32c70, &args![fixture.this, fixture.target_object]);
        // The property is the object: the own controller targets it.
        assert_eq!(
            calls_to(&e, target(SLOT_SET_TARGET)),
            vec![vec![fixture.own, wanted]]
        );
        // The interpolator index is the invalid one: nothing else happens to the item.
        assert_eq!(e.mem.u32(fixture.items + 8), 0);
    }

    #[test]
    fn fn_00a32fa0_copies_the_interpolators_as_pose_interpolators() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        map_statics(&mut e);
        map_float_constants(&mut e);
        e.map(INVALID_BLEND_INDEX, 1);
        silent_interlocked(&mut e);
        e.register(ID_TAG_RESET, |_, _| Ret::default());
        e.register(ID_TAG_DESTRUCT, |_, _| Ret::default());
        let this = sequence(&mut e);
        let destination = sequence(&mut e);
        let (items, tags) = set_items(&mut e, this, 2);
        let (dest_items, dest_tags) = set_items(&mut e, destination, 2);
        // The source: one item with an interpolator and a controller, one without.
        let interpolator = object(&mut e);
        let controller = object(&mut e);
        e.mem.set_u32(items, interpolator);
        e.mem.set_u32(items + 4, controller);
        e.mem.set_u32(items + 8, 0x4545);
        e.mem.set_u8(items + 0xd, 7);
        e.set(this, NiControllerSequence::m_eCycleType, 2);
        set_float(&mut e, this.addr() + 0x28, 1.5);
        set_float(&mut e, this.addr() + 0x2c, 2.5);
        set_float(&mut e, this.addr() + 0x30, 3.5);
        let pose = object(&mut e);
        e.register(target(SLOT_GET_INTERPOLATOR_INDEX_FX), |_, _| {
            3u32.into_ret()
        });
        e.register_double(target(SLOT_CREATE_POSE_INTERPOLATOR), move |_, _| {
            pose.into_ret()
        });
        // The destination's first item is used and is cleared first.
        let old = object(&mut e);
        e.mem.set_u32(dest_items, old);
        e.call(0x00a32fa0, &args![this, destination, 0u32, 0u32, 1u32]);
        assert_eq!(
            calls_to(&e, target(SLOT_GET_INTERPOLATOR_INDEX_FX)),
            vec![vec![controller, tags]]
        );
        assert_eq!(
            calls_to(&e, target(SLOT_CREATE_POSE_INTERPOLATOR)),
            vec![vec![controller, 3]]
        );
        assert_eq!(e.get(destination, NiControllerSequence::m_eCycleType), 2);
        assert_eq!(float_at(&e, destination.addr() + 0x28), 1.5);
        assert_eq!(float_at(&e, destination.addr() + 0x2c), 2.5);
        assert_eq!(float_at(&e, destination.addr() + 0x30), 3.5);
        // The cleared item is the first free one again: the pose interpolator went there.
        assert_eq!(e.mem.u32(dest_items), pose);
        assert_eq!(e.mem.u32(dest_items + 4), controller);
        assert_eq!(e.mem.u32(dest_items + 8), 0x4545);
        assert_eq!(e.mem.u8(dest_items + 0xd), 7);
        // Its tag is the default tag: five times the static handle.
        for word in 0..5 {
            assert_eq!(e.mem.u32(dest_tags + 4 * word), 0x7800);
        }
        assert_eq!(count_of(&e, old), 1);
        assert_eq!(calls_to(&e, ID_TAG_DESTRUCT).len(), 1);
    }

    #[test]
    fn fn_00a32fa0_leaves_the_destination_and_other_cycle_types_alone() {
        let mut e = setup();
        let this = sequence(&mut e);
        let destination = sequence(&mut e);
        set_items(&mut e, this, 1);
        e.set(this, NiControllerSequence::m_eCycleType, 1);
        e.set(destination, NiControllerSequence::m_eCycleType, 2);
        e.call(0x00a32fa0, &args![this, destination, 0u32, 0u32, 0u32]);
        assert_eq!(e.get(destination, NiControllerSequence::m_eCycleType), 2);
        // The item has neither an interpolator nor a controller: nothing is asked of anything.
        assert!(calls_to(&e, target(SLOT_GET_INTERPOLATOR_INDEX_FX)).is_empty());
    }

    /// Doubles for the C runtime functions `00a33130` uses: `strcpy_s` and `strchr`.
    fn c_string_doubles(e: &mut Engine) {
        e.register(STRING_COPY_SECURE, |e, a| {
            let mut offset = 0;
            loop {
                let byte = e.mem.u8(a[2] + offset);
                e.mem.set_u8(a[0] + offset, byte);
                if byte == 0 {
                    break;
                }
                offset += 1;
            }
            Ret::default()
        });
        e.register(STRCHR, |e, a| {
            let mut address = a[0];
            loop {
                let byte = e.mem.u8(address);
                if u32::from(byte) == a[1] {
                    return address.into_ret();
                }
                if byte == 0 {
                    return 0u32.into_ret();
                }
                address += 1;
            }
        });
    }

    /// `NiFixedString` from characters as a double: the handle is 0x20000 plus the text's length.
    fn fixed_string_by_length(e: &mut Engine) {
        e.register(FIXED_STRING_FROM_CHARS, |e, a| {
            let mut length = 0;
            while e.mem.u8(a[0] + length) != 0 {
                length += 1;
            }
            (0x20000 + length).into_ret()
        });
    }

    /// A controller (the test vtable) for `00a33130`: `count` interpolators, class record, ID text.
    fn viewer_controller(e: &mut Engine, count: u32, class_name: &str, id: u32) -> u32 {
        let controller = object(e);
        let class_name = string(e, class_name);
        let record = e.mem.alloc(8);
        e.mem.set_u32(record, class_name);
        e.register_double(target(SLOT_GET_INTERPOLATOR_COUNT), move |_, _| {
            count.into_ret()
        });
        e.register_double(target(SLOT_GET_RTTI), move |_, _| record.into_ret());
        e.register_double(target(SLOT_GET_CTLR_ID), move |_, _| id.into_ret());
        e.register(target(SLOT_GET_INTERPOLATOR_ID), |_, _| 0u32.into_ret());
        e.register(target(SLOT_GET_INTERPOLATOR), |_, a| {
            (0x4000 + a[1]).into_ret()
        });
        controller
    }

    #[test]
    fn fn_00a33130_adds_the_controllers_interpolators_for_an_object_name() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        silent_interlocked(&mut e);
        put_text(&mut e, TEXT_PROP_MARKER, "PROP");
        c_string_doubles(&mut e);
        fixed_string_by_length(&mut e);
        e.register(ID_TAG_RESET, |_, _| Ret::default());
        let this = sequence(&mut e);
        let (items, tags) = set_items(&mut e, this, 2);
        let controller = viewer_controller(&mut e, 2, "NiTransformController", 0);
        let text = string(&mut e, "Bip01");
        assert!(e.call(0x00a33130, &args![this, text, controller]).bool());
        // The two interpolators went into the two items, with priority 0xff and the controller.
        for (index, item) in [items, items + INTERP_ITEM_SIZE].into_iter().enumerate() {
            assert_eq!(e.mem.u32(item), 0x4000 + index as u32);
            assert_eq!(e.mem.u32(item + 4), controller);
            assert_eq!(e.mem.u8(item + 0xd), 0xff);
        }
        let counted = calls_to(&e, INTERLOCKED_INCREMENT)
            .iter()
            .filter(|call| call[0] == controller + 4)
            .count();
        assert_eq!(counted, 2);
        // The tag has the name (length 5), no property type and the class name (21).
        assert_eq!(e.mem.u32(tags), 0x20000 + 5);
        assert_eq!(e.mem.u32(tags + 4), 0x7800);
        assert_eq!(e.mem.u32(tags + 8), 0x20000 + 21);
        assert_eq!(e.mem.u32(tags + 0xc), 0x9000);
        assert_eq!(e.mem.u32(tags + 0x10), 0x9000);
    }

    #[test]
    fn fn_00a33130_takes_the_property_type_from_the_third_line() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        silent_interlocked(&mut e);
        put_text(&mut e, TEXT_PROP_MARKER, "PROP");
        c_string_doubles(&mut e);
        fixed_string_by_length(&mut e);
        e.register(ID_TAG_RESET, |_, _| Ret::default());
        let this = sequence(&mut e);
        let (_, tags) = set_items(&mut e, this, 1);
        let controller = viewer_controller(&mut e, 1, "NiAlpha", 0);
        let text = string(&mut e, "Bip01\nPROP\nNiAlphaProperty");
        assert!(e.call(0x00a33130, &args![this, text, controller]).bool());
        assert_eq!(e.mem.u32(tags), 0x20000 + 5);
        assert_eq!(e.mem.u32(tags + 4), 0x20000 + 15);
        // A second line that is not PROP leaves the property type empty.
        let other = sequence(&mut e);
        let (_, other_tags) = set_items(&mut e, other, 1);
        let text = string(
            &mut e,
            "Bip01
XXXX
NiAlphaProperty",
        );
        e.call(0x00a33130, &args![other, text, controller]);
        assert_eq!(e.mem.u32(other_tags + 4), 0x7800);
    }

    #[test]
    fn fn_00a33130_stops_when_the_sequence_cannot_take_another_interpolator() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        silent_interlocked(&mut e);
        put_text(&mut e, TEXT_PROP_MARKER, "PROP");
        c_string_doubles(&mut e);
        fixed_string_by_length(&mut e);
        e.register(ID_TAG_RESET, |_, _| Ret::default());
        let this = sequence(&mut e);
        let (items, _) = set_items(&mut e, this, 1);
        e.mem.set_u32(items, 0x5000);
        let controller = viewer_controller(&mut e, 1, "NiTransformController", 0);
        let text = string(&mut e, "Bip01");
        assert!(!e.call(0x00a33130, &args![this, text, controller]).bool());
        // The five handles of the temporary tag were released.
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 4 + 5);
    }

    #[test]
    fn fn_00a33130_does_nothing_for_a_controller_without_interpolators() {
        let mut e = setup();
        c_string_doubles(&mut e);
        let this = sequence(&mut e);
        let controller = viewer_controller(&mut e, 0, "NiTransformController", 0);
        let text = string(&mut e, "Bip01");
        assert!(e.call(0x00a33130, &args![this, text, controller]).bool());
    }

    const READ: u32 = 0x0300_3000;
    const WRITE: u32 = 0x0300_3100;

    /// A read function double: each call stores the next queued value (of the size asked for) at
    /// the buffer and answers the size.
    fn read_queue(e: &mut Engine, values: &[u32]) -> Rc<RefCell<Vec<u32>>> {
        let queue = Rc::new(RefCell::new(values.to_vec()));
        let source = queue.clone();
        e.register_double(READ, move |e, a| {
            let value = source.borrow_mut().remove(0);
            match a[2] {
                1 => e.mem.set_u8(a[1], value as u8),
                2 => e.mem.set_u16(a[1], value as u16),
                _ => e.mem.set_u32(a[1], value),
            }
            a[2].into_ret()
        });
        queue
    }

    /// A write function double: records the size and value of every write, in order, with the other
    /// events (`log`) of the test.
    fn write_log(e: &mut Engine, log: &Rc<RefCell<Vec<String>>>) {
        let events = log.clone();
        e.register_double(WRITE, move |e, a| {
            let value = match a[2] {
                1 => u32::from(e.mem.u8(a[1])),
                2 => u32::from(e.mem.u16(a[1])),
                _ => e.mem.u32(a[1]),
            };
            events
                .borrow_mut()
                .push(format!("write {} {:#x}", a[2], value));
            a[2].into_ret()
        });
    }

    /// Doubles for `00a33570` and the functions that run it: the object copy, the clone calls and the
    /// allocation of element arrays.
    fn copy_doubles(e: &mut Engine) {
        map_fixed_strings(e);
        silent_interlocked(e);
        allocator_doubles(e);
        e.register(OBJECT_COPY_MEMBERS, |_, _| Ret::default());
    }

    #[test]
    fn fn_00a33570_copies_a_sequence_without_an_owner() {
        let mut e = setup();
        copy_doubles(&mut e);
        let this = sequence(&mut e);
        let destination = sequence(&mut e);
        let (items, tags) = set_items(&mut e, this, 2);
        e.set(this, NiControllerSequence::m_uiArrayGrowBy, 4);
        e.mem.set_u32(this.addr() + 8, 0x7a00);
        // Item 0 has an interpolator and a controller to clone, item 1 only a priority.
        let interpolator = object(&mut e);
        let controller = object(&mut e);
        let interpolator_clone = object(&mut e);
        let controller_clone = object(&mut e);
        e.mem.set_u32(items, interpolator);
        e.mem.set_u32(items + 4, controller);
        e.mem.set_u8(items + INTERP_ITEM_SIZE + 0xd, 9);
        e.mem.set_u8(items + 0xd, 5);
        e.mem.set_u32(tags + 8, 0x7a10);
        e.mem.set_u32(tags + ID_TAG_SIZE + 0x10, 0x7a20);
        let clones = vec![
            (interpolator, interpolator_clone),
            (controller, controller_clone),
        ];
        e.register_double(target(SLOT_CREATE_CLONE), move |_, a| {
            clones
                .iter()
                .find(|(from, _)| *from == a[0])
                .unwrap()
                .1
                .into_ret()
        });
        e.mem.set_u32(this.addr() + 0x1c, 0x7f80_0001);
        let keys = object(&mut e);
        e.mem.set_u32(this.addr() + 0x20, keys);
        e.mem.set_u32(this.addr() + 0x24, 2);
        set_float(&mut e, this.addr() + 0x28, 1.5);
        set_float(&mut e, this.addr() + 0x2c, 2.5);
        set_float(&mut e, this.addr() + 0x30, 3.5);
        e.mem.set_u32(this.addr() + 0x5c, 0x7a30);
        e.mem.set_u16(this.addr() + 0x68, 7);
        let notes = e.mem.alloc(8);
        let first_note = object(&mut e);
        let second_note = object(&mut e);
        e.mem.set_u32(notes, first_note);
        e.mem.set_u32(notes + 4, second_note);
        e.mem.set_u32(this.addr() + 0x6c, notes);
        e.mem.set_u16(this.addr() + 0x70, 2);
        let process = e.mem.alloc(8);
        e.call(0x00a33570, &args![this, destination, process]);
        assert_eq!(
            calls_to(&e, OBJECT_COPY_MEMBERS),
            vec![vec![this.addr(), destination.addr(), process]]
        );
        assert_eq!(e.mem.u32(destination.addr() + 8), 0x7a00);
        assert_eq!(e.get(destination, NiControllerSequence::m_uiArraySize), 2);
        assert_eq!(e.get(destination, NiControllerSequence::m_uiArrayGrowBy), 4);
        let dest_items = e
            .get(destination, NiControllerSequence::m_pkInterpArray)
            .addr();
        let dest_tags = e
            .get(destination, NiControllerSequence::m_pkIDTagArray)
            .addr();
        assert_eq!(e.mem.u32(dest_items), interpolator_clone);
        assert_eq!(e.mem.u32(dest_items + 4), controller_clone);
        assert_eq!(e.mem.u8(dest_items + 0xd), 5);
        assert_eq!(e.mem.u8(dest_items + INTERP_ITEM_SIZE + 0xd), 9);
        assert_eq!(e.mem.u32(dest_tags + 8), 0x7a10);
        assert_eq!(e.mem.u32(dest_tags + ID_TAG_SIZE + 0x10), 0x7a20);
        assert_eq!(e.mem.u32(destination.addr() + 0x1c), 0x7fc0_0001);
        assert_eq!(e.mem.u32(destination.addr() + 0x20), keys);
        assert_eq!(e.mem.u32(destination.addr() + 0x24), 2);
        assert_eq!(float_at(&e, destination.addr() + 0x28), 1.5);
        assert_eq!(float_at(&e, destination.addr() + 0x2c), 2.5);
        assert_eq!(float_at(&e, destination.addr() + 0x30), 3.5);
        assert_eq!(e.mem.u32(destination.addr() + 0x5c), 0x7a30);
        assert_eq!(e.mem.u16(destination.addr() + 0x68), 7);
        assert_eq!(e.mem.u16(destination.addr() + 0x70), 2);
        let dest_notes = e.mem.u32(destination.addr() + 0x6c);
        assert_eq!(e.mem.u32(dest_notes), first_note);
        assert_eq!(e.mem.u32(dest_notes + 4), second_note);
        // The note array has one count word of 2 in front of it.
        assert_eq!(e.mem.u32(dest_notes - 4), 2);
    }

    #[test]
    fn fn_00a33570_clones_the_interpolator_of_a_sequence_with_an_owner() {
        let mut e = setup();
        copy_doubles(&mut e);
        e.register(OBJECT_CLONE_NEW, |_, a| (a[0] + 0x100).into_ret());
        let this = sequence(&mut e);
        let destination = sequence(&mut e);
        let (items, _) = set_items(&mut e, this, 1);
        let owner = object(&mut e);
        e.mem.set_u32(this.addr() + 0x40, owner);
        let interpolator = object(&mut e);
        e.mem.set_u32(items, interpolator);
        e.mem.set_u8(items + 0xc, 4);
        // The controller is not cloned: the clone item keeps none.
        let controller = object(&mut e);
        e.mem.set_u32(items + 4, controller);
        let process = e.mem.alloc(8);
        e.call(0x00a33570, &args![this, destination, process]);
        assert_eq!(
            calls_to(&e, OBJECT_CLONE_NEW),
            vec![vec![interpolator, process]]
        );
        let dest_items = e
            .get(destination, NiControllerSequence::m_pkInterpArray)
            .addr();
        assert_eq!(e.mem.u32(dest_items), interpolator + 0x100);
        assert_eq!(e.mem.u32(dest_items + 4), 0);
        assert_eq!(e.mem.u8(dest_items + 0xc), 4);
        // Without notes no note array is made.
        assert_eq!(e.mem.u32(destination.addr() + 0x6c), 0);
    }

    #[test]
    fn fn_00a33950_loads_an_item_of_a_new_file() {
        let mut e = setup();
        let links = Rc::new(RefCell::new(vec![0u32; 0]));
        let queue = links.clone();
        e.register_double(STREAM_RESOLVE_LINK_ID, move |_, _| {
            queue.borrow_mut().remove(0).into_ret()
        });
        read_queue(&mut e, &[0x2a]);
        let (stream, input, _) = stream_with(&mut e, READ, 0, 0x0a01_006f, 1);
        let item = e.mem.alloc(INTERP_ITEM_SIZE);
        let interpolator = object(&mut e);
        let controller = object(&mut e);
        *links.borrow_mut() = vec![interpolator, controller];
        e.call(0x00a33950, &args![item, stream]);
        assert_eq!(e.mem.u32(item), interpolator);
        assert_eq!(e.mem.u32(item + 4), controller);
        assert_eq!(e.mem.u8(item + 0xd), 0x2a);
        assert_eq!(count_of(&e, interpolator), 2);
        // One byte was read.
        assert_eq!(e.mem.u32(input + 4), 1);
    }

    #[test]
    fn fn_00a33950_skips_a_dropped_short_in_old_files_and_the_priority_without_a_bs_version() {
        let mut e = setup();
        let links = Rc::new(RefCell::new(vec![0u32, 0, 0]));
        let queue = links.clone();
        e.register_double(STREAM_RESOLVE_LINK_ID, move |_, _| {
            queue.borrow_mut().remove(0).into_ret()
        });
        read_queue(&mut e, &[0x1234]);
        let (stream, input, _) = stream_with(&mut e, READ, 0, 0x0a01_006e, 0);
        let item = e.mem.alloc(INTERP_ITEM_SIZE);
        e.mem.set_u8(item + 0xd, 0x77);
        e.call(0x00a33950, &args![item, stream]);
        // Three link ids were taken, two bytes read and dropped, the priority is kept.
        assert!(links.borrow().is_empty());
        assert_eq!(e.mem.u32(input + 4), 2);
        assert_eq!(e.mem.u8(item + 0xd), 0x77);
    }

    #[test]
    fn id_tag_load_binary_reads_c_strings_in_old_files() {
        let mut e = setup();
        e.register(STREAM_LOAD_C_STRING, |_, _| Ret::default());
        let (stream, _, _) = stream_with(&mut e, READ, 0, 0x0a01_0070, 0);
        let tag = e.mem.alloc(ID_TAG_SIZE);
        e.call(0x00a33a50, &args![tag, stream, 0u32]);
        let expected: Calls = (0..5)
            .map(|word| vec![stream.addr(), tag + 4 * word])
            .collect();
        assert_eq!(calls_to(&e, STREAM_LOAD_C_STRING), expected);
    }

    #[test]
    fn id_tag_load_binary_reads_fixed_strings_in_new_files() {
        let mut e = setup();
        e.register(STREAM_LOAD_FIXED_STRING, |_, _| Ret::default());
        let (stream, _, _) = stream_with(&mut e, READ, 0, 0x1401_0001, 0);
        let tag = e.mem.alloc(ID_TAG_SIZE);
        e.call(0x00a33a50, &args![tag, stream, 0u32]);
        let expected: Calls = (0..5)
            .map(|word| vec![stream.addr(), tag + 4 * word])
            .collect();
        assert_eq!(calls_to(&e, STREAM_LOAD_FIXED_STRING), expected);
    }

    #[test]
    fn id_tag_load_binary_reads_five_words_in_between() {
        let mut e = setup();
        let object = e.mem.alloc(8);
        e.register(STREAM_RESOLVE_LINK_ID, |_, _| 0x4321u32.into_ret());
        read_queue(&mut e, &[0x10, 0x20, 0x30, 0x40, 0x50]);
        let (stream, input, _) = stream_with(&mut e, READ, 0, 0x0a01_0071, 0);
        let tag = e.mem.alloc(ID_TAG_SIZE);
        e.call(0x00a33a50, &args![tag, stream, object]);
        assert_eq!(e.mem.u32(object), 0x4321);
        let words: Vec<u32> = (0..5).map(|word| e.mem.u32(tag + 4 * word)).collect();
        assert_eq!(words, vec![0x10, 0x20, 0x30, 0x40, 0x50]);
        assert_eq!(e.mem.u32(input + 4), 20);
    }

    #[test]
    fn save_binary_writes_the_sequence_in_order() {
        let mut e = setup();
        let log = Rc::new(RefCell::new(Vec::new()));
        e.register(OBJECT_STREAM_HOOK, |_, _| Ret::default());
        let events = log.clone();
        e.register_double(STREAM_SAVE_FIXED_STRING, move |_, a| {
            events.borrow_mut().push(format!("string {:#x}", a[1]));
            Ret::default()
        });
        let events = log.clone();
        e.register_double(target(SLOT_SAVE_LINK_ID), move |_, a| {
            events.borrow_mut().push(format!("link {:#x}", a[1]));
            Ret::default()
        });
        write_log(&mut e, &log);
        let (stream, _, output) = stream_with(&mut e, 0, WRITE, 0, 0);
        e.mem.set_u32(stream.addr(), OBJECT_VTABLE);
        let this = sequence(&mut e);
        let (items, tags) = set_items(&mut e, this, 1);
        e.set(this, NiControllerSequence::m_uiArrayGrowBy, 12);
        e.mem.set_u32(items, 0x1001);
        e.mem.set_u32(items + 4, 0x1002);
        e.mem.set_u8(items + 0xd, 5);
        e.mem.set_u32(this.addr() + 0x1c, 0x3f80_0000);
        e.mem.set_u32(this.addr() + 0x20, 0x1003);
        e.mem.set_u32(this.addr() + 0x24, 1);
        e.mem.set_u32(this.addr() + 0x28, 0x4000_0000);
        e.mem.set_u32(this.addr() + 0x2c, 0x4040_0000);
        e.mem.set_u32(this.addr() + 0x30, 0x4080_0000);
        e.mem.set_u32(this.addr() + 0x40, 0x1004);
        let notes = e.mem.alloc(8);
        e.mem.set_u32(notes, 0x1005);
        e.mem.set_u32(notes + 4, 0x1006);
        e.mem.set_u32(this.addr() + 0x6c, notes);
        e.mem.set_u16(this.addr() + 0x70, 2);
        e.call(0x00a33bf0, &args![this, stream]);
        let mut expected = vec![
            format!("string {:#x}", this.addr() + 8),
            "write 4 0x1".to_string(),
            "write 4 0xc".to_string(),
            "link 0x1001".to_string(),
            "link 0x1002".to_string(),
            "write 1 0x5".to_string(),
        ];
        for word in 0..5 {
            expected.push(format!("string {:#x}", tags + 4 * word));
        }
        expected.extend(
            [
                "write 4 0x3f800000",
                "link 0x1003",
                "write 4 0x1",
                "write 4 0x40000000",
                "write 4 0x40400000",
                "write 4 0x40800000",
                "link 0x1004",
            ]
            .map(String::from),
        );
        expected.push(format!("string {:#x}", this.addr() + 0x5c));
        expected.extend(["write 2 0x2", "link 0x1005", "link 0x1006"].map(String::from));
        assert_eq!(log.borrow().clone(), expected);
        // Every write advanced the position by its size.
        assert_eq!(e.mem.u32(output + 4), 4 + 4 + 1 + 4 * 5 + 2);
    }

    /// Doubles for the arrays `LoadBinary` makes: the allocator and the stream functions that load
    /// strings and link ids.
    fn load_doubles(e: &mut Engine) {
        map_fixed_strings(e);
        silent_interlocked(e);
        allocator_doubles(e);
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_READ_LINK_ID, |_, _| Ret::default());
        e.register(STREAM_LOAD_C_STRING, |e, a| {
            e.mem.set_u32(a[1], 0x7c00);
            Ret::default()
        });
        e.register(STREAM_LOAD_FIXED_STRING, |e, a| {
            e.mem.set_u32(a[1], 0x7d00);
            Ret::default()
        });
        e.register(STREAM_RESOLVE_LINK_ID, |_, _| 0u32.into_ret());
    }

    #[test]
    fn load_binary_reads_the_oldest_format() {
        let mut e = setup();
        load_doubles(&mut e);
        read_queue(&mut e, &[2]);
        let (stream, _, _) = stream_with(&mut e, READ, 0, 0x0a01_0067, 0);
        let this = sequence(&mut e);
        e.call(0x00a33e40, &args![this, stream]);
        assert_eq!(
            calls_to(&e, STREAM_LOAD_C_STRING)[..2],
            vec![
                vec![stream.addr(), this.addr() + 8],
                vec![stream.addr(), this.addr() + 0x5c]
            ]
        );
        assert_eq!(e.get(this, NiControllerSequence::m_uiArraySize), 2);
        assert_eq!(e.get(this, NiControllerSequence::m_uiArrayGrowBy), 12);
        // The tags' object names are C strings; a link id follows the name, the accum root name
        // and each item.
        let tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
        assert_eq!(e.mem.u32(tags), 0x7c00);
        assert_eq!(e.mem.u32(tags + ID_TAG_SIZE), 0x7c00);
        assert_eq!(calls_to(&e, STREAM_READ_LINK_ID).len(), 1 + 2);
        assert_eq!(calls_to(&e, STREAM_LOAD_C_STRING).len(), 4);
    }

    #[test]
    fn load_binary_reads_the_middle_format() {
        let mut e = setup();
        load_doubles(&mut e);
        read_queue(
            &mut e,
            &[
                1,
                9,
                4,
                11,
                12,
                13,
                14,
                15,
                0x3f80_0000,
                2,
                0x4000_0000,
                77,
                0x4040_0000,
                0x4080_0000,
                2,
            ],
        );
        let (stream, _, _) = stream_with(&mut e, READ, 0, 0x0a01_0072, 0x1e);
        let this = sequence(&mut e);
        e.call(0x00a33e40, &args![this, stream]);
        assert_eq!(
            calls_to(&e, STREAM_LOAD_C_STRING),
            vec![
                vec![stream.addr(), this.addr() + 8],
                vec![stream.addr(), this.addr() + 0x5c]
            ]
        );
        assert_eq!(e.get(this, NiControllerSequence::m_uiArraySize), 1);
        assert_eq!(e.get(this, NiControllerSequence::m_uiArrayGrowBy), 9);
        let items = e.get(this, NiControllerSequence::m_pkInterpArray).addr();
        let tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
        assert_eq!(e.mem.u8(items + 0xd), 4);
        let words: Vec<u32> = (0..5).map(|word| e.mem.u32(tags + 4 * word)).collect();
        assert_eq!(words, vec![11, 12, 13, 14, 15]);
        assert_eq!(e.mem.u32(this.addr() + 0x1c), 0x3f80_0000);
        assert_eq!(e.mem.u32(this.addr() + 0x24), 2);
        assert_eq!(e.mem.u32(this.addr() + 0x28), 0x4000_0000);
        assert_eq!(e.mem.u32(this.addr() + 0x2c), 0x4040_0000);
        assert_eq!(e.mem.u32(this.addr() + 0x30), 0x4080_0000);
        assert_eq!(e.mem.u32(this.addr() + 0x64), 0);
        assert_eq!(e.mem.u16(this.addr() + 0x70), 2);
        let notes = e.mem.u32(this.addr() + 0x6c);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR)
                .last()
                .unwrap()
                .clone(),
            vec![notes, 4, 2, POINTER_CONSTRUCTOR]
        );
        // Link ids: after the weight, after the end time, one per note; ids resolved: the item's two,
        // the tag's and the palette's.
        assert_eq!(calls_to(&e, STREAM_READ_LINK_ID).len(), 4);
        assert_eq!(calls_to(&e, STREAM_RESOLVE_LINK_ID).len(), 4);
    }

    #[test]
    fn load_binary_reads_the_newest_format() {
        let mut e = setup();
        load_doubles(&mut e);
        read_queue(
            &mut e,
            &[
                1,
                9,
                4,
                0x3f80_0000,
                1,
                0x4000_0000,
                0x4040_0000,
                0x4080_0000,
            ],
        );
        let (stream, _, _) = stream_with(&mut e, READ, 0, 0x1401_0001, 0x16);
        let this = sequence(&mut e);
        let palette = object(&mut e);
        e.mem.set_u32(this.addr() + 0x64, palette);
        e.call(0x00a33e40, &args![this, stream]);
        assert!(calls_to(&e, STREAM_LOAD_C_STRING).is_empty());
        let mut expected: Calls = vec![vec![stream.addr(), this.addr() + 8]];
        let tags = e.get(this, NiControllerSequence::m_pkIDTagArray).addr();
        expected.extend((0..5).map(|word| vec![stream.addr(), tags + 4 * word]));
        expected.push(vec![stream.addr(), this.addr() + 0x5c]);
        assert_eq!(calls_to(&e, STREAM_LOAD_FIXED_STRING), expected);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x7d00);
        assert_eq!(e.mem.u32(this.addr() + 0x24), 1);
        assert_eq!(e.mem.u32(this.addr() + 0x28), 0x4000_0000);
        // The palette is dropped; a stream version of 0x16 takes one link id and reads no notes.
        assert_eq!(e.mem.u32(this.addr() + 0x64), 0);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).contains(&vec![palette + 4]));
        assert_eq!(calls_to(&e, STREAM_READ_LINK_ID).len(), 3);
        assert_eq!(e.mem.u16(this.addr() + 0x70), 0);
    }

    #[test]
    fn fn_00a343c0_clears_the_destination_before_copying_into_it() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        e.map(INVALID_BLEND_INDEX, 1);
        allocator_doubles(&mut e);
        e.register(ID_TAG_RESET, |_, _| Ret::default());
        e.register(OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        let this = sequence(&mut e);
        let destination = sequence(&mut e);
        let items = element_block(&mut e, 2, INTERP_ITEM_SIZE);
        let tags = element_block(&mut e, 2, ID_TAG_SIZE);
        for word in 0..10 {
            e.mem.set_u32(tags + 4 * word, 0x9000);
        }
        e.set(destination, NiControllerSequence::m_uiArraySize, 2);
        e.set(
            destination,
            NiControllerSequence::m_pkInterpArray,
            Ptr::new(items),
        );
        e.set(
            destination,
            NiControllerSequence::m_pkIDTagArray,
            Ptr::new(tags),
        );
        let interpolator = object(&mut e);
        e.mem.set_u32(items + INTERP_ITEM_SIZE, interpolator);
        let process = e.mem.alloc(8);
        e.call(0x00a343c0, &args![this, destination, process]);
        assert_eq!(count_of(&e, interpolator), 0);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE)[..2],
            vec![
                vec![MEMORY_MANAGER, items - 4],
                vec![MEMORY_MANAGER, tags - 4]
            ]
        );
        assert_eq!(
            calls_to(&e, OBJECT_COPY_MEMBERS),
            vec![vec![this.addr(), destination.addr(), process]]
        );
    }

    #[test]
    fn fn_00a34450_stores_the_vtable_frees_the_elements_and_deletes_on_bit_0() {
        let mut e = setup();
        e.register(POINTER_ARRAY_FREE, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x5100);
        assert_eq!(e.call(0x00a34450, &args![array, 0u32]).u32(), array);
        assert_eq!(e.mem.u32(array), INTERP_CONTROLLER_ARRAY_VTABLE);
        assert_eq!(calls_to(&e, POINTER_ARRAY_FREE), vec![vec![0x5100]]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00a34450, &args![array, 1u32]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, array]]
        );
    }

    #[test]
    fn fn_00a34480_sets_the_controller_up_and_adds_it() {
        let mut e = setup();
        c_string_doubles(&mut e);
        map_fixed_strings(&mut e);
        let this = sequence(&mut e);
        e.set(this, NiControllerSequence::m_eCycleType, 2);
        set_float(&mut e, this.addr() + 0x28, 3.5);
        let controller = viewer_controller(&mut e, 0, "NiTransformController", 0);
        // The class chain: the controller's class, whose parent is NiInterpController.
        let record = e.mem.alloc(8);
        e.mem.set_u32(record + 4, NI_INTERP_CONTROLLER_RTTI);
        e.register_double(target(SLOT_GET_RTTI), move |_, _| record.into_ret());
        e.mem.set_u16(controller + 8, 0xffff);
        let text = string(&mut e, "Bip01");
        assert!(e.call(0x00a34480, &args![this, text, controller]).bool());
        assert_eq!(e.mem.u16(controller + 8), 0xffed);
        assert_eq!(float_at(&e, controller + 0xc), 3.5);
        // The text or the controller missing, or a class that is not an interpolator controller.
        assert!(!e.call(0x00a34480, &args![this, 0u32, controller]).bool());
        assert!(!e.call(0x00a34480, &args![this, text, 0u32]).bool());
        let end_of_chain = e.mem.alloc(8);
        e.register_double(target(SLOT_GET_RTTI), move |_, _| end_of_chain.into_ret());
        assert!(!e.call(0x00a34480, &args![this, text, controller]).bool());
    }

    #[test]
    fn create_clone_makes_a_sequence_and_copies_into_it() {
        let mut e = setup();
        copy_doubles(&mut e);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SEQUENCE_CONSTRUCT_EMPTY, |_, a| a[0].into_ret());
        let this = sequence(&mut e);
        let process = e.mem.alloc(8);
        let clone = e.call(0x00a344f0, &args![this, process]).u32();
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x74]]);
        assert_eq!(calls_to(&e, SEQUENCE_CONSTRUCT_EMPTY), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, OBJECT_COPY_MEMBERS),
            vec![vec![this.addr(), clone, process]]
        );
    }

    #[test]
    fn create_clone_copies_into_address_0_when_there_is_no_memory() {
        let mut e = setup();
        copy_doubles(&mut e);
        e.mem.map(0, 0x1000);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        let this = sequence(&mut e);
        let process = e.mem.alloc(8);
        assert_eq!(e.call(0x00a344f0, &args![this, process]).u32(), 0);
        assert_eq!(
            calls_to(&e, OBJECT_COPY_MEMBERS),
            vec![vec![this.addr(), 0, process]]
        );
    }

    /// The doubles `GetViewerStrings` needs: every viewer string function answers a marker (0x101 the
    /// class name, 0x102 the state text, 0x103 the array size, 0x104 a float, 0x105 the cycle type,
    /// 0x106 the item line), `SetAt` stores the string and makes the size `index + 1`, growing the array
    /// sets its capacity to 100.
    fn viewer_string_doubles(e: &mut Engine) {
        put_text(e, TEXT_STATE, "State");
        e.map(NI_CONTROLLER_SEQUENCE_RTTI, 4);
        e.set_global(NI_CONTROLLER_SEQUENCE_RTTI, 0x5000u32);
        e.register(OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x101u32.into_ret());
        e.register(MEMORY_ALLOCATE, |_, _| 0x102u32.into_ret());
        e.register(SPRINTF, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING_OV8, |_, _| 0x103u32.into_ret());
        e.register(GET_VIEWER_STRING_OV9, |_, _| 0x104u32.into_ret());
        e.register(TIME_CONTROLLER_GET_VIEWER_STRING, |_, _| {
            0x105u32.into_ret()
        });
        e.register(NI_SPRINTF, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING_OV11, |_, _| 0x106u32.into_ret());
        e.register(VIEWER_STRINGS_GROW, |e, a| {
            e.mem.set_u16(a[0] + 8, 100);
            Ret::default()
        });
        e.register(VIEWER_STRINGS_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], text);
            e.mem.set_u16(a[0] + 0xa, (a[1] + 1) as u16);
            Ret::default()
        });
    }

    /// A viewer-string array with the given capacity, growth step 3, no elements.
    fn viewer_array(e: &mut Engine, capacity: u16) -> (u32, u32) {
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x100);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u16(array + 8, capacity);
        e.mem.set_u16(array + 0xe, 3);
        (array, base)
    }

    #[test]
    fn get_viewer_strings_appends_every_field_and_the_item_lines() {
        let mut e = setup();
        viewer_string_doubles(&mut e);
        let this = sequence(&mut e);
        let (_, tags) = set_items(&mut e, this, 1);
        let (items, _) = (e.get(this, NiControllerSequence::m_pkInterpArray).addr(), 0);
        e.mem.set_u32(tags, 0x5100);
        e.mem.set_u32(tags + 8, 0x5200);
        e.mem.set_u8(items + 0xd, 7);
        e.set(this, NiControllerSequence::m_eState, 2);
        e.mem.set_u32(this.addr() + 0x24, 1);
        for (field, value) in [
            (0x1cu32, 1.0f32),
            (0x28, 2.0),
            (0x2c, 3.0),
            (0x30, 4.0),
            (0x34, 5.0),
            (0x38, 6.0),
            (0x3c, 7.0),
            (0x48, 8.0),
            (0x4c, 9.0),
            (0x50, 10.0),
        ] {
            set_float(&mut e, this.addr() + field, value);
        }
        let (array, base) = viewer_array(&mut e, 100);
        e.call(0x00a34540, &args![this, array]);
        assert_eq!(
            calls_to(&e, OBJECT_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), array]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x5000]]);
        assert_eq!(
            calls_to(&e, SPRINTF),
            vec![vec![0x102, TEXT_FORMAT_STATE, TEXT_STATE, STATE_NAMES[2]]]
        );
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_OV8),
            vec![vec![TEXT_ARRAY_SIZE, 1]]
        );
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_GET_VIEWER_STRING),
            vec![vec![TEXT_CYCLE_TYPE, 1]]
        );
        let floats: Vec<(u32, f32)> = calls_to(&e, GET_VIEWER_STRING_OV9)
            .iter()
            .map(|call| (call[0], f32::from_bits(call[1])))
            .collect();
        assert_eq!(
            floats,
            vec![
                (TEXT_WEIGHT, 1.0),
                (TEXT_FREQ, 2.0),
                (TEXT_BEGIN, 3.0),
                (TEXT_END, 4.0),
                (TEXT_LAST, 5.0),
                (TEXT_OFFSET, 8.0),
                (TEXT_START, 9.0),
                (TEXT_END, 10.0),
                (TEXT_WEIGHT_LAST, 6.0),
                (TEXT_LAST_SCALED, 7.0),
            ]
        );
        // The item line: "Target 0, " and "name, class, priority".
        let sprintf = calls_to(&e, NI_SPRINTF);
        assert_eq!(sprintf.len(), 2);
        assert_eq!(sprintf[0][1..], [0x80, TEXT_FORMAT_TARGET, 0]);
        assert_eq!(
            sprintf[1][1..],
            [0x100, TEXT_FORMAT_TARGET_LINE, 0x5100, 0x5200, 7]
        );
        assert_eq!(sprintf[1][0], sprintf[0][0] + 0x80);
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_OV11),
            vec![vec![sprintf[0][0], sprintf[1][0]]]
        );
        // Fifteen strings were stored in order; the item line was counted by the inline store.
        let stored: Vec<u32> = (0..15).map(|index| e.mem.u32(base + 4 * index)).collect();
        let mut expected = vec![0x101, 0x102, 0x103, 0x104, 0x105];
        expected.extend([0x104; 9]);
        expected.push(0x106);
        assert_eq!(stored, expected);
        assert_eq!(e.mem.u16(array + 0xa), 15);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert!(calls_to(&e, VIEWER_STRINGS_GROW).is_empty());
    }

    #[test]
    fn get_viewer_strings_grows_the_array_for_the_item_lines() {
        let mut e = setup();
        viewer_string_doubles(&mut e);
        let this = sequence(&mut e);
        set_items(&mut e, this, 1);
        let (array, base) = viewer_array(&mut e, 14);
        // Another string at the next slot, which the array already counts.
        e.register(VIEWER_STRINGS_GROW, |e, a| {
            e.mem.set_u16(a[0] + 8, 100);
            e.mem.set_u16(a[0] + 0xa, 30);
            Ret::default()
        });
        e.mem.set_u32(base + 4 * 14, 0x4000);
        e.call(0x00a34540, &args![this, array]);
        assert_eq!(calls_to(&e, VIEWER_STRINGS_GROW), vec![vec![array, 3 + 14]]);
        // The slot was set already: the count is unchanged, the pointer replaced.
        assert_eq!(e.mem.u32(base + 4 * 14), 0x106);
        assert_eq!(e.mem.u16(array + 0xa), 30);
        assert_eq!(e.mem.u16(array + 0xc), 0);
    }

    #[test]
    fn get_viewer_strings_counts_a_string_set_into_an_unset_slot_and_uncounts_a_null_one() {
        let mut e = setup();
        viewer_string_doubles(&mut e);
        let this = sequence(&mut e);
        set_items(&mut e, this, 1);
        e.register(VIEWER_STRINGS_GROW, |e, a| {
            e.mem.set_u16(a[0] + 8, 100);
            e.mem.set_u16(a[0] + 0xa, 30);
            Ret::default()
        });
        let (array, base) = viewer_array(&mut e, 14);
        e.call(0x00a34540, &args![this, array]);
        assert_eq!(e.mem.u32(base + 4 * 14), 0x106);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        // A null line (the double answers 0) replaces a set string.
        e.register(GET_VIEWER_STRING_OV11, |_, _| Ret::default());
        let (array, base) = viewer_array(&mut e, 14);
        e.mem.set_u32(base + 4 * 14, 0x4000);
        e.mem.set_u16(array + 0xc, 5);
        e.call(0x00a34540, &args![this, array]);
        assert_eq!(e.mem.u32(base + 4 * 14), 0);
        assert_eq!(e.mem.u16(array + 0xc), 4);
    }

    /// A source object for `00a349e0`: the name text, the count, the texts and controllers.
    fn kf_source(e: &mut Engine, texts: &[u32], controllers: &[u32], keys: u32) -> u32 {
        let source = e.mem.alloc(0x40);
        let name = string(e, "Walk");
        e.mem.set_u32(source + 8, name);
        e.mem.set_u16(source + 0x16, texts.len() as u16);
        let text_array = e.mem.alloc(4 * texts.len() as u32 + 4);
        let controller_array = e.mem.alloc(4 * controllers.len() as u32 + 4);
        for (index, text) in texts.iter().enumerate() {
            e.mem.set_u32(text_array + 4 * index as u32, *text);
        }
        for (index, controller) in controllers.iter().enumerate() {
            e.mem
                .set_u32(controller_array + 4 * index as u32, *controller);
        }
        e.mem.set_u32(source + 0x10, text_array);
        e.mem.set_u32(source + 0x20, controller_array);
        e.mem.set_u32(source + 0x2c, keys);
        source
    }

    #[test]
    fn fn_00a349e0_makes_a_sequence_from_a_source_object() {
        let mut e = setup();
        copy_doubles(&mut e);
        map_float_constants(&mut e);
        c_string_doubles(&mut e);
        fixed_string_by_length(&mut e);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        let controller = viewer_controller(&mut e, 0, "NiTransformController", 0);
        let record = e.mem.alloc(8);
        e.mem.set_u32(record + 4, NI_INTERP_CONTROLLER_RTTI);
        e.register_double(target(SLOT_GET_RTTI), move |_, _| record.into_ret());
        // Flags 0b110: bits 1 and 2 are 3, which is not a cycle type kept; flags 0b100: 2.
        e.mem.set_u16(controller + 8, 0b100);
        set_float(&mut e, controller + 0xc, 1.5);
        set_float(&mut e, controller + 0x14, 2.5);
        set_float(&mut e, controller + 0x18, 3.5);
        let text = string(&mut e, "Bip01");
        let keys = object(&mut e);
        let source = kf_source(&mut e, &[text, 0], &[controller, 0], keys);
        let out = e.mem.alloc(4);
        e.call(0x00a349e0, &args![source, out]);
        let sequence = e.mem.u32(out);
        assert_ne!(sequence, 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x74]]);
        assert_eq!(e.mem.u32(sequence), NI_CONTROLLER_SEQUENCE_VTABLE);
        // The name "Walk" became a fixed string, the size and growth are the count.
        assert_eq!(e.mem.u32(sequence + 8), 0x20000 + 4);
        assert_eq!(e.mem.u32(sequence + 0xc), 2);
        assert_eq!(e.mem.u32(sequence + 0x10), 2);
        assert_eq!(e.mem.u32(sequence + 0x24), 2);
        assert_eq!(float_at(&e, sequence + 0x28), 1.5);
        assert_eq!(float_at(&e, sequence + 0x2c), 2.5);
        assert_eq!(float_at(&e, sequence + 0x30), 3.5);
        assert_eq!(e.mem.u32(sequence + 0x20), keys);
        assert_eq!(e.mem.u32(sequence + 0x5c), 0x7800);
        // The controller's flags and frequency were set by 00a34480.
        assert_eq!(e.mem.u16(controller + 8), 4);
    }

    #[test]
    fn fn_00a349e0_keeps_the_cycle_type_of_odd_flags_at_zero() {
        let mut e = setup();
        copy_doubles(&mut e);
        map_float_constants(&mut e);
        c_string_doubles(&mut e);
        fixed_string_by_length(&mut e);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        let controller = viewer_controller(&mut e, 0, "NiTransformController", 0);
        let record = e.mem.alloc(8);
        e.register_double(target(SLOT_GET_RTTI), move |_, _| record.into_ret());
        e.mem.set_u16(controller + 8, 0b010);
        let text = string(&mut e, "Bip01");
        let source = kf_source(&mut e, &[text], &[controller], 0);
        let out = e.mem.alloc(4);
        e.call(0x00a349e0, &args![source, out]);
        let sequence = e.mem.u32(out);
        assert_eq!(e.mem.u32(sequence + 0x24), 0);
    }

    #[test]
    fn fn_00a35700_releases_and_clears_the_output_for_an_index_past_the_end() {
        let mut e = setup();
        let (stream, _, _) = stream_with(&mut e, 0, 0, 0x0a01_0068, 0);
        e.mem.set_u32(stream.addr() + 0x228, 2);
        let out = e.mem.alloc(4);
        let held = object(&mut e);
        e.mem.set_u32(out, held);
        e.call(0x00a35700, &args![stream, 2u32, out]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(count_of(&e, held), 0);
        // Nothing held: nothing happens.
        e.call(0x00a35700, &args![stream, 5u32, out]);
        assert_eq!(calls_to(&e, target(SLOT_DELETE_THIS)).len(), 1);
    }

    #[test]
    fn fn_00a35700_takes_the_top_object_as_a_sequence_in_new_files() {
        let mut e = setup();
        e.register(OBJECT_CAST, |_, a| a[0].into_ret());
        e.register(POINTER_SLOT_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            if a[1] != 0 {
                e.mem.set_u32(a[1] + 4, e.mem.u32(a[1] + 4) + 1);
            }
            Ret::default()
        });
        let (stream, _, _) = stream_with(&mut e, 0, 0, 0x0500_0000, 0);
        let first = object(&mut e);
        let top_objects = e.mem.alloc(8);
        e.mem.set_u32(top_objects, first);
        e.mem.set_u32(stream.addr() + 0x220, top_objects);
        e.mem.set_u32(stream.addr() + 0x228, 2);
        let out = e.mem.alloc(4);
        e.call(0x00a35700, &args![stream, 0u32, out]);
        assert_eq!(
            calls_to(&e, OBJECT_CAST),
            vec![vec![first, NI_CONTROLLER_SEQUENCE_RTTI]]
        );
        // One count from the slot assignment, one from `out`, the temporary one released.
        assert_eq!(e.mem.u32(out), first);
        assert_eq!(count_of(&e, first), 2);
        // An empty slot is not cast and leaves `out` empty.
        e.call(0x00a35700, &args![stream, 1u32, out]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(calls_to(&e, OBJECT_CAST).len(), 1);
        assert_eq!(count_of(&e, first), 1);
    }

    #[test]
    fn fn_00a35700_makes_a_sequence_from_an_old_style_top_object() {
        let mut e = setup();
        copy_doubles(&mut e);
        map_float_constants(&mut e);
        c_string_doubles(&mut e);
        fixed_string_by_length(&mut e);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        let source = kf_source(&mut e, &[], &[], 0);
        e.register_double(STREAM_GET_OBJECT_AT, move |e, a| {
            e.mem.set_u32(a[2], source);
            e.mem.set_u32(source + 4, 2);
            Ret::default()
        });
        let (stream, _, _) = stream_with(&mut e, 0, 0, 0x0401_0000, 0);
        e.mem.set_u32(stream.addr() + 0x228, 1);
        let out = e.mem.alloc(4);
        e.call(0x00a35700, &args![stream, 0u32, out]);
        assert_eq!(calls_to(&e, STREAM_GET_OBJECT_AT).len(), 1);
        assert_eq!(calls_to(&e, STREAM_GET_OBJECT_AT)[0][1], 0);
        let sequence = e.mem.u32(out);
        assert_ne!(sequence, 0);
        assert_eq!(e.mem.u32(sequence), NI_CONTROLLER_SEQUENCE_VTABLE);
        // The fetched object's extra count was released.
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).contains(&vec![source + 4]));
    }

    #[test]
    fn add_unique_appends_an_element_and_grows_the_set_when_it_is_full() {
        let mut e = setup();
        e.register(SET_REALLOC, |e, a| {
            let base = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0]);
            for index in 0..e.mem.u32(a[0] + 8) {
                let value = e.mem.u32(old + 4 * index);
                e.mem.set_u32(base + 4 * index, value);
            }
            e.mem.set_u32(a[0], base);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        let set = e.mem.alloc(0xc);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, 0x1111);
        // An empty set with no storage grows to 1.
        e.call(0x00e7ec00, &args![set, element]);
        assert_eq!(calls_to(&e, SET_REALLOC), vec![vec![set, 1]]);
        assert_eq!(e.mem.u32(set + 8), 1);
        assert_eq!(e.mem.u32(e.mem.u32(set)), 0x1111);
        // The same element again: nothing.
        e.call(0x00e7ec00, &args![set, element]);
        assert_eq!(e.mem.u32(set + 8), 1);
        assert_eq!(calls_to(&e, SET_REALLOC).len(), 1);
        // A full set doubles.
        e.mem.set_u32(element, 0x2222);
        e.call(0x00e7ec00, &args![set, element]);
        assert_eq!(calls_to(&e, SET_REALLOC)[1], vec![set, 2]);
        e.mem.set_u32(element, 0x3333);
        e.call(0x00e7ec00, &args![set, element]);
        assert_eq!(calls_to(&e, SET_REALLOC)[2], vec![set, 4]);
        // Room left: no growth.
        e.mem.set_u32(element, 0x4444);
        e.call(0x00e7ec00, &args![set, element]);
        assert_eq!(calls_to(&e, SET_REALLOC).len(), 3);
        let stored: Vec<u32> = (0..4)
            .map(|index| e.mem.u32(e.mem.u32(set) + 4 * index))
            .collect();
        assert_eq!(stored, vec![0x1111, 0x2222, 0x3333, 0x4444]);
        assert_eq!(e.mem.u32(set + 8), 4);
    }

    /// A sequence for the `Update` tests: frequency 1.0, 0 to 10, clamped, weight 1.0, start time 0.0 and
    /// end time 4.0, no offset, the destination frame unset, one item whose blend interpolator has
    /// the element array at `+0x14`. Answers the sequence and the element array.
    fn update_sequence(e: &mut Engine, state: u32) -> (Ptr<NiControllerSequence>, u32) {
        map_float_constants(e);
        e.map(NEGATIVE_FLT_MAX_DOUBLE, 8);
        let negative_max = (-f64::from(f32::MAX)).to_bits();
        e.mem.set_u32(NEGATIVE_FLT_MAX_DOUBLE, negative_max as u32);
        e.mem
            .set_u32(NEGATIVE_FLT_MAX_DOUBLE + 4, (negative_max >> 32) as u32);
        let this = sequence(e);
        let owner = e.mem.alloc(0x80);
        let listener = e.mem.alloc(0x10);
        e.mem.set_u32(owner + 0x64, listener);
        e.mem.set_u32(this.addr() + 0x40, owner);
        let (items, _) = set_items(e, this, 1);
        let blend = e.mem.alloc(0x40);
        let elements = e.mem.alloc(0x30);
        e.mem.set_u32(blend + 0x14, elements);
        e.mem.set_u32(items + 8, blend);
        e.set(this, NiControllerSequence::m_eState, state);
        e.set(this, NiControllerSequence::m_eCycleType, 1);
        for (field, value) in [
            (0x1cu32, 1.0f32),
            (0x28, 1.0),
            (0x2c, 0.0),
            (0x30, 10.0),
            (0x34, 0.0),
            (0x38, 0.0),
            (0x3c, 0.0),
            (0x48, 0.0),
            (0x4c, 0.0),
            (0x50, 4.0),
            (0x54, -f32::MAX),
        ] {
            set_float(e, this.addr() + field, value);
        }
        (this, elements)
    }

    fn update(e: &mut Engine, this: Ptr<NiControllerSequence>, time: f32, update: bool) {
        e.call(0x00a34ba0, &args![this, time.to_bits(), u32::from(update)]);
    }

    #[test]
    fn update_of_an_inactive_sequence_only_resets_the_note_index() {
        let mut e = setup();
        let (this, _) = update_sequence(&mut e, 0);
        e.set(this, NiControllerSequence::ssCurAnimNIdx, 5);
        update(&mut e, this, 1.0, true);
        assert_eq!(e.get(this, NiControllerSequence::ssCurAnimNIdx), -1);
        assert_eq!(float_at(&e, this.addr() + 0x4c), 0.0);
    }

    #[test]
    fn update_starts_the_unset_times_from_the_time() {
        let mut e = setup();
        // State 7 does nothing else.
        let (this, _) = update_sequence(&mut e, 7);
        set_float(&mut e, this.addr() + 0x48, -f32::MAX);
        set_float(&mut e, this.addr() + 0x4c, -f32::MAX);
        update(&mut e, this, 3.0, false);
        assert_eq!(float_at(&e, this.addr() + 0x48), -3.0);
        assert_eq!(float_at(&e, this.addr() + 0x4c), 3.0);
        assert_eq!(float_at(&e, this.addr() + 0x50), 7.0);
    }

    #[test]
    fn update_of_an_animating_sequence_stores_the_scaled_time_and_the_weights() {
        let mut e = setup();
        let (this, elements) = update_sequence(&mut e, 1);
        set_float(&mut e, this.addr() + 0x3c, 1.25);
        set_float(&mut e, this.addr() + 0x1c, 0.5);
        update(&mut e, this, 2.0, true);
        // The start time is the last scaled time of before the update.
        assert_eq!(float_at(&e, this.addr() + 0x4c), 1.25);
        // The scaled time of 2.0 (frequency 1, last time 0): 2.0; the weight 0.5 times the second
        // ratio 1.0, the first ratio 1.0 and the scaled time went to the blend interpolator.
        assert_eq!(float_at(&e, elements + 4), 0.5);
        assert_eq!(float_at(&e, elements + 0x10), 1.0);
        assert_eq!(float_at(&e, elements + 0x14), 2.0);
        assert_eq!(float_at(&e, this.addr() + 0x3c), 2.0);
        assert_eq!(float_at(&e, this.addr() + 0x34), 2.0);
        // Without the update flag nothing else happens.
        update(&mut e, this, 3.0, false);
        assert_eq!(float_at(&e, this.addr() + 0x3c), 2.0);
    }

    #[test]
    fn update_of_an_easing_in_sequence_ratios_the_time_within_the_ease() {
        let mut e = setup();
        let (this, elements) = update_sequence(&mut e, 2);
        update(&mut e, this, 1.0, true);
        // (1 - 0) / (4 - 0) goes to the weight ratio (the second word of the call).
        assert_eq!(float_at(&e, elements + 0x10), 0.25);
        assert_eq!(float_at(&e, elements + 4), 1.0);
        // Before the start time the time is held at the start.
        set_float(&mut e, this.addr() + 0x4c, 2.0);
        update(&mut e, this, 1.0, true);
        assert_eq!(float_at(&e, elements + 0x10), 0.0);
        // At the end time the sequence is animating, from the start time 0.
        update(&mut e, this, 4.0, false);
        assert_eq!(e.get(this, NiControllerSequence::m_eState), 1);
        assert_eq!(float_at(&e, this.addr() + 0x4c), 0.0);
    }

    #[test]
    fn update_of_an_easing_out_sequence_deactivates_at_the_end_time() {
        let mut e = setup();
        e.register(target(SLOT_DEACTIVATE), |_, _| Ret::default());
        let (this, elements) = update_sequence(&mut e, 3);
        update(&mut e, this, 1.0, true);
        // (4 - 1) / (4 - 0)
        assert_eq!(float_at(&e, elements + 0x10), 0.75);
        assert!(calls_to(&e, target(SLOT_DEACTIVATE)).is_empty());
        update(&mut e, this, 4.0, true);
        assert_eq!(
            calls_to(&e, target(SLOT_DEACTIVATE)),
            vec![vec![this.addr(), 0, 0]]
        );
    }

    #[test]
    fn update_of_a_transition_source_ratios_the_second_weight_and_deactivates_with_the_flag() {
        let mut e = setup();
        e.register(target(SLOT_DEACTIVATE), |_, _| Ret::default());
        let (this, elements) = update_sequence(&mut e, 4);
        update(&mut e, this, 1.0, true);
        assert_eq!(float_at(&e, elements + 4), 0.75);
        assert_eq!(float_at(&e, elements + 0x10), 1.0);
        update(&mut e, this, 5.0, true);
        assert_eq!(
            calls_to(&e, target(SLOT_DEACTIVATE)),
            vec![vec![this.addr(), 0, 1]]
        );
    }

    #[test]
    fn update_of_a_transition_destination_ends_in_the_destination_frame() {
        let mut e = setup();
        let (this, elements) = update_sequence(&mut e, 5);
        update(&mut e, this, 1.0, true);
        // (1 - 0) / (4 - 0) for the second ratio: 0.25 times the weight 1.0.
        assert_eq!(float_at(&e, elements + 4), 0.25);
        set_float(&mut e, this.addr() + 0x54, 6.0);
        update(&mut e, this, 4.0, false);
        // The offset is the destination frame less the time, the frame is unset again.
        assert_eq!(float_at(&e, this.addr() + 0x48), 2.0);
        assert_eq!(float_at(&e, this.addr() + 0x54), -f32::MAX);
        assert_eq!(e.get(this, NiControllerSequence::m_eState), 1);
        // Without a destination frame the offset stays.
        e.set(this, NiControllerSequence::m_eState, 5);
        set_float(&mut e, this.addr() + 0x48, 9.0);
        update(&mut e, this, 4.0, false);
        assert_eq!(float_at(&e, this.addr() + 0x48), 9.0);
    }

    #[test]
    fn update_of_a_morph_source_matches_the_partner_frame_and_becomes_a_transition_source() {
        let mut e = setup();
        let (partner, subject) = morph_pair(
            &mut e,
            &[(20.0, "morph:a"), (40.0, "morph:b")],
            &[(2.0, "morph:a"), (6.0, "morph:b")],
        );
        let (this, _) = update_sequence(&mut e, 6);
        let keys = e.get(subject, NiControllerSequence::m_spTextKeys);
        e.set(this, NiControllerSequence::m_spTextKeys, keys);
        e.set(
            this,
            NiControllerSequence::m_pkPartnerSequence,
            Ptr::new(partner.addr()),
        );
        e.register(target(SLOT_DEACTIVATE), |_, _| Ret::default());
        set_float(&mut e, this.addr() + 0x50, 10.0);
        update(&mut e, this, 4.0, false);
        // The partner's matching frame is 30 (its keys at 20 and 40, halfway), its offset
        // becomes 30 - 4.
        assert_eq!(float_at(&e, partner.addr() + 0x48), 26.0);
        assert_eq!(e.get(this, NiControllerSequence::m_eState), 4);
        // At the end the transition is over: Deactivate(0.0, 1).
        update(&mut e, this, 10.0, true);
        assert_eq!(
            calls_to(&e, target(SLOT_DEACTIVATE)),
            vec![vec![this.addr(), 0, 1]]
        );
    }

    #[test]
    fn update_uses_the_destination_frame_as_the_time_when_there_is_one() {
        let mut e = setup();
        let (this, elements) = update_sequence(&mut e, 1);
        set_float(&mut e, this.addr() + 0x54, 3.0);
        update(&mut e, this, 1.0, true);
        // The scaled time of the frame 3.0.
        assert_eq!(float_at(&e, elements + 0x14), 3.0);
    }

    #[test]
    fn update_adds_the_offset_to_the_time() {
        let mut e = setup();
        let (this, elements) = update_sequence(&mut e, 1);
        set_float(&mut e, this.addr() + 0x48, 2.0);
        update(&mut e, this, 1.0, true);
        assert_eq!(float_at(&e, elements + 0x14), 3.0);
    }

    #[test]
    fn update_follows_a_partner_with_the_matching_morph_frame() {
        let mut e = setup();
        let (partner, subject) = morph_pair(
            &mut e,
            &[(20.0, "morph:a"), (40.0, "morph:b")],
            &[(2.0, "morph:a"), (6.0, "morph:b")],
        );
        let (this, elements) = update_sequence(&mut e, 1);
        let keys = e.get(subject, NiControllerSequence::m_spTextKeys);
        e.set(this, NiControllerSequence::m_spTextKeys, keys);
        e.set(
            this,
            NiControllerSequence::m_pkPartnerSequence,
            Ptr::new(partner.addr()),
        );
        // The partner is inactive, so updating it only resets its note index; its offset is 3.
        e.set(partner, NiControllerSequence::ssCurAnimNIdx, 4);
        set_float(&mut e, partner.addr() + 0x48, 3.0);
        set_float(&mut e, partner.addr() + 0x34, 9.0);
        update(&mut e, this, 1.0, true);
        assert_eq!(e.get(partner, NiControllerSequence::ssCurAnimNIdx), -1);
        // The partner's offset time is 1 + 3 = 4, its scaled time is 0 (clamped), a frame before its key
        // at 20: against this sequence's keys at 2 and 6 that makes 6 + 0.75 * 6, 10.5, and 0.5 once it is
        // brought back into range.
        assert_eq!(float_at(&e, elements + 0x14), 0.5);
    }

    #[test]
    fn update_announces_the_notes_that_the_time_has_passed() {
        let mut e = setup();
        e.register(LISTENER_NOTIFY, |_, _| Ret::default());
        let (this, _) = update_sequence(&mut e, 1);
        let listener = e.mem.u32(e.mem.u32(this.addr() + 0x40) + 0x64);
        let notes = e.mem.alloc(12);
        let note_objects: Vec<u32> = [0.5f32, 1.5, 2.5]
            .iter()
            .map(|time| {
                let note = e.mem.alloc(0x20);
                set_float(&mut e, note + 0xc, *time);
                note
            })
            .collect();
        for (index, note) in note_objects.iter().enumerate() {
            e.mem.set_u32(notes + 4 * index as u32, *note);
        }
        e.set(this, NiControllerSequence::spAnimNotesA, Ptr::new(notes));
        e.set(this, NiControllerSequence::usNumNotes, 3);
        e.set(this, NiControllerSequence::ssCurAnimNIdx, -1);
        update(&mut e, this, 2.0, true);
        assert_eq!(
            calls_to(&e, LISTENER_NOTIFY),
            vec![
                vec![listener, note_objects[0]],
                vec![listener, note_objects[1]]
            ]
        );
        assert_eq!(e.get(this, NiControllerSequence::ssCurAnimNIdx), 1);
        // Updating again to a later time announces the last note.
        update(&mut e, this, 3.0, true);
        assert_eq!(calls_to(&e, LISTENER_NOTIFY).len(), 3);
        assert_eq!(e.get(this, NiControllerSequence::ssCurAnimNIdx), 2);
        // A scaled time that went down starts the notes again.
        set_float(&mut e, this.addr() + 0x3c, 10.0);
        update(&mut e, this, 1.0, true);
        assert_eq!(calls_to(&e, LISTENER_NOTIFY).len(), 4);
        assert_eq!(e.get(this, NiControllerSequence::ssCurAnimNIdx), 0);
    }

    fn deactivate(
        e: &mut Engine,
        this: Ptr<NiControllerSequence>,
        time: f32,
        transition: bool,
    ) -> bool {
        e.call(
            0x00a35030,
            &args![this, time.to_bits(), u32::from(transition)],
        )
        .bool()
    }

    #[test]
    fn deactivate_of_an_inactive_sequence_answers_false() {
        let mut e = setup();
        let (this, _) = update_sequence(&mut e, 0);
        assert!(!deactivate(&mut e, this, 1.0, false));
        assert_eq!(float_at(&e, this.addr() + 0x4c), 0.0);
    }

    #[test]
    fn deactivate_with_a_time_starts_the_ease_out() {
        let mut e = setup();
        let (this, _) = update_sequence(&mut e, 1);
        assert!(deactivate(&mut e, this, 2.5, false));
        assert_eq!(e.get(this, NiControllerSequence::m_eState), 3);
        assert_eq!(float_at(&e, this.addr() + 0x4c), -f32::MAX);
        assert_eq!(float_at(&e, this.addr() + 0x50), 2.5);
        assert!(deactivate(&mut e, this, 1.0, true));
        assert_eq!(e.get(this, NiControllerSequence::m_eState), 4);
    }

    #[test]
    fn deactivate_without_a_time_stops_the_sequence_at_once() {
        let mut e = setup();
        e.register(SEQUENCE_CLEAR_BLEND, |_, _| Ret::default());
        e.register(MANAGER_DEACTIVATE_SOURCES, |_, _| Ret::default());
        let (this, _) = update_sequence(&mut e, 1);
        let partner = object(&mut e);
        e.mem.set_u32(this.addr() + 0x58, partner);
        // The offset gains (weightedLast / frequency) - lastTime: 6 / 2 - 1.
        set_float(&mut e, this.addr() + 0x28, 2.0);
        set_float(&mut e, this.addr() + 0x38, 6.0);
        set_float(&mut e, this.addr() + 0x34, 1.0);
        set_float(&mut e, this.addr() + 0x48, 0.5);
        set_float(&mut e, this.addr() + 0x54, 3.0);
        assert!(deactivate(&mut e, this, 0.0, false));
        assert_eq!(float_at(&e, this.addr() + 0x48), 2.5);
        assert_eq!(float_at(&e, this.addr() + 0x54), -f32::MAX);
        assert_eq!(e.get(this, NiControllerSequence::m_eState), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x58), 0);
        assert_eq!(calls_to(&e, SEQUENCE_CLEAR_BLEND), vec![vec![this.addr()]]);
        assert!(calls_to(&e, MANAGER_DEACTIVATE_SOURCES).is_empty());
    }

    #[test]
    fn deactivate_keeps_the_offset_without_a_last_time_and_tells_the_owner_of_a_transition() {
        let mut e = setup();
        e.register(SEQUENCE_CLEAR_BLEND, |_, _| Ret::default());
        e.register(MANAGER_DEACTIVATE_SOURCES, |_, _| Ret::default());
        let (this, _) = update_sequence(&mut e, 5);
        let owner = e.mem.u32(this.addr() + 0x40);
        set_float(&mut e, this.addr() + 0x34, -f32::MAX);
        set_float(&mut e, this.addr() + 0x48, 0.5);
        // A NaN time is not positive: the sequence stops at once.
        assert!(deactivate(&mut e, this, f32::NAN, false));
        assert_eq!(float_at(&e, this.addr() + 0x48), 0.5);
        assert_eq!(
            calls_to(&e, MANAGER_DEACTIVATE_SOURCES),
            vec![vec![owner, 0]]
        );
        assert_eq!(e.get(this, NiControllerSequence::m_eState), 0);
    }

    #[test]
    fn deactivate_of_a_transition_destination_without_an_owner_skips_the_owner() {
        let mut e = setup();
        e.register(SEQUENCE_CLEAR_BLEND, |_, _| Ret::default());
        let (this, _) = update_sequence(&mut e, 5);
        e.mem.set_u32(this.addr() + 0x40, 0);
        set_float(&mut e, this.addr() + 0x34, -f32::MAX);
        assert!(deactivate(&mut e, this, -1.0, false));
        assert!(calls_to(&e, MANAGER_DEACTIVATE_SOURCES).is_empty());
    }

    /// Gives the owner of `this` an active set with room for 8 sequences.
    fn active_set(e: &mut Engine, this: Ptr<NiControllerSequence>) {
        let owner = e.mem.u32(this.addr() + 0x40);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0x44, base);
        e.mem.set_u32(owner + 0x48, 8);
    }

    /// The sequences in the active set of the owner of `this`.
    fn active_elements(e: &Engine, this: Ptr<NiControllerSequence>) -> Vec<u32> {
        let owner = e.mem.u32(this.addr() + 0x40);
        let base = e.mem.u32(owner + 0x44);
        (0..e.mem.u32(owner + 0x4c))
            .map(|index| e.mem.u32(base + 4 * index))
            .collect()
    }

    fn activate(
        e: &mut Engine,
        this: Ptr<NiControllerSequence>,
        time_sync: Ptr<NiControllerSequence>,
        ease_in: f32,
        transition: bool,
        start_over: bool,
    ) -> bool {
        e.call(
            0x00a34f20,
            &args![
                this,
                3u32,
                u32::from(start_over),
                0.5f32.to_bits(),
                ease_in.to_bits(),
                time_sync,
                u32::from(transition)
            ],
        )
        .bool()
    }

    #[test]
    fn activate_starts_a_sequence_with_an_ease_in() {
        let mut e = setup();
        let (this, _) = update_sequence(&mut e, 0);
        set_items(&mut e, this, 0);
        active_set(&mut e, this);
        e.mem.set_u32(this.addr() + 0x58, 0x9999);
        assert!(activate(&mut e, this, Ptr::NULL, 2.0, true, true));
        assert_eq!(e.get(this, NiControllerSequence::m_eState), 5);
        assert_eq!(e.mem.u32(this.addr() + 0x58), 0);
        assert_eq!(float_at(&e, this.addr() + 0x1c), 0.5);
        assert_eq!(float_at(&e, this.addr() + 0x4c), -f32::MAX);
        assert_eq!(float_at(&e, this.addr() + 0x50), 2.0);
        assert_eq!(float_at(&e, this.addr() + 0x48), -f32::MAX);
        assert_eq!(float_at(&e, this.addr() + 0x34), -f32::MAX);
        assert_eq!(active_elements(&e, this), vec![this.addr()]);
        // Not a transition: the state 2.
        e.set(this, NiControllerSequence::m_eState, 0);
        set_float(&mut e, this.addr() + 0x48, 7.0);
        assert!(activate(&mut e, this, Ptr::NULL, 2.0, false, false));
        assert_eq!(e.get(this, NiControllerSequence::m_eState), 2);
        assert_eq!(float_at(&e, this.addr() + 0x48), 7.0);
    }

    #[test]
    fn activate_without_a_positive_ease_in_goes_straight_to_animating() {
        let mut e = setup();
        let (this, _) = update_sequence(&mut e, 0);
        set_items(&mut e, this, 0);
        active_set(&mut e, this);
        assert!(activate(&mut e, this, Ptr::NULL, 0.0, true, false));
        assert_eq!(e.get(this, NiControllerSequence::m_eState), 1);
        assert_eq!(float_at(&e, this.addr() + 0x4c), 0.0);
        // A NaN ease-in is not positive either.
        e.set(this, NiControllerSequence::m_eState, 0);
        set_float(&mut e, this.addr() + 0x4c, 3.0);
        assert!(activate(&mut e, this, Ptr::NULL, f32::NAN, true, false));
        assert_eq!(e.get(this, NiControllerSequence::m_eState), 1);
        assert_eq!(float_at(&e, this.addr() + 0x4c), 0.0);
    }

    #[test]
    fn activate_refuses_an_active_sequence() {
        let mut e = setup();
        let (this, _) = update_sequence(&mut e, 1);
        assert!(!activate(&mut e, this, Ptr::NULL, 1.0, false, false));
        assert_eq!(e.mem.u32(this.addr() + 0x58), 0);
    }

    #[test]
    fn activate_follows_a_time_sync_sequence_with_matching_morph_keys() {
        let mut e = setup();
        let (this, other) = morph_pair(&mut e, &[(20.0, "morph:a")], &[(2.0, "morph:a")]);
        e.map(NEGATIVE_FLT_MAX_FLOAT, 4);
        set_float(&mut e, NEGATIVE_FLT_MAX_FLOAT, -f32::MAX);
        let owner = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 0x40, owner);
        active_set(&mut e, this);
        let chain_ok = Rc::new(RefCell::new(true));
        let answer = chain_ok.clone();
        e.register_double(SEQUENCE_PARTNER_CHAIN_CHECK, move |_, _| {
            u32::from(*answer.borrow()).into_ret()
        });
        let partner = sequence(&mut e);
        e.set(
            other,
            NiControllerSequence::m_pkPartnerSequence,
            Ptr::new(partner.addr()),
        );
        assert!(activate(&mut e, this, other, 0.0, false, false));
        assert_eq!(e.mem.u32(this.addr() + 0x58), other.addr());
        assert_eq!(
            calls_to(&e, SEQUENCE_PARTNER_CHAIN_CHECK),
            vec![vec![this.addr(), partner.addr()]]
        );
        // The chain check refuses, and the partner being this sequence refuses.
        e.set(this, NiControllerSequence::m_eState, 0);
        *chain_ok.borrow_mut() = false;
        assert!(!activate(&mut e, this, other, 0.0, false, false));
        assert_eq!(e.mem.u32(this.addr() + 0x58), 0);
        e.set(
            other,
            NiControllerSequence::m_pkPartnerSequence,
            Ptr::new(this.addr()),
        );
        assert!(!activate(&mut e, this, other, 0.0, false, false));
        assert_eq!(calls_to(&e, SEQUENCE_PARTNER_CHAIN_CHECK).len(), 2);
    }

    #[test]
    fn activate_refuses_a_time_sync_sequence_whose_morph_keys_differ() {
        let mut e = setup();
        let (this, other) = morph_pair(&mut e, &[(20.0, "morph:a")], &[(2.0, "morph:b")]);
        assert!(!activate(&mut e, this, other, 0.0, false, false));
        assert_eq!(e.mem.u32(this.addr() + 0x58), 0);
    }

    fn cross_fade_fixture(
        e: &mut Engine,
    ) -> (Ptr<NiControllerSequence>, Ptr<NiControllerSequence>) {
        let (this, _) = update_sequence(e, 0);
        set_items(e, this, 0);
        let other = sequence(e);
        set_items(e, other, 0);
        let owner = e.mem.alloc(0x80);
        e.mem.set_u32(other.addr() + 0x40, owner);
        set_float(e, other.addr() + 0x28, 4.0);
        active_set(e, this);
        active_set(e, other);
        e.register(target(SLOT_DEACTIVATE), |_, _| Ret::default());
        (this, other)
    }

    /// Gives the two sequences the same morph keys, so that `00a34f20` accepts one as the time sync
    /// sequence of the other.
    fn matching_morph_keys(
        e: &mut Engine,
        this: Ptr<NiControllerSequence>,
        other: Ptr<NiControllerSequence>,
    ) {
        string_compare_doubles(e);
        e.register(SEQUENCE_PARTNER_CHAIN_CHECK, |_, _| 1u32.into_ret());
        for sequence in [this, other] {
            let text = string(e, "morph:a");
            set_text_keys(e, sequence, &[(0.0, text)]);
        }
    }

    #[test]
    fn fn_00a350d0_cross_fades_this_sequence_into_the_other() {
        let mut e = setup();
        let (this, other) = cross_fade_fixture(&mut e);
        set_float(&mut e, this.addr() + 0x3c, 6.0);
        set_float(&mut e, this.addr() + 0x28, 2.0);
        assert!(e
            .call(
                0x00a350d0,
                &args![
                    this,
                    other,
                    1.5f32.to_bits(),
                    8.0f32.to_bits(),
                    3u32,
                    0.25f32.to_bits(),
                    0.75f32.to_bits(),
                    0u32
                ],
            )
            .bool());
        assert_eq!(float_at(&e, this.addr() + 0x1c), 0.25);
        assert_eq!(e.get(this, NiControllerSequence::m_eState), 1);
        assert_eq!(float_at(&e, this.addr() + 0x4c), 0.0);
        assert_eq!(float_at(&e, this.addr() + 0x34), -f32::MAX);
        // The other sequence was activated with a transition and the ease-in of 1.5.
        assert_eq!(e.get(other, NiControllerSequence::m_eState), 5);
        assert_eq!(float_at(&e, other.addr() + 0x1c), 0.75);
        assert_eq!(float_at(&e, other.addr() + 0x50), 1.5);
        // This sequence's Deactivate ran with (0.0, 1) first and with (ease-in, 1) at the end.
        assert_eq!(
            calls_to(&e, target(SLOT_DEACTIVATE)),
            vec![
                vec![this.addr(), 0, 1],
                vec![this.addr(), 1.5f32.to_bits(), 1]
            ]
        );
        // Destination frames: this one's last scaled time over its frequency, the other's over its.
        assert_eq!(float_at(&e, this.addr() + 0x54), 3.0);
        assert_eq!(float_at(&e, other.addr() + 0x54), 2.0);
        assert_eq!(active_elements(&e, this), vec![this.addr()]);
        assert_eq!(active_elements(&e, other), vec![other.addr()]);
    }

    #[test]
    fn fn_00a350d0_raises_a_zero_duration_and_refuses_active_sequences() {
        let mut e = setup();
        map_float_constants(&mut e);
        e.map(SMALL_TIME_010141CC, 4);
        set_float(&mut e, SMALL_TIME_010141CC, 0.0001);
        let (this, other) = cross_fade_fixture(&mut e);
        assert!(e
            .call(
                0x00a350d0,
                &args![this, other, 0u32, 0u32, 3u32, 0u32, 0u32, 0u32],
            )
            .bool());
        assert_eq!(float_at(&e, other.addr() + 0x50), 0.0001);
        assert_eq!(
            calls_to(&e, target(SLOT_DEACTIVATE))[1],
            vec![this.addr(), 0.0001f32.to_bits(), 1]
        );
        // Both sequences are active now: nothing more happens.
        assert!(!e
            .call(
                0x00a350d0,
                &args![this, other, 0u32, 0u32, 3u32, 0u32, 0u32, 0u32],
            )
            .bool());
        e.set(other, NiControllerSequence::m_eState, 0);
        assert!(!e
            .call(
                0x00a350d0,
                &args![this, other, 0u32, 0u32, 3u32, 0u32, 0u32, 0u32],
            )
            .bool());
    }

    #[test]
    fn fn_00a351d0_morphs_this_sequence_into_the_other() {
        let mut e = setup();
        let (this, other) = cross_fade_fixture(&mut e);
        matching_morph_keys(&mut e, this, other);
        assert!(e
            .call(
                0x00a351d0,
                &args![
                    this,
                    other,
                    2.0f32.to_bits(),
                    3u32,
                    0.5f32.to_bits(),
                    0.75f32.to_bits()
                ],
            )
            .bool());
        // This sequence was activated (state 1 by a ease-in of 2.0 = state 2) and then became a morph source.
        assert_eq!(e.get(this, NiControllerSequence::m_eState), 6);
        assert_eq!(float_at(&e, this.addr() + 0x1c), 0.5);
        // The other sequence is a transition destination with the ease-in, its offset and last time unset.
        assert_eq!(e.get(other, NiControllerSequence::m_eState), 5);
        assert_eq!(float_at(&e, other.addr() + 0x1c), 0.75);
        assert_eq!(float_at(&e, other.addr() + 0x4c), -f32::MAX);
        assert_eq!(float_at(&e, other.addr() + 0x50), 2.0);
        assert_eq!(float_at(&e, other.addr() + 0x48), -f32::MAX);
        assert_eq!(float_at(&e, other.addr() + 0x34), -f32::MAX);
        assert_eq!(
            calls_to(&e, target(SLOT_DEACTIVATE)),
            vec![vec![this.addr(), 0, 1]]
        );
        assert_eq!(active_elements(&e, other), vec![other.addr()]);
    }

    #[test]
    fn fn_00a351d0_without_an_ease_in_makes_the_other_animating_at_once() {
        let mut e = setup();
        let (this, other) = cross_fade_fixture(&mut e);
        matching_morph_keys(&mut e, this, other);
        assert!(e
            .call(0x00a351d0, &args![this, other, 0u32, 3u32, 0u32, 0u32],)
            .bool());
        assert_eq!(e.get(other, NiControllerSequence::m_eState), 1);
        assert_eq!(float_at(&e, other.addr() + 0x4c), 0.0);
        assert_eq!(e.get(this, NiControllerSequence::m_eState), 6);
    }

    #[test]
    fn fn_00a351d0_refuses_an_active_sequence_or_an_active_other_one() {
        let mut e = setup();
        let (this, other) = cross_fade_fixture(&mut e);
        matching_morph_keys(&mut e, this, other);
        e.set(other, NiControllerSequence::m_eState, 2);
        assert!(!e
            .call(0x00a351d0, &args![this, other, 0u32, 3u32, 0u32, 0u32])
            .bool());
        e.set(other, NiControllerSequence::m_eState, 0);
        e.set(this, NiControllerSequence::m_eState, 1);
        assert!(!e
            .call(0x00a351d0, &args![this, other, 0u32, 3u32, 0u32, 0u32])
            .bool());
        assert_eq!(e.get(other, NiControllerSequence::m_eState), 0);
    }

    #[test]
    fn destructor_frees_the_arrays_and_releases_what_the_sequence_holds() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        silent_interlocked(&mut e);
        allocator_doubles(&mut e);
        e.register(OBJECT_DESTRUCT, |_, _| Ret::default());
        let this = sequence(&mut e);
        let items = element_block(&mut e, 0, INTERP_ITEM_SIZE);
        let tags = element_block(&mut e, 0, ID_TAG_SIZE);
        let notes = element_block(&mut e, 0, 4);
        e.set(this, NiControllerSequence::m_pkInterpArray, Ptr::new(items));
        e.set(this, NiControllerSequence::m_pkIDTagArray, Ptr::new(tags));
        e.set(this, NiControllerSequence::spAnimNotesA, Ptr::new(notes));
        e.set(this, NiControllerSequence::usNumNotes, 1);
        let palette = object(&mut e);
        let keys = object(&mut e);
        e.mem.set_u32(this.addr() + 0x64, palette);
        e.mem.set_u32(this.addr() + 0x20, keys);
        e.mem.set_u32(this.addr() + 0x5c, 0x7e00);
        e.mem.set_u32(this.addr() + 8, 0x7f00);
        e.call(0x00a35640, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NI_CONTROLLER_SEQUENCE_VTABLE);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![
                vec![MEMORY_MANAGER, items - 4],
                vec![MEMORY_MANAGER, tags - 4],
                vec![MEMORY_MANAGER, notes - 4]
            ]
        );
        // Palette, accum root name, text keys and name lost one count each, in that order.
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![
                vec![palette + 4],
                vec![0x7e00 - 8],
                vec![keys + 4],
                vec![0x7f00 - 8]
            ]
        );
        assert_eq!(calls_to(&e, OBJECT_DESTRUCT), vec![vec![this.addr()]]);
    }

    #[test]
    fn destructor_deactivates_an_active_sequence_first() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        silent_interlocked(&mut e);
        e.register(OBJECT_DESTRUCT, |_, _| Ret::default());
        e.register(SEQUENCE_CLEAR_BLEND, |_, _| Ret::default());
        let (this, _) = update_sequence(&mut e, 1);
        set_float(&mut e, this.addr() + 0x34, -f32::MAX);
        e.set(this, NiControllerSequence::m_pkInterpArray, Ptr::NULL);
        e.set(this, NiControllerSequence::m_pkIDTagArray, Ptr::NULL);
        e.mem.set_u32(this.addr() + 0x5c, 0x9000);
        e.mem.set_u32(this.addr() + 8, 0x9000);
        e.call(0x00a35640, &args![this]);
        assert_eq!(e.get(this, NiControllerSequence::m_eState), 0);
        assert_eq!(calls_to(&e, SEQUENCE_CLEAR_BLEND), vec![vec![this.addr()]]);
        // The empty strings are not released.
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn post_link_object_turns_the_palette_offsets_into_strings_and_releases_the_palette() {
        let mut e = setup();
        map_fixed_strings(&mut e);
        e.register(OBJECT_STREAM_HOOK, |_, _| Ret::default());
        let this = sequence(&mut e);
        let (_, tags) = set_items(&mut e, this, 2);
        // Every offset is -1: the tags are only reset to the empty string.
        for word in 0..10 {
            e.mem.set_u32(tags + 4 * word, 0xffff_ffff);
        }
        let palette = object(&mut e);
        e.mem.set_u32(this.addr() + 0x64, palette);
        let (stream, _, _) = stream_with(&mut e, 0, 0, 0x0a01_0068, 0);
        e.call(0x00a352f0, &args![this, stream]);
        for word in 0..10 {
            assert_eq!(e.mem.u32(tags + 4 * word), 0x9000);
        }
        assert_eq!(e.mem.u32(this.addr() + 0x64), 0);
        assert_eq!(count_of(&e, palette), 0);
        // A new NIF version does nothing more.
        assert_eq!(calls_to(&e, TABLE_ALLOCATE).len(), 0);
    }

    #[test]
    fn post_link_object_moves_the_controllers_of_an_old_file_into_the_sequence() {
        let mut e = setup();
        e.register(OBJECT_STREAM_HOOK, |_, _| Ret::default());
        map_float_constants(&mut e);
        e.map(FLT_MAX_DOUBLE, 8);
        e.map(NEGATIVE_FLT_MAX_DOUBLE, 8);
        let max = f64::from(f32::MAX).to_bits();
        e.mem.set_u32(FLT_MAX_DOUBLE, max as u32);
        e.mem.set_u32(FLT_MAX_DOUBLE + 4, (max >> 32) as u32);
        let negative_max = (-f64::from(f32::MAX)).to_bits();
        e.mem.set_u32(NEGATIVE_FLT_MAX_DOUBLE, negative_max as u32);
        e.mem
            .set_u32(NEGATIVE_FLT_MAX_DOUBLE + 4, (negative_max >> 32) as u32);
        e.map(INVALID_BLEND_INDEX, 1);
        e.mem.set_u8(INVALID_BLEND_INDEX, 0xfe);
        map_statics(&mut e);
        c_string_doubles(&mut e);
        e.register(TABLE_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(POINTER_ARRAY_FREE, |_, _| Ret::default());
        e.register(ID_TAG_RESET, |_, _| Ret::default());
        // The class chain of the controllers does not reach NiInterpController: 00a34480 adds nothing.
        let record = e.mem.alloc(8);
        e.register_double(target(SLOT_GET_RTTI), move |_, _| record.into_ret());
        let this = sequence(&mut e);
        let (items, tags) = set_items(&mut e, this, 2);
        let first_controller = object(&mut e);
        let second_controller = object(&mut e);
        let interpolator = object(&mut e);
        e.mem.set_u32(items, interpolator);
        e.mem.set_u32(items + 4, first_controller);
        e.mem
            .set_u32(items + INTERP_ITEM_SIZE + 4, second_controller);
        e.mem.set_u32(items + 8, 0x4444);
        // Controller 0 runs 1 to 5 and has the flags 0b100 (cycle type 2) and the frequency 1.5; controller 1
        // runs 0.5 to 9.
        set_float(&mut e, first_controller + 0x14, 1.0);
        set_float(&mut e, first_controller + 0x18, 5.0);
        set_float(&mut e, first_controller + 0xc, 1.5);
        e.mem.set_u16(first_controller + 8, 0b100);
        set_float(&mut e, second_controller + 0x14, 0.5);
        set_float(&mut e, second_controller + 0x18, 9.0);
        let first_name = string(&mut e, "Bip01");
        let second_name = string(&mut e, "Spine");
        e.mem.set_u32(tags, first_name);
        e.mem.set_u32(tags + ID_TAG_SIZE, second_name);
        let (stream, _, _) = stream_with(&mut e, 0, 0, 0x0a01_0067, 0);
        e.call(0x00a352f0, &args![this, stream]);
        assert_eq!(float_at(&e, this.addr() + 0x2c), 0.5);
        assert_eq!(float_at(&e, this.addr() + 0x30), 9.0);
        assert_eq!(e.mem.u32(this.addr() + 0x24), 2);
        assert_eq!(float_at(&e, this.addr() + 0x28), 1.5);
        // The items were cleared and their tags reset.
        assert_eq!(e.mem.u32(items), 0);
        assert_eq!(e.mem.u32(items + 4), 0);
        assert_eq!(e.mem.u32(items + 8), 0);
        assert_eq!(e.mem.u8(items + 0xc), 0xfe);
        assert_eq!(e.mem.u8(items + 0xd), 0xff);
        assert_eq!(e.mem.u32(items + INTERP_ITEM_SIZE + 4), 0);
        assert_eq!(
            calls_to(&e, ID_TAG_RESET),
            vec![vec![tags], vec![tags + ID_TAG_SIZE]]
        );
        // The item's count of the controller went; the temporary array keeps one (it is freed by the
        // double of `00a47600`, which releases nothing).
        assert_eq!(count_of(&e, first_controller), 1);
        assert_eq!(count_of(&e, interpolator), 0);
        // The name copies and the pointer array were freed.
        assert_eq!(calls_to(&e, TABLE_FREE).len(), 3);
        assert_eq!(calls_to(&e, TABLE_ALLOCATE)[1..], vec![vec![6], vec![6]]);
        assert_eq!(calls_to(&e, POINTER_ARRAY_FREE).len(), 1);
    }

    #[test]
    fn post_link_object_of_an_old_file_without_items_leaves_the_times_at_zero() {
        let mut e = setup();
        e.register(OBJECT_STREAM_HOOK, |_, _| Ret::default());
        map_float_constants(&mut e);
        e.map(FLT_MAX_DOUBLE, 8);
        e.map(NEGATIVE_FLT_MAX_DOUBLE, 8);
        let max = f64::from(f32::MAX).to_bits();
        e.mem.set_u32(FLT_MAX_DOUBLE, max as u32);
        e.mem.set_u32(FLT_MAX_DOUBLE + 4, (max >> 32) as u32);
        let negative_max = (-f64::from(f32::MAX)).to_bits();
        e.mem.set_u32(NEGATIVE_FLT_MAX_DOUBLE, negative_max as u32);
        e.mem
            .set_u32(NEGATIVE_FLT_MAX_DOUBLE + 4, (negative_max >> 32) as u32);
        e.register(POINTER_ARRAY_FREE, |_, _| Ret::default());
        e.register(TABLE_FREE, |_, _| Ret::default());
        let this = sequence(&mut e);
        let (stream, _, _) = stream_with(&mut e, 0, 0, 0x0a01_0067, 0);
        e.call(0x00a352f0, &args![this, stream]);
        assert_eq!(e.mem.u32(this.addr() + 0x2c), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x30), 0);
        assert_eq!(calls_to(&e, TABLE_FREE), vec![vec![0]]);
    }

    #[test]
    fn ni_controller_sequence_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a31730, &args![]).u32(),
            NI_CONTROLLER_SEQUENCE_RTTI
        );
    }

    #[test]
    fn ni_controller_sequence_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a3_5640, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a35820, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a35820, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x74]]);
        assert_eq!(calls_to(&e, 0x00a3_5640), vec![vec![this]; 2]);
    }

    #[test]
    fn create_object_00a326c0_constructs_a_new_sequence_or_answers_null() {
        let mut e = setup();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SEQUENCE_CONSTRUCT_EMPTY, |_, a| a[0].into_ret());
        let object = e.call(0x00a3_26c0, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x74]]);
        assert_eq!(calls_to(&e, SEQUENCE_CONSTRUCT_EMPTY), vec![vec![object]]);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a3_26c0, &args![]).u32(), 0);
        assert!(calls_to(&e, SEQUENCE_CONSTRUCT_EMPTY).is_empty());
    }
}
