//! `fallout shared/run_00555be0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The player character pointer.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// The position of the last seen-data update (an `NiPoint3` at
/// `011ca208`, three floats) and the guard word of its one-time
/// initialisation (bit 0 at `011ca214`).
const LAST_POSITION: u32 = 0x011c_a208;
const LAST_POSITION_INITIALIZED: u32 = 0x011c_a214;
/// `FLT_MAX` (`float`).
const FLOAT_MAX: u32 = 0x0101_6970;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `4096.0` (`double`): one exterior cell.
const CELL_SIZE: u32 = 0x0101_7a10;
/// The float setting whose value (`00403e20`) is the seen distance.
const SEEN_DISTANCE_SETTING: u32 = 0x011c_a094;

/// `NiPoint3` constructor (`00416870`, `thiscall`: x, y, z), answers `this`.
const POINT_CONSTRUCT: u32 = 0x0041_6870;
/// `NiPoint2` constructor (`00452dc0`, `thiscall`: x, y).
const POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
/// `00436aa0` (`thiscall`): the reference's position address.
const POSITION: u32 = 0x0043_6aa0;
/// The seen bit snap (`00879ca0`, `cdecl`, unit `seendata`).
const SNAP_TO_SEEN_SQUARE: u32 = 0x0087_9ca0;
/// `00439090` (`thiscall`, `RET 4`): whether two points differ (`AL`).
const POINTS_DIFFER: u32 = 0x0043_9090;
/// `008d6f30` (`thiscall`): the reference's parent cell.
const PARENT_CELL: u32 = 0x008d_6f30;
/// `00425fd0` (`thiscall`): whether the cell is an interior (`AL`).
const IS_INTERIOR: u32 = 0x0042_5fd0;
/// `memset`-shaped helper (`00403d30`, `cdecl`: destination, value, count).
const MEMSET_WRAPPER: u32 = 0x0040_3d30;
/// `TESObjectCELL::GetWorldSpace` (engine map).
const CELL_WORLD_SPACE: u32 = 0x0054_ddd0;
/// `00406d90` (`cdecl`, one `float`): the float as an integer.
const FLOAT_TO_INT: u32 = 0x0040_6d90;
/// A float setting's value address (`00403e20`, `thiscall`).
const FLOAT_SETTING_VALUE: u32 = 0x0040_3e20;
/// `TESWorldSpace::GetCellFromCellCoord` (engine map), `RET 8`.
const CELL_AT: u32 = 0x0058_75a0;
/// `00555ad0` (`thiscall`): the cell's north rotation, in `ST0`.
const NORTH_ROTATION: u32 = 0x0055_5ad0;
/// `TESObjectCELL::AdjustCoordForNorthRotation` (engine map), `RET 0xc`.
const ADJUST_FOR_NORTH_ROTATION: u32 = 0x0055_5b10;
/// The `vector constructor iterator` (`00401050`, `stdcall`: array,
/// element size, count, constructor).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
/// `TESObjectCELL::GetIntSeenSection` (engine map), `RET 0xc`.
const INT_SEEN_SECTION: u32 = 0x0055_6ef0;
/// The seen-data update of one section (`00879ad0`, unit `seendata`).
const SEEN_DATA_UPDATE: u32 = 0x0087_9ad0;
/// `ExtraDataList::GetSeenData` (engine map).
const GET_SEEN_DATA: u32 = 0x0055_5bc0;
/// `MemoryManager` allocation (`00401000`, `cdecl`: size).
const ALLOCATE: u32 = 0x0040_1000;
/// The `SeenData` constructor (`00879940`, unit `run_008798e0`).
const SEEN_DATA_CONSTRUCT: u32 = 0x0087_9940;
/// `004610d0` (`thiscall`): the cell's extra data list.
const EXTRA_DATA: u32 = 0x0046_10d0;
/// `ExtraDataList::SetSeenData` (engine map), `RET 4`.
const SET_SEEN_DATA: u32 = 0x0042_1940;
/// `TESObjectCELL::GetDataY` and `GetDataX` (engine map).
const CELL_DATA_Y: u32 = 0x0054_4c60;
const CELL_DATA_X: u32 = 0x0054_4c30;
/// `00555be0` (`thiscall`): run when the cell's map is fully seen.
const MARK_FULLY_SEEN: u32 = 0x0055_5be0;
/// `00544440` (`thiscall`, `RET 4`): sets a cell flag.
const SET_CELL_FLAG: u32 = 0x0054_4440;
/// The `NiPoint3` element constructor `00401050` runs (`006815c0`).
const POINT_ELEMENT_CONSTRUCT: u32 = 0x0068_15c0;

