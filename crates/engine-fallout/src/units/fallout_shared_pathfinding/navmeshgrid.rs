//! `fallout shared/pathfinding/navmeshgrid.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The search grid of a navmesh (`NavMeshGrid`, layout in `navmesh.rs`): a
//! square of `iGridSize * iGridSize` cells, each a
//! `BSSimpleArray<unsigned short, 1024>` of the triangles that touch it.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_pathfinding::navmesh::NavMeshGrid;

/// `operator delete(block)` (`00401030`, cdecl).
const DELETE: u32 = 0x0040_1030;
/// `operator new(size)` (`00401000`, cdecl).
const NEW: u32 = 0x0040_1000;
/// `memcpy`-like copy (`00401460(destination, source, size)`, cdecl).
const COPY: u32 = 0x0040_1460;
/// `00401680(file)`: whether the file being read has the other byte order
/// (the byte at `+0x299` of the file object).
const FILE_SWAPS: u32 = 0x0040_1680;
/// `00401080(address, word)`: reverses the bytes of the 32-bit value at
/// `address` (cdecl; the second word is never read).
const SWAP_32: u32 = 0x0040_1080;
/// `00407a90(address, word)`: reverses the bytes of the 16-bit value at
/// `address` (cdecl; the second word is never read).
const SWAP_16: u32 = 0x0040_7a90;
/// `005eee80(this, count, flag)`: `BSSimpleArray<unsigned short, 1024>::SetSize`.
const CELL_SET_SIZE: u32 = 0x005e_ee80;
/// `005eeda0(this, index)`: the address of element `index` (`+4` buffer
/// plus `index * 2`).
const CELL_ELEMENT_ADDRESS: u32 = 0x005e_eda0;
/// `00ec782f(array, element size, count, constructor, destructor)`: the
/// compiler's vector constructor iterator (callee cleans the stack).
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
/// `00ec5fce(array, element size, count, destructor)`: the compiler's
/// vector destructor iterator (callee cleans the stack).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// `006917d0(this)`: constructor of a cell.
const CELL_CONSTRUCT: u32 = 0x0069_17d0;
/// `00691800(this)`: destructor of a cell.
const CELL_DESTRUCT: u32 = 0x0069_1800;
/// `0069dee0(cell, &triangle)`: whether the cell's list holds the triangle
/// (`0069b240` returns its index, or -1, and the result is `!= -1`).
const CELL_CONTAINS: u32 = 0x0069_dee0;

/// Size in bytes of the grid header read from the file: `iGridSize` and the
/// eight floats up to `GridBoundsMax` (the grid pointer is not part of it).
const HEADER_SIZE: u32 = 0x24;
/// Size in bytes of one cell.
const CELL_SIZE: u32 = 0x10;

// Translated from 0069d9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<unsigned short, 1024>::_vector_deleting_destructor_`
/// (Xbox PDB name of the map): with bit 1 of `flags` set, `this` is the first
/// element of an array whose count is the word before it: the vector
/// destructor iterator (`00ec5fce`) destroys all elements with the cell
/// destructor `00691800`, the block starting 4 bytes earlier is freed when
/// bit 0 is set, and that block address is returned. Without bit 1 it
/// destroys the one cell (`00691800`), frees it when bit 0 is set, and
/// returns `this`.
pub fn fn_0069d9a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 2 != 0 {
        let count = e.mem.u32(this.addr().wrapping_sub(4));
        e.call(
            VECTOR_DESTRUCT,
            &args![this, CELL_SIZE, count, CELL_DESTRUCT],
        );
        let block = this.addr().wrapping_sub(4);
        if flags & 1 != 0 {
            e.call(DELETE, &args![block]);
        }
        Ptr::new(block)
    } else {
        e.call(CELL_DESTRUCT, &args![this]);
        if flags & 1 != 0 {
            e.call(DELETE, &args![this]);
        }
        this
    }
}

