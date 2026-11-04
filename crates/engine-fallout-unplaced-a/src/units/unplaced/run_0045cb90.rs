//! `(unplaced)/run_0045cb90` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The run mixes small getters, two `TES` helpers (water height around a
//! position, a pass over the loaded cells), a node free list, and the
//! `Setting` constructors of the INI and renderer setting collections with
//! the renderer collection's singleton. Class names of the vtables come from
//! their RTTI type descriptors in the exe (`Setting`,
//! `SettingT<INISettingCollection>`, `SettingT<RendererSettingCollection>`,
//! `RendererSettingCollection`); `Setting`'s layout is the Xbox PDB's
//! (`uValue` at +4, `pKey` at +8, 0xC bytes), which these constructors match.

#[allow(unused_imports)]
use crate::prelude::*;

/// Drops one reference of a counted object (`NiRefObject`: the count is at
/// `+4`, the object deletes itself through vtable slot `+4` at zero).
const RELEASE_REFERENCE: u32 = 0x0040_1970;

/// The `TES` singleton pointer (`011dea10`).
const TES_SINGLETON: u32 = 0x011d_ea10;
/// The float `-FLT_MAX` (`0xff7fffff`), the height returned when no water
/// is found.
const LOWEST_HEIGHT: u32 = 0x0101_5f5c;
/// `TES::GetWorldSpace` (engine map): the current worldspace of `TES`.
const TES_GET_WORLD_SPACE: u32 = 0x004f_d3e0;
/// A getter that returns the word at `+0x34` (engine map name
/// `ActorMover::GetPreferredMoveMode`; folded code). Called on `this` here.
const GET_FIELD_34: u32 = 0x005f_36f0;
/// Unnamed `TESObjectCELL` function `(cell, position, &height) -> bool`
/// (engine map unit `tesobjectcell.cpp`): writes a water height at the
/// position into `height` on every path and answers whether a water object
/// covered the position.
const CELL_WATER_HEIGHT_AT: u32 = 0x0054_7250;
/// Unnamed flag getter `(cell) -> bool`: bit 0 of the byte at `+0x24`.
const CELL_FLAG_BIT_0: u32 = 0x0042_5fd0;
/// `TESObjectCELL::GetWaterHeight` (engine map): a float in ST0.
const CELL_GET_WATER_HEIGHT: u32 = 0x0054_71e0;
/// `TESWorldSpace::GetCellFromWorldCoord` (engine map): `(worldspace,
/// position) -> cell`.
const WORLD_SPACE_CELL_AT_POSITION: u32 = 0x0058_7550;
/// `TESWorldSpace::GetCellFromCellCoord` (engine map): `(worldspace, x, y)
/// -> cell`.
const WORLD_SPACE_CELL_AT_COORDINATES: u32 = 0x0058_75a0;
/// `TESObjectCELL::GetDataX` / `GetDataY` (engine map).
const CELL_GET_X: u32 = 0x0054_4c30;
const CELL_GET_Y: u32 = 0x0054_4c60;
/// A getter called on `TES` for the grid's X coordinate (engine map name
/// `D3DTexture_LockRect`; folded code).
const TES_GRID_X: u32 = 0x0059_bb30;

/// Address of a setting object's value (`0043d4d0`).
const SETTING_VALUE_ADDRESS: u32 = 0x0043_d4d0;
/// The setting whose value bounds both loops of [`fn_0045cda0`] (a static
/// object; its name is not confirmed here).
const GRID_SIZE_SETTING: u32 = 0x011c_63cc;
/// `GridCellArray::Get` (engine map): `(grid, x, y) -> &cell`.
const GRID_CELL_ARRAY_GET: u32 = 0x004b_a490;
/// Unnamed `TESObjectCELL` function (engine map unit `tesobjectcell.cpp`)
/// called on every loaded cell by [`fn_0045cda0`].
const CELL_VISIT: u32 = 0x0055_8c90;

/// The recursive lock guarding the node free list, taken by `0040fbf0`
/// and released by `0040fba0`.
const FREE_LIST_LOCK: u32 = 0x011c_5f80;
const LOCK_ACQUIRE: u32 = 0x0040_fbf0;
const LOCK_RELEASE: u32 = 0x0040_fba0;
/// Head of the free list of 8-byte nodes (next at +0, element at +4).
const FREE_LIST_HEAD: u32 = 0x011c_5f58;

