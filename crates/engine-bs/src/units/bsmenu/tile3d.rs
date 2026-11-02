//! `BSMenu/tile3d.cpp` (Xbox PDB source unit), subsystem `BSMenu`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `Tile3D` shows a NIF model in a menu: the model file (from the file name
//! trait) is loaded under the tile's `NiNode`, its properties are replaced
//! by tile shader properties, and its `NiControllerManager` plays the
//! sequence the animation trait names.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bsmenu::font::{
    INTERLOCKED_DECREMENT, NI_AV_OBJECT_ATTACH_PROPERTY, NI_AV_OBJECT_UPDATE_PROPERTIES,
    NI_FIXED_STRING_CREATE, NI_FIXED_STRING_EMPTY, NI_NEW, TILE_SHADER_PROPERTY_CONSTRUCTOR,
    TILE_SHADER_PROPERTY_SET_TILE_TEXTURE,
};
use crate::units::bsmenu::tile::{
    Tile, FLAG_RELEASED, TRAIT_ALPHA, TRAIT_BLUE, TRAIT_BRIGHTNESS, TRAIT_FILENAME, TRAIT_GREEN,
    TRAIT_LOCUS, TRAIT_RED, TRAIT_STRING, TRAIT_VISIBLE, UPDATE_NIF_FILE,
};

layout! {
    /// `Tile3D` (Xbox PDB), 0x50 bytes on PC as on the Xbox.
    pub struct Tile3D: 0x50 {
        /// `pAniManager` (Xbox PDB): `NiControllerManager*`.
        0x38 pAniManager: Ptr,
        /// `pCurrentSequence` (Xbox PDB): `NiControllerSequence*`.
        0x3C pCurrentSequence: Ptr,
        /// `strCurrentName` (Xbox PDB): `BSStringT<char>`, text at +0x40,
        /// length (`0xFFFF` when unknown) at +0x44.
        0x40 strCurrentName: u32,
        0x44 strCurrentNameLength: u16,
        /// `strFilename` (Xbox PDB): `BSStringT<char>`.
        0x48 strFilename: u32,
    }
}

/// `Tile::Init` (`009ff600`), `Tile::Release` (`009ff690`) and
/// `Tile::~Tile` (`009ff340`) (Xbox PDB names).
const TILE_INIT: u32 = 0x009f_f600;
const TILE_RELEASE: u32 = 0x009f_f690;
const TILE_DESTRUCTOR: u32 = 0x009f_f340;
/// Sets a float trait (`00a012d0`, thiscall: trait, value, flag).
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
/// `Tile::SetNeedsUpdate` (`00a074d0`) and `Tile::AddNeedsUpdate`
/// (`00a07530`) (Xbox PDB names).
const TILE_SET_NEEDS_UPDATE: u32 = 0x00a0_74d0;
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
/// `Tile::GetParentModel` (`00a089c0`), `Tile::IsTrue` (`00a01230`) and
/// `Tile::GetString` (`00a011f0`) (Xbox PDB names).
const TILE_GET_PARENT_MODEL: u32 = 0x00a0_89c0;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
const TILE_GET_STRING: u32 = 0x00a0_11f0;
/// `Tile::Extra::Extra` (`00a1f610`, Xbox PDB name, in `tilerect.cpp`).
const TILE_EXTRA_CONSTRUCTOR: u32 = 0x00a1_f610;
/// `BSStringT<char>::Set(text, length)` (`004037f0`) and
/// `BSStringT<char>::StrCmp(text, ignore case)` (`00408a80`, Xbox PDB
/// name).
const BS_STRING_SET: u32 = 0x0040_37f0;
const BS_STRING_STR_CMP: u32 = 0x0040_8a80;
/// The formatted string write (`00406f60`, cdecl: string, format, ...).
const BS_STRING_FORMAT: u32 = 0x0040_6f60;
/// `strstr` (`00ec7750`) and `strlen` (`00ec6130`), cdecl.
const STRSTR: u32 = 0x00ec_7750;
const STRLEN: u32 = 0x00ec_6130;
/// The `NiNode` constructor (`00a5ecb0`) and the `NiPointer` assignment
/// (`0066b0d0`).
const NI_NODE_CONSTRUCTOR: u32 = 0x00a5_ecb0;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiObjectNET::AddExtraData` (`00a5bca0`, `_ov2`), `NiObjectNET::
/// GetController` (`00a5c570`), `NiAVObject::RemoveProperty` (`00a5b230`,
/// `_ov2`), `NiAVObject::GetProperty` (`00a59d30`) (Xbox PDB names) and the
/// node update with an `NiUpdateData` (`00a59c60`).
const NI_OBJECT_NET_ADD_EXTRA_DATA: u32 = 0x00a5_bca0;
const NI_OBJECT_NET_GET_CONTROLLER: u32 = 0x00a5_c570;
const NI_AV_OBJECT_REMOVE_PROPERTY: u32 = 0x00a5_b230;
const NI_AV_OBJECT_GET_PROPERTY: u32 = 0x00a5_9d30;
const NI_AV_OBJECT_UPDATE: u32 = 0x00a5_9c60;
/// `NiTArray<NiPointer<NiAVObject>>::Compact` (`004afc80`), `UpdateSize`
/// (`004afe50`) and `RemoveAll` (`004dffa0`; the engine map gives the
/// `BSTempEffect` instance's name to the shared body) (Xbox PDB names).
const CHILDREN_COMPACT: u32 = 0x004a_fc80;
const CHILDREN_UPDATE_SIZE: u32 = 0x004a_fe50;
const CHILDREN_REMOVE_ALL: u32 = 0x004d_ffa0;
/// `BSShaderManager::GetShader` (`00b55560`, Xbox PDB name, cdecl).
const BS_SHADER_MANAGER_GET_SHADER: u32 = 0x00b5_5560;
/// The sequence map look-up (`00853130`, thiscall on the manager's map at
/// +0x50: name handle, out pointer; a `bool`).
const SEQUENCE_MAP_GET: u32 = 0x0085_3130;
/// The sequence activation (`00a34f20`, NiAnimation/nicontrollersequence.cpp,
/// six words; a `bool`).
const SEQUENCE_ACTIVATE: u32 = 0x00a3_4f20;
/// `Interface::ReleaseModelFile` (`00707640`), `LoadModelFile`
/// (`00707660`), `ClearTempModel` (`00707820`), `CopyOrDeepCopyNode`
/// (`00707870`) (Xbox PDB names), the model folder (`00707860`) and the
/// sequence check (`00707970`), cdecl.
const INTERFACE_RELEASE_MODEL_FILE: u32 = 0x0070_7640;
const INTERFACE_LOAD_MODEL_FILE: u32 = 0x0070_7660;
const INTERFACE_CLEAR_TEMP_MODEL: u32 = 0x0070_7820;
const INTERFACE_MODEL_FOLDER: u32 = 0x0070_7860;
const INTERFACE_COPY_OR_DEEP_COPY_NODE: u32 = 0x0070_7870;
const INTERFACE_SEQUENCE_DONE: u32 = 0x0070_7970;
/// `Interface::GetInterfaceRoot` (`00706d30`) and `SetNeedToUpdate`
/// (`00707930`) (Xbox PDB names, cdecl).
const INTERFACE_GET_INTERFACE_ROOT: u32 = 0x0070_6d30;
const INTERFACE_SET_NEED_TO_UPDATE: u32 = 0x0070_7930;
/// `BSTimer::Enable` (`00aa5040`) and the timer's tick (`00aa4ee0`) on
/// `Tile3D::xTimer` (Xbox PDB static, `011f3560`).
const BS_TIMER_ENABLE: u32 = 0x00aa_5040;
const BS_TIMER_TICK: u32 = 0x00aa_4ee0;
const TIMER: u32 = 0x011f_3560;
/// The `GetTickCount` import slot (`00fdf060`).
const GET_TICK_COUNT: u32 = 0x00fd_f060;
/// `NiControllerManager::ms_RTTI` (`011f36ac`).
const NI_CONTROLLER_MANAGER_RTTI: u32 = 0x011f_36ac;
/// `Tile3D`'s vtable (`010948ac`, RTTI `.?AVTile3D@@`).
const TILE_3D_VTABLE: u32 = 0x0109_48ac;
/// Virtual slots: `NiObject::IsNode` (`+0xC`), `IsGeometry` (`+0x18`),
/// the node's attach-child (`+0xDC`), `Tile::SetAlphaAndColor` (`+0x24`)
/// and the sequence's deactivation (`+0x8C`).
const IS_NODE_SLOT: u32 = 0x0c;
const IS_GEOMETRY_SLOT: u32 = 0x18;
const NODE_ATTACH_CHILD_SLOT: u32 = 0xdc;
const SET_ALPHA_AND_COLOR_SLOT: u32 = 0x24;
const SEQUENCE_DEACTIVATE_SLOT: u32 = 0x8c;

