//! `fallout shared/bgsworldlocation.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `BGSWorldLocation` is a point and the form it lies in (`pSpace`).
//! The code tells the two kinds of space apart by form type (`00401170`,
//! the byte at `TESForm + 4`): 0x39, answered as the cell itself, and 0x41,
//! which it hands to `TESWorldSpace::GetCellFromCellCoord` (Xbox PDB).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSWorldLocation` (Xbox PDB), 0x10 bytes (the PC offsets agree).
    pub struct BGSWorldLocation: 0x10 {
        /// `Point` (Xbox PDB): `NiPoint3`.
        0x00 Point: Inline<crate::types::NiPoint3>,
        /// `pSpace` (Xbox PDB): `TESForm*`.
        0x0C pSpace: u32,
    }
}

/// The form type byte of a form (`00401170`: `TESForm + 4`).
const FORM_TYPE: u32 = 0x0040_1170;
/// The space form type answered as a cell.
const SPACE_CELL: u32 = 0x39;
/// The space form type used as a world space.
const SPACE_WORLD: u32 = 0x41;
/// `FLD float; FISTP dword` of a float argument (`00406d90`, cdecl).
const FLOAT_TO_INT: u32 = 0x0040_6d90;
/// `TESWorldSpace::GetCellFromCellCoord` (Xbox PDB, `005875a0`, `RET 8`).
const GET_CELL_FROM_CELL_COORD: u32 = 0x0058_75a0;
/// The `TES` singleton pointer.
const TES_SINGLETON: u32 = 0x011d_ea10;
/// `TES::IsCellLoaded` (Xbox PDB, `004511e0`, `RET 8`).
const IS_CELL_LOADED: u32 = 0x0045_11e0;
/// The far distance answered for points in different spaces: the `float`
/// at `01016970` (`FLT_MAX` in the exe).
const FAR_DISTANCE: u32 = 0x0101_6970;
/// The difference of two points into a result (`00439ef0`, `RET 8`: called
/// on the other point with the result and this point, it gives other - this).
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// The squared length of a point (`004a7290`, `ST0`).
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
/// The length of a point (`00457990`, `ST0`).
const POINT_LENGTH: u32 = 0x0045_7990;
/// Bit 0 of a cell's flag byte at `+0x24` (`00425fd0`).
const CELL_FLAG_0: u32 = 0x0042_5fd0;
/// `004839c0`: the form with a form id (cdecl).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// The `NiPoint3` the default location copies (`011f426c`).
const DEFAULT_POINT: u32 = 0x011f_426c;

/// `FLD a; FMUL dword b; FSTP dword`: `a` is loaded from a `float` (so a
/// signalling NaN comes in quiet), `b` is the memory operand as stored.
/// The exact product of two floats fits a double, so the product is
/// rounded once. NaNs follow the x87 rules as `expression.rs` records them
/// from the real CPU: one NaN gives that NaN quieted; with two, a
/// signalling memory operand gives the register's NaN, otherwise the
/// larger significand wins and on equal significands the positive one; an
/// invalid product (zero times infinity) is the indefinite NaN.
fn x87_mul_store(a: f32, b: f32) -> f32 {
    let a = x87_quiet(a);
    let significand = |v: f32| v.to_bits() & 0x007f_ffff;
    match (a.is_nan(), b.is_nan()) {
        (true, true) => {
            if b.to_bits() & 0x0040_0000 == 0 {
                a
            } else if significand(b) > significand(a) {
                b
            } else if significand(b) < significand(a) || a.is_sign_positive() {
                a
            } else {
                b
            }
        }
        (true, false) => a,
        (false, true) => x87_quiet(b),
        (false, false) => {
            let product = a as f64 * b as f64;
            if product.is_nan() {
                f32::from_bits(0xffc0_0000)
            } else {
                product as f32
            }
        }
    }
}

fn form_type(e: &mut Engine, form: u32) -> u32 {
    e.call(FORM_TYPE, &args![form]).u32()
}

/// `00406d90` on a float word pushed by `FLD`/`FSTP`.
fn float_to_int(e: &mut Engine, at: u32) -> i32 {
    let bits = x87_quiet_bits(e.mem.u32(at));
    e.call(FLOAT_TO_INT, &args![bits]).i32()
}