/// `Setting` vtable (RTTI `.?AVSetting@@`).
const SETTING_VTABLE: u32 = 0x0101_2120;
/// `SettingT<INISettingCollection>` vtable (RTTI).
const INI_SETTING_VTABLE: u32 = 0x0101_7720;
/// `SettingT<RendererSettingCollection>` vtable (RTTI).
const RENDERER_SETTING_VTABLE: u32 = 0x0101_84fc;
/// `RendererSettingCollection` vtable (RTTI).
const RENDERER_COLLECTION_VTABLE: u32 = 0x0101_8508;
/// `Setting` constructor with an integer value `(this, name, value)`.
const SETTING_CONSTRUCT_INT: u32 = 0x0040_4920;
/// `Setting` constructor with a float value `(this, name, value)`.
const SETTING_CONSTRUCT_FLOAT: u32 = 0x0040_e130;
/// `Setting` destructor body (`00404950`).
const SETTING_DESTRUCT: u32 = 0x0040_4950;
/// The INI setting collection singleton getter (`0044f560`).
const INI_COLLECTION_INSTANCE: u32 = 0x0044_f560;
/// The INI setting collection constructor (`0044f600`), the base of the
/// renderer collection.
const INI_COLLECTION_CONSTRUCT: u32 = 0x0044_f600;
/// The renderer collection's destructor body (`00877a10`).
const COLLECTION_DESTRUCT: u32 = 0x0087_7a10;
/// The renderer setting collection singleton (`011f35a4`).
const RENDERER_COLLECTION_INSTANCE: u32 = 0x011f_35a4;
/// Size of the renderer setting collection.
const RENDERER_COLLECTION_SIZE: u32 = 0x114;
/// `operator new` / `operator delete`.
const MEMORY_ALLOCATE: u32 = 0x0040_1000;
const MEMORY_FREE: u32 = 0x0040_1030;

/// Collection vtable slots called by the setting constructors and
/// destructor: `+4` adds the setting, `+8` removes it (from the calls'
/// use; the slot names are not confirmed).
const COLLECTION_ADD_SLOT: u32 = 4;
const COLLECTION_REMOVE_SLOT: u32 = 8;

/// A float the exe copies through `FLD` and `FSTP`: a signaling NaN comes
/// out quiet.
fn quiet(value: f32) -> f32 {
    x87_quiet(value)
}

// Translated from 0045cb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the pointer at `+0x28` of `this` is set and the word at `+4` of
/// the object it points to is not zero. (Only AL is set; callers read AL.)
pub fn fn_0045cb90(e: &mut Engine, this: Ptr) -> bool {
    let held = e.mem.u32(this.addr() + 0x28);
    held != 0 && e.mem.u32(held + 4) != 0
}

/// `00547250(cell, position, out)`: true when water covers the position;
/// the height is written to `out` on every path.
fn cell_water_height_at(e: &mut Engine, cell: Ptr, position: Ptr, out: u32) -> bool {
    e.call(CELL_WATER_HEIGHT_AT, &args![cell, position, out])
        .bool()
}

/// `TES::GetWorldSpace` on the `TES` singleton (read at each call, as the
/// exe does).
fn current_world_space(e: &mut Engine) -> u32 {
    let tes = e.global::<u32>(TES_SINGLETON);
    e.call(TES_GET_WORLD_SPACE, &args![tes]).u32()
}

// Translated from 0045cbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Water height at `position` (a 3-float point) on `this` (callers pass the
/// `TES` singleton). With a `cell`: its water height at the position when
/// water covers it, `-FLT_MAX` when its flag bit (`00425fd0`) is set,
/// otherwise its plain water height. Without a cell: in an interior (the
/// word at `+0x34` of `this`, read through `005f36f0`) the interior's water
/// height at the position whatever it answers; otherwise the worldspace cell
/// under the position, or, with none, the worldspace's default height
/// ([`fn_0045cd80`]). Having a cell, the worldspace cells of the 3 x 3 block
/// around it (`x-1..=x+1` outer, `y-1..=y+1` inner, by
/// `GetCellFromCellCoord`), skipping empty slots and the cell itself, are
/// tried in order: the first with water over the position gives its height.
/// Otherwise the cell's plain water height. The exe also has a branch taking
/// the block's centre from `TES` (`0059bb30`, [`fn_0045cd60`]) when no cell
/// is set; every path that reaches it has a cell, but it is kept.
pub fn fn_0045cbc0(e: &mut Engine, this: Ptr, position: Ptr, cell: Ptr) -> f32 {
    e.with_stack(12, |e, locals| {
        let cell_height = locals.addr();
        let interior_height = locals.addr() + 4;
        let neighbour_height = locals.addr() + 8;
        let mut cell = cell;
        let lowest = if !cell.is_null() {
            if cell_water_height_at(e, cell, position, cell_height) {
                return e.mem.f32(cell_height);
            }
            if e.call(CELL_FLAG_BIT_0, &args![cell]).bool() {
                return e.global::<f32>(LOWEST_HEIGHT);
            }
            e.call(CELL_GET_WATER_HEIGHT, &args![cell]).f32()
        } else {
            if e.call(GET_FIELD_34, &args![this]).u32() != 0 {
                let interior = e.call(GET_FIELD_34, &args![this]).u32();
                cell_water_height_at(e, Ptr::new(interior), position, interior_height);
                return e.mem.f32(interior_height);
            }
            let world_space = current_world_space(e);
            cell = e
                .call(WORLD_SPACE_CELL_AT_POSITION, &args![world_space, position])
                .ptr();
            if cell.is_null() {
                let world_space = current_world_space(e);
                return fn_0045cd80(e, Ptr::new(world_space));
            }
            e.call(CELL_GET_WATER_HEIGHT, &args![cell]).f32()
        };
        // Always true here: both ways that leave `cell` null returned above.
        let (centre_x, centre_y) = if !cell.is_null() {
            (
                e.call(CELL_GET_X, &args![cell]).i32(),
                e.call(CELL_GET_Y, &args![cell]).i32(),
            )
        } else {
            let tes = e.global::<u32>(TES_SINGLETON);
            let x = e.call(TES_GRID_X, &args![tes]).i32();
            let tes = e.global::<u32>(TES_SINGLETON);
            (x, fn_0045cd60(e, Ptr::new(tes)) as i32)
        };
        e.mem.set_f32(neighbour_height, 0.0);
        let mut x = centre_x.wrapping_sub(1);
        while x < centre_x.wrapping_add(2) {
            let mut y = centre_y.wrapping_sub(1);
            while y < centre_y.wrapping_add(2) {
                let world_space = current_world_space(e);
                let neighbour = e
                    .call(WORLD_SPACE_CELL_AT_COORDINATES, &args![world_space, x, y])
                    .ptr();
                if !neighbour.is_null()
                    && neighbour.addr() != cell.addr()
                    && cell_water_height_at(e, neighbour, position, neighbour_height)
                {
                    return e.mem.f32(neighbour_height);
                }
                y = y.wrapping_add(1);
            }
            x = x.wrapping_add(1);
        }
        lowest
    })
}