/// `255.0f` (`01023cd8`): `Init`'s brightness, colour and alpha.
const FULL: u32 = 0x0102_3cd8;
/// The empty string (`01011584`).
const EMPTY_STRING: u32 = 0x0101_1584;
/// `"\Data"` (`01095738`), `"%s"` (`01019f08`) and `"%s\%s"` (`0101dcc4`).
const DATA_FOLDER: u32 = 0x0109_5738;
const FORMAT_ONE: u32 = 0x0101_9f08;
const FORMAT_TWO: u32 = 0x0101_dcc4;
/// `FLT_MAX` (`01095728`, a `float`; negated it marks a sequence that has
/// not run), `-1.0` (`0101a6b0`) and `1.0` (`01012070`) (`double`), and
/// the `float` added to the last time when the animation is advanced
/// (`011f356c`).
const FLOAT_MAX: u32 = 0x0109_5728;
const MINUS_ONE: u32 = 0x0101_a6b0;
const ONE: u32 = 0x0101_2070;
const TIME_STEP: u32 = 0x011f_356c;
/// Trait `0xFD2`: the animation to play (no name in
/// `crate::units::bsmenu::tile`).
const TRAIT_ANIMATION: i32 = 0xfd2;
/// `Tile::uiFlags` bit `0x20000`: the model came from the interface's
/// model files and is released with the tile.
const FLAG_MODEL_FILE: u32 = 0x2_0000;
/// The shader type a tile shader property answers (property +0x1C).
const TILE_SHADER_TYPE: u32 = 0x13;
/// Property types for `GetProperty` (the shade and texturing properties).
const SHADE_PROPERTY: u32 = 3;
const TEXTURING_PROPERTY: u32 = 5;
/// `NiTimeController::m_uFlags` bit 8 on the controller manager (word +8).
const MANAGER_ACTIVE_FLAG: u16 = 8;

/// The children of an `NiNode`: the array's buffer (+0xA0) and element
/// `i`.
fn child_at(e: &Engine, node: u32, i: u32) -> u32 {
    let base = e.mem.u32(node + 0xa0);
    e.mem.u32(base.wrapping_add(i.wrapping_mul(4)))
}

/// Runs the node update (`00a59c60`) on the tile's model with an
/// `NiUpdateData` of time `time` (update controllers set, the other four
/// flags clear).
fn update_model(e: &mut Engine, this: Ptr<Tile3D>, time: u32) {
    e.with_stack(0x0c, |e, data| {
        e.mem.set_u32(data.addr(), time);
        e.mem.set_u8(data.addr() + 4, 1);
        for i in 5..9 {
            e.mem.set_u8(data.addr() + i, 0);
        }
        let model = e.get(this.cast::<Tile>(), Tile::spModel);
        e.call(NI_AV_OBJECT_UPDATE, &args![model, data]);
    });
}

/// Sets bit 0 of the model's flags word (+0x30) for a hidden tile, clears
/// it for a visible one.
fn show_or_hide(e: &mut Engine, this: Ptr<Tile3D>) {
    let model = e.get(this.cast::<Tile>(), Tile::spModel);
    let visible = e.call(TILE_IS_TRUE, &args![this, TRAIT_VISIBLE]).bool();
    let flags = e.mem.u32(model.addr() + 0x30);
    if !visible {
        e.mem.set_u32(model.addr() + 0x30, flags | 1);
    } else {
        e.mem.set_u32(model.addr() + 0x30, flags & !1);
    }
}

/// Sets or clears the manager's active flag; `set` is a constant at each
/// call site (the other branch never runs there).
fn set_manager_active(e: &mut Engine, this: Ptr<Tile3D>, set: bool) {
    let manager = e.get(this, Tile3D::pAniManager);
    let flags = e.mem.u16(manager.addr() + 8);
    if set {
        e.mem
            .set_u16(manager.addr() + 8, flags | MANAGER_ACTIVE_FLAG);
    } else {
        e.mem
            .set_u16(manager.addr() + 8, flags & !MANAGER_ACTIVE_FLAG);
    }
}

// Translated from 00a20650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile3D::~Tile3D` (Xbox PDB): releases the model file when the tile
/// loaded one (flag `0x20000`), runs `Tile::Release` unless the tile is
/// already released, empties the file name and the current name, and runs
/// `Tile::~Tile`.
// Translation incomplete: 00a20650 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn tile_3d_destructor(e: &mut Engine, this: Ptr<Tile3D>) {
    e.mem.set_u32(this.addr(), TILE_3D_VTABLE);
    let tile = this.cast::<Tile>();
    if e.get(tile, Tile::uiFlags) & FLAG_MODEL_FILE != 0 {
        let file = e.get(this, Tile3D::strFilename);
        e.call(INTERFACE_RELEASE_MODEL_FILE, &args![file]);
    }
    if e.get(tile, Tile::uiFlags) & FLAG_RELEASED == 0 {
        e.call(TILE_RELEASE, &args![this]);
    }
    e.call(BS_STRING_SET, &args![this.byte_add(0x48), 0u32, 0u32]);
    e.call(BS_STRING_SET, &args![this.byte_add(0x40), 0u32, 0u32]);
    e.call(TILE_DESTRUCTOR, &args![this]);
}

// Translated from 00a20720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile3D::Init` (Xbox PDB): `Tile::Init` with the same three words;
/// brightness, red, green, blue and alpha the float at `01023cd8`, visible
/// 1 (each with flag word 1); the update bits 3; then enables
/// `Tile3D::xTimer` and ticks it with `GetTickCount()`.
pub fn tile_3d_init(
    e: &mut Engine,
    this: Ptr<Tile3D>,
    parent: Ptr<Tile>,
    name: Ptr,
    set_parent_arg: u32,
) {
    e.call(TILE_INIT, &args![this, parent, name, set_parent_arg]);
    for trait_id in [
        TRAIT_BRIGHTNESS,
        TRAIT_RED,
        TRAIT_GREEN,
        TRAIT_BLUE,
        TRAIT_ALPHA,
    ] {
        let full = x87_quiet_bits(e.global::<u32>(FULL));
        e.call(TILE_SET_FLOAT, &args![this, trait_id, full, 1u32]);
    }
    e.call(TILE_SET_FLOAT, &args![this, TRAIT_VISIBLE, 1.0f32, 1u32]);
    e.call(TILE_SET_NEEDS_UPDATE, &args![this, 3u32]);
    e.call(BS_TIMER_ENABLE, &args![TIMER]);
    let now = e.call(GET_TICK_COUNT, &args![]).u32();
    e.call(BS_TIMER_TICK, &args![TIMER, now]);
}

