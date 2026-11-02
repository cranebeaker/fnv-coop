//! `Havok SDK/hkpstoragesampledheightfieldshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The class (no Xbox PDB record; the field names are descriptive and unconfirmed) is a
//! sampled height field that owns its samples: the vtable at `+0`, the grid size at `+0x10`
//! (columns) and `+0x14` (rows), a flag byte at `+0x1c` (collide mode), the vectors at `+0x20`,
//! `+0x30` and `+0x40`, the pointer to the samples (one `float` each) at `+0x60`, a byte copied
//! from the source's virtual `+0x38` at `+0x64` and a byte at `+0x70` that says the samples are
//! owned. The virtual functions used here: slot `0x34` returns the height at a column and a
//! row (two integers, the result in `ST0`) and slot `0x38` writes a flag byte through its
//! pointer argument (both `stdcall`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpboxmotion::{add, mul, sub};
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, inverse_length, load_vec, store_vec, sum3, ZERO_F32,
};

/// The class's vtable.
const HEIGHT_FIELD_VTABLE: u32 = 0x010c_fb34;
/// The CRT allocation function (`00401000`, cdecl, one word) and free function (`00401030`,
/// cdecl, one pointer).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// The base class's destructor (`00d24660`, `this`).
const BASE_DESTRUCT: u32 = 0x00d2_4660;
/// The constants `collideSpheres` uses: the `float` 1.0 and the large `float` that make the
/// default result `(0, 1, 0, large)`, the `double` 1.0 and the vector (`3 * 2^16` in every
/// lane) that is added to the scaled position so that the column and the row are in the low
/// bits of the sums.
const ONE_F32: u32 = 0x0101_2068;
const LARGE_F32: u32 = 0x0104_da48;
const ONE_F64: u32 = 0x0101_2070;
const CELL_BIAS: u32 = 0x011b_1bd0;

// Translated from 00d496b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpStorageSampledHeightFieldShape::calcSizeForSpu` (Xbox PDB; a virtual method with two
/// stack words, the first unused): 0x80 when the second word (signed) is at least 0x80, else
/// 0xffffffff.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hkp_storage_sampled_height_field_shape_calc_size_for_spu(
    _e: &mut Engine,
    _this: u32,
    _unused: u32,
    buffer_size: u32,
) -> u32 {
    if buffer_size as i32 >= 0x80 {
        0x80
    } else {
        0xffff_ffff
    }
}

