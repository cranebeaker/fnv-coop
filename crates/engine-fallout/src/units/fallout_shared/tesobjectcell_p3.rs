//! `fallout shared/tesobjectcell.cpp` (Xbox PDB source unit), part 3: its functions from `00552470` up to
//! (not including) `00558e20` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesobjectcell`]; anything public there may be used here.
//!
//! # Where this file stops
//!
//! The first batch holds the 40 functions from `00552470` to `00553ee0`: a
//! few flag and field accessors the compiler emitted here, the scene-graph
//! walks that update morph controllers (`UpdateMorphControllersOnly`), the
//! pending-node queue and its worker, `AttachToWorld`, `Detach`, the creation
//! of an interior cell's Havok world, the small wrappers of that world
//! (`005533c0` to `005534f0`), the world cinfo destructors, `DetachHavok`,
//! `UpdateRefSounds` and the ray-test helpers (`Pick`).
//!
//! The second batch holds the 40 functions from `00553f70` to `005574d0`:
//! two vector helpers, the creation of an exterior Havok world
//! (`00554010`), the listener constructors and destructors, the exterior
//! world shift (`MoveExteriorWorld`), `GetLandHeight`, `IsFormCellChild` and
//! its type test, the hide and reference-update walks, the cell's save-game
//! size, save and load functions and their save-buffer variants, the north
//! rotation, `GetSeenValue` and `GetIntSeenSection`, and the loops over a
//! cell's references (`QueueReferences` and four others).
//!
//! The third batch holds the last 40 functions, `005575d0` to `00558df0`:
//! the navmesh obstacle walks of attach and detach, the addon node walk
//! (`ProcessAddonNodesForRefs`), the LOD fade walks, `RenderTestCell` with its
//! orientation loop, frame check, statistics and the `SaveRenderFailureData`
//! report file, the lighting template accessors, the held-word helpers and
//! the constructors at the end of the range. This completes the range of the
//! file; the unit's other parts continue elsewhere.
//!
//! # Conventions of the exe worth knowing
//!
//! - A word pushed before a call to a function that pops fewer words
//!   belongs to a later call (`PUSH x; PUSH 0; CALL 00450b80; ADD ESP,4;
//!   MOV ECX,EAX; CALL 00b5ddf0` passes `x` to `00b5ddf0`); every call here
//!   was matched against the callee's `RET n`.
//! - The 12-byte update record `0043d410` builds on the stack is passed by
//!   address to the scene-graph update virtuals (`+0x94`, `+0xc0`) and to
//!   `00a59c60`.
//! - `NiTMap` walk: `004b9ba0` gives the first position, `006b7f20` takes
//!   the addresses of the position and of two out words (key, value) and
//!   advances the position.
//! - Scope guards (`00404eb0`, 4 bytes on the stack, with the source path
//!   and the line of the allocation), C++ exception unwinding and the stack
//!   cookies are modelled only as far as the guard's constructor and
//!   destructor calls.
//! - The `float` results of `005471e0` and `005723b0` come in `ST0`; the
//!   x87 `FISTP` with truncation of `UpdateRefSounds` is reproduced by
//!   [`truncated_low_word`].

#[allow(unused_imports)]
use super::tesobjectcell::*;
#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------------
// Callees outside this file, by exe address.

/// `(word at +0x30) & mask != 0` (`00456630`) and the setter of the same
/// bits `(this, set, mask)` (`0043b370`): the flag word of the scene-graph
/// objects `fn_00552470`, `fn_005526b0` and `fn_005526d0` read.
const TEST_FLAG_BITS: u32 = 0x0045_6630;
const SET_FLAG_BITS: u32 = 0x0043_b370;
/// Returns a pointer to a byte (`this + 4`, or a static zero byte for null).
const BYTE_HOLDER_GET: u32 = 0x0040_8d60;
/// The object whose address `fn_005524d0` hands to `00408d60`.
const BYTE_HOLDER: u32 = 0x0126_7c24;

/// The lock (`011ca1a0`) and the list (`011ca174`) of nodes waiting for the
/// worker `fn_00552570`, and the calls on them.
const PENDING_LOCK: u32 = 0x011c_a1a0;
const PENDING_LIST: u32 = 0x011c_a174;
const LOCK_ENTER: u32 = 0x0040_fbf0;
const LOCK_LEAVE: u32 = 0x0040_fba0;
/// Appends a smart pointer slot (by address) to the pending list.
const PENDING_LIST_ADD: u32 = 0x0063_1540;
/// The list is empty (`004a4460`) and `RemoveHead` (`004ee8a0`); the address
/// of a list node's item is `006815c0`.
const PENDING_LIST_IS_EMPTY: u32 = 0x004a_4460;
const PENDING_LIST_REMOVE_HEAD: u32 = 0x004e_e8a0;
const LIST_NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
const LIST_NODE_NEXT: u32 = 0x0072_6070;
const LIST_NODE_IS_END: u32 = 0x0082_56d0;
/// `NiPointer` slot: construct from a raw pointer, copy-construct from the
/// address of another slot, read the pointer, release.
const SLOT_FROM_RAW: u32 = 0x0063_3c90;
const SLOT_COPY: u32 = 0x0055_9a40;
const SLOT_GET_POINTER: u32 = 0x0055_9450;
const SLOT_RELEASE: u32 = 0x0045_cec0;
/// `TESObjectREFR::FindReferenceFor3D` (Xbox PDB), cdecl.
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// The parent cell of a reference (`008d6f30`) and the cell function the
/// worker calls on it with `(reference, 1, 0)`.
const REFERENCE_GET_PARENT_CELL: u32 = 0x008d_6f30;
const CELL_UPDATE_FOR_REFERENCE: u32 = 0x0054_a070;

/// The 12-byte update record: constructor `(this, time, flag, flag)`.
const UPDATE_RECORD_CONSTRUCT: u32 = 0x0043_d410;
const UPDATE_RECORD_SIZE: u32 = 12;
/// The child array of a node (virtual `+0x0c` gives it), its count and its
/// element at an index.
const NODE_CHILD_COUNT: u32 = 0x0043_b480;
const NODE_CHILD_AT: u32 = 0x0043_b4a0;
/// Applies an update record to a controller (`00a59c60`).
const CONTROLLER_UPDATE: u32 = 0x00a5_9c60;
/// `NiObjectNET::GetController(this, type)` (`00a5c570`), the type the
/// morph walk asks for, and the update time it passes.
const NODE_GET_CONTROLLER: u32 = 0x00a5_c570;
const MORPH_CONTROLLER_TYPE: u32 = 0x011f_3728;
const UPDATE_TIME: u32 = 0x011c_3c08;

/// `NiTMap` walk (see the header) and the node of an animated reference
/// (`009611e0`).
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
const MAP_NEXT: u32 = 0x006b_7f20;
const ANIMATED_NODE_OWNER: u32 = 0x0096_11e0;

/// The cell's node accessors: `00545cf0` is `Load3D`; `005497a0`,
/// `0054a050`, `0054aa60` and `00524cf0` return nodes of the cell;
/// `00456fc0(cell, index)` returns the child of the cell node at `index`.
const CELL_LOAD_3D: u32 = 0x0054_5cf0;
const CELL_NODE_A: u32 = 0x0054_97a0;
const CELL_NODE_B: u32 = 0x0054_a050;
const CELL_NODE_C: u32 = 0x0054_aa60;
const CELL_NODE_D: u32 = 0x0052_4cf0;
const CELL_CHILD_NODE: u32 = 0x0045_6fc0;
/// Cell helpers: the Havok world of the cell (`004543c0`), the cell's
/// collision world (`00537b30`), "is interior" (`00425fd0`), the cell state
/// setter and getter (`004512a0`, `00450fd0`), "is loaded" (`00450ff0`), the
/// cell lock (`00541ac0`, `00541ae0`) and the list of references (`009604f0`).
const CELL_HAVOK_WORLD: u32 = 0x0045_43c0;
const CELL_COLLISION_WORLD: u32 = 0x0053_7b30;
const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
const CELL_SET_STATE: u32 = 0x0045_12a0;
const CELL_GET_STATE: u32 = 0x0045_0fd0;
const CELL_IS_LOADED: u32 = 0x0045_0ff0;
const CELL_LOCK_ENTER: u32 = 0x0054_1ac0;
const CELL_LOCK_LEAVE: u32 = 0x0054_1ae0;
const CELL_REFERENCES: u32 = 0x0096_04f0;
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
const CELL_GET_DATA_X: u32 = 0x0054_4c30;
const CELL_GET_DATA_Y: u32 = 0x0054_4c60;
const CELL_GET_WATER_HEIGHT: u32 = 0x0054_71e0;
const CELL_REFERENCE_CENTER: u32 = 0x0054_c030;
const CELL_SET_LIGHTS_ATTACHED: u32 = 0x0054_ba80;
const CELL_SET_ADDON_NODES_ATTACHED: u32 = 0x0055_77b0;
const CELL_SET_ATTACHED_FLAG: u32 = 0x0054_b950;
const CELL_AFTER_ATTACH_A: u32 = 0x0055_7470;
const CELL_AFTER_ATTACH_B: u32 = 0x0055_75d0;
const CELL_BEFORE_DETACH: u32 = 0x0055_7c40;
const CELL_DETACH_STEP_A: u32 = 0x0055_74d0;
const CELL_DETACH_STEP_B: u32 = 0x0055_76c0;
const CELL_SET_TERRAIN_UPDATE: u32 = 0x0054_af40;
const CELL_CLEAR_LOADED_STATE: u32 = 0x0054_6c70;
const CELL_PORTAL_GRAPH: u32 = 0x0054_6930;
const CELL_OPTIONS: u32 = 0x009d_9f20;
const CELL_REFERENCE_NODES: u32 = 0x0054_5710;
const CELL_UPDATE_REFERENCE_FLAGS: u32 = 0x0048_4730;
const FORM_FLAG_4000: u32 = 0x0040_77c0;
const FORM_FLAG_2: u32 = 0x0046_0340;
const FORM_ID: u32 = 0x0084_e3a0;
/// The same getter (`mov eax,[ecx+0x0c]`) used on the scene singleton, where
/// the word is the node `Detach` detaches the cell's 3D from.
const WORD_AT_0C: u32 = 0x0084_e3a0;
/// `TESObjectREFR::GetBaseObject` (`007af430`) and the form type byte
/// (`00401170`).
const REFERENCE_BASE_OBJECT: u32 = 0x007a_f430;
const FORM_TYPE: u32 = 0x0040_1170;
/// `fn_00552900` / `Detach` call this on the world space of an exterior
/// cell: virtual `+0xc8`.
const WORLD_SPACE_SLOT: u32 = 0xc8;

/// Singletons: the scene (`011dea10`, the object of `tes.cpp`'s methods), the
/// loader (`011ddf38`), the loader state (`011c3f2c`), the model loader
/// (`011c3b3c`), the object `011de134` and the render pointer `011d6e2c`.
const SCENE_SINGLETON: u32 = 0x011d_ea10;
const LOADER_SINGLETON: u32 = 0x011d_df38;
const LOADER_STATE_SINGLETON: u32 = 0x011c_3f2c;
const MODEL_LOADER_SINGLETON: u32 = 0x011c_3b3c;
const STATE_OBJECT_SINGLETON: u32 = 0x011d_e134;
const RENDER_SINGLETON: u32 = 0x011d_6e2c;
/// Scene (`tes.cpp`) calls: hide the cell's world, attach interior and
/// exterior cells.
const SCENE_REFRESH: u32 = 0x0045_2e50;
const SCENE_ATTACH_INTERIOR: u32 = 0x0045_4c20;
const SCENE_ATTACH_EXTERIOR: u32 = 0x0045_5130;
/// Model loader: `CancelReferencesForCell` (Xbox PDB `ModelLoader`).
const MODEL_LOADER_CANCEL_FOR_CELL: u32 = 0x0044_5670;
/// Render calls on `011d6e2c` (`navmeshrender.cpp`): add and remove the
/// draw-only nav meshes of a cell, and the flag it tests.
const NAV_MESH_RENDER_ADD: u32 = 0x006a_37a0;
const NAV_MESH_RENDER_REMOVE: u32 = 0x006a_5bb0;
/// Save/load of the whole game: "the form has changes" test and the call
/// that drops them.
const FORM_HAS_CHANGES: u32 = 0x0046_9860;
const REMOVE_CHANGES: u32 = 0x0084_a7c0;
/// `tes.cpp`'s table of nodes (`00450b80(index)`), the setter of the shadow
/// scene node (`00b5ddf0`) and the child accessor `0045bc00(entry, 5)`.
const TABLE_ENTRY: u32 = 0x0045_0b80;
const SCENE_NODE_ATTACH: u32 = 0x00b5_ddf0;
const SCENE_NODE_CHILD: u32 = 0x0045_bc00;
/// The navmesh obstacle manager: `006c0720` returns it, `006c1170` runs.
const OBSTACLE_MANAGER_GET: u32 = 0x006c_0720;
const OBSTACLE_MANAGER_RUN: u32 = 0x006c_1170;
/// The portal graph (`bsportalgraph.obj`): count of the always-render array
/// and its add, replace and remove operations.
const PORTAL_ALWAYS_RENDER_COUNT: u32 = 0x00c5_b5d0;
const PORTAL_ALWAYS_RENDER_ADD: u32 = 0x00c5_b1c0;
const PORTAL_ALWAYS_RENDER_SET: u32 = 0x00c5_b9a0;
const PORTAL_ALWAYS_RENDER_REMOVE: u32 = 0x00c5_ba50;

/// Havok world (`bhkWorld`, 0xA0 bytes) calls: the visual debugger flag
/// functions, the world's own setters and the collision world accessors.
const WORLD_SET_VISUAL_DEBUGGER_ENABLED: u32 = 0x0045_4380;
const WORLD_SET_VISUAL_DEBUGGER: u32 = 0x00c6_a640;
const VISUAL_DEBUGGER_SETTING: u32 = 0x0045_4ae0;
const VISUAL_DEBUGGER_SUPPRESSED: u32 = 0x0045_6c70;
const SCENE_HAVOK_SETTING: u32 = 0x0045_3720;
const WORLD_SET_GRAVITY_SCALE: u32 = 0x00c6_a620;
const WORLD_SET_CENTER: u32 = 0x00c6_8750;
const WORLD_CONSTRUCT: u32 = 0x00c6_9010;
const WORLD_UPDATE_RENDERER: u32 = 0x00c6_8a00;
const WORLD_COLLISION_OBJECT: u32 = 0x0045_cd60;
const COLLISION_OBJECT_SET_ACTIVE: u32 = 0x0061_f8e0;
const COLLISION_OBJECT_RELEASE: u32 = 0x0048_93c0;
const COLLISION_WORLD_UNLOAD: u32 = 0x0062_1e80;
const COLLISION_WORLD_CONSTRUCT: u32 = 0x0062_1d10;
/// The `011ca088` object, its constructor and the registration call.
const LISTENER_SINGLETON: u32 = 0x011c_a088;
const LISTENER_CONSTRUCT: u32 = 0x0062_4150;
const LISTENER_REGISTER: u32 = 0x0062_42a0;
/// Cell extra data (`this + 0x28`): store the Havok world and the collision
/// world.
const EXTRA_LIST_OFFSET: u32 = 0x28;
const EXTRA_LIST_SET_HAVOK_WORLD: u32 = 0x0041_b8d0;
const EXTRA_LIST_SET_COLLISION_WORLD: u32 = 0x0041_ba60;
/// The object a world holder gives (`00541c80`); `00483710`, a function whose body is empty (the wrappers call it around their setters, and `UpdateRefSounds` on its sound handle at the end); the world setters.
const HOLDER_GET_WORLD: u32 = 0x0054_1c80;
const EMPTY_FUNCTION: u32 = 0x0048_3710;
const WORLD_SET_GRAVITY_VECTOR: u32 = 0x00c9_3820;
const WORLD_SET_FIELD_A: u32 = 0x00c9_3a20;
const WORLD_SET_FIELD_B: u32 = 0x00c9_3860;
const HOLDER_FIELD_70_ADD: u32 = 0x0062_99c0;
/// Objects the interior world is made of: allocation size and constructor.
const BROAD_PHASE_CONSTRUCT: u32 = 0x0062_d5c0;
const FILTER_CONSTRUCT: u32 = 0x0061_f460;
const WATER_CONSTRUCT: u32 = 0x0062_e630;
const LISTENER_OBJECT_CONSTRUCT: u32 = 0x0062_a460;
const HAS_WATER_TEST: u32 = 0x0045_18e0;
/// The world cinfo (`hkpWorldCinfo`): constructor, setters, the 16-byte
/// vector constructor and the stack block it needs.
const CINFO_CONSTRUCT: u32 = 0x00c6_81c0;
const CINFO_SET_GRAVITY: u32 = 0x004a_3f10;
const CINFO_SET_BROAD_PHASE_SIZE: u32 = 0x00c9_0db0;
const CINFO_SIZE: u32 = 0xf0;
const GRAVITY_Z: u32 = 0x011c_a158;
const BROAD_PHASE_SIZE: u32 = 0x0102_efc0;
/// The cinfo destructor chain.
const CINFO_VTABLE: u32 = 0x0102_f0ec;
const CINFO_MEMBER_DESTRUCT: u32 = 0x0055_8e80;
const CINFO_BASE_DESTRUCT: u32 = 0x0053_8e10;
const CINFO_FREE: u32 = 0x0053_8eb0;
/// Terrain transform of an exterior cell: the transform constructor, the
/// identity rotation, the flags word, `2048.0` as a double and the call.
const TRANSFORM_CONSTRUCT: u32 = 0x0047_6a80;
const IDENTITY_ROTATION: u32 = 0x011a_9448;
const TERRAIN_FLAGS: u32 = 0x011a_9ea4;
const HALF_CELL_SIZE: u32 = 0x0101_6968;
const WORLD_ADD_TERRAIN: u32 = 0x0062_2590;

/// Memory manager: the object (`00401020`), `Allocate` and `Deallocate`;
/// `operator new(size)` (`00401000`) and the allocator `00aa13e0` (cdecl).
const MEMORY_MANAGER_GET: u32 = 0x0040_1020;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
const OPERATOR_NEW: u32 = 0x0040_1000;
const ALLOCATE_CDECL: u32 = 0x00aa_13e0;

/// The allocation scope guard (4 bytes on the stack) and the path of
/// `TESObjectCELL.cpp` it records.
const SCOPE_GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DESTRUCT: u32 = 0x0040_4ee0;
const SCOPE_GUARD_SIZE: u32 = 4;
const SOURCE_FILE_NAME: u32 = 0x0102_ed68;

/// Tests `UpdateRefSounds` makes on a reference: disabled (`00440da0`),
/// destroyed (`00477ba0`), its base form (`00441110`), the chance test
/// (`004dff00`), the sound name of the form (`00511840`), the distance
/// (`005723b0`, from the object at `011dea3c`) and the divisor of the chance.
const REFERENCE_IS_DISABLED: u32 = 0x0044_0da0;
const REFERENCE_IS_DESTROYED: u32 = 0x0047_7ba0;
const REFERENCE_BASE_FORM: u32 = 0x0044_1110;
const SOUND_CHANCE_TEST: u32 = 0x004d_ff00;
const FORM_SOUND_NAME: u32 = 0x0051_1840;
const REFERENCE_DISTANCE: u32 = 0x0057_23b0;
const DISTANCE_SOURCE_SINGLETON: u32 = 0x011d_ea3c;
const SOUND_CHANCE_DIVISOR: u32 = 0x0101_79e0;
/// The audio singleton getter (`00453a70`) and `GetSoundHandleByNumericID`
/// `(this, handle, form id, flags)`, then the sound handle calls.
const AUDIO_GET: u32 = 0x0045_3a70;
const AUDIO_GET_SOUND_HANDLE: u32 = 0x00ad_73b0;
const SOUND_HANDLE_FOLLOW: u32 = 0x00ad_8f20;
const SOUND_HANDLE_SET_POSITION: u32 = 0x00ad_8b60;
const SOUND_HANDLE_SET_PRIORITY: u32 = 0x00ad_9030;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_SIZE: u32 = 12;
const SOUND_FLAGS: u32 = 0x2102;
const SOUND_PRIORITY: u32 = 0xc0;
/// `4e75d0(this, out)`: the lookup `fn_00553bb0` makes on the object at
/// `+0x1c`.
const SOUND_LOOKUP: u32 = 0x004e_75d0;

/// Ray tests of `Pick`: against a node (`004b2800` with a bound, `004b2780`
/// with a child node), node position helpers, vector subtraction and length.
const RAY_TEST_BOUNDED: u32 = 0x004b_2800;
const RAY_TEST_NODE: u32 = 0x004b_2780;
const NODE_POSITION_A: u32 = 0x0050_0940;
const NODE_POSITION_B: u32 = 0x0096_8670;
const NODE_POSITION_C: u32 = 0x0041_3f40;
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
const VECTOR_LENGTH: u32 = 0x0045_7990;
const PICK_WORLD_SHAPE: u32 = 0x00c6_6fd0;
const PICK_FIND_ENTRY: u32 = 0x004b_5a80;
const PICK_ENTRY_SOURCE: u32 = 0x0043_b4f0;
const PICK_ENTRY_TYPE: u32 = 0x0043_b4d0;
const PICK_FALLBACK_NODE: u32 = 0x00c6_6fb0;
const CELL_NODE_OF_CELL: u32 = 0x0054_5cb0;
/// The value `PICK_ENTRY_TYPE` gives for entries whose answer is the cell's
/// own node.
const PICK_OWN_NODE_TYPE: u32 = 0x11;

/// Helper: runs `body` between the construction and the destruction of an
/// allocation scope guard (the guard the game keeps on its stack).
fn with_scope_guard<R>(
    e: &mut Engine,
    tag: u32,
    line: u32,
    body: impl FnOnce(&mut Engine) -> R,
) -> R {
    e.with_stack(SCOPE_GUARD_SIZE, |e, guard| {
        e.call(
            SCOPE_GUARD_CONSTRUCT,
            &args![guard, tag, 1u32, SOURCE_FILE_NAME, line],
        );
        let result = body(e);
        e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
        result
    })
}

/// Helper: the item of a list node: the word at the address `006815c0`
/// returns.
fn node_item(e: &mut Engine, node: Ptr) -> Ptr {
    let address = e.call(LIST_NODE_ITEM_ADDRESS, &args![node]).u32();
    Ptr::new(e.mem.u32(address))
}

/// Helper: the `NiTMap` walk the map loops share. `visit` gets the key and
/// the value of every entry, in the order `006b7f20` gives them.
fn for_each_map_entry(e: &mut Engine, map: Ptr, mut visit: impl FnMut(&mut Engine, Ptr, Ptr)) {
    e.with_stack(12, |e, scratch| {
        let position = scratch.addr();
        let key = scratch.addr() + 4;
        let value = scratch.addr() + 8;
        let first = e.call(MAP_FIRST_POSITION, &args![map]).u32();
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(key, 0);
            e.mem.set_u32(value, 0);
            e.call(MAP_NEXT, &args![map, position, key, value]);
            let (k, v) = (Ptr::new(e.mem.u32(key)), Ptr::new(e.mem.u32(value)));
            visit(e, k, v);
        }
    });
}

/// Helper: builds an update record `(time, a, b)` on the stack and runs
/// `body` with its address.
fn with_update_record<R>(
    e: &mut Engine,
    time: f32,
    a: u32,
    b: u32,
    body: impl FnOnce(&mut Engine, Ptr) -> R,
) -> R {
    e.with_stack(UPDATE_RECORD_SIZE, |e, record| {
        e.call(UPDATE_RECORD_CONSTRUCT, &args![record, time, a, b]);
        body(e, record)
    })
}

/// Helper: what the x87 `FISTP` with truncation leaves in the low word of a
/// `float` (the indefinite value `0x8000000000000000` for NaN and values out
/// of the 64-bit range).
fn truncated_low_word(value: f32) -> u32 {
    let value = value as f64;
    let limit = (1u64 << 63) as f64;
    if value.is_nan() || !(-limit..limit).contains(&value) {
        0
    } else {
        (value.trunc() as i64) as u32
    }
}

// Translated from 00552470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests flag `0x100000` of the flag word at `+0x30` of an object (through
/// `00456630`). The map has no name for it.
pub fn fn_00552470(e: &mut Engine, this: Ptr) -> bool {
    e.call(TEST_FLAG_BITS, &args![this, 0x0010_0000u32]).bool()
}

// Translated from 00552490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the byte at `+0x41` of the object.
pub fn fn_00552490(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x41)
}

// Translated from 005524b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the byte at `+0x8c` of the object.
pub fn fn_005524b0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x8c)
}

// Translated from 005524d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the byte `00408d60` finds for the object at `01267c24`.
pub fn fn_005524d0(e: &mut Engine) -> u8 {
    let holder = e.call(BYTE_HOLDER_GET, &args![BYTE_HOLDER]).u32();
    e.mem.u8(holder)
}

// Translated from 005524f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues a node for the worker `fn_00552570` (cdecl, one word): under the
/// pending lock, wraps `node` in a smart pointer and appends it to the
/// pending list. The exception frame is not translated.
pub fn fn_005524f0(e: &mut Engine, node: u32) {
    e.call(LOCK_ENTER, &args![PENDING_LOCK, 0u32]);
    e.with_stack(4, |e, slot| {
        e.call(SLOT_FROM_RAW, &args![slot, node]);
        e.call(PENDING_LIST_ADD, &args![PENDING_LIST, slot]);
        e.call(SLOT_RELEASE, &args![slot]);
    });
    e.call(LOCK_LEAVE, &args![PENDING_LOCK]);
}

// Translated from 00552570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The worker of the pending list: under the lock, takes each queued node
/// off the list, updates its morph controllers (`fn_005526f0`) and, when the
/// node still has a parent (virtual `+0x10`) and carries flag `0x400000`,
/// finds its reference (`FindReferenceFor3D`), tells the reference's parent
/// cell about it `(reference, 1, 0)` and clears the flag. The exception frame
/// is not translated.
pub fn fn_00552570(e: &mut Engine) {
    e.call(LOCK_ENTER, &args![PENDING_LOCK, 0u32]);
    while !e.call(PENDING_LIST_IS_EMPTY, &args![PENDING_LIST]).bool() {
        e.with_stack(4, |e, slot| {
            let head = e.call(LIST_NODE_ITEM_ADDRESS, &args![PENDING_LIST]).u32();
            e.call(SLOT_COPY, &args![slot, head]);
            e.call(PENDING_LIST_REMOVE_HEAD, &args![PENDING_LIST]);
            let node = e.call(SLOT_GET_POINTER, &args![slot]).ptr();
            fn_005526f0(e, node);
            if !e.call(SLOT_GET_POINTER, &args![slot]).ptr::<()>().is_null() {
                let node = e.call(SLOT_GET_POINTER, &args![slot]).ptr::<()>();
                if e.vcall(node.addr(), 0x10, &[]).u32() != 0 {
                    let node = e.call(SLOT_GET_POINTER, &args![slot]).ptr();
                    if fn_005526b0(e, node) {
                        let node = e.call(SLOT_GET_POINTER, &args![slot]).ptr::<()>();
                        let reference = e.call(FIND_REFERENCE_FOR_3D, &args![node]).ptr::<()>();
                        if !reference.is_null()
                            && e.call(REFERENCE_GET_PARENT_CELL, &args![reference]).u32() != 0
                        {
                            let cell = e.call(REFERENCE_GET_PARENT_CELL, &args![reference]).u32();
                            e.call(
                                CELL_UPDATE_FOR_REFERENCE,
                                &args![cell, reference, 1u32, 0u32],
                            );
                            let node = e.call(SLOT_GET_POINTER, &args![slot]).ptr();
                            fn_005526d0(e, node, false);
                        }
                    }
                }
            }
            e.call(SLOT_RELEASE, &args![slot]);
        });
    }
    e.call(LOCK_LEAVE, &args![PENDING_LOCK]);
}

// Translated from 005526b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests flag `0x400000` of the flag word at `+0x30` of an object.
pub fn fn_005526b0(e: &mut Engine, this: Ptr) -> bool {
    e.call(TEST_FLAG_BITS, &args![this, 0x0040_0000u32]).bool()
}

// Translated from 005526d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears flag `0x400000` of the flag word at `+0x30` of an object.
pub fn fn_005526d0(e: &mut Engine, this: Ptr, set: bool) {
    e.call(SET_FLAG_BITS, &args![this, set, 0x0040_0000u32]);
}

// Translated from 005526f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks a scene-graph node (cdecl, one word): a node with a child array
/// (virtual `+0x0c`) recurses into every child; a node without one asks for
/// its controller (virtual `+0x30`) and, if it has one, applies an update
/// record `(UPDATE_TIME, 1, 0)` to it.
pub fn fn_005526f0(e: &mut Engine, node: Ptr) {
    if node.is_null() {
        return;
    }
    let children = e.vcall(node.addr(), 0x0c, &[]).ptr::<()>();
    if !children.is_null() {
        let count = e.call(NODE_CHILD_COUNT, &args![children]).u32();
        for index in 0..count {
            let child = e.call(NODE_CHILD_AT, &args![children, index]).ptr();
            fn_005526f0(e, child);
        }
    } else {
        let controller = e.vcall(node.addr(), 0x30, &[]).ptr::<()>();
        if !controller.is_null() {
            let time: f32 = e.global(UPDATE_TIME);
            with_update_record(e, time, 1, 0, |e, record| {
                e.call(CONTROLLER_UPDATE, &args![controller, record]);
            });
        }
    }
}

// Translated from 00552790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::UpdateMorphControllersOnly` (Xbox PDB), a walk of the
/// node it is given (cdecl, one word): updates the morph controller of the
/// node (`GetController` with the type at `011f3728`, then virtual `+0x94`
/// with an update record `(UPDATE_TIME, 1, 0)`) and recurses into the
/// children of the child array (virtual `+0x0c`).
pub fn tes_object_cell_update_morph_controllers_only(e: &mut Engine, node: Ptr) {
    if node.is_null() {
        return;
    }
    let controller = e
        .call(NODE_GET_CONTROLLER, &args![node, MORPH_CONTROLLER_TYPE])
        .ptr::<()>();
    if !controller.is_null() {
        let time: f32 = e.global(UPDATE_TIME);
        with_update_record(e, time, 1, 0, |e, record| {
            e.vcall(controller.addr(), 0x94, &args![record]);
        });
    }
    let children = e.vcall(node.addr(), 0x0c, &[]).ptr::<()>();
    if !children.is_null() {
        let count = e.call(NODE_CHILD_COUNT, &args![children]).u32();
        for index in 0..count {
            let child = e.call(NODE_CHILD_AT, &args![children, index]).ptr();
            tes_object_cell_update_morph_controllers_only(e, child);
        }
    }
}

// Translated from 00552840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sends an update record `(0.0, 0, 0)` to the cell's node (`005497a0`,
/// virtual `+0xc0`) and then to every value of the loaded data's animated
/// reference map (`+0x0c` of `pLoadedData`) whose owner (`009611e0`) is not
/// the cell's own node.
pub fn fn_00552840(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    with_update_record(e, 0.0, 0, 0, |e, record| {
        let own = e.call(CELL_NODE_A, &args![this]).ptr::<()>();
        if !own.is_null() {
            e.vcall(own.addr(), 0xc0, &args![record]);
        }
        let loaded = e.get(this, TESObjectCELL::pLoadedData);
        if !loaded.is_null() {
            let map = loaded.byte_add(LoadedCellData::AnimatedRefMap.off);
            for_each_map_entry(e, map, |e, _key, value| {
                if !value.is_null() && e.call(ANIMATED_NODE_OWNER, &args![value]).ptr::<()>() != own
                {
                    e.vcall(value.addr(), 0xc0, &args![record]);
                }
            });
        }
    });
}

// Translated from 00552900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00484730(this, flag)` and, for a form without flag `0x4000`
/// that has flag `0x2` and is an exterior cell with a world space, calls the
/// world space's virtual `+0xc8` with `1`.
pub fn fn_00552900(e: &mut Engine, this: Ptr<TESObjectCELL>, flag: u8) {
    e.call(CELL_UPDATE_REFERENCE_FLAGS, &args![this, flag]);
    if !e.call(FORM_FLAG_4000, &args![this]).bool()
        && e.call(FORM_FLAG_2, &args![this]).bool()
        && !e.call(CELL_IS_INTERIOR, &args![this]).bool()
    {
        let world_space = e.call(CELL_GET_WORLD_SPACE, &args![this]).ptr::<()>();
        if !world_space.is_null() {
            e.vcall(world_space.addr(), WORLD_SPACE_SLOT, &args![1u32]);
        }
    }
}

// Translated from 00552970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::AttachToWorld` (Xbox PDB): loads the cell's 3D, sets the
/// cell state to 5 and attaches the 3D to `parent` (virtual `+0xdc`). With
/// a Havok world: builds the exterior terrain objects for an exterior cell
/// and switches the visual debugger. Then attaches the lights, the add-on
/// nodes and the loaded flag, adds the draw-only nav meshes when the render
/// singleton asks for them, runs the attach follow-ups, registers the cell's
/// nodes with the portal graph's always-render array (only when it is empty)
/// and with the cell's option set, and clears `bCellDetached`.
pub fn tes_object_cell_attach_to_world(e: &mut Engine, this: Ptr<TESObjectCELL>, parent: Ptr) {
    with_scope_guard(e, 0x1a, 0x395d, |e| {
        let node = e.call(CELL_LOAD_3D, &args![this]).u32();
        e.call(CELL_SET_STATE, &args![this, 5u32]);
        e.vcall(parent.addr(), 0xdc, &args![node, 1u32]);
        let world = e.call(CELL_HAVOK_WORLD, &args![this]).ptr::<()>();
        if !world.is_null() {
            if !e.call(CELL_IS_INTERIOR, &args![this]).bool() {
                fn_005535f0(e, this);
            }
            let suppressed = e.call(VISUAL_DEBUGGER_SUPPRESSED, &[]).bool();
            e.call(
                WORLD_SET_VISUAL_DEBUGGER_ENABLED,
                &args![world, !suppressed],
            );
            let setting = e.call(VISUAL_DEBUGGER_SETTING, &[]).u8();
            e.call(WORLD_SET_VISUAL_DEBUGGER, &args![world, setting]);
        }
        e.call(CELL_SET_LIGHTS_ATTACHED, &args![this, 1u32]);
        e.call(CELL_SET_ADDON_NODES_ATTACHED, &args![this, 1u32]);
        e.call(CELL_SET_ATTACHED_FLAG, &args![this, 1u32]);
        let render = fn_00552ba0(e);
        if fn_00552bb0(e, Ptr::new(render)) {
            let render = fn_00552ba0(e);
            e.call(NAV_MESH_RENDER_ADD, &args![render, this, 1u32]);
        }
        e.call(CELL_AFTER_ATTACH_A, &args![this]);
        e.call(CELL_AFTER_ATTACH_B, &args![this]);
        let obstacles = e.call(OBSTACLE_MANAGER_GET, &[]).u32();
        e.call(OBSTACLE_MANAGER_RUN, &args![obstacles]);
        let options = e.call(CELL_OPTIONS, &args![this]).u32();
        let table = e.call(TABLE_ENTRY, &args![0u32]).u32();
        e.call(SCENE_NODE_ATTACH, &args![table, options]);
        if e.call(PORTAL_ALWAYS_RENDER_COUNT, &args![options]).u32() == 0 {
            let node_a = e.call(CELL_NODE_A, &args![this]).u32();
            e.call(PORTAL_ALWAYS_RENDER_ADD, &args![options, node_a]);
            let node_b = e.call(CELL_NODE_B, &args![this]).u32();
            e.call(PORTAL_ALWAYS_RENDER_ADD, &args![options, node_b]);
            let table = e.call(TABLE_ENTRY, &args![0u32]).u32();
            let child = e.call(SCENE_NODE_CHILD, &args![table, 5u32]).u32();
            e.call(PORTAL_ALWAYS_RENDER_ADD, &args![options, child]);
        }
        let child = e.call(CELL_CHILD_NODE, &args![this, 6u32]).ptr::<()>();
        let graph = e.call(CELL_PORTAL_GRAPH, &args![options]).u32();
        e.vcall(child.addr(), 0xdc, &args![graph, 1u32]);
        for node in [CELL_NODE_C, CELL_NODE_B] {
            let node = e.call(node, &args![this]).u32();
            e.call(PORTAL_ALWAYS_RENDER_SET, &args![options, node]);
        }
        let child = e.call(CELL_CHILD_NODE, &args![this, 2u32]).u32();
        e.call(PORTAL_ALWAYS_RENDER_SET, &args![options, child]);
        let node = e.call(CELL_NODE_D, &args![this]).u32();
        e.call(PORTAL_ALWAYS_RENDER_SET, &args![options, node]);
        e.set(this, TESObjectCELL::bCellDetached, false);
    });
}

// Translated from 00552ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the render singleton pointer (`011d6e2c`).
pub fn fn_00552ba0(e: &mut Engine) -> u32 {
    e.global(RENDER_SINGLETON)
}

// Translated from 00552bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the byte at `+0x210` of the render singleton: whether the
/// draw-only nav meshes are wanted.
pub fn fn_00552bb0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x210) != 0
}

// Translated from 00552bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::Detach` (Xbox PDB): under the cell lock, for a cell whose
/// state is 5 or 6: sets the state to 4, cancels the model loader's
/// references for the cell, detaches the cell from the scene (interior and
/// exterior differ), detaches the Havok world when `detach_havok`, detaches
/// the 3D from the scene node, sets the state to 3 and `bCellDetached`,
/// removes the draw-only nav meshes, drops the saved changes when the form
/// has them and removes the cell's nodes from the portal graph's option set.
pub fn tes_object_cell_detach(e: &mut Engine, this: Ptr<TESObjectCELL>, detach_havok: bool) {
    e.call(CELL_LOCK_ENTER, &args![this]);
    let state = e.call(CELL_GET_STATE, &args![this]).u32();
    if state == 6 || state == 5 {
        e.call(CELL_SET_STATE, &args![this, 4u32]);
        e.call(CELL_BEFORE_DETACH, &args![this]);
        let model_loader: u32 = e.global(MODEL_LOADER_SINGLETON);
        e.call(MODEL_LOADER_CANCEL_FOR_CELL, &args![model_loader, this]);
        let scene: u32 = e.global(SCENE_SINGLETON);
        e.call(SCENE_REFRESH, &args![scene]);
        e.call(CELL_SET_LIGHTS_ATTACHED, &args![this, 0u32]);
        let scene: u32 = e.global(SCENE_SINGLETON);
        if e.call(CELL_IS_INTERIOR, &args![this]).bool() {
            e.call(SCENE_ATTACH_INTERIOR, &args![scene, this]);
        } else {
            e.call(SCENE_ATTACH_EXTERIOR, &args![scene, this]);
        }
        if detach_havok {
            fn_00553700(e, this);
        }
        e.call(CELL_SET_ADDON_NODES_ATTACHED, &args![this, 0u32]);
        let node = e.call(CELL_LOAD_3D, &args![this]).u32();
        let scene: u32 = e.global(SCENE_SINGLETON);
        let parent = e.call(WORD_AT_0C, &args![scene]).ptr::<()>();
        e.vcall(parent.addr(), 0xe8, &args![node]);
        e.call(CELL_DETACH_STEP_A, &args![this]);
        e.call(CELL_DETACH_STEP_B, &args![this]);
        e.call(CELL_SET_STATE, &args![this, 3u32]);
        e.set(this, TESObjectCELL::bCellDetached, true);
        let state_object: u32 = e.global(STATE_OBJECT_SINGLETON);
        if fn_00552da0(e, Ptr::new(state_object)) == 0 {
            e.call(CELL_SET_TERRAIN_UPDATE, &args![this, 1u32]);
        }
        e.call(CELL_CLEAR_LOADED_STATE, &args![this]);
        e.call(CELL_SET_ATTACHED_FLAG, &args![this, 0u32]);
        let render = fn_00552ba0(e);
        e.call(NAV_MESH_RENDER_REMOVE, &args![render, this]);
        let form_id = e.call(FORM_ID, &args![this]).u32();
        let loader_state: u32 = e.global(LOADER_STATE_SINGLETON);
        if e.call(FORM_HAS_CHANGES, &args![loader_state, form_id])
            .bool()
        {
            let loader: u32 = e.global(LOADER_SINGLETON);
            e.call(REMOVE_CHANGES, &args![loader, this]);
        }
        let scene: u32 = e.global(SCENE_SINGLETON);
        e.call(SCENE_REFRESH, &args![scene]);
        let options = e.call(CELL_OPTIONS, &args![this]).u32();
        let node_c = e.call(CELL_NODE_C, &args![this]).u32();
        e.call(PORTAL_ALWAYS_RENDER_REMOVE, &args![options, node_c]);
        let node_b = e.call(CELL_NODE_B, &args![this]).u32();
        e.call(PORTAL_ALWAYS_RENDER_REMOVE, &args![options, node_b]);
        let child = e.call(CELL_CHILD_NODE, &args![this, 2u32]).u32();
        e.call(PORTAL_ALWAYS_RENDER_REMOVE, &args![options, child]);
        let node_d = e.call(CELL_NODE_D, &args![this]).u32();
        e.call(PORTAL_ALWAYS_RENDER_REMOVE, &args![options, node_d]);
    }
    e.call(CELL_LOCK_LEAVE, &args![this]);
}

// Translated from 00552da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the byte at `+0x26` of the object (`Detach` calls it on the object
/// at `011de134`).
pub fn fn_00552da0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x26)
}

// Translated from 00552dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the Havok world of an interior cell that has none (the map has no
/// name for it): builds a world cinfo (gravity `(0, 0, g, 0)`, broad phase
/// size, scale `1.0`), then the world with its broad phase, filter, optional
/// water, a listener object and the `011ca088` singleton, positions it at the
/// cell's reference centre, enables the visual debugger setting and stores the
/// world in the cell's extra data. Then, when the cell has no collision world,
/// creates one (`00621d10`) and stores it in the extra data too. The exception
/// frames and the stack cookie are not translated.
pub fn fn_00552dc0(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    let world = e.call(CELL_HAVOK_WORLD, &args![this]).ptr::<()>();
    if e.call(CELL_IS_INTERIOR, &args![this]).bool() && world.is_null() {
        with_scope_guard(e, 0x10, 0x3a3f, |e| {
            e.with_stack(CINFO_SIZE, |e, cinfo| {
                e.call(CINFO_CONSTRUCT, &args![cinfo]);
                let gravity: f32 = e.global(GRAVITY_Z);
                e.with_stack(16, |e, vector| {
                    let vector = fn_005532a0(e, vector, 0.0, 0.0, gravity, 0.0);
                    e.call(CINFO_SET_GRAVITY, &args![cinfo.byte_add(0x10), vector]);
                });
                let size: f32 = e.global(BROAD_PHASE_SIZE);
                e.call(CINFO_SET_BROAD_PHASE_SIZE, &args![cinfo, size]);
                e.mem.set_f32(cinfo.addr() + 0x54, 1.0);
                let raw = fn_00553300(e, 0xa0);
                let world = if raw == 0 {
                    Ptr::NULL
                } else {
                    e.call(WORLD_CONSTRUCT, &args![raw, cinfo]).ptr()
                };
                fn_005533c0(e, world);
                let block = e.call(OPERATOR_NEW, &args![0x24u32]).u32();
                let broad_phase = if block == 0 {
                    0
                } else {
                    e.call(BROAD_PHASE_CONSTRUCT, &args![block]).u32()
                };
                fn_00553480(e, world, if broad_phase == 0 { 0 } else { broad_phase + 8 });
                let block = e.call(OPERATOR_NEW, &args![0x28u32]).u32();
                let filter = if block == 0 {
                    0
                } else {
                    e.call(FILTER_CONSTRUCT, &args![block]).u32()
                };
                fn_005534f0(e, world, filter);
                if e.call(HAS_WATER_TEST, &args![this]).bool() {
                    let block = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
                    let water = if block == 0 {
                        0
                    } else {
                        let height = e.call(CELL_GET_WATER_HEIGHT, &args![this]).f32();
                        e.call(WATER_CONSTRUCT, &args![block, height]).u32()
                    };
                    e.call(WORLD_SET_GRAVITY_SCALE, &args![world, water]);
                }
                let setting = e.call(SCENE_HAVOK_SETTING, &[]).u32();
                fn_00553440(e, world, setting);
                let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
                let listener = if block == 0 {
                    0
                } else {
                    e.call(LISTENER_OBJECT_CONSTRUCT, &args![block]).u32()
                };
                fn_005533f0(e, world, listener);
                if e.global::<u32>(LISTENER_SINGLETON) == 0 {
                    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
                    let singleton = if block == 0 {
                        0
                    } else {
                        e.call(LISTENER_CONSTRUCT, &args![block]).u32()
                    };
                    e.set_global(LISTENER_SINGLETON, singleton);
                }
                let held = e.call(HOLDER_GET_WORLD, &args![world]).u32();
                let singleton: u32 = e.global(LISTENER_SINGLETON);
                e.call(LISTENER_REGISTER, &args![singleton, held]);
                e.with_stack(12, |e, center| {
                    e.call(LIST_NODE_ITEM_ADDRESS, &args![center]);
                    e.call(CELL_REFERENCE_CENTER, &args![this, center]);
                    e.call(WORLD_SET_CENTER, &args![world, center]);
                });
                let setting = e.call(VISUAL_DEBUGGER_SETTING, &[]).u8();
                e.call(WORLD_SET_VISUAL_DEBUGGER, &args![world, setting]);
                e.call(
                    EXTRA_LIST_SET_HAVOK_WORLD,
                    &args![this.byte_add(EXTRA_LIST_OFFSET), world],
                );
                fn_005533c0(e, world);
                e.vcall(world.addr(), 0xc4, &[]);
                let suppressed = e.call(VISUAL_DEBUGGER_SUPPRESSED, &[]).bool();
                e.call(
                    WORLD_SET_VISUAL_DEBUGGER_ENABLED,
                    &args![world, !suppressed],
                );
                fn_00553520(e, cinfo);
            });
        });
    }
    if e.call(CELL_COLLISION_WORLD, &args![this]).u32() == 0 {
        with_scope_guard(e, 0x10, 0x3a91, |e| {
            let block = e.call(ALLOCATE_CDECL, &args![0x18u32]).u32();
            let collision = if block == 0 {
                0
            } else {
                e.call(COLLISION_WORLD_CONSTRUCT, &args![block]).u32()
            };
            e.call(
                EXTRA_LIST_SET_COLLISION_WORLD,
                &args![this.byte_add(EXTRA_LIST_OFFSET), collision],
            );
        });
    }
}

// Translated from 005532a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores four floats at `this` (a 16-byte vector constructor, SSE
/// `UNPCKLPS`): `(x, y, z, w)` in order. Returns `this`.
pub fn fn_005532a0(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32, w: f32) -> Ptr {
    for (index, value) in [x, y, z, w].into_iter().enumerate() {
        e.mem.set_f32(this.addr() + 4 * index as u32, value);
    }
    this
}

// Translated from 00553300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates `size` bytes aligned to 16 (cdecl): `fn_00553320(16, size)`.
pub fn fn_00553300(e: &mut Engine, size: u32) -> u32 {
    fn_00553320(e, 0x10, size)
}

// Translated from 00553320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Aligned allocation (cdecl): allocates `align + size` bytes from the memory
/// manager, moves the pointer up to the next multiple of `align` (a whole
/// `align` when it already is one) and stores the distance moved in the byte
/// before the returned pointer, where `fn_00553380` finds it.
pub fn fn_00553320(e: &mut Engine, align: u8, size: u32) -> u32 {
    let manager = e.call(MEMORY_MANAGER_GET, &[]).u32();
    let total = (align as u32).wrapping_add(size);
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![manager, total])
        .u32();
    let misalignment = (align.wrapping_sub(1) as u32 & block) as u8;
    let distance = align.wrapping_sub(misalignment);
    let aligned = (distance as u32).wrapping_add(block);
    e.mem.set_u8(aligned.wrapping_sub(1), distance);
    aligned
}

// Translated from 00553380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees a block of [`fn_00553320`] (cdecl): moves the pointer back by the
/// distance stored before it and gives it to the memory manager. A null
/// pointer is ignored.
pub fn fn_00553380(e: &mut Engine, pointer: u32) {
    if pointer != 0 {
        let distance = e.mem.u8(pointer.wrapping_sub(1)) as u32;
        let manager = e.call(MEMORY_MANAGER_GET, &[]).u32();
        e.call(
            MEMORY_MANAGER_DEALLOCATE,
            &args![manager, pointer.wrapping_sub(distance)],
        );
    }
}

// Translated from 005533c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Havok world wrapper: takes the world's object (`00541c80`) and, if there is
/// one, calls `00483710` on it.
pub fn fn_005533c0(e: &mut Engine, this: Ptr) {
    let world = e.call(HOLDER_GET_WORLD, &args![this]).u32();
    if world != 0 {
        e.call(EMPTY_FUNCTION, &args![world]);
    }
}

// Translated from 005533f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Havok world wrapper: with the world's object (`00541c80`), if there is
/// one: takes the lock around `00c93820(world, value)`. Then, whether or not
/// there is a world, adds the address of `value` to the holder's list at
/// `+0x70` (`006299c0`).
pub fn fn_005533f0(e: &mut Engine, this: Ptr, value: u32) {
    let world = e.call(HOLDER_GET_WORLD, &args![this]).u32();
    if world != 0 {
        fn_005533c0(e, this);
        e.call(WORLD_SET_GRAVITY_VECTOR, &args![world, value]);
        fn_005533c0(e, this);
    }
    // A null world jumps straight to the `006299c0` call (00553408 JZ
    // 0x00553426), so the address is queued either way.
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(HOLDER_FIELD_70_ADD, &args![this.byte_add(0x70), slot]);
    });
}

// Translated from 00553440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Havok world wrapper: with the world's object (`00541c80`), if there is
/// one: `00c93a20(world, value)` between two calls of `fn_005533c0`.
pub fn fn_00553440(e: &mut Engine, this: Ptr, value: u32) {
    let world = e.call(HOLDER_GET_WORLD, &args![this]).u32();
    if world != 0 {
        fn_005533c0(e, this);
        e.call(WORLD_SET_FIELD_A, &args![world, value]);
        fn_005533c0(e, this);
    }
}

// Translated from 00553480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Havok world wrapper: runs `fn_005533f0(this, value)`, then, with the world's
/// object, sets `00c93860(world, value + 0x14)` (0 for a null `value`)
/// between two calls of `fn_005533c0`; stores `value` at `+0x20`.
pub fn fn_00553480(e: &mut Engine, this: Ptr, value: u32) {
    fn_005533f0(e, this, value);
    let world = e.call(HOLDER_GET_WORLD, &args![this]).u32();
    if world != 0 {
        fn_005533c0(e, this);
        let inner = if value == 0 { 0 } else { value + 0x14 };
        e.call(WORLD_SET_FIELD_B, &args![world, inner]);
        fn_005533c0(e, this);
    }
    e.mem.set_u32(this.addr() + 0x20, value);
}

// Translated from 005534f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Havok world wrapper: runs `fn_005533f0(this, value)` and stores `value` at
/// `+0x28`.
pub fn fn_005534f0(e: &mut Engine, this: Ptr, value: u32) {
    fn_005533f0(e, this, value);
    e.mem.set_u32(this.addr() + 0x28, value);
}

// Translated from 00553520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the destructor [`hkp_world_cinfo_destructor`] of the world cinfo.
pub fn fn_00553520(e: &mut Engine, this: Ptr) {
    hkp_world_cinfo_destructor(e, this);
}

// Translated from 00553540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCinfo::~hkpWorldCinfo` (Xbox PDB): stores its vtable, runs
/// `00558e80` on the members at `+0x6c`, `+0x5c` and `+0x58`, then the base
/// destructor `00538e10`. The exception frame is not translated.
pub fn hkp_world_cinfo_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CINFO_VTABLE);
    for offset in [0x6c, 0x5c, 0x58] {
        e.call(CINFO_MEMBER_DESTRUCT, &args![this.byte_add(offset)]);
    }
    e.call(CINFO_BASE_DESTRUCT, &args![this]);
}

// Translated from 005535c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldCinfo::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor, then frees the block (`00538eb0`) when bit 0 of `flags` is
/// set. Returns `this`.
pub fn hkp_world_cinfo_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    hkp_world_cinfo_destructor(e, this);
    if flags & 1 != 0 {
        e.call(CINFO_FREE, &args![this]);
    }
    this
}

// Translated from 005535f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For an exterior cell with a Havok world and a collision world: builds a
/// transform (identity rotation from `011a9448`, translation
/// `(cellX * 4096 + 2048, cellY * 4096 + 2048, 0)`, scale `1.0`) and gives its
/// translation, with the flags at `011a9ea4`, to the collision world
/// (`00622590`). The exception frame is not translated.
pub fn fn_005535f0(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    let world = e.call(CELL_HAVOK_WORLD, &args![this]).u32();
    let cell_x = e.call(CELL_GET_DATA_X, &args![this]).i32();
    let cell_y = e.call(CELL_GET_DATA_Y, &args![this]).i32();
    let collision = e.call(CELL_COLLISION_WORLD, &args![this]).u32();
    if world != 0 && collision != 0 {
        with_scope_guard(e, 0x10, 0x3aa6, |e| {
            e.with_stack(0x34, |e, transform| {
                let at = transform.addr();
                e.call(TRANSFORM_CONSTRUCT, &args![transform]);
                e.mem.set_f32(at + 0x30, 1.0);
                for index in 0..9 {
                    let word = e.mem.u32(IDENTITY_ROTATION + 4 * index);
                    e.mem.set_u32(at + 4 * index, word);
                }
                let half: f64 = e.global(HALF_CELL_SIZE);
                e.mem
                    .set_f32(at + 0x24, ((cell_x << 12) as f64 + half) as f32);
                e.mem
                    .set_f32(at + 0x28, ((cell_y << 12) as f64 + half) as f32);
                e.mem.set_f32(at + 0x2c, 0.0);
                e.call(
                    WORLD_ADD_TERRAIN,
                    &args![collision, world, at + 0x24, TERRAIN_FLAGS],
                );
            });
        });
    }
}

// Translated from 00553700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::DetachHavok` (Xbox PDB). Interior cell with a Havok
/// world: turns the visual debugger flag and setting off, and if the world
/// has a collision object, activates it with `1` and releases it with `0`.
/// Exterior cell: unloads the cell's collision world (`00621e80`), then under
/// the cell lock calls virtual `+0x1c0` of every reference; releases the Havok
/// world's collision object (`004893c0(.., 0)`) and, if the object at
/// `011de134` says so (`fn_00552da0`), calls `00c68a00(world, 0)`.
pub fn fn_00553700(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    if e.call(CELL_IS_INTERIOR, &args![this]).bool() {
        let world = e.call(CELL_HAVOK_WORLD, &args![this]).u32();
        if world != 0 {
            e.call(WORLD_SET_VISUAL_DEBUGGER_ENABLED, &args![world, false]);
            e.call(WORLD_SET_VISUAL_DEBUGGER, &args![world, 0u32]);
            if e.call(WORLD_COLLISION_OBJECT, &args![world]).u32() != 0 {
                let object = e.call(WORLD_COLLISION_OBJECT, &args![world]).u32();
                e.call(COLLISION_OBJECT_SET_ACTIVE, &args![object, 1u32]);
                e.call(COLLISION_OBJECT_RELEASE, &args![object, 0u32]);
            }
        }
        return;
    }
    let collision = e.call(CELL_COLLISION_WORLD, &args![this]).u32();
    if collision != 0 {
        e.call(COLLISION_WORLD_UNLOAD, &args![collision]);
    }
    e.call(CELL_LOCK_ENTER, &args![this]);
    let mut node = e.call(CELL_REFERENCES, &args![this]).ptr::<()>();
    while !node.is_null() {
        let reference = node_item(e, node);
        node = e.call(LIST_NODE_NEXT, &args![node]).ptr();
        if !reference.is_null() {
            e.vcall(reference.addr(), 0x1c0, &[]);
        }
    }
    let world = e.call(CELL_HAVOK_WORLD, &args![this]).u32();
    if world != 0 {
        let object = e.call(WORLD_COLLISION_OBJECT, &args![world]).u32();
        e.call(COLLISION_OBJECT_RELEASE, &args![object, 0u32]);
    }
    let state_object: u32 = e.global(STATE_OBJECT_SINGLETON);
    if fn_00552da0(e, Ptr::new(state_object)) != 0 {
        e.call(WORLD_UPDATE_RENDERER, &args![world, 0u32]);
    }
    e.call(CELL_LOCK_LEAVE, &args![this]);
}

// Translated from 00553820 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the cell is loaded (`00450ff0`) and has loaded data: for every
/// reference of the animated reference map (`+0x0c` of `pLoadedData`) that
/// answers false to virtual `+0x100`, has a base object (`007af430`) whose
/// form type (`00401170`) is not `0x25`, calls virtual `+0x1e0`.
pub fn fn_00553820(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    if !e.call(CELL_IS_LOADED, &args![this]).bool() {
        return;
    }
    let loaded = e.get(this, TESObjectCELL::pLoadedData);
    if loaded.is_null() {
        return;
    }
    let map = loaded.byte_add(LoadedCellData::AnimatedRefMap.off);
    for_each_map_entry(e, map, |e, reference, _node| {
        if !e.vcall(reference.addr(), 0x100, &[]).bool() {
            let base = e.call(REFERENCE_BASE_OBJECT, &args![reference]).u32();
            if base != 0 {
                let base = e.call(REFERENCE_BASE_OBJECT, &args![reference]).u32();
                if e.call(FORM_TYPE, &args![base]).u32() != 0x25 {
                    e.vcall(reference.addr(), 0x1e0, &[]);
                }
            }
        }
    });
}

// Translated from 005538e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::UpdateRefSounds` (Xbox PDB): under the cell lock, for a
/// loaded cell, walks its references until the first empty list node. A
/// reference that is not disabled (`00440da0`) or destroyed (`00477ba0`),
/// passes `fn_00553bb0`, and (if virtual `+0x21c` is true) is not excluded by
/// virtual `+0x22c(0)`, whose base form (`00441110`) passes the chance test
/// (`fn_00553b60 / divisor`) and has a sound name, and whose distance
/// (truncated to its low word) is below `fn_00553b90 * 100` starts the
/// form's sound: a sound handle for form id and flags `0x2102`, following
/// the reference's 3D (virtual `+0x1d0`), at the position of virtual `+0x1f4`,
/// priority `0xc0`, played.
pub fn tes_object_cell_update_ref_sounds(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    e.call(CELL_LOCK_ENTER, &args![this]);
    if e.call(CELL_IS_LOADED, &args![this]).bool() {
        let mut node = e.call(CELL_REFERENCES, &args![this]).ptr::<()>();
        while !node.is_null() {
            let address = e.call(LIST_NODE_ITEM_ADDRESS, &args![node]).u32();
            if e.mem.u32(address) == 0 {
                break;
            }
            let reference = node_item(e, node);
            if !reference.is_null() {
                update_reference_sound(e, reference);
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).ptr();
        }
    }
    e.call(CELL_LOCK_LEAVE, &args![this]);
}

/// The body of the loop of [`tes_object_cell_update_ref_sounds`] for one
/// reference.
fn update_reference_sound(e: &mut Engine, reference: Ptr) {
    if e.call(REFERENCE_IS_DISABLED, &args![reference]).bool()
        || e.call(REFERENCE_IS_DESTROYED, &args![reference]).bool()
        || !fn_00553bb0(e, reference)
    {
        return;
    }
    if e.vcall(reference.addr(), 0x21c, &[]).bool()
        && e.vcall(reference.addr(), 0x22c, &args![0u32]).bool()
    {
        return;
    }
    let form = e.call(REFERENCE_BASE_FORM, &args![reference]).ptr::<()>();
    if form.is_null() {
        return;
    }
    let chance = fn_00553b60(e, form) as i32;
    let divisor: f64 = e.global(SOUND_CHANCE_DIVISOR);
    let chance = (chance as f64 / divisor) as f32;
    if !e.call(SOUND_CHANCE_TEST, &args![chance]).bool() {
        return;
    }
    let name = e.call(FORM_SOUND_NAME, &args![form]).u32();
    if e.mem.i8(name) == 0 {
        return;
    }
    let range = fn_00553b90(e, form) as u32 * 100;
    let source: u32 = e.global(DISTANCE_SOURCE_SINGLETON);
    let distance = e
        .call(REFERENCE_DISTANCE, &args![source, reference, 0u32, 0u32])
        .f32();
    if range <= truncated_low_word(distance) {
        return;
    }
    e.with_stack(SOUND_HANDLE_SIZE, |e, handle| {
        let audio = e.call(AUDIO_GET, &[]).u32();
        let form_id = e.call(FORM_ID, &args![form]).u32();
        e.call(
            AUDIO_GET_SOUND_HANDLE,
            &args![audio, handle, form_id, SOUND_FLAGS],
        );
        if e.vcall(reference.addr(), 0x1d0, &[]).u32() != 0 {
            let follow = e.vcall(reference.addr(), 0x1d0, &[]).u32();
            e.call(SOUND_HANDLE_FOLLOW, &args![handle, follow]);
        }
        let position = e.vcall(reference.addr(), 0x1f4, &[]).u32();
        let (x, y, z) = (
            e.mem.f32(position),
            e.mem.f32(position + 4),
            e.mem.f32(position + 8),
        );
        e.call(SOUND_HANDLE_SET_POSITION, &args![handle, x, y, z]);
        e.call(SOUND_HANDLE_SET_PRIORITY, &args![handle, SOUND_PRIORITY]);
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        e.call(EMPTY_FUNCTION, &args![handle]);
    });
}

// Translated from 00553b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sound chance of a form: the signed byte at `+0x68` when bit 1 of the
/// word at `+0x48` is set, else 0.
pub fn fn_00553b60(e: &mut Engine, this: Ptr) -> i8 {
    if e.mem.u32(this.addr() + 0x48) & 2 != 0 {
        e.mem.i8(this.addr() + 0x68)
    } else {
        0
    }
}

// Translated from 00553b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the byte at `+0x45` of the form (the sound range in hundreds).
pub fn fn_00553b90(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x45)
}

// Translated from 00553bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the object at `+0x1c` exists and its lookup (`004e75d0`) gives a
/// record whose word at `+4` has bit 1 set.
pub fn fn_00553bb0(e: &mut Engine, this: Ptr) -> bool {
    let holder = e.mem.u32(this.addr() + 0x1c);
    if holder == 0 {
        return false;
    }
    // The exe's local is 0x24 bytes (EBP-0x24, 00553bc2): 004e75d0 copies nine words into it.
    e.with_stack(0x24, |e, out| {
        let record = e.call(SOUND_LOOKUP, &args![holder, out]).u32();
        e.mem.u32(record + 4) & 2 != 0
    })
}

// Translated from 00553bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ray test against the cell's reference nodes (`00545710`): `true` when a
/// reference with a 3D (virtual `+0x1d0`) passes the bounded test
/// `004b2800(object, a, b, limit, 3D)` and the distance from the node's
/// position (`00500940`/`00968670`/`00413f40`, minus `b`'s vector with
/// `00439ef0`) is below `limit`. Stops at an empty list node.
pub fn fn_00553bf0(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    object: Ptr,
    a: u32,
    b: u32,
    limit: f32,
) -> bool {
    if object.is_null() {
        return false;
    }
    let mut node = e.call(CELL_REFERENCE_NODES, &args![this]).ptr::<()>();
    while !node.is_null() {
        if e.call(LIST_NODE_IS_END, &args![node]).bool() {
            return false;
        }
        let reference = node_item(e, node);
        if e.vcall(reference.addr(), 0x1d0, &[]).u32() != 0 {
            let node_3d = e.vcall(reference.addr(), 0x1d0, &[]).u32();
            if e.call(RAY_TEST_BOUNDED, &args![object, a, b, limit, node_3d])
                .bool()
            {
                let position = e.call(NODE_POSITION_A, &args![object]).u32();
                let position = e.call(NODE_POSITION_B, &args![position, 0u32]).u32();
                let position = e.call(NODE_POSITION_C, &args![position]).u32();
                let (x, y, z) = (
                    e.mem.u32(position),
                    e.mem.u32(position + 4),
                    e.mem.u32(position + 8),
                );
                let length = e.with_stack(0x20, |e, scratch| {
                    e.mem.set_u32(scratch.addr() + 0x10, x);
                    e.mem.set_u32(scratch.addr() + 0x14, y);
                    e.mem.set_u32(scratch.addr() + 0x18, z);
                    let difference = e
                        .call(VECTOR_SUBTRACT, &args![scratch.addr() + 0x10, scratch, a])
                        .u32();
                    e.call(VECTOR_LENGTH, &args![difference]).f32()
                });
                if limit > length {
                    return true;
                }
            }
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).ptr();
    }
    false
}

// Translated from 00553d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ray test against the cell's child nodes selected by `mask` (cdecl-like
/// with five words): for each set bit (1, 2, 4, 8, 16, and 8 or 16 again for
/// the last test) runs `004b2780(object, a, b, limit, child, 0)` on the
/// child node `00456fc0` gives for index 0, 1, 2, 3, 4 and 7. Returns whether
/// any test succeeded; false for a null `object` or an empty mask.
pub fn fn_00553d00(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    object: Ptr,
    a: u32,
    b: u32,
    limit: f32,
    mask: i8,
) -> bool {
    let mut hit = false;
    if object.is_null() || mask == 0 {
        return false;
    }
    let low = mask & 0x08 != 0;
    let high = mask & 0x10 != 0;
    let tests = [
        (mask & 0x01 != 0, 0u32),
        (mask & 0x02 != 0, 1),
        (mask & 0x04 != 0, 2),
        (low, 3),
        (high, 4),
        (low || high, 7),
    ];
    for (enabled, index) in tests {
        if enabled {
            let child = e.call(CELL_CHILD_NODE, &args![this, index]).u32();
            if e.call(RAY_TEST_NODE, &args![object, a, b, limit, child, 0u32])
                .bool()
            {
                hit = true;
            }
        }
    }
    hit
}

// Translated from 00553ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::Pick` (Xbox PDB): with a Havok world that accepts the
/// query (virtual `+0xc8(query)`) finds the pick entry of the query's world
/// shape (`00c66fd0`, `004b5a80`) and, when its type (`0043b4d0` of the
/// source `0043b4f0` gives) is `0x11`, returns the cell's own node
/// (`00545cb0`); otherwise the query's fallback node (`00c66fb0`). Without a
/// world, or when the world refuses, returns 0.
pub fn tes_object_cell_pick(e: &mut Engine, this: Ptr<TESObjectCELL>, query: Ptr) -> u32 {
    let world = e.call(CELL_HAVOK_WORLD, &args![this]).ptr::<()>();
    if world.is_null() || !e.vcall(world.addr(), 0xc8, &args![query]).bool() {
        return 0;
    }
    let shape = e.call(PICK_WORLD_SHAPE, &args![query]).u32();
    let entry = e.call(PICK_FIND_ENTRY, &args![shape]).u32();
    if entry != 0 {
        // The source is the frame's word at `EBP-0x10`: `0043b4f0` stores it there and answers
        // its address, and `0043b4d0` reads the type through that address, so the word lives
        // through the type test. (Scoped to `0043b4f0` alone, the type was read from a freed
        // block.)
        let own_node = e.with_stack(4, |e, out| {
            let source = e.call(PICK_ENTRY_SOURCE, &args![entry, out]).u32();
            e.call(PICK_ENTRY_TYPE, &args![source]).u32() == PICK_OWN_NODE_TYPE
        });
        if own_node {
            return e.call(CELL_NODE_OF_CELL, &args![this]).u32();
        }
    }
    e.call(PICK_FALLBACK_NODE, &args![query]).u32()
}

// ---------------------------------------------------------------------------
// Second batch (`00553f70` to `005574d0`): callees outside this file.

/// `(this, index)` thiscall: the address of component `index` of a vector of
/// `float`s.
const VECTOR_COMPONENT: u32 = 0x0056_0d30;
/// cdecl, one `float`: the argument times the factor at `011c582c`, in `ST0`.
const SCALE_FLOAT: u32 = 0x004a_3e90;
/// cdecl `(out, in)`: stores the three components of `in`, each through
/// [`SCALE_FLOAT`], and a zero fourth component into `out`; returns `out`.
const SCALE_VECTOR: u32 = 0x004a_3e00;
/// `(this, vector)`: the call `fn_00554720` makes with the scaled vector.
const WORLD_APPLY_VECTOR: u32 = 0x00c9_a1f0;
/// The global holding the pointer `fn_005546a0` and `fn_00554780` use, and
/// the virtual slot `fn_00554780` calls on it.
const EXTERIOR_WORLD_SLOT: u32 = 0x011c_a0d8;
const EXTERIOR_WORLD_SLOT_CALL: u32 = 0xc4;
/// The world construction of `fn_00554010`: constructor `(this, cinfo, a,
/// b)`, and the three `float` constants it scales.
const EXTERIOR_WORLD_CONSTRUCT: u32 = 0x00c9_9ff0;
const EXTERIOR_BROAD_PHASE_SIZE: u32 = 0x0102_f104;
const EXTERIOR_WORLD_EXTENT: u32 = 0x0102_f100;
const EXTERIOR_WORLD_LIMIT: u32 = 0x0102_f0fc;
/// cdecl, no arguments: the water listener of `teswaterlistener.cpp`.
const WATER_LISTENER_GET: u32 = 0x0062_f3d0;
/// The object `fn_00554590` constructs: base constructor and the vtable it
/// stores (its RTTI says `TESWindListener`); the vtable of `hkpEntityListener`
/// that `fn_00554650` stores; the destructor of the member at `+0x0c` of the
/// object `fn_005545f0` destroys.
const WIND_LISTENER_BASE_CONSTRUCT: u32 = 0x00c7_4760;
const WIND_LISTENER_VTABLE: u32 = 0x0102_f10c;
const ENTITY_LISTENER_VTABLE: u32 = 0x0102_f13c;
const LISTENER_MEMBER_DESTRUCT: u32 = 0x004e_e840;
/// cdecl `Deallocate` of the memory manager (one word).
const DEALLOCATE_BLOCK: u32 = 0x0040_1030;

/// `TESObjectCELL::GetLand` (Xbox PDB) and `TESObjectLAND::GetLandHeight`
/// `(this, position, out)` (Xbox PDB).
const CELL_GET_LAND: u32 = 0x0054_6fb0;
const LAND_GET_HEIGHT: u32 = 0x0053_f180;
/// `TESForm::GetFormTypeFromFormString` (Xbox PDB), cdecl one word; and the
/// word at `01187020` that `fn_00554810` compares the first word of its
/// argument with.
const FORM_TYPE_FROM_STRING: u32 = 0x0048_6890;
const CELL_CHILD_FORM_WORD: u32 = 0x0118_7020;

/// `fn_00554910` / `fn_00554960`: the byte at `+0x16` of the cell's Havok
/// world object (`004bae00`), its virtual `+0xd8(node, flag)` and
/// `PrepareObjectWithNoLightingRecurse` (Xbox PDB, cdecl one word).
const WORLD_BYTE_16: u32 = 0x004b_ae00;
const WORLD_PREPARE_SLOT: u32 = 0xd8;
const PREPARE_WITHOUT_LIGHTING: u32 = 0x004b_6f90;
/// `fn_00554a20`: virtual slots of a reference and of a node, the test of a
/// node against the table at `01202828` (cdecl `(table, node)`).
const REFERENCE_UPDATE_SLOT: u32 = 0x2f0;
const REFERENCE_QUERY_SLOT: u32 = 0x100;
const NODE_FLAG_SLOT: u32 = 0x14;
const NODE_TABLE_TEST: u32 = 0x0045_bad0;
const NODE_TABLE: u32 = 0x0120_2828;
/// `fn_005549d0`: the list of the cell at `+0xac` and the call made on each
/// reference `(reference, flag)`.
const CELL_LIST_AC: u32 = 0xac;
const REFERENCE_UPDATE_FOR_FLAG: u32 = 0x0057_7390;

/// The save-game buffer object (the pointer at `011de45c`,
/// [`SAVE_GAME_POINTER`]) and what the cell's save, load and size functions
/// ask of it.
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
const BUFFER_POSITION: u32 = 0x0082_5c00;
const BUFFER_VERSION: u32 = 0x008d_f040;
const BUFFER_WRITE: u32 = 0x0085_79b0;
const BUFFER_READ: u32 = 0x0085_79e0;
/// `(this)`: true when the word at `+0x48` of the object is zero.
const BUFFER_FIELD_48_IS_ZERO: u32 = 0x0055_f5b0;
/// The form the save or load is on (`004fd3e0` for saving, `004fd3c0` for
/// loading), with its flags at `+5` and version byte at `+9`.
const SAVING_FORM_INFO: u32 = 0x004f_d3e0;
const LOADING_FORM_INFO: u32 = 0x004f_d3c0;
/// The object at `011de4e8` whose byte `00408d60` finds; non-zero when the
/// size and save functions report what they wrote.
const SAVE_TRACE_HOLDER: u32 = 0x011d_e4e8;
/// Virtual `+0x130` of the form `004839c0` finds, called without arguments.
const FORM_NAME_SLOT: u32 = 0x130;
/// `Error` (cdecl, format and arguments).
const ERROR_REPORT: u32 = 0x0040_fbe0;
/// `TESForm::SaveGameDataOLD`, `LoadGameDataOLD`, `SaveNumericID` and
/// `LoadNumericID` (Xbox PDB) `(this, data, size)`; the base class's save
/// `(this, flags)`, size `(this, flags)` and `TESForm::LoadGame` (Xbox PDB)
/// `(this, flags, extra)`.
const FORM_SAVE_GAME_DATA: u32 = 0x0048_4ce0;
const FORM_LOAD_GAME_DATA: u32 = 0x0048_4d00;
const FORM_SAVE_NUMERIC_ID: u32 = 0x0048_4d20;
const FORM_LOAD_NUMERIC_ID: u32 = 0x0048_4d40;
const FORM_SAVE_BASE: u32 = 0x0048_4c20;
const FORM_SAVE_SIZE: u32 = 0x0048_4bf0;
const FORM_LOAD_GAME: u32 = 0x0048_4c50;
/// The base class's save and load against a buffer object `(this, buffer)`.
const FORM_SAVE_TO_BUFFER: u32 = 0x0048_4d60;
const FORM_LOAD_FROM_BUFFER: u32 = 0x0048_4da0;
/// Cell fields through their own functions: the detach time
/// (`TESObjectCELL::GetDetachTime`, Xbox PDB), the owner
/// (`TESObjectCELL::GetOwner`, Xbox PDB), the setter of the owner
/// `(this, form id)`, the seen data (`ExtraDataList::GetSeenData` of the
/// cell, `00555bc0`) and the string length (`0044a670`, cdecl).
const CELL_DETACH_TIME: u32 = 0x0054_6af0;
const CELL_OWNER: u32 = 0x0054_6a40;
const CELL_SET_OWNER: u32 = 0x0054_6bf0;
const CELL_SEEN_DATA: u32 = 0x0055_5bc0;
const STRING_LENGTH: u32 = 0x0044_a670;
/// `ExtraDataList::SetSeenData` (Xbox PDB) and the setter `00421850(list,
/// value)`, both on the extra data list of the cell.
const EXTRA_LIST_SET_SEEN_DATA: u32 = 0x0042_1940;
const EXTRA_LIST_SET_FIELD: u32 = 0x0042_1850;
/// The seen data objects: constructors of the interior kind (`0x2c` bytes,
/// `(this, x, y)`) and the exterior kind (`0x24` bytes).
const INTERIOR_SEEN_DATA_CONSTRUCT: u32 = 0x0087_a100;
const EXTERIOR_SEEN_DATA_CONSTRUCT: u32 = 0x0087_9940;
/// Seen data virtual slots: size `+8(0)`, save to a buffer `+0x0c`, save game
/// `+0x10(0)`, load from a buffer `+0x14`, load game `+0x18(0xffff)`.
const SEEN_DATA_SIZE_SLOT: u32 = 0x08;
const SEEN_DATA_SAVE_BUFFER_SLOT: u32 = 0x0c;
const SEEN_DATA_SAVE_GAME_SLOT: u32 = 0x10;
const SEEN_DATA_LOAD_BUFFER_SLOT: u32 = 0x14;
const SEEN_DATA_LOAD_GAME_SLOT: u32 = 0x18;
/// `00428110(buffer, out)` and `0042ce30(buffer, out)` store a word of the
/// buffer's flags into `out` and return `out`; `004280f0(flags, mask)` tests
/// it.
const BUFFER_FLAGS_GET: u32 = 0x0042_8110;
const BUFFER_FLAGS_GET_OTHER: u32 = 0x0042_ce30;
const FLAGS_TEST: u32 = 0x0042_80f0;
/// The buffer functions: `00865e50(buffer, data, size, 0)`, string
/// `00865e70(buffer, string, 0)`, form id `00865df0(buffer, id, 0)`, and the
/// loads `00864980(buffer, data, size)`, `008649a0(buffer, out)` and
/// `008648e0(buffer, out)`.
const BUFFER_SAVE_BYTES: u32 = 0x0086_5e50;
const BUFFER_SAVE_STRING: u32 = 0x0086_5e70;
const BUFFER_SAVE_FORM_ID: u32 = 0x0086_5df0;
const BUFFER_LOAD_BYTES: u32 = 0x0086_4980;
const BUFFER_LOAD_STRING: u32 = 0x0086_49a0;
const BUFFER_LOAD_FORM_ID: u32 = 0x0086_48e0;
/// `fn_005555d0`'s first call `(this, a, b)` (the map names it
/// `ProcessLists::PrintLists`, a folded body); the function of `fn_00555570`,
/// `fn_005559d0` and `fn_00555a50` that does nothing (`004534f0(this, word)`);
/// the cell calls `00546b10(this, 0, 0)` and `00555be0`.
const CELL_CALL_WITH_TWO_WORDS: u32 = 0x008d_0600;
const CELL_EMPTY_CALL: u32 = 0x0045_34f0;
const CELL_RESET_DETACH_TIME: u32 = 0x0054_6b10;
const CELL_CLEAR_SEEN_DATA: u32 = 0x0055_5be0;
/// cdecl `(buffer, 0, 0x104)`, the call that clears the name buffer of a load.
const CLEAR_BUFFER: u32 = 0x0040_3d30;
/// `fn_00555ad0` and the north rotation: `ExtraDataList`'s accessor of the
/// rotation (`00421a40`, in `ST0`), the `float` at `01012054`, the `double`
/// at `01012060`, the matrix at `011a9448`, `004a0c90(matrix, angle)` and the
/// vector rotation `004b3ae0(out, in, matrix)` (cdecl).
const EXTRA_LIST_GET_ROTATION: u32 = 0x0042_1a40;
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
const ROTATION_EPSILON: u32 = 0x0101_2060;
const ROTATION_MATRIX_BUILD: u32 = 0x004a_0c90;
const ROTATE_VECTOR: u32 = 0x004b_3ae0;
/// `GetSeenValue`: the player object (`011dea3c`), `ftol` (`00406d90`,
/// cdecl, one `float`), the index into seen data
/// `00879da0(position, interior, &x, &y)` (cdecl), `TESWorldSpace::
/// GetCellFromCellCoord` (Xbox PDB), `SeenData::IsSeenBitTrue` (Xbox PDB) and
/// the cell's bit `0x01` of `+0x25`.
const PLAYER_OBJECT: u32 = 0x011d_ea3c;
const FLOAT_TO_INT: u32 = 0x0040_6d90;
const SEEN_DATA_INDEX: u32 = 0x0087_9da0;
const WORLD_SPACE_GET_CELL: u32 = 0x0058_75a0;
const SEEN_BIT_IS_SET: u32 = 0x0087_99c0;
const CELL_SEEN_FLAG: u32 = 0x0055_6850;
/// The type descriptors the cast of `GetIntSeenSection` uses, and the cell's
/// virtual `+0x48(0x80000000)` it calls after storing a new first section.
const SEEN_DATA_CAST_SOURCE: u32 = 0x0118_b794;
const SEEN_DATA_CAST_TARGET: u32 = 0x0118_b7ac;
const CELL_CHANGED_SLOT: u32 = 0x48;
/// Reference loops: the test `005651e0`, the form type of the loops,
/// virtual slots of a reference, the marker forms and the reference tests
/// of `QueueReferences`, and its model loader call `(reference, priority,
/// 1)`.
const REFERENCE_TEST: u32 = 0x0056_51e0;
const REFERENCE_FORM_TYPE: u32 = 0x25;
const REFERENCE_OWNER_SLOT: u32 = 0x1d0;
const OWNER_QUERY_SLOT: u32 = 0x100;
const OWNER_CALL_SLOT: u32 = 0x08;
const MARKER_FORM_FIRST: u32 = 0x011c_a230;
const REFERENCE_FLAG_TEST_A: u32 = 0x0044_0da0;
const REFERENCE_FLAG_TEST_B: u32 = 0x0044_0d80;
const REFERENCE_FLAG_TEST_C: u32 = 0x0056_b250;
const REFERENCE_DEFER_SLOT: u32 = 0x1cc;
const MODEL_LOADER_QUEUE_REFERENCE: u32 = 0x0044_4850;
const CELL_PRIORITY: u32 = 0x0045_8be0;
/// `fn_005573e0`: the reference's model (`0043fcd0`), its virtual `+0x10`, the
/// call `00476ab0` and `BSShaderUtil::RecursiveSetPropertyFadeAlpha` (Xbox
/// PDB, cdecl `(node, alpha)`).
const REFERENCE_MODEL: u32 = 0x0043_fcd0;
const MODEL_TEST_SLOT: u32 = 0x10;
const MODEL_PREPARE: u32 = 0x0047_6ab0;
const SET_PROPERTY_FADE_ALPHA: u32 = 0x00b6_bb30;
/// `fn_00557470` and `fn_005574d0`: the collection object (`011c95c8`) and
/// its calls `(this, reference)`.
const LOADED_REFERENCES: u32 = 0x011c_95c8;
const LOADED_REFERENCES_REMOVE: u32 = 0x0052_6f20;
const LOADED_REFERENCES_ADD: u32 = 0x0052_70b0;
/// `fn_005574d0`: the scene's cell getter (`005f36f0`, the word at `+0x34`),
/// the form test (`00549580`: bit `0x40` of the flags word at `+8`), the
/// reference's world space (`TESObjectREFR::GetWorldSpace`, Xbox PDB) and
/// the calls on the world's terrain manager.
const SCENE_CELL: u32 = 0x005f_36f0;
const SCENE_WORLD_SPACE: u32 = 0x004f_d3e0;
const BASE_FORM_FLAG_40: u32 = 0x0054_9580;
const REFERENCE_WORLD_SPACE: u32 = 0x0057_5d70;
const WORLD_GET_TERRAIN_MANAGER: u32 = 0x0058_6170;
const TERRAIN_HIDE_TREE: u32 = 0x006f_cfa0;
const TERRAIN_UPDATE_TREE: u32 = 0x006f_d010;
const TERRAIN_SET_FLAG: u32 = 0x0092_9260;

// ---------------------------------------------------------------------------
// Second batch: helpers.

/// Helper: the flag test of the buffer functions, `(buffer flags &
/// mask) != 0`, where `getter` (`00428110` or `0042ce30`) stores the buffer's
/// flag word into a local and returns its address.
fn buffer_has_flag(e: &mut Engine, buffer: Ptr, getter: u32, mask: u32) -> bool {
    e.with_stack(4, |e, local| {
        let flags = e.call(getter, &args![buffer, local]).u32();
        e.call(FLAGS_TEST, &args![flags, mask]).bool()
    })
}

/// Helper: the seen data of a cell, created and stored in the cell's extra
/// data list when it has none (an interior cell gets the `0x2c` byte kind,
/// an exterior the `0x24` byte kind). Shared by the load functions
/// `fn_00554fd0` and `fn_00555740`.
fn seen_data_or_create(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    let seen = e.call(CELL_SEEN_DATA, &args![this]).u32();
    if seen != 0 {
        return seen;
    }
    let created = if e.call(CELL_IS_INTERIOR, &args![this]).bool() {
        let block = e.call(OPERATOR_NEW, &args![0x2cu32]).u32();
        if block == 0 {
            0
        } else {
            e.call(INTERIOR_SEEN_DATA_CONSTRUCT, &args![block, 0u32, 0u32])
                .u32()
        }
    } else {
        let block = e.call(OPERATOR_NEW, &args![0x24u32]).u32();
        if block == 0 {
            0
        } else {
            e.call(EXTERIOR_SEEN_DATA_CONSTRUCT, &args![block]).u32()
        }
    };
    let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).u32();
    e.call(EXTRA_LIST_SET_SEEN_DATA, &args![list, created]);
    created
}

/// Helper: the packed cell-flag byte of a save: `(cCellFlags & 0x60) |
/// cCellGameFlags`.
fn packed_cell_flags(e: &Engine, this: Ptr<TESObjectCELL>) -> u8 {
    (e.get(this, TESObjectCELL::cCellFlags) & 0x60) | e.get(this, TESObjectCELL::cCellGameFlags)
}

/// Helper: the reverse of [`packed_cell_flags`], as the two loads do it.
fn unpack_cell_flags(e: &mut Engine, this: Ptr<TESObjectCELL>, packed: u8) {
    let flags = (e.get(this, TESObjectCELL::cCellFlags) & 0x9f) | (packed & 0x60);
    e.set(this, TESObjectCELL::cCellFlags, flags);
    e.set(this, TESObjectCELL::cCellGameFlags, packed & 0x9f);
}

/// Helper: the form `004839c0` finds for the form id at the start of `info`
/// (the form being saved or loaded), or 0 without `info`.
fn find_form_of(e: &mut Engine, info: u32) -> u32 {
    if info == 0 {
        return 0;
    }
    let form_id = e.mem.u32(info);
    e.call(FORM_LOOK_UP, &args![form_id]).u32()
}

/// Helper: the name (virtual `+0x130`) of the form being saved, for the
/// trace messages of the size and save functions.
fn traced_form_name(e: &mut Engine, info: u32) -> u32 {
    let form = find_form_of(e, info);
    e.vcall(form, FORM_NAME_SLOT, &[]).u32()
}

/// Helper: the trace line of `fn_00554b90` and `fn_00554d30` (`Error` with
/// the number of bytes written since `start`): the form's version names it
/// when the save has a current form.
fn report_saved_bytes(
    e: &mut Engine,
    with_form_format: u32,
    plain_format: u32,
    bytes: u32,
    line: u32,
) {
    let buffer: u32 = e.global(SAVE_GAME_POINTER);
    let info = e.call(SAVING_FORM_INFO, &args![buffer]).u32();
    if info == 0 {
        e.call(
            ERROR_REPORT,
            &args![plain_format, bytes, line, SOURCE_FILE_NAME],
        );
    } else {
        let name = traced_form_name(e, info);
        let form_id = e.mem.u32(info);
        let flags = e.mem.u32(info + 5);
        e.call(
            ERROR_REPORT,
            &args![
                with_form_format,
                bytes,
                form_id,
                name,
                flags,
                line,
                SOURCE_FILE_NAME
            ],
        );
    }
}

/// Helper: the report of `fn_00554fd0` when the block header or the block
/// length is wrong (`DEBUG_PRINT`, the `SAVELOAD:` messages). `formats` are
/// the messages with and without a current form; `bytes` is present for the
/// overrun and underrun messages; `info` is the form being loaded (0 for
/// none) and `form` what `004839c0` found for it.
fn report_load_problem(
    e: &mut Engine,
    formats: (u32, u32),
    bytes: Option<u32>,
    line: u32,
    (info, form): (u32, u32),
) {
    let buffer: u32 = e.global(SAVE_GAME_POINTER);
    let mut words = vec![];
    if info == 0 {
        let version = e.call(BUFFER_VERSION, &args![buffer]).u8() as u32;
        words.push(formats.1);
        words.extend(bytes);
        words.extend([SOURCE_FILE_NAME, line, version]);
    } else {
        let name = e.vcall(form, FORM_NAME_SLOT, &[]).u32();
        let form_id = e.mem.u32(info);
        let version = e.mem.u8(info + 9) as u32;
        let flags = e.mem.u32(info + 5);
        words.push(formats.0);
        words.extend(bytes);
        words.extend([SOURCE_FILE_NAME, line, form_id, name, version, flags]);
    }
    e.call(DEBUG_PRINT, &words);
}

/// Helper: runs `visit` on the item of every node of the list that starts at
/// `first` (`fn_005570d0`'s loops), following `00726070` after each item.
fn for_each_list_item(e: &mut Engine, first: u32, mut visit: impl FnMut(&mut Engine, Ptr)) {
    let mut node = first;
    while node != 0 {
        let item = node_item(e, Ptr::new(node));
        visit(e, item);
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

/// Helper: `visit` on every reference of the cell, under the cell lock
/// (`00541ac0` / `00541ae0`), as `fn_005570d0`, `fn_005573e0`, `fn_00557470`
/// and `fn_005574d0` do.
fn for_each_cell_reference_locked(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    visit: impl FnMut(&mut Engine, Ptr),
) {
    e.call(CELL_LOCK_ENTER, &args![this]);
    let first = e.call(CELL_REFERENCES, &args![this]).u32();
    for_each_list_item(e, first, visit);
    e.call(CELL_LOCK_LEAVE, &args![this]);
}

// ---------------------------------------------------------------------------
// Second batch: the functions.

// Translated from 00553f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the four `float`s at `other` to the four at `this` (`ADDPS`, on
/// 16-byte aligned vectors). The map has no name for it.
pub fn fn_00553f70(e: &mut Engine, this: Ptr, other: Ptr) {
    let mut sum = [0f32; 4];
    for (index, slot) in sum.iter_mut().enumerate() {
        let offset = 4 * index as u32;
        *slot = e.mem.f32(this.addr() + offset) + e.mem.f32(other.addr() + offset);
    }
    for (index, value) in sum.into_iter().enumerate() {
        e.mem.set_f32(this.addr() + 4 * index as u32, value);
    }
}

// Translated from 00553fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(destination, source)`: copies the three `float`s at `source` into
/// components 0 to 2 of `destination` (through `00560d30`) and stores `0.0`
/// as component 3. Returns `destination`.
pub fn fn_00553fc0(e: &mut Engine, destination: Ptr, source: Ptr) -> Ptr {
    for index in 0..3u32 {
        let component = e.call(VECTOR_COMPONENT, &args![destination, index]).u32();
        let value = e.mem.f32(source.addr() + 4 * index);
        e.mem.set_f32(component, value);
    }
    let component = e.call(VECTOR_COMPONENT, &args![destination, 3u32]).u32();
    e.mem.set_f32(component, 0.0);
    destination
}

// Translated from 00554010 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(centre)`: builds the Havok world of an exterior (the map has no
/// name for it): a world cinfo with gravity `(0, 0, g, 0)`, the scaled broad
/// phase size and scale `1.0`; two scaled extent vectors; the world itself
/// (`0xd0` bytes, aligned to 16) with its listener, broad phase, water
/// listener, filter, scene setting and wind listener, registered with the
/// `011ca088` singleton; the world is positioned at the three floats of
/// `centre` (the third replaced by `0.0`), gets its virtual `+0xc4`, the
/// visual debugger setting, and is returned. The cinfo is destroyed. The
/// exception frames and the stack cookie are not translated.
pub fn fn_00554010(e: &mut Engine, centre: Ptr) -> Ptr {
    with_scope_guard(e, 0x10, 0x3c1b, |e| {
        // The cinfo, then the position, the extent and the limit vectors
        // (16 bytes each), as one block of the game's stack.
        e.with_stack(CINFO_SIZE + 0x30, |e, block| {
            let cinfo = block;
            let position = block.byte_add(CINFO_SIZE);
            let extent = block.byte_add(CINFO_SIZE + 0x10);
            let limit = block.byte_add(CINFO_SIZE + 0x20);
            e.call(CINFO_CONSTRUCT, &args![cinfo]);
            let gravity: f32 = e.global(GRAVITY_Z);
            e.with_stack(16, |e, vector| {
                let vector = fn_005532a0(e, vector, 0.0, 0.0, gravity, 0.0);
                e.call(CINFO_SET_GRAVITY, &args![cinfo.byte_add(0x10), vector]);
            });
            let size: f32 = e.global(EXTERIOR_BROAD_PHASE_SIZE);
            let scaled = e.call(SCALE_FLOAT, &args![size]).f32();
            e.call(CINFO_SET_BROAD_PHASE_SIZE, &args![cinfo, scaled]);
            e.call(LIST_NODE_ITEM_ADDRESS, &args![position]);
            for index in 0..3u32 {
                let value = e.mem.u32(centre.addr() + 4 * index);
                e.mem.set_u32(position.addr() + 4 * index, value);
            }
            e.mem.set_f32(position.addr() + 8, 0.0);
            e.mem.set_f32(cinfo.addr() + 0x54, 1.0);
            for (vector, constant) in [
                (extent, EXTERIOR_WORLD_EXTENT),
                (limit, EXTERIOR_WORLD_LIMIT),
            ] {
                e.call(LIST_NODE_ITEM_ADDRESS, &args![vector]);
                for index in 0..3u32 {
                    let value: f32 = e.global(constant);
                    let scaled = e.call(SCALE_FLOAT, &args![value]).f32();
                    let component = e.call(VECTOR_COMPONENT, &args![vector, index]).u32();
                    e.mem.set_f32(component, scaled);
                }
            }
            let raw = fn_00553300(e, 0xd0);
            let world: Ptr = if raw == 0 {
                Ptr::NULL
            } else {
                e.call(EXTERIOR_WORLD_CONSTRUCT, &args![raw, cinfo, extent, limit])
                    .ptr()
            };
            let held = e.call(HOLDER_GET_WORLD, &args![world]).u32();
            e.call(EMPTY_FUNCTION, &args![held]);
            let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
            let wind_listener = if block == 0 {
                0
            } else {
                fn_00554590(e, Ptr::new(block)).addr()
            };
            fn_005533f0(e, world, wind_listener);
            let block = e.call(OPERATOR_NEW, &args![0x24u32]).u32();
            let broad_phase = if block == 0 {
                0
            } else {
                e.call(BROAD_PHASE_CONSTRUCT, &args![block]).u32()
            };
            fn_00553480(e, world, if broad_phase == 0 { 0 } else { broad_phase + 8 });
            let water_listener = e.call(WATER_LISTENER_GET, &[]).u32();
            e.call(WORLD_SET_GRAVITY_SCALE, &args![world, water_listener]);
            let block = e.call(OPERATOR_NEW, &args![0x28u32]).u32();
            let filter = if block == 0 {
                0
            } else {
                e.call(FILTER_CONSTRUCT, &args![block]).u32()
            };
            fn_005534f0(e, world, filter);
            let setting = e.call(SCENE_HAVOK_SETTING, &[]).u32();
            fn_00553440(e, world, setting);
            let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
            let listener = if block == 0 {
                0
            } else {
                e.call(LISTENER_OBJECT_CONSTRUCT, &args![block]).u32()
            };
            fn_005533f0(e, world, listener);
            if e.global::<u32>(LISTENER_SINGLETON) == 0 {
                let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
                let singleton = if block == 0 {
                    0
                } else {
                    e.call(LISTENER_CONSTRUCT, &args![block]).u32()
                };
                e.set_global(LISTENER_SINGLETON, singleton);
            }
            let held = e.call(HOLDER_GET_WORLD, &args![world]).u32();
            let singleton: u32 = e.global(LISTENER_SINGLETON);
            e.call(LISTENER_REGISTER, &args![singleton, held]);
            e.call(WORLD_SET_CENTER, &args![world, position]);
            let held = e.call(HOLDER_GET_WORLD, &args![world]).u32();
            e.call(EMPTY_FUNCTION, &args![held]);
            e.vcall(world.addr(), 0xc4, &[]);
            let suppressed = e.call(VISUAL_DEBUGGER_SUPPRESSED, &[]).bool();
            e.call(
                WORLD_SET_VISUAL_DEBUGGER_ENABLED,
                &args![world, !suppressed],
            );
            e.call(WORLD_SET_VISUAL_DEBUGGER, &args![world, 0u32]);
            fn_00553520(e, cinfo);
            world
        })
    })
}

// Translated from 00554590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the object whose vtable is `0102f10c` (its RTTI says
/// `TESWindListener`, derived from `bhkWindListener`): runs the base
/// constructor `00c74760` and stores the vtable. Returns `this`.
pub fn fn_00554590(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(WIND_LISTENER_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), WIND_LISTENER_VTABLE);
    this
}

// Translated from 005545b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `fn_005545d0`.
pub fn fn_005545b0(e: &mut Engine, this: Ptr) {
    fn_005545d0(e, this);
}

// Translated from 005545d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `fn_005545f0`.
pub fn fn_005545d0(e: &mut Engine, this: Ptr) {
    fn_005545f0(e, this);
}

// Translated from 005545f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of a class derived from `hkpEntityListener` (the map names the
/// folded body `std::_Ref_count_del_alloc<..>::~_Ref_count_del_alloc<..>`):
/// destroys the member at `+0x0c` (`004ee840`), then runs `fn_00554650`. The
/// exception frame is not translated.
pub fn fn_005545f0(e: &mut Engine, this: Ptr) {
    e.call(LISTENER_MEMBER_DESTRUCT, &args![this.byte_add(0x0c)]);
    fn_00554650(e, this);
}

// Translated from 00554650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0102f13c` (the RTTI of that vtable says
/// `hkpEntityListener`) at `this`.
pub fn fn_00554650(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ENTITY_LISTENER_VTABLE);
}

// Translated from 00554670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the class of `fn_00554650`: stores its
/// vtable and, when bit 0 of `flags` is set, frees the block with the memory
/// manager's `Deallocate` (`00401030`). Returns `this`.
pub fn fn_00554670(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00554650(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE_BLOCK, &args![this]);
    }
    this
}

// Translated from 005546a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::MoveExteriorWorld` (Xbox PDB), cdecl `(shift)`: under an
/// allocation scope guard, reads the pointer in the global `011ca0d8` (the
/// getter `00559450`) and runs `fn_00554720` on it with `shift`. The
/// exception frame is not translated.
pub fn tes_object_cell_move_exterior_world(e: &mut Engine, shift: Ptr) {
    with_scope_guard(e, 0x10, 0x3c5b, |e| {
        let world = e.call(SLOT_GET_POINTER, &args![EXTERIOR_WORLD_SLOT]).u32();
        fn_00554720(e, Ptr::new(world), shift);
    });
}

// Translated from 00554720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `(this, vector)`: scales the three floats of `vector` into a local
/// vector (`004a3e00`) and gives it to `00c9a1f0` on `this`. The map has no
/// name for it.
pub fn fn_00554720(e: &mut Engine, this: Ptr, vector: Ptr) {
    e.with_stack(16, |e, scaled| {
        e.call(LIST_NODE_ITEM_ADDRESS, &args![scaled]);
        let scaled = e.call(SCALE_VECTOR, &args![scaled, vector]).u32();
        e.call(WORLD_APPLY_VECTOR, &args![this, scaled]);
    });
}

// Translated from 00554780 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the global `011ca0d8` holds a pointer (`00559450`), calls its virtual
/// `+0xc4`. No arguments.
pub fn fn_00554780(e: &mut Engine) {
    if e.call(SLOT_GET_POINTER, &args![EXTERIOR_WORLD_SLOT]).u32() != 0 {
        let world = e.call(SLOT_GET_POINTER, &args![EXTERIOR_WORLD_SLOT]).u32();
        e.vcall(world, EXTERIOR_WORLD_SLOT_CALL, &[]);
    }
}

// Translated from 005547c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::GetLandHeight` (Xbox PDB) `(this, position, out)`: with
/// the cell's land (`TESObjectCELL::GetLand`) answers
/// `TESObjectLAND::GetLandHeight(position, out)`; without land stores `0.0`
/// into `out` and answers 0.
pub fn tes_object_cell_get_land_height(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    position: Ptr,
    out: Ptr,
) -> u8 {
    let land = e.call(CELL_GET_LAND, &args![this]).u32();
    if land == 0 {
        e.mem.set_f32(out.addr(), 0.0);
        0
    } else {
        e.call(LAND_GET_HEIGHT, &args![land, position, out]).u8()
    }
}

// Translated from 00554810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::IsFormCellChild` (Xbox PDB), cdecl `(form)`: 0 for null.
/// When the first word of `form` equals the word at `01187020`, true for
/// the values 6, 8 and 9 of the word at `+0x0c`; otherwise asks
/// `TESForm::GetFormTypeFromFormString` with the first word and answers
/// `fn_005548a0` on the result.
pub fn tes_object_cell_is_form_cell_child(e: &mut Engine, form: Ptr) -> u8 {
    if form.is_null() {
        return 0;
    }
    let first = e.mem.u32(form.addr());
    let expected: u32 = e.global(CELL_CHILD_FORM_WORD);
    if first == expected {
        let kind = e.mem.u32(form.addr() + 0x0c);
        u8::from(kind == 6 || (kind > 7 && kind <= 9))
    } else {
        let kind = e.call(FORM_TYPE_FROM_STRING, &args![first]).u32();
        u8::from(fn_005548a0(e, kind as u8))
    }
}

// Translated from 00554880 (decompiled, FalloutNV.exe 1.4.0.525)
/// thiscall wrapper that ignores `this` and answers `fn_005548a0(kind)`.
pub fn fn_00554880(e: &mut Engine, _unused_0: Ptr, kind: u8) -> bool {
    fn_005548a0(e, kind)
}

// Translated from 005548a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(kind)`: true for the form type bytes `0x3a` to `0x40`, `0x42`,
/// `0x43` and `0x69` (a jump table). The names of those types are not
/// confirmed from the exe.
pub fn fn_005548a0(_e: &mut Engine, kind: u8) -> bool {
    matches!(kind, 0x3a..=0x40 | 0x42 | 0x43 | 0x69)
}

// Translated from 00554910 (decompiled, FalloutNV.exe 1.4.0.525)
/// With the cell's Havok world (`004543c0`): sets `flag = (byte at +0x16 of
/// the world object (004bae00) == 0)`, runs `fn_00554960(flag)` and answers
/// the flag; 0 without a world.
pub fn fn_00554910(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u8 {
    let world = e.call(CELL_HAVOK_WORLD, &args![this]).u32();
    if world == 0 {
        return 0;
    }
    let flag = u8::from(e.call(WORLD_BYTE_16, &args![world]).u8() == 0);
    fn_00554960(e, this, flag);
    flag
}

// Translated from 00554960 (decompiled, FalloutNV.exe 1.4.0.525)
/// With the cell's Havok world and the cell's 3D (`TESObjectCELL::Get3D`):
/// calls the world's virtual `+0xd8(node, flag)`, `fn_00554a20(flag)` and
/// `PrepareObjectWithNoLightingRecurse(node)`.
pub fn fn_00554960(e: &mut Engine, this: Ptr<TESObjectCELL>, flag: u8) {
    let world = e.call(CELL_HAVOK_WORLD, &args![this]).u32();
    let node = e.call(CELL_NODE_OF_CELL, &args![this]).u32();
    if world != 0 && node != 0 {
        e.vcall(world, WORLD_PREPARE_SLOT, &args![node, flag]);
        fn_00554a20(e, this, flag);
        e.call(PREPARE_WITHOUT_LIGHTING, &args![node]);
    }
}

// Translated from 005549d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the list at `+0xac` of the cell; stops at the first node without an
/// item and calls `00577390(item, flag)` on every item before it.
pub fn fn_005549d0(e: &mut Engine, this: Ptr<TESObjectCELL>, flag: u8) {
    let mut node = this.addr() + CELL_LIST_AC;
    while node != 0 {
        let item = node_item(e, Ptr::new(node));
        if item.is_null() {
            return;
        }
        let item = node_item(e, Ptr::new(node));
        e.call(REFERENCE_UPDATE_FOR_FLAG, &args![item, flag]);
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 00554a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates references for `flag`: first, from the last to the first, the
/// children of the cell's node (`0054a050`, `0043b480` and `0043b4a0`
/// give them) that have a reference (`TESObjectREFR::FindReferenceFor3D`)
/// get the reference's virtual `+0x2f0(flag)`. Then, for every child of
/// `00456fc0(cell, 7)` that answers true to its virtual `+0x14` or is in
/// the table at `01202828` (`0045bad0`), every one of its own children whose
/// reference answers true to virtual `+0x100` gets `+0x2f0(flag)`.
pub fn fn_00554a20(e: &mut Engine, this: Ptr<TESObjectCELL>, flag: u8) {
    let node = e.call(CELL_NODE_B, &args![this]).u32();
    let mut remaining = e.call(NODE_CHILD_COUNT, &args![node]).u32();
    while remaining != 0 {
        remaining -= 1;
        let child = e.call(NODE_CHILD_AT, &args![node, remaining]).u32();
        let reference = e.call(FIND_REFERENCE_FOR_3D, &args![child]).u32();
        if reference != 0 {
            e.vcall(reference, REFERENCE_UPDATE_SLOT, &args![flag]);
        }
    }
    let group = e.call(CELL_CHILD_NODE, &args![this, 7u32]).u32();
    let mut index = 0u32;
    while index < e.call(NODE_CHILD_COUNT, &args![group]).u32() {
        let child = e.call(NODE_CHILD_AT, &args![group, index]).u32();
        index += 1;
        if child == 0 {
            continue;
        }
        let qualifies = e.vcall(child, NODE_FLAG_SLOT, &[]).u32() != 0
            || e.call(NODE_TABLE_TEST, &args![NODE_TABLE, child]).bool();
        if !qualifies {
            continue;
        }
        let mut inner = 0u32;
        while inner < e.call(NODE_CHILD_COUNT, &args![child]).u32() {
            let grandchild = e.call(NODE_CHILD_AT, &args![child, inner]).u32();
            inner += 1;
            if grandchild != 0 {
                let reference = e.call(FIND_REFERENCE_FOR_3D, &args![grandchild]).u32();
                if e.vcall(reference, REFERENCE_QUERY_SLOT, &[]).bool() {
                    e.vcall(reference, REFERENCE_UPDATE_SLOT, &args![flag]);
                }
            }
        }
    }
}

// Translated from 00554b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The size of a cell's save-game record for `flags` (the map has no name for
/// it; its trace message says `GetSaveSize`): `4` for flag `0x40000000`, the
/// base class size (`00484bf0`), `6` when save blocks are used, `1` for flag
/// 2, the seen data's size (virtual `+8`) for `0x80000000`, `1` plus the
/// length of the cell's name for 4, `4` for 8. The sum is a 16-bit value.
/// When the trace holder (`011de4e8`) is set it reports the bytes added
/// after the base class's part through `Error`.
pub fn fn_00554b90(e: &mut Engine, this: Ptr<TESObjectCELL>, flags: u32) -> u16 {
    let mut size: u16 = 0;
    if flags & 0x4000_0000 != 0 {
        size = size.wrapping_add(4);
    }
    size = size.wrapping_add(e.call(FORM_SAVE_SIZE, &args![this, flags]).u16());
    let base = size;
    let buffer: u32 = e.global(SAVE_GAME_POINTER);
    if e.call(USE_SAVE_GAME_BLOCKS, &args![buffer]).bool() {
        size = size.wrapping_add(4).wrapping_add(2);
    }
    if flags & 2 != 0 {
        size = size.wrapping_add(1);
    }
    if flags & 0x8000_0000 != 0 {
        let seen = e.call(CELL_SEEN_DATA, &args![this]).u32();
        size = size.wrapping_add(e.vcall(seen, SEEN_DATA_SIZE_SLOT, &args![0u32]).u16());
    }
    if flags & 4 != 0 {
        size = size.wrapping_add(1);
        let name = e.call(TEXT_GET_STRING, &args![this.byte_add(0x18)]).u32();
        size = size.wrapping_add(e.call(STRING_LENGTH, &args![name]).u16());
    }
    if flags & 8 != 0 {
        size = size.wrapping_add(4);
    }
    let holder = e.call(BYTE_HOLDER_GET, &args![SAVE_TRACE_HOLDER]).u32();
    if e.mem.u8(holder) != 0 {
        let bytes = (size as u32).wrapping_sub(base as u32);
        report_saved_bytes(e, 0x0101_2cb0, 0x0101_2c78, bytes, 0x3d51);
    }
    size
}

// Translated from 00554d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the cell's save-game record for `flags` to the save-game buffer
/// (`011de45c`): the detach time for `0x40000000`; the base class part
/// (`00484c20`); when save blocks are used, the marker `BLOK` and a 16-bit
/// length slot, filled at the end with the number of bytes written; the
/// packed flags byte `(cCellFlags & 0x60) | cCellGameFlags` for 2; the seen
/// data (virtual `+0x10`) for `0x80000000`; the name's length byte and
/// characters for 4; the owner's form id for 8. When the trace holder is set
/// it reports the bytes written through `Error`; a block longer than 0xffff
/// bytes is reported through `005b5e40`. The exception frame is not
/// translated.
pub fn fn_00554d30(e: &mut Engine, this: Ptr<TESObjectCELL>, flags: u32) {
    e.with_stack(0x14, |e, locals| {
        let magic = locals;
        let length_slot = locals.byte_add(4);
        let flags_byte = locals.byte_add(8);
        let name_length = locals.byte_add(9);
        let owner_id = locals.byte_add(12);
        let detach_time = locals.byte_add(16);
        if flags & 0x4000_0000 != 0 {
            let time = e.call(CELL_DETACH_TIME, &args![this]).u32();
            e.mem.set_u32(detach_time.addr(), time);
            e.call(FORM_SAVE_GAME_DATA, &args![this, detach_time, 4u32]);
        }
        e.call(FORM_SAVE_BASE, &args![this, flags]);
        e.mem.set_u16(length_slot.addr(), 0);
        let mut block_start = 0u32;
        let buffer: u32 = e.global(SAVE_GAME_POINTER);
        let mut start = e.call(BUFFER_POSITION, &args![buffer]).u32();
        let holder = e.call(BYTE_HOLDER_GET, &args![SAVE_TRACE_HOLDER]).u32();
        if e.mem.u8(holder) != 0 {
            let buffer: u32 = e.global(SAVE_GAME_POINTER);
            start = e.call(BUFFER_POSITION, &args![buffer]).u32();
        }
        let buffer: u32 = e.global(SAVE_GAME_POINTER);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![buffer]).bool() {
            e.mem.set_u32(magic.addr(), 0x424c_4f4b);
            e.call(BUFFER_WRITE, &args![buffer, magic, 4u32]);
            block_start = e.call(BUFFER_POSITION, &args![buffer]).u32();
            e.call(BUFFER_WRITE, &args![buffer, length_slot, 2u32]);
        }
        if flags & 2 != 0 {
            let packed = packed_cell_flags(e, this);
            e.mem.set_u8(flags_byte.addr(), packed);
            e.call(FORM_SAVE_GAME_DATA, &args![this, flags_byte, 1u32]);
        }
        if flags & 0x8000_0000 != 0 {
            let seen = e.call(CELL_SEEN_DATA, &args![this]).u32();
            e.vcall(seen, SEEN_DATA_SAVE_GAME_SLOT, &args![0u32]);
        }
        if flags & 4 != 0 {
            let name = e.call(TEXT_GET_STRING, &args![this.byte_add(0x18)]).u32();
            let length = e.call(STRING_LENGTH, &args![name]).u8();
            e.mem.set_u8(name_length.addr(), length);
            e.call(FORM_SAVE_GAME_DATA, &args![this, name_length, 1u32]);
            if length != 0 {
                e.call(FORM_SAVE_GAME_DATA, &args![this, name, length]);
            }
        }
        if flags & 8 != 0 {
            let owner = e.call(CELL_OWNER, &args![this]).u32();
            let mut id = 0;
            if owner != 0 {
                id = e.call(FORM_ID, &args![owner]).u32();
            }
            e.mem.set_u32(owner_id.addr(), id);
            e.call(FORM_SAVE_NUMERIC_ID, &args![this, owner_id, 4u32]);
        }
        let holder = e.call(BYTE_HOLDER_GET, &args![SAVE_TRACE_HOLDER]).u32();
        if e.mem.u8(holder) != 0 {
            let buffer: u32 = e.global(SAVE_GAME_POINTER);
            let position = e.call(BUFFER_POSITION, &args![buffer]).u32();
            report_saved_bytes(
                e,
                0x0101_53a0,
                0x0101_536c,
                position.wrapping_sub(start),
                0x3d86,
            );
        }
        let buffer: u32 = e.global(SAVE_GAME_POINTER);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![buffer]).bool() {
            let position = e.call(BUFFER_POSITION, &args![buffer]).u32();
            if position > block_start.wrapping_add(0xffff) {
                e.call(
                    DEBUG_PRINT,
                    &args![0x0101_5318u32, SOURCE_FILE_NAME, 0x3d86u32],
                );
            }
            e.mem
                .set_u16(block_start, position.wrapping_sub(block_start) as u16);
        }
    });
}

// Translated from 00554fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the cell's save-game record for `flags` (`extra` goes to the base
/// class's `TESForm::LoadGame`). From the buffer version `0x5a`, the detach
/// time for `0x40000000` is read first (stored through the extra data
/// list's `00421850`); older versions read it after the seen data. When
/// save blocks are used, the marker `BLOK` is checked (a wrong one is
/// reported through `005b5e40`) and the 16-bit length read; flag 2 unpacks
/// the flags byte into `cCellFlags` and `cCellGameFlags`; `0x80000000` reads
/// the seen data (created when the cell has none, virtual `+0x18(0xffff)`);
/// 4 reads the name (length byte and characters, cleared first) into
/// `TESSoundFile::SetSoundFile` of `this + 0x18`; 8 reads the owner's form
/// id. Finally the position is compared with the block's end and an overrun
/// or underrun is reported. The exception frame is not translated.
pub fn fn_00554fd0(e: &mut Engine, this: Ptr<TESObjectCELL>, flags: u32, extra: u32) {
    e.with_stack(0x120, |e, locals| {
        let time = locals;
        let magic = locals.byte_add(4);
        let length_slot = locals.byte_add(8);
        let flags_byte = locals.byte_add(12);
        let name_length = locals.byte_add(13);
        let owner_id = locals.byte_add(16);
        let name_buffer = locals.byte_add(0x14);
        let buffer: u32 = e.global(SAVE_GAME_POINTER);
        let version = e.call(BUFFER_VERSION, &args![buffer]).u8();
        if version >= 0x5a && flags & 0x4000_0000 != 0 {
            e.call(FORM_LOAD_GAME_DATA, &args![this, time, 4u32]);
            let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).u32();
            let value = e.mem.u32(time.addr());
            e.call(EXTRA_LIST_SET_FIELD, &args![list, value]);
        }
        e.call(FORM_LOAD_GAME, &args![this, flags, extra]);
        e.mem.set_u16(length_slot.addr(), 0);
        let mut block_start = 0u32;
        let buffer: u32 = e.global(SAVE_GAME_POINTER);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![buffer]).bool() {
            e.call(BUFFER_READ, &args![buffer, magic, 4u32]);
            if e.mem.u32(magic.addr()) != 0x424c_4f4b {
                let info = e.call(LOADING_FORM_INFO, &args![buffer]).u32();
                let form = find_form_of(e, info);
                report_load_problem(e, (0x0101_5718, 0x0101_56a8), None, 0x3d9d, (info, form));
            }
            block_start = e.call(BUFFER_POSITION, &args![buffer]).u32();
            e.call(BUFFER_READ, &args![buffer, length_slot, 2u32]);
        }
        if flags & 2 != 0 {
            e.call(FORM_LOAD_GAME_DATA, &args![this, flags_byte, 1u32]);
            let packed = e.mem.u8(flags_byte.addr());
            unpack_cell_flags(e, this, packed);
        }
        if flags & 0x8000_0000 != 0 {
            let seen = seen_data_or_create(e, this);
            e.vcall(seen, SEEN_DATA_LOAD_GAME_SLOT, &args![0xffffu32]);
        }
        let buffer: u32 = e.global(SAVE_GAME_POINTER);
        let version = e.call(BUFFER_VERSION, &args![buffer]).u8();
        if version < 0x5a && flags & 0x4000_0000 != 0 {
            e.call(FORM_LOAD_GAME_DATA, &args![this, time, 4u32]);
            let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).u32();
            let value = e.mem.u32(time.addr());
            e.call(EXTRA_LIST_SET_FIELD, &args![list, value]);
        }
        if flags & 4 != 0 {
            e.call(CLEAR_BUFFER, &args![name_buffer, 0u32, 0x104u32]);
            e.call(FORM_LOAD_GAME_DATA, &args![this, name_length, 1u32]);
            let length = e.mem.u8(name_length.addr());
            if length != 0 {
                e.call(FORM_LOAD_GAME_DATA, &args![this, name_buffer, length]);
            }
            e.call(FULL_NAME_SET, &args![this.byte_add(0x18), name_buffer]);
        }
        if flags & 8 != 0 {
            e.call(FORM_LOAD_NUMERIC_ID, &args![this, owner_id, 4u32]);
            let id = e.mem.u32(owner_id.addr());
            e.call(CELL_SET_OWNER, &args![this, id]);
        }
        let buffer: u32 = e.global(SAVE_GAME_POINTER);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![buffer]).bool() {
            let position = e.call(BUFFER_POSITION, &args![buffer]).u32();
            let info = e.call(LOADING_FORM_INFO, &args![buffer]).u32();
            let expected = (e.mem.u16(length_slot.addr()) as u32).wrapping_add(block_start);
            let form = find_form_of(e, info);
            if position > expected {
                report_load_problem(
                    e,
                    (0x0101_5588, 0x0101_54a0),
                    Some(position.wrapping_sub(expected)),
                    0x3dd5,
                    (info, form),
                );
            } else if position < expected {
                report_load_problem(
                    e,
                    (0x0101_5500, 0x0101_5440),
                    Some(expected.wrapping_sub(position)),
                    0x3dd5,
                    (info, form),
                );
            }
        }
    });
}

// Translated from 00555570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `004534f0(this, flags)`; when `0055f5b0` on the save-game buffer
/// is true clears `cCellGameFlags`; for `0x80000000` calls `00555be0`, for
/// `0x40000000` calls `00546b10(this, 0, 0)`.
pub fn fn_00555570(e: &mut Engine, this: Ptr<TESObjectCELL>, flags: u32) {
    e.call(CELL_EMPTY_CALL, &args![this, flags]);
    let buffer: u32 = e.global(SAVE_GAME_POINTER);
    if e.call(BUFFER_FIELD_48_IS_ZERO, &args![buffer]).bool() {
        e.set(this, TESObjectCELL::cCellGameFlags, 0);
    }
    if flags & 0x8000_0000 != 0 {
        e.call(CELL_CLEAR_SEEN_DATA, &args![this]);
    }
    if flags & 0x4000_0000 != 0 {
        e.call(CELL_RESET_DETACH_TIME, &args![this, 0u32, 0u32]);
    }
}

// Translated from 005555d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `008d0600(this, flags, extra)` (the map names that folded body
/// `ProcessLists::PrintLists`); for flag 8, when the cell has an owner
/// (`TESObjectCELL::GetOwner`), sets the owner to the form `004839c0` finds
/// for it.
pub fn fn_005555d0(e: &mut Engine, this: Ptr<TESObjectCELL>, flags: u32, extra: u32) {
    e.call(CELL_CALL_WITH_TWO_WORDS, &args![this, flags, extra]);
    if flags & 8 != 0 {
        let owner = e.call(CELL_OWNER, &args![this]).u32();
        if owner != 0 {
            let form = e.call(FORM_LOOK_UP, &args![owner]).u32();
            e.call(CELL_SET_OWNER, &args![this, form]);
        }
    }
}

// Translated from 00555630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the cell to a save-game buffer object (`buffer`): the base class
/// part (`00484d60`), then for each flag the buffer holds: 2 the packed flags
/// byte, `0x80000000` the seen data (virtual `+0x0c(buffer)`), 4 the name
/// (`00865e70`), 8 the owner's form id (`00865df0`).
pub fn fn_00555630(e: &mut Engine, this: Ptr<TESObjectCELL>, buffer: Ptr) {
    e.call(FORM_SAVE_TO_BUFFER, &args![this, buffer]);
    if buffer_has_flag(e, buffer, BUFFER_FLAGS_GET, 2) {
        let packed = packed_cell_flags(e, this);
        e.with_stack(4, |e, local| {
            e.mem.set_u8(local.addr(), packed);
            e.call(BUFFER_SAVE_BYTES, &args![buffer, local, 1u32, 0u32]);
        });
    }
    if buffer_has_flag(e, buffer, BUFFER_FLAGS_GET, 0x8000_0000) {
        let seen = e.call(CELL_SEEN_DATA, &args![this]).u32();
        e.vcall(seen, SEEN_DATA_SAVE_BUFFER_SLOT, &args![buffer]);
    }
    if buffer_has_flag(e, buffer, BUFFER_FLAGS_GET, 4) {
        let name = e.call(TEXT_GET_STRING, &args![this.byte_add(0x18)]).u32();
        e.call(BUFFER_SAVE_STRING, &args![buffer, name, 0u32]);
    }
    if buffer_has_flag(e, buffer, BUFFER_FLAGS_GET, 8) {
        let owner = e.call(CELL_OWNER, &args![this]).u32();
        e.call(BUFFER_SAVE_FORM_ID, &args![buffer, owner, 0u32]);
    }
}

// Translated from 00555740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the cell from a save-game buffer object: the base class part
/// (`00484da0`), then for each flag the buffer holds: 2 the packed flags byte
/// (`00864980`), `0x80000000` the seen data (created when missing, virtual
/// `+0x14(buffer)`), 4 the name (`008649a0`, then `TESSoundFile::SetSoundFile`
/// of `this + 0x18`), 8 the owner's form id (`008648e0`). The exception frame
/// is not translated.
pub fn fn_00555740(e: &mut Engine, this: Ptr<TESObjectCELL>, buffer: Ptr) {
    e.call(FORM_LOAD_FROM_BUFFER, &args![this, buffer]);
    if buffer_has_flag(e, buffer, BUFFER_FLAGS_GET, 2) {
        let packed = e.with_stack(4, |e, local| {
            e.mem.set_u8(local.addr(), 0);
            e.call(BUFFER_LOAD_BYTES, &args![buffer, local, 1u32]);
            e.mem.u8(local.addr())
        });
        unpack_cell_flags(e, this, packed);
    }
    if buffer_has_flag(e, buffer, BUFFER_FLAGS_GET, 0x8000_0000) {
        let seen = seen_data_or_create(e, this);
        e.vcall(seen, SEEN_DATA_LOAD_BUFFER_SLOT, &args![buffer]);
    }
    if buffer_has_flag(e, buffer, BUFFER_FLAGS_GET, 4) {
        e.with_stack(0x10c, |e, name| {
            e.call(BUFFER_LOAD_STRING, &args![buffer, name]);
            e.call(FULL_NAME_SET, &args![this.byte_add(0x18), name]);
        });
    }
    if buffer_has_flag(e, buffer, BUFFER_FLAGS_GET, 8) {
        let id = e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), 0);
            e.call(BUFFER_LOAD_FORM_ID, &args![buffer, local]);
            e.mem.u32(local.addr())
        });
        e.call(CELL_SET_OWNER, &args![this, id]);
    }
}

// Translated from 005559d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `004534f0(this, buffer)`; for the buffer's flag 8, when the cell has an
/// owner, sets the owner to the form `004839c0` finds for it.
pub fn fn_005559d0(e: &mut Engine, this: Ptr<TESObjectCELL>, buffer: Ptr) {
    e.call(CELL_EMPTY_CALL, &args![this, buffer]);
    if buffer_has_flag(e, buffer, BUFFER_FLAGS_GET, 8) {
        let owner = e.call(CELL_OWNER, &args![this]).u32();
        if owner != 0 {
            let form = e.call(FORM_LOOK_UP, &args![owner]).u32();
            e.call(CELL_SET_OWNER, &args![this, form]);
        }
    }
}

// Translated from 00555a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `004534f0(this, buffer)`, clears `cCellGameFlags`, and for the
/// buffer's flags (read with `0042ce30`) `0x80000000` calls `00555be0`, for
/// `0x40000000` calls `00546b10(this, 0, 0)`.
pub fn fn_00555a50(e: &mut Engine, this: Ptr<TESObjectCELL>, buffer: Ptr) {
    e.call(CELL_EMPTY_CALL, &args![this, buffer]);
    e.set(this, TESObjectCELL::cCellGameFlags, 0);
    if buffer_has_flag(e, buffer, BUFFER_FLAGS_GET_OTHER, 0x8000_0000) {
        e.call(CELL_CLEAR_SEEN_DATA, &args![this]);
    }
    if buffer_has_flag(e, buffer, BUFFER_FLAGS_GET_OTHER, 0x4000_0000) {
        e.call(CELL_RESET_DETACH_TIME, &args![this, 0u32, 0u32]);
    }
}

// Translated from 00555ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cell's north rotation: for an interior cell, `00421a40` of the extra
/// data list (`ST0`, stored as a `float`); `0.0` for an exterior.
pub fn fn_00555ad0(e: &mut Engine, this: Ptr<TESObjectCELL>) -> f32 {
    if e.call(CELL_IS_INTERIOR, &args![this]).bool() {
        let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).u32();
        e.call(EXTRA_LIST_GET_ROTATION, &args![list]).f32()
    } else {
        0.0
    }
}

// Translated from 00555b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::AdjustCoordForNorthRotation` (Xbox PDB) `(this, source,
/// destination, reverse)`: the angle is the cell's north rotation times 1.0
/// (or the `float` at `01012054` when `reverse` is non-zero). When it equals
/// the `double` at `01012060` the three floats of `source` are copied to
/// `destination`; otherwise a rotation matrix (`011a9448` copied to the
/// stack, `004a0c90(matrix, angle)`) rotates `source` (`004b3ae0`) and the
/// result is stored in `destination`.
pub fn tes_object_cell_adjust_coord_for_north_rotation(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    source: Ptr,
    destination: Ptr,
    reverse: u8,
) {
    let sign: f32 = if reverse != 0 {
        e.global(MINUS_ONE_FLOAT)
    } else {
        1.0
    };
    let rotation = fn_00555ad0(e, this);
    let angle = (rotation as f64 * sign as f64) as f32;
    let epsilon: f64 = e.global(ROTATION_EPSILON);
    if angle as f64 == epsilon {
        for index in 0..3u32 {
            let value = e.mem.u32(source.addr() + 4 * index);
            e.mem.set_u32(destination.addr() + 4 * index, value);
        }
    } else {
        e.with_stack(0x24 + 0x0c, |e, block| {
            let matrix = block;
            let rotated = block.byte_add(0x24);
            for index in 0..9u32 {
                let word = e.mem.u32(IDENTITY_ROTATION + 4 * index);
                e.mem.set_u32(matrix.addr() + 4 * index, word);
            }
            e.call(ROTATION_MATRIX_BUILD, &args![matrix, angle]);
            let result = e.call(ROTATE_VECTOR, &args![rotated, source, matrix]).u32();
            for index in 0..3u32 {
                let value = e.mem.u32(result + 4 * index);
                e.mem.set_u32(destination.addr() + 4 * index, value);
            }
        });
    }
}

// Translated from 00556870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::GetSeenValue` (Xbox PDB), cdecl `(position)`: counts, up
/// to 4, how many of the four corners of the 16-by-16 seen-data tile around
/// `position` are seen, for the cell the player object (`011dea3c`) is in
/// (0 without one). An interior cell asks its own sections
/// (`GetIntSeenSection(x, y, 0)`, the coordinates shifted by `0x800`);
/// an exterior one asks the world space's cells (`GetCellFromCellCoord`),
/// where a cell counts as seen if its seen data has the bit or its flag
/// `00556850` is set. A corner at the end of a tile (index 15) reads the next
/// tile.
pub fn tes_object_cell_get_seen_value(e: &mut Engine, position: Ptr) -> i32 {
    let player: u32 = e.global(PLAYER_OBJECT);
    let cell = e
        .call(REFERENCE_PARENT_CELL, &args![player])
        .ptr::<TESObjectCELL>();
    if cell.is_null() {
        return 0;
    }
    let mut count = 0;
    if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
        seen_value_interior(e, cell, position, &mut count);
    } else {
        let world = e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32();
        if world != 0 {
            seen_value_exterior(e, world, position, &mut count);
        }
    }
    count
}

/// Helper: `float` coordinate `index` of `position` as the game's integer
/// (`ftol`, `00406d90`).
fn position_as_integer(e: &mut Engine, position: Ptr, index: u32) -> i32 {
    let value = e.mem.f32(position.addr() + 4 * index);
    e.call(FLOAT_TO_INT, &args![value]).i32()
}

/// Helper: the interior half of [`tes_object_cell_get_seen_value`].
fn seen_value_interior(e: &mut Engine, cell: Ptr<TESObjectCELL>, position: Ptr, count: &mut i32) {
    let section_x = (position_as_integer(e, position, 0) - 0x800) >> 12;
    let section_y = (position_as_integer(e, position, 1) - 0x800) >> 12;
    let (bit_x, bit_y) = seen_bit_index(e, position, 1);
    let is_seen = |e: &mut Engine, section: u32, x: u32, y: u32| {
        section != 0 && e.call(SEEN_BIT_IS_SET, &args![section, x, y]).bool()
    };
    let first = interior_section(e, cell, section_x, section_y);
    if is_seen(e, first, bit_x, bit_y) {
        *count += 1;
    }
    let mut below = 0;
    let mut at_bottom = false;
    if bit_y == 0xf {
        at_bottom = true;
        below = interior_section(e, cell, section_x, section_y + 1);
        if is_seen(e, below, bit_x, 0) {
            *count += 1;
        }
    } else if is_seen(e, first, bit_x, bit_y + 1) {
        *count += 1;
    }
    let mut right = 0;
    let mut at_right = false;
    if bit_x == 0xf {
        at_right = true;
        right = interior_section(e, cell, section_x + 1, section_y);
        if is_seen(e, right, 0, bit_y) {
            *count += 1;
        }
    } else if is_seen(e, first, bit_x + 1, bit_y) {
        *count += 1;
    }
    if at_bottom && at_right {
        let corner = interior_section(e, cell, section_x + 1, section_y + 1);
        if is_seen(e, corner, 0, 0) {
            *count += 1;
        }
    } else if at_bottom {
        if is_seen(e, below, bit_x + 1, 0) {
            *count += 1;
        }
    } else if at_right {
        if is_seen(e, right, 0, bit_y + 1) {
            *count += 1;
        }
    } else if is_seen(e, first, bit_x + 1, bit_y + 1) {
        *count += 1;
    }
}

/// Helper: `00879da0(position, interior, &x, &y)` (cdecl): the bit
/// coordinates of `position` inside its seen-data tile.
fn seen_bit_index(e: &mut Engine, position: Ptr, interior: u32) -> (u32, u32) {
    e.with_stack(8, |e, out| {
        e.mem.set_u32(out.addr(), 0);
        e.mem.set_u32(out.addr() + 4, 0);
        e.call(
            SEEN_DATA_INDEX,
            &args![position, interior, out, out.byte_add(4)],
        );
        (e.mem.u32(out.addr()), e.mem.u32(out.addr() + 4))
    })
}

/// Helper: `GetIntSeenSection(x, y, 0)` of an interior cell.
fn interior_section(e: &mut Engine, cell: Ptr<TESObjectCELL>, x: i32, y: i32) -> u32 {
    tes_object_cell_get_int_seen_section(e, cell, x, y, 0).addr()
}

/// Helper: the exterior half of [`tes_object_cell_get_seen_value`].
fn seen_value_exterior(e: &mut Engine, world: u32, position: Ptr, count: &mut i32) {
    let cell_x = position_as_integer(e, position, 0) >> 12;
    let cell_y = position_as_integer(e, position, 1) >> 12;
    let (bit_x, bit_y) = seen_bit_index(e, position, 0);
    // The cell at the cell coordinates and its seen data (0 for no cell).
    let cell_at = |e: &mut Engine, x: i32, y: i32| -> (u32, u32) {
        let cell = e.call(WORLD_SPACE_GET_CELL, &args![world, x, y]).u32();
        let seen = if cell == 0 {
            0
        } else {
            e.call(CELL_SEEN_DATA, &args![cell]).u32()
        };
        (cell, seen)
    };
    let counts = |e: &mut Engine, cell: u32, seen: u32, x: u32, y: u32| {
        (seen != 0 && e.call(SEEN_BIT_IS_SET, &args![seen, x, y]).bool())
            || (cell != 0 && e.call(CELL_SEEN_FLAG, &args![cell]).bool())
    };
    let (first_cell, first_seen) = cell_at(e, cell_x, cell_y);
    if counts(e, first_cell, first_seen, bit_x, bit_y) {
        *count += 1;
    }
    let (mut below_cell, mut below_seen) = (0, 0);
    let mut at_bottom = false;
    if bit_y == 0xf {
        at_bottom = true;
        (below_cell, below_seen) = cell_at(e, cell_x, cell_y + 1);
        if counts(e, below_cell, below_seen, bit_x, 0) {
            *count += 1;
        }
    } else if counts(e, first_cell, first_seen, bit_x, bit_y + 1) {
        *count += 1;
    }
    let (mut right_cell, mut right_seen) = (0, 0);
    let mut at_right = false;
    if bit_x == 0xf {
        at_right = true;
        (right_cell, right_seen) = cell_at(e, cell_x + 1, cell_y);
        if counts(e, right_cell, right_seen, 0, bit_y) {
            *count += 1;
        }
    } else if counts(e, first_cell, first_seen, bit_x + 1, bit_y) {
        *count += 1;
    }
    if at_bottom && at_right {
        let (corner_cell, corner_seen) = cell_at(e, cell_x + 1, cell_y + 1);
        if counts(e, corner_cell, corner_seen, 0, 0) {
            *count += 1;
        }
    } else if at_bottom {
        if counts(e, below_cell, below_seen, bit_x + 1, 0) {
            *count += 1;
        }
    } else if at_right {
        if counts(e, right_cell, right_seen, 0, bit_y + 1) {
            *count += 1;
        }
    } else if counts(e, first_cell, first_seen, bit_x + 1, bit_y + 1) {
        *count += 1;
    }
}

// Translated from 00556ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::GetIntSeenSection` (Xbox PDB) `(this, x, y, create)`: for
/// an interior cell, finds in the chain of seen-data sections of the cell
/// (the cell's seen data cast to the section type, next section at `+0x28`)
/// the one whose bytes at `+0x24` and `+0x25` equal `x` and `y` (as signed
/// bytes). If none and `create` is non-zero, appends a new section
/// (`0x2c` bytes, constructor `0087a100(x, y)`), or, when the cell had no
/// seen data at all, stores it as the cell's seen data and calls the cell's
/// virtual `+0x48(0x80000000)`. Returns the section or null. The exception
/// frame is not translated.
pub fn tes_object_cell_get_int_seen_section(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    x: i32,
    y: i32,
    create: u8,
) -> Ptr {
    if !e.call(CELL_IS_INTERIOR, &args![this]).bool() {
        return Ptr::NULL;
    }
    let seen = e.call(CELL_SEEN_DATA, &args![this]).u32();
    let mut section = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                seen,
                0u32,
                SEEN_DATA_CAST_SOURCE,
                SEEN_DATA_CAST_TARGET,
                0u32
            ],
        )
        .u32();
    let mut previous = 0u32;
    while section != 0 {
        let section_x = e.mem.i8(section + 0x24) as i32;
        let section_y = e.mem.i8(section + 0x25) as i32;
        if section_x == x && section_y == y {
            break;
        }
        previous = section;
        section = e.mem.u32(section + 0x28);
    }
    if section == 0 && create != 0 {
        let block = e.call(OPERATOR_NEW, &args![0x2cu32]).u32();
        let created = if block == 0 {
            0
        } else {
            e.call(
                INTERIOR_SEEN_DATA_CONSTRUCT,
                &args![block, x as u8 as u32, y as u8 as u32],
            )
            .u32()
        };
        if previous != 0 {
            e.mem.set_u32(previous + 0x28, created);
            section = e.mem.u32(previous + 0x28);
        } else {
            section = created;
            let list = e.call(CELL_EXTRA_DATA_LIST, &args![this]).u32();
            e.call(EXTRA_LIST_SET_SEEN_DATA, &args![list, section]);
            e.vcall(this.addr(), CELL_CHANGED_SLOT, &args![0x8000_0000u32]);
        }
    }
    Ptr::new(section)
}

// Translated from 00557090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_005570b0` on the object at `this + 0x80`.
pub fn fn_00557090(e: &mut Engine, this: Ptr) -> bool {
    fn_005570b0(e, this.byte_add(0x80))
}

// Translated from 005570b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the word at `this` is not zero.
pub fn fn_005570b0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr()) != 0
}

// Translated from 005570d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the cell lock, for every reference of the cell that is not null,
/// fails the test `005651e0` and whose base object (`007af430`) has form
/// type `0x25`: when its virtual `+0x1d0` gives an object, calls that
/// object's virtual `+0x100` and then the virtual `+8(object)` of the result.
/// The map has no name for it.
pub fn fn_005570d0(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    for_each_cell_reference_locked(e, this, |e, reference| {
        if reference.is_null() || e.call(REFERENCE_TEST, &args![reference]).bool() {
            return;
        }
        let base = e.call(REFERENCE_BASE_OBJECT, &args![reference]).u32();
        if e.call(FORM_TYPE, &args![base]).u32() != REFERENCE_FORM_TYPE {
            return;
        }
        if e.vcall(reference.addr(), REFERENCE_OWNER_SLOT, &[]).u32() != 0 {
            let owner = e.vcall(reference.addr(), REFERENCE_OWNER_SLOT, &[]).u32();
            let target = e.vcall(owner, OWNER_QUERY_SLOT, &[]).u32();
            e.vcall(target, OWNER_CALL_SLOT, &args![owner]);
        }
    });
}

// Translated from 005571a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::QueueReferences` (Xbox PDB) `(this, check)`: under the cell
/// lock, with the cell's priority (`TES::GetCellPriority(cell, 0)` on the
/// scene), goes through the references twice: first those whose base object
/// is one of the two marker forms (`011ca238`, `011ca230`), then all the
/// others. A reference with no `+0x1d0` object or one that passes `005651e0`
/// is queued in the model loader `(reference, priority, 1)` unless it is
/// flagged by `00440da0` or `00440d80`, or `check` is non-zero and
/// `0056b250` fails; any other reference that is flagged calls its virtual
/// `+0x1cc(0, first)`, with `first` 1 in the first pass and 0 in the second.
pub fn tes_object_cell_queue_references(e: &mut Engine, this: Ptr<TESObjectCELL>, check: u8) {
    e.call(CELL_LOCK_ENTER, &args![this]);
    let scene: u32 = e.global(SCENE_SINGLETON);
    let priority = e.call(CELL_PRIORITY, &args![scene, this, 0u32]).u32();
    for first_pass in [true, false] {
        let first = e.call(CELL_REFERENCES, &args![this]).u32();
        for_each_list_item(e, first, |e, reference| {
            if reference.is_null() {
                return;
            }
            let is_marker = |e: &mut Engine, reference: Ptr| {
                let base = e.call(REFERENCE_BASE_OBJECT, &args![reference]).u32();
                if base == e.global::<u32>(MARKER_FORM_SECOND) {
                    return true;
                }
                let base = e.call(REFERENCE_BASE_OBJECT, &args![reference]).u32();
                base == e.global::<u32>(MARKER_FORM_FIRST)
            };
            if is_marker(e, reference) != first_pass {
                return;
            }
            let has_owner = e.vcall(reference.addr(), REFERENCE_OWNER_SLOT, &[]).u32() != 0;
            if !has_owner || e.call(REFERENCE_TEST, &args![reference]).bool() {
                if e.call(REFERENCE_FLAG_TEST_A, &args![reference]).bool()
                    || e.call(REFERENCE_FLAG_TEST_B, &args![reference]).bool()
                {
                    return;
                }
                if check != 0 && !e.call(REFERENCE_FLAG_TEST_C, &args![reference]).bool() {
                    return;
                }
                let loader: u32 = e.global(MODEL_LOADER_SINGLETON);
                e.call(
                    MODEL_LOADER_QUEUE_REFERENCE,
                    &args![loader, reference, priority, 1u32],
                );
            } else if e.call(REFERENCE_FLAG_TEST_A, &args![reference]).bool()
                || e.call(REFERENCE_FLAG_TEST_B, &args![reference]).bool()
            {
                e.vcall(
                    reference.addr(),
                    REFERENCE_DEFER_SLOT,
                    &args![0u32, u32::from(first_pass)],
                );
            }
        });
    }
    e.call(CELL_LOCK_LEAVE, &args![this]);
}

// Translated from 005573e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the cell lock, for every non-null reference whose model
/// (`0043fcd0`) is non-null and answers true to its virtual `+0x10`: calls
/// `00476ab0` on the model and sets its fade alpha to `1.0` (`BSShaderUtil::
/// RecursiveSetPropertyFadeAlpha`). The map has no name for it.
pub fn fn_005573e0(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    for_each_cell_reference_locked(e, this, |e, reference| {
        if reference.is_null() {
            return;
        }
        let model = e.call(REFERENCE_MODEL, &args![reference]).u32();
        if model != 0 && e.vcall(model, MODEL_TEST_SLOT, &[]).u32() != 0 {
            e.call(MODEL_PREPARE, &args![model]);
            e.call(SET_PROPERTY_FADE_ALPHA, &args![model, 1.0f32]);
        }
    });
}

// Translated from 00557470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the cell lock, calls `00526f20(reference)` on the object at
/// `011c95c8` for every non-null reference of the cell.
pub fn fn_00557470(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    for_each_cell_reference_locked(e, this, |e, reference| {
        if !reference.is_null() {
            let collection: u32 = e.global(LOADED_REFERENCES);
            e.call(LOADED_REFERENCES_REMOVE, &args![collection, reference]);
        }
    });
}

// Translated from 005574d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the cell lock, for every non-null reference: `005270b0(reference)`
/// on the object at `011c95c8`; then, when the scene has no interior cell
/// (`005f36f0` answers 0), has a world space (`004fd3e0`) and the
/// reference's base object passes `00549580`, and the reference has a world
/// space (`TESObjectREFR::GetWorldSpace`): `006fd010` on that world's
/// terrain manager with the reference, or `BGSTerrainManager::HideTree
/// (reference, 1)` when `00440da0` says the reference is set; then
/// `00929260(1)` on the terrain manager. The map has no name for it.
pub fn fn_005574d0(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    for_each_cell_reference_locked(e, this, |e, reference| {
        if reference.is_null() {
            return;
        }
        let collection: u32 = e.global(LOADED_REFERENCES);
        e.call(LOADED_REFERENCES_ADD, &args![collection, reference]);
        let scene: u32 = e.global(SCENE_SINGLETON);
        if e.call(SCENE_CELL, &args![scene]).u32() != 0 {
            return;
        }
        if e.call(SCENE_WORLD_SPACE, &args![scene]).u32() == 0 {
            return;
        }
        let base = e.call(REFERENCE_BASE_OBJECT, &args![reference]).u32();
        if !e.call(BASE_FORM_FLAG_40, &args![base]).bool() {
            return;
        }
        let world = e.call(REFERENCE_WORLD_SPACE, &args![reference]).u32();
        if world == 0 {
            return;
        }
        if e.call(REFERENCE_FLAG_TEST_A, &args![reference]).bool() {
            let terrain = e.call(WORLD_GET_TERRAIN_MANAGER, &args![world]).u32();
            e.call(TERRAIN_HIDE_TREE, &args![terrain, reference, 1u32]);
        } else {
            let terrain = e.call(WORLD_GET_TERRAIN_MANAGER, &args![world]).u32();
            e.call(TERRAIN_UPDATE_TREE, &args![terrain, reference]);
        }
        let terrain = e.call(WORLD_GET_TERRAIN_MANAGER, &args![world]).u32();
        e.call(TERRAIN_SET_FLAG, &args![terrain, 1u32]);
    });
}

// ---------------------------------------------------------------------------
// Third batch: the callees, constants and helpers of `005575d0` to `00558df0`.

/// Virtual slot `+0x15c` of a reference: true when the reference has a
/// navmesh obstacle (`fn_005575d0` and `fn_005576c0` ask it).
const REFERENCE_OBSTACLE_SLOT: u32 = 0x15c;
/// `NavMeshObstacleManager::AddObstacleForReference` (engine map), thiscall on the
/// manager `OBSTACLE_MANAGER_GET` returns (it takes no word), RET 4: the reference (pushed
/// before the getter, 00557627 and the other sites).
const OBSTACLE_ADD: u32 = 0x006c_0c30;
/// `NavMeshObstacleManager::RemoveObstacleForReference` (engine map), same
/// calling form.
const OBSTACLE_REMOVE: u32 = 0x006c_0c80;
/// `NavMeshObstacleManager::OnDoorClose` (engine map), same calling form.
const OBSTACLE_DOOR_CLOSED: u32 = 0x006c_0f10;
/// Removal call (`006c1060`) `fn_005576c0` makes for door references.
const OBSTACLE_DOOR_REMOVE: u32 = 0x006c_1060;
/// `BGSOpenCloseForm::GetOpenState` (Xbox PDB), cdecl `(reference)`; `1`
/// is the state `fn_005575d0` treats as open.
const GET_OPEN_STATE: u32 = 0x0047_b250;
/// `TESObjectDOOR::IsSlidingDoor` (Xbox PDB), thiscall on the base form.
const IS_SLIDING_DOOR: u32 = 0x0051_8080;
/// The form type (`00401170`) of the base objects the obstacle walks treat
/// as doors (the type `IsSlidingDoor` is asked about).
const DOOR_FORM_TYPE: u32 = 0x1c;
/// `00541a30(object, 1)`: what `fn_00557760` calls on the object it
/// replaces.
const RELEASE_HELD: u32 = 0x0054_1a30;
/// `00578060` / `00578170`: `TESObjectREFR::AddMasterParticleAddonNodes` and
/// `RemoveMasterParticleAddonNodes` (Xbox PDB), cdecl `(node)`.
const ADD_ADDON_NODES: u32 = 0x0057_8060;
const REMOVE_ADDON_NODES: u32 = 0x0057_8170;
/// `004523e0(reference, 2)`: the reference test `ProcessAddonNodesForRefs`
/// makes before it asks for the reference's 3D (virtual `+0x1d0`).
const REFERENCE_ADDON_TEST: u32 = 0x0045_23e0;
const REFERENCE_NODE_SLOT: u32 = 0x1d0;
/// `fn_005578f0`: the cell's node holder (`CELL_NODE_D`), the holder of
/// an entry's node (`00413f40`), its name (`0043b1b0`), the name compare
/// (`00408b20`, cdecl, 0 when equal), the string it compares with and the
/// virtual slot called on a match.
const ENTRY_HOLDER: u32 = 0x0041_3f40;
const HOLDER_NAME: u32 = 0x0043_b1b0;
const STRING_COMPARE: u32 = 0x0040_8b20;
const CELL_LOCATION_MARKER_NAME: u32 = 0x0102_f154;
const MARKER_ENTRY_SLOT: u32 = 0xf0;
/// `fn_00557990`: the node `00456fc0(cell, 7)` gives, the form type it skips
/// (`0x1e`), the form (`011ca234`) it skips and the call `00450f90(node,
/// flag)` it makes on the reference's 3D.
const SKIPPED_FORM_TYPE: u32 = 0x1e;
const SKIPPED_FORM: u32 = 0x011c_a234;
const NODE_SET_FLAG: u32 = 0x0045_0f90;
/// The cell fade-in value `fn_00557ae0` and `fn_00557c40` store (`float`).
const LOD_FADE_VALUE: u32 = 0x0118_b694;
/// `fn_00557ae0`: the model's type test `007058c0`, the type it accepts
/// (6), the model's second test `0099e040` (a `float` in `ST0`) and the
/// `double` it must exceed (`599.9000244140625`).
const MODEL_TYPE: u32 = 0x0070_58c0;
const MODEL_TYPE_ACCEPTED: u32 = 6;
const MODEL_DISTANCE: u32 = 0x0099_e040;
const MODEL_DISTANCE_LIMIT: u32 = 0x0102_f168;
/// `fn_00557c40`: the call `0054b800(model)` made for models of type 6.
const MODEL_DETACH_STEP: u32 = 0x0054_b800;
/// `fn_00557be0`: the scene's cell count (`00453980`) and cell at an index
/// (`00459470`).
const SCENE_CELL_COUNT: u32 = 0x0045_3980;
const SCENE_CELL_AT: u32 = 0x0045_9470;
/// The limits of `RenderTestCell` (`fn_00557d50`): geometry, triangles,
/// passes, lights, time, and a last count.
const RENDER_LIMITS: [u32; 6] = [0x4b0, 800_000, 100, 0xf, 0xffff_ffff, 8];
/// The report callback `fn_00557da0` installs (`fn_005586c0`).
const RENDER_FAILURE_CALLBACK: u32 = 0x0055_86c0;
/// `fn_00558200`, the navmesh triangle accessors: the count of the
/// triangle array (`0044ddc0`), the `NiPoint3` of a vertex (`0068f0a0(navmesh,
/// vertex)`), the in-place vector add (`0063c8a0(sum, other)`) and the
/// scale by a `float` (`00439180(sum, factor)`) and the factor's address.
const TRIANGLE_COUNT: u32 = 0x0044_ddc0;
const NAV_MESH_VERTEX: u32 = 0x0068_f0a0;
const VECTOR_ADD_ASSIGN: u32 = 0x0063_c8a0;
const VECTOR_SCALE: u32 = 0x0043_9180;
const ONE_THIRD: u32 = 0x0102_f200;
/// `RenderTestCell`: the render handle (`0045c670`), a node's local
/// translation (`0043c490`) and rotation (`006a9540`) and their setters
/// (`00440460`, `0043fa80`), the cell's navmesh array (`0070ec90`), the
/// count of that array (`00620b80`), `NavMeshArray::GetNavMeshByIndex`
/// `(array, out slot, index)`, the slot's test (`00458b50`) and release
/// (`0042fa40`), the count of a reference list (`005ae380`), the debug
/// print (`005b5e40`, cdecl, a format and its arguments), and the strings
/// and constants they use.
const RENDER_HANDLE: u32 = 0x0045_c670;
const NODE_LOCAL_TRANSLATION: u32 = 0x0043_c490;
const NODE_LOCAL_ROTATION: u32 = 0x006a_9540;
const NODE_SET_TRANSLATION: u32 = 0x0044_0460;
const NODE_SET_ROTATION: u32 = 0x0043_fa80;
const CELL_NAV_MESH_ARRAY: u32 = 0x0070_ec90;
const NAV_MESH_ARRAY_COUNT: u32 = 0x0062_0b80;
const NAV_MESH_BY_INDEX: u32 = 0x0046_4f60;
const NAV_MESH_SLOT_TEST: u32 = 0x0045_8b50;
const NAV_MESH_SLOT_RELEASE: u32 = 0x0042_fa40;
const LIST_COUNT: u32 = 0x005a_e380;
const DEBUG_PRINT_LINE: u32 = 0x005b_5e40;
const TESTING_TRIANGLES_FORMAT: u32 = 0x0102_f1e0;
const TESTING_POSITIONS_FORMAT: u32 = 0x0102_f1b4;
const RENDER_FAILED_MESSAGE: u32 = 0x0102_f170;
const TRIANGLE_LIFT: u32 = 0x0102_f1d8;
/// Virtual slots of a reference used by `RenderTestCell`: `+0x1f4` gives a
/// pointer to its position, `+0x1d0` its 3D; the world bound accessor of
/// the 3D (`0043d450`, engine map `NiAVObject::GetWorldBound`).
const REFERENCE_POSITION_SLOT: u32 = 0x1f4;
const WORLD_BOUND: u32 = 0x0043_d450;
/// The 3D of the player (`011dea3c`, virtual `+0x1d0`) and its position
/// accessor (`0045bb80`).
const NODE_POSITION_POINTER: u32 = 0x0045_bb80;
/// `fn_00558330`: `NiMatrix3::MakeXRotation` (Xbox PDB, `(matrix, angle)`),
/// the product `0043f8d0(this, out, other)` (engine map
/// `NiMatrix3::operator*`) and the degree factor `0.017453292` (`double`).
const MAKE_X_ROTATION: u32 = 0x0052_4ac0;
const MATRIX_PRODUCT: u32 = 0x0043_f8d0;
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// `fn_00558430`: the tick counter (`00457fe0`), the object `011dea0c` and
/// its call `0086ff70`, the statistics getter (`0043c4b0`, which returns a
/// global and leaves its stack words to `00558600`), the shadow scene node
/// of the table entry 0 (`00b5ac60`).
const TICK_COUNT: u32 = 0x0045_7fe0;
const RENDER_FRAME_OBJECT: u32 = 0x011d_ea0c;
const RENDER_FRAME_STEP: u32 = 0x0086_ff70;
const STATISTICS_HOLDER: u32 = 0x0043_c4b0;
const SHADOW_SCENE_VALUE: u32 = 0x00b5_ac60;
/// The eleven statistics words `fn_00558600` copies, in the order of its
/// parameters.
const STATISTICS_SOURCES: [u32; 11] = [
    0x011f_9fc8,
    0x011f_9fcc,
    0x011f_9fd8,
    0x011f_9fdc,
    0x011f_9fe0,
    0x011f_9fe4,
    0x011f_9fd0,
    0x011f_9fd4,
    0x011f_9fe8,
    0x011f_9ff0,
    0x011f_9fec,
];
/// `SaveRenderFailureData`: the last cell written (`011ca218`), the `RTF`
/// directory name (`0102f2f0`, for `00b00800`) and the path prefix
/// (`0102f2e8`, `".\RTF\"`), the file name formats, the header line, the
/// line format, the `double` that converts radians to degrees, the C
/// library `sprintf` (`00406d00`) and `strlen` (`STRING_LENGTH`), the file
/// deletion (`00aff0b0`), the `BSFile` constructor `(this, path, mode,
/// buffer size, 0)` (`00b00260`), the `ToEulerAnglesXYZ` of a matrix
/// (`00a592c0`) and the name pointer of a cell (`00401280`).
const LAST_RENDER_FAILURE_CELL: u32 = 0x011c_a218;
const RTF_DIRECTORY_NAME: u32 = 0x0102_f2f0;
const RTF_PREFIX: u32 = 0x0102_f2e8;
const CREATE_DIRECTORY: u32 = 0x00b0_0800;
const INTERIOR_FILE_FORMAT: u32 = 0x0102_f2d8;
const EXTERIOR_NAMED_FILE_FORMAT: u32 = 0x0102_f2bc;
const EXTERIOR_FILE_FORMAT: u32 = 0x0102_f2a4;
const HEADER_FORMAT: u32 = 0x0102_f250;
const LINE_FORMAT: u32 = 0x0102_f204;
const RADIANS_TO_DEGREES: u32 = 0x0102_f248;
const FORMAT_TEXT: u32 = 0x0040_6d00;
const DELETE_FILE: u32 = 0x00af_f0b0;
const FILE_CONSTRUCT: u32 = 0x00b0_0260;
const FILE_SIZE: u32 = 0x158;
const FILE_BUFFER_SIZE: u32 = 0x4000;
const FILE_MODE_APPEND: u32 = 2;
const FILE_MODE_CREATE: u32 = 1;
const FILE_SEEK_SLOT: u32 = 0x14;
const FILE_OPEN_SLOT: u32 = 0x20;
const FILE_WRITE_SLOT: u32 = 0x48;
const FILE_SEEK_ORIGIN: u32 = 0x010a_2480;
const MATRIX_TO_EULER: u32 = 0x00a5_92c0;
const CELL_NAME_POINTER: u32 = 0x0040_1280;
const NAME_SLOT: u32 = 0x130;
const PATH_BUFFER_SIZE: u32 = 0x104;
/// `fn_00558c90`: the call `00c6a270(3D, 1, 1, 1)` (engine map `bhkWorld::
/// Activate`).
const WORLD_ACTIVATE: u32 = 0x00c6_a270;
const OWNER_REFUSES_SLOT: u32 = 0x100;
/// `fn_00558ba0`: the slot of a reference holds a pointer at `+0x64` to a
/// word.
const HELD_POINTER_OFFSET: u32 = 0x64;
/// The constructors and the destructor of the three small classes at
/// `00558d40` to `00558df0`: the base constructors, the vtables they set and
/// the member destructor.
const SMALL_CLASS_BASE_A: u32 = 0x0055_8eb0;
const SMALL_CLASS_BASE_B: u32 = 0x0055_8fb0;
const SMALL_CLASS_BASE_C: u32 = 0x0055_90b0;
const SMALL_CLASS_VTABLE_A: u32 = 0x0102_f2f8;
const SMALL_CLASS_VTABLE_B: u32 = 0x0102_f318;
const SMALL_CLASS_VTABLE_C: u32 = 0x0102_f338;
const SMALL_CLASS_DESTRUCT: u32 = 0x0055_8f20;

/// Helper: like [`for_each_list_item`], but the game advances (`00726070`)
/// right after reading the item and before it uses it, so the next node is
/// asked for first.
fn for_each_list_item_advancing_first(
    e: &mut Engine,
    first: u32,
    mut visit: impl FnMut(&mut Engine, Ptr),
) {
    let mut node = first;
    while node != 0 {
        let item = node_item(e, Ptr::new(node));
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        visit(e, item);
    }
}

/// Helper: copies `count` words.
fn copy_words(e: &mut Engine, destination: u32, source: u32, count: u32) {
    for index in 0..count {
        let word = e.mem.u32(source + 4 * index);
        e.mem.set_u32(destination + 4 * index, word);
    }
}

// Translated from 005575d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the cell lock, for every non-null reference: when its virtual
/// `+0x15c` is true, `006c0c30` (`NavMeshObstacleManager::
/// AddObstacleForReference`, engine map) on the value `006c0720(reference)`
/// gives; for door references (form type `0x1c` of the base object) an open
/// door (`GetOpenState == 1`) that is not sliding is added the same way, any
/// other state calls `OnDoorClose` (`006c0f10`). The map has no name for it.
pub fn fn_005575d0(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    for_each_cell_reference_locked(e, this, |e, reference| {
        if reference.is_null() {
            return;
        }
        if e.vcall(reference.addr(), REFERENCE_OBSTACLE_SLOT, &[])
            .bool()
        {
            let obstacle = e.call(OBSTACLE_MANAGER_GET, &args![]).u32();
            e.call(OBSTACLE_ADD, &args![obstacle, reference]);
        }
        let base = e.call(REFERENCE_BASE_OBJECT, &args![reference]).u32();
        if e.call(FORM_TYPE, &args![base]).u32() != DOOR_FORM_TYPE {
            return;
        }
        if e.call(GET_OPEN_STATE, &args![reference]).i32() == 1 {
            let base = e.call(REFERENCE_BASE_OBJECT, &args![reference]).u32();
            if !e.call(IS_SLIDING_DOOR, &args![base]).bool() {
                let obstacle = e.call(OBSTACLE_MANAGER_GET, &args![]).u32();
                e.call(OBSTACLE_ADD, &args![obstacle, reference]);
            }
        } else {
            let obstacle = e.call(OBSTACLE_MANAGER_GET, &args![]).u32();
            e.call(OBSTACLE_DOOR_CLOSED, &args![obstacle, reference]);
        }
    });
}

// Translated from 005576c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The counterpart of `fn_005575d0`, under the cell lock, for every
/// non-null reference: when its virtual `+0x15c` is true,
/// `006c0c80` (`RemoveObstacleForReference`, engine map) on
/// `006c0720(reference)`; for door references `006c1060` on the same value.
/// The map has no name for it.
pub fn fn_005576c0(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    for_each_cell_reference_locked(e, this, |e, reference| {
        if reference.is_null() {
            return;
        }
        if e.vcall(reference.addr(), REFERENCE_OBSTACLE_SLOT, &[])
            .bool()
        {
            let obstacle = e.call(OBSTACLE_MANAGER_GET, &args![]).u32();
            e.call(OBSTACLE_REMOVE, &args![obstacle, reference]);
        }
        let base = e.call(REFERENCE_BASE_OBJECT, &args![reference]).u32();
        if e.call(FORM_TYPE, &args![base]).u32() == DOOR_FORM_TYPE {
            let obstacle = e.call(OBSTACLE_MANAGER_GET, &args![]).u32();
            e.call(OBSTACLE_DOOR_REMOVE, &args![obstacle, reference]);
        }
    });
}

// Translated from 00557760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the pointer at `+0x64`: when there is one, `00541a30(held, 1)`
/// is called on it first (the compiler emitted its null test twice). The
/// map has no name for it.
pub fn fn_00557760(e: &mut Engine, this: Ptr, value: Ptr) {
    let held = e.mem.u32(this.addr() + HELD_POINTER_OFFSET);
    if held != 0 {
        e.call(RELEASE_HELD, &args![held, 1u32]);
    }
    e.mem
        .set_u32(this.addr() + HELD_POINTER_OFFSET, value.addr());
}

// Translated from 005577b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::ProcessAddonNodesForRefs` (Xbox PDB): under the cell
/// lock, for every non-null reference that passes `004523e0(reference, 2)`
/// and has a 3D (virtual `+0x1d0`, asked twice): `AddMasterParticleAddonNodes`
/// (`00578060`) when `add` is non-zero, else `RemoveMasterParticleAddonNodes`
/// (`00578170`), on the 3D's virtual `+0x0c` value. The game advances the
/// list before it handles the item.
pub fn tes_object_cell_process_addon_nodes_for_refs(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
    add: u8,
) {
    e.call(CELL_LOCK_ENTER, &args![this]);
    let first = e.call(CELL_REFERENCES, &args![this]).u32();
    for_each_list_item_advancing_first(e, first, |e, reference| {
        if reference.is_null() || !e.call(REFERENCE_ADDON_TEST, &args![reference, 2u32]).bool() {
            return;
        }
        if e.vcall(reference.addr(), REFERENCE_NODE_SLOT, &[]).u32() == 0 {
            return;
        }
        let node = e.vcall(reference.addr(), REFERENCE_NODE_SLOT, &[]).u32();
        let value = e.vcall(node, 0x0c, &[]).u32();
        let function = if add != 0 {
            ADD_ADDON_NODES
        } else {
            REMOVE_ADDON_NODES
        };
        e.call(function, &args![value]);
    });
    e.call(CELL_LOCK_LEAVE, &args![this]);
}

// Translated from 005578e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method with three stack words that ignores them all and returns `0`.
/// The map has no name for it.
pub fn fn_005578e0(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> u32 {
    0
}

// Translated from 005578f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For every entry `i` of the node `00524cf0(this)` gives (if any), whose
/// holder (`00413f40`) has a name (`0043b1b0`) equal to `"CellLocationMarker"`
/// (`00408b20` answers 0): calls the node's virtual `+0xf0` with `i`. The
/// entry count is asked again at every step. The map has no name for it.
pub fn fn_005578f0(e: &mut Engine, this: Ptr) {
    let node = e.call(CELL_NODE_D, &args![this]).u32();
    if node == 0 {
        return;
    }
    let mut index = 0u32;
    while index < e.call(NODE_CHILD_COUNT, &args![node]).u32() {
        let entry = e.call(NODE_CHILD_AT, &args![node, index]).u32();
        if entry != 0 {
            let holder = e.call(ENTRY_HOLDER, &args![entry]).u32();
            let name = e.call(HOLDER_NAME, &args![holder]).u32();
            if name != 0
                && e.call(STRING_COMPARE, &args![name, CELL_LOCATION_MARKER_NAME])
                    .u32()
                    == 0
            {
                e.vcall(node, MARKER_ENTRY_SLOT, &args![index]);
            }
        }
        index += 1;
    }
}

// Translated from 00557990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the two-level node tree `00456fc0(this, 7)` gives (children of
/// children, the counts asked again at every step). Only when `mode == 4`:
/// for each leaf whose reference (`FindReferenceFor3D`, cdecl) exists and
/// whose base object's form type is not `0x1e` and is not the form at
/// `011ca234`, calls `00450f90(reference 3D, flag)` on the 3D (virtual
/// `+0x1d0`) of the reference. The map has no name for it.
pub fn fn_00557990(e: &mut Engine, this: Ptr, flag: u8, mode: u32) {
    let root = e.call(CELL_CHILD_NODE, &args![this, 7u32]).u32();
    if root == 0 {
        return;
    }
    let mut outer = 0u32;
    while outer < e.call(NODE_CHILD_COUNT, &args![root]).u32() {
        let group = e.call(NODE_CHILD_AT, &args![root, outer]).u32();
        if group != 0 {
            let mut inner = 0u32;
            while inner < e.call(NODE_CHILD_COUNT, &args![group]).u32() {
                let leaf = e.call(NODE_CHILD_AT, &args![group, inner]).u32();
                if leaf != 0 {
                    let reference = e.call(FIND_REFERENCE_FOR_3D, &args![leaf]).u32();
                    if reference != 0 && mode == 4 {
                        let base = e.call(REFERENCE_BASE_OBJECT, &args![reference]).u32();
                        let skipped: u32 = e.global(SKIPPED_FORM);
                        if e.call(FORM_TYPE, &args![base]).u32() != SKIPPED_FORM_TYPE
                            && base != skipped
                        {
                            let node = e.vcall(reference, REFERENCE_NODE_SLOT, &[]).u32();
                            e.call(NODE_SET_FLAG, &args![node, u32::from(flag)]);
                        }
                    }
                }
                inner += 1;
            }
        }
        outer += 1;
    }
}

// Translated from 00557aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts the fade to high detail unless it is running: when
/// `bFadingToHighDetail` is clear, sets it, clears `bFadingToLowDetail` and
/// sets `fLodFadeInPercent` to `0.0`. The map has no name for it.
pub fn fn_00557aa0(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    if !e.get(this, TESObjectCELL::bFadingToHighDetail) {
        e.set(this, TESObjectCELL::bFadingToHighDetail, true);
        e.set(this, TESObjectCELL::bFadingToLowDetail, false);
        e.set(this, TESObjectCELL::fLodFadeInPercent, 0.0);
    }
}

/// Helper: the reference tests `fn_00557ae0` and `fn_00557c40` share: the
/// reference is not the object at `011dea3c` and has a model (`0043fcd0`)
/// whose virtual `+0x10` gives a non-null node. Returns the node.
fn model_node_of_other_reference(e: &mut Engine, reference: Ptr) -> Option<u32> {
    if reference.is_null() {
        return None;
    }
    let player: u32 = e.global(DISTANCE_SOURCE_SINGLETON);
    if reference.addr() == player {
        return None;
    }
    let model = e.call(REFERENCE_MODEL, &args![reference]).u32();
    if model == 0 {
        return None;
    }
    let node = e.vcall(model, MODEL_TEST_SLOT, &[]).u32();
    (node != 0).then_some(node)
}

// Translated from 00557ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finishes the fade to high detail: under the cell lock, for every
/// reference other than the object at `011dea3c` that has a model node
/// (`0043fcd0`, virtual `+0x10`) of type 6 (`007058c0`) or with
/// `0099e040` above `599.9000244140625`: `00476ab0(node)` and fade alpha
/// `1.0`. Then `fLodFadeInPercent` is set from `0118b694`,
/// `bDisplayHighDetail` and `bUpdateTerrain` are set and both fading flags
/// are cleared. The map has no name for it.
pub fn fn_00557ae0(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    e.call(CELL_LOCK_ENTER, &args![this]);
    let first = e.call(CELL_REFERENCES, &args![this]).u32();
    for_each_list_item_advancing_first(e, first, |e, reference| {
        let Some(node) = model_node_of_other_reference(e, reference) else {
            return;
        };
        let accepted = if e.call(MODEL_TYPE, &args![node]).u32() == MODEL_TYPE_ACCEPTED {
            true
        } else {
            let value = e.call(MODEL_DISTANCE, &args![node]).f64();
            let limit: f64 = e.global(MODEL_DISTANCE_LIMIT);
            value > limit
        };
        if accepted {
            e.call(MODEL_PREPARE, &args![node]);
            e.call(SET_PROPERTY_FADE_ALPHA, &args![node, 1.0f32]);
        }
    });
    let fade: f32 = e.global(LOD_FADE_VALUE);
    e.set(this, TESObjectCELL::fLodFadeInPercent, fade);
    e.set(this, TESObjectCELL::bDisplayHighDetail, true);
    e.set(this, TESObjectCELL::bFadingToHighDetail, false);
    e.set(this, TESObjectCELL::bFadingToLowDetail, false);
    e.set(this, TESObjectCELL::bUpdateTerrain, true);
    e.call(CELL_LOCK_LEAVE, &args![this]);
}

// Translated from 00557be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `fn_00557ae0` on every non-null cell of the scene (`011dea10`;
/// count `00453980`, element `00459470`). `this` is not used. The map has
/// no name for it.
pub fn fn_00557be0(e: &mut Engine, _this: Ptr) {
    let scene: u32 = e.global(SCENE_SINGLETON);
    let mut index = 0u32;
    while index < e.call(SCENE_CELL_COUNT, &args![scene]).u32() {
        let cell = e.call(SCENE_CELL_AT, &args![scene, index]).u32();
        if cell != 0 {
            fn_00557ae0(e, Ptr::new(cell));
        }
        index += 1;
    }
}

// Translated from 00557c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Run before a cell is detached: keeps `bFadingToLowDetail` set when it
/// was, clears `bFadingToHighDetail` and `bDisplayHighDetail`, sets
/// `fLodFadeInPercent` from `0118b694`; then, under the cell lock, calls
/// `0054b800(node)` for every reference other than the object at `011dea3c`
/// whose model node (`0043fcd0`, virtual `+0x10`) has type 6. The map has
/// no name for it.
pub fn fn_00557c40(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    if e.get(this, TESObjectCELL::bFadingToLowDetail) {
        e.set(this, TESObjectCELL::bFadingToLowDetail, true);
    }
    e.set(this, TESObjectCELL::bFadingToHighDetail, false);
    let fade: f32 = e.global(LOD_FADE_VALUE);
    e.set(this, TESObjectCELL::fLodFadeInPercent, fade);
    e.set(this, TESObjectCELL::bDisplayHighDetail, false);
    e.call(CELL_LOCK_ENTER, &args![this]);
    let first = e.call(CELL_REFERENCES, &args![this]).u32();
    for_each_list_item_advancing_first(e, first, |e, reference| {
        let Some(node) = model_node_of_other_reference(e, reference) else {
            return;
        };
        if e.call(MODEL_TYPE, &args![node]).u32() == MODEL_TYPE_ACCEPTED {
            e.call(MODEL_DETACH_STEP, &args![node]);
        }
    });
    e.call(CELL_LOCK_LEAVE, &args![this]);
}

// Translated from 00557d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00450fd0(this) != 0` (the cell state getter). The map has no name for
/// it.
pub fn fn_00557d10(e: &mut Engine, this: Ptr<TESObjectCELL>) -> bool {
    e.call(CELL_GET_STATE, &args![this]).u32() != 0
}

// Translated from 00557d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::All3DWithVisibleDistantFadingIn` (Xbox PDB): the byte
/// `bDisplayHighDetail` at `+0xd0`.
pub fn tes_object_cell_all3d_with_visible_distant_fading_in(
    e: &mut Engine,
    this: Ptr<TESObjectCELL>,
) -> u8 {
    e.mem.u8(this.addr() + 0xd0)
}

// Translated from 00557d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the default limits of `RenderTestCell` in the first six words of
/// `this` (`0x4b0`, `800000`, `100`, `15`, `-1`, `8`) and returns `this`.
/// The map has no name for it.
pub fn fn_00557d50(e: &mut Engine, this: Ptr) -> Ptr {
    for (index, value) in RENDER_LIMITS.iter().enumerate() {
        e.mem.set_u32(this.addr() + 4 * index as u32, *value);
    }
    this
}

// Translated from 00557da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the `RenderTestCell` configuration: the default limits,
/// the report callback `fn_005586c0` at `+0x18` and `0` at `+0x1c`.
/// Returns `this`. The map has no name for it.
pub fn fn_00557da0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00557d50(e, this);
    e.mem.set_u32(this.addr() + 0x18, RENDER_FAILURE_CALLBACK);
    e.mem.set_u32(this.addr() + 0x1c, 0);
    this
}

// Translated from 00558200 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of triangles of a navmesh: `0044ddc0` on the array at
/// `this + 0x38`. The map has no name for it.
pub fn fn_00558200(e: &mut Engine, this: Ptr) -> u32 {
    e.call(TRIANGLE_COUNT, &args![this.addr() + 0x38]).u32()
}

// Translated from 00558220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::GetCenter` (Xbox PDB): writes the centroid of triangle
/// `triangle` into `out` and returns `out`. The three vertex positions
/// (`0068f0a0(navmesh, vertex index)`) are summed with `0063c8a0` starting
/// from a copy of the first, and the sum is scaled by the `float` at
/// `0102f200` with `00439180`.
pub fn nav_mesh_get_center(e: &mut Engine, this: Ptr, out: Ptr, triangle: u16) -> Ptr {
    let record = fn_005582f0(e, this, triangle);
    e.with_stack(12, |e, sum| {
        let vertex = fn_005582d0(e, record, 0);
        let position = e
            .call(NAV_MESH_VERTEX, &args![this, u32::from(vertex)])
            .u32();
        copy_words(e, sum.addr(), position, 3);
        for index in 1..3u32 {
            let vertex = fn_005582d0(e, record, index);
            let position = e
                .call(NAV_MESH_VERTEX, &args![this, u32::from(vertex)])
                .u32();
            e.call(VECTOR_ADD_ASSIGN, &args![sum, position]);
        }
        let third: f32 = e.global(ONE_THIRD);
        e.call(VECTOR_SCALE, &args![sum, third]);
        copy_words(e, out.addr(), sum.addr(), 3);
    });
    out
}

// Translated from 005582d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `u16` at `this + 2 * index`. The map has no name for it.
pub fn fn_005582d0(e: &mut Engine, this: Ptr, index: u32) -> u16 {
    e.mem.u16(this.addr().wrapping_add(index.wrapping_mul(2)))
}

// Translated from 005582f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-byte triangle record `triangle` of a navmesh (`fn_00558dd0` on the
/// array at `this + 0x38`). The map has no name for it.
pub fn fn_005582f0(e: &mut Engine, this: Ptr, triangle: u16) -> Ptr {
    fn_00558dd0(e, Ptr::new(this.addr() + 0x38), u32::from(triangle))
}

// Translated from 00558310 (decompiled, FalloutNV.exe 1.4.0.525)
/// The child `0` of a scene node: `0045bc00(this, 0)`. The map has no name
/// for it.
pub fn fn_00558310(e: &mut Engine, this: Ptr) -> u32 {
    e.call(SCENE_NODE_CHILD, &args![this, 0u32]).u32()
}

// Translated from 00558330 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(cell, position, config)`: renders the test cell from `position`
/// in 24 orientations: for the x rotations of 45, 0 and -45 degrees
/// (`MakeXRotation`) and the z rotations of 0, 45, ... 315 degrees
/// (`004a0c90`), multiplied (`0043f8d0`), `fn_00558430(cell, position,
/// matrix, config)`. Returns false when any of them failed.
pub fn fn_00558330(e: &mut Engine, cell: Ptr, position: Ptr, config: Ptr) -> bool {
    let mut passed = true;
    let mut x_degrees = 0x2di32;
    // Three 36-byte matrices: the x rotation, the z rotation, the product.
    e.with_stack(0x6c, |e, frame| {
        let x_rotation = frame;
        let z_rotation = frame.byte_add(0x24);
        let product = frame.byte_add(0x48);
        for _ in 0..3 {
            e.call(LIST_NODE_ITEM_ADDRESS, &args![x_rotation]);
            let factor: f64 = e.global(DEGREES_TO_RADIANS);
            let angle = (f64::from(x_degrees) * factor) as f32;
            e.call(MAKE_X_ROTATION, &args![x_rotation, angle]);
            x_degrees -= 0x2d;
            let mut z_degrees = 0i32;
            for _ in 0..8 {
                e.call(LIST_NODE_ITEM_ADDRESS, &args![z_rotation]);
                let factor: f64 = e.global(DEGREES_TO_RADIANS);
                let angle = (f64::from(z_degrees) * factor) as f32;
                e.call(ROTATION_MATRIX_BUILD, &args![z_rotation, angle]);
                z_degrees += 0x2d;
                e.call(MATRIX_PRODUCT, &args![z_rotation, product, x_rotation]);
                if !fn_00558430(e, cell, position, product, config) {
                    passed = false;
                }
            }
        }
    });
    passed
}

// Translated from 00558430 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(cell, position, rotation, config)`: places the render camera
/// (`0045c670`, child `0`) at `position` with `rotation`, updates it, renders
/// one frame (`0086ff70` on `011dea0c`, timed with `00457fe0`) and reads the
/// statistics: the report record starts with the default limits
/// (`fn_00558690`), then `fn_00558600` stores two statistics in its words 0
/// and 1 (`0043c4b0` hands it the eleven words it reads), word 3 is
/// `00b5ac60` of table entry 0 (low 16 bits) and word 4 the elapsed ticks.
/// A record exceeding one of the limits in `config` (words 0, 1, 2, 3, 5, 4,
/// compared unsigned in that order) is a failure: when `config + 0x18` holds
/// a callback it is called as `callback(&record, config[7])` with the record
/// completed by the cell, the position and the rotation. Returns true when
/// no limit was exceeded.
pub fn fn_00558430(e: &mut Engine, cell: Ptr, position: Ptr, rotation: Ptr, config: Ptr) -> bool {
    let handle = e.call(RENDER_HANDLE, &[]).u32();
    let node = fn_00558310(e, Ptr::new(handle));
    e.call(NODE_SET_TRANSLATION, &args![node, position]);
    let node = fn_00558310(e, Ptr::new(handle));
    e.call(NODE_SET_ROTATION, &args![node, rotation]);
    // The record is 0x4c bytes; the update record and the word the game's
    // exception state shared with the statistics pointers follow it.
    e.with_stack(0x4c + UPDATE_RECORD_SIZE + 4, |e, frame| {
        let record = frame;
        let update = frame.byte_add(0x4c);
        let shared_word = frame.byte_add(0x4c + UPDATE_RECORD_SIZE);
        e.call(UPDATE_RECORD_CONSTRUCT, &args![update, 0.0f32, 0u32, 0u32]);
        let node = fn_00558310(e, Ptr::new(handle));
        e.call(CONTROLLER_UPDATE, &args![node, update]);
        let started = e.call(TICK_COUNT, &[]).u32();
        let frame_object: u32 = e.global(RENDER_FRAME_OBJECT);
        e.call(RENDER_FRAME_STEP, &args![frame_object]);
        e.mem.set_u32(shared_word.addr(), 0);
        fn_00558690(e, record);
        let finished = e.call(TICK_COUNT, &[]).u32();
        e.mem
            .set_u32(record.addr() + 0x10, finished.wrapping_sub(started));
        // The eleven pointers: words 0 and 1 of the record, the rest the
        // shared word, as in the call.
        let holder = e.call(STATISTICS_HOLDER, &[]).u32();
        let source = fn_005585e0(e, Ptr::new(holder));
        let targets = [
            record.addr(),
            shared_word.addr(),
            record.addr() + 4,
            shared_word.addr(),
            shared_word.addr(),
            shared_word.addr(),
            shared_word.addr(),
            shared_word.addr(),
            shared_word.addr(),
            shared_word.addr(),
            shared_word.addr(),
        ];
        fn_00558600(e, Ptr::new(source), targets.map(Ptr::new));
        let table = e.call(TABLE_ENTRY, &args![0u32]).u32();
        let value = e.call(SHADOW_SCENE_VALUE, &args![table]).u16();
        e.mem.set_u32(record.addr() + 0x0c, u32::from(value));
        // Unsigned comparisons in the game's order.
        let limit = |e: &Engine, index: u32| e.mem.u32(config.addr() + 4 * index);
        let word = |e: &Engine, index: u32| e.mem.u32(record.addr() + 4 * index);
        let exceeded = word(e, 0) > limit(e, 0)
            || word(e, 1) > limit(e, 1)
            || word(e, 2) > limit(e, 2)
            || word(e, 3) > limit(e, 3)
            || word(e, 5) > limit(e, 5)
            || word(e, 4) > limit(e, 4);
        let callback = e.mem.u32(config.addr() + 0x18);
        if exceeded && callback != 0 {
            e.mem.set_u32(record.addr() + 0x18, cell.addr());
            copy_words(e, record.addr() + 0x1c, position.addr(), 3);
            copy_words(e, record.addr() + 0x28, rotation.addr(), 9);
            let user_data = e.mem.u32(config.addr() + 0x1c);
            e.call(callback, &args![record, user_data]);
        }
        !exceeded
    })
}

// Translated from 005585e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pointer held by the slot at `this + 8` (`00559450`). The map has no
/// name for it.
pub fn fn_005585e0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(SLOT_GET_POINTER, &args![this.addr() + 8]).u32()
}

// Translated from 00558600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the eleven statistics words (`011f9fc8`, `011f9fcc`, `011f9fd8`,
/// `011f9fdc`, `011f9fe0`, `011f9fe4`, `011f9fd0`, `011f9fd4`, `011f9fe8`,
/// `011f9ff0`, `011f9fec`) through its eleven pointer parameters, in that
/// order (a pointer that appears twice keeps the last word). `this` is not
/// used. The map has no name for it.
pub fn fn_00558600(e: &mut Engine, _this: Ptr, targets: [Ptr; 11]) {
    for (target, source) in targets.iter().zip(STATISTICS_SOURCES) {
        let word: u32 = e.global(source);
        e.mem.set_u32(target.addr(), word);
    }
}

// Translated from 00558690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the statistics record: the default limits
/// (`fn_00557d50`), then `006815c0` (a function that returns its `this`) on
/// `+0x1c` and `+0x28`. Returns `this`. The map has no name for it.
pub fn fn_00558690(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00557d50(e, this);
    e.call(LIST_NODE_ITEM_ADDRESS, &args![this.addr() + 0x1c]);
    e.call(LIST_NODE_ITEM_ADDRESS, &args![this.addr() + 0x28]);
    this
}

// Translated from 005586c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(record)`: the report callback of the default configuration,
/// `SaveRenderFailureData(record)`.
pub fn fn_005586c0(e: &mut Engine, record: Ptr) {
    tes_object_cell_save_render_failure_data(e, record);
}

// Translated from 00557dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::RenderTestCell` (Xbox PDB), cdecl `(cell, config,
/// by_reference)`: does nothing without a cell that has a 3D (`00545cb0`).
/// With the default configuration (`fn_00557da0`) when `config` is null, it
/// saves the render camera's translation and rotation, then renders the
/// cell from a set of positions with `fn_00558330`: the player's
/// (virtual `+0x1d0` of `011dea3c`, `0045bb80`) when `by_reference` is 0,
/// otherwise the positions of the navmesh triangle centres (3D lifted by
/// `118.0`) when the cell has navmeshes, or those of its references
/// (stopping when `008256d0` says so). The camera is restored and
/// `"!!!RenderTestCell failed!!!..."` is printed when any render failed.
/// The C++ exception frames are not translated.
pub fn tes_object_cell_render_test_cell(e: &mut Engine, cell: Ptr, config: Ptr, by_reference: u8) {
    if cell.is_null() || e.call(CELL_NODE_OF_CELL, &args![cell]).u32() == 0 {
        return;
    }
    // The game's locals: the default configuration (0x20), the saved
    // translation (12) and rotation (36), the update record (12), a smart
    // pointer slot (4), the navmesh centre (12) and the reference position
    // (12).
    e.with_stack(0x80, |e, frame| {
        let default_config = frame;
        let saved_translation = frame.byte_add(0x20);
        let saved_rotation = frame.byte_add(0x2c);
        let update = frame.byte_add(0x50);
        let slot = frame.byte_add(0x5c);
        let centre = frame.byte_add(0x60);
        let position = frame.byte_add(0x6c);
        let mut passed = true;
        fn_00557da0(e, default_config);
        let config = if config.is_null() {
            default_config.cast()
        } else {
            config
        };
        e.call(LIST_NODE_ITEM_ADDRESS, &args![saved_translation]);
        e.call(LIST_NODE_ITEM_ADDRESS, &args![saved_rotation]);
        let handle = Ptr::new(e.call(RENDER_HANDLE, &[]).u32());
        let node = fn_00558310(e, handle);
        let translation = e.call(NODE_LOCAL_TRANSLATION, &args![node]).u32();
        copy_words(e, saved_translation.addr(), translation, 3);
        let node = fn_00558310(e, handle);
        let rotation = e.call(NODE_LOCAL_ROTATION, &args![node]).u32();
        copy_words(e, saved_rotation.addr(), rotation, 9);
        if by_reference == 0 {
            let player: u32 = e.global(PLAYER_OBJECT);
            let player_node = e.vcall(player, REFERENCE_NODE_SLOT, &[]).u32();
            let place = e.call(NODE_POSITION_POINTER, &args![player_node]).u32();
            if !fn_00558330(e, cell, Ptr::new(place), config) {
                passed = false;
            }
        } else {
            let array = e.call(CELL_NAV_MESH_ARRAY, &args![cell]).u32();
            if array != 0 && e.call(NAV_MESH_ARRAY_COUNT, &args![array]).u32() != 0 {
                let mut triangles = 0u32;
                let mut index = 0u32;
                while index < e.call(NAV_MESH_ARRAY_COUNT, &args![array]).u32() {
                    e.call(NAV_MESH_BY_INDEX, &args![array, slot, index]);
                    let mesh = e.call(SLOT_GET_POINTER, &args![slot]).u32();
                    let count = fn_00558200(e, Ptr::new(mesh));
                    triangles = count.wrapping_add(triangles);
                    e.call(NAV_MESH_SLOT_RELEASE, &args![slot]);
                    index += 1;
                }
                e.call(
                    DEBUG_PRINT_LINE,
                    &args![TESTING_TRIANGLES_FORMAT, triangles],
                );
                let mut index = 0u32;
                while index < e.call(NAV_MESH_ARRAY_COUNT, &args![array]).u32() {
                    e.call(NAV_MESH_BY_INDEX, &args![array, slot, index]);
                    if e.call(NAV_MESH_SLOT_TEST, &args![slot]).u32() != 0 {
                        let mesh = e.call(SLOT_GET_POINTER, &args![slot]).u32();
                        let count = fn_00558200(e, Ptr::new(mesh));
                        let mut triangle = 0u32;
                        while triangle < count {
                            let mesh = e.call(SLOT_GET_POINTER, &args![slot]).u32();
                            nav_mesh_get_center(e, Ptr::new(mesh), centre, triangle as u16);
                            let lift: f64 = e.global(TRIANGLE_LIFT);
                            let height = (f64::from(e.mem.f32(centre.addr() + 8)) + lift) as f32;
                            e.mem.set_f32(centre.addr() + 8, height);
                            if !fn_00558330(e, cell, centre.cast(), config) {
                                passed = false;
                            }
                            triangle += 1;
                        }
                    }
                    e.call(NAV_MESH_SLOT_RELEASE, &args![slot]);
                    index += 1;
                }
            } else {
                let list = e.call(CELL_REFERENCES, &args![cell]).u32();
                let count = e.call(LIST_COUNT, &args![list]).u32();
                e.call(DEBUG_PRINT_LINE, &args![TESTING_POSITIONS_FORMAT, count]);
                let mut node = e.call(CELL_REFERENCES, &args![cell]).u32();
                while node != 0 && !e.call(LIST_NODE_IS_END, &args![node]).bool() {
                    let item = node_item(e, Ptr::new(node)).addr();
                    node = e.call(LIST_NODE_NEXT, &args![node]).u32();
                    let place = e.vcall(item, REFERENCE_POSITION_SLOT, &[]).u32();
                    copy_words(e, position.addr(), place, 3);
                    if e.vcall(item, REFERENCE_NODE_SLOT, &[]).u32() != 0 {
                        let item_node = e.vcall(item, REFERENCE_NODE_SLOT, &[]).u32();
                        let bound = e.call(WORLD_BOUND, &args![item_node]).u32();
                        let centre_of_bound = e.call(LIST_NODE_ITEM_ADDRESS, &args![bound]).u32();
                        copy_words(e, position.addr(), centre_of_bound, 3);
                    }
                    if !fn_00558330(e, cell, position.cast(), config) {
                        passed = false;
                    }
                }
            }
        }
        let node = fn_00558310(e, handle);
        e.call(NODE_SET_TRANSLATION, &args![node, saved_translation]);
        let node = fn_00558310(e, handle);
        e.call(NODE_SET_ROTATION, &args![node, saved_rotation]);
        e.call(UPDATE_RECORD_CONSTRUCT, &args![update, 0.0f32, 0u32, 0u32]);
        let node = fn_00558310(e, handle);
        e.call(CONTROLLER_UPDATE, &args![node, update]);
        if !passed {
            e.call(DEBUG_PRINT_LINE, &args![RENDER_FAILED_MESSAGE]);
        }
    });
}

// Translated from 005586e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL::SaveRenderFailureData` (Xbox PDB), cdecl `(record)`, the
/// callback `RenderTestCell` calls with a failing render's statistics
/// record (words 0 to 3 and 4 as counts, the cell at `+0x18`, the camera
/// position at `+0x1c`, its rotation matrix at `+0x28`). A null record only
/// clears `011ca218`. Otherwise it makes the `RTF` directory (`00b00800`) and
/// the file name `.\RTF\INT-<cell name>.txt` (interior) or `.\RTF\EXT-<world
/// space name>-(<x>, <y>).txt`, with `-<cell name>` before `.txt` when the
/// cell has a name (`00401280`). When the cell is not the one in `011ca218` the
/// file is deleted (`00aff0b0`), created and given the header line;
/// otherwise it is opened for appending. One line follows: the form id of
/// the cell, the camera position, the rotation as Euler angles in degrees
/// (`ToEulerAnglesXYZ`, times `57.29...`) and the five counts. The file object
/// (`BSFile`, `0x158` bytes) is destroyed through its virtual destructor.
/// The C++ exception frames and the stack cookie are not translated.
pub fn tes_object_cell_save_render_failure_data(e: &mut Engine, record: Ptr) {
    if record.is_null() {
        e.set_global(LAST_RENDER_FAILURE_CELL, 0u32);
        return;
    }
    e.call(CREATE_DIRECTORY, &args![RTF_DIRECTORY_NAME]);
    // The path buffer, the line buffer and the three Euler angles.
    e.with_stack(2 * PATH_BUFFER_SIZE + 12, |e, frame| {
        let path = frame.addr();
        let line = frame.addr() + PATH_BUFFER_SIZE;
        let angles = frame.addr() + 2 * PATH_BUFFER_SIZE;
        let cell = e.mem.u32(record.addr() + 0x18);
        if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
            let name = e.vcall(cell, NAME_SLOT, &[]).u32();
            e.call(
                FORMAT_TEXT,
                &args![
                    path,
                    PATH_BUFFER_SIZE,
                    INTERIOR_FILE_FORMAT,
                    RTF_PREFIX,
                    name
                ],
            );
        } else {
            let world_space = e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32();
            let name_pointer = e.call(CELL_NAME_POINTER, &args![cell]).u32();
            if e.mem.u8(name_pointer) != 0 {
                let cell_name = e.vcall(cell, NAME_SLOT, &[]).u32();
                let y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
                let x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
                let world_name = e.vcall(world_space, NAME_SLOT, &[]).u32();
                e.call(
                    FORMAT_TEXT,
                    &args![
                        path,
                        PATH_BUFFER_SIZE,
                        EXTERIOR_NAMED_FILE_FORMAT,
                        RTF_PREFIX,
                        world_name,
                        x,
                        y,
                        cell_name
                    ],
                );
            } else {
                let y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
                let x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
                let world_name = e.vcall(world_space, NAME_SLOT, &[]).u32();
                e.call(
                    FORMAT_TEXT,
                    &args![
                        path,
                        PATH_BUFFER_SIZE,
                        EXTERIOR_FILE_FORMAT,
                        RTF_PREFIX,
                        world_name,
                        x,
                        y
                    ],
                );
            }
        }
        let last: u32 = e.global(LAST_RENDER_FAILURE_CELL);
        let file;
        if last != cell {
            e.call(DELETE_FILE, &args![path]);
            let memory = e.call(OPERATOR_NEW, &args![FILE_SIZE]).u32();
            file = if memory != 0 {
                e.call(
                    FILE_CONSTRUCT,
                    &args![memory, path, FILE_MODE_CREATE, FILE_BUFFER_SIZE, 0u32],
                )
                .u32()
            } else {
                0
            };
            e.vcall(file, FILE_OPEN_SLOT, &args![0u32, 0u32]);
            let origin: u32 = e.global(FILE_SEEK_ORIGIN);
            e.vcall(file, FILE_SEEK_SLOT, &args![0u32, origin]);
            e.call(FORMAT_TEXT, &args![line, PATH_BUFFER_SIZE, HEADER_FORMAT]);
            let length = e.call(STRING_LENGTH, &args![line]).u32();
            e.vcall(file, FILE_WRITE_SLOT, &args![line, length]);
            e.set_global(LAST_RENDER_FAILURE_CELL, cell);
        } else {
            let memory = e.call(OPERATOR_NEW, &args![FILE_SIZE]).u32();
            file = if memory != 0 {
                e.call(
                    FILE_CONSTRUCT,
                    &args![memory, path, FILE_MODE_APPEND, FILE_BUFFER_SIZE, 0u32],
                )
                .u32()
            } else {
                0
            };
            e.vcall(file, FILE_OPEN_SLOT, &args![0u32, 0u32]);
        }
        // The three angles: x, y, z (the game's locals hold them apart).
        e.call(
            MATRIX_TO_EULER,
            &args![record.addr() + 0x28, angles, angles + 4, angles + 8],
        );
        let degrees: f64 = e.global(RADIANS_TO_DEGREES);
        let angle = |e: &Engine, index: u32| f64::from(e.mem.f32(angles + 4 * index)) * degrees;
        let position = |e: &Engine, offset: u32| f64::from(e.mem.f32(record.addr() + offset));
        let form_id = e.call(FORM_ID, &args![cell]).u32();
        let mut words: Vec<u32> = vec![line, PATH_BUFFER_SIZE, LINE_FORMAT, form_id];
        let values = [
            position(e, 0x1c),
            position(e, 0x20),
            position(e, 0x24),
            angle(e, 0),
            angle(e, 1),
            angle(e, 2),
        ];
        for value in values {
            words.extend(args![value]);
        }
        for offset in [0x10u32, 0x00, 0x04, 0x08, 0x0c] {
            words.push(e.mem.u32(record.addr() + offset));
        }
        e.call(FORMAT_TEXT, &words);
        let length = e.call(STRING_LENGTH, &args![line]).u32();
        e.vcall(file, FILE_WRITE_SLOT, &args![line, length]);
        if file != 0 {
            e.vcall(file, 0, &args![1u32]);
        }
    });
}

// Translated from 00558b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `pLightingTemplate` (`+0xd8`) of the cell. The map has no name for it.
pub fn fn_00558b40(e: &mut Engine, this: Ptr<TESObjectCELL>) -> u32 {
    e.get(this, TESObjectCELL::pLightingTemplate).addr()
}

// Translated from 00558b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `pLightingTemplate` (`+0xd8`). The map has no name for it.
pub fn fn_00558b60(e: &mut Engine, this: Ptr<TESObjectCELL>, value: Ptr) {
    e.set(this, TESObjectCELL::pLightingTemplate, value);
}

// Translated from 00558b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `iLightingTemplateInheritanceFlags & mask != 0` (`+0xdc`). The map has no
/// name for it.
pub fn fn_00558b80(e: &mut Engine, this: Ptr<TESObjectCELL>, mask: u32) -> bool {
    e.get(this, TESObjectCELL::iLightingTemplateInheritanceFlags) & mask != 0
}

// Translated from 00558ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the held word of every object whose held word equals `key`: for
/// each node of the cell's reference list (`009604f0`) up to the first whose
/// item is null, `fn_00558c30(item) == key` leads to `fn_00558c60(item, 0)`;
/// then the same for the object at `011dea3c`. The map has no name for it.
pub fn fn_00558ba0(e: &mut Engine, this: Ptr, key: u32) {
    let mut node = e.call(CELL_REFERENCES, &args![this]).u32();
    while node != 0 {
        let address = e.call(LIST_NODE_ITEM_ADDRESS, &args![node]).u32();
        if e.mem.u32(address) == 0 {
            break;
        }
        let item = node_item(e, Ptr::new(node));
        if fn_00558c30(e, item) == key {
            fn_00558c60(e, item, 0);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    let player = Ptr::new(e.global::<u32>(DISTANCE_SOURCE_SINGLETON));
    if fn_00558c30(e, player) == key {
        fn_00558c60(e, player, 0);
    }
}

// Translated from 00558c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word the pointer at `+0x64` points to, or `0` without a pointer.
/// The map has no name for it.
pub fn fn_00558c30(e: &mut Engine, this: Ptr) -> u32 {
    let held = e.mem.u32(this.addr() + HELD_POINTER_OFFSET);
    if held == 0 {
        0
    } else {
        e.mem.u32(held)
    }
}

// Translated from 00558c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word the pointer at `+0x64` points to, when there
/// is a pointer. The map has no name for it.
pub fn fn_00558c60(e: &mut Engine, this: Ptr, value: u32) {
    let held = e.mem.u32(this.addr() + HELD_POINTER_OFFSET);
    if held != 0 {
        e.mem.set_u32(held, value);
    }
}

// Translated from 00558c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// For each node of the cell's reference list up to the first with a null
/// item: when the item's virtual `+0x100` is false, its base object's form
/// type is not `0x1c`, `00549580(item)` is true and the cell has a 3D
/// (`00545cb0`): `00c6a270(3D, 1, 1, 1)` (engine map `bhkWorld::Activate`).
/// The map has no name for it.
pub fn fn_00558c90(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    let mut node = e.call(CELL_REFERENCES, &args![this]).u32();
    while node != 0 {
        let address = e.call(LIST_NODE_ITEM_ADDRESS, &args![node]).u32();
        if e.mem.u32(address) == 0 {
            break;
        }
        let item = node_item(e, Ptr::new(node)).addr();
        if !e.vcall(item, OWNER_REFUSES_SLOT, &[]).bool() {
            let base = e.call(REFERENCE_BASE_OBJECT, &args![item]).u32();
            if e.call(FORM_TYPE, &args![base]).u32() != DOOR_FORM_TYPE
                && e.call(BASE_FORM_FLAG_40, &args![item]).bool()
                && e.call(CELL_NODE_OF_CELL, &args![this]).u32() != 0
            {
                let node_3d = e.call(CELL_NODE_OF_CELL, &args![this]).u32();
                e.call(WORLD_ACTIVATE, &args![node_3d, 1u32, 1u32, 1u32]);
            }
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 00558d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor `(this, size)` of a small class: the base constructor
/// `00558eb0(size)`, then the vtable `0102f2f8`. Returns `this`. The map has
/// no name for it.
pub fn fn_00558d40(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    e.call(SMALL_CLASS_BASE_A, &args![this, size]);
    e.mem.set_u32(this.addr(), SMALL_CLASS_VTABLE_A);
    this
}

// Translated from 00558d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor `(this, size)` of a small class: the base constructor
/// `00558fb0(size)`, then the vtable `0102f318`. Returns `this`. The map has
/// no name for it.
pub fn fn_00558d70(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    e.call(SMALL_CLASS_BASE_B, &args![this, size]);
    e.mem.set_u32(this.addr(), SMALL_CLASS_VTABLE_B);
    this
}

// Translated from 00558da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor `(this, size)` of a small class: the base constructor
/// `005590b0(size)`, then the vtable `0102f338`. Returns `this`. The map has
/// no name for it.
pub fn fn_00558da0(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    e.call(SMALL_CLASS_BASE_C, &args![this, size]);
    e.mem.set_u32(this.addr(), SMALL_CLASS_VTABLE_C);
    this
}

// Translated from 00558dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of element `index` of the 16-byte records whose array
/// pointer is at `this + 4`. The map has no name for it.
pub fn fn_00558dd0(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    let base = e.mem.u32(this.addr() + 4);
    Ptr::new((index << 4).wrapping_add(base))
}

// Translated from 00558df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor the engine map names
/// `NiTMap<TESObjectREFR*,NiNode*>`: the member destructor `00558f20`, then
/// `00401030(this)` (the block release) when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_00558df0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(SMALL_CLASS_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE_BLOCK, &args![this]);
    }
    this
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00552470, fn_00552470(Ptr) -> bool),
        entry!(0x00552490, fn_00552490(Ptr) -> u8),
        entry!(0x005524b0, fn_005524b0(Ptr) -> u8),
        entry!(0x005524d0, fn_005524d0() -> u8),
        entry!(0x005524f0, fn_005524f0(u32)),
        entry!(0x00552570, fn_00552570()),
        entry!(0x005526b0, fn_005526b0(Ptr) -> bool),
        entry!(0x005526d0, fn_005526d0(Ptr, bool)),
        entry!(0x005526f0, fn_005526f0(Ptr)),
        entry!(
            0x00552790,
            tes_object_cell_update_morph_controllers_only(Ptr)
        ),
        entry!(0x00552840, fn_00552840(Ptr<TESObjectCELL>)),
        entry!(0x00552900, fn_00552900(Ptr<TESObjectCELL>, u8)),
        entry!(
            0x00552970,
            tes_object_cell_attach_to_world(Ptr<TESObjectCELL>, Ptr)
        ),
        entry!(0x00552ba0, fn_00552ba0() -> u32),
        entry!(0x00552bb0, fn_00552bb0(Ptr) -> bool),
        entry!(0x00552bd0, tes_object_cell_detach(Ptr<TESObjectCELL>, bool)),
        entry!(0x00552da0, fn_00552da0(Ptr) -> u8),
        entry!(0x00552dc0, fn_00552dc0(Ptr<TESObjectCELL>)),
        entry!(0x005532a0, fn_005532a0(Ptr, f32, f32, f32, f32) -> Ptr),
        entry!(0x00553300, fn_00553300(u32) -> u32),
        entry!(0x00553320, fn_00553320(u8, u32) -> u32),
        entry!(0x00553380, fn_00553380(u32)),
        entry!(0x005533c0, fn_005533c0(Ptr)),
        entry!(0x005533f0, fn_005533f0(Ptr, u32)),
        entry!(0x00553440, fn_00553440(Ptr, u32)),
        entry!(0x00553480, fn_00553480(Ptr, u32)),
        entry!(0x005534f0, fn_005534f0(Ptr, u32)),
        entry!(0x00553520, fn_00553520(Ptr)),
        entry!(0x00553540, hkp_world_cinfo_destructor(Ptr)),
        entry!(
            0x005535c0,
            hkp_world_cinfo_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x005535f0, fn_005535f0(Ptr<TESObjectCELL>)),
        entry!(0x00553700, fn_00553700(Ptr<TESObjectCELL>)),
        entry!(0x00553820, fn_00553820(Ptr<TESObjectCELL>)),
        entry!(
            0x005538e0,
            tes_object_cell_update_ref_sounds(Ptr<TESObjectCELL>)
        ),
        entry!(0x00553b60, fn_00553b60(Ptr) -> i8),
        entry!(0x00553b90, fn_00553b90(Ptr) -> u8),
        entry!(0x00553bb0, fn_00553bb0(Ptr) -> bool),
        entry!(
            0x00553bf0,
            fn_00553bf0(Ptr<TESObjectCELL>, Ptr, u32, u32, f32) -> bool
        ),
        entry!(
            0x00553d00,
            fn_00553d00(Ptr<TESObjectCELL>, Ptr, u32, u32, f32, i8) -> bool
        ),
        entry!(
            0x00553ee0,
            tes_object_cell_pick(Ptr<TESObjectCELL>, Ptr) -> u32
        ),
        entry!(0x00553f70, fn_00553f70(Ptr, Ptr)),
        entry!(0x00553fc0, fn_00553fc0(Ptr, Ptr) -> Ptr),
        entry!(0x00554010, fn_00554010(Ptr) -> Ptr),
        entry!(0x00554590, fn_00554590(Ptr) -> Ptr),
        entry!(0x005545b0, fn_005545b0(Ptr)),
        entry!(0x005545d0, fn_005545d0(Ptr)),
        entry!(0x005545f0, fn_005545f0(Ptr)),
        entry!(0x00554650, fn_00554650(Ptr)),
        entry!(0x00554670, fn_00554670(Ptr, u32) -> Ptr),
        entry!(0x005546a0, tes_object_cell_move_exterior_world(Ptr)),
        entry!(0x00554720, fn_00554720(Ptr, Ptr)),
        entry!(0x00554780, fn_00554780()),
        entry!(
            0x005547c0,
            tes_object_cell_get_land_height(Ptr<TESObjectCELL>, Ptr, Ptr) -> u8
        ),
        entry!(
            0x00554810,
            tes_object_cell_is_form_cell_child(Ptr) -> u8
        ),
        entry!(0x00554880, fn_00554880(Ptr, u8) -> bool),
        entry!(0x005548a0, fn_005548a0(u8) -> bool),
        entry!(0x00554910, fn_00554910(Ptr<TESObjectCELL>) -> u8),
        entry!(0x00554960, fn_00554960(Ptr<TESObjectCELL>, u8)),
        entry!(0x005549d0, fn_005549d0(Ptr<TESObjectCELL>, u8)),
        entry!(0x00554a20, fn_00554a20(Ptr<TESObjectCELL>, u8)),
        entry!(0x00554b90, fn_00554b90(Ptr<TESObjectCELL>, u32) -> u16),
        entry!(0x00554d30, fn_00554d30(Ptr<TESObjectCELL>, u32)),
        entry!(0x00554fd0, fn_00554fd0(Ptr<TESObjectCELL>, u32, u32)),
        entry!(0x00555570, fn_00555570(Ptr<TESObjectCELL>, u32)),
        entry!(0x005555d0, fn_005555d0(Ptr<TESObjectCELL>, u32, u32)),
        entry!(0x00555630, fn_00555630(Ptr<TESObjectCELL>, Ptr)),
        entry!(0x00555740, fn_00555740(Ptr<TESObjectCELL>, Ptr)),
        entry!(0x005559d0, fn_005559d0(Ptr<TESObjectCELL>, Ptr)),
        entry!(0x00555a50, fn_00555a50(Ptr<TESObjectCELL>, Ptr)),
        entry!(0x00555ad0, fn_00555ad0(Ptr<TESObjectCELL>) -> f32),
        entry!(
            0x00555b10,
            tes_object_cell_adjust_coord_for_north_rotation(Ptr<TESObjectCELL>, Ptr, Ptr, u8)
        ),
        entry!(0x00556870, tes_object_cell_get_seen_value(Ptr) -> i32),
        entry!(
            0x00556ef0,
            tes_object_cell_get_int_seen_section(Ptr<TESObjectCELL>, i32, i32, u8) -> Ptr
        ),
        entry!(0x00557090, fn_00557090(Ptr) -> bool),
        entry!(0x005570b0, fn_005570b0(Ptr) -> bool),
        entry!(0x005570d0, fn_005570d0(Ptr<TESObjectCELL>)),
        entry!(
            0x005571a0,
            tes_object_cell_queue_references(Ptr<TESObjectCELL>, u8)
        ),
        entry!(0x005573e0, fn_005573e0(Ptr<TESObjectCELL>)),
        entry!(0x00557470, fn_00557470(Ptr<TESObjectCELL>)),
        entry!(0x005574d0, fn_005574d0(Ptr<TESObjectCELL>)),
        entry!(0x005575d0, fn_005575d0(Ptr<TESObjectCELL>)),
        entry!(0x005576c0, fn_005576c0(Ptr<TESObjectCELL>)),
        entry!(0x00557760, fn_00557760(Ptr, Ptr)),
        entry!(
            0x005577b0,
            tes_object_cell_process_addon_nodes_for_refs(Ptr<TESObjectCELL>, u8)
        ),
        entry!(0x005578e0, fn_005578e0(Ptr, u32, u32, u32) -> u32),
        entry!(0x005578f0, fn_005578f0(Ptr)),
        entry!(0x00557990, fn_00557990(Ptr, u8, u32)),
        entry!(0x00557aa0, fn_00557aa0(Ptr<TESObjectCELL>)),
        entry!(0x00557ae0, fn_00557ae0(Ptr<TESObjectCELL>)),
        entry!(0x00557be0, fn_00557be0(Ptr)),
        entry!(0x00557c40, fn_00557c40(Ptr<TESObjectCELL>)),
        entry!(0x00557d10, fn_00557d10(Ptr<TESObjectCELL>) -> bool),
        entry!(
            0x00557d30,
            tes_object_cell_all3d_with_visible_distant_fading_in(Ptr<TESObjectCELL>) -> u8
        ),
        entry!(0x00557d50, fn_00557d50(Ptr) -> Ptr),
        entry!(0x00557da0, fn_00557da0(Ptr) -> Ptr),
        entry!(0x00557dd0, tes_object_cell_render_test_cell(Ptr, Ptr, u8)),
        entry!(0x00558200, fn_00558200(Ptr) -> u32),
        entry!(0x00558220, nav_mesh_get_center(Ptr, Ptr, u16) -> Ptr),
        entry!(0x005582d0, fn_005582d0(Ptr, u32) -> u16),
        entry!(0x005582f0, fn_005582f0(Ptr, u16) -> Ptr),
        entry!(0x00558310, fn_00558310(Ptr) -> u32),
        entry!(0x00558330, fn_00558330(Ptr, Ptr, Ptr) -> bool),
        entry!(0x00558430, fn_00558430(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x005585e0, fn_005585e0(Ptr) -> u32),
        (
            0x00558600,
            (|e: &mut Engine, a: &[u32]| {
                let targets: [Ptr; 11] = std::array::from_fn(|index| Ptr::new(a[1 + index]));
                fn_00558600(e, Ptr::new(a[0]), targets);
                Ret::default()
            }) as AbiFn,
        ),
        entry!(0x00558690, fn_00558690(Ptr) -> Ptr),
        entry!(0x005586c0, fn_005586c0(Ptr)),
        entry!(0x005586e0, tes_object_cell_save_render_failure_data(Ptr)),
        entry!(0x00558b40, fn_00558b40(Ptr<TESObjectCELL>) -> u32),
        entry!(0x00558b60, fn_00558b60(Ptr<TESObjectCELL>, Ptr)),
        entry!(0x00558b80, fn_00558b80(Ptr<TESObjectCELL>, u32) -> bool),
        entry!(0x00558ba0, fn_00558ba0(Ptr, u32)),
        entry!(0x00558c30, fn_00558c30(Ptr) -> u32),
        entry!(0x00558c60, fn_00558c60(Ptr, u32)),
        entry!(0x00558c90, fn_00558c90(Ptr<TESObjectCELL>)),
        entry!(0x00558d40, fn_00558d40(Ptr, u32) -> Ptr),
        entry!(0x00558d70, fn_00558d70(Ptr, u32) -> Ptr),
        entry!(0x00558da0, fn_00558da0(Ptr, u32) -> Ptr),
        entry!(0x00558dd0, fn_00558dd0(Ptr, u32) -> Ptr),
        entry!(0x00558df0, fn_00558df0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The vtable of every test object: slot `n` leads to the address
    /// `fake(n)`, where a test registers the double it needs.
    const TEST_VTABLE: u32 = 0x7f00_0000;

    fn fake(slot: u32) -> u32 {
        0x7e00_0000 + slot
    }

    /// An engine with the exe's data pages mapped (zero filled) and the test
    /// vtable.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(TEST_VTABLE, 0x1000);
        for slot in (0..0x300).step_by(4) {
            e.mem.set_u32(TEST_VTABLE + slot, fake(slot));
        }
        e
    }

    /// A zeroed object of `size` bytes whose vtable is the test vtable.
    fn object(e: &mut Engine, size: u32) -> u32 {
        let at = e.mem.alloc(size);
        e.mem.set_u32(at, TEST_VTABLE);
        at
    }

    /// Registers do-nothing doubles returning 0.
    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// A double that returns `value`.
    fn returns(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| value.into_ret());
    }

    fn start_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// The calls made after the one under test, as `(address, words)`.
    fn calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.clone().unwrap().into_iter().skip(1).collect()
    }

    fn addresses(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter().map(|(address, _)| *address).collect()
    }

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Every function of this file is registered under its address.
    #[test]
    fn functions_are_registered() {
        let table = funcs();
        assert_eq!(table.len(), 120);
        let mut seen: Vec<u32> = table.iter().map(|(a, _)| *a).collect();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), 120);
        assert!(seen.iter().all(|a| (0x0055_2470..0x0055_8e20).contains(a)));
    }

    #[test]
    fn flag_100000_is_tested_through_the_flag_word_helper() {
        let mut e = engine();
        returns(&mut e, TEST_FLAG_BITS, 1);
        start_log(&mut e);
        assert!(e.call(0x0055_2470, &args![0x1234u32]).bool());
        assert_eq!(calls(&e), vec![(TEST_FLAG_BITS, vec![0x1234, 0x0010_0000])]);
    }

    #[test]
    fn byte_accessors_read_their_offsets() {
        let mut e = engine();
        let at = e.mem.alloc(0x100);
        e.mem.set_u8(at + 0x41, 7);
        e.mem.set_u8(at + 0x8c, 9);
        assert_eq!(e.call(0x0055_2490, &args![at]).u8(), 7);
        assert_eq!(e.call(0x0055_24b0, &args![at]).u8(), 9);
        assert_eq!(e.call(0x0055_2da0, &args![at - 0x41 + 0x26]).u8(), 0);
        e.mem.set_u8(at + 0x26, 3);
        assert_eq!(e.call(0x0055_2da0, &args![at]).u8(), 3);
        e.mem.set_u8(at + 0x210, 1);
        assert!(e.call(0x0055_2bb0, &args![at]).bool());
        assert!(!e.call(0x0055_2bb0, &args![at + 8]).bool());
    }

    #[test]
    fn byte_holder_is_read_through_its_getter() {
        let mut e = engine();
        let byte = e.mem.alloc(4);
        e.mem.set_u8(byte, 0x5a);
        returns(&mut e, BYTE_HOLDER_GET, byte);
        start_log(&mut e);
        assert_eq!(e.call(0x0055_24d0, &[]).u8(), 0x5a);
        assert_eq!(calls(&e), vec![(BYTE_HOLDER_GET, vec![BYTE_HOLDER])]);
    }

    #[test]
    fn queueing_a_node_wraps_it_in_a_slot_under_the_lock() {
        let mut e = engine();
        quiet(
            &mut e,
            &[LOCK_ENTER, LOCK_LEAVE, PENDING_LIST_ADD, SLOT_RELEASE],
        );
        e.register(SLOT_FROM_RAW, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let slot_seen = Rc::new(RefCell::new(0));
        let seen = slot_seen.clone();
        e.register_double(PENDING_LIST_ADD, move |e, a| {
            *seen.borrow_mut() = e.mem.u32(a[1]);
            Ret::default()
        });
        start_log(&mut e);
        e.call(0x0055_24f0, &args![0x4455u32]);
        let log = calls(&e);
        assert_eq!(
            addresses(&log),
            vec![
                LOCK_ENTER,
                SLOT_FROM_RAW,
                PENDING_LIST_ADD,
                SLOT_RELEASE,
                LOCK_LEAVE
            ]
        );
        assert_eq!(log[0].1, vec![PENDING_LOCK, 0]);
        assert_eq!(calls_to(&log, PENDING_LIST_ADD)[0][0], PENDING_LIST);
        assert_eq!(*slot_seen.borrow(), 0x4455);
        assert_eq!(log[4].1, vec![PENDING_LOCK]);
    }

    /// Two queued nodes: the first has a parent, the flag, a reference and a
    /// parent cell; the second has no parent.
    #[test]
    fn worker_updates_the_cell_of_a_flagged_node_and_clears_the_flag() {
        let mut e = engine();
        let complete = object(&mut e, 0x100);
        let orphan = object(&mut e, 0x100);
        e.mem.set_u32(complete + 0x30, 0x0040_0000);
        e.mem.set_u32(complete + 0x80, 1);
        e.mem.set_u32(orphan + 0x30, 0x0040_0000);
        quiet(
            &mut e,
            &[
                LOCK_ENTER,
                LOCK_LEAVE,
                PENDING_LIST_REMOVE_HEAD,
                SLOT_RELEASE,
                CELL_UPDATE_FOR_REFERENCE,
                SET_FLAG_BITS,
            ],
        );
        // The children and controller virtuals answer nothing; the parent
        // virtual reads a word of the node.
        quiet(&mut e, &[fake(0x0c), fake(0x30)]);
        e.register(fake(0x10), |e, a| e.mem.u32(a[0] + 0x80).into_ret());
        let queue = Rc::new(RefCell::new(vec![orphan, complete]));
        let remaining = queue.clone();
        e.register_double(PENDING_LIST_IS_EMPTY, move |_, _| {
            u32::from(remaining.borrow().is_empty()).into_ret()
        });
        returns(&mut e, LIST_NODE_ITEM_ADDRESS, 0x1000);
        let source = queue.clone();
        e.register_double(SLOT_COPY, move |e, a| {
            let node = source.borrow_mut().pop().unwrap();
            e.mem.set_u32(a[0], node);
            Ret::default()
        });
        e.register(SLOT_GET_POINTER, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(TEST_FLAG_BITS, |e, a| {
            u32::from(e.mem.u32(a[0] + 0x30) & a[1] != 0).into_ret()
        });
        returns(&mut e, FIND_REFERENCE_FOR_3D, 0x9100);
        returns(&mut e, REFERENCE_GET_PARENT_CELL, 0x9200);
        start_log(&mut e);
        e.call(0x0055_2570, &[]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, FIND_REFERENCE_FOR_3D), vec![vec![complete]]);
        assert_eq!(
            calls_to(&log, CELL_UPDATE_FOR_REFERENCE),
            vec![vec![0x9200, 0x9100, 1, 0]]
        );
        assert_eq!(
            calls_to(&log, SET_FLAG_BITS),
            vec![vec![complete, 0, 0x0040_0000]]
        );
        assert_eq!(calls_to(&log, SLOT_RELEASE).len(), 2);
        assert_eq!(calls_to(&log, LOCK_LEAVE).len(), 1);
    }

    #[test]
    fn flag_400000_test_and_set() {
        let mut e = engine();
        returns(&mut e, TEST_FLAG_BITS, 1);
        quiet(&mut e, &[SET_FLAG_BITS]);
        start_log(&mut e);
        assert!(e.call(0x0055_26b0, &args![0x77u32]).bool());
        assert_eq!(calls(&e), vec![(TEST_FLAG_BITS, vec![0x77, 0x0040_0000])]);
        start_log(&mut e);
        e.call(0x0055_26d0, &args![0x77u32, 1u32]);
        assert_eq!(calls(&e), vec![(SET_FLAG_BITS, vec![0x77, 1, 0x0040_0000])]);
        start_log(&mut e);
        e.call(0x0055_26d0, &args![0x77u32, 0u32]);
        assert_eq!(calls(&e), vec![(SET_FLAG_BITS, vec![0x77, 0, 0x0040_0000])]);
    }

    /// Doubles for the update record and the child array (`0x7000_0000 + n`
    /// is child number `n`).
    fn morph_doubles(e: &mut Engine) {
        e.register(UPDATE_RECORD_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        quiet(e, &[CONTROLLER_UPDATE, fake(0x94)]);
        e.register(NODE_CHILD_COUNT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(NODE_CHILD_AT, |e, a| {
            e.mem.u32(a[0] + 4 + 4 * a[1]).into_ret()
        });
    }

    #[test]
    fn controller_walk_updates_leaf_controllers_only() {
        let mut e = engine();
        morph_doubles(&mut e);
        e.set_global(UPDATE_TIME, 0.25f32);
        // The child array of the root: two children; the first has a
        // controller, the second is a node without controller; the root has
        // no controller of its own.
        let array = e.mem.alloc(16);
        let root = object(&mut e, 0x40);
        let with_controller = object(&mut e, 0x40);
        let without = object(&mut e, 0x40);
        let controller = object(&mut e, 0x40);
        e.mem.set_u32(array, 2);
        e.mem.set_u32(array + 4, with_controller);
        e.mem.set_u32(array + 8, without);
        e.mem.set_u32(root + 0x38, array);
        e.mem.set_u32(with_controller + 0x3c, controller);
        e.register(fake(0x0c), |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        e.register(fake(0x30), |e, a| e.mem.u32(a[0] + 0x3c).into_ret());
        start_log(&mut e);
        e.call(0x0055_26f0, &args![root]);
        let log = calls(&e);
        let updates = calls_to(&log, CONTROLLER_UPDATE);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0][0], controller);
        let record = calls_to(&log, UPDATE_RECORD_CONSTRUCT);
        assert_eq!(record.len(), 1);
        assert_eq!(&record[0][1..], &[0.25f32.to_bits(), 1, 0]);
        // A null node does nothing.
        start_log(&mut e);
        e.call(0x0055_26f0, &args![0u32]);
        assert!(calls(&e).is_empty());
    }

    #[test]
    fn morph_walk_updates_every_node_with_a_controller() {
        let mut e = engine();
        morph_doubles(&mut e);
        e.set_global(UPDATE_TIME, 2.0f32);
        let array = e.mem.alloc(16);
        let root = object(&mut e, 0x40);
        let child = object(&mut e, 0x40);
        let controller = object(&mut e, 0x40);
        e.mem.set_u32(array, 1);
        e.mem.set_u32(array + 4, child);
        e.mem.set_u32(root + 0x38, array);
        e.mem.set_u32(child + 0x3c, controller);
        e.register(fake(0x0c), |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        // GetController(node, type): the root has none, the child has one.
        e.register(NODE_GET_CONTROLLER, |e, a| {
            e.mem.u32(a[0] + 0x3c).into_ret()
        });
        start_log(&mut e);
        e.call(0x0055_2790, &args![root]);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, NODE_GET_CONTROLLER),
            vec![
                vec![root, MORPH_CONTROLLER_TYPE],
                vec![child, MORPH_CONTROLLER_TYPE]
            ]
        );
        // The controller's virtual +0x94 got the record.
        assert_eq!(calls_to(&log, fake(0x94)).len(), 1);
        assert_eq!(calls_to(&log, fake(0x94))[0][0], controller);
        assert_eq!(
            calls_to(&log, UPDATE_RECORD_CONSTRUCT)[0][1],
            2.0f32.to_bits()
        );
        start_log(&mut e);
        e.call(0x0055_2790, &args![0u32]);
        assert!(calls(&e).is_empty());
    }

    #[test]
    fn cell_update_sends_a_record_to_the_cell_node_and_the_animated_nodes() {
        let mut e = engine();
        e.register(UPDATE_RECORD_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xfeed);
            Ret::default()
        });
        quiet(&mut e, &[fake(0xc0)]);
        let cell = e.new_object::<TESObjectCELL>();
        let own = object(&mut e, 0x40);
        let other = object(&mut e, 0x40);
        let loaded = e.new_object::<LoadedCellData>();
        e.set(cell, TESObjectCELL::pLoadedData, loaded.cast());
        returns(&mut e, CELL_NODE_A, own);
        // The map holds three entries: a null value, the cell's own node and
        // another node.
        returns(&mut e, MAP_FIRST_POSITION, 1);
        let entries = Rc::new(RefCell::new(vec![other, own, 0]));
        let queue = entries.clone();
        e.register_double(MAP_NEXT, move |e, a| {
            let value = queue.borrow_mut().pop().unwrap();
            e.mem.set_u32(a[2], 0x1111);
            e.mem.set_u32(a[3], value);
            if queue.borrow().is_empty() {
                e.mem.set_u32(a[1], 0);
            }
            Ret::default()
        });
        e.register(ANIMATED_NODE_OWNER, |_, a| a[0].into_ret());
        // For the second entry the owner is made to differ from `own`.
        e.register_double(ANIMATED_NODE_OWNER, move |_, a| {
            if a[0] == own { own } else { 0x1 }.into_ret()
        });
        start_log(&mut e);
        e.call(0x0055_2840, &args![cell]);
        let log = calls(&e);
        let sent = calls_to(&log, fake(0xc0));
        // The cell node and `other` (whose owner differs) got the record.
        assert_eq!(sent.len(), 2);
        assert_eq!(sent[0][0], own);
        assert_eq!(sent[1][0], other);
        let map = loaded.addr() + LoadedCellData::AnimatedRefMap.off;
        assert_eq!(calls_to(&log, MAP_FIRST_POSITION), vec![vec![map]]);
        let record = calls_to(&log, UPDATE_RECORD_CONSTRUCT);
        assert_eq!(&record[0][1..], &[0, 0, 0]);
    }

    #[test]
    fn cell_update_without_loaded_data_only_updates_the_cell_node() {
        let mut e = engine();
        quiet(&mut e, &[UPDATE_RECORD_CONSTRUCT, fake(0xc0)]);
        let cell = e.new_object::<TESObjectCELL>();
        returns(&mut e, CELL_NODE_A, 0);
        start_log(&mut e);
        e.call(0x0055_2840, &args![cell]);
        assert_eq!(
            addresses(&calls(&e)),
            vec![UPDATE_RECORD_CONSTRUCT, CELL_NODE_A]
        );
    }

    #[test]
    fn reference_flags_are_updated_and_exterior_world_space_notified() {
        let mut e = engine();
        quiet(&mut e, &[CELL_UPDATE_REFERENCE_FLAGS, fake(0xc8)]);
        let cell = e.new_object::<TESObjectCELL>();
        let world_space = object(&mut e, 0x40);
        // Flag 0x2 set, no flag 0x4000, exterior, with a world space.
        e.register(FORM_FLAG_4000, |_, _| Ret::default());
        returns(&mut e, FORM_FLAG_2, 1);
        returns(&mut e, CELL_IS_INTERIOR, 0);
        returns(&mut e, CELL_GET_WORLD_SPACE, world_space);
        start_log(&mut e);
        e.call(0x0055_2900, &args![cell, 5u32]);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, CELL_UPDATE_REFERENCE_FLAGS),
            vec![vec![cell.addr(), 5]]
        );
        assert_eq!(calls_to(&log, fake(0xc8)), vec![vec![world_space, 1]]);
        // An interior cell, a cell without flag 2 and one with flag 0x4000
        // leave the world space alone.
        returns(&mut e, CELL_IS_INTERIOR, 1);
        start_log(&mut e);
        e.call(0x0055_2900, &args![cell, 0u32]);
        assert!(calls_to(&calls(&e), fake(0xc8)).is_empty());
        returns(&mut e, CELL_IS_INTERIOR, 0);
        returns(&mut e, FORM_FLAG_2, 0);
        start_log(&mut e);
        e.call(0x0055_2900, &args![cell, 0u32]);
        assert!(calls_to(&calls(&e), fake(0xc8)).is_empty());
        returns(&mut e, FORM_FLAG_2, 1);
        returns(&mut e, FORM_FLAG_4000, 1);
        start_log(&mut e);
        e.call(0x0055_2900, &args![cell, 0u32]);
        assert!(calls_to(&calls(&e), fake(0xc8)).is_empty());
        // No world space.
        returns(&mut e, FORM_FLAG_4000, 0);
        returns(&mut e, CELL_GET_WORLD_SPACE, 0);
        start_log(&mut e);
        e.call(0x0055_2900, &args![cell, 0u32]);
        assert!(calls_to(&calls(&e), fake(0xc8)).is_empty());
    }

    #[test]
    fn render_singleton_is_read_from_its_global() {
        let mut e = engine();
        e.set_global(RENDER_SINGLETON, 0x1357u32);
        assert_eq!(e.call(0x0055_2ba0, &[]).u32(), 0x1357);
    }

    /// Builds a list of `{ item, next }` nodes, returns the first node.
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

    /// The list accessors read nodes of that shape.
    fn list_doubles(e: &mut Engine) {
        e.register(LIST_NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(LIST_NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_NODE_IS_END, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
    }

    /// Doubles for the guard calls and the word-field getters.
    fn guard_doubles(e: &mut Engine) {
        quiet(e, &[SCOPE_GUARD_CONSTRUCT, SCOPE_GUARD_DESTRUCT]);
    }

    /// Doubles for everything `AttachToWorld` calls; returns the cell, the
    /// parent node, the render singleton and the child node of the portal
    /// graph slot. The cell is interior with a Havok world; the render
    /// singleton wants no nav meshes; the always-render array is empty.
    fn attach_setup(e: &mut Engine) -> (Ptr<TESObjectCELL>, u32, u32, u32) {
        guard_doubles(e);
        let cell = e.new_object::<TESObjectCELL>();
        e.set(cell, TESObjectCELL::bCellDetached, true);
        let parent = object(e, 0x40);
        let world = object(e, 0x40);
        let render = e.mem.alloc(0x300);
        e.set_global(RENDER_SINGLETON, render);
        quiet(
            e,
            &[
                CELL_SET_STATE,
                CELL_SET_LIGHTS_ATTACHED,
                CELL_SET_ADDON_NODES_ATTACHED,
                CELL_SET_ATTACHED_FLAG,
                NAV_MESH_RENDER_ADD,
                CELL_AFTER_ATTACH_A,
                CELL_AFTER_ATTACH_B,
                OBSTACLE_MANAGER_RUN,
                SCENE_NODE_ATTACH,
                PORTAL_ALWAYS_RENDER_ADD,
                PORTAL_ALWAYS_RENDER_SET,
                WORLD_SET_VISUAL_DEBUGGER_ENABLED,
                WORLD_SET_VISUAL_DEBUGGER,
                fake(0xdc),
            ],
        );
        returns(e, CELL_LOAD_3D, 0xa3d);
        returns(e, CELL_HAVOK_WORLD, world);
        returns(e, CELL_IS_INTERIOR, 1);
        returns(e, VISUAL_DEBUGGER_SUPPRESSED, 0);
        returns(e, VISUAL_DEBUGGER_SETTING, 1);
        returns(e, OBSTACLE_MANAGER_GET, 0x0b57);
        returns(e, CELL_OPTIONS, 0x0905);
        returns(e, TABLE_ENTRY, 0x7ab1);
        returns(e, PORTAL_ALWAYS_RENDER_COUNT, 0);
        returns(e, CELL_NODE_A, 0xa);
        returns(e, CELL_NODE_B, 0xb);
        returns(e, CELL_NODE_C, 0xc);
        returns(e, CELL_NODE_D, 0xd);
        returns(e, SCENE_NODE_CHILD, 0x5c);
        returns(e, CELL_PORTAL_GRAPH, 0x96);
        let child = object(e, 0x40);
        returns(e, CELL_CHILD_NODE, child);
        (cell, parent, render, child)
    }

    #[test]
    fn attach_to_world_attaches_the_3d_and_registers_the_nodes() {
        let mut e = engine();
        let (cell, parent, render, child) = attach_setup(&mut e);
        e.mem.set_u8(render + 0x210, 1);
        let world = e.call(CELL_HAVOK_WORLD, &args![cell]).u32();
        start_log(&mut e);
        e.call(0x0055_2970, &args![cell, parent]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, CELL_SET_STATE), vec![vec![cell.addr(), 5]]);
        // The 3D goes under the parent (virtual +0xdc), the portal graph slot
        // under the cell's child node.
        assert_eq!(
            calls_to(&log, fake(0xdc)),
            vec![vec![parent, 0xa3d, 1], vec![child, 0x96, 1]]
        );
        // The interior cell skips the exterior terrain, the world gets the
        // visual debugger flags.
        assert!(calls_to(&log, CELL_COLLISION_WORLD).is_empty());
        assert_eq!(
            calls_to(&log, WORLD_SET_VISUAL_DEBUGGER_ENABLED),
            vec![vec![world, 1]]
        );
        assert_eq!(
            calls_to(&log, WORLD_SET_VISUAL_DEBUGGER),
            vec![vec![world, 1]]
        );
        // The render singleton wants the nav meshes.
        assert_eq!(
            calls_to(&log, NAV_MESH_RENDER_ADD),
            vec![vec![render, cell.addr(), 1]]
        );
        assert_eq!(
            calls_to(&log, SCENE_NODE_ATTACH),
            vec![vec![0x7ab1, 0x0905]]
        );
        // The always-render array is empty: the cell's two nodes and the
        // table child are added.
        assert_eq!(
            calls_to(&log, PORTAL_ALWAYS_RENDER_ADD),
            vec![vec![0x0905, 0xa], vec![0x0905, 0xb], vec![0x0905, 0x5c]]
        );
        assert_eq!(
            calls_to(&log, PORTAL_ALWAYS_RENDER_SET),
            vec![
                vec![0x0905, 0xc],
                vec![0x0905, 0xb],
                vec![0x0905, child],
                vec![0x0905, 0xd]
            ]
        );
        assert!(!e.get(cell, TESObjectCELL::bCellDetached));
        let guard = calls_to(&log, SCOPE_GUARD_CONSTRUCT);
        assert_eq!(&guard[0][1..], &[0x1a, 1, SOURCE_FILE_NAME, 0x395d]);
        assert_eq!(calls_to(&log, SCOPE_GUARD_DESTRUCT).len(), 1);
    }

    #[test]
    fn attach_to_world_without_world_or_nav_meshes_and_with_a_filled_array() {
        let mut e = engine();
        let (cell, parent, _render, _child) = attach_setup(&mut e);
        returns(&mut e, CELL_HAVOK_WORLD, 0);
        returns(&mut e, PORTAL_ALWAYS_RENDER_COUNT, 4);
        start_log(&mut e);
        e.call(0x0055_2970, &args![cell, parent]);
        let seen = addresses(&calls(&e));
        assert!(!seen.contains(&NAV_MESH_RENDER_ADD));
        assert!(!seen.contains(&PORTAL_ALWAYS_RENDER_ADD));
        assert!(!seen.contains(&WORLD_SET_VISUAL_DEBUGGER));
    }

    #[test]
    fn attach_to_world_builds_the_terrain_of_an_exterior_cell_with_a_world() {
        let mut e = engine();
        let (cell, parent, _render, _child) = attach_setup(&mut e);
        returns(&mut e, CELL_HAVOK_WORLD, 0x5511);
        returns(&mut e, CELL_IS_INTERIOR, 0);
        returns(&mut e, VISUAL_DEBUGGER_SUPPRESSED, 1);
        // No collision world: the terrain call is skipped, but the data
        // accessors are asked.
        returns(&mut e, CELL_GET_DATA_X, 0);
        returns(&mut e, CELL_GET_DATA_Y, 0);
        returns(&mut e, CELL_COLLISION_WORLD, 0);
        start_log(&mut e);
        e.call(0x0055_2970, &args![cell, parent]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, CELL_COLLISION_WORLD).len(), 1);
        assert_eq!(
            calls_to(&log, WORLD_SET_VISUAL_DEBUGGER_ENABLED),
            vec![vec![0x5511, 0]]
        );
    }

    /// The doubles of `Detach` that stand for the scene, the loader and the
    /// render singleton; returns the cell.
    fn detach_setup(e: &mut Engine, state: u32) -> Ptr<TESObjectCELL> {
        guard_doubles(e);
        let cell = e.new_object::<TESObjectCELL>();
        let render = e.mem.alloc(0x300);
        e.set_global(RENDER_SINGLETON, render);
        e.set_global(SCENE_SINGLETON, 0x5c00u32);
        e.set_global(LOADER_SINGLETON, 0x1d00u32);
        e.set_global(LOADER_STATE_SINGLETON, 0x1500u32);
        e.set_global(MODEL_LOADER_SINGLETON, 0x3d00u32);
        let state_object = e.mem.alloc(0x40);
        e.set_global(STATE_OBJECT_SINGLETON, state_object);
        quiet(
            e,
            &[
                CELL_LOCK_ENTER,
                CELL_LOCK_LEAVE,
                CELL_SET_STATE,
                CELL_BEFORE_DETACH,
                MODEL_LOADER_CANCEL_FOR_CELL,
                SCENE_REFRESH,
                CELL_SET_LIGHTS_ATTACHED,
                SCENE_ATTACH_EXTERIOR,
                SCENE_ATTACH_INTERIOR,
                CELL_SET_ADDON_NODES_ATTACHED,
                CELL_DETACH_STEP_A,
                CELL_DETACH_STEP_B,
                CELL_SET_TERRAIN_UPDATE,
                CELL_CLEAR_LOADED_STATE,
                CELL_SET_ATTACHED_FLAG,
                NAV_MESH_RENDER_REMOVE,
                REMOVE_CHANGES,
                PORTAL_ALWAYS_RENDER_REMOVE,
                fake(0xe8),
            ],
        );
        returns(e, CELL_GET_STATE, state);
        returns(e, CELL_IS_INTERIOR, 0);
        returns(e, CELL_LOAD_3D, 0x3d);
        let parent = object(e, 0x40);
        // `WORD_AT_0C` and `FORM_ID` are the same getter: the form id of the
        // cell (0xf0f0) and the word at +0x0c of the scene singleton (the
        // parent node).
        let cell_address = cell.addr();
        e.register_double(WORD_AT_0C, move |_, a| {
            if a[0] == cell_address { 0xf0f0 } else { parent }.into_ret()
        });
        returns(e, FORM_HAS_CHANGES, 1);
        returns(e, CELL_OPTIONS, 0x0905);
        returns(e, CELL_NODE_B, 0xb);
        returns(e, CELL_NODE_C, 0xc);
        returns(e, CELL_NODE_D, 0xd);
        returns(e, CELL_CHILD_NODE, 0x2c);
        cell
    }

    #[test]
    fn detach_takes_the_cell_out_of_the_scene_in_order() {
        let mut e = engine();
        let cell = detach_setup(&mut e, 5);
        start_log(&mut e);
        e.call(0x0055_2bd0, &args![cell, 0u32]);
        let log = calls(&e);
        let expected = [
            CELL_LOCK_ENTER,
            CELL_GET_STATE,
            CELL_SET_STATE,
            CELL_BEFORE_DETACH,
            MODEL_LOADER_CANCEL_FOR_CELL,
            SCENE_REFRESH,
            CELL_SET_LIGHTS_ATTACHED,
            CELL_IS_INTERIOR,
            SCENE_ATTACH_EXTERIOR,
            CELL_SET_ADDON_NODES_ATTACHED,
            CELL_LOAD_3D,
            WORD_AT_0C,
            fake(0xe8),
            CELL_DETACH_STEP_A,
            CELL_DETACH_STEP_B,
            CELL_SET_STATE,
            CELL_SET_TERRAIN_UPDATE,
            CELL_CLEAR_LOADED_STATE,
            CELL_SET_ATTACHED_FLAG,
            NAV_MESH_RENDER_REMOVE,
            FORM_ID,
            FORM_HAS_CHANGES,
            REMOVE_CHANGES,
            SCENE_REFRESH,
            CELL_OPTIONS,
            CELL_NODE_C,
            PORTAL_ALWAYS_RENDER_REMOVE,
            CELL_NODE_B,
            PORTAL_ALWAYS_RENDER_REMOVE,
            CELL_CHILD_NODE,
            PORTAL_ALWAYS_RENDER_REMOVE,
            CELL_NODE_D,
            PORTAL_ALWAYS_RENDER_REMOVE,
            CELL_LOCK_LEAVE,
        ];
        assert_eq!(addresses(&log), expected);
        assert_eq!(
            calls_to(&log, CELL_SET_STATE),
            vec![vec![cell.addr(), 4], vec![cell.addr(), 3]]
        );
        assert_eq!(
            calls_to(&log, MODEL_LOADER_CANCEL_FOR_CELL),
            vec![vec![0x3d00, cell.addr()]]
        );
        assert_eq!(
            calls_to(&log, SCENE_ATTACH_EXTERIOR),
            vec![vec![0x5c00, cell.addr()]]
        );
        assert_eq!(calls_to(&log, FORM_HAS_CHANGES), vec![vec![0x1500, 0xf0f0]]);
        assert_eq!(
            calls_to(&log, REMOVE_CHANGES),
            vec![vec![0x1d00, cell.addr()]]
        );
        assert_eq!(calls_to(&log, fake(0xe8))[0][1], 0x3d);
        assert!(e.get(cell, TESObjectCELL::bCellDetached));
    }

    #[test]
    fn detach_skips_terrain_update_and_changes_when_the_state_object_and_form_say_so() {
        let mut e = engine();
        let cell = detach_setup(&mut e, 6);
        let state_object: u32 = e.global(STATE_OBJECT_SINGLETON);
        e.mem.set_u8(state_object + 0x26, 1);
        returns(&mut e, FORM_HAS_CHANGES, 0);
        returns(&mut e, CELL_IS_INTERIOR, 1);
        start_log(&mut e);
        e.call(0x0055_2bd0, &args![cell, 0u32]);
        let seen = addresses(&calls(&e));
        assert!(seen.contains(&SCENE_ATTACH_INTERIOR));
        assert!(!seen.contains(&SCENE_ATTACH_EXTERIOR));
        assert!(!seen.contains(&CELL_SET_TERRAIN_UPDATE));
        assert!(!seen.contains(&REMOVE_CHANGES));
    }

    #[test]
    fn detach_does_nothing_for_other_states_and_detaches_havok_on_request() {
        let mut e = engine();
        let cell = detach_setup(&mut e, 4);
        start_log(&mut e);
        e.call(0x0055_2bd0, &args![cell, 1u32]);
        assert_eq!(
            addresses(&calls(&e)),
            vec![CELL_LOCK_ENTER, CELL_GET_STATE, CELL_LOCK_LEAVE]
        );
        // State 5 with `detach_havok`: the interior test of DetachHavok shows
        // up between the scene detach and the add-on nodes.
        returns(&mut e, CELL_GET_STATE, 5);
        returns(&mut e, CELL_HAVOK_WORLD, 0);
        returns(&mut e, CELL_COLLISION_WORLD, 0);
        quiet(&mut e, &[CELL_REFERENCES]);
        let state_object = e.mem.alloc(0x40);
        e.set_global(STATE_OBJECT_SINGLETON, state_object);
        start_log(&mut e);
        e.call(0x0055_2bd0, &args![cell, 1u32]);
        let seen = addresses(&calls(&e));
        let at = seen
            .iter()
            .position(|a| *a == CELL_COLLISION_WORLD)
            .unwrap();
        assert_eq!(seen[at - 1], CELL_IS_INTERIOR);
        assert_eq!(seen[at + 1], CELL_LOCK_ENTER);
    }

    #[test]
    fn vector4_constructor_stores_the_floats_in_order() {
        let mut e = engine();
        let at = e.mem.alloc(16);
        let result = e.call(0x0055_32a0, &args![at, 1.0f32, 2.0f32, -3.5f32, 4.25f32]);
        assert_eq!(result.u32(), at);
        let words: Vec<f32> = (0..4).map(|i| e.mem.f32(at + 4 * i)).collect();
        assert_eq!(words, vec![1.0, 2.0, -3.5, 4.25]);
    }

    fn manager_doubles(e: &mut Engine) {
        returns(e, MEMORY_MANAGER_GET, 0x44);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            assert_eq!(a[0], 0x44);
            e.mem.alloc(a[1]).into_ret()
        });
        e.register(MEMORY_MANAGER_DEALLOCATE, |e, a| {
            assert_eq!(a[0], 0x44);
            e.mem.free(a[1]);
            Ret::default()
        });
    }

    #[test]
    fn aligned_allocation_aligns_and_remembers_the_distance() {
        let mut e = engine();
        manager_doubles(&mut e);
        start_log(&mut e);
        let aligned = e.call(0x0055_3320, &args![16u32, 40u32]).u32();
        assert_eq!(aligned % 16, 0);
        let requested = calls_to(&calls(&e), MEMORY_MANAGER_ALLOCATE);
        assert_eq!(requested, vec![vec![0x44, 56]]);
        let distance = e.mem.u8(aligned - 1) as u32;
        assert!(distance == 8 || distance == 16);
        // Another call: a block that is already aligned moves up by a whole
        // alignment.
        let second = e.call(0x0055_3300, &args![0xa0u32]).u32();
        assert_eq!(second % 16, 0);
        assert!(matches!(e.mem.u8(second - 1), 8 | 16));
        // An alignment of 4 with an 8-aligned block moves by 4.
        let third = e.call(0x0055_3320, &args![4u32, 8u32]).u32();
        assert_eq!(third % 4, 0);
        assert_eq!(e.mem.u8(third - 1), 4);
    }

    #[test]
    fn aligned_free_gives_back_the_original_block() {
        let mut e = engine();
        manager_doubles(&mut e);
        let aligned = e.call(0x0055_3320, &args![16u32, 24u32]).u32();
        let block = aligned - e.mem.u8(aligned - 1) as u32;
        start_log(&mut e);
        e.call(0x0055_3380, &args![aligned]);
        assert_eq!(
            calls_to(&calls(&e), MEMORY_MANAGER_DEALLOCATE),
            vec![vec![0x44, block]]
        );
        start_log(&mut e);
        e.call(0x0055_3380, &args![0u32]);
        assert!(calls(&e).is_empty());
    }

    /// A world holder whose object (the word at `+0`) is the first word.
    fn holder_doubles(e: &mut Engine) {
        e.register(HOLDER_GET_WORLD, |e, a| e.mem.u32(a[0] + 0x90).into_ret());
        quiet(
            e,
            &[
                EMPTY_FUNCTION,
                WORLD_SET_GRAVITY_VECTOR,
                WORLD_SET_FIELD_A,
                WORLD_SET_FIELD_B,
                HOLDER_FIELD_70_ADD,
            ],
        );
    }

    #[test]
    fn world_wrapper_calls_the_empty_function_on_the_world_object() {
        let mut e = engine();
        holder_doubles(&mut e);
        let holder = e.mem.alloc(0xb0);
        start_log(&mut e);
        e.call(0x0055_33c0, &args![holder]);
        assert_eq!(addresses(&calls(&e)), vec![HOLDER_GET_WORLD]);
        e.mem.set_u32(holder + 0x90, 0x7700);
        start_log(&mut e);
        e.call(0x0055_33c0, &args![holder]);
        assert_eq!(
            calls(&e),
            vec![
                (HOLDER_GET_WORLD, vec![holder]),
                (EMPTY_FUNCTION, vec![0x7700])
            ]
        );
    }

    #[test]
    fn world_wrapper_005533f0_sets_the_value_and_queues_its_address() {
        let mut e = engine();
        holder_doubles(&mut e);
        let holder = e.mem.alloc(0xb0);
        let seen = Rc::new(RefCell::new(0));
        let slot_value = seen.clone();
        e.register_double(HOLDER_FIELD_70_ADD, move |e, a| {
            *slot_value.borrow_mut() = e.mem.u32(a[1]);
            Ret::default()
        });
        start_log(&mut e);
        e.call(0x0055_33f0, &args![holder, 0x1234u32]);
        // Without a world object the value is not set but its address is
        // still queued (00553408 jumps to the 006299c0 call).
        let log = calls(&e);
        assert_eq!(addresses(&log), vec![HOLDER_GET_WORLD, HOLDER_FIELD_70_ADD]);
        assert_eq!(calls_to(&log, HOLDER_FIELD_70_ADD)[0][0], holder + 0x70);
        assert_eq!(*seen.borrow(), 0x1234);
        *seen.borrow_mut() = 0;
        e.mem.set_u32(holder + 0x90, 0x7700);
        start_log(&mut e);
        e.call(0x0055_33f0, &args![holder, 0x1234u32]);
        let log = calls(&e);
        assert_eq!(
            addresses(&log),
            vec![
                HOLDER_GET_WORLD,
                HOLDER_GET_WORLD,
                EMPTY_FUNCTION,
                WORLD_SET_GRAVITY_VECTOR,
                HOLDER_GET_WORLD,
                EMPTY_FUNCTION,
                HOLDER_FIELD_70_ADD
            ]
        );
        assert_eq!(
            calls_to(&log, WORLD_SET_GRAVITY_VECTOR),
            vec![vec![0x7700, 0x1234]]
        );
        assert_eq!(calls_to(&log, HOLDER_FIELD_70_ADD)[0][0], holder + 0x70);
        assert_eq!(*seen.borrow(), 0x1234);
    }

    #[test]
    fn world_wrapper_00553440_sets_a_field_between_two_empty_calls() {
        let mut e = engine();
        holder_doubles(&mut e);
        let holder = e.mem.alloc(0xb0);
        e.mem.set_u32(holder + 0x90, 0x7700);
        start_log(&mut e);
        e.call(0x0055_3440, &args![holder, 9u32]);
        let log = calls(&e);
        assert_eq!(
            addresses(&log),
            vec![
                HOLDER_GET_WORLD,
                HOLDER_GET_WORLD,
                EMPTY_FUNCTION,
                WORLD_SET_FIELD_A,
                HOLDER_GET_WORLD,
                EMPTY_FUNCTION
            ]
        );
        assert_eq!(calls_to(&log, WORLD_SET_FIELD_A), vec![vec![0x7700, 9]]);
        // Without a world object nothing is set.
        e.mem.set_u32(holder + 0x90, 0);
        start_log(&mut e);
        e.call(0x0055_3440, &args![holder, 9u32]);
        assert_eq!(addresses(&calls(&e)), vec![HOLDER_GET_WORLD]);
    }

    #[test]
    fn world_wrapper_00553480_sets_the_inner_value_and_stores_it() {
        let mut e = engine();
        holder_doubles(&mut e);
        let holder = e.mem.alloc(0xb0);
        e.mem.set_u32(holder + 0x90, 0x7700);
        start_log(&mut e);
        e.call(0x0055_3480, &args![holder, 0x1000u32]);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, WORLD_SET_GRAVITY_VECTOR),
            vec![vec![0x7700, 0x1000]]
        );
        assert_eq!(
            calls_to(&log, WORLD_SET_FIELD_B),
            vec![vec![0x7700, 0x1014]]
        );
        assert_eq!(e.mem.u32(holder + 0x20), 0x1000);
        // A null value gives a null inner value.
        start_log(&mut e);
        e.call(0x0055_3480, &args![holder, 0u32]);
        assert_eq!(
            calls_to(&calls(&e), WORLD_SET_FIELD_B),
            vec![vec![0x7700, 0]]
        );
        assert_eq!(e.mem.u32(holder + 0x20), 0);
    }

    #[test]
    fn world_wrapper_005534f0_stores_the_value_at_0x28() {
        let mut e = engine();
        holder_doubles(&mut e);
        let holder = e.mem.alloc(0xb0);
        e.call(0x0055_34f0, &args![holder, 0x4321u32]);
        assert_eq!(e.mem.u32(holder + 0x28), 0x4321);
    }

    #[test]
    fn cinfo_destructor_runs_the_member_and_base_destructors() {
        let mut e = engine();
        quiet(
            &mut e,
            &[CINFO_MEMBER_DESTRUCT, CINFO_BASE_DESTRUCT, CINFO_FREE],
        );
        let cinfo = e.mem.alloc(0xf0);
        start_log(&mut e);
        e.call(0x0055_3540, &args![cinfo]);
        let log = calls(&e);
        assert_eq!(
            log,
            vec![
                (CINFO_MEMBER_DESTRUCT, vec![cinfo + 0x6c]),
                (CINFO_MEMBER_DESTRUCT, vec![cinfo + 0x5c]),
                (CINFO_MEMBER_DESTRUCT, vec![cinfo + 0x58]),
                (CINFO_BASE_DESTRUCT, vec![cinfo]),
            ]
        );
        assert_eq!(e.mem.u32(cinfo), CINFO_VTABLE);
        // The 00553520 wrapper runs the same destructor.
        start_log(&mut e);
        e.call(0x0055_3520, &args![cinfo]);
        assert_eq!(calls(&e).len(), 4);
    }

    #[test]
    fn cinfo_deleting_destructor_frees_only_when_asked() {
        let mut e = engine();
        quiet(
            &mut e,
            &[CINFO_MEMBER_DESTRUCT, CINFO_BASE_DESTRUCT, CINFO_FREE],
        );
        let cinfo = e.mem.alloc(0xf0);
        start_log(&mut e);
        assert_eq!(e.call(0x0055_35c0, &args![cinfo, 0u32]).u32(), cinfo);
        assert!(calls_to(&calls(&e), CINFO_FREE).is_empty());
        start_log(&mut e);
        assert_eq!(e.call(0x0055_35c0, &args![cinfo, 3u32]).u32(), cinfo);
        assert_eq!(calls_to(&calls(&e), CINFO_FREE), vec![vec![cinfo]]);
    }

    /// What the interior-world creation builds, noted by the doubles.
    #[derive(Default)]
    struct WorldTrace {
        /// `(size, address)` of every `operator new`.
        allocations: Vec<(u32, u32)>,
        /// The gravity vector given to the cinfo.
        gravity: Vec<f32>,
        /// The scale stored in the cinfo when the world was built.
        scale: f32,
        cinfo: u32,
    }

    /// Doubles for `fn_00552dc0`: a cell that is interior and has no Havok
    /// world and no collision world; the world is made of fresh blocks.
    fn world_creation_setup(e: &mut Engine) -> (Ptr<TESObjectCELL>, Rc<RefCell<WorldTrace>>) {
        guard_doubles(e);
        manager_doubles(e);
        let trace = Rc::new(RefCell::new(WorldTrace::default()));
        let cell = e.new_object::<TESObjectCELL>();
        quiet(
            e,
            &[
                CINFO_CONSTRUCT,
                CINFO_MEMBER_DESTRUCT,
                CINFO_BASE_DESTRUCT,
                EMPTY_FUNCTION,
                WORLD_SET_GRAVITY_VECTOR,
                WORLD_SET_FIELD_A,
                WORLD_SET_FIELD_B,
                HOLDER_FIELD_70_ADD,
                WORLD_SET_GRAVITY_SCALE,
                LISTENER_REGISTER,
                LIST_NODE_ITEM_ADDRESS,
                CELL_REFERENCE_CENTER,
                WORLD_SET_CENTER,
                WORLD_SET_VISUAL_DEBUGGER,
                WORLD_SET_VISUAL_DEBUGGER_ENABLED,
                EXTRA_LIST_SET_HAVOK_WORLD,
                EXTRA_LIST_SET_COLLISION_WORLD,
                CINFO_SET_BROAD_PHASE_SIZE,
                fake(0xc4),
            ],
        );
        returns(e, CELL_HAVOK_WORLD, 0);
        returns(e, CELL_IS_INTERIOR, 1);
        returns(e, CELL_COLLISION_WORLD, 0);
        returns(e, HAS_WATER_TEST, 0);
        returns(e, SCENE_HAVOK_SETTING, 0x42);
        returns(e, VISUAL_DEBUGGER_SETTING, 1);
        returns(e, VISUAL_DEBUGGER_SUPPRESSED, 1);
        e.register(HOLDER_GET_WORLD, |_, a| a[0].into_ret());
        let noted = trace.clone();
        e.register_double(OPERATOR_NEW, move |e, a| {
            let block = e.mem.alloc(a[0]);
            noted.borrow_mut().allocations.push((a[0], block));
            block.into_ret()
        });
        e.register(ALLOCATE_CDECL, |e, a| e.mem.alloc(a[0]).into_ret());
        for constructor in [
            BROAD_PHASE_CONSTRUCT,
            FILTER_CONSTRUCT,
            LISTENER_OBJECT_CONSTRUCT,
            LISTENER_CONSTRUCT,
            COLLISION_WORLD_CONSTRUCT,
        ] {
            e.register(constructor, |_, a| a[0].into_ret());
        }
        e.register(WATER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        let noted = trace.clone();
        e.register_double(CINFO_SET_GRAVITY, move |e, a| {
            noted.borrow_mut().gravity = (0..4).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
            Ret::default()
        });
        let noted = trace.clone();
        e.register_double(WORLD_CONSTRUCT, move |e, a| {
            let mut trace = noted.borrow_mut();
            trace.scale = e.mem.f32(a[1] + 0x54);
            trace.cinfo = a[1];
            // The world object is the raw block; give it a vtable for the
            // virtual call at +0xc4.
            e.mem.set_u32(a[0], TEST_VTABLE);
            a[0].into_ret()
        });
        e.set_global(GRAVITY_Z, -9.5f32);
        e.set_global(BROAD_PHASE_SIZE, 3250.0f32);
        (cell, trace)
    }

    #[test]
    fn interior_world_is_built_from_its_parts_and_stored_in_the_extra_data() {
        let mut e = engine();
        let (cell, trace) = world_creation_setup(&mut e);
        returns(&mut e, HAS_WATER_TEST, 1);
        e.register(CELL_GET_WATER_HEIGHT, |_, _| 12.5f32.into_ret());
        start_log(&mut e);
        e.call(0x0055_2dc0, &args![cell]);
        let log = calls(&e);
        let trace = trace.borrow();
        // The cinfo: gravity (0, 0, g, 0) and scale 1.0 when the world is
        // constructed.
        assert_eq!(trace.gravity, vec![0.0, 0.0, -9.5, 0.0]);
        assert_eq!(trace.scale, 1.0);
        assert_eq!(
            calls_to(&log, CINFO_SET_BROAD_PHASE_SIZE),
            vec![vec![trace.cinfo, 3250.0f32.to_bits()]]
        );
        // The world was allocated as 0xa0 aligned bytes.
        let world = calls_to(&log, WORLD_CONSTRUCT)[0][0];
        assert_eq!(world % 16, 0);
        assert_eq!(
            calls_to(&log, MEMORY_MANAGER_ALLOCATE),
            vec![vec![0x44, 0xa0 + 0x10]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_LIST_SET_HAVOK_WORLD),
            vec![vec![cell.addr() + 0x28, world]]
        );
        // Broad phase (0x24 bytes, stored at +8 inside), filter (0x28), water
        // (0x40, built from the cell's water height), listener object (0x14).
        let sizes: Vec<u32> = trace.allocations.iter().map(|(size, _)| *size).collect();
        assert_eq!(sizes, vec![0x24, 0x28, 0x40, 0x14, 8]);
        let block = |size: u32| {
            trace
                .allocations
                .iter()
                .find(|(s, _)| *s == size)
                .unwrap()
                .1
        };
        assert_eq!(e.mem.u32(world + 0x20), block(0x24) + 8);
        assert_eq!(e.mem.u32(world + 0x28), block(0x28));
        assert_eq!(
            calls_to(&log, WATER_CONSTRUCT),
            vec![vec![block(0x40), 12.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, WORLD_SET_GRAVITY_SCALE),
            vec![vec![world, block(0x40)]]
        );
        assert_eq!(calls_to(&log, WORLD_SET_FIELD_A), vec![vec![world, 0x42]]);
        // The listener singleton is created once and registered with the
        // world holder's object.
        let singleton: u32 = e.global(LISTENER_SINGLETON);
        assert_eq!(singleton, block(8));
        assert_eq!(
            calls_to(&log, LISTENER_REGISTER),
            vec![vec![singleton, world]]
        );
        // Visual debugger, virtual +0xc4, and the destruction of the cinfo.
        assert_eq!(
            calls_to(&log, WORLD_SET_VISUAL_DEBUGGER),
            vec![vec![world, 1]]
        );
        assert_eq!(
            calls_to(&log, WORLD_SET_VISUAL_DEBUGGER_ENABLED),
            vec![vec![world, 0]]
        );
        assert_eq!(calls_to(&log, fake(0xc4)).len(), 1);
        assert_eq!(calls_to(&log, CINFO_BASE_DESTRUCT), vec![vec![trace.cinfo]]);
        // The collision world (0x18 bytes) is created in a second guard.
        let guards = calls_to(&log, SCOPE_GUARD_CONSTRUCT);
        assert_eq!(guards.len(), 2);
        assert_eq!(&guards[0][1..], &[0x10, 1, SOURCE_FILE_NAME, 0x3a3f]);
        assert_eq!(&guards[1][1..], &[0x10, 1, SOURCE_FILE_NAME, 0x3a91]);
        let collision = calls_to(&log, COLLISION_WORLD_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, EXTRA_LIST_SET_COLLISION_WORLD),
            vec![vec![cell.addr() + 0x28, collision]]
        );
    }

    #[test]
    fn interior_world_without_water_and_with_an_existing_listener_singleton() {
        let mut e = engine();
        let (cell, trace) = world_creation_setup(&mut e);
        e.set_global(LISTENER_SINGLETON, 0x1357u32);
        returns(&mut e, CELL_COLLISION_WORLD, 0x777);
        start_log(&mut e);
        e.call(0x0055_2dc0, &args![cell]);
        let log = calls(&e);
        assert!(calls_to(&log, WATER_CONSTRUCT).is_empty());
        assert!(calls_to(&log, WORLD_SET_GRAVITY_SCALE).is_empty());
        let sizes: Vec<u32> = trace
            .borrow()
            .allocations
            .iter()
            .map(|(size, _)| *size)
            .collect();
        assert_eq!(sizes, vec![0x24, 0x28, 0x14]);
        assert_eq!(e.global::<u32>(LISTENER_SINGLETON), 0x1357);
        // A collision world exists: only one guard.
        assert_eq!(calls_to(&log, SCOPE_GUARD_CONSTRUCT).len(), 1);
        assert!(calls_to(&log, EXTRA_LIST_SET_COLLISION_WORLD).is_empty());
    }

    #[test]
    fn world_creation_is_skipped_for_exterior_cells_and_cells_with_a_world() {
        let mut e = engine();
        let (cell, _trace) = world_creation_setup(&mut e);
        returns(&mut e, CELL_COLLISION_WORLD, 0x777);
        returns(&mut e, CELL_IS_INTERIOR, 0);
        start_log(&mut e);
        e.call(0x0055_2dc0, &args![cell]);
        assert_eq!(
            addresses(&calls(&e)),
            vec![CELL_HAVOK_WORLD, CELL_IS_INTERIOR, CELL_COLLISION_WORLD]
        );
        returns(&mut e, CELL_IS_INTERIOR, 1);
        returns(&mut e, CELL_HAVOK_WORLD, 0x555);
        start_log(&mut e);
        e.call(0x0055_2dc0, &args![cell]);
        assert_eq!(
            addresses(&calls(&e)),
            vec![CELL_HAVOK_WORLD, CELL_IS_INTERIOR, CELL_COLLISION_WORLD]
        );
    }

    #[test]
    fn exterior_terrain_transform_is_placed_at_the_cell_centre() {
        let mut e = engine();
        guard_doubles(&mut e);
        let cell = e.new_object::<TESObjectCELL>();
        returns(&mut e, CELL_HAVOK_WORLD, 0x5511);
        returns(&mut e, CELL_COLLISION_WORLD, 0x6622);
        returns(&mut e, CELL_GET_DATA_X, 3);
        returns(&mut e, CELL_GET_DATA_Y, -2i32 as u32);
        quiet(&mut e, &[TRANSFORM_CONSTRUCT]);
        e.set_global(HALF_CELL_SIZE, 2048.0f64);
        for i in 0..9u32 {
            e.set_global(IDENTITY_ROTATION + 4 * i, 0x3f80_0000 + i);
        }
        // The double reads the transform at the call.
        let seen = Rc::new(RefCell::new(Vec::new()));
        let noted = seen.clone();
        e.register_double(WORLD_ADD_TERRAIN, move |e, a| {
            let translation = a[2];
            let mut words: Vec<u32> = (0..9)
                .map(|i| e.mem.u32(translation - 0x24 + 4 * i))
                .collect();
            words.extend((0..3).map(|i| e.mem.f32(translation + 4 * i).to_bits()));
            words.push(e.mem.f32(translation + 12).to_bits());
            *noted.borrow_mut() = words;
            Ret::default()
        });
        start_log(&mut e);
        e.call(0x0055_35f0, &args![cell]);
        let log = calls(&e);
        let terrain = calls_to(&log, WORLD_ADD_TERRAIN);
        assert_eq!(terrain.len(), 1);
        assert_eq!(terrain[0][0], 0x6622);
        assert_eq!(terrain[0][1], 0x5511);
        assert_eq!(terrain[0][3], TERRAIN_FLAGS);
        let words = seen.borrow().clone();
        assert_eq!(
            &words[..9],
            &(0..9).map(|i| 0x3f80_0000 + i).collect::<Vec<u32>>()[..]
        );
        assert_eq!(f32::from_bits(words[9]), 14336.0);
        assert_eq!(f32::from_bits(words[10]), -6144.0);
        assert_eq!(f32::from_bits(words[11]), 0.0);
        assert_eq!(f32::from_bits(words[12]), 1.0);
        let guard = calls_to(&log, SCOPE_GUARD_CONSTRUCT);
        assert_eq!(&guard[0][1..], &[0x10, 1, SOURCE_FILE_NAME, 0x3aa6]);
        // Without a collision world (or a Havok world) nothing is built.
        returns(&mut e, CELL_COLLISION_WORLD, 0);
        start_log(&mut e);
        e.call(0x0055_35f0, &args![cell]);
        assert!(calls_to(&calls(&e), TRANSFORM_CONSTRUCT).is_empty());
        returns(&mut e, CELL_COLLISION_WORLD, 0x6622);
        returns(&mut e, CELL_HAVOK_WORLD, 0);
        start_log(&mut e);
        e.call(0x0055_35f0, &args![cell]);
        assert!(calls_to(&calls(&e), TRANSFORM_CONSTRUCT).is_empty());
    }

    #[test]
    fn detach_havok_of_an_interior_cell_switches_the_debugger_off_and_releases_the_object() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        returns(&mut e, CELL_IS_INTERIOR, 1);
        returns(&mut e, CELL_HAVOK_WORLD, 0x5511);
        returns(&mut e, WORLD_COLLISION_OBJECT, 0x8800);
        quiet(
            &mut e,
            &[
                WORLD_SET_VISUAL_DEBUGGER_ENABLED,
                WORLD_SET_VISUAL_DEBUGGER,
                COLLISION_OBJECT_SET_ACTIVE,
                COLLISION_OBJECT_RELEASE,
            ],
        );
        start_log(&mut e);
        e.call(0x0055_3700, &args![cell]);
        let log = calls(&e);
        assert_eq!(
            addresses(&log),
            vec![
                CELL_IS_INTERIOR,
                CELL_HAVOK_WORLD,
                WORLD_SET_VISUAL_DEBUGGER_ENABLED,
                WORLD_SET_VISUAL_DEBUGGER,
                WORLD_COLLISION_OBJECT,
                WORLD_COLLISION_OBJECT,
                COLLISION_OBJECT_SET_ACTIVE,
                COLLISION_OBJECT_RELEASE,
            ]
        );
        assert_eq!(
            calls_to(&log, COLLISION_OBJECT_SET_ACTIVE),
            vec![vec![0x8800, 1]]
        );
        assert_eq!(
            calls_to(&log, COLLISION_OBJECT_RELEASE),
            vec![vec![0x8800, 0]]
        );
        assert_eq!(
            calls_to(&log, WORLD_SET_VISUAL_DEBUGGER_ENABLED),
            vec![vec![0x5511, 0]]
        );
        // No world: nothing else happens.
        returns(&mut e, CELL_HAVOK_WORLD, 0);
        start_log(&mut e);
        e.call(0x0055_3700, &args![cell]);
        assert_eq!(
            addresses(&calls(&e)),
            vec![CELL_IS_INTERIOR, CELL_HAVOK_WORLD]
        );
    }

    #[test]
    fn detach_havok_of_an_exterior_cell_detaches_every_reference() {
        let mut e = engine();
        list_doubles(&mut e);
        let cell = e.new_object::<TESObjectCELL>();
        let first = object(&mut e, 0x40);
        let second = object(&mut e, 0x40);
        let list = list_of(&mut e, &[first, 0, second]);
        let state_object = e.mem.alloc(0x40);
        e.mem.set_u8(state_object + 0x26, 1);
        e.set_global(STATE_OBJECT_SINGLETON, state_object);
        returns(&mut e, CELL_IS_INTERIOR, 0);
        returns(&mut e, CELL_COLLISION_WORLD, 0x6622);
        returns(&mut e, CELL_REFERENCES, list);
        returns(&mut e, CELL_HAVOK_WORLD, 0x5511);
        returns(&mut e, WORLD_COLLISION_OBJECT, 0x8800);
        quiet(
            &mut e,
            &[
                COLLISION_WORLD_UNLOAD,
                CELL_LOCK_ENTER,
                CELL_LOCK_LEAVE,
                COLLISION_OBJECT_RELEASE,
                WORLD_UPDATE_RENDERER,
                fake(0x1c0),
            ],
        );
        start_log(&mut e);
        e.call(0x0055_3700, &args![cell]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, COLLISION_WORLD_UNLOAD), vec![vec![0x6622]]);
        // The empty item in the middle is skipped, the others are detached.
        let detached: Vec<u32> = calls_to(&log, fake(0x1c0)).iter().map(|c| c[0]).collect();
        assert_eq!(detached, vec![first, second]);
        assert_eq!(
            calls_to(&log, COLLISION_OBJECT_RELEASE),
            vec![vec![0x8800, 0]]
        );
        assert_eq!(calls_to(&log, WORLD_UPDATE_RENDERER), vec![vec![0x5511, 0]]);
        assert_eq!(*addresses(&log).first().unwrap(), CELL_IS_INTERIOR);
        assert_eq!(*addresses(&log).last().unwrap(), CELL_LOCK_LEAVE);
        // Without collision world, Havok world and renderer flag.
        returns(&mut e, CELL_COLLISION_WORLD, 0);
        returns(&mut e, CELL_HAVOK_WORLD, 0);
        e.mem.set_u8(state_object + 0x26, 0);
        start_log(&mut e);
        e.call(0x0055_3700, &args![cell]);
        let seen = addresses(&calls(&e));
        assert!(!seen.contains(&COLLISION_WORLD_UNLOAD));
        assert!(!seen.contains(&COLLISION_OBJECT_RELEASE));
        assert!(!seen.contains(&WORLD_UPDATE_RENDERER));
    }

    #[test]
    fn animated_references_are_asked_and_the_not_excluded_ones_notified() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        let loaded = e.new_object::<LoadedCellData>();
        e.set(cell, TESObjectCELL::pLoadedData, loaded.cast());
        // Three references: one answers true to +0x100; one has no base
        // object; one has form type 0x25; the last one is notified.
        let refs: Vec<u32> = (0..4).map(|_| object(&mut e, 0x40)).collect();
        e.mem.set_u32(refs[0] + 0x30, 1);
        e.mem.set_u32(refs[1] + 0x20, 0);
        let excluded_form = e.mem.alloc(0x10);
        let notified_form = e.mem.alloc(0x10);
        e.mem.set_u8(excluded_form + 4, 0x25);
        e.mem.set_u8(notified_form + 4, 0x10);
        e.mem.set_u32(refs[2] + 0x20, excluded_form);
        e.mem.set_u32(refs[3] + 0x20, notified_form);
        returns(&mut e, CELL_IS_LOADED, 1);
        returns(&mut e, MAP_FIRST_POSITION, 1);
        e.register(fake(0x100), |e, a| e.mem.u32(a[0] + 0x30).into_ret());
        quiet(&mut e, &[fake(0x1e0)]);
        e.register(REFERENCE_BASE_OBJECT, |e, a| {
            e.mem.u32(a[0] + 0x20).into_ret()
        });
        // Form types: 0x25 excludes the reference.
        e.register(FORM_TYPE, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        let queue = Rc::new(RefCell::new(
            refs.iter().rev().copied().collect::<Vec<u32>>(),
        ));
        let source = queue.clone();
        e.register_double(MAP_NEXT, move |e, a| {
            let reference = source.borrow_mut().pop().unwrap();
            e.mem.set_u32(a[2], reference);
            e.mem.set_u32(a[3], 0x3333);
            if source.borrow().is_empty() {
                e.mem.set_u32(a[1], 0);
            }
            Ret::default()
        });
        start_log(&mut e);
        e.call(0x0055_3820, &args![cell]);
        let log = calls(&e);
        let notified: Vec<u32> = calls_to(&log, fake(0x1e0)).iter().map(|c| c[0]).collect();
        assert_eq!(notified, vec![refs[3]]);
        // Not loaded or without loaded data: nothing is walked.
        returns(&mut e, CELL_IS_LOADED, 0);
        start_log(&mut e);
        e.call(0x0055_3820, &args![cell]);
        assert_eq!(addresses(&calls(&e)), vec![CELL_IS_LOADED]);
        returns(&mut e, CELL_IS_LOADED, 1);
        e.set(cell, TESObjectCELL::pLoadedData, Ptr::NULL);
        start_log(&mut e);
        e.call(0x0055_3820, &args![cell]);
        assert_eq!(addresses(&calls(&e)), vec![CELL_IS_LOADED]);
    }

    #[test]
    fn sound_chance_and_range_accessors() {
        let mut e = engine();
        let form = e.mem.alloc(0x80);
        e.mem.set_i8(form + 0x68, -5);
        assert_eq!(e.call(0x0055_3b60, &args![form]).u8() as i8, 0);
        e.mem.set_u32(form + 0x48, 2);
        assert_eq!(e.call(0x0055_3b60, &args![form]).u8() as i8, -5);
        e.mem.set_u32(form + 0x48, 5);
        assert_eq!(e.call(0x0055_3b60, &args![form]).u8() as i8, 0);
        e.mem.set_u8(form + 0x45, 9);
        assert_eq!(e.call(0x0055_3b90, &args![form]).u8(), 9);
    }

    #[test]
    fn sound_lookup_flag_needs_the_holder_and_bit_1() {
        let mut e = engine();
        let object = e.mem.alloc(0x40);
        let record = e.mem.alloc(0x10);
        e.register_double(SOUND_LOOKUP, move |_, a| {
            assert_eq!(a[0], record);
            record.into_ret()
        });
        assert!(!e.call(0x0055_3bb0, &args![object]).bool());
        e.mem.set_u32(object + 0x1c, record);
        assert!(!e.call(0x0055_3bb0, &args![object]).bool());
        e.mem.set_u32(record + 4, 3);
        assert!(e.call(0x0055_3bb0, &args![object]).bool());
        e.mem.set_u32(record + 4, 1);
        assert!(!e.call(0x0055_3bb0, &args![object]).bool());
    }

    /// A form with the sound fields the walk reads.
    fn sound_form(e: &mut Engine, chance: i8, range: u8) -> u32 {
        let form = e.mem.alloc(0x80);
        let name = e.mem.alloc(8);
        e.mem.set_u8(name, b'x');
        e.mem.set_u32(form + 0x48, 2);
        e.mem.set_i8(form + 0x68, chance);
        e.mem.set_u8(form + 0x45, range);
        e.mem.set_u32(form + 0x70, name);
        e.mem.set_u32(form + 0x0c, 0xf0f0);
        form
    }

    /// A reference that qualifies for a sound at distance `distance`.
    fn sound_reference(e: &mut Engine, form: u32, distance: f32) -> u32 {
        let reference = object(e, 0x80);
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record + 4, 2);
        e.mem.set_u32(reference + 0x1c, record);
        e.mem.set_u32(reference + 0x30, form);
        e.mem.set_f32(reference + 0x34, distance);
        e.mem.set_u32(reference + 0x38, 0x3d3d);
        e.mem.set_f32(reference + 0x40, 1.0);
        e.mem.set_f32(reference + 0x44, 2.0);
        e.mem.set_f32(reference + 0x48, 3.0);
        reference
    }

    /// Doubles for `UpdateRefSounds`; the reference words: `+0x20` disabled,
    /// `+0x24` destroyed, `+0x28`/`+0x2c` the virtual answers.
    fn sound_setup(e: &mut Engine, items: &[u32]) -> Ptr<TESObjectCELL> {
        list_doubles(e);
        let cell = e.new_object::<TESObjectCELL>();
        let list = list_of(e, items);
        quiet(
            e,
            &[
                CELL_LOCK_ENTER,
                CELL_LOCK_LEAVE,
                AUDIO_GET_SOUND_HANDLE,
                SOUND_HANDLE_FOLLOW,
                SOUND_HANDLE_SET_POSITION,
                SOUND_HANDLE_SET_PRIORITY,
                SOUND_HANDLE_PLAY,
                EMPTY_FUNCTION,
            ],
        );
        returns(e, CELL_IS_LOADED, 1);
        returns(e, CELL_REFERENCES, list);
        returns(e, AUDIO_GET, 0xa0d0);
        e.register(REFERENCE_IS_DISABLED, |e, a| {
            e.mem.u32(a[0] + 0x20).into_ret()
        });
        e.register(REFERENCE_IS_DESTROYED, |e, a| {
            e.mem.u32(a[0] + 0x24).into_ret()
        });
        e.register(fake(0x21c), |e, a| e.mem.u32(a[0] + 0x28).into_ret());
        e.register(fake(0x22c), |e, a| e.mem.u32(a[0] + 0x2c).into_ret());
        e.register(fake(0x1d0), |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        e.register(fake(0x1f4), |_, a| (a[0] + 0x40).into_ret());
        e.register(REFERENCE_BASE_FORM, |e, a| {
            e.mem.u32(a[0] + 0x30).into_ret()
        });
        e.register(FORM_SOUND_NAME, |e, a| e.mem.u32(a[0] + 0x70).into_ret());
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0x0c).into_ret());
        e.register(SOUND_LOOKUP, |_, a| a[0].into_ret());
        e.register(SOUND_CHANCE_TEST, |_, a| {
            u32::from(f32::from_bits(a[0]) > 0.5).into_ret()
        });
        e.register(REFERENCE_DISTANCE, |e, a| e.mem.f32(a[1] + 0x34).into_ret());
        e.set_global(SOUND_CHANCE_DIVISOR, 100.0f64);
        e.set_global(DISTANCE_SOURCE_SINGLETON, 0xd157u32);
        cell
    }

    #[test]
    fn ref_sounds_start_the_sound_of_a_close_reference_with_a_chance() {
        let mut e = engine();
        let form = sound_form(&mut e, 100, 3);
        let close = sound_reference(&mut e, form, 299.9);
        let disabled = sound_reference(&mut e, form, 1.0);
        e.mem.set_u32(disabled + 0x20, 1);
        let destroyed = sound_reference(&mut e, form, 1.0);
        e.mem.set_u32(destroyed + 0x24, 1);
        let unlikely_form = sound_form(&mut e, 10, 3);
        let unlikely = sound_reference(&mut e, unlikely_form, 1.0);
        let far = sound_reference(&mut e, form, 350.5);
        let after_the_end = sound_reference(&mut e, form, 1.0);
        let cell = sound_setup(
            &mut e,
            &[disabled, destroyed, unlikely, far, close, 0, after_the_end],
        );
        start_log(&mut e);
        e.call(0x0055_38e0, &args![cell]);
        let log = calls(&e);
        // Only the close reference starts a sound: form id and flags 0x2102.
        let started = calls_to(&log, AUDIO_GET_SOUND_HANDLE);
        assert_eq!(started.len(), 1);
        assert_eq!(started[0][0], 0xa0d0);
        assert_eq!(&started[0][2..], &[0xf0f0, 0x2102]);
        let handle = started[0][1];
        // It follows the reference's 3D, is placed at the reference's
        // position, gets priority 0xc0, is played and released.
        assert_eq!(
            calls_to(&log, SOUND_HANDLE_FOLLOW),
            vec![vec![handle, 0x3d3d]]
        );
        assert_eq!(
            calls_to(&log, SOUND_HANDLE_SET_POSITION),
            vec![vec![
                handle,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits()
            ]]
        );
        assert_eq!(
            calls_to(&log, SOUND_HANDLE_SET_PRIORITY),
            vec![vec![handle, 0xc0]]
        );
        assert_eq!(calls_to(&log, SOUND_HANDLE_PLAY), vec![vec![handle, 0]]);
        assert_eq!(calls_to(&log, EMPTY_FUNCTION), vec![vec![handle]]);
        // The chance given to the test is chance / divisor; the references
        // after the empty node are never looked at.
        let chances: Vec<f32> = calls_to(&log, SOUND_CHANCE_TEST)
            .iter()
            .map(|c| f32::from_bits(c[0]))
            .collect();
        assert_eq!(chances, vec![0.1, 1.0, 1.0]);
        assert_eq!(*addresses(&log).first().unwrap(), CELL_LOCK_ENTER);
        assert_eq!(*addresses(&log).last().unwrap(), CELL_LOCK_LEAVE);
        assert!(calls_to(&log, REFERENCE_IS_DISABLED)
            .iter()
            .all(|c| c[0] != after_the_end));
    }

    #[test]
    fn ref_sounds_skip_references_the_virtuals_exclude() {
        let mut e = engine();
        let form = sound_form(&mut e, 100, 3);
        let excluded = sound_reference(&mut e, form, 1.0);
        e.mem.set_u32(excluded + 0x28, 1);
        e.mem.set_u32(excluded + 0x2c, 1);
        let allowed = sound_reference(&mut e, form, 1.0);
        e.mem.set_u32(allowed + 0x28, 1);
        let unflagged = sound_reference(&mut e, form, 1.0);
        e.mem.set_u32(unflagged + 0x1c, 0);
        let nameless_form = sound_form(&mut e, 100, 3);
        let name = e.mem.u32(nameless_form + 0x70);
        e.mem.set_u8(name, 0);
        let nameless = sound_reference(&mut e, nameless_form, 1.0);
        let formless = sound_reference(&mut e, 0, 1.0);
        let cell = sound_setup(&mut e, &[excluded, unflagged, nameless, formless, allowed]);
        start_log(&mut e);
        e.call(0x0055_38e0, &args![cell]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, AUDIO_GET_SOUND_HANDLE).len(), 1);
        // The excluded one was asked with argument 0 and no further.
        assert_eq!(calls_to(&log, fake(0x22c))[0], vec![excluded, 0]);
        // An unloaded cell does nothing but the lock.
        returns(&mut e, CELL_IS_LOADED, 0);
        start_log(&mut e);
        e.call(0x0055_38e0, &args![cell]);
        assert_eq!(
            addresses(&calls(&e)),
            vec![CELL_LOCK_ENTER, CELL_IS_LOADED, CELL_LOCK_LEAVE]
        );
    }

    #[test]
    fn truncation_to_the_low_word_follows_the_x87_rules() {
        assert_eq!(truncated_low_word(299.9), 299);
        assert_eq!(truncated_low_word(-1.5), 0xffff_ffff);
        assert_eq!(truncated_low_word(f32::NAN), 0);
        assert_eq!(truncated_low_word(1.0e30), 0);
        assert_eq!(truncated_low_word(4.0e9), 4_000_000_000);
    }

    /// Doubles for the ray tests: two reference nodes, the first without 3D.
    fn ray_setup(e: &mut Engine) -> (Ptr<TESObjectCELL>, u32) {
        list_doubles(e);
        let cell = e.new_object::<TESObjectCELL>();
        let without = object(e, 0x40);
        let with = object(e, 0x40);
        e.mem.set_u32(with + 0x20, 0x3d3d);
        let list = list_of(e, &[without, with]);
        returns(e, CELL_REFERENCE_NODES, list);
        e.register(fake(0x1d0), |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(RAY_TEST_BOUNDED, |_, a| {
            u32::from(a[4] == 0x3d3d).into_ret()
        });
        returns(e, NODE_POSITION_A, 0x11);
        returns(e, NODE_POSITION_B, 0x22);
        let position = e.mem.alloc(16);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        returns(e, NODE_POSITION_C, position);
        e.register(VECTOR_SUBTRACT, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            a[1].into_ret()
        });
        e.register(VECTOR_LENGTH, |e, a| e.mem.f32(a[0] + 8).into_ret());
        (cell, with)
    }

    #[test]
    fn ray_test_over_reference_nodes_checks_the_distance_to_the_node() {
        let mut e = engine();
        let (cell, with) = ray_setup(&mut e);
        let object_pointer = e.mem.alloc(0x40);
        start_log(&mut e);
        assert!(e
            .call(
                0x0055_3bf0,
                &args![cell, object_pointer, 7u32, 8u32, 10.0f32]
            )
            .bool());
        let log = calls(&e);
        // The bounded test got (object, a, b, limit, 3D) for the node with
        // a 3D only; the position chain starts at the object.
        assert_eq!(
            calls_to(&log, RAY_TEST_BOUNDED),
            vec![vec![object_pointer, 7, 8, 10.0f32.to_bits(), 0x3d3d]]
        );
        assert_eq!(calls_to(&log, NODE_POSITION_A), vec![vec![object_pointer]]);
        assert_eq!(calls_to(&log, NODE_POSITION_B), vec![vec![0x11, 0]]);
        assert_eq!(calls_to(&log, NODE_POSITION_C), vec![vec![0x22]]);
        // The subtraction takes `a` (the first word after the object).
        assert_eq!(calls_to(&log, VECTOR_SUBTRACT)[0][2], 7);
        // A limit below the length (3.0) fails; the list ends.
        start_log(&mut e);
        assert!(!e
            .call(
                0x0055_3bf0,
                &args![cell, object_pointer, 7u32, 8u32, 3.0f32]
            )
            .bool());
        // A null object and an empty list fail without a test.
        start_log(&mut e);
        assert!(!e
            .call(0x0055_3bf0, &args![cell, 0u32, 7u32, 8u32, 10.0f32])
            .bool());
        assert!(calls_to(&calls(&e), RAY_TEST_BOUNDED).is_empty());
        let empty = e.mem.alloc(8);
        returns(&mut e, CELL_REFERENCE_NODES, empty);
        start_log(&mut e);
        assert!(!e
            .call(
                0x0055_3bf0,
                &args![cell, object_pointer, 7u32, 8u32, 10.0f32]
            )
            .bool());
        assert!(calls_to(&calls(&e), RAY_TEST_BOUNDED).is_empty());
        let _ = with;
    }

    #[test]
    fn ray_test_over_reference_nodes_rejects_nodes_the_bounded_test_rejects() {
        let mut e = engine();
        let (cell, _with) = ray_setup(&mut e);
        e.register(RAY_TEST_BOUNDED, |_, _| Ret::default());
        let object_pointer = e.mem.alloc(0x40);
        start_log(&mut e);
        assert!(!e
            .call(
                0x0055_3bf0,
                &args![cell, object_pointer, 7u32, 8u32, 10.0f32]
            )
            .bool());
        assert!(calls_to(&calls(&e), NODE_POSITION_A).is_empty());
    }

    #[test]
    fn mask_selects_the_child_nodes_that_are_ray_tested() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        e.register(CELL_CHILD_NODE, |_, a| (0x1000 + a[1]).into_ret());
        e.register(RAY_TEST_NODE, |_, a| u32::from(a[4] == 0x1007).into_ret());
        let object_pointer = e.mem.alloc(0x40);
        let run = |e: &mut Engine, mask: u32| {
            start_log(e);
            let hit = e
                .call(
                    0x0055_3d00,
                    &args![cell, object_pointer, 7u32, 8u32, 10.0f32, mask],
                )
                .bool();
            let tested: Vec<u32> = calls_to(&calls(e), RAY_TEST_NODE)
                .iter()
                .map(|c| c[4] - 0x1000)
                .collect();
            (hit, tested)
        };
        assert_eq!(run(&mut e, 0x09), (true, vec![0, 3, 7]));
        assert_eq!(run(&mut e, 0x02), (false, vec![1]));
        assert_eq!(run(&mut e, 0x04), (false, vec![2]));
        assert_eq!(run(&mut e, 0x10), (true, vec![4, 7]));
        assert_eq!(run(&mut e, 0x01), (false, vec![0]));
        assert_eq!(run(&mut e, 0x1f), (true, vec![0, 1, 2, 3, 4, 7]));
        assert_eq!(run(&mut e, 0x00), (false, vec![]));
        // The arguments of a test: (object, a, b, limit, node, 0).
        start_log(&mut e);
        e.call(
            0x0055_3d00,
            &args![cell, object_pointer, 7u32, 8u32, 10.0f32, 2u32],
        );
        assert_eq!(
            calls_to(&calls(&e), RAY_TEST_NODE),
            vec![vec![object_pointer, 7, 8, 10.0f32.to_bits(), 0x1001, 0]]
        );
        // A null object never tests.
        start_log(&mut e);
        assert!(!e
            .call(0x0055_3d00, &args![cell, 0u32, 7u32, 8u32, 10.0f32, 1u32])
            .bool());
        assert!(calls_to(&calls(&e), RAY_TEST_NODE).is_empty());
    }

    #[test]
    fn pick_answers_the_cell_node_for_type_0x11_and_the_fallback_otherwise() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        let world = object(&mut e, 0x40);
        e.register(fake(0xc8), |e, a| e.mem.u32(a[0] + 0x30).into_ret());
        returns(&mut e, CELL_HAVOK_WORLD, 0);
        returns(&mut e, PICK_WORLD_SHAPE, 0x51);
        returns(&mut e, CELL_NODE_OF_CELL, 0xc311);
        returns(&mut e, PICK_FALLBACK_NODE, 0xfa11);
        e.register(PICK_FIND_ENTRY, |e, a| {
            assert_eq!(a[0], 0x51);
            e.mem.u32(0x7f00_0800).into_ret()
        });
        e.register(PICK_ENTRY_SOURCE, |_, a| a[1].into_ret());
        e.register(PICK_ENTRY_TYPE, |e, _| e.mem.u32(0x7f00_0804).into_ret());
        // No world.
        assert_eq!(e.call(0x0055_3ee0, &args![cell, 0x9999u32]).u32(), 0);
        // The world refuses the query.
        returns(&mut e, CELL_HAVOK_WORLD, world);
        assert_eq!(e.call(0x0055_3ee0, &args![cell, 0x9999u32]).u32(), 0);
        // The world accepts, the entry has the own-node type.
        e.mem.set_u32(world + 0x30, 1);
        e.mem.set_u32(0x7f00_0800, 0x77);
        e.mem.set_u32(0x7f00_0804, 0x11);
        assert_eq!(e.call(0x0055_3ee0, &args![cell, 0x9999u32]).u32(), 0xc311);
        // Another type, and no entry at all.
        e.mem.set_u32(0x7f00_0804, 0x12);
        assert_eq!(e.call(0x0055_3ee0, &args![cell, 0x9999u32]).u32(), 0xfa11);
        e.mem.set_u32(0x7f00_0800, 0);
        e.mem.set_u32(0x7f00_0804, 0x11);
        assert_eq!(e.call(0x0055_3ee0, &args![cell, 0x9999u32]).u32(), 0xfa11);
    }

    /// The entry's source is the frame's word (`EBP-0x10`): `0043b4f0` stores it there and
    /// answers its address, and `0043b4d0` reads the type through that address while the word
    /// is still there. With the freed-block check a word freed before the type test would read
    /// as the fill word.
    #[test]
    fn pick_reads_the_type_from_a_live_source_word() {
        let mut e = engine();
        e.mem.enable_freed_check(true);
        let cell = e.new_object::<TESObjectCELL>();
        let world = object(&mut e, 0x40);
        e.register(fake(0xc8), |_, _| 1u32.into_ret());
        returns(&mut e, CELL_HAVOK_WORLD, world);
        returns(&mut e, PICK_WORLD_SHAPE, 0x51);
        returns(&mut e, CELL_NODE_OF_CELL, 0xc311);
        returns(&mut e, PICK_FALLBACK_NODE, 0xfa11);
        let entry = e.mem.alloc(8);
        returns(&mut e, PICK_FIND_ENTRY, entry);
        e.register(PICK_ENTRY_SOURCE, |e, a| {
            let word = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], word);
            a[1].into_ret()
        });
        e.register(PICK_ENTRY_TYPE, |e, a| (e.mem.u32(a[0]) & 0x7f).into_ret());
        for (word, node) in [(0x91u32, 0xc311u32), (0x12, 0xfa11)] {
            e.mem.set_u32(entry, word);
            assert_eq!(e.call(0x0055_3ee0, &args![cell, 0x9999u32]).u32(), node);
        }
    }

    // -----------------------------------------------------------------------
    // Second batch (from `00553f70`).

    /// Argument words of recorded calls.
    type Hits = Rc<RefCell<Vec<Vec<u32>>>>;
    /// Recorded `(size, first bytes)` of data calls.
    type DataLog = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;
    /// Recorded `(section, x, y)` seen-bit queries.
    type BitQueries = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    /// Reads `count` floats at `at`.
    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|index| e.mem.f32(at + 4 * index)).collect()
    }

    /// Writes floats at `at`.
    fn put_floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * index as u32, *value);
        }
    }

    /// The component accessor `00560d30` for the vectors the tests build.
    fn component_double(e: &mut Engine) {
        e.register(VECTOR_COMPONENT, |_, a| (a[0] + 4 * a[1]).into_ret());
    }

    /// A reference-like object: the test vtable and room for the fields the
    /// doubles read (`+0x34` slot `0x1d0`, `+0x38` slot `0x100`, `+0x3c` the
    /// answer of virtual `+0x14`).
    fn reference_object(e: &mut Engine) -> u32 {
        let at = object(e, 0x60);
        e.register(fake(0x1d0), |e, a| e.mem.u32(a[0] + 0x34).into_ret());
        e.register(fake(0x100), |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        e.register(fake(0x14), |e, a| e.mem.u32(a[0] + 0x3c).into_ret());
        at
    }

    #[test]
    fn vector_addition_adds_the_four_components() {
        let mut e = engine();
        let a = e.mem.alloc(16);
        let b = e.mem.alloc(16);
        put_floats(&mut e, a, &[1.0, 2.0, 3.0, 4.0]);
        put_floats(&mut e, b, &[0.5, -2.0, 10.0, 0.25]);
        e.call(0x0055_3f70, &args![a, b]);
        assert_eq!(floats(&e, a, 4), vec![1.5, 0.0, 13.0, 4.25]);
        assert_eq!(floats(&e, b, 4), vec![0.5, -2.0, 10.0, 0.25]);
    }

    #[test]
    fn vector_copy_stores_three_components_and_a_zero() {
        let mut e = engine();
        component_double(&mut e);
        let destination = e.mem.alloc(16);
        let source = e.mem.alloc(16);
        put_floats(&mut e, destination, &[9.0; 4]);
        put_floats(&mut e, source, &[1.0, 2.0, 3.0, 7.0]);
        start_log(&mut e);
        let result = e.call(0x0055_3fc0, &args![destination, source]).u32();
        assert_eq!(result, destination);
        assert_eq!(floats(&e, destination, 4), vec![1.0, 2.0, 3.0, 0.0]);
        assert_eq!(
            calls(&e),
            (0..4)
                .map(|index| (VECTOR_COMPONENT, vec![destination, index]))
                .collect::<Vec<_>>()
        );
    }

    /// What the doubles of `fn_00554010` saw.
    #[derive(Default)]
    struct WorldSeen {
        cinfo: u32,
        gravity: Vec<f32>,
        scale_args: Vec<f32>,
        constructor: Vec<u32>,
        extent: Vec<f32>,
        limit: Vec<f32>,
        scale_one: f32,
        centre: Vec<f32>,
    }

    /// Doubles for everything `fn_00554010` calls; the world object is the
    /// block the constructor is given.
    fn exterior_world_setup(e: &mut Engine) -> Rc<RefCell<WorldSeen>> {
        let seen = Rc::new(RefCell::new(WorldSeen::default()));
        guard_doubles(e);
        component_double(e);
        quiet(
            e,
            &[
                CINFO_CONSTRUCT,
                CINFO_SET_BROAD_PHASE_SIZE,
                EMPTY_FUNCTION,
                WIND_LISTENER_BASE_CONSTRUCT,
                WORLD_SET_GRAVITY_VECTOR,
                WORLD_SET_FIELD_A,
                WORLD_SET_FIELD_B,
                HOLDER_FIELD_70_ADD,
                WORLD_SET_GRAVITY_SCALE,
                LISTENER_REGISTER,
                WORLD_SET_VISUAL_DEBUGGER_ENABLED,
                WORLD_SET_VISUAL_DEBUGGER,
                CINFO_MEMBER_DESTRUCT,
                CINFO_BASE_DESTRUCT,
                fake(0xc4),
                LIST_NODE_ITEM_ADDRESS,
            ],
        );
        e.register(MEMORY_MANAGER_GET, |_, _| 0x1111u32.into_ret());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        for address in [
            BROAD_PHASE_CONSTRUCT,
            FILTER_CONSTRUCT,
            LISTENER_OBJECT_CONSTRUCT,
            LISTENER_CONSTRUCT,
        ] {
            e.register(address, |_, a| a[0].into_ret());
        }
        returns(e, HOLDER_GET_WORLD, 0x5000);
        returns(e, WATER_LISTENER_GET, 0xaa00);
        returns(e, SCENE_HAVOK_SETTING, 0x77);
        returns(e, VISUAL_DEBUGGER_SUPPRESSED, 1);
        e.register(SCALE_FLOAT, |_, a| (f32::from_bits(a[0]) * 2.0).into_ret());
        let recorder = seen.clone();
        e.register_double(CINFO_SET_GRAVITY, move |e, a| {
            recorder.borrow_mut().gravity = floats(e, a[1], 4);
            Ret::default()
        });
        let recorder = seen.clone();
        e.register_double(EXTERIOR_WORLD_CONSTRUCT, move |e, a| {
            let mut seen = recorder.borrow_mut();
            seen.constructor = a.to_vec();
            seen.cinfo = a[1];
            seen.extent = floats(e, a[2], 3);
            seen.limit = floats(e, a[3], 3);
            seen.scale_one = e.mem.f32(a[1] + 0x54);
            e.mem.set_u32(a[0], TEST_VTABLE);
            a[0].into_ret()
        });
        let recorder = seen.clone();
        e.register_double(WORLD_SET_CENTER, move |e, a| {
            recorder.borrow_mut().centre = floats(e, a[1], 3);
            Ret::default()
        });
        let recorder = seen.clone();
        e.register_double(SCALE_FLOAT, move |_, a| {
            let value = f32::from_bits(a[0]);
            recorder.borrow_mut().scale_args.push(value);
            (value * 2.0).into_ret()
        });
        e.set_global(GRAVITY_Z, -9.5f32);
        e.set_global(EXTERIOR_BROAD_PHASE_SIZE, 100.0f32);
        e.set_global(EXTERIOR_WORLD_EXTENT, 10.0f32);
        e.set_global(EXTERIOR_WORLD_LIMIT, 3.0f32);
        seen
    }

    #[test]
    fn exterior_world_is_built_in_the_games_order() {
        let mut e = engine();
        let seen = exterior_world_setup(&mut e);
        let centre = e.mem.alloc(12);
        put_floats(&mut e, centre, &[100.0, 200.0, 300.0]);
        start_log(&mut e);
        let world = e.call(0x0055_4010, &args![centre]).u32();
        let log = calls(&e);
        let key = [
            SCOPE_GUARD_CONSTRUCT,
            CINFO_CONSTRUCT,
            CINFO_SET_GRAVITY,
            CINFO_SET_BROAD_PHASE_SIZE,
            EXTERIOR_WORLD_CONSTRUCT,
            WIND_LISTENER_BASE_CONSTRUCT,
            BROAD_PHASE_CONSTRUCT,
            WATER_LISTENER_GET,
            WORLD_SET_GRAVITY_SCALE,
            FILTER_CONSTRUCT,
            SCENE_HAVOK_SETTING,
            LISTENER_OBJECT_CONSTRUCT,
            LISTENER_CONSTRUCT,
            LISTENER_REGISTER,
            WORLD_SET_CENTER,
            VISUAL_DEBUGGER_SUPPRESSED,
            WORLD_SET_VISUAL_DEBUGGER_ENABLED,
            WORLD_SET_VISUAL_DEBUGGER,
            CINFO_BASE_DESTRUCT,
            SCOPE_GUARD_DESTRUCT,
        ];
        let order: Vec<u32> = addresses(&log)
            .into_iter()
            .filter(|address| key.contains(address))
            .collect();
        assert_eq!(order, key.to_vec());
        let seen = seen.borrow();
        assert_eq!(world, seen.constructor[0]);
        assert_ne!(world, 0);
        assert_eq!(seen.gravity, vec![0.0, 0.0, -9.5, 0.0]);
        // The extent and limit vectors went through the scaling.
        assert_eq!(seen.extent, vec![20.0; 3]);
        assert_eq!(seen.limit, vec![6.0; 3]);
        assert_eq!(seen.scale_one, 1.0);
        assert_eq!(seen.centre, vec![100.0, 200.0, 0.0]);
        assert_eq!(seen.scale_args[0], 100.0);
        assert_eq!(
            calls_to(&log, CINFO_SET_BROAD_PHASE_SIZE)[0][1],
            200.0f32.to_bits()
        );
        // Visual debugger: suppressed in the game's setting, so disabled.
        assert_eq!(
            calls_to(&log, WORLD_SET_VISUAL_DEBUGGER_ENABLED),
            vec![vec![world, 0]]
        );
        assert_eq!(
            calls_to(&log, WORLD_SET_VISUAL_DEBUGGER),
            vec![vec![world, 0]]
        );
        // The guard line and the water listener.
        assert_eq!(
            calls_to(&log, SCOPE_GUARD_CONSTRUCT)[0][1..],
            [0x10, 1, SOURCE_FILE_NAME, 0x3c1b]
        );
        assert_eq!(
            calls_to(&log, WORLD_SET_GRAVITY_SCALE),
            vec![vec![world, 0xaa00]]
        );
        // The singleton was created.
        assert_ne!(e.global::<u32>(LISTENER_SINGLETON), 0);
    }

    #[test]
    fn exterior_world_keeps_an_existing_listener_singleton() {
        let mut e = engine();
        let _ = exterior_world_setup(&mut e);
        e.set_global(LISTENER_SINGLETON, 0x4321u32);
        let centre = e.mem.alloc(12);
        start_log(&mut e);
        e.call(0x0055_4010, &args![centre]);
        let log = calls(&e);
        assert!(calls_to(&log, LISTENER_CONSTRUCT).is_empty());
        assert_eq!(e.global::<u32>(LISTENER_SINGLETON), 0x4321);
        assert_eq!(
            calls_to(&log, LISTENER_REGISTER),
            vec![vec![0x4321, 0x5000]]
        );
        // Visual debugger setting not suppressed: enabled.
        returns(&mut e, VISUAL_DEBUGGER_SUPPRESSED, 0);
        start_log(&mut e);
        e.call(0x0055_4010, &args![centre]);
        assert_eq!(
            calls_to(&calls(&e), WORLD_SET_VISUAL_DEBUGGER_ENABLED)[0][1],
            1
        );
    }

    #[test]
    fn wind_listener_constructor_stores_its_vtable() {
        let mut e = engine();
        quiet(&mut e, &[WIND_LISTENER_BASE_CONSTRUCT]);
        let at = e.mem.alloc(0x14);
        start_log(&mut e);
        assert_eq!(e.call(0x0055_4590, &args![at]).u32(), at);
        assert_eq!(calls(&e), vec![(WIND_LISTENER_BASE_CONSTRUCT, vec![at])]);
        assert_eq!(e.mem.u32(at), WIND_LISTENER_VTABLE);
    }

    #[test]
    fn listener_destructor_chain_ends_at_the_entity_listener_vtable() {
        for entry in [0x0055_45b0u32, 0x0055_45d0, 0x0055_45f0] {
            let mut e = engine();
            quiet(&mut e, &[LISTENER_MEMBER_DESTRUCT]);
            let at = e.mem.alloc(0x20);
            e.mem.set_u32(at, 0xdead);
            start_log(&mut e);
            e.call(entry, &args![at]);
            assert_eq!(calls(&e), vec![(LISTENER_MEMBER_DESTRUCT, vec![at + 0x0c])]);
            assert_eq!(e.mem.u32(at), ENTITY_LISTENER_VTABLE);
        }
    }

    #[test]
    fn entity_listener_vtable_store_and_scalar_deleting_destructor() {
        let mut e = engine();
        quiet(&mut e, &[DEALLOCATE_BLOCK]);
        let at = e.mem.alloc(0x20);
        e.call(0x0055_4650, &args![at]);
        assert_eq!(e.mem.u32(at), ENTITY_LISTENER_VTABLE);
        e.mem.set_u32(at, 0);
        start_log(&mut e);
        assert_eq!(e.call(0x0055_4670, &args![at, 0u32]).u32(), at);
        assert!(calls(&e).is_empty());
        assert_eq!(e.mem.u32(at), ENTITY_LISTENER_VTABLE);
        start_log(&mut e);
        assert_eq!(e.call(0x0055_4670, &args![at, 3u32]).u32(), at);
        assert_eq!(calls(&e), vec![(DEALLOCATE_BLOCK, vec![at])]);
    }

    #[test]
    fn move_exterior_world_scales_the_shift_for_the_world_in_the_global() {
        let mut e = engine();
        guard_doubles(&mut e);
        e.register(SLOT_GET_POINTER, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LIST_NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(SCALE_VECTOR, |_, a| a[0].into_ret());
        quiet(&mut e, &[WORLD_APPLY_VECTOR]);
        e.set_global(EXTERIOR_WORLD_SLOT, 0x8800u32);
        let shift = e.mem.alloc(12);
        start_log(&mut e);
        e.call(0x0055_46a0, &args![shift]);
        let log = calls(&e);
        let scaled = calls_to(&log, SCALE_VECTOR)[0].clone();
        assert_eq!(scaled[1], shift);
        assert_eq!(
            addresses(&log),
            vec![
                SCOPE_GUARD_CONSTRUCT,
                SLOT_GET_POINTER,
                LIST_NODE_ITEM_ADDRESS,
                SCALE_VECTOR,
                WORLD_APPLY_VECTOR,
                SCOPE_GUARD_DESTRUCT
            ]
        );
        assert_eq!(
            calls_to(&log, SLOT_GET_POINTER),
            vec![vec![EXTERIOR_WORLD_SLOT]]
        );
        assert_eq!(
            calls_to(&log, WORLD_APPLY_VECTOR),
            vec![vec![0x8800, scaled[0]]]
        );
        assert_eq!(
            calls_to(&log, SCOPE_GUARD_CONSTRUCT)[0][1..],
            [0x10, 1, SOURCE_FILE_NAME, 0x3c5b]
        );
    }

    #[test]
    fn scaled_vector_is_given_to_the_world_call() {
        let mut e = engine();
        e.register(LIST_NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        returns(&mut e, SCALE_VECTOR, 0x6600);
        quiet(&mut e, &[WORLD_APPLY_VECTOR]);
        start_log(&mut e);
        e.call(0x0055_4720, &args![0x1234u32, 0x5678u32]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, SCALE_VECTOR)[0][1], 0x5678);
        assert_eq!(
            calls_to(&log, WORLD_APPLY_VECTOR),
            vec![vec![0x1234, 0x6600]]
        );
    }

    #[test]
    fn exterior_world_refresh_needs_the_pointer() {
        let mut e = engine();
        e.register(SLOT_GET_POINTER, |e, a| e.mem.u32(a[0]).into_ret());
        let hits = Rc::new(RefCell::new(vec![]));
        let recorder = hits.clone();
        e.register_double(fake(0xc4), move |_, a| {
            recorder.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x0055_4780, &[]);
        assert!(hits.borrow().is_empty());
        let world = object(&mut e, 0x20);
        e.set_global(EXTERIOR_WORLD_SLOT, world);
        start_log(&mut e);
        e.call(0x0055_4780, &[]);
        assert_eq!(*hits.borrow(), vec![vec![world]]);
        // The pointer is read twice, as the game does.
        assert_eq!(calls_to(&calls(&e), SLOT_GET_POINTER).len(), 2);
    }

    #[test]
    fn land_height_asks_the_land_or_stores_zero() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        let out = e.mem.alloc(4);
        e.mem.set_f32(out, 5.0);
        returns(&mut e, CELL_GET_LAND, 0);
        assert_eq!(e.call(0x0055_47c0, &args![cell, 0x99u32, out]).u8(), 0);
        assert_eq!(e.mem.f32(out), 0.0);
        returns(&mut e, CELL_GET_LAND, 0x7300);
        returns(&mut e, LAND_GET_HEIGHT, 1);
        start_log(&mut e);
        assert_eq!(e.call(0x0055_47c0, &args![cell, 0x99u32, out]).u8(), 1);
        assert_eq!(
            calls_to(&calls(&e), LAND_GET_HEIGHT),
            vec![vec![0x7300, 0x99, out]]
        );
    }

    #[test]
    fn form_cell_child_test_by_type_word_or_by_form_type() {
        let mut e = engine();
        let form = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0055_4810, &args![0u32]).u8(), 0);
        // First word equal to the word at 01187020: the type at +0x0c.
        e.set_global(CELL_CHILD_FORM_WORD, 0x5555u32);
        e.mem.set_u32(form, 0x5555);
        for (kind, expected) in [(5, 0), (6, 1), (7, 0), (8, 1), (9, 1), (10, 0)] {
            e.mem.set_u32(form + 0x0c, kind);
            assert_eq!(
                e.call(0x0055_4810, &args![form]).u8(),
                expected,
                "kind {kind}"
            );
        }
        // Otherwise the form type of the first word decides.
        e.mem.set_u32(form, 0x6666);
        returns(&mut e, FORM_TYPE_FROM_STRING, 0x3a);
        start_log(&mut e);
        assert_eq!(e.call(0x0055_4810, &args![form]).u8(), 1);
        assert_eq!(
            calls_to(&calls(&e), FORM_TYPE_FROM_STRING),
            vec![vec![0x6666]]
        );
        returns(&mut e, FORM_TYPE_FROM_STRING, 0x41);
        assert_eq!(e.call(0x0055_4810, &args![form]).u8(), 0);
    }

    #[test]
    fn form_types_in_the_jump_table_are_children() {
        let mut e = engine();
        let expected = [0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x42, 0x43, 0x69];
        for kind in 0..=255u32 {
            let want = expected.contains(&kind);
            assert_eq!(e.call(0x0055_48a0, &args![kind]).bool(), want, "{kind:#x}");
            assert_eq!(
                e.call(0x0055_4880, &args![0x1234u32, kind]).bool(),
                want,
                "{kind:#x}"
            );
        }
    }

    /// Doubles for `fn_00554960` and `fn_00554a20` with an empty node tree;
    /// returns the world object (which records its virtual `+0xd8`).
    fn hide_setup(e: &mut Engine) -> (Ptr<TESObjectCELL>, u32, Hits) {
        let cell = e.new_object::<TESObjectCELL>();
        let world = object(e, 0x20);
        let hits = Rc::new(RefCell::new(vec![]));
        let recorder = hits.clone();
        e.register_double(fake(0xd8), move |_, a| {
            recorder.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        returns(e, CELL_HAVOK_WORLD, world);
        returns(e, CELL_NODE_OF_CELL, 0x4400);
        returns(e, CELL_NODE_B, 0x4500);
        returns(e, CELL_CHILD_NODE, 0x4600);
        returns(e, NODE_CHILD_COUNT, 0);
        quiet(e, &[PREPARE_WITHOUT_LIGHTING]);
        (cell, world, hits)
    }

    #[test]
    fn hiding_needs_a_world_and_a_node_and_prepares_the_node() {
        let mut e = engine();
        let (cell, world, hits) = hide_setup(&mut e);
        start_log(&mut e);
        e.call(0x0055_4960, &args![cell, 1u32]);
        assert_eq!(*hits.borrow(), vec![vec![world, 0x4400, 1]]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, PREPARE_WITHOUT_LIGHTING), vec![vec![0x4400]]);
        // The node walk ran for both of the cell's node lookups.
        assert_eq!(calls_to(&log, CELL_NODE_B).len(), 1);
        assert_eq!(calls_to(&log, CELL_CHILD_NODE), vec![vec![cell.addr(), 7]]);
        // No node: nothing.
        returns(&mut e, CELL_NODE_OF_CELL, 0);
        hits.borrow_mut().clear();
        start_log(&mut e);
        e.call(0x0055_4960, &args![cell, 1u32]);
        assert!(hits.borrow().is_empty());
        assert!(calls_to(&calls(&e), PREPARE_WITHOUT_LIGHTING).is_empty());
        // No world: nothing.
        returns(&mut e, CELL_NODE_OF_CELL, 0x4400);
        returns(&mut e, CELL_HAVOK_WORLD, 0);
        start_log(&mut e);
        e.call(0x0055_4960, &args![cell, 1u32]);
        assert!(hits.borrow().is_empty());
    }

    #[test]
    fn hiding_follows_the_byte_of_the_world_object() {
        let mut e = engine();
        let (cell, world, hits) = hide_setup(&mut e);
        returns(&mut e, WORLD_BYTE_16, 0);
        assert_eq!(e.call(0x0055_4910, &args![cell]).u8(), 1);
        assert_eq!(*hits.borrow(), vec![vec![world, 0x4400, 1]]);
        returns(&mut e, WORLD_BYTE_16, 0x80);
        assert_eq!(e.call(0x0055_4910, &args![cell]).u8(), 0);
        assert_eq!(hits.borrow()[1], vec![world, 0x4400, 0]);
        returns(&mut e, CELL_HAVOK_WORLD, 0);
        start_log(&mut e);
        assert_eq!(e.call(0x0055_4910, &args![cell]).u8(), 0);
        assert_eq!(hits.borrow().len(), 2);
        assert_eq!(addresses(&calls(&e)), vec![CELL_HAVOK_WORLD]);
    }

    #[test]
    fn reference_list_stops_at_the_first_empty_item() {
        let mut e = engine();
        list_doubles(&mut e);
        let hits = Rc::new(RefCell::new(vec![]));
        let recorder = hits.clone();
        e.register_double(REFERENCE_UPDATE_FOR_FLAG, move |_, a| {
            recorder.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let cell = e.new_object::<TESObjectCELL>();
        // The list head lives inside the cell, at +0xac.
        e.mem.set_u32(cell.addr() + 0xac, 0x11);
        let second = list_of(&mut e, &[0x22, 0, 0x33]);
        e.mem.set_u32(cell.addr() + 0xb0, second);
        e.call(0x0055_49d0, &args![cell, 1u32]);
        assert_eq!(*hits.borrow(), vec![vec![0x11, 1], vec![0x22, 1]]);
    }

    /// A node tree for `fn_00554a20`: node address to children.
    type Tree = Rc<RefCell<std::collections::HashMap<u32, Vec<u32>>>>;

    fn tree_doubles(e: &mut Engine, tree: &Tree) {
        let children = tree.clone();
        e.register_double(NODE_CHILD_COUNT, move |_, a| {
            (children.borrow().get(&a[0]).map_or(0, Vec::len) as u32).into_ret()
        });
        let children = tree.clone();
        e.register_double(NODE_CHILD_AT, move |_, a| {
            children.borrow()[&a[0]][a[1] as usize].into_ret()
        });
    }

    #[test]
    fn reference_updates_walk_the_cell_node_backwards_and_the_group_forwards() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        let tree: Tree = Default::default();
        tree_doubles(&mut e, &tree);
        let updates = Rc::new(RefCell::new(vec![]));
        let recorder = updates.clone();
        e.register_double(fake(0x2f0), move |_, a| {
            recorder.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        // The 3D objects 0x10, 0x20 and 0x60 have reference objects.
        let mut references = std::collections::HashMap::new();
        for object_3d in [0x10u32, 0x20, 0x60] {
            references.insert(object_3d, reference_object(&mut e));
        }
        let lookup = references.clone();
        e.register_double(FIND_REFERENCE_FOR_3D, move |_, a| {
            lookup.get(&a[0]).copied().unwrap_or(0).into_ret()
        });
        // Only the inner reference answers true to virtual +0x100.
        e.mem.set_u32(references[&0x60] + 0x38, 1);
        // Cell node children: 0x10, none, 0x20.
        tree.borrow_mut().insert(0x4500, vec![0x10, 0, 0x20]);
        // The group: the first node qualifies by its virtual +0x14, the
        // second by neither, the third by the table, and a null entry.
        let nodes: Vec<u32> = (0..3).map(|_| reference_object(&mut e)).collect();
        e.mem.set_u32(nodes[0] + 0x3c, 1);
        tree.borrow_mut()
            .insert(0x4600, vec![nodes[0], nodes[1], nodes[2], 0]);
        for node in &nodes {
            tree.borrow_mut().insert(*node, vec![0x60, 0]);
        }
        let table_hits = Rc::new(RefCell::new(vec![]));
        let recorder = table_hits.clone();
        let accepted = nodes[2];
        e.register_double(NODE_TABLE_TEST, move |_, a| {
            recorder.borrow_mut().push(a.to_vec());
            u32::from(a[1] == accepted).into_ret()
        });
        returns(&mut e, CELL_NODE_B, 0x4500);
        returns(&mut e, CELL_CHILD_NODE, 0x4600);
        e.call(0x0055_4a20, &args![cell, 9u32]);
        assert_eq!(
            *updates.borrow(),
            vec![
                vec![references[&0x20], 9],
                vec![references[&0x10], 9],
                vec![references[&0x60], 9],
                vec![references[&0x60], 9],
            ]
        );
        // The table is asked for the nodes whose virtual answered false.
        assert_eq!(
            *table_hits.borrow(),
            vec![vec![NODE_TABLE, nodes[1]], vec![NODE_TABLE, nodes[2]]]
        );
        // Without the answer of +0x100 only the cell node's two are updated.
        e.mem.set_u32(references[&0x60] + 0x38, 0);
        updates.borrow_mut().clear();
        e.call(0x0055_4a20, &args![cell, 9u32]);
        assert_eq!(updates.borrow().len(), 2);
    }

    /// Doubles shared by the save size and save tests: the cell, the save
    /// buffer object behind `011de45c` and the trace holder byte.
    fn save_setup(e: &mut Engine) -> (Ptr<TESObjectCELL>, u32) {
        let cell = e.new_object::<TESObjectCELL>();
        let buffer = e.mem.alloc(0x40);
        e.set_global(SAVE_GAME_POINTER, buffer);
        let holder = e.mem.alloc(4);
        returns(e, BYTE_HOLDER_GET, holder);
        (cell, holder)
    }

    #[test]
    fn save_size_sums_the_parts_for_the_flags() {
        let mut e = engine();
        let (cell, _holder) = save_setup(&mut e);
        returns(&mut e, FORM_SAVE_SIZE, 3);
        returns(&mut e, USE_SAVE_GAME_BLOCKS, 0);
        // Nothing but the base class.
        assert_eq!(e.call(0x0055_4b90, &args![cell, 0u32]).u16(), 3);
        // Every flag, with save blocks.
        returns(&mut e, USE_SAVE_GAME_BLOCKS, 1);
        let seen = object(&mut e, 0x20);
        e.register(fake(0x08), |_, a| {
            assert_eq!(a[1], 0);
            5u32.into_ret()
        });
        returns(&mut e, CELL_SEEN_DATA, seen);
        returns(&mut e, TEXT_GET_STRING, 0x9000);
        returns(&mut e, STRING_LENGTH, 7);
        start_log(&mut e);
        let flags = 0x4000_0000 | 0x8000_0000 | 2 | 4 | 8;
        // 4 + 3 + (4 + 2) + 1 + 5 + (1 + 7) + 4
        assert_eq!(e.call(0x0055_4b90, &args![cell, flags]).u16(), 31);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, FORM_SAVE_SIZE),
            vec![vec![cell.addr(), flags]]
        );
        assert_eq!(
            calls_to(&log, TEXT_GET_STRING),
            vec![vec![cell.addr() + 0x18]]
        );
        let buffer = e.global::<u32>(SAVE_GAME_POINTER);
        assert_eq!(calls_to(&log, USE_SAVE_GAME_BLOCKS), vec![vec![buffer]]);
        assert_eq!(
            calls_to(&log, BYTE_HOLDER_GET),
            vec![vec![SAVE_TRACE_HOLDER]]
        );
        // The sum wraps at 16 bits: 0xfffe + 6 + 1 + 7.
        returns(&mut e, FORM_SAVE_SIZE, 0xfffe);
        assert_eq!(e.call(0x0055_4b90, &args![cell, 4u32]).u16(), 12);
    }

    #[test]
    fn save_size_reports_what_it_added_when_the_trace_is_on() {
        let mut e = engine();
        let (cell, holder) = save_setup(&mut e);
        returns(&mut e, FORM_SAVE_SIZE, 3);
        returns(&mut e, USE_SAVE_GAME_BLOCKS, 0);
        quiet(&mut e, &[ERROR_REPORT]);
        e.mem.set_u8(holder, 1);
        // No form being saved.
        returns(&mut e, SAVING_FORM_INFO, 0);
        start_log(&mut e);
        e.call(0x0055_4b90, &args![cell, 8u32]);
        assert_eq!(
            calls_to(&calls(&e), ERROR_REPORT),
            vec![vec![0x0101_2c78, 4, 0x3d51, SOURCE_FILE_NAME]]
        );
        // A form being saved: its id, name and flags.
        let info = e.mem.alloc(16);
        e.mem.set_u32(info, 0xf00d);
        e.mem.set_u32(info + 5, 0x1234_5678);
        returns(&mut e, SAVING_FORM_INFO, info);
        let form = object(&mut e, 0x20);
        returns(&mut e, FORM_LOOK_UP, form);
        e.register(fake(0x130), |_, _| 0x7777u32.into_ret());
        start_log(&mut e);
        e.call(0x0055_4b90, &args![cell, 8u32]);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, ERROR_REPORT),
            vec![vec![
                0x0101_2cb0,
                4,
                0xf00d,
                0x7777,
                0x1234_5678,
                0x3d51,
                SOURCE_FILE_NAME
            ]]
        );
        assert_eq!(calls_to(&log, FORM_LOOK_UP), vec![vec![0xf00d]]);
    }

    /// The positions the buffer reports, in order.
    fn position_double(e: &mut Engine, positions: Vec<u32>) {
        let remaining = Rc::new(RefCell::new(positions.into_iter()));
        e.register_double(BUFFER_POSITION, move |_, _| {
            remaining.borrow_mut().next().unwrap().into_ret()
        });
    }

    /// Records `(size, first bytes)` of every call of a `(this, data, size)`
    /// function.
    fn data_recorder(e: &mut Engine, address: u32) -> DataLog {
        let log = Rc::new(RefCell::new(vec![]));
        let recorder = log.clone();
        e.register_double(address, move |e, a| {
            recorder
                .borrow_mut()
                .push((a[2], e.mem.bytes(a[1], a[2].min(8))));
            Ret::default()
        });
        log
    }

    #[test]
    fn save_game_writes_every_part_and_fills_the_block_length() {
        let mut e = engine();
        let (cell, _holder) = save_setup(&mut e);
        returns(&mut e, USE_SAVE_GAME_BLOCKS, 1);
        quiet(&mut e, &[FORM_SAVE_BASE, BUFFER_WRITE]);
        let area = e.mem.alloc(0x100);
        position_double(&mut e, vec![area, area + 0x10, area + 0x50]);
        let saved = data_recorder(&mut e, FORM_SAVE_GAME_DATA);
        let ids = data_recorder(&mut e, FORM_SAVE_NUMERIC_ID);
        returns(&mut e, CELL_DETACH_TIME, 0x0102_0304);
        e.set(cell, TESObjectCELL::cCellFlags, 0x61);
        e.set(cell, TESObjectCELL::cCellGameFlags, 0x05);
        let seen = object(&mut e, 0x20);
        let seen_calls = Rc::new(RefCell::new(vec![]));
        let recorder = seen_calls.clone();
        e.register_double(fake(0x10), move |_, a| {
            recorder.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        returns(&mut e, CELL_SEEN_DATA, seen);
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"Vault");
        returns(&mut e, TEXT_GET_STRING, name);
        returns(&mut e, STRING_LENGTH, 5);
        returns(&mut e, CELL_OWNER, 0x7001);
        returns(&mut e, FORM_ID, 0xabcd);
        let flags = 0x4000_0000 | 0x8000_0000 | 2 | 4 | 8;
        start_log(&mut e);
        e.call(0x0055_4d30, &args![cell, flags]);
        let log = calls(&e);
        assert_eq!(
            *saved.borrow(),
            vec![
                (4, vec![4, 3, 2, 1]),
                (1, vec![0x65]),
                (1, vec![5]),
                (5, b"Vault".to_vec()),
            ]
        );
        assert_eq!(*ids.borrow(), vec![(4, vec![0xcd, 0xab, 0, 0])]);
        assert_eq!(*seen_calls.borrow(), vec![vec![seen, 0]]);
        // The marker, then the zero length slot, were written to the buffer.
        let writes = calls_to(&log, BUFFER_WRITE);
        assert_eq!(writes.len(), 2);
        assert_eq!(writes[0][2], 4);
        assert_eq!(writes[1][2], 2);
        assert_eq!(e.mem.u16(area + 0x10), 0x40);
        assert_eq!(
            calls_to(&log, FORM_SAVE_BASE),
            vec![vec![cell.addr(), flags]]
        );
        assert_eq!(calls_to(&log, FORM_ID), vec![vec![0x7001]]);
    }

    #[test]
    fn save_game_without_owner_writes_a_zero_id() {
        let mut e = engine();
        let (cell, _holder) = save_setup(&mut e);
        returns(&mut e, USE_SAVE_GAME_BLOCKS, 0);
        quiet(&mut e, &[FORM_SAVE_BASE]);
        position_double(&mut e, vec![0]);
        returns(&mut e, CELL_OWNER, 0);
        let ids = data_recorder(&mut e, FORM_SAVE_NUMERIC_ID);
        e.call(0x0055_4d30, &args![cell, 8u32]);
        assert_eq!(*ids.borrow(), vec![(4, vec![0, 0, 0, 0])]);
    }

    #[test]
    fn save_game_reports_overlong_blocks_and_the_trace() {
        let mut e = engine();
        let (cell, holder) = save_setup(&mut e);
        quiet(
            &mut e,
            &[FORM_SAVE_BASE, BUFFER_WRITE, DEBUG_PRINT, ERROR_REPORT],
        );
        returns(&mut e, USE_SAVE_GAME_BLOCKS, 0);
        position_double(&mut e, vec![0]);
        start_log(&mut e);
        e.call(0x0055_4d30, &args![cell, 0u32]);
        let log = calls(&e);
        assert!(calls_to(&log, BUFFER_WRITE).is_empty());
        assert!(calls_to(&log, DEBUG_PRINT).is_empty());
        // Blocks used, a block longer than 0xffff bytes is reported.
        returns(&mut e, USE_SAVE_GAME_BLOCKS, 1);
        let area = e.mem.alloc(0x20000);
        position_double(&mut e, vec![area, area + 4, area + 4 + 0x1_0000]);
        start_log(&mut e);
        e.call(0x0055_4d30, &args![cell, 0u32]);
        assert_eq!(
            calls_to(&calls(&e), DEBUG_PRINT),
            vec![vec![0x0101_5318, SOURCE_FILE_NAME, 0x3d86]]
        );
        // A block of exactly 0xffff bytes is not.
        position_double(&mut e, vec![area, area + 4, area + 4 + 0xffff]);
        start_log(&mut e);
        e.call(0x0055_4d30, &args![cell, 0u32]);
        assert!(calls_to(&calls(&e), DEBUG_PRINT).is_empty());
        assert_eq!(e.mem.u16(area + 4), 0xffff);
        // With the trace holder set the bytes written are reported.
        e.mem.set_u8(holder, 1);
        returns(&mut e, USE_SAVE_GAME_BLOCKS, 0);
        returns(&mut e, SAVING_FORM_INFO, 0);
        position_double(&mut e, vec![area, area, area + 12]);
        start_log(&mut e);
        e.call(0x0055_4d30, &args![cell, 0u32]);
        assert_eq!(
            calls_to(&calls(&e), ERROR_REPORT),
            vec![vec![0x0101_536c, 12, 0x3d86, SOURCE_FILE_NAME]]
        );
    }

    /// Doubles shared by the load tests. The buffer answers the block marker
    /// and `length` as the block length; the cell's extra data list is at
    /// `+0x28`.
    fn load_setup(e: &mut Engine, version: u32, blocks: bool, length: u16) -> Ptr<TESObjectCELL> {
        let (cell, _) = save_setup(e);
        returns(e, BUFFER_VERSION, version);
        returns(e, USE_SAVE_GAME_BLOCKS, u32::from(blocks));
        quiet(e, &[FORM_LOAD_GAME, EXTRA_LIST_SET_FIELD, DEBUG_PRINT]);
        e.register_double(BUFFER_READ, move |e, a| {
            if a[2] == 4 {
                e.mem.set_u32(a[1], 0x424c_4f4b);
            } else {
                e.mem.set_u16(a[1], length);
            }
            Ret::default()
        });
        let list = cell.addr() + 0x28;
        e.register_double(CELL_EXTRA_DATA_LIST, move |_, _| list.into_ret());
        cell
    }

    /// `LoadGameDataOLD` writes the next queued bytes.
    fn load_data_queue(e: &mut Engine, chunks: Vec<Vec<u8>>) {
        let queue = Rc::new(RefCell::new(std::collections::VecDeque::from(chunks)));
        e.register_double(FORM_LOAD_GAME_DATA, move |e, a| {
            let bytes = queue.borrow_mut().pop_front().unwrap();
            assert_eq!(bytes.len() as u32, a[2]);
            for (index, byte) in bytes.into_iter().enumerate() {
                e.mem.set_u8(a[1] + index as u32, byte);
            }
            Ret::default()
        });
    }

    #[test]
    fn load_game_reads_every_part_of_a_current_version() {
        let mut e = engine();
        let cell = load_setup(&mut e, 0x5b, true, 0x30);
        let area = e.mem.alloc(0x100);
        position_double(&mut e, vec![area, area + 0x30]);
        returns(&mut e, LOADING_FORM_INFO, 0);
        load_data_queue(
            &mut e,
            vec![vec![4, 3, 2, 1], vec![0xff], vec![3], b"abc".to_vec()],
        );
        e.register(FORM_LOAD_NUMERIC_ID, |e, a| {
            e.mem.set_u32(a[1], 0x5555);
            Ret::default()
        });
        e.register(CLEAR_BUFFER, |e, a| {
            for index in 0..a[2] {
                e.mem.set_u8(a[0] + index, 0);
            }
            Ret::default()
        });
        let names = Rc::new(RefCell::new(vec![]));
        let recorder = names.clone();
        e.register_double(FULL_NAME_SET, move |e, a| {
            recorder.borrow_mut().push((a[0], e.mem.cstr(a[1])));
            Ret::default()
        });
        quiet(&mut e, &[CELL_SET_OWNER, EXTRA_LIST_SET_SEEN_DATA]);
        returns(&mut e, CELL_SEEN_DATA, 0);
        returns(&mut e, CELL_IS_INTERIOR, 1);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(INTERIOR_SEEN_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], TEST_VTABLE);
            a[0].into_ret()
        });
        let loads = Rc::new(RefCell::new(vec![]));
        let recorder = loads.clone();
        e.register_double(fake(0x18), move |_, a| {
            recorder.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.set(cell, TESObjectCELL::cCellFlags, 0x81);
        let flags = 0x4000_0000 | 0x8000_0000 | 2 | 4 | 8;
        start_log(&mut e);
        e.call(0x0055_4fd0, &args![cell, flags, 0x55u32]);
        let log = calls(&e);
        // The time first (version 0x5b), through the extra data list.
        assert_eq!(
            calls_to(&log, EXTRA_LIST_SET_FIELD),
            vec![vec![cell.addr() + 0x28, 0x0102_0304]]
        );
        assert_eq!(
            calls_to(&log, FORM_LOAD_GAME),
            vec![vec![cell.addr(), flags, 0x55]]
        );
        let order = addresses(&log);
        assert!(
            order.iter().position(|a| *a == EXTRA_LIST_SET_FIELD)
                < order.iter().position(|a| *a == FORM_LOAD_GAME)
        );
        // The flags byte 0xff unpacked.
        assert_eq!(e.get(cell, TESObjectCELL::cCellFlags), 0x81 | 0x60);
        assert_eq!(e.get(cell, TESObjectCELL::cCellGameFlags), 0x9f);
        // A seen data of the interior kind was made, stored and loaded.
        let created = calls_to(&log, INTERIOR_SEEN_DATA_CONSTRUCT);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][1..], [0, 0]);
        assert_eq!(
            calls_to(&log, EXTRA_LIST_SET_SEEN_DATA),
            vec![vec![cell.addr() + 0x28, created[0][0]]]
        );
        assert_eq!(*loads.borrow(), vec![vec![created[0][0], 0xffff]]);
        // The name was cleared first, then read.
        assert_eq!(calls_to(&log, CLEAR_BUFFER)[0][1..], [0, 0x104]);
        assert_eq!(*names.borrow(), vec![(cell.addr() + 0x18, b"abc".to_vec())]);
        assert_eq!(
            calls_to(&log, CELL_SET_OWNER),
            vec![vec![cell.addr(), 0x5555]]
        );
        // The block was read to its end: no report.
        assert!(calls_to(&log, DEBUG_PRINT).is_empty());
    }

    #[test]
    fn load_game_of_an_old_version_reads_the_time_after_the_seen_data() {
        let mut e = engine();
        let cell = load_setup(&mut e, 0x59, false, 0);
        load_data_queue(&mut e, vec![vec![4, 0, 0, 0]]);
        let seen = object(&mut e, 0x20);
        returns(&mut e, CELL_SEEN_DATA, seen);
        e.register(fake(0x18), |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0055_4fd0, &args![cell, 0xc000_0000u32, 0u32]);
        let watched = [
            FORM_LOAD_GAME,
            fake(0x18),
            FORM_LOAD_GAME_DATA,
            EXTRA_LIST_SET_FIELD,
        ];
        let order: Vec<u32> = addresses(&calls(&e))
            .into_iter()
            .filter(|a| watched.contains(a))
            .collect();
        assert_eq!(order, watched.to_vec());
    }

    #[test]
    fn load_game_creates_an_exterior_seen_data_for_an_exterior_cell() {
        let mut e = engine();
        let cell = load_setup(&mut e, 0x5b, false, 0);
        returns(&mut e, CELL_SEEN_DATA, 0);
        returns(&mut e, CELL_IS_INTERIOR, 0);
        quiet(&mut e, &[EXTRA_LIST_SET_SEEN_DATA]);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(EXTERIOR_SEEN_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], TEST_VTABLE);
            a[0].into_ret()
        });
        quiet(&mut e, &[fake(0x18)]);
        start_log(&mut e);
        e.call(0x0055_4fd0, &args![cell, 0x8000_0000u32, 0u32]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x24]]);
        assert_eq!(calls_to(&log, EXTERIOR_SEEN_DATA_CONSTRUCT).len(), 1);
        assert!(calls_to(&log, INTERIOR_SEEN_DATA_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&log, fake(0x18))[0][1], 0xffff);
    }

    #[test]
    fn load_game_reports_a_wrong_block_marker() {
        let mut e = engine();
        let cell = load_setup(&mut e, 0x5b, true, 0);
        // The buffer gives a wrong marker.
        e.register(BUFFER_READ, |e, a| {
            e.mem.set_u32(a[1], if a[2] == 4 { 0x1111_1111 } else { 0 });
            Ret::default()
        });
        position_double(&mut e, vec![0, 0]);
        // No form being loaded: the version is reported.
        returns(&mut e, LOADING_FORM_INFO, 0);
        start_log(&mut e);
        e.call(0x0055_4fd0, &args![cell, 0u32, 0u32]);
        assert_eq!(
            calls_to(&calls(&e), DEBUG_PRINT),
            vec![vec![0x0101_56a8, SOURCE_FILE_NAME, 0x3d9d, 0x5b]]
        );
        // A form being loaded: its id, name, version and flags.
        let info = e.mem.alloc(16);
        e.mem.set_u32(info, 0xf00d);
        e.mem.set_u32(info + 5, 0x1234_5678);
        e.mem.set_u8(info + 9, 0x44);
        returns(&mut e, LOADING_FORM_INFO, info);
        let form = object(&mut e, 0x20);
        returns(&mut e, FORM_LOOK_UP, form);
        e.register(fake(0x130), |_, _| 0x7777u32.into_ret());
        position_double(&mut e, vec![0, 0]);
        start_log(&mut e);
        e.call(0x0055_4fd0, &args![cell, 0u32, 0u32]);
        let version = e.mem.u8(info + 9) as u32;
        let flags = e.mem.u32(info + 5);
        assert_eq!(
            calls_to(&calls(&e), DEBUG_PRINT),
            vec![vec![
                0x0101_5718,
                SOURCE_FILE_NAME,
                0x3d9d,
                0xf00d,
                0x7777,
                version,
                flags
            ]]
        );
    }

    #[test]
    fn load_game_reports_overruns_and_underruns_of_the_block() {
        let mut e = engine();
        let cell = load_setup(&mut e, 0x5b, true, 0x10);
        returns(&mut e, LOADING_FORM_INFO, 0);
        let area = 0x4000_0000u32;
        // Overrun by 0x10: the position is 0x20 past the start.
        position_double(&mut e, vec![area, area + 0x20]);
        start_log(&mut e);
        e.call(0x0055_4fd0, &args![cell, 0u32, 0u32]);
        assert_eq!(
            calls_to(&calls(&e), DEBUG_PRINT),
            vec![vec![0x0101_54a0, 0x10, SOURCE_FILE_NAME, 0x3dd5, 0x5b]]
        );
        // Underrun by 0xc.
        position_double(&mut e, vec![area, area + 4]);
        start_log(&mut e);
        e.call(0x0055_4fd0, &args![cell, 0u32, 0u32]);
        assert_eq!(
            calls_to(&calls(&e), DEBUG_PRINT),
            vec![vec![0x0101_5440, 0xc, SOURCE_FILE_NAME, 0x3dd5, 0x5b]]
        );
        // Exactly the block: nothing.
        position_double(&mut e, vec![area, area + 0x10]);
        start_log(&mut e);
        e.call(0x0055_4fd0, &args![cell, 0u32, 0u32]);
        assert!(calls_to(&calls(&e), DEBUG_PRINT).is_empty());
        // With a form being loaded the form is looked up even without a
        // report, and named in an overrun.
        let info = e.mem.alloc(16);
        e.mem.set_u32(info, 0xf00d);
        e.mem.set_u8(info + 9, 0x44);
        e.mem.set_u32(info + 5, 0x1234_5678);
        returns(&mut e, LOADING_FORM_INFO, info);
        let form = object(&mut e, 0x20);
        returns(&mut e, FORM_LOOK_UP, form);
        e.register(fake(0x130), |_, _| 0x7777u32.into_ret());
        position_double(&mut e, vec![area, area + 0x10]);
        start_log(&mut e);
        e.call(0x0055_4fd0, &args![cell, 0u32, 0u32]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, FORM_LOOK_UP), vec![vec![0xf00d]]);
        assert!(calls_to(&log, DEBUG_PRINT).is_empty());
        let version = e.mem.u8(info + 9) as u32;
        let flags = e.mem.u32(info + 5);
        position_double(&mut e, vec![area, area + 0x18]);
        start_log(&mut e);
        e.call(0x0055_4fd0, &args![cell, 0u32, 0u32]);
        assert_eq!(
            calls_to(&calls(&e), DEBUG_PRINT),
            vec![vec![
                0x0101_5588,
                8,
                SOURCE_FILE_NAME,
                0x3dd5,
                0xf00d,
                0x7777,
                version,
                flags
            ]]
        );
        position_double(&mut e, vec![area, area + 8]);
        start_log(&mut e);
        e.call(0x0055_4fd0, &args![cell, 0u32, 0u32]);
        assert_eq!(calls_to(&calls(&e), DEBUG_PRINT)[0][0], 0x0101_5500);
    }

    #[test]
    fn load_game_reads_the_time_and_the_seen_data_only_when_flagged() {
        // A cell with seen data present: nothing is created.
        let mut e = engine();
        let cell = load_setup(&mut e, 0x5b, false, 0);
        let seen = object(&mut e, 0x20);
        returns(&mut e, CELL_SEEN_DATA, seen);
        quiet(&mut e, &[fake(0x18)]);
        start_log(&mut e);
        e.call(0x0055_4fd0, &args![cell, 0x8000_0000u32, 0u32]);
        let log = calls(&e);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(calls_to(&log, EXTRA_LIST_SET_SEEN_DATA).is_empty());
        assert_eq!(calls_to(&log, fake(0x18)), vec![vec![seen, 0xffff]]);
        // Nothing flagged: only the base class.
        start_log(&mut e);
        e.call(0x0055_4fd0, &args![cell, 0u32, 0u32]);
        let seen_calls = addresses(&calls(&e));
        assert!(!seen_calls.contains(&fake(0x18)));
        assert!(!seen_calls.contains(&FORM_LOAD_GAME_DATA));
    }

    #[test]
    fn flag_set_function_clears_the_game_flags_and_the_requested_data() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        let buffer = e.mem.alloc(0x40);
        e.set_global(SAVE_GAME_POINTER, buffer);
        quiet(
            &mut e,
            &[
                CELL_EMPTY_CALL,
                CELL_CLEAR_SEEN_DATA,
                CELL_RESET_DETACH_TIME,
            ],
        );
        returns(&mut e, BUFFER_FIELD_48_IS_ZERO, 0);
        e.set(cell, TESObjectCELL::cCellGameFlags, 7);
        start_log(&mut e);
        e.call(0x0055_5570, &args![cell, 0u32]);
        assert_eq!(
            calls(&e),
            vec![
                (CELL_EMPTY_CALL, vec![cell.addr(), 0]),
                (BUFFER_FIELD_48_IS_ZERO, vec![buffer])
            ]
        );
        assert_eq!(e.get(cell, TESObjectCELL::cCellGameFlags), 7);
        returns(&mut e, BUFFER_FIELD_48_IS_ZERO, 1);
        start_log(&mut e);
        e.call(0x0055_5570, &args![cell, 0xc000_0000u32]);
        assert_eq!(e.get(cell, TESObjectCELL::cCellGameFlags), 0);
        assert_eq!(
            calls(&e)[2..],
            [
                (CELL_CLEAR_SEEN_DATA, vec![cell.addr()]),
                (CELL_RESET_DETACH_TIME, vec![cell.addr(), 0, 0])
            ]
        );
    }

    #[test]
    fn owner_is_set_to_the_form_found_for_it() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        quiet(&mut e, &[CELL_CALL_WITH_TWO_WORDS, CELL_SET_OWNER]);
        returns(&mut e, CELL_OWNER, 0x7001);
        returns(&mut e, FORM_LOOK_UP, 0x8002);
        start_log(&mut e);
        e.call(0x0055_55d0, &args![cell, 8u32, 0x44u32]);
        assert_eq!(
            calls(&e),
            vec![
                (CELL_CALL_WITH_TWO_WORDS, vec![cell.addr(), 8, 0x44]),
                (CELL_OWNER, vec![cell.addr()]),
                (FORM_LOOK_UP, vec![0x7001]),
                (CELL_SET_OWNER, vec![cell.addr(), 0x8002]),
            ]
        );
        // Without the flag or without an owner only the first call.
        start_log(&mut e);
        e.call(0x0055_55d0, &args![cell, 7u32, 0x44u32]);
        assert_eq!(calls(&e).len(), 1);
        returns(&mut e, CELL_OWNER, 0);
        start_log(&mut e);
        e.call(0x0055_55d0, &args![cell, 8u32, 0x44u32]);
        assert_eq!(
            addresses(&calls(&e)),
            vec![CELL_CALL_WITH_TWO_WORDS, CELL_OWNER]
        );
    }

    /// Doubles of the buffer flag words: `+0x17` for `00428110`, `+0x2c` for
    /// `0042ce30`.
    fn buffer_flag_doubles(e: &mut Engine, flags: u32, other_flags: u32) -> u32 {
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(buffer + 0x17, flags);
        e.mem.set_u32(buffer + 0x2c, other_flags);
        e.register(BUFFER_FLAGS_GET, |e, a| {
            let word = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], word);
            a[1].into_ret()
        });
        e.register(BUFFER_FLAGS_GET_OTHER, |e, a| {
            let word = e.mem.u32(a[0] + 0x2c);
            e.mem.set_u32(a[1], word);
            a[1].into_ret()
        });
        e.register(FLAGS_TEST, |e, a| {
            u32::from(e.mem.u32(a[0]) & a[1] != 0).into_ret()
        });
        buffer
    }

    #[test]
    fn save_to_buffer_writes_the_parts_the_buffer_asks_for() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        quiet(
            &mut e,
            &[FORM_SAVE_TO_BUFFER, BUFFER_SAVE_STRING, BUFFER_SAVE_FORM_ID],
        );
        let saved_flags = Rc::new(RefCell::new(vec![]));
        let recorder = saved_flags.clone();
        e.register_double(BUFFER_SAVE_BYTES, move |e, a| {
            recorder.borrow_mut().push((a[2], a[3], e.mem.u8(a[1])));
            Ret::default()
        });
        let seen = object(&mut e, 0x20);
        let seen_calls = Rc::new(RefCell::new(vec![]));
        let recorder = seen_calls.clone();
        e.register_double(fake(0x0c), move |_, a| {
            recorder.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        returns(&mut e, CELL_SEEN_DATA, seen);
        returns(&mut e, TEXT_GET_STRING, 0x9000);
        returns(&mut e, CELL_OWNER, 0x7001);
        e.set(cell, TESObjectCELL::cCellFlags, 0x41);
        e.set(cell, TESObjectCELL::cCellGameFlags, 0x03);
        let buffer = buffer_flag_doubles(&mut e, 0xffff_ffff, 0);
        start_log(&mut e);
        e.call(0x0055_5630, &args![cell, buffer]);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, FORM_SAVE_TO_BUFFER),
            vec![vec![cell.addr(), buffer]]
        );
        assert_eq!(*saved_flags.borrow(), vec![(1, 0, 0x43)]);
        assert_eq!(*seen_calls.borrow(), vec![vec![seen, buffer]]);
        assert_eq!(
            calls_to(&log, BUFFER_SAVE_STRING),
            vec![vec![buffer, 0x9000, 0]]
        );
        assert_eq!(
            calls_to(&log, BUFFER_SAVE_FORM_ID),
            vec![vec![buffer, 0x7001, 0]]
        );
        assert_eq!(calls_to(&log, BUFFER_FLAGS_GET).len(), 4);
        // No flags: only the base class part.
        e.mem.set_u32(buffer + 0x17, 0);
        start_log(&mut e);
        e.call(0x0055_5630, &args![cell, buffer]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, BUFFER_FLAGS_GET).len(), 4);
        assert_eq!(calls_to(&log, FORM_SAVE_TO_BUFFER).len(), 1);
        assert!(calls_to(&log, BUFFER_SAVE_STRING).is_empty());
        assert_eq!(saved_flags.borrow().len(), 1);
    }

    #[test]
    fn load_from_buffer_reads_the_parts_the_buffer_asks_for() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        quiet(
            &mut e,
            &[
                FORM_LOAD_FROM_BUFFER,
                CELL_SET_OWNER,
                EXTRA_LIST_SET_SEEN_DATA,
            ],
        );
        e.register(BUFFER_LOAD_BYTES, |e, a| {
            e.mem.set_u8(a[1], 0xff);
            Ret::default()
        });
        e.register(BUFFER_LOAD_STRING, |e, a| {
            e.mem.set_cstr(a[1], b"Foo");
            Ret::default()
        });
        e.register(BUFFER_LOAD_FORM_ID, |e, a| {
            e.mem.set_u32(a[1], 0x1234);
            Ret::default()
        });
        let names = Rc::new(RefCell::new(vec![]));
        let recorder = names.clone();
        e.register_double(FULL_NAME_SET, move |e, a| {
            recorder.borrow_mut().push((a[0], e.mem.cstr(a[1])));
            Ret::default()
        });
        let loads = Rc::new(RefCell::new(vec![]));
        let recorder = loads.clone();
        e.register_double(fake(0x14), move |_, a| {
            recorder.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        returns(&mut e, CELL_SEEN_DATA, 0);
        returns(&mut e, CELL_IS_INTERIOR, 0);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(EXTERIOR_SEEN_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], TEST_VTABLE);
            a[0].into_ret()
        });
        returns(&mut e, CELL_EXTRA_DATA_LIST, 0x6100);
        e.set(cell, TESObjectCELL::cCellFlags, 0x01);
        let buffer = buffer_flag_doubles(&mut e, 0xffff_ffff, 0);
        start_log(&mut e);
        e.call(0x0055_5740, &args![cell, buffer]);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, FORM_LOAD_FROM_BUFFER),
            vec![vec![cell.addr(), buffer]]
        );
        assert_eq!(e.get(cell, TESObjectCELL::cCellFlags), 0x01 | 0x60);
        assert_eq!(e.get(cell, TESObjectCELL::cCellGameFlags), 0x9f);
        let created = calls_to(&log, EXTERIOR_SEEN_DATA_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, EXTRA_LIST_SET_SEEN_DATA),
            vec![vec![0x6100, created]]
        );
        assert_eq!(*loads.borrow(), vec![vec![created, buffer]]);
        assert_eq!(*names.borrow(), vec![(cell.addr() + 0x18, b"Foo".to_vec())]);
        assert_eq!(
            calls_to(&log, CELL_SET_OWNER),
            vec![vec![cell.addr(), 0x1234]]
        );
        // No flags: only the base class part.
        e.mem.set_u32(buffer + 0x17, 0);
        start_log(&mut e);
        e.call(0x0055_5740, &args![cell, buffer]);
        assert_eq!(calls_to(&calls(&e), BUFFER_FLAGS_GET).len(), 4);
        assert!(calls_to(&calls(&e), CELL_SET_OWNER).is_empty());
    }

    #[test]
    fn buffer_functions_for_the_owner_and_the_cleared_data() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        quiet(
            &mut e,
            &[
                CELL_EMPTY_CALL,
                CELL_SET_OWNER,
                CELL_CLEAR_SEEN_DATA,
                CELL_RESET_DETACH_TIME,
            ],
        );
        returns(&mut e, CELL_OWNER, 0x7001);
        returns(&mut e, FORM_LOOK_UP, 0x8002);
        let buffer = buffer_flag_doubles(&mut e, 8, 0xc000_0000);
        start_log(&mut e);
        e.call(0x0055_59d0, &args![cell, buffer]);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, CELL_EMPTY_CALL),
            vec![vec![cell.addr(), buffer]]
        );
        assert_eq!(
            calls_to(&log, CELL_SET_OWNER),
            vec![vec![cell.addr(), 0x8002]]
        );
        // Not flagged, or without owner: nothing.
        e.mem.set_u32(buffer + 0x17, 7);
        start_log(&mut e);
        e.call(0x0055_59d0, &args![cell, buffer]);
        assert!(calls_to(&calls(&e), CELL_SET_OWNER).is_empty());
        e.mem.set_u32(buffer + 0x17, 8);
        returns(&mut e, CELL_OWNER, 0);
        start_log(&mut e);
        e.call(0x0055_59d0, &args![cell, buffer]);
        assert!(calls_to(&calls(&e), CELL_SET_OWNER).is_empty());
        // The clearing function reads the other flag word.
        e.set(cell, TESObjectCELL::cCellGameFlags, 9);
        start_log(&mut e);
        e.call(0x0055_5a50, &args![cell, buffer]);
        let log = calls(&e);
        assert_eq!(e.get(cell, TESObjectCELL::cCellGameFlags), 0);
        assert_eq!(
            calls_to(&log, CELL_CLEAR_SEEN_DATA),
            vec![vec![cell.addr()]]
        );
        assert_eq!(
            calls_to(&log, CELL_RESET_DETACH_TIME),
            vec![vec![cell.addr(), 0, 0]]
        );
        assert_eq!(calls_to(&log, BUFFER_FLAGS_GET_OTHER).len(), 2);
        e.mem.set_u32(buffer + 0x2c, 0);
        start_log(&mut e);
        e.call(0x0055_5a50, &args![cell, buffer]);
        assert!(calls_to(&calls(&e), CELL_CLEAR_SEEN_DATA).is_empty());
        assert!(calls_to(&calls(&e), CELL_RESET_DETACH_TIME).is_empty());
    }

    #[test]
    fn north_rotation_of_a_cell_comes_from_the_interior_extra_data() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        returns(&mut e, CELL_IS_INTERIOR, 0);
        assert_eq!(e.call(0x0055_5ad0, &args![cell]).f32(), 0.0);
        returns(&mut e, CELL_IS_INTERIOR, 1);
        returns(&mut e, CELL_EXTRA_DATA_LIST, 0x6100);
        e.register(EXTRA_LIST_GET_ROTATION, |_, a| {
            assert_eq!(a[0], 0x6100);
            0.75f32.into_ret()
        });
        assert_eq!(e.call(0x0055_5ad0, &args![cell]).f32(), 0.75);
    }

    /// Doubles for the rotation of coordinates.
    fn rotation_setup(e: &mut Engine, interior: bool, rotation: f32) -> Rc<RefCell<Vec<u32>>> {
        returns(e, CELL_IS_INTERIOR, u32::from(interior));
        returns(e, CELL_EXTRA_DATA_LIST, 0x6100);
        e.register_double(EXTRA_LIST_GET_ROTATION, move |_, _| rotation.into_ret());
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        e.set_global(ROTATION_EPSILON, 0.0f64);
        for index in 0..9u32 {
            e.mem.set_u32(IDENTITY_ROTATION + 4 * index, 0x100 + index);
        }
        let seen = Rc::new(RefCell::new(vec![]));
        let recorder = seen.clone();
        e.register_double(ROTATION_MATRIX_BUILD, move |e, a| {
            let mut seen = recorder.borrow_mut();
            seen.push(a[1]);
            seen.extend((0..9).map(|index| e.mem.u32(a[0] + 4 * index)));
            Ret::default()
        });
        e.register(ROTATE_VECTOR, |e, a| {
            for (index, value) in [1.5f32, 2.5, 3.5].into_iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * index as u32, value);
            }
            a[0].into_ret()
        });
        seen
    }

    #[test]
    fn north_rotation_copies_the_coordinates_when_the_angle_is_zero() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        let _ = rotation_setup(&mut e, false, 0.0);
        let source = e.mem.alloc(12);
        let destination = e.mem.alloc(12);
        put_floats(&mut e, source, &[10.0, 20.0, 30.0]);
        e.call(0x0055_5b10, &args![cell, source, destination, 0u32]);
        assert_eq!(floats(&e, destination, 3), vec![10.0, 20.0, 30.0]);
        // A zero angle also for the reversed sign.
        put_floats(&mut e, destination, &[0.0; 3]);
        e.call(0x0055_5b10, &args![cell, source, destination, 1u32]);
        assert_eq!(floats(&e, destination, 3), vec![10.0, 20.0, 30.0]);
    }

    #[test]
    fn north_rotation_rotates_with_a_matrix_built_from_the_identity() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        let seen = rotation_setup(&mut e, true, 0.5);
        let source = e.mem.alloc(12);
        let destination = e.mem.alloc(12);
        put_floats(&mut e, source, &[10.0, 20.0, 30.0]);
        e.call(0x0055_5b10, &args![cell, source, destination, 0u32]);
        assert_eq!(floats(&e, destination, 3), vec![1.5, 2.5, 3.5]);
        // The matrix was the copy of the identity at 011a9448, angle 0.5.
        let record = seen.borrow().clone();
        assert_eq!(record[0], 0.5f32.to_bits());
        assert_eq!(record[1..], (0..9).map(|i| 0x100 + i).collect::<Vec<u32>>());
        // Reversed: the angle is negated.
        seen.borrow_mut().clear();
        e.call(0x0055_5b10, &args![cell, source, destination, 1u32]);
        assert_eq!(seen.borrow()[0], (-0.5f32).to_bits());
    }

    #[test]
    fn word_tests_of_the_cell_helpers() {
        let mut e = engine();
        let at = e.mem.alloc(0x100);
        assert!(!e.call(0x0055_70b0, &args![at]).bool());
        e.mem.set_u32(at, 5);
        assert!(e.call(0x0055_70b0, &args![at]).bool());
        assert!(!e.call(0x0055_7090, &args![at]).bool());
        e.mem.set_u32(at + 0x80, 0x100);
        assert!(e.call(0x0055_7090, &args![at]).bool());
    }

    /// A seen-data section object: `x`, `y` at `+0x24`, `+0x25`, next at
    /// `+0x28`.
    fn section(e: &mut Engine, x: i8, y: i8) -> u32 {
        let at = e.mem.alloc(0x30);
        e.mem.set_i8(at + 0x24, x);
        e.mem.set_i8(at + 0x25, y);
        at
    }

    type BitSet = Rc<RefCell<std::collections::HashSet<(u32, u32, u32)>>>;

    /// Doubles shared by the `GetSeenValue` tests: the player's cell, the
    /// float conversion, the bit index and the queries that record which
    /// `(section, x, y)` were asked.
    fn seen_value_setup(
        e: &mut Engine,
        cell: Ptr<TESObjectCELL>,
        bits: (u32, u32),
        interior: bool,
    ) -> (BitSet, BitQueries) {
        e.set_global(PLAYER_OBJECT, 0x5000u32);
        returns(e, REFERENCE_PARENT_CELL, cell.addr());
        returns(e, CELL_IS_INTERIOR, u32::from(interior));
        e.register(FLOAT_TO_INT, |_, a| {
            (f32::from_bits(a[0]) as i32).into_ret()
        });
        e.register_double(SEEN_DATA_INDEX, move |e, a| {
            assert_eq!(a[1], u32::from(interior));
            e.mem.set_u32(a[2], bits.0);
            e.mem.set_u32(a[3], bits.1);
            Ret::default()
        });
        let set: BitSet = Default::default();
        let asked = Rc::new(RefCell::new(vec![]));
        let (lookup, recorder) = (set.clone(), asked.clone());
        e.register_double(SEEN_BIT_IS_SET, move |_, a| {
            recorder.borrow_mut().push((a[0], a[1], a[2]));
            u32::from(lookup.borrow().contains(&(a[0], a[1], a[2]))).into_ret()
        });
        (set, asked)
    }

    /// Four sections around `(2, 3)` as a chain of seen data; returns them as
    /// `[(2,3), (2,4), (3,3), (3,4)]` (0 for an absent one).
    fn interior_sections(e: &mut Engine, cell: Ptr<TESObjectCELL>, present: [bool; 4]) -> [u32; 4] {
        returns(e, CELL_IS_INTERIOR, 1);
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        let coordinates = [(2, 3), (2, 4), (3, 3), (3, 4)];
        let mut sections = [0; 4];
        for index in 0..4 {
            if present[index] {
                sections[index] = section(e, coordinates[index].0, coordinates[index].1);
            }
        }
        let chain: Vec<u32> = sections.iter().copied().filter(|s| *s != 0).collect();
        for pair in chain.windows(2) {
            e.mem.set_u32(pair[0] + 0x28, pair[1]);
        }
        returns(e, CELL_SEEN_DATA, chain.first().copied().unwrap_or(0));
        let _ = cell;
        sections
    }

    /// An interior position inside the section `(2, 3)`.
    fn interior_position(e: &mut Engine) -> u32 {
        let position = e.mem.alloc(12);
        put_floats(
            e,
            position,
            &[
                0x800 as f32 + 2.0 * 4096.0 + 100.0,
                0x800 as f32 + 3.0 * 4096.0 + 7.0,
                0.0,
            ],
        );
        position
    }

    #[test]
    fn seen_value_without_a_player_cell_is_zero() {
        let mut e = engine();
        returns(&mut e, REFERENCE_PARENT_CELL, 0);
        let position = e.mem.alloc(12);
        assert_eq!(e.call(0x0055_6870, &args![position]).i32(), 0);
        assert_eq!(
            addresses(&[(REFERENCE_PARENT_CELL, vec![])]),
            vec![REFERENCE_PARENT_CELL]
        );
    }

    #[test]
    fn interior_seen_value_inside_a_tile_asks_one_section() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        let (set, asked) = seen_value_setup(&mut e, cell, (3, 4), true);
        let sections = interior_sections(&mut e, cell, [true, false, false, false]);
        let position = interior_position(&mut e);
        set.borrow_mut().insert((sections[0], 3, 4));
        set.borrow_mut().insert((sections[0], 4, 5));
        set.borrow_mut().insert((sections[0], 3, 5));
        assert_eq!(e.call(0x0055_6870, &args![position]).i32(), 3);
        assert_eq!(
            *asked.borrow(),
            vec![
                (sections[0], 3, 4),
                (sections[0], 3, 5),
                (sections[0], 4, 4),
                (sections[0], 4, 5)
            ]
        );
    }

    #[test]
    fn interior_seen_value_at_the_corner_of_a_tile_asks_four_sections() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        let (set, asked) = seen_value_setup(&mut e, cell, (15, 15), true);
        let sections = interior_sections(&mut e, cell, [true; 4]);
        let position = interior_position(&mut e);
        for query in [
            (sections[0], 15, 15),
            (sections[1], 15, 0),
            (sections[3], 0, 0),
        ] {
            set.borrow_mut().insert(query);
        }
        assert_eq!(e.call(0x0055_6870, &args![position]).i32(), 3);
        assert_eq!(
            *asked.borrow(),
            vec![
                (sections[0], 15, 15),
                (sections[1], 15, 0),
                (sections[2], 0, 15),
                (sections[3], 0, 0)
            ]
        );
        // A missing neighbour tile is skipped without a query.
        let (_, asked) = seen_value_setup(&mut e, cell, (15, 15), true);
        let sections = interior_sections(&mut e, cell, [true, true, false, false]);
        assert_eq!(e.call(0x0055_6870, &args![position]).i32(), 0);
        assert_eq!(
            *asked.borrow(),
            vec![(sections[0], 15, 15), (sections[1], 15, 0)]
        );
    }

    #[test]
    fn interior_seen_value_at_one_edge_asks_two_sections() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        let position = interior_position(&mut e);
        // Right edge: the tile to the right is asked for the column 0.
        let (_, asked) = seen_value_setup(&mut e, cell, (15, 2), true);
        let sections = interior_sections(&mut e, cell, [true, false, true, false]);
        e.call(0x0055_6870, &args![position]);
        assert_eq!(
            *asked.borrow(),
            vec![
                (sections[0], 15, 2),
                (sections[0], 15, 3),
                (sections[2], 0, 2),
                (sections[2], 0, 3)
            ]
        );
        // Bottom edge: the tile below is asked for the row 0.
        let (set, asked) = seen_value_setup(&mut e, cell, (2, 15), true);
        let sections = interior_sections(&mut e, cell, [true, true, false, false]);
        set.borrow_mut().insert((sections[1], 3, 0));
        assert_eq!(e.call(0x0055_6870, &args![position]).i32(), 1);
        assert_eq!(
            *asked.borrow(),
            vec![
                (sections[0], 2, 15),
                (sections[1], 2, 0),
                (sections[0], 3, 15),
                (sections[1], 3, 0)
            ]
        );
    }

    /// An exterior world: cells by coordinates.
    struct Cells {
        seen: Rc<RefCell<std::collections::HashMap<u32, u32>>>,
        flagged: Rc<RefCell<std::collections::HashSet<u32>>>,
        asked: Rc<RefCell<Vec<(u32, u32)>>>,
    }

    fn exterior_cells(e: &mut Engine, coordinates: &[(i32, i32)]) -> (Cells, Vec<u32>) {
        let cells: Vec<u32> = coordinates.iter().map(|_| e.mem.alloc(0x10)).collect();
        let by_coordinates: std::collections::HashMap<(i32, i32), u32> = coordinates
            .iter()
            .copied()
            .zip(cells.iter().copied())
            .collect();
        let asked = Rc::new(RefCell::new(vec![]));
        let recorder = asked.clone();
        e.register_double(WORLD_SPACE_GET_CELL, move |_, a| {
            assert_eq!(a[0], 0x7777);
            recorder.borrow_mut().push((a[1], a[2]));
            by_coordinates
                .get(&(a[1] as i32, a[2] as i32))
                .copied()
                .unwrap_or(0)
                .into_ret()
        });
        let seen: Rc<RefCell<std::collections::HashMap<u32, u32>>> = Default::default();
        let lookup = seen.clone();
        e.register_double(CELL_SEEN_DATA, move |_, a| {
            lookup.borrow().get(&a[0]).copied().unwrap_or(0).into_ret()
        });
        let flagged: Rc<RefCell<std::collections::HashSet<u32>>> = Default::default();
        let lookup = flagged.clone();
        e.register_double(CELL_SEEN_FLAG, move |_, a| {
            u32::from(lookup.borrow().contains(&a[0])).into_ret()
        });
        (
            Cells {
                seen,
                flagged,
                asked,
            },
            cells,
        )
    }

    fn exterior_position(e: &mut Engine) -> u32 {
        let position = e.mem.alloc(12);
        put_floats(
            e,
            position,
            &[2.0 * 4096.0 + 100.0, 3.0 * 4096.0 + 7.0, 0.0],
        );
        position
    }

    #[test]
    fn exterior_seen_value_counts_seen_bits_and_flagged_cells() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        returns(&mut e, CELL_GET_WORLD_SPACE, 0x7777);
        let (set, asked) = seen_value_setup(&mut e, cell, (3, 4), false);
        let (cells, ids) = exterior_cells(&mut e, &[(2, 3)]);
        let seen_object = 0x6000u32;
        cells.seen.borrow_mut().insert(ids[0], seen_object);
        set.borrow_mut().insert((seen_object, 3, 5));
        set.borrow_mut().insert((seen_object, 4, 4));
        let position = exterior_position(&mut e);
        assert_eq!(e.call(0x0055_6870, &args![position]).i32(), 2);
        assert_eq!(
            *asked.borrow(),
            vec![
                (seen_object, 3, 4),
                (seen_object, 3, 5),
                (seen_object, 4, 4),
                (seen_object, 4, 5)
            ]
        );
        assert_eq!(*cells.asked.borrow(), vec![(2, 3)]);
        // A flagged cell counts for all four corners, even without seen data.
        cells.seen.borrow_mut().clear();
        cells.flagged.borrow_mut().insert(ids[0]);
        assert_eq!(e.call(0x0055_6870, &args![position]).i32(), 4);
    }

    #[test]
    fn exterior_seen_value_at_the_corner_asks_the_four_neighbour_cells() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        returns(&mut e, CELL_GET_WORLD_SPACE, 0x7777);
        let (set, asked) = seen_value_setup(&mut e, cell, (15, 15), false);
        let (cells, ids) = exterior_cells(&mut e, &[(2, 3), (2, 4), (3, 3), (3, 4)]);
        // (2,3): flagged only; (2,4): neither; (3,3): seen bit (0,15);
        // (3,4): flagged.
        cells.flagged.borrow_mut().insert(ids[0]);
        cells.flagged.borrow_mut().insert(ids[3]);
        cells.seen.borrow_mut().insert(ids[2], 0x6003);
        set.borrow_mut().insert((0x6003, 0, 15));
        let position = exterior_position(&mut e);
        assert_eq!(e.call(0x0055_6870, &args![position]).i32(), 3);
        assert_eq!(*cells.asked.borrow(), vec![(2, 3), (2, 4), (3, 3), (3, 4)]);
        // Only the third cell has seen data, so only it is asked for a bit.
        assert_eq!(*asked.borrow(), vec![(0x6003, 0, 15)]);
        // Missing cells count nothing.
        let (cells, _) = exterior_cells(&mut e, &[(2, 3)]);
        assert_eq!(e.call(0x0055_6870, &args![position]).i32(), 0);
        assert_eq!(cells.asked.borrow().len(), 4);
    }

    #[test]
    fn exterior_seen_value_at_one_edge_asks_the_neighbour_cell() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        returns(&mut e, CELL_GET_WORLD_SPACE, 0x7777);
        let position = exterior_position(&mut e);
        // Right edge: (3, 3) is asked for the column 0.
        let (set, asked) = seen_value_setup(&mut e, cell, (15, 2), false);
        let (cells, ids) = exterior_cells(&mut e, &[(2, 3), (3, 3)]);
        cells.seen.borrow_mut().insert(ids[0], 0x6001);
        cells.seen.borrow_mut().insert(ids[1], 0x6002);
        set.borrow_mut().insert((0x6002, 0, 3));
        assert_eq!(e.call(0x0055_6870, &args![position]).i32(), 1);
        assert_eq!(
            *asked.borrow(),
            vec![
                (0x6001, 15, 2),
                (0x6001, 15, 3),
                (0x6002, 0, 2),
                (0x6002, 0, 3)
            ]
        );
        // Bottom edge: (2, 4) is asked for the row 0.
        let (set, asked) = seen_value_setup(&mut e, cell, (2, 15), false);
        let (cells, ids) = exterior_cells(&mut e, &[(2, 3), (2, 4)]);
        cells.seen.borrow_mut().insert(ids[0], 0x6001);
        cells.seen.borrow_mut().insert(ids[1], 0x6002);
        set.borrow_mut().insert((0x6002, 3, 0));
        assert_eq!(e.call(0x0055_6870, &args![position]).i32(), 1);
        assert_eq!(
            *asked.borrow(),
            vec![
                (0x6001, 2, 15),
                (0x6002, 2, 0),
                (0x6001, 3, 15),
                (0x6002, 3, 0)
            ]
        );
        // Without a world space an exterior cell counts nothing.
        returns(&mut e, CELL_GET_WORLD_SPACE, 0);
        assert_eq!(e.call(0x0055_6870, &args![position]).i32(), 0);
    }

    #[test]
    fn int_seen_section_finds_creates_and_links_sections() {
        let mut e = engine();
        let cell: Ptr<TESObjectCELL> = Ptr::new(object(&mut e, 0xe0));
        // An exterior cell has none.
        returns(&mut e, CELL_IS_INTERIOR, 0);
        start_log(&mut e);
        assert_eq!(e.call(0x0055_6ef0, &args![cell, 1u32, 2u32, 1u32]).u32(), 0);
        assert_eq!(addresses(&calls(&e)), vec![CELL_IS_INTERIOR]);
        // Interior: the chain A(-1, 2) -> B(3, 4).
        returns(&mut e, CELL_IS_INTERIOR, 1);
        let a = section(&mut e, -1, 2);
        let b = section(&mut e, 3, 4);
        e.mem.set_u32(a + 0x28, b);
        returns(&mut e, CELL_SEEN_DATA, a);
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        start_log(&mut e);
        assert_eq!(
            e.call(0x0055_6ef0, &args![cell, -1i32, 2i32, 0u32]).u32(),
            a
        );
        assert_eq!(
            calls_to(&calls(&e), RT_DYNAMIC_CAST),
            vec![vec![a, 0, SEEN_DATA_CAST_SOURCE, SEEN_DATA_CAST_TARGET, 0]]
        );
        assert_eq!(e.call(0x0055_6ef0, &args![cell, 3i32, 4i32, 1u32]).u32(), b);
        // Not found, not created.
        assert_eq!(e.call(0x0055_6ef0, &args![cell, 9i32, 9i32, 0u32]).u32(), 0);
        // Not found, created after the last section.
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(INTERIOR_SEEN_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        start_log(&mut e);
        let created = e.call(0x0055_6ef0, &args![cell, -5i32, 9i32, 1u32]).u32();
        assert_ne!(created, 0);
        assert_eq!(e.mem.u32(b + 0x28), created);
        let log = calls(&e);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x2c]]);
        assert_eq!(
            calls_to(&log, INTERIOR_SEEN_DATA_CONSTRUCT),
            vec![vec![created, 0xfb, 9]]
        );
        assert!(calls_to(&log, EXTRA_LIST_SET_SEEN_DATA).is_empty());
    }

    #[test]
    fn int_seen_section_without_seen_data_stores_the_first_section() {
        let mut e = engine();
        let cell: Ptr<TESObjectCELL> = Ptr::new(object(&mut e, 0xe0));
        returns(&mut e, CELL_IS_INTERIOR, 1);
        returns(&mut e, CELL_SEEN_DATA, 0);
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(INTERIOR_SEEN_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        returns(&mut e, CELL_EXTRA_DATA_LIST, 0x6100);
        quiet(&mut e, &[EXTRA_LIST_SET_SEEN_DATA]);
        let changed = Rc::new(RefCell::new(vec![]));
        let recorder = changed.clone();
        e.register_double(fake(0x48), move |_, a| {
            recorder.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        // Without creation: nothing.
        assert_eq!(e.call(0x0055_6ef0, &args![cell, 1u32, 2u32, 0u32]).u32(), 0);
        assert!(changed.borrow().is_empty());
        start_log(&mut e);
        let created = e.call(0x0055_6ef0, &args![cell, 1u32, 2u32, 1u32]).u32();
        assert_ne!(created, 0);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, EXTRA_LIST_SET_SEEN_DATA),
            vec![vec![0x6100, created]]
        );
        assert_eq!(*changed.borrow(), vec![vec![cell.addr(), 0x8000_0000]]);
    }

    /// The cell lock and the reference list of a cell with `items`.
    fn reference_list_setup(e: &mut Engine, items: &[u32]) -> Ptr<TESObjectCELL> {
        let cell = e.new_object::<TESObjectCELL>();
        list_doubles(e);
        quiet(e, &[CELL_LOCK_ENTER, CELL_LOCK_LEAVE]);
        let first = list_of(e, items);
        returns(e, CELL_REFERENCES, first);
        cell
    }

    #[test]
    fn reference_loops_run_under_the_cell_lock() {
        let mut e = engine();
        let cell = reference_list_setup(&mut e, &[]);
        for entry in [0x0055_70d0u32, 0x0055_73e0, 0x0055_7470, 0x0055_74d0] {
            start_log(&mut e);
            e.call(entry, &args![cell]);
            assert_eq!(
                addresses(&calls(&e)),
                vec![CELL_LOCK_ENTER, CELL_REFERENCES, CELL_LOCK_LEAVE],
                "{entry:#x}"
            );
        }
    }

    #[test]
    fn reference_owner_objects_are_called_for_the_references_of_form_type_0x25() {
        let mut e = engine();
        // `+0x34` owner, `+0x3c` form type of the base object (the base
        // object of a test reference is the reference itself), `+0x40` the
        // answer of `005651e0`.
        let owner_target = reference_object(&mut e);
        let owner = reference_object(&mut e);
        e.mem.set_u32(owner + 0x38, owner_target);
        let calls_on_target = Rc::new(RefCell::new(vec![]));
        let recorder = calls_on_target.clone();
        e.register_double(fake(0x08), move |_, a| {
            recorder.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let make = |e: &mut Engine, form_type: u32, owner: u32, rejected: u32| {
            let at = reference_object(e);
            e.mem.set_u32(at + 0x34, owner);
            e.mem.set_u32(at + 0x3c, form_type);
            e.mem.set_u32(at + 0x40, rejected);
            at
        };
        let skipped_by_test = make(&mut e, 0x25, owner, 1);
        let wrong_type = make(&mut e, 0x24, owner, 0);
        let without_owner = make(&mut e, 0x25, 0, 0);
        let working = make(&mut e, 0x25, owner, 0);
        let cell = reference_list_setup(
            &mut e,
            &[0, skipped_by_test, wrong_type, without_owner, working],
        );
        e.register(REFERENCE_TEST, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.register(REFERENCE_BASE_OBJECT, |_, a| a[0].into_ret());
        e.register(FORM_TYPE, |e, a| e.mem.u32(a[0] + 0x3c).into_ret());
        start_log(&mut e);
        e.call(0x0055_70d0, &args![cell]);
        let log = calls(&e);
        // The owner of the working reference is asked twice (the game does).
        assert_eq!(calls_to(&log, fake(0x1d0)).len(), 1 + 2);
        assert_eq!(*calls_on_target.borrow(), vec![vec![owner_target, owner]]);
        assert_eq!(calls_to(&log, REFERENCE_TEST).len(), 4);
    }

    /// A reference for `QueueReferences`: `+0x20` base object, `+0x34`
    /// owner, `+0x44`.. flags of `00440da0`, `00440d80`, `0056b250` and the
    /// answer of `005651e0` at `+0x50`.
    fn queue_reference(e: &mut Engine, base: u32, owner: u32, flags: [u32; 4]) -> u32 {
        let at = object(e, 0x60);
        e.mem.set_u32(at + 0x20, base);
        e.mem.set_u32(at + 0x34, owner);
        for (index, flag) in flags.into_iter().enumerate() {
            e.mem.set_u32(at + 0x44 + 4 * index as u32, flag);
        }
        at
    }

    #[test]
    fn queue_references_goes_through_the_marker_forms_first() {
        let mut e = engine();
        e.set_global(MARKER_FORM_FIRST, 0xaaa1u32);
        e.set_global(MARKER_FORM_SECOND, 0xaaa2u32);
        e.set_global(SCENE_SINGLETON, 0x9900u32);
        e.set_global(MODEL_LOADER_SINGLETON, 0x9a00u32);
        returns(&mut e, CELL_PRIORITY, 0x55);
        e.register(REFERENCE_BASE_OBJECT, |e, a| {
            e.mem.u32(a[0] + 0x20).into_ret()
        });
        e.register(fake(0x1d0), |e, a| e.mem.u32(a[0] + 0x34).into_ret());
        e.register(REFERENCE_FLAG_TEST_A, |e, a| {
            e.mem.u32(a[0] + 0x44).into_ret()
        });
        e.register(REFERENCE_FLAG_TEST_B, |e, a| {
            e.mem.u32(a[0] + 0x48).into_ret()
        });
        e.register(REFERENCE_FLAG_TEST_C, |e, a| {
            e.mem.u32(a[0] + 0x4c).into_ret()
        });
        e.register(REFERENCE_TEST, |e, a| e.mem.u32(a[0] + 0x50).into_ret());
        quiet(&mut e, &[MODEL_LOADER_QUEUE_REFERENCE]);
        let deferred = Rc::new(RefCell::new(vec![]));
        let recorder = deferred.clone();
        e.register_double(fake(0x1cc), move |_, a| {
            recorder.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        // [flagA, flagB, flagC, test]
        let r1 = queue_reference(&mut e, 0xaaa1, 0, [0, 0, 1, 0]);
        let r2 = queue_reference(&mut e, 0x1, 0, [1, 0, 1, 0]);
        let r3 = queue_reference(&mut e, 0x1, 5, [0, 1, 0, 0]);
        let r4 = queue_reference(&mut e, 0xaaa2, 7, [1, 0, 0, 0]);
        let r5 = queue_reference(&mut e, 0x1, 0, [0, 0, 0, 0]);
        let r6 = queue_reference(&mut e, 0x1, 5, [0, 0, 0, 1]);
        let cell = reference_list_setup(&mut e, &[r1, r2, r3, r4, r5, 0, r6]);
        start_log(&mut e);
        e.call(0x0055_71a0, &args![cell, 1u32]);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, CELL_PRIORITY),
            vec![vec![0x9900, cell.addr(), 0]]
        );
        // With the check: a reference must also pass `0056b250`, which only
        // the first one does not: r1 has its flag C set, r5 and r6 not.
        let queued: Vec<u32> = calls_to(&log, MODEL_LOADER_QUEUE_REFERENCE)
            .iter()
            .map(|call| call[1])
            .collect();
        assert_eq!(queued, vec![r1]);
        for call in calls_to(&log, MODEL_LOADER_QUEUE_REFERENCE) {
            assert_eq!(call[0], 0x9a00);
            assert_eq!(call[2..], [0x55, 1]);
        }
        // r4 (marker, owner, not rejected, flagged): first pass, `(0, 1)`;
        // r3 (flagged B) in the second pass: `(0, 0)`.
        assert_eq!(*deferred.borrow(), vec![vec![r4, 0, 1], vec![r3, 0, 0]]);
        // Without the check, the unflagged references are queued too, in the
        // order of the passes: marker forms first, then the others.
        deferred.borrow_mut().clear();
        start_log(&mut e);
        e.call(0x0055_71a0, &args![cell, 0u32]);
        let log = calls(&e);
        let queued: Vec<u32> = calls_to(&log, MODEL_LOADER_QUEUE_REFERENCE)
            .iter()
            .map(|call| call[1])
            .collect();
        assert_eq!(queued, vec![r1, r5, r6]);
        assert_eq!(*deferred.borrow(), vec![vec![r4, 0, 1], vec![r3, 0, 0]]);
        assert_eq!(calls_to(&log, CELL_LOCK_LEAVE).len(), 1);
    }

    #[test]
    fn models_of_the_references_are_prepared_and_made_opaque() {
        let mut e = engine();
        e.register(REFERENCE_MODEL, |e, a| e.mem.u32(a[0] + 0x30).into_ret());
        e.register(fake(0x10), |e, a| e.mem.u32(a[0] + 0x30).into_ret());
        quiet(&mut e, &[MODEL_PREPARE, SET_PROPERTY_FADE_ALPHA]);
        let model_off = object(&mut e, 0x40);
        let model_on = object(&mut e, 0x40);
        e.mem.set_u32(model_on + 0x30, 1);
        let reference = |e: &mut Engine, model: u32| {
            let at = e.mem.alloc(0x40);
            e.mem.set_u32(at + 0x30, model);
            at
        };
        let items = [
            0,
            reference(&mut e, 0),
            reference(&mut e, model_off),
            reference(&mut e, model_on),
        ];
        let cell = reference_list_setup(&mut e, &items);
        start_log(&mut e);
        e.call(0x0055_73e0, &args![cell]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, MODEL_PREPARE), vec![vec![model_on]]);
        assert_eq!(
            calls_to(&log, SET_PROPERTY_FADE_ALPHA),
            vec![vec![model_on, 1.0f32.to_bits()]]
        );
    }

    #[test]
    fn references_of_the_cell_are_handed_to_the_loaded_collection() {
        let mut e = engine();
        e.set_global(LOADED_REFERENCES, 0xc0deu32);
        quiet(&mut e, &[LOADED_REFERENCES_REMOVE]);
        let cell = reference_list_setup(&mut e, &[0, 0x31, 0x32]);
        start_log(&mut e);
        e.call(0x0055_7470, &args![cell]);
        assert_eq!(
            calls_to(&calls(&e), LOADED_REFERENCES_REMOVE),
            vec![vec![0xc0de, 0x31], vec![0xc0de, 0x32]]
        );
    }

    /// Doubles for the terrain updates; the reference flags are `+0x30`
    /// (`00440da0`). The terrain manager of every world is `0x7a`.
    fn terrain_setup(e: &mut Engine) {
        e.set_global(LOADED_REFERENCES, 0xc0deu32);
        e.set_global(SCENE_SINGLETON, 0x9900u32);
        quiet(
            e,
            &[
                LOADED_REFERENCES_ADD,
                TERRAIN_HIDE_TREE,
                TERRAIN_UPDATE_TREE,
                TERRAIN_SET_FLAG,
            ],
        );
        returns(e, SCENE_CELL, 0);
        returns(e, SCENE_WORLD_SPACE, 1);
        returns(e, REFERENCE_BASE_OBJECT, 0xba5e);
        returns(e, BASE_FORM_FLAG_40, 1);
        returns(e, REFERENCE_WORLD_SPACE, 0x3300);
        returns(e, WORLD_GET_TERRAIN_MANAGER, 0x7a);
        e.register(REFERENCE_FLAG_TEST_A, |e, a| {
            e.mem.u32(a[0] + 0x30).into_ret()
        });
    }

    #[test]
    fn terrain_is_told_about_every_reference_of_an_exterior_scene() {
        let mut e = engine();
        terrain_setup(&mut e);
        let plain = e.mem.alloc(0x40);
        let hidden = e.mem.alloc(0x40);
        e.mem.set_u32(hidden + 0x30, 1);
        let cell = reference_list_setup(&mut e, &[0, plain, hidden]);
        start_log(&mut e);
        e.call(0x0055_74d0, &args![cell]);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, LOADED_REFERENCES_ADD),
            vec![vec![0xc0de, plain], vec![0xc0de, hidden]]
        );
        assert_eq!(calls_to(&log, SCENE_CELL), vec![vec![0x9900], vec![0x9900]]);
        assert_eq!(calls_to(&log, TERRAIN_UPDATE_TREE), vec![vec![0x7a, plain]]);
        assert_eq!(
            calls_to(&log, TERRAIN_HIDE_TREE),
            vec![vec![0x7a, hidden, 1]]
        );
        assert_eq!(
            calls_to(&log, TERRAIN_SET_FLAG),
            vec![vec![0x7a, 1], vec![0x7a, 1]]
        );
        assert_eq!(
            calls_to(&log, WORLD_GET_TERRAIN_MANAGER),
            vec![vec![0x3300]; 4]
        );
    }

    #[test]
    fn terrain_is_left_alone_when_the_scene_or_the_reference_does_not_qualify() {
        let reference_at = |e: &mut Engine| e.mem.alloc(0x40);
        for step in 0..4 {
            let mut e = engine();
            terrain_setup(&mut e);
            match step {
                0 => returns(&mut e, SCENE_CELL, 0x44),
                1 => returns(&mut e, SCENE_WORLD_SPACE, 0),
                2 => returns(&mut e, BASE_FORM_FLAG_40, 0),
                _ => returns(&mut e, REFERENCE_WORLD_SPACE, 0),
            }
            let reference = reference_at(&mut e);
            let cell = reference_list_setup(&mut e, &[reference]);
            start_log(&mut e);
            e.call(0x0055_74d0, &args![cell]);
            let log = calls(&e);
            assert_eq!(calls_to(&log, LOADED_REFERENCES_ADD).len(), 1, "{step}");
            assert!(calls_to(&log, TERRAIN_UPDATE_TREE).is_empty(), "{step}");
            assert!(calls_to(&log, TERRAIN_HIDE_TREE).is_empty(), "{step}");
            assert!(calls_to(&log, TERRAIN_SET_FLAG).is_empty(), "{step}");
        }
    }

    // -----------------------------------------------------------------------
    // Third batch.

    /// A reference for the obstacle walks: `+0x30` is the answer of virtual
    /// `+0x15c`, `+0x34` the base object, `+0x38` the open state; a base
    /// object holds its form type at `+4` and the sliding flag at `+8`.
    fn obstacle_reference(
        e: &mut Engine,
        obstacle: bool,
        base_type: u8,
        open_state: u32,
        sliding: bool,
    ) -> u32 {
        let base = e.mem.alloc(0x10);
        e.mem.set_u8(base + 4, base_type);
        e.mem.set_u8(base + 8, u8::from(sliding));
        let at = object(e, 0x60);
        e.mem.set_u8(at + 0x30, u8::from(obstacle));
        e.mem.set_u32(at + 0x34, base);
        e.mem.set_u32(at + 0x38, open_state);
        at
    }

    fn obstacle_setup(e: &mut Engine) {
        e.register(fake(0x15c), |e, a| e.mem.u8(a[0] + 0x30).into_ret());
        e.register(REFERENCE_BASE_OBJECT, |e, a| {
            e.mem.u32(a[0] + 0x34).into_ret()
        });
        e.register(FORM_TYPE, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        e.register(GET_OPEN_STATE, |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        e.register(IS_SLIDING_DOOR, |e, a| e.mem.u8(a[0] + 8).into_ret());
        e.register(OBSTACLE_MANAGER_GET, |_, a| {
            assert!(a.is_empty());
            0x0b5au32.into_ret()
        });
        quiet(
            e,
            &[
                OBSTACLE_ADD,
                OBSTACLE_REMOVE,
                OBSTACLE_DOOR_CLOSED,
                OBSTACLE_DOOR_REMOVE,
            ],
        );
    }

    /// The references of the two obstacle walks, in list order: a null item,
    /// a plain reference, one with an obstacle, a door with an obstacle that
    /// is open and sliding, an open door, an open sliding door and a closed
    /// door.
    fn obstacle_cell(e: &mut Engine) -> (Ptr<TESObjectCELL>, [u32; 6]) {
        obstacle_setup(e);
        let plain = obstacle_reference(e, false, 5, 0, false);
        let flagged = obstacle_reference(e, true, 5, 0, false);
        let door_flagged = obstacle_reference(e, true, 0x1c, 1, true);
        let open_door = obstacle_reference(e, false, 0x1c, 1, false);
        let open_sliding = obstacle_reference(e, false, 0x1c, 1, true);
        let closed = obstacle_reference(e, false, 0x1c, 0, false);
        let cell = reference_list_setup(
            e,
            &[
                0,
                plain,
                flagged,
                door_flagged,
                open_door,
                open_sliding,
                closed,
            ],
        );
        (
            cell,
            [
                flagged,
                door_flagged,
                open_door,
                open_sliding,
                closed,
                plain,
            ],
        )
    }

    #[test]
    fn obstacles_are_added_for_flagged_references_and_open_doors() {
        let mut e = engine();
        let (cell, [flagged, door_flagged, open_door, _open_sliding, closed, _]) =
            obstacle_cell(&mut e);
        start_log(&mut e);
        e.call(0x0055_75d0, &args![cell]);
        let log = calls(&e);
        assert_eq!(addresses(&log)[0], CELL_LOCK_ENTER);
        assert_eq!(*addresses(&log).last().unwrap(), CELL_LOCK_LEAVE);
        assert_eq!(
            calls_to(&log, OBSTACLE_ADD),
            vec![
                vec![0x0b5a, flagged],
                vec![0x0b5a, door_flagged],
                vec![0x0b5a, open_door]
            ]
        );
        assert_eq!(
            calls_to(&log, OBSTACLE_DOOR_CLOSED),
            vec![vec![0x0b5a, closed]]
        );
        // The sliding open doors are asked, not added.
        assert_eq!(calls_to(&log, IS_SLIDING_DOOR).len(), 3);
        assert!(calls_to(&log, OBSTACLE_REMOVE).is_empty());
    }

    #[test]
    fn obstacles_are_removed_for_flagged_references_and_doors() {
        let mut e = engine();
        let (cell, [flagged, door_flagged, open_door, open_sliding, closed, _]) =
            obstacle_cell(&mut e);
        start_log(&mut e);
        e.call(0x0055_76c0, &args![cell]);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, OBSTACLE_REMOVE),
            vec![vec![0x0b5a, flagged], vec![0x0b5a, door_flagged]]
        );
        assert_eq!(
            calls_to(&log, OBSTACLE_DOOR_REMOVE),
            vec![
                vec![0x0b5a, door_flagged],
                vec![0x0b5a, open_door],
                vec![0x0b5a, open_sliding],
                vec![0x0b5a, closed]
            ]
        );
        assert!(calls_to(&log, OBSTACLE_ADD).is_empty());
        assert_eq!(*addresses(&log).last().unwrap(), CELL_LOCK_LEAVE);
    }

    #[test]
    fn the_held_pointer_is_released_before_it_is_replaced() {
        let mut e = engine();
        quiet(&mut e, &[RELEASE_HELD]);
        let at = e.mem.alloc(0x80);
        start_log(&mut e);
        e.call(0x0055_7760, &args![at, 0x1111u32]);
        assert_eq!(calls(&e), vec![]);
        assert_eq!(e.mem.u32(at + 0x64), 0x1111);
        start_log(&mut e);
        e.call(0x0055_7760, &args![at, 0x2222u32]);
        assert_eq!(calls(&e), vec![(RELEASE_HELD, vec![0x1111, 1])]);
        assert_eq!(e.mem.u32(at + 0x64), 0x2222);
        e.call(0x0055_7760, &args![at, 0u32]);
        assert_eq!(e.mem.u32(at + 0x64), 0);
    }

    #[test]
    fn addon_nodes_are_added_or_removed_for_references_with_a_3d() {
        let mut e = engine();
        e.register(REFERENCE_ADDON_TEST, |_, a| u32::from(a[1] == 2).into_ret());
        e.register(fake(0x1d0), |e, a| e.mem.u32(a[0] + 0x34).into_ret());
        e.register(fake(0x0c), |e, a| e.mem.u32(a[0] + 0x34).into_ret());
        quiet(&mut e, &[ADD_ADDON_NODES, REMOVE_ADDON_NODES]);
        let node = object(&mut e, 0x40);
        e.mem.set_u32(node + 0x34, 0xabc);
        let with_3d = object(&mut e, 0x60);
        e.mem.set_u32(with_3d + 0x34, node);
        let without_3d = object(&mut e, 0x60);
        let cell = reference_list_setup(&mut e, &[0, without_3d, with_3d]);
        start_log(&mut e);
        e.call(0x0055_77b0, &args![cell, 1u32]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, ADD_ADDON_NODES), vec![vec![0xabc]]);
        assert!(calls_to(&log, REMOVE_ADDON_NODES).is_empty());
        // The list is advanced before the item is handled.
        assert_eq!(
            addresses(&log)[..5],
            [
                CELL_LOCK_ENTER,
                CELL_REFERENCES,
                LIST_NODE_ITEM_ADDRESS,
                LIST_NODE_NEXT,
                LIST_NODE_ITEM_ADDRESS
            ]
        );
        // The 3D is asked twice for the reference that has one.
        assert_eq!(calls_to(&log, fake(0x1d0)).len(), 3);
        start_log(&mut e);
        e.call(0x0055_77b0, &args![cell, 0u32]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, REMOVE_ADDON_NODES), vec![vec![0xabc]]);
        assert!(calls_to(&log, ADD_ADDON_NODES).is_empty());
        assert_eq!(*addresses(&log).last().unwrap(), CELL_LOCK_LEAVE);
    }

    #[test]
    fn the_empty_method_returns_zero() {
        let mut e = engine();
        assert_eq!(e.call(0x0055_78e0, &args![1u32, 2u32, 3u32, 4u32]).u32(), 0);
    }

    /// A node object with `children`: the count at `+0x10`, the elements from
    /// `+0x14`; the doubles of the child count and element accessors.
    fn node_with_children(e: &mut Engine, children: &[u32]) -> u32 {
        let at = object(e, 0x80);
        e.mem.set_u32(at + 0x10, children.len() as u32);
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(at + 0x14 + 4 * index as u32, *child);
        }
        e.register(NODE_CHILD_COUNT, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(NODE_CHILD_AT, |e, a| {
            e.mem.u32(a[0] + 0x14 + 4 * a[1]).into_ret()
        });
        at
    }

    #[test]
    fn entries_named_cell_location_marker_are_called_back_with_their_index() {
        let mut e = engine();
        e.mem
            .set_cstr(CELL_LOCATION_MARKER_NAME, b"CellLocationMarker");
        e.register(ENTRY_HOLDER, |_, a| a[0].into_ret());
        e.register(HOLDER_NAME, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(STRING_COMPARE, |e, a| {
            u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])).into_ret()
        });
        quiet(&mut e, &[fake(0xf0)]);
        let named = |e: &mut Engine, name: &[u8]| {
            let at = e.mem.alloc(0x20);
            let text = e.mem.alloc(0x20);
            e.mem.set_cstr(text, name);
            e.mem.set_u32(at + 0x10, text);
            at
        };
        let marker_a = named(&mut e, b"CellLocationMarker");
        let other = named(&mut e, b"SomethingElse");
        let marker_b = named(&mut e, b"CellLocationMarker");
        let nameless = e.mem.alloc(0x20);
        let node = node_with_children(&mut e, &[marker_a, 0, other, nameless, marker_b]);
        returns(&mut e, CELL_NODE_D, node);
        start_log(&mut e);
        e.call(0x0055_78f0, &args![0x1234u32]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, CELL_NODE_D), vec![vec![0x1234]]);
        assert_eq!(
            calls_to(&log, fake(0xf0)),
            vec![vec![node, 0], vec![node, 4]]
        );
        // Without a node, nothing else happens.
        returns(&mut e, CELL_NODE_D, 0);
        start_log(&mut e);
        e.call(0x0055_78f0, &args![0x1234u32]);
        assert_eq!(addresses(&calls(&e)), vec![CELL_NODE_D]);
    }

    #[test]
    fn nodes_of_the_second_level_are_updated_only_in_mode_4() {
        let mut e = engine();
        // Leaves hold their reference at `+0x10`; references the base object
        // at `+0x34` and their 3D at `+0x38`; base objects the type at `+4`.
        e.register(FIND_REFERENCE_FOR_3D, |e, a| {
            e.mem.u32(a[0] + 0x10).into_ret()
        });
        e.register(REFERENCE_BASE_OBJECT, |e, a| {
            e.mem.u32(a[0] + 0x34).into_ret()
        });
        e.register(FORM_TYPE, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        e.register(fake(0x1d0), |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        quiet(&mut e, &[NODE_SET_FLAG]);
        let reference = |e: &mut Engine, base_type: u8, base_at: Option<u32>| {
            let base = base_at.unwrap_or_else(|| e.mem.alloc(0x10));
            e.mem.set_u8(base + 4, base_type);
            let at = object(e, 0x60);
            e.mem.set_u32(at + 0x34, base);
            e.mem.set_u32(at + 0x38, 0x3d00 + base_type as u32);
            (at, base)
        };
        let leaf = |e: &mut Engine, reference: u32| {
            let at = e.mem.alloc(0x20);
            e.mem.set_u32(at + 0x10, reference);
            at
        };
        let (plain, _) = reference(&mut e, 5, None);
        let (skipped_type, _) = reference(&mut e, 0x1e, None);
        let (skipped_form, form_base) = reference(&mut e, 6, None);
        e.set_global(SKIPPED_FORM, form_base);
        let leaves = [
            leaf(&mut e, plain),
            0,
            leaf(&mut e, 0),
            leaf(&mut e, skipped_type),
            leaf(&mut e, skipped_form),
        ];
        let group = node_with_children(&mut e, &leaves);
        let root = node_with_children(&mut e, &[0, group]);
        returns(&mut e, CELL_CHILD_NODE, root);
        start_log(&mut e);
        e.call(0x0055_7990, &args![0x1234u32, 1u32, 4u32]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, CELL_CHILD_NODE), vec![vec![0x1234, 7]]);
        assert_eq!(calls_to(&log, NODE_SET_FLAG), vec![vec![0x3d05, 1]]);
        start_log(&mut e);
        e.call(0x0055_7990, &args![0x1234u32, 1u32, 3u32]);
        assert!(calls_to(&calls(&e), NODE_SET_FLAG).is_empty());
        returns(&mut e, CELL_CHILD_NODE, 0);
        start_log(&mut e);
        e.call(0x0055_7990, &args![0x1234u32, 1u32, 4u32]);
        assert_eq!(addresses(&calls(&e)), vec![CELL_CHILD_NODE]);
    }

    #[test]
    fn the_fade_to_high_detail_starts_only_once() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        e.set(cell, TESObjectCELL::bFadingToLowDetail, true);
        e.set(cell, TESObjectCELL::fLodFadeInPercent, 7.5);
        e.call(0x0055_7aa0, &args![cell]);
        assert!(e.get(cell, TESObjectCELL::bFadingToHighDetail));
        assert!(!e.get(cell, TESObjectCELL::bFadingToLowDetail));
        assert_eq!(e.get(cell, TESObjectCELL::fLodFadeInPercent), 0.0);
        e.set(cell, TESObjectCELL::bFadingToLowDetail, true);
        e.set(cell, TESObjectCELL::fLodFadeInPercent, 7.5);
        e.call(0x0055_7aa0, &args![cell]);
        assert!(e.get(cell, TESObjectCELL::bFadingToLowDetail));
        assert_eq!(e.get(cell, TESObjectCELL::fLodFadeInPercent), 7.5);
    }

    /// Models for the two fade walks: a reference holds its model at `+0x30`,
    /// a model its node at `+0x30` (the answer of virtual `+0x10`), a node
    /// its type at `+0` and its distance at `+4`.
    fn model_setup(e: &mut Engine) {
        e.register(REFERENCE_MODEL, |e, a| e.mem.u32(a[0] + 0x30).into_ret());
        e.register(fake(0x10), |e, a| e.mem.u32(a[0] + 0x30).into_ret());
        e.register(MODEL_TYPE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(MODEL_DISTANCE, |e, a| e.mem.f32(a[0] + 4).into_ret());
        quiet(
            e,
            &[MODEL_PREPARE, SET_PROPERTY_FADE_ALPHA, MODEL_DETACH_STEP],
        );
    }

    fn model_reference(e: &mut Engine, node_type: Option<(u32, f32)>) -> (u32, u32) {
        let node = e.mem.alloc(0x10);
        if let Some((kind, distance)) = node_type {
            e.mem.set_u32(node, kind);
            e.mem.set_f32(node + 4, distance);
        }
        let model = object(e, 0x40);
        e.mem
            .set_u32(model + 0x30, if node_type.is_some() { node } else { 0 });
        let reference = e.mem.alloc(0x40);
        e.mem.set_u32(reference + 0x30, model);
        (reference, node)
    }

    #[test]
    fn the_fade_to_high_detail_finishes_the_nodes_that_qualify() {
        let mut e = engine();
        model_setup(&mut e);
        e.set_global(LOD_FADE_VALUE, 1.25f32);
        e.set_global(MODEL_DISTANCE_LIMIT, 599.9000244140625f64);
        let player = e.mem.alloc(0x40);
        e.set_global(DISTANCE_SOURCE_SINGLETON, player);
        let (type_six, node_six) = model_reference(&mut e, Some((6, 0.0)));
        let (far, node_far) = model_reference(&mut e, Some((3, 700.0)));
        let (near, _) = model_reference(&mut e, Some((3, 500.0)));
        let (no_node, _) = model_reference(&mut e, None);
        let no_model = e.mem.alloc(0x40);
        let cell =
            reference_list_setup(&mut e, &[0, player, no_model, no_node, near, type_six, far]);
        e.set(cell, TESObjectCELL::bFadingToLowDetail, true);
        start_log(&mut e);
        e.call(0x0055_7ae0, &args![cell]);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, MODEL_PREPARE),
            vec![vec![node_six], vec![node_far]]
        );
        assert_eq!(
            calls_to(&log, SET_PROPERTY_FADE_ALPHA),
            vec![
                vec![node_six, 1.0f32.to_bits()],
                vec![node_far, 1.0f32.to_bits()]
            ]
        );
        assert_eq!(e.get(cell, TESObjectCELL::fLodFadeInPercent), 1.25);
        assert!(e.get(cell, TESObjectCELL::bDisplayHighDetail));
        assert!(e.get(cell, TESObjectCELL::bUpdateTerrain));
        assert!(!e.get(cell, TESObjectCELL::bFadingToHighDetail));
        assert!(!e.get(cell, TESObjectCELL::bFadingToLowDetail));
        assert_eq!(addresses(&log)[0], CELL_LOCK_ENTER);
        assert_eq!(*addresses(&log).last().unwrap(), CELL_LOCK_LEAVE);
    }

    #[test]
    fn every_cell_of_the_scene_finishes_its_fade() {
        let mut e = engine();
        let first = reference_list_setup(&mut e, &[]);
        let second = e.new_object::<TESObjectCELL>();
        e.set_global(SCENE_SINGLETON, 0x9900u32);
        returns(&mut e, SCENE_CELL_COUNT, 3);
        let cells = [first.addr(), 0, second.addr()];
        let table = e.mem.alloc(12);
        for (index, cell) in cells.iter().enumerate() {
            e.mem.set_u32(table + 4 * index as u32, *cell);
        }
        e.set_global(LOD_FADE_VALUE, 2.0f32);
        e.register_double(SCENE_CELL_AT, move |e, a| {
            e.mem.u32(table + 4 * a[1]).into_ret()
        });
        start_log(&mut e);
        e.call(0x0055_7be0, &args![0xdeadu32]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, SCENE_CELL_COUNT), vec![vec![0x9900]; 4]);
        assert_eq!(
            calls_to(&log, SCENE_CELL_AT),
            vec![vec![0x9900, 0], vec![0x9900, 1], vec![0x9900, 2]]
        );
        for cell in [first, second] {
            assert!(e.get(cell, TESObjectCELL::bUpdateTerrain));
            assert_eq!(e.get(cell, TESObjectCELL::fLodFadeInPercent), 2.0);
        }
        assert_eq!(calls_to(&log, CELL_LOCK_ENTER).len(), 2);
    }

    #[test]
    fn the_detach_preparation_clears_the_fade_and_detaches_type_6_nodes() {
        let mut e = engine();
        model_setup(&mut e);
        e.set_global(LOD_FADE_VALUE, 1.5f32);
        let player = e.mem.alloc(0x40);
        e.set_global(DISTANCE_SOURCE_SINGLETON, player);
        let (type_six, node_six) = model_reference(&mut e, Some((6, 0.0)));
        let (other, _) = model_reference(&mut e, Some((3, 900.0)));
        let (no_node, _) = model_reference(&mut e, None);
        let cell = reference_list_setup(&mut e, &[player, other, 0, no_node, type_six]);
        e.set(cell, TESObjectCELL::bFadingToHighDetail, true);
        e.set(cell, TESObjectCELL::bDisplayHighDetail, true);
        e.set(cell, TESObjectCELL::bFadingToLowDetail, true);
        start_log(&mut e);
        e.call(0x0055_7c40, &args![cell]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, MODEL_DETACH_STEP), vec![vec![node_six]]);
        assert!(calls_to(&log, MODEL_PREPARE).is_empty());
        assert!(e.get(cell, TESObjectCELL::bFadingToLowDetail));
        assert!(!e.get(cell, TESObjectCELL::bFadingToHighDetail));
        assert!(!e.get(cell, TESObjectCELL::bDisplayHighDetail));
        assert_eq!(e.get(cell, TESObjectCELL::fLodFadeInPercent), 1.5);
        // Without the low-detail fade the flag stays clear.
        e.set(cell, TESObjectCELL::bFadingToLowDetail, false);
        e.call(0x0055_7c40, &args![cell]);
        assert!(!e.get(cell, TESObjectCELL::bFadingToLowDetail));
    }

    #[test]
    fn the_cell_state_is_tested_against_zero() {
        let mut e = engine();
        returns(&mut e, CELL_GET_STATE, 0);
        assert!(!e.call(0x0055_7d10, &args![0x10u32]).bool());
        returns(&mut e, CELL_GET_STATE, 4);
        assert!(e.call(0x0055_7d10, &args![0x10u32]).bool());
    }

    #[test]
    fn the_display_high_detail_byte_is_returned() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        assert_eq!(e.call(0x0055_7d30, &args![cell]).u8(), 0);
        e.set(cell, TESObjectCELL::bDisplayHighDetail, true);
        assert_eq!(e.call(0x0055_7d30, &args![cell]).u8(), 1);
    }

    #[test]
    fn the_render_limits_default_to_the_game_values() {
        let mut e = engine();
        let at = e.mem.alloc(0x40);
        assert_eq!(e.call(0x0055_7d50, &args![at]).u32(), at);
        let words: Vec<u32> = (0..7).map(|index| e.mem.u32(at + 4 * index)).collect();
        assert_eq!(words, vec![0x4b0, 800_000, 100, 15, 0xffff_ffff, 8, 0]);
    }

    #[test]
    fn the_render_configuration_installs_the_report_callback() {
        let mut e = engine();
        let at = e.mem.alloc(0x40);
        e.mem.set_u32(at + 0x1c, 0x77);
        assert_eq!(e.call(0x0055_7da0, &args![at]).u32(), at);
        assert_eq!(e.mem.u32(at), 0x4b0);
        assert_eq!(e.mem.u32(at + 0x14), 8);
        assert_eq!(e.mem.u32(at + 0x18), 0x0055_86c0);
        assert_eq!(e.mem.u32(at + 0x1c), 0);
    }

    #[test]
    fn the_triangle_count_comes_from_the_array_at_0x38() {
        let mut e = engine();
        returns(&mut e, TRIANGLE_COUNT, 9);
        start_log(&mut e);
        assert_eq!(e.call(0x0055_8200, &args![0x5000u32]).u32(), 9);
        assert_eq!(calls(&e), vec![(TRIANGLE_COUNT, vec![0x5038])]);
    }

    /// A navmesh with the triangle records at `+0x3c`; the vertex table
    /// `(0,0,0) (3,0,0) (0,6,9)`.
    fn centre_setup(e: &mut Engine) -> (u32, u32) {
        let vertices = e.mem.alloc(36);
        put_floats(e, vertices, &[0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 6.0, 9.0]);
        let records = e.mem.alloc(32);
        // Two 16-byte records: the vertices (1, 2, 0) and (0, 0, 1).
        for (index, vertex) in [(0u32, 1u16), (1, 2), (2, 0), (8, 0), (9, 0), (10, 1)] {
            e.mem.set_u16(records + 2 * index, vertex);
        }
        let mesh = e.mem.alloc(0x100);
        e.mem.set_u32(mesh + 0x3c, records);
        e.register_double(NAV_MESH_VERTEX, move |_, a| {
            (vertices + 12 * a[1]).into_ret()
        });
        e.register(VECTOR_ADD_ASSIGN, |e, a| {
            for index in 0..3 {
                let sum = e.mem.f32(a[0] + 4 * index) + e.mem.f32(a[1] + 4 * index);
                e.mem.set_f32(a[0] + 4 * index, sum);
            }
            Ret::default()
        });
        e.register(VECTOR_SCALE, |e, a| {
            for index in 0..3 {
                let scaled = e.mem.f32(a[0] + 4 * index) * f32::from_bits(a[1]);
                e.mem.set_f32(a[0] + 4 * index, scaled);
            }
            Ret::default()
        });
        e.set_global(ONE_THIRD, 0.5f32);
        (mesh, records)
    }

    #[test]
    fn the_centre_of_a_triangle_is_the_scaled_sum_of_its_vertices() {
        let mut e = engine();
        let (mesh, _) = centre_setup(&mut e);
        let out = e.mem.alloc(16);
        start_log(&mut e);
        assert_eq!(e.call(0x0055_8220, &args![mesh, out, 0u16]).u32(), out);
        assert_eq!(floats(&e, out, 3), vec![1.5, 3.0, 4.5]);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, NAV_MESH_VERTEX),
            vec![vec![mesh, 1], vec![mesh, 2], vec![mesh, 0]]
        );
        // The second triangle holds the vertices 0, 0, 1.
        e.call(0x0055_8220, &args![mesh, out, 1u16]);
        assert_eq!(floats(&e, out, 3), vec![1.5, 0.0, 0.0]);
    }

    #[test]
    fn the_u16_table_entry_and_the_triangle_record_are_addressed() {
        let mut e = engine();
        let table = e.mem.alloc(16);
        e.mem.set_u16(table + 6, 0x1234);
        assert_eq!(e.call(0x0055_82d0, &args![table, 3u32]).u16(), 0x1234);
        let mesh = e.mem.alloc(0x100);
        e.mem.set_u32(mesh + 0x3c, 0x9000);
        assert_eq!(e.call(0x0055_82f0, &args![mesh, 5u16]).u32(), 0x9050);
        assert_eq!(e.call(0x0055_8dd0, &args![mesh + 0x38, 2u32]).u32(), 0x9020);
    }

    #[test]
    fn the_scene_child_zero_is_asked_for() {
        let mut e = engine();
        returns(&mut e, SCENE_NODE_CHILD, 0x4321);
        start_log(&mut e);
        assert_eq!(e.call(0x0055_8310, &args![0x1000u32]).u32(), 0x4321);
        assert_eq!(calls(&e), vec![(SCENE_NODE_CHILD, vec![0x1000, 0])]);
    }

    /// Doubles for one frame of the test render: the render handle, the
    /// camera node (translation at `+0x10`: `1, 2, 3`; rotation at `+0x20`:
    /// `1 .. 9`), the timer (`7` ticks per call), the statistics holder and
    /// the shadow scene value (low 16 bits `4`). Returns the camera node.
    fn render_setup(e: &mut Engine) -> u32 {
        let camera = e.mem.alloc(0x60);
        put_floats(e, camera + 0x10, &[1.0, 2.0, 3.0]);
        put_floats(
            e,
            camera + 0x20,
            &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
        );
        let holder = e.mem.alloc(0x20);
        e.mem.set_u32(holder + 8, 0xaaaa);
        e.set_global(RENDER_FRAME_OBJECT, 0x7000u32);
        e.set_global(DEGREES_TO_RADIANS, 0.017453292519943295f64);
        e.register(LIST_NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        returns(e, RENDER_HANDLE, 0x6000);
        returns(e, SCENE_NODE_CHILD, camera);
        returns(e, STATISTICS_HOLDER, holder);
        returns(e, TABLE_ENTRY, 0x5100);
        returns(e, SHADOW_SCENE_VALUE, 0x1_0004);
        e.register(NODE_LOCAL_TRANSLATION, |_, a| (a[0] + 0x10).into_ret());
        e.register(NODE_LOCAL_ROTATION, |_, a| (a[0] + 0x20).into_ret());
        e.register(SLOT_GET_POINTER, |e, a| e.mem.u32(a[0]).into_ret());
        let ticks = Rc::new(RefCell::new(0u32));
        e.register_double(TICK_COUNT, move |_, _| {
            *ticks.borrow_mut() += 7;
            (*ticks.borrow()).into_ret()
        });
        quiet(
            e,
            &[
                NODE_SET_TRANSLATION,
                NODE_SET_ROTATION,
                UPDATE_RECORD_CONSTRUCT,
                CONTROLLER_UPDATE,
                DEBUG_PRINT_LINE,
                MAKE_X_ROTATION,
                ROTATION_MATRIX_BUILD,
                MATRIX_PRODUCT,
                RENDER_FRAME_STEP,
            ],
        );
        camera
    }

    /// A configuration in memory: the default limits, `callback` at `+0x18`
    /// and `user_data` at `+0x1c`.
    fn render_config(e: &mut Engine, callback: u32, user_data: u32) -> u32 {
        let at = e.mem.alloc(0x20);
        fn_00557da0(e, Ptr::new(at));
        e.mem.set_u32(at + 0x18, callback);
        e.mem.set_u32(at + 0x1c, user_data);
        at
    }

    /// Records the three floats at every `NODE_SET_TRANSLATION` argument.
    fn capture_translations(e: &mut Engine) -> Rc<RefCell<Vec<Vec<f32>>>> {
        let seen = Rc::new(RefCell::new(vec![]));
        let store = seen.clone();
        e.register_double(NODE_SET_TRANSLATION, move |e, a| {
            store.borrow_mut().push(floats(e, a[1], 3));
            Ret::default()
        });
        seen
    }

    /// Records the record words and the user data of every report callback.
    /// (record words, user data) of each report.
    type Reports = Rc<RefCell<Vec<(Vec<u32>, u32)>>>;

    fn capture_reports(e: &mut Engine, callback: u32) -> Reports {
        let seen = Rc::new(RefCell::new(vec![]));
        let store = seen.clone();
        e.register_double(callback, move |e, a| {
            let words = (0..19).map(|index| e.mem.u32(a[0] + 4 * index)).collect();
            store.borrow_mut().push((words, a[1]));
            Ret::default()
        });
        seen
    }

    #[test]
    fn one_frame_is_rendered_and_checked_against_the_limits() {
        let mut e = engine();
        let camera = render_setup(&mut e);
        e.set_global(STATISTICS_SOURCES[0], 3u32);
        e.set_global(STATISTICS_SOURCES[2], 4u32);
        let reports = capture_reports(&mut e, 0x7777_0000);
        let config = render_config(&mut e, 0x7777_0000, 0x55);
        let position = e.mem.alloc(16);
        let rotation = e.mem.alloc(48);
        start_log(&mut e);
        assert!(e
            .call(0x0055_8430, &args![0x1234u32, position, rotation, config])
            .bool());
        assert_eq!(
            addresses(&calls(&e)),
            vec![
                RENDER_HANDLE,
                SCENE_NODE_CHILD,
                NODE_SET_TRANSLATION,
                SCENE_NODE_CHILD,
                NODE_SET_ROTATION,
                UPDATE_RECORD_CONSTRUCT,
                SCENE_NODE_CHILD,
                CONTROLLER_UPDATE,
                TICK_COUNT,
                RENDER_FRAME_STEP,
                LIST_NODE_ITEM_ADDRESS,
                LIST_NODE_ITEM_ADDRESS,
                TICK_COUNT,
                STATISTICS_HOLDER,
                SLOT_GET_POINTER,
                TABLE_ENTRY,
                SHADOW_SCENE_VALUE,
            ]
        );
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, NODE_SET_TRANSLATION),
            vec![vec![camera, position]]
        );
        assert_eq!(
            calls_to(&log, NODE_SET_ROTATION),
            vec![vec![camera, rotation]]
        );
        assert_eq!(calls_to(&log, RENDER_FRAME_STEP), vec![vec![0x7000]]);
        assert!(reports.borrow().is_empty());
    }

    #[test]
    fn a_frame_over_a_limit_reports_the_record() {
        // (what exceeds, setup)
        type Setup = fn(&mut Engine, u32);
        let cases: [(&str, Setup); 6] = [
            ("geometry", |e, _| {
                e.set_global(STATISTICS_SOURCES[0], 5000u32)
            }),
            ("triangles", |e, _| {
                e.set_global(STATISTICS_SOURCES[2], 900_000u32)
            }),
            ("passes", |e, config| e.mem.set_u32(config + 8, 50)),
            ("lights", |e, config| e.mem.set_u32(config + 0x0c, 3)),
            ("last count", |e, config| e.mem.set_u32(config + 0x14, 4)),
            ("time", |e, config| e.mem.set_u32(config + 0x10, 6)),
        ];
        for (what, setup) in cases {
            let mut e = engine();
            render_setup(&mut e);
            let reports = capture_reports(&mut e, 0x7777_0000);
            let config = render_config(&mut e, 0x7777_0000, 0x55);
            setup(&mut e, config);
            let position = e.mem.alloc(16);
            put_floats(&mut e, position, &[10.0, 20.0, 30.0]);
            let rotation = e.mem.alloc(48);
            put_floats(
                &mut e,
                rotation,
                &[9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0],
            );
            let passed = e
                .call(0x0055_8430, &args![0x1234u32, position, rotation, config])
                .bool();
            assert!(!passed, "{what}");
            let reports = reports.borrow();
            assert_eq!(reports.len(), 1, "{what}");
            let (words, user_data) = &reports[0];
            assert_eq!(*user_data, 0x55, "{what}");
            // Words 2 to 5 are the default limits and the measured values;
            // word 6 the cell, then the position and the rotation.
            assert_eq!(words[2], 100, "{what}");
            assert_eq!(words[3], 4, "{what}");
            assert_eq!(words[4], 7, "{what}");
            assert_eq!(words[5], 8, "{what}");
            assert_eq!(words[6], 0x1234, "{what}");
            assert_eq!(
                words[7..10].to_vec(),
                [10.0f32, 20.0, 30.0].map(f32::to_bits).to_vec(),
                "{what}"
            );
            assert_eq!(
                words[10..19].to_vec(),
                [9.0f32, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]
                    .map(f32::to_bits)
                    .to_vec(),
                "{what}"
            );
            if what == "geometry" {
                assert_eq!(words[0], 5000);
            }
        }
    }

    #[test]
    fn a_failing_frame_without_a_callback_only_fails() {
        let mut e = engine();
        render_setup(&mut e);
        e.set_global(STATISTICS_SOURCES[0], 5000u32);
        let config = render_config(&mut e, 0, 0);
        assert!(!e
            .call(0x0055_8430, &args![1u32, 0x100u32, 0x200u32, config])
            .bool());
    }

    #[test]
    fn the_test_cell_is_rendered_from_24_orientations() {
        let mut e = engine();
        let camera = render_setup(&mut e);
        let config = render_config(&mut e, 0, 0);
        let position = e.mem.alloc(16);
        start_log(&mut e);
        assert!(e
            .call(0x0055_8330, &args![0x1234u32, position, config])
            .bool());
        let log = calls(&e);
        let angle = |degrees: i32| ((degrees as f64) * 0.017453292519943295f64) as f32;
        let x_angles: Vec<f32> = calls_to(&log, MAKE_X_ROTATION)
            .iter()
            .map(|call| f32::from_bits(call[1]))
            .collect();
        assert_eq!(x_angles, vec![angle(45), angle(0), angle(-45)]);
        let z_angles: Vec<f32> = calls_to(&log, ROTATION_MATRIX_BUILD)
            .iter()
            .map(|call| f32::from_bits(call[1]))
            .collect();
        assert_eq!(z_angles.len(), 24);
        assert_eq!(
            z_angles[..8],
            [0, 45, 90, 135, 180, 225, 270, 315].map(angle)
        );
        assert_eq!(z_angles[8..16], z_angles[..8]);
        let products = calls_to(&log, MATRIX_PRODUCT);
        assert_eq!(products.len(), 24);
        assert!(products.iter().all(|call| call[1..] == products[0][1..]));
        let translations = calls_to(&log, NODE_SET_TRANSLATION);
        assert_eq!(translations.len(), 24);
        assert!(translations
            .iter()
            .all(|call| *call == vec![camera, position]));
    }

    #[test]
    fn every_orientation_is_tried_even_after_a_failure() {
        let mut e = engine();
        render_setup(&mut e);
        let config = render_config(&mut e, 0, 0);
        let products = Rc::new(RefCell::new(0u32));
        let counter = products.clone();
        e.register_double(MATRIX_PRODUCT, move |e, _| {
            *counter.borrow_mut() += 1;
            let failing = *counter.borrow() == 5;
            e.set_global(STATISTICS_SOURCES[0], if failing { 99_999u32 } else { 0 });
            Ret::default()
        });
        start_log(&mut e);
        assert!(!e.call(0x0055_8330, &args![1u32, 0x100u32, config]).bool());
        assert_eq!(calls_to(&calls(&e), NODE_SET_TRANSLATION).len(), 24);
        assert_eq!(*products.borrow(), 24);
    }

    #[test]
    fn the_test_cell_does_nothing_without_a_3d() {
        let mut e = engine();
        start_log(&mut e);
        e.call(0x0055_7dd0, &args![0u32, 0u32, 0u32]);
        assert_eq!(calls(&e), vec![]);
        returns(&mut e, CELL_NODE_OF_CELL, 0);
        start_log(&mut e);
        e.call(0x0055_7dd0, &args![0x1234u32, 0u32, 1u32]);
        assert_eq!(calls(&e), vec![(CELL_NODE_OF_CELL, vec![0x1234])]);
    }

    #[test]
    fn the_test_cell_is_rendered_from_the_player_position() {
        let mut e = engine();
        render_setup(&mut e);
        returns(&mut e, CELL_NODE_OF_CELL, 0x3d3d);
        let player = object(&mut e, 0x60);
        e.set_global(PLAYER_OBJECT, player);
        e.register(fake(0x1d0), |_, _| 0x5e00u32.into_ret());
        let position = e.mem.alloc(16);
        returns(&mut e, NODE_POSITION_POINTER, position);
        let seen = capture_translations(&mut e);
        start_log(&mut e);
        e.call(0x0055_7dd0, &args![0x1234u32, 0u32, 0u32]);
        let log = calls(&e);
        assert_eq!(calls_to(&log, NODE_POSITION_POINTER), vec![vec![0x5e00]]);
        // 24 renders at the player's position, then the camera is restored
        // to the translation and rotation it had.
        assert_eq!(seen.borrow().len(), 25);
        assert_eq!(seen.borrow()[24], vec![1.0, 2.0, 3.0]);
        assert_eq!(calls_to(&log, NODE_SET_ROTATION).len(), 25);
        assert!(calls_to(&log, DEBUG_PRINT_LINE).is_empty());
        assert_eq!(*addresses(&log).last().unwrap(), CONTROLLER_UPDATE);
    }

    #[test]
    fn a_failed_test_cell_prints_the_failure_message() {
        let mut e = engine();
        render_setup(&mut e);
        returns(&mut e, CELL_NODE_OF_CELL, 0x3d3d);
        let player = object(&mut e, 0x60);
        e.set_global(PLAYER_OBJECT, player);
        e.register(fake(0x1d0), |_, _| 0x5e00u32.into_ret());
        returns(&mut e, NODE_POSITION_POINTER, 0x100);
        e.set_global(STATISTICS_SOURCES[0], 5000u32);
        let config = render_config(&mut e, 0, 0);
        start_log(&mut e);
        e.call(0x0055_7dd0, &args![0x1234u32, config, 0u32]);
        assert_eq!(
            calls_to(&calls(&e), DEBUG_PRINT_LINE),
            vec![vec![RENDER_FAILED_MESSAGE]]
        );
    }

    #[test]
    fn the_test_cell_is_rendered_from_the_navmesh_triangle_centres() {
        let mut e = engine();
        render_setup(&mut e);
        returns(&mut e, CELL_NODE_OF_CELL, 0x3d3d);
        let (first, _) = centre_setup(&mut e);
        e.mem.set_u32(first + 0x60, 2);
        let second = e.mem.alloc(0x100);
        e.mem.set_u32(second + 0x60, 3);
        returns(&mut e, CELL_NAV_MESH_ARRAY, 0xa000);
        returns(&mut e, NAV_MESH_ARRAY_COUNT, 2);
        e.register(TRIANGLE_COUNT, |e, a| {
            e.mem.u32(a[0] - 0x38 + 0x60).into_ret()
        });
        e.register_double(NAV_MESH_BY_INDEX, move |e, a| {
            e.mem.set_u32(a[1], if a[2] == 0 { first } else { second });
            Ret::default()
        });
        e.register_double(NAV_MESH_SLOT_TEST, move |e, a| {
            u32::from(e.mem.u32(a[0]) == first).into_ret()
        });
        quiet(&mut e, &[NAV_MESH_SLOT_RELEASE]);
        e.set_global(TRIANGLE_LIFT, 118.0f64);
        let seen = capture_translations(&mut e);
        start_log(&mut e);
        e.call(0x0055_7dd0, &args![0x1234u32, 0u32, 1u32]);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT_LINE),
            vec![vec![TESTING_TRIANGLES_FORMAT, 5]]
        );
        assert_eq!(calls_to(&log, NAV_MESH_SLOT_RELEASE).len(), 4);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 49);
        assert_eq!(seen[0], vec![1.5, 3.0, 122.5]);
        assert_eq!(seen[23], vec![1.5, 3.0, 122.5]);
        assert_eq!(seen[24], vec![1.5, 0.0, 118.0]);
        assert_eq!(seen[48], vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn the_test_cell_is_rendered_from_the_reference_positions() {
        let mut e = engine();
        render_setup(&mut e);
        returns(&mut e, CELL_NODE_OF_CELL, 0x3d3d);
        returns(&mut e, CELL_NAV_MESH_ARRAY, 0);
        returns(&mut e, LIST_COUNT, 2);
        list_doubles(&mut e);
        // A reference holds a pointer to its position at `+0x34` (virtual
        // `+0x1f4`) and its 3D at `+0x38` (virtual `+0x1d0`); a 3D its world
        // bound at `+0x10`.
        e.register(fake(0x1f4), |e, a| e.mem.u32(a[0] + 0x34).into_ret());
        e.register(fake(0x1d0), |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        e.register(WORLD_BOUND, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        let position_a = e.mem.alloc(16);
        put_floats(&mut e, position_a, &[4.0, 5.0, 6.0]);
        let reference_a = object(&mut e, 0x60);
        e.mem.set_u32(reference_a + 0x34, position_a);
        let bound = e.mem.alloc(16);
        put_floats(&mut e, bound, &[7.0, 8.0, 9.0]);
        let node = e.mem.alloc(0x40);
        e.mem.set_u32(node + 0x10, bound);
        let reference_b = object(&mut e, 0x60);
        e.mem.set_u32(reference_b + 0x34, position_a);
        e.mem.set_u32(reference_b + 0x38, node);
        let head = list_of(&mut e, &[reference_a, reference_b]);
        returns(&mut e, CELL_REFERENCES, head);
        let seen = capture_translations(&mut e);
        start_log(&mut e);
        e.call(0x0055_7dd0, &args![0x1234u32, 0u32, 1u32]);
        let log = calls(&e);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT_LINE),
            vec![vec![TESTING_POSITIONS_FORMAT, 2]]
        );
        let seen = seen.borrow();
        assert_eq!(seen.len(), 49);
        assert_eq!(seen[0], vec![4.0, 5.0, 6.0]);
        assert_eq!(seen[24], vec![7.0, 8.0, 9.0]);
        assert_eq!(seen[48], vec![1.0, 2.0, 3.0]);
        // The reference without a 3D never asks for its bound.
        assert_eq!(calls_to(&log, WORLD_BOUND), vec![vec![node]]);
    }

    #[test]
    fn the_statistics_words_are_stored_through_the_pointers_in_order() {
        let mut e = engine();
        for (index, source) in STATISTICS_SOURCES.iter().enumerate() {
            e.set_global(*source, 100 + index as u32);
        }
        let words = e.mem.alloc(48);
        // The tenth pointer appears twice: the last word it gets is kept.
        let mut pointers = [0u32; 11];
        for (index, slot) in pointers.iter_mut().enumerate() {
            *slot = words + 4 * index as u32;
        }
        pointers[10] = pointers[3];
        let mut call_words = vec![0x1234u32];
        call_words.extend(pointers);
        e.call(0x0055_8600, &call_words);
        let stored: Vec<u32> = (0..11).map(|index| e.mem.u32(words + 4 * index)).collect();
        assert_eq!(
            stored,
            vec![100, 101, 102, 110, 104, 105, 106, 107, 108, 109, 0]
        );
    }

    #[test]
    fn the_statistics_record_starts_with_the_default_limits() {
        let mut e = engine();
        e.register(LIST_NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        let at = e.mem.alloc(0x40);
        start_log(&mut e);
        assert_eq!(e.call(0x0055_8690, &args![at]).u32(), at);
        assert_eq!(e.mem.u32(at + 0x10), 0xffff_ffff);
        assert_eq!(
            calls(&e)
                .iter()
                .map(|(address, words)| (*address, words[0] - at))
                .collect::<Vec<_>>(),
            vec![
                (LIST_NODE_ITEM_ADDRESS, 0x1c),
                (LIST_NODE_ITEM_ADDRESS, 0x28)
            ]
        );
        e.register(SLOT_GET_POINTER, |_, a| (a[0] * 2).into_ret());
        assert_eq!(e.call(0x0055_85e0, &args![0x1000u32]).u32(), 0x2010);
    }

    #[test]
    fn the_report_callback_saves_the_failure_data() {
        let mut e = engine();
        e.set_global(LAST_RENDER_FAILURE_CELL, 0x1234u32);
        start_log(&mut e);
        e.call(0x0055_86c0, &args![0u32]);
        assert_eq!(e.global::<u32>(LAST_RENDER_FAILURE_CELL), 0);
        assert_eq!(calls(&e), vec![]);
    }

    /// The record of a render failure and the doubles `SaveRenderFailureData`
    /// needs. The cell object: `+0x30` interior flag, `+0x34` world space,
    /// `+0x38` pointer to its name's first character, `+0x3c` its name for
    /// virtual `+0x130`, `+0x40` and `+0x44` the coordinates, `+0x0c` the
    /// form id. Returns the record, the cell and the world space.
    fn failure_setup(e: &mut Engine, interior: bool, name: &[u8]) -> (u32, u32, u32) {
        e.register(CELL_IS_INTERIOR, |e, a| e.mem.u8(a[0] + 0x30).into_ret());
        e.register(CELL_GET_WORLD_SPACE, |e, a| {
            e.mem.u32(a[0] + 0x34).into_ret()
        });
        e.register(CELL_NAME_POINTER, |e, a| e.mem.u32(a[0] + 0x38).into_ret());
        e.register(CELL_GET_DATA_X, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.register(CELL_GET_DATA_Y, |e, a| e.mem.u32(a[0] + 0x44).into_ret());
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0x0c).into_ret());
        e.register(fake(0x130), |e, a| e.mem.u32(a[0] + 0x3c).into_ret());
        e.register(FORMAT_TEXT, |e, a| {
            e.mem.set_cstr(a[0], b"0123456789");
            Ret::default()
        });
        e.register(STRING_LENGTH, |e, a| {
            (e.mem.cstr(a[0]).len() as u32).into_ret()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FILE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], TEST_VTABLE);
            a[0].into_ret()
        });
        e.register(MATRIX_TO_EULER, |e, a| {
            e.mem.set_f32(a[1], 0.5);
            e.mem.set_f32(a[2], 1.0);
            e.mem.set_f32(a[3], 2.0);
            Ret::default()
        });
        quiet(
            e,
            &[
                CREATE_DIRECTORY,
                DELETE_FILE,
                fake(0x20),
                fake(0x14),
                fake(0x48),
                fake(0),
            ],
        );
        e.set_global(RADIANS_TO_DEGREES, 2.0f64);
        e.set_global(FILE_SEEK_ORIGIN, 0x42u32);
        let text = e.mem.alloc(0x40);
        e.mem.set_cstr(text, name);
        let world = object(e, 0x40);
        e.mem.set_u32(world + 0x3c, text);
        let cell = object(e, 0x80);
        e.mem.set_u8(cell + 0x30, u8::from(interior));
        e.mem.set_u32(cell + 0x34, world);
        e.mem.set_u32(cell + 0x38, text);
        e.mem.set_u32(cell + 0x3c, text);
        e.mem.set_u32(cell + 0x40, 7);
        e.mem.set_u32(cell + 0x44, 9);
        e.mem.set_u32(cell + 0x0c, 0xabcd);
        let record = e.mem.alloc(0x80);
        for (index, count) in [11u32, 12, 13, 14, 15].iter().enumerate() {
            e.mem.set_u32(record + 4 * index as u32, *count);
        }
        e.mem.set_u32(record + 0x18, cell);
        put_floats(e, record + 0x1c, &[1.0, 2.0, 3.0]);
        (record, cell, world)
    }

    /// The words of `args![..]` for the failure line: the buffer, its size,
    /// the format, the form id, the position and the angles as doubles, and
    /// the five counts.
    fn failure_line(line: u32) -> Vec<u32> {
        let mut words = vec![line, PATH_BUFFER_SIZE, LINE_FORMAT, 0xabcd];
        for value in [1.0f64, 2.0, 3.0, 1.0, 2.0, 4.0] {
            words.extend(args![value]);
        }
        words.extend([15, 11, 12, 13, 14]);
        words
    }

    #[test]
    fn the_first_failure_of_an_interior_cell_creates_the_file_with_a_header() {
        let mut e = engine();
        let (record, cell, _) = failure_setup(&mut e, true, b"Vault");
        start_log(&mut e);
        e.call(0x0055_86e0, &args![record]);
        let log = calls(&e);
        let path = calls_to(&log, DELETE_FILE)[0][0];
        let line = path + PATH_BUFFER_SIZE;
        assert_eq!(
            calls_to(&log, CREATE_DIRECTORY),
            vec![vec![RTF_DIRECTORY_NAME]]
        );
        let name = e.mem.u32(cell + 0x3c);
        let formats = calls_to(&log, FORMAT_TEXT);
        assert_eq!(formats.len(), 3);
        assert_eq!(
            formats[0],
            vec![
                path,
                PATH_BUFFER_SIZE,
                INTERIOR_FILE_FORMAT,
                RTF_PREFIX,
                name
            ]
        );
        assert_eq!(formats[1], vec![line, PATH_BUFFER_SIZE, HEADER_FORMAT]);
        assert_eq!(formats[2], failure_line(line));
        let files = calls_to(&log, FILE_CONSTRUCT);
        assert_eq!(files.len(), 1);
        let file = files[0][0];
        assert_eq!(files[0][1..], [path, 1, 0x4000, 0]);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x158]]);
        assert_eq!(calls_to(&log, fake(0x20)), vec![vec![file, 0, 0]]);
        assert_eq!(calls_to(&log, fake(0x14)), vec![vec![file, 0, 0x42]]);
        assert_eq!(
            calls_to(&log, fake(0x48)),
            vec![vec![file, line, 10], vec![file, line, 10]]
        );
        assert_eq!(calls_to(&log, fake(0)), vec![vec![file, 1]]);
        assert_eq!(e.global::<u32>(LAST_RENDER_FAILURE_CELL), cell);
        assert_eq!(calls_to(&log, MATRIX_TO_EULER)[0][0], record + 0x28);
    }

    #[test]
    fn a_second_failure_of_the_same_cell_appends_a_line() {
        let mut e = engine();
        let (record, cell, _) = failure_setup(&mut e, true, b"Vault");
        e.set_global(LAST_RENDER_FAILURE_CELL, cell);
        start_log(&mut e);
        e.call(0x0055_86e0, &args![record]);
        let log = calls(&e);
        assert!(calls_to(&log, DELETE_FILE).is_empty());
        assert!(calls_to(&log, fake(0x14)).is_empty());
        assert_eq!(calls_to(&log, FORMAT_TEXT).len(), 2);
        let files = calls_to(&log, FILE_CONSTRUCT);
        assert_eq!(files[0][2..], [2, 0x4000, 0]);
        assert_eq!(calls_to(&log, fake(0x20)).len(), 1);
        assert_eq!(calls_to(&log, fake(0x48)).len(), 1);
        assert_eq!(calls_to(&log, fake(0)).len(), 1);
    }

    #[test]
    fn exterior_file_names_carry_the_world_space_the_coordinates_and_the_cell_name() {
        let mut e = engine();
        let (record, cell, world) = failure_setup(&mut e, false, b"Goodsprings");
        let name = e.mem.u32(cell + 0x3c);
        let world_name = e.mem.u32(world + 0x3c);
        start_log(&mut e);
        e.call(0x0055_86e0, &args![record]);
        let log = calls(&e);
        let path = calls_to(&log, DELETE_FILE)[0][0];
        assert_eq!(
            calls_to(&log, FORMAT_TEXT)[0],
            vec![
                path,
                PATH_BUFFER_SIZE,
                EXTERIOR_NAMED_FILE_FORMAT,
                RTF_PREFIX,
                world_name,
                7,
                9,
                name
            ]
        );
        assert_eq!(calls_to(&log, CELL_GET_WORLD_SPACE), vec![vec![cell]]);
        // A cell without a name leaves it out.
        let mut e = engine();
        let (record, cell, world) = failure_setup(&mut e, false, b"");
        let world_name = e.mem.u32(world + 0x3c);
        let _ = cell;
        start_log(&mut e);
        e.call(0x0055_86e0, &args![record]);
        let log = calls(&e);
        let path = calls_to(&log, DELETE_FILE)[0][0];
        assert_eq!(
            calls_to(&log, FORMAT_TEXT)[0],
            vec![
                path,
                PATH_BUFFER_SIZE,
                EXTERIOR_FILE_FORMAT,
                RTF_PREFIX,
                world_name,
                7,
                9
            ]
        );
    }

    #[test]
    fn a_null_failure_record_forgets_the_last_cell() {
        let mut e = engine();
        e.set_global(LAST_RENDER_FAILURE_CELL, 0x1234u32);
        start_log(&mut e);
        e.call(0x0055_86e0, &args![0u32]);
        assert_eq!(e.global::<u32>(LAST_RENDER_FAILURE_CELL), 0);
        assert_eq!(calls(&e), vec![]);
    }

    #[test]
    fn the_lighting_template_accessors_use_the_cell_fields() {
        let mut e = engine();
        let cell = e.new_object::<TESObjectCELL>();
        e.call(0x0055_8b60, &args![cell, 0x4455u32]);
        assert_eq!(e.call(0x0055_8b40, &args![cell]).u32(), 0x4455);
        assert_eq!(e.mem.u32(cell.addr() + 0xd8), 0x4455);
        e.mem.set_u32(cell.addr() + 0xdc, 0b0110);
        assert!(e.call(0x0055_8b80, &args![cell, 0b0100u32]).bool());
        assert!(!e.call(0x0055_8b80, &args![cell, 0b1001u32]).bool());
    }

    /// An object with a held pointer (`+0x64`) to the word `value`.
    fn holder_of(e: &mut Engine, value: u32) -> (u32, u32) {
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, value);
        let at = e.mem.alloc(0x80);
        e.mem.set_u32(at + 0x64, word);
        (at, word)
    }

    #[test]
    fn the_held_word_is_read_and_written_through_the_pointer() {
        let mut e = engine();
        let (at, word) = holder_of(&mut e, 0x77);
        assert_eq!(e.call(0x0055_8c30, &args![at]).u32(), 0x77);
        e.call(0x0055_8c60, &args![at, 0x88u32]);
        assert_eq!(e.mem.u32(word), 0x88);
        let empty = e.mem.alloc(0x80);
        assert_eq!(e.call(0x0055_8c30, &args![empty]).u32(), 0);
        e.call(0x0055_8c60, &args![empty, 0x99u32]);
        assert_eq!(e.mem.u32(empty + 0x64), 0);
    }

    #[test]
    fn held_words_equal_to_the_key_are_cleared_until_a_null_item() {
        let mut e = engine();
        list_doubles(&mut e);
        let (first, first_word) = holder_of(&mut e, 5);
        let (second, second_word) = holder_of(&mut e, 6);
        let (third, third_word) = holder_of(&mut e, 5);
        let (after_null, after_word) = holder_of(&mut e, 5);
        let (player, player_word) = holder_of(&mut e, 5);
        e.set_global(DISTANCE_SOURCE_SINGLETON, player);
        let head = list_of(&mut e, &[first, second, third, 0, after_null]);
        returns(&mut e, CELL_REFERENCES, head);
        e.call(0x0055_8ba0, &args![0x1234u32, 5u32]);
        assert_eq!(e.mem.u32(first_word), 0);
        assert_eq!(e.mem.u32(second_word), 6);
        assert_eq!(e.mem.u32(third_word), 0);
        assert_eq!(e.mem.u32(after_word), 5);
        assert_eq!(e.mem.u32(player_word), 0);
    }

    #[test]
    fn references_are_activated_in_the_havok_world_only_when_they_qualify() {
        let mut e = engine();
        list_doubles(&mut e);
        // `+0x30` answer of virtual `+0x100`, `+0x34` base object (type at
        // `+4`), `+0x38` answer of `00549580`.
        e.register(fake(0x100), |e, a| e.mem.u8(a[0] + 0x30).into_ret());
        e.register(REFERENCE_BASE_OBJECT, |e, a| {
            e.mem.u32(a[0] + 0x34).into_ret()
        });
        e.register(FORM_TYPE, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        e.register(BASE_FORM_FLAG_40, |e, a| e.mem.u8(a[0] + 0x38).into_ret());
        quiet(&mut e, &[WORLD_ACTIVATE]);
        let reference = |e: &mut Engine, refuses: bool, base_type: u8, flagged: bool| {
            let base = e.mem.alloc(0x10);
            e.mem.set_u8(base + 4, base_type);
            let at = object(e, 0x60);
            e.mem.set_u8(at + 0x30, u8::from(refuses));
            e.mem.set_u32(at + 0x34, base);
            e.mem.set_u8(at + 0x38, u8::from(flagged));
            at
        };
        let good = reference(&mut e, false, 5, true);
        let refusing = reference(&mut e, true, 5, true);
        let door = reference(&mut e, false, 0x1c, true);
        let unflagged = reference(&mut e, false, 5, false);
        let good_too = reference(&mut e, false, 6, true);
        let after_null = reference(&mut e, false, 5, true);
        let head = list_of(
            &mut e,
            &[good, refusing, door, unflagged, good_too, 0, after_null],
        );
        returns(&mut e, CELL_REFERENCES, head);
        returns(&mut e, CELL_NODE_OF_CELL, 0x3d3d);
        start_log(&mut e);
        e.call(0x0055_8c90, &args![0x1234u32]);
        assert_eq!(
            calls_to(&calls(&e), WORLD_ACTIVATE),
            vec![vec![0x3d3d, 1, 1, 1]; 2]
        );
        // A cell without a 3D activates nothing.
        returns(&mut e, CELL_NODE_OF_CELL, 0);
        start_log(&mut e);
        e.call(0x0055_8c90, &args![0x1234u32]);
        assert!(calls_to(&calls(&e), WORLD_ACTIVATE).is_empty());
    }

    #[test]
    fn the_small_class_constructors_set_their_vtables() {
        let mut e = engine();
        for (entry, base, vtable) in [
            (0x0055_8d40u32, SMALL_CLASS_BASE_A, SMALL_CLASS_VTABLE_A),
            (0x0055_8d70, SMALL_CLASS_BASE_B, SMALL_CLASS_VTABLE_B),
            (0x0055_8da0, SMALL_CLASS_BASE_C, SMALL_CLASS_VTABLE_C),
        ] {
            quiet(&mut e, &[base]);
            let at = e.mem.alloc(0x20);
            start_log(&mut e);
            assert_eq!(e.call(entry, &args![at, 16u32]).u32(), at);
            assert_eq!(calls(&e), vec![(base, vec![at, 16])]);
            assert_eq!(e.mem.u32(at), vtable);
        }
    }

    #[test]
    fn the_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = engine();
        quiet(&mut e, &[SMALL_CLASS_DESTRUCT, DEALLOCATE_BLOCK]);
        start_log(&mut e);
        assert_eq!(e.call(0x0055_8df0, &args![0x5000u32, 0u32]).u32(), 0x5000);
        assert_eq!(calls(&e), vec![(SMALL_CLASS_DESTRUCT, vec![0x5000])]);
        start_log(&mut e);
        assert_eq!(e.call(0x0055_8df0, &args![0x5000u32, 3u32]).u32(), 0x5000);
        assert_eq!(
            addresses(&calls(&e)),
            vec![SMALL_CLASS_DESTRUCT, DEALLOCATE_BLOCK]
        );
        start_log(&mut e);
        e.call(0x0055_8df0, &args![0x5000u32, 2u32]);
        assert_eq!(addresses(&calls(&e)), vec![SMALL_CLASS_DESTRUCT]);
    }
}
