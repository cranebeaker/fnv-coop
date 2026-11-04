//! `fallout shared/bgssceneinfo.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BGSSceneInfo` is the tool behind the memory-budget reports: it walks a
//! scene graph (or the cells of the world) and adds up counters (instances,
//! geometry, lights, decals, an estimate of the memory the geometry takes)
//! into a [`SceneInfoDataStruct`] of sixteen 32-bit counters.
//!
//! Session 1 (this file's first 40 functions, `004a7ca0` to `004a9de0`)
//! covers the collectors, the memory-budget text, the `NiTMap` and
//! `BSSimpleArray` template instances the unit emits, and the small
//! accessors. Session 2 translated the remaining 20 functions (`004a9e00`
//! to `004aa310` and `004bc320`): the unit is complete.
//!
//! Conventions used below:
//!
//! - The compiler's exception-unwinding frames (`__CxxFrameHandler` states,
//!   the stack-cookie checks) are not translated.
//! - `00559450(slot)` is the load of the pointer stored in `slot` (the
//!   `NiPointer` getter); every `+0xb8` pointer of a geometry-data object
//!   goes through it, and so do the list heads below.
//! - Several flags of `CollectSceneDataRecursive` are compile-time false
//!   (a byte set to zero and never written, tested as `flag && other`);
//!   the blocks they guard never run and are left out.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, NiTMap};

// ---------------------------------------------------------------------
// Layouts
// ---------------------------------------------------------------------

layout! {
    /// `SCENE_INFO_DATA_STRUCT` (Xbox PDB: one array of 64 bytes): the
    /// sixteen 32-bit scene totals the collectors add up. The field names
    /// are the byte offsets; what each counter counts is described where
    /// the collectors add to it.
    pub struct SceneInfoDataStruct: 0x40 {
        /// Incremented once per accepted reference (see
        /// [`bgs_scene_info_collect_scene_data_recursive`]).
        0x00 counter_00: u32,
        /// References whose form's type byte is `0x2a` to `0x2d`.
        0x04 counter_04: u32,
        /// References for which `00565580` (time controllers) is true.
        0x08 counter_08: u32,
        /// Among those, the ones whose form ID was not yet in the
        /// animated-forms list.
        0x0C counter_0c: u32,
        /// Sum of the `u16` that `004a8ab0` returns for objects that cast to
        /// the type at `012024e0`.
        0x10 counter_10: u32,
        /// Number of objects that cast to the type at `012024e0`.
        0x14 counter_14: u32,
        /// Objects that are not scene nodes but have geometry (virtual
        /// slot `0x18` of the object is non-null).
        0x18 counter_18: u32,
        /// Sum of the `u16` that `004a1ff0` returns for geometry data.
        0x1C counter_1c: u32,
        /// Used by [`fn_004a7d20`] only (references found in a cell).
        0x20 counter_20: u32,
        /// Memory estimate of geometry data (see [`geometry_memory_estimate`]).
        0x24 counter_24: u32,
        /// The context word handed to the property visitor `004a8b40`
        /// (the address of this counter is what it receives).
        0x28 counter_28: u32,
        /// Used by [`fn_004a7d20`] only (memory estimate of the objects of
        /// a cell).
        0x2C counter_2c: u32,
        /// Geometry lit by a `ShadowSceneNode` light.
        0x30 counter_30: u32,
        /// Lights whose use count reached the `uLightExcessGeometry`
        /// setting.
        0x34 counter_34: u32,
        /// Decals (the decal reference list entries, and geometry whose
        /// properties pass the decal tests).
        0x38 counter_38: u32,
        /// Sum of [`fn_004a9240`] over the model of a reference.
        0x3C counter_3c: u32,
    }

    /// `BGSSceneInfo` (Xbox PDB), 0x48 bytes: vtable, the scene data at
    /// +0x04 and the associated form at +0x44.
    pub struct BGSSceneInfo: 0x48 {
        /// `SceneData` (Xbox PDB): a [`SceneInfoDataStruct`].
        0x04 SceneData: Inline<SceneInfoDataStruct>,
        /// `pAssociatedForm` (Xbox PDB).
        0x44 pAssociatedForm: Ptr,
    }
}

// ---------------------------------------------------------------------
// Constants: vtables, globals, callees outside this file
// ---------------------------------------------------------------------

/// Vtable of `BGSSceneInfo`.
const BGS_SCENE_INFO_VTABLE: u32 = 0x0101_ec68;
/// Vtable of `NiTMap<NiTexture *, TEX_USER_DATA *>`.
const TEXTURE_USER_DATA_MAP_VTABLE: u32 = 0x0101_ecec;
/// Vtable of `NiTMap<TESObjectCELL *, bool>`.
const CELL_FLAG_MAP_VTABLE: u32 = 0x0101_ed0c;
/// Vtable of `NiTMapBase<..., NiTexture *, TEX_USER_DATA *>`.
const TEXTURE_USER_DATA_MAP_BASE_VTABLE: u32 = 0x0101_ed2c;
/// Vtable of `NiTMapBase<..., TESObjectCELL *, bool>`.
const CELL_FLAG_MAP_BASE_VTABLE: u32 = 0x0101_ed4c;
/// Vtable of `NiTMap<NiTexture *, bool>`.
const TEXTURE_FLAG_MAP_VTABLE: u32 = 0x0101_ed6c;
/// Vtable of `NiTMapBase<..., NiTexture *, bool>`.
const TEXTURE_FLAG_MAP_BASE_VTABLE: u32 = 0x0101_ed8c;
/// Vtable of `BSSimpleArray<SCENE_INFO_DATA_STRUCT, 1024>`.
const SCENE_DATA_ARRAY_VTABLE: u32 = 0x0101_edac;

/// Vtable of `NiTMap<TESActorBase *, bool>`.
const ACTOR_FLAG_MAP_VTABLE: u32 = 0x0101_edc0;
/// Vtable of `NiTMapBase<..., TESActorBase *, bool>`.
const ACTOR_FLAG_MAP_BASE_VTABLE: u32 = 0x0101_ede0;
/// Vtable of `NiTMap<int, int>`.
const INT_MAP_VTABLE: u32 = 0x0101_ee00;
/// Vtable of `NiTMapBase<..., int, int>`.
const INT_MAP_BASE_VTABLE: u32 = 0x0101_ee20;
/// Vtable of `NiTMap<ShadowSceneLight *, int>`.
const LIGHT_INT_MAP_VTABLE: u32 = 0x0101_ee40;
/// Vtable of `NiTMapBase<..., ShadowSceneLight *, int>`.
const LIGHT_INT_MAP_BASE_VTABLE: u32 = 0x0101_ee60;

/// Size in bytes of one element of the scene-data array.
const SCENE_DATA_SIZE: u32 = 0x40;
/// Bucket count the scene collectors give their temporary maps.
const TEMPORARY_MAP_BUCKETS: u32 = 0x25;

/// `TES` singleton pointer (`pTES`).
const TES_SINGLETON: u32 = 0x011d_ea10;
/// `TESDataHandler` singleton pointer.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `ModelLoader` singleton pointer.
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// Static byte read by [`fn_004a82c0`] (the texture tracker is cleared at
/// the end of a collection when it is set).
const CLEAR_TEXTURE_TRACKER: u32 = 0x0118_868c;
/// Static byte `bIncludeAddons` (Xbox PDB).
const INCLUDE_ADDONS: u32 = 0x0118_868d;

/// `SceneDataAddonNodes` (Xbox PDB): `BSSimpleArray<SCENE_INFO_DATA_STRUCT,
/// 1024>`, one entry per addon node.
const SCENE_DATA_ADDON_NODES: Ptr<BSSimpleArray> = Ptr::new(0x011c_5ab4);
/// `ActorMap` (Xbox PDB): `NiTMap<TESActorBase *, bool>`.
const ACTOR_MAP: u32 = 0x011c_5be8;
/// `LightGeomMap` (Xbox PDB): `NiTMap<ShadowSceneLight *, int>`.
const LIGHT_GEOMETRY_MAP: u32 = 0x011c_5ad0;
/// `TextureTracker` (Xbox PDB): `NiTMap<NiTexture *, bool>`.
const TEXTURE_TRACKER: u32 = 0x011c_5c40;
/// `Animated3DList` (Xbox PDB): `BSSimpleList<unsigned int>` of form IDs.
const ANIMATED_3D_LIST: u32 = 0x011c_5b30;

/// INI settings (`Setting` objects; `0043d4d0` gives the address of the
/// value): `uLightExcessGeometry:BudgetCaps`.
const SETTING_LIGHT_EXCESS_GEOMETRY: u32 = 0x011c_5a90;
/// `uLoadedAreaNonActorMemoryBudgetCap:BudgetCaps`.
const SETTING_LOADED_AREA_NON_ACTOR_BUDGET: u32 = 0x011c_5b20;
/// `uWastelandLODBudgetAdjustment:BudgetCaps`.
const SETTING_WASTELAND_LOD_ADJUSTMENT: u32 = 0x011c_5ae0;
/// `uCityLODBudgetAdjustment:BudgetCaps`.
const SETTING_CITY_LOD_ADJUSTMENT: u32 = 0x011c_5c04;
/// `uActorMemoryBudgetCap:BudgetCaps`.
const SETTING_ACTOR_MEMORY_BUDGET: u32 = 0x011c_59e0;

/// `1048576.0` (`double`).
const MEGABYTE: u32 = 0x0101_ece0;
/// `1024.0` (`double`).
const KILOBYTE: u32 = 0x0101_ecb8;
/// `100.0` (`double`).
const HUNDRED: u32 = 0x0101_7a40;
/// `"%.2f%% (%.2f MB / %.2f MB)"`.
const FORMAT_MEGABYTES: u32 = 0x0101_ecc0;
/// `"%.2f%% (%.2f KB / %.2f MB)"`.
const FORMAT_KILOBYTES: u32 = 0x0101_ec9c;
/// `"%.2f%% (%.0f bytes / %.2f MB)"`.
const FORMAT_BYTES: u32 = 0x0101_ec7c;
/// `"FurnitureMarker"`.
const FURNITURE_MARKER: u32 = 0x0101_ec6c;
/// `"Meshes\\"`.
const MESHES_PREFIX: u32 = 0x0101_6fac;

/// RTTI type descriptor the node tests and casts use (a node that carries
/// an addon-node index at its `009ee040`).
const TYPE_NODE_WITH_ADDON_INDEX: u32 = 0x0120_2de8;
/// Type descriptor of the root of a loaded addon model.
const TYPE_ADDON_MODEL_ROOT: u32 = 0x011f_4428;
/// Type descriptor of the objects `ShadowSceneNode::GetLight` takes.
const TYPE_LIGHT_OBJECT: u32 = 0x011f_4a28;
/// Type descriptor whose cast objects `004a8ab0` reads.
const TYPE_ACTOR_OBJECT: u32 = 0x0120_24e0;
/// Type descriptor tested on the parent node (decal tests).
const TYPE_DECAL_PARENT: u32 = 0x011c_7d34;

/// The property visitor `004a8b40` handed to the shader property.
const PROPERTY_VISITOR_CALLBACK: u32 = 0x004a_8b40;

/// `operator delete` (`00401030`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `memset(block, value, size)`.
const MEMSET: u32 = 0x0040_3d30;
/// `memmove(destination, source, size)` (`LIBCMT memmovep`).
const MEMMOVE: u32 = 0x00ec_7230;
/// `_strnicmp(a, b, count)`.
const STRING_COMPARE_NO_CASE: u32 = 0x00ec_7ec0;
/// `strcpy_s(destination, size, source)`.
const STRING_COPY: u32 = 0x0040_6d30;
/// `strcat_s(destination, size, source)`.
const STRING_APPEND: u32 = 0x0040_6d50;
/// `sprintf(destination, format, ...)`.
const STRING_FORMAT: u32 = 0x0040_6f60;
/// Placement `operator new(size, where)`: returns `where`.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// The load of the pointer stored in the slot that is its `this`.
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `NiTMapBase::RemoveAll` (empties a map; `this` = map).
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `NiTMapBase::GetAt(map, key, &value)` -> found.
const MAP_GET_AT: u32 = 0x0057_c850;
/// `NiTMapBase::SetAt(map, key, value)`.
const MAP_SET_AT: u32 = 0x0084_d310;
/// `Setting` value address (`this ? &this->value : &0`).
const SETTING_VALUE_ADDRESS: u32 = 0x0043_d4d0;
/// `NiTList` walk: next node of `node` (`this` = list, one argument).
const LIST_NEXT_NODE: u32 = 0x007b_52d0;
/// `NiTList` walk: address of the element slot of `node` (`node + 8`).
const LIST_ELEMENT_SLOT: u32 = 0x0063_17a0;
/// `BSSimpleList` node: its own address (the item slot is the first word).
const SIMPLE_LIST_NODE: u32 = 0x0068_15c0;
/// `BSSimpleList` node: the next node (`node + 4`).
const SIMPLE_LIST_NEXT: u32 = 0x0072_6070;
/// `BSSimpleArray::IsFull` (`iSize == iReservedSize`).
const ARRAY_IS_FULL: u32 = 0x0043_8b90;
/// `BSSimpleArray` growth rule: the next capacity.
const ARRAY_NEXT_CAPACITY: u32 = 0x009a_3910;
/// `BSSimpleArray` free of the buffer (virtual `Deallocate`), clears
/// `pBuffer`.
const ARRAY_FREE_BUFFER: u32 = 0x006a_8500;
/// `BSSimpleArray` append of an empty slot: grows, bumps the size and
/// returns the new index.
const ARRAY_APPEND_SLOT: u32 = 0x006b_3fd0;
/// `BSSimpleArray` element address: `pBuffer + index * 0x40`.
const ARRAY_ELEMENT_ADDRESS: u32 = 0x006b_30e0;
/// `BSSimpleArray` constructor of the scene-data array
/// (`this`, reserve, size).
const SCENE_DATA_ARRAY_CONSTRUCT: u32 = 0x004a_9e00;
/// Zeroes a `SCENE_INFO_DATA_STRUCT` (`memset(this, 0, 0x40)`).
const SCENE_DATA_CLEAR: u32 = 0x004a_7c80;
/// `BSSimpleArray<SCENE_INFO_DATA_STRUCT, 1024>` `RemoveAll(freeBuffer)`.
const ARRAY_REMOVE_ALL: u32 = 0x0084_54f0;
/// Bucket array allocation and release of the `NiTMapBase` instances.
const ALLOCATE: u32 = 0x00aa_1070;
const DEALLOCATE: u32 = 0x00aa_10f0;
/// Destructor bodies of the folded `NiTMapBase` instances this unit has no
/// translation of yet.
const TEXTURE_USER_DATA_MAP_DESTROY: u32 = 0x004a_9850;
const TEXTURE_USER_DATA_MAP_BASE_DESTROY: u32 = 0x004a_98b0;
const CELL_FLAG_MAP_BASE_DESTROY: u32 = 0x004a_99d0;
/// `NiTMap<TESObjectREFR *, bool>` constructor (`this`, buckets) and
/// destructor body.
const REFERENCE_FLAG_MAP_CONSTRUCT: u32 = 0x0042_f4e0;
const REFERENCE_FLAG_MAP_DESTROY: u32 = 0x0042_fe80;

// Virtual slots (byte offsets) of the objects the collectors walk.
/// Slot `0x0c` of a scene object: the object seen as a node (null when it
/// is not one).
const SLOT_AS_NODE: u32 = 0x0c;
/// Slot `0x10`: non-null for the nodes that carry a reference.
const SLOT_AS_REFERENCE_NODE: u32 = 0x10;
/// Slot `0x18`: the object seen as geometry (null when it is not).
const SLOT_AS_GEOMETRY: u32 = 0x18;
/// Slot `0x1c` of a geometry: its geometry data.
const SLOT_GEOMETRY_DATA: u32 = 0x1c;
/// Slot `0xbc` of a property: visit with a callback and a context.
const SLOT_VISIT: u32 = 0xbc;
/// Slot `0x90` of the object `+0xb8` points at: its vertex count.
const SLOT_VERTEX_COUNT: u32 = 0x90;
/// Slot `0x1d0` of a form in a cell's list.
const SLOT_FORM_OWNER_OBJECT: u32 = 0x1d0;
/// Slot `0x04` of a `BSSimpleArray`: allocate.
const SLOT_ARRAY_ALLOCATE: u32 = 0x04;

// ---------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------

/// `*slot` through the game's pointer getter (`00559450`).
fn pointer_in_slot(e: &mut Engine, slot: Ptr) -> u32 {
    e.call(NI_POINTER_GET, &args![slot]).u32()
}

/// Adds `by` to a counter of a scene-data struct (32-bit wrap-around).
fn bump(
    e: &mut Engine,
    data: Ptr<SceneInfoDataStruct>,
    field: Field<SceneInfoDataStruct, u32>,
    by: u32,
) {
    let value = e.get(data, field).wrapping_add(by);
    e.set(data, field, value);
}

