//! `fallout shared/gridcell.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `GridCellArray` is the square grid of exterior cells around the player
//! (`GridArray` plus the cell slots at `+0x10`, the world position of the
//! centre cell and the canopy-shadow texture). The slot of cell `(x, y)` is
//! at `slots + (dimension * x + y) * 4`; `dimension` is read through the
//! getter at `0084e3a0` (`this + 0xc`), once per use as the code does.
//!
//! The grid walks that apply a cell function to every filled slot all have
//! the same shape: `x` outer, `y` inner, both below the dimension.

#[allow(unused_imports)]
use crate::prelude::*;

/// Getter for the grid's `iDimension` (`this + 0xc`; one function shared by
/// several classes, so the map has no name for it).
const GRID_DIMENSION: u32 = 0x0084_e3a0;
/// `GridArray` destructor body: writes the `GridArray` vtable (`01020634`).
const GRID_ARRAY_DESTRUCTOR_BODY: u32 = 0x004b_9e50;
/// `GridArray::Init` (Xbox PDB).
const GRID_ARRAY_INIT: u32 = 0x004b_9e70;
/// Calls the virtual `DetachAll` (vtable slot `+0x8`) on `ECX`.
const GRID_ARRAY_DETACH_ALL_CALL: u32 = 0x004b_9e90;
/// `GridArray::SetCenter` (Xbox PDB): shifts the grid; true when it moved.
const GRID_ARRAY_SET_CENTER: u32 = 0x004b_9f10;
/// Vtable of `GridCellArray`.
const GRID_CELL_ARRAY_VTABLE: u32 = 0x0102_0698;
/// Vtable offset of the virtual `ClearItem` (`GridCellArray::ClearItem`).
const SLOT_CLEAR_ITEM: u32 = 0x1c;
/// Source file name string passed to the scope timer.
const GRID_CELL_SOURCE_FILE_NAME: u32 = 0x0102_06c0;
/// `cdecl`: frees a block with the game's allocator.
const MEMORY_FREE: u32 = 0x0040_1030;
/// Releases the reference held by the pointer at `ECX`. The map matches it
/// to `Concurrency::cancellation_token_source::~cancellation_token_source`
/// (a library match); the body is "if the pointer is set, release it".
const SMART_POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
/// `NiPointer` assignment: `ECX` = the pointer slot, one stack word.
const SMART_POINTER_SET: u32 = 0x0066_b0d0;
/// Returns the pointer stored at `ECX` (`*this`).
const POINTER_VALUE: u32 = 0x0055_9450;
/// `cdecl`, no arguments: the pointer stored at `011ca0d8`.
const SINGLETON_GET: u32 = 0x0045_1010;
/// `cdecl`, one argument: assigns the pointer stored at `011f95e8`.
const SINGLETON_SET: u32 = 0x0045_1570;
/// `cdecl`, no arguments: returns the global at `011f91a8` (the texture
/// manager that `BSTextureManager::ReturnRenderedTexture` belongs to).
const TEXTURE_MANAGER_GET: u32 = 0x004a_0ea0;
/// `BSTextureManager::ReturnRenderedTexture` (Xbox PDB): `ECX` = manager,
/// one stack word (the texture).
const RETURN_RENDERED_TEXTURE: u32 = 0x00b6_da10;
/// Scope timer constructor (`ECX` = timer; 4 stack words) and destructor.
const SCOPE_TIMER_CONSTRUCT: u32 = 0x0040_4eb0;
const SCOPE_TIMER_DESTROY: u32 = 0x0040_4ee0;
/// `TESObjectCELL::InitStatics` (Xbox PDB).
const CELL_INIT_STATICS: u32 = 0x0054_1b80;
/// Cell statics teardown, called by `KillAll` when the byte at `+0x61d` of
/// the object in `011c3f2c` is set.
const CELL_STATICS_TEARDOWN: u32 = 0x0054_1c00;
/// Returns the byte at `+0x61d` of `ECX`.
const READ_FLAG_61D: u32 = 0x0042_26e0;
/// Global holding the object whose flag at `+0x61d` `KillAll` tests.
const KILL_ALL_FLAG_OWNER: u32 = 0x011c_3f2c;
/// `TESObjectCELL::MoveExteriorWorld` (Xbox PDB): `cdecl`, one stack word
/// (the world centre).
const CELL_MOVE_EXTERIOR_WORLD: u32 = 0x0055_46a0;
/// `TESObjectCELL::Detach` (Xbox PDB): one stack word.
const CELL_DETACH: u32 = 0x0055_2bd0;
/// `TESObjectCELL::AttachToWorld` (Xbox PDB): one stack word (the root).
const CELL_ATTACH_TO_WORLD: u32 = 0x0055_2970;
/// `TESObjectCELL::GetLand` (Xbox PDB).
const CELL_GET_LAND: u32 = 0x0054_6fb0;
/// Land function (`tesland.cpp`) run by `fn_004ba7d0` on a cell's land.
const LAND_FUNCTION_5374F0: u32 = 0x0053_74f0;
/// Cell function (`tesobjectcell.cpp`) run by `fn_004ba7d0`.
const CELL_FUNCTION_5535F0: u32 = 0x0055_35f0;
/// `TESObjectLAND::InitDistantTextureBlending` (Xbox PDB).
const LAND_INIT_DISTANT_TEXTURE_BLENDING: u32 = 0x0054_00e0;
/// `cdecl`, one float word, no `this` (`tesobjectcell.cpp`): runs a virtual
/// (slot `+0xc4`) on the pointer stored at `011ca0d8` when it is set.
const FLOAT_HOOK_554780: u32 = 0x0055_4780;
/// Cell function with a float and a bool argument (`tesobjectcell.cpp`).
const CELL_FUNCTION_551890: u32 = 0x0055_1890;
/// Cell function (`tesobjectcell.cpp`), no arguments.
const CELL_FUNCTION_552840: u32 = 0x0055_2840;
/// Cell function (`tesobjectcell.cpp`), no arguments.
const CELL_FUNCTION_553820: u32 = 0x0055_3820;
/// `TESObjectCELL::UpdateRefSounds` (Xbox PDB).
const CELL_UPDATE_REF_SOUNDS: u32 = 0x0055_38e0;
/// Cell test: true when the byte at `+0x26` of the cell is 6.
const CELL_IS_TYPE_6: u32 = 0x0045_0ff0;
/// Land function (`tesland.cpp`) with one byte argument.
const LAND_FUNCTION_53FA40: u32 = 0x0053_fa40;
/// Cell function (`tesobjectcell.cpp`) with one bool argument.
const CELL_FUNCTION_554960: u32 = 0x0055_4960;
/// Cell function (`tesobjectcell.cpp`) with one byte argument.
const CELL_FUNCTION_5549D0: u32 = 0x0055_49d0;
/// Toggled by `fn_004ba9a0`: which parity of border cells is selected.
const BORDER_TOGGLE_A: u32 = 0x011c_642c;
/// Toggled by `fn_004baab0`.
const BORDER_TOGGLE_B: u32 = 0x011c_642d;
/// `GridCellArray::bRecreateCanopyShadowMaskTexture` (Xbox PDB, static).
const RECREATE_CANOPY_SHADOW_MASK_TEXTURE: u32 = 0x0118_9184;

