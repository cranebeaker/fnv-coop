//! `NiMain/run_00a8a920` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a67b20` (unnamed in the engine map, unit `nigeometrydata.cpp`; `thiscall`, `RET 0x1c`):
/// the `NiGeometryData` base constructor this one runs with the first seven words.
const GEOMETRY_DATA_CONSTRUCTOR: u32 = 0x00a6_7b20;
/// The vtable stored after the base constructor: RTTI `NiTriBasedGeomData`.
const TRI_BASED_GEOM_DATA_VTABLE: u32 = 0x0109_f2f4;

// Translated from 00a8a920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its vtable (`0109f2f4`, RTTI `NiTriBasedGeomData`) the
/// `NiTriBasedGeomData` constructor (`thiscall`, `RET 0x20`): runs the `NiGeometryData`
/// constructor `00a67b20` with the first seven stack words in order, sets the vtable and
/// stores the low half of the eighth word in `m_usTriangles` (Xbox PDB, `+0x40`). Answers
/// `this`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00a8a920(
    e: &mut Engine,
    this: Ptr,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    triangles: u32,
) -> Ptr {
    e.call(
        GEOMETRY_DATA_CONSTRUCTOR,
        &args![this, a1, a2, a3, a4, a5, a6, a7],
    );
    e.mem.set_u32(this.addr(), TRI_BASED_GEOM_DATA_VTABLE);
    e.mem.set_u16(this.addr() + 0x40, triangles as u16);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a8_a920,
        fn_00a8a920(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_the_base_constructor_then_sets_the_vtable_and_triangle_count() {
        let mut e = Engine::new();
        e.register(GEOMETRY_DATA_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0xdead_beef);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x44);
        let answer = e
            .call(
                0x00a8_a920,
                &args![
                    this,
                    1u32,
                    2u32,
                    3u32,
                    4u32,
                    5u32,
                    6u32,
                    7u32,
                    0xabcd_0123u32
                ],
            )
            .u32();
        assert_eq!(answer, this);
        let calls: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == GEOMETRY_DATA_CONSTRUCTOR)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(calls, vec![vec![this, 1, 2, 3, 4, 5, 6, 7]]);
        assert_eq!(e.mem.u32(this), TRI_BASED_GEOM_DATA_VTABLE);
        assert_eq!(e.mem.u16(this + 0x40), 0x0123);
    }
}
