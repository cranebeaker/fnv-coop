//! `fallout shared/teswater.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is `TESWaterSystem` (the singleton `TES::pWaterSystem` points
//! at), `PlaceableWaterGroup` and the small accessors the water code is
//! built from. It has 166 functions, `004e21b0` to `004edd80`.
//!
//! Translated: all 166 functions (the two library ones, `00f426e0` and
//! `00f46cc0`, are not part of the queue). The last session (`004edb60` to
//! `004edd80`) did the base constructor of the wading map, two small
//! two-word constructors (bodies `004edf70` and `004edfe0`, called by
//! address) and the three `NiTMapBase` deleting destructors; `004ed800`,
//! `004eba20` and `004eba90` still reach `004edb60`, `004edc90`/`004edcf0`
//! and `004edc60`/`004edcc0` by address.
//! Fourth session (`004eb540` to `004eda60`): the world and sky reflection
//! finish, the depth setup and render of a group, the clip-plane setup
//! (`004ecef0`, with the D3DX math), the group constructor and destructor and
//! the small accessors; the new callees are under "fourth session". Functions
//! this unit calls in other places of the same file are now called directly
//! (`REFLECTION_PLANE_SETUP`, `INITIALIZE_GREY_TEXTURE`, `FLOAT_SETTING_SET`
//! and `GROUP_DESTRUCT` stay as constants for the earlier call sites, which
//! reach them by address); the layouts of the NiCamera members `+0xdc`/`+0x100`
//! are not what the earlier constant names (`CAMERA_PLANES_ADDRESS`,
//! `CAMERA_FRUSTUM_ADDRESS`) say: going by the camera's size (0x114), `+0xdc`
//! is probably the `NiFrustum` (0x1c bytes, near plane at `+0xf8`, the `float` at `+0xfc`
//! the far/near ratio) and `+0x100` the viewport (not checked).
//! Third session (`004e8000` to `004eb510`): the wading water (ripples,
//! displacement simulation, wading camera), the three reflection setups and
//! the interior finish; the layouts of `WadingWaterData` and the extra
//! `WaterShaderProperty` fields are above, the new callee constants under
//! "third session". A few reads the game does unguarded (an empty group list in
//! `004e8ec0`, no world space in `004eaa00`/`004eaf80`) fault here as there.
//!
//! Notes for the next session:
//! - `00559450` is both `NiPointer<T>::operator T*` and
//!   `NiTPointerListBase::GetHeadPos` (it returns `*this`). A list node is
//!   `{next, prev, item}`: `007b52d0` returns `next`, `006317a0` the address
//!   of `item`. `for_each_list_item` walks a list the way every loop in the
//!   unit does (the next position first, then the item).
//! - `TESWaterSystem` and `PlaceableWaterGroup` keep the Xbox PDB offsets on
//!   the PC. `TESWaterForm` fields from `spNoiseNormalMap` on are 0x10 lower than
//!   the Xbox PDB says, and `WaterShaderProperty` fields from `bFullReflections` on
//!   are 8 lower (checked against the colour, fog and texture fields the
//!   update code moves between them). The layouts below list only the fields
//!   translated code touches.
//! - The engine map names `005f36f0` `ActorMover::GetPreferredMoveMode`; its
//!   body returns `TES::pInteriorCell` (`TES + 0x34`) and the water code
//!   uses it as "the player is in an interior" (`in_interior`).
//! - Water state kept in globals (`0x011c7a58..0x011c7a74`, the
//!   rendered-texture slots `0x011c7b64`, `0x011c7b68`, `0x011c7ad4` and
//!   `0x011c7c2c`) is named for the Xbox PDB static it matches by use; the
//!   INI settings are named by the string their static initializer
//!   registers.
//! - Second session (`004e4730` to `004e7ff0`): the first session's call
//!   sites of `004e4730`, `004e56c0`, `004e58a0` and `004e62e0` still call
//!   them by address (through the function table), which keeps the call logs
//!   its tests read; the new functions call each other directly.
//! - `PlaceableWaterGroup` also has `RefractWaterPlane` (`+0x14`) and
//!   `spGroupReflectionMap` (`+0x54`); `TESWaterSystem` has the four rendered
//!   texture slots at `+0x8`, `+0x10`, `+0x14`, `+0x18`, `WadingWaterMap`
//!   (`+0x7c`) and `WaterSound` (`+0x8c`, a `BSSoundHandle`).
//! - The PC's `TESObjectREFR` virtual table is shifted by 4 against the Xbox
//!   PDB somewhere between `IsActor` (`+0x100` on both) and `Get3D`
//!   (`+0x1d0` here, `+0x1cc` there).
//! - `004e5fe0` hangs in the game when a water zone reference of the object
//!   is in a group's list (it does not advance to the next group); the
//!   translation panics there with that explanation.
//! - Accessors that spill `ECX` but never read it are taken to be member
//!   functions that ignore `this`; their first parameter is `_unused_0`.
//! - The compiler's exception-unwinding frames (`FS:[0]` chains) and the
//!   stack-protector cookie of `UpdatePlaceableWater` are not translated.
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results. The translations compute in `f64` and round to `f32` where the
//! code stores a `float` (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------------
// Callees in other units, by address.

/// `*this`: `NiPointer<T>::operator T*` and `NiTPointerListBase::GetHeadPos`
/// (one folded body).
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `NiTPointerListBase::GetNextPos(this, position)`: the node's `next`.
const LIST_NEXT_POSITION: u32 = 0x007b_52d0;
/// `NiTPointerListBase::GetAt(this, position)`: the address of the node's
/// item (node + 8).
const LIST_ITEM_SLOT: u32 = 0x0063_17a0;
/// Whether a `NiTPointerList` is empty (its count at +8 is zero).
const LIST_IS_EMPTY: u32 = 0x0076_b610;
/// The count of a list or array (`this + 8`).
const LIST_COUNT: u32 = 0x0044_ddc0;
/// `NiTPointerListBase::RemoveAll` (the engine map's name for the body).
const LIST_REMOVE_ALL: u32 = 0x004e_d900;
/// `NiTMapBase::RemoveAll`.
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `NiPointer<T>::operator=(T*)`: `this` is the slot, the argument the new
/// pointer (releases the old object, references the new one).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer<T>::operator=(const NiPointer&)` (the engine map names the
/// instance `NiPointer<PathingDebugGeometryData>::operator_`): `this` is the
/// slot, the argument the address of the slot to copy from.
const NI_POINTER_ASSIGN_FROM: u32 = 0x006e_5cc0;
/// `SettingT<INISettingCollection>::GetValue`: the address of the setting's
/// value (setting + 4).
const SETTING_VALUE: u32 = 0x0040_8d60;
/// `TES::pInteriorCell` read through the `TES` object (`TES + 0x34`).
const TES_GET_INTERIOR_CELL: u32 = 0x005f_36f0;
/// `TESObjectREFR::pBaseForm` (+0x20).
const REFERENCE_BASE_FORM: u32 = 0x007a_f430;
/// `TESObjectREFR::pParentCell` (+0x40 on the PC).
const REFERENCE_PARENT_CELL: u32 = 0x008d_6f30;
/// `ExtraDataList` of a reference (`TESObjectREFR + 0x44`).
const REFERENCE_EXTRA_DATA_LIST: u32 = 0x005d_43c0;
/// `ExtraDataList::GetReflectedRefs` (Xbox PDB name).
const EXTRA_DATA_LIST_GET_REFLECTED_REFS: u32 = 0x0041_f140;
/// `true` when the cell's byte at +0x26 is 6 (`00450fd0` reads the byte).
const CELL_BYTE_IS_SIX: u32 = 0x0045_0ff0;
/// `TESObjectREFR` virtual at +0x1d0 (the Xbox PDB's `Get3D` is at +0x1cc):
/// the reference's 3D node (`NiAVObject*`), null when it has none.
const REFERENCE_GET_3D: u32 = 0x1d0;
/// `true` when `NiAVObject::m_uFlags & 1` (the node is culled).
const NODE_IS_CULLED: u32 = 0x0045_6610;
/// `NiAVObject::m_kWorld.m_Translate` (`this + 0x8c`).
const NODE_WORLD_TRANSLATE: u32 = 0x0045_bb80;
/// `NiAVObject::m_kWorld.m_Rotate` (`this + 0x68`, a 3x3 matrix).
const NODE_WORLD_ROTATE: u32 = 0x0046_1130;
/// `NiAVObject::GetProperty(this, type)` (Xbox PDB name).
const NODE_GET_PROPERTY: u32 = 0x00a5_9d30;
/// `[this + 0xc0]` of a water reference's 3D object, and the type id
/// (`+0x68`) of that object.
const NODE_OWNER: u32 = 0x0040_30b0;
const OWNER_TYPE: u32 = 0x008d_8520;
/// The owner type id the shader-property updates apply to.
const WATER_OWNER_TYPE: u32 = 0x0d;
/// `NiPoint4::NiPoint4(this, x, y, z, w)` (returns `this`).
const NI_POINT4_CONSTRUCT: u32 = 0x0041_4430;
/// `this[0] == 0 && this[1] == 0`.
const WORDS_ARE_ZERO: u32 = 0x0082_56d0;
/// Whether the form flags of a base object have a bit set: `this`, then the
/// mask.
const FORM_FLAG_TEST: u32 = 0x0045_2460;
/// The same with the mask `0x40000000`.
const FORM_FLAG_TEST_40000000: u32 = 0x0045_2440;

/// `TESWaterSystem` and water-group helpers of this unit. Those after
/// `004e4730` are translated below; the earlier call sites reach
/// `REFERENCE_IS_IN_RANGE`, `RELEASE_GROUP`, `FINISH_GROUP` and
/// `ADD_PLACEABLE_WATER_OV2` by address (see the module notes).
/// `this`, reference: the reference's 3D object used for its water shader
/// property, or null.
const WATER_REFERENCE_3D: u32 = 0x004e_8030;
/// `this`, reference, viewer: the range test `UpdatePlaceableWater` and
/// `UpdateLODWater` apply to a reference and the viewer node.
const REFERENCE_IS_IN_RANGE: u32 = 0x004e_62e0;
/// `this`, group, 0, 0: called for a group that is not rendered (and by the
/// LOD update while water is skipped).
const RELEASE_GROUP: u32 = 0x004e_58a0;
/// `this`, group: called for a rendered group once its passes were
/// scheduled.
const FINISH_GROUP: u32 = 0x004e_56c0;
/// `this`, group: called before `FINISH_GROUP` for a rendered group with
/// reflections when the player is in an interior.
const FINISH_GROUP_INTERIOR: u32 = 0x004e_b220;
/// `this`, viewer, group, flag: called for a group with reflections (the
/// flag is 0 for high detail or in an interior, else 1).
const SETUP_GROUP_REFLECTIONS: u32 = 0x004e_9d40;
/// `this`, viewer, group: called for a group that wants depth.
const SETUP_GROUP_DEPTH: u32 = 0x004e_bef0;
/// `this`, viewer: called when the world / sky reflections are on.
const SETUP_WORLD_REFLECTIONS: u32 = 0x004e_aa00;
const SETUP_SKY_REFLECTIONS: u32 = 0x004e_af80;
/// `this`: called when the world / sky reflections are on, after the groups
/// were scheduled.
const FINISH_WORLD_REFLECTIONS: u32 = 0x004e_b540;
const FINISH_SKY_REFLECTIONS: u32 = 0x004e_bbe0;
/// `this`, camera, group, first, stencil mask: called for every group whose
/// depth is rendered.
const RENDER_GROUP_DEPTH: u32 = 0x004e_cb60;
/// `this`, group, stencil mask: what follows a group's depth render.
const AFTER_GROUP_DEPTH: u32 = 0x004e_c800;
/// `cdecl(index, x, y, z, w)`: stores a four-float shader constant (the
/// depth pass sets index 0x1e to the viewer's position).
const VIEWER_POSITION_UPDATE: u32 = 0x004e_20c0;
/// Returns the byte at `0x011c7a59`; the water code skips its passes while it
/// is set.
const WATER_SKIPPED_FLAG: u32 = 0x004e_2180;

/// Renderer calls the passes make.
/// The `NiRenderer*` (`[0x011f9508]`) and its virtuals the depth pass uses.
const RENDERER: u32 = 0x004a_0e90;
const RENDERER_GET_CLEAR_COLOR: u32 = 0xb4;
const RENDERER_SET_CLEAR_COLOR: u32 = 0xac;
/// The `BSTextureManager*` (`[0x011f91a8]`) and
/// `BSTextureManager::ReturnRenderedTexture(this, texture)` (Xbox PDB name).
const TEXTURE_MANAGER: u32 = 0x004a_0ea0;
const RETURN_RENDERED_TEXTURE: u32 = 0x00b6_da10;
/// `BSRenderedTexture::StopOffscreen` (engine map name): `this` is the
/// texture; its result goes to `00b6b8d0(mode, result)`.
const RENDERED_TEXTURE_STOP: u32 = 0x00b6_b260;
const RENDER_TARGET_SET: u32 = 0x00b6_b8d0;
/// No arguments: runs after a group's depth effect.
const RENDER_TARGET_RESET: u32 = 0x00b6_b790;
/// `BSBatchRenderer::EndPass` (Xbox PDB name; static).
const BATCH_RENDERER_END_PASS: u32 = 0x00b9_88e0;
/// `cdecl(2, 0xff, stencil mask, 0)`.
const SET_STENCIL_STATE: u32 = 0x00b9_8180;
/// `ImageSpaceShaderParam` (0x8c bytes on the stack): constructor,
/// `InitConstantMap(this, 0, 1)`, `SetPixelConstant(this, index, x, y, z, w)`
/// and destructor.
const SHADER_PARAM_SIZE: u32 = 0x8c;
const SHADER_PARAM_CONSTRUCT: u32 = 0x00b8_a9e0;
const SHADER_PARAM_INIT_CONSTANT_MAP: u32 = 0x00b8_ab10;
const SHADER_PARAM_SET_PIXEL_CONSTANT: u32 = 0x00b8_a790;
const SHADER_PARAM_DESTRUCT: u32 = 0x00b8_a8e0;
/// `ImageSpaceManager::RenderEffect(this, effect, renderer, texture, param, 0)`.
const IMAGE_SPACE_RENDER_EFFECT: u32 = 0x00b8_c730;
/// `this`, 0x21, renderer, texture, noise map, 0, 1.
const IMAGE_SPACE_RENDER_NOISE: u32 = 0x00b9_75f0;
/// `this` (texture manager), renderer, 0x13, 0, 0, 0: creates the rendered
/// texture the water noise map is drawn into.
const CREATE_RENDERED_TEXTURE: u32 = 0x00b6_e110;
/// `this` (the flush object): runs when its `+0x2b8` byte is clear.
const FLUSH_OBJECT_FLUSH: u32 = 0x00b9_b7f0;
/// `this`, name, slot, 1, 0 (`TES::CreateTextureImage`, Xbox PDB name).
const CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
/// `TESWaterSystem::AddPlaceableWater_ov2` (Xbox PDB name): `this`,
/// reference, position, rotation matrix.
const ADD_PLACEABLE_WATER_OV2: u32 = 0x004e_4730;

/// `TESWaterForm` helpers.
/// Whether the form's `NoiseTexture` (`TESTexture` at +0x64, Xbox +0x74) has
/// a name (`0048cee0` on its string).
const WATER_FORM_HAS_NOISE_TEXTURE: u32 = 0x0058_0080;
/// The name of the form's noise texture (`00408da0` on `this + 0x64`).
const WATER_FORM_NOISE_NAME: u32 = 0x0058_00a0;
/// `this`, slot: `*slot = this->spNoiseTexture` (the form's `NiPointer` at
/// +0x17c, Xbox +0x18c), through `NI_POINTER_ASSIGN_FROM`.
const WATER_FORM_COPY_NOISE_TEXTURE: u32 = 0x0058_00e0;
/// `NiPointer<T>::NiPointer(this, pointer)` (stores the pointer and
/// references it) and `NiPointer<T>::~NiPointer`, for a temporary of 4 bytes.
const POINTER_TEMP_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_TEMP_DESTRUCT: u32 = 0x0045_cec0;
/// `NiTMapBase<TESWaterForm *, bool>::GetAt(this, key, bool *result)` and
/// `SetAt(this, key, value)` (the map is `WaterTypeUpdateMap`).
const WATER_TYPE_MAP_FIND: u32 = 0x0057_c850;
const WATER_TYPE_MAP_SET: u32 = 0x0084_d310;

/// Float helpers and the CRT.
/// `fmodf(a, b)` (cdecl): the CRT `fmod` (`00ec9130`) on two floats.
const FLOAT_REMAINDER: u32 = 0x004b_1520;
/// `max(a, b)` of two floats (cdecl): `b < a ? a : b`.
const FLOAT_MAX: u32 = 0x0040_4010;
/// `cdecl(byte)`: the float at `0x011f940c + 4 * (byte != 0)`.
const FLOAT_TABLE_ENTRY: u32 = 0x0045_2e70;
/// The CRT `ceil`, `cos` and `sin` (cdecl, one `double`).
const CRT_CEIL: u32 = 0x00ec_9e10;
const CRT_COS: u32 = 0x00ec_9f30;
const CRT_SIN: u32 = 0x00ec_a060;
/// `PlaneConstant`: the `float` at `this + 0xc` (ST0).
const PLANE_CONSTANT: u32 = 0x0084_d030;
/// `sprintf_s(buffer, size, format, ...)` and the message logger
/// `(format, ...)`.
const FORMATTED_PRINT: u32 = 0x0040_6d00;
const LOG_MESSAGE: u32 = 0x005b_5e40;

// Callees of the functions from `004e4730` on (second session).

/// `NiPoint3::NiPoint3(this, x, y, z)` (returns `this`).
const NI_POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// `NiPlane::NiPlane_ov3(this, normal, point)` (Xbox PDB name): a plane
/// through `point` with the given `normal` (the constant is their dot
/// product). Returns `this`.
const NI_PLANE_CONSTRUCT: u32 = 0x00a6_9990;
/// `NiMatrix3 * NiPoint3` (`this` is the matrix; then the output point and
/// the input point). Returns the output.
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
/// Scales an `NiPoint3` (`this`) to length one, or to zero when its length
/// is at most `1e-6`.
const POINT3_UNITIZE: u32 = 0x004a_0c10;
/// `cdecl(a, b, tolerance)`: whether every component of the `NiPoint3` at
/// `a` is within `tolerance` of the one at `b`.
const POINTS_NEAR: u32 = 0x0049_e2f0;
/// `cdecl(a, b, tolerance)`: `|a - b| <= tolerance` on floats.
const FLOATS_NEAR: u32 = 0x0049_e390;
/// Returns `this` (used on a `NiPlane` to take the address of its normal).
const ADDRESS_OF_THIS: u32 = 0x0068_15c0;
/// `float` `SettingT::GetValue`: the address of the value (`setting + 4`).
const SETTING_FLOAT_VALUE: u32 = 0x0040_3e20;
/// Integer `SettingT::GetValue`: the address of the value (`setting + 4`).
const SETTING_INT_VALUE: u32 = 0x0043_d4d0;
/// `TES::GetWorldSpace(this)` (Xbox PDB name): the current `TESWorldSpace*`.
const TES_GET_WORLD_SPACE: u32 = 0x004f_d3e0;
/// `TES::GetCurrentCell(this)` (Xbox PDB name).
const TES_GET_CURRENT_CELL: u32 = 0x0045_7070;
/// `TES::AddTempDebugObject(this, node, seconds)` (Xbox PDB name).
const TES_ADD_TEMP_DEBUG_OBJECT: u32 = 0x0045_8e20;
/// `this`, position (`NiPoint3*`), out height (`float*`): reads the land
/// height at the position; false when there is none.
const TES_GET_LAND_HEIGHT: u32 = 0x0045_72e0;
/// `TESWorldSpace` byte at `+0x4c`, bit `0x10`.
const WORLD_SPACE_FLAG_10: u32 = 0x0058_62c0;
/// `TESWorldSpace` water height (`float` at `+0x7c`).
const WORLD_SPACE_WATER_HEIGHT: u32 = 0x0045_cd80;
/// The water form of a world space (`[this + 0x78]`, or the default form at
/// `0x011ca53c`; a world space with a parent asks the parent first).
const WORLD_SPACE_WATER_TYPE: u32 = 0x0058_60c0;
/// `TESWorldSpace::GetCellFromWorldCoord(this, position)` (Xbox PDB name).
const WORLD_SPACE_GET_CELL: u32 = 0x0058_7550;
/// `TESWaterForm::GetPlaceableLODWater(this)` (Xbox PDB name).
const GET_PLACEABLE_LOD_WATER: u32 = 0x0058_0330;
/// `TESObjectCELL::GetWaterHeight(this)` (Xbox PDB name; the result is `float`).
const CELL_GET_WATER_HEIGHT: u32 = 0x0054_71e0;
/// `TESObjectCELL::GetWaterType(this)` (Xbox PDB name).
const CELL_GET_WATER_TYPE: u32 = 0x0054_7770;
/// Sets the cell's water height (`float` at `+0x50`) from the argument.
const CELL_SET_WATER_HEIGHT: u32 = 0x0054_7440;
/// Whether the cell contains the `NiPoint3*` argument (an interior never does).
const CELL_CONTAINS_POINT: u32 = 0x0055_0200;
/// Cell byte `+0x24`, bit 1: interior.
const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
/// Cell byte `+0x24`, bit 2: the cell has water.
const CELL_HAS_WATER: u32 = 0x0045_18e0;
/// The water type's sound (`+0x7c` of the water type).
const WATER_TYPE_SOUND: u32 = 0x0040_7840;
/// Returns `[this + 0xc]`.
const READ_WORD_AT_0C: u32 = 0x0084_e3a0;
/// `NiObjectNET::GetName` (`this + 8`, the address of the name slot) and
/// `NiFixedString` to `const char*`.
const NODE_NAME_SLOT: u32 = 0x0041_3f40;
const FIXED_STRING_TEXT: u32 = 0x0043_b1b0;
/// `cdecl(flags)`: maps the sound's flag word to a flag word of the sound
/// system (`0x10`, `0x2000000`, `0x4000000` or zero).
const SOUND_FLAGS_TO_AUDIO_FLAGS: u32 = 0x005e_39b0;
/// `TESObjectREFR::SetObjectReference(this, form)` (Xbox PDB name).
const REFERENCE_SET_OBJECT_REFERENCE: u32 = 0x0057_5690;
/// `TESObjectREFR::SetLocationOnReference(this, position)` (Xbox PDB name).
const REFERENCE_SET_LOCATION: u32 = 0x0057_5830;
/// `TESObjectREFR::TESObjectREFR(this)` (Xbox PDB name); the object is 0x68
/// bytes.
const REFERENCE_CONSTRUCT: u32 = 0x0055_a2f0;
/// `TESForm::SetTemporary(this)` (Xbox PDB name).
const FORM_SET_TEMPORARY: u32 = 0x0048_4490;
/// `TESObjectREFR::RemoveMasterParticleAddonNodes(node)` (Xbox PDB name;
/// the call pushes the `NiNode*` as its only argument).
const REMOVE_MASTER_PARTICLE_ADDON_NODES: u32 = 0x0057_8170;
/// `this` is a form: the byte `+8` flags tested with `0x1000000`.
const OBJECT_FLAG_1000000: u32 = 0x0045_2370;
/// `this`, mask: false when `this + 0x64` is null, else the answer of
/// `00452420` for that member.
const OBJECT_MEMBER_TEST: u32 = 0x0045_23e0;
/// Reference `+0x64` member has a positive count (`[member + 4] > 0`).
const REFERENCE_HAS_PENDING_NODES: u32 = 0x0057_b200;
/// `this`, 0: removes one of those (called until the test above is false).
const REFERENCE_REMOVE_PENDING_NODE: u32 = 0x0057_b240;
/// `ExtraDataList::QWaterZoneMap(this)` (Xbox PDB name): the map of the
/// extra data entry `0x7e`, or null.
const EXTRA_DATA_LIST_WATER_ZONE_MAP: u32 = 0x0042_f1d0;
/// Map iteration: `this` is the map; first occupied item or null.
const ZONE_MAP_FIRST: u32 = 0x004b_9ba0;
/// `this` (map), `&position`, `&key`, `&value`: reads the item at the
/// position into the key and value, and advances the position.
const ZONE_MAP_GET_NEXT: u32 = 0x006b_7f20;
/// `NiTMapBase<TESObjectREFR *, WadingWaterData *>::GetAt(this, key, &value)`.
const WADING_MAP_GET: u32 = 0x0085_3130;
/// `NiTMapBase::RemoveAt(this, key)` (the engine map names the body after a
/// combat map instance).
const WADING_MAP_REMOVE: u32 = 0x0040_5430;
/// `NiTPointerListBase::AddHead(this, &item)`.
const LIST_ADD_HEAD: u32 = 0x0076_b660;
/// `NiTPointerListBase::AddTail(this, &item)` (no name in the engine map).
const LIST_ADD_TAIL: u32 = 0x004e_d8c0;
/// `NiTPointerListBase::InsertBefore(this, position, &item)`.
const LIST_INSERT_BEFORE: u32 = 0x007b_5330;
/// `this`, `&position`: removes the node at the position and moves the
/// position to the next node.
const LIST_REMOVE_POSITION: u32 = 0x0049_f590;
/// `this`, `&item`, start position (0: the head): the position holding the
/// item, or 0.
const LIST_FIND_POSITION: u32 = 0x0049_c680;
/// `this`, `&position`: the address of the item at the position, moving the
/// position to the next node.
const LIST_NEXT_ITEM: u32 = 0x0057_cbe0;
/// `PlaceableWaterGroup::PlaceableWaterGroup(this)` (Xbox PDB name).
const GROUP_CONSTRUCT: u32 = 0x004e_d5f0;
/// The destructor body `fn_004e52c0` runs (`this`).
const GROUP_DESTRUCT: u32 = 0x004e_d3e0;
/// `cdecl(size)` and `cdecl(pointer)`: the game's `operator new` /
/// `operator delete`.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `cdecl(size)` allocations of the scene-graph classes (`NiAlloc`-style) and
/// of arrays of `short`s.
const NI_ALLOC: u32 = 0x00aa_13e0;
const NI_ALLOC_ARRAY: u32 = 0x00aa_1070;
/// `_vector_constructor_iterator_(array, element size, count, constructor)`
/// (cdecl).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// The scope guard `00404eb0(this, 0x1d, 1, file name, line)` /
/// `00404ee0(this)` the scene-graph allocations are bracketed by.
const ALLOCATION_SCOPE_CONSTRUCT: u32 = 0x0040_4eb0;
const ALLOCATION_SCOPE_DESTRUCT: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\TESWater.cpp"`.
const TESWATER_SOURCE_PATH: u32 = 0x0102_301c;
/// `NiUpdateData::NiUpdateData(this, time, updateControllers, parallel)`.
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
/// `NiAVObject` virtuals: `NiNode::AttachChild(child, firstAvailable)` at
/// `+0xdc` (Xbox PDB), `DetachChild` at `+0xe8`, `UpdateWorldData` at
/// `+0xb8`.
const NODE_ATTACH_CHILD: u32 = 0xdc;
const NODE_DETACH_CHILD: u32 = 0xe8;
const NODE_UPDATE_WORLD_DATA: u32 = 0xb8;
/// `NiAVObject::m_kLocal.m_Translate = *position` (`this + 0x58`).
const NODE_SET_LOCAL_TRANSLATE: u32 = 0x0044_0460;
/// `this`, `&NiUpdateData`: updates the node (calls its virtual `+0xa4` with
/// the data and 0).
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// `NiAVObject::RemoveProperty_ov2(this, type)`, `AttachProperty(this,
/// property)` and `UpdateProperties(this)` (Xbox PDB names).
const NODE_REMOVE_PROPERTY: u32 = 0x00a5_b230;
const NODE_ATTACH_PROPERTY: u32 = 0x0043_9410;
const NODE_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// Return the property type numbers (`3` and `4`).
const WATER_PROPERTY_TYPE: u32 = 0x0043_8220;
const AUTO_WATER_PROPERTY_TYPE: u32 = 0x005d_9660;
/// `NiAVObject` flag bit 1 (+0x30) setter: `this`, value.
const NODE_SET_CULLED: u32 = 0x0045_0f90;
/// `BSShaderManager::PrepareObject(node, 0, 0)` (Xbox PDB name; cdecl).
const SHADER_MANAGER_PREPARE_OBJECT: u32 = 0x00b5_7e30;
/// `WaterShaderProperty::WaterShaderProperty(this)` (Xbox PDB name; 0x150
/// bytes on the PC).
const WATER_SHADER_PROPERTY_CONSTRUCT: u32 = 0x00b6_abb0;
/// The 0x24-byte property the LOD water gets next to its water shader
/// property (constructor, then a call with 3).
const AUTO_WATER_PROPERTY_CONSTRUCT: u32 = 0x0049_ec80;
const AUTO_WATER_PROPERTY_SET: u32 = 0x0049_ee30;
/// `BSFadeNode::BSFadeNode(this)` (Xbox PDB name; 0xe4 bytes).
const FADE_NODE_CONSTRUCT: u32 = 0x00b4_e150;
/// `NiTriShape::NiTriShape_ov2(this, data)` (Xbox PDB name; 0xc4 bytes).
const TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4480;
/// `NiTriShapeData::NiTriShapeData(this, vertexCount, vertices, normals,
/// colors, uvs, textureSets, consistency, 2, triangles)` (Xbox PDB name;
/// 0x58 bytes).
const TRI_SHAPE_DATA_CONSTRUCT: u32 = 0x00a7_b630;
/// `NiPoint2::NiPoint2(this, x, y)` and `NiColorA::NiColorA(this, r, g, b,
/// a)` (both return `this`).
const NI_POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
const NI_COLOR_CONSTRUCT: u32 = 0x0041_4430;
/// The default constructor `_vector_constructor_iterator_` runs on
/// `NiColorA` elements.
const NI_COLOR_DEFAULT_CONSTRUCT: u32 = 0x004a_7800;
/// The `float` setting `fAlpha:Water`: constructor `(this, name, default)`,
/// the exit-time destructor and the setter `(this, value)`.
const FLOAT_SETTING_CONSTRUCT: u32 = 0x0044_f440;
const WATER_ALPHA_SETTING_DESTRUCT: u32 = 0x00fc_a030;
const FLOAT_SETTING_SET: u32 = 0x004e_d780;
/// `float` value of a float setting (`this` is the setting).
const FLOAT_SETTING_GET: u32 = 0x0045_0410;
/// The CRT `atexit(function)`.
const ATEXIT: u32 = 0x00ec_658f;
/// `PlayerCharacter::GetWaterCell(this, radius)` (Xbox PDB name).
const PLAYER_GET_WATER_CELL: u32 = 0x0093_bba0;
/// The position of the player (`NiPoint3`, `this + 0x30`).
const PLAYER_POSITION: u32 = 0x0043_6aa0;
/// `PlayerCharacter` virtual `GetLocationOnReference` (Xbox PDB `+0x1f0`).
const REFERENCE_GET_LOCATION_VIRTUAL: u32 = 0x1f4;
/// `fabsf`: `cdecl(float)`, result in `ST0`.
const FLOAT_ABS: u32 = 0x0040_8840;
/// `NiPoint2::Length(this)`.
const POINT2_LENGTH: u32 = 0x0058_9850;
/// `NiPoint2::operator*(this, out, scalar)` (returns `out`).
const POINT2_SCALE: u32 = 0x004a_4f40;
/// `MakeTriPoint(size, &color, 1)` (Xbox PDB name; cdecl): a debug marker
/// node. `004b3890(&from, &fromColor, &to, &toColor, 1)` (cdecl) builds a
/// debug line.
const MAKE_TRI_POINT: u32 = 0x004b_29b0;
const MAKE_DEBUG_LINE: u32 = 0x004b_3890;
/// `BSAudio::QInstance` (Xbox PDB name): the `BSAudio*` global; then
/// `BSAudio::GetSoundHandleByNumericID(this, &out, sound, id)`.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_GET_SOUND_HANDLE: u32 = 0x00ad_73b0;
/// `BSSoundHandle` methods (Xbox PDB names): `IsValid`, `Release`, `Stop`,
/// `Play(this, 1)`, `IsPlaying` and `SetPosition(this, x, y, z)`; and the
/// copy assignment (`this`, `&source`) and the empty destructor.
const SOUND_HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_IS_PLAYING: u32 = 0x00ad_8930;
const SOUND_HANDLE_SET_POSITION: u32 = 0x00ad_8b60;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
/// `ImageSpaceEffectWaterFFT::FreeUpTextureMemory(this)` (Xbox PDB name).
const WATER_FFT_FREE_TEXTURE_MEMORY: u32 = 0x00ba_1250;
/// A virtual no-op (one byte, `RET`) `fn_004e6620` calls on the object
/// `fn_004e6a60` returns.
const WATER_OBJECT_EMPTY_CALL: u32 = 0x00a2_9680;
/// `IsKindOf`: `cdecl(rtti, object)`; false for a null object.
const IS_KIND_OF: u32 = 0x0045_bad0;
/// The `NiRTTI` the shader property of a water reference is tested with,
/// and the one of the `BSFaceGenNiNode` check.
const WATER_SHADER_PROPERTY_RTTI: u32 = 0x011f_a018;
const FACE_GEN_NODE_RTTI: u32 = 0x0120_2e74;
/// `NiNode::GetAt(this, 0)`-like: child 0 of a node (`NiPointer` read).
const NODE_FIRST_CHILD: u32 = 0x0043_b4a0;
/// `BSFaceGenNiNode::GetAnimationData(this)` (Xbox PDB name), then a
/// `NiPointer` read at `+0xc` of it, then the count of that.
const FACE_GEN_ANIMATION_DATA: u32 = 0x0066_29f0;
const ANIMATION_DATA_TARGET: u32 = 0x0043_b230;
/// `cdecl(node, viewer)`: the range test of a node that is not face-gen.
const NODE_IN_RANGE_OF_VIEWER: u32 = 0x004b_5fc0;

// ---------------------------------------------------------------------------
// Callees and globals of the functions from `004e8000` on (third session).

/// What `fn_004e8000` runs on the water root node before it drops it (the
/// body calls `004dffa0` on `this + 0x9c`; the engine map has no name).
const WATER_ROOT_RELEASE_STEP: u32 = 0x004d_ef90;
/// `[this] + 8`: the address of the first item of an `NiTPointerList` (the
/// engine map has no name).
const LIST_FIRST_ITEM_SLOT: u32 = 0x0073_ac90;
/// A `WaterShaderProperty` method (`this`, the new property); the engine map
/// has no name.
const WATER_PROPERTY_METHOD_00B6AB20: u32 = 0x00b6_ab20;
/// `BSRenderedTexture::GetTexture(this, index)` (Xbox PDB name).
const RENDERED_TEXTURE_GET_TEXTURE: u32 = 0x004b_c320;
/// `NiPoint3::operator-(this, out, other)`: `out = this - other` (returns
/// `out`; it builds the result with `NiPoint3::NiPoint3`).
const POINT3_SUBTRACT: u32 = 0x0043_9ef0;
/// `NiPoint3::Length(this)` (ST0).
const POINT3_LENGTH: u32 = 0x0045_7990;
/// Appends the eight-byte element `*element` (an `NiPoint2`) to the array
/// at `this` (`this`, `&element`; the engine map has no name).
const POINT2_ARRAY_APPEND: u32 = 0x006d_b840;
/// `cdecl(a, b, c, d, value)`: `(b - a) * ((value - c) / (d - c)) + a`, the
/// linear map of `value` from the range `c..d` onto `a..b` (ST0).
const LINEAR_MAP: u32 = 0x004b_3ab0;
/// `TESWaterSystem::InitializeGreyTexture(this)` (Xbox PDB name; next
/// session): returns the rendered texture the wading height map starts as.
const INITIALIZE_GREY_TEXTURE: u32 = 0x004e_d290;
/// `NiTMapBase<TESObjectREFR *, ...>::SetAt(this, key, value)` (the engine
/// map names the body after a combat-threat map instance).
const MAP_SET_AT: u32 = 0x0084_4700;
/// `TESObjectREFR`-derived actor virtual at +0x22c, called with 0 (a
/// `bool` result): the wading-water code skips the actors it answers true
/// for.
const ACTOR_SKIP_VIRTUAL: u32 = 0x22c;
/// `cdecl(value)`: the float wrapper (`0x00406cc0`) around the CRT function
/// at `0x00ec6940`, which rounds down.
const FLOAT_FLOOR: u32 = 0x0040_6cc0;
/// `cdecl(index)`: the word at `0x011f91c8 + 4 * index`.
const RENDER_TABLE_ENTRY: u32 = 0x0045_0b80;
/// `cdecl(index)`: `[0x011f91c0]` for 0, `[0x011f91bc]` otherwise.
const RENDER_SETTING_ENTRY: u32 = 0x004d_c060;
/// Returns the word at `0x011f4748` (`cdecl`, no arguments): the object the
/// camera data are handed to.
const RENDER_GLOBAL_OBJECT: u32 = 0x0043_c4b0;
/// `(this, name, value)`: whether the render object answers `name` with
/// `value` (its byte at +0x208 is compared); `004a0dd0` and `004e9c90` ask it
/// about `"SetCameraData"`.
const RENDER_OBJECT_CHECK: u32 = 0x004a_0e10;
/// `(this, camera data)`: asks `004a0e10` about the string at `0x0101e2c8`,
/// then calls the object's virtual at +0x190 with the argument.
const RENDER_OBJECT_SET_CAMERA_DATA: u32 = 0x004a_0dd0;
/// `NiCamera::NiCamera(this)` (the camera object is 0x114 bytes).
const CAMERA_CONSTRUCT: u32 = 0x00a7_12f0;
/// `NiCamera::~NiCamera(this)`.
const CAMERA_DESTRUCT: u32 = 0x00a6_fae0;
/// `NiCamera::SetViewFrustum(this, &frustum)` (Xbox PDB name).
const CAMERA_SET_VIEW_FRUSTUM: u32 = 0x00a6_faf0;
/// `NiCamera::LookAtWorldPoint(this, &point, &up)` (Xbox PDB name).
const CAMERA_LOOK_AT_WORLD_POINT: u32 = 0x00a7_01b0;
/// `this + 0x100` of the camera (the view frustum) and `this + 0xdc` (the
/// world-to-camera data `NiCullingProcess` takes).
const CAMERA_FRUSTUM_ADDRESS: u32 = 0x004a_0d10;
const CAMERA_PLANES_ADDRESS: u32 = 0x0045_bbe0;
/// `NiFrustum::NiFrustum(this, ortho)` (Xbox PDB name; 0x1c bytes).
const FRUSTUM_CONSTRUCT: u32 = 0x00a7_1b70;
/// `BSCullingProcess::BSCullingProcess(this, 0)` and
/// `BSCullingProcess::~BSCullingProcess(this)` (Xbox PDB name); the object
/// is at most 0xcc bytes on the stack.
const CULLING_PROCESS_CONSTRUCT: u32 = 0x004a_0eb0;
const CULLING_PROCESS_DESTRUCT: u32 = 0x004a_0f60;
const CULLING_PROCESS_SIZE: u32 = 0xd0;
/// Stores its argument (a camera) at `this + 0xc` (the engine map's name,
/// `NonActorMagicCaster::SetCurrentSpell`, belongs to a folded copy).
const CULLING_PROCESS_SET_CAMERA: u32 = 0x0041_fd00;
/// `(this, camera planes)`: `nicullingprocess.cpp`; ends by setting the
/// word at +0x8c to `0x3f`.
const CULLING_PROCESS_SET_PLANES: u32 = 0x00a6_94a0;
/// `BSShaderAccumulator::BSShaderAccumulator(this, 99, 1, 0x2f7)` (Xbox PDB
/// name; 0x280 bytes).
const ACCUMULATOR_CONSTRUCT: u32 = 0x00b6_60d0;
/// Stores its argument at `this + 0x194` and `this + 0x19c` of the
/// accumulator.
const ACCUMULATOR_SET_WORD_194: u32 = 0x004a_1020;
const ACCUMULATOR_SET_WORD_19C: u32 = 0x004a_1040;
/// Stores its byte argument at `this + 0x84` of the accumulator
/// (`bAccumulate`, Xbox PDB).
const ACCUMULATOR_SET_ACCUMULATE: u32 = 0x004b_c4a0;
/// The accumulator's virtual at +0x8c, called with the camera.
const ACCUMULATOR_SET_CAMERA_VIRTUAL: u32 = 0x8c;
/// `BSUtilities::ReflectCameraAboutArbitraryPlane(camera, &plane,
/// reflected camera)` (Xbox PDB name; cdecl).
const REFLECT_CAMERA_ABOUT_PLANE: u32 = 0x00c4_bf40;
/// `MTRenderingSystem::AddAccumTask(this, camera, 0, 0, static objects,
/// dynamic objects, accumulator, 4, stage, 0)` (Xbox PDB name),
/// `MTRenderingSystem::SetThreadStage(this, 0, stage)` (Xbox PDB name) and
/// the engine map's unnamed `(this, 1, stage)` between them.
const MT_ADD_ACCUM_TASK: u32 = 0x00ba_3390;
const MT_SET_THREAD_STAGE: u32 = 0x00ba_30f0;
const MT_SET_THREAD_STAGE_ONE: u32 = 0x00ba_3130;
/// The `NiPointer` member at +4 (read through `00559450`), here of the sky
/// (`TES + 0x68`, `pSky`, Xbox PDB).
const SKY_OBJECT_POINTER: u32 = 0x007f_a950;
/// `TES + 0x68` (`pSky`, Xbox PDB), read through the folded accessor the
/// engine map names `MiddleHighProcess::GetSavedAcquireObject`.
const TES_GET_SKY: u32 = 0x008d_8520;
/// `this + 0x20` (the address of three floats).
const COLOR_FACTOR_ADDRESS: u32 = 0x0089_1170;
/// `NiPointer` read at `0x011deb7c`.
const POINTER_STATIC_011DEB7C: u32 = 0x0045_c670;
/// `TESWorldSpace::GetTerrainManager(this)` (Xbox PDB name).
const WORLD_SPACE_GET_TERRAIN_MANAGER: u32 = 0x0058_6170;
/// Whether the current world space has a terrain manager that answers
/// `00759d80` (byte result).
const TERRAIN_READY: u32 = 0x006f_d150;
/// Terrain helpers the reflection passes call around their object lists:
/// they call `006fb5d0()`, `006fb640()`, `006fd1d0(1)` and `006fd1d0(0)`
/// (no engine-map names).
const TERRAIN_STEP_FIRST: u32 = 0x006f_d1a0;
const TERRAIN_STEP_SECOND: u32 = 0x006f_d1b0;
const TERRAIN_TOGGLE_ONE: u32 = 0x006f_d1c0;
const TERRAIN_TOGGLE_ZERO: u32 = 0x006f_d1e0;
/// `(this = TES, x, y)`: the address of the grid cell slot (`0` when out of
/// range).
const TES_GRID_CELL_SLOT: u32 = 0x0045_7050;
/// `(this = cell, index)`: child `index` of the cell's 3D node of kind 2.
const CELL_NODE_CHILD: u32 = 0x0045_c9a0;
/// `bool` test on the reference (`564e60`, folded; flag `0x8000` of the
/// form or of its base form's `564e40` test): the reflection passes skip the
/// references it answers true for outside interiors.
const REFERENCE_IS_EXCLUDED: u32 = 0x0056_4e60;
/// `this`: the next node of a `BSSimpleList`-style node (`[this + 4]`).
const NODE_NEXT: u32 = 0x0072_6070;
/// `(camera, accumulator, 0)` (cdecl): draws the accumulated scene into the
/// current target.
const RENDER_ACCUMULATED_SCENE: u32 = 0x00b6_c0d0;
/// `(7 words, cdecl)`: a render state call; the water code passes `(1, 0
/// x 6)` and `(0 x 7)`.
const RENDER_STATE_SET: u32 = 0x00b9_8280;
/// `ImageSpaceManager::RenderEffect`-like call: `this`, 0x20, renderer,
/// texture, texture, 0, 1 (the engine map names only the unit).
const IMAGE_SPACE_RENDER_DISPLACEMENT: u32 = 0x00b9_7550;
/// `BSBatchRenderer::RenderPassImmediately(pass, count, 0, 0, 0)` (Xbox PDB
/// name; cdecl) and the property method that returns a geometry's render
/// pass (`this` is the property, then the geometry).
const RENDER_PASS_IMMEDIATELY: u32 = 0x00b9_94f0;
const PROPERTY_RENDER_PASS: u32 = 0x00b6_9f30;
/// `TESWaterSystem` method of the next session; `this`, a plane as four words
/// (normal, then minus the constant), 0.
const REFLECTION_PLANE_SETUP: u32 = 0x004e_cef0;

/// Settings the third session reads.
/// `bUseWaterDisplacements:Water`, `bUseBulletWaterDisplacements:Water`,
/// `fWadingWaterQuadSize:Water`, `bForceHighDetailLandReflections:Water`
/// and `bReflectExplosions:Water`.
const SETTING_USE_WATER_DISPLACEMENTS: u32 = 0x011c_7ac4;
const SETTING_USE_BULLET_WATER_DISPLACEMENTS: u32 = 0x011c_7c80;
const SETTING_WADING_WATER_QUAD_SIZE: u32 = 0x011c_7b54;
const SETTING_FORCE_HIGH_DETAIL_LAND_REFLECTIONS: u32 = 0x011c_7cc0;
const SETTING_REFLECT_EXPLOSIONS: u32 = 0x011c_7ae8;
/// The unsigned setting the `TES` grid loops count to (a cell grid width);
/// its initializer was not found.
const SETTING_GRID_SIZE: u32 = 0x011c_63cc;

/// Globals.
/// `NiPointer<NiCamera>` and `NiPointer<BSShaderAccumulator>` statics of the
/// world and sky reflections (`PlaceableWaterGroup::spWorldReflectionCamera`,
/// `spWorldReflectionSorter`, `spSkyReflectionCamera`, `spSkyReflectionSorter`
/// in the Xbox PDB, matched by use) and the four object lists
/// (`StaticWorldReflectiveObjects`, `DynamicWorldReflectiveObjects`,
/// `StaticSkyReflectiveObjects`, `DynamicSkyReflectiveObjects`, matched by
/// the order `004e19c0` clears them in).
const WORLD_REFLECTION_CAMERA: u32 = 0x011c_7b44;
const WORLD_REFLECTION_SORTER: u32 = 0x011c_7cb8;
const SKY_REFLECTION_CAMERA: u32 = 0x011c_7cbc;
const SKY_REFLECTION_SORTER: u32 = 0x011c_7be8;
const STATIC_WORLD_REFLECTIVE_OBJECTS: u32 = 0x011c_7ce4;
const DYNAMIC_WORLD_REFLECTIVE_OBJECTS: u32 = 0x011c_7b80;
const STATIC_SKY_REFLECTIVE_OBJECTS: u32 = 0x011c_7bcc;
const DYNAMIC_SKY_REFLECTIVE_OBJECTS: u32 = 0x011c_7c30;
/// `PlaceableWaterGroup::ExplosionsList` (Xbox PDB static; matched by use:
/// the explosion code adds to it and `bReflectExplosions` gates the read).
const EXPLOSIONS_LIST: u32 = 0x011c_7b8c;
/// The statics `004ea9a0`, `004ea9c0` and `004ea9e0` read through
/// `NiPointer` (terrain objects; not matched to a name).
const TERRAIN_POINTER_A: u32 = 0x011d_86a8;
const TERRAIN_POINTER_B: u32 = 0x011d_8690;
const TERRAIN_POINTER_C: u32 = 0x011d_86bc;
/// The two `NiPoint2` arrays the displacement passes fill (and
/// `fn_004e8ec0` clears after its image-space pass).
const DISPLACEMENT_POINTS_A: u32 = 0x011a_da00;
const DISPLACEMENT_POINTS_B: u32 = 0x011a_da10;
/// Bytes: `0x011fffec` is set when the wading height map is created,
/// `0x011fffed` while the displacement image-space pass runs, `0x01200000` is
/// cleared when the wading geometry is released, `0x011ad832` makes
/// `fn_004e8ec0` render the wading geometry, `0x011ff375` is set while it
/// does.
const WADING_MAP_CREATED_FLAG: u32 = 0x011f_ffec;
const DISPLACEMENT_PASS_FLAG: u32 = 0x011f_ffed;
const DISPLACEMENT_ACTIVE_FLAG: u32 = 0x0120_0000;
const DISPLACEMENT_RENDER_FLAG: u32 = 0x011a_d832;
const WADING_RENDER_ACTIVE_FLAG: u32 = 0x011f_f375;
/// The floats `fn_004e8ec0` publishes for the displacement shader: four
/// constants (copied from the exe's `0.4`, `0.6`, `0.97` and `0.01`) and the
/// two offsets.
const DISPLACEMENT_CONSTANTS: u32 = 0x0120_0014;
const DISPLACEMENT_SHIFT_X: u32 = 0x0120_0024;
const DISPLACEMENT_SHIFT_Y: u32 = 0x0120_0028;
const DISPLACEMENT_CONSTANT_SOURCES: [u32; 4] =
    [0x0102_31a8, 0x0101_8180, 0x0102_31a4, 0x0101_3ea4];
/// A word `004eb510` lowers by its argument.
const COUNTER_011FFA14: u32 = 0x011f_fa14;
/// `0.0` / `0.1` constants: the camera's near plane of the wading camera
/// (float), `4.0` and `50.0` (doubles), `-FLT_MAX`'s positive twin as float
/// and double.
const WADING_NEAR_PLANE: u32 = 0x0101_e2bc;
const FOUR: u32 = 0x0101_db80;
const FIFTY: u32 = 0x0101_e2c0;
const HIGHEST_FLOAT: u32 = 0x0101_6970;
const HIGHEST_FLOAT_AS_DOUBLE: u32 = 0x0102_31b0;
/// The four floats the displacement mapping uses: `-0.5`, `0.5`, `-512.0`,
/// `512.0` (read in that order by the game).
const MAP_LOW: u32 = 0x0102_295c;
const MAP_HIGH: u32 = 0x0101_6248;
const MAP_SOURCE_LOW: u32 = 0x0102_319c;
const MAP_SOURCE_HIGH: u32 = 0x0102_31a0;
/// The `NiPoint3` `(0, 0, 1)` the wading camera looks up with.
const UP_VECTOR: u32 = 0x011a_9478;
/// A `NiPoint3` of three zero words.
const ZERO_POINT3: u32 = 0x011f_426c;

/// `NiCamera`... `TESWater.cpp` lines the allocation scopes use.
const REFLECTION_SCOPE_LINE_GROUP: u32 = 0xefb;
const REFLECTION_SCOPE_LINE_WORLD: u32 = 0xfe3;
const REFLECTION_SCOPE_LINE_SKY: u32 = 0x1054;
const REFLECTION_SCOPE_LINE_FINISH: u32 = 0x1082;

/// The camera's local translate (`this + 0x58`, an address) and the rotation
/// accessors: `00439f50(this = matrix, column, &out)` copies a column of a
/// `NiMatrix3`, `0045bba0(this = camera, &out)` copies column 0 of the
/// camera's rotation.
const NODE_LOCAL_TRANSLATE_ADDRESS: u32 = 0x0043_c490;
const MATRIX_COLUMN: u32 = 0x0043_9f50;
const CAMERA_COLUMN_ZERO: u32 = 0x0045_bba0;
/// `"SetCameraData"` and the render object's virtual `fn_004e9c90` calls.
const SET_CAMERA_DATA_NAME: u32 = 0x0102_31b8;
const RENDER_OBJECT_SET_CAMERA_DATA_VIRTUAL: u32 = 0x18c;
/// The 16-bit flag word `fn_004e9ce0` edits.
const BIT_FLAGS_011F941C: u32 = 0x011f_941c;
/// The address of the multithreaded rendering system's static object.
const MT_RENDERING_SYSTEM: u32 = 0x0120_0088;
/// Returns the byte of `bUseWaterShader:Water` (no engine-map name).
const WATER_SHADER_ENABLED: u32 = 0x004e_2160;
/// `(this = actor, &location, cell, 1.0)`: whether the actor passes the
/// water test at that location (`actor.cpp`, no name), and `(this = actor)`:
/// `(0x008846e0(this) & 0xf) != 0`.
const ACTOR_IN_WATER_TEST: u32 = 0x0088_5520;
const ACTOR_STATE_TEST: u32 = 0x0049_38e0;
/// The form flag masks the accessors from `004ea8b0` on test.
const FORM_FLAG_0004: u32 = 0x0000_0004;
const FORM_FLAG_0008: u32 = 0x0000_0008;
const FORM_FLAG_0010: u32 = 0x0000_0010;
const FORM_FLAG_0020: u32 = 0x0000_0020;
const FORM_FLAG_0040: u32 = 0x0000_0040;
const FORM_FLAG_0400: u32 = 0x0000_0400;

// ---------------------------------------------------------------------------
// Callees and globals of the functions from `004eb540` on (fourth session).

/// `TESWater.cpp` lines of the allocation scopes the finish and setup
/// functions open.
const SCOPE_LINE_FINISH_WORLD: u32 = 0x10b8;
const SCOPE_LINE_FINISH_SKY: u32 = 0x1114;
const SCOPE_LINE_SETUP_DEPTH: u32 = 0x114f;
const SCOPE_LINE_FINISH_DEPTH: u32 = 0x1209;
const SCOPE_LINE_GREY_TEXTURE: u32 = 0x138a;

/// `bUseWaterReflectionBlur:Water`, `iWaterBlurAmount:Water` and
/// `fRefractionWaterPlaneBias:Water` (the `SettingT<INISettingCollection>`
/// statics, named by the string their static initializers register).
const SETTING_USE_WATER_REFLECTION_BLUR: u32 = 0x011c_7c48;
const SETTING_WATER_BLUR_AMOUNT: u32 = 0x011c_7c1c;
const SETTING_REFRACTION_WATER_PLANE_BIAS: u32 = 0x011c_7c10;
/// The `float` the blur pass publishes for the image-space shaders:
/// `iWaterBlurAmount + 1`.
const WATER_BLUR_FACTOR: u32 = 0x0120_03c4;
/// The image-space manager's effect `0x10 + iWaterBlurAmount` is the blur
/// the world reflection is drawn through; its texture slot 2 takes the
/// blur's rendered texture.
const BLUR_EFFECT_BASE_INDEX: u32 = 0x10;
const BLUR_EFFECT_TEXTURE_SLOT: u32 = 2;
/// `ImageSpaceTexture::SetTexture(this, texture)` and
/// `ImageSpaceTexture::~ImageSpaceTexture` (Xbox PDB names): the 0x10-byte
/// holder `fn_004ebb70` constructs.
const IMAGE_SPACE_TEXTURE_SET: u32 = 0x00ba_37a0;
const IMAGE_SPACE_TEXTURE_DESTRUCT: u32 = 0x00ba_3a00;
/// `ImageSpaceEffect::SetTexture(this, slot, texture holder, 0)` (Xbox PDB
/// name).
const IMAGE_SPACE_EFFECT_SET_TEXTURE: u32 = 0x00ba_3cb0;
/// `(this, index)`: the address of element `index` of the array whose base
/// is `[this + 4]` (no engine-map name; `fn_004ebbc0` reads through it).
const ARRAY_ELEMENT_ADDRESS: u32 = 0x0087_7a30;
/// The vtable of `ImageSpaceEffectParam` (0x24 bytes), and the constructors
/// `(this, 0, 1)` and destructors of its two members at `+4` and `+0x14`
/// (the next session translates the constructors; the destructors belong to
/// no unit of ours).
const IMAGE_SPACE_EFFECT_PARAM_VTABLE: u32 = 0x0102_31cc;
const IMAGE_SPACE_EFFECT_PARAM_SIZE: u32 = 0x24;
const EFFECT_PARAM_FIRST_MEMBER_CONSTRUCT: u32 = 0x004e_dc90;
const EFFECT_PARAM_SECOND_MEMBER_CONSTRUCT: u32 = 0x004e_dcf0;
const EFFECT_PARAM_FIRST_MEMBER_DESTRUCT: u32 = 0x004e_dc60;
const EFFECT_PARAM_SECOND_MEMBER_DESTRUCT: u32 = 0x004e_dcc0;
/// `TES + 0x64` (`pWaterSystem` by use: `fn_004ed3e0` gives the group to the
/// system it returns).
const TES_GET_WATER_SYSTEM: u32 = 0x0070_ec90;
/// `NiTPointerList` constructor and destructor (`this`) the group
/// constructor and destructor run on their list members.
const LIST_CONSTRUCT: u32 = 0x0048_f200;
const LIST_DESTRUCT: u32 = 0x004a_1a30;
/// `NiPlane::NiPlane(this)` (Xbox PDB name).
const NI_PLANE_DEFAULT_CONSTRUCT: u32 = 0x00a6_9940;
/// `NiPoint3::Dot(this, other)` (Xbox PDB name; the result is in `ST0`).
const NI_POINT3_DOT: u32 = 0x004b_6190;
/// The CRT `memcpy(destination, source, count)` (cdecl; the engine map's
/// `_memcpy`).
const MEMORY_COPY: u32 = 0x00ec_44d0;
/// The D3DX import thunks the plane setup calls: `D3DXMatrixMultiply(out,
/// a, b)`, `D3DXMatrixInverse(out, determinant, matrix)` and
/// `D3DXMatrixTranspose(out, matrix)` (engine map names), and the two
/// thunks next to them that the decompiler names `D3DXPlaneNormalize(out,
/// plane)` and `D3DXPlaneTransform(out, plane, matrix)` (checked against
/// the arguments the code passes).
const D3DX_MATRIX_MULTIPLY: u32 = 0x00ee_6de0;
const D3DX_MATRIX_INVERSE: u32 = 0x00ee_6dda;
const D3DX_MATRIX_TRANSPOSE: u32 = 0x00ee_6dd4;
const D3DX_PLANE_NORMALIZE: u32 = 0x00ee_6dce;
const D3DX_PLANE_TRANSFORM: u32 = 0x00ee_6dc8;
/// The `NiPoint3` global `fn_004ed180` returns: the view matrix code takes
/// its dot product with each axis (by use it is the eye position).
const EYE_POSITION: u32 = 0x011f_474c;
/// The offsets in the renderer of the two 4x4 matrices `fn_004ed1f0` and
/// `fn_004ed210` return, and of the device object the depth pass sets
/// states on.
const RENDERER_VIEW_MATRIX_OFFSET: u32 = 0x980;
const RENDERER_PROJECTION_MATRIX_OFFSET: u32 = 0x9c0;
const RENDERER_DEVICE_OFFSET: u32 = 0x8b8;
/// The render object's device (`render object + 0x288`) and its virtual at
/// `+0xdc`, called as `(device, index, plane)` with `ECX` holding the
/// device's vtable pointer (going by its arguments it is Direct3D 9's
/// `SetClipPlane`, a stdcall method that ignores `ECX`).
const RENDER_OBJECT_DEVICE_OFFSET: u32 = 0x288;
const DEVICE_SET_CLIP_PLANE_VIRTUAL: u32 = 0xdc;
/// The renderer device's virtual at `+0x68`, called as `(state, value, 0,
/// 0)` (`00b98070` and `00b98380` call it with the states `0x34` and
/// `0xa8`).
const DEVICE_SET_STATE_VIRTUAL: u32 = 0x68;
/// The state the depth pass resets once it has drawn.
const DEPTH_PASS_RESET_STATE: u32 = 0x98;
/// The render-state counters: an array of words at `0x011ff9d8`; entries 9,
/// 0xf and 0x10 are `COUNTER_011FF9FC`, `COUNTER_011FFA14` and
/// `COUNTER_011FFA18`.
const RENDER_STATE_COUNTERS: u32 = 0x011f_f9d8;
const COUNTER_011FF9FC: u32 = 0x011f_f9fc;
const COUNTER_011FFA18: u32 = 0x011f_fa18;
/// `cdecl(value, count)`: sets state `0x34` of the device when its counter
/// is clear, then adds `count` to the counter; the same for state `0xa8`.
const COUNTED_STATE_34: u32 = 0x00b9_8070;
const COUNTED_STATE_A8: u32 = 0x00b9_8380;
/// `cdecl(0, 0, 0, 1)` and `cdecl(0, 1)`: render-state calls of the depth
/// pass (`bsrenderstate_xenon.cpp`; no engine-map names).
const RENDER_STATE_980C0: u32 = 0x00b9_80c0;
const RENDER_STATE_98230: u32 = 0x00b9_8230;
/// Four floats (the group's reflect plane) and two floats (a `NiPoint2`)
/// the depth pass publishes for the shaders.
const DEPTH_PLANE_SHADER_CONSTANT: u32 = 0x011f_9604;
const DEPTH_RANGE_SHADER_CONSTANT: u32 = 0x011f_9614;
/// Camera accessors the depth cameras are copied from the viewer with (no
/// engine-map names): `(this, &matrix)` copies nine words to `this + 0x34`;
/// `(this, scale)` stores the scale at `this + 0x64`; `NODE_SCALE_SOURCE`
/// is the `float` at `+0x98` of the viewer; the `float` at `+0xfc` is read
/// from the viewer and stored into the camera (a camera member next to the
/// frustum).
const NODE_SET_LOCAL_ROTATE: u32 = 0x0043_fa80;
const NODE_SET_LOCAL_SCALE: u32 = 0x0044_0490;
const NODE_SCALE_SOURCE: u32 = 0x008d_01e0;
const CAMERA_FLOAT_FC_READ: u32 = 0x0064_47f0;
const CAMERA_FLOAT_FC_WRITE: u32 = 0x0050_7700;
/// The `float`s at `+0xa8` and `+0xa4` of a water form (no engine-map
/// names: folded getters), which `fn_004ec800` publishes as a `NiPoint2`
/// (the one at `+0xa4` first).
const WATER_FORM_FLOAT_A8: u32 = 0x009b_88a0;
const WATER_FORM_FLOAT_A4: u32 = 0x0081_2870;
/// The reference value of the depth pass's stencil state.
const DEPTH_STENCIL_REFERENCE: u32 = 0xff;
/// `0x011ca144`: the list head `fn_004ec7b0` returns, a `{item, next}` node
/// chain the depth setup walks for forms with the bit `0x40000000`.
const EXTRA_DEPTH_OBJECT_LIST: u32 = 0x011c_a144;
/// The `float` `0.5` the grey texture is cleared to.
const GREY_VALUE: u32 = 0x0101_6248;
/// The `double` `fn_004ed230` scales its byte with.
const COLOR_STEP_SCALE: u32 = 0x0102_31e8;
/// `cdecl(destination, value, count)`: a wrapper of the CRT `memset`.
const MEMORY_SET: u32 = 0x0040_3d30;
/// Stores its float argument at `this + 4` when `this` is not null and
/// returns `this` (the engine map has no name; `fn_004ed780` calls it).
const SETTING_STORE_FLOAT: u32 = 0x004d_e290;
/// The three pointer maps the water system holds, by what they map:
/// `NiTPointerMap<TESObjectREFR *, TESObjectREFR *>` (the `ReflectionRefMap`
/// and `DepthRefMap` type), `NiTPointerMap<TESWaterForm *, bool>`
/// (`WaterTypeUpdateMap`) and `NiTPointerMap<TESObjectREFR *,
/// WadingWaterData *>` (`WadingWaterMap`): the vtables of the derived
/// classes and of the `NiTMapBase` instances `fn_004ed960` and `fn_004eda60`
/// construct, the base constructor of the third (`004edb60`)
/// and the three base destructor bodies.
const REFERENCE_MAP_VTABLE: u32 = 0x0102_31f4;
const WATER_FORM_MAP_VTABLE: u32 = 0x0102_3214;
const WADING_MAP_VTABLE: u32 = 0x0102_3234;
const REFERENCE_MAP_BASE_VTABLE: u32 = 0x0102_3254;
const WATER_FORM_MAP_BASE_VTABLE: u32 = 0x0102_3274;
const WADING_MAP_BASE_CONSTRUCT: u32 = 0x004e_db60;
/// The vtable `fn_004edb60` sets (the base of the wading map).
const WADING_MAP_BASE_VTABLE: u32 = 0x0102_3294;
/// The destructor bodies of the three `NiTMapBase` instances (`this`).
const REFERENCE_MAP_BASE_DESTRUCT: u32 = 0x004e_da30;
const WATER_FORM_MAP_BASE_DESTRUCT: u32 = 0x004e_db30;
const WADING_MAP_BASE_DESTRUCT: u32 = 0x004e_dc30;
/// The constructor bodies `fn_004edc90` and `fn_004edcf0` call
/// (`this, first, second`; the engine map has no names).
const CONSTRUCT_BODY_004EDF70: u32 = 0x004e_df70;
const CONSTRUCT_BODY_004EDFE0: u32 = 0x004e_dfe0;
const REFERENCE_MAP_DESTRUCT: u32 = 0x004e_d9d0;
const WATER_FORM_MAP_DESTRUCT: u32 = 0x004e_dad0;
const WADING_MAP_DESTRUCT: u32 = 0x004e_dbd0;

// ---------------------------------------------------------------------------
// Globals.

/// The `TES*` (`TES + 0x34` is `pInteriorCell`).
const TES_POINTER: u32 = 0x011d_ea10;
/// The `ImageSpaceManager*` the water passes render effects with.
const IMAGE_SPACE_MANAGER: u32 = 0x011f_91ac;
/// A pointer to an object the water update flushes (`fn_004e4680`).
const FLUSH_OBJECT: u32 = 0x011f_fe44;
/// A byte flag (`fn_004e3c40` returns it; `00575c96` writes it).
const WATER_FLAG_011C7A58: u32 = 0x011c_7a58;
/// An object `UpdatePlaceableWater` asks `WORDS_ARE_ZERO` about
/// (`fn_004e3260` returns its address).
const WATER_OBJECT_011CA13C: u32 = 0x011c_a13c;
/// `PlaceableWaterGroup::bRenderWorldReflections` and
/// `bRenderSkyReflections` (Xbox PDB statics; match by use), and
/// `iWorldReflectionThreadStage` / `iSkyReflectionThreadStage`.
const RENDER_WORLD_REFLECTIONS: u32 = 0x011c_7a66;
const RENDER_SKY_REFLECTIONS: u32 = 0x011c_7a67;
const WORLD_REFLECTION_THREAD_STAGE: u32 = 0x011c_7a6c;
const SKY_REFLECTION_THREAD_STAGE: u32 = 0x011c_7a70;
/// `TESWaterSystem::iActiveWaterGroups` (Xbox PDB static; matches by use).
const ACTIVE_WATER_GROUPS: u32 = 0x011c_7a68;
/// The stencil mask of the group whose depth was rendered last.
const LAST_DEPTH_STENCIL_MASK: u32 = 0x011c_7a74;
/// `PlaceableWaterGroup::spWorldReflectionMap` and `spSkyReflectionMap`
/// (`NiPointer<BSRenderedTexture>` statics; match by use).
const WORLD_REFLECTION_MAP: u32 = 0x011c_7ad4;
const SKY_REFLECTION_MAP: u32 = 0x011c_7c2c;
/// `PlaceableWaterGroup::spDepthMap` and `spWadingWaterHeightMap`
/// (`NiPointer<BSRenderedTexture>` statics; match by use).
const DEPTH_MAP: u32 = 0x011c_7b68;
const WADING_WATER_HEIGHT_MAP: u32 = 0x011c_7b64;
/// Four floats: the clear colour the depth pass sets.
const DEPTH_CLEAR_COLOR: u32 = 0x011a_9bd0;
/// A float `UpdateWaterShaderProperties_ov2` stores (`00885d70` of the form).
const WATER_SHADER_FLOAT: u32 = 0x011f_f108;
/// The 24 words `fn_004e3d60` publishes for the shader: from
/// `0x011ffe48`, the scroll rows and the blend factors (see there).
const SHADER_NOISE_BASE: u32 = 0x011f_fe48;
/// The width and height words of the noise map's rendered texture.
const NOISE_WIDTH: u32 = 0x011a_d81c;
const NOISE_HEIGHT: u32 = 0x011a_d820;

/// INI settings (`SettingT<INISettingCollection>` statics), by the name the
/// game's static initializer registers them under.
/// `bAutoWaterSilhouetteReflections:Water`.
const SETTING_AUTO_SILHOUETTE_REFLECTIONS: u32 = 0x011c_7a90;
/// `bForceLowDetailReflections:Water`.
const SETTING_FORCE_LOW_DETAIL_REFLECTIONS: u32 = 0x011c_7aa8;
/// `bUseWaterReflections:Water`.
const SETTING_USE_WATER_REFLECTIONS: u32 = 0x011c_7b6c;
/// `bUsePerWorldSpaceWaterNoise:Water`.
const SETTING_USE_PER_WORLD_SPACE_NOISE: u32 = 0x011c_7b98;
/// `bUseWaterDepth:Water`.
const SETTING_USE_WATER_DEPTH: u32 = 0x011c_7bbc;
/// `bForceHighDetailReflections:Water`.
const SETTING_FORCE_HIGH_DETAIL_REFLECTIONS: u32 = 0x011c_7c00;
/// `bUseWaterRefractions:Water`.
const SETTING_USE_WATER_REFRACTIONS: u32 = 0x011c_7c60;

/// Globals of the functions from `004e4730` on.
/// `PlayerCharacter*` (`thePlayer`); the sound update and the group lookups
/// use it.
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// `TESWaterSystem::bWaterEnabled` (Xbox PDB static; matches by use):
/// `EnableWaterSystem` sets it, `fn_004e6620` clears it.
const WATER_ENABLED: u32 = 0x0118_9624;
/// The byte `EnableWaterSystem` sets to 1 and `fn_004e6620` clears (not
/// matched to a name).
const WATER_REQUEST_FLAG: u32 = 0x011a_d86e;
/// `TESWaterSystem::iLODWaterObjects` (Xbox PDB static; matches by use).
const LOD_WATER_OBJECTS: u32 = 0x011c_7a4c;
/// `TESWaterSystem::bDisplayWaterSoundPlacement` (Xbox PDB static; matches
/// by use): draw debug markers for the water sound update.
const DISPLAY_WATER_SOUND_PLACEMENT: u32 = 0x011c_7a64;
/// The byte the water sound update keeps: 1 while it started the sound.
const WATER_SOUND_STARTED: u32 = 0x011c_7cfc;
/// `NiPointer` slot (a static `NiNode`, `spWaterRoot` by use) and the object
/// whose three pointers `fn_004e6620` clears.
const WATER_ROOT_SLOT: u32 = 0x011c_7c28;
const WATER_RENDER_OBJECT: u32 = 0x011f_f370;
const WATER_OBJECT_011FFFF8: u32 = 0x011f_fff8;
const WATER_FFT_EFFECT: u32 = 0x0120_006c;
/// `INISettingCollection` settings: `bUseWater:Water`,
/// `fWaterGroupHeightRange:Water`, `uNearWaterRadius:Water`,
/// `uNearWaterPoints:Water`, `fNearWaterIndoorTolerance:Water`,
/// `fNearWaterOutdoorTolerance:Water` and `fAlpha:Water`.
const SETTING_USE_WATER: u32 = 0x011c_7adc;
const SETTING_WATER_GROUP_HEIGHT_RANGE: u32 = 0x011c_7a7c;
const SETTING_NEAR_WATER_RADIUS: u32 = 0x011c_7b2c;
const SETTING_NEAR_WATER_POINTS: u32 = 0x011c_7bb0;
const SETTING_NEAR_WATER_INDOOR_TOLERANCE: u32 = 0x011c_7bd8;
const SETTING_NEAR_WATER_OUTDOOR_TOLERANCE: u32 = 0x011c_7ba4;
const SETTING_WATER_ALPHA: u32 = 0x011c_7d00;
/// The `NiPoint3 (0, 0, 1)` `CreateQuadData` builds once, and its guard
/// bits (1: the point, 2: the alpha setting).
const QUAD_NORMAL: u32 = 0x011c_7d0c;
const QUAD_STATICS_BUILT: u32 = 0x011c_7d18;
/// The `NiMatrix3` identity (nine floats).
const IDENTITY_MATRIX: u32 = 0x011a_9448;
/// A `NiPoint2` of two zero floats (`004e6a80` starts its sum from it).
const ZERO_POINT2: u32 = 0x011f_4980;
/// `0.01` (`float`): the tolerance of the plane comparisons.
const PLANE_TOLERANCE: u32 = 0x0101_3ea4;
/// A `float` global `004e6580` compares the member at `+0xdc` with (1.0 on
/// this exe).
const FLOAT_THRESHOLD_011AD834: u32 = 0x011a_d834;
/// `0.1` (`double`), `2.0`, `1.0`, `0.0`, `-1.0` and `10.0` (`double`s), the
/// `float` `10.0`, `-FLT_MAX` and the length limit `1e-6` (`double`).
const ZERO_POINT_ONE: u32 = 0x0101_ffa0;
const TWO: u32 = 0x0101_1590;
const ONE: u32 = 0x0101_2070;
const ZERO: u32 = 0x0101_2060;
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
const TEN: u32 = 0x0102_0758;
const TEN_FLOAT: u32 = 0x0101_7b78;
const LOWEST_FLOAT: u32 = 0x0101_5f5c;
const LENGTH_LIMIT: u32 = 0x0101_7cf8;
/// `"WATER: Adjusting water object ('%s') height from %f to closeby water
/// group at height : %f"`.
const ADJUSTING_HEIGHT_MESSAGE: u32 = 0x0102_3130;
/// `"fAlpha:Water"`.
const ALPHA_SETTING_NAME: u32 = 0x0102_318c;

/// `0.017453292` (`double`): degrees to radians.
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// `255.0` (`double`).
const BYTE_RANGE: u32 = 0x0101_e568;
/// `100.0` (`double`).
const ONE_HUNDRED: u32 = 0x0101_7a40;
/// `-1.0` (`float`).
const MINUS_ONE: u32 = 0x0101_2054;
/// `"%s"`.
const FORMAT_STRING: u32 = 0x0101_9f08;
/// `"WATER: Can not render water group at height %f using water type %s.
/// You are using too many water groups : %d"`.
const TOO_MANY_GROUPS_MESSAGE: u32 = 0x0102_3060;
/// `"WATER: LOD Water objects still trying to update in an interior when
/// they should not be."`
const LOD_IN_INTERIOR_MESSAGE: u32 = 0x0102_30d0;

/// `TESWaterForm` virtual at +0x130 (the Xbox PDB's `GetObjectTypeName`):
/// the name the log message shows.
const WATER_FORM_GET_NAME: u32 = 0x130;
/// `TESObjectREFR` base form virtual at +0x140: its water form (null when it
/// has none).
const BASE_FORM_GET_WATER_FORM: u32 = 0x140;
/// `TESObjectREFR` virtual at +0x100 (the Xbox PDB's `IsActor` sits at the
/// same offset): true for the references the groups keep in
/// `ActorsInWaterList`.
const REFERENCE_IS_ACTOR_VIRTUAL: u32 = 0x100;
/// `TESObjectREFR` virtual at +0x160: a test `AddPlaceableWater_ov2` skips
/// the height adjustment on. The PC table is shifted by 4 against the Xbox
/// PDB somewhere before `Get3D` (+0x1d0 on the PC, +0x1cc there), so this is
/// the Xbox `GetQuestObject` (+0x15c) or `SetActorCause` (+0x160); not
/// confirmed.
const REFERENCE_SKIP_ADJUST_VIRTUAL: u32 = 0x160;
/// `TESObjectREFR::Set3D(node, 1)` (+0x1cc on the PC, +0x1c8 in the Xbox PDB).
const REFERENCE_SET_3D_VIRTUAL: u32 = 0x1cc;
/// `NiAVObject::IsNode` (+0xc, Xbox PDB): the node itself when it is a node.
const NODE_IS_NODE_VIRTUAL: u32 = 0xc;
/// The 3D object's virtual at +0x18 `UpdateWaterShaderProperties_ov2` tests.
const NODE_PROPERTIES_VIRTUAL: u32 = 0x18;
/// The property's virtual at +0xa4 (a count, taken as a 16-bit value).
const PROPERTY_PASS_COUNT_VIRTUAL: u32 = 0xa4;
/// The rendered texture's virtuals at +0x94 / +0x98 (width, height).
const TEXTURE_WIDTH_VIRTUAL: u32 = 0x94;
const TEXTURE_HEIGHT_VIRTUAL: u32 = 0x98;

/// The bits of the base form's flags the water code tests.
const FORM_FLAG_0001: u32 = 0x0000_0001;
const FORM_FLAG_0200: u32 = 0x0000_0200;
const FORM_FLAG_0800: u32 = 0x0000_0800;
const FORM_FLAG_40000: u32 = 0x0004_0000;
const FORM_FLAG_8000000: u32 = 0x0800_0000;
const FORM_FLAG_10000000: u32 = 0x1000_0000;
const FORM_FLAG_20000000: u32 = 0x2000_0000;
const FORM_FLAG_80000000: u32 = 0x8000_0000;

// ---------------------------------------------------------------------------
// Layouts.

layout! {
    /// `NiTPointerList<T>` / `NiTPointerListSingleThread<T>` (Xbox PDB),
    /// 0xC bytes for every `T`: head, tail, count (not yet in
    /// `crate::types`).
    pub struct NiTPointerList: 0x0c {
    }

    /// `NiTPointerMap<K, V>` (Xbox PDB), 0x10 bytes for every `K`, `V` (not
    /// yet in `crate::types`).
    pub struct NiTPointerMap: 0x10 {
    }

    /// `NiPlane` (Xbox PDB): the normal and the constant (at +0xC).
    pub struct NiPlane: 0x10 {
        /// `m_fConstant` (Xbox PDB).
        0x0c m_fConstant: f32,
    }

    /// `BSSoundHandle` (Xbox PDB), 0xC bytes.
    pub struct BSSoundHandle: 0x0c {
        /// `iSoundID` (Xbox PDB); -1 is an invalid handle.
        0x00 iSoundID: u32,
    }

    /// `NiColorA` (Xbox PDB): four floats.
    pub struct NiColorA: 0x10 {
    }

    /// `TESWaterSystem` (Xbox PDB), 0xA0 bytes on the Xbox (the PC size was
    /// not checked); the fields below sit at the same offsets on the PC.
    pub struct TESWaterSystem: 0xa0 {
        /// `iAccumulationCount` (Xbox PDB): the running stage number
        /// `UpdatePlaceableWater` hands to each pass it sets up.
        0x00 iAccumulationCount: i32,
        /// `iGlobalGetWaterGeometryCount` (Xbox PDB): `fn_004e8030` counts its
        /// calls here.
        0x04 iGlobalGetWaterGeometryCount: i32,
        /// `spWaterHeightMapTexture` (Xbox PDB): `NiPointer<BSRenderedTexture>`.
        0x08 spWaterHeightMapTexture: u32,
        /// `spWaterNormalMapTexture` (Xbox PDB): `NiPointer<BSRenderedTexture>`.
        0x10 spWaterNormalMapTexture: u32,
        /// `spRainHeightMapTexture` (Xbox PDB): `NiPointer<BSRenderedTexture>`.
        0x14 spRainHeightMapTexture: u32,
        /// `spRefractionDepthStencilBuffer` (Xbox PDB):
        /// `NiPointer<NiDepthStencilBuffer>`.
        0x18 spRefractionDepthStencilBuffer: u32,
        /// `spWaterNoiseTexture` (Xbox PDB): `NiPointer<NiTexture>`.
        0x1c spWaterNoiseTexture: u32,
        /// `PlaceableWaterGroupList` (Xbox PDB):
        /// `NiTPointerList<PlaceableWaterGroup *>`.
        0x3c PlaceableWaterGroupList: Inline<NiTPointerList>,
        /// `pLODWaterGroup` (Xbox PDB).
        0x48 pLODWaterGroup: Ptr<PlaceableWaterGroup>,
        /// `ReflectionRefMap` (Xbox PDB).
        0x4c ReflectionRefMap: Inline<NiTPointerMap>,
        /// `DepthRefMap` (Xbox PDB).
        0x5c DepthRefMap: Inline<NiTPointerMap>,
        /// `WaterTypeUpdateMap` (Xbox PDB):
        /// `NiTPointerMap<TESWaterForm *, bool>`.
        0x6c WaterTypeUpdateMap: Inline<NiTPointerMap>,
        /// `WadingWaterMap` (Xbox PDB):
        /// `NiTPointerMap<TESObjectREFR *, WadingWaterData *>`.
        0x7c WadingWaterMap: Inline<NiTPointerMap>,
        /// `WaterSound` (Xbox PDB).
        0x8c WaterSound: Inline<BSSoundHandle>,
        /// `fTimeSinceLastRipplePlaced` (Xbox PDB).
        0x98 fTimeSinceLastRipplePlaced: f32,
        /// `bCull3rdPerson` (Xbox PDB).
        0x9c bCull3rdPerson: bool,
    }

    /// `PlaceableWaterGroup` (Xbox PDB), 0xB0 bytes on the Xbox; the fields
    /// below sit at the same offsets on the PC.
    pub struct PlaceableWaterGroup: 0xb0 {
        /// `pWaterType` (Xbox PDB): `TESWaterForm*`.
        0x00 pWaterType: Ptr,
        /// `ReflectWaterPlane` (Xbox PDB).
        0x04 ReflectWaterPlane: Inline<NiPlane>,
        /// `RefractWaterPlane` (Xbox PDB).
        0x14 RefractWaterPlane: Inline<NiPlane>,
        /// `PlaceableWaterList` (Xbox PDB): the water references of the group.
        0x24 PlaceableWaterList: Inline<NiTPointerList>,
        /// `ObjectInWaterList` (Xbox PDB).
        0x30 ObjectInWaterList: Inline<NiTPointerList>,
        /// `ActorsInWaterList` (Xbox PDB).
        0x3c ActorsInWaterList: Inline<NiTPointerList>,
        /// `spGroupReflectionMap` (Xbox PDB): `NiPointer<BSRenderedTexture>`.
        0x54 spGroupReflectionMap: u32,
        /// `spWadingWaterGeometry` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x58 spWadingWaterGeometry: u32,
        /// `bGroupAtWorldSpaceWaterHeight` (Xbox PDB).
        0x5c bGroupAtWorldSpaceWaterHeight: bool,
        /// `bRenderGroup` (Xbox PDB).
        0x5d bRenderGroup: bool,
        /// `bRenderDepth` (Xbox PDB).
        0x5e bRenderDepth: bool,
        /// `bRenderGroupReflections` (Xbox PDB).
        0x5f bRenderGroupReflections: bool,
        /// `bRenderSilhouetteReflections` (Xbox PDB).
        0x60 bRenderSilhouetteReflections: bool,
        /// `StaticReflectiveObjects` (Xbox PDB).
        0x64 StaticReflectiveObjects: Inline<NiTPointerList>,
        /// `DynamicReflectiveObjects` (Xbox PDB).
        0x70 DynamicReflectiveObjects: Inline<NiTPointerList>,
        /// `StaticDepthObjects` (Xbox PDB).
        0x7c StaticDepthObjects: Inline<NiTPointerList>,
        /// `DynamicDepthObjects` (Xbox PDB).
        0x88 DynamicDepthObjects: Inline<NiTPointerList>,
        /// `spGroupReflectionSorter` (Xbox PDB): `NiPointer<BSShaderAccumulator>`.
        0x94 spGroupReflectionSorter: u32,
        /// `spDepthSorter` (Xbox PDB): `NiPointer<BSShaderAccumulator>`.
        0x98 spDepthSorter: u32,
        /// `iReflectionThreadStage` (Xbox PDB).
        0x9c iReflectionThreadStage: i32,
        /// `iDepthThreadStage` (Xbox PDB).
        0xa0 iDepthThreadStage: i32,
        /// `spReflectionCamera` (Xbox PDB): `NiPointer<NiCamera>`.
        0xa4 spReflectionCamera: u32,
        /// `spDepthCamera` (Xbox PDB): `NiPointer<NiCamera>`.
        0xa8 spDepthCamera: u32,
        /// `iStencilBitMask` (Xbox PDB); the code reads it as a 32-bit word
        /// and as a 16-bit word.
        0xac iStencilBitMask: u32,
    }

    /// `WaterShaderProperty` (Xbox PDB, 0x158 bytes there): the fields the
    /// water update code touches at their PC offsets. Fields from `bFullReflections`
    /// on are 8 lower on the PC than the Xbox PDB says. The members named
    /// `value_...` take a value from a `TESWaterForm` getter; the Xbox PDB
    /// has `NiColorA` members around these offsets and the mapping was not
    /// checked.
    pub struct WaterShaderProperty: 0x150 {
        /// `pRenderPassList` (Xbox PDB, `BSShaderProperty`).
        0x3c pRenderPassList: Ptr,
        /// `bDisplacement` (Xbox PDB; Xbox +0x68).
        0x60 bDisplacement: bool,
        /// `bFullReflections` (Xbox PDB; Xbox +0x6a).
        0x62 bFullReflections: bool,
        /// `bDepth` (Xbox PDB; Xbox +0x6b).
        0x63 bDepth: bool,
        /// `fBlendRadius` (Xbox PDB; Xbox +0x74).
        0x6c fBlendRadius: f32,
        /// `fBlendNormalsAmount` (Xbox PDB; Xbox +0x78).
        0x70 fBlendNormalsAmount: f32,
        /// `fFogFar` (Xbox PDB; Xbox +0x7c).
        0x74 fFogFar: f32,
        /// `fFogRange` (Xbox PDB; Xbox +0x80).
        0x78 fFogRange: f32,
        /// `bUpdateConstants` (Xbox PDB; Xbox +0x87).
        0x7f bUpdateConstants: bool,
        /// `bReflections` (Xbox PDB; Xbox +0x88).
        0x80 bReflections: bool,
        /// `bRefractions` (Xbox PDB; Xbox +0x89).
        0x81 bRefractions: bool,
        /// `bObjectTexCoords` (Xbox PDB; Xbox +0x8a).
        0x82 bObjectTexCoords: bool,
        /// `iStencilMask` (Xbox PDB; Xbox +0x8c).
        0x84 iStencilMask: u32,
        /// `pShallowColor` (Xbox PDB; Xbox +0x90).
        0x88 pShallowColor: Inline<NiColorA>,
        /// `pDeepColor` (Xbox PDB; Xbox +0xa0).
        0x98 pDeepColor: Inline<NiColorA>,
        /// `pReflectionColor` (Xbox PDB; Xbox +0xb0).
        0xa8 pReflectionColor: Inline<NiColorA>,
        0xb8 value_00b8: f32,
        0xbc value_00bc: f32,
        0xc0 value_00c0: f32,
        0xc4 value_00c4: f32,
        0xd0 value_00d0: f32,
        0xd4 value_00d4: f32,
        0xd8 value_00d8: f32,
        0xdc value_00dc: f32,
        0x100 value_0100: f32,
        0x104 value_0104: f32,
        /// `fFresnelAmount` (Xbox PDB; Xbox +0x120).
        0x118 fFresnelAmount: f32,
        /// `fNoiseScale` (Xbox PDB; Xbox +0x124).
        0x11c fNoiseScale: f32,
        /// `fFogAmount` (Xbox PDB; Xbox +0x128).
        0x120 fFogAmount: f32,
        /// `fUVScale` (Xbox PDB; Xbox +0x12c).
        0x124 fUVScale: f32,
        /// `spNoiseHeightMap` (Xbox PDB; Xbox +0x13c): `NiPointer<NiTexture>`.
        0x134 spNoiseHeightMap: u32,
        /// `spNoiseNormalMap` (Xbox PDB; Xbox +0x140).
        0x138 spNoiseNormalMap: u32,
        /// `spReflectionMap` (Xbox PDB; Xbox +0x144).
        0x13c spReflectionMap: u32,
        /// `spRefractionMap` (Xbox PDB; Xbox +0x148).
        0x140 spRefractionMap: u32,
        /// `spDepthMap` (Xbox PDB; Xbox +0x14c).
        0x144 spDepthMap: u32,
        /// `spDisplacementNormalMap` (Xbox PDB; Xbox +0x150):
        /// `NiPointer<NiTexture>`.
        0x148 spDisplacementNormalMap: u32,
    }

    /// `WadingWaterData` (Xbox PDB), 0x1c bytes: what the wading-water
    /// update keeps per water reference. The two `NiPoint2` members and the
    /// `NiPoint3` are listed as floats.
    pub struct WadingWaterData: 0x1c {
        /// `fDisplaceOffset` (Xbox PDB, `NiPoint2`): x.
        0x00 fDisplaceOffsetX: f32,
        /// `fDisplaceOffset` y.
        0x04 fDisplaceOffsetY: f32,
        /// `fLastDisplaceOffset` (Xbox PDB, `NiPoint2`): x.
        0x08 fLastDisplaceOffsetX: f32,
        /// `fLastDisplaceOffset` y.
        0x0c fLastDisplaceOffsetY: f32,
        /// `fLastPosition` (Xbox PDB, `NiPoint3`): x.
        0x10 fLastPositionX: f32,
        /// `fLastPosition` y.
        0x14 fLastPositionY: f32,
        /// `fLastPosition` z.
        0x18 fLastPositionZ: f32,
    }

    /// `TESWaterForm` (Xbox PDB): the fields the water update code touches
    /// at their PC offsets (`spNoiseNormalMap` and later fields are 0x10 lower
    /// than the Xbox PDB says; the `WaterShaderData` member starts at +0x84).
    pub struct TESWaterForm: 0x194 {
        /// `spNoiseNormalMap` (Xbox PDB; Xbox +0x40):
        /// `NiPointer<BSRenderedTexture>`.
        0x30 spNoiseNormalMap: u32,
        /// `fTexScroll0` (Xbox PDB; Xbox +0x44), red.
        0x34 fTexScroll0R: f32,
        /// `fTexScroll0` green.
        0x38 fTexScroll0G: f32,
        /// `fTexScroll1` (Xbox PDB; Xbox +0x54), red.
        0x44 fTexScroll1R: f32,
        /// `fTexScroll1` green.
        0x48 fTexScroll1G: f32,
        /// `fTexScroll2` (Xbox PDB; Xbox +0x64), red.
        0x54 fTexScroll2R: f32,
        /// `fTexScroll2` green.
        0x58 fTexScroll2G: f32,
        /// `Data.fFresnelAmount` (`WaterShaderData` +0x18).
        0x9c fFresnelAmount: f32,
        /// `Data.fNoiseWindDirection0` (`WaterShaderData` +0x64).
        0xe8 fNoiseWindDirection0: f32,
        /// `Data.fNoiseWindDirection2` (`WaterShaderData` +0x6c).
        0xf0 fNoiseWindDirection2: f32,
        /// `Data.fNoiseWindSpeed1` (`WaterShaderData` +0x74).
        0xf8 fNoiseWindSpeed1: f32,
        /// `Data.fFogAmount` (`WaterShaderData` +0x84).
        0x108 fFogAmount: f32,
        /// `Data.fUnderwaterFogDistNear` (`WaterShaderData` +0x90).
        0x114 fUnderwaterFogDistNear: f32,
        /// `Data.fLightRadius` (`WaterShaderData` +0xa4).
        0x128 fLightRadius: f32,
        /// `Data.fLightBrightness` (`WaterShaderData` +0xa8).
        0x12c fLightBrightness: f32,
        /// `Data.fHeightUVScale0` (`WaterShaderData` +0xac).
        0x130 fHeightUVScale0: f32,
        /// `bResetNoiseTexture` (Xbox PDB; Xbox +0x1a0).
        0x190 bResetNoiseTexture: u8,
    }
}

// ---------------------------------------------------------------------------
// Helpers.

/// The address of `field` of `object`.
fn address_of<S, T>(object: Ptr<S>, field: Field<S, T>) -> u32 {
    object.addr() + field.off
}

/// The object a `NiPointer` slot (or the head of a list) holds.
fn pointer_in_slot(e: &mut Engine, slot: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![slot]).u32()
}

/// Stores `value` in a `NiPointer` slot.
fn assign_slot(e: &mut Engine, slot: u32, value: u32) {
    e.call(NI_POINTER_ASSIGN, &args![slot, value]);
}

/// An INI setting read as a `bool` (the byte at the setting's value).
fn setting_flag(e: &mut Engine, setting: u32) -> bool {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.u8(value) != 0
}

/// Whether the current cell is an interior (`TES::pInteriorCell` through
/// `005f36f0`; see the module notes).
fn in_interior(e: &mut Engine) -> bool {
    let tes = e.global::<u32>(TES_POINTER);
    e.call(TES_GET_INTERIOR_CELL, &args![tes]).u32() != 0
}

/// `byte [0x011c7a59]`: the flag `004e2180` returns.
fn water_skipped(e: &mut Engine) -> bool {
    e.call(WATER_SKIPPED_FLAG, &[]).bool()
}

/// Calls `visit` for every item of an `NiTPointerList`, the way every loop
/// of the unit does: the next position is fetched before the item is
/// visited, so the visitor may remove the item.
fn for_each_list_item(e: &mut Engine, list: u32, mut visit: impl FnMut(&mut Engine, u32)) {
    let mut position = pointer_in_slot(e, list);
    while position != 0 {
        let next = e.call(LIST_NEXT_POSITION, &args![list, position]).u32();
        let slot = e.call(LIST_ITEM_SLOT, &args![list, position]).u32();
        let item = e.mem.u32(slot);
        visit(e, item);
        position = next;
    }
}

/// `NiAVObject::GetProperty(node, 3)`: the node's water shader property.
fn water_shader_property(e: &mut Engine, node: u32) -> Ptr<WaterShaderProperty> {
    e.call(NODE_GET_PROPERTY, &args![node, 3u32]).ptr()
}

/// A `float` getter called with `ECX = this` (the result is in ST0).
fn float_getter(e: &mut Engine, getter: u32, this: u32) -> f32 {
    e.call(getter, &args![this]).f32()
}

/// Clears the reflection, refraction and depth texture slots of a water
/// shader property whose feature is off or whose INI setting forbids it,
/// then the normal-map slot.
fn clear_disabled_texture_slots(e: &mut Engine, property: Ptr<WaterShaderProperty>) {
    let reflections = e.get(property, WaterShaderProperty::bReflections);
    if !(reflections && setting_flag(e, SETTING_USE_WATER_REFLECTIONS)) {
        assign_slot(
            e,
            address_of(property, WaterShaderProperty::spReflectionMap),
            0,
        );
    }
    let refractions = e.get(property, WaterShaderProperty::bRefractions);
    if !(refractions && setting_flag(e, SETTING_USE_WATER_REFRACTIONS)) {
        assign_slot(
            e,
            address_of(property, WaterShaderProperty::spRefractionMap),
            0,
        );
    }
    let depth = e.get(property, WaterShaderProperty::bDepth);
    if !(depth && setting_flag(e, SETTING_USE_WATER_DEPTH)) {
        assign_slot(e, address_of(property, WaterShaderProperty::spDepthMap), 0);
    }
    assign_slot(
        e,
        address_of(property, WaterShaderProperty::spNoiseNormalMap),
        0,
    );
}

/// Gives the rendered texture in `slot` back to the texture manager, when
/// there is one (the slot itself is not cleared). Returns whether there was
/// one.
fn return_rendered_texture(e: &mut Engine, slot: u32) -> bool {
    if pointer_in_slot(e, slot) == 0 {
        return false;
    }
    let texture = pointer_in_slot(e, slot);
    let manager = e.call(TEXTURE_MANAGER, &[]).u32();
    e.call(RETURN_RENDERED_TEXTURE, &args![manager, texture]);
    true
}

/// Copies a 16-byte `NiPoint4`/`NiColorA` word for word.
fn copy_four_words(e: &mut Engine, from: u32, to: u32) {
    for i in 0..4 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// Stores a packed colour (red in the low byte) as a `NiColorA` with alpha
/// 1.0 at `destination`.
fn store_packed_color(e: &mut Engine, destination: u32, packed: u32) {
    let range: f64 = e.global(BYTE_RANGE);
    let red = ((packed & 0xff) as f64 / range) as f32;
    let green = (((packed >> 8) & 0xff) as f64 / range) as f32;
    let blue = (((packed >> 16) & 0xff) as f64 / range) as f32;
    e.with_stack(0x10, |e, color| {
        e.call(NI_POINT4_CONSTRUCT, &args![color, red, green, blue, 1.0f32]);
        copy_four_words(e, color.addr(), destination);
    });
}

// ---------------------------------------------------------------------------
// Translated functions.

// Translated from 004e21b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::UpdatePlaceableWater` (Xbox PDB): decides which water
/// groups render this frame (a group renders while one of its water
/// references is near the `viewer` node, or when `force_update` is set),
/// sets up their reflection and depth passes, renders the depth maps and
/// updates the shader properties of the rendered groups. When no group
/// renders it gives the shared textures back.
///
/// `viewer` is an `NiAVObject` (the player's 3D); only its world position is
/// read.
pub fn tes_water_system_update_placeable_water(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    viewer: Ptr,
    force_update: bool,
) {
    let group_list = this.at(TESWaterSystem::PlaceableWaterGroupList).addr();
    let mut any_group_switched_on = false;
    e.set_global(ACTIVE_WATER_GROUPS, 0u32);

    // Which groups render: every group starts off and is switched on by a
    // water reference of its own that is in range.
    for_each_list_item(e, group_list, |e, item| {
        let group: Ptr<PlaceableWaterGroup> = Ptr::new(item);
        e.set(group, PlaceableWaterGroup::bRenderGroup, false);
        e.set(group, PlaceableWaterGroup::bRenderGroupReflections, false);
        e.set(group, PlaceableWaterGroup::bRenderDepth, false);
        e.set(
            group,
            PlaceableWaterGroup::bRenderSilhouetteReflections,
            false,
        );
        let members = group.at(PlaceableWaterGroup::PlaceableWaterList).addr();
        for_each_list_item(e, members, |e, reference| {
            if update_group_member(e, this, viewer, force_update, group, reference) {
                any_group_switched_on = true;
            }
        });
        settle_group_after_members(e, this, group);
    });

    let world_reflections = e.global::<u8>(RENDER_WORLD_REFLECTIONS) != 0;
    let sky_reflections = e.global::<u8>(RENDER_SKY_REFLECTIONS) != 0;
    if !any_group_switched_on && !world_reflections && !sky_reflections {
        // Nothing renders: give the shared textures back.
        return_rendered_texture(e, DEPTH_MAP);
        return_rendered_texture(e, WADING_WATER_HEIGHT_MAP);
        for_each_list_item(e, group_list, |e, item| {
            let group: Ptr<PlaceableWaterGroup> = Ptr::new(item);
            let water_type = e.get(group, PlaceableWaterGroup::pWaterType);
            if !water_type.is_null() {
                let slot = address_of(
                    water_type.cast::<TESWaterForm>(),
                    TESWaterForm::spNoiseNormalMap,
                );
                if return_rendered_texture(e, slot) {
                    assign_slot(e, slot, 0);
                }
            }
        });
        assign_slot(e, DEPTH_MAP, 0);
        assign_slot(e, WADING_WATER_HEIGHT_MAP, 0);
        return;
    }

    let interior = in_interior(e);
    if !interior {
        if e.global::<u8>(RENDER_WORLD_REFLECTIONS) != 0 {
            let stage = e.get(this, TESWaterSystem::iAccumulationCount);
            e.set_global(WORLD_REFLECTION_THREAD_STAGE, stage);
            e.set(
                this,
                TESWaterSystem::iAccumulationCount,
                stage.wrapping_add(1),
            );
            e.call(SETUP_WORLD_REFLECTIONS, &args![this, viewer]);
        }
        if e.global::<u8>(RENDER_SKY_REFLECTIONS) != 0 {
            let stage = e.get(this, TESWaterSystem::iAccumulationCount);
            e.set_global(SKY_REFLECTION_THREAD_STAGE, stage);
            e.set(
                this,
                TESWaterSystem::iAccumulationCount,
                stage.wrapping_add(1),
            );
            e.call(SETUP_SKY_REFLECTIONS, &args![this, viewer]);
        }
    }

    // Set up the passes of every rendered group.
    for_each_list_item(e, group_list, |e, item| {
        let group: Ptr<PlaceableWaterGroup> = Ptr::new(item);
        if !e.get(group, PlaceableWaterGroup::bRenderGroup) || water_skipped(e) {
            return;
        }
        e.call(
            MAP_REMOVE_ALL,
            &args![address_of(this, TESWaterSystem::ReflectionRefMap)],
        );
        e.call(
            MAP_REMOVE_ALL,
            &args![address_of(this, TESWaterSystem::DepthRefMap)],
        );
        e.call(
            LIST_REMOVE_ALL,
            &args![address_of(group, PlaceableWaterGroup::StaticDepthObjects)],
        );
        e.call(
            LIST_REMOVE_ALL,
            &args![address_of(group, PlaceableWaterGroup::DynamicDepthObjects)],
        );
        let stage = e.get(this, TESWaterSystem::iAccumulationCount);
        if stage.wrapping_add(1) < 0x10 {
            if setting_flag(e, SETTING_USE_WATER_REFLECTIONS)
                && e.get(group, PlaceableWaterGroup::bRenderGroupReflections)
            {
                let stage = e.get(this, TESWaterSystem::iAccumulationCount);
                e.set(group, PlaceableWaterGroup::iReflectionThreadStage, stage);
                e.set(
                    this,
                    TESWaterSystem::iAccumulationCount,
                    stage.wrapping_add(1),
                );
                let high_detail =
                    setting_flag(e, SETTING_FORCE_HIGH_DETAIL_REFLECTIONS) || in_interior(e);
                let low_detail: u32 = if high_detail { 0 } else { 1 };
                e.call(
                    SETUP_GROUP_REFLECTIONS,
                    &args![this, viewer, group, low_detail],
                );
            }
            if e.get(group, PlaceableWaterGroup::bRenderDepth)
                && setting_flag(e, SETTING_USE_WATER_DEPTH)
            {
                let stage = e.get(this, TESWaterSystem::iAccumulationCount);
                e.set(group, PlaceableWaterGroup::iDepthThreadStage, stage);
                e.set(
                    this,
                    TESWaterSystem::iAccumulationCount,
                    stage.wrapping_add(1),
                );
                e.call(SETUP_GROUP_DEPTH, &args![this, viewer, group]);
            }
        } else {
            // Too many groups for the 16 stages: log it and switch the group off.
            let count = e.call(LIST_COUNT, &args![group_list]).u32();
            let water_type = e.get(group, PlaceableWaterGroup::pWaterType);
            let name = e.vcall(water_type.addr(), WATER_FORM_GET_NAME, &[]).u32();
            let height = e
                .call(
                    PLANE_CONSTANT,
                    &args![address_of(group, PlaceableWaterGroup::ReflectWaterPlane)],
                )
                .f32();
            e.with_stack(0x100, |e, text| {
                e.call(
                    FORMATTED_PRINT,
                    &args![
                        text,
                        0x100u32,
                        TOO_MANY_GROUPS_MESSAGE,
                        height as f64,
                        name,
                        count
                    ],
                );
                e.call(LOG_MESSAGE, &args![FORMAT_STRING, text]);
            });
            e.set(group, PlaceableWaterGroup::bRenderGroup, false);
        }
    });

    if !in_interior(e) {
        if e.global::<u8>(RENDER_WORLD_REFLECTIONS) != 0 && !water_skipped(e) {
            e.call(FINISH_WORLD_REFLECTIONS, &args![this]);
        }
        if e.global::<u8>(RENDER_SKY_REFLECTIONS) != 0 && !water_skipped(e) {
            e.call(FINISH_SKY_REFLECTIONS, &args![this]);
        }
        for_each_list_item(e, group_list, |e, item| {
            let group: Ptr<PlaceableWaterGroup> = Ptr::new(item);
            if e.get(group, PlaceableWaterGroup::bRenderGroup) && !water_skipped(e) {
                e.call(FINISH_GROUP, &args![this, group]);
            } else {
                e.call(RELEASE_GROUP, &args![this, group, 0u32, 0u32]);
            }
        });
    } else {
        for_each_list_item(e, group_list, |e, item| {
            let group: Ptr<PlaceableWaterGroup> = Ptr::new(item);
            if e.get(group, PlaceableWaterGroup::bRenderGroup) && !water_skipped(e) {
                if setting_flag(e, SETTING_USE_WATER_REFLECTIONS)
                    && e.get(group, PlaceableWaterGroup::bRenderGroupReflections)
                {
                    e.call(FINISH_GROUP_INTERIOR, &args![this, group]);
                }
                e.call(FINISH_GROUP, &args![this, group]);
            } else {
                e.call(RELEASE_GROUP, &args![this, group, 0u32, 0u32]);
            }
        });
    }

    // Render the depth maps: the clear colour is set to the depth colour for
    // the duration and restored afterwards.
    render_depth_maps(e, this, viewer);

    e.call(
        MAP_REMOVE_ALL,
        &args![address_of(this, TESWaterSystem::WaterTypeUpdateMap)],
    );
    for_each_list_item(e, group_list, |e, item| {
        let group: Ptr<PlaceableWaterGroup> = Ptr::new(item);
        if e.get(group, PlaceableWaterGroup::bRenderGroup) {
            tes_water_system_update_water_shader_properties(e, this, group);
            fn_004e3d60(e, this, group);
        }
    });
}

/// What `UpdatePlaceableWater` does for one water reference of a group
/// (`reference` can be a null item): switches the group on when the
/// reference is in range, records what the group has to render (reflections,
/// depth, silhouette reflections) and drops the reference's shader textures
/// the settings do not allow. Returns whether this call switched the group
/// on.
fn update_group_member(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    viewer: Ptr,
    force_update: bool,
    group: Ptr<PlaceableWaterGroup>,
    reference: u32,
) -> bool {
    let mut switched_on = false;
    let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
    if reference != 0 && e.vcall(reference, REFERENCE_GET_3D, &[]).u32() != 0 {
        let parent_cell_matches = {
            let cell = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32();
            cell != 0 && {
                let cell = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32();
                e.call(CELL_BYTE_IS_SIX, &args![cell]).bool()
            }
        };
        let relevant = parent_cell_matches || (fn_004e32c0(e, base_form) && !in_interior(e));
        if relevant {
            let node = e.vcall(reference, REFERENCE_GET_3D, &[]).u32();
            let culled = e.call(NODE_IS_CULLED, &args![node]).bool();
            let in_range = !culled
                && e.call(REFERENCE_IS_IN_RANGE, &args![this, reference, viewer])
                    .bool();
            if in_range || force_update {
                if !e.get(group, PlaceableWaterGroup::bRenderGroup) {
                    let active = e.global::<u32>(ACTIVE_WATER_GROUPS);
                    e.set_global(ACTIVE_WATER_GROUPS, active.wrapping_add(1));
                    e.set(group, PlaceableWaterGroup::bRenderGroup, true);
                    switched_on = true;
                    if !water_skipped(e) {
                        if in_interior(e) {
                            e.set(group, PlaceableWaterGroup::bRenderGroupReflections, true);
                        } else if e.get(group, PlaceableWaterGroup::bGroupAtWorldSpaceWaterHeight) {
                            e.set_global(RENDER_WORLD_REFLECTIONS, 1u8);
                        } else {
                            e.set_global(RENDER_SKY_REFLECTIONS, 1u8);
                        }
                    }
                }
                if fn_004e32e0(e, base_form) && fn_004e3280(e, base_form) {
                    let actors = address_of(group, PlaceableWaterGroup::ActorsInWaterList);
                    let objects = address_of(group, PlaceableWaterGroup::ObjectInWaterList);
                    let needs_depth = !e.call(LIST_IS_EMPTY, &args![actors]).bool()
                        || !e.call(LIST_IS_EMPTY, &args![objects]).bool()
                        || needs_depth_for_reference(e, reference, base_form);
                    if needs_depth {
                        e.set(group, PlaceableWaterGroup::bRenderDepth, true);
                    }
                }
                if !in_interior(e)
                    && setting_flag(e, SETTING_AUTO_SILHOUETTE_REFLECTIONS)
                    && (e.call(FORM_FLAG_TEST_40000000, &args![base_form]).bool()
                        || setting_flag(e, SETTING_FORCE_LOW_DETAIL_REFLECTIONS)
                        || fn_004e3300(e, base_form))
                {
                    e.set(
                        group,
                        PlaceableWaterGroup::bRenderSilhouetteReflections,
                        true,
                    );
                }
            }
        }
    }

    // The reference's own shader property: a reference whose group is not
    // rendering loses all its textures; one that renders keeps those the
    // settings allow.
    let rendering = e.get(group, PlaceableWaterGroup::bRenderGroup);
    let node = e.call(WATER_REFERENCE_3D, &args![this, reference]).u32();
    if node != 0 {
        let owner = e.call(NODE_OWNER, &args![node]).u32();
        if owner != 0 && e.call(OWNER_TYPE, &args![owner]).u32() == WATER_OWNER_TYPE {
            let property = water_shader_property(e, node);
            if reference != 0 && !rendering {
                assign_slot(
                    e,
                    address_of(property, WaterShaderProperty::spReflectionMap),
                    0,
                );
                assign_slot(
                    e,
                    address_of(property, WaterShaderProperty::spRefractionMap),
                    0,
                );
                assign_slot(
                    e,
                    address_of(property, WaterShaderProperty::spNoiseNormalMap),
                    0,
                );
                assign_slot(e, address_of(property, WaterShaderProperty::spDepthMap), 0);
            } else {
                clear_disabled_texture_slots(e, property);
            }
        }
    }
    switched_on
}

/// The part of the depth decision that looks at the reference itself, when
/// nothing is in the water: the reflected references of its extra data, then
/// the base form flags.
fn needs_depth_for_reference(e: &mut Engine, reference: u32, base_form: u32) -> bool {
    let extra = e.call(REFERENCE_EXTRA_DATA_LIST, &args![reference]).u32();
    let reflected = e
        .call(EXTRA_DATA_LIST_GET_REFLECTED_REFS, &args![extra])
        .u32();
    if reflected != 0 {
        let extra = e.call(REFERENCE_EXTRA_DATA_LIST, &args![reference]).u32();
        let reflected = e
            .call(EXTRA_DATA_LIST_GET_REFLECTED_REFS, &args![extra])
            .u32();
        if !e.call(WORDS_ARE_ZERO, &args![reflected]).bool() {
            return true;
        }
    }
    if fn_004e32a0(e, base_form) {
        return true;
    }
    if e.call(FORM_FLAG_TEST_40000000, &args![base_form]).bool() {
        let object = fn_004e3260(e);
        return !e.call(WORDS_ARE_ZERO, &args![object]).bool();
    }
    false
}

/// What `UpdatePlaceableWater` does for a group once all its references were
/// looked at: a group that does not render hands back its wading-water
/// textures and is released; one that does render keeps only the textures
/// the settings allow, and gives the shared depth map back when depth is off.
fn settle_group_after_members(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    group: Ptr<PlaceableWaterGroup>,
) {
    let wading_geometry = address_of(group, PlaceableWaterGroup::spWadingWaterGeometry);
    if !e.get(group, PlaceableWaterGroup::bRenderGroup) {
        if pointer_in_slot(e, wading_geometry) != 0 {
            let geometry = pointer_in_slot(e, wading_geometry);
            let property = water_shader_property(e, geometry);
            assign_slot(
                e,
                address_of(property, WaterShaderProperty::spReflectionMap),
                0,
            );
            assign_slot(
                e,
                address_of(property, WaterShaderProperty::spRefractionMap),
                0,
            );
            assign_slot(e, address_of(property, WaterShaderProperty::spDepthMap), 0);
            assign_slot(
                e,
                address_of(property, WaterShaderProperty::spNoiseNormalMap),
                0,
            );
        }
        e.call(RELEASE_GROUP, &args![this, group, 0u32, 0u32]);
        return;
    }
    if pointer_in_slot(e, wading_geometry) != 0 {
        let geometry = pointer_in_slot(e, wading_geometry);
        let property = water_shader_property(e, geometry);
        clear_disabled_texture_slots(e, property);
    }
    if !setting_flag(e, SETTING_USE_WATER_REFLECTIONS) {
        e.call(RELEASE_GROUP, &args![this, group, 0u32, 0u32]);
    }
    if !setting_flag(e, SETTING_USE_WATER_DEPTH) {
        return_rendered_texture(e, DEPTH_MAP);
        assign_slot(e, DEPTH_MAP, 0);
    }
}

/// The depth-map part of `UpdatePlaceableWater`: with the clear colour set
/// to the depth colour and the viewer's position sent to `004e20c0`, every
/// rendered group that wants depth is rendered into the shared depth map and
/// put through the image-space depth effect; the clear colour is restored
/// at the end.
fn render_depth_maps(e: &mut Engine, this: Ptr<TESWaterSystem>, viewer: Ptr) {
    let group_list = this.at(TESWaterSystem::PlaceableWaterGroupList).addr();
    let mut first_group = true;
    e.with_stack(0x40, |e, scratch| {
        let saved_clear_color = scratch.addr();
        let depth_color = saved_clear_color + 0x10;
        let position_point = saved_clear_color + 0x20;
        e.call(
            NI_POINT4_CONSTRUCT,
            &args![saved_clear_color, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
        );
        let renderer = e.call(RENDERER, &[]).u32();
        e.vcall(
            renderer,
            RENDERER_GET_CLEAR_COLOR,
            &args![saved_clear_color],
        );
        for i in 0..4 {
            let word = e.mem.u32(DEPTH_CLEAR_COLOR + 4 * i);
            e.mem.set_u32(depth_color + 4 * i, word);
        }
        let renderer = e.call(RENDERER, &[]).u32();
        e.vcall(renderer, RENDERER_SET_CLEAR_COLOR, &args![depth_color]);
        let translate = e.call(NODE_WORLD_TRANSLATE, &args![viewer]).u32();
        let x = e.mem.u32(translate);
        let y = e.mem.u32(translate + 4);
        let z = e.mem.u32(translate + 8);
        e.call(
            NI_POINT4_CONSTRUCT,
            &args![
                position_point,
                f32::from_bits(x),
                f32::from_bits(y),
                f32::from_bits(z),
                0.0f32
            ],
        );
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(position_point + 4 * i)).collect();
        e.call(
            VIEWER_POSITION_UPDATE,
            &args![0x1eu32, words[0], words[1], words[2], words[3]],
        );

        for_each_list_item(e, group_list, |e, item| {
            let group: Ptr<PlaceableWaterGroup> = Ptr::new(item);
            if !(e.get(group, PlaceableWaterGroup::bRenderGroup)
                && setting_flag(e, SETTING_USE_WATER_DEPTH)
                && e.get(group, PlaceableWaterGroup::bRenderDepth)
                && !water_skipped(e))
            {
                return;
            }
            let texture = pointer_in_slot(e, DEPTH_MAP);
            let stopped = e.call(RENDERED_TEXTURE_STOP, &args![texture]).u32();
            let mode: u32 = if first_group { 7 } else { 0 };
            e.call(RENDER_TARGET_SET, &args![mode, stopped]);
            let camera = pointer_in_slot(e, address_of(group, PlaceableWaterGroup::spDepthCamera));
            let stencil = e.get(group, PlaceableWaterGroup::iStencilBitMask);
            e.call(
                RENDER_GROUP_DEPTH,
                &args![this, camera, group, first_group as u32, stencil & 0xffff],
            );
            e.call(BATCH_RENDERER_END_PASS, &[]);
            e.set_global(LAST_DEPTH_STENCIL_MASK, stencil);
            e.with_stack(SHADER_PARAM_SIZE, |e, param| {
                e.call(SHADER_PARAM_CONSTRUCT, &args![param]);
                e.call(SHADER_PARAM_INIT_CONSTANT_MAP, &args![param, 0u32, 1u32]);
                e.call(
                    SHADER_PARAM_SET_PIXEL_CONSTANT,
                    &args![param, 0u32, 1.0f32, 0.0f32, 0.0f32, 1.0f32],
                );
                e.call(SET_STENCIL_STATE, &args![2u32, 0xffu32, stencil, 0u32]);
                let texture = pointer_in_slot(e, DEPTH_MAP);
                let renderer = e.call(RENDERER, &[]).u32();
                let manager = fn_004e3270(e);
                e.call(
                    IMAGE_SPACE_RENDER_EFFECT,
                    &args![manager, 0x28u32, renderer, texture, param, 0u32],
                );
                e.call(AFTER_GROUP_DEPTH, &args![this, group, stencil & 0xffff]);
                e.call(RENDER_TARGET_RESET, &[]);
                first_group = false;
                e.call(SHADER_PARAM_DESTRUCT, &args![param]);
            });
        });

        let renderer = e.call(RENDERER, &[]).u32();
        e.vcall(
            renderer,
            RENDERER_SET_CLEAR_COLOR,
            &args![saved_clear_color],
        );
    });
}

// Translated from 004e3260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the static object at `0x011ca13c`, which
/// `UpdatePlaceableWater` asks `008256d0` (both words zero) about.
pub fn fn_004e3260(_e: &mut Engine) -> u32 {
    WATER_OBJECT_011CA13C
}

// Translated from 004e3270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ImageSpaceManager*` global (`0x011f91ac`) the water passes render
/// effects with.
pub fn fn_004e3270(e: &mut Engine) -> u32 {
    e.global(IMAGE_SPACE_MANAGER)
}

/// A base form flag test: whether `mask` is set in the form flags
/// (`FORM_FLAG_TEST`).
fn form_flag(e: &mut Engine, form: u32, mask: u32) -> bool {
    e.call(FORM_FLAG_TEST, &args![form, mask]).bool()
}

// Translated from 004e3280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x200` is set in the form flags.
pub fn fn_004e3280(e: &mut Engine, form: u32) -> bool {
    form_flag(e, form, FORM_FLAG_0200)
}

// Translated from 004e32a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x800` is set in the form flags.
pub fn fn_004e32a0(e: &mut Engine, form: u32) -> bool {
    form_flag(e, form, FORM_FLAG_0800)
}

// Translated from 004e32c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x08000000` is set in the form flags.
pub fn fn_004e32c0(e: &mut Engine, form: u32) -> bool {
    form_flag(e, form, FORM_FLAG_8000000)
}

// Translated from 004e32e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x10000000` is set in the form flags.
pub fn fn_004e32e0(e: &mut Engine, form: u32) -> bool {
    form_flag(e, form, FORM_FLAG_10000000)
}

// Translated from 004e3300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x40000` is set in the form flags.
pub fn fn_004e3300(e: &mut Engine, form: u32) -> bool {
    form_flag(e, form, FORM_FLAG_40000)
}

// Translated from 004e3320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::UpdateLODWater` (Xbox PDB): the same decision as
/// `UpdatePlaceableWater` for the single LOD water group. Logs a message
/// when it runs in an interior (LOD water must not). A water reference in
/// range (or any, with `force_update`) switches the group on and points its
/// shader property at the shared world or sky reflection map; one that is
/// not loses its reflection map; when no reference is left the rendered
/// group's shader properties are updated.
pub fn tes_water_system_update_lod_water(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    viewer: Ptr,
    force_update: bool,
) {
    if e.get(this, TESWaterSystem::pLODWaterGroup).is_null() {
        return;
    }
    if in_interior(e) {
        e.call(LOG_MESSAGE, &args![LOD_IN_INTERIOR_MESSAGE]);
        return;
    }
    let lod_group = e.get(this, TESWaterSystem::pLODWaterGroup);
    e.set(lod_group, PlaceableWaterGroup::bRenderGroup, false);
    let lod_group = e.get(this, TESWaterSystem::pLODWaterGroup);
    let members = lod_group.at(PlaceableWaterGroup::PlaceableWaterList).addr();
    for_each_list_item(e, members, |e, reference| {
        let mut active = false;
        if e.vcall(reference, REFERENCE_GET_3D, &[]).u32() != 0 {
            let node = e.vcall(reference, REFERENCE_GET_3D, &[]).u32();
            let culled = e.call(NODE_IS_CULLED, &args![node]).bool();
            active = !culled
                && e.call(REFERENCE_IS_IN_RANGE, &args![this, reference, viewer])
                    .bool();
        }
        if active || force_update {
            let lod_group = e.get(this, TESWaterSystem::pLODWaterGroup);
            e.set(lod_group, PlaceableWaterGroup::bRenderGroup, true);
            let node = e.call(WATER_REFERENCE_3D, &args![this, reference]).u32();
            if node != 0 {
                let property = water_shader_property(e, node);
                if !water_skipped(e) {
                    let reflection_map = address_of(property, WaterShaderProperty::spReflectionMap);
                    if e.get(property, WaterShaderProperty::bFullReflections) {
                        e.set_global(RENDER_WORLD_REFLECTIONS, 1u8);
                        e.call(
                            NI_POINTER_ASSIGN_FROM,
                            &args![reflection_map, WORLD_REFLECTION_MAP],
                        );
                    } else {
                        e.set_global(RENDER_SKY_REFLECTIONS, 1u8);
                        e.call(
                            NI_POINTER_ASSIGN_FROM,
                            &args![reflection_map, SKY_REFLECTION_MAP],
                        );
                    }
                } else {
                    let lod_group = e.get(this, TESWaterSystem::pLODWaterGroup);
                    e.call(RELEASE_GROUP, &args![this, lod_group, 0u32, 0u32]);
                }
            }
        } else if e.vcall(reference, REFERENCE_GET_3D, &[]).u32() != 0 {
            let node = e.call(WATER_REFERENCE_3D, &args![this, reference]).u32();
            if node != 0 {
                let property = water_shader_property(e, node);
                assign_slot(
                    e,
                    address_of(property, WaterShaderProperty::spReflectionMap),
                    0,
                );
            }
        }
    });
    let lod_group = e.get(this, TESWaterSystem::pLODWaterGroup);
    if e.get(lod_group, PlaceableWaterGroup::bRenderGroup) {
        let lod_group = e.get(this, TESWaterSystem::pLODWaterGroup);
        tes_water_system_update_water_shader_properties(e, this, lod_group);
    }
}

// Translated from 004e3520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::UpdateWaterShaderProperties` (Xbox PDB): updates the
/// shader property of every water reference of `group`.
pub fn tes_water_system_update_water_shader_properties(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    group: Ptr<PlaceableWaterGroup>,
) {
    let members = group.at(PlaceableWaterGroup::PlaceableWaterList).addr();
    for_each_list_item(e, members, |e, reference| {
        tes_water_system_update_water_shader_properties_ov2(e, this, group, reference);
    });
}

// Translated from 004e3590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::UpdateWaterShaderProperties_ov2` (Xbox PDB): copies the
/// water form of one water `reference` into its shader property. The
/// property is only touched when the reference's 3D object has the water
/// owner type and the property has `bUpdateConstants` set: colours (shallow,
/// deep, reflection), the form's amounts, the fog values (the underwater
/// ones when the player is underwater), the settings-controlled flags
/// (reflections, refractions, depth) and the depth falloff. When a flag
/// changed, the render-pass lists of the property and of the group's
/// wading-water property are reset.
pub fn tes_water_system_update_water_shader_properties_ov2(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    group: Ptr<PlaceableWaterGroup>,
    reference: u32,
) {
    let node = e.call(WATER_REFERENCE_3D, &args![this, reference]).u32();
    if node == 0 || e.vcall(node, NODE_PROPERTIES_VIRTUAL, &[]).u32() == 0 {
        return;
    }
    let owner = e.call(NODE_OWNER, &args![node]).u32();
    if owner == 0 || e.call(OWNER_TYPE, &args![owner]).u32() != WATER_OWNER_TYPE {
        return;
    }
    let property = water_shader_property(e, node);
    let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
    let water_form = e.vcall(base_form, BASE_FORM_GET_WATER_FORM, &[]).u32();
    if property.is_null()
        || water_form == 0
        || !e.get(property, WaterShaderProperty::bUpdateConstants)
    {
        return;
    }

    let packed = e.call(0x004f_9bf0, &args![water_form]).u32();
    store_packed_color(
        e,
        address_of(property, WaterShaderProperty::pShallowColor),
        packed,
    );
    let packed = e.call(0x009e_e040, &args![water_form]).u32();
    store_packed_color(
        e,
        address_of(property, WaterShaderProperty::pDeepColor),
        packed,
    );
    let packed = e.call(0x008d_80e0, &args![water_form]).u32();
    store_packed_color(
        e,
        address_of(property, WaterShaderProperty::pReflectionColor),
        packed,
    );

    let value = float_getter(e, 0x0064_4930, water_form);
    e.set(property, WaterShaderProperty::value_00b8, value);
    let value = float_getter(e, 0x008d_01e0, water_form);
    e.set(property, WaterShaderProperty::value_00bc, value);
    let value = float_getter(e, 0x0058_0100, water_form);
    e.set(property, WaterShaderProperty::value_00c0, value);
    let value = float_getter(e, 0x004a_0d90, water_form);
    e.set(property, WaterShaderProperty::value_00c4, value);
    let value = fn_004e3cc0(e, Ptr::new(water_form));
    e.set(property, WaterShaderProperty::fFresnelAmount, value);
    let value = float_getter(e, 0x004a_0db0, water_form);
    e.set(property, WaterShaderProperty::value_0100, value);
    let value = float_getter(e, 0x0064_6f50, water_form);
    e.set(property, WaterShaderProperty::value_0104, value);
    let value = fn_004e3d20(e, Ptr::new(water_form));
    e.set(property, WaterShaderProperty::value_00d0, value);
    let value = fn_004e3d40(e, Ptr::new(water_form));
    e.set(property, WaterShaderProperty::value_00d4, value);
    let value = float_getter(e, 0x009a_5480, water_form);
    e.set(property, WaterShaderProperty::fNoiseScale, value);
    let value = float_getter(e, 0x0088_5d70, water_form);
    e.set_global(WATER_SHADER_FLOAT, value);

    if water_skipped(e) || fn_004e3c40(e) != 0 {
        let fog_far = float_getter(e, 0x0050_7b20, water_form);
        e.set(property, WaterShaderProperty::fFogFar, fog_far);
        let far = float_getter(e, 0x0050_7b20, water_form);
        let near = fn_004e3d00(e, Ptr::new(water_form));
        e.set(
            property,
            WaterShaderProperty::fFogRange,
            (far as f64 - near as f64) as f32,
        );
        if fn_004e3c70(e, base_form) {
            let value = float_getter(e, 0x0050_8070, water_form);
            e.set(property, WaterShaderProperty::fFogAmount, value);
        }
    } else {
        let fog_far = float_getter(e, 0x009b_88a0, water_form);
        e.set(property, WaterShaderProperty::fFogFar, fog_far);
        let far = float_getter(e, 0x009b_88a0, water_form);
        let near = float_getter(e, 0x0081_2870, water_form);
        e.set(
            property,
            WaterShaderProperty::fFogRange,
            (far as f64 - near as f64) as f32,
        );
        if fn_004e3c70(e, base_form) {
            let value = fn_004e3ce0(e, Ptr::new(water_form));
            e.set(property, WaterShaderProperty::fFogAmount, value);
        } else {
            e.set(property, WaterShaderProperty::fFogAmount, 0.0f32);
        }
    }
    let value = float_getter(e, 0x0094_42a0, water_form);
    e.set(property, WaterShaderProperty::fUVScale, value);

    let old_reflections = e.get(property, WaterShaderProperty::bReflections);
    let old_refractions = e.get(property, WaterShaderProperty::bRefractions);
    let old_depth = e.get(property, WaterShaderProperty::bDepth);
    let reflections = fn_004e3c50(e, base_form) && setting_flag(e, SETTING_USE_WATER_REFLECTIONS);
    e.set(property, WaterShaderProperty::bReflections, reflections);
    let refractions = fn_004e3280(e, base_form) && setting_flag(e, SETTING_USE_WATER_REFRACTIONS);
    e.set(property, WaterShaderProperty::bRefractions, refractions);
    let depth = fn_004e32e0(e, base_form) && setting_flag(e, SETTING_USE_WATER_DEPTH);
    e.set(property, WaterShaderProperty::bDepth, depth);
    if reflections != old_reflections || refractions != old_refractions || depth != old_depth {
        fn_004e3c00(e, property);
        let wading = address_of(group, PlaceableWaterGroup::spWadingWaterGeometry);
        if pointer_in_slot(e, wading) != 0 {
            let geometry = pointer_in_slot(e, wading);
            let wading_property = water_shader_property(e, geometry);
            let passes = e.vcall(
                wading_property.addr(),
                PROPERTY_PASS_COUNT_VIRTUAL,
                &args![0u32],
            );
            if passes.u16() != 0 {
                fn_004e3c00(e, wading_property);
            }
        }
    }
    let flag = fn_004e3ca0(e, base_form);
    e.set(property, WaterShaderProperty::bObjectTexCoords, flag);

    let start = float_getter(e, 0x0064_4950, water_form) as f64;
    let end = float_getter(e, 0x0064_4970, water_form);
    if start == end as f64 {
        let minus_one: f32 = e.global(MINUS_ONE);
        e.set(property, WaterShaderProperty::value_00d8, minus_one);
        e.set(property, WaterShaderProperty::value_00dc, 0.0f32);
    } else {
        let value = float_getter(e, 0x0064_4950, water_form);
        e.set(property, WaterShaderProperty::value_00d8, value);
        let value = float_getter(e, 0x0064_4970, water_form);
        e.set(property, WaterShaderProperty::value_00dc, value);
    }
}

// Translated from 004e3c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the render-pass list of a shader property: when it has one
/// (`pRenderPassList`, +0x3c), `004e3c20` clears the list's +0x10 field.
pub fn fn_004e3c00(e: &mut Engine, property: Ptr<WaterShaderProperty>) {
    let list = e.get(property, WaterShaderProperty::pRenderPassList);
    if !list.is_null() {
        fn_004e3c20(e, list);
    }
}

// Translated from 004e3c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at +0x10 of a render-pass list.
pub fn fn_004e3c20(e: &mut Engine, list: Ptr) {
    e.mem.set_u32(list.addr() + 0x10, 0);
}

// Translated from 004e3c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte flag at `0x011c7a58`.
pub fn fn_004e3c40(e: &mut Engine) -> u8 {
    e.global(WATER_FLAG_011C7A58)
}

// Translated from 004e3c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `1` is set in the form flags.
pub fn fn_004e3c50(e: &mut Engine, form: u32) -> bool {
    form_flag(e, form, FORM_FLAG_0001)
}

// Translated from 004e3c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x80000000` is clear in the form flags.
pub fn fn_004e3c70(e: &mut Engine, form: u32) -> bool {
    !form_flag(e, form, FORM_FLAG_80000000)
}

// Translated from 004e3ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x20000000` is set in the form flags.
pub fn fn_004e3ca0(e: &mut Engine, form: u32) -> bool {
    form_flag(e, form, FORM_FLAG_20000000)
}

// Translated from 004e3cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterForm`'s `Data.fFresnelAmount` (the float at +0x9c).
pub fn fn_004e3cc0(e: &mut Engine, form: Ptr<TESWaterForm>) -> f32 {
    e.get(form, TESWaterForm::fFresnelAmount)
}

// Translated from 004e3ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterForm`'s `Data.fFogAmount` (the float at +0x108).
pub fn fn_004e3ce0(e: &mut Engine, form: Ptr<TESWaterForm>) -> f32 {
    e.get(form, TESWaterForm::fFogAmount)
}

// Translated from 004e3d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterForm`'s `Data.fUnderwaterFogDistNear` (the float at +0x114).
pub fn fn_004e3d00(e: &mut Engine, form: Ptr<TESWaterForm>) -> f32 {
    e.get(form, TESWaterForm::fUnderwaterFogDistNear)
}

// Translated from 004e3d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterForm`'s `Data.fLightRadius` (the float at +0x128).
pub fn fn_004e3d20(e: &mut Engine, form: Ptr<TESWaterForm>) -> f32 {
    e.get(form, TESWaterForm::fLightRadius)
}

// Translated from 004e3d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterForm`'s `Data.fLightBrightness` (the float at +0x12c).
pub fn fn_004e3d40(e: &mut Engine, form: Ptr<TESWaterForm>) -> f32 {
    e.get(form, TESWaterForm::fLightBrightness)
}

// Translated from 004e3d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame water form update for the references of `group`: for every
/// reference whose base form has a valid water form, makes sure the water
/// shader property has its noise map (the system's shared one, or the form's
/// own, loading it when needed), creates the form's rendered noise texture
/// once, shares it with the property, scrolls the form's three texture
/// scroll rows by the wind and wraps them into 0..1, publishes the rows and
/// the blend factors for the shader and renders the noise map when the
/// water type has not been updated yet this frame.
pub fn fn_004e3d60(e: &mut Engine, this: Ptr<TESWaterSystem>, group: Ptr<PlaceableWaterGroup>) {
    let members = group.at(PlaceableWaterGroup::PlaceableWaterList).addr();
    for_each_list_item(e, members, |e, reference| {
        update_water_form_for_reference(e, this, reference);
    });
}

/// One iteration of `fn_004e3d60`.
fn update_water_form_for_reference(e: &mut Engine, this: Ptr<TESWaterSystem>, reference: u32) {
    let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
    let water_form_address = e.vcall(base_form, BASE_FORM_GET_WATER_FORM, &[]).u32();
    if water_form_address == 0
        || e.call(WATER_FORM_HAS_NOISE_TEXTURE, &args![water_form_address])
            .u32()
            == 0
    {
        return;
    }
    let water_form: Ptr<TESWaterForm> = Ptr::new(water_form_address);

    let flush_object = fn_004e4680(e);
    if fn_004e4690(e, flush_object) == 0 {
        let flush_object = fn_004e4680(e);
        e.call(FLUSH_OBJECT_FLUSH, &args![flush_object]);
    }

    let node = e.call(WATER_REFERENCE_3D, &args![this, reference]).u32();
    if node == 0 {
        return;
    }
    let owner = e.call(NODE_OWNER, &args![node]).u32();
    if owner == 0 || e.call(OWNER_TYPE, &args![owner]).u32() != WATER_OWNER_TYPE {
        return;
    }
    let property = water_shader_property(e, node);
    let noise_height_map = address_of(property, WaterShaderProperty::spNoiseHeightMap);

    // The property's noise map: the system's shared one, or the form's own.
    if setting_flag(e, SETTING_USE_PER_WORLD_SPACE_NOISE) {
        if pointer_in_slot(e, noise_height_map) == 0 {
            fn_004e45a0(e, this, noise_height_map);
        }
    } else if pointer_in_slot(e, noise_height_map) == 0 || fn_004e4640(e, water_form) != 0 {
        e.with_stack(8, |e, texture| {
            e.call(POINTER_TEMP_CONSTRUCT, &args![texture, 0u32]);
            e.call(WATER_FORM_COPY_NOISE_TEXTURE, &args![water_form, texture]);
            if pointer_in_slot(e, texture.addr()) != 0 {
                e.call(
                    WATER_FORM_COPY_NOISE_TEXTURE,
                    &args![water_form, noise_height_map],
                );
            } else {
                let name = e.call(WATER_FORM_NOISE_NAME, &args![water_form]).u32();
                let tes = e.global::<u32>(TES_POINTER);
                e.call(
                    CREATE_TEXTURE_IMAGE,
                    &args![tes, name, noise_height_map, 1u32, 0u32],
                );
            }
            fn_004e4660(e, water_form, 0);
            e.call(POINTER_TEMP_DESTRUCT, &args![texture]);
        });
    }

    // The form's rendered noise texture, created once from the property's.
    let form_noise_map = address_of(water_form, TESWaterForm::spNoiseNormalMap);
    if pointer_in_slot(e, noise_height_map) != 0 && pointer_in_slot(e, form_noise_map) == 0 {
        let noise = pointer_in_slot(e, noise_height_map);
        let width = e.vcall(noise, TEXTURE_WIDTH_VIRTUAL, &[]).u32();
        e.set_global(NOISE_WIDTH, width);
        let noise = pointer_in_slot(e, noise_height_map);
        let height = e.vcall(noise, TEXTURE_HEIGHT_VIRTUAL, &[]).u32();
        e.set_global(NOISE_HEIGHT, height);
        let renderer = e.call(RENDERER, &[]).u32();
        let manager = e.call(TEXTURE_MANAGER, &[]).u32();
        let created = e.call(
            CREATE_RENDERED_TEXTURE,
            &args![manager, renderer, 0x13u32, 0u32, 0u32, 0u32],
        );
        assign_slot(e, form_noise_map, created.u32());
    }
    e.call(
        NI_POINTER_ASSIGN_FROM,
        &args![
            address_of(property, WaterShaderProperty::spNoiseNormalMap),
            form_noise_map
        ],
    );

    // Only the first reference of a water form in an update scrolls and
    // renders its noise.
    let update_map = address_of(this, TESWaterSystem::WaterTypeUpdateMap);
    let already_updated = e.with_stack(1, |e, flag| {
        e.call(WATER_TYPE_MAP_FIND, &args![update_map, water_form, flag])
            .bool()
    });
    if already_updated {
        return;
    }
    if pointer_in_slot(e, noise_height_map) != 0 {
        scroll_and_render_noise(e, water_form, property);
    }
    e.call(WATER_TYPE_MAP_SET, &args![update_map, water_form, 1u32]);
}

/// A `float` getter of a water form that the texture scroll multiplies by
/// the time step (`this` is the form).
type ScrollCoefficient = fn(&mut Engine, Ptr<TESWaterForm>) -> f32;
/// `sinf` / `cosf` as `fn_004e44b0` / `fn_004e4470`.
type TrigFunction = fn(&mut Engine, f32) -> f32;

/// The scrolling and noise render of `fn_004e3d60`, for the first reference
/// of a water form in an update: scrolls the form's three texture rows by
/// their wind, wraps them into 0..1, publishes them (and the height scales
/// and amplitudes) for the shader and renders the noise map.
fn scroll_and_render_noise(
    e: &mut Engine,
    water_form: Ptr<TESWaterForm>,
    property: Ptr<WaterShaderProperty>,
) {
    let noise_scale = e.get(property, WaterShaderProperty::fNoiseScale);
    e.set_global(SHADER_NOISE_BASE, noise_scale);
    let time_step = e.call(FLOAT_TABLE_ENTRY, &args![1u32]).f32();
    let direction0 = fn_004e45e0(e, water_form);
    let direction1 = float_getter(e, 0x0096_6a20, water_form.addr());
    let direction2 = fn_004e4600(e, water_form);
    let to_radians: f64 = e.global(DEGREES_TO_RADIANS);

    // Each row moves by (speed * time step) times the sine (x) and cosine
    // (y) of its wind direction.
    let speed0: ScrollCoefficient = |e, form| float_getter(e, 0x0064_47d0, form.addr());
    let speed2: ScrollCoefficient = |e, form| float_getter(e, 0x0064_47f0, form.addr());
    let rows: [(
        Field<TESWaterForm, f32>,
        f32,
        TrigFunction,
        ScrollCoefficient,
    ); 6] = [
        (TESWaterForm::fTexScroll0R, direction0, fn_004e44b0, speed0),
        (TESWaterForm::fTexScroll0G, direction0, fn_004e4470, speed0),
        (
            TESWaterForm::fTexScroll1R,
            direction1,
            fn_004e44b0,
            fn_004e4620,
        ),
        (
            TESWaterForm::fTexScroll1G,
            direction1,
            fn_004e4470,
            fn_004e4620,
        ),
        (TESWaterForm::fTexScroll2R, direction2, fn_004e44b0, speed2),
        (TESWaterForm::fTexScroll2G, direction2, fn_004e4470, speed2),
    ];
    for (field, direction, trig, speed) in rows {
        let angle = (direction as f64 * to_radians) as f32;
        let trig_value = trig(e, angle) as f64;
        let speed_value = speed(e, water_form) as f64;
        let old = e.get(water_form, field);
        let new = ((speed_value * time_step as f64) * trig_value + old as f64) as f32;
        e.set(water_form, field, new);
    }
    for field in [
        TESWaterForm::fTexScroll0R,
        TESWaterForm::fTexScroll0G,
        TESWaterForm::fTexScroll1R,
        TESWaterForm::fTexScroll1G,
        TESWaterForm::fTexScroll2R,
        TESWaterForm::fTexScroll2G,
    ] {
        bs_wrap(e, Ptr::new(address_of(water_form, field)), 0.0, 1.0);
    }

    // The three scroll rows (four words each from +0x34, +0x44, +0x54 of the
    // form), published after the noise scale.
    for (row, source) in [0x34u32, 0x44, 0x54].into_iter().enumerate() {
        for i in 0..4 {
            let word = e.mem.u32(water_form.addr() + source + 4 * i);
            e.mem
                .set_u32(SHADER_NOISE_BASE + 4 + 16 * row as u32 + 4 * i, word);
        }
    }

    // Three height scales, then three amplitudes.
    let scale = fn_004e45c0(e, water_form);
    publish_height_scale(e, scale, SHADER_NOISE_BASE + 0x34);
    let scale = float_getter(e, 0x0082_1640, water_form.addr());
    publish_height_scale(e, scale, SHADER_NOISE_BASE + 0x38);
    let scale = float_getter(e, 0x0082_1660, water_form.addr());
    publish_height_scale(e, scale, SHADER_NOISE_BASE + 0x3c);
    let amplitude = float_getter(e, 0x0056_7470, water_form.addr());
    e.set_global(SHADER_NOISE_BASE + 0x44, amplitude);
    let amplitude = float_getter(e, 0x0082_1680, water_form.addr());
    e.set_global(SHADER_NOISE_BASE + 0x48, amplitude);
    let amplitude = float_getter(e, 0x0081_33b0, water_form.addr());
    e.set_global(SHADER_NOISE_BASE + 0x4c, amplitude);

    let form_noise_map = pointer_in_slot(e, address_of(water_form, TESWaterForm::spNoiseNormalMap));
    let height_map = pointer_in_slot(
        e,
        address_of(property, WaterShaderProperty::spNoiseHeightMap),
    );
    let renderer = e.call(RENDERER, &[]).u32();
    let manager = fn_004e3270(e);
    e.call(
        IMAGE_SPACE_RENDER_NOISE,
        &args![
            manager,
            0x21u32,
            renderer,
            height_map,
            form_noise_map,
            0u32,
            1u32
        ],
    );
}

/// A height scale for the shader: the value divided by 100, rounded up
/// (`fn_004e4430`) and combined with 1.0 by `00404010`.
fn publish_height_scale(e: &mut Engine, value: f32, destination: u32) {
    let hundred: f64 = e.global(ONE_HUNDRED);
    let scaled = (value as f64 / hundred) as f32;
    let rounded = fn_004e4430(e, scaled);
    let combined = e.call(FLOAT_MAX, &args![1.0f32, rounded]).f32();
    e.set_global(destination, combined);
}

// Translated from 004e4430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ceilf`: `fn_004e4450`.
pub fn fn_004e4430(e: &mut Engine, value: f32) -> f32 {
    fn_004e4450(e, value)
}

// Translated from 004e4450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ceilf`: the CRT `ceil` (`00ec9e10`) on the value as a `double`, stored
/// as a `float`.
pub fn fn_004e4450(e: &mut Engine, value: f32) -> f32 {
    e.call(CRT_CEIL, &args![value as f64]).f64() as f32
}

// Translated from 004e4470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cosf`: `fn_004e4490`.
pub fn fn_004e4470(e: &mut Engine, value: f32) -> f32 {
    fn_004e4490(e, value)
}

// Translated from 004e4490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cosf`: the CRT `cos` (`00ec9f30`) on the value as a `double`, stored as
/// a `float`.
pub fn fn_004e4490(e: &mut Engine, value: f32) -> f32 {
    e.call(CRT_COS, &args![value as f64]).f64() as f32
}

// Translated from 004e44b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `sinf`: `fn_004e44d0`.
pub fn fn_004e44b0(e: &mut Engine, value: f32) -> f32 {
    fn_004e44d0(e, value)
}

// Translated from 004e44d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `sinf`: the CRT `sin` (`00eca060`) on the value as a `double`, stored as
/// a `float`.
pub fn fn_004e44d0(e: &mut Engine, value: f32) -> f32 {
    e.call(CRT_SIN, &args![value as f64]).f64() as f32
}

// Translated from 004e44f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWrap` (Xbox PDB): wraps the float at `value` into `low..=high`. Does
/// nothing when `high < low` or the value is already inside; a value above
/// `high` becomes `fmod(value - low, high - low) + low`, one below `low`
/// becomes `fmod(value - low, high - low) + high` (the remainder is
/// `004b1520`, taking the offset and the range).
pub fn bs_wrap(e: &mut Engine, value: Ptr, low: f32, high: f32) {
    if high < low {
        return;
    }
    let current = e.mem.f32(value.addr());
    if high < current {
        let range = (high as f64 - low as f64) as f32;
        let offset = (current as f64 - low as f64) as f32;
        let remainder = e.call(FLOAT_REMAINDER, &args![offset, range]).f64();
        e.mem.set_f32(value.addr(), (remainder + low as f64) as f32);
    } else if current < low {
        let range = (high as f64 - low as f64) as f32;
        let offset = (current as f64 - low as f64) as f32;
        let remainder = e.call(FLOAT_REMAINDER, &args![offset, range]).f64();
        e.mem
            .set_f32(value.addr(), (remainder + high as f64) as f32);
    }
}

// Translated from 004e45a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Points the `NiPointer` slot `destination` at the system's shared noise
/// texture (`spWaterNoiseTexture`, +0x1c).
pub fn fn_004e45a0(e: &mut Engine, this: Ptr<TESWaterSystem>, destination: u32) {
    let source = address_of(this, TESWaterSystem::spWaterNoiseTexture);
    e.call(NI_POINTER_ASSIGN_FROM, &args![destination, source]);
}

// Translated from 004e45c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterForm`'s `Data.fHeightUVScale0` (the float at +0x130).
pub fn fn_004e45c0(e: &mut Engine, form: Ptr<TESWaterForm>) -> f32 {
    e.get(form, TESWaterForm::fHeightUVScale0)
}

// Translated from 004e45e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterForm`'s `Data.fNoiseWindDirection0` (the float at +0xe8).
pub fn fn_004e45e0(e: &mut Engine, form: Ptr<TESWaterForm>) -> f32 {
    e.get(form, TESWaterForm::fNoiseWindDirection0)
}

// Translated from 004e4600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterForm`'s `Data.fNoiseWindDirection2` (the float at +0xf0).
pub fn fn_004e4600(e: &mut Engine, form: Ptr<TESWaterForm>) -> f32 {
    e.get(form, TESWaterForm::fNoiseWindDirection2)
}

// Translated from 004e4620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterForm`'s `Data.fNoiseWindSpeed1` (the float at +0xf8).
pub fn fn_004e4620(e: &mut Engine, form: Ptr<TESWaterForm>) -> f32 {
    e.get(form, TESWaterForm::fNoiseWindSpeed1)
}

// Translated from 004e4640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterForm::bResetNoiseTexture` (the byte at +0x190).
pub fn fn_004e4640(e: &mut Engine, form: Ptr<TESWaterForm>) -> u8 {
    e.get(form, TESWaterForm::bResetNoiseTexture)
}

// Translated from 004e4660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `TESWaterForm::bResetNoiseTexture` (the byte at +0x190).
pub fn fn_004e4660(e: &mut Engine, form: Ptr<TESWaterForm>, value: u8) {
    e.set(form, TESWaterForm::bResetNoiseTexture, value);
}

// Translated from 004e4680 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pointer global at `0x011ffe44` (the object whose byte at +0x2b8
/// `fn_004e4690` tests).
pub fn fn_004e4680(e: &mut Engine) -> u32 {
    e.global(FLUSH_OBJECT)
}

// Translated from 004e4690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x2b8 of `object`.
pub fn fn_004e4690(e: &mut Engine, object: u32) -> u8 {
    e.mem.u8(object + 0x2b8)
}

// Translated from 004e46b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::AddPlaceableWater` (Xbox PDB): reads the reference's 3D
/// node's world position and rotation matrix and passes them (copied to
/// locals) to `AddPlaceableWater_ov2` (`004e4730`) with the reference.
pub fn tes_water_system_add_placeable_water(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    reference: u32,
) {
    let node = e.vcall(reference, REFERENCE_GET_3D, &[]).u32();
    let translate = e.call(NODE_WORLD_TRANSLATE, &args![node]).u32();
    let position: Vec<u32> = (0..3).map(|i| e.mem.u32(translate + 4 * i)).collect();
    let node = e.vcall(reference, REFERENCE_GET_3D, &[]).u32();
    let rotate = e.call(NODE_WORLD_ROTATE, &args![node]).u32();
    let matrix: Vec<u32> = (0..9).map(|i| e.mem.u32(rotate + 4 * i)).collect();
    e.with_stack(0x30, |e, locals| {
        for (i, word) in position.iter().chain(matrix.iter()).enumerate() {
            e.mem.set_u32(locals.addr() + 4 * i as u32, *word);
        }
        let matrix_copy = locals.addr() + 0x0c;
        e.call(
            ADD_PLACEABLE_WATER_OV2,
            &args![this, reference, locals, matrix_copy],
        );
    });
}

// ---------------------------------------------------------------------------
// Helpers of the functions from `004e4730` on.

/// Copies `count` words.
fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// `TESObjectREFR::Get3D` (the virtual at +0x1d0): the 3D object of a
/// reference, null when it has none.
fn reference_node(e: &mut Engine, reference: u32) -> u32 {
    e.vcall(reference, REFERENCE_GET_3D, &[]).u32()
}

/// Whether the reference belongs in `ActorsInWaterList` (the virtual at
/// +0x100).
fn reference_is_actor(e: &mut Engine, reference: u32) -> bool {
    e.vcall(reference, REFERENCE_IS_ACTOR_VIRTUAL, &[]).bool()
}

/// A float INI setting read through `00403e20`.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_FLOAT_VALUE, &args![setting]).u32();
    e.mem.f32(value)
}

/// An integer INI setting read through `0043d4d0`.
fn setting_int(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_INT_VALUE, &args![setting]).u32();
    e.mem.u32(value)
}

/// `NiTPointerListBase::AddHead(list, &item)`: the game passes the address
/// of a local holding the item.
fn list_add_head(e: &mut Engine, list: u32, item: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_ADD_HEAD, &args![list, slot]);
    });
}

/// `AddTail(list, &item)`.
fn list_add_tail(e: &mut Engine, list: u32, item: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_ADD_TAIL, &args![list, slot]);
    });
}

/// `InsertBefore(list, position, &item)`.
fn list_insert_before(e: &mut Engine, list: u32, position: u32, item: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_INSERT_BEFORE, &args![list, position, slot]);
    });
}

/// `RemoveAt(list, &position)`: the game passes the address of a local
/// holding the position, and the call moves it to the next node.
fn list_remove_position(e: &mut Engine, list: u32, position: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), position);
        e.call(LIST_REMOVE_POSITION, &args![list, slot]);
    });
}

/// The address of the normal of a `NiPlane` (`006815c0` returns `this`).
fn plane_normal(e: &mut Engine, plane: u32) -> u32 {
    e.call(ADDRESS_OF_THIS, &args![plane]).u32()
}

/// `NiPlane::m_fConstant` through `0084d030`.
fn plane_constant(e: &mut Engine, plane: u32) -> f32 {
    e.call(PLANE_CONSTANT, &args![plane]).f32()
}

/// Whether the normal of the plane at `group_plane` equals the one of
/// `plane` within the `0.01` tolerance (`0049e2f0`). The game reads the
/// tolerance first, then the normal of the new plane, then the group's.
fn normals_match(e: &mut Engine, group_plane: u32, plane: u32) -> bool {
    let tolerance = e.global::<f32>(PLANE_TOLERANCE);
    let new_normal = plane_normal(e, plane);
    let group_normal = plane_normal(e, group_plane);
    e.call(POINTS_NEAR, &args![group_normal, new_normal, tolerance])
        .bool()
}

/// Whether the constants (heights) of the two planes differ by at most
/// `tolerance` (`0049e390`, new plane first).
fn constants_close(e: &mut Engine, group_plane: u32, plane: u32, tolerance: f32) -> bool {
    let new_constant = plane_constant(e, plane);
    let group_constant = plane_constant(e, group_plane);
    e.call(FLOATS_NEAR, &args![group_constant, new_constant, tolerance])
        .bool()
}

/// `vector = matrix * vector`, then the vector is unitized (the game copies
/// the product from a temporary and calls `004a0c10`).
fn rotate_and_unitize(e: &mut Engine, matrix: u32, scratch: u32, vector: u32) {
    let product = e
        .call(MATRIX_TIMES_POINT, &args![matrix, scratch, vector])
        .u32();
    copy_words(e, product, vector, 3);
    e.call(POINT3_UNITIZE, &args![vector]);
}

/// Sets the four texture slots (`+0x13c`, `+0x140`, `+0x138`, `+0x144`) of a
/// water shader property to null, in the order the game does.
fn clear_all_texture_slots(e: &mut Engine, property: Ptr<WaterShaderProperty>) {
    assign_slot(
        e,
        address_of(property, WaterShaderProperty::spReflectionMap),
        0,
    );
    assign_slot(
        e,
        address_of(property, WaterShaderProperty::spRefractionMap),
        0,
    );
    assign_slot(
        e,
        address_of(property, WaterShaderProperty::spNoiseNormalMap),
        0,
    );
    assign_slot(e, address_of(property, WaterShaderProperty::spDepthMap), 0);
}

/// Points the reflection slot of a water shader property at the group's own
/// reflection map in an interior, else at the shared world or sky map
/// (the choice `FINISH_GROUP` makes).
fn assign_group_reflection_map(
    e: &mut Engine,
    property: Ptr<WaterShaderProperty>,
    group: Ptr<PlaceableWaterGroup>,
) {
    let slot = address_of(property, WaterShaderProperty::spReflectionMap);
    if in_interior(e) {
        let own_map = address_of(group, PlaceableWaterGroup::spGroupReflectionMap);
        e.call(NI_POINTER_ASSIGN_FROM, &args![slot, own_map]);
    } else if e.get(group, PlaceableWaterGroup::bGroupAtWorldSpaceWaterHeight) {
        e.call(NI_POINTER_ASSIGN_FROM, &args![slot, WORLD_REFLECTION_MAP]);
    } else {
        e.call(NI_POINTER_ASSIGN_FROM, &args![slot, SKY_REFLECTION_MAP]);
    }
}

/// The list the group keeps `object` in: `ActorsInWaterList` for actors,
/// `ObjectInWaterList` for everything else.
fn object_list_of_group(e: &mut Engine, object: u32, group: u32) -> u32 {
    if reference_is_actor(e, object) {
        group + 0x3c
    } else {
        group + 0x30
    }
}

/// Finds `object` in the group's object or actor list the way
/// `RemoveTESObjectFromWaterGroup_ov2` does (the list is chosen again at every
/// step). Returns the position of the node, or 0.
fn find_object_in_group(e: &mut Engine, object: u32, group: u32) -> u32 {
    let list = object_list_of_group(e, object, group);
    let mut position = pointer_in_slot(e, list);
    let mut found_position = 0;
    let mut found = false;
    while position != 0 && !found {
        let list = object_list_of_group(e, object, group);
        let next = e.call(LIST_NEXT_POSITION, &args![list, position]).u32();
        let list = object_list_of_group(e, object, group);
        let slot = e.call(LIST_ITEM_SLOT, &args![list, position]).u32();
        if e.mem.u32(slot) == object {
            found = true;
            found_position = position;
        }
        position = next;
    }
    found_position
}

// ---------------------------------------------------------------------------
// Translated functions, second session (`004e4730` on).

// Translated from 004e4730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::AddPlaceableWater_ov2` (Xbox PDB): registers the water
/// `reference` placed at `position` with the rotation `rotation` (an
/// `NiMatrix3`). Does nothing (false) while `bUseWater` is off. The plane of
/// the water is the position with the rotated up vector; a group whose plane
/// has the same normal and a height within `fWaterGroupHeightRange` and the
/// same water type takes the reference (a reference that is not the
/// `+0x160` kind and whose height differs by more than `0.01` is moved to the
/// group's height, with a log message). Otherwise a new group is created and
/// put in the group list in front of the first group that is lower. Then the
/// water system is enabled and the stencil bits are reassigned.
pub fn tes_water_system_add_placeable_water_ov2(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    reference: u32,
    position: u32,
    rotation: u32,
) -> bool {
    if !setting_flag(e, SETTING_USE_WATER) {
        return false;
    }
    e.with_stack(0x70, |e, frame| {
        let frame = frame.addr();
        let up = frame;
        let down = frame + 0x0c;
        let scratch = frame + 0x18;
        let plane_up = frame + 0x24;
        let plane_down = frame + 0x34;
        let moved_position = frame + 0x44;
        let update_data = frame + 0x50;

        e.call(NI_POINT3_CONSTRUCT, &args![up, 0.0f32, 0.0f32, 1.0f32]);
        let minus_one: f32 = e.global(MINUS_ONE);
        e.call(NI_POINT3_CONSTRUCT, &args![down, 0.0f32, 0.0f32, minus_one]);
        rotate_and_unitize(e, rotation, scratch, up);
        rotate_and_unitize(e, rotation, scratch, down);
        e.call(NI_PLANE_CONSTRUCT, &args![plane_up, up, position]);
        e.call(NI_PLANE_CONSTRUCT, &args![plane_down, down, position]);
        let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();

        let list = this.at(TESWaterSystem::PlaceableWaterGroupList).addr();
        let mut joined_a_group = false;
        let mut insert_before = 0;
        let mut cursor = pointer_in_slot(e, list);
        while cursor != 0 {
            let next = e.call(LIST_NEXT_POSITION, &args![list, cursor]).u32();
            let slot = e.call(LIST_ITEM_SLOT, &args![list, cursor]).u32();
            let group: Ptr<PlaceableWaterGroup> = Ptr::new(e.mem.u32(slot));
            let group_plane = address_of(group, PlaceableWaterGroup::ReflectWaterPlane);
            let same_plane = normals_match(e, group_plane, plane_up) && {
                let tolerance = setting_float(e, SETTING_WATER_GROUP_HEIGHT_RANGE);
                constants_close(e, group_plane, plane_up, tolerance)
            };
            if same_plane {
                let water_type = e.vcall(base_form, BASE_FORM_GET_WATER_FORM, &[]).u32();
                if water_type == e.get(group, PlaceableWaterGroup::pWaterType).addr() {
                    if !e
                        .vcall(reference, REFERENCE_SKIP_ADJUST_VIRTUAL, &[])
                        .bool()
                        && {
                            let tolerance = e.global::<f32>(PLANE_TOLERANCE);
                            !constants_close(e, group_plane, plane_up, tolerance)
                        }
                    {
                        // Move the reference to the height of the group.
                        let node = reference_node(e, reference);
                        let translate = e.call(NODE_WORLD_TRANSLATE, &args![node]).u32();
                        copy_words(e, translate, moved_position, 3);
                        let group_height = plane_constant(e, group_plane);
                        e.mem.set_f32(moved_position + 8, group_height);
                        let node = reference_node(e, reference);
                        e.call(NODE_SET_LOCAL_TRANSLATE, &args![node, moved_position]);
                        e.call(REFERENCE_SET_LOCATION, &args![reference, moved_position]);
                        e.call(
                            UPDATE_DATA_CONSTRUCT,
                            &args![update_data, 0.0f32, 0u32, 0u32],
                        );
                        let node = reference_node(e, reference);
                        e.vcall(node, NODE_UPDATE_WORLD_DATA, &args![update_data]);
                        if e.call(REFERENCE_PARENT_CELL, &args![reference]).u32() != 0 {
                            let cell = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32();
                            e.call(CELL_SET_WATER_HEIGHT, &args![cell, group_height]);
                        }
                        let old_height = plane_constant(e, plane_up);
                        let node = reference_node(e, reference);
                        let name = e.call(NODE_NAME_SLOT, &args![node]).u32();
                        let name = e.call(FIXED_STRING_TEXT, &args![name]).u32();
                        e.call(
                            LOG_MESSAGE,
                            &args![
                                ADJUSTING_HEIGHT_MESSAGE,
                                name,
                                old_height as f64,
                                group_height as f64
                            ],
                        );
                    }
                    joined_a_group = true;
                    tes_water_system_update_water_shader_properties_ov2(e, this, group, reference);
                    list_add_head(
                        e,
                        address_of(group, PlaceableWaterGroup::PlaceableWaterList),
                        reference,
                    );
                }
                // (The decompiler's `goto` leaves the loop body here.)
            } else {
                // A group lower than the new plane: the new group goes in
                // front of it.
                let new_constant = plane_constant(e, plane_up);
                let group_constant = plane_constant(e, group_plane);
                if group_constant < new_constant {
                    insert_before = cursor;
                    break;
                }
            }
            cursor = next;
        }

        if !joined_a_group {
            let memory = e.call(OPERATOR_NEW, &args![0xb0u32]).u32();
            let group: Ptr<PlaceableWaterGroup> = if memory == 0 {
                Ptr::new(0)
            } else {
                e.call(GROUP_CONSTRUCT, &args![memory]).ptr()
            };
            copy_words(
                e,
                plane_up,
                address_of(group, PlaceableWaterGroup::ReflectWaterPlane),
                4,
            );
            copy_words(
                e,
                plane_down,
                address_of(group, PlaceableWaterGroup::RefractWaterPlane),
                4,
            );
            let water_type = e.vcall(base_form, BASE_FORM_GET_WATER_FORM, &[]).u32();
            e.set(group, PlaceableWaterGroup::pWaterType, Ptr::new(water_type));
            tes_water_system_update_water_shader_properties_ov2(e, this, group, reference);
            list_add_head(
                e,
                address_of(group, PlaceableWaterGroup::PlaceableWaterList),
                reference,
            );
            if insert_before == 0 {
                list_add_tail(e, list, group.addr());
            } else {
                list_insert_before(e, list, insert_before, group.addr());
            }
            if !in_interior(e) {
                let tes = e.global::<u32>(TES_POINTER);
                if e.call(TES_GET_WORLD_SPACE, &args![tes]).u32() != 0 {
                    let tolerance = e.global::<f32>(PLANE_TOLERANCE);
                    let group_height = plane_constant(
                        e,
                        address_of(group, PlaceableWaterGroup::ReflectWaterPlane),
                    );
                    let world_space = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
                    let world_height = e.call(WORLD_SPACE_WATER_HEIGHT, &args![world_space]).f32();
                    if e.call(FLOATS_NEAR, &args![world_height, group_height, tolerance])
                        .bool()
                    {
                        e.set(
                            group,
                            PlaceableWaterGroup::bGroupAtWorldSpaceWaterHeight,
                            true,
                        );
                    }
                }
            }
        }
        tes_water_system_enable_water_system(e, this);
        tes_water_system_reset_stencil_bit_refs(e, this);
        true
    })
}

// Translated from 004e4c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the placeable LOD water for a world space (no Xbox PDB name):
/// a temporary `TESObjectREFR` whose object is the world space's
/// placeable-LOD-water form, a `WaterShaderProperty` (flag bytes `+0x61`,
/// and `+0x62` when `full_reflections` is set) and a 0x24-byte property
/// attached to `geometry`, and a `BSFadeNode` the geometry hangs from (below
/// `inner_parent` when there is one). The LOD group (`pLODWaterGroup`) is
/// created from the world space's water height when it does not exist yet.
/// Returns the reference, or null when this world space (or the current one)
/// has no water (the world space byte `+0x4c` bit `0x10`).
///
/// `parent` receives the fade node as a child at the end.
pub fn fn_004e4c80(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    geometry: u32,
    world_space: u32,
    parent: u32,
    inner_parent: u32,
    full_reflections: bool,
) -> u32 {
    let tes = e.global::<u32>(TES_POINTER);
    let current_world_space = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
    if e.call(WORLD_SPACE_FLAG_10, &args![current_world_space])
        .bool()
    {
        return 0;
    }
    if e.call(WORLD_SPACE_FLAG_10, &args![world_space]).bool() {
        return 0;
    }

    // The temporary reference.
    let memory = e.call(OPERATOR_NEW, &args![0x68u32]).u32();
    let reference = if memory == 0 {
        0
    } else {
        e.call(REFERENCE_CONSTRUCT, &args![memory]).u32()
    };
    e.call(FORM_SET_TEMPORARY, &args![reference]);

    // The water shader property.
    let memory = e.call(NI_ALLOC, &args![0x150u32]).u32();
    let property = if memory == 0 {
        0
    } else {
        e.call(WATER_SHADER_PROPERTY_CONSTRUCT, &args![memory])
            .u32()
    };
    e.mem.set_u8(property + 0x61, 1);
    if full_reflections {
        e.mem.set_u8(property + 0x62, 1);
    }
    let property_type = e.call(WATER_PROPERTY_TYPE, &[]).u32();
    e.call(NODE_REMOVE_PROPERTY, &args![geometry, property_type]);
    e.call(NODE_ATTACH_PROPERTY, &args![geometry, property]);
    e.call(NODE_UPDATE_PROPERTIES, &args![geometry]);
    e.call(SHADER_MANAGER_PREPARE_OBJECT, &args![geometry, 0u32, 0u32]);

    // The second property.
    let memory = e.call(NI_ALLOC, &args![0x24u32]).u32();
    let second_property = if memory == 0 {
        0
    } else {
        e.call(AUTO_WATER_PROPERTY_CONSTRUCT, &args![memory]).u32()
    };
    e.call(AUTO_WATER_PROPERTY_SET, &args![second_property, 3u32]);
    let property_type = e.call(AUTO_WATER_PROPERTY_TYPE, &[]).u32();
    e.call(NODE_REMOVE_PROPERTY, &args![geometry, property_type]);
    e.call(NODE_ATTACH_PROPERTY, &args![geometry, second_property]);

    // The reference's object is the world space's LOD water form.
    let owner = e.call(WORLD_SPACE_WATER_TYPE, &args![world_space]).u32();
    let water_form = e.call(GET_PLACEABLE_LOD_WATER, &args![owner]).u32();
    e.call(
        REFERENCE_SET_OBJECT_REFERENCE,
        &args![reference, water_form],
    );

    // The fade node and the hierarchy.
    let memory = e.call(NI_ALLOC, &args![0xe4u32]).u32();
    let fade_node = if memory == 0 {
        0
    } else {
        e.call(FADE_NODE_CONSTRUCT, &args![memory]).u32()
    };
    let mut innermost = fade_node;
    if inner_parent == 0 {
        e.vcall(fade_node, NODE_ATTACH_CHILD, &args![geometry, 1u32]);
    } else {
        innermost = inner_parent;
        e.vcall(fade_node, NODE_ATTACH_CHILD, &args![inner_parent, 1u32]);
        e.vcall(innermost, NODE_ATTACH_CHILD, &args![geometry, 1u32]);
    }
    e.vcall(reference, REFERENCE_SET_3D_VIRTUAL, &args![fade_node, 1u32]);

    e.with_stack(0x90, |e, frame| {
        let frame = frame.addr();
        let update_data = frame;
        let water_point = frame + 0x0c;
        let matrix = frame + 0x18;
        let up = frame + 0x3c;
        let scratch = frame + 0x48;
        let plane = frame + 0x54;
        let reference_slot = frame + 0x64;
        e.mem.set_u32(reference_slot, reference);

        e.call(
            UPDATE_DATA_CONSTRUCT,
            &args![update_data, 0.0f32, 0u32, 0u32],
        );
        e.call(NODE_UPDATE, &args![geometry, update_data]);
        e.call(NODE_UPDATE, &args![fade_node, update_data]);
        e.call(NODE_UPDATE, &args![innermost, update_data]);
        let height = e.call(WORLD_SPACE_WATER_HEIGHT, &args![world_space]).f32();
        e.call(
            NI_POINT3_CONSTRUCT,
            &args![water_point, 0.0f32, 0.0f32, height],
        );
        copy_words(e, IDENTITY_MATRIX, matrix, 9);

        let lod_group = e.get(this, TESWaterSystem::pLODWaterGroup);
        if lod_group.is_null() {
            let memory = e.call(OPERATOR_NEW, &args![0xb0u32]).u32();
            let group: Ptr<PlaceableWaterGroup> = if memory == 0 {
                Ptr::new(0)
            } else {
                e.call(GROUP_CONSTRUCT, &args![memory]).ptr()
            };
            e.call(NI_POINT3_CONSTRUCT, &args![up, 0.0f32, 0.0f32, 1.0f32]);
            rotate_and_unitize(e, matrix, scratch, up);
            e.call(NI_PLANE_CONSTRUCT, &args![plane, up, water_point]);
            copy_words(
                e,
                plane,
                address_of(group, PlaceableWaterGroup::ReflectWaterPlane),
                4,
            );
            let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
            let water_type = e.vcall(base_form, BASE_FORM_GET_WATER_FORM, &[]).u32();
            e.set(group, PlaceableWaterGroup::pWaterType, Ptr::new(water_type));
            list_add_head(
                e,
                address_of(group, PlaceableWaterGroup::PlaceableWaterList),
                reference,
            );
            if !setting_flag(e, SETTING_FORCE_HIGH_DETAIL_REFLECTIONS) {
                e.set(
                    group,
                    PlaceableWaterGroup::bRenderSilhouetteReflections,
                    true,
                );
            }
            e.set(this, TESWaterSystem::pLODWaterGroup, group);
        } else {
            list_add_head(
                e,
                address_of(lod_group, PlaceableWaterGroup::PlaceableWaterList),
                reference,
            );
            let count = e.global::<i32>(LOD_WATER_OBJECTS);
            e.set_global(LOD_WATER_OBJECTS, count.wrapping_add(1));
        }
    });
    e.vcall(parent, NODE_ATTACH_CHILD, &args![fade_node, 1u32]);
    tes_water_system_enable_water_system(e, this);
    reference
}

// Translated from 004e5140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes a LOD water `reference` away (no Xbox PDB name): its 3D object is
/// detached from `parent` (the virtual at +0xe8), and when the reference is in
/// the LOD group's list it leaves the list, `iLODWaterObjects` goes down,
/// its shader property loses the reflection map, and when the list is empty
/// the LOD group is destroyed; when no water group is left the water system
/// is disabled.
///
/// The second argument is not read.
pub fn fn_004e5140(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    reference: u32,
    _unused_1: u32,
    parent: u32,
) {
    let node = reference_node(e, reference);
    e.vcall(parent, NODE_DETACH_CHILD, &args![node]);
    if e.get(this, TESWaterSystem::pLODWaterGroup).is_null() {
        return;
    }
    // The game reads `pLODWaterGroup` again at every use.
    fn lod_list(e: &mut Engine, this: Ptr<TESWaterSystem>) -> u32 {
        e.get(this, TESWaterSystem::pLODWaterGroup).addr() + 0x24
    }
    let list = lod_list(e, this);
    let mut cursor = pointer_in_slot(e, list);
    while cursor != 0 {
        let list = lod_list(e, this);
        let next = e.call(LIST_NEXT_POSITION, &args![list, cursor]).u32();
        let list = lod_list(e, this);
        let slot = e.call(LIST_ITEM_SLOT, &args![list, cursor]).u32();
        let item = e.mem.u32(slot);
        if item == reference {
            let list = lod_list(e, this);
            list_remove_position(e, list, cursor);
            let count = e.global::<i32>(LOD_WATER_OBJECTS);
            e.set_global(LOD_WATER_OBJECTS, count.wrapping_sub(1));
            let node = e.call(WATER_REFERENCE_3D, &args![this, item]).u32();
            if node != 0 {
                let property = water_shader_property(e, node);
                assign_slot(
                    e,
                    address_of(property, WaterShaderProperty::spReflectionMap),
                    0,
                );
            }
            let list = lod_list(e, this);
            if e.call(LIST_COUNT, &args![list]).u32() == 0 {
                let group = e.get(this, TESWaterSystem::pLODWaterGroup);
                if !group.is_null() {
                    fn_004e52c0(e, group.addr(), 1);
                }
                e.set(this, TESWaterSystem::pLODWaterGroup, Ptr::new(0));
            }
            let groups = this.at(TESWaterSystem::PlaceableWaterGroupList).addr();
            if e.call(LIST_IS_EMPTY, &args![groups]).bool()
                && e.get(this, TESWaterSystem::pLODWaterGroup).is_null()
            {
                fn_004e6620(e, this, true, false);
            }
        }
        cursor = next;
    }
}

// Translated from 004e52c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `PlaceableWaterGroup` (no Xbox PDB
/// name): runs the destructor body (`004ed3e0`) and, when bit 0 of `flags`
/// is set, frees the object. Returns `this`.
pub fn fn_004e52c0(e: &mut Engine, this: u32, flags: u32) -> u32 {
    e.call(GROUP_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004e52f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RemovePlaceableWater` for a `reference` (no Xbox PDB name): reads the
/// position and the rotation matrix of the reference's 3D object, copies them
/// to locals and calls `RemovePlaceableWater(reference, &position,
/// &matrix)`.
pub fn fn_004e52f0(e: &mut Engine, this: Ptr<TESWaterSystem>, reference: u32) {
    e.with_stack(0x30, |e, frame| {
        let frame = frame.addr();
        let node = reference_node(e, reference);
        let translate = e.call(NODE_WORLD_TRANSLATE, &args![node]).u32();
        copy_words(e, translate, frame, 3);
        let node = reference_node(e, reference);
        let rotate = e.call(NODE_WORLD_ROTATE, &args![node]).u32();
        copy_words(e, rotate, frame + 0x0c, 9);
        tes_water_system_remove_placeable_water(e, this, reference, frame, frame + 0x0c);
    });
}

// Translated from 004e5370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::RemovePlaceableWater` (Xbox PDB): takes `reference`,
/// placed at `position` with rotation `rotation`, out of the water group
/// whose plane (same normal and height within `0.01`) it was added to. Its
/// shader property loses its four texture slots; an empty group is removed
/// from the system and destroyed; when no group is left the water system
/// is disabled, and the stencil bits are reassigned. Returns whether the
/// reference was found. Does nothing (false) while water is not enabled
/// (`bWaterEnabled`) or `bUseWater` is off.
///
/// The game also copies the three words at `0x011f426c` to a local it
/// overwrites before use; that dead store is not translated.
pub fn tes_water_system_remove_placeable_water(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    reference: u32,
    position: u32,
    rotation: u32,
) -> bool {
    if e.global::<u8>(WATER_ENABLED) == 0 || !setting_flag(e, SETTING_USE_WATER) {
        return false;
    }
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        let up = frame;
        let scratch = frame + 0x0c;
        let plane = frame + 0x18;
        let point = frame + 0x28;
        e.call(NI_POINT3_CONSTRUCT, &args![up, 0.0f32, 0.0f32, 1.0f32]);
        rotate_and_unitize(e, rotation, scratch, up);
        copy_words(e, position, point, 3);
        e.call(NI_PLANE_CONSTRUCT, &args![plane, up, point]);

        let groups = this.at(TESWaterSystem::PlaceableWaterGroupList).addr();
        let mut group_cursor = pointer_in_slot(e, groups);
        while group_cursor != 0 {
            let next_group = e
                .call(LIST_NEXT_POSITION, &args![groups, group_cursor])
                .u32();
            let slot = e.call(LIST_ITEM_SLOT, &args![groups, group_cursor]).u32();
            let group: Ptr<PlaceableWaterGroup> = Ptr::new(e.mem.u32(slot));
            let group_plane = address_of(group, PlaceableWaterGroup::ReflectWaterPlane);
            let same_plane = normals_match(e, group_plane, plane) && {
                let tolerance = e.global::<f32>(PLANE_TOLERANCE);
                constants_close(e, group_plane, plane, tolerance)
            };
            if same_plane {
                let members = address_of(group, PlaceableWaterGroup::PlaceableWaterList);
                let mut cursor = pointer_in_slot(e, members);
                while cursor != 0 {
                    let next = e.call(LIST_NEXT_POSITION, &args![members, cursor]).u32();
                    let slot = e.call(LIST_ITEM_SLOT, &args![members, cursor]).u32();
                    let item = e.mem.u32(slot);
                    if item == reference {
                        list_remove_position(e, members, cursor);
                        let node = e.call(WATER_REFERENCE_3D, &args![this, item]).u32();
                        if node != 0 {
                            let property = water_shader_property(e, node);
                            clear_all_texture_slots(e, property);
                        }
                        if e.call(LIST_COUNT, &args![members]).u32() == 0 {
                            list_remove_position(e, groups, group_cursor);
                            if !group.is_null() {
                                fn_004e52c0(e, group.addr(), 1);
                            }
                        }
                        if e.call(LIST_IS_EMPTY, &args![groups]).bool() {
                            fn_004e6620(e, this, true, false);
                        }
                        tes_water_system_reset_stencil_bit_refs(e, this);
                        return true;
                    }
                    cursor = next;
                }
            }
            group_cursor = next_group;
        }
        false
    })
}

// Translated from 004e5640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::ResetStencilBitRefs` (Xbox PDB): gives the groups, in
/// list order, the stencil masks `1 << 1`, `1 << 2`, ... (`iStencilBitMask`).
pub fn tes_water_system_reset_stencil_bit_refs(e: &mut Engine, this: Ptr<TESWaterSystem>) {
    let groups = this.at(TESWaterSystem::PlaceableWaterGroupList).addr();
    let mut bit = 1u32;
    for_each_list_item(e, groups, |e, item| {
        let group: Ptr<PlaceableWaterGroup> = Ptr::new(item);
        e.set(
            group,
            PlaceableWaterGroup::iStencilBitMask,
            1u32 << (bit & 31),
        );
        bit += 1;
    });
}

// Translated from 004e56c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finishes a rendered group (no Xbox PDB name): the reflection slot of the
/// shader property of every water reference of the group (and of the
/// wading-water geometry) is pointed at the group's reflection map. In an
/// interior that is the group's own map (`+0x54`), outdoors the shared world
/// map when the group is at the world space water height, else the shared sky
/// map. A reference is used when its cell is of kind 6 (byte `+0x26`) or
/// when its base form has the `0x08000000` flag and the player is not in an
/// interior, and only when its water shader property is of the water
/// shader property class.
pub fn fn_004e56c0(e: &mut Engine, this: Ptr<TESWaterSystem>, group: Ptr<PlaceableWaterGroup>) {
    let members = address_of(group, PlaceableWaterGroup::PlaceableWaterList);
    let mut cursor = pointer_in_slot(e, members);
    while cursor != 0 {
        let reference = e.with_stack(4, |e, position| {
            e.mem.set_u32(position.addr(), cursor);
            let item = e.call(LIST_NEXT_ITEM, &args![members, position]).u32();
            cursor = e.mem.u32(position.addr());
            e.mem.u32(item)
        });
        let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
        if reference == 0 {
            continue;
        }
        let in_cell_of_kind_6 = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32() != 0 && {
            let cell = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32();
            e.call(CELL_BYTE_IS_SIX, &args![cell]).bool()
        };
        if !in_cell_of_kind_6 && !(base_form != 0 && fn_004e32c0(e, base_form) && !in_interior(e)) {
            continue;
        }
        let node = e.call(WATER_REFERENCE_3D, &args![this, reference]).u32();
        if node == 0 {
            continue;
        }
        let property = water_shader_property(e, node);
        let is_water_property = e
            .call(IS_KIND_OF, &args![WATER_SHADER_PROPERTY_RTTI, property])
            .bool();
        if is_water_property && !property.is_null() {
            assign_group_reflection_map(e, property, group);
        }
    }
    let geometry_slot = address_of(group, PlaceableWaterGroup::spWadingWaterGeometry);
    if pointer_in_slot(e, geometry_slot) != 0 {
        let geometry = pointer_in_slot(e, geometry_slot);
        let property = water_shader_property(e, geometry);
        assign_group_reflection_map(e, property, group);
    }
}

// Translated from 004e58a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a group's reflection (no Xbox PDB name): gives the group's own
/// reflection map (`+0x54`) back to the texture manager, clears the
/// reflection slot of the shader property of every water reference of the
/// group whose 3D object has the water owner type, and of the wading-water
/// geometry. Does nothing for a null group.
///
/// The two words after the group are not read; callers pass zeros.
pub fn fn_004e58a0(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    group: Ptr<PlaceableWaterGroup>,
    _unused_1: u32,
    _unused_2: u32,
) {
    if group.is_null() {
        return;
    }
    let own_map = address_of(group, PlaceableWaterGroup::spGroupReflectionMap);
    if return_rendered_texture(e, own_map) {
        assign_slot(e, own_map, 0);
    }
    let members = address_of(group, PlaceableWaterGroup::PlaceableWaterList);
    for_each_list_item(e, members, |e, reference| {
        if reference == 0 || reference_node(e, reference) == 0 {
            return;
        }
        let node = e.call(WATER_REFERENCE_3D, &args![this, reference]).u32();
        if node == 0 {
            return;
        }
        let owner = e.call(NODE_OWNER, &args![node]).u32();
        if owner != 0 && e.call(OWNER_TYPE, &args![owner]).u32() == WATER_OWNER_TYPE {
            let property = water_shader_property(e, node);
            assign_slot(
                e,
                address_of(property, WaterShaderProperty::spReflectionMap),
                0,
            );
        }
    });
    let geometry_slot = address_of(group, PlaceableWaterGroup::spWadingWaterGeometry);
    if pointer_in_slot(e, geometry_slot) != 0 {
        let geometry = pointer_in_slot(e, geometry_slot);
        let property = water_shader_property(e, geometry);
        assign_slot(
            e,
            address_of(property, WaterShaderProperty::spReflectionMap),
            0,
        );
    }
}

// Translated from 004e59f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the water group of a position (no Xbox PDB name). With a water
/// `reference` the plane is the reference's: its 3D object's rotated up
/// vector through its world position, and the water type is the one of its
/// base form (null result when it has no 3D object). Without one the plane
/// is horizontal at `height`, and the water type is the one of the cell of
/// `object` (`TESObjectCELL::GetWaterType`). Returns the first group with the
/// same normal and a height within `0.01` and the same water type (when there
/// is a reference, it must also be in the group's list), or null.
pub fn fn_004e59f0(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    reference: u32,
    object: u32,
    height: f32,
) -> u32 {
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        let up = frame;
        let scratch = frame + 0x0c;
        let point = frame + 0x18;
        let plane = frame + 0x24;
        e.call(NI_POINT3_CONSTRUCT, &args![up, 0.0f32, 0.0f32, 1.0f32]);
        let water_type = if reference == 0 {
            let made = e
                .call(NI_POINT3_CONSTRUCT, &args![scratch, 0.0f32, 0.0f32, height])
                .u32();
            copy_words(e, made, point, 3);
            let cell = e.call(REFERENCE_PARENT_CELL, &args![object]).u32();
            e.call(CELL_GET_WATER_TYPE, &args![cell]).u32()
        } else {
            if reference_node(e, reference) == 0 {
                return 0;
            }
            let node = reference_node(e, reference);
            let rotate = e.call(NODE_WORLD_ROTATE, &args![node]).u32();
            rotate_and_unitize(e, rotate, scratch, up);
            let node = reference_node(e, reference);
            let translate = e.call(NODE_WORLD_TRANSLATE, &args![node]).u32();
            copy_words(e, translate, point, 3);
            let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
            e.vcall(base_form, BASE_FORM_GET_WATER_FORM, &[]).u32()
        };
        e.call(NI_PLANE_CONSTRUCT, &args![plane, up, point]);

        let groups = this.at(TESWaterSystem::PlaceableWaterGroupList).addr();
        let mut cursor = pointer_in_slot(e, groups);
        while cursor != 0 {
            let next = e.call(LIST_NEXT_POSITION, &args![groups, cursor]).u32();
            let slot = e.call(LIST_ITEM_SLOT, &args![groups, cursor]).u32();
            let group: Ptr<PlaceableWaterGroup> = Ptr::new(e.mem.u32(slot));
            let group_plane = address_of(group, PlaceableWaterGroup::ReflectWaterPlane);
            let same_plane = normals_match(e, group_plane, plane) && {
                let tolerance = e.global::<f32>(PLANE_TOLERANCE);
                constants_close(e, group_plane, plane, tolerance)
            };
            if same_plane && e.get(group, PlaceableWaterGroup::pWaterType).addr() == water_type {
                if reference == 0 {
                    return group.addr();
                }
                let members = address_of(group, PlaceableWaterGroup::PlaceableWaterList);
                let mut member_cursor = pointer_in_slot(e, members);
                while member_cursor != 0 {
                    let next_member = e
                        .call(LIST_NEXT_POSITION, &args![members, member_cursor])
                        .u32();
                    let slot = e.call(LIST_ITEM_SLOT, &args![members, member_cursor]).u32();
                    let member = e.mem.u32(slot);
                    if reference == 0 || member == reference {
                        return group.addr();
                    }
                    member_cursor = next_member;
                }
            }
            cursor = next;
        }
        0
    })
}

// Translated from 004e5c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::AddTESObjectToWaterGroup` (Xbox PDB): puts `object`
/// into the object list (or, for an actor, the actor list) of the group of
/// the water `reference` (or, without one, of the water at `height` in the
/// object's cell). Does nothing when there is neither a reference nor a
/// cell, when no group is found or when the object is already in the group's
/// object list. For an object whose form flag `0x1000000` is set and that has
/// a 3D object and passes `004523e0(2)`, the master particle addon nodes of
/// its 3D object are removed.
pub fn tes_water_system_add_tes_object_to_water_group(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    reference: u32,
    object: u32,
    height: f32,
) {
    if reference == 0 && e.call(REFERENCE_PARENT_CELL, &args![object]).u32() == 0 {
        return;
    }
    let group = fn_004e59f0(e, this, reference, object, height);
    if group == 0 {
        return;
    }
    let objects = group + 0x30;
    let mut cursor = pointer_in_slot(e, objects);
    let mut already_in = false;
    while cursor != 0 && !already_in {
        let next = e.call(LIST_NEXT_POSITION, &args![objects, cursor]).u32();
        let slot = e.call(LIST_ITEM_SLOT, &args![objects, cursor]).u32();
        if e.mem.u32(slot) == object {
            already_in = true;
        }
        cursor = next;
    }
    if already_in {
        return;
    }
    if reference_is_actor(e, object) {
        let actors = group + 0x3c;
        let found = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), object);
            e.call(LIST_FIND_POSITION, &args![actors, slot, 0u32]).u32()
        });
        if found == 0 {
            list_add_head(e, actors, object);
        }
    } else {
        list_add_head(e, objects, object);
    }
    if e.call(OBJECT_FLAG_1000000, &args![object]).bool()
        && reference_node(e, object) != 0
        && e.call(OBJECT_MEMBER_TEST, &args![object, 2u32]).bool()
    {
        let node = reference_node(e, object);
        let as_node = e.vcall(node, NODE_IS_NODE_VIRTUAL, &[]).u32();
        e.call(REMOVE_MASTER_PARTICLE_ADDON_NODES, &args![as_node]);
    }
}

// Translated from 004e5df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::RemoveTESObjectFromWaterGroup` (Xbox PDB): finds the
/// group the same way `AddTESObjectToWaterGroup` does and takes `object` out
/// of it (`RemoveTESObjectFromWaterGroup_ov2`). Does nothing without a
/// reference unless the object has a cell.
pub fn tes_water_system_remove_tes_object_from_water_group(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    reference: u32,
    object: u32,
    height: f32,
) {
    if reference == 0 && (object == 0 || e.call(REFERENCE_PARENT_CELL, &args![object]).u32() == 0) {
        return;
    }
    let group = fn_004e59f0(e, this, reference, object, height);
    if group == 0 {
        return;
    }
    tes_water_system_remove_tes_object_from_water_group_ov2(e, this, object, group);
}

// Translated from 004e5e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::RemoveTESObjectFromWaterGroup_ov2` (Xbox PDB): takes
/// `object` out of the group's actor list (when it is an actor) or object
/// list. For an actor the wading-water map entry of the object is removed
/// too.
pub fn tes_water_system_remove_tes_object_from_water_group_ov2(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    object: u32,
    group: u32,
) {
    let found_position = find_object_in_group(e, object, group);
    if found_position == 0 {
        return;
    }
    if reference_is_actor(e, object) {
        list_remove_position(e, group + 0x3c, found_position);
        let wading_map = address_of(this, TESWaterSystem::WadingWaterMap);
        let found = e.with_stack(4, |e, value| {
            e.mem.set_u32(value.addr(), 0);
            e.call(WADING_MAP_GET, &args![wading_map, object, value])
                .bool()
        });
        if found {
            e.call(WADING_MAP_REMOVE, &args![wading_map, object]);
        }
    } else {
        list_remove_position(e, group + 0x30, found_position);
    }
}

// Translated from 004e5fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes `object` out of every water group that none of the water zone
/// references of its extra data belongs to (no Xbox PDB name). The zone map
/// is the `ExtraDataList::QWaterZoneMap` of the object; without one (or with
/// no zone reference in the group's list) the object is removed from the
/// group as `RemoveTESObjectFromWaterGroup_ov2` does, after the pending
/// nodes of the object (`0057b200` / `0057b240`) are removed.
///
/// When a zone reference is in a group's list the game does not advance to the
/// next group (the position variable is only updated on the removal path) and
/// runs the same iteration for ever; that hang is reported as a panic.
pub fn fn_004e5fe0(e: &mut Engine, this: Ptr<TESWaterSystem>, object: u32) {
    let extra = e.call(REFERENCE_EXTRA_DATA_LIST, &args![object]).u32();
    let zone_map = e.call(EXTRA_DATA_LIST_WATER_ZONE_MAP, &args![extra]).u32();
    let groups = this.at(TESWaterSystem::PlaceableWaterGroupList).addr();
    let mut cursor = pointer_in_slot(e, groups);
    while cursor != 0 {
        let next = e.call(LIST_NEXT_POSITION, &args![groups, cursor]).u32();
        let slot = e.call(LIST_ITEM_SLOT, &args![groups, cursor]).u32();
        let group = e.mem.u32(slot);
        let mut not_in_zone = true;
        if zone_map != 0 {
            let mut iterator = e.call(ZONE_MAP_FIRST, &args![zone_map]).u32();
            while iterator != 0 {
                let key = e.with_stack(0x0c, |e, frame| {
                    let frame = frame.addr();
                    e.mem.set_u32(frame, iterator);
                    e.call(
                        ZONE_MAP_GET_NEXT,
                        &args![zone_map, frame, frame + 4, frame + 8],
                    );
                    iterator = e.mem.u32(frame);
                    e.mem.u32(frame + 4)
                });
                let zone_reference = e.call(READ_WORD_AT_0C, &args![key + 4]).u32();
                let members = group + 0x24;
                let mut member_cursor = pointer_in_slot(e, members);
                while member_cursor != 0 {
                    let next_member = e
                        .call(LIST_NEXT_POSITION, &args![members, member_cursor])
                        .u32();
                    let slot = e.call(LIST_ITEM_SLOT, &args![members, member_cursor]).u32();
                    if e.mem.u32(slot) == zone_reference {
                        not_in_zone = false;
                        break;
                    }
                    member_cursor = next_member;
                }
                if !not_in_zone {
                    break;
                }
            }
        }
        if !not_in_zone {
            panic!(
                "FalloutNV.exe 004e5fe0 loops for ever here: it does not advance to the next water group after a zone match"
            );
        }
        // Remove the object from the group.
        let found_position = find_object_in_group(e, object, group);
        if found_position != 0 {
            while e.call(REFERENCE_HAS_PENDING_NODES, &args![object]).bool() {
                e.call(REFERENCE_REMOVE_PENDING_NODE, &args![object, 0u32]);
            }
            if reference_is_actor(e, object) {
                list_remove_position(e, group + 0x3c, found_position);
                let wading_map = address_of(this, TESWaterSystem::WadingWaterMap);
                let found = e.with_stack(4, |e, value| {
                    e.mem.set_u32(value.addr(), 0);
                    e.call(WADING_MAP_GET, &args![wading_map, object, value])
                        .bool()
                });
                if found {
                    e.call(WADING_MAP_REMOVE, &args![wading_map, object]);
                }
            } else {
                list_remove_position(e, group + 0x30, found_position);
            }
        }
        cursor = next;
    }
}

// Translated from 004e62e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the water `reference` counts as in range of the `viewer` (no
/// Xbox PDB name): false without a 3D object. When the first child of the 3D
/// node is a `BSFaceGenNiNode` the answer is whether the target of its
/// animation data holds exactly one entry; otherwise `004b5fc0(node, viewer)`
/// decides.
///
/// `this` is not read.
pub fn fn_004e62e0(e: &mut Engine, _unused_0: u32, reference: u32, viewer: u32) -> bool {
    let node = reference_node(e, reference);
    if node == 0 {
        return false;
    }
    let child = e.call(NODE_FIRST_CHILD, &args![node, 0u32]).u32();
    if e.call(IS_KIND_OF, &args![FACE_GEN_NODE_RTTI, child]).bool() {
        let child = e.call(NODE_FIRST_CHILD, &args![node, 0u32]).u32();
        let animation_data = e.call(FACE_GEN_ANIMATION_DATA, &args![child]).u32();
        let target = e.call(ANIMATION_DATA_TARGET, &args![animation_data]).u32();
        e.call(LIST_COUNT, &args![target]).u32() == 1
    } else {
        e.call(NODE_IN_RANGE_OF_VIEWER, &args![node, viewer]).bool()
    }
}

// Translated from 004e6370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows, hides or toggles the 3D objects of the water references (no Xbox
/// PDB name). For every reference of every group, restricted to those whose
/// base form has the `0x08000000` flag when `only_flagged` is set, the
/// node's culled flag (`00450f90`) becomes `!show`, or, with `toggle`, the
/// opposite of what it is.
pub fn fn_004e6370(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    show: bool,
    only_flagged: bool,
    toggle: bool,
) {
    let groups = this.at(TESWaterSystem::PlaceableWaterGroupList).addr();
    for_each_list_item(e, groups, |e, group| {
        let members = group + 0x24;
        for_each_list_item(e, members, |e, reference| {
            if only_flagged {
                let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
                if !fn_004e32c0(e, base_form) {
                    return;
                }
            }
            if toggle {
                let node = reference_node(e, reference);
                let culled = e.call(NODE_IS_CULLED, &args![node]).bool();
                let node = reference_node(e, reference);
                e.call(NODE_SET_CULLED, &args![node, !culled]);
            } else {
                let node = reference_node(e, reference);
                e.call(NODE_SET_CULLED, &args![node, !show]);
            }
        });
    });
}

// Translated from 004e6540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointer<T>::operator T*` on the member at `+0xf8` (a folded accessor
/// the engine map has no name for; its callers in the engine map are
/// `ToggleVATSLight`, `CalculateLightValue` and `SetMode`).
pub fn fn_004e6540(e: &mut Engine, this: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![this + 0xf8]).u32()
}

// Translated from 004e6560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit member at `+0x110` of the same object as `fn_004e6540`.
pub fn fn_004e6560(e: &mut Engine, this: u32) -> u16 {
    e.mem.u16(this + 0x110)
}

// Translated from 004e6580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the float at `+0xdc` is at least the global `0x011ad834` and the
/// float at `+0xd8` is at most `0.1` (same object as `fn_004e6540`). False
/// when either value is not a number.
pub fn fn_004e6580(e: &mut Engine, this: u32) -> bool {
    let threshold = e.global::<f32>(FLOAT_THRESHOLD_011AD834);
    let value = e.mem.f32(this + 0xdc);
    #[allow(clippy::neg_cmp_op_on_partial_ord)] // false for NaN, as the x87 test is
    let below = !(threshold <= value);
    if below {
        return false;
    }
    let value = e.mem.f32(this + 0xd8);
    let limit = e.global::<f64>(ZERO_POINT_ONE);
    (value as f64) <= limit
}

// Translated from 004e65d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::EnableWaterSystem` (Xbox PDB): sets the byte at
/// `0x011ad86e` to 1; when water is not enabled yet and `bUseWater` is on,
/// enables it (`bWaterEnabled`) and shows the water references (`004e6370`
/// with show set). Returns whether it did.
pub fn tes_water_system_enable_water_system(e: &mut Engine, this: Ptr<TESWaterSystem>) -> bool {
    e.set_global(WATER_REQUEST_FLAG, 1u8);
    if e.global::<u8>(WATER_ENABLED) == 0 && setting_flag(e, SETTING_USE_WATER) {
        e.set_global(WATER_ENABLED, 1u8);
        fn_004e6370(e, this, true, false, false);
        return true;
    }
    false
}

// Translated from 004e6620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Disables the water system (no Xbox PDB name): clears `bWaterEnabled` and
/// the byte at `0x011ad86e`. With `release` set it also clears the texture
/// slots of every water shader property (the wading-water geometry of each
/// group and the references with the water owner type), gives the water
/// height, normal, rain height and depth maps and the wading-water height map
/// back to the texture manager, empties their slots, releases the objects
/// `fn_004e69d0`, `fn_004e6a70` and `fn_004e6a60` return and the water sound,
/// whatever of them exists. `iActiveWaterGroups` becomes 0. Unless
/// `keep_visibility` is set the water references are hidden (`004e6370(0, 0,
/// 0)`). Always returns true.
pub fn fn_004e6620(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    release: bool,
    keep_visibility: bool,
) -> bool {
    e.set_global(WATER_ENABLED, 0u8);
    e.set_global(WATER_REQUEST_FLAG, 0u8);
    if release {
        let groups = this.at(TESWaterSystem::PlaceableWaterGroupList).addr();
        for_each_list_item(e, groups, |e, group| {
            let group: Ptr<PlaceableWaterGroup> = Ptr::new(group);
            let geometry_slot = address_of(group, PlaceableWaterGroup::spWadingWaterGeometry);
            if pointer_in_slot(e, geometry_slot) != 0 {
                let geometry = pointer_in_slot(e, geometry_slot);
                let property = water_shader_property(e, geometry);
                clear_all_texture_slots(e, property);
            }
            let members = address_of(group, PlaceableWaterGroup::PlaceableWaterList);
            for_each_list_item(e, members, |e, reference| {
                if reference == 0 {
                    return;
                }
                let node = e.call(WATER_REFERENCE_3D, &args![this, reference]).u32();
                if node == 0 {
                    return;
                }
                let owner = e.call(NODE_OWNER, &args![node]).u32();
                if owner != 0 && e.call(OWNER_TYPE, &args![owner]).u32() == WATER_OWNER_TYPE {
                    let property = water_shader_property(e, node);
                    clear_all_texture_slots(e, property);
                }
            });
        });
        return_rendered_texture(e, address_of(this, TESWaterSystem::spWaterHeightMapTexture));
        return_rendered_texture(e, address_of(this, TESWaterSystem::spWaterNormalMapTexture));
        return_rendered_texture(e, address_of(this, TESWaterSystem::spRainHeightMapTexture));
        return_rendered_texture(e, DEPTH_MAP);
        for slot in [
            address_of(this, TESWaterSystem::spWaterHeightMapTexture),
            address_of(this, TESWaterSystem::spWaterNormalMapTexture),
            address_of(this, TESWaterSystem::spRainHeightMapTexture),
            address_of(this, TESWaterSystem::spRefractionDepthStencilBuffer),
            DEPTH_MAP,
        ] {
            assign_slot(e, slot, 0);
        }
        return_rendered_texture(e, WADING_WATER_HEIGHT_MAP);
        assign_slot(e, WADING_WATER_HEIGHT_MAP, 0);

        if fn_004e69d0(e) != 0 {
            let object = fn_004e69d0(e);
            fn_004e6a00(e, object, 0);
            let object = fn_004e69d0(e);
            fn_004e69e0(e, object, 0);
            let object = fn_004e69d0(e);
            fn_004e6a20(e, object, 0);
        }
        if fn_004e6a70(e) != 0 {
            let effect = fn_004e6a70(e);
            e.call(WATER_FFT_FREE_TEXTURE_MEMORY, &args![effect]);
        }
        if fn_004e6a60(e) != 0 {
            let object = fn_004e6a60(e);
            e.call(WATER_OBJECT_EMPTY_CALL, &args![object]);
        }
        let sound = address_of(this, TESWaterSystem::WaterSound);
        if e.call(SOUND_HANDLE_IS_VALID, &args![sound]).bool() {
            e.call(SOUND_HANDLE_RELEASE, &args![sound]);
        }
        if fn_004e69d0(e) != 0 {
            let object = fn_004e69d0(e);
            fn_004e6a40(e, object, 0);
        }
    }
    e.set_global(ACTIVE_WATER_GROUPS, 0u32);
    if !keep_visibility {
        fn_004e6370(e, this, false, false, false);
    }
    true
}

// Translated from 004e69d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pointer global at `0x011ff370`.
pub fn fn_004e69d0(e: &mut Engine) -> u32 {
    e.global(WATER_RENDER_OBJECT)
}

// Translated from 004e69e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the `NiPointer` at `+0x23c` of the `fn_004e69d0` object.
pub fn fn_004e69e0(e: &mut Engine, this: u32, value: u32) {
    assign_slot(e, this + 0x23c, value);
}

// Translated from 004e6a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the `NiPointer` at `+0x238` of the `fn_004e69d0` object.
pub fn fn_004e6a00(e: &mut Engine, this: u32, value: u32) {
    assign_slot(e, this + 0x238, value);
}

// Translated from 004e6a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the `NiPointer` at `+0x248` of the `fn_004e69d0` object.
pub fn fn_004e6a20(e: &mut Engine, this: u32, value: u32) {
    assign_slot(e, this + 0x248, value);
}

// Translated from 004e6a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the `NiPointer` at `+0x24c` of the `fn_004e69d0` object.
pub fn fn_004e6a40(e: &mut Engine, this: u32, value: u32) {
    assign_slot(e, this + 0x24c, value);
}

// Translated from 004e6a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pointer global at `0x011ffff8`.
pub fn fn_004e6a60(e: &mut Engine) -> u32 {
    e.global(WATER_OBJECT_011FFFF8)
}

// Translated from 004e6a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pointer global at `0x0120006c`.
pub fn fn_004e6a70(e: &mut Engine) -> u32 {
    e.global(WATER_FFT_EFFECT)
}

// Translated from 004e6a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::UpdateWaterSounds` (Xbox PDB): keeps the sound of the
/// water near the player. The water cell is the player's
/// (`PlayerCharacter::GetWaterCell` within `uNearWaterRadius`), or the
/// current cell without a player; without water the running sound is
/// stopped. `fraction` (0 to 1) is how much water there is around the
/// player: in an interior it falls from 1 to 0 as the player's distance to
/// the water height grows to `fNearWaterIndoorTolerance`. Outdoors a
/// `uNearWaterPoints` by `uNearWaterPoints` grid of side `2 *
/// uNearWaterRadius` around the player is sampled; the samples whose cell
/// water is higher than the land give the share of the grid that is water
/// and the centre of that water, and the height distance is weighed with
/// `fNearWaterOutdoorTolerance`. The sound sits at the centre of the water
/// moved towards the player by `uNearWaterRadius * (1 - fraction)`, at the
/// height of the water; it is stopped and released when `fraction` is 0 or
/// less, started when it is above 0, and created (from the water type of the
/// player's cell) when there is no sound yet. With
/// `bDisplayWaterSoundPlacement` set, debug markers are added. The flag is
/// cleared at the end.
pub fn tes_water_system_update_water_sounds(e: &mut Engine, this: Ptr<TESWaterSystem>) {
    let tes = e.global::<u32>(TES_POINTER);
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let sound = address_of(this, TESWaterSystem::WaterSound);
    let cell = if player != 0 {
        let radius = fn_004e7600(e, this.addr());
        e.call(PLAYER_GET_WATER_CELL, &args![player, radius as f32])
            .u32()
    } else {
        e.call(TES_GET_CURRENT_CELL, &args![tes]).u32()
    };
    if cell == 0 || !e.call(CELL_HAS_WATER, &args![cell]).bool() {
        if e.global::<u8>(WATER_SOUND_STARTED) != 0
            && e.call(SOUND_HANDLE_IS_VALID, &args![sound]).bool()
        {
            e.set_global(WATER_SOUND_STARTED, 0u8);
            e.call(SOUND_HANDLE_STOP, &args![sound]);
        }
        e.set_global(DISPLAY_WATER_SOUND_PLACEMENT, 0u8);
        return;
    }

    e.with_stack(0xe8, |e, frame| {
        let frame = frame.addr();
        // The sum (then the position) of the sound: an `NiPoint2`.
        let sum = frame;
        let sample = frame + 0x08;
        let land_height = frame + 0x14;
        let made = frame + 0x18;
        let made_again = frame + 0x24;
        let color = frame + 0x30;
        let scaled = frame + 0x40;
        let player_point = frame + 0x48;
        let marker_point = frame + 0x54;
        let color_b = frame + 0x60;
        let color_c = frame + 0x70;
        let handle = frame + 0x80;
        let buffer_a = frame + 0xa0;
        let buffer_b = frame + 0xc4;

        let water_height = e.call(CELL_GET_WATER_HEIGHT, &args![cell]).f32();
        copy_words(e, ZERO_POINT2, sum, 2);
        let mut fraction: f32;
        if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
            let tolerance = fn_004e7640(e, this.addr()) as f64;
            let position = e.call(PLAYER_POSITION, &args![player]).u32();
            let player_z = e.mem.f32(position + 8);
            if water_skipped(e) {
                fraction = 0.0;
            } else {
                let difference = (water_height as f64 - player_z as f64) as f32;
                let distance = e.call(FLOAT_ABS, &args![difference]).f32();
                if (distance as f64) < tolerance {
                    let difference = (water_height as f64 - player_z as f64) as f32;
                    let distance = e.call(FLOAT_ABS, &args![difference]).f32();
                    fraction = ((tolerance - distance as f64) / tolerance) as f32;
                } else {
                    fraction = 0.0;
                }
            }
        } else {
            let points = fn_004e7620(e, this.addr()) as f32;
            let radius = fn_004e7600(e, this.addr()) as f32;
            let tolerance = fn_004e7660(e, this.addr()) as f64;
            let two: f64 = e.global(TWO);
            let one: f64 = e.global(ONE);
            let minus_one: f64 = e.global(MINUS_ONE_DOUBLE);
            let ten: f64 = e.global(TEN);
            let mut count = 0u32;
            let step = ((radius as f64 * two) / (points as f64 - one)) as f32;
            let position = e.call(PLAYER_POSITION, &args![player]).u32();
            let player_z = e.mem.f32(position + 8);
            let mut cached_cell = 0u32;
            let mut i = 0i32;
            while (i as f64) < points as f64 {
                let mut j = 0i32;
                while (j as f64) < points as f64 {
                    let position = e.call(PLAYER_POSITION, &args![player]).u32();
                    copy_words(e, position, sample, 3);
                    e.mem.set_f32(land_height, 0.0);
                    let x = (radius as f64 * minus_one
                        + i as f64 * step as f64
                        + e.mem.f32(sample) as f64) as f32;
                    e.mem.set_f32(sample, x);
                    let y = (radius as f64 * minus_one
                        + j as f64 * step as f64
                        + e.mem.f32(sample + 4) as f64) as f32;
                    e.mem.set_f32(sample + 4, y);

                    let mut inside_cached_cell = false;
                    if cached_cell != 0 {
                        e.call(NI_POINT3_CONSTRUCT, &args![made, x, y, 0.0f32]);
                        inside_cached_cell = e
                            .call(CELL_CONTAINS_POINT, &args![cached_cell, made])
                            .bool();
                    }
                    if !inside_cached_cell {
                        e.call(NI_POINT3_CONSTRUCT, &args![made_again, x, y, 0.0f32]);
                        let world_space = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
                        cached_cell = e
                            .call(WORLD_SPACE_GET_CELL, &args![world_space, made_again])
                            .u32();
                    }
                    let height_here = if cached_cell != 0 {
                        e.call(CELL_GET_WATER_HEIGHT, &args![cached_cell]).f32()
                    } else {
                        e.global::<f32>(LOWEST_FLOAT)
                    };
                    let on_land = e
                        .call(TES_GET_LAND_HEIGHT, &args![tes, sample, land_height])
                        .bool();
                    let land = e.mem.f32(land_height);
                    let is_water = on_land && land < height_here;
                    if is_water {
                        count += 1;
                        let total = (e.mem.f32(sum) as f64 + x as f64) as f32;
                        e.mem.set_f32(sum, total);
                        let total = (e.mem.f32(sum + 4) as f64 + y as f64) as f32;
                        e.mem.set_f32(sum + 4, total);
                    }
                    if e.global::<u8>(DISPLAY_WATER_SOUND_PLACEMENT) != 0 {
                        let rgba = if is_water {
                            [0.0f32, 1.0, 0.0, 0.0]
                        } else {
                            [0.0f32, 0.0, 1.0, 0.0]
                        };
                        e.call(
                            NI_COLOR_CONSTRUCT,
                            &args![color, rgba[0], rgba[1], rgba[2], rgba[3]],
                        );
                        let size = e.global::<f32>(TEN_FLOAT);
                        let marker = e.call(MAKE_TRI_POINT, &args![size, color, 1u32]).u32();
                        let water = e.call(CELL_GET_WATER_HEIGHT, &args![cell]).f32();
                        let top = e.call(FLOAT_MAX, &args![water, land]).f32();
                        let z = (top as f64 + ten) as f32;
                        let at = e
                            .call(NI_POINT3_CONSTRUCT, &args![marker_point, x, y, z])
                            .u32();
                        e.call(NODE_SET_LOCAL_TRANSLATE, &args![marker, at]);
                        let seconds = e.global::<f32>(TEN_FLOAT);
                        e.call(TES_ADD_TEMP_DEBUG_OBJECT, &args![tes, marker, seconds]);
                    }
                    j += 1;
                }
                i += 1;
            }
            fn_004e7530(e, sum, count as f32);
            let position = e.call(PLAYER_POSITION, &args![player]).u32();
            let value = (e.mem.f32(sum) as f64 - e.mem.f32(position) as f64) as f32;
            e.mem.set_f32(sum, value);
            let position = e.call(PLAYER_POSITION, &args![player]).u32();
            let value = (e.mem.f32(sum + 4) as f64 - e.mem.f32(position + 4) as f64) as f32;
            e.mem.set_f32(sum + 4, value);
            fraction = ((count as f64 * two) / (points as f64 * points as f64)) as f32;
            if water_skipped(e) {
                fraction = 0.0;
            } else {
                let difference = (water_height as f64 - player_z as f64) as f32;
                let distance = e.call(FLOAT_ABS, &args![difference]).f32();
                if (distance as f64) < tolerance {
                    let difference = (water_height as f64 - player_z as f64) as f32;
                    let distance = e.call(FLOAT_ABS, &args![difference]).f32();
                    let weight = ((tolerance - distance as f64) / tolerance) as f32;
                    fraction = (weight as f64 * fraction as f64) as f32;
                } else {
                    fraction = 0.0;
                }
            }
        }

        let one: f64 = e.global(ONE);
        if fraction as f64 > one {
            fraction = 1.0;
        }
        fn_004e7560(e, sum);
        let remaining = one - fraction as f64;
        let radius = fn_004e7600(e, this.addr());
        let distance = (radius as f64 * remaining) as f32;
        let product = e.call(POINT2_SCALE, &args![sum, scaled, distance]).u32();
        copy_words(e, product, sum, 2);
        let position = e.call(PLAYER_POSITION, &args![player]).u32();
        let value = (e.mem.f32(sum) as f64 + e.mem.f32(position) as f64) as f32;
        e.mem.set_f32(sum, value);
        let position = e.call(PLAYER_POSITION, &args![player]).u32();
        let value = (e.mem.f32(sum + 4) as f64 + e.mem.f32(position + 4) as f64) as f32;
        e.mem.set_f32(sum + 4, value);

        let zero: f64 = e.global(ZERO);
        if e.call(SOUND_HANDLE_IS_VALID, &args![sound]).bool() {
            if e.global::<u8>(DISPLAY_WATER_SOUND_PLACEMENT) != 0 {
                let location = e.vcall(player, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
                copy_words(e, location, player_point, 3);
                let ten: f64 = e.global(TEN);
                let raised = (e.mem.f32(player_point + 8) as f64 + ten) as f32;
                e.mem.set_f32(player_point + 8, raised);
                let (x, y) = (e.mem.f32(sum), e.mem.f32(sum + 4));
                e.call(NI_POINT3_CONSTRUCT, &args![marker_point, x, y, raised]);
                e.call(
                    NI_COLOR_CONSTRUCT,
                    &args![color, 1.0f32, 0.0f32, 0.0f32, 0.0f32],
                );
                let size = e.global::<f32>(TEN_FLOAT);
                let marker = e.call(MAKE_TRI_POINT, &args![size, color, 1u32]).u32();
                e.call(NODE_SET_LOCAL_TRANSLATE, &args![marker, marker_point]);
                let seconds = e.global::<f32>(TEN_FLOAT);
                e.call(TES_ADD_TEMP_DEBUG_OBJECT, &args![tes, marker, seconds]);
                e.call(
                    NI_COLOR_CONSTRUCT,
                    &args![color_b, 1.0f32, 1.0f32, 0.0f32, 0.0f32],
                );
                e.call(
                    NI_COLOR_CONSTRUCT,
                    &args![color_c, 1.0f32, 1.0f32, 0.0f32, 0.0f32],
                );
                let line = e
                    .call(
                        MAKE_DEBUG_LINE,
                        &args![player_point, color_c, marker_point, color_b, 1u32],
                    )
                    .u32();
                let seconds = e.global::<f32>(TEN_FLOAT);
                e.call(TES_ADD_TEMP_DEBUG_OBJECT, &args![tes, line, seconds]);
            }
            let height = e.call(CELL_GET_WATER_HEIGHT, &args![cell]).f32();
            let (x, y) = (e.mem.f32(sum), e.mem.f32(sum + 4));
            e.call(SOUND_HANDLE_SET_POSITION, &args![sound, x, y, height]);
            if e.call(SOUND_HANDLE_IS_PLAYING, &args![sound]).bool() && fraction as f64 <= zero {
                e.set_global(WATER_SOUND_STARTED, 0u8);
                e.call(SOUND_HANDLE_STOP, &args![sound]);
                e.call(SOUND_HANDLE_RELEASE, &args![sound]);
            } else if !e.call(SOUND_HANDLE_IS_PLAYING, &args![sound]).bool()
                && fraction as f64 > zero
            {
                e.set_global(WATER_SOUND_STARTED, 1u8);
                e.call(SOUND_HANDLE_PLAY, &args![sound, 1u32]);
            }
        } else if fraction as f64 > zero && player != 0 {
            let player_cell = e.call(REFERENCE_PARENT_CELL, &args![player]).u32();
            if player_cell != 0 {
                let water_type = e.call(CELL_GET_WATER_TYPE, &args![player_cell]).u32();
                if water_type != 0 && e.call(WATER_TYPE_SOUND, &args![water_type]).u32() != 0 {
                    let water_sound = |e: &mut Engine| {
                        let cell = e.call(REFERENCE_PARENT_CELL, &args![player]).u32();
                        let water_type = e.call(CELL_GET_WATER_TYPE, &args![cell]).u32();
                        e.call(WATER_TYPE_SOUND, &args![water_type]).u32()
                    };
                    let water_sound_a = water_sound(e);
                    let copy = fn_004e75d0(e, water_sound_a, buffer_a);
                    let first_word = e.mem.u32(copy + 4);
                    let audio_flags = e.call(SOUND_FLAGS_TO_AUDIO_FLAGS, &args![first_word]).u32();
                    let copy = fn_004e75d0(e, water_sound_a, buffer_b);
                    let second_word = e.mem.u32(copy + 4);
                    let mask = if second_word & 0x40 != 0 {
                        0x07ff_ffff
                    } else {
                        0
                    };
                    let numeric_id = (mask + 2) | audio_flags;
                    let water_sound_b = water_sound(e);
                    let sound_number = e.call(READ_WORD_AT_0C, &args![water_sound_b]).u32();
                    let audio = e.call(AUDIO_INSTANCE, &[]).u32();
                    let created = e
                        .call(
                            AUDIO_GET_SOUND_HANDLE,
                            &args![audio, handle, sound_number, numeric_id],
                        )
                        .u32();
                    e.call(SOUND_HANDLE_ASSIGN, &args![sound, created]);
                    e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
                }
            }
        }
    });
    e.set_global(DISPLAY_WATER_SOUND_PLACEMENT, 0u8);
}

// Translated from 004e7530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint2::operator/=(float)` (no Xbox PDB name): divides both floats of
/// the point by `divisor`. Returns `this`.
pub fn fn_004e7530(e: &mut Engine, this: u32, divisor: f32) -> u32 {
    let x = (e.mem.f32(this) as f64 / divisor as f64) as f32;
    e.mem.set_f32(this, x);
    let y = (e.mem.f32(this + 4) as f64 / divisor as f64) as f32;
    e.mem.set_f32(this + 4, y);
    this
}

// Translated from 004e7560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint2::Unitize` (no Xbox PDB name): scales the point to length one
/// (each float times `1 / length`) and returns the length; a point of length
/// `1e-6` or less (or not a number) becomes (0, 0) and the result is 0.
pub fn fn_004e7560(e: &mut Engine, this: u32) -> f32 {
    let length = e.call(POINT2_LENGTH, &args![this]).f32();
    let limit: f64 = e.global(LENGTH_LIMIT);
    if (length as f64) <= limit || length.is_nan() {
        e.mem.set_f32(this, 0.0);
        e.mem.set_f32(this + 4, 0.0);
        return 0.0;
    }
    let inverse = (1.0f64 / length as f64) as f32;
    let x = (e.mem.f32(this) as f64 * inverse as f64) as f32;
    e.mem.set_f32(this, x);
    let y = (e.mem.f32(this + 4) as f64 * inverse as f64) as f32;
    e.mem.set_f32(this + 4, y);
    length
}

// Translated from 004e75d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the nine words at `this + 0x44` to `out` (no Xbox PDB name; a
/// folded accessor). Returns `out`.
pub fn fn_004e75d0(e: &mut Engine, this: u32, out: u32) -> u32 {
    copy_words(e, this + 0x44, out, 9);
    out
}

// Translated from 004e7600 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of the `uNearWaterRadius:Water` setting. `this` is not read.
pub fn fn_004e7600(e: &mut Engine, _unused_0: u32) -> u32 {
    setting_int(e, SETTING_NEAR_WATER_RADIUS)
}

// Translated from 004e7620 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of the `uNearWaterPoints:Water` setting. `this` is not read.
pub fn fn_004e7620(e: &mut Engine, _unused_0: u32) -> u32 {
    setting_int(e, SETTING_NEAR_WATER_POINTS)
}

// Translated from 004e7640 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of the `fNearWaterIndoorTolerance:Water` setting. `this` is not
/// read.
pub fn fn_004e7640(e: &mut Engine, _unused_0: u32) -> f32 {
    setting_float(e, SETTING_NEAR_WATER_INDOOR_TOLERANCE)
}

// Translated from 004e7660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of the `fNearWaterOutdoorTolerance:Water` setting. `this` is
/// not read.
pub fn fn_004e7660(e: &mut Engine, _unused_0: u32) -> f32 {
    setting_float(e, SETTING_NEAR_WATER_OUTDOOR_TOLERANCE)
}

// Translated from 004e7680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates an `NiTriShape` for `data` (no Xbox PDB name): a 0xc4-byte
/// object built by `NiTriShape::NiTriShape_ov2(data)` inside the memory
/// scope of `TESWater.cpp` line `0xb91`. Returns null when the allocation
/// fails. `this` is not read.
pub fn fn_004e7680(e: &mut Engine, _unused_0: u32, data: u32) -> u32 {
    e.with_stack(4, |e, scope| {
        e.call(
            ALLOCATION_SCOPE_CONSTRUCT,
            &args![scope, 0x1du32, 1u32, TESWATER_SOURCE_PATH, 0xb91u32],
        );
        let memory = e.call(NI_ALLOC, &args![0xc4u32]).u32();
        let shape = if memory == 0 {
            0
        } else {
            e.call(TRI_SHAPE_CONSTRUCT, &args![memory, data]).u32()
        };
        e.call(ALLOCATION_SCOPE_DESTRUCT, &args![scope]);
        shape
    })
}

// Translated from 004e7730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `CreateQuadData(size, size, ignored_word, texture_scale, normals,
/// colors)`: a square quad of side `size`. The word after `size` is passed on
/// and `CreateQuadData` never reads it (the one caller passes `0x200`).
pub fn fn_004e7730(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    size: f32,
    ignored_word: u32,
    texture_scale: u32,
    normals: bool,
    colors: bool,
) -> u32 {
    tes_water_system_create_quad_data(
        e,
        this.addr(),
        size,
        size,
        ignored_word,
        texture_scale,
        normals,
        colors,
    )
}

// Translated from 004e7770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::CreateQuadData` (Xbox PDB): builds the `NiTriShapeData`
/// of a quad in the XY plane centred on the origin, `size_x` by `size_y`
/// (four vertices, the triangles 0-1-2 and 0-2-3). With `normals` the four
/// normals are `(0, 0, 1)`; with `colors` the four vertex colours are white
/// with the alpha of `fAlpha:Water` (clamped to 0..1 in the setting). The
/// texture coordinates run from 0 to `texture_scale` (1 when it is 1). Returns
/// null (after freeing what was allocated) when an allocation or the data
/// construction fails. The arrays are allocated in the memory scope of
/// `TESWater.cpp` line `0xbac`.
///
/// `this` and the word after the sizes are not read.
#[allow(clippy::too_many_arguments)]
pub fn tes_water_system_create_quad_data(
    e: &mut Engine,
    _unused_0: u32,
    size_x: f32,
    size_y: f32,
    _unused_3: u32,
    texture_scale: u32,
    normals: bool,
    colors: bool,
) -> u32 {
    e.with_stack(4, |e, scope| {
        e.call(
            ALLOCATION_SCOPE_CONSTRUCT,
            &args![scope, 0x1du32, 1u32, TESWATER_SOURCE_PATH, 0xbacu32],
        );
        let data = quad_data_in_scope(e, size_x, size_y, texture_scale, normals, colors);
        e.call(ALLOCATION_SCOPE_DESTRUCT, &args![scope]);
        data
    })
}

/// Allocates `count` elements of `element_size` bytes (`size` bytes in all)
/// and runs `_vector_constructor_iterator_` with `constructor` on them;
/// null when the allocation fails.
fn new_constructed_array(
    e: &mut Engine,
    size: u32,
    element_size: u32,
    count: u32,
    constructor: u32,
) -> u32 {
    let memory = e.call(OPERATOR_NEW, &args![size]).u32();
    if memory != 0 {
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![memory, element_size, count, constructor],
        );
    }
    memory
}

/// The body of `CreateQuadData` between the creation and the destruction of
/// the memory scope.
fn quad_data_in_scope(
    e: &mut Engine,
    size_x: f32,
    size_y: f32,
    texture_scale: u32,
    normals: bool,
    colors: bool,
) -> u32 {
    let vertices = new_constructed_array(e, 0x30, 0x0c, 4, ADDRESS_OF_THIS);
    if vertices == 0 {
        return 0;
    }
    let two: f64 = e.global(TWO);
    let half_x = (size_x as f64 / two) as f32;
    let half_y = (size_y as f64 / two) as f32;
    let minus_half_x = (-(size_x as f64) / two) as f32;
    let minus_half_y = (-(size_y as f64) / two) as f32;
    e.with_stack(0x0c, |e, corner| {
        for (index, (x, y)) in [
            (half_x, half_y),
            (minus_half_x, half_y),
            (minus_half_x, minus_half_y),
            (half_x, minus_half_y),
        ]
        .into_iter()
        .enumerate()
        {
            let made = e
                .call(NI_POINT3_CONSTRUCT, &args![corner, x, y, 0.0f32])
                .u32();
            copy_words(e, made, vertices + 0x0c * index as u32, 3);
        }
    });
    if e.global::<u32>(QUAD_STATICS_BUILT) & 1 == 0 {
        let built = e.global::<u32>(QUAD_STATICS_BUILT);
        e.set_global(QUAD_STATICS_BUILT, built | 1);
        e.call(
            NI_POINT3_CONSTRUCT,
            &args![QUAD_NORMAL, 0.0f32, 0.0f32, 1.0f32],
        );
    }
    let uvs = new_constructed_array(e, 0x20, 0x08, 4, ADDRESS_OF_THIS);
    if uvs == 0 {
        e.call(OPERATOR_DELETE, &args![vertices]);
        return 0;
    }
    let triangles = e.call(NI_ALLOC_ARRAY, &args![0x0cu32]).u32();
    if triangles == 0 {
        e.call(OPERATOR_DELETE, &args![vertices]);
        e.call(OPERATOR_DELETE, &args![uvs]);
        return 0;
    }
    for (index, value) in [0u16, 1, 2, 0, 2, 3].into_iter().enumerate() {
        e.mem.set_u16(triangles + 2 * index as u32, value);
    }

    let mut normal_array = 0;
    if normals {
        normal_array = new_constructed_array(e, 0x30, 0x0c, 4, ADDRESS_OF_THIS);
        if normal_array != 0 {
            for index in 0..4 {
                copy_words(e, QUAD_NORMAL, normal_array + 0x0c * index, 3);
            }
        }
    }

    let mut color_array = 0;
    if colors {
        color_array = new_constructed_array(e, 0x40, 0x10, 4, NI_COLOR_DEFAULT_CONSTRUCT);
        if color_array != 0 {
            if e.global::<u32>(QUAD_STATICS_BUILT) & 2 == 0 {
                let built = e.global::<u32>(QUAD_STATICS_BUILT);
                e.set_global(QUAD_STATICS_BUILT, built | 2);
                e.call(
                    FLOAT_SETTING_CONSTRUCT,
                    &args![SETTING_WATER_ALPHA, ALPHA_SETTING_NAME, 1.0f32],
                );
                e.call(ATEXIT, &args![WATER_ALPHA_SETTING_DESTRUCT]);
            }
            if setting_float(e, SETTING_WATER_ALPHA) as f64 > e.global::<f64>(ONE) {
                e.call(FLOAT_SETTING_SET, &args![SETTING_WATER_ALPHA, 1.0f32]);
            } else if (setting_float(e, SETTING_WATER_ALPHA) as f64) < e.global::<f64>(ZERO) {
                e.call(FLOAT_SETTING_SET, &args![SETTING_WATER_ALPHA, 0.0f32]);
            }
            e.with_stack(0x10, |e, color| {
                for index in 0..4 {
                    let alpha = e.call(FLOAT_SETTING_GET, &args![SETTING_WATER_ALPHA]).f32();
                    let made = e
                        .call(
                            NI_COLOR_CONSTRUCT,
                            &args![color, 1.0f32, 1.0f32, 1.0f32, alpha],
                        )
                        .u32();
                    copy_words(e, made, color_array + 0x10 * index, 4);
                }
            });
        }
    }

    let scale = if texture_scale == 1 {
        1.0f32
    } else {
        texture_scale as f32
    };
    e.with_stack(0x08, |e, point| {
        for (index, (x, y)) in [(scale, 0.0f32), (0.0, 0.0), (0.0, scale), (scale, scale)]
            .into_iter()
            .enumerate()
        {
            let made = e.call(NI_POINT2_CONSTRUCT, &args![point, x, y]).u32();
            copy_words(e, made, uvs + 8 * index as u32, 2);
        }
    });

    let memory = e.call(NI_ALLOC, &args![0x58u32]).u32();
    let data = if memory == 0 {
        0
    } else {
        e.call(
            TRI_SHAPE_DATA_CONSTRUCT,
            &args![
                memory,
                4u32,
                vertices,
                normal_array,
                color_array,
                uvs,
                1u32,
                0u32,
                2u32,
                triangles
            ],
        )
        .u32()
    };
    if data == 0 {
        for array in [vertices, normal_array, uvs, triangles, color_array] {
            e.call(OPERATOR_DELETE, &args![array]);
        }
    }
    data
}

// Translated from 004e7ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointer<T>::operator T*` on the static at `0x011c7c28` (the water root
/// node, `spWaterRoot`).
pub fn fn_004e7ff0(e: &mut Engine) -> u32 {
    e.call(NI_POINTER_GET, &args![WATER_ROOT_SLOT]).u32()
}

// ---------------------------------------------------------------------------
// Helpers of the functions from `004e8000` on (third session).

/// Whether `test` is true for an item of an `NiTPointerList`: the same walk
/// as `for_each_list_item` (the next position first, then the item), but it
/// stops at the first item `test` accepts.
fn any_list_item(
    e: &mut Engine,
    list: u32,
    mut test: impl FnMut(&mut Engine, u32) -> bool,
) -> bool {
    let mut position = pointer_in_slot(e, list);
    while position != 0 {
        let next = e.call(LIST_NEXT_POSITION, &args![list, position]).u32();
        let slot = e.call(LIST_ITEM_SLOT, &args![list, position]).u32();
        let item = e.mem.u32(slot);
        if test(e, item) {
            return true;
        }
        position = next;
    }
    false
}

/// `004b3ab0` applied to the displacement offsets: maps `value` from the
/// range `-512..512` onto `-0.5..0.5` (the four floats are read from the exe
/// in the order the game pushes them).
fn map_displacement(e: &mut Engine, value: f32) -> f32 {
    let low = e.global::<f32>(MAP_LOW);
    let high = e.global::<f32>(MAP_HIGH);
    let source_low = e.global::<f32>(MAP_SOURCE_LOW);
    let source_high = e.global::<f32>(MAP_SOURCE_HIGH);
    e.call(
        LINEAR_MAP,
        &args![low, high, source_low, source_high, value],
    )
    .f32()
}

/// `value / 2.0` computed in extended precision and stored as a `float`.
fn half_of(e: &mut Engine, value: f32) -> f32 {
    let two = e.global::<f64>(TWO);
    (value as f64 / two) as f32
}

/// `a - b` computed in extended precision and stored as a `float`.
fn float_difference(a: f32, b: f32) -> f32 {
    (a as f64 - b as f64) as f32
}

/// Allocates (`NiAlloc`) and constructs an `NiCamera` and stores it in the
/// `NiPointer` slot (null when the allocation fails).
fn assign_new_camera(e: &mut Engine, slot: u32) {
    let memory = e.call(NI_ALLOC, &args![0x114u32]).u32();
    let camera = if memory == 0 {
        0
    } else {
        e.call(CAMERA_CONSTRUCT, &args![memory]).u32()
    };
    assign_slot(e, slot, camera);
}

/// The same for a `BSShaderAccumulator` (0x280 bytes).
fn assign_new_accumulator(e: &mut Engine, slot: u32) {
    let memory = e.call(NI_ALLOC, &args![0x280u32]).u32();
    let accumulator = if memory == 0 {
        0
    } else {
        e.call(ACCUMULATOR_CONSTRUCT, &args![memory, 99u32, 1u32, 0x2f7u32])
            .u32()
    };
    assign_slot(e, slot, accumulator);
}

/// What every reflection setup does to its accumulator once the camera is
/// in place: the render-table word 0 into +0x194, `fn_004ea860(1)`, the
/// camera into the accumulator's virtual at +0x8c, and `bAccumulate`.
fn prepare_accumulator(e: &mut Engine, sorter_slot: u32, camera_slot: u32) {
    let table_entry = e.call(RENDER_TABLE_ENTRY, &args![0u32]).u32();
    let accumulator = pointer_in_slot(e, sorter_slot);
    e.call(ACCUMULATOR_SET_WORD_194, &args![accumulator, table_entry]);
    let accumulator = pointer_in_slot(e, sorter_slot);
    fn_004ea860(e, accumulator, 1);
    let accumulator = pointer_in_slot(e, sorter_slot);
    let camera = pointer_in_slot(e, camera_slot);
    e.vcall(accumulator, ACCUMULATOR_SET_CAMERA_VIRTUAL, &args![camera]);
    let accumulator = pointer_in_slot(e, sorter_slot);
    e.call(ACCUMULATOR_SET_ACCUMULATE, &args![accumulator, 1u32]);
}

/// The silhouette colour of an accumulator: +0x19c is set to 0xf, and the
/// colour is the sky's three floats times the three floats `fn_004ea980`
/// leads to, with alpha 1.0.
fn set_silhouette_color(e: &mut Engine, sorter_slot: u32) {
    let accumulator = pointer_in_slot(e, sorter_slot);
    e.call(ACCUMULATOR_SET_WORD_19C, &args![accumulator, 0xfu32]);
    let tes = e.global::<u32>(TES_POINTER);
    let sky = e.call(TES_GET_SKY, &args![tes]).u32();
    let sky_color = fn_004ea950(e, sky);
    let color: Vec<f32> = (0..3).map(|i| e.mem.f32(sky_color + 4 * i)).collect();
    let table_entry = e.call(RENDER_TABLE_ENTRY, &args![0u32]).u32();
    let holder = fn_004ea980(e, table_entry);
    let factor_address = e.call(COLOR_FACTOR_ADDRESS, &args![holder]).u32();
    let factor: Vec<f32> = (0..3).map(|i| e.mem.f32(factor_address + 4 * i)).collect();
    e.with_stack(0x10, |e, value| {
        e.call(
            NI_POINT4_CONSTRUCT,
            &args![
                value,
                color[0] * factor[0],
                color[1] * factor[1],
                color[2] * factor[2],
                1.0f32
            ],
        );
        let accumulator = pointer_in_slot(e, sorter_slot);
        fn_004ea880(e, accumulator, value.addr());
    });
}

/// Hands the accumulator's work to the multithreaded renderer: the
/// `AddAccumTask` and `SetThreadStage` calls that end every reflection
/// setup.
fn add_accumulator_task(
    e: &mut Engine,
    camera_slot: u32,
    static_objects: u32,
    dynamic_objects: u32,
    sorter_slot: u32,
    stage: i32,
) {
    let accumulator = pointer_in_slot(e, sorter_slot);
    let camera = pointer_in_slot(e, camera_slot);
    let renderer = fn_004ea970(e);
    e.call(
        MT_ADD_ACCUM_TASK,
        &args![
            renderer,
            camera,
            0u32,
            0u32,
            static_objects,
            dynamic_objects,
            accumulator,
            4u32,
            stage,
            0u32
        ],
    );
    let renderer = fn_004ea970(e);
    e.call(MT_SET_THREAD_STAGE, &args![renderer, 0u32, stage]);
}

/// Builds the reflection plane the world and sky setups use: normal `(0, 0,
/// 1)` through `(0, 0, h)` with `h` the world space's water height, into
/// `plane` (0x10 bytes) with `normal` and `point` (0xc bytes each) as
/// scratch.
fn build_world_water_plane(e: &mut Engine, plane: u32, normal: u32, point: u32) {
    e.call(NI_POINT3_CONSTRUCT, &args![normal, 0.0f32, 0.0f32, 1.0f32]);
    let tes = e.global::<u32>(TES_POINTER);
    let world_space = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
    let height = e.call(WORLD_SPACE_WATER_HEIGHT, &args![world_space]).f32();
    e.call(NI_POINT3_CONSTRUCT, &args![point, 0.0f32, 0.0f32, height]);
    e.call(NI_PLANE_CONSTRUCT, &args![plane, normal, point]);
}

/// `TES::GetWorldSpace` on the `TES` global.
fn current_world_space(e: &mut Engine) -> u32 {
    let tes = e.global::<u32>(TES_POINTER);
    e.call(TES_GET_WORLD_SPACE, &args![tes]).u32()
}

// ---------------------------------------------------------------------------
// Translated functions, third session (`004e8000` on).

// Translated from 004e8000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drops the water root node (`spWaterRoot`, the static `NiPointer` at
/// `0x011c7c28`): when there is one, `004def90` runs on it, and the slot is
/// set to null.
pub fn fn_004e8000(e: &mut Engine) {
    if pointer_in_slot(e, WATER_ROOT_SLOT) != 0 {
        let root = pointer_in_slot(e, WATER_ROOT_SLOT);
        e.call(WATER_ROOT_RELEASE_STEP, &args![root]);
    }
    assign_slot(e, WATER_ROOT_SLOT, 0);
}

// Translated from 004e8030 (decompiled, FalloutNV.exe 1.4.0.525)
/// The geometry of a water reference (no Xbox PDB name; the counter it
/// raises is the one the Xbox PDB calls `iGlobalGetWaterGeometryCount`): the
/// reference's 3D object must be a node whose first child is a node; the
/// result is that child's first child, or, when the reference's base form
/// has the form flag `0x40000000`, the first child of the child's first
/// child (null when that is not a node). Null when any step fails.
pub fn fn_004e8030(e: &mut Engine, this: Ptr<TESWaterSystem>, reference: u32) -> u32 {
    if reference == 0 || reference_node(e, reference) == 0 {
        return 0;
    }
    let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
    let count = e.get(this, TESWaterSystem::iGlobalGetWaterGeometryCount);
    e.set(
        this,
        TESWaterSystem::iGlobalGetWaterGeometryCount,
        count.wrapping_add(1),
    );
    if reference_node(e, reference) == 0 {
        return 0;
    }
    let node = reference_node(e, reference);
    if e.vcall(node, NODE_IS_NODE_VIRTUAL, &[]).u32() == 0 {
        return 0;
    }
    let node = reference_node(e, reference);
    let child = e.call(NODE_FIRST_CHILD, &args![node, 0u32]).u32();
    if child == 0 || e.vcall(child, NODE_IS_NODE_VIRTUAL, &[]).u32() == 0 {
        return 0;
    }
    let grandchild = e.call(NODE_FIRST_CHILD, &args![child, 0u32]).u32();
    if base_form == 0 || !e.call(FORM_FLAG_TEST_40000000, &args![base_form]).bool() {
        return grandchild;
    }
    if grandchild != 0 && e.vcall(grandchild, NODE_IS_NODE_VIRTUAL, &[]).u32() != 0 {
        return e.call(NODE_FIRST_CHILD, &args![grandchild, 0u32]).u32();
    }
    0
}

// Translated from 004e8160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the wading-water geometry of `group` (no Xbox PDB name): a square
/// quad of side `size` (`fn_004e7730` with texture scale 1, normals and
/// colours) in an `NiTriShape` at the player's position, hung on the water
/// root node, with a new `WaterShaderProperty` (displacement on, blend radius
/// and blend-normals amount 1.0, the group's stencil mask, the group's noise
/// map and the shared depth and wading height maps). When the group's first
/// water reference has a water shader property, that property is handed the
/// new one (`00b6ab20`). Resets `fTimeSinceLastRipplePlaced` and returns the
/// shape.
///
/// The word after `group` (the callers pass the water root) is not read.
pub fn fn_004e8160(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    group: Ptr<PlaceableWaterGroup>,
    _unused_2: u32,
    size: f32,
) -> u32 {
    let data = fn_004e7730(e, this, size, 0x200, 1, true, true);
    let shape = fn_004e7680(e, this.addr(), data);
    e.with_stack(0x0c, |e, origin| {
        e.call(NI_POINT3_CONSTRUCT, &args![origin, 0.0f32, 0.0f32, 0.0f32]);
        let player = e.global::<u32>(PLAYER_CHARACTER);
        if player == 0 {
            e.call(NODE_SET_LOCAL_TRANSLATE, &args![shape, origin]);
        } else {
            let position = e.call(PLAYER_POSITION, &args![player]).u32();
            e.call(NODE_SET_LOCAL_TRANSLATE, &args![shape, position]);
        }
    });
    let root = fn_004e7ff0(e);
    e.vcall(root, NODE_ATTACH_CHILD, &args![shape, 1u32]);
    let memory = e.call(NI_ALLOC, &args![0x150u32]).u32();
    let property: Ptr<WaterShaderProperty> = if memory == 0 {
        Ptr::new(0)
    } else {
        e.call(WATER_SHADER_PROPERTY_CONSTRUCT, &args![memory])
            .ptr()
    };
    e.call(NODE_ATTACH_PROPERTY, &args![shape, property]);
    e.call(SHADER_MANAGER_PREPARE_OBJECT, &args![shape, 0u32, 0u32]);
    e.set(property, WaterShaderProperty::fBlendNormalsAmount, 1.0);

    let first_slot = e
        .call(
            LIST_FIRST_ITEM_SLOT,
            &args![group.at(PlaceableWaterGroup::PlaceableWaterList)],
        )
        .u32();
    let first_reference = e.mem.u32(first_slot);
    if first_reference != 0 {
        let geometry = fn_004e8030(e, this, first_reference);
        if geometry != 0 {
            let owner = e.call(NODE_OWNER, &args![geometry]).u32();
            if owner != 0 && e.call(OWNER_TYPE, &args![owner]).u32() == WATER_OWNER_TYPE {
                let other = water_shader_property(e, geometry);
                e.call(WATER_PROPERTY_METHOD_00B6AB20, &args![other, property]);
            }
        }
    }

    assign_group_reflection_map(e, property, group);
    e.call(
        NI_POINTER_ASSIGN_FROM,
        &args![
            address_of(property, WaterShaderProperty::spDepthMap),
            DEPTH_MAP
        ],
    );
    let water_type = e.get(group, PlaceableWaterGroup::pWaterType);
    e.call(
        NI_POINTER_ASSIGN_FROM,
        &args![
            address_of(property, WaterShaderProperty::spNoiseNormalMap),
            address_of(
                water_type.cast::<TESWaterForm>(),
                TESWaterForm::spNoiseNormalMap
            )
        ],
    );
    let height_map = pointer_in_slot(e, WADING_WATER_HEIGHT_MAP);
    let texture = e
        .call(RENDERED_TEXTURE_GET_TEXTURE, &args![height_map, 0u32])
        .u32();
    assign_slot(
        e,
        address_of(property, WaterShaderProperty::spDisplacementNormalMap),
        texture,
    );
    e.set(property, WaterShaderProperty::bDisplacement, true);
    e.set(property, WaterShaderProperty::fBlendRadius, 1.0);
    let stencil_mask = e.get(group, PlaceableWaterGroup::iStencilBitMask);
    e.set(property, WaterShaderProperty::iStencilMask, stencil_mask);
    e.set(this, TESWaterSystem::fTimeSinceLastRipplePlaced, 0.0);
    shape
}

// Translated from 004e83e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the player, or an actor in the group's `ActorsInWaterList` that
/// the actor virtual at +0x22c (called with 0) does not skip, is within
/// `fWadingWaterQuadSize` of the player on the ground plane (the heights are
/// zeroed before the distance is taken). The word before `group` (`this`) is
/// not read.
pub fn fn_004e83e0(e: &mut Engine, _unused_0: u32, group: Ptr<PlaceableWaterGroup>) -> bool {
    let actors = group.at(PlaceableWaterGroup::ActorsInWaterList).addr();
    any_list_item(e, actors, |e, actor| {
        let player = e.global::<u32>(PLAYER_CHARACTER);
        if player == actor {
            return true;
        }
        if e.vcall(actor, ACTOR_SKIP_VIRTUAL, &args![0u32]).bool() {
            return false;
        }
        e.with_stack(0x24, |e, frame| {
            let frame = frame.addr();
            let player_point = frame;
            let actor_point = frame + 0x0c;
            let difference = frame + 0x18;
            let position = e.vcall(player, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
            copy_words(e, position, player_point, 3);
            let position = e.vcall(actor, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
            copy_words(e, position, actor_point, 3);
            e.mem.set_f32(player_point + 8, 0.0);
            e.mem.set_f32(actor_point + 8, 0.0);
            let result = e
                .call(
                    POINT3_SUBTRACT,
                    &args![player_point, difference, actor_point],
                )
                .u32();
            let distance = e.call(POINT3_LENGTH, &args![result]).f32();
            let limit = setting_float(e, SETTING_WADING_WATER_QUAD_SIZE);
            distance <= limit
        })
    })
}

// Translated from 004e8510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::AddRipple` (Xbox PDB): a ripple at `(x, y)` on the water
/// plane at `height`. Does nothing unless `bUseBulletWaterDisplacements` and
/// `bUseWaterDisplacements` are on and the point is within
/// `fWadingWaterQuadSize` of the player. Every group within
/// `fWaterGroupHeightRange` of `height` that has no wading geometry yet gets
/// one (creating the wading height map first when there is none), at the
/// player's position and the group's height. The offset from the player is
/// mapped to `-0.5..0.5` and, shifted by `size / 2` in both directions, put
/// into the two displacement arrays.
pub fn tes_water_system_add_ripple(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    x: f32,
    y: f32,
    height: f32,
    size: f32,
) {
    if !setting_flag(e, SETTING_USE_BULLET_WATER_DISPLACEMENTS) {
        return;
    }
    if !setting_flag(e, SETTING_USE_WATER_DISPLACEMENTS) {
        return;
    }
    e.with_stack(0x30, |e, frame| {
        let frame = frame.addr();
        let player_point = frame;
        let ripple_point = frame + 0x08;
        let difference = frame + 0x10;
        let first_offset = frame + 0x18;
        let second_offset = frame + 0x20;
        e.call(ADDRESS_OF_THIS, &args![player_point]);
        e.call(ADDRESS_OF_THIS, &args![ripple_point]);
        let player = e.global::<u32>(PLAYER_CHARACTER);
        let position = e.vcall(player, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
        let word = e.mem.u32(position);
        e.mem.set_u32(player_point, word);
        let position = e.vcall(player, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
        let word = e.mem.u32(position + 4);
        e.mem.set_u32(player_point + 4, word);
        e.mem.set_f32(ripple_point, x);
        e.mem.set_f32(ripple_point + 4, y);
        let separation = fn_004e8880(e, player_point, difference, ripple_point);
        let distance = e.call(POINT2_LENGTH, &args![separation]).f32();
        let quad_size = setting_float(e, SETTING_WADING_WATER_QUAD_SIZE);
        // Not within the quad size (an unordered distance counts as outside).
        if !matches!(
            distance.partial_cmp(&quad_size),
            Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
        ) {
            return;
        }

        let group_list = this.at(TESWaterSystem::PlaceableWaterGroupList).addr();
        for_each_list_item(e, group_list, |e, item| {
            let group: Ptr<PlaceableWaterGroup> = Ptr::new(item);
            let range = setting_float(e, SETTING_WATER_GROUP_HEIGHT_RANGE);
            let plane = group.at(PlaceableWaterGroup::ReflectWaterPlane).addr();
            let constant = plane_constant(e, plane);
            if !e.call(FLOATS_NEAR, &args![constant, height, range]).bool() {
                return;
            }
            let geometry_slot = address_of(group, PlaceableWaterGroup::spWadingWaterGeometry);
            if pointer_in_slot(e, geometry_slot) != 0 {
                return;
            }
            if pointer_in_slot(e, WADING_WATER_HEIGHT_MAP) == 0 {
                let texture = e.call(INITIALIZE_GREY_TEXTURE, &args![this]).u32();
                assign_slot(e, WADING_WATER_HEIGHT_MAP, texture);
                e.set_global(WADING_MAP_CREATED_FLAG, 1u8);
            }
            let quad_size = setting_float(e, SETTING_WADING_WATER_QUAD_SIZE);
            let root = fn_004e7ff0(e);
            let geometry = fn_004e8160(e, this, group, root, quad_size);
            assign_slot(e, geometry_slot, geometry);
            e.with_stack(0x0c, |e, spot| {
                let spot = spot.addr();
                let position = e.vcall(player, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
                copy_words(e, position, spot, 3);
                let constant = plane_constant(e, plane);
                e.mem.set_f32(spot + 8, constant);
                let node = pointer_in_slot(e, geometry_slot);
                e.call(NODE_SET_LOCAL_TRANSLATE, &args![node, spot]);
            });
            e.with_stack(0x10, |e, update_data| {
                e.call(
                    UPDATE_DATA_CONSTRUCT,
                    &args![update_data, 0.0f32, 0u32, 0u32],
                );
                let node = pointer_in_slot(e, geometry_slot);
                e.call(NODE_UPDATE, &args![node, update_data]);
            });
        });

        let position = e.vcall(player, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
        let player_x = e.mem.f32(position);
        e.mem.set_f32(ripple_point, float_difference(x, player_x));
        let position = e.vcall(player, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
        let player_y = e.mem.f32(position + 4);
        e.mem
            .set_f32(ripple_point + 4, float_difference(y, player_y));
        let mapped = e.mem.f32(ripple_point);
        let mapped = map_displacement(e, mapped);
        e.mem.set_f32(ripple_point, mapped);
        let mapped = e.mem.f32(ripple_point + 4);
        let mapped = map_displacement(e, mapped);
        e.mem.set_f32(ripple_point + 4, mapped);

        let half = half_of(e, size);
        let offset = e
            .call(NI_POINT2_CONSTRUCT, &args![first_offset, half, half])
            .u32();
        fn_004e88d0(e, ripple_point, offset);
        e.call(
            POINT2_ARRAY_APPEND,
            &args![DISPLACEMENT_POINTS_A, ripple_point],
        );
        let half = half_of(e, size);
        let offset = e
            .call(NI_POINT2_CONSTRUCT, &args![second_offset, half, half])
            .u32();
        fn_004e8910(e, ripple_point, offset);
        e.call(
            POINT2_ARRAY_APPEND,
            &args![DISPLACEMENT_POINTS_B, ripple_point],
        );
    });
}

// Translated from 004e8880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint2` subtraction (no Xbox PDB name): `out = this - other`; returns
/// `out`.
pub fn fn_004e8880(e: &mut Engine, this: u32, out: u32, other: u32) -> u32 {
    let x = float_difference(e.mem.f32(this), e.mem.f32(other));
    let y = float_difference(e.mem.f32(this + 4), e.mem.f32(other + 4));
    e.mem.set_f32(out, x);
    e.mem.set_f32(out + 4, y);
    out
}

// Translated from 004e88d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint2` addition (no Xbox PDB name): `this += other`; returns `this`.
pub fn fn_004e88d0(e: &mut Engine, this: u32, other: u32) -> u32 {
    let x = (e.mem.f32(this) as f64 + e.mem.f32(other) as f64) as f32;
    e.mem.set_f32(this, x);
    let y = (e.mem.f32(this + 4) as f64 + e.mem.f32(other + 4) as f64) as f32;
    e.mem.set_f32(this + 4, y);
    this
}

// Translated from 004e8910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint2` subtraction in place (no Xbox PDB name): `this -= other`;
/// returns `this`.
pub fn fn_004e8910(e: &mut Engine, this: u32, other: u32) -> u32 {
    let x = float_difference(e.mem.f32(this), e.mem.f32(other));
    e.mem.set_f32(this, x);
    let y = float_difference(e.mem.f32(this + 4), e.mem.f32(other + 4));
    e.mem.set_f32(this + 4, y);
    this
}

// Translated from 004e8950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `bUseWaterDisplacements` is on and some group passes
/// `fn_004e83e0` (no Xbox PDB name).
pub fn fn_004e8950(e: &mut Engine, this: Ptr<TESWaterSystem>) -> bool {
    if fn_004e89e0(e) == 0 {
        return false;
    }
    let group_list = this.at(TESWaterSystem::PlaceableWaterGroupList).addr();
    any_list_item(e, group_list, |e, group| {
        fn_004e83e0(e, this.addr(), Ptr::new(group))
    })
}

// Translated from 004e89e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte of the `bUseWaterDisplacements:Water` setting.
pub fn fn_004e89e0(e: &mut Engine) -> u8 {
    let value = e
        .call(SETTING_VALUE, &args![SETTING_USE_WATER_DISPLACEMENTS])
        .u32();
    e.mem.u8(value)
}

// Translated from 004e8a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The wading-water update (no Xbox PDB name; the exception-unwinding frame
/// is not translated): unless the current interior cell has no water, every
/// rendered group whose actor list is not empty and which passes
/// `fn_004e83e0` handles its actors. An actor that is not in the water
/// (`00885520` on its location, its parent cell and 1.0) and passes `004938e0`
/// gets a `WadingWaterData` in the `WadingWaterMap` (created, with the
/// group's wading geometry, when it has none) holding its position and the
/// displacement offsets from the player, mapped to `-0.5..0.5`; any other
/// actor loses its entry.
pub fn fn_004e8a00(e: &mut Engine, this: Ptr<TESWaterSystem>) {
    let tes = e.global::<u32>(TES_POINTER);
    let interior_cell = e.call(TES_GET_INTERIOR_CELL, &args![tes]).u32();
    if interior_cell != 0 && !e.call(CELL_HAS_WATER, &args![interior_cell]).bool() {
        return;
    }
    let group_list = this.at(TESWaterSystem::PlaceableWaterGroupList).addr();
    for_each_list_item(e, group_list, |e, item| {
        let group: Ptr<PlaceableWaterGroup> = Ptr::new(item);
        if !e.get(group, PlaceableWaterGroup::bRenderGroup) {
            return;
        }
        let actors = group.at(PlaceableWaterGroup::ActorsInWaterList).addr();
        if e.call(LIST_IS_EMPTY, &args![actors]).bool() {
            return;
        }
        if !fn_004e83e0(e, this.addr(), group) {
            return;
        }
        for_each_list_item(e, actors, |e, actor| {
            update_wading_actor(e, this, group, actor);
        });
    });
}

/// One actor of `fn_004e8a00`.
fn update_wading_actor(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    group: Ptr<PlaceableWaterGroup>,
    actor: u32,
) {
    let map = address_of(this, TESWaterSystem::WadingWaterMap);
    let cell = e.call(REFERENCE_PARENT_CELL, &args![actor]).u32();
    let location = e.vcall(actor, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
    let in_water = e
        .call(ACTOR_IN_WATER_TEST, &args![actor, location, cell, 1.0f32])
        .bool();
    if in_water || !e.call(ACTOR_STATE_TEST, &args![actor]).bool() {
        e.with_stack(4, |e, out| {
            if e.call(WADING_MAP_GET, &args![map, actor, out]).bool() {
                e.call(WADING_MAP_REMOVE, &args![map, actor]);
            }
        });
        return;
    }

    let geometry_slot = address_of(group, PlaceableWaterGroup::spWadingWaterGeometry);
    if pointer_in_slot(e, geometry_slot) == 0 {
        let quad_size = setting_float(e, SETTING_WADING_WATER_QUAD_SIZE);
        let root = fn_004e7ff0(e);
        let geometry = fn_004e8160(e, this, group, root, quad_size);
        assign_slot(e, geometry_slot, geometry);
    }
    let plane = group.at(PlaceableWaterGroup::ReflectWaterPlane).addr();
    e.with_stack(0x30, |e, frame| {
        let frame = frame.addr();
        let player_position = frame;
        let actor_position = frame + 0x0c;
        let item_slot = frame + 0x18;
        let first_offset = frame + 0x1c;
        let second_offset = frame + 0x24;
        let player = e.global::<u32>(PLAYER_CHARACTER);
        let position = e.vcall(player, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
        copy_words(e, position, player_position, 3);
        let position = e.vcall(actor, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
        copy_words(e, position, actor_position, 3);
        let constant = plane_constant(e, plane);
        e.mem.set_f32(actor_position + 8, constant);
        let found = e.call(WADING_MAP_GET, &args![map, actor, item_slot]).bool();
        let mut item = e.mem.u32(item_slot);
        if !found {
            let memory = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
            item = if memory == 0 {
                0
            } else {
                fn_004e8e40(e, Ptr::new(memory)).addr()
            };
            copy_words(e, actor_position, item + 0x10, 3);
            e.call(MAP_SET_AT, &args![map, actor, item]);
        }
        e.call(ADDRESS_OF_THIS, &args![first_offset]);
        e.call(ADDRESS_OF_THIS, &args![second_offset]);
        let data: Ptr<WadingWaterData> = Ptr::new(item);
        let player_x = e.mem.f32(player_position);
        let player_y = e.mem.f32(player_position + 4);
        let first_x = float_difference(e.mem.f32(actor_position), player_x);
        let first_y = float_difference(e.mem.f32(actor_position + 4), player_y);
        let last_x = e.get(data, WadingWaterData::fLastPositionX);
        let last_y = e.get(data, WadingWaterData::fLastPositionY);
        let second_x = float_difference(last_x, player_x);
        let second_y = float_difference(last_y, player_y);
        copy_words(e, actor_position, item + 0x10, 3);
        let mapped = map_displacement(e, first_x);
        e.set(data, WadingWaterData::fDisplaceOffsetX, mapped);
        let mapped = map_displacement(e, first_y);
        e.set(data, WadingWaterData::fDisplaceOffsetY, mapped);
        let mapped = map_displacement(e, second_x);
        e.set(data, WadingWaterData::fLastDisplaceOffsetX, mapped);
        let mapped = map_displacement(e, second_y);
        e.set(data, WadingWaterData::fLastDisplaceOffsetY, mapped);
    });
}

// Translated from 004e8e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `WadingWaterData::WadingWaterData` (Xbox PDB class, no name in the
/// engine map): runs the `NiPoint2` / `NiPoint3` default constructors on
/// the members, sets both `NiPoint2` members to the zero point at
/// `0x011f4980` and `fLastPosition` to the zero point at `0x011f426c`.
pub fn fn_004e8e40(e: &mut Engine, this: Ptr<WadingWaterData>) -> Ptr<WadingWaterData> {
    let address = this.addr();
    e.call(ADDRESS_OF_THIS, &args![address]);
    e.call(ADDRESS_OF_THIS, &args![address + 8]);
    e.call(ADDRESS_OF_THIS, &args![address + 0x10]);
    copy_words(e, ZERO_POINT2, address, 2);
    copy_words(e, ZERO_POINT2, address + 8, 2);
    copy_words(e, ZERO_POINT3, address + 0x10, 3);
    this
}

// Translated from 004e8ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::UpdateWaterDisplacementSimulation` (Xbox PDB). While
/// `fTimeSinceLastRipplePlaced` is below 10.0 it renders the wading height
/// map: resets the render target when the renderer is outside a frame
/// (`fn_004e9510`), creates the grey height map when there is none, stops the
/// offscreen render into render-target mode 6, renders the wading geometry
/// (`fn_004e9550`) when the byte at `0x011ad832` asks for it, and moves each
/// group's wading geometry to the player (snapped to a 4.0 grid when the
/// player moved more than 4.0 from the geometry of the first group that has
/// one), refreshing the stencil mask and textures of its shader property. It
/// then publishes the displacement shader constants, fills the two
/// displacement arrays from the `WadingWaterMap`, advances the timer (or
/// resets it when no displacement was added) and renders the displacement
/// image-space effect. Otherwise it detaches the wading geometry of every
/// group and gives the height map back.
pub fn tes_water_system_update_water_displacement_simulation(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
) {
    let elapsed = e.get(this, TESWaterSystem::fTimeSinceLastRipplePlaced);
    let limit = e.global::<f64>(TEN);
    let group_list = this.at(TESWaterSystem::PlaceableWaterGroupList).addr();
    // Not below 10.0 (an unordered timer counts as over, as the game's compare does).
    if !matches!(
        (elapsed as f64).partial_cmp(&limit),
        Some(std::cmp::Ordering::Less)
    ) {
        // The simulation is over: detach the geometry and give the map back.
        for_each_list_item(e, group_list, |e, item| {
            let group: Ptr<PlaceableWaterGroup> = Ptr::new(item);
            let geometry_slot = address_of(group, PlaceableWaterGroup::spWadingWaterGeometry);
            if pointer_in_slot(e, geometry_slot) == 0 {
                return;
            }
            let root = fn_004e7ff0(e);
            let geometry = pointer_in_slot(e, geometry_slot);
            e.vcall(root, NODE_DETACH_CHILD, &args![geometry]);
            assign_slot(e, geometry_slot, 0);
        });
        if return_rendered_texture(e, WADING_WATER_HEIGHT_MAP) {
            assign_slot(e, WADING_WATER_HEIGHT_MAP, 0);
        }
        e.set_global(DISPLACEMENT_ACTIVE_FLAG, 0u8);
        return;
    }

    if !fn_004e9510(e) {
        e.call(RENDER_TARGET_RESET, &[]);
    }
    if pointer_in_slot(e, WADING_WATER_HEIGHT_MAP) == 0 {
        let texture = e.call(INITIALIZE_GREY_TEXTURE, &args![this]).u32();
        assign_slot(e, WADING_WATER_HEIGHT_MAP, texture);
        e.set_global(WADING_MAP_CREATED_FLAG, 1u8);
    }
    let height_map = pointer_in_slot(e, WADING_WATER_HEIGHT_MAP);
    let stopped = e.call(RENDERED_TEXTURE_STOP, &args![height_map]).u32();
    e.call(RENDER_TARGET_SET, &args![6u32, stopped]);
    if e.global::<u8>(DISPLACEMENT_RENDER_FLAG) != 0 {
        fn_004e9550(e, this);
    }
    if !fn_004e9510(e) {
        e.call(RENDER_TARGET_RESET, &[]);
    }

    // The first group with wading geometry (or the last group). The game
    // reads the item of the first position before it tests the position, so an
    // empty list reads address 8.
    let mut position = pointer_in_slot(e, group_list);
    let slot = e.call(LIST_ITEM_SLOT, &args![group_list, position]).u32();
    let mut group_address = e.mem.u32(slot);
    while position != 0
        && pointer_in_slot(
            e,
            group_address + PlaceableWaterGroup::spWadingWaterGeometry.off,
        ) == 0
    {
        let next = e
            .call(LIST_NEXT_POSITION, &args![group_list, position])
            .u32();
        let slot = e.call(LIST_ITEM_SLOT, &args![group_list, position]).u32();
        group_address = e.mem.u32(slot);
        position = next;
    }

    let player = e.global::<u32>(PLAYER_CHARACTER);
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        let player_point = frame;
        let geometry_point = frame + 0x0c;
        let difference = frame + 0x18;
        let update_data = frame + 0x20;
        let position = e.vcall(player, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
        copy_words(e, position, player_point, 3);
        e.call(ADDRESS_OF_THIS, &args![geometry_point]);
        let first_geometry_slot = group_address + PlaceableWaterGroup::spWadingWaterGeometry.off;
        if pointer_in_slot(e, first_geometry_slot) != 0 {
            let node = pointer_in_slot(e, first_geometry_slot);
            let translate = e.call(NODE_LOCAL_TRANSLATE_ADDRESS, &args![node]).u32();
            copy_words(e, translate, geometry_point, 3);
        } else {
            copy_words(e, player_point, geometry_point, 3);
        }
        e.call(ADDRESS_OF_THIS, &args![difference]);
        let difference_x = float_difference(e.mem.f32(player_point), e.mem.f32(geometry_point));
        let difference_y =
            float_difference(e.mem.f32(player_point + 4), e.mem.f32(geometry_point + 4));
        e.mem.set_f32(difference, difference_x);
        e.mem.set_f32(difference + 4, difference_y);
        let mut moved = false;
        let grid = e.global::<f64>(FOUR);
        let far_x = e.call(FLOAT_ABS, &args![difference_x]).f32();
        let far_y = if far_x as f64 > grid {
            0.0
        } else {
            e.call(FLOAT_ABS, &args![difference_y]).f32()
        };
        if far_x as f64 > grid || far_y as f64 > grid {
            moved = true;
            for offset in [0, 4] {
                let coordinate = e.mem.f32(player_point + offset);
                let scaled = (coordinate as f64 / grid) as f32;
                let floored = e.call(FLOAT_FLOOR, &args![scaled]).f32();
                e.mem
                    .set_f32(player_point + offset, (floored as f64 * grid) as f32);
            }
        }

        for_each_list_item(e, group_list, |e, item| {
            let group: Ptr<PlaceableWaterGroup> = Ptr::new(item);
            let geometry_slot = address_of(group, PlaceableWaterGroup::spWadingWaterGeometry);
            if pointer_in_slot(e, geometry_slot) == 0 {
                return;
            }
            let plane = group.at(PlaceableWaterGroup::ReflectWaterPlane).addr();
            let constant = plane_constant(e, plane);
            e.mem.set_f32(player_point + 8, constant);
            if moved {
                let node = pointer_in_slot(e, geometry_slot);
                e.call(NODE_SET_LOCAL_TRANSLATE, &args![node, player_point]);
                e.call(
                    UPDATE_DATA_CONSTRUCT,
                    &args![update_data, 0.0f32, 0u32, 0u32],
                );
                let node = pointer_in_slot(e, geometry_slot);
                e.call(NODE_UPDATE, &args![node, update_data]);
            }
            let node = pointer_in_slot(e, geometry_slot);
            let property = water_shader_property(e, node);
            let stencil_mask = e.get(group, PlaceableWaterGroup::iStencilBitMask);
            e.set(property, WaterShaderProperty::iStencilMask, stencil_mask);
            e.call(
                NI_POINTER_ASSIGN_FROM,
                &args![
                    address_of(property, WaterShaderProperty::spDepthMap),
                    DEPTH_MAP
                ],
            );
            let water_type = e.get(group, PlaceableWaterGroup::pWaterType);
            e.call(
                NI_POINTER_ASSIGN_FROM,
                &args![
                    address_of(property, WaterShaderProperty::spNoiseNormalMap),
                    address_of(
                        water_type.cast::<TESWaterForm>(),
                        TESWaterForm::spNoiseNormalMap
                    )
                ],
            );
            let height_map = pointer_in_slot(e, WADING_WATER_HEIGHT_MAP);
            let texture = e
                .call(RENDERED_TEXTURE_GET_TEXTURE, &args![height_map, 0u32])
                .u32();
            assign_slot(
                e,
                address_of(property, WaterShaderProperty::spDisplacementNormalMap),
                texture,
            );
        });

        if moved {
            for (offset, shift) in [(0, DISPLACEMENT_SHIFT_X), (4, DISPLACEMENT_SHIFT_Y)] {
                let moved_by = float_difference(
                    e.mem.f32(player_point + offset),
                    e.mem.f32(geometry_point + offset),
                );
                let quad_size = setting_float(e, SETTING_WADING_WATER_QUAD_SIZE);
                let half = half_of(e, quad_size);
                let quad_size = setting_float(e, SETTING_WADING_WATER_QUAD_SIZE);
                let negative_half = -half_of(e, quad_size);
                let low = e.global::<f32>(MAP_LOW);
                let high = e.global::<f32>(MAP_HIGH);
                // The y shift is mapped onto the reversed range.
                let (first, second) = if offset == 0 {
                    (low, high)
                } else {
                    (high, low)
                };
                let shifted = e
                    .call(
                        LINEAR_MAP,
                        &args![first, second, negative_half, half, moved_by],
                    )
                    .f32();
                e.set_global(shift, shifted);
            }
        } else {
            e.set_global(DISPLACEMENT_SHIFT_X, 0.0f32);
            e.set_global(DISPLACEMENT_SHIFT_Y, 0.0f32);
        }

        if e.call(RENDER_SETTING_ENTRY, &args![0u32]).i32() >= 2 {
            e.set_global(DISPLACEMENT_PASS_FLAG, 1u8);
            for (i, source) in DISPLACEMENT_CONSTANT_SOURCES.into_iter().enumerate() {
                let value = e.global::<f32>(source);
                e.set_global(DISPLACEMENT_CONSTANTS + 4 * i as u32, value);
            }
            let map = address_of(this, TESWaterSystem::WadingWaterMap);
            e.with_stack(0x0c, |e, locals| {
                let locals = locals.addr();
                let first = e.call(ZONE_MAP_FIRST, &args![map]).u32();
                e.mem.set_u32(locals, first);
                while e.mem.u32(locals) != 0 {
                    e.mem.set_u32(locals + 4, 0);
                    e.mem.set_u32(locals + 8, 0);
                    e.call(
                        ZONE_MAP_GET_NEXT,
                        &args![map, locals, locals + 4, locals + 8],
                    );
                    let data = e.mem.u32(locals + 8);
                    e.call(POINT2_ARRAY_APPEND, &args![DISPLACEMENT_POINTS_A, data]);
                    e.call(POINT2_ARRAY_APPEND, &args![DISPLACEMENT_POINTS_B, data + 8]);
                }
            });
            if e.call(LIST_COUNT, &args![DISPLACEMENT_POINTS_A]).u32() == 0 {
                e.set(this, TESWaterSystem::fTimeSinceLastRipplePlaced, 0.0);
            } else {
                let step = e.call(FLOAT_TABLE_ENTRY, &args![1u32]).f32();
                let timer = e.get(this, TESWaterSystem::fTimeSinceLastRipplePlaced);
                e.set(
                    this,
                    TESWaterSystem::fTimeSinceLastRipplePlaced,
                    (step as f64 + timer as f64) as f32,
                );
            }
            let first = pointer_in_slot(e, WADING_WATER_HEIGHT_MAP);
            let second = pointer_in_slot(e, WADING_WATER_HEIGHT_MAP);
            let renderer = e.call(RENDERER, &[]).u32();
            let manager = fn_004e3270(e);
            e.call(
                IMAGE_SPACE_RENDER_DISPLACEMENT,
                &args![manager, 0x20u32, renderer, second, first, 0u32, 1u32],
            );
            e.set_global(DISPLACEMENT_PASS_FLAG, 0u8);
        }
    });
}

// Translated from 004e9510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the word at +0x200 of the object `0x011f4748` holds is zero
/// (`fn_004e9530` of it).
pub fn fn_004e9510(e: &mut Engine) -> bool {
    let object = e.call(RENDER_GLOBAL_OBJECT, &[]).u32();
    fn_004e9530(e, object) == 0
}

// Translated from 004e9530 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x200`.
pub fn fn_004e9530(e: &mut Engine, this: u32) -> u32 {
    e.mem.u32(this + 0x200)
}

// Translated from 004e9550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Renders the wading geometry (no Xbox PDB name; the exception-unwinding
/// frame is not translated): builds an orthographic `NiCamera` (frustum
/// `fWadingWaterQuadSize` wide, near plane from the exe, far plane the
/// distance between the lowest and highest group plane plus 100.0) 50.0 above
/// the highest group plane at the player's x and y, looking at the player's x
/// and y on that plane, makes it the render object's camera data and, for
/// every water reference of every group whose cell kind is 6 and whose
/// geometry's water shader property has owner type `0xd`, forces the property's
/// stencil mask to 1 and draws the reference's render pass immediately when it
/// is in range of the camera.
///
/// A local `NiPoint3` the game fills from `0x011f426c` is never read and is
/// left out.
pub fn fn_004e9550(e: &mut Engine, this: Ptr<TESWaterSystem>) {
    e.with_stack(0x150 + CULLING_PROCESS_SIZE + 0x20, |e, frame| {
        let frame = frame.addr();
        let camera = frame;
        let update_data = frame + 0x120;
        let frustum = frame + 0x130;
        let culling = frame + 0x150;
        let eye_point = culling + CULLING_PROCESS_SIZE;
        let target_point = eye_point + 0x10;
        e.call(CAMERA_CONSTRUCT, &args![camera]);

        // The highest and lowest group plane.
        let highest = e.global::<f32>(HIGHEST_FLOAT);
        let highest_as_double = e.global::<f64>(HIGHEST_FLOAT_AS_DOUBLE);
        let mut top = highest;
        let mut bottom = highest;
        let group_list = this.at(TESWaterSystem::PlaceableWaterGroupList).addr();
        for_each_list_item(e, group_list, |e, item| {
            let group: Ptr<PlaceableWaterGroup> = Ptr::new(item);
            let plane = group.at(PlaceableWaterGroup::ReflectWaterPlane).addr();
            let constant = plane_constant(e, plane);
            if top < constant || top as f64 == highest_as_double {
                top = constant;
            }
            if constant < bottom || bottom as f64 == highest_as_double {
                bottom = constant;
            }
        });

        e.call(FRUSTUM_CONSTRUCT, &args![frustum, 0u32]);
        e.mem.set_u8(frustum + 0x18, 1);
        let near_plane = e.global::<f32>(WADING_NEAR_PLANE);
        e.mem.set_f32(frustum + 0x10, near_plane);
        let quad_size = setting_float(e, SETTING_WADING_WATER_QUAD_SIZE);
        let left = -half_of(e, quad_size);
        e.mem.set_f32(frustum, left);
        let quad_size = setting_float(e, SETTING_WADING_WATER_QUAD_SIZE);
        let right = half_of(e, quad_size);
        e.mem.set_f32(frustum + 4, right);
        let quad_size = setting_float(e, SETTING_WADING_WATER_QUAD_SIZE);
        let upper = half_of(e, quad_size);
        e.mem.set_f32(frustum + 8, upper);
        let quad_size = setting_float(e, SETTING_WADING_WATER_QUAD_SIZE);
        let lower = -half_of(e, quad_size);
        e.mem.set_f32(frustum + 0x0c, lower);
        let span = float_difference(bottom, top);
        let span = e.call(FLOAT_ABS, &args![span]).f32();
        let hundred = e.global::<f64>(ONE_HUNDRED);
        e.mem
            .set_f32(frustum + 0x14, (span as f64 + hundred) as f32);

        // The camera sits above the highest plane at the player's x and y.
        let fifty = e.global::<f64>(FIFTY);
        let eye_height = (top as f64 + fifty) as f32;
        let player = e.global::<u32>(PLAYER_CHARACTER);
        let position = e.vcall(player, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
        let player_y = e.mem.f32(position + 4);
        let position = e.vcall(player, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
        let player_x = e.mem.f32(position);
        let eye = e
            .call(
                NI_POINT3_CONSTRUCT,
                &args![eye_point, player_x, player_y, eye_height],
            )
            .u32();
        e.call(NODE_SET_LOCAL_TRANSLATE, &args![camera, eye]);
        e.call(CAMERA_SET_VIEW_FRUSTUM, &args![camera, frustum]);
        e.call(
            UPDATE_DATA_CONSTRUCT,
            &args![update_data, 0.0f32, 0u32, 0u32],
        );
        e.call(NODE_UPDATE, &args![camera, update_data]);
        let position = e.vcall(player, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
        let player_y = e.mem.f32(position + 4);
        let position = e.vcall(player, REFERENCE_GET_LOCATION_VIRTUAL, &[]).u32();
        let player_x = e.mem.f32(position);
        e.call(
            NI_POINT3_CONSTRUCT,
            &args![target_point, player_x, player_y, top],
        );
        e.call(
            CAMERA_LOOK_AT_WORLD_POINT,
            &args![camera, target_point, UP_VECTOR],
        );
        let frustum_address = e.call(CAMERA_FRUSTUM_ADDRESS, &args![camera]).u32();
        let object = e.call(RENDER_GLOBAL_OBJECT, &[]).u32();
        e.call(
            RENDER_OBJECT_SET_CAMERA_DATA,
            &args![object, frustum_address],
        );
        e.set_global(WADING_RENDER_ACTIVE_FLAG, 1u8);
        e.call(CULLING_PROCESS_CONSTRUCT, &args![culling, 0u32]);
        e.call(CULLING_PROCESS_SET_CAMERA, &args![culling, camera]);
        let object = e.call(RENDER_GLOBAL_OBJECT, &[]).u32();
        fn_004e9bb0(e, object, camera);
        let planes = e.call(CAMERA_PLANES_ADDRESS, &args![camera]).u32();
        e.call(CULLING_PROCESS_SET_PLANES, &args![culling, planes]);

        for_each_list_item(e, group_list, |e, item| {
            let group: Ptr<PlaceableWaterGroup> = Ptr::new(item);
            let references = group.at(PlaceableWaterGroup::PlaceableWaterList).addr();
            for_each_list_item(e, references, |e, reference| {
                draw_wading_reference(e, this, camera, reference);
            });
        });

        e.call(CULLING_PROCESS_SET_CAMERA, &args![culling, 0u32]);
        fn_004e9ce0(e, 0, 0);
        e.set_global(WADING_RENDER_ACTIVE_FLAG, 0u8);
        e.call(CULLING_PROCESS_DESTRUCT, &args![culling]);
        e.call(CAMERA_DESTRUCT, &args![camera]);
    });
}

/// One water reference of `fn_004e9550`.
fn draw_wading_reference(e: &mut Engine, this: Ptr<TESWaterSystem>, camera: u32, reference: u32) {
    if reference == 0 {
        return;
    }
    if e.call(REFERENCE_PARENT_CELL, &args![reference]).u32() == 0 {
        return;
    }
    let cell = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32();
    if !e.call(CELL_BYTE_IS_SIX, &args![cell]).bool() {
        return;
    }
    let geometry = fn_004e8030(e, this, reference);
    if geometry == 0 {
        return;
    }
    let owner = e.call(NODE_OWNER, &args![geometry]).u32();
    if owner == 0 || e.call(OWNER_TYPE, &args![owner]).u32() != WATER_OWNER_TYPE {
        return;
    }
    let property = water_shader_property(e, geometry);
    let saved_mask = e.get(property, WaterShaderProperty::iStencilMask);
    e.set(property, WaterShaderProperty::iStencilMask, 1);
    if fn_004e62e0(e, 0, reference, camera)
        && !property.is_null()
        && e.call(PROPERTY_RENDER_PASS, &args![property, geometry])
            .u32()
            != 0
    {
        let pass = e
            .call(PROPERTY_RENDER_PASS, &args![property, geometry])
            .u32();
        let count = e.mem.u16(pass + 4) as u32;
        e.call(
            RENDER_PASS_IMMEDIATELY,
            &args![pass, count, 0u32, 0u32, 0u32],
        );
    }
    e.set(property, WaterShaderProperty::iStencilMask, saved_mask);
}

// Translated from 004e9bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands the camera to the render object `this` (no Xbox PDB name): its
/// world translate, the three columns of its rotation and its two data
/// blocks (`004a0d10` and `0045bbe0`) go to `fn_004e9c90`.
pub fn fn_004e9bb0(e: &mut Engine, this: u32, camera: u32) {
    e.with_stack(0x24, |e, columns| {
        let columns = columns.addr();
        let frustum = e.call(CAMERA_FRUSTUM_ADDRESS, &args![camera]).u32();
        let planes = e.call(CAMERA_PLANES_ADDRESS, &args![camera]).u32();
        let column_two = fn_004e9c50(e, camera, columns);
        let column_one = fn_004e9c10(e, camera, columns + 0x0c);
        let column_zero = e
            .call(CAMERA_COLUMN_ZERO, &args![camera, columns + 0x18])
            .u32();
        let translate = e.call(NODE_WORLD_TRANSLATE, &args![camera]).u32();
        fn_004e9c90(
            e,
            this,
            translate,
            column_zero,
            column_one,
            column_two,
            planes,
            frustum,
        );
    });
}

// Translated from 004e9c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Column 1 of the camera's rotation matrix (`this + 0x68`) into `out`
/// (`NiMatrix3::GetCol`-style `00439f50`); returns `out`.
pub fn fn_004e9c10(e: &mut Engine, this: u32, out: u32) -> u32 {
    rotation_column(e, this, 1, out)
}

// Translated from 004e9c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Column 2 of the camera's rotation matrix (`this + 0x68`) into `out`;
/// returns `out`.
pub fn fn_004e9c50(e: &mut Engine, this: u32, out: u32) -> u32 {
    rotation_column(e, this, 2, out)
}

/// The body of `fn_004e9c10` and `fn_004e9c50`: a local `NiPoint3` is
/// constructed, filled by `00439f50(column, &local)` on `this + 0x68` and
/// copied to `out`.
fn rotation_column(e: &mut Engine, this: u32, column: u32, out: u32) -> u32 {
    e.with_stack(0x0c, |e, local| {
        let local = local.addr();
        e.call(ADDRESS_OF_THIS, &args![local]);
        e.call(MATRIX_COLUMN, &args![this + 0x68, column, local]);
        copy_words(e, local, out, 3);
    });
    out
}

// Translated from 004e9c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes the camera data on (no Xbox PDB name): when the render object `this`
/// answers `004a0e10("SetCameraData", 1)` it calls its virtual at +0x18c with
/// the six words.
#[allow(clippy::too_many_arguments)]
pub fn fn_004e9c90(
    e: &mut Engine,
    this: u32,
    translate: u32,
    column_zero: u32,
    column_one: u32,
    column_two: u32,
    planes: u32,
    frustum: u32,
) {
    if !e
        .call(
            RENDER_OBJECT_CHECK,
            &args![this, SET_CAMERA_DATA_NAME, 1u32],
        )
        .bool()
    {
        return;
    }
    e.vcall(
        this,
        RENDER_OBJECT_SET_CAMERA_DATA_VIRTUAL,
        &args![
            translate,
            column_zero,
            column_one,
            column_two,
            planes,
            frustum
        ],
    );
}

// Translated from 004e9ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`enabled` non-zero) or clears bit `index % 16` of the 16-bit word
/// at `0x011f941c` (cdecl).
pub fn fn_004e9ce0(e: &mut Engine, index: u32, enabled: u8) {
    let bit = 1u32 << (index % 16);
    let flags = e.global::<u16>(BIT_FLAGS_011F941C) as u32;
    let flags = if enabled != 0 {
        flags | bit
    } else {
        flags & !bit
    };
    e.set_global(BIT_FLAGS_011F941C, flags as u16);
}

// Translated from 004e9d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets up the reflection of one group (no Xbox PDB name; the
/// exception-unwinding frame is not translated). Within an allocation scope
/// of `TESWater.cpp` line `0xefb`, when `bUseWaterShader` is on and there is a
/// `viewer` camera: makes the group's reflection camera by reflecting the
/// viewer about the group's plane, makes the group's reflection accumulator
/// (when it has none) and prepares it, sets its silhouette colour when the
/// group has silhouette reflections, and fills the group's static and dynamic
/// reflective object lists (the sky, the land LOD children of the loaded
/// grid, the terrain objects, the objects the group's water references
/// reflect, the explosions) as the form flags of the water references and the
/// INI settings ask. `low_detail` (the caller passes 0 for high detail
/// reflections or in an interior) keeps the land, reflected references and
/// the object lists of the whole reference list out. The task is then handed
/// to the multithreaded renderer.
pub fn fn_004e9d40(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    viewer: u32,
    group: Ptr<PlaceableWaterGroup>,
    low_detail: bool,
) {
    e.with_stack(4, |e, scope| {
        e.call(
            ALLOCATION_SCOPE_CONSTRUCT,
            &args![
                scope,
                0x1du32,
                1u32,
                TESWATER_SOURCE_PATH,
                REFLECTION_SCOPE_LINE_GROUP
            ],
        );
        setup_group_reflections(e, this, viewer, group, low_detail);
        e.call(ALLOCATION_SCOPE_DESTRUCT, &args![scope]);
    });
}

/// The body of `fn_004e9d40`.
fn setup_group_reflections(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    viewer: u32,
    group: Ptr<PlaceableWaterGroup>,
    low_detail: bool,
) {
    if !e.call(WATER_SHADER_ENABLED, &[]).bool() || viewer == 0 {
        return;
    }
    let camera_slot = address_of(group, PlaceableWaterGroup::spReflectionCamera);
    let sorter_slot = address_of(group, PlaceableWaterGroup::spGroupReflectionSorter);
    assign_new_camera(e, camera_slot);
    let camera = pointer_in_slot(e, camera_slot);
    let plane = group.at(PlaceableWaterGroup::ReflectWaterPlane).addr();
    e.call(REFLECT_CAMERA_ABOUT_PLANE, &args![viewer, plane, camera]);
    if pointer_in_slot(e, sorter_slot) == 0 {
        assign_new_accumulator(e, sorter_slot);
    }
    prepare_accumulator(e, sorter_slot, camera_slot);
    if e.get(group, PlaceableWaterGroup::bRenderSilhouetteReflections) {
        set_silhouette_color(e, sorter_slot);
    }

    let static_objects = group
        .at(PlaceableWaterGroup::StaticReflectiveObjects)
        .addr();
    let dynamic_objects = group
        .at(PlaceableWaterGroup::DynamicReflectiveObjects)
        .addr();
    e.call(LIST_REMOVE_ALL, &args![static_objects]);
    e.call(LIST_REMOVE_ALL, &args![dynamic_objects]);
    let references = group.at(PlaceableWaterGroup::PlaceableWaterList).addr();
    let reflected_refs_map = address_of(this, TESWaterSystem::ReflectionRefMap);
    let mut sky_added = false;
    let mut land_added = false;
    let mut terrain_a_added = false;
    let mut terrain_b_added = false;
    let mut terrain_c_added = false;
    e.with_stack(4, |e, position_slot| {
        let position_slot = position_slot.addr();
        let first = pointer_in_slot(e, references);
        e.mem.set_u32(position_slot, first);
        while e.mem.u32(position_slot) != 0 {
            let item = e
                .call(LIST_NEXT_ITEM, &args![references, position_slot])
                .u32();
            let reference = e.mem.u32(item);
            let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
            if reference == 0 {
                continue;
            }
            // Which references count: a reference in a cell of kind 6, or one
            // whose base form has the flag `0x08000000` outside interiors.
            let cell = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32();
            let cell_kind_six = cell != 0 && {
                let cell = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32();
                e.call(CELL_BYTE_IS_SIX, &args![cell]).bool()
            };
            if !cell_kind_six && (base_form == 0 || !fn_004e32c0(e, base_form) || in_interior(e)) {
                continue;
            }
            let geometry = fn_004e8030(e, this, reference);
            let mut property = 0;
            if geometry != 0 {
                let candidate = e.call(NODE_GET_PROPERTY, &args![geometry, 3u32]).u32();
                if e.call(IS_KIND_OF, &args![WATER_SHADER_PROPERTY_RTTI, candidate])
                    .bool()
                {
                    property = candidate;
                }
            }
            if property == 0 {
                continue;
            }
            e.call(
                NI_POINTER_ASSIGN_FROM,
                &args![
                    property + WaterShaderProperty::spReflectionMap.off,
                    address_of(group, PlaceableWaterGroup::spGroupReflectionMap)
                ],
            );

            if base_form != 0 && fn_004ea930(e, base_form) && !sky_added {
                sky_added = true;
                let tes = e.global::<u32>(TES_POINTER);
                let sky = e.call(TES_GET_SKY, &args![tes]).u32();
                let sky_objects = e.call(SKY_OBJECT_POINTER, &args![sky]).u32();
                list_add_head(e, static_objects, sky_objects);
            }

            if !low_detail
                && (setting_flag(e, SETTING_FORCE_HIGH_DETAIL_LAND_REFLECTIONS)
                    || (base_form != 0
                        && fn_004ea8b0(e, base_form)
                        && !land_added
                        && !in_interior(e)))
            {
                land_added = true;
                let mut x = 0;
                while x < setting_int(e, SETTING_GRID_SIZE) {
                    let mut y = 0;
                    while y < setting_int(e, SETTING_GRID_SIZE) {
                        let tes = e.global::<u32>(TES_POINTER);
                        let slot = e.call(TES_GRID_CELL_SLOT, &args![tes, x, y]).u32();
                        let grid_cell = e.mem.u32(slot);
                        if grid_cell != 0 {
                            for index in 0..4u32 {
                                let grid_cell = e.mem.u32(slot);
                                let child = e.call(CELL_NODE_CHILD, &args![grid_cell, index]).u32();
                                list_add_head(e, static_objects, child);
                            }
                        }
                        y += 1;
                    }
                    x += 1;
                }
            }

            if current_world_space(e) != 0 {
                let world_space = current_world_space(e);
                let terrain = e
                    .call(WORLD_SPACE_GET_TERRAIN_MANAGER, &args![world_space])
                    .u32();
                if terrain != 0 {
                    let terrain_ready = e.call(TERRAIN_READY, &[]).bool();
                    if !in_interior(e) && terrain_ready {
                        e.call(TERRAIN_STEP_FIRST, &[]);
                        if base_form != 0 && fn_004ea8d0(e, base_form) && !terrain_a_added {
                            terrain_a_added = true;
                            add_terrain_object(e, terrain, fn_004ea9a0, static_objects);
                        }
                        e.call(TERRAIN_STEP_SECOND, &[]);
                        if base_form != 0 && fn_004ea8f0(e, base_form) && !terrain_b_added {
                            terrain_b_added = true;
                            add_terrain_object(e, terrain, fn_004ea9c0, static_objects);
                        }
                        if base_form != 0 && fn_004ea910(e, base_form) && !terrain_c_added {
                            terrain_c_added = true;
                            add_terrain_object(e, terrain, fn_004ea9e0, static_objects);
                        }
                    }
                }
            }

            if !low_detail && fn_004e62e0(e, this.addr(), reference, viewer) {
                let extra_data = e.call(REFERENCE_EXTRA_DATA_LIST, &args![reference]).u32();
                let mut reflected = e
                    .call(EXTRA_DATA_LIST_GET_REFLECTED_REFS, &args![extra_data])
                    .u32();
                while reflected != 0 && !e.call(WORDS_ARE_ZERO, &args![reflected]).bool() {
                    let entry = e.call(ADDRESS_OF_THIS, &args![reflected]).u32();
                    let data = e.mem.u32(entry);
                    if e.mem.u32(data + 4) & 1 != 0 {
                        let entry = e.call(ADDRESS_OF_THIS, &args![reflected]).u32();
                        let data = e.mem.u32(entry);
                        let object = e.mem.u32(data);
                        add_reflected_reference(
                            e,
                            reflected_refs_map,
                            static_objects,
                            object,
                            true,
                        );
                    }
                    reflected = e.call(NODE_NEXT, &args![reflected]).u32();
                }
            }

            if !in_interior(e)
                && !low_detail
                && base_form != 0
                && e.call(FORM_FLAG_TEST_40000000, &args![base_form]).bool()
            {
                let mut node = fn_004e3260(e);
                while node != 0 && !e.call(WORDS_ARE_ZERO, &args![node]).bool() {
                    let entry = e.call(ADDRESS_OF_THIS, &args![node]).u32();
                    let object = e.mem.u32(entry);
                    if object != 0 {
                        add_reflected_reference(
                            e,
                            reflected_refs_map,
                            static_objects,
                            object,
                            false,
                        );
                    }
                    node = e.call(NODE_NEXT, &args![node]).u32();
                }
            }
        }
    });

    if setting_flag(e, SETTING_REFLECT_EXPLOSIONS) {
        // Only the first explosion is looked at (the game's loop has no
        // second iteration).
        let position = pointer_in_slot(e, EXPLOSIONS_LIST);
        let _next = e
            .call(LIST_NEXT_POSITION, &args![EXPLOSIONS_LIST, position])
            .u32();
        if position != 0 {
            let slot = e
                .call(LIST_ITEM_SLOT, &args![EXPLOSIONS_LIST, position])
                .u32();
            let explosion = e.mem.u32(slot);
            if reference_node(e, explosion) != 0 {
                let found = e.with_stack(4, |e, out| {
                    e.mem.set_u32(out.addr(), explosion);
                    e.call(WADING_MAP_GET, &args![reflected_refs_map, explosion, out])
                        .bool()
                });
                if !found {
                    let node = reference_node(e, explosion);
                    list_add_head(e, dynamic_objects, node);
                    e.call(MAP_SET_AT, &args![reflected_refs_map, explosion, explosion]);
                }
            }
        }
    }

    let stage = e.get(group, PlaceableWaterGroup::iReflectionThreadStage);
    add_accumulator_task(
        e,
        camera_slot,
        static_objects,
        dynamic_objects,
        sorter_slot,
        stage.wrapping_add(1),
    );
}

/// Adds the 3D object of a terrain accessor's result (`fn_004ea9a0`,
/// `fn_004ea9c0` or `fn_004ea9e0` on the terrain manager) to `list`, when
/// the current cell is not an interior, there is a world space and the
/// result is not null.
fn add_terrain_object(
    e: &mut Engine,
    terrain: u32,
    accessor: fn(&mut Engine, u32) -> u32,
    list: u32,
) {
    if in_interior(e) || current_world_space(e) == 0 {
        return;
    }
    if accessor(e, terrain) == 0 {
        return;
    }
    let object = accessor(e, terrain);
    list_add_head(e, list, object);
}

/// A reflected reference of `fn_004e9d40`: when it has a 3D object and
/// `ReflectionRefMap` does not have it yet, its 3D object goes to `list` and
/// the reference into the map. Outside interiors the references `00564e60`
/// excludes are left out; `interior_kept` makes the game's first group of
/// callers keep every reference in an interior (it tests the interior first
/// and again before the exclusion test), the second group tests the exclusion
/// only.
fn add_reflected_reference(e: &mut Engine, map: u32, list: u32, object: u32, interior_kept: bool) {
    if reference_node(e, object) == 0 {
        return;
    }
    let found = e.with_stack(4, |e, out| {
        e.mem.set_u32(out.addr(), object);
        e.call(WADING_MAP_GET, &args![map, object, out]).bool()
    });
    if found {
        return;
    }
    if interior_kept {
        if !in_interior(e) {
            if in_interior(e) {
                return;
            }
            if e.call(REFERENCE_IS_EXCLUDED, &args![object]).bool() {
                return;
            }
        }
    } else if e.call(REFERENCE_IS_EXCLUDED, &args![object]).bool() {
        return;
    }
    let node = reference_node(e, object);
    list_add_head(e, list, node);
    e.call(MAP_SET_AT, &args![map, object, object]);
}

// Translated from 004ea860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` at `this + 0x164` (of a `BSShaderAccumulator`;
/// no Xbox PDB name).
pub fn fn_004ea860(e: &mut Engine, this: u32, value: u8) {
    e.mem.set_u8(this + 0x164, value);
}

// Translated from 004ea880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the four words at `color` to `this + 0x154` (of a
/// `BSShaderAccumulator`; no Xbox PDB name).
pub fn fn_004ea880(e: &mut Engine, this: u32, color: u32) {
    copy_words(e, color, this + 0x154, 4);
}

// Translated from 004ea8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `4` is set in the form flags.
pub fn fn_004ea8b0(e: &mut Engine, form: u32) -> bool {
    form_flag(e, form, FORM_FLAG_0004)
}

// Translated from 004ea8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `8` is set in the form flags.
pub fn fn_004ea8d0(e: &mut Engine, form: u32) -> bool {
    form_flag(e, form, FORM_FLAG_0008)
}

// Translated from 004ea8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x10` is set in the form flags.
pub fn fn_004ea8f0(e: &mut Engine, form: u32) -> bool {
    form_flag(e, form, FORM_FLAG_0010)
}

// Translated from 004ea910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x20` is set in the form flags.
pub fn fn_004ea910(e: &mut Engine, form: u32) -> bool {
    form_flag(e, form, FORM_FLAG_0020)
}

// Translated from 004ea930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x40` is set in the form flags.
pub fn fn_004ea930(e: &mut Engine, form: u32) -> bool {
    form_flag(e, form, FORM_FLAG_0040)
}

// Translated from 004ea950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the member at `this + 0x60` (the `Sky`'s three floats the
/// silhouette colour starts from).
pub fn fn_004ea950(_e: &mut Engine, this: u32) -> u32 {
    this + 0x60
}

// Translated from 004ea970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the multithreaded rendering system's static object,
/// `0x01200088`.
pub fn fn_004ea970(_e: &mut Engine) -> u32 {
    MT_RENDERING_SYSTEM
}

// Translated from 004ea980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPointer` member at `this + 0x134`, through `00559450`.
pub fn fn_004ea980(e: &mut Engine, this: u32) -> u32 {
    pointer_in_slot(e, this + 0x134)
}

// Translated from 004ea9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPointer` static at `0x011d86a8`, through `00559450`. `this` is not
/// read.
pub fn fn_004ea9a0(e: &mut Engine, _unused_0: u32) -> u32 {
    pointer_in_slot(e, TERRAIN_POINTER_A)
}

// Translated from 004ea9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPointer` static at `0x011d8690`, through `00559450`. `this` is not
/// read.
pub fn fn_004ea9c0(e: &mut Engine, _unused_0: u32) -> u32 {
    pointer_in_slot(e, TERRAIN_POINTER_B)
}

// Translated from 004ea9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPointer` static at `0x011d86bc`, through `00559450`. `this` is not
/// read.
pub fn fn_004ea9e0(e: &mut Engine, _unused_0: u32) -> u32 {
    pointer_in_slot(e, TERRAIN_POINTER_C)
}

// Translated from 004eaa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `UpdatePlaceableWater`'s world reflection setup (no Xbox PDB name; the
/// exception-unwinding frame is not translated). Within an allocation scope
/// of `TESWater.cpp` line `0xfe3`, when `bUseWaterShader` is on and there is
/// a `viewer`: makes the world reflection camera by reflecting the viewer
/// about the plane through the world space's water height, makes the world
/// reflection accumulator (when it has none) and prepares it, sets the
/// silhouette colour when `bAutoWaterSilhouetteReflections` or
/// `bForceLowDetailReflections` is on, and fills the static and dynamic world
/// object lists: with `bForceHighDetailReflections` the player's 3D object is
/// culled (and `bCull3rdPerson` set) when the player is in third person and
/// the `NiPointer` at `0x011deb7c` is added; otherwise the sky's objects and
/// the terrain objects are. The task goes to the multithreaded renderer with
/// the world reflection thread stage plus one.
pub fn fn_004eaa00(e: &mut Engine, this: Ptr<TESWaterSystem>, viewer: u32) {
    e.with_stack(4, |e, scope| {
        e.call(
            ALLOCATION_SCOPE_CONSTRUCT,
            &args![
                scope,
                0x1du32,
                1u32,
                TESWATER_SOURCE_PATH,
                REFLECTION_SCOPE_LINE_WORLD
            ],
        );
        setup_world_reflections(e, this, viewer);
        e.call(ALLOCATION_SCOPE_DESTRUCT, &args![scope]);
    });
}

/// The body of `fn_004eaa00`.
fn setup_world_reflections(e: &mut Engine, this: Ptr<TESWaterSystem>, viewer: u32) {
    if !e.call(WATER_SHADER_ENABLED, &[]).bool() || viewer == 0 {
        return;
    }
    assign_new_camera(e, WORLD_REFLECTION_CAMERA);
    e.with_stack(0x34, |e, scratch| {
        let scratch = scratch.addr();
        build_world_water_plane(e, scratch, scratch + 0x10, scratch + 0x1c);
        let camera = pointer_in_slot(e, WORLD_REFLECTION_CAMERA);
        e.call(REFLECT_CAMERA_ABOUT_PLANE, &args![viewer, scratch, camera]);
    });
    if pointer_in_slot(e, WORLD_REFLECTION_SORTER) == 0 {
        assign_new_accumulator(e, WORLD_REFLECTION_SORTER);
    }
    prepare_accumulator(e, WORLD_REFLECTION_SORTER, WORLD_REFLECTION_CAMERA);
    if setting_flag(e, SETTING_AUTO_SILHOUETTE_REFLECTIONS)
        || setting_flag(e, SETTING_FORCE_LOW_DETAIL_REFLECTIONS)
    {
        set_silhouette_color(e, WORLD_REFLECTION_SORTER);
    }
    e.call(LIST_REMOVE_ALL, &args![STATIC_WORLD_REFLECTIVE_OBJECTS]);
    e.call(LIST_REMOVE_ALL, &args![DYNAMIC_WORLD_REFLECTIVE_OBJECTS]);
    let static_objects = STATIC_WORLD_REFLECTIVE_OBJECTS;
    if setting_flag(e, SETTING_FORCE_HIGH_DETAIL_REFLECTIONS) {
        let player = e.global::<u32>(PLAYER_CHARACTER);
        if fn_004eaf60(e, player) == 0 {
            e.set(this, TESWaterSystem::bCull3rdPerson, true);
            let node = reference_node(e, player);
            e.call(NODE_SET_CULLED, &args![node, 1u32]);
        }
        if current_world_space(e) != 0 {
            let object = e.call(POINTER_STATIC_011DEB7C, &[]).u32();
            list_add_head(e, static_objects, object);
        }
    } else {
        let tes = e.global::<u32>(TES_POINTER);
        let sky = e.call(TES_GET_SKY, &args![tes]).u32();
        let sky_objects = e.call(SKY_OBJECT_POINTER, &args![sky]).u32();
        list_add_head(e, static_objects, sky_objects);
        if current_world_space(e) != 0 {
            let world_space = current_world_space(e);
            let terrain = e
                .call(WORLD_SPACE_GET_TERRAIN_MANAGER, &args![world_space])
                .u32();
            if terrain != 0 {
                let terrain_ready = e.call(TERRAIN_READY, &[]).bool();
                if !in_interior(e) && terrain_ready {
                    e.call(TERRAIN_STEP_FIRST, &[]);
                    add_terrain_object(e, terrain, fn_004ea9a0, static_objects);
                    e.call(TERRAIN_STEP_SECOND, &[]);
                    add_terrain_object(e, terrain, fn_004ea9c0, static_objects);
                    add_terrain_object(e, terrain, fn_004ea9e0, static_objects);
                }
            }
        }
    }
    let stage = e.global::<i32>(WORLD_REFLECTION_THREAD_STAGE);
    add_accumulator_task(
        e,
        WORLD_REFLECTION_CAMERA,
        STATIC_WORLD_REFLECTIVE_OBJECTS,
        DYNAMIC_WORLD_REFLECTIVE_OBJECTS,
        WORLD_REFLECTION_SORTER,
        stage.wrapping_add(1),
    );
}

// Translated from 004eaf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0x64a` (of the player; the world reflection setup
/// culls the player's 3D object when it is clear).
pub fn fn_004eaf60(e: &mut Engine, this: u32) -> u8 {
    e.mem.u8(this + 0x64a)
}

// Translated from 004eaf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `UpdatePlaceableWater`'s sky reflection setup (no Xbox PDB name; the
/// exception-unwinding frame is not translated): `fn_004eaa00` for the sky
/// camera, accumulator and lists, within the allocation scope of
/// `TESWater.cpp` line `0x1054`, without silhouette colour, with only the
/// sky's objects in the static list and with the sky reflection thread
/// stage plus one.
pub fn fn_004eaf80(e: &mut Engine, _unused_0: u32, viewer: u32) {
    e.with_stack(4, |e, scope| {
        e.call(
            ALLOCATION_SCOPE_CONSTRUCT,
            &args![
                scope,
                0x1du32,
                1u32,
                TESWATER_SOURCE_PATH,
                REFLECTION_SCOPE_LINE_SKY
            ],
        );
        setup_sky_reflections(e, viewer);
        e.call(ALLOCATION_SCOPE_DESTRUCT, &args![scope]);
    });
}

/// The body of `fn_004eaf80`.
fn setup_sky_reflections(e: &mut Engine, viewer: u32) {
    if !e.call(WATER_SHADER_ENABLED, &[]).bool() || viewer == 0 {
        return;
    }
    assign_new_camera(e, SKY_REFLECTION_CAMERA);
    e.with_stack(0x34, |e, scratch| {
        let scratch = scratch.addr();
        build_world_water_plane(e, scratch, scratch + 0x10, scratch + 0x1c);
        let camera = pointer_in_slot(e, SKY_REFLECTION_CAMERA);
        e.call(REFLECT_CAMERA_ABOUT_PLANE, &args![viewer, scratch, camera]);
    });
    if pointer_in_slot(e, SKY_REFLECTION_SORTER) == 0 {
        assign_new_accumulator(e, SKY_REFLECTION_SORTER);
    }
    prepare_accumulator(e, SKY_REFLECTION_SORTER, SKY_REFLECTION_CAMERA);
    e.call(LIST_REMOVE_ALL, &args![STATIC_SKY_REFLECTIVE_OBJECTS]);
    e.call(LIST_REMOVE_ALL, &args![DYNAMIC_SKY_REFLECTIVE_OBJECTS]);
    let tes = e.global::<u32>(TES_POINTER);
    let sky = e.call(TES_GET_SKY, &args![tes]).u32();
    let sky_objects = e.call(SKY_OBJECT_POINTER, &args![sky]).u32();
    list_add_head(e, STATIC_SKY_REFLECTIVE_OBJECTS, sky_objects);
    let stage = e.global::<i32>(SKY_REFLECTION_THREAD_STAGE);
    add_accumulator_task(
        e,
        SKY_REFLECTION_CAMERA,
        STATIC_SKY_REFLECTIVE_OBJECTS,
        DYNAMIC_SKY_REFLECTIVE_OBJECTS,
        SKY_REFLECTION_SORTER,
        stage.wrapping_add(1),
    );
}

// Translated from 004eb220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finishes the reflection of a group in an interior (no Xbox PDB name; the
/// exception-unwinding frame is not translated). Within an allocation scope
/// of `TESWater.cpp` line `0x1082`, when `bUseWaterShader` is on and the group
/// has a reflection camera: clears the renderer's clear-colour alpha, creates
/// the group's reflection texture when it has none, stops the offscreen
/// render of it into render-target mode 7, hands the camera to the render
/// object, runs `fn_004ecef0` with the reflection plane (its normal and the
/// negated constant), draws the group's accumulated scene, resets the render
/// target, restores the clear colour and drops the camera.
pub fn fn_004eb220(e: &mut Engine, this: Ptr<TESWaterSystem>, group: Ptr<PlaceableWaterGroup>) {
    e.with_stack(4, |e, scope| {
        e.call(
            ALLOCATION_SCOPE_CONSTRUCT,
            &args![
                scope,
                0x1du32,
                1u32,
                TESWATER_SOURCE_PATH,
                REFLECTION_SCOPE_LINE_FINISH
            ],
        );
        finish_group_interior(e, this, group);
        e.call(ALLOCATION_SCOPE_DESTRUCT, &args![scope]);
    });
}

/// The body of `fn_004eb220`.
fn finish_group_interior(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    group: Ptr<PlaceableWaterGroup>,
) {
    if !e.call(WATER_SHADER_ENABLED, &[]).bool() {
        return;
    }
    let camera_slot = address_of(group, PlaceableWaterGroup::spReflectionCamera);
    if pointer_in_slot(e, camera_slot) == 0 {
        return;
    }
    let map_slot = address_of(group, PlaceableWaterGroup::spGroupReflectionMap);
    let sorter_slot = address_of(group, PlaceableWaterGroup::spGroupReflectionSorter);
    e.with_stack(0x20, |e, colors| {
        let colors = colors.addr();
        let original = colors;
        let transparent = colors + 0x10;
        e.call(
            NI_POINT4_CONSTRUCT,
            &args![original, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
        );
        let renderer = e.call(RENDERER, &[]).u32();
        e.vcall(renderer, RENDERER_GET_CLEAR_COLOR, &args![original]);
        copy_words(e, original, transparent, 4);
        e.mem.set_f32(transparent + 12, 0.0);
        let renderer = e.call(RENDERER, &[]).u32();
        e.vcall(renderer, RENDERER_SET_CLEAR_COLOR, &args![transparent]);

        if pointer_in_slot(e, map_slot) == 0 {
            let renderer = e.call(RENDERER, &[]).u32();
            let manager = e.call(TEXTURE_MANAGER, &[]).u32();
            let texture = e
                .call(
                    CREATE_RENDERED_TEXTURE,
                    &args![manager, renderer, 9u32, 0u32, 0u32, 0u32],
                )
                .u32();
            assign_slot(e, map_slot, texture);
        }
        let terrain_ready = e.call(TERRAIN_READY, &[]).bool();
        if !in_interior(e) && terrain_ready {
            e.call(TERRAIN_TOGGLE_ONE, &[]);
        }
        let texture = pointer_in_slot(e, map_slot);
        let stopped = e.call(RENDERED_TEXTURE_STOP, &args![texture]).u32();
        e.call(RENDER_TARGET_SET, &args![7u32, stopped]);
        let camera = pointer_in_slot(e, camera_slot);
        let frustum = e.call(CAMERA_FRUSTUM_ADDRESS, &args![camera]).u32();
        let object = e.call(RENDER_GLOBAL_OBJECT, &[]).u32();
        e.call(RENDER_OBJECT_SET_CAMERA_DATA, &args![object, frustum]);
        let stage = e
            .get(group, PlaceableWaterGroup::iReflectionThreadStage)
            .wrapping_add(1);
        let renderer = fn_004ea970(e);
        e.call(MT_SET_THREAD_STAGE_ONE, &args![renderer, 1u32, stage]);
        let camera = pointer_in_slot(e, camera_slot);
        let object = e.call(RENDER_GLOBAL_OBJECT, &[]).u32();
        fn_004e9bb0(e, object, camera);

        let plane = group.at(PlaceableWaterGroup::ReflectWaterPlane).addr();
        let constant = plane_constant(e, plane);
        let negated = -constant;
        let normal_z = {
            let normal = e.call(ADDRESS_OF_THIS, &args![plane]).u32();
            e.mem.f32(normal + 8)
        };
        let normal_y = {
            let normal = e.call(ADDRESS_OF_THIS, &args![plane]).u32();
            e.mem.f32(normal + 4)
        };
        let normal_x = {
            let normal = e.call(ADDRESS_OF_THIS, &args![plane]).u32();
            e.mem.f32(normal)
        };
        e.with_stack(0x10, |e, vector| {
            let vector = vector.addr();
            e.call(
                NI_POINT4_CONSTRUCT,
                &args![vector, normal_x, normal_y, normal_z, negated],
            );
            let words: Vec<u32> = (0..4).map(|i| e.mem.u32(vector + 4 * i)).collect();
            e.call(
                REFLECTION_PLANE_SETUP,
                &args![this, words[0], words[1], words[2], words[3], 0u32],
            );
        });
        e.call(
            RENDER_STATE_SET,
            &args![1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        let sorter = pointer_in_slot(e, sorter_slot);
        let camera = pointer_in_slot(e, camera_slot);
        e.call(RENDER_ACCUMULATED_SCENE, &args![camera, sorter, 0u32]);
        fn_004eb510(e, 0);
        e.call(RENDER_TARGET_RESET, &[]);
        if !in_interior(e) && terrain_ready {
            e.call(TERRAIN_TOGGLE_ZERO, &[]);
        }
        let renderer = e.call(RENDERER, &[]).u32();
        e.vcall(renderer, RENDERER_SET_CLEAR_COLOR, &args![original]);
        assign_slot(e, camera_slot, 0);
    });
}

// Translated from 004eb510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lowers the word at `0x011ffa14` by `amount`, then makes the render state
/// call `00b98280(0, 0, 0, 0, 0, 0, 0)` (cdecl).
pub fn fn_004eb510(e: &mut Engine, amount: u32) {
    let counter = e.global::<u32>(COUNTER_011FFA14);
    e.set_global(COUNTER_011FFA14, counter.wrapping_sub(amount));
    e.call(
        RENDER_STATE_SET,
        &args![0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
}

// ---------------------------------------------------------------------------
// Helpers of the functions from `004eb540` on (fourth session).

/// Runs `body` between the allocation scope guard of `TESWater.cpp` line
/// `line` (`00404eb0(scope, 0x1d, 1, file name, line)` and `00404ee0`); the
/// guard is a four-byte object on the game's stack.
fn in_allocation_scope(e: &mut Engine, line: u32, body: impl FnOnce(&mut Engine)) {
    e.with_stack(4, |e, scope| {
        e.call(
            ALLOCATION_SCOPE_CONSTRUCT,
            &args![scope, 0x1du32, 1u32, TESWATER_SOURCE_PATH, line],
        );
        body(e);
        e.call(ALLOCATION_SCOPE_DESTRUCT, &args![scope]);
    });
}

/// `BSTextureManager::CreateRenderedTexture(manager, renderer, kind, 0, 0,
/// 0)` (`00b6e110`): the game fetches the renderer, then the texture
/// manager.
fn create_rendered_texture(e: &mut Engine, kind: u32) -> u32 {
    let renderer = e.call(RENDERER, &[]).u32();
    let manager = e.call(TEXTURE_MANAGER, &[]).u32();
    e.call(
        CREATE_RENDERED_TEXTURE,
        &args![manager, renderer, kind, 0u32, 0u32, 0u32],
    )
    .u32()
}

/// Reads the renderer's clear colour into `original` (0x10 bytes) and sets
/// the same colour with alpha 0, built in `transparent` (0x10 bytes).
fn set_transparent_clear_color(e: &mut Engine, original: u32, transparent: u32) {
    e.call(
        NI_POINT4_CONSTRUCT,
        &args![original, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
    );
    let renderer = e.call(RENDERER, &[]).u32();
    e.vcall(renderer, RENDERER_GET_CLEAR_COLOR, &args![original]);
    copy_words(e, original, transparent, 4);
    e.mem.set_f32(transparent + 12, 0.0);
    let renderer = e.call(RENDERER, &[]).u32();
    e.vcall(renderer, RENDERER_SET_CLEAR_COLOR, &args![transparent]);
}

/// Gives the clear colour at `original` back to the renderer.
fn restore_clear_color(e: &mut Engine, original: u32) {
    let renderer = e.call(RENDERER, &[]).u32();
    e.vcall(renderer, RENDERER_SET_CLEAR_COLOR, &args![original]);
}

/// Stops the offscreen render of the rendered texture in `map_slot` and
/// makes the result render-target mode 7 (`00b6b8d0(7, stopped)`).
fn stop_into_render_target(e: &mut Engine, map_slot: u32) {
    let texture = pointer_in_slot(e, map_slot);
    let stopped = e.call(RENDERED_TEXTURE_STOP, &args![texture]).u32();
    e.call(RENDER_TARGET_SET, &args![7u32, stopped]);
}

/// Hands the frustum of the camera in `camera_slot` to the render object
/// (`004a0dd0(object, 004a0d10(camera))`).
fn give_camera_data_to_render_object(e: &mut Engine, camera_slot: u32) {
    let camera = pointer_in_slot(e, camera_slot);
    let frustum = e.call(CAMERA_FRUSTUM_ADDRESS, &args![camera]).u32();
    let object = e.call(RENDER_GLOBAL_OBJECT, &[]).u32();
    e.call(RENDER_OBJECT_SET_CAMERA_DATA, &args![object, frustum]);
}

/// `MTRenderingSystem::SetThreadStage`-like `00ba3130(system, 1, stage)`.
fn set_thread_stage_one(e: &mut Engine, stage: i32) {
    let system = fn_004ea970(e);
    e.call(MT_SET_THREAD_STAGE_ONE, &args![system, 1u32, stage]);
}

/// Hands the reflection plane `plane` to the plane setup `fn_004ecef0`: its
/// normal and `last` as the fourth word (the game reads the normal's z, y
/// and x in that order).
fn submit_reflection_plane(e: &mut Engine, this: u32, plane: u32, last: f32) {
    let normal_z = {
        let normal = plane_normal(e, plane);
        e.mem.f32(normal + 8)
    };
    let normal_y = {
        let normal = plane_normal(e, plane);
        e.mem.f32(normal + 4)
    };
    let normal_x = {
        let normal = plane_normal(e, plane);
        e.mem.f32(normal)
    };
    e.with_stack(0x10, |e, vector| {
        let vector = vector.addr();
        e.call(
            NI_POINT4_CONSTRUCT,
            &args![vector, normal_x, normal_y, normal_z, last],
        );
        let words: Vec<f32> = (0..4).map(|i| e.mem.f32(vector + 4 * i)).collect();
        fn_004ecef0(e, this, words[0], words[1], words[2], words[3], 0);
    });
}

/// What the world and the sky reflection finish do once the render target
/// is set: the world-space water plane (`plane`, 0x10 bytes, with `normal`
/// and `point`, 0xc bytes each, as scratch), the camera to the render
/// object, the plane to `fn_004ecef0` (its normal and the negated
/// constant), the accumulated scene drawn with the sorter, and the render
/// state reset.
fn draw_reflection_pass(
    e: &mut Engine,
    this: u32,
    camera_slot: u32,
    sorter_slot: u32,
    scratch: u32,
) {
    let plane = scratch;
    build_world_water_plane(e, plane, scratch + 0x10, scratch + 0x1c);
    let camera = pointer_in_slot(e, camera_slot);
    let object = e.call(RENDER_GLOBAL_OBJECT, &[]).u32();
    fn_004e9bb0(e, object, camera);
    let constant = plane_constant(e, plane);
    submit_reflection_plane(e, this, plane, -constant);
    e.call(
        RENDER_STATE_SET,
        &args![1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
    let sorter = pointer_in_slot(e, sorter_slot);
    let camera = pointer_in_slot(e, camera_slot);
    e.call(RENDER_ACCUMULATED_SCENE, &args![camera, sorter, 0u32]);
    fn_004eb510(e, 0);
    e.call(RENDER_TARGET_RESET, &[]);
}

/// The blur the world reflection is drawn through (`bUseWaterReflectionBlur`):
/// a rendered texture of kind `0x16` goes into slot 2 of the image-space
/// effect `0x10 + iWaterBlurAmount`, the effect is rendered with the world
/// reflection map as its input, and the texture is given back. `scratch`
/// (0x34 bytes) holds the `ImageSpaceTexture` holder and the
/// `ImageSpaceEffectParam`.
fn blur_world_reflection(e: &mut Engine, scratch: u32) {
    let texture_holder = scratch;
    let param = scratch + 0x10;
    let blur_texture = create_rendered_texture(e, 0x16);
    fn_004ebb70(e, texture_holder);
    e.call(
        IMAGE_SPACE_TEXTURE_SET,
        &args![texture_holder, blur_texture],
    );
    image_space_effect_param_image_space_effect_param(e, param);
    let amount = setting_int(e, SETTING_WATER_BLUR_AMOUNT);
    let one = e.global::<f64>(ONE);
    e.set_global(WATER_BLUR_FACTOR, (amount as f64 + one) as f32);
    let index = setting_int(e, SETTING_WATER_BLUR_AMOUNT).wrapping_add(BLUR_EFFECT_BASE_INDEX);
    let manager = fn_004e3270(e);
    let effect = fn_004ebbc0(e, manager, index);
    e.call(
        IMAGE_SPACE_EFFECT_SET_TEXTURE,
        &args![effect, BLUR_EFFECT_TEXTURE_SLOT, texture_holder, 0u32],
    );
    let texture_a = pointer_in_slot(e, WORLD_REFLECTION_MAP);
    let texture_b = pointer_in_slot(e, WORLD_REFLECTION_MAP);
    let renderer = e.call(RENDERER, &[]).u32();
    let manager = fn_004e3270(e);
    e.call(
        IMAGE_SPACE_RENDER_DISPLACEMENT,
        &args![manager, index, renderer, texture_b, texture_a, param, 1u32],
    );
    let texture_manager = e.call(TEXTURE_MANAGER, &[]).u32();
    e.call(
        RETURN_RENDERED_TEXTURE,
        &args![texture_manager, blur_texture],
    );
    fn_004eba90(e, param);
    e.call(IMAGE_SPACE_TEXTURE_DESTRUCT, &args![texture_holder]);
}

/// If the 3D object of `reference` exists and the system's `DepthRefMap`
/// has no entry for the reference: adds the 3D object to `list` and maps
/// the reference to itself. The game keeps the reference in a local it
/// passes to `GetAt` (which fills it when it finds an entry) and reads it
/// again for `SetAt`.
fn add_depth_object(e: &mut Engine, this: Ptr<TESWaterSystem>, list: u32, reference: u32) {
    if reference_node(e, reference) == 0 {
        return;
    }
    let map = address_of(this, TESWaterSystem::DepthRefMap);
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, reference);
        let key = e.mem.u32(local);
        if e.call(WADING_MAP_GET, &args![map, key, local]).bool() {
            return;
        }
        let node = reference_node(e, reference);
        list_add_head(e, list, node);
        let key = e.mem.u32(local);
        let value = e.mem.u32(local);
        e.call(MAP_SET_AT, &args![map, key, value]);
    });
}

/// What `fn_004ebef0` remembers while it walks the water references of a
/// group: the grid cells and the actors of the group are added only once.
#[derive(Default)]
struct DepthSetupDone {
    grid_cells: bool,
    actors: bool,
}

/// The cell nodes of the loaded grid (four children of every loaded cell)
/// become static depth objects of the group.
fn add_grid_depth_objects(e: &mut Engine, group: Ptr<PlaceableWaterGroup>) {
    let static_objects = address_of(group, PlaceableWaterGroup::StaticDepthObjects);
    let mut x = 0u32;
    while x < setting_int(e, SETTING_GRID_SIZE) {
        let mut y = 0u32;
        while y < setting_int(e, SETTING_GRID_SIZE) {
            let tes = e.global::<u32>(TES_POINTER);
            let slot = e.call(TES_GRID_CELL_SLOT, &args![tes, x, y]).u32();
            let cell = e.mem.u32(slot);
            if cell != 0 {
                for index in 0..4u32 {
                    let child = e.call(CELL_NODE_CHILD, &args![cell, index]).u32();
                    list_add_head(e, static_objects, child);
                }
            }
            y += 1;
        }
        x += 1;
    }
}

/// The actors in the water of the group become dynamic depth objects (the
/// player only when `fn_004eaf60` allows it).
fn add_actor_depth_objects(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    group: Ptr<PlaceableWaterGroup>,
) {
    let actors = address_of(group, PlaceableWaterGroup::ActorsInWaterList);
    let dynamic_objects = address_of(group, PlaceableWaterGroup::DynamicDepthObjects);
    let mut position = pointer_in_slot(e, actors);
    while position != 0 {
        let next = e.call(LIST_NEXT_POSITION, &args![actors, position]).u32();
        let slot = e.call(LIST_ITEM_SLOT, &args![actors, position]).u32();
        let actor = e.mem.u32(slot);
        let player = e.global::<u32>(PLAYER_CHARACTER);
        if actor != player || fn_004eaf60(e, player) != 0 {
            add_depth_object(e, this, dynamic_objects, actor);
        }
        position = next;
    }
}

/// The objects reflected in the water reference (`ExtraDataList::
/// GetReflectedRefs`) whose record has bit 2 set become static depth
/// objects.
fn add_reflected_depth_objects(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    group: Ptr<PlaceableWaterGroup>,
    reference: u32,
) {
    let static_objects = address_of(group, PlaceableWaterGroup::StaticDepthObjects);
    let extra_data = e.call(REFERENCE_EXTRA_DATA_LIST, &args![reference]).u32();
    let mut node = e
        .call(EXTRA_DATA_LIST_GET_REFLECTED_REFS, &args![extra_data])
        .u32();
    while node != 0 && !e.call(WORDS_ARE_ZERO, &args![node]).bool() {
        let record = {
            let slot = e.call(ADDRESS_OF_THIS, &args![node]).u32();
            e.mem.u32(slot)
        };
        // The record: the reflected reference, then a flag word.
        if e.mem.u32(record + 4) & 2 != 0 {
            let record = {
                let slot = e.call(ADDRESS_OF_THIS, &args![node]).u32();
                e.mem.u32(slot)
            };
            let reflected = e.mem.u32(record);
            add_depth_object(e, this, static_objects, reflected);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

/// The depth setup of one water reference of a group (`fn_004ebef0`).
fn setup_depth_for_reference(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    viewer: u32,
    group: Ptr<PlaceableWaterGroup>,
    reference: u32,
    done: &mut DepthSetupDone,
) {
    if reference == 0 || e.call(REFERENCE_PARENT_CELL, &args![reference]).u32() == 0 {
        return;
    }
    let cell = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32();
    if !e.call(CELL_BYTE_IS_SIX, &args![cell]).bool() {
        return;
    }
    let geometry = fn_004e8030(e, this, reference);
    let property = if geometry != 0 {
        water_shader_property(e, geometry)
    } else {
        Ptr::new(0)
    };
    if property.is_null() || !fn_004e62e0(e, this.addr(), reference, viewer) {
        return;
    }
    e.call(
        NI_POINTER_ASSIGN_FROM,
        &args![
            address_of(property, WaterShaderProperty::spDepthMap),
            DEPTH_MAP
        ],
    );
    let form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
    if !fn_004e32e0(e, form) {
        return;
    }
    if form != 0 && fn_004e32a0(e, form) && !done.grid_cells {
        done.grid_cells = true;
        add_grid_depth_objects(e, group);
    }
    if form != 0 && fn_004ec7e0(e, form) && !done.actors {
        done.actors = true;
        add_actor_depth_objects(e, this, group);
    }
    add_reflected_depth_objects(e, this, group, reference);
    // The objects in the water: the game takes the next position from the
    // group's `PlaceableWaterList` (the node's `next` does not depend on
    // the list it asks).
    let objects = address_of(group, PlaceableWaterGroup::ObjectInWaterList);
    let references = address_of(group, PlaceableWaterGroup::PlaceableWaterList);
    let dynamic_objects = address_of(group, PlaceableWaterGroup::DynamicDepthObjects);
    let mut position = pointer_in_slot(e, objects);
    while position != 0 {
        let next = e
            .call(LIST_NEXT_POSITION, &args![references, position])
            .u32();
        let slot = e.call(LIST_ITEM_SLOT, &args![objects, position]).u32();
        let object = e.mem.u32(slot);
        add_depth_object(e, this, dynamic_objects, object);
        position = next;
    }
    if form != 0 && e.call(FORM_FLAG_TEST_40000000, &args![form]).bool() {
        let static_objects = address_of(group, PlaceableWaterGroup::StaticDepthObjects);
        let mut node = fn_004ec7b0(e);
        while node != 0 && !e.call(WORDS_ARE_ZERO, &args![node]).bool() {
            let object = {
                let slot = e.call(ADDRESS_OF_THIS, &args![node]).u32();
                e.mem.u32(slot)
            };
            if object != 0 {
                add_depth_object(e, this, static_objects, object);
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    }
}

// ---------------------------------------------------------------------------
// Translated functions, fourth session (`004eb540` on).

// Translated from 004eb540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The world reflection finish of `UpdatePlaceableWater` (no Xbox PDB name;
/// the exception-unwinding frame is not translated). Within an allocation
/// scope of `TESWater.cpp` line `0x10b8`, when `bUseWaterShader` is on and
/// the world reflection camera exists: clears the clear colour's alpha,
/// makes the world reflection texture (kind 9) when there is none, stops the
/// offscreen render of it into render-target mode 7, gives the camera to the
/// render object and sets the world thread stage plus one. The distant
/// terrain is toggled around the draw when it is ready (and
/// `bForceHighDetailReflections` is off) outside interiors; the player's 3D
/// object is shown again when the world setup had culled it. The scene is
/// drawn through the water plane (`fn_004ecef0`), the clear colour restored
/// and the camera dropped; with `bUseWaterReflectionBlur` the reflection is
/// then blurred by the image-space effect `0x10 + iWaterBlurAmount`.
pub fn fn_004eb540(e: &mut Engine, this: Ptr<TESWaterSystem>) {
    in_allocation_scope(e, SCOPE_LINE_FINISH_WORLD, |e| {
        finish_world_reflections(e, this);
    });
}

/// The body of `fn_004eb540`.
fn finish_world_reflections(e: &mut Engine, this: Ptr<TESWaterSystem>) {
    if !e.call(WATER_SHADER_ENABLED, &[]).bool() || pointer_in_slot(e, WORLD_REFLECTION_CAMERA) == 0
    {
        return;
    }
    e.with_stack(0x60 + IMAGE_SPACE_EFFECT_PARAM_SIZE, |e, frame| {
        let frame = frame.addr();
        let original = frame;
        set_transparent_clear_color(e, original, frame + 0x10);
        if pointer_in_slot(e, WORLD_REFLECTION_MAP) == 0 {
            let texture = create_rendered_texture(e, 9);
            assign_slot(e, WORLD_REFLECTION_MAP, texture);
        }
        let terrain_ready = e.call(TERRAIN_READY, &[]).bool()
            && !setting_flag(e, SETTING_FORCE_HIGH_DETAIL_REFLECTIONS);
        if !in_interior(e) && terrain_ready {
            e.call(TERRAIN_TOGGLE_ONE, &[]);
        }
        stop_into_render_target(e, WORLD_REFLECTION_MAP);
        give_camera_data_to_render_object(e, WORLD_REFLECTION_CAMERA);
        let stage = e
            .global::<i32>(WORLD_REFLECTION_THREAD_STAGE)
            .wrapping_add(1);
        set_thread_stage_one(e, stage);
        if e.get(this, TESWaterSystem::bCull3rdPerson) {
            let player = e.global::<u32>(PLAYER_CHARACTER);
            if fn_004eaf60(e, player) != 0 {
                let node = reference_node(e, player);
                e.call(NODE_SET_CULLED, &args![node, 0u32]);
            }
        }
        draw_reflection_pass(
            e,
            this.addr(),
            WORLD_REFLECTION_CAMERA,
            WORLD_REFLECTION_SORTER,
            frame + 0x20,
        );
        if !in_interior(e) && terrain_ready {
            e.call(TERRAIN_TOGGLE_ZERO, &[]);
        }
        restore_clear_color(e, original);
        assign_slot(e, WORLD_REFLECTION_CAMERA, 0);
        if setting_flag(e, SETTING_USE_WATER_REFLECTION_BLUR) {
            blur_world_reflection(e, frame + 0x50);
        }
    });
}

// Translated from 004eba20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffectParam::ImageSpaceEffectParam` (Xbox PDB): sets the
/// vtable and constructs its two members (at `+4` and `+0x14`) with `(0, 1)`.
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn image_space_effect_param_image_space_effect_param(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, IMAGE_SPACE_EFFECT_PARAM_VTABLE);
    e.call(
        EFFECT_PARAM_FIRST_MEMBER_CONSTRUCT,
        &args![this + 4, 0u32, 1u32],
    );
    e.call(
        EFFECT_PARAM_SECOND_MEMBER_CONSTRUCT,
        &args![this + 0x14, 0u32, 1u32],
    );
    this
}

// Translated from 004eba90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `ImageSpaceEffectParam` (the engine map has no
/// name): resets the vtable and destroys the second, then the first member.
pub fn fn_004eba90(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, IMAGE_SPACE_EFFECT_PARAM_VTABLE);
    fn_004ebb50(e, this + 0x14);
    fn_004ebb30(e, this + 4);
}

// Translated from 004ebb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffectParam::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body and, when bit 0 of `flags` is set, frees the object.
/// Returns `this`.
pub fn image_space_effect_param_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    fn_004eba90(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004ebb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the first member (at `+4`) of an `ImageSpaceEffectParam`: calls
/// its destructor `004edc60` (the engine map has no name here).
pub fn fn_004ebb30(e: &mut Engine, this: u32) {
    e.call(EFFECT_PARAM_FIRST_MEMBER_DESTRUCT, &args![this]);
}

// Translated from 004ebb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the second member (at `+0x14`) of an `ImageSpaceEffectParam`:
/// calls its destructor `004edcc0`.
pub fn fn_004ebb50(e: &mut Engine, this: u32) {
    e.call(EFFECT_PARAM_SECOND_MEMBER_DESTRUCT, &args![this]);
}

// Translated from 004ebb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the 0x10-byte `ImageSpaceTexture` holder (the engine map has no
/// name): three flag bytes cleared, the `NiPointer` at `+4` set to null
/// through `00633c90`, and the words at `+8` and `+0xc` cleared. Returns
/// `this`.
pub fn fn_004ebb70(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u8(this, 0);
    e.mem.set_u8(this + 1, 0);
    e.mem.set_u8(this + 2, 0);
    e.call(POINTER_TEMP_CONSTRUCT, &args![this + 4, 0u32]);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 0xc, 0);
    this
}

// Translated from 004ebbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Element `index` of the pointer array at `[this + 8]` (the engine map
/// names the body `CDocManager::GetNextDocTemplate`, a folded library
/// name): the pointer stored there. The image-space manager's effects are
/// read this way.
pub fn fn_004ebbc0(e: &mut Engine, this: u32, index: u32) -> u32 {
    let slot = e.call(ARRAY_ELEMENT_ADDRESS, &args![this + 4, index]).u32();
    e.mem.u32(slot)
}

// Translated from 004ebbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sky reflection finish of `UpdatePlaceableWater` (no Xbox PDB name;
/// the exception-unwinding frame is not translated): `fn_004eb540` for the
/// sky camera, map and sorter, within the allocation scope of `TESWater.cpp`
/// line `0x1114`, with the sky thread stage plus one, the distant terrain
/// toggled whenever it is ready outside interiors, without the player
/// culling and without the blur.
pub fn fn_004ebbe0(e: &mut Engine, this: Ptr<TESWaterSystem>) {
    in_allocation_scope(e, SCOPE_LINE_FINISH_SKY, |e| {
        finish_sky_reflections(e, this);
    });
}

/// The body of `fn_004ebbe0`.
fn finish_sky_reflections(e: &mut Engine, this: Ptr<TESWaterSystem>) {
    if !e.call(WATER_SHADER_ENABLED, &[]).bool() || pointer_in_slot(e, SKY_REFLECTION_CAMERA) == 0 {
        return;
    }
    e.with_stack(0x50, |e, frame| {
        let frame = frame.addr();
        let original = frame;
        set_transparent_clear_color(e, original, frame + 0x10);
        if pointer_in_slot(e, SKY_REFLECTION_MAP) == 0 {
            let texture = create_rendered_texture(e, 9);
            assign_slot(e, SKY_REFLECTION_MAP, texture);
        }
        let terrain_ready = e.call(TERRAIN_READY, &[]).bool();
        if !in_interior(e) && terrain_ready {
            e.call(TERRAIN_TOGGLE_ONE, &[]);
        }
        stop_into_render_target(e, SKY_REFLECTION_MAP);
        give_camera_data_to_render_object(e, SKY_REFLECTION_CAMERA);
        let stage = e.global::<i32>(SKY_REFLECTION_THREAD_STAGE).wrapping_add(1);
        set_thread_stage_one(e, stage);
        draw_reflection_pass(
            e,
            this.addr(),
            SKY_REFLECTION_CAMERA,
            SKY_REFLECTION_SORTER,
            frame + 0x20,
        );
        if !in_interior(e) && terrain_ready {
            e.call(TERRAIN_TOGGLE_ZERO, &[]);
        }
        restore_clear_color(e, original);
        assign_slot(e, SKY_REFLECTION_CAMERA, 0);
    });
}

// Translated from 004ebef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The depth setup of a water group (`SETUP_GROUP_DEPTH`; no Xbox PDB name;
/// the exception-unwinding frame is not translated). Within an allocation
/// scope of `TESWater.cpp` line `0x114f`, when `bUseWaterShader` is on and
/// there is a `viewer`: makes the group's depth camera from the viewer's
/// world translate, rotation, scale, the `float` at `+0xfc` and frustum,
/// updates it, makes the depth accumulator when the group has none and
/// prepares it, makes the shared depth map (kind `0x11`) when there is none,
/// and fills the group's static and dynamic depth object lists: for every
/// water reference of the group in range, the reference's shader property
/// takes the depth map and, by the flags of its base form, the loaded grid
/// cells (once), the actors in the water (once), the reflected references,
/// the objects in the water and the list at `0x011ca144` are added unless
/// the system's `DepthRefMap` already has them. The task goes to the
/// multithreaded renderer with the group's depth thread stage plus one.
pub fn fn_004ebef0(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    viewer: u32,
    group: Ptr<PlaceableWaterGroup>,
) {
    in_allocation_scope(e, SCOPE_LINE_SETUP_DEPTH, |e| {
        setup_group_depth(e, this, viewer, group);
    });
}

/// The body of `fn_004ebef0`.
fn setup_group_depth(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    viewer: u32,
    group: Ptr<PlaceableWaterGroup>,
) {
    if !e.call(WATER_SHADER_ENABLED, &[]).bool() || viewer == 0 {
        return;
    }
    let camera_slot = address_of(group, PlaceableWaterGroup::spDepthCamera);
    let sorter_slot = address_of(group, PlaceableWaterGroup::spDepthSorter);
    assign_new_camera(e, camera_slot);
    let translate = e.call(NODE_WORLD_TRANSLATE, &args![viewer]).u32();
    let camera = pointer_in_slot(e, camera_slot);
    e.call(NODE_SET_LOCAL_TRANSLATE, &args![camera, translate]);
    let rotate = e.call(NODE_WORLD_ROTATE, &args![viewer]).u32();
    let camera = pointer_in_slot(e, camera_slot);
    e.call(NODE_SET_LOCAL_ROTATE, &args![camera, rotate]);
    let scale = e.call(NODE_SCALE_SOURCE, &args![viewer]).f32();
    let camera = pointer_in_slot(e, camera_slot);
    e.call(NODE_SET_LOCAL_SCALE, &args![camera, scale]);
    let far_value = e.call(CAMERA_FLOAT_FC_READ, &args![viewer]).f32();
    let camera = pointer_in_slot(e, camera_slot);
    e.call(CAMERA_FLOAT_FC_WRITE, &args![camera, far_value]);
    let frustum = e.call(CAMERA_PLANES_ADDRESS, &args![viewer]).u32();
    let camera = pointer_in_slot(e, camera_slot);
    e.call(CAMERA_SET_VIEW_FRUSTUM, &args![camera, frustum]);
    e.with_stack(0x10, |e, update_data| {
        e.call(
            UPDATE_DATA_CONSTRUCT,
            &args![update_data, 0.0f32, 0u32, 0u32],
        );
        let camera = pointer_in_slot(e, camera_slot);
        e.call(NODE_UPDATE, &args![camera, update_data]);
    });
    if pointer_in_slot(e, sorter_slot) == 0 {
        assign_new_accumulator(e, sorter_slot);
    }
    let table_entry = e.call(RENDER_TABLE_ENTRY, &args![0u32]).u32();
    let sorter = pointer_in_slot(e, sorter_slot);
    e.call(ACCUMULATOR_SET_WORD_194, &args![sorter, table_entry]);
    let sorter = pointer_in_slot(e, sorter_slot);
    fn_004ec7c0(e, sorter, 1);
    let sorter = pointer_in_slot(e, sorter_slot);
    let camera = pointer_in_slot(e, camera_slot);
    e.vcall(sorter, ACCUMULATOR_SET_CAMERA_VIRTUAL, &args![camera]);
    let sorter = pointer_in_slot(e, sorter_slot);
    e.call(ACCUMULATOR_SET_ACCUMULATE, &args![sorter, 1u32]);
    if pointer_in_slot(e, DEPTH_MAP) == 0 {
        let texture = create_rendered_texture(e, 0x11);
        assign_slot(e, DEPTH_MAP, texture);
    }

    let references = address_of(group, PlaceableWaterGroup::PlaceableWaterList);
    let mut done = DepthSetupDone::default();
    e.with_stack(4, |e, cursor| {
        let cursor = cursor.addr();
        let head = pointer_in_slot(e, references);
        e.mem.set_u32(cursor, head);
        while e.mem.u32(cursor) != 0 {
            let slot = e.call(LIST_NEXT_ITEM, &args![references, cursor]).u32();
            let reference = e.mem.u32(slot);
            setup_depth_for_reference(e, this, viewer, group, reference, &mut done);
        }
    });

    let stage = e.get(group, PlaceableWaterGroup::iDepthThreadStage);
    let sorter = pointer_in_slot(e, sorter_slot);
    let camera = pointer_in_slot(e, camera_slot);
    let system = fn_004ea970(e);
    e.call(
        MT_ADD_ACCUM_TASK,
        &args![
            system,
            camera,
            0u32,
            0u32,
            address_of(group, PlaceableWaterGroup::StaticDepthObjects),
            address_of(group, PlaceableWaterGroup::DynamicDepthObjects),
            sorter,
            0u32,
            stage.wrapping_add(1),
            0u32
        ],
    );
    let stage = e.get(group, PlaceableWaterGroup::iDepthThreadStage);
    let system = fn_004ea970(e);
    e.call(
        MT_SET_THREAD_STAGE,
        &args![system, 0u32, stage.wrapping_add(1)],
    );
}

// Translated from 004ec7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the list head at `0x011ca144`.
pub fn fn_004ec7b0(_e: &mut Engine) -> u32 {
    EXTRA_DEPTH_OBJECT_LIST
}

// Translated from 004ec7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at `this + 0x165` (of a `BSShaderAccumulator`; the byte
/// next to the one `fn_004ea860` sets).
pub fn fn_004ec7c0(e: &mut Engine, this: u32, value: u8) {
    e.mem.set_u8(this + 0x165, value);
}

// Translated from 004ec7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x400` is set in the form flags.
pub fn fn_004ec7e0(e: &mut Engine, form: u32) -> bool {
    form_flag(e, form, FORM_FLAG_0400)
}

// Translated from 004ec800 (decompiled, FalloutNV.exe 1.4.0.525)
/// The depth render of a water group (`AFTER_GROUP_DEPTH`; no Xbox PDB
/// name; the exception-unwinding frame is not translated). Within an
/// allocation scope of `TESWater.cpp` line `0x1209`, when `bUseWaterShader`
/// is on and the group has a depth camera: hands the camera to the render
/// object, sets the group's depth thread stage plus one, publishes the water
/// form's two floats (`+0xa4`, `+0xa8`) and the group's reflect plane for the
/// shaders (`0x011f9604`, `0x011f9614`), sets the depth accumulator's word
/// `+0x19c` to `0xe` and the render states of the pass (stencil reference
/// `0xff`, mask `stencil_mask`), sends the refract plane lowered by
/// `fRefractionWaterPlaneBias` through `fn_004ecef0`, draws the group's
/// accumulated scene and resets the states and the camera.
pub fn fn_004ec800(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    group: Ptr<PlaceableWaterGroup>,
    stencil_mask: u16,
) {
    in_allocation_scope(e, SCOPE_LINE_FINISH_DEPTH, |e| {
        finish_group_depth(e, this, group, stencil_mask);
    });
}

/// The body of `fn_004ec800`.
fn finish_group_depth(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    group: Ptr<PlaceableWaterGroup>,
    stencil_mask: u16,
) {
    let camera_slot = address_of(group, PlaceableWaterGroup::spDepthCamera);
    let sorter_slot = address_of(group, PlaceableWaterGroup::spDepthSorter);
    if !e.call(WATER_SHADER_ENABLED, &[]).bool() || pointer_in_slot(e, camera_slot) == 0 {
        return;
    }
    give_camera_data_to_render_object(e, camera_slot);
    let stage = e.get(group, PlaceableWaterGroup::iDepthThreadStage);
    set_thread_stage_one(e, stage.wrapping_add(1));
    let camera = pointer_in_slot(e, camera_slot);
    let object = e.call(RENDER_GLOBAL_OBJECT, &[]).u32();
    fn_004e9bb0(e, object, camera);
    let water_type = e.get(group, PlaceableWaterGroup::pWaterType).addr();
    let second = e.call(WATER_FORM_FLOAT_A8, &args![water_type]).f32();
    let first = e.call(WATER_FORM_FLOAT_A4, &args![water_type]).f32();
    e.with_stack(0x20, |e, scratch| {
        let scratch = scratch.addr();
        let range = e
            .call(NI_POINT2_CONSTRUCT, &args![scratch, first, second])
            .u32();
        let x = e.mem.u32(range);
        let y = e.mem.u32(range + 4);
        e.set_global(DEPTH_RANGE_SHADER_CONSTANT, x);
        e.set_global(DEPTH_RANGE_SHADER_CONSTANT + 4, y);
    });
    let sorter = pointer_in_slot(e, sorter_slot);
    e.call(ACCUMULATOR_SET_WORD_19C, &args![sorter, 0xeu32]);
    e.call(COUNTED_STATE_34, &args![1u32, 1u32]);
    e.call(
        SET_STENCIL_STATE,
        &args![2u32, DEPTH_STENCIL_REFERENCE, stencil_mask as u32, 1u32],
    );
    e.call(RENDER_STATE_980C0, &args![0u32, 0u32, 0u32, 1u32]);
    e.call(RENDER_STATE_98230, &args![0u32, 1u32]);
    let camera = pointer_in_slot(e, camera_slot);
    let object = e.call(RENDER_GLOBAL_OBJECT, &[]).u32();
    fn_004e9bb0(e, object, camera);
    let reflect_plane = address_of(group, PlaceableWaterGroup::ReflectWaterPlane);
    for i in 0..4 {
        let word = e.mem.u32(reflect_plane + 4 * i);
        e.set_global(DEPTH_PLANE_SHADER_CONSTANT + 4 * i, word);
    }
    let refract_plane = address_of(group, PlaceableWaterGroup::RefractWaterPlane);
    let constant = plane_constant(e, refract_plane);
    let bias = {
        let value = e
            .call(
                SETTING_FLOAT_VALUE,
                &args![SETTING_REFRACTION_WATER_PLANE_BIAS],
            )
            .u32();
        e.mem.f32(value)
    };
    let lowered = (bias as f64 + (-constant) as f64) as f32;
    submit_reflection_plane(e, this.addr(), refract_plane, lowered);
    e.call(
        RENDER_STATE_SET,
        &args![1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
    let sorter = pointer_in_slot(e, sorter_slot);
    let camera = pointer_in_slot(e, camera_slot);
    e.call(RENDER_ACCUMULATED_SCENE, &args![camera, sorter, 0u32]);
    fn_004eb510(e, 0);
    let renderer = e.call(RENDERER, &[]).u32();
    let device = fn_004ecaf0(e, renderer);
    e.vcall(
        device,
        DEVICE_SET_STATE_VIRTUAL,
        &args![DEPTH_PASS_RESET_STATE, 0u32, 0u32, 0u32],
    );
    fn_004ecb40(e, 1);
    fn_004ecb10(e, 10);
    fn_004ecb10(e, 11);
    fn_004ecb10(e, 12);
    assign_slot(e, camera_slot, 0);
}

// Translated from 004ecaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The device object at `this + 0x8b8` (of the renderer).
pub fn fn_004ecaf0(e: &mut Engine, this: u32) -> u32 {
    e.mem.u32(this + RENDERER_DEVICE_OFFSET)
}

// Translated from 004ecb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lowers the render-state counter `index` (the words at `0x011ff9d8`) by one
/// when it is not zero.
pub fn fn_004ecb10(e: &mut Engine, index: u32) {
    let counter = RENDER_STATE_COUNTERS.wrapping_add(index.wrapping_mul(4));
    let value = e.global::<u32>(counter);
    if value != 0 {
        e.set_global(counter, value.wrapping_sub(1));
    }
}

// Translated from 004ecb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lowers the word at `0x011ff9fc` by `amount`, then makes the counted state
/// call `00b98070(0, 0)` (cdecl).
pub fn fn_004ecb40(e: &mut Engine, amount: u32) {
    let counter = e.global::<u32>(COUNTER_011FF9FC);
    e.set_global(COUNTER_011FF9FC, counter.wrapping_sub(amount));
    e.call(COUNTED_STATE_34, &args![0u32, 0u32]);
}

// Translated from 004ecb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Draws the water of a group into the depth pass (`RENDER_GROUP_DEPTH`; no
/// Xbox PDB name; the exception-unwinding frame is not translated). Does
/// nothing without `bUseWaterShader` or a `viewer`. Sets the flag at
/// `0x011ff375` and makes a camera on the stack from the viewer (translate,
/// rotation, scale, the `float` at `+0xfc`, frustum), hands it to the render
/// object, and culls with it; every water reference of the group in a cell of
/// kind 6, whose 3D object has the water owner type and is in range of the
/// viewer, gets `stencil_mask` as the stencil mask of its water shader
/// property and its first render pass is drawn immediately. The word after
/// the group is not read.
pub fn fn_004ecb60(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    viewer: u32,
    group: Ptr<PlaceableWaterGroup>,
    _unused_3: u32,
    stencil_mask: u16,
) {
    if !e.call(WATER_SHADER_ENABLED, &[]).bool() || viewer == 0 {
        return;
    }
    e.set_global(WADING_RENDER_ACTIVE_FLAG, 1u8);
    e.call(COUNTED_STATE_A8, &args![0u32, 1u32]);
    e.with_stack(0x114 + 0x10 + 0x10 + CULLING_PROCESS_SIZE, |e, frame| {
        let camera = frame.addr();
        let update_data = camera + 0x114;
        let culling = camera + 0x124;
        e.call(CAMERA_CONSTRUCT, &args![camera]);
        let translate = e.call(NODE_WORLD_TRANSLATE, &args![viewer]).u32();
        e.call(NODE_SET_LOCAL_TRANSLATE, &args![camera, translate]);
        let rotate = e.call(NODE_WORLD_ROTATE, &args![viewer]).u32();
        e.call(NODE_SET_LOCAL_ROTATE, &args![camera, rotate]);
        let scale = e.call(NODE_SCALE_SOURCE, &args![viewer]).f32();
        e.call(NODE_SET_LOCAL_SCALE, &args![camera, scale]);
        let frustum = e.call(CAMERA_FRUSTUM_ADDRESS, &args![camera]).u32();
        let object = e.call(RENDER_GLOBAL_OBJECT, &[]).u32();
        e.call(RENDER_OBJECT_SET_CAMERA_DATA, &args![object, frustum]);
        let far_value = e.call(CAMERA_FLOAT_FC_READ, &args![viewer]).f32();
        e.call(CAMERA_FLOAT_FC_WRITE, &args![camera, far_value]);
        let view_frustum = e.call(CAMERA_PLANES_ADDRESS, &args![viewer]).u32();
        e.call(CAMERA_SET_VIEW_FRUSTUM, &args![camera, view_frustum]);
        e.call(
            UPDATE_DATA_CONSTRUCT,
            &args![update_data, 0.0f32, 0u32, 0u32],
        );
        e.call(NODE_UPDATE, &args![camera, update_data]);
        e.call(CULLING_PROCESS_CONSTRUCT, &args![culling, 0u32]);
        e.call(CULLING_PROCESS_SET_CAMERA, &args![culling, camera]);
        let object = e.call(RENDER_GLOBAL_OBJECT, &[]).u32();
        fn_004e9bb0(e, object, camera);
        let planes = e.call(CAMERA_PLANES_ADDRESS, &args![camera]).u32();
        e.call(CULLING_PROCESS_SET_PLANES, &args![culling, planes]);

        let references = address_of(group, PlaceableWaterGroup::PlaceableWaterList);
        let mut position = pointer_in_slot(e, references);
        while position != 0 {
            let next = e
                .call(LIST_NEXT_POSITION, &args![references, position])
                .u32();
            let slot = e.call(LIST_ITEM_SLOT, &args![references, position]).u32();
            let reference = e.mem.u32(slot);
            draw_depth_reference(e, this, viewer, reference, stencil_mask);
            position = next;
        }

        e.call(CULLING_PROCESS_SET_CAMERA, &args![culling, 0u32]);
        fn_004eced0(e, 1);
        e.set_global(WADING_RENDER_ACTIVE_FLAG, 0u8);
        e.call(CULLING_PROCESS_DESTRUCT, &args![culling]);
        e.call(CAMERA_DESTRUCT, &args![camera]);
    });
}

/// One water reference of `fn_004ecb60`.
fn draw_depth_reference(
    e: &mut Engine,
    this: Ptr<TESWaterSystem>,
    viewer: u32,
    reference: u32,
    stencil_mask: u16,
) {
    if reference == 0 || e.call(REFERENCE_PARENT_CELL, &args![reference]).u32() == 0 {
        return;
    }
    let cell = e.call(REFERENCE_PARENT_CELL, &args![reference]).u32();
    if !e.call(CELL_BYTE_IS_SIX, &args![cell]).bool() {
        return;
    }
    let geometry = fn_004e8030(e, this, reference);
    if geometry == 0 {
        return;
    }
    let owner = e.call(NODE_OWNER, &args![geometry]).u32();
    if owner == 0 || e.call(OWNER_TYPE, &args![owner]).u32() != WATER_OWNER_TYPE {
        return;
    }
    let property = water_shader_property(e, geometry);
    e.set(
        property,
        WaterShaderProperty::iStencilMask,
        stencil_mask as u32,
    );
    if fn_004e62e0(e, this.addr(), reference, viewer)
        && !property.is_null()
        && e.call(PROPERTY_RENDER_PASS, &args![property, geometry])
            .u32()
            != 0
    {
        let pass = e
            .call(PROPERTY_RENDER_PASS, &args![property, geometry])
            .u32();
        let count = e.mem.u16(pass + 4) as u32;
        e.call(
            RENDER_PASS_IMMEDIATELY,
            &args![pass, count, 0u32, 0u32, 0u32],
        );
    }
}

// Translated from 004eced0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lowers the word at `0x011ffa18` by `amount`, then makes the counted state
/// call `00b98380(7, 0)` (cdecl).
pub fn fn_004eced0(e: &mut Engine, amount: u32) {
    let counter = e.global::<u32>(COUNTER_011FFA18);
    e.set_global(COUNTER_011FFA18, counter.wrapping_sub(amount));
    e.call(COUNTED_STATE_A8, &args![7u32, 0u32]);
}

// Translated from 004ecef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the clip plane the reflection and depth passes cut with (no Xbox PDB
/// name). The four floats are a plane; it is normalized and moved by the
/// transpose of the inverse of the renderer's view matrix (rebuilt here from
/// its rotation and the eye position) times its projection matrix, and handed
/// to the render object's device as clip plane 0. The word after the plane
/// and `ECX` are not read.
pub fn fn_004ecef0(
    e: &mut Engine,
    _unused_0: u32,
    plane_a: f32,
    plane_b: f32,
    plane_c: f32,
    plane_d: f32,
    _unused_5: u32,
) {
    // The 4x4 matrices: view at +0, projection at +0x40, their product at
    // +0x80, its inverse at +0xc0; then the plane at +0x100, its transform at
    // +0x110, and the three axes at +0x120.
    e.with_stack(0x150, |e, frame| {
        let frame = frame.addr();
        let view = frame;
        let projection = frame + 0x40;
        let product = frame + 0x80;
        let inverse = frame + 0xc0;
        let plane = frame + 0x100;
        let transformed = frame + 0x110;
        let axes = frame + 0x120;
        let renderer = e.call(RENDERER, &[]).u32();
        let view_source = fn_004ed1f0(e, renderer);
        fn_004ed110(e, view, view_source);
        // The translation row: minus the eye position's dot product with each
        // axis (a column of the rotation).
        for column in 0..3u32 {
            let axis = axes + 0x0c * column;
            let x = e.mem.f32(view + 4 * column);
            let y = e.mem.f32(view + 0x10 + 4 * column);
            let z = e.mem.f32(view + 0x20 + 4 * column);
            let point = e.call(NI_POINT3_CONSTRUCT, &args![axis, x, y, z]).u32();
            let eye = fn_004ed180(e);
            let dot = e.call(NI_POINT3_DOT, &args![eye, point]).f32();
            e.mem.set_f32(view + 0x30 + 4 * column, -dot);
        }
        let renderer = e.call(RENDERER, &[]).u32();
        let projection_source = fn_004ed210(e, renderer);
        fn_004ed110(e, projection, projection_source);
        fn_004ed140(e, view, product, projection);
        e.call(D3DX_MATRIX_INVERSE, &args![inverse, 0u32, product]);
        e.call(D3DX_MATRIX_TRANSPOSE, &args![product, inverse]);
        for (i, value) in [plane_a, plane_b, plane_c, plane_d].into_iter().enumerate() {
            e.mem.set_f32(plane + 4 * i as u32, value);
        }
        e.call(D3DX_PLANE_NORMALIZE, &args![plane, plane]);
        e.call(D3DX_PLANE_TRANSFORM, &args![transformed, plane, product]);
        copy_words(e, transformed, plane, 4);
        let object = e.call(RENDER_GLOBAL_OBJECT, &[]).u32();
        fn_004ed190(e, object, 0, 1, plane);
    });
}

// Translated from 004ed110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a 4x4 matrix (0x40 bytes) from `source` to `this` (`memcpy`).
/// Returns `this`.
pub fn fn_004ed110(e: &mut Engine, this: u32, source: u32) -> u32 {
    e.call(MEMORY_COPY, &args![this, source, 0x40u32]);
    this
}

// Translated from 004ed140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `out = this * other` for 4x4 matrices (`D3DXMatrixMultiply` into a local,
/// then copied to `out`). Returns `out`.
pub fn fn_004ed140(e: &mut Engine, this: u32, out: u32, other: u32) -> u32 {
    e.with_stack(0x40, |e, product| {
        let product = product.addr();
        e.call(D3DX_MATRIX_MULTIPLY, &args![product, this, other]);
        copy_words(e, product, out, 16);
    });
    out
}

// Translated from 004ed180 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the `NiPoint3` at `0x011f474c`.
pub fn fn_004ed180(_e: &mut Engine) -> u32 {
    EYE_POSITION
}

// Translated from 004ed190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the clip planes `first` to `first + count - 1` of the render object's
/// device (`this + 0x288`) to the 16-byte planes at `array`: the device's
/// virtual at `+0xdc` is called as `(device, index, plane)` with `ECX`
/// holding the vtable pointer.
pub fn fn_004ed190(e: &mut Engine, this: u32, first: i32, count: i32, array: u32) {
    let mut i = 0i32;
    while i < count {
        let device = e.mem.u32(this + RENDER_OBJECT_DEVICE_OFFSET);
        let vtable = e.mem.u32(device);
        let target = e
            .mem
            .u32(vtable.wrapping_add(DEVICE_SET_CLIP_PLANE_VIRTUAL));
        let plane = array.wrapping_add((i as u32).wrapping_mul(16));
        e.call(target, &args![vtable, device, first.wrapping_add(i), plane]);
        i += 1;
    }
}

// Translated from 004ed1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the 4x4 matrix at `this + 0x980` (of the renderer).
pub fn fn_004ed1f0(_e: &mut Engine, this: u32) -> u32 {
    this.wrapping_add(RENDERER_VIEW_MATRIX_OFFSET)
}

// Translated from 004ed210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the 4x4 matrix at `this + 0x9c0` (of the renderer).
pub fn fn_004ed210(_e: &mut Engine, this: u32) -> u32 {
    this.wrapping_add(RENDERER_PROJECTION_MATRIX_OFFSET)
}

// Translated from 004ed230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Interpolates between `b` and `a` by the byte at `this + offset + 0xe0`
/// scaled by the double at `0x010231e8`: `byte * scale * (a - b) + b`.
pub fn fn_004ed230(e: &mut Engine, this: u32, offset: u32, a: f32, b: f32) -> f32 {
    let byte = e.mem.u8(this.wrapping_add(offset).wrapping_add(0xe0));
    let scale = e.global::<f64>(COLOR_STEP_SCALE);
    ((byte as f64 * scale) * (a as f64 - b as f64) + b as f64) as f32
}

// Translated from 004ed270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 of the byte at `this + 0xeb` is set.
pub fn fn_004ed270(e: &mut Engine, this: u32) -> bool {
    e.mem.u8(this + 0xeb) & 4 != 0
}

// Translated from 004ed290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem::InitializeGreyTexture` (Xbox PDB): within an allocation
/// scope of `TESWater.cpp` line `0x138a`, makes a rendered texture of kind 8
/// and clears it to `(0.5, 0.5, 0.5, 0.5)` (the renderer's clear colour is
/// set for the draw and restored). Returns the texture. `this` is not read.
pub fn tes_water_system_initialize_grey_texture(
    e: &mut Engine,
    _unused_0: Ptr<TESWaterSystem>,
) -> u32 {
    let mut texture = 0;
    in_allocation_scope(e, SCOPE_LINE_GREY_TEXTURE, |e| {
        e.with_stack(0x20, |e, colors| {
            let original = colors.addr();
            let grey = original + 0x10;
            e.call(
                NI_POINT4_CONSTRUCT,
                &args![original, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
            );
            texture = create_rendered_texture(e, 8);
            let renderer = e.call(RENDERER, &[]).u32();
            e.vcall(renderer, RENDERER_GET_CLEAR_COLOR, &args![original]);
            let renderer = e.call(RENDERER, &[]).u32();
            let half = e.global::<f32>(GREY_VALUE);
            let color = e
                .call(NI_POINT4_CONSTRUCT, &args![grey, half, half, half, half])
                .u32();
            e.vcall(renderer, RENDERER_SET_CLEAR_COLOR, &args![color]);
            let stopped = e.call(RENDERED_TEXTURE_STOP, &args![texture]).u32();
            e.call(RENDER_TARGET_SET, &args![7u32, stopped]);
            e.call(RENDER_TARGET_RESET, &[]);
            restore_clear_color(e, original);
        });
    });
    texture
}

// Translated from 004ed3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `PlaceableWaterGroup` (the engine map has no
/// name; the exception-unwinding frame is not translated). Drops the two
/// accumulators, releases the group in the water system, hands the water
/// form's noise normal map back to the texture manager when the group held
/// its last other reference, empties the objects-in-water and actor lists,
/// detaches the wading geometry from the water root, then destroys the
/// members in reverse order.
pub fn fn_004ed3e0(e: &mut Engine, this: Ptr<PlaceableWaterGroup>) {
    assign_slot(
        e,
        address_of(this, PlaceableWaterGroup::spGroupReflectionSorter),
        0,
    );
    assign_slot(e, address_of(this, PlaceableWaterGroup::spDepthSorter), 0);
    let tes = e.global::<u32>(TES_POINTER);
    let system = e.call(TES_GET_WATER_SYSTEM, &args![tes]).u32();
    fn_004e58a0(e, Ptr::new(system), this, 1, 0);
    let water_type = e.get(this, PlaceableWaterGroup::pWaterType);
    if !water_type.is_null() {
        let form: Ptr<TESWaterForm> = Ptr::new(water_type.addr());
        let map_slot = address_of(form, TESWaterForm::spNoiseNormalMap);
        if pointer_in_slot(e, map_slot) != 0 {
            let texture = pointer_in_slot(e, map_slot);
            // The word at +4 of the texture is its reference count.
            if e.call(NODE_NEXT, &args![texture]).u32() == 2 {
                let texture = pointer_in_slot(e, map_slot);
                let manager = e.call(TEXTURE_MANAGER, &[]).u32();
                e.call(RETURN_RENDERED_TEXTURE, &args![manager, texture]);
                assign_slot(e, map_slot, 0);
            }
        }
    }
    e.call(
        LIST_REMOVE_ALL,
        &args![address_of(this, PlaceableWaterGroup::ObjectInWaterList)],
    );
    e.call(
        LIST_REMOVE_ALL,
        &args![address_of(this, PlaceableWaterGroup::ActorsInWaterList)],
    );
    let geometry_slot = address_of(this, PlaceableWaterGroup::spWadingWaterGeometry);
    if pointer_in_slot(e, geometry_slot) != 0 {
        let root = fn_004e7ff0(e);
        let geometry = pointer_in_slot(e, geometry_slot);
        e.vcall(root, NODE_DETACH_CHILD, &args![geometry]);
        assign_slot(e, geometry_slot, 0);
    }
    for offset in [0xa8, 0xa4, 0x98, 0x94] {
        e.call(POINTER_TEMP_DESTRUCT, &args![this.addr() + offset]);
    }
    for offset in [0x88, 0x7c, 0x70, 0x64] {
        e.call(LIST_DESTRUCT, &args![this.addr() + offset]);
    }
    for offset in [0x58, 0x54] {
        e.call(POINTER_TEMP_DESTRUCT, &args![this.addr() + offset]);
    }
    for offset in [0x48, 0x3c, 0x30, 0x24] {
        e.call(LIST_DESTRUCT, &args![this.addr() + offset]);
    }
}

// Translated from 004ed5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlaceableWaterGroup::PlaceableWaterGroup` (Xbox PDB): constructs the two
/// planes (`+4`, `+0x14`), the lists (`+0x24`, `+0x30`, `+0x3c`, `+0x48`,
/// `+0x64`, `+0x70`, `+0x7c`, `+0x88`) and the null `NiPointer`s (`+0x54`,
/// `+0x58`, `+0x94`, `+0x98`, `+0xa4`, `+0xa8`), clears the sorters and the
/// wading geometry again, and the flags `bGroupAtWorldSpaceWaterHeight`,
/// `bRenderGroupReflections` and `bRenderSilhouetteReflections`. Returns
/// `this`; the other members are not initialized.
pub fn placeable_water_group_placeable_water_group(
    e: &mut Engine,
    this: Ptr<PlaceableWaterGroup>,
) -> Ptr<PlaceableWaterGroup> {
    let base = this.addr();
    e.call(NI_PLANE_DEFAULT_CONSTRUCT, &args![base + 4]);
    e.call(NI_PLANE_DEFAULT_CONSTRUCT, &args![base + 0x14]);
    for offset in [0x24, 0x30, 0x3c, 0x48] {
        e.call(LIST_CONSTRUCT, &args![base + offset]);
    }
    for offset in [0x54, 0x58] {
        e.call(POINTER_TEMP_CONSTRUCT, &args![base + offset, 0u32]);
    }
    for offset in [0x64, 0x70, 0x7c, 0x88] {
        e.call(LIST_CONSTRUCT, &args![base + offset]);
    }
    for offset in [0x94, 0x98, 0xa4, 0xa8] {
        e.call(POINTER_TEMP_CONSTRUCT, &args![base + offset, 0u32]);
    }
    assign_slot(e, base + 0x94, 0);
    assign_slot(e, base + 0x98, 0);
    assign_slot(e, base + 0x58, 0);
    e.set(
        this,
        PlaceableWaterGroup::bRenderSilhouetteReflections,
        false,
    );
    e.set(this, PlaceableWaterGroup::bRenderGroupReflections, false);
    e.set(
        this,
        PlaceableWaterGroup::bGroupAtWorldSpaceWaterHeight,
        false,
    );
    this
}

// Translated from 004ed780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` through `004de290(this, value)` (the float at
/// `setting + 4`); returns `this`.
pub fn fn_004ed780(e: &mut Engine, this: u32, value: f32) -> u32 {
    e.call(SETTING_STORE_FLOAT, &args![this, value]);
    this
}

// Translated from 004ed7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<TESObjectREFR *, TESObjectREFR *>` constructor (the engine
/// map has no name): `004ed960(this, size)`, then the vtable `0x010231f4`.
/// Returns `this`.
pub fn fn_004ed7a0(e: &mut Engine, this: u32, size: u32) -> u32 {
    fn_004ed960(e, this, size);
    e.mem.set_u32(this, REFERENCE_MAP_VTABLE);
    this
}

// Translated from 004ed7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<TESWaterForm *, bool>` constructor: `004eda60(this,
/// size)`, then the vtable `0x01023214`. Returns `this`.
pub fn fn_004ed7d0(e: &mut Engine, this: u32, size: u32) -> u32 {
    fn_004eda60(e, this, size);
    e.mem.set_u32(this, WATER_FORM_MAP_VTABLE);
    this
}

// Translated from 004ed800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<TESObjectREFR *, WadingWaterData *>` constructor:
/// `004edb60(this, size)`, then the vtable
/// `0x01023234`. Returns `this`.
pub fn fn_004ed800(e: &mut Engine, this: u32, size: u32) -> u32 {
    e.call(WADING_MAP_BASE_CONSTRUCT, &args![this, size]);
    e.mem.set_u32(this, WADING_MAP_VTABLE);
    this
}

// Translated from 004ed830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<TESObjectREFR_P_TESObjectREFR_P>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor body `004ed9d0` and, when bit 0 of
/// `flags` is set, frees the object. Returns `this`.
pub fn ni_t_pointer_map_tes_object_refr_p_tes_object_refr_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    e.call(REFERENCE_MAP_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004ed860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<TESWaterForm_P_bool>::_scalar_deleting_destructor_` (Xbox
/// PDB): the same with the destructor body `004edad0`.
pub fn ni_t_pointer_map_tes_water_form_p_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    e.call(WATER_FORM_MAP_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004ed890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<TESObjectREFR_P_WadingWaterData_P>::_scalar_deleting_destructor_`
/// (Xbox PDB): the same with the destructor body `004edbd0`.
pub fn ni_t_pointer_map_tes_object_refr_p_wading_water_data_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    e.call(WADING_MAP_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// The constructor body of the three pointer maps: the vtable, the bucket
/// count at `+4`, the entry count at `+0xc` (zero) and the bucket array at
/// `+8` (`size * 4` bytes from `NiAlloc`, cleared by `memset`).
fn construct_pointer_map_base(e: &mut Engine, this: u32, size: u32, vtable: u32) -> u32 {
    e.mem.set_u32(this, vtable);
    e.mem.set_u32(this + 4, size);
    e.mem.set_u32(this + 0xc, 0);
    let bytes = e.mem.u32(this + 4) << 2;
    let buckets = e.call(NI_ALLOC_ARRAY, &args![bytes]).u32();
    e.mem.set_u32(this + 8, buckets);
    let bytes = e.mem.u32(this + 4) << 2;
    let buckets = e.mem.u32(this + 8);
    e.call(MEMORY_SET, &args![buckets, 0u32, bytes]);
    this
}

// Translated from 004ed960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The base constructor of `NiTPointerMap<TESObjectREFR *, TESObjectREFR *>`
/// (the engine map has no name; the vtable is `0x01023254`): see
/// `construct_pointer_map_base`. Returns `this`.
pub fn fn_004ed960(e: &mut Engine, this: u32, size: u32) -> u32 {
    construct_pointer_map_base(e, this, size, REFERENCE_MAP_BASE_VTABLE)
}

// Translated from 004eda60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same for `NiTPointerMap<TESWaterForm *, bool>` (the vtable is
/// `0x01023274`). Returns `this`.
pub fn fn_004eda60(e: &mut Engine, this: u32, size: u32) -> u32 {
    construct_pointer_map_base(e, this, size, WATER_FORM_MAP_BASE_VTABLE)
}

// Translated from 004edb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The base constructor of `NiTPointerMap<TESObjectREFR *, WadingWaterData *>`
/// (the engine map has no name; the vtable is `0x01023294`): see
/// `construct_pointer_map_base`. Returns `this`.
pub fn fn_004edb60(e: &mut Engine, this: u32, size: u32) -> u32 {
    construct_pointer_map_base(e, this, size, WADING_MAP_BASE_VTABLE)
}

// Translated from 004edc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor with two word arguments (the engine map has no name): the
/// body `004edf70(this, first, second)` (both words are 16-bit values there),
/// then the vtable `0x010232bc`. Returns `this`.
pub fn fn_004edc90(e: &mut Engine, this: u32, first: u32, second: u32) -> u32 {
    e.call(CONSTRUCT_BODY_004EDF70, &args![this, first, second]);
    e.mem.set_u32(this, 0x0102_32bc);
    this
}

// Translated from 004edcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same with the body `004edfe0` and the vtable `0x010232cc`.
pub fn fn_004edcf0(e: &mut Engine, this: u32, first: u32, second: u32) -> u32 {
    e.call(CONSTRUCT_BODY_004EDFE0, &args![this, first, second]);
    e.mem.set_u32(this, 0x0102_32cc);
    this
}

// Translated from 004edd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned_int>,TESObjectREFR *,TESObjectREFR *>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor body `004eda30` and, when bit 0 of
/// `flags` is set, frees the object. Returns `this`.
pub fn ni_t_map_base_tes_object_refr_p_tes_object_refr_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    e.call(REFERENCE_MAP_BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004edd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned_int>,TESWaterForm *,bool>::_scalar_deleting_destructor_`
/// (Xbox PDB): the same with the destructor body `004edb30`.
pub fn ni_t_map_base_tes_water_form_p_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    e.call(WATER_FORM_MAP_BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004edd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned_int>,TESObjectREFR *,WadingWaterData *>::_scalar_deleting_destructor_`
/// (Xbox PDB): the same with the destructor body `004edc30`.
pub fn ni_t_map_base_tes_object_refr_p_wading_water_data_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    e.call(WADING_MAP_BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004e21b0,
            tes_water_system_update_placeable_water(Ptr<TESWaterSystem>, Ptr, bool)
        ),
        entry!(0x004e3260, fn_004e3260() -> u32),
        entry!(0x004e3270, fn_004e3270() -> u32),
        entry!(0x004e3280, fn_004e3280(u32) -> bool),
        entry!(0x004e32a0, fn_004e32a0(u32) -> bool),
        entry!(0x004e32c0, fn_004e32c0(u32) -> bool),
        entry!(0x004e32e0, fn_004e32e0(u32) -> bool),
        entry!(0x004e3300, fn_004e3300(u32) -> bool),
        entry!(
            0x004e3320,
            tes_water_system_update_lod_water(Ptr<TESWaterSystem>, Ptr, bool)
        ),
        entry!(
            0x004e3520,
            tes_water_system_update_water_shader_properties(
                Ptr<TESWaterSystem>,
                Ptr<PlaceableWaterGroup>,
            )
        ),
        entry!(
            0x004e3590,
            tes_water_system_update_water_shader_properties_ov2(
                Ptr<TESWaterSystem>,
                Ptr<PlaceableWaterGroup>,
                u32,
            )
        ),
        entry!(0x004e3c00, fn_004e3c00(Ptr<WaterShaderProperty>)),
        entry!(0x004e3c20, fn_004e3c20(Ptr)),
        entry!(0x004e3c40, fn_004e3c40() -> u8),
        entry!(0x004e3c50, fn_004e3c50(u32) -> bool),
        entry!(0x004e3c70, fn_004e3c70(u32) -> bool),
        entry!(0x004e3ca0, fn_004e3ca0(u32) -> bool),
        entry!(0x004e3cc0, fn_004e3cc0(Ptr<TESWaterForm>) -> f32),
        entry!(0x004e3ce0, fn_004e3ce0(Ptr<TESWaterForm>) -> f32),
        entry!(0x004e3d00, fn_004e3d00(Ptr<TESWaterForm>) -> f32),
        entry!(0x004e3d20, fn_004e3d20(Ptr<TESWaterForm>) -> f32),
        entry!(0x004e3d40, fn_004e3d40(Ptr<TESWaterForm>) -> f32),
        entry!(
            0x004e3d60,
            fn_004e3d60(Ptr<TESWaterSystem>, Ptr<PlaceableWaterGroup>)
        ),
        entry!(0x004e4430, fn_004e4430(f32) -> f32),
        entry!(0x004e4450, fn_004e4450(f32) -> f32),
        entry!(0x004e4470, fn_004e4470(f32) -> f32),
        entry!(0x004e4490, fn_004e4490(f32) -> f32),
        entry!(0x004e44b0, fn_004e44b0(f32) -> f32),
        entry!(0x004e44d0, fn_004e44d0(f32) -> f32),
        entry!(0x004e44f0, bs_wrap(Ptr, f32, f32)),
        entry!(0x004e45a0, fn_004e45a0(Ptr<TESWaterSystem>, u32)),
        entry!(0x004e45c0, fn_004e45c0(Ptr<TESWaterForm>) -> f32),
        entry!(0x004e45e0, fn_004e45e0(Ptr<TESWaterForm>) -> f32),
        entry!(0x004e4600, fn_004e4600(Ptr<TESWaterForm>) -> f32),
        entry!(0x004e4620, fn_004e4620(Ptr<TESWaterForm>) -> f32),
        entry!(0x004e4640, fn_004e4640(Ptr<TESWaterForm>) -> u8),
        entry!(0x004e4660, fn_004e4660(Ptr<TESWaterForm>, u8)),
        entry!(0x004e4680, fn_004e4680() -> u32),
        entry!(0x004e4690, fn_004e4690(u32) -> u8),
        entry!(
            0x004e46b0,
            tes_water_system_add_placeable_water(Ptr<TESWaterSystem>, u32)
        ),
        entry!(
            0x004e4730,
            tes_water_system_add_placeable_water_ov2(Ptr<TESWaterSystem>, u32, u32, u32) -> bool
        ),
        entry!(
            0x004e4c80,
            fn_004e4c80(Ptr<TESWaterSystem>, u32, u32, u32, u32, bool) -> u32
        ),
        entry!(0x004e5140, fn_004e5140(Ptr<TESWaterSystem>, u32, u32, u32)),
        entry!(0x004e52c0, fn_004e52c0(u32, u32) -> u32),
        entry!(0x004e52f0, fn_004e52f0(Ptr<TESWaterSystem>, u32)),
        entry!(
            0x004e5370,
            tes_water_system_remove_placeable_water(Ptr<TESWaterSystem>, u32, u32, u32) -> bool
        ),
        entry!(
            0x004e5640,
            tes_water_system_reset_stencil_bit_refs(Ptr<TESWaterSystem>)
        ),
        entry!(
            0x004e56c0,
            fn_004e56c0(Ptr<TESWaterSystem>, Ptr<PlaceableWaterGroup>)
        ),
        entry!(
            0x004e58a0,
            fn_004e58a0(Ptr<TESWaterSystem>, Ptr<PlaceableWaterGroup>, u32, u32)
        ),
        entry!(
            0x004e59f0,
            fn_004e59f0(Ptr<TESWaterSystem>, u32, u32, f32) -> u32
        ),
        entry!(
            0x004e5c80,
            tes_water_system_add_tes_object_to_water_group(Ptr<TESWaterSystem>, u32, u32, f32)
        ),
        entry!(
            0x004e5df0,
            tes_water_system_remove_tes_object_from_water_group(Ptr<TESWaterSystem>, u32, u32, f32)
        ),
        entry!(
            0x004e5e50,
            tes_water_system_remove_tes_object_from_water_group_ov2(Ptr<TESWaterSystem>, u32, u32)
        ),
        entry!(0x004e5fe0, fn_004e5fe0(Ptr<TESWaterSystem>, u32)),
        entry!(0x004e62e0, fn_004e62e0(u32, u32, u32) -> bool),
        entry!(
            0x004e6370,
            fn_004e6370(Ptr<TESWaterSystem>, bool, bool, bool)
        ),
        entry!(0x004e6540, fn_004e6540(u32) -> u32),
        entry!(0x004e6560, fn_004e6560(u32) -> u16),
        entry!(0x004e6580, fn_004e6580(u32) -> bool),
        entry!(
            0x004e65d0,
            tes_water_system_enable_water_system(Ptr<TESWaterSystem>) -> bool
        ),
        entry!(
            0x004e6620,
            fn_004e6620(Ptr<TESWaterSystem>, bool, bool) -> bool
        ),
        entry!(0x004e69d0, fn_004e69d0() -> u32),
        entry!(0x004e69e0, fn_004e69e0(u32, u32)),
        entry!(0x004e6a00, fn_004e6a00(u32, u32)),
        entry!(0x004e6a20, fn_004e6a20(u32, u32)),
        entry!(0x004e6a40, fn_004e6a40(u32, u32)),
        entry!(0x004e6a60, fn_004e6a60() -> u32),
        entry!(0x004e6a70, fn_004e6a70() -> u32),
        entry!(
            0x004e6a80,
            tes_water_system_update_water_sounds(Ptr<TESWaterSystem>)
        ),
        entry!(0x004e7530, fn_004e7530(u32, f32) -> u32),
        entry!(0x004e7560, fn_004e7560(u32) -> f32),
        entry!(0x004e75d0, fn_004e75d0(u32, u32) -> u32),
        entry!(0x004e7600, fn_004e7600(u32) -> u32),
        entry!(0x004e7620, fn_004e7620(u32) -> u32),
        entry!(0x004e7640, fn_004e7640(u32) -> f32),
        entry!(0x004e7660, fn_004e7660(u32) -> f32),
        entry!(0x004e7680, fn_004e7680(u32, u32) -> u32),
        entry!(
            0x004e7730,
            fn_004e7730(Ptr<TESWaterSystem>, f32, u32, u32, bool, bool) -> u32
        ),
        entry!(
            0x004e7770,
            tes_water_system_create_quad_data(u32, f32, f32, u32, u32, bool, bool) -> u32
        ),
        entry!(0x004e7ff0, fn_004e7ff0() -> u32),
        entry!(0x004e8000, fn_004e8000()),
        entry!(0x004e8030, fn_004e8030(Ptr<TESWaterSystem>, u32) -> u32),
        entry!(
            0x004e8160,
            fn_004e8160(Ptr<TESWaterSystem>, Ptr<PlaceableWaterGroup>, u32, f32) -> u32
        ),
        entry!(
            0x004e83e0,
            fn_004e83e0(u32, Ptr<PlaceableWaterGroup>) -> bool
        ),
        entry!(
            0x004e8510,
            tes_water_system_add_ripple(Ptr<TESWaterSystem>, f32, f32, f32, f32)
        ),
        entry!(0x004e8880, fn_004e8880(u32, u32, u32) -> u32),
        entry!(0x004e88d0, fn_004e88d0(u32, u32) -> u32),
        entry!(0x004e8910, fn_004e8910(u32, u32) -> u32),
        entry!(0x004e8950, fn_004e8950(Ptr<TESWaterSystem>) -> bool),
        entry!(0x004e89e0, fn_004e89e0() -> u8),
        entry!(0x004e8a00, fn_004e8a00(Ptr<TESWaterSystem>)),
        entry!(
            0x004e8e40,
            fn_004e8e40(Ptr<WadingWaterData>) -> Ptr<WadingWaterData>
        ),
        entry!(
            0x004e8ec0,
            tes_water_system_update_water_displacement_simulation(Ptr<TESWaterSystem>)
        ),
        entry!(0x004e9510, fn_004e9510() -> bool),
        entry!(0x004e9530, fn_004e9530(u32) -> u32),
        entry!(0x004e9550, fn_004e9550(Ptr<TESWaterSystem>)),
        entry!(0x004e9bb0, fn_004e9bb0(u32, u32)),
        entry!(0x004e9c10, fn_004e9c10(u32, u32) -> u32),
        entry!(0x004e9c50, fn_004e9c50(u32, u32) -> u32),
        entry!(0x004e9c90, fn_004e9c90(u32, u32, u32, u32, u32, u32, u32)),
        entry!(0x004e9ce0, fn_004e9ce0(u32, u8)),
        entry!(
            0x004e9d40,
            fn_004e9d40(Ptr<TESWaterSystem>, u32, Ptr<PlaceableWaterGroup>, bool)
        ),
        entry!(0x004ea860, fn_004ea860(u32, u8)),
        entry!(0x004ea880, fn_004ea880(u32, u32)),
        entry!(0x004ea8b0, fn_004ea8b0(u32) -> bool),
        entry!(0x004ea8d0, fn_004ea8d0(u32) -> bool),
        entry!(0x004ea8f0, fn_004ea8f0(u32) -> bool),
        entry!(0x004ea910, fn_004ea910(u32) -> bool),
        entry!(0x004ea930, fn_004ea930(u32) -> bool),
        entry!(0x004ea950, fn_004ea950(u32) -> u32),
        entry!(0x004ea970, fn_004ea970() -> u32),
        entry!(0x004ea980, fn_004ea980(u32) -> u32),
        entry!(0x004ea9a0, fn_004ea9a0(u32) -> u32),
        entry!(0x004ea9c0, fn_004ea9c0(u32) -> u32),
        entry!(0x004ea9e0, fn_004ea9e0(u32) -> u32),
        entry!(0x004eaa00, fn_004eaa00(Ptr<TESWaterSystem>, u32)),
        entry!(0x004eaf60, fn_004eaf60(u32) -> u8),
        entry!(0x004eaf80, fn_004eaf80(u32, u32)),
        entry!(
            0x004eb220,
            fn_004eb220(Ptr<TESWaterSystem>, Ptr<PlaceableWaterGroup>)
        ),
        entry!(0x004eb510, fn_004eb510(u32)),
        entry!(0x004eb540, fn_004eb540(Ptr<TESWaterSystem>)),
        entry!(
            0x004eba20,
            image_space_effect_param_image_space_effect_param(u32) -> u32
        ),
        entry!(0x004eba90, fn_004eba90(u32)),
        entry!(
            0x004ebb00,
            image_space_effect_param_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(0x004ebb30, fn_004ebb30(u32)),
        entry!(0x004ebb50, fn_004ebb50(u32)),
        entry!(0x004ebb70, fn_004ebb70(u32) -> u32),
        entry!(0x004ebbc0, fn_004ebbc0(u32, u32) -> u32),
        entry!(0x004ebbe0, fn_004ebbe0(Ptr<TESWaterSystem>)),
        entry!(
            0x004ebef0,
            fn_004ebef0(Ptr<TESWaterSystem>, u32, Ptr<PlaceableWaterGroup>)
        ),
        entry!(0x004ec7b0, fn_004ec7b0() -> u32),
        entry!(0x004ec7c0, fn_004ec7c0(u32, u8)),
        entry!(0x004ec7e0, fn_004ec7e0(u32) -> bool),
        entry!(
            0x004ec800,
            fn_004ec800(Ptr<TESWaterSystem>, Ptr<PlaceableWaterGroup>, u16)
        ),
        entry!(0x004ecaf0, fn_004ecaf0(u32) -> u32),
        entry!(0x004ecb10, fn_004ecb10(u32)),
        entry!(0x004ecb40, fn_004ecb40(u32)),
        entry!(
            0x004ecb60,
            fn_004ecb60(Ptr<TESWaterSystem>, u32, Ptr<PlaceableWaterGroup>, u32, u16)
        ),
        entry!(0x004eced0, fn_004eced0(u32)),
        entry!(0x004ecef0, fn_004ecef0(u32, f32, f32, f32, f32, u32)),
        entry!(0x004ed110, fn_004ed110(u32, u32) -> u32),
        entry!(0x004ed140, fn_004ed140(u32, u32, u32) -> u32),
        entry!(0x004ed180, fn_004ed180() -> u32),
        entry!(0x004ed190, fn_004ed190(u32, i32, i32, u32)),
        entry!(0x004ed1f0, fn_004ed1f0(u32) -> u32),
        entry!(0x004ed210, fn_004ed210(u32) -> u32),
        entry!(0x004ed230, fn_004ed230(u32, u32, f32, f32) -> f32),
        entry!(0x004ed270, fn_004ed270(u32) -> bool),
        entry!(
            0x004ed290,
            tes_water_system_initialize_grey_texture(Ptr<TESWaterSystem>) -> u32
        ),
        entry!(0x004ed3e0, fn_004ed3e0(Ptr<PlaceableWaterGroup>)),
        entry!(
            0x004ed5f0,
            placeable_water_group_placeable_water_group(
                Ptr<PlaceableWaterGroup>,
            ) -> Ptr<PlaceableWaterGroup>
        ),
        entry!(0x004ed780, fn_004ed780(u32, f32) -> u32),
        entry!(0x004ed7a0, fn_004ed7a0(u32, u32) -> u32),
        entry!(0x004ed7d0, fn_004ed7d0(u32, u32) -> u32),
        entry!(0x004ed800, fn_004ed800(u32, u32) -> u32),
        entry!(
            0x004ed830,
            ni_t_pointer_map_tes_object_refr_p_tes_object_refr_p_scalar_deleting_destructor(
                u32, u32
            ) -> u32
        ),
        entry!(
            0x004ed860,
            ni_t_pointer_map_tes_water_form_p_bool_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(
            0x004ed890,
            ni_t_pointer_map_tes_object_refr_p_wading_water_data_p_scalar_deleting_destructor(
                u32, u32
            ) -> u32
        ),
        entry!(0x004ed960, fn_004ed960(u32, u32) -> u32),
        entry!(0x004eda60, fn_004eda60(u32, u32) -> u32),
        entry!(0x004edb60, fn_004edb60(u32, u32) -> u32),
        entry!(0x004edc90, fn_004edc90(u32, u32, u32) -> u32),
        entry!(0x004edcf0, fn_004edcf0(u32, u32, u32) -> u32),
        entry!(
            0x004edd20,
            ni_t_map_base_tes_object_refr_p_tes_object_refr_p_scalar_deleting_destructor(
                u32, u32
            ) -> u32
        ),
        entry!(
            0x004edd50,
            ni_t_map_base_tes_water_form_p_bool_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(
            0x004edd80,
            ni_t_map_base_tes_object_refr_p_wading_water_data_p_scalar_deleting_destructor(
                u32, u32
            ) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Vtables of the test objects (each is mapped at its own address).
    const REFERENCE_VTABLE: u32 = 0x0200_0000;
    const BASE_FORM_VTABLE: u32 = 0x0200_1000;
    const NODE_VTABLE: u32 = 0x0200_2000;
    const WATER_FORM_VTABLE: u32 = 0x0200_3000;
    const TEXTURE_VTABLE: u32 = 0x0200_4000;
    const RENDERER_VTABLE: u32 = 0x0200_5000;
    const PROPERTY_VTABLE: u32 = 0x0200_6000;
    /// Addresses of the doubles the vtables point at.
    const REFERENCE_GET_3D_DOUBLE: u32 = 0x0300_0001;
    const BASE_FORM_WATER_FORM_DOUBLE: u32 = 0x0300_0002;
    const NODE_VIRTUAL_DOUBLE: u32 = 0x0300_0003;
    const WATER_FORM_NAME_DOUBLE: u32 = 0x0300_0004;
    const TEXTURE_WIDTH_DOUBLE: u32 = 0x0300_0005;
    const TEXTURE_HEIGHT_DOUBLE: u32 = 0x0300_0006;
    const RENDERER_GET_COLOR_DOUBLE: u32 = 0x0300_0007;
    const RENDERER_SET_COLOR_DOUBLE: u32 = 0x0300_0008;
    const PROPERTY_PASS_COUNT_DOUBLE: u32 = 0x0300_0009;

    // Field offsets of the test objects (the doubles read them).
    /// Reference: the 3D node `Get3D` returns, the water geometry
    /// `004e8030` returns, and the answer of the range test.
    const REFERENCE_NODE: u32 = 0x90;
    const REFERENCE_WATER_NODE: u32 = 0x94;
    const REFERENCE_IN_RANGE: u32 = 0x98;
    /// Base form: the flags `FORM_FLAG_TEST` reads, the water form.
    const BASE_FORM_FLAGS: u32 = 0x10;
    const BASE_FORM_WATER_FORM: u32 = 0x14;
    /// Water geometry node: the property `GetProperty` returns, and the
    /// answer of its virtual at +0x18.
    const NODE_PROPERTY: u32 = 0xa0;
    const NODE_VIRTUAL_RESULT: u32 = 0xb0;
    /// Property: the answer of its virtual at +0xa4.
    const PROPERTY_PASS_COUNT: u32 = 0x1f0;

    /// An object of `size` bytes with `vtable` at +0.
    fn object(e: &mut Engine, vtable: u32, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A vtable at `address` with the given (byte offset, target) slots.
    fn vtable(e: &mut Engine, address: u32, slots: &[(u32, u32)]) {
        let length = slots
            .iter()
            .map(|(offset, _)| offset / 4 + 1)
            .max()
            .unwrap_or(1);
        let mut words = vec![0u32; length as usize];
        for (offset, target) in slots {
            words[(offset / 4) as usize] = *target;
        }
        e.put_vtable(address, &words);
    }

    /// An `NiTPointerList`: head, tail, count, and nodes `{next, prev, item}`.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let header = e.mem.alloc(0x0c);
        let mut previous = 0;
        for item in items {
            let node = e.mem.alloc(12);
            e.mem.set_u32(node + 4, previous);
            e.mem.set_u32(node + 8, *item);
            if previous == 0 {
                e.mem.set_u32(header, node);
            } else {
                e.mem.set_u32(previous, node);
            }
            e.mem.set_u32(header + 4, node);
            previous = node;
        }
        e.mem.set_u32(header + 8, items.len() as u32);
        header
    }

    /// Fills the list embedded at `address` (a `NiTPointerList` field).
    fn fill_list(e: &mut Engine, address: u32, items: &[u32]) {
        let built = list(e, items);
        for i in 0..3 {
            let word = e.mem.u32(built + 4 * i);
            e.mem.set_u32(address + 4 * i, word);
        }
    }

    /// Test doubles for the callees every function here shares: the
    /// `NiPointer`/list helpers, the INI settings (the byte at setting + 4),
    /// the `NiPointer` assignments, the form flag tests and the object
    /// virtuals, plus the pages holding the exe globals.
    fn water_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0101_7000,
            0x0101_e000,
            0x0102_3000,
            0x011a_9000,
            0x011a_d000,
            0x011c_7000,
            0x011d_e000,
            0x011f_9000,
            0x011f_f000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(BYTE_RANGE, 255.0f64);
        e.set_global(ONE_HUNDRED, 100.0f64);
        e.set_global(DEGREES_TO_RADIANS, 0.017453292f32 as f64);

        // `*this` (NiPointer / list head), list next / item, emptiness, count.
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(LIST_NEXT_POSITION, |e, a| ret(e.mem.u32(a[1])));
        e.register(LIST_ITEM_SLOT, |_, a| ret(a[1] + 8));
        e.register(LIST_IS_EMPTY, |e, a| ret((e.mem.u32(a[0] + 8) == 0) as u32));
        e.register(LIST_COUNT, |e, a| ret(e.mem.u32(a[0] + 8)));
        // Settings: the value byte at setting + 4.
        e.register(SETTING_VALUE, |_, a| ret(a[0] + 4));
        // NiPointer assignments.
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(NI_POINTER_ASSIGN_FROM, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            ret(a[0])
        });
        // The TES object: `pInteriorCell` at +0x34.
        let tes = e.mem.alloc(0xc4);
        e.set_global(TES_POINTER, tes);
        e.register(TES_GET_INTERIOR_CELL, |e, a| ret(e.mem.u32(a[0] + 0x34)));
        // The reference: base form at +0x20, parent cell at +0x40; the cell's
        // byte at +0x26; the 3D node's culled flag at +0x30.
        e.register(REFERENCE_BASE_FORM, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(REFERENCE_PARENT_CELL, |e, a| ret(e.mem.u32(a[0] + 0x40)));
        e.register(CELL_BYTE_IS_SIX, |e, a| {
            ret((e.mem.u8(a[0] + 0x26) == 6) as u32)
        });
        e.register(NODE_IS_CULLED, |e, a| ret(e.mem.u32(a[0] + 0x30) & 1));
        e.register(REFERENCE_EXTRA_DATA_LIST, |_, a| ret(a[0] + 0x44));
        e.register(EXTRA_DATA_LIST_GET_REFLECTED_REFS, |e, a| {
            ret(e.mem.u32(a[0]))
        });
        e.register(WORDS_ARE_ZERO, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        // Form flags: the word at base form + 0x10.
        e.register(FORM_FLAG_TEST, |e, a| {
            ret((e.mem.u32(a[0] + BASE_FORM_FLAGS) & a[1] != 0) as u32)
        });
        e.register(FORM_FLAG_TEST_40000000, |e, a| {
            ret((e.mem.u32(a[0] + BASE_FORM_FLAGS) & 0x4000_0000 != 0) as u32)
        });
        // The water reference's 3D object and its owner / property.
        e.register(WATER_REFERENCE_3D, |e, a| {
            ret(e.mem.u32(a[1] + REFERENCE_WATER_NODE))
        });
        e.register(NODE_OWNER, |e, a| ret(e.mem.u32(a[0] + 0xc0)));
        e.register(OWNER_TYPE, |e, a| ret(e.mem.u32(a[0] + 0x68)));
        e.register(NODE_GET_PROPERTY, |e, a| {
            assert_eq!(a[1], 3, "the water property is type 3");
            ret(e.mem.u32(a[0] + NODE_PROPERTY))
        });
        e.register(NODE_WORLD_TRANSLATE, |_, a| ret(a[0] + 0x8c));
        e.register(NODE_WORLD_ROTATE, |_, a| ret(a[0] + 0x68));
        e.register(REFERENCE_IS_IN_RANGE, |e, a| {
            ret(e.mem.u8(a[1] + REFERENCE_IN_RANGE) as u32)
        });
        e.register(WATER_SKIPPED_FLAG, |e, _| ret(e.mem.u8(0x011c_7a59) as u32));
        e.register(NI_POINT4_CONSTRUCT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            ret(a[0])
        });
        // The vtable slots.
        e.register(REFERENCE_GET_3D_DOUBLE, |e, a| {
            ret(e.mem.u32(a[0] + REFERENCE_NODE))
        });
        e.register(BASE_FORM_WATER_FORM_DOUBLE, |e, a| {
            ret(e.mem.u32(a[0] + BASE_FORM_WATER_FORM))
        });
        e.register(NODE_VIRTUAL_DOUBLE, |e, a| {
            ret(e.mem.u32(a[0] + NODE_VIRTUAL_RESULT))
        });
        e.register(PROPERTY_PASS_COUNT_DOUBLE, |e, a| {
            ret(e.mem.u32(a[0] + PROPERTY_PASS_COUNT))
        });
        e.register(WATER_FORM_NAME_DOUBLE, |_, _| ret(0x0555_0000));
        e.register(TEXTURE_WIDTH_DOUBLE, |_, _| ret(512));
        e.register(TEXTURE_HEIGHT_DOUBLE, |_, _| ret(256));
        e.register(RENDERER_GET_COLOR_DOUBLE, |e, a| {
            for (i, value) in [0.1f32, 0.2, 0.3, 0.4].into_iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, value);
            }
            Ret::default()
        });
        e.register(RENDERER_SET_COLOR_DOUBLE, |_, _| Ret::default());
        vtable(
            &mut e,
            REFERENCE_VTABLE,
            &[(REFERENCE_GET_3D, REFERENCE_GET_3D_DOUBLE)],
        );
        vtable(
            &mut e,
            BASE_FORM_VTABLE,
            &[(BASE_FORM_GET_WATER_FORM, BASE_FORM_WATER_FORM_DOUBLE)],
        );
        vtable(
            &mut e,
            NODE_VTABLE,
            &[(NODE_PROPERTIES_VIRTUAL, NODE_VIRTUAL_DOUBLE)],
        );
        vtable(
            &mut e,
            PROPERTY_VTABLE,
            &[(PROPERTY_PASS_COUNT_VIRTUAL, PROPERTY_PASS_COUNT_DOUBLE)],
        );
        vtable(
            &mut e,
            WATER_FORM_VTABLE,
            &[(WATER_FORM_GET_NAME, WATER_FORM_NAME_DOUBLE)],
        );
        vtable(
            &mut e,
            TEXTURE_VTABLE,
            &[
                (TEXTURE_WIDTH_VIRTUAL, TEXTURE_WIDTH_DOUBLE),
                (TEXTURE_HEIGHT_VIRTUAL, TEXTURE_HEIGHT_DOUBLE),
            ],
        );
        vtable(
            &mut e,
            RENDERER_VTABLE,
            &[
                (RENDERER_GET_CLEAR_COLOR, RENDERER_GET_COLOR_DOUBLE),
                (RENDERER_SET_CLEAR_COLOR, RENDERER_SET_COLOR_DOUBLE),
            ],
        );
        e
    }

    /// The argument words of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn start_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// Makes `setting` read as `value`.
    fn set_setting(e: &mut Engine, setting: u32, value: bool) {
        e.set_global(setting + 4, value as u8);
    }

    // --- the small accessors ------------------------------------------------

    #[test]
    fn address_returning_accessors() {
        let mut e = water_engine();
        assert_eq!(e.call(0x004e_3260, &[]).u32(), 0x011c_a13c);
        e.set_global(IMAGE_SPACE_MANAGER, 0x1234_5678u32);
        assert_eq!(e.call(0x004e_3270, &[]).u32(), 0x1234_5678);
        e.set_global(WATER_FLAG_011C7A58, 7u8);
        assert_eq!(e.call(0x004e_3c40, &[]).u8(), 7);
        e.set_global(FLUSH_OBJECT, 0x0abc_0000u32);
        assert_eq!(e.call(0x004e_4680, &[]).u32(), 0x0abc_0000);
    }

    /// A form flag test: the function must pass `mask` to `00452460`, and
    /// return its result (or the negation, for `fn_004e3c70`).
    fn check_flag_function(address: u32, mask: u32, negated: bool) {
        let mut e = water_engine();
        let form = e.mem.alloc(0x40);
        e.mem.set_u32(form + BASE_FORM_FLAGS, mask);
        start_log(&mut e);
        let with_bit = e.call(address, &args![form]).bool();
        assert_eq!(with_bit, !negated, "{address:08x} with the bit set");
        assert_eq!(calls(&e, FORM_FLAG_TEST), vec![vec![form, mask]]);
        e.mem.set_u32(form + BASE_FORM_FLAGS, !mask);
        let without_bit = e.call(address, &args![form]).bool();
        assert_eq!(without_bit, negated, "{address:08x} without the bit");
    }

    #[test]
    fn form_flag_test_200() {
        check_flag_function(0x004e_3280, 0x200, false);
    }

    #[test]
    fn form_flag_test_800() {
        check_flag_function(0x004e_32a0, 0x800, false);
    }

    #[test]
    fn form_flag_test_8000000() {
        check_flag_function(0x004e_32c0, 0x0800_0000, false);
    }

    #[test]
    fn form_flag_test_10000000() {
        check_flag_function(0x004e_32e0, 0x1000_0000, false);
    }

    #[test]
    fn form_flag_test_40000() {
        check_flag_function(0x004e_3300, 0x4_0000, false);
    }

    #[test]
    fn form_flag_test_1() {
        check_flag_function(0x004e_3c50, 1, false);
    }

    #[test]
    fn form_flag_test_80000000_is_negated() {
        check_flag_function(0x004e_3c70, 0x8000_0000, true);
    }

    #[test]
    fn form_flag_test_20000000() {
        check_flag_function(0x004e_3ca0, 0x2000_0000, false);
    }

    #[test]
    fn render_pass_list_field_is_cleared() {
        let mut e = water_engine();
        let list = e.mem.alloc(0x20);
        e.mem.set_u32(list + 0x10, 99);
        e.call(0x004e_3c20, &args![list]);
        assert_eq!(e.mem.u32(list + 0x10), 0);
    }

    #[test]
    fn render_pass_list_reset_only_when_the_property_has_one() {
        let mut e = water_engine();
        let property = e.mem.alloc(0x150);
        let list = e.mem.alloc(0x20);
        e.mem.set_u32(list + 0x10, 99);
        e.call(0x004e_3c00, &args![property]);
        assert_eq!(e.mem.u32(list + 0x10), 99);
        e.mem.set_u32(property + 0x3c, list);
        e.call(0x004e_3c00, &args![property]);
        assert_eq!(e.mem.u32(list + 0x10), 0);
    }

    /// A `TESWaterForm` float getter reads its offset.
    fn check_form_getter(address: u32, offset: u32) {
        let mut e = water_engine();
        let form = e.mem.alloc(0x1a4);
        e.mem.set_f32(form + offset, 2.5);
        assert_eq!(e.call(address, &args![form]).f32(), 2.5, "{address:08x}");
    }

    #[test]
    fn water_form_fresnel_amount_getter() {
        check_form_getter(0x004e_3cc0, 0x9c);
    }

    #[test]
    fn water_form_fog_amount_getter() {
        check_form_getter(0x004e_3ce0, 0x108);
    }

    #[test]
    fn water_form_underwater_fog_near_getter() {
        check_form_getter(0x004e_3d00, 0x114);
    }

    #[test]
    fn water_form_light_radius_getter() {
        check_form_getter(0x004e_3d20, 0x128);
    }

    #[test]
    fn water_form_light_brightness_getter() {
        check_form_getter(0x004e_3d40, 0x12c);
    }

    #[test]
    fn water_form_height_uv_scale_getter() {
        check_form_getter(0x004e_45c0, 0x130);
    }

    #[test]
    fn water_form_wind_direction_0_getter() {
        check_form_getter(0x004e_45e0, 0xe8);
    }

    #[test]
    fn water_form_wind_direction_2_getter() {
        check_form_getter(0x004e_4600, 0xf0);
    }

    #[test]
    fn water_form_wind_speed_1_getter() {
        check_form_getter(0x004e_4620, 0xf8);
    }

    #[test]
    fn noise_texture_reset_flag_round_trips_its_byte() {
        let mut e = water_engine();
        let form = e.mem.alloc(0x1a4);
        assert_eq!(e.call(0x004e_4640, &args![form]).u8(), 0);
        e.call(0x004e_4660, &args![form, 1u8]);
        assert_eq!(e.mem.u8(form + 0x190), 1);
        assert_eq!(e.call(0x004e_4640, &args![form]).u8(), 1);
        e.call(0x004e_4660, &args![form, 0u8]);
        assert_eq!(e.mem.u8(form + 0x190), 0);
    }

    #[test]
    fn flush_object_flag_is_the_byte_at_2b8() {
        let mut e = water_engine();
        let object = e.mem.alloc(0x2c0);
        e.mem.set_u8(object + 0x2b8, 9);
        assert_eq!(e.call(0x004e_4690, &args![object]).u8(), 9);
    }

    #[test]
    fn shared_noise_texture_is_copied_into_the_slot() {
        let mut e = water_engine();
        let system = e.mem.alloc(0xa0);
        e.mem.set_u32(system + 0x1c, 0x5555);
        let slot = e.mem.alloc(4);
        start_log(&mut e);
        e.call(0x004e_45a0, &args![system, slot]);
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN_FROM),
            vec![vec![slot, system + 0x1c]]
        );
        assert_eq!(e.mem.u32(slot), 0x5555);
    }

    // --- the CRT wrappers ---------------------------------------------------

    #[test]
    fn ceil_wrappers_round_up_through_a_double() {
        let mut e = water_engine();
        e.register(CRT_CEIL, |_, a| f64::take(a, &mut 0).ceil().into_ret());
        assert_eq!(e.call(0x004e_4450, &args![2.25f32]).f32(), 3.0);
        assert_eq!(e.call(0x004e_4430, &args![-0.5f32]).f32(), 0.0);
        assert_eq!(e.call(0x004e_4430, &args![7.0f32]).f32(), 7.0);
    }

    #[test]
    fn cos_wrappers_pass_a_double_and_return_a_float() {
        let mut e = water_engine();
        e.register(CRT_COS, |_, a| f64::take(a, &mut 0).cos().into_ret());
        start_log(&mut e);
        let direct = e.call(0x004e_4490, &args![1.5f32]).f32();
        let wrapped = e.call(0x004e_4470, &args![1.5f32]).f32();
        assert_eq!(direct, (1.5f64.cos()) as f32);
        assert_eq!(wrapped, direct);
        let logged = calls(&e, CRT_COS);
        assert_eq!(logged.len(), 2);
        assert_eq!(f64::take(&logged[0], &mut 0), 1.5);
    }

    #[test]
    fn sin_wrappers_pass_a_double_and_return_a_float() {
        let mut e = water_engine();
        e.register(CRT_SIN, |_, a| f64::take(a, &mut 0).sin().into_ret());
        start_log(&mut e);
        let direct = e.call(0x004e_44d0, &args![0.25f32]).f32();
        let wrapped = e.call(0x004e_44b0, &args![0.25f32]).f32();
        assert_eq!(direct, (0.25f64.sin()) as f32);
        assert_eq!(wrapped, direct);
        assert_eq!(calls(&e, CRT_SIN).len(), 2);
    }

    // --- BSWrap -------------------------------------------------------------

    /// `BSWrap(&value, low, high)` with `004b1520` standing in as `fmod`.
    fn wrap(value: f32, low: f32, high: f32) -> f32 {
        let mut e = water_engine();
        e.register(FLOAT_REMAINDER, |_, a| {
            ((f32::from_bits(a[0]) as f64) % (f32::from_bits(a[1]) as f64)).into_ret()
        });
        let slot = e.mem.alloc(4);
        e.mem.set_f32(slot, value);
        e.call(0x004e_44f0, &args![slot, low, high]);
        e.mem.f32(slot)
    }

    #[test]
    fn wrap_leaves_values_inside_the_range() {
        assert_eq!(wrap(0.25, 0.0, 1.0), 0.25);
        assert_eq!(wrap(1.0, 0.0, 1.0), 1.0);
        assert_eq!(wrap(0.0, 0.0, 1.0), 0.0);
    }

    #[test]
    fn wrap_brings_values_above_the_range_down() {
        assert_eq!(wrap(1.5, 0.0, 1.0), 0.5);
        assert_eq!(wrap(7.25, 2.0, 4.0), 3.25);
    }

    #[test]
    fn wrap_brings_values_below_the_range_up_from_the_top() {
        // fmod(-0.25, 1) = -0.25, plus the high end 1.0.
        assert_eq!(wrap(-0.25, 0.0, 1.0), 0.75);
    }

    #[test]
    fn wrap_does_nothing_when_the_range_is_reversed() {
        assert_eq!(wrap(5.0, 2.0, 1.0), 5.0);
    }

    // --- AddPlaceableWater --------------------------------------------------

    #[test]
    fn add_placeable_water_passes_the_node_position_and_rotation() {
        let mut e = water_engine();
        let system = e.mem.alloc(0xa0);
        let node = e.mem.alloc(0x100);
        let reference = object(&mut e, REFERENCE_VTABLE, 0x100);
        e.mem.set_u32(reference + REFERENCE_NODE, node);
        for i in 0..3 {
            e.mem.set_f32(node + 0x8c + 4 * i, 10.0 + i as f32);
        }
        for i in 0..9 {
            e.mem.set_f32(node + 0x68 + 4 * i, i as f32);
        }
        e.register_double(ADD_PLACEABLE_WATER_OV2, |e, a| {
            // The position and rotation are passed by address, in locals.
            assert_eq!(a[3], a[2] + 0x0c);
            let mut words = Vec::new();
            for i in 0..12 {
                words.push(e.mem.f32(a[2] + 4 * i));
            }
            e.mem.set_f32(a[1] + 0x40, words[0]);
            e.mem.set_f32(a[1] + 0x44, words[2]);
            e.mem.set_f32(a[1] + 0x48, words[3]);
            e.mem.set_f32(a[1] + 0x4c, words[11]);
            Ret::default()
        });
        start_log(&mut e);
        e.call(0x004e_46b0, &args![system, reference]);
        let ov2 = calls(&e, ADD_PLACEABLE_WATER_OV2);
        assert_eq!(ov2.len(), 1);
        assert_eq!(&ov2[0][..2], &[system, reference]);
        assert_eq!(e.mem.f32(reference + 0x40), 10.0);
        assert_eq!(e.mem.f32(reference + 0x44), 12.0);
        assert_eq!(e.mem.f32(reference + 0x48), 0.0);
        assert_eq!(e.mem.f32(reference + 0x4c), 8.0);
    }

    // --- a scene for the group updates --------------------------------------

    /// Pass functions `UpdatePlaceableWater` schedules and the renderer calls
    /// of the depth pass: doubles that do nothing (the tests read the log).
    const QUIET_CALLS: [u32; 26] = [
        SETUP_WORLD_REFLECTIONS,
        SETUP_SKY_REFLECTIONS,
        FINISH_WORLD_REFLECTIONS,
        FINISH_SKY_REFLECTIONS,
        FINISH_GROUP,
        FINISH_GROUP_INTERIOR,
        RELEASE_GROUP,
        SETUP_GROUP_REFLECTIONS,
        SETUP_GROUP_DEPTH,
        MAP_REMOVE_ALL,
        LIST_REMOVE_ALL,
        RENDER_GROUP_DEPTH,
        AFTER_GROUP_DEPTH,
        VIEWER_POSITION_UPDATE,
        RENDER_TARGET_SET,
        RENDER_TARGET_RESET,
        BATCH_RENDERER_END_PASS,
        SET_STENCIL_STATE,
        SHADER_PARAM_CONSTRUCT,
        SHADER_PARAM_INIT_CONSTANT_MAP,
        SHADER_PARAM_SET_PIXEL_CONSTANT,
        SHADER_PARAM_DESTRUCT,
        IMAGE_SPACE_RENDER_EFFECT,
        RETURN_RENDERED_TEXTURE,
        LOG_MESSAGE,
        FORMATTED_PRINT,
    ];

    /// The depth map and the wading-water height map the tests start with.
    const DEPTH_TEXTURE: u32 = 0x0d00_0001;
    const WADING_TEXTURE: u32 = 0x0d00_0002;
    const FORM_NOISE_TEXTURE: u32 = 0x0d00_0003;

    struct Scene {
        e: Engine,
        this: Ptr<TESWaterSystem>,
        group: Ptr<PlaceableWaterGroup>,
        reference: u32,
        base_form: u32,
        water_node: u32,
        property: Ptr<WaterShaderProperty>,
        viewer: u32,
        water_form: u32,
        renderer: u32,
        texture_manager: u32,
    }

    /// An exterior scene with one group of one reference that is in range,
    /// whose base form allows reflections, refractions and depth, with the
    /// settings for reflections, depth and refractions on.
    fn scene() -> Scene {
        let mut e = water_engine();
        for address in QUIET_CALLS {
            e.register(address, |_, _| Ret::default());
        }
        let renderer = object(&mut e, RENDERER_VTABLE, 0x40);
        e.set_global(0x011f_9508u32, renderer);
        let texture_manager = e.mem.alloc(0x40);
        e.set_global(0x011f_91a8u32, texture_manager);
        e.register(RENDERER, |e, _| ret(e.mem.u32(0x011f_9508)));
        e.register(TEXTURE_MANAGER, |e, _| ret(e.mem.u32(0x011f_91a8)));
        e.register(RENDERED_TEXTURE_STOP, |_, _| ret(0x7777));
        e.register(PLANE_CONSTANT, |e, a| e.mem.f32(a[0] + 0xc).into_ret());
        e.set_global(IMAGE_SPACE_MANAGER, 0x0ee0_0000u32);
        e.set_global(DEPTH_MAP, DEPTH_TEXTURE);
        e.set_global(WADING_WATER_HEIGHT_MAP, WADING_TEXTURE);
        for (i, value) in [0.5f32, 0.25, 0.125, 1.0].into_iter().enumerate() {
            e.set_global(DEPTH_CLEAR_COLOR + 4 * i as u32, value);
        }
        for setting in [
            SETTING_USE_WATER_REFLECTIONS,
            SETTING_USE_WATER_DEPTH,
            SETTING_USE_WATER_REFRACTIONS,
        ] {
            set_setting(&mut e, setting, true);
        }

        let this = e.mem.alloc(0xa0);
        let water_form = object(&mut e, WATER_FORM_VTABLE, 0x1a4);
        e.mem.set_u32(water_form + 0x30, FORM_NOISE_TEXTURE);
        let group = e.mem.alloc(0xb0);
        e.mem.set_u32(group, water_form);
        e.mem.set_f32(group + 0x04 + 0x0c, 5.5);
        e.mem.set_u32(group + 0xa8, 0x0ca0_0001);
        e.mem.set_u32(group + 0xac, 0x0007_0003);
        e.mem.set_u8(group + 0x5c, 1);

        let base_form = object(&mut e, BASE_FORM_VTABLE, 0x40);
        e.mem.set_u32(
            base_form + BASE_FORM_FLAGS,
            0x0800_0000 | 0x1000_0000 | 0x200,
        );
        let node = e.mem.alloc(0x100);
        let owner = e.mem.alloc(0x80);
        e.mem.set_u32(owner + 0x68, WATER_OWNER_TYPE);
        let property = object(&mut e, PROPERTY_VTABLE, 0x200);
        let water_node = object(&mut e, NODE_VTABLE, 0x200);
        e.mem.set_u32(water_node + 0xc0, owner);
        e.mem.set_u32(water_node + NODE_PROPERTY, property);
        let reference = object(&mut e, REFERENCE_VTABLE, 0x100);
        e.mem.set_u32(reference + 0x20, base_form);
        e.mem.set_u32(reference + REFERENCE_NODE, node);
        e.mem.set_u32(reference + REFERENCE_WATER_NODE, water_node);
        e.mem.set_u8(reference + REFERENCE_IN_RANGE, 1);
        // The property has all its texture slots filled.
        for offset in [0x13c, 0x140, 0x144, 0x138] {
            e.mem.set_u32(property + offset, 0x0b00_0000 + offset);
        }

        e.map(0x011c_a000, 0x1000);
        let viewer = e.mem.alloc(0x100);
        for (i, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(viewer + 0x8c + 4 * i as u32, value);
        }
        fill_list(&mut e, this + 0x3c, &[group]);
        fill_list(&mut e, group + 0x24, &[reference]);
        Scene {
            e,
            this: Ptr::new(this),
            group: Ptr::new(group),
            reference,
            base_form,
            water_node,
            property: Ptr::new(property),
            viewer,
            water_form,
            renderer,
            texture_manager,
        }
    }

    impl Scene {
        fn update(&mut self, force_update: bool) {
            start_log(&mut self.e);
            let (this, viewer) = (self.this, self.viewer);
            self.e.call(0x004e_21b0, &args![this, viewer, force_update]);
        }

        fn group_flag(&self, offset: u32) -> bool {
            self.e.mem.u8(self.group.addr() + offset) != 0
        }

        fn calls(&self, address: u32) -> Vec<Vec<u32>> {
            calls(&self.e, address)
        }
    }

    // --- UpdatePlaceableWater -----------------------------------------------

    #[test]
    fn a_reference_in_range_switches_its_group_on_and_sets_up_world_reflections() {
        let mut s = scene();
        s.update(false);
        let (this, group, viewer) = (s.this.addr(), s.group.addr(), s.viewer);
        assert!(s.group_flag(0x5d), "bRenderGroup");
        assert_eq!(s.e.global::<u32>(ACTIVE_WATER_GROUPS), 1);
        // The group sits at the world-space water height: world reflections.
        assert_eq!(s.e.global::<u8>(RENDER_WORLD_REFLECTIONS), 1);
        assert_eq!(s.e.global::<u8>(RENDER_SKY_REFLECTIONS), 0);
        assert_eq!(s.calls(SETUP_WORLD_REFLECTIONS), vec![vec![this, viewer]]);
        assert!(s.calls(SETUP_SKY_REFLECTIONS).is_empty());
        assert_eq!(s.e.global::<u32>(WORLD_REFLECTION_THREAD_STAGE), 0);
        assert_eq!(s.e.mem.u32(this), 1, "one stage was used");
        assert_eq!(s.calls(FINISH_WORLD_REFLECTIONS), vec![vec![this]]);
        assert_eq!(s.calls(FINISH_GROUP), vec![vec![this, group]]);
        assert!(s.calls(RELEASE_GROUP).is_empty());
        assert_eq!(
            s.calls(MAP_REMOVE_ALL),
            vec![vec![this + 0x4c], vec![this + 0x5c], vec![this + 0x6c]]
        );
        assert_eq!(
            s.calls(LIST_REMOVE_ALL),
            vec![vec![group + 0x7c], vec![group + 0x88]]
        );
        // No depth was asked for; the viewer's position went to 004e20c0.
        assert!(s.calls(RENDER_GROUP_DEPTH).is_empty());
        assert_eq!(
            s.calls(VIEWER_POSITION_UPDATE),
            vec![vec![
                0x1e,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                0
            ]]
        );
        // The reference's property: bReflections etc. are off, so every
        // texture slot was cleared.
        for offset in [0x13c, 0x140, 0x144, 0x138] {
            assert_eq!(
                s.e.mem.u32(s.property.addr() + offset),
                0,
                "slot {offset:x}"
            );
        }
    }

    #[test]
    fn a_group_not_at_the_world_height_asks_for_sky_reflections() {
        let mut s = scene();
        s.e.mem.set_u8(s.group.addr() + 0x5c, 0);
        s.update(false);
        let (this, viewer) = (s.this.addr(), s.viewer);
        assert_eq!(s.e.global::<u8>(RENDER_SKY_REFLECTIONS), 1);
        assert_eq!(s.e.global::<u8>(RENDER_WORLD_REFLECTIONS), 0);
        assert_eq!(s.calls(SETUP_SKY_REFLECTIONS), vec![vec![this, viewer]]);
        assert_eq!(s.e.global::<u32>(SKY_REFLECTION_THREAD_STAGE), 0);
        assert_eq!(s.calls(FINISH_SKY_REFLECTIONS), vec![vec![this]]);
    }

    #[test]
    fn something_in_the_water_adds_a_depth_pass_per_group() {
        let mut s = scene();
        // A second group with the same reference.
        let second = s.e.mem.alloc(0xb0);
        s.e.mem.set_u32(second, s.water_form);
        s.e.mem.set_u8(second + 0x5c, 1);
        s.e.mem.set_u32(second + 0xa8, 0x0ca0_0002);
        s.e.mem.set_u32(second + 0xac, 0x0001_0005);
        fill_list(&mut s.e, second + 0x24, &[s.reference]);
        fill_list(&mut s.e, s.this.addr() + 0x3c, &[s.group.addr(), second]);
        // An actor is in the water of both groups.
        fill_list(&mut s.e, s.group.addr() + 0x3c, &[1]);
        fill_list(&mut s.e, second + 0x3c, &[1]);
        s.update(false);
        let (this, group, viewer) = (s.this.addr(), s.group.addr(), s.viewer);
        assert!(s.group_flag(0x5e), "bRenderDepth");
        assert_eq!(
            s.calls(SETUP_GROUP_DEPTH),
            vec![vec![this, viewer, group], vec![this, viewer, second]]
        );
        // Stage 0 went to the world reflections.
        assert_eq!(s.e.mem.u32(group + 0xa0), 1);
        assert_eq!(s.e.mem.u32(second + 0xa0), 2);
        // The depth pass: the first group puts the target in mode 7.
        assert_eq!(
            s.calls(RENDER_TARGET_SET),
            vec![vec![7, 0x7777], vec![0, 0x7777]]
        );
        assert_eq!(s.calls(RENDERED_TEXTURE_STOP), vec![vec![DEPTH_TEXTURE]; 2]);
        assert_eq!(
            s.calls(RENDER_GROUP_DEPTH),
            vec![
                vec![this, 0x0ca0_0001, group, 1, 3],
                vec![this, 0x0ca0_0002, second, 0, 5]
            ]
        );
        assert_eq!(s.calls(BATCH_RENDERER_END_PASS).len(), 2);
        assert_eq!(s.e.global::<u32>(LAST_DEPTH_STENCIL_MASK), 0x0001_0005);
        let constructs = s.calls(SHADER_PARAM_CONSTRUCT);
        assert_eq!(constructs.len(), 2);
        let param = constructs[0][0];
        assert_eq!(
            s.calls(SHADER_PARAM_INIT_CONSTANT_MAP)[0],
            vec![param, 0, 1]
        );
        assert_eq!(
            s.calls(SHADER_PARAM_SET_PIXEL_CONSTANT)[0],
            vec![param, 0, 1.0f32.to_bits(), 0, 0, 1.0f32.to_bits()]
        );
        assert_eq!(
            s.calls(SET_STENCIL_STATE),
            vec![vec![2, 0xff, 0x0007_0003, 0], vec![2, 0xff, 0x0001_0005, 0]]
        );
        assert_eq!(
            s.calls(IMAGE_SPACE_RENDER_EFFECT)[0],
            vec![0x0ee0_0000, 0x28, s.renderer, DEPTH_TEXTURE, param, 0]
        );
        assert_eq!(
            s.calls(AFTER_GROUP_DEPTH),
            vec![vec![this, group, 3], vec![this, second, 5]]
        );
        assert_eq!(s.calls(RENDER_TARGET_RESET).len(), 2);
        assert_eq!(s.calls(SHADER_PARAM_DESTRUCT).len(), 2);
        // The clear colour is saved, set to the depth colour and restored.
        let saved = s.calls(RENDERER_GET_COLOR_DOUBLE);
        assert_eq!(saved.len(), 1);
        let set = s.calls(RENDERER_SET_COLOR_DOUBLE);
        assert_eq!(set.len(), 2);
        assert_eq!(set[1], saved[0], "the saved colour is restored");
        assert_eq!(set[0][0], s.renderer);
    }

    #[test]
    fn no_depth_pass_without_the_depth_setting() {
        let mut s = scene();
        fill_list(&mut s.e, s.group.addr() + 0x3c, &[1]);
        set_setting(&mut s.e, SETTING_USE_WATER_DEPTH, false);
        s.update(false);
        assert!(s.group_flag(0x5e), "the group still wants depth");
        assert!(s.calls(SETUP_GROUP_DEPTH).is_empty());
        assert!(s.calls(RENDER_GROUP_DEPTH).is_empty());
        // The shared depth map is given back instead.
        assert_eq!(
            s.calls(RETURN_RENDERED_TEXTURE),
            vec![vec![s.texture_manager, DEPTH_TEXTURE]]
        );
        assert_eq!(s.e.global::<u32>(DEPTH_MAP), 0);
    }

    #[test]
    fn a_reference_out_of_range_leaves_its_group_off_and_gives_the_textures_back() {
        let mut s = scene();
        s.e.mem.set_u8(s.reference + REFERENCE_IN_RANGE, 0);
        s.update(false);
        let (this, group) = (s.this.addr(), s.group.addr());
        assert!(!s.group_flag(0x5d));
        assert_eq!(s.e.global::<u32>(ACTIVE_WATER_GROUPS), 0);
        // The group is released, no pass is set up.
        assert_eq!(s.calls(RELEASE_GROUP), vec![vec![this, group, 0, 0]]);
        assert!(s.calls(SETUP_WORLD_REFLECTIONS).is_empty());
        assert!(s.calls(FINISH_GROUP).is_empty());
        // The shared depth and wading maps and the water form's noise map
        // are returned and cleared.
        let tm = s.texture_manager;
        assert_eq!(
            s.calls(RETURN_RENDERED_TEXTURE),
            vec![
                vec![tm, DEPTH_TEXTURE],
                vec![tm, WADING_TEXTURE],
                vec![tm, FORM_NOISE_TEXTURE]
            ]
        );
        assert_eq!(s.e.global::<u32>(DEPTH_MAP), 0);
        assert_eq!(s.e.global::<u32>(WADING_WATER_HEIGHT_MAP), 0);
        assert_eq!(s.e.mem.u32(s.water_form + 0x30), 0);
        // The reference's property lost all its textures.
        for offset in [0x13c, 0x140, 0x144, 0x138] {
            assert_eq!(
                s.e.mem.u32(s.property.addr() + offset),
                0,
                "slot {offset:x}"
            );
        }
    }

    #[test]
    fn a_forced_update_switches_the_group_on_out_of_range() {
        let mut s = scene();
        s.e.mem.set_u8(s.reference + REFERENCE_IN_RANGE, 0);
        s.update(true);
        assert!(s.group_flag(0x5d));
        assert_eq!(s.calls(FINISH_GROUP).len(), 1);
    }

    #[test]
    fn a_culled_node_is_never_in_range() {
        let mut s = scene();
        let node = s.e.mem.u32(s.reference + REFERENCE_NODE);
        s.e.mem.set_u32(node + 0x30, 1);
        s.update(false);
        assert!(!s.group_flag(0x5d));
        // The range test is not even asked.
        assert!(s.calls(REFERENCE_IS_IN_RANGE).is_empty());
    }

    #[test]
    fn a_reference_without_the_flag_is_ignored_unless_its_cell_matches() {
        let mut s = scene();
        s.e.mem.set_u32(s.base_form + BASE_FORM_FLAGS, 0x1000_0000);
        s.update(false);
        assert!(!s.group_flag(0x5d), "the 0x08000000 flag is missing");
        let cell = s.e.mem.alloc(0x40);
        s.e.mem.set_u8(cell + 0x26, 6);
        s.e.mem.set_u32(s.reference + 0x40, cell);
        s.update(false);
        assert!(s.group_flag(0x5d), "a parent cell of kind 6 is enough");
    }

    #[test]
    fn in_an_interior_the_group_renders_its_own_reflections() {
        let mut s = scene();
        let tes = s.e.global::<u32>(TES_POINTER);
        s.e.mem.set_u32(tes + 0x34, 0x00ce_0000);
        s.e.mem.set_u32(s.this.addr(), 3);
        let cell = s.e.mem.alloc(0x40);
        s.e.mem.set_u8(cell + 0x26, 6);
        s.e.mem.set_u32(s.reference + 0x40, cell);
        s.update(false);
        let (this, group, viewer) = (s.this.addr(), s.group.addr(), s.viewer);
        assert!(s.group_flag(0x5d));
        assert!(s.group_flag(0x5f), "bRenderGroupReflections");
        assert_eq!(s.e.global::<u8>(RENDER_WORLD_REFLECTIONS), 0);
        assert_eq!(s.e.global::<u8>(RENDER_SKY_REFLECTIONS), 0);
        assert!(s.calls(SETUP_WORLD_REFLECTIONS).is_empty());
        assert!(s.calls(FINISH_WORLD_REFLECTIONS).is_empty());
        // The high-detail flag is 0 in an interior.
        assert_eq!(
            s.calls(SETUP_GROUP_REFLECTIONS),
            vec![vec![this, viewer, group, 0]]
        );
        assert_eq!(s.e.mem.u32(group + 0x9c), 3);
        assert_eq!(s.e.mem.u32(this), 4);
        assert_eq!(s.calls(FINISH_GROUP_INTERIOR), vec![vec![this, group]]);
        assert_eq!(s.calls(FINISH_GROUP), vec![vec![this, group]]);
    }

    #[test]
    fn too_many_groups_log_a_message_and_switch_the_group_off() {
        let mut s = scene();
        s.e.mem.set_u32(s.this.addr(), 15);
        s.update(false);
        let (this, group) = (s.this.addr(), s.group.addr());
        // The world reflections took stage 15, so the group has none left.
        assert_eq!(s.e.global::<u32>(WORLD_REFLECTION_THREAD_STAGE), 15);
        assert!(!s.group_flag(0x5d));
        let print = s.calls(FORMATTED_PRINT);
        assert_eq!(print.len(), 1);
        let height = 5.5f64.to_bits();
        assert_eq!(
            print[0][1..],
            [
                0x100,
                TOO_MANY_GROUPS_MESSAGE,
                height as u32,
                (height >> 32) as u32,
                0x0555_0000,
                1
            ]
        );
        let text = print[0][0];
        assert_eq!(s.calls(LOG_MESSAGE), vec![vec![FORMAT_STRING, text]]);
        assert!(s.calls(SETUP_GROUP_REFLECTIONS).is_empty());
        assert_eq!(s.calls(RELEASE_GROUP), vec![vec![this, group, 0, 0]]);
    }

    /// Runs the scene with `flags` as the base form flags and returns the
    /// group's `bRenderDepth`.
    fn depth_wanted(mut s: Scene, flags: u32) -> bool {
        s.e.mem.set_u32(s.base_form + BASE_FORM_FLAGS, flags);
        s.update(false);
        s.group_flag(0x5e)
    }

    #[test]
    fn depth_needs_both_the_depth_and_the_refraction_flags() {
        let base = 0x0800_0000;
        assert!(!depth_wanted(scene(), base | 0x1000_0000));
        assert!(!depth_wanted(scene(), base | 0x200));
        // With both flags but nothing in the water and no reason: no depth.
        assert!(!depth_wanted(scene(), base | 0x1000_0000 | 0x200));
    }

    #[test]
    fn depth_is_wanted_for_objects_in_the_water() {
        let mut s = scene();
        fill_list(&mut s.e, s.group.addr() + 0x30, &[1]);
        assert!(depth_wanted(s, 0x0800_0000 | 0x1000_0000 | 0x200));
    }

    #[test]
    fn depth_is_wanted_for_the_800_flag() {
        let s = scene();
        assert!(depth_wanted(s, 0x0800_0000 | 0x1000_0000 | 0x200 | 0x800));
    }

    #[test]
    fn depth_is_wanted_when_the_reflected_refs_are_not_empty() {
        let mut s = scene();
        // `ExtraDataList::GetReflectedRefs` returns the word at +0x44 of the
        // reference; the object it names is not empty.
        let reflected = s.e.mem.alloc(8);
        s.e.mem.set_u32(reflected, 0x1234);
        s.e.mem.set_u32(s.reference + 0x44, reflected);
        assert!(depth_wanted(s, 0x0800_0000 | 0x1000_0000 | 0x200));
        // An empty one is not a reason.
        let mut s = scene();
        let reflected = s.e.mem.alloc(8);
        s.e.mem.set_u32(s.reference + 0x44, reflected);
        assert!(!depth_wanted(s, 0x0800_0000 | 0x1000_0000 | 0x200));
    }

    #[test]
    fn depth_is_wanted_for_the_40000000_flag_with_the_static_object_filled() {
        let flags = 0x0800_0000 | 0x1000_0000 | 0x200 | 0x4000_0000;
        let mut s = scene();
        s.e.mem.set_u32(WATER_OBJECT_011CA13C, 1);
        assert!(depth_wanted(s, flags));
        let s = scene();
        assert!(!depth_wanted(s, flags));
    }

    #[test]
    fn silhouette_reflections_follow_the_auto_setting_and_the_form_flags() {
        let mut s = scene();
        set_setting(&mut s.e, SETTING_AUTO_SILHOUETTE_REFLECTIONS, true);
        s.e.mem.set_u32(
            s.base_form + BASE_FORM_FLAGS,
            0x0800_0000 | 0x1000_0000 | 0x200 | 0x4_0000,
        );
        s.update(false);
        assert!(s.group_flag(0x60));
        // Without the auto setting they stay off.
        let mut s = scene();
        s.e.mem.set_u32(
            s.base_form + BASE_FORM_FLAGS,
            0x0800_0000 | 0x1000_0000 | 0x200 | 0x4_0000,
        );
        s.update(false);
        assert!(!s.group_flag(0x60));
        // The low-detail setting alone is enough.
        let mut s = scene();
        set_setting(&mut s.e, SETTING_AUTO_SILHOUETTE_REFLECTIONS, true);
        set_setting(&mut s.e, SETTING_FORCE_LOW_DETAIL_REFLECTIONS, true);
        s.update(false);
        assert!(s.group_flag(0x60));
    }

    #[test]
    fn a_rendered_group_keeps_only_the_textures_the_settings_allow() {
        let mut s = scene();
        // The property wants reflections and depth; refractions are off.
        s.e.mem.set_u8(s.property.addr() + 0x80, 1);
        s.e.mem.set_u8(s.property.addr() + 0x63, 1);
        s.update(false);
        let property = s.property.addr();
        assert_ne!(s.e.mem.u32(property + 0x13c), 0, "reflection map kept");
        assert_eq!(s.e.mem.u32(property + 0x140), 0, "refraction map cleared");
        assert_ne!(s.e.mem.u32(property + 0x144), 0, "depth map kept");
        assert_eq!(s.e.mem.u32(property + 0x138), 0, "normal map cleared");
    }

    #[test]
    fn a_rendered_group_with_reflections_off_is_released() {
        let mut s = scene();
        set_setting(&mut s.e, SETTING_USE_WATER_REFLECTIONS, false);
        s.update(false);
        let (this, group) = (s.this.addr(), s.group.addr());
        assert_eq!(s.calls(RELEASE_GROUP), vec![vec![this, group, 0, 0]]);
    }

    #[test]
    fn the_wading_geometry_of_a_group_loses_its_textures_when_the_group_is_off() {
        let mut s = scene();
        s.e.mem.set_u8(s.reference + REFERENCE_IN_RANGE, 0);
        let wading = s.e.mem.alloc(0x200);
        let wading_property = s.e.mem.alloc(0x200);
        s.e.mem.set_u32(wading + NODE_PROPERTY, wading_property);
        for offset in [0x13c, 0x140, 0x144, 0x138] {
            s.e.mem.set_u32(wading_property + offset, 0xaaaa);
        }
        s.e.mem.set_u32(s.group.addr() + 0x58, wading);
        s.update(false);
        for offset in [0x13c, 0x140, 0x144, 0x138] {
            assert_eq!(s.e.mem.u32(wading_property + offset), 0, "slot {offset:x}");
        }
    }

    // --- UpdateLODWater -----------------------------------------------------

    /// A scene whose single group is the system's LOD water group.
    fn lod_scene() -> Scene {
        let mut s = scene();
        let (this, group) = (s.this.addr(), s.group.addr());
        s.e.mem.set_u32(this + 0x48, group);
        s.e.mem.set_u8(group + 0x5d, 0);
        // The shared reflection maps the LOD group points the property at.
        s.e.set_global(WORLD_REFLECTION_MAP, 0x1111u32);
        s.e.set_global(SKY_REFLECTION_MAP, 0x2222u32);
        s
    }

    impl Scene {
        fn update_lod(&mut self, force_update: bool) {
            start_log(&mut self.e);
            let (this, viewer) = (self.this, self.viewer);
            self.e.call(0x004e_3320, &args![this, viewer, force_update]);
        }
    }

    #[test]
    fn lod_water_without_a_group_does_nothing() {
        let mut s = scene();
        s.update_lod(false);
        // Only the call itself is in the log.
        assert_eq!(s.e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn lod_water_in_an_interior_only_logs_a_message() {
        let mut s = lod_scene();
        let tes = s.e.global::<u32>(TES_POINTER);
        s.e.mem.set_u32(tes + 0x34, 1);
        s.e.mem.set_u8(s.group.addr() + 0x5d, 1);
        s.update_lod(false);
        assert!(s.calls(WATER_REFERENCE_3D).is_empty());
        assert_eq!(s.calls(LOG_MESSAGE), vec![vec![LOD_IN_INTERIOR_MESSAGE]]);
        assert!(s.group_flag(0x5d), "the group is left as it was");
    }

    #[test]
    fn lod_water_in_range_points_the_property_at_the_sky_map() {
        let mut s = lod_scene();
        s.update_lod(false);
        assert!(s.group_flag(0x5d));
        assert_eq!(s.e.global::<u8>(RENDER_SKY_REFLECTIONS), 1);
        assert_eq!(s.e.global::<u8>(RENDER_WORLD_REFLECTIONS), 0);
        assert_eq!(
            s.calls(NI_POINTER_ASSIGN_FROM),
            vec![vec![s.property.addr() + 0x13c, SKY_REFLECTION_MAP]]
        );
        assert_eq!(s.e.mem.u32(s.property.addr() + 0x13c), 0x2222);
        // The rendered group's shader properties are updated afterwards.
        assert_eq!(
            s.calls(WATER_REFERENCE_3D).last().unwrap(),
            &vec![s.this.addr(), s.reference]
        );
    }

    #[test]
    fn lod_water_with_full_reflections_uses_the_world_map() {
        let mut s = lod_scene();
        s.e.mem.set_u8(s.property.addr() + 0x62, 1);
        s.update_lod(false);
        assert_eq!(s.e.global::<u8>(RENDER_WORLD_REFLECTIONS), 1);
        assert_eq!(s.e.global::<u8>(RENDER_SKY_REFLECTIONS), 0);
        assert_eq!(s.e.mem.u32(s.property.addr() + 0x13c), 0x1111);
    }

    #[test]
    fn lod_water_out_of_range_drops_the_reflection_map() {
        let mut s = lod_scene();
        s.e.mem.set_u8(s.reference + REFERENCE_IN_RANGE, 0);
        s.update_lod(false);
        assert!(!s.group_flag(0x5d));
        assert_eq!(s.e.mem.u32(s.property.addr() + 0x13c), 0);
        // The other slots stay.
        assert_ne!(s.e.mem.u32(s.property.addr() + 0x140), 0);
        // Nothing is updated for a group that is not rendered.
        assert_eq!(s.calls(WATER_REFERENCE_3D).len(), 1);
    }

    #[test]
    fn lod_water_forced_update_ignores_the_range() {
        let mut s = lod_scene();
        s.e.mem.set_u8(s.reference + REFERENCE_IN_RANGE, 0);
        s.update_lod(true);
        assert!(s.group_flag(0x5d));
        assert_eq!(s.e.global::<u8>(RENDER_SKY_REFLECTIONS), 1);
    }

    #[test]
    fn lod_water_while_water_is_skipped_releases_the_group() {
        let mut s = lod_scene();
        s.e.mem.set_u8(0x011c_7a59, 1);
        s.update_lod(false);
        let (this, group) = (s.this.addr(), s.group.addr());
        assert_eq!(s.calls(RELEASE_GROUP), vec![vec![this, group, 0, 0]]);
        assert_eq!(s.e.global::<u8>(RENDER_SKY_REFLECTIONS), 0);
        assert!(s.calls(NI_POINTER_ASSIGN_FROM).is_empty());
    }

    // --- UpdateWaterShaderProperties ----------------------------------------

    #[test]
    fn shader_properties_are_updated_for_every_reference_of_the_group() {
        let mut s = scene();
        let second = object(&mut s.e, REFERENCE_VTABLE, 0x100);
        let (this, group) = (s.this.addr(), s.group.addr());
        fill_list(&mut s.e, group + 0x24, &[s.reference, second]);
        start_log(&mut s.e);
        s.e.call(0x004e_3520, &args![this, group]);
        assert_eq!(
            s.calls(WATER_REFERENCE_3D),
            vec![vec![this, s.reference], vec![this, second]]
        );
    }

    // --- UpdateWaterShaderProperties_ov2 ------------------------------------

    /// The packed colours the water form returns: red 0, green 0x80, blue 0xff.
    const SHALLOW_COLOR: u32 = 0x00ff_8000;
    const DEEP_COLOR: u32 = 0x0000_ff00;
    const REFLECTION_COLOR: u32 = 0x0033_6699;

    /// A scene ready for `ov2`: the reference's water form exists, the
    /// property wants its constants updated and all the water form getters
    /// answer with distinct values.
    fn shader_scene() -> Scene {
        let mut s = scene();
        s.e.mem
            .set_u32(s.base_form + BASE_FORM_WATER_FORM, s.water_form);
        s.e.mem.set_u32(s.water_node + NODE_VIRTUAL_RESULT, 1);
        s.e.mem.set_u8(s.property.addr() + 0x7f, 1);
        let form = s.water_form;
        s.e.mem.set_f32(form + 0x9c, 0.75);
        s.e.mem.set_f32(form + 0x108, 0.3);
        s.e.mem.set_f32(form + 0x114, 20.0);
        s.e.mem.set_f32(form + 0x128, 30.0);
        s.e.mem.set_f32(form + 0x12c, 40.0);
        for (address, color) in [
            (0x004f_9bf0u32, SHALLOW_COLOR),
            (0x009e_e040, DEEP_COLOR),
            (0x008d_80e0, REFLECTION_COLOR),
        ] {
            s.e.register_double(address, move |_, _| ret(color));
        }
        for (address, value) in [
            (0x0064_4930u32, 1.5f32),
            (0x008d_01e0, 2.5),
            (0x0058_0100, 3.5),
            (0x004a_0d90, 4.5),
            (0x004a_0db0, 5.5),
            (0x0064_6f50, 6.5),
            (0x009a_5480, 7.5),
            (0x0088_5d70, 8.5),
            (0x0050_7b20, 100.0),
            (0x0050_8070, 9.5),
            (0x009b_88a0, 50.0),
            (0x0081_2870, 10.0),
            (0x0094_42a0, 11.5),
            (0x0064_4950, 12.5),
            (0x0064_4970, 13.5),
        ] {
            s.e.register_double(address, move |_, _| value.into_ret());
        }
        s
    }

    impl Scene {
        fn update_shader(&mut self) {
            start_log(&mut self.e);
            let (this, group, reference) = (self.this, self.group, self.reference);
            self.e.call(0x004e_3590, &args![this, group, reference]);
        }

        fn property_f32(&self, offset: u32) -> f32 {
            self.e.mem.f32(self.property.addr() + offset)
        }
    }

    #[test]
    fn the_water_form_is_copied_into_the_shader_property() {
        let mut s = shader_scene();
        s.update_shader();
        // Colours: red, green, blue, alpha 1.0.
        let color = |s: &Scene, offset: u32| -> [f32; 4] {
            [0, 4, 8, 12].map(|i| s.property_f32(offset + i))
        };
        assert_eq!(color(&s, 0x88), [0.0, 128.0 / 255.0, 1.0, 1.0]);
        assert_eq!(color(&s, 0x98), [0.0, 1.0, 0.0, 1.0]);
        assert_eq!(
            color(&s, 0xa8),
            [
                (0x99 as f64 / 255.0) as f32,
                (0x66 as f64 / 255.0) as f32,
                (0x33 as f64 / 255.0) as f32,
                1.0
            ]
        );
        // Amounts from the water form.
        assert_eq!(s.property_f32(0xb8), 1.5);
        assert_eq!(s.property_f32(0xbc), 2.5);
        assert_eq!(s.property_f32(0xc0), 3.5);
        assert_eq!(s.property_f32(0xc4), 4.5);
        assert_eq!(s.property_f32(0x118), 0.75);
        assert_eq!(s.property_f32(0x100), 5.5);
        assert_eq!(s.property_f32(0x104), 6.5);
        assert_eq!(s.property_f32(0xd0), 30.0);
        assert_eq!(s.property_f32(0xd4), 40.0);
        assert_eq!(s.property_f32(0x11c), 7.5);
        assert_eq!(s.e.global::<f32>(WATER_SHADER_FLOAT), 8.5);
        assert_eq!(s.property_f32(0x124), 11.5);
        // Not underwater: the fog is 50 (far) and 50 - 10 (range), and the
        // form's own fog amount while its 0x80000000 flag is clear.
        assert_eq!(s.property_f32(0x74), 50.0);
        assert_eq!(s.property_f32(0x78), 40.0);
        assert_eq!(s.property_f32(0x120), 0.3);
        // The depth falloff.
        assert_eq!(s.property_f32(0xd8), 12.5);
        assert_eq!(s.property_f32(0xdc), 13.5);
    }

    #[test]
    fn underwater_the_fog_comes_from_the_underwater_values() {
        let mut s = shader_scene();
        s.e.mem.set_u8(0x011c_7a59, 1);
        s.e.mem.set_f32(s.property.addr() + 0x120, 77.0);
        s.update_shader();
        assert_eq!(s.property_f32(0x74), 100.0);
        // 100 minus the form's fUnderwaterFogDistNear (20).
        assert_eq!(s.property_f32(0x78), 80.0);
        assert_eq!(s.property_f32(0x120), 9.5);
        // With the 0x80000000 flag set the fog amount is left alone.
        let mut s = shader_scene();
        s.e.mem.set_u8(0x011c_7a59, 1);
        s.e.mem.set_u32(s.base_form + BASE_FORM_FLAGS, 0x8000_0000);
        s.e.mem.set_f32(s.property.addr() + 0x120, 77.0);
        s.update_shader();
        assert_eq!(s.property_f32(0x120), 77.0);
    }

    #[test]
    fn the_flag_byte_at_011c7a58_selects_the_underwater_values_too() {
        let mut s = shader_scene();
        s.e.set_global(WATER_FLAG_011C7A58, 1u8);
        s.update_shader();
        assert_eq!(s.property_f32(0x74), 100.0);
    }

    #[test]
    fn a_form_with_the_80000000_flag_has_no_fog_amount_when_not_underwater() {
        let mut s = shader_scene();
        s.e.mem.set_u32(s.base_form + BASE_FORM_FLAGS, 0x8000_0000);
        s.e.mem.set_f32(s.property.addr() + 0x120, 77.0);
        s.update_shader();
        assert_eq!(s.property_f32(0x120), 0.0);
    }

    #[test]
    fn the_property_flags_follow_the_form_and_the_settings() {
        let mut s = shader_scene();
        // Base form: reflections flag (1) and the depth flag, no refraction flag.
        s.e.mem
            .set_u32(s.base_form + BASE_FORM_FLAGS, 1 | 0x1000_0000 | 0x2000_0000);
        s.update_shader();
        let property = s.property.addr();
        assert_eq!(s.e.mem.u8(property + 0x80), 1, "bReflections");
        assert_eq!(s.e.mem.u8(property + 0x81), 0, "bRefractions");
        assert_eq!(s.e.mem.u8(property + 0x63), 1, "bDepth");
        assert_eq!(s.e.mem.u8(property + 0x82), 1, "bObjectTexCoords");
        // A setting that is off keeps the flag off.
        let mut s = shader_scene();
        set_setting(&mut s.e, SETTING_USE_WATER_REFLECTIONS, false);
        s.e.mem.set_u32(s.base_form + BASE_FORM_FLAGS, 1);
        s.update_shader();
        assert_eq!(s.e.mem.u8(s.property.addr() + 0x80), 0);
    }

    #[test]
    fn a_changed_flag_resets_the_render_pass_lists() {
        let mut s = shader_scene();
        let property = s.property.addr();
        let passes = s.e.mem.alloc(0x20);
        s.e.mem.set_u32(passes + 0x10, 5);
        s.e.mem.set_u32(property + 0x3c, passes);
        // The group's wading geometry has a property with passes.
        let wading = s.e.mem.alloc(0x200);
        let wading_property = object(&mut s.e, PROPERTY_VTABLE, 0x200);
        let wading_passes = s.e.mem.alloc(0x20);
        s.e.mem.set_u32(wading_passes + 0x10, 6);
        s.e.mem.set_u32(wading_property + 0x3c, wading_passes);
        s.e.mem.set_u32(wading_property + PROPERTY_PASS_COUNT, 2);
        s.e.mem.set_u32(wading + NODE_PROPERTY, wading_property);
        s.e.mem.set_u32(s.group.addr() + 0x58, wading);
        s.update_shader();
        assert_eq!(s.e.mem.u32(passes + 0x10), 0);
        assert_eq!(s.e.mem.u32(wading_passes + 0x10), 0);
        // The property's virtual at +0xa4 was asked with 0.
        assert_eq!(
            s.calls(PROPERTY_PASS_COUNT_DOUBLE),
            vec![vec![wading_property, 0]]
        );
    }

    #[test]
    fn unchanged_flags_leave_the_render_pass_lists_alone() {
        let mut s = shader_scene();
        let property = s.property.addr();
        // The flags the default base form and settings give: refractions, depth.
        s.e.mem.set_u8(property + 0x81, 1);
        s.e.mem.set_u8(property + 0x63, 1);
        let passes = s.e.mem.alloc(0x20);
        s.e.mem.set_u32(passes + 0x10, 5);
        s.e.mem.set_u32(property + 0x3c, passes);
        s.update_shader();
        assert_eq!(s.e.mem.u32(passes + 0x10), 5);
        assert!(s.calls(PROPERTY_PASS_COUNT_DOUBLE).is_empty());
    }

    #[test]
    fn a_wading_property_without_passes_keeps_its_list() {
        let mut s = shader_scene();
        let wading = s.e.mem.alloc(0x200);
        let wading_property = object(&mut s.e, PROPERTY_VTABLE, 0x200);
        let wading_passes = s.e.mem.alloc(0x20);
        s.e.mem.set_u32(wading_passes + 0x10, 6);
        s.e.mem.set_u32(wading_property + 0x3c, wading_passes);
        s.e.mem.set_u32(wading + NODE_PROPERTY, wading_property);
        s.e.mem.set_u32(s.group.addr() + 0x58, wading);
        s.update_shader();
        assert_eq!(s.e.mem.u32(wading_passes + 0x10), 6);
    }

    #[test]
    fn an_equal_depth_falloff_pair_becomes_minus_one_and_zero() {
        let mut s = shader_scene();
        s.e.register_double(0x0064_4970, |_, _| 12.5f32.into_ret());
        s.e.mem.set_f32(s.property.addr() + 0xdc, 9.0);
        s.update_shader();
        assert_eq!(s.property_f32(0xd8), -1.0);
        assert_eq!(s.property_f32(0xdc), 0.0);
    }

    #[test]
    fn the_shader_property_is_left_alone_without_its_conditions() {
        // No water form for the base object.
        let mut s = shader_scene();
        s.e.mem.set_u32(s.base_form + BASE_FORM_WATER_FORM, 0);
        s.update_shader();
        assert_eq!(s.property_f32(0xb8), 0.0);
        // The property does not want its constants updated.
        let mut s = shader_scene();
        s.e.mem.set_u8(s.property.addr() + 0x7f, 0);
        s.update_shader();
        assert_eq!(s.property_f32(0xb8), 0.0);
        // The owner is not of the water type.
        let mut s = shader_scene();
        let owner = s.e.mem.u32(s.water_node + 0xc0);
        s.e.mem.set_u32(owner + 0x68, 3);
        s.update_shader();
        assert_eq!(s.property_f32(0xb8), 0.0);
        // The node's virtual at +0x18 says no.
        let mut s = shader_scene();
        s.e.mem.set_u32(s.water_node + NODE_VIRTUAL_RESULT, 0);
        s.update_shader();
        assert_eq!(s.property_f32(0xb8), 0.0);
        // The reference has no water node at all.
        let mut s = shader_scene();
        s.e.mem.set_u32(s.reference + REFERENCE_WATER_NODE, 0);
        s.update_shader();
        assert_eq!(s.property_f32(0xb8), 0.0);
    }

    // --- the water form update (004e3d60) -----------------------------------

    /// A flag the `WaterTypeUpdateMap` lookup double answers with.
    const MAP_FIND_ANSWER: u32 = 0x011c_7a5a;

    /// A scene ready for `fn_004e3d60`: the reference's base form has a water
    /// form with a noise texture, the property already has the system's
    /// noise map and the form its rendered noise texture, and every callee
    /// the scroll uses answers with a fixed value.
    fn form_scene() -> Scene {
        let mut s = shader_scene();
        let property = s.property.addr();
        let form = s.water_form;
        s.e.register(WATER_FORM_HAS_NOISE_TEXTURE, |_, _| ret(1));
        for address in [
            FLUSH_OBJECT_FLUSH,
            IMAGE_SPACE_RENDER_NOISE,
            WATER_TYPE_MAP_SET,
            POINTER_TEMP_DESTRUCT,
            CREATE_TEXTURE_IMAGE,
        ] {
            s.e.register(address, |_, _| Ret::default());
        }
        s.e.register(WATER_TYPE_MAP_FIND, |e, _| {
            ret(e.mem.u8(MAP_FIND_ANSWER) as u32)
        });
        s.e.register(FLOAT_TABLE_ENTRY, |_, _| 0.1f32.into_ret());
        s.e.register(CRT_SIN, |_, _| 0.5f64.into_ret());
        s.e.register(CRT_COS, |_, _| 0.25f64.into_ret());
        s.e.register(FLOAT_REMAINDER, |_, a| {
            ((f32::from_bits(a[0]) as f64) % (f32::from_bits(a[1]) as f64)).into_ret()
        });
        s.e.register(FLOAT_MAX, |_, a| {
            f32::from_bits(a[0]).max(f32::from_bits(a[1])).into_ret()
        });
        s.e.register(CRT_CEIL, |_, a| f64::take(a, &mut 0).ceil().into_ret());
        for (address, value) in [
            (0x0096_6a20u32, 10.0f32),
            (0x0064_47d0, 2.0),
            (0x0064_47f0, 10.0),
            (0x0082_1640, 50.0),
            (0x0082_1660, 20.0),
            (0x0056_7470, 1.25),
            (0x0082_1680, 2.25),
            (0x0081_33b0, 3.25),
        ] {
            s.e.register_double(address, move |_, _| value.into_ret());
        }
        let flush_object = s.e.mem.alloc(0x300);
        s.e.set_global(FLUSH_OBJECT, flush_object);
        // The water form: texture scroll rows, wind and the height scale.
        for (offset, value) in [
            (0x34u32, 0.2f32),
            (0x38, 0.1),
            (0x44, 0.3),
            (0x48, 0.4),
            (0x54, 0.9),
            (0x58, 0.5),
            (0xe8, 90.0),
            (0xf0, 45.0),
            (0xf8, 4.0),
            (0x130, 250.0),
        ] {
            s.e.mem.set_f32(form + offset, value);
        }
        s.e.mem.set_f32(property + 0x11c, 0.5);
        s.e.mem.set_u32(property + 0x134, 0xbeef);
        set_setting(&mut s.e, SETTING_USE_PER_WORLD_SPACE_NOISE, true);
        s.e.mem.set_u8(MAP_FIND_ANSWER, 0);
        s
    }

    impl Scene {
        fn update_forms(&mut self) {
            start_log(&mut self.e);
            let (this, group) = (self.this, self.group);
            self.e.call(0x004e_3d60, &args![this, group]);
        }
    }

    /// The scroll arithmetic: `(speed * step) * trig + old`, rounded to float.
    fn scrolled(old: f32, speed: f32, step: f32, trig: f64) -> f32 {
        (((speed as f64) * (step as f64)) * trig + old as f64) as f32
    }

    #[test]
    fn the_first_reference_of_a_form_scrolls_wraps_and_publishes_its_rows() {
        let mut s = form_scene();
        let (this, form) = (s.this.addr(), s.water_form);
        s.update_forms();
        // The reference's property shares the form's rendered noise texture.
        assert_eq!(s.e.mem.u32(s.property.addr() + 0x138), FORM_NOISE_TEXTURE);
        // Sine (0.5) moves the first channel of a row, cosine (0.25) the second.
        let step = 0.1f32;
        let row0 = (
            scrolled(0.2, 2.0, step, 0.5),
            scrolled(0.1, 2.0, step, 0.25),
        );
        let row1 = (
            scrolled(0.3, 4.0, step, 0.5),
            scrolled(0.4, 4.0, step, 0.25),
        );
        // 0.9 + 10 * 0.1 * 0.5 = 1.4 wraps to 0.4.
        let row2_x = scrolled(0.9, 10.0, step, 0.5);
        assert!(row2_x > 1.0);
        let wrapped = ((row2_x as f64) % 1.0) as f32;
        let row2 = (wrapped, scrolled(0.5, 10.0, step, 0.25));
        assert_eq!(s.e.mem.f32(form + 0x34), row0.0);
        assert_eq!(s.e.mem.f32(form + 0x38), row0.1);
        assert_eq!(s.e.mem.f32(form + 0x44), row1.0);
        assert_eq!(s.e.mem.f32(form + 0x48), row1.1);
        assert_eq!(s.e.mem.f32(form + 0x54), row2.0);
        assert_eq!(s.e.mem.f32(form + 0x58), row2.1);
        assert!(row2.0 < 0.5 && row2.0 > 0.39);
        // Published: the noise scale, the three rows and the scales.
        assert_eq!(s.e.global::<f32>(SHADER_NOISE_BASE), 0.5);
        assert_eq!(s.e.global::<f32>(SHADER_NOISE_BASE + 4), row0.0);
        assert_eq!(s.e.global::<f32>(SHADER_NOISE_BASE + 8), row0.1);
        assert_eq!(s.e.global::<f32>(SHADER_NOISE_BASE + 0x14), row1.0);
        assert_eq!(s.e.global::<f32>(SHADER_NOISE_BASE + 0x24), row2.0);
        // 250 / 100 rounded up is 3; 50 / 100 and 20 / 100 round up to 1.
        assert_eq!(s.e.global::<f32>(SHADER_NOISE_BASE + 0x34), 3.0);
        assert_eq!(s.e.global::<f32>(SHADER_NOISE_BASE + 0x38), 1.0);
        assert_eq!(s.e.global::<f32>(SHADER_NOISE_BASE + 0x3c), 1.0);
        assert_eq!(s.e.global::<f32>(SHADER_NOISE_BASE + 0x44), 1.25);
        assert_eq!(s.e.global::<f32>(SHADER_NOISE_BASE + 0x48), 2.25);
        assert_eq!(s.e.global::<f32>(SHADER_NOISE_BASE + 0x4c), 3.25);
        // The noise map is rendered and the form is marked as done.
        assert_eq!(
            s.calls(IMAGE_SPACE_RENDER_NOISE),
            vec![vec![
                0x0ee0_0000,
                0x21,
                s.renderer,
                0xbeef,
                FORM_NOISE_TEXTURE,
                0,
                1
            ]]
        );
        assert_eq!(
            s.calls(WATER_TYPE_MAP_SET),
            vec![vec![this + 0x6c, form, 1]]
        );
        assert_eq!(s.calls(WATER_TYPE_MAP_FIND)[0][..2], [this + 0x6c, form]);
    }

    #[test]
    fn a_form_already_updated_this_frame_is_not_scrolled_again() {
        let mut s = form_scene();
        s.e.mem.set_u8(MAP_FIND_ANSWER, 1);
        s.update_forms();
        assert_eq!(s.e.mem.f32(s.water_form + 0x34), 0.2);
        assert!(s.calls(IMAGE_SPACE_RENDER_NOISE).is_empty());
        assert!(s.calls(WATER_TYPE_MAP_SET).is_empty());
        // The property still shares the rendered noise texture.
        assert_eq!(s.e.mem.u32(s.property.addr() + 0x138), FORM_NOISE_TEXTURE);
    }

    #[test]
    fn the_flush_object_is_flushed_when_its_byte_is_clear() {
        let mut s = form_scene();
        s.update_forms();
        let flush_object = s.e.global::<u32>(FLUSH_OBJECT);
        assert_eq!(s.calls(FLUSH_OBJECT_FLUSH), vec![vec![flush_object]]);
        let mut s = form_scene();
        let flush_object = s.e.global::<u32>(FLUSH_OBJECT);
        s.e.mem.set_u8(flush_object + 0x2b8, 1);
        s.update_forms();
        assert!(s.calls(FLUSH_OBJECT_FLUSH).is_empty());
    }

    #[test]
    fn references_without_a_usable_water_form_are_skipped() {
        let mut s = form_scene();
        s.e.mem.set_u32(s.base_form + BASE_FORM_WATER_FORM, 0);
        s.update_forms();
        assert!(s.calls(FLUSH_OBJECT_FLUSH).is_empty());
        assert!(s.calls(WATER_REFERENCE_3D).is_empty());

        let mut s = form_scene();
        s.e.register(WATER_FORM_HAS_NOISE_TEXTURE, |_, _| ret(0));
        s.update_forms();
        assert!(s.calls(FLUSH_OBJECT_FLUSH).is_empty());

        // A reference without a water node, or with an owner of another type.
        let mut s = form_scene();
        s.e.mem.set_u32(s.reference + REFERENCE_WATER_NODE, 0);
        s.update_forms();
        assert!(s.calls(WATER_TYPE_MAP_FIND).is_empty());
        let mut s = form_scene();
        let owner = s.e.mem.u32(s.water_node + 0xc0);
        s.e.mem.set_u32(owner + 0x68, 4);
        s.update_forms();
        assert!(s.calls(WATER_TYPE_MAP_FIND).is_empty());
    }

    #[test]
    fn the_shared_noise_map_fills_an_empty_property_slot() {
        let mut s = form_scene();
        let (this, property) = (s.this.addr(), s.property.addr());
        s.e.mem.set_u32(this + 0x1c, 0x3131);
        s.e.mem.set_u32(property + 0x134, 0);
        // The slot is filled from the system's texture; the noise map is not
        // a real texture, so the form keeps the noise map it has.
        s.e.mem.set_u8(MAP_FIND_ANSWER, 1);
        s.update_forms();
        assert_eq!(
            s.calls(NI_POINTER_ASSIGN_FROM)[0],
            vec![property + 0x134, this + 0x1c]
        );
        assert_eq!(s.e.mem.u32(property + 0x134), 0x3131);
    }

    /// The doubles the form's own noise texture loading uses: a temporary
    /// `NiPointer` stores its initial pointer, and the form's `+0x17c`
    /// texture is copied into a slot.
    fn own_noise_scene(form_texture: u32) -> Scene {
        let mut s = form_scene();
        set_setting(&mut s.e, SETTING_USE_PER_WORLD_SPACE_NOISE, false);
        s.e.register(POINTER_TEMP_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        s.e.register(WATER_FORM_COPY_NOISE_TEXTURE, |e, a| {
            let texture = e.mem.u32(a[0] + 0x17c);
            e.mem.set_u32(a[1], texture);
            ret(a[1])
        });
        s.e.register(WATER_FORM_NOISE_NAME, |_, _| ret(0x7a7a));
        let form = s.water_form;
        s.e.mem.set_u32(form + 0x17c, form_texture);
        s.e.mem.set_u8(form + 0x190, 1);
        s.e.mem.set_u32(s.property.addr() + 0x134, 0);
        s.e.mem.set_u8(MAP_FIND_ANSWER, 1);
        s
    }

    #[test]
    fn a_form_without_a_loaded_noise_texture_creates_it_from_its_file() {
        let mut s = own_noise_scene(0);
        let (property, form) = (s.property.addr(), s.water_form);
        let tes = s.e.global::<u32>(TES_POINTER);
        s.update_forms();
        assert_eq!(
            s.calls(CREATE_TEXTURE_IMAGE),
            vec![vec![tes, 0x7a7a, property + 0x134, 1, 0]]
        );
        // The reset byte is cleared and the temporary is destroyed.
        assert_eq!(s.e.mem.u8(form + 0x190), 0);
        assert_eq!(s.calls(POINTER_TEMP_DESTRUCT).len(), 1);
        assert_eq!(s.calls(POINTER_TEMP_CONSTRUCT)[0][1], 0);
    }

    #[test]
    fn a_form_with_a_loaded_noise_texture_shares_it_with_the_property() {
        let mut s = own_noise_scene(0x4444);
        let (property, form) = (s.property.addr(), s.water_form);
        s.update_forms();
        assert!(s.calls(CREATE_TEXTURE_IMAGE).is_empty());
        assert_eq!(s.e.mem.u32(property + 0x134), 0x4444);
        assert_eq!(s.e.mem.u8(form + 0x190), 0);
    }

    #[test]
    fn a_property_with_a_noise_map_is_left_alone_unless_the_form_asks_for_a_reset() {
        let mut s = own_noise_scene(0x4444);
        let (property, form) = (s.property.addr(), s.water_form);
        s.e.mem.set_u32(property + 0x134, 0xbeef);
        s.e.mem.set_u8(form + 0x190, 0);
        s.update_forms();
        assert!(s.calls(POINTER_TEMP_CONSTRUCT).is_empty());
        assert_eq!(s.e.mem.u32(property + 0x134), 0xbeef);
        // With the reset byte set the texture is loaded again.
        s.e.mem.set_u8(form + 0x190, 1);
        s.update_forms();
        assert_eq!(s.calls(POINTER_TEMP_CONSTRUCT).len(), 1);
        assert_eq!(s.e.mem.u32(property + 0x134), 0x4444);
    }

    #[test]
    fn a_form_without_a_rendered_noise_texture_gets_one_the_size_of_the_noise_map() {
        let mut s = form_scene();
        let (property, form) = (s.property.addr(), s.water_form);
        let texture = object(&mut s.e, TEXTURE_VTABLE, 0x40);
        s.e.mem.set_u32(property + 0x134, texture);
        s.e.mem.set_u32(form + 0x30, 0);
        s.e.register(CREATE_RENDERED_TEXTURE, |_, _| ret(0x9999));
        s.e.mem.set_u8(MAP_FIND_ANSWER, 1);
        s.update_forms();
        assert_eq!(s.e.global::<u32>(NOISE_WIDTH), 512);
        assert_eq!(s.e.global::<u32>(NOISE_HEIGHT), 256);
        assert_eq!(
            s.calls(CREATE_RENDERED_TEXTURE),
            vec![vec![s.texture_manager, s.renderer, 0x13, 0, 0, 0]]
        );
        assert_eq!(s.e.mem.u32(form + 0x30), 0x9999);
        assert_eq!(s.e.mem.u32(property + 0x138), 0x9999);
    }

    // =======================================================================
    // Second session: the functions from `004e4730` on.
    // =======================================================================
    mod second_session {
        use super::*;

        pub(super) fn word(value: f32) -> u32 {
            value.to_bits()
        }

        pub(super) fn float_arg(a: &[u32], index: usize) -> f32 {
            f32::from_bits(a[index])
        }

        /// Vtables and doubles of the second batch.
        const REFERENCE2_VTABLE: u32 = 0x0200_7000;
        const NODE2_VTABLE: u32 = 0x0200_8000;
        const IS_ACTOR_DOUBLE: u32 = 0x0300_0101;
        const SKIP_ADJUST_DOUBLE: u32 = 0x0300_0102;
        const SET_3D_DOUBLE: u32 = 0x0300_0103;
        const LOCATION_DOUBLE: u32 = 0x0300_0104;
        const ATTACH_CHILD_DOUBLE: u32 = 0x0300_0105;
        const DETACH_CHILD_DOUBLE: u32 = 0x0300_0106;
        const UPDATE_WORLD_DATA_DOUBLE: u32 = 0x0300_0107;
        const IS_NODE_DOUBLE: u32 = 0x0300_0108;

        // Fields of the test reference (the doubles read them).
        /// The 3D node `Get3D` returns.
        pub(super) const REF_NODE: u32 = 0x90;
        /// The node `004e8030` returns.
        pub(super) const REF_WATER_NODE: u32 = 0x94;
        /// Answer of `IsActor`.
        pub(super) const REF_IS_ACTOR: u32 = 0x9c;
        /// Answer of the `+0x160` virtual.
        pub(super) const REF_SKIP_ADJUST: u32 = 0xa0;
        /// Set by `Set3D`: the node and the flag.
        pub(super) const REF_SET_3D_NODE: u32 = 0xa4;
        pub(super) const REF_SET_3D_FLAG: u32 = 0xa8;
        /// The location `GetLocationOnReference` returns (a point inside the
        /// object).
        pub(super) const REF_LOCATION: u32 = 0xb0;
        /// The base form, the cell (at +0x20 and +0x40 as in `water_engine`).
        pub(super) const REF_BASE_FORM: u32 = 0x20;
        pub(super) const REF_CELL: u32 = 0x40;
        /// The form flags and water form of the test base form.
        const FORM_FLAGS: u32 = BASE_FORM_FLAGS;
        const FORM_WATER: u32 = BASE_FORM_WATER_FORM;

        fn register_math(e: &mut Engine) {
            e.register(NI_POINT3_CONSTRUCT, |e, a| {
                for i in 0..3 {
                    e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
                }
                ret(a[0])
            });
            e.register(NI_PLANE_CONSTRUCT, |e, a| {
                let normal: Vec<f32> = (0..3).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
                let point: Vec<f32> = (0..3).map(|i| e.mem.f32(a[2] + 4 * i)).collect();
                for i in 0..3 {
                    e.mem.set_f32(a[0] + 4 * i, normal[i as usize]);
                }
                let constant = normal[0] * point[0] + normal[1] * point[1] + normal[2] * point[2];
                e.mem.set_f32(a[0] + 12, constant);
                ret(a[0])
            });
            e.register(MATRIX_TIMES_POINT, |e, a| {
                let v: Vec<f32> = (0..3).map(|i| e.mem.f32(a[2] + 4 * i)).collect();
                for row in 0..3 {
                    let m: Vec<f32> = (0..3).map(|i| e.mem.f32(a[0] + 12 * row + 4 * i)).collect();
                    e.mem
                        .set_f32(a[1] + 4 * row, m[0] * v[0] + m[1] * v[1] + m[2] * v[2]);
                }
                ret(a[1])
            });
            e.register(POINT3_UNITIZE, |e, a| {
                let v: Vec<f32> = (0..3).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
                let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
                for i in 0..3 {
                    let value = if length > 1e-6 {
                        v[i as usize] / length
                    } else {
                        0.0
                    };
                    e.mem.set_f32(a[0] + 4 * i, value);
                }
                Ret::default()
            });
            e.register(POINTS_NEAR, |e, a| {
                let tolerance = float_arg(a, 2);
                ret((0..3)
                    .all(|i| (e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[1] + 4 * i)).abs() <= tolerance)
                    as u32)
            });
            e.register(FLOATS_NEAR, |_, a| {
                ret(((float_arg(a, 0) - float_arg(a, 1)).abs() <= float_arg(a, 2)) as u32)
            });
            e.register(ADDRESS_OF_THIS, |_, a| ret(a[0]));
            e.register(PLANE_CONSTANT, |e, a| e.mem.f32(a[0] + 0xc).into_ret());
            e.register(SETTING_FLOAT_VALUE, |_, a| ret(a[0] + 4));
            e.register(SETTING_INT_VALUE, |_, a| ret(a[0] + 4));
        }

        /// A `NiTPointerList` with the operations the second batch uses,
        /// on the layout of `list()`: head, tail, count; nodes {next, prev,
        /// item}.
        fn register_list_operations(e: &mut Engine) {
            e.register(LIST_ADD_HEAD, |e, a| {
                let item = e.mem.u32(a[1]);
                let node = e.mem.alloc(12);
                let head = e.mem.u32(a[0]);
                e.mem.set_u32(node, head);
                e.mem.set_u32(node + 8, item);
                if head != 0 {
                    e.mem.set_u32(head + 4, node);
                } else {
                    e.mem.set_u32(a[0] + 4, node);
                }
                e.mem.set_u32(a[0], node);
                let count = e.mem.u32(a[0] + 8);
                e.mem.set_u32(a[0] + 8, count + 1);
                Ret::default()
            });
            e.register(LIST_ADD_TAIL, |e, a| {
                let item = e.mem.u32(a[1]);
                let node = e.mem.alloc(12);
                let tail = e.mem.u32(a[0] + 4);
                e.mem.set_u32(node + 4, tail);
                e.mem.set_u32(node + 8, item);
                if tail != 0 {
                    e.mem.set_u32(tail, node);
                } else {
                    e.mem.set_u32(a[0], node);
                }
                e.mem.set_u32(a[0] + 4, node);
                let count = e.mem.u32(a[0] + 8);
                e.mem.set_u32(a[0] + 8, count + 1);
                Ret::default()
            });
            e.register(LIST_INSERT_BEFORE, |e, a| {
                let item = e.mem.u32(a[2]);
                let node = e.mem.alloc(12);
                let previous = e.mem.u32(a[1] + 4);
                e.mem.set_u32(node, a[1]);
                e.mem.set_u32(node + 4, previous);
                e.mem.set_u32(node + 8, item);
                if previous != 0 {
                    e.mem.set_u32(previous, node);
                } else {
                    e.mem.set_u32(a[0], node);
                }
                e.mem.set_u32(a[1] + 4, node);
                let count = e.mem.u32(a[0] + 8);
                e.mem.set_u32(a[0] + 8, count + 1);
                Ret::default()
            });
            e.register(LIST_REMOVE_POSITION, |e, a| {
                let node = e.mem.u32(a[1]);
                let next = e.mem.u32(node);
                let previous = e.mem.u32(node + 4);
                if previous != 0 {
                    e.mem.set_u32(previous, next);
                } else {
                    e.mem.set_u32(a[0], next);
                }
                if next != 0 {
                    e.mem.set_u32(next + 4, previous);
                } else {
                    e.mem.set_u32(a[0] + 4, previous);
                }
                let count = e.mem.u32(a[0] + 8);
                e.mem.set_u32(a[0] + 8, count - 1);
                e.mem.set_u32(a[1], next);
                Ret::default()
            });
            e.register(LIST_FIND_POSITION, |e, a| {
                let wanted = e.mem.u32(a[1]);
                let mut node = if a[2] != 0 { a[2] } else { e.mem.u32(a[0]) };
                while node != 0 {
                    if e.mem.u32(node + 8) == wanted {
                        return ret(node);
                    }
                    node = e.mem.u32(node);
                }
                ret(0)
            });
            e.register(LIST_NEXT_ITEM, |e, a| {
                let node = e.mem.u32(a[1]);
                let next = e.mem.u32(node);
                e.mem.set_u32(a[1], next);
                ret(node + 8)
            });
            e.register(OPERATOR_NEW, |e, a| {
                let block = e.mem.alloc(a[0]);
                ret(block)
            });
            e.register(OPERATOR_DELETE, |_, _| Ret::default());
        }

        /// The engine for this batch: the shared doubles of the first
        /// session plus math, list and reference-virtual doubles.
        pub(super) fn engine() -> Engine {
            let mut e = water_engine();
            // `water_engine` stands in for the range test (`004e62e0`); the real
            // one is under test here.
            for (address, function) in funcs() {
                if address == REFERENCE_IS_IN_RANGE {
                    e.register(address, function);
                }
            }
            for page in [
                0x0101_1000,
                0x0101_3000,
                0x0101_5000,
                0x0101_a000,
                0x0101_f000,
                0x0102_0000,
                0x0118_9000,
                0x011f_4000,
                0x0120_0000,
                0x0203_0000,
            ] {
                e.map(page, 0x1000);
            }
            e.set_global(TWO, 2.0f64);
            e.set_global(ONE, 1.0f64);
            e.set_global(ZERO, 0.0f64);
            e.set_global(MINUS_ONE_DOUBLE, -1.0f64);
            e.set_global(TEN, 10.0f64);
            e.set_global(TEN_FLOAT, 10.0f32);
            e.set_global(LOWEST_FLOAT, f32::MIN);
            e.set_global(LENGTH_LIMIT, 1.0e-6f64);
            e.set_global(ZERO_POINT_ONE, 0.1f32 as f64);
            e.set_global(PLANE_TOLERANCE, 0.01f32);
            e.set_global(FLOAT_THRESHOLD_011AD834, 1.0f32);
            // The identity matrix.
            for i in 0..9 {
                e.set_global(
                    IDENTITY_MATRIX + 4 * i,
                    if i % 4 == 0 { 1.0f32 } else { 0.0 },
                );
            }
            register_math(&mut e);
            register_list_operations(&mut e);
            // Reference virtuals.
            e.register(IS_ACTOR_DOUBLE, |e, a| {
                ret(e.mem.u8(a[0] + REF_IS_ACTOR) as u32)
            });
            e.register(SKIP_ADJUST_DOUBLE, |e, a| {
                ret(e.mem.u8(a[0] + REF_SKIP_ADJUST) as u32)
            });
            e.register(SET_3D_DOUBLE, |e, a| {
                e.mem.set_u32(a[0] + REF_SET_3D_NODE, a[1]);
                e.mem.set_u32(a[0] + REF_SET_3D_FLAG, a[2]);
                Ret::default()
            });
            e.register(LOCATION_DOUBLE, |_, a| ret(a[0] + REF_LOCATION));
            vtable(
                &mut e,
                REFERENCE2_VTABLE,
                &[
                    (REFERENCE_GET_3D, REFERENCE_GET_3D_DOUBLE),
                    (REFERENCE_IS_ACTOR_VIRTUAL, IS_ACTOR_DOUBLE),
                    (REFERENCE_SKIP_ADJUST_VIRTUAL, SKIP_ADJUST_DOUBLE),
                    (REFERENCE_SET_3D_VIRTUAL, SET_3D_DOUBLE),
                    (REFERENCE_GET_LOCATION_VIRTUAL, LOCATION_DOUBLE),
                ],
            );
            // Node virtuals: the calls are logged, nothing happens.
            for double in [
                ATTACH_CHILD_DOUBLE,
                DETACH_CHILD_DOUBLE,
                UPDATE_WORLD_DATA_DOUBLE,
            ] {
                e.register(double, |_, _| Ret::default());
            }
            e.register(IS_NODE_DOUBLE, |_, a| ret(a[0]));
            vtable(
                &mut e,
                NODE2_VTABLE,
                &[
                    (NODE_ATTACH_CHILD, ATTACH_CHILD_DOUBLE),
                    (NODE_DETACH_CHILD, DETACH_CHILD_DOUBLE),
                    (NODE_UPDATE_WORLD_DATA, UPDATE_WORLD_DATA_DOUBLE),
                    (NODE_IS_NODE_VIRTUAL, IS_NODE_DOUBLE),
                ],
            );
            e
        }

        /// A reference with the second vtable, its base form, no node.
        pub(super) fn reference(e: &mut Engine) -> u32 {
            let reference = object(e, REFERENCE2_VTABLE, 0x100);
            let base_form = object(e, BASE_FORM_VTABLE, 0x40);
            e.mem.set_u32(reference + REF_BASE_FORM, base_form);
            reference
        }

        pub(super) fn node(e: &mut Engine) -> u32 {
            let node = object(e, NODE2_VTABLE, 0x200);
            // World translate (+0x8c) and rotation (+0x68, identity).
            for i in 0..9 {
                e.mem
                    .set_f32(node + 0x68 + 4 * i, if i % 4 == 0 { 1.0 } else { 0.0 });
            }
            node
        }

        /// A water group at `height` with the up normal and `water_type`,
        /// not yet in any list.
        pub(super) fn group(e: &mut Engine, height: f32, water_type: u32) -> u32 {
            let group = e.mem.alloc(0xb0);
            e.mem.set_u32(group, water_type);
            e.mem.set_f32(group + 0x04 + 8, 1.0);
            e.mem.set_f32(group + 0x04 + 12, height);
            group
        }

        pub(super) fn list_items(e: &Engine, list: u32) -> Vec<u32> {
            let mut items = vec![];
            let mut node = e.mem.u32(list);
            while node != 0 {
                items.push(e.mem.u32(node + 8));
                node = e.mem.u32(node);
            }
            items
        }

        // --- the small accessors --------------------------------------------

        #[test]
        fn scalar_deleting_destructor_runs_the_destructor_and_frees_on_bit_zero() {
            let mut e = engine();
            e.register(GROUP_DESTRUCT, |_, _| Ret::default());
            start_log(&mut e);
            assert_eq!(e.call(0x004e_52c0, &args![0x1234u32, 0u32]).u32(), 0x1234);
            assert_eq!(calls(&e, GROUP_DESTRUCT), vec![vec![0x1234]]);
            assert!(calls(&e, OPERATOR_DELETE).is_empty());
            assert_eq!(e.call(0x004e_52c0, &args![0x1234u32, 1u32]).u32(), 0x1234);
            assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x1234]]);
        }

        #[test]
        fn pointer_member_at_0xf8_is_read() {
            let mut e = engine();
            let object = e.mem.alloc(0x120);
            e.mem.set_u32(object + 0xf8, 0x4321);
            assert_eq!(e.call(0x004e_6540, &args![object]).u32(), 0x4321);
        }

        #[test]
        fn word_member_at_0x110_is_read() {
            let mut e = engine();
            let object = e.mem.alloc(0x120);
            e.mem.set_u16(object + 0x110, 0xbeef);
            assert_eq!(e.call(0x004e_6560, &args![object]).u32() & 0xffff, 0xbeef);
        }

        #[test]
        fn float_pair_test_needs_the_threshold_and_a_small_second_value() {
            let mut e = engine();
            let object = e.mem.alloc(0x120);
            let check = |e: &mut Engine, high: f32, low: f32| {
                e.mem.set_f32(object + 0xdc, high);
                e.mem.set_f32(object + 0xd8, low);
                e.call(0x004e_6580, &args![object]).bool()
            };
            assert!(check(&mut e, 1.0, 0.1));
            assert!(check(&mut e, 2.0, 0.0));
            assert!(!check(&mut e, 0.5, 0.0), "below the threshold");
            assert!(!check(&mut e, 2.0, 0.2), "second value too large");
            assert!(!check(&mut e, f32::NAN, 0.0));
            assert!(!check(&mut e, 2.0, f32::NAN));
        }

        #[test]
        fn render_object_global_is_returned() {
            let mut e = engine();
            e.set_global(WATER_RENDER_OBJECT, 0x1111u32);
            assert_eq!(e.call(0x004e_69d0, &[]).u32(), 0x1111);
        }

        #[test]
        fn water_object_global_011ffff8_is_returned() {
            let mut e = engine();
            e.set_global(WATER_OBJECT_011FFFF8, 0x2222u32);
            assert_eq!(e.call(0x004e_6a60, &[]).u32(), 0x2222);
        }

        #[test]
        fn water_fft_effect_global_is_returned() {
            let mut e = engine();
            e.set_global(WATER_FFT_EFFECT, 0x3333u32);
            assert_eq!(e.call(0x004e_6a70, &[]).u32(), 0x3333);
        }

        #[test]
        fn water_root_slot_is_read_through_the_pointer_accessor() {
            let mut e = engine();
            e.set_global(WATER_ROOT_SLOT, 0x4444u32);
            assert_eq!(e.call(0x004e_7ff0, &[]).u32(), 0x4444);
        }

        /// One of the four slot setters: assigns `offset` of the object.
        fn check_slot_setter(address: u32, offset: u32) {
            let mut e = engine();
            let object = e.mem.alloc(0x260);
            e.mem.set_u32(object + offset, 0x1111);
            e.call(address, &args![object, 0x7000_0000u32 + offset]);
            assert_eq!(e.mem.u32(object + offset), 0x7000_0000 + offset);
            // The other slots stay.
            for other in [0x238u32, 0x23c, 0x248, 0x24c] {
                if other != offset {
                    assert_eq!(e.mem.u32(object + other), 0);
                }
            }
        }

        #[test]
        fn slot_setter_at_0x23c() {
            check_slot_setter(0x004e_69e0, 0x23c);
        }

        #[test]
        fn slot_setter_at_0x238() {
            check_slot_setter(0x004e_6a00, 0x238);
        }

        #[test]
        fn slot_setter_at_0x248() {
            check_slot_setter(0x004e_6a20, 0x248);
        }

        #[test]
        fn slot_setter_at_0x24c() {
            check_slot_setter(0x004e_6a40, 0x24c);
        }

        #[test]
        fn point2_divide_divides_both_floats() {
            let mut e = engine();
            let point = e.mem.alloc(8);
            e.mem.set_f32(point, 3.0);
            e.mem.set_f32(point + 4, -9.0);
            assert_eq!(e.call(0x004e_7530, &args![point, 3.0f32]).u32(), point);
            assert_eq!(e.mem.f32(point), 1.0);
            assert_eq!(e.mem.f32(point + 4), -3.0);
        }

        #[test]
        fn point2_unitize_scales_by_the_inverse_length() {
            let mut e = engine();
            e.register(POINT2_LENGTH, |e, a| {
                (e.mem.f32(a[0]).powi(2) + e.mem.f32(a[0] + 4).powi(2))
                    .sqrt()
                    .into_ret()
            });
            let point = e.mem.alloc(8);
            e.mem.set_f32(point, 3.0);
            e.mem.set_f32(point + 4, 4.0);
            assert_eq!(e.call(0x004e_7560, &args![point]).f32(), 5.0);
            assert!((e.mem.f32(point) - 0.6).abs() < 1e-6);
            assert!((e.mem.f32(point + 4) - 0.8).abs() < 1e-6);
            // A point that is too short becomes zero.
            e.mem.set_f32(point, 1e-8);
            e.mem.set_f32(point + 4, 0.0);
            assert_eq!(e.call(0x004e_7560, &args![point]).f32(), 0.0);
            assert_eq!(e.mem.f32(point), 0.0);
            assert_eq!(e.mem.f32(point + 4), 0.0);
        }

        #[test]
        fn nine_words_are_copied_from_offset_0x44() {
            let mut e = engine();
            let source = e.mem.alloc(0x80);
            let out = e.mem.alloc(0x24);
            for i in 0..9 {
                e.mem.set_u32(source + 0x44 + 4 * i, 100 + i);
            }
            assert_eq!(e.call(0x004e_75d0, &args![source, out]).u32(), out);
            for i in 0..9 {
                assert_eq!(e.mem.u32(out + 4 * i), 100 + i);
            }
        }

        #[test]
        fn near_water_radius_setting_is_read() {
            let mut e = engine();
            e.set_global(SETTING_NEAR_WATER_RADIUS + 4, 600u32);
            assert_eq!(e.call(0x004e_7600, &args![0u32]).u32(), 600);
        }

        #[test]
        fn near_water_points_setting_is_read() {
            let mut e = engine();
            e.set_global(SETTING_NEAR_WATER_POINTS + 4, 9u32);
            assert_eq!(e.call(0x004e_7620, &args![0u32]).u32(), 9);
        }

        #[test]
        fn near_water_indoor_tolerance_setting_is_read() {
            let mut e = engine();
            e.set_global(SETTING_NEAR_WATER_INDOOR_TOLERANCE + 4, 3.5f32);
            assert_eq!(e.call(0x004e_7640, &args![0u32]).f32(), 3.5);
        }

        #[test]
        fn near_water_outdoor_tolerance_setting_is_read() {
            let mut e = engine();
            e.set_global(SETTING_NEAR_WATER_OUTDOOR_TOLERANCE + 4, 7.25f32);
            assert_eq!(e.call(0x004e_7660, &args![0u32]).f32(), 7.25);
        }

        #[test]
        fn tri_shape_is_built_in_the_allocation_scope() {
            let mut e = engine();
            e.register(ALLOCATION_SCOPE_CONSTRUCT, |_, a| ret(a[0]));
            e.register(ALLOCATION_SCOPE_DESTRUCT, |_, _| Ret::default());
            e.register(NI_ALLOC, |e, a| ret(e.mem.alloc(a[0])));
            e.register(TRI_SHAPE_CONSTRUCT, |_, a| ret(a[0] + 1));
            start_log(&mut e);
            let shape = e.call(0x004e_7680, &args![0u32, 0x5555u32]).u32();
            assert_ne!(shape, 0);
            let construct = calls(&e, TRI_SHAPE_CONSTRUCT);
            assert_eq!(construct.len(), 1);
            assert_eq!(construct[0][1], 0x5555);
            assert_eq!(shape, construct[0][0] + 1);
            assert_eq!(calls(&e, NI_ALLOC), vec![vec![0xc4]]);
            let scope = calls(&e, ALLOCATION_SCOPE_CONSTRUCT);
            assert_eq!(scope.len(), 1);
            assert_eq!(&scope[0][1..], &[0x1d, 1, TESWATER_SOURCE_PATH, 0xb91]);
            assert_eq!(
                calls(&e, ALLOCATION_SCOPE_DESTRUCT),
                vec![vec![scope[0][0]]]
            );
            // A failed allocation gives null.
            e.register(NI_ALLOC, |_, _| ret(0));
            assert_eq!(e.call(0x004e_7680, &args![0u32, 0x5555u32]).u32(), 0);
        }

        /// Registers doubles that do nothing and return 0.
        pub(super) fn quiet(e: &mut Engine, addresses: &[u32]) {
            for address in addresses {
                e.register(*address, |_, _| Ret::default());
            }
        }

        /// The current world space (`TES + 0x50`) with the water height at
        /// `+0x7c`.
        fn world_space(e: &mut Engine, height: f32) -> u32 {
            let world_space = e.mem.alloc(0x100);
            e.mem.set_f32(world_space + 0x7c, height);
            let tes = e.global::<u32>(TES_POINTER);
            e.mem.set_u32(tes + 0x50, world_space);
            e.register(TES_GET_WORLD_SPACE, |e, a| ret(e.mem.u32(a[0] + 0x50)));
            e.register(WORLD_SPACE_WATER_HEIGHT, |e, a| {
                e.mem.f32(a[0] + 0x7c).into_ret()
            });
            e.register(WORLD_SPACE_FLAG_10, |e, a| {
                ret((e.mem.u8(a[0] + 0x4c) & 0x10 != 0) as u32)
            });
            world_space
        }

        // --- AddPlaceableWater_ov2 ------------------------------------------

        struct Adding {
            e: Engine,
            system: u32,
            reference: u32,
            node: u32,
            base_form: u32,
            position: u32,
        }

        /// A system without groups and a reference that sits at height
        /// `height` (x 1, y 2) with water type `0x7777`, with the group
        /// plane tolerance 10.
        fn adding(height: f32) -> Adding {
            let mut e = engine();
            set_setting(&mut e, SETTING_USE_WATER, true);
            e.set_global(SETTING_WATER_GROUP_HEIGHT_RANGE + 4, 10.0f32);
            register_list_operations(&mut e);
            world_space(&mut e, height);
            quiet(
                &mut e,
                &[
                    LOG_MESSAGE,
                    NODE_SET_LOCAL_TRANSLATE,
                    REFERENCE_SET_LOCATION,
                    UPDATE_DATA_CONSTRUCT,
                    CELL_SET_WATER_HEIGHT,
                    NODE_SET_CULLED,
                ],
            );
            e.register(GROUP_CONSTRUCT, |_, a| ret(a[0]));
            e.register(NODE_NAME_SLOT, |_, a| ret(a[0] + 8));
            e.register(FIXED_STRING_TEXT, |_, _| ret(0x0abc_0000));
            let system = e.mem.alloc(0xa0);
            let reference = reference(&mut e);
            let base_form = e.mem.u32(reference + REF_BASE_FORM);
            e.mem.set_u32(base_form + FORM_WATER, 0x7777);
            let node = node(&mut e);
            e.mem.set_u32(reference + REF_NODE, node);
            for (i, value) in [1.0f32, 2.0, height].into_iter().enumerate() {
                e.mem.set_f32(node + 0x8c + 4 * i as u32, value);
            }
            let position = e.mem.alloc(12);
            for (i, value) in [1.0f32, 2.0, height].into_iter().enumerate() {
                e.mem.set_f32(position + 4 * i as u32, value);
            }
            Adding {
                e,
                system,
                reference,
                node,
                base_form,
                position,
            }
        }

        impl Adding {
            fn add(&mut self) -> bool {
                start_log(&mut self.e);
                let (system, reference, position) = (self.system, self.reference, self.position);
                self.e
                    .call(
                        0x004e_4730,
                        &args![system, reference, position, IDENTITY_MATRIX],
                    )
                    .bool()
            }

            fn groups(&self) -> Vec<u32> {
                list_items(&self.e, self.system + 0x3c)
            }
        }

        #[test]
        fn add_placeable_water_does_nothing_while_water_is_off() {
            let mut a = adding(50.0);
            set_setting(&mut a.e, SETTING_USE_WATER, false);
            assert!(!a.add());
            assert!(a.groups().is_empty());
            assert!(calls(&a.e, NI_PLANE_CONSTRUCT).is_empty());
        }

        #[test]
        fn add_placeable_water_creates_a_group_for_the_first_reference() {
            let mut a = adding(50.0);
            assert!(a.add());
            let groups = a.groups();
            assert_eq!(groups.len(), 1);
            let group = groups[0];
            // The group got the up plane at height 50, the down plane, the
            // water type of the base form and the reference.
            assert_eq!(a.e.mem.u32(group), 0x7777);
            assert_eq!(a.e.mem.f32(group + 4 + 8), 1.0);
            assert_eq!(a.e.mem.f32(group + 4 + 12), 50.0);
            assert_eq!(a.e.mem.f32(group + 0x14 + 8), -1.0);
            assert_eq!(a.e.mem.f32(group + 0x14 + 12), -50.0);
            assert_eq!(list_items(&a.e, group + 0x24), vec![a.reference]);
            // It sits at the world space water height.
            assert_eq!(a.e.mem.u8(group + 0x5c), 1);
            // The water system was enabled and the stencil bits reset.
            assert_eq!(a.e.global::<u8>(WATER_ENABLED), 1);
            assert_eq!(a.e.global::<u8>(WATER_REQUEST_FLAG), 1);
            assert_eq!(a.e.mem.u32(group + 0xac), 2);
            assert_eq!(calls(&a.e, OPERATOR_NEW), vec![vec![0xb0]]);
        }

        #[test]
        fn a_group_away_from_the_world_height_is_not_marked() {
            let mut a = adding(50.0);
            world_space(&mut a.e, 10.0);
            assert!(a.add());
            let group = a.groups()[0];
            assert_eq!(a.e.mem.u8(group + 0x5c), 0);
        }

        #[test]
        fn an_interior_group_is_not_compared_with_the_world_space() {
            let mut a = adding(50.0);
            let tes = a.e.global::<u32>(TES_POINTER);
            a.e.mem.set_u32(tes + 0x34, 0x1234);
            assert!(a.add());
            let group = a.groups()[0];
            assert_eq!(a.e.mem.u8(group + 0x5c), 0);
            assert!(calls(&a.e, TES_GET_WORLD_SPACE).is_empty());
        }

        #[test]
        fn a_reference_joins_a_group_with_the_same_plane_and_water_type() {
            let mut a = adding(50.0);
            let existing = group(&mut a.e, 50.0, 0x7777);
            fill_list(&mut a.e, a.system + 0x3c, &[existing]);
            assert!(a.add());
            assert_eq!(a.groups(), vec![existing]);
            assert_eq!(list_items(&a.e, existing + 0x24), vec![a.reference]);
            assert!(calls(&a.e, OPERATOR_NEW).is_empty());
            // The heights agree: the reference is not moved.
            assert!(calls(&a.e, REFERENCE_SET_LOCATION).is_empty());
        }

        #[test]
        fn a_reference_slightly_off_the_group_height_is_moved_to_it() {
            let mut a = adding(50.05);
            let existing = group(&mut a.e, 50.0, 0x7777);
            fill_list(&mut a.e, a.system + 0x3c, &[existing]);
            let cell = a.e.mem.alloc(0x40);
            let reference = a.reference;
            a.e.mem.set_u32(reference + REF_CELL, cell);
            assert!(a.add());
            assert_eq!(list_items(&a.e, existing + 0x24), vec![a.reference]);
            // The new position keeps x and y and takes the group height.
            let location = calls(&a.e, REFERENCE_SET_LOCATION);
            assert_eq!(location.len(), 1);
            assert_eq!(location[0][0], a.reference);
            let moved = location[0][1];
            assert_eq!(a.e.mem.f32(moved), 1.0);
            assert_eq!(a.e.mem.f32(moved + 4), 2.0);
            assert_eq!(a.e.mem.f32(moved + 8), 50.0);
            assert_eq!(
                calls(&a.e, NODE_SET_LOCAL_TRANSLATE),
                vec![vec![a.node, moved]]
            );
            // The cell's water height follows, and the move is logged.
            assert_eq!(
                calls(&a.e, CELL_SET_WATER_HEIGHT),
                vec![vec![cell, word(50.0)]]
            );
            let log = calls(&a.e, LOG_MESSAGE);
            assert_eq!(log.len(), 1);
            let old_height = (50.05f32 as f64).to_bits();
            let new_height = (50.0f32 as f64).to_bits();
            assert_eq!(
                log[0],
                vec![
                    ADJUSTING_HEIGHT_MESSAGE,
                    0x0abc_0000,
                    old_height as u32,
                    (old_height >> 32) as u32,
                    new_height as u32,
                    (new_height >> 32) as u32
                ]
            );
            // The world data of the node is updated through its virtual.
            assert_eq!(calls(&a.e, UPDATE_WORLD_DATA_DOUBLE).len(), 1);
        }

        #[test]
        fn the_skip_virtual_keeps_the_reference_where_it_is() {
            let mut a = adding(50.05);
            let existing = group(&mut a.e, 50.0, 0x7777);
            fill_list(&mut a.e, a.system + 0x3c, &[existing]);
            let reference = a.reference;
            a.e.mem.set_u8(reference + REF_SKIP_ADJUST, 1);
            assert!(a.add());
            assert_eq!(list_items(&a.e, existing + 0x24), vec![a.reference]);
            assert!(calls(&a.e, REFERENCE_SET_LOCATION).is_empty());
            assert!(calls(&a.e, LOG_MESSAGE).is_empty());
        }

        #[test]
        fn a_group_with_another_water_type_is_not_joined() {
            let mut a = adding(50.0);
            let existing = group(&mut a.e, 50.0, 0x1111);
            fill_list(&mut a.e, a.system + 0x3c, &[existing]);
            assert!(a.add());
            let groups = a.groups();
            assert_eq!(groups.len(), 2);
            assert_eq!(groups[0], existing);
            assert!(list_items(&a.e, existing + 0x24).is_empty());
            assert_eq!(a.e.mem.u32(groups[1]), 0x7777);
            // Stencil bits follow the list order.
            assert_eq!(a.e.mem.u32(groups[0] + 0xac), 2);
            assert_eq!(a.e.mem.u32(groups[1] + 0xac), 4);
        }

        #[test]
        fn a_new_group_goes_in_front_of_the_first_lower_group() {
            let mut a = adding(50.0);
            let high = group(&mut a.e, 100.0, 0x1111);
            let low = group(&mut a.e, 10.0, 0x1111);
            fill_list(&mut a.e, a.system + 0x3c, &[high, low]);
            assert!(a.add());
            let groups = a.groups();
            assert_eq!(groups.len(), 3);
            assert_eq!(groups[0], high);
            assert_eq!(groups[2], low);
            assert_eq!(a.e.mem.u32(groups[1]), 0x7777);
            assert_eq!(calls(&a.e, LIST_INSERT_BEFORE).len(), 1);
            assert!(calls(&a.e, LIST_ADD_TAIL).is_empty());
        }

        #[test]
        fn a_new_group_goes_last_when_no_group_is_lower() {
            let mut a = adding(50.0);
            let high = group(&mut a.e, 100.0, 0x1111);
            fill_list(&mut a.e, a.system + 0x3c, &[high]);
            assert!(a.add());
            let groups = a.groups();
            assert_eq!(groups.len(), 2);
            assert_eq!(groups[0], high);
            assert_eq!(calls(&a.e, LIST_ADD_TAIL).len(), 1);
            let _ = a.base_form;
        }

        // --- the LOD water ---------------------------------------------------

        struct Lod {
            e: Engine,
            system: u32,
            geometry: u32,
            parent: u32,
            world_space: u32,
        }

        /// The doubles `fn_004e4c80` needs, a system, a geometry node and a
        /// world space with water height 25.
        fn lod() -> Lod {
            let mut e = engine();
            set_setting(&mut e, SETTING_USE_WATER, true);
            set_setting(&mut e, SETTING_FORCE_HIGH_DETAIL_REFLECTIONS, false);
            world_space(&mut e, 40.0);
            let world_space = e.mem.alloc(0x100);
            e.mem.set_f32(world_space + 0x7c, 25.0);
            quiet(
                &mut e,
                &[
                    FORM_SET_TEMPORARY,
                    NODE_REMOVE_PROPERTY,
                    NODE_ATTACH_PROPERTY,
                    NODE_UPDATE_PROPERTIES,
                    SHADER_MANAGER_PREPARE_OBJECT,
                    AUTO_WATER_PROPERTY_SET,
                    UPDATE_DATA_CONSTRUCT,
                    NODE_UPDATE,
                    NODE_SET_CULLED,
                ],
            );
            e.register(NI_ALLOC, |e, a| ret(e.mem.alloc(a[0])));
            e.register(REFERENCE_CONSTRUCT, |e, a| {
                e.mem.set_u32(a[0], REFERENCE2_VTABLE);
                ret(a[0])
            });
            e.register(WATER_SHADER_PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
            e.register(AUTO_WATER_PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
            e.register(FADE_NODE_CONSTRUCT, |e, a| {
                e.mem.set_u32(a[0], NODE2_VTABLE);
                ret(a[0])
            });
            e.register(GROUP_CONSTRUCT, |_, a| ret(a[0]));
            e.register(WATER_PROPERTY_TYPE, |_, _| ret(3));
            e.register(AUTO_WATER_PROPERTY_TYPE, |_, _| ret(4));
            e.register(WORLD_SPACE_WATER_TYPE, |_, a| ret(a[0] + 0x1000));
            e.register(GET_PLACEABLE_LOD_WATER, |_, a| ret(a[0] + 0x10));
            e.register(REFERENCE_SET_OBJECT_REFERENCE, |e, a| {
                // The base form of the reference is made from the water form.
                let base_form = object(e, BASE_FORM_VTABLE, 0x40);
                e.mem.set_u32(base_form + FORM_WATER, a[1]);
                e.mem.set_u32(a[0] + REF_BASE_FORM, base_form);
                Ret::default()
            });
            let system = e.mem.alloc(0xa0);
            let geometry = node(&mut e);
            let parent = node(&mut e);
            Lod {
                e,
                system,
                geometry,
                parent,
                world_space,
            }
        }

        impl Lod {
            fn create(&mut self, inner_parent: u32, full_reflections: bool) -> u32 {
                start_log(&mut self.e);
                let (system, geometry, world_space, parent) =
                    (self.system, self.geometry, self.world_space, self.parent);
                self.e
                    .call(
                        0x004e_4c80,
                        &args![
                            system,
                            geometry,
                            world_space,
                            parent,
                            inner_parent,
                            full_reflections
                        ],
                    )
                    .u32()
            }
        }

        #[test]
        fn lod_water_is_not_created_for_a_world_space_without_water() {
            let mut l = lod();
            let world_space = l.world_space;
            l.e.mem.set_u8(world_space + 0x4c, 0x10);
            assert_eq!(l.create(0, false), 0);
            assert!(calls(&l.e, OPERATOR_NEW).is_empty());
            // Nor when the current world space has none.
            let mut l = lod();
            let current = l.e.mem.u32(l.e.global::<u32>(TES_POINTER) + 0x50);
            l.e.mem.set_u8(current + 0x4c, 0x10);
            assert_eq!(l.create(0, false), 0);
        }

        #[test]
        fn lod_water_creates_the_reference_the_properties_and_the_group() {
            let mut l = lod();
            let reference = l.create(0, true);
            assert_ne!(reference, 0);
            // The reference is temporary, 0x68 bytes, with the LOD water form.
            assert_eq!(calls(&l.e, OPERATOR_NEW)[0], vec![0x68]);
            assert_eq!(calls(&l.e, FORM_SET_TEMPORARY), vec![vec![reference]]);
            let base_form = l.e.mem.u32(reference + REF_BASE_FORM);
            assert_eq!(
                l.e.mem.u32(base_form + FORM_WATER),
                l.world_space + 0x1000 + 0x10
            );
            // The water shader property: flags, type 3 removed, attached.
            let property_calls = calls(&l.e, NODE_ATTACH_PROPERTY);
            assert_eq!(property_calls.len(), 2);
            let property = property_calls[0][1];
            assert_eq!(l.e.mem.u8(property + 0x61), 1);
            assert_eq!(l.e.mem.u8(property + 0x62), 1, "full reflections");
            assert_eq!(
                calls(&l.e, NODE_REMOVE_PROPERTY),
                vec![vec![l.geometry, 3], vec![l.geometry, 4]]
            );
            assert_eq!(
                calls(&l.e, SHADER_MANAGER_PREPARE_OBJECT),
                vec![vec![l.geometry, 0, 0]]
            );
            assert_eq!(calls(&l.e, AUTO_WATER_PROPERTY_SET).len(), 1);
            assert_eq!(calls(&l.e, AUTO_WATER_PROPERTY_SET)[0][1], 3);
            // The fade node: geometry attached to it, it to the parent, and
            // it is the 3D object of the reference.
            let fade_node = l.e.mem.u32(reference + REF_SET_3D_NODE);
            assert_ne!(fade_node, 0);
            assert_eq!(l.e.mem.u32(reference + REF_SET_3D_FLAG), 1);
            let attached = calls(&l.e, ATTACH_CHILD_DOUBLE);
            assert_eq!(
                attached,
                vec![vec![fade_node, l.geometry, 1], vec![l.parent, fade_node, 1]]
            );
            // The three nodes were updated.
            let updates = calls(&l.e, NODE_UPDATE);
            assert_eq!(updates.len(), 3);
            assert_eq!(updates[0][0], l.geometry);
            assert_eq!(updates[1][0], fade_node);
            assert_eq!(updates[2][0], fade_node);
            // The LOD group: plane at the world space water height.
            let group = l.e.mem.u32(l.system + 0x48);
            assert_ne!(group, 0);
            assert_eq!(l.e.mem.f32(group + 4 + 8), 1.0);
            assert_eq!(l.e.mem.f32(group + 4 + 12), 25.0);
            assert_eq!(l.e.mem.u32(group), l.world_space + 0x1000 + 0x10);
            assert_eq!(list_items(&l.e, group + 0x24), vec![reference]);
            assert_eq!(l.e.mem.u8(group + 0x60), 1, "silhouette reflections");
            // The water system is enabled.
            assert_eq!(l.e.global::<u8>(WATER_ENABLED), 1);
        }

        #[test]
        fn lod_water_with_an_inner_parent_hangs_the_geometry_below_it() {
            let mut l = lod();
            let inner = node(&mut l.e);
            let reference = l.create(inner, false);
            let fade_node = l.e.mem.u32(reference + REF_SET_3D_NODE);
            let property = calls(&l.e, NODE_ATTACH_PROPERTY)[0][1];
            assert_eq!(l.e.mem.u8(property + 0x62), 0);
            assert_eq!(
                calls(&l.e, ATTACH_CHILD_DOUBLE),
                vec![
                    vec![fade_node, inner, 1],
                    vec![inner, l.geometry, 1],
                    vec![l.parent, fade_node, 1]
                ]
            );
            let updates = calls(&l.e, NODE_UPDATE);
            assert_eq!(updates[2][0], inner);
        }

        #[test]
        fn lod_water_with_high_detail_reflections_keeps_the_silhouette_flag_off() {
            let mut l = lod();
            set_setting(&mut l.e, SETTING_FORCE_HIGH_DETAIL_REFLECTIONS, true);
            l.create(0, false);
            let group = l.e.mem.u32(l.system + 0x48);
            assert_eq!(l.e.mem.u8(group + 0x60), 0);
        }

        #[test]
        fn lod_water_joins_the_existing_group_and_counts_the_object() {
            let mut l = lod();
            let existing = group(&mut l.e, 25.0, 0x1111);
            l.e.mem.set_u32(l.system + 0x48, existing);
            l.e.set_global(LOD_WATER_OBJECTS, 4u32);
            let reference = l.create(0, false);
            assert_eq!(l.e.mem.u32(l.system + 0x48), existing);
            assert_eq!(list_items(&l.e, existing + 0x24), vec![reference]);
            assert_eq!(l.e.global::<u32>(LOD_WATER_OBJECTS), 5);
            assert_eq!(calls(&l.e, OPERATOR_NEW).len(), 1, "only the reference");
        }

        // --- fn_004e5140: taking LOD water away -----------------------------

        /// A water reference with a water node whose property has the four
        /// slots filled (reflection map at `+0x13c`).
        fn water_reference(e: &mut Engine) -> (u32, u32, u32) {
            let reference = reference(e);
            let own_node = node(e);
            e.mem.set_u32(reference + REF_NODE, own_node);
            let water_node = node(e);
            let owner = e.mem.alloc(0x80);
            e.mem.set_u32(owner + 0x68, WATER_OWNER_TYPE);
            e.mem.set_u32(water_node + 0xc0, owner);
            let property = e.mem.alloc(0x150);
            for offset in [0x13c, 0x140, 0x138, 0x144] {
                e.mem.set_u32(property + offset, 0x0b00_0000 + offset);
            }
            e.mem.set_u32(water_node + NODE_PROPERTY, property);
            e.mem.set_u32(reference + REF_WATER_NODE, water_node);
            (reference, water_node, property)
        }

        #[test]
        fn removing_a_lod_reference_takes_it_out_of_the_group() {
            let mut e = engine();
            quiet(&mut e, &[GROUP_DESTRUCT]);
            let system = e.mem.alloc(0xa0);
            let (first, _, first_property) = water_reference(&mut e);
            let (second, _, second_property) = water_reference(&mut e);
            let lod_group = group(&mut e, 25.0, 0x1111);
            fill_list(&mut e, lod_group + 0x24, &[first, second]);
            e.mem.set_u32(system + 0x48, lod_group);
            let other_group = group(&mut e, 5.0, 0x1111);
            fill_list(&mut e, system + 0x3c, &[other_group]);
            e.set_global(LOD_WATER_OBJECTS, 2u32);
            let parent = node(&mut e);
            start_log(&mut e);
            e.call(0x004e_5140, &args![system, first, 0u32, parent]);
            // The 3D object is detached from the parent.
            let own_node = e.mem.u32(first + REF_NODE);
            assert_eq!(calls(&e, DETACH_CHILD_DOUBLE), vec![vec![parent, own_node]]);
            assert_eq!(list_items(&e, lod_group + 0x24), vec![second]);
            assert_eq!(e.global::<u32>(LOD_WATER_OBJECTS), 1);
            // Only the reflection map of the removed reference is cleared.
            assert_eq!(e.mem.u32(first_property + 0x13c), 0);
            assert_eq!(e.mem.u32(first_property + 0x140), 0x0b00_0140);
            assert_eq!(e.mem.u32(second_property + 0x13c), 0x0b00_013c);
            assert_eq!(e.mem.u32(system + 0x48), lod_group);
            assert!(calls(&e, GROUP_DESTRUCT).is_empty());
        }

        #[test]
        fn removing_the_last_lod_reference_destroys_the_group() {
            let mut e = engine();
            quiet(&mut e, &[GROUP_DESTRUCT]);
            let system = e.mem.alloc(0xa0);
            let (only, _, _) = water_reference(&mut e);
            let lod_group = group(&mut e, 25.0, 0x1111);
            fill_list(&mut e, lod_group + 0x24, &[only]);
            e.mem.set_u32(system + 0x48, lod_group);
            let other_group = group(&mut e, 5.0, 0x1111);
            fill_list(&mut e, system + 0x3c, &[other_group]);
            let parent = node(&mut e);
            start_log(&mut e);
            e.call(0x004e_5140, &args![system, only, 0u32, parent]);
            assert_eq!(e.mem.u32(system + 0x48), 0);
            assert_eq!(calls(&e, GROUP_DESTRUCT), vec![vec![lod_group]]);
            assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![lod_group]]);
            // There is another group: the water system stays enabled.
            assert!(calls(&e, SOUND_HANDLE_IS_VALID).is_empty());
        }

        #[test]
        fn removing_the_last_water_disables_the_water_system() {
            let mut e = engine();
            quiet(&mut e, &[GROUP_DESTRUCT, NODE_SET_CULLED]);
            e.register(SOUND_HANDLE_IS_VALID, |_, _| ret(0));
            let system = e.mem.alloc(0xa0);
            let (only, _, _) = water_reference(&mut e);
            let lod_group = group(&mut e, 25.0, 0x1111);
            fill_list(&mut e, lod_group + 0x24, &[only]);
            e.mem.set_u32(system + 0x48, lod_group);
            e.set_global(WATER_ENABLED, 1u8);
            e.set_global(WATER_REQUEST_FLAG, 1u8);
            e.set_global(ACTIVE_WATER_GROUPS, 3u32);
            let parent = node(&mut e);
            e.call(0x004e_5140, &args![system, only, 0u32, parent]);
            assert_eq!(e.mem.u32(system + 0x48), 0);
            assert_eq!(e.global::<u8>(WATER_ENABLED), 0);
            assert_eq!(e.global::<u8>(WATER_REQUEST_FLAG), 0);
            assert_eq!(e.global::<u32>(ACTIVE_WATER_GROUPS), 0);
        }

        #[test]
        fn a_reference_that_is_not_in_the_lod_list_is_left_alone() {
            let mut e = engine();
            let system = e.mem.alloc(0xa0);
            let (kept, _, _) = water_reference(&mut e);
            let (stranger, _, _) = water_reference(&mut e);
            let lod_group = group(&mut e, 25.0, 0x1111);
            fill_list(&mut e, lod_group + 0x24, &[kept]);
            e.mem.set_u32(system + 0x48, lod_group);
            e.set_global(LOD_WATER_OBJECTS, 1u32);
            let parent = node(&mut e);
            e.call(0x004e_5140, &args![system, stranger, 0u32, parent]);
            assert_eq!(list_items(&e, lod_group + 0x24), vec![kept]);
            assert_eq!(e.global::<u32>(LOD_WATER_OBJECTS), 1);
            // Without a LOD group only the detach happens.
            e.mem.set_u32(system + 0x48, 0);
            e.call(0x004e_5140, &args![system, stranger, 0u32, parent]);
        }

        // --- RemovePlaceableWater -------------------------------------------

        struct Removing {
            e: Engine,
            system: u32,
            reference: u32,
            property: u32,
            group: u32,
            other_group: u32,
            position: u32,
        }

        /// A system with two groups (heights 50 and 5); the reference is in
        /// the first.
        fn removing() -> Removing {
            let mut e = engine();
            set_setting(&mut e, SETTING_USE_WATER, true);
            e.set_global(WATER_ENABLED, 1u8);
            quiet(&mut e, &[GROUP_DESTRUCT]);
            let system = e.mem.alloc(0xa0);
            let (reference, _, property) = water_reference(&mut e);
            let group = group(&mut e, 50.0, 0x7777);
            let other_group = self::group(&mut e, 5.0, 0x7777);
            fill_list(&mut e, group + 0x24, &[reference]);
            fill_list(&mut e, system + 0x3c, &[group, other_group]);
            let position = e.mem.alloc(12);
            for (i, value) in [1.0f32, 2.0, 50.0].into_iter().enumerate() {
                e.mem.set_f32(position + 4 * i as u32, value);
            }
            Removing {
                e,
                system,
                reference,
                property,
                group,
                other_group,
                position,
            }
        }

        impl Removing {
            fn remove(&mut self) -> bool {
                start_log(&mut self.e);
                let (system, reference, position) = (self.system, self.reference, self.position);
                self.e
                    .call(
                        0x004e_5370,
                        &args![system, reference, position, IDENTITY_MATRIX],
                    )
                    .bool()
            }
        }

        #[test]
        fn remove_placeable_water_takes_the_reference_out_of_its_group() {
            let mut r = removing();
            // A second member keeps the group alive.
            let (second, _, _) = water_reference(&mut r.e);
            let reference = r.reference;
            fill_list(&mut r.e, r.group + 0x24, &[reference, second]);
            assert!(r.remove());
            assert_eq!(list_items(&r.e, r.group + 0x24), vec![second]);
            // The four texture slots of the removed reference were cleared.
            for offset in [0x13c, 0x140, 0x138, 0x144] {
                assert_eq!(r.e.mem.u32(r.property + offset), 0);
            }
            assert_eq!(
                list_items(&r.e, r.system + 0x3c),
                vec![r.group, r.other_group]
            );
            assert!(calls(&r.e, GROUP_DESTRUCT).is_empty());
            // The stencil bits were reassigned.
            assert_eq!(r.e.mem.u32(r.group + 0xac), 2);
            assert_eq!(r.e.mem.u32(r.other_group + 0xac), 4);
        }

        #[test]
        fn removing_the_last_reference_destroys_the_group() {
            let mut r = removing();
            assert!(r.remove());
            assert_eq!(list_items(&r.e, r.system + 0x3c), vec![r.other_group]);
            assert_eq!(calls(&r.e, GROUP_DESTRUCT), vec![vec![r.group]]);
            assert_eq!(calls(&r.e, OPERATOR_DELETE), vec![vec![r.group]]);
            assert_eq!(r.e.mem.u32(r.other_group + 0xac), 2);
        }

        #[test]
        fn removing_the_last_group_disables_the_water_system() {
            let mut r = removing();
            quiet(&mut r.e, &[NODE_SET_CULLED]);
            r.e.register(SOUND_HANDLE_IS_VALID, |_, _| ret(0));
            fill_list(&mut r.e, r.system + 0x3c, &[r.group]);
            assert!(r.remove());
            assert!(list_items(&r.e, r.system + 0x3c).is_empty());
            assert_eq!(r.e.global::<u8>(WATER_ENABLED), 0);
        }

        #[test]
        fn remove_placeable_water_needs_a_matching_group_and_enabled_water() {
            // A reference at another height matches no group.
            let mut r = removing();
            r.e.mem.set_f32(r.position + 8, 80.0);
            assert!(!r.remove());
            assert_eq!(list_items(&r.e, r.group + 0x24).len(), 1);
            // Water that is not enabled, or a setting that is off.
            let mut r = removing();
            r.e.set_global(WATER_ENABLED, 0u8);
            assert!(!r.remove());
            assert!(calls(&r.e, NI_PLANE_CONSTRUCT).is_empty());
            let mut r = removing();
            set_setting(&mut r.e, SETTING_USE_WATER, false);
            assert!(!r.remove());
            // A matching height without the reference in the group.
            let mut r = removing();
            let (stranger, _, _) = water_reference(&mut r.e);
            let (system, position) = (r.system, r.position);
            let removed =
                r.e.call(
                    0x004e_5370,
                    &args![system, stranger, position, IDENTITY_MATRIX],
                )
                .bool();
            assert!(!removed);
        }

        #[test]
        fn the_reference_wrapper_reads_the_position_and_rotation_of_its_node() {
            let mut r = removing();
            let reference = r.reference;
            let own_node = r.e.mem.u32(reference + REF_NODE);
            for (i, value) in [1.0f32, 2.0, 50.0].into_iter().enumerate() {
                r.e.mem.set_f32(own_node + 0x8c + 4 * i as u32, value);
            }
            let system = r.system;
            start_log(&mut r.e);
            r.e.call(0x004e_52f0, &args![system, reference]);
            assert_eq!(calls(&r.e, NI_PLANE_CONSTRUCT).len(), 1);
            assert_eq!(calls(&r.e, MATRIX_TIMES_POINT).len(), 1);
            assert_eq!(list_items(&r.e, system + 0x3c), vec![r.other_group]);
        }

        // --- ResetStencilBitRefs --------------------------------------------

        #[test]
        fn stencil_masks_follow_the_group_order() {
            let mut e = engine();
            let system = e.mem.alloc(0xa0);
            let groups: Vec<u32> = (0..3).map(|_| group(&mut e, 0.0, 0)).collect();
            fill_list(&mut e, system + 0x3c, &groups);
            e.call(0x004e_5640, &args![system]);
            let masks: Vec<u32> = groups.iter().map(|g| e.mem.u32(g + 0xac)).collect();
            assert_eq!(masks, vec![2, 4, 8]);
        }

        #[test]
        fn stencil_mask_shift_wraps_at_32() {
            let mut e = engine();
            let system = e.mem.alloc(0xa0);
            let groups: Vec<u32> = (0..33).map(|_| group(&mut e, 0.0, 0)).collect();
            fill_list(&mut e, system + 0x3c, &groups);
            e.call(0x004e_5640, &args![system]);
            // The 31st group gets bit 31; the 32nd wraps to bit 0.
            assert_eq!(e.mem.u32(groups[30] + 0xac), 0x8000_0000);
            assert_eq!(e.mem.u32(groups[31] + 0xac), 1);
        }

        // --- FINISH_GROUP and RELEASE_GROUP ---------------------------------

        const GROUP_OWN_MAP: u32 = 0x0c00_00cc;
        const WORLD_MAP: u32 = 0x0c00_00aa;
        const SKY_MAP: u32 = 0x0c00_00bb;

        struct Finishing {
            e: Engine,
            system: u32,
            group: u32,
            reference: u32,
            property: u32,
            wading_property: u32,
        }

        /// A group with one reference (in a cell of kind 6, with a water
        /// property) and a wading-water geometry with its own property.
        fn finishing() -> Finishing {
            let mut e = engine();
            e.register(IS_KIND_OF, |_, a| ret((a[1] != 0) as u32));
            e.set_global(WORLD_REFLECTION_MAP, WORLD_MAP);
            e.set_global(SKY_REFLECTION_MAP, SKY_MAP);
            let system = e.mem.alloc(0xa0);
            let (reference, _, property) = water_reference(&mut e);
            let cell = e.mem.alloc(0x40);
            e.mem.set_u8(cell + 0x26, 6);
            e.mem.set_u32(reference + REF_CELL, cell);
            let group = group(&mut e, 10.0, 0x1111);
            fill_list(&mut e, group + 0x24, &[reference]);
            e.mem.set_u32(group + 0x54, GROUP_OWN_MAP);
            let geometry = node(&mut e);
            let wading_property = e.mem.alloc(0x150);
            e.mem.set_u32(geometry + NODE_PROPERTY, wading_property);
            e.mem.set_u32(group + 0x58, geometry);
            Finishing {
                e,
                system,
                group,
                reference,
                property,
                wading_property,
            }
        }

        impl Finishing {
            fn finish(&mut self) {
                let (system, group) = (self.system, self.group);
                self.e.call(0x004e_56c0, &args![system, group]);
            }
        }

        #[test]
        fn finishing_a_world_height_group_shares_the_world_reflection_map() {
            let mut f = finishing();
            f.e.mem.set_u8(f.group + 0x5c, 1);
            f.finish();
            assert_eq!(f.e.mem.u32(f.property + 0x13c), WORLD_MAP);
            assert_eq!(f.e.mem.u32(f.wading_property + 0x13c), WORLD_MAP);
        }

        #[test]
        fn finishing_another_group_shares_the_sky_reflection_map() {
            let mut f = finishing();
            f.finish();
            assert_eq!(f.e.mem.u32(f.property + 0x13c), SKY_MAP);
            assert_eq!(f.e.mem.u32(f.wading_property + 0x13c), SKY_MAP);
        }

        #[test]
        fn finishing_a_group_in_an_interior_uses_its_own_map() {
            let mut f = finishing();
            let tes = f.e.global::<u32>(TES_POINTER);
            f.e.mem.set_u32(tes + 0x34, 0x1234);
            // The reference is in a cell of kind 6, so it counts.
            f.finish();
            assert_eq!(f.e.mem.u32(f.property + 0x13c), GROUP_OWN_MAP);
            assert_eq!(f.e.mem.u32(f.wading_property + 0x13c), GROUP_OWN_MAP);
        }

        #[test]
        fn a_reference_outside_kind_6_cells_needs_the_flag_and_an_exterior() {
            let mut f = finishing();
            let (reference, group) = (f.reference, f.group);
            f.e.mem.set_u32(reference + REF_CELL, 0);
            // No flag on the base form: untouched.
            f.finish();
            assert_eq!(f.e.mem.u32(f.property + 0x13c), 0x0b00_013c);
            // With the flag it is used outdoors...
            let base_form = f.e.mem.u32(reference + REF_BASE_FORM);
            f.e.mem.set_u32(base_form + FORM_FLAGS, 0x0800_0000);
            f.finish();
            assert_eq!(f.e.mem.u32(f.property + 0x13c), SKY_MAP);
            // ... but not in an interior.
            f.e.mem.set_u32(f.property + 0x13c, 0x0b00_013c);
            let tes = f.e.global::<u32>(TES_POINTER);
            f.e.mem.set_u32(tes + 0x34, 0x1234);
            f.finish();
            assert_eq!(f.e.mem.u32(f.property + 0x13c), 0x0b00_013c);
            let _ = group;
        }

        #[test]
        fn finishing_skips_properties_of_another_class_and_references_without_water() {
            let mut f = finishing();
            f.e.register(IS_KIND_OF, |_, _| ret(0));
            f.finish();
            assert_eq!(f.e.mem.u32(f.property + 0x13c), 0x0b00_013c);
            // The wading geometry is not class checked.
            assert_eq!(f.e.mem.u32(f.wading_property + 0x13c), SKY_MAP);
            // A reference without a water node.
            let mut f = finishing();
            let reference = f.reference;
            f.e.mem.set_u32(reference + REF_WATER_NODE, 0);
            f.finish();
            assert_eq!(f.e.mem.u32(f.property + 0x13c), 0x0b00_013c);
        }

        #[test]
        fn releasing_a_group_gives_back_its_map_and_clears_the_reflection_slots() {
            let mut f = finishing();
            f.e.register(TEXTURE_MANAGER, |_, _| ret(0x0aaa_0000));
            quiet(&mut f.e, &[RETURN_RENDERED_TEXTURE]);
            start_log(&mut f.e);
            let (system, group) = (f.system, f.group);
            f.e.call(0x004e_58a0, &args![system, group, 0u32, 0u32]);
            assert_eq!(
                calls(&f.e, RETURN_RENDERED_TEXTURE),
                vec![vec![0x0aaa_0000, GROUP_OWN_MAP]]
            );
            assert_eq!(f.e.mem.u32(group + 0x54), 0);
            assert_eq!(f.e.mem.u32(f.property + 0x13c), 0);
            assert_eq!(
                f.e.mem.u32(f.property + 0x140),
                0x0b00_0140,
                "only reflection"
            );
            assert_eq!(f.e.mem.u32(f.wading_property + 0x13c), 0);
        }

        #[test]
        fn releasing_a_group_without_a_map_or_with_another_owner_type() {
            let mut f = finishing();
            quiet(&mut f.e, &[RETURN_RENDERED_TEXTURE]);
            f.e.mem.set_u32(f.group + 0x54, 0);
            // The reference's node has another owner type.
            let reference = f.reference;
            let water_node = f.e.mem.u32(reference + REF_WATER_NODE);
            let owner = f.e.mem.u32(water_node + 0xc0);
            f.e.mem.set_u32(owner + 0x68, 5);
            start_log(&mut f.e);
            let (system, group) = (f.system, f.group);
            f.e.call(0x004e_58a0, &args![system, group, 0u32, 0u32]);
            assert!(calls(&f.e, RETURN_RENDERED_TEXTURE).is_empty());
            assert_eq!(f.e.mem.u32(f.property + 0x13c), 0x0b00_013c);
            // A null group does nothing at all.
            f.e.call(0x004e_58a0, &args![system, 0u32, 0u32, 0u32]);
        }

        // --- finding the group of an object ---------------------------------

        /// The group lookups: cells keep their water type at `+0x10`.
        fn lookups() -> (Engine, u32) {
            let mut e = engine();
            e.register(CELL_GET_WATER_TYPE, |e, a| ret(e.mem.u32(a[0] + 0x10)));
            let system = e.mem.alloc(0xa0);
            (e, system)
        }

        /// An object (a reference, for the lists) in a cell of water type
        /// `water_type`.
        fn object_in_cell(e: &mut Engine, water_type: u32) -> u32 {
            let object = reference(e);
            let cell = e.mem.alloc(0x40);
            e.mem.set_u32(cell + 0x10, water_type);
            e.mem.set_u32(object + REF_CELL, cell);
            let own_node = node(e);
            e.mem.set_u32(object + REF_NODE, own_node);
            object
        }

        #[test]
        fn group_lookup_without_a_reference_uses_the_height_and_the_cell_water_type() {
            let (mut e, system) = lookups();
            let object = object_in_cell(&mut e, 0x1111);
            let low = group(&mut e, 10.0, 0x1111);
            let high = group(&mut e, 90.0, 0x1111);
            let other_type = group(&mut e, 10.0, 0x2222);
            fill_list(&mut e, system + 0x3c, &[other_type, low, high]);
            let find = |e: &mut Engine, height: f32| {
                e.call(0x004e_59f0, &args![system, 0u32, object, height])
                    .u32()
            };
            assert_eq!(find(&mut e, 10.0), low);
            assert_eq!(find(&mut e, 10.005), low, "within 0.01");
            assert_eq!(find(&mut e, 90.0), high);
            assert_eq!(find(&mut e, 50.0), 0);
        }

        #[test]
        fn group_lookup_with_a_reference_needs_it_in_the_group() {
            let (mut e, system) = lookups();
            let object = object_in_cell(&mut e, 0x1111);
            let (reference, _, _) = water_reference(&mut e);
            let own_node = e.mem.u32(reference + REF_NODE);
            for (i, value) in [3.0f32, 4.0, 30.0].into_iter().enumerate() {
                e.mem.set_f32(own_node + 0x8c + 4 * i as u32, value);
            }
            let base_form = e.mem.u32(reference + REF_BASE_FORM);
            e.mem.set_u32(base_form + FORM_WATER, 0x7777);
            let right = group(&mut e, 30.0, 0x7777);
            let wrong_type = group(&mut e, 30.0, 0x1111);
            fill_list(&mut e, right + 0x24, &[reference]);
            fill_list(&mut e, system + 0x3c, &[wrong_type, right]);
            let found = e
                .call(0x004e_59f0, &args![system, reference, object, 0.0f32])
                .u32();
            assert_eq!(found, right);
            // A group with the right plane and type but without the reference.
            fill_list(&mut e, right + 0x24, &[]);
            let found = e
                .call(0x004e_59f0, &args![system, reference, object, 0.0f32])
                .u32();
            assert_eq!(found, 0);
            // A reference without a 3D object has no group.
            e.mem.set_u32(reference + REF_NODE, 0);
            let found = e
                .call(0x004e_59f0, &args![system, reference, object, 0.0f32])
                .u32();
            assert_eq!(found, 0);
        }

        // --- AddTESObjectToWaterGroup / RemoveTESObjectFromWaterGroup -------

        fn register_object_tests(e: &mut Engine) {
            e.register(OBJECT_FLAG_1000000, |e, a| {
                ret((e.mem.u32(a[0] + 8) & 0x0100_0000 != 0) as u32)
            });
            e.register(OBJECT_MEMBER_TEST, |e, a| {
                ret((e.mem.u32(a[0] + 0x64) != 0) as u32)
            });
            quiet(e, &[REMOVE_MASTER_PARTICLE_ADDON_NODES, WADING_MAP_REMOVE]);
            e.register(WADING_MAP_GET, |e, a| {
                e.mem.set_u32(a[2], 0x1234);
                ret(1)
            });
        }

        #[test]
        fn an_object_is_added_to_the_object_list_of_its_group_once() {
            let (mut e, system) = lookups();
            register_object_tests(&mut e);
            let object = object_in_cell(&mut e, 0x1111);
            let target = group(&mut e, 10.0, 0x1111);
            fill_list(&mut e, system + 0x3c, &[target]);
            let add = |e: &mut Engine| {
                e.call(0x004e_5c80, &args![system, 0u32, object, 10.0f32]);
            };
            add(&mut e);
            assert_eq!(list_items(&e, target + 0x30), vec![object]);
            assert!(list_items(&e, target + 0x3c).is_empty());
            add(&mut e);
            assert_eq!(list_items(&e, target + 0x30), vec![object]);
        }

        #[test]
        fn an_actor_goes_to_the_actor_list_once() {
            let (mut e, system) = lookups();
            register_object_tests(&mut e);
            let object = object_in_cell(&mut e, 0x1111);
            e.mem.set_u8(object + REF_IS_ACTOR, 1);
            let target = group(&mut e, 10.0, 0x1111);
            fill_list(&mut e, system + 0x3c, &[target]);
            for _ in 0..2 {
                e.call(0x004e_5c80, &args![system, 0u32, object, 10.0f32]);
            }
            assert_eq!(list_items(&e, target + 0x3c), vec![object]);
            assert!(list_items(&e, target + 0x30).is_empty());
        }

        #[test]
        fn adding_an_object_without_a_cell_or_a_group_does_nothing() {
            let (mut e, system) = lookups();
            register_object_tests(&mut e);
            let lonely = reference(&mut e);
            start_log(&mut e);
            e.call(0x004e_5c80, &args![system, 0u32, lonely, 10.0f32]);
            let object = object_in_cell(&mut e, 0x1111);
            e.call(0x004e_5c80, &args![system, 0u32, object, 10.0f32]);
            assert!(calls(&e, LIST_ADD_HEAD).is_empty());
        }

        #[test]
        fn particle_nodes_are_removed_for_flagged_objects_with_a_node() {
            let (mut e, system) = lookups();
            register_object_tests(&mut e);
            let object = object_in_cell(&mut e, 0x1111);
            let own_node = e.mem.u32(object + REF_NODE);
            let target = group(&mut e, 10.0, 0x1111);
            fill_list(&mut e, system + 0x3c, &[target]);
            e.mem.set_u32(object + 8, 0x0100_0000);
            e.mem.set_u32(object + 0x64, 1);
            start_log(&mut e);
            e.call(0x004e_5c80, &args![system, 0u32, object, 10.0f32]);
            assert_eq!(
                calls(&e, REMOVE_MASTER_PARTICLE_ADDON_NODES),
                vec![vec![own_node]]
            );
            // Without the member the test fails and nothing is removed.
            let object = object_in_cell(&mut e, 0x1111);
            e.mem.set_u32(object + 8, 0x0100_0000);
            start_log(&mut e);
            e.call(0x004e_5c80, &args![system, 0u32, object, 10.0f32]);
            assert!(calls(&e, REMOVE_MASTER_PARTICLE_ADDON_NODES).is_empty());
        }

        #[test]
        fn an_object_is_removed_from_the_object_list() {
            let (mut e, system) = lookups();
            register_object_tests(&mut e);
            let object = object_in_cell(&mut e, 0x1111);
            let other = object_in_cell(&mut e, 0x1111);
            let target = group(&mut e, 10.0, 0x1111);
            fill_list(&mut e, system + 0x3c, &[target]);
            fill_list(&mut e, target + 0x30, &[other, object]);
            e.call(0x004e_5df0, &args![system, 0u32, object, 10.0f32]);
            assert_eq!(list_items(&e, target + 0x30), vec![other]);
            // An object that is not in the list changes nothing.
            e.call(0x004e_5df0, &args![system, 0u32, object, 10.0f32]);
            assert_eq!(list_items(&e, target + 0x30), vec![other]);
            // Neither a reference nor a cell: nothing.
            let lonely = reference(&mut e);
            e.call(0x004e_5df0, &args![system, 0u32, lonely, 10.0f32]);
            e.call(0x004e_5df0, &args![system, 0u32, 0u32, 10.0f32]);
        }

        #[test]
        fn removing_an_actor_also_removes_its_wading_entry() {
            let (mut e, system) = lookups();
            register_object_tests(&mut e);
            let actor = object_in_cell(&mut e, 0x1111);
            e.mem.set_u8(actor + REF_IS_ACTOR, 1);
            let target = group(&mut e, 10.0, 0x1111);
            fill_list(&mut e, target + 0x3c, &[actor]);
            start_log(&mut e);
            e.call(0x004e_5e50, &args![system, actor, target]);
            assert!(list_items(&e, target + 0x3c).is_empty());
            let get = calls(&e, WADING_MAP_GET);
            assert_eq!(get.len(), 1);
            assert_eq!(&get[0][..2], &[system + 0x7c, actor]);
            assert_eq!(
                calls(&e, WADING_MAP_REMOVE),
                vec![vec![system + 0x7c, actor]]
            );
        }

        #[test]
        fn removing_a_plain_object_leaves_the_wading_map_alone() {
            let (mut e, system) = lookups();
            register_object_tests(&mut e);
            let object = object_in_cell(&mut e, 0x1111);
            let target = group(&mut e, 10.0, 0x1111);
            fill_list(&mut e, target + 0x30, &[object]);
            start_log(&mut e);
            e.call(0x004e_5e50, &args![system, object, target]);
            assert!(list_items(&e, target + 0x30).is_empty());
            assert!(calls(&e, WADING_MAP_GET).is_empty());
        }

        // --- fn_004e5fe0 -----------------------------------------------------

        fn register_zone_tests(e: &mut Engine) {
            register_object_tests(e);
            e.register(EXTRA_DATA_LIST_WATER_ZONE_MAP, |e, a| {
                ret(e.mem.u32(a[0] + 0x30))
            });
            e.register(ZONE_MAP_FIRST, |_, _| ret(1));
            e.register(ZONE_MAP_GET_NEXT, |e, a| {
                // One entry: the key is the address kept at map + 0x10.
                let key = e.mem.u32(a[0] + 0x10);
                e.mem.set_u32(a[1], 0);
                e.mem.set_u32(a[2], key);
                Ret::default()
            });
            e.register(READ_WORD_AT_0C, |e, a| ret(e.mem.u32(a[0] + 0x0c)));
            e.register(REFERENCE_HAS_PENDING_NODES, |e, a| {
                ret((e.mem.u32(a[0] + 0x60) > 0) as u32)
            });
            e.register(REFERENCE_REMOVE_PENDING_NODE, |e, a| {
                let count = e.mem.u32(a[0] + 0x60);
                e.mem.set_u32(a[0] + 0x60, count - 1);
                Ret::default()
            });
        }

        /// An object whose extra data list (`+0x44`) holds `zone_map`.
        fn object_with_zone_map(e: &mut Engine, zone_map: u32) -> u32 {
            let object = object_in_cell(e, 0x1111);
            e.mem.set_u32(object + 0x44 + 0x30, zone_map);
            object
        }

        #[test]
        fn zone_cleanup_removes_the_object_from_every_group_without_a_zone_map() {
            let (mut e, system) = lookups();
            register_zone_tests(&mut e);
            let object = object_with_zone_map(&mut e, 0);
            let first = group(&mut e, 10.0, 0x1111);
            let second = group(&mut e, 20.0, 0x1111);
            let third = group(&mut e, 30.0, 0x1111);
            fill_list(&mut e, system + 0x3c, &[first, second, third]);
            fill_list(&mut e, first + 0x30, &[object]);
            fill_list(&mut e, third + 0x30, &[object]);
            e.mem.set_u32(object + 0x60, 2);
            start_log(&mut e);
            e.call(0x004e_5fe0, &args![system, object]);
            assert!(list_items(&e, first + 0x30).is_empty());
            assert!(list_items(&e, third + 0x30).is_empty());
            // The pending nodes were removed before the first removal.
            assert_eq!(e.mem.u32(object + 0x60), 0);
            assert_eq!(calls(&e, REFERENCE_REMOVE_PENDING_NODE).len(), 2);
        }

        #[test]
        fn zone_cleanup_with_a_zone_map_removes_it_where_no_zone_reference_is() {
            let (mut e, system) = lookups();
            register_zone_tests(&mut e);
            let zone_map = e.mem.alloc(0x40);
            let key = e.mem.alloc(0x40);
            let zone_reference = reference(&mut e);
            e.mem.set_u32(key + 4 + 0xc, zone_reference);
            e.mem.set_u32(zone_map + 0x10, key);
            let object = object_with_zone_map(&mut e, zone_map);
            let first = group(&mut e, 10.0, 0x1111);
            fill_list(&mut e, system + 0x3c, &[first]);
            fill_list(&mut e, first + 0x30, &[object]);
            e.call(0x004e_5fe0, &args![system, object]);
            assert!(list_items(&e, first + 0x30).is_empty());
        }

        #[test]
        #[should_panic(expected = "loops for ever")]
        fn zone_cleanup_hangs_in_the_game_when_a_zone_reference_is_in_a_group() {
            let (mut e, system) = lookups();
            register_zone_tests(&mut e);
            let zone_map = e.mem.alloc(0x40);
            let key = e.mem.alloc(0x40);
            let zone_reference = reference(&mut e);
            e.mem.set_u32(key + 4 + 0xc, zone_reference);
            e.mem.set_u32(zone_map + 0x10, key);
            let object = object_with_zone_map(&mut e, zone_map);
            let first = group(&mut e, 10.0, 0x1111);
            fill_list(&mut e, system + 0x3c, &[first]);
            fill_list(&mut e, first + 0x24, &[zone_reference]);
            e.call(0x004e_5fe0, &args![system, object]);
        }

        #[test]
        fn zone_cleanup_removes_actors_and_their_wading_entries() {
            let (mut e, system) = lookups();
            register_zone_tests(&mut e);
            let actor = object_with_zone_map(&mut e, 0);
            e.mem.set_u8(actor + REF_IS_ACTOR, 1);
            let first = group(&mut e, 10.0, 0x1111);
            fill_list(&mut e, system + 0x3c, &[first]);
            fill_list(&mut e, first + 0x3c, &[actor]);
            start_log(&mut e);
            e.call(0x004e_5fe0, &args![system, actor]);
            assert!(list_items(&e, first + 0x3c).is_empty());
            assert_eq!(
                calls(&e, WADING_MAP_REMOVE),
                vec![vec![system + 0x7c, actor]]
            );
        }

        // --- the range test, the visibility, the enable / disable pair -------

        #[test]
        fn range_test_of_a_reference() {
            let mut e = engine();
            e.register(NODE_FIRST_CHILD, |e, a| ret(e.mem.u32(a[0] + 0x9c)));
            e.register(IS_KIND_OF, |e, a| {
                ret((a[1] != 0 && e.mem.u32(a[1]) == 0xface) as u32)
            });
            e.register(FACE_GEN_ANIMATION_DATA, |e, a| ret(e.mem.u32(a[0] + 0xac)));
            e.register(ANIMATION_DATA_TARGET, |e, a| ret(e.mem.u32(a[0] + 0xc)));
            e.register(NODE_IN_RANGE_OF_VIEWER, |e, a| ret(e.mem.u32(a[1] + 0x10)));
            let reference = reference(&mut e);
            // No 3D object: not in range.
            assert!(!e.call(0x004e_62e0, &args![0u32, reference, 0x77u32]).bool());
            let own_node = node(&mut e);
            e.mem.set_u32(reference + REF_NODE, own_node);
            // An ordinary child: the range test of the viewer decides.
            let child = e.mem.alloc(0x100);
            e.mem.set_u32(own_node + 0x9c, child);
            let viewer = e.mem.alloc(0x40);
            e.mem.set_u32(viewer + 0x10, 1);
            start_log(&mut e);
            assert!(e.call(0x004e_62e0, &args![0u32, reference, viewer]).bool());
            assert_eq!(
                calls(&e, NODE_IN_RANGE_OF_VIEWER),
                vec![vec![own_node, viewer]]
            );
            e.mem.set_u32(viewer + 0x10, 0);
            assert!(!e.call(0x004e_62e0, &args![0u32, reference, viewer]).bool());
            // A face-gen child: the animation data target holds exactly one entry.
            e.mem.set_u32(child, 0xface);
            let animation = e.mem.alloc(0x40);
            let target = e.mem.alloc(0x40);
            e.mem.set_u32(child + 0xac, animation);
            e.mem.set_u32(animation + 0xc, target);
            e.mem.set_u32(target + 8, 1);
            assert!(e.call(0x004e_62e0, &args![0u32, reference, viewer]).bool());
            e.mem.set_u32(target + 8, 2);
            assert!(!e.call(0x004e_62e0, &args![0u32, reference, viewer]).bool());
        }

        #[test]
        fn visibility_is_set_or_toggled_on_the_nodes_of_the_water_references() {
            let mut e = engine();
            quiet(&mut e, &[NODE_SET_CULLED]);
            let system = e.mem.alloc(0xa0);
            let (first, _, _) = water_reference(&mut e);
            let (second, _, _) = water_reference(&mut e);
            let target = group(&mut e, 10.0, 0x1111);
            fill_list(&mut e, target + 0x24, &[first, second]);
            fill_list(&mut e, system + 0x3c, &[target]);
            let first_node = e.mem.u32(first + REF_NODE);
            let second_node = e.mem.u32(second + REF_NODE);
            // The second is flagged and culled.
            let second_form = e.mem.u32(second + REF_BASE_FORM);
            e.mem.set_u32(second_form + FORM_FLAGS, 0x0800_0000);
            e.mem.set_u32(second_node + 0x30, 1);
            start_log(&mut e);
            e.call(0x004e_6370, &args![system, true, false, false]);
            assert_eq!(
                calls(&e, NODE_SET_CULLED),
                vec![vec![first_node, 0], vec![second_node, 0]]
            );
            start_log(&mut e);
            e.call(0x004e_6370, &args![system, false, false, false]);
            assert_eq!(
                calls(&e, NODE_SET_CULLED),
                vec![vec![first_node, 1], vec![second_node, 1]]
            );
            // Only the flagged reference.
            start_log(&mut e);
            e.call(0x004e_6370, &args![system, true, true, false]);
            assert_eq!(calls(&e, NODE_SET_CULLED), vec![vec![second_node, 0]]);
            // Toggling: the culled node is shown, the shown one is hidden.
            start_log(&mut e);
            e.call(0x004e_6370, &args![system, true, false, true]);
            assert_eq!(
                calls(&e, NODE_SET_CULLED),
                vec![vec![first_node, 1], vec![second_node, 0]]
            );
            start_log(&mut e);
            e.call(0x004e_6370, &args![system, true, true, true]);
            assert_eq!(calls(&e, NODE_SET_CULLED), vec![vec![second_node, 0]]);
        }

        #[test]
        fn enabling_the_water_system_happens_once_and_needs_the_setting() {
            let mut e = engine();
            quiet(&mut e, &[NODE_SET_CULLED]);
            let system = e.mem.alloc(0xa0);
            let (reference, _, _) = water_reference(&mut e);
            let target = group(&mut e, 10.0, 0x1111);
            fill_list(&mut e, target + 0x24, &[reference]);
            fill_list(&mut e, system + 0x3c, &[target]);
            // The setting is off.
            assert!(!e.call(0x004e_65d0, &args![system]).bool());
            assert_eq!(e.global::<u8>(WATER_REQUEST_FLAG), 1);
            assert_eq!(e.global::<u8>(WATER_ENABLED), 0);
            // On: enabled, and the references are shown.
            set_setting(&mut e, SETTING_USE_WATER, true);
            start_log(&mut e);
            assert!(e.call(0x004e_65d0, &args![system]).bool());
            assert_eq!(e.global::<u8>(WATER_ENABLED), 1);
            assert_eq!(calls(&e, NODE_SET_CULLED).len(), 1);
            assert_eq!(calls(&e, NODE_SET_CULLED)[0][1], 0);
            // Already enabled: nothing more.
            start_log(&mut e);
            assert!(!e.call(0x004e_65d0, &args![system]).bool());
            assert!(calls(&e, NODE_SET_CULLED).is_empty());
        }

        // --- fn_004e6620: disabling the water system -------------------------

        #[test]
        fn disabling_with_release_clears_every_texture_and_object() {
            let mut e = engine();
            quiet(
                &mut e,
                &[
                    RETURN_RENDERED_TEXTURE,
                    SOUND_HANDLE_RELEASE,
                    WATER_FFT_FREE_TEXTURE_MEMORY,
                    WATER_OBJECT_EMPTY_CALL,
                    NODE_SET_CULLED,
                ],
            );
            e.register(TEXTURE_MANAGER, |_, _| ret(0x0aaa_0000));
            e.register(SOUND_HANDLE_IS_VALID, |_, _| ret(1));
            let system = e.mem.alloc(0xa0);
            for (offset, value) in [
                (0x08, 0x111u32),
                (0x10, 0x222),
                (0x14, 0x333),
                (0x18, 0x444),
            ] {
                e.mem.set_u32(system + offset, value);
            }
            e.set_global(DEPTH_MAP, 0x555u32);
            e.set_global(WADING_WATER_HEIGHT_MAP, 0x666u32);
            let (reference, _, property) = water_reference(&mut e);
            let target = group(&mut e, 10.0, 0x1111);
            fill_list(&mut e, target + 0x24, &[reference]);
            let geometry = node(&mut e);
            let wading_property = e.mem.alloc(0x150);
            for offset in [0x13c, 0x140, 0x138, 0x144] {
                e.mem
                    .set_u32(wading_property + offset, 0x0b00_0000 + offset);
            }
            e.mem.set_u32(geometry + NODE_PROPERTY, wading_property);
            e.mem.set_u32(target + 0x58, geometry);
            fill_list(&mut e, system + 0x3c, &[target]);
            let object = e.mem.alloc(0x260);
            for offset in [0x238, 0x23c, 0x248, 0x24c] {
                e.mem.set_u32(object + offset, 0xdead);
            }
            e.set_global(WATER_RENDER_OBJECT, object);
            e.set_global(WATER_FFT_EFFECT, 0x0fff_0000u32);
            e.set_global(WATER_OBJECT_011FFFF8, 0x0aaa_0001u32);
            e.set_global(WATER_ENABLED, 1u8);
            e.set_global(WATER_REQUEST_FLAG, 1u8);
            e.set_global(ACTIVE_WATER_GROUPS, 5u32);
            start_log(&mut e);
            assert!(e.call(0x004e_6620, &args![system, true, false]).bool());

            assert_eq!(e.global::<u8>(WATER_ENABLED), 0);
            assert_eq!(e.global::<u8>(WATER_REQUEST_FLAG), 0);
            assert_eq!(e.global::<u32>(ACTIVE_WATER_GROUPS), 0);
            for offset in [0x13c, 0x140, 0x138, 0x144] {
                assert_eq!(e.mem.u32(wading_property + offset), 0);
                assert_eq!(e.mem.u32(property + offset), 0);
            }
            // The textures went back to the manager in this order and the
            // slots were cleared.
            assert_eq!(
                calls(&e, RETURN_RENDERED_TEXTURE),
                [0x111u32, 0x222, 0x333, 0x555, 0x666]
                    .iter()
                    .map(|texture| vec![0x0aaa_0000, *texture])
                    .collect::<Vec<_>>()
            );
            for offset in [0x08, 0x10, 0x14, 0x18] {
                assert_eq!(e.mem.u32(system + offset), 0);
            }
            assert_eq!(e.global::<u32>(DEPTH_MAP), 0);
            assert_eq!(e.global::<u32>(WADING_WATER_HEIGHT_MAP), 0);
            for offset in [0x238, 0x23c, 0x248, 0x24c] {
                assert_eq!(e.mem.u32(object + offset), 0);
            }
            assert_eq!(
                calls(&e, WATER_FFT_FREE_TEXTURE_MEMORY),
                vec![vec![0x0fff_0000]]
            );
            assert_eq!(calls(&e, WATER_OBJECT_EMPTY_CALL), vec![vec![0x0aaa_0001]]);
            assert_eq!(calls(&e, SOUND_HANDLE_RELEASE), vec![vec![system + 0x8c]]);
            // The references are hidden.
            let own_node = e.mem.u32(reference + REF_NODE);
            assert_eq!(calls(&e, NODE_SET_CULLED), vec![vec![own_node, 1]]);
        }

        #[test]
        fn disabling_without_release_only_clears_the_flags_and_hides_the_water() {
            let mut e = engine();
            quiet(&mut e, &[NODE_SET_CULLED, RETURN_RENDERED_TEXTURE]);
            let system = e.mem.alloc(0xa0);
            e.mem.set_u32(system + 8, 0x111);
            let (reference, _, _) = water_reference(&mut e);
            let target = group(&mut e, 10.0, 0x1111);
            fill_list(&mut e, target + 0x24, &[reference]);
            fill_list(&mut e, system + 0x3c, &[target]);
            e.set_global(WATER_ENABLED, 1u8);
            e.set_global(ACTIVE_WATER_GROUPS, 5u32);
            start_log(&mut e);
            assert!(e.call(0x004e_6620, &args![system, false, false]).bool());
            assert_eq!(e.global::<u8>(WATER_ENABLED), 0);
            assert_eq!(e.global::<u32>(ACTIVE_WATER_GROUPS), 0);
            assert_eq!(e.mem.u32(system + 8), 0x111);
            assert!(calls(&e, RETURN_RENDERED_TEXTURE).is_empty());
            assert_eq!(calls(&e, NODE_SET_CULLED).len(), 1);
            // With the visibility kept nothing is hidden.
            start_log(&mut e);
            assert!(e.call(0x004e_6620, &args![system, false, true]).bool());
            assert!(calls(&e, NODE_SET_CULLED).is_empty());
        }

        // --- UpdateWaterSounds ----------------------------------------------

        struct Sounds {
            e: Engine,
            system: u32,
            player: u32,
            cell: u32,
            tes: u32,
        }

        /// Doubles for the sound update. The player stands at (1000, 2000, 5)
        /// in a cell with water at height 5 (`+0x50`; flags at `+0x24`: bit 1
        /// interior, bit 2 water). The settings: radius 100, 3 points,
        /// tolerances 10. A sound handle is valid when its first word is not
        /// zero and playing when the byte at +4 is set.
        fn sounds() -> Sounds {
            let mut e = engine();
            let world_space = world_space(&mut e, 5.0);
            e.register(PLAYER_GET_WATER_CELL, |e, a| ret(e.mem.u32(a[0] + 0x1000)));
            e.register(TES_GET_CURRENT_CELL, |e, a| ret(e.mem.u32(a[0] + 0x58)));
            e.register(CELL_HAS_WATER, |e, a| {
                ret(((e.mem.u8(a[0] + 0x24) & 2) != 0) as u32)
            });
            e.register(CELL_IS_INTERIOR, |e, a| {
                ret((e.mem.u8(a[0] + 0x24) & 1) as u32)
            });
            e.register(CELL_GET_WATER_HEIGHT, |e, a| {
                e.mem.f32(a[0] + 0x50).into_ret()
            });
            e.register(PLAYER_POSITION, |_, a| ret(a[0] + 0x30));
            e.register(FLOAT_ABS, |_, a| float_arg(a, 0).abs().into_ret());
            e.register(POINT2_LENGTH, |e, a| {
                (e.mem.f32(a[0]).powi(2) + e.mem.f32(a[0] + 4).powi(2))
                    .sqrt()
                    .into_ret()
            });
            e.register(POINT2_SCALE, |e, a| {
                let scale = float_arg(a, 2);
                let (x, y) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4));
                e.mem.set_f32(a[1], x * scale);
                e.mem.set_f32(a[1] + 4, y * scale);
                ret(a[1])
            });
            e.register(SOUND_HANDLE_IS_VALID, |e, a| {
                ret((e.mem.u32(a[0]) != 0) as u32)
            });
            e.register(SOUND_HANDLE_IS_PLAYING, |e, a| {
                ret(e.mem.u8(a[0] + 4) as u32)
            });
            quiet(
                &mut e,
                &[
                    SOUND_HANDLE_STOP,
                    SOUND_HANDLE_RELEASE,
                    SOUND_HANDLE_PLAY,
                    SOUND_HANDLE_SET_POSITION,
                    TES_ADD_TEMP_DEBUG_OBJECT,
                    NODE_SET_LOCAL_TRANSLATE,
                ],
            );
            e.register(WORLD_SPACE_GET_CELL, |e, a| ret(e.mem.u32(a[0] + 0x60)));
            e.register(
                CELL_CONTAINS_POINT,
                |e, a| ret(e.mem.u8(a[0] + 0x70) as u32),
            );
            e.register(TES_GET_LAND_HEIGHT, |e, a| {
                // Land is 10 high left of the cutoff in TES + 0x60, else 0.
                let land = if e.mem.f32(a[1]) < e.mem.f32(a[0] + 0x60) {
                    10.0
                } else {
                    0.0
                };
                e.mem.set_f32(a[2], land);
                ret(1)
            });
            e.register(NI_COLOR_CONSTRUCT, |e, a| {
                for i in 0..4 {
                    e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
                }
                ret(a[0])
            });
            e.register(MAKE_TRI_POINT, |e, _| ret(e.mem.alloc(0x20)));
            e.register(MAKE_DEBUG_LINE, |e, _| ret(e.mem.alloc(0x20)));
            e.register(FLOAT_MAX, |_, a| {
                let (first, second) = (float_arg(a, 0), float_arg(a, 1));
                (if second < first { first } else { second }).into_ret()
            });
            e.set_global(SETTING_NEAR_WATER_RADIUS + 4, 100u32);
            e.set_global(SETTING_NEAR_WATER_POINTS + 4, 3u32);
            e.set_global(SETTING_NEAR_WATER_INDOOR_TOLERANCE + 4, 10.0f32);
            e.set_global(SETTING_NEAR_WATER_OUTDOOR_TOLERANCE + 4, 10.0f32);

            let system = e.mem.alloc(0xa0);
            let player = object(&mut e, REFERENCE2_VTABLE, 0x1100);
            e.set_global(PLAYER_CHARACTER, player);
            for (i, value) in [1000.0f32, 2000.0, 5.0].into_iter().enumerate() {
                e.mem.set_f32(player + 0x30 + 4 * i as u32, value);
            }
            let cell = e.mem.alloc(0x100);
            e.mem.set_u8(cell + 0x24, 2);
            e.mem.set_f32(cell + 0x50, 5.0);
            e.mem.set_u32(player + 0x1000, cell);
            e.mem.set_u32(player + REF_CELL, cell);
            e.mem.set_f32(player + REF_LOCATION + 8, 7.0);
            let tes = e.global::<u32>(TES_POINTER);
            e.mem.set_u32(tes + 0x58, cell);
            e.mem.set_u32(world_space + 0x60, cell);
            Sounds {
                e,
                system,
                player,
                cell,
                tes,
            }
        }

        impl Sounds {
            fn update(&mut self) {
                start_log(&mut self.e);
                let system = self.system;
                self.e.call(0x004e_6a80, &args![system]);
            }

            fn sound(&self) -> u32 {
                self.system + 0x8c
            }

            fn make_interior(&mut self) {
                let cell = self.cell;
                self.e.mem.set_u8(cell + 0x24, 3);
            }

            /// A running sound: a valid handle that is playing.
            fn start_sound(&mut self, playing: bool) {
                let sound = self.sound();
                self.e.mem.set_u32(sound, 0x77);
                self.e.mem.set_u8(sound + 4, playing as u8);
            }
        }

        #[test]
        fn a_running_sound_is_stopped_when_there_is_no_water_cell() {
            let mut s = sounds();
            s.start_sound(true);
            s.e.set_global(WATER_SOUND_STARTED, 1u8);
            let cell = s.cell;
            s.e.mem.set_u8(cell + 0x24, 0);
            s.e.set_global(DISPLAY_WATER_SOUND_PLACEMENT, 1u8);
            s.update();
            let sound = s.sound();
            assert_eq!(calls(&s.e, SOUND_HANDLE_STOP), vec![vec![sound]]);
            assert_eq!(s.e.global::<u8>(WATER_SOUND_STARTED), 0);
            assert_eq!(s.e.global::<u8>(DISPLAY_WATER_SOUND_PLACEMENT), 0);
            assert!(calls(&s.e, SOUND_HANDLE_SET_POSITION).is_empty());
            // The sound is left alone when this code did not start it.
            s.e.set_global(WATER_SOUND_STARTED, 0u8);
            s.update();
            assert!(calls(&s.e, SOUND_HANDLE_STOP).is_empty());
            // Without a player the current cell is used.
            s.e.set_global(PLAYER_CHARACTER, 0u32);
            s.e.set_global(WATER_SOUND_STARTED, 1u8);
            s.update();
            assert_eq!(calls(&s.e, TES_GET_CURRENT_CELL).len(), 1);
            assert!(calls(&s.e, PLAYER_GET_WATER_CELL).is_empty());
            assert_eq!(calls(&s.e, SOUND_HANDLE_STOP).len(), 1);
        }

        #[test]
        fn the_water_cell_is_looked_up_within_the_near_water_radius() {
            let mut s = sounds();
            s.make_interior();
            s.start_sound(false);
            s.update();
            assert_eq!(
                calls(&s.e, PLAYER_GET_WATER_CELL),
                vec![vec![s.player, word(100.0)]]
            );
        }

        #[test]
        fn indoors_the_sound_starts_when_the_player_is_at_the_water_height() {
            let mut s = sounds();
            s.make_interior();
            s.start_sound(false);
            s.update();
            let sound = s.sound();
            // The sound sits at the player, at the water height.
            assert_eq!(
                calls(&s.e, SOUND_HANDLE_SET_POSITION),
                vec![vec![sound, word(1000.0), word(2000.0), word(5.0)]]
            );
            assert_eq!(calls(&s.e, SOUND_HANDLE_PLAY), vec![vec![sound, 1]]);
            assert_eq!(s.e.global::<u8>(WATER_SOUND_STARTED), 1);
            assert!(calls(&s.e, SOUND_HANDLE_STOP).is_empty());
        }

        #[test]
        fn indoors_a_player_far_from_the_water_stops_the_sound() {
            let mut s = sounds();
            s.make_interior();
            s.start_sound(true);
            // 15 away from the water, tolerance 10: the fraction is 0.
            let player = s.player;
            s.e.mem.set_f32(player + 0x30 + 8, 20.0);
            s.update();
            let sound = s.sound();
            assert_eq!(calls(&s.e, SOUND_HANDLE_STOP), vec![vec![sound]]);
            assert_eq!(calls(&s.e, SOUND_HANDLE_RELEASE), vec![vec![sound]]);
            assert!(calls(&s.e, SOUND_HANDLE_PLAY).is_empty());
            assert_eq!(s.e.global::<u8>(WATER_SOUND_STARTED), 0);
            // A sound that is not playing stays off.
            s.start_sound(false);
            s.update();
            assert!(calls(&s.e, SOUND_HANDLE_PLAY).is_empty());
            assert!(calls(&s.e, SOUND_HANDLE_STOP).is_empty());
        }

        #[test]
        fn indoors_a_running_sound_keeps_playing_while_the_fraction_is_positive() {
            let mut s = sounds();
            s.make_interior();
            s.start_sound(true);
            let player = s.player;
            s.e.mem.set_f32(player + 0x30 + 8, 9.0);
            s.update();
            assert!(calls(&s.e, SOUND_HANDLE_STOP).is_empty());
            assert!(calls(&s.e, SOUND_HANDLE_PLAY).is_empty());
            assert_eq!(calls(&s.e, SOUND_HANDLE_SET_POSITION).len(), 1);
        }

        #[test]
        fn skipped_water_silences_the_sound() {
            let mut s = sounds();
            s.make_interior();
            s.start_sound(true);
            s.e.set_global(0x011c_7a59u32, 1u8);
            s.update();
            let sound = s.sound();
            assert_eq!(calls(&s.e, SOUND_HANDLE_STOP), vec![vec![sound]]);
        }

        #[test]
        fn outdoors_a_grid_of_water_gives_the_centre_and_a_full_fraction() {
            let mut s = sounds();
            s.start_sound(false);
            s.update();
            let sound = s.sound();
            // 3 x 3 samples, 100 apart, around the player; every one is water.
            assert_eq!(
                calls(&s.e, SOUND_HANDLE_SET_POSITION),
                vec![vec![sound, word(1000.0), word(2000.0), word(5.0)]]
            );
            assert_eq!(calls(&s.e, SOUND_HANDLE_PLAY), vec![vec![sound, 1]]);
            assert_eq!(calls(&s.e, TES_GET_LAND_HEIGHT).len(), 9);
            // The sample cell is looked up for every sample (no cache hit).
            assert_eq!(calls(&s.e, WORLD_SPACE_GET_CELL).len(), 9);
            let first = calls(&s.e, TES_GET_LAND_HEIGHT)[0].clone();
            assert_eq!(first[0], s.tes);
        }

        #[test]
        fn the_cell_of_the_last_sample_is_reused_while_it_contains_the_point() {
            let mut s = sounds();
            s.start_sound(false);
            let cell = s.cell;
            s.e.mem.set_u8(cell + 0x70, 1);
            s.update();
            assert_eq!(calls(&s.e, WORLD_SPACE_GET_CELL).len(), 1);
            assert_eq!(calls(&s.e, CELL_CONTAINS_POINT).len(), 8);
        }

        #[test]
        fn outdoors_land_above_the_water_reduces_the_fraction_and_moves_the_centre() {
            let mut s = sounds();
            s.start_sound(false);
            // Land is high for x below 1050: only the column x = 1100 is water.
            let tes = s.tes;
            s.e.mem.set_f32(tes + 0x60, 1050.0);
            s.update();
            let sound = s.sound();
            let position = calls(&s.e, SOUND_HANDLE_SET_POSITION);
            assert_eq!(position.len(), 1);
            // Centre (1100, 2000); fraction 3 * 2 / 9; the sound is moved from
            // the centre towards the player by 100 * (1 - fraction).
            let fraction = 6.0f64 / 9.0;
            let expected_x = 1000.0 + 100.0 * (1.0 - fraction);
            assert_eq!(position[0][0], sound);
            assert!((float_arg(&position[0], 1) as f64 - expected_x).abs() < 0.01);
            assert_eq!(float_arg(&position[0], 2), 2000.0);
            assert_eq!(float_arg(&position[0], 3), 5.0);
            assert_eq!(calls(&s.e, SOUND_HANDLE_PLAY).len(), 1);
        }

        #[test]
        fn outdoors_no_water_in_the_grid_means_no_sound() {
            let mut s = sounds();
            s.start_sound(true);
            let tes = s.tes;
            s.e.mem.set_f32(tes + 0x60, 5000.0);
            s.update();
            let sound = s.sound();
            assert_eq!(calls(&s.e, SOUND_HANDLE_STOP), vec![vec![sound]]);
            assert_eq!(calls(&s.e, SOUND_HANDLE_RELEASE), vec![vec![sound]]);
        }

        #[test]
        fn outdoors_the_height_difference_weighs_the_fraction() {
            let mut s = sounds();
            s.start_sound(false);
            // The player is 5 above the water: weight (10 - 5) / 10.
            let player = s.player;
            s.e.mem.set_f32(player + 0x30 + 8, 10.0);
            s.update();
            let position = calls(&s.e, SOUND_HANDLE_SET_POSITION);
            // Fraction 2 * 0.5 = 1: the sound is at the centre (the player).
            assert_eq!(float_arg(&position[0], 1), 1000.0);
            assert_eq!(calls(&s.e, SOUND_HANDLE_PLAY).len(), 1);
            // 20 above the water: nothing.
            s.e.mem.set_f32(player + 0x30 + 8, 25.0);
            s.update();
            assert!(calls(&s.e, SOUND_HANDLE_PLAY).is_empty());
        }

        /// The sound a water type plays: its form has the flag word at
        /// `+0x48` and the sound number at `+0xc`.
        fn give_water_sound(s: &mut Sounds, flag_word: u32) -> u32 {
            let cell = s.cell;
            let water_type = s.e.mem.alloc(0x100);
            let sound_form = s.e.mem.alloc(0x100);
            s.e.mem.set_u32(cell + 0x10, water_type);
            s.e.mem.set_u32(water_type + 0x7c, sound_form);
            s.e.mem.set_u32(sound_form + 0xc, 0x4321);
            s.e.mem.set_u32(sound_form + 0x48, flag_word);
            s.e.register(CELL_GET_WATER_TYPE, |e, a| ret(e.mem.u32(a[0] + 0x10)));
            s.e.register(WATER_TYPE_SOUND, |e, a| ret(e.mem.u32(a[0] + 0x7c)));
            s.e.register(READ_WORD_AT_0C, |e, a| ret(e.mem.u32(a[0] + 0x0c)));
            s.e.register(SOUND_FLAGS_TO_AUDIO_FLAGS, |_, a| {
                ret(if a[0] & 1 != 0 { 0x10 } else { 0 })
            });
            s.e.register(AUDIO_INSTANCE, |_, _| ret(0x0aaa_0000));
            s.e.register(AUDIO_GET_SOUND_HANDLE, |e, a| {
                e.mem.set_u32(a[1], 0x1234);
                ret(a[1])
            });
            s.e.register(SOUND_HANDLE_ASSIGN, |e, a| {
                let id = e.mem.u32(a[1]);
                e.mem.set_u32(a[0], id);
                ret(a[0])
            });
            quiet(&mut s.e, &[SOUND_HANDLE_DESTRUCT]);
            sound_form
        }

        #[test]
        fn a_sound_is_created_from_the_water_type_when_there_is_none() {
            let mut s = sounds();
            s.make_interior();
            give_water_sound(&mut s, 0x41);
            s.update();
            let created = calls(&s.e, AUDIO_GET_SOUND_HANDLE);
            assert_eq!(created.len(), 1);
            // The id: bit 0x40 gives 0x7ffffff, plus 2, with the audio flags
            // for the flag word.
            assert_eq!(created[0][0], 0x0aaa_0000);
            assert_eq!(created[0][2], 0x4321);
            assert_eq!(created[0][3], (0x07ff_ffff + 2) | 0x10);
            let sound = s.sound();
            assert_eq!(s.e.mem.u32(sound), 0x1234, "the handle was assigned");
            assert_eq!(
                calls(&s.e, SOUND_HANDLE_ASSIGN),
                vec![vec![sound, created[0][1]]]
            );
            assert_eq!(
                calls(&s.e, SOUND_HANDLE_DESTRUCT),
                vec![vec![created[0][1]]]
            );
            // The creation does not start it: the next update does.
            assert!(calls(&s.e, SOUND_HANDLE_PLAY).is_empty());
        }

        #[test]
        fn the_id_without_bit_0x40_is_just_two_with_the_audio_flags() {
            let mut s = sounds();
            s.make_interior();
            give_water_sound(&mut s, 0x00);
            s.update();
            let created = calls(&s.e, AUDIO_GET_SOUND_HANDLE);
            assert_eq!(created[0][3], 2);
        }

        #[test]
        fn no_sound_is_created_without_water_type_sound_or_fraction() {
            let mut s = sounds();
            s.make_interior();
            // No water type on the cell.
            s.e.register(CELL_GET_WATER_TYPE, |_, _| ret(0));
            s.update();
            assert!(calls(&s.e, AUDIO_GET_SOUND_HANDLE).is_empty());
            // A water type without a sound.
            let mut s = sounds();
            s.make_interior();
            give_water_sound(&mut s, 0);
            let cell = s.cell;
            let water_type = s.e.mem.u32(cell + 0x10);
            s.e.mem.set_u32(water_type + 0x7c, 0);
            s.update();
            assert!(calls(&s.e, AUDIO_GET_SOUND_HANDLE).is_empty());
            // A fraction of zero.
            let mut s = sounds();
            s.make_interior();
            give_water_sound(&mut s, 0);
            let player = s.player;
            s.e.mem.set_f32(player + 0x30 + 8, 99.0);
            s.update();
            assert!(calls(&s.e, AUDIO_GET_SOUND_HANDLE).is_empty());
        }

        #[test]
        fn debug_markers_are_drawn_for_the_samples_and_the_sound() {
            let mut s = sounds();
            s.start_sound(false);
            s.e.set_global(DISPLAY_WATER_SOUND_PLACEMENT, 1u8);
            // Land is high for x below 950: 3 samples are land, 6 are water.
            let tes = s.tes;
            s.e.mem.set_f32(tes + 0x60, 950.0);
            let sound = s.sound();
            s.e.mem.set_u32(sound, 0x77);
            s.update();
            let colors = calls(&s.e, NI_COLOR_CONSTRUCT);
            let sample_colors: Vec<_> = colors[..9].iter().map(|c| c[1..].to_vec()).collect();
            let green = vec![word(0.0), word(1.0), word(0.0), word(0.0)];
            let blue = vec![word(0.0), word(0.0), word(1.0), word(0.0)];
            let greens = sample_colors.iter().filter(|c| **c == green).count();
            let blues = sample_colors.iter().filter(|c| **c == blue).count();
            assert_eq!((greens, blues), (6, 3));
            // One marker per sample, then the sound's marker and line.
            assert_eq!(calls(&s.e, MAKE_TRI_POINT).len(), 10);
            assert_eq!(calls(&s.e, MAKE_DEBUG_LINE).len(), 1);
            assert_eq!(calls(&s.e, TES_ADD_TEMP_DEBUG_OBJECT).len(), 11);
            let line = &calls(&s.e, MAKE_DEBUG_LINE)[0];
            assert_eq!(line.len(), 5);
            assert_eq!(line[4], 1);
            // The sound's marker is red and sits 10 above the player's
            // location (7).
            let red = vec![word(1.0), word(0.0), word(0.0), word(0.0)];
            assert_eq!(colors[9][1..].to_vec(), red);
            assert_eq!(s.e.global::<u8>(DISPLAY_WATER_SOUND_PLACEMENT), 0);
        }

        // --- CreateQuadData --------------------------------------------------

        pub(super) struct Quad {
            pub(super) e: Engine,
        }

        /// The doubles for the allocations and constructors of the quad.
        pub(super) fn quad() -> Quad {
            let mut e = engine();
            e.register(ALLOCATION_SCOPE_CONSTRUCT, |_, a| ret(a[0]));
            quiet(
                &mut e,
                &[
                    ALLOCATION_SCOPE_DESTRUCT,
                    VECTOR_CONSTRUCTOR_ITERATOR,
                    ATEXIT,
                ],
            );
            e.register(NI_ALLOC, |e, a| ret(e.mem.alloc(a[0])));
            e.register(NI_ALLOC_ARRAY, |e, a| ret(e.mem.alloc(a[0])));
            e.register(NI_POINT2_CONSTRUCT, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                e.mem.set_u32(a[0] + 4, a[2]);
                ret(a[0])
            });
            e.register(NI_COLOR_CONSTRUCT, |e, a| {
                for i in 0..4 {
                    e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
                }
                ret(a[0])
            });
            e.register(FLOAT_SETTING_CONSTRUCT, |e, a| {
                e.mem.set_u32(a[0] + 4, a[2]);
                ret(a[0])
            });
            e.register(FLOAT_SETTING_SET, |e, a| {
                e.mem.set_u32(a[0] + 4, a[1]);
                ret(a[0])
            });
            e.register(FLOAT_SETTING_GET, |e, a| e.mem.f32(a[0] + 4).into_ret());
            e.register(TRI_SHAPE_DATA_CONSTRUCT, |e, a| {
                // Keeps the arrays it was given at the start of the object.
                for i in 0..9 {
                    e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
                }
                ret(a[0])
            });
            Quad { e }
        }

        impl Quad {
            fn create(
                &mut self,
                width: f32,
                height: f32,
                scale: u32,
                normals: bool,
                colors: bool,
            ) -> u32 {
                start_log(&mut self.e);
                self.e
                    .call(
                        0x004e_7770,
                        &args![0u32, width, height, 0x200u32, scale, normals, colors],
                    )
                    .u32()
            }

            fn floats(&self, address: u32, count: u32) -> Vec<f32> {
                (0..count)
                    .map(|i| self.e.mem.f32(address + 4 * i))
                    .collect()
            }
        }

        #[test]
        fn a_quad_has_four_corners_two_triangles_and_texture_coordinates() {
            let mut q = quad();
            let data = q.create(8.0, 4.0, 1, false, false);
            assert_ne!(data, 0);
            // (vertex count, vertices, normals, colors, uvs, 1, 0, 2, triangles)
            let words: Vec<u32> = (0..9).map(|i| q.e.mem.u32(data + 4 * i)).collect();
            assert_eq!(words[0], 4);
            assert_eq!(
                q.floats(words[1], 12),
                vec![4.0, 2.0, 0.0, -4.0, 2.0, 0.0, -4.0, -2.0, 0.0, 4.0, -2.0, 0.0]
            );
            assert_eq!(words[2], 0, "no normals");
            assert_eq!(words[3], 0, "no colors");
            assert_eq!(
                q.floats(words[4], 8),
                vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0]
            );
            assert_eq!(&words[5..8], &[1, 0, 2]);
            let triangles: Vec<u16> = (0..6).map(|i| q.e.mem.u16(words[8] + 2 * i)).collect();
            assert_eq!(triangles, vec![0, 1, 2, 0, 2, 3]);
            // The arrays came from the allocator with their constructors.
            assert_eq!(calls(&q.e, OPERATOR_NEW), vec![vec![0x30], vec![0x20]]);
            assert_eq!(
                calls(&q.e, VECTOR_CONSTRUCTOR_ITERATOR),
                vec![
                    vec![words[1], 0x0c, 4, ADDRESS_OF_THIS],
                    vec![words[4], 0x08, 4, ADDRESS_OF_THIS]
                ]
            );
            assert_eq!(calls(&q.e, NI_ALLOC_ARRAY), vec![vec![0x0c]]);
            // The memory scope of line 0xbac is closed again.
            let scope = calls(&q.e, ALLOCATION_SCOPE_CONSTRUCT);
            assert_eq!(&scope[0][1..], &[0x1d, 1, TESWATER_SOURCE_PATH, 0xbac]);
            assert_eq!(
                calls(&q.e, ALLOCATION_SCOPE_DESTRUCT),
                vec![vec![scope[0][0]]]
            );
        }

        #[test]
        fn the_texture_scale_stretches_the_coordinates() {
            let mut q = quad();
            let data = q.create(2.0, 2.0, 4, false, false);
            let uvs = q.e.mem.u32(data + 16);
            assert_eq!(
                q.floats(uvs, 8),
                vec![4.0, 0.0, 0.0, 0.0, 0.0, 4.0, 4.0, 4.0]
            );
        }

        #[test]
        fn normals_and_colors_are_added_on_request() {
            let mut q = quad();
            // The first call builds the setting (default 1); then it is set.
            q.create(2.0, 2.0, 1, false, true);
            q.e.set_global(SETTING_WATER_ALPHA + 4, 0.5f32);
            let data = q.create(2.0, 2.0, 1, true, true);
            let normals = q.e.mem.u32(data + 8);
            let colors = q.e.mem.u32(data + 12);
            assert_eq!(
                q.floats(normals, 12),
                vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0]
            );
            assert_eq!(q.floats(colors, 16), [1.0, 1.0, 1.0, 0.5].repeat(4));
            assert_eq!(
                calls(&q.e, VECTOR_CONSTRUCTOR_ITERATOR)[2..].to_vec(),
                vec![
                    vec![normals, 0x0c, 4, ADDRESS_OF_THIS],
                    vec![colors, 0x10, 4, NI_COLOR_DEFAULT_CONSTRUCT]
                ]
            );
        }

        #[test]
        fn the_alpha_setting_is_built_once_and_clamped() {
            let mut q = quad();
            q.create(2.0, 2.0, 1, false, true);
            // The first call built the setting, the shared normal and the
            // exit-time destructor.
            assert_eq!(
                calls(&q.e, FLOAT_SETTING_CONSTRUCT),
                vec![vec![SETTING_WATER_ALPHA, ALPHA_SETTING_NAME, word(1.0)]]
            );
            assert_eq!(
                calls(&q.e, ATEXIT),
                vec![vec![WATER_ALPHA_SETTING_DESTRUCT]]
            );
            assert_eq!(q.e.global::<u32>(QUAD_STATICS_BUILT), 3);
            assert_eq!(
                calls(&q.e, NI_POINT3_CONSTRUCT)
                    .iter()
                    .filter(|c| c[0] == QUAD_NORMAL)
                    .count(),
                1
            );
            // The second call does not build them again; too large and too
            // small values are clamped in the setting.
            q.e.set_global(SETTING_WATER_ALPHA + 4, 3.0f32);
            let data = q.create(2.0, 2.0, 1, false, true);
            assert!(calls(&q.e, FLOAT_SETTING_CONSTRUCT).is_empty());
            assert_eq!(
                calls(&q.e, FLOAT_SETTING_SET),
                vec![vec![SETTING_WATER_ALPHA, word(1.0)]]
            );
            let colors = q.e.mem.u32(data + 12);
            assert_eq!(q.e.mem.f32(colors + 12), 1.0);
            q.e.set_global(SETTING_WATER_ALPHA + 4, -2.0f32);
            let data = q.create(2.0, 2.0, 1, false, true);
            assert_eq!(
                calls(&q.e, FLOAT_SETTING_SET),
                vec![vec![SETTING_WATER_ALPHA, word(0.0)]]
            );
            let colors = q.e.mem.u32(data + 12);
            assert_eq!(q.e.mem.f32(colors + 12), 0.0);
        }

        #[test]
        fn a_failed_allocation_frees_what_was_allocated() {
            // The vertices cannot be allocated.
            let mut q = quad();
            q.e.register(OPERATOR_NEW, |_, _| ret(0));
            assert_eq!(q.create(2.0, 2.0, 1, true, true), 0);
            assert!(calls(&q.e, OPERATOR_DELETE).is_empty());
            assert_eq!(calls(&q.e, ALLOCATION_SCOPE_DESTRUCT).len(), 1);
            // The texture coordinates cannot.
            let mut q = quad();
            q.e.register_double(OPERATOR_NEW, {
                let mut count = 0;
                move |e, a| {
                    count += 1;
                    ret(if count == 2 { 0 } else { e.mem.alloc(a[0]) })
                }
            });
            assert_eq!(q.create(2.0, 2.0, 1, true, true), 0);
            assert_eq!(calls(&q.e, OPERATOR_DELETE).len(), 1);
            // The triangles cannot.
            let mut q = quad();
            q.e.register(NI_ALLOC_ARRAY, |_, _| ret(0));
            assert_eq!(q.create(2.0, 2.0, 1, true, true), 0);
            assert_eq!(calls(&q.e, OPERATOR_DELETE).len(), 2);
            // The data object cannot be allocated: everything is freed.
            let mut q = quad();
            q.e.register(NI_ALLOC, |_, _| ret(0));
            assert_eq!(q.create(2.0, 2.0, 1, true, true), 0);
            assert_eq!(calls(&q.e, OPERATOR_DELETE).len(), 5);
            assert_eq!(calls(&q.e, ALLOCATION_SCOPE_DESTRUCT).len(), 1);
        }

        #[test]
        fn the_square_wrapper_builds_a_square_with_the_given_flags() {
            let mut q = quad();
            start_log(&mut q.e);
            let data =
                q.e.call(
                    0x004e_7730,
                    &args![0x99u32, 3.0f32, 0x200u32, 2u32, true, false],
                )
                .u32();
            assert_ne!(data, 0);
            let vertices = q.e.mem.u32(data + 4);
            assert_eq!(
                q.floats(vertices, 12),
                vec![1.5, 1.5, 0.0, -1.5, 1.5, 0.0, -1.5, -1.5, 0.0, 1.5, -1.5, 0.0]
            );
            assert_ne!(q.e.mem.u32(data + 8), 0, "normals");
            assert_eq!(q.e.mem.u32(data + 12), 0, "no colors");
            let uvs = q.e.mem.u32(data + 16);
            assert_eq!(
                q.floats(uvs, 8),
                vec![2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0]
            );
        }
    }

    /// The tests of the functions from `004e8000` on (third session).
    mod third_session {
        use super::second_session::{
            float_arg, group, list_items, node, quad, quiet, reference, REF_BASE_FORM, REF_CELL,
            REF_LOCATION, REF_NODE,
        };
        use super::*;
        use std::cell::RefCell;
        use std::collections::HashMap;
        use std::rc::Rc;

        const ACTOR_VTABLE: u32 = 0x0200_9000;
        const NOT_A_NODE_VTABLE: u32 = 0x0200_a000;
        const ACTOR_SKIP_DOUBLE: u32 = 0x0300_0201;
        const NOT_A_NODE_DOUBLE: u32 = 0x0300_0202;
        const ACTOR_LOCATION_DOUBLE: u32 = 0x0300_0203;
        const GREY_TEXTURE: u32 = 0x0e00_0001;
        const RENDER_OBJECT_VTABLE: u32 = 0x0200_b000;
        const RENDER_OBJECT_SET_CAMERA_DATA_DOUBLE: u32 = 0x0300_0204;
        /// The word the render object accessor `0043c4b0` returns.
        const RENDER_OBJECT_GLOBAL: u32 = 0x011f_4748;
        /// The doubles of the node virtuals `NODE_ATTACH_CHILD` and `NODE_DETACH_CHILD`.
        const ATTACH_CHILD_DOUBLE_ADDRESS: u32 = 0x0300_0105;
        const DETACH_CHILD_DOUBLE_ADDRESS: u32 = 0x0300_0106;

        // Fields of the test objects (the doubles read them).
        /// Actor: answers of the `+0x22c` virtual, `00885520` and `004938e0`.
        const ACTOR_SKIP: u32 = 0xc0;
        const ACTOR_IN_WATER: u32 = 0xc4;
        const ACTOR_STATE: u32 = 0xc5;
        /// Node: the first child (`NODE_FIRST_CHILD` reads `+0xd0 + 4 * index`),
        /// the local translate (+0x58) and the answer of the range test.
        const CHILD: u32 = 0xd0;
        const NODE_LOCAL: u32 = 0x58;
        const NODE_RANGE: u32 = 0xf0;
        /// Node (the geometry): the owner's `+0xc0`, the property at +0xa0 and
        /// the owner's type at +0x68 are those `water_engine` reads.
        const GEOMETRY_PROPERTY: u32 = 0xa0;
        const GEOMETRY_OWNER: u32 = 0xc0;

        /// The engine for the third batch: the allocation doubles of the quad
        /// tests, math and renderer doubles, and the pages the exe globals
        /// sit on.
        fn engine() -> Engine {
            let mut e = quad().e;
            for page in [
                0x0101_6000,
                0x0101_8000,
                0x0101_d000,
                0x0102_2000,
                0x011c_6000,
                0x011d_8000,
            ] {
                e.map(page, 0x1000);
            }
            e.set_global(FOUR, 4.0f64);
            e.set_global(FIFTY, 50.0f64);
            e.set_global(HIGHEST_FLOAT, f32::MAX);
            e.set_global(HIGHEST_FLOAT_AS_DOUBLE, f32::MAX as f64);
            e.set_global(WADING_NEAR_PLANE, 0.1f32);
            e.set_global(MAP_LOW, -0.5f32);
            e.set_global(MAP_HIGH, 0.5f32);
            e.set_global(MAP_SOURCE_LOW, -512.0f32);
            e.set_global(MAP_SOURCE_HIGH, 512.0f32);
            for (source, value) in DISPLACEMENT_CONSTANT_SOURCES
                .into_iter()
                .zip([0.4f32, 0.6, 0.97, 0.01])
            {
                e.set_global(source, value);
            }
            e.register(LINEAR_MAP, |_, a| {
                let (low, high) = (float_arg(a, 0), float_arg(a, 1));
                let (source_low, source_high) = (float_arg(a, 2), float_arg(a, 3));
                let value = float_arg(a, 4);
                ((high - low) * ((value - source_low) / (source_high - source_low)) + low)
                    .into_ret()
            });
            e.register(FLOAT_ABS, |_, a| float_arg(a, 0).abs().into_ret());
            e.register(FLOAT_FLOOR, |_, a| float_arg(a, 0).floor().into_ret());
            e.register(POINT3_SUBTRACT, |e, a| {
                for i in 0..3 {
                    let value = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                    e.mem.set_f32(a[1] + 4 * i, value);
                }
                ret(a[1])
            });
            e.register(POINT3_LENGTH, |e, a| {
                let sum: f32 = (0..3).map(|i| e.mem.f32(a[0] + 4 * i).powi(2)).sum();
                sum.sqrt().into_ret()
            });
            e.register(POINT2_LENGTH, |e, a| {
                let sum: f32 = (0..2).map(|i| e.mem.f32(a[0] + 4 * i).powi(2)).sum();
                sum.sqrt().into_ret()
            });
            e.register(LIST_FIRST_ITEM_SLOT, |e, a| ret(e.mem.u32(a[0]) + 8));
            e.register(NODE_FIRST_CHILD, |e, a| {
                ret(e.mem.u32(a[0] + CHILD + 4 * a[1]))
            });
            e.register(NODE_LOCAL_TRANSLATE_ADDRESS, |_, a| ret(a[0] + NODE_LOCAL));
            e.register(PLAYER_POSITION, |_, a| ret(a[0] + REF_LOCATION));
            e.register(INITIALIZE_GREY_TEXTURE, |_, _| ret(GREY_TEXTURE));
            e.register(RENDERED_TEXTURE_GET_TEXTURE, |_, a| ret(a[0] + 0x10));
            e.register(NODE_IN_RANGE_OF_VIEWER, |e, a| {
                ret(e.mem.u8(a[0] + NODE_RANGE) as u32)
            });
            e.register(IS_KIND_OF, |_, a| {
                ret((a[0] == WATER_SHADER_PROPERTY_RTTI && a[1] != 0) as u32)
            });
            e.register(TRI_SHAPE_CONSTRUCT, |_, a| ret(a[0]));
            e.register(CELL_HAS_WATER, |e, a| {
                ret(((e.mem.u8(a[0] + 0x24) & 2) != 0) as u32)
            });
            e.register(WATER_SHADER_ENABLED, |_, _| ret(1));
            e.register(TES_GET_WORLD_SPACE, |e, a| ret(e.mem.u32(a[0] + 0x50)));
            e.register(ACTOR_IN_WATER_TEST, |e, a| {
                ret(e.mem.u8(a[0] + ACTOR_IN_WATER) as u32)
            });
            e.register(ACTOR_STATE_TEST, |e, a| {
                ret(e.mem.u8(a[0] + ACTOR_STATE) as u32)
            });
            e.register(ACTOR_SKIP_DOUBLE, |e, a| {
                ret(e.mem.u8(a[0] + ACTOR_SKIP) as u32)
            });
            e.register(ACTOR_LOCATION_DOUBLE, |_, a| ret(a[0] + REF_LOCATION));
            e.register(NOT_A_NODE_DOUBLE, |_, _| ret(0));
            vtable(
                &mut e,
                ACTOR_VTABLE,
                &[
                    (REFERENCE_GET_3D, REFERENCE_GET_3D_DOUBLE),
                    (REFERENCE_GET_LOCATION_VIRTUAL, ACTOR_LOCATION_DOUBLE),
                    (ACTOR_SKIP_VIRTUAL, ACTOR_SKIP_DOUBLE),
                ],
            );
            vtable(
                &mut e,
                NOT_A_NODE_VTABLE,
                &[(NODE_IS_NODE_VIRTUAL, NOT_A_NODE_DOUBLE)],
            );
            e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
            let renderer = object(&mut e, RENDERER_VTABLE, 0x100);
            e.register_double(RENDERER, move |_, _| ret(renderer));
            e.register(TEXTURE_MANAGER, |_, _| ret(0x0aaa_0000));
            e.register(RENDERED_TEXTURE_STOP, |_, a| ret(a[0] + 1));
            e.register(RENDER_SETTING_ENTRY, |_, _| ret(2));
            e.register(ZONE_MAP_FIRST, |_, _| ret(0));
            e.register(FLOAT_TABLE_ENTRY, |_, _| 0.5f32.into_ret());
            let render_object = object(&mut e, RENDER_OBJECT_VTABLE, 0x300);
            e.set_global(RENDER_OBJECT_GLOBAL, render_object);
            e.register(RENDER_GLOBAL_OBJECT, |e, _| {
                ret(e.global(RENDER_OBJECT_GLOBAL))
            });
            e.register(RENDER_OBJECT_SET_CAMERA_DATA_DOUBLE, |_, _| Ret::default());
            vtable(
                &mut e,
                RENDER_OBJECT_VTABLE,
                &[(
                    RENDER_OBJECT_SET_CAMERA_DATA_VIRTUAL,
                    RENDER_OBJECT_SET_CAMERA_DATA_DOUBLE,
                )],
            );
            quiet(
                &mut e,
                &[
                    RENDER_TARGET_RESET,
                    RENDER_TARGET_SET,
                    CAMERA_CONSTRUCT,
                    CAMERA_DESTRUCT,
                    FRUSTUM_CONSTRUCT,
                    CULLING_PROCESS_CONSTRUCT,
                    CULLING_PROCESS_DESTRUCT,
                    CULLING_PROCESS_SET_CAMERA,
                    CULLING_PROCESS_SET_PLANES,
                    CAMERA_SET_VIEW_FRUSTUM,
                    CAMERA_LOOK_AT_WORLD_POINT,
                    RENDER_OBJECT_SET_CAMERA_DATA,
                    IMAGE_SPACE_RENDER_DISPLACEMENT,
                ],
            );
            e.register(CAMERA_FRUSTUM_ADDRESS, |_, a| ret(a[0] + 0x100));
            e.register(CAMERA_PLANES_ADDRESS, |_, a| ret(a[0] + 0xdc));
            e.register(CAMERA_COLUMN_ZERO, |e, a| {
                for i in 0..3 {
                    e.mem.set_f32(a[1] + 4 * i, 10.0 + i as f32);
                }
                ret(a[1])
            });
            e.register(MATRIX_COLUMN, |e, a| {
                for i in 0..3 {
                    e.mem.set_f32(a[2] + 4 * i, 20.0 * a[1] as f32 + i as f32);
                }
                Ret::default()
            });
            e.register(RENDER_OBJECT_CHECK, |_, _| ret(1));
            // `water_engine` stands in for the water geometry lookup; the real one
            // is under test here.
            for (address, function) in funcs() {
                if address == WATER_REFERENCE_3D {
                    e.register(address, function);
                }
            }
            e
        }

        /// A test actor: its location is `REF_LOCATION`.
        fn actor(e: &mut Engine, x: f32, y: f32, z: f32) -> u32 {
            let actor = object(e, ACTOR_VTABLE, 0x100);
            for (i, value) in [x, y, z].into_iter().enumerate() {
                e.mem.set_f32(actor + REF_LOCATION + 4 * i as u32, value);
            }
            actor
        }

        /// Makes a player at the location and installs it as the global.
        fn player_at(e: &mut Engine, x: f32, y: f32, z: f32) -> u32 {
            let player = reference(e);
            for (i, value) in [x, y, z].into_iter().enumerate() {
                e.mem.set_f32(player + REF_LOCATION + 4 * i as u32, value);
            }
            e.set_global(PLAYER_CHARACTER, player);
            player
        }

        /// Sets a float setting's value (`SETTING_FLOAT_VALUE` reads +4).
        fn set_float_setting(e: &mut Engine, setting: u32, value: f32) {
            e.set_global(setting + 4, value);
        }

        /// A water system with `groups` in its group list.
        fn system_with(e: &mut Engine, groups: &[u32]) -> Ptr<TESWaterSystem> {
            let system = e.new_object::<TESWaterSystem>();
            fill_list(e, system.addr() + 0x3c, groups);
            system
        }

        // --- 004e8000 ---------------------------------------------------------

        #[test]
        fn the_water_root_is_released_and_its_slot_cleared() {
            let mut e = engine();
            quiet(&mut e, &[WATER_ROOT_RELEASE_STEP]);
            e.set_global(WATER_ROOT_SLOT, 0x7777u32);
            start_log(&mut e);
            e.call(0x004e_8000, &[]);
            assert_eq!(calls(&e, WATER_ROOT_RELEASE_STEP), vec![vec![0x7777]]);
            assert_eq!(calls(&e, NI_POINTER_ASSIGN), vec![vec![WATER_ROOT_SLOT, 0]]);
            assert_eq!(e.global::<u32>(WATER_ROOT_SLOT), 0);
            // Without a root nothing is released, the slot is still assigned.
            start_log(&mut e);
            e.call(0x004e_8000, &[]);
            assert!(calls(&e, WATER_ROOT_RELEASE_STEP).is_empty());
            assert_eq!(calls(&e, NI_POINTER_ASSIGN).len(), 1);
        }

        // --- 004e8030 ---------------------------------------------------------

        /// A reference whose 3D node has a child with a child with a child:
        /// `(reference, child, grandchild, great grandchild)`.
        fn geometry_chain(e: &mut Engine) -> (u32, u32, u32, u32) {
            let reference = reference(e);
            let top = node(e);
            let child = node(e);
            let grandchild = node(e);
            let great = node(e);
            e.mem.set_u32(reference + REF_NODE, top);
            e.mem.set_u32(top + CHILD, child);
            e.mem.set_u32(child + CHILD, grandchild);
            e.mem.set_u32(grandchild + CHILD, great);
            (reference, child, grandchild, great)
        }

        fn geometry_of(e: &mut Engine, system: Ptr<TESWaterSystem>, reference: u32) -> u32 {
            e.call(0x004e_8030, &args![system, reference]).u32()
        }

        #[test]
        fn the_water_geometry_is_the_first_child_of_the_first_child() {
            let mut e = engine();
            let system = e.new_object::<TESWaterSystem>();
            let (reference, _, grandchild, _) = geometry_chain(&mut e);
            assert_eq!(geometry_of(&mut e, system, reference), grandchild);
            assert_eq!(
                e.get(system, TESWaterSystem::iGlobalGetWaterGeometryCount),
                1
            );
            geometry_of(&mut e, system, reference);
            assert_eq!(
                e.get(system, TESWaterSystem::iGlobalGetWaterGeometryCount),
                2
            );
        }

        #[test]
        fn the_40000000_flag_goes_one_level_deeper() {
            let mut e = engine();
            let system = e.new_object::<TESWaterSystem>();
            let (reference, _, _, great) = geometry_chain(&mut e);
            let base_form = e.mem.u32(reference + 0x20);
            e.mem.set_u32(base_form + BASE_FORM_FLAGS, 0x4000_0000);
            assert_eq!(geometry_of(&mut e, system, reference), great);
            // A grandchild that is not a node gives null.
            let (reference, _, grandchild, _) = geometry_chain(&mut e);
            let base_form = e.mem.u32(reference + 0x20);
            e.mem.set_u32(base_form + BASE_FORM_FLAGS, 0x4000_0000);
            e.mem.set_u32(grandchild, NOT_A_NODE_VTABLE);
            assert_eq!(geometry_of(&mut e, system, reference), 0);
        }

        #[test]
        fn a_missing_reference_node_or_child_gives_null() {
            let mut e = engine();
            let system = e.new_object::<TESWaterSystem>();
            assert_eq!(geometry_of(&mut e, system, 0), 0);
            // No 3D object: null, and the counter is not raised.
            let bare = reference(&mut e);
            assert_eq!(geometry_of(&mut e, system, bare), 0);
            assert_eq!(
                e.get(system, TESWaterSystem::iGlobalGetWaterGeometryCount),
                0
            );
            // A first child that is not a node.
            let (reference, child, _, _) = geometry_chain(&mut e);
            e.mem.set_u32(child, NOT_A_NODE_VTABLE);
            assert_eq!(geometry_of(&mut e, system, reference), 0);
            // No child at all.
            let (reference, child, _, _) = geometry_chain(&mut e);
            let top = e.mem.u32(reference + REF_NODE);
            let _ = child;
            e.mem.set_u32(top + CHILD, 0);
            assert_eq!(geometry_of(&mut e, system, reference), 0);
            // The reference's node is not a node.
            let (reference, _, _, _) = geometry_chain(&mut e);
            let top = e.mem.u32(reference + REF_NODE);
            e.mem.set_u32(top, NOT_A_NODE_VTABLE);
            assert_eq!(geometry_of(&mut e, system, reference), 0);
        }

        // --- a water system with one group, for the wading-water tests -----------

        type Translations = Rc<RefCell<Vec<(u32, [f32; 3])>>>;

        struct Wading {
            e: Engine,
            system: Ptr<TESWaterSystem>,
            group: Ptr<PlaceableWaterGroup>,
            root: u32,
            /// The geometry of the group's water reference and the property
            /// it already has.
            other_property: u32,
            /// The group's water reference and its geometry.
            water_reference: u32,
            geometry: u32,
            /// The calls of `NODE_SET_LOCAL_TRANSLATE`: node and position.
            translations: Translations,
            /// The `WadingWaterMap` the map doubles keep (key to value).
            map: Rc<RefCell<HashMap<u32, u32>>>,
        }

        /// A system with one group whose plane is at `height`, one water
        /// reference (with a geometry that has a water shader property of its
        /// own), the player at `(10, 20, 0)`, the water root node and doubles
        /// for the wading map.
        fn wading(height: f32) -> Wading {
            let mut e = engine();
            quiet(
                &mut e,
                &[
                    NODE_UPDATE,
                    UPDATE_DATA_CONSTRUCT,
                    NODE_ATTACH_PROPERTY,
                    SHADER_MANAGER_PREPARE_OBJECT,
                    WATER_PROPERTY_METHOD_00B6AB20,
                    RETURN_RENDERED_TEXTURE,
                ],
            );
            e.register(WATER_SHADER_PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
            let translations: Translations = Rc::default();
            {
                let log = translations.clone();
                e.register_double(NODE_SET_LOCAL_TRANSLATE, move |e, a| {
                    let point = [e.mem.f32(a[1]), e.mem.f32(a[1] + 4), e.mem.f32(a[1] + 8)];
                    log.borrow_mut().push((a[0], point));
                    Ret::default()
                });
            }
            let map: Rc<RefCell<HashMap<u32, u32>>> = Rc::default();
            {
                let entries = map.clone();
                e.register_double(WADING_MAP_GET, move |e, a| {
                    match entries.borrow().get(&a[1]) {
                        Some(value) => {
                            e.mem.set_u32(a[2], *value);
                            ret(1)
                        }
                        None => ret(0),
                    }
                });
                let entries = map.clone();
                e.register_double(MAP_SET_AT, move |_, a| {
                    entries.borrow_mut().insert(a[1], a[2]);
                    Ret::default()
                });
                let entries = map.clone();
                e.register_double(WADING_MAP_REMOVE, move |_, a| {
                    entries.borrow_mut().remove(&a[1]);
                    ret(1)
                });
            }
            let root = node(&mut e);
            e.set_global(WATER_ROOT_SLOT, root);
            let water_type = e.mem.alloc(0x200);
            e.mem.set_u32(water_type + 0x30, 0x0e00_0002);
            let group_address = group(&mut e, height, water_type);
            let group: Ptr<PlaceableWaterGroup> = Ptr::new(group_address);
            e.set(group, PlaceableWaterGroup::iStencilBitMask, 0x55);
            e.set(
                group,
                PlaceableWaterGroup::bGroupAtWorldSpaceWaterHeight,
                true,
            );
            e.set(group, PlaceableWaterGroup::bRenderGroup, true);
            // The water reference of the group, with an owner of type 0xd.
            let (water_reference, _, geometry, _) = geometry_chain(&mut e);
            let owner = e.mem.alloc(0x100);
            e.mem.set_u32(owner + 0x68, WATER_OWNER_TYPE);
            e.mem.set_u32(geometry + GEOMETRY_OWNER, owner);
            let other_property = e.mem.alloc(0x150);
            e.mem.set_u32(other_property + 0x84, 7);
            e.mem.set_u32(geometry + GEOMETRY_PROPERTY, other_property);
            fill_list(&mut e, group_address + 0x24, &[water_reference]);
            let system = system_with(&mut e, &[group_address]);
            e.set_global(DEPTH_MAP, 0x0e00_0003u32);
            e.set_global(WORLD_REFLECTION_MAP, 0x0e00_0004u32);
            e.set_global(SKY_REFLECTION_MAP, 0x0e00_0005u32);
            e.set_global(WADING_WATER_HEIGHT_MAP, 0x0e00_0100u32);
            player_at(&mut e, 10.0, 20.0, 0.0);
            set_float_setting(&mut e, SETTING_WADING_WATER_QUAD_SIZE, 100.0);
            set_float_setting(&mut e, SETTING_WATER_GROUP_HEIGHT_RANGE, 5.0);
            set_setting(&mut e, SETTING_USE_BULLET_WATER_DISPLACEMENTS, true);
            set_setting(&mut e, SETTING_USE_WATER_DISPLACEMENTS, true);
            Wading {
                e,
                system,
                group,
                root,
                other_property,
                water_reference,
                geometry,
                translations,
                map,
            }
        }

        fn floats_at(e: &Engine, address: u32, count: u32) -> Vec<f32> {
            (0..count).map(|i| e.mem.f32(address + 4 * i)).collect()
        }

        // --- 004e8160 -----------------------------------------------------------

        #[test]
        fn the_wading_geometry_is_a_quad_on_the_water_root_with_its_own_property() {
            let mut w = wading(7.0);
            w.e.set(w.system, TESWaterSystem::fTimeSinceLastRipplePlaced, 5.0);
            start_log(&mut w.e);
            let shape =
                w.e.call(0x004e_8160, &args![w.system, w.group, w.root, 64.0f32])
                    .u32();
            assert_ne!(shape, 0);
            // The quad: side 64, with normals and colours.
            let data = calls(&w.e, TRI_SHAPE_CONSTRUCT)[0][1];
            assert_eq!(
                floats_at(&w.e, w.e.mem.u32(data + 4), 3),
                vec![32.0, 32.0, 0.0]
            );
            assert_ne!(w.e.mem.u32(data + 8), 0, "normals");
            assert_ne!(w.e.mem.u32(data + 12), 0, "colours");
            // It sits at the player and hangs on the root.
            assert_eq!(w.translations.borrow()[0], (shape, [10.0, 20.0, 0.0]));
            assert_eq!(
                calls(&w.e, ATTACH_CHILD_DOUBLE_ADDRESS),
                vec![vec![w.root, shape, 1]]
            );
            // The property.
            let property = calls(&w.e, WATER_SHADER_PROPERTY_CONSTRUCT)[0][0];
            let property_ptr: Ptr<WaterShaderProperty> = Ptr::new(property);
            assert_eq!(
                calls(&w.e, NODE_ATTACH_PROPERTY),
                vec![vec![shape, property]]
            );
            assert_eq!(
                calls(&w.e, SHADER_MANAGER_PREPARE_OBJECT),
                vec![vec![shape, 0, 0]]
            );
            assert!(w.e.get(property_ptr, WaterShaderProperty::bDisplacement));
            assert_eq!(
                w.e.get(property_ptr, WaterShaderProperty::fBlendRadius),
                1.0
            );
            assert_eq!(
                w.e.get(property_ptr, WaterShaderProperty::fBlendNormalsAmount),
                1.0
            );
            assert_eq!(
                w.e.get(property_ptr, WaterShaderProperty::iStencilMask),
                0x55
            );
            assert_eq!(
                w.e.get(property_ptr, WaterShaderProperty::spDepthMap),
                0x0e00_0003
            );
            assert_eq!(
                w.e.get(property_ptr, WaterShaderProperty::spNoiseNormalMap),
                0x0e00_0002
            );
            assert_eq!(
                w.e.get(property_ptr, WaterShaderProperty::spReflectionMap),
                0x0e00_0004,
                "the group is at the world height"
            );
            assert_eq!(
                w.e.get(property_ptr, WaterShaderProperty::spDisplacementNormalMap),
                0x0e00_0110
            );
            // The property of the first water reference's geometry is told about it.
            assert_eq!(
                calls(&w.e, WATER_PROPERTY_METHOD_00B6AB20),
                vec![vec![w.other_property, property]]
            );
            assert_eq!(
                w.e.get(w.system, TESWaterSystem::fTimeSinceLastRipplePlaced),
                0.0
            );
        }

        #[test]
        fn without_a_player_the_wading_geometry_is_at_the_origin() {
            let mut w = wading(7.0);
            w.e.set_global(PLAYER_CHARACTER, 0u32);
            // The group's reference has no 3D object: no property method call.
            let bare = reference(&mut w.e);
            fill_list(&mut w.e, w.group.addr() + 0x24, &[bare]);
            // The sky reflection map is used away from the world height.
            w.e.set(
                w.group,
                PlaceableWaterGroup::bGroupAtWorldSpaceWaterHeight,
                false,
            );
            start_log(&mut w.e);
            let shape =
                w.e.call(0x004e_8160, &args![w.system, w.group, w.root, 8.0f32])
                    .u32();
            assert_eq!(w.translations.borrow()[0], (shape, [0.0, 0.0, 0.0]));
            assert!(calls(&w.e, WATER_PROPERTY_METHOD_00B6AB20).is_empty());
            let property = calls(&w.e, WATER_SHADER_PROPERTY_CONSTRUCT)[0][0];
            assert_eq!(
                w.e.get(
                    Ptr::<WaterShaderProperty>::new(property),
                    WaterShaderProperty::spReflectionMap
                ),
                0x0e00_0005
            );
        }

        // --- 004e83e0 -----------------------------------------------------------

        fn actors_near(e: &mut Engine, actors: &[u32]) -> bool {
            let group_address = group(e, 0.0, 0);
            fill_list(e, group_address + 0x3c, actors);
            e.call(0x004e_83e0, &args![0x1234u32, group_address]).bool()
        }

        #[test]
        fn the_player_in_the_group_is_always_near() {
            let mut e = engine();
            let player = player_at(&mut e, 0.0, 0.0, 0.0);
            let far = actor(&mut e, 1000.0, 0.0, 0.0);
            set_float_setting(&mut e, SETTING_WADING_WATER_QUAD_SIZE, 1.0);
            assert!(actors_near(&mut e, &[far, player]));
            assert!(!actors_near(&mut e, &[far]));
            assert!(!actors_near(&mut e, &[]));
        }

        #[test]
        fn an_actor_is_near_when_it_is_within_the_quad_size_ignoring_heights() {
            let mut e = engine();
            player_at(&mut e, 0.0, 0.0, 0.0);
            set_float_setting(&mut e, SETTING_WADING_WATER_QUAD_SIZE, 10.0);
            let exactly = actor(&mut e, 6.0, 8.0, 500.0);
            assert!(
                actors_near(&mut e, &[exactly]),
                "distance 10, heights ignored"
            );
            let beyond = actor(&mut e, 6.0, 8.5, 0.0);
            assert!(!actors_near(&mut e, &[beyond]));
            // The second actor of the list is looked at too.
            assert!(actors_near(&mut e, &[beyond, exactly]));
        }

        #[test]
        fn an_actor_the_virtual_skips_is_not_near() {
            let mut e = engine();
            player_at(&mut e, 0.0, 0.0, 0.0);
            set_float_setting(&mut e, SETTING_WADING_WATER_QUAD_SIZE, 10.0);
            let skipped = actor(&mut e, 1.0, 1.0, 0.0);
            e.mem.set_u8(skipped + ACTOR_SKIP, 1);
            assert!(!actors_near(&mut e, &[skipped]));
        }

        // --- 004e8510 -----------------------------------------------------------

        type Points = Rc<RefCell<Vec<(u32, f32, f32)>>>;

        /// Records the elements put into the displacement arrays.
        fn record_points(e: &mut Engine) -> Points {
            let points: Points = Rc::default();
            let log = points.clone();
            e.register_double(POINT2_ARRAY_APPEND, move |e, a| {
                log.borrow_mut()
                    .push((a[0], e.mem.f32(a[1]), e.mem.f32(a[1] + 4)));
                Ret::default()
            });
            points
        }

        fn add_ripple(w: &mut Wading, x: f32, y: f32, height: f32, size: f32) {
            start_log(&mut w.e);
            w.e.call(0x004e_8510, &args![w.system, x, y, height, size]);
        }

        #[test]
        fn a_ripple_near_the_player_creates_the_geometry_and_fills_the_arrays() {
            let mut w = wading(50.0);
            let points = record_points(&mut w.e);
            w.e.set_global(WADING_WATER_HEIGHT_MAP, 0u32);
            add_ripple(&mut w, 13.0, 24.0, 52.0, 4.0);
            // The grey height map was created, the geometry too.
            assert_eq!(w.e.global::<u32>(WADING_WATER_HEIGHT_MAP), GREY_TEXTURE);
            assert_eq!(w.e.global::<u8>(WADING_MAP_CREATED_FLAG), 1);
            let geometry = w.e.get(w.group, PlaceableWaterGroup::spWadingWaterGeometry);
            assert_ne!(geometry, 0);
            // The geometry is put at the player's x and y and the group's height.
            let translations = w.translations.borrow().clone();
            assert_eq!(translations[0], (geometry, [10.0, 20.0, 0.0]));
            assert_eq!(translations[1], (geometry, [10.0, 20.0, 50.0]));
            assert_eq!(calls(&w.e, NODE_UPDATE)[0][0], geometry);
            // The offset (3, 4) maps to 3/1024 and 4/1024; the first array gets it
            // plus half the size, the second the same offset again.
            assert_eq!(
                *points.borrow(),
                vec![
                    (
                        DISPLACEMENT_POINTS_A,
                        2.0 + 3.0 / 1024.0,
                        2.0 + 4.0 / 1024.0
                    ),
                    (DISPLACEMENT_POINTS_B, 3.0 / 1024.0, 4.0 / 1024.0),
                ]
            );
        }

        #[test]
        fn a_ripple_keeps_the_geometry_a_group_already_has() {
            let mut w = wading(50.0);
            let points = record_points(&mut w.e);
            w.e.set(w.group, PlaceableWaterGroup::spWadingWaterGeometry, 0x5151);
            add_ripple(&mut w, 13.0, 24.0, 52.0, 4.0);
            assert!(w.translations.borrow().is_empty());
            assert_eq!(
                w.e.get(w.group, PlaceableWaterGroup::spWadingWaterGeometry),
                0x5151
            );
            assert_eq!(points.borrow().len(), 2);
        }

        #[test]
        fn a_ripple_far_from_the_group_height_creates_no_geometry() {
            let mut w = wading(50.0);
            let points = record_points(&mut w.e);
            add_ripple(&mut w, 13.0, 24.0, 60.0, 4.0);
            assert_eq!(
                w.e.get(w.group, PlaceableWaterGroup::spWadingWaterGeometry),
                0
            );
            assert!(w.translations.borrow().is_empty());
            assert_eq!(points.borrow().len(), 2, "the arrays are filled anyway");
        }

        #[test]
        fn a_ripple_beyond_the_quad_size_or_with_the_settings_off_does_nothing() {
            let mut w = wading(50.0);
            let points = record_points(&mut w.e);
            add_ripple(&mut w, 200.0, 20.0, 50.0, 4.0);
            assert!(points.borrow().is_empty());
            assert!(calls(&w.e, FLOATS_NEAR).is_empty());
            set_setting(&mut w.e, SETTING_USE_WATER_DISPLACEMENTS, false);
            add_ripple(&mut w, 13.0, 24.0, 50.0, 4.0);
            assert!(points.borrow().is_empty());
            assert!(calls(&w.e, POINT2_LENGTH).is_empty());
            set_setting(&mut w.e, SETTING_USE_WATER_DISPLACEMENTS, true);
            set_setting(&mut w.e, SETTING_USE_BULLET_WATER_DISPLACEMENTS, false);
            add_ripple(&mut w, 13.0, 24.0, 50.0, 4.0);
            assert!(points.borrow().is_empty());
            assert_eq!(calls(&w.e, SETTING_VALUE).len(), 1);
        }

        // --- the NiPoint2 operators ---------------------------------------------

        #[test]
        fn point2_subtraction_writes_the_difference_to_out() {
            let mut e = engine();
            let block = e.mem.alloc(0x20);
            e.mem.set_f32(block, 5.0);
            e.mem.set_f32(block + 4, 1.5);
            e.mem.set_f32(block + 8, 2.0);
            e.mem.set_f32(block + 12, 4.0);
            let out = block + 0x10;
            assert_eq!(
                e.call(0x004e_8880, &args![block, out, block + 8]).u32(),
                out
            );
            assert_eq!(floats_at(&e, out, 2), vec![3.0, -2.5]);
            assert_eq!(floats_at(&e, block, 4), vec![5.0, 1.5, 2.0, 4.0]);
        }

        #[test]
        fn point2_addition_and_subtraction_work_in_place() {
            let mut e = engine();
            let block = e.mem.alloc(0x10);
            e.mem.set_f32(block, 1.0);
            e.mem.set_f32(block + 4, 2.0);
            e.mem.set_f32(block + 8, 0.25);
            e.mem.set_f32(block + 12, -4.0);
            assert_eq!(e.call(0x004e_88d0, &args![block, block + 8]).u32(), block);
            assert_eq!(floats_at(&e, block, 2), vec![1.25, -2.0]);
            assert_eq!(e.call(0x004e_8910, &args![block, block + 8]).u32(), block);
            assert_eq!(floats_at(&e, block, 2), vec![1.0, 2.0]);
            assert_eq!(floats_at(&e, block + 8, 2), vec![0.25, -4.0]);
        }

        // --- 004e8950, 004e89e0 -------------------------------------------------

        #[test]
        fn the_displacement_setting_byte_is_read() {
            let mut e = engine();
            set_setting(&mut e, SETTING_USE_WATER_DISPLACEMENTS, true);
            assert_eq!(e.call(0x004e_89e0, &[]).u8(), 1);
            set_setting(&mut e, SETTING_USE_WATER_DISPLACEMENTS, false);
            assert_eq!(e.call(0x004e_89e0, &[]).u8(), 0);
        }

        #[test]
        fn displacements_are_active_when_the_setting_is_on_and_a_group_has_a_near_actor() {
            let mut e = engine();
            let player = player_at(&mut e, 0.0, 0.0, 0.0);
            set_float_setting(&mut e, SETTING_WADING_WATER_QUAD_SIZE, 10.0);
            let far = actor(&mut e, 1000.0, 0.0, 0.0);
            let empty = group(&mut e, 0.0, 0);
            let without = group(&mut e, 0.0, 0);
            fill_list(&mut e, without + 0x3c, &[far]);
            let with = group(&mut e, 0.0, 0);
            fill_list(&mut e, with + 0x3c, &[far, player]);
            set_setting(&mut e, SETTING_USE_WATER_DISPLACEMENTS, true);
            let system = system_with(&mut e, &[empty, without]);
            assert!(!e.call(0x004e_8950, &args![system]).bool());
            let system = system_with(&mut e, &[empty, without, with]);
            assert!(e.call(0x004e_8950, &args![system]).bool());
            // With the setting off the groups are not looked at.
            set_setting(&mut e, SETTING_USE_WATER_DISPLACEMENTS, false);
            start_log(&mut e);
            assert!(!e.call(0x004e_8950, &args![system]).bool());
            assert!(calls(&e, LIST_NEXT_POSITION).is_empty());
        }

        // --- 004e8e40 -----------------------------------------------------------

        #[test]
        fn wading_water_data_starts_at_the_zero_points() {
            let mut e = engine();
            let data = e.mem.alloc(0x1c);
            for i in 0..7 {
                e.mem.set_u32(data + 4 * i, 0xdead_beef);
            }
            e.set_global(ZERO_POINT2, 0.5f32);
            e.set_global(ZERO_POINT2 + 4, 0.25f32);
            for (i, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
                e.set_global(ZERO_POINT3 + 4 * i as u32, value);
            }
            start_log(&mut e);
            assert_eq!(e.call(0x004e_8e40, &args![data]).u32(), data);
            assert_eq!(
                floats_at(&e, data, 7),
                vec![0.5, 0.25, 0.5, 0.25, 1.0, 2.0, 3.0]
            );
            assert_eq!(
                calls(&e, ADDRESS_OF_THIS),
                vec![vec![data], vec![data + 8], vec![data + 0x10]]
            );
        }

        // --- 004e8a00 -----------------------------------------------------------

        /// A wading scene with the player at the origin, the group's plane at
        /// 2.0 and one actor at `(8, -4, 7)` in the group's actor list.
        fn wading_actor() -> (Wading, u32) {
            let mut w = wading(2.0);
            player_at(&mut w.e, 0.0, 0.0, 0.0);
            let actor = actor(&mut w.e, 8.0, -4.0, 7.0);
            w.e.mem.set_u8(actor + ACTOR_STATE, 1);
            fill_list(&mut w.e, w.group.addr() + 0x3c, &[actor]);
            (w, actor)
        }

        fn update_wading(w: &mut Wading) {
            start_log(&mut w.e);
            w.e.call(0x004e_8a00, &args![w.system]);
        }

        fn wading_data(w: &Wading, actor: u32) -> Ptr<WadingWaterData> {
            Ptr::new(*w.map.borrow().get(&actor).expect("an entry for the actor"))
        }

        #[test]
        fn an_actor_out_of_the_water_gets_displacement_offsets_from_the_player() {
            let (mut w, actor) = wading_actor();
            update_wading(&mut w);
            // The group got its wading geometry.
            assert_ne!(
                w.e.get(w.group, PlaceableWaterGroup::spWadingWaterGeometry),
                0
            );
            // The actor's data: the offsets from the player mapped to -0.5..0.5,
            // a last offset equal to it (the entry starts with its last position at
            // the current one), and the position at the group height.
            let data = wading_data(&w, actor);
            assert_eq!(
                w.e.get(data, WadingWaterData::fDisplaceOffsetX),
                8.0 / 1024.0
            );
            assert_eq!(
                w.e.get(data, WadingWaterData::fDisplaceOffsetY),
                -4.0 / 1024.0
            );
            assert_eq!(
                w.e.get(data, WadingWaterData::fLastDisplaceOffsetX),
                8.0 / 1024.0
            );
            assert_eq!(
                w.e.get(data, WadingWaterData::fLastDisplaceOffsetY),
                -4.0 / 1024.0
            );
            assert_eq!(w.e.get(data, WadingWaterData::fLastPositionX), 8.0);
            assert_eq!(w.e.get(data, WadingWaterData::fLastPositionY), -4.0);
            assert_eq!(w.e.get(data, WadingWaterData::fLastPositionZ), 2.0);
            // The water test asked about the actor's location, parent cell and 1.0.
            let test = calls(&w.e, ACTOR_IN_WATER_TEST);
            assert_eq!(test.len(), 1);
            assert_eq!(test[0][0], actor);
            assert_eq!(test[0][1], actor + REF_LOCATION);
            assert_eq!(test[0][3], 1.0f32.to_bits());
        }

        #[test]
        fn a_second_update_keeps_the_entry_and_remembers_the_last_position() {
            let (mut w, actor) = wading_actor();
            update_wading(&mut w);
            let data = wading_data(&w, actor);
            w.e.mem.set_f32(actor + REF_LOCATION, 10.0);
            update_wading(&mut w);
            assert_eq!(wading_data(&w, actor).addr(), data.addr());
            assert_eq!(
                w.e.get(data, WadingWaterData::fDisplaceOffsetX),
                10.0 / 1024.0
            );
            assert_eq!(
                w.e.get(data, WadingWaterData::fLastDisplaceOffsetX),
                8.0 / 1024.0
            );
            assert_eq!(
                w.e.get(data, WadingWaterData::fLastDisplaceOffsetY),
                -4.0 / 1024.0
            );
            assert_eq!(w.e.get(data, WadingWaterData::fLastPositionX), 10.0);
            // The geometry exists already: no second quad.
            assert!(calls(&w.e, TRI_SHAPE_CONSTRUCT).is_empty());
        }

        #[test]
        fn an_actor_in_the_water_or_in_a_state_the_test_refuses_loses_its_entry() {
            let (mut w, actor) = wading_actor();
            update_wading(&mut w);
            assert_eq!(w.map.borrow().len(), 1);
            w.e.mem.set_u8(actor + ACTOR_IN_WATER, 1);
            update_wading(&mut w);
            assert!(w.map.borrow().is_empty());
            // The state test is asked only for an actor that is not in the water.
            assert!(calls(&w.e, ACTOR_STATE_TEST).is_empty());
            w.e.mem.set_u8(actor + ACTOR_IN_WATER, 0);
            update_wading(&mut w);
            assert_eq!(w.map.borrow().len(), 1);
            w.e.mem.set_u8(actor + ACTOR_STATE, 0);
            update_wading(&mut w);
            assert!(w.map.borrow().is_empty());
        }

        #[test]
        fn nothing_is_updated_without_a_rendered_group_with_a_near_actor() {
            // A group that is not rendered.
            let (mut w, _) = wading_actor();
            w.e.set(w.group, PlaceableWaterGroup::bRenderGroup, false);
            update_wading(&mut w);
            assert!(calls(&w.e, ACTOR_IN_WATER_TEST).is_empty());
            // An actor list that is empty.
            let (mut w, _) = wading_actor();
            fill_list(&mut w.e, w.group.addr() + 0x3c, &[]);
            update_wading(&mut w);
            assert!(calls(&w.e, ACTOR_IN_WATER_TEST).is_empty());
            // An actor beyond the quad size.
            let (mut w, _) = wading_actor();
            set_float_setting(&mut w.e, SETTING_WADING_WATER_QUAD_SIZE, 1.0);
            update_wading(&mut w);
            assert!(calls(&w.e, ACTOR_IN_WATER_TEST).is_empty());
            assert!(w.map.borrow().is_empty());
        }

        #[test]
        fn in_an_interior_without_water_nothing_is_updated() {
            let (mut w, _) = wading_actor();
            let cell = w.e.mem.alloc(0x40);
            let tes = w.e.global::<u32>(TES_POINTER);
            w.e.mem.set_u32(tes + 0x34, cell);
            update_wading(&mut w);
            assert!(calls(&w.e, LIST_IS_EMPTY).is_empty());
            // With water in the interior cell the update runs.
            w.e.mem.set_u8(cell + 0x24, 2);
            update_wading(&mut w);
            assert_eq!(w.map.borrow().len(), 1);
        }

        // --- 004e8ec0 -----------------------------------------------------------

        /// The wading scene with a geometry on the group, its property and the
        /// geometry's local translate at `(3, 0, 0)`; the player at `(x, 0, 0)`.
        fn simulation(player_x: f32) -> (Wading, u32, u32) {
            let mut w = wading(6.0);
            player_at(&mut w.e, player_x, 0.0, 0.0);
            let geometry = node(&mut w.e);
            w.e.mem.set_f32(geometry + NODE_LOCAL, 3.0);
            let property = w.e.mem.alloc(0x150);
            w.e.mem.set_u32(geometry + GEOMETRY_PROPERTY, property);
            w.e.set(
                w.group,
                PlaceableWaterGroup::spWadingWaterGeometry,
                geometry,
            );
            w.e.set(w.system, TESWaterSystem::fTimeSinceLastRipplePlaced, 1.0);
            (w, geometry, property)
        }

        fn simulate(w: &mut Wading) {
            start_log(&mut w.e);
            w.e.call(0x004e_8ec0, &args![w.system]);
        }

        #[test]
        fn a_player_who_moved_far_drags_the_geometry_along_on_a_four_unit_grid() {
            let (mut w, geometry, property) = simulation(10.0);
            simulate(&mut w);
            // The player x is snapped to the grid: floor(10 / 4) * 4 = 8.
            assert_eq!(*w.translations.borrow(), vec![(geometry, [8.0, 0.0, 6.0])]);
            assert_eq!(calls(&w.e, NODE_UPDATE)[0][0], geometry);
            // The shift is the move mapped onto the quad: 8 - 3 along x, 0 along y.
            let shift = (1.0f32 * ((5.0f32 + 50.0) / 100.0)) + -0.5;
            assert_eq!(w.e.global::<f32>(DISPLACEMENT_SHIFT_X), shift);
            assert_eq!(w.e.global::<f32>(DISPLACEMENT_SHIFT_Y), 0.0);
            // The property follows the group.
            let property: Ptr<WaterShaderProperty> = Ptr::new(property);
            assert_eq!(w.e.get(property, WaterShaderProperty::iStencilMask), 0x55);
            assert_eq!(
                w.e.get(property, WaterShaderProperty::spDepthMap),
                0x0e00_0003
            );
            assert_eq!(
                w.e.get(property, WaterShaderProperty::spNoiseNormalMap),
                0x0e00_0002
            );
            assert_eq!(
                w.e.get(property, WaterShaderProperty::spDisplacementNormalMap),
                0x0e00_0110
            );
            // The displacement pass ran and left its flag clear.
            assert_eq!(calls(&w.e, IMAGE_SPACE_RENDER_DISPLACEMENT).len(), 1);
            assert_eq!(w.e.global::<u8>(DISPLACEMENT_PASS_FLAG), 0);
        }

        #[test]
        fn a_player_who_stays_near_leaves_the_geometry_where_it_is() {
            let (mut w, _, _) = simulation(5.0);
            simulate(&mut w);
            assert!(w.translations.borrow().is_empty());
            assert!(calls(&w.e, NODE_UPDATE).is_empty());
            assert_eq!(w.e.global::<f32>(DISPLACEMENT_SHIFT_X), 0.0);
            assert_eq!(w.e.global::<f32>(DISPLACEMENT_SHIFT_Y), 0.0);
        }

        #[test]
        fn the_displacement_constants_and_the_wading_offsets_are_published() {
            let (mut w, _, _) = simulation(5.0);
            let points = record_points(&mut w.e);
            // One wading entry: the data's two points go into the two arrays.
            let data = w.e.mem.alloc(0x1c);
            for (i, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
                w.e.mem.set_f32(data + 4 * i as u32, value);
            }
            w.e.register(ZONE_MAP_FIRST, |_, _| ret(1));
            w.e.register_double(ZONE_MAP_GET_NEXT, move |e, a| {
                e.mem.set_u32(a[1], 0);
                e.mem.set_u32(a[3], data);
                Ret::default()
            });
            // A count of one in the first array advances the timer.
            w.e.set_global(DISPLACEMENT_POINTS_A + 8, 1u32);
            simulate(&mut w);
            assert_eq!(
                *points.borrow(),
                vec![
                    (DISPLACEMENT_POINTS_A, 1.0, 2.0),
                    (DISPLACEMENT_POINTS_B, 3.0, 4.0)
                ]
            );
            assert_eq!(
                floats_at(&w.e, DISPLACEMENT_CONSTANTS, 4),
                vec![0.4, 0.6, 0.97, 0.01]
            );
            assert_eq!(
                w.e.get(w.system, TESWaterSystem::fTimeSinceLastRipplePlaced),
                1.5
            );
            let render = calls(&w.e, IMAGE_SPACE_RENDER_DISPLACEMENT);
            assert_eq!(render[0][1], 0x20);
            assert_ne!(render[0][2], 0, "the renderer");
            assert_eq!(render[0][3..], [0x0e00_0100, 0x0e00_0100, 0, 1]);
            // Without entries the count stays zero and the timer restarts.
            w.e.register(ZONE_MAP_FIRST, |_, _| ret(0));
            w.e.set_global(DISPLACEMENT_POINTS_A + 8, 0u32);
            simulate(&mut w);
            assert_eq!(
                w.e.get(w.system, TESWaterSystem::fTimeSinceLastRipplePlaced),
                0.0
            );
        }

        #[test]
        fn the_simulation_is_skipped_without_the_renderer_setting_two() {
            let (mut w, _, _) = simulation(5.0);
            w.e.register(RENDER_SETTING_ENTRY, |_, _| ret(1));
            simulate(&mut w);
            assert!(calls(&w.e, IMAGE_SPACE_RENDER_DISPLACEMENT).is_empty());
            assert_eq!(w.e.global::<u8>(DISPLACEMENT_PASS_FLAG), 0);
            // The height map was stopped into render target mode 6.
            assert_eq!(
                calls(&w.e, RENDER_TARGET_SET),
                vec![vec![6, 0x0e00_0100 + 1]]
            );
        }

        #[test]
        fn the_render_target_is_reset_outside_a_frame_and_the_geometry_rendered_on_request() {
            let (mut w, _, _) = simulation(5.0);
            w.e.set_global(WADING_WATER_HEIGHT_MAP, 0u32);
            let render_object = w.e.global::<u32>(RENDER_OBJECT_GLOBAL);
            w.e.mem.set_u32(render_object + 0x200, 1);
            w.e.set_global(DISPLACEMENT_RENDER_FLAG, 1u8);
            // The geometry render needs only the doubles of the wading camera;
            // it is looked at in its own tests.
            quiet(&mut w.e, &[ATTACH_CHILD_DOUBLE_ADDRESS]);
            simulate(&mut w);
            // Once before and once after the optional render.
            assert_eq!(calls(&w.e, RENDER_TARGET_RESET).len(), 2);
            // The missing height map was created.
            assert_eq!(w.e.global::<u32>(WADING_WATER_HEIGHT_MAP), GREY_TEXTURE);
            assert_eq!(w.e.global::<u8>(WADING_MAP_CREATED_FLAG), 1);
            assert_eq!(calls(&w.e, CAMERA_CONSTRUCT).len(), 1);
        }

        #[test]
        fn after_ten_units_of_time_the_geometry_is_detached_and_the_map_returned() {
            let (mut w, geometry, _) = simulation(5.0);
            w.e.set(w.system, TESWaterSystem::fTimeSinceLastRipplePlaced, 10.0);
            w.e.set_global(DISPLACEMENT_ACTIVE_FLAG, 1u8);
            // A second group without geometry is left alone.
            let other = group(&mut w.e, 3.0, 0);
            let groups = [w.group.addr(), other];
            fill_list(&mut w.e, w.system.addr() + 0x3c, &groups);
            simulate(&mut w);
            assert_eq!(
                calls(&w.e, DETACH_CHILD_DOUBLE_ADDRESS),
                vec![vec![w.root, geometry]]
            );
            assert_eq!(
                w.e.get(w.group, PlaceableWaterGroup::spWadingWaterGeometry),
                0
            );
            assert_eq!(
                calls(&w.e, RETURN_RENDERED_TEXTURE),
                vec![vec![0x0aaa_0000, 0x0e00_0100]]
            );
            assert_eq!(w.e.global::<u32>(WADING_WATER_HEIGHT_MAP), 0);
            assert_eq!(w.e.global::<u8>(DISPLACEMENT_ACTIVE_FLAG), 0);
            // Nothing of the simulation ran.
            assert!(calls(&w.e, RENDER_TARGET_SET).is_empty());
        }

        // --- 004e9510, 004e9530 -------------------------------------------------

        #[test]
        fn the_renderer_is_inside_a_frame_when_its_word_at_0x200_is_nonzero() {
            let mut e = engine();
            let render_object = e.global::<u32>(RENDER_OBJECT_GLOBAL);
            assert!(e.call(0x004e_9510, &[]).bool());
            e.mem.set_u32(render_object + 0x200, 5);
            assert!(!e.call(0x004e_9510, &[]).bool());
            assert_eq!(e.call(0x004e_9530, &args![render_object]).u32(), 5);
        }

        // --- 004e9550 -----------------------------------------------------------

        /// What the wading camera doubles saw.
        #[derive(Default)]
        struct CameraLog {
            frustum: Vec<f32>,
            ortho: u8,
            look_at: Vec<f32>,
            drawn: Vec<(u32, u32, u32)>,
        }

        /// A scene for the wading camera: two groups (planes 10 and 4), the
        /// first with a water reference in a cell of kind 6 that is in range.
        fn camera_scene() -> (Wading, Rc<RefCell<CameraLog>>) {
            let mut w = wading(10.0);
            let second = group(&mut w.e, 4.0, 0);
            let first = w.group.addr();
            fill_list(&mut w.e, w.system.addr() + 0x3c, &[first, second]);
            let cell = w.e.mem.alloc(0x40);
            w.e.mem.set_u8(cell + 0x26, 6);
            w.e.mem.set_u32(w.water_reference + REF_CELL, cell);
            let top = w.e.mem.u32(w.water_reference + REF_NODE);
            w.e.mem.set_u8(top + NODE_RANGE, 1);
            let log = Rc::new(RefCell::new(CameraLog::default()));
            let frustum_log = log.clone();
            w.e.register_double(CAMERA_SET_VIEW_FRUSTUM, move |e, a| {
                let mut log = frustum_log.borrow_mut();
                log.frustum = (0..6).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
                log.ortho = e.mem.u8(a[1] + 0x18);
                Ret::default()
            });
            let look_at_log = log.clone();
            w.e.register_double(CAMERA_LOOK_AT_WORLD_POINT, move |e, a| {
                look_at_log.borrow_mut().look_at =
                    (0..3).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
                assert_eq!(a[2], UP_VECTOR);
                Ret::default()
            });
            let drawn_log = log.clone();
            let property = w.other_property;
            w.e.register_double(RENDER_PASS_IMMEDIATELY, move |e, a| {
                drawn_log
                    .borrow_mut()
                    .drawn
                    .push((a[0], a[1], e.mem.u32(property + 0x84)));
                Ret::default()
            });
            w.e.register(PROPERTY_RENDER_PASS, |e, _| {
                let pass = e.mem.alloc(0x20);
                e.mem.set_u16(pass + 4, 3);
                ret(pass)
            });
            w.e.set_global(BIT_FLAGS_011F941C, 0xffffu16);
            (w, log)
        }

        fn render_wading(w: &mut Wading) {
            start_log(&mut w.e);
            w.e.call(0x004e_9550, &args![w.system]);
        }

        #[test]
        fn the_wading_camera_looks_down_from_above_the_highest_group() {
            let (mut w, log) = camera_scene();
            render_wading(&mut w);
            let camera = calls(&w.e, CAMERA_CONSTRUCT)[0][0];
            // Above the highest plane (10 + 50) at the player's x and y.
            assert_eq!(*w.translations.borrow(), vec![(camera, [10.0, 20.0, 60.0])]);
            assert_eq!(log.borrow().look_at, vec![10.0, 20.0, 10.0]);
            // Orthographic, half the quad size (100) each way, 0.1 to
            // |4 - 10| + 100 away.
            assert_eq!(
                log.borrow().frustum,
                vec![-50.0, 50.0, 50.0, -50.0, 0.1, 106.0]
            );
            assert_eq!(log.borrow().ortho, 1);
            assert_eq!(calls(&w.e, FRUSTUM_CONSTRUCT)[0][1], 0);
            // The camera is made the render object's, the culling process works
            // for it and the camera and process are destroyed again.
            let object = w.e.global::<u32>(RENDER_OBJECT_GLOBAL);
            assert_eq!(
                calls(&w.e, RENDER_OBJECT_SET_CAMERA_DATA),
                vec![vec![object, camera + 0x100]]
            );
            let culling = calls(&w.e, CULLING_PROCESS_CONSTRUCT)[0][0];
            assert_eq!(
                calls(&w.e, CULLING_PROCESS_SET_CAMERA),
                vec![vec![culling, camera], vec![culling, 0]]
            );
            assert_eq!(
                calls(&w.e, CULLING_PROCESS_SET_PLANES),
                vec![vec![culling, camera + 0xdc]]
            );
            assert_eq!(calls(&w.e, CULLING_PROCESS_DESTRUCT), vec![vec![culling]]);
            assert_eq!(calls(&w.e, CAMERA_DESTRUCT), vec![vec![camera]]);
            // The camera data went to the render object's virtual.
            assert_eq!(calls(&w.e, RENDER_OBJECT_SET_CAMERA_DATA_DOUBLE).len(), 1);
            assert_eq!(w.e.global::<u8>(WADING_RENDER_ACTIVE_FLAG), 0);
            assert_eq!(w.e.global::<u16>(BIT_FLAGS_011F941C), 0xfffe);
        }

        #[test]
        fn a_water_reference_in_range_is_drawn_at_once_with_stencil_mask_one() {
            let (mut w, log) = camera_scene();
            render_wading(&mut w);
            let drawn = log.borrow().drawn.clone();
            assert_eq!(drawn.len(), 1);
            assert_eq!(drawn[0].1, 3, "the pass count");
            assert_eq!(drawn[0].2, 1, "the stencil mask while drawing");
            assert_eq!(w.e.mem.u32(w.other_property + 0x84), 7, "restored");
            let request = calls(&w.e, RENDER_PASS_IMMEDIATELY);
            assert_eq!(request[0][2..], [0, 0, 0]);
        }

        #[test]
        fn a_reference_out_of_range_is_not_drawn_but_its_mask_is_restored() {
            let (mut w, log) = camera_scene();
            let top = w.e.mem.u32(w.water_reference + REF_NODE);
            w.e.mem.set_u8(top + NODE_RANGE, 0);
            render_wading(&mut w);
            assert!(log.borrow().drawn.is_empty());
            assert!(calls(&w.e, PROPERTY_RENDER_PASS).is_empty());
            assert_eq!(w.e.mem.u32(w.other_property + 0x84), 7);
        }

        #[test]
        fn references_in_other_cells_or_with_other_owners_are_left_alone() {
            let (mut w, log) = camera_scene();
            // A cell of another kind.
            let cell = w.e.mem.u32(w.water_reference + REF_CELL);
            w.e.mem.set_u8(cell + 0x26, 5);
            render_wading(&mut w);
            assert!(calls(&w.e, NODE_GET_PROPERTY).is_empty());
            // No cell at all.
            w.e.mem.set_u32(w.water_reference + REF_CELL, 0);
            render_wading(&mut w);
            assert!(calls(&w.e, CELL_BYTE_IS_SIX).is_empty());
            // An owner of another type.
            w.e.mem.set_u32(w.water_reference + REF_CELL, cell);
            w.e.mem.set_u8(cell + 0x26, 6);
            let owner = w.e.mem.u32(w.geometry + GEOMETRY_OWNER);
            w.e.mem.set_u32(owner + 0x68, WATER_OWNER_TYPE + 1);
            render_wading(&mut w);
            assert!(calls(&w.e, NODE_GET_PROPERTY).is_empty());
            assert!(log.borrow().drawn.is_empty());
        }

        // --- 004e9bb0, 004e9c10, 004e9c50, 004e9c90, 004e9ce0 --------------------

        #[test]
        fn the_rotation_columns_are_copied_through_a_local_point() {
            let mut e = engine();
            let out = e.mem.alloc(0x20);
            start_log(&mut e);
            assert_eq!(e.call(0x004e_9c10, &args![0x4000u32, out]).u32(), out);
            assert_eq!(floats_at(&e, out, 3), vec![20.0, 21.0, 22.0]);
            assert_eq!(
                e.call(0x004e_9c50, &args![0x4000u32, out + 0x10]).u32(),
                out + 0x10
            );
            assert_eq!(floats_at(&e, out + 0x10, 3), vec![40.0, 41.0, 42.0]);
            let columns = calls(&e, MATRIX_COLUMN);
            assert_eq!(columns[0][..2], [0x4000 + 0x68, 1]);
            assert_eq!(columns[1][..2], [0x4000 + 0x68, 2]);
            assert_eq!(calls(&e, ADDRESS_OF_THIS).len(), 2);
        }

        #[test]
        fn the_camera_data_goes_to_the_render_object_with_the_camera_columns() {
            let mut e = engine();
            let camera = e.mem.alloc(0x120);
            let object = e.global::<u32>(RENDER_OBJECT_GLOBAL);
            let seen: Rc<RefCell<Vec<Vec<f32>>>> = Rc::default();
            let words: Rc<RefCell<Vec<u32>>> = Rc::default();
            {
                let seen = seen.clone();
                let words = words.clone();
                e.register_double(RENDER_OBJECT_SET_CAMERA_DATA_DOUBLE, move |e, a| {
                    *words.borrow_mut() = a.to_vec();
                    // The three columns, passed by address.
                    *seen.borrow_mut() = (2..5)
                        .map(|i| (0..3).map(|j| e.mem.f32(a[i] + 4 * j)).collect())
                        .collect();
                    Ret::default()
                });
            }
            e.call(0x004e_9bb0, &args![object, camera]);
            let words = words.borrow().clone();
            assert_eq!(words[0], object);
            assert_eq!(words[1], camera + 0x8c, "the world translate");
            assert_eq!(words[5], camera + 0xdc);
            assert_eq!(words[6], camera + 0x100);
            assert_eq!(
                *seen.borrow(),
                vec![
                    vec![10.0, 11.0, 12.0],
                    vec![20.0, 21.0, 22.0],
                    vec![40.0, 41.0, 42.0]
                ]
            );
        }

        #[test]
        fn camera_data_is_only_passed_on_when_the_render_object_asks_for_it() {
            let mut e = engine();
            let object = e.global::<u32>(RENDER_OBJECT_GLOBAL);
            start_log(&mut e);
            e.call(
                0x004e_9c90,
                &args![object, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32],
            );
            assert_eq!(
                calls(&e, RENDER_OBJECT_CHECK),
                vec![vec![object, SET_CAMERA_DATA_NAME, 1]]
            );
            assert_eq!(
                calls(&e, RENDER_OBJECT_SET_CAMERA_DATA_DOUBLE),
                vec![vec![object, 1, 2, 3, 4, 5, 6]]
            );
            e.register(RENDER_OBJECT_CHECK, |_, _| ret(0));
            start_log(&mut e);
            e.call(
                0x004e_9c90,
                &args![object, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32],
            );
            assert!(calls(&e, RENDER_OBJECT_SET_CAMERA_DATA_DOUBLE).is_empty());
        }

        #[test]
        fn a_bit_of_the_flag_word_is_set_or_cleared_by_index_modulo_sixteen() {
            let mut e = engine();
            e.set_global(BIT_FLAGS_011F941C, 0u16);
            e.call(0x004e_9ce0, &args![3u32, 1u32]);
            assert_eq!(e.global::<u16>(BIT_FLAGS_011F941C), 0x0008);
            e.call(0x004e_9ce0, &args![16u32 + 5, 1u32]);
            assert_eq!(e.global::<u16>(BIT_FLAGS_011F941C), 0x0028);
            e.call(0x004e_9ce0, &args![3u32, 0u32]);
            assert_eq!(e.global::<u16>(BIT_FLAGS_011F941C), 0x0020);
            e.call(0x004e_9ce0, &args![15u32, 7u32]);
            assert_eq!(e.global::<u16>(BIT_FLAGS_011F941C), 0x8020);
            e.call(0x004e_9ce0, &args![0u32, 0u32]);
            assert_eq!(
                e.global::<u16>(BIT_FLAGS_011F941C),
                0x8020,
                "clearing a clear bit"
            );
        }

        // --- the small accessors ------------------------------------------------

        #[test]
        fn accumulator_setters_store_their_arguments() {
            let mut e = engine();
            let accumulator = e.mem.alloc(0x280);
            e.call(0x004e_a860, &args![accumulator, 1u32]);
            assert_eq!(e.mem.u8(accumulator + 0x164), 1);
            e.call(0x004e_a860, &args![accumulator, 0x1ffu32]);
            assert_eq!(e.mem.u8(accumulator + 0x164), 0xff);
            let color = e.mem.alloc(0x10);
            for (i, value) in [0.1f32, 0.2, 0.3, 0.4].into_iter().enumerate() {
                e.mem.set_f32(color + 4 * i as u32, value);
            }
            e.call(0x004e_a880, &args![accumulator, color]);
            assert_eq!(
                floats_at(&e, accumulator + 0x154, 4),
                vec![0.1, 0.2, 0.3, 0.4]
            );
        }

        fn check_flag_accessor(address: u32, mask: u32) {
            let mut e = engine();
            let form = e.mem.alloc(0x40);
            e.mem.set_u32(form + BASE_FORM_FLAGS, mask);
            start_log(&mut e);
            assert!(e.call(address, &args![form]).bool());
            assert_eq!(calls(&e, FORM_FLAG_TEST), vec![vec![form, mask]]);
            e.mem.set_u32(form + BASE_FORM_FLAGS, !mask);
            assert!(!e.call(address, &args![form]).bool());
        }

        #[test]
        fn form_flag_accessors_test_one_bit_each() {
            check_flag_accessor(0x004e_a8b0, 0x04);
            check_flag_accessor(0x004e_a8d0, 0x08);
            check_flag_accessor(0x004e_a8f0, 0x10);
            check_flag_accessor(0x004e_a910, 0x20);
            check_flag_accessor(0x004e_a930, 0x40);
        }

        #[test]
        fn address_accessors_return_members_and_statics() {
            let mut e = engine();
            // The sky colour address, the rendering system's static and the
            // pointer members.
            assert_eq!(e.call(0x004e_a950, &args![0x1000u32]).u32(), 0x1060);
            assert_eq!(e.call(0x004e_a970, &[]).u32(), MT_RENDERING_SYSTEM);
            let holder = e.mem.alloc(0x140);
            e.mem.set_u32(holder + 0x134, 0x6677);
            assert_eq!(e.call(0x004e_a980, &args![holder]).u32(), 0x6677);
            e.set_global(TERRAIN_POINTER_A, 0xaaaau32);
            e.set_global(TERRAIN_POINTER_B, 0xbbbbu32);
            e.set_global(TERRAIN_POINTER_C, 0xccccu32);
            assert_eq!(e.call(0x004e_a9a0, &args![0x1u32]).u32(), 0xaaaa);
            assert_eq!(e.call(0x004e_a9c0, &args![0x1u32]).u32(), 0xbbbb);
            assert_eq!(e.call(0x004e_a9e0, &args![0x1u32]).u32(), 0xcccc);
            let player = e.mem.alloc(0x700);
            e.mem.set_u8(player + 0x64a, 3);
            assert_eq!(e.call(0x004e_af60, &args![player]).u8(), 3);
        }

        #[test]
        fn the_counter_is_lowered_and_the_render_state_called_with_zeros() {
            let mut e = engine();
            quiet(&mut e, &[RENDER_STATE_SET]);
            e.set_global(COUNTER_011FFA14, 10u32);
            start_log(&mut e);
            e.call(0x004e_b510, &args![3u32]);
            assert_eq!(e.global::<u32>(COUNTER_011FFA14), 7);
            assert_eq!(calls(&e, RENDER_STATE_SET), vec![vec![0; 7]]);
        }

        // --- 004e9d40, 004eaa00, 004eaf80 ---------------------------------------

        const ACCUMULATOR_VTABLE: u32 = 0x0200_c000;
        const ACCUMULATOR_CAMERA_DOUBLE: u32 = 0x0300_0205;
        /// The `NiPointer` the sky holds at +4 (`SKY_OBJECT_POINTER`).
        const SKY_OBJECTS: u32 = 0x5151;
        /// The statics `fn_004ea9a0`, `fn_004ea9c0` and `fn_004ea9e0` read.
        const TERRAIN_A: u32 = 0xaaa1;
        const TERRAIN_B: u32 = 0xbbb2;
        const TERRAIN_C: u32 = 0xccc3;
        /// The 3D objects `NODE_FIRST_CHILD`-less doubles give the grid cells:
        /// child `i` of the cell at address `a` is `a * 16 + i`.
        const GRID_CELL_A: u32 = 0x40;
        const GRID_CELL_B: u32 = 0x80;

        struct Reflection {
            w: Wading,
            viewer: u32,
            base_form: u32,
            /// The render table entry (`RENDER_TABLE_ENTRY` returns it).
            table_entry: u32,
            /// The calls `REFLECT_CAMERA_ABOUT_PLANE` and the virtual at +0x8c
            /// of the accumulators saw.
            accumulator_cameras: Rc<RefCell<Vec<(u32, u32)>>>,
        }

        /// The wading scene (one group at height 10, one water reference in a
        /// cell of kind 6, with a water shader property) plus the doubles of
        /// the reflection setups: a sky, a world space with a terrain manager,
        /// a 2 by 2 cell grid and a viewer.
        fn reflection() -> Reflection {
            let mut w = wading(10.0);
            let e = &mut w.e;
            e.register(CAMERA_CONSTRUCT, |_, a| ret(a[0]));
            e.register(ACCUMULATOR_CONSTRUCT, |e, a| {
                e.mem.set_u32(a[0], ACCUMULATOR_VTABLE);
                ret(a[0])
            });
            let accumulator_cameras: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
            {
                let log = accumulator_cameras.clone();
                e.register_double(ACCUMULATOR_CAMERA_DOUBLE, move |_, a| {
                    log.borrow_mut().push((a[0], a[1]));
                    Ret::default()
                });
            }
            vtable(
                e,
                ACCUMULATOR_VTABLE,
                &[(ACCUMULATOR_SET_CAMERA_VIRTUAL, ACCUMULATOR_CAMERA_DOUBLE)],
            );
            quiet(
                e,
                &[
                    REFLECT_CAMERA_ABOUT_PLANE,
                    ACCUMULATOR_SET_WORD_194,
                    ACCUMULATOR_SET_WORD_19C,
                    ACCUMULATOR_SET_ACCUMULATE,
                    MT_ADD_ACCUM_TASK,
                    MT_SET_THREAD_STAGE,
                    MT_SET_THREAD_STAGE_ONE,
                    TERRAIN_STEP_FIRST,
                    TERRAIN_STEP_SECOND,
                    TERRAIN_TOGGLE_ONE,
                    TERRAIN_TOGGLE_ZERO,
                ],
            );
            e.register(LIST_REMOVE_ALL, |e, a| {
                for i in 0..3 {
                    e.mem.set_u32(a[0] + 4 * i, 0);
                }
                Ret::default()
            });
            e.register(TES_GET_SKY, |e, a| ret(e.mem.u32(a[0] + 0x68)));
            e.register(SKY_OBJECT_POINTER, |e, a| ret(e.mem.u32(a[0] + 4)));
            e.register(TERRAIN_READY, |_, _| ret(1));
            e.register(COLOR_FACTOR_ADDRESS, |_, a| ret(a[0] + 0x20));
            e.register(WORLD_SPACE_GET_TERRAIN_MANAGER, |e, a| {
                ret(e.mem.u32(a[0] + 0x3c))
            });
            e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
            e.register(REFERENCE_IS_EXCLUDED, |e, a| {
                ret(e.mem.u8(a[0] + 0xc8) as u32)
            });
            // The render table entry leads to the three factors at +0x20.
            let holder = e.mem.alloc(0x200);
            let factors = e.mem.alloc(0x40);
            e.mem.set_u32(holder + 0x134, factors);
            for (i, value) in [2.0f32, 4.0, 0.5].into_iter().enumerate() {
                e.mem.set_f32(factors + 0x20 + 4 * i as u32, value);
            }
            e.register_double(RENDER_TABLE_ENTRY, move |_, _| ret(holder));
            e.register(TES_GRID_CELL_SLOT, |e, a| {
                // Cells (0, 0) and (1, 1) exist.
                let cells = e.global::<u32>(0x011d_8800);
                if a[1] == a[2] {
                    ret(cells + 4 * a[1])
                } else {
                    ret(cells + 8)
                }
            });
            e.register(CELL_NODE_CHILD, |_, a| ret(a[0] * 16 + a[1]));
            e.map(0x011c_a000, 0x1000);
            let cells = e.mem.alloc(0x10);
            e.mem.set_u32(cells, GRID_CELL_A);
            e.mem.set_u32(cells + 4, GRID_CELL_B);
            e.set_global(0x011d_8800u32, cells);
            set_int_setting(e, SETTING_GRID_SIZE, 2);
            e.set_global(TERRAIN_POINTER_A, TERRAIN_A);
            e.set_global(TERRAIN_POINTER_B, TERRAIN_B);
            e.set_global(TERRAIN_POINTER_C, TERRAIN_C);
            // The sky, with its colour and the objects it holds.
            let sky = e.mem.alloc(0x100);
            e.mem.set_u32(sky + 4, SKY_OBJECTS);
            for (i, value) in [0.5f32, 0.25, 1.0].into_iter().enumerate() {
                e.mem.set_f32(sky + 0x60 + 4 * i as u32, value);
            }
            let tes = e.global::<u32>(TES_POINTER);
            e.mem.set_u32(tes + 0x68, sky);
            // The world space and its terrain manager.
            let world_space = e.mem.alloc(0x100);
            let terrain = e.mem.alloc(0x10);
            e.mem.set_u32(world_space + 0x3c, terrain);
            e.mem.set_u32(tes + 0x50, world_space);
            // The water reference: in a cell of kind 6, in range.
            let cell = e.mem.alloc(0x40);
            e.mem.set_u8(cell + 0x26, 6);
            e.mem.set_u32(w.water_reference + REF_CELL, cell);
            let base_form = e.mem.u32(w.water_reference + REF_BASE_FORM);
            let viewer = e.mem.alloc(0x100);
            let top = e.mem.u32(w.water_reference + REF_NODE);
            e.mem.set_u8(top + NODE_RANGE, 1);
            Reflection {
                w,
                viewer,
                base_form,
                table_entry: holder,
                accumulator_cameras,
            }
        }

        fn set_int_setting(e: &mut Engine, setting: u32, value: u32) {
            e.set_global(setting + 4, value);
        }

        impl Reflection {
            fn set_flags(&mut self, flags: u32) {
                self.w
                    .e
                    .mem
                    .set_u32(self.base_form + BASE_FORM_FLAGS, flags);
            }

            fn run(&mut self, low_detail: bool) {
                start_log(&mut self.w.e);
                self.w.e.call(
                    0x004e_9d40,
                    &args![self.w.system, self.viewer, self.w.group, low_detail],
                );
            }

            fn static_objects(&self) -> Vec<u32> {
                list_items(&self.w.e, self.w.group.addr() + 0x64)
            }

            fn dynamic_objects(&self) -> Vec<u32> {
                list_items(&self.w.e, self.w.group.addr() + 0x70)
            }
        }

        #[test]
        fn the_group_reflection_makes_a_camera_and_an_accumulator_and_hands_them_over() {
            let mut r = reflection();
            r.set_flags(0x40);
            r.w.e
                .set(r.w.group, PlaceableWaterGroup::iReflectionThreadStage, 4);
            r.run(false);
            let group = r.w.group;
            let camera = r.w.e.get(group, PlaceableWaterGroup::spReflectionCamera);
            let accumulator =
                r.w.e
                    .get(group, PlaceableWaterGroup::spGroupReflectionSorter);
            assert_ne!(camera, 0);
            assert_ne!(accumulator, 0);
            // The viewer is reflected about the group's plane into the new camera.
            assert_eq!(
                calls(&r.w.e, REFLECT_CAMERA_ABOUT_PLANE),
                vec![vec![r.viewer, group.addr() + 4, camera]]
            );
            // The accumulator was prepared: the render table word, the byte at
            // +0x164, the camera and `bAccumulate`.
            assert_eq!(
                calls(&r.w.e, ACCUMULATOR_SET_WORD_194),
                vec![vec![accumulator, r.table_entry]]
            );
            assert_eq!(r.w.e.mem.u8(accumulator + 0x164), 1);
            assert_eq!(*r.accumulator_cameras.borrow(), vec![(accumulator, camera)]);
            assert_eq!(
                calls(&r.w.e, ACCUMULATOR_SET_ACCUMULATE),
                vec![vec![accumulator, 1]]
            );
            // The task: camera, the two lists, the accumulator, 4 and stage 5.
            let renderer = MT_RENDERING_SYSTEM;
            assert_eq!(
                calls(&r.w.e, MT_ADD_ACCUM_TASK),
                vec![vec![
                    renderer,
                    camera,
                    0,
                    0,
                    group.addr() + 0x64,
                    group.addr() + 0x70,
                    accumulator,
                    4,
                    5,
                    0
                ]]
            );
            assert_eq!(
                calls(&r.w.e, MT_SET_THREAD_STAGE),
                vec![vec![renderer, 0, 5]]
            );
            // The scope of `TESWater.cpp` line 0xefb is opened and closed.
            let scope = calls(&r.w.e, ALLOCATION_SCOPE_CONSTRUCT);
            assert_eq!(&scope[0][1..], &[0x1d, 1, TESWATER_SOURCE_PATH, 0xefb]);
            assert_eq!(
                calls(&r.w.e, ALLOCATION_SCOPE_DESTRUCT),
                vec![vec![scope[0][0]]]
            );
        }

        #[test]
        fn the_reference_properties_take_the_group_reflection_map_and_the_sky_is_listed_once() {
            let mut r = reflection();
            r.set_flags(0x40);
            // A second reference with the same base form.
            let (second, _, geometry, _) = geometry_chain(&mut r.w.e);
            let property = r.w.e.mem.alloc(0x150);
            r.w.e.mem.set_u32(geometry + GEOMETRY_PROPERTY, property);
            r.w.e.mem.set_u32(second + REF_BASE_FORM, r.base_form);
            let cell = r.w.e.mem.u32(r.w.water_reference + REF_CELL);
            r.w.e.mem.set_u32(second + REF_CELL, cell);
            let first = r.w.water_reference;
            fill_list(&mut r.w.e, r.w.group.addr() + 0x24, &[first, second]);
            r.w.e.mem.set_u32(r.w.group.addr() + 0x54, 0x0e00_0099);
            r.run(false);
            // Both properties are pointed at the group's reflection map.
            for property in [r.w.other_property, property] {
                assert_eq!(
                    r.w.e.get(
                        Ptr::<WaterShaderProperty>::new(property),
                        WaterShaderProperty::spReflectionMap
                    ),
                    0x0e00_0099,
                    "the group's own reflection map"
                );
            }
            // The sky's objects only once, whatever the number of references.
            assert_eq!(r.static_objects(), vec![SKY_OBJECTS]);
        }

        #[test]
        fn a_group_with_silhouette_reflections_gets_the_sky_colour() {
            let mut r = reflection();
            r.set_flags(0);
            r.w.e.set(
                r.w.group,
                PlaceableWaterGroup::bRenderSilhouetteReflections,
                true,
            );
            r.run(false);
            let accumulator =
                r.w.e
                    .get(r.w.group, PlaceableWaterGroup::spGroupReflectionSorter);
            assert_eq!(
                calls(&r.w.e, ACCUMULATOR_SET_WORD_19C),
                vec![vec![accumulator, 0xf]]
            );
            assert_eq!(
                floats_at(&r.w.e, accumulator + 0x154, 4),
                vec![1.0, 1.0, 0.5, 1.0]
            );
            // A group without them keeps the colour.
            let mut r = reflection();
            r.run(false);
            assert!(calls(&r.w.e, ACCUMULATOR_SET_WORD_19C).is_empty());
        }

        #[test]
        fn the_land_children_of_the_loaded_grid_are_added_unless_the_detail_is_low() {
            let mut r = reflection();
            r.set_flags(0x04);
            r.run(false);
            // Cells (0, 0) and (1, 1) exist; each gives its four children, the
            // newest first in the list.
            let mut expected = vec![];
            for cell in [GRID_CELL_A, GRID_CELL_B] {
                for i in 0..4 {
                    expected.push(cell * 16 + i);
                }
            }
            expected.reverse();
            assert_eq!(r.static_objects(), expected);
            // Low detail leaves them out.
            let mut r = reflection();
            r.set_flags(0x04);
            r.run(true);
            assert!(r.static_objects().is_empty());
            // So does an interior.
            let mut r = reflection();
            r.set_flags(0x04);
            let tes = r.w.e.global::<u32>(TES_POINTER);
            r.w.e.mem.set_u32(tes + 0x34, 0x77);
            r.run(false);
            assert!(r.static_objects().is_empty());
            // The setting adds them whatever the form flags say.
            let mut r = reflection();
            r.set_flags(0);
            set_setting(&mut r.w.e, SETTING_FORCE_HIGH_DETAIL_LAND_REFLECTIONS, true);
            r.run(false);
            assert_eq!(r.static_objects().len(), 8);
        }

        #[test]
        fn the_terrain_objects_are_added_once_each_by_their_flags() {
            let mut r = reflection();
            r.set_flags(0x08 | 0x10 | 0x20);
            // A second reference with the same flags adds nothing more.
            let (second, _, geometry, _) = geometry_chain(&mut r.w.e);
            let property = r.w.e.mem.alloc(0x150);
            r.w.e.mem.set_u32(geometry + GEOMETRY_PROPERTY, property);
            r.w.e.mem.set_u32(second + REF_BASE_FORM, r.base_form);
            let cell = r.w.e.mem.u32(r.w.water_reference + REF_CELL);
            r.w.e.mem.set_u32(second + REF_CELL, cell);
            let first = r.w.water_reference;
            fill_list(&mut r.w.e, r.w.group.addr() + 0x24, &[first, second]);
            r.run(false);
            assert_eq!(r.static_objects(), vec![TERRAIN_C, TERRAIN_B, TERRAIN_A]);
            // The terrain helpers run around the first two.
            assert_eq!(calls(&r.w.e, TERRAIN_STEP_FIRST).len(), 2);
            assert_eq!(calls(&r.w.e, TERRAIN_STEP_SECOND).len(), 2);
            // Without a ready terrain nothing is added.
            let mut r = reflection();
            r.set_flags(0x08 | 0x10 | 0x20);
            r.w.e.register(TERRAIN_READY, |_, _| ret(0));
            r.run(false);
            assert!(r.static_objects().is_empty());
        }

        #[test]
        fn a_reference_outside_the_exterior_cell_counts_only_with_the_08000000_flag() {
            let mut r = reflection();
            r.set_flags(0x40);
            let cell = r.w.e.mem.u32(r.w.water_reference + REF_CELL);
            r.w.e.mem.set_u8(cell + 0x26, 5);
            r.run(false);
            assert!(r.static_objects().is_empty(), "wrong cell kind, no flag");
            assert!(calls(&r.w.e, NODE_GET_PROPERTY).is_empty());
            r.set_flags(0x40 | 0x0800_0000);
            r.run(false);
            assert_eq!(r.static_objects(), vec![SKY_OBJECTS]);
            // In an interior the flag does not help.
            let tes = r.w.e.global::<u32>(TES_POINTER);
            r.w.e.mem.set_u32(tes + 0x34, 0x77);
            let group = r.w.group;
            r.w.e.mem.set_u32(group.addr() + 0x64, 0);
            r.w.e.mem.set_u32(group.addr() + 0x68, 0);
            r.w.e.mem.set_u32(group.addr() + 0x6c, 0);
            r.run(false);
            assert!(r.static_objects().is_empty());
            // A reference whose geometry has no water shader property is skipped.
            let mut r = reflection();
            r.set_flags(0x40);
            r.w.e.mem.set_u32(r.w.geometry + GEOMETRY_PROPERTY, 0);
            r.run(false);
            assert!(r.static_objects().is_empty());
            assert!(calls(&r.w.e, NI_POINTER_ASSIGN_FROM).is_empty());
        }

        /// A reflected-references list node `{entry, next}` whose entry is
        /// `{reference, flags}`.
        fn reflected_node(e: &mut Engine, reference: u32, flags: u32, next: u32) -> u32 {
            let entry = e.mem.alloc(8);
            e.mem.set_u32(entry, reference);
            e.mem.set_u32(entry + 4, flags);
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, entry);
            e.mem.set_u32(node + 4, next);
            node
        }

        /// A reference with a 3D object.
        fn reflected_reference(e: &mut Engine) -> u32 {
            let reflected = reference(e);
            let top = node(e);
            e.mem.set_u32(reflected + REF_NODE, top);
            reflected
        }

        #[test]
        fn references_the_reference_reflects_are_listed_and_remembered() {
            let mut r = reflection();
            r.set_flags(0);
            let listed = reflected_reference(&mut r.w.e);
            let ignored = reflected_reference(&mut r.w.e);
            let known = reflected_reference(&mut r.w.e);
            let excluded = reflected_reference(&mut r.w.e);
            r.w.e.mem.set_u8(excluded + 0xc8, 1);
            r.w.map.borrow_mut().insert(known, known);
            let last = reflected_node(&mut r.w.e, excluded, 1, 0);
            let third = reflected_node(&mut r.w.e, known, 1, last);
            let second = reflected_node(&mut r.w.e, ignored, 0, third);
            let first = reflected_node(&mut r.w.e, listed, 1, second);
            r.w.e.mem.set_u32(r.w.water_reference + 0x44, first);
            r.run(false);
            // Only the reflected reference the map did not know, that is not
            // excluded and whose entry has the flag, is listed, with the map.
            let listed_node = r.w.e.mem.u32(listed + REF_NODE);
            assert_eq!(r.static_objects(), vec![listed_node]);
            assert_eq!(r.w.map.borrow().get(&listed), Some(&listed));
            assert!(!r.w.map.borrow().contains_key(&ignored));
            assert!(!r.w.map.borrow().contains_key(&excluded));
            // In an interior the excluded reference is listed too.
            let mut r = reflection();
            r.set_flags(0);
            let excluded = reflected_reference(&mut r.w.e);
            r.w.e.mem.set_u8(excluded + 0xc8, 1);
            let only = reflected_node(&mut r.w.e, excluded, 1, 0);
            r.w.e.mem.set_u32(r.w.water_reference + 0x44, only);
            let tes = r.w.e.global::<u32>(TES_POINTER);
            r.w.e.mem.set_u32(tes + 0x34, 0x77);
            r.run(false);
            assert_eq!(r.static_objects().len(), 1);
            // Low detail does not look at them at all.
            let mut r = reflection();
            r.set_flags(0);
            let listed = reflected_reference(&mut r.w.e);
            let only = reflected_node(&mut r.w.e, listed, 1, 0);
            r.w.e.mem.set_u32(r.w.water_reference + 0x44, only);
            r.run(true);
            assert!(r.static_objects().is_empty());
        }

        #[test]
        fn the_static_water_object_list_is_walked_for_the_40000000_flag() {
            let mut r = reflection();
            r.set_flags(0x4000_0000);
            // The flag moves the geometry one level deeper.
            let deeper = r.w.e.mem.u32(r.w.geometry + CHILD);
            let property = r.w.e.mem.alloc(0x150);
            r.w.e.mem.set_u32(deeper + GEOMETRY_PROPERTY, property);
            let listed = reflected_reference(&mut r.w.e);
            let excluded = reflected_reference(&mut r.w.e);
            r.w.e.mem.set_u8(excluded + 0xc8, 1);
            // The static object at 0x011ca13c: nodes `{reference, next}`.
            let second = r.w.e.mem.alloc(8);
            r.w.e.mem.set_u32(second, excluded);
            r.w.e.mem.set_u32(WATER_OBJECT_011CA13C, listed);
            r.w.e.mem.set_u32(WATER_OBJECT_011CA13C + 4, second);
            r.run(false);
            let node_of_listed = r.w.e.mem.u32(listed + REF_NODE);
            assert_eq!(r.static_objects(), vec![node_of_listed]);
            assert!(r.w.map.borrow().contains_key(&listed));
            assert!(!r.w.map.borrow().contains_key(&excluded));
        }

        #[test]
        fn only_the_first_explosion_is_reflected_when_the_setting_is_on() {
            let mut r = reflection();
            r.set_flags(0);
            let explosion = reflected_reference(&mut r.w.e);
            let later = reflected_reference(&mut r.w.e);
            fill_list(&mut r.w.e, EXPLOSIONS_LIST, &[explosion, later]);
            r.run(false);
            assert!(r.dynamic_objects().is_empty(), "the setting is off");
            set_setting(&mut r.w.e, SETTING_REFLECT_EXPLOSIONS, true);
            r.run(false);
            let node_of_explosion = r.w.e.mem.u32(explosion + REF_NODE);
            assert_eq!(r.dynamic_objects(), vec![node_of_explosion]);
            assert!(r.w.map.borrow().contains_key(&explosion));
            assert!(!r.w.map.borrow().contains_key(&later));
            // A known explosion is not listed again.
            r.run(false);
            assert!(r.dynamic_objects().is_empty());
        }

        #[test]
        fn nothing_is_set_up_without_the_water_shader_or_a_viewer() {
            let mut r = reflection();
            r.set_flags(0x40);
            r.w.e.register(WATER_SHADER_ENABLED, |_, _| ret(0));
            r.run(false);
            assert!(calls(&r.w.e, NI_ALLOC).is_empty());
            // The scope is still opened and closed.
            assert_eq!(calls(&r.w.e, ALLOCATION_SCOPE_CONSTRUCT).len(), 1);
            assert_eq!(calls(&r.w.e, ALLOCATION_SCOPE_DESTRUCT).len(), 1);
            let mut r = reflection();
            r.viewer = 0;
            r.run(false);
            assert!(calls(&r.w.e, NI_ALLOC).is_empty());
            assert!(calls(&r.w.e, MT_ADD_ACCUM_TASK).is_empty());
        }

        /// The world and sky reflection setups: the `Reflection` scene with
        /// the world space's water height at 8.0 and a recorder for the
        /// reflection plane.
        struct WorldReflection {
            r: Reflection,
            /// The plane `REFLECT_CAMERA_ABOUT_PLANE` was given (floats).
            plane: Rc<RefCell<Vec<f32>>>,
        }

        fn world_reflection() -> WorldReflection {
            let mut r = reflection();
            let tes = r.w.e.global::<u32>(TES_POINTER);
            let world_space = r.w.e.mem.u32(tes + 0x50);
            r.w.e.mem.set_f32(world_space + 0x7c, 8.0);
            r.w.e.register(WORLD_SPACE_WATER_HEIGHT, |e, a| {
                e.mem.f32(a[0] + 0x7c).into_ret()
            });
            let plane: Rc<RefCell<Vec<f32>>> = Rc::default();
            let log = plane.clone();
            r.w.e
                .register_double(REFLECT_CAMERA_ABOUT_PLANE, move |e, a| {
                    *log.borrow_mut() = (0..4).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
                    Ret::default()
                });
            r.w.e.set_global(WORLD_REFLECTION_THREAD_STAGE, 3u32);
            r.w.e.set_global(SKY_REFLECTION_THREAD_STAGE, 6u32);
            WorldReflection { r, plane }
        }

        impl WorldReflection {
            fn world(&mut self) {
                start_log(&mut self.r.w.e);
                let system = self.r.w.system;
                let viewer = self.r.viewer;
                self.r.w.e.call(0x004e_aa00, &args![system, viewer]);
            }

            fn sky(&mut self) {
                start_log(&mut self.r.w.e);
                let system = self.r.w.system;
                let viewer = self.r.viewer;
                self.r.w.e.call(0x004e_af80, &args![system, viewer]);
            }

            fn items(&self, list: u32) -> Vec<u32> {
                list_items(&self.r.w.e, list)
            }
        }

        #[test]
        fn the_world_reflection_reflects_the_viewer_about_the_water_plane_and_lists_the_sky_and_terrain(
        ) {
            let mut w = world_reflection();
            w.world();
            let e = &w.r.w.e;
            let camera = e.global::<u32>(WORLD_REFLECTION_CAMERA);
            let accumulator = e.global::<u32>(WORLD_REFLECTION_SORTER);
            assert_ne!(camera, 0);
            assert_ne!(accumulator, 0);
            // The plane through (0, 0, 8) with the normal (0, 0, 1).
            assert_eq!(*w.plane.borrow(), vec![0.0, 0.0, 1.0, 8.0]);
            assert_eq!(calls(e, REFLECT_CAMERA_ABOUT_PLANE)[0][0], w.r.viewer);
            assert_eq!(
                *w.r.accumulator_cameras.borrow(),
                vec![(accumulator, camera)]
            );
            // The sky first, then the three terrain objects (newest first).
            assert_eq!(
                w.items(STATIC_WORLD_REFLECTIVE_OBJECTS),
                vec![TERRAIN_C, TERRAIN_B, TERRAIN_A, SKY_OBJECTS]
            );
            assert!(w.items(DYNAMIC_WORLD_REFLECTIVE_OBJECTS).is_empty());
            // The task for stage 3 + 1.
            assert_eq!(
                calls(e, MT_ADD_ACCUM_TASK),
                vec![vec![
                    MT_RENDERING_SYSTEM,
                    camera,
                    0,
                    0,
                    STATIC_WORLD_REFLECTIVE_OBJECTS,
                    DYNAMIC_WORLD_REFLECTIVE_OBJECTS,
                    accumulator,
                    4,
                    4,
                    0
                ]]
            );
            assert_eq!(
                calls(e, MT_SET_THREAD_STAGE),
                vec![vec![MT_RENDERING_SYSTEM, 0, 4]]
            );
            let scope = calls(e, ALLOCATION_SCOPE_CONSTRUCT);
            assert_eq!(&scope[0][1..], &[0x1d, 1, TESWATER_SOURCE_PATH, 0xfe3]);
            assert_eq!(calls(e, ALLOCATION_SCOPE_DESTRUCT).len(), 1);
            // Without the silhouette settings the colour is left alone.
            assert!(calls(e, ACCUMULATOR_SET_WORD_19C).is_empty());
        }

        #[test]
        fn the_world_reflection_in_an_interior_lists_only_the_sky() {
            let mut w = world_reflection();
            let tes = w.r.w.e.global::<u32>(TES_POINTER);
            w.r.w.e.mem.set_u32(tes + 0x34, 0x77);
            w.world();
            assert_eq!(w.items(STATIC_WORLD_REFLECTIVE_OBJECTS), vec![SKY_OBJECTS]);
        }

        #[test]
        fn the_world_reflection_has_a_silhouette_colour_with_either_setting() {
            for setting in [
                SETTING_AUTO_SILHOUETTE_REFLECTIONS,
                SETTING_FORCE_LOW_DETAIL_REFLECTIONS,
            ] {
                let mut w = world_reflection();
                set_setting(&mut w.r.w.e, setting, true);
                w.world();
                let accumulator = w.r.w.e.global::<u32>(WORLD_REFLECTION_SORTER);
                assert_eq!(
                    calls(&w.r.w.e, ACCUMULATOR_SET_WORD_19C),
                    vec![vec![accumulator, 0xf]]
                );
            }
        }

        #[test]
        fn forced_high_detail_culls_the_third_person_player_and_lists_one_object() {
            let mut w = world_reflection();
            set_setting(&mut w.r.w.e, SETTING_FORCE_HIGH_DETAIL_REFLECTIONS, true);
            let player = object(&mut w.r.w.e, ACTOR_VTABLE, 0x700);
            let player_node = node(&mut w.r.w.e);
            w.r.w.e.mem.set_u32(player + REF_NODE, player_node);
            w.r.w.e.set_global(PLAYER_CHARACTER, player);
            w.r.w
                .e
                .register(POINTER_STATIC_011DEB7C, |_, _| ret(0x6161));
            w.r.w.e.register(NODE_SET_CULLED, |e, a| {
                e.mem.set_u32(a[0] + 0x30, a[1]);
                Ret::default()
            });
            w.world();
            assert!(w.r.w.e.get(w.r.w.system, TESWaterSystem::bCull3rdPerson));
            assert_eq!(w.r.w.e.mem.u32(player_node + 0x30), 1);
            assert_eq!(w.items(STATIC_WORLD_REFLECTIVE_OBJECTS), vec![0x6161]);
            // A player in first person is not culled.
            let mut w = world_reflection();
            set_setting(&mut w.r.w.e, SETTING_FORCE_HIGH_DETAIL_REFLECTIONS, true);
            let player = object(&mut w.r.w.e, ACTOR_VTABLE, 0x700);
            w.r.w.e.mem.set_u8(player + 0x64a, 1);
            w.r.w.e.set_global(PLAYER_CHARACTER, player);
            w.r.w
                .e
                .register(POINTER_STATIC_011DEB7C, |_, _| ret(0x6161));
            w.world();
            assert!(!w.r.w.e.get(w.r.w.system, TESWaterSystem::bCull3rdPerson));
            assert_eq!(w.items(STATIC_WORLD_REFLECTIVE_OBJECTS), vec![0x6161]);
        }

        #[test]
        fn the_world_reflection_needs_the_water_shader_and_a_viewer() {
            let mut w = world_reflection();
            w.r.w.e.register(WATER_SHADER_ENABLED, |_, _| ret(0));
            w.world();
            assert!(calls(&w.r.w.e, NI_ALLOC).is_empty());
            assert_eq!(calls(&w.r.w.e, ALLOCATION_SCOPE_DESTRUCT).len(), 1);
            let mut w = world_reflection();
            w.r.viewer = 0;
            w.world();
            assert!(calls(&w.r.w.e, MT_ADD_ACCUM_TASK).is_empty());
        }

        #[test]
        fn the_sky_reflection_uses_its_own_camera_accumulator_and_lists() {
            let mut w = world_reflection();
            set_setting(&mut w.r.w.e, SETTING_AUTO_SILHOUETTE_REFLECTIONS, true);
            w.sky();
            let e = &w.r.w.e;
            let camera = e.global::<u32>(SKY_REFLECTION_CAMERA);
            let accumulator = e.global::<u32>(SKY_REFLECTION_SORTER);
            assert_ne!(camera, 0);
            assert_ne!(accumulator, 0);
            assert_eq!(e.global::<u32>(WORLD_REFLECTION_CAMERA), 0);
            assert_eq!(*w.plane.borrow(), vec![0.0, 0.0, 1.0, 8.0]);
            assert_eq!(
                *w.r.accumulator_cameras.borrow(),
                vec![(accumulator, camera)]
            );
            assert_eq!(w.items(STATIC_SKY_REFLECTIVE_OBJECTS), vec![SKY_OBJECTS]);
            assert!(w.items(DYNAMIC_SKY_REFLECTIVE_OBJECTS).is_empty());
            assert_eq!(
                calls(e, MT_ADD_ACCUM_TASK),
                vec![vec![
                    MT_RENDERING_SYSTEM,
                    camera,
                    0,
                    0,
                    STATIC_SKY_REFLECTIVE_OBJECTS,
                    DYNAMIC_SKY_REFLECTIVE_OBJECTS,
                    accumulator,
                    4,
                    7,
                    0
                ]]
            );
            assert_eq!(
                calls(e, MT_SET_THREAD_STAGE),
                vec![vec![MT_RENDERING_SYSTEM, 0, 7]]
            );
            let scope = calls(e, ALLOCATION_SCOPE_CONSTRUCT);
            assert_eq!(&scope[0][1..], &[0x1d, 1, TESWATER_SOURCE_PATH, 0x1054]);
            // The sky setup never sets a silhouette colour.
            assert!(calls(e, ACCUMULATOR_SET_WORD_19C).is_empty());
        }

        #[test]
        fn the_sky_reflection_needs_the_water_shader_and_a_viewer() {
            let mut w = world_reflection();
            w.r.w.e.register(WATER_SHADER_ENABLED, |_, _| ret(0));
            w.sky();
            assert!(calls(&w.r.w.e, NI_ALLOC).is_empty());
            let mut w = world_reflection();
            w.r.viewer = 0;
            w.sky();
            assert!(calls(&w.r.w.e, MT_ADD_ACCUM_TASK).is_empty());
        }

        // --- 004eb220 -----------------------------------------------------------

        /// What `fn_004eb220` told the renderer: the clear colours set.
        struct Finish {
            r: Reflection,
            clear_colors: Rc<RefCell<Vec<Vec<f32>>>>,
            plane_setup: Rc<RefCell<Vec<Vec<u32>>>>,
            camera: u32,
        }

        fn finish() -> Finish {
            let mut r = reflection();
            let clear_colors: Rc<RefCell<Vec<Vec<f32>>>> = Rc::default();
            let log = clear_colors.clone();
            r.w.e
                .register_double(RENDERER_SET_COLOR_DOUBLE, move |e, a| {
                    log.borrow_mut()
                        .push((0..4).map(|i| e.mem.f32(a[1] + 4 * i)).collect());
                    Ret::default()
                });
            let plane_setup: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
            let log = plane_setup.clone();
            r.w.e.register_double(REFLECTION_PLANE_SETUP, move |_, a| {
                log.borrow_mut().push(a.to_vec());
                Ret::default()
            });
            quiet(
                &mut r.w.e,
                &[
                    RENDER_STATE_SET,
                    RENDER_ACCUMULATED_SCENE,
                    RENDER_OBJECT_SET_CAMERA_DATA,
                ],
            );
            r.w.e
                .register(CREATE_RENDERED_TEXTURE, |_, _| ret(0x0e00_0777));
            let camera = r.w.e.mem.alloc(0x120);
            let group = r.w.group;
            r.w.e
                .set(group, PlaceableWaterGroup::spReflectionCamera, camera);
            r.w.e
                .set(group, PlaceableWaterGroup::spGroupReflectionSorter, 0x4242);
            r.w.e
                .set(group, PlaceableWaterGroup::iReflectionThreadStage, 9);
            r.w.e.set_global(COUNTER_011FFA14, 5u32);
            Finish {
                r,
                clear_colors,
                plane_setup,
                camera,
            }
        }

        impl Finish {
            fn run(&mut self) {
                start_log(&mut self.r.w.e);
                let system = self.r.w.system;
                let group = self.r.w.group;
                self.r.w.e.call(0x004e_b220, &args![system, group]);
            }
        }

        #[test]
        fn finishing_an_interior_group_renders_its_reflection_texture() {
            let mut f = finish();
            f.run();
            let e = &f.r.w.e;
            let group = f.r.w.group;
            // The clear colour alpha is 0 while the group is rendered, then restored.
            assert_eq!(
                *f.clear_colors.borrow(),
                vec![vec![0.1, 0.2, 0.3, 0.0], vec![0.1, 0.2, 0.3, 0.4]]
            );
            // The reflection texture is created with 9 and stopped into mode 7.
            let renderer = calls(e, RENDERER).len();
            assert!(renderer > 0);
            assert_eq!(
                calls(e, CREATE_RENDERED_TEXTURE)[0][0],
                0x0aaa_0000,
                "the texture manager"
            );
            assert_eq!(calls(e, CREATE_RENDERED_TEXTURE)[0][2..], [9, 0, 0, 0]);
            assert_eq!(
                e.get(group, PlaceableWaterGroup::spGroupReflectionMap),
                0x0e00_0777
            );
            assert_eq!(calls(e, RENDER_TARGET_SET), vec![vec![7, 0x0e00_0778]]);
            // The camera data, the thread stage and the plane.
            assert_eq!(
                calls(e, RENDER_OBJECT_SET_CAMERA_DATA),
                vec![vec![
                    e.global::<u32>(RENDER_OBJECT_GLOBAL),
                    f.camera + 0x100
                ]]
            );
            assert_eq!(
                calls(e, MT_SET_THREAD_STAGE_ONE),
                vec![vec![MT_RENDERING_SYSTEM, 1, 10]]
            );
            // The plane (0, 0, 1) and the negated constant 10, then 0.
            let setup = f.plane_setup.borrow().clone();
            assert_eq!(setup.len(), 1);
            assert_eq!(setup[0][0], f.r.w.system.addr());
            assert_eq!(
                setup[0][1..],
                [
                    0.0f32.to_bits(),
                    0.0f32.to_bits(),
                    1.0f32.to_bits(),
                    (-10.0f32).to_bits(),
                    0
                ]
            );
            // The accumulated scene is drawn with the group's camera and sorter,
            // the render target reset and the camera dropped.
            assert_eq!(
                calls(e, RENDER_ACCUMULATED_SCENE),
                vec![vec![f.camera, 0x4242, 0]]
            );
            assert_eq!(
                calls(e, RENDER_STATE_SET),
                vec![vec![1, 0, 0, 0, 0, 0, 0], vec![0; 7]]
            );
            assert_eq!(e.global::<u32>(COUNTER_011FFA14), 5);
            assert_eq!(calls(e, RENDER_TARGET_RESET).len(), 1);
            assert_eq!(e.get(group, PlaceableWaterGroup::spReflectionCamera), 0);
            // Terrain helper calls around it.
            assert_eq!(calls(e, TERRAIN_TOGGLE_ONE).len(), 1);
            assert_eq!(calls(e, TERRAIN_TOGGLE_ZERO).len(), 1);
            let scope = calls(e, ALLOCATION_SCOPE_CONSTRUCT);
            assert_eq!(&scope[0][1..], &[0x1d, 1, TESWATER_SOURCE_PATH, 0x1082]);
            assert_eq!(calls(e, ALLOCATION_SCOPE_DESTRUCT).len(), 1);
        }

        #[test]
        fn an_existing_reflection_texture_is_kept_and_no_terrain_toggles_in_an_interior() {
            let mut f = finish();
            let group = f.r.w.group;
            f.r.w.e.set(
                group,
                PlaceableWaterGroup::spGroupReflectionMap,
                0x0e00_0123,
            );
            let tes = f.r.w.e.global::<u32>(TES_POINTER);
            f.r.w.e.mem.set_u32(tes + 0x34, 0x77);
            f.run();
            assert!(calls(&f.r.w.e, CREATE_RENDERED_TEXTURE).is_empty());
            assert_eq!(
                calls(&f.r.w.e, RENDER_TARGET_SET),
                vec![vec![7, 0x0e00_0124]]
            );
            assert!(calls(&f.r.w.e, TERRAIN_TOGGLE_ONE).is_empty());
            assert!(calls(&f.r.w.e, TERRAIN_TOGGLE_ZERO).is_empty());
        }

        #[test]
        fn a_group_without_a_reflection_camera_or_without_the_water_shader_is_left_alone() {
            let mut f = finish();
            let group = f.r.w.group;
            f.r.w
                .e
                .set(group, PlaceableWaterGroup::spReflectionCamera, 0);
            f.run();
            assert!(f.clear_colors.borrow().is_empty());
            assert!(calls(&f.r.w.e, RENDER_TARGET_SET).is_empty());
            let mut f = finish();
            f.r.w.e.register(WATER_SHADER_ENABLED, |_, _| ret(0));
            f.run();
            assert!(f.clear_colors.borrow().is_empty());
            assert_eq!(calls(&f.r.w.e, ALLOCATION_SCOPE_DESTRUCT).len(), 1);
        }

        // --- the fourth session: 004eb540 to 004eda60 ------------------------

        mod fourth_session {
            use super::*;

            const DEVICE_VTABLE: u32 = 0x0200_d000;
            const STATE_DEVICE_VTABLE: u32 = 0x0200_e000;
            const CLIP_PLANE_DOUBLE: u32 = 0x0300_0301;
            const STATE_DOUBLE: u32 = 0x0300_0302;
            const IDENTITY: [f32; 16] = [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ];

            type ClipPlanes = Rc<RefCell<Vec<(u32, u32, Vec<f32>)>>>;
            type Products = Rc<RefCell<Vec<([f32; 16], [f32; 16])>>>;
            type States = Rc<RefCell<Vec<Vec<u32>>>>;

            /// Puts the translation back at `address`, over a double the
            /// shared fixtures register there.
            fn use_real_function(e: &mut Engine, address: u32) {
                for (candidate, function) in funcs() {
                    if candidate == address {
                        e.register(candidate, function);
                    }
                }
            }

            fn matrix_at(e: &Engine, address: u32) -> [f32; 16] {
                std::array::from_fn(|i| e.mem.f32(address + 4 * i as u32))
            }

            fn put_matrix(e: &mut Engine, address: u32, matrix: &[f32; 16]) {
                for (i, value) in matrix.iter().enumerate() {
                    e.mem.set_f32(address + 4 * i as u32, *value);
                }
            }

            /// The inverse of a 4x4 matrix (Gauss-Jordan in `f64`).
            #[allow(clippy::needless_range_loop)]
            fn invert(m: &[f32; 16]) -> [f32; 16] {
                let mut a = [[0.0f64; 8]; 4];
                for r in 0..4 {
                    for c in 0..4 {
                        a[r][c] = m[r * 4 + c] as f64;
                    }
                    a[r][4 + r] = 1.0;
                }
                for col in 0..4 {
                    let pivot = (col..4)
                        .max_by(|&x, &y| a[x][col].abs().partial_cmp(&a[y][col].abs()).unwrap())
                        .unwrap();
                    a.swap(col, pivot);
                    let divisor = a[col][col];
                    for c in 0..8 {
                        a[col][c] /= divisor;
                    }
                    for r in 0..4 {
                        if r != col {
                            let factor = a[r][col];
                            for c in 0..8 {
                                a[r][c] -= factor * a[col][c];
                            }
                        }
                    }
                }
                std::array::from_fn(|i| a[i / 4][4 + i % 4] as f32)
            }

            /// What the plane setup talks to.
            struct PlaneMath {
                /// The clip planes set on the render object's device: device,
                /// index, plane.
                clip_planes: ClipPlanes,
                /// The matrices `D3DXMatrixMultiply` was given.
                products: Products,
                /// The calls to the renderer device's virtual at +0x68.
                states: States,
                renderer: u32,
                clip_device: u32,
                state_device: u32,
            }

            /// Doubles for the plane setup: a renderer with identity view and
            /// projection matrices (the rotation is the identity, the eye is
            /// at (1, 2, 3)), the D3DX math with the real formulas, `memcpy`,
            /// the dot product, and the devices the clip planes and the
            /// states go to.
            fn install_plane_math(e: &mut Engine) -> PlaneMath {
                let renderer = object(e, RENDERER_VTABLE, 0xa40);
                put_matrix(e, renderer + RENDERER_VIEW_MATRIX_OFFSET, &IDENTITY);
                put_matrix(e, renderer + RENDERER_PROJECTION_MATRIX_OFFSET, &IDENTITY);
                e.register_double(RENDERER, move |_, _| ret(renderer));
                for (i, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
                    e.set_global(EYE_POSITION + 4 * i as u32, value);
                }
                e.register(MEMORY_COPY, |e, a| {
                    for i in 0..a[2] {
                        let byte = e.mem.u8(a[1] + i);
                        e.mem.set_u8(a[0] + i, byte);
                    }
                    ret(a[0])
                });
                e.register(NI_POINT3_DOT, |e, a| {
                    (0..3)
                        .map(|i| e.mem.f32(a[0] + 4 * i) * e.mem.f32(a[1] + 4 * i))
                        .sum::<f32>()
                        .into_ret()
                });
                let products: Products = Rc::default();
                let log = products.clone();
                e.register_double(D3DX_MATRIX_MULTIPLY, move |e, a| {
                    let (x, y) = (matrix_at(e, a[1]), matrix_at(e, a[2]));
                    log.borrow_mut().push((x, y));
                    let mut out = [0.0f32; 16];
                    for r in 0..4 {
                        for c in 0..4 {
                            out[r * 4 + c] = (0..4).map(|k| x[r * 4 + k] * y[k * 4 + c]).sum();
                        }
                    }
                    put_matrix(e, a[0], &out);
                    ret(a[0])
                });
                e.register(D3DX_MATRIX_INVERSE, |e, a| {
                    assert_eq!(a[1], 0, "no determinant is asked for");
                    let inverse = invert(&matrix_at(e, a[2]));
                    put_matrix(e, a[0], &inverse);
                    ret(a[0])
                });
                e.register(D3DX_MATRIX_TRANSPOSE, |e, a| {
                    let m = matrix_at(e, a[1]);
                    let transposed: [f32; 16] = std::array::from_fn(|i| m[(i % 4) * 4 + i / 4]);
                    put_matrix(e, a[0], &transposed);
                    ret(a[0])
                });
                e.register(D3DX_PLANE_NORMALIZE, |e, a| {
                    let p: Vec<f32> = (0..4).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
                    let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
                    for i in 0..4 {
                        e.mem.set_f32(a[0] + 4 * i, p[i as usize] / length);
                    }
                    ret(a[0])
                });
                e.register(D3DX_PLANE_TRANSFORM, |e, a| {
                    let p: Vec<f32> = (0..4).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
                    let m = matrix_at(e, a[2]);
                    for i in 0..4 {
                        let value: f32 = (0..4).map(|j| p[j] * m[j * 4 + i as usize]).sum();
                        e.mem.set_f32(a[0] + 4 * i, value);
                    }
                    ret(a[0])
                });
                // The render object's device: its clip plane virtual.
                let render_object = e.global::<u32>(RENDER_OBJECT_GLOBAL);
                let clip_device = object(e, DEVICE_VTABLE, 0x40);
                vtable(
                    e,
                    DEVICE_VTABLE,
                    &[(DEVICE_SET_CLIP_PLANE_VIRTUAL, CLIP_PLANE_DOUBLE)],
                );
                e.mem
                    .set_u32(render_object + RENDER_OBJECT_DEVICE_OFFSET, clip_device);
                let clip_planes: ClipPlanes = Rc::default();
                let log = clip_planes.clone();
                e.register_double(CLIP_PLANE_DOUBLE, move |e, a| {
                    let plane = (0..4).map(|i| e.mem.f32(a[3] + 4 * i)).collect();
                    log.borrow_mut().push((a[1], a[2], plane));
                    Ret::default()
                });
                // The renderer's device: its state virtual.
                let state_device = object(e, STATE_DEVICE_VTABLE, 0x40);
                vtable(
                    e,
                    STATE_DEVICE_VTABLE,
                    &[(DEVICE_SET_STATE_VIRTUAL, STATE_DOUBLE)],
                );
                e.mem
                    .set_u32(renderer + RENDERER_DEVICE_OFFSET, state_device);
                let states: States = Rc::default();
                let log = states.clone();
                e.register_double(STATE_DOUBLE, move |_, a| {
                    log.borrow_mut().push(a.to_vec());
                    Ret::default()
                });
                PlaneMath {
                    clip_planes,
                    products,
                    states,
                    renderer,
                    clip_device,
                    state_device,
                }
            }

            // --- the small accessors --------------------------------------

            #[test]
            fn the_address_accessors_return_their_constants() {
                let mut e = water_engine();
                assert_eq!(e.call(0x004e_c7b0, &[]).u32(), 0x011c_a144);
                assert_eq!(e.call(0x004e_d180, &[]).u32(), 0x011f_474c);
                assert_eq!(e.call(0x004e_d1f0, &args![0x1000u32]).u32(), 0x1980);
                assert_eq!(e.call(0x004e_d210, &args![0x1000u32]).u32(), 0x19c0);
                let renderer = e.mem.alloc(0x900);
                e.mem.set_u32(renderer + 0x8b8, 0xd00d);
                assert_eq!(e.call(0x004e_caf0, &args![renderer]).u32(), 0xd00d);
            }

            #[test]
            fn the_accumulator_byte_and_the_form_flag() {
                let mut e = water_engine();
                let accumulator = e.mem.alloc(0x200);
                e.mem.set_u8(accumulator + 0x164, 9);
                e.call(0x004e_c7c0, &args![accumulator, 1u8]);
                assert_eq!(e.mem.u8(accumulator + 0x165), 1);
                assert_eq!(
                    e.mem.u8(accumulator + 0x164),
                    9,
                    "the next byte is left alone"
                );
                let form = e.mem.alloc(0x40);
                assert!(!e.call(0x004e_c7e0, &args![form]).bool());
                e.mem.set_u32(form + BASE_FORM_FLAGS, 0x400);
                assert!(e.call(0x004e_c7e0, &args![form]).bool());
                e.mem.set_u32(form + BASE_FORM_FLAGS, 0x3ff);
                assert!(!e.call(0x004e_c7e0, &args![form]).bool());
            }

            #[test]
            fn bit_two_of_the_byte_at_eb_is_tested() {
                let mut e = water_engine();
                let object = e.mem.alloc(0x100);
                assert!(!e.call(0x004e_d270, &args![object]).bool());
                e.mem.set_u8(object + 0xeb, 0xfb);
                assert!(!e.call(0x004e_d270, &args![object]).bool());
                e.mem.set_u8(object + 0xeb, 0x04);
                assert!(e.call(0x004e_d270, &args![object]).bool());
            }

            #[test]
            fn the_byte_interpolation_scales_the_byte_between_the_two_floats() {
                let mut e = water_engine();
                e.set_global(COLOR_STEP_SCALE, 0.5f64);
                let object = e.mem.alloc(0x200);
                e.mem.set_u8(object + 0x10 + 0xe0, 4);
                // 4 * 0.5 * (3 - 1) + 1
                let value = e
                    .call(0x004e_d230, &args![object, 0x10u32, 3.0f32, 1.0f32])
                    .f32();
                assert_eq!(value, 5.0);
                e.mem.set_u8(object + 0x10 + 0xe0, 0);
                let value = e
                    .call(0x004e_d230, &args![object, 0x10u32, 3.0f32, 1.0f32])
                    .f32();
                assert_eq!(value, 1.0);
            }

            #[test]
            fn the_render_state_counters_stop_at_zero() {
                let mut e = water_engine();
                e.set_global(RENDER_STATE_COUNTERS + 4 * 10, 3u32);
                e.set_global(RENDER_STATE_COUNTERS + 4 * 11, 0u32);
                e.call(0x004e_cb10, &args![10u32]);
                e.call(0x004e_cb10, &args![11u32]);
                assert_eq!(e.global::<u32>(RENDER_STATE_COUNTERS + 4 * 10), 2);
                assert_eq!(e.global::<u32>(RENDER_STATE_COUNTERS + 4 * 11), 0);
            }

            #[test]
            fn the_counter_helpers_lower_a_word_and_make_the_counted_state_call() {
                let mut e = water_engine();
                quiet(&mut e, &[COUNTED_STATE_34, COUNTED_STATE_A8]);
                e.set_global(COUNTER_011FF9FC, 5u32);
                e.set_global(COUNTER_011FFA18, 8u32);
                start_log(&mut e);
                e.call(0x004e_cb40, &args![2u32]);
                e.call(0x004e_ced0, &args![3u32]);
                assert_eq!(e.global::<u32>(COUNTER_011FF9FC), 3);
                assert_eq!(e.global::<u32>(COUNTER_011FFA18), 5);
                assert_eq!(calls(&e, COUNTED_STATE_34), vec![vec![0, 0]]);
                assert_eq!(calls(&e, COUNTED_STATE_A8), vec![vec![7, 0]]);
            }

            #[test]
            fn a_float_setting_is_stored_through_its_setter() {
                let mut e = water_engine();
                e.register(SETTING_STORE_FLOAT, |e, a| {
                    e.mem.set_u32(a[0] + 4, a[1]);
                    ret(a[0])
                });
                let setting = e.mem.alloc(0x10);
                assert_eq!(e.call(0x004e_d780, &args![setting, 2.5f32]).u32(), setting);
                assert_eq!(e.mem.f32(setting + 4), 2.5);
            }

            // --- the matrices and the plane setup ---------------------------

            #[test]
            fn a_matrix_is_copied_with_memcpy() {
                let mut e = water_engine();
                e.register(MEMORY_COPY, |_, a| ret(a[0]));
                start_log(&mut e);
                assert_eq!(e.call(0x004e_d110, &args![0x100u32, 0x200u32]).u32(), 0x100);
                assert_eq!(calls(&e, MEMORY_COPY), vec![vec![0x100, 0x200, 0x40]]);
            }

            #[test]
            fn matrices_are_multiplied_in_a_temporary_and_copied_out() {
                let mut e = engine();
                let m = install_plane_math(&mut e);
                let (x, y, out) = (e.mem.alloc(0x40), e.mem.alloc(0x40), e.mem.alloc(0x40));
                let first: [f32; 16] = std::array::from_fn(|i| i as f32);
                put_matrix(&mut e, x, &first);
                put_matrix(&mut e, y, &IDENTITY);
                assert_eq!(e.call(0x004e_d140, &args![x, out, y]).u32(), out);
                assert_eq!(*m.products.borrow(), vec![(first, IDENTITY)]);
                assert_eq!(matrix_at(&e, out), first);
            }

            #[test]
            fn clip_planes_are_set_from_the_first_index_for_the_count() {
                let mut e = engine();
                let m = install_plane_math(&mut e);
                let object = e.global::<u32>(RENDER_OBJECT_GLOBAL);
                let planes = e.mem.alloc(0x30);
                for i in 0..12 {
                    e.mem.set_f32(planes + 4 * i, i as f32);
                }
                e.call(0x004e_d190, &args![object, 5i32, 3i32, planes]);
                assert_eq!(
                    *m.clip_planes.borrow(),
                    vec![
                        (m.clip_device, 5, vec![0.0, 1.0, 2.0, 3.0]),
                        (m.clip_device, 6, vec![4.0, 5.0, 6.0, 7.0]),
                        (m.clip_device, 7, vec![8.0, 9.0, 10.0, 11.0]),
                    ]
                );
                e.call(0x004e_d190, &args![object, 0i32, 0i32, planes]);
                e.call(0x004e_d190, &args![object, 0i32, -1i32, planes]);
                assert_eq!(m.clip_planes.borrow().len(), 3, "nothing for a count <= 0");
            }

            #[test]
            fn the_clip_plane_is_moved_into_view_space_and_set_on_the_device() {
                let mut e = engine();
                let m = install_plane_math(&mut e);
                start_log(&mut e);
                // The plane z = 5 (normal (0, 0, 1), minus the constant).
                e.call(
                    0x004e_cef0,
                    &args![0u32, 0.0f32, 0.0f32, 1.0f32, -5.0f32, 0u32],
                );
                // With the eye at (1, 2, 3) it is z = 2 in front of the eye.
                assert_eq!(
                    *m.clip_planes.borrow(),
                    vec![(m.clip_device, 0, vec![0.0, 0.0, 1.0, -2.0])]
                );
                // The view matrix is the renderer's rotation with the
                // translation row minus the eye's dot product with each axis.
                let products = m.products.borrow();
                assert_eq!(products.len(), 1);
                assert_eq!(
                    products[0].0,
                    [
                        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -1.0, -2.0,
                        -3.0, 1.0
                    ]
                );
                assert_eq!(products[0].1, IDENTITY);
            }

            #[test]
            fn the_view_translation_row_follows_the_rotation_columns() {
                let mut e = engine();
                let m = install_plane_math(&mut e);
                // Rows (0, 1, 0), (-1, 0, 0), (0, 0, 1): the columns are (0, -1,
                // 0), (1, 0, 0) and (0, 0, 1).
                let view = [
                    0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 9.0, 9.0, 9.0, 1.0,
                ];
                put_matrix(&mut e, m.renderer + RENDERER_VIEW_MATRIX_OFFSET, &view);
                e.call(
                    0x004e_cef0,
                    &args![0u32, 0.0f32, 0.0f32, 1.0f32, -5.0f32, 0u32],
                );
                let products = m.products.borrow();
                assert_eq!(
                    products[0].0,
                    [
                        0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 2.0, -1.0,
                        -3.0, 1.0
                    ]
                );
            }

            #[test]
            fn the_plane_is_normalized_before_it_is_moved() {
                let mut e = engine();
                let m = install_plane_math(&mut e);
                e.call(
                    0x004e_cef0,
                    &args![0u32, 0.0f32, 0.0f32, 2.0f32, -10.0f32, 0u32],
                );
                assert_eq!(
                    m.clip_planes.borrow()[0].2,
                    vec![0.0, 0.0, 1.0, -2.0],
                    "(0, 0, 2, -10) is (0, 0, 1, -5)"
                );
            }

            // --- the group constructor, destructor and the grey texture ---------

            #[test]
            fn a_group_is_constructed_with_its_members_and_three_flags_cleared() {
                let mut e = water_engine();
                quiet(
                    &mut e,
                    &[
                        NI_PLANE_DEFAULT_CONSTRUCT,
                        LIST_CONSTRUCT,
                        POINTER_TEMP_CONSTRUCT,
                    ],
                );
                let group = e.mem.alloc(0xb0);
                for offset in [0x5c, 0x5d, 0x5e, 0x5f, 0x60] {
                    e.mem.set_u8(group + offset, 1);
                }
                for offset in [0x58, 0x94, 0x98] {
                    e.mem.set_u32(group + offset, 0x77);
                }
                start_log(&mut e);
                assert_eq!(e.call(0x004e_d5f0, &args![group]).u32(), group);
                assert_eq!(
                    calls(&e, NI_PLANE_DEFAULT_CONSTRUCT),
                    vec![vec![group + 4], vec![group + 0x14]]
                );
                let lists: Vec<u32> = calls(&e, LIST_CONSTRUCT)
                    .iter()
                    .map(|words| words[0] - group)
                    .collect();
                assert_eq!(lists, vec![0x24, 0x30, 0x3c, 0x48, 0x64, 0x70, 0x7c, 0x88]);
                let pointers: Vec<(u32, u32)> = calls(&e, POINTER_TEMP_CONSTRUCT)
                    .iter()
                    .map(|words| (words[0] - group, words[1]))
                    .collect();
                assert_eq!(
                    pointers,
                    vec![
                        (0x54, 0),
                        (0x58, 0),
                        (0x94, 0),
                        (0x98, 0),
                        (0xa4, 0),
                        (0xa8, 0)
                    ]
                );
                // The sorters and the wading geometry are assigned null again.
                for offset in [0x58, 0x94, 0x98] {
                    assert_eq!(e.mem.u32(group + offset), 0);
                }
                // Only three of the flags are cleared.
                assert_eq!(
                    [0x5c, 0x5d, 0x5e, 0x5f, 0x60].map(|offset| e.mem.u8(group + offset)),
                    [0, 1, 1, 0, 0]
                );
            }

            #[test]
            fn a_group_destructor_gives_back_the_noise_map_it_alone_references() {
                let mut r = reflection();
                let e = &mut r.w.e;
                quiet(
                    e,
                    &[
                        POINTER_TEMP_DESTRUCT,
                        LIST_DESTRUCT,
                        RETURN_RENDERED_TEXTURE,
                    ],
                );
                let system = r.w.system;
                let group = r.w.group;
                let tes = e.global::<u32>(TES_POINTER);
                e.mem.set_u32(tes + 0x64, system.addr());
                e.register(TES_GET_WATER_SYSTEM, |e, a| ret(e.mem.u32(a[0] + 0x64)));
                // The water form's noise map has two references.
                let water_type = e.get(group, PlaceableWaterGroup::pWaterType).addr();
                let texture = e.mem.alloc(0x20);
                e.mem.set_u32(texture + 4, 2);
                e.mem.set_u32(water_type + 0x30, texture);
                // The group's own sorters, a wading geometry and member lists.
                e.mem.set_u32(r.w.other_property + 0x13c, 0x1234);
                e.mem.set_u32(group.addr() + 0x94, 0x11);
                e.mem.set_u32(group.addr() + 0x98, 0x22);
                let geometry = node(e);
                let geometry_property = e.mem.alloc(0x150);
                e.mem
                    .set_u32(geometry + GEOMETRY_PROPERTY, geometry_property);
                e.mem.set_u32(group.addr() + 0x58, geometry);
                let root = r.w.root;
                start_log(e);
                e.call(0x004e_d3e0, &args![group]);
                assert_eq!(e.mem.u32(group.addr() + 0x94), 0);
                assert_eq!(e.mem.u32(group.addr() + 0x98), 0);
                // The group is released from the system first: its water
                // reference loses the reflection map.
                assert_eq!(
                    e.get(
                        Ptr::<WaterShaderProperty>::new(r.w.other_property),
                        WaterShaderProperty::spReflectionMap
                    ),
                    0
                );
                // The noise map goes back to the texture manager and is cleared.
                assert_eq!(
                    calls(e, RETURN_RENDERED_TEXTURE),
                    vec![vec![0x0aaa_0000, texture]]
                );
                assert_eq!(e.mem.u32(water_type + 0x30), 0);
                // The two lists are emptied.
                assert_eq!(
                    calls(e, LIST_REMOVE_ALL),
                    vec![vec![group.addr() + 0x30], vec![group.addr() + 0x3c]]
                );
                // The wading geometry is detached from the water root and dropped.
                assert_eq!(
                    calls(e, DETACH_CHILD_DOUBLE_ADDRESS),
                    vec![vec![root, geometry]]
                );
                assert_eq!(e.mem.u32(group.addr() + 0x58), 0);
                // The members are destroyed in reverse order.
                let pointers: Vec<u32> = calls(e, POINTER_TEMP_DESTRUCT)
                    .iter()
                    .map(|words| words[0] - group.addr())
                    .collect();
                assert_eq!(pointers, vec![0xa8, 0xa4, 0x98, 0x94, 0x58, 0x54]);
                let lists: Vec<u32> = calls(e, LIST_DESTRUCT)
                    .iter()
                    .map(|words| words[0] - group.addr())
                    .collect();
                assert_eq!(lists, vec![0x88, 0x7c, 0x70, 0x64, 0x48, 0x3c, 0x30, 0x24]);
            }

            #[test]
            fn a_group_destructor_keeps_a_noise_map_others_reference() {
                let mut r = reflection();
                let e = &mut r.w.e;
                quiet(
                    e,
                    &[
                        POINTER_TEMP_DESTRUCT,
                        LIST_DESTRUCT,
                        RETURN_RENDERED_TEXTURE,
                    ],
                );
                let system = r.w.system;
                let group = r.w.group;
                let tes = e.global::<u32>(TES_POINTER);
                e.mem.set_u32(tes + 0x64, system.addr());
                e.register(TES_GET_WATER_SYSTEM, |e, a| ret(e.mem.u32(a[0] + 0x64)));
                let water_type = e.get(group, PlaceableWaterGroup::pWaterType).addr();
                let texture = e.mem.alloc(0x20);
                e.mem.set_u32(texture + 4, 3);
                e.mem.set_u32(water_type + 0x30, texture);
                start_log(e);
                e.call(0x004e_d3e0, &args![group]);
                assert!(calls(e, RETURN_RENDERED_TEXTURE).is_empty());
                assert_eq!(e.mem.u32(water_type + 0x30), texture);
                // No wading geometry: nothing is detached.
                assert!(calls(e, DETACH_CHILD_DOUBLE_ADDRESS).is_empty());
                // A group without a water type skips the map altogether.
                e.mem.set_u32(group.addr(), 0);
                e.call(0x004e_d3e0, &args![group]);
                assert!(calls(e, RETURN_RENDERED_TEXTURE).is_empty());
            }

            #[test]
            fn the_grey_texture_is_cleared_to_one_half() {
                let mut r = reflection();
                let e = &mut r.w.e;
                use_real_function(e, 0x004e_d290);
                e.set_global(GREY_VALUE, 0.5f32);
                e.register(CREATE_RENDERED_TEXTURE, |_, _| ret(0x0e00_0042));
                let log: Rc<RefCell<Vec<Vec<f32>>>> = Rc::default();
                let colors = log.clone();
                e.register_double(RENDERER_SET_COLOR_DOUBLE, move |e, a| {
                    colors
                        .borrow_mut()
                        .push((0..4).map(|i| e.mem.f32(a[1] + 4 * i)).collect());
                    Ret::default()
                });
                start_log(e);
                let system = r.w.system;
                let texture = e.call(0x004e_d290, &args![system]).u32();
                assert_eq!(texture, 0x0e00_0042);
                // Half grey while it is drawn, the original colour afterwards.
                assert_eq!(*log.borrow(), vec![vec![0.5; 4], vec![0.1, 0.2, 0.3, 0.4]]);
                assert_eq!(calls(e, CREATE_RENDERED_TEXTURE)[0][2..], [8, 0, 0, 0]);
                assert_eq!(calls(e, RENDER_TARGET_SET), vec![vec![7, 0x0e00_0043]]);
                assert_eq!(calls(e, RENDER_TARGET_RESET).len(), 1);
                let scope = calls(e, ALLOCATION_SCOPE_CONSTRUCT);
                assert_eq!(&scope[0][1..], &[0x1d, 1, TESWATER_SOURCE_PATH, 0x138a]);
                assert_eq!(calls(e, ALLOCATION_SCOPE_DESTRUCT).len(), 1);
            }

            // --- the image-space helpers ------------------------------------

            #[test]
            fn the_effect_param_is_constructed_destroyed_and_deleted() {
                let mut e = water_engine();
                quiet(
                    &mut e,
                    &[
                        EFFECT_PARAM_FIRST_MEMBER_CONSTRUCT,
                        EFFECT_PARAM_SECOND_MEMBER_CONSTRUCT,
                        EFFECT_PARAM_FIRST_MEMBER_DESTRUCT,
                        EFFECT_PARAM_SECOND_MEMBER_DESTRUCT,
                    ],
                );
                e.register(OPERATOR_DELETE, |_, _| Ret::default());
                let param = e.mem.alloc(0x24);
                start_log(&mut e);
                assert_eq!(e.call(0x004e_ba20, &args![param]).u32(), param);
                assert_eq!(e.mem.u32(param), 0x0102_31cc);
                assert_eq!(
                    calls(&e, EFFECT_PARAM_FIRST_MEMBER_CONSTRUCT),
                    vec![vec![param + 4, 0, 1]]
                );
                assert_eq!(
                    calls(&e, EFFECT_PARAM_SECOND_MEMBER_CONSTRUCT),
                    vec![vec![param + 0x14, 0, 1]]
                );
                // The destructor body: the second member goes first.
                e.mem.set_u32(param, 0);
                start_log(&mut e);
                e.call(0x004e_ba90, &args![param]);
                assert_eq!(e.mem.u32(param), 0x0102_31cc);
                let order: Vec<(u32, u32)> = e
                    .call_log
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(address, words)| (*address, words[0]))
                    .collect();
                assert_eq!(
                    order,
                    vec![
                        (0x004e_ba90, param),
                        (EFFECT_PARAM_SECOND_MEMBER_DESTRUCT, param + 0x14),
                        (EFFECT_PARAM_FIRST_MEMBER_DESTRUCT, param + 4),
                    ]
                );
                // The scalar deleting destructor frees only on bit 0.
                start_log(&mut e);
                assert_eq!(e.call(0x004e_bb00, &args![param, 0u32]).u32(), param);
                assert!(calls(&e, OPERATOR_DELETE).is_empty());
                assert_eq!(e.call(0x004e_bb00, &args![param, 1u32]).u32(), param);
                assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![param]]);
            }

            #[test]
            fn the_member_destructors_forward_to_their_bodies() {
                let mut e = water_engine();
                quiet(
                    &mut e,
                    &[
                        EFFECT_PARAM_FIRST_MEMBER_DESTRUCT,
                        EFFECT_PARAM_SECOND_MEMBER_DESTRUCT,
                    ],
                );
                start_log(&mut e);
                e.call(0x004e_bb30, &args![0x100u32]);
                e.call(0x004e_bb50, &args![0x200u32]);
                assert_eq!(
                    calls(&e, EFFECT_PARAM_FIRST_MEMBER_DESTRUCT),
                    vec![vec![0x100]]
                );
                assert_eq!(
                    calls(&e, EFFECT_PARAM_SECOND_MEMBER_DESTRUCT),
                    vec![vec![0x200]]
                );
            }

            #[test]
            fn the_texture_holder_starts_empty() {
                let mut e = water_engine();
                e.register(POINTER_TEMP_CONSTRUCT, |e, a| {
                    e.mem.set_u32(a[0], a[1]);
                    ret(a[0])
                });
                let holder = e.mem.alloc(0x10);
                for i in 0..4 {
                    e.mem.set_u32(holder + 4 * i, 0x0101_0101);
                }
                start_log(&mut e);
                assert_eq!(e.call(0x004e_bb70, &args![holder]).u32(), holder);
                assert_eq!(calls(&e, POINTER_TEMP_CONSTRUCT), vec![vec![holder + 4, 0]]);
                assert_eq!(
                    [0, 1, 2, 3].map(|offset| e.mem.u8(holder + offset)),
                    [0, 0, 0, 0x01]
                );
                assert_eq!(e.mem.u32(holder + 4), 0);
                assert_eq!(e.mem.u32(holder + 8), 0);
                assert_eq!(e.mem.u32(holder + 0xc), 0);
            }

            #[test]
            fn an_effect_is_read_from_the_array_of_the_image_space_manager() {
                let mut e = water_engine();
                e.register(ARRAY_ELEMENT_ADDRESS, |e, a| {
                    ret(e.mem.u32(a[0] + 4) + 4 * a[1])
                });
                let manager = e.mem.alloc(0x20);
                let array = e.mem.alloc(0x40);
                e.mem.set_u32(manager + 8, array);
                e.mem.set_u32(array + 4 * 5, 0xeffec7);
                assert_eq!(e.call(0x004e_bbc0, &args![manager, 5u32]).u32(), 0xeffec7);
            }

            // --- the pointer maps -------------------------------------------

            #[test]
            fn the_map_constructors_set_their_vtables_after_the_base() {
                let mut e = water_engine();
                e.register(NI_ALLOC_ARRAY, |e, a| ret(e.mem.alloc(a[0])));
                e.register(MEMORY_SET, |e, a| {
                    for i in 0..a[2] {
                        e.mem.set_u8(a[0] + i, a[1] as u8);
                    }
                    Ret::default()
                });
                e.register(WADING_MAP_BASE_CONSTRUCT, |e, a| {
                    e.mem.set_u32(a[0], 0xba5e);
                    ret(a[0])
                });
                for (constructor, vtable) in
                    [(0x004e_d7a0u32, 0x0102_31f4u32), (0x004e_d7d0, 0x0102_3214)]
                {
                    let map = e.mem.alloc(0x10);
                    start_log(&mut e);
                    assert_eq!(e.call(constructor, &args![map, 8u32]).u32(), map);
                    assert_eq!(e.mem.u32(map), vtable, "the derived vtable wins");
                    assert_eq!(e.mem.u32(map + 4), 8);
                    assert_eq!(e.mem.u32(map + 0xc), 0);
                    let buckets = e.mem.u32(map + 8);
                    assert_ne!(buckets, 0);
                    assert_eq!(calls(&e, NI_ALLOC_ARRAY), vec![vec![32]]);
                    assert_eq!(calls(&e, MEMORY_SET), vec![vec![buckets, 0, 32]]);
                }
                // The wading map's base constructor is the next session's.
                let map = e.mem.alloc(0x10);
                start_log(&mut e);
                assert_eq!(e.call(0x004e_d800, &args![map, 8u32]).u32(), map);
                assert_eq!(calls(&e, WADING_MAP_BASE_CONSTRUCT), vec![vec![map, 8]]);
                assert_eq!(e.mem.u32(map), 0x0102_3234);
            }

            #[test]
            fn the_base_constructors_allocate_and_clear_the_buckets() {
                let mut e = water_engine();
                e.register(NI_ALLOC_ARRAY, |e, a| ret(e.mem.alloc(a[0])));
                e.register(MEMORY_SET, |_, _| Ret::default());
                for (constructor, vtable) in
                    [(0x004e_d960u32, 0x0102_3254u32), (0x004e_da60, 0x0102_3274)]
                {
                    let map = e.mem.alloc(0x10);
                    e.mem.set_u32(map + 0xc, 99);
                    start_log(&mut e);
                    assert_eq!(e.call(constructor, &args![map, 0x10u32]).u32(), map);
                    assert_eq!(e.mem.u32(map), vtable);
                    assert_eq!(e.mem.u32(map + 4), 0x10);
                    assert_eq!(e.mem.u32(map + 0xc), 0, "the count");
                    let buckets = e.mem.u32(map + 8);
                    assert_eq!(calls(&e, NI_ALLOC_ARRAY), vec![vec![0x40]]);
                    assert_eq!(calls(&e, MEMORY_SET), vec![vec![buckets, 0, 0x40]]);
                }
            }

            #[test]
            fn the_map_deleting_destructors_free_on_bit_zero() {
                let mut e = water_engine();
                e.register(OPERATOR_DELETE, |_, _| Ret::default());
                for (function, body) in [
                    (0x004e_d830u32, REFERENCE_MAP_DESTRUCT),
                    (0x004e_d860, WATER_FORM_MAP_DESTRUCT),
                    (0x004e_d890, WADING_MAP_DESTRUCT),
                ] {
                    e.register(body, |_, _| Ret::default());
                    start_log(&mut e);
                    assert_eq!(e.call(function, &args![0x500u32, 0u32]).u32(), 0x500);
                    assert_eq!(calls(&e, body), vec![vec![0x500]]);
                    assert!(calls(&e, OPERATOR_DELETE).is_empty());
                    assert_eq!(e.call(function, &args![0x500u32, 1u32]).u32(), 0x500);
                    assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x500]]);
                }
            }

            #[test]
            fn the_wading_map_base_constructor_allocates_and_clears_the_buckets() {
                let mut e = water_engine();
                e.register(NI_ALLOC_ARRAY, |e, a| ret(e.mem.alloc(a[0])));
                e.register(MEMORY_SET, |_, _| Ret::default());
                let map = e.mem.alloc(0x10);
                e.mem.set_u32(map + 0xc, 99);
                start_log(&mut e);
                assert_eq!(e.call(0x004e_db60, &args![map, 0x20u32]).u32(), map);
                assert_eq!(e.mem.u32(map), 0x0102_3294);
                assert_eq!(e.mem.u32(map + 4), 0x20);
                assert_eq!(e.mem.u32(map + 0xc), 0, "the count");
                let buckets = e.mem.u32(map + 8);
                assert_eq!(calls(&e, NI_ALLOC_ARRAY), vec![vec![0x80]]);
                assert_eq!(calls(&e, MEMORY_SET), vec![vec![buckets, 0, 0x80]]);
            }

            #[test]
            fn the_two_word_constructors_run_their_body_then_set_the_vtable() {
                let mut e = water_engine();
                for (constructor, body, vtable) in [
                    (0x004e_dc90u32, CONSTRUCT_BODY_004EDF70, 0x0102_32bcu32),
                    (0x004e_dcf0, CONSTRUCT_BODY_004EDFE0, 0x0102_32cc),
                ] {
                    e.register(body, |e, a| {
                        e.mem.set_u32(a[0], 0xba5e);
                        ret(a[0])
                    });
                    let object = e.mem.alloc(0x10);
                    start_log(&mut e);
                    assert_eq!(
                        e.call(constructor, &args![object, 6u32, 9u32]).u32(),
                        object
                    );
                    assert_eq!(calls(&e, body), vec![vec![object, 6, 9]]);
                    assert_eq!(e.mem.u32(object), vtable, "the derived vtable wins");
                }
            }

            #[test]
            fn the_map_base_deleting_destructors_free_on_bit_zero() {
                let mut e = water_engine();
                e.register(OPERATOR_DELETE, |_, _| Ret::default());
                for (function, body) in [
                    (0x004e_dd20u32, REFERENCE_MAP_BASE_DESTRUCT),
                    (0x004e_dd50, WATER_FORM_MAP_BASE_DESTRUCT),
                    (0x004e_dd80, WADING_MAP_BASE_DESTRUCT),
                ] {
                    e.register(body, |_, _| Ret::default());
                    start_log(&mut e);
                    assert_eq!(e.call(function, &args![0x500u32, 0u32]).u32(), 0x500);
                    assert_eq!(calls(&e, body), vec![vec![0x500]]);
                    assert!(calls(&e, OPERATOR_DELETE).is_empty());
                    assert_eq!(e.call(function, &args![0x500u32, 1u32]).u32(), 0x500);
                    assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x500]]);
                }
            }

            // --- the world and sky reflection finish ------------------------

            struct Finishing {
                r: Reflection,
                math: PlaneMath,
                /// The clear colours the renderer was given.
                clear_colors: Rc<RefCell<Vec<Vec<f32>>>>,
                world_camera: u32,
                sky_camera: u32,
                player: u32,
                player_node: u32,
            }

            /// The reflection scene (one group at height 10, the world space's
            /// water height at 8) with the cameras and sorters of the world and
            /// sky reflections, the plane math, a player with a 3D node and the
            /// doubles of the finish: the world thread stage is 3, the sky's 6.
            fn finishing() -> Finishing {
                let mut r = reflection();
                let e = &mut r.w.e;
                let tes = e.global::<u32>(TES_POINTER);
                let world_space = e.mem.u32(tes + 0x50);
                e.mem.set_f32(world_space + 0x7c, 8.0);
                e.register(WORLD_SPACE_WATER_HEIGHT, |e, a| {
                    e.mem.f32(a[0] + 0x7c).into_ret()
                });
                let math = install_plane_math(e);
                let clear_colors: Rc<RefCell<Vec<Vec<f32>>>> = Rc::default();
                let log = clear_colors.clone();
                e.register_double(RENDERER_SET_COLOR_DOUBLE, move |e, a| {
                    log.borrow_mut()
                        .push((0..4).map(|i| e.mem.f32(a[1] + 4 * i)).collect());
                    Ret::default()
                });
                quiet(e, &[RENDER_STATE_SET, RENDER_ACCUMULATED_SCENE]);
                e.register(CREATE_RENDERED_TEXTURE, |_, _| ret(0x0e00_0777));
                e.register(NODE_SET_CULLED, |e, a| {
                    e.mem.set_u32(a[0] + 0x30, a[1]);
                    Ret::default()
                });
                e.set_global(WORLD_REFLECTION_THREAD_STAGE, 3u32);
                e.set_global(SKY_REFLECTION_THREAD_STAGE, 6u32);
                let world_camera = e.mem.alloc(0x120);
                let sky_camera = e.mem.alloc(0x120);
                e.set_global(WORLD_REFLECTION_CAMERA, world_camera);
                e.set_global(SKY_REFLECTION_CAMERA, sky_camera);
                e.set_global(WORLD_REFLECTION_SORTER, 0x4242u32);
                e.set_global(SKY_REFLECTION_SORTER, 0x4343u32);
                let player = object(e, ACTOR_VTABLE, 0x700);
                let player_node = node(e);
                e.mem.set_u32(player + REF_NODE, player_node);
                e.set_global(PLAYER_CHARACTER, player);
                Finishing {
                    r,
                    math,
                    clear_colors,
                    world_camera,
                    sky_camera,
                    player,
                    player_node,
                }
            }

            impl Finishing {
                fn world(&mut self) {
                    start_log(&mut self.r.w.e);
                    let system = self.r.w.system;
                    self.r.w.e.call(0x004e_b540, &args![system]);
                }

                fn sky(&mut self) {
                    start_log(&mut self.r.w.e);
                    let system = self.r.w.system;
                    self.r.w.e.call(0x004e_bbe0, &args![system]);
                }
            }

            #[test]
            fn the_world_reflection_is_finished_through_the_water_plane() {
                let mut f = finishing();
                f.world();
                let e = &f.r.w.e;
                let camera = f.world_camera;
                // The clear colour has alpha 0 while the reflection renders and is
                // restored afterwards.
                assert_eq!(
                    *f.clear_colors.borrow(),
                    vec![vec![0.1, 0.2, 0.3, 0.0], vec![0.1, 0.2, 0.3, 0.4]]
                );
                assert!(
                    calls(e, CREATE_RENDERED_TEXTURE).is_empty(),
                    "the map exists"
                );
                // The terrain is toggled around the draw.
                assert_eq!(calls(e, TERRAIN_TOGGLE_ONE).len(), 1);
                assert_eq!(calls(e, TERRAIN_TOGGLE_ZERO).len(), 1);
                // The world map (0x0e000004) is stopped into render target 7.
                assert_eq!(calls(e, RENDER_TARGET_SET), vec![vec![7, 0x0e00_0005]]);
                let object = e.global::<u32>(RENDER_OBJECT_GLOBAL);
                assert_eq!(
                    calls(e, RENDER_OBJECT_SET_CAMERA_DATA),
                    vec![vec![object, camera + 0x100]]
                );
                assert_eq!(
                    calls(e, MT_SET_THREAD_STAGE_ONE),
                    vec![vec![MT_RENDERING_SYSTEM, 1, 4]]
                );
                // The plane through (0, 0, 8): normal (0, 0, 1), minus the
                // constant, moved into view space (the eye is at z = 3).
                assert_eq!(
                    *f.math.clip_planes.borrow(),
                    vec![(f.math.clip_device, 0, vec![0.0, 0.0, 1.0, -5.0])]
                );
                assert_eq!(
                    calls(e, RENDER_STATE_SET),
                    vec![vec![1, 0, 0, 0, 0, 0, 0], vec![0; 7]]
                );
                assert_eq!(
                    calls(e, RENDER_ACCUMULATED_SCENE),
                    vec![vec![camera, 0x4242, 0]]
                );
                assert_eq!(calls(e, RENDER_TARGET_RESET).len(), 1);
                assert_eq!(e.global::<u32>(WORLD_REFLECTION_CAMERA), 0);
                // The sky is left alone, and so is the blur (the setting is off).
                assert_eq!(e.global::<u32>(SKY_REFLECTION_CAMERA), f.sky_camera);
                assert!(calls(e, IMAGE_SPACE_RENDER_DISPLACEMENT).is_empty());
                let scope = calls(e, ALLOCATION_SCOPE_CONSTRUCT);
                assert_eq!(&scope[0][1..], &[0x1d, 1, TESWATER_SOURCE_PATH, 0x10b8]);
                assert_eq!(calls(e, ALLOCATION_SCOPE_DESTRUCT).len(), 1);
            }

            #[test]
            fn the_world_reflection_makes_its_map_when_there_is_none() {
                let mut f = finishing();
                f.r.w.e.set_global(WORLD_REFLECTION_MAP, 0u32);
                f.world();
                let e = &f.r.w.e;
                let renderer = f.math.renderer;
                assert_eq!(
                    calls(e, CREATE_RENDERED_TEXTURE),
                    vec![vec![0x0aaa_0000, renderer, 9, 0, 0, 0]]
                );
                assert_eq!(e.global::<u32>(WORLD_REFLECTION_MAP), 0x0e00_0777);
                assert_eq!(calls(e, RENDER_TARGET_SET), vec![vec![7, 0x0e00_0778]]);
            }

            #[test]
            fn the_terrain_is_left_alone_in_an_interior_when_not_ready_or_forced_high_detail() {
                // An interior.
                let mut f = finishing();
                let tes = f.r.w.e.global::<u32>(TES_POINTER);
                f.r.w.e.mem.set_u32(tes + 0x34, 0x77);
                f.world();
                assert!(calls(&f.r.w.e, TERRAIN_TOGGLE_ONE).is_empty());
                assert!(calls(&f.r.w.e, TERRAIN_TOGGLE_ZERO).is_empty());
                // The terrain is not ready.
                let mut f = finishing();
                f.r.w.e.register(TERRAIN_READY, |_, _| ret(0));
                f.world();
                assert!(calls(&f.r.w.e, TERRAIN_TOGGLE_ONE).is_empty());
                assert!(calls(&f.r.w.e, TERRAIN_TOGGLE_ZERO).is_empty());
                // bForceHighDetailReflections keeps the world reflection off it.
                let mut f = finishing();
                set_setting(&mut f.r.w.e, SETTING_FORCE_HIGH_DETAIL_REFLECTIONS, true);
                f.world();
                assert!(calls(&f.r.w.e, TERRAIN_TOGGLE_ONE).is_empty());
                assert!(calls(&f.r.w.e, TERRAIN_TOGGLE_ZERO).is_empty());
                // The sky does not look at that setting.
                let mut f = finishing();
                set_setting(&mut f.r.w.e, SETTING_FORCE_HIGH_DETAIL_REFLECTIONS, true);
                f.sky();
                assert_eq!(calls(&f.r.w.e, TERRAIN_TOGGLE_ONE).len(), 1);
                assert_eq!(calls(&f.r.w.e, TERRAIN_TOGGLE_ZERO).len(), 1);
            }

            #[test]
            fn the_culled_player_is_shown_again_when_the_world_setup_had_culled_it() {
                let mut f = finishing();
                f.r.w.e.mem.set_u32(f.player_node + 0x30, 1);
                f.r.w.e.mem.set_u8(f.player + 0x64a, 1);
                let system = f.r.w.system;
                f.r.w.e.set(system, TESWaterSystem::bCull3rdPerson, true);
                f.world();
                assert_eq!(
                    calls(&f.r.w.e, NODE_SET_CULLED),
                    vec![vec![f.player_node, 0]]
                );
                assert_eq!(f.r.w.e.mem.u32(f.player_node + 0x30), 0);
                // Nothing for a player the setup did not cull, or without the flag.
                let mut f = finishing();
                let system = f.r.w.system;
                f.r.w.e.set(system, TESWaterSystem::bCull3rdPerson, true);
                f.world();
                assert!(calls(&f.r.w.e, NODE_SET_CULLED).is_empty());
                let mut f = finishing();
                f.r.w.e.mem.set_u8(f.player + 0x64a, 1);
                f.world();
                assert!(calls(&f.r.w.e, NODE_SET_CULLED).is_empty());
            }

            #[test]
            fn the_reflection_finishes_need_the_water_shader_and_a_camera() {
                let mut f = finishing();
                f.r.w.e.register(WATER_SHADER_ENABLED, |_, _| ret(0));
                f.world();
                f.sky();
                assert!(f.clear_colors.borrow().is_empty());
                assert!(calls(&f.r.w.e, RENDER_TARGET_SET).is_empty());
                assert_eq!(
                    f.r.w.e.global::<u32>(WORLD_REFLECTION_CAMERA),
                    f.world_camera
                );
                // The scope is opened and closed all the same.
                assert_eq!(calls(&f.r.w.e, ALLOCATION_SCOPE_DESTRUCT).len(), 1);
                let mut f = finishing();
                f.r.w.e.set_global(WORLD_REFLECTION_CAMERA, 0u32);
                f.world();
                assert!(f.clear_colors.borrow().is_empty());
                assert!(calls(&f.r.w.e, RENDER_TARGET_SET).is_empty());
                let mut f = finishing();
                f.r.w.e.set_global(SKY_REFLECTION_CAMERA, 0u32);
                f.sky();
                assert!(f.clear_colors.borrow().is_empty());
                assert!(calls(&f.r.w.e, RENDER_TARGET_SET).is_empty());
            }

            #[test]
            fn the_blur_draws_the_world_reflection_through_the_blur_effect() {
                let mut f = finishing();
                let e = &mut f.r.w.e;
                set_setting(e, SETTING_USE_WATER_REFLECTION_BLUR, true);
                set_int_setting(e, SETTING_WATER_BLUR_AMOUNT, 3);
                e.set_global(ONE, 1.0f64);
                // The image-space manager: its effect array holds the blur effect
                // 0x10 + 3.
                let manager = e.mem.alloc(0x20);
                let array = e.mem.alloc(0x100);
                let effect = e.mem.alloc(0x40);
                e.mem.set_u32(manager + 8, array);
                e.mem.set_u32(array + 4 * 0x13, effect);
                e.set_global(IMAGE_SPACE_MANAGER, manager);
                e.register(ARRAY_ELEMENT_ADDRESS, |e, a| {
                    ret(e.mem.u32(a[0] + 4) + 4 * a[1])
                });
                quiet(
                    e,
                    &[
                        POINTER_TEMP_CONSTRUCT,
                        IMAGE_SPACE_TEXTURE_SET,
                        IMAGE_SPACE_TEXTURE_DESTRUCT,
                        IMAGE_SPACE_EFFECT_SET_TEXTURE,
                        EFFECT_PARAM_FIRST_MEMBER_CONSTRUCT,
                        EFFECT_PARAM_SECOND_MEMBER_CONSTRUCT,
                        EFFECT_PARAM_FIRST_MEMBER_DESTRUCT,
                        EFFECT_PARAM_SECOND_MEMBER_DESTRUCT,
                    ],
                );
                f.world();
                let e = &f.r.w.e;
                let renderer = f.math.renderer;
                // The blur texture (kind 0x16) goes into slot 2 of the effect.
                assert_eq!(
                    calls(e, CREATE_RENDERED_TEXTURE),
                    vec![vec![0x0aaa_0000, renderer, 0x16, 0, 0, 0]]
                );
                let holder = calls(e, IMAGE_SPACE_TEXTURE_SET)[0][0];
                assert_eq!(
                    calls(e, IMAGE_SPACE_TEXTURE_SET),
                    vec![vec![holder, 0x0e00_0777]]
                );
                assert_eq!(calls(e, POINTER_TEMP_CONSTRUCT), vec![vec![holder + 4, 0]]);
                assert_eq!(
                    calls(e, IMAGE_SPACE_EFFECT_SET_TEXTURE),
                    vec![vec![effect, 2, holder, 0]]
                );
                // The effect param is the 0x24-byte object after the holder.
                let param = holder + 0x10;
                assert_eq!(e.mem.u32(param), 0x0102_31cc);
                assert_eq!(
                    calls(e, IMAGE_SPACE_RENDER_DISPLACEMENT),
                    vec![vec![
                        manager,
                        0x13,
                        renderer,
                        0x0e00_0004,
                        0x0e00_0004,
                        param,
                        1
                    ]]
                );
                assert_eq!(e.global::<f32>(WATER_BLUR_FACTOR), 4.0);
                // The blur texture is handed back, the param and the holder destroyed.
                assert_eq!(
                    calls(e, RETURN_RENDERED_TEXTURE),
                    vec![vec![0x0aaa_0000, 0x0e00_0777]]
                );
                assert_eq!(
                    calls(e, EFFECT_PARAM_SECOND_MEMBER_DESTRUCT),
                    vec![vec![param + 0x14]]
                );
                assert_eq!(
                    calls(e, EFFECT_PARAM_FIRST_MEMBER_DESTRUCT),
                    vec![vec![param + 4]]
                );
                assert_eq!(calls(e, IMAGE_SPACE_TEXTURE_DESTRUCT), vec![vec![holder]]);
                // The blur is the last thing the finish does (after the camera is dropped).
                assert_eq!(e.global::<u32>(WORLD_REFLECTION_CAMERA), 0);
            }

            #[test]
            fn the_sky_reflection_is_finished_with_its_own_camera_map_and_stage() {
                let mut f = finishing();
                // The sky has no map yet; the blur setting does not concern it.
                f.r.w.e.set_global(SKY_REFLECTION_MAP, 0u32);
                set_setting(&mut f.r.w.e, SETTING_USE_WATER_REFLECTION_BLUR, true);
                f.r.w.e.mem.set_u8(f.player + 0x64a, 1);
                let system = f.r.w.system;
                f.r.w.e.set(system, TESWaterSystem::bCull3rdPerson, true);
                f.sky();
                let e = &f.r.w.e;
                let camera = f.sky_camera;
                assert_eq!(
                    *f.clear_colors.borrow(),
                    vec![vec![0.1, 0.2, 0.3, 0.0], vec![0.1, 0.2, 0.3, 0.4]]
                );
                assert_eq!(calls(e, CREATE_RENDERED_TEXTURE)[0][2..], [9, 0, 0, 0]);
                assert_eq!(e.global::<u32>(SKY_REFLECTION_MAP), 0x0e00_0777);
                assert_eq!(calls(e, RENDER_TARGET_SET), vec![vec![7, 0x0e00_0778]]);
                let object = e.global::<u32>(RENDER_OBJECT_GLOBAL);
                assert_eq!(
                    calls(e, RENDER_OBJECT_SET_CAMERA_DATA),
                    vec![vec![object, camera + 0x100]]
                );
                assert_eq!(
                    calls(e, MT_SET_THREAD_STAGE_ONE),
                    vec![vec![MT_RENDERING_SYSTEM, 1, 7]]
                );
                assert_eq!(
                    calls(e, RENDER_ACCUMULATED_SCENE),
                    vec![vec![camera, 0x4343, 0]]
                );
                assert_eq!(
                    *f.math.clip_planes.borrow(),
                    vec![(f.math.clip_device, 0, vec![0.0, 0.0, 1.0, -5.0])]
                );
                assert_eq!(e.global::<u32>(SKY_REFLECTION_CAMERA), 0);
                assert_eq!(e.global::<u32>(WORLD_REFLECTION_CAMERA), f.world_camera);
                // No culling and no blur for the sky.
                assert!(calls(e, NODE_SET_CULLED).is_empty());
                assert!(calls(e, IMAGE_SPACE_RENDER_DISPLACEMENT).is_empty());
                let scope = calls(e, ALLOCATION_SCOPE_CONSTRUCT);
                assert_eq!(&scope[0][1..], &[0x1d, 1, TESWATER_SOURCE_PATH, 0x1114]);
            }

            // --- the depth setup of a group (004ebef0) -------------------------

            /// The reflection scene with the doubles of the depth camera: the
            /// viewer's translate (1, 2, 3), scale 2.5 and the float at +0xfc
            /// 3.5; the group's depth thread stage is 5. The base form has the
            /// bit `0x10000000` the setup needs for the lists.
            fn depth_scene() -> Reflection {
                let mut r = reflection();
                r.set_flags(0x1000_0000);
                let viewer = r.viewer;
                let e = &mut r.w.e;
                for (i, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
                    e.mem.set_f32(viewer + 0x8c + 4 * i as u32, value);
                }
                e.mem.set_f32(viewer + 0x98, 2.5);
                e.mem.set_f32(viewer + 0xfc, 3.5);
                e.register(NODE_SCALE_SOURCE, |e, a| e.mem.f32(a[0] + 0x98).into_ret());
                e.register(CAMERA_FLOAT_FC_READ, |e, a| {
                    e.mem.f32(a[0] + 0xfc).into_ret()
                });
                e.register(CAMERA_FLOAT_FC_WRITE, |e, a| {
                    e.mem.set_u32(a[0] + 0xfc, a[1]);
                    Ret::default()
                });
                quiet(e, &[NODE_SET_LOCAL_ROTATE, NODE_SET_LOCAL_SCALE]);
                e.register(CREATE_RENDERED_TEXTURE, |_, _| ret(0x0e00_0777));
                let group = r.w.group;
                r.w.e.set(group, PlaceableWaterGroup::iDepthThreadStage, 5);
                r
            }

            fn run_depth(r: &mut Reflection) {
                start_log(&mut r.w.e);
                let (system, viewer, group) = (r.w.system, r.viewer, r.w.group);
                r.w.e.call(0x004e_bef0, &args![system, viewer, group]);
            }

            fn static_depth_objects(r: &Reflection) -> Vec<u32> {
                list_items(&r.w.e, r.w.group.addr() + 0x7c)
            }

            fn dynamic_depth_objects(r: &Reflection) -> Vec<u32> {
                list_items(&r.w.e, r.w.group.addr() + 0x88)
            }

            /// A reference with a 3D node of its own (and the node).
            fn reference_with_node(e: &mut Engine) -> (u32, u32) {
                let reference = actor(e, 0.0, 0.0, 0.0);
                let node = node(e);
                e.mem.set_u32(reference + REF_NODE, node);
                (reference, node)
            }

            /// A second water reference of the group, in range, with a shader
            /// property of its own.
            fn add_second_reference(r: &mut Reflection) -> u32 {
                let (second, _, geometry, _) = geometry_chain(&mut r.w.e);
                let property = r.w.e.mem.alloc(0x150);
                r.w.e.mem.set_u32(geometry + GEOMETRY_PROPERTY, property);
                r.w.e.mem.set_u32(second + REF_BASE_FORM, r.base_form);
                let cell = r.w.e.mem.u32(r.w.water_reference + REF_CELL);
                r.w.e.mem.set_u32(second + REF_CELL, cell);
                let top = r.w.e.mem.u32(second + REF_NODE);
                r.w.e.mem.set_u8(top + NODE_RANGE, 1);
                let first = r.w.water_reference;
                fill_list(&mut r.w.e, r.w.group.addr() + 0x24, &[first, second]);
                property
            }

            #[test]
            fn the_depth_camera_is_copied_from_the_viewer_and_the_task_is_handed_over() {
                let mut r = depth_scene();
                run_depth(&mut r);
                let e = &r.w.e;
                let group = r.w.group;
                let viewer = r.viewer;
                let camera = e.get(group, PlaceableWaterGroup::spDepthCamera);
                assert_ne!(camera, 0);
                assert_eq!(*r.w.translations.borrow(), vec![(camera, [1.0, 2.0, 3.0])]);
                assert_eq!(
                    calls(e, NODE_SET_LOCAL_ROTATE),
                    vec![vec![camera, viewer + 0x68]]
                );
                assert_eq!(
                    calls(e, NODE_SET_LOCAL_SCALE),
                    vec![vec![camera, 2.5f32.to_bits()]]
                );
                assert_eq!(
                    calls(e, CAMERA_FLOAT_FC_WRITE),
                    vec![vec![camera, 3.5f32.to_bits()]]
                );
                assert_eq!(e.mem.f32(camera + 0xfc), 3.5);
                assert_eq!(
                    calls(e, CAMERA_SET_VIEW_FRUSTUM),
                    vec![vec![camera, viewer + 0xdc]]
                );
                // The camera is updated with a fresh `NiUpdateData`.
                let update = calls(e, UPDATE_DATA_CONSTRUCT);
                assert_eq!(update[0][1..], [0.0f32.to_bits(), 0, 0]);
                assert_eq!(calls(e, NODE_UPDATE), vec![vec![camera, update[0][0]]]);
                // The depth accumulator is made and prepared.
                let accumulator = e.get(group, PlaceableWaterGroup::spDepthSorter);
                assert_ne!(accumulator, 0);
                assert_eq!(
                    calls(e, ACCUMULATOR_SET_WORD_194),
                    vec![vec![accumulator, r.table_entry]]
                );
                assert_eq!(e.mem.u8(accumulator + 0x165), 1);
                assert_eq!(*r.accumulator_cameras.borrow(), vec![(accumulator, camera)]);
                assert_eq!(
                    calls(e, ACCUMULATOR_SET_ACCUMULATE),
                    vec![vec![accumulator, 1]]
                );
                // Stage 5 + 1.
                assert_eq!(
                    calls(e, MT_ADD_ACCUM_TASK),
                    vec![vec![
                        MT_RENDERING_SYSTEM,
                        camera,
                        0,
                        0,
                        group.addr() + 0x7c,
                        group.addr() + 0x88,
                        accumulator,
                        0,
                        6,
                        0
                    ]]
                );
                assert_eq!(
                    calls(e, MT_SET_THREAD_STAGE),
                    vec![vec![MT_RENDERING_SYSTEM, 0, 6]]
                );
                let scope = calls(e, ALLOCATION_SCOPE_CONSTRUCT);
                assert_eq!(&scope[0][1..], &[0x1d, 1, TESWATER_SOURCE_PATH, 0x114f]);
                assert_eq!(calls(e, ALLOCATION_SCOPE_DESTRUCT), vec![vec![scope[0][0]]]);
            }

            #[test]
            fn an_existing_depth_accumulator_and_map_are_kept() {
                let mut r = depth_scene();
                let group = r.w.group;
                let accumulator = r.w.e.mem.alloc(0x280);
                r.w.e.mem.set_u32(accumulator, ACCUMULATOR_VTABLE);
                r.w.e
                    .set(group, PlaceableWaterGroup::spDepthSorter, accumulator);
                run_depth(&mut r);
                assert_eq!(
                    r.w.e.get(group, PlaceableWaterGroup::spDepthSorter),
                    accumulator
                );
                assert_eq!(r.w.e.mem.u8(accumulator + 0x165), 1);
                assert!(calls(&r.w.e, ACCUMULATOR_CONSTRUCT).is_empty());
                assert!(calls(&r.w.e, CREATE_RENDERED_TEXTURE).is_empty());
            }

            #[test]
            fn the_properties_of_references_in_range_take_the_depth_map() {
                let mut r = depth_scene();
                let second_property = add_second_reference(&mut r);
                run_depth(&mut r);
                for property in [r.w.other_property, second_property] {
                    assert_eq!(
                        r.w.e.get(
                            Ptr::<WaterShaderProperty>::new(property),
                            WaterShaderProperty::spDepthMap
                        ),
                        0x0e00_0003
                    );
                }
                assert!(calls(&r.w.e, CREATE_RENDERED_TEXTURE).is_empty());
                // Without a depth map the setup makes one (kind 0x11).
                let mut r = depth_scene();
                r.w.e.set_global(DEPTH_MAP, 0u32);
                run_depth(&mut r);
                let renderer = r.w.e.call(RENDERER, &[]).u32();
                assert_eq!(
                    calls(&r.w.e, CREATE_RENDERED_TEXTURE),
                    vec![vec![0x0aaa_0000, renderer, 0x11, 0, 0, 0]]
                );
                assert_eq!(r.w.e.global::<u32>(DEPTH_MAP), 0x0e00_0777);
                assert_eq!(
                    r.w.e.get(
                        Ptr::<WaterShaderProperty>::new(r.w.other_property),
                        WaterShaderProperty::spDepthMap
                    ),
                    0x0e00_0777
                );
            }

            #[test]
            fn a_reference_out_of_range_or_in_another_cell_is_left_alone() {
                let mut r = depth_scene();
                let top = r.w.e.mem.u32(r.w.water_reference + REF_NODE);
                r.w.e.mem.set_u8(top + NODE_RANGE, 0);
                run_depth(&mut r);
                assert_eq!(
                    r.w.e.get(
                        Ptr::<WaterShaderProperty>::new(r.w.other_property),
                        WaterShaderProperty::spDepthMap
                    ),
                    0
                );
                let mut r = depth_scene();
                let cell = r.w.e.mem.u32(r.w.water_reference + REF_CELL);
                r.w.e.mem.set_u8(cell + 0x26, 5);
                run_depth(&mut r);
                assert!(calls(&r.w.e, NODE_GET_PROPERTY).is_empty());
                let mut r = depth_scene();
                r.w.e.mem.set_u32(r.w.water_reference + REF_CELL, 0);
                run_depth(&mut r);
                assert!(calls(&r.w.e, NODE_GET_PROPERTY).is_empty());
                // A reference without a shader property is skipped too.
                let mut r = depth_scene();
                let geometry = r.w.geometry;
                r.w.e.mem.set_u32(geometry + GEOMETRY_PROPERTY, 0);
                run_depth(&mut r);
                assert!(calls(&r.w.e, NI_POINTER_ASSIGN_FROM).is_empty());
            }

            #[test]
            fn a_form_without_the_10000000_bit_adds_no_objects() {
                let mut r = depth_scene();
                r.set_flags(0x400 | 0x800);
                let (actor, _) = reference_with_node(&mut r.w.e);
                fill_list(&mut r.w.e, r.w.group.addr() + 0x3c, &[actor]);
                run_depth(&mut r);
                // The property still takes the depth map.
                assert_eq!(
                    r.w.e.get(
                        Ptr::<WaterShaderProperty>::new(r.w.other_property),
                        WaterShaderProperty::spDepthMap
                    ),
                    0x0e00_0003
                );
                assert!(static_depth_objects(&r).is_empty());
                assert!(dynamic_depth_objects(&r).is_empty());
            }

            #[test]
            fn the_grid_cells_become_static_depth_objects_once() {
                let mut r = depth_scene();
                r.set_flags(0x1000_0800);
                add_second_reference(&mut r);
                run_depth(&mut r);
                // Cells (0, 0) and (1, 1) exist; each gives its four children, the
                // newest first in the list; the second reference adds none.
                let mut expected = vec![];
                for cell in [GRID_CELL_A, GRID_CELL_B] {
                    for i in 0..4 {
                        expected.push(cell * 16 + i);
                    }
                }
                expected.reverse();
                assert_eq!(static_depth_objects(&r), expected);
                assert!(dynamic_depth_objects(&r).is_empty());
                // Without the 0x800 bit there are none.
                let mut r = depth_scene();
                r.set_flags(0x1000_0000);
                run_depth(&mut r);
                assert!(static_depth_objects(&r).is_empty());
            }

            #[test]
            fn the_actors_in_the_water_become_dynamic_depth_objects_once() {
                let mut r = depth_scene();
                r.set_flags(0x1000_0400);
                add_second_reference(&mut r);
                let (first, first_node) = reference_with_node(&mut r.w.e);
                let (known, _) = reference_with_node(&mut r.w.e);
                let without_node = actor(&mut r.w.e, 0.0, 0.0, 0.0);
                let player = object(&mut r.w.e, ACTOR_VTABLE, 0x700);
                let player_node = node(&mut r.w.e);
                r.w.e.mem.set_u32(player + REF_NODE, player_node);
                r.w.e.set_global(PLAYER_CHARACTER, player);
                fill_list(
                    &mut r.w.e,
                    r.w.group.addr() + 0x3c,
                    &[first, known, without_node, player],
                );
                r.w.map.borrow_mut().insert(known, known);
                run_depth(&mut r);
                // The player is only listed when `fn_004eaf60` allows it.
                assert_eq!(dynamic_depth_objects(&r), vec![first_node]);
                assert_eq!(r.w.map.borrow().get(&first), Some(&first));
                assert!(!r.w.map.borrow().contains_key(&without_node));
                assert!(!r.w.map.borrow().contains_key(&player));
                // The map is the system's `DepthRefMap`.
                let queries = calls(&r.w.e, WADING_MAP_GET);
                assert_eq!(queries[0][0], r.w.system.addr() + 0x5c);
                // A player in first person is listed.
                let mut r = depth_scene();
                r.set_flags(0x1000_0400);
                let player = object(&mut r.w.e, ACTOR_VTABLE, 0x700);
                let player_node = node(&mut r.w.e);
                r.w.e.mem.set_u32(player + REF_NODE, player_node);
                r.w.e.mem.set_u8(player + 0x64a, 1);
                r.w.e.set_global(PLAYER_CHARACTER, player);
                fill_list(&mut r.w.e, r.w.group.addr() + 0x3c, &[player]);
                run_depth(&mut r);
                assert_eq!(dynamic_depth_objects(&r), vec![player_node]);
            }

            #[test]
            fn the_reflected_references_with_bit_two_become_static_depth_objects() {
                let mut r = depth_scene();
                let (first, first_node) = reference_with_node(&mut r.w.e);
                let (second, _) = reference_with_node(&mut r.w.e);
                // A chain of `{record, next}` nodes; a record is `{reference,
                // flags}`.
                let e = &mut r.w.e;
                let first_record = e.mem.alloc(8);
                e.mem.set_u32(first_record, first);
                e.mem.set_u32(first_record + 4, 2);
                let second_record = e.mem.alloc(8);
                e.mem.set_u32(second_record, second);
                e.mem.set_u32(second_record + 4, 1);
                let last = e.mem.alloc(8);
                e.mem.set_u32(last, second_record);
                let head = e.mem.alloc(8);
                e.mem.set_u32(head, first_record);
                e.mem.set_u32(head + 4, last);
                let water_reference = r.w.water_reference;
                e.mem.set_u32(water_reference + 0x44, head);
                run_depth(&mut r);
                assert_eq!(static_depth_objects(&r), vec![first_node]);
                assert_eq!(r.w.map.borrow().get(&first), Some(&first));
                assert!(!r.w.map.borrow().contains_key(&second));
            }

            #[test]
            fn the_objects_in_the_water_become_dynamic_depth_objects() {
                let mut r = depth_scene();
                let (first, first_node) = reference_with_node(&mut r.w.e);
                let (known, _) = reference_with_node(&mut r.w.e);
                let without_node = actor(&mut r.w.e, 0.0, 0.0, 0.0);
                let (last, last_node) = reference_with_node(&mut r.w.e);
                fill_list(
                    &mut r.w.e,
                    r.w.group.addr() + 0x30,
                    &[first, known, without_node, last],
                );
                r.w.map.borrow_mut().insert(known, known);
                run_depth(&mut r);
                assert_eq!(dynamic_depth_objects(&r), vec![last_node, first_node]);
                assert!(r.w.map.borrow().contains_key(&last));
                assert!(static_depth_objects(&r).is_empty());
            }

            #[test]
            fn the_list_at_011ca144_is_walked_for_forms_with_the_40000000_bit() {
                let mut r = depth_scene();
                r.set_flags(0x5000_0000);
                // The 40000000 bit makes the water geometry one level deeper.
                let geometry = r.w.geometry;
                let deeper = r.w.e.mem.u32(geometry + CHILD);
                let property = r.w.e.mem.u32(geometry + GEOMETRY_PROPERTY);
                let owner = r.w.e.mem.u32(geometry + GEOMETRY_OWNER);
                r.w.e.mem.set_u32(deeper + GEOMETRY_PROPERTY, property);
                r.w.e.mem.set_u32(deeper + GEOMETRY_OWNER, owner);
                let (first, first_node) = reference_with_node(&mut r.w.e);
                let (second, second_node) = reference_with_node(&mut r.w.e);
                let e = &mut r.w.e;
                let tail = e.mem.alloc(8);
                e.mem.set_u32(tail, second);
                e.mem.set_u32(EXTRA_DEPTH_OBJECT_LIST, first);
                e.mem.set_u32(EXTRA_DEPTH_OBJECT_LIST + 4, tail);
                run_depth(&mut r);
                assert_eq!(static_depth_objects(&r), vec![second_node, first_node]);
                assert!(r.w.map.borrow().contains_key(&first));
                assert!(r.w.map.borrow().contains_key(&second));
                // An empty list head adds nothing.
                let mut r = depth_scene();
                r.set_flags(0x5000_0000);
                let geometry = r.w.geometry;
                let deeper = r.w.e.mem.u32(geometry + CHILD);
                let property = r.w.e.mem.u32(geometry + GEOMETRY_PROPERTY);
                let owner = r.w.e.mem.u32(geometry + GEOMETRY_OWNER);
                r.w.e.mem.set_u32(deeper + GEOMETRY_PROPERTY, property);
                r.w.e.mem.set_u32(deeper + GEOMETRY_OWNER, owner);
                r.w.e.mem.set_u32(EXTRA_DEPTH_OBJECT_LIST, 0);
                r.w.e.mem.set_u32(EXTRA_DEPTH_OBJECT_LIST + 4, 0);
                run_depth(&mut r);
                assert!(static_depth_objects(&r).is_empty());
            }

            #[test]
            fn the_depth_setup_needs_the_water_shader_and_a_viewer() {
                let mut r = depth_scene();
                r.w.e.register(WATER_SHADER_ENABLED, |_, _| ret(0));
                run_depth(&mut r);
                assert!(calls(&r.w.e, NI_ALLOC).is_empty());
                assert_eq!(calls(&r.w.e, ALLOCATION_SCOPE_CONSTRUCT).len(), 1);
                assert_eq!(calls(&r.w.e, ALLOCATION_SCOPE_DESTRUCT).len(), 1);
                let mut r = depth_scene();
                r.viewer = 0;
                run_depth(&mut r);
                assert!(calls(&r.w.e, MT_ADD_ACCUM_TASK).is_empty());
            }

            // --- the depth render of a group (004ec800, 004ecb60) -----------------

            /// The finishing scene with a depth camera and sorter on the group, a
            /// water form with the floats 0.25 (+0xa4) and 0.75 (+0xa8), a
            /// refract plane at 6 and the doubles of the depth pass's states.
            fn depth_finishing() -> Finishing {
                let mut f = finishing();
                let e = &mut f.r.w.e;
                let group = f.r.w.group;
                let camera = e.mem.alloc(0x120);
                e.set(group, PlaceableWaterGroup::spDepthCamera, camera);
                e.set(group, PlaceableWaterGroup::spDepthSorter, 0x5050);
                e.set(group, PlaceableWaterGroup::iDepthThreadStage, 5);
                let water_type = e.get(group, PlaceableWaterGroup::pWaterType).addr();
                e.mem.set_f32(water_type + 0xa4, 0.25);
                e.mem.set_f32(water_type + 0xa8, 0.75);
                e.register(WATER_FORM_FLOAT_A4, |e, a| {
                    e.mem.f32(a[0] + 0xa4).into_ret()
                });
                e.register(WATER_FORM_FLOAT_A8, |e, a| {
                    e.mem.f32(a[0] + 0xa8).into_ret()
                });
                // The refract plane: normal (0, 0, 1), constant 6.
                e.mem.set_f32(group.addr() + 0x14 + 8, 1.0);
                e.mem.set_f32(group.addr() + 0x14 + 12, 6.0);
                set_float_setting(e, SETTING_REFRACTION_WATER_PLANE_BIAS, 2.0);
                quiet(
                    e,
                    &[
                        COUNTED_STATE_34,
                        SET_STENCIL_STATE,
                        RENDER_STATE_980C0,
                        RENDER_STATE_98230,
                    ],
                );
                e.set_global(RENDER_STATE_COUNTERS + 4 * 10, 5u32);
                e.set_global(RENDER_STATE_COUNTERS + 4 * 11, 0u32);
                e.set_global(RENDER_STATE_COUNTERS + 4 * 12, 3u32);
                e.set_global(COUNTER_011FF9FC, 7u32);
                f
            }

            fn run_finish_depth(f: &mut Finishing, mask: u32) {
                start_log(&mut f.r.w.e);
                let (system, group) = (f.r.w.system, f.r.w.group);
                f.r.w.e.call(0x004e_c800, &args![system, group, mask]);
            }

            #[test]
            fn the_depth_pass_publishes_its_constants_and_draws_the_group() {
                let mut f = depth_finishing();
                let group = f.r.w.group;
                let camera = f.r.w.e.get(group, PlaceableWaterGroup::spDepthCamera);
                run_finish_depth(&mut f, 0x1_0055);
                let e = &f.r.w.e;
                assert_eq!(
                    e.get(group, PlaceableWaterGroup::spDepthCamera),
                    0,
                    "the camera is dropped at the end"
                );
                let object = e.global::<u32>(RENDER_OBJECT_GLOBAL);
                assert_eq!(
                    calls(e, RENDER_OBJECT_SET_CAMERA_DATA),
                    vec![vec![object, camera + 0x100]]
                );
                assert_eq!(
                    calls(e, MT_SET_THREAD_STAGE_ONE),
                    vec![vec![MT_RENDERING_SYSTEM, 1, 6]]
                );
                // The reflect plane (0, 0, 1, 10) and the two floats of the water
                // form (+0xa4 first) are published for the shaders.
                let plane: Vec<f32> = (0..4)
                    .map(|i| e.global::<f32>(DEPTH_PLANE_SHADER_CONSTANT + 4 * i))
                    .collect();
                assert_eq!(plane, vec![0.0, 0.0, 1.0, 10.0]);
                let range: Vec<f32> = (0..2)
                    .map(|i| e.global::<f32>(DEPTH_RANGE_SHADER_CONSTANT + 4 * i))
                    .collect();
                assert_eq!(range, vec![0.25, 0.75]);
                // The render states: the mask is a 16-bit word.
                assert_eq!(calls(e, ACCUMULATOR_SET_WORD_19C), vec![vec![0x5050, 0xe]]);
                assert_eq!(calls(e, COUNTED_STATE_34), vec![vec![1, 1], vec![0, 0]]);
                assert_eq!(calls(e, SET_STENCIL_STATE), vec![vec![2, 0xff, 0x55, 1]]);
                assert_eq!(calls(e, RENDER_STATE_980C0), vec![vec![0, 0, 0, 1]]);
                assert_eq!(calls(e, RENDER_STATE_98230), vec![vec![0, 1]]);
                // The refract plane (0, 0, 1) at 6, lowered by the bias (2) to -4,
                // moved into view space (the eye is at z = 3).
                assert_eq!(
                    *f.math.clip_planes.borrow(),
                    vec![(f.math.clip_device, 0, vec![0.0, 0.0, 1.0, -1.0])]
                );
                assert_eq!(
                    calls(e, RENDER_ACCUMULATED_SCENE),
                    vec![vec![camera, 0x5050, 0]]
                );
                // The pass ends with a state on the renderer's device and the
                // counters lowered.
                assert_eq!(
                    *f.math.states.borrow(),
                    vec![vec![f.math.state_device, 0x98, 0, 0, 0]]
                );
                assert_eq!(e.global::<u32>(COUNTER_011FF9FC), 6);
                assert_eq!(e.global::<u32>(RENDER_STATE_COUNTERS + 4 * 10), 4);
                assert_eq!(e.global::<u32>(RENDER_STATE_COUNTERS + 4 * 11), 0);
                assert_eq!(e.global::<u32>(RENDER_STATE_COUNTERS + 4 * 12), 2);
                let scope = calls(e, ALLOCATION_SCOPE_CONSTRUCT);
                assert_eq!(&scope[0][1..], &[0x1d, 1, TESWATER_SOURCE_PATH, 0x1209]);
                assert_eq!(calls(e, ALLOCATION_SCOPE_DESTRUCT).len(), 1);
            }

            #[test]
            fn the_depth_pass_needs_the_water_shader_and_a_depth_camera() {
                let mut f = depth_finishing();
                f.r.w.e.register(WATER_SHADER_ENABLED, |_, _| ret(0));
                run_finish_depth(&mut f, 0x55);
                assert!(calls(&f.r.w.e, MT_SET_THREAD_STAGE_ONE).is_empty());
                assert_eq!(calls(&f.r.w.e, ALLOCATION_SCOPE_DESTRUCT).len(), 1);
                let mut f = depth_finishing();
                let group = f.r.w.group;
                f.r.w.e.set(group, PlaceableWaterGroup::spDepthCamera, 0);
                run_finish_depth(&mut f, 0x55);
                assert!(calls(&f.r.w.e, MT_SET_THREAD_STAGE_ONE).is_empty());
                assert!(f.math.states.borrow().is_empty());
            }

            /// The scene of the wading camera with the doubles of the depth
            /// camera, a viewer with a translate, scale and the float at +0xfc.
            fn depth_render_scene() -> (Wading, u32) {
                let (mut w, _log) = camera_scene();
                let viewer = w.e.mem.alloc(0x100);
                for (i, value) in [4.0f32, 5.0, 6.0].into_iter().enumerate() {
                    w.e.mem.set_f32(viewer + 0x8c + 4 * i as u32, value);
                }
                w.e.mem.set_f32(viewer + 0x98, 2.5);
                w.e.mem.set_f32(viewer + 0xfc, 3.5);
                w.e.register(NODE_SCALE_SOURCE, |e, a| e.mem.f32(a[0] + 0x98).into_ret());
                w.e.register(CAMERA_FLOAT_FC_READ, |e, a| {
                    e.mem.f32(a[0] + 0xfc).into_ret()
                });
                quiet(
                    &mut w.e,
                    &[
                        NODE_SET_LOCAL_ROTATE,
                        NODE_SET_LOCAL_SCALE,
                        CAMERA_FLOAT_FC_WRITE,
                        COUNTED_STATE_A8,
                    ],
                );
                w.e.set_global(COUNTER_011FFA18, 4u32);
                (w, viewer)
            }

            fn run_render_depth(w: &mut Wading, viewer: u32, mask: u32) {
                start_log(&mut w.e);
                let (system, group) = (w.system, w.group);
                w.e.call(0x004e_cb60, &args![system, viewer, group, 0xdeadu32, mask]);
            }

            #[test]
            fn the_depth_camera_on_the_stack_draws_the_references_in_range_at_once() {
                let (mut w, viewer) = depth_render_scene();
                run_render_depth(&mut w, viewer, 0x34);
                let camera = calls(&w.e, CAMERA_CONSTRUCT)[0][0];
                // A camera on the stack, set up like the depth camera.
                assert_eq!(*w.translations.borrow(), vec![(camera, [4.0, 5.0, 6.0])]);
                assert_eq!(
                    calls(&w.e, NODE_SET_LOCAL_ROTATE),
                    vec![vec![camera, viewer + 0x68]]
                );
                assert_eq!(
                    calls(&w.e, NODE_SET_LOCAL_SCALE),
                    vec![vec![camera, 2.5f32.to_bits()]]
                );
                assert_eq!(
                    calls(&w.e, CAMERA_FLOAT_FC_WRITE),
                    vec![vec![camera, 3.5f32.to_bits()]]
                );
                assert_eq!(
                    calls(&w.e, CAMERA_SET_VIEW_FRUSTUM),
                    vec![vec![camera, viewer + 0xdc]]
                );
                // The camera's frustum goes to the render object; the culling
                // process works for the camera and is torn down again.
                let object = w.e.global::<u32>(RENDER_OBJECT_GLOBAL);
                assert_eq!(
                    calls(&w.e, RENDER_OBJECT_SET_CAMERA_DATA),
                    vec![vec![object, camera + 0x100]]
                );
                let culling = calls(&w.e, CULLING_PROCESS_CONSTRUCT)[0][0];
                assert_eq!(
                    calls(&w.e, CULLING_PROCESS_SET_CAMERA),
                    vec![vec![culling, camera], vec![culling, 0]]
                );
                assert_eq!(
                    calls(&w.e, CULLING_PROCESS_SET_PLANES),
                    vec![vec![culling, camera + 0xdc]]
                );
                assert_eq!(calls(&w.e, CULLING_PROCESS_DESTRUCT), vec![vec![culling]]);
                assert_eq!(calls(&w.e, CAMERA_DESTRUCT), vec![vec![camera]]);
                // The reference gets the mask (and keeps it), its pass is drawn.
                assert_eq!(w.e.mem.u32(w.other_property + 0x84), 0x34);
                let request = calls(&w.e, RENDER_PASS_IMMEDIATELY);
                assert_eq!(request.len(), 1);
                assert_eq!(request[0][1..], [3, 0, 0, 0]);
                // The state counters: set on entry, lowered on exit.
                assert_eq!(calls(&w.e, COUNTED_STATE_A8), vec![vec![0, 1], vec![7, 0]]);
                assert_eq!(w.e.global::<u32>(COUNTER_011FFA18), 3);
                assert_eq!(w.e.global::<u8>(WADING_RENDER_ACTIVE_FLAG), 0);
            }

            #[test]
            fn a_reference_out_of_range_gets_the_mask_but_is_not_drawn() {
                let (mut w, viewer) = depth_render_scene();
                let top = w.e.mem.u32(w.water_reference + REF_NODE);
                w.e.mem.set_u8(top + NODE_RANGE, 0);
                run_render_depth(&mut w, viewer, 0x34);
                assert_eq!(w.e.mem.u32(w.other_property + 0x84), 0x34);
                assert!(calls(&w.e, PROPERTY_RENDER_PASS).is_empty());
                assert!(calls(&w.e, RENDER_PASS_IMMEDIATELY).is_empty());
            }

            #[test]
            fn references_with_other_owners_or_cells_are_not_touched_by_the_depth_render() {
                let (mut w, viewer) = depth_render_scene();
                let cell = w.e.mem.u32(w.water_reference + REF_CELL);
                w.e.mem.set_u8(cell + 0x26, 5);
                run_render_depth(&mut w, viewer, 0x34);
                assert_eq!(w.e.mem.u32(w.other_property + 0x84), 7);
                let (mut w, viewer) = depth_render_scene();
                let geometry = w.geometry;
                let owner = w.e.mem.u32(geometry + GEOMETRY_OWNER);
                w.e.mem.set_u32(owner + 0x68, 0x0c);
                run_render_depth(&mut w, viewer, 0x34);
                assert_eq!(w.e.mem.u32(w.other_property + 0x84), 7);
                assert!(calls(&w.e, RENDER_PASS_IMMEDIATELY).is_empty());
            }

            #[test]
            fn the_depth_render_needs_the_water_shader_and_a_viewer() {
                let (mut w, viewer) = depth_render_scene();
                w.e.register(WATER_SHADER_ENABLED, |_, _| ret(0));
                run_render_depth(&mut w, viewer, 0x34);
                assert!(calls(&w.e, CAMERA_CONSTRUCT).is_empty());
                assert!(calls(&w.e, COUNTED_STATE_A8).is_empty());
                let (mut w, _) = depth_render_scene();
                run_render_depth(&mut w, 0, 0x34);
                assert!(calls(&w.e, CAMERA_CONSTRUCT).is_empty());
                assert_eq!(w.e.global::<u32>(COUNTER_011FFA18), 4);
            }
        }
    }
}