layout! {
    /// `GridCellArray` (Xbox PDB), 0x28 bytes (`GridArray` is the first 0x10).
    pub struct GridCellArray: 0x28 {
        /// The vtable (`01020698`).
        0x00 vfptr: u32,
        /// `GridArray::iCenterX` (Xbox PDB).
        0x04 iCenterX: i32,
        /// `GridArray::iCenterY` (Xbox PDB).
        0x08 iCenterY: i32,
        /// `GridArray::iDimension` (Xbox PDB).
        0x0C iDimension: u32,
        /// `pGridCellA` (Xbox PDB): the cell slots, `dimension * dimension`
        /// pointers.
        0x10 pGridCellA: Ptr,
        /// `WorldCenter.x` (Xbox PDB: `NiPoint3`).
        0x14 WorldCenterX: f32,
        /// `WorldCenter.y`.
        0x18 WorldCenterY: f32,
        /// `WorldCenter.z`.
        0x1C WorldCenterZ: f32,
        /// `bLandAttached` (Xbox PDB).
        0x20 bLandAttached: bool,
        /// `spTexRendered` (Xbox PDB): `NiPointer<BSRenderedTexture>`.
        0x24 spTexRendered: Ptr,
    }
}

fn dimension(e: &mut Engine, this: Ptr<GridCellArray>) -> u32 {
    e.call(GRID_DIMENSION, &args![this]).u32()
}

/// The cell pointer stored in the slot at `slot`.
fn cell_in(e: &Engine, slot: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(slot.addr()))
}

// Translated from 004ba3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridCellArray::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and, when bit 0 of `flags` is set, frees the object. Returns
/// `this`.
pub fn grid_cell_array_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<GridCellArray>,
    flags: u32,
) -> Ptr<GridCellArray> {
    fn_004ba400(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 004ba400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ba400 - the compiler's exception-unwinding frame (FS:[0] handler, state table 00ef0ef3) is not translated
/// The destructor body that the scalar deleting destructor calls (the map
/// gives it no name): writes the vtable, kills every cell, clears the canopy
/// shadow mask texture, frees the slot array, releases `spTexRendered` and
/// runs the `GridArray` destructor body.
pub fn fn_004ba400(e: &mut Engine, this: Ptr<GridCellArray>) {
    e.set(this, GridCellArray::vfptr, GRID_CELL_ARRAY_VTABLE);
    grid_cell_array_kill_all(e, this);
    grid_cell_array_clear_canopy_shadow_mask_texture(e, this);
    let slots = e.get(this, GridCellArray::pGridCellA);
    e.call(MEMORY_FREE, &args![slots]);
    e.call(SMART_POINTER_DESTRUCTOR, &args![this.byte_add(0x24)]);
    e.call(GRID_ARRAY_DESTRUCTOR_BODY, &args![this]);
}

// Translated from 004ba490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridCellArray::Get` (Xbox PDB): the address of the slot of cell
/// `(x, y)`, or 0 when either coordinate is not below the dimension.
pub fn grid_cell_array_get(e: &mut Engine, this: Ptr<GridCellArray>, x: u32, y: u32) -> Ptr {
    let limit = dimension(e, this);
    if x < limit && y < limit {
        let row_length = dimension(e, this);
        let index = row_length.wrapping_mul(x).wrapping_add(y);
        let slots = e.get(this, GridCellArray::pGridCellA);
        Ptr::new(slots.addr().wrapping_add(index.wrapping_mul(4)))
    } else {
        Ptr::NULL
    }
}

// Translated from 004ba4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the cell pointer that `value` points to into slot `(x, y)`
/// (nothing when `(x, y)` is outside the grid); a null `value` calls the
/// virtual `ClearItem` (slot `+0x1c`) instead. The map gives it no name.
pub fn fn_004ba4e0(e: &mut Engine, this: Ptr<GridCellArray>, x: u32, y: u32, value: Ptr) {
    let limit = dimension(e, this);
    if x < limit && y < limit {
        if value.is_null() {
            e.vcall(this.addr(), SLOT_CLEAR_ITEM, &args![x, y]);
        } else {
            let row_length = dimension(e, this);
            let index = row_length.wrapping_mul(x).wrapping_add(y);
            let slots = e.get(this, GridCellArray::pGridCellA);
            let cell = e.mem.u32(value.addr());
            e.mem
                .set_u32(slots.addr().wrapping_add(index.wrapping_mul(4)), cell);
        }
    }
}

// Translated from 004ba550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridCellArray::Set` (Xbox PDB): stores `cell` in slot `(x, y)` (found
/// with `Get`, which returns null outside the grid); a null `cell` calls the
/// virtual `ClearItem` (slot `+0x1c`).
pub fn grid_cell_array_set(e: &mut Engine, this: Ptr<GridCellArray>, x: u32, y: u32, cell: Ptr) {
    if cell.is_null() {
        e.vcall(this.addr(), SLOT_CLEAR_ITEM, &args![x, y]);
    } else {
        let slot = grid_cell_array_get(e, this, x, y);
        e.mem.set_u32(slot.addr(), cell.addr());
    }
}

// Translated from 004ba5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridCellArray::GetAttached` (Xbox PDB): false for an empty slot,
/// otherwise the cell test at `00450ff0` (true when the byte at `+0x26` of
/// the cell is 6).
pub fn grid_cell_array_get_attached(
    e: &mut Engine,
    this: Ptr<GridCellArray>,
    x: u32,
    y: u32,
) -> bool {
    let slot = grid_cell_array_get(e, this, x, y);
    let cell = cell_in(e, slot);
    if cell.is_null() {
        false
    } else {
        e.call(CELL_IS_TYPE_6, &args![cell]).bool()
    }
}

// Translated from 004ba5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridCellArray::Init` (Xbox PDB): `GridArray::Init`, then
/// `TESObjectCELL::InitStatics`.
pub fn grid_cell_array_init(e: &mut Engine, this: Ptr<GridCellArray>) {
    e.call(GRID_ARRAY_INIT, &args![this]);
    e.call(CELL_INIT_STATICS, &args![]);
}

// Translated from 004ba600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridCellArray::KillAll` (Xbox PDB): detaches every cell (the virtual
/// `DetachAll`), then tears down the cell statics when the flag at `+0x61d`
/// of the object in `011c3f2c` is set.
pub fn grid_cell_array_kill_all(e: &mut Engine, this: Ptr<GridCellArray>) {
    e.call(GRID_ARRAY_DETACH_ALL_CALL, &args![this]);
    let owner: u32 = e.global(KILL_ALL_FLAG_OWNER);
    if e.call(READ_FLAG_61D, &args![owner]).u8() != 0 {
        e.call(CELL_STATICS_TEARDOWN, &args![]);
    }
}

// Translated from 004ba630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridCellArray::SetCenter` (Xbox PDB): when `GridArray::SetCenter` moves
/// the grid, stores the world centre `(x << 12, y << 12, 0)`, hands it to
/// `TESObjectCELL::MoveExteriorWorld` and clears `bLandAttached`. Returns
/// whether the grid moved.
pub fn grid_cell_array_set_center(
    e: &mut Engine,
    this: Ptr<GridCellArray>,
    x: i32,
    y: i32,
) -> bool {
    let moved = e.call(GRID_ARRAY_SET_CENTER, &args![this, x, y]).u8() != 0;
    if moved {
        e.set(this, GridCellArray::WorldCenterX, x.wrapping_shl(12) as f32);
        e.set(this, GridCellArray::WorldCenterY, y.wrapping_shl(12) as f32);
        e.set(this, GridCellArray::WorldCenterZ, 0.0);
        e.call(CELL_MOVE_EXTERIOR_WORLD, &args![this.byte_add(0x14)]);
        e.set(this, GridCellArray::bLandAttached, false);
    }
    moved
}

// Translated from 004ba6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridCellArray::Detach` (Xbox PDB): when slot `(x, y)` holds a cell,
/// detaches it (`TESObjectCELL::Detach(1)`) and clears the slot through the
/// virtual `ClearItem`.
pub fn grid_cell_array_detach(e: &mut Engine, this: Ptr<GridCellArray>, x: u32, y: u32) {
    let slot = grid_cell_array_get(e, this, x, y);
    let cell = cell_in(e, slot);
    if !cell.is_null() {
        e.call(CELL_DETACH, &args![cell, 1u32]);
        e.vcall(this.addr(), SLOT_CLEAR_ITEM, &args![x, y]);
    }
}

// Translated from 004ba700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridCellArray::ClearItem` (Xbox PDB): empties slot `(x, y)` and clears
/// `bLandAttached`.
pub fn grid_cell_array_clear_item(e: &mut Engine, this: Ptr<GridCellArray>, x: u32, y: u32) {
    let slot = grid_cell_array_get(e, this, x, y);
    e.mem.set_u32(slot.addr(), 0);
    e.set(this, GridCellArray::bLandAttached, false);
}

// Translated from 004ba740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridCellArray::MoveItem` (Xbox PDB): stores the cell of slot
/// `(from_x, from_y)` into slot `(to_x, to_y)`. A source outside the grid
/// has no slot, so the destination is cleared through the virtual
/// `ClearItem`.
pub fn grid_cell_array_move_item(
    e: &mut Engine,
    this: Ptr<GridCellArray>,
    from_x: u32,
    from_y: u32,
    to_x: u32,
    to_y: u32,
) {
    let source = grid_cell_array_get(e, this, from_x, from_y);
    fn_004ba4e0(e, this, to_x, to_y, source);
}

// Translated from 004ba770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridCellArray::SwapItem` (Xbox PDB): exchanges the cells of slots
/// `(x1, y1)` and `(x2, y2)`.
pub fn grid_cell_array_swap_item(
    e: &mut Engine,
    this: Ptr<GridCellArray>,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
) {
    let first = grid_cell_array_get(e, this, x1, y1);
    let saved = e.mem.u32(first.addr());
    let second = grid_cell_array_get(e, this, x2, y2);
    fn_004ba4e0(e, this, x1, y1, second);
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), saved);
        fn_004ba4e0(e, this, x2, y2, local);
    });
}