/// Address of element `index` of a buffer of 0x40-byte elements.
fn element_address(buffer: u32, index: u32) -> u32 {
    buffer.wrapping_add(index.wrapping_mul(SCENE_DATA_SIZE))
}

/// The value of a `Setting` (`*0043d4d0(setting)`).
fn setting_value(e: &mut Engine, setting: u32) -> u32 {
    let address = e.call(SETTING_VALUE_ADDRESS, &args![setting]).u32();
    e.mem.u32(address)
}

/// The form ID of a reference: `0084e3a0(007af430(reference))`.
fn reference_form_id(e: &mut Engine, reference: u32) -> u32 {
    let form = e.call(0x007a_f430, &args![reference]).u32();
    e.call(0x0084_e3a0, &args![form]).u32()
}

/// The memory estimate both collectors compute for geometry data `data`
/// (the same inline sequence in [`fn_004a7d20`] and
/// [`bgs_scene_info_collect_scene_data_recursive`]): with the vertex count
/// `n` (`u16`), 12 bytes each if `0049ec60` is non-zero, 16 if `004a8050`,
/// 12 if `004a8030`, 8 if `004a8070`.
fn geometry_memory_estimate(e: &mut Engine, data: Ptr) -> u32 {
    let count = fn_004a8090(e, data) & 0xffff;
    let mut total = 0u32;
    if e.call(0x0049_ec60, &args![data]).u32() != 0 {
        total = total.wrapping_add(count.wrapping_mul(12));
    }
    if fn_004a8050(e, data) != 0 {
        total = total.wrapping_add(count << 4);
    }
    if fn_004a8030(e, data) != 0 {
        total = total.wrapping_add(count.wrapping_mul(12));
    }
    if fn_004a8070(e, data) != 0 {
        total = total.wrapping_add(count.wrapping_mul(8));
    }
    total
}

/// Frees `this` with `operator delete` when bit 0 of `flags` is set (the
/// tail of every scalar deleting destructor).
fn delete_if_flagged(e: &mut Engine, this: Ptr, flags: u32) {
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
}

/// Body shared by the `NiTMapBase` constructors of this unit: stores the
/// vtable, the bucket count (+4) and a zero count (+0x0C), allocates
/// `buckets * 4` bytes for the bucket array (+8) and clears them.
fn construct_map_base(e: &mut Engine, this: Ptr<NiTMap>, vtable: u32, buckets: u32) -> Ptr<NiTMap> {
    e.mem.set_u32(this.addr(), vtable);
    e.set(this, NiTMap::m_uiHashSize, buckets);
    e.set(this, NiTMap::m_uiCount, 0);
    let table = e.call(ALLOCATE, &args![buckets << 2]).u32();
    e.set(this, NiTMap::m_ppkHashTable, table);
    let table = e.get(this, NiTMap::m_ppkHashTable);
    let size = e.get(this, NiTMap::m_uiHashSize) << 2;
    e.call(MEMSET, &args![table, 0u32, size]);
    this
}

/// Body shared by the `NiTMapBase` destructor bodies added in session 2:
/// stores the base vtable, empties the map (`00438af0`) and frees the
/// bucket array at +8 (`00aa10f0`).
fn destroy_map_base(e: &mut Engine, this: Ptr<NiTMap>, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTMap::m_ppkHashTable);
    e.call(DEALLOCATE, &args![table]);
}

// ---------------------------------------------------------------------
// BGSSceneInfo
// ---------------------------------------------------------------------