// Translated from 0045cd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `+0x28` of `this` (a plain field getter).
pub fn fn_0045cd60(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x28)
}

// Translated from 0045cd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the float at `+0x7C` of `this` (a plain field getter; called on
/// the current worldspace by [`fn_0045cbc0`]).
pub fn fn_0045cd80(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x7c)
}

// Translated from 0045cda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00558c90` on the loaded cells of `this` (callers' `TES`): on the
/// cell at `+0x34` when it is set (the interior), otherwise on every
/// non-null cell of the grid array at `+0x8` (`GridCellArray::Get(x, y)`),
/// with `x` and `y` from 0 while below the value of the setting at
/// `011c63cc` (re-read before every comparison, unsigned).
pub fn fn_0045cda0(e: &mut Engine, this: Ptr) {
    let interior = e.mem.u32(this.addr() + 0x34);
    if interior != 0 {
        e.call(CELL_VISIT, &args![interior]);
        return;
    }
    let mut x = 0u32;
    loop {
        let size = e
            .call(SETTING_VALUE_ADDRESS, &args![GRID_SIZE_SETTING])
            .u32();
        if x >= e.mem.u32(size) {
            break;
        }
        let mut y = 0u32;
        loop {
            let size = e
                .call(SETTING_VALUE_ADDRESS, &args![GRID_SIZE_SETTING])
                .u32();
            if y >= e.mem.u32(size) {
                break;
            }
            let grid = e.mem.u32(this.addr() + 0x8);
            let slot = e.call(GRID_CELL_ARRAY_GET, &args![grid, x, y]).u32();
            let cell = e.mem.u32(slot);
            if cell != 0 {
                e.call(CELL_VISIT, &args![cell]);
            }
            y = y.wrapping_add(1);
        }
        x = x.wrapping_add(1);
    }
}

// Translated from 0045ce40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Increments the word at `+0xB8` of `this`.
pub fn fn_0045ce40(e: &mut Engine, this: Ptr) {
    let count = e.mem.u32(this.addr() + 0xb8);
    e.mem.set_u32(this.addr() + 0xb8, count.wrapping_add(1));
}

// Translated from 0045ce60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Decrements the word at `+0xB8` of `this`.
pub fn fn_0045ce60(e: &mut Engine, this: Ptr) {
    let count = e.mem.u32(this.addr() + 0xb8);
    e.mem.set_u32(this.addr() + 0xb8, count.wrapping_sub(1));
}

// Translated from 0045ce80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs [`fn_0045cea0`] with `value` and returns `this`.
pub fn fn_0045ce80(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    fn_0045cea0(e, this, value);
    this
}

// Translated from 0045cea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at `+4` of `this` when `this` is not null; returns `this`.
pub fn fn_0045cea0(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    if !this.is_null() {
        e.mem.set_u32(this.addr() + 4, value);
    }
    this
}

// Translated from 0045cec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Smart-pointer destructor: if the pointer held at `+0` of `this` is not
/// null, drops its reference. (The decompiler's library name for this address
/// is a false match.)
pub fn fn_0045cec0(e: &mut Engine, this: Ptr) {
    let held = e.mem.u32(this.addr());
    if held != 0 {
        e.call(RELEASE_REFERENCE, &args![held]);
    }
}

