//! `NiCollision/run_00e9c8c0` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The bounding volume dispatch: four tables of function pointers that the NiCollision start-up
//! (`fn_00e98ff0` in `run_00e99300`) fills, indexed by the volumes' virtual `Type` (slot +0xc),
//! and the three conversion functions it stores at `0126faa4` to `0126faac`. The Xbox PDB lists
//! `NiBoundingVolume`'s statics `ms_pfnTestFuncTable`, `ms_pfnFindFuncTable`,
//! `ms_pfnTriTestFuncTable`, `ms_pfnTriFindFuncTable`, `ms_pfnConvertToSphereBV_Fast`,
//! `ms_pfnConvertToSphereBV_Safe` and `ms_pfnConvertToBoxBV`; which global is which is read
//! from their use here, not confirmed by a symbol.

#[allow(unused_imports)]
use crate::prelude::*;

/// The pair table of the intersection tests (6 x 6 function pointers, row = first volume's
/// type): the global holding its address.
const TEST_TABLE: u32 = 0x0126_fa94;
/// The pair table of the intersection finds (6 x 6).
const FIND_TABLE: u32 = 0x0126_fa98;
/// The table of the volume-triangle tests (6 entries, by the volume's type).
const TRI_TEST_TABLE: u32 = 0x0126_fa9c;
/// The table of the volume-triangle finds (6 entries).
const TRI_FIND_TABLE: u32 = 0x0126_faa0;
/// The static sphere volume `fn_00e9c7d0` fills (its center at +4, its radius at +0x10).
const STATIC_SPHERE: u32 = 0x0126_fa68;
/// The memory manager instance and its `Allocate` (`00aa3e40`, thiscall: size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The sphere volume constructor from a radius and a center (`00eab8a0`, thiscall).
const SPHERE_BV_CONSTRUCTOR: u32 = 0x00ea_b8a0;
/// The three words at `011f426c` (zero in the image) handed to the constructors as a point.
const ZERO_POINT_011F426C: u32 = 0x011f_426c;
/// The box volume constructor (`00ea0820`, thiscall, five pointers; unit `niboxbv.cpp`).
const BOX_BV_CONSTRUCTOR: u32 = 0x00ea_0820;
/// Unnamed (`00eadf30`, thiscall on a 0x3c-byte box: the point count and the points): fits a
/// box around points.
const BOX_FROM_POINTS: u32 = 0x00ea_df30;

/// The virtual `Type` (slot +0xc) of a bounding volume.
fn volume_type(e: &mut Engine, volume: Ptr) -> u32 {
    e.vcall(volume.addr(), 0xc, &args![]).u32()
}

/// The entry `row * 6 + column` of the 6 x 6 table whose address is in `table`.
fn pair_entry(e: &Engine, table: u32, row: u32, column: u32) -> u32 {
    let base = e.mem.u32(table);
    let index = row.wrapping_mul(6).wrapping_add(column);
    e.mem.u32(base.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00e9c7d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; stored at `0126faa4`, `cdecl`): the fast sphere conversion.
/// Writes `radius` (as `FLD`/`FSTP` leave it) and the three words of `center` into the static
/// sphere volume at `0126fa68` and answers it.
pub fn fn_00e9c7d0(e: &mut Engine, radius: f32, center: Ptr) -> Ptr {
    e.mem
        .set_u32(STATIC_SPHERE + 0x10, x87_quiet_bits(radius.to_bits()));
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(center.addr().wrapping_add(offset));
        e.mem.set_u32(STATIC_SPHERE + 4 + offset, word);
    }
    Ptr::new(STATIC_SPHERE)
}

