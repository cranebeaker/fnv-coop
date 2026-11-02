//! `NiMain/run_00a8a990` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `NiTriBasedGeomData` (RTTI `.?AVNiTriBasedGeomData@@`).
const NI_TRI_BASED_GEOM_DATA_VTABLE: u32 = 0x0109_f2f4;
/// `00a67c70` (unit `nigeometrydata.cpp`): the destructor body this one
/// jumps to.
const GEOMETRY_DATA_DESTRUCT: u32 = 0x00a6_7c70;

// Translated from 00a8a990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiTriBasedGeomData` destructor. Stores
/// the class vtable, then jumps to the `NiGeometryData` destructor body
/// (`00a67c70`).
pub fn fn_00a8a990(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_TRI_BASED_GEOM_DATA_VTABLE);
    e.call(GEOMETRY_DATA_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_a990, fn_00a8a990(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a8a990_sets_the_vtable_then_runs_the_geometry_data_destructor() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.register_double(GEOMETRY_DATA_DESTRUCT, |e, a| {
            assert_eq!(e.mem.u32(a[0]), NI_TRI_BASED_GEOM_DATA_VTABLE);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a8_a990, &args![this]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|c| c.0 == GEOMETRY_DATA_DESTRUCT)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, [this]);
    }
}