// Translated from 00a20800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile3D::MakeNode` (Xbox PDB): detaches and drops an old model (its
/// parent word +0x18 cleared), makes a new `NiNode` (null when the
/// allocation fails) the model, loads the NIF into it
/// ([`tile_3d_update_nif`]) and, when there is a model, attaches it to the
/// parent tile's nearest model (or the interface root) and adds the tile's
/// `Tile::Extra`. Returns the model.
// Translation incomplete: 00a20800 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn tile_3d_make_node(e: &mut Engine, this: Ptr<Tile3D>) -> Ptr {
    let tile = this.cast::<Tile>();
    let old = e.get(tile, Tile::spModel);
    if !old.is_null() {
        let old = e.get(tile, Tile::spModel);
        e.mem.set_u32(old.addr() + 0x18, 0);
        e.call(NI_POINTER_ASSIGN, &args![this.byte_add(0x2c), 0u32]);
    }
    let block = e.call(NI_NEW, &args![0xacu32]).u32();
    let node = if block == 0 {
        0
    } else {
        e.call(NI_NODE_CONSTRUCTOR, &args![block, 0u32]).u32()
    };
    e.call(NI_POINTER_ASSIGN, &args![this.byte_add(0x2c), node]);
    tile_3d_update_nif(e, this);
    if !e.get(tile, Tile::spModel).is_null() {
        let parent_tile = e.get(tile, Tile::pParent);
        let mut parent = e.call(TILE_GET_PARENT_MODEL, &args![parent_tile]).u32();
        if parent == 0 {
            parent = e.call(INTERFACE_GET_INTERFACE_ROOT, &args![]).u32();
        }
        let model = e.get(tile, Tile::spModel);
        e.vcall(parent, NODE_ATTACH_CHILD_SLOT, &args![model, 1u32]);
        let block = e.call(NI_NEW, &args![0x14u32]).u32();
        let extra = if block == 0 {
            0
        } else {
            let model = e.get(tile, Tile::spModel);
            e.call(TILE_EXTRA_CONSTRUCTOR, &args![block, this, model])
                .u32()
        };
        let model = e.get(tile, Tile::spModel);
        e.call(NI_OBJECT_NET_ADD_EXTRA_DATA, &args![model, extra]);
    }
    e.get(tile, Tile::spModel)
}

// Translated from 00a20980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile3D::UpdateNIF` (Xbox PDB): with a file name trait, writes the
/// model path into `strFilename` (the name itself when it contains
/// `"\Data"`, else `"%s\%s"` of the interface's model folder and the name)
/// and loads it (`Interface::LoadModelFile`). No model: the tile's model is
/// dropped. Otherwise the model-file flag (`0x20000`) follows the loader's
/// answer, a model that already has a parent is copied
/// (`CopyOrDeepCopyNode`), its controller manager is found, the tile's
/// node loses its children and the model its properties, which are
/// replaced by tile shader properties; the model is attached to the tile's
/// node, the locus set to 1, the node hidden or shown, its properties
/// updated, the NIF-file update bit cleared when set, the updates 9
/// queued, the interface told it needs an update and its temporary model
/// cleared.
pub fn tile_3d_update_nif(e: &mut Engine, this: Ptr<Tile3D>) {
    let tile = this.cast::<Tile>();
    let name = e.call(TILE_GET_STRING, &args![this, TRAIT_FILENAME]).u32();
    if name == 0 {
        return;
    }
    let file = this.byte_add(0x48);
    if e.call(STRSTR, &args![name, DATA_FOLDER]).u32() != 0 {
        e.call(BS_STRING_FORMAT, &args![file, FORMAT_ONE, name]);
    } else {
        let folder = e.call(INTERFACE_MODEL_FOLDER, &args![name]).u32();
        // `00707860` is cdecl and leaves its argument on the stack, where
        // it becomes the format's second string.
        e.call(BS_STRING_FORMAT, &args![file, FORMAT_TWO, folder, name]);
    }
    let model = e.with_stack(4, |e, from_file| {
        e.mem.set_u8(from_file.addr(), 0);
        let path = e.get(this, Tile3D::strFilename);
        let model = e
            .call(INTERFACE_LOAD_MODEL_FILE, &args![path, from_file])
            .u32();
        (model, e.mem.u8(from_file.addr()) != 0)
    });
    let (mut model, from_file) = model;
    if model == 0 {
        e.call(NI_POINTER_ASSIGN, &args![this.byte_add(0x2c), 0u32]);
        return;
    }
    let flags = e.get(tile, Tile::uiFlags);
    if from_file {
        e.set(tile, Tile::uiFlags, flags | FLAG_MODEL_FILE);
    } else {
        e.set(tile, Tile::uiFlags, flags & !FLAG_MODEL_FILE);
    }
    // `NiAVObject::m_pkParent` (+0x18).
    if e.mem.u32(model + 0x18) != 0 {
        model = e
            .call(INTERFACE_COPY_OR_DEEP_COPY_NODE, &args![model])
            .u32();
    }
    tile_3d_find_manager_node(e, this, Ptr::new(model));
    let node = e.get(tile, Tile::spModel);
    e.call(CHILDREN_REMOVE_ALL, &args![node.addr() + 0x9c]);
    tile_3d_remove_properties(e, this, Ptr::new(model));
    tile_3d_setup_shader_properties_recurse(e, this, Ptr::new(model));
    let node = e.get(tile, Tile::spModel);
    e.vcall(node.addr(), NODE_ATTACH_CHILD_SLOT, &args![model, 1u32]);
    e.call(TILE_SET_FLOAT, &args![this, TRAIT_LOCUS, 1.0f32, 1u32]);
    show_or_hide(e, this);
    let node = e.get(tile, Tile::spModel);
    e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![node]);
    let flags = e.get(tile, Tile::uiFlags);
    if flags & UPDATE_NIF_FILE != 0 {
        e.set(tile, Tile::uiFlags, flags ^ UPDATE_NIF_FILE);
    }
    e.call(TILE_ADD_NEEDS_UPDATE, &args![this, 9u32]);
    e.call(INTERFACE_SET_NEED_TO_UPDATE, &args![1u32]);
    e.call(INTERFACE_CLEAR_TEMP_MODEL, &args![]);
}

// Translated from 00a20ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile3D::RemoveProperties` (Xbox PDB): removes `node`'s shade property
/// (type 2 here), compacts its children array (+0x9C) and, for each child
/// up to the array's count (+0xA8), removes the child's property 2 and
/// recurses into children that are nodes.
pub fn tile_3d_remove_properties(e: &mut Engine, this: Ptr<Tile3D>, node: Ptr) {
    e.call(NI_AV_OBJECT_REMOVE_PROPERTY, &args![node, 2u32]);
    e.call(CHILDREN_COMPACT, &args![node.addr() + 0x9c]);
    e.call(CHILDREN_UPDATE_SIZE, &args![node.addr() + 0x9c]);
    let mut i = 0u32;
    while i < u32::from(e.mem.u16(node.addr() + 0xa8)) {
        let child = child_at(e, node.addr(), i);
        if child != 0 {
            e.call(NI_AV_OBJECT_REMOVE_PROPERTY, &args![child, 2u32]);
            let sub = e.vcall(child, IS_NODE_SLOT, &args![]).u32();
            if sub != 0 {
                tile_3d_remove_properties(e, this, Ptr::new(sub));
            }
        }
        i += 1;
    }
}

// Translated from 00a20c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile3D::PostParse` (Xbox PDB): a new string trait empties the current
/// sequence name. Returns 0; the value and the third word are not read.
pub fn tile_3d_post_parse(
    e: &mut Engine,
    this: Ptr<Tile3D>,
    trait_id: i32,
    _unused_2: u32,
    _unused_3: u32,
) -> u32 {
    if trait_id == TRAIT_STRING {
        e.call(
            BS_STRING_SET,
            &args![this.byte_add(0x40), EMPTY_STRING, 0u32],
        );
    }
    0
}

// Translated from 00a20c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile3D::FindManagerNode` (Xbox PDB): keeps `node`'s
/// `NiControllerManager` controller as the tile's animation manager and
/// answers whether it has one (the manager is left as it was when not).
pub fn tile_3d_find_manager_node(e: &mut Engine, this: Ptr<Tile3D>, node: Ptr) -> bool {
    let manager = e
        .call(
            NI_OBJECT_NET_GET_CONTROLLER,
            &args![node, NI_CONTROLLER_MANAGER_RTTI],
        )
        .ptr();
    if manager.is_null() {
        false
    } else {
        e.set(this, Tile3D::pAniManager, manager);
        true
    }
}