// Translated from 00d496d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `hkpStorageSampledHeightFieldShape` (no engine-map name): installs the
/// vtable, frees the sample array (`00401030` on the pointer at `+0x60`) when the byte at
/// `+0x70` says it is owned and runs the base class's destructor (`00d24660`).
pub fn fn_00d496d0(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    let owned = e.mem.u8(object + 0x70);
    e.mem.set_u32(object, HEIGHT_FIELD_VTABLE);
    if owned != 0 {
        let samples = e.mem.u32(object + 0x60);
        e.call(FREE, &args![samples]);
    }
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 00d49750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor that copies another height field (no engine-map name), `(this, source)`: the
/// 16-bit count at `+6` is 1, the words at `+8` and `+0xc` are 0 and 0xd, the vtable is
/// installed, the byte at `+0x1d` is 0 and the grid size (`+0x10`, `+0x14`) is copied. The
/// source's virtual `0x38` writes a byte that goes to `+0x64`; the `float` at `+0x18` is copied
/// through the x87 (a signalling NaN comes out quiet) and the four 16-byte values at `+0x20`
/// to `+0x50`. `columns * rows * 4` bytes are allocated (`00401000`; the request is 0xffffffff
/// when the byte count does not fit in 32 bits) as the sample array (`+0x60`), the owned byte
/// (`+0x70`) is set and, for each row and column (signed counts), the source's height (virtual
/// `0x34`, column and row) is stored as a `float` at index `columns * row + column`. Returns
/// `this`.
pub fn fn_00d49750(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let object = this.addr();
    let src = source.addr();
    e.mem.set_u16(object + 6, 1);
    e.mem.set_u32(object + 8, 0);
    e.mem.set_u32(object + 0xc, 0xd);
    e.mem.set_u32(object, HEIGHT_FIELD_VTABLE);
    e.mem.set_u8(object + 0x1d, 0);
    let columns = e.mem.u32(src + 0x10);
    e.mem.set_u32(object + 0x10, columns);
    let rows = e.mem.u32(src + 0x14);
    e.mem.set_u32(object + 0x14, rows);
    let flag = e.with_stack(4, |e, byte| {
        let byte = byte.addr();
        e.vcall(src, 0x38, &args![byte]);
        e.mem.u8(byte)
    });
    e.mem.set_u8(object + 0x64, flag);
    let spacing = x87_quiet_bits(e.mem.u32(src + 0x18));
    e.mem.set_u32(object + 0x18, spacing);
    let samples = rows.wrapping_mul(columns);
    for offset in [0x20, 0x30, 0x40, 0x50] {
        copy_vec(e, src + offset, object + offset);
    }
    let bytes = u64::from(samples) * 4;
    let request = if bytes >> 32 != 0 {
        0xffff_ffff
    } else {
        bytes as u32
    };
    let storage = e.call(ALLOCATE, &args![request]).u32();
    e.mem.set_u32(object + 0x60, storage);
    e.mem.set_u8(object + 0x70, 1);
    let mut row = 0u32;
    if (e.mem.u32(object + 0x14) as i32) > 0 {
        loop {
            let mut column = 0u32;
            if (e.mem.u32(object + 0x10) as i32) > 0 {
                loop {
                    let height = e.vcall(src, 0x34, &args![column, row]).f32();
                    let index = e
                        .mem
                        .u32(object + 0x10)
                        .wrapping_mul(row)
                        .wrapping_add(column);
                    column = column.wrapping_add(1);
                    let base = e.mem.u32(object + 0x60);
                    e.mem
                        .set_f32(base.wrapping_add(index.wrapping_mul(4)), height);
                    if column as i32 >= e.mem.u32(object + 0x10) as i32 {
                        break;
                    }
                }
            }
            row = row.wrapping_add(1);
            if row as i32 >= e.mem.u32(object + 0x14) as i32 {
                break;
            }
        }
    }
    this
}

/// An `f32` widened as the x87 does when it loads it.
fn wide(value: f32) -> f64 {
    f64::from(value)
}

/// The height at a column and a row: the virtual at slot `0x34`, the result from `ST0`.
fn height_at(e: &mut Engine, field: u32, column: u32, row: u32) -> f32 {
    e.vcall(field, 0x34, &args![column, row]).f32()
}

/// The surface `collideSpheres` found under one sphere.
struct CellSample {
    /// The slope along the columns and along the rows (the result holds their negatives).
    slope_column: f32,
    slope_row: f32,
    /// The height of the surface at the sphere's position.
    height: f32,
    /// The column and the height of the corner the plane is taken through.
    corner_column: u32,
    corner_height: f32,
}

/// The triangle of the cell `(column, row)` that the fractions `(fx, fz)` fall in and its plane.
/// With the flip byte set the cell is split along the diagonal from `(column, row)` to
/// `(column + 1, row + 1)`, `fz < fx` choosing the triangle with `(column + 1, row)`;
/// otherwise along the other diagonal, `fz + fx > 1.0` (the `double` at `01012070`) choosing
/// the triangle with `(column + 1, row + 1)`. The slopes are rounded to `float` where the exe
/// stores them; the sums are formed in double precision.
fn sample_cell(
    e: &mut Engine,
    field: u32,
    column: u32,
    row: u32,
    fx: f32,
    fz: f32,
    flip: bool,
) -> CellSample {
    if flip {
        let h00 = height_at(e, field, column, row);
        let h11 = height_at(e, field, column.wrapping_add(1), row.wrapping_add(1));
        let (slope_column, slope_row) = if fz < fx {
            let h10 = height_at(e, field, column.wrapping_add(1), row);
            (
                (wide(h10) - wide(h00)) as f32,
                (wide(h11) - wide(h10)) as f32,
            )
        } else {
            let h01 = height_at(e, field, column, row.wrapping_add(1));
            (
                (wide(h11) - wide(h01)) as f32,
                (wide(h01) - wide(h00)) as f32,
            )
        };
        let height =
            ((wide(h00) + wide(slope_row) * wide(fz)) + wide(slope_column) * wide(fx)) as f32;
        CellSample {
            slope_column,
            slope_row,
            height,
            corner_column: column,
            corner_height: h00,
        }
    } else {
        let h10 = height_at(e, field, column.wrapping_add(1), row);
        let h01 = height_at(e, field, column, row.wrapping_add(1));
        let limit = e.mem.f64(ONE_F64);
        let (slope_column, slope_row, height) = if wide(fz) + wide(fx) > limit {
            let h11 = height_at(e, field, column.wrapping_add(1), row.wrapping_add(1));
            let slope_column = (wide(h11) - wide(h01)) as f32;
            let slope_row = (wide(h11) - wide(h10)) as f32;
            let near = wide(h10) + wide(slope_row) * wide(fz);
            let height = ((wide(fx) - limit) * wide(slope_column) + near) as f32;
            (slope_column, slope_row, height)
        } else {
            let h00 = height_at(e, field, column, row);
            let slope_column = (wide(h10) - wide(h00)) as f32;
            let slope_row = (wide(h01) - wide(h00)) as f32;
            let height =
                ((wide(h00) + wide(slope_row) * wide(fz)) + wide(slope_column) * wide(fx)) as f32;
            (slope_column, slope_row, height)
        };
        CellSample {
            slope_column,
            slope_row,
            height,
            corner_column: column.wrapping_add(1),
            corner_height: h10,
        }
    }
}

// Translated from 00d49850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpStorageSampledHeightFieldShape::collideSpheres` (engine map), `(this, input, output)`
/// (`stdcall` with two stack words): `input` holds a pointer to the spheres (16-byte vectors:
/// position and radius) and their count, `output` receives one 16-byte result per sphere. Each
/// result starts as the default `(0, 1, 0, large)` (the `float`s at `01011d78`, `01012068`,
/// `01011d78` and `0104da48`). The position scaled by the vector at `+0x30` is also formed
/// with the vector at `+0x40` added first and the bias at `011b1bd0` added after; the column
/// and the row are the low 16 bits of the bit patterns of that sum's x and z lanes shifted
/// right by 6. A sphere whose column is not below `columns - 1` or whose row is not below
/// `rows - 1` (signed) keeps the default. For the others the fractions `fx` and `fz` are the
/// scaled position less the column and the row (rounded to `float`) and the flip byte is
/// written by the virtual at slot `0x38`; [`sample_cell`] gives the slopes and the surface
/// height. The result's x and z become the negated slopes, the result is scaled by the vector at
/// `+0x30` and normalised (the squares summed as `z + (y + x)`, the refined `RSQRTSS`, 0 for a
/// zero sum) and its w lane is the signed distance: with the byte at `+0x1c` zero it is `(scaled
/// y - surface height) * (float at +0x24) - radius`, otherwise the sum (as `z + (y + x)`) of
/// `(scaled position - (corner column, corner height, row, 0)) * (vector at +0x20) * normal`
/// less the radius.
pub fn hkp_storage_sampled_height_field_shape_collide_spheres(
    e: &mut Engine,
    this: Ptr,
    input: Ptr,
    output: Ptr,
) {
    let field = this.addr();
    let spheres = e.mem.u32(input.addr());
    let count = e.mem.u32(input.addr() + 4);
    let default = [
        e.mem.u32(ZERO_F32),
        e.mem.u32(ONE_F32),
        e.mem.u32(ZERO_F32),
        e.mem.u32(LARGE_F32),
    ];
    let mut remaining = count.wrapping_sub(1) as i32;
    let mut cursor = spheres;
    let mut out = output.addr();
    while remaining >= 0 {
        for (lane, word) in default.into_iter().enumerate() {
            e.mem.set_u32(out + 4 * lane as u32, word);
        }
        let sphere = load_vec(e, cursor);
        let scale = load_vec(e, field + 0x30);
        let scaled = mul(sphere, scale);
        let shifted = mul(add(sphere, load_vec(e, field + 0x40)), scale);
        let biased = add(shifted, load_vec(e, CELL_BIAS));
        let column = (biased[0].to_bits() >> 6) & 0xffff;
        let row = (biased[2].to_bits() >> 6) & 0xffff;
        let last_column = e.mem.u32(field + 0x10).wrapping_sub(1) as i32;
        let last_row = e.mem.u32(field + 0x14).wrapping_sub(1) as i32;
        if (column as i32) < last_column && (row as i32) < last_row {
            let fx = (wide(scaled[0]) - f64::from(column)) as f32;
            let fz = (wide(scaled[2]) - f64::from(row)) as f32;
            let flip = e.with_stack(4, |e, byte| {
                let byte = byte.addr();
                e.vcall(field, 0x38, &args![byte]);
                e.mem.u8(byte)
            });
            let cell = sample_cell(e, field, column, row, fx, fz, flip != 0);
            e.mem.set_f32(out, -cell.slope_column);
            e.mem.set_f32(out + 8, -cell.slope_row);
            let result = load_vec(e, out);
            let direction = mul(load_vec(e, field + 0x30), result);
            let length_squared = sum3(mul(direction, direction));
            store_vec(e, out, direction);
            let normal = mul([inverse_length(e, length_squared); 4], direction);
            store_vec(e, out, normal);
            let radius = e.mem.f32(cursor + 0xc);
            let distance = if e.mem.u8(field + 0x1c) == 0 {
                let above = wide(scaled[1]) - wide(cell.height);
                (above * wide(e.mem.f32(field + 0x24)) - wide(radius)) as f32
            } else {
                let origin = [
                    cell.corner_column as f32,
                    cell.corner_height,
                    row as f32,
                    f32::from_bits(e.mem.u32(ZERO_F32)),
                ];
                let relative = mul(mul(sub(scaled, origin), load_vec(e, field + 0x20)), normal);
                (wide(sum3(relative)) - wide(radius)) as f32
            };
            e.mem.set_f32(out + 0xc, distance);
        }
        cursor = cursor.wrapping_add(0x10);
        out = out.wrapping_add(0x10);
        remaining = remaining.wrapping_sub(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_96b0, hkp_storage_sampled_height_field_shape_calc_size_for_spu(u32, u32, u32) -> u32),
        entry!(0x00d4_96d0, fn_00d496d0(Ptr)),
        entry!(0x00d4_9750, fn_00d49750(Ptr, Ptr) -> Ptr),
        entry!(
            0x00d4_9850,
            hkp_storage_sampled_height_field_shape_collide_spheres(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{HALF_F32, THREE_F32};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    const HEIGHT_SLOT_DOUBLE: u32 = 0x0900_0034;
    const FLAG_SLOT_DOUBLE: u32 = 0x0900_0038;
    const LARGE: f32 = 3.0e30;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(ONE_F32, 1.0f32);
        e.set_global(LARGE_F32, LARGE);
        e.set_global(ONE_F64, 1.0f64);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        for lane in 0..4 {
            e.set_global(CELL_BIAS + 4 * lane, 196608.0f32);
        }
        e
    }

    /// A surface that is not a plane: `2 * column + 3 * row + column * row / 4`.
    fn surface(column: u32, row: u32) -> f64 {
        f64::from(2 * column + 3 * row) + f64::from(column * row) / 4.0
    }

    /// A height field object whose slot `0x34` answers [`surface`] (and logs the cells asked
    /// for) and whose slot `0x38` writes `flip`.
    fn height_field(e: &mut Engine, flip: u8) -> (u32, Rc<RefCell<Vec<(u32, u32)>>>) {
        let asked = Rc::new(RefCell::new(Vec::new()));
        let asked_log = Rc::clone(&asked);
        e.register_double(HEIGHT_SLOT_DOUBLE, move |_, a| {
            asked_log.borrow_mut().push((a[1], a[2]));
            Ret {
                st0: surface(a[1], a[2]),
                ..Ret::default()
            }
        });
        e.register_double(FLAG_SLOT_DOUBLE, move |e, a| {
            e.mem.set_u8(a[1], flip);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x34, HEIGHT_SLOT_DOUBLE);
        e.mem.set_u32(vtable + 0x38, FLAG_SLOT_DOUBLE);
        let field = e.mem.alloc(0x80);
        e.mem.set_u32(field, vtable);
        e.mem.set_u32(field + 0x10, 4);
        e.mem.set_u32(field + 0x14, 4);
        for lane in 0..4 {
            e.mem.set_f32(field + 0x20 + 4 * lane, 1.0);
            e.mem.set_f32(field + 0x30 + 4 * lane, 1.0);
        }
        (field, asked)
    }

    /// Runs `collideSpheres` for one sphere (position `p`, radius 0.5).
    fn collide(e: &mut Engine, field: u32, position: [f32; 3]) -> [f32; 4] {
        let sphere = e.mem.alloc(0x10);
        for (lane, value) in position.into_iter().enumerate() {
            e.mem.set_f32(sphere + 4 * lane as u32, value);
        }
        e.mem.set_f32(sphere + 0xc, 0.5);
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, sphere);
        e.mem.set_u32(input + 4, 1);
        let output = e.mem.alloc(0x10);
        e.call(0x00d4_9850, &args![field, input, output]);
        std::array::from_fn(|lane| e.mem.f32(output + 4 * lane as u32))
    }

    fn assert_near(got: f32, want: f64) {
        assert!(
            (f64::from(got) - want).abs() < 1e-4,
            "got {got}, wanted {want}"
        );
    }

    #[test]
    fn calc_size_for_spu_needs_a_signed_size_of_at_least_0x80() {
        let mut e = engine();
        let address = 0x00d4_96b0;
        assert_eq!(
            e.call(address, &args![VCALL_THIS, 7u32, 0x80u32]).u32(),
            0x80
        );
        assert_eq!(
            e.call(address, &args![VCALL_THIS, 7u32, 0x7fff_ffffu32])
                .u32(),
            0x80
        );
        assert_eq!(
            e.call(address, &args![VCALL_THIS, 7u32, 0x7fu32]).u32(),
            0xffff_ffff
        );
        assert_eq!(
            e.call(address, &args![VCALL_THIS, 7u32, 0xffff_ff00u32])
                .u32(),
            0xffff_ffff
        );
    }

    #[test]
    fn destructor_frees_the_samples_only_when_owned() {
        let mut e = engine();
        let log = Rc::new(RefCell::new(Vec::new()));
        let free_log = Rc::clone(&log);
        e.register_double(FREE, move |_, a| {
            free_log.borrow_mut().push((FREE, a[0]));
            Ret::default()
        });
        let base_log = Rc::clone(&log);
        e.register_double(BASE_DESTRUCT, move |_, a| {
            base_log.borrow_mut().push((BASE_DESTRUCT, a[0]));
            Ret::default()
        });
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x60, 0x1234);
        e.mem.set_u8(this + 0x70, 1);
        e.call(0x00d4_96d0, &args![this]);
        assert_eq!(e.mem.u32(this), HEIGHT_FIELD_VTABLE);
        assert_eq!(*log.borrow(), vec![(FREE, 0x1234), (BASE_DESTRUCT, this)]);
        log.borrow_mut().clear();
        e.mem.set_u8(this + 0x70, 0);
        e.call(0x00d4_96d0, &args![this]);
        assert_eq!(*log.borrow(), vec![(BASE_DESTRUCT, this)]);
    }

    #[test]
    fn constructor_copies_the_source_and_samples_its_heights() {
        let mut e = engine();
        let (source, asked) = height_field(&mut e, 1);
        e.mem.set_u32(source + 0x10, 3);
        e.mem.set_u32(source + 0x14, 2);
        e.mem.set_u32(source + 0x18, 0x7fa0_0000);
        for lane in 0..16 {
            e.mem.set_u8(source + 0x20 + lane, lane as u8 + 1);
            e.mem.set_u8(source + 0x50 + lane, 0x80 + lane as u8);
        }
        let requests = Rc::new(RefCell::new(Vec::new()));
        let request_log = Rc::clone(&requests);
        let storage = e.mem.alloc(0x40);
        e.register_double(ALLOCATE, move |_, a| {
            request_log.borrow_mut().push(a[0]);
            Ret {
                eax: storage,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x80);
        let got = e.call(0x00d4_9750, &args![this, source]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0xd);
        assert_eq!(e.mem.u32(this), HEIGHT_FIELD_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 3);
        assert_eq!(e.mem.u32(this + 0x14), 2);
        assert_eq!(e.mem.u8(this + 0x64), 1);
        assert_eq!(e.mem.u32(this + 0x18), 0x7fe0_0000);
        assert_eq!(e.mem.bytes(this + 0x20, 16), e.mem.bytes(source + 0x20, 16));
        assert_eq!(e.mem.bytes(this + 0x50, 16), e.mem.bytes(source + 0x50, 16));
        assert_eq!(*requests.borrow(), vec![24]);
        assert_eq!(e.mem.u32(this + 0x60), storage);
        assert_eq!(e.mem.u8(this + 0x70), 1);
        let rows_then_columns: Vec<(u32, u32)> = (0..2)
            .flat_map(|row| (0..3).map(move |column| (column, row)))
            .collect();
        assert_eq!(*asked.borrow(), rows_then_columns);
        for (column, row) in rows_then_columns {
            let sample = e.mem.f32(storage + 4 * (3 * row + column));
            assert_eq!(f64::from(sample), surface(column, row));
        }
    }

    #[test]
    fn constructor_requests_all_ones_when_the_byte_count_overflows() {
        let mut e = engine();
        let (source, asked) = height_field(&mut e, 0);
        e.mem.set_u32(source + 0x10, 0x4000_0000);
        e.mem.set_u32(source + 0x14, 0xffff_ffff);
        let requests = Rc::new(RefCell::new(Vec::new()));
        let request_log = Rc::clone(&requests);
        e.register_double(ALLOCATE, move |_, a| {
            request_log.borrow_mut().push(a[0]);
            Ret::default()
        });
        let this = e.mem.alloc(0x80);
        e.call(0x00d4_9750, &args![this, source]);
        assert_eq!(*requests.borrow(), vec![0xffff_ffff]);
        assert!(asked.borrow().is_empty());
    }

    #[test]
    fn lower_triangle_of_an_unflipped_cell_uses_the_origin_corner() {
        let mut e = engine();
        let (field, asked) = height_field(&mut e, 0);
        let result = collide(&mut e, field, [1.25, 5.0, 1.5]);
        assert_eq!(*asked.borrow(), vec![(2, 1), (1, 2), (1, 1)]);
        // The plane through (1, 1), (2, 1) and (1, 2); the sphere is 0.25 and 0.5 into the cell.
        let (h00, h10, h01) = (surface(1, 1), surface(2, 1), surface(1, 2));
        let (slope_column, slope_row) = (h10 - h00, h01 - h00);
        let height = h00 + slope_row * 0.5 + slope_column * 0.25;
        let length = (slope_column * slope_column + 1.0 + slope_row * slope_row).sqrt();
        assert_near(result[0], -slope_column / length);
        assert_near(result[1], 1.0 / length);
        assert_near(result[2], -slope_row / length);
        assert_near(result[3], (5.0 - height) - 0.5);
    }

    #[test]
    fn upper_triangle_of_an_unflipped_cell_uses_the_far_corner() {
        let mut e = engine();
        let (field, asked) = height_field(&mut e, 0);
        let result = collide(&mut e, field, [1.75, 5.0, 1.5]);
        assert_eq!(*asked.borrow(), vec![(2, 1), (1, 2), (2, 2)]);
        let (h10, h01, h11) = (surface(2, 1), surface(1, 2), surface(2, 2));
        let (slope_column, slope_row) = (h11 - h01, h11 - h10);
        let height = (h10 + slope_row * 0.5) + (0.75 - 1.0) * slope_column;
        let length = (slope_column * slope_column + 1.0 + slope_row * slope_row).sqrt();
        assert_near(result[0], -slope_column / length);
        assert_near(result[2], -slope_row / length);
        assert_near(result[3], (5.0 - height) - 0.5);
    }

    #[test]
    fn flipped_cell_picks_its_triangle_by_the_fractions() {
        let mut e = engine();
        let (field, asked) = height_field(&mut e, 1);
        // fz < fx: the triangle with (column + 1, row).
        let result = collide(&mut e, field, [1.75, 5.0, 1.25]);
        assert_eq!(*asked.borrow(), vec![(1, 1), (2, 2), (2, 1)]);
        let (h00, h10, h11) = (surface(1, 1), surface(2, 1), surface(2, 2));
        let (slope_column, slope_row) = (h10 - h00, h11 - h10);
        let height = h00 + slope_row * 0.25 + slope_column * 0.75;
        let length = (slope_column * slope_column + 1.0 + slope_row * slope_row).sqrt();
        assert_near(result[0], -slope_column / length);
        assert_near(result[2], -slope_row / length);
        assert_near(result[3], (5.0 - height) - 0.5);
        // fz >= fx: the triangle with (column, row + 1).
        asked.borrow_mut().clear();
        let result = collide(&mut e, field, [1.25, 5.0, 1.75]);
        assert_eq!(*asked.borrow(), vec![(1, 1), (2, 2), (1, 2)]);
        let h01 = surface(1, 2);
        let (slope_column, slope_row) = (h11 - h01, h01 - h00);
        let height = h00 + slope_row * 0.75 + slope_column * 0.25;
        let length = (slope_column * slope_column + 1.0 + slope_row * slope_row).sqrt();
        assert_near(result[0], -slope_column / length);
        assert_near(result[2], -slope_row / length);
        assert_near(result[3], (5.0 - height) - 0.5);
    }

    #[test]
    fn spheres_outside_the_grid_keep_the_default_result() {
        let mut e = engine();
        let (field, asked) = height_field(&mut e, 0);
        // The last column (3) and the last row (3) are not cells.
        for position in [[3.5, 5.0, 1.5], [1.5, 5.0, 3.5], [3.0, 5.0, 3.0]] {
            let result = collide(&mut e, field, position);
            assert_eq!(result, [0.0, 1.0, 0.0, LARGE]);
        }
        assert!(asked.borrow().is_empty());
    }

    #[test]
    fn the_collide_flag_measures_along_the_normal_from_the_corner() {
        let mut e = engine();
        let (field, _) = height_field(&mut e, 0);
        e.mem.set_u8(field + 0x1c, 1);
        let result = collide(&mut e, field, [1.25, 5.0, 1.5]);
        let (h00, h10, h01) = (surface(1, 1), surface(2, 1), surface(1, 2));
        let (slope_column, slope_row) = (h10 - h00, h01 - h00);
        let length = (slope_column * slope_column + 1.0 + slope_row * slope_row).sqrt();
        // The corner is (column + 1, height at (2, 1), row) in this triangle.
        let along = (-slope_column * (1.25 - 2.0) + (5.0 - h10) - slope_row * (1.5 - 1.0)) / length;
        assert_near(result[3], along - 0.5);
    }

    #[test]
    fn the_scale_vector_scales_the_result_before_it_is_normalised() {
        let mut e = engine();
        let (field, _) = height_field(&mut e, 0);
        e.mem.set_f32(field + 0x30 + 4, 2.0);
        e.mem.set_f32(field + 0x24, 0.5);
        let result = collide(&mut e, field, [1.25, 2.5, 1.5]);
        // The position's y is scaled to 5.0; the result vector keeps y = 1 times the scale 2.
        let (h00, h10, h01) = (surface(1, 1), surface(2, 1), surface(1, 2));
        let (slope_column, slope_row) = (h10 - h00, h01 - h00);
        let height = h00 + slope_row * 0.5 + slope_column * 0.25;
        let length = (slope_column * slope_column + 4.0 + slope_row * slope_row).sqrt();
        assert_near(result[0], -slope_column / length);
        assert_near(result[1], 2.0 / length);
        assert_near(result[3], (5.0 - height) * 0.5 - 0.5);
    }
}