// Translated from 004a7ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSceneInfo::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body [`fn_004a7cd0`] and frees the object when bit 0 of
/// `flags` is set. Returns `this`.
pub fn bgs_scene_info_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BGSSceneInfo>,
    flags: u32,
) -> Ptr<BGSSceneInfo> {
    fn_004a7cd0(e, this);
    delete_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 004a7cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `BGSSceneInfo`: stores its vtable.
pub fn fn_004a7cd0(e: &mut Engine, this: Ptr<BGSSceneInfo>) {
    e.mem.set_u32(this.addr(), BGS_SCENE_INFO_VTABLE);
}

// Translated from 004a7cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counter `index` (0 to 15) of the scene data, in order of offset
/// (`this + 4 + index * 4`); 0 for an index outside that range.
pub fn fn_004a7cf0(e: &mut Engine, this: Ptr<BGSSceneInfo>, index: i32) -> u32 {
    if (0..16).contains(&index) {
        e.mem.u32(this.addr() + 4 + (index as u32) * 4)
    } else {
        0
    }
}

// Translated from 004a7d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the world's list of cells (the list at `+0x3c` of the object at
/// `+0x64` of the `TES` singleton). For every entry it visits the list at
/// `+0x24` of the entry's object and counts in `counter_20` (`this + 0x24`)
/// the references whose word `+0x40` (their parent cell) is `cell`, once
/// per reference (a map of references already counted guards against
/// counting one twice; a second map keyed by parent cell is emptied after
/// every entry). Then it walks the list of objects of `cell` (the list at
/// `+0x5c` of the object at `+0xc4`) and adds the geometry-memory estimate
/// of each object whose model has geometry data to `counter_2c`
/// (`this + 0x30`).
///
/// Two `NiTMap`s with 0x25 buckets are built on the stack and destroyed at
/// the end (the exception frame is not translated). The compiler kept two
/// dead stores (a counter of references not yet in the cell map and a flag
/// after the final lookup of `cell`); only the lookups remain.
pub fn fn_004a7d20(e: &mut Engine, this: Ptr<BGSSceneInfo>, cell: u32) {
    let tes = e.global::<u32>(TES_SINGLETON);
    let holder = e.call(0x0070_ec90, &args![tes]).u32();
    let cell_list = e.call(0x005a_8080, &args![holder]).u32();

    let cell_map = Ptr::<NiTMap>::new(e.mem.alloc(0x10));
    fn_004a9600(e, cell_map, TEMPORARY_MAP_BUCKETS);
    let reference_map = e.mem.alloc(0x10);
    e.call(
        REFERENCE_FLAG_MAP_CONSTRUCT,
        &args![reference_map, TEMPORARY_MAP_BUCKETS],
    );
    let flag = e.mem.alloc(4);

    let mut node = pointer_in_slot(e, Ptr::new(cell_list));
    while node != 0 {
        let next = e.call(LIST_NEXT_NODE, &args![cell_list, node]).u32();
        let slot = e.call(LIST_ELEMENT_SLOT, &args![cell_list, node]).u32();
        let owner = e.mem.u32(slot);
        if owner != 0 {
            let owner_list = owner + 0x24;
            let mut inner = pointer_in_slot(e, Ptr::new(owner_list));
            while inner != 0 {
                let inner_next = e.call(LIST_NEXT_NODE, &args![owner_list, inner]).u32();
                let slot = e.call(LIST_ELEMENT_SLOT, &args![owner_list, inner]).u32();
                let reference = e.mem.u32(slot);

                let parent = e.call(0x008d_6f30, &args![reference]).u32();
                e.call(MAP_GET_AT, &args![cell_map, parent, flag]);
                let seen = e
                    .call(MAP_GET_AT, &args![reference_map, reference, flag])
                    .bool();
                if !seen && e.call(0x008d_6f30, &args![reference]).u32() == cell {
                    let count = e.mem.u32(this.addr() + 0x24).wrapping_add(1);
                    e.mem.set_u32(this.addr() + 0x24, count);
                    e.call(MAP_SET_AT, &args![reference_map, reference, 1u32]);
                }
                let parent = e.call(0x008d_6f30, &args![reference]).u32();
                e.call(MAP_SET_AT, &args![cell_map, parent, 1u32]);
                inner = inner_next;
            }
            e.call(MAP_GET_AT, &args![cell_map, cell, flag]);
        }
        e.call(MAP_REMOVE_ALL, &args![cell_map]);
        node = next;
    }

    let mut object_node = e.call(0x0054_5710, &args![cell]).u32();
    while object_node != 0 {
        let slot = e.call(SIMPLE_LIST_NODE, &args![object_node]).u32();
        let form = e.mem.u32(slot);
        if form == 0 {
            break;
        }
        let owner_object = e.vcall(form, SLOT_FORM_OWNER_OBJECT, &[]).u32();
        if owner_object != 0 && e.vcall(owner_object, SLOT_AS_REFERENCE_NODE, &[]).u32() != 0 {
            let model = e.call(0x0043_b4a0, &args![owner_object, 0u32]).u32();
            let geometry_data = e.vcall(model, SLOT_GEOMETRY_DATA, &[]).u32();
            if geometry_data != 0 {
                let estimate = geometry_memory_estimate(e, Ptr::new(geometry_data));
                let total = e.mem.u32(this.addr() + 0x30).wrapping_add(estimate);
                e.mem.set_u32(this.addr() + 0x30, total);
            }
        }
        object_node = e.call(SIMPLE_LIST_NEXT, &args![object_node]).u32();
    }

    e.call(MAP_REMOVE_ALL, &args![reference_map]);
    e.call(REFERENCE_FLAG_MAP_DESTROY, &args![reference_map]);
    fn_004a9970(e, cell_map);
    e.mem.free(flag);
    e.mem.free(reference_map);
    e.mem.free(cell_map.addr());
}

// Translated from 004a8030 (decompiled, FalloutNV.exe 1.4.0.525)
/// The accessor `0059bb30` (a word at +0x24) on the object that `+0xb8` of
/// `this` points at.
pub fn fn_004a8030(e: &mut Engine, this: Ptr) -> u32 {
    let data = pointer_in_slot(e, this.byte_add(0xb8));
    e.call(0x0059_bb30, &args![data]).u32()
}

// Translated from 004a8050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0045cd60` (a word at +0x28) on the object that `+0xb8` of `this`
/// points at.
pub fn fn_004a8050(e: &mut Engine, this: Ptr) -> u32 {
    let data = pointer_in_slot(e, this.byte_add(0xb8));
    e.call(0x0045_cd60, &args![data]).u32()
}

// Translated from 004a8070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0055b980` (a word at +0x2c) on the object that `+0xb8` of `this`
/// points at.
pub fn fn_004a8070(e: &mut Engine, this: Ptr) -> u32 {
    let data = pointer_in_slot(e, this.byte_add(0xb8));
    e.call(0x0055_b980, &args![data]).u32()
}

// Translated from 004a8090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual slot `0x90` (vertex count) of the object that `+0xb8` of `this`
/// points at; callers use its low 16 bits.
pub fn fn_004a8090(e: &mut Engine, this: Ptr) -> u32 {
    let data = pointer_in_slot(e, this.byte_add(0xb8));
    e.vcall(data, SLOT_VERTEX_COUNT, &[]).u32()
}

// Translated from 004a80c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSceneInfo::CollectSceneDataFromAddonNodes` (Xbox PDB): for every
/// addon node of the data handler (`0045a120` counts them) appends an empty
/// scene-data entry to [`SCENE_DATA_ADDON_NODES`], loads the node's model
/// (`"Meshes\\"` plus the node's file name) through the model loader and,
/// when the model loaded and `00c511f0` says no, collects its scene data
/// into the new entry; the model is released again.
///
/// The stack cookie check and the two dead flags of the original are not
/// translated.
pub fn bgs_scene_info_collect_scene_data_from_addon_nodes(e: &mut Engine, this: Ptr<BGSSceneInfo>) {
    let handler = e.global::<u32>(DATA_HANDLER);
    let count = e.call(0x0045_a120, &args![handler]).u32();
    for index in 0..count {
        let entry = e.mem.alloc(SCENE_DATA_SIZE);
        e.call(SCENE_DATA_CLEAR, &args![entry]);
        fn_004a9690(e, SCENE_DATA_ADDON_NODES, index, Ptr::new(entry));
        e.mem.free(entry);

        let handler = e.global::<u32>(DATA_HANDLER);
        let addon = e.call(0x0046_17e0, &args![handler, index]).u32();
        if addon == 0 {
            continue;
        }
        let name_holder = addon + 0x30;
        if e.call(0x0048_cee0, &args![name_holder]).u32() == 0 {
            continue;
        }
        let path = e.mem.alloc(0x104);
        e.call(STRING_COPY, &args![path, 0x104u32, MESHES_PREFIX]);
        let file_name = e.vcall(name_holder, 0x14, &[]).u32();
        e.call(STRING_APPEND, &args![path, 0x104u32, file_name]);
        let loader = e.global::<u32>(MODEL_LOADER);
        let model = e
            .call(
                0x0044_7080,
                &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
            )
            .u32();
        if model != 0 {
            if !e.call(0x00c5_11f0, &args![model]).bool() {
                let element = fn_004a95b0(e, SCENE_DATA_ADDON_NODES, index);
                let root = e
                    .call(0x0065_3270, &args![TYPE_ADDON_MODEL_ROOT, model])
                    .u32();
                bgs_scene_info_collect_scene_data_recursive(
                    e,
                    this,
                    Ptr::new(root),
                    element.cast(),
                    false,
                    Ptr::NULL,
                );
            }
            let loader = e.global::<u32>(MODEL_LOADER);
            e.call(0x0045_a5e0, &args![loader, path]);
        }
        e.mem.free(path);
    }
}

// Translated from 004a8230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSceneInfo::CollectSceneDataFromNode` (Xbox PDB): empties the actor
/// map and the light map, collects the addon nodes first when `with_addons`
/// and `bIncludeAddons` are set and the addon array is still empty, clears
/// this info's 0x40 counters, collects the scene graph below `node` into
/// them and, when [`fn_004a82c0`] says so, empties the texture tracker.
pub fn bgs_scene_info_collect_scene_data_from_node(
    e: &mut Engine,
    this: Ptr<BGSSceneInfo>,
    node: Ptr,
    with_addons: bool,
    descend_anyway: bool,
) {
    e.call(MAP_REMOVE_ALL, &args![ACTOR_MAP]);
    e.call(MAP_REMOVE_ALL, &args![LIGHT_GEOMETRY_MAP]);
    if with_addons
        && e.global::<u8>(INCLUDE_ADDONS) != 0
        && e.call(0x0044_ddc0, &args![SCENE_DATA_ADDON_NODES]).u32() == 0
    {
        bgs_scene_info_collect_scene_data_from_addon_nodes(e, this);
    }
    let data = this.byte_add(4);
    e.call(MEMSET, &args![data, 0u32, SCENE_DATA_SIZE]);
    bgs_scene_info_collect_scene_data_recursive(
        e,
        this,
        node,
        data.cast(),
        descend_anyway,
        Ptr::NULL,
    );
    if fn_004a82c0(e) {
        e.call(MAP_REMOVE_ALL, &args![TEXTURE_TRACKER]);
    }
}

// Translated from 004a82c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The static byte at `0118868c` (whether the texture tracker is cleared
/// after a collection).
pub fn fn_004a82c0(e: &mut Engine) -> bool {
    e.global::<u8>(CLEAR_TEXTURE_TRACKER) != 0
}

// Translated from 004a82d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSceneInfo::CollectSceneDataRecursive` (Xbox PDB): adds the counters
/// of the scene object `node` to `data_param` (this info's own scene data
/// when it is null) and recurses into its children. `descend_anyway` lets
/// the walk go below a reference whose form type is `0x2a` to `0x2d`;
/// `parent` is the node the object hangs from (null at the top).
///
/// For an object that is a node (virtual slot `0x0c` non-null):
/// - a node named like `"FurnitureMarker"` (first 15 characters, ignoring
///   case) ends the walk without any further effect;
/// - if it is of the type at `01202de8`, the addon scene data of the entry
///   its index selects is added to `data` (counters 4 to 15);
/// - when slot `0x10` is non-null and the reference (`009ad610`) exists and
///   `00444ed0` is false, the instance counter, the time-controller
///   counters (with the animated-forms list), the decal list entries, the
///   form-type counter and the model sum `counter_3c` are updated;
/// - the children are visited with `data_param` passed on unchanged.
///
/// For other objects: a geometry (slot `0x18`) adds to the geometry counters
/// (visitor callback, decal tests, lights, vertex estimate, additional
/// geometry data); a non-geometry that casts to the type at `011f4a28`
/// counts in `counter_30` if `ShadowSceneNode::GetLight` finds a light. Any
/// object that casts to the type at `012024e0` counts in `counter_14` and
/// adds to `counter_10`.
///
/// The `TES` call that every path but the two early returns ends with is
/// `0045a1c0(TES, 0)`.
pub fn bgs_scene_info_collect_scene_data_recursive(
    e: &mut Engine,
    this: Ptr<BGSSceneInfo>,
    node: Ptr,
    data_param: Ptr<SceneInfoDataStruct>,
    descend_anyway: bool,
    parent: Ptr,
) {
    let data: Ptr<SceneInfoDataStruct> = if data_param.is_null() {
        this.byte_add(4).cast()
    } else {
        data_param
    };
    if !node.is_null() {
        let as_node = e.vcall(node.addr(), SLOT_AS_NODE, &[]).u32();
        if as_node != 0 {
            // Node name (`NiFixedString` at +8 of the node).
            let name_slot = e.call(0x0041_3f40, &args![as_node]).u32();
            if e.call(0x0043_b1b0, &args![name_slot]).u32() != 0 {
                let name_slot = e.call(0x0041_3f40, &args![as_node]).u32();
                let name = e.call(0x0043_b1b0, &args![name_slot]).u32();
                let same = e
                    .call(
                        STRING_COMPARE_NO_CASE,
                        &args![name, FURNITURE_MARKER, 15u32],
                    )
                    .i32();
                if same == 0 {
                    return;
                }
            }
            if e.call(0x0043_b300, &args![TYPE_NODE_WITH_ADDON_INDEX, as_node])
                .bool()
            {
                let typed = e
                    .call(0x0065_3270, &args![TYPE_NODE_WITH_ADDON_INDEX, as_node])
                    .u32();
                let index = e.call(0x009e_e040, &args![typed]).i32();
                let handler = e.global::<u32>(DATA_HANDLER);
                if index >= 0
                    && index < e.call(0x0045_a120, &args![handler]).i32()
                    && index < e.call(0x0044_ddc0, &args![SCENE_DATA_ADDON_NODES]).i32()
                {
                    for slot in 0..16u32 {
                        if slot > 3 {
                            let element = fn_004a95b0(e, SCENE_DATA_ADDON_NODES, index as u32);
                            let added = e.mem.u32(element.addr() + slot * 4);
                            let total = e.mem.u32(data.addr() + slot * 4).wrapping_add(added);
                            e.mem.set_u32(data.addr() + slot * 4, total);
                        }
                    }
                }
            }

            let mut stop_below = false;
            let mut fade_node = 0;
            if e.vcall(as_node, SLOT_AS_REFERENCE_NODE, &[]).u32() != 0 {
                fade_node = as_node;
            }
            if fade_node != 0 {
                let reference = e.call(0x009a_d610, &args![fade_node]).u32();
                if reference != 0 && !e.call(0x0044_4ed0, &args![reference]).bool() {
                    bump(e, data, SceneInfoDataStruct::counter_00, 1);
                    if e.call(0x0056_5580, &args![node]).bool() {
                        bump(e, data, SceneInfoDataStruct::counter_08, 1);
                        let form_id = reference_form_id(e, reference);
                        let known = e.with_stack(4, |e, key| {
                            e.mem.set_u32(key.addr(), form_id);
                            e.call(0x005f_65d0, &args![ANIMATED_3D_LIST, key]).bool()
                        });
                        if !known {
                            bump(e, data, SceneInfoDataStruct::counter_0c, 1);
                            let form_id = reference_form_id(e, reference);
                            e.with_stack(4, |e, key| {
                                e.mem.set_u32(key.addr(), form_id);
                                e.call(0x0090_5820, &args![ANIMATED_3D_LIST, key]);
                            });
                        }
                    }
                    let extra = e.call(0x005d_43c0, &args![reference]).u32();
                    let mut decal = e.call(0x0041_f050, &args![extra]).u32();
                    while decal != 0 {
                        let slot = e.call(SIMPLE_LIST_NODE, &args![decal]).u32();
                        if e.mem.u32(slot) == 0 {
                            break;
                        }
                        bump(e, data, SceneInfoDataStruct::counter_38, 1);
                        decal = e.call(SIMPLE_LIST_NEXT, &args![decal]).u32();
                    }
                    let form = e.call(0x007a_f430, &args![reference]).u32();
                    let form_type = e.call(0x0040_1170, &args![form]).u32();
                    if (0x2a..=0x2d).contains(&form_type) {
                        bump(e, data, SceneInfoDataStruct::counter_04, 1);
                        stop_below = true;
                    }
                    let owner = e.call(0x004a_8b00, &args![node]).u32();
                    if owner != 0 {
                        let holder = e.call(0x006f_a820, &args![owner]).u32();
                        let model_slot = if holder != 0 {
                            e.call(0x0043_b560, &args![holder]).u32()
                        } else {
                            0
                        };
                        let model = if model_slot != 0 {
                            pointer_in_slot(e, Ptr::new(model_slot))
                        } else {
                            0
                        };
                        if model != 0 {
                            let sum = fn_004a9240(e, Ptr::new(model));
                            bump(e, data, SceneInfoDataStruct::counter_3c, sum);
                        }
                    }
                }
            }
            if stop_below && !descend_anyway {
                return;
            }
            let children = e.call(0x0043_b480, &args![as_node]).u32();
            for child_index in 0..children {
                let child = e.call(0x0043_b4a0, &args![as_node, child_index]).u32();
                bgs_scene_info_collect_scene_data_recursive(
                    e,
                    this,
                    Ptr::new(child),
                    data_param,
                    false,
                    Ptr::new(as_node),
                );
            }
        } else {
            let geometry = e.vcall(node.addr(), SLOT_AS_GEOMETRY, &[]).u32();
            if geometry != 0 {
                collect_geometry(e, data, geometry, parent);
            } else {
                let light_object = e.call(0x0065_3270, &args![TYPE_LIGHT_OBJECT, node]).u32();
                if light_object != 0 {
                    let scene_node = e.call(0x0045_0b80, &args![0u32]).u32();
                    if scene_node != 0
                        && e.call(0x00b5_b4a0, &args![scene_node, light_object]).u32() != 0
                    {
                        bump(e, data, SceneInfoDataStruct::counter_30, 1);
                    }
                }
            }
            let actor_object = e.call(0x0065_3270, &args![TYPE_ACTOR_OBJECT, node]).u32();
            if actor_object != 0 {
                bump(e, data, SceneInfoDataStruct::counter_14, 1);
                let value = u32::from(fn_004a8ab0(e, Ptr::new(actor_object)));
                bump(e, data, SceneInfoDataStruct::counter_10, value);
            }
        }
    }
    let tes = e.global::<u32>(TES_SINGLETON);
    e.call(0x0045_a1c0, &args![tes]);
}

/// The geometry branch of [`bgs_scene_info_collect_scene_data_recursive`]
/// (`geometry` is virtual slot `0x18` of the object, non-null).
fn collect_geometry(e: &mut Engine, data: Ptr<SceneInfoDataStruct>, geometry: u32, parent: Ptr) {
    bump(e, data, SceneInfoDataStruct::counter_18, 1);

    // The property of type 3 (`NiAVObject::GetProperty`), kept only if its
    // word at +0x1c is not -1.
    let property = e.call(0x00a5_9d30, &args![geometry, 3u32]).u32();
    let visited = if property == 0 {
        0
    } else if e.call(0x0044_1110, &args![property]).i32() != -1 {
        property
    } else {
        0
    };
    if visited != 0 {
        e.with_stack(4, |e, context| {
            e.mem.set_u32(context.addr(), data.addr() + 0x28);
            e.vcall(
                visited,
                SLOT_VISIT,
                &args![PROPERTY_VISITOR_CALLBACK, context],
            );
        });
        let decal = (!parent.is_null()
            && e.call(0x0043_b300, &args![TYPE_DECAL_PARENT, parent])
                .bool())
            || e.call(0x004a_2020, &args![visited, 0x1au32]).bool()
            || e.call(0x004a_2020, &args![visited, 0x1bu32]).bool();
        if decal {
            bump(e, data, SceneInfoDataStruct::counter_38, 1);
        }
    }

    // The same property, kept only if its word at +0x1c is 8 to 12: its
    // active lights are counted per light.
    let lit = if property == 0 {
        0
    } else {
        let range = e.call(0x0044_1110, &args![property]).i32();
        if (8..=0x0c).contains(&range) {
            property
        } else {
            0
        }
    };
    if lit != 0 {
        e.call(0x00b7_0790, &args![lit]);
        e.with_stack(8, |e, scratch| {
            let iterator = scratch;
            let count = scratch.byte_add(4);
            let mut light = e.call(0x00b7_0590, &args![lit, iterator]).u32();
            while light != 0 {
                e.mem.set_u32(count.addr(), 0);
                e.call(0x0085_3130, &args![LIGHT_GEOMETRY_MAP, light, count]);
                let uses = e.mem.u32(count.addr()).wrapping_add(1);
                e.mem.set_u32(count.addr(), uses);
                e.call(0x0084_4700, &args![LIGHT_GEOMETRY_MAP, light, uses]);
                let limit = setting_value(e, SETTING_LIGHT_EXCESS_GEOMETRY);
                if uses == limit {
                    bump(e, data, SceneInfoDataStruct::counter_34, 1);
                }
                light = e.call(0x00b7_0680, &args![lit, iterator]).u32();
            }
        });
    }

    // Geometry data: vertex counter and memory estimate.
    let geometry_data = e.vcall(geometry, SLOT_GEOMETRY_DATA, &[]).u32();
    if geometry_data != 0 {
        let vertices = e.call(0x004a_1ff0, &args![geometry_data]).u32() & 0xffff;
        bump(e, data, SceneInfoDataStruct::counter_1c, vertices);
        let estimate = geometry_memory_estimate(e, Ptr::new(geometry_data));
        bump(e, data, SceneInfoDataStruct::counter_24, estimate);
    }

    // Additional geometry data streams.
    if e.call(0x0054_95f0, &args![geometry]).u32() != 0 {
        let streams = e.call(0x0054_95f0, &args![geometry]).u32();
        let list = e.call(0x004a_8a90, &args![streams]).u32();
        let mut index = 0u32;
        while list != 0 && index < e.call(0x00a7_27b0, &args![list]).u32() {
            let size = e.with_stack(4, |e, out| {
                e.mem.set_u32(out.addr(), 0);
                e.call(0x00a7_2550, &args![list, index, out]);
                e.mem.u32(out.addr())
            });
            bump(e, data, SceneInfoDataStruct::counter_24, size);
            index += 1;
        }
    }
}

// Translated from 004a8ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004a8ae0` (the `u16` at +8; the engine map names it
/// `TESActorBaseData::GetFatigue`) on the object that `+0xb8` of `this`
/// points at.
pub fn fn_004a8ab0(e: &mut Engine, this: Ptr) -> u16 {
    let data = pointer_in_slot(e, this.byte_add(0xb8));
    e.call(0x004a_8ae0, &args![data]).u16()
}

// Translated from 004a9050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The memory budget of the loaded area: the value of
/// `uLoadedAreaNonActorMemoryBudgetCap`, reduced (when the `TES` object has
/// no interior cell at +0x34 and its current worldspace, followed up the
/// chain of `00586390(world, 1)` parents, exists) by the wasteland LOD
/// adjustment if that worldspace's form ID is `0x3c`, or by the city LOD
/// adjustment otherwise.
pub fn fn_004a9050(e: &mut Engine) -> i32 {
    let mut budget = setting_value(e, SETTING_LOADED_AREA_NON_ACTOR_BUDGET) as i32;
    let tes = e.global::<u32>(TES_SINGLETON);
    if e.call(0x005f_36f0, &args![tes]).u32() == 0 {
        let mut world = e.call(0x004f_d3e0, &args![tes]).u32();
        while world != 0 {
            if e.call(0x0058_6390, &args![world, 1u32]).u32() == 0 {
                break;
            }
            world = e.call(0x0058_6390, &args![world, 1u32]).u32();
        }
        if world != 0 {
            let adjustment = if e.call(0x0084_e3a0, &args![world]).u32() == 0x3c {
                setting_value(e, SETTING_WASTELAND_LOD_ADJUSTMENT)
            } else {
                setting_value(e, SETTING_CITY_LOD_ADJUSTMENT)
            };
            budget = budget.wrapping_sub(adjustment as i32);
        }
    }
    budget
}

// Translated from 004a9100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of `uActorMemoryBudgetCap`.
pub fn fn_004a9100(e: &mut Engine) -> u32 {
    setting_value(e, SETTING_ACTOR_MEMORY_BUDGET)
}

// Translated from 004a9120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSceneInfo::BuildMNumberString` (Xbox PDB), cdecl with no `this`:
/// writes into `text` the share of `value` bytes in a budget (the actor
/// memory budget [`fn_004a9100`] if `actor_budget`, else the area budget
/// [`fn_004a9050`]) as `"%.2f%% (<amount> / %.2f MB)"`, where the amount is
/// in bytes up to 1024, in KB up to 1048576 and in MB above (the budget is
/// always shown in MB). x87 `float` stores are modelled by rounding the
/// intermediate results to `f32`.
pub fn bgs_scene_info_build_m_number_string(
    e: &mut Engine,
    value: f32,
    text: Ptr,
    actor_budget: bool,
) {
    let budget = if actor_budget {
        fn_004a9100(e)
    } else {
        fn_004a9050(e) as u32
    };
    // `FILD` of the unsigned budget, stored as a `float`.
    let mut budget_bytes = budget as f32;
    let hundred: f64 = e.global(HUNDRED);
    let megabyte: f64 = e.global(MEGABYTE);
    let kilobyte: f64 = e.global(KILOBYTE);
    let percent = ((value as f64 * hundred) / budget_bytes as f64) as f32;
    budget_bytes = (budget_bytes as f64 / megabyte) as f32;
    let (format, shown) = if value as f64 > megabyte {
        (FORMAT_MEGABYTES, (value as f64 / megabyte) as f32)
    } else if value as f64 > kilobyte {
        (FORMAT_KILOBYTES, (value as f64 / kilobyte) as f32)
    } else {
        (FORMAT_BYTES, value)
    };
    e.call(
        STRING_FORMAT,
        &args![
            text,
            format,
            percent as f64,
            shown as f64,
            budget_bytes as f64
        ],
    );
}

// Translated from 004a9240 (decompiled, FalloutNV.exe 1.4.0.525)
/// A size score of a model object, cdecl: 0 for null; with the kind
/// `k = 0043b230(object)` (the pointer at +0xc): 1 for kind 3, the sum of
/// this function over the children for kinds 9, 10, 0x16 and 0x17 (the
/// children come from virtual slots `0x10` (container), `0x08` (first
/// position), `0x14` (child at position, 0x200-byte scratch buffer) and
/// `0x0c` (next position), until the position is -1 or a child is null),
/// and 10 for every other kind. The four container cases of the original
/// are the same code with separate stack buffers.
pub fn fn_004a9240(e: &mut Engine, object: Ptr) -> u32 {
    if object.is_null() {
        return 0;
    }
    let kind = e.call(0x0043_b230, &args![object]).u32();
    let index = kind.wrapping_sub(3);
    match index {
        0 => 1,
        6 | 7 | 19 | 20 => {
            let container = e.vcall(object.addr(), 0x10, &[]).u32();
            let mut total = 0u32;
            e.with_stack(0x200, |e, buffer| {
                let mut position = e.vcall(container, 0x08, &[]).u32() as i32;
                while position != -1 {
                    let child = e.vcall(container, 0x14, &args![position, buffer]).u32();
                    if child == 0 {
                        break;
                    }
                    total = total.wrapping_add(fn_004a9240(e, Ptr::new(child)));
                    position = e.vcall(container, 0x0c, &args![position]).u32() as i32;
                }
            });
            total
        }
        _ => 0x0a,
    }
}

// ---------------------------------------------------------------------
// Containers
// ---------------------------------------------------------------------

// Translated from 004a95b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of element `index` of the scene-data array (`006b30e0`:
/// `pBuffer + index * 0x40`).
pub fn fn_004a95b0(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32) -> Ptr {
    e.call(ARRAY_ELEMENT_ADDRESS, &args![this, index]).ptr()
}

// Translated from 004a95d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiTMap<NiTexture *, TEX_USER_DATA *>`: the base
/// constructor [`fn_004a97e0`], then the vtable `0101ecec`. Returns `this`.
pub fn fn_004a95d0(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    fn_004a97e0(e, this, buckets);
    e.mem.set_u32(this.addr(), TEXTURE_USER_DATA_MAP_VTABLE);
    this
}

// Translated from 004a9600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiTMap<TESObjectCELL *, bool>`: the base constructor
/// [`fn_004a98e0`], then the vtable `0101ed0c`. Returns `this`.
pub fn fn_004a9600(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    fn_004a98e0(e, this, buckets);
    e.mem.set_u32(this.addr(), CELL_FLAG_MAP_VTABLE);
    this
}

// Translated from 004a9630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<NiTexture *, TEX_USER_DATA *>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor body `004a9850` and frees the block when
/// bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_ni_texture_p_tex_user_data_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    e.call(TEXTURE_USER_DATA_MAP_DESTROY, &args![this]);
    delete_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 004a9660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObjectCELL *, bool>::_scalar_deleting_destructor_` (Xbox
/// PDB): runs the destructor body [`fn_004a9970`] and frees the block when
/// bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_tes_object_cell_p_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_004a9970(e, this);
    delete_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 004a9690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts the 0x40-byte scene-data entry `element` at `index` of the
/// scene-data array (`BSSimpleArray<SCENE_INFO_DATA_STRUCT, 1024>`), moving
/// the entries from `index` on up by one. At `index == iSize` it is an
/// append ([`fn_004a9a60`]). When the array is full it grows to the size
/// `009a3910` gives (virtual allocate, slot `0x04`), copies around the hole
/// into the new buffer and releases the old one (`006a8500`).
pub fn fn_004a9690(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, element: Ptr) {
    let size = e.get(this, BSSimpleArray::iSize);
    if index == size {
        fn_004a9a60(e, this, element);
        return;
    }
    if e.call(ARRAY_IS_FULL, &args![this]).bool() {
        let capacity = e.call(ARRAY_NEXT_CAPACITY, &args![this]).u32();
        let new_buffer = e
            .vcall(this.addr(), SLOT_ARRAY_ALLOCATE, &args![capacity])
            .u32();
        let old_buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_004a9b50(e, this, new_buffer, old_buffer, index);
        fn_004a9ab0(e, this, element_address(new_buffer, index), 1);
        let size = e.get(this, BSSimpleArray::iSize);
        let old_buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_004a9b50(
            e,
            this,
            element_address(new_buffer, index + 1),
            element_address(old_buffer, index),
            size.wrapping_sub(index),
        );
        e.call(ARRAY_FREE_BUFFER, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, new_buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    } else {
        let size = e.get(this, BSSimpleArray::iSize);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_004a9b50(
            e,
            this,
            element_address(buffer, index + 1),
            element_address(buffer, index),
            size.wrapping_sub(index),
        );
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_004a9ab0(e, this, element_address(buffer, index), 1);
    }
    let size = e.get(this, BSSimpleArray::iSize).wrapping_add(1);
    e.set(this, BSSimpleArray::iSize, size);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let bytes = e.mem.bytes(element.addr(), SCENE_DATA_SIZE);
    e.mem.write(element_address(buffer, index), &bytes);
}

// Translated from 004a97e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., NiTexture *, TEX_USER_DATA *>` constructor with
/// `buckets` buckets: vtable `0101ed2c`, the bucket array (`buckets * 4`
/// bytes from `00aa1070`) cleared with `memset`, count 0. Returns `this`.
pub fn fn_004a97e0(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    construct_map_base(e, this, TEXTURE_USER_DATA_MAP_BASE_VTABLE, buckets)
}

// Translated from 004a98e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., TESObjectCELL *, bool>` constructor with `buckets`
/// buckets: the same body as [`fn_004a97e0`] with the vtable `0101ed4c`.
/// Returns `this`.
pub fn fn_004a98e0(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    construct_map_base(e, this, CELL_FLAG_MAP_BASE_VTABLE, buckets)
}

// Translated from 004a9970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `NiTMap<TESObjectCELL *, bool>` (fastcall `this`):
/// vtable `0101ed0c`, `00438af0` (empties the map), then the base
/// destructor body `004a99d0`. (The compiler's exception frame is not
/// translated.)
pub fn fn_004a9970(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), CELL_FLAG_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(CELL_FLAG_MAP_BASE_DESTROY, &args![this]);
}