/// `00406d90` of the float word `bits` (pushed after `FLD`).
fn float_to_int(e: &mut Engine, bits: u32) -> i32 {
    e.call(FLOAT_TO_INT, &args![x87_quiet_bits(bits)]).i32()
}

/// `FILD dword`, `FSTP dword`: an integer as a float word.
fn int_to_float(value: i32) -> u32 {
    x87_store32(f64::from(value))
}

/// The seen distance (the float setting at `011ca094`) as an `x87_m32`
/// operand.
fn seen_distance(e: &mut Engine) -> f64 {
    let value = e
        .call(FLOAT_SETTING_VALUE, &args![SEEN_DISTANCE_SETTING])
        .u32();
    x87_m32(e.mem.u32(value))
}

/// The step toward a neighbouring cell along one axis: -1 when `position`
/// is within the seen distance of the low edge, 1 when it is within it of
/// the high edge (the exe's compares, NaN included), else 0.
fn edge_step(e: &mut Engine, position: u32, low: u32, high: u32) -> i32 {
    let below = x87_sub(x87_load32(position), seen_distance(e));
    let low = x87_load32(low);
    if !(low < below || low.is_nan() || below.is_nan()) {
        return -1;
    }
    let above = x87_add(x87_load32(position), seen_distance(e));
    let high = x87_load32(high);
    i32::from(high <= above)
}