// Translated from 0069da10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the triangle list of grid cell `cell_index` (the 16-byte cell at
/// `GridData + cell_index * 16`) holds `triangle`: `0069dee0` on the cell with
/// the address of a stack word holding `triangle`. The map has no name for
/// it.
pub fn fn_0069da10(e: &mut Engine, this: Ptr<NavMeshGrid>, triangle: u32, cell_index: u32) -> bool {
    let grid = e.get(this, NavMeshGrid::GridData);
    let cell = cell_index.wrapping_mul(CELL_SIZE).wrapping_add(grid.addr());
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), triangle);
        e.call(CELL_CONTAINS, &args![cell, word]).bool()
    })
}

/// Bytes the game asks for to hold the count word and `cells` cells: the
/// product with 16 plus 4, or `0xFFFFFFFF` when either step overflows.
fn allocation_bytes(cells: u32) -> u32 {
    cells
        .checked_mul(CELL_SIZE)
        .and_then(|bytes| bytes.checked_add(4))
        .unwrap_or(u32::MAX)
}

/// Swaps the 16-bit value at `address` in place when `swaps` (the other
/// byte order), as the loader does, and returns it.
fn read_short(e: &mut Engine, address: u32, swaps: bool) -> u16 {
    if swaps {
        e.call(SWAP_16, &args![address, 0u32]);
    }
    e.mem.u16(address)
}

