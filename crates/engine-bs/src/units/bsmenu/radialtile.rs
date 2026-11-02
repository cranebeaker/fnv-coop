//! `BSMenu/radialtile.cpp` (Xbox PDB source unit), subsystem `BSMenu`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TileImage::TileImage` (engine map, `0070b5d0`; thiscall, no stack words).
const TILE_IMAGE_CONSTRUCT: u32 = 0x0070_b5d0;
/// `RadialTile`'s vtable (RTTI `.?AVRadialTile@@`).
const RADIAL_TILE_VTABLE: u32 = 0x0109_5750;

// Translated from 00a21660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RadialTile::RadialTile` (engine map; thiscall, no stack words): constructs the
/// `TileImage` base (`0070b5d0`), then stores `RadialTile`'s vtable at `+0`. Returns `this`.
pub fn radial_tile_radial_tile(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(TILE_IMAGE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), RADIAL_TILE_VTABLE);
    this
}

/// `TileImage::~TileImage` (engine map, `0070b6c0`; thiscall, no stack words).
const TILE_IMAGE_DESTRUCT: u32 = 0x0070_b6c0;
/// The memory manager singleton and its `Deallocate` (`00aa4060`, thiscall, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00a21690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RadialTile::~RadialTile` (engine map; thiscall, no stack words): stores `RadialTile`'s
/// vtable at `+0`, then destroys the `TileImage` base (`0070b6c0`), whose EAX it passes on.
pub fn radial_tile_destructor(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u32(this.addr(), RADIAL_TILE_VTABLE);
    e.call(TILE_IMAGE_DESTRUCT, &args![this]).u32()
}

