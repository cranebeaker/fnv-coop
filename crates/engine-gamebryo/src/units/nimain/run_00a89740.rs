//! `NiMain/run_00a89740` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a80540` (unit `nigeometry.cpp`, `thiscall`): `NiGeometry`'s default constructor;
/// `NiGeometry::~NiGeometry` (`thiscall`).
const GEOMETRY_CONSTRUCT: u32 = 0x00a8_0540;
const GEOMETRY_DESTRUCT: u32 = 0x00a8_0580;
/// `NiTriBasedGeom`'s vtable (by its RTTI type descriptor).
const NI_TRI_BASED_GEOM_VTABLE: u32 = 0x0109_ed94;

// Translated from 00a89740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (by its body `NiTriBasedGeom`'s default constructor): the
/// `NiGeometry` constructor (`00a80540`), then the vtable `0109ed94`. Answers `this`.
pub fn fn_00a89740(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(GEOMETRY_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_TRI_BASED_GEOM_VTABLE);
    this
}

// Translated from 00a89760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (by its body `NiTriBasedGeom`'s destructor body): stores the
/// vtable `0109ed94` and jumps to `NiGeometry::~NiGeometry` (`00a80580`).
pub fn fn_00a89760(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_TRI_BASED_GEOM_VTABLE);
    e.call(GEOMETRY_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_9740, fn_00a89740(Ptr) -> Ptr),
        entry!(0x00a8_9760, fn_00a89760(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_and_destructor_store_the_vtable_around_the_base() {
        let mut e = Engine::new();
        e.register(GEOMETRY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        e.register(GEOMETRY_DESTRUCT, |e, a| {
            assert_eq!(e.mem.u32(a[0]), NI_TRI_BASED_GEOM_VTABLE);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_9740, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TRI_BASED_GEOM_VTABLE);
        e.mem.set_u32(this, 0);
        e.call(0x00a8_9760, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|c| c.0 == GEOMETRY_DESTRUCT && c.1 == vec![this]));
        assert!(log
            .iter()
            .any(|c| c.0 == GEOMETRY_CONSTRUCT && c.1 == vec![this]));
    }
}