// Translated from 00a20cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts the animation sequence `name` (no PDB name): with a model, a
/// manager and a name, looks the sequence up in the manager's map (+0x50).
/// When found: the current sequence (when there is one) is finished at
/// time 1 ([`fn_00a20ed0`]) if its state is 1 and deactivated (slot
/// `+0x8C` with 0.0 and 0); the found one is deactivated when its state is
/// not 0 and activated (`00a34f20` with `(0, 0, 1.0, 0.0, 0, 0)`); on
/// success the manager's active flag (word +8, bit 8) is set and it becomes
/// the current sequence. Answers whether it started.
// Translation incomplete: 00a20cc0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn fn_00a20cc0(e: &mut Engine, this: Ptr<Tile3D>, name: Ptr) -> bool {
    let model = e.get(this.cast::<Tile>(), Tile::spModel);
    if model.is_null() || e.get(this, Tile3D::pAniManager).is_null() || name.is_null() {
        return false;
    }
    let handle = if name.is_null() {
        e.global::<u32>(NI_FIXED_STRING_EMPTY)
    } else {
        e.call(NI_FIXED_STRING_CREATE, &args![name]).u32()
    };
    let sequence = e.with_stack(4, |e, found| {
        let manager = e.get(this, Tile3D::pAniManager);
        let ok = e
            .call(
                SEQUENCE_MAP_GET,
                &args![manager.addr() + 0x50, handle, found],
            )
            .bool();
        if ok {
            e.mem.u32(found.addr())
        } else {
            0
        }
    });
    if handle != e.global::<u32>(NI_FIXED_STRING_EMPTY) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
    if sequence == 0 {
        return false;
    }
    let mut started = false;
    let current = e.get(this, Tile3D::pCurrentSequence);
    if !current.is_null() {
        // `NiControllerSequence::m_eState` (+0x44).
        if e.mem.u32(current.addr() + 0x44) == 1 {
            fn_00a20ed0(e, this, 1.0, true);
        }
        let current = e.get(this, Tile3D::pCurrentSequence);
        e.vcall(
            current.addr(),
            SEQUENCE_DEACTIVATE_SLOT,
            &args![0.0f32, 0u32],
        );
    }
    if !started {
        if e.mem.u32(sequence + 0x44) != 0 {
            e.vcall(sequence, SEQUENCE_DEACTIVATE_SLOT, &args![0.0f32, 0u32]);
        }
        started = e
            .call(
                SEQUENCE_ACTIVATE,
                &args![sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32, 0u32],
            )
            .bool();
    }
    if started {
        set_manager_active(e, this, std::hint::black_box(true));
        e.set(this, Tile3D::pCurrentSequence, Ptr::new(sequence));
        return true;
    }
    false
}

// Translated from 00a20ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the tile's animation (no PDB name): when the animation trait
/// (`0xFD2`) differs from the current name (case-insensitive `StrCmp`),
/// takes it and starts it ([`fn_00a20cc0`]), emptying the name when that
/// fails. Then, with a model, a manager and a visible tile (hiding the
/// model were it not visible) and a current sequence, updates the model
/// with an `NiUpdateData` time: 0 for a sequence that has not run (last
/// time `-FLT_MAX`); for `time == -1`, the last time plus the step at
/// `011f356c` wrapped below the end key time, after ticking the timer, and
/// a finished sequence (`00707970`) is deactivated and dropped and the
/// manager's active flag cleared; `time` times the end key time clamped to
/// `0 ..= end` when `force` or `time > 1`; `time` otherwise. Answers
/// whether the model was updated.
pub fn fn_00a20ed0(e: &mut Engine, this: Ptr<Tile3D>, time: f32, force: bool) -> bool {
    let tile = this.cast::<Tile>();
    let animation = e.call(TILE_GET_STRING, &args![this, TRAIT_ANIMATION]).u32();
    let current_name = this.byte_add(0x40);
    if e.get(this, Tile3D::strCurrentName) != 0 || animation != 0 {
        let differs = e
            .call(BS_STRING_STR_CMP, &args![current_name, animation, 1u32])
            .u32()
            != 0;
        if differs {
            e.call(BS_STRING_SET, &args![current_name, animation, 0u32]);
            let length = if e.get(this, Tile3D::strCurrentNameLength) == 0xffff {
                let text = e.get(this, Tile3D::strCurrentName);
                e.call(STRLEN, &args![text]).u32()
            } else {
                u32::from(e.get(this, Tile3D::strCurrentNameLength))
            };
            if length != 0 {
                let text = Ptr::new(e.get(this, Tile3D::strCurrentName));
                if !fn_00a20cc0(e, this, text) {
                    e.call(BS_STRING_SET, &args![current_name, 0u32, 0u32]);
                }
            }
        }
    }
    if e.get(tile, Tile::spModel).is_null()
        || e.get(this, Tile3D::pAniManager).is_null()
        || !e.call(TILE_IS_TRUE, &args![this, TRAIT_VISIBLE]).bool()
    {
        return false;
    }
    if !e.call(TILE_IS_TRUE, &args![this, TRAIT_VISIBLE]).bool() {
        let model = e.get(tile, Tile::spModel);
        let flags = e.mem.u32(model.addr() + 0x30);
        if std::hint::black_box(true) {
            e.mem.set_u32(model.addr() + 0x30, flags | 1);
        } else {
            e.mem.set_u32(model.addr() + 0x30, flags & !1);
        }
    }
    let current = e.get(this, Tile3D::pCurrentSequence);
    if current.is_null() {
        return false;
    }
    // `NiControllerSequence::m_fLastTime` (+0x34) and `m_fEndKeyTime`
    // (+0x30).
    let last = x87_quiet_bits(e.mem.u32(current.addr() + 0x34));
    let never_run = -x87_load32(e.global::<u32>(FLOAT_MAX));
    if x87_load32(last) == never_run {
        update_model(e, this, 0.0f32.to_bits());
    } else if x87_load32(time.to_bits()) == e.global::<f64>(MINUS_ONE) {
        let current = e.get(this, Tile3D::pCurrentSequence);
        let last = x87_quiet_bits(e.mem.u32(current.addr() + 0x34));
        let step = x87_quiet_bits(e.global::<u32>(TIME_STEP));
        let mut wrapped = x87_store32(x87_add(x87_load32(last), x87_m32(step)));
        let current = e.get(this, Tile3D::pCurrentSequence);
        let end = x87_quiet_bits(e.mem.u32(current.addr() + 0x30));
        while x87_load32(end) < x87_load32(wrapped) {
            wrapped = x87_store32(x87_sub(x87_load32(wrapped), x87_m32(end)));
        }
        let now = e.call(GET_TICK_COUNT, &args![]).u32();
        e.call(BS_TIMER_TICK, &args![TIMER, now]);
        update_model(e, this, x87_quiet_bits(wrapped));
        let current = e.get(this, Tile3D::pCurrentSequence);
        if e.call(INTERFACE_SEQUENCE_DONE, &args![current]).bool() {
            let current = e.get(this, Tile3D::pCurrentSequence);
            e.vcall(
                current.addr(),
                SEQUENCE_DEACTIVATE_SLOT,
                &args![0.0f32, 0u32],
            );
            e.set(this, Tile3D::pCurrentSequence, Ptr::NULL);
            set_manager_active(e, this, std::hint::black_box(false));
        }
    } else if force || x87_load32(time.to_bits()) > e.global::<f64>(ONE) {
        let current = e.get(this, Tile3D::pCurrentSequence);
        let end = x87_quiet_bits(e.mem.u32(current.addr() + 0x30));
        let scaled = x87_store32(x87_mul(x87_load32(time.to_bits()), x87_m32(end)));
        let clamped = if x87_load32(scaled) < 0.0 {
            0.0f32.to_bits()
        } else {
            x87_quiet_bits(scaled)
        };
        let result = if x87_load32(clamped) > x87_load32(end) {
            x87_quiet_bits(end)
        } else {
            x87_quiet_bits(clamped)
        };
        update_model(e, this, x87_quiet_bits(result));
    } else {
        update_model(e, this, x87_quiet_bits(time.to_bits()));
    }
    true
}