// Translated from 0045cee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees a list node: under the lock at `011c5f80`, clears the node's
/// element word (`+4`) and pushes it on the free list whose head is at
/// `011c5f58` (the old head goes to `+0`). `this` (callers pass their
/// list's allocator) is not read.
pub fn fn_0045cee0(e: &mut Engine, _this: Ptr, node: Ptr) {
    // `NV_SPEECH_CHECK` (diagnostic only, reads memory): a node still in a watched list.
    crate::units::fallout_shared_facegen::bsfacegenkeyframe::speech_check::node_freeing(
        e,
        node.addr(),
    );
    e.call(LOCK_ACQUIRE, &args![FREE_LIST_LOCK, 0u32]);
    e.mem.set_u32(node.addr() + 4, 0);
    let head = e.global::<u32>(FREE_LIST_HEAD);
    e.mem.set_u32(node.addr(), head);
    e.set_global(FREE_LIST_HEAD, node.addr());
    e.call(LOCK_RELEASE, &args![FREE_LIST_LOCK]);
}

// Translated from 0045cf30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0045cf30 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// `SettingT<INISettingCollection>` constructor with a byte value: the
/// `Setting` constructor ([`fn_0045cfb0`]), the derived vtable, then adds
/// itself to the INI setting collection (`0044f560`, vtable slot `+4`);
/// returns `this`.
pub fn fn_0045cf30(e: &mut Engine, this: Ptr, name: Ptr, value: u8) -> Ptr {
    fn_0045cfb0(e, this, name, value);
    e.mem.set_u32(this.addr(), INI_SETTING_VTABLE);
    let collection = e.call(INI_COLLECTION_INSTANCE, &[]).u32();
    e.vcall(collection, COLLECTION_ADD_SLOT, &args![this]);
    this
}

// Translated from 0045cfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Setting` constructor with a byte value: the `Setting` vtable,
/// `pKey` (+8) = `name`, the low byte of `uValue` (+4) = `value`; returns
/// `this`.
pub fn fn_0045cfb0(e: &mut Engine, this: Ptr, name: Ptr, value: u8) -> Ptr {
    e.mem.set_u32(this.addr(), SETTING_VTABLE);
    e.mem.set_u32(this.addr() + 8, name.addr());
    e.mem.set_u8(this.addr() + 4, value);
    this
}

// Translated from 0045cfe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0045cfe0 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// `SettingT<INISettingCollection>` constructor with a word value: the
/// `Setting` constructor `00404920`, the derived vtable, then adds itself to
/// the INI setting collection (`0044f560`, vtable slot `+4`); returns
/// `this`.
pub fn fn_0045cfe0(e: &mut Engine, this: Ptr, name: Ptr, value: u32) -> Ptr {
    e.call(SETTING_CONSTRUCT_INT, &args![this, name, value]);
    e.mem.set_u32(this.addr(), INI_SETTING_VTABLE);
    let collection = e.call(INI_COLLECTION_INSTANCE, &[]).u32();
    e.vcall(collection, COLLECTION_ADD_SLOT, &args![this]);
    this
}

// Translated from 0045d060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0045d060 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// `SettingT<RendererSettingCollection>` constructor with a float value:
/// the `Setting` constructor `0040e130` (the float copied through the x87
/// stack), the derived vtable, then adds itself to the renderer setting
/// collection ([`fn_0045d180`], vtable slot `+4`); returns `this`.
pub fn fn_0045d060(e: &mut Engine, this: Ptr, name: Ptr, value: f32) -> Ptr {
    e.call(
        SETTING_CONSTRUCT_FLOAT,
        &args![this, name, quiet(value).to_bits()],
    );
    e.mem.set_u32(this.addr(), RENDERER_SETTING_VTABLE);
    let collection = fn_0045d180(e);
    e.vcall(collection.addr(), COLLECTION_ADD_SLOT, &args![this]);
    this
}

// Translated from 0045d0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `SettingT<RendererSettingCollection>`:
/// [`fn_0045d110`], then frees the block when bit 0 of `flags` is set;
/// returns `this`.
pub fn fn_0045d0e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0045d110(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 0045d110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0045d110 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// Destructor body of `SettingT<RendererSettingCollection>`: its vtable,
/// removes itself from the renderer setting collection ([`fn_0045d180`],
/// vtable slot `+8`), then the `Setting` destructor `00404950`.
pub fn fn_0045d110(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), RENDERER_SETTING_VTABLE);
    let collection = fn_0045d180(e);
    e.vcall(collection.addr(), COLLECTION_REMOVE_SLOT, &args![this]);
    e.call(SETTING_DESTRUCT, &args![this]);
}

// Translated from 0045d180 (decompiled, FalloutNV.exe 1.4.0.525)
/// The renderer setting collection singleton (`011f35a4`), created by
/// [`fn_0045d190`] on first use.
pub fn fn_0045d180(e: &mut Engine) -> Ptr {
    fn_0045d190(e);
    Ptr::new(e.global::<u32>(RENDERER_COLLECTION_INSTANCE))
}

// Translated from 0045d190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0045d190 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// Creates the renderer setting collection singleton when it does not
/// exist yet: a 0x114-byte block (`operator new`) built by
/// [`fn_0045d220`], or 0 when the allocation fails, stored at `011f35a4`.
pub fn fn_0045d190(e: &mut Engine) {
    if e.global::<u32>(RENDERER_COLLECTION_INSTANCE) == 0 {
        let block = e
            .call(MEMORY_ALLOCATE, &args![RENDERER_COLLECTION_SIZE])
            .u32();
        let instance = if block == 0 {
            0
        } else {
            fn_0045d220(e, Ptr::new(block)).addr()
        };
        e.set_global(RENDERER_COLLECTION_INSTANCE, instance);
    }
}