// Translated from 00e9c800 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; stored at `0126faa8`, `cdecl`): the safe sphere conversion.
/// Allocates 0x14 bytes from the memory manager; a block is made a sphere volume of radius 1.0
/// at the point `011f426c` (`00eab8a0`). Then `radius` (as `FLD`/`FSTP` leave it) goes to +0x10
/// and the three words of `center` to +4, also when the allocation failed (the exe writes
/// through null then). Answers the volume.
pub fn fn_00e9c800(e: &mut Engine, radius: f32, center: Ptr) -> Ptr {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x14u32])
        .u32();
    let volume = if block != 0 {
        e.call(
            SPHERE_BV_CONSTRUCTOR,
            &args![block, 1.0f32, ZERO_POINT_011F426C],
        )
        .u32()
    } else {
        0
    };
    e.mem
        .set_u32(volume.wrapping_add(0x10), x87_quiet_bits(radius.to_bits()));
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(center.addr().wrapping_add(offset));
        e.mem.set_u32(volume.wrapping_add(4 + offset), word);
    }
    Ptr::new(volume)
}

// Translated from 00e9c850 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; stored at `0126faac`, `cdecl`): the box conversion of a point
/// set. Answers null when the 16-bit point count or `points` is 0. Otherwise fits a 0x3c-byte
/// box on its stack around the points (`00eadf30`, given the whole count word), allocates 0x40
/// bytes from the memory manager, makes a block a box volume (`00ea0820` with `011a9490`,
/// `011f426c`, `011a946c`, `011a9478` and `011a9484`) and copies the fitted box over its words
/// +4 to +0x3f (also when the allocation failed: the exe writes through null then). Answers the
/// volume.
pub fn fn_00e9c850(e: &mut Engine, count: u32, points: Ptr) -> Ptr {
    if count as u16 == 0 || points.is_null() {
        return Ptr::NULL;
    }
    e.with_stack(0x3c, |e, fitted| {
        e.call(BOX_FROM_POINTS, &args![fitted, count, points]);
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x40u32])
            .u32();
        let volume = if block != 0 {
            e.call(
                BOX_BV_CONSTRUCTOR,
                &args![
                    block,
                    0x011a_9490u32,
                    ZERO_POINT_011F426C,
                    0x011a_946cu32,
                    0x011a_9478u32,
                    0x011a_9484u32
                ],
            )
            .u32()
        } else {
            0
        };
        for offset in (0..0x3cu32).step_by(4) {
            let word = e.mem.u32(fitted.addr() + offset);
            e.mem.set_u32(volume.wrapping_add(4 + offset), word);
        }
        Ptr::new(volume)
    })
}

// Translated from 00e9c8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, five words): the intersection test of two moving
/// volumes during `time`. The function at `first type * 6 + second type` of the test table
/// (`0126fa94`) is called with `(time, first, first_velocity, second, second_velocity)`; when
/// that entry is null, the one at `second type * 6 + first type` is called with the volumes
/// swapped (the types are then read again, the second volume's first; that entry is not
/// checked). `time` passes through `FLD`/`FSTP`. Answers the called function's result.
pub fn fn_00e9c8c0(
    e: &mut Engine,
    time: f32,
    first: Ptr,
    first_velocity: Ptr,
    second: Ptr,
    second_velocity: Ptr,
) -> bool {
    let time = x87_quiet_bits(time.to_bits());
    let first_type = volume_type(e, first);
    let second_type = volume_type(e, second);
    let test = pair_entry(e, TEST_TABLE, first_type, second_type);
    if test != 0 {
        return e
            .call(
                test,
                &args![time, first, first_velocity, second, second_velocity],
            )
            .bool();
    }
    let second_type = volume_type(e, second);
    let first_type = volume_type(e, first);
    let test = pair_entry(e, TEST_TABLE, second_type, first_type);
    e.call(
        test,
        &args![time, second, second_velocity, first, first_velocity],
    )
    .bool()
}