// Translated from 00555c20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00555c20 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Updates the local map's seen data around the player (no name in the
/// engine map; no parameters). The first call sets the last position to
/// `(FLT_MAX, FLT_MAX, FLT_MAX)`. With a player, its position snapped to a
/// seen square (`00879ca0`) is compared with the last one (`00439090`);
/// when it moved, it becomes the last position and, in the player's cell:
/// an interior cell updates itself (`00555f60`); an exterior cell in a
/// world space updates itself and the neighbouring cells (`5875a0`) whose
/// edge is within the seen distance of the position (up to three more,
/// stopping at the first missing one).
pub fn fn_00555c20(e: &mut Engine) {
    let guard = e.global::<u32>(LAST_POSITION_INITIALIZED);
    if guard & 1 == 0 {
        e.set_global(LAST_POSITION_INITIALIZED, guard | 1);
        let max = x87_quiet_bits(e.global::<u32>(FLOAT_MAX));
        e.call(POINT_CONSTRUCT, &args![LAST_POSITION, max, max, max]);
    }
    if e.global::<u32>(PLAYER_POINTER) == 0 {
        return;
    }
    e.with_stack(0x30, |e, locals| {
        let position = locals.addr();
        let cells = position + 0xc;
        let low = position + 0x1c;
        let high = position + 0x24;
        let player = e.global::<u32>(PLAYER_POINTER);
        let current = e.call(POSITION, &args![player]).u32();
        for i in 0..3 {
            let word = e.mem.u32(current + 4 * i);
            e.mem.set_u32(position + 4 * i, word);
        }
        e.call(SNAP_TO_SEEN_SQUARE, &args![position]);
        if !e
            .call(POINTS_DIFFER, &args![position, LAST_POSITION])
            .bool()
        {
            return;
        }
        for i in 0..3 {
            let word = e.mem.u32(position + 4 * i);
            e.set_global(LAST_POSITION + 4 * i, word);
        }
        let player = e.global::<u32>(PLAYER_POINTER);
        let cell = e.call(PARENT_CELL, &args![player]).u32();
        if cell == 0 {
            return;
        }
        if e.call(IS_INTERIOR, &args![cell]).bool() {
            fn_00555f60(e, Ptr::new(cell), position);
            return;
        }
        e.call(MEMSET_WRAPPER, &args![cells, 0u32, 0x10u32]);
        e.mem.set_u32(cells, cell);
        let world_space = e.call(CELL_WORLD_SPACE, &args![cell]).u32();
        if world_space != 0 {
            let x_bits = e.mem.u32(position);
            let cell_x = float_to_int(e, x_bits) >> 12;
            let y_bits = e.mem.u32(position + 4);
            let cell_y = float_to_int(e, y_bits) >> 12;
            e.call(
                POINT2_CONSTRUCT,
                &args![
                    low,
                    int_to_float(cell_x.wrapping_shl(12)),
                    int_to_float(cell_y.wrapping_shl(12))
                ],
            );
            let cell_size = e.global::<f64>(CELL_SIZE);
            let high_y = x87_store32(x87_add(f64::from(cell_y.wrapping_shl(12)), cell_size));
            let high_x = x87_store32(x87_add(f64::from(cell_x.wrapping_shl(12)), cell_size));
            e.call(
                POINT2_CONSTRUCT,
                &args![high, x87_quiet_bits(high_x), x87_quiet_bits(high_y)],
            );
            let (x, low_x, high_x) = (e.mem.u32(position), e.mem.u32(low), e.mem.u32(high));
            let step_x = edge_step(e, x, low_x, high_x);
            let (y, low_y, high_y) = (
                e.mem.u32(position + 4),
                e.mem.u32(low + 4),
                e.mem.u32(high + 4),
            );
            let step_y = edge_step(e, y, low_y, high_y);
            let mut count = 1u32;
            if step_x != 0 {
                let neighbour = e
                    .call(
                        CELL_AT,
                        &args![world_space, cell_x.wrapping_add(step_x), cell_y],
                    )
                    .u32();
                e.mem.set_u32(cells + 4 * count, neighbour);
                count += 1;
            }
            if step_y != 0 {
                let neighbour = e
                    .call(
                        CELL_AT,
                        &args![world_space, cell_x, cell_y.wrapping_add(step_y)],
                    )
                    .u32();
                e.mem.set_u32(cells + 4 * count, neighbour);
                count += 1;
            }
            if step_x != 0 && step_y != 0 {
                let neighbour = e
                    .call(
                        CELL_AT,
                        &args![
                            world_space,
                            cell_x.wrapping_add(step_x),
                            cell_y.wrapping_add(step_y)
                        ],
                    )
                    .u32();
                e.mem.set_u32(cells + 4 * count, neighbour);
            }
        }
        for index in 0..4 {
            let cell = e.mem.u32(cells + 4 * index);
            if cell == 0 {
                break;
            }
            fn_00555f60(e, Ptr::new(cell), position);
        }
    });
}

/// The center of seen section `(x, y)` (`(x << 12) + 0x800`) as a float
/// word.
fn section_center(value: i32) -> u32 {
    int_to_float(value.wrapping_shl(12).wrapping_add(0x800))
}

/// Builds the point `(x, y, z)` with the `NiPoint3` constructor on the
/// temporary and copies it to `out`.
fn build_point(e: &mut Engine, temporary: u32, out: u32, x: u32, y: u32, z: u32) {
    let point = e
        .call(POINT_CONSTRUCT, &args![temporary, x, y, x87_quiet_bits(z)])
        .u32();
    for i in 0..3 {
        let word = e.mem.u32(point + 4 * i);
        e.mem.set_u32(out + 4 * i, word);
    }
}