// Translated from 00a219e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RadialTile::scalar deleting destructor` (engine map; thiscall, one word): destroys the
/// tile ([`radial_tile_destructor`]) and, with bit 0 of `flags`, gives its memory back to the
/// memory manager. Answers `this`.
pub fn radial_tile_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    radial_tile_destructor(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The tile's float value getter (`00a011b0`, `BSMenu/tile.cpp`, thiscall, one word: the
/// value id; result in ST0).
const TILE_GET_FLOAT_00A011B0: u32 = 0x00a0_11b0;
/// `Interface::GetRealScreenWidth` (engine map, `00706e40`; result in ST0).
const INTERFACE_GET_REAL_SCREEN_WIDTH: u32 = 0x0070_6e40;
/// `Interface::GetMenusRoot` (engine map, `00706cd0`).
const INTERFACE_GET_MENUS_ROOT: u32 = 0x0070_6cd0;
/// `Tile::GetValue` (engine map, `00a00f30`; thiscall, one word: the value id).
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
/// The CRT function the angle comes from (`00ec9010`, `cdecl`, one `double`; result in ST0).
const CRT_ANGLE_00EC9010: u32 = 0x00ec_9010;
/// The CRT function the magnitude comes from (`00ec6054`, `cdecl`, one `double`; result in
/// ST0).
const CRT_MAGNITUDE_00EC6054: u32 = 0x00ec_6054;
/// The `BSStringT` formatter (`00406f60`, `cdecl`: string, format, values).
const STRING_FORMAT_00406F60: u32 = 0x0040_6f60;
/// The message printer (`005b5e40`, `cdecl`, one word: the text).
const PRINT_MESSAGE_005B5E40: u32 = 0x005b_5e40;
/// `BSStringT<char>` set (`004037f0`, thiscall: text, length), here with 0, 0.
const STRING_SET: u32 = 0x0040_37f0;
/// The `double` 0.0 the comparisons use (`01012060`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The angle floats for a point straight on the horizontal: `01071e28` (x not above 0,
/// about 4.71238) and `01071e2c` (x above 0, about 1.57075).
const ANGLE_LEFT_FLOAT: u32 = 0x0107_1e28;
const ANGLE_RIGHT_FLOAT: u32 = 0x0107_1e2c;
/// The `double`s added to the angle: `01071e60` (y above 0, about 3.14159) and `010957a0`
/// (x and y below 0, about 6.28318).
const HALF_TURN_DOUBLE: u32 = 0x0107_1e60;
const FULL_TURN_DOUBLE: u32 = 0x0109_57a0;
/// The format of the debug line (`01095778`, "--- fx %f fy %f  --- Magnitude: %f ").
const MAGNITUDE_FORMAT: u32 = 0x0109_5778;
/// The value id of the menus root's value the scale divides (`0xfb1`).
const MENUS_ROOT_WIDTH_VALUE: u32 = 0xfb1;

/// A tile float value through `00a011b0`, stored as a `float`.
fn tile_float(e: &mut Engine, this: Ptr, id: u32) -> u32 {
    x87_store32(e.call(TILE_GET_FLOAT_00A011B0, &args![this, id]).st0)
}

// Translated from 00a216b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RadialTile::SpecialBoundsCheck` (engine map; thiscall, two floats): whether the point
/// (`x`, `y`) lies in the tile's ring sector. Reads the tile's float values 0x1004 to 0x1009
/// (centre x and y, smallest and largest angle, smallest and largest distance); scales the
/// point by the menus root's value 0xfb1 (`+8`) over the real screen width (1.0 without a
/// root or value, a zero width counts as 1.0), takes it relative to the centre, finds its
/// angle (`00ec9010` of `-x / y`, or 4.71238 / 1.57075 when `y` is 0, plus 3.14159 for
/// `y` above 0 and 6.28318 for both below 0) and, inside the angle range, its distance
/// (`00ec6054` of `x*x + y*y`), which it prints as a debug line before checking the
/// distance range. The string local's length clamps (`0 > 0xffff`, folded constants) can
/// never run and are left out.
// Translation incomplete: 00a216b0 - its FS:[0] exception frame (unwinding of the string local) is not translated
pub fn radial_tile_special_bounds_check(e: &mut Engine, this: Ptr, x: f32, y: f32) -> bool {
    let centre_x = tile_float(e, this, 0x1004);
    let centre_y = tile_float(e, this, 0x1005);
    let angle_min = tile_float(e, this, 0x1006);
    let angle_max = tile_float(e, this, 0x1007);
    let distance_min = tile_float(e, this, 0x1008);
    let distance_max = tile_float(e, this, 0x1009);
    let zero = e.global::<f64>(ZERO_DOUBLE);
    let mut scale = x87_store32(e.call(INTERFACE_GET_REAL_SCREEN_WIDTH, &args![]).st0);
    if x87_load32(scale) == zero {
        scale = 1.0f32.to_bits();
    }
    let root = e.call(INTERFACE_GET_MENUS_ROOT, &args![]).u32();
    if root != 0 {
        let value = e
            .call(TILE_GET_VALUE, &args![root, MENUS_ROOT_WIDTH_VALUE])
            .u32();
        if value != 0 {
            // Tile::Value float at +8.
            let width = x87_load32(e.mem.u32(value.wrapping_add(8)));
            scale = x87_store32(x87_div(width, x87_m32(scale)));
        } else {
            scale = 1.0f32.to_bits();
        }
    } else {
        scale = 1.0f32.to_bits();
    }
    let mut x = x87_store32(x87_mul(x87_load32(x.to_bits()), x87_m32(scale)));
    let mut y = x87_store32(x87_mul(x87_load32(y.to_bits()), x87_m32(scale)));
    x = x87_store32(x87_sub(x87_load32(x), x87_m32(centre_x)));
    y = x87_store32(x87_sub(x87_load32(y), x87_m32(centre_y)));
    let mut angle;
    if x87_load32(y) == zero {
        let fx = x87_load32(x);
        angle = if fx > zero {
            x87_quiet_bits(e.global::<u32>(ANGLE_RIGHT_FLOAT))
        } else {
            x87_quiet_bits(e.global::<u32>(ANGLE_LEFT_FLOAT))
        };
    } else {
        // FLD y; FLD x; FCHS; FDIVRP: -x / y.
        let ratio = x87_store32(x87_div(-x87_load32(x), x87_load32(y)));
        let r = e.call(CRT_ANGLE_00EC9010, &args![x87_load32(ratio)]).st0;
        angle = x87_quiet_bits(x87_store32(r));
    }
    if x87_load32(y) > zero {
        let turn = f64::from_bits(e.global::<u64>(HALF_TURN_DOUBLE));
        angle = x87_store32(x87_add(x87_load32(angle), turn));
    }
    if x87_load32(x) < zero && x87_load32(y) < zero {
        let turn = f64::from_bits(e.global::<u64>(FULL_TURN_DOUBLE));
        angle = x87_store32(x87_add(x87_load32(angle), turn));
    }
    if !(x87_load32(angle) >= x87_load32(angle_min)) {
        return false;
    }
    if !(x87_load32(angle_max) >= x87_load32(angle)) {
        return false;
    }
    let yy = x87_mul(x87_load32(y), x87_m32(y));
    let xx = x87_mul(x87_load32(x), x87_m32(x));
    let squared = x87_store32(x87_add(yy, xx));
    let r = e
        .call(CRT_MAGNITUDE_00EC6054, &args![x87_load32(squared)])
        .st0;
    let magnitude = x87_quiet_bits(x87_store32(r));
    e.with_stack(8, |e, text| {
        // BSStringT: text pointer, length and capacity words cleared.
        e.mem.set_u32(text.addr(), 0);
        e.mem.set_u16(text.addr().wrapping_add(4), 0);
        e.mem.set_u16(text.addr().wrapping_add(6), 0);
        e.call(
            STRING_FORMAT_00406F60,
            &args![
                text,
                MAGNITUDE_FORMAT,
                x87_load32(x),
                x87_load32(y),
                x87_load32(magnitude)
            ],
        );
        let message = e.mem.u32(text.addr());
        e.call(PRINT_MESSAGE_005B5E40, &args![message]);
        let inside = x87_load32(magnitude) >= x87_load32(distance_min)
            && x87_load32(distance_max) >= x87_load32(magnitude);
        e.call(STRING_SET, &args![text, 0u32, 0u32]);
        inside
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2_1660, radial_tile_radial_tile(Ptr) -> Ptr),
        entry!(0x00a2_1690, radial_tile_destructor(Ptr) -> u32),
        entry!(0x00a2_19e0, radial_tile_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a2_16b0,
            radial_tile_special_bounds_check(Ptr, f32, f32) -> bool
        ),
    ]
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn radial_tile_constructs_the_base_then_sets_its_vtable() {
        let mut e = Engine::new();
        e.register(TILE_IMAGE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111_1111);
            Ret::default()
        });
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        let back = e.call(0x00a2_1660, &args![this]).u32();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this), RADIAL_TILE_VTABLE);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == TILE_IMAGE_CONSTRUCT)
            .collect();
        assert_eq!(base.len(), 1);
        assert_eq!(base[0].1, vec![this]);
    }
}