// Translated from 004ba7d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ba7d0 - the compiler's exception-unwinding frame (FS:[0] handler, state table 00ef0f18) is not translated; the scope timer is destroyed on the normal path only
/// Skipped while `bLandAttached` is set. Otherwise, under a scope timer
/// (source `GridCell.cpp`, line 0x10d), runs the land function `005374f0` on
/// the land of every filled slot's cell and then the cell function
/// `005535f0` on the cell, and sets `bLandAttached`. The map gives it no
/// name.
pub fn fn_004ba7d0(e: &mut Engine, this: Ptr<GridCellArray>) {
    if e.get(this, GridCellArray::bLandAttached) {
        return;
    }
    let timer = e.mem.alloc(8);
    e.call(
        SCOPE_TIMER_CONSTRUCT,
        &args![timer, 0x1bu32, 1u32, GRID_CELL_SOURCE_FILE_NAME, 0x10du32],
    );
    let limit = dimension(e, this);
    for x in 0..limit {
        for y in 0..limit {
            let slot = grid_cell_array_get(e, this, x, y);
            if !slot.is_null() {
                let cell = cell_in(e, slot);
                if !cell.is_null() {
                    let land = e.call(CELL_GET_LAND, &args![cell]).u32();
                    e.call(LAND_FUNCTION_5374F0, &args![land]);
                    e.call(CELL_FUNCTION_5535F0, &args![cell]);
                }
            }
        }
    }
    e.set(this, GridCellArray::bLandAttached, true);
    e.call(SCOPE_TIMER_DESTROY, &args![timer]);
    e.mem.free(timer);
}

// Translated from 004ba8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `TESObjectLAND::InitDistantTextureBlending` on the land of every
/// filled slot's cell. The map gives it no name.
pub fn fn_004ba8d0(e: &mut Engine, this: Ptr<GridCellArray>) {
    let limit = dimension(e, this);
    for x in 0..limit {
        for y in 0..limit {
            let slot = grid_cell_array_get(e, this, x, y);
            if !slot.is_null() {
                let cell = cell_in(e, slot);
                if !cell.is_null() {
                    let land = e.call(CELL_GET_LAND, &args![cell]).u32();
                    e.call(LAND_INIT_DISTANT_TEXTURE_BLENDING, &args![land]);
                }
            }
        }
    }
}

// Translated from 004ba960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridCellArray::AttachToWorld` (Xbox PDB): attaches the cell of slot
/// `(x, y)`, if any, to `root`.
pub fn grid_cell_array_attach_to_world(
    e: &mut Engine,
    this: Ptr<GridCellArray>,
    root: Ptr,
    x: u32,
    y: u32,
) {
    let slot = grid_cell_array_get(e, this, x, y);
    let cell = cell_in(e, slot);
    if !cell.is_null() {
        e.call(CELL_ATTACH_TO_WORLD, &args![cell, root]);
    }
}

/// Whether the border pattern of `fn_004ba9a0` and `fn_004baab0` selects
/// cell `(x, y)` of a grid whose last index is `last`: cells off the border
/// are always selected, border cells alternate by parity of `x ^ y`, and the
/// toggle byte inverts the border.
fn border_pattern(x: u32, y: u32, last: u32, toggle: bool) -> bool {
    let mut selected = true;
    if x == 0 || y == 0 || x == last || y == last {
        if (x ^ y) & 1 != 0 {
            selected = false;
        }
        if toggle {
            selected = !selected;
        }
    }
    selected
}