// Translated from 0045d220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RendererSettingCollection` constructor: the INI setting collection
/// constructor `0044f600`, then its own vtable; returns `this`.
pub fn fn_0045d220(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(INI_COLLECTION_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), RENDERER_COLLECTION_VTABLE);
    this
}

// Translated from 0045d240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00877a10` (the collection's destructor body) on `this`. EAX keeps
/// that call's result on return; the function is `void`.
pub fn fn_0045d240(e: &mut Engine, this: Ptr) {
    e.call(COLLECTION_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0045cb90, fn_0045cb90(Ptr) -> bool),
        entry!(0x0045cbc0, fn_0045cbc0(Ptr, Ptr, Ptr) -> f32),
        entry!(0x0045cd60, fn_0045cd60(Ptr) -> u32),
        entry!(0x0045cd80, fn_0045cd80(Ptr) -> f32),
        entry!(0x0045cda0, fn_0045cda0(Ptr)),
        entry!(0x0045ce40, fn_0045ce40(Ptr)),
        entry!(0x0045ce60, fn_0045ce60(Ptr)),
        entry!(0x0045ce80, fn_0045ce80(Ptr, u32) -> Ptr),
        entry!(0x0045cea0, fn_0045cea0(Ptr, u32) -> Ptr),
        entry!(0x0045cec0, fn_0045cec0(Ptr)),
        entry!(0x0045cee0, fn_0045cee0(Ptr, Ptr)),
        entry!(0x0045cf30, fn_0045cf30(Ptr, Ptr, u8) -> Ptr),
        entry!(0x0045cfb0, fn_0045cfb0(Ptr, Ptr, u8) -> Ptr),
        entry!(0x0045cfe0, fn_0045cfe0(Ptr, Ptr, u32) -> Ptr),
        entry!(0x0045d060, fn_0045d060(Ptr, Ptr, f32) -> Ptr),
        entry!(0x0045d0e0, fn_0045d0e0(Ptr, u32) -> Ptr),
        entry!(0x0045d110, fn_0045d110(Ptr)),
        entry!(0x0045d180, fn_0045d180() -> Ptr),
        entry!(0x0045d190, fn_0045d190()),
        entry!(0x0045d220, fn_0045d220(Ptr) -> Ptr),
        entry!(0x0045d240, fn_0045d240(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The argument words of every logged call to `address`.
    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(target, _)| *target == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    #[test]
    fn checks_the_held_object_and_its_word_at_4() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x30));
        assert!(!e.call(0x0045_cb90, &args![this]).bool());
        let held = e.mem.alloc(8);
        e.mem.set_u32(this.addr() + 0x28, held);
        assert!(!e.call(0x0045_cb90, &args![this]).bool());
        e.mem.set_u32(held + 4, 7);
        assert!(e.call(0x0045_cb90, &args![this]).bool());
    }

    const TES: u32 = 0x0300_0000;
    const WORLD: u32 = 0x0300_1000;
    const START_CELL: u32 = 0x0300_2000;
    const NEIGHBOUR: u32 = 0x0300_3000;

    /// Water height doubles: `00547250` writes `height` and answers `found`
    /// for the cells listed; every other cell answers false with height 1.
    fn water_engine(found: &'static [(u32, f32)]) -> Engine {
        let mut e = Engine::new();
        e.map(TES_SINGLETON, 4);
        e.set_global(TES_SINGLETON, TES);
        e.map(LOWEST_HEIGHT, 4);
        e.set_global(LOWEST_HEIGHT, f32::from_bits(0xff7f_ffff));
        e.map(TES, 0x40);
        e.map(WORLD, 0x80);
        e.mem.set_f32(WORLD + 0x7c, -55.0);
        e.register_double(CELL_WATER_HEIGHT_AT, move |e, a| {
            match found.iter().find(|(cell, _)| *cell == a[0]) {
                Some((_, height)) => {
                    e.mem.set_f32(a[2], *height);
                    eax(1)
                }
                None => {
                    e.mem.set_f32(a[2], 1.0);
                    eax(0)
                }
            }
        });
        e.register(CELL_FLAG_BIT_0, |_, _| eax(0));
        e.register(CELL_GET_WATER_HEIGHT, |_, _| st0(-7.5));
        e.register(GET_FIELD_34, |e, a| eax(e.mem.u32(a[0] + 0x34)));
        e.register(TES_GET_WORLD_SPACE, |_, _| eax(WORLD));
        e.register(WORLD_SPACE_CELL_AT_POSITION, |_, _| eax(START_CELL));
        e.register(CELL_GET_X, |_, _| eax(10));
        e.register(CELL_GET_Y, |_, _| eax(-3i32 as u32));
        e.register(TES_GRID_X, |_, _| eax(0));
        e.register(WORLD_SPACE_CELL_AT_COORDINATES, |_, a| {
            // The cell itself at its own coordinates, a neighbour at (11, -2).
            match (a[1] as i32, a[2] as i32) {
                (10, -3) => eax(START_CELL),
                (11, -2) => eax(NEIGHBOUR),
                _ => eax(0),
            }
        });
        e
    }

    #[test]
    fn water_height_with_a_cell() {
        let position = 0x0400_0000u32;
        // Water over the given cell.
        let mut e = water_engine(&[(START_CELL, 12.5)]);
        let height = e.call(0x0045_cbc0, &args![TES, position, START_CELL]).f32();
        assert_eq!(height, 12.5);

        // Flag bit set: -FLT_MAX.
        let mut e = water_engine(&[]);
        e.register(CELL_FLAG_BIT_0, |_, _| eax(1));
        let height = e.call(0x0045_cbc0, &args![TES, position, START_CELL]).f32();
        assert_eq!(height.to_bits(), 0xff7f_ffff);

        // No water over it: the neighbour at (11, -2) answers; the cell
        // itself is skipped in the grid pass.
        let mut e = water_engine(&[(NEIGHBOUR, 3.0)]);
        e.call_log = Some(vec![]);
        let height = e.call(0x0045_cbc0, &args![TES, position, START_CELL]).f32();
        assert_eq!(height, 3.0);
        let lookups = calls_to(&e, WORLD_SPACE_CELL_AT_COORDINATES);
        let expected: Vec<Vec<u32>> = [
            (9, -4),
            (9, -3),
            (9, -2),
            (10, -4),
            (10, -3),
            (10, -2),
            (11, -4),
            (11, -3),
            (11, -2),
        ]
        .iter()
        .map(|&(x, y): &(i32, i32)| vec![WORLD, x as u32, y as u32])
        .collect();
        assert_eq!(lookups, expected);
        let checked: Vec<u32> = calls_to(&e, CELL_WATER_HEIGHT_AT)
            .iter()
            .map(|words| words[0])
            .collect();
        assert_eq!(checked, vec![START_CELL, NEIGHBOUR]);

        // Nothing anywhere: the cell's plain water height.
        let mut e = water_engine(&[]);
        let height = e.call(0x0045_cbc0, &args![TES, position, START_CELL]).f32();
        assert_eq!(height, -7.5);
    }

    #[test]
    fn water_height_without_a_cell() {
        let position = 0x0400_0000u32;
        // Interior: its answer, even when it found no water.
        let mut e = water_engine(&[]);
        let interior = 0x0300_4000u32;
        e.mem.set_u32(TES + 0x34, interior);
        e.call_log = Some(vec![]);
        let height = e.call(0x0045_cbc0, &args![TES, position, 0u32]).f32();
        assert_eq!(height, 1.0);
        assert_eq!(
            calls_to(&e, CELL_WATER_HEIGHT_AT)[0][..2],
            [interior, position]
        );

        // No cell under the position: the worldspace's default height.
        let mut e = water_engine(&[]);
        e.register(WORLD_SPACE_CELL_AT_POSITION, |_, _| eax(0));
        let height = e.call(0x0045_cbc0, &args![TES, position, 0u32]).f32();
        assert_eq!(height, -55.0);

        // The cell under the position, then its grid pass.
        let mut e = water_engine(&[(NEIGHBOUR, 4.0)]);
        e.call_log = Some(vec![]);
        let height = e.call(0x0045_cbc0, &args![TES, position, 0u32]).f32();
        assert_eq!(height, 4.0);
        assert_eq!(
            calls_to(&e, WORLD_SPACE_CELL_AT_POSITION),
            vec![vec![WORLD, position]]
        );
        assert_eq!(calls_to(&e, CELL_GET_X), vec![vec![START_CELL]]);
    }

    #[test]
    fn getter_reads_the_word_at_0x28() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x30));
        e.mem.set_u32(object.addr() + 0x28, 0x1234_5678);
        assert_eq!(e.call(0x0045_cd60, &args![object]).u32(), 0x1234_5678);
    }

    #[test]
    fn getter_reads_the_float_at_0x7c() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x80));
        e.mem.set_f32(object.addr() + 0x7c, -2.25);
        assert_eq!(e.call(0x0045_cd80, &args![object]).f32(), -2.25);
    }

    #[test]
    fn visits_the_interior_or_every_grid_cell() {
        let mut e = Engine::new();
        e.register(CELL_VISIT, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x34, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x0045_cda0, &args![this]);
        assert_eq!(calls_to(&e, CELL_VISIT), vec![vec![0x7000]]);
        assert!(calls_to(&e, GRID_CELL_ARRAY_GET).is_empty());

        // Exterior: a 2 x 2 grid with one empty slot.
        let size = e.mem.alloc(4);
        e.mem.set_u32(size, 2);
        e.register_double(SETTING_VALUE_ADDRESS, move |_, a| {
            assert_eq!(a[0], GRID_SIZE_SETTING);
            eax(size)
        });
        let slots = e.mem.alloc(16);
        for (i, cell) in [0x7100u32, 0, 0x7300, 0x7400].iter().enumerate() {
            e.mem.set_u32(slots + 4 * i as u32, *cell);
        }
        e.register_double(GRID_CELL_ARRAY_GET, move |_, a| {
            assert_eq!(a[0], 0x5000);
            eax(slots + 4 * (a[1] * 2 + a[2]))
        });
        e.mem.set_u32(this + 0x34, 0);
        e.mem.set_u32(this + 0x8, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0045_cda0, &args![this]);
        assert_eq!(
            calls_to(&e, GRID_CELL_ARRAY_GET),
            vec![
                vec![0x5000, 0, 0],
                vec![0x5000, 0, 1],
                vec![0x5000, 1, 0],
                vec![0x5000, 1, 1]
            ]
        );
        assert_eq!(
            calls_to(&e, CELL_VISIT),
            vec![vec![0x7100], vec![0x7300], vec![0x7400]]
        );
    }

    #[test]
    fn counter_at_0xb8_steps_up_and_down() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc0);
        e.call(0x0045_ce60, &args![this]);
        assert_eq!(e.mem.u32(this + 0xb8), u32::MAX);
        e.call(0x0045_ce40, &args![this]);
        e.call(0x0045_ce40, &args![this]);
        assert_eq!(e.mem.u32(this + 0xb8), 1);
    }

    #[test]
    fn stores_the_word_at_4_unless_null() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x0045_ce80, &args![this, 9u32]).u32(), this);
        assert_eq!(e.mem.u32(this + 4), 9);
        assert_eq!(e.call(0x0045_cea0, &args![this, 5u32]).u32(), this);
        assert_eq!(e.mem.u32(this + 4), 5);
        assert_eq!(e.call(0x0045_cea0, &args![0u32, 5u32]).u32(), 0);
    }

    #[test]
    fn destructor_releases_a_held_object_only() {
        let mut e = Engine::new();
        e.register(RELEASE_REFERENCE, |_, _| Ret::default());
        let holder: Ptr = Ptr::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        e.call(0x0045_cec0, &args![holder]);
        assert!(calls_to(&e, RELEASE_REFERENCE).is_empty());

        e.mem.set_u32(holder.addr(), 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0045_cec0, &args![holder]);
        assert_eq!(calls_to(&e, RELEASE_REFERENCE), vec![vec![0x5000]]);
    }

    #[test]
    fn freed_node_goes_on_the_free_list_under_the_lock() {
        let mut e = Engine::new();
        e.map(FREE_LIST_HEAD, 4);
        e.set_global(FREE_LIST_HEAD, 0x8800u32);
        e.register_double(LOCK_ACQUIRE, |e, _| {
            assert_eq!(e.global::<u32>(FREE_LIST_HEAD), 0x8800);
            Ret::default()
        });
        e.register(LOCK_RELEASE, |_, _| Ret::default());
        let node = e.mem.alloc(8);
        e.mem.set_u32(node + 4, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0045_cee0, &args![0x9999u32, node]);
        assert_eq!(e.mem.u32(node), 0x8800);
        assert_eq!(e.mem.u32(node + 4), 0);
        assert_eq!(e.global::<u32>(FREE_LIST_HEAD), node);
        assert_eq!(calls_to(&e, LOCK_ACQUIRE), vec![vec![FREE_LIST_LOCK, 0]]);
        assert_eq!(calls_to(&e, LOCK_RELEASE), vec![vec![FREE_LIST_LOCK]]);
    }

    const COLLECTION_ADD: u32 = 0x0060_0004;
    const COLLECTION_REMOVE: u32 = 0x0060_0008;

    /// A collection object at a fresh address whose vtable's `+4` and `+8`
    /// are the doubles above.
    fn collection(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(12);
        e.mem.set_u32(vtable + 4, COLLECTION_ADD);
        e.mem.set_u32(vtable + 8, COLLECTION_REMOVE);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, vtable);
        e.register(COLLECTION_ADD, |_, _| Ret::default());
        e.register(COLLECTION_REMOVE, |_, _| Ret::default());
        object
    }

    #[test]
    fn byte_setting_constructors() {
        let mut e = Engine::new();
        let this = e.mem.alloc(12);
        assert_eq!(
            e.call(0x0045_cfb0, &args![this, 0x0100_0000u32, 0x1ffu32])
                .u32(),
            this
        );
        assert_eq!(e.mem.u32(this), SETTING_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x0100_0000);
        assert_eq!(e.mem.u32(this + 4), 0xff);

        let ini = collection(&mut e);
        e.register_double(INI_COLLECTION_INSTANCE, move |_, _| eax(ini));
        let setting = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0045_cf30, &args![setting, 0x0100_0010u32, 1u32])
                .u32(),
            setting
        );
        assert_eq!(e.mem.u32(setting), INI_SETTING_VTABLE);
        assert_eq!(e.mem.u32(setting + 8), 0x0100_0010);
        assert_eq!(e.mem.u8(setting + 4), 1);
        assert_eq!(calls_to(&e, COLLECTION_ADD), vec![vec![ini, setting]]);
    }

    #[test]
    fn word_setting_constructor() {
        let mut e = Engine::new();
        e.register(SETTING_CONSTRUCT_INT, |e, a| {
            e.mem.set_u32(a[0], SETTING_VTABLE);
            eax(a[0])
        });
        let ini = collection(&mut e);
        e.register_double(INI_COLLECTION_INSTANCE, move |_, _| eax(ini));
        let setting = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0045_cfe0, &args![setting, 0x0100_0020u32, 42u32])
                .u32(),
            setting
        );
        assert_eq!(
            calls_to(&e, SETTING_CONSTRUCT_INT),
            vec![vec![setting, 0x0100_0020, 42]]
        );
        assert_eq!(e.mem.u32(setting), INI_SETTING_VTABLE);
        assert_eq!(calls_to(&e, COLLECTION_ADD), vec![vec![ini, setting]]);
    }

    #[test]
    fn renderer_setting_constructor_and_destructors() {
        let mut e = Engine::new();
        let renderer = collection(&mut e);
        e.map(RENDERER_COLLECTION_INSTANCE, 4);
        e.set_global(RENDERER_COLLECTION_INSTANCE, renderer);
        e.register(SETTING_CONSTRUCT_FLOAT, |e, a| {
            e.mem.set_u32(a[0], SETTING_VTABLE);
            eax(a[0])
        });
        e.register(SETTING_DESTRUCT, |e, a| {
            e.mem.set_u32(a[0], SETTING_VTABLE);
            Ret::default()
        });
        e.register(MEMORY_FREE, |_, _| Ret::default());
        let setting = e.mem.alloc(12);
        // A signaling NaN comes out of the x87 copy quiet.
        let signaling = 0x7f80_0001u32;
        e.call_log = Some(vec![]);
        let made = e
            .call(0x0045_d060, &args![setting, 0x0100_0030u32, signaling])
            .u32();
        assert_eq!(made, setting);
        assert_eq!(
            calls_to(&e, SETTING_CONSTRUCT_FLOAT),
            vec![vec![setting, 0x0100_0030, 0x7fc0_0001]]
        );
        assert_eq!(e.mem.u32(setting), RENDERER_SETTING_VTABLE);
        assert_eq!(calls_to(&e, COLLECTION_ADD), vec![vec![renderer, setting]]);

        // Without bit 0 the block is kept.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0045_d0e0, &args![setting, 2u32]).u32(), setting);
        assert_eq!(
            calls_to(&e, COLLECTION_REMOVE),
            vec![vec![renderer, setting]]
        );
        assert_eq!(calls_to(&e, SETTING_DESTRUCT), vec![vec![setting]]);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());

        e.call_log = Some(vec![]);
        e.call(0x0045_d0e0, &args![setting, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![setting]]);

        // The destructor body alone sets its vtable before removing itself.
        e.mem.set_u32(setting, 0);
        e.register_double(COLLECTION_REMOVE, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), RENDERER_SETTING_VTABLE);
            Ret::default()
        });
        e.call(0x0045_d110, &args![setting]);
        assert_eq!(e.mem.u32(setting), SETTING_VTABLE);
    }

    #[test]
    fn renderer_collection_singleton_is_made_once() {
        let mut e = Engine::new();
        e.map(RENDERER_COLLECTION_INSTANCE, 4);
        let block = e.mem.alloc(RENDERER_COLLECTION_SIZE);
        e.register_double(MEMORY_ALLOCATE, move |_, a| {
            assert_eq!(a[0], RENDERER_COLLECTION_SIZE);
            eax(block)
        });
        e.register(INI_COLLECTION_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0101_772c);
            eax(a[0])
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0045_d180, &[]).u32(), block);
        assert_eq!(e.mem.u32(block), RENDERER_COLLECTION_VTABLE);
        assert_eq!(e.call(0x0045_d180, &[]).u32(), block);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE).len(), 1);
        assert_eq!(calls_to(&e, INI_COLLECTION_CONSTRUCT), vec![vec![block]]);

        // A failed allocation leaves the singleton null.
        e.set_global(RENDERER_COLLECTION_INSTANCE, 0u32);
        e.register(MEMORY_ALLOCATE, |_, _| eax(0));
        e.call(0x0045_d190, &[]);
        assert_eq!(e.global::<u32>(RENDERER_COLLECTION_INSTANCE), 0);
    }

    #[test]
    fn renderer_collection_constructor_and_destructor_body() {
        let mut e = Engine::new();
        e.register(INI_COLLECTION_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0101_772c);
            eax(a[0])
        });
        e.register(COLLECTION_DESTRUCT, |_, _| eax(0));
        let this = e.mem.alloc(RENDERER_COLLECTION_SIZE);
        assert_eq!(e.call(0x0045_d220, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), RENDERER_COLLECTION_VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x0045_d240, &args![this]);
        assert_eq!(calls_to(&e, COLLECTION_DESTRUCT), vec![vec![this]]);
    }
}
