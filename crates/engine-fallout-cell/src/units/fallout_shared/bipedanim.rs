//! `fallout shared/bipedanim.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BipedAnim` is the set of body parts an actor wears: a root `NiNode`, five
//! named bones, and 20 biped slots, each with the form that fills it, the
//! model part, the loaded 3D clone and a skinned flag. A second array of 20
//! slots (the "buffered" objects) keeps the previous set while a new one is
//! loaded, so a part that did not change is reused.
//!
//! Session 1 (b0021) covers `004aaca0` to `004ad4a0`, the first 40 functions:
//! construction and `InitRoot`, `RemoveAllParts`, the slot clearing and
//! lookup helpers, the weapon helpers, the form to slot assignment
//! (`004abad0`), the model and texture queueing (`004abd30`, `004abfe0`,
//! `QueueSkinTexture`), `LoadBipedParts`, the small `NiCloningProcess`
//! members this unit emits, `ApplySkinnedObjects` and `AttachSkinnedObject`.
//!
//! Session 2 (b0021) covers `004add50` to `004afc50`, the next 40 functions:
//! the small members of the bone table and node array (`004add50` to
//! `004ade20`), `AttachToSkeleton`, `AttachToParent`, `LoadFaceGenModel`,
//! `CloneHelmet`, `AttachHelmet`, the queueing of a FaceGen part
//! (`004aede0`, `004aee60`), `LoadAndAttachAddOn`, `AddAddonNodes`,
//! `GetLightingProperty`, `GetSkinBipedObject`, `AdjustSkinComplexion`, the
//! first person path helpers (`004af950` to `004afa50`), and the constructors
//! and destructors of the clone map, the process map and the node array. The
//! next session continues at `004afc80` (`NiTArray<NiPointer<NiAVObject>>`
//! `Compact`) with the rest of the queue (the map and array members up to
//! `004b01f0`, `005e0ba0` and `00ba87e0`).
//!
//! Session 3 (b0021) finishes the unit: `Compact` and `UpdateSize` of the
//! node array, the base constructors and destructors of the two maps, their
//! scalar deleting destructors, `NiTNewInterface<NiPointer<NiAVObject>>::
//! Allocate` and `BipedAnim::GetParentBone`. Every function of the unit is
//! translated; the older `FN_` constants stay because earlier tests stand in
//! for those functions.
//!
//! Conventions this file uses, so the next session finds them:
//!
//! - The layouts ([`BipedAnim`], [`BipedBone`], [`BipedObject`],
//!   [`NiCloningProcess`]) and the slot accessors ([`bone`], [`object`],
//!   [`buffered`]) are first below and `pub(crate)`. A function of this unit
//!   that is not translated yet (the ones from `004afc80` on) is called by
//!   address through a `FN_` constant until a later session translates it; one
//!   that is translated is called directly. The calls session 1 wrote to the
//!   functions session 2 translated (`FN_004ADDA0` to `FN_004AFC50`) still go
//!   by address, because session 1's tests stand in for them; they behave the
//!   same.
//! - Tiny functions of other units are called by address through the
//!   constants below. `00559450` is the one the linker folded into dozens of
//!   getters: it returns the word at the address in `ECX`, so on an
//!   `NiPointer` it is the pointer itself ([`ni_pointer_get`]).
//! - The compiler's exception-unwinding frames and stack cookies are not
//!   translated. A local the game keeps on its stack and passes by address is
//!   a block from `stack_alloc` (released with `stack_free`).
//! - The decompiler hung pushed words on the wrong calls in this unit
//!   (`008a16d0`, `008b70d0`, `0043f8d0`, `006a9540` take no or other stack
//!   words than it shows); every call below follows the disassembly.

#[allow(unused_imports)]
use crate::prelude::*;

// ---- Layouts -----------------------------------------------------------------

layout! {
    /// `BipedAnim` (Xbox PDB), 0x2b4 bytes: the root node at +0, the five
    /// bones at +4 (8 bytes each, [`bone`]), the 20 slots at +0x2c (16 bytes
    /// each, [`object`]), the 20 buffered slots at +0x16c ([`buffered`]), the
    /// weapon offset and the requester.
    pub struct BipedAnim: 0x2b4 {
        /// `root` (Xbox PDB): the skeleton's `NiNode*`.
        0x000 root: Ptr,
        /// `m_fWeaponOffset` (Xbox PDB).
        0x2ac m_fWeaponOffset: f32,
        /// `m_pRequester` (Xbox PDB): the `TESObjectREFR*` (an actor) the
        /// parts are loaded for.
        0x2b0 m_pRequester: Ptr,
    }

    /// `BIPBONE` (Xbox PDB), 8 bytes: one named bone of the skeleton.
    pub struct BipedBone: 0x08 {
        /// `cFlags` (Xbox PDB): bit 0 is set once the bone is found.
        0x00 cFlags: u8,
        /// `pParent` (Xbox PDB): the bone's node, where parts are attached.
        0x04 pParent: Ptr,
    }

    /// `BIPOBJECT` (Xbox PDB), 16 bytes: one biped slot.
    pub struct BipedObject: 0x10 {
        /// `pParent` (Xbox PDB): the form (`TESForm*`) filling the slot.
        0x00 pParent: Ptr,
        /// `pPart` (Xbox PDB): the model (`TESModel*`) of the part.
        0x04 pPart: Ptr,
        /// `pPartClone` (Xbox PDB): the loaded clone (`NiAVObject*`).
        0x08 pPartClone: Ptr,
        /// `bSkinned` (Xbox PDB).
        0x0C bSkinned: bool,
    }

    /// `NiCloningProcess` (Xbox PDB), 0x10 bytes on the Xbox; the PC build
    /// adds the scale (three floats) at +0x10, set by the constructors here.
    pub struct NiCloningProcess: 0x1c {
        /// `m_pkCloneMap` (Xbox PDB): `NiTPointerMap<NiObject *,NiObject *>*`.
        0x00 m_pkCloneMap: Ptr,
        /// `m_pkProcessMap` (Xbox PDB): `NiTPointerMap<NiObject *,bool>*`.
        0x04 m_pkProcessMap: Ptr,
        /// `m_eCopyType` (Xbox PDB).
        0x08 m_eCopyType: u32,
        /// `m_cAppendChar` (Xbox PDB).
        0x0C m_cAppendChar: u8,
        /// The scale, x (PC only).
        0x10 m_fScale_x: f32,
        /// The scale, y (PC only).
        0x14 m_fScale_y: f32,
        /// The scale, z (PC only).
        0x18 m_fScale_z: f32,
    }
}

/// Number of bones in [`BipedAnim`] (`BipedAnim::bone`).
pub(crate) const BONE_COUNT: u32 = 5;
/// Number of biped slots (and of buffered slots).
pub(crate) const SLOT_COUNT: u32 = 20;
/// Offsets of the three arrays inside [`BipedAnim`].
pub(crate) const BONES_OFFSET: u32 = 0x04;
pub(crate) const OBJECTS_OFFSET: u32 = 0x2c;
pub(crate) const BUFFERED_OFFSET: u32 = 0x16c;
/// The slot of the weapon.
pub(crate) const WEAPON_SLOT: u32 = 5;

/// Bone `index` of `this` (`BipedAnim::bone[index]`).
pub(crate) fn bone(this: Ptr<BipedAnim>, index: u32) -> Ptr<BipedBone> {
    Ptr::new(this.addr().wrapping_add(BONES_OFFSET + 8 * index))
}

/// Slot `slot` of `this` (`BipedAnim::object[slot]`).
pub(crate) fn object(this: Ptr<BipedAnim>, slot: u32) -> Ptr<BipedObject> {
    Ptr::new(
        this.addr()
            .wrapping_add(OBJECTS_OFFSET.wrapping_add(slot.wrapping_mul(0x10))),
    )
}

/// Buffered slot `slot` of `this` (`BipedAnim::bufferedObjects[slot]`).
pub(crate) fn buffered(this: Ptr<BipedAnim>, slot: u32) -> Ptr<BipedObject> {
    Ptr::new(
        this.addr()
            .wrapping_add(BUFFERED_OFFSET.wrapping_add(slot.wrapping_mul(0x10))),
    )
}

// ---- Globals and data of the exe ----------------------------------------------

/// The global holding the `ModelLoader` (`011c3b3c`).
pub(crate) const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The global holding the player character (`011dea3c`).
pub(crate) const PLAYER: u32 = 0x011d_ea3c;
/// The global holding the `TES` object (`011dea10`), `this` of
/// `TES::CreateDeepCopySameTextures`.
const TES_GLOBAL: u32 = 0x011d_ea10;
/// The data handler singleton pointer (`011c3f2c`).
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The save-load object pointer (`011de45c`).
const SAVE_LOAD: u32 = 0x011d_e45c;
/// The object at `011ddf38` whose flags `0042ce10` reads.
const FLAGS_OBJECT: u32 = 0x011d_df38;
/// The object `004538a0` and `004538c0` (the Xbox name of the latter is
/// `RtlLeaveCriticalSection`) bracket the removal of all parts with.
const PART_LOCK: u32 = 0x011c_5cf0;
/// A byte global tested before the FaceGen body texture palette is cleared
/// and before skin complexion is adjusted (its meaning is not confirmed).
const PALETTE_FLAG: u32 = 0x011c_5cb4;
/// Setting objects whose string value (`00408d60` gives its address) the
/// biped code tests for "not empty" (which settings they are is not
/// confirmed): `SETTING_FACE` alone, and the pair `004abfa0` tests.
const SETTING_FACE: u32 = 0x011c_5ef0;
const SETTING_PAIR_FIRST: u32 = 0x011d_5a10;
const SETTING_PAIR_SECOND: u32 = 0x011d_5a44;
/// Three-word global (zero) copied into a cloned node's translation
/// (`00440460`), and the nine-word global (identity) copied into its
/// rotation (`0043fa80`).
const ZERO_TRANSLATION: u32 = 0x011f_426c;
const IDENTITY_ROTATION: u32 = 0x011a_9448;
/// `-pi / 2` as a `float` (`01016b78`): the angle of the rotation about Y
/// `LoadBipedParts` applies to a FaceGen part.
const QUARTER_TURN_BACK: u32 = 0x0101_6b78;
/// The word `004ad1b0` returns and the byte `004ad1c0` returns: copied into
/// every new `NiCloningProcess`.
const CLONE_COPY_TYPE: u32 = 0x011f_4300;
const CLONE_APPEND_CHAR: u32 = 0x011a_94a8;
/// The global `004ab220` returns: the key of the extra data
/// `AttachSkinnedObject` keeps on the weapon bone (`NiFixedString`).
const EXTRA_DATA_KEY: u32 = 0x011c_61e4;
/// Class argument of `00653270` (the class `ApplySkinnedObjects` asks the
/// object about).
const SKINNED_CLASS: u32 = 0x011f_9140;

/// The bone names: five `char*` (`01188b74`).
const BONE_NAMES: u32 = 0x0118_8b74;
/// The three slots that carry skin (`01188b88`; slots 2, 3 and 4).
const SKIN_SLOTS: u32 = 0x0118_8b88;
/// The 20 slot names, `char*` each (`01188b98`).
const SLOT_NAMES: u32 = 0x0118_8b98;
/// For each slot, the bone index it attaches to, or -1 (`01188be8`).
const SLOT_BONES: u32 = 0x0118_8be8;

/// Type descriptors `__RTDynamicCast` is given: `TESForm`, `TESRace`,
/// `TESBoundObject` and `TESNPC`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_RACE: u32 = 0x0118_6370;
const TYPE_TES_BOUND_OBJECT: u32 = 0x0118_3108;
const TYPE_TES_NPC: u32 = 0x0118_3a1c;

/// The form type (`TESForm::cFormType`) `004ab400` and `004ab750` require
/// (a weapon form) and the one `LoadBipedParts` requires of the requester's
/// base form (an NPC).
const FORM_TYPE_WEAPON: u32 = 0x28;
const FORM_TYPE_NPC: u32 = 0x2a;
/// The form id `004ab400` treats specially when choosing the weapon model.
const FORM_ID_SPECIAL_WEAPON: u32 = 0x0017_35d4;

/// Texts: the root node name (`Bip01`), the log formats, the weapon bone's
/// attachment node name, the backpack node, the normal map suffix (`_n`),
/// `NULL` and the prefixes of the pip-boy node names.
const ROOT_NODE_NAME: u32 = 0x0101_e460;
const MISSING_BONE_FORMAT: u32 = 0x0101_f684;
const BACKPACK_NAME: u32 = 0x0101_f5cc;
const SLOT_NAME_FORMAT: u32 = 0x0101_f6a8;
const SKIN_ATTACHMENT_NAME: u32 = 0x0101_f724;
const NON_GEOMETRY_FORMAT: u32 = 0x0101_f6f4;
const SHOULD_BE_SKINNED_FORMAT: u32 = 0x0101_f6bc;
const NORMAL_MAP_SUFFIX: u32 = 0x0101_f6b8;
const UNNAMED_BONE_FORMAT: u32 = 0x0101_f738;
const NULL_TEXT: u32 = 0x0101_f7a8;
const PIPBOY_OFF_PREFIX: u32 = 0x0101_f7bc;
const PIPBOY_ON_PREFIX: u32 = 0x0101_f7b0;

// ---- Callees outside this unit --------------------------------------------------

/// `memset(destination, value, size)` (`00403d30`, cdecl).
const MEMSET: u32 = 0x0040_3d30;
/// `operator new(size)` (`00401000`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `NiMemObject::operator new(size)` (`00aa13e0`).
const NI_OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `__ehvec_ctor` and `__ehvec_dtor` (`00ec782f`, `00ec5fce`).
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// `_strnicmp`-style `(a, b, count)` compare (`00ec7ec0`), and the string
/// compare `(a, b)` (`00404dc0`).
const COMPARE_PREFIX: u32 = 0x00ec_7ec0;
const COMPARE_TEXT: u32 = 0x0040_4dc0;
/// `strlen` (`0044a670`, cdecl).
const TEXT_LENGTH: u32 = 0x0044_a670;
/// The log function `(format, ...)` (`005b5e40`, cdecl) and `sprintf` into a
/// `BSString` (`00406f60`: `(string, format, ...)`).
const LOG: u32 = 0x005b_5e40;
const SPRINTF: u32 = 0x0040_6f60;
/// `BSString` (8 bytes) constructor and destructor.
const STRING_INIT: u32 = 0x0040_37b0;
const STRING_FREE: u32 = 0x0040_37d0;
/// `BSString::BSString(const char *)` (`0040c0e0`).
const STRING_FROM_TEXT: u32 = 0x0040_c0e0;
/// `NiFixedString` constructor from `const char *` (`00438170`, returns the
/// fixed string) and destructor (`004381b0`).
const FIXED_STRING_INIT: u32 = 0x0043_8170;
const FIXED_STRING_FREE: u32 = 0x0043_81b0;
/// `NiObjectNET::SetName(const NiFixedString &)` (`00a5b950`).
const SET_NAME: u32 = 0x00a5_b950;
/// `NiPointer` operations: init from a pointer, assign, release, and the
/// folded getter `00559450`.
const NI_POINTER_INIT: u32 = 0x0063_3c90;
const NI_POINTER_SET: u32 = 0x0066_b0d0;
const NI_POINTER_RELEASE: u32 = 0x0045_cec0;
const READ_WORD: u32 = 0x0055_9450;
/// `NiNode` child accessors: the count of the child array (`0043b480`) and
/// the child at an index (`0043b4a0`).
const CHILD_COUNT: u32 = 0x0043_b480;
const CHILD_AT: u32 = 0x0043_b4a0;
/// `00413f40(node)` gives the address of the node's name field (`this+8`)
/// and `0043b1b0` turns it into the name text.
const NAME_FIELD: u32 = 0x0041_3f40;
const NAME_TEXT: u32 = 0x0043_b1b0;
/// `0043fad0(node)`: the `NiPointer` at `+0xbc` of a geometry node (its
/// skin or property data); `0043b230(data)` and `005495f0(node)` read
/// further fields of such objects.
const NODE_DATA_POINTER: u32 = 0x0043_fad0;
const NODE_DATA_FIELD: u32 = 0x0043_b230;
const NODE_SKIN_DATA: u32 = 0x0054_95f0;
/// `NiObjectNET::GetExtraData(key)`, `AddExtraData(key, data)`,
/// `RemoveExtraData(key)`.
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
const ADD_EXTRA_DATA: u32 = 0x00a5_bc40;
const REMOVE_EXTRA_DATA: u32 = 0x00a5_be90;
/// `NiObject::Clone(cloning process)`, `Clone_ov2` and
/// `CreateDeepCopy(out NiPointer)`.
const NI_OBJECT_CLONE: u32 = 0x00a5_d2c0;
const NI_OBJECT_CLONE_OV2: u32 = 0x00a5_d680;
const NI_OBJECT_DEEP_COPY: u32 = 0x00a5_d510;
/// `TES::CreateDeepCopySameTextures(model, cloning process)`.
const DEEP_COPY_SAME_TEXTURES: u32 = 0x0045_7ba0;
/// `HasMorpherController(model)`.
const HAS_MORPHER_CONTROLLER: u32 = 0x004b_5bf0;
/// `NiAVObject::UpdateProperties` (`00a5a040`), the `NiNode` constructor
/// `(this, 0)` (`00a5ecb0`) and `BSFadeNode::SetLODMultType` (`00b4dec0`).
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const NI_NODE_INIT: u32 = 0x00a5_ecb0;
const SET_LOD_MULT_TYPE: u32 = 0x00b4_dec0;
/// The form type getter (`TESForm::cFormType`, `00401170`) and the form id
/// getter (`0084e3a0`, `this+0xc`).
const FORM_TYPE: u32 = 0x0040_1170;
const FORM_ID: u32 = 0x0084_e3a0;
/// The reference's base form getter (`007af430`; the maps name it
/// `BGSSaveFormBuffer::GetForm`, a folded name).
const REFERENCE_FORM: u32 = 0x007a_f430;
/// `TESBipedModelForm::GetFormAsBipedModel` (`00480db0`, cdecl),
/// `FillsBipedSlot(slot, flag, flag)` (`00480af0`) and the model path getter
/// `(this)` `00480ce0`.
const GET_FORM_AS_BIPED_MODEL: u32 = 0x0048_0db0;
const FILLS_BIPED_SLOT: u32 = 0x0048_0af0;
const BIPED_MODEL_PATH: u32 = 0x0048_0ce0;
/// `TESModelTextureSwap::SwapTextures(node)` and
/// `SwapPlatformLanguageTextures(node)`.
const SWAP_TEXTURES: u32 = 0x0048_afe0;
const SWAP_PLATFORM_TEXTURES: u32 = 0x004b_7660;
/// `00522df0(form; flag, 1)`: the model a form wears (`TESModel*`).
const FORM_MODEL: u32 = 0x0052_2df0;
/// `008d8b00(form)` (named `MiddleHighProcess::GetHeadAnims`, a folded
/// name): the word at `+0x250` of a weapon form.
const FORM_ANIMS: u32 = 0x008d_8b00;
/// `0048cee0(model)`: whether a model has a path.
const MODEL_HAS_PATH: u32 = 0x0048_cee0;
/// The model loader methods: `LoadFile(path, 3, 1, 0, flag, 0)`,
/// `QueueTexture(path, a, b)`, the two `QueueModel(path, a, b, 3, 1, 0, 0)`
/// overloads and `0045a5e0(path)` (releases a queued model).
const LOAD_FILE: u32 = 0x0044_7080;
const QUEUE_TEXTURE: u32 = 0x0044_36c0;
const QUEUE_MODEL_FORM: u32 = 0x0044_3d30;
const QUEUE_MODEL: u32 = 0x0044_4040;
const RELEASE_MODEL: u32 = 0x0045_a5e0;
/// `BSShaderManager::GetModifiedTextureFilename(out, in, suffix, 1)`.
const MODIFIED_TEXTURE_FILENAME: u32 = 0x00b4_f5e0;
/// The race body texture helpers: `TESRace::GetBodyModTextureName`
/// (`006147e0`), `GetBodyModTextureFileName` (`006148f0`) and the skin
/// texture path `00614640(npc, slot, out)`.
const BODY_MOD_TEXTURE_NAME: u32 = 0x0061_47e0;
const BODY_MOD_TEXTURE_FILE_NAME: u32 = 0x0061_48f0;
const SKIN_TEXTURE_PATH: u32 = 0x0061_4640;
/// `006151f0(form; arg, slot)`: the item a form gives for a slot.
const FORM_SLOT_ITEM: u32 = 0x0061_51f0;
/// `00726070(object)`: reads the word at `+4`.
const READ_WORD_PLUS_4: u32 = 0x0072_6070;
/// The player: `GetBiped(first)` (`00950b00`), `Is1stPersonBiped(biped)`
/// (`00950b30`), `GetNode(first)` (`00950bb0`) and
/// `GetAnimation(first)` (`00950a60`).
const GET_BIPED: u32 = 0x0095_0b00;
const IS_FIRST_PERSON_BIPED: u32 = 0x0095_0b30;
const PLAYER_NODE: u32 = 0x0095_0bb0;
const PLAYER_ANIMATION: u32 = 0x0095_0a60;
/// Actor: `GetSavedAcquireObject` (`008d8520`, the word at `+0x68`),
/// `GetAnimation` (`008b70d0`), `IsWeaponDrawn` (`008a16d0`) and
/// `ReloadTargets(flag)` (`008b0b00`).
const SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
const ACTOR_ANIMATION: u32 = 0x008b_70d0;
const IS_WEAPON_DRAWN: u32 = 0x008a_16d0;
const RELOAD_TARGETS: u32 = 0x008b_0b00;
/// `Animation::BlendOut(4, 0)` (`004994f0`) and `FindSkinnedNode(node)`
/// (`004910d0`).
const BLEND_OUT: u32 = 0x0049_94f0;
const FIND_SKINNED_NODE: u32 = 0x0049_10d0;
/// Tiny flag getters of other units: the data handler's flag (`004226e0`),
/// the save-load object's (`0047c850`), `0042ce10` of the flags object and
/// `0043faf0`/`00408d60` of the settings.
const DATA_HANDLER_FLAG: u32 = 0x0042_26e0;
const SAVE_LOAD_FLAG: u32 = 0x0047_c850;
const FLAGS_OBJECT_FLAG: u32 = 0x0042_ce10;
const SETTING_BYTE_VALUE: u32 = 0x0043_faf0;
const SETTING_BYTE_POINTER: u32 = 0x0040_8d60;
/// The weapon helper `00450f90(node, flag)`.
const SET_WEAPON_FLAG: u32 = 0x0045_0f90;
/// `ItemChange::GetModSlots` (`004bd820`) and `HasModEffectActive`
/// (`004bd8d0`).
const GET_MOD_SLOTS: u32 = 0x004b_d820;
const HAS_MOD_EFFECT_ACTIVE: u32 = 0x004b_d8d0;
/// `00504e60(form)` and `00709c20(value)` (cdecl).
const WEAPON_MODEL_VALUE: u32 = 0x0050_4e60;
const SHOW_WEAPON: u32 = 0x0070_9c20;
/// `00571760(actor; form)`.
const ACTOR_SET_WEAPON: u32 = 0x0057_1760;
/// The dismemberment: `005d43c0(actor)` gives the extra data list,
/// `GetDismembermentExtra` (`0042e8c0`), `Dismembered(limb)` (`004303e0`)
/// and `BGSBodyPart::HideLimb(limb, node)` (`005e4730`).
const ACTOR_EXTRA_LIST: u32 = 0x005d_43c0;
const GET_DISMEMBERMENT_EXTRA: u32 = 0x0042_e8c0;
const LIMB_DISMEMBERED: u32 = 0x0043_03e0;
const HIDE_LIMB: u32 = 0x005e_4730;
/// `0055d520(actor)`: the text naming the actor in a log line.
const ACTOR_LOG_NAME: u32 = 0x0055_d520;
/// `0043fcd0(actor)`: the actor's 3D root, `00440460(node, translation)`,
/// `0043fa80(node, rotation)`.
const ACTOR_ROOT: u32 = 0x0043_fcd0;
const SET_TRANSLATION: u32 = 0x0044_0460;
const SET_ROTATION: u32 = 0x0043_fa80;
/// `NiMatrix3::MakeYRotation(angle)` (`0043f850`), `NiMatrix3::operator*`
/// (`0043f8d0`: `this` is the left matrix, then the result and the right
/// matrix), `006a9540(node)`, the address of the node's rotation, and the
/// matrix constructor `006815c0`.
const MAKE_Y_ROTATION: u32 = 0x0043_f850;
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
const NODE_ROTATION: u32 = 0x006a_9540;
const MATRIX_INIT: u32 = 0x0068_15c0;
/// `TESNPC::GetFaceCoord(out)` (`00603ad0`) and the FaceGen model method
/// `0065a970(coords, node, 0)`.
const GET_FACE_COORD: u32 = 0x0060_3ad0;
const APPLY_FACE_COORDS: u32 = 0x0065_a970;
/// `BSFaceGenManager::ClearBodyTexturesFromPalette(form)`.
const CLEAR_BODY_PALETTE: u32 = 0x0065_70e0;
/// Constructor and destructor of one FaceGen coordinate (0x20 bytes).
const FACE_COORD_INIT: u32 = 0x0044_9610;
const FACE_COORD_FREE: u32 = 0x0044_9680;
/// `00653270(class, node)`.
const CAST_TO_CLASS: u32 = 0x0065_3270;
/// The shadow scene node table getter `00450b80(index)`, `RemoveObject`
/// (`00b5b1c0`), the node's parent (`009611e0`) and
/// `bhkWorld::RemoveObjects(node, 1, 0)` (`00c69ee0`).
const SHADOW_SCENE_NODE: u32 = 0x0045_0b80;
const SHADOW_REMOVE_OBJECT: u32 = 0x00b5_b1c0;
const NODE_PARENT: u32 = 0x0096_11e0;
const WORLD_REMOVE_OBJECTS: u32 = 0x00c6_9ee0;
/// `004b6ce0(node)`.
const PREPARE_DETACH: u32 = 0x004b_6ce0;
/// The deferred detach: `008c7aa0()` (whether detaching must be deferred),
/// `004537b0()` (the task queue) and `0087acb0(queue; node)`.
const MUST_DEFER: u32 = 0x008c_7aa0;
const TASK_QUEUE_GETTER: u32 = 0x0045_37b0;
const TASK_QUEUE_DETACH: u32 = 0x0087_acb0;
/// `004538a0(lock, 0)` and `004538c0(lock)`.
const LOCK_ENTER: u32 = 0x0045_38a0;
const LOCK_LEAVE: u32 = 0x0045_38c0;
/// `0049c680(list, &key, 0)` and `004700d0(list, &item)` (the weapon bone's
/// extra data list).
const FIND_IN_LIST: u32 = 0x0049_c680;
const ADD_TO_LIST: u32 = 0x0047_00d0;
/// `00528cb0(this, 0)` and `0044cbf0(this)` (a four-byte object built and
/// destroyed by `004abd30`), `0092c820(arg; object)`, `00437730(data; arg)`.
const QUEUE_OBJECT_INIT: u32 = 0x0052_8cb0;
const QUEUE_OBJECT_FREE: u32 = 0x0044_cbf0;
const QUEUE_OBJECT_USE: u32 = 0x0092_c820;
const SET_PARENT_OBJECT: u32 = 0x0043_7730;
/// The body part lights of a geometry's data: `005585e0(data)`,
/// `008041a0(owner)` (count) and `00825c00(data)` (array of items).
const BODY_PART_OWNER: u32 = 0x0055_85e0;
const BODY_PART_COUNT: u32 = 0x0080_41a0;
const BODY_PART_ARRAY: u32 = 0x0082_5c00;
/// `00c4b310(node, name, 1)`: the object named `name` under `node`.
const FIND_OBJECT_BY_NAME: u32 = 0x00c4_b310;
/// Functions of this unit that session 1 called by address because they were
/// not translated yet. Session 2 translated them (`004add50` to `004afc50`),
/// but these call sites keep the address: session 1's tests stand in for the
/// functions. Session 2's own calls between functions of this file are direct.
const FN_004ADDA0: u32 = 0x004a_dda0;
const FN_004ADDC0: u32 = 0x004a_ddc0;
const FN_004ADDE0: u32 = 0x004a_dde0;
const FN_004ADE00: u32 = 0x004a_de00;
const FN_004ADE20: u32 = 0x004a_de20;
const FN_004ADD50: u32 = 0x004a_dd50;
const FN_004ADD70: u32 = 0x004a_dd70;
const FN_004AE250: u32 = 0x004a_e250;
const FN_004AE790: u32 = 0x004a_e790;
const FN_004AE8A0: u32 = 0x004a_e8a0;
const FN_004AEDE0: u32 = 0x004a_ede0;
const FN_004AEE60: u32 = 0x004a_ee60;
const FN_004AEED0: u32 = 0x004a_eed0;
const FN_004AF240: u32 = 0x004a_f240;
const FN_004AF490: u32 = 0x004a_f490;
const FN_004AF950: u32 = 0x004a_f950;
const FN_004AFA20: u32 = 0x004a_fa20;
const FN_004AFA50: u32 = 0x004a_fa50;
const FN_004AFAD0: u32 = 0x004a_fad0;
const FN_004AFB20: u32 = 0x004a_fb20;
const FN_004AFB50: u32 = 0x004a_fb50;
const FN_004AFB80: u32 = 0x004a_fb80;
const FN_004AFBA0: u32 = 0x004a_fba0;
const FN_004AFC50: u32 = 0x004a_fc50;
const FN_004AFF00: u32 = 0x004a_ff00;

// ---- Session 2: functions of this unit not translated yet --------------------------

/// The next session translates these (`NiTArray<NiPointer<NiAVObject>>`
/// `Compact` and `UpdateSize`, the constructors and destructors of the clone
/// map and the process map, the array destructor and the add at an index).
const FN_004AFC80: u32 = 0x004a_fc80;
const FN_004AFE50: u32 = 0x004a_fe50;
const FN_004AFF30: u32 = 0x004a_ff30;
const FN_004B0030: u32 = 0x004b_0030;
const FN_004AFFA0: u32 = 0x004a_ffa0;
const FN_004B00A0: u32 = 0x004b_00a0;
const FN_004B0220: u32 = 0x004b_0220;
/// Session 3: the callees of the array and map members below.
/// `NiPointer::operator!=` on two handles (`00631820(this, other)`: whether
/// the pointers differ), `NiPointer::operator=(NiPointer &)`
/// (`006e5cc0(this, other)`), the `NiPointer` constructor `__ehvec_ctor`
/// runs on each element (`006694e0`), `NiMemObject` allocation (`00aa1070`,
/// cdecl, size), `NiTMapBase::RemoveAll` (`00438af0`) and the two base map
/// destructors (`004b0000` for the clone map, `004b0100` for the process map).
const NI_POINTER_DIFFERS: u32 = 0x0063_1820;
const NI_POINTER_COPY: u32 = 0x006e_5cc0;
const NI_POINTER_ELEMENT_INIT: u32 = 0x0066_94e0;
const NI_ALLOC: u32 = 0x00aa_1070;
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
const CLONE_MAP_BASE_DESTRUCT: u32 = 0x004b_0000;
const PROCESS_MAP_BASE_DESTRUCT: u32 = 0x004b_0100;
/// The vtables the base constructors store (`0101fbdc`, `0101fbfc`).
const CLONE_MAP_BASE_VTABLE: u32 = 0x0101_fbdc;
const PROCESS_MAP_BASE_VTABLE: u32 = 0x0101_fbfc;
const FN_004B02D0: u32 = 0x004b_02d0;

// ---- Session 2: data of the exe ------------------------------------------------------

/// The global holding the `NiFixedString` that is the key of the parent node
/// extra data on a part (`004ae780` returns it; the log text names the key
/// `Prn`).
const PARENT_NODE_KEY: u32 = 0x011c_61e8;
/// Class arguments of `00653270`: the string extra data class (the log text
/// says `NiStringExtraData`) and the texture class `AdjustSkinComplexion`
/// asks the property's texture about.
const STRING_EXTRA_CLASS: u32 = 0x011f_4a38;
const TEXTURE_CLASS: u32 = 0x011f_444c;
/// An object at `011f6394` whose word at `+0x14` (read with `00825c00`)
/// `AttachToParent` turns into the float of a record.
const COUNT_OBJECT: u32 = 0x011f_6394;
/// `Scb`: the name of the node `AttachToParent` looks for under a part (the
/// scabbard the `RemoveScabard` call is named after), and `UPB`: the key of
/// the extra data on the backpack node.
const SCABBARD_NODE_NAME: u32 = 0x0101_fa04;
const BACKPACK_KEY_NAME: u32 = 0x0101_fa00;
/// `Meshes\` and `Meshes`, the format `Data\%s\%s`, and the suffix `1st.nif`
/// of the first person model of a part.
const MESHES_PREFIX: u32 = 0x0101_6fac;
const MESHES_FOLDER: u32 = 0x0101_dccc;
const DATA_PATH_FORMAT: u32 = 0x0101_fb74;
const FIRST_PERSON_SUFFIX: u32 = 0x0101_fb80;
/// The five skin part names (`char*` each, `01188c38`) `GetSkinBipedObject`
/// compares the property's name with, and the sex names (`01199e8c`).
const SKIN_PART_NAMES: u32 = 0x0118_8c38;
const SEX_NAMES: u32 = 0x0119_9e8c;
/// The table of 0x18-byte path entries `004afa20` fills (`011c5d10`).
const PATH_TABLE: u32 = 0x011c_5d10;
/// The vtables `004afb20`, `004afb50` and `004afc20` store.
const CLONE_MAP_VTABLE: u32 = 0x0101_fb8c;
const PROCESS_MAP_VTABLE: u32 = 0x0101_fbac;
const NODE_ARRAY_VTABLE: u32 = 0x0101_fbcc;
/// The form type for which a missing parent node makes `AttachToParent` call
/// `RemoveScabard` instead of logging (what type `0x2b` is has not been
/// confirmed from the exe).
const FORM_TYPE_REMOVES_SCABBARD: u32 = 0x2b;
/// The layer `AttachToParent` expects of a weapon's collision.
const WEAPON_LAYER: u32 = 5;
/// Log formats of session 2.
const BONE_IN_PART_FORMAT: u32 = 0x0101_f8d8;
const BONE_REQUESTED_FORMAT: u32 = 0x0101_f890;
const BONE_ONLY_FORMAT: u32 = 0x0101_f840;
const EXPORTED_WRONG_FORMAT: u32 = 0x0101_f7c8;
const PARENT_EXTRA_MISSING_FORMAT: u32 = 0x0101_fa08;
const EXTRA_NOT_STRING_FORMAT: u32 = 0x0101_fa40;
const PARENT_NOT_FOUND_FORMAT: u32 = 0x0101_fa80;
const NO_HAVOK_FORMAT: u32 = 0x0101_f944;
const WEAPON_LAYER_FORMAT: u32 = 0x0101_f980;
const NO_SHAPE_FORMAT: u32 = 0x0101_f9c4;
const ADD_ON_SKINNED_FORMAT: u32 = 0x0101_fac0;
const BAD_SKIN_NAME_FORMAT: u32 = 0x0101_fb04;
const MISSING_RACE_TEXTURE_FORMAT: u32 = 0x0101_fb38;
/// The global holding the object `004af8a0` reads a `NiPointer` from (at
/// `+0x11a0`).
const TEXTURE_OWNER: u32 = 0x011d_59e8;

// ---- Session 2: callees outside this unit --------------------------------------------

/// `strcpy_s(destination, size, source)` (`00406d30`), `strcat_s` (`00406d50`)
/// and `sprintf_s(destination, size, format, ...)` (`00406d00`).
const STRING_COPY: u32 = 0x0040_6d30;
const STRING_APPEND: u32 = 0x0040_6d50;
const FORMAT_INTO: u32 = 0x0040_6d00;
/// `strrchr(text, character)` (`0040ab30`), `FileFinder::Exist(path, 0, 6, 1)`
/// (`00456a20`, Xbox PDB name), `_strncpy_s(destination, size, source,
/// count)` (`00ec8d5f`) and `_strlwr_s(text, size)` (`00ec8feb`).
const FIND_LAST_CHAR: u32 = 0x0040_ab30;
const FILE_EXISTS: u32 = 0x0045_6a20;
const STRNCPY_S: u32 = 0x00ec_8d5f;
const STRLWR_S: u32 = 0x00ec_8feb;
/// `operator delete(block)` (`00401030`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `BSUtilities::GetObjectByName(root, &fixed name, 1)` (`00c4b470`, Xbox PDB
/// name): the object named by the `NiFixedString` at `&fixed name`.
const GET_OBJECT_BY_NAME: u32 = 0x00c4_b470;
/// `0043b300(class, node)`: whether the node is of the class (a `bool`).
const IS_KIND_OF: u32 = 0x0043_b300;
/// `00453470(node)`: a test on the object at `+0x9c` of a node (which test
/// has not been confirmed).
const NODE_CHECK_9C: u32 = 0x0045_3470;
/// `0048d150(extra)`: the address of the text field (`+0xc`) of a string
/// extra data.
const STRING_EXTRA_TEXT: u32 = 0x0048_d150;
/// `0056f930(root)`: the reference whose 3D is under `root` (the Xbox name is
/// `TESObjectREFR::FindReferenceFor3D`), `004b5b20(node)`: `RemoveScabard`.
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
const REMOVE_SCABBARD: u32 = 0x004b_5b20;
/// `009cdae0(node)`: the float at `+0x64` of a node (its scale), and
/// `00440490(node, scale)`: sets it.
const NODE_SCALE: u32 = 0x009c_dae0;
const SET_NODE_SCALE: u32 = 0x0044_0490;
/// `0043d410(record, float, byte, byte)`: builds the 9-byte record
/// `AttachToParent` passes to virtual slot 0xa4 of the part.
const BUILD_RECORD: u32 = 0x0043_d410;
/// `ShadowSceneNode::AddObject(object)` (`00b5eeb0`).
const SHADOW_ADD_OBJECT: u32 = 0x00b5_eeb0;
/// `00c6c0e0(parent, 1)` (called after attaching the scabbard node).
const NOTIFY_ATTACHED: u32 = 0x00c6_c0e0;
/// `bhkWorld::KillHavok(node, 1, 1)` (`00c6a2e0`) and
/// `bhkWorld::SetMotion(node, 4, 1, 1, 1)` (`00c6a350`).
const KILL_HAVOK: u32 = 0x00c6_a2e0;
const SET_MOTION: u32 = 0x00c6_a350;
/// The weapon's collision: `bhkCollisionObject::GetbhkCollisionObject(node)`
/// (`0043b610`), `006fa820(collision)` (the word at `+0x10`),
/// `004ae6a0(body)` (the shape), `0043b4f0(body, out)` (writes the layer
/// object and returns it) and `0043b4d0(layer)` (the layer: its low seven
/// bits).
const GET_COLLISION_OBJECT: u32 = 0x0043_b610;
const COLLISION_BODY: u32 = 0x006f_a820;
const BODY_SHAPE: u32 = 0x004a_e6a0;
const COPY_LAYER: u32 = 0x0043_b4f0;
const LAYER_OF: u32 = 0x0043_b4d0;
/// `NiObjectNET::RemoveAllExtraData` (`00a5bfa0`, named by its body) and
/// `TESObjectREFR::AddMasterParticleAddonNodes(node)` (`00578060`).
const REMOVE_ALL_EXTRA_DATA: u32 = 0x00a5_bfa0;
const ADD_MASTER_PARTICLE_ADDON_NODES: u32 = 0x0057_8060;
/// `TESObjectREFR::AddAddonNodes(node)` (`00577e20`), the key of the add-on
/// extra data (`00448a80`) and the test on that extra data (`00448a40`).
const ADD_ADDON_NODES: u32 = 0x0057_7e20;
const ADDON_KEY: u32 = 0x0044_8a80;
const ADDON_EXTRA_CHECK: u32 = 0x0044_8a40;
/// The queue of the model loader: `004450c0(loader; biped, handle, first,
/// second, slot)`, and `NiPointer<QueuedFile>::operator=` (`006f74f0`).
const LOADER_QUEUE_PART: u32 = 0x0044_50c0;
const QUEUED_FILE_ASSIGN: u32 = 0x006f_74f0;
/// The FaceGen model: `BSFaceGenManager::GetAsEGMFile(out, path, -1)`
/// (`00653520`, Xbox PDB name) and `00651b50(file, 0, 0, 0, 1, -1, 0)`.
const GET_EGM_FILE: u32 = 0x0065_3520;
const FACE_GEN_LOAD: u32 = 0x0065_1b50;
/// A recursive lock keyed by thread (owner at `+0`, count at `+4`):
/// `0040fbf0(lock, 0)` takes it and `0040fba0(lock)` releases it.
const RECURSIVE_LOCK_ENTER: u32 = 0x0040_fbf0;
const RECURSIVE_LOCK_LEAVE: u32 = 0x0040_fba0;
/// Properties: `NiAVObject::GetProperty(type)` (`00a59d30`),
/// `PathingLocation::GetWorldspace` (`00441110`, a folded name) on a
/// property, `004a2020(property, bit)` and `BSShaderProperty::SetFlag(bit,
/// value)` (`00441130`).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_WORLDSPACE: u32 = 0x0044_1110;
const PROPERTY_FLAG: u32 = 0x004a_2020;
const SET_SHADER_FLAG: u32 = 0x0044_1130;
/// The race body textures: `TESRace::GetBodyTexture(race; out, out, npc, slot,
/// first person)` (`006149b0`), the race name text `00408da0(race + 0x18)`,
/// `TESActorBase::GetSex` (`005f0cc0`), `00464f30(x, _)` (returns `x`) and
/// `BSShaderManager::GetTexture(path, flag, out, 1, 0)` (`00b55840`).
const GET_BODY_TEXTURE: u32 = 0x0061_49b0;
const RACE_NAME_TEXT: u32 = 0x0040_8da0;
const ACTOR_BASE_SEX: u32 = 0x005f_0cc0;
const IDENTITY_TEXT: u32 = 0x0046_4f30;
const GET_TEXTURE: u32 = 0x00b5_5840;
/// `006a9540(object)`: the address of the field at `+0x34` (the path of a
/// texture; the same body is the node's rotation, [`NODE_ROTATION`]).
const TEXTURE_PATH_FIELD: u32 = 0x006a_9540;
/// `SetSoundFile`-named folded function (`00489100`): copies a path into a
/// 0x18-byte entry of [`PATH_TABLE`].
const SET_PATH_ENTRY: u32 = 0x0048_9100;

// ---- Small helpers --------------------------------------------------------------

/// `memset`.
fn memset(e: &mut Engine, destination: u32, value: u32, size: u32) {
    e.call(MEMSET, &args![destination, value, size]);
}

/// `NiPointer::NiPointer(T*)` on the four-byte `handle`.
fn ni_pointer_new(e: &mut Engine, handle: Ptr, value: Ptr) {
    e.call(NI_POINTER_INIT, &args![handle, value]);
}

/// `NiPointer::~NiPointer` on `handle`.
fn ni_pointer_release(e: &mut Engine, handle: Ptr) {
    e.call(NI_POINTER_RELEASE, &args![handle]);
}

/// `NiPointer::operator T*` (`00559450`): the pointer held in `handle`.
pub(crate) fn ni_pointer_get(e: &mut Engine, handle: Ptr) -> Ptr {
    e.call(READ_WORD, &args![handle]).ptr()
}

/// `NiPointer::operator=(T*)` on `handle`.
fn ni_pointer_assign(e: &mut Engine, handle: Ptr, value: Ptr) {
    e.call(NI_POINTER_SET, &args![handle, value]);
}

/// The number of children of `node` (`0043b480`).
fn child_count(e: &mut Engine, node: Ptr) -> u32 {
    e.call(CHILD_COUNT, &args![node]).u32()
}

/// The child of `node` at `index` (`0043b4a0`).
fn child_at(e: &mut Engine, node: Ptr, index: u32) -> Ptr {
    e.call(CHILD_AT, &args![node, index]).ptr()
}

/// The name text of `node` (`0043b1b0(00413f40(node))`).
fn node_name(e: &mut Engine, node: Ptr) -> Ptr {
    let field = e.call(NAME_FIELD, &args![node]).u32();
    e.call(NAME_TEXT, &args![field]).ptr()
}

/// `__RTDynamicCast(object, 0, source, target, 0)`.
fn dynamic_cast(e: &mut Engine, object: Ptr, source: u32, target: u32) -> Ptr {
    e.call(RT_DYNAMIC_CAST, &args![object, 0u32, source, target, 0u32])
        .ptr()
}

/// The base form of `requester` cast to `TESNPC` (null when it is not one).
fn requester_npc(e: &mut Engine, requester: Ptr) -> Ptr {
    let form = e.call(REFERENCE_FORM, &args![requester]).ptr();
    dynamic_cast(e, form, TYPE_TES_BOUND_OBJECT, TYPE_TES_NPC)
}

/// `PlayerCharacter::GetBiped(first)`.
fn player_biped(e: &mut Engine, first: u8) -> u32 {
    let player = e.global::<u32>(PLAYER);
    e.call(GET_BIPED, &args![player, first]).u32()
}

/// A block for a local the game keeps on its stack; `stack_free` releases it.
fn stack_alloc(e: &mut Engine, size: u32) -> Ptr {
    Ptr::new(e.mem.alloc(size))
}

fn stack_free(e: &mut Engine, block: Ptr) {
    e.mem.free(block.addr());
}

/// Whether the string setting object at `setting` holds a non-empty text.
fn setting_text_set(e: &mut Engine, setting: u32) -> bool {
    let text = e.call(SETTING_BYTE_POINTER, &args![setting]).u32();
    e.mem.u8(text) != 0
}

/// `004ae8a0(this; slot)`: whether the slot holds FaceGen data (a function
/// of this unit, translated as [`fn_004ae8a0`], called by address as session 1 wrote it).
fn is_face_gen_slot(e: &mut Engine, this: Ptr<BipedAnim>, slot: u32) -> bool {
    e.call(FN_004AE8A0, &args![this, slot]).bool()
}

/// Adds `item` to the array `array` (`NiTArray<NiPointer<NiAVObject>>::
/// Add`, `004afc50`) through a temporary `NiPointer`.
fn array_add(e: &mut Engine, array: Ptr, item: Ptr) {
    let handle = stack_alloc(e, 4);
    ni_pointer_new(e, handle, item);
    e.call(FN_004AFC50, &args![array, handle]);
    ni_pointer_release(e, handle);
    stack_free(e, handle);
}

/// Names `node` `"<slot name> <form name> (<form id>)"` (the text built with
/// `sprintf` into a `BSString`, turned into an `NiFixedString`), as the
/// weapon and the other parts are named.
fn name_part_node(e: &mut Engine, node: Ptr, form: Ptr, slot: u32) {
    let text = stack_alloc(e, 8);
    e.call(STRING_INIT, &args![text]);
    name_part_node_with(e, node, form, slot, text);
    e.call(STRING_FREE, &args![text]);
    stack_free(e, text);
}

/// [`name_part_node`] with the caller's `BSString` `text`.
fn name_part_node_with(e: &mut Engine, node: Ptr, form: Ptr, slot: u32, text: Ptr) {
    let id = e.call(FORM_ID, &args![form]).u32();
    // Virtual slot 0x130 of the form: its name.
    let name = e.vcall(form.addr(), 0x130, &[]).u32();
    let slot_name = e.global::<u32>(SLOT_NAMES + 4 * slot);
    e.call(SPRINTF, &args![text, SLOT_NAME_FORMAT, slot_name, name, id]);
    let chars = ni_pointer_get(e, text);
    let fixed = stack_alloc(e, 4);
    let fixed_name = e.call(FIXED_STRING_INIT, &args![fixed, chars]).u32();
    e.call(SET_NAME, &args![node, fixed_name]);
    e.call(FIXED_STRING_FREE, &args![fixed]);
    stack_free(e, fixed);
}

/// Tells the actor's middle-high process (`008d8520`) whether the weapon is
/// drawn, with the biped, the animation and the actor (virtual slot 0x1cc;
/// the player's animation `00950a60(1)` when `this` is the player's first
/// person biped, `008b70d0` otherwise).
fn fire_weapon_state(e: &mut Engine, this: Ptr<BipedAnim>, requester: Ptr) {
    let player = e.global::<u32>(PLAYER);
    let on_player_biped = requester.addr() == player && player_biped(e, 1) == this.addr();
    let process = e.call(SAVED_ACQUIRE_OBJECT, &args![requester]).u32();
    let animation = if on_player_biped {
        e.call(PLAYER_ANIMATION, &args![player, 1u8]).u32()
    } else {
        e.call(ACTOR_ANIMATION, &args![requester]).u32()
    };
    let drawn = e.call(IS_WEAPON_DRAWN, &args![requester]).bool();
    e.vcall(process, 0x1cc, &args![drawn, this, animation, requester]);
}

/// The part shared by `RemoveBipedWeapon` and `004ab750`: when the actor has
/// a process and neither the data handler nor the save-load object says
/// otherwise, and `this` is not the player's first person biped, the
/// actor's targets are reloaded with `flag`.
fn reload_targets_unless_blocked(
    e: &mut Engine,
    this: Ptr<BipedAnim>,
    requester: Ptr,
    process: u32,
    flag: u8,
) {
    if process == 0 {
        return;
    }
    let handler = e.global::<u32>(DATA_HANDLER);
    if e.call(DATA_HANDLER_FLAG, &args![handler]).bool() {
        return;
    }
    let save_load = e.global::<u32>(SAVE_LOAD);
    if e.call(SAVE_LOAD_FLAG, &args![save_load]).bool() {
        return;
    }
    if this.addr() != player_biped(e, 1) {
        e.call(RELOAD_TARGETS, &args![requester, flag]);
    }
}

// ---- Functions ----------------------------------------------------------------------

// Translated from 004aaca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::BipedAnim` (Xbox PDB): clears the object and the buffered
/// slots, stores the requester and, given a root node, finds the bones
/// (`InitRoot`). Returns `this`.
pub fn biped_anim_biped_anim(
    e: &mut Engine,
    this: Ptr<BipedAnim>,
    requester: Ptr,
    root: Ptr,
) -> Ptr<BipedAnim> {
    memset(e, this.addr(), 0, 0x2b4);
    memset(e, buffered(this, 0).addr(), 0, 0x140);
    e.set(this, BipedAnim::m_pRequester, requester);
    if !root.is_null() {
        biped_anim_init_root(e, this, root);
    }
    this
}

// Translated from 004aad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::InitRoot` (Xbox PDB): stores the node named `Bip01` under
/// `root`, then looks up the five bones by name; a bone found (and a node)
/// gets flag bit 0 and its node stored, one that is missing is logged.
pub fn biped_anim_init_root(e: &mut Engine, this: Ptr<BipedAnim>, root: Ptr) {
    let root_node = fn_004aae30(e, root, Ptr::new(ROOT_NODE_NAME));
    e.set(this, BipedAnim::root, root_node);
    for index in 0..BONE_COUNT {
        let name = e.global::<u32>(BONE_NAMES + 4 * index);
        let found = fn_004aae30(e, root, Ptr::new(name));
        let handle = stack_alloc(e, 4);
        ni_pointer_new(e, handle, found);
        let mut is_node = false;
        if !ni_pointer_get(e, handle).is_null() {
            let object = ni_pointer_get(e, handle);
            // Virtual slot 0xc of the object: non-zero for a node.
            if e.vcall(object.addr(), 0x0c, &[]).u32() != 0 {
                is_node = true;
            }
        }
        if is_node {
            let flags = e.get(bone(this, index), BipedBone::cFlags);
            e.set(bone(this, index), BipedBone::cFlags, flags | 1);
            let object = ni_pointer_get(e, handle);
            e.set(bone(this, index), BipedBone::pParent, object);
        } else {
            let owner = node_name(e, root);
            e.call(LOG, &args![MISSING_BONE_FORMAT, name, owner]);
        }
        ni_pointer_release(e, handle);
        stack_free(e, handle);
    }
}

// Translated from 004aae30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks up the object named `name` under `node` (`00c4b310(node, name, 1)`).
pub fn fn_004aae30(e: &mut Engine, node: Ptr, name: Ptr) -> Ptr {
    e.call(FIND_OBJECT_BY_NAME, &args![node, name, 1u32]).ptr()
}

// Translated from 004aae50 (decompiled, FalloutNV.exe 1.4.0.525)
/// A wrapper of [`biped_anim_remove_all_parts`] (`ECX` is passed on).
pub fn fn_004aae50(e: &mut Engine, this: Ptr<BipedAnim>) {
    biped_anim_remove_all_parts(e, this);
}

// Translated from 004aae70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::RemoveAllParts` (Xbox PDB): with the model loader's queue
/// lock (`004afb80` on the loader's first object) and the part lock held,
/// removes every slot and every buffered slot, clearing their forms.
pub fn biped_anim_remove_all_parts(e: &mut Engine, this: Ptr<BipedAnim>) {
    let loader = e.global::<u32>(MODEL_LOADER);
    fn_004aaef0(e, Ptr::new(loader));
    e.call(LOCK_ENTER, &args![PART_LOCK, 0u32]);
    for slot in 0..SLOT_COUNT {
        fn_004aaff0(e, this, slot, 1, Ptr::NULL);
        fn_004ab020(e, this, buffered(this, slot), 1, Ptr::NULL);
    }
    e.call(LOCK_LEAVE, &args![PART_LOCK]);
    let loader = e.global::<u32>(MODEL_LOADER);
    fn_004aaf10(e, Ptr::new(loader));
}

// Translated from 004aaef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `004afb80` on the object the `ModelLoader` holds in its first
/// field.
pub fn fn_004aaef0(e: &mut Engine, this: Ptr) {
    let first = e.mem.u32(this.addr());
    e.call(FN_004AFB80, &args![first]);
}

// Translated from 004aaf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `004afba0` on the object the `ModelLoader` holds in its first
/// field.
pub fn fn_004aaf10(e: &mut Engine, this: Ptr) {
    let first = e.mem.u32(this.addr());
    e.call(FN_004AFBA0, &args![first]);
}

// Translated from 004aaf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The slot whose form is `form`: its address, or null (also for a null
/// form).
pub fn fn_004aaf30(e: &mut Engine, this: Ptr<BipedAnim>, form: Ptr) -> Ptr<BipedObject> {
    if form.is_null() {
        return Ptr::NULL;
    }
    for slot in 0..SLOT_COUNT {
        let candidate = object(this, slot);
        if e.get(candidate, BipedObject::pParent) == form {
            return candidate;
        }
    }
    Ptr::NULL
}

// Translated from 004aaf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears every slot whose form is `form` (`004aaff0(slot, 1, 0)`).
pub fn fn_004aaf90(e: &mut Engine, this: Ptr<BipedAnim>, form: Ptr) {
    if form.is_null() {
        return;
    }
    for slot in 0..SLOT_COUNT {
        if e.get(object(this, slot), BipedObject::pParent) == form {
            fn_004aaff0(e, this, slot, 1, Ptr::NULL);
        }
    }
}

// Translated from 004aaff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_004ab020`] on slot `slot`.
pub fn fn_004aaff0(e: &mut Engine, this: Ptr<BipedAnim>, slot: u32, clear: u8, new_part: Ptr) {
    fn_004ab020(e, this, object(this, slot), clear, new_part);
}

// Translated from 004ab020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the slot `entry`: when it holds a loaded clone, removes the
/// decal data from the weapon bone, detaches the clone (at once, or queued
/// when `008c7aa0` says detaching must be deferred), releases the model with
/// the loader and clears the clone. With `clear` non-zero the form is
/// cleared too and the part replaced by `new_part`.
pub fn fn_004ab020(
    e: &mut Engine,
    this: Ptr<BipedAnim>,
    entry: Ptr<BipedObject>,
    clear: u8,
    new_part: Ptr,
) {
    let clone = e.get(entry, BipedObject::pPartClone);
    if !clone.is_null() {
        biped_anim_recursively_remove_all_decal_placement_vector_data_from_weapon_bone(
            e, this, clone,
        );
        if !e.call(MUST_DEFER, &[]).bool() {
            biped_anim_run_biped_3d_detach(e, clone);
        } else {
            let queue = e.call(TASK_QUEUE_GETTER, &[]).u32();
            e.call(TASK_QUEUE_DETACH, &args![queue, clone]);
        }
        let part = e.get(entry, BipedObject::pPart);
        // Virtual slot 0x14 of the model: its path.
        let path = e.vcall(part.addr(), 0x14, &[]).u32();
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(RELEASE_MODEL, &args![loader, path]);
        e.set(entry, BipedObject::pPartClone, Ptr::NULL);
    }
    if clear != 0 {
        e.set(entry, BipedObject::pParent, Ptr::NULL);
        e.set(entry, BipedObject::pPart, new_part);
    }
}

// Translated from 004ab0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::RunBiped3DDetach` (Xbox PDB): detaches `node`: prepares it
/// (`004b6ce0`), removes it from the first shadow scene node and, when it has
/// a parent, removes its physics objects and the node from the parent
/// (virtual slot 0xe8 of the parent).
pub fn biped_anim_run_biped_3d_detach(e: &mut Engine, node: Ptr) {
    if node.is_null() {
        return;
    }
    e.call(PREPARE_DETACH, &args![node]);
    let shadow = e.call(SHADOW_SCENE_NODE, &args![0u32]).u32();
    e.call(SHADOW_REMOVE_OBJECT, &args![shadow, node]);
    if e.call(NODE_PARENT, &args![node]).u32() != 0 {
        e.call(WORLD_REMOVE_OBJECTS, &args![node, 1u32, 0u32]);
        let parent = e.call(NODE_PARENT, &args![node]).u32();
        e.vcall(parent, 0xe8, &args![node]);
    }
}

// Translated from 004ab130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::RecursivelyRemoveAllDecalPlacementVectorDataFromWeaponBone`
/// (Xbox PDB): looks up the extra data the weapon bone (bone 1) keeps under
/// the key at `011c61e4`; for a geometry node (virtual slot 0x1c) that has
/// data (`0043fad0`) and is found in that extra data's list (at `+0x34`),
/// removes the extra data from the bone; otherwise recurses through the
/// node's children (virtual slot 0xc gives the node).
pub fn biped_anim_recursively_remove_all_decal_placement_vector_data_from_weapon_bone(
    e: &mut Engine,
    this: Ptr<BipedAnim>,
    node: Ptr,
) {
    if node.is_null() {
        return;
    }
    let weapon_bone = fn_004ab230(e, this, 1);
    let key = fn_004ab220(e);
    let extra = e.call(GET_EXTRA_DATA, &args![weapon_bone, key]).ptr::<()>();
    if extra.is_null() {
        return;
    }
    if e.vcall(node.addr(), 0x1c, &[]).u32() != 0 {
        if e.call(NODE_DATA_POINTER, &args![node]).u32() != 0 {
            let key_slot = stack_alloc(e, 4);
            e.mem.set_u32(key_slot.addr(), node.addr());
            let found = e
                .call(FIND_IN_LIST, &args![extra.byte_add(0x34), key_slot, 0u32])
                .u32();
            stack_free(e, key_slot);
            if found != 0 {
                let key = fn_004ab220(e);
                e.call(REMOVE_EXTRA_DATA, &args![weapon_bone, key]);
            }
        }
    } else {
        let children = e.vcall(node.addr(), 0x0c, &[]).ptr::<()>();
        if !children.is_null() {
            let mut index = 0;
            while index < child_count(e, children) {
                let child = child_at(e, children, index);
                biped_anim_recursively_remove_all_decal_placement_vector_data_from_weapon_bone(
                    e, this, child,
                );
                index += 1;
            }
        }
    }
}

// Translated from 004ab220 (decompiled, FalloutNV.exe 1.4.0.525)
/// The key of the extra data kept on the weapon bone (the global at
/// `011c61e4`).
pub fn fn_004ab220(e: &mut Engine) -> Ptr {
    Ptr::new(e.global::<u32>(EXTRA_DATA_KEY))
}

// Translated from 004ab230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The node of bone `index` (`bone[index].pParent`).
pub fn fn_004ab230(e: &mut Engine, this: Ptr<BipedAnim>, index: u32) -> Ptr {
    e.get(bone(this, index), BipedBone::pParent)
}

// Translated from 004ab250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the slots and puts `form` into the three skin slots. When no
/// buffered slot is in use the slots are first copied to the buffered ones
/// (slot by slot for an actor whose virtual slot 0x1d0 is non-zero, the
/// slots are then cleared one by one; otherwise both arrays are cleared at
/// once). For each of the three skin slots that has no bone or whose bone
/// was found, the item `006151f0` gives for `form` is assigned by
/// `004abad0`.
pub fn fn_004ab250(e: &mut Engine, this: Ptr<BipedAnim>, form: Ptr, argument: u32) {
    if form.is_null() {
        return;
    }
    let mut buffer_empty = true;
    for slot in 0..SLOT_COUNT {
        if !e.get(buffered(this, slot), BipedObject::pParent).is_null() {
            buffer_empty = false;
        }
    }
    let requester = e.get(this, BipedAnim::m_pRequester);
    if !requester.is_null() && e.vcall(requester.addr(), 0x1d0, &[]).u32() != 0 {
        for slot in 0..SLOT_COUNT {
            if buffer_empty {
                let from = object(this, slot);
                let to = buffered(this, slot);
                for word in 0..4 {
                    let value = e.mem.u32(from.addr() + 4 * word);
                    e.mem.set_u32(to.addr() + 4 * word, value);
                }
            }
            memset(e, object(this, slot).addr(), 0, 0x10);
        }
    } else {
        if buffer_empty {
            memset(e, buffered(this, 0).addr(), 0, 0x140);
        }
        memset(e, object(this, 0).addr(), 0, 0x140);
    }
    for index in 0..3 {
        let slot = e.global::<u32>(SKIN_SLOTS + 4 * index);
        let bone_index = e.global::<u32>(SLOT_BONES + 4 * slot);
        if bone_index == 0xffff_ffff || e.get(bone(this, bone_index), BipedBone::cFlags) & 1 != 0 {
            let slot = e.global::<u32>(SKIN_SLOTS + 4 * index);
            let item = e.call(FORM_SLOT_ITEM, &args![form, argument, slot]).ptr();
            fn_004abad0(e, this, form, item, slot);
        }
    }
}

// Translated from 004ab400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Chooses the weapon form and model for slot 5: for a weapon form (type
/// 0x28) that is not already loaded, clears slot 5 and stores the form and
/// the model `00522df0` gives; for the player's biped (first or third
/// person) the model is the weapon's animation model (`+0x30` of the item
/// `008d8b00` gives, or of the item `004ab500` gives for the flags when the
/// form id is not `0x1735d4`).
pub fn fn_004ab400(e: &mut Engine, this: Ptr<BipedAnim>, form: Ptr, flags: u8) {
    if form.is_null() {
        return;
    }
    if e.call(FORM_TYPE, &args![form]).u32() != FORM_TYPE_WEAPON {
        return;
    }
    let weapon = object(this, WEAPON_SLOT);
    if form == e.get(weapon, BipedObject::pParent)
        && !e.get(weapon, BipedObject::pPartClone).is_null()
    {
        return;
    }
    fn_004aaff0(e, this, WEAPON_SLOT, 1, Ptr::NULL);
    let mut model = e.call(FORM_MODEL, &args![form, flags, 1u32]).ptr::<()>();
    let on_player_biped = this.addr() == player_biped(e, 1) || this.addr() == player_biped(e, 0);
    if on_player_biped {
        let mut anims = e.call(FORM_ANIMS, &args![form]).ptr::<()>();
        if flags != 0 && e.call(FORM_ID, &args![form]).u32() != FORM_ID_SPECIAL_WEAPON {
            let chosen = fn_004ab500(e, form, flags);
            if chosen != 0 {
                anims = Ptr::new(chosen);
            }
        }
        if !anims.is_null() {
            model = anims.byte_add(0x30);
        }
    }
    e.set(weapon, BipedObject::pParent, form);
    e.set(weapon, BipedObject::pPart, model);
}

// Translated from 004ab500 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word of the weapon form `form` that the low three bits of `flags`
/// select among the eight words from `+0x250` to `+0x26c` (bit 0 and bit 1
/// and bit 2 each pick a variant).
pub fn fn_004ab500(e: &mut Engine, form: Ptr, flags: u8) -> u32 {
    let offset = match (flags & 1 != 0, flags & 2 != 0, flags & 4 != 0) {
        (false, false, false) => 0x250,
        (false, false, true) => 0x25c,
        (false, true, false) => 0x258,
        (false, true, true) => 0x264,
        (true, false, false) => 0x254,
        (true, false, true) => 0x268,
        (true, true, false) => 0x260,
        (true, true, true) => 0x26c,
    };
    e.mem.u32(form.addr() + offset)
}

// Translated from 004ab5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::RemoveBipedWeapon` (Xbox PDB): lets go of the weapon: clears
/// the weapon flag of the clone of slot 7's part when slot 5's form has the
/// flag, empties the buffered slots holding the buffered weapon form and
/// slot 5, and, for an actor (virtual slot 0x100), reloads its targets and
/// removes the backpack node from its parent.
pub fn biped_anim_remove_biped_weapon(e: &mut Engine, this: Ptr<BipedAnim>) {
    let weapon_form = e.get(object(this, WEAPON_SLOT), BipedObject::pParent);
    if !weapon_form.is_null() && fn_004ab730(e, weapon_form) {
        let clone = e.get(object(this, 7), BipedObject::pPartClone);
        if !clone.is_null() {
            e.call(SET_WEAPON_FLAG, &args![clone, 0u32]);
        }
    }
    let buffered_form = e.get(buffered(this, WEAPON_SLOT), BipedObject::pParent);
    if !buffered_form.is_null() {
        for slot in 0..SLOT_COUNT {
            if e.get(buffered(this, slot), BipedObject::pParent) == buffered_form {
                fn_004ab020(e, this, buffered(this, slot), 1, Ptr::NULL);
            }
        }
    }
    fn_004aaff0(e, this, WEAPON_SLOT, 1, Ptr::NULL);
    let requester = e.get(this, BipedAnim::m_pRequester);
    if e.vcall(requester.addr(), 0x100, &[]).bool() {
        let process = e.call(SAVED_ACQUIRE_OBJECT, &args![requester]).u32();
        reload_targets_unless_blocked(e, this, requester, process, 0);
        let root = e.get(this, BipedAnim::root);
        let backpack = fn_004aae30(e, root, Ptr::new(BACKPACK_NAME));
        if !backpack.is_null() {
            let parent = e.call(NODE_PARENT, &args![backpack]).u32();
            e.vcall(parent, 0xe8, &args![backpack]);
        }
    }
}

// Translated from 004ab730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 4 of the byte at `+0x100` of `form` is set.
pub fn fn_004ab730(e: &mut Engine, form: Ptr) -> bool {
    e.mem.u8(form.addr() + 0x100) & 0x10 != 0
}

// Translated from 004ab750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the weapon `form` into slot 5: for a weapon form not already
/// loaded, chooses the form and model (`004ab400`), loads and attaches it
/// (`004aeed0`; for the player's own actor with the node `00950bb0` gives),
/// names the loaded node `"<slot name> <form name> (<form id>)"`, applies
/// the weapon flag and, for an actor, reloads its targets, blends out the
/// animation and tells the middle-high process the weapon state (virtual
/// slot 0x1cc).
pub fn fn_004ab750(e: &mut Engine, this: Ptr<BipedAnim>, form: Ptr, flags: u8) {
    if form.is_null() {
        return;
    }
    if e.call(FORM_TYPE, &args![form]).u32() != FORM_TYPE_WEAPON {
        return;
    }
    let weapon = object(this, WEAPON_SLOT);
    if form == e.get(weapon, BipedObject::pParent)
        && !e.get(weapon, BipedObject::pPartClone).is_null()
    {
        return;
    }
    fn_004ab400(e, this, form, flags);
    let requester = e.get(this, BipedAnim::m_pRequester);
    let player = e.global::<u32>(PLAYER);
    let weapon_form = e.get(weapon, BipedObject::pParent);
    let weapon_model = e.get(weapon, BipedObject::pPart);
    let node = if requester.addr() == player {
        let first_person = e.call(IS_FIRST_PERSON_BIPED, &args![player, this]).u8();
        e.call(PLAYER_NODE, &args![player, first_person]).u32()
    } else {
        0
    };
    let loaded = e
        .call(
            FN_004AEED0,
            &args![weapon_form, weapon_model, WEAPON_SLOT, requester, node],
        )
        .ptr::<()>();
    e.set(weapon, BipedObject::pPartClone, loaded);
    if loaded.is_null() {
        return;
    }
    let weapon_form = e.get(weapon, BipedObject::pParent);
    name_part_node(e, loaded, weapon_form, WEAPON_SLOT);
    let weapon_clone = e.get(object(this, 7), BipedObject::pPartClone);
    if fn_004ab730(e, form) && !weapon_clone.is_null() {
        e.call(SET_WEAPON_FLAG, &args![weapon_clone, 1u32]);
    }
    if e.vcall(requester.addr(), 0x100, &[]).bool() {
        let process = e.call(SAVED_ACQUIRE_OBJECT, &args![requester]).u32();
        reload_targets_unless_blocked(e, this, requester, process, 1);
        let flags_object = e.global::<u32>(FLAGS_OBJECT);
        if process != 0 && !e.call(FLAGS_OBJECT_FLAG, &args![flags_object]).bool() {
            let on_player_biped = requester.addr() == player && player_biped(e, 1) == this.addr();
            let animation = if on_player_biped {
                e.call(PLAYER_ANIMATION, &args![player, 1u8]).u32()
            } else {
                e.call(ACTOR_ANIMATION, &args![requester]).u32()
            };
            if animation != 0 {
                e.call(BLEND_OUT, &args![animation, 4u32, 0u32]);
            }
            fire_weapon_state(e, this, requester);
        }
    }
}

// Translated from 004abad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `form` (with its item `item`) to biped slots. A form that is a
/// biped model (`GetFormAsBipedModel`) is put into the first slot it fills
/// (`FillsBipedSlot(slot, 1, 0)`), after the slots it fills that hold a
/// different form are emptied (by slot for a race, by form otherwise); a
/// race (`TESRace`) goes into slot `slot` alone. For the player's first
/// person biped the item may be replaced by the one `004afa20` gives for the
/// slot (when `004af950` says so).
pub fn fn_004abad0(e: &mut Engine, this: Ptr<BipedAnim>, form: Ptr, item: Ptr, slot: u32) {
    let mut item = item;
    let biped_model = e.call(GET_FORM_AS_BIPED_MODEL, &args![form]).ptr::<()>();
    let race = dynamic_cast(e, form, TYPE_TES_FORM, TYPE_TES_RACE);
    let buffer = stack_alloc(e, 0x104);
    let mut replace_item = false;
    if !item.is_null() {
        let player = e.global::<u32>(PLAYER);
        if e.call(IS_FIRST_PERSON_BIPED, &args![player, this]).bool() {
            replace_item = e.call(FN_004AF950, &args![this, item, buffer]).bool();
        }
    }
    if !biped_model.is_null() {
        for index in 0..SLOT_COUNT {
            let fills = e
                .call(FILLS_BIPED_SLOT, &args![biped_model, index, 0u32, 0u32])
                .bool();
            if fills && e.get(object(this, index), BipedObject::pParent) != form {
                let current = e.get(object(this, index), BipedObject::pParent);
                if !current.is_null()
                    && !dynamic_cast(e, current, TYPE_TES_FORM, TYPE_TES_RACE).is_null()
                {
                    fn_004aaff0(e, this, index, 1, Ptr::NULL);
                } else {
                    fn_004aaf90(e, this, current);
                }
            }
        }
        for index in 0..SLOT_COUNT {
            if e.call(FILLS_BIPED_SLOT, &args![biped_model, index, 1u32, 0u32])
                .bool()
            {
                if replace_item {
                    item = e.call(FN_004AFA20, &args![this, index, buffer]).ptr();
                }
                e.set(object(this, index), BipedObject::pParent, form);
                e.set(object(this, index), BipedObject::pPart, item);
                break;
            }
        }
    } else if !race.is_null() {
        fn_004aaff0(e, this, slot, 1, Ptr::NULL);
        if replace_item {
            item = e.call(FN_004AFA20, &args![this, slot, buffer]).ptr();
        }
        e.set(object(this, slot), BipedObject::pParent, form);
        e.set(object(this, slot), BipedObject::pPart, item);
    }
    stack_free(e, buffer);
}

// Translated from 004abd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues the models and textures of the parts with the model loader. For
/// each slot with a part, adds its part flags (`004abf80`) to a mask; when
/// the slot is a FaceGen slot (`004ae8a0`) and the settings ask for it the
/// FaceGen files are queued (`004aee60`), otherwise the part's model (and
/// the player's alternative `004afa50` gives) and the biped model's own path
/// are queued. Finally the object `0092c820` is told and the skin textures
/// of the mask are queued (`004abfe0`). `first` and `third` are the two
/// arguments passed on to every queueing call, `second` the object told at
/// the end.
pub fn fn_004abd30(e: &mut Engine, this: Ptr<BipedAnim>, first: u32, second: Ptr, third: u32) {
    let object_state = stack_alloc(e, 4);
    e.call(QUEUE_OBJECT_INIT, &args![object_state, 0u32]);
    let mut flags = 0u32;
    let requester = e.get(this, BipedAnim::m_pRequester);
    let npc = requester_npc(e, requester);
    if !npc.is_null() {
        for slot in 0..SLOT_COUNT {
            let part = e.get(object(this, slot), BipedObject::pPart);
            if part.is_null() || part.addr() == 0xffff_ffff {
                continue;
            }
            flags |= fn_004abf80(e, part);
            if is_face_gen_slot(e, this, slot)
                && setting_text_set(e, SETTING_FACE)
                && e.call(SETTING_BYTE_VALUE, &[]).bool()
                && fn_004abfa0(e)
            {
                e.call(FN_004AEE60, &args![this, first, third, object_state, slot]);
                continue;
            }
            let loader = e.global::<u32>(MODEL_LOADER);
            if requester.addr() == e.global::<u32>(PLAYER) {
                let alternative = e.call(FN_004AFA50, &args![this, slot, part]).u32();
                if alternative != part.addr() {
                    e.call(
                        QUEUE_MODEL_FORM,
                        &args![loader, alternative, first, third, 3u32, 1u32, 0u32, 0u32],
                    );
                }
            }
            e.call(
                QUEUE_MODEL_FORM,
                &args![loader, part, first, third, 3u32, 1u32, 0u32, 0u32],
            );
            let form = e.get(object(this, slot), BipedObject::pParent);
            let biped_model = e.call(GET_FORM_AS_BIPED_MODEL, &args![form]).u32();
            if biped_model != 0 {
                let path = e.call(BIPED_MODEL_PATH, &args![biped_model]).u32();
                if e.mem.i8(path) != 0 {
                    e.call(
                        QUEUE_MODEL,
                        &args![loader, path, first, third, 3u32, 1u32, 0u32, 0u32],
                    );
                }
            }
        }
    }
    e.call(QUEUE_OBJECT_USE, &args![second, object_state]);
    fn_004abfe0(e, this, flags, first, third);
    e.call(QUEUE_OBJECT_FREE, &args![object_state]);
    stack_free(e, object_state);
}

// Translated from 004abf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bits 1 to 3 of the byte at `+0x14` of `item`.
pub fn fn_004abf80(e: &mut Engine, item: Ptr) -> u32 {
    (e.mem.u8(item.addr() + 0x14) & 0x0e) as u32
}

// Translated from 004abfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the two settings at `011d5a10` and `011d5a44` both hold a
/// non-empty text.
pub fn fn_004abfa0(e: &mut Engine) -> bool {
    setting_text_set(e, SETTING_PAIR_FIRST) && setting_text_set(e, SETTING_PAIR_SECOND)
}

// Translated from 004abfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues the skin textures for the parts in `mask` (bit 1: slot 2, bit 2:
/// slot 4, bit 3: slot 3) and the race's body modification texture, for the
/// requester's NPC.
pub fn fn_004abfe0(e: &mut Engine, this: Ptr<BipedAnim>, mask: u32, first: u32, second: u32) {
    if mask == 0 {
        return;
    }
    let requester = e.get(this, BipedAnim::m_pRequester);
    let npc = requester_npc(e, requester);
    if npc.is_null() {
        return;
    }
    if mask & 2 != 0 {
        biped_anim_queue_skin_texture(e, this, npc, 2, first, second);
    }
    if mask & 4 != 0 {
        biped_anim_queue_skin_texture(e, this, npc, 4, first, second);
    }
    if mask & 8 != 0 {
        biped_anim_queue_skin_texture(e, this, npc, 3, first, second);
    }
    let name = stack_alloc(e, 0x104);
    let file = stack_alloc(e, 0x104);
    let race = fn_004ac110(e, npc);
    e.call(BODY_MOD_TEXTURE_NAME, &args![race, npc, name]);
    let race = fn_004ac110(e, npc);
    if e.call(BODY_MOD_TEXTURE_FILE_NAME, &args![race, npc, name, file])
        .bool()
    {
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(QUEUE_TEXTURE, &args![loader, file, first, second]);
    }
    stack_free(e, file);
    stack_free(e, name);
}

// Translated from 004ac110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x110` of the NPC `npc` (its race, `this` of the `TESRace`
/// methods), read with `00726070` on `npc + 0x10c`.
pub fn fn_004ac110(e: &mut Engine, npc: Ptr) -> u32 {
    e.call(READ_WORD_PLUS_4, &args![npc.byte_add(0x10c)]).u32()
}

// Translated from 004ac130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::QueueSkinTexture` (Xbox PDB): when the NPC's race has a skin
/// texture for `slot` (`00614640`), queues it with the model loader and the
/// matching normal map (the path `GetModifiedTextureFilename` gives for the
/// suffix `_n`, when it is not empty).
pub fn biped_anim_queue_skin_texture(
    e: &mut Engine,
    _this: Ptr<BipedAnim>,
    npc: Ptr,
    slot: u32,
    first: u32,
    second: u32,
) {
    let path = stack_alloc(e, 0x104);
    let normal = stack_alloc(e, 0x104);
    let race = fn_004ac110(e, npc);
    if e.call(SKIN_TEXTURE_PATH, &args![race, npc, slot, path])
        .bool()
    {
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(QUEUE_TEXTURE, &args![loader, path, first, second]);
        e.call(
            MODIFIED_TEXTURE_FILENAME,
            &args![normal, path, NORMAL_MAP_SUFFIX, 1u32],
        );
        if e.mem.i8(normal.addr()) != 0 {
            e.call(QUEUE_TEXTURE, &args![loader, normal, first, second]);
        }
    }
    stack_free(e, normal);
    stack_free(e, path);
}

// Translated from 004ac1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::LoadBipedParts` (Xbox PDB): for each of the 20 slots, brings
/// the loaded 3D in line with the slot's form and model. A slot whose form
/// and part equal the buffered slot's (and that has a buffered clone) takes
/// the buffered clone over; the other slots are loaded: the file is loaded
/// with the model loader and cloned, textures swapped, skinned or attached
/// to its bone or the root, named, its add-on nodes added and limbs hidden
/// as dismembered. Slot 5 (the weapon) is loaded through `004ab750`
/// instead. With `flag` zero a FaceGen slot may be handled by `004aede0`.
/// The actor's 3D properties are updated at the end.
pub fn biped_anim_load_biped_parts(e: &mut Engine, this: Ptr<BipedAnim>, flag: u8) {
    let requester = e.get(this, BipedAnim::m_pRequester);
    let player = e.global::<u32>(PLAYER);
    let mut first_person = false;
    let has_slot_6 = !e.get(object(this, 6), BipedObject::pParent).is_null();
    if requester.addr() == player && e.call(IS_FIRST_PERSON_BIPED, &args![player, this]).bool() {
        first_person = true;
    }
    if e.global::<u8>(PALETTE_FLAG) != 0 {
        let form = e.call(REFERENCE_FORM, &args![requester]).u32();
        e.call(CLEAR_BODY_PALETTE, &args![form]);
    }
    for slot in 0..SLOT_COUNT {
        load_parts_step(e, this, flag, slot, first_person, has_slot_6);
    }
    let requester = e.get(this, BipedAnim::m_pRequester);
    let root = e.call(ACTOR_ROOT, &args![requester]).u32();
    e.call(UPDATE_PROPERTIES, &args![root]);
}

/// One pass of `LoadBipedParts`' loop (slot `slot`).
fn load_parts_step(
    e: &mut Engine,
    this: Ptr<BipedAnim>,
    flag: u8,
    slot: u32,
    first_person: bool,
    has_slot_6: bool,
) {
    let current = object(this, slot);
    let previous = buffered(this, slot);
    let part = e.get(current, BipedObject::pPart);
    let reuse = !part.is_null()
        && part == e.get(previous, BipedObject::pPart)
        && e.get(current, BipedObject::pParent) == e.get(previous, BipedObject::pParent)
        && !e.get(previous, BipedObject::pPartClone).is_null();
    if reuse {
        // The buffered slot becomes the slot, the buffered slot is cleared.
        for word in 0..4 {
            let value = e.mem.u32(previous.addr() + 4 * word);
            e.mem.set_u32(current.addr() + 4 * word, value);
        }
        memset(e, previous.addr(), 0, 0x10);
        let clone = e.get(current, BipedObject::pPartClone);
        if e.global::<u8>(PALETTE_FLAG) != 0 && !clone.is_null() {
            // Virtual slot 0xc of the clone: its node.
            let node = e.vcall(clone.addr(), 0x0c, &[]).ptr::<()>();
            if !node.is_null() {
                let mut index = 0;
                while index < child_count(e, node) {
                    let child = child_at(e, node, index);
                    e.call(FN_004AF490, &args![this, child, 0u32]);
                    index += 1;
                }
            } else {
                e.call(FN_004AF490, &args![this, clone, 0u32]);
            }
        }
        let weapon_form = e.get(object(this, WEAPON_SLOT), BipedObject::pParent);
        if slot == 7 && !weapon_form.is_null() && fn_004ab730(e, weapon_form) {
            let clone = e.get(current, BipedObject::pPartClone);
            if !clone.is_null() {
                e.call(SET_WEAPON_FLAG, &args![clone, 1u32]);
            }
        }
        return;
    }
    if !e.get(previous, BipedObject::pPartClone).is_null() {
        fn_004ab020(e, this, previous, 1, Ptr::NULL);
    } else {
        memset(e, previous.addr(), 0, 0x10);
    }
    load_part(e, this, flag, slot, first_person, has_slot_6);
}

/// The second half of one pass of `LoadBipedParts`' loop: loads the part of
/// slot `slot` (after the buffered slot was dealt with).
fn load_part(
    e: &mut Engine,
    this: Ptr<BipedAnim>,
    flag: u8,
    slot: u32,
    first_person: bool,
    has_slot_6: bool,
) {
    let current = object(this, slot);
    let requester = e.get(this, BipedAnim::m_pRequester);
    let player = e.global::<u32>(PLAYER);
    let form = e.get(current, BipedObject::pParent);
    if slot == WEAPON_SLOT && !form.is_null() {
        if requester.addr() == player {
            let process = e.call(SAVED_ACQUIRE_OBJECT, &args![player]).u32();
            // Virtual slot 0x148 of the process: the item change of the
            // equipped weapon.
            let item_change = e.vcall(process, 0x148, &[]).u32();
            let mod_slots = e.call(GET_MOD_SLOTS, &args![item_change]).u8();
            let effect = stack_alloc(e, 4);
            e.mem.set_f32(effect.addr(), 0.0);
            let has_effect = e
                .call(HAS_MOD_EFFECT_ACTIVE, &args![item_change, 0x0eu32, effect])
                .bool();
            stack_free(e, effect);
            fn_004ab750(e, this, form, mod_slots);
            if fn_004ad010(e, form)
                && player_biped(e, 0) == this.addr()
                && (!fn_004ad030(e, form) || has_effect)
            {
                let value = e.call(WEAPON_MODEL_VALUE, &args![form]).u32();
                e.call(SHOW_WEAPON, &args![value]);
            }
        } else {
            e.call(ACTOR_SET_WEAPON, &args![requester, form]);
        }
        return;
    }
    let part = e.get(current, BipedObject::pPart);
    if part.is_null() || part.addr() == 0xffff_ffff {
        return;
    }
    if first_person && !(2..=6).contains(&slot) {
        return;
    }
    if flag == 0
        && is_face_gen_slot(e, this, slot)
        && requester.addr() != player
        && setting_text_set(e, SETTING_FACE)
        && e.call(SETTING_BYTE_VALUE, &[]).bool()
        && fn_004abfa0(e)
    {
        e.call(FN_004AEDE0, &args![this, 0u32, 0u32, slot]);
        return;
    }
    if e.call(MODEL_HAS_PATH, &args![part]).u32() == 0 {
        return;
    }
    let bone_index = e.global::<u32>(SLOT_BONES + 4 * slot);
    let face_slot = is_face_gen_slot(e, this, slot);
    // Virtual slot 0x14 of the model: its path.
    let path = e.vcall(part.addr(), 0x14, &[]).u32();
    let loader = e.global::<u32>(MODEL_LOADER);
    let file = e
        .call(
            LOAD_FILE,
            &args![loader, path, 3u32, 1u32, 0u32, face_slot, 0u32],
        )
        .ptr::<()>();
    let cloning = stack_alloc(e, 0x1c);
    fn_004ad050(e, cloning, 1.0);
    let handle = stack_alloc(e, 4);
    ni_pointer_new(e, handle, Ptr::NULL);
    let copy = if e.call(HAS_MORPHER_CONTROLLER, &args![file]).bool() {
        let tes = e.global::<u32>(TES_GLOBAL);
        let deep = e
            .call(DEEP_COPY_SAME_TEXTURES, &args![tes, file, cloning])
            .ptr();
        ni_pointer_assign(e, handle, deep);
        ni_pointer_get(e, handle)
    } else {
        e.call(NI_OBJECT_CLONE, &args![file, cloning]).ptr()
    };
    // Virtual slot 0xc of the copy: its node.
    let node = e.vcall(copy.addr(), 0x0c, &[]).ptr::<()>();
    let mut result = Ptr::NULL;
    if !node.is_null() {
        let part = e.get(current, BipedObject::pPart);
        let swap = if part.is_null() {
            0
        } else {
            e.vcall(part.addr(), 0x1c, &[]).u32()
        };
        if swap != 0 {
            e.call(SWAP_TEXTURES, &args![swap, node]);
        }
        let parent_form = e.get(current, BipedObject::pParent);
        if !parent_form.is_null() && e.vcall(parent_form.addr(), 0xac, &[]).bool() {
            e.call(SWAP_PLATFORM_TEXTURES, &args![node]);
        }
        // Virtual slot 0x10 of the node: the fade node, if it is one.
        let fade = e.vcall(node.addr(), 0x10, &[]).u32();
        if fade != 0 {
            e.call(SET_LOD_MULT_TYPE, &args![fade, 7u32]);
        }
        e.call(SET_TRANSLATION, &args![node, ZERO_TRANSLATION]);
        e.call(SET_ROTATION, &args![node, IDENTITY_ROTATION]);
        result = biped_anim_apply_skinned_objects(
            e,
            this,
            node,
            slot,
            first_person as u8,
            has_slot_6 as u8,
            Ptr::NULL,
        );
        if result.is_null() {
            result = node;
            if !this.is_null() && is_face_gen_slot(e, this, slot) {
                let face = e.call(FN_004AE790, &args![this, slot]).ptr::<()>();
                if !face.is_null() {
                    let mut index = 0;
                    while index < child_count(e, node) {
                        let child = child_at(e, node, index);
                        // Virtual slot 0x1c of the child: its geometry.
                        let geometry = e.vcall(child.addr(), 0x1c, &[]).ptr::<()>();
                        if !child.is_null() && geometry.is_null() {
                            let part = e.get(current, BipedObject::pPart);
                            let name = e.vcall(part.addr(), 0x14, &[]).u32();
                            e.call(LOG, &args![NON_GEOMETRY_FORMAT, name]);
                        }
                        if !geometry.is_null()
                            && e.call(NODE_SKIN_DATA, &args![geometry]).u32() != 0
                            && !requester.is_null()
                            && e.call(REFERENCE_FORM, &args![requester]).u32() != 0
                        {
                            let base = e.call(REFERENCE_FORM, &args![requester]);
                            if e.call(FORM_TYPE, &args![base.u32()]).u32() == FORM_TYPE_NPC {
                                apply_face_gen(e, requester, face, geometry);
                            }
                        }
                        index += 1;
                    }
                }
            }
            let root = e.get(this, BipedAnim::root);
            e.call(FN_004AE250, &args![root, node, 0u32, this, slot, 0u32]);
            let save_load = e.global::<u32>(SAVE_LOAD);
            if !e.call(SAVE_LOAD_FLAG, &args![save_load]).bool() && slot == WEAPON_SLOT {
                let requester = e.get(this, BipedAnim::m_pRequester);
                let process = e.call(SAVED_ACQUIRE_OBJECT, &args![requester]).u32();
                if process != 0 {
                    fire_weapon_state(e, this, requester);
                }
            }
        }
    }
    if !result.is_null() {
        finish_part(e, this, slot, node, file, result, bone_index);
    }
    ni_pointer_release(e, handle);
    stack_free(e, handle);
    ni_cloning_process_destructor(e, cloning.cast());
    stack_free(e, cloning);
}

/// The FaceGen block of `LoadBipedParts`: a copy of the geometry's skin data
/// (`005495f0`) is made and set on the geometry (virtual slot 0xe4), the
/// NPC's face coordinates are fetched and applied to the FaceGen model
/// `face`; when they are (or the setting `0043faf0` is off) the geometry's
/// rotation is turned by a quarter turn about Y.
fn apply_face_gen(e: &mut Engine, requester: Ptr, face: Ptr, geometry: Ptr) {
    let npc = e.call(REFERENCE_FORM, &args![requester]).ptr::<()>();
    let coords = stack_alloc(e, 0x80);
    e.call(
        VECTOR_CONSTRUCT,
        &args![coords, 0x20u32, 4u32, FACE_COORD_INIT, FACE_COORD_FREE],
    );
    let first = stack_alloc(e, 4);
    ni_pointer_new(e, first, Ptr::NULL);
    let second = stack_alloc(e, 4);
    ni_pointer_new(e, second, Ptr::NULL);
    let skin = e.call(NODE_SKIN_DATA, &args![geometry]).ptr::<()>();
    e.call(NI_OBJECT_DEEP_COPY, &args![skin, first]);
    let copy = ni_pointer_get(e, first);
    ni_pointer_assign(e, second, copy);
    let copy = ni_pointer_get(e, second);
    e.vcall(geometry.addr(), 0xe4, &args![copy]);
    e.call(GET_FACE_COORD, &args![npc, coords]);
    let setting_on = e.call(SETTING_BYTE_VALUE, &[]).bool();
    if !setting_on
        || e.call(APPLY_FACE_COORDS, &args![face, coords, geometry, 0u32])
            .bool()
    {
        let turn = stack_alloc(e, 0x24);
        let product = stack_alloc(e, 0x24);
        e.call(MATRIX_INIT, &args![turn]);
        let angle = e.global::<f32>(QUARTER_TURN_BACK);
        e.call(MAKE_Y_ROTATION, &args![turn, angle]);
        let rotation = e.call(NODE_ROTATION, &args![geometry]).u32();
        let result = e
            .call(MATRIX_MULTIPLY, &args![rotation, product, turn])
            .u32();
        e.call(SET_ROTATION, &args![geometry, result]);
        stack_free(e, product);
        stack_free(e, turn);
    }
    ni_pointer_release(e, second);
    stack_free(e, second);
    ni_pointer_release(e, first);
    stack_free(e, first);
    e.call(
        VECTOR_DESTRUCT,
        &args![coords, 0x20u32, 4u32, FACE_COORD_FREE],
    );
    stack_free(e, coords);
}

/// The last part of loading slot `slot`: names the node `result`
/// (`"<slot name> <form name> (<form id>)"`), adds the add-on nodes of the
/// model `file`, attaches an unskinned node to its bone (or the root, or
/// logs when the slot has no bone), hides dismembered limbs, flags the
/// weapon and stores `result` as the slot's clone.
fn finish_part(
    e: &mut Engine,
    this: Ptr<BipedAnim>,
    slot: u32,
    node: Ptr,
    file: Ptr,
    result: Ptr,
    bone_index: u32,
) {
    let current = object(this, slot);
    let requester = e.get(this, BipedAnim::m_pRequester);
    let mut result = result;
    let form = e.get(current, BipedObject::pParent);
    name_part_node(e, result, form, slot);
    e.call(FN_004AF240, &args![file, node, slot, requester]);
    if !e.get(current, BipedObject::bSkinned) && e.vcall(result.addr(), 0x0c, &[]).u32() != 0 {
        let world = e.call(NODE_PARENT, &args![result]).u32();
        if bone_index == 0xffff_ffff {
            let actor_name = e.call(ACTOR_LOG_NAME, &args![requester]).u32();
            let part = e.get(current, BipedObject::pPart);
            let path = e.vcall(part.addr(), 0x14, &[]).u32();
            e.call(LOG, &args![SHOULD_BE_SKINNED_FORMAT, path, actor_name]);
            if world != 0 {
                e.vcall(world, 0xe8, &args![result]);
            } else {
                let handle = stack_alloc(e, 4);
                ni_pointer_new(e, handle, result);
                ni_pointer_release(e, handle);
                stack_free(e, handle);
            }
            result = Ptr::NULL;
        } else if !fn_004ab230(e, this, bone_index).is_null() {
            let target = fn_004ab230(e, this, bone_index);
            e.vcall(target.addr(), 0xdc, &args![result, 1u32]);
        } else if e.call(NODE_PARENT, &args![result]).u32() != 0 {
            let root = e.get(this, BipedAnim::root);
            e.vcall(root.addr(), 0xdc, &args![result, 1u32]);
        }
    }
    let list = e.call(ACTOR_EXTRA_LIST, &args![requester]).u32();
    let extra = e.call(GET_DISMEMBERMENT_EXTRA, &args![list]).u32();
    if extra != 0 {
        for limb in 0..15u32 {
            if e.call(LIMB_DISMEMBERED, &args![extra, limb]).bool() {
                e.call(HIDE_LIMB, &args![limb, result]);
            }
        }
    }
    let weapon_form = e.get(object(this, WEAPON_SLOT), BipedObject::pParent);
    if slot == 7 && !weapon_form.is_null() && fn_004ab730(e, weapon_form) && !result.is_null() {
        e.call(SET_WEAPON_FLAG, &args![result, 1u32]);
    }
    e.set(current, BipedObject::pPartClone, result);
}

// Translated from 004ad010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 of the byte at `+0x100` of `form` is set.
pub fn fn_004ad010(e: &mut Engine, form: Ptr) -> bool {
    e.mem.u8(form.addr() + 0x100) & 4 != 0
}

// Translated from 004ad030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 13 of the word at `+0x12c` of `form` is set.
pub fn fn_004ad030(e: &mut Engine, form: Ptr) -> bool {
    e.mem.u32(form.addr() + 0x12c) & 0x2000 != 0
}

// Translated from 004ad050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCloningProcess::NiCloningProcess(float)`: builds the process with hash
/// size `0x101`, initializes the vector at `+0x10` and sets its three floats
/// to `scale`. Returns `this`.
pub fn fn_004ad050(e: &mut Engine, this: Ptr, scale: f32) -> Ptr {
    fn_004ad0c0(e, this.cast(), 0x101);
    e.call(MATRIX_INIT, &args![this.byte_add(0x10)]);
    fn_004ad240(e, this.cast(), scale);
    this
}

// Translated from 004ad0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCloningProcess::NiCloningProcess(hash size)`: allocates the clone map
/// and the process map (16 bytes each, built by `004afb20` and `004afb50`
/// with `hash_size`) and stores the copy type (the word `004ad1b0` gives)
/// and the append character (the byte `004ad1c0` gives). Returns `this`.
pub fn fn_004ad0c0(e: &mut Engine, this: Ptr<NiCloningProcess>, hash_size: u32) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let clone_map = if block != 0 {
        e.call(FN_004AFB20, &args![block, hash_size]).ptr()
    } else {
        Ptr::NULL
    };
    e.set(this, NiCloningProcess::m_pkCloneMap, clone_map);
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let process_map = if block != 0 {
        e.call(FN_004AFB50, &args![block, hash_size]).ptr()
    } else {
        Ptr::NULL
    };
    e.set(this, NiCloningProcess::m_pkProcessMap, process_map);
    let copy_type = fn_004ad1b0(e);
    e.set(this, NiCloningProcess::m_eCopyType, copy_type);
    let append = fn_004ad1c0(e);
    e.set(this, NiCloningProcess::m_cAppendChar, append);
    this.cast()
}

// Translated from 004ad1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011f4300`.
pub fn fn_004ad1b0(e: &mut Engine) -> u32 {
    e.global::<u32>(CLONE_COPY_TYPE)
}

// Translated from 004ad1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `011a94a8`.
pub fn fn_004ad1c0(e: &mut Engine) -> u8 {
    e.global::<u8>(CLONE_APPEND_CHAR)
}

// Translated from 004ad1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the two maps of an `NiCloningProcess` (virtual slot 0 with the
/// delete flag 1, when not null).
pub fn fn_004ad1d0(e: &mut Engine, this: Ptr<NiCloningProcess>) {
    let clone_map = e.get(this, NiCloningProcess::m_pkCloneMap);
    if !clone_map.is_null() {
        e.vcall(clone_map.addr(), 0, &args![1u32]);
    }
    let process_map = e.get(this, NiCloningProcess::m_pkProcessMap);
    if !process_map.is_null() {
        e.vcall(process_map.addr(), 0, &args![1u32]);
    }
}

// Translated from 004ad240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the three scale floats of an `NiCloningProcess` to `scale`.
pub fn fn_004ad240(e: &mut Engine, this: Ptr<NiCloningProcess>, scale: f32) {
    e.set(this, NiCloningProcess::m_fScale_z, x87_quiet(scale));
    e.set(this, NiCloningProcess::m_fScale_y, x87_quiet(scale));
    e.set(this, NiCloningProcess::m_fScale_x, x87_quiet(scale));
}

// Translated from 004ad270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCloningProcess::~NiCloningProcess` (Xbox PDB): destroys the two maps
/// ([`fn_004ad1d0`]).
pub fn ni_cloning_process_destructor(e: &mut Engine, this: Ptr<NiCloningProcess>) {
    fn_004ad1d0(e, this);
}

// Translated from 004ad290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::ApplySkinnedObjects` (Xbox PDB): for the loaded node `node`
/// of slot `slot`, records whether it holds skinned nodes (`FindSkinnedNode`)
/// in the slot's skinned flag and, when it does and the biped has a root,
/// attaches them under the root's `SkinAttachment` node (or the root's
/// parent) with [`biped_anim_attach_skinned_object`]: once for `node`, or for
/// each child when `00653270` says the object is of the skinned class.
/// Returns the attached node; 0 when the node is not skinned, `node` itself
/// when the biped has no root (or `node` is null).
pub fn biped_anim_apply_skinned_objects(
    e: &mut Engine,
    this: Ptr<BipedAnim>,
    node: Ptr,
    slot: u32,
    first_flag: u8,
    second_flag: u8,
    face_model: Ptr,
) -> Ptr {
    let skinned_class = e
        .call(CAST_TO_CLASS, &args![SKINNED_CLASS, node])
        .ptr::<()>();
    let handle = stack_alloc(e, 4);
    ni_pointer_new(e, handle, Ptr::NULL);
    let result;
    if node.is_null() {
        result = node;
    } else {
        let skinned = e.call(FIND_SKINNED_NODE, &args![node]).u8();
        e.mem.set_u8(object(this, slot).addr() + 0xc, skinned);
        if e.get(this, BipedAnim::root).is_null() {
            result = node;
        } else if e.get(object(this, slot), BipedObject::bSkinned) {
            let attachment = stack_alloc(e, 4);
            ni_pointer_new(e, attachment, Ptr::NULL);
            let root = e.get(this, BipedAnim::root);
            let found = fn_004aae30(e, root, Ptr::new(SKIN_ATTACHMENT_NAME));
            if !found.is_null() {
                ni_pointer_assign(e, attachment, found);
            } else {
                let parent = e.call(NODE_PARENT, &args![root]).ptr();
                ni_pointer_assign(e, attachment, parent);
            }
            let mut attached = Ptr::NULL;
            if skinned_class.is_null() {
                let parent_object = ni_pointer_get(e, attachment);
                attached = biped_anim_attach_skinned_object(
                    e,
                    this,
                    node,
                    parent_object,
                    slot,
                    first_flag,
                    second_flag,
                    face_model,
                );
            } else {
                let mut index = 0;
                while index < child_count(e, skinned_class) {
                    let child = child_at(e, skinned_class, index);
                    attached = biped_anim_attach_skinned_object(
                        e,
                        this,
                        child,
                        Ptr::NULL,
                        slot,
                        first_flag,
                        second_flag,
                        face_model,
                    );
                    index += 1;
                }
            }
            result = attached;
            if attached != node {
                // A reference is taken on `node` and given back at once.
                let extra = stack_alloc(e, 4);
                ni_pointer_new(e, extra, node);
                ni_pointer_release(e, extra);
                stack_free(e, extra);
            }
            ni_pointer_release(e, attachment);
            stack_free(e, attachment);
        } else {
            result = Ptr::NULL;
        }
    }
    ni_pointer_release(e, handle);
    stack_free(e, handle);
    result
}

// Translated from 004ad4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::AttachSkinnedObject` (Xbox PDB): attaches the skinned parts
/// of `node`. A node `parent_object` is given: when `node`'s second child
/// has a geometry (virtual slot 0x18), a new `NiNode` is made, added to
/// `parent_object` (virtual slot 0xdc) and becomes the node the parts go
/// to. The weapon bone keeps the node's extra data (`011c61e4`) and the
/// list of its geometries. Children without a geometry (and, per the flags,
/// those named like the pip-boy or the third slot) are collected and
/// removed from `node`; each child with data has its FaceGen data prepared,
/// its skin copied, its body part lights bound to the skeleton's bones by
/// name and is added to the node. Returns the node made, else the first
/// child with data, else 0.
#[allow(clippy::too_many_arguments)]
pub fn biped_anim_attach_skinned_object(
    e: &mut Engine,
    this: Ptr<BipedAnim>,
    node: Ptr,
    parent_object: Ptr,
    slot: u32,
    flag_a: u8,
    flag_b: u8,
    face_model: Ptr,
) -> Ptr {
    let mut result = Ptr::NULL;
    let mut attach_to = parent_object;
    let handle = stack_alloc(e, 4);
    ni_pointer_new(e, handle, Ptr::NULL);
    e.call(FN_004ADD70, &args![node]);
    let mut face = face_model;
    if face.is_null() && is_face_gen_slot(e, this, slot) {
        face = e.call(FN_004AE790, &args![this, slot]).ptr();
    }
    e.call(SET_TRANSLATION, &args![node, ZERO_TRANSLATION]);
    if !parent_object.is_null() && child_count(e, node) >= 2 {
        let second = child_at(e, node, 1);
        if !second.is_null() {
            let second = child_at(e, node, 1);
            // Virtual slot 0x18: the geometry of a node.
            if e.vcall(second.addr(), 0x18, &[]).u32() != 0 {
                let block = e.call(NI_OPERATOR_NEW, &args![0xacu32]).u32();
                attach_to = if block != 0 {
                    e.call(NI_NODE_INIT, &args![block, 0u32]).ptr()
                } else {
                    Ptr::NULL
                };
                e.vcall(parent_object.addr(), 0xdc, &args![attach_to, 1u32]);
                result = attach_to;
            }
        }
    }
    let key = fn_004ab220(e);
    let extra = e.call(GET_EXTRA_DATA, &args![node, key]).ptr::<()>();
    if !extra.is_null() {
        let weapon_bone = fn_004ab230(e, this, 1);
        if !weapon_bone.is_null() {
            let key = fn_004ab220(e);
            e.call(REMOVE_EXTRA_DATA, &args![weapon_bone, key]);
        }
        let key = fn_004ab220(e);
        e.call(ADD_EXTRA_DATA, &args![weapon_bone, key, extra]);
        let mut index = 0;
        while index < child_count(e, node) {
            let child = child_at(e, node, index);
            if e.vcall(child.addr(), 0x18, &[]).u32() != 0 {
                let geometry = e.vcall(child.addr(), 0x18, &[]).u32();
                if e.call(NODE_DATA_POINTER, &args![geometry]).u32() != 0 {
                    let item = stack_alloc(e, 4);
                    let child = child_at(e, node, index);
                    e.mem.set_u32(item.addr(), child.addr());
                    e.call(ADD_TO_LIST, &args![extra.byte_add(0x34), item]);
                    stack_free(e, item);
                }
            }
            index += 1;
        }
    }
    let collected = stack_alloc(e, 0x10);
    e.call(FN_004AFF00, &args![collected, 0u32, 1u32]);
    let mut index = 0;
    while index < child_count(e, node) {
        let child = child_at(e, node, index);
        if !child.is_null() && e.vcall(child.addr(), 0x18, &[]).u32() == 0 {
            array_add(e, collected, child);
            e.vcall(node.addr(), 0xe8, &args![child]);
            index += 1;
            continue;
        }
        if flag_a != 0 {
            let name = node_name(e, child);
            if !name.is_null() {
                let prefix = e.global::<u32>(SLOT_NAMES + 8);
                let length = e.call(TEXT_LENGTH, &args![prefix]).u32();
                let name = node_name(e, child);
                if e.call(COMPARE_PREFIX, &args![name, prefix, length]).i32() == 0 {
                    array_add(e, collected, child);
                    e.vcall(node.addr(), 0xe8, &args![child]);
                    index += 1;
                    continue;
                }
            }
        }
        let name = node_name(e, child);
        if !name.is_null() {
            let name = node_name(e, child);
            let text = stack_alloc(e, 8);
            e.call(STRING_FROM_TEXT, &args![text, name]);
            e.call(FN_004AFAD0, &args![text]);
            let prefix = stack_alloc(e, 12);
            let chars = ni_pointer_get(e, text);
            let (count, wanted) = if flag_b != 0 {
                (9u32, PIPBOY_OFF_PREFIX)
            } else {
                (8u32, PIPBOY_ON_PREFIX)
            };
            e.call(FN_004ADD50, &args![prefix, chars, count]);
            e.mem.set_u8(prefix.addr() + count, 0);
            let matched = e.call(COMPARE_TEXT, &args![prefix, wanted]).i32() == 0;
            stack_free(e, prefix);
            if matched {
                array_add(e, collected, child);
                e.vcall(node.addr(), 0xe8, &args![child]);
                e.call(STRING_FREE, &args![text]);
                stack_free(e, text);
                index += 1;
                continue;
            }
            e.call(STRING_FREE, &args![text]);
            stack_free(e, text);
        }
        if !child.is_null() {
            let requester = e.get(this, BipedAnim::m_pRequester);
            if e.call(NODE_SKIN_DATA, &args![child]).u32() != 0
                && e.call(NODE_DATA_POINTER, &args![child]).u32() != 0
                && !face.is_null()
                && !requester.is_null()
                && e.call(REFERENCE_FORM, &args![requester]).u32() != 0
            {
                let base = e.call(REFERENCE_FORM, &args![requester]).u32();
                if e.call(FORM_TYPE, &args![base]).u32() == FORM_TYPE_NPC {
                    attach_face_gen(e, requester, face, child, handle);
                }
            }
        }
        e.call(FN_004AF490, &args![this, child, 0u32]);
        let data = e.call(NODE_DATA_POINTER, &args![child]).ptr::<()>();
        if !data.is_null() {
            if result.is_null() {
                result = child;
            }
            let owner = e.call(BODY_PART_OWNER, &args![data]).u32();
            let count = e.call(BODY_PART_COUNT, &args![owner]).u32();
            let items = e.call(BODY_PART_ARRAY, &args![data]).u32();
            let mut light = 0;
            while light < count {
                let item = e.mem.u32(items + 4 * light);
                if node_name(e, Ptr::new(item)).is_null() {
                    let parent_name = if e.call(NODE_PARENT, &args![node]).u32() != 0 {
                        let parent = e.call(NODE_PARENT, &args![node]).ptr();
                        node_name(e, parent)
                    } else {
                        Ptr::new(NULL_TEXT)
                    };
                    let own_name = node_name(e, node);
                    e.call(
                        LOG,
                        &args![UNNAMED_BONE_FORMAT, light, parent_name, own_name],
                    );
                    light += 1;
                    continue;
                }
                let root = e.get(this, BipedAnim::root);
                let field = e.call(NAME_FIELD, &args![item]).u32();
                let bone_node = e.call(FN_004ADE00, &args![root, field]).u32();
                if bone_node != 0 {
                    e.call(FN_004ADDA0, &args![data, light, bone_node]);
                }
                light += 1;
            }
            if !attach_to.is_null() {
                e.call(SET_PARENT_OBJECT, &args![data, parent_object]);
                e.vcall(attach_to.addr(), 0xdc, &args![child, 1u32]);
            }
        }
        index += 1;
    }
    e.call(FN_004ADD70, &args![node]);
    e.call(FN_004ADE20, &args![collected]);
    stack_free(e, collected);
    ni_pointer_release(e, handle);
    stack_free(e, handle);
    result
}

/// The FaceGen block of `AttachSkinnedObject` (see [`apply_face_gen`] for the
/// part it shares): the geometry's skin copy is set, then its property data
/// is cloned and attached, the NPC's face coordinates are fetched and, when
/// the setting `0043faf0` is on, applied to `face`. `handle` is the
/// function's `NiPointer` the deep copies are written to.
fn attach_face_gen(e: &mut Engine, requester: Ptr, face: Ptr, child: Ptr, handle: Ptr) {
    let npc = e.call(REFERENCE_FORM, &args![requester]).ptr::<()>();
    let coords = stack_alloc(e, 0x80);
    e.call(
        VECTOR_CONSTRUCT,
        &args![coords, 0x20u32, 4u32, FACE_COORD_INIT, FACE_COORD_FREE],
    );
    let copy_handle = stack_alloc(e, 4);
    ni_pointer_new(e, copy_handle, Ptr::NULL);
    let skin = e.call(NODE_SKIN_DATA, &args![child]).ptr::<()>();
    e.call(NI_OBJECT_DEEP_COPY, &args![skin, handle]);
    let copy = ni_pointer_get(e, handle);
    ni_pointer_assign(e, copy_handle, copy);
    let copy = ni_pointer_get(e, copy_handle);
    e.vcall(child.addr(), 0xe4, &args![copy]);
    if e.call(NODE_DATA_POINTER, &args![child]).u32() != 0 {
        let data = e.call(NODE_DATA_POINTER, &args![child]).u32();
        if e.call(NODE_DATA_FIELD, &args![data]).u32() != 0 {
            let data = e.call(NODE_DATA_POINTER, &args![child]).u32();
            let field = e.call(NODE_DATA_FIELD, &args![data]).u32();
            e.call(NI_OBJECT_DEEP_COPY, &args![field, handle]);
            let copied = ni_pointer_get(e, handle);
            let property_handle = stack_alloc(e, 4);
            ni_pointer_new(e, property_handle, copied);
            let data = e.call(NODE_DATA_POINTER, &args![child]).u32();
            let cloned = e.call(NI_OBJECT_CLONE_OV2, &args![data]).u32();
            e.call(FN_004ADDE0, &args![child, cloned]);
            let kept = ni_pointer_get(e, property_handle);
            let data = e.call(NODE_DATA_POINTER, &args![child]).u32();
            e.call(FN_004ADDC0, &args![data, kept]);
            ni_pointer_release(e, property_handle);
            stack_free(e, property_handle);
        }
    }
    e.call(GET_FACE_COORD, &args![npc, coords]);
    if e.call(SETTING_BYTE_VALUE, &[]).bool() {
        e.call(APPLY_FACE_COORDS, &args![face, coords, child, 0u32]);
    }
    ni_pointer_release(e, copy_handle);
    stack_free(e, copy_handle);
    e.call(
        VECTOR_DESTRUCT,
        &args![coords, 0x20u32, 4u32, FACE_COORD_FREE],
    );
    stack_free(e, coords);
}

// ---- Session 2: the functions from 004add50 -----------------------------------------

// Translated from 004add50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strncpy_s(destination, count + 1, source, count)`: copies `count`
/// characters of `source` and terminates `destination`. Returns the CRT
/// function's result.
pub fn fn_004add50(e: &mut Engine, destination: Ptr, source: Ptr, count: u32) -> u32 {
    e.call(
        STRNCPY_S,
        &args![destination, count.wrapping_add(1), source, count],
    )
    .u32()
}

// Translated from 004add70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compacts the `NiTArray<NiPointer<NiAVObject>>` at `+0x9c` of `node`
/// (`004afc80`) and then updates its size (`004afe50`); the map names the two
/// `Compact` and `UpdateSize`.
pub fn fn_004add70(e: &mut Engine, node: Ptr) {
    let array = node.byte_add(0x9c);
    e.call(FN_004AFC80, &args![array]);
    e.call(FN_004AFE50, &args![array]);
}

// Translated from 004adda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` as element `index` of the array of words at `+0x14` of
/// `this` (the bone table of a geometry's data).
pub fn fn_004adda0(e: &mut Engine, this: Ptr, index: u32, value: Ptr) {
    let table = e.mem.u32(this.addr().wrapping_add(0x14));
    e.mem
        .set_u32(table.wrapping_add(index.wrapping_mul(4)), value.addr());
}

// Translated from 004addc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `value` to the `NiPointer` at `+0xc` of `this`.
pub fn fn_004addc0(e: &mut Engine, this: Ptr, value: Ptr) {
    ni_pointer_assign(e, this.byte_add(0x0c), value);
}

// Translated from 004adde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `value` to the `NiPointer` at `+0xbc` of `this` (the data
/// pointer of a geometry node).
pub fn fn_004adde0(e: &mut Engine, this: Ptr, value: Ptr) {
    ni_pointer_assign(e, this.byte_add(0xbc), value);
}

// Translated from 004ade00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks up the object named by the `NiFixedString` at `name` under `root`
/// (`BSUtilities::GetObjectByName(root, name, 1)`, `00c4b470`).
pub fn fn_004ade00(e: &mut Engine, root: Ptr, name: Ptr) -> Ptr {
    e.call(GET_OBJECT_BY_NAME, &args![root, name, 1u32]).ptr()
}

// Translated from 004ade20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the node array `this` (the map names the body
/// `~basic_streambuf<>`, a folded name; it is [`fn_004afc20`]).
pub fn fn_004ade20(e: &mut Engine, this: Ptr) {
    fn_004afc20(e, this);
}

// Translated from 004ade40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::AttachToSkeleton` (Xbox PDB): binds the bone table of the
/// geometry `node` to the objects named in it under `skeleton` (a bone not
/// found is replaced by `skeleton` itself and, with `flag`, logged), attaches
/// the geometry to `parent` (virtual slot 0xdc) when given and, for a `node`
/// with children, attaches every child the same way (a child that is not a
/// geometry is moved out of `node`, after an incorrect export is logged once).
/// The children of the skinned class are attached with `flag` 1.
pub fn biped_anim_attach_to_skeleton(
    e: &mut Engine,
    skeleton: Ptr,
    node: Ptr,
    parent: Ptr,
    flag: u8,
) {
    if node.is_null() {
        return;
    }
    // Virtual slot 0xc of the node: its `NiNode`, if it is one.
    let as_node = e.vcall(node.addr(), 0x0c, &[]).ptr::<()>();
    // Virtual slot 0x18: its geometry, if it is one.
    if e.vcall(node.addr(), 0x18, &[]).u32() != 0 {
        let data = e.call(NODE_DATA_POINTER, &args![node]).ptr::<()>();
        if !data.is_null() {
            let owner = e.call(BODY_PART_OWNER, &args![data]).u32();
            let count = e.call(BODY_PART_COUNT, &args![owner]).u32();
            let items = e.call(BODY_PART_ARRAY, &args![data]).u32();
            for index in 0..count {
                let item = e.mem.u32(items.wrapping_add(index.wrapping_mul(4)));
                if item == 0 {
                    continue;
                }
                let name = e.call(NAME_FIELD, &args![item]).u32();
                let found = fn_004ade00(e, skeleton, Ptr::new(name));
                if !found.is_null() {
                    fn_004adda0(e, data, index, found);
                    continue;
                }
                fn_004adda0(e, data, index, skeleton);
                if flag == 0 {
                    continue;
                }
                if !as_node.is_null() {
                    let bone_text = node_name(e, Ptr::new(item));
                    let length = e.call(TEXT_LENGTH, &args![bone_text]).u32();
                    let bone_text = node_name(e, Ptr::new(item));
                    let node_text = node_name(e, node);
                    if e.call(COMPARE_PREFIX, &args![node_text, bone_text, length])
                        .u32()
                        == 0
                    {
                        let as_node_name = node_name(e, as_node);
                        let above = e.call(NODE_PARENT, &args![as_node]).ptr::<()>();
                        let above_name = node_name(e, above);
                        let bone_name = node_name(e, Ptr::new(item));
                        e.call(
                            LOG,
                            &args![BONE_IN_PART_FORMAT, bone_name, above_name, as_node_name],
                        );
                        continue;
                    }
                }
                if !as_node.is_null() && !skeleton.is_null() {
                    let skeleton_name = node_name(e, skeleton);
                    let as_node_name = node_name(e, as_node);
                    let bone_name = node_name(e, Ptr::new(item));
                    e.call(
                        LOG,
                        &args![
                            BONE_REQUESTED_FORMAT,
                            bone_name,
                            as_node_name,
                            skeleton_name
                        ],
                    );
                } else {
                    let bone_name = node_name(e, Ptr::new(item));
                    e.call(LOG, &args![BONE_ONLY_FORMAT, bone_name]);
                }
            }
            if !parent.is_null() {
                e.call(SET_PARENT_OBJECT, &args![data, parent]);
                e.vcall(parent.addr(), 0xdc, &args![node, 1u32]);
            }
        }
    }
    if as_node.is_null() {
        return;
    }
    let mut reported = false;
    fn_004add70(e, as_node);
    let moved = stack_alloc(e, 0x10);
    e.call(FN_004AFF00, &args![moved, 0u32, 1u32]);
    let mut index = 0;
    // The child count is read again on every round: children are removed.
    while index < child_count(e, as_node) {
        let child = child_at(e, as_node, index);
        index += 1;
        if child.is_null() {
            continue;
        }
        let child_node = e.vcall(child.addr(), 0x0c, &[]).ptr::<()>();
        if e.call(IS_KIND_OF, &args![SKINNED_CLASS, as_node]).bool() && !child_node.is_null() {
            biped_anim_attach_to_skeleton(e, skeleton, child, parent, 1);
        } else if e.vcall(child.addr(), 0x18, &[]).u32() == 0 {
            if !reported && !child_node.is_null() && e.call(NODE_CHECK_9C, &args![child]).u32() != 0
            {
                let skeleton_name = node_name(e, skeleton);
                let as_node_name = node_name(e, as_node);
                let above = e.call(NODE_PARENT, &args![as_node]).ptr::<()>();
                let above_name = node_name(e, above);
                e.call(
                    LOG,
                    &args![
                        EXPORTED_WRONG_FORMAT,
                        above_name,
                        as_node_name,
                        skeleton_name
                    ],
                );
                reported = true;
            }
            array_add(e, moved, child);
            e.vcall(as_node.addr(), 0xe8, &args![child]);
        } else {
            biped_anim_attach_to_skeleton(e, skeleton, child, parent, flag);
        }
    }
    fn_004add70(e, as_node);
    fn_004ade20(e, moved);
    stack_free(e, moved);
}

// Translated from 004ae250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::AttachToParent` (Xbox PDB): attaches the part `node` to the
/// node its parent-node extra data (`Prn`, a string extra data, from
/// `source` or else from `node`) names under `root`; logs when the extra data
/// or the parent is missing (for form type `0x2b`, `RemoveScabard` instead).
/// With a parent: takes its scale, attaches `node`, records it with the
/// shadow scene and attaches the `Scb` child. Then handles the `Backpack`
/// child (the player's first person node gets the weapon flag, the others
/// the scale and an `UPB` parent), and finally either kills the Havok of the
/// first person biped or checks a weapon's collision (slot 5: data, shape
/// and layer, each logged when wrong) and sets the motion of `node`.
/// `_unused_5` is a word the callers pass and the function never reads.
pub fn biped_anim_attach_to_parent(
    e: &mut Engine,
    root: Ptr,
    node: Ptr,
    source: Ptr,
    biped: Ptr<BipedAnim>,
    slot: u32,
    _unused_5: u32,
) {
    let mut extra: Ptr = Ptr::NULL;
    let mut parent = Ptr::NULL;
    if !source.is_null() {
        let key = fn_004ae780(e);
        extra = e.call(GET_EXTRA_DATA, &args![source, key]).ptr();
    }
    if extra.is_null() {
        let key = fn_004ae780(e);
        extra = e.call(GET_EXTRA_DATA, &args![node, key]).ptr();
    }
    if extra.is_null() {
        let name = node_name(e, node);
        e.call(LOG, &args![PARENT_EXTRA_MISSING_FORMAT, name]);
    } else {
        let text_extra = e
            .call(CAST_TO_CLASS, &args![STRING_EXTRA_CLASS, extra])
            .ptr::<()>();
        if text_extra.is_null() {
            let name = node_name(e, node);
            e.call(LOG, &args![EXTRA_NOT_STRING_FORMAT, name]);
        } else {
            let field = e.call(STRING_EXTRA_TEXT, &args![text_extra]).u32();
            parent = fn_004ade00(e, root, Ptr::new(field));
            if parent.is_null() {
                let reference = e.call(FIND_REFERENCE_FOR_3D, &args![root]).ptr::<()>();
                let mut scabbard = false;
                if !reference.is_null()
                    && !e
                        .call(REFERENCE_FORM, &args![reference])
                        .ptr::<()>()
                        .is_null()
                {
                    let form = e.call(REFERENCE_FORM, &args![reference]).u32();
                    if e.call(FORM_TYPE, &args![form]).u32() == FORM_TYPE_REMOVES_SCABBARD {
                        e.call(REMOVE_SCABBARD, &args![node]);
                        scabbard = true;
                    }
                }
                if !scabbard {
                    let node_text = node_name(e, node);
                    let field = e.call(STRING_EXTRA_TEXT, &args![text_extra]).u32();
                    let parent_text = e.call(NAME_TEXT, &args![field]).ptr::<()>();
                    e.call(LOG, &args![PARENT_NOT_FOUND_FORMAT, parent_text, node_text]);
                }
            }
        }
    }
    let mut scale = 1.0f32;
    if !parent.is_null() {
        scale = e.call(NODE_SCALE, &args![parent]).f32();
        e.vcall(parent.addr(), 0xdc, &args![node, 1u32]);
        let count = e.call(BODY_PART_ARRAY, &args![COUNT_OBJECT]).u32();
        let record = stack_alloc(e, 0x10);
        e.call(BUILD_RECORD, &args![record, count as f32, 0u8, 0u8]);
        e.vcall(node.addr(), 0xa4, &args![record, 0u32]);
        stack_free(e, record);
        let scene = e.call(SHADOW_SCENE_NODE, &args![0u32]).u32();
        e.call(SHADOW_ADD_OBJECT, &args![scene, node]);
        let scabbard_node = fn_004aae30(e, node, Ptr::new(SCABBARD_NODE_NAME));
        if !scabbard_node.is_null() {
            e.vcall(parent.addr(), 0xdc, &args![scabbard_node, 1u32]);
            e.call(NOTIFY_ATTACHED, &args![parent, 1u32]);
            e.call(UPDATE_PROPERTIES, &args![scabbard_node]);
            fn_004add70(e, node);
        }
    }
    let backpack = fn_004aae30(e, node, Ptr::new(BACKPACK_NAME));
    if !backpack.is_null() {
        let player = e.global::<u32>(PLAYER);
        let player_node = e.call(PLAYER_NODE, &args![player, 1u32]).u32();
        if root.addr() == player_node {
            e.call(SET_WEAPON_FLAG, &args![backpack, 1u32]);
        } else {
            e.call(SET_NODE_SCALE, &args![backpack, scale]);
            let fixed = stack_alloc(e, 4);
            let key = e
                .call(FIXED_STRING_INIT, &args![fixed, BACKPACK_KEY_NAME])
                .u32();
            extra = e.call(GET_EXTRA_DATA, &args![backpack, key]).ptr();
            e.call(FIXED_STRING_FREE, &args![fixed]);
            stack_free(e, fixed);
            if !extra.is_null() {
                let text_extra = e
                    .call(CAST_TO_CLASS, &args![STRING_EXTRA_CLASS, extra])
                    .ptr::<()>();
                if !text_extra.is_null() {
                    let field = e.call(STRING_EXTRA_TEXT, &args![text_extra]).u32();
                    parent = fn_004ade00(e, root, Ptr::new(field));
                    if !parent.is_null() {
                        e.vcall(parent.addr(), 0xdc, &args![backpack, 1u32]);
                    }
                }
            }
        }
    }
    if !biped.is_null() && biped.addr() == player_biped(e, 1) {
        e.call(KILL_HAVOK, &args![node, 1u32, 1u32]);
    } else {
        if slot == WEAPON_SLOT {
            let collision = e.call(GET_COLLISION_OBJECT, &args![node]).u32();
            if collision == 0 {
                let name = node_name(e, node);
                e.call(LOG, &args![NO_HAVOK_FORMAT, name]);
            } else {
                // The code tests `collision` for null again here; it cannot be.
                let body = e.call(COLLISION_BODY, &args![collision]).u32();
                let shape = if body == 0 {
                    0
                } else {
                    e.call(BODY_SHAPE, &args![body]).u32()
                };
                if shape == 0 {
                    let name = node_name(e, node);
                    e.call(LOG, &args![NO_SHAPE_FORMAT, name]);
                }
                if body != 0 {
                    let out = stack_alloc(e, 4);
                    let layer = e.call(COPY_LAYER, &args![body, out]).u32();
                    if e.call(LAYER_OF, &args![layer]).u32() != WEAPON_LAYER {
                        let name = node_name(e, node);
                        e.call(LOG, &args![WEAPON_LAYER_FORMAT, name]);
                    }
                    stack_free(e, out);
                }
            }
        }
        e.call(SET_MOTION, &args![node, 4u32, 1u32, 1u32, 1u32]);
    }
}

// Translated from 004ae780 (decompiled, FalloutNV.exe 1.4.0.525)
/// The key of the parent node extra data: the global at `011c61e8`.
pub fn fn_004ae780(e: &mut Engine) -> Ptr {
    Ptr::new(e.global::<u32>(PARENT_NODE_KEY))
}

// Translated from 004ae790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::LoadFaceGenModel` (Xbox PDB): with the part lock held, for a
/// slot with a model: builds `Meshes\<model path>`, asks the FaceGen manager
/// for the matching `.egm` file and loads it. Returns the loaded FaceGen
/// model, or 0 (no model in the slot).
pub fn biped_anim_load_face_gen_model(e: &mut Engine, this: Ptr<BipedAnim>, slot: u32) -> Ptr {
    e.call(LOCK_ENTER, &args![PART_LOCK, 0u32]);
    let part = e.get(object(this, slot), BipedObject::pPart);
    let mut result = Ptr::NULL;
    if !part.is_null() {
        let text = stack_alloc(e, 8);
        e.call(STRING_INIT, &args![text]);
        let buffer = stack_alloc(e, 0x104);
        e.call(STRING_COPY, &args![buffer, 0x104u32, MESHES_PREFIX]);
        // Virtual slot 0x14 of the model: its path.
        let path = e.vcall(part.addr(), 0x14, &[]).u32();
        e.call(STRING_APPEND, &args![buffer, 0x104u32, path]);
        let file = e
            .call(GET_EGM_FILE, &args![text, buffer, 0xffff_ffffu32])
            .u32();
        result = e
            .call(
                FACE_GEN_LOAD,
                &args![file, 0u32, 0u32, 0u32, 1u32, 0xffff_ffffu32, 0u32],
            )
            .ptr();
        e.call(STRING_FREE, &args![text]);
        stack_free(e, buffer);
        stack_free(e, text);
    }
    e.call(LOCK_LEAVE, &args![PART_LOCK]);
    result
}

// Translated from 004ae8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether slot `slot` has a model and that model has bit 0 of the byte at
/// `+0x14` set (`004ae8f0`): the slot holds FaceGen data.
pub fn fn_004ae8a0(e: &mut Engine, this: Ptr<BipedAnim>, slot: u32) -> bool {
    let part = e.get(object(this, slot), BipedObject::pPart);
    !part.is_null() && fn_004ae8f0(e, part) != 0
}

// Translated from 004ae8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0 of the byte at `+0x14` of `this`.
pub fn fn_004ae8f0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x14)) & 1
}

// Translated from 004ae910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::CloneHelmet` (Xbox PDB): for a slot that has no clone yet and
/// given `face_model` (only tested for null; `AttachHelmet` passes the same
/// word on as the FaceGen model) and the loaded `file`, clones the file
/// (deep copy with the same textures when it has a morpher controller),
/// sets the fade node's LOD multiplier type to 7, resets the translation and
/// rotation, swaps the textures the slot's model and form ask for and
/// returns the clone. Returns 0 when the slot has a clone or an argument is
/// null, or the clone fails.
pub fn biped_anim_clone_helmet(
    e: &mut Engine,
    this: Ptr<BipedAnim>,
    face_model: Ptr,
    file: Ptr,
    slot: u32,
) -> Ptr {
    let current = object(this, slot);
    if !e.get(current, BipedObject::pPartClone).is_null() {
        return Ptr::NULL;
    }
    if face_model.is_null() || file.is_null() {
        return Ptr::NULL;
    }
    let cloning = stack_alloc(e, 0x1c);
    fn_004ad050(e, cloning, 1.0);
    let handle = stack_alloc(e, 4);
    ni_pointer_new(e, handle, Ptr::NULL);
    let copy = if e.call(HAS_MORPHER_CONTROLLER, &args![file]).bool() {
        let tes = e.global::<u32>(TES_GLOBAL);
        let deep = e
            .call(DEEP_COPY_SAME_TEXTURES, &args![tes, file, cloning])
            .ptr();
        ni_pointer_assign(e, handle, deep);
        ni_pointer_get(e, handle)
    } else {
        e.call(NI_OBJECT_CLONE, &args![file, cloning]).ptr()
    };
    let mut result = Ptr::NULL;
    if !copy.is_null() {
        if e.call(IS_KIND_OF, &args![SKINNED_CLASS, copy]).bool() {
            e.call(SET_LOD_MULT_TYPE, &args![copy, 7u32]);
        }
        e.call(SET_TRANSLATION, &args![copy, ZERO_TRANSLATION]);
        e.call(SET_ROTATION, &args![copy, IDENTITY_ROTATION]);
        let part = e.get(current, BipedObject::pPart);
        if !part.is_null() {
            // Virtual slot 0x1c of the model: the texture swap.
            let swap = e.vcall(part.addr(), 0x1c, &[]).u32();
            if swap != 0 {
                e.call(SWAP_TEXTURES, &args![swap, copy]);
            }
        }
        let form = e.get(current, BipedObject::pParent);
        if !form.is_null() && e.vcall(form.addr(), 0xac, &[]).bool() {
            e.call(SWAP_PLATFORM_TEXTURES, &args![copy]);
        }
        result = copy;
    }
    ni_pointer_release(e, handle);
    stack_free(e, handle);
    ni_cloning_process_destructor(e, cloning.cast());
    stack_free(e, cloning);
    result
}

// Translated from 004aeb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::AttachHelmet` (Xbox PDB): for `node` (given `face_model`),
/// applies the skinned objects like `LoadBipedParts` does (the first person
/// player flags as there) or else attaches it with
/// [`biped_anim_attach_to_parent`]; the result is named after the slot and
/// the form, gets its add-on nodes and, when the slot is not skinned, is
/// attached to the bone of the slot (the root when there is no such bone and
/// the result has a parent). Stored as the slot's clone. At the end the
/// actor's 3D properties are updated.
pub fn biped_anim_attach_helmet(
    e: &mut Engine,
    this: Ptr<BipedAnim>,
    face_model: Ptr,
    node: Ptr,
    slot: u32,
) {
    let requester = e.get(this, BipedAnim::m_pRequester);
    if !face_model.is_null() && !node.is_null() {
        let bone_index = e.global::<u32>(SLOT_BONES + 4 * slot);
        // The result of this first test is not used.
        if requester.addr() == e.global::<u32>(PLAYER) {
            let player = e.global::<u32>(PLAYER);
            e.call(IS_FIRST_PERSON_BIPED, &args![player, this]);
        }
        let has_slot_6 = !e.get(object(this, 6), BipedObject::pParent).is_null();
        let mut first_person = false;
        if requester.addr() == e.global::<u32>(PLAYER) {
            let player = e.global::<u32>(PLAYER);
            if e.call(IS_FIRST_PERSON_BIPED, &args![player, this]).bool() {
                first_person = true;
            }
        }
        let mut result = biped_anim_apply_skinned_objects(
            e,
            this,
            node,
            slot,
            first_person as u8,
            has_slot_6 as u8,
            face_model,
        );
        if result.is_null() {
            result = node;
            let root = e.get(this, BipedAnim::root);
            biped_anim_attach_to_parent(e, root, node, Ptr::NULL, this, slot, face_model.addr());
        }
        if !result.is_null() {
            let form = e.get(object(this, slot), BipedObject::pParent);
            name_part_node(e, result, form, slot);
            fn_004af240(e, node, node, slot, requester.addr());
            let skinned = e.get(object(this, slot), BipedObject::bSkinned);
            if !skinned && e.vcall(result.addr(), 0x0c, &[]).u32() != 0 {
                e.call(NODE_PARENT, &args![result]);
                // `bone[bone_index].pParent`, whose address wraps for index -1.
                let bone_node: Ptr = Ptr::new(
                    e.mem.u32(
                        this.addr()
                            .wrapping_add(bone_index.wrapping_mul(8))
                            .wrapping_add(8),
                    ),
                );
                if !bone_node.is_null() {
                    e.vcall(bone_node.addr(), 0xdc, &args![result, 1u32]);
                } else if !e.call(NODE_PARENT, &args![result]).ptr::<()>().is_null() {
                    let root = e.get(this, BipedAnim::root);
                    e.vcall(root.addr(), 0xdc, &args![result, 1u32]);
                }
            }
            e.set(object(this, slot), BipedObject::pPartClone, result);
        }
    }
    if !requester.is_null() {
        let actor_root = e.call(ACTOR_ROOT, &args![requester]).ptr::<()>();
        if !actor_root.is_null() {
            let actor_root = e.call(ACTOR_ROOT, &args![requester]).ptr::<()>();
            e.call(UPDATE_PROPERTIES, &args![actor_root]);
        }
    }
}

// Translated from 004aede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues the slot's model through [`fn_004aee60`] with a fresh handle (the
/// four-byte object `00528cb0` builds and `0044cbf0` destroys).
pub fn fn_004aede0(e: &mut Engine, this: Ptr<BipedAnim>, first: u32, second: u32, slot: u32) {
    let handle = stack_alloc(e, 4);
    e.call(QUEUE_OBJECT_INIT, &args![handle, 0u32]);
    fn_004aee60(e, this, first, second, handle, slot);
    e.call(QUEUE_OBJECT_FREE, &args![handle]);
    stack_free(e, handle);
}

// Translated from 004aee60 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the `NiPointer` `handle` holds nothing: for a slot without a model
/// (or with the model word -1) assigns null to it (`006f74f0`), otherwise
/// asks the model loader to queue the slot's part (`004450c0`).
pub fn fn_004aee60(
    e: &mut Engine,
    this: Ptr<BipedAnim>,
    first: u32,
    second: u32,
    handle: Ptr,
    slot: u32,
) {
    if !ni_pointer_get(e, handle).is_null() {
        return;
    }
    let part = e.get(object(this, slot), BipedObject::pPart);
    if part.is_null() || part.addr() == 0xffff_ffff {
        e.call(QUEUED_FILE_ASSIGN, &args![handle, 0u32]);
    } else {
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(
            LOADER_QUEUE_PART,
            &args![loader, this, handle, first, second, slot],
        );
    }
}

// Translated from 004aeed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::LoadAndAttachAddOn` (Xbox PDB): loads the model `model` (not
/// null, with a path) for `requester` and clones it (like `LoadBipedParts`),
/// swaps its textures (the model's and, when `form` asks, the platform
/// ones), adds the add-on nodes, puts back the extra data of the clone under
/// the key `004ab220` gives after removing all of it, resets its
/// translation and rotation and, unless it is skinned (then logged),
/// attaches it with [`biped_anim_attach_to_parent`] and, when it has no
/// parent yet, under the bone of `slot` or the root (`root_override`, else
/// the actor's 3D root). Returns the clone, or 0.
pub fn biped_anim_load_and_attach_add_on(
    e: &mut Engine,
    form: Ptr,
    model: Ptr,
    slot: u32,
    requester: Ptr,
    root_override: Ptr,
) -> Ptr {
    if model.is_null() || requester.is_null() {
        return Ptr::NULL;
    }
    // Virtual slot 0x14 of the model: its path.
    let path = e.vcall(model.addr(), 0x14, &[]).ptr::<()>();
    if path.is_null() || e.mem.i8(path.addr()) == 0 {
        return Ptr::NULL;
    }
    let mut root = root_override;
    if root.is_null() {
        root = e.call(ACTOR_ROOT, &args![requester]).ptr();
    }
    if root.is_null() {
        return Ptr::NULL;
    }
    let loader = e.global::<u32>(MODEL_LOADER);
    let file = e
        .call(
            LOAD_FILE,
            &args![loader, path, 3u32, 1u32, 0u32, 0u32, 0u32],
        )
        .ptr::<()>();
    let cloning = stack_alloc(e, 0x1c);
    fn_004ad050(e, cloning, 1.0);
    let handle = stack_alloc(e, 4);
    ni_pointer_new(e, handle, Ptr::NULL);
    let copy = if e.call(HAS_MORPHER_CONTROLLER, &args![file]).bool() {
        let tes = e.global::<u32>(TES_GLOBAL);
        let deep = e
            .call(DEEP_COPY_SAME_TEXTURES, &args![tes, file, cloning])
            .ptr();
        ni_pointer_assign(e, handle, deep);
        ni_pointer_get(e, handle)
    } else {
        e.call(NI_OBJECT_CLONE, &args![file, cloning]).ptr()
    };
    let mut skinned = false;
    // Virtual slot 0x1c of the model: the texture swap.
    let swap = e.vcall(model.addr(), 0x1c, &[]).u32();
    if swap != 0 {
        e.call(SWAP_TEXTURES, &args![swap, copy]);
    }
    if !form.is_null() && e.vcall(form.addr(), 0xac, &[]).bool() {
        e.call(SWAP_PLATFORM_TEXTURES, &args![copy]);
    }
    fn_004af240(e, file, copy, slot, requester.addr());
    e.call(ADD_MASTER_PARTICLE_ADDON_NODES, &args![copy]);
    let key = fn_004ab220(e);
    let extra = e.call(GET_EXTRA_DATA, &args![copy, key]).ptr();
    let held = stack_alloc(e, 4);
    ni_pointer_new(e, held, extra);
    e.call(REMOVE_ALL_EXTRA_DATA, &args![copy]);
    if !ni_pointer_get(e, held).is_null() {
        let value = ni_pointer_get(e, held);
        let key = fn_004ab220(e);
        e.call(ADD_EXTRA_DATA, &args![copy, key, value]);
        ni_pointer_assign(e, held, Ptr::NULL);
    }
    if !copy.is_null() {
        if e.call(IS_KIND_OF, &args![SKINNED_CLASS, copy]).bool() {
            e.call(SET_LOD_MULT_TYPE, &args![copy, 7u32]);
        }
        e.call(SET_TRANSLATION, &args![copy, ZERO_TRANSLATION]);
        e.call(SET_ROTATION, &args![copy, IDENTITY_ROTATION]);
        if e.call(FIND_SKINNED_NODE, &args![copy]).u8() != 0 {
            skinned = true;
        }
        if skinned {
            let name = node_name(e, copy);
            e.call(LOG, &args![ADD_ON_SKINNED_FORMAT, name]);
        } else {
            biped_anim_attach_to_parent(e, root, copy, file, Ptr::NULL, slot, 0);
        }
        if !skinned && e.vcall(copy.addr(), 0x0c, &[]).u32() != 0 {
            let above = e.call(NODE_PARENT, &args![copy]).ptr::<()>();
            if above.is_null() {
                let mut bone_node = Ptr::NULL;
                if slot != 0xffff_ffff {
                    let bone_index = e.global::<u32>(SLOT_BONES + 4 * slot);
                    if bone_index != 0xffff_ffff {
                        let name = e.global::<u32>(BONE_NAMES + 4 * bone_index);
                        bone_node = fn_004aae30(e, root, Ptr::new(name));
                    }
                }
                if !bone_node.is_null() {
                    e.vcall(bone_node.addr(), 0xdc, &args![copy, 1u32]);
                } else {
                    e.vcall(root.addr(), 0xdc, &args![copy, 1u32]);
                }
            }
        }
        e.call(UPDATE_PROPERTIES, &args![copy]);
    }
    ni_pointer_release(e, held);
    stack_free(e, held);
    ni_pointer_release(e, handle);
    stack_free(e, handle);
    ni_cloning_process_destructor(e, cloning.cast());
    stack_free(e, cloning);
    copy
}

// Translated from 004af240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::AddAddonNodes` (Xbox PDB): when the extra data the add-on key
/// (`00448a80`) names on `file` passes the test `00448a40`, adds the add-on
/// nodes to `node` (`TESObjectREFR::AddAddonNodes`). `_unused_2` and
/// `_unused_3` are words the callers pass (the slot and the requester).
pub fn fn_004af240(e: &mut Engine, file: Ptr, node: Ptr, _unused_2: u32, _unused_3: u32) {
    let key = e.call(ADDON_KEY, &[]).u32();
    let extra = e.call(GET_EXTRA_DATA, &args![file, key]).u32();
    if extra != 0 && e.call(ADDON_EXTRA_CHECK, &args![extra]).bool() {
        e.call(ADD_ADDON_NODES, &args![node]);
    }
}

// Translated from 004af290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::GetLightingProperty` (Xbox PDB): for a geometry `node`, its
/// lighting property (`GetProperty(3)`) when that property's value
/// (`00441110`) is between 8 and 12, else null. The properties of type
/// `004af350()` and `004af340()` are fetched too and not used.
pub fn biped_anim_get_lighting_property(e: &mut Engine, node: Ptr) -> Ptr {
    // Virtual slot 0x18 of the node: its geometry.
    if node.is_null() || e.vcall(node.addr(), 0x18, &[]).u32() == 0 {
        return Ptr::NULL;
    }
    let first = fn_004af350(e);
    e.call(GET_PROPERTY, &args![node, first]);
    let second = fn_004af340(e);
    e.call(GET_PROPERTY, &args![node, second]);
    let property = e.call(GET_PROPERTY, &args![node, 3u32]).ptr::<()>();
    // The value is asked for again for the upper bound, as the code does.
    let in_range = !property.is_null()
        && e.call(PROPERTY_WORLDSPACE, &args![property]).i32() >= 8
        && e.call(PROPERTY_WORLDSPACE, &args![property]).i32() <= 12;
    if in_range {
        property
    } else {
        Ptr::NULL
    }
}

// Translated from 004af340 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constant 5 (a property type).
pub fn fn_004af340(_e: &mut Engine) -> u32 {
    5
}

// Translated from 004af350 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constant 2 (a property type).
pub fn fn_004af350(_e: &mut Engine) -> u32 {
    2
}

// Translated from 004af360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::GetSkinBipedObject` (Xbox PDB): the slot a geometry's skin
/// belongs to, or -1. Takes the lighting property (`property`, or
/// [`biped_anim_get_lighting_property`] when null); without one, or without
/// its bit 10, -1. Otherwise the geometry's name is compared (case
/// insensitively, by prefix) with the five skin part names: 0, 1 and 4 give
/// slot 2, 2 gives 3 and 3 gives 4; no match logs the name and gives 2.
pub fn biped_anim_get_skin_biped_object(e: &mut Engine, node: Ptr, property: Ptr) -> u32 {
    let mut property = property;
    if property.is_null() {
        property = biped_anim_get_lighting_property(e, node);
    }
    if property.is_null() || !e.call(PROPERTY_FLAG, &args![property, 10u32]).bool() {
        return 0xffff_ffff;
    }
    let text = node_name(e, node);
    let mut result = 2;
    let mut index = 0;
    while index < 5 {
        if !text.is_null() {
            let part_name = e.global::<u32>(SKIN_PART_NAMES + 4 * index);
            let length = e.call(TEXT_LENGTH, &args![part_name]).u32();
            let part_name = e.global::<u32>(SKIN_PART_NAMES + 4 * index);
            if e.call(COMPARE_PREFIX, &args![text, part_name, length])
                .u32()
                == 0
            {
                break;
            }
        }
        index += 1;
    }
    match index {
        0 | 1 | 4 => result = 2,
        2 => result = 3,
        3 => result = 4,
        _ => {
            let above = e.call(NODE_PARENT, &args![node]).ptr::<()>();
            let above_name = node_name(e, above);
            e.call(LOG, &args![BAD_SKIN_NAME_FORMAT, text, above_name]);
        }
    }
    result
}

// Translated from 004af490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::AdjustSkinComplexion` (Xbox PDB): for a geometry `node` with a
/// lighting property and a skin slot, fetches the requester's NPC (its face
/// coordinates are read but not used further) and the race's body textures
/// for that slot (`GetBodyTexture`, with `flag` as the first person
/// argument). Missing race textures are logged. Otherwise the property gets
/// the textures: the base one in slot 0 of virtual method 0xfc (and a normal
/// map, from the base texture's path with `_n`, in slot 0 of method 0x100),
/// the second in slot 1 of method 0xfc, [`fn_004af8a0`] in slot 1 of method
/// 0x100, and its shader flag 10 is set.
pub fn biped_anim_adjust_skin_complexion(
    e: &mut Engine,
    this: Ptr<BipedAnim>,
    node: Ptr,
    flag: u8,
) {
    let property = biped_anim_get_lighting_property(e, node);
    if property.is_null() {
        return;
    }
    let skin = biped_anim_get_skin_biped_object(e, node, property);
    if skin == 0xffff_ffff {
        return;
    }
    let coords = stack_alloc(e, 0x80);
    e.call(
        VECTOR_CONSTRUCT,
        &args![coords, 0x20u32, 4u32, FACE_COORD_INIT, FACE_COORD_FREE],
    );
    let requester = e.get(this, BipedAnim::m_pRequester);
    let npc = requester_npc(e, requester);
    let base = stack_alloc(e, 4);
    ni_pointer_new(e, base, Ptr::NULL);
    let second = stack_alloc(e, 4);
    ni_pointer_new(e, second, Ptr::NULL);
    if !npc.is_null() {
        e.call(GET_FACE_COORD, &args![npc, coords]);
    }
    let race = fn_004ac110(e, npc);
    let first_person = (flag != 0) as u32;
    let found = e
        .call(
            GET_BODY_TEXTURE,
            &args![race, base, second, npc, skin, first_person],
        )
        .bool();
    if !npc.is_null() && !found {
        let race = fn_004ac110(e, npc);
        let race_name = e
            .call(RACE_NAME_TEXT, &args![race.wrapping_add(0x18)])
            .u32();
        let slot_name = e.global::<u32>(SLOT_NAMES.wrapping_add(skin.wrapping_mul(4)));
        let slot_text = e.call(IDENTITY_TEXT, &args![slot_name, 1u32]).u32();
        let sex = e.call(ACTOR_BASE_SEX, &args![npc, 0u32, slot_text]).u32();
        let sex_name = e.global::<u32>(SEX_NAMES.wrapping_add(sex.wrapping_mul(4)));
        let sex_text = e.call(IDENTITY_TEXT, &args![sex_name, 0u32]).u32();
        e.call(
            LOG,
            &args![MISSING_RACE_TEXTURE_FORMAT, sex_text, slot_text, race_name],
        );
    } else {
        let texture = ni_pointer_get(e, base);
        e.vcall(property.addr(), 0xfc, &args![0u32, texture]);
        let texture = ni_pointer_get(e, base);
        let typed = e
            .call(CAST_TO_CLASS, &args![TEXTURE_CLASS, texture])
            .ptr::<()>();
        let mut path_text: Ptr = Ptr::NULL;
        let mut mipmaps = 1u32;
        if !typed.is_null() {
            let hidden = fn_004af860(e, typed);
            mipmaps = if hidden { 0 } else { 1 };
            fn_004af880(e, typed);
            let field = e.call(TEXTURE_PATH_FIELD, &args![typed]).u32();
            path_text = e.call(NAME_TEXT, &args![field]).ptr();
        }
        let normal = stack_alloc(e, 0x104);
        e.call(
            MODIFIED_TEXTURE_FILENAME,
            &args![normal, path_text, NORMAL_MAP_SUFFIX, 1u32],
        );
        if e.mem.i8(normal.addr()) != 0 {
            let handle = stack_alloc(e, 4);
            ni_pointer_new(e, handle, Ptr::NULL);
            e.call(GET_TEXTURE, &args![normal, mipmaps, handle, 1u32, 0u32]);
            if !ni_pointer_get(e, handle).is_null() {
                let loaded = ni_pointer_get(e, handle);
                e.vcall(property.addr(), 0x100, &args![0u32, loaded]);
            }
            ni_pointer_release(e, handle);
            stack_free(e, handle);
        }
        stack_free(e, normal);
        let texture = ni_pointer_get(e, second);
        e.vcall(property.addr(), 0xfc, &args![1u32, texture]);
        let owner_texture = fn_004af8a0(e);
        e.vcall(property.addr(), 0x100, &args![1u32, owner_texture]);
        e.call(SET_SHADER_FLAG, &args![property, 10u32, 1u32]);
    }
    ni_pointer_release(e, second);
    stack_free(e, second);
    ni_pointer_release(e, base);
    stack_free(e, base);
    e.call(
        VECTOR_DESTRUCT,
        &args![coords, 0x20u32, 4u32, FACE_COORD_FREE],
    );
    stack_free(e, coords);
}

// Translated from 004af860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Always false: the code loads the signed byte at `+0x43` of `this` and
/// ANDs it with zero.
pub fn fn_004af860(_e: &mut Engine, _this: Ptr) -> bool {
    false
}

// Translated from 004af880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the byte at `+0x43` of `this` onto itself (no change).
pub fn fn_004af880(e: &mut Engine, this: Ptr) {
    let address = this.addr().wrapping_add(0x43);
    let value = e.mem.u8(address);
    e.mem.set_u8(address, value);
}

// Translated from 004af8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word of the `NiPointer` at `+0x11a0` of the object in the global
/// `011d59e8`; without that object, the word of a fresh null `NiPointer`
/// (0).
pub fn fn_004af8a0(e: &mut Engine) -> u32 {
    let owner = e.global::<u32>(TEXTURE_OWNER);
    if owner != 0 {
        return e.call(READ_WORD, &args![owner.wrapping_add(0x11a0)]).u32();
    }
    let handle = stack_alloc(e, 4);
    ni_pointer_new(e, handle, Ptr::NULL);
    let value = ni_pointer_get(e, handle);
    ni_pointer_release(e, handle);
    stack_free(e, handle);
    value.addr()
}

// Translated from 004af950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds in `buffer` the path of the first person model of `item`: its
/// model path (virtual slot 0x14) with the extension replaced by `1st.nif`;
/// true when `Data\Meshes\<that path>` exists (`FileFinder::Exist`). False
/// when the path has no extension.
pub fn fn_004af950(e: &mut Engine, _this: Ptr, item: Ptr, buffer: Ptr) -> bool {
    let path = e.vcall(item.addr(), 0x14, &[]).u32();
    e.call(STRING_COPY, &args![buffer, 0x104u32, path]);
    let dot = e.call(FIND_LAST_CHAR, &args![buffer, 0x2eu32]).u32();
    if dot == 0 {
        return false;
    }
    e.mem.set_u8(dot, 0);
    let room = 0x104u32.wrapping_sub(dot.wrapping_sub(buffer.addr()));
    e.call(STRING_APPEND, &args![dot, room, FIRST_PERSON_SUFFIX]);
    let full = stack_alloc(e, 0x104);
    e.call(
        FORMAT_INTO,
        &args![full, 0x104u32, DATA_PATH_FORMAT, MESHES_FOLDER, buffer],
    );
    let exists = e.call(FILE_EXISTS, &args![full, 0u32, 6u32, 1u32]).u32() != 0;
    stack_free(e, full);
    exists
}

// Translated from 004afa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets entry `index` of the 0x18-byte path table (`011c5d10`) to `path`
/// (`00489100`) and returns the entry's address.
pub fn fn_004afa20(e: &mut Engine, _this: Ptr, index: u32, path: Ptr) -> u32 {
    let entry = PATH_TABLE.wrapping_add(index.wrapping_mul(0x18));
    e.call(SET_PATH_ENTRY, &args![entry, path]);
    entry
}

// Translated from 004afa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The alternative path entry for `part`: `part` itself when it already is
/// the table entry of `slot`; the entry of `slot` set to the first person
/// model path of `part` ([`fn_004af950`]) when that file exists; else `part`.
pub fn fn_004afa50(e: &mut Engine, this: Ptr, slot: u32, part: Ptr) -> u32 {
    let entry = PATH_TABLE.wrapping_add(slot.wrapping_mul(0x18));
    if part.addr() == entry {
        return part.addr();
    }
    let buffer = stack_alloc(e, 0x104);
    let result = if fn_004af950(e, this, part, buffer) {
        fn_004afa20(e, this, slot, buffer)
    } else {
        part.addr()
    };
    stack_free(e, buffer);
    result
}

// Translated from 004afad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lower-cases in place the text `this` holds (a text pointer at `+0` and a
/// 16-bit length at `+4`), with the size length + 1 ([`fn_004afb00`]); does
/// nothing without text.
pub fn fn_004afad0(e: &mut Engine, this: Ptr) {
    let text = e.mem.u32(this.addr());
    if text != 0 {
        let size = e.mem.u16(this.addr().wrapping_add(4)) as u32 + 1;
        fn_004afb00(e, text, size);
    }
}

// Translated from 004afb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_strlwr_s(text, size)` (`00ec8feb`); the decompiler's name for the
/// function, `previous_character`, is a folded library name. Returns the CRT
/// function's result.
pub fn fn_004afb00(e: &mut Engine, text: u32, size: u32) -> u32 {
    e.call(STRLWR_S, &args![text, size]).u32()
}

// Translated from 004afb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the clone map (`NiTPointerMap<NiObject *,NiObject *>`,
/// whose scalar deleting destructor is the first slot of the vtable it
/// stores): the base constructor (`004aff30`) with `hash_size`, then the
/// vtable at `0101fb8c`. Returns `this`.
pub fn fn_004afb20(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    e.call(FN_004AFF30, &args![this, hash_size]);
    e.mem.set_u32(this.addr(), CLONE_MAP_VTABLE);
    this
}

// Translated from 004afb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the process map (`NiTPointerMap<NiObject *,bool>`): the
/// base constructor (`004b0030`) with `hash_size`, then the vtable at
/// `0101fbac`. Returns `this`.
pub fn fn_004afb50(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    e.call(FN_004B0030, &args![this, hash_size]);
    e.mem.set_u32(this.addr(), PROCESS_MAP_VTABLE);
    this
}

// Translated from 004afb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the recursive lock at `+0x20` of `this`.
pub fn fn_004afb80(e: &mut Engine, this: Ptr) {
    e.call(RECURSIVE_LOCK_ENTER, &args![this.byte_add(0x20), 0u32]);
}

// Translated from 004afba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the recursive lock at `+0x20` of `this`.
pub fn fn_004afba0(e: &mut Engine, this: Ptr) {
    e.call(RECURSIVE_LOCK_LEAVE, &args![this.byte_add(0x20)]);
}

// Translated from 004afbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<NiObject *,NiObject *>::scalar deleting destructor`
/// (Xbox PDB): destroys the map (`004affa0`) and, with bit 0 of `flags`,
/// frees it. Returns `this`.
pub fn ni_t_pointer_map_object_object_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(FN_004AFFA0, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004afbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<NiObject *,bool>::scalar deleting destructor` (Xbox PDB):
/// destroys the map (`004b00a0`) and, with bit 0 of `flags`, frees it.
/// Returns `this`.
pub fn ni_t_pointer_map_object_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(FN_004B00A0, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004afc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys a node array: stores its vtable (`0101fbcc`) and releases the
/// block at `+4` (`004b0220`). The map names the body `~basic_streambuf<>`,
/// a folded library name.
pub fn fn_004afc20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NODE_ARRAY_VTABLE);
    let block = e.mem.u32(this.addr().wrapping_add(4));
    e.call(FN_004B0220, &args![block]);
}

// Translated from 004afc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `item` (the address of an `NiPointer`) to the array `this` at index
/// `u16(+0xa)`, the number of elements (`004b02d0`).
pub fn fn_004afc50(e: &mut Engine, this: Ptr, item: Ptr) {
    let count = e.mem.u16(this.addr().wrapping_add(0x0a)) as u32;
    e.call(FN_004B02D0, &args![this, count, item]);
}

// Translated from 004afc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<NiAVObject>,NiTNewInterface<NiPointer<NiAVObject>>>::
/// Compact` (Xbox PDB): when the number of non-null elements (`+0xc`) differs
/// from the size (`+0xa`), moves the non-null elements to the front, then
/// reallocates the block to exactly that many elements (copying them
/// `NiPointer` by `NiPointer`) and releases the old block (`004b0220`). With
/// no elements, the block becomes null. The exception frame is not
/// translated.
pub fn fn_004afc80(e: &mut Engine, this: Ptr) {
    let block_at = this.addr().wrapping_add(4);
    let size_at = this.addr().wrapping_add(0x0a);
    let element_count_at = this.addr().wrapping_add(0x0c);
    if e.mem.u16(element_count_at) == e.mem.u16(size_at) {
        return;
    }
    if e.mem.u16(element_count_at) != 0 {
        let mut kept: u32 = 0;
        let mut index: u32 = 0;
        while index < e.mem.u16(size_at) as u32 {
            let base = e.mem.u32(block_at);
            let null_handle = stack_alloc(e, 4);
            ni_pointer_new(e, null_handle, Ptr::new(0));
            let slot = base.wrapping_add(index * 4);
            let present = e.call(NI_POINTER_DIFFERS, &args![slot, null_handle]).bool();
            ni_pointer_release(e, null_handle);
            stack_free(e, null_handle);
            if present {
                let base = e.mem.u32(block_at);
                let target = base.wrapping_add((kept & 0xffff) * 4);
                let source = base.wrapping_add(index * 4);
                if e.call(NI_POINTER_DIFFERS, &args![target, source]).bool() {
                    e.call(NI_POINTER_COPY, &args![target, source]);
                }
                kept = kept.wrapping_add(1);
            }
            index += 1;
        }
    }
    let old_block = e.mem.u32(block_at);
    let count = e.mem.u16(element_count_at);
    e.mem.set_u16(size_at, count);
    e.mem.set_u16(this.addr().wrapping_add(8), count);
    if count == 0 {
        e.mem.set_u32(block_at, 0);
    } else {
        let block = fn_005e0ba0(e, count as u32);
        e.mem.set_u32(block_at, block.addr());
        let mut index: u32 = 0;
        while index < e.mem.u16(size_at) as u32 {
            let target = block.addr().wrapping_add(index * 4);
            let source = old_block.wrapping_add(index * 4);
            e.call(NI_POINTER_COPY, &args![target, source]);
            index += 1;
        }
    }
    e.call(FN_004B0220, &args![old_block]);
}

// Translated from 004afe50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<NiPointer<NiAVObject>,NiTNewInterface<NiPointer<NiAVObject>>>::
/// UpdateSize` (Xbox PDB): lowers the size (`+0xa`) while the last element is
/// null.
pub fn fn_004afe50(e: &mut Engine, this: Ptr) {
    let size_at = this.addr().wrapping_add(0x0a);
    while e.mem.u16(size_at) != 0 {
        let size = e.mem.u16(size_at) as u32;
        let base = e.mem.u32(this.addr().wrapping_add(4));
        let last = base.wrapping_add(size * 4).wrapping_sub(4);
        let null_handle = stack_alloc(e, 4);
        ni_pointer_new(e, null_handle, Ptr::new(0));
        let present = e.call(NI_POINTER_DIFFERS, &args![last, null_handle]).bool();
        ni_pointer_release(e, null_handle);
        stack_free(e, null_handle);
        if present {
            break;
        }
        e.mem.set_u16(size_at, (size as u16).wrapping_sub(1));
    }
}

/// The shared body of the two map base constructors: stores `vtable`, the
/// bucket count (`+4`), a zero count (`+0xc`) and a zeroed table of
/// `hash_size` words (`+8`).
fn map_base_construct(e: &mut Engine, this: Ptr, hash_size: u32, vtable: u32) -> Ptr {
    e.mem.set_u32(this.addr(), vtable);
    e.mem.set_u32(this.addr().wrapping_add(4), hash_size);
    e.mem.set_u32(this.addr().wrapping_add(0x0c), 0);
    let bytes = hash_size.wrapping_shl(2);
    let table = e.call(NI_ALLOC, &args![bytes]).u32();
    e.mem.set_u32(this.addr().wrapping_add(8), table);
    let bytes = e.mem.u32(this.addr().wrapping_add(4)).wrapping_shl(2);
    e.call(MEMSET, &args![table, 0u32, bytes]);
    this
}

// Translated from 004aff30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of the clone map (`NiTMapBase<..,NiObject *,NiObject *>`):
/// vtable `0101fbdc`, `hash_size` buckets (`+4`), no items (`+0xc`) and a
/// zeroed table (`+8`). Returns `this`.
pub fn fn_004aff30(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    map_base_construct(e, this, hash_size, CLONE_MAP_BASE_VTABLE)
}

// Translated from 004affa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the clone map (`NiTPointerMap<NiObject *,NiObject *>`): the
/// vtable `0101fb8c`, `RemoveAll` (`00438af0`), then the base destructor
/// (`004b0000`). The map names the body `ctype<char>`, a folded library name.
pub fn fn_004affa0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CLONE_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(CLONE_MAP_BASE_DESTRUCT, &args![this]);
}

// Translated from 004b0030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of the process map (`NiTMapBase<..,NiObject *,bool>`):
/// as [`fn_004aff30`] with the vtable `0101fbfc`. Returns `this`.
pub fn fn_004b0030(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    map_base_construct(e, this, hash_size, PROCESS_MAP_BASE_VTABLE)
}

// Translated from 004b00a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the process map (`NiTPointerMap<NiObject *,bool>`): the
/// vtable `0101fbac`, `RemoveAll` (`00438af0`), then the base destructor
/// (`004b0100`).
pub fn fn_004b00a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), PROCESS_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(PROCESS_MAP_BASE_DESTRUCT, &args![this]);
}

// Translated from 004b01c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,NiObject *,NiObject *>::
/// scalar deleting destructor` (Xbox PDB): the base destructor (`004b0000`)
/// and, with bit 0 of `flags`, frees `this`. Returns `this`.
pub fn ni_t_map_base_object_object_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(CLONE_MAP_BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004b01f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,NiObject *,bool>::scalar
/// deleting destructor` (Xbox PDB): the base destructor (`004b0100`) and,
/// with bit 0 of `flags`, frees `this`. Returns `this`.
pub fn ni_t_map_base_object_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(PROCESS_MAP_BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 005e0ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTNewInterface<NiPointer<NiAVObject>>::Allocate` (Xbox PDB): a block of
/// `count` `NiPointer`s. `operator new` gets `4 * count + 4` bytes (all ones
/// when that overflows); the block stores `count` at its start and its
/// elements are constructed after it (`__ehvec_ctor`). Returns the address of
/// the first element, or null when the allocation failed.
pub fn fn_005e0ba0(e: &mut Engine, count: u32) -> Ptr {
    let (bytes, overflow) = count.overflowing_mul(4);
    let bytes = if overflow { u32::MAX } else { bytes };
    let (total, carry) = bytes.overflowing_add(4);
    let total = if carry { u32::MAX } else { total };
    let block = e.call(OPERATOR_NEW, &args![total]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.mem.set_u32(block, count);
    let first = block.wrapping_add(4);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            first,
            4u32,
            count,
            NI_POINTER_ELEMENT_INIT,
            NI_POINTER_RELEASE
        ],
    );
    Ptr::new(first)
}

// Translated from 00ba87e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BipedAnim::GetParentBone` (Xbox PDB): `bone[index].pParent`.
pub fn biped_anim_get_parent_bone(e: &mut Engine, this: Ptr<BipedAnim>, index: u32) -> Ptr {
    e.get(bone(this, index), BipedBone::pParent)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004aaca0,
            biped_anim_biped_anim(Ptr<BipedAnim>, Ptr, Ptr) -> Ptr<BipedAnim>
        ),
        entry!(0x004aad00, biped_anim_init_root(Ptr<BipedAnim>, Ptr)),
        entry!(0x004aae30, fn_004aae30(Ptr, Ptr) -> Ptr),
        entry!(0x004aae50, fn_004aae50(Ptr<BipedAnim>)),
        entry!(0x004aae70, biped_anim_remove_all_parts(Ptr<BipedAnim>)),
        entry!(0x004aaef0, fn_004aaef0(Ptr)),
        entry!(0x004aaf10, fn_004aaf10(Ptr)),
        entry!(
            0x004aaf30,
            fn_004aaf30(Ptr<BipedAnim>, Ptr) -> Ptr<BipedObject>
        ),
        entry!(0x004aaf90, fn_004aaf90(Ptr<BipedAnim>, Ptr)),
        entry!(0x004aaff0, fn_004aaff0(Ptr<BipedAnim>, u32, u8, Ptr)),
        entry!(
            0x004ab020,
            fn_004ab020(Ptr<BipedAnim>, Ptr<BipedObject>, u8, Ptr)
        ),
        entry!(0x004ab0c0, biped_anim_run_biped_3d_detach(Ptr)),
        entry!(
            0x004ab130,
            biped_anim_recursively_remove_all_decal_placement_vector_data_from_weapon_bone(
                Ptr<BipedAnim>,
                Ptr,
            )
        ),
        entry!(0x004ab220, fn_004ab220() -> Ptr),
        entry!(0x004ab230, fn_004ab230(Ptr<BipedAnim>, u32) -> Ptr),
        entry!(0x004ab250, fn_004ab250(Ptr<BipedAnim>, Ptr, u32)),
        entry!(0x004ab400, fn_004ab400(Ptr<BipedAnim>, Ptr, u8)),
        entry!(0x004ab500, fn_004ab500(Ptr, u8) -> u32),
        entry!(0x004ab5b0, biped_anim_remove_biped_weapon(Ptr<BipedAnim>)),
        entry!(0x004ab730, fn_004ab730(Ptr) -> bool),
        entry!(0x004ab750, fn_004ab750(Ptr<BipedAnim>, Ptr, u8)),
        entry!(0x004abad0, fn_004abad0(Ptr<BipedAnim>, Ptr, Ptr, u32)),
        entry!(0x004abd30, fn_004abd30(Ptr<BipedAnim>, u32, Ptr, u32)),
        entry!(0x004abf80, fn_004abf80(Ptr) -> u32),
        entry!(0x004abfa0, fn_004abfa0() -> bool),
        entry!(0x004abfe0, fn_004abfe0(Ptr<BipedAnim>, u32, u32, u32)),
        entry!(0x004ac110, fn_004ac110(Ptr) -> u32),
        entry!(
            0x004ac130,
            biped_anim_queue_skin_texture(Ptr<BipedAnim>, Ptr, u32, u32, u32)
        ),
        entry!(0x004ac1e0, biped_anim_load_biped_parts(Ptr<BipedAnim>, u8)),
        entry!(0x004ad010, fn_004ad010(Ptr) -> bool),
        entry!(0x004ad030, fn_004ad030(Ptr) -> bool),
        entry!(0x004ad050, fn_004ad050(Ptr, f32) -> Ptr),
        entry!(0x004ad0c0, fn_004ad0c0(Ptr<NiCloningProcess>, u32) -> Ptr),
        entry!(0x004ad1b0, fn_004ad1b0() -> u32),
        entry!(0x004ad1c0, fn_004ad1c0() -> u8),
        entry!(0x004ad1d0, fn_004ad1d0(Ptr<NiCloningProcess>)),
        entry!(0x004ad240, fn_004ad240(Ptr<NiCloningProcess>, f32)),
        entry!(
            0x004ad270,
            ni_cloning_process_destructor(Ptr<NiCloningProcess>)
        ),
        entry!(
            0x004ad290,
            biped_anim_apply_skinned_objects(Ptr<BipedAnim>, Ptr, u32, u8, u8, Ptr) -> Ptr
        ),
        entry!(
            0x004ad4a0,
            biped_anim_attach_skinned_object(Ptr<BipedAnim>, Ptr, Ptr, u32, u8, u8, Ptr) -> Ptr
        ),
        // Session 2.
        entry!(0x004add50, fn_004add50(Ptr, Ptr, u32) -> u32),
        entry!(0x004add70, fn_004add70(Ptr)),
        entry!(0x004adda0, fn_004adda0(Ptr, u32, Ptr)),
        entry!(0x004addc0, fn_004addc0(Ptr, Ptr)),
        entry!(0x004adde0, fn_004adde0(Ptr, Ptr)),
        entry!(0x004ade00, fn_004ade00(Ptr, Ptr) -> Ptr),
        entry!(0x004ade20, fn_004ade20(Ptr)),
        entry!(0x004ade40, biped_anim_attach_to_skeleton(Ptr, Ptr, Ptr, u8)),
        entry!(
            0x004ae250,
            biped_anim_attach_to_parent(Ptr, Ptr, Ptr, Ptr<BipedAnim>, u32, u32)
        ),
        entry!(0x004ae780, fn_004ae780() -> Ptr),
        entry!(
            0x004ae790,
            biped_anim_load_face_gen_model(Ptr<BipedAnim>, u32) -> Ptr
        ),
        entry!(0x004ae8a0, fn_004ae8a0(Ptr<BipedAnim>, u32) -> bool),
        entry!(0x004ae8f0, fn_004ae8f0(Ptr) -> u8),
        entry!(
            0x004ae910,
            biped_anim_clone_helmet(Ptr<BipedAnim>, Ptr, Ptr, u32) -> Ptr
        ),
        entry!(
            0x004aeb20,
            biped_anim_attach_helmet(Ptr<BipedAnim>, Ptr, Ptr, u32)
        ),
        entry!(0x004aede0, fn_004aede0(Ptr<BipedAnim>, u32, u32, u32)),
        entry!(0x004aee60, fn_004aee60(Ptr<BipedAnim>, u32, u32, Ptr, u32)),
        entry!(
            0x004aeed0,
            biped_anim_load_and_attach_add_on(Ptr, Ptr, u32, Ptr, Ptr) -> Ptr
        ),
        entry!(0x004af240, fn_004af240(Ptr, Ptr, u32, u32)),
        entry!(
            0x004af290,
            biped_anim_get_lighting_property(Ptr) -> Ptr
        ),
        entry!(0x004af340, fn_004af340() -> u32),
        entry!(0x004af350, fn_004af350() -> u32),
        entry!(
            0x004af360,
            biped_anim_get_skin_biped_object(Ptr, Ptr) -> u32
        ),
        entry!(
            0x004af490,
            biped_anim_adjust_skin_complexion(Ptr<BipedAnim>, Ptr, u8)
        ),
        entry!(0x004af860, fn_004af860(Ptr) -> bool),
        entry!(0x004af880, fn_004af880(Ptr)),
        entry!(0x004af8a0, fn_004af8a0() -> u32),
        entry!(0x004af950, fn_004af950(Ptr, Ptr, Ptr) -> bool),
        entry!(0x004afa20, fn_004afa20(Ptr, u32, Ptr) -> u32),
        entry!(0x004afa50, fn_004afa50(Ptr, u32, Ptr) -> u32),
        entry!(0x004afad0, fn_004afad0(Ptr)),
        entry!(0x004afb00, fn_004afb00(u32, u32) -> u32),
        entry!(0x004afb20, fn_004afb20(Ptr, u32) -> Ptr),
        entry!(0x004afb50, fn_004afb50(Ptr, u32) -> Ptr),
        entry!(0x004afb80, fn_004afb80(Ptr)),
        entry!(0x004afba0, fn_004afba0(Ptr)),
        entry!(
            0x004afbc0,
            ni_t_pointer_map_object_object_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x004afbf0,
            ni_t_pointer_map_object_bool_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x004afc20, fn_004afc20(Ptr)),
        entry!(0x004afc50, fn_004afc50(Ptr, Ptr)),
        entry!(0x004afc80, fn_004afc80(Ptr)),
        entry!(0x004afe50, fn_004afe50(Ptr)),
        entry!(0x004aff30, fn_004aff30(Ptr, u32) -> Ptr),
        entry!(0x004affa0, fn_004affa0(Ptr)),
        entry!(0x004b0030, fn_004b0030(Ptr, u32) -> Ptr),
        entry!(0x004b00a0, fn_004b00a0(Ptr)),
        entry!(
            0x004b01c0,
            ni_t_map_base_object_object_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x004b01f0,
            ni_t_map_base_object_bool_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x005e0ba0, fn_005e0ba0(u32) -> Ptr),
        entry!(
            0x00ba87e0,
            biped_anim_get_parent_bone(Ptr<BipedAnim>, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Every function outside this unit (and every later function of it) that
    /// the translations call; `engine` stubs them all so a test only runs the
    /// code it is about.
    const CALLEES: &[u32] = &[
        MEMSET,
        OPERATOR_NEW,
        NI_OPERATOR_NEW,
        RT_DYNAMIC_CAST,
        VECTOR_CONSTRUCT,
        VECTOR_DESTRUCT,
        COMPARE_PREFIX,
        COMPARE_TEXT,
        TEXT_LENGTH,
        LOG,
        SPRINTF,
        STRING_INIT,
        STRING_FREE,
        STRING_FROM_TEXT,
        FIXED_STRING_INIT,
        FIXED_STRING_FREE,
        SET_NAME,
        NI_POINTER_INIT,
        NI_POINTER_SET,
        NI_POINTER_RELEASE,
        READ_WORD,
        CHILD_COUNT,
        CHILD_AT,
        NAME_FIELD,
        NAME_TEXT,
        NODE_DATA_POINTER,
        NODE_DATA_FIELD,
        NODE_SKIN_DATA,
        GET_EXTRA_DATA,
        ADD_EXTRA_DATA,
        REMOVE_EXTRA_DATA,
        NI_OBJECT_CLONE,
        NI_OBJECT_CLONE_OV2,
        NI_OBJECT_DEEP_COPY,
        DEEP_COPY_SAME_TEXTURES,
        HAS_MORPHER_CONTROLLER,
        UPDATE_PROPERTIES,
        NI_NODE_INIT,
        SET_LOD_MULT_TYPE,
        FORM_TYPE,
        FORM_ID,
        REFERENCE_FORM,
        GET_FORM_AS_BIPED_MODEL,
        FILLS_BIPED_SLOT,
        BIPED_MODEL_PATH,
        SWAP_TEXTURES,
        SWAP_PLATFORM_TEXTURES,
        FORM_MODEL,
        FORM_ANIMS,
        MODEL_HAS_PATH,
        LOAD_FILE,
        QUEUE_TEXTURE,
        QUEUE_MODEL_FORM,
        QUEUE_MODEL,
        RELEASE_MODEL,
        MODIFIED_TEXTURE_FILENAME,
        BODY_MOD_TEXTURE_NAME,
        BODY_MOD_TEXTURE_FILE_NAME,
        SKIN_TEXTURE_PATH,
        FORM_SLOT_ITEM,
        READ_WORD_PLUS_4,
        GET_BIPED,
        IS_FIRST_PERSON_BIPED,
        PLAYER_NODE,
        PLAYER_ANIMATION,
        SAVED_ACQUIRE_OBJECT,
        ACTOR_ANIMATION,
        IS_WEAPON_DRAWN,
        RELOAD_TARGETS,
        BLEND_OUT,
        FIND_SKINNED_NODE,
        DATA_HANDLER_FLAG,
        SAVE_LOAD_FLAG,
        FLAGS_OBJECT_FLAG,
        SETTING_BYTE_VALUE,
        SETTING_BYTE_POINTER,
        SET_WEAPON_FLAG,
        GET_MOD_SLOTS,
        HAS_MOD_EFFECT_ACTIVE,
        WEAPON_MODEL_VALUE,
        SHOW_WEAPON,
        ACTOR_SET_WEAPON,
        ACTOR_EXTRA_LIST,
        GET_DISMEMBERMENT_EXTRA,
        LIMB_DISMEMBERED,
        HIDE_LIMB,
        ACTOR_LOG_NAME,
        ACTOR_ROOT,
        SET_TRANSLATION,
        SET_ROTATION,
        MAKE_Y_ROTATION,
        MATRIX_MULTIPLY,
        NODE_ROTATION,
        MATRIX_INIT,
        GET_FACE_COORD,
        APPLY_FACE_COORDS,
        CLEAR_BODY_PALETTE,
        FACE_COORD_INIT,
        FACE_COORD_FREE,
        CAST_TO_CLASS,
        SHADOW_SCENE_NODE,
        SHADOW_REMOVE_OBJECT,
        NODE_PARENT,
        WORLD_REMOVE_OBJECTS,
        PREPARE_DETACH,
        MUST_DEFER,
        TASK_QUEUE_GETTER,
        TASK_QUEUE_DETACH,
        LOCK_ENTER,
        LOCK_LEAVE,
        FIND_IN_LIST,
        ADD_TO_LIST,
        QUEUE_OBJECT_INIT,
        QUEUE_OBJECT_FREE,
        QUEUE_OBJECT_USE,
        SET_PARENT_OBJECT,
        BODY_PART_OWNER,
        BODY_PART_COUNT,
        BODY_PART_ARRAY,
        FIND_OBJECT_BY_NAME,
        FN_004ADDA0,
        FN_004ADDC0,
        FN_004ADDE0,
        FN_004ADE00,
        FN_004ADE20,
        FN_004ADD50,
        FN_004ADD70,
        FN_004AE250,
        FN_004AE790,
        FN_004AE8A0,
        FN_004AEDE0,
        FN_004AEE60,
        FN_004AEED0,
        FN_004AF240,
        FN_004AF490,
        FN_004AF950,
        FN_004AFA20,
        FN_004AFA50,
        FN_004AFAD0,
        FN_004AFB20,
        FN_004AFB50,
        FN_004AFB80,
        FN_004AFBA0,
        FN_004AFC50,
        FN_004AFF00,
        FN_004AFC80,
        FN_004AFE50,
        FN_004AFF30,
        FN_004B0030,
        FN_004AFFA0,
        FN_004B00A0,
        FN_004B0220,
        FN_004B02D0,
        NI_POINTER_DIFFERS,
        NI_POINTER_COPY,
        NI_ALLOC,
        MAP_REMOVE_ALL,
        CLONE_MAP_BASE_DESTRUCT,
        PROCESS_MAP_BASE_DESTRUCT,
        STRING_COPY,
        STRING_APPEND,
        FORMAT_INTO,
        FIND_LAST_CHAR,
        FILE_EXISTS,
        STRNCPY_S,
        STRLWR_S,
        OPERATOR_DELETE,
        GET_OBJECT_BY_NAME,
        IS_KIND_OF,
        NODE_CHECK_9C,
        STRING_EXTRA_TEXT,
        FIND_REFERENCE_FOR_3D,
        REMOVE_SCABBARD,
        NODE_SCALE,
        SET_NODE_SCALE,
        BUILD_RECORD,
        SHADOW_ADD_OBJECT,
        NOTIFY_ATTACHED,
        KILL_HAVOK,
        SET_MOTION,
        GET_COLLISION_OBJECT,
        COLLISION_BODY,
        BODY_SHAPE,
        COPY_LAYER,
        LAYER_OF,
        REMOVE_ALL_EXTRA_DATA,
        ADD_MASTER_PARTICLE_ADDON_NODES,
        ADD_ADDON_NODES,
        ADDON_KEY,
        ADDON_EXTRA_CHECK,
        LOADER_QUEUE_PART,
        QUEUED_FILE_ASSIGN,
        GET_EGM_FILE,
        FACE_GEN_LOAD,
        RECURSIVE_LOCK_ENTER,
        RECURSIVE_LOCK_LEAVE,
        GET_PROPERTY,
        PROPERTY_WORLDSPACE,
        PROPERTY_FLAG,
        SET_SHADER_FLAG,
        GET_BODY_TEXTURE,
        RACE_NAME_TEXT,
        ACTOR_BASE_SEX,
        IDENTITY_TEXT,
        GET_TEXTURE,
        TEXTURE_PATH_FIELD,
        SET_PATH_ENTRY,
    ];

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine whose callees are stubs returning 0 (`memset` really fills
    /// and the `NiPointer` functions keep the pointer in their four bytes),
    /// with the exe pages the translations read mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for (start, length) in [
            (0x0101_6000u32, 0x1000u32),
            (0x0118_8000, 0x1000),
            (0x0119_9000, 0x1000),
            (0x011a_9000, 0x1000),
            (0x011c_3000, 0x4000),
            (0x011d_5000, 0x1000),
            (0x011d_d000, 0x3000),
            (0x011f_4000, 0x6000),
        ] {
            e.map(start, length);
        }
        for &address in CALLEES {
            e.register(address, |_, _| Ret::default());
        }
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            Ret::default()
        });
        e.register(NI_POINTER_INIT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(NI_POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(READ_WORD, |e, a| ret(e.mem.u32(a[0])));
        // The player is not any object a test builds.
        e.set_global(PLAYER, 0x0f00_0000);
        e.call_log = Some(vec![]);
        e
    }

    /// Makes `address` return `value`.
    fn returns(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| ret(value));
    }

    /// The calls made since the last call of `log` (the log starts again).
    fn log(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.replace(vec![]).unwrap()
    }

    /// The argument words of the calls to `address` in `calls`.
    fn calls_to(calls: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        calls
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The addresses of `calls`, in order.
    fn addresses(calls: &[(u32, Vec<u32>)]) -> Vec<u32> {
        calls.iter().map(|(a, _)| *a).collect()
    }

    fn biped(e: &mut Engine) -> Ptr<BipedAnim> {
        e.new_object::<BipedAnim>()
    }

    /// A zeroed block of game memory.
    fn block(e: &mut Engine, size: u32) -> u32 {
        e.mem.alloc(size)
    }

    /// A NUL-terminated text in game memory.
    fn text(e: &mut Engine, s: &str) -> u32 {
        let address = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(address, s.as_bytes());
        address
    }

    /// An object whose vtable gives, for each `(offset, value)`, a virtual
    /// function (a double at `0x7000_0000 + vtable + offset`) returning
    /// `value`.
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x400);
        for &(offset, value) in slots {
            let target = 0x7000_0000 + vtable + offset;
            e.register_double(target, move |_, _| ret(value));
            e.mem.set_u32(vtable + offset, target);
        }
        let object = e.mem.alloc(0x300);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The address a virtual function of `object` at `offset` has.
    fn slot_target(e: &Engine, object: u32, offset: u32) -> u32 {
        e.mem.u32(e.mem.u32(object) + offset)
    }

    /// Sets slot `slot` of `this`.
    fn set_slot(e: &mut Engine, this: Ptr<BipedAnim>, slot: u32, form: u32, part: u32, clone: u32) {
        e.set(object(this, slot), BipedObject::pParent, Ptr::new(form));
        e.set(object(this, slot), BipedObject::pPart, Ptr::new(part));
        e.set(object(this, slot), BipedObject::pPartClone, Ptr::new(clone));
    }

    fn slot_words(e: &Engine, address: Ptr<BipedObject>) -> [u32; 4] {
        let a = address.addr();
        [
            e.mem.u32(a),
            e.mem.u32(a + 4),
            e.mem.u32(a + 8),
            e.mem.u32(a + 12),
        ]
    }

    /// Writes the bone names, the skin slots and the slot-to-bone table.
    fn tables(e: &mut Engine) {
        for i in 0..5 {
            e.mem.set_u32(BONE_NAMES + 4 * i, 0x0118_8c00 + 0x10 * i);
        }
        for (i, slot) in [2u32, 3, 4].into_iter().enumerate() {
            e.mem.set_u32(SKIN_SLOTS + 4 * i as u32, slot);
        }
        for slot in 0..20 {
            e.mem
                .set_u32(SLOT_NAMES + 4 * slot, 0x0118_8d00 + 0x10 * slot);
            e.mem.set_u32(SLOT_BONES + 4 * slot, 0xffff_ffff);
        }
    }

    #[test]
    fn the_unit_registers_ninety_functions() {
        let table = funcs();
        assert_eq!(table.len(), 90);
        let mut addresses: Vec<u32> = table.iter().map(|(a, _)| *a).collect();
        addresses.sort_unstable();
        addresses.dedup();
        assert_eq!(addresses.len(), 90);
        let mut e = engine();
        // Through the uniform form: the weapon flag word table and a flag bit.
        let form = block(&mut e, 0x300);
        e.mem.set_u32(form + 0x258, 77);
        assert_eq!(e.call(0x004a_b500, &args![form, 2u8]).u32(), 77);
        e.mem.set_u8(form + 0x100, 0x10);
        assert!(e.call(0x004a_b730, &args![form]).bool());
    }

    #[test]
    fn the_constructor_clears_the_object_and_stores_the_requester() {
        let mut e = engine();
        let this = biped(&mut e);
        e.mem.set_u32(this.addr() + 0x100, 0xdead_beef);
        e.mem.set_u32(this.addr() + 0x2a0, 0xdead_beef);
        e.mem.set_u32(this.addr() + 0x170, 0xdead_beef);
        let result = biped_anim_biped_anim(&mut e, this, Ptr::new(0x2222_0000), Ptr::NULL);
        assert_eq!(result, this);
        assert_eq!(e.get(this, BipedAnim::m_pRequester).addr(), 0x2222_0000);
        assert_eq!(e.mem.u32(this.addr() + 0x100), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x170), 0);
        let calls = log(&mut e);
        assert_eq!(
            calls,
            vec![
                (MEMSET, vec![this.addr(), 0, 0x2b4]),
                (MEMSET, vec![this.addr() + 0x16c, 0, 0x140]),
            ]
        );
    }

    #[test]
    fn the_constructor_finds_the_bones_when_given_a_root() {
        let mut e = engine();
        tables(&mut e);
        let this = biped(&mut e);
        biped_anim_biped_anim(&mut e, this, Ptr::new(0x2222_0000), Ptr::new(0x3333_0000));
        let calls = log(&mut e);
        // The root node name and the five bones are looked up under the root.
        let finds = calls_to(&calls, FIND_OBJECT_BY_NAME);
        assert_eq!(finds.len(), 6);
        assert_eq!(finds[0], vec![0x3333_0000, ROOT_NODE_NAME, 1]);
        assert_eq!(finds[1], vec![0x3333_0000, 0x0118_8c00, 1]);
    }

    #[test]
    fn init_root_flags_and_stores_the_bones_that_are_nodes_and_logs_the_others() {
        let mut e = engine();
        tables(&mut e);
        let this = biped(&mut e);
        let root_node = 0x4444_0000;
        let node = object_with_slots(&mut e, &[(0x0c, 1)]);
        let not_a_node = object_with_slots(&mut e, &[(0x0c, 0)]);
        let table: Vec<(u32, u32)> = vec![
            (ROOT_NODE_NAME, root_node),
            (0x0118_8c00, node),
            (0x0118_8c10, not_a_node),
            (0x0118_8c20, 0),
            (0x0118_8c30, node),
            (0x0118_8c40, 0),
        ];
        e.register_double(FIND_OBJECT_BY_NAME, move |_, a| {
            ret(table.iter().find(|(name, _)| *name == a[1]).unwrap().1)
        });
        returns(&mut e, NAME_FIELD, 0x1234);
        returns(&mut e, NAME_TEXT, 0x5678);
        biped_anim_init_root(&mut e, this, Ptr::new(0x3333_0000));
        assert_eq!(e.get(this, BipedAnim::root).addr(), root_node);
        let flags: Vec<u8> = (0..5)
            .map(|i| e.get(bone(this, i), BipedBone::cFlags))
            .collect();
        assert_eq!(flags, vec![1, 0, 0, 1, 0]);
        assert_eq!(e.get(bone(this, 0), BipedBone::pParent).addr(), node);
        assert_eq!(e.get(bone(this, 3), BipedBone::pParent).addr(), node);
        assert!(e.get(bone(this, 1), BipedBone::pParent).is_null());
        let calls = log(&mut e);
        // The three bones that are missing or not nodes are logged with the
        // name of the root's owner.
        assert_eq!(
            calls_to(&calls, LOG),
            vec![
                vec![MISSING_BONE_FORMAT, 0x0118_8c10, 0x5678],
                vec![MISSING_BONE_FORMAT, 0x0118_8c20, 0x5678],
                vec![MISSING_BONE_FORMAT, 0x0118_8c40, 0x5678],
            ]
        );
        assert_eq!(calls_to(&calls, NAME_TEXT)[0], vec![0x1234]);
        // Each NiPointer temporary is released.
        assert_eq!(calls_to(&calls, NI_POINTER_RELEASE).len(), 5);
    }

    #[test]
    fn find_object_by_name_asks_for_the_recursive_search() {
        let mut e = engine();
        returns(&mut e, FIND_OBJECT_BY_NAME, 0x77);
        let found = fn_004aae30(&mut e, Ptr::new(0x10), Ptr::new(0x20));
        assert_eq!(found.addr(), 0x77);
        assert_eq!(
            calls_to(&log(&mut e), FIND_OBJECT_BY_NAME),
            vec![vec![0x10, 0x20, 1]]
        );
    }

    #[test]
    fn remove_all_parts_clears_every_slot_between_the_locks() {
        let mut e = engine();
        let this = biped(&mut e);
        let loader = block(&mut e, 0x10);
        e.mem.set_u32(loader, 0x4444);
        e.set_global(MODEL_LOADER, loader);
        set_slot(&mut e, this, 3, 0x1111, 0x2222, 0);
        e.set(buffered(this, 4), BipedObject::pParent, Ptr::new(0x3333));
        e.set(buffered(this, 4), BipedObject::pPart, Ptr::new(0x3334));
        biped_anim_remove_all_parts(&mut e, this);
        for slot in 0..20 {
            assert_eq!(slot_words(&e, object(this, slot)), [0; 4]);
            assert_eq!(slot_words(&e, buffered(this, slot)), [0; 4]);
        }
        let calls = log(&mut e);
        let order = addresses(&calls);
        assert_eq!(order[0], FN_004AFB80);
        assert_eq!(order[1], LOCK_ENTER);
        assert_eq!(order[order.len() - 2], LOCK_LEAVE);
        assert_eq!(order[order.len() - 1], FN_004AFBA0);
        assert_eq!(calls[0].1, vec![0x4444]);
        assert_eq!(calls[1].1, vec![PART_LOCK, 0]);
        assert_eq!(calls[calls.len() - 1].1, vec![0x4444]);
    }

    #[test]
    fn the_wrapper_removes_all_parts() {
        let mut e = engine();
        let this = biped(&mut e);
        let loader = block(&mut e, 0x10);
        e.set_global(MODEL_LOADER, loader);
        set_slot(&mut e, this, 2, 0x1111, 0x2222, 0);
        fn_004aae50(&mut e, this);
        assert_eq!(slot_words(&e, object(this, 2)), [0; 4]);
    }

    #[test]
    fn the_loader_lock_helpers_call_the_object_in_the_first_field() {
        let mut e = engine();
        let loader = block(&mut e, 0x10);
        e.mem.set_u32(loader, 0x4444);
        fn_004aaef0(&mut e, Ptr::new(loader));
        fn_004aaf10(&mut e, Ptr::new(loader));
        let calls = log(&mut e);
        assert_eq!(
            calls,
            vec![(FN_004AFB80, vec![0x4444]), (FN_004AFBA0, vec![0x4444])]
        );
    }

    #[test]
    fn the_slot_of_a_form_is_found_or_null() {
        let mut e = engine();
        let this = biped(&mut e);
        set_slot(&mut e, this, 9, 0xaaaa, 0, 0);
        set_slot(&mut e, this, 4, 0xbbbb, 0, 0);
        set_slot(&mut e, this, 12, 0xaaaa, 0, 0);
        assert_eq!(fn_004aaf30(&mut e, this, Ptr::new(0xaaaa)), object(this, 9));
        assert_eq!(fn_004aaf30(&mut e, this, Ptr::new(0xbbbb)), object(this, 4));
        assert!(fn_004aaf30(&mut e, this, Ptr::new(0xcccc)).is_null());
        // A null form finds nothing even though empty slots exist.
        assert!(fn_004aaf30(&mut e, this, Ptr::NULL).is_null());
    }

    #[test]
    fn clearing_the_slots_of_a_form_leaves_the_others() {
        let mut e = engine();
        let this = biped(&mut e);
        set_slot(&mut e, this, 9, 0xaaaa, 0x10, 0);
        set_slot(&mut e, this, 4, 0xbbbb, 0x20, 0);
        set_slot(&mut e, this, 12, 0xaaaa, 0x30, 0);
        fn_004aaf90(&mut e, this, Ptr::new(0xaaaa));
        assert_eq!(slot_words(&e, object(this, 9)), [0; 4]);
        assert_eq!(slot_words(&e, object(this, 12)), [0; 4]);
        assert_eq!(slot_words(&e, object(this, 4)), [0xbbbb, 0x20, 0, 0]);
        // A null form clears nothing.
        log(&mut e);
        fn_004aaf90(&mut e, this, Ptr::NULL);
        assert!(log(&mut e).is_empty());
        assert_eq!(slot_words(&e, object(this, 4)), [0xbbbb, 0x20, 0, 0]);
    }

    #[test]
    fn clearing_a_slot_by_number_replaces_the_part_when_asked() {
        let mut e = engine();
        let this = biped(&mut e);
        set_slot(&mut e, this, 4, 0xbbbb, 0x20, 0);
        fn_004aaff0(&mut e, this, 4, 0, Ptr::new(0x99));
        assert_eq!(slot_words(&e, object(this, 4)), [0xbbbb, 0x20, 0, 0]);
        fn_004aaff0(&mut e, this, 4, 1, Ptr::new(0x99));
        assert_eq!(slot_words(&e, object(this, 4)), [0, 0x99, 0, 0]);
    }

    /// A slot with a loaded clone, whose model has a path.
    fn slot_with_clone(e: &mut Engine, this: Ptr<BipedAnim>, slot: u32, clone: u32) {
        let model = object_with_slots(e, &[(0x14, 0xabc0)]);
        set_slot(e, this, slot, 0x1111, model, clone);
    }

    #[test]
    fn emptying_a_slot_with_a_clone_detaches_it_and_releases_the_model() {
        let mut e = engine();
        let this = biped(&mut e);
        e.set_global(MODEL_LOADER, 0x5555);
        slot_with_clone(&mut e, this, 6, 0x6666);
        fn_004ab020(&mut e, this, object(this, 6), 1, Ptr::NULL);
        let calls = log(&mut e);
        // Not deferred: the clone is detached at once.
        assert_eq!(calls_to(&calls, MUST_DEFER), vec![Vec::<u32>::new()]);
        assert_eq!(calls_to(&calls, PREPARE_DETACH), vec![vec![0x6666]]);
        assert!(calls_to(&calls, TASK_QUEUE_DETACH).is_empty());
        assert_eq!(calls_to(&calls, RELEASE_MODEL), vec![vec![0x5555, 0xabc0]]);
        assert_eq!(slot_words(&e, object(this, 6)), [0; 4]);
    }

    #[test]
    fn emptying_a_slot_queues_the_detach_when_it_must_be_deferred() {
        let mut e = engine();
        let this = biped(&mut e);
        e.set_global(MODEL_LOADER, 0x5555);
        returns(&mut e, MUST_DEFER, 1);
        returns(&mut e, TASK_QUEUE_GETTER, 0x7700);
        slot_with_clone(&mut e, this, 6, 0x6666);
        fn_004ab020(&mut e, this, object(this, 6), 0, Ptr::NULL);
        let calls = log(&mut e);
        assert!(calls_to(&calls, PREPARE_DETACH).is_empty());
        assert_eq!(
            calls_to(&calls, TASK_QUEUE_DETACH),
            vec![vec![0x7700, 0x6666]]
        );
        // The clone is gone, the form stays (clear == 0).
        assert!(e.get(object(this, 6), BipedObject::pPartClone).is_null());
        assert_eq!(e.get(object(this, 6), BipedObject::pParent).addr(), 0x1111);
    }

    #[test]
    fn emptying_a_slot_without_a_clone_only_clears_the_form() {
        let mut e = engine();
        let this = biped(&mut e);
        set_slot(&mut e, this, 6, 0x1111, 0x2222, 0);
        fn_004ab020(&mut e, this, object(this, 6), 1, Ptr::new(0x77));
        assert!(log(&mut e).is_empty());
        assert_eq!(slot_words(&e, object(this, 6)), [0, 0x77, 0, 0]);
    }

    #[test]
    fn detaching_a_node_removes_it_from_the_shadow_scene_and_its_parent() {
        let mut e = engine();
        biped_anim_run_biped_3d_detach(&mut e, Ptr::NULL);
        assert!(log(&mut e).is_empty());
        // No parent: only the preparation and the shadow scene node.
        returns(&mut e, SHADOW_SCENE_NODE, 0x8800);
        biped_anim_run_biped_3d_detach(&mut e, Ptr::new(0x6666));
        let calls = log(&mut e);
        assert_eq!(
            addresses(&calls),
            vec![
                PREPARE_DETACH,
                SHADOW_SCENE_NODE,
                SHADOW_REMOVE_OBJECT,
                NODE_PARENT
            ]
        );
        assert_eq!(calls[2].1, vec![0x8800, 0x6666]);
        // With a parent: the physics objects and the parent's slot 0xe8.
        let parent = object_with_slots(&mut e, &[(0xe8, 0)]);
        returns(&mut e, NODE_PARENT, parent);
        biped_anim_run_biped_3d_detach(&mut e, Ptr::new(0x6666));
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, WORLD_REMOVE_OBJECTS),
            vec![vec![0x6666, 1, 0]]
        );
        let target = slot_target(&e, parent, 0xe8);
        assert_eq!(calls_to(&calls, target), vec![vec![parent, 0x6666]]);
    }

    #[test]
    fn decal_data_removal_does_nothing_without_the_extra_data() {
        let mut e = engine();
        let this = biped(&mut e);
        e.set(bone(this, 1), BipedBone::pParent, Ptr::new(0x7000));
        e.set_global(EXTRA_DATA_KEY, 0x9900);
        biped_anim_recursively_remove_all_decal_placement_vector_data_from_weapon_bone(
            &mut e,
            this,
            Ptr::NULL,
        );
        assert!(log(&mut e).is_empty());
        biped_anim_recursively_remove_all_decal_placement_vector_data_from_weapon_bone(
            &mut e,
            this,
            Ptr::new(0x6666),
        );
        assert_eq!(log(&mut e), vec![(GET_EXTRA_DATA, vec![0x7000, 0x9900])]);
    }

    #[test]
    fn decal_data_of_a_geometry_found_in_the_list_is_removed_from_the_bone() {
        let mut e = engine();
        let this = biped(&mut e);
        e.set(bone(this, 1), BipedBone::pParent, Ptr::new(0x7000));
        e.set_global(EXTRA_DATA_KEY, 0x9900);
        returns(&mut e, GET_EXTRA_DATA, 0x8000);
        returns(&mut e, NODE_DATA_POINTER, 0x1);
        let geometry = object_with_slots(&mut e, &[(0x1c, 1)]);
        // The key the list is searched with is the node, in a stack word.
        let searched = Rc::new(RefCell::new(0));
        let seen = searched.clone();
        e.register_double(FIND_IN_LIST, move |e, a| {
            *seen.borrow_mut() = e.mem.u32(a[1]);
            ret(1)
        });
        biped_anim_recursively_remove_all_decal_placement_vector_data_from_weapon_bone(
            &mut e,
            this,
            Ptr::new(geometry),
        );
        assert_eq!(*searched.borrow(), geometry);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, FIND_IN_LIST)[0][0], 0x8034);
        assert_eq!(calls_to(&calls, FIND_IN_LIST)[0][2], 0);
        assert_eq!(
            calls_to(&calls, REMOVE_EXTRA_DATA),
            vec![vec![0x7000, 0x9900]]
        );
    }

    #[test]
    fn decal_data_removal_recurses_through_the_children_of_a_node() {
        let mut e = engine();
        let this = biped(&mut e);
        e.set(bone(this, 1), BipedBone::pParent, Ptr::new(0x7000));
        e.set_global(EXTRA_DATA_KEY, 0x9900);
        returns(&mut e, GET_EXTRA_DATA, 0x8000);
        let leaf = object_with_slots(&mut e, &[(0x1c, 1)]);
        let list = 0x6600_0000;
        let node = object_with_slots(&mut e, &[(0x1c, 0), (0x0c, list)]);
        returns(&mut e, CHILD_COUNT, 2);
        e.register_double(CHILD_AT, move |_, _| ret(leaf));
        biped_anim_recursively_remove_all_decal_placement_vector_data_from_weapon_bone(
            &mut e,
            this,
            Ptr::new(node),
        );
        let calls = log(&mut e);
        // The node and its two children each look the extra data up; the
        // leaves have no data so nothing is removed.
        assert_eq!(calls_to(&calls, GET_EXTRA_DATA).len(), 3);
        assert!(calls_to(&calls, REMOVE_EXTRA_DATA).is_empty());
        assert_eq!(
            calls_to(&calls, CHILD_AT),
            vec![vec![list, 0], vec![list, 1]]
        );
    }

    #[test]
    fn the_key_and_bone_getters_read_the_global_and_the_bone() {
        let mut e = engine();
        let this = biped(&mut e);
        e.set_global(EXTRA_DATA_KEY, 0x9900);
        assert_eq!(fn_004ab220(&mut e).addr(), 0x9900);
        e.set(bone(this, 3), BipedBone::pParent, Ptr::new(0x7003));
        assert_eq!(fn_004ab230(&mut e, this, 3).addr(), 0x7003);
        assert!(fn_004ab230(&mut e, this, 2).is_null());
    }

    #[test]
    fn resetting_the_slots_for_an_actor_moves_them_to_the_buffer() {
        let mut e = engine();
        tables(&mut e);
        let this = biped(&mut e);
        let requester = object_with_slots(&mut e, &[(0x1d0, 1)]);
        e.set(this, BipedAnim::m_pRequester, Ptr::new(requester));
        set_slot(&mut e, this, 3, 0x1111, 0x2222, 0x3333);
        returns(&mut e, FORM_SLOT_ITEM, 0);
        log(&mut e);
        fn_004ab250(&mut e, this, Ptr::new(0xf0f0), 0x55);
        // The buffer was empty, so it took the slots over; the slots are clear.
        assert_eq!(
            slot_words(&e, buffered(this, 3)),
            [0x1111, 0x2222, 0x3333, 0]
        );
        assert_eq!(slot_words(&e, object(this, 3)), [0; 4]);
        // The three skin slots (no bone: any) ask the form for their item.
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, FORM_SLOT_ITEM),
            vec![
                vec![0xf0f0, 0x55, 2],
                vec![0xf0f0, 0x55, 3],
                vec![0xf0f0, 0x55, 4]
            ]
        );
    }

    #[test]
    fn resetting_the_slots_keeps_a_used_buffer() {
        let mut e = engine();
        tables(&mut e);
        let this = biped(&mut e);
        let requester = object_with_slots(&mut e, &[(0x1d0, 1)]);
        e.set(this, BipedAnim::m_pRequester, Ptr::new(requester));
        set_slot(&mut e, this, 3, 0x1111, 0x2222, 0x3333);
        e.set(buffered(this, 9), BipedObject::pParent, Ptr::new(0x4444));
        fn_004ab250(&mut e, this, Ptr::new(0xf0f0), 0);
        assert_eq!(slot_words(&e, buffered(this, 3)), [0; 4]);
        assert_eq!(slot_words(&e, buffered(this, 9))[0], 0x4444);
        assert_eq!(slot_words(&e, object(this, 3)), [0; 4]);
    }

    #[test]
    fn resetting_the_slots_without_an_actor_clears_both_arrays() {
        let mut e = engine();
        tables(&mut e);
        let this = biped(&mut e);
        set_slot(&mut e, this, 3, 0x1111, 0x2222, 0x3333);
        // The requester's virtual slot 0x1d0 says no.
        let requester = object_with_slots(&mut e, &[(0x1d0, 0)]);
        e.set(this, BipedAnim::m_pRequester, Ptr::new(requester));
        fn_004ab250(&mut e, this, Ptr::new(0xf0f0), 0);
        assert_eq!(slot_words(&e, object(this, 3)), [0; 4]);
        assert_eq!(slot_words(&e, buffered(this, 3)), [0; 4]);
    }

    #[test]
    fn resetting_the_slots_skips_a_skin_slot_whose_bone_was_not_found() {
        let mut e = engine();
        tables(&mut e);
        // Slot 3 attaches to bone 1, which was not found; slot 4 to bone 2.
        e.mem.set_u32(SLOT_BONES + 4 * 3, 1);
        e.mem.set_u32(SLOT_BONES + 4 * 4, 2);
        let this = biped(&mut e);
        e.set(bone(this, 2), BipedBone::cFlags, 1);
        returns(&mut e, FORM_SLOT_ITEM, 0);
        log(&mut e);
        fn_004ab250(&mut e, this, Ptr::new(0xf0f0), 0);
        let slots: Vec<u32> = calls_to(&log(&mut e), FORM_SLOT_ITEM)
            .into_iter()
            .map(|a| a[2])
            .collect();
        assert_eq!(slots, vec![2, 4]);
        // Without a form nothing happens at all.
        fn_004ab250(&mut e, this, Ptr::NULL, 0);
        assert!(log(&mut e).is_empty());
    }

    /// A weapon form: a block with room for the +0x250.. words and the flags.
    fn weapon_form(e: &mut Engine) -> u32 {
        let form = block(e, 0x300);
        returns(e, FORM_TYPE, 0x28);
        form
    }

    #[test]
    fn choosing_the_weapon_stores_the_form_and_its_model() {
        let mut e = engine();
        let this = biped(&mut e);
        let form = weapon_form(&mut e);
        returns(&mut e, FORM_MODEL, 0xaaaa);
        set_slot(&mut e, this, 5, 0x7777, 0x8888, 0);
        log(&mut e);
        fn_004ab400(&mut e, this, Ptr::new(form), 3);
        assert_eq!(
            slot_words(&e, object(this, 5)),
            [form, 0xaaaa, 0, 0],
            "the old weapon was cleared first"
        );
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, FORM_MODEL), vec![vec![form, 3, 1]]);
    }

    #[test]
    fn choosing_the_weapon_ignores_other_forms_and_a_loaded_weapon() {
        let mut e = engine();
        let this = biped(&mut e);
        let form = block(&mut e, 0x300);
        returns(&mut e, FORM_TYPE, 0x2a);
        fn_004ab400(&mut e, this, Ptr::new(form), 0);
        fn_004ab400(&mut e, this, Ptr::NULL, 0);
        assert_eq!(slot_words(&e, object(this, 5)), [0; 4]);
        // The same weapon with a clone is not chosen again.
        returns(&mut e, FORM_TYPE, 0x28);
        set_slot(&mut e, this, 5, form, 0x8888, 0x9999);
        log(&mut e);
        fn_004ab400(&mut e, this, Ptr::new(form), 0);
        assert!(calls_to(&log(&mut e), FORM_MODEL).is_empty());
        assert_eq!(slot_words(&e, object(this, 5)), [form, 0x8888, 0x9999, 0]);
    }

    #[test]
    fn the_players_biped_gets_the_weapons_animation_model() {
        let mut e = engine();
        let this = biped(&mut e);
        let form = weapon_form(&mut e);
        returns(&mut e, FORM_MODEL, 0xaaaa);
        returns(&mut e, FORM_ANIMS, 0xb000);
        e.set_global(PLAYER, 0x1000);
        // GetBiped(1) is this biped.
        returns(&mut e, GET_BIPED, this.addr());
        fn_004ab400(&mut e, this, Ptr::new(form), 0);
        assert_eq!(e.get(object(this, 5), BipedObject::pPart).addr(), 0xb030);
        // With flags, the word the flags select is used, except for the
        // special form id.
        e.mem.set_u32(form + 0x254, 0xc000);
        returns(&mut e, FORM_ID, 0x1234);
        fn_004ab400(&mut e, this, Ptr::new(form), 1);
        assert_eq!(e.get(object(this, 5), BipedObject::pPart).addr(), 0xc030);
        returns(&mut e, FORM_ID, 0x0017_35d4);
        fn_004ab400(&mut e, this, Ptr::new(form), 1);
        assert_eq!(e.get(object(this, 5), BipedObject::pPart).addr(), 0xb030);
        // No animation model: the form's own model stays.
        returns(&mut e, FORM_ANIMS, 0);
        fn_004ab400(&mut e, this, Ptr::new(form), 0);
        assert_eq!(e.get(object(this, 5), BipedObject::pPart).addr(), 0xaaaa);
    }

    #[test]
    fn the_third_person_biped_of_the_player_also_gets_it() {
        let mut e = engine();
        let this = biped(&mut e);
        let form = weapon_form(&mut e);
        returns(&mut e, FORM_MODEL, 0xaaaa);
        returns(&mut e, FORM_ANIMS, 0xb000);
        e.set_global(PLAYER, 0x1000);
        let third = this.addr();
        e.register_double(GET_BIPED, move |_, a| {
            ret(if a[1] == 0 { third } else { 0 })
        });
        fn_004ab400(&mut e, this, Ptr::new(form), 0);
        assert_eq!(e.get(object(this, 5), BipedObject::pPart).addr(), 0xb030);
    }

    #[test]
    fn the_weapon_word_is_chosen_by_the_low_three_flag_bits() {
        let mut e = engine();
        let form = block(&mut e, 0x300);
        for (i, offset) in (0x250..=0x26c).step_by(4).enumerate() {
            e.mem.set_u32(form + offset, 100 + i as u32);
        }
        // flags 0..7: bit 0 adds 4, bit 1 adds 8, bit 2 adds 0xc, except that
        // bits 0 and 1 together give +0x10.
        let expected_offsets = [0x250, 0x254, 0x258, 0x260, 0x25c, 0x268, 0x264, 0x26c];
        for (flags, offset) in expected_offsets.into_iter().enumerate() {
            assert_eq!(
                fn_004ab500(&mut e, Ptr::new(form), flags as u8),
                e.mem.u32(form + offset),
                "flags {flags}"
            );
        }
        // Higher bits are ignored.
        assert_eq!(
            fn_004ab500(&mut e, Ptr::new(form), 0x18),
            e.mem.u32(form + 0x250)
        );
    }

    #[test]
    fn the_weapon_flag_is_bit_four_of_the_byte_at_0x100() {
        let mut e = engine();
        let form = block(&mut e, 0x300);
        assert!(!fn_004ab730(&mut e, Ptr::new(form)));
        e.mem.set_u8(form + 0x100, 0xef);
        assert!(!fn_004ab730(&mut e, Ptr::new(form)));
        e.mem.set_u8(form + 0x100, 0x10);
        assert!(fn_004ab730(&mut e, Ptr::new(form)));
    }

    /// An actor-like requester for the weapon removal: virtual slot 0x100 gives
    /// `is_actor`.
    fn requester_with(e: &mut Engine, this: Ptr<BipedAnim>, is_actor: u32) -> u32 {
        let requester = object_with_slots(e, &[(0x100, is_actor)]);
        e.set(this, BipedAnim::m_pRequester, Ptr::new(requester));
        requester
    }

    #[test]
    fn removing_the_weapon_clears_its_slots_and_reloads_the_targets() {
        let mut e = engine();
        let this = biped(&mut e);
        let form = block(&mut e, 0x300);
        e.mem.set_u8(form + 0x100, 0x10);
        set_slot(&mut e, this, 5, form, 0x2222, 0);
        // Slot 7's clone carries the weapon flag.
        set_slot(&mut e, this, 7, 0x1212, 0x3434, 0x5656);
        e.set(buffered(this, 5), BipedObject::pParent, Ptr::new(0x6161));
        e.set(buffered(this, 2), BipedObject::pParent, Ptr::new(0x6161));
        e.set(buffered(this, 3), BipedObject::pParent, Ptr::new(0x6262));
        let requester = requester_with(&mut e, this, 1);
        returns(&mut e, SAVED_ACQUIRE_OBJECT, 0x4141);
        returns(&mut e, GET_BIPED, 0x1);
        let backpack = 0x7171;
        returns(&mut e, FIND_OBJECT_BY_NAME, backpack);
        let parent = object_with_slots(&mut e, &[(0xe8, 0)]);
        returns(&mut e, NODE_PARENT, parent);
        // The model of slot 7 is released through its (stubbed) loader.
        e.set(this, BipedAnim::root, Ptr::new(0x1818));
        log(&mut e);
        biped_anim_remove_biped_weapon(&mut e, this);
        assert_eq!(slot_words(&e, object(this, 5)), [0; 4]);
        assert_eq!(slot_words(&e, buffered(this, 5)), [0; 4]);
        assert_eq!(slot_words(&e, buffered(this, 2)), [0; 4]);
        assert_eq!(slot_words(&e, buffered(this, 3))[0], 0x6262);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, SET_WEAPON_FLAG), vec![vec![0x5656, 0]]);
        assert_eq!(calls_to(&calls, RELOAD_TARGETS), vec![vec![requester, 0]]);
        assert_eq!(
            calls_to(&calls, FIND_OBJECT_BY_NAME),
            vec![vec![0x1818, BACKPACK_NAME, 1]]
        );
        let target = slot_target(&e, parent, 0xe8);
        assert_eq!(calls_to(&calls, target), vec![vec![parent, backpack]]);
    }

    #[test]
    fn removing_the_weapon_keeps_the_targets_for_the_first_person_biped_and_non_actors() {
        let mut e = engine();
        let this = biped(&mut e);
        requester_with(&mut e, this, 1);
        returns(&mut e, SAVED_ACQUIRE_OBJECT, 0x4141);
        // GetBiped(1) is this biped: no reload.
        returns(&mut e, GET_BIPED, this.addr());
        log(&mut e);
        biped_anim_remove_biped_weapon(&mut e, this);
        let calls = log(&mut e);
        assert!(calls_to(&calls, RELOAD_TARGETS).is_empty());
        // A data handler flag blocks it too.
        returns(&mut e, GET_BIPED, 0x1);
        returns(&mut e, DATA_HANDLER_FLAG, 1);
        biped_anim_remove_biped_weapon(&mut e, this);
        assert!(calls_to(&log(&mut e), RELOAD_TARGETS).is_empty());
        // Not an actor: neither the reload nor the backpack lookup.
        returns(&mut e, DATA_HANDLER_FLAG, 0);
        requester_with(&mut e, this, 0);
        biped_anim_remove_biped_weapon(&mut e, this);
        let calls = log(&mut e);
        assert!(calls_to(&calls, RELOAD_TARGETS).is_empty());
        assert!(calls_to(&calls, FIND_OBJECT_BY_NAME).is_empty());
    }

    #[test]
    fn loading_a_weapon_attaches_names_and_flags_it() {
        let mut e = engine();
        tables(&mut e);
        let this = biped(&mut e);
        // The weapon form has a name (virtual slot 0x130) and the weapon flag.
        let named = object_with_slots(&mut e, &[(0x130, 0x7e7e)]);
        e.mem.set_u8(named + 0x100, 0x10);
        let requester = requester_with(&mut e, this, 0);
        returns(&mut e, FORM_MODEL, 0xaaaa);
        returns(&mut e, FORM_ID, 0x4321);
        returns(&mut e, FN_004AEED0, 0x9090);
        returns(&mut e, FORM_TYPE, 0x28);
        // Slot 7 has a clone that gets the weapon flag.
        set_slot(&mut e, this, 7, 0x1212, 0x3434, 0x5656);
        returns(&mut e, FIXED_STRING_INIT, 0x1f1f);
        log(&mut e);
        fn_004ab750(&mut e, this, Ptr::new(named), 0);
        assert_eq!(slot_words(&e, object(this, 5)), [named, 0xaaaa, 0x9090, 0]);
        let calls = log(&mut e);
        // Not the player: no first person node.
        assert_eq!(
            calls_to(&calls, FN_004AEED0),
            vec![vec![named, 0xaaaa, 5, requester, 0]]
        );
        let slot_name = e.mem.u32(SLOT_NAMES + 4 * 5);
        let sprintf = calls_to(&calls, SPRINTF);
        assert_eq!(sprintf.len(), 1);
        assert_eq!(
            sprintf[0][1..],
            [SLOT_NAME_FORMAT, slot_name, 0x7e7e, 0x4321]
        );
        assert_eq!(calls_to(&calls, SET_NAME), vec![vec![0x9090, 0x1f1f]]);
        assert_eq!(calls_to(&calls, SET_WEAPON_FLAG), vec![vec![0x5656, 1]]);
    }

    #[test]
    fn a_weapon_that_fails_to_load_is_not_named() {
        let mut e = engine();
        tables(&mut e);
        let this = biped(&mut e);
        let form = weapon_form(&mut e);
        fn_004ab750(&mut e, this, Ptr::new(form), 0);
        let calls = log(&mut e);
        assert!(calls_to(&calls, SPRINTF).is_empty());
        assert!(calls_to(&calls, STRING_INIT).is_empty());
        // Other forms and a loaded weapon are ignored.
        returns(&mut e, FORM_TYPE, 0x2a);
        fn_004ab750(&mut e, this, Ptr::new(form), 0);
        fn_004ab750(&mut e, this, Ptr::NULL, 0);
        assert!(calls_to(&log(&mut e), FN_004AEED0).is_empty());
        returns(&mut e, FORM_TYPE, 0x28);
        set_slot(&mut e, this, 5, form, 1, 2);
        fn_004ab750(&mut e, this, Ptr::new(form), 0);
        assert!(calls_to(&log(&mut e), FN_004AEED0).is_empty());
    }

    #[test]
    fn the_player_loads_the_weapon_with_the_first_person_node() {
        let mut e = engine();
        tables(&mut e);
        let this = biped(&mut e);
        let form = weapon_form(&mut e);
        let requester = 0x1000;
        e.set_global(PLAYER, requester);
        e.set(this, BipedAnim::m_pRequester, Ptr::new(requester));
        returns(&mut e, FORM_MODEL, 0xaaaa);
        returns(&mut e, IS_FIRST_PERSON_BIPED, 1);
        returns(&mut e, PLAYER_NODE, 0x2468);
        returns(&mut e, FN_004AEED0, 0);
        log(&mut e);
        fn_004ab750(&mut e, this, Ptr::new(form), 0);
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, IS_FIRST_PERSON_BIPED),
            vec![vec![requester, this.addr()]]
        );
        assert_eq!(calls_to(&calls, PLAYER_NODE), vec![vec![requester, 1]]);
        assert_eq!(
            calls_to(&calls, FN_004AEED0),
            vec![vec![form, 0xaaaa, 5, requester, 0x2468]]
        );
    }

    #[test]
    fn an_actors_weapon_is_announced_to_its_process() {
        let mut e = engine();
        tables(&mut e);
        let this = biped(&mut e);
        let form = object_with_slots(&mut e, &[(0x130, 0x7e7e)]);
        returns(&mut e, FORM_TYPE, 0x28);
        let process = object_with_slots(&mut e, &[(0x1cc, 0)]);
        let requester = requester_with(&mut e, this, 1);
        returns(&mut e, SAVED_ACQUIRE_OBJECT, process);
        returns(&mut e, FN_004AEED0, 0x9090);
        returns(&mut e, GET_BIPED, 0x1);
        returns(&mut e, ACTOR_ANIMATION, 0x3131);
        returns(&mut e, IS_WEAPON_DRAWN, 1);
        log(&mut e);
        fn_004ab750(&mut e, this, Ptr::new(form), 0);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, RELOAD_TARGETS), vec![vec![requester, 1]]);
        assert_eq!(calls_to(&calls, BLEND_OUT), vec![vec![0x3131, 4, 0]]);
        let target = slot_target(&e, process, 0x1cc);
        assert_eq!(
            calls_to(&calls, target),
            vec![vec![process, 1, this.addr(), 0x3131, requester]]
        );
        // The flags object blocks the announcement.
        returns(&mut e, FLAGS_OBJECT_FLAG, 1);
        set_slot(&mut e, this, 5, 0, 0, 0);
        fn_004ab750(&mut e, this, Ptr::new(form), 0);
        let calls = log(&mut e);
        assert!(calls_to(&calls, target).is_empty());
        assert!(calls_to(&calls, BLEND_OUT).is_empty());
    }

    /// Makes the dynamic cast answer `object` for the given objects only
    /// (`races`), and null otherwise.
    fn races_are(e: &mut Engine, races: Vec<u32>) {
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            ret(if races.contains(&a[0]) { a[0] } else { 0 })
        });
    }

    #[test]
    fn a_biped_model_form_takes_the_first_slot_it_fills_after_emptying_the_others() {
        let mut e = engine();
        let this = biped(&mut e);
        let form = 0x1111;
        let race = 0x2222;
        let other = 0x3333;
        set_slot(&mut e, this, 3, race, 0xa1, 0);
        set_slot(&mut e, this, 7, other, 0xa2, 0);
        set_slot(&mut e, this, 9, other, 0xa3, 0);
        set_slot(&mut e, this, 11, form, 0xa4, 0);
        returns(&mut e, GET_FORM_AS_BIPED_MODEL, 0x4800);
        races_are(&mut e, vec![race]);
        // The model fills slots 3, 7, 9 and 11 for the first question (any
        // form) and 7 and 9 for the second (the first of those is taken).
        e.register_double(FILLS_BIPED_SLOT, |_, a| {
            let slot = a[1];
            ret(match a[2] {
                0 => [3, 7, 9, 11].contains(&slot) as u32,
                _ => [7, 9].contains(&slot) as u32,
            })
        });
        let item = Ptr::new(0x9999);
        fn_004abad0(&mut e, this, Ptr::new(form), item, 0);
        // Slot 3 held a race: emptied by slot. Slots 7 and 9 held another
        // form: emptied by form. Slot 11 already held `form`: untouched
        // until it is overwritten below (it is not the first slot filled).
        assert_eq!(slot_words(&e, object(this, 3)), [0; 4]);
        assert_eq!(slot_words(&e, object(this, 9)), [0; 4]);
        assert_eq!(slot_words(&e, object(this, 7)), [form, 0x9999, 0, 0]);
        assert_eq!(slot_words(&e, object(this, 11)), [form, 0xa4, 0, 0]);
    }

    #[test]
    fn a_race_goes_into_the_given_slot() {
        let mut e = engine();
        let this = biped(&mut e);
        let race = 0x2222;
        set_slot(&mut e, this, 6, 0x5151, 0xa1, 0);
        races_are(&mut e, vec![race]);
        fn_004abad0(&mut e, this, Ptr::new(race), Ptr::new(0x9999), 6);
        assert_eq!(slot_words(&e, object(this, 6)), [race, 0x9999, 0, 0]);
        // Neither a biped model nor a race: nothing changes.
        fn_004abad0(&mut e, this, Ptr::new(0x4242), Ptr::new(0x7777), 8);
        assert_eq!(slot_words(&e, object(this, 8)), [0; 4]);
    }

    #[test]
    fn the_first_person_player_gets_the_slots_own_item() {
        let mut e = engine();
        let this = biped(&mut e);
        let race = 0x2222;
        races_are(&mut e, vec![race]);
        e.set_global(PLAYER, 0x1000);
        returns(&mut e, IS_FIRST_PERSON_BIPED, 1);
        returns(&mut e, FN_004AF950, 1);
        returns(&mut e, FN_004AFA20, 0x5a5a);
        log(&mut e);
        fn_004abad0(&mut e, this, Ptr::new(race), Ptr::new(0x9999), 6);
        assert_eq!(slot_words(&e, object(this, 6)), [race, 0x5a5a, 0, 0]);
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, IS_FIRST_PERSON_BIPED),
            vec![vec![0x1000, this.addr()]]
        );
        let asked = calls_to(&calls, FN_004AF950);
        assert_eq!((asked[0][0], asked[0][1]), (this.addr(), 0x9999));
        // The item `004af950` answered no for is kept, and a null item skips the question.
        returns(&mut e, FN_004AF950, 0);
        fn_004abad0(&mut e, this, Ptr::new(race), Ptr::new(0x9999), 6);
        assert_eq!(slot_words(&e, object(this, 6)), [race, 0x9999, 0, 0]);
        log(&mut e);
        fn_004abad0(&mut e, this, Ptr::new(race), Ptr::NULL, 6);
        assert!(calls_to(&log(&mut e), FN_004AF950).is_empty());
    }

    /// The setting object answers: a text pointer whose first byte is `byte`
    /// for `setting`.
    fn setting_is(e: &mut Engine, settings: Vec<(u32, u8)>) {
        let byte = |e: &mut Engine, value: u8| {
            let address = e.mem.alloc(8);
            e.mem.set_u8(address, value);
            address
        };
        let table: Vec<(u32, u32)> = settings.into_iter().map(|(s, v)| (s, byte(e, v))).collect();
        e.register_double(SETTING_BYTE_POINTER, move |_, a| {
            ret(table.iter().find(|(s, _)| *s == a[0]).unwrap().1)
        });
    }

    /// A requester whose base form is an NPC (the casts succeed).
    fn npc_requester(e: &mut Engine, this: Ptr<BipedAnim>) -> u32 {
        let requester = 0x0abc_0000;
        e.set(this, BipedAnim::m_pRequester, Ptr::new(requester));
        returns(e, REFERENCE_FORM, 0x0ccc_0000);
        races_are(e, vec![0x0ccc_0000]);
        requester
    }

    #[test]
    fn queueing_models_queues_each_part_and_the_biped_model_path() {
        let mut e = engine();
        e.set_global(MODEL_LOADER, 0x5555);
        let this = biped(&mut e);
        npc_requester(&mut e, this);
        setting_is(&mut e, vec![(SETTING_FACE, 0)]);
        let part = block(&mut e, 0x40);
        e.mem.set_u8(part + 0x14, 0x0e);
        set_slot(&mut e, this, 3, 0x1111, part, 0);
        returns(&mut e, GET_FORM_AS_BIPED_MODEL, 0x4800);
        let path = text(&mut e, "meshes\\x.nif");
        returns(&mut e, BIPED_MODEL_PATH, path);
        log(&mut e);
        fn_004abd30(&mut e, this, 11, Ptr::new(0x7a7a), 22);
        let calls = log(&mut e);
        let order = addresses(&calls);
        assert_eq!(order[0], QUEUE_OBJECT_INIT);
        assert_eq!(
            calls_to(&calls, QUEUE_MODEL_FORM),
            vec![vec![0x5555, part, 11, 22, 3, 1, 0, 0]]
        );
        assert_eq!(
            calls_to(&calls, QUEUE_MODEL),
            vec![vec![0x5555, path, 11, 22, 3, 1, 0, 0]]
        );
        // The mask of the part flags drives the skin textures: bits 2, 4 and 8.
        let slots: Vec<u32> = calls_to(&calls, SKIN_TEXTURE_PATH)
            .into_iter()
            .map(|a| a[2])
            .collect();
        assert_eq!(slots, vec![2, 4, 3]);
        let used = calls_to(&calls, QUEUE_OBJECT_USE);
        assert_eq!(used.len(), 1);
        assert_eq!(used[0][0], 0x7a7a);
        assert_eq!(*order.last().unwrap(), QUEUE_OBJECT_FREE);
    }

    #[test]
    fn queueing_models_hands_a_face_gen_slot_to_the_face_gen_queue() {
        let mut e = engine();
        e.set_global(MODEL_LOADER, 0x5555);
        let this = biped(&mut e);
        npc_requester(&mut e, this);
        setting_is(
            &mut e,
            vec![
                (SETTING_FACE, 1),
                (SETTING_PAIR_FIRST, 1),
                (SETTING_PAIR_SECOND, 1),
            ],
        );
        returns(&mut e, FN_004AE8A0, 1);
        returns(&mut e, SETTING_BYTE_VALUE, 1);
        let part = block(&mut e, 0x40);
        set_slot(&mut e, this, 3, 0x1111, part, 0);
        log(&mut e);
        fn_004abd30(&mut e, this, 11, Ptr::new(0x7a7a), 22);
        let calls = log(&mut e);
        let faces = calls_to(&calls, FN_004AEE60);
        assert_eq!(faces.len(), 1);
        assert_eq!(
            (faces[0][0], faces[0][1], faces[0][2], faces[0][4]),
            (this.addr(), 11, 22, 3)
        );
        assert!(calls_to(&calls, QUEUE_MODEL_FORM).is_empty());
        // One of the pair of settings empty: the part is queued normally.
        setting_is(
            &mut e,
            vec![
                (SETTING_FACE, 1),
                (SETTING_PAIR_FIRST, 1),
                (SETTING_PAIR_SECOND, 0),
            ],
        );
        fn_004abd30(&mut e, this, 11, Ptr::new(0x7a7a), 22);
        let calls = log(&mut e);
        assert!(calls_to(&calls, FN_004AEE60).is_empty());
        assert_eq!(calls_to(&calls, QUEUE_MODEL_FORM).len(), 1);
    }

    #[test]
    fn queueing_models_for_the_player_also_queues_the_alternative_part() {
        let mut e = engine();
        e.set_global(MODEL_LOADER, 0x5555);
        let this = biped(&mut e);
        let requester = npc_requester(&mut e, this);
        e.set_global(PLAYER, requester);
        setting_is(&mut e, vec![(SETTING_FACE, 0)]);
        let part = block(&mut e, 0x40);
        set_slot(&mut e, this, 3, 0x1111, part, 0);
        set_slot(&mut e, this, 4, 0x1112, 0xffff_ffff, 0);
        returns(&mut e, FN_004AFA50, 0x6b6b);
        log(&mut e);
        fn_004abd30(&mut e, this, 1, Ptr::new(0x7a7a), 2);
        let calls = log(&mut e);
        // The alternative first, then the part; the slot with part -1 is skipped.
        assert_eq!(
            calls_to(&calls, QUEUE_MODEL_FORM),
            vec![
                vec![0x5555, 0x6b6b, 1, 2, 3, 1, 0, 0],
                vec![0x5555, part, 1, 2, 3, 1, 0, 0]
            ]
        );
        assert_eq!(
            calls_to(&calls, FN_004AFA50),
            vec![vec![this.addr(), 3, part]]
        );
    }

    #[test]
    fn queueing_models_for_something_that_is_not_an_npc_only_tells_the_object() {
        let mut e = engine();
        let this = biped(&mut e);
        e.set(this, BipedAnim::m_pRequester, Ptr::new(0x0abc_0000));
        let part = block(&mut e, 0x40);
        set_slot(&mut e, this, 3, 0x1111, part, 0);
        log(&mut e);
        fn_004abd30(&mut e, this, 1, Ptr::new(0x7a7a), 2);
        let order = addresses(&log(&mut e));
        assert_eq!(
            order,
            vec![
                QUEUE_OBJECT_INIT,
                REFERENCE_FORM,
                RT_DYNAMIC_CAST,
                QUEUE_OBJECT_USE,
                QUEUE_OBJECT_FREE
            ]
        );
    }

    #[test]
    fn the_part_flags_are_bits_one_to_three() {
        let mut e = engine();
        let item = block(&mut e, 0x40);
        e.mem.set_u8(item + 0x14, 0xff);
        assert_eq!(fn_004abf80(&mut e, Ptr::new(item)), 0x0e);
        e.mem.set_u8(item + 0x14, 0x01);
        assert_eq!(fn_004abf80(&mut e, Ptr::new(item)), 0);
    }

    #[test]
    fn the_pair_of_settings_must_both_be_set() {
        let mut e = engine();
        setting_is(
            &mut e,
            vec![(SETTING_PAIR_FIRST, 1), (SETTING_PAIR_SECOND, 1)],
        );
        assert!(fn_004abfa0(&mut e));
        setting_is(
            &mut e,
            vec![(SETTING_PAIR_FIRST, 1), (SETTING_PAIR_SECOND, 0)],
        );
        assert!(!fn_004abfa0(&mut e));
        setting_is(
            &mut e,
            vec![(SETTING_PAIR_FIRST, 0), (SETTING_PAIR_SECOND, 1)],
        );
        assert!(!fn_004abfa0(&mut e));
    }

    #[test]
    fn skin_textures_are_queued_for_the_masked_slots_and_the_race_texture() {
        let mut e = engine();
        e.set_global(MODEL_LOADER, 0x5555);
        let this = biped(&mut e);
        npc_requester(&mut e, this);
        returns(&mut e, READ_WORD_PLUS_4, 0x7ace);
        returns(&mut e, BODY_MOD_TEXTURE_FILE_NAME, 1);
        log(&mut e);
        fn_004abfe0(&mut e, this, 0, 5, 6);
        assert!(log(&mut e).is_empty());
        fn_004abfe0(&mut e, this, 8, 5, 6);
        let calls = log(&mut e);
        let skins = calls_to(&calls, SKIN_TEXTURE_PATH);
        assert_eq!(skins.len(), 1);
        assert_eq!(
            (skins[0][0], skins[0][1], skins[0][2]),
            (0x7ace, 0x0ccc_0000, 3)
        );
        let names = calls_to(&calls, BODY_MOD_TEXTURE_NAME);
        let files = calls_to(&calls, BODY_MOD_TEXTURE_FILE_NAME);
        assert_eq!((names[0][0], names[0][1]), (0x7ace, 0x0ccc_0000));
        assert_eq!(files[0][..3], [0x7ace, 0x0ccc_0000, names[0][2]]);
        // The race texture file is queued with the loader and the two arguments.
        let queued = calls_to(&calls, QUEUE_TEXTURE);
        assert_eq!(queued.len(), 1);
        assert_eq!((queued[0][0], queued[0][2], queued[0][3]), (0x5555, 5, 6));
        assert_eq!(queued[0][1], files[0][3]);
    }

    #[test]
    fn skin_textures_need_an_npc() {
        let mut e = engine();
        let this = biped(&mut e);
        e.set(this, BipedAnim::m_pRequester, Ptr::new(0x0abc_0000));
        log(&mut e);
        fn_004abfe0(&mut e, this, 2, 5, 6);
        assert!(calls_to(&log(&mut e), SKIN_TEXTURE_PATH).is_empty());
    }

    #[test]
    fn the_race_of_an_npc_is_read_through_its_inner_object() {
        let mut e = engine();
        returns(&mut e, READ_WORD_PLUS_4, 0x7ace);
        assert_eq!(fn_004ac110(&mut e, Ptr::new(0x1000)), 0x7ace);
        assert_eq!(log(&mut e), vec![(READ_WORD_PLUS_4, vec![0x110c])]);
    }

    #[test]
    fn a_skin_texture_is_queued_with_its_normal_map() {
        let mut e = engine();
        e.set_global(MODEL_LOADER, 0x5555);
        let this = biped(&mut e);
        returns(&mut e, READ_WORD_PLUS_4, 0x7ace);
        // No texture for the race: nothing is queued.
        biped_anim_queue_skin_texture(&mut e, this, Ptr::new(0x1000), 4, 8, 9);
        assert!(calls_to(&log(&mut e), QUEUE_TEXTURE).is_empty());
        // A texture and a normal map: both are queued.
        returns(&mut e, SKIN_TEXTURE_PATH, 1);
        e.register_double(MODIFIED_TEXTURE_FILENAME, |e, a| {
            e.mem.set_u8(a[0], b'n');
            ret(a[0])
        });
        biped_anim_queue_skin_texture(&mut e, this, Ptr::new(0x1000), 4, 8, 9);
        let calls = log(&mut e);
        let skin = calls_to(&calls, SKIN_TEXTURE_PATH);
        assert_eq!(skin[0][..3], [0x7ace, 0x1000, 4]);
        let queued = calls_to(&calls, QUEUE_TEXTURE);
        assert_eq!(queued.len(), 2);
        assert_eq!(queued[0], vec![0x5555, skin[0][3], 8, 9]);
        let modified = calls_to(&calls, MODIFIED_TEXTURE_FILENAME);
        assert_eq!(modified[0][1..], [skin[0][3], NORMAL_MAP_SUFFIX, 1]);
        assert_eq!(queued[1], vec![0x5555, modified[0][0], 8, 9]);
        // The normal map is left out when its path is empty.
        returns(&mut e, MODIFIED_TEXTURE_FILENAME, 0);
        biped_anim_queue_skin_texture(&mut e, this, Ptr::new(0x1000), 4, 8, 9);
        assert_eq!(calls_to(&log(&mut e), QUEUE_TEXTURE).len(), 1);
    }

    /// A biped for `load_biped_parts` with one loadable part in `slot` (its
    /// bone index in the table is `bone_index`): the model, the form with a
    /// name, the loaded file and its clone's node.
    struct Scene {
        this: Ptr<BipedAnim>,
        node: u32,
        form: u32,
        model: u32,
    }

    fn scene(e: &mut Engine, slot: u32, bone_index: u32) -> Scene {
        tables(e);
        e.mem.set_u32(SLOT_BONES + 4 * slot, bone_index);
        e.set_global(MODEL_LOADER, 0x5555);
        let this = biped(e);
        e.set(this, BipedAnim::m_pRequester, Ptr::new(0x0abc_0000));
        e.set(this, BipedAnim::root, Ptr::new(0x1818));
        let model = object_with_slots(e, &[(0x14, 0xabc0), (0x1c, 0)]);
        let form = object_with_slots(e, &[(0xac, 0), (0x130, 0x7e7e)]);
        set_slot(e, this, slot, form, model, 0);
        let node = object_with_slots(e, &[(0x0c, 1), (0x10, 0)]);
        let copy = object_with_slots(e, &[(0x0c, node)]);
        returns(e, MODEL_HAS_PATH, 1);
        returns(e, LOAD_FILE, 0xf11e);
        returns(e, NI_OBJECT_CLONE, copy);
        returns(e, FORM_ID, 0x4321);
        Scene {
            this,
            node,
            form,
            model,
        }
    }

    #[test]
    fn a_part_that_did_not_change_takes_the_buffered_clone_over() {
        let mut e = engine();
        let this = biped(&mut e);
        e.set(this, BipedAnim::m_pRequester, Ptr::new(0x0abc_0000));
        set_slot(&mut e, this, 2, 0x1111, 0x2222, 0);
        e.set(buffered(this, 2), BipedObject::pParent, Ptr::new(0x1111));
        e.set(buffered(this, 2), BipedObject::pPart, Ptr::new(0x2222));
        e.set(buffered(this, 2), BipedObject::pPartClone, Ptr::new(0x3333));
        returns(&mut e, ACTOR_ROOT, 0x1234);
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, this, 0);
        assert_eq!(slot_words(&e, object(this, 2)), [0x1111, 0x2222, 0x3333, 0]);
        assert_eq!(slot_words(&e, buffered(this, 2)), [0; 4]);
        let calls = log(&mut e);
        assert!(calls_to(&calls, LOAD_FILE).is_empty());
        assert_eq!(calls_to(&calls, ACTOR_ROOT), vec![vec![0x0abc_0000]]);
        let last = calls.last().unwrap();
        assert_eq!(last, &(UPDATE_PROPERTIES, vec![0x1234]));
    }

    #[test]
    fn a_reused_part_adjusts_its_skin_complexion_when_the_flag_is_set() {
        let mut e = engine();
        let this = biped(&mut e);
        e.set(this, BipedAnim::m_pRequester, Ptr::new(0x0abc_0000));
        e.set_global(PALETTE_FLAG, 1u8);
        returns(&mut e, REFERENCE_FORM, 0x77);
        let list = 0x6600_0000;
        let node = object_with_slots(&mut e, &[(0x0c, list)]);
        set_slot(&mut e, this, 2, 0x1111, 0x2222, 0);
        e.set(buffered(this, 2), BipedObject::pParent, Ptr::new(0x1111));
        e.set(buffered(this, 2), BipedObject::pPart, Ptr::new(0x2222));
        e.set(buffered(this, 2), BipedObject::pPartClone, Ptr::new(node));
        returns(&mut e, CHILD_COUNT, 2);
        e.register_double(CHILD_AT, |_, a| ret(0x8000 + a[1]));
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, this, 0);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, CLEAR_BODY_PALETTE), vec![vec![0x77]]);
        assert_eq!(
            calls_to(&calls, FN_004AF490),
            vec![vec![this.addr(), 0x8000, 0], vec![this.addr(), 0x8001, 0]]
        );
        // A clone that is not a node is adjusted itself.
        let leaf = object_with_slots(&mut e, &[(0x0c, 0)]);
        set_slot(&mut e, this, 2, 0x1111, 0x2222, 0);
        set_slot(&mut e, this, 0, 0, 0, 0);
        e.set(buffered(this, 2), BipedObject::pParent, Ptr::new(0x1111));
        e.set(buffered(this, 2), BipedObject::pPart, Ptr::new(0x2222));
        e.set(buffered(this, 2), BipedObject::pPartClone, Ptr::new(leaf));
        biped_anim_load_biped_parts(&mut e, this, 0);
        assert_eq!(
            calls_to(&log(&mut e), FN_004AF490),
            vec![vec![this.addr(), leaf, 0]]
        );
    }

    #[test]
    fn a_reused_back_slot_keeps_the_weapon_flag_of_the_weapon() {
        let mut e = engine();
        let this = biped(&mut e);
        e.set(this, BipedAnim::m_pRequester, Ptr::new(0x0abc_0000));
        let weapon = block(&mut e, 0x300);
        e.mem.set_u8(weapon + 0x100, 0x10);
        // Slot 5 holds the weapon form; slot 7 is reused with its clone.
        e.set(object(this, 5), BipedObject::pParent, Ptr::new(weapon));
        set_slot(&mut e, this, 7, 0x1111, 0x2222, 0);
        e.set(buffered(this, 7), BipedObject::pParent, Ptr::new(0x1111));
        e.set(buffered(this, 7), BipedObject::pPart, Ptr::new(0x2222));
        e.set(buffered(this, 7), BipedObject::pPartClone, Ptr::new(0x3333));
        // (Slot 5 itself would be loaded through 004ab750: make it a non-weapon.)
        returns(&mut e, FORM_TYPE, 0x2a);
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, this, 0);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, SET_WEAPON_FLAG), vec![vec![0x3333, 1]]);
    }

    #[test]
    fn a_changed_slot_lets_go_of_its_buffered_part() {
        let mut e = engine();
        let this = biped(&mut e);
        e.set(this, BipedAnim::m_pRequester, Ptr::new(0x0abc_0000));
        let model = object_with_slots(&mut e, &[(0x14, 0xabc0)]);
        e.set(buffered(this, 4), BipedObject::pParent, Ptr::new(0x1111));
        e.set(buffered(this, 4), BipedObject::pPart, Ptr::new(model));
        e.set(buffered(this, 4), BipedObject::pPartClone, Ptr::new(0x3333));
        e.set_global(MODEL_LOADER, 0x5555);
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, this, 0);
        let calls = log(&mut e);
        assert_eq!(slot_words(&e, buffered(this, 4)), [0; 4]);
        assert_eq!(calls_to(&calls, PREPARE_DETACH), vec![vec![0x3333]]);
        assert_eq!(calls_to(&calls, RELEASE_MODEL), vec![vec![0x5555, 0xabc0]]);
    }

    #[test]
    fn a_new_part_is_loaded_cloned_named_and_attached_to_its_bone() {
        let mut e = engine();
        let s = scene(&mut e, 8, 4);
        let bone_node = object_with_slots(&mut e, &[(0xdc, 0)]);
        e.set(bone(s.this, 4), BipedBone::pParent, Ptr::new(bone_node));
        returns(&mut e, FIXED_STRING_INIT, 0x1f1f);
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, s.this, 0);
        assert_eq!(
            e.get(object(s.this, 8), BipedObject::pPartClone).addr(),
            s.node
        );
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, LOAD_FILE),
            vec![vec![0x5555, 0xabc0, 3, 1, 0, 0, 0]]
        );
        // Cloned with a fresh cloning process (no morpher controller).
        let cloned = calls_to(&calls, NI_OBJECT_CLONE);
        assert_eq!(cloned[0][0], 0xf11e);
        // The clone's node is reset to the origin and identity, then attached
        // to the root by 004ae250 (not skinned) and to its bone.
        assert_eq!(
            calls_to(&calls, SET_TRANSLATION),
            vec![vec![s.node, ZERO_TRANSLATION]]
        );
        assert_eq!(
            calls_to(&calls, SET_ROTATION),
            vec![vec![s.node, IDENTITY_ROTATION]]
        );
        assert_eq!(
            calls_to(&calls, FN_004AE250),
            vec![vec![0x1818, s.node, 0, s.this.addr(), 8, 0]]
        );
        let slot_name = e.mem.u32(SLOT_NAMES + 4 * 8);
        let sprintf = calls_to(&calls, SPRINTF);
        assert_eq!(
            sprintf[0][1..],
            [SLOT_NAME_FORMAT, slot_name, 0x7e7e, 0x4321]
        );
        assert_eq!(calls_to(&calls, SET_NAME), vec![vec![s.node, 0x1f1f]]);
        assert_eq!(
            calls_to(&calls, FN_004AF240),
            vec![vec![0xf11e, s.node, 8, 0x0abc_0000]]
        );
        let attach = slot_target(&e, bone_node, 0xdc);
        assert_eq!(calls_to(&calls, attach), vec![vec![bone_node, s.node, 1]]);
        // The cloning process is destroyed (its maps were never made).
        assert!(calls_to(&calls, SWAP_TEXTURES).is_empty());
        let _ = (s.form, s.model);
    }

    #[test]
    fn a_morphing_model_is_deep_copied_with_the_same_textures() {
        let mut e = engine();
        let s = scene(&mut e, 8, 4);
        returns(&mut e, HAS_MORPHER_CONTROLLER, 1);
        let copy = object_with_slots(&mut e, &[(0x0c, s.node)]);
        returns(&mut e, DEEP_COPY_SAME_TEXTURES, copy);
        e.set_global(TES_GLOBAL, 0x7e57);
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, s.this, 0);
        let calls = log(&mut e);
        let deep = calls_to(&calls, DEEP_COPY_SAME_TEXTURES);
        assert_eq!((deep[0][0], deep[0][1]), (0x7e57, 0xf11e));
        assert!(calls_to(&calls, NI_OBJECT_CLONE).is_empty());
        assert_eq!(
            e.get(object(s.this, 8), BipedObject::pPartClone).addr(),
            s.node
        );
    }

    #[test]
    fn textures_are_swapped_when_the_model_or_the_form_ask() {
        let mut e = engine();
        let s = scene(&mut e, 8, 4);
        // The model has a texture swap (virtual slot 0x1c) and the form is a
        // platform-language form (virtual slot 0xac).
        let swap_model = object_with_slots(&mut e, &[(0x14, 0xabc0), (0x1c, 0x5757)]);
        let swap_form = object_with_slots(&mut e, &[(0xac, 1), (0x130, 0x7e7e)]);
        e.set(object(s.this, 8), BipedObject::pPart, Ptr::new(swap_model));
        e.set(object(s.this, 8), BipedObject::pParent, Ptr::new(swap_form));
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, s.this, 0);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, SWAP_TEXTURES), vec![vec![0x5757, s.node]]);
        assert_eq!(calls_to(&calls, SWAP_PLATFORM_TEXTURES), vec![vec![s.node]]);
    }

    #[test]
    fn a_part_without_a_bone_that_is_not_skinned_is_logged_and_dropped() {
        let mut e = engine();
        let s = scene(&mut e, 3, 0xffff_ffff);
        returns(&mut e, ACTOR_LOG_NAME, 0x1919);
        let parent = object_with_slots(&mut e, &[(0xe8, 0)]);
        returns(&mut e, NODE_PARENT, parent);
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, s.this, 0);
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, LOG),
            vec![vec![SHOULD_BE_SKINNED_FORMAT, 0xabc0, 0x1919]]
        );
        let target = slot_target(&e, parent, 0xe8);
        assert_eq!(calls_to(&calls, target), vec![vec![parent, s.node]]);
        assert!(e.get(object(s.this, 3), BipedObject::pPartClone).is_null());
    }

    #[test]
    fn a_part_whose_bone_is_missing_goes_under_the_root() {
        let mut e = engine();
        let s = scene(&mut e, 8, 4);
        let root = object_with_slots(&mut e, &[(0xdc, 0)]);
        e.set(s.this, BipedAnim::root, Ptr::new(root));
        returns(&mut e, NODE_PARENT, 0x6767);
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, s.this, 0);
        let calls = log(&mut e);
        let target = slot_target(&e, root, 0xdc);
        assert_eq!(calls_to(&calls, target), vec![vec![root, s.node, 1]]);
        assert_eq!(
            e.get(object(s.this, 8), BipedObject::pPartClone).addr(),
            s.node
        );
    }

    #[test]
    fn dismembered_limbs_are_hidden_on_the_loaded_part() {
        let mut e = engine();
        let s = scene(&mut e, 8, 4);
        returns(&mut e, GET_DISMEMBERMENT_EXTRA, 0x4545);
        e.register_double(
            LIMB_DISMEMBERED,
            |_, a| ret((a[1] == 2 || a[1] == 9) as u32),
        );
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, s.this, 0);
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, HIDE_LIMB),
            vec![vec![2, s.node], vec![9, s.node]]
        );
        assert_eq!(calls_to(&calls, LIMB_DISMEMBERED).len(), 15);
    }

    #[test]
    fn the_back_slot_part_gets_the_weapon_flag_when_the_weapon_has_it() {
        let mut e = engine();
        let s = scene(&mut e, 7, 3);
        let weapon = block(&mut e, 0x300);
        e.mem.set_u8(weapon + 0x100, 0x10);
        // Slot 5 holds a weapon that is reused (non-weapon type, no clone).
        e.set(object(s.this, 5), BipedObject::pParent, Ptr::new(weapon));
        returns(&mut e, FORM_TYPE, 0x2a);
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, s.this, 0);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, SET_WEAPON_FLAG), vec![vec![s.node, 1]]);
    }

    #[test]
    fn a_face_gen_slot_may_be_deferred_unless_the_flag_is_set() {
        let mut e = engine();
        let s = scene(&mut e, 3, 0xffff_ffff);
        setting_is(
            &mut e,
            vec![
                (SETTING_FACE, 1),
                (SETTING_PAIR_FIRST, 1),
                (SETTING_PAIR_SECOND, 1),
            ],
        );
        returns(&mut e, FN_004AE8A0, 1);
        returns(&mut e, SETTING_BYTE_VALUE, 1);
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, s.this, 0);
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, FN_004AEDE0),
            vec![vec![s.this.addr(), 0, 0, 3]]
        );
        assert!(calls_to(&calls, LOAD_FILE).is_empty());
        // With the flag set the part is loaded (as a FaceGen slot).
        biped_anim_load_biped_parts(&mut e, s.this, 1);
        let calls = log(&mut e);
        assert!(calls_to(&calls, FN_004AEDE0).is_empty());
        assert_eq!(calls_to(&calls, LOAD_FILE)[0][5], 1);
    }

    #[test]
    fn the_first_person_player_loads_only_the_slots_two_to_six() {
        let mut e = engine();
        let s = scene(&mut e, 3, 0xffff_ffff);
        let requester = 0x1000;
        e.set_global(PLAYER, requester);
        e.set(s.this, BipedAnim::m_pRequester, Ptr::new(requester));
        returns(&mut e, IS_FIRST_PERSON_BIPED, 1);
        // Slots 1 and 8 also hold loadable parts; only slot 3 is loaded.
        let model = object_with_slots(&mut e, &[(0x14, 0xabc0), (0x1c, 0)]);
        let form = object_with_slots(&mut e, &[(0xac, 0), (0x130, 0x7e7e)]);
        set_slot(&mut e, s.this, 1, form, model, 0);
        set_slot(&mut e, s.this, 8, form, model, 0);
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, s.this, 0);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, LOAD_FILE).len(), 1);
    }

    #[test]
    fn the_players_weapon_slot_goes_through_the_weapon_loader() {
        let mut e = engine();
        tables(&mut e);
        let this = biped(&mut e);
        let requester = 0x1000;
        e.set_global(PLAYER, requester);
        e.set(this, BipedAnim::m_pRequester, Ptr::new(requester));
        let form = block(&mut e, 0x300);
        e.mem.set_u8(form + 0x100, 0x04);
        e.mem.set_u32(form + 0x12c, 0x2000);
        e.set(object(this, 5), BipedObject::pParent, Ptr::new(form));
        let item_change = 0x1c1c;
        let process = object_with_slots(&mut e, &[(0x148, item_change)]);
        returns(&mut e, SAVED_ACQUIRE_OBJECT, process);
        returns(&mut e, GET_MOD_SLOTS, 3);
        returns(&mut e, HAS_MOD_EFFECT_ACTIVE, 1);
        returns(&mut e, GET_BIPED, this.addr());
        returns(&mut e, WEAPON_MODEL_VALUE, 0x6d6d);
        // 004ab750 ignores a form that is not a weapon.
        returns(&mut e, FORM_TYPE, 0x2a);
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, this, 0);
        let calls = log(&mut e);
        let effect = calls_to(&calls, HAS_MOD_EFFECT_ACTIVE);
        assert_eq!((effect[0][0], effect[0][1]), (item_change, 0x0e));
        assert_eq!(calls_to(&calls, GET_MOD_SLOTS), vec![vec![item_change]]);
        assert_eq!(calls_to(&calls, FORM_TYPE), vec![vec![form]]);
        // The modded weapon with the flag shows the weapon model value.
        assert_eq!(calls_to(&calls, SHOW_WEAPON), vec![vec![0x6d6d]]);
        // Without the mod effect and with the flag word bit set it does not.
        returns(&mut e, HAS_MOD_EFFECT_ACTIVE, 0);
        biped_anim_load_biped_parts(&mut e, this, 0);
        assert!(calls_to(&log(&mut e), SHOW_WEAPON).is_empty());
        // Without the flag word bit it does.
        e.mem.set_u32(form + 0x12c, 0);
        biped_anim_load_biped_parts(&mut e, this, 0);
        assert_eq!(calls_to(&log(&mut e), SHOW_WEAPON).len(), 1);
    }

    #[test]
    fn another_actors_weapon_slot_sets_the_weapon_directly() {
        let mut e = engine();
        tables(&mut e);
        let this = biped(&mut e);
        e.set(this, BipedAnim::m_pRequester, Ptr::new(0x0abc_0000));
        e.set(object(this, 5), BipedObject::pParent, Ptr::new(0x4141));
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, this, 0);
        assert_eq!(
            calls_to(&log(&mut e), ACTOR_SET_WEAPON),
            vec![vec![0x0abc_0000, 0x4141]]
        );
    }

    #[test]
    fn a_face_gen_part_gets_its_coordinates_and_a_quarter_turn() {
        let mut e = engine();
        let s = scene(&mut e, 3, 0xffff_ffff);
        let npc = 0x0ccc_0000;
        returns(&mut e, REFERENCE_FORM, npc);
        returns(&mut e, FORM_TYPE, 0x2a);
        returns(&mut e, FN_004AE8A0, 1);
        returns(&mut e, FN_004AE790, 0x4141);
        e.mem.set_f32(QUARTER_TURN_BACK, -1.5707964);
        setting_is(&mut e, vec![(SETTING_FACE, 0)]);
        // The clone's node has one child, whose geometry has skin data.
        let geometry = object_with_slots(&mut e, &[(0xe4, 0)]);
        let child = object_with_slots(&mut e, &[(0x1c, geometry)]);
        returns(&mut e, CHILD_COUNT, 1);
        returns(&mut e, CHILD_AT, child);
        returns(&mut e, NODE_SKIN_DATA, 0x5a5a);
        returns(&mut e, NODE_ROTATION, 0x3000);
        returns(&mut e, MATRIX_MULTIPLY, 0x3100);
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, s.this, 0);
        let calls = log(&mut e);
        let coords = calls_to(&calls, GET_FACE_COORD);
        assert_eq!(coords[0][0], npc);
        let construct = calls_to(&calls, VECTOR_CONSTRUCT);
        assert_eq!(
            construct[0][..],
            [coords[0][1], 0x20, 4, FACE_COORD_INIT, FACE_COORD_FREE]
        );
        assert_eq!(
            calls_to(&calls, VECTOR_DESTRUCT),
            vec![vec![coords[0][1], 0x20, 4, FACE_COORD_FREE]]
        );
        // The setting is off, so the rotation is turned by -pi/2 about Y.
        let turn = calls_to(&calls, MAKE_Y_ROTATION);
        assert_eq!(f32::from_bits(turn[0][1]), -1.5707964);
        let multiply = calls_to(&calls, MATRIX_MULTIPLY);
        assert_eq!(multiply[0][0], 0x3000);
        assert_eq!(multiply[0][2], turn[0][0]);
        let set = calls_to(&calls, SET_ROTATION);
        assert!(set.contains(&vec![geometry, 0x3100]));
        // The skin data was copied and set on the geometry.
        assert_eq!(calls_to(&calls, NI_OBJECT_DEEP_COPY)[0][0], 0x5a5a);
        let target = slot_target(&e, geometry, 0xe4);
        assert_eq!(calls_to(&calls, target).len(), 1);
    }

    #[test]
    fn a_child_that_is_not_a_geometry_is_logged_when_loading_face_gen() {
        let mut e = engine();
        let s = scene(&mut e, 3, 0xffff_ffff);
        returns(&mut e, FN_004AE8A0, 1);
        returns(&mut e, FN_004AE790, 0x4141);
        setting_is(&mut e, vec![(SETTING_FACE, 0)]);
        let child = object_with_slots(&mut e, &[(0x1c, 0)]);
        returns(&mut e, CHILD_COUNT, 1);
        returns(&mut e, CHILD_AT, child);
        log(&mut e);
        biped_anim_load_biped_parts(&mut e, s.this, 0);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, LOG)[0], vec![NON_GEOMETRY_FORMAT, 0xabc0]);
    }

    #[test]
    fn the_cloning_process_flag_getters_read_their_bits() {
        let mut e = engine();
        let form = block(&mut e, 0x300);
        assert!(!fn_004ad010(&mut e, Ptr::new(form)));
        assert!(!fn_004ad030(&mut e, Ptr::new(form)));
        e.mem.set_u8(form + 0x100, 0x04);
        e.mem.set_u32(form + 0x12c, 0x2000);
        assert!(fn_004ad010(&mut e, Ptr::new(form)));
        assert!(fn_004ad030(&mut e, Ptr::new(form)));
        e.mem.set_u8(form + 0x100, 0xfb);
        e.mem.set_u32(form + 0x12c, 0xffff_dfff);
        assert!(!fn_004ad010(&mut e, Ptr::new(form)));
        assert!(!fn_004ad030(&mut e, Ptr::new(form)));
        e.set_global(CLONE_COPY_TYPE, 0x0102);
        e.set_global(CLONE_APPEND_CHAR, 0x2au8);
        assert_eq!(fn_004ad1b0(&mut e), 0x0102);
        assert_eq!(fn_004ad1c0(&mut e), 0x2a);
    }

    #[test]
    fn a_cloning_process_builds_its_two_maps() {
        let mut e = engine();
        e.set_global(CLONE_COPY_TYPE, 7);
        e.set_global(CLONE_APPEND_CHAR, 0x2au8);
        let next = Rc::new(RefCell::new(0x6000u32));
        let counter = next.clone();
        e.register_double(OPERATOR_NEW, move |_, _| {
            let mut value = counter.borrow_mut();
            let result = *value;
            *value += 0x100;
            ret(result)
        });
        e.register(FN_004AFB20, |_, a| ret(a[0]));
        e.register(FN_004AFB50, |_, a| ret(a[0]));
        let this = block(&mut e, 0x1c);
        log(&mut e);
        let result = fn_004ad0c0(&mut e, Ptr::new(this), 0x101);
        assert_eq!(result.addr(), this);
        let process = Ptr::<NiCloningProcess>::new(this);
        assert_eq!(
            e.get(process, NiCloningProcess::m_pkCloneMap).addr(),
            0x6000
        );
        assert_eq!(
            e.get(process, NiCloningProcess::m_pkProcessMap).addr(),
            0x6100
        );
        assert_eq!(e.get(process, NiCloningProcess::m_eCopyType), 7);
        assert_eq!(e.get(process, NiCloningProcess::m_cAppendChar), 0x2a);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, OPERATOR_NEW), vec![vec![0x10], vec![0x10]]);
        assert_eq!(calls_to(&calls, FN_004AFB20), vec![vec![0x6000, 0x101]]);
        assert_eq!(calls_to(&calls, FN_004AFB50), vec![vec![0x6100, 0x101]]);
        // When the allocation fails the maps are null and nothing is built.
        returns(&mut e, OPERATOR_NEW, 0);
        fn_004ad0c0(&mut e, Ptr::new(this), 0x101);
        assert!(e.get(process, NiCloningProcess::m_pkCloneMap).is_null());
        assert!(e.get(process, NiCloningProcess::m_pkProcessMap).is_null());
        assert!(calls_to(&log(&mut e), FN_004AFB20).is_empty());
    }

    #[test]
    fn the_scaled_cloning_process_sets_the_scale_vector() {
        let mut e = engine();
        let this = block(&mut e, 0x1c);
        log(&mut e);
        let result = fn_004ad050(&mut e, Ptr::new(this), 2.5);
        assert_eq!(result.addr(), this);
        let process = Ptr::<NiCloningProcess>::new(this);
        assert_eq!(
            [
                e.get(process, NiCloningProcess::m_fScale_x),
                e.get(process, NiCloningProcess::m_fScale_y),
                e.get(process, NiCloningProcess::m_fScale_z)
            ],
            [2.5; 3]
        );
        let calls = log(&mut e);
        // Hash size 0x101 for the maps, the vector at +0x10 initialized.
        assert_eq!(calls_to(&calls, MATRIX_INIT), vec![vec![this + 0x10]]);
        assert_eq!(calls_to(&calls, OPERATOR_NEW).len(), 2);
        // The same through the uniform form (the float is one word).
        e.call(0x004a_d240, &args![this, 1.5f32]);
        assert_eq!(e.get(process, NiCloningProcess::m_fScale_y), 1.5);
    }

    #[test]
    fn destroying_a_cloning_process_deletes_its_maps() {
        let mut e = engine();
        let clone_map = object_with_slots(&mut e, &[(0, 0)]);
        let process_map = object_with_slots(&mut e, &[(0, 0)]);
        let this = block(&mut e, 0x1c);
        let process = Ptr::<NiCloningProcess>::new(this);
        e.set(process, NiCloningProcess::m_pkCloneMap, Ptr::new(clone_map));
        e.set(
            process,
            NiCloningProcess::m_pkProcessMap,
            Ptr::new(process_map),
        );
        log(&mut e);
        ni_cloning_process_destructor(&mut e, process);
        let calls = log(&mut e);
        let first = slot_target(&e, clone_map, 0);
        let second = slot_target(&e, process_map, 0);
        assert_eq!(
            calls,
            vec![(first, vec![clone_map, 1]), (second, vec![process_map, 1])]
        );
        // Null maps are left alone.
        e.set(process, NiCloningProcess::m_pkCloneMap, Ptr::NULL);
        e.set(process, NiCloningProcess::m_pkProcessMap, Ptr::NULL);
        fn_004ad1d0(&mut e, process);
        assert!(log(&mut e).is_empty());
    }

    #[test]
    fn nothing_is_attached_without_a_node() {
        let mut e = engine();
        let s = scene(&mut e, 3, 0xffff_ffff);
        returns(&mut e, CAST_TO_CLASS, 0);
        log(&mut e);
        let result =
            biped_anim_apply_skinned_objects(&mut e, s.this, Ptr::NULL, 3, 0, 0, Ptr::NULL);
        assert!(result.is_null());
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, CAST_TO_CLASS),
            vec![vec![SKINNED_CLASS, 0]]
        );
        assert!(calls_to(&calls, FIND_SKINNED_NODE).is_empty());
    }

    #[test]
    fn a_biped_without_a_root_keeps_the_node_and_records_the_skin_flag() {
        let mut e = engine();
        let s = scene(&mut e, 3, 0xffff_ffff);
        e.set(s.this, BipedAnim::root, Ptr::NULL);
        returns(&mut e, FIND_SKINNED_NODE, 1);
        let result =
            biped_anim_apply_skinned_objects(&mut e, s.this, Ptr::new(s.node), 3, 0, 0, Ptr::NULL);
        assert_eq!(result.addr(), s.node);
        assert!(e.get(object(s.this, 3), BipedObject::bSkinned));
    }

    #[test]
    fn a_node_without_skinned_objects_attaches_nothing() {
        let mut e = engine();
        let s = scene(&mut e, 3, 0xffff_ffff);
        e.mem.set_u8(object(s.this, 3).addr() + 0xc, 1);
        log(&mut e);
        let result =
            biped_anim_apply_skinned_objects(&mut e, s.this, Ptr::new(s.node), 3, 0, 0, Ptr::NULL);
        assert!(result.is_null());
        assert!(!e.get(object(s.this, 3), BipedObject::bSkinned));
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, FIND_SKINNED_NODE), vec![vec![s.node]]);
        assert!(calls_to(&calls, FN_004ADD70).is_empty());
    }

    #[test]
    fn skinned_objects_are_attached_under_the_skin_attachment_node() {
        let mut e = engine();
        let s = scene(&mut e, 3, 0xffff_ffff);
        returns(&mut e, FIND_SKINNED_NODE, 1);
        let attachment = 0x4a4a;
        returns(&mut e, FIND_OBJECT_BY_NAME, attachment);
        log(&mut e);
        // The node is not of the skinned class: it is attached as a whole.
        // The attached node has too few children to make a node, so nothing
        // comes back, and the node is referenced and released once.
        let result = biped_anim_apply_skinned_objects(
            &mut e,
            s.this,
            Ptr::new(s.node),
            3,
            1,
            0,
            Ptr::new(0x66),
        );
        assert!(result.is_null());
        assert!(e.get(object(s.this, 3), BipedObject::bSkinned));
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, FIND_OBJECT_BY_NAME),
            vec![vec![0x1818, SKIN_ATTACHMENT_NAME, 1]]
        );
        assert_eq!(
            calls_to(&calls, FN_004ADD70),
            vec![vec![s.node], vec![s.node]]
        );
        assert_eq!(
            calls_to(&calls, SET_TRANSLATION),
            vec![vec![s.node, ZERO_TRANSLATION]]
        );
        assert_eq!(calls_to(&calls, GET_EXTRA_DATA).len(), 1);
        let refs = calls_to(&calls, NI_POINTER_INIT);
        assert_eq!(refs.last().unwrap()[1], s.node);
    }

    #[test]
    fn without_a_skin_attachment_node_the_roots_parent_is_used() {
        let mut e = engine();
        let s = scene(&mut e, 3, 0xffff_ffff);
        returns(&mut e, FIND_SKINNED_NODE, 1);
        returns(&mut e, FIND_OBJECT_BY_NAME, 0);
        returns(&mut e, NODE_PARENT, 0x5b5b);
        log(&mut e);
        biped_anim_apply_skinned_objects(&mut e, s.this, Ptr::new(s.node), 3, 0, 0, Ptr::NULL);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, NODE_PARENT), vec![vec![0x1818]]);
        assert_eq!(calls_to(&calls, NI_POINTER_SET).len(), 1);
        assert_eq!(calls_to(&calls, NI_POINTER_SET)[0][1], 0x5b5b);
    }

    /// Makes the node children come from `children`.
    fn children_are(e: &mut Engine, children: Vec<u32>) {
        returns(e, CHILD_COUNT, children.len() as u32);
        e.register_double(CHILD_AT, move |_, a| {
            ret(children.get(a[1] as usize).copied().unwrap_or(0))
        });
    }

    #[test]
    fn a_node_of_the_skinned_class_attaches_each_child() {
        let mut e = engine();
        let s = scene(&mut e, 3, 0xffff_ffff);
        returns(&mut e, FIND_SKINNED_NODE, 1);
        returns(&mut e, FIND_OBJECT_BY_NAME, 0x4a4a);
        returns(&mut e, CAST_TO_CLASS, 0x6600_0000);
        let first = object_with_slots(&mut e, &[]);
        let second = object_with_slots(&mut e, &[]);
        // The class's children are attached one after the other, with no
        // parent object; their own children are none.
        e.register_double(CHILD_COUNT, |_, a| {
            ret(if a[0] == 0x6600_0000 { 2 } else { 0 })
        });
        e.register_double(CHILD_AT, move |_, a| ret([first, second][a[1] as usize]));
        log(&mut e);
        biped_anim_apply_skinned_objects(&mut e, s.this, Ptr::new(s.node), 3, 0, 0, Ptr::NULL);
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, SET_TRANSLATION),
            vec![
                vec![first, ZERO_TRANSLATION],
                vec![second, ZERO_TRANSLATION]
            ]
        );
    }

    /// A node for `attach`: it has a virtual slot 0xe8 (remove a child), the
    /// biped has a root.
    fn attach_scene(e: &mut Engine) -> (Ptr<BipedAnim>, u32) {
        let s = scene(e, 3, 0xffff_ffff);
        let node = object_with_slots(e, &[(0xe8, 0)]);
        (s.this, node)
    }

    #[test]
    fn attaching_a_node_without_children_returns_nothing() {
        let mut e = engine();
        let (this, node) = attach_scene(&mut e);
        e.set_global(EXTRA_DATA_KEY, 0x9900);
        log(&mut e);
        let result = biped_anim_attach_skinned_object(
            &mut e,
            this,
            Ptr::new(node),
            Ptr::NULL,
            3,
            0,
            0,
            Ptr::NULL,
        );
        assert!(result.is_null());
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, FN_004ADD70), vec![vec![node], vec![node]]);
        assert_eq!(
            calls_to(&calls, SET_TRANSLATION),
            vec![vec![node, ZERO_TRANSLATION]]
        );
        assert_eq!(calls_to(&calls, GET_EXTRA_DATA), vec![vec![node, 0x9900]]);
        let collected = calls_to(&calls, FN_004AFF00);
        assert_eq!(collected[0][1..], [0, 1]);
        assert_eq!(calls_to(&calls, FN_004ADE20), vec![vec![collected[0][0]]]);
    }

    #[test]
    fn a_geometry_in_the_second_child_makes_a_new_node_under_the_parent_object() {
        let mut e = engine();
        let (this, node) = attach_scene(&mut e);
        let parent = object_with_slots(&mut e, &[(0xdc, 0)]);
        let first = object_with_slots(&mut e, &[(0x18, 0x1)]);
        let second = object_with_slots(&mut e, &[(0x18, 0x1)]);
        children_are(&mut e, vec![first, second]);
        returns(&mut e, NI_OPERATOR_NEW, 0x9000);
        returns(&mut e, NI_NODE_INIT, 0x9100);
        log(&mut e);
        let result = biped_anim_attach_skinned_object(
            &mut e,
            this,
            Ptr::new(node),
            Ptr::new(parent),
            3,
            0,
            0,
            Ptr::NULL,
        );
        // The new node is what comes back when no child has data.
        assert_eq!(result.addr(), 0x9100);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, NI_OPERATOR_NEW), vec![vec![0xac]]);
        assert_eq!(calls_to(&calls, NI_NODE_INIT), vec![vec![0x9000, 0]]);
        let add = slot_target(&e, parent, 0xdc);
        assert_eq!(calls_to(&calls, add), vec![vec![parent, 0x9100, 1]]);
    }

    #[test]
    fn an_allocation_failure_for_the_new_node_is_passed_on_as_null() {
        let mut e = engine();
        let (this, node) = attach_scene(&mut e);
        let parent = object_with_slots(&mut e, &[(0xdc, 0)]);
        let first = object_with_slots(&mut e, &[(0x18, 0x1)]);
        let second = object_with_slots(&mut e, &[(0x18, 0x1)]);
        children_are(&mut e, vec![first, second]);
        log(&mut e);
        let result = biped_anim_attach_skinned_object(
            &mut e,
            this,
            Ptr::new(node),
            Ptr::new(parent),
            3,
            0,
            0,
            Ptr::NULL,
        );
        assert!(result.is_null());
        let calls = log(&mut e);
        assert!(calls_to(&calls, NI_NODE_INIT).is_empty());
        let add = slot_target(&e, parent, 0xdc);
        assert_eq!(calls_to(&calls, add), vec![vec![parent, 0, 1]]);
    }

    #[test]
    fn children_without_a_geometry_are_collected_and_removed() {
        let mut e = engine();
        let (this, node) = attach_scene(&mut e);
        let plain = object_with_slots(&mut e, &[(0x18, 0)]);
        let geometry = object_with_slots(&mut e, &[(0x18, 0x1)]);
        children_are(&mut e, vec![plain, geometry]);
        log(&mut e);
        biped_anim_attach_skinned_object(
            &mut e,
            this,
            Ptr::new(node),
            Ptr::NULL,
            3,
            0,
            0,
            Ptr::NULL,
        );
        let calls = log(&mut e);
        // The plain child went into the collection and out of the node.
        let adds = calls_to(&calls, FN_004AFC50);
        assert_eq!(adds.len(), 1);
        let collected = calls_to(&calls, FN_004AFF00)[0][0];
        assert_eq!(adds[0][0], collected);
        let references = calls_to(&calls, NI_POINTER_INIT);
        assert!(references
            .iter()
            .any(|r| r[0] == adds[0][1] && r[1] == plain));
        let remove = slot_target(&e, node, 0xe8);
        assert_eq!(calls_to(&calls, remove), vec![vec![node, plain]]);
        // The geometry child is adjusted (004af490) but not collected.
        assert_eq!(
            calls_to(&calls, FN_004AF490),
            vec![vec![this.addr(), geometry, 0]]
        );
    }

    #[test]
    fn children_named_like_the_third_slot_are_collected_when_asked() {
        let mut e = engine();
        let (this, node) = attach_scene(&mut e);
        let named = object_with_slots(&mut e, &[(0x18, 0x1)]);
        children_are(&mut e, vec![named]);
        let name = text(&mut e, "UpperBody1");
        returns(&mut e, NAME_TEXT, name);
        let prefix = e.mem.u32(SLOT_NAMES + 8);
        returns(&mut e, TEXT_LENGTH, 9);
        returns(&mut e, COMPARE_PREFIX, 0);
        // The pip-boy prefix test never matches in this test.
        returns(&mut e, COMPARE_TEXT, 1);
        log(&mut e);
        biped_anim_attach_skinned_object(
            &mut e,
            this,
            Ptr::new(node),
            Ptr::NULL,
            3,
            1,
            0,
            Ptr::NULL,
        );
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, TEXT_LENGTH), vec![vec![prefix]]);
        assert_eq!(
            calls_to(&calls, COMPARE_PREFIX),
            vec![vec![name, prefix, 9]]
        );
        let remove = slot_target(&e, node, 0xe8);
        assert_eq!(calls_to(&calls, remove), vec![vec![node, named]]);
        // Another name stays; and the flag off never compares.
        returns(&mut e, COMPARE_PREFIX, 1);
        biped_anim_attach_skinned_object(
            &mut e,
            this,
            Ptr::new(node),
            Ptr::NULL,
            3,
            1,
            0,
            Ptr::NULL,
        );
        assert!(calls_to(&log(&mut e), remove).is_empty());
        biped_anim_attach_skinned_object(
            &mut e,
            this,
            Ptr::new(node),
            Ptr::NULL,
            3,
            0,
            0,
            Ptr::NULL,
        );
        assert!(calls_to(&log(&mut e), COMPARE_PREFIX).is_empty());
    }

    #[test]
    fn the_pip_boy_children_are_collected_by_a_name_prefix() {
        let mut e = engine();
        let (this, node) = attach_scene(&mut e);
        let named = object_with_slots(&mut e, &[(0x18, 0x1)]);
        children_are(&mut e, vec![named]);
        let name = text(&mut e, "PipBoyOff");
        returns(&mut e, NAME_TEXT, name);
        returns(&mut e, COMPARE_TEXT, 0);
        log(&mut e);
        // With the second flag: nine characters are compared with "pipboyoff".
        biped_anim_attach_skinned_object(
            &mut e,
            this,
            Ptr::new(node),
            Ptr::NULL,
            3,
            0,
            1,
            Ptr::NULL,
        );
        let calls = log(&mut e);
        let from_text = calls_to(&calls, STRING_FROM_TEXT);
        assert_eq!(from_text[0][1], name);
        assert_eq!(calls_to(&calls, FN_004AFAD0), vec![vec![from_text[0][0]]]);
        let copied = calls_to(&calls, FN_004ADD50);
        assert_eq!(copied[0][2], 9);
        let compared = calls_to(&calls, COMPARE_TEXT);
        assert_eq!(compared[0], vec![copied[0][0], PIPBOY_OFF_PREFIX]);
        let remove = slot_target(&e, node, 0xe8);
        assert_eq!(calls_to(&calls, remove), vec![vec![node, named]]);
        assert_eq!(calls_to(&calls, STRING_FREE), vec![vec![from_text[0][0]]]);
        // Without it, eight characters are compared with "pipboyon".
        biped_anim_attach_skinned_object(
            &mut e,
            this,
            Ptr::new(node),
            Ptr::NULL,
            3,
            0,
            0,
            Ptr::NULL,
        );
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, FN_004ADD50)[0][2], 8);
        assert_eq!(calls_to(&calls, COMPARE_TEXT)[0][1], PIPBOY_ON_PREFIX);
        // A name that does not match keeps the child and frees the string.
        returns(&mut e, COMPARE_TEXT, 1);
        biped_anim_attach_skinned_object(
            &mut e,
            this,
            Ptr::new(node),
            Ptr::NULL,
            3,
            0,
            0,
            Ptr::NULL,
        );
        let calls = log(&mut e);
        assert!(calls_to(&calls, remove).is_empty());
        assert_eq!(calls_to(&calls, STRING_FREE).len(), 1);
    }

    #[test]
    fn a_child_with_data_binds_its_lights_to_the_bones_and_joins_the_attached_node() {
        let mut e = engine();
        let (this, node) = attach_scene(&mut e);
        let child = object_with_slots(&mut e, &[(0x18, 0x1)]);
        children_are(&mut e, vec![child]);
        let parent = object_with_slots(&mut e, &[(0xdc, 0)]);
        let data = 0x0d0d_0000;
        returns(&mut e, NODE_DATA_POINTER, data);
        returns(&mut e, BODY_PART_OWNER, 0x77);
        returns(&mut e, BODY_PART_COUNT, 2);
        let items = block(&mut e, 8);
        e.mem.set_u32(items, 0x1010);
        e.mem.set_u32(items + 4, 0x2020);
        returns(&mut e, BODY_PART_ARRAY, items);
        // Only the first light has a name; names come from the field address.
        e.register(NAME_FIELD, |_, a| ret(a[0]));
        e.register_double(NAME_TEXT, |_, a| ret((a[0] == 0x1010) as u32));
        returns(&mut e, FN_004ADE00, 0x5151);
        e.set(this, BipedAnim::root, Ptr::new(0x1818));
        log(&mut e);
        let result = biped_anim_attach_skinned_object(
            &mut e,
            this,
            Ptr::new(node),
            Ptr::new(parent),
            3,
            0,
            0,
            Ptr::NULL,
        );
        assert_eq!(result.addr(), child);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, BODY_PART_OWNER), vec![vec![data]]);
        assert_eq!(calls_to(&calls, BODY_PART_COUNT), vec![vec![0x77]]);
        assert_eq!(calls_to(&calls, FN_004ADE00), vec![vec![0x1818, 0x1010]]);
        assert_eq!(calls_to(&calls, FN_004ADDA0), vec![vec![data, 0, 0x5151]]);
        // The unnamed light is logged with the node's parent name (here none).
        assert_eq!(
            calls_to(&calls, LOG),
            vec![vec![UNNAMED_BONE_FORMAT, 1, NULL_TEXT, 0]]
        );
        assert_eq!(
            calls_to(&calls, SET_PARENT_OBJECT),
            vec![vec![data, parent]]
        );
        let add = slot_target(&e, parent, 0xdc);
        assert_eq!(calls_to(&calls, add), vec![vec![parent, child, 1]]);
    }

    #[test]
    fn the_extra_data_of_the_node_moves_to_the_weapon_bone_with_its_geometries() {
        let mut e = engine();
        let (this, node) = attach_scene(&mut e);
        e.set(bone(this, 1), BipedBone::pParent, Ptr::new(0x7000));
        e.set_global(EXTRA_DATA_KEY, 0x9900);
        returns(&mut e, GET_EXTRA_DATA, 0x8000);
        let geometry = object_with_slots(&mut e, &[(0x18, 0x1)]);
        children_are(&mut e, vec![geometry]);
        returns(&mut e, NODE_DATA_POINTER, 0x0d0d_0000);
        log(&mut e);
        biped_anim_attach_skinned_object(
            &mut e,
            this,
            Ptr::new(node),
            Ptr::NULL,
            3,
            0,
            0,
            Ptr::NULL,
        );
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, REMOVE_EXTRA_DATA),
            vec![vec![0x7000, 0x9900]]
        );
        assert_eq!(
            calls_to(&calls, ADD_EXTRA_DATA),
            vec![vec![0x7000, 0x9900, 0x8000]]
        );
        // The geometry (with data) is added to the extra data's list at +0x34.
        let added = calls_to(&calls, ADD_TO_LIST);
        assert_eq!(added[0][0], 0x8034);
    }

    #[test]
    fn a_skinned_child_of_an_npc_gets_its_face_gen_data() {
        let mut e = engine();
        let (this, node) = attach_scene(&mut e);
        let requester = 0x0abc_0000;
        e.set(this, BipedAnim::m_pRequester, Ptr::new(requester));
        let npc = 0x0ccc_0000;
        returns(&mut e, REFERENCE_FORM, npc);
        returns(&mut e, FORM_TYPE, 0x2a);
        let child = object_with_slots(&mut e, &[(0x18, 0x1), (0xe4, 0)]);
        children_are(&mut e, vec![child]);
        returns(&mut e, NODE_SKIN_DATA, 0x5a5a);
        returns(&mut e, NODE_DATA_POINTER, 0x0d0d_0000);
        returns(&mut e, NODE_DATA_FIELD, 0x0e0e);
        returns(&mut e, NI_OBJECT_CLONE_OV2, 0x0f0f);
        returns(&mut e, SETTING_BYTE_VALUE, 1);
        // The face model comes from the caller.
        log(&mut e);
        biped_anim_attach_skinned_object(
            &mut e,
            this,
            Ptr::new(node),
            Ptr::NULL,
            3,
            0,
            0,
            Ptr::new(0x4141),
        );
        let calls = log(&mut e);
        let coords = calls_to(&calls, GET_FACE_COORD);
        assert_eq!(coords[0][0], npc);
        assert_eq!(
            calls_to(&calls, APPLY_FACE_COORDS),
            vec![vec![0x4141, coords[0][1], child, 0]]
        );
        // The skin and the property data were copied; the property clone and
        // the copy were handed to 004adde0 and 004addc0.
        let copies = calls_to(&calls, NI_OBJECT_DEEP_COPY);
        assert_eq!((copies[0][0], copies[1][0]), (0x5a5a, 0x0e0e));
        assert_eq!(calls_to(&calls, FN_004ADDE0), vec![vec![child, 0x0f0f]]);
        let kept = calls_to(&calls, FN_004ADDC0);
        assert_eq!(kept[0][0], 0x0d0d_0000);
        let set_skin = slot_target(&e, child, 0xe4);
        assert_eq!(calls_to(&calls, set_skin).len(), 1);
        assert_eq!(
            calls_to(&calls, VECTOR_DESTRUCT),
            vec![vec![coords[0][1], 0x20, 4, FACE_COORD_FREE]]
        );
    }

    #[test]
    fn without_a_face_model_the_slots_own_face_gen_model_is_used_and_no_npc_skips_it() {
        let mut e = engine();
        let (this, node) = attach_scene(&mut e);
        let requester = 0x0abc_0000;
        e.set(this, BipedAnim::m_pRequester, Ptr::new(requester));
        returns(&mut e, FN_004AE8A0, 1);
        returns(&mut e, FN_004AE790, 0x4a4a);
        returns(&mut e, REFERENCE_FORM, 0x0ccc_0000);
        // The base form is not an NPC (type 0x2b): no face data is made.
        returns(&mut e, FORM_TYPE, 0x2b);
        let child = object_with_slots(&mut e, &[(0x18, 0x1)]);
        children_are(&mut e, vec![child]);
        returns(&mut e, NODE_SKIN_DATA, 0x5a5a);
        returns(&mut e, NODE_DATA_POINTER, 0x0d0d_0000);
        log(&mut e);
        biped_anim_attach_skinned_object(
            &mut e,
            this,
            Ptr::new(node),
            Ptr::NULL,
            3,
            0,
            0,
            Ptr::NULL,
        );
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, FN_004AE790), vec![vec![this.addr(), 3]]);
        assert!(calls_to(&calls, GET_FACE_COORD).is_empty());
    }

    // ---- Session 2 -------------------------------------------------------------

    /// Makes a node's fixed-string field its own address and its name text the
    /// bytes at `+0x100`, so `node_name(x)` is [`name_of`].
    fn names_are(e: &mut Engine) {
        e.register(NAME_FIELD, |_, a| ret(a[0].wrapping_add(8)));
        e.register(NAME_TEXT, |_, a| ret(a[0].wrapping_add(0x100)));
    }

    /// The pointer `node_name` gives for the object at `address` once
    /// [`names_are`] was called.
    fn name_of(address: u32) -> u32 {
        address.wrapping_add(0x108)
    }

    /// An object whose virtual slot 0xc returns itself (an `NiNode`), the
    /// other slots as `slots`.
    fn node_object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let object = object_with_slots(e, slots);
        let vtable = e.mem.u32(object);
        let target = 0x7000_0000 + vtable + 0x0c;
        e.register_double(target, move |_, _| ret(object));
        e.mem.set_u32(vtable + 0x0c, target);
        object
    }

    /// A geometry node (virtual slot 0x18 non-zero) that is not an `NiNode`,
    /// with the other slots as `slots`.
    fn geometry_object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let mut all = vec![(0x0c, 0), (0x18, 1)];
        all.extend_from_slice(slots);
        object_with_slots(e, &all)
    }

    #[test]
    fn the_constant_functions_are_registered_by_address() {
        let mut e = engine();
        assert_eq!(e.call(0x004a_f340, &[]).u32(), 5);
        assert_eq!(e.call(0x004a_f350, &[]).u32(), 2);
    }

    #[test]
    fn the_bounded_copy_asks_for_one_more_character_of_room() {
        let mut e = engine();
        returns(&mut e, STRNCPY_S, 22);
        log(&mut e);
        let result = fn_004add50(&mut e, Ptr::new(0x100), Ptr::new(0x200), 7);
        assert_eq!(result, 22);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, STRNCPY_S), vec![vec![0x100, 8, 0x200, 7]]);
    }

    #[test]
    fn compacting_a_nodes_children_compacts_then_updates_the_array_at_0x9c() {
        let mut e = engine();
        log(&mut e);
        fn_004add70(&mut e, Ptr::new(0x1000));
        let calls = log(&mut e);
        assert_eq!(
            calls,
            vec![(FN_004AFC80, vec![0x109c]), (FN_004AFE50, vec![0x109c])]
        );
    }

    #[test]
    fn a_bone_is_stored_in_the_table_at_0x14() {
        let mut e = engine();
        let data = block(&mut e, 0x40);
        let table = block(&mut e, 0x20);
        e.mem.set_u32(data + 0x14, table);
        fn_004adda0(&mut e, Ptr::new(data), 3, Ptr::new(0x7777));
        assert_eq!(e.mem.u32(table + 12), 0x7777);
        assert_eq!(e.mem.u32(table), 0);
    }

    #[test]
    fn the_two_pointer_setters_assign_to_their_fields() {
        let mut e = engine();
        let object = block(&mut e, 0x100);
        log(&mut e);
        fn_004addc0(&mut e, Ptr::new(object), Ptr::new(0x1111));
        fn_004adde0(&mut e, Ptr::new(object), Ptr::new(0x2222));
        assert_eq!(e.mem.u32(object + 0x0c), 0x1111);
        assert_eq!(e.mem.u32(object + 0xbc), 0x2222);
        let calls = log(&mut e);
        assert_eq!(
            calls,
            vec![
                (NI_POINTER_SET, vec![object + 0x0c, 0x1111]),
                (NI_POINTER_SET, vec![object + 0xbc, 0x2222])
            ]
        );
    }

    #[test]
    fn the_object_by_fixed_name_lookup_is_recursive() {
        let mut e = engine();
        returns(&mut e, GET_OBJECT_BY_NAME, 0x4242);
        log(&mut e);
        assert_eq!(
            fn_004ade00(&mut e, Ptr::new(0x10), Ptr::new(0x20)),
            Ptr::new(0x4242)
        );
        assert_eq!(log(&mut e), vec![(GET_OBJECT_BY_NAME, vec![0x10, 0x20, 1])]);
    }

    #[test]
    fn destroying_a_node_array_stores_its_vtable_and_releases_the_block() {
        let mut e = engine();
        let array = block(&mut e, 0x10);
        e.mem.set_u32(array + 4, 0x3333);
        log(&mut e);
        fn_004ade20(&mut e, Ptr::new(array));
        assert_eq!(e.mem.u32(array), NODE_ARRAY_VTABLE);
        assert_eq!(log(&mut e), vec![(FN_004B0220, vec![0x3333])]);
        let again = block(&mut e, 0x10);
        e.mem.set_u32(again + 4, 0x4444);
        fn_004afc20(&mut e, Ptr::new(again));
        assert_eq!(e.mem.u32(again), NODE_ARRAY_VTABLE);
    }

    /// A geometry with a data object whose bone table has three entries (the
    /// middle one empty); `GetObjectByName` finds the first only.
    fn bone_scene(e: &mut Engine) -> (u32, u32, u32, u32) {
        names_are(e);
        let data = block(e, 0x40);
        let table = block(e, 0x10);
        e.mem.set_u32(data + 0x14, table);
        let items = block(e, 0x10);
        e.mem.set_u32(items, 0x6000);
        e.mem.set_u32(items + 8, 0x6100);
        returns(e, NODE_DATA_POINTER, data);
        returns(e, BODY_PART_OWNER, 0x77);
        returns(e, BODY_PART_COUNT, 3);
        returns(e, BODY_PART_ARRAY, items);
        e.register(GET_OBJECT_BY_NAME, |_, a| {
            ret(if a[1] == 0x6008 { 0x7777 } else { 0 })
        });
        (data, table, 0x6000, 0x6100)
    }

    #[test]
    fn attaching_nothing_does_nothing() {
        let mut e = engine();
        log(&mut e);
        biped_anim_attach_to_skeleton(&mut e, Ptr::new(0x5100), Ptr::NULL, Ptr::NULL, 1);
        assert!(log(&mut e).is_empty());
    }

    #[test]
    fn attaching_a_geometry_binds_its_bones_to_the_objects_found() {
        let mut e = engine();
        let (data, table, _, _) = bone_scene(&mut e);
        let geometry = geometry_object(&mut e, &[(0xdc, 0)]);
        let parent = object_with_slots(&mut e, &[(0xdc, 0)]);
        log(&mut e);
        biped_anim_attach_to_skeleton(
            &mut e,
            Ptr::new(0x5100),
            Ptr::new(geometry),
            Ptr::new(parent),
            0,
        );
        // Found, left empty, and not found (the skeleton itself).
        assert_eq!(e.mem.u32(table), 0x7777);
        assert_eq!(e.mem.u32(table + 4), 0);
        assert_eq!(e.mem.u32(table + 8), 0x5100);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, BODY_PART_COUNT), vec![vec![0x77]]);
        assert_eq!(
            calls_to(&calls, SET_PARENT_OBJECT),
            vec![vec![data, parent]]
        );
        let attach = slot_target(&e, parent, 0xdc);
        assert_eq!(calls_to(&calls, attach), vec![vec![parent, geometry, 1]]);
        assert!(calls_to(&calls, LOG).is_empty());
        // A geometry that is not a node stops there.
        assert!(calls_to(&calls, FN_004AFF00).is_empty());
    }

    #[test]
    fn a_missing_bone_of_a_geometry_is_logged_by_name_when_asked() {
        let mut e = engine();
        bone_scene(&mut e);
        let geometry = geometry_object(&mut e, &[]);
        log(&mut e);
        biped_anim_attach_to_skeleton(&mut e, Ptr::new(0x5100), Ptr::new(geometry), Ptr::NULL, 1);
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, LOG),
            vec![vec![BONE_ONLY_FORMAT, name_of(0x6100)]]
        );
        assert!(calls_to(&calls, SET_PARENT_OBJECT).is_empty());
    }

    #[test]
    fn a_missing_bone_of_a_node_names_the_part_when_the_node_name_starts_with_the_bone() {
        let mut e = engine();
        bone_scene(&mut e);
        // A geometry that is also a node.
        let node = node_object(&mut e, &[(0x18, 1), (0xe8, 0)]);
        // The names are equal as far as the bone's name is long.
        returns(&mut e, COMPARE_PREFIX, 0);
        returns(&mut e, NODE_PARENT, 0x9000);
        log(&mut e);
        biped_anim_attach_to_skeleton(&mut e, Ptr::new(0x5100), Ptr::new(node), Ptr::NULL, 1);
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, LOG),
            vec![vec![
                BONE_IN_PART_FORMAT,
                name_of(0x6100),
                name_of(0x9000),
                name_of(node)
            ]]
        );
    }

    #[test]
    fn a_missing_bone_of_another_part_names_the_node_and_the_skeleton() {
        let mut e = engine();
        bone_scene(&mut e);
        let node = node_object(&mut e, &[(0x18, 1), (0xe8, 0)]);
        returns(&mut e, COMPARE_PREFIX, 1);
        log(&mut e);
        biped_anim_attach_to_skeleton(&mut e, Ptr::new(0x5100), Ptr::new(node), Ptr::NULL, 1);
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, LOG),
            vec![vec![
                BONE_REQUESTED_FORMAT,
                name_of(0x6100),
                name_of(node),
                name_of(0x5100)
            ]]
        );
        // Without a skeleton only the bone is named.
        log(&mut e);
        biped_anim_attach_to_skeleton(&mut e, Ptr::NULL, Ptr::new(node), Ptr::NULL, 1);
        let calls = log(&mut e);
        assert!(calls_to(&calls, LOG)
            .iter()
            .any(|words| words[0] == BONE_ONLY_FORMAT));
    }

    #[test]
    fn children_that_are_not_geometries_are_moved_out_of_the_node() {
        let mut e = engine();
        names_are(&mut e);
        let node = node_object(&mut e, &[(0x18, 0), (0xe8, 0)]);
        let moved_child = node_object(&mut e, &[(0x18, 0)]);
        let geometry_child = geometry_object(&mut e, &[]);
        children_are(&mut e, vec![0, moved_child, moved_child, geometry_child]);
        returns(&mut e, NODE_CHECK_9C, 1);
        log(&mut e);
        biped_anim_attach_to_skeleton(&mut e, Ptr::new(0x5100), Ptr::new(node), Ptr::NULL, 0);
        let calls = log(&mut e);
        // The array of moved children is made, compacted before and after.
        assert_eq!(calls_to(&calls, FN_004AFF00).len(), 1);
        assert_eq!(calls_to(&calls, FN_004AFC80), vec![vec![node + 0x9c]; 2]);
        assert_eq!(calls_to(&calls, FN_004AFE50), vec![vec![node + 0x9c]; 2]);
        assert_eq!(calls_to(&calls, FN_004AFC50).len(), 2);
        let remove = slot_target(&e, node, 0xe8);
        assert_eq!(
            calls_to(&calls, remove),
            vec![vec![node, moved_child], vec![node, moved_child]]
        );
        // The wrong export is logged once: the node above, the node, the skeleton.
        assert_eq!(
            calls_to(&calls, LOG),
            vec![vec![
                EXPORTED_WRONG_FORMAT,
                name_of(0),
                name_of(node),
                name_of(0x5100)
            ]]
        );
        // The array is destroyed.
        assert_eq!(calls_to(&calls, FN_004B0220).len(), 1);
    }

    #[test]
    fn children_of_the_skinned_class_are_attached_with_the_flag_set() {
        let mut e = engine();
        names_are(&mut e);
        let node = node_object(&mut e, &[(0x18, 0), (0xe8, 0)]);
        let child = node_object(&mut e, &[(0x18, 0)]);
        e.register_double(CHILD_COUNT, move |_, a| ret((a[0] == node) as u32));
        e.register_double(CHILD_AT, move |_, _| ret(child));
        returns(&mut e, IS_KIND_OF, 1);
        log(&mut e);
        biped_anim_attach_to_skeleton(&mut e, Ptr::new(0x5100), Ptr::new(node), Ptr::NULL, 0);
        let calls = log(&mut e);
        // The child was attached itself (its own array), nothing was moved.
        assert_eq!(calls_to(&calls, FN_004AFF00).len(), 2);
        assert!(calls_to(&calls, FN_004AFC50).is_empty());
        assert_eq!(calls_to(&calls, IS_KIND_OF)[0], vec![SKINNED_CLASS, node]);
    }

    #[test]
    fn a_child_that_is_a_geometry_is_attached_with_the_callers_flag() {
        let mut e = engine();
        names_are(&mut e);
        let node = node_object(&mut e, &[(0x18, 0), (0xe8, 0)]);
        let child = geometry_object(&mut e, &[]);
        e.register_double(CHILD_COUNT, move |_, a| ret((a[0] == node) as u32));
        e.register_double(CHILD_AT, move |_, _| ret(child));
        returns(&mut e, NODE_DATA_POINTER, 0x6600);
        returns(&mut e, BODY_PART_COUNT, 1);
        returns(&mut e, BODY_PART_ARRAY, 0x6700);
        // The bone table of the child's data.
        let items = block(&mut e, 4);
        e.mem.set_u32(items, 0x6000);
        returns(&mut e, BODY_PART_ARRAY, items);
        let table = block(&mut e, 4);
        let data = block(&mut e, 0x20);
        e.mem.set_u32(data + 0x14, table);
        returns(&mut e, NODE_DATA_POINTER, data);
        log(&mut e);
        biped_anim_attach_to_skeleton(&mut e, Ptr::new(0x5100), Ptr::new(node), Ptr::NULL, 1);
        let calls = log(&mut e);
        // The child's bone was not found: the flag reaches it, and as the child
        // is not a node, only the bone name is logged.
        assert_eq!(e.mem.u32(table), 0x5100);
        assert_eq!(
            calls_to(&calls, LOG),
            vec![vec![BONE_ONLY_FORMAT, name_of(0x6000)]]
        );
    }

    /// The scene of `AttachToParent`: names work, the key global is set, the
    /// extra data is a string extra data naming a parent found by name.
    /// Returns the node, the parent and the root.
    fn parent_scene(e: &mut Engine) -> (u32, u32, u32) {
        names_are(e);
        e.set_global(PARENT_NODE_KEY, 0x2468);
        returns(e, GET_EXTRA_DATA, 0x5500);
        returns(e, CAST_TO_CLASS, 0x6600);
        e.register(STRING_EXTRA_TEXT, |_, a| ret(a[0].wrapping_add(0x0c)));
        let parent = object_with_slots(e, &[(0xdc, 0)]);
        returns(e, GET_OBJECT_BY_NAME, parent);
        let node = object_with_slots(e, &[(0xa4, 0)]);
        (node, parent, 0x5100)
    }

    #[test]
    fn a_part_without_parent_extra_data_is_logged_and_moved() {
        let mut e = engine();
        names_are(&mut e);
        e.set_global(PARENT_NODE_KEY, 0x2468);
        log(&mut e);
        biped_anim_attach_to_parent(
            &mut e,
            Ptr::new(0x5100),
            Ptr::new(0x7000),
            Ptr::new(0x7100),
            Ptr::NULL,
            3,
            0,
        );
        let calls = log(&mut e);
        // Both the source and the node were asked, with the key.
        assert_eq!(
            calls_to(&calls, GET_EXTRA_DATA),
            vec![vec![0x7100, 0x2468], vec![0x7000, 0x2468]]
        );
        assert_eq!(
            calls_to(&calls, LOG),
            vec![vec![PARENT_EXTRA_MISSING_FORMAT, name_of(0x7000)]]
        );
        assert_eq!(calls_to(&calls, SET_MOTION), vec![vec![0x7000, 4, 1, 1, 1]]);
    }

    #[test]
    fn extra_data_that_is_not_a_string_is_logged() {
        let mut e = engine();
        let (node, _, root) = parent_scene(&mut e);
        returns(&mut e, CAST_TO_CLASS, 0);
        log(&mut e);
        biped_anim_attach_to_parent(
            &mut e,
            Ptr::new(root),
            Ptr::new(node),
            Ptr::NULL,
            Ptr::NULL,
            3,
            0,
        );
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, CAST_TO_CLASS),
            vec![vec![STRING_EXTRA_CLASS, 0x5500]]
        );
        assert_eq!(
            calls_to(&calls, LOG),
            vec![vec![EXTRA_NOT_STRING_FORMAT, name_of(node)]]
        );
    }

    #[test]
    fn a_part_is_attached_to_the_parent_the_extra_data_names() {
        let mut e = engine();
        let (node, parent, root) = parent_scene(&mut e);
        e.register_double(NODE_SCALE, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        returns(&mut e, BODY_PART_ARRAY, 9);
        returns(&mut e, SHADOW_SCENE_NODE, 0x5a5a);
        let scabbard = object_with_slots(&mut e, &[]);
        e.register_double(FIND_OBJECT_BY_NAME, move |_, a| {
            ret(if a[1] == SCABBARD_NODE_NAME {
                scabbard
            } else {
                0
            })
        });
        log(&mut e);
        biped_anim_attach_to_parent(
            &mut e,
            Ptr::new(root),
            Ptr::new(node),
            Ptr::NULL,
            Ptr::NULL,
            3,
            0,
        );
        let calls = log(&mut e);
        // The parent is looked up by the text of the string extra data (+0xc).
        assert_eq!(
            calls_to(&calls, GET_OBJECT_BY_NAME),
            vec![vec![root, 0x660c, 1]]
        );
        let attach = slot_target(&e, parent, 0xdc);
        assert_eq!(
            calls_to(&calls, attach),
            vec![vec![parent, node, 1], vec![parent, scabbard, 1]]
        );
        // The record built from the count: its float, and two zero bytes.
        let record = calls_to(&calls, BUILD_RECORD);
        assert_eq!(
            record[0][1..],
            [9.0f32.to_bits(), 0, 0],
            "the count as a float"
        );
        assert_eq!(calls_to(&calls, BODY_PART_ARRAY), vec![vec![COUNT_OBJECT]]);
        let record_slot = slot_target(&e, node, 0xa4);
        assert_eq!(
            calls_to(&calls, record_slot),
            vec![vec![node, record[0][0], 0]]
        );
        assert_eq!(
            calls_to(&calls, SHADOW_ADD_OBJECT),
            vec![vec![0x5a5a, node]]
        );
        assert_eq!(calls_to(&calls, NOTIFY_ATTACHED), vec![vec![parent, 1]]);
        assert_eq!(calls_to(&calls, UPDATE_PROPERTIES), vec![vec![scabbard]]);
        assert_eq!(calls_to(&calls, FN_004AFC80), vec![vec![node + 0x9c]]);
        assert!(calls_to(&calls, LOG).is_empty());
    }

    #[test]
    fn a_missing_parent_of_a_form_type_0x2b_reference_removes_the_scabbard() {
        let mut e = engine();
        let (node, _, root) = parent_scene(&mut e);
        returns(&mut e, GET_OBJECT_BY_NAME, 0);
        returns(&mut e, FIND_REFERENCE_FOR_3D, 0x8800);
        returns(&mut e, REFERENCE_FORM, 0x9900);
        returns(&mut e, FORM_TYPE, 0x2b);
        log(&mut e);
        biped_anim_attach_to_parent(
            &mut e,
            Ptr::new(root),
            Ptr::new(node),
            Ptr::NULL,
            Ptr::NULL,
            3,
            0,
        );
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, FIND_REFERENCE_FOR_3D), vec![vec![root]]);
        assert_eq!(calls_to(&calls, REMOVE_SCABBARD), vec![vec![node]]);
        assert!(calls_to(&calls, LOG).is_empty());
    }

    #[test]
    fn a_missing_parent_is_logged_by_name_otherwise() {
        let mut e = engine();
        let (node, _, root) = parent_scene(&mut e);
        returns(&mut e, GET_OBJECT_BY_NAME, 0);
        returns(&mut e, FIND_REFERENCE_FOR_3D, 0x8800);
        returns(&mut e, REFERENCE_FORM, 0x9900);
        returns(&mut e, FORM_TYPE, 0x2a);
        log(&mut e);
        biped_anim_attach_to_parent(
            &mut e,
            Ptr::new(root),
            Ptr::new(node),
            Ptr::NULL,
            Ptr::NULL,
            3,
            0,
        );
        let calls = log(&mut e);
        assert!(calls_to(&calls, REMOVE_SCABBARD).is_empty());
        // The text of the extra data: the name of 0x660c.
        assert_eq!(
            calls_to(&calls, LOG),
            vec![vec![PARENT_NOT_FOUND_FORMAT, 0x660c + 0x100, name_of(node)]]
        );
        // Without a reference there is nothing to ask the form of either.
        returns(&mut e, FIND_REFERENCE_FOR_3D, 0);
        log(&mut e);
        biped_anim_attach_to_parent(
            &mut e,
            Ptr::new(root),
            Ptr::new(node),
            Ptr::NULL,
            Ptr::NULL,
            3,
            0,
        );
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, LOG).len(), 1);
        assert!(calls_to(&calls, REMOVE_SCABBARD).is_empty());
    }

    #[test]
    fn the_players_first_person_node_gets_the_weapon_flag_on_its_backpack() {
        let mut e = engine();
        let (node, _, root) = parent_scene(&mut e);
        returns(&mut e, FIND_OBJECT_BY_NAME, 0x4400);
        returns(&mut e, PLAYER_NODE, root);
        log(&mut e);
        biped_anim_attach_to_parent(
            &mut e,
            Ptr::new(root),
            Ptr::new(node),
            Ptr::NULL,
            Ptr::NULL,
            3,
            0,
        );
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, PLAYER_NODE), vec![vec![0x0f00_0000, 1]]);
        assert_eq!(calls_to(&calls, SET_WEAPON_FLAG), vec![vec![0x4400, 1]]);
        assert!(calls_to(&calls, SET_NODE_SCALE).is_empty());
    }

    #[test]
    fn another_backpack_gets_the_scale_and_the_parent_its_extra_data_names() {
        let mut e = engine();
        let (node, _, root) = parent_scene(&mut e);
        let backpack_parent = object_with_slots(&mut e, &[(0xdc, 0)]);
        // The backpack is found by name; its parent comes from the `UPB` data.
        e.register_double(FIND_OBJECT_BY_NAME, |_, a| {
            ret(if a[1] == BACKPACK_NAME { 0x4400 } else { 0 })
        });
        returns(&mut e, PLAYER_NODE, 0x1111);
        e.register_double(NODE_SCALE, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        returns(&mut e, FIXED_STRING_INIT, 0x7755);
        e.register_double(GET_OBJECT_BY_NAME, move |_, a| {
            // The skeleton's parent node for `node` first, then the backpack's.
            ret(if a[1] == 0x660c { backpack_parent } else { 0 })
        });
        log(&mut e);
        biped_anim_attach_to_parent(
            &mut e,
            Ptr::new(root),
            Ptr::new(node),
            Ptr::NULL,
            Ptr::NULL,
            3,
            0,
        );
        let calls = log(&mut e);
        // The scale is the parent's (0.5); the lookups by `0x660c` find the same parent.
        assert_eq!(
            calls_to(&calls, SET_NODE_SCALE),
            vec![vec![0x4400, 0.5f32.to_bits()]]
        );
        let keyed = calls_to(&calls, GET_EXTRA_DATA);
        assert_eq!(keyed.last().unwrap(), &vec![0x4400, 0x7755]);
        assert_eq!(calls_to(&calls, FIXED_STRING_INIT)[0][1], BACKPACK_KEY_NAME);
        let attach = slot_target(&e, backpack_parent, 0xdc);
        assert_eq!(
            calls_to(&calls, attach),
            vec![
                vec![backpack_parent, node, 1],
                vec![backpack_parent, 0x4400, 1]
            ]
        );
    }

    #[test]
    fn the_first_person_biped_has_its_havok_killed() {
        let mut e = engine();
        let (node, _, root) = parent_scene(&mut e);
        let this = biped(&mut e);
        returns(&mut e, GET_BIPED, this.addr());
        log(&mut e);
        biped_anim_attach_to_parent(
            &mut e,
            Ptr::new(root),
            Ptr::new(node),
            Ptr::NULL,
            this,
            5,
            0,
        );
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, KILL_HAVOK), vec![vec![node, 1, 1]]);
        assert!(calls_to(&calls, SET_MOTION).is_empty());
        assert!(calls_to(&calls, GET_COLLISION_OBJECT).is_empty());
    }

    #[test]
    fn a_weapon_without_havok_data_is_logged_and_still_gets_its_motion() {
        let mut e = engine();
        let (node, _, root) = parent_scene(&mut e);
        let this = biped(&mut e);
        returns(&mut e, GET_COLLISION_OBJECT, 0);
        log(&mut e);
        biped_anim_attach_to_parent(
            &mut e,
            Ptr::new(root),
            Ptr::new(node),
            Ptr::NULL,
            this,
            WEAPON_SLOT,
            0,
        );
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, LOG),
            vec![vec![NO_HAVOK_FORMAT, name_of(node)]]
        );
        assert_eq!(calls_to(&calls, SET_MOTION), vec![vec![node, 4, 1, 1, 1]]);
    }

    #[test]
    fn a_weapon_with_a_bad_shape_or_layer_is_logged() {
        let mut e = engine();
        let (node, _, root) = parent_scene(&mut e);
        let this = biped(&mut e);
        returns(&mut e, GET_COLLISION_OBJECT, 0x31);
        returns(&mut e, COLLISION_BODY, 0x32);
        returns(&mut e, BODY_SHAPE, 0);
        returns(&mut e, COPY_LAYER, 0x34);
        returns(&mut e, LAYER_OF, 4);
        log(&mut e);
        biped_anim_attach_to_parent(
            &mut e,
            Ptr::new(root),
            Ptr::new(node),
            Ptr::NULL,
            this,
            WEAPON_SLOT,
            0,
        );
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, COLLISION_BODY), vec![vec![0x31]]);
        assert_eq!(calls_to(&calls, BODY_SHAPE), vec![vec![0x32]]);
        assert_eq!(calls_to(&calls, LAYER_OF), vec![vec![0x34]]);
        assert_eq!(
            calls_to(&calls, LOG),
            vec![
                vec![NO_SHAPE_FORMAT, name_of(node)],
                vec![WEAPON_LAYER_FORMAT, name_of(node)]
            ]
        );
        // A good weapon logs nothing.
        returns(&mut e, BODY_SHAPE, 0x33);
        returns(&mut e, LAYER_OF, WEAPON_LAYER);
        log(&mut e);
        biped_anim_attach_to_parent(
            &mut e,
            Ptr::new(root),
            Ptr::new(node),
            Ptr::NULL,
            this,
            WEAPON_SLOT,
            0,
        );
        assert!(calls_to(&log(&mut e), LOG).is_empty());
    }

    #[test]
    fn the_parent_extra_data_key_is_the_global() {
        let mut e = engine();
        e.set_global(PARENT_NODE_KEY, 0x2468);
        assert_eq!(fn_004ae780(&mut e), Ptr::new(0x2468));
        assert_eq!(e.call(0x004a_e780, &[]).u32(), 0x2468);
    }

    #[test]
    fn the_face_gen_model_of_a_slot_is_loaded_under_the_part_lock() {
        let mut e = engine();
        let this = biped(&mut e);
        let model = object_with_slots(&mut e, &[(0x14, 0xabc0)]);
        set_slot(&mut e, this, 4, 0x1111, model, 0);
        returns(&mut e, GET_EGM_FILE, 0x6161);
        returns(&mut e, FACE_GEN_LOAD, 0x7272);
        log(&mut e);
        let result = biped_anim_load_face_gen_model(&mut e, this, 4);
        assert_eq!(result, Ptr::new(0x7272));
        let calls = log(&mut e);
        let order: Vec<u32> = addresses(&calls);
        assert_eq!(order[0], LOCK_ENTER);
        assert_eq!(order[order.len() - 1], LOCK_LEAVE);
        assert_eq!(calls[0].1, vec![PART_LOCK, 0]);
        let copy = calls_to(&calls, STRING_COPY);
        assert_eq!(copy[0][1..], [0x104, MESHES_PREFIX]);
        let append = calls_to(&calls, STRING_APPEND);
        assert_eq!(append[0][1..], [0x104, 0xabc0]);
        assert_eq!(copy[0][0], append[0][0]);
        let text = calls_to(&calls, STRING_INIT)[0][0];
        assert_eq!(
            calls_to(&calls, GET_EGM_FILE),
            vec![vec![text, copy[0][0], 0xffff_ffff]]
        );
        assert_eq!(
            calls_to(&calls, FACE_GEN_LOAD),
            vec![vec![0x6161, 0, 0, 0, 1, 0xffff_ffff, 0]]
        );
        assert_eq!(calls_to(&calls, STRING_FREE), vec![vec![text]]);
    }

    #[test]
    fn a_slot_without_a_model_loads_no_face_gen_model() {
        let mut e = engine();
        let this = biped(&mut e);
        log(&mut e);
        assert_eq!(biped_anim_load_face_gen_model(&mut e, this, 4), Ptr::NULL);
        assert_eq!(
            log(&mut e),
            vec![
                (LOCK_ENTER, vec![PART_LOCK, 0]),
                (LOCK_LEAVE, vec![PART_LOCK])
            ]
        );
    }

    #[test]
    fn a_slot_holds_face_gen_data_when_its_model_has_bit_0() {
        let mut e = engine();
        let this = biped(&mut e);
        assert!(!fn_004ae8a0(&mut e, this, 2));
        let model = block(&mut e, 0x40);
        set_slot(&mut e, this, 2, 0x1111, model, 0);
        assert!(!fn_004ae8a0(&mut e, this, 2));
        e.mem.set_u8(model + 0x14, 0x03);
        assert!(fn_004ae8a0(&mut e, this, 2));
    }

    #[test]
    fn the_face_gen_bit_is_bit_0_of_the_byte_at_0x14() {
        let mut e = engine();
        let model = block(&mut e, 0x40);
        e.mem.set_u8(model + 0x14, 0xfe);
        assert_eq!(fn_004ae8f0(&mut e, Ptr::new(model)), 0);
        e.mem.set_u8(model + 0x14, 0x07);
        assert_eq!(fn_004ae8f0(&mut e, Ptr::new(model)), 1);
    }

    /// A slot-3 scene for the helmet functions: the part's model has the
    /// texture swap slot (0x1c) and the form the platform one (0xac).
    fn helmet_scene(e: &mut Engine, swap: u32, platform: u32) -> (Ptr<BipedAnim>, u32, u32) {
        let s = scene(e, 3, 1);
        let model = object_with_slots(e, &[(0x14, 0xabc0), (0x1c, swap)]);
        let form = object_with_slots(e, &[(0xac, platform), (0x130, 0x7e7e)]);
        set_slot(e, s.this, 3, form, model, 0);
        let file = block(e, 0x40);
        (s.this, file, s.node)
    }

    #[test]
    fn a_helmet_is_cloned_and_its_textures_swapped() {
        let mut e = engine();
        let (this, file, _) = helmet_scene(&mut e, 0x6677, 1);
        let copy = node_object(&mut e, &[]);
        returns(&mut e, NI_OBJECT_CLONE, copy);
        returns(&mut e, IS_KIND_OF, 1);
        log(&mut e);
        let result = biped_anim_clone_helmet(&mut e, this, Ptr::new(0x5151), Ptr::new(file), 3);
        assert_eq!(result, Ptr::new(copy));
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, NI_OBJECT_CLONE)[0][0], file);
        assert_eq!(calls_to(&calls, SET_LOD_MULT_TYPE), vec![vec![copy, 7]]);
        assert_eq!(
            calls_to(&calls, SET_TRANSLATION),
            vec![vec![copy, ZERO_TRANSLATION]]
        );
        assert_eq!(
            calls_to(&calls, SET_ROTATION),
            vec![vec![copy, IDENTITY_ROTATION]]
        );
        assert_eq!(calls_to(&calls, SWAP_TEXTURES), vec![vec![0x6677, copy]]);
        assert_eq!(calls_to(&calls, SWAP_PLATFORM_TEXTURES), vec![vec![copy]]);
    }

    #[test]
    fn a_morphing_helmet_is_deep_copied_and_other_cases_swap_nothing() {
        let mut e = engine();
        let (this, file, _) = helmet_scene(&mut e, 0, 0);
        let copy = object_with_slots(&mut e, &[]);
        returns(&mut e, HAS_MORPHER_CONTROLLER, 1);
        returns(&mut e, DEEP_COPY_SAME_TEXTURES, copy);
        e.set_global(TES_GLOBAL, 0x3030);
        log(&mut e);
        let result = biped_anim_clone_helmet(&mut e, this, Ptr::new(0x5151), Ptr::new(file), 3);
        assert_eq!(result, Ptr::new(copy));
        let calls = log(&mut e);
        let deep = calls_to(&calls, DEEP_COPY_SAME_TEXTURES);
        assert_eq!((deep[0][0], deep[0][1]), (0x3030, file));
        assert!(calls_to(&calls, NI_OBJECT_CLONE).is_empty());
        assert!(calls_to(&calls, SET_LOD_MULT_TYPE).is_empty());
        assert!(calls_to(&calls, SWAP_TEXTURES).is_empty());
        assert!(calls_to(&calls, SWAP_PLATFORM_TEXTURES).is_empty());
    }

    #[test]
    fn a_helmet_is_not_cloned_twice_or_without_its_arguments() {
        let mut e = engine();
        let (this, file, _) = helmet_scene(&mut e, 0, 0);
        e.set(object(this, 3), BipedObject::pPartClone, Ptr::new(0x9090));
        log(&mut e);
        assert_eq!(
            biped_anim_clone_helmet(&mut e, this, Ptr::new(0x5151), Ptr::new(file), 3),
            Ptr::NULL
        );
        e.set(object(this, 3), BipedObject::pPartClone, Ptr::NULL);
        assert_eq!(
            biped_anim_clone_helmet(&mut e, this, Ptr::NULL, Ptr::new(file), 3),
            Ptr::NULL
        );
        assert_eq!(
            biped_anim_clone_helmet(&mut e, this, Ptr::new(0x5151), Ptr::NULL, 3),
            Ptr::NULL
        );
        assert!(log(&mut e).is_empty());
        // A clone that fails gives null.
        returns(&mut e, NI_OBJECT_CLONE, 0);
        assert_eq!(
            biped_anim_clone_helmet(&mut e, this, Ptr::new(0x5151), Ptr::new(file), 3),
            Ptr::NULL
        );
    }

    #[test]
    fn an_attached_helmet_is_named_and_hung_on_the_bone_of_its_slot() {
        let mut e = engine();
        let s = scene(&mut e, 3, 1);
        let bone_object = object_with_slots(&mut e, &[(0xdc, 0)]);
        e.set(bone(s.this, 1), BipedBone::pParent, Ptr::new(bone_object));
        returns(&mut e, ACTOR_ROOT, 0x1234);
        log(&mut e);
        biped_anim_attach_helmet(&mut e, s.this, Ptr::new(0x5151), Ptr::new(s.node), 3);
        let calls = log(&mut e);
        // Not skinned: attached to the parent given by the extra data (none
        // here), named, add-on nodes asked, then hung on the bone.
        assert_eq!(calls_to(&calls, SET_NAME).len(), 1);
        assert_eq!(calls_to(&calls, SET_NAME)[0][0], s.node);
        let hang = slot_target(&e, bone_object, 0xdc);
        assert_eq!(calls_to(&calls, hang), vec![vec![bone_object, s.node, 1]]);
        assert_eq!(slot_words(&e, object(s.this, 3))[2], s.node);
        assert_eq!(calls_to(&calls, SET_MOTION).len(), 1);
        // The actor's 3D is refreshed at the end.
        assert_eq!(calls.last().unwrap(), &(UPDATE_PROPERTIES, vec![0x1234]));
        assert_eq!(calls_to(&calls, ACTOR_ROOT).len(), 2);
    }

    #[test]
    fn a_helmet_without_a_bone_index_goes_through_the_word_before_the_bones() {
        let mut e = engine();
        let s = scene(&mut e, 3, 0xffff_ffff);
        // Index -1 reads `bone[-1].pParent` at `this + 0`: the root.
        let root = object_with_slots(&mut e, &[(0xdc, 0)]);
        e.set(s.this, BipedAnim::root, Ptr::new(root));
        log(&mut e);
        biped_anim_attach_helmet(&mut e, s.this, Ptr::new(0x5151), Ptr::new(s.node), 3);
        let calls = log(&mut e);
        let hang = slot_target(&e, root, 0xdc);
        assert_eq!(calls_to(&calls, hang), vec![vec![root, s.node, 1]]);
    }

    #[test]
    fn a_helmet_whose_bone_is_missing_goes_under_the_root_when_it_has_a_parent() {
        let mut e = engine();
        let s = scene(&mut e, 3, 2);
        let root = object_with_slots(&mut e, &[(0xdc, 0)]);
        e.set(s.this, BipedAnim::root, Ptr::new(root));
        log(&mut e);
        // No parent: not attached anywhere.
        biped_anim_attach_helmet(&mut e, s.this, Ptr::new(0x5151), Ptr::new(s.node), 3);
        let calls = log(&mut e);
        let hang = slot_target(&e, root, 0xdc);
        assert!(calls_to(&calls, hang).is_empty());
        returns(&mut e, NODE_PARENT, 0x9999);
        biped_anim_attach_helmet(&mut e, s.this, Ptr::new(0x5151), Ptr::new(s.node), 3);
        let calls = log(&mut e);
        // The root gets the part, since the bone is missing and it has a parent.
        assert!(calls_to(&calls, hang).contains(&vec![root, s.node, 1]));
    }

    #[test]
    fn a_skinned_helmet_is_not_hung_on_a_bone() {
        let mut e = engine();
        let s = scene(&mut e, 3, 1);
        let bone_object = object_with_slots(&mut e, &[(0xdc, 0)]);
        e.set(bone(s.this, 1), BipedBone::pParent, Ptr::new(bone_object));
        // The node holds skinned objects: ApplySkinnedObjects marks the slot.
        returns(&mut e, FIND_SKINNED_NODE, 1);
        returns(&mut e, FIND_OBJECT_BY_NAME, 0x4a4a);
        log(&mut e);
        biped_anim_attach_helmet(&mut e, s.this, Ptr::new(0x5151), Ptr::new(s.node), 3);
        let calls = log(&mut e);
        let hang = slot_target(&e, bone_object, 0xdc);
        assert!(calls_to(&calls, hang).is_empty());
    }

    #[test]
    fn the_first_person_player_flags_are_passed_to_the_skinned_objects() {
        let mut e = engine();
        let s = scene(&mut e, 3, 1);
        e.set(s.this, BipedAnim::m_pRequester, Ptr::new(0x0f00_0000));
        returns(&mut e, IS_FIRST_PERSON_BIPED, 1);
        returns(&mut e, ACTOR_ROOT, 0x1234);
        log(&mut e);
        biped_anim_attach_helmet(&mut e, s.this, Ptr::new(0x5151), Ptr::new(s.node), 3);
        let calls = log(&mut e);
        // Asked twice (the first answer is not used).
        assert_eq!(
            calls_to(&calls, IS_FIRST_PERSON_BIPED),
            vec![vec![0x0f00_0000, s.this.addr()]; 2]
        );
    }

    #[test]
    fn nothing_is_attached_without_a_face_model_or_a_node_but_the_3d_is_refreshed() {
        let mut e = engine();
        let s = scene(&mut e, 3, 1);
        returns(&mut e, ACTOR_ROOT, 0x1234);
        log(&mut e);
        biped_anim_attach_helmet(&mut e, s.this, Ptr::NULL, Ptr::new(s.node), 3);
        biped_anim_attach_helmet(&mut e, s.this, Ptr::new(1), Ptr::NULL, 3);
        let calls = log(&mut e);
        assert_eq!(
            addresses(&calls),
            vec![
                ACTOR_ROOT,
                ACTOR_ROOT,
                UPDATE_PROPERTIES,
                ACTOR_ROOT,
                ACTOR_ROOT,
                UPDATE_PROPERTIES
            ]
        );
        // Nor does an actor without a 3D root get an update.
        returns(&mut e, ACTOR_ROOT, 0);
        log(&mut e);
        biped_anim_attach_helmet(&mut e, s.this, Ptr::NULL, Ptr::NULL, 3);
        assert_eq!(log(&mut e), vec![(ACTOR_ROOT, vec![0x0abc_0000])]);
    }

    #[test]
    fn queueing_through_a_fresh_handle_builds_and_destroys_it() {
        let mut e = engine();
        let this = biped(&mut e);
        let model = block(&mut e, 0x40);
        set_slot(&mut e, this, 2, 0x1111, model, 0);
        e.set_global(MODEL_LOADER, 0x5555);
        log(&mut e);
        fn_004aede0(&mut e, this, 5, 6, 2);
        let calls = log(&mut e);
        let handle = calls_to(&calls, QUEUE_OBJECT_INIT)[0][0];
        assert_eq!(calls_to(&calls, QUEUE_OBJECT_INIT), vec![vec![handle, 0]]);
        assert_eq!(
            calls_to(&calls, LOADER_QUEUE_PART),
            vec![vec![0x5555, this.addr(), handle, 5, 6, 2]]
        );
        assert_eq!(calls.last().unwrap(), &(QUEUE_OBJECT_FREE, vec![handle]));
    }

    #[test]
    fn a_part_already_queued_is_not_queued_again() {
        let mut e = engine();
        let this = biped(&mut e);
        let model = block(&mut e, 0x40);
        set_slot(&mut e, this, 2, 0x1111, model, 0);
        let handle = block(&mut e, 4);
        e.mem.set_u32(handle, 0x7a7a);
        log(&mut e);
        fn_004aee60(&mut e, this, 5, 6, Ptr::new(handle), 2);
        assert_eq!(log(&mut e), vec![(READ_WORD, vec![handle])]);
    }

    #[test]
    fn a_slot_without_a_model_clears_the_queued_file() {
        let mut e = engine();
        let this = biped(&mut e);
        let handle = block(&mut e, 4);
        log(&mut e);
        fn_004aee60(&mut e, this, 5, 6, Ptr::new(handle), 2);
        assert_eq!(
            log(&mut e),
            vec![
                (READ_WORD, vec![handle]),
                (QUEUED_FILE_ASSIGN, vec![handle, 0])
            ]
        );
        // A model word of -1 does too.
        e.set(object(this, 2), BipedObject::pPart, Ptr::new(0xffff_ffff));
        fn_004aee60(&mut e, this, 5, 6, Ptr::new(handle), 2);
        assert_eq!(calls_to(&log(&mut e), QUEUED_FILE_ASSIGN).len(), 1);
    }

    /// The model, form and requester of an add-on, with the file, the clone
    /// and the root `LoadAndAttachAddOn` works with.
    struct AddOn {
        form: u32,
        model: u32,
        requester: u32,
        root: u32,
        file: u32,
        copy: u32,
    }

    fn add_on(e: &mut Engine) -> AddOn {
        names_are(e);
        e.set_global(MODEL_LOADER, 0x5555);
        e.set_global(EXTRA_DATA_KEY, 0x9900);
        tables(e);
        let path = text(e, "add.nif");
        let model = object_with_slots(e, &[(0x14, path), (0x1c, 0x6677)]);
        let form = object_with_slots(e, &[(0xac, 1)]);
        let root = object_with_slots(e, &[(0xdc, 0)]);
        let copy = node_object(e, &[]);
        returns(e, LOAD_FILE, 0xf11e);
        returns(e, NI_OBJECT_CLONE, copy);
        returns(e, GET_EXTRA_DATA, 0x8000);
        AddOn {
            form,
            model,
            requester: 0x0abc_0000,
            root,
            file: 0xf11e,
            copy,
        }
    }

    #[test]
    fn an_add_on_is_loaded_cloned_attached_and_hung_under_the_root() {
        let mut e = engine();
        let a = add_on(&mut e);
        log(&mut e);
        let result = biped_anim_load_and_attach_add_on(
            &mut e,
            Ptr::new(a.form),
            Ptr::new(a.model),
            0xffff_ffff,
            Ptr::new(a.requester),
            Ptr::new(a.root),
        );
        assert_eq!(result, Ptr::new(a.copy));
        let calls = log(&mut e);
        let path = calls_to(&calls, LOAD_FILE);
        assert_eq!(path[0][0], 0x5555);
        assert_eq!(calls_to(&calls, NI_OBJECT_CLONE)[0][0], a.file);
        assert_eq!(path[0][2..], [3, 1, 0, 0, 0]);
        assert_eq!(calls_to(&calls, SWAP_TEXTURES), vec![vec![0x6677, a.copy]]);
        assert_eq!(calls_to(&calls, SWAP_PLATFORM_TEXTURES), vec![vec![a.copy]]);
        // The extra data kept under the key was taken out and put back.
        assert_eq!(calls_to(&calls, REMOVE_ALL_EXTRA_DATA), vec![vec![a.copy]]);
        assert_eq!(
            calls_to(&calls, ADD_EXTRA_DATA),
            vec![vec![a.copy, 0x9900, 0x8000]]
        );
        let order = addresses(&calls);
        let remove = order.iter().position(|&x| x == REMOVE_ALL_EXTRA_DATA);
        let add = order.iter().position(|&x| x == ADD_EXTRA_DATA);
        assert!(remove < add);
        assert_eq!(
            calls_to(&calls, ADD_MASTER_PARTICLE_ADDON_NODES),
            vec![vec![a.copy]]
        );
        // The part is attached by the parent extra data (the file is asked)...
        assert_eq!(calls_to(&calls, SET_MOTION), vec![vec![a.copy, 4, 1, 1, 1]]);
        // ...and, having no parent, hung under the root.
        let hang = slot_target(&e, a.root, 0xdc);
        assert_eq!(calls_to(&calls, hang), vec![vec![a.root, a.copy, 1]]);
        assert_eq!(calls_to(&calls, UPDATE_PROPERTIES), vec![vec![a.copy]]);
    }

    #[test]
    fn an_add_on_goes_under_the_bone_of_its_slot_and_defaults_to_the_actors_root() {
        let mut e = engine();
        let a = add_on(&mut e);
        e.mem.set_u32(SLOT_BONES + 4 * 3, 1);
        let bone_object = object_with_slots(&mut e, &[(0xdc, 0)]);
        e.register_double(FIND_OBJECT_BY_NAME, move |_, a| {
            ret(if a[1] == 0x0118_8c10 { bone_object } else { 0 })
        });
        returns(&mut e, ACTOR_ROOT, a.root);
        log(&mut e);
        let result = biped_anim_load_and_attach_add_on(
            &mut e,
            Ptr::NULL,
            Ptr::new(a.model),
            3,
            Ptr::new(a.requester),
            Ptr::NULL,
        );
        assert_eq!(result, Ptr::new(a.copy));
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, ACTOR_ROOT), vec![vec![a.requester]]);
        assert!(calls_to(&calls, SWAP_PLATFORM_TEXTURES).is_empty());
        let hang = slot_target(&e, bone_object, 0xdc);
        assert_eq!(calls_to(&calls, hang), vec![vec![bone_object, a.copy, 1]]);
        // The root's search used the bone name of the table.
        let searched: Vec<_> = calls_to(&calls, FIND_OBJECT_BY_NAME)
            .into_iter()
            .filter(|words| words[0] == a.root)
            .collect();
        assert_eq!(searched, vec![vec![a.root, 0x0118_8c10, 1]]);
    }

    #[test]
    fn a_skinned_add_on_is_logged_and_not_attached() {
        let mut e = engine();
        let a = add_on(&mut e);
        returns(&mut e, FIND_SKINNED_NODE, 1);
        log(&mut e);
        let result = biped_anim_load_and_attach_add_on(
            &mut e,
            Ptr::new(a.form),
            Ptr::new(a.model),
            3,
            Ptr::new(a.requester),
            Ptr::new(a.root),
        );
        assert_eq!(result, Ptr::new(a.copy));
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, LOG),
            vec![vec![ADD_ON_SKINNED_FORMAT, name_of(a.copy)]]
        );
        assert!(calls_to(&calls, SET_MOTION).is_empty());
        let hang = slot_target(&e, a.root, 0xdc);
        assert!(calls_to(&calls, hang).is_empty());
        assert_eq!(calls_to(&calls, UPDATE_PROPERTIES), vec![vec![a.copy]]);
    }

    #[test]
    fn a_morphing_add_on_is_deep_copied() {
        let mut e = engine();
        let a = add_on(&mut e);
        returns(&mut e, HAS_MORPHER_CONTROLLER, 1);
        returns(&mut e, DEEP_COPY_SAME_TEXTURES, a.copy);
        log(&mut e);
        let result = biped_anim_load_and_attach_add_on(
            &mut e,
            Ptr::new(a.form),
            Ptr::new(a.model),
            0xffff_ffff,
            Ptr::new(a.requester),
            Ptr::new(a.root),
        );
        assert_eq!(result, Ptr::new(a.copy));
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, DEEP_COPY_SAME_TEXTURES).len(), 1);
        assert!(calls_to(&calls, NI_OBJECT_CLONE).is_empty());
    }

    #[test]
    fn an_add_on_needs_a_model_with_a_path_a_requester_and_a_root() {
        let mut e = engine();
        let a = add_on(&mut e);
        log(&mut e);
        let go = |e: &mut Engine, model: u32, requester: u32, root: u32| {
            biped_anim_load_and_attach_add_on(
                e,
                Ptr::new(a.form),
                Ptr::new(model),
                3,
                Ptr::new(requester),
                Ptr::new(root),
            )
        };
        assert_eq!(go(&mut e, 0, a.requester, a.root), Ptr::NULL);
        assert_eq!(go(&mut e, a.model, 0, a.root), Ptr::NULL);
        let empty = text(&mut e, "");
        let no_path = object_with_slots(&mut e, &[(0x14, empty)]);
        assert_eq!(go(&mut e, no_path, a.requester, a.root), Ptr::NULL);
        let nothing = object_with_slots(&mut e, &[(0x14, 0)]);
        assert_eq!(go(&mut e, nothing, a.requester, a.root), Ptr::NULL);
        // No root of its own: the actor has no 3D.
        returns(&mut e, ACTOR_ROOT, 0);
        assert_eq!(go(&mut e, a.model, a.requester, 0), Ptr::NULL);
        assert!(calls_to(&log(&mut e), LOAD_FILE).is_empty());
    }

    #[test]
    fn add_on_nodes_are_added_when_the_extra_data_passes_the_test() {
        let mut e = engine();
        returns(&mut e, ADDON_KEY, 0x2468);
        returns(&mut e, GET_EXTRA_DATA, 0x8000);
        returns(&mut e, ADDON_EXTRA_CHECK, 1);
        log(&mut e);
        fn_004af240(&mut e, Ptr::new(0xf11e), Ptr::new(0xc10e), 3, 0xabc);
        assert_eq!(
            log(&mut e),
            vec![
                (ADDON_KEY, vec![]),
                (GET_EXTRA_DATA, vec![0xf11e, 0x2468]),
                (ADDON_EXTRA_CHECK, vec![0x8000]),
                (ADD_ADDON_NODES, vec![0xc10e])
            ]
        );
        returns(&mut e, ADDON_EXTRA_CHECK, 0);
        fn_004af240(&mut e, Ptr::new(0xf11e), Ptr::new(0xc10e), 3, 0xabc);
        assert!(calls_to(&log(&mut e), ADD_ADDON_NODES).is_empty());
        returns(&mut e, GET_EXTRA_DATA, 0);
        fn_004af240(&mut e, Ptr::new(0xf11e), Ptr::new(0xc10e), 3, 0xabc);
        assert!(calls_to(&log(&mut e), ADDON_EXTRA_CHECK).is_empty());
    }

    #[test]
    fn the_lighting_property_needs_a_geometry_and_a_value_between_8_and_12() {
        let mut e = engine();
        let geometry = geometry_object(&mut e, &[]);
        let plain = object_with_slots(&mut e, &[(0x18, 0)]);
        returns(&mut e, GET_PROPERTY, 0x6a6a);
        returns(&mut e, PROPERTY_WORLDSPACE, 10);
        log(&mut e);
        assert_eq!(
            biped_anim_get_lighting_property(&mut e, Ptr::new(geometry)),
            Ptr::new(0x6a6a)
        );
        // Types 2 and 5 are fetched first, then 3.
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, GET_PROPERTY),
            vec![vec![geometry, 2], vec![geometry, 5], vec![geometry, 3]]
        );
        assert_eq!(
            biped_anim_get_lighting_property(&mut e, Ptr::new(plain)),
            Ptr::NULL
        );
        assert_eq!(
            biped_anim_get_lighting_property(&mut e, Ptr::NULL),
            Ptr::NULL
        );
        for (value, expected) in [(7, 0), (8, 0x6a6a), (12, 0x6a6a), (13, 0)] {
            returns(&mut e, PROPERTY_WORLDSPACE, value);
            assert_eq!(
                biped_anim_get_lighting_property(&mut e, Ptr::new(geometry)),
                Ptr::new(expected)
            );
        }
        returns(&mut e, GET_PROPERTY, 0);
        assert_eq!(
            biped_anim_get_lighting_property(&mut e, Ptr::new(geometry)),
            Ptr::NULL
        );
    }

    #[test]
    fn two_constants_give_the_property_types() {
        let mut e = engine();
        assert_eq!(fn_004af340(&mut e), 5);
        assert_eq!(fn_004af350(&mut e), 2);
    }

    /// The skin part names, as real texts, and a prefix compare that works.
    fn skin_names(e: &mut Engine) {
        for (index, name) in ["UpperBody", "LeftHand", "RightHand", "Head", "Body"]
            .into_iter()
            .enumerate()
        {
            let address = text(e, name);
            e.mem.set_u32(SKIN_PART_NAMES + 4 * index as u32, address);
        }
        e.register(TEXT_LENGTH, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        e.register(COMPARE_PREFIX, |e, a| {
            let left = e.mem.cstr(a[0]);
            let right = e.mem.cstr(a[1]);
            let count = a[2] as usize;
            let equal = left.len() >= count
                && right.len() >= count
                && left[..count].eq_ignore_ascii_case(&right[..count]);
            ret(if equal { 0 } else { 1 })
        });
        // The node's own address is its name text.
        e.register(NAME_FIELD, |_, a| ret(a[0]));
        e.register(NAME_TEXT, |_, a| ret(a[0]));
    }

    #[test]
    fn the_skin_slot_is_chosen_by_the_prefix_of_the_geometry_name() {
        let mut e = engine();
        skin_names(&mut e);
        returns(&mut e, PROPERTY_FLAG, 1);
        for (name, slot) in [
            ("UpperBody01", 2),
            ("lefthand", 2),
            ("RightHand:0", 3),
            ("Head", 4),
            ("body", 2),
        ] {
            let node = text(&mut e, name);
            assert_eq!(
                biped_anim_get_skin_biped_object(&mut e, Ptr::new(node), Ptr::new(0x6a6a)),
                slot,
                "{name}"
            );
        }
        assert!(calls_to(&log(&mut e), LOG).is_empty());
    }

    #[test]
    fn an_unknown_skin_name_is_logged_and_gives_slot_2() {
        let mut e = engine();
        skin_names(&mut e);
        returns(&mut e, PROPERTY_FLAG, 1);
        let node = text(&mut e, "Elbow");
        let above = text(&mut e, "Above");
        returns(&mut e, NODE_PARENT, above);
        log(&mut e);
        assert_eq!(
            biped_anim_get_skin_biped_object(&mut e, Ptr::new(node), Ptr::new(0x6a6a)),
            2
        );
        assert_eq!(
            calls_to(&log(&mut e), LOG),
            vec![vec![BAD_SKIN_NAME_FORMAT, node, above]]
        );
    }

    #[test]
    fn a_skin_without_the_property_flag_has_no_slot() {
        let mut e = engine();
        skin_names(&mut e);
        let node = text(&mut e, "Head");
        returns(&mut e, PROPERTY_FLAG, 0);
        assert_eq!(
            biped_anim_get_skin_biped_object(&mut e, Ptr::new(node), Ptr::new(0x6a6a)),
            0xffff_ffff
        );
        assert_eq!(
            calls_to(&log(&mut e), PROPERTY_FLAG),
            vec![vec![0x6a6a, 10]]
        );
    }

    #[test]
    fn without_a_property_the_lighting_property_is_used() {
        let mut e = engine();
        skin_names(&mut e);
        // A geometry named Head with a lighting property in range.
        let node = geometry_object(&mut e, &[]);
        e.mem.set_cstr(node + 0x40, b"Head");
        returns(&mut e, GET_PROPERTY, 0x6a6a);
        returns(&mut e, PROPERTY_WORLDSPACE, 9);
        returns(&mut e, PROPERTY_FLAG, 1);
        e.register(NAME_FIELD, |_, a| ret(a[0] + 0x40));
        e.register(NAME_TEXT, |_, a| ret(a[0]));
        assert_eq!(
            biped_anim_get_skin_biped_object(&mut e, Ptr::new(node), Ptr::NULL),
            4
        );
        returns(&mut e, GET_PROPERTY, 0);
        assert_eq!(
            biped_anim_get_skin_biped_object(&mut e, Ptr::new(node), Ptr::NULL),
            0xffff_ffff
        );
    }

    /// A geometry with a lighting property for `AdjustSkinComplexion`: the
    /// property's virtual slots 0xfc and 0x100 set a texture. The geometry's
    /// name is `Head`.
    fn complexion_scene(e: &mut Engine) -> (Ptr<BipedAnim>, u32, u32, u32) {
        skin_names(e);
        let this = biped(e);
        e.set(this, BipedAnim::m_pRequester, Ptr::new(0x0abc_0000));
        let node = geometry_object(e, &[]);
        e.mem.set_cstr(node + 0x40, b"Head");
        e.register(NAME_FIELD, |_, a| ret(a[0] + 0x40));
        e.register(NAME_TEXT, |_, a| ret(a[0]));
        let property = object_with_slots(e, &[(0xfc, 0), (0x100, 0)]);
        returns(e, GET_PROPERTY, property);
        returns(e, PROPERTY_WORLDSPACE, 9);
        returns(e, PROPERTY_FLAG, 1);
        // The requester's base form is an NPC.
        let npc = block(e, 0x300);
        returns(e, REFERENCE_FORM, 0x3030);
        returns(e, RT_DYNAMIC_CAST, npc);
        // The race texture handles are filled by `GetBodyTexture`.
        e.register_double(GET_BODY_TEXTURE, |e, a| {
            e.mem.set_u32(a[1], 0x1a1a);
            e.mem.set_u32(a[2], 0x2b2b);
            ret(1)
        });
        returns(e, READ_WORD_PLUS_4, 0x6060);
        (this, node, property, npc)
    }

    #[test]
    fn a_skin_gets_its_race_textures_and_the_shader_flag() {
        let mut e = engine();
        let (this, node, property, npc) = complexion_scene(&mut e);
        let texture_object = block(&mut e, 0x80);
        returns(&mut e, CAST_TO_CLASS, texture_object);
        e.register(TEXTURE_PATH_FIELD, |_, a| ret(a[0] + 0x34));
        let owner = block(&mut e, 0x1200);
        e.mem.set_u32(owner + 0x11a0, 0x4e4e);
        e.mem.set_u32(TEXTURE_OWNER, owner);
        log(&mut e);
        biped_anim_adjust_skin_complexion(&mut e, this, Ptr::new(node), 1);
        let calls = log(&mut e);
        let body = calls_to(&calls, GET_BODY_TEXTURE);
        assert_eq!(body[0][0], 0x6060);
        assert_eq!(body[0][3..], [npc, 4, 1]);
        assert_eq!(
            calls_to(&calls, GET_FACE_COORD).len(),
            1,
            "the face coordinates are read"
        );
        let set_base = slot_target(&e, property, 0xfc);
        let set_normal = slot_target(&e, property, 0x100);
        assert_eq!(
            calls_to(&calls, set_base),
            vec![vec![property, 0, 0x1a1a], vec![property, 1, 0x2b2b]]
        );
        // The filter flag of the texture: the cast object gives 1; the path is
        // that of the cast object (+0x34 is the field, its text the same).
        assert_eq!(
            calls_to(&calls, MODIFIED_TEXTURE_FILENAME)[0][1..],
            [texture_object + 0x34, NORMAL_MAP_SUFFIX, 1]
        );
        assert_eq!(
            calls_to(&calls, CAST_TO_CLASS)
                .iter()
                .filter(|words| words[0] == TEXTURE_CLASS)
                .count(),
            1
        );
        // The second texture comes from the owner's NiPointer.
        assert_eq!(
            calls_to(&calls, set_normal).last().unwrap(),
            &vec![property, 1, 0x4e4e]
        );
        assert_eq!(
            calls_to(&calls, SET_SHADER_FLAG),
            vec![vec![property, 10, 1]]
        );
        assert_eq!(calls_to(&calls, VECTOR_DESTRUCT).len(), 1);
    }

    #[test]
    fn a_normal_map_found_for_the_base_texture_is_set_too() {
        let mut e = engine();
        let (this, node, property, _) = complexion_scene(&mut e);
        let texture_object = block(&mut e, 0x80);
        returns(&mut e, CAST_TO_CLASS, texture_object);
        e.register(MODIFIED_TEXTURE_FILENAME, |e, a| {
            e.mem.set_cstr(a[0], b"x_n.dds");
            Ret::default()
        });
        e.register_double(GET_TEXTURE, |e, a| {
            e.mem.set_u32(a[2], 0x3c3c);
            Ret::default()
        });
        e.register(READ_WORD, |e, a| ret(e.mem.u32(a[0])));
        log(&mut e);
        biped_anim_adjust_skin_complexion(&mut e, this, Ptr::new(node), 0);
        let calls = log(&mut e);
        let texture = calls_to(&calls, GET_TEXTURE);
        assert_eq!(texture[0][1], 1, "mipmaps stay on");
        assert_eq!(texture[0][3..], [1, 0]);
        let set_normal = slot_target(&e, property, 0x100);
        assert_eq!(calls_to(&calls, set_normal)[0], vec![property, 0, 0x3c3c]);
        assert_eq!(calls_to(&calls, GET_BODY_TEXTURE)[0][5], 0);
    }

    #[test]
    fn a_skin_without_race_textures_is_logged_with_the_race_and_sex_names() {
        let mut e = engine();
        let (this, node, property, npc) = complexion_scene(&mut e);
        returns(&mut e, GET_BODY_TEXTURE, 0);
        returns(&mut e, ACTOR_BASE_SEX, 1);
        e.register(IDENTITY_TEXT, |_, a| ret(a[0]));
        returns(&mut e, RACE_NAME_TEXT, 0x8181);
        let sex_name = text(&mut e, "Female");
        e.mem.set_u32(SEX_NAMES + 4, sex_name);
        log(&mut e);
        biped_anim_adjust_skin_complexion(&mut e, this, Ptr::new(node), 0);
        let calls = log(&mut e);
        let slot_name = e.mem.u32(SLOT_NAMES + 4 * 4);
        assert_eq!(
            calls_to(&calls, ACTOR_BASE_SEX),
            vec![vec![npc, 0, slot_name]]
        );
        assert_eq!(
            calls_to(&calls, LOG),
            vec![vec![
                MISSING_RACE_TEXTURE_FORMAT,
                sex_name,
                slot_name,
                0x8181
            ]]
        );
        let set_base = slot_target(&e, property, 0xfc);
        assert!(calls_to(&calls, set_base).is_empty());
        assert!(calls_to(&calls, SET_SHADER_FLAG).is_empty());
        assert_eq!(calls_to(&calls, VECTOR_DESTRUCT).len(), 1);
    }

    #[test]
    fn a_skin_without_an_npc_still_gets_its_textures() {
        let mut e = engine();
        let (this, node, property, _) = complexion_scene(&mut e);
        returns(&mut e, RT_DYNAMIC_CAST, 0);
        returns(&mut e, GET_BODY_TEXTURE, 0);
        log(&mut e);
        biped_anim_adjust_skin_complexion(&mut e, this, Ptr::new(node), 0);
        let calls = log(&mut e);
        assert!(calls_to(&calls, GET_FACE_COORD).is_empty());
        assert!(calls_to(&calls, LOG).is_empty());
        assert_eq!(
            calls_to(&calls, SET_SHADER_FLAG),
            vec![vec![property, 10, 1]]
        );
    }

    #[test]
    fn a_node_without_a_property_or_a_skin_slot_is_left_alone() {
        let mut e = engine();
        let (this, node, _, _) = complexion_scene(&mut e);
        returns(&mut e, GET_PROPERTY, 0);
        log(&mut e);
        biped_anim_adjust_skin_complexion(&mut e, this, Ptr::new(node), 0);
        assert!(calls_to(&log(&mut e), VECTOR_CONSTRUCT).is_empty());
        returns(&mut e, PROPERTY_FLAG, 0);
        let (this, node, _, _) = complexion_scene(&mut e);
        returns(&mut e, PROPERTY_FLAG, 0);
        biped_anim_adjust_skin_complexion(&mut e, this, Ptr::new(node), 0);
        assert!(calls_to(&log(&mut e), VECTOR_CONSTRUCT).is_empty());
    }

    #[test]
    fn two_small_texture_helpers_are_neutral() {
        let mut e = engine();
        let object = block(&mut e, 0x80);
        e.mem.set_u8(object + 0x43, 0x99);
        assert!(!fn_004af860(&mut e, Ptr::new(object)));
        fn_004af880(&mut e, Ptr::new(object));
        assert_eq!(e.mem.u8(object + 0x43), 0x99);
        assert!(!e.call(0x004a_f860, &args![object]).bool());
    }

    #[test]
    fn the_owner_texture_is_read_from_the_owner_or_a_fresh_pointer() {
        let mut e = engine();
        returns(&mut e, READ_WORD, 0x4d4d);
        e.mem.set_u32(TEXTURE_OWNER, 0x7070);
        log(&mut e);
        assert_eq!(fn_004af8a0(&mut e), 0x4d4d);
        assert_eq!(log(&mut e), vec![(READ_WORD, vec![0x7070 + 0x11a0])]);
        e.mem.set_u32(TEXTURE_OWNER, 0);
        e.register(READ_WORD, |e, a| ret(e.mem.u32(a[0])));
        assert_eq!(fn_004af8a0(&mut e), 0);
        let calls = log(&mut e);
        assert_eq!(
            addresses(&calls),
            vec![NI_POINTER_INIT, READ_WORD, NI_POINTER_RELEASE]
        );
    }

    #[test]
    fn the_first_person_model_path_replaces_the_extension() {
        let mut e = engine();
        let item = object_with_slots(&mut e, &[(0x14, 0xabc0)]);
        e.register(STRING_COPY, |e, a| {
            e.mem.set_cstr(a[0], b"armor\\helmet.nif");
            ret(0)
        });
        e.register(FIND_LAST_CHAR, |e, a| {
            let text = e.mem.cstr(a[0]);
            ret(text
                .iter()
                .rposition(|&c| c == b'.')
                .map_or(0, |i| a[0] + i as u32))
        });
        e.register(STRING_APPEND, |e, a| {
            e.mem.set_cstr(a[0], b".1st.nif");
            ret(0)
        });
        returns(&mut e, FILE_EXISTS, 1);
        let buffer = block(&mut e, 0x104);
        log(&mut e);
        assert!(fn_004af950(
            &mut e,
            Ptr::NULL,
            Ptr::new(item),
            Ptr::new(buffer)
        ));
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, STRING_COPY),
            vec![vec![buffer, 0x104, 0xabc0]]
        );
        assert_eq!(calls_to(&calls, FIND_LAST_CHAR), vec![vec![buffer, 0x2e]]);
        // The text was cut at the dot and the suffix appended with the room left.
        assert_eq!(e.mem.u8(buffer + 12), b'.');
        assert_eq!(
            calls_to(&calls, STRING_APPEND),
            vec![vec![buffer + 12, 0x104 - 12, FIRST_PERSON_SUFFIX]]
        );
        let format = calls_to(&calls, FORMAT_INTO);
        assert_eq!(
            format[0][1..],
            [0x104, DATA_PATH_FORMAT, MESHES_FOLDER, buffer]
        );
        assert_eq!(
            calls_to(&calls, FILE_EXISTS),
            vec![vec![format[0][0], 0, 6, 1]]
        );
        returns(&mut e, FILE_EXISTS, 0);
        assert!(!fn_004af950(
            &mut e,
            Ptr::NULL,
            Ptr::new(item),
            Ptr::new(buffer)
        ));
    }

    #[test]
    fn a_path_without_an_extension_has_no_first_person_model() {
        let mut e = engine();
        let item = object_with_slots(&mut e, &[(0x14, 0xabc0)]);
        let buffer = block(&mut e, 0x104);
        log(&mut e);
        assert!(!fn_004af950(
            &mut e,
            Ptr::NULL,
            Ptr::new(item),
            Ptr::new(buffer)
        ));
        assert_eq!(addresses(&log(&mut e))[1..], [STRING_COPY, FIND_LAST_CHAR]);
    }

    #[test]
    fn a_path_entry_is_set_and_its_address_returned() {
        let mut e = engine();
        log(&mut e);
        assert_eq!(
            fn_004afa20(&mut e, Ptr::NULL, 2, Ptr::new(0xabc0)),
            PATH_TABLE + 0x30
        );
        assert_eq!(
            log(&mut e),
            vec![(SET_PATH_ENTRY, vec![PATH_TABLE + 0x30, 0xabc0])]
        );
    }

    #[test]
    fn the_alternative_path_entry_is_the_part_itself_unless_a_first_person_model_exists() {
        let mut e = engine();
        let item = object_with_slots(&mut e, &[(0x14, 0xabc0)]);
        // The part already is the entry of the slot.
        log(&mut e);
        assert_eq!(
            fn_004afa50(&mut e, Ptr::NULL, 2, Ptr::new(PATH_TABLE + 0x30)),
            PATH_TABLE + 0x30
        );
        assert!(log(&mut e).is_empty());
        // No first person model (no extension): the part.
        assert_eq!(fn_004afa50(&mut e, Ptr::NULL, 2, Ptr::new(item)), item);
        // A first person model: the entry, set from the built path.
        e.register(FIND_LAST_CHAR, |_, a| ret(a[0] + 5));
        returns(&mut e, FILE_EXISTS, 1);
        log(&mut e);
        assert_eq!(
            fn_004afa50(&mut e, Ptr::NULL, 2, Ptr::new(item)),
            PATH_TABLE + 0x30
        );
        let calls = log(&mut e);
        let set = calls_to(&calls, SET_PATH_ENTRY);
        assert_eq!(set[0][0], PATH_TABLE + 0x30);
        assert_eq!(set[0][1], calls_to(&calls, STRING_COPY)[0][0]);
    }

    #[test]
    fn a_text_is_lowered_with_room_for_its_length_plus_one() {
        let mut e = engine();
        let holder = block(&mut e, 8);
        log(&mut e);
        fn_004afad0(&mut e, Ptr::new(holder));
        assert!(log(&mut e).is_empty());
        e.mem.set_u32(holder, 0xabc0);
        e.mem.set_u16(holder + 4, 6);
        returns(&mut e, STRLWR_S, 0);
        fn_004afad0(&mut e, Ptr::new(holder));
        assert_eq!(log(&mut e), vec![(STRLWR_S, vec![0xabc0, 7])]);
    }

    #[test]
    fn the_lowering_wrapper_passes_both_words_on() {
        let mut e = engine();
        returns(&mut e, STRLWR_S, 22);
        log(&mut e);
        assert_eq!(fn_004afb00(&mut e, 0xabc0, 9), 22);
        assert_eq!(log(&mut e), vec![(STRLWR_S, vec![0xabc0, 9])]);
    }

    #[test]
    fn the_clone_map_and_process_map_constructors_store_their_vtables() {
        let mut e = engine();
        let clone_map = block(&mut e, 0x10);
        let process_map = block(&mut e, 0x10);
        log(&mut e);
        assert_eq!(
            fn_004afb20(&mut e, Ptr::new(clone_map), 0x101),
            Ptr::new(clone_map)
        );
        assert_eq!(e.mem.u32(clone_map), CLONE_MAP_VTABLE);
        assert_eq!(
            fn_004afb50(&mut e, Ptr::new(process_map), 0x101),
            Ptr::new(process_map)
        );
        assert_eq!(e.mem.u32(process_map), PROCESS_MAP_VTABLE);
        assert_eq!(
            log(&mut e),
            vec![
                (FN_004AFF30, vec![clone_map, 0x101]),
                (FN_004B0030, vec![process_map, 0x101])
            ]
        );
    }

    #[test]
    fn the_queue_lock_is_the_recursive_lock_at_0x20() {
        let mut e = engine();
        log(&mut e);
        fn_004afb80(&mut e, Ptr::new(0x4000));
        fn_004afba0(&mut e, Ptr::new(0x4000));
        assert_eq!(
            log(&mut e),
            vec![
                (RECURSIVE_LOCK_ENTER, vec![0x4020, 0]),
                (RECURSIVE_LOCK_LEAVE, vec![0x4020])
            ]
        );
    }

    #[test]
    fn the_clone_map_is_freed_only_when_the_flag_says_so() {
        let mut e = engine();
        log(&mut e);
        assert_eq!(
            ni_t_pointer_map_object_object_scalar_deleting_destructor(&mut e, Ptr::new(0x4000), 0),
            Ptr::new(0x4000)
        );
        assert_eq!(log(&mut e), vec![(FN_004AFFA0, vec![0x4000])]);
        ni_t_pointer_map_object_object_scalar_deleting_destructor(&mut e, Ptr::new(0x4000), 3);
        assert_eq!(
            log(&mut e),
            vec![(FN_004AFFA0, vec![0x4000]), (OPERATOR_DELETE, vec![0x4000])]
        );
    }

    #[test]
    fn the_process_map_is_freed_only_when_the_flag_says_so() {
        let mut e = engine();
        log(&mut e);
        assert_eq!(
            ni_t_pointer_map_object_bool_scalar_deleting_destructor(&mut e, Ptr::new(0x4000), 2),
            Ptr::new(0x4000)
        );
        assert_eq!(log(&mut e), vec![(FN_004B00A0, vec![0x4000])]);
        ni_t_pointer_map_object_bool_scalar_deleting_destructor(&mut e, Ptr::new(0x4000), 1);
        assert_eq!(
            log(&mut e),
            vec![(FN_004B00A0, vec![0x4000]), (OPERATOR_DELETE, vec![0x4000])]
        );
    }

    #[test]
    fn adding_to_the_array_appends_at_the_current_count() {
        let mut e = engine();
        let array = block(&mut e, 0x20);
        e.mem.set_u16(array + 0x0a, 4);
        log(&mut e);
        fn_004afc50(&mut e, Ptr::new(array), Ptr::new(0x4141));
        assert_eq!(log(&mut e), vec![(FN_004B02D0, vec![array, 4, 0x4141])]);
    }

    /// Stands in for the `NiPointer` comparison and copy the array code uses.
    fn pointer_doubles(e: &mut Engine) {
        e.register(NI_POINTER_DIFFERS, |e, a| {
            ret((e.mem.u32(a[0]) != e.mem.u32(a[1])) as u32)
        });
        e.register(NI_POINTER_COPY, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            ret(a[0])
        });
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
    }

    /// A node array and its block of `items.len()` words, with the given
    /// non-null count.
    fn node_array(e: &mut Engine, items: &[u32], element_count: u16) -> (u32, u32) {
        let array = block(e, 0x10);
        let data = block(e, 4 * items.len() as u32);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *item);
        }
        e.mem.set_u32(array + 4, data);
        e.mem.set_u16(array + 8, items.len() as u16);
        e.mem.set_u16(array + 0x0a, items.len() as u16);
        e.mem.set_u16(array + 0x0c, element_count);
        (array, data)
    }

    #[test]
    fn compact_does_nothing_when_no_element_is_null() {
        let mut e = engine();
        pointer_doubles(&mut e);
        let (array, data) = node_array(&mut e, &[0x11, 0x22], 2);
        log(&mut e);
        fn_004afc80(&mut e, Ptr::new(array));
        assert_eq!(e.mem.u32(array + 4), data);
        assert!(calls_to(&log(&mut e), FN_004B0220).is_empty());
    }

    #[test]
    fn compact_moves_the_elements_into_a_block_of_exactly_their_count() {
        let mut e = engine();
        pointer_doubles(&mut e);
        let (array, data) = node_array(&mut e, &[0x11, 0, 0x22, 0], 2);
        log(&mut e);
        fn_004afc80(&mut e, Ptr::new(array));
        let new_data = e.mem.u32(array + 4);
        assert_ne!(new_data, data);
        assert_eq!(e.mem.u16(array + 8), 2);
        assert_eq!(e.mem.u16(array + 0x0a), 2);
        assert_eq!(e.mem.u32(new_data), 0x11);
        assert_eq!(e.mem.u32(new_data + 4), 0x22);
        // The old block was packed in place before it was copied and freed.
        assert_eq!(e.mem.u32(data), 0x11);
        assert_eq!(e.mem.u32(data + 4), 0x22);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, FN_004B0220), vec![vec![data]]);
        assert_eq!(calls_to(&calls, VECTOR_CONSTRUCT).len(), 1);
    }

    #[test]
    fn compact_of_an_array_with_no_element_leaves_a_null_block() {
        let mut e = engine();
        pointer_doubles(&mut e);
        let (array, data) = node_array(&mut e, &[0, 0, 0], 0);
        log(&mut e);
        fn_004afc80(&mut e, Ptr::new(array));
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u16(array + 8), 0);
        assert_eq!(e.mem.u16(array + 0x0a), 0);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, FN_004B0220), vec![vec![data]]);
        assert!(calls_to(&calls, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn update_size_drops_the_null_elements_at_the_end() {
        let mut e = engine();
        pointer_doubles(&mut e);
        let (array, _) = node_array(&mut e, &[0x11, 0x22, 0, 0], 2);
        fn_004afe50(&mut e, Ptr::new(array));
        assert_eq!(e.mem.u16(array + 0x0a), 2);
        let (all_null, _) = node_array(&mut e, &[0, 0], 0);
        fn_004afe50(&mut e, Ptr::new(all_null));
        assert_eq!(e.mem.u16(all_null + 0x0a), 0);
        let (full, _) = node_array(&mut e, &[0x11, 0x22], 2);
        fn_004afe50(&mut e, Ptr::new(full));
        assert_eq!(e.mem.u16(full + 0x0a), 2);
    }

    #[test]
    fn the_map_base_constructors_store_the_table_and_the_vtable() {
        let mut e = engine();
        e.register(NI_ALLOC, |e, a| ret(e.mem.alloc(a[0])));
        let clone_map = block(&mut e, 0x10);
        let process_map = block(&mut e, 0x10);
        e.mem.set_u32(clone_map + 0x0c, 9);
        log(&mut e);
        assert_eq!(
            fn_004aff30(&mut e, Ptr::new(clone_map), 0x25),
            Ptr::new(clone_map)
        );
        assert_eq!(e.mem.u32(clone_map), CLONE_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(clone_map + 4), 0x25);
        assert_eq!(e.mem.u32(clone_map + 0x0c), 0);
        let table = e.mem.u32(clone_map + 8);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, NI_ALLOC), vec![vec![0x94]]);
        assert_eq!(calls_to(&calls, MEMSET), vec![vec![table, 0, 0x94]]);
        fn_004b0030(&mut e, Ptr::new(process_map), 7);
        assert_eq!(e.mem.u32(process_map), PROCESS_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(process_map + 4), 7);
    }

    #[test]
    fn the_map_destructors_reset_the_vtable_then_remove_all_and_destroy_the_base() {
        let mut e = engine();
        let map = block(&mut e, 0x10);
        log(&mut e);
        fn_004affa0(&mut e, Ptr::new(map));
        assert_eq!(e.mem.u32(map), CLONE_MAP_VTABLE);
        assert_eq!(
            log(&mut e),
            vec![
                (MAP_REMOVE_ALL, vec![map]),
                (CLONE_MAP_BASE_DESTRUCT, vec![map])
            ]
        );
        fn_004b00a0(&mut e, Ptr::new(map));
        assert_eq!(e.mem.u32(map), PROCESS_MAP_VTABLE);
        assert_eq!(
            log(&mut e),
            vec![
                (MAP_REMOVE_ALL, vec![map]),
                (PROCESS_MAP_BASE_DESTRUCT, vec![map])
            ]
        );
    }

    #[test]
    fn the_scalar_deleting_destructors_of_the_map_bases_free_on_bit_0() {
        let mut e = engine();
        log(&mut e);
        assert_eq!(
            ni_t_map_base_object_object_scalar_deleting_destructor(&mut e, Ptr::new(0x4000), 2),
            Ptr::new(0x4000)
        );
        assert_eq!(log(&mut e), vec![(CLONE_MAP_BASE_DESTRUCT, vec![0x4000])]);
        ni_t_map_base_object_object_scalar_deleting_destructor(&mut e, Ptr::new(0x4000), 1);
        assert_eq!(
            log(&mut e),
            vec![
                (CLONE_MAP_BASE_DESTRUCT, vec![0x4000]),
                (OPERATOR_DELETE, vec![0x4000])
            ]
        );
        ni_t_map_base_object_bool_scalar_deleting_destructor(&mut e, Ptr::new(0x4000), 0);
        assert_eq!(log(&mut e), vec![(PROCESS_MAP_BASE_DESTRUCT, vec![0x4000])]);
        ni_t_map_base_object_bool_scalar_deleting_destructor(&mut e, Ptr::new(0x4000), 3);
        assert_eq!(
            log(&mut e),
            vec![
                (PROCESS_MAP_BASE_DESTRUCT, vec![0x4000]),
                (OPERATOR_DELETE, vec![0x4000])
            ]
        );
    }

    #[test]
    fn allocate_stores_the_count_and_constructs_the_elements() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        log(&mut e);
        let first = fn_005e0ba0(&mut e, 3);
        assert_eq!(e.mem.u32(first.addr() - 4), 3);
        let calls = log(&mut e);
        assert_eq!(calls_to(&calls, OPERATOR_NEW), vec![vec![16]]);
        assert_eq!(
            calls_to(&calls, VECTOR_CONSTRUCT),
            vec![vec![
                first.addr(),
                4,
                3,
                NI_POINTER_ELEMENT_INIT,
                NI_POINTER_RELEASE
            ]]
        );
    }

    #[test]
    fn allocate_asks_for_all_ones_when_the_size_overflows_and_fails_on_null() {
        let mut e = engine();
        log(&mut e);
        assert_eq!(fn_005e0ba0(&mut e, 0x4000_0000), Ptr::new(0));
        assert_eq!(fn_005e0ba0(&mut e, 0x3fff_ffff), Ptr::new(0));
        let calls = log(&mut e);
        assert_eq!(
            calls_to(&calls, OPERATOR_NEW),
            vec![vec![u32::MAX], vec![u32::MAX]]
        );
        assert!(calls_to(&calls, VECTOR_CONSTRUCT).is_empty());
    }

    #[test]
    fn the_parent_bone_is_the_node_of_the_indexed_bone() {
        let mut e = engine();
        let anim = biped(&mut e);
        e.mem.set_u32(bone(anim, 3).addr() + 4, 0x5555);
        assert_eq!(
            biped_anim_get_parent_bone(&mut e, anim, 3),
            Ptr::new(0x5555)
        );
        assert_eq!(biped_anim_get_parent_bone(&mut e, anim, 0), Ptr::new(0));
    }
}