// Translated from 00a21340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile3D::SetupShaderPropertiesRecurse` (Xbox PDB): for each child of
/// `node` (up to the array's count at +0xA8) that is geometry with a shade
/// property (type 3) that is not a tile shader property (type `0x13` at
/// +0x1C): replaces it by a new `TileShaderProperty` (null when the
/// allocation fails) whose tile texture is the texturing property's (type
/// 5) first map's texture, and gives the geometry the shader
/// `BSShaderManager::GetShader(0x20)` (+0xC0). Recurses into children that
/// are nodes.
// Translation incomplete: 00a21340 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn tile_3d_setup_shader_properties_recurse(e: &mut Engine, this: Ptr<Tile3D>, node: Ptr) {
    if node.is_null() {
        return;
    }
    let mut i = 0u32;
    while i < u32::from(e.mem.u16(node.addr() + 0xa8)) {
        let child = child_at(e, node.addr(), i);
        if child != 0 {
            let child = child_at(e, node.addr(), i);
            let geometry = e.vcall(child, IS_GEOMETRY_SLOT, &args![]).u32();
            if geometry != 0 {
                let property = e
                    .call(NI_AV_OBJECT_GET_PROPERTY, &args![geometry, SHADE_PROPERTY])
                    .u32();
                if property != 0 {
                    let is_tile_shader = if property == 0 {
                        false
                    } else {
                        e.mem.u32(property + 0x1c) == TILE_SHADER_TYPE
                    };
                    if !is_tile_shader {
                        let block = e.call(NI_NEW, &args![0xb0u32]).u32();
                        let tile_property = if block == 0 {
                            0
                        } else {
                            e.call(TILE_SHADER_PROPERTY_CONSTRUCTOR, &args![block, 0u32])
                                .u32()
                        };
                        e.call(
                            NI_AV_OBJECT_REMOVE_PROPERTY,
                            &args![geometry, SHADE_PROPERTY],
                        );
                        e.call(
                            NI_AV_OBJECT_ATTACH_PROPERTY,
                            &args![geometry, tile_property],
                        );
                        let texturing = e
                            .call(
                                NI_AV_OBJECT_GET_PROPERTY,
                                &args![geometry, TEXTURING_PROPERTY],
                            )
                            .u32();
                        if texturing != 0 {
                            // `NiTexturingProperty` maps (+0x20), map 0, its
                            // texture (+8).
                            let maps = e.mem.u32(texturing + 0x20);
                            let map = e.mem.u32(maps);
                            let texture = if map != 0 { e.mem.u32(map + 8) } else { 0 };
                            e.call(
                                TILE_SHADER_PROPERTY_SET_TILE_TEXTURE,
                                &args![tile_property, texture],
                            );
                        }
                        let shader = e.call(BS_SHADER_MANAGER_GET_SHADER, &args![0x20u32]).u32();
                        e.mem.set_u32(geometry + 0xc0, shader);
                    }
                }
            }
            let child = child_at(e, node.addr(), i);
            let sub = e.vcall(child, IS_NODE_SLOT, &args![]).u32();
            if sub != 0 {
                tile_3d_setup_shader_properties_recurse(e, this, Ptr::new(sub));
            }
        }
        i += 1;
    }
}