// Translated from 004a9a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<NiTexture *, TEX_USER_DATA *> >, NiTexture
/// *, TEX_USER_DATA *>::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// base destructor body `004a98b0` and frees the block when bit 0 of
/// `flags` is set. Returns `this`.
pub fn ni_t_map_base_ni_texture_p_tex_user_data_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    e.call(TEXTURE_USER_DATA_MAP_BASE_DESTROY, &args![this]);
    delete_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 004a9a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<TESObjectCELL *, bool> >, TESObjectCELL *,
/// bool>::_scalar_deleting_destructor_` (Xbox PDB): runs the base
/// destructor body `004a99d0` and frees the block when bit 0 of `flags` is
/// set. Returns `this`.
pub fn ni_t_map_base_tes_object_cell_p_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    e.call(CELL_FLAG_MAP_BASE_DESTROY, &args![this]);
    delete_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 004a9a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends the 0x40-byte scene-data entry `element` to the array: takes the
/// new index from `006b3fd0` (which grows the array and bumps the size),
/// constructs an empty entry there ([`fn_004a9ab0`]) and copies `element`
/// over it. Returns the index.
pub fn fn_004a9a60(e: &mut Engine, this: Ptr<BSSimpleArray>, element: Ptr) -> u32 {
    let index = e.call(ARRAY_APPEND_SLOT, &args![this]).u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    fn_004a9ab0(e, this, element_address(buffer, index), 1);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let bytes = e.mem.bytes(element.addr(), SCENE_DATA_SIZE);
    e.mem.write(element_address(buffer, index), &bytes);
    index
}

// Translated from 004a9ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs `count` scene-data entries in place at `first`: for each,
/// the placement `new` (`006e6da0(0x40, address)`) and, when it returns a
/// non-null address, the clearing constructor `004a7c80` (`memset` to 0).
/// (The exception frame is not translated.)
pub fn fn_004a9ab0(e: &mut Engine, _this: Ptr<BSSimpleArray>, first: u32, count: u32) {
    for index in 0..count {
        let address = e
            .call(
                PLACEMENT_NEW,
                &args![SCENE_DATA_SIZE, element_address(first, index)],
            )
            .u32();
        if address != 0 {
            e.call(SCENE_DATA_CLEAR, &args![address]);
        }
    }
}

// Translated from 004a9b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `memmove` of `count` 0x40-byte entries from `source` to `destination`,
/// one entry at a time: forward when the destination is below the source,
/// backward when it is above, nothing when they are equal.
pub fn fn_004a9b50(
    e: &mut Engine,
    _this: Ptr<BSSimpleArray>,
    destination: u32,
    source: u32,
    count: u32,
) {
    if count == 0 {
        return;
    }
    if destination < source {
        for index in 0..count {
            e.call(
                MEMMOVE,
                &args![
                    element_address(destination, index),
                    element_address(source, index),
                    SCENE_DATA_SIZE
                ],
            );
        }
    } else if source < destination {
        for index in (0..count).rev() {
            e.call(
                MEMMOVE,
                &args![
                    element_address(destination, index),
                    element_address(source, index),
                    SCENE_DATA_SIZE
                ],
            );
        }
    }
}

// Translated from 004a9bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiTMap<NiTexture *, bool>`: the base constructor
/// [`fn_004a9c50`], then the vtable `0101ed6c`. Returns `this`.
pub fn fn_004a9bf0(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    fn_004a9c50(e, this, buckets);
    e.mem.set_u32(this.addr(), TEXTURE_FLAG_MAP_VTABLE);
    this
}

// Translated from 004a9c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<NiTexture *, bool>::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor body [`fn_004a9cc0`] and frees the block when bit 0
/// of `flags` is set. Returns `this`.
pub fn ni_t_map_ni_texture_p_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_004a9cc0(e, this);
    delete_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 004a9c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., NiTexture *, bool>` constructor with `buckets` buckets:
/// the same body as [`fn_004a97e0`] with the vtable `0101ed8c`. Returns
/// `this`.
pub fn fn_004a9c50(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    construct_map_base(e, this, TEXTURE_FLAG_MAP_BASE_VTABLE, buckets)
}

// Translated from 004a9cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `NiTMap<NiTexture *, bool>` (fastcall `this`): vtable
/// `0101ed6c`, `00438af0` (empties the map), then the base destructor body
/// [`fn_004a9d20`]. (The compiler's exception frame is not translated.)
pub fn fn_004a9cc0(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), TEXTURE_FLAG_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_004a9d20(e, this);
}

// Translated from 004a9d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., NiTexture *, bool>` destructor body (fastcall `this`):
/// vtable `0101ed8c`, `00438af0` (empties the map), then frees the bucket
/// array at +8 (`00aa10f0`).
pub fn fn_004a9d20(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), TEXTURE_FLAG_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTMap::m_ppkHashTable);
    e.call(DEALLOCATE, &args![table]);
}

// Translated from 004a9d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<NiTexture *, bool> >, NiTexture *,
/// bool>::_scalar_deleting_destructor_` (Xbox PDB): runs the base
/// destructor body [`fn_004a9d20`] and frees the block when bit 0 of
/// `flags` is set. Returns `this`.
pub fn ni_t_map_base_ni_texture_p_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_004a9d20(e, this);
    delete_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 004a9d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the scene-data array (`BSSimpleArray<SCENE_INFO_DATA_STRUCT,
/// 1024>`): vtable `0101edac`, then `004a9e00(this, 0, 0)` (empty, nothing
/// reserved). Returns `this`.
pub fn fn_004a9d80(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), SCENE_DATA_ARRAY_VTABLE);
    e.call(SCENE_DATA_ARRAY_CONSTRUCT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 004a9db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<SCENE_INFO_DATA_STRUCT, 1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor body [`fn_004a9de0`] and frees the block
/// when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_scene_info_data_struct_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_004a9de0(e, this);
    delete_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 004a9de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the scene-data array (fastcall `this`): vtable
/// `0101edac`, then `008454f0(this, 1)` (releases the entries and the
/// buffer).
pub fn fn_004a9de0(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), SCENE_DATA_ARRAY_VTABLE);
    e.call(ARRAY_REMOVE_ALL, &args![this, 1u32]);
}

// Translated from 004a9e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor body of the scene-data array (`BSSimpleArray<SCENE_INFO_DATA_STRUCT,
/// 1024>`, thiscall, two stack words): empties the array, reserves
/// `max(reserve, size)` entries through the virtual allocate (slot 4) when
/// that is not zero, and when `size` is not zero constructs `size` empty
/// entries ([`fn_004a9ab0`]) and sets the count to `size`.
pub fn fn_004a9e00(e: &mut Engine, this: Ptr<BSSimpleArray>, reserve: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let reserve = reserve.max(size);
    if reserve != 0 {
        let buffer = e
            .vcall(this.addr(), SLOT_ARRAY_ALLOCATE, &args![reserve])
            .u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserve);
    }
    if size != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_004a9ab0(e, this, buffer, size);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 004a9e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiTMap<TESActorBase *, bool>`: the base constructor
/// [`fn_004a9ef0`], then the vtable `0101edc0`. Returns `this`.
pub fn fn_004a9e90(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    fn_004a9ef0(e, this, buckets);
    e.mem.set_u32(this.addr(), ACTOR_FLAG_MAP_VTABLE);
    this
}

// Translated from 004a9ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESActorBase *, bool>::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor body [`fn_004a9f60`] and frees the block when bit 0
/// of `flags` is set. Returns `this`.
pub fn ni_t_map_tes_actor_base_p_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_004a9f60(e, this);
    delete_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 004a9ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., TESActorBase *, bool>` constructor with `buckets`
/// buckets: the same body as [`fn_004a97e0`] with the vtable `0101ede0`.
/// Returns `this`.
pub fn fn_004a9ef0(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    construct_map_base(e, this, ACTOR_FLAG_MAP_BASE_VTABLE, buckets)
}

// Translated from 004a9f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `NiTMap<TESActorBase *, bool>` (fastcall `this`):
/// vtable `0101edc0`, `00438af0` (empties the map), then the base
/// destructor body [`fn_004a9fc0`]. (The compiler's exception frame is
/// not translated.)
pub fn fn_004a9f60(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), ACTOR_FLAG_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_004a9fc0(e, this);
}

// Translated from 004a9fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., TESActorBase *, bool>` destructor body (fastcall
/// `this`): vtable `0101ede0`, `00438af0` (empties the map), then frees the
/// bucket array at +8 (`00aa10f0`).
pub fn fn_004a9fc0(e: &mut Engine, this: Ptr<NiTMap>) {
    destroy_map_base(e, this, ACTOR_FLAG_MAP_BASE_VTABLE);
}

// Translated from 004a9ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<TESActorBase *, bool> >, TESActorBase *, bool>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the base destructor body [`fn_004a9fc0`] and frees
/// the block when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_base_tes_actor_base_p_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_004a9fc0(e, this);
    delete_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 004aa020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiTMap<int, int>`: the base constructor
/// [`fn_004aa080`], then the vtable `0101ee00`. Returns `this`.
pub fn fn_004aa020(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    fn_004aa080(e, this, buckets);
    e.mem.set_u32(this.addr(), INT_MAP_VTABLE);
    this
}

// Translated from 004aa050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<int, int>::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor body [`fn_004aa0f0`] and frees the block when bit 0
/// of `flags` is set. Returns `this`.
pub fn ni_t_map_int_int_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_004aa0f0(e, this);
    delete_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 004aa080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., int, int>` constructor with `buckets`
/// buckets: the same body as [`fn_004a97e0`] with the vtable `0101ee20`.
/// Returns `this`.
pub fn fn_004aa080(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    construct_map_base(e, this, INT_MAP_BASE_VTABLE, buckets)
}

// Translated from 004aa0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `NiTMap<int, int>` (fastcall `this`):
/// vtable `0101ee00`, `00438af0` (empties the map), then the base
/// destructor body [`fn_004aa150`]. (The compiler's exception frame is
/// not translated.)
pub fn fn_004aa0f0(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), INT_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_004aa150(e, this);
}

// Translated from 004aa150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., int, int>` destructor body (fastcall
/// `this`): vtable `0101ee20`, `00438af0` (empties the map), then frees the
/// bucket array at +8 (`00aa10f0`).
pub fn fn_004aa150(e: &mut Engine, this: Ptr<NiTMap>) {
    destroy_map_base(e, this, INT_MAP_BASE_VTABLE);
}

// Translated from 004aa180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<int, int> >, int, int>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the base destructor body [`fn_004aa150`] and frees
/// the block when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_base_int_int_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_004aa150(e, this);
    delete_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 004aa1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiTMap<ShadowSceneLight *, int>`: the base constructor
/// [`fn_004aa210`], then the vtable `0101ee40`. Returns `this`.
pub fn fn_004aa1b0(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    fn_004aa210(e, this, buckets);
    e.mem.set_u32(this.addr(), LIGHT_INT_MAP_VTABLE);
    this
}

// Translated from 004aa1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<ShadowSceneLight *, int>::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor body [`fn_004aa280`] and frees the block when bit 0
/// of `flags` is set. Returns `this`.
pub fn ni_t_map_shadow_scene_light_p_int_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_004aa280(e, this);
    delete_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 004aa210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., ShadowSceneLight *, int>` constructor with `buckets`
/// buckets: the same body as [`fn_004a97e0`] with the vtable `0101ee60`.
/// Returns `this`.
pub fn fn_004aa210(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    construct_map_base(e, this, LIGHT_INT_MAP_BASE_VTABLE, buckets)
}

// Translated from 004aa280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `NiTMap<ShadowSceneLight *, int>` (fastcall `this`):
/// vtable `0101ee40`, `00438af0` (empties the map), then the base
/// destructor body [`fn_004aa2e0`]. (The compiler's exception frame is
/// not translated.)
pub fn fn_004aa280(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), LIGHT_INT_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_004aa2e0(e, this);
}

// Translated from 004aa2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., ShadowSceneLight *, int>` destructor body (fastcall
/// `this`): vtable `0101ee60`, `00438af0` (empties the map), then frees the
/// bucket array at +8 (`00aa10f0`).
pub fn fn_004aa2e0(e: &mut Engine, this: Ptr<NiTMap>) {
    destroy_map_base(e, this, LIGHT_INT_MAP_BASE_VTABLE);
}

// Translated from 004aa310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<ShadowSceneLight *, int> >, ShadowSceneLight *, int>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the base destructor body [`fn_004aa2e0`] and frees
/// the block when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_base_shadow_scene_light_p_int_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_004aa2e0(e, this);
    delete_if_flagged(e, this.cast(), flags);
    this
}