// Translated from 00e9c950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, ten words): finds when and where two moving volumes
/// meet. The function at `first type * 6 + second type` of the find table (`0126fa98`) is
/// called with all ten words (`time`, the first volume and its velocity, the second and its
/// velocity, the out time, the out point, the normals flag and the two out normals); when that
/// entry is null, the one at `second type * 6 + first type` is called with the volumes and
/// their velocities swapped and the two out normals swapped (types read again, the second
/// volume's first; the entry is not checked). `time` passes through `FLD`/`FSTP`. Answers the
/// called function's result.
pub fn fn_00e9c950(
    e: &mut Engine,
    time: f32,
    first: Ptr,
    first_velocity: Ptr,
    second: Ptr,
    second_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    calculate_normals: u32,
    first_normal: Ptr,
    second_normal: Ptr,
) -> bool {
    let time = x87_quiet_bits(time.to_bits());
    let first_type = volume_type(e, first);
    let second_type = volume_type(e, second);
    let find = pair_entry(e, FIND_TABLE, first_type, second_type);
    if find != 0 {
        return e
            .call(
                find,
                &args![
                    time,
                    first,
                    first_velocity,
                    second,
                    second_velocity,
                    out_time,
                    out_point,
                    calculate_normals,
                    first_normal,
                    second_normal
                ],
            )
            .bool();
    }
    let second_type = volume_type(e, second);
    let first_type = volume_type(e, first);
    let find = pair_entry(e, FIND_TABLE, second_type, first_type);
    e.call(
        find,
        &args![
            time,
            second,
            second_velocity,
            first,
            first_velocity,
            out_time,
            out_point,
            calculate_normals,
            second_normal,
            first_normal
        ],
    )
    .bool()
}

// Translated from 00e9ca10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, seven words): the test of a moving volume against a
/// moving triangle. Calls the entry of the volume's type in the triangle test table
/// (`0126fa9c`) with the seven words (`time` through `FLD`/`FSTP`) and answers its result.
pub fn fn_00e9ca10(
    e: &mut Engine,
    time: f32,
    volume: Ptr,
    velocity: Ptr,
    vertex0: Ptr,
    vertex1: Ptr,
    vertex2: Ptr,
    triangle_velocity: Ptr,
) -> bool {
    let time = x87_quiet_bits(time.to_bits());
    let kind = volume_type(e, volume);
    let table = e.mem.u32(TRI_TEST_TABLE);
    let test = e.mem.u32(table.wrapping_add(kind.wrapping_mul(4)));
    e.call(
        test,
        &args![
            time,
            volume,
            velocity,
            vertex0,
            vertex1,
            vertex2,
            triangle_velocity
        ],
    )
    .bool()
}

