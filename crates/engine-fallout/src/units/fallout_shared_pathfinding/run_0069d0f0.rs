//! `fallout shared/pathfinding/run_0069d0f0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A run of small pathfinding helpers the engine map does not name: the
//! element construction and initialization of an array of 6-byte elements,
//! the two `EdgeExtraInfo` loaders the navmesh loader runs on the elements
//! of `NavMesh::ExtraEdgeInfo` (`+0x48`), and the search grid's
//! (`NavMeshGrid`, layout in `navmesh.rs`) constructor, reset, cell lookup,
//! cell collection, cell corner and set-up.
//!
//! x87 note: the grid code computes in extended precision and stores
//! `float`s; the translations compute in `f64` and round to `f32` at each
//! `float` store.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_pathfinding::navmesh::{EdgeExtraInfo, NavMeshGrid};

/// `006e6da0(size, element)` (cdecl): the placement `operator new` run on
/// each element (returns the element).
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `00401680(file)`: whether the file being read has the other byte order.
const FILE_SWAPS: u32 = 0x0040_1680;
/// `00401080(address, word)` (cdecl): reverses the bytes of the 32-bit
/// value at `address`.
const SWAP_32: u32 = 0x0040_1080;
/// The `TES` singleton pointer.
const TES_SINGLETON: u32 = 0x011d_ea10;
/// `TES::GetNavMeshInfoMap` (`0045af00`, Xbox PDB name in the map; thiscall
/// on the `TES`, no stack arguments).
const TES_GET_NAV_MESH_INFO_MAP: u32 = 0x0045_af00;
/// `NavMeshTriangleEdgePortal::Load` (`0069dfd0`, Xbox PDB name in the map;
/// thiscall `(portal, info map, file, data)`).
const EDGE_PORTAL_LOAD: u32 = 0x0069_dfd0;
/// `006815c0(point)`: the constructor run on an `NiPoint3` member.
const POINT_CONSTRUCTOR: u32 = 0x0068_15c0;
/// The `float` the grid's minimum x bound is reset to (`FLT_MAX` in the
/// exe's data).
const GRID_RESET_BOUND: u32 = 0x0101_6970;
/// `operator delete(block)` (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `operator new(size)` (`00401000`, cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00406ce0(value)` (cdecl, `float` argument): `floor`, result in ST0.
const FLOOR: u32 = 0x0040_6ce0;
/// `_ftol2` (`00ec62c0`): truncates ST0 to an integer (the value is passed
/// as a leading `f64`).
const FTOL: u32 = 0x00ec_62c0;
/// `006967c0(grid, column, row)`: the address of a grid cell.
const GRID_CELL: u32 = 0x0069_67c0;
/// `00647ba0(&value, minimum, maximum)` (cdecl): clamps an `int` in place.
const CLAMP_INT: u32 = 0x0064_7ba0;
/// `00647bd0(&value, minimum, maximum)` (cdecl): clamps an unsigned word in
/// place.
const CLAMP_UINT: u32 = 0x0064_7bd0;
/// `0044ddc0(array)`: an array's `iSize`.
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `0069dec0(array, index)`: the address of element `index` of a
/// `BSSimpleArray<unsigned short>`.
const SHORT_ELEMENT: u32 = 0x0069_dec0;
/// `00691820(list, &triangle)`: appends the triangle to a triangle list.
const LIST_ADD_TRIANGLE: u32 = 0x0069_1820;
/// `0076b610(array)`: whether the array's `iSize` is 0.
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
/// `0069aeb0(list, compare)`: sorts the list with the compare function.
const LIST_SORT: u32 = 0x0069_aeb0;
/// The compare function passed to [`LIST_SORT`].
const COMPARE_TRIANGLES: u32 = 0x0069_ae60;
/// `00438660(array)` / `00438690(array)`: constructor and destructor of a
/// `BSSimpleArray<unsigned int>` (the local list of duplicate indexes).
const INDEX_ARRAY_CONSTRUCTOR: u32 = 0x0043_8660;
const INDEX_ARRAY_DESTRUCTOR: u32 = 0x0043_8690;
/// `007cb2e0(array, &value)`: appends a word.
const INDEX_ARRAY_ADD: u32 = 0x007c_b2e0;
/// `00877a30(array, index)`: the address of element `index` of a 4-byte
/// element array.
const WORD_ELEMENT: u32 = 0x0087_7a30;
/// `0069add0(list, index, keep_order)`: removes element `index`.
const LIST_REMOVE_AT: u32 = 0x0069_add0;
/// `00439ef0(point, out, other)`: `out = point - other` (`NiPoint3`).
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
/// `004941c0(point, divisor)`: divides the point by a `float` in place.
const VECTOR_DIVIDE: u32 = 0x0049_41c0;
/// `00408840(value)` (cdecl, `float` argument): `fabs`, result in ST0.
const FABS: u32 = 0x0040_8840;
/// `00416870(point, x, y, z)`: `NiPoint3` constructor from three floats.
const POINT_FROM_FLOATS: u32 = 0x0041_6870;
/// `00ec782f(array, element size, count, constructor, destructor)`: the
/// compiler's vector constructor iterator.
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
/// `006917d0` / `00691800`: constructor and destructor of a grid cell.
const CELL_CONSTRUCT: u32 = 0x0069_17d0;
const CELL_DESTRUCT: u32 = 0x0069_1800;
/// Bytes of one grid cell.
const CELL_SIZE: u32 = 0x10;