// Translated from 00555f60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00555f60 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Updates the cell's seen data from `position` (no name in the engine
/// map; `RET 4`).
///
/// An interior cell keeps one seen section per 4096-unit square
/// (`00556ef0`): with a north rotation (`00555ad0` not 0.0) the position is
/// rotated (`00555b10`) and the nine sections around it are updated
/// (`00879ad0` with each section's center), stopping at the first missing
/// one; without rotation the position's section and the neighbours whose
/// edge is within the seen distance (up to three) are updated.
///
/// An exterior cell that is not fully seen (`00556850`) gets its seen data
/// (`00555bc0`, made with `00879940` and stored with `00421940`, marking
/// the cell changed with slot +0x48 and `0x80000000`, when missing), updates
/// it from the cell's corner (`GetDataX`/`GetDataY` times 4096 and the
/// position's z), and when it is then fully seen runs `00555be0` and sets
/// cell flag 1 (`00544440`).
pub fn fn_00555f60(e: &mut Engine, this: Ptr, position: u32) {
    let cell = this.addr();
    if !e.call(IS_INTERIOR, &args![cell]).bool() {
        if fn_00556850(e, this) {
            return;
        }
        let mut seen = e.call(GET_SEEN_DATA, &args![cell]).u32();
        if seen == 0 {
            let block = e.call(ALLOCATE, &args![0x24u32]).u32();
            seen = if block != 0 {
                e.call(SEEN_DATA_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            let extra = e.call(EXTRA_DATA, &args![cell]).u32();
            e.call(SET_SEEN_DATA, &args![extra, seen]);
            e.vcall(cell, 0x48, &args![0x8000_0000u32]);
        }
        let z = e.mem.u32(position + 8);
        e.with_stack(0xc, |e, corner| {
            let y = e.call(CELL_DATA_Y, &args![cell]).i32();
            let y = int_to_float(y.wrapping_shl(12));
            let x = e.call(CELL_DATA_X, &args![cell]).i32();
            let x = int_to_float(x.wrapping_shl(12));
            e.call(POINT_CONSTRUCT, &args![corner, x, y, x87_quiet_bits(z)]);
            if e.call(SEEN_DATA_UPDATE, &args![seen, position, corner])
                .bool()
            {
                e.call(MARK_FULLY_SEEN, &args![cell]);
                e.call(SET_CELL_FLAG, &args![cell, 1u32]);
            }
        });
        return;
    }
    e.with_stack(0xd0, |e, locals| {
        let local_position = locals.addr();
        let cells = local_position + 0xc;
        let points = local_position + 0x30;
        let temporary = local_position + 0x9c;
        let low = local_position + 0xa8;
        let high = local_position + 0xb0;
        for i in 0..3 {
            let word = e.mem.u32(position + 4 * i);
            e.mem.set_u32(local_position + 4 * i, word);
        }
        let rotation = e.call(NORTH_ROTATION, &args![cell]).f64();
        if rotation != e.global::<f64>(ZERO) {
            e.call(
                ADJUST_FOR_NORTH_ROTATION,
                &args![cell, position, local_position, 1u32],
            );
            e.call(
                VECTOR_CONSTRUCT,
                &args![points, 0xcu32, 9u32, POINT_ELEMENT_CONSTRUCT],
            );
            e.call(MEMSET_WRAPPER, &args![cells, 0u32, 0x24u32]);
            e.call(MEMSET_WRAPPER, &args![points, 0u32, 0x6cu32]);
            let x_bits = e.mem.u32(local_position);
            let section_x = float_to_int(e, x_bits).wrapping_sub(0x800) >> 12;
            let y_bits = e.mem.u32(local_position + 4);
            let section_y = float_to_int(e, y_bits).wrapping_sub(0x800) >> 12;
            let mut count = 0u32;
            for dx in -1i32..=1 {
                for dy in -1i32..=1 {
                    let x = section_x.wrapping_add(dx);
                    let y = section_y.wrapping_add(dy);
                    let section = e.call(INT_SEEN_SECTION, &args![cell, x, y, 1u32]).u32();
                    e.mem.set_u32(cells + 4 * count, section);
                    let z = e.mem.u32(local_position + 8);
                    build_point(
                        e,
                        temporary,
                        points + 0xc * count,
                        section_center(x),
                        section_center(y),
                        z,
                    );
                    count += 1;
                }
            }
            for index in 0..9 {
                let section = e.mem.u32(cells + 4 * index);
                if section == 0 {
                    break;
                }
                e.call(
                    SEEN_DATA_UPDATE,
                    &args![section, local_position, points + 0xc * index],
                );
            }
            return;
        }
        e.call(
            VECTOR_CONSTRUCT,
            &args![points, 0xcu32, 4u32, POINT_ELEMENT_CONSTRUCT],
        );
        e.call(MEMSET_WRAPPER, &args![cells, 0u32, 0x10u32]);
        e.call(MEMSET_WRAPPER, &args![points, 0u32, 0x30u32]);
        let x_bits = e.mem.u32(local_position);
        let section_x = float_to_int(e, x_bits).wrapping_sub(0x800) >> 12;
        let y_bits = e.mem.u32(local_position + 4);
        let section_y = float_to_int(e, y_bits).wrapping_sub(0x800) >> 12;
        let section = e
            .call(INT_SEEN_SECTION, &args![cell, section_x, section_y, 1u32])
            .u32();
        e.mem.set_u32(cells, section);
        let z = e.mem.u32(local_position + 8);
        build_point(
            e,
            temporary,
            points,
            section_center(section_x),
            section_center(section_y),
            z,
        );
        e.call(
            POINT2_CONSTRUCT,
            &args![low, section_center(section_x), section_center(section_y)],
        );
        e.call(
            POINT2_CONSTRUCT,
            &args![
                high,
                section_center(section_x.wrapping_add(0x1000)),
                section_center(section_y.wrapping_add(0x1000))
            ],
        );
        let (x, low_x, high_x) = (e.mem.u32(local_position), e.mem.u32(low), e.mem.u32(high));
        let step_x = edge_step(e, x, low_x, high_x);
        let (y, low_y, high_y) = (
            e.mem.u32(local_position + 4),
            e.mem.u32(low + 4),
            e.mem.u32(high + 4),
        );
        let step_y = edge_step(e, y, low_y, high_y);
        let mut count = 1u32;
        let neighbours = [
            (step_x != 0, step_x, 0),
            (step_y != 0, 0, step_y),
            (step_x != 0 && step_y != 0, step_x, step_y),
        ];
        for (wanted, dx, dy) in neighbours {
            if !wanted {
                continue;
            }
            let x = section_x.wrapping_add(dx);
            let y = section_y.wrapping_add(dy);
            let z = e.mem.u32(local_position + 8);
            build_point(
                e,
                temporary,
                points + 0xc * count,
                section_center(x),
                section_center(y),
                z,
            );
            let section = e.call(INT_SEEN_SECTION, &args![cell, x, y, 1u32]).u32();
            e.mem.set_u32(cells + 4 * count, section);
            count += 1;
        }
        for index in 0..4 {
            let section = e.mem.u32(cells + 4 * index);
            if section == 0 {
                break;
            }
            e.call(
                SEEN_DATA_UPDATE,
                &args![section, local_position, points + 0xc * index],
            );
        }
    });
}

// Translated from 00556850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the byte at +0x25 of the cell is set (no name in the
/// engine map; `SETNZ AL`), the flag that skips the seen-data update.
pub fn fn_00556850(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr().wrapping_add(0x25)) & 1 != 0
}

// Translated from 00555be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`): clears the seen data of the reference's extra
/// data (`004610d0`, then `ExtraDataList::SetSeenData(0)`), clears bit 0 of
/// the byte at +0x25 and calls its own slot +0x4C with 0x80000000.
pub fn fn_00555be0(e: &mut Engine, this: Ptr) {
    let extra = e.call(EXTRA_DATA, &args![this]).u32();
    e.call(SET_SEEN_DATA, &args![extra, 0u32]);
    let at = this.addr() + 0x25;
    let byte = e.mem.u8(at) & 0xfe;
    e.mem.set_u8(at, byte);
    e.vcall(this.addr(), 0x4c, &args![0x8000_0000u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0055_5c20, fn_00555c20()),
        entry!(0x0055_5f60, fn_00555f60(Ptr, u32)),
        entry!(0x0055_6850, fn_00556850(Ptr) -> bool),
        entry!(0x0055_5be0, fn_00555be0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// Globals, the seen distance (200) and the plain doubles.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.map(0x011c_a000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.set_global(FLOAT_MAX, f32::MAX);
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.map(0x0101_7000, 0x1000);
        e.set_global(CELL_SIZE, 4096.0f64);
        let distance = e.mem.alloc(4);
        e.mem.set_f32(distance, 200.0);
        e.register_double(FLOAT_SETTING_VALUE, move |_, _| word(distance));
        e.register(
            FLOAT_TO_INT,
            |_, a| word(f32::from_bits(a[0]) as i32 as u32),
        );
        e.register(POINT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            word(a[0])
        });
        e.register(POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            word(a[0])
        });
        e.register(MEMSET_WRAPPER, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(VECTOR_CONSTRUCT, nothing);
        e.register(IS_INTERIOR, |e, a| {
            word(u32::from(e.mem.u8(a[0] + 0x30) == 1))
        });
        e
    }

    /// A cell (0x40 bytes); `interior` sets the byte the `00425fd0`
    /// double reads, `skip` the flag `00556850` reads.
    fn cell(e: &mut Engine, interior: bool, skip: bool) -> u32 {
        let cell = e.mem.alloc(0x40);
        e.mem.set_u8(cell + 0x30, u8::from(interior));
        e.mem.set_u8(cell + 0x25, u8::from(skip));
        cell
    }

    #[test]
    fn fn_00556850_reads_bit_0_at_0x25() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        e.mem.set_u8(this + 0x25, 0xfe);
        assert!(!e.call(0x0055_6850, &args![this]).bool());
        e.mem.set_u8(this + 0x25, 1);
        assert!(e.call(0x0055_6850, &args![this]).bool());
    }

    #[test]
    fn fn_00555c20_initialises_once_and_needs_a_player() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0055_5c20, &[]);
        e.call(0x0055_5c20, &[]);
        let max = f32::MAX.to_bits();
        assert_eq!(
            calls_to(&e, POINT_CONSTRUCT),
            vec![vec![LAST_POSITION, max, max, max]]
        );
        assert_eq!(e.global::<f32>(LAST_POSITION), f32::MAX);
        assert!(calls_to(&e, POSITION).is_empty());
    }

    fn player_at(e: &mut Engine, x: f32, y: f32) -> u32 {
        let player = e.mem.alloc(0x20);
        e.mem.set_f32(player + 0x10, x);
        e.mem.set_f32(player + 0x14, y);
        e.mem.set_f32(player + 0x18, 5.0);
        e.set_global(PLAYER_POINTER, player);
        e.register(POSITION, |_, a| word(a[0] + 0x10));
        e.register(SNAP_TO_SEEN_SQUARE, nothing);
        player
    }

    #[test]
    fn fn_00555c20_stops_when_the_position_did_not_change() {
        let mut e = engine();
        player_at(&mut e, 100.0, 4000.0);
        e.register(POINTS_DIFFER, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x0055_5c20, &[]);
        assert_eq!(calls_to(&e, POINTS_DIFFER)[0][1], LAST_POSITION);
        assert!(calls_to(&e, PARENT_CELL).is_empty());
    }

    #[test]
    fn fn_00555c20_updates_the_cell_and_its_near_neighbours() {
        let mut e = engine();
        let player = player_at(&mut e, 100.0, 4000.0);
        e.register(POINTS_DIFFER, |_, _| word(1));
        let home = cell(&mut e, false, true);
        e.register_double(PARENT_CELL, move |_, _| word(home));
        e.register(CELL_WORLD_SPACE, |_, _| word(0x7000));
        let neighbours = e.mem.alloc(0x100);
        // Each neighbour is a skipped exterior cell.
        for i in 0..4 {
            e.mem.set_u8(neighbours + 0x40 * i + 0x25, 1);
        }
        e.register_double(CELL_AT, move |_, a| {
            let index = (a[1] as i32 + 2) as u32 + 2 * (a[2] as i32 + 1) as u32;
            word(neighbours + 0x40 * (index % 4))
        });
        e.call_log = Some(vec![]);
        e.call(0x0055_5c20, &[]);
        assert_eq!(e.global::<f32>(LAST_POSITION + 4), 4000.0);
        assert_eq!(calls_to(&e, PARENT_CELL), vec![vec![player]]);
        // Near the low x edge and the high y edge.
        assert_eq!(
            calls_to(&e, CELL_AT),
            vec![
                vec![0x7000, -1i32 as u32, 0],
                vec![0x7000, 0, 1],
                vec![0x7000, -1i32 as u32, 1]
            ]
        );
        assert_eq!(
            calls_to(&e, POINT2_CONSTRUCT)[1],
            vec![
                calls_to(&e, POINT2_CONSTRUCT)[1][0],
                4096f32.to_bits(),
                4096f32.to_bits()
            ]
        );
        // The cell and the three neighbours are updated.
        assert_eq!(calls_to(&e, IS_INTERIOR).len(), 5);
    }

    #[test]
    fn fn_00555f60_skips_a_flagged_exterior_cell() {
        let mut e = engine();
        let this = cell(&mut e, false, true);
        e.call_log = Some(vec![]);
        e.call(0x0055_5f60, &args![this, 0x1234u32]);
        assert!(calls_to(&e, GET_SEEN_DATA).is_empty());
    }

    #[test]
    fn fn_00555f60_creates_the_seen_data_of_an_exterior_cell() {
        let mut e = engine();
        let this = cell(&mut e, false, false);
        e.put_vtable(0x0300_0000, &[0; 0x13]);
        e.mem.set_u32(0x0300_0000 + 0x48, 0x0300_1000);
        e.mem.set_u32(this, 0x0300_0000);
        e.register(0x0300_1000, nothing);
        e.register(GET_SEEN_DATA, |_, _| word(0));
        e.register(ALLOCATE, |_, _| word(0x0060_0000));
        e.register(SEEN_DATA_CONSTRUCT, |_, a| word(a[0] + 1));
        e.register(EXTRA_DATA, |_, a| word(a[0] + 0x100));
        e.register(SET_SEEN_DATA, nothing);
        e.register(CELL_DATA_X, |_, _| word(2));
        e.register(CELL_DATA_Y, |_, _| word(-3i32 as u32));
        e.register(SEEN_DATA_UPDATE, |_, _| word(1));
        e.register(MARK_FULLY_SEEN, nothing);
        e.register(SET_CELL_FLAG, nothing);
        let position = e.mem.alloc(0xc);
        e.mem.set_f32(position + 8, 7.0);
        e.call_log = Some(vec![]);
        e.call(0x0055_5f60, &args![this, position]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x24]]);
        assert_eq!(
            calls_to(&e, SET_SEEN_DATA),
            vec![vec![this + 0x100, 0x0060_0001]]
        );
        assert_eq!(calls_to(&e, 0x0300_1000), vec![vec![this, 0x8000_0000]]);
        let corner = calls_to(&e, POINT_CONSTRUCT)[0].clone();
        assert_eq!(
            &corner[1..],
            &[8192f32.to_bits(), (-12288f32).to_bits(), 7f32.to_bits()]
        );
        assert_eq!(
            calls_to(&e, SEEN_DATA_UPDATE),
            vec![vec![0x0060_0001, position, corner[0]]]
        );
        assert_eq!(calls_to(&e, MARK_FULLY_SEEN), vec![vec![this]]);
        assert_eq!(calls_to(&e, SET_CELL_FLAG), vec![vec![this, 1]]);
    }

    #[test]
    fn fn_00555f60_updates_the_interior_section_and_its_neighbours() {
        let mut e = engine();
        let this = cell(&mut e, true, false);
        e.register(NORTH_ROTATION, |_, _| Ret::default());
        e.register(INT_SEEN_SECTION, |_, a| {
            word(0x5000u32.wrapping_add(a[1] << 4).wrapping_add(a[2] & 0xf))
        });
        e.register(SEEN_DATA_UPDATE, |e, a| {
            // Keep the x of each section center seen.
            let x = e.mem.f32(a[2]);
            Ret {
                eax: x as u32,
                ..Ret::default()
            }
        });
        let position = e.mem.alloc(0xc);
        // Section (0, 0): x 2100 is within 200 of its low corner (2048);
        // y 3000 is near neither corner.
        e.mem.set_f32(position, 2100.0);
        e.mem.set_f32(position + 4, 3000.0);
        e.mem.set_f32(position + 8, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x0055_5f60, &args![this, position]);
        // Section (0, 0) and its low-x neighbour (-1, 0).
        assert_eq!(
            calls_to(&e, INT_SEEN_SECTION),
            vec![vec![this, 0, 0, 1], vec![this, -1i32 as u32, 0, 1]]
        );
        let updates = calls_to(&e, SEEN_DATA_UPDATE);
        assert_eq!(updates.len(), 2);
        assert_eq!(updates[0][0], 0x5000);
        assert_eq!(e.mem.f32(updates[0][2]), 2048.0);
        assert_eq!(e.mem.f32(updates[0][2] + 8), 1.0);
        assert_eq!(updates[1][0], 0x5000u32.wrapping_add(0xffff_ffff << 4));
        // The high corner is the exe's ((section + 0x1000) << 12) + 0x800.
        let corners = calls_to(&e, POINT2_CONSTRUCT);
        assert_eq!(corners[1][1], (0x0100_0800 as f32).to_bits());
    }

    #[test]
    fn fn_00555f60_updates_nine_rotated_sections_until_one_is_missing() {
        let mut e = engine();
        let this = cell(&mut e, true, false);
        e.register(NORTH_ROTATION, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.register(ADJUST_FOR_NORTH_ROTATION, |e, a| {
            e.mem.set_f32(a[2], 10000.0);
            e.mem.set_f32(a[2] + 4, 10000.0);
            Ret::default()
        });
        // The fifth section is missing.
        e.register(INT_SEEN_SECTION, |_, a| {
            word(if a[1] == 1 && a[2] == 1 {
                0
            } else {
                0x5000 + a[1]
            })
        });
        e.register(SEEN_DATA_UPDATE, nothing);
        let position = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        e.call(0x0055_5f60, &args![this, position]);
        // 10000 - 0x800 = 7952, >> 12 = 1: sections 0..2 around (1, 1).
        let sections = calls_to(&e, INT_SEEN_SECTION);
        assert_eq!(sections.len(), 9);
        assert_eq!(sections[0], vec![this, 0, 0, 1]);
        assert_eq!(sections[8], vec![this, 2, 2, 1]);
        assert_eq!(calls_to(&e, SEEN_DATA_UPDATE).len(), 4);
        assert_eq!(
            calls_to(&e, ADJUST_FOR_NORTH_ROTATION)[0][..2],
            [this, position]
        );
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT)[0][1..],
            [0xc, 9, POINT_ELEMENT_CONSTRUCT]
        );
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    #[test]
    fn fn_00555be0_clears_the_seen_data() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA, |_, a| Ret {
            eax: a[0] + 0x44,
            ..Ret::default()
        });
        e.register(SET_SEEN_DATA, |_, _| Ret::default());
        e.register(0x0090_004c, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        let vtable = e.mem.alloc(0x50);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0x4c, 0x0090_004c);
        e.mem.set_u8(this + 0x25, 0xff);
        e.call_log = Some(vec![]);
        e.call(0x0055_5be0, &[this]);
        assert_eq!(e.mem.u8(this + 0x25), 0xfe);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| [SET_SEEN_DATA, 0x0090_004c].contains(a))
            .collect();
        assert_eq!(
            log,
            vec![
                (SET_SEEN_DATA, vec![this + 0x44, 0]),
                (0x0090_004c, vec![this, 0x8000_0000]),
            ]
        );
    }
}