// Translated from 00e9ca50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, twelve words): the find of a moving volume against a
/// moving triangle. Calls the entry of the volume's type in the triangle find table
/// (`0126faa0`) with the twelve words (`time` through `FLD`/`FSTP`; then the volume, its
/// velocity, the three vertices, the triangle's velocity, the out time, the out point, the
/// normals flag and the two out normals) and answers its result.
pub fn fn_00e9ca50(
    e: &mut Engine,
    time: f32,
    volume: Ptr,
    velocity: Ptr,
    vertex0: Ptr,
    vertex1: Ptr,
    vertex2: Ptr,
    triangle_velocity: Ptr,
    out_time: Ptr,
    out_point: Ptr,
    calculate_normals: u32,
    first_normal: Ptr,
    second_normal: Ptr,
) -> bool {
    let time = x87_quiet_bits(time.to_bits());
    let kind = volume_type(e, volume);
    let table = e.mem.u32(TRI_FIND_TABLE);
    let find = e.mem.u32(table.wrapping_add(kind.wrapping_mul(4)));
    e.call(
        find,
        &args![
            time,
            volume,
            velocity,
            vertex0,
            vertex1,
            vertex2,
            triangle_velocity,
            out_time,
            out_point,
            calculate_normals,
            first_normal,
            second_normal
        ],
    )
    .bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_c7d0, fn_00e9c7d0(f32, Ptr) -> Ptr),
        entry!(0x00e9_c800, fn_00e9c800(f32, Ptr) -> Ptr),
        entry!(0x00e9_c850, fn_00e9c850(u32, Ptr) -> Ptr),
        entry!(0x00e9_c8c0, fn_00e9c8c0(f32, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(
            0x00e9_c950,
            fn_00e9c950(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, u32, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00e9_ca10,
            fn_00e9ca10(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00e9_ca50,
            fn_00e9ca50(f32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, u32, Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    /// A volume whose `Type` (slot +0xc) is the function at `type_fn`.
    fn volume(e: &mut Engine, type_fn: u32) -> u32 {
        let vtable = e.mem.alloc(0x30);
        e.mem.set_u32(vtable + 0xc, type_fn);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Two volumes of types 1 and 2 and a 6 x 6 table at the global `table` whose entries
    /// (1, 2) and (2, 1) are `forward` and `backward`.
    fn setup(e: &mut Engine, table: u32, forward: u32, backward: u32) -> (u32, u32) {
        e.map(0x0126_f000, 0x1000);
        e.register(0x00ab_0001, |_, _| 1u32.into_ret());
        e.register(0x00ab_0002, |_, _| 2u32.into_ret());
        let first = volume(e, 0x00ab_0001);
        let second = volume(e, 0x00ab_0002);
        let block = e.mem.alloc(36 * 4);
        e.mem.set_u32(block + (6 + 2) * 4, forward);
        e.mem.set_u32(block + (2 * 6 + 1) * 4, backward);
        e.mem.set_u32(table, block);
        (first, second)
    }

    #[test]
    fn fast_sphere_fills_the_static_volume() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let center = e.mem.alloc(12);
        for (i, w) in [1u32, 2, 3].iter().enumerate() {
            e.mem.set_u32(center + 4 * i as u32, *w);
        }
        let r = e.call(0x00e9_c7d0, &args![0x7f80_0001u32, center]).u32();
        assert_eq!(r, STATIC_SPHERE);
        assert_eq!(e.mem.u32(STATIC_SPHERE + 0x10), 0x7fc0_0001);
        assert_eq!(e.mem.u32(STATIC_SPHERE + 4), 1);
        assert_eq!(e.mem.u32(STATIC_SPHERE + 0xc), 3);
    }

    #[test]
    fn safe_sphere_allocates_and_constructs() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x14);
        e.register_double(MEMORY_ALLOCATE, move |_, _| block.into_ret());
        e.register(SPHERE_BV_CONSTRUCTOR, |_, a| a[0].into_ret());
        let center = e.mem.alloc(12);
        e.mem.set_u32(center + 8, 9);
        e.call_log = Some(vec![]);
        let r = e.call(0x00e9_c800, &args![2.5f32, center]).u32();
        assert_eq!(r, block);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x14]]
        );
        assert_eq!(
            calls_to(&e, SPHERE_BV_CONSTRUCTOR),
            vec![vec![block, 1.0f32.to_bits(), ZERO_POINT_011F426C]]
        );
        assert_eq!(e.mem.u32(block + 0x10), 2.5f32.to_bits());
        assert_eq!(e.mem.u32(block + 0xc), 9);
    }

    #[test]
    fn box_conversion_fits_and_copies() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_c850, &args![0x1_0000u32, 0x5000u32]).u32(), 0);
        assert_eq!(e.call(0x00e9_c850, &args![3u32, 0u32]).u32(), 0);
        assert_eq!(e.call_log.as_ref().map(Vec::len), Some(2));

        let block = e.mem.alloc(0x40);
        e.register_double(MEMORY_ALLOCATE, move |_, _| block.into_ret());
        e.register(BOX_BV_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(BOX_FROM_POINTS, |e, a| {
            for i in 0..15u32 {
                e.mem.set_u32(a[0] + 4 * i, 0x100 + i);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let r = e.call(0x00e9_c850, &args![0x0002_0005u32, 0x5000u32]).u32();
        assert_eq!(r, block);
        let fits = calls_to(&e, BOX_FROM_POINTS);
        assert_eq!(&fits[0][1..], &[0x0002_0005, 0x5000]);
        assert_eq!(
            calls_to(&e, BOX_BV_CONSTRUCTOR),
            vec![vec![
                block,
                0x011a_9490,
                ZERO_POINT_011F426C,
                0x011a_946c,
                0x011a_9478,
                0x011a_9484
            ]]
        );
        assert_eq!(e.mem.u32(block + 4), 0x100);
        assert_eq!(e.mem.u32(block + 0x3c), 0x10e);
    }

    #[test]
    fn test_dispatch_uses_the_pair_or_swaps() {
        let mut e = Engine::new();
        let (first, second) = setup(&mut e, TEST_TABLE, 0x00ab_0010, 0x00ab_0011);
        e.register(0x00ab_0010, |_, _| 1u32.into_ret());
        e.register(0x00ab_0011, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        let r = e.call(
            0x00e9_c8c0,
            &args![0x7f80_0001u32, first, 0x11u32, second, 0x22u32],
        );
        assert!(r.bool());
        assert_eq!(
            calls_to(&e, 0x00ab_0010),
            vec![vec![0x7fc0_0001, first, 0x11, second, 0x22]]
        );
        // Entry (1, 2) null: the (2, 1) entry with the volumes swapped.
        let table = e.mem.u32(TEST_TABLE);
        e.mem.set_u32(table + 8 * 4, 0);
        e.call_log = Some(vec![]);
        let r = e.call(0x00e9_c8c0, &args![1.0f32, first, 0x11u32, second, 0x22u32]);
        assert!(!r.bool());
        assert_eq!(
            calls_to(&e, 0x00ab_0011),
            vec![vec![1.0f32.to_bits(), second, 0x22, first, 0x11]]
        );
        assert_eq!(calls_to(&e, 0x00ab_0002).len(), 2);
    }

    #[test]
    fn find_dispatch_swaps_volumes_and_normals() {
        let mut e = Engine::new();
        let (first, second) = setup(&mut e, FIND_TABLE, 0x00ab_0020, 0x00ab_0021);
        e.register(0x00ab_0020, |_, _| 1u32.into_ret());
        e.register(0x00ab_0021, |_, _| 1u32.into_ret());
        let words = |a: u32, b: u32| {
            args![2.0f32, a, 0x11u32, b, 0x22u32, 0x33u32, 0x44u32, 1u32, 0x55u32, 0x66u32]
        };
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_c950, &words(first, second)).bool());
        assert_eq!(calls_to(&e, 0x00ab_0020), vec![words(first, second)]);
        let table = e.mem.u32(FIND_TABLE);
        e.mem.set_u32(table + 8 * 4, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_c950, &words(first, second)).bool());
        assert_eq!(
            calls_to(&e, 0x00ab_0021),
            vec![args![
                2.0f32, second, 0x22u32, first, 0x11u32, 0x33u32, 0x44u32, 1u32, 0x66u32, 0x55u32
            ]]
        );
    }

    #[test]
    fn triangle_dispatch_indexes_by_the_volume_type() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(0x00ab_0002, |_, _| 2u32.into_ret());
        let volume = volume(&mut e, 0x00ab_0002);
        let tests = e.mem.alloc(24);
        let finds = e.mem.alloc(24);
        e.mem.set_u32(tests + 8, 0x00ab_0030);
        e.mem.set_u32(finds + 8, 0x00ab_0031);
        e.mem.set_u32(TRI_TEST_TABLE, tests);
        e.mem.set_u32(TRI_FIND_TABLE, finds);
        e.register(0x00ab_0030, |_, _| 1u32.into_ret());
        e.register(0x00ab_0031, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        let test = args![0x7f80_0002u32, volume, 1u32, 2u32, 3u32, 4u32, 5u32];
        assert!(e.call(0x00e9_ca10, &test).bool());
        let mut expected = test.clone();
        expected[0] = 0x7fc0_0002;
        assert_eq!(calls_to(&e, 0x00ab_0030), vec![expected]);
        let find =
            args![1.5f32, volume, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32];
        assert!(!e.call(0x00e9_ca50, &find).bool());
        assert_eq!(calls_to(&e, 0x00ab_0031), vec![find]);
    }
}