/// The far distance as `FLD` loads it.
fn far_distance(e: &Engine) -> f32 {
    x87_quiet(e.global::<f32>(FAR_DISTANCE))
}

// Translated from 00527900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSWorldLocation::GetCell` (Xbox PDB): null without a space; the space
/// itself when its form type is 0x39; for type 0x41 the cell
/// `TESWorldSpace::GetCellFromCellCoord` answers for the point's cell
/// coordinates (x and y each through `00406d90`, shifted right by 12,
/// arithmetic); null for any other type.
pub fn bgs_world_location_get_cell(e: &mut Engine, this: Ptr<BGSWorldLocation>) -> u32 {
    let space = e.get(this, BGSWorldLocation::pSpace);
    if space == 0 {
        return 0;
    }
    match form_type(e, space) {
        SPACE_CELL => space,
        SPACE_WORLD => {
            let x = float_to_int(e, this.addr()) >> 12;
            let y = float_to_int(e, this.addr() + 4) >> 12;
            e.call(GET_CELL_FROM_CELL_COORD, &args![space, x, y]).u32()
        }
        _ => 0,
    }
}

// Translated from 00527990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSWorldLocation::GetCellOrWorld` (Xbox PDB): for a space of type 0x41
/// stores it in `*world` and null in `*cell`; for type 0x39 stores it in
/// `*cell` and null in `*world`; true in both cases. False, storing
/// nothing, without a space or for another type.
pub fn bgs_world_location_get_cell_or_world(
    e: &mut Engine,
    this: Ptr<BGSWorldLocation>,
    cell: Ptr,
    world: Ptr,
) -> bool {
    let space = e.get(this, BGSWorldLocation::pSpace);
    if space == 0 {
        return false;
    }
    match form_type(e, space) {
        SPACE_WORLD => {
            e.mem.set_u32(world.addr(), space);
            e.mem.set_u32(cell.addr(), 0);
            true
        }
        SPACE_CELL => {
            e.mem.set_u32(cell.addr(), space);
            e.mem.set_u32(world.addr(), 0);
            true
        }
        _ => false,
    }
}

// Translated from 00527a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSWorldLocation::IsLocationLoaded` (Xbox PDB): false without a cell
/// (`GetCell`), otherwise `TES::IsCellLoaded(cell, 0)` (Xbox PDB) on the
/// `TES` singleton.
pub fn bgs_world_location_is_location_loaded(e: &mut Engine, this: Ptr<BGSWorldLocation>) -> bool {
    let cell = bgs_world_location_get_cell(e, this);
    if cell == 0 {
        return false;
    }
    let tes = e.global::<u32>(TES_SINGLETON);
    e.call(IS_CELL_LOADED, &args![tes, cell, 0u32]).bool()
}

/// `other - this` (`00439ef0` on `other`) into a stack point, then `length`
/// on it; the far distance when the spaces differ.
fn measure(
    e: &mut Engine,
    this: Ptr<BGSWorldLocation>,
    other: Ptr<BGSWorldLocation>,
    length: u32,
) -> f32 {
    if e.get(this, BGSWorldLocation::pSpace) != e.get(other, BGSWorldLocation::pSpace) {
        return far_distance(e);
    }
    e.with_stack(0xc, |e, difference| {
        let point = e
            .call(POINT_SUBTRACT, &args![other, difference, this])
            .u32();
        e.call(length, &args![point]).f32()
    })
}

// Translated from 00527a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSWorldLocation::GetDistanceSquared` (Xbox PDB): the squared length
/// (`004a7290`) of the difference of the points (`00439ef0` on `other`
/// with a stack result and this location), or the `float` at `01016970`
/// when the spaces differ.
pub fn bgs_world_location_get_distance_squared(
    e: &mut Engine,
    this: Ptr<BGSWorldLocation>,
    other: Ptr<BGSWorldLocation>,
) -> f32 {
    measure(e, this, other, POINT_LENGTH_SQUARED)
}

// Translated from 00527a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSWorldLocation::GetDistance` (Xbox PDB): as `GetDistanceSquared`
/// with the length (`00457990`).
pub fn bgs_world_location_get_distance(
    e: &mut Engine,
    this: Ptr<BGSWorldLocation>,
    other: Ptr<BGSWorldLocation>,
) -> f32 {
    measure(e, this, other, POINT_LENGTH)
}