// Translated from 004ba9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00554780(value)`, flips the byte at `011c642c`, then runs the cell
/// function `00551890(value, hidden)` on every filled slot's cell, where
/// `hidden` is the opposite of the border pattern for the slot. The map
/// gives it no name.
pub fn fn_004ba9a0(e: &mut Engine, this: Ptr<GridCellArray>, value: f32) {
    e.call(FLOAT_HOOK_554780, &args![value]);
    let limit = dimension(e, this);
    let last = limit.wrapping_sub(1);
    let toggle: u8 = e.global(BORDER_TOGGLE_A);
    let toggle = u8::from(toggle == 0);
    e.set_global(BORDER_TOGGLE_A, toggle);
    for x in 0..limit {
        for y in 0..limit {
            let selected = border_pattern(x, y, last, toggle != 0);
            let slot = grid_cell_array_get(e, this, x, y);
            let cell = cell_in(e, slot);
            if !cell.is_null() {
                e.call(
                    CELL_FUNCTION_551890,
                    &args![cell, value, u32::from(!selected)],
                );
            }
        }
    }
}

// Translated from 004baab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flips the byte at `011c642d` and runs the cell function `00552840` on
/// the cell of every filled slot that the border pattern selects. The map
/// gives it no name.
pub fn fn_004baab0(e: &mut Engine, this: Ptr<GridCellArray>) {
    let limit = dimension(e, this);
    let last = limit.wrapping_sub(1);
    let toggle: u8 = e.global(BORDER_TOGGLE_B);
    let toggle = u8::from(toggle == 0);
    e.set_global(BORDER_TOGGLE_B, toggle);
    for x in 0..limit {
        for y in 0..limit {
            if border_pattern(x, y, last, toggle != 0) {
                let slot = grid_cell_array_get(e, this, x, y);
                let cell = cell_in(e, slot);
                if !cell.is_null() {
                    e.call(CELL_FUNCTION_552840, &args![cell]);
                }
            }
        }
    }
}

// Translated from 004baba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the cell function `00553820` on the cell of every filled slot. The
/// map gives it no name.
pub fn fn_004baba0(e: &mut Engine, this: Ptr<GridCellArray>) {
    let mut x = 0;
    while x < dimension(e, this) {
        let mut y = 0;
        while y < dimension(e, this) {
            let slot = grid_cell_array_get(e, this, x, y);
            let cell = cell_in(e, slot);
            if !cell.is_null() {
                e.call(CELL_FUNCTION_553820, &args![cell]);
            }
            y += 1;
        }
        x += 1;
    }
}

// Translated from 004bac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `TESObjectCELL::UpdateRefSounds` on the cell of every filled slot.
/// The map gives it no name.
pub fn fn_004bac20(e: &mut Engine, this: Ptr<GridCellArray>) {
    let mut x = 0;
    while x < dimension(e, this) {
        let mut y = 0;
        while y < dimension(e, this) {
            let slot = grid_cell_array_get(e, this, x, y);
            if !slot.is_null() {
                let cell = cell_in(e, slot);
                if !cell.is_null() {
                    e.call(CELL_UPDATE_REF_SOUNDS, &args![cell]);
                }
            }
            y += 1;
        }
        x += 1;
    }
}

// Translated from 004baca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For every filled slot whose cell passes the test `00450ff0` and has a
/// land, runs the land function `0053fa40(flag)` on the land. The map gives
/// it no name.
pub fn fn_004baca0(e: &mut Engine, this: Ptr<GridCellArray>, flag: u8) {
    let mut x = 0;
    while x < dimension(e, this) {
        let mut y = 0;
        while y < dimension(e, this) {
            let slot = grid_cell_array_get(e, this, x, y);
            let cell = cell_in(e, slot);
            if !cell.is_null() && e.call(CELL_IS_TYPE_6, &args![cell]).bool() {
                let land = e.call(CELL_GET_LAND, &args![cell]).u32();
                if land != 0 {
                    e.call(LAND_FUNCTION_53FA40, &args![land, flag as u32]);
                }
            }
            y += 1;
        }
        x += 1;
    }
}

// Translated from 004bad40 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the object `00451010` returns is not null: runs the cell function
/// `00554960(!flag)` on every filled slot's cell that passes `00450ff0`,
/// where `flag` is the byte `fn_004bae00` reads from that object. The map
/// gives it no name.
pub fn fn_004bad40(e: &mut Engine, this: Ptr<GridCellArray>) {
    let object = e.call(SINGLETON_GET, &args![]).u32();
    if object == 0 {
        return;
    }
    let flag = fn_004bae00(e, Ptr::new(object));
    let argument = u32::from(flag == 0);
    let mut x = 0;
    while x < dimension(e, this) {
        let mut y = 0;
        while y < dimension(e, this) {
            let slot = grid_cell_array_get(e, this, x, y);
            let cell = cell_in(e, slot);
            if !cell.is_null() && e.call(CELL_IS_TYPE_6, &args![cell]).bool() {
                e.call(CELL_FUNCTION_554960, &args![cell, argument]);
            }
            y += 1;
        }
        x += 1;
    }
}

// Translated from 004bae00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x16` of `this` (the object that `00451010` returns; its
/// class is not named in the map).
pub fn fn_004bae00(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x16)
}

// Translated from 004bae20 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the object `00451010` returns is not null: runs the cell function
/// `005549d0(flag)` on every filled slot's cell that passes `00450ff0`. The
/// map gives it no name.
pub fn fn_004bae20(e: &mut Engine, this: Ptr<GridCellArray>, flag: u8) {
    if e.call(SINGLETON_GET, &args![]).u32() == 0 {
        return;
    }
    let mut x = 0;
    while x < dimension(e, this) {
        let mut y = 0;
        while y < dimension(e, this) {
            let slot = grid_cell_array_get(e, this, x, y);
            let cell = cell_in(e, slot);
            if !cell.is_null() && e.call(CELL_IS_TYPE_6, &args![cell]).bool() {
                e.call(CELL_FUNCTION_5549D0, &args![cell, flag as u32]);
            }
            y += 1;
        }
        x += 1;
    }
}