// Translated from 004bc320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRenderedTexture::GetTexture` (Xbox PDB): the texture pointer stored at
/// `this + 0x30 + index * 4` (loaded through `00559450`). With a null
/// `this` the slot is a temporary smart pointer built by `00633c90(0)`
/// (so the result is null), destroyed by `0045cec0` afterwards.
pub fn bs_rendered_texture_get_texture(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    if !this.is_null() {
        let slot = this.addr().wrapping_add(0x30).wrapping_add(index << 2);
        return pointer_in_slot(e, Ptr::new(slot));
    }
    let mut result = 0;
    e.with_stack(4, |e, temporary| {
        let slot = e.call(0x0063_3c90, &args![temporary, 0u32]).u32();
        result = pointer_in_slot(e, Ptr::new(slot));
        e.call(0x0045_cec0, &args![temporary]);
    });
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004a7ca0,
            bgs_scene_info_scalar_deleting_destructor(Ptr<BGSSceneInfo>, u32) -> Ptr<BGSSceneInfo>
        ),
        entry!(0x004a7cd0, fn_004a7cd0(Ptr<BGSSceneInfo>)),
        entry!(0x004a7cf0, fn_004a7cf0(Ptr<BGSSceneInfo>, i32) -> u32),
        entry!(0x004a7d20, fn_004a7d20(Ptr<BGSSceneInfo>, u32)),
        entry!(0x004a8030, fn_004a8030(Ptr) -> u32),
        entry!(0x004a8050, fn_004a8050(Ptr) -> u32),
        entry!(0x004a8070, fn_004a8070(Ptr) -> u32),
        entry!(0x004a8090, fn_004a8090(Ptr) -> u32),
        entry!(
            0x004a80c0,
            bgs_scene_info_collect_scene_data_from_addon_nodes(Ptr<BGSSceneInfo>)
        ),
        entry!(
            0x004a8230,
            bgs_scene_info_collect_scene_data_from_node(Ptr<BGSSceneInfo>, Ptr, bool, bool)
        ),
        entry!(0x004a82c0, fn_004a82c0() -> bool),
        entry!(
            0x004a82d0,
            bgs_scene_info_collect_scene_data_recursive(
                Ptr<BGSSceneInfo>,
                Ptr,
                Ptr<SceneInfoDataStruct>,
                bool,
                Ptr,
            )
        ),
        entry!(0x004a8ab0, fn_004a8ab0(Ptr) -> u16),
        entry!(0x004a9050, fn_004a9050() -> i32),
        entry!(0x004a9100, fn_004a9100() -> u32),
        entry!(
            0x004a9120,
            bgs_scene_info_build_m_number_string(f32, Ptr, bool)
        ),
        entry!(0x004a9240, fn_004a9240(Ptr) -> u32),
        entry!(0x004a95b0, fn_004a95b0(Ptr<BSSimpleArray>, u32) -> Ptr),
        entry!(0x004a95d0, fn_004a95d0(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x004a9600, fn_004a9600(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(
            0x004a9630,
            ni_t_map_ni_texture_p_tex_user_data_p_scalar_deleting_destructor(
                Ptr<NiTMap>,
                u32,
            )
                -> Ptr<NiTMap>
        ),
        entry!(
            0x004a9660,
            ni_t_map_tes_object_cell_p_bool_scalar_deleting_destructor(
                Ptr<NiTMap>,
                u32,
            ) -> Ptr<NiTMap>
        ),
        entry!(0x004a9690, fn_004a9690(Ptr<BSSimpleArray>, u32, Ptr)),
        entry!(0x004a97e0, fn_004a97e0(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x004a98e0, fn_004a98e0(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x004a9970, fn_004a9970(Ptr<NiTMap>)),
        entry!(
            0x004a9a00,
            ni_t_map_base_ni_texture_p_tex_user_data_p_scalar_deleting_destructor(
                Ptr<NiTMap>,
                u32,
            )
                -> Ptr<NiTMap>
        ),
        entry!(
            0x004a9a30,
            ni_t_map_base_tes_object_cell_p_bool_scalar_deleting_destructor(
                Ptr<NiTMap>,
                u32,
            )
                -> Ptr<NiTMap>
        ),
        entry!(0x004a9a60, fn_004a9a60(Ptr<BSSimpleArray>, Ptr) -> u32),
        entry!(0x004a9ab0, fn_004a9ab0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x004a9b50, fn_004a9b50(Ptr<BSSimpleArray>, u32, u32, u32)),
        entry!(0x004a9bf0, fn_004a9bf0(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(
            0x004a9c20,
            ni_t_map_ni_texture_p_bool_scalar_deleting_destructor(Ptr<NiTMap>, u32) -> Ptr<NiTMap>
        ),
        entry!(0x004a9c50, fn_004a9c50(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x004a9cc0, fn_004a9cc0(Ptr<NiTMap>)),
        entry!(0x004a9d20, fn_004a9d20(Ptr<NiTMap>)),
        entry!(
            0x004a9d50,
            ni_t_map_base_ni_texture_p_bool_scalar_deleting_destructor(
                Ptr<NiTMap>,
                u32,
            ) -> Ptr<NiTMap>
        ),
        entry!(
            0x004a9d80,
            fn_004a9d80(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x004a9db0,
            bs_simple_array_scene_info_data_struct_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(0x004a9de0, fn_004a9de0(Ptr<BSSimpleArray>)),
        entry!(0x004a9e00, fn_004a9e00(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x004a9e90, fn_004a9e90(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(
            0x004a9ec0,
            ni_t_map_tes_actor_base_p_bool_scalar_deleting_destructor(
                Ptr<NiTMap>,
                u32,
            ) -> Ptr<NiTMap>
        ),
        entry!(0x004a9ef0, fn_004a9ef0(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x004a9f60, fn_004a9f60(Ptr<NiTMap>)),
        entry!(0x004a9fc0, fn_004a9fc0(Ptr<NiTMap>)),
        entry!(
            0x004a9ff0,
            ni_t_map_base_tes_actor_base_p_bool_scalar_deleting_destructor(
                Ptr<NiTMap>,
                u32,
            )
                -> Ptr<NiTMap>
        ),
        entry!(0x004aa020, fn_004aa020(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(
            0x004aa050,
            ni_t_map_int_int_scalar_deleting_destructor(Ptr<NiTMap>, u32) -> Ptr<NiTMap>
        ),
        entry!(0x004aa080, fn_004aa080(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x004aa0f0, fn_004aa0f0(Ptr<NiTMap>)),
        entry!(0x004aa150, fn_004aa150(Ptr<NiTMap>)),
        entry!(
            0x004aa180,
            ni_t_map_base_int_int_scalar_deleting_destructor(Ptr<NiTMap>, u32) -> Ptr<NiTMap>
        ),
        entry!(0x004aa1b0, fn_004aa1b0(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(
            0x004aa1e0,
            ni_t_map_shadow_scene_light_p_int_scalar_deleting_destructor(
                Ptr<NiTMap>,
                u32,
            )
                -> Ptr<NiTMap>
        ),
        entry!(0x004aa210, fn_004aa210(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x004aa280, fn_004aa280(Ptr<NiTMap>)),
        entry!(0x004aa2e0, fn_004aa2e0(Ptr<NiTMap>)),
        entry!(
            0x004aa310,
            ni_t_map_base_shadow_scene_light_p_int_scalar_deleting_destructor(
                Ptr<NiTMap>,
                u32,
            )
                -> Ptr<NiTMap>
        ),
        entry!(0x004bc320, bs_rendered_texture_get_texture(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Doubles returning 0 for callees a test does not care about.
    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// An engine with the pages that hold the unit's globals and constants
    /// mapped, and doubles for the callees that are plain memory reads,
    /// fills or list steps (their bodies are one or two instructions).
    fn scene_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_6000,
            0x0101_7000,
            0x0101_e000,
            0x0118_8000,
            0x011c_3000,
            0x011c_5000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(HUNDRED, 100.0f64);
        e.set_global(MEGABYTE, 1048576.0f64);
        e.set_global(KILOBYTE, 1024.0f64);
        e.mem.set_cstr(MESHES_PREFIX, b"Meshes\\");
        e.mem.set_cstr(FURNITURE_MARKER, b"FurnitureMarker");
        e.register(NI_POINTER_GET, |e, a| eax(e.mem.u32(a[0])));
        e.register(LIST_NEXT_NODE, |e, a| {
            eax(if a[1] != 0 { e.mem.u32(a[1]) } else { 0 })
        });
        e.register(LIST_ELEMENT_SLOT, |_, a| eax(a[1] + 8));
        e.register(SIMPLE_LIST_NODE, |_, a| eax(a[0]));
        e.register(SIMPLE_LIST_NEXT, |e, a| eax(e.mem.u32(a[0] + 4)));
        e.register(SETTING_VALUE_ADDRESS, |_, a| eax(a[0] + 4));
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            eax(a[0])
        });
        e.register(MEMMOVE, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            eax(a[0])
        });
        e.register(PLACEMENT_NEW, |_, a| eax(a[1]));
        e.register(SCENE_DATA_CLEAR, |e, a| {
            e.mem.write(a[0], &[0u8; 0x40]);
            eax(a[0])
        });
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e
    }

    /// An object whose vtable slots return constants: `(slot offset,
    /// value)`.
    fn object_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x400);
        e.mem.set_u32(object, vtable);
        for &(slot, value) in slots {
            let target = (vtable + slot) ^ 0x8000_0000;
            e.register_double(target, move |_, _| eax(value));
            e.mem.set_u32(vtable + slot, target);
        }
        object
    }

    /// A geometry-data object: the pointer at +0xb8 leads to an object whose
    /// vertex-count slot (0x90) returns `vertices`.
    fn geometry_data_with(e: &mut Engine, vertices: u32) -> u32 {
        let inner = object_with(e, &[(SLOT_VERTEX_COUNT, vertices)]);
        let data = e.mem.alloc(0x100);
        e.mem.set_u32(data + 0xb8, inner);
        data
    }

    /// Doubles for the four flag getters of the memory estimate: 12 bytes
    /// per vertex (`0049ec60`), 16 (`0045cd60`), none (`0059bb30`), 8
    /// (`0055b980`): 36 + 48 + 24 = 108 for 3 vertices.
    fn estimate_doubles(e: &mut Engine) {
        e.register(0x0049_ec60, |_, _| eax(1));
        e.register(0x0045_cd60, |_, _| eax(1));
        e.register(0x0059_bb30, |_, _| eax(0));
        e.register(0x0055_b980, |_, _| eax(1));
    }

    /// A `NiTList` whose head word is at `at` (nodes: next, previous,
    /// element).
    fn ni_list_at(e: &mut Engine, at: u32, items: &[u32]) {
        let mut next = 0;
        for &item in items.iter().rev() {
            let node = e.mem.alloc(0x0c);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 8, item);
            next = node;
        }
        e.mem.set_u32(at, next);
    }

    /// A `BSSimpleList` (nodes: item, next); returns the first node.
    fn simple_list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for &item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    type MapTable = Rc<RefCell<HashMap<(u32, u32), u32>>>;

    /// `GetAt`, `SetAt` and `RemoveAll` over one table keyed by (map, key).
    fn map_doubles(e: &mut Engine) -> MapTable {
        let table: MapTable = Rc::new(RefCell::new(HashMap::new()));
        let get = Rc::clone(&table);
        e.register_double(MAP_GET_AT, move |e, a| {
            let found = get.borrow().get(&(a[0], a[1])).copied();
            match found {
                Some(value) => {
                    e.mem.set_u32(a[2], value);
                    eax(1)
                }
                None => eax(0),
            }
        });
        let set = Rc::clone(&table);
        e.register_double(MAP_SET_AT, move |_, a| {
            set.borrow_mut().insert((a[0], a[1]), a[2]);
            Ret::default()
        });
        let clear = Rc::clone(&table);
        e.register_double(MAP_REMOVE_ALL, move |_, a| {
            clear.borrow_mut().retain(|(map, _), _| *map != a[0]);
            Ret::default()
        });
        table
    }

    /// The scene-data array's append and element address, over a real
    /// buffer.
    fn array_doubles(e: &mut Engine) {
        e.register(ARRAY_APPEND_SLOT, |e, a| {
            let this = a[0];
            if e.mem.u32(this + 4) == 0 {
                let buffer = e.mem.alloc(0x40 * 8);
                e.mem.set_u32(this + 4, buffer);
                e.mem.set_u32(this + 0xc, 8);
            }
            let index = e.mem.u32(this + 8);
            e.mem.set_u32(this + 8, index + 1);
            eax(index)
        });
        e.register(ARRAY_ELEMENT_ADDRESS, |e, a| {
            eax(e.mem.u32(a[0] + 4) + a[1] * 0x40)
        });
    }

    /// An array object (`BSSimpleArray`, 0x10 bytes) with `buffer` holding
    /// `size` entries filled with their index + 1 and room for `reserved`.
    fn array_with(e: &mut Engine, size: u32, reserved: u32) -> (Ptr<BSSimpleArray>, u32) {
        let array: Ptr<BSSimpleArray> = e.new_object();
        let buffer = e.mem.alloc(0x40 * reserved.max(1));
        for index in 0..size {
            e.mem.write(buffer + index * 0x40, &[index as u8 + 1; 0x40]);
        }
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iSize, size);
        e.set(array, BSSimpleArray::iReservedSize, reserved);
        (array, buffer)
    }

    fn entry_bytes(e: &Engine, buffer: u32, count: u32) -> Vec<u8> {
        (0..count).map(|i| e.mem.u8(buffer + i * 0x40)).collect()
    }

    // ---- BGSSceneInfo ---------------------------------------------------

    #[test]
    fn scalar_deleting_destructor_sets_the_vtable_and_frees_only_with_bit_zero() {
        let mut e = scene_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004a_7ca0, &args![info, 0u32]).ptr::<()>(),
            info.cast()
        );
        assert_eq!(e.mem.u32(info.addr()), BGS_SCENE_INFO_VTABLE);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.call(0x004a_7ca0, &args![info, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![info.addr()]]);
    }

    #[test]
    fn destructor_body_stores_the_vtable() {
        let mut e = scene_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        e.call(0x004a_7cd0, &args![info]);
        assert_eq!(e.mem.u32(info.addr()), 0x0101_ec68);
    }

    #[test]
    fn counter_accessor_returns_zero_outside_zero_to_fifteen() {
        let mut e = scene_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        for index in 0..16u32 {
            e.mem.set_u32(info.addr() + 4 + index * 4, 100 + index);
        }
        assert_eq!(e.call(0x004a_7cf0, &args![info, 0i32]).u32(), 100);
        assert_eq!(e.call(0x004a_7cf0, &args![info, 15i32]).u32(), 115);
        assert_eq!(e.call(0x004a_7cf0, &args![info, -1i32]).u32(), 0);
        assert_eq!(e.call(0x004a_7cf0, &args![info, 16i32]).u32(), 0);
    }

    /// The world's cell list holds two entries whose owner objects list
    /// references of cell 5 (a, b) and 7 (c), a appearing in both owners;
    /// the cell's own list holds one form whose model has geometry data of
    /// 3 vertices.
    #[test]
    fn cell_counts_count_each_reference_of_the_cell_once_and_add_the_estimate() {
        let mut e = scene_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        let table = map_doubles(&mut e);
        stub(
            &mut e,
            &[
                REFERENCE_FLAG_MAP_CONSTRUCT,
                REFERENCE_FLAG_MAP_DESTROY,
                CELL_FLAG_MAP_BASE_DESTROY,
            ],
        );
        e.register(ALLOCATE, |e, a| eax(e.mem.alloc(a[0])));
        e.register(0x008d_6f30, |e, a| eax(e.mem.u32(a[0] + 0x40)));
        estimate_doubles(&mut e);

        e.mem.set_u32(TES_SINGLETON, 0x1111);
        let cell_list = e.mem.alloc(8);
        e.register(0x0070_ec90, |_, _| eax(0x2222));
        e.register_double(0x005a_8080, move |_, _| eax(cell_list));
        let reference = |e: &mut Engine, cell: u32| {
            let reference = e.mem.alloc(0x50);
            e.mem.set_u32(reference + 0x40, cell);
            reference
        };
        let a = reference(&mut e, 5);
        let b = reference(&mut e, 5);
        let c = reference(&mut e, 7);
        let owner_one = e.mem.alloc(0x40);
        let owner_two = e.mem.alloc(0x40);
        ni_list_at(&mut e, owner_one + 0x24, &[a, b, c]);
        ni_list_at(&mut e, owner_two + 0x24, &[a]);
        ni_list_at(&mut e, cell_list, &[owner_one, owner_two]);

        // The cell: its object list holds the form.
        let geometry_data = geometry_data_with(&mut e, 3);
        let model = object_with(&mut e, &[(SLOT_GEOMETRY_DATA, geometry_data)]);
        let owner_object = object_with(&mut e, &[(SLOT_AS_REFERENCE_NODE, 1)]);
        let form = object_with(&mut e, &[(SLOT_FORM_OWNER_OBJECT, owner_object)]);
        let nodes = simple_list(&mut e, &[form]);
        e.register_double(0x0054_5710, move |_, _| eax(nodes));
        e.register_double(0x0043_b4a0, move |_, a| {
            eax(if a[0] == owner_object { model } else { 0 })
        });

        e.call_log = Some(vec![]);
        e.call(0x004a_7d20, &args![info, 5u32]);
        assert_eq!(e.mem.u32(info.addr() + 0x24), 2, "a and b, a once");
        assert_eq!(e.mem.u32(info.addr() + 0x30), 108);
        // Cell map emptied after each of the two list entries and by its
        // destructor, reference map once at the end.
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL).len(), 4);
        assert!(table.borrow().is_empty());
    }

    #[test]
    fn cell_counts_ignore_references_of_other_cells() {
        let mut e = scene_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        map_doubles(&mut e);
        stub(
            &mut e,
            &[
                REFERENCE_FLAG_MAP_CONSTRUCT,
                REFERENCE_FLAG_MAP_DESTROY,
                CELL_FLAG_MAP_BASE_DESTROY,
            ],
        );
        e.register(ALLOCATE, |e, a| eax(e.mem.alloc(a[0])));
        e.register(0x008d_6f30, |e, a| eax(e.mem.u32(a[0] + 0x40)));
        e.mem.set_u32(TES_SINGLETON, 0x1111);
        let cell_list = e.mem.alloc(8);
        e.register(0x0070_ec90, |_, _| eax(0x2222));
        e.register_double(0x005a_8080, move |_, _| eax(cell_list));
        let reference = e.mem.alloc(0x50);
        e.mem.set_u32(reference + 0x40, 7);
        let owner = e.mem.alloc(0x40);
        ni_list_at(&mut e, owner + 0x24, &[reference]);
        ni_list_at(&mut e, cell_list, &[owner]);
        e.register(0x0054_5710, |_, _| eax(0));
        e.call(0x004a_7d20, &args![info, 5u32]);
        assert_eq!(e.mem.u32(info.addr() + 0x24), 0);
        assert_eq!(e.mem.u32(info.addr() + 0x30), 0);
    }

    #[test]
    fn geometry_data_getters_go_through_the_pointer_at_0xb8() {
        let mut e = scene_engine();
        let data = Ptr::<()>::new(geometry_data_with(&mut e, 9));
        let inner = e.mem.u32(data.addr() + 0xb8);
        e.register(0x0059_bb30, |_, a| eax(a[0]));
        e.register(0x0045_cd60, |_, a| eax(a[0] + 1));
        e.register(0x0055_b980, |_, a| eax(a[0] + 2));
        assert_eq!(e.call(0x004a_8030, &args![data]).u32(), inner);
        assert_eq!(e.call(0x004a_8050, &args![data]).u32(), inner + 1);
        assert_eq!(e.call(0x004a_8070, &args![data]).u32(), inner + 2);
        assert_eq!(e.call(0x004a_8090, &args![data]).u32(), 9);
    }

    #[test]
    fn addon_nodes_are_loaded_collected_and_released() {
        let mut e = scene_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        array_doubles(&mut e);
        e.mem.set_u32(DATA_HANDLER, 0x5000);
        e.mem.set_u32(MODEL_LOADER, 0x6000);
        e.mem.set_u32(TES_SINGLETON, 0x1234);
        e.register(0x0045_a120, |_, _| eax(2));
        // Addon 0 does not exist; addon 1 has a file name.
        let addon = e.mem.alloc(0x100);
        let name_holder = addon + 0x30;
        let vtable = e.mem.alloc(0x40);
        let file_name = e.mem.alloc(16);
        e.mem.set_cstr(file_name, b"foo.nif");
        e.register_double(0x7fff_0014, move |_, _| eax(file_name));
        e.mem.set_u32(vtable + 0x14, 0x7fff_0014);
        e.mem.set_u32(name_holder, vtable);
        e.register_double(0x0046_17e0, move |_, a| {
            eax(if a[1] == 1 { addon } else { 0 })
        });
        e.register_double(0x0048_cee0, move |_, a| eax(u32::from(a[0] == name_holder)));
        e.register(STRING_COPY, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRING_APPEND, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        let loaded = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&loaded);
        e.register_double(0x0044_7080, move |e, a| {
            record
                .borrow_mut()
                .push((a[0], e.mem.cstr(a[1]), a[2..].to_vec()));
            eax(0x7777)
        });
        e.register(0x00c5_11f0, |_, _| eax(0));
        let root = object_with(&mut e, &[(SLOT_AS_NODE, 0), (SLOT_AS_GEOMETRY, 0)]);
        e.register_double(0x0065_3270, move |_, a| {
            eax(if a[0] == TYPE_ADDON_MODEL_ROOT {
                root
            } else {
                0
            })
        });
        stub(&mut e, &[0x0045_a5e0, 0x0045_a1c0]);
        e.call_log = Some(vec![]);

        e.call(0x004a_80c0, &args![info]);
        assert_eq!(e.get(SCENE_DATA_ADDON_NODES, BSSimpleArray::iSize), 2);
        let loads = loaded.borrow();
        assert_eq!(loads.len(), 1);
        assert_eq!(loads[0].0, 0x6000);
        assert_eq!(loads[0].1, b"Meshes\\foo.nif");
        assert_eq!(loads[0].2, vec![0, 1, 0, 0, 0]);
        let released = calls_to(&e, 0x0045_a5e0);
        assert_eq!(released.len(), 1);
        assert_eq!(released[0][0], 0x6000);
        assert_eq!(calls_to(&e, 0x0045_a1c0), vec![vec![0x1234]]);
    }

    #[test]
    fn addon_model_the_check_rejects_is_released_without_collecting() {
        let mut e = scene_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        array_doubles(&mut e);
        e.mem.set_u32(DATA_HANDLER, 0x5000);
        e.register(0x0045_a120, |_, _| eax(1));
        let addon = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x40);
        e.register_double(0x7fff_0014, |_, _| eax(0));
        e.mem.set_u32(vtable + 0x14, 0x7fff_0014);
        e.mem.set_u32(addon + 0x30, vtable);
        e.register_double(0x0046_17e0, move |_, _| eax(addon));
        e.register(0x0048_cee0, |_, _| eax(1));
        stub(
            &mut e,
            &[STRING_COPY, STRING_APPEND, 0x0045_a5e0, 0x0065_3270],
        );
        e.register(0x0044_7080, |_, _| eax(0x7777));
        e.register(0x00c5_11f0, |_, _| eax(1));
        e.call_log = Some(vec![]);
        e.call(0x004a_80c0, &args![info]);
        assert!(calls_to(&e, 0x0065_3270).is_empty());
        assert_eq!(calls_to(&e, 0x0045_a5e0).len(), 1);
    }

    #[test]
    fn collect_from_node_clears_the_maps_and_counters_and_the_tracker_when_asked() {
        let mut e = scene_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        stub(&mut e, &[MAP_REMOVE_ALL, 0x0045_a1c0]);
        e.mem.set_u32(TES_SINGLETON, 0x1234);
        for index in 0..16u32 {
            e.mem.set_u32(info.addr() + 4 + index * 4, 9);
        }
        e.mem.set_u32(info.addr() + 0x44, 0xabcd);
        e.mem.set_u8(CLEAR_TEXTURE_TRACKER, 1);
        e.call_log = Some(vec![]);
        e.call(0x004a_8230, &args![info, Ptr::<()>::NULL, false, false]);
        let cleared: Vec<u32> = calls_to(&e, MAP_REMOVE_ALL).iter().map(|w| w[0]).collect();
        assert_eq!(
            cleared,
            vec![ACTOR_MAP, LIGHT_GEOMETRY_MAP, TEXTURE_TRACKER]
        );
        for index in 0..16u32 {
            assert_eq!(e.mem.u32(info.addr() + 4 + index * 4), 0);
        }
        assert_eq!(e.mem.u32(info.addr() + 0x44), 0xabcd, "form untouched");
        assert_eq!(calls_to(&e, 0x0045_a1c0).len(), 1);

        e.mem.set_u8(CLEAR_TEXTURE_TRACKER, 0);
        e.call_log = Some(vec![]);
        e.call(0x004a_8230, &args![info, Ptr::<()>::NULL, false, false]);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL).len(), 2);
    }

    #[test]
    fn collect_from_node_gathers_the_addon_nodes_only_when_the_array_is_empty() {
        let mut e = scene_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        stub(&mut e, &[MAP_REMOVE_ALL, 0x0045_a1c0]);
        e.mem.set_u32(DATA_HANDLER, 0x5000);
        e.register(0x0045_a120, |_, _| eax(0));
        e.mem.set_u8(INCLUDE_ADDONS, 1);
        e.register(0x0044_ddc0, |_, _| eax(0));
        e.call_log = Some(vec![]);
        e.call(0x004a_8230, &args![info, Ptr::<()>::NULL, true, false]);
        assert_eq!(calls_to(&e, 0x0045_a120).len(), 1);

        // Without the flag argument, or with entries already, no addons.
        e.call_log = Some(vec![]);
        e.call(0x004a_8230, &args![info, Ptr::<()>::NULL, false, false]);
        e.register(0x0044_ddc0, |_, _| eax(3));
        e.call(0x004a_8230, &args![info, Ptr::<()>::NULL, true, false]);
        assert!(calls_to(&e, 0x0045_a120).is_empty());
    }

    #[test]
    fn texture_tracker_flag_is_the_static_byte() {
        let mut e = scene_engine();
        assert!(!e.call(0x004a_82c0, &args![]).bool());
        e.mem.set_u8(CLEAR_TEXTURE_TRACKER, 1);
        assert!(e.call(0x004a_82c0, &args![]).bool());
    }

    // ---- CollectSceneDataRecursive -------------------------------------

    /// Doubles returning 0 for every callee of the recursive collector.
    fn recursion_engine() -> Engine {
        let mut e = scene_engine();
        stub(
            &mut e,
            &[
                0x0041_3f40,
                0x0043_b1b0,
                STRING_COMPARE_NO_CASE,
                0x0043_b300,
                0x0065_3270,
                0x009e_e040,
                0x0045_a120,
                0x0044_ddc0,
                0x009a_d610,
                0x0044_4ed0,
                0x0056_5580,
                0x005f_65d0,
                0x0090_5820,
                0x005d_43c0,
                0x0041_f050,
                0x007a_f430,
                0x0084_e3a0,
                0x0040_1170,
                0x004a_8b00,
                0x006f_a820,
                0x0043_b560,
                0x0043_b230,
                0x0043_b480,
                0x0043_b4a0,
                0x0045_0b80,
                0x00b5_b4a0,
                0x00a5_9d30,
                0x0044_1110,
                0x004a_2020,
                0x00b7_0790,
                0x00b7_0590,
                0x00b7_0680,
                0x0085_3130,
                0x0084_4700,
                0x004a_1ff0,
                0x0054_95f0,
                0x004a_8a90,
                0x00a7_27b0,
                0x00a7_2550,
                0x004a_8ae0,
                0x0045_a1c0,
                0x0049_ec60,
                0x0059_bb30,
                0x0045_cd60,
                0x0055_b980,
            ],
        );
        e.mem.set_u32(TES_SINGLETON, 0x1234);
        e
    }

    /// Calls the collector with no explicit data and no parent.
    fn collect(e: &mut Engine, info: Ptr<BGSSceneInfo>, node: u32, descend: bool) {
        e.call(
            0x004a_82d0,
            &args![
                info,
                Ptr::<()>::new(node),
                Ptr::<()>::NULL,
                descend,
                Ptr::<()>::NULL
            ],
        );
    }

    #[test]
    fn recursive_collector_with_no_node_only_makes_the_final_call() {
        let mut e = recursion_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        e.call_log = Some(vec![]);
        collect(&mut e, info, 0, false);
        assert_eq!(calls_to(&e, 0x0045_a1c0), vec![vec![0x1234]]);
        assert_eq!(
            e.call_log.as_ref().unwrap().len(),
            2,
            "this call and the end call"
        );
    }

    #[test]
    fn a_node_named_furniture_marker_ends_the_walk_without_the_final_call() {
        let mut e = recursion_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        let as_node = object_with(&mut e, &[(SLOT_AS_REFERENCE_NODE, 1)]);
        let node = object_with(&mut e, &[(SLOT_AS_NODE, as_node)]);
        let name = e.mem.alloc(32);
        e.mem.set_cstr(name, b"furnituremarker_01");
        e.register_double(0x0043_b1b0, move |_, _| eax(name));
        e.register(STRING_COMPARE_NO_CASE, |e, a| {
            let left = e.mem.cstr(a[0]).to_ascii_lowercase();
            let right = e.mem.cstr(a[1]).to_ascii_lowercase();
            let n = a[2] as usize;
            eax(u32::from(
                left[..n.min(left.len())] != right[..n.min(right.len())],
            ))
        });
        e.register(0x009a_d610, |_, _| eax(0x4444));
        e.call_log = Some(vec![]);
        collect(&mut e, info, node, false);
        assert!(calls_to(&e, 0x0045_a1c0).is_empty());
        assert!(
            calls_to(&e, 0x009a_d610).is_empty(),
            "returned before the reference"
        );

        // A node with another name goes on to the reference.
        e.mem.set_cstr(name, b"Other");
        collect(&mut e, info, node, false);
        assert_eq!(calls_to(&e, 0x009a_d610).len(), 1);
    }

    /// A reference node with time controllers, two decals, a model and
    /// two children.
    #[test]
    fn a_reference_node_counts_itself_its_controllers_decals_model_and_children() {
        let mut e = recursion_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        let child_a = object_with(&mut e, &[(SLOT_AS_NODE, 0), (SLOT_AS_GEOMETRY, 0)]);
        let child_b = object_with(&mut e, &[(SLOT_AS_NODE, 0), (SLOT_AS_GEOMETRY, 0)]);
        let as_node = object_with(&mut e, &[(SLOT_AS_REFERENCE_NODE, 1)]);
        let node = object_with(&mut e, &[(SLOT_AS_NODE, as_node)]);
        e.register(0x009a_d610, |_, _| eax(0x4444));
        e.register(0x0056_5580, |_, _| eax(1));
        e.register(0x007a_f430, |_, _| eax(0x5555));
        e.register(0x0084_e3a0, |_, _| eax(0x1234_5678));
        e.register(0x0040_1170, |_, _| eax(0x10));
        let decals = simple_list(&mut e, &[1, 2]);
        e.register_double(0x0041_f050, move |_, _| eax(decals));
        // The model: kind 3 scores 1.
        let model_slot = e.mem.alloc(4);
        e.mem.set_u32(model_slot, 0x6666);
        e.register(0x004a_8b00, |_, _| eax(0x7000));
        e.register(0x006f_a820, |_, _| eax(0x7001));
        e.register_double(0x0043_b560, move |_, _| eax(model_slot));
        e.register(0x0043_b230, |_, _| eax(3));
        e.register(0x0043_b480, |_, _| eax(2));
        e.register_double(0x0043_b4a0, move |_, a| {
            eax(if a[1] == 0 { child_a } else { child_b })
        });
        // Form IDs seen so far: none, so the controllers are new.
        let inserted = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&inserted);
        e.register_double(0x0090_5820, move |e, a| {
            record.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        collect(&mut e, info, node, false);
        let data = |offset: u32| e.mem.u32(info.addr() + 4 + offset);
        assert_eq!(data(0x00), 1);
        assert_eq!(data(0x04), 0, "form type 0x10 is not a marker kind");
        assert_eq!(data(0x08), 1);
        assert_eq!(data(0x0c), 1, "form id was not in the list");
        assert_eq!(data(0x38), 2, "two decals");
        assert_eq!(data(0x3c), 1);
        assert_eq!(*inserted.borrow(), vec![(ANIMATED_3D_LIST, 0x1234_5678)]);
        // Two children plus this call each end with the TES call.
        assert_eq!(calls_to(&e, 0x0045_a1c0).len(), 3);
        let top_level = calls_to(&e, 0x004a_82d0);
        assert_eq!(top_level.len(), 1, "the children are direct calls");
    }

    #[test]
    fn known_animated_forms_are_not_counted_or_inserted_again() {
        let mut e = recursion_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        let as_node = object_with(&mut e, &[(SLOT_AS_REFERENCE_NODE, 1)]);
        let node = object_with(&mut e, &[(SLOT_AS_NODE, as_node)]);
        e.register(0x009a_d610, |_, _| eax(0x4444));
        e.register(0x0056_5580, |_, _| eax(1));
        e.register(0x005f_65d0, |_, _| eax(1));
        collect(&mut e, info, node, false);
        assert_eq!(e.mem.u32(info.addr() + 4 + 0x08), 1);
        assert_eq!(e.mem.u32(info.addr() + 4 + 0x0c), 0);
    }

    #[test]
    fn a_marker_form_type_stops_below_unless_told_to_descend() {
        for (descend, children_visited) in [(false, 0usize), (true, 1)] {
            let mut e = recursion_engine();
            let info: Ptr<BGSSceneInfo> = e.new_object();
            let child = object_with(&mut e, &[(SLOT_AS_NODE, 0), (SLOT_AS_GEOMETRY, 0)]);
            let as_node = object_with(&mut e, &[(SLOT_AS_REFERENCE_NODE, 1)]);
            let node = object_with(&mut e, &[(SLOT_AS_NODE, as_node)]);
            e.register(0x009a_d610, |_, _| eax(0x4444));
            e.register(0x0040_1170, |_, _| eax(0x2d));
            e.register(0x0043_b480, |_, _| eax(1));
            e.register_double(0x0043_b4a0, move |_, _| eax(child));
            e.call_log = Some(vec![]);
            collect(&mut e, info, node, descend);
            assert_eq!(e.mem.u32(info.addr() + 4 + 0x04), 1, "marker counter");
            assert_eq!(calls_to(&e, 0x0043_b4a0).len(), children_visited);
            // Stopping below returns without the TES call.
            assert_eq!(calls_to(&e, 0x0045_a1c0).len(), if descend { 2 } else { 0 });
        }
    }

    #[test]
    fn addon_scene_data_is_added_for_the_selected_entry() {
        let mut e = recursion_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        let as_node = object_with(&mut e, &[(SLOT_AS_REFERENCE_NODE, 0)]);
        let node = object_with(&mut e, &[(SLOT_AS_NODE, as_node)]);
        e.register(0x0043_b300, |_, _| eax(1));
        e.register(0x0065_3270, |_, _| eax(0x8000));
        e.register(0x009e_e040, |_, _| eax(1));
        e.register(0x0045_a120, |_, _| eax(2));
        e.register(0x0044_ddc0, |_, _| eax(2));
        let element = e.mem.alloc(0x40);
        for index in 0..16u32 {
            e.mem.set_u32(element + index * 4, 100 + index);
        }
        e.register_double(ARRAY_ELEMENT_ADDRESS, move |_, _| eax(element));
        let data: Ptr<SceneInfoDataStruct> = e.new_object();
        for index in 0..16u32 {
            e.mem.set_u32(data.addr() + index * 4, index);
        }
        e.call(
            0x004a_82d0,
            &args![info, Ptr::<()>::new(node), data, false, Ptr::<()>::NULL],
        );
        for index in 0..16u32 {
            let expected = if index > 3 {
                index + 100 + index
            } else {
                index
            };
            assert_eq!(
                e.mem.u32(data.addr() + index * 4),
                expected,
                "counter {index}"
            );
        }
        // An index past the entries leaves the data alone.
        e.register(0x009e_e040, |_, _| eax(2));
        let before = e.mem.bytes(data.addr(), 0x40);
        e.call(
            0x004a_82d0,
            &args![info, Ptr::<()>::new(node), data, false, Ptr::<()>::NULL],
        );
        assert_eq!(e.mem.bytes(data.addr(), 0x40), before);
    }

    #[test]
    fn geometry_adds_to_the_geometry_counters() {
        let mut e = recursion_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        estimate_doubles(&mut e);
        let geometry_data = geometry_data_with(&mut e, 3);
        let geometry = object_with(&mut e, &[(SLOT_GEOMETRY_DATA, geometry_data)]);
        let node = object_with(&mut e, &[(SLOT_AS_NODE, 0), (SLOT_AS_GEOMETRY, geometry)]);
        let property = object_with(&mut e, &[(SLOT_VISIT, 0)]);
        // The visitor records the context word while the scratch block lives.
        let visit_target = e.mem.u32(e.mem.u32(property) + SLOT_VISIT);
        let visits = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&visits);
        e.register_double(visit_target, move |e, a| {
            record.borrow_mut().push((a[0], a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        e.register_double(0x00a5_9d30, move |_, _| eax(property));
        e.register(0x0044_1110, |_, _| eax(9));
        e.register(0x004a_2020, |_, a| eax(u32::from(a[1] == 0x1a)));
        // One active light, used once; the excess setting is 1.
        e.register(0x00b7_0590, |_, _| eax(0x9000));
        e.register(0x00b7_0680, |_, _| eax(0));
        e.mem.set_u32(SETTING_LIGHT_EXCESS_GEOMETRY + 4, 1);
        e.register(0x004a_1ff0, |_, _| eax(0x0012_2345));
        // Additional streams: two, 10 bytes each.
        e.register(0x0054_95f0, |_, _| eax(0x7000));
        e.register(0x004a_8a90, |_, _| eax(0x7100));
        e.register(0x00a7_27b0, |_, _| eax(2));
        e.register(0x00a7_2550, |e, a| {
            e.mem.set_u32(a[2], 10);
            eax(1)
        });
        e.call_log = Some(vec![]);
        collect(&mut e, info, node, false);
        let data = |offset: u32| e.mem.u32(info.addr() + 4 + offset);
        assert_eq!(data(0x18), 1);
        assert_eq!(data(0x38), 1, "decal test 0x1a");
        assert_eq!(data(0x34), 1, "light use count reached the setting");
        assert_eq!(data(0x1c), 0x2345);
        assert_eq!(data(0x24), 108 + 20);
        // The property visitor got the callback and the address of counter 0x28.
        assert_eq!(
            *visits.borrow(),
            vec![(property, PROPERTY_VISITOR_CALLBACK, info.addr() + 4 + 0x28)]
        );
    }

    #[test]
    fn geometry_with_an_unset_property_skips_the_property_work() {
        let mut e = recursion_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        let geometry = object_with(&mut e, &[(SLOT_GEOMETRY_DATA, 0)]);
        let node = object_with(&mut e, &[(SLOT_AS_NODE, 0), (SLOT_AS_GEOMETRY, geometry)]);
        e.register(0x00a5_9d30, |_, _| eax(0x8000));
        e.register(0x0044_1110, |_, _| eax(u32::MAX));
        e.call_log = Some(vec![]);
        collect(&mut e, info, node, false);
        assert_eq!(e.mem.u32(info.addr() + 4 + 0x18), 1);
        assert_eq!(e.mem.u32(info.addr() + 4 + 0x38), 0);
        assert!(calls_to(&e, 0x004a_2020).is_empty());
        assert!(calls_to(&e, 0x00b7_0590).is_empty());
    }

    #[test]
    fn a_decal_parent_counts_a_geometry_decal_without_the_other_tests() {
        let mut e = recursion_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        let geometry = object_with(&mut e, &[(SLOT_GEOMETRY_DATA, 0)]);
        let node = object_with(&mut e, &[(SLOT_AS_NODE, 0), (SLOT_AS_GEOMETRY, geometry)]);
        let property = object_with(&mut e, &[(SLOT_VISIT, 0)]);
        e.register_double(0x00a5_9d30, move |_, _| eax(property));
        e.register(0x0044_1110, |_, _| eax(0));
        e.register(0x0043_b300, |_, a| {
            eax(u32::from(a[0] == TYPE_DECAL_PARENT))
        });
        e.call_log = Some(vec![]);
        e.call(
            0x004a_82d0,
            &args![
                info,
                Ptr::<()>::new(node),
                Ptr::<()>::NULL,
                false,
                Ptr::<()>::new(0x5555)
            ],
        );
        assert_eq!(e.mem.u32(info.addr() + 4 + 0x38), 1);
        assert!(calls_to(&e, 0x004a_2020).is_empty(), "short-circuited");
    }

    #[test]
    fn other_objects_count_lights_and_actors() {
        let mut e = recursion_engine();
        let info: Ptr<BGSSceneInfo> = e.new_object();
        let node = object_with(&mut e, &[(SLOT_AS_NODE, 0), (SLOT_AS_GEOMETRY, 0)]);
        let actor = e.mem.alloc(0x100);
        e.mem.set_u32(actor + 0xb8, 0x7800);
        e.register_double(0x0065_3270, move |_, a| {
            eax(match a[0] {
                TYPE_LIGHT_OBJECT => 0x7700,
                TYPE_ACTOR_OBJECT => actor,
                _ => 0,
            })
        });
        e.register(0x0045_0b80, |_, _| eax(0x7900));
        e.register(0x00b5_b4a0, |_, a| {
            eax(u32::from(a[0] == 0x7900 && a[1] == 0x7700))
        });
        e.register(0x004a_8ae0, |_, a| {
            eax(if a[0] == 0x7800 { 0xffff_1234 } else { 0 })
        });
        collect(&mut e, info, node, false);
        let data = |offset: u32| e.mem.u32(info.addr() + 4 + offset);
        assert_eq!(data(0x30), 1);
        assert_eq!(data(0x14), 1);
        assert_eq!(data(0x10), 0x1234);
    }

    #[test]
    fn actor_accessor_returns_the_low_word() {
        let mut e = scene_engine();
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0xb8, 0x4321);
        e.register(0x004a_8ae0, |_, a| {
            eax(if a[0] == 0x4321 { 0x0007_0099 } else { 0 })
        });
        assert_eq!(
            e.call(0x004a_8ab0, &args![Ptr::<()>::new(object)]).u32(),
            0x99
        );
    }

    // ---- Budget text ---------------------------------------------------

    /// The settings and the `TES` object for the area budget: a loaded-area
    /// budget of `budget` bytes, wasteland adjustment 0x100000, city
    /// adjustment 0x40000, actor budget 0x200000.
    fn budget_engine(budget: u32) -> Engine {
        let mut e = scene_engine();
        e.mem
            .set_u32(SETTING_LOADED_AREA_NON_ACTOR_BUDGET + 4, budget);
        e.mem
            .set_u32(SETTING_WASTELAND_LOD_ADJUSTMENT + 4, 0x10_0000);
        e.mem.set_u32(SETTING_CITY_LOD_ADJUSTMENT + 4, 0x4_0000);
        e.mem.set_u32(SETTING_ACTOR_MEMORY_BUDGET + 4, 0x20_0000);
        e.mem.set_u32(TES_SINGLETON, 0x1111);
        e
    }

    #[test]
    fn area_budget_is_the_setting_when_inside_an_interior_cell() {
        let mut e = budget_engine(0x80_0000);
        e.register(0x005f_36f0, |_, _| eax(0x9999));
        assert_eq!(e.call(0x004a_9050, &args![]).i32(), 0x80_0000);
    }

    #[test]
    fn area_budget_subtracts_the_lod_adjustment_of_the_outermost_world() {
        for (form_id, adjustment) in [(0x3cu32, 0x10_0000i32), (0x1234, 0x4_0000)] {
            let mut e = budget_engine(0x80_0000);
            e.register(0x005f_36f0, |_, _| eax(0));
            e.register(0x004f_d3e0, |_, _| eax(0xa));
            // World 0xa has the parent 0xb, which has none.
            e.register(0x0058_6390, |_, a| eax(if a[0] == 0xa { 0xb } else { 0 }));
            e.register_double(0x0084_e3a0, move |_, a| {
                assert_eq!(a[0], 0xb);
                eax(form_id)
            });
            assert_eq!(e.call(0x004a_9050, &args![]).i32(), 0x80_0000 - adjustment);
        }
    }

    #[test]
    fn area_budget_without_a_world_is_the_setting() {
        let mut e = budget_engine(0x80_0000);
        e.register(0x005f_36f0, |_, _| eax(0));
        e.register(0x004f_d3e0, |_, _| eax(0));
        assert_eq!(e.call(0x004a_9050, &args![]).i32(), 0x80_0000);
    }

    #[test]
    fn actor_budget_is_the_setting() {
        let mut e = budget_engine(0);
        assert_eq!(e.call(0x004a_9100, &args![]).u32(), 0x20_0000);
    }

    type FormatCall = (u32, u32, [f64; 3]);

    /// Records the `sprintf` call: destination, format, then the three
    /// doubles.
    fn capture_format(e: &mut Engine) -> Rc<RefCell<Vec<FormatCall>>> {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&calls);
        e.register_double(STRING_FORMAT, move |_, a| {
            let double = |i: usize| f64::from_bits(u64::from(a[i]) | u64::from(a[i + 1]) << 32);
            record
                .borrow_mut()
                .push((a[0], a[1], [double(2), double(4), double(6)]));
            Ret::default()
        });
        calls
    }

    #[test]
    fn m_number_string_picks_megabytes_kilobytes_or_bytes() {
        let mut e = budget_engine(0);
        let calls = capture_format(&mut e);
        let text = Ptr::<()>::new(0x5000);
        // The actor budget is 2 MB.
        for (value, format, shown, percent) in [
            (3145728.0f32, FORMAT_MEGABYTES, 3.0, 150.0),
            (2048.0, FORMAT_KILOBYTES, 2.0, 0.09765625),
            (512.0, FORMAT_BYTES, 512.0, 0.0244140625),
            (1024.0, FORMAT_BYTES, 1024.0, 0.048828125),
            (1048576.0, FORMAT_KILOBYTES, 1024.0, 50.0),
        ] {
            e.call(0x004a_9120, &args![value, text, true]);
            let call = calls.borrow().last().copied().unwrap();
            assert_eq!(call.0, text.addr());
            assert_eq!(call.1, format, "value {value}");
            assert_eq!(call.2, [percent, shown, 2.0], "value {value}");
        }
    }

    #[test]
    fn m_number_string_uses_the_area_budget_without_the_actor_flag() {
        let mut e = budget_engine(0x30_0000);
        e.register(0x005f_36f0, |_, _| eax(1));
        let calls = capture_format(&mut e);
        e.call(
            0x004a_9120,
            &args![1572864.0f32, Ptr::<()>::new(0x5000), false],
        );
        let call = calls.borrow()[0];
        assert_eq!(call.1, FORMAT_MEGABYTES);
        assert_eq!(call.2, [50.0, 1.5, 3.0]);
    }

    // ---- Model score ---------------------------------------------------

    /// A leaf object whose kind word (the pointer at +0xc) is `kind`.
    fn kind_object(e: &mut Engine, kind: u32) -> u32 {
        let object = e.mem.alloc(0x20);
        let marker = e.mem.alloc(4);
        e.mem.set_u32(marker, kind);
        e.mem.set_u32(object + 0x0c, marker);
        object
    }

    /// A container of kind `kind` whose children are returned in order at
    /// positions 0, 1, ...; the walk ends at position -1.
    fn container_of(e: &mut Engine, kind: u32, children: &[u32]) -> u32 {
        let table = e.mem.alloc(4 * children.len() as u32 + 4);
        for (i, child) in children.iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *child);
        }
        let first = if children.is_empty() { u32::MAX } else { 0 };
        let count = children.len() as u32;
        let list = object_with(e, &[(0x08, first)]);
        let vtable = e.mem.u32(list);
        let child_at = 0x7ffe_0014 + list;
        e.register_double(child_at, move |e, a| eax(e.mem.u32(table + 4 * a[1])));
        e.mem.set_u32(vtable + 0x14, child_at);
        let next_of = 0x7ffe_000c + list;
        e.register_double(next_of, move |_, a| {
            eax(if a[1] + 1 < count { a[1] + 1 } else { u32::MAX })
        });
        e.mem.set_u32(vtable + 0x0c, next_of);
        let object = kind_object(e, kind);
        let object_vtable = e.mem.alloc(0x20);
        let list_of = 0x7ffd_0010 + list;
        e.register_double(list_of, move |_, _| eax(list));
        e.mem.set_u32(object_vtable + 0x10, list_of);
        e.mem.set_u32(object, object_vtable);
        object
    }

    #[test]
    fn model_score_by_kind() {
        let mut e = scene_engine();
        e.register(0x0043_b230, |e, a| eax(e.mem.u32(e.mem.u32(a[0] + 0x0c))));
        let score =
            |e: &mut Engine, object: u32| e.call(0x004a_9240, &args![Ptr::<()>::new(object)]).u32();
        assert_eq!(score(&mut e, 0), 0);
        let one = kind_object(&mut e, 3);
        assert_eq!(score(&mut e, one), 1);
        for other in [4u32, 8, 0x0b, 0x15, 0x18, 0x100] {
            let object = kind_object(&mut e, other);
            assert_eq!(score(&mut e, object), 10, "kind {other}");
        }
        // Containers of kinds 9, 10, 0x16 and 0x17 add up their children.
        for kind in [9u32, 10, 0x16, 0x17] {
            let a = kind_object(&mut e, 3);
            let b = kind_object(&mut e, 5);
            let c = kind_object(&mut e, 3);
            let container = container_of(&mut e, kind, &[a, b, c]);
            assert_eq!(score(&mut e, container), 1 + 10 + 1, "kind {kind}");
        }
        // An empty container, and a null child ending the walk.
        let empty = container_of(&mut e, 9, &[]);
        assert_eq!(score(&mut e, empty), 0);
        let a = kind_object(&mut e, 3);
        let stops = container_of(&mut e, 10, &[a, 0, a]);
        assert_eq!(score(&mut e, stops), 1);
    }

    // ---- Containers ----------------------------------------------------

    #[test]
    fn element_address_goes_through_the_array_helper() {
        let mut e = scene_engine();
        array_doubles(&mut e);
        let (array, buffer) = array_with(&mut e, 3, 4);
        assert_eq!(
            e.call(0x004a_95b0, &args![array, 2u32]).u32(),
            buffer + 0x80
        );
    }

    /// Checks a map constructor: vtable, bucket count, zero count and a
    /// cleared bucket array of `buckets * 4` bytes.
    fn check_map_constructor(address: u32, vtable: u32) {
        let mut e = scene_engine();
        e.register(ALLOCATE, |e, a| {
            let table = e.mem.alloc(a[0]);
            e.mem.write(table, &vec![0xff; a[0] as usize]);
            eax(table)
        });
        let map: Ptr<NiTMap> = e.new_object();
        e.set(map, NiTMap::m_uiCount, 77);
        let result = e.call(address, &args![map, 0x25u32]).ptr::<NiTMap>();
        assert_eq!(result, map);
        assert_eq!(e.mem.u32(map.addr()), vtable);
        assert_eq!(e.get(map, NiTMap::m_uiHashSize), 0x25);
        assert_eq!(e.get(map, NiTMap::m_uiCount), 0);
        let table = e.get(map, NiTMap::m_ppkHashTable);
        assert_eq!(e.mem.bytes(table, 0x25 * 4), vec![0u8; 0x25 * 4]);
    }

    #[test]
    fn texture_user_data_map_base_constructor() {
        check_map_constructor(0x004a_97e0, 0x0101_ed2c);
    }

    #[test]
    fn cell_flag_map_base_constructor() {
        check_map_constructor(0x004a_98e0, 0x0101_ed4c);
    }

    #[test]
    fn texture_flag_map_base_constructor() {
        check_map_constructor(0x004a_9c50, 0x0101_ed8c);
    }

    #[test]
    fn texture_user_data_map_constructor_sets_its_own_vtable_last() {
        check_map_constructor(0x004a_95d0, 0x0101_ecec);
    }

    #[test]
    fn cell_flag_map_constructor_sets_its_own_vtable_last() {
        check_map_constructor(0x004a_9600, 0x0101_ed0c);
    }

    #[test]
    fn texture_flag_map_constructor_sets_its_own_vtable_last() {
        check_map_constructor(0x004a_9bf0, 0x0101_ed6c);
    }

    #[test]
    fn cell_flag_map_destructor_empties_then_runs_the_base_body() {
        let mut e = scene_engine();
        let map: Ptr<NiTMap> = e.new_object();
        stub(&mut e, &[MAP_REMOVE_ALL, CELL_FLAG_MAP_BASE_DESTROY]);
        e.call_log = Some(vec![]);
        e.call(0x004a_9970, &args![map]);
        assert_eq!(e.mem.u32(map.addr()), 0x0101_ed0c);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![0x004a_9970, MAP_REMOVE_ALL, CELL_FLAG_MAP_BASE_DESTROY]
        );
    }

    #[test]
    fn texture_flag_map_base_destructor_empties_then_frees_the_buckets() {
        let mut e = scene_engine();
        let map: Ptr<NiTMap> = e.new_object();
        e.set(map, NiTMap::m_ppkHashTable, 0x4242);
        stub(&mut e, &[MAP_REMOVE_ALL, DEALLOCATE]);
        e.call_log = Some(vec![]);
        e.call(0x004a_9d20, &args![map]);
        assert_eq!(e.mem.u32(map.addr()), 0x0101_ed8c);
        let log = e.call_log.clone().unwrap();
        assert_eq!(log[1], (MAP_REMOVE_ALL, vec![map.addr()]));
        assert_eq!(log[2], (DEALLOCATE, vec![0x4242]));
    }

    #[test]
    fn texture_flag_map_destructor_runs_the_base_body_last() {
        let mut e = scene_engine();
        let map: Ptr<NiTMap> = e.new_object();
        stub(&mut e, &[MAP_REMOVE_ALL, DEALLOCATE]);
        e.call_log = Some(vec![]);
        e.call(0x004a_9cc0, &args![map]);
        assert_eq!(e.mem.u32(map.addr()), 0x0101_ed8c);
        let called: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            called,
            vec![0x004a_9cc0, MAP_REMOVE_ALL, MAP_REMOVE_ALL, DEALLOCATE]
        );
    }

    /// A scalar deleting destructor runs its body and frees with bit 0.
    fn check_deleting_destructor(address: u32, body: u32) {
        let mut e = scene_engine();
        let map: Ptr<NiTMap> = e.new_object();
        stub(&mut e, &[body]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(address, &args![map, 0u32]).ptr::<NiTMap>(), map);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, body), vec![vec![map.addr()]]);
        e.call(address, &args![map, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![map.addr()]]);
    }

    #[test]
    fn texture_user_data_map_deleting_destructor() {
        check_deleting_destructor(0x004a_9630, TEXTURE_USER_DATA_MAP_DESTROY);
    }

    #[test]
    fn cell_flag_map_deleting_destructor() {
        let mut e = scene_engine();
        let map: Ptr<NiTMap> = e.new_object();
        stub(&mut e, &[MAP_REMOVE_ALL, CELL_FLAG_MAP_BASE_DESTROY]);
        e.call_log = Some(vec![]);
        e.call(0x004a_9660, &args![map, 1u32]);
        assert_eq!(e.mem.u32(map.addr()), 0x0101_ed0c);
        assert_eq!(calls_to(&e, CELL_FLAG_MAP_BASE_DESTROY).len(), 1);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![map.addr()]]);
    }

    #[test]
    fn texture_user_data_map_base_deleting_destructor() {
        check_deleting_destructor(0x004a_9a00, TEXTURE_USER_DATA_MAP_BASE_DESTROY);
    }

    #[test]
    fn cell_flag_map_base_deleting_destructor() {
        check_deleting_destructor(0x004a_9a30, CELL_FLAG_MAP_BASE_DESTROY);
    }

    #[test]
    fn texture_flag_map_deleting_destructors_run_the_bodies_in_this_file() {
        let mut e = scene_engine();
        stub(&mut e, &[MAP_REMOVE_ALL, DEALLOCATE]);
        let map: Ptr<NiTMap> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x004a_9c20, &args![map, 1u32]);
        assert_eq!(e.mem.u32(map.addr()), 0x0101_ed8c);
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x004a_9d50, &args![map, 0u32]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, DEALLOCATE).len(), 1);
    }

    #[test]
    fn append_constructs_and_copies_the_entry() {
        let mut e = scene_engine();
        array_doubles(&mut e);
        let (array, buffer) = array_with(&mut e, 2, 4);
        let element = e.mem.alloc(0x40);
        e.mem.write(element, &[0x77; 0x40]);
        e.mem.write(buffer + 0x80, &[0xee; 0x40]);
        let index = e
            .call(0x004a_9a60, &args![array, Ptr::<()>::new(element)])
            .u32();
        assert_eq!(index, 2);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);
        assert_eq!(e.mem.bytes(buffer + 0x80, 0x40), vec![0x77; 0x40]);
    }

    #[test]
    fn placement_construction_clears_each_entry_unless_new_returns_null() {
        let mut e = scene_engine();
        let (array, buffer) = array_with(&mut e, 0, 3);
        e.mem.write(buffer, &[0xff; 0xc0]);
        e.call(0x004a_9ab0, &args![array, buffer + 0x40, 2u32]);
        assert_eq!(e.mem.bytes(buffer, 0x40), vec![0xff; 0x40]);
        assert_eq!(e.mem.bytes(buffer + 0x40, 0x80), vec![0; 0x80]);
        e.mem.write(buffer, &[0xff; 0xc0]);
        e.register(PLACEMENT_NEW, |_, _| eax(0));
        e.call(0x004a_9ab0, &args![array, buffer, 3u32]);
        assert_eq!(e.mem.bytes(buffer, 0xc0), vec![0xff; 0xc0]);
    }

    #[test]
    fn move_entries_handles_both_directions_and_nothing_to_do() {
        let mut e = scene_engine();
        let (array, buffer) = array_with(&mut e, 4, 5);
        // Up by one (destination above source): copies from the end.
        e.call_log = Some(vec![]);
        e.call(0x004a_9b50, &args![array, buffer + 0x40, buffer, 4u32]);
        assert_eq!(entry_bytes(&e, buffer, 5), vec![1, 1, 2, 3, 4]);
        let moves = calls_to(&e, MEMMOVE);
        assert_eq!(moves.len(), 4);
        assert_eq!(moves[0][0], buffer + 0x40 + 3 * 0x40, "last entry first");
        // Down by one (destination below source): copies from the start.
        e.call_log = Some(vec![]);
        e.call(0x004a_9b50, &args![array, buffer, buffer + 0x40, 4u32]);
        assert_eq!(entry_bytes(&e, buffer, 5), vec![1, 2, 3, 4, 4]);
        assert_eq!(calls_to(&e, MEMMOVE)[0][0], buffer);
        // Equal addresses or zero entries do nothing.
        e.call_log = Some(vec![]);
        e.call(0x004a_9b50, &args![array, buffer, buffer, 4u32]);
        e.call(0x004a_9b50, &args![array, buffer, buffer + 0x40, 0u32]);
        assert!(calls_to(&e, MEMMOVE).is_empty());
    }

    fn full_check(e: &mut Engine) {
        e.register(ARRAY_IS_FULL, |e, a| {
            eax(u32::from(e.mem.u32(a[0] + 8) == e.mem.u32(a[0] + 0xc)))
        });
    }

    #[test]
    fn insert_at_the_end_appends() {
        let mut e = scene_engine();
        array_doubles(&mut e);
        let (array, buffer) = array_with(&mut e, 2, 4);
        let element = e.mem.alloc(0x40);
        e.mem.write(element, &[9; 0x40]);
        e.call(0x004a_9690, &args![array, 2u32, Ptr::<()>::new(element)]);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);
        assert_eq!(entry_bytes(&e, buffer, 3), vec![1, 2, 9]);
    }

    #[test]
    fn insert_in_the_middle_shifts_the_tail_up() {
        let mut e = scene_engine();
        full_check(&mut e);
        let (array, buffer) = array_with(&mut e, 3, 4);
        let element = e.mem.alloc(0x40);
        e.mem.write(element, &[9; 0x40]);
        e.call(0x004a_9690, &args![array, 1u32, Ptr::<()>::new(element)]);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 4);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), buffer);
        assert_eq!(entry_bytes(&e, buffer, 4), vec![1, 9, 2, 3]);
    }

    #[test]
    fn insert_into_a_full_array_grows_into_a_new_buffer() {
        let mut e = scene_engine();
        full_check(&mut e);
        e.register(ARRAY_NEXT_CAPACITY, |_, _| eax(8));
        e.register(ARRAY_FREE_BUFFER, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        let (array, old_buffer) = array_with(&mut e, 3, 3);
        // Virtual allocate (slot 4) hands out a fresh block of entries.
        let vtable = e.mem.alloc(0x20);
        e.register(0x7ffd_0004, |e, a| eax(e.mem.alloc(a[1] * 0x40)));
        e.mem.set_u32(vtable + 4, 0x7ffd_0004);
        e.mem.set_u32(array.addr(), vtable);
        let element = e.mem.alloc(0x40);
        e.mem.write(element, &[9; 0x40]);
        e.call_log = Some(vec![]);
        e.call(0x004a_9690, &args![array, 1u32, Ptr::<()>::new(element)]);
        let new_buffer = e.get(array, BSSimpleArray::pBuffer);
        assert_ne!(new_buffer, old_buffer);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 4);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 8);
        assert_eq!(entry_bytes(&e, new_buffer, 4), vec![1, 9, 2, 3]);
        assert_eq!(calls_to(&e, ARRAY_FREE_BUFFER), vec![vec![array.addr()]]);
        assert_eq!(calls_to(&e, 0x7ffd_0004), vec![vec![array.addr(), 8]]);
    }

    #[test]
    fn scene_data_array_constructor_and_destructor() {
        let mut e = scene_engine();
        stub(&mut e, &[SCENE_DATA_ARRAY_CONSTRUCT, ARRAY_REMOVE_ALL]);
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004a_9d80, &args![array]).ptr::<BSSimpleArray>(),
            array
        );
        assert_eq!(e.mem.u32(array.addr()), 0x0101_edac);
        assert_eq!(
            calls_to(&e, SCENE_DATA_ARRAY_CONSTRUCT),
            vec![vec![array.addr(), 0, 0]]
        );
        e.mem.set_u32(array.addr(), 0);
        e.call(0x004a_9de0, &args![array]);
        assert_eq!(e.mem.u32(array.addr()), 0x0101_edac);
        assert_eq!(calls_to(&e, ARRAY_REMOVE_ALL), vec![vec![array.addr(), 1]]);
    }

    #[test]
    fn scene_data_array_deleting_destructor() {
        let mut e = scene_engine();
        stub(&mut e, &[ARRAY_REMOVE_ALL]);
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x004a_9db0, &args![array, 0u32]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, ARRAY_REMOVE_ALL).len(), 1);
        e.call(0x004a_9db0, &args![array, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![array.addr()]]);
    }

    #[test]
    fn scene_data_array_constructor_reserves_and_fills() {
        let mut e = scene_engine();
        e.register(0x7ffd_0004, |e, a| eax(e.mem.alloc(a[1] * 0x40)));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, 0x7ffd_0004);
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(array.addr(), vtable);
        e.call_log = Some(vec![]);
        // Nothing requested: stays empty, no allocation.
        e.call(0x004a_9e00, &args![array, 0u32, 0u32]);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 0);
        assert!(calls_to(&e, 0x7ffd_0004).is_empty());
        // Reserve smaller than size: the size wins.
        e.call(0x004a_9e00, &args![array, 2u32, 5u32]);
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        assert_ne!(buffer, 0);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 5);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 5);
        assert_eq!(calls_to(&e, 0x7ffd_0004), vec![vec![array.addr(), 5]]);
        assert_ne!(buffer, 0);
        // Reserve only: capacity without entries.
        e.call(0x004a_9e00, &args![array, 7u32, 0u32]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 7);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 0);
    }

    /// The three map families added in session 2:
    /// (constructor, deleting destructor, base constructor, destructor body,
    /// base destructor body, base deleting destructor, vtable, base vtable).
    const MAP_FAMILIES: [[u32; 8]; 3] = [
        [
            0x004a_9e90,
            0x004a_9ec0,
            0x004a_9ef0,
            0x004a_9f60,
            0x004a_9fc0,
            0x004a_9ff0,
            0x0101_edc0,
            0x0101_ede0,
        ],
        [
            0x004a_a020,
            0x004a_a050,
            0x004a_a080,
            0x004a_a0f0,
            0x004a_a150,
            0x004a_a180,
            0x0101_ee00,
            0x0101_ee20,
        ],
        [
            0x004a_a1b0,
            0x004a_a1e0,
            0x004a_a210,
            0x004a_a280,
            0x004a_a2e0,
            0x004a_a310,
            0x0101_ee40,
            0x0101_ee60,
        ],
    ];

    fn allocation_doubles(e: &mut Engine) {
        e.register(ALLOCATE, |e, a| eax(e.mem.alloc(a[0])));
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
    }

    #[test]
    fn map_constructors_build_the_bucket_array() {
        for [ctor, _, base_ctor, _, _, _, vtable, base_vtable] in MAP_FAMILIES {
            let mut e = scene_engine();
            allocation_doubles(&mut e);
            let map: Ptr<NiTMap> = e.new_object();
            e.mem.set_u32(map.addr() + 0x0c, 99);
            assert_eq!(e.call(ctor, &args![map, 0x25u32]).ptr::<NiTMap>(), map);
            assert_eq!(e.mem.u32(map.addr()), vtable);
            assert_eq!(e.get(map, NiTMap::m_uiHashSize), 0x25);
            assert_eq!(e.get(map, NiTMap::m_uiCount), 0);
            let table = e.get(map, NiTMap::m_ppkHashTable);
            assert_ne!(table, 0);
            assert_eq!(e.mem.bytes(table, 0x25 * 4), vec![0; 0x25 * 4]);
            // The base constructor alone stores the base vtable.
            e.call(base_ctor, &args![map, 3u32]);
            assert_eq!(e.mem.u32(map.addr()), base_vtable);
            assert_eq!(e.get(map, NiTMap::m_uiHashSize), 3);
        }
    }

    #[test]
    fn map_destructor_bodies_empty_the_map_and_free_the_buckets() {
        for [_, _, _, dtor, base_dtor, _, vtable, base_vtable] in MAP_FAMILIES {
            let mut e = scene_engine();
            allocation_doubles(&mut e);
            let map: Ptr<NiTMap> = e.new_object();
            e.set(map, NiTMap::m_ppkHashTable, 0x1234);
            e.call_log = Some(vec![]);
            e.call(dtor, &args![map]);
            assert_eq!(e.mem.u32(map.addr()), base_vtable);
            assert_eq!(calls_to(&e, MAP_REMOVE_ALL).len(), 2);
            assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![0x1234]]);
            let _ = vtable;
            e.call_log = Some(vec![]);
            e.call(base_dtor, &args![map]);
            assert_eq!(calls_to(&e, MAP_REMOVE_ALL), vec![vec![map.addr()]]);
            assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![0x1234]]);
        }
    }

    #[test]
    fn map_deleting_destructors_free_only_with_bit_zero() {
        for [_, sdd, _, _, _, base_sdd, _, _] in MAP_FAMILIES {
            for address in [sdd, base_sdd] {
                let mut e = scene_engine();
                allocation_doubles(&mut e);
                let map: Ptr<NiTMap> = e.new_object();
                e.call_log = Some(vec![]);
                assert_eq!(e.call(address, &args![map, 0u32]).ptr::<NiTMap>(), map);
                assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
                assert_eq!(calls_to(&e, DEALLOCATE).len(), 1);
                e.call(address, &args![map, 1u32]);
                assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![map.addr()]]);
            }
        }
    }

    #[test]
    fn rendered_texture_get_texture_reads_the_slot_or_a_null_temporary() {
        let mut e = scene_engine();
        e.register(0x0063_3c90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            eax(a[0])
        });
        e.register(0x0045_cec0, |_, _| Ret::default());
        let texture = e.mem.alloc(0x40);
        e.mem.set_u32(texture + 0x30 + 8, 0xabcd);
        e.call_log = Some(vec![]);
        let got = e.call(0x004b_c320, &args![texture, 2u32]).u32();
        assert_eq!(got, 0xabcd);
        assert!(calls_to(&e, 0x0063_3c90).is_empty());
        // Null `this`: a temporary smart pointer holding null is read and
        // destroyed.
        let got = e.call(0x004b_c320, &args![0u32, 2u32]).u32();
        assert_eq!(got, 0);
        assert_eq!(calls_to(&e, 0x0063_3c90).len(), 1);
        assert_eq!(calls_to(&e, 0x0045_cec0).len(), 1);
    }
}