/// `FISTP qword` under truncation, keeping the low dword: values outside
/// the 64-bit range (and NaN) give the integer indefinite, whose low dword
/// is 0.
fn truncate_low_dword(value: f64) -> u32 {
    const LIMIT: f64 = 9_223_372_036_854_775_808.0;
    if value.is_nan() || !(-LIMIT..LIMIT).contains(&value) {
        0
    } else {
        value.trunc() as i64 as u32
    }
}

/// `floor` of a `float` (the call's ST0 result).
fn floor(e: &mut Engine, value: f32) -> f64 {
    e.call(FLOOR, &args![value]).f64()
}

// Translated from 0069d0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs the placement `operator new`
/// (`006e6da0(6, element)`) on each of the `count` 6-byte elements starting
/// at `data`. `this` is not read. `RET 8`.
pub fn fn_0069d0f0(e: &mut Engine, _this: Ptr, data: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        e.call(
            PLACEMENT_NEW,
            &args![6u32, index.wrapping_mul(6).wrapping_add(data)],
        );
        index += 1;
    }
}

// Translated from 0069d140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: initializes a `BSSimpleArray` of 6-byte
/// elements: clears buffer, size and capacity; reserves `max(reserve,
/// size)` elements through the array's virtual allocator (vtable `+4`,
/// one word) when that is not 0, setting the capacity; and constructs
/// `size` elements ([`fn_0069d0f0`]) when `size` is not 0, setting the size.
/// `RET 8`.
pub fn fn_0069d140(
    e: &mut Engine,
    this: Ptr<crate::types::BSSimpleArray>,
    reserve: u32,
    size: u32,
) {
    use crate::types::BSSimpleArray;
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let capacity = if size > reserve { size } else { reserve };
    if capacity != 0 {
        let buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    }
    if size != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        fn_0069d0f0(e, this.cast(), buffer, size);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 0069d1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: loads an `EdgeExtraInfo` (the navmesh loader
/// runs it on an element of `NavMesh::ExtraEdgeInfo`) from the chunk at
/// `data`: the first word is `eType` (byte-reversed in place, `00401080`,
/// when `file` has the other byte order, `00401680`), then the portal is
/// loaded from the following bytes by `NavMeshTriangleEdgePortal::Load`
/// with the `TES` singleton's navmesh info map. `RET 8`.
pub fn fn_0069d1d0(e: &mut Engine, this: Ptr<EdgeExtraInfo>, file: Ptr, data: u32) {
    let kind = e.mem.u32(data);
    e.set(this, EdgeExtraInfo::eType, kind);
    if e.call(FILE_SWAPS, &args![file]).bool() {
        e.call(SWAP_32, &args![this, 0u32]);
    }
    load_portal(e, this, file, data.wrapping_add(4));
}

/// The common tail of the two loaders: `NavMeshTriangleEdgePortal::Load` on
/// the portal at `+4` with the `TES` singleton's navmesh info map.
fn load_portal(e: &mut Engine, this: Ptr<EdgeExtraInfo>, file: Ptr, data: u32) {
    let tes: u32 = e.global(TES_SINGLETON);
    let info_map = e.call(TES_GET_NAV_MESH_INFO_MAP, &args![tes]).u32();
    e.call(
        EDGE_PORTAL_LOAD,
        &args![this.addr().wrapping_add(4), info_map, file, data],
    );
}

// Translated from 0069d240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the other `EdgeExtraInfo` loader: sets
/// `eType` to 0 and loads the portal from `data` itself (see
/// [`fn_0069d1d0`]). `RET 8`.
pub fn fn_0069d240(e: &mut Engine, this: Ptr<EdgeExtraInfo>, file: Ptr, data: u32) {
    e.set(this, EdgeExtraInfo::eType, 0);
    load_portal(e, this, file, data);
}

// Translated from 0069d280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NavMeshGrid` constructor: `iGridSize` 0,
/// the point constructor (`006815c0`) on both bounds, `GridData` null and
/// the minimum x bound set to the `float` at `01016970` (`FLT_MAX`).
/// Returns `this`.
pub fn fn_0069d280(e: &mut Engine, this: Ptr<NavMeshGrid>) -> Ptr<NavMeshGrid> {
    e.set(this, NavMeshGrid::iGridSize, 0);
    e.call(POINT_CONSTRUCTOR, &args![this.addr().wrapping_add(0xc)]);
    e.call(POINT_CONSTRUCTOR, &args![this.addr().wrapping_add(0x18)]);
    e.set(this, NavMeshGrid::GridData, Ptr::NULL);
    let bound = x87_quiet(e.global::<f32>(GRID_RESET_BOUND));
    e.set(this, NavMeshGrid::GridBoundsMin_x, bound);
    this
}

// Translated from 0069d2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: resets the grid: when `GridData` is set, it is
/// deleted (with no cell count in the word before it, `operator delete` on
/// that word's address; otherwise the cells' vector deleting destructor,
/// vtable slot 0 of the first cell, with flags 3) and cleared; then
/// `iGridSize` is 0 and the minimum x bound the `float` at `01016970`.
pub fn fn_0069d2d0(e: &mut Engine, this: Ptr<NavMeshGrid>) {
    let cells = e.get(this, NavMeshGrid::GridData);
    if !cells.is_null() {
        let first = cells.addr();
        if e.mem.u32(first.wrapping_sub(4)) == 0 {
            e.call(OPERATOR_DELETE, &args![first.wrapping_sub(4)]);
        } else {
            e.vcall(first, 0, &args![3u32]);
        }
        e.set(this, NavMeshGrid::GridData, Ptr::NULL);
    }
    e.set(this, NavMeshGrid::iGridSize, 0);
    let bound = x87_quiet(e.global::<f32>(GRID_RESET_BOUND));
    e.set(this, NavMeshGrid::GridBoundsMin_x, bound);
}

/// The column and row offsets of `point` in cell units: `(x - min.x) /
/// fColumnSectionLen` and `(y - min.y) / fRowSectionLen`, each step stored
/// as a `float`.
fn cell_offsets(e: &mut Engine, this: Ptr<NavMeshGrid>, point: u32) -> (f32, f32) {
    let x = e.mem.f32(point);
    let y = e.mem.f32(point.wrapping_add(4));
    let min_x = e.get(this, NavMeshGrid::GridBoundsMin_x);
    let min_y = e.get(this, NavMeshGrid::GridBoundsMin_y);
    let dx = (f64::from(x) - f64::from(min_x)) as f32;
    let dy = (f64::from(y) - f64::from(min_y)) as f32;
    let column_length = e.get(this, NavMeshGrid::fColumnSectionLen);
    let row_length = e.get(this, NavMeshGrid::fRowSectionLen);
    let column = (f64::from(dx) / f64::from(column_length)) as f32;
    let row = (f64::from(dy) / f64::from(row_length)) as f32;
    (column, row)
}

// Translated from 0069d370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: writes to `out` the grid cell (`006967c0`)
/// holding the x/y of `point`, or 0 when the point is outside the grid's
/// x/y bounds (a NaN coordinate counts as inside). The cell indexes are the
/// floors (`00406ce0`) of the offsets in cell units, truncated as 64-bit
/// integers, and limited to `iGridSize - 1` (compared unsigned). `RET 8`.
pub fn fn_0069d370(e: &mut Engine, this: Ptr<NavMeshGrid>, point: Ptr, out: Ptr) {
    let p = point.addr();
    let x = e.mem.f32(p);
    let y = e.mem.f32(p.wrapping_add(4));
    let outside = x < e.get(this, NavMeshGrid::GridBoundsMin_x)
        || x > e.get(this, NavMeshGrid::GridBoundsMax_x)
        || y < e.get(this, NavMeshGrid::GridBoundsMin_y)
        || y > e.get(this, NavMeshGrid::GridBoundsMax_y);
    if outside {
        e.mem.set_u32(out.addr(), 0);
        return;
    }
    let (column_offset, row_offset) = cell_offsets(e, this, p);
    let mut column = truncate_low_dword(floor(e, column_offset));
    let mut row = truncate_low_dword(floor(e, row_offset));
    let size = e.get(this, NavMeshGrid::iGridSize);
    if column >= size {
        column = size.wrapping_sub(1);
    }
    if row >= size {
        row = size.wrapping_sub(1);
    }
    let cell = e.call(GRID_CELL, &args![this, column, row]).u32();
    e.mem.set_u32(out.addr(), cell);
}

/// `_ftol2` of the floor of a cell offset.
fn floor_to_int(e: &mut Engine, value: f32) -> i32 {
    let floored = floor(e, value);
    e.call(FTOL, &args![floored]).i32()
}

// Translated from 0069d4b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0069d4b0 - FS:[0] exception frame (unwinding of the local index array) is not translated.
/// Unnamed in the engine map: appends to `list` (`00691820`) the
/// triangles of every grid cell in the rectangle of cells between two
/// corners. The code computes both corners from `point` (the second
/// argument, `_unused_2`, is never read), so the rectangle is the one cell
/// of `point`; each index is the `_ftol2` of the floor of its offset in
/// cell units, clamped to `0..iGridSize - 1` (`00647ba0`, signed loops).
/// Unless the list is then
/// empty (`0076b610`), it is sorted (`0069aeb0` with `0069ae60`) and every
/// element equal to the one before it is removed: their indexes are
/// gathered in a local `BSSimpleArray<unsigned int>` (`00438660`,
/// `007cb2e0`) and removed from the last to the first (`0069add0(list,
/// index, 1)`), then the local array is destroyed (`00438690`). `RET 0xc`.
pub fn fn_0069d4b0(e: &mut Engine, this: Ptr<NavMeshGrid>, point: Ptr, _unused_2: u32, list: Ptr) {
    // The stack locals: the duplicate index array (0x10 bytes), the word
    // passed to its append, and the four clamped indexes.
    e.with_stack(0x28, |e, stack| {
        let s = stack.addr();
        let duplicates = s;
        let appended = s + 0x10;
        let low_column = s + 0x14;
        let low_row = s + 0x18;
        let high_column = s + 0x1c;
        let high_row = s + 0x20;
        let size = e.get(this, NavMeshGrid::iGridSize);
        let last = size.wrapping_sub(1);

        let (column_offset, row_offset) = cell_offsets(e, this, point.addr());
        let column = floor_to_int(e, column_offset);
        e.mem.set_u32(low_column, column as u32);
        let row = floor_to_int(e, row_offset);
        e.mem.set_u32(low_row, row as u32);
        e.call(CLAMP_INT, &args![low_row, 0u32, last]);
        e.call(CLAMP_INT, &args![low_column, 0u32, last]);

        let (column_offset, row_offset) = cell_offsets(e, this, point.addr());
        let column = floor_to_int(e, column_offset);
        e.mem.set_u32(high_column, column as u32);
        let row = floor_to_int(e, row_offset);
        e.mem.set_u32(high_row, row as u32);
        e.call(CLAMP_INT, &args![high_row, 0u32, last]);
        e.call(CLAMP_INT, &args![high_column, 0u32, last]);

        let mut row = e.mem.u32(low_row) as i32;
        while row <= e.mem.u32(high_row) as i32 {
            let mut column = e.mem.u32(low_column) as i32;
            while column <= e.mem.u32(high_column) as i32 {
                let cell = e.call(GRID_CELL, &args![this, column, row]).u32();
                let mut index = 0u32;
                while index < e.call(ARRAY_SIZE, &args![cell]).u32() {
                    let triangle = e.call(SHORT_ELEMENT, &args![cell, index]).u32();
                    e.call(LIST_ADD_TRIANGLE, &args![list, triangle]);
                    index += 1;
                }
                column += 1;
            }
            row += 1;
        }

        if e.call(ARRAY_IS_EMPTY, &args![list]).bool() {
            return;
        }
        e.call(LIST_SORT, &args![list, COMPARE_TRIANGLES]);
        e.call(INDEX_ARRAY_CONSTRUCTOR, &args![duplicates]);
        let mut index = 0u32;
        while index < e.call(ARRAY_SIZE, &args![list]).u32().wrapping_sub(1) {
            let next = e.call(SHORT_ELEMENT, &args![list, index + 1]).u32();
            let next = e.mem.u16(next);
            let current = e.call(SHORT_ELEMENT, &args![list, index]).u32();
            if next == e.mem.u16(current) {
                e.mem.set_u32(appended, index + 1);
                e.call(INDEX_ARRAY_ADD, &args![duplicates, appended]);
            }
            index += 1;
        }
        if !e.call(ARRAY_IS_EMPTY, &args![duplicates]).bool() {
            let count = e.call(ARRAY_SIZE, &args![duplicates]).u32();
            let mut position = count.wrapping_sub(1) as i32;
            while position >= 0 {
                let slot = e.call(WORD_ELEMENT, &args![duplicates, position]).u32();
                let removed = e.mem.u32(slot);
                e.call(LIST_REMOVE_AT, &args![list, removed, 1u32]);
                position -= 1;
            }
        }
        e.call(INDEX_ARRAY_DESTRUCTOR, &args![duplicates]);
    });
}

// Translated from 0069d790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: writes to `out` (`00416870`) the corner of
/// cell (`column`, `row`): `column * fColumnSectionLen + min.x`, `row *
/// fRowSectionLen + min.y` (indexes unsigned, each stored as a `float`) and
/// the minimum z bound. Returns `out`. `RET 0xc`.
pub fn fn_0069d790(e: &mut Engine, this: Ptr<NavMeshGrid>, out: Ptr, column: u32, row: u32) -> Ptr {
    let z = x87_quiet(e.get(this, NavMeshGrid::GridBoundsMin_z));
    let row_length = e.get(this, NavMeshGrid::fRowSectionLen);
    let min_y = e.get(this, NavMeshGrid::GridBoundsMin_y);
    let y = (f64::from(row) * f64::from(row_length) + f64::from(min_y)) as f32;
    let column_length = e.get(this, NavMeshGrid::fColumnSectionLen);
    let min_x = e.get(this, NavMeshGrid::GridBoundsMin_x);
    let x = (f64::from(column) * f64::from(column_length) + f64::from(min_x)) as f32;
    e.call(
        POINT_FROM_FLOATS,
        &args![out, x87_quiet(x), x87_quiet(y), z],
    );
    out
}

/// Bytes `operator new` is asked for: the count word and `cells` 16-byte
/// cells, `0xFFFFFFFF` when either step overflows.
fn allocation_bytes(cells: u32) -> u32 {
    cells
        .checked_mul(CELL_SIZE)
        .and_then(|bytes| bytes.checked_add(4))
        .unwrap_or(u32::MAX)
}

// Translated from 0069d800 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0069d800 - FS:[0] exception frame (unwinding of the cell array allocation) is not translated.
/// Unnamed in the engine map: sets the grid up over the box `min`..`max`
/// for a mesh of `triangle_count` triangles: copies both bounds, sizes the
/// grid (1 for up to 16 triangles, else `triangle_count / 50 + 2` clamped
/// to 2..12 by `00647bd0`), divides the box's size (`max - min`, `00439ef0`)
/// by the grid size (`004941c0`, the size as a `float`) and stores the
/// absolute x and y (`00408840`) as the column and row lengths. Then it
/// allocates `size * size` cells behind a count word (`operator new`,
/// `0xFFFFFFFF` bytes when the size overflows) and builds them with the
/// vector constructor iterator (`00ec782f`, cell constructor `006917d0`,
/// destructor `00691800`); `GridData` is the first cell, or null when the
/// allocation fails. `RET 0xc`.
pub fn fn_0069d800(
    e: &mut Engine,
    this: Ptr<NavMeshGrid>,
    min: Ptr,
    max: Ptr,
    triangle_count: u32,
) {
    let t = this.addr();
    for word in 0..3 {
        let value = e.mem.u32(min.addr().wrapping_add(4 * word));
        e.mem.set_u32(t.wrapping_add(0xc + 4 * word), value);
    }
    for word in 0..3 {
        let value = e.mem.u32(max.addr().wrapping_add(4 * word));
        e.mem.set_u32(t.wrapping_add(0x18 + 4 * word), value);
    }
    e.with_stack(0xc, |e, extent| {
        e.call(
            VECTOR_SUBTRACT,
            &args![t.wrapping_add(0x18), extent, t.wrapping_add(0xc)],
        );
        if triangle_count <= 0x10 {
            e.set(this, NavMeshGrid::iGridSize, 1);
        } else {
            e.set(this, NavMeshGrid::iGridSize, triangle_count / 0x32 + 2);
            e.call(CLAMP_UINT, &args![this, 2u32, 0xcu32]);
        }
        let size = e.get(this, NavMeshGrid::iGridSize);
        e.call(VECTOR_DIVIDE, &args![extent, size as f32]);
        let extent_x = x87_quiet(e.mem.f32(extent.addr()));
        let column_length = e.call(FABS, &args![extent_x]).f64() as f32;
        e.set(this, NavMeshGrid::fColumnSectionLen, column_length);
        let extent_y = x87_quiet(e.mem.f32(extent.addr() + 4));
        let row_length = e.call(FABS, &args![extent_y]).f64() as f32;
        e.set(this, NavMeshGrid::fRowSectionLen, row_length);
    });
    let size = e.get(this, NavMeshGrid::iGridSize);
    let cells = size.wrapping_mul(size);
    let block = e.call(OPERATOR_NEW, &args![allocation_bytes(cells)]).u32();
    let grid = if block == 0 {
        0
    } else {
        e.mem.set_u32(block, cells);
        e.call(
            VECTOR_CONSTRUCT,
            &args![block + 4, CELL_SIZE, cells, CELL_CONSTRUCT, CELL_DESTRUCT],
        );
        block + 4
    };
    e.set(this, NavMeshGrid::GridData, Ptr::new(grid));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0069d0f0, fn_0069d0f0(Ptr, u32, u32)),
        entry!(
            0x0069d140,
            fn_0069d140(Ptr<crate::types::BSSimpleArray>, u32, u32)
        ),
        entry!(0x0069d1d0, fn_0069d1d0(Ptr<EdgeExtraInfo>, Ptr, u32)),
        entry!(0x0069d240, fn_0069d240(Ptr<EdgeExtraInfo>, Ptr, u32)),
        entry!(
            0x0069d280,
            fn_0069d280(Ptr<NavMeshGrid>) -> Ptr<NavMeshGrid>
        ),
        entry!(0x0069d2d0, fn_0069d2d0(Ptr<NavMeshGrid>)),
        entry!(0x0069d370, fn_0069d370(Ptr<NavMeshGrid>, Ptr, Ptr)),
        entry!(0x0069d4b0, fn_0069d4b0(Ptr<NavMeshGrid>, Ptr, u32, Ptr)),
        entry!(
            0x0069d790,
            fn_0069d790(Ptr<NavMeshGrid>, Ptr, u32, u32) -> Ptr
        ),
        entry!(0x0069d800, fn_0069d800(Ptr<NavMeshGrid>, Ptr, Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::BSSimpleArray;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet_doubles(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn take_log(e: &mut Engine) -> Log {
        e.call_log.take().unwrap()
    }

    #[test]
    fn construct_elements_runs_placement_new_on_each() {
        let mut e = Engine::new();
        quiet_doubles(&mut e, &[PLACEMENT_NEW]);
        e.call_log = Some(vec![]);
        e.call(0x0069d0f0, &args![0u32, 0x9000u32, 3u32]);
        let log = take_log(&mut e);
        assert_eq!(
            calls(&log, PLACEMENT_NEW),
            vec![vec![6, 0x9000], vec![6, 0x9006], vec![6, 0x900c]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0069d0f0, &args![0u32, 0x9000u32, 0u32]);
        assert!(calls(&take_log(&mut e), PLACEMENT_NEW).is_empty());
    }

    #[test]
    fn initialize_reserves_the_larger_count_and_constructs_size() {
        let mut e = Engine::new();
        quiet_doubles(&mut e, &[PLACEMENT_NEW]);
        e.register(0x0077_0000, |_, a| (0x5000 + a[1]).into_ret());
        e.put_vtable(0x0077_1000, &[0, 0x0077_0000]);
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(array.addr(), 0x0077_1000);
        for (reserve, size, capacity, constructed) in [(2u32, 3u32, 3u32, 3usize), (5, 1, 5, 1)] {
            e.call_log = Some(vec![]);
            e.call(0x0069d140, &args![array, reserve, size]);
            let log = take_log(&mut e);
            assert_eq!(calls(&log, 0x0077_0000), vec![vec![array.addr(), capacity]]);
            assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0x5000 + capacity);
            assert_eq!(e.get(array, BSSimpleArray::iReservedSize), capacity);
            assert_eq!(e.get(array, BSSimpleArray::iSize), size);
            let built = calls(&log, PLACEMENT_NEW);
            assert_eq!(built.len(), constructed);
            assert_eq!(built[0], vec![6, 0x5000 + capacity]);
        }
        // Nothing reserved, nothing built: all three words cleared.
        e.call_log = Some(vec![]);
        e.call(0x0069d140, &args![array, 0u32, 0u32]);
        let log = take_log(&mut e);
        assert!(calls(&log, 0x0077_0000).is_empty());
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 0);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 0);
    }

    fn loader_engine(swaps: bool) -> Engine {
        let mut e = Engine::new();
        e.map(TES_SINGLETON & !0xfff, 0x1000);
        e.set_global(TES_SINGLETON, 0x4440u32);
        if swaps {
            e.register(FILE_SWAPS, |_, _| 1u32.into_ret());
        } else {
            e.register(FILE_SWAPS, |_, _| 0u32.into_ret());
        }
        quiet_doubles(&mut e, &[SWAP_32, EDGE_PORTAL_LOAD]);
        e.register(TES_GET_NAV_MESH_INFO_MAP, |_, _| 0x7770u32.into_ret());
        e
    }

    #[test]
    fn edge_extra_info_load_reads_the_type_then_the_portal() {
        for swaps in [false, true] {
            let mut e = loader_engine(swaps);
            let info: Ptr<EdgeExtraInfo> = e.new_object();
            let data = e.mem.alloc(16);
            e.mem.set_u32(data, 2);
            e.call_log = Some(vec![]);
            e.call(0x0069d1d0, &args![info, 0x3330u32, data]);
            let log = take_log(&mut e);
            assert_eq!(e.get(info, EdgeExtraInfo::eType), 2);
            assert_eq!(calls(&log, FILE_SWAPS), vec![vec![0x3330]]);
            let swapped = calls(&log, SWAP_32);
            if swaps {
                assert_eq!(swapped, vec![vec![info.addr(), 0]]);
            } else {
                assert!(swapped.is_empty());
            }
            assert_eq!(calls(&log, TES_GET_NAV_MESH_INFO_MAP), vec![vec![0x4440]]);
            assert_eq!(
                calls(&log, EDGE_PORTAL_LOAD),
                vec![vec![info.addr() + 4, 0x7770, 0x3330, data + 4]]
            );
        }
    }

    #[test]
    fn edge_extra_info_load_without_type() {
        let mut e = loader_engine(false);
        let info: Ptr<EdgeExtraInfo> = e.new_object();
        e.set(info, EdgeExtraInfo::eType, 9);
        e.call_log = Some(vec![]);
        e.call(0x0069d240, &args![info, 0x3330u32, 0x8000u32]);
        let log = take_log(&mut e);
        assert_eq!(e.get(info, EdgeExtraInfo::eType), 0);
        assert!(calls(&log, FILE_SWAPS).is_empty());
        assert_eq!(
            calls(&log, EDGE_PORTAL_LOAD),
            vec![vec![info.addr() + 4, 0x7770, 0x3330, 0x8000]]
        );
    }

    fn grid_engine() -> Engine {
        let mut e = Engine::new();
        e.map(GRID_RESET_BOUND & !0xfff, 0x1000);
        // A signalling NaN shows the FLD/FSTP copy.
        e.set_global(GRID_RESET_BOUND, 0x7f80_0001u32);
        e.register(FLOOR, |_, a| {
            f64::from(f32::from_bits(a[0])).floor().into_ret()
        });
        e.register(FTOL, |_, a| {
            let value = f64::take(a, &mut 0);
            (value as i32).into_ret()
        });
        e
    }

    #[test]
    fn grid_constructor_and_reset() {
        let mut e = grid_engine();
        quiet_doubles(&mut e, &[POINT_CONSTRUCTOR, OPERATOR_DELETE]);
        let grid: Ptr<NavMeshGrid> = e.new_object();
        e.set(grid, NavMeshGrid::iGridSize, 4);
        e.set(grid, NavMeshGrid::GridData, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069d280, &args![grid]).u32(), grid.addr());
        let log = take_log(&mut e);
        assert_eq!(
            calls(&log, POINT_CONSTRUCTOR),
            vec![vec![grid.addr() + 0xc], vec![grid.addr() + 0x18]]
        );
        assert_eq!(e.get(grid, NavMeshGrid::iGridSize), 0);
        assert!(e.get(grid, NavMeshGrid::GridData).is_null());
        assert_eq!(e.mem.u32(grid.addr() + 0xc), 0x7fc0_0001);

        // Reset with no cells: only the size and bound.
        e.set(grid, NavMeshGrid::iGridSize, 4);
        e.mem.set_u32(grid.addr() + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0069d2d0, &args![grid]);
        let log = take_log(&mut e);
        assert_eq!(log.len(), 1);
        assert_eq!(e.get(grid, NavMeshGrid::iGridSize), 0);
        assert_eq!(e.mem.u32(grid.addr() + 0xc), 0x7fc0_0001);

        // A count of 0 before the cells: operator delete on the count word.
        let block = e.mem.alloc(0x20);
        e.set(grid, NavMeshGrid::GridData, Ptr::new(block + 4));
        e.call_log = Some(vec![]);
        e.call(0x0069d2d0, &args![grid]);
        let log = take_log(&mut e);
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![block]]);
        assert!(e.get(grid, NavMeshGrid::GridData).is_null());

        // Cells counted: their vector deleting destructor with flags 3.
        e.register(0x0077_2000, |_, _| Ret::default());
        e.put_vtable(0x0077_3000, &[0x0077_2000]);
        e.mem.set_u32(block, 2);
        e.mem.set_u32(block + 4, 0x0077_3000);
        e.set(grid, NavMeshGrid::GridData, Ptr::new(block + 4));
        e.call_log = Some(vec![]);
        e.call(0x0069d2d0, &args![grid]);
        let log = take_log(&mut e);
        assert_eq!(calls(&log, 0x0077_2000), vec![vec![block + 4, 3]]);
        assert!(calls(&log, OPERATOR_DELETE).is_empty());
        assert!(e.get(grid, NavMeshGrid::GridData).is_null());
    }

    /// A 4x4 grid over (0,0)..(40,20): cells 10 wide and 5 deep.
    fn sample_grid(e: &mut Engine) -> Ptr<NavMeshGrid> {
        let grid: Ptr<NavMeshGrid> = e.new_object();
        e.set(grid, NavMeshGrid::iGridSize, 4);
        e.set(grid, NavMeshGrid::fColumnSectionLen, 10.0);
        e.set(grid, NavMeshGrid::fRowSectionLen, 5.0);
        e.set(grid, NavMeshGrid::GridBoundsMax_x, 40.0);
        e.set(grid, NavMeshGrid::GridBoundsMax_y, 20.0);
        e.set(grid, NavMeshGrid::GridBoundsMin_z, -3.0);
        grid
    }

    fn point(e: &mut Engine, x: f32, y: f32) -> u32 {
        let p = e.mem.alloc(12);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        p
    }

    #[test]
    fn cell_of_point() {
        let mut e = grid_engine();
        e.register(GRID_CELL, |_, a| (0x1000 * a[1] + a[2]).into_ret());
        let grid = sample_grid(&mut e);
        let out = e.mem.alloc(4);
        let cases: [(f32, f32, u32); 6] = [
            (15.0, 7.0, 0x1001),
            // On the maximum bounds: the indexes 4 are limited to 3.
            (40.0, 20.0, 0x3003),
            (0.0, 0.0, 0),
            // Outside: x below, x above, then y above.
            (-1.0, 3.0, 0),
            (41.0, 3.0, 0),
            (3.0, 21.0, 0),
        ];
        for (x, y, want) in cases {
            e.mem.set_u32(out, 0xdead);
            let p = point(&mut e, x, y);
            e.call(0x0069d370, &args![grid, p, out]);
            assert_eq!(e.mem.u32(out), want, "{x} {y}");
        }
        // A NaN coordinate passes the bound tests; its index (the integer
        // indefinite's low word, 0) is used as is.
        let p = point(&mut e, f32::NAN, 7.0);
        e.call(0x0069d370, &args![grid, p, out]);
        assert_eq!(e.mem.u32(out), 1);
        assert_eq!(truncate_low_dword(-1.5), u32::MAX);
        assert_eq!(truncate_low_dword(1e30), 0);
    }

    /// Doubles for the list, cell and duplicate-index arrays of
    /// [`fn_0069d4b0`]: each a `BSSimpleArray` kept in memory (buffer at +4,
    /// size at +8); the cell at `cell` holds the shorts in `cell_buffer`.
    fn list_doubles(e: &mut Engine, cell: u32, cell_buffer: u32) {
        e.register_double(GRID_CELL, move |_, a| {
            assert_eq!((a[1], a[2]), (1, 1));
            cell.into_ret()
        });
        e.register(CLAMP_INT, |e, a| {
            let value = e.mem.u32(a[0]) as i32;
            let clamped = value.clamp(a[1] as i32, a[2] as i32);
            e.mem.set_u32(a[0], clamped as u32);
            Ret::default()
        });
        e.register_double(ARRAY_SIZE, move |e, a| {
            if a[0] == cell {
                3u32.into_ret()
            } else {
                e.mem.u32(a[0] + 8).into_ret()
            }
        });
        e.register_double(SHORT_ELEMENT, move |e, a| {
            if a[0] == cell {
                (cell_buffer + 2 * a[1]).into_ret()
            } else {
                (e.mem.u32(a[0] + 4) + 2 * a[1]).into_ret()
            }
        });
        e.register(LIST_ADD_TRIANGLE, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            let value = e.mem.u16(a[1]);
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.set_u16(buffer + 2 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        e.register(ARRAY_IS_EMPTY, |e, a| (e.mem.u32(a[0] + 8) == 0).into_ret());
        e.register(LIST_SORT, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            let buffer = e.mem.u32(a[0] + 4);
            let mut items: Vec<u16> = (0..size).map(|i| e.mem.u16(buffer + 2 * i)).collect();
            items.sort_unstable();
            for (i, v) in items.iter().enumerate() {
                e.mem.set_u16(buffer + 2 * i as u32, *v);
            }
            Ret::default()
        });
        e.register(INDEX_ARRAY_CONSTRUCTOR, |e, a| {
            let buffer = e.mem.alloc(32);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
        e.register(INDEX_ARRAY_ADD, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            let value = e.mem.u32(a[1]);
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(buffer + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        e.register(WORD_ELEMENT, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        quiet_doubles(e, &[LIST_REMOVE_AT, INDEX_ARRAY_DESTRUCTOR]);
    }

    #[test]
    fn collect_cell_triangles_and_drop_duplicates() {
        let mut e = grid_engine();
        let cell = e.mem.alloc(16);
        let cell_buffer = e.mem.alloc(8);
        for (i, t) in [7u16, 3, 7].iter().enumerate() {
            e.mem.set_u16(cell_buffer + 2 * i as u32, *t);
        }
        list_doubles(&mut e, cell, cell_buffer);
        let list: Ptr<BSSimpleArray> = e.new_object();
        let buffer = e.mem.alloc(32);
        e.set(list, BSSimpleArray::pBuffer, buffer);
        let grid = sample_grid(&mut e);
        let p = point(&mut e, 15.0, 7.0);
        e.call_log = Some(vec![]);
        e.call(0x0069d4b0, &args![grid, p, 0u32, list]);
        let log = take_log(&mut e);
        assert_eq!(calls(&log, GRID_CELL).len(), 1);
        assert_eq!(calls(&log, LIST_ADD_TRIANGLE).len(), 3);
        // Low row, low column, high row, high column, each to 0..3.
        let clamps = calls(&log, CLAMP_INT);
        assert_eq!(clamps.len(), 4);
        assert!(clamps.iter().all(|c| c[1] == 0 && c[2] == 3));
        assert_eq!(
            calls(&log, LIST_SORT),
            vec![vec![list.addr(), COMPARE_TRIANGLES]]
        );
        // Sorted 3, 7, 7: index 2 repeats and is removed.
        assert_eq!(calls(&log, LIST_REMOVE_AT), vec![vec![list.addr(), 2, 1]]);
        assert_eq!(calls(&log, INDEX_ARRAY_DESTRUCTOR).len(), 1);

        // Without duplicates the local array is destroyed without removals.
        e.mem.set_u32(list.addr() + 8, 0);
        e.mem.set_u16(cell_buffer + 4, 9);
        e.call_log = Some(vec![]);
        e.call(0x0069d4b0, &args![grid, p, 0u32, list]);
        let log = take_log(&mut e);
        assert!(calls(&log, LIST_REMOVE_AT).is_empty());
        assert_eq!(calls(&log, INDEX_ARRAY_DESTRUCTOR).len(), 1);

        // An empty result is neither sorted nor deduplicated.
        e.mem.set_u32(list.addr() + 8, 0);
        e.register(LIST_ADD_TRIANGLE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0069d4b0, &args![grid, p, 0u32, list]);
        let log = take_log(&mut e);
        assert!(calls(&log, LIST_SORT).is_empty());
        assert!(calls(&log, INDEX_ARRAY_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn cell_corner() {
        let mut e = grid_engine();
        quiet_doubles(&mut e, &[POINT_FROM_FLOATS]);
        let grid = sample_grid(&mut e);
        e.set(grid, NavMeshGrid::GridBoundsMin_x, 1.0);
        e.set(grid, NavMeshGrid::GridBoundsMin_y, 2.0);
        e.call_log = Some(vec![]);
        let back = e.call(0x0069d790, &args![grid, 0x6000u32, 3u32, 2u32]);
        assert_eq!(back.u32(), 0x6000);
        let log = take_log(&mut e);
        assert_eq!(
            calls(&log, POINT_FROM_FLOATS),
            vec![vec![
                0x6000,
                31.0f32.to_bits(),
                12.0f32.to_bits(),
                (-3.0f32).to_bits()
            ]]
        );
    }

    fn setup_doubles(e: &mut Engine) {
        e.register(VECTOR_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(VECTOR_DIVIDE, |e, a| {
            let d = f32::from_bits(a[1]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) / d;
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            Ret::default()
        });
        e.register(FABS, |_, a| {
            f64::from(f32::from_bits(a[0]).abs()).into_ret()
        });
        e.register(CLAMP_UINT, |e, a| {
            let v = e.mem.u32(a[0]).clamp(a[1], a[2]);
            e.mem.set_u32(a[0], v);
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        quiet_doubles(e, &[VECTOR_CONSTRUCT]);
    }

    #[test]
    fn grid_setup_sizes_and_allocates() {
        let mut e = grid_engine();
        setup_doubles(&mut e);
        let min = e.mem.alloc(12);
        let max = e.mem.alloc(12);
        for (i, (lo, hi)) in [(10.0f32, 50.0f32), (40.0, 0.0), (1.0, 2.0)]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(min + 4 * i as u32, *lo);
            e.mem.set_f32(max + 4 * i as u32, *hi);
        }
        let grid: Ptr<NavMeshGrid> = e.new_object();
        // 16 triangles: one cell.
        e.call_log = Some(vec![]);
        e.call(0x0069d800, &args![grid, min, max, 16u32]);
        let log = take_log(&mut e);
        assert_eq!(e.get(grid, NavMeshGrid::iGridSize), 1);
        assert!(calls(&log, CLAMP_UINT).is_empty());
        assert_eq!(e.get(grid, NavMeshGrid::fColumnSectionLen), 40.0);
        assert_eq!(e.get(grid, NavMeshGrid::fRowSectionLen), 40.0);
        assert_eq!(e.get(grid, NavMeshGrid::GridBoundsMin_x), 10.0);
        assert_eq!(e.get(grid, NavMeshGrid::GridBoundsMax_z), 2.0);
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![0x14]]);
        let cells = e.get(grid, NavMeshGrid::GridData).addr();
        assert_eq!(e.mem.u32(cells - 4), 1);
        assert_eq!(
            calls(&log, VECTOR_CONSTRUCT),
            vec![vec![cells, 0x10, 1, CELL_CONSTRUCT, CELL_DESTRUCT]]
        );

        // 150 triangles: 150 / 50 + 2 = 5 cells per side.
        e.call_log = Some(vec![]);
        e.call(0x0069d800, &args![grid, min, max, 150u32]);
        let log = take_log(&mut e);
        assert_eq!(calls(&log, CLAMP_UINT), vec![vec![grid.addr(), 2, 12]]);
        assert_eq!(e.get(grid, NavMeshGrid::iGridSize), 5);
        assert_eq!(calls(&log, VECTOR_DIVIDE)[0][1], 5.0f32.to_bits());
        assert_eq!(e.get(grid, NavMeshGrid::fColumnSectionLen), 8.0);
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![25 * 16 + 4]]);

        // 2000 triangles clamp to 12; a failed allocation leaves no cells.
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0069d800, &args![grid, min, max, 2000u32]);
        let log = take_log(&mut e);
        assert_eq!(e.get(grid, NavMeshGrid::iGridSize), 12);
        assert!(calls(&log, VECTOR_CONSTRUCT).is_empty());
        assert!(e.get(grid, NavMeshGrid::GridData).is_null());
        assert_eq!(allocation_bytes(0x1000_0000), u32::MAX);
        assert_eq!(allocation_bytes(0x0fff_ffff), 0xffff_fff4);
    }
}