// Translated from 004baec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridCellArray::ClearCanopyShadowMaskTexture` (Xbox PDB): when
/// `spTexRendered` is set, clears the pointer at `011f95e8`, returns the
/// texture to the texture manager (`ReturnRenderedTexture`), empties
/// `spTexRendered` and sets `bRecreateCanopyShadowMaskTexture`.
pub fn grid_cell_array_clear_canopy_shadow_mask_texture(e: &mut Engine, this: Ptr<GridCellArray>) {
    let slot = this.byte_add(0x24);
    if e.call(POINTER_VALUE, &args![slot]).u32() != 0 {
        e.call(SINGLETON_SET, &args![0u32]);
        let texture = e.call(POINTER_VALUE, &args![slot]).u32();
        let manager = e.call(TEXTURE_MANAGER_GET, &args![]).u32();
        e.call(RETURN_RENDERED_TEXTURE, &args![manager, texture]);
        e.call(SMART_POINTER_SET, &args![slot, 0u32]);
        e.set_global(RECREATE_CANOPY_SHADOW_MASK_TEXTURE, 1u8);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004ba3d0,
            grid_cell_array_scalar_deleting_destructor(
                Ptr<GridCellArray>,
                u32,
            ) -> Ptr<GridCellArray>
        ),
        entry!(0x004ba400, fn_004ba400(Ptr<GridCellArray>)),
        entry!(
            0x004ba490,
            grid_cell_array_get(Ptr<GridCellArray>, u32, u32) -> Ptr
        ),
        entry!(0x004ba4e0, fn_004ba4e0(Ptr<GridCellArray>, u32, u32, Ptr)),
        entry!(
            0x004ba550,
            grid_cell_array_set(Ptr<GridCellArray>, u32, u32, Ptr)
        ),
        entry!(
            0x004ba5a0,
            grid_cell_array_get_attached(Ptr<GridCellArray>, u32, u32) -> bool
        ),
        entry!(0x004ba5e0, grid_cell_array_init(Ptr<GridCellArray>)),
        entry!(0x004ba600, grid_cell_array_kill_all(Ptr<GridCellArray>)),
        entry!(
            0x004ba630,
            grid_cell_array_set_center(Ptr<GridCellArray>, i32, i32) -> bool
        ),
        entry!(
            0x004ba6b0,
            grid_cell_array_detach(Ptr<GridCellArray>, u32, u32)
        ),
        entry!(
            0x004ba700,
            grid_cell_array_clear_item(Ptr<GridCellArray>, u32, u32)
        ),
        entry!(
            0x004ba740,
            grid_cell_array_move_item(Ptr<GridCellArray>, u32, u32, u32, u32)
        ),
        entry!(
            0x004ba770,
            grid_cell_array_swap_item(Ptr<GridCellArray>, u32, u32, u32, u32)
        ),
        entry!(0x004ba7d0, fn_004ba7d0(Ptr<GridCellArray>)),
        entry!(0x004ba8d0, fn_004ba8d0(Ptr<GridCellArray>)),
        entry!(
            0x004ba960,
            grid_cell_array_attach_to_world(Ptr<GridCellArray>, Ptr, u32, u32)
        ),
        entry!(0x004ba9a0, fn_004ba9a0(Ptr<GridCellArray>, f32)),
        entry!(0x004baab0, fn_004baab0(Ptr<GridCellArray>)),
        entry!(0x004baba0, fn_004baba0(Ptr<GridCellArray>)),
        entry!(0x004bac20, fn_004bac20(Ptr<GridCellArray>)),
        entry!(0x004baca0, fn_004baca0(Ptr<GridCellArray>, u8)),
        entry!(0x004bad40, fn_004bad40(Ptr<GridCellArray>)),
        entry!(0x004bae00, fn_004bae00(Ptr) -> u8),
        entry!(0x004bae20, fn_004bae20(Ptr<GridCellArray>, u8)),
        entry!(
            0x004baec0,
            grid_cell_array_clear_canopy_shadow_mask_texture(Ptr<GridCellArray>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Where the tests put the grid's vtable; only the `ClearItem` slot is
    /// filled (with a double).
    const TEST_VTABLE: u32 = 0x0150_0000;
    /// Address of the `ClearItem` double.
    const CLEAR_ITEM_DOUBLE: u32 = 0x0150_1000;

    fn answer(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine with the dimension getter, the pages for the globals this
    /// unit touches and the vtable with its `ClearItem` slot.
    fn grid_engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x011c_3000, 0x011c_6000, 0x0118_9000] {
            e.map(page, 0x1000);
        }
        e.register(GRID_DIMENSION, |e, a| answer(e.mem.u32(a[0] + 0xc)));
        let mut slots = vec![0u32; 10];
        slots[(SLOT_CLEAR_ITEM / 4) as usize] = CLEAR_ITEM_DOUBLE;
        e.put_vtable(TEST_VTABLE, &slots);
        e.register(CLEAR_ITEM_DOUBLE, |_, _| Ret::default());
        e
    }

    /// A grid of `size` by `size` empty slots.
    fn grid(e: &mut Engine, size: u32) -> Ptr<GridCellArray> {
        let this: Ptr<GridCellArray> = e.new_object();
        let slots = e.mem.alloc(size * size * 4);
        e.set(this, GridCellArray::vfptr, TEST_VTABLE);
        e.set(this, GridCellArray::iDimension, size);
        e.set(this, GridCellArray::pGridCellA, Ptr::new(slots));
        this
    }

    fn slot_address(e: &Engine, this: Ptr<GridCellArray>, x: u32, y: u32) -> u32 {
        let size = e.get(this, GridCellArray::iDimension);
        e.get(this, GridCellArray::pGridCellA).addr() + (size * x + y) * 4
    }

    /// Puts a new cell (a zeroed block) into slot `(x, y)`.
    fn fill(e: &mut Engine, this: Ptr<GridCellArray>, x: u32, y: u32) -> u32 {
        let cell = e.mem.alloc(0x40);
        let slot = slot_address(e, this, x, y);
        e.mem.set_u32(slot, cell);
        cell
    }

    fn cell_at(e: &Engine, this: Ptr<GridCellArray>, x: u32, y: u32) -> u32 {
        e.mem.u32(slot_address(e, this, x, y))
    }

    /// Doubles that do nothing for every callee of the destructor chain.
    fn quiet_destructor_callees(e: &mut Engine) {
        for address in [
            GRID_ARRAY_DETACH_ALL_CALL,
            READ_FLAG_61D,
            CELL_STATICS_TEARDOWN,
            POINTER_VALUE,
            MEMORY_FREE,
            SMART_POINTER_DESTRUCTOR,
            GRID_ARRAY_DESTRUCTOR_BODY,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// The addresses called, without the dimension getter (which every slot
    /// lookup calls).
    fn addresses(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter()
            .map(|(address, _)| *address)
            .filter(|address| *address != GRID_DIMENSION)
            .collect()
    }

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn scalar_deleting_destructor_frees_the_object_only_on_request() {
        let mut e = grid_engine();
        quiet_destructor_callees(&mut e);
        let this = grid(&mut e, 2);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x004b_a3d0, &args![this, 0u32])
            .ptr::<GridCellArray>();
        assert_eq!(back, this);
        let log = e.call_log.take().unwrap();
        assert!(!calls_to(&log, MEMORY_FREE).contains(&vec![this.addr()]));
        e.call_log = Some(vec![]);
        e.call(0x004b_a3d0, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MEMORY_FREE).last(), Some(&vec![this.addr()]));
    }

    #[test]
    fn destructor_tears_down_in_order() {
        let mut e = grid_engine();
        quiet_destructor_callees(&mut e);
        let this = grid(&mut e, 2);
        let slots = e.get(this, GridCellArray::pGridCellA);
        e.call_log = Some(vec![]);
        e.call(0x004b_a400, &args![this]);
        assert_eq!(e.get(this, GridCellArray::vfptr), GRID_CELL_ARRAY_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            addresses(&log),
            vec![
                0x004b_a400,
                GRID_ARRAY_DETACH_ALL_CALL,
                READ_FLAG_61D,
                POINTER_VALUE,
                MEMORY_FREE,
                SMART_POINTER_DESTRUCTOR,
                GRID_ARRAY_DESTRUCTOR_BODY,
            ]
        );
        assert_eq!(log[4].1, vec![slots.addr()]);
        assert_eq!(log[5].1, vec![this.addr() + 0x24]);
    }

    #[test]
    fn get_returns_the_slot_address_or_null_outside() {
        let mut e = grid_engine();
        let this = grid(&mut e, 3);
        let base = e.get(this, GridCellArray::pGridCellA).addr();
        let slot = e.call(0x004b_a490, &args![this, 1u32, 2u32]).u32();
        assert_eq!(slot, base + (3 + 2) * 4);
        assert_eq!(e.call(0x004b_a490, &args![this, 3u32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x004b_a490, &args![this, 0u32, 3u32]).u32(), 0);
    }

    #[test]
    fn store_through_a_pointer_copies_the_cell_or_clears() {
        let mut e = grid_engine();
        let this = grid(&mut e, 3);
        let cell = e.mem.alloc(0x40);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, cell);
        e.call(0x004b_a4e0, &args![this, 2u32, 1u32, holder]);
        assert_eq!(cell_at(&e, this, 2, 1), cell);
        // Outside the grid: nothing happens, not even the virtual call.
        e.call_log = Some(vec![]);
        e.call(0x004b_a4e0, &args![this, 3u32, 1u32, holder]);
        assert_eq!(addresses(&e.call_log.take().unwrap()), vec![0x004b_a4e0]);
        // A null value clears through the virtual `ClearItem`.
        e.call_log = Some(vec![]);
        e.call(0x004b_a4e0, &args![this, 1u32, 2u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, CLEAR_ITEM_DOUBLE),
            vec![vec![this.addr(), 1, 2]]
        );
    }

    #[test]
    fn set_stores_the_cell_or_clears_through_the_virtual() {
        let mut e = grid_engine();
        let this = grid(&mut e, 3);
        let cell = e.mem.alloc(0x40);
        e.call(0x004b_a550, &args![this, 1u32, 1u32, cell]);
        assert_eq!(cell_at(&e, this, 1, 1), cell);
        e.call_log = Some(vec![]);
        e.call(0x004b_a550, &args![this, 2u32, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, CLEAR_ITEM_DOUBLE),
            vec![vec![this.addr(), 2, 0]]
        );
    }

    #[test]
    fn get_attached_tests_only_filled_slots() {
        let mut e = grid_engine();
        let this = grid(&mut e, 2);
        e.register(CELL_IS_TYPE_6, |e, a| {
            answer(u32::from(e.mem.u8(a[0] + 0x26) == 6))
        });
        assert!(!e.call(0x004b_a5a0, &args![this, 0u32, 0u32]).bool());
        let cell = fill(&mut e, this, 0, 0);
        assert!(!e.call(0x004b_a5a0, &args![this, 0u32, 0u32]).bool());
        e.mem.set_u8(cell + 0x26, 6);
        assert!(e.call(0x004b_a5a0, &args![this, 0u32, 0u32]).bool());
    }

    #[test]
    fn init_runs_the_base_init_then_the_cell_statics() {
        let mut e = grid_engine();
        e.register(GRID_ARRAY_INIT, |_, _| Ret::default());
        e.register(CELL_INIT_STATICS, |_, _| Ret::default());
        let this = grid(&mut e, 2);
        e.call_log = Some(vec![]);
        e.call(0x004b_a5e0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            addresses(&log),
            vec![0x004b_a5e0, GRID_ARRAY_INIT, CELL_INIT_STATICS]
        );
        assert_eq!(log[1].1, vec![this.addr()]);
    }

    #[test]
    fn kill_all_tears_down_the_statics_only_when_flagged() {
        let mut e = grid_engine();
        for address in [GRID_ARRAY_DETACH_ALL_CALL, CELL_STATICS_TEARDOWN] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(READ_FLAG_61D, |e, a| {
            answer(u32::from(e.mem.u8(a[0] + 0x61d)))
        });
        let owner = e.mem.alloc(0x700);
        e.set_global(KILL_ALL_FLAG_OWNER, owner);
        let this = grid(&mut e, 2);
        e.call_log = Some(vec![]);
        e.call(0x004b_a600, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, GRID_ARRAY_DETACH_ALL_CALL).len(), 1);
        assert!(calls_to(&log, CELL_STATICS_TEARDOWN).is_empty());
        e.mem.set_u8(owner + 0x61d, 1);
        e.call_log = Some(vec![]);
        e.call(0x004b_a600, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CELL_STATICS_TEARDOWN).len(), 1);
    }

    #[test]
    fn set_center_publishes_the_world_centre_when_the_grid_moves() {
        let mut e = grid_engine();
        e.register(CELL_MOVE_EXTERIOR_WORLD, |_, _| Ret::default());
        e.register(GRID_ARRAY_SET_CENTER, |_, a| answer(u32::from(a[1] != 0)));
        let this = grid(&mut e, 2);
        e.set(this, GridCellArray::bLandAttached, true);
        e.call_log = Some(vec![]);
        let moved = e.call(0x004b_a630, &args![this, 3i32, -2i32]).bool();
        assert!(moved);
        assert_eq!(e.get(this, GridCellArray::WorldCenterX), 12288.0);
        assert_eq!(e.get(this, GridCellArray::WorldCenterY), -8192.0);
        assert_eq!(e.get(this, GridCellArray::WorldCenterZ), 0.0);
        assert!(!e.get(this, GridCellArray::bLandAttached));
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, CELL_MOVE_EXTERIOR_WORLD),
            vec![vec![this.addr() + 0x14]]
        );
    }

    #[test]
    fn set_center_leaves_everything_when_the_grid_does_not_move() {
        let mut e = grid_engine();
        e.register(CELL_MOVE_EXTERIOR_WORLD, |_, _| Ret::default());
        e.register(GRID_ARRAY_SET_CENTER, |_, _| answer(0));
        let this = grid(&mut e, 2);
        e.set(this, GridCellArray::bLandAttached, true);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004b_a630, &args![this, 3i32, 4i32]).bool());
        assert!(e.get(this, GridCellArray::bLandAttached));
        assert_eq!(e.get(this, GridCellArray::WorldCenterX), 0.0);
        assert_eq!(
            addresses(&e.call_log.take().unwrap()),
            vec![0x004b_a630, GRID_ARRAY_SET_CENTER]
        );
    }

    #[test]
    fn detach_detaches_a_filled_slot_and_clears_it() {
        let mut e = grid_engine();
        e.register(CELL_DETACH, |_, _| Ret::default());
        let this = grid(&mut e, 2);
        let cell = fill(&mut e, this, 1, 0);
        e.call_log = Some(vec![]);
        e.call(0x004b_a6b0, &args![this, 1u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            addresses(&log),
            vec![0x004b_a6b0, CELL_DETACH, CLEAR_ITEM_DOUBLE]
        );
        assert_eq!(calls_to(&log, CELL_DETACH), vec![vec![cell, 1]]);
        assert_eq!(
            calls_to(&log, CLEAR_ITEM_DOUBLE),
            vec![vec![this.addr(), 1, 0]]
        );
        // An empty slot does nothing.
        e.call_log = Some(vec![]);
        e.call(0x004b_a6b0, &args![this, 0u32, 0u32]);
        assert_eq!(addresses(&e.call_log.take().unwrap()), vec![0x004b_a6b0]);
    }

    #[test]
    fn clear_item_empties_the_slot_and_the_land_flag() {
        let mut e = grid_engine();
        let this = grid(&mut e, 2);
        fill(&mut e, this, 1, 1);
        e.set(this, GridCellArray::bLandAttached, true);
        e.call(0x004b_a700, &args![this, 1u32, 1u32]);
        assert_eq!(cell_at(&e, this, 1, 1), 0);
        assert!(!e.get(this, GridCellArray::bLandAttached));
    }

    #[test]
    fn move_item_copies_the_source_slot() {
        let mut e = grid_engine();
        let this = grid(&mut e, 3);
        let cell = fill(&mut e, this, 0, 1);
        e.call(0x004b_a740, &args![this, 0u32, 1u32, 2u32, 2u32]);
        assert_eq!(cell_at(&e, this, 2, 2), cell);
        assert_eq!(cell_at(&e, this, 0, 1), cell);
        // A source outside the grid has no slot: the destination is cleared
        // through the virtual.
        e.call_log = Some(vec![]);
        e.call(0x004b_a740, &args![this, 5u32, 5u32, 1u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, CLEAR_ITEM_DOUBLE),
            vec![vec![this.addr(), 1, 1]]
        );
    }

    #[test]
    fn swap_item_exchanges_the_two_slots() {
        let mut e = grid_engine();
        let this = grid(&mut e, 3);
        let first = fill(&mut e, this, 0, 0);
        let second = fill(&mut e, this, 2, 1);
        e.call(0x004b_a770, &args![this, 0u32, 0u32, 2u32, 1u32]);
        assert_eq!(cell_at(&e, this, 0, 0), second);
        assert_eq!(cell_at(&e, this, 2, 1), first);
    }

    /// Doubles for the callees of the land walks.
    fn land_callees(e: &mut Engine) {
        // The land of a cell is the word at +0x30 of the cell.
        e.register(CELL_GET_LAND, |e, a| answer(e.mem.u32(a[0] + 0x30)));
        for address in [
            LAND_FUNCTION_5374F0,
            CELL_FUNCTION_5535F0,
            LAND_INIT_DISTANT_TEXTURE_BLENDING,
            SCOPE_TIMER_CONSTRUCT,
            SCOPE_TIMER_DESTROY,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    #[test]
    fn land_walk_runs_once_under_the_scope_timer() {
        let mut e = grid_engine();
        land_callees(&mut e);
        let this = grid(&mut e, 2);
        let cell = fill(&mut e, this, 1, 0);
        e.mem.set_u32(cell + 0x30, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x004b_a7d0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            addresses(&log),
            vec![
                0x004b_a7d0,
                SCOPE_TIMER_CONSTRUCT,
                CELL_GET_LAND,
                LAND_FUNCTION_5374F0,
                CELL_FUNCTION_5535F0,
                SCOPE_TIMER_DESTROY,
            ]
        );
        assert_eq!(
            calls_to(&log, SCOPE_TIMER_CONSTRUCT)[0][1..],
            [0x1b, 1, GRID_CELL_SOURCE_FILE_NAME, 0x10d]
        );
        assert_eq!(calls_to(&log, LAND_FUNCTION_5374F0), vec![vec![0x1234]]);
        assert_eq!(calls_to(&log, CELL_FUNCTION_5535F0), vec![vec![cell]]);
        assert!(e.get(this, GridCellArray::bLandAttached));
        // Already attached: nothing runs.
        e.call_log = Some(vec![]);
        e.call(0x004b_a7d0, &args![this]);
        assert_eq!(addresses(&e.call_log.take().unwrap()), vec![0x004b_a7d0]);
    }

    #[test]
    fn distant_texture_blending_runs_on_the_land_of_filled_slots() {
        let mut e = grid_engine();
        land_callees(&mut e);
        let this = grid(&mut e, 2);
        let cell = fill(&mut e, this, 0, 1);
        e.mem.set_u32(cell + 0x30, 0x4321);
        e.call_log = Some(vec![]);
        e.call(0x004b_a8d0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LAND_INIT_DISTANT_TEXTURE_BLENDING),
            vec![vec![0x4321]]
        );
        assert_eq!(calls_to(&log, CELL_GET_LAND), vec![vec![cell]]);
    }

    #[test]
    fn attach_to_world_attaches_only_a_filled_slot() {
        let mut e = grid_engine();
        e.register(CELL_ATTACH_TO_WORLD, |_, _| Ret::default());
        let this = grid(&mut e, 2);
        let cell = fill(&mut e, this, 1, 1);
        e.call_log = Some(vec![]);
        e.call(0x004b_a960, &args![this, 0x7000u32, 1u32, 1u32]);
        e.call(0x004b_a960, &args![this, 0x7000u32, 0u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, CELL_ATTACH_TO_WORLD),
            vec![vec![cell, 0x7000]]
        );
    }

    #[test]
    fn border_pattern_selects_interior_and_alternates_the_border() {
        // Interior cells are always selected.
        assert!(border_pattern(1, 1, 2, false));
        assert!(border_pattern(1, 1, 2, true));
        // Border cells: even parity selected, odd not; the toggle inverts.
        assert!(border_pattern(0, 0, 2, false));
        assert!(!border_pattern(0, 1, 2, false));
        assert!(!border_pattern(0, 0, 2, true));
        assert!(border_pattern(0, 1, 2, true));
        assert!(!border_pattern(2, 1, 2, false));
    }

    #[test]
    fn border_fade_passes_the_value_and_the_hidden_flag() {
        let mut e = grid_engine();
        e.register(FLOAT_HOOK_554780, |_, _| Ret::default());
        e.register(CELL_FUNCTION_551890, |_, _| Ret::default());
        let this = grid(&mut e, 3);
        let corner = fill(&mut e, this, 0, 0);
        let edge = fill(&mut e, this, 0, 1);
        let middle = fill(&mut e, this, 1, 1);
        e.call_log = Some(vec![]);
        e.call(0x004b_a9a0, &args![this, 0.5f32]);
        assert_eq!(e.global::<u8>(BORDER_TOGGLE_A), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FLOAT_HOOK_554780),
            vec![vec![0.5f32.to_bits()]]
        );
        // Toggle is now set: the corner is not selected (hidden), the edge
        // is selected, the middle is selected.
        assert_eq!(
            calls_to(&log, CELL_FUNCTION_551890),
            vec![
                vec![corner, 0.5f32.to_bits(), 1],
                vec![edge, 0.5f32.to_bits(), 0],
                vec![middle, 0.5f32.to_bits(), 0],
            ]
        );
        // The next run flips the toggle back.
        e.call(0x004b_a9a0, &args![this, 0.5f32]);
        assert_eq!(e.global::<u8>(BORDER_TOGGLE_A), 0);
    }

    #[test]
    fn border_pass_runs_on_the_selected_cells_only() {
        let mut e = grid_engine();
        e.register(CELL_FUNCTION_552840, |_, _| Ret::default());
        let this = grid(&mut e, 3);
        let mut cells = vec![];
        for x in 0..3 {
            for y in 0..3 {
                cells.push(((x, y), fill(&mut e, this, x, y)));
            }
        }
        e.call_log = Some(vec![]);
        e.call(0x004b_aab0, &args![this]);
        assert_eq!(e.global::<u8>(BORDER_TOGGLE_B), 1);
        let log = e.call_log.take().unwrap();
        let called: Vec<u32> = calls_to(&log, CELL_FUNCTION_552840)
            .into_iter()
            .map(|words| words[0])
            .collect();
        // With the toggle set the odd-parity border cells and the middle.
        let wanted: Vec<u32> = cells
            .iter()
            .filter(|((x, y), _)| border_pattern(*x, *y, 2, true))
            .map(|(_, cell)| *cell)
            .collect();
        assert_eq!(called, wanted);
        assert_eq!(called.len(), 5);
    }

    #[test]
    fn plain_walks_visit_every_filled_slot() {
        let mut e = grid_engine();
        for address in [CELL_FUNCTION_553820, CELL_UPDATE_REF_SOUNDS] {
            e.register(address, |_, _| Ret::default());
        }
        let this = grid(&mut e, 2);
        let first = fill(&mut e, this, 0, 0);
        let second = fill(&mut e, this, 1, 0);
        e.call_log = Some(vec![]);
        e.call(0x004b_aba0, &args![this]);
        e.call(0x004b_ac20, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, CELL_FUNCTION_553820),
            vec![vec![first], vec![second]]
        );
        assert_eq!(
            calls_to(&log, CELL_UPDATE_REF_SOUNDS),
            vec![vec![first], vec![second]]
        );
    }

    #[test]
    fn land_flag_walk_needs_the_cell_test_and_a_land() {
        let mut e = grid_engine();
        e.register(CELL_IS_TYPE_6, |e, a| {
            answer(u32::from(e.mem.u8(a[0] + 0x26) == 6))
        });
        e.register(CELL_GET_LAND, |e, a| answer(e.mem.u32(a[0] + 0x30)));
        e.register(LAND_FUNCTION_53FA40, |_, _| Ret::default());
        let this = grid(&mut e, 2);
        let passing = fill(&mut e, this, 0, 0);
        e.mem.set_u8(passing + 0x26, 6);
        e.mem.set_u32(passing + 0x30, 0x5000);
        // Passes the test but has no land.
        let no_land = fill(&mut e, this, 0, 1);
        e.mem.set_u8(no_land + 0x26, 6);
        // Has a land but fails the test.
        let failing = fill(&mut e, this, 1, 0);
        e.mem.set_u32(failing + 0x30, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x004b_aca0, &args![this, 1u8]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LAND_FUNCTION_53FA40), vec![vec![0x5000, 1]]);
    }

    #[test]
    fn flag_byte_getter_reads_offset_0x16() {
        let mut e = grid_engine();
        let object = e.mem.alloc(0x20);
        e.mem.set_u8(object + 0x16, 9);
        assert_eq!(e.call(0x004b_ae00, &args![object]).u8(), 9);
    }

    #[test]
    fn inverted_flag_walk_follows_the_singleton() {
        let mut e = grid_engine();
        e.register(CELL_IS_TYPE_6, |e, a| {
            answer(u32::from(e.mem.u8(a[0] + 0x26) == 6))
        });
        e.register(CELL_FUNCTION_554960, |_, _| Ret::default());
        let object = e.mem.alloc(0x20);
        e.register_double(SINGLETON_GET, move |_, _| answer(object));
        let this = grid(&mut e, 2);
        let cell = fill(&mut e, this, 1, 1);
        e.mem.set_u8(cell + 0x26, 6);
        let other = fill(&mut e, this, 0, 0);
        e.mem.set_u8(other + 0x26, 5);
        // Flag 0: the argument is 1.
        e.call_log = Some(vec![]);
        e.call(0x004b_ad40, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CELL_FUNCTION_554960), vec![vec![cell, 1]]);
        // Flag set: the argument is 0.
        e.mem.set_u8(object + 0x16, 3);
        e.call_log = Some(vec![]);
        e.call(0x004b_ad40, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CELL_FUNCTION_554960), vec![vec![cell, 0]]);
    }

    #[test]
    fn inverted_flag_walk_does_nothing_without_the_singleton() {
        let mut e = grid_engine();
        e.register(SINGLETON_GET, |_, _| answer(0));
        let this = grid(&mut e, 2);
        fill(&mut e, this, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x004b_ad40, &args![this]);
        e.call(0x004b_ae20, &args![this, 1u8]);
        let addrs = addresses(&e.call_log.take().unwrap());
        assert_eq!(
            addrs,
            vec![0x004b_ad40, SINGLETON_GET, 0x004b_ae20, SINGLETON_GET]
        );
    }

    #[test]
    fn byte_flag_walk_passes_the_argument() {
        let mut e = grid_engine();
        e.register(CELL_IS_TYPE_6, |e, a| {
            answer(u32::from(e.mem.u8(a[0] + 0x26) == 6))
        });
        e.register(CELL_FUNCTION_5549D0, |_, _| Ret::default());
        e.register(SINGLETON_GET, |_, _| answer(0x1000));
        let this = grid(&mut e, 2);
        let cell = fill(&mut e, this, 1, 0);
        e.mem.set_u8(cell + 0x26, 6);
        fill(&mut e, this, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x004b_ae20, &args![this, 7u8]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CELL_FUNCTION_5549D0), vec![vec![cell, 7]]);
    }

    #[test]
    fn clear_canopy_shadow_mask_texture_returns_the_texture() {
        let mut e = grid_engine();
        e.register(POINTER_VALUE, |e, a| answer(e.mem.u32(a[0])));
        e.register(SINGLETON_SET, |_, _| Ret::default());
        e.register(TEXTURE_MANAGER_GET, |_, _| answer(0x8000));
        e.register(RETURN_RENDERED_TEXTURE, |_, _| Ret::default());
        e.register(SMART_POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let this = grid(&mut e, 2);
        // No texture: nothing but the check.
        e.call_log = Some(vec![]);
        e.call(0x004b_aec0, &args![this]);
        assert_eq!(
            addresses(&e.call_log.take().unwrap()),
            vec![0x004b_aec0, POINTER_VALUE]
        );
        assert_eq!(e.global::<u8>(RECREATE_CANOPY_SHADOW_MASK_TEXTURE), 0);
        // With a texture.
        e.set(this, GridCellArray::spTexRendered, Ptr::new(0x9000));
        e.call_log = Some(vec![]);
        e.call(0x004b_aec0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            addresses(&log),
            vec![
                0x004b_aec0,
                POINTER_VALUE,
                SINGLETON_SET,
                POINTER_VALUE,
                TEXTURE_MANAGER_GET,
                RETURN_RENDERED_TEXTURE,
                SMART_POINTER_SET,
            ]
        );
        assert_eq!(log[2].1, vec![0]);
        assert_eq!(log[5].1, vec![0x8000, 0x9000]);
        assert!(e.get(this, GridCellArray::spTexRendered).is_null());
        assert_eq!(e.global::<u8>(RECREATE_CANOPY_SHADOW_MASK_TEXTURE), 1);
    }
}