// Translated from 00a21570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile3D::SetAlphaAndColor` (Xbox PDB): for each child of `node` (up to
/// the array's size at +0xA6), passes the child as a node (`IsNode`, null
/// when it is not one) with `alpha` and `color` to the tile's own
/// `SetAlphaAndColor` (slot `+0x24`), and for geometry stores `alpha` at
/// +0x78 and the four colour words at +0x68 of its shade property (type
/// 3).
pub fn tile_3d_set_alpha_and_color(
    e: &mut Engine,
    this: Ptr<Tile3D>,
    node: Ptr,
    alpha: f32,
    color: Ptr,
) {
    if node.is_null() {
        return;
    }
    let mut i = 0u32;
    while i < u32::from(e.mem.u16(node.addr() + 0xa6)) {
        let child = child_at(e, node.addr(), i);
        if child != 0 {
            let sub = e.vcall(child, IS_NODE_SLOT, &args![]).u32();
            e.vcall(
                this.addr(),
                SET_ALPHA_AND_COLOR_SLOT,
                &args![sub, x87_quiet_bits(alpha.to_bits()), color],
            );
            let geometry = e.vcall(child, IS_GEOMETRY_SLOT, &args![]).u32();
            if geometry != 0 {
                let property = e
                    .call(NI_AV_OBJECT_GET_PROPERTY, &args![geometry, SHADE_PROPERTY])
                    .u32();
                e.mem
                    .set_u32(property.wrapping_add(0x78), x87_quiet_bits(alpha.to_bits()));
                for k in 0..4 {
                    let word = e.mem.u32(color.addr() + 4 * k);
                    e.mem.set_u32(property.wrapping_add(0x68 + 4 * k), word);
                }
            }
        }
        i += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2_0650, tile_3d_destructor(Ptr<Tile3D>)),
        entry!(0x00a2_0720, tile_3d_init(Ptr<Tile3D>, Ptr<Tile>, Ptr, u32)),
        entry!(0x00a2_0800, tile_3d_make_node(Ptr<Tile3D>) -> Ptr),
        entry!(0x00a2_0980, tile_3d_update_nif(Ptr<Tile3D>)),
        entry!(0x00a2_0ba0, tile_3d_remove_properties(Ptr<Tile3D>, Ptr)),
        entry!(
            0x00a2_0c50,
            tile_3d_post_parse(Ptr<Tile3D>, i32, u32, u32) -> u32
        ),
        entry!(
            0x00a2_0c80,
            tile_3d_find_manager_node(Ptr<Tile3D>, Ptr) -> bool
        ),
        entry!(0x00a2_0cc0, fn_00a20cc0(Ptr<Tile3D>, Ptr) -> bool),
        entry!(0x00a2_0ed0, fn_00a20ed0(Ptr<Tile3D>, f32, bool) -> bool),
        entry!(
            0x00a2_1340,
            tile_3d_setup_shader_properties_recurse(Ptr<Tile3D>, Ptr)
        ),
        entry!(
            0x00a2_1570,
            tile_3d_set_alpha_and_color(Ptr<Tile3D>, Ptr, f32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ATTACH: u32 = 0x0000_1dc0;
    const IS_NODE: u32 = 0x0000_1d0c;
    const IS_GEOMETRY: u32 = 0x0000_1d18;
    const DEACTIVATE: u32 = 0x0000_1d8c;
    const SET_ALPHA: u32 = 0x0000_1d24;
    const OBJECT_VTABLE: u32 = 0x0003_0000;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// A vtable for every test object: `IsNode` answers the object when
    /// its word +0x40 is 1, `IsGeometry` when it is 2.
    fn objects(e: &mut Engine) {
        let mut slots = vec![0u32; 0x38];
        slots[(IS_NODE_SLOT / 4) as usize] = IS_NODE;
        slots[(IS_GEOMETRY_SLOT / 4) as usize] = IS_GEOMETRY;
        slots[(NODE_ATTACH_CHILD_SLOT / 4) as usize] = ATTACH;
        slots[(SET_ALPHA_AND_COLOR_SLOT / 4) as usize] = SET_ALPHA;
        slots[(SEQUENCE_DEACTIVATE_SLOT / 4) as usize] = DEACTIVATE;
        e.put_vtable(OBJECT_VTABLE, &slots);
        e.register(IS_NODE, |e, a| Ret {
            eax: if e.mem.u32(a[0] + 0x40) == 1 { a[0] } else { 0 },
            ..Ret::default()
        });
        e.register(IS_GEOMETRY, |e, a| Ret {
            eax: if e.mem.u32(a[0] + 0x40) == 2 { a[0] } else { 0 },
            ..Ret::default()
        });
        quiet(e, &[ATTACH, SET_ALPHA, DEACTIVATE]);
    }

    /// An object of kind `kind` (1 node, 2 geometry) with `children`.
    fn object(e: &mut Engine, kind: u32, children: &[u32]) -> u32 {
        let object = e.mem.alloc(0xd0);
        e.mem.set_u32(object, OBJECT_VTABLE);
        e.mem.set_u32(object + 0x40, kind);
        let array = e.mem.alloc(4 * children.len() as u32 + 4);
        for (i, child) in children.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *child);
        }
        e.mem.set_u32(object + 0xa0, array);
        e.mem.set_u16(object + 0xa6, children.len() as u16);
        e.mem.set_u16(object + 0xa8, children.len() as u16);
        object
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let address = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(address, s);
        address
    }

    #[test]
    fn tile_3d_destructor_releases_the_model_file_and_the_strings() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                INTERFACE_RELEASE_MODEL_FILE,
                TILE_RELEASE,
                BS_STRING_SET,
                TILE_DESTRUCTOR,
            ],
        );
        let this = e.new_object::<Tile3D>();
        e.set(this.cast::<Tile>(), Tile::uiFlags, FLAG_MODEL_FILE);
        e.set(this, Tile3D::strFilename, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x00a2_0650, &args![this]);
        let t = this.addr();
        assert_eq!(e.mem.u32(t), TILE_3D_VTABLE);
        assert_eq!(
            calls_to(&e, INTERFACE_RELEASE_MODEL_FILE),
            vec![vec![0x6000]]
        );
        assert_eq!(calls_to(&e, TILE_RELEASE), vec![vec![t]]);
        assert_eq!(
            calls_to(&e, BS_STRING_SET),
            vec![vec![t + 0x48, 0, 0], vec![t + 0x40, 0, 0]]
        );
        assert_eq!(calls_to(&e, TILE_DESTRUCTOR), vec![vec![t]]);

        e.set(this.cast::<Tile>(), Tile::uiFlags, FLAG_RELEASED);
        e.call_log = Some(vec![]);
        e.call(0x00a2_0650, &args![this]);
        assert!(calls_to(&e, INTERFACE_RELEASE_MODEL_FILE).is_empty());
        assert!(calls_to(&e, TILE_RELEASE).is_empty());
    }

    #[test]
    fn tile_3d_init_sets_the_traits_and_starts_the_timer() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                TILE_INIT,
                TILE_SET_FLOAT,
                TILE_SET_NEEDS_UPDATE,
                BS_TIMER_ENABLE,
                BS_TIMER_TICK,
            ],
        );
        e.register(GET_TICK_COUNT, |_, _| Ret {
            eax: 777,
            ..Ret::default()
        });
        e.map(FULL, 4);
        e.set_global(FULL, 255.0f32);
        e.call_log = Some(vec![]);
        e.call(0x00a2_0720, &args![0x5000u32, 1u32, 2u32, 3u32]);
        assert_eq!(calls_to(&e, TILE_INIT), vec![vec![0x5000, 1, 2, 3]]);
        let set = calls_to(&e, TILE_SET_FLOAT);
        assert_eq!(set.len(), 6);
        assert_eq!(set[0], vec![0x5000, 0xff5, 255.0f32.to_bits(), 1]);
        assert_eq!(set[5], vec![0x5000, 0xfa3, 1.0f32.to_bits(), 1]);
        assert_eq!(calls_to(&e, TILE_SET_NEEDS_UPDATE), vec![vec![0x5000, 3]]);
        assert_eq!(calls_to(&e, BS_TIMER_ENABLE), vec![vec![TIMER]]);
        assert_eq!(calls_to(&e, BS_TIMER_TICK), vec![vec![TIMER, 777]]);
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        objects(&mut e);
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(NI_NODE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], OBJECT_VTABLE);
            e.mem.set_u32(a[0] + 0x40, 1);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e
    }

    #[test]
    fn tile_3d_make_node_replaces_the_model_and_attaches_it() {
        let mut e = engine();
        e.register(TILE_GET_STRING, |_, _| Ret::default());
        e.register(TILE_GET_PARENT_MODEL, |_, _| Ret::default());
        let root = object(&mut e, 1, &[]);
        e.register_double(INTERFACE_GET_INTERFACE_ROOT, move |_, _| Ret {
            eax: root,
            ..Ret::default()
        });
        e.register(TILE_EXTRA_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        quiet(&mut e, &[NI_OBJECT_NET_ADD_EXTRA_DATA]);
        let this = e.new_object::<Tile3D>();
        let old = object(&mut e, 1, &[]);
        e.mem.set_u32(old + 0x18, 0x1234);
        e.set(this.cast::<Tile>(), Tile::spModel, Ptr::new(old));
        e.set(this.cast::<Tile>(), Tile::pParent, Ptr::new(0x7000));
        e.call_log = Some(vec![]);
        let model = e.call(0x00a2_0800, &args![this]).u32();
        assert_ne!(model, old);
        assert_eq!(e.mem.u32(old + 0x18), 0);
        assert_eq!(calls_to(&e, TILE_GET_PARENT_MODEL), vec![vec![0x7000]]);
        assert_eq!(calls_to(&e, ATTACH), vec![vec![root, model, 1]]);
        let extra = calls_to(&e, NI_OBJECT_NET_ADD_EXTRA_DATA);
        assert_eq!(extra[0][0], model);
        assert_ne!(extra[0][1], 0);
    }

    fn nif_engine(path: u32, loaded: u32) -> Engine {
        let mut e = engine();
        e.register_double(TILE_GET_STRING, move |_, _| Ret {
            eax: path,
            ..Ret::default()
        });
        e.register(STRSTR, |e, a| Ret {
            eax: e.mem.cstr(a[0]).windows(5).any(|w| w == b"\\Data") as u32,
            ..Ret::default()
        });
        e.register(INTERFACE_MODEL_FOLDER, |_, _| Ret {
            eax: 0x0000_6100,
            ..Ret::default()
        });
        e.register_double(INTERFACE_LOAD_MODEL_FILE, move |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret {
                eax: loaded,
                ..Ret::default()
            }
        });
        e.register(INTERFACE_COPY_OR_DEEP_COPY_NODE, |e, a| {
            let copy = e.mem.alloc(0xd0);
            let words = e.mem.bytes(a[0], 0xd0);
            e.mem.write(copy, &words);
            e.mem.set_u32(copy + 0x18, 0);
            Ret {
                eax: copy,
                ..Ret::default()
            }
        });
        e.register(TILE_IS_TRUE, |_, _| Ret::default());
        quiet(
            &mut e,
            &[
                BS_STRING_FORMAT,
                NI_OBJECT_NET_GET_CONTROLLER,
                CHILDREN_REMOVE_ALL,
                CHILDREN_COMPACT,
                CHILDREN_UPDATE_SIZE,
                NI_AV_OBJECT_REMOVE_PROPERTY,
                TILE_SET_FLOAT,
                NI_AV_OBJECT_UPDATE_PROPERTIES,
                TILE_ADD_NEEDS_UPDATE,
                INTERFACE_SET_NEED_TO_UPDATE,
                INTERFACE_CLEAR_TEMP_MODEL,
            ],
        );
        e
    }

    #[test]
    fn tile_3d_update_nif_needs_a_file_and_a_model() {
        let mut e = nif_engine(0, 0);
        let this = e.new_object::<Tile3D>();
        e.call_log = Some(vec![]);
        e.call(0x00a2_0980, &args![this]);
        assert!(calls_to(&e, BS_STRING_FORMAT).is_empty());

        let mut e = nif_engine(0, 0);
        let path = text(&mut e, b"interface\\a.nif");
        e.register_double(TILE_GET_STRING, move |_, _| Ret {
            eax: path,
            ..Ret::default()
        });
        let this = e.new_object::<Tile3D>();
        e.set(this.cast::<Tile>(), Tile::spModel, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        e.call(0x00a2_0980, &args![this]);
        assert_eq!(
            calls_to(&e, BS_STRING_FORMAT),
            vec![vec![this.addr() + 0x48, FORMAT_TWO, 0x6100, path]]
        );
        // Not loaded: the model is dropped.
        assert!(e.get(this.cast::<Tile>(), Tile::spModel).is_null());
    }

    #[test]
    fn tile_3d_update_nif_copies_a_parented_model() {
        let mut e = nif_engine(0, 0);
        let loaded = object(&mut e, 1, &[]);
        e.mem.set_u32(loaded + 0x18, 0x9999);
        let path = text(&mut e, b"C:\\Data\\b.nif");
        e.register_double(TILE_GET_STRING, move |_, _| Ret {
            eax: path,
            ..Ret::default()
        });
        e.register_double(INTERFACE_LOAD_MODEL_FILE, move |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret {
                eax: loaded,
                ..Ret::default()
            }
        });
        let this = e.new_object::<Tile3D>();
        let node = object(&mut e, 1, &[]);
        e.set(this.cast::<Tile>(), Tile::spModel, Ptr::new(node));
        e.set(this.cast::<Tile>(), Tile::uiFlags, UPDATE_NIF_FILE);
        e.call_log = Some(vec![]);
        e.call(0x00a2_0980, &args![this]);
        assert_eq!(
            calls_to(&e, BS_STRING_FORMAT),
            vec![vec![this.addr() + 0x48, FORMAT_ONE, path]]
        );
        let copy = calls_to(&e, ATTACH)[0][1];
        assert_ne!(copy, loaded);
        assert_eq!(calls_to(&e, ATTACH), vec![vec![node, copy, 1]]);
        assert_eq!(calls_to(&e, CHILDREN_REMOVE_ALL), vec![vec![node + 0x9c]]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_NET_GET_CONTROLLER),
            vec![vec![copy, NI_CONTROLLER_MANAGER_RTTI]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![this.addr(), 0xfa8, 1.0f32.to_bits(), 1]]
        );
        let flags = e.get(this.cast::<Tile>(), Tile::uiFlags);
        assert_eq!(flags, FLAG_MODEL_FILE);
        // Hidden (the visible trait is false): bit 0 set.
        assert_eq!(e.mem.u32(node + 0x30) & 1, 1);
        assert_eq!(
            calls_to(&e, TILE_ADD_NEEDS_UPDATE),
            vec![vec![this.addr(), 9]]
        );
        assert_eq!(calls_to(&e, INTERFACE_CLEAR_TEMP_MODEL).len(), 1);
    }

    #[test]
    fn tile_3d_remove_properties_walks_the_tree() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                NI_AV_OBJECT_REMOVE_PROPERTY,
                CHILDREN_COMPACT,
                CHILDREN_UPDATE_SIZE,
            ],
        );
        let leaf = object(&mut e, 2, &[]);
        let inner = object(&mut e, 1, &[leaf]);
        let root = object(&mut e, 1, &[0, inner, leaf]);
        e.call_log = Some(vec![]);
        e.call(0x00a2_0ba0, &args![0x5000u32, root]);
        assert_eq!(
            calls_to(&e, NI_AV_OBJECT_REMOVE_PROPERTY),
            vec![
                vec![root, 2],
                vec![inner, 2],
                vec![inner, 2],
                vec![leaf, 2],
                vec![leaf, 2]
            ]
        );
        assert_eq!(
            calls_to(&e, CHILDREN_COMPACT),
            vec![vec![root + 0x9c], vec![inner + 0x9c]]
        );
    }

    #[test]
    fn tile_3d_post_parse_empties_the_name_for_a_new_string() {
        let mut e = Engine::new();
        quiet(&mut e, &[BS_STRING_SET]);
        e.call_log = Some(vec![]);
        let result = e.call(0x00a2_0c50, &args![0x5000u32, 0xfc4u32, 0u32, 0u32]);
        assert_eq!(result.u32(), 0);
        let result = e.call(0x00a2_0c50, &args![0x5000u32, 0xfc5u32, 0u32, 0u32]);
        assert_eq!(result.u32(), 0);
        assert_eq!(
            calls_to(&e, BS_STRING_SET),
            vec![vec![0x5040, EMPTY_STRING, 0]]
        );
    }

    #[test]
    fn tile_3d_find_manager_node_keeps_the_manager() {
        let mut e = Engine::new();
        let answer = Rc::new(RefCell::new(0u32));
        let seen = answer.clone();
        e.register_double(NI_OBJECT_NET_GET_CONTROLLER, move |_, _| Ret {
            eax: *seen.borrow(),
            ..Ret::default()
        });
        let this = e.new_object::<Tile3D>();
        e.set(this, Tile3D::pAniManager, Ptr::new(0x4444));
        assert!(!e.call(0x00a2_0c80, &args![this, 0x6000u32]).bool());
        assert_eq!(e.get(this, Tile3D::pAniManager).addr(), 0x4444);
        *answer.borrow_mut() = 0x7777;
        assert!(e.call(0x00a2_0c80, &args![this, 0x6000u32]).bool());
        assert_eq!(e.get(this, Tile3D::pAniManager).addr(), 0x7777);
    }

    /// A tile with a model and a manager.
    fn animated(e: &mut Engine) -> (Ptr<Tile3D>, u32) {
        let this = e.new_object::<Tile3D>();
        let model = object(e, 1, &[]);
        e.set(this.cast::<Tile>(), Tile::spModel, Ptr::new(model));
        let manager = e.mem.alloc(0x60);
        e.set(this, Tile3D::pAniManager, Ptr::new(manager));
        (this, manager)
    }

    fn sequence(e: &mut Engine, state: u32, end: f32, last: f32) -> u32 {
        let sequence = e.mem.alloc(0x80);
        e.mem.set_u32(sequence, OBJECT_VTABLE);
        e.mem.set_f32(sequence + 0x30, end);
        e.mem.set_f32(sequence + 0x34, last);
        e.mem.set_u32(sequence + 0x44, state);
        sequence
    }

    fn names(e: &mut Engine) {
        e.map(NI_FIXED_STRING_EMPTY, 4);
        e.register(NI_FIXED_STRING_CREATE, |_, _| Ret {
            eax: 0x9008,
            ..Ret::default()
        });
        quiet(e, &[INTERLOCKED_DECREMENT]);
    }

    #[test]
    fn fn_00a20cc0_starts_a_found_sequence() {
        let mut e = engine();
        names(&mut e);
        let (this, manager) = animated(&mut e);
        let found = sequence(&mut e, 2, 1.0, 0.0);
        let current = sequence(&mut e, 0, 1.0, 0.0);
        e.set(this, Tile3D::pCurrentSequence, Ptr::new(current));
        e.register_double(SEQUENCE_MAP_GET, move |e, a| {
            e.mem.set_u32(a[2], found);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register(SEQUENCE_ACTIVATE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a2_0cc0, &args![this, 0x6000u32]).bool());
        assert_eq!(
            calls_to(&e, SEQUENCE_MAP_GET)[0][..2],
            [manager + 0x50, 0x9008]
        );
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT), vec![vec![0x9000]]);
        assert_eq!(
            calls_to(&e, DEACTIVATE),
            vec![vec![current, 0, 0], vec![found, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, SEQUENCE_ACTIVATE),
            vec![vec![found, 0, 0, 1.0f32.to_bits(), 0, 0, 0]]
        );
        assert_eq!(e.mem.u16(manager + 8), 8);
        assert_eq!(e.get(this, Tile3D::pCurrentSequence).addr(), found);
    }

    #[test]
    fn fn_00a20cc0_fails_without_a_name_or_a_sequence() {
        let mut e = engine();
        names(&mut e);
        e.register(SEQUENCE_MAP_GET, |_, _| Ret::default());
        let (this, _) = animated(&mut e);
        assert!(!e.call(0x00a2_0cc0, &args![this, 0u32]).bool());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a2_0cc0, &args![this, 0x6000u32]).bool());
        assert!(calls_to(&e, SEQUENCE_ACTIVATE).is_empty());
        // The activation failing: not started.
        let found = sequence(&mut e, 0, 1.0, 0.0);
        e.register_double(SEQUENCE_MAP_GET, move |e, a| {
            e.mem.set_u32(a[2], found);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register(SEQUENCE_ACTIVATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a2_0cc0, &args![this, 0x6000u32]).bool());
        assert!(calls_to(&e, DEACTIVATE).is_empty());
        assert!(e.get(this, Tile3D::pCurrentSequence).is_null());
    }

    fn update_engine() -> Engine {
        let mut e = engine();
        for (address, value) in [(MINUS_ONE, -1.0f64), (ONE, 1.0)] {
            e.map(address, 8);
            e.set_global(address, value);
        }
        e.map(FLOAT_MAX, 4);
        e.set_global(FLOAT_MAX, f32::MAX);
        e.map(TIME_STEP, 4);
        e.set_global(TIME_STEP, 0.75f32);
        e.register(TILE_GET_STRING, |_, _| Ret::default());
        e.register(TILE_IS_TRUE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(GET_TICK_COUNT, |_, _| Ret {
            eax: 55,
            ..Ret::default()
        });
        quiet(&mut e, &[BS_TIMER_TICK, NI_AV_OBJECT_UPDATE]);
        e
    }

    #[test]
    fn fn_00a20ed0_updates_with_the_sequence_time() {
        let mut e = update_engine();
        let times = Rc::new(RefCell::new(vec![]));
        let seen = times.clone();
        e.register_double(NI_AV_OBJECT_UPDATE, move |e, a| {
            seen.borrow_mut()
                .push((e.mem.f32(a[1]), e.mem.bytes(a[1] + 4, 5)));
            Ret::default()
        });
        e.register(INTERFACE_SEQUENCE_DONE, |_, _| Ret::default());
        let (this, _) = animated(&mut e);
        let current = sequence(&mut e, 1, 2.0, -f32::MAX);
        e.set(this, Tile3D::pCurrentSequence, Ptr::new(current));
        let run = |e: &mut Engine, time: f32, force: bool| {
            e.call(0x00a2_0ed0, &args![this, time, force]).bool()
        };
        assert!(run(&mut e, 0.5, false));
        e.mem.set_f32(current + 0x34, 3.0);
        assert!(run(&mut e, -1.0, false));
        assert!(run(&mut e, 0.25, true));
        assert!(run(&mut e, 2.0, false));
        assert!(run(&mut e, -0.5, true));
        assert!(run(&mut e, 0.5, false));
        assert!(run(&mut e, f32::NAN, false));
        let times = times.borrow();
        let values: Vec<f32> = times.iter().map(|(t, _)| *t).collect();
        // Never run: 0. -1: 3 + 0.75 wrapped below 2. Forced: 0.25 * 2;
        // above 1: clamped to 2; negative: 0; plain: the time.
        assert_eq!(values[..6], [0.0, 1.75, 0.5, 2.0, 0.0, 0.5]);
        assert!(values[6].is_nan());
        assert!(times.iter().all(|(_, flags)| flags == &[1, 0, 0, 0, 0]));
    }

    #[test]
    fn fn_00a20ed0_drops_a_finished_sequence_and_retries_a_new_name() {
        let mut e = update_engine();
        e.register(INTERFACE_SEQUENCE_DONE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let (this, manager) = animated(&mut e);
        e.mem.set_u16(manager + 8, 0xffff);
        let current = sequence(&mut e, 1, 2.0, 0.5);
        e.set(this, Tile3D::pCurrentSequence, Ptr::new(current));
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a2_0ed0, &args![this, -1.0f32, false]).bool());
        assert_eq!(calls_to(&e, BS_TIMER_TICK), vec![vec![TIMER, 55]]);
        assert_eq!(calls_to(&e, DEACTIVATE), vec![vec![current, 0, 0]]);
        assert!(e.get(this, Tile3D::pCurrentSequence).is_null());
        assert_eq!(e.mem.u16(manager + 8), 0xfff7);
        // No current sequence now: nothing to update.
        assert!(!e.call(0x00a2_0ed0, &args![this, 0.5f32, false]).bool());

        // A new animation name that cannot be started is emptied again.
        let name = text(&mut e, b"Idle");
        e.register_double(TILE_GET_STRING, move |_, _| Ret {
            eax: name,
            ..Ret::default()
        });
        e.register(BS_STRING_STR_CMP, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(BS_STRING_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            let length = if a[1] == 0 {
                0
            } else {
                e.mem.cstr(a[1]).len() as u16
            };
            e.mem.set_u16(a[0] + 4, length);
            Ret::default()
        });
        names(&mut e);
        e.register(SEQUENCE_MAP_GET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a2_0ed0, &args![this, 0.5f32, false]);
        assert_eq!(
            calls_to(&e, BS_STRING_SET),
            vec![
                vec![this.addr() + 0x40, name, 0],
                vec![this.addr() + 0x40, 0, 0]
            ]
        );
        assert_eq!(calls_to(&e, SEQUENCE_MAP_GET).len(), 1);
    }

    #[test]
    fn fn_00a20ed0_needs_a_visible_tile() {
        let mut e = update_engine();
        e.register(TILE_IS_TRUE, |_, _| Ret::default());
        let (this, _) = animated(&mut e);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a2_0ed0, &args![this, 0.5f32, false]).bool());
        assert!(calls_to(&e, NI_AV_OBJECT_UPDATE).is_empty());
    }

    #[test]
    fn tile_3d_setup_shader_properties_recurse_replaces_shade_properties() {
        let mut e = engine();
        let plain = object(&mut e, 2, &[]);
        let tiled = object(&mut e, 2, &[]);
        let inner = object(&mut e, 1, &[plain]);
        let root = object(&mut e, 1, &[tiled, inner, 0]);
        let shade = e.mem.alloc(0x80);
        let tile_shade = e.mem.alloc(0x80);
        e.mem.set_u32(tile_shade + 0x1c, TILE_SHADER_TYPE);
        let texturing = e.mem.alloc(0x40);
        let maps = e.mem.alloc(8);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x7e57);
        e.mem.set_u32(maps, map);
        e.mem.set_u32(texturing + 0x20, maps);
        e.register_double(NI_AV_OBJECT_GET_PROPERTY, move |_, a| Ret {
            eax: match (a[0] == tiled, a[1]) {
                (true, 3) => tile_shade,
                (false, 3) => shade,
                (_, _) => texturing,
            },
            ..Ret::default()
        });
        e.register(TILE_SHADER_PROPERTY_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(BS_SHADER_MANAGER_GET_SHADER, |_, _| Ret {
            eax: 0x5e00,
            ..Ret::default()
        });
        quiet(
            &mut e,
            &[
                NI_AV_OBJECT_REMOVE_PROPERTY,
                NI_AV_OBJECT_ATTACH_PROPERTY,
                TILE_SHADER_PROPERTY_SET_TILE_TEXTURE,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x00a2_1340, &args![0x5000u32, root]);
        assert_eq!(
            calls_to(&e, NI_AV_OBJECT_REMOVE_PROPERTY),
            vec![vec![plain, 3]]
        );
        let attached = calls_to(&e, NI_AV_OBJECT_ATTACH_PROPERTY);
        assert_eq!(attached.len(), 1);
        assert_eq!(attached[0][0], plain);
        assert_eq!(
            calls_to(&e, TILE_SHADER_PROPERTY_SET_TILE_TEXTURE),
            vec![vec![attached[0][1], 0x7e57]]
        );
        assert_eq!(e.mem.u32(plain + 0xc0), 0x5e00);
        assert_eq!(e.mem.u32(tiled + 0xc0), 0);
        // A null node: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00a2_1340, &args![0x5000u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn tile_3d_set_alpha_and_color_colours_every_geometry() {
        let mut e = engine();
        let leaf = object(&mut e, 2, &[]);
        let inner = object(&mut e, 1, &[]);
        let root = object(&mut e, 1, &[leaf, 0, inner]);
        let shade = e.mem.alloc(0x80);
        e.register_double(NI_AV_OBJECT_GET_PROPERTY, move |_, _| Ret {
            eax: shade,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this, OBJECT_VTABLE);
        let color = e.mem.alloc(0x10);
        for (i, value) in [0.1f32, 0.2, 0.3, 0.4].iter().enumerate() {
            e.mem.set_f32(color + 4 * i as u32, *value);
        }
        e.call_log = Some(vec![]);
        e.call(0x00a2_1570, &args![this, root, 0.5f32, color]);
        let half = 0.5f32.to_bits();
        assert_eq!(
            calls_to(&e, SET_ALPHA),
            vec![vec![this, 0, half, color], vec![this, inner, half, color]]
        );
        assert_eq!(e.mem.f32(shade + 0x78), 0.5);
        let colour: Vec<f32> = (0..4).map(|i| e.mem.f32(shade + 0x68 + 4 * i)).collect();
        assert_eq!(colour, vec![0.1, 0.2, 0.3, 0.4]);
    }
}
