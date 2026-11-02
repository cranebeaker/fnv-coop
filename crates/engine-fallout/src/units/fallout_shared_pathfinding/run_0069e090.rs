//! `fallout shared/pathfinding/run_0069e090` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00416870`: a point's constructor from three floats (`thiscall`).
const POINT_CONSTRUCT: u32 = 0x0041_6870;

// Translated from 0069e0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the 2D point at `this` scaled by `scale` as a 3D point in
/// `result` (`z` 0), each product rounded to a `float`. Returns `result`.
pub fn fn_0069e0c0(e: &mut Engine, this: Ptr, result: Ptr, scale: f32) -> Ptr {
    let base = this.addr();
    let y = x87_store32(x87_mul(
        x87_load32(scale.to_bits()),
        x87_m32(e.mem.u32(base + 4)),
    ));
    let x = x87_store32(x87_mul(
        x87_load32(scale.to_bits()),
        x87_m32(e.mem.u32(base)),
    ));
    e.call(POINT_CONSTRUCT, &args![result, x, y, 0u32]);
    result
}

// Translated from 0069e090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the reference at `this` is set (`thiscall`, plain `RET`): false
/// when the word at `+0` is 0 or the 16-bit index at `+4` is 0xFFFF, true
/// otherwise. The engine map gives it no name.
pub fn fn_0069e090(e: &mut Engine, this: Ptr) -> bool {
    if e.mem.u32(this.addr()) == 0 {
        return false;
    }
    e.mem.u16(this.addr() + 4) != 0xffff
}

/// `NavMeshPOVSearch`'s vtable (`0106b39c`, RTTI
/// `.?AVNavMeshPOVSearch@@`).
const VTABLE_NAV_MESH_POV_SEARCH: u32 = 0x0106_b39c;
/// The constructors `0069e110` runs (`thiscall`, no stack words): the base
/// part (`006a1500`, on `this`) and the members at `+0x2058` (`006a14b0`),
/// `+0x2068` (`006a16d0`), `+0x2088` and `+0x20b0`
/// (`PathingLocation::PathingLocation`, engine map, `006dcbb0`), `+0x20d8`
/// (`006a1860`) and `+0x20e8` (`006a18b0`).
const BASE_CONSTRUCT_006A1500: u32 = 0x006a_1500;
const MEMBER_CONSTRUCT_006A14B0: u32 = 0x006a_14b0;
const MEMBER_CONSTRUCT_006A16D0: u32 = 0x006a_16d0;
const PATHING_LOCATION_CONSTRUCT: u32 = 0x006d_cbb0;
const MEMBER_CONSTRUCT_006A1860: u32 = 0x006a_1860;
const MEMBER_CONSTRUCT_006A18B0: u32 = 0x006a_18b0;

// Translated from 0069e110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0069e110 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs a `NavMeshPOVSearch` (vtable `0106b39c`, RTTI) at `this`
/// (`thiscall`, plain `RET`): the base part (`006a1500`), then the vtable,
/// then the members at `+0x2058`, `+0x2068`, `+0x2088`, `+0x20b0`
/// (two `PathingLocation`s), `+0x20d8` and `+0x20e8`, in that order.
/// Returns `this`.
pub fn fn_0069e110(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_006A1500, &args![this]);
    e.mem.set_u32(base, VTABLE_NAV_MESH_POV_SEARCH);
    e.call(MEMBER_CONSTRUCT_006A14B0, &args![base + 0x2058]);
    e.call(MEMBER_CONSTRUCT_006A16D0, &args![base + 0x2068]);
    e.call(PATHING_LOCATION_CONSTRUCT, &args![base + 0x2088]);
    e.call(PATHING_LOCATION_CONSTRUCT, &args![base + 0x20b0]);
    e.call(MEMBER_CONSTRUCT_006A1860, &args![base + 0x20d8]);
    e.call(MEMBER_CONSTRUCT_006A18B0, &args![base + 0x20e8]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0069_e0c0, fn_0069e0c0(Ptr, Ptr, f32) -> Ptr),
        entry!(0x0069_e090, fn_0069e090(Ptr) -> bool),
        entry!(0x0069_e110, fn_0069e110(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scales_into_a_3d_point() {
        let mut e = Engine::new();
        e.register(POINT_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            Ret::default()
        });
        let this = e.mem.alloc(8);
        e.mem.set_f32(this, 1.5);
        e.mem.set_f32(this + 4, -2.0);
        let result = e.mem.alloc(12);
        e.mem.set_f32(result + 8, 9.0);
        assert_eq!(
            e.call(0x0069_e0c0, &args![this, result, 2.0f32]).u32(),
            result
        );
        assert_eq!(e.mem.f32(result), 3.0);
        assert_eq!(e.mem.f32(result + 4), -4.0);
        assert_eq!(e.mem.f32(result + 8), 0.0);
    }

    #[test]
    fn at_0069e090_needs_a_word_and_an_index() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        assert!(!e.call(0x0069_e090, &args![this]).bool());
        e.mem.set_u32(this, 0x4000);
        e.mem.set_u32(this + 4, 0x1234_ffff);
        assert!(!e.call(0x0069_e090, &args![this]).bool());
        e.mem.set_u32(this + 4, 0xffff_0003);
        assert!(e.call(0x0069_e090, &args![this]).bool());
    }

    #[test]
    fn at_0069e110_constructs_the_base_then_the_members() {
        let mut e = Engine::new();
        // The base constructor sees no vtable yet; it writes one of its own.
        e.register(BASE_CONSTRUCT_006A1500, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        for addr in [
            MEMBER_CONSTRUCT_006A14B0,
            MEMBER_CONSTRUCT_006A16D0,
            PATHING_LOCATION_CONSTRUCT,
            MEMBER_CONSTRUCT_006A1860,
            MEMBER_CONSTRUCT_006A18B0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x2100);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_e110, &args![this]).u32(), this);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x0069_e110)
            .collect();
        assert_eq!(
            log,
            vec![
                (BASE_CONSTRUCT_006A1500, vec![this]),
                (MEMBER_CONSTRUCT_006A14B0, vec![this + 0x2058]),
                (MEMBER_CONSTRUCT_006A16D0, vec![this + 0x2068]),
                (PATHING_LOCATION_CONSTRUCT, vec![this + 0x2088]),
                (PATHING_LOCATION_CONSTRUCT, vec![this + 0x20b0]),
                (MEMBER_CONSTRUCT_006A1860, vec![this + 0x20d8]),
                (MEMBER_CONSTRUCT_006A18B0, vec![this + 0x20e8]),
            ]
        );
        assert_eq!(e.mem.u32(this), VTABLE_NAV_MESH_POV_SEARCH);
    }
}
