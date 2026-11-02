//! `fallout shared/tesscriptfunctions.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds the bodies of the script commands (script and console
//! functions). Every body is `cdecl` and receives the same eight stack
//! words, [`ScriptArgs`]; most of them hand the words to
//! `Script::ParseParameters` (`005accb0`) to read their arguments into locals
//! passed by address ([`ScriptArgs::parse_into`]). The result is `AL`: false
//! when the arguments do not parse. Bodies that never read a word
//! (`005b5b90`, `005b5cc0`, ...) take no parameters here.
//!
//! Progress: the queue entries from `005b4b20` to `005bc3b0` (160 functions,
//! the whole range `00000000` to `005bc430` of this file) are translated.
//!
//! Notes on the exe's code that the translations rely on:
//! - Several tiny accessors are called by address and named here after what
//!   they do, not after the engine map (the linker folded identical code, so
//!   the map often carries another class's name: `007af430` is
//!   `*(this + 0x20)`, the base form of a reference, although the map calls
//!   it `BGSSaveFormBuffer::GetForm`).
//! - The console print `00703c00` is `printf`-like: format address first,
//!   `double` arguments take two words.
//! - Virtual slots are named by byte offset, exactly as the code indexes the
//!   vtable; the Xbox PDB name of a slot is only given where the PC
//!   behaviour matches it.
//! - The decompiler often attaches pushes to the wrong call: a `PUSH` before a
//!   call that is followed by no stack clean-up belongs to the next callee
//!   (`RET n`), and a call written with arguments in C can be a call without
//!   (`004e3270` takes none, `00b8cb00` takes the pushed word). Argument
//!   order here is always read from the disassembly.
//! - The compiler's exception-unwinding frames are not translated.

#[allow(unused_imports)]
use crate::prelude::*;

// ---- The eight words of a script command body ----------------------------

/// The eight stack words of a script command body (`cdecl`, in the order the
/// caller pushes them: the first is the one at `[ebp+8]`). Field names follow
/// how the bodies use them: `this_obj` is the reference the command runs on,
/// `script_obj` the running script (its name is printed by `005b4be0`) and
/// `result` the `double` a function-style command writes its value to;
/// `param_info`, `script_data`, `containing_obj`, `event_list` and
/// `opcode_offset` are only passed on to `Script::ParseParameters` (and
/// `containing_obj` is a second reference in `005b53d0` and `005b5860`).
#[derive(Clone, Copy, Debug)]
pub struct ScriptArgs {
    /// `[ebp+0x08]`.
    pub param_info: u32,
    /// `[ebp+0x0c]`.
    pub script_data: u32,
    /// `[ebp+0x10]`.
    pub this_obj: Ptr,
    /// `[ebp+0x14]`.
    pub containing_obj: Ptr,
    /// `[ebp+0x18]`.
    pub script_obj: Ptr,
    /// `[ebp+0x1c]`.
    pub event_list: u32,
    /// `[ebp+0x20]`.
    pub result: Ptr,
    /// `[ebp+0x24]`.
    pub opcode_offset: u32,
}

impl Arg for ScriptArgs {
    const WORDS: usize = 8;
    fn take(words: &[u32], i: &mut usize) -> Self {
        ScriptArgs {
            param_info: u32::take(words, i),
            script_data: u32::take(words, i),
            this_obj: Ptr::take(words, i),
            containing_obj: Ptr::take(words, i),
            script_obj: Ptr::take(words, i),
            event_list: u32::take(words, i),
            result: Ptr::take(words, i),
            opcode_offset: u32::take(words, i),
        }
    }
    fn put(self, out: &mut Vec<u32>) {
        self.param_info.put(out);
        self.script_data.put(out);
        self.this_obj.put(out);
        self.containing_obj.put(out);
        self.script_obj.put(out);
        self.event_list.put(out);
        self.result.put(out);
        self.opcode_offset.put(out);
    }
}

impl ScriptArgs {
    /// `Script::ParseParameters` (`005accb0`) with the given output addresses
    /// after the seven fixed words: its `AL`.
    pub(crate) fn parse(self, e: &mut Engine, outs: &[u32]) -> bool {
        let mut words = args![
            self.param_info,
            self.script_data,
            self.opcode_offset,
            self.this_obj,
            self.containing_obj,
            self.script_obj,
            self.event_list
        ];
        words.extend_from_slice(outs);
        e.call(PARSE_PARAMETERS, &words).bool()
    }

    /// [`ScriptArgs::parse`] with `N` word-sized locals (the stack slots the
    /// game passes by address) initialised to `init`. `None` when the
    /// parameters do not parse, otherwise the values left in the locals.
    pub(crate) fn parse_into<const N: usize>(
        self,
        e: &mut Engine,
        init: [u32; N],
    ) -> Option<[u32; N]> {
        let block = e.mem.alloc(4 * N as u32);
        let mut outs = [0u32; N];
        for (i, value) in init.iter().enumerate() {
            outs[i] = block + 4 * i as u32;
            e.mem.set_u32(outs[i], *value);
        }
        let ok = self.parse(e, &outs);
        let mut values = init;
        for (i, value) in values.iter_mut().enumerate() {
            *value = e.mem.u32(outs[i]);
        }
        e.mem.free(block);
        ok.then_some(values)
    }
}

// ---- Callees outside the unit (by exe address) -----------------------------

/// `Script::ParseParameters` (Xbox PDB), `cdecl`, varargs.
pub(crate) const PARSE_PARAMETERS: u32 = 0x005a_ccb0;
/// The console print: format address, then the arguments (`cdecl`).
pub(crate) const CONSOLE_PRINT: u32 = 0x0070_3c00;
/// `Script::GetDistanceConditionFunction` (Xbox PDB), `cdecl`
/// (`thisObj, object, 0, result`).
pub(crate) const GET_DISTANCE_CONDITION: u32 = 0x0059_bfa0;
/// `Script::GetInZoneConditionFunction` (Xbox PDB), `cdecl`, same arguments.
pub(crate) const GET_IN_ZONE_CONDITION: u32 = 0x0059_c010;
/// `Script::GetItemCountConditionFunction` (Xbox PDB), `cdecl`
/// (`actor, item, 0, double* result`).
pub(crate) const GET_ITEM_COUNT_CONDITION: u32 = 0x0059_d8e0;
/// `__RTDynamicCast` (`object, 0, source type, target type, 0`).
pub(crate) const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptor of `TESObjectREFR` (`.?AVTESObjectREFR@@`).
pub(crate) const RTTI_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// RTTI type descriptor of `Actor` (`.?AVActor@@`).
pub(crate) const RTTI_ACTOR: u32 = 0x0118_46d4;
/// `_ftol2_sse`: the `double` in `ST0` (a leading `f64` argument) truncated
/// into `EAX`.
pub(crate) const FTOL: u32 = 0x00ec_62c0;
/// `_memset` (`dest, value, count`).
pub(crate) const MEMSET: u32 = 0x00ec_61c0;
/// `_strlen`.
pub(crate) const STRLEN: u32 = 0x00ec_6130;
/// `strcpy_s` (`dest, size, source`).
pub(crate) const STRCPY_S: u32 = 0x0040_6d30;
/// `strcat_s` (`dest, size, source`).
pub(crate) const STRCAT_S: u32 = 0x0040_6d50;
/// `sprintf_s` (`dest, size, format, ...`).
pub(crate) const SPRINTF_S: u32 = 0x0040_6d00;
/// `strncpy`-like copy that terminates (`dest, source, count`).
pub(crate) const STRNCPY_S: u32 = 0x004a_dd50;
/// `strstr`-like search (`haystack, needle`): non-zero when found.
pub(crate) const FIND_SUBSTRING: u32 = 0x0048_12f0;
/// `eh_vector_constructor_iterator` (`array, size, count, constructor,
/// destructor`).
pub(crate) const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
/// `eh_vector_destructor_iterator` (`array, size, count, destructor`).
pub(crate) const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;

/// Form type byte (`this + 4`, `movzx`).
pub(crate) const FORM_TYPE: u32 = 0x0040_1170;
/// `*(this + 0x20)`: the base form of a reference.
pub(crate) const GET_BASE_FORM: u32 = 0x007a_f430;
/// `this + 0x44`: the extra data list of a reference.
pub(crate) const EXTRA_DATA_LIST: u32 = 0x005d_43c0;
/// `ExtraDataList::GetContainerChanges` (Xbox PDB).
pub(crate) const GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
/// `InventoryChanges::GetInventoryChanges` (Xbox PDB), `cdecl` (`reference`).
pub(crate) const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `InventoryChanges::WearingObject` (Xbox PDB), `thiscall` (`item, 0`).
pub(crate) const WEARING_OBJECT: u32 = 0x004b_fda0;
/// `ExtraDataList::SetCanNotWear` (Xbox PDB), `thiscall` (`0`).
pub(crate) const SET_CAN_NOT_WEAR: u32 = 0x0041_ab70;
/// `InventoryChanges::DuplicateAllItems` (Xbox PDB), `thiscall`
/// (`reference, target`).
pub(crate) const DUPLICATE_ALL_ITEMS: u32 = 0x004c_d9c0;
/// `thiscall` on the inventory changes (`reference, item, 0, flag, 0, flag,
/// slot, extra`); returns a `float` in `ST0` that the caller discards.
pub(crate) const FN_004CE340: u32 = 0x004c_e340;
/// Same shape as [`FN_004CE340`] (`reference, item, 0, flag, 0, flag, -1,
/// 0`).
pub(crate) const FN_004CE380: u32 = 0x004c_e380;
/// `thiscall` on the inventory changes (`form`): the entry for the form.
pub(crate) const FN_004CBA70: u32 = 0x004c_ba70;
/// `thiscall` on an actor (`form`): whether the actor has the form.
pub(crate) const FN_00575400: u32 = 0x0057_5400;
/// `thiscall` on an extra data list (`form`).
pub(crate) const FN_00419700: u32 = 0x0041_9700;
/// `cdecl` (`reference, form, 0, result`).
pub(crate) const FN_005A2E20: u32 = 0x005a_2e20;
/// `PlayerCharacter::CheckForQuestTargetUpdate` (Xbox PDB), `thiscall` on
/// the player (`reference`).
pub(crate) const CHECK_QUEST_TARGET_UPDATE: u32 = 0x0095_2c30;
/// `Interface::IsInMenuMode` (Xbox PDB).
pub(crate) const IS_IN_MENU_MODE: u32 = 0x0070_2360;
/// Interface function the item commands call after the player's inventory
/// changed (`cdecl`, no arguments).
pub(crate) const FN_00704AF0: u32 = 0x0070_4af0;
/// `thiscall` on the base form of a reference (`reference, extra, 1`).
pub(crate) const FN_00606540: u32 = 0x0060_6540;
/// `TESContainer::TESContainer` (Xbox PDB): a 0x10-byte object.
pub(crate) const TES_CONTAINER_CONSTRUCT: u32 = 0x0048_1610;
/// Destructor of the 0x10-byte container object.
pub(crate) const TES_CONTAINER_DESTRUCT: u32 = 0x0048_1680;
/// `thiscall` on `leveled list + 0x30` (`level, count, container, 0`).
pub(crate) const FN_00487F70: u32 = 0x0048_7f70;
/// `thiscall` on the container object (`item, count, 0`).
pub(crate) const FN_004818E0: u32 = 0x0048_18e0;
/// `thiscall` on the container object (`float`).
pub(crate) const FN_00482090: u32 = 0x0048_2090;
/// `thiscall` on the container object (`reference, flag`).
pub(crate) const FN_004821A0: u32 = 0x0048_21a0;
/// `TESObjectREFR::GetCalcLevel` (Xbox PDB), `thiscall` (`0`).
pub(crate) const GET_CALC_LEVEL: u32 = 0x0056_7e10;
/// Signed byte at `this + 0xf4`.
pub(crate) const FN_00446390: u32 = 0x0044_6390;
/// `TESObjectREFR::GetActionRef` (Xbox PDB).
pub(crate) const GET_ACTION_REF: u32 = 0x0057_2e30;
/// `thiscall` (`action`): whether the reference has the action set.
pub(crate) const HAS_ACTION: u32 = 0x0057_2d30;
/// `thiscall` (`action`): sets the action.
pub(crate) const SET_ACTION: u32 = 0x0057_2d50;
/// `TESObjectREFR::ClearAction` (Xbox PDB), `thiscall` (`action`).
pub(crate) const CLEAR_ACTION: u32 = 0x0057_2db0;
/// `TESObjectREFR::Activate` (Xbox PDB), `thiscall` (`activator, 0, 0, 1`).
pub(crate) const ACTIVATE: u32 = 0x0057_3170;
/// Tests form flag `0x800` (`this + 8`).
pub(crate) const FN_00440DA0: u32 = 0x0044_0da0;
/// `thiscall` on the item list element: the actor's `+0x94` member
/// (`item, 1`).
pub(crate) const FN_008248E0: u32 = 0x0082_48e0;
/// `MiddleHighProcess::GetSavedAcquireObject` (Xbox PDB), `thiscall`.
pub(crate) const MIDDLE_HIGH_PROCESS_SAVED_ACQUIRE: u32 = 0x008d_8520;
/// Interface message with icon (`cdecl`: `text, 0, icon path, sound, float,
/// 0`).
pub(crate) const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// `Actor::GetPickUpSoundName` (Xbox PDB), `thiscall` on the player
/// (`item, 0, 0`).
pub(crate) const GET_PICK_UP_SOUND_NAME: u32 = 0x008a_dcf0;
/// `TESFullName::GetFullName` (Xbox PDB), `cdecl` (`form`).
pub(crate) const GET_FULL_NAME: u32 = 0x0048_2720;
/// `this + 0x10` of the data handler (`00460140`): the world space list.
pub(crate) const DATA_HANDLER_WORLD_SPACES: u32 = 0x0046_0140;
/// `this + 0x40` of the data handler (`0087eaa0`): the spell list.
pub(crate) const DATA_HANDLER_SPELLS: u32 = 0x0087_eaa0;
/// `TESWorldSpace::BuildMapMarkerList` (Xbox PDB), `thiscall` (`0`): returns a
/// freshly allocated list.
pub(crate) const BUILD_MAP_MARKER_LIST: u32 = 0x0058_82a0;
/// `TESObjectREFR::GetMapMarkerData` (Xbox PDB).
pub(crate) const GET_MAP_MARKER_DATA: u32 = 0x0056_9060;
/// `MapMarkerData::GetHidden` (Xbox PDB).
pub(crate) const MAP_MARKER_GET_HIDDEN: u32 = 0x0043_8f10;
/// `MapMarkerData::GetTravelLoc` (Xbox PDB).
pub(crate) const MAP_MARKER_GET_TRAVEL_LOC: u32 = 0x0043_8ef0;
/// `MapMarkerData::SetVisible` (Xbox PDB), `thiscall` (`bool`).
pub(crate) const MAP_MARKER_SET_VISIBLE: u32 = 0x0044_de40;
/// `MapMarkerData::SetTravelLoc` (Xbox PDB), `thiscall` (`bool`).
pub(crate) const MAP_MARKER_SET_TRAVEL_LOC: u32 = 0x0044_de80;
/// `ProcessLists::PrintLists` (Xbox PDB), `thiscall` (`detection, -1`).
pub(crate) const PROCESS_LISTS_PRINT_LISTS: u32 = 0x008d_0600;
/// Empty `thiscall` on the process lists singleton.
pub(crate) const FN_00483710: u32 = 0x0048_3710;
/// `PlayerCharacter::SetGodMode` (Xbox PDB), `cdecl` (`bool`).
pub(crate) const SET_GOD_MODE: u32 = 0x0095_26a0;
/// `PlayerCharacter::IsGodMode` (Xbox PDB).
pub(crate) const IS_GOD_MODE: u32 = 0x0095_26b0;
/// `PlayerCharacter::SetDemigodMode` (Xbox PDB), `cdecl` (`bool`).
pub(crate) const SET_DEMIGOD_MODE: u32 = 0x0095_26e0;
/// `PlayerCharacter::IsDemigodMode` (Xbox PDB).
pub(crate) const IS_DEMIGOD_MODE: u32 = 0x0095_26f0;

// Linked list nodes: item at +0, next node at +4 (`BSSimpleList`).
/// `true` when the node has neither an item nor a next node.
pub(crate) const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// Returns `this`: the address of the node's item.
pub(crate) const LIST_ITEM_PTR: u32 = 0x0068_15c0;
/// `*(this + 4)`: the next node (the same code reads the reference count of
/// an `NiRefObject`, whose field is also at +4).
pub(crate) const LIST_NEXT: u32 = 0x0072_6070;
/// Pulls the next node's content into the head and frees that node.
pub(crate) const LIST_POP_FRONT: u32 = 0x0063_f7b0;
/// Number of nodes with a non-null item (`thiscall`).
pub(crate) const LIST_COUNT: u32 = 0x005a_e380;
/// Initialises an empty list head (a two-word object).
pub(crate) const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `thiscall` on the list head: appends the item whose *cell address* is
/// passed.
pub(crate) const LIST_APPEND: u32 = 0x005a_e3d0;
/// Destructor of the list head.
pub(crate) const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// Scalar deleting destructor of a heap list (`this, 1`).
pub(crate) const LIST_DELETE: u32 = 0x0047_02f0;
/// `this + 0x18`: the item list embedded in a form list.
pub(crate) const FORM_LIST_ITEMS: u32 = 0x0050_0940;

// Strings.
/// `BSStringT` constructor (an 8-byte object).
pub(crate) const BS_STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// `BSStringT` destructor.
pub(crate) const BS_STRING_DESTRUCT: u32 = 0x0040_37d0;
/// `BSStringT` format (`string, format, ...`).
pub(crate) const BS_STRING_FORMAT: u32 = 0x0040_6f60;
/// `BSStringT<char>::operator=` / append (`string, text`).
pub(crate) const BS_STRING_APPEND: u32 = 0x0040_4820;
/// `*(this + 4)` of a string global: its text.
pub(crate) const BS_STRING_TEXT: u32 = 0x0040_3df0;
/// `NiPointer::operator T*` and `BSStringT::c_str`: `*this`.
pub(crate) const NI_POINTER_GET: u32 = 0x0055_9450;

// Memory statistics.
/// Constructs the 0x18-byte memory heap dump file object (`file name`).
pub(crate) const MEM_STATS_FILE_CONSTRUCT: u32 = 0x0087_83c0;
/// Its destructor.
pub(crate) const MEM_STATS_FILE_DESTRUCT: u32 = 0x0087_84d0;
/// `cdecl` (`dump object, flag`): returns the output object (global `011f6238`).
pub(crate) const MEM_STATS_OUTPUT_GET: u32 = 0x0040_1020;
/// `thiscall` on the output object (its `RET 8` pops the two words the
/// previous call left).
pub(crate) const MEM_STATS_OUTPUT_SET: u32 = 0x00aa_47c0;
/// `BSFile::BSFile` (Xbox PDB), `thiscall` (`path, 1, 0x4000, 0`).
pub(crate) const BS_FILE_CONSTRUCT: u32 = 0x00b0_0260;
/// `BSFile::Open` (Xbox PDB), `thiscall` (`0, 0`).
pub(crate) const BS_FILE_OPEN: u32 = 0x00af_f300;
/// `BSFile::Close` (Xbox PDB).
pub(crate) const BS_FILE_CLOSE: u32 = 0x00af_fd10;
/// `BSFile::~BSFile` (Xbox PDB).
pub(crate) const BS_FILE_DESTRUCT: u32 = 0x00af_f240;
/// `thiscall` on a `BSFile` (`buffer, size`): writes bytes.
pub(crate) const BS_FILE_WRITE: u32 = 0x00af_f950;
/// `ModelLoader::OutputModelMapContents` (Xbox PDB), `thiscall`
/// (`file, 0`).
pub(crate) const MODEL_LOADER_OUTPUT_MODEL_MAP: u32 = 0x0044_3270;
/// `BSFaceGenManager::GetModelCache` (Xbox PDB).
pub(crate) const GET_MODEL_CACHE: u32 = 0x0065_2110;
/// `BSFaceGenModelMap::OutputModelMapContents` (Xbox PDB), `thiscall`
/// (`file`).
pub(crate) const FACE_GEN_OUTPUT_MODEL_MAP: u32 = 0x0064_fe80;
/// `NiPointer` constructor passed to the vector iterator.
pub(crate) const NI_POINTER_CONSTRUCT: u32 = 0x0066_94e0;
/// `NiPointer` destructor passed to the vector iterator.
pub(crate) const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// Zero-fills (`array, 0, bytes`).
pub(crate) const ZERO_FILL: u32 = 0x0040_3d30;
/// First texture of the global list.
pub(crate) const FIRST_TEXTURE: u32 = 0x0045_6510;
/// `this + 0x2c`: the next texture.
pub(crate) const NEXT_TEXTURE: u32 = 0x0055_b980;
/// `this + 0x24`: the texture's Direct3D texture.
pub(crate) const GET_D3D_TEXTURE: u32 = 0x0059_bb30;
/// `NiPointer::operator=` (`to`, `from`).
pub(crate) const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// Assigns a raw pointer to an `NiPointer` (`slot, pointer`).
pub(crate) const NI_POINTER_SET: u32 = 0x0066_b0d0;
/// `cdecl` (`class, texture`): whether the texture is of the class.
pub(crate) const TEXTURE_IS_SOURCE: u32 = 0x0043_b300;
/// `thiscall`: the C string of a name handle.
pub(crate) const NAME_TEXT: u32 = 0x0043_b1b0;
/// `this + 8`.
pub(crate) const FN_00413F40: u32 = 0x0041_3f40;
/// `cdecl` (`text, size`): fixes the path in place.
pub(crate) const FIX_PATH: u32 = 0x004a_fb00;
/// `this + 0x14`.
pub(crate) const FN_007D6BB0: u32 = 0x007d_6bb0;
/// `cdecl` (`surface`): the Direct3D format id.
pub(crate) const D3D_FORMAT_ID: u32 = 0x00e7_bd20;
/// `NiXenonRenderer::GetD3DFormatString` (Xbox PDB), `cdecl`
/// (`format id`).
pub(crate) const GET_D3D_FORMAT_STRING: u32 = 0x00e7_a9a0;
/// `thiscall` (`texture`): 3 for the textures flagged "L".
pub(crate) const TEXTURE_KIND: u32 = 0x0043_c430;
/// `BGSTextureUseMap` constructor (a 0x14-byte object).
pub(crate) const TEXTURE_USE_MAP_CONSTRUCT: u32 = 0x004a_8f50;
/// `thiscall` (`node, 1`): collects the textures below a scene node.
pub(crate) const TEXTURE_USE_MAP_ADD_NODE: u32 = 0x004a_a3d0;
/// `thiscall` (`1`): collects the textures of the whole scene.
pub(crate) const TEXTURE_USE_MAP_ADD_SCENE: u32 = 0x004a_abb0;
/// `BGSTextureUseMap::WriteToFile` (Xbox PDB), `thiscall` (`path`).
pub(crate) const TEXTURE_USE_MAP_WRITE_TO_FILE: u32 = 0x004a_a900;
/// Sum of the sizes of the textures in the map.
pub(crate) const TEXTURE_USE_MAP_TOTAL_SIZE: u32 = 0x004a_a890;
/// `BGSTextureUseMap` destructor.
pub(crate) const TEXTURE_USE_MAP_DESTRUCT: u32 = 0x004a_a340;
/// Archive profiling (`BSSystem/archive.cpp`), compiled to `XOR AL,AL` in
/// this build.
pub(crate) const ARCHIVE_PROFILE_START: u32 = 0x00af_4390;

// Process and memory level singletons.
/// Singleton getter (loads the global `011f96a0`).
pub(crate) const FN_0044F560: u32 = 0x0044_f560;
/// Singleton getter (loads the global `011f35a0`).
pub(crate) const FN_004DE490: u32 = 0x004d_e490;
/// `this + 4` (`BaseProcess::GetActorPackageThatIsRunning` in the engine
/// map).
pub(crate) const PLUS_4: u32 = 0x0071_7e50;
/// `thiscall` (`package`).
pub(crate) const FN_005E01B0: u32 = 0x005e_01b0;
/// `cdecl` (`scope, 1, 1, 1`): initialises a 12-byte level scope.
pub(crate) const FN_00878160: u32 = 0x0087_8160;
/// `cdecl` (`scope`): tears the scope down.
pub(crate) const FN_00878200: u32 = 0x0087_8200;
/// `MemoryLevelManager::FreeReleasedObjects` (Xbox PDB), `cdecl`
/// (`level byte`).
pub(crate) const FREE_RELEASED_OBJECTS: u32 = 0x0087_8250;
/// `thiscall` on the singleton [`GLOBAL_0011DEA10`] (`1, 0`).
pub(crate) const FN_004539A0: u32 = 0x0045_39a0;
/// `thiscall` on the save/load singleton (`0, 0`).
pub(crate) const FN_00848E70: u32 = 0x0084_8e70;

// ---- Globals ---------------------------------------------------------------

/// The `PlayerCharacter` singleton pointer.
pub(crate) const PLAYER: u32 = 0x011d_ea3c;
/// Pointer to the data handler singleton.
pub(crate) const DATA_HANDLER: u32 = 0x011c_3f2c;
/// Singleton passed as `this` to `ProcessLists::PrintLists` and `00483710`.
pub(crate) const PROCESS_LISTS: u32 = 0x011e_0e80;
/// Pointer to the model loader (non-null once created).
pub(crate) const MODEL_LOADER: u32 = 0x011c_3b3c;
/// Texture class object passed to [`TEXTURE_IS_SOURCE`].
pub(crate) const TEXTURE_CLASS: u32 = 0x011f_444c;
/// Pointer to the singleton whose byte at `+1` `005b6cb0` sets.
pub(crate) const GLOBAL_0011DEA0C: u32 = 0x011d_ea0c;
/// Pointer to the singleton `005b6cd0` calls `004539a0` on.
pub(crate) const GLOBAL_0011DEA10: u32 = 0x011d_ea10;
/// Pointer to the save/load singleton.
pub(crate) const SAVE_LOAD_GAME: u32 = 0x011d_df38;
/// Byte set by `005b5d40`.
pub(crate) const FLAG_0011F122D: u32 = 0x011f_122d;
/// Byte set by `005b5da0`.
pub(crate) const FLAG_0011F9FC0: u32 = 0x011f_9fc0;
/// Byte toggled by `Script::ToggleOcclusion`.
pub(crate) const OCCLUSION_QUERY: u32 = 0x011f_9181;
/// Byte of `005b6f60` / `005b6f70` (verbose messages).
pub(crate) const VERBOSE_MESSAGES: u32 = 0x011f_158c;
/// `float` the item commands pass as the pick-up message volume.
pub(crate) const PICKUP_VOLUME: u32 = 0x0101_62c0;
/// `float` `005b4be0` passes to `00482090`.
pub(crate) const ADD_ITEM_FLOAT: u32 = 0x0102_31e0;
/// Byte at `+0x268` of the TLS block: commands echo to the console when set.
pub(crate) const TLS_ECHO: u32 = 0x268;
/// Dword at `+0x28c` of the TLS block: nesting depth of the Activate command.
pub(crate) const TLS_ACTIVATE_DEPTH: u32 = 0x28c;
/// An Activate nested deeper than this does nothing.
pub(crate) const ACTIVATE_DEPTH_LIMIT: u32 = 5;
/// The player's list of message parts: the texts pasted into the "item
/// removed" message (`BSStringT`-like globals whose text is at +4).
pub(crate) const MESSAGE_PART_A: u32 = 0x011d_3db4;
pub(crate) const MESSAGE_PART_B: u32 = 0x011d_3114;

/// Size of a `BSFile` object (`BSFile::BSFile`).
pub(crate) const BS_FILE_SIZE: u32 = 0x160;
/// Number of slots in `OutputMemStats`'s sorted texture table.
pub(crate) const TEXTURE_SLOTS: u32 = 0x800;

// ---- String literals (addresses in the exe's data) ---------------------------

/// `"SCRIPTS: AddItem in script '%s' failed to generate an item."`
pub(crate) const MSG_ADD_ITEM_FAILED: u32 = 0x0103_9358;
/// `"SCRIPTS: Never have the player character activate something in a script very Bad"`
pub(crate) const MSG_PLAYER_ACTIVATE: u32 = 0x0103_93b0;
/// `"IsActionRef >> %0.2f"`
pub(crate) const MSG_IS_ACTION_REF: u32 = 0x0103_9394;
/// `"Command removed. Use TDT and switch to COMBAT INFO page."`
pub(crate) const MSG_COMBAT_STATS_REMOVED: u32 = 0x0103_9404;
/// `"%d spells added to Player Character"`
pub(crate) const MSG_SPELLS_ADDED: u32 = 0x0103_9440;
/// `"Detection list printed"`
pub(crate) const MSG_DETECTION_LIST_PRINTED: u32 = 0x0103_9464;
/// `"Ai Lists Printed"`
pub(crate) const MSG_AI_LISTS_PRINTED: u32 = 0x0103_947c;
/// `"Deprecated, use TFC 2 instead"`
pub(crate) const MSG_FREEZE_RENDERER_DEPRECATED: u32 = 0x0103_9490;
/// `"Occlusion Query : %s"`
pub(crate) const MSG_OCCLUSION_QUERY: u32 = 0x0103_94b0;
/// `"off"`
pub(crate) const TEXT_OFF: u32 = 0x0103_94c8;
/// `"on"`
pub(crate) const TEXT_ON: u32 = 0x0101_22c8;
/// `"OutputMemContexts is no longer supported.  Use Sherlock or OutputMemStats for context info."`
pub(crate) const MSG_MEM_CONTEXTS_UNSUPPORTED: u32 = 0x0103_94d0;
/// `"Bye."`
pub(crate) const MSG_BYE: u32 = 0x0103_97a4;
/// `"shown."`
pub(crate) const TEXT_SHOWN: u32 = 0x0103_97c8;
/// `"hidden."`
pub(crate) const TEXT_HIDDEN: u32 = 0x0103_97c0;
/// `"Verbose messages %s"`
pub(crate) const MSG_VERBOSE: u32 = 0x0103_97d0;
/// `"All map markers %s"`
pub(crate) const MSG_MAP_MARKERS: u32 = 0x0103_97ac;
/// `"God Mode %s"`
pub(crate) const MSG_GOD_MODE: u32 = 0x0103_97e4;
/// `"Demigod Mode %s"`
pub(crate) const MSG_DEMIGOD_MODE: u32 = 0x0103_9820;
/// `"Demigod Mode disabled"`
pub(crate) const MSG_DEMIGOD_DISABLED: u32 = 0x0103_9808;
/// `"enabled."`
pub(crate) const TEXT_ENABLED: u32 = 0x0103_97fc;
/// `"disabled."`
pub(crate) const TEXT_DISABLED: u32 = 0x0103_97f0;
/// `".txt"`
pub(crate) const EXTENSION_TXT: u32 = 0x0103_9788;
/// `"ArchiveProfile.txt"`
pub(crate) const DEFAULT_ARCHIVE_PROFILE: u32 = 0x0103_9790;
/// `"Outputting Archive profile to file %s"`
pub(crate) const MSG_ARCHIVE_PROFILE_OUTPUT: u32 = 0x0103_9760;
/// `"Archive profiling is not enabled"`
pub(crate) const MSG_ARCHIVE_PROFILE_DISABLED: u32 = 0x0103_973c;
/// `"TextureUseMap.txt"`
pub(crate) const DEFAULT_TEXTURE_USE_MAP: u32 = 0x0103_9728;
/// `"FAILED to write to '%s'"`
pub(crate) const MSG_MAP_WRITE_FAILED: u32 = 0x0103_96f4;
/// `"map written to file '%s'"`
pub(crate) const MSG_MAP_WRITTEN: u32 = 0x0103_970c;
/// `"%d MB"`
pub(crate) const FORMAT_MB: u32 = 0x0103_96ec;
/// `"%d KB"`
pub(crate) const FORMAT_KB: u32 = 0x0103_96e4;
/// `"%d bytes"`
pub(crate) const FORMAT_BYTES: u32 = 0x0103_96d8;
/// `" textures in use."`
pub(crate) const TEXT_TEXTURES_IN_USE: u32 = 0x0103_96c4;
/// `"OutputMemStats.mhd"`
pub(crate) const DEFAULT_MEM_STATS: u32 = 0x0103_96b0;
/// `"MemStats%s.mhd"`
pub(crate) const FORMAT_MEM_STATS: u32 = 0x0103_96a0;
/// `"ModelDump%s.txt"`
pub(crate) const FORMAT_MODEL_DUMP: u32 = 0x0103_9690;
/// `"TextureDump%s.txt"`
pub(crate) const FORMAT_TEXTURE_DUMP: u32 = 0x0103_967c;
/// `"UNNAMED SOURCE TEXTURE"`
pub(crate) const TEXT_UNNAMED_SOURCE_TEXTURE: u32 = 0x0103_9664;
/// `"RENDERED_TEXTURE (%s)"`
pub(crate) const FORMAT_RENDERED_TEXTURE: u32 = 0x0103_964c;
/// `"RENDERED_TEXTURE (none)"`
pub(crate) const TEXT_RENDERED_TEXTURE_NONE: u32 = 0x0103_9634;
/// `"TEXTURES: %d:\t%s\t%s\t%dx%d\t%d\t%s\trefcount %d\t%s\t%s\r\n"`
pub(crate) const FORMAT_TEXTURE_LINE: u32 = 0x0103_952c;
/// `"L"`
pub(crate) const TEXT_L: u32 = 0x0103_9564;
/// `"H"`
pub(crate) const TEXT_H: u32 = 0x0103_9560;
/// `"%i %s%s %s"`
pub(crate) const FORMAT_ITEM_COUNT_MESSAGE: u32 = 0x0101_c178;
/// `"%s %s"`
pub(crate) const FORMAT_ITEM_MESSAGE: u32 = 0x0101_2058;
/// `"Interface\Icons\Message Icons\glow_message_vaultboy_neutral.dds"`
pub(crate) const ICON_VAULT_BOY: u32 = 0x0102_08e0;
// The path fragments `OutputMemStats` groups textures by.
/// `"\armor\"`
pub(crate) const PATH_ARMOR: u32 = 0x0103_9624;
/// `"\creatures\"`
pub(crate) const PATH_CREATURES: u32 = 0x0103_9618;
/// `"\characters\"`
pub(crate) const PATH_CHARACTERS: u32 = 0x0103_9608;
/// `"\pipboy3000\"`
pub(crate) const PATH_PIPBOY: u32 = 0x0103_95f8;
/// `"\weapons\"`
pub(crate) const PATH_WEAPONS: u32 = 0x0103_95ec;
/// `"\decals\"`
pub(crate) const PATH_DECALS: u32 = 0x0103_95e0;
/// `"\projectiles\"`
pub(crate) const PATH_PROJECTILES: u32 = 0x0103_95d0;
/// `"\fonts\"`
pub(crate) const PATH_FONTS: u32 = 0x0103_95c0;
/// `"\interface\"`
pub(crate) const PATH_INTERFACE: u32 = 0x0103_95b4;
/// `"\effects\"`
pub(crate) const PATH_EFFECTS: u32 = 0x0103_95a0;
/// `"\gore\"`
pub(crate) const PATH_GORE: u32 = 0x0103_9598;
/// `"\sky\"`
pub(crate) const PATH_SKY: u32 = 0x0103_9588;
/// `"\lod\"`
pub(crate) const PATH_LOD: u32 = 0x0103_957c;
/// `"\water\"`
pub(crate) const PATH_WATER: u32 = 0x0103_9570;
/// `"SYSTEM"`
pub(crate) const CATEGORY_SYSTEM: u32 = 0x0103_962c;
/// `"ACTOR"`
pub(crate) const CATEGORY_ACTOR: u32 = 0x0103_95c8;
/// `"MENUS"`
pub(crate) const CATEGORY_MENUS: u32 = 0x0103_95ac;
/// `"EFFECTS"`
pub(crate) const CATEGORY_EFFECTS: u32 = 0x0103_9590;
/// `"SKY"`
pub(crate) const CATEGORY_SKY: u32 = 0x0103_9584;
/// `"LOD"`
pub(crate) const CATEGORY_LOD: u32 = 0x0103_9578;
/// `"WATER"`
pub(crate) const CATEGORY_WATER: u32 = 0x0102_3014;
/// `"MISCREF"`
pub(crate) const CATEGORY_MISCREF: u32 = 0x0103_9568;

// ---- Second batch (`005b7070` onward): callees, globals and strings --------------
//
// Offsets named "PC offset" are the ones the PC code reads; the Xbox layouts
// of `TESObjectREFR` and `TESObjectCELL` differ from the PC ones.

/// `Script::SetProcessScripts` (Xbox PDB), `cdecl` (`bool`).
pub(crate) const SET_PROCESS_SCRIPTS: u32 = 0x005a_c730;
/// `Script::GetProcessScripts` (Xbox PDB).
pub(crate) const GET_PROCESS_SCRIPTS: u32 = 0x005a_c740;

// Grass display.
/// `cdecl`, no arguments: loads the global `011ca438`, the object the grass
/// toggle works on (zero when there is none).
pub(crate) const FN_0054F4C0: u32 = 0x0054_f4c0;
/// `thiscall` on that object: the flag the toggle reads (calls
/// `00456630(this, 1)`).
pub(crate) const FN_00456610: u32 = 0x0045_6610;
/// `thiscall` on that object (`flag`): calls `0043b370(this, flag, 1)`.
pub(crate) const FN_00450F90: u32 = 0x0045_0f90;
/// `cdecl`, no arguments: run when the flag is set.
pub(crate) const FN_00B62A20: u32 = 0x00b6_2a20;
/// `cdecl`, no arguments: loads the global `011deb7c`.
pub(crate) const FN_0045C670: u32 = 0x0045_c670;
/// `BSFaceGenNiNode::GetAnimationData` (engine map name; the body reads the
/// pointer at `this + 0xac`).
pub(crate) const FN_006629F0: u32 = 0x0066_29f0;
/// `thiscall`, no stack word: returns `this + 0x8c`, the address of a vector.
pub(crate) const FN_0045BB80: u32 = 0x0045_bb80;
/// `cdecl` (seven words, ADD ESP,0x1c): the three words of that vector, the three
/// words of the start vector and a `float`.
pub(crate) const FN_0057D0A0: u32 = 0x0057_d0a0;

// Cell tests and the `TES` singleton (`GLOBAL_0011DEA10`).
/// `TES::TestAllCells` (Xbox PDB), `thiscall` (`cell count or -1`).
pub(crate) const TES_TEST_ALL_CELLS: u32 = 0x0045_56d0;
/// `thiscall` on the `TES`: `bRunningCellTests` or `bRunningCellTests2`
/// (`+0x51`, `+0x52`).
pub(crate) const TES_IS_RUNNING_CELL_TESTS: u32 = 0x0045_1530;
/// `thiscall` on a reference: the parent cell (PC offset `+0x40`).
pub(crate) const REFR_GET_PARENT_CELL: u32 = 0x008d_6f30;
/// `TESObjectCELL::RenderTestCell` (Xbox PDB), `cdecl` (`cell, 0, flag`).
pub(crate) const CELL_RENDER_TEST_CELL: u32 = 0x0055_7dd0;
/// `thiscall` on the `TES`, no arguments.
pub(crate) const FN_00456A60: u32 = 0x0045_6a60;
/// `thiscall` on the `TES`, no arguments.
pub(crate) const FN_00456AA0: u32 = 0x0045_6aa0;
/// `thiscall` on the `TES` (`flag`): sets `bShowLANDborders` (`+0x60`) and
/// passes it on.
pub(crate) const TES_SET_SHOW_LAND_BORDERS: u32 = 0x0045_6cb0;
/// `thiscall` on the `TES` (`name`): appends `".hkx"` (`01017cec`) to the
/// name and hands the path to `00c67430`.
pub(crate) const FN_00456B10: u32 = 0x0045_6b10;
/// `*(this + 0x34)` (the engine map names it
/// `ActorMover::GetPreferredMoveMode`; the code is shared).
pub(crate) const FN_005F36F0: u32 = 0x005f_36f0;

// Ini refresh.
/// `cdecl`, no arguments: the address `01202fa0`.
pub(crate) const FN_004DC110: u32 = 0x004d_c110;
/// `thiscall` on an ini collection singleton (`path`): calls
/// `005e0830(this, path, 0)` and, when that succeeds, virtual slots `0x24`
/// and `0x1c`.
pub(crate) const FN_005E0200: u32 = 0x005e_0200;

// Sky and weather.
/// `Sky::GetInstance` (Xbox PDB), `cdecl`.
pub(crate) const SKY_GET_INSTANCE: u32 = 0x0046_dd00;
/// `*(sky + 0x0c)`: `pCurrentClimate` (Xbox PDB, same on PC).
pub(crate) const SKY_GET_CURRENT_CLIMATE: u32 = 0x0084_e3a0;
/// `*(sky + 0x10)`: `pCurrentWeather` (Xbox PDB, same on PC); the engine map
/// names it `BaseProcess::GetCurrentProcedureIndex` (identical code).
pub(crate) const SKY_GET_CURRENT_WEATHER: u32 = 0x0044_edb0;
/// `TESForm::GetFile` (Xbox PDB), `thiscall` (`-1`).
pub(crate) const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESFile::GetThreadSafeFile` (Xbox PDB), `thiscall`.
pub(crate) const FILE_GET_THREAD_SAFE_FILE: u32 = 0x0047_39b0;
/// `TESFile::OpenTES` (Xbox PDB), `thiscall` (`0, 0`).
pub(crate) const FILE_OPEN_TES: u32 = 0x0047_0c70;
/// `TESFile::FindForm` (Xbox PDB), `thiscall` (`form`).
pub(crate) const FILE_FIND_FORM: u32 = 0x0047_34d0;
/// `Sky::SetCurrentClimate` (Xbox PDB), `thiscall` (`climate, 1`).
pub(crate) const SKY_SET_CURRENT_CLIMATE: u32 = 0x0063_c8f0;
/// `Sky::ForceWeather` (Xbox PDB), `thiscall` (`weather, flag`).
pub(crate) const SKY_FORCE_WEATHER: u32 = 0x0063_d0e0;
/// `thiscall` on the player (`0`), run after the default weather was set.
pub(crate) const FN_0093A7A0: u32 = 0x0093_a7a0;
/// `thiscall` on the sky (`1`), run after a weather was set.
pub(crate) const FN_0063E860: u32 = 0x0063_e860;
/// Stores 0 at `this + 0x1c` (`pOverrideWeather`); the engine map names it
/// `MagicHitEffect::ClearTarget` (identical code).
pub(crate) const SKY_CLEAR_OVERRIDE_WEATHER: u32 = 0x0081_bc50;
/// `pDefaultWeather` in `Sky` (Xbox PDB, same on PC).
pub(crate) const SKY_DEFAULT_WEATHER: u32 = 0x18;
/// `pOverrideWeather` in `Sky` (Xbox PDB, same on PC).
pub(crate) const SKY_OVERRIDE_WEATHER: u32 = 0x1c;

// Image space modifiers.
/// `ImageSpaceModifierInstanceForm::Trigger` (Xbox PDB), `cdecl` (`form,
/// strength, 0`).
pub(crate) const IMAGE_SPACE_MODIFIER_TRIGGER: u32 = 0x0052_99a0;
/// `ImageSpaceModifierInstanceForm::Stop` (Xbox PDB), `cdecl` (`form`).
pub(crate) const IMAGE_SPACE_MODIFIER_STOP: u32 = 0x0052_9c90;
/// `thiscall` (`argument`): calls `004610d0(this, argument)` and
/// `0041c290` on its result.
pub(crate) const FN_00547750: u32 = 0x0054_7750;
/// `cdecl`, no arguments: loads the global `011f91ac`.
pub(crate) const GET_IMAGE_SPACE_MANAGER: u32 = 0x004e_3270;
/// `thiscall` on that object (`form`): stores the form at `this + 0xb0`.
pub(crate) const IMAGE_SPACE_MANAGER_SET_OVERRIDE: u32 = 0x00b8_cb00;
/// `thiscall` on that object, no arguments.
pub(crate) const FN_00B8B500: u32 = 0x00b8_b500;

// Allocation and construction.
/// `operator new` (`MemoryManager::Allocate`), `cdecl` (`size`).
pub(crate) const OPERATOR_NEW: u32 = 0x0040_1000;
/// `NiMemObject::operator new`, `cdecl` (`size`).
pub(crate) const NI_ALLOC: u32 = 0x00aa_13e0;
/// Allocates an array of `short`s, `cdecl` (`bytes`).
pub(crate) const NI_ALLOC_SHORTS: u32 = 0x00aa_1070;
/// `NiNode::NiNode(capacity)`, `thiscall` (`0`).
pub(crate) const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// The array constructor loop, `cdecl` (`array, element size, count,
/// constructor`): calls the constructor (`thiscall`) on every element.
pub(crate) const VECTOR_CONSTRUCT_SIMPLE: u32 = 0x0040_1050;
/// A trivial constructor: returns `this` (the same code as
/// [`LIST_ITEM_PTR`]).
pub(crate) const TRIVIAL_CONSTRUCT: u32 = 0x0068_15c0;
/// `NiPoint3::NiPoint3` (`x, y, z`), `thiscall`, returns `this`.
pub(crate) const NI_POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// `NiColorA::NiColorA` (`r, g, b, a`), `thiscall`, returns `this`.
pub(crate) const NI_COLOR_A_CONSTRUCT: u32 = 0x0041_4430;
/// The constructor of one `NiColorA` element: `NiColorA(0, 0, 0, 0)`.
pub(crate) const NI_COLOR_A_ZERO_CONSTRUCT: u32 = 0x004a_7800;
/// `NiPoint2::NiPoint2` (`u, v`), `thiscall`, returns `this`.
pub(crate) const NI_POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
/// `thiscall` (`angle`): fills the 36-byte matrix at `this` with the
/// rotation about z by `angle`.
pub(crate) const NI_MATRIX_FROM_Z_ANGLE: u32 = 0x004a_0c90;
/// `thiscall` on a node (`matrix`): copies the 36 bytes to `this + 0x34`.
pub(crate) const NODE_SET_LOCAL_ROTATE: u32 = 0x0043_fa80;
/// `thiscall` on a node (`vector`): copies 12 bytes to `this + 0x58`.
pub(crate) const NODE_SET_LOCAL_TRANSLATE: u32 = 0x0044_0460;
/// `thiscall` on a node (`x, y, z`): stores the vector at `this + 0x58`.
pub(crate) const NODE_SET_LOCAL_TRANSLATE_XYZ: u32 = 0x004b_c1f0;
/// `thiscall` on a node (`update data`): calls its virtual slot `0xa4`
/// (`data, 0`) and, when `this + 0x18` is set, slot `0xfc` of that object.
pub(crate) const NODE_UPDATE_WITH_DATA: u32 = 0x00a5_9c60;
/// `NiAVObject::UpdateProperties` (Xbox PDB), `thiscall`.
pub(crate) const NODE_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// Constructor of a 9-byte object, `thiscall` (`float`, `byte`, `byte`).
pub(crate) const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
/// `TES::AddTempDebugObject` (Xbox PDB), `thiscall` on the `TES` (`node,
/// seconds`).
pub(crate) const TES_ADD_TEMP_DEBUG_OBJECT: u32 = 0x0045_8e20;
/// `NiTriShape::NiTriShape` (Xbox PDB), `thiscall` (`vertex count,
/// positions, normals, colours, uvs, 1, 0, triangle count, indices`).
pub(crate) const NI_TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4410;
/// `NiTexturingProperty` constructor, `thiscall`.
pub(crate) const NI_TEXTURING_PROPERTY_CONSTRUCT: u32 = 0x00a6_aa40;
/// `thiscall` on the texturing property (`0`): sets a value of its base map
/// (the engine map names it `NiTexturingProperty::SetBaseClampMode`).
pub(crate) const NI_TEXTURING_PROPERTY_SET_BASE_CLAMP_MODE: u32 = 0x004f_3200;
/// `thiscall` on the texturing property (`2`): calls
/// `00439360(this, 2, 0xe, 1)`.
pub(crate) const FN_00533FB0: u32 = 0x0053_3fb0;
/// `thiscall` on a shape (`property`): attaches the property (engine map
/// name `NiAVObject::AttachProperty`).
pub(crate) const NI_AV_OBJECT_ATTACH_PROPERTY: u32 = 0x0043_9410;
/// `NiPointer` constructor taking the pointer, `thiscall` (`pointer`).
pub(crate) const NI_POINTER_INIT: u32 = 0x0063_3c90;
/// `MakeTriangle` (Xbox PDB), `cdecl` (three vectors by value, `colour`,
/// `1`): a coloured triangle node.
pub(crate) const MAKE_TRIANGLE: u32 = 0x004b_3570;
/// Address of element `index` (`base + index * 4`) of the array at `this`,
/// `thiscall` (`index`).
pub(crate) const ARRAY_ELEMENT_ADDRESS: u32 = 0x0087_7a30;
/// `thiscall` on an array (`index, &value`): stores the element.
pub(crate) const ARRAY_SET_ELEMENT: u32 = 0x0096_ae90;
/// Constructor of a 0x10-byte object (the map of a texturing property),
/// `thiscall`.
pub(crate) const TEXTURE_MAP_CONSTRUCT: u32 = 0x00a6_9dd0;
/// `thiscall` on that object (`texture`): stores the texture through
/// `NI_POINTER_SET`.
pub(crate) const TEXTURE_MAP_SET_TEXTURE: u32 = 0x004d_c540;

// Cells, references and the player.
/// `TESObjectREFR::GetInterior` (Xbox PDB), `thiscall`.
pub(crate) const REFR_GET_INTERIOR: u32 = 0x0057_5d10;
/// `TESObjectREFR::GetWorldSpace` (Xbox PDB), `thiscall`.
pub(crate) const REFR_GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// `TESWorldSpace::GetCellFromCellCoord` (Xbox PDB), `thiscall` (`x, y`).
pub(crate) const WORLD_SPACE_GET_CELL_FROM_CELL_COORD: u32 = 0x0058_75a0;
/// The seen data of a cell (`ExtraDataList::GetSeenData` of its extra data
/// list), `thiscall` on the cell.
pub(crate) const CELL_GET_SEEN_DATA: u32 = 0x0055_5bc0;
/// `TESObjectCELL::AdjustCoordForNorthRotation` (Xbox PDB), `thiscall`
/// (`source vector, destination vector, 1`).
pub(crate) const CELL_ADJUST_COORD_FOR_NORTH_ROTATION: u32 = 0x0055_5b10;
/// `TESObjectCELL::GetIntSeenSection` (Xbox PDB), `thiscall` (`x, y, 0`).
pub(crate) const CELL_GET_INT_SEEN_SECTION: u32 = 0x0055_6ef0;
/// `TESObjectCELL::GetInteriorLocalMapTexture` (Xbox PDB), `thiscall` (`x,
/// y, texture slot`).
pub(crate) const CELL_GET_INTERIOR_LOCAL_MAP_TEXTURE: u32 = 0x0054_e750;
/// `thiscall` on a cell (`texture slot`).
pub(crate) const FN_0054E640: u32 = 0x0054_e640;
/// `TESObjectCELL::GetSeenValue` (Xbox PDB), `cdecl` (`vector`): an `int`.
pub(crate) const CELL_GET_SEEN_VALUE: u32 = 0x0055_6870;
/// `TESObjectCELL::GetDataX` (Xbox PDB), `thiscall`.
pub(crate) const CELL_GET_DATA_X: u32 = 0x0054_4c30;
/// `TESObjectCELL::GetDataY` (Xbox PDB), `thiscall`.
pub(crate) const CELL_GET_DATA_Y: u32 = 0x0054_4c60;
/// Bit 0 of the byte at `+0x24` (PC offset): the cell is an interior.
pub(crate) const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
/// `float` result in `ST0`: the north rotation of an interior cell, 0 for an
/// exterior.
pub(crate) const CELL_GET_NORTH_ROTATION: u32 = 0x0055_5ad0;
/// `thiscall` on the `TES` (`x, y`): the address of the cell slot of a grid
/// position.
pub(crate) const TES_GET_GRID_CELL_SLOT: u32 = 0x0045_7050;
/// Virtual slot of the player that returns the address of its position
/// vector.
pub(crate) const PLAYER_POSITION_SLOT: u32 = 0x1f4;
/// Virtual slot of the seen data that draws it (`node, vector, 1`).
pub(crate) const SEEN_DATA_DRAW_SLOT: u32 = 0x4;
/// Virtual slot of a node that attaches a child (`child, 1`).
pub(crate) const NODE_ATTACH_CHILD_SLOT: u32 = 0xdc;
/// `thiscall` on the player: `this + 0x24` (PC offset).
pub(crate) const FN_00430830: u32 = 0x0043_0830;
/// Float to `int`, `cdecl` (`float`), rounding like `FISTP`.
pub(crate) const FLOAT_TO_INT: u32 = 0x0040_6d90;
/// `float` result in `ST0`: the global `011a31e8`.
pub(crate) const GET_LOCAL_MAP_SCALE: u32 = 0x0087_9d90;
/// `thiscall` on a vector (`other`): adds `other` in place.
pub(crate) const POINT3_ADD_ASSIGN: u32 = 0x0063_c8a0;
/// `thiscall` on a vector (`out, reference`): `out = this - reference`.
pub(crate) const POINT3_SUBTRACT: u32 = 0x0043_9ef0;
/// The `uGridsToLoad` setting (`Setting` object at `011c63cc`).
pub(crate) const SETTING_GRIDS_TO_LOAD: u32 = 0x011c_63cc;
/// Address of the value of an integer setting (`this + 4`), `thiscall`.
pub(crate) const SETTING_INT_VALUE_ADDRESS: u32 = 0x0043_d4d0;
/// `cdecl` (`&a, &b, 1, 0`): fills two `float`s; the second is a size in
/// bytes.
pub(crate) const FN_004A8BB0: u32 = 0x004a_8bb0;

// The interface.
/// `Interface::ToggleFullHelp` (Xbox PDB), `cdecl`.
pub(crate) const INTERFACE_TOGGLE_FULL_HELP: u32 = 0x0070_3100;
/// `Interface::GetFullHelp` (Xbox PDB), `cdecl`.
pub(crate) const INTERFACE_GET_FULL_HELP: u32 = 0x0070_3150;
/// `Interface::ToggleSafeZone` (Xbox PDB), `cdecl` (`2`).
pub(crate) const INTERFACE_TOGGLE_SAFE_ZONE: u32 = 0x0070_38e0;
/// `cdecl` (`flag`).
pub(crate) const FN_00703810: u32 = 0x0070_3810;
/// `MobileObject::GetCharController` (Xbox PDB), `thiscall`.
pub(crate) const MOBILE_OBJECT_GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `bhkCharacterController::SetShapeType` (Xbox PDB), `thiscall` (`type`).
pub(crate) const CHAR_CONTROLLER_SET_SHAPE_TYPE: u32 = 0x00c7_0830;

// Globals of the second batch.
/// Byte: `1` when the grass display is on.
pub(crate) const GRASS_DISPLAY_FLAG: u32 = 0x0118_c000;
/// Three words (a vector) in the data section.
pub(crate) const START_VECTOR: u32 = 0x011f_426c;
/// Byte: the NPC facial emotions flag.
pub(crate) const EMOTIONS_FLAG: u32 = 0x0119_b4e0;
/// Pointer to the name of `Fallout.ini`.
pub(crate) const FALLOUT_INI_NAME: u32 = 0x011a_2ff0;
/// Pointer to the name of `Custom.ini`.
pub(crate) const CUSTOM_INI_NAME: u32 = 0x011a_2ff4;
/// Pointer to the name of `FalloutPrefs.ini`.
pub(crate) const FALLOUT_PREFS_INI_NAME: u32 = 0x011a_2ff8;
/// Byte: the conversation stats flag.
pub(crate) const CONVERSATION_STATS_FLAG: u32 = 0x011c_beac;
/// Byte: the magic stats flag.
pub(crate) const MAGIC_STATS_FLAG: u32 = 0x011c_3534;
/// Byte set to 1 while `005b9240` runs `00456a60`.
pub(crate) const FLAG_012680FC: u32 = 0x0126_80fc;
/// Byte: the borders flag.
pub(crate) const BORDERS_FLAG: u32 = 0x011c_ae44;
/// Byte: the projectile debug flag.
pub(crate) const PROJECTILE_DEBUG_FLAG: u32 = 0x011f_20a4;
/// Byte: the menus flag.
pub(crate) const MENUS_FLAG: u32 = 0x0118_c6f0;
/// Four words: a colour.
pub(crate) const DEFAULT_TILE_COLOUR: u32 = 0x011a_9be0;
/// Nine words: a 3x3 matrix.
pub(crate) const MATRIX_011A9448: u32 = 0x011a_9448;
/// `double` constants in `.rdata`: 0.0, -1.0, 2.0, 4.0, 10.0, 4096.0,
/// 1048576.0.
pub(crate) const DOUBLE_ZERO: u32 = 0x0101_2060;
pub(crate) const DOUBLE_MINUS_ONE: u32 = 0x0101_a6b0;
pub(crate) const DOUBLE_TWO: u32 = 0x0101_1590;
pub(crate) const DOUBLE_FOUR: u32 = 0x0101_db80;
pub(crate) const DOUBLE_TEN: u32 = 0x0102_0758;
pub(crate) const DOUBLE_4096: u32 = 0x0101_7a10;
pub(crate) const DOUBLE_MEGABYTE: u32 = 0x0101_ece0;
/// `float` constants: the seconds a debug object stays, and the corners of
/// the player marker triangle.
pub(crate) const DEBUG_OBJECT_SECONDS: u32 = 0x0101_8f5c;
pub(crate) const MARKER_TOP_Y: u32 = 0x0101_6088;
pub(crate) const MARKER_RIGHT_X: u32 = 0x0101_6248;
pub(crate) const MARKER_BOTTOM: u32 = 0x0102_295c;

// Strings of the second batch.
/// `"Script processing %s"`
pub(crate) const MSG_SCRIPT_PROCESSING: u32 = 0x0103_9830;
/// `"Grass Display %s"`
pub(crate) const MSG_GRASS_DISPLAY: u32 = 0x0103_9848;
/// `"Disabled."`
pub(crate) const TEXT_DISABLED_CAPITAL: u32 = 0x0103_985c;
/// `"Enabled."`
pub(crate) const TEXT_ENABLED_CAPITAL: u32 = 0x0103_9868;
/// `"TestAllCells %s"`
pub(crate) const MSG_TEST_ALL_CELLS: u32 = 0x0103_9874;
/// `"stopped"`
pub(crate) const TEXT_STOPPED: u32 = 0x0103_9884;
/// `"running"`
pub(crate) const TEXT_RUNNING: u32 = 0x0103_988c;
/// `"RenderTestCell failed: no cell"`
pub(crate) const MSG_RENDER_TEST_CELL_FAILED: u32 = 0x0103_9894;
/// `"RenderTestCell complete. Check warnings file for more info."`
pub(crate) const MSG_RENDER_TEST_CELL_COMPLETE: u32 = 0x0103_98b4;
/// `"The in-game settings have been refreshed from the Fallout.ini file."`
pub(crate) const MSG_INI_REFRESHED: u32 = 0x0103_98f0;
/// `"NPC Facial Emotions %s"`
pub(crate) const MSG_NPC_EMOTIONS: u32 = 0x0103_9934;
/// `"M# for loaded area = %.0f MB"`
pub(crate) const MSG_LOADED_AREA_MEGABYTES: u32 = 0x0103_994c;
/// `"Conversation stats %s"`
pub(crate) const MSG_CONVERSATION_STATS: u32 = 0x0103_996c;
/// `"Toggle Full Help %s"`
pub(crate) const MSG_TOGGLE_FULL_HELP: u32 = 0x0103_9984;
/// `"Magic stats %s"`
pub(crate) const MSG_MAGIC_STATS: u32 = 0x0103_9998;
/// `"Borders -> %s"`
pub(crate) const MSG_BORDERS: u32 = 0x0103_99a8;
/// `"Off"`
pub(crate) const TEXT_OFF_CAPITAL: u32 = 0x0103_99b8;
/// `"On"`
pub(crate) const TEXT_ON_CAPITAL: u32 = 0x0103_99bc;
/// `"Debug lines/shapes now show for projectiles."`
pub(crate) const MSG_PROJECTILE_DEBUG_1: u32 = 0x0103_9a88;
/// `"    -Yellow line for targeting."`
pub(crate) const MSG_PROJECTILE_DEBUG_2: u32 = 0x0103_9a68;
/// `"    -Red diamond for spawn point."`
pub(crate) const MSG_PROJECTILE_DEBUG_3: u32 = 0x0103_9a44;
/// `"    -Blue diamond for non-supersonic/non-hit-scan projectile sound start."`
pub(crate) const MSG_PROJECTILE_DEBUG_4: u32 = 0x0103_99f8;
/// `"    -Teal diamond for near miss sound from supersonic."`
pub(crate) const MSG_PROJECTILE_DEBUG_5: u32 = 0x0103_99c0;
/// `"Menus -> %s"`
pub(crate) const MSG_MENUS: u32 = 0x0103_9ab8;

// ---- Third batch (`005b9540` onward): callees, globals and strings ------------

/// The `pSky` member of the `TES` (`TES::pSky`, Xbox PDB, `+0x68`, the same on
/// PC): `*(this + 0x68)`. The engine map names this body
/// `MiddleHighProcess::GetSavedAcquireObject` (identical code).
pub(crate) const TES_GET_SKY: u32 = 0x008d_8520;
/// `Sky::SetMode` (Xbox PDB), `thiscall` on the sky (`mode`).
pub(crate) const SKY_SET_MODE: u32 = 0x0063_a3f0;
/// `thiscall` on a cell: tests bit `0x80` of the byte at `this + 0x24` (PC
/// offset).
pub(crate) const CELL_TEST_FLAG_80: u32 = 0x0045_4b10;
/// `thiscall` on a form: tests bit `0x10` of the flags dword at `this + 8`.
pub(crate) const FORM_TEST_FLAG_10: u32 = 0x0050_d4a0;
/// `thiscall` on a form (`set`): sets or clears bit `0x10` of the flags dword
/// at `this + 8`.
pub(crate) const FORM_SET_FLAG_10: u32 = 0x0069_3ef0;
/// `thiscall` on the `TES`, no arguments: flips the byte [`FN_00456C70`]
/// reads and passes the new value on to the setters that apply it.
pub(crate) const FN_00456BE0: u32 = 0x0045_6be0;
/// `cdecl`, no arguments: the byte at `011c3c0d` (the global collision flag).
pub(crate) const FN_00456C70: u32 = 0x0045_6c70;
/// `thiscall` on a reference, no arguments: the text of its name (the
/// `%s` of the messages).
pub(crate) const REFR_GET_NAME_TEXT: u32 = 0x0055_d520;

// Tree leaves.
/// `cdecl`, no arguments: `0051d740()` (a byte setting) and the byte at `+0x18`
/// of the object [`FN_00664840`] returns.
pub(crate) const FN_0054EE20: u32 = 0x0054_ee20;
/// `cdecl` (`create`): the singleton in the global `011d5c48`, created when it
/// is missing and `create` is non-zero.
pub(crate) const FN_00664840: u32 = 0x0066_4840;

// Wireframe.
/// `cdecl` (`index`): the dword at `011f91c8 + index * 4`.
pub(crate) const FN_00450B80: u32 = 0x0045_0b80;

// Refraction, falloff and motion blur.
/// `cdecl`, no arguments: the byte at `011f9180`.
pub(crate) const FN_004DC0A0: u32 = 0x004d_c0a0;
/// `cdecl` (`flag`): the dword at `011f91bc` when `flag` is non-zero,
/// otherwise the one at `011f91c0`.
pub(crate) const FN_004DC060: u32 = 0x004d_c060;
/// `cdecl` (`a, b`), result in `ST0`: `b` when `b <= a` (or the two are
/// unordered), otherwise `a`.
pub(crate) const FLOAT_MIN: u32 = 0x0040_ebd0;
/// `cdecl` (`a, b`), result in `ST0`: `a` when `b < a`, otherwise `b`.
pub(crate) const FLOAT_MAX: u32 = 0x0040_4010;
/// `BSShaderPPLightingProperty::SetRefractionRecurse` (Xbox PDB), `cdecl`
/// (`property, enable, power, fire flag, fire period, 0`).
pub(crate) const SET_REFRACTION_RECURSE: u32 = 0x00b6_8770;
/// `BSShaderNoLightingProperty::SetFalloffRecurse` (Xbox PDB), `cdecl`
/// (`property, falloff colour`).
pub(crate) const SET_FALLOFF_RECURSE: u32 = 0x00b6_fa40;
/// `BSShaderProperty::ToggleMotionBlurRecurse` (Xbox PDB), `cdecl`
/// (`property`).
pub(crate) const TOGGLE_MOTION_BLUR_RECURSE: u32 = 0x00ba_90a0;
/// `thiscall` on a four-float colour (`out, scalar`): `out = *this * scalar`,
/// returns `out`.
pub(crate) const COLOUR_SCALE: u32 = 0x0053_2f60;
/// `cdecl` (`float`), result in `ST0`: a conversion of an angle in radians
/// (forwards to `004e4490`, which calls the runtime library on a `double`).
pub(crate) const FN_004E4470: u32 = 0x004e_4470;
/// Virtual slot of an `Actor` (`enable, power`): sets the refraction of the
/// actor.
pub(crate) const ACTOR_SET_REFRACTION_SLOT: u32 = 0x384;
/// Virtual slot of a reference with no arguments: the object whose shader
/// properties the commands change (and whose velocity `SetVel` changes).
pub(crate) const REFR_GET_TARGET_SLOT: u32 = 0x1d0;
/// Virtual slot of a reference with no arguments: the text of its name.
pub(crate) const REFR_NAME_SLOT: u32 = 0x130;

// Image space values (`004e3270` is the image space manager getter above).
/// `thiscall` on the singleton [`GET_IMAGE_SPACE_MANAGER`] returns (`index`):
/// the element at `index` of the array at `this + 4`
/// (`*[ARRAY_ELEMENT_ADDRESS](this + 4, index)`).
pub(crate) const IMAGE_SPACE_ARRAY_GET: u32 = 0x004e_bbc0;
/// Takes nothing and does nothing but return 1 in `AL`.
pub(crate) const FN_005D4A40: u32 = 0x005d_4a40;

// Inverse kinematics and animation flags.
/// `thiscall` (`this`): `this ? this + 4 : address of a static zero byte`, the
/// address of the value byte of a global object.
pub(crate) const GLOBAL_VALUE_ADDRESS: u32 = 0x0040_8d60;
/// The global object whose value byte `00495580` reads: the foot IK flag.
pub(crate) const FOOT_IK_OBJECT: u32 = 0x0126_7c30;
/// The global object of the grab IK flag.
pub(crate) const GRAB_IK_OBJECT: u32 = 0x0126_7c3c;
/// The global object of the look IK flag.
pub(crate) const LOOK_IK_OBJECT: u32 = 0x0126_7c48;
/// `cdecl`, no arguments: the value byte of [`FOOT_IK_OBJECT`].
pub(crate) const FOOT_IK_GET: u32 = 0x0049_5580;
/// `cdecl`, no arguments: the value byte of the global object `01267c24`
/// (the ragdoll animation flag).
pub(crate) const RAGDOLL_ANIM_GET: u32 = 0x0055_24d0;
/// `cdecl` (`value`): sets the value byte of that object.
pub(crate) const RAGDOLL_ANIM_SET: u32 = 0x0045_6c90;
/// `TESHavokUtilities::AddVelocity` (Xbox PDB), `cdecl` (`object, velocity,
/// 1`).
pub(crate) const ADD_VELOCITY: u32 = 0x0062_b8d0;
/// Offset in an `Actor` (PC) of the pointer to the object that holds the foot,
/// grab and look IK flags.
pub(crate) const ACTOR_IK_DATA: u32 = 0xac;

// Globals of the third batch.
/// Byte: the sky flag.
pub(crate) const SKY_FLAG: u32 = 0x011c_ae45;
/// Byte: the wireframe flag.
pub(crate) const WIREFRAME_FLAG: u32 = 0x011c_ae46;
/// Byte: the AI detection stats flag.
pub(crate) const DETECTION_STATS_FLAG: u32 = 0x011f_1222;
/// Dword set by `ToggleDetectionStats`.
pub(crate) const DETECTION_STATS_VALUE: u32 = 0x011f_1224;
/// Byte flipped by `005ba6b0`.
pub(crate) const FLAG_011F9FC2: u32 = 0x011f_9fc2;
/// Byte: the Lite Brite flag.
pub(crate) const LITE_BRITE_FLAG: u32 = 0x011f_91a7;
/// The four-float colour `SetTargetFalloff` starts from (`this` of
/// [`COLOUR_SCALE`]).
pub(crate) const FALLOFF_BASE_COLOUR: u32 = 0x011a_9be0;
/// `float` `-1.0`: the scalar applied to that colour, and so the "unset"
/// value of each channel.
pub(crate) const FLOAT_MINUS_ONE: u32 = 0x0101_2054;
/// `float` `90.0`: the largest angle of the falloff angles.
pub(crate) const FALLOFF_MAX_ANGLE: u32 = 0x0102_49d8;
/// `double` `pi / 180`.
pub(crate) const DOUBLE_DEGREES_TO_RADIANS: u32 = 0x0102_3128;

// Strings of the third batch.
/// `"Sky -> %s"`
pub(crate) const MSG_SKY: u32 = 0x0103_9ac4;
/// `"Collision -> %s"`
pub(crate) const MSG_COLLISION: u32 = 0x0103_9ad0;
/// `"Ref '%s' Collision -> %s"`
pub(crate) const MSG_REF_COLLISION: u32 = 0x0103_9ae0;
/// `"Leaves -> %s"`
pub(crate) const MSG_LEAVES: u32 = 0x0103_9afc;
/// `"Wireframe -> %s"`
pub(crate) const MSG_WIREFRAME: u32 = 0x0103_9b0c;
/// `"AI Detection stats printing is  %s"`
pub(crate) const MSG_DETECTION_STATS: u32 = 0x0103_9b1c;
/// `"%s refraction has been set to %f"`
pub(crate) const MSG_REFRACTION_SET: u32 = 0x0103_9b40;
/// `"%s refraction fire has been set to %f, period of %d"`
pub(crate) const MSG_REFRACTION_FIRE_SET: u32 = 0x0103_9b64;
/// `"No Reference Selected"`
pub(crate) const MSG_NO_REFERENCE: u32 = 0x0103_9b98;
/// `"This function is no longer supported"`
pub(crate) const MSG_NO_LONGER_SUPPORTED: u32 = 0x0103_9bb0;
/// `"FootIK on REF %s %s"`
pub(crate) const MSG_FOOT_IK_REF: u32 = 0x0103_9bd8;
/// `"FootIK %s"`
pub(crate) const MSG_FOOT_IK: u32 = 0x0103_9bec;
/// `"GrabIK on REF %s %s"`
pub(crate) const MSG_GRAB_IK_REF: u32 = 0x0103_9bf8;
/// `"GrabIK %s"`
pub(crate) const MSG_GRAB_IK: u32 = 0x0103_9c0c;
/// `"LookIK on REF %s %s"`
pub(crate) const MSG_LOOK_IK_REF: u32 = 0x0103_9c18;
/// `"LookIK %s"`
pub(crate) const MSG_LOOK_IK: u32 = 0x0103_9c2c;
/// `"Ragdoll Animation %s"`
pub(crate) const MSG_RAGDOLL_ANIMATION: u32 = 0x0103_9c38;
/// `"Lite Brite -> %s"`
pub(crate) const MSG_LITE_BRITE: u32 = 0x0103_9c50;

// ---- Small helpers -------------------------------------------------------------

pub(crate) fn console_print(e: &mut Engine, words: &[u32]) {
    e.call(CONSOLE_PRINT, words);
}

/// The item of a list node (`*LIST_ITEM_PTR(node)`).
pub(crate) fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(LIST_ITEM_PTR, &args![node]).u32();
    e.mem.u32(slot)
}

pub(crate) fn list_is_empty(e: &mut Engine, node: u32) -> bool {
    e.call(LIST_IS_EMPTY, &args![node]).bool()
}

pub(crate) fn form_type(e: &mut Engine, form: u32) -> u32 {
    e.call(FORM_TYPE, &args![form]).u32()
}

pub(crate) fn ni_pointer_get(e: &mut Engine, slot: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![slot]).u32()
}

pub(crate) fn player(e: &Engine) -> u32 {
    e.global::<u32>(PLAYER)
}

/// Appends `element` to the list head `list`: `005ae3d0` takes the address of
/// a cell holding the element (the game's stack local).
pub(crate) fn list_append(e: &mut Engine, list: Ptr, element: u32) {
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), element);
        e.call(LIST_APPEND, &args![list, cell]);
    });
}

/// Whether the commands echo their result to the console: the byte at
/// `+0x268` of the TLS block.
pub(crate) fn echo_enabled(e: &mut Engine) -> bool {
    let tls = e.tls();
    e.mem.u8(tls + TLS_ECHO) != 0
}

/// An `int` stored in a `float` the way `FILD` then `FSTP` does it.
pub(crate) fn float_from_int(value: i32) -> f32 {
    value as f32
}

/// `NiPoint3::NiPoint3(x, y, z)`: builds the point in a 12-byte block of its
/// own and returns its three words.
pub(crate) fn make_point(e: &mut Engine, x: f32, y: f32, z: f32) -> [u32; 3] {
    e.with_stack(12, |e, block| {
        let point = e.call(NI_POINT3_CONSTRUCT, &args![block, x, y, z]).u32();
        [e.mem.u32(point), e.mem.u32(point + 4), e.mem.u32(point + 8)]
    })
}

/// `new NiNode(0)`: the allocation (0xac bytes) and the constructor, zero
/// when the allocation failed.
pub(crate) fn new_ni_node(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOC, &args![0xacu32]).u32();
    if block == 0 {
        0
    } else {
        e.call(NI_NODE_CONSTRUCT, &args![block, 0u32]).u32()
    }
}

/// The end the two debug drawings share: an update-data object (`0.0, 0, 0`)
/// on the stack, the node updated with it and its properties updated, and the
/// node handed to `TES::AddTempDebugObject` for the time in
/// [`DEBUG_OBJECT_SECONDS`].
pub(crate) fn show_debug_node(e: &mut Engine, node: u32) {
    e.with_stack(12, |e, data| {
        e.call(UPDATE_DATA_CONSTRUCT, &args![data, 0.0f32, 0u32, 0u32]);
        e.call(NODE_UPDATE_WITH_DATA, &args![node, data]);
        e.call(NODE_UPDATE_PROPERTIES, &args![node]);
        let seconds = e.global::<f32>(DEBUG_OBJECT_SECONDS);
        let tes = e.global::<u32>(GLOBAL_0011DEA10);
        e.call(TES_ADD_TEMP_DEBUG_OBJECT, &args![tes, node, seconds]);
    });
}

/// The value of the `uGridsToLoad` setting (`SETTING_GRIDS_TO_LOAD`), read
/// through the setting's address getter every time the game does.
pub(crate) fn grids_to_load(e: &mut Engine) -> u32 {
    let value = e
        .call(SETTING_INT_VALUE_ADDRESS, &args![SETTING_GRIDS_TO_LOAD])
        .u32();
    e.mem.u32(value)
}

/// `FISTP` of a `float` through `00406d90`.
pub(crate) fn float_to_int(e: &mut Engine, value: f32) -> i32 {
    e.call(FLOAT_TO_INT, &args![value]).i32()
}

/// An array of `count` elements of `size` bytes: `operator new` of
/// `count * size` (all ones when that overflows) and the element constructor
/// loop (`00401050`) when the block came back.
pub(crate) fn new_element_array(e: &mut Engine, count: u32, size: u32, constructor: u32) -> u32 {
    let bytes = count.saturating_mul(size);
    let block = e.call(OPERATOR_NEW, &args![bytes]).u32();
    if block != 0 {
        e.call(
            VECTOR_CONSTRUCT_SIMPLE,
            &args![block, size, count, constructor],
        );
    }
    block
}

/// `NiPoint3::NiPoint3(x, y, z)` in a 12-byte block of its own that lives
/// while `f` runs with the block's address (a point the game keeps on its
/// stack and passes by address).
pub(crate) fn with_point<R>(
    e: &mut Engine,
    x: f32,
    y: f32,
    z: f32,
    f: impl FnOnce(&mut Engine, u32) -> R,
) -> R {
    e.with_stack(12, |e, block| {
        e.call(NI_POINT3_CONSTRUCT, &args![block, x, y, z]);
        f(e, block.addr())
    })
}

// ---- Translated functions --------------------------------------------------------

// Translated from 005b4b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDistanceFunction` (Xbox PDB): parses one object argument and
/// returns `Script::GetDistanceConditionFunction(thisObj, object, 0, result)`.
pub fn script_get_distance_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([object]) = a.parse_into(e, [0]) else {
        return false;
    };
    e.call(
        GET_DISTANCE_CONDITION,
        &args![a.this_obj, object, 0u32, a.result],
    )
    .bool()
}

// Translated from 005b4b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetInZoneFunction` (Xbox PDB): parses one object argument and
/// returns `Script::GetInZoneConditionFunction(thisObj, object, 0, result)`.
pub fn script_get_in_zone_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([object]) = a.parse_into(e, [0]) else {
        return false;
    };
    e.call(
        GET_IN_ZONE_CONDITION,
        &args![a.this_obj, object, 0u32, a.result],
    )
    .bool()
}

// Translated from 005b4be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AddItem` body (its failure message names AddItem): arguments are an
/// item form, a count and a flag. The items go into a temporary
/// `TESContainer` (0x10 bytes) that is then handed to the reference
/// (`004821a0`): a leveled list (form type `0x34`) is rolled at the
/// reference's level, a form list (type `0x55`) adds every element that
/// passes virtual slot `0xe4`, and any other form that passes it is added
/// directly. When nothing could be added the failure message is logged
/// (through the stub [`fn_005b5e40`]). A reference that answers true to virtual
/// slots `0x100` and `0x218` and is not the player finally gets `00606540(base
/// form, reference, result of slot 0x1e8, 1)`.
pub fn fn_005b4be0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([form, count, flag]) = a.parse_into(e, [0, 0, 0]) else {
        return false;
    };
    if a.this_obj.is_null() {
        return true;
    }
    let level = e.call(GET_CALC_LEVEL, &args![a.this_obj, 0u32]).u32();
    e.with_stack(0x10, |e, container| {
        e.call(TES_CONTAINER_CONSTRUCT, &args![container]);
        let mut leveled_list = 0u32;
        let mut form_list = 0u32;
        let mut item = 0u32;
        let mut type_0x28 = false;
        let mut check_item = false;
        match form_type(e, form) {
            0x28 => {
                type_0x28 = true;
                check_item = true;
            }
            0x34 => leveled_list = form,
            0x55 => form_list = form,
            _ => check_item = true,
        }
        if check_item && e.vcall(form, 0xe4, &args![]).bool() {
            item = form;
        }
        if leveled_list != 0 {
            e.call(
                FN_00487F70,
                &args![
                    leveled_list + 0x30,
                    level & 0xffff,
                    count & 0xffff,
                    container,
                    0u32
                ],
            );
        } else if form_list != 0 && count != 0 {
            let mut node = e.call(FORM_LIST_ITEMS, &args![form_list]).u32();
            while node != 0 && !list_is_empty(e, node) {
                let element = node_item(e, node);
                node = e.call(LIST_NEXT, &args![node]).u32();
                if e.vcall(element, 0xe4, &args![]).bool() {
                    e.call(FN_004818E0, &args![container, element, count, 0u32]);
                }
            }
        } else if item != 0 && count != 0 {
            e.call(FN_004818E0, &args![container, item, count, 0u32]);
        } else {
            // `MSG_ADD_ITEM_FAILED` with the script's name (virtual slot
            // `0x130`) goes to the stub, which discards it.
            let _name = e.vcall(a.script_obj.addr(), 0x130, &args![]).u32();
            let _ = MSG_ADD_ITEM_FAILED;
            fn_005b5e40(e);
        }
        if type_0x28 && e.call(FN_00446390, &args![form]).i32() < 10 {
            let value: f32 = e.global(ADD_ITEM_FLOAT);
            e.call(FN_00482090, &args![container, value]);
        }
        e.call(FN_004821A0, &args![container, a.this_obj, flag == 1]);
        if e.vcall(a.this_obj.addr(), 0x100, &args![]).bool()
            && a.this_obj.addr() != player(e)
            && e.vcall(a.this_obj.addr(), 0x218, &args![]).bool()
        {
            let extra = e.vcall(a.this_obj.addr(), 0x1e8, &args![]).u32();
            let base = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
            e.call(FN_00606540, &args![base, a.this_obj, extra, 1u32]);
        }
        e.call(TES_CONTAINER_DESTRUCT, &args![container]);
    });
    true
}

// Translated from 005b4e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes items from the reference: arguments are an item form, a count
/// and a flag that silences the player's message.
///
/// The forms to remove are collected into a temporary list: the form itself
/// when it passes virtual slot `0xe4` (a form of type `0x34` is looked up in
/// the reference's inventory changes instead), or every passing element of a
/// form list (type `0x55`). For each entry: a reference that is not an actor
/// just gets the entry removed through virtual slot `0x17c`; an actor has
/// the count capped to what it carries (`GetItemCountConditionFunction`), a
/// worn entry marked not wearable, the removal done through slot `0x17c`
/// and, for the player, the on-screen message
/// ([`player_removed_message`]) and the interface refresh `00704af0`.
pub fn fn_005b4e90(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([form, count, silent]) = a.parse_into(e, [0, 0, 0]) else {
        return false;
    };
    if a.this_obj.is_null() || form == 0 {
        return true;
    }
    e.with_stack(8, |e, list| {
        e.call(LIST_CONSTRUCT, &args![list]);
        let mut item = 0u32;
        if e.vcall(form, 0xe4, &args![]).bool() {
            item = form;
        }
        if form_type(e, form) == 0x34 {
            let extra = e.call(EXTRA_DATA_LIST, &args![a.this_obj]).u32();
            let changes = e.call(GET_CONTAINER_CHANGES, &args![extra]).u32();
            if changes != 0 {
                item = e.call(FN_004CBA70, &args![changes, form]).u32();
            }
        }
        if item != 0 {
            list_append(e, list, item);
        } else if form_type(e, form) == 0x55 {
            let mut node = e.call(FORM_LIST_ITEMS, &args![form]).u32();
            while node != 0 && !list_is_empty(e, node) {
                let element = node_item(e, node);
                node = e.call(LIST_NEXT, &args![node]).u32();
                if e.vcall(element, 0xe4, &args![]).bool() {
                    list_append(e, list, element);
                }
            }
        }
        while !list_is_empty(e, list.addr()) {
            let item = node_item(e, list.addr());
            e.call(LIST_POP_FRONT, &args![list]);
            let mut count = count;
            let actor = e
                .call(
                    DYNAMIC_CAST,
                    &args![a.this_obj, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
                )
                .u32();
            if actor == 0 {
                e.vcall(
                    a.this_obj.addr(),
                    0x17c,
                    &args![item, 0u32, count, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
                );
                continue;
            }
            // What the actor carries, written as a `double` by the callee.
            let carried = e.with_stack(8, |e, have| {
                e.mem.set_f64(have.addr(), count as i32 as f64);
                e.call(
                    GET_ITEM_COUNT_CONDITION,
                    &args![a.this_obj, item, 0u32, have],
                );
                e.mem.f64(have.addr())
            });
            if (count as i32 as f64) > carried {
                count = e.call(FTOL, &args![carried]).u32();
            }
            let mut worn = 0u32;
            if e.call(FN_00575400, &args![actor, item]).bool() {
                let changes = e.call(GET_INVENTORY_CHANGES, &args![a.this_obj]).u32();
                if changes != 0 {
                    worn = e.call(WEARING_OBJECT, &args![changes, item, 0u32]).u32();
                    if worn != 0 {
                        e.call(SET_CAN_NOT_WEAR, &args![worn, 0u32]);
                    }
                }
            }
            if (count as i32) <= 0 {
                continue;
            }
            let wearable = e.call(FN_00575400, &args![actor, item]).bool();
            if wearable {
                e.call(FN_008248E0, &args![actor + 0x94, item, 1u32]);
            }
            e.vcall(
                actor,
                0x17c,
                &args![item, worn, count, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
            );
            if actor == player(e) {
                if silent == 0 {
                    player_removed_message(e, item, count as i32, silent);
                }
                e.call(FN_00704AF0, &args![]);
            } else if wearable {
                let process = e
                    .call(MIDDLE_HIGH_PROCESS_SAVED_ACQUIRE, &args![actor])
                    .u32();
                if process != 0 && e.vcall(actor, 0x1d0, &args![]).u32() != 0 {
                    let process = e
                        .call(MIDDLE_HIGH_PROCESS_SAVED_ACQUIRE, &args![actor])
                        .u32();
                    e.vcall(process, 0xc4, &args![actor, 0u32]);
                }
            }
        }
        e.call(LIST_DESTRUCT, &args![list]);
    });
    true
}

/// The player's "item removed" message of `005b4e90`: formats
/// `"<count> <name><b> <a>"` (or `"<name> <a>"` for a single item) into a
/// temporary `BSStringT` from the two string globals; for the form types
/// below it is then shown with the vault boy icon and the item's pick-up
/// sound (none when `silent`), for every other type it is dropped.
fn player_removed_message(e: &mut Engine, item: u32, count: i32, silent: u32) {
    let text = e.mem.alloc(8);
    e.call(BS_STRING_CONSTRUCT, &args![text]);
    if count > 1 {
        let part_a = e.call(BS_STRING_TEXT, &args![MESSAGE_PART_A]).u32();
        let part_b = e.call(BS_STRING_TEXT, &args![MESSAGE_PART_B]).u32();
        let name = e.call(GET_FULL_NAME, &args![item]).u32();
        e.call(
            BS_STRING_FORMAT,
            &args![text, FORMAT_ITEM_COUNT_MESSAGE, count, name, part_b, part_a],
        );
    } else {
        let part_a = e.call(BS_STRING_TEXT, &args![MESSAGE_PART_A]).u32();
        let name = e.call(GET_FULL_NAME, &args![item]).u32();
        e.call(
            BS_STRING_FORMAT,
            &args![text, FORMAT_ITEM_MESSAGE, name, part_a],
        );
    }
    // The compiler's switch on `form type - 0x18` (a jump table over a byte
    // table): these types take the icon branch, all others do nothing.
    if matches!(
        form_type(e, item),
        0x18 | 0x19 | 0x1e | 0x1f | 0x28 | 0x29 | 0x2e | 0x2f | 0x32 | 0x67 | 0x6c | 0x73 | 0x74
    ) {
        let sound = if silent != 0 {
            0
        } else {
            let player = player(e);
            e.call(GET_PICK_UP_SOUND_NAME, &args![player, item, 0u32, 0u32])
                .u32()
        };
        let volume: f32 = e.global(PICKUP_VOLUME);
        let string = ni_pointer_get(e, text);
        e.call(
            SHOW_MESSAGE,
            &args![string, 0u32, ICON_VAULT_BOY, sound, volume, 0u32],
        );
    }
    e.call(BS_STRING_DESTRUCT, &args![text]);
    e.mem.free(text);
}

// Translated from 005b53d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives `containingObj` the base form of `thisObj` through virtual slot
/// `0x17c` (one parsed argument is passed along), carrying the worn entry
/// when `containingObj` is an actor that has the form, then refreshes the
/// player's quest targets. False when it did something; true when either
/// reference is missing.
pub fn fn_005b53d0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([extra]) = a.parse_into(e, [0]) else {
        return false;
    };
    if a.this_obj.is_null() || a.containing_obj.is_null() {
        return true;
    }
    let actor = e
        .call(
            DYNAMIC_CAST,
            &args![
                a.containing_obj,
                0u32,
                RTTI_TES_OBJECT_REFR,
                RTTI_ACTOR,
                0u32
            ],
        )
        .u32();
    let mut worn = 0u32;
    if actor != 0 {
        let base = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
        if e.call(FN_00575400, &args![actor, base]).bool() {
            let changes = e
                .call(GET_INVENTORY_CHANGES, &args![a.containing_obj])
                .u32();
            if changes != 0 {
                let base = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
                worn = e.call(WEARING_OBJECT, &args![changes, base, 0u32]).u32();
            }
        }
    }
    let base = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
    e.vcall(
        a.containing_obj.addr(),
        0x17c,
        &args![base, worn, 1u32, 0u32, 0u32, extra, 0u32, 0u32, 1u32, 0u32],
    );
    let player = player(e);
    e.call(CHECK_QUEST_TARGET_UPDATE, &args![player, a.this_obj]);
    if a.containing_obj.addr() == player {
        e.call(FN_00704AF0, &args![]);
    }
    false
}

// Translated from 005b5500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::DuplicateAllItemsFunction` (Xbox PDB): duplicates all of
/// `thisObj`'s inventory into the parsed target reference
/// (`InventoryChanges::DuplicateAllItems(thisObj, target)`) when its extra
/// data list has inventory changes.
pub fn script_duplicate_all_items_function(e: &mut Engine, a: ScriptArgs) -> bool {
    // The second argument is parsed into a local flag the function never
    // reads.
    let Some([target, _unused]) = a.parse_into(e, [0, 0]) else {
        return false;
    };
    if !a.this_obj.is_null() {
        let list = e.call(EXTRA_DATA_LIST, &args![a.this_obj]).u32();
        if list != 0 {
            let list = e.call(EXTRA_DATA_LIST, &args![a.this_obj]).u32();
            let changes = e.call(GET_CONTAINER_CHANGES, &args![list]).u32();
            if changes != 0 {
                e.call(DUPLICATE_ALL_ITEMS, &args![changes, a.this_obj, target]);
            }
        }
    }
    true
}

// Translated from 005b55a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the inventory-changes routine `004ce340` for `thisObj` with the
/// five parsed arguments (item, flag, option, slot -1 by default, extra),
/// then refreshes the player's quest targets.
pub fn fn_005b55a0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([item, flag, option, slot, extra]) = a.parse_into(e, [0, 0, 0, 0xffff_ffff, 0]) else {
        return false;
    };
    if !a.this_obj.is_null() {
        let list = e.call(EXTRA_DATA_LIST, &args![a.this_obj]).u32();
        if list != 0 {
            let list = e.call(EXTRA_DATA_LIST, &args![a.this_obj]).u32();
            let changes = e.call(GET_CONTAINER_CHANGES, &args![list]).u32();
            if changes != 0 {
                // The routine returns a `float` in `ST0`, which is discarded.
                e.call(
                    FN_004CE340,
                    &args![
                        changes,
                        a.this_obj,
                        item,
                        0u32,
                        flag != 0,
                        0u32,
                        option != 0,
                        slot,
                        extra
                    ],
                );
                let player = player(e);
                e.call(CHECK_QUEST_TARGET_UPDATE, &args![player, a.this_obj]);
            }
        }
    }
    true
}

// Translated from 005b5690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_005b55a0`] with `004ce380`, three parsed arguments (item, flag,
/// option), slot `-1` and extra `0`.
pub fn fn_005b5690(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([item, flag, option]) = a.parse_into(e, [0, 0, 0]) else {
        return false;
    };
    if !a.this_obj.is_null() {
        let list = e.call(EXTRA_DATA_LIST, &args![a.this_obj]).u32();
        if list != 0 {
            let list = e.call(EXTRA_DATA_LIST, &args![a.this_obj]).u32();
            let changes = e.call(GET_CONTAINER_CHANGES, &args![list]).u32();
            if changes != 0 {
                e.call(
                    FN_004CE380,
                    &args![
                        changes,
                        a.this_obj,
                        item,
                        0u32,
                        flag != 0,
                        0u32,
                        option != 0,
                        0xffff_ffffu32,
                        0u32
                    ],
                );
                let player = player(e);
                e.call(CHECK_QUEST_TARGET_UPDATE, &args![player, a.this_obj]);
            }
        }
    }
    true
}

// Translated from 005b5760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Function-style command: clears the result, parses a form (default: the
/// player's base form) and calls `005a2e20(thisObj, form, 0, result)`, which
/// fills `*result`.
pub fn fn_005b5760(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let Some([form]) = a.parse_into(e, [0]) else {
        return false;
    };
    let form = if form == 0 {
        let player = player(e);
        e.call(GET_BASE_FORM, &args![player]).u32()
    } else {
        form
    };
    e.call(FN_005A2E20, &args![a.this_obj, form, 0u32, a.result]);
    true
}

// Translated from 005b57e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a form (default: the player's base form); for a reference, calls
/// `00419700` on its extra data list with the form and then virtual slot
/// `0x48` with `0x40`.
pub fn fn_005b57e0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([form]) = a.parse_into(e, [0]) else {
        return false;
    };
    let form = if form == 0 {
        let player = player(e);
        e.call(GET_BASE_FORM, &args![player]).u32()
    } else {
        form
    };
    if !a.this_obj.is_null() {
        let list = e.call(EXTRA_DATA_LIST, &args![a.this_obj]).u32();
        e.call(FN_00419700, &args![list, form]);
        e.vcall(a.this_obj.addr(), 0x48, &args![0x40u32]);
    }
    true
}

// Translated from 005b5860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives `containingObj` the base form of `thisObj` through virtual slot
/// `0x17c`, refreshes the player's quest targets and, in menu mode, calls the
/// interface refresh `00704af0`. False when it did something; true when
/// either reference is missing.
pub fn fn_005b5860(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() || a.containing_obj.is_null() {
        return true;
    }
    let base = e.call(GET_BASE_FORM, &args![a.this_obj]).u32();
    e.vcall(
        a.containing_obj.addr(),
        0x17c,
        &args![base, 0u32, 1u32, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32],
    );
    let player = player(e);
    e.call(CHECK_QUEST_TARGET_UPDATE, &args![player, a.this_obj]);
    if e.call(IS_IN_MENU_MODE, &args![]).bool() {
        e.call(FN_00704AF0, &args![]);
    }
    false
}

// Translated from 005b58d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a form and a count and has `thisObj` process them through its
/// virtual slot `0x17c`.
pub fn fn_005b58d0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([form, count]) = a.parse_into(e, [0, 0]) else {
        return false;
    };
    if !a.this_obj.is_null() {
        e.vcall(
            a.this_obj.addr(),
            0x17c,
            &args![form, 0u32, count, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32],
        );
    }
    true
}

// Translated from 005b5950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsActionRefFunction` (Xbox PDB): `*result` is 1.0 when the
/// parsed reference is `thisObj`'s action reference, else 0.0; echoes the
/// value to the console when the TLS echo flag is set.
pub fn script_is_action_ref_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.mem.set_f64(a.result.addr(), 0.0);
    let Some([reference]) = a.parse_into(e, [0]) else {
        return false;
    };
    if !a.this_obj.is_null() && reference != 0 {
        let action_ref = e.call(GET_ACTION_REF, &args![a.this_obj]).u32();
        if reference == action_ref {
            e.mem.set_f64(a.result.addr(), 1.0);
        }
    }
    let tls = e.tls();
    if e.mem.u8(tls + TLS_ECHO) != 0 {
        let value = e.mem.f64(a.result.addr());
        console_print(e, &args![MSG_IS_ACTION_REF, value]);
    }
    true
}

// Translated from 005b59f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ActivateFunction` (Xbox PDB): has `thisObj` activate the parsed
/// reference (default: its action reference) unless that reference has form
/// flag `0x800`. Nested more than [`ACTIVATE_DEPTH_LIMIT`] + 1 deep (a
/// counter in the TLS block) it does nothing. With the second argument zero,
/// action 1 is set for the duration if it was not set, and action 2 always.
/// The player may not run it: the command logs a message (through the stub
/// [`fn_005b5e40`]) and fails.
pub fn script_activate_function(e: &mut Engine, a: ScriptArgs) -> bool {
    if player(e) == a.this_obj.addr() {
        let _ = MSG_PLAYER_ACTIVATE;
        fn_005b5e40(e);
        return false;
    }
    let Some([target, direct]) = a.parse_into(e, [0, 0]) else {
        return false;
    };
    if a.this_obj.is_null() {
        return true;
    }
    let tls = e.tls();
    let depth = e.mem.u32(tls + TLS_ACTIVATE_DEPTH);
    if depth > ACTIVATE_DEPTH_LIMIT {
        return true;
    }
    e.mem
        .set_u32(tls + TLS_ACTIVATE_DEPTH, depth.wrapping_add(1));
    let target = if target == 0 {
        e.call(GET_ACTION_REF, &args![a.this_obj]).u32()
    } else {
        target
    };
    if target != 0 && !e.call(FN_00440DA0, &args![target]).bool() {
        if direct == 0 {
            let had_action = e.call(HAS_ACTION, &args![a.this_obj, 1u32]).bool();
            if !had_action {
                e.call(SET_ACTION, &args![a.this_obj, 1u32]);
            }
            e.call(SET_ACTION, &args![a.this_obj, 2u32]);
            e.call(ACTIVATE, &args![a.this_obj, target, 0u32, 0u32, 1u32]);
            e.call(CLEAR_ACTION, &args![a.this_obj, 2u32]);
            if !had_action {
                e.call(CLEAR_ACTION, &args![a.this_obj, 1u32]);
            }
        } else {
            e.call(ACTIVATE, &args![a.this_obj, target, 0u32, 0u32, 1u32]);
        }
    }
    let depth = e.mem.u32(tls + TLS_ACTIVATE_DEPTH);
    e.mem
        .set_u32(tls + TLS_ACTIVATE_DEPTH, depth.wrapping_sub(1));
    true
}

// Translated from 005b5b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleCombatStatsFunction` (Xbox PDB): prints that the command
/// was removed.
pub fn script_toggle_combat_stats_function(e: &mut Engine) -> bool {
    console_print(e, &args![MSG_COMBAT_STATS_REMOVED]);
    true
}

// Translated from 005b5bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::PlayerSpellBook` (Xbox PDB): walks the data handler's spell list
/// (`+0x40`); for every non-null entry whose member at `+0x18` reports the
/// type 0, 2 or 3 through its virtual slot `0x18`, asks the player (virtual
/// slot `0x3e0`) to take it, counts the successes and prints the count.
pub fn script_player_spell_book(e: &mut Engine) -> bool {
    let handler = e.global::<u32>(DATA_HANDLER);
    let mut node = e.call(DATA_HANDLER_SPELLS, &args![handler]).u32();
    let mut added = 0u32;
    while node != 0 {
        if e.call(LIST_COUNT, &args![node]).u32() == 0 {
            break;
        }
        if node_item(e, node) != 0 {
            // The type is asked afresh at every test, as the game does.
            let kind = |e: &mut Engine| {
                let entry = node_item(e, node);
                e.vcall(entry + 0x18, 0x18, &args![]).u32()
            };
            let take = kind(e) == 0 || kind(e) == 2 || kind(e) == 3;
            if take {
                let spell = node_item(e, node);
                let player = player(e);
                if e.vcall(player, 0x3e0, &args![spell]).bool() {
                    added += 1;
                }
            }
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    console_print(e, &args![MSG_SPELLS_ADDED, added]);
    true
}

// Translated from 005b5cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowWhoDetectsPlayer` (Xbox PDB): prints the detection lists.
pub fn script_show_who_detects_player(e: &mut Engine) -> bool {
    e.call(
        PROCESS_LISTS_PRINT_LISTS,
        &args![PROCESS_LISTS, 1u32, -1i32],
    );
    console_print(e, &args![MSG_DETECTION_LIST_PRINTED]);
    true
}

// Translated from 005b5cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::PrintAILists` (Xbox PDB): prints the AI lists.
pub fn script_print_ai_lists(e: &mut Engine) -> bool {
    e.call(
        PROCESS_LISTS_PRINT_LISTS,
        &args![PROCESS_LISTS, 0u32, -1i32],
    );
    console_print(e, &args![MSG_AI_LISTS_PRINTED]);
    true
}

// Translated from 005b5d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the (empty) `00483710` on the process lists singleton.
pub fn fn_005b5d20(e: &mut Engine) -> bool {
    e.call(FN_00483710, &args![PROCESS_LISTS]);
    true
}

// Translated from 005b5d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one integer and stores whether it is non-zero in the byte global
/// `011f122d`; false when the arguments do not parse.
pub fn fn_005b5d40(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = a.parse_into(e, [0]) else {
        return false;
    };
    e.set_global(FLAG_0011F122D, (value != 0) as u8);
    true
}

// Translated from 005b5da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte global `011f9fc0` to 1.
pub fn fn_005b5da0(e: &mut Engine) -> bool {
    e.set_global(FLAG_0011F9FC0, 1u8);
    true
}

// Translated from 005b5db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::FreezeRendererAccumulation` (Xbox PDB): prints that the command
/// is deprecated.
pub fn script_freeze_renderer_accumulation(e: &mut Engine) -> bool {
    console_print(e, &args![MSG_FREEZE_RENDERER_DEPRECATED]);
    true
}

// Translated from 005b5dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleOcclusion` (Xbox PDB): flips the occlusion query flag and
/// prints its new state.
pub fn script_toggle_occlusion(e: &mut Engine) -> bool {
    let enabled = e.global::<u8>(OCCLUSION_QUERY) == 0;
    e.set_global(OCCLUSION_QUERY, enabled as u8);
    let state = if enabled { TEXT_ON } else { TEXT_OFF };
    console_print(e, &args![MSG_OCCLUSION_QUERY, state]);
    true
}

// Translated from 005b5e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::OutputMemContexts` (Xbox PDB): prints that the command is no
/// longer supported.
pub fn script_output_mem_contexts(e: &mut Engine) -> bool {
    console_print(e, &args![MSG_MEM_CONTEXTS_UNSUPPORTED]);
    true
}

// Translated from 005b5e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// A logging stub: accepts whatever arguments the caller pushes and returns
/// 0 (the release build compiles the log call down to this). Callers in this
/// unit pass no arguments here.
pub fn fn_005b5e40(_e: &mut Engine) -> u32 {
    0
}

// Translated from 005b5e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::OutputMemStats` (Xbox PDB): the memory statistics dump.
///
/// Opens the memory heap dump file named by the argument (default
/// `OutputMemStats.mhd`, otherwise `MemStats<name>.mhd`), then writes
/// `ModelDump<name>.txt` (the model loader's and the face-gen manager's
/// model maps) and `TextureDump<name>.txt`: every live texture is inserted
/// into a table of 0x800 `NiPointer`s ordered by size (the size function is
/// the stub [`fn_005b5e40`], so every size is 0), then one tab-separated line
/// is written per table entry (index, category by path, description, width x
/// height, size in KB, format, reference count minus one, "L" or "H", and
/// the owner's name).
///
/// Differences from the game: when the argument does not parse, the game's
/// `<name>` buffer is uninitialised stack (zeroed here); two locals the game
/// fills (total size, texture count) and never reads are left out.
pub fn script_output_mem_stats(e: &mut Engine, a: ScriptArgs) -> bool {
    let path = e.mem.alloc(0x200);
    let name = e.mem.alloc(0x104);
    let description = e.mem.alloc(0x104);
    let line = e.mem.alloc(0x200);
    let heap_dump = e.mem.alloc(0x18);
    let model_file = e.mem.alloc(BS_FILE_SIZE);
    let texture_file = e.mem.alloc(BS_FILE_SIZE);
    let table = e.mem.alloc(4 * TEXTURE_SLOTS);

    e.mem.set_u8(path, 0);
    e.call(MEMSET, &args![path + 1, 0u32, 0x1ffu32]);
    if a.parse(e, &[path]) {
        e.call(STRCPY_S, &args![name, 0x104u32, path]);
        e.call(SPRINTF_S, &args![path, 0x200u32, FORMAT_MEM_STATS, name]);
    } else {
        e.call(STRCPY_S, &args![path, 0x200u32, DEFAULT_MEM_STATS]);
    }
    e.call(MEM_STATS_FILE_CONSTRUCT, &args![heap_dump, path]);
    // 005b5f56: `00401020` takes no word (plain RET, answers 011f6238); the pushed dump and
    // 0 are the two words of `00aa47c0` (RET 8).
    let output = e.call(MEM_STATS_OUTPUT_GET, &args![]).u32();
    e.call(MEM_STATS_OUTPUT_SET, &args![output, heap_dump, 0u32]);

    e.call(SPRINTF_S, &args![path, 0x200u32, FORMAT_MODEL_DUMP, name]);
    e.call(
        BS_FILE_CONSTRUCT,
        &args![model_file, path, 1u32, 0x4000u32, 0u32],
    );
    e.call(BS_FILE_OPEN, &args![model_file, 0u32, 0u32]);
    let model_loader = e.global::<u32>(MODEL_LOADER);
    if model_loader != 0 {
        e.call(
            MODEL_LOADER_OUTPUT_MODEL_MAP,
            &args![model_loader, model_file, 0u32],
        );
    }
    if e.call(GET_MODEL_CACHE, &args![]).u32() != 0 {
        let cache = e.call(GET_MODEL_CACHE, &args![]).u32();
        e.call(FACE_GEN_OUTPUT_MODEL_MAP, &args![cache, model_file]);
    }
    e.call(BS_FILE_CLOSE, &args![model_file]);

    e.call(SPRINTF_S, &args![path, 0x200u32, FORMAT_TEXTURE_DUMP, name]);
    e.call(
        BS_FILE_CONSTRUCT,
        &args![texture_file, path, 1u32, 0x4000u32, 0u32],
    );
    e.call(BS_FILE_OPEN, &args![texture_file, 0u32, 0u32]);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            table,
            4u32,
            TEXTURE_SLOTS,
            NI_POINTER_CONSTRUCT,
            NI_POINTER_DESTRUCT
        ],
    );
    e.call(ZERO_FILL, &args![table, 0u32, 4 * TEXTURE_SLOTS]);

    // Insert every texture before the first table entry that is empty or
    // smaller than it, shifting the entries above up by one.
    let mut texture = e.call(FIRST_TEXTURE, &args![]).u32();
    while texture != 0 {
        if e.call(GET_D3D_TEXTURE, &args![texture]).u32() != 0 {
            let size = fn_005b5e40(e);
            let mut insert_at = -1i32;
            let mut i = 0i32;
            while i < TEXTURE_SLOTS as i32 && insert_at == -1 {
                let slot = table + 4 * i as u32;
                if ni_pointer_get(e, slot) == 0 {
                    insert_at = i;
                } else {
                    // The game fetches the pointer again to pass it to the
                    // size stub.
                    let _other = ni_pointer_get(e, slot);
                    if fn_005b5e40(e) < size {
                        insert_at = i;
                    }
                }
                i += 1;
            }
            if insert_at != -1 {
                let mut j = TEXTURE_SLOTS as i32 - 1;
                while j > insert_at {
                    let to = table + 4 * j as u32;
                    e.call(NI_POINTER_ASSIGN, &args![to, to - 4]);
                    j -= 1;
                }
                e.call(
                    NI_POINTER_SET,
                    &args![table + 4 * insert_at as u32, texture],
                );
            }
        }
        texture = e.call(NEXT_TEXTURE, &args![texture]).u32();
    }

    for i in 0..TEXTURE_SLOTS {
        let slot = table + 4 * i;
        if ni_pointer_get(e, slot) == 0 {
            continue;
        }
        let mut rendered = false;
        let candidate = ni_pointer_get(e, slot);
        if e.call(TEXTURE_IS_SOURCE, &args![TEXTURE_CLASS, candidate])
            .bool()
        {
            let texture = ni_pointer_get(e, slot);
            let source = e.vcall(texture, 0x9c, &args![]).u32();
            let mut text = e.call(NAME_TEXT, &args![source]).u32();
            if text == 0 || e.mem.i8(text) == 0 {
                let texture = ni_pointer_get(e, slot);
                let source = e.call(FN_00413F40, &args![texture]).u32();
                text = e.call(NAME_TEXT, &args![source]).u32();
            }
            if text == 0 || e.mem.i8(text) == 0 {
                e.call(
                    STRCPY_S,
                    &args![description, 0x104u32, TEXT_UNNAMED_SOURCE_TEXTURE],
                );
            } else {
                e.call(STRCPY_S, &args![description, 0x104u32, text]);
            }
        } else {
            rendered = true;
            let texture = ni_pointer_get(e, slot);
            let source = e.call(FN_00413F40, &args![texture]).u32();
            let text = e.call(NAME_TEXT, &args![source]).u32();
            if text != 0 && e.mem.i8(text) != 0 {
                e.call(
                    SPRINTF_S,
                    &args![description, 0x104u32, FORMAT_RENDERED_TEXTURE, text],
                );
            } else {
                e.call(
                    SPRINTF_S,
                    &args![description, 0x104u32, TEXT_RENDERED_TEXTURE_NONE],
                );
            }
        }
        e.call(FIX_PATH, &args![description, 0x104u32]);
        let mut category = CATEGORY_SYSTEM;
        if !rendered {
            let contains = |e: &mut Engine, needle: u32| {
                e.call(FIND_SUBSTRING, &args![description, needle]).u32() != 0
            };
            category = if [
                PATH_ARMOR,
                PATH_CREATURES,
                PATH_CHARACTERS,
                PATH_PIPBOY,
                PATH_WEAPONS,
                PATH_DECALS,
                PATH_PROJECTILES,
            ]
            .into_iter()
            .any(|needle| contains(e, needle))
            {
                CATEGORY_ACTOR
            } else if contains(e, PATH_FONTS) || contains(e, PATH_INTERFACE) {
                CATEGORY_MENUS
            } else if contains(e, PATH_EFFECTS) || contains(e, PATH_GORE) {
                CATEGORY_EFFECTS
            } else if contains(e, PATH_SKY) {
                CATEGORY_SKY
            } else if contains(e, PATH_LOD) {
                CATEGORY_LOD
            } else if contains(e, PATH_WATER) {
                CATEGORY_WATER
            } else {
                CATEGORY_MISCREF
            };
        }
        // The game passes the pointer to the size stub.
        let _size_argument = ni_pointer_get(e, slot);
        let size = fn_005b5e40(e);
        let texture = ni_pointer_get(e, slot);
        let d3d_texture = e.call(GET_D3D_TEXTURE, &args![texture]).u32();
        let surface = e.call(FN_007D6BB0, &args![d3d_texture]).u32();
        let format_id = e.call(D3D_FORMAT_ID, &args![surface]).u32();
        let format_name = e.call(GET_D3D_FORMAT_STRING, &args![format_id]).u32();
        let texture = ni_pointer_get(e, slot);
        let owner = e.vcall(texture, 8, &args![]).u32();
        let owner_name = ni_pointer_get(e, owner);
        let texture = ni_pointer_get(e, slot);
        let flag_text = if e.call(TEXTURE_KIND, &args![texture]).u32() == 3 {
            TEXT_L
        } else {
            TEXT_H
        };
        let for_height = ni_pointer_get(e, slot);
        let for_width = ni_pointer_get(e, slot);
        let texture = ni_pointer_get(e, slot);
        let references = e.call(LIST_NEXT, &args![texture]).u32().wrapping_sub(1);
        let height = e.vcall(for_height, 0x98, &args![]).u32();
        let width = e.vcall(for_width, 0x94, &args![]).u32();
        e.call(
            SPRINTF_S,
            &args![
                line,
                0x200u32,
                FORMAT_TEXTURE_LINE,
                i + 1,
                category,
                description,
                width,
                height,
                size >> 10,
                format_name,
                references,
                flag_text,
                owner_name
            ],
        );
        fn_005b5e40(e);
        let length = e.call(STRLEN, &args![line]).u32();
        e.call(BS_FILE_WRITE, &args![texture_file, line, length + 1]);
    }

    e.call(BS_FILE_CLOSE, &args![texture_file]);
    e.call(
        VECTOR_DESTRUCT,
        &args![table, 4u32, TEXTURE_SLOTS, NI_POINTER_DESTRUCT],
    );
    e.call(BS_FILE_DESTRUCT, &args![texture_file]);
    e.call(BS_FILE_DESTRUCT, &args![model_file]);
    e.call(MEM_STATS_FILE_DESTRUCT, &args![heap_dump]);

    for block in [
        path,
        name,
        description,
        line,
        heap_dump,
        model_file,
        texture_file,
        table,
    ] {
        e.mem.free(block);
    }
    true
}

// Translated from 005b6800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::OutputTextureUseMap` (Xbox PDB): writes the texture use map of
/// the reference's 3D (virtual slot `0x1d0`), or of the whole scene when
/// there is none, to the file named by the argument (default
/// `TextureUseMap.txt`), prints whether that worked and the size of the
/// textures in use.
pub fn script_output_texture_use_map(e: &mut Engine, a: ScriptArgs) -> bool {
    let path = e.mem.alloc(0x200);
    let map = e.mem.alloc(0x14);
    e.mem.set_u8(path, 0);
    e.call(MEMSET, &args![path + 1, 0u32, 0x1ffu32]);
    e.call(STRCPY_S, &args![path, 0x200u32, DEFAULT_TEXTURE_USE_MAP]);
    if !a.parse(e, &[path]) {
        e.call(STRCPY_S, &args![path, 0x200u32, DEFAULT_TEXTURE_USE_MAP]);
    }
    e.call(TEXTURE_USE_MAP_CONSTRUCT, &args![map]);
    if !a.this_obj.is_null() && e.vcall(a.this_obj.addr(), 0x1d0, &args![]).u32() != 0 {
        let node = e.vcall(a.this_obj.addr(), 0x1d0, &args![]).u32();
        e.call(TEXTURE_USE_MAP_ADD_NODE, &args![map, node, 1u32]);
    } else {
        e.call(TEXTURE_USE_MAP_ADD_SCENE, &args![map, 1u32]);
    }
    if e.call(TEXTURE_USE_MAP_WRITE_TO_FILE, &args![map, path])
        .bool()
    {
        console_print(e, &args![MSG_MAP_WRITTEN, path]);
    } else {
        console_print(e, &args![MSG_MAP_WRITE_FAILED, path]);
    }
    let text = e.mem.alloc(8);
    e.call(BS_STRING_CONSTRUCT, &args![text]);
    let mut size = e.call(TEXTURE_USE_MAP_TOTAL_SIZE, &args![map]).u32();
    if size > 0x10_0000 {
        size >>= 20;
        e.call(BS_STRING_FORMAT, &args![text, FORMAT_MB, size]);
    } else if size > 0x400 {
        size >>= 10;
        e.call(BS_STRING_FORMAT, &args![text, FORMAT_KB, size]);
    } else {
        e.call(BS_STRING_FORMAT, &args![text, FORMAT_BYTES, size]);
    }
    e.call(BS_STRING_APPEND, &args![text, TEXT_TEXTURES_IN_USE]);
    let message = ni_pointer_get(e, text);
    console_print(e, &args![message]);
    e.call(BS_STRING_DESTRUCT, &args![text]);
    e.call(TEXTURE_USE_MAP_DESTRUCT, &args![map]);
    e.mem.free(text);
    e.mem.free(map);
    e.mem.free(path);
    true
}

// Translated from 005b6a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Opens the memory heap dump file named by the argument (default
/// `OutputMemStats.mhd`) the way `Script::OutputMemStats` does and
/// configures the output object `011f6238` with flag 1 (where
/// `OutputMemStats` uses flag 0).
pub fn fn_005b6a30(e: &mut Engine, a: ScriptArgs) -> bool {
    let path = e.mem.alloc(0x200);
    let heap_dump = e.mem.alloc(0x18);
    e.mem.set_u8(path, 0);
    e.call(MEMSET, &args![path + 1, 0u32, 0x1ffu32]);
    e.call(STRNCPY_S, &args![path, DEFAULT_MEM_STATS, 0x1ffu32]);
    if !a.parse(e, &[path]) {
        e.call(STRNCPY_S, &args![path, DEFAULT_MEM_STATS, 0x1ffu32]);
    }
    e.call(MEM_STATS_FILE_CONSTRUCT, &args![heap_dump, path]);
    // As above: the dump and 1 are the two words of `00aa47c0`.
    let output = e.call(MEM_STATS_OUTPUT_GET, &args![]).u32();
    e.call(MEM_STATS_OUTPUT_SET, &args![output, heap_dump, 1u32]);
    e.call(MEM_STATS_FILE_DESTRUCT, &args![heap_dump]);
    e.mem.free(heap_dump);
    e.mem.free(path);
    true
}

// Translated from 005b6b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::OutputArchiveProfile` (Xbox PDB): the file name argument
/// (default `ArchiveProfile.txt`) gets `.txt` appended when it has none;
/// archive profiling (`00af4390`) is compiled out of this build, so the
/// command normally prints that it is not enabled.
pub fn script_output_archive_profile(e: &mut Engine, a: ScriptArgs) -> bool {
    let path = e.mem.alloc(0x200);
    e.call(STRCPY_S, &args![path, 0x200u32, DEFAULT_ARCHIVE_PROFILE]);
    if !a.parse(e, &[path]) {
        e.call(STRCPY_S, &args![path, 0x200u32, DEFAULT_ARCHIVE_PROFILE]);
    }
    if e.call(FIND_SUBSTRING, &args![path, EXTENSION_TXT]).u32() == 0 {
        e.call(STRCAT_S, &args![path, 0x200u32, EXTENSION_TXT]);
    }
    if e.call(ARCHIVE_PROFILE_START, &args![path]).bool() {
        console_print(e, &args![MSG_ARCHIVE_PROFILE_OUTPUT, path]);
    } else {
        console_print(e, &args![MSG_ARCHIVE_PROFILE_DISABLED]);
    }
    e.mem.free(path);
    true
}

// Translated from 005b6c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// For each of the two singletons `0044f560` and `004de490` return: calls
/// `005e01b0(singleton, singleton + 4)`.
pub fn fn_005b6c50(e: &mut Engine) -> bool {
    for getter in [FN_0044F560, FN_004DE490] {
        let singleton = e.call(getter, &args![]).u32();
        let plus_4 = e.call(PLUS_4, &args![singleton]).u32();
        let singleton = e.call(getter, &args![]).u32();
        e.call(FN_005E01B0, &args![singleton, plus_4]);
    }
    true
}

// Translated from 005b6c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Prints "Bye." and sets the byte at `+1` of the singleton `011dea0c`
/// (`005b6cb0`).
pub fn fn_005b6c90(e: &mut Engine) -> bool {
    console_print(e, &args![MSG_BYE]);
    let singleton = Ptr::new(e.global::<u32>(GLOBAL_0011DEA0C));
    fn_005b6cb0(e, singleton);
    true
}

// Translated from 005b6cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `this + 1` to 1.
pub fn fn_005b6cb0(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 1, 1);
}

// Translated from 005b6cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees released objects: sets up a 12-byte level scope `(1, 1, 1)`
/// (`00878160`), calls `004539a0(1, 0)` on the singleton `011dea10`, calls
/// `MemoryLevelManager::FreeReleasedObjects` with the byte at `+5` of the
/// scope and tears the scope down (`00878200`).
pub fn fn_005b6cd0(e: &mut Engine) -> bool {
    e.with_stack(12, |e, scope| {
        e.call(FN_00878160, &args![scope, 1u32, 1u32, 1u32]);
        let singleton = e.global::<u32>(GLOBAL_0011DEA10);
        e.call(FN_004539A0, &args![singleton, 1u32, 0u32]);
        let level = e.mem.u8(scope.addr() + 5);
        e.call(FREE_RELEASED_OBJECTS, &args![level]);
        e.call(FN_00878200, &args![scope]);
    });
    true
}

// Translated from 005b6d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00848e70(0, 0)` on the save/load singleton `011ddf38`.
pub fn fn_005b6d20(e: &mut Engine) -> bool {
    let singleton = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(FN_00848E70, &args![singleton, 0u32, 0u32]);
    true
}

// Translated from 005b6d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleMapMarkersFunction` (Xbox PDB): shows or hides the map
/// markers of every world space in the data handler's list.
///
/// Arguments (defaults 1, 1, 1): `visible`, `travel`, `skip_hidden`. A marker
/// whose data is hidden is skipped when `skip_hidden` is set; otherwise its
/// visibility is set to `visible` and, when shown and it has no travel
/// location yet, the travel location to `travel` (when hidden both are
/// cleared); then virtual slot `0x48` is called with `0x80000000`. Prints
/// the new state when the TLS echo flag is set.
pub fn script_toggle_map_markers_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([visible, travel, skip_hidden]) = a.parse_into(e, [1, 1, 1]) else {
        return false;
    };
    let handler = e.global::<u32>(DATA_HANDLER);
    let mut world_node = e.call(DATA_HANDLER_WORLD_SPACES, &args![handler]).u32();
    while world_node != 0 && !list_is_empty(e, world_node) {
        let world = node_item(e, world_node);
        world_node = e.call(LIST_NEXT, &args![world_node]).u32();
        let markers = e.call(BUILD_MAP_MARKER_LIST, &args![world, 0u32]).u32();
        if markers == 0 {
            continue;
        }
        while !list_is_empty(e, markers) {
            let marker = node_item(e, markers);
            e.call(LIST_POP_FRONT, &args![markers]);
            let data = e.call(GET_MAP_MARKER_DATA, &args![marker]).u32();
            if skip_hidden != 0 && e.call(MAP_MARKER_GET_HIDDEN, &args![data]).bool() {
                continue;
            }
            if visible == 0 {
                e.call(MAP_MARKER_SET_VISIBLE, &args![data, 0u32]);
                e.call(MAP_MARKER_SET_TRAVEL_LOC, &args![data, 0u32]);
            } else {
                e.call(MAP_MARKER_SET_VISIBLE, &args![data, 1u32]);
                if !e.call(MAP_MARKER_GET_TRAVEL_LOC, &args![data]).bool() {
                    e.call(MAP_MARKER_SET_TRAVEL_LOC, &args![data, travel != 0]);
                }
            }
            e.vcall(marker, 0x48, &args![0x8000_0000u32]);
        }
        e.call(LIST_DELETE, &args![markers, 1u32]);
    }
    let tls = e.tls();
    if e.mem.u8(tls + TLS_ECHO) != 0 {
        let state = if (visible as i32) > 0 {
            TEXT_SHOWN
        } else {
            TEXT_HIDDEN
        };
        console_print(e, &args![MSG_MAP_MARKERS, state]);
    }
    true
}

// Translated from 005b6f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleVerboseFunction` (Xbox PDB): flips the verbose messages
/// flag and prints the new state.
pub fn script_toggle_verbose_function(e: &mut Engine) -> bool {
    let verbose = fn_005b6f70(e);
    fn_005b6f60(e, !verbose);
    let state = if fn_005b6f70(e) {
        TEXT_SHOWN
    } else {
        TEXT_HIDDEN
    };
    console_print(e, &args![MSG_VERBOSE, state]);
    true
}

// Translated from 005b6f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the verbose messages flag (byte global `011f158c`).
pub fn fn_005b6f60(e: &mut Engine, value: bool) {
    e.set_global(VERBOSE_MESSAGES, value as u8);
}

// Translated from 005b6f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The verbose messages flag (byte global `011f158c`).
pub fn fn_005b6f70(e: &mut Engine) -> bool {
    e.global::<u8>(VERBOSE_MESSAGES) != 0
}

// Translated from 005b6f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleGodModeFunction` (Xbox PDB): flips god mode; turning it
/// off also turns demigod mode off (with a message); prints the new state.
pub fn script_toggle_god_mode_function(e: &mut Engine) -> bool {
    let god = e.call(IS_GOD_MODE, &args![]).bool();
    e.call(SET_GOD_MODE, &args![!god]);
    if !e.call(IS_GOD_MODE, &args![]).bool() && e.call(IS_DEMIGOD_MODE, &args![]).bool() {
        e.call(SET_DEMIGOD_MODE, &args![false]);
        console_print(e, &args![MSG_DEMIGOD_DISABLED]);
    }
    let state = if e.call(IS_GOD_MODE, &args![]).bool() {
        TEXT_ENABLED
    } else {
        TEXT_DISABLED
    };
    console_print(e, &args![MSG_GOD_MODE, state]);
    true
}

// Translated from 005b7000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleDemigodModeFunction` (Xbox PDB): flips demigod mode, sets
/// god mode to the same new value and prints the new demigod state.
pub fn script_toggle_demigod_mode_function(e: &mut Engine) -> bool {
    let demigod = e.call(IS_DEMIGOD_MODE, &args![]).bool();
    e.call(SET_DEMIGOD_MODE, &args![!demigod]);
    e.call(SET_GOD_MODE, &args![!demigod]);
    let state = if e.call(IS_DEMIGOD_MODE, &args![]).bool() {
        TEXT_ENABLED
    } else {
        TEXT_DISABLED
    };
    console_print(e, &args![MSG_DEMIGOD_MODE, state]);
    true
}

// Translated from 005b7070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleScriptsFunction` (Xbox PDB): flips script processing and
/// prints the new state.
pub fn script_toggle_scripts_function(e: &mut Engine) -> bool {
    let processing = e.call(GET_PROCESS_SCRIPTS, &args![]).bool();
    e.call(SET_PROCESS_SCRIPTS, &args![!processing]);
    let state = if e.call(GET_PROCESS_SCRIPTS, &args![]).bool() {
        TEXT_ENABLED
    } else {
        TEXT_DISABLED
    };
    console_print(e, &args![MSG_SCRIPT_PROCESSING, state]);
    true
}

// Translated from 005b70c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleGrassFunction` (Xbox PDB): when the grass object
/// (`0054f4c0`) exists, flips its flag (`00450f90`) and keeps the byte
/// [`GRASS_DISPLAY_FLAG`] at the opposite value. Turning the flag on runs
/// `00b62a20`; turning it off takes the animation data of the object behind
/// `0045c670`, and when there is one passes the vector `0045bb80` returns to
/// `0057d0a0`. Prints "Grass Display Enabled." or "Disabled.".
pub fn script_toggle_grass_function(e: &mut Engine) -> bool {
    let object = e.call(FN_0054F4C0, &args![]).u32();
    if object == 0 {
        return true;
    }
    let shown = e.call(FN_00456610, &args![object]).bool();
    let flag = !shown;
    e.call(FN_00450F90, &args![object, flag]);
    e.set_global(GRASS_DISPLAY_FLAG, (!flag) as u8);
    if flag {
        e.call(FN_00B62A20, &args![]);
    } else {
        let holder = e.call(FN_0045C670, &args![]).u32();
        let animation = e.call(FN_006629F0, &args![holder]).u32();
        if animation != 0 {
            let start = [
                e.global::<u32>(START_VECTOR),
                e.global::<u32>(START_VECTOR + 4),
                e.global::<u32>(START_VECTOR + 8),
            ];
            // 005b7156: `0045bb80` takes no stack word (plain RET); the 0.0 and the
            // start vector copied before it are the last four words of the cdecl
            // `0057d0a0` (ADD ESP,0x1c).
            let vector = e.call(FN_0045BB80, &args![animation]).u32();
            let words = [
                e.mem.u32(vector),
                e.mem.u32(vector + 4),
                e.mem.u32(vector + 8),
            ];
            e.call(
                FN_0057D0A0,
                &args![words[0], words[1], words[2], start[0], start[1], start[2], 0.0f32],
            );
        }
    }
    let state = if flag {
        TEXT_DISABLED_CAPITAL
    } else {
        TEXT_ENABLED_CAPITAL
    };
    console_print(e, &args![MSG_GRASS_DISPLAY, state]);
    true
}

// Translated from 005b71b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::TestAllCellsFunction` (Xbox PDB): one optional argument, the
/// number of cells (0, the default, becomes -1), passed to
/// `TES::TestAllCells`; then prints whether the cell test is "running" or
/// "stopped".
pub fn script_test_all_cells_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([mut count]) = a.parse_into(e, [0]) else {
        return false;
    };
    if count == 0 {
        count = u32::MAX;
    }
    let tes = e.global::<u32>(GLOBAL_0011DEA10);
    e.call(TES_TEST_ALL_CELLS, &args![tes, count]);
    let tes = e.global::<u32>(GLOBAL_0011DEA10);
    let state = if e.call(TES_IS_RUNNING_CELL_TESTS, &args![tes]).bool() {
        TEXT_RUNNING
    } else {
        TEXT_STOPPED
    };
    console_print(e, &args![MSG_TEST_ALL_CELLS, state]);
    true
}

/// The body both `RenderTestCell` commands share: renders the player's
/// parent cell with `flag` and prints the outcome.
fn render_test_cell_command(e: &mut Engine, flag: u32) -> bool {
    let player = player(e);
    let cell = e.call(REFR_GET_PARENT_CELL, &args![player]).u32();
    if cell == 0 {
        console_print(e, &args![MSG_RENDER_TEST_CELL_FAILED]);
    } else {
        let cell = e.call(REFR_GET_PARENT_CELL, &args![player]).u32();
        e.call(CELL_RENDER_TEST_CELL, &args![cell, 0u32, flag]);
        console_print(e, &args![MSG_RENDER_TEST_CELL_COMPLETE]);
    }
    true
}

// Translated from 005b7250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The "RenderTestCell" command with the flag 1: `TESObjectCELL::RenderTestCell`
/// on the player's parent cell, or "RenderTestCell failed: no cell".
pub fn fn_005b7250(e: &mut Engine) -> bool {
    render_test_cell_command(e, 1)
}

// Translated from 005b72a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The "RenderTestCell" command with the flag 0 (see [`fn_005b7250`]).
pub fn fn_005b72a0(e: &mut Engine) -> bool {
    render_test_cell_command(e, 0)
}

// Translated from 005b72f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reloads the ini files: for each of `Fallout.ini`, `Custom.ini`
/// (`0044f560` collection) and `FalloutPrefs.ini` (`004de490` collection)
/// whose name pointer is set, builds `<game directory><name>` in a 0x104-byte
/// buffer (`strcpy_s`, `strcat_s`) and hands it to `005e0200` on the
/// collection. Prints that the settings were refreshed. (The stack protector
/// cookie check is left out.)
pub fn fn_005b72f0(e: &mut Engine) -> bool {
    for (name_pointer, collection_getter) in [
        (FALLOUT_INI_NAME, FN_0044F560),
        (CUSTOM_INI_NAME, FN_0044F560),
        (FALLOUT_PREFS_INI_NAME, FN_004DE490),
    ] {
        let name = e.global::<u32>(name_pointer);
        if name != 0 {
            e.with_stack(0x104, |e, path| {
                let directory = e.call(FN_004DC110, &args![]).u32();
                e.call(STRCPY_S, &args![path, 0x104u32, directory]);
                e.call(STRCAT_S, &args![path, 0x104u32, name]);
                let collection = e.call(collection_getter, &args![]).u32();
                e.call(FN_005E0200, &args![collection, path]);
            });
        }
    }
    console_print(e, &args![MSG_INI_REFRESHED]);
    true
}

// Translated from 005b7420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleEmotionsFunction` (Xbox PDB): flips the NPC facial
/// emotions flag and prints "Enabled." or "Disabled.".
pub fn script_toggle_emotions_function(e: &mut Engine) -> bool {
    let flag = fn_005b7470(e);
    fn_005b7480(e, (flag == 0) as u8);
    let state = if fn_005b7470(e) != 0 {
        TEXT_ENABLED_CAPITAL
    } else {
        TEXT_DISABLED_CAPITAL
    };
    console_print(e, &args![MSG_NPC_EMOTIONS, state]);
    true
}

// Translated from 005b7470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The NPC facial emotions flag (byte global `0119b4e0`).
pub fn fn_005b7470(e: &mut Engine) -> u8 {
    e.global::<u8>(EMOTIONS_FLAG)
}

// Translated from 005b7480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the NPC facial emotions flag (byte global `0119b4e0`).
pub fn fn_005b7480(e: &mut Engine, value: u8) {
    e.set_global(EMOTIONS_FLAG, value);
}

/// Reloads a form from the file it came from, as `ReloadCurrentClimate` and
/// `005b7520` do: the form's last file (`TESForm::GetFile(-1)`) made thread
/// safe, opened (`OpenTES(0, 0)`), the form found in it, then virtual slots
/// `0x18` (clear) and `0x20` (load, with the file). Returns false when the
/// form has no file.
fn reload_form_from_file(e: &mut Engine, form: u32) -> bool {
    let file = e.call(FORM_GET_FILE, &args![form, u32::MAX]).u32();
    if file == 0 {
        return false;
    }
    let file = e.call(FILE_GET_THREAD_SAFE_FILE, &args![file]).u32();
    e.call(FILE_OPEN_TES, &args![file, 0u32, 0u32]);
    e.call(FILE_FIND_FORM, &args![file, form]);
    e.vcall(form, 0x18, &args![]);
    e.vcall(form, 0x20, &args![file]);
    true
}

// Translated from 005b7490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ReloadCurrentClimate` (Xbox PDB): reloads the sky's current
/// climate from its file and makes it the sky's current climate again
/// (`Sky::SetCurrentClimate(climate, 1)`).
pub fn script_reload_current_climate(e: &mut Engine) -> bool {
    let sky = e.call(SKY_GET_INSTANCE, &args![]).u32();
    let climate = e.call(SKY_GET_CURRENT_CLIMATE, &args![sky]).u32();
    if reload_form_from_file(e, climate) {
        let sky = e.call(SKY_GET_INSTANCE, &args![]).u32();
        e.call(SKY_SET_CURRENT_CLIMATE, &args![sky, climate, 1u32]);
    }
    true
}

// Translated from 005b7520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reloads the sky's current weather from its file.
pub fn fn_005b7520(e: &mut Engine) -> bool {
    let sky = e.call(SKY_GET_INSTANCE, &args![]).u32();
    let weather = e.call(SKY_GET_CURRENT_WEATHER, &args![sky]).u32();
    reload_form_from_file(e, weather);
    true
}

// Translated from 005b7590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the weather: arguments `weather` and `override`. While the sky has a
/// current weather, the weather becomes the override weather
/// (`Sky + 0x1c`) when `override` is set, otherwise the default weather
/// (`Sky + 0x18`, followed by `0093a7a0(0)` on the player); then, unless the
/// TLS echo flag is set, `0063e860(1)` on the sky. Without a current weather
/// it is `Sky::ForceWeather(weather, override != 0)`.
pub fn fn_005b7590(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([weather, override_flag]) = a.parse_into(e, [0, 0]) else {
        return false;
    };
    let sky = e.call(SKY_GET_INSTANCE, &args![]).u32();
    if e.call(SKY_GET_CURRENT_WEATHER, &args![sky]).u32() != 0 {
        if override_flag != 0 {
            e.mem.set_u32(sky + SKY_OVERRIDE_WEATHER, weather);
        } else {
            e.mem.set_u32(sky + SKY_DEFAULT_WEATHER, weather);
            let player = player(e);
            e.call(FN_0093A7A0, &args![player, 0u32]);
        }
        if !echo_enabled(e) {
            e.call(FN_0063E860, &args![sky, 1u32]);
        }
    } else {
        e.call(SKY_FORCE_WEATHER, &args![sky, weather, override_flag != 0]);
    }
    true
}

// Translated from 005b7660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forces the weather: arguments `weather` and `flag`, passed to
/// `Sky::ForceWeather` on the sky instance.
pub fn fn_005b7660(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([weather, flag]) = a.parse_into(e, [0, 0]) else {
        return false;
    };
    let sky = e.call(SKY_GET_INSTANCE, &args![]).u32();
    e.call(SKY_FORCE_WEATHER, &args![sky, weather, flag != 0]);
    true
}

// Translated from 005b76d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the sky's override weather (`Sky + 0x1c`).
pub fn fn_005b76d0(e: &mut Engine) -> bool {
    let sky = e.call(SKY_GET_INSTANCE, &args![]).u32();
    e.call(SKY_CLEAR_OVERRIDE_WEATHER, &args![sky]);
    true
}

// Translated from 005b76f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ApplyImageSpaceModifier` (Xbox PDB): arguments the modifier form
/// and a strength (default 1.0); calls
/// `ImageSpaceModifierInstanceForm::Trigger(form, strength, 0)`.
pub fn script_apply_image_space_modifier(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([form, strength]) = a.parse_into(e, [0, 1.0f32.to_bits()]) else {
        return false;
    };
    e.call(IMAGE_SPACE_MODIFIER_TRIGGER, &args![form, strength, 0u32]);
    true
}

// Translated from 005b7760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::RemoveImageSpaceModifier` (Xbox PDB): arguments the modifier form
/// and a strength (default 1.0, unused); calls
/// `ImageSpaceModifierInstanceForm::Stop(form)`.
pub fn script_remove_image_space_modifier(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([form, _strength]) = a.parse_into(e, [0, 1.0f32.to_bits()]) else {
        return false;
    };
    e.call(IMAGE_SPACE_MODIFIER_STOP, &args![form]);
    true
}

// Translated from 005b77c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Two arguments: calls `00547750(first, second)` (`thiscall` on the first).
pub fn fn_005b77c0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([first, second]) = a.parse_into(e, [0, 0]) else {
        return false;
    };
    e.call(FN_00547750, &args![first, second]);
    true
}

// Translated from 005b7820 (decompiled, FalloutNV.exe 1.4.0.525)
/// One argument, an object with a list at `+0x18` (or none): sets the image
/// space manager's `+0xb0` to that list's address (0 for none), then calls
/// `00b8b500` on the manager.
pub fn fn_005b7820(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([object]) = a.parse_into(e, [0]) else {
        return false;
    };
    let value = if object != 0 {
        e.call(FORM_LIST_ITEMS, &args![object]).u32()
    } else {
        0
    };
    let manager = e.call(GET_IMAGE_SPACE_MANAGER, &args![]).u32();
    e.call(IMAGE_SPACE_MANAGER_SET_OVERRIDE, &args![manager, value]);
    let manager = e.call(GET_IMAGE_SPACE_MANAGER, &args![]).u32();
    e.call(FN_00B8B500, &args![manager]);
    true
}

// Translated from 005b78a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Draws the seen data of the cells around the player as a debug object (a
/// `NiNode`). In an interior it asks the parent cell's seen data (`00555bc0`)
/// to draw itself at the point `(0, 0, player z)` (virtual slot `4` of the
/// seen data with the node, the point and 1) and, when the cell has a north
/// rotation (`00555ad0` times -1) other than 0, rotates the node by it
/// (`004a0c90` into the identity matrix `011a9448`, `0043fa80`). In an
/// exterior it walks the `uGridsToLoad` squared grid cells of the `TES`
/// (`00457050`) and has each one's seen data draw itself at the cell's
/// corner `(GetDataX << 12, GetDataY << 12, player z)`. The node is finished
/// by [`show_debug_node`]. (The exception frame around the node allocation
/// is left out.)
pub fn fn_005b78a0(e: &mut Engine) -> bool {
    let node = new_ni_node(e);
    let player = player(e);
    let parent_cell = e.call(REFR_GET_PARENT_CELL, &args![player]).u32();
    if e.call(CELL_IS_INTERIOR, &args![parent_cell]).bool() {
        let seen = e.call(CELL_GET_SEEN_DATA, &args![parent_cell]).u32();
        if seen != 0 {
            let position = e.vcall(player, PLAYER_POSITION_SLOT, &args![]).u32();
            let z = e.mem.f32(position + 8);
            with_point(e, 0.0, 0.0, z, |e, point| {
                e.vcall(seen, SEEN_DATA_DRAW_SLOT, &args![node, point, 1u32]);
            });
            let rotation = e.call(CELL_GET_NORTH_ROTATION, &args![parent_cell]).f64();
            let angle = (rotation * e.global::<f64>(DOUBLE_MINUS_ONE)) as f32;
            if f64::from(angle) != e.global::<f64>(DOUBLE_ZERO) {
                e.with_stack(36, |e, matrix| {
                    let identity = e.mem.bytes(MATRIX_011A9448, 36);
                    e.mem.write(matrix.addr(), &identity);
                    e.call(NI_MATRIX_FROM_Z_ANGLE, &args![matrix, angle]);
                    e.call(NODE_SET_LOCAL_ROTATE, &args![node, matrix]);
                });
            }
        }
    } else {
        let mut outer = 0u32;
        while outer < grids_to_load(e) {
            let mut inner = 0u32;
            while inner < grids_to_load(e) {
                let tes = e.global::<u32>(GLOBAL_0011DEA10);
                let slot = e
                    .call(TES_GET_GRID_CELL_SLOT, &args![tes, outer, inner])
                    .u32();
                let cell = e.mem.u32(slot);
                let seen = if cell != 0 {
                    e.call(CELL_GET_SEEN_DATA, &args![cell]).u32()
                } else {
                    0
                };
                if seen != 0 {
                    let position = e.vcall(player, PLAYER_POSITION_SLOT, &args![]).u32();
                    let z = e.mem.f32(position + 8);
                    let y = e.call(CELL_GET_DATA_Y, &args![cell]).i32();
                    let y = float_from_int(y.wrapping_shl(12));
                    let x = e.call(CELL_GET_DATA_X, &args![cell]).i32();
                    let x = float_from_int(x.wrapping_shl(12));
                    with_point(e, x, y, z, |e, point| {
                        e.vcall(seen, SEEN_DATA_DRAW_SLOT, &args![node, point, 1u32]);
                    });
                }
                inner += 1;
            }
            outer += 1;
        }
    }
    show_debug_node(e, node);
    true
}

// ---- TestLocalMap (005b7b40) -------------------------------------------------

/// The numbers `TestLocalMap` fixes in its frame: one tile is 0x40 units
/// wide and has 0x10 x 0x10 quads (0x11 x 0x11 vertices), so a quad is
/// `0x40 / 0x10 = 4` units wide.
const LOCAL_MAP_TILE_SIZE: i32 = 0x40;
const LOCAL_MAP_QUADS_PER_SIDE: i32 = 0x10;
const LOCAL_MAP_QUAD_COUNT: i32 = LOCAL_MAP_QUADS_PER_SIDE * LOCAL_MAP_QUADS_PER_SIDE;
const LOCAL_MAP_QUAD_SIZE: i32 = LOCAL_MAP_TILE_SIZE / LOCAL_MAP_QUADS_PER_SIDE;
const LOCAL_MAP_VERTICES_PER_SIDE: i32 = 0x11;
const LOCAL_MAP_VERTEX_COUNT: u32 =
    (LOCAL_MAP_VERTICES_PER_SIDE * LOCAL_MAP_VERTICES_PER_SIDE) as u32;

/// What the loop body of `TestLocalMap` needs from the command's frame.
struct LocalMapFrame {
    /// The root node the tiles and the marker are attached to.
    node: u32,
    /// The command's argument: when set, the tile colours are the seen values.
    use_seen_values: bool,
    /// The player.
    player: u32,
    /// Address of the player's position copy (x, y, z + 10.0).
    position: u32,
    /// `uGridsToLoad`: tiles per side.
    grid: i32,
    /// Width of all tiles together (`0x40 * grid`).
    extent: i32,
    /// The float `00879d90` returns: the distance of two colour samples.
    scale: f32,
}

// Translated from 005b7b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::TestLocalMap` (Xbox PDB): builds a debug picture of the local map
/// around the player and shows it as a temporary debug object.
///
/// One argument, `use_seen_values`. A root `NiNode` and the player's
/// position (virtual slot `0x1f4`, z raised by 10.0) are taken first. Then
/// for every cell of the `uGridsToLoad` squared grid ([`local_map_tile`])
/// a 17 x 17 vertex tile `NiTriShape` is built, attached to the root and
/// moved to its place, and given the cell's local map texture as the base
/// map of a new `NiTexturingProperty`. Finally a red triangle (the player
/// marker, `MakeTriangle`) is attached at the player's place in the grid,
/// rotated by the cell's north rotation plus `Player + 0x24 + 8`, the root is
/// moved to the player's position and [`show_debug_node`] shows it.
///
/// The exception-unwinding states in the frame are not translated.
pub fn script_test_local_map(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([use_seen_values]) = a.parse_into(e, [0]) else {
        return false;
    };
    let node = new_ni_node(e);
    let player = player(e);
    // The player's position and z + 10.0 (a copy the draw calls take by address).
    let position = e.mem.alloc(12);
    let source = e.vcall(player, PLAYER_POSITION_SLOT, &args![]).u32();
    for i in 0..3 {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(position + 4 * i, word);
    }
    let raised = (f64::from(e.mem.f32(position + 8)) + e.global::<f64>(DOUBLE_TEN)) as f32;
    e.mem.set_f32(position + 8, raised);

    let grid = grids_to_load(e) as i32;
    let frame = LocalMapFrame {
        node,
        use_seen_values: use_seen_values != 0,
        player,
        position,
        grid,
        extent: LOCAL_MAP_TILE_SIZE.wrapping_mul(grid),
        scale: e.call(GET_LOCAL_MAP_SCALE, &args![]).f32(),
    };
    let mut index = 0i32;
    while index < grid.wrapping_mul(grid) {
        local_map_tile(e, &frame, index);
        index += 1;
    }

    // The player marker.
    let marker_centre = e.mem.alloc(12);
    e.call(TRIVIAL_CONSTRUCT, &args![marker_centre]);
    let source = e.vcall(player, PLAYER_POSITION_SLOT, &args![]).u32();
    let player_position = e.mem.alloc(12);
    for i in 0..3 {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(player_position + 4 * i, word);
    }
    if e.call(REFR_GET_INTERIOR, &args![player]).bool() {
        let adjusted = e.mem.alloc(12);
        e.call(TRIVIAL_CONSTRUCT, &args![adjusted]);
        let source = e.vcall(player, PLAYER_POSITION_SLOT, &args![]).u32();
        let cell = e.call(REFR_GET_PARENT_CELL, &args![player]).u32();
        e.call(
            CELL_ADJUST_COORD_FOR_NORTH_ROTATION,
            &args![cell, source, adjusted, 1u32],
        );
        let adjusted_x = e.mem.f32(adjusted);
        let cell_x = (float_to_int(e, adjusted_x).wrapping_sub(0x800)) >> 12;
        let adjusted_y = e.mem.f32(adjusted + 4);
        let cell_y = (float_to_int(e, adjusted_y).wrapping_sub(0x800)) >> 12;
        let centre = make_point(
            e,
            float_from_int(cell_x.wrapping_shl(12).wrapping_add(0x1000)),
            float_from_int(cell_y.wrapping_shl(12).wrapping_add(0x1000)),
            0.0,
        );
        copy_words(e, marker_centre, &centre);
        for i in 0..3 {
            let word = e.mem.u32(adjusted + 4 * i);
            e.mem.set_u32(player_position + 4 * i, word);
        }
        e.mem.free(adjusted);
    } else {
        let source = e.vcall(player, PLAYER_POSITION_SLOT, &args![]).u32();
        let source_x = e.mem.f32(source);
        let cell_x = float_to_int(e, source_x) >> 12;
        let source = e.vcall(player, PLAYER_POSITION_SLOT, &args![]).u32();
        let source_y = e.mem.f32(source + 4);
        let cell_y = float_to_int(e, source_y) >> 12;
        let centre = make_point(
            e,
            float_from_int(cell_x.wrapping_shl(12).wrapping_add(0x800)),
            float_from_int(cell_y.wrapping_shl(12).wrapping_add(0x800)),
            0.0,
        );
        copy_words(e, marker_centre, &centre);
    }
    e.mem.set_f32(player_position + 8, 0.0);
    // The marker's offset from the tile centre, in tile units.
    let offset = e.mem.alloc(12);
    e.call(
        POINT3_SUBTRACT,
        &args![player_position, offset, marker_centre],
    );
    let unit = (f64::from(LOCAL_MAP_TILE_SIZE) / e.global::<f64>(DOUBLE_4096)) as f32;
    for i in 0..2 {
        let scaled = (f64::from(e.mem.f32(offset + 4 * i)) * f64::from(unit)) as f32;
        e.mem.set_f32(offset + 4 * i, scaled);
    }
    e.mem.set_f32(offset + 8, 0.0);
    let (marker_top, marker_right, marker_bottom) = (
        e.global::<f32>(MARKER_TOP_Y),
        e.global::<f32>(MARKER_RIGHT_X),
        e.global::<f32>(MARKER_BOTTOM),
    );
    let top = make_point(e, 0.0, marker_top, 1.0);
    let right = make_point(e, marker_right, marker_bottom, 1.0);
    let left = make_point(e, marker_bottom, marker_bottom, 1.0);
    let colour_block = e.mem.alloc(16);
    let red = e
        .call(
            NI_COLOR_A_CONSTRUCT,
            &args![colour_block, 1.0f32, 0.0f32, 0.0f32, 0.0f32],
        )
        .u32();
    let triangle = e
        .call(
            MAKE_TRIANGLE,
            &args![
                top[0], top[1], top[2], right[0], right[1], right[2], left[0], left[1], left[2],
                red, 1u32
            ],
        )
        .u32();
    e.call(NODE_SET_LOCAL_TRANSLATE, &args![triangle, offset]);
    e.with_stack(36, |e, matrix| {
        let identity = e.mem.bytes(MATRIX_011A9448, 36);
        e.mem.write(matrix.addr(), &identity);
        let cell = e.call(REFR_GET_PARENT_CELL, &args![player]).u32();
        let north = e.call(CELL_GET_NORTH_ROTATION, &args![cell]).f64();
        let data = e.call(FN_00430830, &args![player]).u32();
        let angle = (f64::from(e.mem.f32(data + 8)) + north) as f32;
        e.call(NI_MATRIX_FROM_Z_ANGLE, &args![matrix, angle]);
        e.call(NODE_SET_LOCAL_ROTATE, &args![triangle, matrix]);
    });
    e.vcall(node, NODE_ATTACH_CHILD_SLOT, &args![triangle, 1u32]);
    e.call(NODE_SET_LOCAL_TRANSLATE, &args![node, position]);
    show_debug_node(e, node);
    for block in [
        position,
        marker_centre,
        player_position,
        offset,
        colour_block,
    ] {
        e.mem.free(block);
    }
    true
}

/// Copies the three words of a point to `address`.
fn copy_words(e: &mut Engine, address: u32, words: &[u32; 3]) {
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(address + 4 * i as u32, *word);
    }
}

/// The loop body of `TestLocalMap` for the tile `index` (row-major in the
/// `uGridsToLoad` squared grid): finds the cell, builds the tile's vertex
/// arrays and index array, the `NiTriShape` and the texturing property.
fn local_map_tile(e: &mut Engine, frame: &LocalMapFrame, index: i32) {
    let grid = frame.grid;
    let player = frame.player;
    let divisor = grids_to_load(e);
    let tile_x = index as u32 % divisor;
    let divisor = grids_to_load(e);
    let tile_y = index as u32 / divisor;

    // The tile's texture, a `NiPointer` the game keeps on its stack.
    let texture_slot = e.mem.alloc(4);
    e.call(NI_POINTER_INIT, &args![texture_slot, 0u32]);
    // Where the tile starts in the world; the start vector unless a cell sets it.
    let origin = e.mem.alloc(12);
    for i in 0..3 {
        let word = e.global::<u32>(START_VECTOR + 4 * i);
        e.mem.set_u32(origin + 4 * i, word);
    }
    let half_grid = grid >> 1;
    if e.call(REFR_GET_INTERIOR, &args![player]).bool() {
        let cell = e.call(REFR_GET_PARENT_CELL, &args![player]).u32();
        if cell != 0 {
            let adjusted = e.mem.alloc(12);
            for i in 0..3 {
                let word = e.mem.u32(frame.position + 4 * i);
                e.mem.set_u32(adjusted + 4 * i, word);
            }
            e.call(
                CELL_ADJUST_COORD_FOR_NORTH_ROTATION,
                &args![cell, frame.position, adjusted, 1u32],
            );
            let adjusted_x = e.mem.f32(adjusted);
            let base_x = (float_to_int(e, adjusted_x).wrapping_sub(0x800)) >> 12;
            let adjusted_y = e.mem.f32(adjusted + 4);
            let base_y = (float_to_int(e, adjusted_y).wrapping_sub(0x800)) >> 12;
            e.mem.free(adjusted);
            let cell_x = base_x.wrapping_add(tile_x as i32).wrapping_sub(half_grid);
            let cell_y = base_y.wrapping_add(tile_y as i32).wrapping_sub(half_grid);
            let start = make_point(
                e,
                float_from_int(cell_x.wrapping_shl(12).wrapping_add(0x800)),
                float_from_int(cell_y.wrapping_shl(12).wrapping_add(0x800)),
                0.0,
            );
            copy_words(e, origin, &start);
            e.call(
                CELL_GET_INT_SEEN_SECTION,
                &args![cell, cell_x, cell_y, 0u32],
            );
            e.call(
                CELL_GET_INTERIOR_LOCAL_MAP_TEXTURE,
                &args![cell, cell_x, cell_y, texture_slot],
            );
        }
    } else {
        let world = e.call(REFR_GET_WORLD_SPACE, &args![player]).u32();
        let position_x = e.mem.f32(frame.position);
        let base_x = float_to_int(e, position_x) >> 12;
        let position_y = e.mem.f32(frame.position + 4);
        let base_y = float_to_int(e, position_y) >> 12;
        let cell_x = base_x.wrapping_add(tile_x as i32).wrapping_sub(half_grid);
        let cell_y = base_y.wrapping_add(tile_y as i32).wrapping_sub(half_grid);
        let start = make_point(
            e,
            float_from_int(cell_x.wrapping_shl(12)),
            float_from_int(cell_y.wrapping_shl(12)),
            0.0,
        );
        copy_words(e, origin, &start);
        let mut cell = 0;
        if world != 0 {
            cell = e
                .call(
                    WORLD_SPACE_GET_CELL_FROM_CELL_COORD,
                    &args![world, cell_x, cell_y],
                )
                .u32();
        }
        if cell != 0 {
            e.call(CELL_GET_SEEN_DATA, &args![cell]);
            e.call(FN_0054E640, &args![cell, texture_slot]);
        }
    }

    // The vertex arrays: positions, normals, uvs and colours.
    let positions = new_element_array(e, LOCAL_MAP_VERTEX_COUNT, 12, TRIVIAL_CONSTRUCT);
    let normals = new_element_array(e, LOCAL_MAP_VERTEX_COUNT, 12, TRIVIAL_CONSTRUCT);
    let uvs = new_element_array(e, LOCAL_MAP_VERTEX_COUNT, 8, TRIVIAL_CONSTRUCT);
    let colours = new_element_array(e, LOCAL_MAP_VERTEX_COUNT, 0x10, NI_COLOR_A_ZERO_CONSTRUCT);
    let indices = e
        .call(
            NI_ALLOC_SHORTS,
            &args![((LOCAL_MAP_QUAD_COUNT << 1) * 3) << 1],
        )
        .u32();

    let half_tile = f64::from(LOCAL_MAP_TILE_SIZE) / e.global::<f64>(DOUBLE_TWO);
    let side = LOCAL_MAP_VERTICES_PER_SIDE;
    let mut vertex = 0u32;
    for row in 0..side {
        for column in 0..side {
            let x = (f64::from(column * LOCAL_MAP_QUAD_SIZE) - half_tile) as f32;
            let y = (f64::from(row * LOCAL_MAP_QUAD_SIZE) - half_tile) as f32;
            let point = make_point(e, x, y, 0.0);
            copy_words(e, positions + 12 * vertex, &point);
            vertex += 1;
        }
    }
    let mut vertex = 0u32;
    for row in 0..side {
        for column in 0..side {
            let up = make_point(e, 0.0, 0.0, 1.0);
            copy_words(e, normals + 12 * vertex, &up);
            let v = (1.0 - f64::from(row) / f64::from(side)) as f32;
            let u = (f64::from(column) / f64::from(side)) as f32;
            let uv = e.with_stack(8, |e, block| {
                let result = e.call(NI_POINT2_CONSTRUCT, &args![block, u, v]).u32();
                [e.mem.u32(result), e.mem.u32(result + 4)]
            });
            e.mem.set_u32(uvs + 8 * vertex, uv[0]);
            e.mem.set_u32(uvs + 8 * vertex + 4, uv[1]);
            if !frame.use_seen_values {
                for i in 0..4 {
                    let word = e.global::<u32>(DEFAULT_TILE_COLOUR + 4 * i);
                    e.mem.set_u32(colours + 16 * vertex + 4 * i, word);
                }
            } else {
                let sample_x = (f64::from(column) * f64::from(frame.scale)) as f32;
                let sample_y = (f64::from(row) * f64::from(frame.scale)) as f32;
                let seen = e.with_stack(12, |e, sample| {
                    e.call(
                        NI_POINT3_CONSTRUCT,
                        &args![sample, sample_x, sample_y, 0.0f32],
                    );
                    e.call(POINT3_ADD_ASSIGN, &args![sample, origin]);
                    e.call(CELL_GET_SEEN_VALUE, &args![sample]).i32()
                });
                let value = float_from_int(seen);
                let shade = (f64::from(value) / e.global::<f64>(DOUBLE_FOUR)) as f32;
                let colour = e.with_stack(16, |e, block| {
                    let result = e
                        .call(
                            NI_COLOR_A_CONSTRUCT,
                            &args![block, shade, shade, shade, 0.0f32],
                        )
                        .u32();
                    [
                        e.mem.u32(result),
                        e.mem.u32(result + 4),
                        e.mem.u32(result + 8),
                        e.mem.u32(result + 12),
                    ]
                });
                for (i, word) in colour.iter().enumerate() {
                    e.mem.set_u32(colours + 16 * vertex + 4 * i as u32, *word);
                }
            }
            vertex += 1;
        }
    }

    // The index array: two triangles per quad, the diagonal alternating.
    let mut at = 0u32;
    for row in 0..LOCAL_MAP_QUADS_PER_SIDE {
        for column in 0..LOCAL_MAP_QUADS_PER_SIDE {
            let below = (row + 1) * side;
            let this_row = row * side;
            let corners: [i32; 6] = if row % 2 == column % 2 {
                [
                    below + column + 1,
                    below + column,
                    this_row + column,
                    this_row + column,
                    this_row + column + 1,
                    below + column + 1,
                ]
            } else {
                [
                    below + column,
                    this_row + column,
                    this_row + column + 1,
                    this_row + column + 1,
                    below + column + 1,
                    below + column,
                ]
            };
            for corner in corners {
                e.mem.set_u16(indices + 2 * at, corner as u16);
                at += 1;
            }
        }
    }

    // The shape, attached to the root and moved to the tile's place.
    let block = e.call(NI_ALLOC, &args![0xc4u32]).u32();
    let shape = if block == 0 {
        0
    } else {
        e.call(
            NI_TRI_SHAPE_CONSTRUCT,
            &args![
                block,
                LOCAL_MAP_VERTEX_COUNT & 0xffff,
                positions,
                normals,
                colours,
                uvs,
                1u32,
                0u32,
                (LOCAL_MAP_QUAD_COUNT << 1) as u32,
                indices
            ],
        )
        .u32()
    };
    e.vcall(frame.node, NODE_ATTACH_CHILD_SLOT, &args![shape, 1u32]);
    let half_extent = f64::from(frame.extent) / e.global::<f64>(DOUBLE_TWO);
    let offset = |tile: u32, e: &Engine| {
        let tile_offset = f64::from(LOCAL_MAP_TILE_SIZE.wrapping_mul(tile as i32));
        ((tile_offset + f64::from(LOCAL_MAP_TILE_SIZE) / e.global::<f64>(DOUBLE_TWO)) - half_extent)
            as f32
    };
    let y = offset(tile_y, e);
    let x = offset(tile_x, e);
    e.call(NODE_SET_LOCAL_TRANSLATE_XYZ, &args![shape, x, y, 0.0f32]);

    // The texture, as the base map of a new texturing property.
    if e.call(NI_POINTER_GET, &args![texture_slot]).u32() != 0 {
        let block = e.call(NI_ALLOC, &args![0x30u32]).u32();
        let property = if block == 0 {
            0
        } else {
            e.call(NI_TEXTURING_PROPERTY_CONSTRUCT, &args![block]).u32()
        };
        let texture = e.call(NI_POINTER_GET, &args![texture_slot]).u32();
        fn_005b8fc0(e, Ptr::new(property), texture);
        e.call(
            NI_TEXTURING_PROPERTY_SET_BASE_CLAMP_MODE,
            &args![property, 0u32],
        );
        e.call(FN_00533FB0, &args![property, 2u32]);
        e.call(NI_AV_OBJECT_ATTACH_PROPERTY, &args![shape, property]);
        e.call(NI_POINTER_SET, &args![texture_slot, 0u32]);
    }
    e.call(NI_POINTER_DESTRUCT, &args![texture_slot]);
    e.mem.free(texture_slot);
    e.mem.free(origin);
}

// Translated from 005b8fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the texture of the base map of a texturing property: the first
/// element of the array at `this + 0x1c` (created by `00a69dd0` when it is
/// missing and stored through `0096ae90`), then `004dc540(map, texture)`.
/// (The same shape as the engine map's `NiTexturingProperty::SetBaseClampMode`
/// at `004f3200`.)
pub fn fn_005b8fc0(e: &mut Engine, this: Ptr, texture: u32) {
    let list = this.addr() + 0x1c;
    let slot = e.call(ARRAY_ELEMENT_ADDRESS, &args![list, 0u32]).u32();
    let existing = e.mem.u32(slot);
    let cell = e.mem.alloc(4);
    e.mem.set_u32(cell, existing);
    if existing == 0 {
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let map = if block == 0 {
            0
        } else {
            e.call(TEXTURE_MAP_CONSTRUCT, &args![block]).u32()
        };
        e.mem.set_u32(cell, map);
        e.call(ARRAY_SET_ELEMENT, &args![list, 0u32, cell]);
    }
    let map = e.mem.u32(cell);
    e.call(TEXTURE_MAP_SET_TEXTURE, &args![map, texture]);
    e.mem.free(cell);
}

// Translated from 005b9070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::TestCode` (Xbox PDB): one argument (unused), then asks `004a8bb0`
/// for the memory in use by the loaded area and prints it in megabytes
/// ("M# for loaded area = %.0f MB").
pub fn script_test_code(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.parse_into(e, [0]).is_none() {
        return false;
    }
    e.with_stack(8, |e, outputs| {
        let size = outputs.addr() + 4;
        e.mem.set_f32(outputs.addr(), 0.0);
        e.mem.set_f32(size, 0.0);
        e.call(FN_004A8BB0, &args![outputs, size, 1u32, 0u32]);
        let bytes = e.mem.f32(size);
        let megabytes = (f64::from(bytes) / e.global::<f64>(DOUBLE_MEGABYTE)) as f32;
        e.mem.set_f32(size, megabytes);
        console_print(e, &args![MSG_LOADED_AREA_MEGABYTES, f64::from(megabytes)]);
    });
    true
}

// Translated from 005b9100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleConversationStatsFunction` (Xbox PDB): flips the
/// conversation stats flag and prints "shown." or "hidden.".
pub fn script_toggle_conversation_stats_function(e: &mut Engine) -> bool {
    let flag = fn_005b9160(e);
    fn_005b9150(e, (flag == 0) as u8);
    let state = if fn_005b9160(e) != 0 {
        TEXT_SHOWN
    } else {
        TEXT_HIDDEN
    };
    console_print(e, &args![MSG_CONVERSATION_STATS, state]);
    true
}

// Translated from 005b9150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the conversation stats flag (byte global `011cbeac`).
pub fn fn_005b9150(e: &mut Engine, value: u8) {
    e.set_global(CONVERSATION_STATS_FLAG, value);
}

// Translated from 005b9160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The conversation stats flag (byte global `011cbeac`).
pub fn fn_005b9160(e: &mut Engine) -> u8 {
    e.global::<u8>(CONVERSATION_STATS_FLAG)
}

// Translated from 005b9170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleFullHelp` (Xbox PDB): `Interface::ToggleFullHelp`, then
/// prints "shown." or "hidden." after `Interface::GetFullHelp`.
pub fn script_toggle_full_help(e: &mut Engine) -> bool {
    e.call(INTERFACE_TOGGLE_FULL_HELP, &args![]);
    let state = if e.call(INTERFACE_GET_FULL_HELP, &args![]).bool() {
        TEXT_SHOWN
    } else {
        TEXT_HIDDEN
    };
    console_print(e, &args![MSG_TOGGLE_FULL_HELP, state]);
    true
}

// Translated from 005b91b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleMagicStatsFunction` (Xbox PDB): flips the magic stats flag
/// and prints "shown." or "hidden.".
pub fn script_toggle_magic_stats_function(e: &mut Engine) -> bool {
    let flag = fn_005b9210(e);
    fn_005b9200(e, (flag == 0) as u8);
    let state = if fn_005b9210(e) != 0 {
        TEXT_SHOWN
    } else {
        TEXT_HIDDEN
    };
    console_print(e, &args![MSG_MAGIC_STATS, state]);
    true
}

// Translated from 005b9200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the magic stats flag (byte global `011c3534`).
pub fn fn_005b9200(e: &mut Engine, value: u8) {
    e.set_global(MAGIC_STATS_FLAG, value);
}

// Translated from 005b9210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The magic stats flag (byte global `011c3534`).
pub fn fn_005b9210(e: &mut Engine) -> u8 {
    e.global::<u8>(MAGIC_STATS_FLAG)
}

// Translated from 005b9220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00456a60` on the `TES`.
pub fn fn_005b9220(e: &mut Engine) -> bool {
    let tes = e.global::<u32>(GLOBAL_0011DEA10);
    e.call(FN_00456A60, &args![tes]);
    true
}

// Translated from 005b9240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00456a60` on the `TES` with the byte `012680fc` set to 1 meanwhile.
pub fn fn_005b9240(e: &mut Engine) -> bool {
    e.set_global(FLAG_012680FC, 1u8);
    let tes = e.global::<u32>(GLOBAL_0011DEA10);
    e.call(FN_00456A60, &args![tes]);
    e.set_global(FLAG_012680FC, 0u8);
    true
}

// Translated from 005b9260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleCharControllerShapeFunction` (Xbox PDB): when the command
/// runs on a reference that is an `Actor` (`__RTDynamicCast` from
/// `TESObjectREFR`) with a character controller, switches the controller's
/// shape type (`this + 0x59c`, [`fn_005b92e0`]) between 0 and 1: 0 becomes 1,
/// 1 becomes 0, any other value is set again unchanged.
pub fn script_toggle_char_controller_shape_function(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        return true;
    }
    let actor = e
        .call(
            DYNAMIC_CAST,
            &args![a.this_obj, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
        )
        .u32();
    if actor == 0 {
        return true;
    }
    let controller = e
        .call(MOBILE_OBJECT_GET_CHAR_CONTROLLER, &args![actor])
        .u32();
    if controller == 0 {
        return true;
    }
    let mut shape = fn_005b92e0(e, Ptr::new(controller));
    if shape == 0 {
        shape = 1;
    } else if shape == 1 {
        shape = 0;
    }
    e.call(CHAR_CONTROLLER_SET_SHAPE_TYPE, &args![controller, shape]);
    true
}

// Translated from 005b92e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword at `this + 0x59c` (the shape type of a
/// `bhkCharacterController`, PC offset).
pub fn fn_005b92e0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x59c)
}

// Translated from 005b9300 (decompiled, FalloutNV.exe 1.4.0.525)
/// One string argument (a 0x204-byte buffer), handed to `00456b10` on the
/// `TES`. (The stack protector cookie check is left out.)
pub fn fn_005b9300(e: &mut Engine, a: ScriptArgs) -> bool {
    e.with_stack(0x204, |e, buffer| {
        if !a.parse(e, &[buffer.addr()]) {
            return false;
        }
        let tes = e.global::<u32>(GLOBAL_0011DEA10);
        e.call(FN_00456B10, &args![tes, buffer]);
        true
    })
}

// Translated from 005b9370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleBordersFunction` (Xbox PDB): flips the borders flag; while
/// `005f36f0` of the `TES` is 0, passes the flag to
/// `TES::bShowLANDborders`'s setter (`00456cb0`); echoes "Borders -> On" or
/// "Off" when the TLS echo flag is set.
pub fn script_toggle_borders_function(e: &mut Engine) -> bool {
    let flag = e.global::<u8>(BORDERS_FLAG) ^ 1;
    e.set_global(BORDERS_FLAG, flag);
    let tes = e.global::<u32>(GLOBAL_0011DEA10);
    if e.call(FN_005F36F0, &args![tes]).u32() == 0 {
        let flag = e.global::<u8>(BORDERS_FLAG);
        let tes = e.global::<u32>(GLOBAL_0011DEA10);
        e.call(TES_SET_SHOW_LAND_BORDERS, &args![tes, u32::from(flag)]);
    }
    if echo_enabled(e) {
        let state = if e.global::<u8>(BORDERS_FLAG) != 0 {
            TEXT_ON_CAPITAL
        } else {
            TEXT_OFF_CAPITAL
        };
        console_print(e, &args![MSG_BORDERS, state]);
    }
    true
}

// Translated from 005b9400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleProjectileDebugFunction` (Xbox PDB): flips the projectile
/// debug flag ([`fn_005b9470`]) and, with the TLS echo flag set, prints what
/// the debug lines and diamonds mean.
pub fn script_toggle_projectile_debug_function(e: &mut Engine) -> bool {
    fn_005b9470(e);
    if echo_enabled(e) {
        for message in [
            MSG_PROJECTILE_DEBUG_1,
            MSG_PROJECTILE_DEBUG_2,
            MSG_PROJECTILE_DEBUG_3,
            MSG_PROJECTILE_DEBUG_4,
            MSG_PROJECTILE_DEBUG_5,
        ] {
            console_print(e, &args![message]);
        }
    }
    true
}

// Translated from 005b9470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flips the projectile debug flag (byte global `011f20a4`).
pub fn fn_005b9470(e: &mut Engine) {
    let flag = e.global::<u8>(PROJECTILE_DEBUG_FLAG);
    e.set_global(PROJECTILE_DEBUG_FLAG, (flag == 0) as u8);
}

// Translated from 005b9490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleMenusFunction` (Xbox PDB): flips the menus flag, passes it
/// to `00703810` and, with the TLS echo flag set, prints "Menus -> On" or
/// "Off".
pub fn script_toggle_menus_function(e: &mut Engine) -> bool {
    let flag = e.global::<u8>(MENUS_FLAG) ^ 1;
    e.set_global(MENUS_FLAG, flag);
    let flag = e.global::<u8>(MENUS_FLAG);
    e.call(FN_00703810, &args![u32::from(flag)]);
    if echo_enabled(e) {
        let state = if e.global::<u8>(MENUS_FLAG) != 0 {
            TEXT_ON_CAPITAL
        } else {
            TEXT_OFF_CAPITAL
        };
        console_print(e, &args![MSG_MENUS, state]);
    }
    true
}

// Translated from 005b9500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00456aa0` on the `TES`.
pub fn fn_005b9500(e: &mut Engine) -> bool {
    let tes = e.global::<u32>(GLOBAL_0011DEA10);
    e.call(FN_00456AA0, &args![tes]);
    true
}

// Translated from 005b9520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Interface::ToggleSafeZone(2)`.
pub fn fn_005b9520(e: &mut Engine) -> bool {
    e.call(INTERFACE_TOGGLE_SAFE_ZONE, &args![2u32]);
    true
}

/// "On" or "Off" for a flag, the way the toggles print it (`"On"` for a
/// non-zero flag).
pub(crate) fn on_off(flag: bool) -> u32 {
    if flag {
        TEXT_ON_CAPITAL
    } else {
        TEXT_OFF_CAPITAL
    }
}

/// `Sky::SetMode(mode)` on the sky of the `TES` (`TES::pSky`).
fn set_sky_mode(e: &mut Engine, mode: u32) {
    let tes = e.global::<u32>(GLOBAL_0011DEA10);
    let sky = e.call(TES_GET_SKY, &args![tes]).u32();
    e.call(SKY_SET_MODE, &args![sky, mode]);
}

// Translated from 005b9540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleSkyFunction` (Xbox PDB): flips the sky flag and, when the
/// `TES` exists, sets the mode of its sky: 0 when the flag is now set;
/// otherwise 3 without an interior cell (`005f36f0` of the `TES`), 2 when
/// that cell has bit `0x80` set (`00454b10`) and 1 when it has not. Echoes
/// "Sky -> Off" / "Sky -> On" (flag set / clear) when the TLS echo flag is
/// set.
pub fn script_toggle_sky_function(e: &mut Engine) -> bool {
    let flag = e.global::<u8>(SKY_FLAG) ^ 1;
    e.set_global(SKY_FLAG, flag);
    if e.global::<u32>(GLOBAL_0011DEA10) != 0 {
        if e.global::<u8>(SKY_FLAG) != 0 {
            set_sky_mode(e, 0);
        } else {
            let tes = e.global::<u32>(GLOBAL_0011DEA10);
            if e.call(FN_005F36F0, &args![tes]).u32() == 0 {
                set_sky_mode(e, 3);
            } else {
                let tes = e.global::<u32>(GLOBAL_0011DEA10);
                let cell = e.call(FN_005F36F0, &args![tes]).u32();
                if e.call(CELL_TEST_FLAG_80, &args![cell]).bool() {
                    set_sky_mode(e, 2);
                } else {
                    set_sky_mode(e, 1);
                }
            }
        }
    }
    if echo_enabled(e) {
        let state = on_off(e.global::<u8>(SKY_FLAG) == 0);
        console_print(e, &args![MSG_SKY, state]);
    }
    true
}

// Translated from 005b9640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleCollisionFunction` (Xbox PDB). On a reference: inverts bit
/// `0x10` of its flags (`0050d4a0`, `00693ef0`) and echoes "Ref '<name>'
/// Collision -> Off" when the bit is now set, "On" when clear. Without a
/// reference: `00456be0` on the `TES` and, when echoing, "Collision -> Off" if
/// the global collision flag (`00456c70`) is set, "On" otherwise.
pub fn script_toggle_collision_function(e: &mut Engine, a: ScriptArgs) -> bool {
    if !a.this_obj.is_null() {
        let was_set = e.call(FORM_TEST_FLAG_10, &args![a.this_obj]).bool();
        let now_set = !was_set;
        e.call(FORM_SET_FLAG_10, &args![a.this_obj, u32::from(now_set)]);
        if echo_enabled(e) {
            let state = on_off(!now_set);
            let name = e.call(REFR_GET_NAME_TEXT, &args![a.this_obj]).u32();
            console_print(e, &args![MSG_REF_COLLISION, name, state]);
        }
    } else {
        let tes = e.global::<u32>(GLOBAL_0011DEA10);
        e.call(FN_00456BE0, &args![tes]);
        if echo_enabled(e) {
            let flag = e.call(FN_00456C70, &args![]).bool();
            console_print(e, &args![MSG_COLLISION, on_off(!flag)]);
        }
    }
    true
}

// Translated from 005b9720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleLeavesFunction` (Xbox PDB): sets the leaves flag
/// ([`fn_005b97d0`]) to the opposite of what
/// [`bs_tree_manager_are_leaves_visible`] says and echoes "Leaves -> On" or
/// "Off" for the new state.
pub fn script_toggle_leaves_function(e: &mut Engine) -> bool {
    let visible = bs_tree_manager_are_leaves_visible(e);
    fn_005b97d0(e, u8::from(!visible));
    if echo_enabled(e) {
        let visible = bs_tree_manager_are_leaves_visible(e);
        console_print(e, &args![MSG_LEAVES, on_off(visible)]);
    }
    true
}

// Translated from 005b9790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeManager::AreLeavesVisible` (Xbox PDB): `0054ee20` says yes and the
/// byte at `+0x19` of the object `00664840(1)` returns is set. (The body
/// never reads `this`.)
pub fn bs_tree_manager_are_leaves_visible(e: &mut Engine) -> bool {
    if e.call(FN_0054EE20, &args![]).bool() {
        let manager = e.call(FN_00664840, &args![1u32]).u32();
        e.mem.u8(manager + 0x19) != 0
    } else {
        false
    }
}

// Translated from 005b97d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `visible` in the byte at `+0x19` of the object `00664840(1)`
/// returns (the byte [`bs_tree_manager_are_leaves_visible`] tests).
pub fn fn_005b97d0(e: &mut Engine, visible: u8) {
    let manager = e.call(FN_00664840, &args![1u32]).u32();
    e.mem.set_u8(manager + 0x19, visible);
}

// Translated from 005b97f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleWireframeFunction` (Xbox PDB): flips the wireframe flag,
/// calls `0045c670` (its result is not used), then [`fn_005b9870`] on the
/// object `00450b80(0)` returns, and echoes "Wireframe -> On" or "Off".
pub fn script_toggle_wireframe_function(e: &mut Engine) -> bool {
    let flag = e.global::<u8>(WIREFRAME_FLAG) ^ 1;
    e.set_global(WIREFRAME_FLAG, flag);
    let _unused = e.call(FN_0045C670, &args![]).u32();
    let target = e.call(FN_00450B80, &args![0u32]).u32();
    fn_005b9870(e, Ptr::new(target));
    if echo_enabled(e) {
        let state = on_off(e.global::<u8>(WIREFRAME_FLAG) != 0);
        console_print(e, &args![MSG_WIREFRAME, state]);
    }
    true
}

// Translated from 005b9870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flips the byte at `this + 0x131` (0 becomes 1, anything else 0).
pub fn fn_005b9870(e: &mut Engine, this: Ptr) {
    let value = e.mem.u8(this.addr() + 0x131);
    e.mem.set_u8(this.addr() + 0x131, u8::from(value == 0));
}

// Translated from 005b98a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleDetectionStats` (Xbox PDB): flips the detection stats flag
/// (before the arguments are read), parses one integer into the dword
/// `011f1224` and echoes "AI Detection stats printing is  On" or "Off".
pub fn script_toggle_detection_stats(e: &mut Engine, a: ScriptArgs) -> bool {
    let flag = e.global::<u8>(DETECTION_STATS_FLAG);
    e.set_global(DETECTION_STATS_FLAG, u8::from(flag == 0));
    let Some([value]) = a.parse_into(e, [0]) else {
        return false;
    };
    e.set_global(DETECTION_STATS_VALUE, value);
    if echo_enabled(e) {
        let state = on_off(e.global::<u8>(DETECTION_STATS_FLAG) != 0);
        console_print(e, &args![MSG_DETECTION_STATS, state]);
    }
    true
}

/// `00404010(0040ebd0(value, 1.0), 0.0)`: the value limited to 0..1.
fn limit_unit(e: &mut Engine, value: f32) -> f32 {
    let limited = e.call(FLOAT_MIN, &args![value, 1.0f32]).f32();
    e.call(FLOAT_MAX, &args![limited, 0.0f32]).f32()
}

// Translated from 005b9950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetTargetRefraction` (Xbox PDB): when [`fn_005b9b00`] allows it,
/// parses one `float`, limits it to 0..1 and sets the refraction of the
/// reference (the player when none is given): through slot `0x384` of an
/// `Actor` (`enable, power`), otherwise on the shader property of the object
/// slot `0x1d0` returns (`SetRefractionRecurse` three times: cleared, fire
/// cleared, then the power). `enable` is "power > 0". Echoes "<name>
/// refraction has been set to <power>".
pub fn script_set_target_refraction(e: &mut Engine, a: ScriptArgs) -> bool {
    if !fn_005b9b00(e) {
        return true;
    }
    let Some([power]) = a.parse_into(e, [0.0f32.to_bits()]) else {
        return false;
    };
    let mut this_obj = a.this_obj;
    if this_obj.is_null() {
        this_obj = Ptr::new(player(e));
    }
    let power = limit_unit(e, f32::from_bits(power));
    let enable = f64::from(power) > e.global::<f64>(DOUBLE_ZERO);
    let actor = e
        .call(
            DYNAMIC_CAST,
            &args![this_obj, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
        )
        .u32();
    if actor != 0 {
        e.vcall(actor, ACTOR_SET_REFRACTION_SLOT, &args![enable, power]);
    } else {
        let property = e
            .vcall(this_obj.addr(), REFR_GET_TARGET_SLOT, &args![])
            .u32();
        if property != 0 {
            e.call(
                SET_REFRACTION_RECURSE,
                &args![property, 0u32, 0.0f32, 0u32, 0.0f32, 0u32],
            );
            e.call(
                SET_REFRACTION_RECURSE,
                &args![property, 0u32, 0.0f32, 1u32, 0.0f32, 0u32],
            );
            e.call(
                SET_REFRACTION_RECURSE,
                &args![property, enable, power, 0u32, 0.0f32, 0u32],
            );
        }
    }
    if echo_enabled(e) {
        let name = e.call(REFR_GET_NAME_TEXT, &args![this_obj]).u32();
        console_print(e, &args![MSG_REFRACTION_SET, name, f64::from(power)]);
    }
    true
}

// Translated from 005b9b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the refraction commands may run: the byte `004dc0a0` returns is set
/// and `004dc060(0)` is at least 2. (The body never reads `this`.)
pub fn fn_005b9b00(e: &mut Engine) -> bool {
    e.call(FN_004DC0A0, &args![]).bool() && e.call(FN_004DC060, &args![0u32]).i32() >= 2
}

// Translated from 005b9b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetTargetRefractionFire` (Xbox PDB): when [`fn_005b9b00`] allows
/// it, parses a `float` and an integer and, for a reference that is not an
/// `Actor`, limits the power to 0..1 and calls `SetRefractionRecurse` on the
/// shader property slot `0x1d0` returns (`enable = power > 0`, fire flag 1,
/// the integer as the fire period). Echoes "<name> refraction fire has been
/// set to <power>, period of <period>" when it did.
pub fn script_set_target_refraction_fire(e: &mut Engine, a: ScriptArgs) -> bool {
    if !fn_005b9b00(e) {
        return true;
    }
    let Some([power, period]) = a.parse_into(e, [0.0f32.to_bits(), 0]) else {
        return false;
    };
    let actor = e
        .call(
            DYNAMIC_CAST,
            &args![a.this_obj, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
        )
        .u32();
    if actor == 0 {
        let power = limit_unit(e, f32::from_bits(power));
        let enable = f64::from(power) > e.global::<f64>(DOUBLE_ZERO);
        let property = e
            .vcall(a.this_obj.addr(), REFR_GET_TARGET_SLOT, &args![])
            .u32();
        if property != 0 {
            let fire_period = float_from_int(period as i32);
            e.call(
                SET_REFRACTION_RECURSE,
                &args![property, enable, power, 1u32, fire_period, 0u32],
            );
            if echo_enabled(e) {
                let name = e.call(REFR_GET_NAME_TEXT, &args![a.this_obj]).u32();
                console_print(
                    e,
                    &args![MSG_REFRACTION_FIRE_SET, name, f64::from(power), period],
                );
            }
        }
    }
    true
}

// Translated from 005b9c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetTargetFalloff` (Xbox PDB): without a reference only echoes "No
/// Reference Selected". Otherwise starts from the colour `011a9be0 * -1.0`
/// (all channels -1.0, "unset"), parses four `float`s into its channels and
/// converts every channel that was given: the first two are angles (limited
/// to 0..90 degrees, to radians, then through [`fn_005b9e80`]), the last two
/// are limited to 0..1. The colour goes to `SetFalloffRecurse` on the shader
/// property slot `0x1d0` returns.
pub fn script_set_target_falloff(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        if echo_enabled(e) {
            console_print(e, &args![MSG_NO_REFERENCE]);
        }
        return true;
    }
    let scalar = e.global::<f32>(FLOAT_MINUS_ONE);
    e.with_stack(16, |e, colour| {
        let base = colour.addr();
        e.call(COLOUR_SCALE, &args![FALLOFF_BASE_COLOUR, colour, scalar]);
        if !a.parse(e, &[base, base + 4, base + 8, base + 12]) {
            return false;
        }
        for channel in 0..4u32 {
            let address = base + 4 * channel;
            let value = e.mem.f32(address);
            if f64::from(value) == e.global::<f64>(DOUBLE_MINUS_ONE) {
                continue;
            }
            let is_angle = channel < 2;
            let upper = if is_angle {
                e.global::<f32>(FALLOFF_MAX_ANGLE)
            } else {
                1.0
            };
            let limited = e.call(FLOAT_MIN, &args![value, upper]).f32();
            let mut result = e.call(FLOAT_MAX, &args![limited, 0.0f32]).f32();
            if is_angle {
                let factor = e.global::<f64>(DOUBLE_DEGREES_TO_RADIANS);
                let radians = (f64::from(result) * factor) as f32;
                result = fn_005b9e80(e, radians);
            }
            e.mem.set_f32(address, result);
        }
        let property = e
            .vcall(a.this_obj.addr(), REFR_GET_TARGET_SLOT, &args![])
            .u32();
        if property != 0 {
            e.call(SET_FALLOFF_RECURSE, &args![property, colour]);
        }
        true
    })
}

// Translated from 005b9e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands its `float` to `004e4470` and returns that function's `float`
/// result.
pub fn fn_005b9e80(e: &mut Engine, value: f32) -> f32 {
    e.call(FN_004E4470, &args![value]).f32()
}

// Translated from 005b9ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetTargetDOF` (Xbox PDB): only echoes "This function is no
/// longer supported".
pub fn script_set_target_dof(e: &mut Engine) -> bool {
    if echo_enabled(e) {
        console_print(e, &args![MSG_NO_LONGER_SUPPORTED]);
    }
    true
}

// Translated from 005b9ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `005d4a40` (which does nothing but return 1) and returns true.
pub fn fn_005b9ed0(e: &mut Engine) -> bool {
    e.call(FN_005D4A40, &args![]);
    true
}

// Translated from 005b9ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses one `float` and hands it to [`fn_005b9f50`] on the singleton
/// `004e3270` returns.
pub fn fn_005b9ef0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = a.parse_into(e, [0.0f32.to_bits()]) else {
        return false;
    };
    let manager = e.call(GET_IMAGE_SPACE_MANAGER, &args![]).u32();
    fn_005b9f50(e, Ptr::new(manager), f32::from_bits(value));
    true
}

// Translated from 005b9f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes element 12 of the array at `this + 4` (`004ebbc0(this, 0xc)`) and
/// gives it `value` ([`fn_005b9f80`]).
pub fn fn_005b9f50(e: &mut Engine, this: Ptr, value: f32) {
    let element = e.call(IMAGE_SPACE_ARRAY_GET, &args![this, 0xcu32]).u32();
    fn_005b9f80(e, Ptr::new(element), value);
}

// Translated from 005b9f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `this + 5` and stores `value` in the `float` at
/// `this + 0x58`.
pub fn fn_005b9f80(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_u8(this.addr() + 5, 1);
    e.mem.set_f32(this.addr() + 0x58, x87_quiet(value));
}

// Translated from 005b9fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a reference: `BSShaderProperty::ToggleMotionBlurRecurse` on the shader
/// property slot `0x1d0` returns, when there is one.
pub fn fn_005b9fa0(e: &mut Engine, a: ScriptArgs) -> bool {
    if !a.this_obj.is_null() {
        let property = e
            .vcall(a.this_obj.addr(), REFR_GET_TARGET_SLOT, &args![])
            .u32();
        if property != 0 {
            e.call(TOGGLE_MOTION_BLUR_RECURSE, &args![property]);
        }
    }
    true
}

// Translated from 005b9fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleFootIK` (Xbox PDB). Without a reference: inverts the global
/// foot IK flag (`00495580`, [`fn_005ba110`]) and echoes "FootIK On" / "Off".
/// On an `Actor` that has the IK object (`+0xac`, PC offset): inverts its
/// per-actor flag ([`fn_005ba180`], [`fn_005ba130`]) and echoes "FootIK on
/// REF <name> On" / "Off".
pub fn script_toggle_foot_ik(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        let flag = e.call(FOOT_IK_GET, &args![]).u8();
        fn_005ba110(e, flag ^ 1);
        if echo_enabled(e) {
            let flag = e.call(FOOT_IK_GET, &args![]).u8();
            console_print(e, &args![MSG_FOOT_IK, on_off(flag != 0)]);
        }
        return true;
    }
    let actor = e
        .call(
            DYNAMIC_CAST,
            &args![a.this_obj, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
        )
        .u32();
    if actor != 0 && e.mem.u32(actor + ACTOR_IK_DATA) != 0 {
        let ik = Ptr::new(e.mem.u32(actor + ACTOR_IK_DATA));
        let flag = fn_005ba180(e, ik);
        fn_005ba130(e, ik, flag ^ 1);
        if echo_enabled(e) {
            let ik = Ptr::new(e.mem.u32(actor + ACTOR_IK_DATA));
            let state = on_off(fn_005ba180(e, ik) != 0);
            let name = e.vcall(a.this_obj.addr(), REFR_NAME_SLOT, &args![]).u32();
            console_print(e, &args![MSG_FOOT_IK_REF, name, state]);
        }
    }
    true
}

// Translated from 005ba110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the value byte of the global foot IK object
/// ([`FOOT_IK_OBJECT`], through `00408d60`).
pub fn fn_005ba110(e: &mut Engine, value: u8) {
    let address = e.call(GLOBAL_VALUE_ADDRESS, &args![FOOT_IK_OBJECT]).u32();
    e.mem.set_u8(address, value);
}

// Translated from 005ba130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `this + 0x220` (the per-actor foot IK flag) to 1 when
/// the byte at `this + 0x21f` and `value` are both non-zero, otherwise 0.
pub fn fn_005ba130(e: &mut Engine, this: Ptr, value: u8) {
    let allowed = e.mem.u8(this.addr() + 0x21f) != 0;
    e.mem
        .set_u8(this.addr() + 0x220, u8::from(allowed && value != 0));
}

// Translated from 005ba180 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0x220` (the per-actor foot IK flag).
pub fn fn_005ba180(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x220)
}

// Translated from 005ba1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleGrabIK` (Xbox PDB): as [`script_toggle_foot_ik`] for the
/// grab IK flags ([`fn_005ba2d0`], [`fn_005ba2f0`] globally; `+0x21c` and
/// `+0x21d` of the IK object per actor); echoes "GrabIK <state>" or
/// "GrabIK on REF <name> <state>".
pub fn script_toggle_grab_ik(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        let flag = fn_005ba2d0(e);
        fn_005ba2f0(e, flag ^ 1);
        if echo_enabled(e) {
            let flag = fn_005ba2d0(e);
            console_print(e, &args![MSG_GRAB_IK, on_off(flag != 0)]);
        }
        return true;
    }
    let actor = e
        .call(
            DYNAMIC_CAST,
            &args![a.this_obj, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
        )
        .u32();
    if actor != 0 && e.mem.u32(actor + ACTOR_IK_DATA) != 0 {
        let ik = Ptr::new(e.mem.u32(actor + ACTOR_IK_DATA));
        let flag = fn_005ba360(e, ik);
        fn_005ba310(e, ik, flag ^ 1);
        if echo_enabled(e) {
            let ik = Ptr::new(e.mem.u32(actor + ACTOR_IK_DATA));
            let state = on_off(fn_005ba360(e, ik) != 0);
            let name = e.vcall(a.this_obj.addr(), REFR_NAME_SLOT, &args![]).u32();
            console_print(e, &args![MSG_GRAB_IK_REF, name, state]);
        }
    }
    true
}

// Translated from 005ba2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value byte of the global grab IK object ([`GRAB_IK_OBJECT`]).
pub fn fn_005ba2d0(e: &mut Engine) -> u8 {
    let address = e.call(GLOBAL_VALUE_ADDRESS, &args![GRAB_IK_OBJECT]).u32();
    e.mem.u8(address)
}

// Translated from 005ba2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the value byte of the global grab IK object.
pub fn fn_005ba2f0(e: &mut Engine, value: u8) {
    let address = e.call(GLOBAL_VALUE_ADDRESS, &args![GRAB_IK_OBJECT]).u32();
    e.mem.set_u8(address, value);
}

// Translated from 005ba310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `this + 0x21d` (the per-actor grab IK flag) to 1 when
/// the byte at `this + 0x21c` and `value` are both non-zero, otherwise 0.
pub fn fn_005ba310(e: &mut Engine, this: Ptr, value: u8) {
    let allowed = e.mem.u8(this.addr() + 0x21c) != 0;
    e.mem
        .set_u8(this.addr() + 0x21d, u8::from(allowed && value != 0));
}

// Translated from 005ba360 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0x21d` (the per-actor grab IK flag).
pub fn fn_005ba360(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x21d)
}

// Translated from 005ba380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleLookIK` (Xbox PDB): as [`script_toggle_foot_ik`] for the
/// look IK flags ([`fn_005ba4b0`], [`fn_005ba4d0`] globally; `+0xb0` and
/// `+0xb1` of the IK object per actor); echoes "LookIK <state>" or "LookIK on
/// REF <name> <state>".
pub fn script_toggle_look_ik(e: &mut Engine, a: ScriptArgs) -> bool {
    if a.this_obj.is_null() {
        let flag = fn_005ba4b0(e);
        fn_005ba4d0(e, flag ^ 1);
        if echo_enabled(e) {
            let flag = fn_005ba4b0(e);
            console_print(e, &args![MSG_LOOK_IK, on_off(flag != 0)]);
        }
        return true;
    }
    let actor = e
        .call(
            DYNAMIC_CAST,
            &args![a.this_obj, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
        )
        .u32();
    if actor != 0 && e.mem.u32(actor + ACTOR_IK_DATA) != 0 {
        let ik = Ptr::new(e.mem.u32(actor + ACTOR_IK_DATA));
        let flag = fn_005ba540(e, ik);
        fn_005ba4f0(e, ik, flag ^ 1);
        if echo_enabled(e) {
            let ik = Ptr::new(e.mem.u32(actor + ACTOR_IK_DATA));
            let state = on_off(fn_005ba540(e, ik) != 0);
            let name = e.vcall(a.this_obj.addr(), REFR_NAME_SLOT, &args![]).u32();
            console_print(e, &args![MSG_LOOK_IK_REF, name, state]);
        }
    }
    true
}

// Translated from 005ba4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value byte of the global look IK object ([`LOOK_IK_OBJECT`]).
pub fn fn_005ba4b0(e: &mut Engine) -> u8 {
    let address = e.call(GLOBAL_VALUE_ADDRESS, &args![LOOK_IK_OBJECT]).u32();
    e.mem.u8(address)
}

// Translated from 005ba4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the value byte of the global look IK object.
pub fn fn_005ba4d0(e: &mut Engine, value: u8) {
    let address = e.call(GLOBAL_VALUE_ADDRESS, &args![LOOK_IK_OBJECT]).u32();
    e.mem.set_u8(address, value);
}

// Translated from 005ba4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `this + 0xb1` (the per-actor look IK flag) to 1 when
/// the byte at `this + 0xb0` and `value` are both non-zero, otherwise 0.
pub fn fn_005ba4f0(e: &mut Engine, this: Ptr, value: u8) {
    let allowed = e.mem.u8(this.addr() + 0xb0) != 0;
    e.mem
        .set_u8(this.addr() + 0xb1, u8::from(allowed && value != 0));
}

// Translated from 005ba540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0xb1` (the per-actor look IK flag).
pub fn fn_005ba540(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xb1)
}

// Translated from 005ba560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleRagdollAnim` (Xbox PDB): inverts the ragdoll animation
/// flag (`005524d0` reads it, `00456c90` sets it) and echoes "Ragdoll
/// Animation On" / "Off".
pub fn script_toggle_ragdoll_anim(e: &mut Engine) -> bool {
    let flag = e.call(RAGDOLL_ANIM_GET, &args![]).u8();
    e.call(RAGDOLL_ANIM_SET, &args![u32::from(flag) ^ 1]);
    if echo_enabled(e) {
        let flag = e.call(RAGDOLL_ANIM_GET, &args![]).u8();
        console_print(e, &args![MSG_RAGDOLL_ANIMATION, on_off(flag != 0)]);
    }
    true
}

// Translated from 005ba5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetVelFunction` (Xbox PDB): parses an axis letter and a `float`;
/// for a reference builds the vector (0, 0, 0), puts the value in the
/// component named by the letter ('X', 'Y' or 'Z'; any other letter leaves
/// the vector zero) and gives it to `TESHavokUtilities::AddVelocity` with the
/// object slot `0x1d0` returns (and 1).
pub fn script_set_vel_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([axis, value]) = a.parse_into(e, [0, 0.0f32.to_bits()]) else {
        return false;
    };
    if !a.this_obj.is_null() {
        with_point(e, 0.0, 0.0, 0.0, |e, vector| {
            match axis as u8 {
                b'X' => e.mem.set_u32(vector, value),
                b'Y' => e.mem.set_u32(vector + 4, value),
                b'Z' => e.mem.set_u32(vector + 8, value),
                _ => {}
            }
            let target = e
                .vcall(a.this_obj.addr(), REFR_GET_TARGET_SLOT, &args![])
                .u32();
            e.call(ADD_VELOCITY, &args![target, vector, 1u32]);
        });
    }
    true
}

// Translated from 005ba690 (decompiled, FalloutNV.exe 1.4.0.525)
/// When [`fn_005b9b00`] allows it, flips the byte `011f9fc2`
/// ([`fn_005ba6b0`]).
pub fn fn_005ba690(e: &mut Engine) -> bool {
    if fn_005b9b00(e) {
        fn_005ba6b0(e);
    }
    true
}

// Translated from 005ba6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flips the byte `011f9fc2` (0 becomes 1, anything else 0).
pub fn fn_005ba6b0(e: &mut Engine) {
    let flag = e.global::<u8>(FLAG_011F9FC2);
    e.set_global(FLAG_011F9FC2, u8::from(flag == 0));
}

// Translated from 005ba6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleLiteBriteFunction` (Xbox PDB): flips the Lite Brite flag
/// ([`fn_005ba740`], [`fn_005ba750`]) and echoes "Lite Brite -> On" or "Off".
pub fn script_toggle_lite_brite_function(e: &mut Engine) -> bool {
    let flag = fn_005ba740(e);
    fn_005ba750(e, u8::from(flag == 0));
    if echo_enabled(e) {
        let flag = fn_005ba740(e);
        console_print(e, &args![MSG_LITE_BRITE, on_off(flag != 0)]);
    }
    true
}

// Translated from 005ba740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The Lite Brite flag (byte global `011f91a7`).
pub fn fn_005ba740(e: &mut Engine) -> u8 {
    e.global::<u8>(LITE_BRITE_FLAG)
}

// Translated from 005ba750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the Lite Brite flag (byte global `011f91a7`).
pub fn fn_005ba750(e: &mut Engine, value: u8) {
    e.set_global(LITE_BRITE_FLAG, value);
}

// ---- Fourth batch (`005ba760` onward): callees, globals and strings ----------

// The debug text and the scene graph windows.
/// `Interface::ToggleDebugTextVisible` (Xbox PDB), `cdecl`, no arguments (it
/// never reads `ECX`).
pub(crate) const INTERFACE_TOGGLE_DEBUG_TEXT_VISIBLE: u32 = 0x0070_3000;
/// `Interface::IsDebugTextVisible` (Xbox PDB), `cdecl`: `AL`.
pub(crate) const INTERFACE_IS_DEBUG_TEXT_VISIBLE: u32 = 0x0070_2fc0;
/// The debug text table (`0x1120` bytes), created on first use by
/// [`FN_00802A90`]; zero until then.
pub(crate) const DEBUG_TEXT_TABLE: u32 = 0x011d_b800;
/// `cdecl`, no arguments: creates the object [`DEBUG_TEXT_TABLE`] points to
/// (`new` of 0x1120 bytes and its constructor `008027d0`) when it is missing.
pub(crate) const FN_00802A90: u32 = 0x0080_2a90;
/// `thiscall` on the debug text table (`RET 4`: `callback`): builds the
/// table's names in rows of up to five, 0x44-byte entries from `this + 0x20`
/// (separated by the string at `01012630`) and calls `callback(row text)` for
/// each row.
pub(crate) const DEBUG_TEXT_TABLE_LIST: u32 = 0x0080_2880;
/// `thiscall` on the debug text table (`RET 8`: `name, value`): looks the name
/// up (`00802a20`); when found stores the index at `this + 0x18` and `value`
/// at `this + 0x1c`. Returns whether the name was found.
pub(crate) const DEBUG_TEXT_TABLE_SELECT: u32 = 0x0080_29c0;
/// Import slot of `WritePrivateProfileStringA` (`stdcall`: section, key, value,
/// file name).
pub(crate) const WRITE_PRIVATE_PROFILE_STRING: u32 = 0x00fd_f0bc;
/// The scene graph window callback `005ba8f0` and `005bac90` store with
/// [`fn_005bac80`].
pub(crate) const SCENE_GRAPH_CALLBACK: u32 = 0x004d_78c0;
/// Global stored by [`fn_005bac80`].
pub(crate) const SCENE_GRAPH_CALLBACK_GLOBAL: u32 = 0x011c_6f18;
/// `thiscall` on the singleton in [`GLOBAL_0011DEA0C`]: `*(this + 8)`.
pub(crate) const FN_0044DDC0: u32 = 0x0044_ddc0;
/// `thiscall` on the singleton in [`GLOBAL_0011DEA0C`]: `*(this + 0x0c)` (the
/// engine map and [`SKY_GET_CURRENT_CLIMATE`] name this body for a `Sky`; the
/// code is the same).
pub(crate) const FN_0084E3A0: u32 = 0x0084_e3a0;
/// `thiscall` constructor of a scene graph window object (0x30 bytes, `RET
/// 0x20`): (`value +0 of the window`, `value +4`, `root object`, `title`,
/// `x`, `y`, `width`, `height`); returns `this`.
pub(crate) const SCENE_GRAPH_WINDOW_CONSTRUCT: u32 = 0x004d_61b0;
/// `thiscall` on a scene graph window (`RET 8`: `object, title`): adds an
/// object to the window's tree (the title is used when it is non-zero).
pub(crate) const SCENE_GRAPH_WINDOW_ADD_OBJECT: u32 = 0x004d_64c0;
/// `thiscall` on a scene graph window (`RET 4`: `object`): fills the window
/// from a reference's object.
pub(crate) const SCENE_GRAPH_WINDOW_FILL: u32 = 0x004d_65a0;
/// `thiscall` on the player: the byte at `this + 0x64a` (the third-person
/// flag).
pub(crate) const PLAYER_IS_THIRD_PERSON: u32 = 0x004e_af60;
/// `thiscall` on the player (`RET 4`: `flag`): a `NiPointer` read at `this +
/// 0x694` when the flag is non-zero, otherwise the result of `0043fcd0`.
pub(crate) const PLAYER_GET_PERSON_OBJECT: u32 = 0x0095_0bb0;
/// `thiscall` on the player: the `NiPointer` read at `this + 0x69c` (the
/// inventory menu object).
pub(crate) const PLAYER_GET_INVENTORY_MENU_OBJECT: u32 = 0x0049_0f80;
/// `thiscall` on an object: `this + 0x9c` handed to `009938b0`.
pub(crate) const FN_00453470: u32 = 0x0045_3470;
/// `NiPointer` slot globals read through [`NI_POINTER_GET`] by the scene graph
/// commands.
pub(crate) const SLOT_SPLATTER_SCENE: u32 = 0x011c_786c;
pub(crate) const SLOT_INTERFACE_SCENE: u32 = 0x011d_ee84;
pub(crate) const SLOT_INTERFACE_3D_SCENE: u32 = 0x011d_eca4;
/// Virtual slot of an object (no arguments) the viewer command tests: non-zero
/// when the object is a node.
pub(crate) const OBJECT_IS_NODE_SLOT: u32 = 0xc;
/// Virtual slot of an object (`collector`): collects the object's strings into
/// the collector.
pub(crate) const OBJECT_COLLECT_STRINGS_SLOT: u32 = 0x60;
/// `thiscall` constructor of a string collector (`RET 8`: `0, 1`).
pub(crate) const STRING_COLLECTOR_CONSTRUCT: u32 = 0x004d_9100;
/// `thiscall` on a node: the number of children.
pub(crate) const NODE_CHILD_COUNT: u32 = 0x0043_b480;
/// `thiscall` on a node (`index`): the child at `index`.
pub(crate) const NODE_CHILD_AT: u32 = 0x0043_b4a0;
/// `thiscall` on a string collector: the number of collected strings.
pub(crate) const STRING_COLLECTOR_COUNT: u32 = 0x0065_8930;
/// `thiscall` on a string collector (`index`): the address of the string
/// pointer at `index` (`*(this + 4) + index * 4`).
pub(crate) const STRING_COLLECTOR_STRING_ADDRESS: u32 = 0x0087_7a30;
/// `thiscall` on a string collector: clears its entries and its counts
/// (`005e03d0`, a function of this unit outside this range).
pub(crate) const STRING_COLLECTOR_CLEAR: u32 = 0x005e_03d0;
/// `thiscall` on a node: an attached object (zero when none).
pub(crate) const NODE_GET_ATTACHED_OBJECT: u32 = 0x0068_38b0;
/// `cdecl` (`type descriptor, object`): the object cast to that type (zero
/// when it does not convert).
pub(crate) const FN_00653270: u32 = 0x0065_3270;
/// The type descriptor passed to [`FN_00653270`].
pub(crate) const TYPE_012043F8: u32 = 0x0120_43f8;
/// `thiscall` on the object [`FN_00653270`] returns: the object whose strings
/// are collected next.
pub(crate) const FN_006FA820: u32 = 0x006f_a820;
/// `TES::GetWorldSpace` (Xbox PDB), `thiscall` on the `TES`: `*(this + 0x88)`.
pub(crate) const TES_GET_WORLD_SPACE: u32 = 0x004f_d3e0;
/// `TESWorldSpace::GetTerrainManager` (Xbox PDB), `thiscall`.
pub(crate) const WORLD_SPACE_GET_TERRAIN_MANAGER: u32 = 0x0058_6170;
/// A wrapper of `_stricmp` (`00ec6da0`), `cdecl` (`a, b`): zero when the two
/// strings are equal, ignoring case.
pub(crate) const COMPARE_STRINGS_NO_CASE: u32 = 0x0040_8b20;
/// `thiscall` on any object (ignores `this`): the trees LOD object, a
/// `NiPointer` read of the global `011d86bc`.
pub(crate) const LOD_TREES_OBJECT: u32 = 0x004e_a9e0;
/// As [`LOD_TREES_OBJECT`] for the land LOD object.
pub(crate) const LOD_LAND_OBJECT: u32 = 0x004e_a9a0;
/// As [`LOD_TREES_OBJECT`] for the objects LOD object.
pub(crate) const LOD_OBJECTS_OBJECT: u32 = 0x004e_a9c0;
/// As [`LOD_TREES_OBJECT`] for the water LOD object.
pub(crate) const LOD_WATER_OBJECT: u32 = 0x0043_b230;
/// `thiscall` on a LOD object: the flag the toggle reads (`00456630(this,
/// 1)`), `AL`.
pub(crate) const LOD_OBJECT_GET_FLAG: u32 = FN_00456610;
/// `thiscall` on a LOD object (`flag`): `0043b370(this, flag, 1)`.
pub(crate) const LOD_OBJECT_SET_FLAG: u32 = FN_00450F90;
/// `thiscall` on a setting (`float`, `RET 4`): sets the setting's value
/// (`004de290`) and returns `this`.
pub(crate) const SETTING_SET_FLOAT: u32 = 0x004e_d780;
/// The two settings `SetLODObjectDistance` sets.
pub(crate) const SETTING_LOD_DISTANCE_NEAR: u32 = 0x011d_8724;
pub(crate) const SETTING_LOD_DISTANCE_FAR: u32 = 0x011d_877c;
/// `thiscall` (`0`) on the object [`FN_0045C670`] returns: forwards to
/// `0045bc00`.
pub(crate) const FN_00558310: u32 = 0x0055_8310;
/// `thiscall`: `this + 0x58`, the address of a position vector.
pub(crate) const FN_0043C490: u32 = 0x0043_c490;
/// `BGSTerrainManager::Update` (Xbox PDB), `thiscall` (`position, 1`).
pub(crate) const TERRAIN_MANAGER_UPDATE: u32 = 0x006f_ca90;

// Strings: the debug text and the scene graph titles.
/// `"Debug Text -> %s"`
pub(crate) const MSG_DEBUG_TEXT: u32 = 0x0103_9c64;
/// `"Display"`
pub(crate) const TEXT_SECTION_DISPLAY: u32 = 0x0103_9c78;
/// `"sDebugText"`
pub(crate) const TEXT_KEY_DEBUG_TEXT: u32 = 0x0103_9c80;
/// `"Custom.ini"`
pub(crate) const TEXT_CUSTOM_INI: u32 = 0x0103_9c8c;
/// `"Fallout Screen Splatter Scene Graph"`
pub(crate) const TITLE_SPLATTER_SCENE: u32 = 0x0103_9c98;
/// `"Fallout Menu Player 3D Scene Graph"`
pub(crate) const TITLE_MENU_PLAYER_3D: u32 = 0x0103_9cbc;
/// `"Fallout Menu Object 3D Scene Graph"`
pub(crate) const TITLE_MENU_OBJECT_3D: u32 = 0x0103_9ce0;
/// `"Fallout Interface 3D Object Scene Graph"`
pub(crate) const TITLE_INTERFACE_3D_OBJECT: u32 = 0x0103_9d04;
/// `"Fallout Interface Scene Graph"`
pub(crate) const TITLE_INTERFACE: u32 = 0x0103_9d2c;
/// `"Fallout World Scene Graph"`
pub(crate) const TITLE_WORLD: u32 = 0x0103_9d4c;
/// `"Player Inventory Menu"`
pub(crate) const TITLE_PLAYER_INVENTORY_MENU: u32 = 0x0103_9d68;
/// `"Player 1st Person"`
pub(crate) const TITLE_PLAYER_FIRST_PERSON: u32 = 0x0103_9d80;
/// `"Player 3rd Person"`
pub(crate) const TITLE_PLAYER_THIRD_PERSON: u32 = 0x0103_9d94;
/// `"Fallout Rendered Menu 3D Scene Graph"`
pub(crate) const TITLE_RENDERED_MENU_3D: u32 = 0x0103_9da8;
/// `"Must supply a valid reference to show viewer string."`
pub(crate) const MSG_VIEWER_NEEDS_REFERENCE: u32 = 0x0103_9dd0;
/// `"Node with no children supplied as reference..."`
pub(crate) const MSG_VIEWER_NODE_WITHOUT_CHILDREN: u32 = 0x0103_9e08;
/// `"Invalid parameter.  Please specify land, objects, trees, water, or all."`
pub(crate) const MSG_LOD_INVALID_PARAMETER: u32 = 0x0103_9e38;
/// `"Toggling objects."`
pub(crate) const MSG_TOGGLING_OBJECTS: u32 = 0x0103_9e80;
/// `"buildings"`
pub(crate) const TEXT_BUILDINGS: u32 = 0x0103_9e94;
/// `"objects"`
pub(crate) const TEXT_OBJECTS: u32 = 0x0103_9ea0;
/// `"Toggling trees."`
pub(crate) const MSG_TOGGLING_TREES: u32 = 0x0103_9ea8;
/// `"trees"`
pub(crate) const TEXT_TREES: u32 = 0x0103_9eb8;
/// `"Toggling land."`
pub(crate) const MSG_TOGGLING_LAND: u32 = 0x0103_9ec0;
/// `"land"`
pub(crate) const TEXT_LAND: u32 = 0x0103_9ed0;
/// `"Toggling water."`
pub(crate) const MSG_TOGGLING_WATER: u32 = 0x0103_9ed8;
/// `"Toggling all LOD."`
pub(crate) const MSG_TOGGLING_ALL_LOD: u32 = 0x0103_9ee8;
/// `"all"`
pub(crate) const TEXT_ALL: u32 = 0x0103_9efc;
/// `"water"`
pub(crate) const TEXT_WATER: u32 = 0x0102_36fc;

// Translated from 005ba760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleDebugTextFunction` (Xbox PDB): `Interface::ToggleDebugTextVisible`
/// and, when the commands echo, "Debug Text -> On" or "Off"
/// (`Interface::IsDebugTextVisible`). The decompiler shows an `ECX` argument
/// that neither callee reads.
pub fn script_toggle_debug_text_function(e: &mut Engine) -> bool {
    e.call(INTERFACE_TOGGLE_DEBUG_TEXT_VISIBLE, &args![]);
    if echo_enabled(e) {
        let visible = e.call(INTERFACE_IS_DEBUG_TEXT_VISIBLE, &args![]).u8();
        console_print(e, &args![MSG_DEBUG_TEXT, on_off(visible != 0)]);
    }
    true
}

// Translated from 005ba7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a string (up to 0x200 bytes, empty by default) and an `int`. With
/// no string it asks the debug text table (`005ba8d0`) to list its names
/// through the console print (`00802880`). With a string it selects that name
/// in the table (`008029c0`) and writes it to `Custom.ini` under `[Display]`
/// `sDebugText` (`WritePrivateProfileStringA` on `<game directory>Custom.ini`).
/// (The stack protector cookie check is left out.)
pub fn fn_005ba7c0(e: &mut Engine, a: ScriptArgs) -> bool {
    e.with_stack(0x200, |e, name| {
        e.with_stack(4, |e, value| {
            if !a.parse(e, &[name.addr(), value.addr()]) {
                return false;
            }
            let table = fn_005ba8d0(e);
            if e.mem.u8(name.addr()) == 0 {
                e.call(DEBUG_TEXT_TABLE_LIST, &args![table, CONSOLE_PRINT]);
            } else {
                let value = e.mem.u32(value.addr());
                e.call(DEBUG_TEXT_TABLE_SELECT, &args![table, name, value]);
                e.with_stack(0x104, |e, path| {
                    let directory = e.call(FN_004DC110, &args![]).u32();
                    e.call(STRCPY_S, &args![path, 0x104u32, directory]);
                    e.call(STRCAT_S, &args![path, 0x104u32, TEXT_CUSTOM_INI]);
                    e.call(
                        WRITE_PRIVATE_PROFILE_STRING,
                        &args![TEXT_SECTION_DISPLAY, TEXT_KEY_DEBUG_TEXT, name, path],
                    );
                });
            }
            true
        })
    })
}

// Translated from 005ba8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The debug text table ([`DEBUG_TEXT_TABLE`]), created by `00802a90` when it
/// does not exist yet.
pub fn fn_005ba8d0(e: &mut Engine) -> u32 {
    if e.global::<u32>(DEBUG_TEXT_TABLE) == 0 {
        e.call(FN_00802A90, &args![]);
    }
    e.global::<u32>(DEBUG_TEXT_TABLE)
}

/// A scene graph window object: `new` of 0x30 bytes and, when the block came
/// back, the constructor `004d61b0` with the values of the singleton in
/// [`GLOBAL_0011DEA0C`] (`*(this + 0x0c)` and `*(this + 8)`), the title and
/// root object that `inputs` produces (called after the allocation, as the
/// game computes them there) and the fixed placement 0x80000000, 0x80000000,
/// 800 by 600. Zero when the allocation failed.
fn new_scene_graph_window(e: &mut Engine, inputs: impl FnOnce(&mut Engine) -> (u32, u32)) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x30u32]).u32();
    if block == 0 {
        return 0;
    }
    let (title, root) = inputs(e);
    let singleton = e.global::<u32>(GLOBAL_0011DEA0C);
    let field_8 = e.call(FN_0044DDC0, &args![singleton]).u32();
    let singleton = e.global::<u32>(GLOBAL_0011DEA0C);
    let field_c = e.call(FN_0084E3A0, &args![singleton]).u32();
    e.call(
        SCENE_GRAPH_WINDOW_CONSTRUCT,
        &args![
            block,
            field_c,
            field_8,
            root,
            title,
            0x8000_0000u32,
            0x8000_0000u32,
            800u32,
            600u32
        ],
    )
    .u32()
}

/// `window.AddObject(object, title)`.
fn scene_graph_add(e: &mut Engine, window: u32, object: u32, title: u32) {
    e.call(SCENE_GRAPH_WINDOW_ADD_OBJECT, &args![window, object, title]);
}

/// The objects the world scene graph windows add after the first, in the order
/// of the game: the interface and the 3D object interface scenes, then the
/// object of `FN_00450B80(1)` (when set) under `menu_title`.
fn add_interface_scenes(e: &mut Engine, window: u32, menu_title: u32) {
    let object = fn_005bac60(e);
    scene_graph_add(e, window, object, TITLE_INTERFACE);
    let object = fn_005bac70(e);
    scene_graph_add(e, window, object, TITLE_INTERFACE_3D_OBJECT);
    if e.call(FN_00450B80, &args![1u32]).u32() != 0 {
        let object = e.call(FN_00450B80, &args![1u32]).u32();
        scene_graph_add(e, window, object, menu_title);
    }
}

/// Adds the splatter scene to a scene graph window when it exists.
fn add_splatter_scene(e: &mut Engine, window: u32) {
    if fn_005bac20(e) {
        let object = fn_005bac50(e);
        scene_graph_add(e, window, object, TITLE_SPLATTER_SCENE);
    }
}

// Translated from 005ba8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Opens a scene graph window (the debugging "show scene graph" command) after
/// storing the window callback ([`fn_005bac80`]). Without a reference (or with
/// one whose object slot `0x1d0` returns zero) the window is titled "Fallout
/// World Scene Graph" with the world scene (`0045c670`) and gets the interface
/// scenes, the menu object scene (`FN_00450B80(1)`), the menu player scene
/// (`FN_00450B80(3)`) and the splatter scene. For the player it is titled
/// "Player 1st Person" or "Player 3rd Person" after the player's third person
/// flag, holds the object of the other person (`00950bb0`) and the inventory
/// menu object; for any other reference it is titled with the reference's
/// name (`0055d520`) and holds the reference's object only. (Exception
/// unwinding frames are not translated.)
pub fn fn_005ba8f0(e: &mut Engine, a: ScriptArgs) -> bool {
    fn_005bac80(e, SCENE_GRAPH_CALLBACK);
    let object = if a.this_obj.is_null() {
        0
    } else {
        e.vcall(a.this_obj.addr(), REFR_GET_TARGET_SLOT, &args![])
            .u32()
    };
    if object == 0 {
        let window = new_scene_graph_window(e, |e| {
            let world = e.call(FN_0045C670, &args![]).u32();
            (TITLE_WORLD, world)
        });
        add_interface_scenes(e, window, TITLE_MENU_OBJECT_3D);
        if e.call(FN_00450B80, &args![3u32]).u32() != 0 {
            let object = e.call(FN_00450B80, &args![3u32]).u32();
            scene_graph_add(e, window, object, TITLE_MENU_PLAYER_3D);
        }
        add_splatter_scene(e, window);
        return true;
    }
    let name = e.call(REFR_GET_NAME_TEXT, &args![a.this_obj]).u32();
    let player = e.global::<u32>(PLAYER);
    if a.this_obj.addr() == player {
        let window = new_scene_graph_window(e, |e| {
            let player = e.global::<u32>(PLAYER);
            let third = e.call(PLAYER_IS_THIRD_PERSON, &args![player]).u8();
            let title = if third != 0 {
                TITLE_PLAYER_THIRD_PERSON
            } else {
                TITLE_PLAYER_FIRST_PERSON
            };
            let root = e
                .vcall(a.this_obj.addr(), REFR_GET_TARGET_SLOT, &args![])
                .u32();
            (title, root)
        });
        let player = e.global::<u32>(PLAYER);
        let third = e.call(PLAYER_IS_THIRD_PERSON, &args![player]).u8();
        let title = if third != 0 {
            TITLE_PLAYER_FIRST_PERSON
        } else {
            TITLE_PLAYER_THIRD_PERSON
        };
        let player = e.global::<u32>(PLAYER);
        let third = e.call(PLAYER_IS_THIRD_PERSON, &args![player]).u8();
        let player = e.global::<u32>(PLAYER);
        let other = e
            .call(PLAYER_GET_PERSON_OBJECT, &args![player, u32::from(third)])
            .u32();
        scene_graph_add(e, window, other, title);
        let player = e.global::<u32>(PLAYER);
        if e.call(PLAYER_GET_INVENTORY_MENU_OBJECT, &args![player])
            .u32()
            != 0
        {
            let player = e.global::<u32>(PLAYER);
            let menu = e
                .call(PLAYER_GET_INVENTORY_MENU_OBJECT, &args![player])
                .u32();
            scene_graph_add(e, window, menu, TITLE_PLAYER_INVENTORY_MENU);
        }
    } else {
        new_scene_graph_window(e, |e| {
            let root = e
                .vcall(a.this_obj.addr(), REFR_GET_TARGET_SLOT, &args![])
                .u32();
            (name, root)
        });
    }
    true
}

// Translated from 005bac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the splatter scene (the `NiPointer` at `011c786c`) exists and its
/// object answers `00453470` with a non-zero value.
pub fn fn_005bac20(e: &mut Engine) -> bool {
    if ni_pointer_get(e, SLOT_SPLATTER_SCENE) == 0 {
        return false;
    }
    let scene = ni_pointer_get(e, SLOT_SPLATTER_SCENE);
    e.call(FN_00453470, &args![scene]).u32() != 0
}

// Translated from 005bac50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The splatter scene: the `NiPointer` at `011c786c` read.
pub fn fn_005bac50(e: &mut Engine) -> u32 {
    ni_pointer_get(e, SLOT_SPLATTER_SCENE)
}

// Translated from 005bac60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The interface scene: the `NiPointer` at `011dee84` read.
pub fn fn_005bac60(e: &mut Engine) -> u32 {
    ni_pointer_get(e, SLOT_INTERFACE_SCENE)
}

// Translated from 005bac70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 3D object interface scene: the `NiPointer` at `011deca4` read.
pub fn fn_005bac70(e: &mut Engine) -> u32 {
    ni_pointer_get(e, SLOT_INTERFACE_3D_SCENE)
}

// Translated from 005bac80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the global `011c6f18` (the scene graph window callback).
pub fn fn_005bac80(e: &mut Engine, value: u32) {
    e.set_global(SCENE_GRAPH_CALLBACK_GLOBAL, value);
}

// Translated from 005bac90 (decompiled, FalloutNV.exe 1.4.0.525)
/// As [`fn_005ba8f0`] for the world: stores the callback, opens the world
/// scene graph window, fills it from the reference's object (slot `0x1d0`,
/// `SCENE_GRAPH_WINDOW_FILL`) when a reference is given, then adds the
/// interface scenes, the rendered menu scene (`FN_00450B80(1)`) and the
/// splatter scene.
pub fn fn_005bac90(e: &mut Engine, a: ScriptArgs) -> bool {
    fn_005bac80(e, SCENE_GRAPH_CALLBACK);
    let window = new_scene_graph_window(e, |e| {
        let world = e.call(FN_0045C670, &args![]).u32();
        (TITLE_WORLD, world)
    });
    if !a.this_obj.is_null() {
        let object = e
            .vcall(a.this_obj.addr(), REFR_GET_TARGET_SLOT, &args![])
            .u32();
        e.call(SCENE_GRAPH_WINDOW_FILL, &args![window, object]);
    }
    add_interface_scenes(e, window, TITLE_RENDERED_MENU_3D);
    add_splatter_scene(e, window);
    true
}

/// The strings a collector holds, printed one by one through the console
/// print (the string is the format), then the collector cleared.
fn print_collected_strings(e: &mut Engine, collector: u32) {
    let mut index = 0u32;
    while index < e.call(STRING_COLLECTOR_COUNT, &args![collector]).u32() {
        let slot = e
            .call(STRING_COLLECTOR_STRING_ADDRESS, &args![collector, index])
            .u32();
        let text = e.mem.u32(slot);
        console_print(e, &args![text]);
        index += 1;
    }
    e.call(STRING_COLLECTOR_CLEAR, &args![collector]);
}

// Translated from 005bade0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowViewerStringsFunction` (Xbox PDB): prints the strings of the
/// reference's object. Without a reference (or an object) it prints "Must
/// supply a valid reference to show viewer string." A node (slot `0xc`
/// non-zero) is replaced by its first child (`0043b480`, `0043b4a0`); none
/// prints "Node with no children supplied as reference...". Otherwise the
/// object collects its strings (slot `0x60`) into a collector (`004d9100`,
/// 0x10 bytes) that are printed; when the object has an attached object
/// (`006838b0`) that converts through `00653270` with the type descriptor
/// `012043f8`, that object's strings (`006fa820`, slot `0x60`) are printed
/// too. The collector is deleted (virtual slot 0, argument 1); the "no
/// children" path leaves it allocated, as the game does.
pub fn script_show_viewer_strings_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let has_object = !a.this_obj.is_null()
        && e.vcall(a.this_obj.addr(), REFR_GET_TARGET_SLOT, &args![])
            .u32()
            != 0;
    if !has_object {
        console_print(e, &args![MSG_VIEWER_NEEDS_REFERENCE]);
        return true;
    }
    let mut object = e
        .vcall(a.this_obj.addr(), REFR_GET_TARGET_SLOT, &args![])
        .u32();
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let collector = if block != 0 {
        e.call(STRING_COLLECTOR_CONSTRUCT, &args![block, 0u32, 1u32])
            .u32()
    } else {
        0
    };
    if object != 0 && e.vcall(object, OBJECT_IS_NODE_SLOT, &args![]).u32() != 0 {
        let node = object;
        object = 0;
        let mut index = 0u32;
        while index < e.call(NODE_CHILD_COUNT, &args![node]).u32() && object == 0 {
            object = e.call(NODE_CHILD_AT, &args![node, index]).u32();
            index += 1;
        }
    }
    if object == 0 {
        console_print(e, &args![MSG_VIEWER_NODE_WITHOUT_CHILDREN]);
        return true;
    }
    e.vcall(object, OBJECT_COLLECT_STRINGS_SLOT, &args![collector]);
    print_collected_strings(e, collector);
    if e.call(NODE_GET_ATTACHED_OBJECT, &args![object]).u32() != 0 {
        let attached = e.call(NODE_GET_ATTACHED_OBJECT, &args![object]).u32();
        let converted = e.call(FN_00653270, &args![TYPE_012043F8, attached]).u32();
        if converted != 0 {
            let target = e.call(FN_006FA820, &args![converted]).u32();
            e.vcall(target, OBJECT_COLLECT_STRINGS_SLOT, &args![collector]);
            print_collected_strings(e, collector);
        }
    }
    if collector != 0 {
        e.vcall(collector, 0, &args![1u32]);
    }
    true
}

/// One LOD toggle of `Script::ToggleLODLandFunction`: when the LOD object
/// `getter` returns exists and `selected`, sets its flag to the opposite of
/// what `00456610` reads.
fn toggle_lod_object(e: &mut Engine, terrain: u32, getter: u32, selected: bool) {
    if e.call(getter, &args![terrain]).u32() != 0 && selected {
        let object = e.call(getter, &args![terrain]).u32();
        let flag = e.call(LOD_OBJECT_GET_FLAG, &args![object]).u8();
        let object = e.call(getter, &args![terrain]).u32();
        e.call(LOD_OBJECT_SET_FLAG, &args![object, u32::from(flag == 0)]);
    }
}

/// Whether the string at `text` equals `word` ignoring case (`00408b20`).
fn text_is(e: &mut Engine, text: u32, word: u32) -> bool {
    e.call(COMPARE_STRINGS_NO_CASE, &args![text, word]).u32() == 0
}

// Translated from 005bb030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleLODLandFunction` (Xbox PDB): parses an optional string
/// (0x200 bytes; skipped when the first word is zero). With the current world
/// space's terrain manager (`TES::GetWorldSpace`, `GetTerrainManager`; none
/// ends the command) it selects the LOD kinds: an empty string or "all" (all
/// four, "Toggling all LOD."), "water", "land", "trees", "objects" or
/// "buildings" (each prints "Toggling <kind>."; "objects" and "buildings"
/// both select the objects). Nothing selected prints the invalid parameter
/// message. Each selected, existing LOD object (trees `004ea9e0`, land
/// `004ea9a0`, objects `004ea9c0`, water `0043b230`) gets its flag set to the
/// opposite of the one `00456610` reads, through `00450f90`.
pub fn script_toggle_lod_land_function(e: &mut Engine, a: ScriptArgs) -> bool {
    e.with_stack(0x200, |e, text| {
        let text = text.addr();
        if a.param_info != 0 && !a.parse(e, &[text]) {
            return false;
        }
        let tes = e.global::<u32>(GLOBAL_0011DEA10);
        let world_space = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
        let terrain = e
            .call(WORLD_SPACE_GET_TERRAIN_MANAGER, &args![world_space])
            .u32();
        if terrain == 0 {
            return true;
        }
        let (mut objects, mut trees, mut land, mut water) = (false, false, false, false);
        if e.mem.u8(text) == 0 || text_is(e, text, TEXT_ALL) {
            console_print(e, &args![MSG_TOGGLING_ALL_LOD]);
            objects = true;
            trees = true;
            land = true;
            water = true;
        }
        if text_is(e, text, TEXT_WATER) {
            console_print(e, &args![MSG_TOGGLING_WATER]);
            water = true;
        }
        if text_is(e, text, TEXT_LAND) {
            console_print(e, &args![MSG_TOGGLING_LAND]);
            land = true;
        }
        if text_is(e, text, TEXT_TREES) {
            console_print(e, &args![MSG_TOGGLING_TREES]);
            trees = true;
        }
        if text_is(e, text, TEXT_OBJECTS) || text_is(e, text, TEXT_BUILDINGS) {
            console_print(e, &args![MSG_TOGGLING_OBJECTS]);
            objects = true;
        }
        if !(water || land || trees || objects) {
            console_print(e, &args![MSG_LOD_INVALID_PARAMETER]);
            return true;
        }
        toggle_lod_object(e, terrain, LOD_TREES_OBJECT, trees);
        toggle_lod_object(e, terrain, LOD_LAND_OBJECT, land);
        toggle_lod_object(e, terrain, LOD_OBJECTS_OBJECT, objects);
        toggle_lod_object(e, terrain, LOD_WATER_OBJECT, water);
        true
    })
}

// ---- Fourth batch, second part (`005bb390` onward) ---------------------------

// The interface objects the commands hand values to.
/// `thiscall`-free getter, `cdecl`, no arguments: `Interface + 0x1dc` when the
/// interface singleton (`004b7210`) exists and `009373f0` agrees, otherwise
/// zero.
pub(crate) const INTERFACE_GET_1DC_OBJECT: u32 = 0x0070_5910;
/// As [`INTERFACE_GET_1DC_OBJECT`] for `Interface + 0x178`.
pub(crate) const INTERFACE_GET_178_OBJECT: u32 = 0x0070_5950;
/// `Interface::GetPipboy` (Xbox PDB), `cdecl`, no arguments.
pub(crate) const INTERFACE_GET_PIPBOY: u32 = 0x0070_5990;
/// `thiscall` (`RET 4`: `source`): `*this = *source`.
pub(crate) const FN_0044AC20: u32 = 0x0044_ac20;
/// `thiscall` on [`INTERFACE_GET_1DC_OBJECT`]'s object (`RET 0xc`: `reference,
/// ticks, 0`).
pub(crate) const FN_00800AC0: u32 = 0x0080_0ac0;
/// `thiscall` on that object (`RET 4`: `reference`).
pub(crate) const FN_00800D50: u32 = 0x0080_0d50;
/// `thiscall` on that object (`RET 4`: `reference`).
pub(crate) const FN_00800E50: u32 = 0x0080_0e50;
/// `thiscall` on that object, no arguments.
pub(crate) const FN_00800ED0: u32 = 0x0080_0ed0;
/// `cdecl` (`float, float, float`).
pub(crate) const FN_007E4FE0: u32 = 0x007e_4fe0;
/// `thiscall` on [`INTERFACE_GET_178_OBJECT`]'s object (`RET 0xc`: `float,
/// float, float`): stores the three values and the time of the clock at
/// `011f6394` in the object.
pub(crate) const FN_007F7880: u32 = 0x007f_7880;
/// `FOPipboyManager::MoveKnob` (Xbox PDB), `thiscall` (`RET 0x10`: `knob,
/// amount, step, 1`).
pub(crate) const FN_007F8610: u32 = 0x007f_8610;
/// `thiscall` on the global object `011e090c`: `this ? this + 4 : address of a
/// static zero float`, the address of the `float` the object holds.
pub(crate) const FLOAT_VALUE_ADDRESS: u32 = 0x0040_3e20;
/// The global object whose float `005bb870` sets.
pub(crate) const GLOBAL_011E090C: u32 = 0x011e_090c;
/// `thiscall` on the object `FN_00450B80(0)` returns (`RET 0x18`: `flag, vector
/// x, y, z, float, float`); a three-byte stub in this build.
pub(crate) const FN_00B5A8C0: u32 = 0x00b5_a8c0;
/// Byte flipped (or set) by `005bb8c0`.
pub(crate) const FLAG_011CAE47: u32 = 0x011c_ae47;
/// `thiscall` on a matrix (`out, other`; `RET 8`): `NiMatrix3::operator*`
/// (engine map name): writes `this * other` to `out` and returns `out`.
pub(crate) const NI_MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// `thiscall`: `this + 0x68`, the address of a matrix.
pub(crate) const FN_00461130: u32 = 0x0046_1130;
/// `thiscall`: `*(this + 0x1e0)`.
pub(crate) const FN_00454B30: u32 = 0x0045_4b30;
/// `thiscall` on the object `FN_00454B30` returns (`RET 4`: `object`): adds the
/// object to a list kept at `this + 8`.
pub(crate) const FN_00C5AE70: u32 = 0x00c5_ae70;
/// Constructor of a 0xfc-byte object, `thiscall` (`RET 0xc`: `position, 2D
/// point, matrix`).
pub(crate) const FN_004A5FF0: u32 = 0x004a_5ff0;
/// `thiscall` on `this + 8` of the object `FN_00454B30` returns: removes all
/// entries (`NiTPointerListBase::RemoveAll`, Xbox PDB).
pub(crate) const FN_004ED900: u32 = 0x004e_d900;
/// `float` `15.0`.
pub(crate) const F32_0101E580: u32 = 0x0101_e580;
/// `float` `500.0`.
pub(crate) const F32_01013D84: u32 = 0x0101_3d84;
/// `float` `0.1`.
pub(crate) const F32_0101E2BC: u32 = 0x0101_e2bc;
/// `float` `0.05`.
pub(crate) const F32_010181BC: u32 = 0x0101_81bc;
/// `float` `0.0005`.
pub(crate) const F32_010780A4: u32 = 0x0107_80a4;
/// `float` `0.5`.
pub(crate) const F32_01016248: u32 = 0x0101_6248;
/// `float` `50.0`.
pub(crate) const F32_0101B268: u32 = 0x0101_b268;
/// `float` `pi / 2`.
pub(crate) const FLOAT_HALF_PI: u32 = 0x0103_9f00;
/// `ImageSpaceModifierInstance::Stop` (Xbox PDB), `cdecl` (`instance`).
pub(crate) const IMAGE_SPACE_MODIFIER_INSTANCE_STOP: u32 = 0x0052_93f0;
/// `ImageSpaceModifierInstanceRB::Trigger` (Xbox PDB), `cdecl` (`strength,
/// ramp up, start, ramp down, down start, 2D point, 0.0`): the new instance.
pub(crate) const RADIAL_BLUR_TRIGGER: u32 = 0x0052_a440;
/// Pointer: the radial blur instance `SetGlobalRadialBlur` made.
pub(crate) const RADIAL_BLUR_INSTANCE: u32 = 0x011c_ae48;
/// Byte flipped by `005bbc30`.
pub(crate) const FLAG_011AD884: u32 = 0x011a_d884;
/// Byte: AI detection (flipped by `005bbda0`).
pub(crate) const AI_DETECTION_FLAG: u32 = 0x011f_1221;
/// Byte: all AI processing (flipped by `ToggleAI` without an actor).
pub(crate) const ALL_AI_FLAG: u32 = 0x011f_1220;
/// Byte: AI processing for actors in high (flipped by `ToggleHighProcess`).
pub(crate) const HIGH_PROCESS_FLAG: u32 = 0x011f_1228;
/// Byte: all combat AI processing (flipped by `005bc130`).
pub(crate) const COMBAT_AI_FLAG: u32 = 0x011a_4880;
/// Byte set by `005bc1a0` from its argument.
pub(crate) const FLAG_011DF678: u32 = 0x011d_f678;
/// Byte set by `005bc200` from its argument.
pub(crate) const FLAG_012682F8: u32 = 0x0126_82f8;
/// Offset in an `Actor` (PC) of the byte the AI commands flip and print.
pub(crate) const ACTOR_AI_FLAG: u32 = 0xbc;
/// `Actor::EndMovement` (Xbox PDB), `thiscall`.
pub(crate) const ACTOR_END_MOVEMENT: u32 = 0x0087_faa0;
/// `ProcessLists::TurnOffMovementinHigh` (Xbox PDB), `thiscall` on
/// [`PROCESS_LISTS`].
pub(crate) const PROCESS_LISTS_TURN_OFF_MOVEMENT_IN_HIGH: u32 = 0x0097_1ba0;
/// `thiscall` on the player (`RET 4`: `flag`): `AL`.
pub(crate) const FN_00961E30: u32 = 0x0096_1e30;
/// `cdecl` (`flag`).
pub(crate) const FN_00BA8B60: u32 = 0x00ba_8b60;
/// `ExtraDataList::GetHeadTrackTargetExtra` (Xbox PDB), `thiscall`: the
/// reference the actor looks at, zero when none.
pub(crate) const HEAD_TRACK_TARGET_EXTRA: u32 = 0x0041_9ea0;

// Strings.
/// `"SetGlobalRadialBlur strength rampup start rampdown downstart"`
pub(crate) const MSG_SET_GLOBAL_RADIAL_BLUR_USAGE: u32 = 0x0103_9f04;
/// `"AI Detection is  %s"`
pub(crate) const MSG_AI_DETECTION_IS: u32 = 0x0103_9f44;
/// `"%s processing is  %s"`
pub(crate) const MSG_PROCESSING_IS: u32 = 0x0103_9f58;
/// `"All AI Processing is  %s"`
pub(crate) const MSG_ALL_AI_PROCESSING_IS: u32 = 0x0103_9f70;
/// `"All Combat AI processing is %s"`
pub(crate) const MSG_ALL_COMBAT_AI_PROCESSING_IS: u32 = 0x0103_9f8c;
/// `"%s set to not look at anyone"`
pub(crate) const MSG_NOT_LOOKING_AT_ANYONE: u32 = 0x0103_9fac;
/// `"%s set to look at %s"`
pub(crate) const MSG_LOOKING_AT: u32 = 0x0103_9fcc;
/// `"AI Processing for actors in high is  %s"`
pub(crate) const MSG_HIGH_AI_PROCESSING_IS: u32 = 0x0103_9fe4;

/// `__RTDynamicCast(object, 0, TESObjectREFR, Actor, 0)`: the object as an
/// `Actor`, zero when it is none (also for a null object).
fn cast_to_actor(e: &mut Engine, object: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![object, 0u32, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0u32],
    )
    .u32()
}

/// The three words at `address` (a vector).
fn read_vector(e: &Engine, address: u32) -> [u32; 3] {
    [
        e.mem.u32(address),
        e.mem.u32(address + 4),
        e.mem.u32(address + 8),
    ]
}

// Translated from 005bb390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetLODObjectDistanceFunction` (Xbox PDB): parses two `float`s
/// (zero by default; not parsed when the first word is zero). With the current
/// world space's terrain manager (none ends the command) it sets the two
/// settings `011d8724` and `011d877c` to them (`004ed780`) and updates the
/// terrain manager (`BGSTerrainManager::Update`) with the player's position
/// (virtual slot `0x1f4`). The game also builds a local vector from the start
/// vector `011f426c` and from the cell position (`005bb4d0` set) or the player
/// position and never uses it; only its calls are kept.
pub fn script_set_lod_object_distance_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let (near, far) = if a.param_info == 0 {
        (0, 0)
    } else {
        let Some([near, far]) = a.parse_into(e, [0, 0]) else {
            return false;
        };
        (near, far)
    };
    let tes = e.global::<u32>(GLOBAL_0011DEA10);
    let world_space = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
    let terrain = e
        .call(WORLD_SPACE_GET_TERRAIN_MANAGER, &args![world_space])
        .u32();
    if terrain == 0 {
        return true;
    }
    e.call(SETTING_SET_FLOAT, &args![SETTING_LOD_DISTANCE_NEAR, near]);
    e.call(SETTING_SET_FLOAT, &args![SETTING_LOD_DISTANCE_FAR, far]);
    let singleton = e.global::<u32>(GLOBAL_0011DEA0C);
    if fn_005bb4d0(e, Ptr::new(singleton)) != 0 {
        let world = e.call(FN_0045C670, &args![]).u32();
        let cell = e.call(FN_00558310, &args![world]).u32();
        e.call(FN_0043C490, &args![cell]);
    } else {
        let player = e.global::<u32>(PLAYER);
        e.vcall(player, PLAYER_POSITION_SLOT, &args![]);
    }
    let player = e.global::<u32>(PLAYER);
    let position = e.vcall(player, PLAYER_POSITION_SLOT, &args![]).u32();
    e.call(TERRAIN_MANAGER_UPDATE, &args![terrain, position, 1u32]);
    true
}

// Translated from 005bb4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 6`.
pub fn fn_005bb4d0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 6)
}

// Translated from 005bb4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses three `float`s (0.0, 0.0 and 1.0 by default, and a fourth word the
/// game never reads) and passes the first three to `007e4fe0`.
pub fn fn_005bb4f0(e: &mut Engine, a: ScriptArgs) -> bool {
    let defaults = [0.0f32.to_bits(), 0.0f32.to_bits(), 1.0f32.to_bits(), 0];
    let Some([first, second, third, _]) = a.parse_into(e, defaults) else {
        return false;
    };
    e.call(FN_007E4FE0, &args![first, second, third]);
    true
}

/// Stores `value` in the first word of the interface object
/// [`INTERFACE_GET_1DC_OBJECT`] returns, the way the game does: a local
/// holding the value passed by address to `0044ac20`.
fn set_interface_object_word(e: &mut Engine, value: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), value);
        let target = e.call(INTERFACE_GET_1DC_OBJECT, &args![]).u32();
        e.call(FN_0044AC20, &args![target, local]);
    });
}

// Translated from 005bb570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a `float` (default: the global `0101e580`, 15.0) and three words the
/// game never reads. For a reference with an object (slot `0x1d0`) it hands
/// the reference and the value truncated to an `int` (`_ftol2`) to
/// `00800ac0` on the interface object [`INTERFACE_GET_1DC_OBJECT`], with 0
/// as third argument, and then stores 0 in the first word of that object.
pub fn fn_005bb570(e: &mut Engine, a: ScriptArgs) -> bool {
    let default = e.global::<u32>(F32_0101E580);
    let Some([value, _, _, _]) = a.parse_into(e, [default, 0, 0, 0]) else {
        return false;
    };
    if !a.this_obj.is_null()
        && e.vcall(a.this_obj.addr(), REFR_GET_TARGET_SLOT, &args![])
            .u32()
            != 0
    {
        let ticks = e.call(FTOL, &args![f64::from(f32::from_bits(value))]).u32();
        let target = e.call(INTERFACE_GET_1DC_OBJECT, &args![]).u32();
        e.call(FN_00800AC0, &args![target, a.this_obj, ticks, 0u32]);
        set_interface_object_word(e, 0);
    }
    true
}

// Translated from 005bb610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses the same arguments as [`fn_005bb570`] and ignores them. For a
/// reference with an object it hands the reference to `00800d50` and `00800e50`
/// on the interface object [`INTERFACE_GET_1DC_OBJECT`] and stores 1 in the
/// first word of that object.
pub fn fn_005bb610(e: &mut Engine, a: ScriptArgs) -> bool {
    let default = e.global::<u32>(F32_0101E580);
    if a.parse_into(e, [default, 0, 0, 0]).is_none() {
        return false;
    }
    if !a.this_obj.is_null()
        && e.vcall(a.this_obj.addr(), REFR_GET_TARGET_SLOT, &args![])
            .u32()
            != 0
    {
        let target = e.call(INTERFACE_GET_1DC_OBJECT, &args![]).u32();
        e.call(FN_00800D50, &args![target, a.this_obj]);
        let target = e.call(INTERFACE_GET_1DC_OBJECT, &args![]).u32();
        e.call(FN_00800E50, &args![target, a.this_obj]);
        set_interface_object_word(e, 1);
    }
    true
}

// Translated from 005bb6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses the same arguments as [`fn_005bb570`] and ignores them. For a
/// reference with an object it calls `00800ac0(0, 0, 0)` and `00800ed0` on the
/// interface object [`INTERFACE_GET_1DC_OBJECT`] and stores 0 in the first word
/// of that object.
pub fn fn_005bb6c0(e: &mut Engine, a: ScriptArgs) -> bool {
    let default = e.global::<u32>(F32_0101E580);
    if a.parse_into(e, [default, 0, 0, 0]).is_none() {
        return false;
    }
    if !a.this_obj.is_null()
        && e.vcall(a.this_obj.addr(), REFR_GET_TARGET_SLOT, &args![])
            .u32()
            != 0
    {
        let target = e.call(INTERFACE_GET_1DC_OBJECT, &args![]).u32();
        e.call(FN_00800AC0, &args![target, 0u32, 0u32, 0u32]);
        let target = e.call(INTERFACE_GET_1DC_OBJECT, &args![]).u32();
        e.call(FN_00800ED0, &args![target]);
        set_interface_object_word(e, 0);
    }
    true
}

// Translated from 005bb770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses three `float`s (defaults: the globals `01013d84`, `0101e2bc` and
/// `010181bc`, 500.0, 0.1 and 0.05) and a fourth word the game never reads, and
/// hands them to `007f7880` on the object [`INTERFACE_GET_178_OBJECT`] returns.
pub fn fn_005bb770(e: &mut Engine, a: ScriptArgs) -> bool {
    let defaults = [
        e.global::<u32>(F32_01013D84),
        e.global::<u32>(F32_0101E2BC),
        e.global::<u32>(F32_010181BC),
        0,
    ];
    let Some([first, second, third, _]) = a.parse_into(e, defaults) else {
        return false;
    };
    let target = e.call(INTERFACE_GET_178_OBJECT, &args![]).u32();
    e.call(FN_007F7880, &args![target, first, second, third]);
    true
}

// Translated from 005bb800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::MovePipboyKnobFunction` (Xbox PDB): parses an `int` and a `float`
/// and calls `FOPipboyManager::MoveKnob(knob, amount, step, 1)` on
/// `Interface::GetPipboy`, with `step` the global `float` `010780a4`.
pub fn script_move_pipboy_knob_function(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([knob, amount]) = a.parse_into(e, [0, 0]) else {
        return false;
    };
    let step = e.global::<u32>(F32_010780A4);
    let pipboy = e.call(INTERFACE_GET_PIPBOY, &args![]).u32();
    e.call(FN_007F8610, &args![pipboy, knob, amount, step, 1u32]);
    true
}

// Translated from 005bb870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses a `float` and stores it in the `float` of the global object
/// `011e090c` (`00403e20` gives its address).
pub fn fn_005bb870(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = a.parse_into(e, [0]) else {
        return false;
    };
    let address = e.call(FLOAT_VALUE_ADDRESS, &args![GLOBAL_011E090C]).u32();
    e.mem.set_f32(address, f32::from_bits(value));
    true
}

// Translated from 005bb8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses two `float`s (both -1.0, the global `01012054`, by default). When
/// either is above zero (or not a number) the byte `011cae47` is set to 1,
/// otherwise it is flipped. Then calls `00b5a8c0` on the object
/// `FN_00450B80(0)` returns with that byte, the three words at its `+0x1f0`
/// ([`fn_005bb9b0`]) and the two values (a stub in this build).
pub fn fn_005bb8c0(e: &mut Engine, a: ScriptArgs) -> bool {
    let unset = e.global::<u32>(FLOAT_MINUS_ONE);
    let Some([first, second]) = a.parse_into(e, [unset, unset]) else {
        return false;
    };
    let zero = e.global::<f64>(DOUBLE_ZERO);
    let (first_value, second_value) = (
        f64::from(f32::from_bits(first)),
        f64::from(f32::from_bits(second)),
    );
    // `FCOMP` against zero: above it, or not comparable (not a number).
    let above = |value: f64| value > zero || value.is_nan();
    if above(first_value) || above(second_value) {
        e.set_global(FLAG_011CAE47, 1u8);
    } else {
        let flag = e.global::<u8>(FLAG_011CAE47);
        e.set_global(FLAG_011CAE47, u8::from(flag == 0));
    }
    let object = e.call(FN_00450B80, &args![0u32]).u32();
    let vector = e.with_stack(12, |e, vector| {
        fn_005bb9b0(e, Ptr::new(object), vector);
        read_vector(e, vector.addr())
    });
    let flag = e.global::<u8>(FLAG_011CAE47);
    let object = e.call(FN_00450B80, &args![0u32]).u32();
    e.call(
        FN_00B5A8C0,
        &args![
            object,
            u32::from(flag),
            vector[0],
            vector[1],
            vector[2],
            first,
            second
        ],
    );
    true
}

// Translated from 005bb9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three words at `this + 0x1f0` to `out` and returns `out`.
pub fn fn_005bb9b0(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    for offset in [0, 4, 8] {
        let word = e.mem.u32(this.addr() + 0x1f0 + offset);
        e.mem.set_u32(out.addr() + offset, word);
    }
    out.addr()
}

/// Copies the nine words (a 3x3 matrix) at `from` to `to`.
fn copy_matrix(e: &mut Engine, from: u32, to: u32) {
    let bytes = e.mem.bytes(from, 36);
    e.mem.write(to, &bytes);
}

// Translated from 005bb9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses two `float`s into a 2D point (-1.0, the global `01012054`, by
/// default; the first is parsed into the point's second component). When
/// either value is zero it empties the list kept by the object
/// `FN_00450B80(0)` refers to (`00454b30`, `005bbc10`). Otherwise a negative
/// value becomes 50.0 (`0101b268`) and a 0xfc-byte object (`004a5ff0`) is built
/// from the position at `+0x1f0` of that object (`005bb9b0`), the 2D point and
/// a matrix: the matrix of the world scene's animation data (`0045c670`,
/// `006629f0`, `00461130`) times a rotation about z of pi / 2
/// (`004a0c90`, `0043f8d0`). The new object is held in a `NiPointer` (`00633c90`)
/// and added to that list (`00c5ae70`). (Exception unwinding frames are not
/// translated.)
pub fn fn_005bb9e0(e: &mut Engine, a: ScriptArgs) -> bool {
    e.with_stack(8, |e, point| {
        let point = point.addr();
        let unset = e.global::<f32>(FLOAT_MINUS_ONE);
        e.call(NI_POINT2_CONSTRUCT, &args![point, unset, unset]);
        if !a.parse(e, &[point + 4, point]) {
            return false;
        }
        let (x, y) = (e.mem.f32(point), e.mem.f32(point + 4));
        let zero = e.global::<f64>(DOUBLE_ZERO);
        if f64::from(x) == zero || f64::from(y) == zero {
            let object = e.call(FN_00450B80, &args![0u32]).u32();
            let list = e.call(FN_00454B30, &args![object]).u32();
            fn_005bbc10(e, Ptr::new(list));
            return true;
        }
        let default = e.global::<f32>(F32_0101B268);
        if f64::from(x) <= zero {
            e.mem.set_f32(point, default);
        }
        if f64::from(y) <= zero {
            e.mem.set_f32(point + 4, default);
        }
        let object = e.call(FN_00450B80, &args![0u32]).u32();
        e.with_stack(12, |e, position| {
            fn_005bb9b0(e, Ptr::new(object), position);
            let world = e.call(FN_0045C670, &args![]).u32();
            let animation = e.call(FN_006629F0, &args![world]).u32();
            let source = e.call(FN_00461130, &args![animation]).u32();
            e.with_stack(36, |e, matrix| {
                copy_matrix(e, source, matrix.addr());
                e.with_stack(36, |e, rotation| {
                    e.call(TRIVIAL_CONSTRUCT, &args![rotation]);
                    let angle = e.global::<f32>(FLOAT_HALF_PI);
                    e.call(NI_MATRIX_FROM_Z_ANGLE, &args![rotation, angle]);
                    e.with_stack(36, |e, product| {
                        let result = e
                            .call(NI_MATRIX_MULTIPLY, &args![matrix, product, rotation])
                            .u32();
                        copy_matrix(e, result, matrix.addr());
                    });
                });
                let block = e.call(NI_ALLOC, &args![0xfcu32]).u32();
                let created = if block != 0 {
                    e.call(FN_004A5FF0, &args![block, position, point, matrix])
                        .u32()
                } else {
                    0
                };
                e.with_stack(4, |e, held| {
                    e.call(NI_POINTER_INIT, &args![held, created]);
                    let object = e.call(NI_POINTER_GET, &args![held]).u32();
                    let owner = e.call(FN_00450B80, &args![0u32]).u32();
                    let list = e.call(FN_00454B30, &args![owner]).u32();
                    e.call(FN_00C5AE70, &args![list, object]);
                    e.call(NI_POINTER_DESTRUCT, &args![held]);
                });
            });
        });
        true
    })
}

// Translated from 005bbc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fastcall` (`ECX = this`): `NiTPointerListBase::RemoveAll` (`004ed900`, Xbox
/// PDB) on `this + 8`.
pub fn fn_005bbc10(e: &mut Engine, this: Ptr) {
    e.call(FN_004ED900, &args![this.addr() + 8]);
}

// Translated from 005bbc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flips the byte `011ad884`.
pub fn fn_005bbc30(e: &mut Engine) -> bool {
    let flag = e.global::<u8>(FLAG_011AD884);
    e.set_global(FLAG_011AD884, u8::from(flag == 0));
    true
}

// Translated from 005bbc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetGlobalRadialBlur` (Xbox PDB): stops the instance in the global
/// `011cae48` (`ImageSpaceModifierInstance::Stop`) and clears it, then parses
/// five `float`s. When they do not parse it prints the usage text if the
/// commands echo and returns false. Otherwise it triggers a radial blur
/// (`ImageSpaceModifierInstanceRB::Trigger`) from them with the 2D point
/// (0.5, 0.5, the global `01016248`) and 0.0, keeps the instance in the global
/// and, if there is one, sets bit 0 of its field `+0x18` ([`fn_005bbd60`]).
pub fn script_set_global_radial_blur(e: &mut Engine, a: ScriptArgs) -> bool {
    let instance = e.global::<u32>(RADIAL_BLUR_INSTANCE);
    if instance != 0 {
        e.call(IMAGE_SPACE_MODIFIER_INSTANCE_STOP, &args![instance]);
        e.set_global(RADIAL_BLUR_INSTANCE, 0u32);
    }
    let Some([strength, ramp_up, start, ramp_down, down_start]) = a.parse_into(e, [0; 5]) else {
        if echo_enabled(e) {
            console_print(e, &args![MSG_SET_GLOBAL_RADIAL_BLUR_USAGE]);
        }
        return false;
    };
    let instance = e.with_stack(8, |e, point| {
        let half = e.global::<f32>(F32_01016248);
        e.call(NI_POINT2_CONSTRUCT, &args![point, half, half]);
        e.call(
            RADIAL_BLUR_TRIGGER,
            &args![strength, ramp_up, start, ramp_down, down_start, point, 0.0f32],
        )
        .u32()
    });
    e.set_global(RADIAL_BLUR_INSTANCE, instance);
    if instance != 0 {
        fn_005bbd60(e, Ptr::new(instance), 1);
    }
    true
}

// Translated from 005bbd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` non-zero) or clears bit 0 of the dword at `this + 0x18`.
pub fn fn_005bbd60(e: &mut Engine, this: Ptr, set: u8) {
    let flags = e.mem.u32(this.addr() + 0x18);
    let flags = if set != 0 { flags | 1 } else { flags & !1 };
    e.mem.set_u32(this.addr() + 0x18, flags);
}

// Translated from 005bbda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleAiDetection` (Xbox PDB): flips the byte `011f1221` and
/// echoes "AI Detection is  On" or "Off".
pub fn script_toggle_ai_detection(e: &mut Engine) -> bool {
    let flag = e.global::<u8>(AI_DETECTION_FLAG);
    e.set_global(AI_DETECTION_FLAG, u8::from(flag == 0));
    if echo_enabled(e) {
        let flag = e.global::<u8>(AI_DETECTION_FLAG);
        console_print(e, &args![MSG_AI_DETECTION_IS, on_off(flag != 0)]);
    }
    true
}

/// Prints "<name> processing is  <On|Off>" for an actor after the byte at
/// `+0xbc` changed, when the commands echo.
fn print_actor_processing(e: &mut Engine, actor: u32) {
    if echo_enabled(e) {
        let state = on_off(e.mem.u8(actor + ACTOR_AI_FLAG) != 0);
        let name = e.call(REFR_GET_NAME_TEXT, &args![actor]).u32();
        console_print(e, &args![MSG_PROCESSING_IS, name, state]);
    }
}

// Translated from 005bbe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// For the reference as an `Actor` (none does nothing): flips the byte at
/// `+0xbc` and echoes "<name> processing is  On" or "Off".
pub fn fn_005bbe10(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = cast_to_actor(e, a.this_obj);
    if actor != 0 {
        let flag = e.mem.u8(actor + ACTOR_AI_FLAG);
        e.mem.set_u8(actor + ACTOR_AI_FLAG, u8::from(flag == 0));
        print_actor_processing(e, actor);
    }
    true
}

// Translated from 005bbec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For the reference as an `Actor` (none does nothing): writes 1.0 to the
/// result `double` when the byte at `+0xbc` is zero, otherwise 0.0, and echoes
/// "<name> processing is  On" or "Off".
pub fn fn_005bbec0(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = cast_to_actor(e, a.this_obj);
    if actor != 0 {
        let flag = e.mem.u8(actor + ACTOR_AI_FLAG);
        e.mem
            .set_f64(a.result.addr(), if flag == 0 { 1.0 } else { 0.0 });
        print_actor_processing(e, actor);
    }
    true
}

// Translated from 005bbf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleAI` (Xbox PDB): parses an optional actor reference (the
/// reference the command runs on, as an `Actor`, when it is omitted). For an
/// actor it flips the byte at `+0xbc`, calls `Actor::EndMovement` and echoes
/// "<name> processing is  On" or "Off". Without an actor it flips the byte
/// `011f1220`, echoes "All AI Processing is  On" or "Off" and, when the byte
/// became zero, calls `ProcessLists::TurnOffMovementinHigh`.
pub fn script_toggle_ai(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([parsed]) = a.parse_into(e, [0]) else {
        return false;
    };
    let actor = if parsed == 0 {
        cast_to_actor(e, a.this_obj)
    } else {
        parsed
    };
    if actor != 0 {
        let flag = e.mem.u8(actor + ACTOR_AI_FLAG);
        e.mem.set_u8(actor + ACTOR_AI_FLAG, u8::from(flag == 0));
        e.call(ACTOR_END_MOVEMENT, &args![actor]);
        print_actor_processing(e, actor);
    } else {
        let flag = e.global::<u8>(ALL_AI_FLAG);
        e.set_global(ALL_AI_FLAG, u8::from(flag == 0));
        if echo_enabled(e) {
            let flag = e.global::<u8>(ALL_AI_FLAG);
            console_print(e, &args![MSG_ALL_AI_PROCESSING_IS, on_off(flag != 0)]);
        }
        if e.global::<u8>(ALL_AI_FLAG) == 0 {
            e.call(
                PROCESS_LISTS_TURN_OFF_MOVEMENT_IN_HIGH,
                &args![PROCESS_LISTS],
            );
        }
    }
    true
}

// Translated from 005bc0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleCombatAI` (Xbox PDB): flips the combat AI byte
/// ([`fn_005bc130`]) and echoes "All Combat AI processing is On" or "Off".
pub fn script_toggle_combat_ai(e: &mut Engine) -> bool {
    let flag = fn_005bc130(e);
    if echo_enabled(e) {
        console_print(e, &args![MSG_ALL_COMBAT_AI_PROCESSING_IS, on_off(flag)]);
    }
    true
}

// Translated from 005bc130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flips the combat AI byte `011a4880` and returns the new value.
pub fn fn_005bc130(e: &mut Engine) -> bool {
    let flag = e.global::<u8>(COMBAT_AI_FLAG);
    e.set_global(COMBAT_AI_FLAG, u8::from(flag == 0));
    e.global::<u8>(COMBAT_AI_FLAG) != 0
}

// Translated from 005bc150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an `int` and ignores it: whether the arguments parse.
pub fn fn_005bc150(e: &mut Engine, a: ScriptArgs) -> bool {
    a.parse_into(e, [0]).is_some()
}

// Translated from 005bc1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an `int` and sets the byte `011df678` to whether it is non-zero.
pub fn fn_005bc1a0(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = a.parse_into(e, [0]) else {
        return false;
    };
    e.set_global(FLAG_011DF678, u8::from(value != 0));
    true
}

// Translated from 005bc200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an `int` and sets the byte `012682f8` to whether it is non-zero.
pub fn fn_005bc200(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([value]) = a.parse_into(e, [0]) else {
        return false;
    };
    e.set_global(FLAG_012682F8, u8::from(value != 0));
    true
}

// Translated from 005bc260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an `int` mode. Unless the mode is 4 it asks the player
/// (`00961e30`) with 1 for modes 1, 2 and 5 and 0 for the others. Then calls
/// `00ba8b60` with 1 when that answer is non-zero and the mode is at least 1,
/// otherwise 0.
pub fn fn_005bc260(e: &mut Engine, a: ScriptArgs) -> bool {
    let Some([mode]) = a.parse_into(e, [0]) else {
        return false;
    };
    let mode = mode as i32;
    let mut answer = 0u8;
    if mode != 4 {
        let flag = (1..=2).contains(&mode) || mode == 5;
        let player = e.global::<u32>(PLAYER);
        answer = e.call(FN_00961E30, &args![player, u32::from(flag)]).u8();
    }
    e.call(FN_00BA8B60, &args![u32::from(answer != 0 && mode >= 1)]);
    true
}

// Translated from 005bc310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ShowHeadTrackTarget` (Xbox PDB): for the reference as an `Actor`
/// (none does nothing) looks up the head track target of its extra data
/// (`ExtraDataList::GetHeadTrackTargetExtra`) and, when the commands echo,
/// prints "<name> set to look at <target name>" or "<name> set to not look at
/// anyone".
pub fn script_show_head_track_target(e: &mut Engine, a: ScriptArgs) -> bool {
    let actor = cast_to_actor(e, a.this_obj);
    if actor != 0 {
        let list = e.call(EXTRA_DATA_LIST, &args![actor]).u32();
        let target = e.call(HEAD_TRACK_TARGET_EXTRA, &args![list]).u32();
        if echo_enabled(e) {
            if target != 0 {
                let target_name = e.call(REFR_GET_NAME_TEXT, &args![target]).u32();
                let name = e.call(REFR_GET_NAME_TEXT, &args![actor]).u32();
                console_print(e, &args![MSG_LOOKING_AT, name, target_name]);
            } else {
                let name = e.call(REFR_GET_NAME_TEXT, &args![actor]).u32();
                console_print(e, &args![MSG_NOT_LOOKING_AT_ANYONE, name]);
            }
        }
    }
    true
}

// Translated from 005bc3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ToggleHighProcess` (Xbox PDB): flips the byte `011f1228`, echoes
/// "AI Processing for actors in high is  On" or "Off" and, when the byte
/// became zero, calls `ProcessLists::TurnOffMovementinHigh`.
pub fn script_toggle_high_process(e: &mut Engine) -> bool {
    let flag = e.global::<u8>(HIGH_PROCESS_FLAG);
    e.set_global(HIGH_PROCESS_FLAG, u8::from(flag == 0));
    if echo_enabled(e) {
        let flag = e.global::<u8>(HIGH_PROCESS_FLAG);
        console_print(e, &args![MSG_HIGH_AI_PROCESSING_IS, on_off(flag != 0)]);
    }
    if e.global::<u8>(HIGH_PROCESS_FLAG) == 0 {
        e.call(
            PROCESS_LISTS_TURN_OFF_MOVEMENT_IN_HIGH,
            &args![PROCESS_LISTS],
        );
    }
    true
}

// ---- n045: three bodies Ghidra made no function of (read from `disassemble_bytes`) ----------
//
// The three bodies keep an aligned frame (`EBX` holds the caller's `ESP`, the eight words at
// `EBX + 8`); their locals are translated as a stack block addressed from the frame base, at
// the exe's own `EBP` offsets.

/// The player reference pointer (`011dea3c`) and the `TES` pointer `TES::Pick` runs on
/// (`011dea10`).
const N045_PLAYER: u32 = 0x011d_ea3c;
const N045_TES: u32 = 0x011d_ea10;
/// The data handler pointer (`011c3f2c`) whose `004698a0` (`RET 0x20`) places a new
/// reference: form, position, rotation, two words from the anchor reference (`008d6f30` and
/// `00575d70`, the world space), then three zero words.
const N045_DATA_HANDLER: u32 = 0x011c_3f2c;
const N045_PLACE_REFERENCE: u32 = 0x0046_98a0;
/// The callees, by exe address: `00430830` (a point of a reference, three floats), `00416870` (`RET 0xC`,
/// sets a point from three floats), `00c82870` (`bhkCollisionFilter::SetMultiLayerCollision`,
/// `cdecl`, three words), `004a3c20` (constructs the pick data), `008c71b0` (`RET 4`,
/// constructs a flags word), `004a39f0` (`RET 4`, sets a flag), `005dbe10` (the next pick
/// id, a counter at `011b0808` that skips 0 by restarting at 10), `0059ce80` (`RET 4`),
/// `004a3f70`, `004a3da0`, `004a3eb0` (`RET 4`: the pick data's filter, start and end),
/// `TES::Pick` (`00458420`, `RET 4`), `005c53f0` (`RET 4`), `005609f0`, `00559450` (the
/// first word), `004a3a40` (`cdecl`), `005dbe40` (`RET 4`), `005e05d0` (`RET 8`),
/// `0043b230`, `005e05b0` (`RET 4`), `00c84f50` (`RET 8`), `005e0600`, `006815c0` (answers
/// `this`), `PathingLocation::PathingLocation_ov3` (`006dcd70`, `RET 4`),
/// `Pathing::GetNavmeshNormalForLocation` (`006d9100`, `cdecl`), `004ff7e0`, `00458620`
/// (`cdecl`), `BSUtilities::GetRotationZToVectorTranspose` (`00c4b8a0`, `cdecl`, an output
/// and a point by value), `NiMatrix3::ToEulerAnglesXYZ` (`00a592c0`, `RET 0xC`), `005dbe60`
/// (`RET 4`), `TESObjectREFR::GetWorldSpace` (`00575d70`), `008d6f30`, `00564c60`,
/// `0056c780`, `TESObjectREFR::SetRefPersists` (`00565480`), `005d43c0` (a list),
/// `0041e340` (`RET 4`, adds to it), `0040ebd0` (`cdecl`, two floats, a float in `ST0`),
/// `0045c670`, `006629f0`, `00461130` (a 3x3 matrix), `005df560` (`RET 0x10`, reads a
/// matrix row), `005df510` (`RET 8`, writes a row), `005df5b0` (+0xDE0 of the player),
/// `004b4500` (`RET 8`), `00439e90` (`RET 8`), `00931ed0` (`RET 4`), `004a3a20`,
/// `TESHealthForm::GetFormAsHealthForm` (`004872e0`, `cdecl`), `00451110` (`RET 4`),
/// `Actor::FadeIn` (`008bcd20`), `00568bd0` (`RET 4`, a float),
/// `TESObjectREFR::FindReferenceFor3D` (`0056f930`, `cdecl`), `0084e3a0` (the form id) and
/// `Script::PutNumericIDInDouble` (`005acc70`, `cdecl`).
const N045_REFERENCE_POINT: u32 = 0x0043_0830;
const N045_SET_POINT: u32 = 0x0041_6870;
const N045_SET_MULTI_LAYER_COLLISION: u32 = 0x00c8_2870;
const N045_PICK_DATA_CONSTRUCT: u32 = 0x004a_3c20;
const N045_FLAGS_CONSTRUCT: u32 = 0x008c_71b0;
const N045_FLAGS_SET: u32 = 0x004a_39f0;
const N045_NEXT_PICK_ID: u32 = 0x005d_be10;
const N045_FLAGS_COMBINE: u32 = 0x0059_ce80;
const N045_PICK_SET_FILTER: u32 = 0x004a_3f70;
const N045_PICK_SET_START: u32 = 0x004a_3da0;
const N045_PICK_SET_END: u32 = 0x004a_3eb0;
const N045_TES_PICK: u32 = 0x0045_8420;
const N045_PICK_RESULT_OBJECT: u32 = 0x005c_53f0;
const N045_RESULT_TEST: u32 = 0x0056_09f0;
const N045_FIRST_WORD: u32 = 0x0055_9450;
const N045_OBJECT_OF: u32 = 0x004a_3a40;
const N045_OBJECT_TEST: u32 = 0x005d_be40;
const N045_INDEX_LIST_CONSTRUCT: u32 = 0x005e_05d0;
const N045_OBJECT_KIND: u32 = 0x0043_b230;
const N045_INDEX_LIST_AT: u32 = 0x005e_05b0;
const N045_INDEX_LIST_DESTRUCT: u32 = 0x005e_0600;
const N045_LAYER_TEST: u32 = 0x00c8_4f50;
const N045_SELF: u32 = 0x0068_15c0;
const N045_PATHING_LOCATION: u32 = 0x006d_cd70;
const N045_NAVMESH_NORMAL: u32 = 0x006d_9100;
const N045_PATHING_LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
const N045_PICK_NORMAL: u32 = 0x0045_8620;
const N045_ROTATION_TO_VECTOR: u32 = 0x00c4_b8a0;
const N045_TO_EULER_ANGLES: u32 = 0x00a5_92c0;
const N045_PICK_POINT: u32 = 0x005d_be60;
const N045_WORLD_SPACE: u32 = 0x0057_5d70;
const N045_PARENT_CELL: u32 = 0x008d_6f30;
const N045_REFERENCE_FLAG_564C60: u32 = 0x0056_4c60;
const N045_REFERENCE_FLAG_56C780: u32 = 0x0056_c780;
const N045_SET_REF_PERSISTS: u32 = 0x0056_5480;
const N045_REFERENCE_LIST: u32 = 0x005d_43c0;
const N045_LIST_ADD: u32 = 0x0041_e340;
const N045_ANGLE_FUNCTION: u32 = 0x0040_ebd0;
const N045_SINGLETON_45C670: u32 = 0x0045_c670;
const N045_ANIMATION_DATA: u32 = 0x0066_29f0;
const N045_MATRIX_OF: u32 = 0x0046_1130;
const N045_MATRIX_GET_ROW: u32 = 0x005d_f560;
const N045_MATRIX_SET_ROW: u32 = 0x005d_f510;
const N045_PLAYER_POINT: u32 = 0x005d_f5b0;
const N045_MATRIX_TIMES: u32 = 0x004b_4500;
const N045_POINT_PLUS: u32 = 0x0043_9e90;
const N045_MOBILE_931ED0: u32 = 0x0093_1ed0;
const N045_FLAGS_OF: u32 = 0x004a_3a20;
const N045_HEALTH_FORM: u32 = 0x0048_72e0;
const N045_PICK_HIT: u32 = 0x0045_1110;
const N045_FADE_IN: u32 = 0x008b_cd20;
const N045_SET_SCALE_LIKE: u32 = 0x0056_8bd0;
const N045_FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
const N045_FORM_ID: u32 = 0x0084_e3a0;
const N045_PUT_NUMERIC_ID_IN_DOUBLE: u32 = 0x005a_cc70;
/// The constants the bodies read: two doubles added to and taken from a height (`0102f070`,
/// `010231d8`), the float `0103d24c` given to `0040ebd0`, the double `01017a40` a health
/// value is multiplied by, and the two forms `005db870` chooses between (`011ca280` for mode
/// 2, else `011ca27c`).
const N045_HEIGHT_UP: u32 = 0x0102_f070;
const N045_HEIGHT_DOWN: u32 = 0x0102_31d8;
const N045_ANGLE_CONSTANT: u32 = 0x0103_d24c;
const N045_HEALTH_SCALE: u32 = 0x0101_7a40;
const N045_FORM_MODE_2: u32 = 0x011c_a280;
const N045_FORM_OTHER: u32 = 0x011c_a27c;

/// Copies the three words at `from` to `to` (a point copied by value).
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 005db870 (PC disassembly, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005db870 - the FS:[0] exception frame and the /GS stack cookie check (00ec408c) are not translated.
/// A script command body (no Ghidra function, no map name): with its one parameter (a mode)
/// parsed and an actor this reference (vtable slot 0x100), it picks from the reference's
/// position (slot 0x1F4) a little above and below (the doubles at `0102f070` and `010231d8`),
/// with a new pick id (`005dbe10`) and filter flag 0x27 (`TES::Pick`). When the pick hits
/// (`005c53f0`, `005609f0`), the hit object's layer is tested (`005dbe40` with 0x40, or by
/// walking its index list (`005e05d0`) to its last valid node, `00c84f50`), the normal is
/// taken from the navmesh (`Pathing::GetNavmeshNormalForLocation`) or the pick
/// (`00458620`), turned into a rotation (`00c4b8a0`, `NiMatrix3::ToEulerAnglesXYZ`) and the
/// point is the pick's (`005dbe60`). A reference of the form `011ca280` (mode 2) or
/// `011ca27c` is placed there (`004698a0`), its flags set (`00564c60` with "no rotation",
/// `0056c780`, `SetRefPersists`), linked both ways with this reference (`0041e340`) and
/// both marked changed (slot 0x48 with 0x80000000). Answers false only when the parameters
/// do not parse. The exception frame and the stack cookie check are not translated.
pub fn fn_005db870(e: &mut Engine, args: ScriptArgs) -> bool {
    e.with_stack(0x1a0, |e, frame| {
        let ebp = frame.addr() + 0x1a0;
        let at = |offset: u32| ebp - offset;
        e.mem.set_u32(at(0x18), 0);
        if !args.parse(e, &[at(0x18)]) {
            return false;
        }
        let this = args.this_obj.addr();
        if this == 0 || !e.vcall(this, 0x100, &[]).bool() {
            return true;
        }
        let kind = e.vcall(this, 0x1d0, &[]).u32();
        e.mem.set_u32(at(0x1c), kind);
        let position = e.vcall(this, 0x1f4, &[]).u32();
        copy_point(e, at(0x28), position);
        let location = e.call(N045_REFERENCE_POINT, &args![this]).u32();
        let height = x87_quiet_bits(e.mem.u32(location + 8));
        e.call(N045_SET_POINT, &args![at(0x34), 0u32, 0u32, height]);
        e.mem.set_u8(at(0x35), 0);
        e.mem.set_u32(at(0x3c), 0x0802_2802);
        let layers = e.mem.u32(at(0x3c));
        e.call(
            N045_SET_MULTI_LAYER_COLLISION,
            &args![0x27u32, layers, 0u32],
        );
        e.call(N045_PICK_DATA_CONSTRUCT, &args![at(0xf0)]);
        e.call(N045_FLAGS_CONSTRUCT, &args![at(0xf4), 0u32]);
        e.call(N045_FLAGS_SET, &args![at(0xf4), 0x27u32]);
        let id = e.call(N045_NEXT_PICK_ID, &[]).u32();
        e.call(N045_FLAGS_COMBINE, &args![at(0xf4), id]);
        let flags = e.mem.u32(at(0xf4));
        e.call(N045_PICK_SET_FILTER, &args![at(0xf0), flags]);
        copy_point(e, at(0x100), at(0x28));
        let up = x87_add(
            x87_load32(e.mem.u32(at(0xf8))),
            e.global::<f64>(N045_HEIGHT_UP),
        );
        e.mem.set_u32(at(0xf8), x87_store32(up));
        copy_point(e, at(0x10c), at(0x28));
        let down = x87_sub(
            x87_load32(e.mem.u32(at(0x104))),
            e.global::<f64>(N045_HEIGHT_DOWN),
        );
        e.mem.set_u32(at(0x104), x87_store32(down));
        e.call(N045_PICK_SET_START, &args![at(0xf0), at(0x100)]);
        e.call(N045_PICK_SET_END, &args![at(0xf0), at(0x10c)]);
        let tes = e.global::<u32>(N045_TES);
        e.call(N045_TES_PICK, &args![tes, at(0xf0)]);
        let result = e
            .call(N045_PICK_RESULT_OBJECT, &args![at(0xc0), at(0x10d)])
            .u32();
        if e.call(N045_RESULT_TEST, &args![result]).bool() {
            e.mem.set_u8(at(0x10e), 0);
            let holder = e.mem.u32(at(0x70));
            let first = e.call(N045_FIRST_WORD, &args![holder]).u32();
            e.mem.set_u32(at(0x114), first);
            let object = e.call(N045_OBJECT_OF, &args![first]).u32();
            e.mem.set_u32(at(0x118), object);
            let hit = e.call(N045_OBJECT_TEST, &args![object, 0x40u32]).u8();
            e.mem.set_u8(at(0x10e), hit);
            if e.mem.u8(at(0x10e)) == 0 {
                walk_hit_layers(e, ebp);
            }
            e.call(N045_SELF, &args![at(0x144)]);
            if e.mem.u8(at(0x10e)) != 0 {
                e.call(N045_PATHING_LOCATION, &args![at(0x16c), this]);
                e.call(N045_NAVMESH_NORMAL, &args![at(0x16c), at(0x144)]);
                e.call(N045_PATHING_LOCATION_DESTRUCT, &args![at(0x16c)]);
            } else {
                e.call(N045_PICK_NORMAL, &args![at(0x144), at(0xc0)]);
            }
            let (x, y, z) = (
                e.mem.u32(at(0x144)),
                e.mem.u32(at(0x140)),
                e.mem.u32(at(0x13c)),
            );
            e.call(N045_ROTATION_TO_VECTOR, &args![at(0x190), x, y, z]);
            e.call(
                N045_TO_EULER_ANGLES,
                &args![at(0x190), at(0x34), at(0x30), at(0x2c)],
            );
            e.mem.set_u8(at(0x35), 1);
            e.call(N045_PICK_POINT, &args![at(0xf0), at(0x28)]);
        }
        let form = if e.mem.u32(at(0x18)) == 2 {
            e.global::<u32>(N045_FORM_MODE_2)
        } else {
            e.global::<u32>(N045_FORM_OTHER)
        };
        e.mem.set_u32(at(0x194), form);
        e.mem.set_u32(at(0x198), form);
        let world_space = e.call(N045_WORLD_SPACE, &args![this]).u32();
        let cell = e.call(N045_PARENT_CELL, &args![this]).u32();
        let handler = e.global::<u32>(N045_DATA_HANDLER);
        let form = e.mem.u32(at(0x198));
        let placed = e
            .call(
                N045_PLACE_REFERENCE,
                &args![
                    handler,
                    form,
                    at(0x28),
                    at(0x34),
                    cell,
                    world_space,
                    0u32,
                    0u32,
                    0u32
                ],
            )
            .u32();
        e.mem.set_u32(at(0x19c), placed);
        let unrotated = u32::from(e.mem.u8(at(0x35)) == 0);
        e.call(N045_REFERENCE_FLAG_564C60, &args![placed, unrotated]);
        e.call(N045_REFERENCE_FLAG_56C780, &args![placed, 1u32]);
        e.call(N045_SET_REF_PERSISTS, &args![placed, 1u32]);
        let list = e.call(N045_REFERENCE_LIST, &args![placed]).u32();
        e.call(N045_LIST_ADD, &args![list, this]);
        let list = e.call(N045_REFERENCE_LIST, &args![this]).u32();
        e.call(N045_LIST_ADD, &args![list, placed]);
        e.vcall(placed, 0x48, &args![0x8000_0000u32]);
        e.vcall(this, 0x48, &args![0x8000_0000u32]);
        true
    })
}

/// `005db870`'s walk of the hit object's nodes when `005dbe40` refused it: an index list of 8
/// (`005e05d0`) and up to eight indices from the pick result (`EBP - 0xA0`, -1 ends them); a
/// node of kind 0xA (`0043b230`) skips one index; otherwise the node and index are kept and
/// the walk goes to the child (slot 0x14 of slot 0x10's object, with the index and the list
/// entry `005e05b0`). The kept node's object (`004a3a40`), when there is one, gives the layer
/// test (`00c84f50` with the index and 0x40). The list is destroyed (`005e0600`).
fn walk_hit_layers(e: &mut Engine, ebp: u32) {
    let at = |offset: u32| ebp - offset;
    let index_at = |i: u32| at(0xa0).wrapping_add(i.wrapping_mul(4));
    let node = e.mem.u32(at(0x114));
    e.mem.set_u32(at(0x11c), node);
    e.mem.set_u32(at(0x120), 0);
    e.mem.set_u32(at(0x124), 0xffff_ffff);
    e.call(N045_INDEX_LIST_CONSTRUCT, &args![at(0x12c), 8u32, 0u32]);
    let mut i = 0i32;
    while i < 8 && e.mem.u32(index_at(i as u32)) != 0xffff_ffff {
        let node = e.mem.u32(at(0x11c));
        if e.call(N045_OBJECT_KIND, &args![node]).u32() == 0xa {
            i += 1;
            if i >= 8 || e.mem.u32(index_at(i as u32)) == 0xffff_ffff {
                break;
            }
        }
        let index = e.mem.u32(index_at(i as u32));
        e.mem.set_u32(at(0x124), index);
        let node = e.mem.u32(at(0x11c));
        e.mem.set_u32(at(0x120), node);
        let holder = e.vcall(node, 0x10, &[]).u32();
        e.mem.set_u32(at(0x134), holder);
        let entry = e
            .call(N045_INDEX_LIST_AT, &args![at(0x12c), i as u32])
            .u32();
        let index = e.mem.u32(index_at(i as u32));
        let child = e.vcall(holder, 0x14, &args![index, entry]).u32();
        e.mem.set_u32(at(0x11c), child);
        i += 1;
    }
    let kept = e.mem.u32(at(0x120));
    let object = e.call(N045_OBJECT_OF, &args![kept]).u32();
    e.mem.set_u32(at(0x138), object);
    if object != 0 {
        let index = e.mem.u32(at(0x124));
        let hit = e.call(N045_LAYER_TEST, &args![object, index, 0x40u32]).u8();
        e.mem.set_u8(at(0x10e), hit);
    }
    e.call(N045_INDEX_LIST_DESTRUCT, &args![at(0x12c)]);
}

/// The front `005df040` and `005df5d0` share (frame base `ebp`): the angle parameter
/// (`EBP - 0x14`) through `0040ebd0` with `0103d24c`, a pick data, a point (0, angle, 0), the
/// player's position, the transposed head matrix of the player's animation data (`0045c670`,
/// `006629f0`, `00461130`, rows moved with `005df560`/`005df510`), the look point from the
/// player's point (`005df5b0`) through `004b4500` and `00439e90`, the player's flags
/// (`00931ed0`, `004a3a20`, `0059ce80` on a flags word with 6), and `TES::Pick` from the
/// position to that point; answers the pick's result.
fn pick_from_player(e: &mut Engine, ebp: u32) -> u32 {
    let at = |offset: u32| ebp - offset;
    let angle = x87_quiet_bits(e.mem.u32(at(0x14)));
    let constant = x87_quiet_bits(e.global::<u32>(N045_ANGLE_CONSTANT));
    let turned = e.call(N045_ANGLE_FUNCTION, &args![angle, constant]).f64();
    e.mem.set_u32(at(0x14), x87_store32(turned));
    e.call(N045_PICK_DATA_CONSTRUCT, &args![at(0xd0)]);
    let angle = x87_quiet_bits(e.mem.u32(at(0x14)));
    e.call(N045_SET_POINT, &args![at(0xdc), 0u32, angle, 0u32]);
    let player = e.global::<u32>(N045_PLAYER);
    let position = e.vcall(player, 0x1f4, &[]).u32();
    copy_point(e, at(0xe8), position);
    let singleton = e.call(N045_SINGLETON_45C670, &[]).u32();
    let data = e.call(N045_ANIMATION_DATA, &args![singleton]).u32();
    e.mem.set_u32(at(0xec), data);
    let matrix = e.call(N045_MATRIX_OF, &args![data]).u32();
    for i in 0..9 {
        let word = e.mem.u32(matrix + 4 * i);
        e.mem.set_u32(at(0x110) + 4 * i, word);
    }
    e.call(N045_SELF, &args![at(0x134)]);
    e.call(N045_SELF, &args![at(0x140)]);
    for row in 0..3u32 {
        e.call(
            N045_MATRIX_GET_ROW,
            &args![at(0x110), row, at(0x13c), at(0x138), at(0x140)],
        );
        e.call(N045_MATRIX_SET_ROW, &args![at(0x134), row, at(0x140)]);
    }
    let player = e.global::<u32>(N045_PLAYER);
    let point = e.call(N045_PLAYER_POINT, &args![player]).u32();
    copy_point(e, at(0xe8), point);
    let product = e
        .call(N045_MATRIX_TIMES, &args![at(0x134), at(0x158), at(0xdc)])
        .u32();
    let sum = e
        .call(N045_POINT_PLUS, &args![product, at(0x14c), at(0xe8)])
        .u32();
    copy_point(e, at(0xdc), sum);
    let player = e.global::<u32>(N045_PLAYER);
    e.call(N045_MOBILE_931ED0, &args![player, at(0x15c)]);
    e.call(N045_FLAGS_CONSTRUCT, &args![at(0x160), 0u32]);
    e.call(N045_FLAGS_SET, &args![at(0x160), 6u32]);
    let player = e.global::<u32>(N045_PLAYER);
    let source = e.call(N045_MOBILE_931ED0, &args![player, at(0x164)]).u32();
    let flags = e.call(N045_FLAGS_OF, &args![source]).u32();
    e.call(N045_FLAGS_COMBINE, &args![at(0x160), flags]);
    let flags = e.mem.u32(at(0x160));
    e.call(N045_PICK_SET_FILTER, &args![at(0xd0), flags]);
    e.call(N045_PICK_SET_START, &args![at(0xd0), at(0xe8)]);
    e.call(N045_PICK_SET_END, &args![at(0xd0), at(0xdc)]);
    let tes = e.global::<u32>(N045_TES);
    let picked = e.call(N045_TES_PICK, &args![tes, at(0xd0)]).u32();
    e.mem.set_u32(at(0x168), picked);
    picked
}

/// The locals both bodies set before parsing: the result 0.0, a form word 0, a flag 1 and two
/// floats 0.0 (`EBP - 8` to `EBP - 0x14`).
fn clear_pick_locals(e: &mut Engine, args: ScriptArgs, ebp: u32) {
    e.mem.set_f64(args.result.addr(), 0.0);
    e.mem.set_u32(ebp - 8, 0);
    e.mem.set_u32(ebp - 0xc, 1);
    e.mem.set_u32(ebp - 0x10, x87_store32(0.0));
    e.mem.set_u32(ebp - 0x14, x87_store32(0.0));
}

// Translated from 005df040 (PC disassembly, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005df040 - the /GS stack cookie check (00ec408c) is not translated.
/// A script command body (no Ghidra function, no map name): parses a form, a flag and two
/// floats, picks from the player (`pick_from_player`) and takes the pick's point
/// (`005dbe60`, else the player's position). For a form that passes slot 0xE4 and a pick that
/// hit (`00451110`), sets the result to 1.0 and places a reference of the form there
/// (`004698a0` at the player's position, cell and world space); none answers false. An actor
/// placed (slot 0x100) gets slot 0x41C with 0.0 and fades in (`Actor::FadeIn`); a form with
/// health (`TESHealthForm::GetFormAsHealthForm`) gives it its health (slot 0x10, as a float,
/// times the double `01017a40`, `00568bd0`). Answers true otherwise, false when the parameters
/// do not parse. The stack cookie check is not translated.
pub fn fn_005df040(e: &mut Engine, args: ScriptArgs) -> bool {
    e.with_stack(0x1b0, |e, frame| {
        let ebp = frame.addr() + 0x1b0;
        let at = |offset: u32| ebp - offset;
        clear_pick_locals(e, args, ebp);
        if !args.parse(e, &[at(8), at(0xc), at(0x10), at(0x14)]) {
            return false;
        }
        pick_from_player(e, ebp);
        e.call(N045_SELF, &args![at(0x174)]);
        e.call(N045_PICK_POINT, &args![at(0xd0), at(0x174)]);
        let player = e.global::<u32>(N045_PLAYER);
        let position = e.vcall(player, 0x1f4, &[]).u32();
        copy_point(e, at(0x180), position);
        let form = e.mem.u32(at(8));
        if !e.vcall(form, 0xe4, &[]).bool() {
            return true;
        }
        let form = e.mem.u32(at(8));
        e.mem.set_u32(at(0x184), form);
        let health = e.call(N045_HEALTH_FORM, &args![form]).u32();
        e.mem.set_u32(at(0x188), health);
        let tes = e.global::<u32>(N045_TES);
        if !e.call(N045_PICK_HIT, &args![tes, at(0x174)]).bool() {
            return true;
        }
        copy_point(e, at(0x180), at(0x174));
        e.mem.set_f64(args.result.addr(), 1.0);
        let player = e.global::<u32>(N045_PLAYER);
        let world_space = e.call(N045_WORLD_SPACE, &args![player]).u32();
        let player = e.global::<u32>(N045_PLAYER);
        let cell = e.call(N045_PARENT_CELL, &args![player]).u32();
        let player = e.global::<u32>(N045_PLAYER);
        let point = e.call(N045_REFERENCE_POINT, &args![player]).u32();
        let handler = e.global::<u32>(N045_DATA_HANDLER);
        let form = e.mem.u32(at(0x184));
        let placed = e
            .call(
                N045_PLACE_REFERENCE,
                &args![
                    handler,
                    form,
                    at(0x180),
                    point,
                    cell,
                    world_space,
                    0u32,
                    0u32,
                    0u32
                ],
            )
            .u32();
        e.mem.set_u32(at(0x18c), placed);
        if placed == 0 {
            return false;
        }
        if e.vcall(placed, 0x100, &[]).bool() {
            e.mem.set_u32(at(0x190), placed);
            e.vcall(placed, 0x41c, &args![x87_store32(0.0)]);
            e.call(N045_FADE_IN, &args![placed]);
        }
        let health = e.mem.u32(at(0x188));
        if health != 0 {
            let value = e.vcall(health, 0x10, &[]).u32();
            e.mem.set_u32(at(0x198), value);
            e.mem.set_u32(at(0x194), 0);
            let whole = f64::from(value);
            e.mem.set_u32(at(0x19c), x87_store32(whole));
            let scaled = x87_mul(
                x87_load32(e.mem.u32(at(0x19c))),
                e.global::<f64>(N045_HEALTH_SCALE),
            );
            e.mem.set_u32(at(0x19c), x87_store32(scaled));
            let scaled = x87_quiet_bits(e.mem.u32(at(0x19c)));
            e.call(N045_SET_SCALE_LIKE, &args![placed, scaled]);
        }
        true
    })
}

// Translated from 005df5d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005df5d0 - the /GS stack cookie check (00ec408c) is not translated.
/// A script command body (no Ghidra function, no map name): parses two floats, picks from the
/// player (`pick_from_player`) and, when the pick found a 3D object whose reference
/// `TESObjectREFR::FindReferenceFor3D` gives, writes that reference's form id into the result
/// (`Script::PutNumericIDInDouble`); the result stays 0.0 otherwise. Answers true, false when
/// the parameters do not parse. The stack cookie check is not translated.
pub fn fn_005df5d0(e: &mut Engine, args: ScriptArgs) -> bool {
    e.with_stack(0x180, |e, frame| {
        let ebp = frame.addr() + 0x180;
        let at = |offset: u32| ebp - offset;
        clear_pick_locals(e, args, ebp);
        if !args.parse(e, &[at(0x10), at(0x14)]) {
            return false;
        }
        let picked = pick_from_player(e, ebp);
        let reference = if picked != 0 {
            e.call(N045_FIND_REFERENCE_FOR_3D, &args![picked]).u32()
        } else {
            0
        };
        e.mem.set_u32(at(0x16c), reference);
        e.mem.set_u32(at(0x170), reference);
        if reference != 0 {
            let id = e.call(N045_FORM_ID, &args![reference]).u32();
            e.mem.set_u32(at(0x174), id);
            e.call(
                N045_PUT_NUMERIC_ID_IN_DOUBLE,
                &args![at(0x174), args.result.addr()],
            );
        }
        true
    })
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005b4b20, script_get_distance_function(ScriptArgs) -> bool),
        entry!(0x005b4b80, script_get_in_zone_function(ScriptArgs) -> bool),
        entry!(0x005b4be0, fn_005b4be0(ScriptArgs) -> bool),
        entry!(0x005b4e90, fn_005b4e90(ScriptArgs) -> bool),
        entry!(0x005b53d0, fn_005b53d0(ScriptArgs) -> bool),
        entry!(
            0x005b5500,
            script_duplicate_all_items_function(ScriptArgs) -> bool
        ),
        entry!(0x005b55a0, fn_005b55a0(ScriptArgs) -> bool),
        entry!(0x005b5690, fn_005b5690(ScriptArgs) -> bool),
        entry!(0x005b5760, fn_005b5760(ScriptArgs) -> bool),
        entry!(0x005b57e0, fn_005b57e0(ScriptArgs) -> bool),
        entry!(0x005b5860, fn_005b5860(ScriptArgs) -> bool),
        entry!(0x005b58d0, fn_005b58d0(ScriptArgs) -> bool),
        entry!(
            0x005b5950,
            script_is_action_ref_function(ScriptArgs) -> bool
        ),
        entry!(0x005b59f0, script_activate_function(ScriptArgs) -> bool),
        entry!(0x005b5b90, script_toggle_combat_stats_function() -> bool),
        entry!(0x005b5bb0, script_player_spell_book() -> bool),
        entry!(0x005b5cc0, script_show_who_detects_player() -> bool),
        entry!(0x005b5cf0, script_print_ai_lists() -> bool),
        entry!(0x005b5d20, fn_005b5d20() -> bool),
        entry!(0x005b5d40, fn_005b5d40(ScriptArgs) -> bool),
        entry!(0x005b5da0, fn_005b5da0() -> bool),
        entry!(0x005b5db0, script_freeze_renderer_accumulation() -> bool),
        entry!(0x005b5dd0, script_toggle_occlusion() -> bool),
        entry!(0x005b5e20, script_output_mem_contexts() -> bool),
        entry!(0x005b5e40, fn_005b5e40() -> u32),
        entry!(0x005b5e60, script_output_mem_stats(ScriptArgs) -> bool),
        entry!(
            0x005b6800,
            script_output_texture_use_map(ScriptArgs) -> bool
        ),
        entry!(0x005b6a30, fn_005b6a30(ScriptArgs) -> bool),
        entry!(
            0x005b6b50,
            script_output_archive_profile(ScriptArgs) -> bool
        ),
        entry!(0x005b6c50, fn_005b6c50() -> bool),
        entry!(0x005b6c90, fn_005b6c90() -> bool),
        entry!(0x005b6cb0, fn_005b6cb0(Ptr)),
        entry!(0x005b6cd0, fn_005b6cd0() -> bool),
        entry!(0x005b6d20, fn_005b6d20() -> bool),
        entry!(
            0x005b6d40,
            script_toggle_map_markers_function(ScriptArgs) -> bool
        ),
        entry!(0x005b6f10, script_toggle_verbose_function() -> bool),
        entry!(0x005b6f60, fn_005b6f60(bool)),
        entry!(0x005b6f70, fn_005b6f70() -> bool),
        entry!(0x005b6f80, script_toggle_god_mode_function() -> bool),
        entry!(0x005b7000, script_toggle_demigod_mode_function() -> bool),
        entry!(0x005b7070, script_toggle_scripts_function() -> bool),
        entry!(0x005b70c0, script_toggle_grass_function() -> bool),
        entry!(
            0x005b71b0,
            script_test_all_cells_function(ScriptArgs) -> bool
        ),
        entry!(0x005b7250, fn_005b7250() -> bool),
        entry!(0x005b72a0, fn_005b72a0() -> bool),
        entry!(0x005b72f0, fn_005b72f0() -> bool),
        entry!(0x005b7420, script_toggle_emotions_function() -> bool),
        entry!(0x005b7470, fn_005b7470() -> u8),
        entry!(0x005b7480, fn_005b7480(u8)),
        entry!(0x005b7490, script_reload_current_climate() -> bool),
        entry!(0x005b7520, fn_005b7520() -> bool),
        entry!(0x005b7590, fn_005b7590(ScriptArgs) -> bool),
        entry!(0x005b7660, fn_005b7660(ScriptArgs) -> bool),
        entry!(0x005b76d0, fn_005b76d0() -> bool),
        entry!(
            0x005b76f0,
            script_apply_image_space_modifier(ScriptArgs) -> bool
        ),
        entry!(
            0x005b7760,
            script_remove_image_space_modifier(ScriptArgs) -> bool
        ),
        entry!(0x005b77c0, fn_005b77c0(ScriptArgs) -> bool),
        entry!(0x005b7820, fn_005b7820(ScriptArgs) -> bool),
        entry!(0x005b78a0, fn_005b78a0() -> bool),
        entry!(0x005b7b40, script_test_local_map(ScriptArgs) -> bool),
        entry!(0x005b8fc0, fn_005b8fc0(Ptr, u32)),
        entry!(0x005b9070, script_test_code(ScriptArgs) -> bool),
        entry!(
            0x005b9100,
            script_toggle_conversation_stats_function() -> bool
        ),
        entry!(0x005b9150, fn_005b9150(u8)),
        entry!(0x005b9160, fn_005b9160() -> u8),
        entry!(0x005b9170, script_toggle_full_help() -> bool),
        entry!(0x005b91b0, script_toggle_magic_stats_function() -> bool),
        entry!(0x005b9200, fn_005b9200(u8)),
        entry!(0x005b9210, fn_005b9210() -> u8),
        entry!(0x005b9220, fn_005b9220() -> bool),
        entry!(0x005b9240, fn_005b9240() -> bool),
        entry!(
            0x005b9260,
            script_toggle_char_controller_shape_function(ScriptArgs) -> bool
        ),
        entry!(0x005b92e0, fn_005b92e0(Ptr) -> u32),
        entry!(0x005b9300, fn_005b9300(ScriptArgs) -> bool),
        entry!(0x005b9370, script_toggle_borders_function() -> bool),
        entry!(
            0x005b9400,
            script_toggle_projectile_debug_function() -> bool
        ),
        entry!(0x005b9470, fn_005b9470()),
        entry!(0x005b9490, script_toggle_menus_function() -> bool),
        entry!(0x005b9500, fn_005b9500() -> bool),
        entry!(0x005b9520, fn_005b9520() -> bool),
        entry!(0x005b9540, script_toggle_sky_function() -> bool),
        entry!(
            0x005b9640,
            script_toggle_collision_function(ScriptArgs) -> bool
        ),
        entry!(0x005b9720, script_toggle_leaves_function() -> bool),
        entry!(0x005b9790, bs_tree_manager_are_leaves_visible() -> bool),
        entry!(0x005b97d0, fn_005b97d0(u8)),
        entry!(0x005b97f0, script_toggle_wireframe_function() -> bool),
        entry!(0x005b9870, fn_005b9870(Ptr)),
        entry!(
            0x005b98a0,
            script_toggle_detection_stats(ScriptArgs) -> bool
        ),
        entry!(
            0x005b9950,
            script_set_target_refraction(ScriptArgs) -> bool
        ),
        entry!(0x005b9b00, fn_005b9b00() -> bool),
        entry!(
            0x005b9b40,
            script_set_target_refraction_fire(ScriptArgs) -> bool
        ),
        entry!(0x005b9c90, script_set_target_falloff(ScriptArgs) -> bool),
        entry!(0x005b9e80, fn_005b9e80(f32) -> f32),
        entry!(0x005b9ea0, script_set_target_dof() -> bool),
        entry!(0x005b9ed0, fn_005b9ed0() -> bool),
        entry!(0x005b9ef0, fn_005b9ef0(ScriptArgs) -> bool),
        entry!(0x005b9f50, fn_005b9f50(Ptr, f32)),
        entry!(0x005b9f80, fn_005b9f80(Ptr, f32)),
        entry!(0x005b9fa0, fn_005b9fa0(ScriptArgs) -> bool),
        entry!(0x005b9fe0, script_toggle_foot_ik(ScriptArgs) -> bool),
        entry!(0x005ba110, fn_005ba110(u8)),
        entry!(0x005ba130, fn_005ba130(Ptr, u8)),
        entry!(0x005ba180, fn_005ba180(Ptr) -> u8),
        entry!(0x005ba1a0, script_toggle_grab_ik(ScriptArgs) -> bool),
        entry!(0x005ba2d0, fn_005ba2d0() -> u8),
        entry!(0x005ba2f0, fn_005ba2f0(u8)),
        entry!(0x005ba310, fn_005ba310(Ptr, u8)),
        entry!(0x005ba360, fn_005ba360(Ptr) -> u8),
        entry!(0x005ba380, script_toggle_look_ik(ScriptArgs) -> bool),
        entry!(0x005ba4b0, fn_005ba4b0() -> u8),
        entry!(0x005ba4d0, fn_005ba4d0(u8)),
        entry!(0x005ba4f0, fn_005ba4f0(Ptr, u8)),
        entry!(0x005ba540, fn_005ba540(Ptr) -> u8),
        entry!(0x005ba560, script_toggle_ragdoll_anim() -> bool),
        entry!(0x005ba5d0, script_set_vel_function(ScriptArgs) -> bool),
        entry!(0x005ba690, fn_005ba690() -> bool),
        entry!(0x005ba6b0, fn_005ba6b0()),
        entry!(
            0x005ba6d0,
            script_toggle_lite_brite_function() -> bool
        ),
        entry!(0x005ba740, fn_005ba740() -> u8),
        entry!(0x005ba750, fn_005ba750(u8)),
        entry!(0x005ba760, script_toggle_debug_text_function() -> bool),
        entry!(0x005ba7c0, fn_005ba7c0(ScriptArgs) -> bool),
        entry!(0x005ba8d0, fn_005ba8d0() -> u32),
        entry!(0x005ba8f0, fn_005ba8f0(ScriptArgs) -> bool),
        entry!(0x005bac20, fn_005bac20() -> bool),
        entry!(0x005bac50, fn_005bac50() -> u32),
        entry!(0x005bac60, fn_005bac60() -> u32),
        entry!(0x005bac70, fn_005bac70() -> u32),
        entry!(0x005bac80, fn_005bac80(u32)),
        entry!(0x005bac90, fn_005bac90(ScriptArgs) -> bool),
        entry!(
            0x005bade0,
            script_show_viewer_strings_function(ScriptArgs) -> bool
        ),
        entry!(
            0x005bb030,
            script_toggle_lod_land_function(ScriptArgs) -> bool
        ),
        entry!(
            0x005bb390,
            script_set_lod_object_distance_function(ScriptArgs) -> bool
        ),
        entry!(0x005bb4d0, fn_005bb4d0(Ptr) -> u8),
        entry!(0x005bb4f0, fn_005bb4f0(ScriptArgs) -> bool),
        entry!(0x005bb570, fn_005bb570(ScriptArgs) -> bool),
        entry!(0x005bb610, fn_005bb610(ScriptArgs) -> bool),
        entry!(0x005bb6c0, fn_005bb6c0(ScriptArgs) -> bool),
        entry!(0x005bb770, fn_005bb770(ScriptArgs) -> bool),
        entry!(
            0x005bb800,
            script_move_pipboy_knob_function(ScriptArgs) -> bool
        ),
        entry!(0x005bb870, fn_005bb870(ScriptArgs) -> bool),
        entry!(0x005bb8c0, fn_005bb8c0(ScriptArgs) -> bool),
        entry!(0x005bb9b0, fn_005bb9b0(Ptr, Ptr) -> u32),
        entry!(0x005bb9e0, fn_005bb9e0(ScriptArgs) -> bool),
        entry!(0x005bbc10, fn_005bbc10(Ptr)),
        entry!(0x005bbc30, fn_005bbc30() -> bool),
        entry!(0x005bbc50, script_set_global_radial_blur(ScriptArgs) -> bool),
        entry!(0x005bbd60, fn_005bbd60(Ptr, u8)),
        entry!(0x005bbda0, script_toggle_ai_detection() -> bool),
        entry!(0x005bbe10, fn_005bbe10(ScriptArgs) -> bool),
        entry!(0x005bbec0, fn_005bbec0(ScriptArgs) -> bool),
        entry!(0x005bbf70, script_toggle_ai(ScriptArgs) -> bool),
        entry!(0x005bc0d0, script_toggle_combat_ai() -> bool),
        entry!(0x005bc130, fn_005bc130() -> bool),
        entry!(0x005bc150, fn_005bc150(ScriptArgs) -> bool),
        entry!(0x005bc1a0, fn_005bc1a0(ScriptArgs) -> bool),
        entry!(0x005bc200, fn_005bc200(ScriptArgs) -> bool),
        entry!(0x005bc260, fn_005bc260(ScriptArgs) -> bool),
        entry!(
            0x005bc310,
            script_show_head_track_target(ScriptArgs) -> bool
        ),
        entry!(0x005bc3b0, script_toggle_high_process() -> bool),
        entry!(0x005d_b870, fn_005db870(ScriptArgs) -> bool),
        entry!(0x005d_f040, fn_005df040(ScriptArgs) -> bool),
        entry!(0x005d_f5d0, fn_005df5d0(ScriptArgs) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fake virtual functions the tests put into vtables.
    const V_TRUE: u32 = 0x0900_0001;
    const V_FALSE: u32 = 0x0900_0002;
    const V_NINE: u32 = 0x0900_0003;
    const V_RECORD: u32 = 0x0900_0004;
    /// `V_KIND + k` returns `k`.
    const V_KIND: u32 = 0x0900_0010;

    /// An engine with the pages of the globals these commands touch mapped
    /// and the small accessors every command uses replaced by doubles that
    /// behave like the exe's code (see the constants' documentation).
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0101_6000,
            0x0102_3000,
            0x0103_9000,
            0x011c_3000,
            0x011d_d000,
            0x011d_e000,
            0x011e_0000,
            0x011f_1000,
            0x011f_6000,
            0x011f_9000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(V_TRUE, |_, _| true.into_ret());
        e.register(V_FALSE, |_, _| false.into_ret());
        e.register(V_NINE, |_, _| 9u32.into_ret());
        e.register(V_RECORD, |_, _| Ret::default());
        for k in 0..4u32 {
            e.register_double(V_KIND + k, move |_, _| k.into_ret());
        }
        e.register(FORM_TYPE, |e, a| (e.mem.u8(a[0] + 4) as u32).into_ret());
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(EXTRA_DATA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        list_doubles(&mut e);
        e
    }

    /// Doubles for the `BSSimpleList` node functions, behaving like the
    /// exe's (item at +0, next at +4).
    fn list_doubles(e: &mut Engine) {
        e.register(LIST_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_ITEM_PTR, |_, a| a[0].into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_POP_FRONT, |e, a| {
            let head = a[0];
            let next = e.mem.u32(head + 4);
            if next != 0 {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(head, item);
                e.mem.set_u32(head + 4, after);
            } else {
                e.mem.set_u32(head, 0);
            }
            Ret::default()
        });
        e.register(LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(LIST_APPEND, |e, a| {
            let item = e.mem.u32(a[1]);
            if item != 0 {
                let head = a[0];
                if e.mem.u32(head) == 0 {
                    e.mem.set_u32(head, item);
                } else {
                    let mut tail = head;
                    while e.mem.u32(tail + 4) != 0 {
                        tail = e.mem.u32(tail + 4);
                    }
                    let node = e.mem.alloc(8);
                    e.mem.set_u32(node, item);
                    e.mem.set_u32(tail + 4, node);
                }
            }
            Ret::default()
        });
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.register(LIST_DELETE, |_, _| Ret::default());
        e.register(LIST_COUNT, |e, a| {
            let mut n = 0u32;
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) != 0 {
                    n += 1;
                }
                node = e.mem.u32(node + 4);
            }
            n.into_ret()
        });
    }

    /// Nodes holding `items`; returns the first node (0 for none).
    fn list_of(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    /// An object whose vtable (in the heap) has the given `(byte offset,
    /// function)` slots.
    fn object_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x400);
        for (offset, function) in slots {
            e.mem.set_u32(vtable + offset, *function);
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The standard eight words with `this_obj` set.
    fn script(this_obj: u32) -> ScriptArgs {
        ScriptArgs {
            param_info: 1,
            script_data: 2,
            this_obj: Ptr::new(this_obj),
            containing_obj: Ptr::NULL,
            script_obj: Ptr::new(5),
            event_list: 6,
            result: Ptr::NULL,
            opcode_offset: 8,
        }
    }

    /// `ParseParameters` double: returns `ok` and stores `outs` through its
    /// output pointers (after the seven fixed words).
    fn parse_gives(e: &mut Engine, ok: bool, outs: &[u32]) {
        let outs = outs.to_vec();
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            for (i, value) in outs.iter().enumerate() {
                e.mem.set_u32(a[7 + i], *value);
            }
            ok.into_ret()
        });
    }

    fn start_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
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

    /// The console prints, as argument words.
    fn printed(e: &Engine) -> Vec<Vec<u32>> {
        calls(e, CONSOLE_PRINT)
    }

    /// Registers doubles that just accept the call.
    fn accept(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn script_args_round_trip_through_the_uniform_form() {
        let a = ScriptArgs {
            result: Ptr::new(0x77),
            containing_obj: Ptr::new(0x66),
            ..script(0x55)
        };
        let words = args![a];
        assert_eq!(words, vec![1, 2, 0x55, 0x66, 5, 6, 0x77, 8]);
        let back = ScriptArgs::take(&words, &mut 0);
        assert_eq!(back.this_obj, Ptr::new(0x55));
        assert_eq!(back.result, Ptr::new(0x77));
        assert_eq!(back.opcode_offset, 8);
    }

    #[test]
    fn get_distance_passes_the_parsed_object_to_the_condition_function() {
        let mut e = engine();
        parse_gives(&mut e, true, &[0x77]);
        e.register(GET_DISTANCE_CONDITION, |_, _| true.into_ret());
        start_log(&mut e);
        let a = ScriptArgs {
            result: Ptr::new(0x1234),
            ..script(0x40)
        };
        assert!(e.call(0x005b_4b20, &args![a]).bool());
        // ParseParameters gets: info, data, opcode offset, thisObj, containing,
        // script, event list, then the address of the local.
        assert_eq!(
            calls(&e, PARSE_PARAMETERS)[0][..7],
            [1, 2, 8, 0x40, 0, 5, 6]
        );
        assert_eq!(
            calls(&e, GET_DISTANCE_CONDITION),
            vec![vec![0x40, 0x77, 0, 0x1234]]
        );
        // Parameters that do not parse: false, nothing called.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_4b20, &args![a]).bool());
        assert!(calls(&e, GET_DISTANCE_CONDITION).is_empty());
    }

    #[test]
    fn get_in_zone_passes_the_parsed_object_to_the_condition_function() {
        let mut e = engine();
        parse_gives(&mut e, true, &[0x78]);
        e.register(GET_IN_ZONE_CONDITION, |_, _| false.into_ret());
        start_log(&mut e);
        let a = ScriptArgs {
            result: Ptr::new(0x1234),
            ..script(0x41)
        };
        assert!(!e.call(0x005b_4b80, &args![a]).bool());
        assert_eq!(
            calls(&e, GET_IN_ZONE_CONDITION),
            vec![vec![0x41, 0x78, 0, 0x1234]]
        );
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_4b80, &args![a]).bool());
        assert!(calls(&e, GET_IN_ZONE_CONDITION).is_empty());
    }

    /// Doubles for the callees of `005b4be0`.
    fn add_item_engine() -> Engine {
        let mut e = engine();
        e.register(GET_CALC_LEVEL, |_, _| 0x0001_0007u32.into_ret());
        accept(
            &mut e,
            &[
                TES_CONTAINER_CONSTRUCT,
                TES_CONTAINER_DESTRUCT,
                FN_00487F70,
                FN_004818E0,
                FN_00482090,
                FN_004821A0,
                FN_00606540,
            ],
        );
        e.register(FN_00446390, |_, _| 5u32.into_ret());
        e.set_global(ADD_ITEM_FLOAT, 3.5f32);
        e
    }

    #[test]
    fn add_item_rolls_a_leveled_list_at_the_references_level() {
        let mut e = add_item_engine();
        let bound = object_with(&mut e, &[(0xe4, V_TRUE)]);
        e.mem.set_u8(bound + 4, 0x34);
        // An actor (slots 0x100 and 0x218) that is not the player, with
        // base form 0xba5e and slot 0x1e8 answering 9.
        let actor = object_with(&mut e, &[(0x100, V_TRUE), (0x218, V_TRUE), (0x1e8, V_NINE)]);
        e.mem.set_u32(actor + 0x20, 0xba5e);
        e.set_global(PLAYER, 0x4444u32);
        parse_gives(&mut e, true, &[bound, 0x0001_0003, 1]);
        start_log(&mut e);
        assert!(e.call(0x005b_4be0, &args![script(actor)]).bool());
        let container = calls(&e, TES_CONTAINER_CONSTRUCT)[0][0];
        // Level and count are the low 16 bits; the roll happens at +0x30.
        assert_eq!(
            calls(&e, FN_00487F70),
            vec![vec![bound + 0x30, 7, 3, container, 0]]
        );
        assert!(calls(&e, FN_004818E0).is_empty());
        assert_eq!(calls(&e, FN_004821A0), vec![vec![container, actor, 1]]);
        assert_eq!(calls(&e, FN_00606540), vec![vec![0xba5e, actor, 9, 1]]);
        assert_eq!(calls(&e, TES_CONTAINER_DESTRUCT), vec![vec![container]]);
    }

    #[test]
    fn add_item_adds_a_single_item_and_the_player_gets_no_equip_call() {
        let mut e = add_item_engine();
        let item = object_with(&mut e, &[(0xe4, V_TRUE)]);
        e.mem.set_u8(item + 4, 0x28);
        let player = object_with(&mut e, &[(0x100, V_TRUE), (0x218, V_TRUE), (0x1e8, V_NINE)]);
        e.set_global(PLAYER, player);
        parse_gives(&mut e, true, &[item, 4, 0]);
        start_log(&mut e);
        assert!(e.call(0x005b_4be0, &args![script(player)]).bool());
        let container = calls(&e, TES_CONTAINER_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, FN_004818E0), vec![vec![container, item, 4, 0]]);
        // Type 0x28 with a small signed byte at +0xf4 calls 00482090.
        assert_eq!(
            calls(&e, FN_00482090),
            vec![vec![container, 3.5f32.to_bits()]]
        );
        // The flag was 0: the container is handed over with `false`.
        assert_eq!(calls(&e, FN_004821A0), vec![vec![container, player, 0]]);
        assert!(calls(&e, FN_00606540).is_empty());
    }

    #[test]
    fn add_item_walks_a_form_list_and_reports_failure() {
        let mut e = add_item_engine();
        let good = object_with(&mut e, &[(0xe4, V_TRUE)]);
        let bad = object_with(&mut e, &[(0xe4, V_FALSE)]);
        let list = object_with(&mut e, &[(0xe4, V_FALSE)]);
        e.mem.set_u8(list + 4, 0x55);
        let nodes = list_of(&mut e, &[bad, good]);
        e.register_double(FORM_LIST_ITEMS, move |_, a| {
            assert_eq!(a[0], list);
            nodes.into_ret()
        });
        let this_obj = object_with(&mut e, &[(0x100, V_FALSE)]);
        parse_gives(&mut e, true, &[list, 2, 0]);
        start_log(&mut e);
        assert!(e.call(0x005b_4be0, &args![script(this_obj)]).bool());
        let container = calls(&e, TES_CONTAINER_CONSTRUCT)[0][0];
        // Only the element that passes slot 0xe4 is added.
        assert_eq!(calls(&e, FN_004818E0), vec![vec![container, good, 2, 0]]);

        // A plain form that does not pass slot 0xe4: nothing is added and the
        // script's name (slot 0x130) is fetched for the failure message.
        let script_obj = object_with(&mut e, &[(0x130, V_RECORD)]);
        e.mem.set_u8(bad + 4, 0x2a);
        parse_gives(&mut e, true, &[bad, 1, 0]);
        start_log(&mut e);
        let a = ScriptArgs {
            script_obj: Ptr::new(script_obj),
            ..script(this_obj)
        };
        assert!(e.call(0x005b_4be0, &args![a]).bool());
        assert!(calls(&e, FN_004818E0).is_empty());
        assert_eq!(calls(&e, V_RECORD), vec![vec![script_obj]]);

        // No reference: nothing happens; bad parameters: false.
        start_log(&mut e);
        assert!(e.call(0x005b_4be0, &args![script(0)]).bool());
        assert!(calls(&e, TES_CONTAINER_CONSTRUCT).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_4be0, &args![script(this_obj)]).bool());
    }

    /// Doubles for the callees of `005b4e90`. By convention of these tests,
    /// byte `+0x50` of a reference marks an actor (the RTTI cast succeeds),
    /// `+0x58` is the `double` the actor carries of every item, and, on an
    /// item, byte `+0x51` says the actor can wear it and `+0x52` that a worn
    /// entry exists.
    fn remove_item_engine() -> Engine {
        let mut e = engine();
        e.register(DYNAMIC_CAST, |e, a| {
            let actor = if e.mem.u8(a[0] + 0x50) != 0 { a[0] } else { 0 };
            actor.into_ret()
        });
        e.register(GET_ITEM_COUNT_CONDITION, |e, a| {
            let carried = e.mem.f64(a[0] + 0x58);
            e.mem.set_f64(a[3], carried);
            Ret::default()
        });
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32 as u32).into_ret());
        e.register(FN_00575400, |e, a| (e.mem.u8(a[1] + 0x51) != 0).into_ret());
        e.register(GET_INVENTORY_CHANGES, |_, _| 0x5150u32.into_ret());
        e.register(WEARING_OBJECT, |e, a| {
            let worn = if e.mem.u8(a[1] + 0x52) != 0 {
                0x7777
            } else {
                0
            };
            (worn as u32).into_ret()
        });
        e.register(GET_CONTAINER_CHANGES, |_, _| 0xc0deu32.into_ret());
        e.register(FN_004CBA70, |_, _| 0x77u32.into_ret());
        e.register(BS_STRING_TEXT, |_, a| (a[0] + 1).into_ret());
        e.register(GET_FULL_NAME, |_, _| 0xaaaau32.into_ret());
        e.register(BS_STRING_FORMAT, |e, a| {
            e.mem.set_u32(a[0], 0x5151);
            Ret::default()
        });
        e.register(GET_PICK_UP_SOUND_NAME, |_, _| 0x50du32.into_ret());
        e.set_global(PICKUP_VOLUME, 2.0f32);
        accept(
            &mut e,
            &[
                SET_CAN_NOT_WEAR,
                FN_008248E0,
                FN_00704AF0,
                BS_STRING_CONSTRUCT,
                BS_STRING_DESTRUCT,
                SHOW_MESSAGE,
            ],
        );
        e
    }

    #[test]
    fn remove_item_from_a_plain_reference_removes_the_form_as_is() {
        let mut e = remove_item_engine();
        let item = object_with(&mut e, &[(0xe4, V_TRUE)]);
        let this_obj = object_with(&mut e, &[(0x17c, V_RECORD)]);
        parse_gives(&mut e, true, &[item, 3, 0]);
        start_log(&mut e);
        assert!(e.call(0x005b_4e90, &args![script(this_obj)]).bool());
        assert_eq!(
            calls(&e, V_RECORD),
            vec![vec![this_obj, item, 0, 3, 0, 0, 0, 0, 0, 1, 0]]
        );
        assert_eq!(calls(&e, LIST_DESTRUCT).len(), 1);
        // No reference or no form: nothing; bad parameters: false.
        start_log(&mut e);
        assert!(e.call(0x005b_4e90, &args![script(0)]).bool());
        parse_gives(&mut e, true, &[0, 3, 0]);
        assert!(e.call(0x005b_4e90, &args![script(this_obj)]).bool());
        assert!(calls(&e, LIST_CONSTRUCT).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_4e90, &args![script(this_obj)]).bool());
    }

    #[test]
    fn remove_item_from_the_player_caps_the_count_and_tells_the_player() {
        let mut e = remove_item_engine();
        let item = object_with(&mut e, &[(0xe4, V_TRUE)]);
        e.mem.set_u8(item + 4, 0x18);
        e.mem.set_u8(item + 0x51, 1);
        e.mem.set_u8(item + 0x52, 1);
        let player = object_with(&mut e, &[(0x17c, V_RECORD)]);
        e.mem.set_u8(player + 0x50, 1);
        e.mem.set_f64(player + 0x58, 2.0);
        e.set_global(PLAYER, player);
        parse_gives(&mut e, true, &[item, 5, 0]);
        start_log(&mut e);
        assert!(e.call(0x005b_4e90, &args![script(player)]).bool());
        // The player carries 2 of the 5 asked for.
        assert_eq!(calls(&e, FTOL).len(), 1);
        assert_eq!(calls(&e, SET_CAN_NOT_WEAR), vec![vec![0x7777, 0]]);
        assert_eq!(calls(&e, FN_008248E0), vec![vec![player + 0x94, item, 1]]);
        assert_eq!(
            calls(&e, V_RECORD),
            vec![vec![player, item, 0x7777, 2, 0, 0, 0, 0, 0, 1, 0]]
        );
        let text = calls(&e, BS_STRING_CONSTRUCT)[0][0];
        // "<count> <name><b> <a>" for several items.
        assert_eq!(
            calls(&e, BS_STRING_FORMAT),
            vec![vec![
                text,
                FORMAT_ITEM_COUNT_MESSAGE,
                2,
                0xaaaa,
                MESSAGE_PART_B + 1,
                MESSAGE_PART_A + 1
            ]]
        );
        // Form type 0x18 shows the message with the icon and the pick-up
        // sound.
        assert_eq!(
            calls(&e, SHOW_MESSAGE),
            vec![vec![0x5151, 0, ICON_VAULT_BOY, 0x50d, 2.0f32.to_bits(), 0]]
        );
        assert_eq!(calls(&e, FN_00704AF0).len(), 1);

        // Silenced: no message, the interface refresh still happens.
        parse_gives(&mut e, true, &[item, 1, 1]);
        e.mem.set_f64(player + 0x58, 5.0);
        start_log(&mut e);
        assert!(e.call(0x005b_4e90, &args![script(player)]).bool());
        assert!(calls(&e, BS_STRING_FORMAT).is_empty());
        assert_eq!(calls(&e, FN_00704AF0).len(), 1);

        // One item: "<name> <a>".
        parse_gives(&mut e, true, &[item, 1, 0]);
        start_log(&mut e);
        assert!(e.call(0x005b_4e90, &args![script(player)]).bool());
        let text = calls(&e, BS_STRING_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, BS_STRING_FORMAT),
            vec![vec![text, FORMAT_ITEM_MESSAGE, 0xaaaa, MESSAGE_PART_A + 1]]
        );

        // A type the switch does not know: the message is built but not shown.
        e.mem.set_u8(item + 4, 0x2a);
        start_log(&mut e);
        assert!(e.call(0x005b_4e90, &args![script(player)]).bool());
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
    }

    #[test]
    fn remove_item_skips_entries_the_actor_does_not_carry() {
        let mut e = remove_item_engine();
        let item = object_with(&mut e, &[(0xe4, V_TRUE)]);
        let other = object_with(&mut e, &[(0x17c, V_RECORD)]);
        e.mem.set_u8(other + 0x50, 1);
        e.mem.set_f64(other + 0x58, 0.0);
        parse_gives(&mut e, true, &[item, 3, 0]);
        start_log(&mut e);
        assert!(e.call(0x005b_4e90, &args![script(other)]).bool());
        // Carrying none: the count becomes 0 and nothing is removed.
        assert!(calls(&e, V_RECORD).is_empty());
    }

    #[test]
    fn remove_item_from_another_actor_lets_a_wearer_drop_the_acquire_object() {
        let mut e = remove_item_engine();
        let item = object_with(&mut e, &[(0xe4, V_TRUE)]);
        e.mem.set_u8(item + 0x51, 1);
        e.set_global(PLAYER, 0x4444u32);
        let process = object_with(&mut e, &[(0xc4, V_RECORD)]);
        e.register_double(MIDDLE_HIGH_PROCESS_SAVED_ACQUIRE, move |_, _| {
            process.into_ret()
        });
        let actor = object_with(&mut e, &[(0x17c, V_FALSE), (0x1d0, V_NINE)]);
        e.mem.set_u8(actor + 0x50, 1);
        e.mem.set_f64(actor + 0x58, 10.0);
        parse_gives(&mut e, true, &[item, 3, 0]);
        start_log(&mut e);
        assert!(e.call(0x005b_4e90, &args![script(actor)]).bool());
        assert_eq!(calls(&e, V_RECORD), vec![vec![process, actor, 0]]);
        assert!(calls(&e, FN_00704AF0).is_empty());
    }

    #[test]
    fn remove_item_resolves_leveled_entries_and_form_lists() {
        let mut e = remove_item_engine();
        let this_obj = object_with(&mut e, &[(0x17c, V_RECORD)]);
        // Type 0x34: the entry comes from the inventory changes (0x77).
        let leveled = object_with(&mut e, &[(0xe4, V_FALSE)]);
        e.mem.set_u8(leveled + 4, 0x34);
        parse_gives(&mut e, true, &[leveled, 2, 0]);
        start_log(&mut e);
        assert!(e.call(0x005b_4e90, &args![script(this_obj)]).bool());
        assert_eq!(calls(&e, FN_004CBA70), vec![vec![0xc0de, leveled]]);
        assert_eq!(
            calls(&e, V_RECORD),
            vec![vec![this_obj, 0x77, 0, 2, 0, 0, 0, 0, 0, 1, 0]]
        );
        // Type 0x55: every element passing slot 0xe4 is removed, in order.
        let a = object_with(&mut e, &[(0xe4, V_TRUE)]);
        let b = object_with(&mut e, &[(0xe4, V_FALSE)]);
        let c = object_with(&mut e, &[(0xe4, V_TRUE)]);
        let nodes = list_of(&mut e, &[a, b, c]);
        e.register_double(FORM_LIST_ITEMS, move |_, _| nodes.into_ret());
        let list = object_with(&mut e, &[(0xe4, V_FALSE)]);
        e.mem.set_u8(list + 4, 0x55);
        parse_gives(&mut e, true, &[list, 1, 0]);
        start_log(&mut e);
        assert!(e.call(0x005b_4e90, &args![script(this_obj)]).bool());
        let removed: Vec<u32> = calls(&e, V_RECORD).iter().map(|w| w[1]).collect();
        assert_eq!(removed, vec![a, c]);
    }

    #[test]
    fn transfer_gives_the_target_the_base_form_and_the_worn_entry() {
        let mut e = engine();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(FN_00575400, |_, _| true.into_ret());
        e.register(GET_INVENTORY_CHANGES, |_, _| 0x5150u32.into_ret());
        e.register(WEARING_OBJECT, |_, _| 0x7777u32.into_ret());
        accept(&mut e, &[CHECK_QUEST_TARGET_UPDATE, FN_00704AF0]);
        let this_obj = object_with(&mut e, &[]);
        e.mem.set_u32(this_obj + 0x20, 0xba5e);
        let target = object_with(&mut e, &[(0x17c, V_RECORD)]);
        e.set_global(PLAYER, target);
        parse_gives(&mut e, true, &[9]);
        start_log(&mut e);
        let a = ScriptArgs {
            containing_obj: Ptr::new(target),
            ..script(this_obj)
        };
        assert!(!e.call(0x005b_53d0, &args![a]).bool());
        assert_eq!(calls(&e, WEARING_OBJECT), vec![vec![0x5150, 0xba5e, 0]]);
        assert_eq!(
            calls(&e, V_RECORD),
            vec![vec![target, 0xba5e, 0x7777, 1, 0, 0, 9, 0, 0, 1, 0]]
        );
        assert_eq!(
            calls(&e, CHECK_QUEST_TARGET_UPDATE),
            vec![vec![target, this_obj]]
        );
        // The target is the player: the interface is refreshed.
        assert_eq!(calls(&e, FN_00704AF0).len(), 1);

        // Not an actor, not the player: no worn entry, no refresh.
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.set_global(PLAYER, 0x4444u32);
        start_log(&mut e);
        assert!(!e.call(0x005b_53d0, &args![a]).bool());
        assert!(calls(&e, WEARING_OBJECT).is_empty());
        assert_eq!(calls(&e, V_RECORD)[0][2], 0);
        assert!(calls(&e, FN_00704AF0).is_empty());

        // Either reference missing: true and nothing else.
        start_log(&mut e);
        assert!(e.call(0x005b_53d0, &args![script(this_obj)]).bool());
        assert!(calls(&e, V_RECORD).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_53d0, &args![a]).bool());
    }

    #[test]
    fn duplicate_all_items_needs_the_inventory_changes() {
        let mut e = engine();
        e.register(GET_CONTAINER_CHANGES, |_, a| (a[0] + 1).into_ret());
        accept(&mut e, &[DUPLICATE_ALL_ITEMS]);
        parse_gives(&mut e, true, &[0x99, 1]);
        start_log(&mut e);
        assert!(e.call(0x005b_5500, &args![script(0x1000)]).bool());
        // The extra data list is at +0x44; the changes come from it.
        assert_eq!(
            calls(&e, DUPLICATE_ALL_ITEMS),
            vec![vec![0x1045, 0x1000, 0x99]]
        );
        // No changes: nothing is duplicated.
        e.register(GET_CONTAINER_CHANGES, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_5500, &args![script(0x1000)]).bool());
        assert!(calls(&e, DUPLICATE_ALL_ITEMS).is_empty());
        // No reference: true; bad parameters: false.
        assert!(e.call(0x005b_5500, &args![script(0)]).bool());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_5500, &args![script(0x1000)]).bool());
    }

    #[test]
    fn inventory_changes_commands_pass_their_arguments_and_defaults() {
        let mut e = engine();
        e.register(GET_CONTAINER_CHANGES, |_, a| (a[0] + 1).into_ret());
        accept(
            &mut e,
            &[FN_004CE340, FN_004CE380, CHECK_QUEST_TARGET_UPDATE],
        );
        e.set_global(PLAYER, 0x4444u32);
        parse_gives(&mut e, true, &[7, 1, 2, 3, 4]);
        start_log(&mut e);
        assert!(e.call(0x005b_55a0, &args![script(0x1000)]).bool());
        assert_eq!(
            calls(&e, FN_004CE340),
            vec![vec![0x1045, 0x1000, 7, 0, 1, 0, 1, 3, 4]]
        );
        assert_eq!(
            calls(&e, CHECK_QUEST_TARGET_UPDATE),
            vec![vec![0x4444, 0x1000]]
        );
        // Nothing parsed into the locals: their initial values (slot -1).
        parse_gives(&mut e, true, &[]);
        start_log(&mut e);
        assert!(e.call(0x005b_55a0, &args![script(0x1000)]).bool());
        assert_eq!(
            calls(&e, FN_004CE340),
            vec![vec![0x1045, 0x1000, 0, 0, 0, 0, 0, 0xffff_ffff, 0]]
        );

        // The second command has a fixed slot of -1.
        parse_gives(&mut e, true, &[7, 1, 2]);
        start_log(&mut e);
        assert!(e.call(0x005b_5690, &args![script(0x1000)]).bool());
        assert_eq!(
            calls(&e, FN_004CE380),
            vec![vec![0x1045, 0x1000, 7, 0, 1, 0, 1, 0xffff_ffff, 0]]
        );
        assert_eq!(calls(&e, CHECK_QUEST_TARGET_UPDATE).len(), 1);
        // No inventory changes: nothing is called.
        e.register(GET_CONTAINER_CHANGES, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_5690, &args![script(0x1000)]).bool());
        assert!(e.call(0x005b_55a0, &args![script(0x1000)]).bool());
        assert!(calls(&e, FN_004CE380).is_empty());
        assert!(calls(&e, FN_004CE340).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_5690, &args![script(0x1000)]).bool());
        assert!(!e.call(0x005b_55a0, &args![script(0x1000)]).bool());
    }

    #[test]
    fn form_parameter_commands_default_to_the_players_base_form() {
        let mut e = engine();
        let player = e.mem.alloc(0x100);
        e.mem.set_u32(player + 0x20, 0xba5e);
        e.set_global(PLAYER, player);
        accept(&mut e, &[FN_005A2E20, FN_00419700]);
        // 005b5760: the result is cleared first, even when the parse fails.
        let result = e.mem.alloc(8);
        e.mem.set_f64(result, 5.0);
        parse_gives(&mut e, false, &[]);
        let a = ScriptArgs {
            result: Ptr::new(result),
            ..script(0x1000)
        };
        assert!(!e.call(0x005b_5760, &args![a]).bool());
        assert_eq!(e.mem.f64(result), 0.0);
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005b_5760, &args![a]).bool());
        assert_eq!(
            calls(&e, FN_005A2E20),
            vec![vec![0x1000, 0xba5e, 0, result]]
        );
        parse_gives(&mut e, true, &[0x33]);
        start_log(&mut e);
        assert!(e.call(0x005b_5760, &args![a]).bool());
        assert_eq!(calls(&e, FN_005A2E20)[0][1], 0x33);

        // 005b57e0: extra data list (+0x44) gets the form, the reference
        // slot 0x48 gets 0x40.
        let this_obj = object_with(&mut e, &[(0x48, V_RECORD)]);
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005b_57e0, &args![script(this_obj)]).bool());
        assert_eq!(calls(&e, FN_00419700), vec![vec![this_obj + 0x44, 0xba5e]]);
        assert_eq!(calls(&e, V_RECORD), vec![vec![this_obj, 0x40]]);
        // No reference: only the default is looked up.
        start_log(&mut e);
        assert!(e.call(0x005b_57e0, &args![script(0)]).bool());
        assert!(calls(&e, FN_00419700).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_57e0, &args![script(this_obj)]).bool());
    }

    #[test]
    fn give_base_form_to_the_other_reference_refreshes_quest_targets() {
        let mut e = engine();
        accept(&mut e, &[CHECK_QUEST_TARGET_UPDATE, FN_00704AF0]);
        e.register(IS_IN_MENU_MODE, |_, _| true.into_ret());
        e.set_global(PLAYER, 0x4444u32);
        let this_obj = object_with(&mut e, &[]);
        e.mem.set_u32(this_obj + 0x20, 0xba5e);
        let target = object_with(&mut e, &[(0x17c, V_RECORD)]);
        let a = ScriptArgs {
            containing_obj: Ptr::new(target),
            ..script(this_obj)
        };
        start_log(&mut e);
        assert!(!e.call(0x005b_5860, &args![a]).bool());
        assert_eq!(
            calls(&e, V_RECORD),
            vec![vec![target, 0xba5e, 0, 1, 0, 1, 0, 0, 0, 1, 0]]
        );
        assert_eq!(
            calls(&e, CHECK_QUEST_TARGET_UPDATE),
            vec![vec![0x4444, this_obj]]
        );
        assert_eq!(calls(&e, FN_00704AF0).len(), 1);
        // Outside menu mode there is no refresh.
        e.register(IS_IN_MENU_MODE, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(!e.call(0x005b_5860, &args![a]).bool());
        assert!(calls(&e, FN_00704AF0).is_empty());
        // A missing reference: true, nothing done.
        start_log(&mut e);
        assert!(e.call(0x005b_5860, &args![script(this_obj)]).bool());
        assert!(calls(&e, V_RECORD).is_empty());
    }

    #[test]
    fn form_and_count_command_forwards_to_slot_0x17c() {
        let mut e = engine();
        let this_obj = object_with(&mut e, &[(0x17c, V_RECORD)]);
        parse_gives(&mut e, true, &[3, 4]);
        start_log(&mut e);
        assert!(e.call(0x005b_58d0, &args![script(this_obj)]).bool());
        assert_eq!(
            calls(&e, V_RECORD),
            vec![vec![this_obj, 3, 0, 4, 0, 1, 0, 0, 0, 1, 0]]
        );
        start_log(&mut e);
        assert!(e.call(0x005b_58d0, &args![script(0)]).bool());
        assert!(calls(&e, V_RECORD).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_58d0, &args![script(this_obj)]).bool());
    }

    #[test]
    fn is_action_ref_compares_with_the_action_reference_and_echoes() {
        let mut e = engine();
        e.register(GET_ACTION_REF, |_, _| 0x55u32.into_ret());
        let result = e.mem.alloc(8);
        let a = ScriptArgs {
            result: Ptr::new(result),
            ..script(0x1000)
        };
        parse_gives(&mut e, true, &[0x55]);
        start_log(&mut e);
        assert!(e.call(0x005b_5950, &args![a]).bool());
        assert_eq!(e.mem.f64(result), 1.0);
        // The echo flag (TLS +0x268) is off: no print.
        assert!(printed(&e).is_empty());
        parse_gives(&mut e, true, &[0x56]);
        assert!(e.call(0x005b_5950, &args![a]).bool());
        assert_eq!(e.mem.f64(result), 0.0);
        // With the flag set the value is printed as a `double`.
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_ECHO, 1);
        accept(&mut e, &[CONSOLE_PRINT]);
        parse_gives(&mut e, true, &[0x55]);
        start_log(&mut e);
        assert!(e.call(0x005b_5950, &args![a]).bool());
        assert_eq!(printed(&e), vec![args![MSG_IS_ACTION_REF, 1.0f64]]);
        // No reference: the result stays 0.
        assert!(e
            .call(
                0x005b_5950,
                &args![ScriptArgs {
                    this_obj: Ptr::NULL,
                    ..a
                }]
            )
            .bool());
        assert_eq!(e.mem.f64(result), 0.0);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_5950, &args![a]).bool());
    }

    #[test]
    fn activate_runs_with_the_action_flags_and_guards_against_nesting() {
        let mut e = engine();
        accept(&mut e, &[SET_ACTION, CLEAR_ACTION, ACTIVATE]);
        e.register(GET_ACTION_REF, |_, _| 0x55u32.into_ret());
        e.register(FN_00440DA0, |_, _| false.into_ret());
        e.register(HAS_ACTION, |_, _| false.into_ret());
        e.set_global(PLAYER, 0x4444u32);
        let tls = e.tls();
        // No explicit target, second argument 0: the default target, with
        // actions 1 and 2 around the activation.
        parse_gives(&mut e, true, &[0, 0]);
        start_log(&mut e);
        assert!(e.call(0x005b_59f0, &args![script(0x1000)]).bool());
        let order: Vec<(u32, u32)> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| [SET_ACTION, ACTIVATE, CLEAR_ACTION].contains(a))
            .map(|(a, w)| (*a, w[1]))
            .collect();
        assert_eq!(
            order,
            vec![
                (SET_ACTION, 1),
                (SET_ACTION, 2),
                (ACTIVATE, 0x55),
                (CLEAR_ACTION, 2),
                (CLEAR_ACTION, 1)
            ]
        );
        assert_eq!(calls(&e, ACTIVATE), vec![vec![0x1000, 0x55, 0, 0, 1]]);
        // The nesting counter is back to 0.
        assert_eq!(e.mem.u32(tls + TLS_ACTIVATE_DEPTH), 0);

        // With the second argument non-zero: just the activation.
        parse_gives(&mut e, true, &[0x66, 1]);
        start_log(&mut e);
        assert!(e.call(0x005b_59f0, &args![script(0x1000)]).bool());
        assert_eq!(calls(&e, ACTIVATE), vec![vec![0x1000, 0x66, 0, 0, 1]]);
        assert!(calls(&e, SET_ACTION).is_empty());

        // A flagged target is left alone.
        e.register(FN_00440DA0, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_59f0, &args![script(0x1000)]).bool());
        assert!(calls(&e, ACTIVATE).is_empty());
        assert_eq!(e.mem.u32(tls + TLS_ACTIVATE_DEPTH), 0);

        // Nested deeper than the limit: succeeds without doing anything and
        // without touching the counter.
        e.register(FN_00440DA0, |_, _| false.into_ret());
        e.mem.set_u32(tls + TLS_ACTIVATE_DEPTH, 6);
        start_log(&mut e);
        assert!(e.call(0x005b_59f0, &args![script(0x1000)]).bool());
        assert!(calls(&e, ACTIVATE).is_empty());
        assert_eq!(e.mem.u32(tls + TLS_ACTIVATE_DEPTH), 6);
        e.mem.set_u32(tls + TLS_ACTIVATE_DEPTH, 5);
        start_log(&mut e);
        assert!(e.call(0x005b_59f0, &args![script(0x1000)]).bool());
        assert_eq!(calls(&e, ACTIVATE).len(), 1);
        assert_eq!(e.mem.u32(tls + TLS_ACTIVATE_DEPTH), 5);

        // The player may not run it; a missing reference is fine.
        e.set_global(PLAYER, 0x1000u32);
        start_log(&mut e);
        assert!(!e.call(0x005b_59f0, &args![script(0x1000)]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        e.set_global(PLAYER, 0x4444u32);
        assert!(e.call(0x005b_59f0, &args![script(0)]).bool());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_59f0, &args![script(0x1000)]).bool());
    }

    /// Runs the no-argument command at `addr` and returns its console prints.
    fn run_and_print(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        accept(e, &[CONSOLE_PRINT]);
        start_log(e);
        assert!(e.call(addr, &args![]).bool());
        printed(e)
    }

    #[test]
    fn message_only_commands_print_their_message() {
        let mut e = engine();
        for (addr, message) in [
            (0x005b_5b90, MSG_COMBAT_STATS_REMOVED),
            (0x005b_5db0, MSG_FREEZE_RENDERER_DEPRECATED),
            (0x005b_5e20, MSG_MEM_CONTEXTS_UNSUPPORTED),
        ] {
            assert_eq!(run_and_print(&mut e, addr), vec![vec![message]]);
        }
    }

    #[test]
    fn spell_book_counts_the_spells_the_player_takes() {
        let mut e = engine();
        // Entries with the type their +0x18 member reports through slot 0x18
        // (0, 1, 2 and 3), and a null entry.
        let mut entries = vec![];
        for kind in [0, 1, 2, 3, 3] {
            let entry = e.mem.alloc(0x40);
            let vtable = e.mem.alloc(0x40);
            e.mem.set_u32(vtable + 0x18, V_KIND + kind);
            e.mem.set_u32(entry + 0x18, vtable);
            entries.push(entry);
        }
        entries.insert(2, 0);
        let nodes = list_of(&mut e, &entries);
        e.set_global(DATA_HANDLER, 0x5000u32);
        e.register_double(DATA_HANDLER_SPELLS, move |_, a| {
            assert_eq!(a[0], 0x5000);
            nodes.into_ret()
        });
        // The player (slot 0x3e0) takes every spell but the last one.
        let add_spell = 0x0900_0020;
        let refused = entries[5];
        e.register_double(add_spell, move |_, a| (a[1] != refused).into_ret());
        let player = object_with(&mut e, &[(0x3e0, add_spell)]);
        e.set_global(PLAYER, player);
        accept(&mut e, &[CONSOLE_PRINT]);
        start_log(&mut e);
        assert!(e.call(0x005b_5bb0, &args![]).bool());
        // Asked: the kinds 0, 2, 3 and 3 entries; not the kind 1 or the null
        // entry.
        let asked: Vec<u32> = calls(&e, add_spell).iter().map(|w| w[1]).collect();
        assert_eq!(asked, vec![entries[0], entries[3], entries[4], refused]);
        // Three accepted of four asked.
        assert_eq!(printed(&e), vec![vec![MSG_SPELLS_ADDED, 3]]);
    }

    #[test]
    fn process_list_commands() {
        let mut e = engine();
        accept(&mut e, &[PROCESS_LISTS_PRINT_LISTS, FN_00483710]);
        assert_eq!(
            run_and_print(&mut e, 0x005b_5cc0),
            vec![vec![MSG_DETECTION_LIST_PRINTED]]
        );
        assert_eq!(
            calls(&e, PROCESS_LISTS_PRINT_LISTS),
            vec![vec![PROCESS_LISTS, 1, 0xffff_ffff]]
        );
        assert_eq!(
            run_and_print(&mut e, 0x005b_5cf0),
            vec![vec![MSG_AI_LISTS_PRINTED]]
        );
        assert_eq!(
            calls(&e, PROCESS_LISTS_PRINT_LISTS),
            vec![vec![PROCESS_LISTS, 0, 0xffff_ffff]]
        );
        start_log(&mut e);
        assert!(e.call(0x005b_5d20, &args![]).bool());
        assert_eq!(calls(&e, FN_00483710), vec![vec![PROCESS_LISTS]]);
    }

    #[test]
    fn flag_setting_commands() {
        let mut e = engine();
        parse_gives(&mut e, true, &[5]);
        assert!(e.call(0x005b_5d40, &args![script(0)]).bool());
        assert_eq!(e.global::<u8>(FLAG_0011F122D), 1);
        parse_gives(&mut e, true, &[0]);
        assert!(e.call(0x005b_5d40, &args![script(0)]).bool());
        assert_eq!(e.global::<u8>(FLAG_0011F122D), 0);
        // Bad parameters leave the flag alone and fail.
        e.set_global(FLAG_0011F122D, 1u8);
        parse_gives(&mut e, false, &[0]);
        assert!(!e.call(0x005b_5d40, &args![script(0)]).bool());
        assert_eq!(e.global::<u8>(FLAG_0011F122D), 1);

        assert!(e.call(0x005b_5da0, &args![]).bool());
        assert_eq!(e.global::<u8>(FLAG_0011F9FC0), 1);
    }

    #[test]
    fn toggle_occlusion_flips_and_prints_the_state() {
        let mut e = engine();
        assert_eq!(
            run_and_print(&mut e, 0x005b_5dd0),
            vec![vec![MSG_OCCLUSION_QUERY, TEXT_ON]]
        );
        assert_eq!(e.global::<u8>(OCCLUSION_QUERY), 1);
        assert_eq!(
            run_and_print(&mut e, 0x005b_5dd0),
            vec![vec![MSG_OCCLUSION_QUERY, TEXT_OFF]]
        );
        assert_eq!(e.global::<u8>(OCCLUSION_QUERY), 0);
    }

    #[test]
    fn the_logging_stub_returns_zero() {
        let mut e = engine();
        assert_eq!(e.call(0x005b_5e40, &args![1u32, 2u32]).u32(), 0);
    }

    /// Writes the C string `text` at `addr` (a page the engine mapped).
    fn put_str(e: &mut Engine, addr: u32, text: &str) {
        e.mem.set_cstr(addr, text.as_bytes());
    }

    /// Doubles for the string and file functions `OutputMemStats` and its
    /// relatives call: the C library ones behave like the real ones, the
    /// formatting ones only record their arguments.
    fn output_engine() -> Engine {
        let mut e = engine();
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRNCPY_S, |e, a| {
            let text = e.mem.cstr(a[1]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRCAT_S, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(SPRINTF_S, |_, _| Ret::default());
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(FIND_SUBSTRING, |e, a| {
            let haystack = e.mem.cstr(a[0]);
            let needle = e.mem.cstr(a[1]);
            let found = !needle.is_empty()
                && haystack
                    .windows(needle.len())
                    .any(|window| window == needle.as_slice());
            (found as u32).into_ret()
        });
        accept(
            &mut e,
            &[
                MEM_STATS_FILE_CONSTRUCT,
                MEM_STATS_FILE_DESTRUCT,
                MEM_STATS_OUTPUT_SET,
                BS_FILE_CONSTRUCT,
                BS_FILE_OPEN,
                BS_FILE_CLOSE,
                BS_FILE_DESTRUCT,
                BS_FILE_WRITE,
                MODEL_LOADER_OUTPUT_MODEL_MAP,
                FACE_GEN_OUTPUT_MODEL_MAP,
                VECTOR_CONSTRUCT,
                VECTOR_DESTRUCT,
                ZERO_FILL,
                FIX_PATH,
                CONSOLE_PRINT,
            ],
        );
        e.register(MEM_STATS_OUTPUT_GET, |_, _| 0x11f_6238u32.into_ret());
        e.register(GET_MODEL_CACHE, |_, _| 0u32.into_ret());
        e.register(FIRST_TEXTURE, |_, _| 0u32.into_ret());
        e
    }

    #[test]
    fn mem_stats_names_its_files_after_the_argument_and_dumps_the_model_maps() {
        let mut e = output_engine();
        // The argument is parsed into the path buffer (first output).
        e.register_double(PARSE_PARAMETERS, |e, a| {
            e.mem.set_cstr(a[7], b"Run1");
            true.into_ret()
        });
        e.set_global(MODEL_LOADER, 0x6000u32);
        e.register(GET_MODEL_CACHE, |_, _| 0x7000u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_5e60, &args![script(0)]).bool());
        let formats: Vec<u32> = calls(&e, SPRINTF_S).iter().map(|w| w[2]).collect();
        assert_eq!(
            formats,
            vec![FORMAT_MEM_STATS, FORMAT_MODEL_DUMP, FORMAT_TEXTURE_DUMP]
        );
        // The name argument ("Run1") is copied aside and used in each
        // format; the heap dump object is configured with flag 0.
        let name = calls(&e, STRCPY_S)[0][0];
        assert!(calls(&e, SPRINTF_S).iter().all(|w| w[3] == name));
        let dump = calls(&e, MEM_STATS_FILE_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, MEM_STATS_OUTPUT_GET), vec![Vec::<u32>::new()]);
        assert_eq!(
            calls(&e, MEM_STATS_OUTPUT_SET),
            vec![vec![0x11f_6238, dump, 0]]
        );
        let model_file = calls(&e, BS_FILE_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, MODEL_LOADER_OUTPUT_MODEL_MAP),
            vec![vec![0x6000, model_file, 0]]
        );
        assert_eq!(
            calls(&e, FACE_GEN_OUTPUT_MODEL_MAP),
            vec![vec![0x7000, model_file]]
        );
        assert_eq!(calls(&e, BS_FILE_CLOSE).len(), 2);
        assert_eq!(calls(&e, BS_FILE_DESTRUCT).len(), 2);
        assert_eq!(calls(&e, MEM_STATS_FILE_DESTRUCT), vec![vec![dump]]);
        // The table of 0x800 NiPointers is built and destroyed.
        assert_eq!(calls(&e, VECTOR_CONSTRUCT)[0][1..4], [4, 0x800, 0x6694e0]);
        assert_eq!(calls(&e, VECTOR_DESTRUCT)[0][1..], [4, 0x800, 0x45cec0]);

        // Without an argument the default name is used and the model
        // loader's map is skipped.
        e.register(PARSE_PARAMETERS, |_, _| false.into_ret());
        e.set_global(MODEL_LOADER, 0u32);
        start_log(&mut e);
        assert!(e.call(0x005b_5e60, &args![script(0)]).bool());
        assert_eq!(calls(&e, STRCPY_S)[0][2], DEFAULT_MEM_STATS);
        assert!(calls(&e, MODEL_LOADER_OUTPUT_MODEL_MAP).is_empty());
    }

    #[test]
    fn mem_stats_lists_the_textures_with_their_category() {
        let mut e = output_engine();
        // The path fragments the categories are found by.
        for (addr, text) in [
            (PATH_ARMOR, "\\armor\\"),
            (PATH_CREATURES, "\\creatures\\"),
            (PATH_CHARACTERS, "\\characters\\"),
            (PATH_PIPBOY, "\\pipboy3000\\"),
            (PATH_WEAPONS, "\\weapons\\"),
            (PATH_DECALS, "\\decals\\"),
            (PATH_PROJECTILES, "\\projectiles\\"),
            (PATH_FONTS, "\\fonts\\"),
            (PATH_INTERFACE, "\\interface\\"),
            (PATH_EFFECTS, "\\effects\\"),
            (PATH_GORE, "\\gore\\"),
            (PATH_SKY, "\\sky\\"),
            (PATH_LOD, "\\lod\\"),
            (PATH_WATER, "\\water\\"),
        ] {
            put_str(&mut e, addr, text);
        }
        parse_gives(&mut e, false, &[]);
        // Three textures: a source texture under \armor\, a source texture
        // without a name, and a rendered one. Layout of the test objects:
        // +0x24 Direct3D texture, +0x2c next texture, +4 reference count,
        // +0x30 "is a source texture", +0x34 name text, +0x38 "L" kind.
        let vtable = e.mem.alloc(0x100);
        let owner_name = e.mem.alloc(8);
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, owner_name);
        e.register_double(V_KIND + 8, move |_, _| owner.into_ret());
        e.register(V_KIND + 9, |_, _| 128u32.into_ret());
        e.register(V_KIND + 10, |_, _| 64u32.into_ret());
        e.register(V_KIND + 11, |_, a| a[0].into_ret());
        e.mem.set_u32(vtable + 0x08, V_KIND + 8);
        e.mem.set_u32(vtable + 0x94, V_KIND + 9);
        e.mem.set_u32(vtable + 0x98, V_KIND + 10);
        e.mem.set_u32(vtable + 0x9c, V_KIND + 11);
        let armor_name = e.mem.alloc(0x40);
        put_str(&mut e, armor_name, "textures\\armor\\vault.dds");
        let empty = e.mem.alloc(8);
        let mut textures = vec![];
        for (is_source, name, kind) in [(1, armor_name, 3), (1, 0, 1), (0, empty, 1)] {
            let t = e.mem.alloc(0x60);
            e.mem.set_u32(t, vtable);
            e.mem.set_u32(t + 4, 3);
            e.mem.set_u32(t + 0x24, 0xd3d0 + textures.len() as u32);
            e.mem.set_u32(t + 0x30, is_source);
            e.mem.set_u32(t + 0x34, name);
            e.mem.set_u32(t + 0x38, kind);
            textures.push(t);
        }
        for i in 0..2 {
            e.mem.set_u32(textures[i] + 0x2c, textures[i + 1]);
        }
        let first = textures[0];
        e.register_double(FIRST_TEXTURE, move |_, _| first.into_ret());
        e.register(NEXT_TEXTURE, |e, a| e.mem.u32(a[0] + 0x2c).into_ret());
        e.register(GET_D3D_TEXTURE, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        e.register(NI_POINTER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(NI_POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(TEXTURE_IS_SOURCE, |e, a| {
            assert_eq!(a[0], TEXTURE_CLASS);
            (e.mem.u32(a[1] + 0x30) != 0).into_ret()
        });
        // A source texture's name comes from slot 0x9c (a handle that is the
        // texture itself here); a nameless one falls back to `+8`'s handle.
        e.register(NAME_TEXT, |e, a| e.mem.u32(a[0] + 0x34).into_ret());
        e.register(FN_00413F40, |_, a| a[0].into_ret());
        e.register(FN_007D6BB0, |_, a| a[0].into_ret());
        e.register(D3D_FORMAT_ID, |_, a| (a[0] + 1).into_ret());
        e.register(GET_D3D_FORMAT_STRING, |_, a| (a[0] + 2).into_ret());
        e.register(TEXTURE_KIND, |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_5e60, &args![script(0)]).bool());

        // The textures were inserted in order (all sizes are 0).
        let table = calls(&e, NI_POINTER_SET);
        assert_eq!(table.iter().map(|w| w[1]).collect::<Vec<_>>(), textures);
        assert_eq!(table[1][0], table[0][0] + 4);
        // Descriptions: the armor path, the unnamed one and the rendered one.
        let describing: Vec<u32> = calls(&e, STRCPY_S).iter().skip(1).map(|w| w[2]).collect();
        assert_eq!(describing, vec![armor_name, TEXT_UNNAMED_SOURCE_TEXTURE]);
        let rendered = calls(&e, SPRINTF_S)
            .into_iter()
            .filter(|w| w[2] == TEXT_RENDERED_TEXTURE_NONE)
            .count();
        assert_eq!(rendered, 1);
        // One line per texture: index, category, description, width, height,
        // size in KB, format, reference count minus one, "L"/"H", owner name.
        let lines: Vec<Vec<u32>> = calls(&e, SPRINTF_S)
            .into_iter()
            .filter(|w| w[2] == FORMAT_TEXTURE_LINE)
            .collect();
        assert_eq!(lines.len(), 3);
        let description = lines[0][5];
        assert_eq!(
            lines[0][3..],
            [
                1,
                CATEGORY_ACTOR,
                description,
                128,
                64,
                0,
                0xd3d0 + 3,
                2,
                TEXT_L,
                owner_name
            ]
        );
        // No path fragment matches the unnamed texture: MISCREF.
        assert_eq!(lines[1][3..5], [2, CATEGORY_MISCREF]);
        assert_eq!(lines[1][11], TEXT_H);
        // The rendered one is the "SYSTEM" category, whatever its text says.
        assert_eq!(lines[2][3..5], [3, CATEGORY_SYSTEM]);
        assert_eq!(calls(&e, BS_FILE_WRITE).len(), 3);
    }

    #[test]
    fn mem_stats_categories_follow_the_path() {
        // Each fragment, in the game's order, selects its category.
        for (path, category) in [
            ("a\\creatures\\x", CATEGORY_ACTOR),
            ("a\\projectiles\\x", CATEGORY_ACTOR),
            ("a\\interface\\x", CATEGORY_MENUS),
            ("a\\gore\\x", CATEGORY_EFFECTS),
            ("a\\sky\\x", CATEGORY_SKY),
            ("a\\lod\\x", CATEGORY_LOD),
            ("a\\water\\x", CATEGORY_WATER),
            ("a\\other\\x", CATEGORY_MISCREF),
        ] {
            let mut e = output_engine();
            for (addr, text) in [
                (PATH_ARMOR, "\\armor\\"),
                (PATH_CREATURES, "\\creatures\\"),
                (PATH_CHARACTERS, "\\characters\\"),
                (PATH_PIPBOY, "\\pipboy3000\\"),
                (PATH_WEAPONS, "\\weapons\\"),
                (PATH_DECALS, "\\decals\\"),
                (PATH_PROJECTILES, "\\projectiles\\"),
                (PATH_FONTS, "\\fonts\\"),
                (PATH_INTERFACE, "\\interface\\"),
                (PATH_EFFECTS, "\\effects\\"),
                (PATH_GORE, "\\gore\\"),
                (PATH_SKY, "\\sky\\"),
                (PATH_LOD, "\\lod\\"),
                (PATH_WATER, "\\water\\"),
            ] {
                put_str(&mut e, addr, text);
            }
            parse_gives(&mut e, false, &[]);
            let name = e.mem.alloc(0x40);
            put_str(&mut e, name, path);
            let vtable = e.mem.alloc(0x100);
            let owner = e.mem.alloc(8);
            e.register_double(V_KIND + 8, move |_, _| owner.into_ret());
            e.mem.set_u32(vtable + 0x08, V_KIND + 8);
            e.register(V_KIND + 9, |_, a| a[0].into_ret());
            e.mem.set_u32(vtable + 0x9c, V_KIND + 9);
            e.mem.set_u32(vtable + 0x94, V_KIND);
            e.mem.set_u32(vtable + 0x98, V_KIND);
            let texture = e.mem.alloc(0x60);
            e.mem.set_u32(texture, vtable);
            e.mem.set_u32(texture + 0x34, name);
            e.register_double(FIRST_TEXTURE, move |_, _| texture.into_ret());
            e.register(NEXT_TEXTURE, |_, _| 0u32.into_ret());
            e.register(GET_D3D_TEXTURE, |_, _| 1u32.into_ret());
            e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
            e.register(NI_POINTER_SET, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                Ret::default()
            });
            e.register(TEXTURE_IS_SOURCE, |_, _| true.into_ret());
            e.register(NAME_TEXT, |e, a| e.mem.u32(a[0] + 0x34).into_ret());
            accept(&mut e, &[FN_00413F40, FN_007D6BB0, D3D_FORMAT_ID]);
            e.register(GET_D3D_FORMAT_STRING, |_, _| 0u32.into_ret());
            e.register(TEXTURE_KIND, |_, _| 0u32.into_ret());
            start_log(&mut e);
            assert!(e.call(0x005b_5e60, &args![script(0)]).bool());
            let line = calls(&e, SPRINTF_S)
                .into_iter()
                .find(|w| w[2] == FORMAT_TEXTURE_LINE)
                .unwrap();
            assert_eq!(line[4], category, "{path}");
        }
    }

    #[test]
    fn texture_use_map_writes_the_file_and_reports_the_size() {
        let mut e = output_engine();
        accept(
            &mut e,
            &[
                TEXTURE_USE_MAP_CONSTRUCT,
                TEXTURE_USE_MAP_ADD_NODE,
                TEXTURE_USE_MAP_ADD_SCENE,
                TEXTURE_USE_MAP_DESTRUCT,
                BS_STRING_CONSTRUCT,
                BS_STRING_DESTRUCT,
                BS_STRING_APPEND,
            ],
        );
        e.register(BS_STRING_FORMAT, |e, a| {
            e.mem.set_u32(a[0], 0x5151);
            Ret::default()
        });
        put_str(&mut e, DEFAULT_TEXTURE_USE_MAP, "TextureUseMap.txt");
        e.register_double(PARSE_PARAMETERS, |e, a| {
            e.mem.set_cstr(a[7], b"my.txt");
            true.into_ret()
        });
        let this_obj = object_with(&mut e, &[(0x1d0, V_NINE)]);

        // A reference with a 3D (slot 0x1d0): its node; written; 5 MB.
        e.register(TEXTURE_USE_MAP_WRITE_TO_FILE, |_, _| true.into_ret());
        e.register(TEXTURE_USE_MAP_TOTAL_SIZE, |_, _| 0x50_0000u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_6800, &args![script(this_obj)]).bool());
        let map = calls(&e, TEXTURE_USE_MAP_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, TEXTURE_USE_MAP_ADD_NODE), vec![vec![map, 9, 1]]);
        assert!(calls(&e, TEXTURE_USE_MAP_ADD_SCENE).is_empty());
        let path = calls(&e, TEXTURE_USE_MAP_WRITE_TO_FILE)[0][1];
        assert_eq!(e.mem.cstr(path), b"my.txt");
        let text = calls(&e, BS_STRING_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, BS_STRING_FORMAT), vec![vec![text, FORMAT_MB, 5]]);
        assert_eq!(
            calls(&e, BS_STRING_APPEND),
            vec![vec![text, TEXT_TEXTURES_IN_USE]]
        );
        assert_eq!(printed(&e), vec![vec![MSG_MAP_WRITTEN, path], vec![0x5151]]);
        assert_eq!(calls(&e, TEXTURE_USE_MAP_DESTRUCT), vec![vec![map]]);

        // No reference: the whole scene. A failed write is reported; sizes
        // pick KB above 0x400 bytes and bytes at or below.
        e.register(TEXTURE_USE_MAP_WRITE_TO_FILE, |_, _| false.into_ret());
        for (size, format, shown) in [
            (0x2000u32, FORMAT_KB, 8),
            (0x401, FORMAT_KB, 1),
            (0x400, FORMAT_BYTES, 0x400),
            (0x10_0000, FORMAT_KB, 0x400),
            (0x10_0001, FORMAT_MB, 1),
        ] {
            e.register_double(TEXTURE_USE_MAP_TOTAL_SIZE, move |_, _| size.into_ret());
            start_log(&mut e);
            assert!(e.call(0x005b_6800, &args![script(0)]).bool());
            let map = calls(&e, TEXTURE_USE_MAP_CONSTRUCT)[0][0];
            assert_eq!(calls(&e, TEXTURE_USE_MAP_ADD_SCENE), vec![vec![map, 1]]);
            let text = calls(&e, BS_STRING_CONSTRUCT)[0][0];
            assert_eq!(calls(&e, BS_STRING_FORMAT), vec![vec![text, format, shown]]);
            let path = calls(&e, TEXTURE_USE_MAP_WRITE_TO_FILE)[0][1];
            assert_eq!(printed(&e)[0], vec![MSG_MAP_WRITE_FAILED, path]);
        }

        // Bad parameters: the default file name is used.
        e.register(PARSE_PARAMETERS, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_6800, &args![script(0)]).bool());
        assert_eq!(calls(&e, STRCPY_S).len(), 2);
        let path = calls(&e, TEXTURE_USE_MAP_WRITE_TO_FILE)[0][1];
        assert_eq!(e.mem.cstr(path), b"TextureUseMap.txt");
    }

    #[test]
    fn the_other_memory_dump_command_configures_the_output_with_flag_one() {
        let mut e = output_engine();
        put_str(&mut e, DEFAULT_MEM_STATS, "OutputMemStats.mhd");
        e.register_double(PARSE_PARAMETERS, |e, a| {
            e.mem.set_cstr(a[7], b"Other.mhd");
            true.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005b_6a30, &args![script(0)]).bool());
        let dump = calls(&e, MEM_STATS_FILE_CONSTRUCT)[0][0];
        let path = calls(&e, MEM_STATS_FILE_CONSTRUCT)[0][1];
        assert_eq!(e.mem.cstr(path), b"Other.mhd");
        assert_eq!(calls(&e, MEM_STATS_OUTPUT_GET), vec![Vec::<u32>::new()]);
        assert_eq!(
            calls(&e, MEM_STATS_OUTPUT_SET),
            vec![vec![0x11f_6238, dump, 1]]
        );
        assert_eq!(calls(&e, MEM_STATS_FILE_DESTRUCT), vec![vec![dump]]);
        // Bad parameters: the default name is used.
        e.register(PARSE_PARAMETERS, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_6a30, &args![script(0)]).bool());
        let path = calls(&e, MEM_STATS_FILE_CONSTRUCT)[0][1];
        assert_eq!(e.mem.cstr(path), b"OutputMemStats.mhd");
        assert_eq!(calls(&e, STRNCPY_S).len(), 2);
    }

    #[test]
    fn archive_profile_appends_the_extension_and_reports_it_is_disabled() {
        let mut e = output_engine();
        put_str(&mut e, EXTENSION_TXT, ".txt");
        put_str(&mut e, DEFAULT_ARCHIVE_PROFILE, "ArchiveProfile.txt");
        e.register(ARCHIVE_PROFILE_START, |_, _| false.into_ret());
        e.register_double(PARSE_PARAMETERS, |e, a| {
            e.mem.set_cstr(a[7], b"prof");
            true.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005b_6b50, &args![script(0)]).bool());
        let path = calls(&e, STRCAT_S)[0][0];
        assert_eq!(calls(&e, STRCAT_S), vec![vec![path, 0x200, EXTENSION_TXT]]);
        assert_eq!(e.mem.cstr(path), b"prof.txt");
        assert_eq!(printed(&e), vec![vec![MSG_ARCHIVE_PROFILE_DISABLED]]);

        // A name that has the extension is kept; when profiling starts the
        // file name is printed.
        e.register_double(PARSE_PARAMETERS, |e, a| {
            e.mem.set_cstr(a[7], b"prof.txt");
            true.into_ret()
        });
        e.register(ARCHIVE_PROFILE_START, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_6b50, &args![script(0)]).bool());
        assert!(calls(&e, STRCAT_S).is_empty());
        let path = calls(&e, ARCHIVE_PROFILE_START)[0][0];
        assert_eq!(printed(&e), vec![vec![MSG_ARCHIVE_PROFILE_OUTPUT, path]]);

        // Bad parameters: the default name, which has the extension.
        e.register(PARSE_PARAMETERS, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_6b50, &args![script(0)]).bool());
        let path = calls(&e, ARCHIVE_PROFILE_START)[0][0];
        assert_eq!(e.mem.cstr(path), b"ArchiveProfile.txt");
    }

    #[test]
    fn two_singletons_get_their_plus_4_member_handed_back() {
        let mut e = engine();
        e.register(FN_0044F560, |_, _| 0x1000u32.into_ret());
        e.register(FN_004DE490, |_, _| 0x2000u32.into_ret());
        e.register(PLUS_4, |_, a| (a[0] + 4).into_ret());
        accept(&mut e, &[FN_005E01B0]);
        start_log(&mut e);
        assert!(e.call(0x005b_6c50, &args![]).bool());
        assert_eq!(
            calls(&e, FN_005E01B0),
            vec![vec![0x1000, 0x1004], vec![0x2000, 0x2004]]
        );
    }

    #[test]
    fn bye_sets_the_flag_byte_of_the_singleton() {
        let mut e = engine();
        let singleton = e.mem.alloc(8);
        e.set_global(GLOBAL_0011DEA0C, singleton);
        assert_eq!(run_and_print(&mut e, 0x005b_6c90), vec![vec![MSG_BYE]]);
        assert_eq!(e.mem.u8(singleton + 1), 1);
        // The setter on its own.
        let other = e.mem.alloc(8);
        e.call(0x005b_6cb0, &args![other]);
        assert_eq!(e.mem.u8(other + 1), 1);
        assert_eq!(e.mem.u8(other), 0);
    }

    #[test]
    fn free_released_objects_wraps_the_call_in_a_level_scope() {
        let mut e = engine();
        e.register(FN_00878160, |e, a| {
            e.mem.set_u8(a[0] + 5, 0x42);
            Ret::default()
        });
        accept(&mut e, &[FN_00878200, FN_004539A0, FREE_RELEASED_OBJECTS]);
        e.set_global(GLOBAL_0011DEA10, 0x1234u32);
        start_log(&mut e);
        assert!(e.call(0x005b_6cd0, &args![]).bool());
        // Skip the logged call of the command itself.
        let log: Vec<(u32, Vec<u32>)> = e.call_log.clone().unwrap().into_iter().skip(1).collect();
        let scope = log[0].1[0];
        assert_eq!(
            log,
            vec![
                (FN_00878160, vec![scope, 1, 1, 1]),
                (FN_004539A0, vec![0x1234, 1, 0]),
                (FREE_RELEASED_OBJECTS, vec![0x42]),
                (FN_00878200, vec![scope]),
            ]
        );
    }

    #[test]
    fn the_save_load_singleton_is_called_with_zeros() {
        let mut e = engine();
        accept(&mut e, &[FN_00848E70]);
        e.set_global(SAVE_LOAD_GAME, 0x7777u32);
        start_log(&mut e);
        assert!(e.call(0x005b_6d20, &args![]).bool());
        assert_eq!(calls(&e, FN_00848E70), vec![vec![0x7777, 0, 0]]);
    }

    /// Doubles and objects for `ToggleMapMarkers`: two world spaces (the
    /// second has no markers) and, for the first, two markers; the first
    /// marker's data (at +0x10) is hidden. Data bytes: +0 hidden, +1 has a
    /// travel location.
    fn map_marker_world() -> (Engine, [u32; 2]) {
        let mut e = engine();
        let m1 = object_with(&mut e, &[(0x48, V_RECORD)]);
        let m2 = object_with(&mut e, &[(0x48, V_RECORD)]);
        e.mem.set_u8(m1 + 0x10, 1);
        let w1 = e.mem.alloc(8);
        let w2 = e.mem.alloc(8);
        let worlds = list_of(&mut e, &[w1, w2]);
        e.set_global(DATA_HANDLER, 0x5000u32);
        e.register_double(DATA_HANDLER_WORLD_SPACES, move |_, a| {
            assert_eq!(a[0], 0x5000);
            worlds.into_ret()
        });
        e.register_double(BUILD_MAP_MARKER_LIST, move |e, a| {
            assert_eq!(a[1], 0);
            if a[0] == w1 {
                list_of(e, &[m1, m2]).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(GET_MAP_MARKER_DATA, |_, a| (a[0] + 0x10).into_ret());
        e.register(MAP_MARKER_GET_HIDDEN, |e, a| {
            (e.mem.u8(a[0]) != 0).into_ret()
        });
        e.register(MAP_MARKER_GET_TRAVEL_LOC, |e, a| {
            (e.mem.u8(a[0] + 1) != 0).into_ret()
        });
        accept(&mut e, &[MAP_MARKER_SET_VISIBLE, MAP_MARKER_SET_TRAVEL_LOC]);
        (e, [m1, m2])
    }

    #[test]
    fn toggle_map_markers_shows_the_markers_that_are_not_hidden() {
        let (mut e, [m1, m2]) = map_marker_world();
        parse_gives(&mut e, true, &[1, 1, 1]);
        start_log(&mut e);
        assert!(e.call(0x005b_6d40, &args![script(0)]).bool());
        // The hidden first marker is skipped; the second is made visible, its
        // travel location set (it had none), and slot 0x48 called on it.
        assert_eq!(calls(&e, MAP_MARKER_SET_VISIBLE), vec![vec![m2 + 0x10, 1]]);
        assert_eq!(
            calls(&e, MAP_MARKER_SET_TRAVEL_LOC),
            vec![vec![m2 + 0x10, 1]]
        );
        assert_eq!(calls(&e, V_RECORD), vec![vec![m2, 0x8000_0000]]);
        // The marker list is deleted; the second world has none.
        assert_eq!(calls(&e, LIST_DELETE).len(), 1);
        assert_eq!(calls(&e, LIST_DELETE)[0][1], 1);
        // The echo flag is off: no print.
        assert!(printed(&e).is_empty());

        // A marker that already has a travel location keeps it.
        e.mem.set_u8(m2 + 0x11, 1);
        start_log(&mut e);
        assert!(e.call(0x005b_6d40, &args![script(0)]).bool());
        assert!(calls(&e, MAP_MARKER_SET_TRAVEL_LOC).is_empty());
        let _ = m1;
    }

    #[test]
    fn toggle_map_markers_hides_everything_and_echoes() {
        let (mut e, [m1, m2]) = map_marker_world();
        // Hide all, hidden markers included (third argument 0).
        parse_gives(&mut e, true, &[0, 1, 0]);
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_ECHO, 1);
        accept(&mut e, &[CONSOLE_PRINT]);
        start_log(&mut e);
        assert!(e.call(0x005b_6d40, &args![script(0)]).bool());
        assert_eq!(
            calls(&e, MAP_MARKER_SET_VISIBLE),
            vec![vec![m1 + 0x10, 0], vec![m2 + 0x10, 0]]
        );
        assert_eq!(
            calls(&e, MAP_MARKER_SET_TRAVEL_LOC),
            vec![vec![m1 + 0x10, 0], vec![m2 + 0x10, 0]]
        );
        assert_eq!(
            calls(&e, V_RECORD),
            vec![vec![m1, 0x8000_0000], vec![m2, 0x8000_0000]]
        );
        assert_eq!(printed(&e), vec![vec![MSG_MAP_MARKERS, TEXT_HIDDEN]]);
        // Shown: the message says so.
        parse_gives(&mut e, true, &[1, 1, 1]);
        start_log(&mut e);
        assert!(e.call(0x005b_6d40, &args![script(0)]).bool());
        assert_eq!(printed(&e), vec![vec![MSG_MAP_MARKERS, TEXT_SHOWN]]);
        // Bad parameters: false and no walk.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_6d40, &args![script(0)]).bool());
        assert!(printed(&e).is_empty());
    }

    #[test]
    fn verbose_flag_accessors_and_toggle() {
        let mut e = engine();
        assert!(!e.call(0x005b_6f70, &args![]).bool());
        e.call(0x005b_6f60, &args![true]);
        assert!(e.call(0x005b_6f70, &args![]).bool());
        assert_eq!(e.global::<u8>(VERBOSE_MESSAGES), 1);
        e.call(0x005b_6f60, &args![false]);
        assert_eq!(e.global::<u8>(VERBOSE_MESSAGES), 0);
        // The toggle flips and prints the new state.
        assert_eq!(
            run_and_print(&mut e, 0x005b_6f10),
            vec![vec![MSG_VERBOSE, TEXT_SHOWN]]
        );
        assert_eq!(
            run_and_print(&mut e, 0x005b_6f10),
            vec![vec![MSG_VERBOSE, TEXT_HIDDEN]]
        );
    }

    /// God and demigod mode doubles over the exe's two flag bytes.
    fn mode_engine() -> Engine {
        let mut e = engine();
        e.register(IS_GOD_MODE, |e, _| (e.mem.u8(0x011e_07ba) != 0).into_ret());
        e.register(IS_DEMIGOD_MODE, |e, _| {
            (e.mem.u8(0x011e_07bb) != 0).into_ret()
        });
        e.register(SET_GOD_MODE, |e, a| {
            e.mem.set_u8(0x011e_07ba, a[0] as u8);
            Ret::default()
        });
        e.register(SET_DEMIGOD_MODE, |e, a| {
            e.mem.set_u8(0x011e_07bb, a[0] as u8);
            Ret::default()
        });
        e
    }

    #[test]
    fn god_mode_toggle_also_ends_demigod_mode_when_turned_off() {
        let mut e = mode_engine();
        assert_eq!(
            run_and_print(&mut e, 0x005b_6f80),
            vec![vec![MSG_GOD_MODE, TEXT_ENABLED]]
        );
        assert_eq!(e.mem.u8(0x011e_07ba), 1);
        // Demigod mode on as well: turning god mode off clears it with a
        // message of its own.
        e.mem.set_u8(0x011e_07bb, 1);
        assert_eq!(
            run_and_print(&mut e, 0x005b_6f80),
            vec![
                vec![MSG_DEMIGOD_DISABLED],
                vec![MSG_GOD_MODE, TEXT_DISABLED]
            ]
        );
        assert_eq!(e.mem.u8(0x011e_07ba), 0);
        assert_eq!(e.mem.u8(0x011e_07bb), 0);
        // Off with demigod mode off: just the one message.
        assert_eq!(
            run_and_print(&mut e, 0x005b_6f80),
            vec![vec![MSG_GOD_MODE, TEXT_ENABLED]]
        );
    }

    #[test]
    fn demigod_mode_toggle_sets_both_flags_alike() {
        let mut e = mode_engine();
        assert_eq!(
            run_and_print(&mut e, 0x005b_7000),
            vec![vec![MSG_DEMIGOD_MODE, TEXT_ENABLED]]
        );
        assert_eq!(e.mem.u8(0x011e_07ba), 1);
        assert_eq!(e.mem.u8(0x011e_07bb), 1);
        assert_eq!(
            run_and_print(&mut e, 0x005b_7000),
            vec![vec![MSG_DEMIGOD_MODE, TEXT_DISABLED]]
        );
        assert_eq!(e.mem.u8(0x011e_07ba), 0);
        assert_eq!(e.mem.u8(0x011e_07bb), 0);
    }

    // ---- The second batch (005b7070 onward) -----------------------------------

    /// [`engine`] with the pages of the globals and constants of the second
    /// batch mapped.
    fn engine_b() -> Engine {
        let mut e = engine();
        for page in [
            0x0101_1000,
            0x0101_7000,
            0x0101_8000,
            0x0101_a000,
            0x0101_d000,
            0x0101_e000,
            0x0102_0000,
            0x0102_2000,
            0x0118_c000,
            0x0119_b000,
            0x011a_2000,
            0x011a_9000,
            0x011c_6000,
            0x011c_a000,
            0x011c_b000,
            0x011f_2000,
            0x011f_4000,
            0x0126_8000,
        ] {
            e.map(page, 0x1000);
        }
        e
    }

    /// Sets the TLS echo flag.
    fn set_echo(e: &mut Engine, on: bool) {
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_ECHO, on as u8);
    }

    #[test]
    fn toggle_scripts_flips_script_processing() {
        let mut e = engine_b();
        // Script processing is a byte at 0x011f9000 in the doubles.
        e.register(GET_PROCESS_SCRIPTS, |e, _| {
            (e.mem.u8(0x011f_9000) != 0).into_ret()
        });
        e.register(SET_PROCESS_SCRIPTS, |e, a| {
            e.mem.set_u8(0x011f_9000, a[0] as u8);
            Ret::default()
        });
        assert_eq!(
            run_and_print(&mut e, 0x005b_7070),
            vec![vec![MSG_SCRIPT_PROCESSING, TEXT_ENABLED]]
        );
        assert_eq!(
            run_and_print(&mut e, 0x005b_7070),
            vec![vec![MSG_SCRIPT_PROCESSING, TEXT_DISABLED]]
        );
        assert_eq!(e.mem.u8(0x011f_9000), 0);
    }

    /// The grass object of `ToggleGrass`: a byte at +0 is its flag.
    fn grass_world(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(16);
        e.register_double(FN_0054F4C0, move |_, _| object.into_ret());
        e.register(FN_00456610, |e, a| (e.mem.u8(a[0]) != 0).into_ret());
        e.register(FN_00450F90, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            Ret::default()
        });
        accept(e, &[CONSOLE_PRINT, FN_00B62A20, FN_0057D0A0]);
        e.register(FN_0045C670, |_, _| 0x5000u32.into_ret());
        object
    }

    #[test]
    fn toggle_grass_does_nothing_without_the_grass_object() {
        let mut e = engine_b();
        e.register(FN_0054F4C0, |_, _| 0u32.into_ret());
        assert!(run_and_print(&mut e, 0x005b_70c0).is_empty());
        assert_eq!(e.global::<u8>(GRASS_DISPLAY_FLAG), 0);
    }

    #[test]
    fn toggle_grass_sets_the_flag_and_the_display_byte() {
        let mut e = engine_b();
        let object = grass_world(&mut e);
        // The object's flag is 0: the new flag is 1, the display byte 0 and
        // `00b62a20` runs; "Disabled." is printed.
        start_log(&mut e);
        assert_eq!(
            run_and_print(&mut e, 0x005b_70c0),
            vec![vec![MSG_GRASS_DISPLAY, TEXT_DISABLED_CAPITAL]]
        );
        assert_eq!(calls(&e, FN_00450F90), vec![vec![object, 1]]);
        assert_eq!(e.mem.u8(object), 1);
        assert_eq!(e.global::<u8>(GRASS_DISPLAY_FLAG), 0);
        assert_eq!(calls(&e, FN_00B62A20).len(), 1);
        assert!(calls(&e, FN_0057D0A0).is_empty());
    }

    #[test]
    fn toggle_grass_off_places_the_grass_at_the_animation_data_vector() {
        let mut e = engine_b();
        let object = grass_world(&mut e);
        e.mem.set_u8(object, 1);
        // `0045c670` -> holder 0x5000; its animation data (`006629f0`) is
        // 0x6000; `0045bb80` returns a vector in memory.
        let animation = e.mem.alloc(0x100);
        let vector = e.mem.alloc(12);
        for (i, w) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(vector + 4 * i as u32, *w);
        }
        for (i, w) in [7.0f32, 8.0, 9.0].iter().enumerate() {
            e.set_global(START_VECTOR + 4 * i as u32, *w);
        }
        e.register_double(FN_006629F0, move |_, a| {
            assert_eq!(a[0], 0x5000);
            animation.into_ret()
        });
        e.register_double(FN_0045BB80, move |_, _| vector.into_ret());
        start_log(&mut e);
        assert_eq!(
            run_and_print(&mut e, 0x005b_70c0),
            vec![vec![MSG_GRASS_DISPLAY, TEXT_ENABLED_CAPITAL]]
        );
        assert_eq!(calls(&e, FN_00450F90), vec![vec![object, 0]]);
        assert_eq!(e.global::<u8>(GRASS_DISPLAY_FLAG), 1);
        assert!(calls(&e, FN_00B62A20).is_empty());
        // `0045bb80`'s vector, then the start vector and a 0.0, go to `0057d0a0`.
        assert_eq!(calls(&e, FN_0045BB80), vec![vec![animation]]);
        assert_eq!(
            calls(&e, FN_0057D0A0),
            vec![vec![
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                7.0f32.to_bits(),
                8.0f32.to_bits(),
                9.0f32.to_bits(),
                0
            ]]
        );
        // Without animation data nothing is placed.
        e.mem.set_u8(object, 1);
        e.register(FN_006629F0, |_, _| 0u32.into_ret());
        start_log(&mut e);
        run_and_print(&mut e, 0x005b_70c0);
        assert!(calls(&e, FN_0057D0A0).is_empty());
    }

    #[test]
    fn test_all_cells_passes_the_count_and_reports_the_state() {
        let mut e = engine_b();
        let tes = 0x7000u32;
        e.set_global(GLOBAL_0011DEA10, tes);
        accept(&mut e, &[TES_TEST_ALL_CELLS, CONSOLE_PRINT]);
        e.register(TES_IS_RUNNING_CELL_TESTS, |_, _| true.into_ret());
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005b_71b0, &args![script(0)]).bool());
        // No count: -1.
        assert_eq!(calls(&e, TES_TEST_ALL_CELLS), vec![vec![tes, u32::MAX]]);
        assert_eq!(printed(&e), vec![vec![MSG_TEST_ALL_CELLS, TEXT_RUNNING]]);
        // A count is passed as it is; a stopped test says so.
        parse_gives(&mut e, true, &[25]);
        e.register(TES_IS_RUNNING_CELL_TESTS, |_, _| false.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_71b0, &args![script(0)]).bool());
        assert_eq!(calls(&e, TES_TEST_ALL_CELLS), vec![vec![tes, 25]]);
        assert_eq!(printed(&e), vec![vec![MSG_TEST_ALL_CELLS, TEXT_STOPPED]]);
        // Bad parameters.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_71b0, &args![script(0)]).bool());
        assert!(calls(&e, TES_TEST_ALL_CELLS).is_empty());
    }

    #[test]
    fn render_test_cell_commands_render_the_players_parent_cell() {
        let mut e = engine_b();
        e.set_global(PLAYER, 0x4000u32);
        accept(&mut e, &[CELL_RENDER_TEST_CELL, CONSOLE_PRINT]);
        // No parent cell.
        e.register(REFR_GET_PARENT_CELL, |_, _| 0u32.into_ret());
        for addr in [0x005b_7250, 0x005b_72a0] {
            assert_eq!(
                run_and_print(&mut e, addr),
                vec![vec![MSG_RENDER_TEST_CELL_FAILED]]
            );
            assert!(calls(&e, CELL_RENDER_TEST_CELL).is_empty());
        }
        // A cell: flag 1 for the first command, 0 for the second.
        e.register(REFR_GET_PARENT_CELL, |_, a| {
            assert_eq!(a, [0x4000]);
            0x8000u32.into_ret()
        });
        assert_eq!(
            run_and_print(&mut e, 0x005b_7250),
            vec![vec![MSG_RENDER_TEST_CELL_COMPLETE]]
        );
        assert_eq!(calls(&e, CELL_RENDER_TEST_CELL), vec![vec![0x8000, 0, 1]]);
        assert_eq!(
            run_and_print(&mut e, 0x005b_72a0),
            vec![vec![MSG_RENDER_TEST_CELL_COMPLETE]]
        );
        assert_eq!(calls(&e, CELL_RENDER_TEST_CELL), vec![vec![0x8000, 0, 0]]);
    }

    #[test]
    fn ini_refresh_loads_each_ini_file_that_has_a_name() {
        let mut e = engine_b();
        // The names the pointers point to and the game directory.
        let directory = e.mem.alloc(16);
        e.mem.set_cstr(directory, b"C:\\Game\\");
        let names = [
            (FALLOUT_INI_NAME, b"Fallout.ini".as_slice()),
            (CUSTOM_INI_NAME, b"Custom.ini".as_slice()),
            (FALLOUT_PREFS_INI_NAME, b"FalloutPrefs.ini".as_slice()),
        ];
        for (slot, text) in names {
            let at = e.mem.alloc(32);
            e.mem.set_cstr(at, text);
            e.set_global(slot, at);
        }
        e.register_double(FN_004DC110, move |_, _| directory.into_ret());
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRCAT_S, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(FN_0044F560, |_, _| 0xa000u32.into_ret());
        e.register(FN_004DE490, |_, _| 0xb000u32.into_ret());
        // The double sees the path while the buffer lives.
        e.register_double(FN_005E0200, |e, a| {
            let path = String::from_utf8(e.mem.cstr(a[1])).unwrap();
            let mut log = e.mem.cstr(0x0126_8100);
            log.extend_from_slice(format!("{}|{}\n", a[0], path).as_bytes());
            e.mem.set_cstr(0x0126_8100, &log);
            Ret::default()
        });
        accept(&mut e, &[CONSOLE_PRINT]);
        start_log(&mut e);
        assert!(e.call(0x005b_72f0, &args![]).bool());
        let log = String::from_utf8(e.mem.cstr(0x0126_8100)).unwrap();
        assert_eq!(
            log,
            format!(
                "{}|C:\\Game\\Fallout.ini\n{}|C:\\Game\\Custom.ini\n{}|C:\\Game\\FalloutPrefs.ini\n",
                0xa000, 0xa000, 0xb000
            )
        );
        assert_eq!(printed(&e), vec![vec![MSG_INI_REFRESHED]]);
        // A missing name is skipped.
        e.set_global(CUSTOM_INI_NAME, 0u32);
        e.mem.set_cstr(0x0126_8100, b"");
        assert!(e.call(0x005b_72f0, &args![]).bool());
        let log = String::from_utf8(e.mem.cstr(0x0126_8100)).unwrap();
        assert_eq!(log.lines().count(), 2);
    }

    #[test]
    fn emotions_flag_accessors_and_toggle() {
        let mut e = engine_b();
        assert_eq!(e.call(0x005b_7470, &args![]).u8(), 0);
        e.call(0x005b_7480, &args![1u32]);
        assert_eq!(e.call(0x005b_7470, &args![]).u8(), 1);
        assert_eq!(e.global::<u8>(EMOTIONS_FLAG), 1);
        assert_eq!(
            run_and_print(&mut e, 0x005b_7420),
            vec![vec![MSG_NPC_EMOTIONS, TEXT_DISABLED_CAPITAL]]
        );
        assert_eq!(
            run_and_print(&mut e, 0x005b_7420),
            vec![vec![MSG_NPC_EMOTIONS, TEXT_ENABLED_CAPITAL]]
        );
    }

    /// A sky with a climate and a weather, a form file for each and doubles
    /// for the file functions. Returns `(sky, climate, weather)`.
    fn sky_world(e: &mut Engine) -> (u32, u32, u32) {
        let sky = e.mem.alloc(0x138);
        let climate = object_with(e, &[(0x18, V_TRUE), (0x20, V_RECORD)]);
        let weather = object_with(e, &[(0x18, V_TRUE), (0x20, V_RECORD)]);
        e.mem.set_u32(sky + 0x0c, climate);
        e.mem.set_u32(sky + 0x10, weather);
        e.register_double(SKY_GET_INSTANCE, move |_, _| sky.into_ret());
        e.register(SKY_GET_CURRENT_CLIMATE, |e, a| {
            e.mem.u32(a[0] + 0x0c).into_ret()
        });
        e.register(SKY_GET_CURRENT_WEATHER, |e, a| {
            e.mem.u32(a[0] + 0x10).into_ret()
        });
        e.register(FORM_GET_FILE, |_, a| (a[0] + 0xe000).into_ret());
        e.register(FILE_GET_THREAD_SAFE_FILE, |_, a| (a[0] + 0x10).into_ret());
        accept(
            e,
            &[
                FILE_OPEN_TES,
                FILE_FIND_FORM,
                SKY_SET_CURRENT_CLIMATE,
                SKY_FORCE_WEATHER,
                FN_0093A7A0,
                FN_0063E860,
                SKY_CLEAR_OVERRIDE_WEATHER,
            ],
        );
        (sky, climate, weather)
    }

    #[test]
    fn reload_current_climate_reloads_the_climate_from_its_file() {
        let mut e = engine_b();
        let (sky, climate, _) = sky_world(&mut e);
        start_log(&mut e);
        assert!(e.call(0x005b_7490, &args![]).bool());
        let file = climate + 0xe000 + 0x10;
        assert_eq!(calls(&e, FORM_GET_FILE), vec![vec![climate, u32::MAX]]);
        assert_eq!(calls(&e, FILE_OPEN_TES), vec![vec![file, 0, 0]]);
        assert_eq!(calls(&e, FILE_FIND_FORM), vec![vec![file, climate]]);
        // Virtual slots 0x18 (no arguments) and 0x20 (the file).
        assert_eq!(calls(&e, V_TRUE), vec![vec![climate]]);
        assert_eq!(calls(&e, V_RECORD), vec![vec![climate, file]]);
        assert_eq!(
            calls(&e, SKY_SET_CURRENT_CLIMATE),
            vec![vec![sky, climate, 1]]
        );
        // A climate without a file: nothing is reloaded.
        e.register(FORM_GET_FILE, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_7490, &args![]).bool());
        assert!(calls(&e, FILE_OPEN_TES).is_empty());
        assert!(calls(&e, SKY_SET_CURRENT_CLIMATE).is_empty());
    }

    #[test]
    fn reload_current_weather_reloads_the_weather_only() {
        let mut e = engine_b();
        let (_, _, weather) = sky_world(&mut e);
        start_log(&mut e);
        assert!(e.call(0x005b_7520, &args![]).bool());
        let file = weather + 0xe000 + 0x10;
        assert_eq!(calls(&e, FILE_FIND_FORM), vec![vec![file, weather]]);
        assert_eq!(calls(&e, V_RECORD), vec![vec![weather, file]]);
        assert!(calls(&e, SKY_SET_CURRENT_CLIMATE).is_empty());
        e.register(FORM_GET_FILE, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_7520, &args![]).bool());
        assert!(calls(&e, FILE_OPEN_TES).is_empty());
    }

    #[test]
    fn set_weather_stores_the_weather_in_the_override_or_default_slot() {
        let mut e = engine_b();
        let (sky, _, weather) = sky_world(&mut e);
        e.set_global(PLAYER, 0x4000u32);
        // Override flag set: the override slot, `0063e860(1)` as the echo is off.
        parse_gives(&mut e, true, &[0x1111, 1]);
        start_log(&mut e);
        assert!(e.call(0x005b_7590, &args![script(0)]).bool());
        assert_eq!(e.mem.u32(sky + SKY_OVERRIDE_WEATHER), 0x1111);
        assert_eq!(e.mem.u32(sky + SKY_DEFAULT_WEATHER), 0);
        assert_eq!(calls(&e, FN_0063E860), vec![vec![sky, 1]]);
        assert!(calls(&e, FN_0093A7A0).is_empty());
        // No override: the default slot and the player is told.
        parse_gives(&mut e, true, &[0x2222, 0]);
        start_log(&mut e);
        assert!(e.call(0x005b_7590, &args![script(0)]).bool());
        assert_eq!(e.mem.u32(sky + SKY_DEFAULT_WEATHER), 0x2222);
        assert_eq!(calls(&e, FN_0093A7A0), vec![vec![0x4000, 0]]);
        // With the echo flag set `0063e860` is not called.
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005b_7590, &args![script(0)]).bool());
        assert!(calls(&e, FN_0063E860).is_empty());
        set_echo(&mut e, false);
        // Without a current weather the weather is forced.
        e.mem.set_u32(sky + 0x10, 0);
        parse_gives(&mut e, true, &[0x3333, 5]);
        start_log(&mut e);
        assert!(e.call(0x005b_7590, &args![script(0)]).bool());
        assert_eq!(calls(&e, SKY_FORCE_WEATHER), vec![vec![sky, 0x3333, 1]]);
        assert!(calls(&e, FN_0063E860).is_empty());
        // Bad parameters.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_7590, &args![script(0)]).bool());
        assert!(calls(&e, SKY_FORCE_WEATHER).is_empty());
        let _ = weather;
    }

    #[test]
    fn force_weather_command_forces_the_weather() {
        let mut e = engine_b();
        let (sky, _, _) = sky_world(&mut e);
        parse_gives(&mut e, true, &[0x4444, 3]);
        start_log(&mut e);
        assert!(e.call(0x005b_7660, &args![script(0)]).bool());
        assert_eq!(calls(&e, SKY_FORCE_WEATHER), vec![vec![sky, 0x4444, 1]]);
        parse_gives(&mut e, true, &[0x4444, 0]);
        start_log(&mut e);
        assert!(e.call(0x005b_7660, &args![script(0)]).bool());
        assert_eq!(calls(&e, SKY_FORCE_WEATHER), vec![vec![sky, 0x4444, 0]]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_7660, &args![script(0)]).bool());
        assert!(calls(&e, SKY_FORCE_WEATHER).is_empty());
    }

    #[test]
    fn clear_override_weather_command_calls_the_sky() {
        let mut e = engine_b();
        let (sky, _, _) = sky_world(&mut e);
        start_log(&mut e);
        assert!(e.call(0x005b_76d0, &args![]).bool());
        assert_eq!(calls(&e, SKY_CLEAR_OVERRIDE_WEATHER), vec![vec![sky]]);
    }

    #[test]
    fn image_space_modifier_commands() {
        let mut e = engine_b();
        accept(
            &mut e,
            &[IMAGE_SPACE_MODIFIER_TRIGGER, IMAGE_SPACE_MODIFIER_STOP],
        );
        // The strength defaults to 1.0 (the parse double leaves the default
        // when it stores nothing).
        e.register_double(PARSE_PARAMETERS, |e, a| {
            assert_eq!(e.mem.u32(a[8]), 1.0f32.to_bits());
            e.mem.set_u32(a[7], 0x77);
            true.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005b_76f0, &args![script(0)]).bool());
        assert_eq!(
            calls(&e, IMAGE_SPACE_MODIFIER_TRIGGER),
            vec![vec![0x77, 1.0f32.to_bits(), 0]]
        );
        assert!(e.call(0x005b_7760, &args![script(0)]).bool());
        assert_eq!(calls(&e, IMAGE_SPACE_MODIFIER_STOP), vec![vec![0x77]]);
        // A given strength is passed on.
        parse_gives(&mut e, true, &[0x88, 0.5f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005b_76f0, &args![script(0)]).bool());
        assert_eq!(
            calls(&e, IMAGE_SPACE_MODIFIER_TRIGGER),
            vec![vec![0x88, 0.5f32.to_bits(), 0]]
        );
        // Bad parameters.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_76f0, &args![script(0)]).bool());
        assert!(!e.call(0x005b_7760, &args![script(0)]).bool());
        assert!(calls(&e, IMAGE_SPACE_MODIFIER_TRIGGER).is_empty());
        assert!(calls(&e, IMAGE_SPACE_MODIFIER_STOP).is_empty());
    }

    #[test]
    fn two_argument_command_calls_005477_50_with_both() {
        let mut e = engine_b();
        accept(&mut e, &[FN_00547750]);
        parse_gives(&mut e, true, &[0x10, 0x20]);
        start_log(&mut e);
        assert!(e.call(0x005b_77c0, &args![script(0)]).bool());
        assert_eq!(calls(&e, FN_00547750), vec![vec![0x10, 0x20]]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_77c0, &args![script(0)]).bool());
        assert!(calls(&e, FN_00547750).is_empty());
    }

    #[test]
    fn image_space_override_command_sets_the_managers_list() {
        let mut e = engine_b();
        e.register(GET_IMAGE_SPACE_MANAGER, |_, _| 0x9000u32.into_ret());
        accept(&mut e, &[IMAGE_SPACE_MANAGER_SET_OVERRIDE, FN_00B8B500]);
        e.register(FORM_LIST_ITEMS, |_, a| (a[0] + 0x18).into_ret());
        parse_gives(&mut e, true, &[0x100]);
        start_log(&mut e);
        assert!(e.call(0x005b_7820, &args![script(0)]).bool());
        assert_eq!(
            calls(&e, IMAGE_SPACE_MANAGER_SET_OVERRIDE),
            vec![vec![0x9000, 0x118]]
        );
        assert_eq!(calls(&e, FN_00B8B500), vec![vec![0x9000]]);
        // No argument: the list is cleared.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005b_7820, &args![script(0)]).bool());
        assert_eq!(
            calls(&e, IMAGE_SPACE_MANAGER_SET_OVERRIDE),
            vec![vec![0x9000, 0]]
        );
        assert!(calls(&e, FORM_LIST_ITEMS).is_empty());
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_7820, &args![script(0)]).bool());
        assert!(calls(&e, FN_00B8B500).is_empty());
    }

    /// Shared records the doubles of the debug drawings write to.
    type Record = std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>;

    /// Virtual functions of the debug world: the player's position slot and
    /// the seen data's draw slot.
    const V_POSITION: u32 = 0x0900_0020;
    const V_DRAW: u32 = 0x0900_0021;
    /// Address of the node vtable (slot `0xdc` is [`V_ATTACH`]).
    const NODE_VTABLE: u32 = 0x0950_0000;
    const V_ATTACH: u32 = 0x0900_0022;

    /// The pieces of the world that both debug drawings use: the player with
    /// a position, the `TES` global, node allocation, point construction and
    /// the calls that finish a debug node. Returns `(player, position)`.
    fn debug_world(e: &mut Engine, position: [f32; 3]) -> (u32, u32) {
        let at = e.mem.alloc(12);
        for (i, v) in position.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
        e.register_double(V_POSITION, move |_, _| at.into_ret());
        let player = object_with(e, &[(PLAYER_POSITION_SLOT, V_POSITION)]);
        e.set_global(PLAYER, player);
        e.set_global(GLOBAL_0011DEA10, 0x7000u32);
        e.set_global(DEBUG_OBJECT_SECONDS, 30.0f32);
        e.set_global(DOUBLE_MINUS_ONE, -1.0f64);
        e.set_global(DOUBLE_ZERO, 0.0f64);
        e.set_global(DOUBLE_TWO, 2.0f64);
        e.set_global(DOUBLE_FOUR, 4.0f64);
        e.set_global(DOUBLE_TEN, 10.0f64);
        e.set_global(DOUBLE_4096, 4096.0f64);
        for i in 0..9u32 {
            e.set_global(
                MATRIX_011A9448 + 4 * i,
                [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0][i as usize],
            );
        }
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        e.put_vtable(NODE_VTABLE, &[0; 56]);
        e.mem.set_u32(NODE_VTABLE + 0xdc, V_ATTACH);
        e.register(V_ATTACH, |_, _| Ret::default());
        e.register(NI_NODE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], NODE_VTABLE);
            a[0].into_ret()
        });
        e.register(NI_POINT3_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        accept(
            e,
            &[
                UPDATE_DATA_CONSTRUCT,
                NODE_UPDATE_WITH_DATA,
                NODE_UPDATE_PROPERTIES,
                TES_ADD_TEMP_DEBUG_OBJECT,
                NODE_SET_LOCAL_ROTATE,
            ],
        );
        e.register(NI_MATRIX_FROM_Z_ANGLE, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        (player, at)
    }

    /// The calls that end a debug drawing were made for `node`: the update
    /// data constructed as `(0.0, 0, 0)`, the node updated and handed to the
    /// `TES` for 30 seconds.
    fn assert_debug_node_shown(e: &Engine, node: u32) {
        assert_eq!(calls(e, UPDATE_DATA_CONSTRUCT).len(), 1);
        assert_eq!(calls(e, UPDATE_DATA_CONSTRUCT)[0][1..], [0, 0, 0]);
        assert_eq!(calls(e, NODE_UPDATE_WITH_DATA)[0][0], node);
        assert_eq!(calls(e, NODE_UPDATE_PROPERTIES), vec![vec![node]]);
        assert_eq!(
            calls(e, TES_ADD_TEMP_DEBUG_OBJECT),
            vec![vec![0x7000, node, 30.0f32.to_bits()]]
        );
    }

    #[test]
    fn seen_data_drawing_in_an_interior_draws_once_and_rotates_by_the_north() {
        let mut e = engine_b();
        let (_, _) = debug_world(&mut e, [10.0, 20.0, 30.0]);
        let draws: Record = Record::default();
        let record = draws.clone();
        e.register_double(V_DRAW, move |e, a| {
            record.borrow_mut().push(vec![
                a[0],
                a[1],
                e.mem.u32(a[2]),
                e.mem.u32(a[2] + 4),
                e.mem.u32(a[2] + 8),
                a[3],
            ]);
            Ret::default()
        });
        let seen = object_with(&mut e, &[(SEEN_DATA_DRAW_SLOT, V_DRAW)]);
        e.register(REFR_GET_PARENT_CELL, |_, _| 0x8000u32.into_ret());
        e.register(CELL_IS_INTERIOR, |_, _| true.into_ret());
        e.register_double(CELL_GET_SEEN_DATA, move |_, a| {
            assert_eq!(a[0], 0x8000);
            seen.into_ret()
        });
        e.register(CELL_GET_NORTH_ROTATION, |_, _| 0.5f32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_78a0, &args![]).bool());
        let node = calls(&e, NI_NODE_CONSTRUCT)[0][0];
        // The seen data draws the node at (0, 0, z + 0 of the player).
        assert_eq!(
            *draws.borrow(),
            vec![vec![
                seen,
                node,
                0.0f32.to_bits(),
                0.0f32.to_bits(),
                30.0f32.to_bits(),
                1
            ]]
        );
        // The north rotation 0.5 times -1 turns the node.
        let matrix_calls = calls(&e, NI_MATRIX_FROM_Z_ANGLE);
        assert_eq!(matrix_calls.len(), 1);
        assert_eq!(matrix_calls[0][1], (-0.5f32).to_bits());
        let rotate = calls(&e, NODE_SET_LOCAL_ROTATE);
        assert_eq!(rotate.len(), 1);
        assert_eq!(rotate[0][0], node);
        assert_debug_node_shown(&e, node);

        // North rotation 0: the node is not rotated.
        e.register(CELL_GET_NORTH_ROTATION, |_, _| 0.0f32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_78a0, &args![]).bool());
        assert!(calls(&e, NODE_SET_LOCAL_ROTATE).is_empty());
        assert_eq!(draws.borrow().len(), 2);

        // A cell without seen data: no drawing, no rotation.
        e.register(CELL_GET_SEEN_DATA, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_78a0, &args![]).bool());
        assert_eq!(draws.borrow().len(), 2);
        assert!(calls(&e, NI_MATRIX_FROM_Z_ANGLE).is_empty());
    }

    #[test]
    fn seen_data_drawing_in_an_exterior_walks_the_grid() {
        let mut e = engine_b();
        debug_world(&mut e, [10.0, 20.0, 30.0]);
        let draws: Record = Record::default();
        let record = draws.clone();
        e.register_double(V_DRAW, move |e, a| {
            record.borrow_mut().push(vec![
                a[0],
                e.mem.u32(a[2]),
                e.mem.u32(a[2] + 4),
                e.mem.u32(a[2] + 8),
            ]);
            Ret::default()
        });
        // uGridsToLoad is 2: four grid positions; (outer 1, inner 0) has no
        // cell, (1, 1) has a cell without seen data.
        let grid = e.mem.alloc(4);
        e.mem.set_u32(grid, 2);
        e.register_double(SETTING_INT_VALUE_ADDRESS, move |_, a| {
            assert_eq!(a[0], SETTING_GRIDS_TO_LOAD);
            grid.into_ret()
        });
        // Cell objects: +0 the seen data, +4 data x, +8 data y.
        let seen_a = object_with(&mut e, &[(SEEN_DATA_DRAW_SLOT, V_DRAW)]);
        let seen_b = object_with(&mut e, &[(SEEN_DATA_DRAW_SLOT, V_DRAW)]);
        let mut cells = vec![];
        for (seen, x, y) in [(seen_a, 3, 0xffff_fffeu32), (seen_b, 5, 7), (0, 0, 0)] {
            let cell = e.mem.alloc(16);
            e.mem.set_u32(cell, seen);
            e.mem.set_u32(cell + 4, x);
            e.mem.set_u32(cell + 8, y);
            cells.push(cell);
        }
        let slots = e.mem.alloc(16);
        e.mem.set_u32(slots, cells[0]);
        e.mem.set_u32(slots + 4, cells[1]);
        e.mem.set_u32(slots + 8, 0);
        e.mem.set_u32(slots + 12, cells[2]);
        e.register_double(TES_GET_GRID_CELL_SLOT, move |_, a| {
            assert_eq!(a[0], 0x7000);
            (slots + 8 * a[1] + 4 * a[2]).into_ret()
        });
        e.register(REFR_GET_PARENT_CELL, |_, _| 0x8000u32.into_ret());
        e.register(CELL_IS_INTERIOR, |_, _| false.into_ret());
        e.register(CELL_GET_SEEN_DATA, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CELL_GET_DATA_X, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(CELL_GET_DATA_Y, |e, a| e.mem.u32(a[0] + 8).into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_78a0, &args![]).bool());
        let node = calls(&e, NI_NODE_CONSTRUCT)[0][0];
        // Cell (0, 0): x 3 << 12, y -2 << 12, z of the player; cell (0, 1):
        // 5 << 12, 7 << 12. The setting is read for every loop test.
        assert_eq!(
            *draws.borrow(),
            vec![
                vec![
                    seen_a,
                    (3.0f32 * 4096.0).to_bits(),
                    (-2.0f32 * 4096.0).to_bits(),
                    30.0f32.to_bits()
                ],
                vec![
                    seen_b,
                    (5.0f32 * 4096.0).to_bits(),
                    (7.0f32 * 4096.0).to_bits(),
                    30.0f32.to_bits()
                ],
            ]
        );
        assert!(calls(&e, NODE_SET_LOCAL_ROTATE).is_empty());
        assert_debug_node_shown(&e, node);
    }

    #[test]
    fn texture_map_is_created_once_and_given_the_texture() {
        let mut e = engine_b();
        // The property's array: element 0 is at `list + 0x40`.
        let property = e.mem.alloc(0x100);
        e.register(ARRAY_ELEMENT_ADDRESS, |_, a| {
            assert_eq!(a[1], 0);
            (a[0] + 0x40).into_ret()
        });
        e.register(OPERATOR_NEW, |e, a| {
            assert_eq!(a[0], 0x10);
            e.mem.alloc(0x10).into_ret()
        });
        e.register(TEXTURE_MAP_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(ARRAY_SET_ELEMENT, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x40 + 4 * a[1], value);
            Ret::default()
        });
        accept(&mut e, &[TEXTURE_MAP_SET_TEXTURE]);
        start_log(&mut e);
        e.call(0x005b_8fc0, &args![Ptr::<()>::new(property), 0x1234u32]);
        let map = e.mem.u32(property + 0x1c + 0x40);
        assert_ne!(map, 0);
        assert_eq!(calls(&e, TEXTURE_MAP_CONSTRUCT), vec![vec![map]]);
        assert_eq!(calls(&e, TEXTURE_MAP_SET_TEXTURE), vec![vec![map, 0x1234]]);
        // The map exists now: it is reused.
        start_log(&mut e);
        e.call(0x005b_8fc0, &args![Ptr::<()>::new(property), 0x5678u32]);
        assert!(calls(&e, TEXTURE_MAP_CONSTRUCT).is_empty());
        assert!(calls(&e, ARRAY_SET_ELEMENT).is_empty());
        assert_eq!(calls(&e, TEXTURE_MAP_SET_TEXTURE), vec![vec![map, 0x5678]]);
    }

    #[test]
    fn test_code_prints_the_loaded_area_in_megabytes() {
        let mut e = engine_b();
        e.set_global(DOUBLE_MEGABYTE, 1048576.0f64);
        e.register(FN_004A8BB0, |e, a| {
            assert_eq!(a[2..], [1, 0]);
            e.mem.set_f32(a[1], 3.0 * 1048576.0 + 524288.0);
            Ret::default()
        });
        accept(&mut e, &[CONSOLE_PRINT]);
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005b_9070, &args![script(0)]).bool());
        let mut words = vec![MSG_LOADED_AREA_MEGABYTES];
        words.extend(args![3.5f64]);
        assert_eq!(printed(&e), vec![words]);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_9070, &args![script(0)]).bool());
        assert!(printed(&e).is_empty());
    }

    #[test]
    fn stats_flags_accessors_and_toggles() {
        let mut e = engine_b();
        for (set, get, toggle, flag, message) in [
            (
                0x005b_9150,
                0x005b_9160,
                0x005b_9100,
                CONVERSATION_STATS_FLAG,
                MSG_CONVERSATION_STATS,
            ),
            (
                0x005b_9200,
                0x005b_9210,
                0x005b_91b0,
                MAGIC_STATS_FLAG,
                MSG_MAGIC_STATS,
            ),
        ] {
            assert_eq!(e.call(get, &args![]).u8(), 0);
            e.call(set, &args![1u32]);
            assert_eq!(e.call(get, &args![]).u8(), 1);
            assert_eq!(e.global::<u8>(flag), 1);
            assert_eq!(
                run_and_print(&mut e, toggle),
                vec![vec![message, TEXT_HIDDEN]]
            );
            assert_eq!(e.global::<u8>(flag), 0);
            assert_eq!(
                run_and_print(&mut e, toggle),
                vec![vec![message, TEXT_SHOWN]]
            );
            assert_eq!(e.global::<u8>(flag), 1);
        }
    }

    #[test]
    fn full_help_toggle_prints_the_new_state() {
        let mut e = engine_b();
        // The help flag is a byte of the double.
        e.register(INTERFACE_TOGGLE_FULL_HELP, |e, _| {
            let flag = e.mem.u8(0x011f_9100);
            e.mem.set_u8(0x011f_9100, (flag == 0) as u8);
            Ret::default()
        });
        e.register(INTERFACE_GET_FULL_HELP, |e, _| {
            (e.mem.u8(0x011f_9100) != 0).into_ret()
        });
        assert_eq!(
            run_and_print(&mut e, 0x005b_9170),
            vec![vec![MSG_TOGGLE_FULL_HELP, TEXT_SHOWN]]
        );
        assert_eq!(
            run_and_print(&mut e, 0x005b_9170),
            vec![vec![MSG_TOGGLE_FULL_HELP, TEXT_HIDDEN]]
        );
    }

    #[test]
    fn tes_commands_call_their_function_on_the_tes() {
        let mut e = engine_b();
        e.set_global(GLOBAL_0011DEA10, 0x7000u32);
        accept(&mut e, &[FN_00456AA0, INTERFACE_TOGGLE_SAFE_ZONE]);
        // `005b9220` plain; `005b9240` with the byte set while it runs.
        e.register(FN_00456A60, |e, a| {
            assert_eq!(a, [0x7000]);
            let during = e.mem.u8(FLAG_012680FC);
            e.mem.set_u8(0x0126_8100, during);
            Ret::default()
        });
        start_log(&mut e);
        assert!(e.call(0x005b_9220, &args![]).bool());
        assert_eq!(e.mem.u8(0x0126_8100), 0);
        assert!(e.call(0x005b_9240, &args![]).bool());
        assert_eq!(e.mem.u8(0x0126_8100), 1);
        assert_eq!(e.global::<u8>(FLAG_012680FC), 0);
        assert_eq!(calls(&e, FN_00456A60).len(), 2);
        assert!(e.call(0x005b_9500, &args![]).bool());
        assert_eq!(calls(&e, FN_00456AA0), vec![vec![0x7000]]);
        assert!(e.call(0x005b_9520, &args![]).bool());
        assert_eq!(calls(&e, INTERFACE_TOGGLE_SAFE_ZONE), vec![vec![2]]);
    }

    #[test]
    fn char_controller_shape_switches_between_zero_and_one() {
        let mut e = engine_b();
        e.register(DYNAMIC_CAST, |_, a| {
            assert_eq!(a[1..], [0, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0]);
            (if a[0] == 0x66 { 0u32 } else { a[0] + 1 }).into_ret()
        });
        let controller = e.mem.alloc(0x600);
        e.register_double(MOBILE_OBJECT_GET_CHAR_CONTROLLER, move |_, a| {
            (if a[0] == 0x41 { controller } else { 0 }).into_ret()
        });
        accept(&mut e, &[CHAR_CONTROLLER_SET_SHAPE_TYPE]);
        for (before, after) in [(0u32, 1u32), (1, 0), (2, 2)] {
            e.mem.set_u32(controller + 0x59c, before);
            start_log(&mut e);
            assert!(e.call(0x005b_9260, &args![script(0x40)]).bool());
            assert_eq!(
                calls(&e, CHAR_CONTROLLER_SET_SHAPE_TYPE),
                vec![vec![controller, after]]
            );
        }
        assert_eq!(
            e.call(0x005b_92e0, &args![Ptr::<()>::new(controller)])
                .u32(),
            2
        );
        // No reference, a reference that is no actor, an actor without a
        // controller: nothing changes.
        for this_obj in [0, 0x66, 0x50] {
            start_log(&mut e);
            assert!(e.call(0x005b_9260, &args![script(this_obj)]).bool());
            assert!(calls(&e, CHAR_CONTROLLER_SET_SHAPE_TYPE).is_empty());
        }
    }

    #[test]
    fn string_command_hands_its_string_to_the_tes() {
        let mut e = engine_b();
        e.set_global(GLOBAL_0011DEA10, 0x7000u32);
        e.register(FN_00456B10, |e, a| {
            assert_eq!(a[0], 0x7000);
            assert_eq!(e.mem.cstr(a[1]), b"mapname".to_vec());
            Ret::default()
        });
        e.register_double(PARSE_PARAMETERS, |e, a| {
            e.mem.set_cstr(a[7], b"mapname");
            true.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005b_9300, &args![script(0)]).bool());
        assert_eq!(calls(&e, FN_00456B10).len(), 1);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_9300, &args![script(0)]).bool());
        assert!(calls(&e, FN_00456B10).is_empty());
    }

    #[test]
    fn borders_toggle_flips_the_flag_and_tells_the_tes() {
        let mut e = engine_b();
        e.set_global(GLOBAL_0011DEA10, 0x7000u32);
        accept(&mut e, &[TES_SET_SHOW_LAND_BORDERS, CONSOLE_PRINT]);
        e.register(FN_005F36F0, |e, _| e.mem.u32(0x0126_8100).into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_9370, &args![]).bool());
        assert_eq!(e.global::<u8>(BORDERS_FLAG), 1);
        assert_eq!(calls(&e, TES_SET_SHOW_LAND_BORDERS), vec![vec![0x7000, 1]]);
        // The echo flag is off: no print.
        assert!(printed(&e).is_empty());
        // `005f36f0` non-zero: the TES is left alone; with the echo flag the
        // state is printed.
        e.mem.set_u32(0x0126_8100, 5);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005b_9370, &args![]).bool());
        assert_eq!(e.global::<u8>(BORDERS_FLAG), 0);
        assert!(calls(&e, TES_SET_SHOW_LAND_BORDERS).is_empty());
        assert_eq!(printed(&e), vec![vec![MSG_BORDERS, TEXT_OFF_CAPITAL]]);
        start_log(&mut e);
        assert!(e.call(0x005b_9370, &args![]).bool());
        assert_eq!(printed(&e), vec![vec![MSG_BORDERS, TEXT_ON_CAPITAL]]);
    }

    #[test]
    fn projectile_debug_toggle_explains_the_shapes_when_echoing() {
        let mut e = engine_b();
        accept(&mut e, &[CONSOLE_PRINT]);
        start_log(&mut e);
        assert!(e.call(0x005b_9400, &args![]).bool());
        assert_eq!(e.global::<u8>(PROJECTILE_DEBUG_FLAG), 1);
        assert!(printed(&e).is_empty());
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005b_9400, &args![]).bool());
        assert_eq!(e.global::<u8>(PROJECTILE_DEBUG_FLAG), 0);
        assert_eq!(
            printed(&e),
            vec![
                vec![MSG_PROJECTILE_DEBUG_1],
                vec![MSG_PROJECTILE_DEBUG_2],
                vec![MSG_PROJECTILE_DEBUG_3],
                vec![MSG_PROJECTILE_DEBUG_4],
                vec![MSG_PROJECTILE_DEBUG_5],
            ]
        );
        // The flip alone.
        e.mem.set_u8(PROJECTILE_DEBUG_FLAG, 7);
        e.call(0x005b_9470, &args![]);
        assert_eq!(e.global::<u8>(PROJECTILE_DEBUG_FLAG), 0);
        e.call(0x005b_9470, &args![]);
        assert_eq!(e.global::<u8>(PROJECTILE_DEBUG_FLAG), 1);
    }

    #[test]
    fn menus_toggle_passes_the_flag_on() {
        let mut e = engine_b();
        accept(&mut e, &[FN_00703810, CONSOLE_PRINT]);
        start_log(&mut e);
        assert!(e.call(0x005b_9490, &args![]).bool());
        assert_eq!(e.global::<u8>(MENUS_FLAG), 1);
        assert_eq!(calls(&e, FN_00703810), vec![vec![1]]);
        assert!(printed(&e).is_empty());
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005b_9490, &args![]).bool());
        assert_eq!(calls(&e, FN_00703810), vec![vec![0]]);
        assert_eq!(printed(&e), vec![vec![MSG_MENUS, TEXT_OFF_CAPITAL]]);
        start_log(&mut e);
        e.call(0x005b_9490, &args![]);
        assert_eq!(printed(&e), vec![vec![MSG_MENUS, TEXT_ON_CAPITAL]]);
    }

    /// The words at `addr`.
    fn words_at(e: &Engine, addr: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(addr + 4 * i)).collect()
    }

    /// The records `TestLocalMap`'s doubles keep.
    struct LocalMapRecords {
        /// `NiTriShape::NiTriShape` arguments.
        shapes: Record,
        /// `(shape, x, y, z)` of the tile moves.
        tile_moves: Record,
        /// `(target, x, y, z)` of the `NODE_SET_LOCAL_TRANSLATE` calls.
        node_moves: Record,
        /// The points `CELL_GET_SEEN_VALUE` was asked about.
        seen_points: Record,
        /// The words `MAKE_TRIANGLE` got.
        triangles: Record,
    }

    /// The world of the `TestLocalMap` tests: an exterior with the player at
    /// `position`, `grid` tiles per side, a world space whose cells have a
    /// local map texture and doubles that record what the command builds.
    fn local_map_world(e: &mut Engine, position: [f32; 3], grid: u32) -> LocalMapRecords {
        let (_, _) = debug_world(e, position);
        let grid_value = e.mem.alloc(4);
        e.mem.set_u32(grid_value, grid);
        e.register_double(SETTING_INT_VALUE_ADDRESS, move |_, _| grid_value.into_ret());
        e.register(GET_LOCAL_MAP_SCALE, |_, _| 2.0f32.into_ret());
        e.register(REFR_GET_INTERIOR, |_, _| false.into_ret());
        e.register(REFR_GET_WORLD_SPACE, |_, _| 0x3000u32.into_ret());
        e.register(REFR_GET_PARENT_CELL, |_, _| 0x8000u32.into_ret());
        e.register(WORLD_SPACE_GET_CELL_FROM_CELL_COORD, |_, _| {
            0x3100u32.into_ret()
        });
        e.register(CELL_GET_SEEN_DATA, |_, _| 0u32.into_ret());
        e.register(FN_0054E640, |e, a| {
            e.mem.set_u32(a[1], 0x5555);
            Ret::default()
        });
        e.register(FLOAT_TO_INT, |_, a| {
            (f32::from_bits(a[0]).round_ties_even() as i32 as u32).into_ret()
        });
        e.register(NI_POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        e.register(NI_COLOR_A_CONSTRUCT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        accept(e, &[VECTOR_CONSTRUCT_SIMPLE]);
        e.register(NI_ALLOC_SHORTS, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(TRIVIAL_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(NI_POINTER_INIT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        accept(
            e,
            &[
                NI_POINTER_DESTRUCT,
                TEXTURE_MAP_SET_TEXTURE,
                NI_TEXTURING_PROPERTY_SET_BASE_CLAMP_MODE,
                FN_00533FB0,
                NI_AV_OBJECT_ATTACH_PROPERTY,
            ],
        );
        e.register(NI_TEXTURING_PROPERTY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(ARRAY_ELEMENT_ADDRESS, |_, a| (a[0] + 4 * a[1]).into_ret());
        e.register(TEXTURE_MAP_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(ARRAY_SET_ELEMENT, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 4 * a[1], value);
            Ret::default()
        });
        e.register(POINT3_ADD_ASSIGN, |e, a| {
            for i in 0..3 {
                let sum = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, sum);
            }
            Ret::default()
        });
        e.register(POINT3_SUBTRACT, |e, a| {
            for i in 0..3 {
                let difference = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, difference);
            }
            a[1].into_ret()
        });
        e.register(CELL_ADJUST_COORD_FOR_NORTH_ROTATION, |e, a| {
            assert_eq!(a[3], 1);
            for i in 0..3 {
                let shift = [100.0f32, 200.0, 0.0][i as usize];
                let value = e.mem.f32(a[1] + 4 * i) + shift;
                e.mem.set_f32(a[2] + 4 * i, value);
            }
            Ret::default()
        });
        e.register(CELL_GET_NORTH_ROTATION, |_, _| 0.25f32.into_ret());
        let data = e.mem.alloc(16);
        e.mem.set_f32(data + 8, 1.5);
        e.register_double(FN_00430830, move |_, _| data.into_ret());
        for (addr, value) in [
            (MARKER_TOP_Y, 1.5f32),
            (MARKER_RIGHT_X, 0.5),
            (MARKER_BOTTOM, -0.5),
        ] {
            e.set_global(addr, value);
        }
        for (i, value) in [0.1f32, 0.2, 0.3, 0.4].iter().enumerate() {
            e.set_global(DEFAULT_TILE_COLOUR + 4 * i as u32, *value);
        }
        let records = LocalMapRecords {
            shapes: Record::default(),
            tile_moves: Record::default(),
            node_moves: Record::default(),
            seen_points: Record::default(),
            triangles: Record::default(),
        };
        let record = records.shapes.clone();
        e.register_double(NI_TRI_SHAPE_CONSTRUCT, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            a[0].into_ret()
        });
        let record = records.tile_moves.clone();
        e.register_double(NODE_SET_LOCAL_TRANSLATE_XYZ, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let record = records.node_moves.clone();
        e.register_double(NODE_SET_LOCAL_TRANSLATE, move |e, a| {
            record.borrow_mut().push(vec![
                a[0],
                e.mem.u32(a[1]),
                e.mem.u32(a[1] + 4),
                e.mem.u32(a[1] + 8),
            ]);
            Ret::default()
        });
        let record = records.seen_points.clone();
        e.register_double(CELL_GET_SEEN_VALUE, move |e, a| {
            record.borrow_mut().push(vec![
                e.mem.u32(a[0]),
                e.mem.u32(a[0] + 4),
                e.mem.u32(a[0] + 8),
            ]);
            8u32.into_ret()
        });
        let record = records.triangles.clone();
        e.register_double(MAKE_TRIANGLE, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            0x6000u32.into_ret()
        });
        records
    }

    #[test]
    fn test_local_map_builds_one_tile_and_the_marker_in_an_exterior() {
        let mut e = engine_b();
        let records = local_map_world(&mut e, [5000.0, 9000.0, 50.0], 1);
        parse_gives(&mut e, true, &[0]);
        e.register_double(WORLD_SPACE_GET_CELL_FROM_CELL_COORD, |_, a| {
            // The tile's cell: the player's cell (5000 >> 12, 9000 >> 12).
            assert_eq!(a, [0x3000, 1, 2]);
            0x3100u32.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005b_7b40, &args![script(0)]).bool());
        let root = calls(&e, NI_NODE_CONSTRUCT)[0][0];

        // Four element arrays and the index array were allocated.
        let constructed = calls(&e, VECTOR_CONSTRUCT_SIMPLE);
        assert_eq!(constructed.len(), 4);
        for (call, (size, constructor)) in constructed.iter().zip([
            (12, TRIVIAL_CONSTRUCT),
            (12, TRIVIAL_CONSTRUCT),
            (8, TRIVIAL_CONSTRUCT),
            (16, NI_COLOR_A_ZERO_CONSTRUCT),
        ]) {
            assert_eq!(call[1..], [size, 289, constructor]);
        }
        let shapes = records.shapes.borrow();
        assert_eq!(shapes.len(), 1);
        let shape = &shapes[0];
        let (positions, normals, colours, uvs, indices) = (
            constructed[0][0],
            constructed[1][0],
            constructed[3][0],
            constructed[2][0],
            shape[9],
        );
        assert_eq!(
            shape[1..],
            [289, positions, normals, colours, uvs, 1, 0, 512, indices]
        );

        // Positions: x = column * 4 - 32, y = row * 4 - 32, z = 0.
        let point = |i: u32| {
            words_at(&e, positions + 12 * i, 3)
                .iter()
                .map(|w| f32::from_bits(*w))
                .collect::<Vec<_>>()
        };
        assert_eq!(point(0), [-32.0, -32.0, 0.0]);
        assert_eq!(point(1), [-28.0, -32.0, 0.0]);
        assert_eq!(point(17), [-32.0, -28.0, 0.0]);
        assert_eq!(point(288), [32.0, 32.0, 0.0]);
        // Normals point up, the uvs follow the grid, the colours are the default.
        assert_eq!(
            words_at(&e, normals + 12 * 100, 3),
            [0, 0, 1.0f32.to_bits()]
        );
        assert_eq!(
            words_at(&e, uvs + 8, 2),
            [((1.0f64 / 17.0) as f32).to_bits(), 1.0f32.to_bits()]
        );
        assert_eq!(
            words_at(&e, uvs + 8 * 17, 2),
            [0, ((1.0f64 - 1.0 / 17.0) as f32).to_bits()]
        );
        assert_eq!(
            words_at(&e, colours + 16 * 288, 4),
            [0.1f32, 0.2, 0.3, 0.4].map(f32::to_bits)
        );
        // Indices: the diagonal alternates between neighbouring quads.
        let index = |k: u32| e.mem.u16(indices + 2 * k);
        assert_eq!(
            (0..12).map(index).collect::<Vec<_>>(),
            [18, 17, 0, 0, 1, 18, 18, 1, 2, 2, 19, 18]
        );
        assert_eq!(index(1535), 288);
        // The shape hangs on the root at the tile's place, 0 for one tile.
        let attached = calls(&e, V_ATTACH);
        assert_eq!(attached[0], vec![root, shape[0], 1]);
        assert_eq!(
            *records.tile_moves.borrow(),
            vec![vec![
                shape[0],
                0.0f32.to_bits(),
                0.0f32.to_bits(),
                0.0f32.to_bits()
            ]]
        );
        // The cell's texture became the base map of a new property.
        let property = calls(&e, NI_TEXTURING_PROPERTY_CONSTRUCT)[0][0];
        let map = e.mem.u32(property + 0x1c);
        assert_eq!(calls(&e, TEXTURE_MAP_SET_TEXTURE), vec![vec![map, 0x5555]]);
        assert_eq!(
            calls(&e, NI_AV_OBJECT_ATTACH_PROPERTY),
            vec![vec![shape[0], property]]
        );
        assert_eq!(calls(&e, NI_POINTER_SET).len(), 1);
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT).len(), 1);

        // The marker: a red triangle at the player's offset from the centre of
        // the player's cell, turned by the north rotation + 1.5.
        let triangles = records.triangles.borrow();
        assert_eq!(triangles.len(), 1);
        let float_words = |values: [f32; 9]| values.map(f32::to_bits);
        assert_eq!(
            triangles[0][..9],
            float_words([0.0, 1.5, 1.0, 0.5, -0.5, 1.0, -0.5, -0.5, 1.0])
        );
        assert_eq!(triangles[0][10], 1);
        let colour = triangles[0][9];
        assert_eq!(
            calls(&e, NI_COLOR_A_CONSTRUCT)[0],
            vec![colour, 1.0f32.to_bits(), 0, 0, 0]
        );
        let moves = records.node_moves.borrow();
        assert_eq!(
            moves[0],
            vec![
                0x6000,
                (-17.875f32).to_bits(),
                (-19.375f32).to_bits(),
                0.0f32.to_bits()
            ]
        );
        assert_eq!(attached[1], vec![root, 0x6000, 1]);
        assert_eq!(calls(&e, NI_MATRIX_FROM_Z_ANGLE)[0][1], 1.75f32.to_bits());
        // The root goes to the player's position, z raised by 10.
        assert_eq!(
            moves[1],
            vec![
                root,
                5000.0f32.to_bits(),
                9000.0f32.to_bits(),
                60.0f32.to_bits()
            ]
        );
        assert_debug_node_shown(&e, root);
    }

    #[test]
    fn test_local_map_colours_the_vertices_by_the_seen_value() {
        let mut e = engine_b();
        let records = local_map_world(&mut e, [5000.0, 9000.0, 50.0], 1);
        parse_gives(&mut e, true, &[1]);
        start_log(&mut e);
        assert!(e.call(0x005b_7b40, &args![script(0)]).bool());
        let constructed = calls(&e, VECTOR_CONSTRUCT_SIMPLE);
        let colours = constructed[3][0];
        // 17 x 17 samples, column * 2 (the scale), row * 2, 0 plus the tile
        // origin (1 << 12, 2 << 12, 0).
        let seen = records.seen_points.borrow();
        assert_eq!(seen.len(), 289);
        let sample = |x: f32, y: f32| vec![x.to_bits(), y.to_bits(), 0.0f32.to_bits()];
        assert_eq!(seen[0], sample(4096.0, 8192.0));
        assert_eq!(seen[1], sample(4098.0, 8192.0));
        assert_eq!(seen[17], sample(4096.0, 8194.0));
        assert_eq!(seen[288], sample(4128.0, 8224.0));
        // A seen value of 8 divided by 4 is the colour (2, 2, 2, 0).
        for vertex in [0, 100, 288] {
            assert_eq!(
                words_at(&e, colours + 16 * vertex, 4),
                [2.0f32, 2.0, 2.0, 0.0].map(f32::to_bits)
            );
        }
    }

    #[test]
    fn test_local_map_in_an_interior_uses_the_adjusted_position() {
        let mut e = engine_b();
        let records = local_map_world(&mut e, [10000.0, 20000.0, 5.0], 1);
        e.register(REFR_GET_INTERIOR, |_, _| true.into_ret());
        accept(
            &mut e,
            &[
                CELL_GET_INT_SEEN_SECTION,
                CELL_GET_INTERIOR_LOCAL_MAP_TEXTURE,
            ],
        );
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005b_7b40, &args![script(0)]).bool());
        // The adjusted position is (10100, 20200): cell ((x - 0x800) >> 12,
        // ...) = (1, 4); the seen section and the texture are asked for it.
        let slot = calls(&e, CELL_GET_INTERIOR_LOCAL_MAP_TEXTURE)[0][3];
        assert_eq!(
            calls(&e, CELL_GET_INT_SEEN_SECTION),
            vec![vec![0x8000, 1, 4, 0]]
        );
        assert_eq!(
            calls(&e, CELL_GET_INTERIOR_LOCAL_MAP_TEXTURE),
            vec![vec![0x8000, 1, 4, slot]]
        );
        assert!(calls(&e, WORLD_SPACE_GET_CELL_FROM_CELL_COORD).is_empty());
        // The marker's centre is the middle of the cell (1, 4) plus one cell:
        // (8192, 20480); the player's offset from it is in tile units.
        let moves = records.node_moves.borrow();
        assert_eq!(
            moves[0],
            vec![
                0x6000,
                29.8125f32.to_bits(),
                (-4.375f32).to_bits(),
                0.0f32.to_bits()
            ]
        );
        // The root goes to the unadjusted position.
        assert_eq!(
            moves[1][1..],
            [
                10000.0f32.to_bits(),
                20000.0f32.to_bits(),
                15.0f32.to_bits()
            ]
        );
        drop(moves);
        // Without a parent cell the tile asks for nothing.
        e.register(REFR_GET_PARENT_CELL, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_7b40, &args![script(0)]).bool());
        assert!(calls(&e, CELL_GET_INT_SEEN_SECTION).is_empty());
    }

    #[test]
    fn test_local_map_covers_a_grid_of_tiles() {
        let mut e = engine_b();
        let records = local_map_world(&mut e, [5000.0, 9000.0, 50.0], 2);
        parse_gives(&mut e, true, &[0]);
        let cells = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let record = cells.clone();
        e.register_double(WORLD_SPACE_GET_CELL_FROM_CELL_COORD, move |_, a| {
            record.borrow_mut().push((a[1], a[2]));
            0x3100u32.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005b_7b40, &args![script(0)]).bool());
        // 2 x 2 tiles around the player's cell (1, 2): cells (0..2, 1..3).
        assert_eq!(*cells.borrow(), vec![(0, 1), (1, 1), (0, 2), (1, 2)]);
        // The tiles are 64 wide and centred on the whole 128 wide map.
        let moves = records.tile_moves.borrow();
        let xy: Vec<(f32, f32)> = moves
            .iter()
            .map(|m| (f32::from_bits(m[1]), f32::from_bits(m[2])))
            .collect();
        assert_eq!(
            xy,
            [(-32.0, -32.0), (32.0, -32.0), (-32.0, 32.0), (32.0, 32.0)]
        );
        assert_eq!(records.shapes.borrow().len(), 4);
        // Bad parameters: nothing is built.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_7b40, &args![script(0)]).bool());
        assert!(calls(&e, NI_ALLOC).is_empty());
    }

    // ---- Third batch (`005b9540` onward) ------------------------------------

    /// `engine_b` plus the pages of the third batch's globals.
    fn engine_c() -> Engine {
        let mut e = engine_b();
        for page in [0x0102_4000, 0x0126_7000] {
            e.map(page, 0x1000);
        }
        e
    }

    /// The two words of a `double` argument.
    fn f64_words(value: f64) -> [u32; 2] {
        let bits = value.to_bits();
        [bits as u32, (bits >> 32) as u32]
    }

    /// `FLOAT_MIN` and `FLOAT_MAX` behave like the exe's functions.
    fn float_doubles(e: &mut Engine) {
        e.register(FLOAT_MIN, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            (if y <= x { y } else { x }).into_ret()
        });
        e.register(FLOAT_MAX, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            (if y < x { x } else { y }).into_ret()
        });
    }

    /// `__RTDynamicCast` double: an object is an actor when the byte at
    /// `+0xf0` is set.
    fn cast_double(e: &mut Engine) {
        e.register(DYNAMIC_CAST, |e, a| {
            assert_eq!(a[1..], [0, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0]);
            (if e.mem.u8(a[0] + 0xf0) != 0 { a[0] } else { 0 }).into_ret()
        });
    }

    /// An object (0x100 bytes) with the given virtual slots, an actor when
    /// `actor` is set.
    fn reference(e: &mut Engine, slots: &[(u32, u32)], actor: bool) -> u32 {
        let object = object_with(e, slots);
        e.mem.set_u8(object + 0xf0, actor as u8);
        object
    }

    /// The doubles that make [`fn_005b9b00`] answer `allowed`.
    fn refraction_gate(e: &mut Engine, allowed: bool) {
        e.register(FN_004DC0A0, |e, _| e.mem.u8(0x011f_9180).into_ret());
        e.register(FN_004DC060, |e, a| {
            e.mem
                .u32(if a[0] != 0 { 0x011f_91bc } else { 0x011f_91c0 })
                .into_ret()
        });
        e.mem.set_u8(0x011f_9180, allowed as u8);
        e.mem.set_u32(0x011f_91c0, 2);
    }

    #[test]
    fn sky_toggle_picks_the_mode_from_the_interior_cell() {
        let mut e = engine_c();
        e.set_global(GLOBAL_0011DEA10, 0x7000u32);
        e.register(TES_GET_SKY, |_, a| {
            assert_eq!(a, [0x7000]);
            0x7100u32.into_ret()
        });
        accept(&mut e, &[SKY_SET_MODE, CONSOLE_PRINT]);
        e.register(FN_005F36F0, |e, _| e.mem.u32(0x0126_8100).into_ret());
        e.register(CELL_TEST_FLAG_80, |e, a| {
            (e.mem.u8(a[0] + 0x24) & 0x80 != 0).into_ret()
        });
        let cell = e.mem.alloc(0x40);
        // The flag becomes set: mode 0.
        start_log(&mut e);
        assert!(e.call(0x005b_9540, &args![]).bool());
        assert_eq!(e.global::<u8>(SKY_FLAG), 1);
        assert_eq!(calls(&e, SKY_SET_MODE), vec![vec![0x7100, 0]]);
        assert!(printed(&e).is_empty());
        // The flag becomes clear: no interior cell is mode 3, an interior
        // cell with bit 0x80 mode 2, without it mode 1.
        for (interior, bits, mode) in [(0, 0u8, 3u32), (cell, 0x80, 2), (cell, 0x7f, 1)] {
            e.set_global(SKY_FLAG, 1u8);
            e.mem.set_u32(0x0126_8100, interior);
            e.mem.set_u8(cell + 0x24, bits);
            start_log(&mut e);
            assert!(e.call(0x005b_9540, &args![]).bool());
            assert_eq!(e.global::<u8>(SKY_FLAG), 0);
            assert_eq!(calls(&e, SKY_SET_MODE), vec![vec![0x7100, mode]]);
        }
        // With the echo flag the new state is printed; without a TES the sky
        // is left alone.
        set_echo(&mut e, true);
        e.set_global(GLOBAL_0011DEA10, 0u32);
        start_log(&mut e);
        assert!(e.call(0x005b_9540, &args![]).bool());
        assert!(calls(&e, SKY_SET_MODE).is_empty());
        assert_eq!(printed(&e), vec![vec![MSG_SKY, TEXT_OFF_CAPITAL]]);
        assert!(e.call(0x005b_9540, &args![]).bool());
        assert_eq!(printed(&e).last().unwrap(), &vec![MSG_SKY, TEXT_ON_CAPITAL]);
    }

    #[test]
    fn collision_toggle_inverts_the_flag_of_a_reference_or_asks_the_tes() {
        let mut e = engine_c();
        e.register(FORM_TEST_FLAG_10, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x10 != 0).into_ret()
        });
        e.register(FORM_SET_FLAG_10, |e, a| {
            let flags = e.mem.u32(a[0] + 8);
            let flags = if a[1] as u8 != 0 {
                flags | 0x10
            } else {
                flags & !0x10
            };
            e.mem.set_u32(a[0] + 8, flags);
            Ret::default()
        });
        e.register(REFR_GET_NAME_TEXT, |_, _| 0x5555u32.into_ret());
        accept(&mut e, &[CONSOLE_PRINT, FN_00456BE0]);
        let form = e.mem.alloc(0x40);
        e.mem.set_u32(form + 8, 0x3);
        start_log(&mut e);
        assert!(e.call(0x005b_9640, &args![script(form)]).bool());
        assert_eq!(e.mem.u32(form + 8), 0x13);
        assert!(printed(&e).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005b_9640, &args![script(form)]).bool());
        assert_eq!(e.mem.u32(form + 8), 0x3);
        assert_eq!(
            printed(&e),
            vec![vec![MSG_REF_COLLISION, 0x5555, TEXT_ON_CAPITAL]]
        );
        assert!(e.call(0x005b_9640, &args![script(form)]).bool());
        assert_eq!(
            printed(&e).last().unwrap(),
            &vec![MSG_REF_COLLISION, 0x5555, TEXT_OFF_CAPITAL]
        );
        // No reference: the TES does it, the global flag is reported.
        e.set_global(GLOBAL_0011DEA10, 0x7000u32);
        e.register(FN_00456C70, |e, _| e.mem.u8(0x011c_3c0d).into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_9640, &args![script(0)]).bool());
        assert_eq!(calls(&e, FN_00456BE0), vec![vec![0x7000]]);
        assert_eq!(printed(&e), vec![vec![MSG_COLLISION, TEXT_ON_CAPITAL]]);
        e.mem.set_u8(0x011c_3c0d, 1);
        start_log(&mut e);
        assert!(e.call(0x005b_9640, &args![script(0)]).bool());
        assert_eq!(printed(&e), vec![vec![MSG_COLLISION, TEXT_OFF_CAPITAL]]);
    }

    #[test]
    fn leaves_flag_accessors_and_toggle() {
        let mut e = engine_c();
        let manager = e.mem.alloc(0x40);
        e.register_double(FN_00664840, move |_, a| {
            assert_eq!(a, [1]);
            manager.into_ret()
        });
        e.register(FN_0054EE20, |e, _| e.mem.u8(0x0126_8104).into_ret());
        accept(&mut e, &[CONSOLE_PRINT]);
        // `AreLeavesVisible`: needs both `0054ee20` and the manager byte.
        for (gate, byte, visible) in [(0u8, 1u8, false), (1, 0, false), (1, 1, true)] {
            e.mem.set_u8(0x0126_8104, gate);
            e.mem.set_u8(manager + 0x19, byte);
            assert_eq!(e.call(0x005b_9790, &args![]).bool(), visible);
        }
        e.call(0x005b_97d0, &args![0x35u32]);
        assert_eq!(e.mem.u8(manager + 0x19), 0x35);
        // The toggle writes the opposite of the visibility it read.
        e.mem.set_u8(0x0126_8104, 1);
        e.mem.set_u8(manager + 0x19, 1);
        start_log(&mut e);
        assert!(e.call(0x005b_9720, &args![]).bool());
        assert_eq!(e.mem.u8(manager + 0x19), 0);
        assert!(printed(&e).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005b_9720, &args![]).bool());
        assert_eq!(e.mem.u8(manager + 0x19), 1);
        assert_eq!(printed(&e), vec![vec![MSG_LEAVES, TEXT_ON_CAPITAL]]);
        e.mem.set_u8(0x0126_8104, 0);
        start_log(&mut e);
        assert!(e.call(0x005b_9720, &args![]).bool());
        assert_eq!(e.mem.u8(manager + 0x19), 1);
        assert_eq!(printed(&e), vec![vec![MSG_LEAVES, TEXT_OFF_CAPITAL]]);
    }

    #[test]
    fn wireframe_toggle_flips_the_flag_of_the_renderer_object() {
        let mut e = engine_c();
        let target = e.mem.alloc(0x200);
        e.register_double(FN_00450B80, move |_, a| {
            assert_eq!(a, [0]);
            target.into_ret()
        });
        accept(&mut e, &[FN_0045C670, CONSOLE_PRINT]);
        start_log(&mut e);
        assert!(e.call(0x005b_97f0, &args![]).bool());
        assert_eq!(e.global::<u8>(WIREFRAME_FLAG), 1);
        assert_eq!(e.mem.u8(target + 0x131), 1);
        assert_eq!(calls(&e, FN_0045C670).len(), 1);
        assert!(printed(&e).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005b_97f0, &args![]).bool());
        assert_eq!(e.global::<u8>(WIREFRAME_FLAG), 0);
        assert_eq!(e.mem.u8(target + 0x131), 0);
        assert_eq!(printed(&e), vec![vec![MSG_WIREFRAME, TEXT_OFF_CAPITAL]]);
        // The flip alone: any non-zero value becomes 0.
        e.mem.set_u8(target + 0x131, 7);
        e.call(0x005b_9870, &args![Ptr::<()>::new(target)]);
        assert_eq!(e.mem.u8(target + 0x131), 0);
        e.call(0x005b_9870, &args![Ptr::<()>::new(target)]);
        assert_eq!(e.mem.u8(target + 0x131), 1);
    }

    #[test]
    fn detection_stats_toggle_stores_its_argument() {
        let mut e = engine_c();
        accept(&mut e, &[CONSOLE_PRINT]);
        parse_gives(&mut e, true, &[1234]);
        start_log(&mut e);
        assert!(e.call(0x005b_98a0, &args![script(0)]).bool());
        assert_eq!(e.global::<u8>(DETECTION_STATS_FLAG), 1);
        assert_eq!(e.global::<u32>(DETECTION_STATS_VALUE), 1234);
        assert!(printed(&e).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005b_98a0, &args![script(0)]).bool());
        assert_eq!(e.global::<u8>(DETECTION_STATS_FLAG), 0);
        assert_eq!(
            printed(&e),
            vec![vec![MSG_DETECTION_STATS, TEXT_OFF_CAPITAL]]
        );
        // Arguments that do not parse: the flag was flipped, the value stays.
        parse_gives(&mut e, false, &[99]);
        start_log(&mut e);
        assert!(!e.call(0x005b_98a0, &args![script(0)]).bool());
        assert_eq!(e.global::<u8>(DETECTION_STATS_FLAG), 1);
        assert_eq!(e.global::<u32>(DETECTION_STATS_VALUE), 1234);
        assert!(printed(&e).is_empty());
    }

    #[test]
    fn refraction_gate_needs_the_byte_and_at_least_two() {
        let mut e = engine_c();
        refraction_gate(&mut e, true);
        assert!(e.call(0x005b_9b00, &args![]).bool());
        e.mem.set_u32(0x011f_91c0, 1);
        assert!(!e.call(0x005b_9b00, &args![]).bool());
        e.mem.set_u32(0x011f_91c0, 0xffff_ffff);
        assert!(!e.call(0x005b_9b00, &args![]).bool());
        e.mem.set_u32(0x011f_91c0, 5);
        e.mem.set_u8(0x011f_9180, 0);
        assert!(!e.call(0x005b_9b00, &args![]).bool());
    }

    #[test]
    fn set_target_refraction_applies_the_limited_power() {
        const V_PROPERTY: u32 = 0x0900_0040;
        const V_ACTOR_REFRACTION: u32 = 0x0900_0041;
        let mut e = engine_c();
        refraction_gate(&mut e, true);
        float_doubles(&mut e);
        cast_double(&mut e);
        accept(
            &mut e,
            &[SET_REFRACTION_RECURSE, CONSOLE_PRINT, V_ACTOR_REFRACTION],
        );
        e.register(V_PROPERTY, |_, _| 0x6000u32.into_ret());
        e.register(REFR_GET_NAME_TEXT, |_, _| 0x5555u32.into_ret());
        let zero = 0.0f32.to_bits();
        // Not an actor: the shader property is cleared, then set.
        let plain = reference(&mut e, &[(0x1d0, V_PROPERTY)], false);
        parse_gives(&mut e, true, &[1.5f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005b_9950, &args![script(plain)]).bool());
        assert_eq!(
            calls(&e, SET_REFRACTION_RECURSE),
            vec![
                vec![0x6000, 0, zero, 0, zero, 0],
                vec![0x6000, 0, zero, 1, zero, 0],
                vec![0x6000, 1, 1.0f32.to_bits(), 0, zero, 0],
            ]
        );
        assert!(printed(&e).is_empty());
        // A negative power is limited to 0 and does not enable it.
        parse_gives(&mut e, true, &[(-0.5f32).to_bits()]);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005b_9950, &args![script(plain)]).bool());
        assert_eq!(
            calls(&e, SET_REFRACTION_RECURSE)[2],
            vec![0x6000, 0, zero, 0, zero, 0]
        );
        let [lo, hi] = f64_words(0.0);
        assert_eq!(printed(&e), vec![vec![MSG_REFRACTION_SET, 0x5555, lo, hi]]);
        // An actor gets the call through its own slot; without a reference
        // the player is the target.
        let actor = reference(&mut e, &[(0x384, V_ACTOR_REFRACTION)], true);
        e.set_global(PLAYER, actor);
        parse_gives(&mut e, true, &[0.25f32.to_bits()]);
        start_log(&mut e);
        assert!(e.call(0x005b_9950, &args![script(0)]).bool());
        assert_eq!(
            calls(&e, V_ACTOR_REFRACTION),
            vec![vec![actor, 1, 0.25f32.to_bits()]]
        );
        assert!(calls(&e, SET_REFRACTION_RECURSE).is_empty());
        let [lo, hi] = f64_words(0.25);
        assert_eq!(printed(&e), vec![vec![MSG_REFRACTION_SET, 0x5555, lo, hi]]);
        // Arguments that do not parse; the gate closed.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_9950, &args![script(plain)]).bool());
        refraction_gate(&mut e, false);
        start_log(&mut e);
        assert!(e.call(0x005b_9950, &args![script(plain)]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
    }

    #[test]
    fn set_target_refraction_fire_passes_the_period() {
        const V_PROPERTY: u32 = 0x0900_0042;
        let mut e = engine_c();
        refraction_gate(&mut e, true);
        float_doubles(&mut e);
        cast_double(&mut e);
        accept(&mut e, &[SET_REFRACTION_RECURSE, CONSOLE_PRINT]);
        e.register(V_PROPERTY, |_, _| 0x6000u32.into_ret());
        e.register(REFR_GET_NAME_TEXT, |_, _| 0x5555u32.into_ret());
        let plain = reference(&mut e, &[(0x1d0, V_PROPERTY)], false);
        parse_gives(&mut e, true, &[2.0f32.to_bits(), 3]);
        set_echo(&mut e, true);
        start_log(&mut e);
        assert!(e.call(0x005b_9b40, &args![script(plain)]).bool());
        assert_eq!(
            calls(&e, SET_REFRACTION_RECURSE),
            vec![vec![0x6000, 1, 1.0f32.to_bits(), 1, 3.0f32.to_bits(), 0]]
        );
        let [lo, hi] = f64_words(1.0);
        assert_eq!(
            printed(&e),
            vec![vec![MSG_REFRACTION_FIRE_SET, 0x5555, lo, hi, 3]]
        );
        // An actor is left alone.
        let actor = reference(&mut e, &[(0x1d0, V_PROPERTY)], true);
        start_log(&mut e);
        assert!(e.call(0x005b_9b40, &args![script(actor)]).bool());
        assert!(calls(&e, SET_REFRACTION_RECURSE).is_empty());
        // No shader property: nothing is applied or printed.
        e.register(V_PROPERTY, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_9b40, &args![script(plain)]).bool());
        assert!(calls(&e, SET_REFRACTION_RECURSE).is_empty());
        assert!(printed(&e).is_empty());
        // Bad arguments; the gate closed.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_9b40, &args![script(plain)]).bool());
        refraction_gate(&mut e, false);
        start_log(&mut e);
        assert!(e.call(0x005b_9b40, &args![script(plain)]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
    }

    #[test]
    fn set_target_falloff_converts_the_given_channels() {
        const V_PROPERTY: u32 = 0x0900_0043;
        let mut e = engine_c();
        float_doubles(&mut e);
        accept(&mut e, &[CONSOLE_PRINT]);
        // The base colour is (1, 1, 1, 1); `-1.0` scales it to "unset".
        e.mem.set_f32(FLOAT_MINUS_ONE, -1.0);
        e.mem.set_f64(DOUBLE_MINUS_ONE, -1.0);
        e.mem.set_f32(FALLOFF_MAX_ANGLE, 90.0);
        e.mem
            .set_f64(DOUBLE_DEGREES_TO_RADIANS, std::f64::consts::PI / 180.0);
        for i in 0..4 {
            e.mem.set_f32(FALLOFF_BASE_COLOUR + 4 * i, 1.0);
        }
        e.register(COLOUR_SCALE, |e, a| {
            for i in 0..4 {
                let value = e.mem.f32(a[0] + 4 * i) * f32::from_bits(a[2]);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            a[1].into_ret()
        });
        // The angle conversion `004e4470` is a double that doubles.
        e.register(FN_004E4470, |_, a| (f32::from_bits(a[0]) * 2.0).into_ret());
        let colours = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let record = colours.clone();
        e.register_double(SET_FALLOFF_RECURSE, move |e, a| {
            record.borrow_mut().push((
                a[0],
                (0..4).map(|i| e.mem.f32(a[1] + 4 * i)).collect::<Vec<_>>(),
            ));
            Ret::default()
        });
        e.register(V_PROPERTY, |_, _| 0x6000u32.into_ret());
        let target = reference(&mut e, &[(0x1d0, V_PROPERTY)], false);
        // 120 degrees is limited to 90; the second angle is not given; 0.5 is
        // kept; 2.0 is limited to 1.
        parse_gives(
            &mut e,
            true,
            &[
                120.0f32.to_bits(),
                (-1.0f32).to_bits(),
                0.5f32.to_bits(),
                2.0f32.to_bits(),
            ],
        );
        assert!(e.call(0x005b_9c90, &args![script(target)]).bool());
        let radians = ((90.0f64 * (std::f64::consts::PI / 180.0)) as f32) * 2.0;
        assert_eq!(
            *colours.borrow(),
            vec![(0x6000, vec![radians, -1.0, 0.5, 1.0])]
        );
        // A negative colour channel is limited to 0.
        colours.borrow_mut().clear();
        parse_gives(
            &mut e,
            true,
            &[
                (-1.0f32).to_bits(),
                (-5.0f32).to_bits(),
                (-0.5f32).to_bits(),
                (-1.0f32).to_bits(),
            ],
        );
        assert!(e.call(0x005b_9c90, &args![script(target)]).bool());
        assert_eq!(
            *colours.borrow(),
            vec![(0x6000, vec![-1.0, 0.0, 0.0, -1.0])]
        );
        // Bad arguments: nothing is applied.
        colours.borrow_mut().clear();
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_9c90, &args![script(target)]).bool());
        assert!(colours.borrow().is_empty());
        // No reference: only the message, and only when echoing.
        start_log(&mut e);
        assert!(e.call(0x005b_9c90, &args![script(0)]).bool());
        assert!(printed(&e).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005b_9c90, &args![script(0)]).bool());
        assert_eq!(printed(&e), vec![vec![MSG_NO_REFERENCE]]);
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
    }

    #[test]
    fn the_float_wrapper_and_the_unsupported_command() {
        let mut e = engine_c();
        e.register(FN_004E4470, |_, a| (f32::from_bits(a[0]) + 1.0).into_ret());
        assert_eq!(e.call(0x005b_9e80, &args![2.5f32]).f32(), 3.5);
        accept(&mut e, &[CONSOLE_PRINT, FN_005D4A40]);
        start_log(&mut e);
        assert!(e.call(0x005b_9ea0, &args![]).bool());
        assert!(printed(&e).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005b_9ea0, &args![]).bool());
        assert_eq!(printed(&e), vec![vec![MSG_NO_LONGER_SUPPORTED]]);
        start_log(&mut e);
        assert!(e.call(0x005b_9ed0, &args![]).bool());
        assert_eq!(calls(&e, FN_005D4A40).len(), 1);
    }

    #[test]
    fn image_space_value_command_stores_the_value_in_element_twelve() {
        let mut e = engine_c();
        let manager = e.mem.alloc(0x20);
        let element = e.mem.alloc(0x80);
        e.register_double(GET_IMAGE_SPACE_MANAGER, move |_, _| manager.into_ret());
        e.register_double(IMAGE_SPACE_ARRAY_GET, move |_, a| {
            assert_eq!(a, [manager, 0xc]);
            element.into_ret()
        });
        parse_gives(&mut e, true, &[0.75f32.to_bits()]);
        assert!(e.call(0x005b_9ef0, &args![script(0)]).bool());
        assert_eq!(e.mem.u8(element + 5), 1);
        assert_eq!(e.mem.f32(element + 0x58), 0.75);
        // The two helpers on their own.
        e.call(0x005b_9f50, &args![Ptr::<()>::new(manager), 1.25f32]);
        assert_eq!(e.mem.f32(element + 0x58), 1.25);
        e.mem.set_u8(element + 5, 0);
        e.call(0x005b_9f80, &args![Ptr::<()>::new(element), 2.0f32]);
        assert_eq!(e.mem.u8(element + 5), 1);
        assert_eq!(e.mem.f32(element + 0x58), 2.0);
        // Bad arguments: nothing is touched.
        parse_gives(&mut e, false, &[]);
        e.mem.set_u8(element + 5, 0);
        assert!(!e.call(0x005b_9ef0, &args![script(0)]).bool());
        assert_eq!(e.mem.u8(element + 5), 0);
    }

    #[test]
    fn motion_blur_command_toggles_the_property_of_the_reference() {
        const V_PROPERTY: u32 = 0x0900_0044;
        let mut e = engine_c();
        e.register(V_PROPERTY, |_, _| 0x6000u32.into_ret());
        accept(&mut e, &[TOGGLE_MOTION_BLUR_RECURSE]);
        let plain = reference(&mut e, &[(0x1d0, V_PROPERTY)], false);
        start_log(&mut e);
        assert!(e.call(0x005b_9fa0, &args![script(plain)]).bool());
        assert_eq!(calls(&e, TOGGLE_MOTION_BLUR_RECURSE), vec![vec![0x6000]]);
        // No property, no reference: nothing.
        e.register(V_PROPERTY, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_9fa0, &args![script(plain)]).bool());
        assert!(e.call(0x005b_9fa0, &args![script(0)]).bool());
        assert!(calls(&e, TOGGLE_MOTION_BLUR_RECURSE).is_empty());
    }

    /// The doubles of the IK commands: the global objects' value bytes are
    /// at `object + 4` as `00408d60` returns them, and an actor `0x100` bytes
    /// long whose IK object (at `+0xac`) is `0x300` bytes.
    fn ik_engine() -> (Engine, u32, u32) {
        const V_NAME: u32 = 0x0900_0050;
        let mut e = engine_c();
        cast_double(&mut e);
        accept(&mut e, &[CONSOLE_PRINT]);
        e.register(GLOBAL_VALUE_ADDRESS, |_, a| (a[0] + 4).into_ret());
        e.register(FOOT_IK_GET, |e, _| e.mem.u8(FOOT_IK_OBJECT + 4).into_ret());
        e.register(V_NAME, |_, _| 0x5555u32.into_ret());
        let actor = reference(&mut e, &[(0x130, V_NAME)], true);
        let ik = e.mem.alloc(0x300);
        e.mem.set_u32(actor + 0xac, ik);
        (e, actor, ik)
    }

    /// One IK command: the global flag flips with an echo, and an actor's
    /// flag follows when the permission byte at `allow` is set.
    fn check_ik_command(address: u32, global: u32, allow: u32, flag: u32, messages: (u32, u32)) {
        let (mut e, actor, ik) = ik_engine();
        // Global flag.
        start_log(&mut e);
        assert!(e.call(address, &args![script(0)]).bool());
        assert_eq!(e.mem.u8(global + 4), 1);
        assert!(printed(&e).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(address, &args![script(0)]).bool());
        assert_eq!(e.mem.u8(global + 4), 0);
        assert_eq!(printed(&e), vec![vec![messages.0, TEXT_OFF_CAPITAL]]);
        // An actor whose IK object does not allow the flag stays off.
        start_log(&mut e);
        assert!(e.call(address, &args![script(actor)]).bool());
        assert_eq!(e.mem.u8(ik + flag), 0);
        assert_eq!(
            printed(&e),
            vec![vec![messages.1, 0x5555, TEXT_OFF_CAPITAL]]
        );
        // With the permission it flips back and forth.
        e.mem.set_u8(ik + allow, 1);
        start_log(&mut e);
        assert!(e.call(address, &args![script(actor)]).bool());
        assert_eq!(e.mem.u8(ik + flag), 1);
        assert_eq!(printed(&e), vec![vec![messages.1, 0x5555, TEXT_ON_CAPITAL]]);
        assert!(e.call(address, &args![script(actor)]).bool());
        assert_eq!(e.mem.u8(ik + flag), 0);
        // A reference that is no actor, an actor without the IK object.
        let plain = reference(&mut e, &[], false);
        e.mem.set_u32(actor + 0xac, 0);
        start_log(&mut e);
        assert!(e.call(address, &args![script(plain)]).bool());
        assert!(e.call(address, &args![script(actor)]).bool());
        assert!(printed(&e).is_empty());
    }

    #[test]
    fn foot_ik_toggle_flips_the_global_or_the_actor_flag() {
        check_ik_command(
            0x005b_9fe0,
            FOOT_IK_OBJECT,
            0x21f,
            0x220,
            (MSG_FOOT_IK, MSG_FOOT_IK_REF),
        );
        // The setters and getters on their own.
        let (mut e, _, ik) = ik_engine();
        e.call(0x005b_a110, &args![9u32]);
        assert_eq!(e.mem.u8(FOOT_IK_OBJECT + 4), 9);
        e.mem.set_u8(ik + 0x21f, 1);
        e.call(0x005b_a130, &args![Ptr::<()>::new(ik), 5u32]);
        assert_eq!(e.call(0x005b_a180, &args![Ptr::<()>::new(ik)]).u8(), 1);
        e.call(0x005b_a130, &args![Ptr::<()>::new(ik), 0u32]);
        assert_eq!(e.mem.u8(ik + 0x220), 0);
    }

    #[test]
    fn grab_ik_toggle_flips_the_global_or_the_actor_flag() {
        check_ik_command(
            0x005b_a1a0,
            GRAB_IK_OBJECT,
            0x21c,
            0x21d,
            (MSG_GRAB_IK, MSG_GRAB_IK_REF),
        );
        let (mut e, _, ik) = ik_engine();
        e.call(0x005b_a2f0, &args![4u32]);
        assert_eq!(e.call(0x005b_a2d0, &args![]).u8(), 4);
        assert_eq!(e.mem.u8(GRAB_IK_OBJECT + 4), 4);
        e.mem.set_u8(ik + 0x21c, 1);
        e.call(0x005b_a310, &args![Ptr::<()>::new(ik), 1u32]);
        assert_eq!(e.call(0x005b_a360, &args![Ptr::<()>::new(ik)]).u8(), 1);
        // Without the permission byte the flag is cleared.
        e.mem.set_u8(ik + 0x21c, 0);
        e.call(0x005b_a310, &args![Ptr::<()>::new(ik), 1u32]);
        assert_eq!(e.mem.u8(ik + 0x21d), 0);
    }

    #[test]
    fn look_ik_toggle_flips_the_global_or_the_actor_flag() {
        check_ik_command(
            0x005b_a380,
            LOOK_IK_OBJECT,
            0xb0,
            0xb1,
            (MSG_LOOK_IK, MSG_LOOK_IK_REF),
        );
        let (mut e, _, ik) = ik_engine();
        e.call(0x005b_a4d0, &args![6u32]);
        assert_eq!(e.call(0x005b_a4b0, &args![]).u8(), 6);
        assert_eq!(e.mem.u8(LOOK_IK_OBJECT + 4), 6);
        e.mem.set_u8(ik + 0xb0, 1);
        e.call(0x005b_a4f0, &args![Ptr::<()>::new(ik), 1u32]);
        assert_eq!(e.call(0x005b_a540, &args![Ptr::<()>::new(ik)]).u8(), 1);
        e.call(0x005b_a4f0, &args![Ptr::<()>::new(ik), 0u32]);
        assert_eq!(e.mem.u8(ik + 0xb1), 0);
    }

    #[test]
    fn ragdoll_animation_toggle_inverts_the_flag() {
        let mut e = engine_c();
        accept(&mut e, &[CONSOLE_PRINT]);
        e.register(RAGDOLL_ANIM_GET, |e, _| e.mem.u8(0x0126_7c28).into_ret());
        e.register(RAGDOLL_ANIM_SET, |e, a| {
            e.mem.set_u8(0x0126_7c28, a[0] as u8);
            Ret::default()
        });
        start_log(&mut e);
        assert!(e.call(0x005b_a560, &args![]).bool());
        assert_eq!(e.mem.u8(0x0126_7c28), 1);
        assert_eq!(calls(&e, RAGDOLL_ANIM_SET), vec![vec![1]]);
        assert!(printed(&e).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005b_a560, &args![]).bool());
        assert_eq!(e.mem.u8(0x0126_7c28), 0);
        assert_eq!(
            printed(&e),
            vec![vec![MSG_RAGDOLL_ANIMATION, TEXT_OFF_CAPITAL]]
        );
    }

    #[test]
    fn set_vel_puts_the_value_on_the_named_axis() {
        const V_TARGET: u32 = 0x0900_0045;
        let mut e = engine_c();
        e.register(NI_POINT3_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        e.register(V_TARGET, |_, _| 0x6000u32.into_ret());
        let velocities = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let record = velocities.clone();
        e.register_double(ADD_VELOCITY, move |e, a| {
            record.borrow_mut().push((
                a[0],
                [e.mem.f32(a[1]), e.mem.f32(a[1] + 4), e.mem.f32(a[1] + 8)],
                a[2],
            ));
            Ret::default()
        });
        let target = reference(&mut e, &[(0x1d0, V_TARGET)], false);
        for (axis, expected) in [
            (b'X', [3.5, 0.0, 0.0]),
            (b'Y', [0.0, 3.5, 0.0]),
            (b'Z', [0.0, 0.0, 3.5]),
            (b'Q', [0.0, 0.0, 0.0]),
        ] {
            velocities.borrow_mut().clear();
            parse_gives(&mut e, true, &[u32::from(axis), 3.5f32.to_bits()]);
            assert!(e.call(0x005b_a5d0, &args![script(target)]).bool());
            assert_eq!(*velocities.borrow(), vec![(0x6000, expected, 1)]);
        }
        // No reference: nothing happens; bad arguments: false.
        velocities.borrow_mut().clear();
        assert!(e.call(0x005b_a5d0, &args![script(0)]).bool());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_a5d0, &args![script(target)]).bool());
        assert!(velocities.borrow().is_empty());
    }

    #[test]
    fn flag_flip_command_needs_the_refraction_gate() {
        let mut e = engine_c();
        refraction_gate(&mut e, false);
        assert!(e.call(0x005b_a690, &args![]).bool());
        assert_eq!(e.global::<u8>(FLAG_011F9FC2), 0);
        refraction_gate(&mut e, true);
        assert!(e.call(0x005b_a690, &args![]).bool());
        assert_eq!(e.global::<u8>(FLAG_011F9FC2), 1);
        e.call(0x005b_a6b0, &args![]);
        assert_eq!(e.global::<u8>(FLAG_011F9FC2), 0);
        e.mem.set_u8(FLAG_011F9FC2, 7);
        e.call(0x005b_a6b0, &args![]);
        assert_eq!(e.global::<u8>(FLAG_011F9FC2), 0);
    }

    #[test]
    fn lite_brite_toggle_and_accessors() {
        let mut e = engine_c();
        accept(&mut e, &[CONSOLE_PRINT]);
        start_log(&mut e);
        assert!(e.call(0x005b_a6d0, &args![]).bool());
        assert_eq!(e.call(0x005b_a740, &args![]).u8(), 1);
        assert!(printed(&e).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005b_a6d0, &args![]).bool());
        assert_eq!(e.global::<u8>(LITE_BRITE_FLAG), 0);
        assert_eq!(printed(&e), vec![vec![MSG_LITE_BRITE, TEXT_OFF_CAPITAL]]);
        e.call(0x005b_a750, &args![5u32]);
        assert_eq!(e.global::<u8>(LITE_BRITE_FLAG), 5);
        // Any non-zero value counts as set and is switched off.
        assert!(e.call(0x005b_a6d0, &args![]).bool());
        assert_eq!(e.global::<u8>(LITE_BRITE_FLAG), 0);
    }

    // ---- Fourth batch (`005ba760` onward) -----------------------------------

    /// `engine_c` plus the pages of the fourth batch's globals.
    fn engine_d() -> Engine {
        let mut e = engine_c();
        for page in [
            0x011a_4000,
            0x011a_d000,
            0x011c_6000,
            0x011c_7000,
            0x011c_a000,
            0x011d_8000,
            0x011d_b000,
            0x011d_f000,
            0x011f_4000,
            0x0120_4000,
            0x0126_8000,
            0x0101_3000,
            0x0101_8000,
            0x0101_b000,
            0x0101_e000,
            0x0107_8000,
        ] {
            if !e.mem.is_mapped(page) {
                e.map(page, 0x1000);
            }
        }
        e
    }

    static NEXT_FAKE_SLOT_FUNCTION: std::sync::atomic::AtomicU32 =
        std::sync::atomic::AtomicU32::new(0x0a00_0000);

    /// An object (0x100 bytes) whose virtual slots return the given `(byte
    /// offset, value)` pairs.
    fn object_answering(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let mut table = vec![];
        for (slot, value) in slots {
            let function =
                NEXT_FAKE_SLOT_FUNCTION.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let value = *value;
            e.register_double(function, move |_, _| value.into_ret());
            table.push((*slot, function));
        }
        object_with(e, &table)
    }

    /// A NUL-terminated copy of `text` in the heap.
    fn text_at(e: &mut Engine, text: &[u8]) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(at, text);
        at
    }

    /// `ParseParameters` double that writes the string `text` to its first
    /// output and `value` to its second.
    fn parse_gives_text(e: &mut Engine, text: &'static [u8], value: u32) {
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            e.mem.set_cstr(a[7], text);
            e.mem.set_u32(a[8], value);
            true.into_ret()
        });
    }

    #[test]
    fn toggle_debug_text_prints_the_new_state_when_echoing() {
        let mut e = engine_d();
        accept(
            &mut e,
            &[CONSOLE_PRINT, INTERFACE_TOGGLE_DEBUG_TEXT_VISIBLE],
        );
        e.register(INTERFACE_IS_DEBUG_TEXT_VISIBLE, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_a760, &args![]).bool());
        assert!(printed(&e).is_empty());
        assert_eq!(calls(&e, INTERFACE_TOGGLE_DEBUG_TEXT_VISIBLE).len(), 1);
        set_echo(&mut e, true);
        assert!(e.call(0x005b_a760, &args![]).bool());
        assert_eq!(printed(&e), vec![vec![MSG_DEBUG_TEXT, TEXT_ON_CAPITAL]]);
        e.register(INTERFACE_IS_DEBUG_TEXT_VISIBLE, |_, _| false.into_ret());
        assert!(e.call(0x005b_a760, &args![]).bool());
        assert_eq!(
            printed(&e).last().unwrap(),
            &vec![MSG_DEBUG_TEXT, TEXT_OFF_CAPITAL]
        );
    }

    #[test]
    fn debug_text_table_is_created_once() {
        let mut e = engine_d();
        e.register(FN_00802A90, |e, _| {
            e.set_global(DEBUG_TEXT_TABLE, 0x7000u32);
            0x7000u32.into_ret()
        });
        start_log(&mut e);
        assert_eq!(e.call(0x005b_a8d0, &args![]).u32(), 0x7000);
        assert_eq!(e.call(0x005b_a8d0, &args![]).u32(), 0x7000);
        assert_eq!(calls(&e, FN_00802A90).len(), 1);
    }

    #[test]
    fn debug_text_command_lists_or_selects_a_name() {
        let mut e = engine_d();
        e.set_global(DEBUG_TEXT_TABLE, 0x7000u32);
        accept(
            &mut e,
            &[
                DEBUG_TEXT_TABLE_LIST,
                DEBUG_TEXT_TABLE_SELECT,
                WRITE_PRIVATE_PROFILE_STRING,
            ],
        );
        let directory = text_at(&mut e, b"C:\\Game\\");
        e.mem.set_cstr(TEXT_CUSTOM_INI, b"Custom.ini");
        e.register_double(FN_004DC110, move |_, _| directory.into_ret());
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRCAT_S, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register_double(WRITE_PRIVATE_PROFILE_STRING, |e, a| {
            let line = format!(
                "{}|{}|{}|{}",
                a[0],
                a[1],
                String::from_utf8(e.mem.cstr(a[2])).unwrap(),
                String::from_utf8(e.mem.cstr(a[3])).unwrap()
            );
            e.mem.set_cstr(0x0126_8200, line.as_bytes());
            Ret::default()
        });
        // No name: the table lists its names through the console print.
        parse_gives_text(&mut e, b"", 0);
        start_log(&mut e);
        assert!(e.call(0x005b_a7c0, &args![script(0)]).bool());
        assert_eq!(
            calls(&e, DEBUG_TEXT_TABLE_LIST),
            vec![vec![0x7000, CONSOLE_PRINT]]
        );
        assert!(calls(&e, DEBUG_TEXT_TABLE_SELECT).is_empty());
        assert!(calls(&e, WRITE_PRIVATE_PROFILE_STRING).is_empty());
        // A name: selected in the table and saved to Custom.ini.
        parse_gives_text(&mut e, b"Stats", 3);
        start_log(&mut e);
        assert!(e.call(0x005b_a7c0, &args![script(0)]).bool());
        assert!(calls(&e, DEBUG_TEXT_TABLE_LIST).is_empty());
        let select = calls(&e, DEBUG_TEXT_TABLE_SELECT);
        assert_eq!((select[0][0], select[0][2]), (0x7000, 3));
        assert_eq!(
            String::from_utf8(e.mem.cstr(0x0126_8200)).unwrap(),
            format!(
                "{}|{}|Stats|C:\\Game\\Custom.ini",
                TEXT_SECTION_DISPLAY, TEXT_KEY_DEBUG_TEXT
            )
        );
        // Parameters that do not parse: false, nothing else.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_a7c0, &args![script(0)]).bool());
        assert!(calls(&e, DEBUG_TEXT_TABLE_LIST).is_empty());
        assert!(calls(&e, DEBUG_TEXT_TABLE_SELECT).is_empty());
    }

    #[test]
    fn scene_slot_getters_and_the_callback_global() {
        let mut e = engine_d();
        e.mem.set_u32(SLOT_SPLATTER_SCENE, 0x1111);
        e.mem.set_u32(SLOT_INTERFACE_SCENE, 0x2222);
        e.mem.set_u32(SLOT_INTERFACE_3D_SCENE, 0x3333);
        assert_eq!(e.call(0x005b_ac50, &args![]).u32(), 0x1111);
        assert_eq!(e.call(0x005b_ac60, &args![]).u32(), 0x2222);
        assert_eq!(e.call(0x005b_ac70, &args![]).u32(), 0x3333);
        e.call(0x005b_ac80, &args![0x4d78c0u32]);
        assert_eq!(e.global::<u32>(SCENE_GRAPH_CALLBACK_GLOBAL), 0x4d78c0);
    }

    #[test]
    fn splatter_scene_needs_the_scene_and_a_non_zero_answer() {
        let mut e = engine_d();
        e.register(FN_00453470, |_, a| (a[0] == 0x1111).into_ret());
        // No scene: false, the object is not asked.
        start_log(&mut e);
        assert!(!e.call(0x005b_ac20, &args![]).bool());
        assert!(calls(&e, FN_00453470).is_empty());
        // A scene the object answers for.
        e.mem.set_u32(SLOT_SPLATTER_SCENE, 0x1111);
        assert!(e.call(0x005b_ac20, &args![]).bool());
        assert_eq!(calls(&e, FN_00453470), vec![vec![0x1111]]);
        // A scene it answers zero for.
        e.mem.set_u32(SLOT_SPLATTER_SCENE, 0x2222);
        assert!(!e.call(0x005b_ac20, &args![]).bool());
    }

    /// Doubles for the scene graph windows: the allocation, the singleton
    /// getters and the constructor and `AddObject` (which log their words);
    /// `FN_00450B80(i)` answers `0x5000 + i` for the menu slots 1 and 3 and
    /// zero for the others.
    fn scene_graph_doubles(e: &mut Engine) {
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FN_0044DDC0, |_, _| 0x8888u32.into_ret());
        e.register(FN_0084E3A0, |_, _| 0x9999u32.into_ret());
        e.register(FN_0045C670, |_, _| 0x4444u32.into_ret());
        e.register(SCENE_GRAPH_WINDOW_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SCENE_GRAPH_WINDOW_ADD_OBJECT, |_, _| Ret::default());
        e.register(SCENE_GRAPH_WINDOW_FILL, |_, _| Ret::default());
        e.register(FN_00450B80, |_, a| {
            (if a[0] == 1 || a[0] == 3 {
                0x5000 + a[0]
            } else {
                0
            })
            .into_ret()
        });
        e.mem.set_u32(SLOT_INTERFACE_SCENE, 0x2222);
        e.mem.set_u32(SLOT_INTERFACE_3D_SCENE, 0x3333);
        e.set_global(GLOBAL_0011DEA0C, 0x7777u32);
    }

    /// The `(object, title)` pairs added to windows, in order.
    fn added_objects(e: &Engine) -> Vec<(u32, u32)> {
        calls(e, SCENE_GRAPH_WINDOW_ADD_OBJECT)
            .iter()
            .map(|words| (words[1], words[2]))
            .collect()
    }

    #[test]
    fn scene_graph_command_without_a_reference_opens_the_world_window() {
        let mut e = engine_d();
        scene_graph_doubles(&mut e);
        e.mem.set_u32(SLOT_SPLATTER_SCENE, 0x1111);
        e.register(FN_00453470, |_, _| true.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_a8f0, &args![script(0)]).bool());
        assert_eq!(
            e.global::<u32>(SCENE_GRAPH_CALLBACK_GLOBAL),
            SCENE_GRAPH_CALLBACK
        );
        let construct = calls(&e, SCENE_GRAPH_WINDOW_CONSTRUCT);
        let window = construct[0][0];
        assert_ne!(window, 0);
        // this, *(singleton + 0x0c), *(singleton + 8), world scene, title,
        // placement.
        assert_eq!(
            construct[0][1..],
            [
                0x9999,
                0x8888,
                0x4444,
                TITLE_WORLD,
                0x8000_0000,
                0x8000_0000,
                800,
                600
            ]
        );
        assert_eq!(
            added_objects(&e),
            vec![
                (0x2222, TITLE_INTERFACE),
                (0x3333, TITLE_INTERFACE_3D_OBJECT),
                (0x5001, TITLE_MENU_OBJECT_3D),
                (0x5003, TITLE_MENU_PLAYER_3D),
                (0x1111, TITLE_SPLATTER_SCENE),
            ]
        );
        assert!(calls(&e, SCENE_GRAPH_WINDOW_ADD_OBJECT)
            .iter()
            .all(|words| words[0] == window));
        // A reference whose object slot gives nothing is the same command.
        let nothing = object_answering(&mut e, &[(0x1d0, 0)]);
        start_log(&mut e);
        assert!(e.call(0x005b_a8f0, &args![script(nothing)]).bool());
        assert_eq!(calls(&e, SCENE_GRAPH_WINDOW_CONSTRUCT).len(), 1);
        assert_eq!(added_objects(&e).len(), 5);
    }

    #[test]
    fn scene_graph_command_without_optional_scenes_adds_only_the_interface() {
        let mut e = engine_d();
        scene_graph_doubles(&mut e);
        e.register(FN_00450B80, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_a8f0, &args![script(0)]).bool());
        assert_eq!(
            added_objects(&e),
            vec![
                (0x2222, TITLE_INTERFACE),
                (0x3333, TITLE_INTERFACE_3D_OBJECT)
            ]
        );
    }

    #[test]
    fn scene_graph_command_for_the_player_names_the_window_after_the_view() {
        let mut e = engine_d();
        scene_graph_doubles(&mut e);
        let player = object_answering(&mut e, &[(0x1d0, 0x6000)]);
        e.set_global(PLAYER, player);
        e.register(REFR_GET_NAME_TEXT, |_, _| 0xaaaau32.into_ret());
        e.register(PLAYER_GET_PERSON_OBJECT, |_, a| (0x6100 + a[1]).into_ret());
        e.register(PLAYER_GET_INVENTORY_MENU_OBJECT, |_, _| {
            0x6200u32.into_ret()
        });
        for (third, window_title, other_title) in [
            (1u8, TITLE_PLAYER_THIRD_PERSON, TITLE_PLAYER_FIRST_PERSON),
            (0u8, TITLE_PLAYER_FIRST_PERSON, TITLE_PLAYER_THIRD_PERSON),
        ] {
            e.register_double(PLAYER_IS_THIRD_PERSON, move |_, _| third.into_ret());
            start_log(&mut e);
            assert!(e.call(0x005b_a8f0, &args![script(player)]).bool());
            let construct = calls(&e, SCENE_GRAPH_WINDOW_CONSTRUCT);
            assert_eq!(
                construct[0][1..],
                [
                    0x9999,
                    0x8888,
                    0x6000,
                    window_title,
                    0x8000_0000,
                    0x8000_0000,
                    800,
                    600
                ]
            );
            assert_eq!(
                added_objects(&e),
                vec![
                    (0x6100 + u32::from(third), other_title),
                    (0x6200, TITLE_PLAYER_INVENTORY_MENU),
                ]
            );
        }
        // Without an inventory menu object only the other person is added.
        e.register(PLAYER_GET_INVENTORY_MENU_OBJECT, |_, _| 0u32.into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_a8f0, &args![script(player)]).bool());
        assert_eq!(added_objects(&e).len(), 1);
    }

    #[test]
    fn scene_graph_command_for_another_reference_is_titled_with_its_name() {
        let mut e = engine_d();
        scene_graph_doubles(&mut e);
        let player = object_answering(&mut e, &[(0x1d0, 0x6000)]);
        e.set_global(PLAYER, player);
        let other = object_answering(&mut e, &[(0x1d0, 0x6500)]);
        e.register(REFR_GET_NAME_TEXT, |_, a| (a[0] + 1).into_ret());
        start_log(&mut e);
        assert!(e.call(0x005b_a8f0, &args![script(other)]).bool());
        let construct = calls(&e, SCENE_GRAPH_WINDOW_CONSTRUCT);
        assert_eq!(
            construct[0][1..],
            [
                0x9999,
                0x8888,
                0x6500,
                other + 1,
                0x8000_0000,
                0x8000_0000,
                800,
                600
            ]
        );
        assert!(added_objects(&e).is_empty());
    }

    #[test]
    fn world_scene_graph_command_fills_the_window_from_the_reference() {
        let mut e = engine_d();
        scene_graph_doubles(&mut e);
        e.register(FN_00450B80, |_, a| {
            (if a[0] == 1 { 0x5001u32 } else { 0 }).into_ret()
        });
        let target = object_answering(&mut e, &[(0x1d0, 0x6500)]);
        start_log(&mut e);
        assert!(e.call(0x005b_ac90, &args![script(target)]).bool());
        assert_eq!(
            e.global::<u32>(SCENE_GRAPH_CALLBACK_GLOBAL),
            SCENE_GRAPH_CALLBACK
        );
        let window = calls(&e, SCENE_GRAPH_WINDOW_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, SCENE_GRAPH_WINDOW_FILL),
            vec![vec![window, 0x6500]]
        );
        assert_eq!(
            added_objects(&e),
            vec![
                (0x2222, TITLE_INTERFACE),
                (0x3333, TITLE_INTERFACE_3D_OBJECT),
                (0x5001, TITLE_RENDERED_MENU_3D),
            ]
        );
        // No reference: no fill.
        start_log(&mut e);
        assert!(e.call(0x005b_ac90, &args![script(0)]).bool());
        assert!(calls(&e, SCENE_GRAPH_WINDOW_FILL).is_empty());
    }

    /// Doubles for the viewer strings: a collector of the strings in
    /// `strings`, and a log of the object whose slot `0x60` collected.
    fn viewer_doubles(e: &mut Engine, strings: &[&'static [u8]]) {
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        // A collector is an object whose virtual slot 0 is its deleting
        // destructor ([`V_COLLECTOR_DELETE`]).
        e.register(STRING_COLLECTOR_CONSTRUCT, |e, a| {
            let vtable = e.mem.alloc(0x10);
            e.mem.set_u32(vtable, V_COLLECTOR_DELETE);
            e.mem.set_u32(a[0], vtable);
            a[0].into_ret()
        });
        e.register(V_COLLECTOR_DELETE, |_, _| Ret::default());
        let addresses: Vec<u32> = strings.iter().map(|s| text_at(e, s)).collect();
        // The string pointers are in an array at 0x0126_8300.
        for (i, address) in addresses.iter().enumerate() {
            e.mem.set_u32(0x0126_8300 + 4 * i as u32, *address);
        }
        let count = addresses.len() as u32;
        e.register_double(STRING_COLLECTOR_COUNT, move |_, _| count.into_ret());
        e.register(STRING_COLLECTOR_STRING_ADDRESS, |_, a| {
            (0x0126_8300 + 4 * a[1]).into_ret()
        });
        e.register(STRING_COLLECTOR_CLEAR, |_, _| Ret::default());
        e.register(NODE_GET_ATTACHED_OBJECT, |_, _| 0u32.into_ret());
    }

    /// Slot 0 of the collectors the viewer doubles make.
    const V_COLLECTOR_DELETE: u32 = 0x0900_0060;

    #[test]
    fn viewer_strings_needs_a_reference_with_an_object() {
        let mut e = engine_d();
        accept(&mut e, &[CONSOLE_PRINT]);
        start_log(&mut e);
        assert!(e.call(0x005b_ade0, &args![script(0)]).bool());
        let no_object = object_answering(&mut e, &[(0x1d0, 0)]);
        assert!(e.call(0x005b_ade0, &args![script(no_object)]).bool());
        assert_eq!(
            printed(&e),
            vec![
                vec![MSG_VIEWER_NEEDS_REFERENCE],
                vec![MSG_VIEWER_NEEDS_REFERENCE]
            ]
        );
    }

    #[test]
    fn viewer_strings_prints_the_collected_strings_of_the_object() {
        let mut e = engine_d();
        accept(&mut e, &[CONSOLE_PRINT]);
        viewer_doubles(&mut e, &[b"first", b"second"]);
        e.register(0x0a10_0001, |e, a| {
            // Collects: remember the collector in a global.
            e.mem.set_u32(0x0126_8388, a[1]);
            Ret::default()
        });
        let object = object_with(&mut e, &[(0x0c, V_FALSE), (0x60, 0x0a10_0001)]);
        let reference = object_answering(&mut e, &[(0x1d0, object)]);
        // `object_answering` made a new vtable: the reference returns `object`.
        start_log(&mut e);
        assert!(e.call(0x005b_ade0, &args![script(reference)]).bool());
        let collector = e.mem.u32(0x0126_8388);
        assert_ne!(collector, 0);
        let printed_text: Vec<Vec<u32>> = printed(&e);
        assert_eq!(printed_text.len(), 2);
        assert_eq!(e.mem.cstr(printed_text[0][0]), b"first".to_vec());
        assert_eq!(e.mem.cstr(printed_text[1][0]), b"second".to_vec());
        assert_eq!(calls(&e, STRING_COLLECTOR_CLEAR).len(), 1);
        // The collector is deleted (slot 0, argument 1).
        assert_eq!(calls(&e, V_COLLECTOR_DELETE), vec![vec![collector, 1]]);
    }

    #[test]
    fn viewer_strings_uses_the_first_child_of_a_node_and_the_attached_object() {
        let mut e = engine_d();
        accept(&mut e, &[CONSOLE_PRINT]);
        viewer_doubles(&mut e, &[b"only"]);
        e.register(0x0a10_0002, |e, a| {
            let n = e.mem.u32(0x0126_8390);
            e.mem.set_u32(0x0126_8390, n + 1);
            e.mem.set_u32(0x0126_8394 + 4 * n, a[0]);
            Ret::default()
        });
        let child = object_with(&mut e, &[(0x0c, V_FALSE), (0x60, 0x0a10_0002)]);
        let node = object_with(&mut e, &[(0x0c, V_TRUE), (0x60, 0x0a10_0002)]);
        let reference = object_answering(&mut e, &[(0x1d0, node)]);
        // The node has two children; the first is empty, the second used.
        e.register(NODE_CHILD_COUNT, |_, _| 2u32.into_ret());
        e.register_double(NODE_CHILD_AT, move |_, a| {
            (if a[1] == 0 { 0 } else { child }).into_ret()
        });
        // The child has an attached object that converts to a target object.
        e.register(NODE_GET_ATTACHED_OBJECT, |_, _| 0xb000u32.into_ret());
        e.register(FN_00653270, |_, a| {
            assert_eq!(a, [TYPE_012043F8, 0xb000]);
            0xb100u32.into_ret()
        });
        let target = object_with(&mut e, &[(0x60, 0x0a10_0002)]);
        e.register_double(FN_006FA820, move |_, a| {
            assert_eq!(a, [0xb100]);
            target.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005b_ade0, &args![script(reference)]).bool());
        // Collected from the child, then from the target object.
        assert_eq!(e.mem.u32(0x0126_8390), 2);
        assert_eq!(e.mem.u32(0x0126_8394), child);
        assert_eq!(e.mem.u32(0x0126_8398), target);
        assert_eq!(printed(&e).len(), 2);
        assert_eq!(calls(&e, STRING_COLLECTOR_CLEAR).len(), 2);
        // The child was fetched for index 0 (empty) and index 1.
        assert_eq!(calls(&e, NODE_CHILD_AT).len(), 2);
    }

    #[test]
    fn viewer_strings_reports_a_node_without_children() {
        let mut e = engine_d();
        accept(&mut e, &[CONSOLE_PRINT]);
        viewer_doubles(&mut e, &[]);
        e.register(NODE_CHILD_COUNT, |_, _| 0u32.into_ret());
        let node = object_with(&mut e, &[(0x0c, V_TRUE)]);
        let reference = object_answering(&mut e, &[(0x1d0, node)]);
        start_log(&mut e);
        assert!(e.call(0x005b_ade0, &args![script(reference)]).bool());
        assert_eq!(printed(&e), vec![vec![MSG_VIEWER_NODE_WITHOUT_CHILDREN]]);
        assert!(calls(&e, STRING_COLLECTOR_CLEAR).is_empty());
    }

    /// Doubles for the LOD command: the world space and terrain manager
    /// getters, a string compare ignoring case, and four LOD objects
    /// (`0x7100` trees, `0x7200` land, `0x7300` objects, `0x7400` water) whose
    /// flag byte is at `+0`; the flag setter logs.
    fn lod_doubles(e: &mut Engine, terrain: u32) {
        e.map(0x7000, 0x1000);
        e.set_global(GLOBAL_0011DEA10, 0x7000u32);
        e.register(TES_GET_WORLD_SPACE, |_, a| (a[0] + 1).into_ret());
        for (address, text) in [
            (TEXT_ALL, &b"all"[..]),
            (TEXT_WATER, b"water"),
            (TEXT_LAND, b"land"),
            (TEXT_TREES, b"trees"),
            (TEXT_OBJECTS, b"objects"),
            (TEXT_BUILDINGS, b"buildings"),
        ] {
            e.mem.set_cstr(address, text);
        }
        e.register_double(WORLD_SPACE_GET_TERRAIN_MANAGER, move |_, a| {
            assert_eq!(a, [0x7001]);
            terrain.into_ret()
        });
        e.register(COMPARE_STRINGS_NO_CASE, |e, a| {
            let (x, y) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            (x.eq_ignore_ascii_case(&y) as u32 ^ 1).into_ret()
        });
        for (getter, object) in [
            (LOD_TREES_OBJECT, 0x7100u32),
            (LOD_LAND_OBJECT, 0x7200),
            (LOD_OBJECTS_OBJECT, 0x7300),
            (LOD_WATER_OBJECT, 0x7400),
        ] {
            e.register_double(getter, move |_, _| object.into_ret());
        }
        e.register(LOD_OBJECT_GET_FLAG, |e, a| e.mem.u8(a[0]).into_ret());
        e.register(LOD_OBJECT_SET_FLAG, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            Ret::default()
        });
    }

    #[test]
    fn lod_command_selects_the_kinds_named_by_the_argument() {
        let flags = |e: &Engine| [0x7100u32, 0x7200, 0x7300, 0x7400].map(|object| e.mem.u8(object));
        for (word, expected, message) in [
            (&b"all"[..], [1, 1, 1, 1], MSG_TOGGLING_ALL_LOD),
            (b"", [1, 1, 1, 1], MSG_TOGGLING_ALL_LOD),
            (b"TREES", [1, 0, 0, 0], MSG_TOGGLING_TREES),
            (b"land", [0, 1, 0, 0], MSG_TOGGLING_LAND),
            (b"objects", [0, 0, 1, 0], MSG_TOGGLING_OBJECTS),
            (b"buildings", [0, 0, 1, 0], MSG_TOGGLING_OBJECTS),
            (b"water", [0, 0, 0, 1], MSG_TOGGLING_WATER),
        ] {
            let mut e = engine_d();
            lod_doubles(&mut e, 0x7500);
            accept(&mut e, &[CONSOLE_PRINT]);
            let text = word;
            e.register_double(PARSE_PARAMETERS, move |e, a| {
                e.mem.set_cstr(a[7], text);
                true.into_ret()
            });
            start_log(&mut e);
            assert!(e.call(0x005b_b030, &args![script(0)]).bool());
            assert_eq!(printed(&e), vec![vec![message]]);
            // Every selected flag byte went from 0 to 1.
            assert_eq!(flags(&e), expected, "{:?}", String::from_utf8_lossy(word));
        }
    }

    #[test]
    fn lod_command_flips_a_set_flag_back_and_skips_missing_objects() {
        let mut e = engine_d();
        lod_doubles(&mut e, 0x7500);
        accept(&mut e, &[CONSOLE_PRINT]);
        e.register_double(PARSE_PARAMETERS, |e, a| {
            e.mem.set_cstr(a[7], b"land");
            true.into_ret()
        });
        e.mem.set_u8(0x7200, 1);
        assert!(e.call(0x005b_b030, &args![script(0)]).bool());
        assert_eq!(e.mem.u8(0x7200), 0);
        // A missing LOD object is skipped: land answers nothing.
        e.register(LOD_LAND_OBJECT, |_, _| 0u32.into_ret());
        assert!(e.call(0x005b_b030, &args![script(0)]).bool());
        assert_eq!(e.mem.u8(0x7200), 0);
    }

    #[test]
    fn lod_command_rejects_an_unknown_kind_and_needs_a_terrain_manager() {
        let mut e = engine_d();
        lod_doubles(&mut e, 0x7500);
        accept(&mut e, &[CONSOLE_PRINT]);
        e.register_double(PARSE_PARAMETERS, |e, a| {
            e.mem.set_cstr(a[7], b"sky");
            true.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005b_b030, &args![script(0)]).bool());
        assert_eq!(printed(&e), vec![vec![MSG_LOD_INVALID_PARAMETER]]);
        assert!(calls(&e, LOD_OBJECT_SET_FLAG).is_empty());
        // The first word zero: the string is not parsed and counts as empty.
        let mut quiet = ScriptArgs {
            param_info: 0,
            ..script(0)
        };
        quiet.param_info = 0;
        start_log(&mut e);
        assert!(e.call(0x005b_b030, &args![quiet]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        assert_eq!(printed(&e).last().unwrap(), &vec![MSG_TOGGLING_ALL_LOD]);
        // Parameters that do not parse: false.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_b030, &args![script(0)]).bool());
        // No terrain manager: true, nothing printed or toggled.
        let mut e = engine_d();
        lod_doubles(&mut e, 0);
        accept(&mut e, &[CONSOLE_PRINT]);
        parse_gives(&mut e, true, &[]);
        start_log(&mut e);
        assert!(e.call(0x005b_b030, &args![script(0)]).bool());
        assert!(printed(&e).is_empty());
    }

    // ---- Fourth batch, second part (`005bb390` onward) ----------------------

    /// Doubles for the interface objects: [`INTERFACE_GET_1DC_OBJECT`] answers
    /// `0x7100`, [`INTERFACE_GET_178_OBJECT`] `0x7200` and
    /// [`INTERFACE_GET_PIPBOY`] `0x7300`; `0044ac20` copies the word at its
    /// argument to `this`.
    fn interface_doubles(e: &mut Engine) {
        e.map(0x7000, 0x1000);
        e.register(INTERFACE_GET_1DC_OBJECT, |_, _| 0x7100u32.into_ret());
        e.register(INTERFACE_GET_178_OBJECT, |_, _| 0x7200u32.into_ret());
        e.register(INTERFACE_GET_PIPBOY, |_, _| 0x7300u32.into_ret());
        e.register(FN_0044AC20, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
    }

    /// `ParseParameters` double for commands whose arguments are all
    /// `float`s: the values are given as `f32`.
    fn parse_gives_floats(e: &mut Engine, values: &[f32]) {
        let bits: Vec<u32> = values.iter().map(|v| v.to_bits()).collect();
        parse_gives(e, true, &bits);
    }

    #[test]
    fn set_lod_object_distance_sets_the_settings_and_updates_the_terrain() {
        let mut e = engine_d();
        e.map(0x7000, 0x1000);
        e.set_global(GLOBAL_0011DEA10, 0x7000u32);
        e.set_global(GLOBAL_0011DEA0C, 0x7400u32);
        let player = object_answering(&mut e, &[(PLAYER_POSITION_SLOT, 0x7800)]);
        e.set_global(PLAYER, player);
        e.register(TES_GET_WORLD_SPACE, |_, a| (a[0] + 1).into_ret());
        e.register(WORLD_SPACE_GET_TERRAIN_MANAGER, |_, a| {
            assert_eq!(a, [0x7001]);
            0x7500u32.into_ret()
        });
        accept(
            &mut e,
            &[
                SETTING_SET_FLOAT,
                TERRAIN_MANAGER_UPDATE,
                FN_00558310,
                FN_0043C490,
            ],
        );
        e.register(FN_0045C670, |_, _| 0x7600u32.into_ret());
        parse_gives_floats(&mut e, &[250.0, 600.0]);
        // The singleton's byte at +6 is zero: the player's position is read
        // twice (the unused local vector, then the update).
        start_log(&mut e);
        assert!(e.call(0x005b_b390, &args![script(0)]).bool());
        assert_eq!(
            calls(&e, SETTING_SET_FLOAT),
            vec![
                vec![SETTING_LOD_DISTANCE_NEAR, 250.0f32.to_bits()],
                vec![SETTING_LOD_DISTANCE_FAR, 600.0f32.to_bits()]
            ]
        );
        assert_eq!(
            calls(&e, TERRAIN_MANAGER_UPDATE),
            vec![vec![0x7500, 0x7800, 1]]
        );
        let position_calls = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(_, words)| words.first() == Some(&player))
            .count();
        assert_eq!(position_calls, 2);
        assert!(calls(&e, FN_00558310).is_empty());
        // The byte at +6 set: the cell position route, and one player read.
        e.mem.set_u8(0x7406, 1);
        e.register(FN_00558310, |_, a| {
            assert_eq!(a, [0x7600]);
            0x7700u32.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005b_b390, &args![script(0)]).bool());
        assert_eq!(calls(&e, FN_0043C490), vec![vec![0x7700]]);
        let position_calls = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(_, words)| words.first() == Some(&player))
            .count();
        assert_eq!(position_calls, 1);
    }

    #[test]
    fn set_lod_object_distance_without_parsing_or_a_terrain_manager() {
        let mut e = engine_d();
        e.map(0x7000, 0x1000);
        e.set_global(GLOBAL_0011DEA10, 0x7000u32);
        e.register(TES_GET_WORLD_SPACE, |_, _| 0u32.into_ret());
        e.register(WORLD_SPACE_GET_TERRAIN_MANAGER, |_, _| 0u32.into_ret());
        accept(&mut e, &[SETTING_SET_FLOAT]);
        // Bad arguments.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_b390, &args![script(0)]).bool());
        // The first word zero: not parsed; no terrain manager: nothing set.
        let unparsed = ScriptArgs {
            param_info: 0,
            ..script(0)
        };
        start_log(&mut e);
        assert!(e.call(0x005b_b390, &args![unparsed]).bool());
        assert!(calls(&e, PARSE_PARAMETERS).is_empty());
        assert!(calls(&e, SETTING_SET_FLOAT).is_empty());
        // A terrain manager with the first word zero: both values are 0.0.
        e.register(WORLD_SPACE_GET_TERRAIN_MANAGER, |_, _| 0x7500u32.into_ret());
        e.set_global(GLOBAL_0011DEA0C, 0x7400u32);
        let player = object_answering(&mut e, &[(PLAYER_POSITION_SLOT, 1)]);
        e.set_global(PLAYER, player);
        accept(&mut e, &[TERRAIN_MANAGER_UPDATE]);
        start_log(&mut e);
        assert!(e.call(0x005b_b390, &args![unparsed]).bool());
        assert_eq!(
            calls(&e, SETTING_SET_FLOAT),
            vec![
                vec![SETTING_LOD_DISTANCE_NEAR, 0],
                vec![SETTING_LOD_DISTANCE_FAR, 0]
            ]
        );
    }

    #[test]
    fn byte_accessor_at_offset_six() {
        let mut e = engine_d();
        let object = e.mem.alloc(16);
        e.mem.set_u8(object + 6, 0x5a);
        assert_eq!(e.call(0x005b_b4d0, &args![object]).u8(), 0x5a);
    }

    #[test]
    fn three_float_command_passes_the_values_on() {
        let mut e = engine_d();
        accept(&mut e, &[FN_007E4FE0]);
        // Defaults: 0.0, 0.0, 1.0.
        parse_gives(&mut e, true, &[]);
        start_log(&mut e);
        assert!(e.call(0x005b_b4f0, &args![script(0)]).bool());
        assert_eq!(calls(&e, FN_007E4FE0), vec![vec![0, 0, 1.0f32.to_bits()]]);
        // Parsed values (four outputs are passed).
        parse_gives_floats(&mut e, &[2.0, 3.0, 4.0, 9.0]);
        start_log(&mut e);
        assert!(e.call(0x005b_b4f0, &args![script(0)]).bool());
        assert_eq!(calls(&e, PARSE_PARAMETERS)[0].len(), 11);
        assert_eq!(
            calls(&e, FN_007E4FE0),
            vec![vec![2.0f32.to_bits(), 3.0f32.to_bits(), 4.0f32.to_bits()]]
        );
        // Bad arguments.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_b4f0, &args![script(0)]).bool());
        assert!(calls(&e, FN_007E4FE0).is_empty());
    }

    #[test]
    fn interface_object_commands_need_a_reference_with_an_object() {
        let mut e = engine_d();
        interface_doubles(&mut e);
        e.set_global(F32_0101E580, 15.0f32);
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32 as u32).into_ret());
        accept(
            &mut e,
            &[FN_00800AC0, FN_00800D50, FN_00800E50, FN_00800ED0],
        );
        let target = object_answering(&mut e, &[(0x1d0, 0x6000)]);
        let no_object = object_answering(&mut e, &[(0x1d0, 0)]);

        // 005bb570: the value (-2.75, truncated to -2) and the reference.
        parse_gives_floats(&mut e, &[-2.75]);
        e.mem.set_u32(0x7100, 0xdead);
        start_log(&mut e);
        assert!(e.call(0x005b_b570, &args![script(target)]).bool());
        assert_eq!(calls(&e, PARSE_PARAMETERS)[0].len(), 11);
        assert_eq!(
            calls(&e, FN_00800AC0),
            vec![vec![0x7100, target, (-2i32) as u32, 0]]
        );
        assert_eq!(e.mem.u32(0x7100), 0);
        // The default is the global 15.0.
        parse_gives(&mut e, true, &[]);
        start_log(&mut e);
        assert!(e.call(0x005b_b570, &args![script(target)]).bool());
        assert_eq!(calls(&e, FN_00800AC0), vec![vec![0x7100, target, 15, 0]]);
        // No reference, or one without an object: nothing.
        for this_obj in [0, no_object] {
            e.mem.set_u32(0x7100, 0xdead);
            start_log(&mut e);
            assert!(e.call(0x005b_b570, &args![script(this_obj)]).bool());
            assert!(calls(&e, FN_00800AC0).is_empty());
            assert_eq!(e.mem.u32(0x7100), 0xdead);
        }
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_b570, &args![script(target)]).bool());

        // 005bb610: the reference to two methods, then the word 1.
        parse_gives(&mut e, true, &[]);
        start_log(&mut e);
        assert!(e.call(0x005b_b610, &args![script(target)]).bool());
        assert_eq!(calls(&e, FN_00800D50), vec![vec![0x7100, target]]);
        assert_eq!(calls(&e, FN_00800E50), vec![vec![0x7100, target]]);
        assert_eq!(e.mem.u32(0x7100), 1);
        e.mem.set_u32(0x7100, 0xdead);
        start_log(&mut e);
        assert!(e.call(0x005b_b610, &args![script(no_object)]).bool());
        assert!(calls(&e, FN_00800D50).is_empty());
        assert_eq!(e.mem.u32(0x7100), 0xdead);
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_b610, &args![script(target)]).bool());

        // 005bb6c0: zeros to one method, another one, then the word 0.
        parse_gives(&mut e, true, &[]);
        e.mem.set_u32(0x7100, 0xdead);
        start_log(&mut e);
        assert!(e.call(0x005b_b6c0, &args![script(target)]).bool());
        assert_eq!(calls(&e, FN_00800AC0), vec![vec![0x7100, 0, 0, 0]]);
        assert_eq!(calls(&e, FN_00800ED0), vec![vec![0x7100]]);
        assert_eq!(e.mem.u32(0x7100), 0);
        start_log(&mut e);
        assert!(e.call(0x005b_b6c0, &args![script(0)]).bool());
        assert!(calls(&e, FN_00800ED0).is_empty());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_b6c0, &args![script(target)]).bool());
    }

    #[test]
    fn interface_float_command_hands_three_values_to_the_object() {
        let mut e = engine_d();
        interface_doubles(&mut e);
        accept(&mut e, &[FN_007F7880]);
        e.set_global(F32_01013D84, 500.0f32);
        e.set_global(F32_0101E2BC, 0.1f32);
        e.set_global(F32_010181BC, 0.05f32);
        parse_gives(&mut e, true, &[]);
        start_log(&mut e);
        assert!(e.call(0x005b_b770, &args![script(0)]).bool());
        assert_eq!(
            calls(&e, FN_007F7880),
            vec![vec![
                0x7200,
                500.0f32.to_bits(),
                0.1f32.to_bits(),
                0.05f32.to_bits()
            ]]
        );
        parse_gives_floats(&mut e, &[1.0, 2.0, 3.0, 4.0]);
        start_log(&mut e);
        assert!(e.call(0x005b_b770, &args![script(0)]).bool());
        assert_eq!(
            calls(&e, FN_007F7880),
            vec![vec![
                0x7200,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits()
            ]]
        );
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_b770, &args![script(0)]).bool());
        assert!(calls(&e, FN_007F7880).is_empty());
    }

    #[test]
    fn move_pipboy_knob_passes_the_knob_the_amount_and_the_step() {
        let mut e = engine_d();
        interface_doubles(&mut e);
        accept(&mut e, &[FN_007F8610]);
        e.set_global(F32_010780A4, 0.0005f32);
        e.register_double(PARSE_PARAMETERS, |e, a| {
            assert_eq!(a.len(), 9);
            e.mem.set_u32(a[7], 3);
            e.mem.set_u32(a[8], (-0.5f32).to_bits());
            true.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x005b_b800, &args![script(0)]).bool());
        assert_eq!(
            calls(&e, FN_007F8610),
            vec![vec![0x7300, 3, (-0.5f32).to_bits(), 0.0005f32.to_bits(), 1]]
        );
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_b800, &args![script(0)]).bool());
        assert!(calls(&e, FN_007F8610).is_empty());
    }

    #[test]
    fn float_setting_command_stores_the_value_in_the_global_object() {
        let mut e = engine_d();
        e.register(FLOAT_VALUE_ADDRESS, |_, a| {
            assert_eq!(a, [GLOBAL_011E090C]);
            (a[0] + 4).into_ret()
        });
        parse_gives_floats(&mut e, &[2.5]);
        assert!(e.call(0x005b_b870, &args![script(0)]).bool());
        assert_eq!(e.mem.f32(GLOBAL_011E090C + 4), 2.5);
        parse_gives(&mut e, false, &[]);
        e.mem.set_f32(GLOBAL_011E090C + 4, 7.0);
        assert!(!e.call(0x005b_b870, &args![script(0)]).bool());
        assert_eq!(e.mem.f32(GLOBAL_011E090C + 4), 7.0);
    }

    #[test]
    fn position_copy_reads_the_three_words_at_0x1f0() {
        let mut e = engine_d();
        let object = e.mem.alloc(0x210);
        for (i, word) in [11u32, 22, 33].iter().enumerate() {
            e.mem.set_u32(object + 0x1f0 + 4 * i as u32, *word);
        }
        let out = e.mem.alloc(16);
        assert_eq!(e.call(0x005b_b9b0, &args![object, out]).u32(), out);
        assert_eq!(read_vector(&e, out), [11, 22, 33]);
    }

    #[test]
    fn debug_marker_command_flips_or_sets_its_byte() {
        let mut e = engine_d();
        e.set_global(FLOAT_MINUS_ONE, -1.0f32);
        e.register(FN_00450B80, |e, a| {
            assert_eq!(a, [0]);
            let object = e.mem.u32(0x0126_8500);
            object.into_ret()
        });
        let object = e.mem.alloc(0x210);
        for (i, word) in [1u32, 2, 3].iter().enumerate() {
            e.mem.set_u32(object + 0x1f0 + 4 * i as u32, *word);
        }
        e.mem.set_u32(0x0126_8500, object);
        accept(&mut e, &[FN_00B5A8C0]);
        // Defaults (-1.0, -1.0): both below zero, so the byte is flipped.
        parse_gives(&mut e, true, &[]);
        start_log(&mut e);
        assert!(e.call(0x005b_b8c0, &args![script(0)]).bool());
        assert_eq!(e.global::<u8>(FLAG_011CAE47), 1);
        let minus_one = (-1.0f32).to_bits();
        assert_eq!(
            calls(&e, FN_00B5A8C0),
            vec![vec![object, 1, 1, 2, 3, minus_one, minus_one]]
        );
        assert!(e.call(0x005b_b8c0, &args![script(0)]).bool());
        assert_eq!(e.global::<u8>(FLAG_011CAE47), 0);
        // Zero is not above zero: the byte is flipped again.
        parse_gives_floats(&mut e, &[0.0, -0.0]);
        assert!(e.call(0x005b_b8c0, &args![script(0)]).bool());
        assert_eq!(e.global::<u8>(FLAG_011CAE47), 1);
        // A value above zero (either one) sets the byte whatever it was.
        for values in [[2.0f32, -1.0], [-1.0, 0.25], [f32::NAN, -1.0]] {
            e.set_global(FLAG_011CAE47, 0u8);
            parse_gives_floats(&mut e, &values);
            assert!(e.call(0x005b_b8c0, &args![script(0)]).bool());
            assert_eq!(e.global::<u8>(FLAG_011CAE47), 1, "{values:?}");
            e.set_global(FLAG_011CAE47, 1u8);
            assert!(e.call(0x005b_b8c0, &args![script(0)]).bool());
            assert_eq!(e.global::<u8>(FLAG_011CAE47), 1, "{values:?}");
        }
        // The two values reach the callee in parse order.
        parse_gives_floats(&mut e, &[2.0, 4.0]);
        start_log(&mut e);
        assert!(e.call(0x005b_b8c0, &args![script(0)]).bool());
        assert_eq!(
            calls(&e, FN_00B5A8C0),
            vec![vec![object, 1, 1, 2, 3, 2.0f32.to_bits(), 4.0f32.to_bits()]]
        );
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_b8c0, &args![script(0)]).bool());
    }

    /// Doubles for `005bb9e0`: the list owner object (with a position at
    /// `+0x1f0`), the list and the construction calls, which are logged.
    fn marker_doubles(e: &mut Engine) -> u32 {
        e.set_global(FLOAT_MINUS_ONE, -1.0f32);
        e.set_global(F32_0101B268, 50.0f32);
        e.set_global(FLOAT_HALF_PI, std::f32::consts::FRAC_PI_2);
        let owner = e.mem.alloc(0x210);
        for (i, word) in [1.5f32, 2.5, 3.5].iter().enumerate() {
            e.mem.set_f32(owner + 0x1f0 + 4 * i as u32, *word);
        }
        e.mem.set_u32(0x0126_8500, owner);
        e.register(NI_POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        e.register(FN_00450B80, |e, a| {
            assert_eq!(a, [0]);
            e.mem.u32(0x0126_8500).into_ret()
        });
        e.register(FN_00454B30, |_, a| (a[0] + 0x10).into_ret());
        accept(
            e,
            &[
                FN_004ED900,
                TRIVIAL_CONSTRUCT,
                NI_MATRIX_FROM_Z_ANGLE,
                NI_POINTER_DESTRUCT,
                FN_00C5AE70,
            ],
        );
        e.register(FN_0045C670, |_, _| 0x7600u32.into_ret());
        e.register(FN_006629F0, |_, a| {
            assert_eq!(a, [0x7600]);
            0x7700u32.into_ret()
        });
        // The matrix lives at 0x7770 + ..., nine words 1..=9.
        e.map(0x7000, 0x1000);
        e.register(FN_00461130, |_, a| {
            assert_eq!(a, [0x7700]);
            0x7770u32.into_ret()
        });
        for i in 0..9u32 {
            e.mem.set_u32(0x7770 + 4 * i, i + 1);
        }
        // The product: every word 100 + index, returned as the out pointer.
        e.register(NI_MATRIX_MULTIPLY, |e, a| {
            for i in 0..9u32 {
                e.mem.set_u32(a[1] + 4 * i, 100 + i);
            }
            a[1].into_ret()
        });
        e.register(NI_ALLOC, |e, a| {
            assert_eq!(a, [0xfc]);
            e.mem.alloc(0xfc).into_ret()
        });
        // The 0xfc-byte object constructor: records what it was given.
        e.register_double(FN_004A5FF0, |e, a| {
            e.mem.set_u32(0x0126_8600, a[0]);
            for i in 0..3u32 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(0x0126_8610 + 4 * i, word);
            }
            for i in 0..2u32 {
                let word = e.mem.u32(a[2] + 4 * i);
                e.mem.set_u32(0x0126_8620 + 4 * i, word);
            }
            for i in 0..9u32 {
                let word = e.mem.u32(a[3] + 4 * i);
                e.mem.set_u32(0x0126_8630 + 4 * i, word);
            }
            a[0].into_ret()
        });
        e.register(NI_POINTER_INIT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        owner
    }

    #[test]
    fn marker_command_builds_an_object_from_the_position_point_and_matrix() {
        let mut e = engine_d();
        let owner = marker_doubles(&mut e);
        // First value goes to the second component: y = 3.0, x = -2.0 (becomes
        // 50.0).
        parse_gives_floats(&mut e, &[3.0, -2.0]);
        start_log(&mut e);
        assert!(e.call(0x005b_b9e0, &args![script(0)]).bool());
        let block = e.mem.u32(0x0126_8600);
        assert_ne!(block, 0);
        let float_words = |e: &Engine, at: u32, n: u32| -> Vec<f32> {
            (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
        };
        assert_eq!(float_words(&e, 0x0126_8610, 3), vec![1.5, 2.5, 3.5]);
        assert_eq!(float_words(&e, 0x0126_8620, 2), vec![50.0, 3.0]);
        let matrix: Vec<u32> = (0..9).map(|i| e.mem.u32(0x0126_8630 + 4 * i)).collect();
        assert_eq!(matrix, (100..109).collect::<Vec<u32>>());
        // The rotation about z is pi / 2 and the product is of the world matrix.
        assert_eq!(
            calls(&e, NI_MATRIX_FROM_Z_ANGLE)[0][1],
            std::f32::consts::FRAC_PI_2.to_bits()
        );
        assert_eq!(calls(&e, NI_MATRIX_MULTIPLY)[0].len(), 3);
        // The new object is held, added to the owner's list and released.
        let held = calls(&e, FN_00C5AE70);
        assert_eq!(held, vec![vec![owner + 0x10, block]]);
        assert_eq!(calls(&e, NI_POINTER_INIT)[0][1], block);
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT).len(), 1);
        assert!(calls(&e, FN_004ED900).is_empty());
        // A value of 0.0 empties the list instead.
        for values in [[0.0f32, 4.0], [4.0, 0.0]] {
            parse_gives_floats(&mut e, &values);
            start_log(&mut e);
            assert!(e.call(0x005b_b9e0, &args![script(0)]).bool());
            assert_eq!(calls(&e, FN_004ED900), vec![vec![owner + 0x10 + 8]]);
            assert!(calls(&e, NI_ALLOC).is_empty());
        }
        // Bad arguments.
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_b9e0, &args![script(0)]).bool());
        assert!(calls(&e, FN_004ED900).is_empty());
    }

    #[test]
    fn marker_command_keeps_positive_values_and_defaults_negative_ones() {
        let mut e = engine_d();
        marker_doubles(&mut e);
        // Both parsed values positive: kept (y = 7.0 first, x = 6.0 second).
        parse_gives_floats(&mut e, &[7.0, 6.0]);
        assert!(e.call(0x005b_b9e0, &args![script(0)]).bool());
        assert_eq!(e.mem.f32(0x0126_8620), 6.0);
        assert_eq!(e.mem.f32(0x0126_8624), 7.0);
        // Both negative: both become 50.0.
        parse_gives_floats(&mut e, &[-7.0, -6.0]);
        assert!(e.call(0x005b_b9e0, &args![script(0)]).bool());
        assert_eq!(e.mem.f32(0x0126_8620), 50.0);
        assert_eq!(e.mem.f32(0x0126_8624), 50.0);
    }

    #[test]
    fn list_clear_and_small_flag_commands() {
        let mut e = engine_d();
        accept(&mut e, &[FN_004ED900]);
        start_log(&mut e);
        e.call(0x005b_bc10, &args![0x7000u32]);
        assert_eq!(calls(&e, FN_004ED900), vec![vec![0x7008]]);
        assert!(e.call(0x005b_bc30, &args![]).bool());
        assert_eq!(e.global::<u8>(FLAG_011AD884), 1);
        assert!(e.call(0x005b_bc30, &args![]).bool());
        assert_eq!(e.global::<u8>(FLAG_011AD884), 0);
        e.mem.set_u8(FLAG_011AD884, 9);
        assert!(e.call(0x005b_bc30, &args![]).bool());
        assert_eq!(e.global::<u8>(FLAG_011AD884), 0);
        // Bit 0 of the dword at +0x18.
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x18, 0x10);
        e.call(0x005b_bd60, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object + 0x18), 0x11);
        e.call(0x005b_bd60, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object + 0x18), 0x10);
    }

    #[test]
    fn radial_blur_replaces_the_running_instance() {
        let mut e = engine_d();
        e.set_global(F32_01016248, 0.5f32);
        e.register(NI_POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        accept(&mut e, &[IMAGE_SPACE_MODIFIER_INSTANCE_STOP, CONSOLE_PRINT]);
        e.register_double(RADIAL_BLUR_TRIGGER, |e, a| {
            // The 2D point is read while it lives.
            e.mem.set_u32(0x0126_8700, e.mem.u32(a[5]));
            e.mem.set_u32(0x0126_8704, e.mem.u32(a[5] + 4));
            0x7400u32.into_ret()
        });
        e.map(0x7000, 0x1000);
        e.set_global(RADIAL_BLUR_INSTANCE, 0x7200u32);
        parse_gives_floats(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0]);
        start_log(&mut e);
        assert!(e.call(0x005b_bc50, &args![script(0)]).bool());
        assert_eq!(calls(&e, PARSE_PARAMETERS)[0].len(), 12);
        assert_eq!(
            calls(&e, IMAGE_SPACE_MODIFIER_INSTANCE_STOP),
            vec![vec![0x7200]]
        );
        let trigger = calls(&e, RADIAL_BLUR_TRIGGER);
        assert_eq!(trigger.len(), 1);
        assert_eq!(
            trigger[0][..5],
            [1.0f32, 2.0, 3.0, 4.0, 5.0].map(f32::to_bits)
        );
        assert_eq!(trigger[0][6], 0);
        assert_eq!(e.mem.f32(0x0126_8700), 0.5);
        assert_eq!(e.mem.f32(0x0126_8704), 0.5);
        assert_eq!(e.global::<u32>(RADIAL_BLUR_INSTANCE), 0x7400);
        // Bit 0 of the new instance's +0x18 is set.
        assert_eq!(e.mem.u32(0x7418) & 1, 1);
        assert!(printed(&e).is_empty());
    }

    #[test]
    fn radial_blur_with_bad_arguments_prints_the_usage_and_fails() {
        let mut e = engine_d();
        accept(&mut e, &[IMAGE_SPACE_MODIFIER_INSTANCE_STOP, CONSOLE_PRINT]);
        e.register(RADIAL_BLUR_TRIGGER, |_, _| 0u32.into_ret());
        e.set_global(RADIAL_BLUR_INSTANCE, 0x7200u32);
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        // Not echoing: no usage text, the old instance is still stopped.
        assert!(!e.call(0x005b_bc50, &args![script(0)]).bool());
        assert!(printed(&e).is_empty());
        assert_eq!(calls(&e, IMAGE_SPACE_MODIFIER_INSTANCE_STOP).len(), 1);
        assert_eq!(e.global::<u32>(RADIAL_BLUR_INSTANCE), 0);
        set_echo(&mut e, true);
        assert!(!e.call(0x005b_bc50, &args![script(0)]).bool());
        assert_eq!(printed(&e), vec![vec![MSG_SET_GLOBAL_RADIAL_BLUR_USAGE]]);
        // No old instance: nothing stopped. No new instance: no flag set.
        parse_gives(&mut e, true, &[]);
        start_log(&mut e);
        e.register(NI_POINT2_CONSTRUCT, |_, a| a[0].into_ret());
        assert!(e.call(0x005b_bc50, &args![script(0)]).bool());
        assert!(calls(&e, IMAGE_SPACE_MODIFIER_INSTANCE_STOP).is_empty());
        assert_eq!(e.global::<u32>(RADIAL_BLUR_INSTANCE), 0);
    }

    /// `__RTDynamicCast` double: an object is an actor when the byte at
    /// `+0xf0` is set; a null object gives zero.
    fn actor_cast_double(e: &mut Engine) {
        e.register(DYNAMIC_CAST, |e, a| {
            assert_eq!(a[1..], [0, RTTI_TES_OBJECT_REFR, RTTI_ACTOR, 0]);
            (if a[0] != 0 && e.mem.u8(a[0] + 0xf0) != 0 {
                a[0]
            } else {
                0
            })
            .into_ret()
        });
        e.register(REFR_GET_NAME_TEXT, |_, a| (a[0] + 0x1000).into_ret());
    }

    #[test]
    fn ai_detection_toggle_flips_its_byte() {
        let mut e = engine_d();
        accept(&mut e, &[CONSOLE_PRINT]);
        start_log(&mut e);
        assert!(e.call(0x005b_bda0, &args![]).bool());
        assert_eq!(e.global::<u8>(AI_DETECTION_FLAG), 1);
        assert!(printed(&e).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005b_bda0, &args![]).bool());
        assert_eq!(e.global::<u8>(AI_DETECTION_FLAG), 0);
        assert_eq!(
            printed(&e),
            vec![vec![MSG_AI_DETECTION_IS, TEXT_OFF_CAPITAL]]
        );
        assert!(e.call(0x005b_bda0, &args![]).bool());
        assert_eq!(printed(&e)[1], vec![MSG_AI_DETECTION_IS, TEXT_ON_CAPITAL]);
    }

    #[test]
    fn actor_ai_flag_commands_work_on_actors_only() {
        let mut e = engine_d();
        actor_cast_double(&mut e);
        accept(&mut e, &[CONSOLE_PRINT]);
        set_echo(&mut e, true);
        let actor = reference(&mut e, &[], true);
        let other = reference(&mut e, &[], false);
        // 005bbe10 flips the byte and echoes the new state.
        start_log(&mut e);
        assert!(e.call(0x005b_be10, &args![script(actor)]).bool());
        assert_eq!(e.mem.u8(actor + 0xbc), 1);
        assert_eq!(
            printed(&e),
            vec![vec![MSG_PROCESSING_IS, actor + 0x1000, TEXT_ON_CAPITAL]]
        );
        assert!(e.call(0x005b_be10, &args![script(actor)]).bool());
        assert_eq!(e.mem.u8(actor + 0xbc), 0);
        assert_eq!(printed(&e)[1][2], TEXT_OFF_CAPITAL);
        // 005bbec0 writes 1.0 when the byte is zero, 0.0 otherwise.
        let result = e.mem.alloc(8);
        let with_result = ScriptArgs {
            result: Ptr::new(result),
            ..script(actor)
        };
        assert!(e.call(0x005b_bec0, &args![with_result]).bool());
        assert_eq!(e.mem.f64(result), 1.0);
        e.mem.set_u8(actor + 0xbc, 5);
        assert!(e.call(0x005b_bec0, &args![with_result]).bool());
        assert_eq!(e.mem.f64(result), 0.0);
        assert_eq!(e.mem.u8(actor + 0xbc), 5);
        // Not an actor, or no reference: nothing.
        start_log(&mut e);
        for this_obj in [other, 0] {
            assert!(e.call(0x005b_be10, &args![script(this_obj)]).bool());
            assert!(e.call(0x005b_bec0, &args![script(this_obj)]).bool());
        }
        assert!(printed(&e).is_empty());
        assert_eq!(e.mem.u8(other + 0xbc), 0);
    }

    #[test]
    fn toggle_ai_works_on_an_actor_or_on_all_ai() {
        let mut e = engine_d();
        actor_cast_double(&mut e);
        accept(
            &mut e,
            &[
                CONSOLE_PRINT,
                ACTOR_END_MOVEMENT,
                PROCESS_LISTS_TURN_OFF_MOVEMENT_IN_HIGH,
            ],
        );
        set_echo(&mut e, true);
        let actor = reference(&mut e, &[], true);
        let other = reference(&mut e, &[], true);
        // The reference the command runs on, as an actor.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005b_bf70, &args![script(actor)]).bool());
        assert_eq!(e.mem.u8(actor + 0xbc), 1);
        assert_eq!(calls(&e, ACTOR_END_MOVEMENT), vec![vec![actor]]);
        assert_eq!(
            printed(&e),
            vec![vec![MSG_PROCESSING_IS, actor + 0x1000, TEXT_ON_CAPITAL]]
        );
        assert!(calls(&e, DYNAMIC_CAST).len() == 1);
        // A parsed actor wins and is not cast.
        parse_gives(&mut e, true, &[other]);
        start_log(&mut e);
        assert!(e.call(0x005b_bf70, &args![script(actor)]).bool());
        assert_eq!(e.mem.u8(other + 0xbc), 1);
        assert_eq!(e.mem.u8(actor + 0xbc), 1);
        assert!(calls(&e, DYNAMIC_CAST).is_empty());
        // Neither: all AI processing is flipped; turned off, movement stops.
        parse_gives(&mut e, true, &[0]);
        start_log(&mut e);
        assert!(e.call(0x005b_bf70, &args![script(0)]).bool());
        assert_eq!(e.global::<u8>(ALL_AI_FLAG), 1);
        assert_eq!(
            printed(&e),
            vec![vec![MSG_ALL_AI_PROCESSING_IS, TEXT_ON_CAPITAL]]
        );
        assert!(calls(&e, PROCESS_LISTS_TURN_OFF_MOVEMENT_IN_HIGH).is_empty());
        assert!(e.call(0x005b_bf70, &args![script(0)]).bool());
        assert_eq!(e.global::<u8>(ALL_AI_FLAG), 0);
        assert_eq!(printed(&e)[1][1], TEXT_OFF_CAPITAL);
        assert_eq!(
            calls(&e, PROCESS_LISTS_TURN_OFF_MOVEMENT_IN_HIGH),
            vec![vec![PROCESS_LISTS]]
        );
        // Bad arguments.
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_bf70, &args![script(actor)]).bool());
    }

    #[test]
    fn combat_ai_and_high_process_toggles() {
        let mut e = engine_d();
        accept(
            &mut e,
            &[CONSOLE_PRINT, PROCESS_LISTS_TURN_OFF_MOVEMENT_IN_HIGH],
        );
        start_log(&mut e);
        // The flip returns the new value.
        assert!(e.call(0x005b_c130, &args![]).bool());
        assert!(!e.call(0x005b_c130, &args![]).bool());
        assert!(e.call(0x005b_c130, &args![]).bool());
        assert_eq!(e.global::<u8>(COMBAT_AI_FLAG), 1);
        // 005bc0d0 prints the new state only when echoing.
        assert!(e.call(0x005b_c0d0, &args![]).bool());
        assert_eq!(e.global::<u8>(COMBAT_AI_FLAG), 0);
        assert!(printed(&e).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005b_c0d0, &args![]).bool());
        assert_eq!(
            printed(&e),
            vec![vec![MSG_ALL_COMBAT_AI_PROCESSING_IS, TEXT_ON_CAPITAL]]
        );
        // High process: the movement is turned off when the byte became zero.
        assert!(e.call(0x005b_c3b0, &args![]).bool());
        assert_eq!(e.global::<u8>(HIGH_PROCESS_FLAG), 1);
        assert_eq!(
            printed(&e)[1],
            vec![MSG_HIGH_AI_PROCESSING_IS, TEXT_ON_CAPITAL]
        );
        assert!(calls(&e, PROCESS_LISTS_TURN_OFF_MOVEMENT_IN_HIGH).is_empty());
        assert!(e.call(0x005b_c3b0, &args![]).bool());
        assert_eq!(e.global::<u8>(HIGH_PROCESS_FLAG), 0);
        assert_eq!(
            printed(&e)[2],
            vec![MSG_HIGH_AI_PROCESSING_IS, TEXT_OFF_CAPITAL]
        );
        assert_eq!(
            calls(&e, PROCESS_LISTS_TURN_OFF_MOVEMENT_IN_HIGH),
            vec![vec![PROCESS_LISTS]]
        );
    }

    #[test]
    fn int_flag_commands_parse_one_int() {
        let mut e = engine_d();
        // 005bc150 only reports whether the arguments parse.
        parse_gives(&mut e, true, &[7]);
        assert!(e.call(0x005b_c150, &args![script(0)]).bool());
        parse_gives(&mut e, false, &[]);
        assert!(!e.call(0x005b_c150, &args![script(0)]).bool());
        // 005bc1a0 and 005bc200 store whether the int is non-zero.
        for (addr, flag) in [
            (0x005b_c1a0u32, FLAG_011DF678),
            (0x005b_c200, FLAG_012682F8),
        ] {
            parse_gives(&mut e, true, &[5]);
            assert!(e.call(addr, &args![script(0)]).bool());
            assert_eq!(e.global::<u8>(flag), 1);
            parse_gives(&mut e, true, &[0]);
            assert!(e.call(addr, &args![script(0)]).bool());
            assert_eq!(e.global::<u8>(flag), 0);
            e.set_global(flag, 1u8);
            parse_gives(&mut e, false, &[]);
            assert!(!e.call(addr, &args![script(0)]).bool());
            assert_eq!(e.global::<u8>(flag), 1);
        }
    }

    #[test]
    fn mode_command_asks_the_player_for_the_modes_that_use_it() {
        let mut e = engine_d();
        e.set_global(PLAYER, 0x7000u32);
        accept(&mut e, &[FN_00BA8B60]);
        // (mode, expected question, answer, expected flag)
        for (mode, question, answer, flag) in [
            (4i32, None, 1u8, 0u32),
            (1, Some(1u32), 1, 1),
            (2, Some(1), 1, 1),
            (3, Some(0), 1, 1),
            (5, Some(1), 0, 0),
            (0, Some(0), 1, 0),
            (-3, Some(0), 1, 0),
            (6, Some(0), 0, 0),
        ] {
            e.register_double(FN_00961E30, move |_, _| answer.into_ret());
            parse_gives(&mut e, true, &[mode as u32]);
            start_log(&mut e);
            assert!(e.call(0x005b_c260, &args![script(0)]).bool());
            let asked = calls(&e, FN_00961E30);
            match question {
                None => assert!(asked.is_empty(), "mode {mode}"),
                Some(q) => assert_eq!(asked, vec![vec![0x7000, q]], "mode {mode}"),
            }
            assert_eq!(calls(&e, FN_00BA8B60), vec![vec![flag]], "mode {mode}");
        }
        parse_gives(&mut e, false, &[]);
        start_log(&mut e);
        assert!(!e.call(0x005b_c260, &args![script(0)]).bool());
        assert!(calls(&e, FN_00BA8B60).is_empty());
    }

    #[test]
    fn head_track_target_is_reported_for_actors() {
        let mut e = engine_d();
        actor_cast_double(&mut e);
        accept(&mut e, &[CONSOLE_PRINT]);
        let actor = reference(&mut e, &[], true);
        let other = reference(&mut e, &[], false);
        // The extra data list is `actor + 0x44` (the engine() double).
        e.register_double(HEAD_TRACK_TARGET_EXTRA, move |_, a| {
            assert_eq!(a[0], actor + 0x44);
            0u32.into_ret()
        });
        // No echo: the lookup happens, nothing is printed.
        start_log(&mut e);
        assert!(e.call(0x005b_c310, &args![script(actor)]).bool());
        assert_eq!(calls(&e, HEAD_TRACK_TARGET_EXTRA).len(), 1);
        assert!(printed(&e).is_empty());
        set_echo(&mut e, true);
        assert!(e.call(0x005b_c310, &args![script(actor)]).bool());
        assert_eq!(
            printed(&e),
            vec![vec![MSG_NOT_LOOKING_AT_ANYONE, actor + 0x1000]]
        );
        // With a target: both names, the actor's first.
        e.register_double(HEAD_TRACK_TARGET_EXTRA, move |_, _| other.into_ret());
        assert!(e.call(0x005b_c310, &args![script(actor)]).bool());
        assert_eq!(
            printed(&e)[1],
            vec![MSG_LOOKING_AT, actor + 0x1000, other + 0x1000]
        );
        // Not an actor, or no reference: nothing is looked up.
        start_log(&mut e);
        assert!(e.call(0x005b_c310, &args![script(other)]).bool());
        assert!(e.call(0x005b_c310, &args![script(0)]).bool());
        assert!(calls(&e, HEAD_TRACK_TARGET_EXTRA).is_empty());
    }
}

#[cfg(test)]
mod n045_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object whose vtable maps the given slots to the given addresses.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let o = e.mem.alloc(0x20);
        let vt = e.mem.alloc(0x420);
        e.mem.set_u32(o, vt);
        for (slot, target) in slots {
            e.mem.set_u32(vt + slot, *target);
        }
        o
    }

    /// Script arguments with a result double.
    fn script_args(e: &mut Engine, this: u32) -> (ScriptArgs, u32) {
        let result = e.mem.alloc(8);
        e.mem.set_f64(result, 7.0);
        (
            ScriptArgs {
                param_info: 1,
                script_data: 2,
                this_obj: Ptr::new(this),
                containing_obj: Ptr::NULL,
                script_obj: Ptr::NULL,
                event_list: 0,
                result: Ptr::new(result),
                opcode_offset: 3,
            },
            result,
        )
    }

    fn words(a: ScriptArgs) -> Vec<u32> {
        let mut out = vec![];
        a.put(&mut out);
        out
    }

    /// A parse double writing `values` to the outputs (after the seven fixed words).
    fn parse_writes(e: &mut Engine, values: Vec<u32>) {
        e.register_double(PARSE_PARAMETERS, move |e, a| {
            for (i, v) in values.iter().enumerate() {
                e.mem.set_u32(a[7 + i], *v);
            }
            ret(1)
        });
    }

    fn quiet_all(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, ok);
        }
    }

    fn base_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_0000, 0x2_0000);
        e.map(0x0101_0000, 0x4_0000);
        quiet_all(
            &mut e,
            &[
                N045_SET_POINT,
                N045_SET_MULTI_LAYER_COLLISION,
                N045_PICK_DATA_CONSTRUCT,
                N045_FLAGS_CONSTRUCT,
                N045_FLAGS_SET,
                N045_FLAGS_COMBINE,
                N045_PICK_SET_FILTER,
                N045_PICK_SET_START,
                N045_PICK_SET_END,
                N045_SELF,
                N045_PATHING_LOCATION,
                N045_NAVMESH_NORMAL,
                N045_PATHING_LOCATION_DESTRUCT,
                N045_PICK_NORMAL,
                N045_ROTATION_TO_VECTOR,
                N045_TO_EULER_ANGLES,
                N045_PICK_POINT,
                N045_REFERENCE_FLAG_564C60,
                N045_REFERENCE_FLAG_56C780,
                N045_SET_REF_PERSISTS,
                N045_LIST_ADD,
                N045_MATRIX_GET_ROW,
                N045_MATRIX_SET_ROW,
                N045_MOBILE_931ED0,
                N045_FADE_IN,
                N045_SET_SCALE_LIKE,
                N045_PUT_NUMERIC_ID_IN_DOUBLE,
                N045_INDEX_LIST_CONSTRUCT,
                N045_INDEX_LIST_DESTRUCT,
            ],
        );
        e.register(N045_NEXT_PICK_ID, |_, _| ret(11));
        e.register(N045_REFERENCE_LIST, |_, a| ret(a[0] + 0x10));
        e.register(N045_WORLD_SPACE, |_, _| ret(0x5100));
        e.register(N045_PARENT_CELL, |_, _| ret(0x5200));
        e.register(N045_FLAGS_OF, |_, _| ret(0x22));
        e
    }

    #[test]
    fn fn_005db870_places_a_marker_at_the_pick() {
        let mut e = base_engine();
        let (a, _) = script_args(&mut e, 0);
        e.register(PARSE_PARAMETERS, |_, _| ret(0));
        assert!(!e.call(0x005d_b870, &words(a)).bool());
        parse_writes(&mut e, vec![2]);
        assert!(e.call(0x005d_b870, &words(a)).bool());
        // An actor this reference, no pick hit: placed with the mode 2 form, unrotated.
        let point = e.mem.alloc(0xc);
        e.mem.set_f32(point, 1.0);
        e.mem.set_f32(point + 4, 2.0);
        e.mem.set_f32(point + 8, 3.0);
        e.register_double(0x0050_0000, move |_, _| ret(point));
        e.register(0x0050_0010, |_, _| ret(1));
        e.register(0x0050_0020, ok);
        e.register(0x0050_0030, |_, _| ret(4));
        let this = object(
            &mut e,
            &[
                (0x1f4, 0x0050_0000),
                (0x100, 0x0050_0010),
                (0x48, 0x0050_0020),
                (0x1d0, 0x0050_0030),
            ],
        );
        e.register_double(N045_REFERENCE_POINT, move |_, _| ret(point));
        e.register(N045_TES_PICK, ok);
        e.register(N045_PICK_RESULT_OBJECT, |_, a| ret(a[0]));
        e.register(N045_RESULT_TEST, |_, _| ret(0));
        let placed = object(&mut e, &[(0x48, 0x0050_0020)]);
        e.register_double(N045_PLACE_REFERENCE, move |_, _| ret(placed));
        e.set_global::<u32>(N045_FORM_MODE_2, 0x7200);
        e.set_global::<u32>(N045_FORM_OTHER, 0x7300);
        e.set_global::<u32>(N045_DATA_HANDLER, 0x7400);
        e.set_global::<f64>(N045_HEIGHT_UP, 0.5);
        e.set_global::<f64>(N045_HEIGHT_DOWN, 0.25);
        let heights = std::rc::Rc::new(std::cell::Cell::new((0.0f32, 0.0f32)));
        let h1 = heights.clone();
        e.register_double(N045_PICK_SET_START, move |e, a| {
            h1.set((e.mem.f32(a[1] + 8), h1.get().1));
            Ret::default()
        });
        let h2 = heights.clone();
        e.register_double(N045_PICK_SET_END, move |e, a| {
            h2.set((h2.get().0, e.mem.f32(a[1] + 8)));
            Ret::default()
        });
        let (a, _) = script_args(&mut e, this);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005d_b870, &words(a)).bool());
        let log = e.call_log.take().unwrap();
        let set = only(&log, N045_SET_POINT);
        assert_eq!(&set[0][1..], &[0, 0, 3.0f32.to_bits()]);
        assert_eq!(
            only(&log, N045_SET_MULTI_LAYER_COLLISION),
            vec![vec![0x27, 0x0802_2802, 0]]
        );
        assert_eq!(heights.get(), (3.5, 2.75));
        let place = only(&log, N045_PLACE_REFERENCE);
        assert_eq!(place[0][0], 0x7400);
        assert_eq!(place[0][1], 0x7200);
        assert_eq!(&place[0][4..], &[0x5200, 0x5100, 0, 0, 0]);
        assert_eq!(
            only(&log, N045_REFERENCE_FLAG_564C60),
            vec![vec![placed, 1]]
        );
        assert_eq!(
            only(&log, N045_LIST_ADD),
            vec![vec![placed + 0x10, this], vec![this + 0x10, placed]]
        );
        assert_eq!(only(&log, 0x0050_0020).len(), 2);
        assert!(only(&log, N045_TO_EULER_ANGLES).is_empty());
        // A hit on an accepted object: the navmesh normal and a rotation.
        e.register(N045_RESULT_TEST, |_, _| ret(1));
        e.register(N045_FIRST_WORD, |_, _| ret(0x6600));
        e.register(N045_OBJECT_OF, |_, a| ret(a[0]));
        e.register(N045_OBJECT_TEST, |_, _| ret(1));
        parse_writes(&mut e, vec![1]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005d_b870, &words(a)).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, N045_OBJECT_TEST), vec![vec![0x6600, 0x40]]);
        assert_eq!(only(&log, N045_NAVMESH_NORMAL).len(), 1);
        assert_eq!(only(&log, N045_TO_EULER_ANGLES).len(), 1);
        assert_eq!(
            only(&log, N045_REFERENCE_FLAG_564C60),
            vec![vec![placed, 0]]
        );
        assert_eq!(only(&log, N045_PLACE_REFERENCE)[0][1], 0x7300);
        // A refused object walks its nodes (none here) and uses the pick's normal.
        e.register(N045_OBJECT_TEST, |_, _| ret(0));
        e.register(N045_PICK_RESULT_OBJECT, |e, a| {
            e.mem.set_u32(a[0] + 0x20, 0xffff_ffff);
            ret(a[0])
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x005d_b870, &words(a)).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, N045_INDEX_LIST_CONSTRUCT)[0][1..], [8, 0]);
        assert_eq!(only(&log, N045_PICK_NORMAL).len(), 1);
        assert!(only(&log, N045_LAYER_TEST).is_empty());
    }

    /// Doubles for the player pick of `005df040` and `005df5d0`.
    fn pick_engine() -> Engine {
        let mut e = base_engine();
        let point = e.mem.alloc(0xc);
        e.mem.set_f32(point, 5.0);
        e.register_double(0x0050_0000, move |_, _| ret(point));
        e.register(0x0050_0010, |_, _| ret(1));
        e.register(0x0050_0020, ok);
        let player = object(&mut e, &[(0x1f4, 0x0050_0000), (0x100, 0x0050_0010)]);
        e.set_global::<u32>(N045_PLAYER, player);
        e.set_global::<u32>(N045_TES, 0x7500);
        e.register(N045_ANGLE_FUNCTION, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) * 2.0,
            ..Ret::default()
        });
        e.register(N045_SINGLETON_45C670, |_, _| ret(0x6100));
        e.register(N045_ANIMATION_DATA, |_, _| ret(0x6200));
        let matrix = e.mem.alloc(0x24);
        e.register_double(N045_MATRIX_OF, move |_, _| ret(matrix));
        e.register_double(N045_PLAYER_POINT, move |_, _| ret(point));
        e.register_double(N045_MATRIX_TIMES, move |_, _| ret(point));
        e.register_double(N045_POINT_PLUS, move |_, _| ret(point));
        e.register_double(N045_REFERENCE_POINT, move |_, _| ret(point));
        e.register(N045_TES_PICK, |_, _| ret(0x6300));
        e
    }

    #[test]
    fn fn_005df040_places_the_form_where_the_player_looks() {
        let mut e = pick_engine();
        let health = object(&mut e, &[(0x10, 0x0050_0040)]);
        e.register(0x0050_0040, |_, _| ret(50));
        e.register(0x0050_0030, |_, _| ret(1));
        let form = object(&mut e, &[(0xe4, 0x0050_0030)]);
        e.register_double(N045_HEALTH_FORM, move |_, _| ret(health));
        e.register(N045_PICK_HIT, |_, _| ret(1));
        e.register(0x0050_0050, ok);
        let placed = object(&mut e, &[(0x100, 0x0050_0010), (0x41c, 0x0050_0050)]);
        e.register_double(N045_PLACE_REFERENCE, move |_, _| ret(placed));
        e.set_global::<f64>(N045_HEALTH_SCALE, 0.5);
        e.set_global::<u32>(N045_ANGLE_CONSTANT, 1.0f32.to_bits());
        parse_writes(&mut e, vec![form, 1, 0, 1.5f32.to_bits()]);
        let (a, result) = script_args(&mut e, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005d_f040, &words(a)).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.f64(result), 1.0);
        let angle = only(&log, N045_ANGLE_FUNCTION);
        assert_eq!(angle[0], vec![1.5f32.to_bits(), 1.0f32.to_bits()]);
        let set = only(&log, N045_SET_POINT);
        assert_eq!(&set[0][1..], &[0, 3.0f32.to_bits(), 0]);
        assert_eq!(only(&log, N045_MATRIX_GET_ROW).len(), 3);
        assert_eq!(only(&log, N045_FLAGS_SET)[0][1], 6);
        assert_eq!(only(&log, N045_PLACE_REFERENCE)[0][1], form);
        assert_eq!(only(&log, 0x0050_0050)[0][1], 0);
        assert_eq!(only(&log, N045_FADE_IN), vec![vec![placed]]);
        assert_eq!(
            only(&log, N045_SET_SCALE_LIKE),
            vec![vec![placed, 25.0f32.to_bits()]]
        );
        // No placed reference answers false; a refused form leaves the result 0.
        e.register(N045_PLACE_REFERENCE, |_, _| ret(0));
        assert!(!e.call(0x005d_f040, &words(a)).bool());
        e.register(0x0050_0030, |_, _| ret(0));
        assert!(e.call(0x005d_f040, &words(a)).bool());
        assert_eq!(e.mem.f64(result), 0.0);
        e.register(PARSE_PARAMETERS, |_, _| ret(0));
        assert!(!e.call(0x005d_f040, &words(a)).bool());
    }

    #[test]
    fn fn_005df5d0_answers_the_picked_reference() {
        let mut e = pick_engine();
        parse_writes(&mut e, vec![0, 0]);
        e.register(N045_FIND_REFERENCE_FOR_3D, |_, _| ret(0x6400));
        e.register(N045_FORM_ID, |_, _| ret(0x0001_2345));
        let (a, result) = script_args(&mut e, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005d_f5d0, &words(a)).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, N045_FIND_REFERENCE_FOR_3D), vec![vec![0x6300]]);
        let put = only(&log, N045_PUT_NUMERIC_ID_IN_DOUBLE);
        assert_eq!(put[0][1], result);
        assert_eq!(e.mem.f64(result), 0.0);
        e.register(N045_TES_PICK, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x005d_f5d0, &words(a)).bool());
        let log = e.call_log.take().unwrap();
        assert!(only(&log, N045_FIND_REFERENCE_FOR_3D).is_empty());
    }
}