// Translated from 00527ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The squared length of the difference of the points scaled per axis by
/// the three floats at `scale` (each component multiplied in x87 and
/// stored back as a `float`), or the `float` at `01016970` when the spaces
/// differ.
pub fn fn_00527ac0(
    e: &mut Engine,
    this: Ptr<BGSWorldLocation>,
    other: Ptr<BGSWorldLocation>,
    scale: Ptr,
) -> f32 {
    if e.get(this, BGSWorldLocation::pSpace) != e.get(other, BGSWorldLocation::pSpace) {
        return far_distance(e);
    }
    e.with_stack(0xc, |e, difference| {
        e.call(POINT_SUBTRACT, &args![other, difference, this]);
        for axis in 0..3 {
            let at = difference.addr() + 4 * axis;
            let product = x87_mul_store(e.mem.f32(at), e.mem.f32(scale.addr() + 4 * axis));
            e.mem.set_u32(at, product.to_bits());
        }
        e.call(POINT_LENGTH_SQUARED, &args![difference]).f32()
    })
}

// Translated from 00527b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `space` is acceptable as a location's space (cdecl): true for
/// null and for type 0x41; for type 0x39 bit 0 of the cell's flag byte at
/// `+0x24` (`00425fd0`); false otherwise.
pub fn fn_00527b20(e: &mut Engine, space: u32) -> bool {
    if space == 0 {
        return true;
    }
    match form_type(e, space) {
        SPACE_CELL => e.call(CELL_FLAG_0, &args![space]).bool(),
        SPACE_WORLD => true,
        _ => false,
    }
}

// Translated from 00527b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSWorldLocation::SaveGame` (Xbox PDB): writes the point's 12 bytes
/// (`00865e50` on the buffer with the location, 0xC and 0) and the space's
/// form id (`BGSSaveGameBuffer::SaveFormID_ov2`, Xbox PDB, `00865df0`, with
/// the space and 0).
pub fn bgs_world_location_save_game(e: &mut Engine, this: Ptr<BGSWorldLocation>, buffer: Ptr) {
    e.call(0x0086_5e50, &args![buffer, this, 0xcu32, 0u32]);
    let space = e.get(this, BGSWorldLocation::pSpace);
    e.call(0x0086_5df0, &args![buffer, space, 0u32]);
}

// Translated from 00527ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSWorldLocation::LoadGame` (Xbox PDB): reads the point's 12 bytes
/// (`00864980`), then a form id (`BGSLoadGameBuffer::LoadFormID`, Xbox PDB,
/// `008648a0`) looked up as the space (`004839c0`); a space `00527b20`
/// refuses is cleared.
pub fn bgs_world_location_load_game(e: &mut Engine, this: Ptr<BGSWorldLocation>, buffer: Ptr) {
    e.call(0x0086_4980, &args![buffer, this, 0xcu32]);
    let id = e.call(0x0086_48a0, &args![buffer]).u32();
    let space = e.call(LOOKUP_FORM, &args![id]).u32();
    e.set(this, BGSWorldLocation::pSpace, space);
    let space = e.get(this, BGSWorldLocation::pSpace);
    if !fn_00527b20(e, space) {
        e.set(this, BGSWorldLocation::pSpace, 0);
    }
}