// Translated from 0069da40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0069da40 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `NavMeshGrid::Load` (Xbox PDB): reads the grid from the navmesh chunk at
/// `data` (`_unused_2` is the chunk size the caller passes and the function
/// never reads; `file` is the file object whose byte-order flag `00401680`
/// tests). Copies the 0x24-byte header over the grid, reverses the nine
/// header words when the file has the other byte order, and returns 0 with
/// `GridData` cleared when `iGridSize` is 0. Otherwise it allocates
/// `iGridSize * iGridSize` cells (an element count word, then the cells,
/// built by the vector constructor iterator `00ec782f` with the cell
/// constructor `006917d0` and destructor `00691800`; a size that overflows
/// is passed as `0xFFFFFFFF`), and for each cell reads a 16-bit count,
/// sets the cell's list to that size (`005eee80`) and copies the triangle
/// indexes into it, reversing each when the file has the other byte order.
/// Returns 1. The exception-unwinding frame is not translated.
pub fn fn_0069da40(
    e: &mut Engine,
    this: Ptr<NavMeshGrid>,
    data: u32,
    _unused_2: u32,
    file: Ptr,
) -> bool {
    e.call(COPY, &args![this, data, HEADER_SIZE]);
    let swaps = e.call(FILE_SWAPS, &args![file]).bool();
    if swaps {
        for word in 0..9u32 {
            e.call(SWAP_32, &args![this.addr().wrapping_add(4 * word), 0u32]);
        }
    }
    let grid_size = e.get(this, NavMeshGrid::iGridSize);
    if grid_size == 0 {
        e.set(this, NavMeshGrid::GridData, Ptr::NULL);
        return false;
    }
    let cells = grid_size.wrapping_mul(grid_size);
    let block = e.call(NEW, &args![allocation_bytes(cells)]).u32();
    let grid = if block == 0 {
        0
    } else {
        e.mem.set_u32(block, cells);
        e.call(
            VECTOR_CONSTRUCT,
            &args![
                block.wrapping_add(4),
                CELL_SIZE,
                cells,
                CELL_CONSTRUCT,
                CELL_DESTRUCT
            ],
        );
        block.wrapping_add(4)
    };
    e.set(this, NavMeshGrid::GridData, Ptr::new(grid));
    let mut cursor = data.wrapping_add(HEADER_SIZE);
    for cell_index in 0..cells {
        let count = read_short(e, cursor, swaps);
        cursor = cursor.wrapping_add(2);
        let cell = e
            .get(this, NavMeshGrid::GridData)
            .addr()
            .wrapping_add(cell_index.wrapping_mul(CELL_SIZE));
        e.call(CELL_SET_SIZE, &args![cell, u32::from(count), 1u32]);
        if count > 0 {
            if swaps {
                for triangle in 0..count {
                    let address = cursor.wrapping_add(u32::from(triangle) * 2);
                    e.call(SWAP_16, &args![address, 0u32]);
                }
            }
            let destination = e.call(CELL_ELEMENT_ADDRESS, &args![cell, 0u32]).u32();
            let size = u32::from(count) * 2;
            e.call(COPY, &args![destination, cursor, size]);
            cursor = cursor.wrapping_add(size);
        }
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0069d9a0, fn_0069d9a0(Ptr, u32) -> Ptr),
        entry!(0x0069da10, fn_0069da10(Ptr<NavMeshGrid>, u32, u32) -> bool),
        entry!(
            0x0069da40,
            fn_0069da40(Ptr<NavMeshGrid>, u32, u32, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn silent(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.clone().unwrap()
    }

    #[test]
    fn deleting_destructor_of_one_cell() {
        let mut e = Engine::new();
        silent(&mut e, &[CELL_DESTRUCT, DELETE, VECTOR_DESTRUCT]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069d9a0, &args![0x4000u32, 0u32]).u32(), 0x4000);
        assert_eq!(calls(&e).len(), 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069d9a0, &args![0x4000u32, 1u32]).u32(), 0x4000);
        assert_eq!(calls(&e)[2], (DELETE, vec![0x4000]));
    }

    #[test]
    fn deleting_destructor_of_an_array_reads_the_count_before_it() {
        let mut e = Engine::new();
        silent(&mut e, &[CELL_DESTRUCT, DELETE, VECTOR_DESTRUCT]);
        let block = e.mem.alloc(0x40);
        e.mem.set_u32(block, 3);
        e.call_log = Some(vec![]);
        let result = e.call(0x0069d9a0, &args![block + 4, 3u32]).u32();
        assert_eq!(result, block);
        assert_eq!(
            calls(&e)[1..],
            [
                (VECTOR_DESTRUCT, vec![block + 4, 0x10, 3, CELL_DESTRUCT]),
                (DELETE, vec![block]),
            ]
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069d9a0, &args![block + 4, 2u32]).u32(), block);
        assert_eq!(calls(&e).len(), 2);
    }

    #[test]
    fn cell_lookup_passes_the_address_of_the_triangle_word() {
        let mut e = Engine::new();
        e.register_double(CELL_CONTAINS, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[1]) == 0x1234),
            ..Ret::default()
        });
        let grid: Ptr<NavMeshGrid> = e.new_object();
        e.set(grid, NavMeshGrid::GridData, Ptr::new(0x8000));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0069da10, &args![grid, 0x1234u32, 3u32]).bool());
        assert_eq!(calls(&e)[1].1[0], 0x8000 + 3 * 16);
        assert!(!e.call(0x0069da10, &args![grid, 0x1235u32, 3u32]).bool());
    }

    /// Doubles for everything `Load` calls outside the file: byte swaps in
    /// memory, a `SetSize` that allocates the cell's buffer, and the
    /// element address.
    fn load_doubles(e: &mut Engine, swaps: bool) {
        e.register_double(FILE_SWAPS, move |_, _| Ret {
            eax: u32::from(swaps),
            ..Ret::default()
        });
        e.register(SWAP_32, |e, a| {
            let value = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], value.swap_bytes());
            Ret::default()
        });
        e.register(SWAP_16, |e, a| {
            let value = e.mem.u16(a[0]);
            e.mem.set_u16(a[0], value.swap_bytes());
            Ret::default()
        });
        e.register(CELL_SET_SIZE, |e, a| {
            let buffer = e.mem.alloc(a[1].max(1) * 2);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(CELL_ELEMENT_ADDRESS, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4) + a[1] * 2,
            ..Ret::default()
        });
        silent(e, &[VECTOR_CONSTRUCT]);
    }

    /// A 2x2 grid chunk: cell counts 1, 0, 2, 1 and triangles 7, 8 9, 10.
    fn chunk(e: &mut Engine, swapped: bool) -> u32 {
        let words: Vec<u16> = vec![1, 7, 0, 2, 8, 9, 1, 10];
        let data = e.mem.alloc(0x24 + 2 * words.len() as u32);
        let size = if swapped { 2u32.swap_bytes() } else { 2 };
        e.mem.set_u32(data, size);
        for (i, w) in words.iter().enumerate() {
            let w = if swapped { w.swap_bytes() } else { *w };
            e.mem.set_u16(data + 0x24 + 2 * i as u32, w);
        }
        data
    }

    fn cell_triangles(e: &Engine, grid: u32, cell: u32) -> Vec<u16> {
        let count = e.mem.u32(grid + cell * 16 + 8);
        let buffer = e.mem.u32(grid + cell * 16 + 4);
        (0..count).map(|i| e.mem.u16(buffer + 2 * i)).collect()
    }

    #[test]
    fn load_reads_the_cells() {
        let mut e = Engine::new();
        load_doubles(&mut e, false);
        let data = chunk(&mut e, false);
        let grid: Ptr<NavMeshGrid> = e.new_object();
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0069da40, &args![grid, data, 99u32, 0x5000u32])
            .bool());
        assert_eq!(e.get(grid, NavMeshGrid::iGridSize), 2);
        let cells = e.get(grid, NavMeshGrid::GridData).addr();
        assert_eq!(e.mem.u32(cells - 4), 4);
        assert_eq!(cell_triangles(&e, cells, 0), vec![7]);
        assert_eq!(cell_triangles(&e, cells, 1), Vec::<u16>::new());
        assert_eq!(cell_triangles(&e, cells, 2), vec![8, 9]);
        assert_eq!(cell_triangles(&e, cells, 3), vec![10]);
        let log = calls(&e);
        let of = |address: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|c| c.0 == address)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(of(COPY)[0], vec![grid.addr(), data, 0x24]);
        assert_eq!(of(FILE_SWAPS)[0], vec![0x5000]);
        assert_eq!(of(NEW), vec![vec![4 * 16 + 4]]);
        assert_eq!(
            of(VECTOR_CONSTRUCT),
            vec![vec![cells, 0x10, 4, CELL_CONSTRUCT, CELL_DESTRUCT]]
        );
        assert!(log.iter().all(|c| c.0 != SWAP_16 && c.0 != SWAP_32));
    }

    #[test]
    fn load_reverses_the_bytes_of_a_file_in_the_other_order() {
        let mut e = Engine::new();
        load_doubles(&mut e, true);
        let data = chunk(&mut e, true);
        let grid: Ptr<NavMeshGrid> = e.new_object();
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0069da40, &args![grid, data, 99u32, 0x5000u32])
            .bool());
        assert_eq!(e.get(grid, NavMeshGrid::iGridSize), 2);
        let cells = e.get(grid, NavMeshGrid::GridData).addr();
        assert_eq!(cell_triangles(&e, cells, 2), vec![8, 9]);
        assert_eq!(cell_triangles(&e, cells, 3), vec![10]);
        let swaps32 = calls(&e).iter().filter(|c| c.0 == SWAP_32).count();
        assert_eq!(swaps32, 9);
        // One swap per cell count (four) and one per triangle (four).
        let swaps16 = calls(&e).iter().filter(|c| c.0 == SWAP_16).count();
        assert_eq!(swaps16, 8);
    }

    #[test]
    fn load_with_an_empty_grid_fails_and_clears_the_cells() {
        let mut e = Engine::new();
        load_doubles(&mut e, false);
        let data = e.mem.alloc(0x40);
        let grid: Ptr<NavMeshGrid> = e.new_object();
        e.set(grid, NavMeshGrid::GridData, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0069da40, &args![grid, data, 0u32, 0x5000u32])
            .bool());
        assert_eq!(e.get(grid, NavMeshGrid::GridData), Ptr::NULL);
        assert!(calls(&e).iter().all(|c| c.0 != NEW));
    }

    #[test]
    fn allocation_size_saturates_when_it_overflows() {
        assert_eq!(allocation_bytes(4), 68);
        assert_eq!(allocation_bytes(0), 4);
        assert_eq!(allocation_bytes(0x1000_0000), u32::MAX);
        assert_eq!(allocation_bytes(u32::MAX), u32::MAX);
        // 0xFFFF_FFF0 bytes of cells plus the 4-byte count do not overflow.
        assert_eq!(allocation_bytes(0x0fff_ffff), 0xffff_fff0 + 4);
    }
}
