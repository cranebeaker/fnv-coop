//! `fallout shared/tes.cpp` (Xbox PDB source unit), part 2: its functions from `004579e0` up to
//! (not including) `004fd3e0` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tes`]; anything public there may be used here.
//!
//! Session 1 of this part (the first 40 functions, `004579e0` to `00458ad0`)
//! holds three groups:
//!
//! - the geometry copy helpers (`MakeNewGeomData`, `CreateDeepCopySameTextures`)
//!   and `TES::InitModelsToLoad`, which loads the cells around the player and
//!   shows the loading menu;
//! - the loading-menu glue (`00457d70` and the small accessors on the loading
//!   menu singleton at `011da0c0`, whose word at `+0x222` holds menu state
//!   bits), `TES::SetWorldSpace`, and `TES::Pick`, `TES::Pick_ov2` and
//!   `TES::PickNI` (the Havok ray pick that `TES::Pick_ov2` falls back on);
//! - the `bhkPickData` vector helpers: SSE `hkVector4` operations on 16-byte
//!   vectors (copy of one lane, add, subtract, absolute value, compare, sign
//!   mask) that the pick code uses. They are translated lane by lane in `f32`,
//!   which is what the SSE instructions compute.
//!
//! Not translated: the compiler's exception-unwinding frames, the stack
//! cookie checks and the 16-byte stack realignment prologues.
//!
//! Session 2 (the next 40 functions, `00458b20` to `0045a730`) holds:
//!
//! - the `BSTempNode` class (constructor, `GetRTTI`, destructors) and
//!   `TES::AddTempDebugObject`, which hangs a node under one for debugging;
//! - `TES::GetMapNameForLocation` and `TES::GetCellPriority` (the cell
//!   priority cache);
//! - the `TES::DeadCount` list at `+0x9C` (an 8-byte entry per form: the
//!   form pointer and a 16-bit count) and its save and load code:
//!   `GetSaveSize`, `TES::SaveGame`, `TES::SaveGame_ov2`, `TES::LoadGame`;
//! - `TES::CreateFurnitureList`, the two cell walks `fn_00459920` and
//!   `fn_00459a00`, and the cell test run (`TES::RunCellTest`,
//!   `TES::TestCell`);
//! - the preloading of add-on nodes, forms, default models, the blood decal and
//!   the water textures (`TES::PreloadAddonNodes`, `TES::PreloadForms`,
//!   `fn_0045a370`, `fn_0045a520`, `fn_0045a600`) with their one-line helpers.
//!
//! Session 3 (the next 40 functions, `0045a750` to `0045cac0`) holds the
//! temporary particle cache (`TES::SetupTemporaryParticleCache`,
//! `TES::AddTemporaryParticleObjectToCache`, `TES::GetUnusedCachedParticleObject`,
//! the cache entry's constructor and destructors), the `NavMeshInfoMap` accessors,
//! the two long walks that add the loaded world to (`fn_0045b070`) and take it
//! out of (`fn_0045bc80`) the first shadow scene node, a set of small accessors
//! of classes the compiler folded into this unit, the two passes over the 5x5
//! grid of cells (`fn_0045c780`, `fn_0045c840`) and
//! `TES::InitDistantTextureBlending`.
//!
//! The next session continues at `0045cb90`.

#[allow(unused_imports)]
use super::tes::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// The loading menu singleton (written by the menu's `Create`, `00788daa`).
const LOADING_MENU: u32 = 0x011d_a0c0;
/// The HUD main menu singleton (written by the `Create` at `0076c0eb`).
const HUD_MAIN_MENU: u32 = 0x011d_96c0;
/// Tick count (ms) at which the loading menu may be closed even if a hide
/// request did not come (0 = none set).
const LOADING_MENU_DEADLINE: u32 = 0x011c_3efc;
/// `bShowLoadingAreaMessage:General` (an `INISetting`).
const SETTING_SHOW_LOADING_AREA_MESSAGE: u32 = 0x011c_3dbc;
/// `bPreemptivelyUnloadCells:General`.
const SETTING_PREEMPTIVELY_UNLOAD_CELLS: u32 = 0x011c_3d40;
/// `uGridsToLoad:General`.
const SETTING_GRIDS_TO_LOAD: u32 = 0x011c_63cc;
/// Returns a pointer to an unsigned setting's value.
const SETTING_VALUE_ADDRESS_INT: u32 = 0x0043_d4d0;
/// Returns a pointer to a boolean setting's value.
const SETTING_VALUE_ADDRESS_BYTE: u32 = 0x0040_8d60;
/// A `float` the pick code multiplies Havok coordinates by (set at start-up
/// by `00f38070` to the reciprocal of `01017820`).
const HAVOK_SCALE: u32 = 0x011c_3d18;
/// `0.001f`: how close a pick vector must be to zero to count as empty.
const PICK_EPSILON: u32 = 0x0101_7d00;
/// Four words `0x7fffffff`: the mask that clears the sign of a vector's lanes.
const ABS_MASK: u32 = 0x0101_7d10;
/// A byte `TES::SetWorldSpace` stores at its end (see `fn_004583f0`): set when
/// `005862e0` says the new world space is false.
const WORLD_SPACE_FLAG: u32 = 0x011a_d80c;
/// The `TES` singleton (`TES::this` of the game; its `+0x64` is the water system).
const TES_OBJECT: u32 = 0x011d_ea10;
/// The `TESDataHandler` singleton.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// Source path passed to the scope guard (`D:\_Fallout3\...\TES.cpp`).
const TES_CPP_PATH: u32 = 0x0101_7874;

/// `GetTickCount` (import slot).
const GET_TICK_COUNT: u32 = 0x00fd_f060;
/// `NiPointer<T>::NiPointer(T*)` (`ecx` = the pointer cell; takes a reference).
const NI_POINTER_CTOR: u32 = 0x0063_3c90;
/// `NiPointer<T>::operator T*` (returns the stored pointer).
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer<T>::~NiPointer` (releases the reference).
const NI_POINTER_DTOR: u32 = 0x0045_cec0;
/// `NiPointer<T>::operator=(T*)`.
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The scope guard around allocations made under a memory tag.
const SCOPE_GUARD_CTOR: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DTOR: u32 = 0x0040_4ee0;
/// A trivial constructor of an `hkVector4`/`NiPoint3`: returns `this`.
const VECTOR_CTOR: u32 = 0x0068_15c0;
/// Copies the 16 bytes at the argument into `this` (`hkVector4` assignment).
const VECTOR_COPY: u32 = 0x004a_3c90;
/// `TESForm::GetFormType` (the form-type byte at `+4`).
const FORM_GET_TYPE: u32 = 0x0040_1170;
/// `TESObjectCELL::IsInterior` (bit 0 of the cell flags at `+0x24`).
const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
/// `TESObjectCELL::GetWorldSpace` (Xbox PDB).
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
/// `TESObjectREFR::GetWorldSpace` (Xbox PDB).
const REFERENCE_GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// Returns `this->pInteriorCell` (the map names this `ActorMover::GetPreferredMoveMode`).
const GET_INTERIOR_CELL: u32 = 0x005f_36f0;
/// `TES::GetWorldSpace` (`004fd3e0`, in the next part of this unit).
const GET_WORLD_SPACE: u32 = 0x004f_d3e0;
/// `GridCellArray::Get` (Xbox PDB): pointer to the cell slot at (x, y).
const GRID_CELL_ARRAY_GET: u32 = 0x004b_a490;
/// `GridCellArray::GetAttached` (Xbox PDB).
const GRID_CELL_ARRAY_GET_ATTACHED: u32 = 0x004b_a5a0;
/// `TESWorldSpace::LoadCell` (Xbox PDB).
const WORLD_SPACE_LOAD_CELL: u32 = 0x0058_5b30;
/// `TESDataHandler` function that creates the cell at (x, y) in a world space.
const DATA_HANDLER_CREATE_CELL: u32 = 0x0046_1c20;
/// `TESObjectCELL::Pick` (Xbox PDB).
const CELL_PICK: u32 = 0x0055_3ee0;

/// Reads `uGridsToLoad`, the way the code does: one call per use.
fn grids_to_load(e: &mut Engine) -> u32 {
    let value = e
        .call(SETTING_VALUE_ADDRESS_INT, &args![SETTING_GRIDS_TO_LOAD])
        .u32();
    e.mem.u32(value)
}

/// The stored pointer of the `NiPointer` at `cell`.
fn ni_pointer_get(e: &mut Engine, cell: Ptr) -> u32 {
    e.call(NI_POINTER_GET, &args![cell]).u32()
}

// Translated from 004579e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes its `float` on to `004019b0` (which hands it to `004019d0`, a
/// wrapper over the CRT routine `00ec6054` taking a `double`) and returns that
/// function's `float` result (x87 `ST0`).
pub fn fn_004579e0(e: &mut Engine, value: f32) -> f32 {
    e.call(0x0040_19b0, &args![value]).f32()
}

// Translated from 00457a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MakeNewGeomData` (Xbox PDB): gives a scene-graph node its own copy of its
/// geometry data. For a node whose slot `0x18` returns an object (`geometry`),
/// the `NiPointer` at `geometry + 0xB8` is read; when the pointed-to object's
/// word at `+0xE` has none of the bits `0x7000` and
/// `BGSSaveFormBuffer::GetForm` (`007af430`) says it has a form, the object is
/// deep-copied (`NiObject::CreateDeepCopy`) and the copy is installed through
/// `geometry`'s slot `0xE4`; otherwise the original is installed through the
/// same slot. For any other node the node's slot `0xC` returns a child list
/// and every child is processed the same way, last to first.
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn make_new_geom_data(e: &mut Engine, node: Ptr) {
    e.with_stack(4, |e, copy| {
        e.call(NI_POINTER_CTOR, &args![copy, 0u32]);
        if !node.is_null() {
            let geometry = e.vcall(node.addr(), 0x18, &[]).u32();
            if geometry != 0 {
                let shape = e.call(0x0054_95f0, &args![geometry]).u32();
                e.with_stack(4, |e, held| {
                    e.call(NI_POINTER_CTOR, &args![held, shape]);
                    if ni_pointer_get(e, held) != 0 {
                        let object = ni_pointer_get(e, held);
                        let mut copied = false;
                        if fn_00457b80(e, Ptr::new(object)) == 0 {
                            let object = ni_pointer_get(e, held);
                            if e.call(0x007a_f430, &args![object]).u32() != 0 {
                                let object = ni_pointer_get(e, held);
                                e.call(0x00a5_d510, &args![object, copy]);
                                let new_object = ni_pointer_get(e, copy);
                                e.call(NI_POINTER_ASSIGN, &args![held, new_object]);
                                let installed = ni_pointer_get(e, held);
                                e.vcall(geometry, 0xe4, &args![installed]);
                                copied = true;
                            }
                        }
                        if !copied {
                            let installed = ni_pointer_get(e, held);
                            e.vcall(geometry, 0xe4, &args![installed]);
                        }
                    }
                    e.call(NI_POINTER_DTOR, &args![held]);
                });
            } else {
                let children = e.vcall(node.addr(), 0xc, &[]).u32();
                if children != 0 {
                    let mut count = e.call(0x0043_b480, &args![children]).u32();
                    while count != 0 {
                        count -= 1;
                        let child = e.call(0x0043_b4a0, &args![children, count]).u32();
                        make_new_geom_data(e, Ptr::new(child));
                    }
                }
            }
        }
        e.call(NI_POINTER_DTOR, &args![copy]);
    });
}

// Translated from 00457b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0xE` of the object, masked with `0x7000`.
pub fn fn_00457b80(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0xe) & 0x7000
}

// Translated from 00457ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::CreateDeepCopySameTextures` (Xbox PDB): for a non-null `node`, clones
/// it (`NiObject::Clone`, `00a5d2c0`, with `clone_argument`) and runs
/// [`make_new_geom_data`] on the clone; returns the clone (null for a null
/// node). `this` is not used.
pub fn tes_create_deep_copy_same_textures(
    e: &mut Engine,
    _this: Ptr<TES>,
    node: Ptr,
    clone_argument: u32,
) -> Ptr {
    let mut copy = Ptr::NULL;
    if !node.is_null() {
        copy = e.call(0x00a5_d2c0, &args![node, clone_argument]).ptr();
        make_new_geom_data(e, copy);
    }
    copy
}

// Translated from 00457be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::InitModelsToLoad` (Xbox PDB): walks the `uGridsToLoad` x
/// `uGridsToLoad` square of cells around the current grid position. A cell is
/// visited when `mode` is 0 and the grid array has no attached cell there,
/// when `mode` is 1 and the grid array has no cell there, or for any other
/// `mode`. For each visited cell, with a world space present, the world
/// space's `LoadCell` (or, when it gives none, the data handler's cell
/// creation) gives a cell; if `00450fb0` is false for that cell, `00454400`
/// is run on it and its result counted. When anything was counted, the
/// loading menu code [`fn_00457d70`] runs with (show, world space, 0).
pub fn tes_init_models_to_load(e: &mut Engine, this: Ptr<TES>, mode: u32) {
    let mut counted = 0u32;
    let half = grids_to_load(e) >> 1;
    let first_x = e.get(this, TES::iCurrentGridX).wrapping_sub(half as i32);
    let half = grids_to_load(e) >> 1;
    let first_y = e.get(this, TES::iCurrentGridY).wrapping_sub(half as i32);
    let grid_array = e.get(this, TES::pGridCellA);

    let mut x = 0u32;
    while x < grids_to_load(e) {
        let mut y = 0u32;
        while y < grids_to_load(e) {
            let skip = match mode {
                0 => e
                    .call(GRID_CELL_ARRAY_GET_ATTACHED, &args![grid_array, x, y])
                    .bool(),
                1 => e.call(GRID_CELL_ARRAY_GET, &args![grid_array, x, y]).u32() != 0,
                _ => false,
            };
            if !skip && e.call(GET_WORLD_SPACE, &args![this]).u32() != 0 {
                let cell_x = first_x.wrapping_add(x as i32);
                let cell_y = first_y.wrapping_add(y as i32);
                let world_space = e.get(this, TES::pWorldSpace);
                let mut cell = e
                    .call(WORLD_SPACE_LOAD_CELL, &args![world_space, cell_x, cell_y])
                    .u32();
                if cell == 0 {
                    let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
                    let data_handler: u32 = e.global(DATA_HANDLER);
                    cell = e
                        .call(
                            DATA_HANDLER_CREATE_CELL,
                            &args![data_handler, cell_x, cell_y, world_space, 1u32],
                        )
                        .u32();
                }
                if cell != 0 && !e.call(0x0045_0fb0, &args![cell]).bool() {
                    counted = counted.wrapping_add(e.call(0x0045_4400, &args![cell]).u32());
                }
            }
            y += 1;
        }
        x += 1;
    }
    if counted != 0 {
        let world_space = e.get(this, TES::pWorldSpace);
        fn_00457d70(e, this, 1, world_space, 0);
    }
}

// Translated from 00457d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows (`show` != 0) or hides the loading menu. Returns true only when it
/// started showing it.
///
/// Hiding: the menu is closed (deadline cleared, menu bit 10 cleared,
/// `Interface::CloseLoadingMenu`, and the HUD main menu mode set to 2 unless
/// it is already `0x18`) when bit 10 is clear, or when it is set and
/// `force_close` is; otherwise a deadline five seconds from now is set the
/// first time, and the menu is closed once the tick count passes it.
///
/// Showing: if the loading menu is visible and bit 3 is clear, only
/// [`fn_00458180`] runs (with `form`). Otherwise, unless `00451530` says no,
/// the sky is told (`0063e010`), all sounds of type `0x14` end, and for a
/// `form` the game looks up a world space through its type (`0x39`: a cell,
/// replaced by its world space unless it is an interior; `0x3a` to `0x40` and
/// `0x69`: a reference, whose cell's or own world space is looked up, a
/// result the game then never uses), `Interface::CreateLoadingMenu(form,
/// force_close)` runs, and the state words of the `00aff100` object are
/// switched ([`fn_00458010`], its slot `0x30`, [`fn_00457ff0`]).
pub fn fn_00457d70(e: &mut Engine, this: Ptr<TES>, show: u8, form: Ptr, force_close: u8) -> bool {
    if show == 0 {
        if !fn_004580e0(e) || (fn_004580e0(e) && force_close != 0) {
            close_loading_menu(e);
        } else {
            if e.global::<u32>(LOADING_MENU_DEADLINE) == 0 {
                let now = fn_00457fe0(e);
                e.set_global(LOADING_MENU_DEADLINE, now.wrapping_add(5000));
            }
            let now = fn_00457fe0(e);
            if now > e.global::<u32>(LOADING_MENU_DEADLINE) {
                close_loading_menu(e);
            }
        }
        return false;
    }

    if e.call(0x0070_5e80, &[]).bool() && !fn_00458140(e) {
        fn_00458180(e, form);
        return false;
    }
    if e.call(0x0045_1530, &args![this]).bool() {
        return false;
    }
    let sky = e.get(this, TES::pSky);
    e.call(0x0063_e010, &args![sky]);
    // `BSAudio::QInstance` takes no argument; the 0x14 is the sound type that
    // `BSAudio::EndAllSoundsOfType` (`00ad8780`) takes.
    let audio = e.call(0x0045_3a70, &[]).u32();
    e.call(0x00ad_8780, &args![audio, 0x14u32]);

    let mut form = form;
    if !form.is_null() {
        let form_type = e.call(FORM_GET_TYPE, &args![form]).u32();
        match form_type.wrapping_sub(0x39) {
            0 => {
                if !e.call(CELL_IS_INTERIOR, &args![form]).bool() {
                    form = e.call(CELL_GET_WORLD_SPACE, &args![form]).ptr();
                }
            }
            1..=7 | 0x30 => {
                let cell = e.call(0x008d_6f30, &args![form]).u32();
                if cell != 0 {
                    if !e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
                        e.call(CELL_GET_WORLD_SPACE, &args![cell]);
                    }
                } else {
                    e.call(REFERENCE_GET_WORLD_SPACE, &args![form]);
                }
            }
            _ => {}
        }
    }
    e.call(0x0070_5e00, &args![form, force_close as u32]);
    let menu_state = e.call(0x00af_f100, &[]).ptr();
    fn_00458010(e, menu_state);
    let object = e.call(0x00af_f100, &[]).u32();
    e.vcall(object, 0x30, &[]);
    let menu_state = e.call(0x00af_f100, &[]).ptr();
    fn_00457ff0(e, menu_state);
    true
}

/// The closing sequence of the loading menu, shared by both hide paths of
/// [`fn_00457d70`].
fn close_loading_menu(e: &mut Engine) {
    e.set_global(LOADING_MENU_DEADLINE, 0u32);
    fn_00458060(e, 0);
    e.call(0x0070_5e30, &[]);
    if fn_00458030(e) != 0x18 {
        e.call(0x0077_1700, &args![2u32]);
    }
}

// Translated from 00457fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetTickCount` (through its import slot).
pub fn fn_00457fe0(e: &mut Engine) -> u32 {
    e.call(GET_TICK_COUNT, &[]).u32()
}

// Translated from 00457ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Changes the state word at `+0x5C` from 1 to 2 (any other value stays).
pub fn fn_00457ff0(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr() + 0x5c) == 1 {
        e.mem.set_u32(this.addr() + 0x5c, 2);
    }
}

// Translated from 00458010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Changes the state word at `+0x5C` from 2 to 1 (any other value stays).
pub fn fn_00458010(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr() + 0x5c) == 2 {
        e.mem.set_u32(this.addr() + 0x5c, 1);
    }
}

// Translated from 00458030 (decompiled, FalloutNV.exe 1.4.0.525)
/// The HUD main menu's mode (`+0x1C4`), 0 when there is no HUD main menu.
pub fn fn_00458030(e: &mut Engine) -> u32 {
    let menu: u32 = e.global(HUD_MAIN_MENU);
    if menu == 0 {
        0
    } else {
        e.mem.u32(menu + 0x1c4)
    }
}

// Translated from 00458060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit 10 of the loading menu's state word (if there is a
/// loading menu).
pub fn fn_00458060(e: &mut Engine, set: u8) {
    let menu: u32 = e.global(LOADING_MENU);
    if menu != 0 {
        fn_00458080(e, Ptr::new(menu), 10, set);
    }
}

// Translated from 00458080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` != 0) or clears bit `bit` (mod 32, as the shift does) of the
/// 16-bit state word at `+0x222` of the loading menu.
pub fn fn_00458080(e: &mut Engine, this: Ptr, bit: u32, set: u8) {
    let mask = 1u32 << (bit & 0x1f);
    let state = e.mem.u16(this.addr() + 0x222) as u32;
    let state = if set != 0 {
        state | mask
    } else {
        state & !mask
    };
    e.mem.set_u16(this.addr() + 0x222, state as u16);
}

// Translated from 004580e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 10 of the loading menu's state word; false without a loading menu.
pub fn fn_004580e0(e: &mut Engine) -> bool {
    let menu: u32 = e.global(LOADING_MENU);
    menu != 0 && fn_00458110(e, Ptr::new(menu), 10)
}

// Translated from 00458110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `bit` (mod 32) of the 16-bit state word at `+0x222` is set.
pub fn fn_00458110(e: &mut Engine, this: Ptr, bit: u32) -> bool {
    let state = e.mem.u16(this.addr() + 0x222) as u32;
    state & (1u32 << (bit & 0x1f)) != 0
}

// Translated from 00458140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 3 of the loading menu's state word; false without a loading menu.
pub fn fn_00458140(e: &mut Engine) -> bool {
    let menu: u32 = e.global(LOADING_MENU);
    menu != 0 && fn_00458110(e, Ptr::new(menu), 3)
}

// Translated from 00458180 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a loading menu whose word at `+0x1E8` is 0 and a non-null `form`:
/// sets bit 3 of the state word and calls `00788ab0` on the menu with `form`.
pub fn fn_00458180(e: &mut Engine, form: Ptr) {
    let menu: u32 = e.global(LOADING_MENU);
    if menu != 0 && !form.is_null() && e.mem.u32(menu + 0x1e8) == 0 {
        fn_004581c0(e, 1);
        e.call(0x0078_8ab0, &args![menu, form]);
    }
}

// Translated from 004581c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit 3 of the loading menu's state word (if there is a
/// loading menu).
pub fn fn_004581c0(e: &mut Engine, set: u8) {
    let menu: u32 = e.global(LOADING_MENU);
    if menu != 0 {
        fn_00458080(e, Ptr::new(menu), 3, set);
    }
}

// Translated from 004581e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the value of the INI setting
/// `bShowLoadingAreaMessage:General`. `this` and the one stack word are not
/// used.
pub fn fn_004581e0(e: &mut Engine, _this: u32, _unused_1: u32) -> u32 {
    e.call(
        SETTING_VALUE_ADDRESS_BYTE,
        &args![SETTING_SHOW_LOADING_AREA_MESSAGE],
    )
    .u32()
}

// Translated from 00458200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::SetWorldSpace` (Xbox PDB): switches the world space (nothing for a
/// null one or the current one).
///
/// With `bPreemptivelyUnloadCells` set: if an interior cell is loaded
/// `00453940` runs first, the cells of the old world space are unloaded
/// (`00455200` with (1, old world space), plus `00454d50(1)` unless an
/// interior cell is loaded) and, if that unloaded anything, the unused
/// textures are cleaned up (`00452490(0)`). Then `pWorldSpace` is stored; the
/// word at `+0xD0` of the new world space ([`fn_00458400`]) is run through
/// `00526270` when set; `00453940`, `00450d80`, `007043c0` and
/// `Interface::SetRootMapSpace` (`00709d20`) run; and the water system's
/// `0052d3b0` is called with the new world space's texture (loaded through a
/// `NiPointer` and `004568c0` when the byte `00454b50` points at is set, else
/// with 0). The last step stores `005862e0(new) == 0` ([`fn_004583f0`]).
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn tes_set_world_space(e: &mut Engine, this: Ptr<TES>, world_space: Ptr) {
    if world_space.is_null() || e.get(this, TES::pWorldSpace) == world_space {
        return;
    }
    let setting = e
        .call(
            SETTING_VALUE_ADDRESS_BYTE,
            &args![SETTING_PREEMPTIVELY_UNLOAD_CELLS],
        )
        .u32();
    if e.mem.u8(setting) != 0 {
        if e.call(GET_INTERIOR_CELL, &args![this]).u32() != 0 {
            e.call(0x0045_3940, &args![this]);
        }
        let old = e.get(this, TES::pWorldSpace);
        let mut unloaded = e.call(0x0045_5200, &args![this, 1u32, old]).u32();
        if e.call(GET_INTERIOR_CELL, &args![this]).u32() == 0 {
            unloaded = unloaded.wrapping_add(e.call(0x0045_4d50, &args![this, 1u32]).u32());
        }
        if unloaded != 0 {
            e.call(0x0045_2490, &args![this, 0u32]);
        }
    }
    e.set(this, TES::pWorldSpace, world_space);
    let extra = fn_00458400(e, world_space);
    if extra != 0 {
        e.call(0x0052_6270, &args![extra]);
    }
    let unknown = e.call(0x0067_33e0, &args![world_space]).u32();
    e.call(0x0058_eeb0, &args![unknown]);
    e.call(0x0045_3940, &args![this]);
    e.call(0x0045_0d80, &args![this]);
    e.call(0x0070_43c0, &[]);
    e.call(0x0070_9d20, &args![world_space]);

    let flag = e.call(0x0045_4b50, &args![world_space]).u32();
    let tes_object: u32 = e.global(TES_OBJECT);
    if e.mem.i8(flag) != 0 {
        e.with_stack(4, |e, guard| {
            e.call(
                SCOPE_GUARD_CTOR,
                &args![guard, 0x1du32, 1u32, TES_CPP_PATH, 0x1751u32],
            );
            e.with_stack(4, |e, texture| {
                e.call(NI_POINTER_CTOR, &args![texture, 0u32]);
                let name = e.call(0x0045_4b50, &args![world_space]).u32();
                e.call(0x0045_68c0, &args![tes_object, name, texture, 1u32, 1u32]);
                let loaded = ni_pointer_get(e, texture);
                let water = e.call(0x0070_ec90, &args![tes_object]).u32();
                e.call(0x0052_d3b0, &args![water, loaded]);
                e.call(NI_POINTER_DTOR, &args![texture]);
            });
            e.call(SCOPE_GUARD_DTOR, &args![guard]);
        });
    } else {
        let water = e.call(0x0070_ec90, &args![tes_object]).u32();
        e.call(0x0052_d3b0, &args![water, 0u32]);
    }
    let known = e.call(0x0058_62e0, &args![world_space]).bool();
    fn_004583f0(e, !known as u8);
}

// Translated from 004583f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte in the global `011ad80c`.
pub fn fn_004583f0(e: &mut Engine, value: u8) {
    e.set_global(WORLD_SPACE_FLAG, value);
}

// Translated from 00458400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0xD0` of a `TESWorldSpace` (what it holds is not confirmed;
/// [`tes_set_world_space`] passes it to `00526270` when it is not null).
pub fn fn_00458400(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xd0)
}

// Translated from 00458420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::Pick` (Xbox PDB): [`tes_pick_ov2`] with the cell-based pick enabled.
pub fn tes_pick(e: &mut Engine, this: Ptr<TES>, pick: Ptr) -> u32 {
    tes_pick_ov2(e, this, pick, 1)
}

// Translated from 00458440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::Pick_ov2` (Xbox PDB). With `by_cell` == 0: [`tes_pick_ni`]. Otherwise
/// the pick runs on the interior cell or, without one, on the centre cell of
/// the grid (`uGridsToLoad / 2` in x and y, read from the grid array), through
/// `TESObjectCELL::Pick` (`00553ee0`); 0 when there is no such cell.
pub fn tes_pick_ov2(e: &mut Engine, this: Ptr<TES>, pick: Ptr, by_cell: u8) -> u32 {
    if by_cell == 0 {
        return tes_pick_ni(e, this, pick);
    }
    let mut cell = e.call(GET_INTERIOR_CELL, &args![this]).u32();
    if cell == 0 {
        let first = grids_to_load(e) >> 1;
        let second = grids_to_load(e) >> 1;
        let grid_array = e.get(this, TES::pGridCellA);
        let slot = e
            .call(GRID_CELL_ARRAY_GET, &args![grid_array, second, first])
            .u32();
        cell = e.mem.u32(slot);
    }
    if cell == 0 {
        0
    } else {
        e.call(CELL_PICK, &args![cell, pick]).u32()
    }
}

// Translated from 004584d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::PickNI` (Xbox PDB): the Havok ray described by a `bhkPickData`
/// (`pick`) run as a `NiPick` over the scene-graph root. The ray starts at the
/// pick data's `m_from` and ends at its `m_to` (`bhkPickData::CalcTo` first
/// completes it from `hkLength`), both scaled by the `float` at `011c3d18`
/// into game units; the direction is end minus start (`00439ef0`). A
/// `NiPick` (0x34 bytes, constructor `00e98f20` with (0, 8)) targets the root
/// (`0084e3a0`), gets its flag byte at `+0x11` set (`00458b30`), and picks
/// (`NiPick::PickObjects`). Returns the first result's object (through the
/// `NiPointer` getter `00458b50`), or 0 when nothing was hit.
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn tes_pick_ni(e: &mut Engine, this: Ptr<TES>, pick: Ptr) -> u32 {
    // Locals: origin, direction, end and difference (12 bytes each), the NiPick.
    e.with_stack(0x70, |e, block| {
        let origin = block;
        let direction = block.byte_add(12);
        let end = block.byte_add(24);
        let difference = block.byte_add(36);
        let ni_pick = block.byte_add(48);
        e.call(VECTOR_CTOR, &args![direction]);
        e.call(VECTOR_CTOR, &args![origin]);
        e.call(VECTOR_CTOR, &args![end]);
        fn_004585f0(e, pick, origin);
        fn_004587f0(e, pick, end);
        let delta = e.call(0x0043_9ef0, &args![end, difference, origin]).u32();
        for word in 0..3 {
            let value = e.mem.u32(delta + 4 * word);
            e.mem.set_u32(direction.addr() + 4 * word, value);
        }
        e.call(0x00e9_8f20, &args![ni_pick, 0u32, 8u32]);
        let root = e.call(0x0084_e3a0, &args![this]).u32();
        e.call(0x0070_5fc0, &args![ni_pick, root]);
        e.call(0x0045_8b30, &args![ni_pick, 1u32]);
        let mut result = 0;
        if e.call(0x00e9_8e20, &args![ni_pick, origin, direction, 0u32])
            .bool()
        {
            let results = e.call(0x0050_0940, &args![ni_pick]).u32();
            let first = e.call(0x0096_8670, &args![results, 0u32]).u32();
            result = e.call(0x0045_8b50, &args![first]).u32();
        }
        e.call(0x00e9_8fa0, &args![ni_pick]);
        result
    })
}

// Translated from 004585f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the first three lanes of the vector at `this` (`m_from` of a
/// `bhkPickData`), scaled ([`fn_00458620`]), to the three floats at `out`.
/// Returns `out`.
pub fn fn_004585f0(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let vector = e.call(VECTOR_CTOR, &args![this]).ptr();
    fn_00458620(e, out, vector)
}

// Translated from 00458620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes lanes 0 to 2 of the vector at `vector`, each multiplied by the
/// `float` at `011c3d18` ([`fn_004587d0`]), to the three floats at `out`.
/// Returns `out`. (cdecl; the compiler realigns the stack to 16 bytes.)
pub fn fn_00458620(e: &mut Engine, out: Ptr, vector: Ptr) -> Ptr {
    e.with_stack(0x30, |e, temporaries| {
        for lane in 0..3u32 {
            let lane_copy = temporaries.byte_add(16 * lane);
            let broadcast = fn_00458700(e, vector, lane_copy, lane as i32);
            let value = fn_004586d0(e, broadcast);
            let scaled = fn_004587d0(e, value);
            e.mem.set_f32(out.addr() + 4 * lane, scaled);
        }
    });
    out
}

// Translated from 004586d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lane 0 of the vector at `this`, as a `float`.
pub fn fn_004586d0(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr())
}

// Translated from 00458700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores into the 16-byte vector at `out` (through the vector copy
/// `004a3c90`) lane `index` of the vector at `this` repeated in all four
/// lanes (`SHUFPS`); for `index` 0 the vector at `this` is copied as it is.
/// Any other `index` (negative ones included) gives lane 3. Returns `out`.
pub fn fn_00458700(e: &mut Engine, this: Ptr, out: Ptr, index: i32) -> Ptr {
    if index == 0 {
        e.call(VECTOR_COPY, &args![out, this]);
    } else {
        // Only exact 1 and 2 are tested (0045872d, 00458733); everything else
        // jumps to the `SHUFPS 0xff` block (00458735 -> 00458792).
        let lane: u32 = match index {
            1 => 1,
            2 => 2,
            _ => 3,
        };
        let value = e.mem.u32(this.addr() + 4 * lane);
        e.with_stack(16, |e, shuffled| {
            for word in 0..4 {
                e.mem.set_u32(shuffled.addr() + 4 * word, value);
            }
            e.call(VECTOR_COPY, &args![out, shuffled]);
        });
    }
    out
}

// Translated from 004587d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `value` times the `float` at `011c3d18` (Havok to game scale), as a `float`.
pub fn fn_004587d0(e: &mut Engine, value: f32) -> f32 {
    let scale: f32 = e.global(HAVOK_SCALE);
    (scale as f64 * value as f64) as f32
}

// Translated from 004587f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_004585f0`] for `m_to` (`this + 0x10`, after [`fn_00458820`] has
/// completed it from the length). Returns `out`.
pub fn fn_004587f0(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let to = fn_00458820(e, this);
    fn_00458620(e, out, to)
}

// Translated from 00458820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `bhkPickData::CalcTo` on the pick data and returns the address of its
/// `m_to` vector (`+0x10`).
pub fn fn_00458820(e: &mut Engine, this: Ptr) -> Ptr {
    bhk_pick_data_calc_to(e, this);
    this.byte_add(0x10)
}

// Translated from 00458840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPickData::CalcTo` (Xbox PDB): when the length vector `hkLength`
/// (`+0x90`) is not (nearly) zero ([`fn_004588d0`]), sets `m_to` (`+0x10`) to
/// `m_from` (`+0x00`) plus `hkLength`.
pub fn bhk_pick_data_calc_to(e: &mut Engine, this: Ptr) {
    if fn_004588d0(e, this) {
        fn_00458880(e, this.byte_add(0x10), this, this.byte_add(0x90));
    }
}

// Translated from 00458880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this = a + b`, lane by lane (`ADDPS`, `float`) on 16-byte vectors.
pub fn fn_00458880(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr) {
    let lanes: Vec<f32> = (0..4)
        .map(|lane| e.mem.f32(a.addr() + 4 * lane) + e.mem.f32(b.addr() + 4 * lane))
        .collect();
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_f32(this.addr() + 4 * lane as u32, value);
    }
}

// Translated from 004588d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the vector at `this + 0x90` (`hkLength`) is NOT within the
/// `float` at `01017d00` (0.001) of the zero vector that `00458b20` returns,
/// in lanes x, y and z ([`fn_00458900`]).
pub fn fn_004588d0(e: &mut Engine, this: Ptr) -> bool {
    let epsilon: f32 = e.global(PICK_EPSILON);
    let zero = e.call(0x0045_8b20, &[]).ptr();
    !fn_00458900(e, this.byte_add(0x90), zero, epsilon)
}

// Translated from 00458900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `|this - other| <= tolerance` in lanes x, y and z (lane w is not
/// tested): the difference ([`fn_00458a10`]), its absolute value
/// ([`fn_00458ad0`]), the tolerance repeated in four lanes ([`fn_004589c0`]),
/// the lane compare ([`fn_00458a60`]) and the sign-bit test with mask 7
/// ([`fn_00458990`]).
pub fn fn_00458900(e: &mut Engine, this: Ptr, other: Ptr, tolerance: f32) -> bool {
    e.with_stack(0x30, |e, block| {
        let difference = block;
        let limit = block.byte_add(16);
        let flags = block.byte_add(32);
        e.call(VECTOR_CTOR, &args![difference]);
        fn_00458a10(e, difference, this, other);
        fn_00458ad0(e, difference, difference);
        e.call(VECTOR_CTOR, &args![limit]);
        fn_004589c0(e, limit, tolerance);
        let flags = fn_00458a60(e, difference, flags, limit);
        fn_00458990(e, flags, 7)
    })
}

// Translated from 00458990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MOVMSKPS`: collects the sign bits of the four lanes (lane 0 is bit 0);
/// true when all bits of `mask` are set.
pub fn fn_00458990(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    let mut signs = 0u32;
    for lane in 0..4 {
        signs |= (e.mem.u32(this.addr() + 4 * lane) >> 31) << lane;
    }
    signs & mask == mask
}

// Translated from 004589c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in all four lanes of the vector at `this` (`SHUFPS`).
pub fn fn_004589c0(e: &mut Engine, this: Ptr, value: f32) {
    for lane in 0..4 {
        e.mem.set_u32(this.addr() + 4 * lane, value.to_bits());
    }
}

// Translated from 00458a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this = a - b`, lane by lane (`SUBPS`, `float`).
pub fn fn_00458a10(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr) {
    let lanes: Vec<f32> = (0..4)
        .map(|lane| e.mem.f32(a.addr() + 4 * lane) - e.mem.f32(b.addr() + 4 * lane))
        .collect();
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_f32(this.addr() + 4 * lane as u32, value);
    }
}

// Translated from 00458a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CMPLEPS`: for each lane, `out` gets all ones where `this <= b` (false for
/// NaN) and zero elsewhere. Returns `out`.
pub fn fn_00458a60(e: &mut Engine, this: Ptr, out: Ptr, b: Ptr) -> Ptr {
    let lanes: Vec<u32> = (0..4)
        .map(|lane| {
            if e.mem.f32(this.addr() + 4 * lane) <= e.mem.f32(b.addr() + 4 * lane) {
                u32::MAX
            } else {
                0
            }
        })
        .collect();
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_u32(out.addr() + 4 * lane as u32, value);
    }
    out
}

// Translated from 00458ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this = source & mask` for the four words, with the mask (all
/// `0x7fffffff`) read from `01017d10`: the absolute value of each lane.
pub fn fn_00458ad0(e: &mut Engine, this: Ptr, source: Ptr) {
    let words: Vec<u32> = (0..4)
        .map(|word| e.mem.u32(source.addr() + 4 * word) & e.mem.u32(ABS_MASK + 4 * word))
        .collect();
    for (word, value) in words.into_iter().enumerate() {
        e.mem.set_u32(this.addr() + 4 * word as u32, value);
    }
}

// ----- Session 2: `00458b20` to `0045a730` -----

/// `sRagdollDataDefault:General` (a string `INISetting`; `00403df0` returns a
/// pointer to its text).
const SETTING_RAGDOLL_DATA_DEFAULT: u32 = 0x011d_14c4;
/// `sBloodParticleDefault:General`.
const SETTING_BLOOD_PARTICLE_DEFAULT: u32 = 0x011c_ebb0;
/// `sSplashParticles:General`.
const SETTING_SPLASH_PARTICLES: u32 = 0x011d_12c0;
/// `sExplosionSplashParticles:General`.
const SETTING_EXPLOSION_SPLASH_PARTICLES: u32 = 0x011d_0ce4;
/// `sBloodTextureDefault:General`.
const SETTING_BLOOD_TEXTURE_DEFAULT: u32 = 0x011c_fd40;
/// `bUsePerWorldSpaceWaterNoise:Water`.
const SETTING_PER_WORLD_SPACE_WATER_NOISE: u32 = 0x011c_7b98;
/// A byte `INISetting` (`011de4e8`) which, when set, makes the save-size and
/// save code log how many bytes they wrote.
const SETTING_SAVE_SIZE_LOG: u32 = 0x011d_e4e8;
/// Returns a pointer to a string setting's text.
const SETTING_VALUE_ADDRESS_STRING: u32 = 0x0040_3df0;

/// The cell `TES::GetCellPriority` answered for last.
const CELL_PRIORITY_LAST_CELL: u32 = 0x011c_3f04;
/// The priority (1 or 3) it answered.
const CELL_PRIORITY_VALUE: u32 = 0x011c_3f00;
/// A word the save code appends to the save when the save version is at
/// least 0x32 (four bytes of the global at this address).
const SAVE_EXTRA_WORD: u32 = 0x011c_3c08;
/// The weapon object whose `QueueFiles` `TES::PreloadForms` calls.
const WEAPON_QUEUE_OBJECT: u32 = 0x011c_a278;
/// The `NiPointer` cell `fn_0045a190` reads (the scene root the loaded add-on
/// nodes are attached to).
const ADDON_PARENT_CELL: u32 = 0x011d_ed58;
/// `RTTI` descriptors `TES::LoadGame` hands the dynamic cast.
const RTTI_LOAD_SOURCE: u32 = 0x0118_3028;
const RTTI_LOAD_TARGET: u32 = 0x0118_46e8;
/// `BSTempNode`'s vtable and `NiRTTI`.
const BS_TEMP_NODE_VTABLE: u32 = 0x0101_7d24;
const BS_TEMP_NODE_RTTI: u32 = 0x0120_2e00;

/// `Error` (the game's formatted debug-message function; C varargs).
const ERROR_LOG: u32 = 0x0040_fbe0;
/// `strlen`.
const STRING_LENGTH: u32 = 0x0044_a670;
/// A `sprintf`-style formatter `(buffer, size, format, ...)`.
const FORMAT_STRING: u32 = 0x0040_6d00;
/// The integer absolute value (negates when below zero).
const ABSOLUTE_VALUE: u32 = 0x00ec_7d40;
/// `__RTDynamicCast(object, vfDelta, source, target, isReference)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The text of a string object (a `NiPointer` to the text at `+4`; an empty
/// constant when null).
const STRING_OBJECT_TEXT: u32 = 0x0040_8da0;
/// Whether a list node has neither item nor successor.
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `NiNode::NiNode(unsigned short)` and `NiNode::~NiNode`.
const NI_NODE_CTOR: u32 = 0x00a5_ecb0;
const NI_NODE_DTOR: u32 = 0x00a5_e930;
/// `NiAVObject::UpdateProperties` (Xbox PDB).
const NI_AV_OBJECT_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// Gives the scene node the debug objects are attached to (`this` is the
/// `TES`).
const TES_DEBUG_PARENT: u32 = 0x0096_11e0;
/// A form's id (the word the game keeps for it).
const FORM_ID_GETTER: u32 = 0x0084_e3a0;
/// Looks a form up by its id.
const FORM_LOOKUP: u32 = 0x0048_39c0;
/// `TESSaveLoadGame::UseSaveGameBlocks` (Xbox PDB).
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
/// The current write position (an address) of the save buffer.
const SAVE_BUFFER_POSITION: u32 = 0x0082_5c00;
/// Writes `(source, length)` bytes to the save buffer.
const SAVE_BUFFER_WRITE: u32 = 0x0085_79b0;
/// `TESSaveLoadGame::SaveNumericID` (Xbox PDB): `(&id, length)`.
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
/// The save version byte (`>= 0x32` adds a word to the save).
const SAVE_VERSION: u32 = 0x008d_f040;
/// Gives `this + 0x9C` (the list head the save code walks; takes the `TES`).
const SAVE_LIST_HEAD: u32 = 0x0050_d100;
/// Number of nodes with a non-null item in a list (`VATS::GetCount` in the
/// Xbox PDB, which names the same body).
const LIST_COUNT: u32 = 0x005a_e380;
/// `TESDataHandler` function `(handler, formId)` that says whether the id is
/// one the data handler knows.
const DATA_HANDLER_HAS_FORM: u32 = 0x0046_9860;
/// `TESDataHandler::GetAddonNode` (Xbox PDB) `(handler, index)`.
const DATA_HANDLER_GET_ADDON_NODE: u32 = 0x0046_17e0;
/// `TESDataHandler` interior-cell accessors: `PrepareInteriors`, count and
/// cell at an index.
const DATA_HANDLER_PREPARE_INTERIORS: u32 = 0x0046_19b0;
const DATA_HANDLER_INTERIOR_COUNT: u32 = 0x0046_1960;
const DATA_HANDLER_INTERIOR_AT: u32 = 0x0046_1980;
/// `TESObjectCELL::QueueReferences` (Xbox PDB) `(cell, flag)`.
const CELL_QUEUE_REFERENCES: u32 = 0x0055_71a0;
/// A `TESObjectCELL` function taking only `this`, called by the same walk.
const CELL_QUEUE_B: u32 = 0x0055_73e0;
/// `TESObjectCELL::AddFurnitureToList` (Xbox PDB) `(cell, list)`.
const CELL_ADD_FURNITURE_TO_LIST: u32 = 0x0054_ca00;
/// Cell function `0054ba00` (run on loaded cells of the buffers).
const CELL_FINISH: u32 = 0x0054_ba00;
/// `TES::IsCellLoaded` (Xbox PDB) `(tes, cell, flag)`.
const TES_IS_CELL_LOADED: u32 = 0x0045_11e0;
/// Cell state test (`fn_00450ff0` in the unit's main file).
const CELL_STATE_TEST: u32 = 0x0045_0ff0;
/// `TESWorldSpace::GetCellFromCellCoord` (Xbox PDB) `(worldSpace, x, y)`.
const WORLD_SPACE_GET_CELL: u32 = 0x0058_75a0;
/// `BGSSaveGameBuffer` / `BGSLoadGameBuffer` methods (Xbox PDB names where
/// the map has them).
const SAVE_BUFFER_START_VARIABLE: u32 = 0x0086_5f20;
const SAVE_BUFFER_SAVE_FORM_ID_POINTER: u32 = 0x0086_5df0;
const SAVE_BUFFER_WRITE_BYTES: u32 = 0x0086_5e50;
const SAVE_BUFFER_END_VARIABLE: u32 = 0x0086_5ff0;
const SAVE_BUFFER_SAVE_FORM_ID: u32 = 0x0086_5db0;
const LOAD_BUFFER_VARIABLE_SIZE: u32 = 0x0086_4a60;
const LOAD_BUFFER_FORM_ID: u32 = 0x0086_48a0;
const LOAD_BUFFER_FORM_ID_OUT: u32 = 0x0086_48e0;
const LOAD_BUFFER_READ_BYTES: u32 = 0x0086_4980;
/// `BGSSaveLoadGame::ClearFormID` (Xbox PDB).
const SAVE_LOAD_CLEAR_FORM_ID: u32 = 0x0084_a9c0;
/// The cell's slot `TES::LoadGame` hands the loaded id and `1`.
const CELL_SLOT_LOAD: u32 = 0x128;
/// `ModelLoader` methods (Xbox PDB names).
const MODEL_LOADER_QUEUE_MODEL: u32 = 0x0044_4040;
const MODEL_LOADER_QUEUE_FILE: u32 = 0x0044_3d30;
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080;
const MODEL_LOADER_RELEASE_MODEL: u32 = 0x0044_5300;
/// Test on a data handler add-on node that picks the `LoadFile` path.
const ADDON_NODE_LOADS_NOW: u32 = 0x0044_8bf0;
/// `QueuedFile::QueuedFile` (Xbox PDB) `(this, priority)`.
const QUEUED_FILE_CTOR: u32 = 0x00c3_c590;
/// `NiPointer<QueuedFile>::operator=` (Xbox PDB), `(cell, pointer)`.
const QUEUED_FILE_POINTER_ASSIGN: u32 = 0x006f_74f0;
/// `TES::CreateTextureImage` (Xbox PDB).
const CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;

// Translated from 00458b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the address of the constant at `01267e30` (the zero `hkVector4` the
/// pick helpers hand out).
pub fn fn_00458b20(_e: &mut Engine) -> Ptr {
    Ptr::new(0x0126_7e30)
}

// Translated from 00458b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte at `this + 0x11`.
pub fn fn_00458b30(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x11, value);
}

// Translated from 00458b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The stored pointer of the `NiPointer` at `this` (`00559450`).
pub fn fn_00458b50(e: &mut Engine, this: Ptr) -> u32 {
    ni_pointer_get(e, this)
}

// Translated from 00458b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::GetMapNameForLocation` (Xbox PDB): asks a world space (the one
/// passed, or the current one from `TES::GetWorldSpace`, `004fd3e0`) for the
/// map name of a location, through its slot `0x138`. Returns 0 without a
/// world space. The four words after `this` are passed on as they are: the
/// first alone, the next three being a 12-byte structure the game copies by
/// value on the stack.
pub fn tes_get_map_name_for_location(
    e: &mut Engine,
    this: Ptr<TES>,
    first: u32,
    structure_0: u32,
    structure_1: u32,
    structure_2: u32,
    world_space: Ptr,
) -> u32 {
    let world_space = if world_space.is_null() {
        e.call(GET_WORLD_SPACE, &args![this]).ptr::<()>()
    } else {
        world_space
    };
    if world_space.is_null() {
        return 0;
    }
    e.vcall(
        world_space.addr(),
        0x138,
        &args![first, structure_0, structure_1, structure_2],
    )
    .u32()
}

// Translated from 00458be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::GetCellPriority` (Xbox PDB): 1 for a cell near the centre of the
/// loaded grid, 3 for one farther out, cached for the last cell asked about.
///
/// The distance is how far the cell is, in cells, from `position` (the
/// position's `x` and `y` shifted down to cell units; the current grid
/// centre `iCurrentGridX`/`iCurrentGridY` when null; the middle of the grid
/// for an interior cell), less half of `uGridsToLoad`, in the larger of the
/// two axes. The class is `10 + 10 * distance` as a byte, above 20 giving 3.
pub fn tes_get_cell_priority(e: &mut Engine, this: Ptr<TES>, cell: Ptr, position: Ptr) -> u32 {
    if e.global::<u32>(CELL_PRIORITY_LAST_CELL) == cell.addr() {
        return e.global(CELL_PRIORITY_VALUE);
    }
    let offset_x: i32;
    let offset_y: i32;
    if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
        offset_x = (grids_to_load(e) >> 1) as i32;
        offset_y = (grids_to_load(e) >> 1) as i32;
    } else if !position.is_null() {
        let data_x = e.call(CELL_GET_DATA_X, &args![cell]).i32();
        let x = e.mem.f32(position.addr());
        let position_x = e.call(FLOAT_TO_INT_ROUNDED, &args![x]).i32() >> 12;
        let half = (grids_to_load(e) >> 1) as i32;
        offset_x = data_x.wrapping_sub(position_x.wrapping_sub(half));
        let data_y = e.call(CELL_GET_DATA_Y, &args![cell]).i32();
        let y = e.mem.f32(position.addr() + 4);
        let position_y = e.call(FLOAT_TO_INT_ROUNDED, &args![y]).i32() >> 12;
        let half = (grids_to_load(e) >> 1) as i32;
        offset_y = data_y.wrapping_sub(position_y.wrapping_sub(half));
    } else {
        let data_x = e.call(CELL_GET_DATA_X, &args![cell]).i32();
        let half = (grids_to_load(e) >> 1) as i32;
        let grid_x = e.get(this, TES::iCurrentGridX);
        offset_x = data_x.wrapping_sub(grid_x.wrapping_sub(half));
        let data_y = e.call(CELL_GET_DATA_Y, &args![cell]).i32();
        let half = (grids_to_load(e) >> 1) as i32;
        let grid_y = e.get(this, TES::iCurrentGridY);
        offset_y = data_y.wrapping_sub(grid_y.wrapping_sub(half));
    }
    let half = (grids_to_load(e) >> 1) as i32;
    let across = offset_x.wrapping_sub(half);
    let half = (grids_to_load(e) >> 1) as i32;
    let down = offset_y.wrapping_sub(half);
    let farther = if across > down { across } else { down };
    let distance = e.call(ABSOLUTE_VALUE, &args![farther]).i32();
    let scaled = distance.wrapping_mul(10).wrapping_add(10) as u8;
    let class = if scaled > 0x14 { 0x28u8 } else { 0x14 };
    match class {
        0x14 => e.set_global(CELL_PRIORITY_VALUE, 1u32),
        _ => e.set_global(CELL_PRIORITY_VALUE, 3u32),
    }
    e.set_global(CELL_PRIORITY_LAST_CELL, cell.addr());
    e.global(CELL_PRIORITY_VALUE)
}

// Translated from 00458e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::AddTempDebugObject` (Xbox PDB): for a non-null `node`, builds a
/// `BSTempNode` (0xB0 bytes, `fn_00458ef0`) carrying `value` at `+0xAC`,
/// attaches `node` to it (slot `0xDC`, second argument 1), attaches the
/// temp node to the node `009611e0` gives for `this`, and updates the temp
/// node's properties.
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn tes_add_temp_debug_object(e: &mut Engine, this: Ptr<TES>, node: Ptr, value: f32) {
    if node.is_null() {
        return;
    }
    let memory = e.call(NI_OPERATOR_NEW, &args![0xb0u32]).u32();
    let temp_node = if memory == 0 {
        0
    } else {
        fn_00458ef0(e, Ptr::new(memory), value).addr()
    };
    e.vcall(temp_node, 0xdc, &args![node, 1u32]);
    let parent = e.call(TES_DEBUG_PARENT, &args![this]).u32();
    e.vcall(parent, 0xdc, &args![temp_node, 1u32]);
    e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![temp_node]);
}

// Translated from 00458ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSTempNode` constructor (vtable `01017d24`): `NiNode::NiNode(0)`,
/// then the vtable and the float at `+0xAC`. Returns `this`.
pub fn fn_00458ef0(e: &mut Engine, this: Ptr, value: f32) -> Ptr {
    e.call(NI_NODE_CTOR, &args![this, 0u32]);
    e.mem.set_u32(this.addr(), BS_TEMP_NODE_VTABLE);
    e.mem.set_f32(this.addr() + 0xac, value);
    this
}

// Translated from 00458f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempNode::GetRTTI` (Xbox PDB): the class's `NiRTTI`.
pub fn bs_temp_node_get_rtti(_e: &mut Engine, _this: Ptr) -> Ptr {
    Ptr::new(BS_TEMP_NODE_RTTI)
}

// Translated from 00458f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempNode::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and, when bit 0 of `flags` is set, frees the 0xB0-byte
/// object. Returns `this`.
pub fn bs_temp_node_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00458f70(e, this);
    if flags & 1 != 0 {
        e.call(NI_OPERATOR_DELETE, &args![this, 0xb0u32]);
    }
    this
}

// Translated from 00458f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSTempNode` destructor: restores the vtable and runs the `NiNode`
/// destructor (`00a5e930`).
pub fn fn_00458f70(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BS_TEMP_NODE_VTABLE);
    e.call(NI_NODE_DTOR, &args![this]);
}

// Translated from 00458f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the 8-byte entries (form pointer, count) of the list at `this +
/// 0x9C` (`TES::DeadCount`) and then empties the list (`00470470`). The walk
/// stops at the first node whose item word is null.
pub fn fn_00458f90(e: &mut Engine, this: Ptr<TES>) {
    let list = this.addr() + 0x9c;
    let mut node = list;
    while node != 0 {
        if list_node_item(e, node) == 0 {
            break;
        }
        let entry = list_node_item(e, node);
        e.call(OPERATOR_DELETE, &args![entry]);
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    e.call(0x0047_0470, &args![list]);
}

// Translated from 00459000 (decompiled, FalloutNV.exe 1.4.0.525)
/// The count kept for `form` in the list at `this + 0x9C`
/// (`TES::DeadCount`), or 0 when it has no entry. Each entry is 8 bytes: the
/// form pointer, then a 16-bit count.
pub fn fn_00459000(e: &mut Engine, this: Ptr<TES>, form: u32) -> u16 {
    let mut node = this.addr() + 0x9c;
    while node != 0 {
        if list_node_item(e, node) == 0 {
            break;
        }
        let entry = list_node_item(e, node);
        if e.mem.u32(entry) == form {
            return e.mem.u16(entry + 4);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    0
}

// Translated from 00459060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `delta` to the count kept for `form` in the list at `this + 0x9C`
/// (`TES::DeadCount`), or puts a new 8-byte entry (form, `delta`) at the head
/// of the list when it has none.
pub fn fn_00459060(e: &mut Engine, this: Ptr<TES>, form: u32, delta: i16) {
    let list = this.addr() + 0x9c;
    let mut node = list;
    while node != 0 {
        if list_node_item(e, node) == 0 {
            break;
        }
        let entry = list_node_item(e, node);
        if e.mem.u32(entry) == form {
            let count = e.mem.i16(entry + 4) as i32 + delta as i32;
            e.mem.set_u16(entry + 4, count as u16);
            return;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    let entry = e.call(OPERATOR_NEW, &args![8u32]).u32();
    e.mem.set_u32(entry, form);
    e.mem.set_u16(entry + 4, delta as u16);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry);
        e.call(LIST_ADD_HEAD, &args![list, slot]);
    });
}

/// Logs the byte count of a save step the way `fn_00459100` and
/// `tes_save_game` do when the log byte is set: with the current world
/// space's form id, name and flags when there is one (`TES::GetWorldSpace`
/// is called with the save object as `this`, as the game does), else a
/// shorter line.
fn log_save_bytes(
    e: &mut Engine,
    save_load: u32,
    bytes: u32,
    line: u32,
    format_plain: u32,
    format_with_form: u32,
) {
    let world_space = e.call(GET_WORLD_SPACE, &args![save_load]).u32();
    if world_space == 0 {
        e.call(ERROR_LOG, &args![format_plain, bytes, line, TES_CPP_PATH]);
    } else {
        let form_id = e.mem.u32(world_space);
        let form = e.call(FORM_LOOKUP, &args![form_id]).u32();
        let flags = e.mem.u32(world_space + 5);
        let name = e.vcall(form, FORM_SLOT_NAME, &args![]).u32();
        e.call(
            ERROR_LOG,
            &args![
                format_with_form,
                bytes,
                form_id,
                name,
                flags,
                line,
                TES_CPP_PATH
            ],
        );
    }
}

// Translated from 00459100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The size, in bytes, the save of this class will take (its own error
/// message calls it `GetSaveSize()`): 6 when the save uses blocks, 4 for the
/// list count, 6 for each entry of the list `0050d100` gives (`this + 0x9C`), 4 more when the
/// save version (`008df040`) is at least 0x32. When the log byte setting is
/// set, the size is logged (see `log_save_bytes`).
pub fn fn_00459100(e: &mut Engine, this: Ptr<TES>) -> u16 {
    let save_load: u32 = e.global(SAVE_LOAD_GAME);
    let mut size: u16 = 0;
    let start = size;
    if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool() {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    let list = e.call(SAVE_LIST_HEAD, &args![this]).u32();
    size = size.wrapping_add(4);
    let count = e.call(LIST_COUNT, &args![list]).u32();
    size = (size as u32).wrapping_add(count.wrapping_mul(6)) as u16;
    if e.call(SAVE_VERSION, &args![save_load]).u8() >= 0x32 {
        size = size.wrapping_add(4);
    }
    if setting_byte(e, SETTING_SAVE_SIZE_LOG) != 0 {
        let bytes = (size as u32).wrapping_sub(start as u32);
        log_save_bytes(e, save_load, bytes, 0x1882, 0x0101_2c78, 0x0101_2cb0);
    }
    size
}

// Translated from 00459230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::SaveGame` (Xbox PDB): writes this class's part of the save: when
/// the save uses blocks, the tag `BLOK` and a 16-bit size placeholder; the
/// length of the list `0050d100` gives (`this + 0x9C`) and, per entry, the numeric form id
/// and the 16-bit count; the word at `011c3c08` when the save version is at
/// least 0x32; and, at the end, the size of the block, patched into the
/// placeholder (a message is printed when it exceeds 16 bits). When the log
/// byte setting is set the number of bytes written is logged.
pub fn tes_save_game(e: &mut Engine, this: Ptr<TES>) {
    let save_load: u32 = e.global(SAVE_LOAD_GAME);
    e.with_stack(16, |e, scratch| {
        let tag = scratch.addr();
        let placeholder = scratch.addr() + 4;
        let count_cell = scratch.addr() + 8;
        let id_cell = scratch.addr() + 12;
        e.mem.set_u16(placeholder, 0);
        let mut block = 0u32;
        let mut start = e.call(SAVE_BUFFER_POSITION, &args![save_load]).u32();
        if setting_byte(e, SETTING_SAVE_SIZE_LOG) != 0 {
            start = e.call(SAVE_BUFFER_POSITION, &args![save_load]).u32();
        }
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool() {
            e.mem.set_u32(tag, 0x424c_4f4b);
            e.call(SAVE_BUFFER_WRITE, &args![save_load, tag, 4u32]);
            block = e.call(SAVE_BUFFER_POSITION, &args![save_load]).u32();
            e.call(SAVE_BUFFER_WRITE, &args![save_load, placeholder, 2u32]);
        }
        let mut node = e.call(SAVE_LIST_HEAD, &args![this]).u32();
        let count = e.call(LIST_COUNT, &args![node]).u32();
        e.mem.set_u32(count_cell, count);
        e.call(SAVE_BUFFER_WRITE, &args![save_load, count_cell, 4u32]);
        while node != 0 {
            let entry = list_node_item(e, node);
            if entry != 0 {
                e.mem.set_u32(id_cell, 0);
                let form = e.mem.u32(entry);
                if form != 0 {
                    let id = e.call(FORM_ID_GETTER, &args![form]).u32();
                    e.mem.set_u32(id_cell, id);
                }
                e.call(SAVE_NUMERIC_ID, &args![save_load, id_cell, 4u32]);
                e.call(SAVE_BUFFER_WRITE, &args![save_load, entry + 4, 2u32]);
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
        if e.call(SAVE_VERSION, &args![save_load]).u8() >= 0x32 {
            e.call(SAVE_BUFFER_WRITE, &args![save_load, SAVE_EXTRA_WORD, 4u32]);
        }
        if setting_byte(e, SETTING_SAVE_SIZE_LOG) != 0 {
            let end = e.call(SAVE_BUFFER_POSITION, &args![save_load]).u32();
            log_save_bytes(
                e,
                save_load,
                end.wrapping_sub(start),
                0x18e1,
                0x0101_536c,
                0x0101_53a0,
            );
        }
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool() {
            let end = e.call(SAVE_BUFFER_POSITION, &args![save_load]).u32();
            if end > block.wrapping_add(0xffff) {
                e.call(DEBUG_PRINT, &args![0x0101_5318u32, TES_CPP_PATH, 0x18e1u32]);
            }
            e.mem.set_u16(block, end.wrapping_sub(block) as u16);
        }
    });
}

// Translated from 00459470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `index`th slot of `pExteriorBuffer` (`TES + 0x3C`).
pub fn fn_00459470(e: &mut Engine, this: Ptr<TES>, index: u32) -> u32 {
    let buffer = e.get(this, TES::pExteriorBuffer);
    e.mem.u32(buffer.addr().wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00459490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::SaveGame_ov2` (Xbox PDB): the save of this class into a
/// `BGSSaveGameBuffer`: a variable-sized block with the entries of the list
/// at `this + 0x9C` (form id and 16-bit count each), `uGridsToLoad`, and then
/// one form id per cell of the grid (0 for an empty slot or a cell the data
/// handler does not know).
pub fn tes_save_game_ov2(e: &mut Engine, this: Ptr<TES>, buffer: Ptr) {
    let mut entries = 0u32;
    let start = e.call(SAVE_BUFFER_START_VARIABLE, &args![buffer]).u32();
    let mut node = this.addr() + 0x9c;
    while node != 0 {
        let entry = list_node_item(e, node);
        if entry != 0 {
            let form = e.mem.u32(entry);
            e.call(SAVE_BUFFER_SAVE_FORM_ID_POINTER, &args![buffer, form, 0u32]);
            e.call(
                SAVE_BUFFER_WRITE_BYTES,
                &args![buffer, entry + 4, 2u32, 0u32],
            );
            entries += 1;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    e.call(SAVE_BUFFER_END_VARIABLE, &args![buffer, entries, start]);
    let grids = grids_to_load(e);
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), grids);
        e.call(SAVE_BUFFER_WRITE_BYTES, &args![buffer, cell, 4u32, 0u32]);
    });
    let mut x = 0u32;
    while x < grids {
        let mut y = 0u32;
        while y < grids {
            let grid_cells = e.get(this, TES::pGridCellA);
            let slot = e.call(GRID_CELL_ARRAY_GET, &args![grid_cells, x, y]).u32();
            let cell = e.mem.u32(slot);
            let mut id = 0u32;
            if cell != 0 {
                let cell_id = e.call(FORM_ID_GETTER, &args![cell]).u32();
                let handler: u32 = e.global(DATA_HANDLER);
                if e.call(DATA_HANDLER_HAS_FORM, &args![handler, cell_id])
                    .bool()
                {
                    id = e.call(FORM_ID_GETTER, &args![cell]).u32();
                }
            }
            e.call(SAVE_BUFFER_SAVE_FORM_ID, &args![buffer, id, 0u32]);
            y += 1;
        }
        x += 1;
    }
}

// Translated from 004595e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::LoadGame` (Xbox PDB): reads back what `tes_save_game_ov2` wrote:
/// the list of (form, count) entries (each form looked up by its loaded id
/// and dynamic-cast, `__RTDynamicCast`), then `uGridsToLoad` and one form id
/// per grid cell; a non-empty id for a cell the data handler knows is
/// cleared (`BGSSaveLoadGame::ClearFormID`) and handed to the cell's slot
/// `0x128` with 1.
pub fn tes_load_game(e: &mut Engine, this: Ptr<TES>, buffer: Ptr) {
    let count = e.call(LOAD_BUFFER_VARIABLE_SIZE, &args![buffer]).u32();
    let mut index = 0u32;
    while index < count {
        let entry = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let id = e.call(LOAD_BUFFER_FORM_ID, &args![buffer]).u32();
        let form = e.call(FORM_LOOKUP, &args![id]).u32();
        let cast = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, RTTI_LOAD_SOURCE, RTTI_LOAD_TARGET, 0u32],
            )
            .u32();
        e.mem.set_u32(entry, cast);
        e.call(LOAD_BUFFER_READ_BYTES, &args![buffer, entry + 4, 2u32]);
        let list = this.addr() + 0x9c;
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), entry);
            e.call(LIST_ADD_HEAD, &args![list, slot]);
        });
        index += 1;
    }
    let grids = e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), 0);
        e.call(LOAD_BUFFER_READ_BYTES, &args![buffer, cell, 4u32]);
        e.mem.u32(cell.addr())
    });
    let mut x = 0u32;
    while x < grids {
        let mut y = 0u32;
        while y < grids {
            let grid_cells = e.get(this, TES::pGridCellA);
            let slot = e.call(GRID_CELL_ARRAY_GET, &args![grid_cells, x, y]).u32();
            let cell = e.mem.u32(slot);
            let id = e.with_stack(4, |e, id_cell| {
                e.mem.set_u32(id_cell.addr(), 0);
                e.call(LOAD_BUFFER_FORM_ID_OUT, &args![buffer, id_cell]);
                e.mem.u32(id_cell.addr())
            });
            if cell != 0 && id != 0 {
                let cell_id = e.call(FORM_ID_GETTER, &args![cell]).u32();
                let handler: u32 = e.global(DATA_HANDLER);
                if e.call(DATA_HANDLER_HAS_FORM, &args![handler, cell_id])
                    .bool()
                {
                    let script_context: u32 = e.global(SCRIPT_CONTEXT);
                    e.call(SAVE_LOAD_CLEAR_FORM_ID, &args![script_context, id]);
                    e.vcall(cell, CELL_SLOT_LOAD, &args![id, 1u32]);
                }
            }
            y += 1;
        }
        x += 1;
    }
}

// Translated from 00459750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finishes the cells held in `pInteriorBuffer` and `pExteriorBuffer`
/// (`TES + 0x38` and `+0x3C`, as many slots as `uInterior Cell Buffer` and
/// `uExterior Cell Buffer` say): for each non-null slot whose cell
/// `TES::IsCellLoaded` (`004511e0`, second argument 1) says is loaded,
/// `0054ba00` runs on it. Then the `NiPointer` at `+0xC0`
/// (`spLoadedAreaBound`) is read and handed to `00628da0`.
pub fn fn_00459750(e: &mut Engine, this: Ptr<TES>) {
    for (buffer_field, setting) in [
        (TES::pInteriorBuffer, SETTING_INTERIOR_CELL_BUFFER),
        (TES::pExteriorBuffer, SETTING_EXTERIOR_CELL_BUFFER),
    ] {
        let mut index = 0i32;
        loop {
            let slots = e.call(SETTING_VALUE_ADDRESS_INT, &args![setting]).u32();
            if index >= e.mem.i32(slots) {
                break;
            }
            let buffer = e.get(this, buffer_field);
            let at = buffer.addr().wrapping_add((index as u32).wrapping_mul(4));
            let cell = e.mem.u32(at);
            if cell != 0 {
                let tes: u32 = e.global(TES_SINGLETON);
                if e.call(TES_IS_CELL_LOADED, &args![tes, cell, 1u32]).bool() {
                    let buffer = e.get(this, buffer_field);
                    let at = buffer.addr().wrapping_add((index as u32).wrapping_mul(4));
                    let cell = e.mem.u32(at);
                    e.call(CELL_FINISH, &args![cell]);
                }
            }
            index += 1;
        }
    }
    let bound = ni_pointer_get(e, Ptr::new(this.addr() + 0xc0));
    e.call(0x0062_8da0, &args![bound]);
}

// Translated from 00459840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the `TES::DeadCount` list (`fn_00458f90`) and sets `iSaveGridX`
/// and `iSaveGridY` (`+0x48`, `+0x4C`) to `0x7fffffff`.
pub fn fn_00459840(e: &mut Engine, this: Ptr<TES>) {
    fn_00458f90(e, this);
    e.set(this, TES::iSaveGridX, 0x7fff_ffff);
    e.set(this, TES::iSaveGridY, 0x7fff_ffff);
}

// Translated from 00459870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::CreateFurnitureList` (Xbox PDB): fills the list at `this + 0x94`
/// (`ListofBedsAndChairs`) with the furniture of the interior cell
/// (`TESObjectCELL::AddFurnitureToList`), or of every slot of the grid when
/// there is none. The grid slots are not checked for null.
pub fn tes_create_furniture_list(e: &mut Engine, this: Ptr<TES>) {
    let list = this.addr() + 0x94;
    let interior = e.get(this, TES::pInteriorCell);
    if !interior.is_null() {
        e.call(CELL_ADD_FURNITURE_TO_LIST, &args![interior, list]);
        return;
    }
    let mut x = 0u32;
    while x < grids_to_load(e) {
        let mut y = 0u32;
        while y < grids_to_load(e) {
            let grid_cells = e.get(this, TES::pGridCellA);
            let slot = e.call(GRID_CELL_ARRAY_GET, &args![grid_cells, x, y]).u32();
            let cell = e.mem.u32(slot);
            e.call(CELL_ADD_FURNITURE_TO_LIST, &args![cell, list]);
            y += 1;
        }
        x += 1;
    }
}

/// Runs `action` on the interior cell (when there is one and
/// `fn_00450ff0` accepts it) and on every non-null grid cell `fn_00450ff0`
/// accepts, the way `fn_00459920` and `fn_00459a00` do. Does nothing
/// without a `TESDataHandler`.
fn for_each_ready_cell(e: &mut Engine, this: Ptr<TES>, action: fn(&mut Engine, u32)) {
    let handler: u32 = e.global(DATA_HANDLER);
    if handler == 0 {
        return;
    }
    if e.call(GET_INTERIOR_CELL, &args![this]).u32() != 0 {
        let interior = e.call(GET_INTERIOR_CELL, &args![this]).u32();
        if e.call(CELL_STATE_TEST, &args![interior]).bool() {
            let interior = e.call(GET_INTERIOR_CELL, &args![this]).u32();
            action(e, interior);
        }
    }
    let mut x = 0u32;
    while x < grids_to_load(e) {
        let mut y = 0u32;
        while y < grids_to_load(e) {
            let grid_cells = e.get(this, TES::pGridCellA);
            let slot = e.call(GRID_CELL_ARRAY_GET, &args![grid_cells, x, y]).u32();
            let cell = e.mem.u32(slot);
            if cell != 0 && e.call(CELL_STATE_TEST, &args![cell]).bool() {
                action(e, cell);
            }
            y += 1;
        }
        x += 1;
    }
}

// Translated from 00459920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues the references (`TESObjectCELL::QueueReferences`, second argument
/// 0) of the interior cell and of every grid cell whose state test
/// (`fn_00450ff0`) passes. Does nothing without a `TESDataHandler`.
pub fn fn_00459920(e: &mut Engine, this: Ptr<TES>) {
    for_each_ready_cell(e, this, |e, cell| {
        e.call(CELL_QUEUE_REFERENCES, &args![cell, 0u32]);
    });
}

// Translated from 00459a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Same walk as `fn_00459920`, calling `005573e0` on each cell.
pub fn fn_00459a00(e: &mut Engine, this: Ptr<TES>) {
    for_each_ready_cell(e, this, |e, cell| {
        e.call(CELL_QUEUE_B, &args![cell]);
    });
}

// Translated from 00459ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::RunCellTest` (Xbox PDB): the cell test run: sets `bRunningCellTests`
/// (`+0x51`), turns on the player's god mode, logs "Running Cell Test",
/// closes the menus, and then takes every world space of the data handler in
/// turn (stopping when the list node is empty, `008256d0`): enters it
/// (`TES::SetWorldSpace`), logs its form id, name and cell bounds, walks the
/// bounds in steps of `uGridsToLoad` cells (loading cells that are not there)
/// and tests each cell (`TES::TestCell`, `tes_test_cell`). Then the interior
/// cells of the data handler are tested the same way. `bRunningCellTests2`
/// (`+0x52`) is cleared at the end.
pub fn tes_run_cell_test(e: &mut Engine, this: Ptr<TES>, mode: u32) {
    e.set(this, TES::bRunningCellTests, true);
    e.call(0x0095_26a0, &args![1u32]);
    e.call(ERROR_LOG, &args![0x0101_7e5cu32]);
    e.call(0x0070_37e0, &args![]);
    let start = fn_00457fe0(e);
    let handler: u32 = e.global(DATA_HANDLER);
    let mut node = e
        .call(DATA_HANDLER_FIRST_WORLD_SPACE, &args![handler])
        .u32();
    while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
        let world_space = list_node_item(e, node);
        if world_space != 0 {
            tes_set_world_space(e, this, Ptr::new(world_space));
            let left = world_space_bound_in_cells(e, 0x0045_65f0, world_space);
            let bottom = world_space_bound_in_cells(e, 0x0081_2870, world_space);
            let right = world_space_bound_in_cells(e, 0x009b_88a0, world_space);
            let top = world_space_bound_in_cells(e, 0x009b_88c0, world_space);
            let mut name = e.call(STRING_OBJECT_TEXT, &args![world_space + 0x18]).u32();
            if name == 0 || e.call(STRING_LENGTH, &args![name]).u32() == 0 {
                name = e.vcall(world_space, FORM_SLOT_NAME, &args![]).u32();
            }
            let form_id = e.call(FORM_ID_GETTER, &args![world_space]).u32();
            e.call(
                ERROR_LOG,
                &args![0x0101_7e24u32, form_id, name, left, bottom, right, top],
            );
            let mut x = left;
            while x < right {
                let mut y = bottom;
                while y < top {
                    let mut cell = e
                        .call(WORLD_SPACE_GET_CELL, &args![world_space, x, y])
                        .u32();
                    if cell == 0 {
                        cell = e
                            .call(WORLD_SPACE_LOAD_CELL, &args![world_space, x, y])
                            .u32();
                    }
                    if cell != 0 {
                        tes_test_cell(e, this, Ptr::new(cell), mode, start);
                    }
                    y = y.wrapping_add(grids_to_load(e) as i32);
                }
                x = x.wrapping_add(grids_to_load(e) as i32);
            }
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    let handler: u32 = e.global(DATA_HANDLER);
    e.call(DATA_HANDLER_PREPARE_INTERIORS, &args![handler]);
    let handler: u32 = e.global(DATA_HANDLER);
    let count = e.call(DATA_HANDLER_INTERIOR_COUNT, &args![handler]).u32();
    let mut index = 0u32;
    while index < count {
        let handler: u32 = e.global(DATA_HANDLER);
        let cell = e
            .call(DATA_HANDLER_INTERIOR_AT, &args![handler, index])
            .u32();
        tes_test_cell(e, this, Ptr::new(cell), mode, start);
        index += 1;
    }
    e.set(this, TES::bRunningCellTests2, false);
}

// Translated from 00459d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::TestCell` (Xbox PDB): for a non-null cell, logs "Moving to
/// interior/exterior cell ...", centres the player on it
/// (`PlayerCharacter::CenterOnCell`), loads the queued files
/// (`IOManager::LoadQueuedPriority`), logs how long the load took and the
/// total test time since `start_time` (`hh:mm:ss`), updates the process lists
/// (`ProcessLists::ChangeProcessLevelTempList`, `UpdateProcessLists`,
/// `ChangeProcessLevelTempList`), and for modes 4 and 5 runs
/// `TESSaveLoadGame::TestAllCells(mode)`. (`this` is not used.)
pub fn tes_test_cell(e: &mut Engine, _this: Ptr<TES>, cell: Ptr, mode: u32, start_time: u32) {
    if cell.is_null() {
        return;
    }
    if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
        let name = e.call(STRING_OBJECT_TEXT, &args![cell.addr() + 0x18]).u32();
        let form_id = e.call(FORM_ID_GETTER, &args![cell]).u32();
        e.call(ERROR_LOG, &args![0x0101_7f54u32, form_id, name]);
    } else {
        let name = e.call(STRING_OBJECT_TEXT, &args![cell.addr() + 0x18]).u32();
        let data_y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
        let data_x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
        let form_id = e.call(FORM_ID_GETTER, &args![cell]).u32();
        e.call(
            ERROR_LOG,
            &args![0x0101_7f2cu32, form_id, data_x, data_y, name],
        );
    }
    let before = fn_00457fe0(e);
    let player: u32 = e.global(PLAYER_CHARACTER);
    e.call(0x0093_db60, &args![player, 0u32, cell]);
    let io_manager: u32 = e.global(IO_MANAGER);
    e.call(0x0045_6520, &args![io_manager]);
    let after = fn_00457fe0(e);
    let total = fn_00457fe0(e).wrapping_sub(start_time);
    let hours = total / 3_600_000;
    let rest = total - hours * 3_600_000;
    let minutes = rest / 60_000;
    let rest = rest - minutes * 60_000;
    let seconds = rest / 1000;
    let loading = after.wrapping_sub(before) as f64 / e.mem.f64(0x0101_7b70);
    if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
        e.call(
            ERROR_LOG,
            &args![0x0101_7ed8u32, loading, hours, minutes, seconds],
        );
    } else {
        let data_y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
        let data_x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
        e.call(
            ERROR_LOG,
            &args![
                0x0101_7e78u32,
                data_x,
                data_y,
                loading,
                hours,
                minutes,
                seconds
            ],
        );
    }
    let delay = e.mem.f32(0x0101_7e70);
    e.call(0x0086_7a40, &args![OBJECT_011DE7B8, delay]);
    e.call(0x0096_eb40, &args![OBJECT_011E0E80]);
    e.call(0x0096_d810, &args![OBJECT_011E0E80]);
    e.call(0x0096_eb40, &args![OBJECT_011E0E80]);
    if (4..=5).contains(&mode) {
        let save_load: u32 = e.global(SAVE_LOAD_GAME);
        e.call(0x0086_1f20, &args![save_load, mode]);
    }
}

/// Allocates a `QueuedFile` of priority 5 (`operator new` 0x28 bytes and
/// `QueuedFile::QueuedFile`, or null when the allocation fails) and stores it
/// in the `NiPointer` at `slot`.
fn store_new_queued_file(e: &mut Engine, slot: u32) {
    let memory = e.call(OPERATOR_NEW, &args![0x28u32]).u32();
    let queued = if memory == 0 {
        0
    } else {
        e.call(QUEUED_FILE_CTOR, &args![memory, 5u32]).u32()
    };
    e.call(QUEUED_FILE_POINTER_ASSIGN, &args![slot, queued]);
}

// Translated from 00459f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::PreloadAddonNodes` (Xbox PDB): unless `spPreloadedAddonNodes`
/// (`+0xA4`) is already set, makes a `QueuedFile` (priority 5) for it and
/// goes through the data handler's add-on nodes (`fn_0045a120` many). For a
/// node `00448bf0` accepts, the model named by slot `0x14` of the node's
/// sub-object (`node + 0x30`) is loaded (`ModelLoader::LoadFile`) and, when it
/// loads, given the node's data (`0068a830`, `0059e300`, the word at
/// `node + 0x58`) and attached to the scene root `fn_0045a190` returns; the
/// model is then released (`ModelLoader::ReleaseModel`). For any other node
/// the model is queued on the `QueuedFile`.
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn tes_preload_addon_nodes(e: &mut Engine, this: Ptr<TES>) {
    let slot = this.addr() + 0xa4;
    if ni_pointer_get(e, Ptr::new(slot)) != 0 {
        return;
    }
    e.with_stack(SCOPE_GUARD_SIZE, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 7u32, 1u32, TES_CPP_PATH, 0x1b8eu32],
        );
        store_new_queued_file(e, slot);
        let handler: u32 = e.global(DATA_HANDLER);
        let count = fn_0045a120(e, Ptr::new(handler));
        let mut index = 0u32;
        while index < count {
            let handler: u32 = e.global(DATA_HANDLER);
            let node = e
                .call(DATA_HANDLER_GET_ADDON_NODE, &args![handler, index])
                .u32();
            if node != 0 {
                if e.call(ADDON_NODE_LOADS_NOW, &args![node]).bool() {
                    let name = e.vcall(node + 0x30, 0x14, &args![]).u32();
                    let model_loader: u32 = e.global(MODEL_LOADER);
                    let model = e
                        .call(
                            MODEL_LOADER_LOAD_FILE,
                            &args![model_loader, name, 0u32, 1u32, 0u32, 0u32, 0u32],
                        )
                        .u32();
                    if model != 0 {
                        let first = e.call(0x0068_a830, &args![node]).u32();
                        e.call(0x0040_30d0, &args![model, first]);
                        let second = e.call(0x0059_e300, &args![node]).u32();
                        e.call(0x0070_5b10, &args![model, second]);
                        let word = fn_0045a1a0(e, Ptr::new(node));
                        fn_0045a140(e, Ptr::new(model), word);
                        let root = fn_0045a190(e);
                        fn_0045a160(e, Ptr::new(root), Ptr::new(model));
                    }
                    let name = e.vcall(node + 0x30, 0x14, &args![]).u32();
                    let model_loader: u32 = e.global(MODEL_LOADER);
                    e.call(
                        MODEL_LOADER_RELEASE_MODEL,
                        &args![model_loader, name, 1u32, 1u32],
                    );
                } else {
                    let queued = ni_pointer_get(e, Ptr::new(slot));
                    let model_loader: u32 = e.global(MODEL_LOADER);
                    e.call(
                        MODEL_LOADER_QUEUE_FILE,
                        &args![
                            model_loader,
                            node + 0x30,
                            5u32,
                            queued,
                            0u32,
                            1u32,
                            0u32,
                            0u32
                        ],
                    );
                }
            }
            index += 1;
        }
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
    });
}

// Translated from 0045a120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands `this + 0x1EC` (the data handler's add-on node list) to `00658930`
/// and returns its result (the number of add-on nodes).
pub fn fn_0045a120(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0065_8930, &args![this.addr() + 0x1ec]).u32()
}

// Translated from 0045a140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a 16-bit value at `this + 0xBC`.
pub fn fn_0045a140(e: &mut Engine, this: Ptr, value: u16) {
    e.mem.set_u16(this.addr() + 0xbc, value);
}

// Translated from 0045a160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls slot `0xDC` of `this` with `(child, 1)`.
pub fn fn_0045a160(e: &mut Engine, this: Ptr, child: Ptr) {
    e.vcall(this.addr(), 0xdc, &args![child, 1u32]);
}

// Translated from 0045a190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The stored pointer of the `NiPointer` at `011ded58`.
pub fn fn_0045a190(e: &mut Engine) -> u32 {
    ni_pointer_get(e, Ptr::new(ADDON_PARENT_CELL))
}

// Translated from 0045a1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit word at `this + 0x58`.
pub fn fn_0045a1a0(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0x58)
}

// Translated from 0045a1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears `spPreloadedAddonNodes` (`+0xA4`).
pub fn fn_0045a1c0(e: &mut Engine, this: Ptr<TES>) {
    e.call(QUEUED_FILE_POINTER_ASSIGN, &args![this.addr() + 0xa4, 0u32]);
}

// Translated from 0045a1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::PreloadForms` (Xbox PDB): unless `spPreloadedForms` (`+0xAC`) is
/// already set, makes a `QueuedFile` (priority 5) for it, asks every entry of
/// the data handler's list at `+0x180` (until a node reports empty,
/// `008256d0`) to queue its files (slot `0x10` of the sub-object at
/// `entry + 0x30`, with `(5, queuedFile)`), and, when the weapon object at
/// `011ca278` exists, `TESObjectWEAP::QueueFiles(5, queuedFile, 0)`.
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn tes_preload_forms(e: &mut Engine, this: Ptr<TES>) {
    let slot = this.addr() + 0xac;
    if ni_pointer_get(e, Ptr::new(slot)) != 0 {
        return;
    }
    e.with_stack(SCOPE_GUARD_SIZE, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 7u32, 1u32, TES_CPP_PATH, 0x1bd0u32],
        );
        store_new_queued_file(e, slot);
        let handler: u32 = e.global(DATA_HANDLER);
        let mut node = fn_0045a330(e, Ptr::new(handler));
        while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
            let entry = list_node_item(e, node);
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
            if entry != 0 {
                let queued = ni_pointer_get(e, Ptr::new(slot));
                e.vcall(entry + 0x30, 0x10, &args![5u32, queued]);
            }
        }
        let weapons: u32 = e.global(WEAPON_QUEUE_OBJECT);
        if weapons != 0 {
            let queued = ni_pointer_get(e, Ptr::new(slot));
            e.call(0x0052_5510, &args![weapons, 5u32, queued, 0u32]);
        }
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
    });
}

// Translated from 0045a330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this + 0x180` (the data handler's list head at that offset).
pub fn fn_0045a330(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x180)
}

// Translated from 0045a350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears `spPreloadedForms` (`+0xAC`).
pub fn fn_0045a350(e: &mut Engine, this: Ptr<TES>) {
    e.call(QUEUED_FILE_POINTER_ASSIGN, &args![this.addr() + 0xac, 0u32]);
}

/// The text of a string setting (`00403df0`).
fn setting_text(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_VALUE_ADDRESS_STRING, &args![setting]).u32()
}

/// The four string settings that name the models to preload, in the order
/// the code visits them.
const PRELOAD_MODEL_SETTINGS: [u32; 4] = [
    SETTING_RAGDOLL_DATA_DEFAULT,
    SETTING_BLOOD_PARTICLE_DEFAULT,
    SETTING_SPLASH_PARTICLES,
    SETTING_EXPLOSION_SPLASH_PARTICLES,
];

// Translated from 0045a370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues the models named by `sRagdollDataDefault`, `sBloodParticleDefault`,
/// `sSplashParticles` and `sExplosionSplashParticles` (each when its text is
/// not empty; `ModelLoader::QueueModel(name, 5, 0, 0, 1, 1, 0)`) and loads
/// the blood texture named by `sBloodTextureDefault` as `Data\Textures\<name>`
/// into `BloodDecalPreload1` (`+0xA8`, `TES::CreateTextureImage`).
///
/// Not translated: the compiler's exception-unwinding frame and the stack
/// cookie check.
pub fn fn_0045a370(e: &mut Engine, this: Ptr<TES>) {
    e.with_stack(SCOPE_GUARD_SIZE, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 7u32, 1u32, TES_CPP_PATH, 0x1bf0u32],
        );
        for setting in PRELOAD_MODEL_SETTINGS {
            let text = setting_text(e, setting);
            if e.mem.i8(text) != 0 {
                let name = setting_text(e, setting);
                let model_loader: u32 = e.global(MODEL_LOADER);
                e.call(
                    MODEL_LOADER_QUEUE_MODEL,
                    &args![model_loader, name, 5u32, 0u32, 0u32, 1u32, 1u32, 0u32],
                );
            }
        }
        let text = setting_text(e, SETTING_BLOOD_TEXTURE_DEFAULT);
        if e.mem.i8(text) != 0 {
            let name = setting_text(e, SETTING_BLOOD_TEXTURE_DEFAULT);
            e.with_stack(0x104, |e, buffer| {
                e.call(
                    FORMAT_STRING,
                    &args![
                        buffer,
                        0x104u32,
                        0x0101_7f74u32,
                        0x0101_7f80u32,
                        0x0101_7f88u32,
                        name
                    ],
                );
                e.call(
                    CREATE_TEXTURE_IMAGE,
                    &args![this, buffer, this.addr() + 0xa8, 0u32, 0u32],
                );
            });
        }
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
    });
}

// Translated from 0045a520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the four models `fn_0045a370` queued (`fn_0045a5e0`, each when
/// its setting text is not empty) and clears `BloodDecalPreload1` (`+0xA8`).
pub fn fn_0045a520(e: &mut Engine, this: Ptr<TES>) {
    for setting in PRELOAD_MODEL_SETTINGS {
        let text = setting_text(e, setting);
        if e.mem.i8(text) != 0 {
            let name = setting_text(e, setting);
            let model_loader: u32 = e.global(MODEL_LOADER);
            fn_0045a5e0(e, Ptr::new(model_loader), name);
        }
    }
    e.call(NI_POINTER_ASSIGN, &args![this.addr() + 0xa8, 0u32]);
}

// Translated from 0045a5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::ReleaseModel(name, 1, 1)` on the model loader `this`.
pub fn fn_0045a5e0(e: &mut Engine, this: Ptr, name: u32) {
    e.call(MODEL_LOADER_RELEASE_MODEL, &args![this, name, 1u32, 1u32]);
}

// Translated from 0045a600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Preloads the water textures: after `0070ec90` (the water system getter)
/// and unless `bUsePerWorldSpaceWaterNoise` is set, goes through the data
/// handler's list at `+0x130` (until a node reports empty); for each entry
/// that `00580080` accepts, `TES::CreateTextureImage(name, &texture, 1, 0)`
/// loads the texture `005800a0` names into a local `NiPointer`, which
/// `005800c0` then receives; the pointer is released.
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn fn_0045a600(e: &mut Engine, this: Ptr<TES>) {
    e.with_stack(SCOPE_GUARD_SIZE, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 0x1du32, 1u32, TES_CPP_PATH, 0x1c27u32],
        );
        e.call(TES_WATER_SYSTEM_GETTER, &args![this]);
        if setting_byte(e, SETTING_PER_WORLD_SPACE_WATER_NOISE) != 0 {
            e.call(SCOPE_GUARD_DTOR, &args![guard]);
            return;
        }
        let handler: u32 = e.global(DATA_HANDLER);
        let mut node = fn_0045a730(e, Ptr::new(handler));
        while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
            let entry = list_node_item(e, node);
            if e.call(0x0058_0080, &args![entry]).u32() != 0 {
                e.with_stack(4, |e, texture| {
                    e.call(NI_POINTER_CTOR, &args![texture, 0u32]);
                    let name = e.call(0x0058_00a0, &args![entry]).u32();
                    let tes: u32 = e.global(TES_SINGLETON);
                    e.call(CREATE_TEXTURE_IMAGE, &args![tes, name, texture, 1u32, 0u32]);
                    let loaded = ni_pointer_get(e, texture);
                    e.call(0x0058_00c0, &args![entry, loaded]);
                    e.call(NI_POINTER_DTOR, &args![texture]);
                });
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
    });
}

// Translated from 0045a730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this + 0x130` (the data handler's list head at that offset).
pub fn fn_0045a730(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x130)
}

// ----- Session 3: `0045a750` to `0045cac0` -----
//
// The particle cache, the `NavMeshInfoMap` accessors, two long walks over the
// loaded cells and the water system (`fn_0045b070`, `fn_0045bc80`) and a set
// of small accessors on the objects they use. Several small functions belong
// to other classes whose bodies the compiler folded into this unit: they are
// named by address and described by what they do.

/// `sDismemberParticleDefault` (a string `INISetting`).
const SETTING_DISMEMBER_PARTICLE_DEFAULT: u32 = 0x011c_e0c0;
/// `sDismemberRobotParticleDefault`.
const SETTING_DISMEMBER_ROBOT_PARTICLE_DEFAULT: u32 = 0x011c_fa28;
/// `sBloodParticleMeleeDefault`.
const SETTING_BLOOD_PARTICLE_MELEE_DEFAULT: u32 = 0x011c_e1a0;
/// `sImpactParticleWoodDefault`.
const SETTING_IMPACT_PARTICLE_WOOD_DEFAULT: u32 = 0x011c_fa64;
/// `sImpactParticleMetalDefault`.
const SETTING_IMPACT_PARTICLE_METAL_DEFAULT: u32 = 0x011c_f3a4;
/// `sImpactParticleConcreteDefault`.
const SETTING_IMPACT_PARTICLE_CONCRETE_DEFAULT: u32 = 0x011c_f54c;

/// The particle models `TES::SetupTemporaryParticleCache` loads, in the order
/// it visits them.
const PARTICLE_CACHE_SETTINGS: [u32; 8] = [
    SETTING_DISMEMBER_PARTICLE_DEFAULT,
    SETTING_DISMEMBER_ROBOT_PARTICLE_DEFAULT,
    SETTING_SPLASH_PARTICLES,
    SETTING_BLOOD_PARTICLE_MELEE_DEFAULT,
    SETTING_IMPACT_PARTICLE_WOOD_DEFAULT,
    SETTING_IMPACT_PARTICLE_METAL_DEFAULT,
    SETTING_IMPACT_PARTICLE_CONCRETE_DEFAULT,
    SETTING_BLOOD_PARTICLE_DEFAULT,
];
/// The same settings in the order `fn_0045ac80` releases their models.
const PARTICLE_CACHE_RELEASE_ORDER: [u32; 8] = [
    SETTING_BLOOD_PARTICLE_DEFAULT,
    SETTING_SPLASH_PARTICLES,
    SETTING_BLOOD_PARTICLE_MELEE_DEFAULT,
    SETTING_IMPACT_PARTICLE_METAL_DEFAULT,
    SETTING_IMPACT_PARTICLE_CONCRETE_DEFAULT,
    SETTING_IMPACT_PARTICLE_WOOD_DEFAULT,
    SETTING_DISMEMBER_PARTICLE_DEFAULT,
    SETTING_DISMEMBER_ROBOT_PARTICLE_DEFAULT,
];
/// A byte set once the particle cache has been filled.
const PARTICLE_CACHE_READY: u32 = 0x011c_3f08;

/// `operator new(size)`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(pointer)`.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `_eh_vector_constructor_iterator_(array, size, count, constructor,
/// destructor)`.
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
/// `NiPointer()` with a null pointer (what the array constructor calls).
const NI_POINTER_DEFAULT_CONSTRUCT: u32 = 0x0066_94e0;
/// The deleting destructor of an array of `NiPointer`s (flags `3`: destroy the
/// elements, free the block).
const NI_POINTER_ARRAY_DELETING_DESTRUCTOR: u32 = 0x0066_7120;
/// `ecx` = a particle cache entry: whether its first word is the argument.
const CACHE_ENTRY_HOLDS_OBJECT: u32 = 0x0082_2510;
/// `this->+8` (the next link of a cache entry).
const LINK_GET_NEXT: u32 = 0x0044_ddc0;
/// `this->+8 = argument`.
const LINK_SET_NEXT: u32 = 0x0040_3550;
/// `NiObject::Clone` (Xbox PDB `NiObject::Clone_ov2`: no argument).
const NI_OBJECT_CLONE: u32 = 0x00a5_d680;
/// `NiPointer<T>::operator=(const NiPointer<T>&)`: the argument is the address
/// of the other pointer cell.
const NI_POINTER_COPY_ASSIGN: u32 = 0x006e_5cc0;
/// `NiPointer<T>::NiPointer(const NiPointer<T>&)`: the argument is the address
/// of the other pointer cell.
const NI_POINTER_COPY_CONSTRUCT: u32 = 0x0055_9a40;
/// Accessor for the parent pointer at `+0x18` of a scene object.
const NODE_PARENT: u32 = 0x0096_11e0;
/// `NavMeshInfoMap::NavMeshInfoMap` (the object is 0x40 bytes).
const NAV_MESH_INFO_MAP_CONSTRUCT: u32 = 0x006b_5c30;
/// Logs a warning (`const char*`).
const LOG_WARNING: u32 = 0x005b_5e40;
/// `"AI: Multiple NavMeshInfoMaps loaded"`.
const MESSAGE_MULTIPLE_NAV_MESH_INFO_MAPS: u32 = 0x0101_7f94;
/// The test on the data handler (`004516b0`, in the main part of this unit)
/// that stops `TES::GetNavMeshInfoMap` from creating the map.
const DATA_HANDLER_BLOCKS_NAV_MESH_INFO_MAP: u32 = 0x0045_16b0;

/// The calls `fn_0045b020` makes, in order.
const STATIC_SETUP_CALLS: [u32; 6] = [
    0x0064_9e40,
    0x0065_c570,
    0x0066_2760,
    0x004f_92e0,
    0x004e_0050,
    0x004d_e580,
];
/// The calls `fn_0045b050` makes, in order.
const STATIC_TEARDOWN_CALLS: [u32; 5] = [
    0x0064_9ec0,
    0x0065_c5f0,
    0x0066_27e0,
    0x004f_9360,
    0x004e_00d0,
];

/// The value of the pointer stored in the cell at `address`
/// (`NiPointer::operator T*`).
fn pointer_at(e: &mut Engine, address: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![address]).u32()
}

// Translated from 0045a750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::SetupTemporaryParticleCache` (Xbox PDB): once (the byte at
/// `011c3f08` marks the cache as filled), loads the eight default particle
/// models named by the settings in `PARTICLE_CACHE_SETTINGS`
/// (`ModelLoader::LoadFile(name, 0, 1, 0, 0, 0)`, for each whose text is not
/// empty) and adds each to the particle cache
/// (`TES::AddTemporaryParticleObjectToCache`).
///
/// Not translated: the compiler's exception-unwinding frame and the stack
/// cookie check.
pub fn tes_setup_temporary_particle_cache(e: &mut Engine, this: Ptr<TES>) {
    if e.mem.u8(PARTICLE_CACHE_READY) != 0 {
        return;
    }
    e.with_stack(SCOPE_GUARD_SIZE, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 0x32u32, 1u32, TES_CPP_PATH, 0x1c66u32],
        );
        e.mem.set_u8(PARTICLE_CACHE_READY, 1);
        for setting in PARTICLE_CACHE_SETTINGS {
            let text = setting_text(e, setting);
            if e.mem.i8(text) != 0 {
                let name = setting_text(e, setting);
                let model_loader: u32 = e.global(MODEL_LOADER);
                let model = e
                    .call(
                        MODEL_LOADER_LOAD_FILE,
                        &args![model_loader, name, 0u32, 1u32, 0u32, 0u32, 0u32],
                    )
                    .u32();
                tes_add_temporary_particle_object_to_cache(e, this, model);
            }
        }
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
    });
}

// Translated from 0045a9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::AddTemporaryParticleObjectToCache` (Xbox PDB): unless `object` is
/// null or already held by an entry of the list at `TES::pParticleCacheHead`
/// (`+0xB0`), makes a new 12-byte entry (`fn_0045ab30`) holding it, puts it at
/// the head of the list, and gives it an array of three `NiPointer`s: the
/// first holds `object`, the other two hold clones of it.
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn tes_add_temporary_particle_object_to_cache(e: &mut Engine, this: Ptr<TES>, object: u32) {
    if object == 0 {
        return;
    }
    let mut node = e.get(this, TES::pParticleCacheHead).addr();
    while node != 0 {
        if e.call(CACHE_ENTRY_HOLDS_OBJECT, &args![node, object])
            .bool()
        {
            return;
        }
        node = e.call(LINK_GET_NEXT, &args![node]).u32();
    }
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let entry = if block != 0 {
        fn_0045ab30(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.call(NI_POINTER_ASSIGN, &args![entry, object]);
    let old_head = e.get(this, TES::pParticleCacheHead).addr();
    e.call(LINK_SET_NEXT, &args![entry, old_head]);
    e.set(this, TES::pParticleCacheHead, Ptr::new(entry));
    let raw = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let cells = if raw != 0 {
        e.mem.set_u32(raw, 3);
        e.call(
            VECTOR_CONSTRUCT,
            &args![
                raw + 4,
                4u32,
                3u32,
                NI_POINTER_DEFAULT_CONSTRUCT,
                NI_POINTER_DTOR
            ],
        );
        raw + 4
    } else {
        0
    };
    e.mem.set_u32(entry + 4, cells);
    e.call(NI_POINTER_ASSIGN, &args![cells, object]);
    for index in 1..3u32 {
        let clone = e.call(NI_OBJECT_CLONE, &args![object]).u32();
        e.call(NI_POINTER_ASSIGN, &args![cells + 4 * index, clone]);
    }
}

// Translated from 0045ab30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a particle cache entry: a null `NiPointer` at `+0`, the
/// array of cached objects at `+4` and the next link at `+8`, both null.
/// Returns `this`.
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn fn_0045ab30(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_POINTER_CTOR, &args![this, 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![this, 0u32]);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 8, 0);
    this
}

// Translated from 0045aba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::GetUnusedCachedParticleObject` (Xbox PDB): clears the `NiPointer`
/// cell `out`, then looks for the cache entry holding `key`. Returns 0 when
/// there is none, 1 when there is one but none of its three cached objects
/// is both present and without a parent, and 2 when `out` has been set to the
/// first such object.
pub fn tes_get_unused_cached_particle_object(
    e: &mut Engine,
    this: Ptr<TES>,
    key: u32,
    out: u32,
) -> u32 {
    e.call(NI_POINTER_ASSIGN, &args![out, 0u32]);
    let mut result = 0;
    let mut node = e.get(this, TES::pParticleCacheHead).addr();
    while node != 0 {
        if e.call(CACHE_ENTRY_HOLDS_OBJECT, &args![node, key]).bool() {
            result = 1;
            break;
        }
        node = e.call(LINK_GET_NEXT, &args![node]).u32();
    }
    if node != 0 {
        let cells = e.mem.u32(node + 4);
        if cells != 0 {
            for index in 0..3u32 {
                let slot = cells + 4 * index;
                if pointer_at(e, slot) != 0 {
                    let object = pointer_at(e, slot);
                    if e.call(NODE_PARENT, &args![object]).u32() == 0 {
                        e.call(NI_POINTER_COPY_ASSIGN, &args![out, slot]);
                        return 2;
                    }
                }
            }
        }
    }
    result
}

// Translated from 0045ac80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the particle cache: destroys every entry of the list at `+0xB0`
/// (`fn_0045ae20` with flag 1), clears the head, and releases the models of
/// the eight settings (`fn_0045a5e0`, each whose text is not empty) in the
/// order of `PARTICLE_CACHE_RELEASE_ORDER`.
pub fn fn_0045ac80(e: &mut Engine, this: Ptr<TES>) {
    let mut node = e.get(this, TES::pParticleCacheHead).addr();
    while node != 0 {
        let next = e.call(LINK_GET_NEXT, &args![node]).u32();
        if node != 0 {
            fn_0045ae20(e, Ptr::new(node), 1);
        }
        node = next;
    }
    e.set(this, TES::pParticleCacheHead, Ptr::new(0));
    for setting in PARTICLE_CACHE_RELEASE_ORDER {
        let text = setting_text(e, setting);
        if e.mem.i8(text) != 0 {
            let name = setting_text(e, setting);
            let model_loader: u32 = e.global(MODEL_LOADER);
            fn_0045a5e0(e, Ptr::new(model_loader), name);
        }
    }
}

// Translated from 0045ae20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of a particle cache entry: destroys it
/// (`fn_0045ae50`) and, when bit 0 of `flags` is set, frees it. Returns
/// `this`.
pub fn fn_0045ae20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0045ae50(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0045ae50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of a particle cache entry: clears the `NiPointer` at `+0`,
/// destroys the array of three cached objects at `+4` (when there is one) and
/// releases the entry's own pointer.
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn fn_0045ae50(e: &mut Engine, this: Ptr) {
    e.call(NI_POINTER_ASSIGN, &args![this, 0u32]);
    let cells = e.mem.u32(this.addr() + 4);
    if cells != 0 {
        e.call(NI_POINTER_ARRAY_DELETING_DESTRUCTOR, &args![cells, 3u32]);
    }
    e.call(NI_POINTER_DTOR, &args![this]);
}

// Translated from 0045aee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` as `TES::bAllowUnusedPurge` (`+0xB5`).
pub fn fn_0045aee0(e: &mut Engine, this: Ptr<TES>, value: u8) {
    e.mem
        .set_u8(this.addr() + TES::bAllowUnusedPurge.off, value);
}

// Translated from 0045af00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::GetNavMeshInfoMap` (Xbox PDB): returns `pNavMeshInfoMap` (`+0xBC`),
/// first creating it (a 0x40-byte `NavMeshInfoMap`) when there is none, the
/// data handler exists and `004516b0` says it may be created.
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn tes_get_nav_mesh_info_map(e: &mut Engine, this: Ptr<TES>) -> Ptr {
    if e.get(this, TES::pNavMeshInfoMap).is_null() {
        let handler: u32 = e.global(DATA_HANDLER);
        if handler != 0
            && !e
                .call(DATA_HANDLER_BLOCKS_NAV_MESH_INFO_MAP, &args![handler])
                .bool()
        {
            let block = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
            let map = if block != 0 {
                e.call(NAV_MESH_INFO_MAP_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            e.set(this, TES::pNavMeshInfoMap, Ptr::new(map));
        }
    }
    e.get(this, TES::pNavMeshInfoMap)
}

// Translated from 0045afb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::SetNavMeshInfoMap` (Xbox PDB): when a map is already set, logs
/// "AI: Multiple NavMeshInfoMaps loaded" if `map` is not null, and deletes
/// the old one (its slot `0x10` with argument 1); then stores `map`.
pub fn tes_set_nav_mesh_info_map(e: &mut Engine, this: Ptr<TES>, map: Ptr) {
    let old = e.get(this, TES::pNavMeshInfoMap);
    if !old.is_null() {
        if !map.is_null() {
            e.call(LOG_WARNING, &args![MESSAGE_MULTIPLE_NAV_MESH_INFO_MAPS]);
        }
        e.vcall(old.addr(), 0x10, &args![1u32]);
    }
    e.set(this, TES::pNavMeshInfoMap, map);
}

// Translated from 0045b020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls six set-up routines in turn (`00649e40`, `0065c570`, `00662760`,
/// `004f92e0`, `004e0050`, `004de580`).
pub fn fn_0045b020(e: &mut Engine) {
    for function in STATIC_SETUP_CALLS {
        e.call(function, &[]);
    }
}

// Translated from 0045b050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls five tear-down routines in turn (`00649ec0`, `0065c5f0`, `006627e0`,
/// `004f9360`, `004e00d0`).
pub fn fn_0045b050(e: &mut Engine) {
    for function in STATIC_TEARDOWN_CALLS {
        e.call(function, &[]);
    }
}

// ----- the scene walks (`0045b070`, `0045bc80`) -----

/// `fn_00450b80`: the entry of the shadow scene node table.
const SHADOW_SCENE_NODE_GETTER: u32 = 0x0045_0b80;
/// `fn_00454b30`: the word at `+0x1E0` of the scene node (its state object).
const SCENE_NODE_STATE: u32 = 0x0045_4b30;
/// `fn_00456fc0`: `ecx` = a cell, argument = the index of one of its lists;
/// returns the list's object (or 0).
const CELL_GET_LIST: u32 = 0x0045_6fc0;
/// Number of entries of the array at `+0x9C` of a node (`ecx` = the node).
const NODE_CHILD_COUNT: u32 = 0x0043_b480;
/// Entry `n` of the array at `+0x9C` of a node (`ecx` = the node).
const NODE_CHILD_AT: u32 = 0x0043_b4a0;
/// World space function giving the terrain manager.
const WORLD_SPACE_TERRAIN: u32 = 0x0058_6170;
/// Terrain manager call made with the camera before the add walk.
const TERRAIN_PREPARE_FOR_CAMERA: u32 = 0x006f_d080;
/// Terrain manager call made before the removal walk.
const TERRAIN_PREPARE_FOR_REMOVAL: u32 = 0x006f_d130;
/// `ecx` = a form: a helper whose result the scene walks do not use.
const FORM_HAS_BUFFER_FORM: u32 = 0x007a_f430;
/// `ecx` = the water system: its second list holder.
const WATER_SYSTEM_SECOND_HOLDER: u32 = 0x0067_33e0;
/// `ecx` = the water system: address of its list header at `+0x3C`.
const WATER_SYSTEM_LIST: u32 = 0x005a_8080;
/// The next position of the list `ecx`, after the position argument.
const LIST_NEXT_POSITION: u32 = 0x007b_52d0;
/// The address of the item of the position argument in the list `ecx`.
const LIST_ITEM_ADDRESS: u32 = 0x0063_17a0;
/// Advances the position cell argument of the list `ecx`; returns the address
/// of the item it passed.
const LIST_ITERATE: u32 = 0x0057_cbe0;
/// The `NiRTTI` the first object of a water entry must have.
const WATER_ENTRY_RTTI: u32 = 0x0120_2e74;
/// A global the scene walks hand to `00483710` (an empty function) before and
/// after.
const SCENE_WALK_MARKER: u32 = 0x011c_3c0f;
/// `00483710`: an empty function taking `ecx` = the marker address.
const SCENE_WALK_MARKER_CALL: u32 = 0x0048_3710;
/// Size of the culling process object on the stack.
const CULLING_PROCESS_SIZE: u32 = 0x90;
/// Culling process constructor (`ecx` = the object, argument 0).
const CULLING_PROCESS_CONSTRUCT: u32 = 0x00a6_9400;
/// Culling process: set the camera.
const CULLING_PROCESS_SET_CAMERA: u32 = 0x0041_fd00;
/// Culling process: set the frustum.
const CULLING_PROCESS_SET_FRUSTUM: u32 = 0x00a6_94a0;
/// Culling process destructor.
const CULLING_PROCESS_DESTRUCT: u32 = 0x00a6_93e0;
/// `ecx` = the culling process: the address of its member at `+0x2C` (a
/// stack argument pushed before the call is ignored and stays pushed).
const CULLING_PROCESS_MEMBER: u32 = 0x005d_8a70;
/// `ecx` = the culling process: the word at `+0x0C`.
const CULLING_PROCESS_WORD: u32 = 0x0084_e3a0;
/// Shadow scene node call that adds an object seen from a position:
/// (object, x, y, z, culling process).
const SCENE_ADD_OBJECT: u32 = 0x00b5_bbe0;
/// Shadow scene node call that adds a candidate object:
/// (object, culling process word, culling process member, flag).
const SCENE_ADD_CANDIDATE: u32 = 0x00b5_f170;
/// `ecx` = a scene object: the first step of reaching `[[object + 0xAC] + 0xC]`.
const OBJECT_FIRST_LINK: u32 = 0x0066_29f0;
/// Second step of the link above.
const OBJECT_SECOND_LINK: u32 = 0x0043_b230;
/// Call made on the node the two links reach, to take the object out of the
/// scene.
const NODE_REMOVE: u32 = 0x0052_8820;
/// `ecx` = a state object: its list of tracked objects.
const SCENE_STATE_LIST: u32 = 0x0041_3f40;
/// `ecx` = a state object: its queue of objects.
const SCENE_STATE_QUEUE: u32 = 0x0089_1170;
/// `ecx` = a queue: whether it is empty.
const SCENE_STATE_QUEUE_IS_EMPTY: u32 = 0x0076_b610;
/// `ecx` = a state object: the object it holds.
const SCENE_STATE_HELD: u32 = 0x0056_c7f0;
/// `ecx` = an object: its first list of attached items.
const OBJECT_ITEMS_A: u32 = 0x006a_b360;
/// `ecx` = an object: its second list of attached items.
const OBJECT_ITEMS_B: u32 = 0x0051_4f30;
/// `ecx` = an item: the object behind its pointer at `+0x104`.
const ITEM_GET_TARGET_OBJECT: u32 = 0x004f_9bf0;
/// `ecx` = an object: releases it (argument 0).
const OBJECT_RELEASE: u32 = 0x00c4_7980;
/// `ecx` = a point: adds the point `other` (second argument) and stores the
/// sum in the first argument.
const POINT_ADD: u32 = 0x0043_9e90;
/// `ecx` = a matrix: copies its column (first argument) to the point (second
/// argument).
const MATRIX_GET_COLUMN: u32 = 0x0043_9f50;
/// Constructor of a vector of three `float`s: (x, y, z).
const VECTOR3_CONSTRUCT: u32 = 0x0041_6870;

/// Adds `object` to `scene`, seen from the camera position `position`.
fn scene_add_object(e: &mut Engine, scene: u32, object: u32, position: [u32; 3], process: Ptr) {
    e.call(
        SCENE_ADD_OBJECT,
        &args![
            scene,
            object,
            position[0],
            position[1],
            position[2],
            process
        ],
    );
}

/// Takes `object` out of the scene: reaches `[[object + 0xAC] + 0xC]` and
/// calls `00528820` on it.
fn scene_remove_object(e: &mut Engine, object: u32) {
    let first = e.call(OBJECT_FIRST_LINK, &args![object]).u32();
    let node = e.call(OBJECT_SECOND_LINK, &args![first]).u32();
    e.call(NODE_REMOVE, &args![node]);
}

/// The water system of `this` (`TES::pWaterSystem`).
fn water_system(e: &mut Engine, this: Ptr<TES>) -> u32 {
    e.call(TES_WATER_SYSTEM_GETTER, &args![this]).u32()
}

/// The list header at `+0x3C` of the water system.
fn water_system_list(e: &mut Engine, this: Ptr<TES>) -> u32 {
    let system = water_system(e, this);
    e.call(WATER_SYSTEM_LIST, &args![system]).u32()
}

/// Goes through the list at `holder + 0x24`: for each entry (after calling
/// `007af430` on it) whose slot `0x1D0` object has a first child with the
/// water entry `NiRTTI`, calls `visit` with that child.
fn visit_water_entries(e: &mut Engine, holder: u32, visit: &mut dyn FnMut(&mut Engine, u32)) {
    let list = holder + 0x24;
    let mut position = pointer_at(e, list);
    while position != 0 {
        let next = e.call(LIST_NEXT_POSITION, &args![list, position]).u32();
        if position != 0 {
            let item_address = e.call(LIST_ITEM_ADDRESS, &args![list, position]).u32();
            let entry = e.mem.u32(item_address);
            e.call(FORM_HAS_BUFFER_FORM, &args![entry]);
            let objects = e.vcall(entry, 0x1d0, &[]).u32();
            let first = e.call(NODE_CHILD_AT, &args![objects, 0u32]).u32();
            if fn_0045bad0(e, WATER_ENTRY_RTTI, Ptr::new(first)) {
                let object = e.call(NODE_CHILD_AT, &args![objects, 0u32]).u32();
                visit(e, object);
            }
        }
        position = next;
    }
}

/// The water system walk both scene functions make: when there is a water
/// system, every entry of its list at `+0x3C` (each holding a list at
/// `+0x24`) and then its second holder are visited with
/// `visit_water_entries`.
fn visit_water_system(e: &mut Engine, this: Ptr<TES>, visit: &mut dyn FnMut(&mut Engine, u32)) {
    if water_system(e, this) == 0 {
        return;
    }
    let list = water_system_list(e, this);
    let mut position = pointer_at(e, list);
    while position != 0 {
        let list = water_system_list(e, this);
        let next = e.call(LIST_NEXT_POSITION, &args![list, position]).u32();
        if position != 0 {
            let list = water_system_list(e, this);
            let item_address = e.call(LIST_ITEM_ADDRESS, &args![list, position]).u32();
            let holder = e.mem.u32(item_address);
            visit_water_entries(e, holder, visit);
        }
        position = next;
    }
    let system = water_system(e, this);
    let second = e.call(WATER_SYSTEM_SECOND_HOLDER, &args![system]).u32();
    if second != 0 {
        visit_water_entries(e, second, visit);
    }
}

/// Calls `visit` with each child of the node `list`, the way the scene walks
/// do: the count is read again before every step, null children are skipped.
fn visit_children(e: &mut Engine, list: u32, visit: &mut dyn FnMut(&mut Engine, u32)) {
    let mut index = 0;
    while index < e.call(NODE_CHILD_COUNT, &args![list]).u32() {
        let object = e.call(NODE_CHILD_AT, &args![list, index]).u32();
        if object != 0 {
            visit(e, object);
        }
        index += 1;
    }
}

/// The cell pointer in slot (x, y) of the grid array.
fn grid_cell_at(e: &mut Engine, this: Ptr<TES>, x: u32, y: u32) -> u32 {
    let grid = e.get(this, TES::pGridCellA);
    let slot = e.call(GRID_CELL_ARRAY_GET, &args![grid, x, y]).u32();
    e.mem.u32(slot)
}

// Translated from 0045b070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the objects of the loaded world to the first shadow scene node
/// (`fn_00450b80(0)`), for the camera `camera`. With a culling process on
/// the stack (set to the camera and its frustum, `fn_0045bbe0`) and the
/// camera position (`fn_0045bb80`, three words), `00b5bbe0` is called for: the
/// interior cell's list 7 entries whose slot `0x104` returns 0; or, outdoors,
/// the terrain manager is told of the camera and every loaded grid cell's
/// lists 7 and 2 are added; then the children of the scene node's child 6
/// (slot `0xC`) and the water system's entries (`visit_water_system`).
///
/// When the scene node has a state object (`fn_00454b30`), see
/// `scene_update_held_object`.
///
/// The empty function `00483710` is called with `011c3c0f` before and after.
/// Not translated: the compiler's exception-unwinding frame, the stack cookie
/// check, and a local the code initialises and never reads.
pub fn fn_0045b070(e: &mut Engine, this: Ptr<TES>, camera: u32) {
    e.call(SCENE_WALK_MARKER_CALL, &args![SCENE_WALK_MARKER]);
    e.with_stack(CULLING_PROCESS_SIZE, |e, process| {
        e.call(CULLING_PROCESS_CONSTRUCT, &args![process, 0u32]);
        e.call(CULLING_PROCESS_SET_CAMERA, &args![process, camera]);
        let frustum = fn_0045bbe0(e, Ptr::new(camera));
        e.call(CULLING_PROCESS_SET_FRUSTUM, &args![process, frustum]);
        let location = fn_0045bb80(e, Ptr::new(camera)).addr();
        let position = [
            e.mem.u32(location),
            e.mem.u32(location + 4),
            e.mem.u32(location + 8),
        ];
        let scene = e.call(SHADOW_SCENE_NODE_GETTER, &args![0u32]).u32();
        let mut add = |e: &mut Engine, object: u32| {
            scene_add_object(e, scene, object, position, process);
        };
        let interior = e.get(this, TES::pInteriorCell).addr();
        if interior != 0 {
            let list = e.call(CELL_GET_LIST, &args![interior, 7u32]).u32();
            if list != 0 {
                let mut index = 0;
                while index < e.call(NODE_CHILD_COUNT, &args![list]).u32() {
                    let object = e.call(NODE_CHILD_AT, &args![list, index]).u32();
                    if object != 0 && e.vcall(object, 0x104, &[]).u32() == 0 {
                        add(e, object);
                    }
                    index += 1;
                }
            }
        } else {
            let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
            let terrain = if world_space != 0 {
                let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
                e.call(WORLD_SPACE_TERRAIN, &args![world_space]).u32()
            } else {
                0
            };
            if terrain != 0 {
                e.call(TERRAIN_PREPARE_FOR_CAMERA, &args![terrain, camera]);
            }
            let mut x = 0;
            while x < grids_to_load(e) {
                let mut y = 0;
                while y < grids_to_load(e) {
                    let cell = grid_cell_at(e, this, x, y);
                    if cell != 0 {
                        let list = e.call(CELL_GET_LIST, &args![cell, 7u32]).u32();
                        if list != 0 {
                            visit_children(e, list, &mut add);
                        }
                        let list = e.call(CELL_GET_LIST, &args![cell, 2u32]).u32();
                        visit_children(e, list, &mut add);
                    }
                    y += 1;
                }
                x += 1;
            }
        }
        let child = fn_0045bc00(e, Ptr::new(scene), 6);
        let children = if child != 0 {
            e.vcall(child, 0xc, &[]).u32()
        } else {
            0
        };
        if children != 0 {
            visit_children(e, children, &mut add);
        }
        visit_water_system(e, this, &mut add);
        let first = e.call(SHADOW_SCENE_NODE_GETTER, &args![0u32]).u32();
        if e.call(SCENE_NODE_STATE, &args![first]).u32() != 0 {
            scene_update_held_object(e, camera, scene, process);
        }
        e.call(SCENE_WALK_MARKER_CALL, &args![SCENE_WALK_MARKER]);
        e.call(CULLING_PROCESS_DESTRUCT, &args![process]);
    });
}

/// The second half of `fn_0045b070`, run when the scene node has a state
/// object. The state's list at `+0x30` is cleared (`fn_0045bc60`) and a point
/// `near` units in front of the camera (the camera's first matrix column times
/// the frustum's near distance, plus its position) is built. The held object
/// (a `NiPointer` cell set from `0056c7f0`) is kept only if its slot `0x108`
/// accepts the point (otherwise the cell is cleared); a kept one is offered to
/// the scene with `00b5f170`. If the cell is empty, the state's queue
/// (`00891170`, when not empty) is walked: each object whose slot `0x108`
/// accepts the point is offered, the first one also fills the cell. If the cell
/// is still empty, a null object is offered. The held object becomes the
/// state's pointer (`fn_0045bc40`).
fn scene_update_held_object(e: &mut Engine, camera: u32, scene: u32, process: Ptr) {
    let node = e.call(SHADOW_SCENE_NODE_GETTER, &args![0u32]).u32();
    let state = e.call(SCENE_NODE_STATE, &args![node]).u32();
    fn_0045bc60(e, Ptr::new(state));
    let node = e.call(SHADOW_SCENE_NODE_GETTER, &args![0u32]).u32();
    let state = e.call(SCENE_NODE_STATE, &args![node]).u32();
    let held_object = e.call(SCENE_STATE_HELD, &args![state]).u32();
    // The held cell (4 bytes), then three points of 12 bytes.
    e.with_stack(4 + 36, |e, cell| {
        e.call(NI_POINTER_CTOR, &args![cell, held_object]);
        let held = cell.addr();
        let column_point = held + 4;
        let scaled_point = held + 16;
        let point = held + 28;
        let frustum = fn_0045bbe0(e, Ptr::new(camera));
        let near = e.mem.f32(frustum.addr() + 0x10);
        let column = fn_0045bba0(e, Ptr::new(camera), Ptr::new(column_point));
        let scaled = fn_0045bb20(e, column, Ptr::new(scaled_point), near);
        let location = fn_0045bb80(e, Ptr::new(camera));
        e.call(POINT_ADD, &args![location, point, scaled]);
        let mut flag = 1u32;
        if pointer_at(e, held) != 0 {
            let object = pointer_at(e, held);
            if !e.vcall(object, 0x108, &args![point]).bool() {
                e.call(NI_POINTER_ASSIGN, &args![held, 0u32]);
            } else {
                let object = pointer_at(e, held);
                scene_add_candidate(e, scene, process, object, 1);
            }
        }
        if pointer_at(e, held) == 0 {
            let state = e.call(SCENE_NODE_STATE, &args![scene]).u32();
            let queue = e.call(SCENE_STATE_QUEUE, &args![state]).u32();
            if !e.call(SCENE_STATE_QUEUE_IS_EMPTY, &args![queue]).bool() {
                e.with_stack(4, |e, position| {
                    let first = pointer_at(e, queue);
                    e.mem.set_u32(position.addr(), first);
                    while e.mem.u32(position.addr()) != 0 {
                        let item_address = e.call(LIST_ITERATE, &args![queue, position]).u32();
                        let item = pointer_at(e, item_address);
                        if item != 0 && e.vcall(item, 0x108, &args![point]).bool() {
                            scene_add_candidate(e, scene, process, item, flag);
                            if pointer_at(e, held) == 0 {
                                e.call(NI_POINTER_ASSIGN, &args![held, item]);
                                flag = 0;
                            }
                        }
                    }
                });
            }
        }
        if pointer_at(e, held) == 0 {
            scene_add_candidate(e, scene, process, 0, flag);
        }
        let value = pointer_at(e, held);
        let state = e.call(SCENE_NODE_STATE, &args![scene]).u32();
        fn_0045bc40(e, Ptr::new(state), value);
        e.call(NI_POINTER_DTOR, &args![held]);
    });
}

/// `00b5f170(object, word, member, flag)` with the two culling process values
/// the code reads through `0084e3a0` and `005d8a70` (which ignores the flag
/// pushed before it; the flag stays on the stack as the last argument).
fn scene_add_candidate(e: &mut Engine, scene: u32, process: Ptr, object: u32, flag: u32) {
    let member = e.call(CULLING_PROCESS_MEMBER, &args![process]).u32();
    let word = e.call(CULLING_PROCESS_WORD, &args![process]).u32();
    e.call(
        SCENE_ADD_CANDIDATE,
        &args![scene, object, word, member, flag],
    );
}

// Translated from 0045bad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl`: false for a null `object`, otherwise whether `object`'s slot 8
/// returns `rtti` (`fn_0045baf0`).
pub fn fn_0045bad0(e: &mut Engine, rtti: u32, object: Ptr) -> bool {
    if object.is_null() {
        return false;
    }
    fn_0045baf0(e, object, rtti)
}

// Translated from 0045baf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether this object's slot 8 (`GetRTTI`) returns `rtti`.
pub fn fn_0045baf0(e: &mut Engine, this: Ptr, rtti: u32) -> bool {
    e.vcall(this.addr(), 8, &[]).u32() == rtti
}

// Translated from 0045bb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores into `out` (a three-`float` vector) the vector `this` (three
/// `float`s) multiplied by `scale`; returns `out`.
pub fn fn_0045bb20(e: &mut Engine, this: Ptr, out: Ptr, scale: f32) -> Ptr {
    let x = e.mem.f32(this.addr());
    let y = e.mem.f32(this.addr() + 4);
    let z = e.mem.f32(this.addr() + 8);
    e.call(
        VECTOR3_CONSTRUCT,
        &args![out, x * scale, y * scale, z * scale],
    );
    out
}

// Translated from 0045bb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this + 0x8C` (the camera's world position).
pub fn fn_0045bb80(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x8c))
}

// Translated from 0045bba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores into `out` the first column of the 3x3 matrix at `this + 0x68`
/// (`00439f50` with column 0, into a local vector); returns `out`.
pub fn fn_0045bba0(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    e.with_stack(12, |e, local| {
        e.call(VECTOR_CTOR, &args![local]);
        e.call(
            MATRIX_GET_COLUMN,
            &args![this.addr().wrapping_add(0x68), 0u32, local],
        );
        for word in 0..3 {
            let value = e.mem.u32(local.addr() + 4 * word);
            e.mem.set_u32(out.addr() + 4 * word, value);
        }
    });
    out
}

// Translated from 0045bbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this + 0xDC` (the camera's frustum).
pub fn fn_0045bbe0(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0xdc))
}

// Translated from 0045bc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entry `index` of the node's child array (`+0x9C`, its count a 16-bit word
/// at `+0xA`), or 0 when `index` is not below the count.
pub fn fn_0045bc00(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let array = this.addr().wrapping_add(0x9c);
    let count = e.call(CHILD_ARRAY_COUNT, &args![array]).u32();
    if count <= index {
        return 0;
    }
    let slot = e.call(CHILD_ARRAY_SLOT, &args![array, index]).u32();
    pointer_at(e, slot)
}

/// `ecx` = an array: its count (the 16-bit word at `+0xA`).
const CHILD_ARRAY_COUNT: u32 = 0x0065_8930;
/// `ecx` = an array: the address of entry `n`.
const CHILD_ARRAY_SLOT: u32 = 0x0087_7a30;

// Translated from 0045bc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointer` assignment to the cell at `this + 0x2C`.
pub fn fn_0045bc40(e: &mut Engine, this: Ptr, value: u32) {
    e.call(
        NI_POINTER_ASSIGN,
        &args![this.addr().wrapping_add(0x2c), value],
    );
}

// Translated from 0045bc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the list at `this + 0x30` (`004a4650`).
pub fn fn_0045bc60(e: &mut Engine, this: Ptr) {
    e.call(LIST_CLEAR, &args![this.addr().wrapping_add(0x30)]);
}

/// `ecx` = a list: removes all its nodes.
const LIST_CLEAR: u32 = 0x004a_4650;

// Translated from 0045bc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the objects of the loaded world out of the scene (the counterpart
/// of `fn_0045b070`; every removal is `scene_remove_object`, which reaches
/// `[[object + 0xAC] + 0xC]` and calls `00528820` on it).
///
/// - Interior: the cell's list 7 entries whose slot `0x104` returns 0.
/// - Outdoors: the terrain manager is told (`006fd130`), then every loaded
///   grid cell's list 7 entries get their slot `0xBC` called before being
///   removed, and, when `with_lists` is set, the list 2 entries are removed.
/// - Then the children of the first shadow scene node's child 6 (slot `0xC`)
///   and the water system's entries.
///
/// When the scene node has a state object: its tracked list is walked and
/// `fn_0045c4a0` run on each item; then, when its queue is not empty and
/// `with_queue` is set, each queued object (held in a copied `NiPointer`) gets
/// its slot `0xBC` called, `fn_0045c570(3)` and `fn_0045c4e0(0)` run on it,
/// both its item lists are walked (`fn_0045c4a0` on every item; for the
/// first list's items whose `fn_0045c4c0` target is not null, that target's
/// object is released) before the pointer is dropped.
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn fn_0045bc80(e: &mut Engine, this: Ptr<TES>, with_lists: u8, with_queue: u8) {
    let interior = e.get(this, TES::pInteriorCell).addr();
    if interior != 0 {
        let list = e.call(CELL_GET_LIST, &args![interior, 7u32]).u32();
        if list != 0 {
            let mut index = 0;
            while index < e.call(NODE_CHILD_COUNT, &args![list]).u32() {
                let object = e.call(NODE_CHILD_AT, &args![list, index]).u32();
                if object != 0 && e.vcall(object, 0x104, &[]).u32() == 0 {
                    scene_remove_object(e, object);
                }
                index += 1;
            }
        }
    } else {
        let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
        let terrain = if world_space != 0 {
            let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
            e.call(WORLD_SPACE_TERRAIN, &args![world_space]).u32()
        } else {
            0
        };
        if terrain != 0 {
            e.call(TERRAIN_PREPARE_FOR_REMOVAL, &args![terrain]);
        }
        let mut x = 0;
        while x < grids_to_load(e) {
            let mut y = 0;
            while y < grids_to_load(e) {
                let cell = grid_cell_at(e, this, x, y);
                if cell != 0 {
                    let list = e.call(CELL_GET_LIST, &args![cell, 7u32]).u32();
                    if list != 0 {
                        visit_children(e, list, &mut |e, object| {
                            e.vcall(object, 0xbc, &[]);
                            scene_remove_object(e, object);
                        });
                    }
                    if with_lists != 0 {
                        let list = e.call(CELL_GET_LIST, &args![cell, 2u32]).u32();
                        visit_children(e, list, &mut |e, object| {
                            scene_remove_object(e, object);
                        });
                    }
                }
                y += 1;
            }
            x += 1;
        }
    }
    let node = e.call(SHADOW_SCENE_NODE_GETTER, &args![0u32]).u32();
    let child = fn_0045bc00(e, Ptr::new(node), 6);
    let children = if child != 0 {
        e.vcall(child, 0xc, &[]).u32()
    } else {
        0
    };
    if children != 0 {
        visit_children(e, children, &mut |e, object| {
            scene_remove_object(e, object);
        });
    }
    let node = e.call(SHADOW_SCENE_NODE_GETTER, &args![0u32]).u32();
    if e.call(SCENE_NODE_STATE, &args![node]).u32() != 0 {
        scene_release_tracked(e, with_queue != 0);
    }
    visit_water_system(e, this, &mut |e, object| {
        scene_remove_object(e, object);
    });
}

/// The state-object part of `fn_0045bc80` (see there).
fn scene_release_tracked(e: &mut Engine, with_queue: bool) {
    let tracked = |e: &mut Engine| {
        let node = e.call(SHADOW_SCENE_NODE_GETTER, &args![0u32]).u32();
        let state = e.call(SCENE_NODE_STATE, &args![node]).u32();
        e.call(SCENE_STATE_LIST, &args![state]).u32()
    };
    let list = tracked(e);
    e.with_stack(4, |e, position| {
        let first = pointer_at(e, list);
        e.mem.set_u32(position.addr(), first);
        while e.mem.u32(position.addr()) != 0 {
            let list = tracked(e);
            let item_address = e.call(LIST_ITERATE, &args![list, position]).u32();
            let item = e.mem.u32(item_address);
            if item != 0 {
                fn_0045c4a0(e, Ptr::new(item));
            }
        }
    });
    let node = e.call(SHADOW_SCENE_NODE_GETTER, &args![0u32]).u32();
    let state = e.call(SCENE_NODE_STATE, &args![node]).u32();
    let queue = e.call(SCENE_STATE_QUEUE, &args![state]).u32();
    if e.call(SCENE_STATE_QUEUE_IS_EMPTY, &args![queue]).bool() || !with_queue {
        return;
    }
    e.with_stack(4, |e, position| {
        let first = pointer_at(e, queue);
        e.mem.set_u32(position.addr(), first);
        while e.mem.u32(position.addr()) != 0 {
            let item_address = e.call(LIST_ITERATE, &args![queue, position]).u32();
            e.with_stack(4, |e, held| {
                e.call(NI_POINTER_COPY_CONSTRUCT, &args![held, item_address]);
                if pointer_at(e, held.addr()) != 0 {
                    scene_release_queued(e, held.addr());
                }
                e.call(NI_POINTER_DTOR, &args![held]);
            });
        }
    });
}

/// One queued object of `scene_release_tracked`, held by the `NiPointer`
/// cell `held`.
fn scene_release_queued(e: &mut Engine, held: u32) {
    let object = pointer_at(e, held);
    e.vcall(object, 0xbc, &[]);
    let object = pointer_at(e, held);
    fn_0045c570(e, Ptr::new(object), 3);
    let object = pointer_at(e, held);
    fn_0045c4e0(e, Ptr::new(object), 0);
    let object = pointer_at(e, held);
    let list = e.call(OBJECT_ITEMS_A, &args![object]).u32();
    e.with_stack(4, |e, position| {
        let first = pointer_at(e, list);
        e.mem.set_u32(position.addr(), first);
        while e.mem.u32(position.addr()) != 0 {
            let object = pointer_at(e, held);
            let list = e.call(OBJECT_ITEMS_A, &args![object]).u32();
            let item_address = e.call(LIST_ITERATE, &args![list, position]).u32();
            let item = e.mem.u32(item_address);
            if item != 0 {
                fn_0045c4a0(e, Ptr::new(item));
                let target_cell = fn_0045c4c0(e, Ptr::new(item));
                if e.call(ITEM_GET_TARGET_OBJECT, &args![target_cell]).u32() != 0 {
                    let target_cell = fn_0045c4c0(e, Ptr::new(item));
                    let target = e.call(ITEM_GET_TARGET_OBJECT, &args![target_cell]).u32();
                    e.call(OBJECT_RELEASE, &args![target, 0u32]);
                }
            }
        }
    });
    let object = pointer_at(e, held);
    let list = e.call(OBJECT_ITEMS_B, &args![object]).u32();
    e.with_stack(4, |e, position| {
        let first = pointer_at(e, list);
        e.mem.set_u32(position.addr(), first);
        while e.mem.u32(position.addr()) != 0 {
            let object = pointer_at(e, held);
            let list = e.call(OBJECT_ITEMS_B, &args![object]).u32();
            let item_address = e.call(LIST_ITERATE, &args![list, position]).u32();
            let item = e.mem.u32(item_address);
            if item != 0 {
                fn_0045c4a0(e, Ptr::new(item));
            }
        }
    });
}

// Translated from 0045c4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at `this + 0x40`.
pub fn fn_0045c4a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr().wrapping_add(0x40), 0);
}

// Translated from 0045c4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pointer stored in the `NiPointer` cell at `this + 0x104`.
pub fn fn_0045c4c0(e: &mut Engine, this: Ptr) -> u32 {
    pointer_at(e, this.addr().wrapping_add(0x104))
}

// Translated from 0045c4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the object at `this + 0xCC` by `value`: when there is one that
/// differs from `value`, it is destroyed first (`fn_0045c520`).
pub fn fn_0045c4e0(e: &mut Engine, this: Ptr, value: u32) {
    let current = e.mem.u32(this.addr().wrapping_add(0xcc));
    if current != 0 && current != value {
        fn_0045c520(e, this);
    }
    e.mem.set_u32(this.addr().wrapping_add(0xcc), value);
}

// Translated from 0045c520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys and frees the object at `this + 0xCC` (`fn_0045c5f0`, flag 1)
/// and clears the field.
pub fn fn_0045c520(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(this.addr().wrapping_add(0xcc));
    if object != 0 {
        fn_0045c5f0(e, Ptr::new(object), 1);
    }
    e.mem.set_u32(this.addr().wrapping_add(0xcc), 0);
}

// Translated from 0045c570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at `+8` of the node `[[this + 0xAC] + 0xC]`
/// (`00403550`, with `value` left on the stack by the two link steps), then in
/// the same word of the node `[item + 0xC]` of every item of the list at
/// `this + 0xD0`.
pub fn fn_0045c570(e: &mut Engine, this: Ptr, value: u32) {
    let first = e.call(OBJECT_FIRST_LINK, &args![this]).u32();
    let node = e.call(OBJECT_SECOND_LINK, &args![first]).u32();
    e.call(LINK_SET_NEXT, &args![node, value]);
    let list = this.addr().wrapping_add(0xd0);
    e.with_stack(4, |e, position| {
        let head = pointer_at(e, list);
        e.mem.set_u32(position.addr(), head);
        while e.mem.u32(position.addr()) != 0 {
            let item_address = e.call(LIST_ITERATE, &args![list, position]).u32();
            let item = pointer_at(e, item_address);
            if item != 0 {
                let node = e.call(OBJECT_SECOND_LINK, &args![item]).u32();
                e.call(LINK_SET_NEXT, &args![node, value]);
            }
        }
    });
}

// Translated from 0045c5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor: runs the destructor `00c47770` and, when bit 0
/// of `flags` is set, frees the object. Returns `this`.
pub fn fn_0045c5f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// The destructor `fn_0045c5f0` runs.
const OBJECT_DESTRUCT: u32 = 0x00c4_7770;

// Translated from 0045c620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFrustumPlanes::NiFrustumPlanes` (Xbox PDB): constructs the six planes
/// of 0x10 bytes (`00a69940` each) and sets the active planes word at
/// `+0x60` to `0x3F`. Returns `this`.
pub fn ni_frustum_planes_ni_frustum_planes(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        ARRAY_CONSTRUCT,
        &args![this, 0x10u32, 6u32, FRUSTUM_PLANE_CONSTRUCT],
    );
    e.mem.set_u32(this.addr().wrapping_add(0x60), 0x3f);
    this
}

/// `_vector_constructor_iterator_(array, size, count, constructor)`.
const ARRAY_CONSTRUCT: u32 = 0x0040_1050;
/// The plane constructor.
const FRUSTUM_PLANE_CONSTRUCT: u32 = 0x00a6_9940;

// Translated from 0045c650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this + 0xD0`.
pub fn fn_0045c650(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0xd0))
}

// Translated from 0045c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pointer held by the `NiPointer` cell at `011deb7c`.
pub fn fn_0045c670(e: &mut Engine) -> u32 {
    pointer_at(e, 0x011d_eb7c)
}

// Translated from 0045c680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::RangedNodeVisibilityChangedCB` (Xbox PDB): calls `00578060` with
/// `node` when `visible` is not 0, else `00578170`.
pub fn tes_ranged_node_visibility_changed_cb(e: &mut Engine, node: u32, visible: u8) {
    if visible != 0 {
        e.call(RANGED_NODE_SHOWN, &args![node]);
    } else {
        e.call(RANGED_NODE_HIDDEN, &args![node]);
    }
}

/// Called with a node that became visible.
const RANGED_NODE_SHOWN: u32 = 0x0057_8060;
/// Called with a node that became hidden.
const RANGED_NODE_HIDDEN: u32 = 0x0057_8170;

// Translated from 0045c6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map names this `TES::GetLODMult`; the body takes a form and returns a
/// class from its form type (`TESForm::GetFormType`, minus `0x18`): 3 for
/// types `0x2A` and `0x2B`; 2 for types `0x18`, `0x19`, `0x1A`, `0x1D`,
/// `0x1F`, `0x28`, `0x29`, `0x2E`, `0x2F`, `0x32`, `0x67`, `0x6C`, `0x73` and
/// `0x74`; 1 for every other type.
pub fn fn_0045c6b0(e: &mut Engine, form: Ptr) -> u32 {
    let form_type = e.call(FORM_GET_TYPE, &args![form]).u32();
    if form_type.wrapping_sub(0x18) > 0x5c {
        return 1;
    }
    match form_type {
        0x2a | 0x2b => 3,
        0x18 | 0x19 | 0x1a | 0x1d | 0x1f | 0x28 | 0x29 | 0x2e | 0x2f | 0x32 | 0x67 | 0x6c
        | 0x73 | 0x74 => 2,
        _ => 1,
    }
}

// ----- the grid cell passes (`0045c780` to `0045cac0`) -----

/// A byte; when set the pass of `fn_0045c780` marks only the centre cell
/// (2, 2) of the 5x5 grid.
const GRID_PASS_CENTRE_ONLY: u32 = 0x011c_3c0c;
/// The 5x5 table of flags `fn_0045c780` marks cells with (row `x`, column
/// `y`).
const GRID_FLAGS_FIRST: u32 = 0x0101_7830;
/// The 5x5 table of flags `fn_0045c840` marks cells with.
const GRID_FLAGS_SECOND: u32 = 0x0101_784c;
/// `ecx` = the grid array: its side length.
const GRID_SIDE: u32 = 0x0084_e3a0;
/// Returns the object used by the two marking calls (no arguments).
const MARKING_CONTEXT: u32 = 0x0043_8220;
/// `ecx` = an object, argument = the context: the target of the marking call.
const MARKING_TARGET: u32 = 0x00a5_9d30;
/// `ecx` = the marking target: (operation, flag).
const MARKING_APPLY: u32 = 0x0044_1130;
/// `ecx` = a cell: its scene object (0 when it has none).
const CELL_SCENE_OBJECT: u32 = 0x0054_6fb0;
/// `ecx` = a cell scene object: whether the cell is to be refreshed.
const SCENE_OBJECT_NEEDS_REFRESH: u32 = 0x0045_cb90;
/// `ecx` = a cell scene object: refreshes it.
const SCENE_OBJECT_REFRESH: u32 = 0x0054_00e0;

// Translated from 0045c780 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `uGridsToLoad` is 5: for every cell (x, y) of the grid array, runs
/// `fn_0045c8d0(x, y, flag)`, the flag being (x == 2 and y == 2) when the byte
/// at `011c3c0c` is set, else the byte at row `x`, column `y` of the 5x5 table
/// at `01017830`.
pub fn fn_0045c780(e: &mut Engine, this: Ptr<TES>) {
    if grids_to_load(e) != 5 {
        return;
    }
    let mut x = 0;
    while x < grid_side(e, this) {
        let mut y = 0;
        while y < grid_side(e, this) {
            let flag = if e.mem.u8(GRID_PASS_CENTRE_ONLY) != 0 {
                (x == 2 && y == 2) as u8
            } else {
                e.mem.u8(GRID_FLAGS_FIRST + x * 5 + y)
            };
            fn_0045c8d0(e, this, x, y, flag);
            y += 1;
        }
        x += 1;
    }
}

/// The side length of the grid array of `this`.
fn grid_side(e: &mut Engine, this: Ptr<TES>) -> u32 {
    let grid = e.get(this, TES::pGridCellA);
    e.call(GRID_SIDE, &args![grid]).u32()
}

// Translated from 0045c840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `fn_0045c780` but with the second table (`0101784c`) and
/// `fn_0045c9e0`, and without the centre-only switch.
pub fn fn_0045c840(e: &mut Engine, this: Ptr<TES>) {
    if grids_to_load(e) != 5 {
        return;
    }
    let mut x = 0;
    while x < grid_side(e, this) {
        let mut y = 0;
        while y < grid_side(e, this) {
            let flag = e.mem.u8(GRID_FLAGS_SECOND + x * 5 + y);
            fn_0045c9e0(e, this, x, y, flag);
            y += 1;
        }
        x += 1;
    }
}

/// For the grid cell at (x, y), when it is loaded, for the
/// two marking passes: for each of its four list-2 children, the
/// object its first child's slot `0x18` returns is passed to the marking call
/// with `operation` and `flag`.
fn mark_cell_objects(e: &mut Engine, this: Ptr<TES>, x: u32, y: u32, operation: u32, flag: u32) {
    let grid = e.get(this, TES::pGridCellA);
    let slot = e.call(GRID_CELL_ARRAY_GET, &args![grid, x, y]).u32();
    if slot == 0 || e.mem.u32(slot) == 0 {
        return;
    }
    let mut index = 0;
    while index < 4 {
        let cell = e.mem.u32(slot);
        let node = fn_0045c9a0(e, Ptr::new(cell), index);
        let object = if fn_0045bc00(e, Ptr::new(node), 0) != 0 {
            let first = e.call(NODE_CHILD_AT, &args![node, 0u32]).u32();
            e.vcall(first, 0x18, &[]).u32()
        } else {
            0
        };
        if object != 0 {
            let context = e.call(MARKING_CONTEXT, &[]).u32();
            let target = e.call(MARKING_TARGET, &args![object, context]).u32();
            e.call(MARKING_APPLY, &args![target, operation, flag]);
        }
        index += 1;
    }
}

// Translated from 0045c8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For the grid cell at (x, y), when it is loaded: for each of its four
/// quadrant nodes (`fn_0045c9a0`), takes the object of the node's first
/// child (its slot `0x18`) and applies `00441130` with operation `0x2E` and
/// `flag`.
pub fn fn_0045c8d0(e: &mut Engine, this: Ptr<TES>, x: u32, y: u32, flag: u8) {
    mark_cell_objects(e, this, x, y, 0x2e, flag as u32);
}

// Translated from 0045c9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entry `index` of the cell's list 2 (`fn_00456fc0(cell, 2)`), or 0 when the
/// cell has no such list.
pub fn fn_0045c9a0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let list = e.call(CELL_GET_LIST, &args![this, 2u32]).u32();
    if list != 0 {
        e.call(NODE_CHILD_AT, &args![list, index]).u32()
    } else {
        0
    }
}

// Translated from 0045c9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same walk as `fn_0045c8d0` with operation `0x34` and the flag
/// inverted (1 when `flag` is 0).
pub fn fn_0045c9e0(e: &mut Engine, this: Ptr<TES>, x: u32, y: u32, flag: u8) {
    mark_cell_objects(e, this, x, y, 0x34, (flag == 0) as u32);
}

// Translated from 0045cac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::InitDistantTextureBlending` (Xbox PDB): for every cell (x, y) of the
/// `uGridsToLoad` square whose slot is loaded and which has a scene object
/// (`00546fb0`) that `0045cb90` accepts, calls `005400e0` on the cell's scene
/// object.
pub fn tes_init_distant_texture_blending(e: &mut Engine, this: Ptr<TES>) {
    let mut x = 0;
    while x < grids_to_load(e) {
        let mut y = 0;
        while y < grids_to_load(e) {
            let cell = grid_cell_at(e, this, x, y);
            let object = if cell != 0 {
                e.call(CELL_SCENE_OBJECT, &args![cell]).u32()
            } else {
                0
            };
            if cell != 0 && object != 0 && e.call(SCENE_OBJECT_NEEDS_REFRESH, &args![object]).bool()
            {
                let object = e.call(CELL_SCENE_OBJECT, &args![cell]).u32();
                e.call(SCENE_OBJECT_REFRESH, &args![object]);
            }
            y += 1;
        }
        x += 1;
    }
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004579e0, fn_004579e0(f32) -> f32),
        entry!(0x00457a00, make_new_geom_data(Ptr)),
        entry!(0x00457b80, fn_00457b80(Ptr) -> u16),
        entry!(
            0x00457ba0,
            tes_create_deep_copy_same_textures(Ptr<TES>, Ptr, u32) -> Ptr
        ),
        entry!(0x00457be0, tes_init_models_to_load(Ptr<TES>, u32)),
        entry!(0x00457d70, fn_00457d70(Ptr<TES>, u8, Ptr, u8) -> bool),
        entry!(0x00457fe0, fn_00457fe0() -> u32),
        entry!(0x00457ff0, fn_00457ff0(Ptr)),
        entry!(0x00458010, fn_00458010(Ptr)),
        entry!(0x00458030, fn_00458030() -> u32),
        entry!(0x00458060, fn_00458060(u8)),
        entry!(0x00458080, fn_00458080(Ptr, u32, u8)),
        entry!(0x004580e0, fn_004580e0() -> bool),
        entry!(0x00458110, fn_00458110(Ptr, u32) -> bool),
        entry!(0x00458140, fn_00458140() -> bool),
        entry!(0x00458180, fn_00458180(Ptr)),
        entry!(0x004581c0, fn_004581c0(u8)),
        entry!(0x004581e0, fn_004581e0(u32, u32) -> u32),
        entry!(0x00458200, tes_set_world_space(Ptr<TES>, Ptr)),
        entry!(0x004583f0, fn_004583f0(u8)),
        entry!(0x00458400, fn_00458400(Ptr) -> u32),
        entry!(0x00458420, tes_pick(Ptr<TES>, Ptr) -> u32),
        entry!(0x00458440, tes_pick_ov2(Ptr<TES>, Ptr, u8) -> u32),
        entry!(0x004584d0, tes_pick_ni(Ptr<TES>, Ptr) -> u32),
        entry!(0x004585f0, fn_004585f0(Ptr, Ptr) -> Ptr),
        entry!(0x00458620, fn_00458620(Ptr, Ptr) -> Ptr),
        entry!(0x004586d0, fn_004586d0(Ptr) -> f32),
        entry!(0x00458700, fn_00458700(Ptr, Ptr, i32) -> Ptr),
        entry!(0x004587d0, fn_004587d0(f32) -> f32),
        entry!(0x004587f0, fn_004587f0(Ptr, Ptr) -> Ptr),
        entry!(0x00458820, fn_00458820(Ptr) -> Ptr),
        entry!(0x00458840, bhk_pick_data_calc_to(Ptr)),
        entry!(0x00458880, fn_00458880(Ptr, Ptr, Ptr)),
        entry!(0x004588d0, fn_004588d0(Ptr) -> bool),
        entry!(0x00458900, fn_00458900(Ptr, Ptr, f32) -> bool),
        entry!(0x00458990, fn_00458990(Ptr, u32) -> bool),
        entry!(0x004589c0, fn_004589c0(Ptr, f32)),
        entry!(0x00458a10, fn_00458a10(Ptr, Ptr, Ptr)),
        entry!(0x00458a60, fn_00458a60(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00458ad0, fn_00458ad0(Ptr, Ptr)),
        entry!(0x00458b20, fn_00458b20() -> Ptr),
        entry!(0x00458b30, fn_00458b30(Ptr, u8)),
        entry!(0x00458b50, fn_00458b50(Ptr) -> u32),
        entry!(
            0x00458b70,
            tes_get_map_name_for_location(Ptr<TES>, u32, u32, u32, u32, Ptr) -> u32
        ),
        entry!(0x00458be0, tes_get_cell_priority(Ptr<TES>, Ptr, Ptr) -> u32),
        entry!(0x00458e20, tes_add_temp_debug_object(Ptr<TES>, Ptr, f32)),
        entry!(0x00458ef0, fn_00458ef0(Ptr, f32) -> Ptr),
        entry!(0x00458f20, bs_temp_node_get_rtti(Ptr) -> Ptr),
        entry!(
            0x00458f30,
            bs_temp_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00458f70, fn_00458f70(Ptr)),
        entry!(0x00458f90, fn_00458f90(Ptr<TES>)),
        entry!(0x00459000, fn_00459000(Ptr<TES>, u32) -> u16),
        entry!(0x00459060, fn_00459060(Ptr<TES>, u32, i16)),
        entry!(0x00459100, fn_00459100(Ptr<TES>) -> u16),
        entry!(0x00459230, tes_save_game(Ptr<TES>)),
        entry!(0x00459470, fn_00459470(Ptr<TES>, u32) -> u32),
        entry!(0x00459490, tes_save_game_ov2(Ptr<TES>, Ptr)),
        entry!(0x004595e0, tes_load_game(Ptr<TES>, Ptr)),
        entry!(0x00459750, fn_00459750(Ptr<TES>)),
        entry!(0x00459840, fn_00459840(Ptr<TES>)),
        entry!(0x00459870, tes_create_furniture_list(Ptr<TES>)),
        entry!(0x00459920, fn_00459920(Ptr<TES>)),
        entry!(0x00459a00, fn_00459a00(Ptr<TES>)),
        entry!(0x00459ae0, tes_run_cell_test(Ptr<TES>, u32)),
        entry!(0x00459d20, tes_test_cell(Ptr<TES>, Ptr, u32, u32)),
        entry!(0x00459f10, tes_preload_addon_nodes(Ptr<TES>)),
        entry!(0x0045a120, fn_0045a120(Ptr) -> u32),
        entry!(0x0045a140, fn_0045a140(Ptr, u16)),
        entry!(0x0045a160, fn_0045a160(Ptr, Ptr)),
        entry!(0x0045a190, fn_0045a190() -> u32),
        entry!(0x0045a1a0, fn_0045a1a0(Ptr) -> u16),
        entry!(0x0045a1c0, fn_0045a1c0(Ptr<TES>)),
        entry!(0x0045a1e0, tes_preload_forms(Ptr<TES>)),
        entry!(0x0045a330, fn_0045a330(Ptr) -> u32),
        entry!(0x0045a350, fn_0045a350(Ptr<TES>)),
        entry!(0x0045a370, fn_0045a370(Ptr<TES>)),
        entry!(0x0045a520, fn_0045a520(Ptr<TES>)),
        entry!(0x0045a5e0, fn_0045a5e0(Ptr, u32)),
        entry!(0x0045a600, fn_0045a600(Ptr<TES>)),
        entry!(0x0045a730, fn_0045a730(Ptr) -> u32),
        entry!(0x0045a750, tes_setup_temporary_particle_cache(Ptr<TES>)),
        entry!(
            0x0045a9a0,
            tes_add_temporary_particle_object_to_cache(Ptr<TES>, u32)
        ),
        entry!(0x0045ab30, fn_0045ab30(Ptr) -> Ptr),
        entry!(
            0x0045aba0,
            tes_get_unused_cached_particle_object(Ptr<TES>, u32, u32) -> u32
        ),
        entry!(0x0045ac80, fn_0045ac80(Ptr<TES>)),
        entry!(0x0045ae20, fn_0045ae20(Ptr, u32) -> Ptr),
        entry!(0x0045ae50, fn_0045ae50(Ptr)),
        entry!(0x0045aee0, fn_0045aee0(Ptr<TES>, u8)),
        entry!(0x0045af00, tes_get_nav_mesh_info_map(Ptr<TES>) -> Ptr),
        entry!(0x0045afb0, tes_set_nav_mesh_info_map(Ptr<TES>, Ptr)),
        entry!(0x0045b020, fn_0045b020()),
        entry!(0x0045b050, fn_0045b050()),
        entry!(0x0045b070, fn_0045b070(Ptr<TES>, u32)),
        entry!(0x0045bad0, fn_0045bad0(u32, Ptr) -> bool),
        entry!(0x0045baf0, fn_0045baf0(Ptr, u32) -> bool),
        entry!(0x0045bb20, fn_0045bb20(Ptr, Ptr, f32) -> Ptr),
        entry!(0x0045bb80, fn_0045bb80(Ptr) -> Ptr),
        entry!(0x0045bba0, fn_0045bba0(Ptr, Ptr) -> Ptr),
        entry!(0x0045bbe0, fn_0045bbe0(Ptr) -> Ptr),
        entry!(0x0045bc00, fn_0045bc00(Ptr, u32) -> u32),
        entry!(0x0045bc40, fn_0045bc40(Ptr, u32)),
        entry!(0x0045bc60, fn_0045bc60(Ptr)),
        entry!(0x0045bc80, fn_0045bc80(Ptr<TES>, u8, u8)),
        entry!(0x0045c4a0, fn_0045c4a0(Ptr)),
        entry!(0x0045c4c0, fn_0045c4c0(Ptr) -> u32),
        entry!(0x0045c4e0, fn_0045c4e0(Ptr, u32)),
        entry!(0x0045c520, fn_0045c520(Ptr)),
        entry!(0x0045c570, fn_0045c570(Ptr, u32)),
        entry!(0x0045c5f0, fn_0045c5f0(Ptr, u32) -> Ptr),
        entry!(
            0x0045c620,
            ni_frustum_planes_ni_frustum_planes(Ptr) -> Ptr
        ),
        entry!(0x0045c650, fn_0045c650(Ptr) -> Ptr),
        entry!(0x0045c670, fn_0045c670() -> u32),
        entry!(0x0045c680, tes_ranged_node_visibility_changed_cb(u32, u8)),
        entry!(0x0045c6b0, fn_0045c6b0(Ptr) -> u32),
        entry!(0x0045c780, fn_0045c780(Ptr<TES>)),
        entry!(0x0045c840, fn_0045c840(Ptr<TES>)),
        entry!(0x0045c8d0, fn_0045c8d0(Ptr<TES>, u32, u32, u8)),
        entry!(0x0045c9a0, fn_0045c9a0(Ptr, u32) -> u32),
        entry!(0x0045c9e0, fn_0045c9e0(Ptr<TES>, u32, u32, u8)),
        entry!(0x0045cac0, tes_init_distant_texture_blending(Ptr<TES>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// A double that records the argument words of each call and returns
    /// `value`.
    fn recording(e: &mut Engine, addr: u32, value: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(vec![]));
        let inner = log.clone();
        e.register_double(addr, move |_, a| {
            inner.borrow_mut().push(a.to_vec());
            ret(value)
        });
        log
    }

    /// The pages holding the globals this file reads.
    fn map_globals(e: &mut Engine) {
        for page in [
            0x0101_7000u32,
            0x011a_d000,
            0x011c_3000,
            0x011c_6000,
            0x011d_9000,
            0x011d_a000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
    }

    /// A setting object (value at +4) at `addr`, and the value-pointer
    /// functions for it.
    fn put_setting(e: &mut Engine, addr: u32, value: u32) {
        e.mem.set_u32(addr + 4, value);
        for function in [SETTING_VALUE_ADDRESS_INT, SETTING_VALUE_ADDRESS_BYTE] {
            e.register(function, |_, a| ret(a[0] + 4));
        }
    }

    /// A vtable with the given (byte offset, function address) slots.
    fn vtable(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x400);
        for (offset, function) in slots {
            e.mem.set_u32(table + offset, *function);
        }
        table
    }

    fn object_with(e: &mut Engine, table: u32) -> Ptr {
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, table);
        Ptr::new(object)
    }

    fn ni_pointer_doubles(e: &mut Engine) {
        e.register(NI_POINTER_CTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(NI_POINTER_DTOR, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
    }

    fn vector(e: &mut Engine, lanes: [f32; 4]) -> Ptr {
        let vector = Ptr::<()>::new(e.mem.alloc(16));
        for (lane, value) in lanes.iter().enumerate() {
            e.mem.set_f32(vector.addr() + 4 * lane as u32, *value);
        }
        vector
    }

    fn lanes(e: &Engine, vector: Ptr) -> [f32; 4] {
        [0, 1, 2, 3].map(|lane| e.mem.f32(vector.addr() + 4 * lane))
    }

    /// The vector constructor (returns `this`), the vector copy, the constants
    /// the vector code reads and the zero vector `00458b20` returns.
    fn vector_engine() -> Engine {
        let mut e = Engine::new();
        map_globals(&mut e);
        e.register(VECTOR_CTOR, |_, a| ret(a[0]));
        e.register(VECTOR_COPY, |e, a| {
            for word in 0..4 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            ret(a[0])
        });
        for word in 0..4 {
            e.set_global(ABS_MASK + 4 * word, 0x7fff_ffffu32);
        }
        e.set_global(PICK_EPSILON, 0.001f32);
        e.set_global(HAVOK_SCALE, 2.0f32);
        let zero = Ptr::<()>::new(e.mem.alloc(16));
        e.register_double(0x0045_8b20, move |_, _| ret(zero.addr()));
        e
    }

    // ----- 004579e0 -----

    #[test]
    fn float_wrapper_returns_the_callee_result() {
        let mut e = Engine::new();
        e.register(0x0040_19b0, |_, a| Ret {
            st0: f32::from_bits(a[0]) as f64 * 2.0,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0045_79e0, &args![1.5f32]).f32(), 3.0);
    }

    // ----- 00457a00, 00457b80, 00457ba0 -----

    struct Geometry {
        e: Engine,
        node: Ptr,
        installs: Log,
        copy_object: u32,
        shape: u32,
    }

    /// A node whose slot 0x18 gives a geometry object holding `shape` (whose
    /// word at +0xE is `flags`); `has_form` is what `007af430` says.
    fn geometry_world(flags: u16, has_form: bool) -> Geometry {
        let mut e = Engine::new();
        ni_pointer_doubles(&mut e);
        let geometry_vtable = vtable(&mut e, &[(0xe4, 0x7000_00e4)]);
        let geometry = object_with(&mut e, geometry_vtable).addr();
        let installs = recording(&mut e, 0x7000_00e4, 0);
        let node_vtable = vtable(&mut e, &[(0x18, 0x7000_0018), (0xc, 0x7000_000c)]);
        let node = object_with(&mut e, node_vtable);
        returns(&mut e, 0x7000_0018, geometry);
        returns(&mut e, 0x7000_000c, 0);
        let shape = e.mem.alloc(0x20);
        e.mem.set_u16(shape + 0xe, flags);
        returns(&mut e, 0x0054_95f0, shape);
        returns(&mut e, 0x007a_f430, has_form as u32);
        let copy_object = e.mem.alloc(0x20);
        e.register_double(0x00a5_d510, move |e, a| {
            e.mem.set_u32(a[1], copy_object);
            Ret::default()
        });
        Geometry {
            e,
            node,
            installs,
            copy_object,
            shape,
        }
    }

    #[test]
    fn geom_data_is_copied_when_the_shape_has_a_form_and_no_flags() {
        let mut world = geometry_world(0x0fff, true);
        world.e.call(0x0045_7a00, &args![world.node]);
        let installs = world.installs.borrow();
        assert_eq!(installs.len(), 1);
        assert_eq!(installs[0][1], world.copy_object);
    }

    #[test]
    fn geom_data_keeps_the_original_without_a_form() {
        let mut world = geometry_world(0, false);
        world.e.call(0x0045_7a00, &args![world.node]);
        let installs = world.installs.borrow();
        assert_eq!(installs.len(), 1);
        assert_eq!(installs[0][1], world.shape);
    }

    #[test]
    fn geom_data_keeps_the_original_when_a_flag_bit_is_set() {
        let mut world = geometry_world(0x1000, true);
        let form_checks = recording(&mut world.e, 0x007a_f430, 1);
        world.e.call(0x0045_7a00, &args![world.node]);
        assert!(form_checks.borrow().is_empty());
        assert_eq!(world.installs.borrow()[0][1], world.shape);
    }

    #[test]
    fn geom_data_walks_the_children_last_to_first() {
        let mut e = Engine::new();
        ni_pointer_doubles(&mut e);
        // Leaves have neither geometry nor children; the parent's slot 0xC
        // gives a child list.
        let leaf_vtable = vtable(&mut e, &[(0x18, 0x7000_0018), (0xc, 0x7000_000c)]);
        let parent_vtable = vtable(&mut e, &[(0x18, 0x7000_0018), (0xc, 0x7000_0010)]);
        let parent = object_with(&mut e, parent_vtable);
        returns(&mut e, 0x7000_0010, 0x0aaa_0000);
        returns(&mut e, 0x7000_000c, 0);
        let geometry_asks = recording(&mut e, 0x7000_0018, 0);
        returns(&mut e, 0x0043_b480, 2);
        let children = [
            object_with(&mut e, leaf_vtable).addr(),
            object_with(&mut e, leaf_vtable).addr(),
        ];
        let order: Log = Rc::new(RefCell::new(vec![]));
        let inner = order.clone();
        e.register_double(0x0043_b4a0, move |_, a| {
            inner.borrow_mut().push(a.to_vec());
            ret(children[a[1] as usize])
        });
        e.call(0x0045_7a00, &args![parent]);
        // The child at index 1 comes before the child at index 0.
        let indexes: Vec<u32> = order.borrow().iter().map(|call| call[1]).collect();
        assert_eq!(indexes, vec![1, 0]);
        // The parent and its two children each asked for their geometry.
        let askers: Vec<u32> = geometry_asks.borrow().iter().map(|call| call[0]).collect();
        assert_eq!(askers, vec![parent.addr(), children[1], children[0]]);
    }

    #[test]
    fn geom_data_of_a_null_node_only_builds_and_drops_the_pointer() {
        let mut e = Engine::new();
        ni_pointer_doubles(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0045_7a00, &args![0u32]);
        let log = e.call_log.take().unwrap();
        let addrs: Vec<u32> = log.iter().map(|call| call.0).collect();
        assert_eq!(addrs, vec![0x0045_7a00, NI_POINTER_CTOR, NI_POINTER_DTOR]);
    }

    #[test]
    fn flag_word_is_masked_with_7000() {
        let mut e = Engine::new();
        let object = Ptr::<()>::new(e.mem.alloc(0x20));
        e.mem.set_u16(object.addr() + 0xe, 0xffff);
        assert_eq!(e.call(0x0045_7b80, &args![object]).u32(), 0x7000);
        e.mem.set_u16(object.addr() + 0xe, 0x0fff);
        assert_eq!(e.call(0x0045_7b80, &args![object]).u32(), 0);
    }

    #[test]
    fn deep_copy_clones_the_node_and_rebuilds_its_geometry() {
        let mut e = Engine::new();
        ni_pointer_doubles(&mut e);
        let leaf_vtable = vtable(&mut e, &[(0x18, 0x7000_0018), (0xc, 0x7000_000c)]);
        let clone = object_with(&mut e, leaf_vtable);
        returns(&mut e, 0x7000_0018, 0);
        returns(&mut e, 0x7000_000c, 0);
        let clones = recording(&mut e, 0x00a5_d2c0, clone.addr());
        let tes: Ptr<TES> = e.new_object();
        let result = e.call(0x0045_7ba0, &args![tes, 0x1234u32, 7u32]).u32();
        assert_eq!(result, clone.addr());
        assert_eq!(clones.borrow()[0], vec![0x1234, 7]);
        // A null node gives null without cloning.
        assert_eq!(e.call(0x0045_7ba0, &args![tes, 0u32, 7u32]).u32(), 0);
        assert_eq!(clones.borrow().len(), 1);
    }

    // ----- 00457be0 -----

    struct Grid {
        e: Engine,
        tes: Ptr<TES>,
        loads: Log,
        counted: Log,
    }

    /// A 3 x 3 grid around (10, 20) in world space 0x9000 where every
    /// `LoadCell` succeeds except (9, 19); `00454400` counts `count` per cell.
    fn grid_world(count: u32) -> Grid {
        let mut e = Engine::new();
        map_globals(&mut e);
        put_setting(&mut e, SETTING_GRIDS_TO_LOAD, 3);
        let tes: Ptr<TES> = e.new_object();
        e.set(tes, TES::iCurrentGridX, 10);
        e.set(tes, TES::iCurrentGridY, 20);
        e.set(tes, TES::pGridCellA, Ptr::new(0x0bad_0000));
        e.set(tes, TES::pWorldSpace, Ptr::new(0x9000));
        returns(&mut e, GET_WORLD_SPACE, 0x9000);
        e.set_global(DATA_HANDLER, 0x6000u32);
        e.register_double(WORLD_SPACE_LOAD_CELL, |_, a| {
            ret(if (a[1], a[2]) == (9, 19) {
                0
            } else {
                0x5000 + a[1] * 16 + a[2]
            })
        });
        let loads = recording(&mut e, DATA_HANDLER_CREATE_CELL, 0x7777);
        returns(&mut e, 0x0045_0fb0, 0);
        let counted = recording(&mut e, 0x0045_4400, count);
        // The loading menu code stops at "a load is pending".
        returns(&mut e, 0x0070_5e80, 0);
        returns(&mut e, 0x0045_1530, 1);
        Grid {
            e,
            tes,
            loads,
            counted,
        }
    }

    #[test]
    fn models_load_every_cell_of_the_square_for_other_modes() {
        let mut grid = grid_world(1);
        let pending = recording(&mut grid.e, 0x0045_1530, 1);
        let loaded = recording(&mut grid.e, WORLD_SPACE_LOAD_CELL, 0x5001);
        grid.e.call(0x0045_7be0, &args![grid.tes, 2u32]);
        let coordinates: Vec<(u32, u32)> = loaded.borrow().iter().map(|a| (a[1], a[2])).collect();
        // First cell is 9, 19: the centre (10, 20) minus uGridsToLoad / 2;
        // x is the outer loop.
        assert_eq!(coordinates.len(), 9);
        assert_eq!(coordinates[0], (9, 19));
        assert_eq!(coordinates[1], (9, 20));
        assert_eq!(coordinates[3], (10, 19));
        assert_eq!(coordinates[8], (11, 21));
        assert_eq!(grid.counted.borrow().len(), 9);
        // Something was counted: the loading menu code ran (show = 1).
        assert_eq!(pending.borrow().len(), 1);
    }

    #[test]
    fn models_fall_back_to_creating_the_cell() {
        let mut grid = grid_world(1);
        grid.e.call(0x0045_7be0, &args![grid.tes, 2u32]);
        // LoadCell failed at (9, 19): the data handler created that cell in
        // the world space `004fd3e0` gives, at the same coordinates.
        let created = grid.loads.borrow();
        assert_eq!(created.len(), 1);
        assert_eq!(created[0], vec![0x6000, 9, 19, 0x9000, 1]);
        // 0x7777 is among the cells counted.
        assert!(grid.counted.borrow().iter().any(|a| a[0] == 0x7777));
    }

    #[test]
    fn models_mode_zero_skips_attached_cells() {
        let mut grid = grid_world(1);
        // Attached cells are the ones in row y = 1.
        grid.e
            .register(GRID_CELL_ARRAY_GET_ATTACHED, |_, a| ret((a[2] == 1) as u32));
        grid.e.call(0x0045_7be0, &args![grid.tes, 0u32]);
        assert_eq!(grid.counted.borrow().len(), 6);
    }

    #[test]
    fn models_mode_one_skips_cells_the_array_has() {
        let mut grid = grid_world(1);
        // The array has the cells of column x = 0.
        grid.e
            .register(GRID_CELL_ARRAY_GET, |_, a| ret((a[1] == 0) as u32));
        grid.e.call(0x0045_7be0, &args![grid.tes, 1u32]);
        assert_eq!(grid.counted.borrow().len(), 6);
    }

    #[test]
    fn models_do_not_call_the_loading_menu_when_nothing_was_counted() {
        let mut grid = grid_world(0);
        let pending = recording(&mut grid.e, 0x0045_1530, 1);
        grid.e.call(0x0045_7be0, &args![grid.tes, 2u32]);
        assert!(pending.borrow().is_empty());
    }

    #[test]
    fn models_load_nothing_without_a_world_space() {
        let mut grid = grid_world(1);
        returns(&mut grid.e, GET_WORLD_SPACE, 0);
        grid.e.call(0x0045_7be0, &args![grid.tes, 2u32]);
        assert!(grid.counted.borrow().is_empty());
    }

    // ----- 00457d70 -----

    struct Menu {
        e: Engine,
        tes: Ptr<TES>,
        menu: u32,
        ticks: Rc<std::cell::Cell<u32>>,
        modes: Log,
        closes: Log,
        created: Log,
    }

    /// A loading menu with the given state word, a HUD main menu in mode
    /// `hud_mode`, a settable tick count and recording doubles for the menu
    /// calls.
    fn menu_world(state: u16, hud_mode: u32) -> Menu {
        let mut e = Engine::new();
        map_globals(&mut e);
        let menu = e.mem.alloc(0x300);
        e.mem.set_u16(menu + 0x222, state);
        e.set_global(LOADING_MENU, menu);
        let hud = e.mem.alloc(0x200);
        e.mem.set_u32(hud + 0x1c4, hud_mode);
        e.set_global(HUD_MAIN_MENU, hud);
        let ticks = Rc::new(std::cell::Cell::new(1000u32));
        let inner = ticks.clone();
        e.register_double(GET_TICK_COUNT, move |_, _| ret(inner.get()));
        let modes = recording(&mut e, 0x0077_1700, 0);
        let closes = recording(&mut e, 0x0070_5e30, 0);
        let created = recording(&mut e, 0x0070_5e00, 0);
        let tes: Ptr<TES> = e.new_object();
        Menu {
            e,
            tes,
            menu,
            ticks,
            modes,
            closes,
            created,
        }
    }

    #[test]
    fn hiding_a_menu_that_is_not_open_closes_it() {
        let mut world = menu_world(0, 0);
        let shown = world.e.call(0x0045_7d70, &args![world.tes, 0u8, 0u32, 0u8]);
        assert!(!shown.bool());
        assert_eq!(world.closes.borrow().len(), 1);
        // The HUD main menu is switched to mode 2 unless it is in mode 0x18.
        assert_eq!(world.modes.borrow()[0], vec![2]);
        assert_eq!(world.e.global::<u32>(LOADING_MENU_DEADLINE), 0);
    }

    #[test]
    fn hiding_in_hud_mode_0x18_leaves_the_mode_alone() {
        let mut world = menu_world(0, 0x18);
        world.e.call(0x0045_7d70, &args![world.tes, 0u8, 0u32, 0u8]);
        assert_eq!(world.closes.borrow().len(), 1);
        assert!(world.modes.borrow().is_empty());
    }

    #[test]
    fn hiding_an_open_menu_waits_five_seconds_then_closes() {
        let mut world = menu_world(1 << 10, 0);
        world.e.call(0x0045_7d70, &args![world.tes, 0u8, 0u32, 0u8]);
        assert!(world.closes.borrow().is_empty());
        assert_eq!(world.e.global::<u32>(LOADING_MENU_DEADLINE), 6000);
        // Still open before the deadline (the deadline stays).
        world.ticks.set(6000);
        world.e.call(0x0045_7d70, &args![world.tes, 0u8, 0u32, 0u8]);
        assert!(world.closes.borrow().is_empty());
        world.ticks.set(6001);
        world.e.call(0x0045_7d70, &args![world.tes, 0u8, 0u32, 0u8]);
        assert_eq!(world.closes.borrow().len(), 1);
        assert_eq!(world.e.global::<u32>(LOADING_MENU_DEADLINE), 0);
        // Bit 10 was cleared by the close.
        assert_eq!(world.e.mem.u16(world.menu + 0x222), 0);
    }

    #[test]
    fn hiding_an_open_menu_with_the_force_flag_closes_at_once() {
        let mut world = menu_world(1 << 10, 0);
        world.e.call(0x0045_7d70, &args![world.tes, 0u8, 0u32, 1u8]);
        assert_eq!(world.closes.borrow().len(), 1);
    }

    #[test]
    fn showing_a_visible_menu_without_bit_3_only_hands_over_the_form() {
        let mut world = menu_world(0, 0);
        returns(&mut world.e, 0x0070_5e80, 1);
        let handed = recording(&mut world.e, 0x0078_8ab0, 0);
        let shown = world
            .e
            .call(0x0045_7d70, &args![world.tes, 1u8, 0x4444u32, 0u8]);
        assert!(!shown.bool());
        assert_eq!(handed.borrow()[0], vec![world.menu, 0x4444]);
        // Bit 3 is now set, and nothing else ran.
        assert_eq!(world.e.mem.u16(world.menu + 0x222), 1 << 3);
        assert!(world.created.borrow().is_empty());
    }

    #[test]
    fn showing_does_nothing_while_a_load_is_pending() {
        let mut world = menu_world(0, 0);
        returns(&mut world.e, 0x0070_5e80, 0);
        returns(&mut world.e, 0x0045_1530, 1);
        let shown = world
            .e
            .call(0x0045_7d70, &args![world.tes, 1u8, 0x4444u32, 0u8]);
        assert!(!shown.bool());
        assert!(world.created.borrow().is_empty());
    }

    /// Everything the show path calls, with the `00aff100` object's state
    /// word at +0x5C set to 2 and its slot 0x30 recording that word.
    fn show_world() -> (Menu, Log) {
        let mut world = menu_world(0, 0);
        returns(&mut world.e, 0x0070_5e80, 0);
        returns(&mut world.e, 0x0045_1530, 0);
        noop(&mut world.e, &[0x0063_e010, 0x0045_3a70, 0x00ad_8780]);
        let object_vtable = vtable(&mut world.e, &[(0x30, 0x7000_0030)]);
        let object = object_with(&mut world.e, object_vtable);
        world.e.mem.set_u32(object.addr() + 0x5c, 2);
        returns(&mut world.e, 0x00af_f100, object.addr());
        let seen: Log = Rc::new(RefCell::new(vec![]));
        let inner = seen.clone();
        world.e.register_double(0x7000_0030, move |e, a| {
            inner.borrow_mut().push(vec![e.mem.u32(a[0] + 0x5c)]);
            Ret::default()
        });
        (world, seen)
    }

    #[test]
    fn showing_for_a_non_interior_cell_creates_the_menu_for_its_world_space() {
        let (mut world, seen) = show_world();
        returns(&mut world.e, FORM_GET_TYPE, 0x39);
        returns(&mut world.e, CELL_IS_INTERIOR, 0);
        returns(&mut world.e, CELL_GET_WORLD_SPACE, 0x8888);
        let shown = world
            .e
            .call(0x0045_7d70, &args![world.tes, 1u8, 0x4444u32, 1u8]);
        assert!(shown.bool());
        assert_eq!(world.created.borrow()[0], vec![0x8888, 1]);
        // The state word was 2 -> 1 before slot 0x30 ran and is 2 afterwards.
        assert_eq!(seen.borrow()[0], vec![1]);
        let object = world.e.call(0x00af_f100, &[]).u32();
        assert_eq!(world.e.mem.u32(object + 0x5c), 2);
    }

    #[test]
    fn showing_for_an_interior_cell_keeps_the_cell() {
        let (mut world, _) = show_world();
        returns(&mut world.e, FORM_GET_TYPE, 0x39);
        returns(&mut world.e, CELL_IS_INTERIOR, 1);
        world
            .e
            .call(0x0045_7d70, &args![world.tes, 1u8, 0x4444u32, 0u8]);
        assert_eq!(world.created.borrow()[0], vec![0x4444, 0]);
    }

    #[test]
    fn showing_for_a_reference_looks_the_world_space_up_but_passes_the_form() {
        let (mut world, _) = show_world();
        returns(&mut world.e, FORM_GET_TYPE, 0x3a);
        returns(&mut world.e, 0x008d_6f30, 0x5555);
        returns(&mut world.e, CELL_IS_INTERIOR, 0);
        let cell_lookup = recording(&mut world.e, CELL_GET_WORLD_SPACE, 0x8888);
        let reference_lookup = recording(&mut world.e, REFERENCE_GET_WORLD_SPACE, 0x8888);
        world
            .e
            .call(0x0045_7d70, &args![world.tes, 1u8, 0x4444u32, 0u8]);
        assert_eq!(cell_lookup.borrow()[0], vec![0x5555]);
        assert!(reference_lookup.borrow().is_empty());
        assert_eq!(world.created.borrow()[0], vec![0x4444, 0]);
        // Without a cell the reference's own world space is looked up.
        returns(&mut world.e, 0x008d_6f30, 0);
        world
            .e
            .call(0x0045_7d70, &args![world.tes, 1u8, 0x4444u32, 0u8]);
        assert_eq!(reference_lookup.borrow()[0], vec![0x4444]);
    }

    #[test]
    fn showing_for_other_form_types_does_no_lookup() {
        let (mut world, _) = show_world();
        let cell_lookup = recording(&mut world.e, CELL_GET_WORLD_SPACE, 0);
        let reference_lookup = recording(&mut world.e, REFERENCE_GET_WORLD_SPACE, 0);
        for form_type in [0x38u32, 0x41, 0x6a] {
            returns(&mut world.e, FORM_GET_TYPE, form_type);
            world
                .e
                .call(0x0045_7d70, &args![world.tes, 1u8, 0x4444u32, 0u8]);
        }
        assert!(cell_lookup.borrow().is_empty());
        assert!(reference_lookup.borrow().is_empty());
        // The reference type 0x69 does look up.
        returns(&mut world.e, FORM_GET_TYPE, 0x69);
        returns(&mut world.e, 0x008d_6f30, 0);
        world
            .e
            .call(0x0045_7d70, &args![world.tes, 1u8, 0x4444u32, 0u8]);
        assert_eq!(reference_lookup.borrow().len(), 1);
    }

    // ----- the loading menu accessors -----

    #[test]
    fn tick_count_comes_from_the_import() {
        let mut e = Engine::new();
        returns(&mut e, GET_TICK_COUNT, 4242);
        assert_eq!(e.call(0x0045_7fe0, &[]).u32(), 4242);
    }

    #[test]
    fn state_word_toggles_between_one_and_two() {
        let mut e = Engine::new();
        let object = Ptr::<()>::new(e.mem.alloc(0x80));
        e.mem.set_u32(object.addr() + 0x5c, 1);
        e.call(0x0045_7ff0, &args![object]);
        assert_eq!(e.mem.u32(object.addr() + 0x5c), 2);
        e.call(0x0045_7ff0, &args![object]);
        assert_eq!(e.mem.u32(object.addr() + 0x5c), 2);
        e.call(0x0045_8010, &args![object]);
        assert_eq!(e.mem.u32(object.addr() + 0x5c), 1);
        e.call(0x0045_8010, &args![object]);
        assert_eq!(e.mem.u32(object.addr() + 0x5c), 1);
        e.mem.set_u32(object.addr() + 0x5c, 7);
        e.call(0x0045_7ff0, &args![object]);
        e.call(0x0045_8010, &args![object]);
        assert_eq!(e.mem.u32(object.addr() + 0x5c), 7);
    }

    #[test]
    fn hud_mode_is_zero_without_a_hud() {
        let mut e = Engine::new();
        map_globals(&mut e);
        assert_eq!(e.call(0x0045_8030, &[]).u32(), 0);
        let hud = e.mem.alloc(0x200);
        e.mem.set_u32(hud + 0x1c4, 0x18);
        e.set_global(HUD_MAIN_MENU, hud);
        assert_eq!(e.call(0x0045_8030, &[]).u32(), 0x18);
    }

    #[test]
    fn menu_bits_are_set_and_cleared_in_the_state_word() {
        let mut e = Engine::new();
        let menu = Ptr::<()>::new(e.mem.alloc(0x300));
        e.call(0x0045_8080, &args![menu, 10u32, 1u8]);
        assert_eq!(e.mem.u16(menu.addr() + 0x222), 1 << 10);
        e.call(0x0045_8080, &args![menu, 3u32, 1u8]);
        assert_eq!(e.mem.u16(menu.addr() + 0x222), (1 << 10) | (1 << 3));
        e.call(0x0045_8080, &args![menu, 10u32, 0u8]);
        assert_eq!(e.mem.u16(menu.addr() + 0x222), 1 << 3);
        // The shift count wraps at 32: bit 35 is bit 3.
        e.call(0x0045_8080, &args![menu, 35u32, 0u8]);
        assert_eq!(e.mem.u16(menu.addr() + 0x222), 0);
        // Bits above 15 do not fit the word.
        e.call(0x0045_8080, &args![menu, 20u32, 1u8]);
        assert_eq!(e.mem.u16(menu.addr() + 0x222), 0);
    }

    #[test]
    fn menu_bits_are_read_from_the_state_word() {
        let mut e = Engine::new();
        let menu = Ptr::<()>::new(e.mem.alloc(0x300));
        e.mem.set_u16(menu.addr() + 0x222, 1 << 10);
        assert!(e.call(0x0045_8110, &args![menu, 10u32]).bool());
        assert!(!e.call(0x0045_8110, &args![menu, 3u32]).bool());
        assert!(e.call(0x0045_8110, &args![menu, 42u32]).bool());
    }

    #[test]
    fn menu_flag_accessors_need_a_loading_menu() {
        let mut e = Engine::new();
        map_globals(&mut e);
        // Without a loading menu: false, and the setters do nothing.
        assert!(!e.call(0x0045_80e0, &[]).bool());
        assert!(!e.call(0x0045_8140, &[]).bool());
        e.call(0x0045_8060, &args![1u8]);
        e.call(0x0045_81c0, &args![1u8]);
        let menu = e.mem.alloc(0x300);
        e.set_global(LOADING_MENU, menu);
        e.call(0x0045_8060, &args![1u8]);
        assert_eq!(e.mem.u16(menu + 0x222), 1 << 10);
        assert!(e.call(0x0045_80e0, &[]).bool());
        assert!(!e.call(0x0045_8140, &[]).bool());
        e.call(0x0045_81c0, &args![1u8]);
        assert!(e.call(0x0045_8140, &[]).bool());
        e.call(0x0045_8060, &args![0u8]);
        e.call(0x0045_81c0, &args![0u8]);
        assert_eq!(e.mem.u16(menu + 0x222), 0);
    }

    #[test]
    fn handing_a_form_to_the_menu_needs_a_menu_a_form_and_no_pending_word() {
        let mut e = Engine::new();
        map_globals(&mut e);
        let handed = recording(&mut e, 0x0078_8ab0, 0);
        e.call(0x0045_8180, &args![0x4444u32]);
        assert!(handed.borrow().is_empty());
        let menu = e.mem.alloc(0x300);
        e.set_global(LOADING_MENU, menu);
        e.call(0x0045_8180, &args![0u32]);
        assert!(handed.borrow().is_empty());
        e.mem.set_u32(menu + 0x1e8, 1);
        e.call(0x0045_8180, &args![0x4444u32]);
        assert!(handed.borrow().is_empty());
        e.mem.set_u32(menu + 0x1e8, 0);
        e.call(0x0045_8180, &args![0x4444u32]);
        assert_eq!(handed.borrow()[0], vec![menu, 0x4444]);
        assert_eq!(e.mem.u16(menu + 0x222), 1 << 3);
    }

    #[test]
    fn show_loading_area_message_setting_address_is_returned() {
        let mut e = Engine::new();
        let asked = recording(&mut e, SETTING_VALUE_ADDRESS_BYTE, 0x0123_4567);
        assert_eq!(e.call(0x0045_81e0, &args![0u32, 0u32]).u32(), 0x0123_4567);
        assert_eq!(asked.borrow()[0], vec![SETTING_SHOW_LOADING_AREA_MESSAGE]);
    }

    // ----- 00458200 -----

    struct WorldSpaceChange {
        e: Engine,
        tes: Ptr<TES>,
        new_space: Ptr,
        log: Vec<(u32, Vec<u32>)>,
    }

    /// Sets up the doubles for a switch to a new world space whose `+0xD0`
    /// word is `extra`, with `unload` the setting `bPreemptivelyUnloadCells`,
    /// `interior` the interior cell, `flag_byte` the byte `00454b50` points at
    /// and `known` what `005862e0` says.
    fn switch_world_space(
        unload: u32,
        interior: u32,
        extra: u32,
        flag_byte: u8,
        known: bool,
        unloaded: u32,
    ) -> WorldSpaceChange {
        let mut e = Engine::new();
        map_globals(&mut e);
        put_setting(&mut e, SETTING_PREEMPTIVELY_UNLOAD_CELLS, unload);
        let tes: Ptr<TES> = e.new_object();
        e.set(tes, TES::pWorldSpace, Ptr::new(0x1111));
        e.set_global(TES_OBJECT, 0x0bee_0000u32);
        returns(&mut e, GET_INTERIOR_CELL, interior);
        noop(
            &mut e,
            &[
                0x0045_3940,
                0x0045_2490,
                0x0052_6270,
                0x0058_eeb0,
                0x0045_0d80,
                0x0070_43c0,
                0x0070_9d20,
                0x0045_68c0,
                0x0052_d3b0,
                SCOPE_GUARD_CTOR,
                SCOPE_GUARD_DTOR,
            ],
        );
        ni_pointer_doubles(&mut e);
        returns(&mut e, 0x0045_5200, unloaded);
        returns(&mut e, 0x0045_4d50, 0);
        let new_space = Ptr::<()>::new(e.mem.alloc(0x100));
        e.mem.set_u32(new_space.addr() + 0xd0, extra);
        returns(&mut e, 0x0067_33e0, 0x3333);
        let byte = e.mem.alloc(8);
        e.mem.set_u8(byte, flag_byte);
        returns(&mut e, 0x0045_4b50, byte);
        returns(&mut e, 0x0070_ec90, 0x7a7a);
        returns(&mut e, 0x0058_62e0, known as u32);
        e.call_log = Some(vec![]);
        e.call(0x0045_8200, &args![tes, new_space]);
        let log = e.call_log.take().unwrap();
        WorldSpaceChange {
            e,
            tes,
            new_space,
            log,
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn switching_to_no_world_space_or_the_current_one_does_nothing() {
        let mut e = Engine::new();
        let tes: Ptr<TES> = e.new_object();
        e.set(tes, TES::pWorldSpace, Ptr::new(0x1111));
        e.call_log = Some(vec![]);
        e.call(0x0045_8200, &args![tes, 0u32]);
        e.call(0x0045_8200, &args![tes, 0x1111u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn switching_without_unloading_stores_the_space_and_updates_the_water() {
        let change = switch_world_space(0, 0, 0, 0, true, 0);
        assert_eq!(change.e.get(change.tes, TES::pWorldSpace), change.new_space);
        let log = &change.log;
        // No unloading, no extra data.
        assert!(calls_to(log, 0x0045_5200).is_empty());
        assert!(calls_to(log, 0x0052_6270).is_empty());
        // The sky/interface calls take the new world space.
        assert_eq!(calls_to(log, 0x0058_eeb0), vec![vec![0x3333]]);
        assert_eq!(
            calls_to(log, 0x0070_9d20),
            vec![vec![change.new_space.addr()]]
        );
        // The flag byte is 0: the water gets no texture.
        assert_eq!(calls_to(log, 0x0052_d3b0), vec![vec![0x7a7a, 0]]);
        assert!(calls_to(log, 0x0045_68c0).is_empty());
        // `005862e0` true: the final byte is 0.
        assert_eq!(change.e.global::<u8>(WORLD_SPACE_FLAG), 0);
    }

    #[test]
    fn switching_runs_the_extra_data_and_stores_one_when_the_space_is_unknown() {
        let change = switch_world_space(0, 0, 0xe0e0, 0, false, 0);
        assert_eq!(calls_to(&change.log, 0x0052_6270), vec![vec![0xe0e0]]);
        assert_eq!(change.e.global::<u8>(WORLD_SPACE_FLAG), 1);
    }

    #[test]
    fn switching_with_unloading_unloads_the_old_space_and_cleans_up() {
        let change = switch_world_space(1, 0, 0, 0, true, 5);
        let log = &change.log;
        assert_eq!(
            calls_to(log, 0x0045_5200),
            vec![vec![change.tes.addr(), 1, 0x1111]]
        );
        assert_eq!(calls_to(log, 0x0045_4d50), vec![vec![change.tes.addr(), 1]]);
        assert_eq!(calls_to(log, 0x0045_2490), vec![vec![change.tes.addr(), 0]]);
        // No interior cell: `00453940` runs only once, after the switch.
        assert_eq!(calls_to(log, 0x0045_3940).len(), 1);
    }

    #[test]
    fn switching_from_an_interior_cell_skips_the_second_unload() {
        let change = switch_world_space(1, 0x2222, 0, 0, true, 0);
        let log = &change.log;
        // `00453940` runs for the interior cell and again after the switch.
        assert_eq!(calls_to(log, 0x0045_3940).len(), 2);
        assert!(calls_to(log, 0x0045_4d50).is_empty());
        // Nothing was unloaded: no texture clean-up.
        assert!(calls_to(log, 0x0045_2490).is_empty());
    }

    #[test]
    fn switching_loads_the_texture_when_the_flag_byte_is_set() {
        let change = switch_world_space(0, 0, 0, 1, true, 0);
        let log = &change.log;
        let guard = calls_to(log, SCOPE_GUARD_CTOR);
        assert_eq!(guard.len(), 1);
        assert_eq!(&guard[0][1..], &[0x1d, 1, TES_CPP_PATH, 0x1751]);
        let texture = calls_to(log, 0x0045_68c0);
        assert_eq!(texture.len(), 1);
        assert_eq!(texture[0][0], 0x0bee_0000);
        assert_eq!(&texture[0][3..], &[1, 1]);
        // The water system gets whatever the `NiPointer` holds afterwards
        // (null here: the texture loader is a double).
        assert_eq!(calls_to(log, 0x0052_d3b0), vec![vec![0x7a7a, 0]]);
        assert_eq!(calls_to(log, SCOPE_GUARD_DTOR).len(), 1);
    }

    #[test]
    fn world_space_flag_byte_is_stored() {
        let mut e = Engine::new();
        map_globals(&mut e);
        e.call(0x0045_83f0, &args![1u8]);
        assert_eq!(e.global::<u8>(WORLD_SPACE_FLAG), 1);
        e.call(0x0045_83f0, &args![0u8]);
        assert_eq!(e.global::<u8>(WORLD_SPACE_FLAG), 0);
    }

    #[test]
    fn world_space_extra_word_is_read_at_0xd0() {
        let mut e = Engine::new();
        let space = Ptr::<()>::new(e.mem.alloc(0x100));
        e.mem.set_u32(space.addr() + 0xd0, 0xabcd);
        assert_eq!(e.call(0x0045_8400, &args![space]).u32(), 0xabcd);
    }

    // ----- picks -----

    #[test]
    fn pick_defaults_to_the_cell_pick() {
        let mut e = Engine::new();
        let tes: Ptr<TES> = e.new_object();
        returns(&mut e, GET_INTERIOR_CELL, 0x2222);
        let picked = recording(&mut e, CELL_PICK, 0x77);
        assert_eq!(e.call(0x0045_8420, &args![tes, 0x4444u32]).u32(), 0x77);
        assert_eq!(picked.borrow()[0], vec![0x2222, 0x4444]);
    }

    #[test]
    fn cell_pick_uses_the_centre_cell_without_an_interior_cell() {
        let mut e = Engine::new();
        map_globals(&mut e);
        put_setting(&mut e, SETTING_GRIDS_TO_LOAD, 5);
        let tes: Ptr<TES> = e.new_object();
        e.set(tes, TES::pGridCellA, Ptr::new(0x0bad_0000));
        returns(&mut e, GET_INTERIOR_CELL, 0);
        let slot = e.mem.alloc(8);
        e.mem.set_u32(slot, 0x3333);
        let looked_up = recording(&mut e, GRID_CELL_ARRAY_GET, slot);
        let picked = recording(&mut e, CELL_PICK, 0x77);
        assert_eq!(e.call(0x0045_8440, &args![tes, 0x4444u32, 1u8]).u32(), 0x77);
        // uGridsToLoad / 2 in both coordinates.
        assert_eq!(looked_up.borrow()[0], vec![0x0bad_0000, 2, 2]);
        assert_eq!(picked.borrow()[0], vec![0x3333, 0x4444]);
        // A null cell gives 0 without picking.
        e.mem.set_u32(slot, 0);
        assert_eq!(e.call(0x0045_8440, &args![tes, 0x4444u32, 1u8]).u32(), 0);
        assert_eq!(picked.borrow().len(), 1);
    }

    /// The doubles for the callees of `TES::PickNI`; the pick data at the
    /// returned pointer has `from`, `to` and `length` vectors and the scale is
    /// 2.0. `hit` is what `NiPick::PickObjects` says.
    struct PickWorld {
        e: Engine,
        tes: Ptr<TES>,
        pick: Ptr,
        picked: Log,
        calls: Log,
    }

    fn pick_world(from: [f32; 4], to: [f32; 4], length: [f32; 4], hit: bool) -> PickWorld {
        let mut e = vector_engine();
        let tes: Ptr<TES> = e.new_object();
        let pick = Ptr::<()>::new(e.mem.alloc(0xb0));
        for (lane, value) in from.iter().enumerate() {
            e.mem.set_f32(pick.addr() + 4 * lane as u32, *value);
        }
        for (lane, value) in to.iter().enumerate() {
            e.mem.set_f32(pick.addr() + 0x10 + 4 * lane as u32, *value);
        }
        for (lane, value) in length.iter().enumerate() {
            e.mem.set_f32(pick.addr() + 0x90 + 4 * lane as u32, *value);
        }
        // `end - origin`, as three floats, in the out parameter.
        e.register(0x0043_9ef0, |e, a| {
            for lane in 0..3 {
                let value = e.mem.f32(a[0] + 4 * lane) - e.mem.f32(a[2] + 4 * lane);
                e.mem.set_f32(a[1] + 4 * lane, value);
            }
            ret(a[1])
        });
        let calls: Log = Rc::new(RefCell::new(vec![]));
        let picked: Log = Rc::new(RefCell::new(vec![]));
        let inner = calls.clone();
        e.register_double(0x00e9_8f20, move |_, a| {
            inner.borrow_mut().push(a.to_vec());
            ret(a[0])
        });
        returns(&mut e, 0x0084_e3a0, 0x1234);
        noop(&mut e, &[0x0070_5fc0, 0x0045_8b30, 0x00e9_8fa0]);
        let seen = picked.clone();
        e.register_double(0x00e9_8e20, move |e, a| {
            let mut words = vec![a[0]];
            for lane in 0..3 {
                words.push(e.mem.f32(a[1] + 4 * lane).to_bits());
            }
            for lane in 0..3 {
                words.push(e.mem.f32(a[2] + 4 * lane).to_bits());
            }
            words.push(a[3]);
            seen.borrow_mut().push(words);
            ret(hit as u32)
        });
        returns(&mut e, 0x0050_0940, 0x6000);
        returns(&mut e, 0x0096_8670, 0x6100);
        returns(&mut e, 0x0045_8b50, 0x6200);
        PickWorld {
            e,
            tes,
            pick,
            picked,
            calls,
        }
    }

    fn floats(words: &[u32]) -> Vec<f32> {
        words.iter().map(|word| f32::from_bits(*word)).collect()
    }

    #[test]
    fn ni_pick_casts_the_scaled_ray_and_returns_the_hit_object() {
        let mut world = pick_world([1.0, 2.0, 3.0, 0.0], [4.0, 6.0, 8.0, 0.0], [0.0; 4], true);
        let hit = world.e.call(0x0045_84d0, &args![world.tes, world.pick]);
        assert_eq!(hit.u32(), 0x6200);
        // Origin (1,2,3) * 2 and direction (end - origin) = (8,12,16) - (2,4,6).
        let picked = world.picked.borrow();
        assert_eq!(picked.len(), 1);
        assert_eq!(floats(&picked[0][1..4]), vec![2.0, 4.0, 6.0]);
        assert_eq!(floats(&picked[0][4..7]), vec![6.0, 8.0, 10.0]);
        assert_eq!(picked[0][7], 0);
        // The NiPick is built with (0, 8).
        assert_eq!(&world.calls.borrow()[0][1..], &[0, 8]);
    }

    #[test]
    fn ni_pick_completes_the_end_from_the_length_first() {
        let mut world = pick_world(
            [1.0, 2.0, 3.0, 0.0],
            [100.0, 100.0, 100.0, 0.0],
            [1.0, 1.0, 1.0, 0.0],
            true,
        );
        world.e.call(0x0045_84d0, &args![world.tes, world.pick]);
        // `m_to` became `m_from + hkLength` = (2, 3, 4): end (4, 6, 8) scaled;
        // the direction is (4,6,8) - (2,4,6).
        let picked = world.picked.borrow();
        assert_eq!(floats(&picked[0][4..7]), vec![2.0, 2.0, 2.0]);
    }

    #[test]
    fn ni_pick_without_a_hit_returns_zero_and_still_drops_the_pick() {
        let mut world = pick_world([0.0; 4], [1.0, 1.0, 1.0, 0.0], [0.0; 4], false);
        let dropped = recording(&mut world.e, 0x00e9_8fa0, 0);
        let hit = world.e.call(0x0045_84d0, &args![world.tes, world.pick]);
        assert_eq!(hit.u32(), 0);
        assert_eq!(dropped.borrow().len(), 1);
    }

    #[test]
    fn pick_without_the_cell_flag_runs_the_ni_pick() {
        let mut world = pick_world([0.0; 4], [1.0, 1.0, 1.0, 0.0], [0.0; 4], true);
        let hit = world
            .e
            .call(0x0045_8440, &args![world.tes, world.pick, 0u8]);
        assert_eq!(hit.u32(), 0x6200);
        assert_eq!(world.picked.borrow().len(), 1);
    }

    // ----- the vector helpers -----

    #[test]
    fn first_lane_is_read_as_a_float() {
        let mut e = vector_engine();
        let v = vector(&mut e, [1.5, 2.0, 3.0, 4.0]);
        assert_eq!(e.call(0x0045_86d0, &args![v]).f32(), 1.5);
    }

    #[test]
    fn scale_multiplies_by_the_global() {
        let mut e = vector_engine();
        assert_eq!(e.call(0x0045_87d0, &args![1.5f32]).f32(), 3.0);
    }

    #[test]
    fn lane_broadcast_copies_the_vector_or_one_lane() {
        let mut e = vector_engine();
        let v = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let out = Ptr::<()>::new(e.mem.alloc(16));
        let back = e.call(0x0045_8700, &args![v, out, 0u32]).ptr::<()>();
        assert_eq!(back, out);
        assert_eq!(lanes(&e, out), [1.0, 2.0, 3.0, 4.0]);
        e.call(0x0045_8700, &args![v, out, 1u32]);
        assert_eq!(lanes(&e, out), [2.0; 4]);
        e.call(0x0045_8700, &args![v, out, 2u32]);
        assert_eq!(lanes(&e, out), [3.0; 4]);
        e.call(0x0045_8700, &args![v, out, 3u32]);
        assert_eq!(lanes(&e, out), [4.0; 4]);
        e.call(0x0045_8700, &args![v, out, 9u32]);
        assert_eq!(lanes(&e, out), [4.0; 4]);
    }

    #[test]
    fn lane_broadcast_of_a_negative_index_gives_lane_3() {
        // 00458723..00458735 test only for exact 0, 1 and 2; a negative index
        // falls through to `SHUFPS 0xff` (00458792) like any other value.
        let mut e = vector_engine();
        let v = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
        for index in [0xffff_ffffu32, 0xffff_fffe, 0x8000_0000] {
            let out = Ptr::<()>::new(e.mem.alloc(16));
            let back = e.call(0x0045_8700, &args![v, out, index]).ptr::<()>();
            assert_eq!(back, out);
            assert_eq!(lanes(&e, out), [4.0; 4]);
        }
    }

    #[test]
    fn three_lanes_are_scaled_into_an_out_vector() {
        let mut e = vector_engine();
        let v = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let out = Ptr::<()>::new(e.mem.alloc(16));
        let back = e.call(0x0045_8620, &args![out, v]).ptr::<()>();
        assert_eq!(back, out);
        assert_eq!(lanes(&e, out), [2.0, 4.0, 6.0, 0.0]);
    }

    #[test]
    fn pick_vectors_are_scaled_from_from_and_to() {
        let mut e = vector_engine();
        let pick = Ptr::<()>::new(e.mem.alloc(0xb0));
        for (lane, value) in [1.0f32, 2.0, 3.0, 0.0].iter().enumerate() {
            e.mem.set_f32(pick.addr() + 4 * lane as u32, *value);
        }
        for (lane, value) in [5.0f32, 6.0, 7.0, 0.0].iter().enumerate() {
            e.mem.set_f32(pick.addr() + 0x10 + 4 * lane as u32, *value);
        }
        let from = Ptr::<()>::new(e.mem.alloc(16));
        let to = Ptr::<()>::new(e.mem.alloc(16));
        e.call(0x0045_85f0, &args![pick, from]);
        e.call(0x0045_87f0, &args![pick, to]);
        assert_eq!(lanes(&e, from), [2.0, 4.0, 6.0, 0.0]);
        assert_eq!(lanes(&e, to), [10.0, 12.0, 14.0, 0.0]);
    }

    #[test]
    fn to_vector_address_is_returned_after_calc_to() {
        let mut e = vector_engine();
        let pick = Ptr::<()>::new(e.mem.alloc(0xb0));
        let to = e.call(0x0045_8820, &args![pick]).u32();
        assert_eq!(to, pick.addr() + 0x10);
    }

    #[test]
    fn calc_to_adds_a_non_zero_length_to_the_start() {
        let mut e = vector_engine();
        let pick = Ptr::<()>::new(e.mem.alloc(0xb0));
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].iter().enumerate() {
            e.mem.set_f32(pick.addr() + 4 * lane as u32, *value);
            e.mem.set_f32(pick.addr() + 0x10 + 4 * lane as u32, 9.0);
        }
        // A zero length leaves `m_to` alone.
        e.call(0x0045_8840, &args![pick]);
        assert_eq!(lanes(&e, pick.byte_add(0x10)), [9.0; 4]);
        for (lane, value) in [0.5f32, 0.0, 0.0, 0.25].iter().enumerate() {
            e.mem.set_f32(pick.addr() + 0x90 + 4 * lane as u32, *value);
        }
        e.call(0x0045_8840, &args![pick]);
        assert_eq!(lanes(&e, pick.byte_add(0x10)), [1.5, 2.0, 3.0, 4.25]);
    }

    #[test]
    fn vectors_are_added_lane_by_lane() {
        let mut e = vector_engine();
        let a = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let b = vector(&mut e, [10.0, 20.0, 30.0, 40.0]);
        let out = Ptr::<()>::new(e.mem.alloc(16));
        e.call(0x0045_8880, &args![out, a, b]);
        assert_eq!(lanes(&e, out), [11.0, 22.0, 33.0, 44.0]);
    }

    #[test]
    fn vectors_are_subtracted_lane_by_lane() {
        let mut e = vector_engine();
        let a = vector(&mut e, [10.0, 20.0, 30.0, 40.0]);
        let b = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let out = Ptr::<()>::new(e.mem.alloc(16));
        e.call(0x0045_8a10, &args![out, a, b]);
        assert_eq!(lanes(&e, out), [9.0, 18.0, 27.0, 36.0]);
    }

    #[test]
    fn absolute_value_clears_the_sign_bits() {
        let mut e = vector_engine();
        let a = vector(&mut e, [-1.0, 2.0, -0.0, -4.5]);
        let out = Ptr::<()>::new(e.mem.alloc(16));
        e.call(0x0045_8ad0, &args![out, a]);
        assert_eq!(lanes(&e, out), [1.0, 2.0, 0.0, 4.5]);
        assert_eq!(e.mem.u32(out.addr() + 8), 0);
    }

    #[test]
    fn broadcast_stores_the_value_in_every_lane() {
        let mut e = vector_engine();
        let out = Ptr::<()>::new(e.mem.alloc(16));
        e.call(0x0045_89c0, &args![out, 0.25f32]);
        assert_eq!(lanes(&e, out), [0.25; 4]);
    }

    #[test]
    fn less_or_equal_gives_all_ones_masks_and_false_for_nan() {
        let mut e = vector_engine();
        let a = vector(&mut e, [1.0, 2.0, f32::NAN, 5.0]);
        let b = vector(&mut e, [1.0, 1.0, 3.0, 6.0]);
        let out = Ptr::<()>::new(e.mem.alloc(16));
        let back = e.call(0x0045_8a60, &args![a, out, b]).ptr::<()>();
        assert_eq!(back, out);
        let words: Vec<u32> = (0..4)
            .map(|lane| e.mem.u32(out.addr() + 4 * lane))
            .collect();
        assert_eq!(words, vec![u32::MAX, 0, 0, u32::MAX]);
    }

    #[test]
    fn sign_mask_test_needs_all_requested_bits() {
        let mut e = vector_engine();
        let v = Ptr::<()>::new(e.mem.alloc(16));
        for (lane, word) in [u32::MAX, 0, u32::MAX, 0].iter().enumerate() {
            e.mem.set_u32(v.addr() + 4 * lane as u32, *word);
        }
        assert!(e.call(0x0045_8990, &args![v, 5u32]).bool());
        assert!(!e.call(0x0045_8990, &args![v, 7u32]).bool());
        assert!(e.call(0x0045_8990, &args![v, 0u32]).bool());
        // Only the sign bit counts.
        e.mem.set_u32(v.addr() + 4, 0x7fff_ffff);
        assert!(!e.call(0x0045_8990, &args![v, 2u32]).bool());
    }

    #[test]
    fn closeness_test_compares_x_y_z_but_not_w() {
        let mut e = vector_engine();
        let a = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let near = vector(&mut e, [1.05, 1.95, 3.0, 400.0]);
        assert!(e.call(0x0045_8900, &args![a, near, 0.1f32]).bool());
        assert!(!e.call(0x0045_8900, &args![a, near, 0.01f32]).bool());
        let nan = vector(&mut e, [f32::NAN, 2.0, 3.0, 4.0]);
        assert!(!e.call(0x0045_8900, &args![a, nan, 1.0f32]).bool());
    }

    #[test]
    fn length_is_empty_only_within_the_epsilon_of_zero() {
        let mut e = vector_engine();
        let pick = Ptr::<()>::new(e.mem.alloc(0xb0));
        // All zero: not "non-empty".
        assert!(!e.call(0x0045_88d0, &args![pick]).bool());
        e.mem.set_f32(pick.addr() + 0x90 + 4, 0.0005);
        assert!(!e.call(0x0045_88d0, &args![pick]).bool());
        e.mem.set_f32(pick.addr() + 0x90 + 8, -0.5);
        assert!(e.call(0x0045_88d0, &args![pick]).bool());
        // The w lane is ignored.
        e.mem.set_f32(pick.addr() + 0x90 + 8, 0.0);
        e.mem.set_f32(pick.addr() + 0x90 + 12, 50.0);
        assert!(!e.call(0x0045_88d0, &args![pick]).bool());
    }

    // ===== Session 2: `00458b20` to `0045a730` =====

    /// The pages the second session's code reads, on top of [`map_globals`].
    fn map_more_globals(e: &mut Engine) {
        map_globals(e);
        for page in [
            0x011c_7000u32,
            0x011c_a000,
            0x011c_e000,
            0x011c_f000,
            0x011d_0000,
            0x011d_1000,
            0x011d_d000,
            0x011e_0000,
            0x0120_2000,
        ] {
            e.map(page, 0x1000);
        }
    }

    /// Item and next accessors for `BSSimpleList` nodes: the item word at
    /// +0, the next node at +4.
    fn list_doubles(e: &mut Engine) {
        e.register(LIST_NODE_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
    }

    /// Fills the list whose first (inline) node is at `head` with `items`.
    fn fill_list(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if index + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    /// An 8-byte list entry: form pointer, 16-bit count.
    fn dead_entry(e: &mut Engine, form: u32, count: u16) -> u32 {
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, form);
        e.mem.set_u16(entry + 4, count);
        entry
    }

    /// A cell for the priority code: interior flag, data x, data y.
    fn cell_object(e: &mut Engine, interior: bool, x: i32, y: i32) -> Ptr {
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(cell, interior as u32);
        e.mem.set_i32(cell + 4, x);
        e.mem.set_i32(cell + 8, y);
        Ptr::new(cell)
    }

    // ----- 00458b20, 00458b30, 00458b50 -----

    #[test]
    fn zero_vector_address_is_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0045_8b20, &args![]).u32(), 0x0126_7e30);
    }

    #[test]
    fn byte_setter_stores_at_0x11() {
        let mut e = Engine::new();
        let object = Ptr::<()>::new(e.mem.alloc(0x20));
        e.call(0x0045_8b30, &args![object, 7u8]);
        assert_eq!(e.mem.u8(object.addr() + 0x11), 7);
    }

    #[test]
    fn pointer_getter_forwards_to_the_ni_pointer_getter() {
        let mut e = Engine::new();
        ni_pointer_doubles(&mut e);
        let cell = e.mem.alloc(8);
        e.mem.set_u32(cell, 0x1234);
        assert_eq!(
            e.call(0x0045_8b50, &args![Ptr::<()>::new(cell)]).u32(),
            0x1234
        );
    }

    // ----- 00458b70 -----

    #[test]
    fn map_name_asks_the_given_world_space_through_slot_0x138() {
        let mut e = Engine::new();
        let table = vtable(&mut e, &[(0x138, 0x7000_0138)]);
        let world_space = object_with(&mut e, table);
        let asked = recording(&mut e, 0x7000_0138, 0xabcd);
        let current = recording(&mut e, GET_WORLD_SPACE, 0);
        let tes = e.new_object::<TES>();
        let name = e
            .call(
                0x0045_8b70,
                &args![tes, 1u32, 2u32, 3u32, 4u32, world_space],
            )
            .u32();
        assert_eq!(name, 0xabcd);
        assert_eq!(*asked.borrow(), vec![vec![world_space.addr(), 1, 2, 3, 4]]);
        assert!(current.borrow().is_empty());
    }

    #[test]
    fn map_name_falls_back_to_the_current_world_space() {
        let mut e = Engine::new();
        let table = vtable(&mut e, &[(0x138, 0x7000_0138)]);
        let world_space = object_with(&mut e, table);
        let asked = recording(&mut e, 0x7000_0138, 0x77);
        let current = recording(&mut e, GET_WORLD_SPACE, world_space.addr());
        let tes = e.new_object::<TES>();
        let name = e
            .call(
                0x0045_8b70,
                &args![tes, 9u32, 8u32, 7u32, 6u32, Ptr::<()>::NULL],
            )
            .u32();
        assert_eq!(name, 0x77);
        assert_eq!(*current.borrow(), vec![vec![tes.addr()]]);
        assert_eq!(asked.borrow()[0][1..], [9, 8, 7, 6]);
    }

    #[test]
    fn map_name_is_zero_without_any_world_space() {
        let mut e = Engine::new();
        returns(&mut e, GET_WORLD_SPACE, 0);
        let tes = e.new_object::<TES>();
        let name = e
            .call(
                0x0045_8b70,
                &args![tes, 1u32, 2u32, 3u32, 4u32, Ptr::<()>::NULL],
            )
            .u32();
        assert_eq!(name, 0);
    }

    // ----- 00458be0 -----

    fn priority_engine(grids: u32) -> Engine {
        let mut e = Engine::new();
        map_more_globals(&mut e);
        put_setting(&mut e, SETTING_GRIDS_TO_LOAD, grids);
        e.register(CELL_IS_INTERIOR, |e, a| ret(e.mem.u32(a[0])));
        e.register(CELL_GET_DATA_X, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(CELL_GET_DATA_Y, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(FLOAT_TO_INT_ROUNDED, |_, a| {
            ret(f32::from_bits(a[0]).round() as i32 as u32)
        });
        e.register(ABSOLUTE_VALUE, |_, a| ret((a[0] as i32).unsigned_abs()));
        e
    }

    #[test]
    fn cell_priority_is_cached_for_the_last_cell() {
        let mut e = priority_engine(5);
        let asked = recording(&mut e, CELL_IS_INTERIOR, 0);
        let tes = e.new_object::<TES>();
        e.set_global(CELL_PRIORITY_LAST_CELL, 0x4444u32);
        e.set_global(CELL_PRIORITY_VALUE, 3u32);
        let priority = e.call(0x0045_8be0, &args![tes, 0x4444u32, Ptr::<()>::NULL]);
        assert_eq!(priority.u32(), 3);
        assert!(asked.borrow().is_empty());
    }

    #[test]
    fn interior_cells_are_always_near() {
        let mut e = priority_engine(5);
        let tes = e.new_object::<TES>();
        let cell = cell_object(&mut e, true, 500, -500);
        let priority = e.call(0x0045_8be0, &args![tes, cell, Ptr::<()>::NULL]);
        assert_eq!(priority.u32(), 1);
        assert_eq!(e.global::<u32>(CELL_PRIORITY_LAST_CELL), cell.addr());
        assert_eq!(e.global::<u32>(CELL_PRIORITY_VALUE), 1);
    }

    #[test]
    fn exterior_priority_follows_the_distance_from_the_grid_centre() {
        let mut e = priority_engine(5);
        let tes = e.new_object::<TES>();
        e.set(tes, TES::iCurrentGridX, 100);
        e.set(tes, TES::iCurrentGridY, 100);
        // grids/2 = 2: a cell at distance 1 is still near (10 + 10 = 20), at
        // distance 2 it is far (30). The game takes the larger of the two
        // signed offsets and then its absolute value, so a cell far away
        // on the negative side of one axis only is still near, and one far
        // on the negative side of both is far.
        for (x, y, expected) in [
            (100, 100, 1),
            (101, 100, 1),
            (100, 99, 1),
            (102, 100, 3),
            (100, 102, 3),
            (104, 100, 3),
            (96, 100, 1),
            (96, 96, 3),
        ] {
            let cell = cell_object(&mut e, false, x, y);
            let priority = e.call(0x0045_8be0, &args![tes, cell, Ptr::<()>::NULL]);
            assert_eq!(priority.u32(), expected, "cell ({x}, {y})");
        }
    }

    #[test]
    fn exterior_priority_with_a_position_uses_its_cell() {
        let mut e = priority_engine(5);
        let tes = e.new_object::<TES>();
        let position = Ptr::<()>::new(e.mem.alloc(16));
        e.mem.set_f32(position.addr(), 100.0 * 4096.0);
        e.mem.set_f32(position.addr() + 4, 100.0 * 4096.0);
        let near = cell_object(&mut e, false, 100, 101);
        let far = cell_object(&mut e, false, 106, 100);
        assert_eq!(e.call(0x0045_8be0, &args![tes, near, position]).u32(), 1);
        assert_eq!(e.call(0x0045_8be0, &args![tes, far, position]).u32(), 3);
    }

    // ----- 00458e20, 00458ef0, 00458f20, 00458f30, 00458f70 -----

    #[test]
    fn temp_debug_object_is_built_and_attached() {
        let mut e = Engine::new();
        map_more_globals(&mut e);
        let temp_memory = e.mem.alloc(0xb0);
        returns(&mut e, NI_OPERATOR_NEW, temp_memory);
        let constructed = recording(&mut e, NI_NODE_CTOR, 0);
        e.mem.set_u32(BS_TEMP_NODE_VTABLE + 0xdc, 0x7000_00dc);
        let temp_attach = recording(&mut e, 0x7000_00dc, 0);
        let parent_table = vtable(&mut e, &[(0xdc, 0x7000_01dc)]);
        let parent = object_with(&mut e, parent_table);
        returns(&mut e, TES_DEBUG_PARENT, parent.addr());
        let parent_attach = recording(&mut e, 0x7000_01dc, 0);
        let updated = recording(&mut e, NI_AV_OBJECT_UPDATE_PROPERTIES, 0);
        let tes = e.new_object::<TES>();
        let node = Ptr::<()>::new(0x6000_0000);
        e.call(0x0045_8e20, &args![tes, node, 1.5f32]);
        assert_eq!(*constructed.borrow(), vec![vec![temp_memory, 0]]);
        assert_eq!(e.mem.u32(temp_memory), BS_TEMP_NODE_VTABLE);
        assert_eq!(e.mem.f32(temp_memory + 0xac), 1.5);
        assert_eq!(
            *temp_attach.borrow(),
            vec![vec![temp_memory, node.addr(), 1]]
        );
        assert_eq!(
            *parent_attach.borrow(),
            vec![vec![parent.addr(), temp_memory, 1]]
        );
        assert_eq!(*updated.borrow(), vec![vec![temp_memory]]);
    }

    #[test]
    fn temp_debug_object_does_nothing_for_a_null_node() {
        let mut e = Engine::new();
        let allocated = recording(&mut e, NI_OPERATOR_NEW, 0);
        let tes = e.new_object::<TES>();
        e.call(0x0045_8e20, &args![tes, Ptr::<()>::NULL, 1.5f32]);
        assert!(allocated.borrow().is_empty());
    }

    #[test]
    fn temp_node_constructor_sets_vtable_and_float() {
        let mut e = Engine::new();
        map_more_globals(&mut e);
        let constructed = recording(&mut e, NI_NODE_CTOR, 0);
        let node = Ptr::<()>::new(e.mem.alloc(0xb0));
        let back = e.call(0x0045_8ef0, &args![node, 2.25f32]).ptr::<()>();
        assert_eq!(back, node);
        assert_eq!(*constructed.borrow(), vec![vec![node.addr(), 0]]);
        assert_eq!(e.mem.u32(node.addr()), BS_TEMP_NODE_VTABLE);
        assert_eq!(e.mem.f32(node.addr() + 0xac), 2.25);
    }

    #[test]
    fn temp_node_rtti_is_the_constant() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x0045_8f20, &args![0x1000u32]).u32(),
            BS_TEMP_NODE_RTTI
        );
    }

    #[test]
    fn temp_node_deleting_destructor_frees_only_when_asked() {
        let mut e = Engine::new();
        map_more_globals(&mut e);
        let node_dtor = recording(&mut e, NI_NODE_DTOR, 0);
        let freed = recording(&mut e, NI_OPERATOR_DELETE, 0);
        let node = Ptr::<()>::new(e.mem.alloc(0xb0));
        let back = e.call(0x0045_8f30, &args![node, 0u32]).ptr::<()>();
        assert_eq!(back, node);
        assert!(freed.borrow().is_empty());
        e.call(0x0045_8f30, &args![node, 1u32]);
        assert_eq!(*freed.borrow(), vec![vec![node.addr(), 0xb0]]);
        assert_eq!(node_dtor.borrow().len(), 2);
    }

    #[test]
    fn temp_node_destructor_restores_the_vtable_and_runs_the_base() {
        let mut e = Engine::new();
        map_more_globals(&mut e);
        let node_dtor = recording(&mut e, NI_NODE_DTOR, 0);
        let node = Ptr::<()>::new(e.mem.alloc(0xb0));
        e.mem.set_u32(node.addr(), 0x1111);
        e.call(0x0045_8f70, &args![node]);
        assert_eq!(e.mem.u32(node.addr()), BS_TEMP_NODE_VTABLE);
        assert_eq!(*node_dtor.borrow(), vec![vec![node.addr()]]);
    }

    // ----- 00458f90, 00459000, 00459060, 00459840 -----

    #[test]
    fn dead_count_list_is_freed_entry_by_entry_and_cleared() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let tes = e.new_object::<TES>();
        let first = dead_entry(&mut e, 0x10, 1);
        let second = dead_entry(&mut e, 0x20, 2);
        fill_list(&mut e, tes.addr() + 0x9c, &[first, second]);
        let freed = recording(&mut e, OPERATOR_DELETE, 0);
        let cleared = recording(&mut e, 0x0047_0470, 0);
        e.call(0x0045_8f90, &args![tes]);
        assert_eq!(*freed.borrow(), vec![vec![first], vec![second]]);
        assert_eq!(*cleared.borrow(), vec![vec![tes.addr() + 0x9c]]);
    }

    #[test]
    fn dead_count_list_walk_stops_at_a_null_item() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let tes = e.new_object::<TES>();
        let first = dead_entry(&mut e, 0x10, 1);
        let later = dead_entry(&mut e, 0x20, 2);
        fill_list(&mut e, tes.addr() + 0x9c, &[first, 0, later]);
        let freed = recording(&mut e, OPERATOR_DELETE, 0);
        let cleared = recording(&mut e, 0x0047_0470, 0);
        e.call(0x0045_8f90, &args![tes]);
        assert_eq!(*freed.borrow(), vec![vec![first]]);
        assert_eq!(cleared.borrow().len(), 1);
    }

    #[test]
    fn dead_count_lookup_finds_the_form_or_gives_zero() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let tes = e.new_object::<TES>();
        let first = dead_entry(&mut e, 0x10, 4);
        let second = dead_entry(&mut e, 0x20, 9);
        fill_list(&mut e, tes.addr() + 0x9c, &[first, second]);
        assert_eq!(e.call(0x0045_9000, &args![tes, 0x20u32]).u16(), 9);
        assert_eq!(e.call(0x0045_9000, &args![tes, 0x10u32]).u16(), 4);
        assert_eq!(e.call(0x0045_9000, &args![tes, 0x30u32]).u16(), 0);
    }

    #[test]
    fn dead_count_add_updates_an_existing_entry() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let tes = e.new_object::<TES>();
        let entry = dead_entry(&mut e, 0x10, 4);
        fill_list(&mut e, tes.addr() + 0x9c, &[entry]);
        let added = recording(&mut e, LIST_ADD_HEAD, 0);
        e.call(0x0045_9060, &args![tes, 0x10u32, -6i16]);
        assert_eq!(e.mem.i16(entry + 4), -2);
        assert!(added.borrow().is_empty());
    }

    #[test]
    fn dead_count_add_puts_a_new_entry_at_the_head() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let tes = e.new_object::<TES>();
        let entry = dead_entry(&mut e, 0x10, 4);
        fill_list(&mut e, tes.addr() + 0x9c, &[entry]);
        let fresh = e.mem.alloc(8);
        let requested = recording(&mut e, OPERATOR_NEW, fresh);
        let seen = Rc::new(RefCell::new(vec![]));
        let inner = seen.clone();
        e.register_double(LIST_ADD_HEAD, move |e, a| {
            inner.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call(0x0045_9060, &args![tes, 0x99u32, 3i16]);
        assert_eq!(*requested.borrow(), vec![vec![8]]);
        assert_eq!(e.mem.u32(fresh), 0x99);
        assert_eq!(e.mem.u16(fresh + 4), 3);
        assert_eq!(*seen.borrow(), vec![(tes.addr() + 0x9c, fresh)]);
        // The existing entry is untouched.
        assert_eq!(e.mem.u16(entry + 4), 4);
    }

    #[test]
    fn save_reset_clears_the_list_and_the_save_grid() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let tes = e.new_object::<TES>();
        let entry = dead_entry(&mut e, 0x10, 4);
        fill_list(&mut e, tes.addr() + 0x9c, &[entry]);
        noop(&mut e, &[OPERATOR_DELETE]);
        let cleared = recording(&mut e, 0x0047_0470, 0);
        e.call(0x0045_9840, &args![tes]);
        assert_eq!(cleared.borrow().len(), 1);
        assert_eq!(e.get(tes, TES::iSaveGridX), 0x7fff_ffff);
        assert_eq!(e.get(tes, TES::iSaveGridY), 0x7fff_ffff);
    }

    // ----- 00459100 -----

    /// An engine for the save-size and save code: the save object global,
    /// the block and version answers, the list helpers and the log byte.
    fn save_engine(blocks: bool, version: u8, log: bool) -> Engine {
        let mut e = Engine::new();
        map_more_globals(&mut e);
        list_doubles(&mut e);
        e.set_global(SAVE_LOAD_GAME, 0x7100_0000u32);
        returns(&mut e, USE_SAVE_GAME_BLOCKS, blocks as u32);
        returns(&mut e, SAVE_VERSION, version as u32);
        e.register(SAVE_LIST_HEAD, |_, a| ret(a[0] + 0x9c));
        e.register(LIST_COUNT, |e, a| {
            let mut node = a[0];
            let mut count = 0;
            while node != 0 {
                if e.mem.u32(node) != 0 {
                    count += 1;
                }
                node = e.mem.u32(node + 4);
            }
            ret(count)
        });
        put_setting(&mut e, SETTING_SAVE_SIZE_LOG, log as u32);
        e
    }

    #[test]
    fn save_size_counts_blocks_entries_and_version() {
        let mut e = save_engine(true, 0x32, false);
        let tes = e.new_object::<TES>();
        let first = dead_entry(&mut e, 0x10, 1);
        let second = dead_entry(&mut e, 0x20, 2);
        fill_list(&mut e, tes.addr() + 0x9c, &[first, second]);
        let logged = recording(&mut e, ERROR_LOG, 0);
        // 6 (block tag and size) + 4 (count) + 2 * 6 + 4 (version).
        assert_eq!(e.call(0x0045_9100, &args![tes]).u16(), 26);
        assert!(logged.borrow().is_empty());
    }

    #[test]
    fn save_size_without_blocks_or_entries_is_just_the_count() {
        let mut e = save_engine(false, 0x31, false);
        let tes = e.new_object::<TES>();
        assert_eq!(e.call(0x0045_9100, &args![tes]).u16(), 4);
    }

    #[test]
    fn save_size_logs_the_size_without_a_world_space() {
        let mut e = save_engine(false, 0x31, true);
        let tes = e.new_object::<TES>();
        let logged = recording(&mut e, ERROR_LOG, 0);
        let world = recording(&mut e, GET_WORLD_SPACE, 0);
        e.call(0x0045_9100, &args![tes]);
        assert_eq!(
            *logged.borrow(),
            vec![vec![0x0101_2c78, 4, 0x1882, TES_CPP_PATH]]
        );
        assert_eq!(*world.borrow(), vec![vec![0x7100_0000]]);
    }

    #[test]
    fn save_size_logs_the_world_space_form_when_there_is_one() {
        let mut e = save_engine(false, 0x31, true);
        let tes = e.new_object::<TES>();
        let logged = recording(&mut e, ERROR_LOG, 0);
        let world_space = e.mem.alloc(0x20);
        e.mem.set_u32(world_space, 0xabc);
        e.mem.set_u32(world_space + 5, 0x1234_5678);
        returns(&mut e, GET_WORLD_SPACE, world_space);
        let table = vtable(&mut e, &[(0x130, 0x7000_0130)]);
        let form = object_with(&mut e, table);
        returns(&mut e, 0x7000_0130, 0x5555);
        let looked_up = recording(&mut e, FORM_LOOKUP, form.addr());
        e.call(0x0045_9100, &args![tes]);
        assert_eq!(*looked_up.borrow(), vec![vec![0xabc]]);
        assert_eq!(
            *logged.borrow(),
            vec![vec![
                0x0101_2cb0,
                4,
                0xabc,
                0x5555,
                0x1234_5678,
                0x1882,
                TES_CPP_PATH
            ]]
        );
    }

    // ----- 00459230 -----

    /// A save buffer in memory: the position double and the writers, which
    /// copy bytes at the cursor. A write of the extra word adds `extra`
    /// bytes to the cursor (to make a block too big).
    fn save_buffer(e: &mut Engine, extra: u32) -> u32 {
        let base = e.mem.alloc(0x200);
        let cursor = Rc::new(RefCell::new(base));
        let position = cursor.clone();
        e.register_double(SAVE_BUFFER_POSITION, move |_, _| ret(*position.borrow()));
        for function in [SAVE_BUFFER_WRITE, SAVE_NUMERIC_ID] {
            let position = cursor.clone();
            e.register_double(function, move |e, a| {
                let at = *position.borrow();
                for offset in 0..a[2] {
                    let byte = e.mem.u8(a[1] + offset);
                    e.mem.set_u8(at + offset, byte);
                }
                *position.borrow_mut() += a[2];
                if a[1] == SAVE_EXTRA_WORD {
                    *position.borrow_mut() += extra;
                }
                Ret::default()
            });
        }
        base
    }

    #[test]
    fn save_writes_a_block_with_its_size_patched_in() {
        let mut e = save_engine(true, 0x31, false);
        let base = save_buffer(&mut e, 0);
        returns(&mut e, FORM_ID_GETTER, 0xaa55);
        let tes = e.new_object::<TES>();
        let entry = dead_entry(&mut e, 0x500, 3);
        fill_list(&mut e, tes.addr() + 0x9c, &[entry]);
        e.call(0x0045_9230, &args![tes]);
        assert_eq!(e.mem.u32(base), 0x424c_4f4b);
        // 2 (size) + 4 (count) + 4 (id) + 2 (count) bytes follow the tag.
        assert_eq!(e.mem.u16(base + 4), 12);
        assert_eq!(e.mem.u32(base + 6), 1);
        assert_eq!(e.mem.u32(base + 10), 0xaa55);
        assert_eq!(e.mem.u16(base + 14), 3);
    }

    #[test]
    fn save_writes_a_zero_id_for_an_entry_without_a_form() {
        let mut e = save_engine(false, 0x31, false);
        let base = save_buffer(&mut e, 0);
        let ids = recording(&mut e, FORM_ID_GETTER, 0xaa55);
        let tes = e.new_object::<TES>();
        let entry = dead_entry(&mut e, 0, 3);
        fill_list(&mut e, tes.addr() + 0x9c, &[entry]);
        e.call(0x0045_9230, &args![tes]);
        assert!(ids.borrow().is_empty());
        // No block: the count comes first, then id 0 and the 16-bit count.
        assert_eq!(e.mem.u32(base), 1);
        assert_eq!(e.mem.u32(base + 4), 0);
        assert_eq!(e.mem.u16(base + 8), 3);
    }

    #[test]
    fn save_appends_the_extra_word_for_new_versions() {
        let mut e = save_engine(true, 0x32, false);
        let base = save_buffer(&mut e, 0);
        e.set_global(SAVE_EXTRA_WORD, 0xdead_beefu32);
        let tes = e.new_object::<TES>();
        e.call(0x0045_9230, &args![tes]);
        // Tag, size, count, then the word: 2 + 4 + 4 bytes in the block.
        assert_eq!(e.mem.u16(base + 4), 10);
        assert_eq!(e.mem.u32(base + 10), 0xdead_beef);
    }

    #[test]
    fn save_complains_when_the_block_is_bigger_than_16_bits() {
        let mut e = save_engine(true, 0x32, false);
        let base = save_buffer(&mut e, 0x1_0000);
        let printed = recording(&mut e, DEBUG_PRINT, 0);
        let tes = e.new_object::<TES>();
        e.call(0x0045_9230, &args![tes]);
        assert_eq!(
            *printed.borrow(),
            vec![vec![0x0101_5318, TES_CPP_PATH, 0x18e1]]
        );
        // 2 + 4 + 4 + 0x10000 truncated to 16 bits.
        assert_eq!(e.mem.u16(base + 4), 10);
    }

    #[test]
    fn save_logs_the_bytes_written_when_asked() {
        let mut e = save_engine(false, 0x31, true);
        save_buffer(&mut e, 0);
        returns(&mut e, GET_WORLD_SPACE, 0);
        let logged = recording(&mut e, ERROR_LOG, 0);
        let tes = e.new_object::<TES>();
        e.call(0x0045_9230, &args![tes]);
        // Only the 4-byte count was written.
        assert_eq!(
            *logged.borrow(),
            vec![vec![0x0101_536c, 4, 0x18e1, TES_CPP_PATH]]
        );
    }

    // ----- 00459470 -----

    #[test]
    fn exterior_buffer_slot_is_read_by_index() {
        let mut e = Engine::new();
        let tes = e.new_object::<TES>();
        let buffer = e.mem.alloc(16);
        e.mem.set_u32(buffer + 8, 0xc0de);
        e.set(tes, TES::pExteriorBuffer, Ptr::new(buffer));
        assert_eq!(e.call(0x0045_9470, &args![tes, 2u32]).u32(), 0xc0de);
        assert_eq!(e.call(0x0045_9470, &args![tes, 0u32]).u32(), 0);
    }

    // ----- 00459490 -----

    /// A `grids` by `grids` array of cell slots for `GridCellArray::Get`
    /// (the double answers with the address of the slot at `(x, y)`).
    fn grid_with_cells(e: &mut Engine, tes: Ptr<TES>, grids: u32, cells: &[u32]) {
        let table = e.mem.alloc(4 * grids * grids);
        for (index, cell) in cells.iter().enumerate() {
            e.mem.set_u32(table + 4 * index as u32, *cell);
        }
        let array = e.mem.alloc(0x28);
        e.set(tes, TES::pGridCellA, Ptr::new(array));
        e.register_double(GRID_CELL_ARRAY_GET, move |_, a| {
            assert_eq!(a[0], array);
            ret(table + 4 * (a[1] * grids + a[2]))
        });
    }

    #[test]
    fn save_ov2_writes_the_entries_the_grid_size_and_the_cell_ids() {
        let mut e = save_engine(false, 0x31, false);
        put_setting(&mut e, SETTING_GRIDS_TO_LOAD, 2);
        let tes = e.new_object::<TES>();
        let entry = dead_entry(&mut e, 0x700, 4);
        fill_list(&mut e, tes.addr() + 0x9c, &[entry]);
        grid_with_cells(&mut e, tes, 2, &[0, 0x1000, 0x2000, 0x3000]);
        e.set_global(DATA_HANDLER, 0x7200_0000u32);
        e.register(FORM_ID_GETTER, |_, a| ret(a[0] + 1));
        // The data handler knows every cell but the one at 0x2000.
        e.register(DATA_HANDLER_HAS_FORM, |_, a| {
            assert_eq!(a[0], 0x7200_0000);
            ret((a[1] != 0x2001) as u32)
        });
        returns(&mut e, SAVE_BUFFER_START_VARIABLE, 0x55);
        let form_pointers = recording(&mut e, SAVE_BUFFER_SAVE_FORM_ID_POINTER, 0);
        let writes = Rc::new(RefCell::new(vec![]));
        let inner = writes.clone();
        e.register_double(SAVE_BUFFER_WRITE_BYTES, move |e, a| {
            let value = if a[2] == 4 { e.mem.u32(a[1]) } else { 0 };
            inner.borrow_mut().push((a[2], a[3], value));
            Ret::default()
        });
        let ended = recording(&mut e, SAVE_BUFFER_END_VARIABLE, 0);
        let ids = recording(&mut e, SAVE_BUFFER_SAVE_FORM_ID, 0);
        let buffer = Ptr::<()>::new(0x7300_0000);
        e.call(0x0045_9490, &args![tes, buffer]);
        assert_eq!(*form_pointers.borrow(), vec![vec![buffer.addr(), 0x700, 0]]);
        assert_eq!(*writes.borrow(), vec![(2, 0, 0), (4, 0, 2)]);
        assert_eq!(*ended.borrow(), vec![vec![buffer.addr(), 1, 0x55]]);
        let saved: Vec<u32> = ids.borrow().iter().map(|call| call[1]).collect();
        assert_eq!(saved, vec![0, 0x1001, 0, 0x3001]);
    }

    // ----- 004595e0 -----

    #[test]
    fn load_reads_the_entries_then_clears_and_loads_known_cells() {
        let mut e = Engine::new();
        map_more_globals(&mut e);
        list_doubles(&mut e);
        put_setting(&mut e, SETTING_GRIDS_TO_LOAD, 1);
        e.set_global(DATA_HANDLER, 0x7200_0000u32);
        e.set_global(SCRIPT_CONTEXT, 0x7400_0000u32);
        let tes = e.new_object::<TES>();
        let table = vtable(&mut e, &[(0x128, 0x7000_0128)]);
        let cell = object_with(&mut e, table).addr();
        grid_with_cells(&mut e, tes, 1, &[cell]);
        e.register_double(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        returns(&mut e, LOAD_BUFFER_VARIABLE_SIZE, 1);
        returns(&mut e, LOAD_BUFFER_FORM_ID, 0x77);
        let looked_up = recording(&mut e, FORM_LOOKUP, 0x88);
        let cast = recording(&mut e, RT_DYNAMIC_CAST, 0x99);
        // Reads: 2 bytes are a count of 5, 4 bytes are the grid size 1.
        e.register(LOAD_BUFFER_READ_BYTES, |e, a| {
            if a[2] == 2 {
                e.mem.set_u16(a[1], 5);
            } else {
                e.mem.set_u32(a[1], 1);
            }
            Ret::default()
        });
        let seen = Rc::new(RefCell::new(vec![]));
        let inner = seen.clone();
        e.register_double(LIST_ADD_HEAD, move |e, a| {
            let entry = e.mem.u32(a[1]);
            inner
                .borrow_mut()
                .push((a[0], e.mem.u32(entry), e.mem.u16(entry + 4)));
            Ret::default()
        });
        e.register(LOAD_BUFFER_FORM_ID_OUT, |e, a| {
            e.mem.set_u32(a[1], 0x42);
            Ret::default()
        });
        e.register(FORM_ID_GETTER, |_, a| ret(a[0] + 1));
        returns(&mut e, DATA_HANDLER_HAS_FORM, 1);
        let cleared = recording(&mut e, SAVE_LOAD_CLEAR_FORM_ID, 0);
        let loaded = recording(&mut e, 0x7000_0128, 0);
        let buffer = Ptr::<()>::new(0x7300_0000);
        e.call(0x0045_95e0, &args![tes, buffer]);
        assert_eq!(*looked_up.borrow(), vec![vec![0x77]]);
        assert_eq!(
            *cast.borrow(),
            vec![vec![0x88, 0, RTTI_LOAD_SOURCE, RTTI_LOAD_TARGET, 0]]
        );
        assert_eq!(*seen.borrow(), vec![(tes.addr() + 0x9c, 0x99, 5)]);
        assert_eq!(*cleared.borrow(), vec![vec![0x7400_0000, 0x42]]);
        assert_eq!(*loaded.borrow(), vec![vec![cell, 0x42, 1]]);
    }

    #[test]
    fn load_skips_cells_with_no_id_or_that_the_data_handler_lacks() {
        let mut e = Engine::new();
        map_more_globals(&mut e);
        put_setting(&mut e, SETTING_GRIDS_TO_LOAD, 1);
        e.set_global(DATA_HANDLER, 0x7200_0000u32);
        let tes = e.new_object::<TES>();
        let table = vtable(&mut e, &[(0x128, 0x7000_0128)]);
        let cell = object_with(&mut e, table).addr();
        grid_with_cells(&mut e, tes, 1, &[cell]);
        returns(&mut e, LOAD_BUFFER_VARIABLE_SIZE, 0);
        e.register(LOAD_BUFFER_READ_BYTES, |e, a| {
            e.mem.set_u32(a[1], 1);
            Ret::default()
        });
        e.register(FORM_ID_GETTER, |_, a| ret(a[0] + 1));
        let cleared = recording(&mut e, SAVE_LOAD_CLEAR_FORM_ID, 0);
        let loaded = recording(&mut e, 0x7000_0128, 0);
        // First an id of 0, then an id the data handler does not know.
        e.register(LOAD_BUFFER_FORM_ID_OUT, |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        returns(&mut e, DATA_HANDLER_HAS_FORM, 1);
        let buffer = Ptr::<()>::new(0x7300_0000);
        e.call(0x0045_95e0, &args![tes, buffer]);
        e.register(LOAD_BUFFER_FORM_ID_OUT, |e, a| {
            e.mem.set_u32(a[1], 0x42);
            Ret::default()
        });
        returns(&mut e, DATA_HANDLER_HAS_FORM, 0);
        e.call(0x0045_95e0, &args![tes, buffer]);
        assert!(cleared.borrow().is_empty());
        assert!(loaded.borrow().is_empty());
    }

    // ----- 00459750 -----

    #[test]
    fn buffered_cells_that_are_loaded_are_finished() {
        let mut e = Engine::new();
        map_more_globals(&mut e);
        ni_pointer_doubles(&mut e);
        put_setting(&mut e, SETTING_INTERIOR_CELL_BUFFER, 2);
        put_setting(&mut e, SETTING_EXTERIOR_CELL_BUFFER, 3);
        e.set_global(TES_SINGLETON, 0x7200_0000u32);
        let tes = e.new_object::<TES>();
        let interior = e.mem.alloc(8);
        e.mem.set_u32(interior, 0xc1);
        let exterior = e.mem.alloc(12);
        e.mem.set_u32(exterior + 4, 0xc2);
        e.mem.set_u32(exterior + 8, 0xc3);
        e.set(tes, TES::pInteriorBuffer, Ptr::new(interior));
        e.set(tes, TES::pExteriorBuffer, Ptr::new(exterior));
        let bound = e.mem.alloc(4);
        e.mem.set_u32(bound, 0xb0b0);
        e.mem.set_u32(tes.addr() + 0xc0, bound);
        // Loaded: 0xc1 and 0xc3.
        let asked = Rc::new(RefCell::new(vec![]));
        let inner = asked.clone();
        e.register_double(TES_IS_CELL_LOADED, move |_, a| {
            inner.borrow_mut().push(a.to_vec());
            ret((a[1] != 0xc2) as u32)
        });
        let finished = recording(&mut e, CELL_FINISH, 0);
        let bound_user = recording(&mut e, 0x0062_8da0, 0);
        e.call(0x0045_9750, &args![tes]);
        assert_eq!(
            *asked.borrow(),
            vec![
                vec![0x7200_0000, 0xc1, 1],
                vec![0x7200_0000, 0xc2, 1],
                vec![0x7200_0000, 0xc3, 1]
            ]
        );
        assert_eq!(*finished.borrow(), vec![vec![0xc1], vec![0xc3]]);
        assert_eq!(*bound_user.borrow(), vec![vec![bound]]);
    }

    // ----- 00459870 -----

    #[test]
    fn furniture_list_uses_the_interior_cell_alone_when_there_is_one() {
        let mut e = Engine::new();
        map_more_globals(&mut e);
        put_setting(&mut e, SETTING_GRIDS_TO_LOAD, 2);
        let tes = e.new_object::<TES>();
        e.set(tes, TES::pInteriorCell, Ptr::new(0xce11));
        let added = recording(&mut e, CELL_ADD_FURNITURE_TO_LIST, 0);
        e.call(0x0045_9870, &args![tes]);
        assert_eq!(*added.borrow(), vec![vec![0xce11, tes.addr() + 0x94]]);
    }

    #[test]
    fn furniture_list_walks_the_whole_grid_without_an_interior_cell() {
        let mut e = Engine::new();
        map_more_globals(&mut e);
        put_setting(&mut e, SETTING_GRIDS_TO_LOAD, 2);
        let tes = e.new_object::<TES>();
        grid_with_cells(&mut e, tes, 2, &[0xc0, 0xc1, 0xc2, 0xc3]);
        let added = recording(&mut e, CELL_ADD_FURNITURE_TO_LIST, 0);
        e.call(0x0045_9870, &args![tes]);
        let list = tes.addr() + 0x94;
        assert_eq!(
            *added.borrow(),
            vec![
                vec![0xc0, list],
                vec![0xc1, list],
                vec![0xc2, list],
                vec![0xc3, list]
            ]
        );
    }

    // ----- 00459920, 00459a00 -----

    /// A grid of `[null, ready, not ready, ready]` cells and an interior
    /// cell (`ready` too) for the two cell walks. A cell is ready when its
    /// first word is 1.
    fn walk_world(with_handler: bool) -> (Engine, Ptr<TES>, Vec<u32>) {
        let mut e = Engine::new();
        map_more_globals(&mut e);
        put_setting(&mut e, SETTING_GRIDS_TO_LOAD, 2);
        e.set_global(DATA_HANDLER, if with_handler { 0x7200_0000u32 } else { 0 });
        let tes = e.new_object::<TES>();
        let mut cells = vec![];
        for ready in [1, 0, 1, 1] {
            let cell = e.mem.alloc(8);
            e.mem.set_u32(cell, ready);
            cells.push(cell);
        }
        grid_with_cells(&mut e, tes, 2, &[0, cells[0], cells[1], cells[2]]);
        e.register(CELL_STATE_TEST, |e, a| ret(e.mem.u32(a[0])));
        returns(&mut e, GET_INTERIOR_CELL, cells[3]);
        (e, tes, cells)
    }

    #[test]
    fn references_are_queued_for_the_ready_cells() {
        let (mut e, tes, cells) = walk_world(true);
        let queued = recording(&mut e, CELL_QUEUE_REFERENCES, 0);
        e.call(0x0045_9920, &args![tes]);
        assert_eq!(
            *queued.borrow(),
            vec![vec![cells[3], 0], vec![cells[0], 0], vec![cells[2], 0]]
        );
    }

    #[test]
    fn references_are_not_queued_without_a_data_handler() {
        let (mut e, tes, _) = walk_world(false);
        let queued = recording(&mut e, CELL_QUEUE_REFERENCES, 0);
        e.call(0x0045_9920, &args![tes]);
        assert!(queued.borrow().is_empty());
    }

    #[test]
    fn the_second_cell_walk_calls_the_other_cell_function() {
        let (mut e, tes, cells) = walk_world(true);
        let queued = recording(&mut e, CELL_QUEUE_B, 0);
        e.call(0x0045_9a00, &args![tes]);
        assert_eq!(
            *queued.borrow(),
            vec![vec![cells[3]], vec![cells[0]], vec![cells[2]]]
        );
        let (mut e, tes, _) = walk_world(false);
        let queued = recording(&mut e, CELL_QUEUE_B, 0);
        e.call(0x0045_9a00, &args![tes]);
        assert!(queued.borrow().is_empty());
    }

    // ----- 00459ae0, 00459d20 -----

    /// Doubles for `TES::TestCell`: the clock steps through `ticks`, the
    /// logger and the player, I/O and process-list calls are recorded.
    struct CellTest {
        e: Engine,
        errors: Log,
        centred: Log,
        mode_runs: Log,
        delay: Log,
    }

    fn cell_test_engine(ticks: Vec<u32>) -> CellTest {
        let mut e = priority_engine(1);
        e.set_global(PLAYER_CHARACTER, 0x7500_0000u32);
        e.set_global(IO_MANAGER, 0x7600_0000u32);
        e.set_global(SAVE_LOAD_GAME, 0x7100_0000u32);
        e.set_global(0x0101_7b70u32, 1000.0f64);
        e.set_global(0x0101_7e70u32, 120.0f32);
        let next = Rc::new(RefCell::new(0usize));
        e.register_double(GET_TICK_COUNT, move |_, _| {
            let mut next = next.borrow_mut();
            let tick = ticks[(*next).min(ticks.len() - 1)];
            *next += 1;
            ret(tick)
        });
        e.register(STRING_OBJECT_TEXT, |_, a| ret(a[0] + 0x100));
        e.register(FORM_ID_GETTER, |_, a| ret(a[0] ^ 0xf000_0000));
        let errors = recording(&mut e, ERROR_LOG, 0);
        let centred = recording(&mut e, 0x0093_db60, 0);
        let delay = recording(&mut e, 0x0086_7a40, 0);
        let mode_runs = recording(&mut e, 0x0086_1f20, 0);
        noop(&mut e, &[0x0045_6520, 0x0096_eb40, 0x0096_d810]);
        CellTest {
            e,
            errors,
            centred,
            mode_runs,
            delay,
        }
    }

    #[test]
    fn test_cell_does_nothing_for_a_null_cell() {
        let mut test = cell_test_engine(vec![0]);
        let tes = test.e.new_object::<TES>();
        test.e
            .call(0x0045_9d20, &args![tes, Ptr::<()>::NULL, 4u32, 0u32]);
        assert!(test.errors.borrow().is_empty());
        assert!(test.centred.borrow().is_empty());
    }

    #[test]
    fn test_cell_reports_an_interior_cell_and_the_elapsed_times() {
        // before = 10000, after = 12500, now = 5000 + 1h 1m 1s.
        let mut test = cell_test_engine(vec![10_000, 12_500, 3_666_000]);
        let tes = test.e.new_object::<TES>();
        let cell = cell_object(&mut test.e, true, 0, 0);
        test.e.call(0x0045_9d20, &args![tes, cell, 3u32, 5_000u32]);
        let id = cell.addr() ^ 0xf000_0000;
        let name = cell.addr() + 0x18 + 0x100;
        assert_eq!(
            *test.errors.borrow(),
            vec![
                args![0x0101_7f54u32, id, name],
                args![0x0101_7ed8u32, 2.5f64, 1u32, 1u32, 1u32]
            ]
        );
        assert_eq!(
            *test.centred.borrow(),
            vec![vec![0x7500_0000, 0, cell.addr()]]
        );
        assert_eq!(
            *test.delay.borrow(),
            vec![vec![OBJECT_011DE7B8, 120.0f32.to_bits()]]
        );
        assert!(test.mode_runs.borrow().is_empty());
    }

    #[test]
    fn test_cell_reports_an_exterior_cell_and_runs_the_save_test_in_mode_4() {
        let mut test = cell_test_engine(vec![0, 1_000, 61_500]);
        let tes = test.e.new_object::<TES>();
        let cell = cell_object(&mut test.e, false, -3, 8);
        test.e.call(0x0045_9d20, &args![tes, cell, 4u32, 0u32]);
        let id = cell.addr() ^ 0xf000_0000;
        let name = cell.addr() + 0x18 + 0x100;
        let minus_three = (-3i32) as u32;
        assert_eq!(
            *test.errors.borrow(),
            vec![
                args![0x0101_7f2cu32, id, minus_three, 8u32, name],
                args![0x0101_7e78u32, minus_three, 8u32, 1.0f64, 0u32, 1u32, 1u32]
            ]
        );
        assert_eq!(*test.mode_runs.borrow(), vec![vec![0x7100_0000, 4]]);
    }

    #[test]
    fn test_cell_runs_the_save_test_only_for_modes_4_and_5() {
        for (mode, runs) in [(3u32, 0), (4, 1), (5, 1), (6, 0)] {
            let mut test = cell_test_engine(vec![0]);
            let tes = test.e.new_object::<TES>();
            let cell = cell_object(&mut test.e, true, 0, 0);
            test.e.call(0x0045_9d20, &args![tes, cell, mode, 0u32]);
            assert_eq!(test.mode_runs.borrow().len(), runs, "mode {mode}");
        }
    }

    #[test]
    fn cell_test_run_walks_each_world_space_and_then_the_interiors() {
        let mut test = cell_test_engine(vec![0]);
        let e = &mut test.e;
        put_setting(e, SETTING_GRIDS_TO_LOAD, 1);
        let tes = e.new_object::<TES>();
        e.set(tes, TES::bRunningCellTests2, true);
        // One world space with bounds (0,0) to (2,2): four cells, the one at
        // (1,1) not loaded yet.
        let world_space = e.mem.alloc(0x100);
        e.set(tes, TES::pWorldSpace, Ptr::new(world_space));
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, world_space);
        e.set_global(DATA_HANDLER, 0x7200_0000u32);
        returns(e, DATA_HANDLER_FIRST_WORLD_SPACE, node);
        e.register(LIST_NODE_IS_EMPTY, |_, _| ret(0));
        noop(e, &[0x0095_26a0, 0x0070_37e0]);
        list_doubles(e);
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            ret(value as i32 as u32)
        });
        for (getter, bound) in [
            (0x0045_65f0u32, 0.0f64),
            (0x0081_2870, 0.0),
            (0x009b_88a0, 2.0),
            (0x009b_88c0, 2.0),
        ] {
            e.register_double(getter, move |_, _| Ret {
                st0: bound * 4096.0,
                ..Ret::default()
            });
        }
        e.register(STRING_LENGTH, |_, _| ret(0));
        let table = vtable(e, &[(0x130, 0x7000_0130)]);
        e.mem.set_u32(world_space, table);
        returns(e, 0x7000_0130, 0x5eed);
        let loaded_cell = cell_object(e, false, 1, 1).addr();
        e.register_double(WORLD_SPACE_GET_CELL, move |e, a| {
            if (a[1], a[2]) == (1, 1) {
                ret(0)
            } else {
                ret(cell_object(e, false, a[1] as i32, a[2] as i32).addr())
            }
        });
        let loaded = recording(e, WORLD_SPACE_LOAD_CELL, loaded_cell);
        // Two interior cells.
        returns(e, DATA_HANDLER_PREPARE_INTERIORS, 0);
        returns(e, DATA_HANDLER_INTERIOR_COUNT, 2);
        let interior_a = cell_object(e, true, 0, 0).addr();
        let interior_b = cell_object(e, true, 0, 0).addr();
        e.register_double(DATA_HANDLER_INTERIOR_AT, move |_, a| {
            ret([interior_a, interior_b][a[1] as usize])
        });
        e.call(0x0045_9ae0, &args![tes, 3u32]);
        let centred: Vec<u32> = test.centred.borrow().iter().map(|call| call[2]).collect();
        assert_eq!(centred.len(), 6);
        assert_eq!(centred[3], loaded_cell);
        assert_eq!(centred[4..], [interior_a, interior_b]);
        assert_eq!(*loaded.borrow(), vec![vec![world_space, 1, 1]]);
        let errors = test.errors.borrow();
        assert_eq!(errors[0], args![0x0101_7e5cu32]);
        assert_eq!(
            errors[1],
            args![
                0x0101_7e24u32,
                world_space ^ 0xf000_0000,
                0x5eedu32,
                0u32,
                0u32,
                2u32,
                2u32
            ]
        );
        assert!(test.e.get(tes, TES::bRunningCellTests));
        assert!(!test.e.get(tes, TES::bRunningCellTests2));
    }

    // ----- 00459f10 and its helpers -----

    #[test]
    fn small_addon_helpers_read_and_write_their_fields() {
        let mut e = Engine::new();
        ni_pointer_doubles(&mut e);
        map_more_globals(&mut e);
        let object = Ptr::<()>::new(e.mem.alloc(0x200));
        e.mem.set_u16(object.addr() + 0x58, 0x1357);
        assert_eq!(e.call(0x0045_a1a0, &args![object]).u16(), 0x1357);
        e.call(0x0045_a140, &args![object, 0x2468u16]);
        assert_eq!(e.mem.u16(object.addr() + 0xbc), 0x2468);
        assert_eq!(
            e.call(0x0045_a330, &args![object]).u32(),
            object.addr() + 0x180
        );
        assert_eq!(
            e.call(0x0045_a730, &args![object]).u32(),
            object.addr() + 0x130
        );
        let counted = recording(&mut e, 0x0065_8930, 7);
        assert_eq!(e.call(0x0045_a120, &args![object]).u32(), 7);
        assert_eq!(*counted.borrow(), vec![vec![object.addr() + 0x1ec]]);
        e.mem.set_u32(ADDON_PARENT_CELL, 0xfeed);
        assert_eq!(e.call(0x0045_a190, &args![]).u32(), 0xfeed);
    }

    #[test]
    fn attach_helper_calls_slot_0xdc_with_the_child_and_one() {
        let mut e = Engine::new();
        let table = vtable(&mut e, &[(0xdc, 0x7000_00dc)]);
        let parent = object_with(&mut e, table);
        let attached = recording(&mut e, 0x7000_00dc, 0);
        e.call(0x0045_a160, &args![parent, Ptr::<()>::new(0xc41d)]);
        assert_eq!(*attached.borrow(), vec![vec![parent.addr(), 0xc41d, 1]]);
    }

    #[test]
    fn preloaded_pointers_are_cleared_through_the_queued_file_assignment() {
        let mut e = Engine::new();
        let assigned = recording(&mut e, QUEUED_FILE_POINTER_ASSIGN, 0);
        let tes = e.new_object::<TES>();
        e.call(0x0045_a1c0, &args![tes]);
        e.call(0x0045_a350, &args![tes]);
        assert_eq!(
            *assigned.borrow(),
            vec![vec![tes.addr() + 0xa4, 0], vec![tes.addr() + 0xac, 0]]
        );
    }

    /// The doubles shared by the two preload functions: the data handler
    /// object, the model loader, the scope guard, the queued-file allocation
    /// and the `NiPointer` getter.
    struct Preload {
        e: Engine,
        tes: Ptr<TES>,
        handler: u32,
        queued_file: u32,
        guard: Log,
    }

    fn preload_world() -> Preload {
        let mut e = Engine::new();
        map_more_globals(&mut e);
        ni_pointer_doubles(&mut e);
        list_doubles(&mut e);
        let tes = e.new_object::<TES>();
        let handler = e.mem.alloc(0x400);
        e.set_global(DATA_HANDLER, handler);
        e.set_global(MODEL_LOADER, 0x7700_0000u32);
        let queued_file = e.mem.alloc(0x28);
        returns(&mut e, OPERATOR_NEW, queued_file);
        e.register(QUEUED_FILE_CTOR, |_, a| ret(a[0]));
        e.register(QUEUED_FILE_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let guard = recording(&mut e, SCOPE_GUARD_CTOR, 0);
        noop(&mut e, &[SCOPE_GUARD_DTOR]);
        Preload {
            e,
            tes,
            handler,
            queued_file,
            guard,
        }
    }

    /// An add-on node whose sub-object (`node + 0x30`) has slot `0x14` giving
    /// `name`, flag byte (what `00448bf0` answers) and the words the loader
    /// path reads.
    fn addon_node(e: &mut Engine, name: u32) -> u32 {
        let table = vtable(e, &[(0x14, 0x7000_0014), (0x10, 0x7000_0010)]);
        let node = e.mem.alloc(0x100);
        e.mem.set_u32(node + 0x30, table);
        e.mem.set_u16(node + 0x58, 0x0abc);
        returns(e, 0x7000_0014, name);
        node
    }

    #[test]
    fn addon_nodes_do_nothing_when_already_preloaded() {
        let mut world = preload_world();
        world
            .e
            .set(world.tes, TES::spPreloadedAddonNodes, Ptr::new(0x1234));
        world.e.call(0x0045_9f10, &args![world.tes]);
        assert!(world.guard.borrow().is_empty());
    }

    #[test]
    fn addon_nodes_that_load_now_are_loaded_attached_and_released() {
        let mut world = preload_world();
        let e = &mut world.e;
        let node = addon_node(e, 0x6000);
        e.mem.set_u32(world.handler + 0x1ec, 0);
        returns(e, 0x0065_8930, 1);
        e.register_double(DATA_HANDLER_GET_ADDON_NODE, move |_, a| {
            assert_eq!(a[1], 0);
            ret(node)
        });
        returns(e, ADDON_NODE_LOADS_NOW, 1);
        let model = e.mem.alloc(0x100);
        let loaded = recording(e, MODEL_LOADER_LOAD_FILE, model);
        returns(e, 0x0068_a830, 0x11);
        returns(e, 0x0059_e300, 0x22);
        let first = recording(e, 0x0040_30d0, 0);
        let second = recording(e, 0x0070_5b10, 0);
        let root_table = vtable(e, &[(0xdc, 0x7000_00dc)]);
        let root = object_with(e, root_table);
        e.mem.set_u32(ADDON_PARENT_CELL, root.addr());
        let attached = recording(e, 0x7000_00dc, 0);
        let released = recording(e, MODEL_LOADER_RELEASE_MODEL, 0);
        let queued = recording(e, MODEL_LOADER_QUEUE_FILE, 0);
        e.call(0x0045_9f10, &args![world.tes]);
        assert_eq!(world.guard.borrow()[0][1..], [7, 1, TES_CPP_PATH, 0x1b8e]);
        assert_eq!(
            e.get(world.tes, TES::spPreloadedAddonNodes).addr(),
            world.queued_file
        );
        assert_eq!(
            *loaded.borrow(),
            vec![vec![0x7700_0000, 0x6000, 0, 1, 0, 0, 0]]
        );
        assert_eq!(*first.borrow(), vec![vec![model, 0x11]]);
        assert_eq!(*second.borrow(), vec![vec![model, 0x22]]);
        assert_eq!(e.mem.u16(model + 0xbc), 0x0abc);
        assert_eq!(*attached.borrow(), vec![vec![root.addr(), model, 1]]);
        assert_eq!(*released.borrow(), vec![vec![0x7700_0000, 0x6000, 1, 1]]);
        assert!(queued.borrow().is_empty());
    }

    #[test]
    fn addon_nodes_that_fail_to_load_are_still_released() {
        let mut world = preload_world();
        let e = &mut world.e;
        let node = addon_node(e, 0x6000);
        returns(e, 0x0065_8930, 1);
        returns(e, DATA_HANDLER_GET_ADDON_NODE, node);
        returns(e, ADDON_NODE_LOADS_NOW, 1);
        returns(e, MODEL_LOADER_LOAD_FILE, 0);
        let attached = recording(e, 0x0070_5b10, 0);
        let released = recording(e, MODEL_LOADER_RELEASE_MODEL, 0);
        e.call(0x0045_9f10, &args![world.tes]);
        assert!(attached.borrow().is_empty());
        assert_eq!(released.borrow().len(), 1);
    }

    #[test]
    fn other_addon_nodes_are_queued_on_the_queued_file() {
        let mut world = preload_world();
        let e = &mut world.e;
        let first = addon_node(e, 0x6000);
        let second = addon_node(e, 0x6100);
        returns(e, 0x0065_8930, 3);
        e.register_double(DATA_HANDLER_GET_ADDON_NODE, move |_, a| {
            ret([first, 0, second][a[1] as usize])
        });
        returns(e, ADDON_NODE_LOADS_NOW, 0);
        let queued = recording(e, MODEL_LOADER_QUEUE_FILE, 0);
        let loaded = recording(e, MODEL_LOADER_LOAD_FILE, 0);
        e.call(0x0045_9f10, &args![world.tes]);
        let file = world.queued_file;
        assert_eq!(
            *queued.borrow(),
            vec![
                vec![0x7700_0000, first + 0x30, 5, file, 0, 1, 0, 0],
                vec![0x7700_0000, second + 0x30, 5, file, 0, 1, 0, 0]
            ]
        );
        assert!(loaded.borrow().is_empty());
    }

    // ----- 0045a1e0 -----

    fn empty_node(e: &mut Engine) -> u32 {
        let node = e.mem.alloc(8);
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        node
    }

    #[test]
    fn preload_forms_queues_the_files_of_each_entry_and_of_the_weapons() {
        let mut world = preload_world();
        let e = &mut world.e;
        empty_node(e);
        let first = addon_node(e, 0);
        let second = addon_node(e, 0);
        fill_list(e, world.handler + 0x180, &[first, 0, second]);
        let asked_first = recording(e, 0x7000_0010, 0);
        e.set_global(WEAPON_QUEUE_OBJECT, 0x7800_0000u32);
        let weapons = recording(e, 0x0052_5510, 0);
        e.call(0x0045_a1e0, &args![world.tes]);
        let file = world.queued_file;
        assert_eq!(
            *asked_first.borrow(),
            vec![vec![first + 0x30, 5, file], vec![second + 0x30, 5, file]]
        );
        assert_eq!(*weapons.borrow(), vec![vec![0x7800_0000, 5, file, 0]]);
        assert_eq!(
            e.get(world.tes, TES::spPreloadedForms).addr(),
            world.queued_file
        );
        assert_eq!(world.guard.borrow()[0][1..], [7, 1, TES_CPP_PATH, 0x1bd0]);
    }

    #[test]
    fn preload_forms_stops_at_an_empty_node_and_skips_missing_weapons() {
        let mut world = preload_world();
        let e = &mut world.e;
        empty_node(e);
        let entry = addon_node(e, 0);
        // The head node holds nothing and leads nowhere: the list is empty.
        let asked = recording(e, 0x7000_0010, 0);
        let weapons = recording(e, 0x0052_5510, 0);
        e.call(0x0045_a1e0, &args![world.tes]);
        assert!(asked.borrow().is_empty());
        assert!(weapons.borrow().is_empty());
        // Already preloaded: nothing at all.
        let mut world = preload_world();
        world
            .e
            .set(world.tes, TES::spPreloadedForms, Ptr::new(0x99));
        world.e.call(0x0045_a1e0, &args![world.tes]);
        assert!(world.guard.borrow().is_empty());
        let _ = entry;
    }

    // ----- 0045a370, 0045a520, 0045a5e0 -----

    /// String settings with text: the double gives `setting + 8`.
    fn text_settings(e: &mut Engine, texts: &[(u32, &str)]) {
        e.register(SETTING_VALUE_ADDRESS_STRING, |_, a| ret(a[0] + 8));
        for setting in [
            SETTING_RAGDOLL_DATA_DEFAULT,
            SETTING_BLOOD_PARTICLE_DEFAULT,
            SETTING_SPLASH_PARTICLES,
            SETTING_EXPLOSION_SPLASH_PARTICLES,
            SETTING_BLOOD_TEXTURE_DEFAULT,
        ] {
            e.mem.set_cstr(setting + 8, b"");
        }
        for (setting, text) in texts {
            e.mem.set_cstr(setting + 8, text.as_bytes());
        }
    }

    #[test]
    fn default_models_are_queued_and_the_blood_texture_is_loaded() {
        let mut world = preload_world();
        let e = &mut world.e;
        text_settings(
            e,
            &[
                (SETTING_RAGDOLL_DATA_DEFAULT, "a.rdt"),
                (SETTING_SPLASH_PARTICLES, "c.nif"),
                (SETTING_BLOOD_TEXTURE_DEFAULT, "blood.dds"),
            ],
        );
        let queued = recording(e, MODEL_LOADER_QUEUE_MODEL, 0);
        let formatted = recording(e, FORMAT_STRING, 0);
        let texture = recording(e, CREATE_TEXTURE_IMAGE, 0);
        e.call(0x0045_a370, &args![world.tes]);
        assert_eq!(
            *queued.borrow(),
            vec![
                vec![
                    0x7700_0000,
                    SETTING_RAGDOLL_DATA_DEFAULT + 8,
                    5,
                    0,
                    0,
                    1,
                    1,
                    0
                ],
                vec![0x7700_0000, SETTING_SPLASH_PARTICLES + 8, 5, 0, 0, 1, 1, 0]
            ]
        );
        let format = formatted.borrow()[0].clone();
        assert_eq!(
            format[1..],
            [
                0x104,
                0x0101_7f74,
                0x0101_7f80,
                0x0101_7f88,
                SETTING_BLOOD_TEXTURE_DEFAULT + 8
            ]
        );
        assert_eq!(
            *texture.borrow(),
            vec![vec![
                world.tes.addr(),
                format[0],
                world.tes.addr() + 0xa8,
                0,
                0
            ]]
        );
        assert_eq!(world.guard.borrow()[0][1..], [7, 1, TES_CPP_PATH, 0x1bf0]);
    }

    #[test]
    fn nothing_is_queued_when_the_settings_are_empty() {
        let mut world = preload_world();
        let e = &mut world.e;
        text_settings(e, &[]);
        let queued = recording(e, MODEL_LOADER_QUEUE_MODEL, 0);
        let texture = recording(e, CREATE_TEXTURE_IMAGE, 0);
        e.call(0x0045_a370, &args![world.tes]);
        assert!(queued.borrow().is_empty());
        assert!(texture.borrow().is_empty());
    }

    #[test]
    fn preloaded_models_are_released_and_the_blood_decal_cleared() {
        let mut world = preload_world();
        let e = &mut world.e;
        text_settings(
            e,
            &[
                (SETTING_BLOOD_PARTICLE_DEFAULT, "b.nif"),
                (SETTING_EXPLOSION_SPLASH_PARTICLES, "d.nif"),
            ],
        );
        let released = recording(e, MODEL_LOADER_RELEASE_MODEL, 0);
        let cleared = recording(e, NI_POINTER_ASSIGN, 0);
        e.call(0x0045_a520, &args![world.tes]);
        assert_eq!(
            *released.borrow(),
            vec![
                vec![0x7700_0000, SETTING_BLOOD_PARTICLE_DEFAULT + 8, 1, 1],
                vec![0x7700_0000, SETTING_EXPLOSION_SPLASH_PARTICLES + 8, 1, 1]
            ]
        );
        assert_eq!(*cleared.borrow(), vec![vec![world.tes.addr() + 0xa8, 0]]);
    }

    #[test]
    fn release_helper_passes_the_name_and_two_ones() {
        let mut e = Engine::new();
        let released = recording(&mut e, MODEL_LOADER_RELEASE_MODEL, 0);
        e.call(0x0045_a5e0, &args![Ptr::<()>::new(0x7700_0000), 0xabcdu32]);
        assert_eq!(*released.borrow(), vec![vec![0x7700_0000, 0xabcd, 1, 1]]);
    }

    // ----- 0045a600 -----

    #[test]
    fn water_textures_are_loaded_into_a_local_pointer_for_accepted_entries() {
        let mut world = preload_world();
        let e = &mut world.e;
        empty_node(e);
        e.mem.set_u8(SETTING_PER_WORLD_SPACE_WATER_NOISE + 4, 0);
        put_setting(e, SETTING_PER_WORLD_SPACE_WATER_NOISE, 0);
        e.set_global(TES_SINGLETON, 0x7900_0000u32);
        e.register(NI_POINTER_CTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        let accepted = e.mem.alloc(8);
        let rejected = e.mem.alloc(8);
        fill_list(e, world.handler + 0x130, &[accepted, rejected]);
        e.register_double(0x0058_0080, move |_, a| ret((a[0] == accepted) as u32));
        returns(e, TES_WATER_SYSTEM_GETTER, 0);
        returns(e, 0x0058_00a0, 0x4e41);
        let texture = Rc::new(RefCell::new(0u32));
        let inner = texture.clone();
        e.register_double(CREATE_TEXTURE_IMAGE, move |e, a| {
            *inner.borrow_mut() = a[2];
            e.mem.set_u32(a[2], 0x7e57);
            Ret::default()
        });
        let received = recording(e, 0x0058_00c0, 0);
        e.call(0x0045_a600, &args![world.tes]);
        assert_eq!(*received.borrow(), vec![vec![accepted, 0x7e57]]);
        assert_eq!(
            world.guard.borrow()[0][1..],
            [0x1d, 1, TES_CPP_PATH, 0x1c27]
        );
        // The local pointer was released after use.
        assert_eq!(e.mem.u32(*texture.borrow()), 0);
    }

    #[test]
    fn water_textures_are_skipped_with_per_world_space_noise() {
        let mut world = preload_world();
        let e = &mut world.e;
        put_setting(e, SETTING_PER_WORLD_SPACE_WATER_NOISE, 1);
        let listed = recording(e, 0x0058_0080, 0);
        let water = recording(e, TES_WATER_SYSTEM_GETTER, 0);
        e.call(0x0045_a600, &args![world.tes]);
        assert!(listed.borrow().is_empty());
        assert_eq!(*water.borrow(), vec![vec![world.tes.addr()]]);
    }

    // ===== Session 3: `0045a750` to `0045cac0` =====

    /// The text of the C string at `address`.
    fn cstr(e: &Engine, address: u32) -> String {
        let mut text = String::new();
        let mut at = address;
        while e.mem.u8(at) != 0 {
            text.push(e.mem.u8(at) as char);
            at += 1;
        }
        text
    }

    /// An engine with the globals mapped and the particle cache's callees
    /// (list links, `NiPointer`, allocation) replaced by simple doubles.
    fn cache_engine() -> Engine {
        let mut e = Engine::new();
        map_globals(&mut e);
        for page in [0x011c_e000u32, 0x011c_f000, 0x011d_1000] {
            e.map(page, 0x1000);
        }
        ni_pointer_doubles(&mut e);
        noop(
            &mut e,
            &[SCOPE_GUARD_CTOR, SCOPE_GUARD_DTOR, VECTOR_CONSTRUCT],
        );
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(CACHE_ENTRY_HOLDS_OBJECT, |e, a| {
            ret((e.mem.u32(a[0]) == a[1]) as u32)
        });
        e.register(LINK_GET_NEXT, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(LINK_SET_NEXT, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(NI_OBJECT_CLONE, |_, a| ret(a[0] + 0x100));
        e.register(NI_POINTER_COPY_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(NODE_PARENT, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e
    }

    /// Gives each listed string setting the text given; the other particle
    /// settings are empty.
    fn set_particle_texts(e: &mut Engine, texts: &[(u32, &str)]) {
        let empty = e.mem.alloc(4);
        let mut table = vec![];
        for setting in PARTICLE_CACHE_SETTINGS {
            let text = match texts.iter().find(|(s, _)| *s == setting) {
                Some((_, text)) => {
                    let at = e.mem.alloc(32);
                    e.mem.write(at, text.as_bytes());
                    at
                }
                None => empty,
            };
            table.push((setting, text));
        }
        e.register_double(SETTING_VALUE_ADDRESS_STRING, move |_, a| {
            ret(table.iter().find(|(s, _)| *s == a[0]).unwrap().1)
        });
    }

    /// A cache entry holding `object` with three cached cells, as
    /// `TES::AddTemporaryParticleObjectToCache` builds them; `parents` gives
    /// the parent word of each cached object (0 = none).
    fn cache_entry(e: &mut Engine, object: u32, parents: [u32; 3], next: u32) -> u32 {
        let entry = e.mem.alloc(12);
        e.mem.set_u32(entry, object);
        e.mem.set_u32(entry + 8, next);
        let cells = e.mem.alloc(12);
        e.mem.set_u32(entry + 4, cells);
        for (index, parent) in parents.iter().enumerate() {
            let cached = e.mem.alloc(0x20);
            e.mem.set_u32(cached + 0x18, *parent);
            e.mem.set_u32(cells + 4 * index as u32, cached);
        }
        entry
    }

    // ----- 0045a750 -----

    #[test]
    fn setup_loads_the_models_with_text_once_and_caches_them() {
        let mut e = cache_engine();
        set_particle_texts(
            &mut e,
            &[
                (SETTING_DISMEMBER_PARTICLE_DEFAULT, "dismember.nif"),
                (SETTING_BLOOD_PARTICLE_DEFAULT, "blood.nif"),
            ],
        );
        e.set_global(MODEL_LOADER, 0x5555u32);
        let loads = Rc::new(RefCell::new(vec![]));
        let inner = loads.clone();
        e.register_double(MODEL_LOADER_LOAD_FILE, move |_, a| {
            inner.borrow_mut().push(a.to_vec());
            ret(0x7000 * inner.borrow().len() as u32)
        });
        let tes: Ptr<TES> = e.new_object();
        e.call(0x0045_a750, &args![tes]);
        {
            let loads = loads.borrow();
            assert_eq!(loads.len(), 2);
            assert_eq!(cstr(&e, loads[0][1]), "dismember.nif");
            assert_eq!(cstr(&e, loads[1][1]), "blood.nif");
            for load in loads.iter() {
                assert_eq!((load[0], load[2..].to_vec()), (0x5555, vec![0, 1, 0, 0, 0]));
            }
        }
        assert_eq!(e.mem.u8(PARTICLE_CACHE_READY), 1);
        // Two entries: the later model first.
        let head = e.get(tes, TES::pParticleCacheHead).addr();
        assert_eq!(e.mem.u32(head), 0x7000 * 2);
        let tail = e.mem.u32(head + 8);
        assert_eq!(e.mem.u32(tail), 0x7000);
        assert_eq!(e.mem.u32(tail + 8), 0);
        // A second call finds the cache filled and does nothing.
        e.call(0x0045_a750, &args![tes]);
        assert_eq!(loads.borrow().len(), 2);
    }

    // ----- 0045a9a0, 0045ab30 -----

    #[test]
    fn adding_an_object_makes_an_entry_with_the_object_and_two_clones() {
        let mut e = cache_engine();
        let tes: Ptr<TES> = e.new_object();
        e.call(0x0045_a9a0, &args![tes, 0u32]);
        assert!(e.get(tes, TES::pParticleCacheHead).is_null());
        e.call(0x0045_a9a0, &args![tes, 0x4000u32]);
        let entry = e.get(tes, TES::pParticleCacheHead).addr();
        assert_eq!(e.mem.u32(entry), 0x4000);
        assert_eq!(e.mem.u32(entry + 8), 0);
        let cells = e.mem.u32(entry + 4);
        assert_eq!(e.mem.u32(cells - 4), 3);
        assert_eq!(
            [0, 4, 8].map(|offset| e.mem.u32(cells + offset)),
            [0x4000, 0x4100, 0x4100]
        );
        // The same object again changes nothing; another one goes first.
        e.call(0x0045_a9a0, &args![tes, 0x4000u32]);
        assert_eq!(e.get(tes, TES::pParticleCacheHead).addr(), entry);
        e.call(0x0045_a9a0, &args![tes, 0x5000u32]);
        let second = e.get(tes, TES::pParticleCacheHead).addr();
        assert_ne!(second, entry);
        assert_eq!(e.mem.u32(second + 8), entry);
    }

    #[test]
    fn a_cache_entry_starts_empty() {
        let mut e = cache_engine();
        let block = e.mem.alloc(12);
        e.mem.set_u32(block, 0x1111);
        e.mem.set_u32(block + 4, 0x2222);
        e.mem.set_u32(block + 8, 0x3333);
        assert_eq!(e.call(0x0045_ab30, &args![block]).u32(), block);
        assert_eq!([0, 4, 8].map(|offset| e.mem.u32(block + offset)), [0, 0, 0]);
    }

    // ----- 0045aba0 -----

    #[test]
    fn the_unused_cached_object_is_the_first_without_a_parent() {
        let mut e = cache_engine();
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x1234);
        let tes: Ptr<TES> = e.new_object();
        // No entry for the key: 0, and the cell was cleared.
        assert_eq!(e.call(0x0045_aba0, &args![tes, 0x4000u32, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), 0);
        // An entry whose three objects all have parents: 1.
        let used = cache_entry(&mut e, 0x4000, [1, 1, 1], 0);
        e.set(tes, TES::pParticleCacheHead, Ptr::new(used));
        assert_eq!(e.call(0x0045_aba0, &args![tes, 0x4000u32, out]).u32(), 1);
        assert_eq!(e.mem.u32(out), 0);
        // The second object is free: 2, with the cell set to it.
        let free = cache_entry(&mut e, 0x4000, [1, 0, 0], used);
        e.set(tes, TES::pParticleCacheHead, Ptr::new(free));
        assert_eq!(e.call(0x0045_aba0, &args![tes, 0x4000u32, out]).u32(), 2);
        let cells = e.mem.u32(free + 4);
        assert_eq!(e.mem.u32(out), e.mem.u32(cells + 4));
        // An entry without its array: 1.
        let bare = cache_entry(&mut e, 0x6000, [0, 0, 0], 0);
        e.mem.set_u32(bare + 4, 0);
        e.set(tes, TES::pParticleCacheHead, Ptr::new(bare));
        assert_eq!(e.call(0x0045_aba0, &args![tes, 0x6000u32, out]).u32(), 1);
    }

    // ----- 0045ac80, 0045ae20, 0045ae50 -----

    #[test]
    fn emptying_the_cache_destroys_the_entries_and_releases_the_models() {
        let mut e = cache_engine();
        set_particle_texts(
            &mut e,
            &[
                (SETTING_DISMEMBER_ROBOT_PARTICLE_DEFAULT, "robot.nif"),
                (SETTING_BLOOD_PARTICLE_DEFAULT, "blood.nif"),
            ],
        );
        e.set_global(MODEL_LOADER, 0x5555u32);
        let freed = recording(&mut e, OPERATOR_DELETE, 0);
        let arrays = recording(&mut e, NI_POINTER_ARRAY_DELETING_DESTRUCTOR, 0);
        let released = recording(&mut e, MODEL_LOADER_RELEASE_MODEL, 0);
        let tes: Ptr<TES> = e.new_object();
        let second = cache_entry(&mut e, 0x5000, [0, 0, 0], 0);
        let first = cache_entry(&mut e, 0x4000, [0, 0, 0], second);
        e.set(tes, TES::pParticleCacheHead, Ptr::new(first));
        e.call(0x0045_ac80, &args![tes]);
        assert!(e.get(tes, TES::pParticleCacheHead).is_null());
        assert_eq!(*freed.borrow(), vec![vec![first], vec![second]]);
        assert_eq!(arrays.borrow().len(), 2);
        // Blood first, the robot dismember particle last.
        let released = released.borrow();
        assert_eq!(released.len(), 2);
        assert_eq!(cstr(&e, released[0][1]), "blood.nif");
        assert_eq!(cstr(&e, released[1][1]), "robot.nif");
        assert_eq!(released[0][0], 0x5555);
        assert_eq!(released[0][2..], [1, 1]);
    }

    #[test]
    fn destroying_an_entry_frees_it_only_with_the_flag() {
        let mut e = cache_engine();
        let freed = recording(&mut e, OPERATOR_DELETE, 0);
        let arrays = recording(&mut e, NI_POINTER_ARRAY_DELETING_DESTRUCTOR, 0);
        let entry = cache_entry(&mut e, 0x4000, [0, 0, 0], 0);
        let cells = e.mem.u32(entry + 4);
        assert_eq!(e.call(0x0045_ae20, &args![entry, 0u32]).u32(), entry);
        assert!(freed.borrow().is_empty());
        assert_eq!(*arrays.borrow(), vec![vec![cells, 3]]);
        // The pointer cell was cleared (assignment of null, then release).
        assert_eq!(e.mem.u32(entry), 0);
        assert_eq!(e.call(0x0045_ae20, &args![entry, 3u32]).u32(), entry);
        assert_eq!(*freed.borrow(), vec![vec![entry]]);
        // An entry without an array has nothing to destroy.
        let bare = e.mem.alloc(12);
        e.call(0x0045_ae50, &args![bare]);
        assert_eq!(arrays.borrow().len(), 2);
    }

    // ----- 0045aee0 -----

    #[test]
    fn the_purge_switch_stores_the_byte() {
        let mut e = Engine::new();
        let tes: Ptr<TES> = e.new_object();
        e.call(0x0045_aee0, &args![tes, 1u8]);
        assert!(e.get(tes, TES::bAllowUnusedPurge));
        e.call(0x0045_aee0, &args![tes, 0u8]);
        assert!(!e.get(tes, TES::bAllowUnusedPurge));
    }

    // ----- 0045af00, 0045afb0 -----

    #[test]
    fn the_nav_mesh_info_map_is_created_on_demand() {
        let mut e = cache_engine();
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        let created = recording(&mut e, NAV_MESH_INFO_MAP_CONSTRUCT, 0x9999);
        let blocked = Rc::new(RefCell::new(0u32));
        let inner = blocked.clone();
        e.register_double(DATA_HANDLER_BLOCKS_NAV_MESH_INFO_MAP, move |_, _| {
            ret(*inner.borrow())
        });
        let tes: Ptr<TES> = e.new_object();
        // No data handler: nothing is made.
        assert!(e.call(0x0045_af00, &args![tes]).ptr::<()>().is_null());
        // A data handler that blocks the map: still nothing.
        e.set_global(DATA_HANDLER, 0x1234u32);
        *blocked.borrow_mut() = 1;
        assert!(e.call(0x0045_af00, &args![tes]).ptr::<()>().is_null());
        assert!(created.borrow().is_empty());
        // Allowed: a 0x40-byte object is built and kept.
        *blocked.borrow_mut() = 0;
        assert_eq!(e.call(0x0045_af00, &args![tes]).u32(), 0x9999);
        assert_eq!(created.borrow().len(), 1);
        assert_eq!(e.get(tes, TES::pNavMeshInfoMap).addr(), 0x9999);
        // Already there: returned as is.
        assert_eq!(e.call(0x0045_af00, &args![tes]).u32(), 0x9999);
        assert_eq!(created.borrow().len(), 1);
    }

    #[test]
    fn setting_the_nav_mesh_info_map_deletes_the_old_one() {
        let mut e = Engine::new();
        map_globals(&mut e);
        let warnings = recording(&mut e, LOG_WARNING, 0);
        let deleted = recording(&mut e, 0x7000_0010, 0);
        let table = vtable(&mut e, &[(0x10, 0x7000_0010)]);
        let old = object_with(&mut e, table);
        let tes: Ptr<TES> = e.new_object();
        // No old map: just stored.
        e.call(0x0045_afb0, &args![tes, 0x2000u32]);
        assert_eq!(e.get(tes, TES::pNavMeshInfoMap).addr(), 0x2000);
        assert!(warnings.borrow().is_empty());
        // An old map and a new one: warning, old deleted with argument 1.
        e.set(tes, TES::pNavMeshInfoMap, old);
        e.call(0x0045_afb0, &args![tes, 0x3000u32]);
        assert_eq!(
            *warnings.borrow(),
            vec![vec![MESSAGE_MULTIPLE_NAV_MESH_INFO_MAPS]]
        );
        assert_eq!(*deleted.borrow(), vec![vec![old.addr(), 1]]);
        assert_eq!(e.get(tes, TES::pNavMeshInfoMap).addr(), 0x3000);
        // An old map and none: deleted without a warning.
        e.set(tes, TES::pNavMeshInfoMap, old);
        e.call(0x0045_afb0, &args![tes, 0u32]);
        assert_eq!(warnings.borrow().len(), 1);
        assert_eq!(deleted.borrow().len(), 2);
        assert!(e.get(tes, TES::pNavMeshInfoMap).is_null());
    }

    // ----- 0045b020, 0045b050 -----

    #[test]
    fn the_two_setup_functions_call_their_routines_in_order() {
        let mut e = Engine::new();
        noop(&mut e, &STATIC_SETUP_CALLS);
        noop(&mut e, &STATIC_TEARDOWN_CALLS);
        e.call_log = Some(vec![]);
        e.call(0x0045_b020, &[]);
        e.call(0x0045_b050, &[]);
        let log = e.call_log.take().unwrap();
        let called: Vec<u32> = log.iter().map(|call| call.0).collect();
        let mut expected = vec![0x0045_b020];
        expected.extend(STATIC_SETUP_CALLS);
        expected.push(0x0045_b050);
        expected.extend(STATIC_TEARDOWN_CALLS);
        assert_eq!(called, expected);
    }

    // ----- 0045bad0, 0045baf0 -----

    #[test]
    fn the_exact_type_test_compares_slot_8_with_the_rtti() {
        let mut e = Engine::new();
        let table = vtable(&mut e, &[(8, 0x7000_0008)]);
        let object = object_with(&mut e, table);
        returns(&mut e, 0x7000_0008, 0xabcd);
        assert_eq!(
            e.call(0x0045_baf0, &args![object, 0xabcdu32]).u32() & 0xff,
            1
        );
        assert_eq!(
            e.call(0x0045_baf0, &args![object, 0xabceu32]).u32() & 0xff,
            0
        );
        // `fn_0045bad0` takes the RTTI first and tests for a null object.
        assert!(e.call(0x0045_bad0, &args![0xabcdu32, object]).bool());
        assert!(!e.call(0x0045_bad0, &args![0xabceu32, object]).bool());
        assert!(!e.call(0x0045_bad0, &args![0xabcdu32, 0u32]).bool());
    }

    // ----- 0045bb20, 0045bb80, 0045bba0, 0045bbe0, 0045c650 -----

    #[test]
    fn a_scaled_vector_goes_to_the_out_vector_through_the_constructor() {
        let mut e = Engine::new();
        let constructed = recording(&mut e, VECTOR3_CONSTRUCT, 0);
        let this = e.mem.alloc(12);
        for (index, value) in [1.0f32, -2.0, 4.0].iter().enumerate() {
            e.mem.set_f32(this + 4 * index as u32, *value);
        }
        let out = e.mem.alloc(12);
        assert_eq!(e.call(0x0045_bb20, &args![this, out, 0.5f32]).u32(), out);
        assert_eq!(
            *constructed.borrow(),
            vec![vec![
                out,
                0.5f32.to_bits(),
                (-1.0f32).to_bits(),
                2.0f32.to_bits()
            ]]
        );
    }

    #[test]
    fn the_field_address_helpers_add_their_offsets() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0045_bb80, &args![0x1000u32]).u32(), 0x108c);
        assert_eq!(e.call(0x0045_bbe0, &args![0x1000u32]).u32(), 0x10dc);
        assert_eq!(e.call(0x0045_c650, &args![0x1000u32]).u32(), 0x10d0);
    }

    #[test]
    fn the_first_matrix_column_is_copied_to_the_out_vector() {
        let mut e = Engine::new();
        e.register(VECTOR_CTOR, |_, a| ret(a[0]));
        let asked = Rc::new(RefCell::new(vec![]));
        let inner = asked.clone();
        e.register_double(MATRIX_GET_COLUMN, move |e, a| {
            inner.borrow_mut().push(a.to_vec());
            for (index, value) in [7.0f32, 8.0, 9.0].iter().enumerate() {
                e.mem.set_f32(a[2] + 4 * index as u32, *value);
            }
            Ret::default()
        });
        let out = e.mem.alloc(12);
        assert_eq!(e.call(0x0045_bba0, &args![0x2000u32, out]).u32(), out);
        let asked = asked.borrow();
        assert_eq!((asked[0][0], asked[0][1]), (0x2068, 0));
        assert_eq!(
            [0, 4, 8].map(|offset| e.mem.f32(out + offset)),
            [7.0, 8.0, 9.0]
        );
    }

    // ----- 0045bc00, 0045bc40, 0045bc60 -----

    #[test]
    fn a_child_is_read_only_below_the_count() {
        let mut e = Engine::new();
        ni_pointer_doubles(&mut e);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0xc0de);
        returns(&mut e, CHILD_ARRAY_COUNT, 3);
        let asked = recording(&mut e, CHILD_ARRAY_SLOT, slot);
        assert_eq!(e.call(0x0045_bc00, &args![0x1000u32, 2u32]).u32(), 0xc0de);
        assert_eq!(*asked.borrow(), vec![vec![0x1000 + 0x9c, 2]]);
        // At or past the count: 0, with no lookup.
        assert_eq!(e.call(0x0045_bc00, &args![0x1000u32, 3u32]).u32(), 0);
        assert_eq!(asked.borrow().len(), 1);
    }

    #[test]
    fn the_scene_helpers_work_on_their_fields() {
        let mut e = Engine::new();
        ni_pointer_doubles(&mut e);
        let this = e.mem.alloc(0x40);
        e.call(0x0045_bc40, &args![this, 0x1234u32]);
        assert_eq!(e.mem.u32(this + 0x2c), 0x1234);
        let cleared = recording(&mut e, LIST_CLEAR, 0);
        e.call(0x0045_bc60, &args![this]);
        assert_eq!(*cleared.borrow(), vec![vec![this + 0x30]]);
    }

    // ----- 0045c4a0 to 0045c5f0 -----

    #[test]
    fn small_field_accessors() {
        let mut e = Engine::new();
        ni_pointer_doubles(&mut e);
        let this = e.mem.alloc(0x110);
        e.mem.set_u32(this + 0x40, 0xffff);
        e.call(0x0045_c4a0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x40), 0);
        e.mem.set_u32(this + 0x104, 0x4321);
        assert_eq!(e.call(0x0045_c4c0, &args![this]).u32(), 0x4321);
    }

    #[test]
    fn the_global_pointer_cell_is_read() {
        let mut e = Engine::new();
        map_globals(&mut e);
        ni_pointer_doubles(&mut e);
        e.set_global(0x011d_eb7cu32, 0x7777u32);
        assert_eq!(e.call(0x0045_c670, &[]).u32(), 0x7777);
    }

    #[test]
    fn replacing_the_object_destroys_a_different_old_one() {
        let mut e = Engine::new();
        let destroyed = recording(&mut e, OBJECT_DESTRUCT, 0);
        let freed = recording(&mut e, OPERATOR_DELETE, 0);
        let this = e.mem.alloc(0xe0);
        // Nothing there yet: just stored.
        e.call(0x0045_c4e0, &args![this, 0x9000u32]);
        assert_eq!(e.mem.u32(this + 0xcc), 0x9000);
        assert!(destroyed.borrow().is_empty());
        // The same object again: unchanged, nothing destroyed.
        e.call(0x0045_c4e0, &args![this, 0x9000u32]);
        assert!(destroyed.borrow().is_empty());
        // A different one: the old one is destroyed and freed first.
        e.call(0x0045_c4e0, &args![this, 0xa000u32]);
        assert_eq!(*destroyed.borrow(), vec![vec![0x9000]]);
        assert_eq!(*freed.borrow(), vec![vec![0x9000]]);
        assert_eq!(e.mem.u32(this + 0xcc), 0xa000);
        // Clearing destroys it as well.
        e.call(0x0045_c4e0, &args![this, 0u32]);
        assert_eq!(destroyed.borrow().len(), 2);
        assert_eq!(e.mem.u32(this + 0xcc), 0);
        // `fn_0045c520` with nothing held does nothing.
        e.call(0x0045_c520, &args![this]);
        assert_eq!(destroyed.borrow().len(), 2);
    }

    #[test]
    fn the_scalar_deleting_destructor_frees_with_bit_0() {
        let mut e = Engine::new();
        let destroyed = recording(&mut e, OBJECT_DESTRUCT, 0);
        let freed = recording(&mut e, OPERATOR_DELETE, 0);
        assert_eq!(e.call(0x0045_c5f0, &args![0x9000u32, 0u32]).u32(), 0x9000);
        assert!(freed.borrow().is_empty());
        e.call(0x0045_c5f0, &args![0x9000u32, 1u32]);
        assert_eq!(destroyed.borrow().len(), 2);
        assert_eq!(*freed.borrow(), vec![vec![0x9000]]);
    }

    /// A chain of nodes `[next, 0, item]` and a cell holding its head;
    /// returns the cell.
    fn chain_cell(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(12);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 8, *item);
            next = node;
        }
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, next);
        cell
    }

    /// The head node of a chain made like `chain_cell` (0 when empty).
    fn chain_head(e: &mut Engine, items: &[u32]) -> u32 {
        let cell = chain_cell(e, items);
        e.mem.u32(cell)
    }

    /// The doubles for walking a chain made by `chain_cell`.
    fn chain_doubles(e: &mut Engine) {
        e.register(LIST_NEXT_POSITION, |e, a| {
            ret(if a[1] != 0 { e.mem.u32(a[1]) } else { 0 })
        });
        e.register(LIST_ITEM_ADDRESS, |_, a| ret(a[1] + 8));
        e.register(LIST_ITERATE, |e, a| {
            let node = e.mem.u32(a[1]);
            let next = e.mem.u32(node);
            e.mem.set_u32(a[1], next);
            ret(node + 8)
        });
    }

    #[test]
    fn the_word_is_stored_on_the_linked_node_and_on_every_listed_item() {
        let mut e = Engine::new();
        ni_pointer_doubles(&mut e);
        chain_doubles(&mut e);
        e.register(OBJECT_FIRST_LINK, |e, a| ret(e.mem.u32(a[0] + 0xac)));
        e.register(OBJECT_SECOND_LINK, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        let stored = recording(&mut e, LINK_SET_NEXT, 0);
        // The node `[[this + 0xac] + 0xc]`, and the nodes `[item + 0xc]`.
        let own_node = e.mem.alloc(0x10);
        let own_holder = e.mem.alloc(0x20);
        e.mem.set_u32(own_holder + 0xc, own_node);
        let item_one = e.mem.alloc(0x20);
        e.mem.set_u32(item_one + 0xc, 0x5100);
        let item_two = e.mem.alloc(0x20);
        e.mem.set_u32(item_two + 0xc, 0x5200);
        let this = e.mem.alloc(0x110);
        e.mem.set_u32(this + 0xac, own_holder);
        let cell = chain_cell(&mut e, &[item_one, 0, item_two]);
        let head = e.mem.u32(cell);
        e.mem.set_u32(this + 0xd0, head);
        e.call(0x0045_c570, &args![this, 3u32]);
        assert_eq!(
            *stored.borrow(),
            vec![vec![own_node, 3], vec![0x5100, 3], vec![0x5200, 3]]
        );
    }

    // ----- 0045c620, 0045c680, 0045c6b0 -----

    #[test]
    fn the_frustum_planes_constructor_builds_six_planes() {
        let mut e = Engine::new();
        let built = recording(&mut e, ARRAY_CONSTRUCT, 0);
        let this = e.mem.alloc(0x70);
        assert_eq!(e.call(0x0045_c620, &args![this]).u32(), this);
        assert_eq!(
            *built.borrow(),
            vec![vec![this, 0x10, 6, FRUSTUM_PLANE_CONSTRUCT]]
        );
        assert_eq!(e.mem.u32(this + 0x60), 0x3f);
    }

    #[test]
    fn the_visibility_callback_picks_by_the_flag() {
        let mut e = Engine::new();
        let shown = recording(&mut e, RANGED_NODE_SHOWN, 0);
        let hidden = recording(&mut e, RANGED_NODE_HIDDEN, 0);
        e.call(0x0045_c680, &args![0x10u32, 1u8]);
        e.call(0x0045_c680, &args![0x20u32, 0u8]);
        assert_eq!(*shown.borrow(), vec![vec![0x10]]);
        assert_eq!(*hidden.borrow(), vec![vec![0x20]]);
    }

    #[test]
    fn a_form_type_gives_its_class() {
        let mut e = Engine::new();
        let form_type = Rc::new(RefCell::new(0u32));
        let inner = form_type.clone();
        e.register_double(FORM_GET_TYPE, move |_, _| ret(*inner.borrow()));
        for (value, class) in [
            (0x2a, 3),
            (0x2b, 3),
            (0x18, 2),
            (0x1d, 2),
            (0x32, 2),
            (0x74, 2),
            (0x1b, 1),
            (0x30, 1),
            (0x75, 1),
            (0x17, 1),
            (0, 1),
        ] {
            *form_type.borrow_mut() = value;
            assert_eq!(
                e.call(0x0045_c6b0, &args![0x1000u32]).u32(),
                class,
                "{value:#x}"
            );
        }
    }

    // ----- 0045c780, 0045c840, 0045c8d0, 0045c9a0, 0045c9e0 -----

    struct MarkWorld {
        e: Engine,
        tes: Ptr<TES>,
        applied: Log,
    }

    /// A 5x5 grid whose cells all have four quadrant nodes with an object
    /// (every marking call is logged as (target, operation, flag)).
    fn mark_world(grids: u32) -> MarkWorld {
        let mut e = Engine::new();
        map_globals(&mut e);
        ni_pointer_doubles(&mut e);
        put_setting(&mut e, SETTING_GRIDS_TO_LOAD, grids);
        e.register(GRID_SIDE, |_, _| ret(5));
        e.register(GRID_CELL_ARRAY_GET, |e, a| {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, 0x10_0000 + a[1] * 16 + a[2]);
            ret(slot)
        });
        e.register(CELL_GET_LIST, |_, a| ret(a[0] + 0x1000 * a[1]));
        let leaf_table = vtable(&mut e, &[(0x18, 0x7000_0018)]);
        let leaf = object_with(&mut e, leaf_table).addr();
        returns(&mut e, 0x7000_0018, leaf + 1);
        e.register_double(NODE_CHILD_AT, move |_, a| {
            // A cell list gives quadrant nodes; a node gives the leaf.
            ret(if a[0] < 0x20_0000 {
                0x30_0000 + a[1]
            } else {
                leaf
            })
        });
        returns(&mut e, CHILD_ARRAY_COUNT, 1);
        let present = e.mem.alloc(4);
        e.mem.set_u32(present, 1);
        returns(&mut e, CHILD_ARRAY_SLOT, present);
        returns(&mut e, MARKING_CONTEXT, 0xc0de);
        returns(&mut e, MARKING_TARGET, 0x7a76);
        let applied = recording(&mut e, MARKING_APPLY, 0);
        let tes: Ptr<TES> = e.new_object();
        MarkWorld { e, tes, applied }
    }

    fn put_table(e: &mut Engine, address: u32, flags: &[u8; 25]) {
        e.mem.write(address, flags);
    }

    fn pattern() -> [u8; 25] {
        let mut flags = [0u8; 25];
        for (index, flag) in flags.iter_mut().enumerate() {
            *flag = (index % 3 == 0) as u8;
        }
        flags
    }

    #[test]
    fn the_first_pass_marks_each_cell_by_the_table() {
        let mut w = mark_world(5);
        put_table(&mut w.e, GRID_FLAGS_FIRST, &pattern());
        w.e.call(0x0045_c780, &args![w.tes]);
        let applied = w.applied.borrow();
        assert_eq!(applied.len(), 100);
        for (cell, chunk) in applied.chunks(4).enumerate() {
            for call in chunk {
                assert_eq!(call[1..], [0x2e, pattern()[cell] as u32], "cell {cell}");
            }
        }
        assert_eq!(applied[0][..1], [0x7a76]);
    }

    #[test]
    fn the_first_pass_can_mark_only_the_centre_cell() {
        let mut w = mark_world(5);
        put_table(&mut w.e, GRID_FLAGS_FIRST, &[1; 25]);
        w.e.mem.set_u8(GRID_PASS_CENTRE_ONLY, 1);
        w.e.call(0x0045_c780, &args![w.tes]);
        let applied = w.applied.borrow();
        for (cell, chunk) in applied.chunks(4).enumerate() {
            for call in chunk {
                assert_eq!(call[2], (cell == 12) as u32, "cell {cell}");
            }
        }
    }

    #[test]
    fn the_passes_do_nothing_unless_five_grids_are_loaded() {
        let mut w = mark_world(3);
        w.e.call(0x0045_c780, &args![w.tes]);
        w.e.call(0x0045_c840, &args![w.tes]);
        assert!(w.applied.borrow().is_empty());
    }

    #[test]
    fn the_second_pass_marks_with_the_inverted_table() {
        let mut w = mark_world(5);
        put_table(&mut w.e, GRID_FLAGS_SECOND, &pattern());
        w.e.call(0x0045_c840, &args![w.tes]);
        let applied = w.applied.borrow();
        assert_eq!(applied.len(), 100);
        for (cell, chunk) in applied.chunks(4).enumerate() {
            for call in chunk {
                assert_eq!(
                    call[1..],
                    [0x34, (pattern()[cell] == 0) as u32],
                    "cell {cell}"
                );
            }
        }
    }

    #[test]
    fn marking_a_cell_stops_at_what_is_missing() {
        let mut w = mark_world(5);
        // A missing slot, then a slot whose cell is null.
        w.e.register(GRID_CELL_ARRAY_GET, |e, a| {
            let slot = e.mem.alloc(4);
            if a[1] != 0 {
                e.mem.set_u32(slot, 0x10_0000);
            }
            ret(if a[2] == 9 { 0 } else { slot })
        });
        w.e.call(0x0045_c8d0, &args![w.tes, 9u32, 9u32, 1u8]);
        w.e.call(0x0045_c8d0, &args![w.tes, 0u32, 0u32, 1u8]);
        // A complete cell gives four markings; then a node without a first child
        // and a first child without an object give none.
        w.e.call(0x0045_c8d0, &args![w.tes, 1u32, 0u32, 1u8]);
        assert_eq!(w.applied.borrow().len(), 4);
        returns(&mut w.e, CHILD_ARRAY_COUNT, 0);
        w.e.call(0x0045_c9e0, &args![w.tes, 1u32, 0u32, 1u8]);
        assert_eq!(w.applied.borrow().len(), 4);
        returns(&mut w.e, CHILD_ARRAY_COUNT, 1);
        returns(&mut w.e, 0x7000_0018, 0);
        w.e.call(0x0045_c8d0, &args![w.tes, 1u32, 0u32, 1u8]);
        assert_eq!(w.applied.borrow().len(), 4);
    }

    #[test]
    fn a_quadrant_comes_from_the_second_cell_list() {
        let mut e = Engine::new();
        let asked = recording(&mut e, CELL_GET_LIST, 0);
        let cell = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0045_c9a0, &args![cell, 3u32]).u32(), 0);
        assert_eq!(*asked.borrow(), vec![vec![cell, 2]]);
        returns(&mut e, CELL_GET_LIST, 0x6000);
        let node = recording(&mut e, NODE_CHILD_AT, 0x6100);
        assert_eq!(e.call(0x0045_c9a0, &args![cell, 3u32]).u32(), 0x6100);
        assert_eq!(*node.borrow(), vec![vec![0x6000, 3]]);
    }

    // ----- 0045cac0 -----

    #[test]
    fn distant_blending_refreshes_the_cells_that_accept_it() {
        let mut e = Engine::new();
        map_globals(&mut e);
        put_setting(&mut e, SETTING_GRIDS_TO_LOAD, 2);
        // Cell (1, 0) is not loaded; the others are cells 100 + 10x + y.
        e.register(GRID_CELL_ARRAY_GET, |e, a| {
            let slot = e.mem.alloc(4);
            if (a[1], a[2]) != (1, 0) {
                e.mem.set_u32(slot, 100 + 10 * a[1] + a[2]);
            }
            ret(slot)
        });
        // Cell 100 has no scene object; 101 one the test refuses.
        e.register(CELL_SCENE_OBJECT, |_, a| {
            ret(if a[0] == 100 { 0 } else { a[0] + 1000 })
        });
        e.register(
            SCENE_OBJECT_NEEDS_REFRESH,
            |_, a| ret((a[0] == 1111) as u32),
        );
        let refreshed = recording(&mut e, SCENE_OBJECT_REFRESH, 0);
        let tes: Ptr<TES> = e.new_object();
        e.call(0x0045_cac0, &args![tes]);
        assert_eq!(*refreshed.borrow(), vec![vec![1111]]);
    }

    // ----- 0045b070, 0045bc80 -----

    /// Vtable slots of the scene objects, all reading a field of the object:
    /// `+0x50` is what slot `0x104` and `0x108` return, `+0x54` slot `0xC`,
    /// `+0x58` slot `0x1D0`, `+0x5C` slot 8.
    fn scene_vtable(e: &mut Engine) -> u32 {
        e.register(0x7100_0104, |e, a| ret(e.mem.u32(a[0] + 0x50)));
        e.register(0x7100_0108, |e, a| ret(e.mem.u32(a[0] + 0x50)));
        e.register(0x7100_000c, |e, a| ret(e.mem.u32(a[0] + 0x54)));
        e.register(0x7100_01d0, |e, a| ret(e.mem.u32(a[0] + 0x58)));
        e.register(0x7100_0008, |e, a| ret(e.mem.u32(a[0] + 0x5c)));
        vtable(
            e,
            &[
                (0x104, 0x7100_0104),
                (0x108, 0x7100_0108),
                (0xc, 0x7100_000c),
                (0x1d0, 0x7100_01d0),
                (8, 0x7100_0008),
                (0xbc, 0x7100_00bc),
            ],
        )
    }

    type Lists = Rc<RefCell<std::collections::HashMap<u32, Vec<u32>>>>;

    struct SceneWorld {
        e: Engine,
        tes: Ptr<TES>,
        scene: u32,
        camera: u32,
        table: u32,
        lists: Lists,
        cell_lists: Rc<RefCell<std::collections::HashMap<(u32, u32), u32>>>,
        grid_cells: Rc<RefCell<std::collections::HashMap<(u32, u32), u32>>>,
        added: Log,
        removed: Log,
        shown_to_slot_bc: Log,
        water: Rc<std::cell::Cell<u32>>,
        state: Rc<std::cell::Cell<u32>>,
    }

    impl SceneWorld {
        fn new() -> SceneWorld {
            let mut e = Engine::new();
            map_globals(&mut e);
            ni_pointer_doubles(&mut e);
            chain_doubles(&mut e);
            returns(&mut e, CHILD_ARRAY_COUNT, 0);
            returns(&mut e, CHILD_ARRAY_SLOT, 0);
            put_setting(&mut e, SETTING_GRIDS_TO_LOAD, 2);
            let table = scene_vtable(&mut e);
            let scene = e.mem.alloc(0x200);
            returns(&mut e, SHADOW_SCENE_NODE_GETTER, scene);
            let camera = e.mem.alloc(0x200);
            for (index, value) in [10.0f32, 20.0, 30.0].iter().enumerate() {
                e.mem.set_f32(camera + 0x8c + 4 * index as u32, *value);
            }
            e.mem.set_f32(camera + 0xdc + 0x10, 2.0);
            noop(
                &mut e,
                &[
                    SCENE_WALK_MARKER_CALL,
                    CULLING_PROCESS_CONSTRUCT,
                    CULLING_PROCESS_SET_CAMERA,
                    CULLING_PROCESS_SET_FRUSTUM,
                    CULLING_PROCESS_DESTRUCT,
                    FORM_HAS_BUFFER_FORM,
                ],
            );
            let lists: Lists = Rc::new(RefCell::new(Default::default()));
            let inner = lists.clone();
            e.register_double(NODE_CHILD_COUNT, move |_, a| {
                ret(inner
                    .borrow()
                    .get(&a[0])
                    .map_or(0, |list| list.len() as u32))
            });
            let inner = lists.clone();
            e.register_double(NODE_CHILD_AT, move |_, a| {
                ret(inner.borrow()[&a[0]][a[1] as usize])
            });
            let cell_lists: Rc<RefCell<std::collections::HashMap<(u32, u32), u32>>> =
                Rc::new(RefCell::new(Default::default()));
            let inner = cell_lists.clone();
            e.register_double(CELL_GET_LIST, move |_, a| {
                ret(inner.borrow().get(&(a[0], a[1])).copied().unwrap_or(0))
            });
            let grid_cells: Rc<RefCell<std::collections::HashMap<(u32, u32), u32>>> =
                Rc::new(RefCell::new(Default::default()));
            let inner = grid_cells.clone();
            e.register_double(GRID_CELL_ARRAY_GET, move |e, a| {
                let slot = e.mem.alloc(4);
                let cell = inner.borrow().get(&(a[1], a[2])).copied().unwrap_or(0);
                e.mem.set_u32(slot, cell);
                ret(slot)
            });
            let water = Rc::new(std::cell::Cell::new(0));
            let inner = water.clone();
            e.register_double(TES_WATER_SYSTEM_GETTER, move |_, _| ret(inner.get()));
            let state = Rc::new(std::cell::Cell::new(0));
            let inner = state.clone();
            e.register_double(SCENE_NODE_STATE, move |_, _| ret(inner.get()));
            // The links `scene_remove_object` follows: the node is object + 2.
            e.register(OBJECT_FIRST_LINK, |_, a| ret(a[0] + 1));
            e.register(OBJECT_SECOND_LINK, |_, a| ret(a[0] + 1));
            let removed = recording(&mut e, NODE_REMOVE, 0);
            let added = recording(&mut e, SCENE_ADD_OBJECT, 0);
            let shown_to_slot_bc = recording(&mut e, 0x7100_00bc, 0);
            let tes: Ptr<TES> = e.new_object();
            SceneWorld {
                e,
                tes,
                scene,
                camera,
                table,
                lists,
                cell_lists,
                grid_cells,
                added,
                removed,
                shown_to_slot_bc,
                water,
                state,
            }
        }

        /// A scene object; `flag` is what slots 0x104 and 0x108 return.
        fn object(&mut self, flag: u32) -> u32 {
            let object = self.e.mem.alloc(0x100);
            self.e.mem.set_u32(object, self.table);
            self.e.mem.set_u32(object + 0x50, flag);
            object
        }

        /// A child list with the given entries; returns its id.
        fn list(&mut self, entries: &[u32]) -> u32 {
            let id = self.e.mem.alloc(4);
            self.lists.borrow_mut().insert(id, entries.to_vec());
            id
        }

        fn add_args(&self) -> Vec<Vec<u32>> {
            self.added.borrow().clone()
        }

        /// The objects `SCENE_ADD_OBJECT` was called with, in order.
        fn added_objects(&self) -> Vec<u32> {
            self.added.borrow().iter().map(|call| call[1]).collect()
        }

        /// The nodes `NODE_REMOVE` was called with, in order, as objects.
        fn removed_objects(&self) -> Vec<u32> {
            self.removed
                .borrow()
                .iter()
                .map(|call| call[0] - 2)
                .collect()
        }
    }

    #[test]
    fn the_add_walk_takes_interior_objects_that_slot_104_accepts() {
        let mut w = SceneWorld::new();
        let accepted = w.object(0);
        let refused = w.object(1);
        let list = w.list(&[accepted, 0, refused]);
        w.cell_lists.borrow_mut().insert((0xce11, 7), list);
        w.e.set(w.tes, TES::pInteriorCell, Ptr::new(0xce11));
        w.e.call_log = Some(vec![]);
        w.e.call(0x0045_b070, &args![w.tes, w.camera]);
        let log = w.e.call_log.take().unwrap();
        assert_eq!(w.added_objects(), vec![accepted]);
        // (scene, object, x, y, z, process) with the camera position.
        let call = &w.add_args()[0];
        assert_eq!(call[0], w.scene);
        assert_eq!(
            call[2..5],
            [10.0f32.to_bits(), 20.0f32.to_bits(), 30.0f32.to_bits()]
        );
        // The empty marker function is called before and after, and the
        // culling process is set to the camera and destroyed at the end.
        let markers = calls_to(&log, SCENE_WALK_MARKER_CALL);
        assert_eq!(
            markers,
            vec![vec![SCENE_WALK_MARKER], vec![SCENE_WALK_MARKER]]
        );
        assert_eq!(calls_to(&log, CULLING_PROCESS_SET_CAMERA)[0][1], w.camera);
        assert_eq!(
            calls_to(&log, CULLING_PROCESS_SET_FRUSTUM)[0][1],
            w.camera + 0xdc
        );
        assert_eq!(
            calls_to(&log, CULLING_PROCESS_DESTRUCT),
            vec![vec![call[5]]]
        );
        // No terrain work indoors.
        assert!(calls_to(&log, GET_WORLD_SPACE).is_empty());
    }

    #[test]
    fn the_add_walk_goes_through_the_loaded_grid_cells_outdoors() {
        let mut w = SceneWorld::new();
        let (p, q, r, s) = (w.object(0), w.object(0), w.object(0), w.object(0));
        let seven = w.list(&[p]);
        let two_a = w.list(&[q]);
        let two_b = w.list(&[r, s]);
        {
            let mut cells = w.cell_lists.borrow_mut();
            cells.insert((0xc1, 7), seven);
            cells.insert((0xc1, 2), two_a);
            // Cell 0xc2 has no list 7.
            cells.insert((0xc2, 2), two_b);
        }
        let nothing = w.list(&[]);
        w.cell_lists.borrow_mut().insert((0xc3, 2), nothing);
        {
            let mut grid = w.grid_cells.borrow_mut();
            grid.insert((0, 0), 0xc1);
            grid.insert((1, 0), 0xc2);
            grid.insert((1, 1), 0xc3);
        }
        returns(&mut w.e, GET_WORLD_SPACE, 0x77);
        returns(&mut w.e, WORLD_SPACE_TERRAIN, 0x88);
        let terrain = recording(&mut w.e, TERRAIN_PREPARE_FOR_CAMERA, 0);
        w.e.call(0x0045_b070, &args![w.tes, w.camera]);
        assert_eq!(w.added_objects(), vec![p, q, r, s]);
        assert_eq!(*terrain.borrow(), vec![vec![0x88, w.camera]]);
        // Without a world space there is no terrain call.
        returns(&mut w.e, GET_WORLD_SPACE, 0);
        w.e.call(0x0045_b070, &args![w.tes, w.camera]);
        assert_eq!(terrain.borrow().len(), 1);
        assert_eq!(w.added.borrow().len(), 8);
    }

    #[test]
    fn the_add_walk_also_takes_the_scene_children_and_the_water_entries() {
        let mut w = SceneWorld::new();
        w.e.set(w.tes, TES::pInteriorCell, Ptr::new(0xce11));
        // Child 6 of the scene node, whose slot 0xC gives a list of two.
        let (m, n) = (w.object(0), w.object(0));
        let children = w.list(&[m, 0, n]);
        let child = w.object(0);
        w.e.mem.set_u32(child + 0x54, children);
        let cell = w.e.mem.alloc(4);
        w.e.mem.set_u32(cell, child);
        returns(&mut w.e, CHILD_ARRAY_COUNT, 7);
        returns(&mut w.e, CHILD_ARRAY_SLOT, cell);
        // The water system: one holder with an entry whose object has the
        // right type, then a second holder whose entry has another type.
        let accepted = w.object(0);
        w.e.mem.set_u32(accepted + 0x5c, WATER_ENTRY_RTTI);
        let refused = w.object(0);
        w.e.mem.set_u32(refused + 0x5c, WATER_ENTRY_RTTI + 4);
        let good_objects = w.list(&[accepted]);
        let bad_objects = w.list(&[refused]);
        let good_entry = w.object(0);
        w.e.mem.set_u32(good_entry + 0x58, good_objects);
        let bad_entry = w.object(0);
        w.e.mem.set_u32(bad_entry + 0x58, bad_objects);
        let first_holder = w.e.mem.alloc(0x40);
        let head = chain_head(&mut w.e, &[good_entry]);
        w.e.mem.set_u32(first_holder + 0x24, head);
        let second_holder = w.e.mem.alloc(0x40);
        let head = chain_head(&mut w.e, &[bad_entry]);
        w.e.mem.set_u32(second_holder + 0x24, head);
        let system = w.e.mem.alloc(0x40);
        w.water.set(system);
        let holders = chain_cell(&mut w.e, &[first_holder]);
        returns(&mut w.e, WATER_SYSTEM_LIST, holders);
        returns(&mut w.e, WATER_SYSTEM_SECOND_HOLDER, second_holder);
        let reviewed = recording(&mut w.e, FORM_HAS_BUFFER_FORM, 0);
        w.e.call(0x0045_b070, &args![w.tes, w.camera]);
        assert_eq!(w.added_objects(), vec![m, n, accepted]);
        // `007af430` was called on both entries.
        assert_eq!(*reviewed.borrow(), vec![vec![good_entry], vec![bad_entry]]);
    }

    /// The state-object half of the add walk: the held object, the
    /// candidates, the point in front of the camera.
    type Asked = Rc<RefCell<Vec<(u32, [f32; 3])>>>;

    struct HeldWorld {
        w: SceneWorld,
        candidates: Log,
        asked: Asked,
        state: u32,
    }

    fn held_world() -> HeldWorld {
        let mut w = SceneWorld::new();
        w.e.set(w.tes, TES::pInteriorCell, Ptr::new(0xce11));
        let state = w.e.mem.alloc(0x40);
        w.state.set(state);
        returns(&mut w.e, SCENE_STATE_HELD, 0);
        let queue_cell = chain_cell(&mut w.e, &[]);
        returns(&mut w.e, SCENE_STATE_QUEUE, queue_cell);
        returns(&mut w.e, SCENE_STATE_QUEUE_IS_EMPTY, 1);
        // The point is the position plus the first column times near (2).
        w.e.register(VECTOR_CTOR, |_, a| ret(a[0]));
        w.e.register(MATRIX_GET_COLUMN, |e, a| {
            for (index, value) in [1.0f32, 0.0, 0.0].iter().enumerate() {
                e.mem.set_f32(a[2] + 4 * index as u32, *value);
            }
            Ret::default()
        });
        w.e.register(VECTOR3_CONSTRUCT, |e, a| {
            for word in 0..3 {
                e.mem.set_u32(a[0] + 4 * word, a[1 + word as usize]);
            }
            ret(a[0])
        });
        w.e.register(POINT_ADD, |e, a| {
            for word in 0..3 {
                let sum = e.mem.f32(a[0] + 4 * word) + e.mem.f32(a[2] + 4 * word);
                e.mem.set_f32(a[1] + 4 * word, sum);
            }
            ret(a[1])
        });
        w.e.register(CULLING_PROCESS_MEMBER, |_, a| ret(a[0] + 0x2c));
        returns(&mut w.e, CULLING_PROCESS_WORD, 0x3133);
        let candidates = recording(&mut w.e, SCENE_ADD_CANDIDATE, 0);
        // Slot 0x108 records the point it is asked about.
        let asked = Rc::new(RefCell::new(vec![]));
        let inner = asked.clone();
        w.e.register_double(0x7100_0108, move |e, a| {
            let point = [0, 4, 8].map(|offset| e.mem.f32(a[1] + offset));
            inner.borrow_mut().push((a[0], point));
            ret(e.mem.u32(a[0] + 0x50))
        });
        noop(&mut w.e, &[LIST_CLEAR]);
        HeldWorld {
            w,
            candidates,
            asked,
            state,
        }
    }

    fn stored_state_pointer(h: &HeldWorld) -> u32 {
        h.w.e.mem.u32(h.state + 0x2c)
    }

    #[test]
    fn a_held_object_that_accepts_the_point_is_offered_and_kept() {
        let mut h = held_world();
        let held = h.w.object(1);
        returns(&mut h.w.e, SCENE_STATE_HELD, held);
        let cleared = recording(&mut h.w.e, LIST_CLEAR, 0);
        h.w.e.call(0x0045_b070, &args![h.w.tes, h.w.camera]);
        // The tracked list was cleared first.
        assert_eq!(*cleared.borrow(), vec![vec![h.state + 0x30]]);
        // The point: position (10, 20, 30) + column (1, 0, 0) * near (2).
        assert_eq!(*h.asked.borrow(), vec![(held, [12.0, 20.0, 30.0])]);
        let candidates = h.candidates.borrow();
        assert_eq!(candidates.len(), 1);
        // (scene, object, process word, process member, flag 1).
        assert_eq!(candidates[0][..3], [h.w.scene, held, 0x3133]);
        assert_eq!(candidates[0][4], 1);
        assert_eq!(stored_state_pointer(&h), held);
    }

    #[test]
    fn a_held_object_that_refuses_the_point_is_dropped_for_the_queue() {
        let mut h = held_world();
        let held = h.w.object(0);
        let (first, second, third) = (h.w.object(0), h.w.object(1), h.w.object(1));
        returns(&mut h.w.e, SCENE_STATE_HELD, held);
        let queue = chain_cell(&mut h.w.e, &[0, first, second, third]);
        returns(&mut h.w.e, SCENE_STATE_QUEUE, queue);
        returns(&mut h.w.e, SCENE_STATE_QUEUE_IS_EMPTY, 0);
        h.w.e.call(0x0045_b070, &args![h.w.tes, h.w.camera]);
        // The held object, the null entry skipped, then the queue's
        // refusing entry; the accepting ones are offered, the first with flag
        // 1 and filling the cell, the next with flag 0.
        let asked: Vec<u32> = h.asked.borrow().iter().map(|call| call.0).collect();
        assert_eq!(asked, vec![held, first, second, third]);
        let candidates = h.candidates.borrow();
        let offered: Vec<(u32, u32)> = candidates.iter().map(|call| (call[1], call[4])).collect();
        assert_eq!(offered, vec![(second, 1), (third, 0)]);
        assert_eq!(stored_state_pointer(&h), second);
    }

    #[test]
    fn with_nothing_to_hold_a_null_candidate_is_offered() {
        let mut h = held_world();
        h.w.e.call(0x0045_b070, &args![h.w.tes, h.w.camera]);
        let candidates = h.candidates.borrow();
        assert_eq!(candidates.len(), 1);
        assert_eq!((candidates[0][1], candidates[0][4]), (0, 1));
        assert!(h.asked.borrow().is_empty());
        assert_eq!(stored_state_pointer(&h), 0);
    }

    // ----- 0045bc80 -----

    #[test]
    fn the_removal_walk_takes_out_interior_objects_that_slot_104_accepts() {
        let mut w = SceneWorld::new();
        let accepted = w.object(0);
        let refused = w.object(1);
        let list = w.list(&[accepted, refused]);
        w.cell_lists.borrow_mut().insert((0xce11, 7), list);
        w.e.set(w.tes, TES::pInteriorCell, Ptr::new(0xce11));
        let terrain = recording(&mut w.e, TERRAIN_PREPARE_FOR_REMOVAL, 0);
        w.e.call(0x0045_bc80, &args![w.tes, 1u8, 1u8]);
        assert_eq!(w.removed_objects(), vec![accepted]);
        assert!(terrain.borrow().is_empty());
        assert!(w.shown_to_slot_bc.borrow().is_empty());
    }

    #[test]
    fn the_removal_walk_outdoors_calls_slot_bc_and_optionally_removes_list_2() {
        for with_lists in [0u8, 1] {
            let mut w = SceneWorld::new();
            let (p, q, r) = (w.object(0), w.object(0), w.object(0));
            let seven = w.list(&[p]);
            let two = w.list(&[q, r]);
            w.cell_lists.borrow_mut().insert((0xc1, 7), seven);
            w.cell_lists.borrow_mut().insert((0xc1, 2), two);
            w.grid_cells.borrow_mut().insert((0, 0), 0xc1);
            returns(&mut w.e, GET_WORLD_SPACE, 0x77);
            returns(&mut w.e, WORLD_SPACE_TERRAIN, 0x88);
            let terrain = recording(&mut w.e, TERRAIN_PREPARE_FOR_REMOVAL, 0);
            w.e.call(0x0045_bc80, &args![w.tes, with_lists, 0u8]);
            assert_eq!(*terrain.borrow(), vec![vec![0x88]]);
            // Slot 0xBC is called for the list 7 object only.
            assert_eq!(*w.shown_to_slot_bc.borrow(), vec![vec![p]]);
            let expected = if with_lists == 0 {
                vec![p]
            } else {
                vec![p, q, r]
            };
            assert_eq!(w.removed_objects(), expected, "{with_lists}");
        }
    }

    #[test]
    fn the_removal_walk_takes_out_the_scene_children_and_the_water_entries() {
        let mut w = SceneWorld::new();
        w.e.set(w.tes, TES::pInteriorCell, Ptr::new(0xce11));
        let (m, n) = (w.object(0), w.object(0));
        let children = w.list(&[m, n]);
        let child = w.object(0);
        w.e.mem.set_u32(child + 0x54, children);
        let cell = w.e.mem.alloc(4);
        w.e.mem.set_u32(cell, child);
        returns(&mut w.e, CHILD_ARRAY_COUNT, 7);
        returns(&mut w.e, CHILD_ARRAY_SLOT, cell);
        let accepted = w.object(0);
        w.e.mem.set_u32(accepted + 0x5c, WATER_ENTRY_RTTI);
        let objects = w.list(&[accepted]);
        let entry = w.object(0);
        w.e.mem.set_u32(entry + 0x58, objects);
        // The entry is under the water system's second holder only.
        let second_holder = w.e.mem.alloc(0x40);
        let head = chain_head(&mut w.e, &[entry]);
        w.e.mem.set_u32(second_holder + 0x24, head);
        let system = w.e.mem.alloc(0x40);
        w.water.set(system);
        let no_holders = chain_cell(&mut w.e, &[]);
        returns(&mut w.e, WATER_SYSTEM_LIST, no_holders);
        returns(&mut w.e, WATER_SYSTEM_SECOND_HOLDER, second_holder);
        w.e.call(0x0045_bc80, &args![w.tes, 0u8, 0u8]);
        assert_eq!(w.removed_objects(), vec![m, n, accepted]);
    }

    #[test]
    fn the_removal_walk_resets_the_tracked_items_and_releases_queued_objects() {
        let mut w = SceneWorld::new();
        w.e.set(w.tes, TES::pInteriorCell, Ptr::new(0xce11));
        let state = w.e.mem.alloc(0x40);
        w.state.set(state);
        // Tracked items, each with a word at +0x40 the walk clears.
        let tracked: Vec<u32> = (0..2).map(|_| w.e.mem.alloc(0x120)).collect();
        for item in &tracked {
            w.e.mem.set_u32(item + 0x40, 0xaa);
        }
        let tracked_list = chain_cell(&mut w.e, &[tracked[0], 0, tracked[1]]);
        returns(&mut w.e, SCENE_STATE_LIST, tracked_list);
        // One queued object with two items in its first list (one with a
        // binding target) and one in its second.
        let queued = w.object(0);
        w.e.mem.set_u32(queued + 0xcc, 0);
        let queue = chain_cell(&mut w.e, &[queued]);
        returns(&mut w.e, SCENE_STATE_QUEUE, queue);
        returns(&mut w.e, SCENE_STATE_QUEUE_IS_EMPTY, 0);
        w.e.register(NI_POINTER_COPY_CONSTRUCT, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            ret(a[0])
        });
        let with_target = w.e.mem.alloc(0x120);
        let without_target = w.e.mem.alloc(0x120);
        let second_item = w.e.mem.alloc(0x120);
        for item in [with_target, without_target, second_item] {
            w.e.mem.set_u32(item + 0x40, 0xaa);
        }
        w.e.mem.set_u32(with_target + 0x104, 0x7a00);
        w.e.mem.set_u32(without_target + 0x104, 0x7b00);
        let first_list = chain_cell(&mut w.e, &[with_target, without_target]);
        let second_list = chain_cell(&mut w.e, &[second_item]);
        returns(&mut w.e, OBJECT_ITEMS_A, first_list);
        returns(&mut w.e, OBJECT_ITEMS_B, second_list);
        w.e.register(ITEM_GET_TARGET_OBJECT, |_, a| {
            ret((a[0] == 0x7a00) as u32 * 0x9a00)
        });
        let released = recording(&mut w.e, OBJECT_RELEASE, 0);
        let stored = recording(&mut w.e, LINK_SET_NEXT, 0);
        // `with_queue` off: only the tracked items are reset.
        w.e.call(0x0045_bc80, &args![w.tes, 0u8, 0u8]);
        assert_eq!(
            tracked
                .iter()
                .map(|item| w.e.mem.u32(item + 0x40))
                .collect::<Vec<_>>(),
            vec![0, 0]
        );
        assert_eq!(w.e.mem.u32(with_target + 0x40), 0xaa);
        assert!(released.borrow().is_empty());
        // `with_queue` on: the queued object is reset and its items released.
        w.e.call(0x0045_bc80, &args![w.tes, 0u8, 1u8]);
        assert_eq!(w.shown_to_slot_bc.borrow().clone(), vec![vec![queued]]);
        assert_eq!(*released.borrow(), vec![vec![0x9a00, 0]]);
        for item in [with_target, without_target, second_item] {
            assert_eq!(w.e.mem.u32(item + 0x40), 0, "{item:#x}");
        }
        // The queued object's link node got the value 3 (`fn_0045c570`).
        assert_eq!(stored.borrow()[0], vec![queued + 2, 3]);
        assert_eq!(w.e.mem.u32(queued + 0xcc), 0);
    }
}