#[cfg(test)]
mod n083_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn destructor_sets_the_vtable_then_destroys_the_base() {
        let mut e = Engine::new();
        e.register(TILE_IMAGE_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x00a2_1690, &args![this]);
        assert_eq!(e.mem.u32(this), RADIAL_TILE_VTABLE);
        assert_eq!(calls_to(&e, TILE_IMAGE_DESTRUCT), [vec![this]]);
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(TILE_IMAGE_DESTRUCT, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_19e0, &args![this, 0u32]).u32(), this);
        assert!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
        assert_eq!(calls_to(&e, TILE_IMAGE_DESTRUCT).len(), 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_19e0, &args![this, 3u32]).u32(), this);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            [vec![MEMORY_MANAGER, this]]
        );
    }

    fn st0(v: f64) -> Ret {
        Ret {
            st0: v,
            ..Ret::default()
        }
    }

    /// A tile centred on (10, 20), angles 0 to 7, distances 1 to 100, a screen width of
    /// 1280 and a root value of 640 (scale 0.5).
    fn setup(e: &mut Engine) -> u32 {
        e.map(ZERO_DOUBLE, 8);
        e.map(0x0107_1e00, 0x100);
        e.map(0x0109_5700, 0x100);
        e.set_global::<u32>(ANGLE_LEFT_FLOAT, 4.5f32.to_bits());
        e.set_global::<u32>(ANGLE_RIGHT_FLOAT, 1.5f32.to_bits());
        e.set_global::<u64>(HALF_TURN_DOUBLE, 3.25f64.to_bits());
        e.set_global::<u64>(FULL_TURN_DOUBLE, 6.5f64.to_bits());
        e.register(TILE_GET_FLOAT_00A011B0, |_, w| {
            st0(match w[1] {
                0x1004 => 10.0,
                0x1005 => 20.0,
                0x1006 => 0.0,
                0x1007 => 7.0,
                0x1008 => 1.0,
                _ => 100.0,
            })
        });
        e.register(INTERFACE_GET_REAL_SCREEN_WIDTH, |_, _| st0(1280.0));
        e.register(INTERFACE_GET_MENUS_ROOT, |_, _| Ret {
            eax: 0x0300_0000,
            ..Ret::default()
        });
        let value = e.mem.alloc(0x10);
        e.mem.set_f32(value + 8, 640.0);
        e.register_double(TILE_GET_VALUE, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
        e.register(CRT_ANGLE_00EC9010, |_, _| st0(2.0));
        e.register(CRT_MAGNITUDE_00EC6054, |_, w| {
            st0(f64::from_bits(u64::from(w[0]) | u64::from(w[1]) << 32).sqrt())
        });
        e.register(STRING_FORMAT_00406F60, |_, _| Ret::default());
        e.register(PRINT_MESSAGE_005B5E40, |_, _| Ret::default());
        e.register(STRING_SET, |_, _| Ret::default());
        e.mem.alloc(0x40)
    }

    #[test]
    fn point_on_the_horizontal_inside_the_ring_is_in() {
        let mut e = Engine::new();
        let this = setup(&mut e);
        e.call_log = Some(vec![]);
        // (60, 40) scaled by 0.5 is (30, 20): 20 right of the centre.
        let r = e.call(0x00a2_16b0, &args![this, 60.0f32, 40.0f32]);
        assert!(r.bool());
        assert!(calls_to(&e, CRT_ANGLE_00EC9010).is_empty());
        let format = calls_to(&e, STRING_FORMAT_00406F60);
        assert_eq!(format.len(), 1);
        assert_eq!(format[0][1], MAGNITUDE_FORMAT);
        assert_eq!(format[0][2..4], args![20.0f64]);
        assert_eq!(format[0][4..6], args![0.0f64]);
        assert_eq!(format[0][6..8], args![20.0f64]);
        assert_eq!(calls_to(&e, PRINT_MESSAGE_005B5E40).len(), 1);
        assert_eq!(calls_to(&e, STRING_SET)[0][1..], [0, 0]);
        // Too far: printed, then out.
        e.call_log = Some(vec![]);
        let r = e.call(0x00a2_16b0, &args![this, 500.0f32, 40.0f32]);
        assert!(!r.bool());
        assert_eq!(calls_to(&e, PRINT_MESSAGE_005B5E40).len(), 1);
        assert_eq!(calls_to(&e, STRING_SET).len(), 1);
    }

    #[test]
    fn angle_outside_the_range_is_out_without_a_line() {
        let mut e = Engine::new();
        let this = setup(&mut e);
        e.call_log = Some(vec![]);
        // (14, 32) scaled is (7, 16): (-3, -4) from the centre; 2.0 + 6.5 > 7.
        let r = e.call(0x00a2_16b0, &args![this, 14.0f32, 32.0f32]);
        assert!(!r.bool());
        assert_eq!(calls_to(&e, CRT_ANGLE_00EC9010), [args![-0.75f64]]);
        assert!(calls_to(&e, STRING_FORMAT_00406F60).is_empty());
        // (-3, 4): 2.0 + 3.25 is inside; distance 5.
        e.call_log = Some(vec![]);
        let r = e.call(0x00a2_16b0, &args![this, 14.0f32, 48.0f32]);
        assert!(r.bool());
        assert_eq!(calls_to(&e, CRT_ANGLE_00EC9010), [args![0.75f64]]);
        // Left on the horizontal: 4.5 is inside.
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a2_16b0, &args![this, 0.0f32, 40.0f32]).bool());
    }

    #[test]
    fn scale_falls_back_to_one() {
        let mut e = Engine::new();
        let this = setup(&mut e);
        e.register(TILE_GET_VALUE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        // Unscaled (30, 20) is 20 right of the centre.
        assert!(e.call(0x00a2_16b0, &args![this, 30.0f32, 20.0f32]).bool());
        assert_eq!(
            calls_to(&e, STRING_FORMAT_00406F60)[0][2..4],
            args![20.0f64]
        );
        e.register(INTERFACE_GET_MENUS_ROOT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a2_16b0, &args![this, 30.0f32, 20.0f32]).bool());
        assert!(calls_to(&e, TILE_GET_VALUE).is_empty());
        // A zero width counts as 1.0: 640 / 1 scales (0.0625, 0.03125) to (40, 20).
        e.register(INTERFACE_GET_MENUS_ROOT, |_, _| Ret {
            eax: 0x0300_0000,
            ..Ret::default()
        });
        let value = e.mem.alloc(0x10);
        e.mem.set_f32(value + 8, 640.0);
        e.register_double(TILE_GET_VALUE, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
        e.register(INTERFACE_GET_REAL_SCREEN_WIDTH, |_, _| st0(0.0));
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a2_16b0, &args![this, 0.0625f32, 0.03125f32])
            .bool());
        assert_eq!(
            calls_to(&e, STRING_FORMAT_00406F60)[0][2..4],
            args![30.0f64]
        );
    }
}