// Translated from 0097ede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A default `BGSWorldLocation` (the engine map's name
/// `BGSWorldLocation::CheckSpace` does not fit the body): copies the three
/// words of the point at `011f426c` into `Point`, clears `pSpace`, returns
/// `this`.
pub fn fn_0097ede0(e: &mut Engine, this: Ptr<BGSWorldLocation>) -> Ptr<BGSWorldLocation> {
    for word in 0..3 {
        let value = e.global::<u32>(DEFAULT_POINT + 4 * word);
        e.mem.set_u32(this.addr() + 4 * word, value);
    }
    e.set(this, BGSWorldLocation::pSpace, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0052_7900,
            bgs_world_location_get_cell(Ptr<BGSWorldLocation>) -> u32
        ),
        entry!(
            0x0052_7990,
            bgs_world_location_get_cell_or_world(Ptr<BGSWorldLocation>, Ptr, Ptr) -> bool
        ),
        entry!(
            0x0052_7a00,
            bgs_world_location_is_location_loaded(Ptr<BGSWorldLocation>) -> bool
        ),
        entry!(
            0x0052_7a40,
            bgs_world_location_get_distance_squared(
                Ptr<BGSWorldLocation>,
                Ptr<BGSWorldLocation>,
            ) -> f32
        ),
        entry!(
            0x0052_7a80,
            bgs_world_location_get_distance(Ptr<BGSWorldLocation>, Ptr<BGSWorldLocation>) -> f32
        ),
        entry!(
            0x0052_7ac0,
            fn_00527ac0(Ptr<BGSWorldLocation>, Ptr<BGSWorldLocation>, Ptr) -> f32
        ),
        entry!(0x0052_7b20, fn_00527b20(u32) -> bool),
        entry!(
            0x0052_7b70,
            bgs_world_location_save_game(Ptr<BGSWorldLocation>, Ptr)
        ),
        entry!(
            0x0052_7ba0,
            bgs_world_location_load_game(Ptr<BGSWorldLocation>, Ptr)
        ),
        entry!(
            0x0097_ede0,
            fn_0097ede0(Ptr<BGSWorldLocation>) -> Ptr<BGSWorldLocation>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// A form of `kind` (type byte at `+4`) with room for a cell's flags.
    fn form(e: &mut Engine, kind: u8) -> u32 {
        let f = e.mem.alloc(0x30);
        e.mem.set_u8(f + 4, kind);
        f
    }

    fn location(e: &mut Engine, x: f32, y: f32, z: f32, space: u32) -> u32 {
        let l = e.mem.alloc(0x10);
        e.mem.set_f32(l, x);
        e.mem.set_f32(l + 4, y);
        e.mem.set_f32(l + 8, z);
        e.mem.set_u32(l + 0xc, space);
        l
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(FORM_TYPE, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register(FLOAT_TO_INT, |_, a| ret(f32::from_bits(a[0]) as i32 as u32));
        e
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn get_cell_by_space_type() {
        let mut e = engine();
        e.register(GET_CELL_FROM_CELL_COORD, |_, _| ret(0x0100_0000));
        let cell = form(&mut e, 0x39);
        let world = form(&mut e, 0x41);
        let other = form(&mut e, 0x3a);
        let none = location(&mut e, 0.0, 0.0, 0.0, 0);
        assert_eq!(e.call(0x0052_7900, &[none]).u32(), 0);
        let in_cell = location(&mut e, 5.0, 6.0, 0.0, cell);
        assert_eq!(e.call(0x0052_7900, &[in_cell]).u32(), cell);
        let in_world = location(&mut e, 8192.5, -1.0, 0.0, world);
        e.call_log = Some(vec![]);
        // x 8192 >> 12 = 2, y -1 >> 12 = -1 (arithmetic).
        assert_eq!(e.call(0x0052_7900, &[in_world]).u32(), 0x0100_0000);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FLOAT_TO_INT),
            vec![vec![8192.5f32.to_bits()], vec![(-1.0f32).to_bits()]]
        );
        assert_eq!(
            calls_to(&log, GET_CELL_FROM_CELL_COORD),
            vec![vec![world, 2, u32::MAX]]
        );
        let elsewhere = location(&mut e, 0.0, 0.0, 0.0, other);
        assert_eq!(e.call(0x0052_7900, &[elsewhere]).u32(), 0);
    }

    #[test]
    fn get_cell_passes_a_signalling_nan_coordinate_quieted() {
        let mut e = engine();
        e.register(GET_CELL_FROM_CELL_COORD, |_, _| ret(0));
        let world = form(&mut e, 0x41);
        let l = location(&mut e, 0.0, 0.0, 0.0, world);
        e.mem.set_u32(l, 0x7f80_0001);
        e.call_log = Some(vec![]);
        e.call(0x0052_7900, &[l]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FLOAT_TO_INT)[0], vec![0x7fc0_0001]);
    }

    #[test]
    fn get_cell_or_world_fills_the_matching_slot() {
        let mut e = engine();
        let out = e.mem.alloc(8);
        let (cell_out, world_out) = (out, out + 4);
        let cell = form(&mut e, 0x39);
        let world = form(&mut e, 0x41);
        let other = form(&mut e, 0x20);
        let l = location(&mut e, 0.0, 0.0, 0.0, world);
        e.mem.set_u32(cell_out, 9);
        assert!(e.call(0x0052_7990, &[l, cell_out, world_out]).bool());
        assert_eq!((e.mem.u32(cell_out), e.mem.u32(world_out)), (0, world));
        e.mem.set_u32(l + 0xc, cell);
        assert!(e.call(0x0052_7990, &[l, cell_out, world_out]).bool());
        assert_eq!((e.mem.u32(cell_out), e.mem.u32(world_out)), (cell, 0));
        e.mem.set_u32(cell_out, 7);
        e.mem.set_u32(l + 0xc, other);
        assert!(!e.call(0x0052_7990, &[l, cell_out, world_out]).bool());
        e.mem.set_u32(l + 0xc, 0);
        assert!(!e.call(0x0052_7990, &[l, cell_out, world_out]).bool());
        assert_eq!(e.mem.u32(cell_out), 7);
    }

    #[test]
    fn is_location_loaded_asks_tes_for_the_cell() {
        let mut e = engine();
        e.map(TES_SINGLETON, 4);
        e.set_global(TES_SINGLETON, 0x0700_0000u32);
        e.register(IS_CELL_LOADED, |_, a| {
            ret(if a[2] == 0 { 0xffff_ff01 } else { 0 })
        });
        let cell = form(&mut e, 0x39);
        let l = location(&mut e, 0.0, 0.0, 0.0, cell);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0052_7a00, &[l]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, IS_CELL_LOADED),
            vec![vec![0x0700_0000, cell, 0]]
        );
        e.register(IS_CELL_LOADED, |_, _| ret(0x100));
        assert!(!e.call(0x0052_7a00, &[l]).bool());
        let nowhere = location(&mut e, 0.0, 0.0, 0.0, 0);
        assert!(!e.call(0x0052_7a00, &[nowhere]).bool());
    }

    /// Doubles for the point arithmetic: the subtraction writes
    /// `other - this` into the result and returns it.
    fn point_doubles(e: &mut Engine) {
        e.register(POINT_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            ret(a[1])
        });
        e.register(POINT_LENGTH_SQUARED, |e, a| {
            let v: f64 = (0..3)
                .map(|i| (e.mem.f32(a[0] + 4 * i) as f64).powi(2))
                .sum();
            st0(v)
        });
        e.register(POINT_LENGTH, |e, a| {
            let v: f64 = (0..3)
                .map(|i| (e.mem.f32(a[0] + 4 * i) as f64).powi(2))
                .sum();
            st0(v.sqrt())
        });
        e.map(FAR_DISTANCE, 4);
        e.set_global(FAR_DISTANCE, f32::MAX);
    }

    #[test]
    fn distances_in_one_space_and_across_spaces() {
        let mut e = engine();
        point_doubles(&mut e);
        let a = location(&mut e, 1.0, 2.0, 3.0, 0x0123);
        let b = location(&mut e, 4.0, 6.0, 3.0, 0x0123);
        let c = location(&mut e, 4.0, 6.0, 3.0, 0x0456);
        assert_eq!(e.call(0x0052_7a40, &[a, b]).f32(), 25.0);
        assert_eq!(e.call(0x0052_7a80, &[a, b]).f32(), 5.0);
        assert_eq!(e.call(0x0052_7a40, &[a, c]).f32(), f32::MAX);
        assert_eq!(e.call(0x0052_7a80, &[a, c]).f32(), f32::MAX);
        e.call_log = Some(vec![]);
        e.call(0x0052_7a40, &[a, b]);
        let log = e.call_log.take().unwrap();
        let sub = calls_to(&log, POINT_SUBTRACT);
        assert_eq!((sub[0][0], sub[0][2]), (b, a));
    }

    #[test]
    fn scaled_distance_multiplies_each_axis() {
        let mut e = engine();
        point_doubles(&mut e);
        let a = location(&mut e, 1.0, 2.0, 3.0, 0x0123);
        let b = location(&mut e, 4.0, 6.0, 5.0, 0x0123);
        let c = location(&mut e, 4.0, 6.0, 5.0, 0);
        let scale = e.mem.alloc(0xc);
        for (i, v) in [2.0f32, 0.5, 0.0].iter().enumerate() {
            e.mem.set_f32(scale + 4 * i as u32, *v);
        }
        // (3*2)^2 + (4*0.5)^2 + 0 = 40.
        assert_eq!(e.call(0x0052_7ac0, &[a, b, scale]).f32(), 40.0);
        assert_eq!(e.call(0x0052_7ac0, &[a, c, scale]).f32(), f32::MAX);
    }

    #[test]
    fn x87_multiply_nan_rules() {
        let snan = f32::from_bits(0x7f80_0001);
        let qnan = f32::from_bits(0x7fc0_0002);
        assert_eq!(x87_mul_store(1.5, 2.0), 3.0);
        assert_eq!(x87_mul_store(snan, 2.0).to_bits(), 0x7fc0_0001);
        assert_eq!(x87_mul_store(2.0, snan).to_bits(), 0x7fc0_0001);
        assert_eq!(x87_mul_store(qnan, snan).to_bits(), 0x7fc0_0002);
        assert_eq!(x87_mul_store(snan, qnan).to_bits(), 0x7fc0_0002);
        assert_eq!(
            x87_mul_store(f32::from_bits(0xffc0_0002), qnan).to_bits(),
            0x7fc0_0002
        );
        assert_eq!(x87_mul_store(0.0, f32::INFINITY).to_bits(), 0xffc0_0000);
    }

    #[test]
    fn space_check_by_type() {
        let mut e = engine();
        e.register(CELL_FLAG_0, |e, a| ret((e.mem.u8(a[0] + 0x24) & 1) as u32));
        assert!(e.call(0x0052_7b20, &[0]).bool());
        let world = form(&mut e, 0x41);
        assert!(e.call(0x0052_7b20, &[world]).bool());
        let cell = form(&mut e, 0x39);
        assert!(!e.call(0x0052_7b20, &[cell]).bool());
        e.mem.set_u8(cell + 0x24, 1);
        assert!(e.call(0x0052_7b20, &[cell]).bool());
        let other = form(&mut e, 0x2a);
        assert!(!e.call(0x0052_7b20, &[other]).bool());
    }

    #[test]
    fn save_game_writes_point_and_space() {
        let mut e = engine();
        e.register(0x0086_5e50, |_, _| Ret::default());
        e.register(0x0086_5df0, |_, _| Ret::default());
        let l = location(&mut e, 0.0, 0.0, 0.0, 0x0555);
        let buffer = 0x0500_0000;
        e.call_log = Some(vec![]);
        e.call(0x0052_7b70, &[l, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0086_5e50), vec![vec![buffer, l, 0xc, 0]]);
        assert_eq!(calls_to(&log, 0x0086_5df0), vec![vec![buffer, 0x0555, 0]]);
    }

    #[test]
    fn load_game_keeps_only_an_accepted_space() {
        let mut e = engine();
        e.register(0x0086_4980, |_, _| Ret::default());
        e.register(CELL_FLAG_0, |e, a| ret((e.mem.u8(a[0] + 0x24) & 1) as u32));
        let world = form(&mut e, 0x41);
        let cell = form(&mut e, 0x39);
        let buffer = e.mem.alloc(4);
        e.mem.set_u32(buffer, world);
        e.register(0x0086_48a0, |e, a| ret(e.mem.u32(a[0])));
        e.register(LOOKUP_FORM, |_, a| ret(a[0]));
        let l = location(&mut e, 0.0, 0.0, 0.0, 0);
        e.call(0x0052_7ba0, &[l, buffer]);
        assert_eq!(e.mem.u32(l + 0xc), world);
        e.mem.set_u32(buffer, cell);
        e.call(0x0052_7ba0, &[l, buffer]);
        assert_eq!(e.mem.u32(l + 0xc), 0);
    }

    #[test]
    fn default_location_copies_the_point_and_clears_the_space() {
        let mut e = engine();
        e.map(DEFAULT_POINT, 12);
        for (i, w) in [1u32, 2, 3].iter().enumerate() {
            e.set_global(DEFAULT_POINT + 4 * i as u32, *w);
        }
        let l = location(&mut e, 9.0, 9.0, 9.0, 0x77);
        assert_eq!(e.call(0x0097_ede0, &[l]).u32(), l);
        assert_eq!(
            (0..4).map(|i| e.mem.u32(l + 4 * i)).collect::<Vec<_>>(),
            vec![1, 2, 3, 0]
        );
    }
}
