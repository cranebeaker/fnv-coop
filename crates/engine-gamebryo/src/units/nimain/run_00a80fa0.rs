//! `NiMain/run_00a80fa0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiObject`'s constructor (`00a5d3a0`, `this` only).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// `NiObject`'s destructor body (`00a5d3d0`, `this` only).
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
/// The `NiCollisionObject` vtable (its RTTI locator names
/// `.?AVNiCollisionObject@@`).
const NI_COLLISION_OBJECT_VTABLE: u32 = 0x0109_e174;
/// `NiCollisionObject::m_pkSceneObject` (Xbox PDB) +0x08.
const SCENE_OBJECT: u32 = 0x08;

// Translated from 00a80fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiCollisionObject` default constructor: runs the `NiObject`
/// constructor, stores the class's vtable and clears the scene object.
/// Returns `this`.
pub fn fn_00a80fa0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_COLLISION_OBJECT_VTABLE);
    e.mem.set_u32(this.addr().wrapping_add(SCENE_OBJECT), 0);
    this
}

// Translated from 00a80fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiCollisionObject` destructor body: stores the class's vtable and
/// continues into the `NiObject` destructor body (a tail jump).
pub fn fn_00a80fc0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_COLLISION_OBJECT_VTABLE);
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a80fa0, fn_00a80fa0(Ptr) -> Ptr),
        entry!(0x00a80fc0, fn_00a80fc0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn constructor_runs_the_base_and_clears_the_scene_object() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        let this = e.mem.alloc(0x0c);
        e.mem.set_u32(this + SCENE_OBJECT, 0xdead_beef);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_0fa0, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, NI_OBJECT_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), NI_COLLISION_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + SCENE_OBJECT), 0);
    }

    #[test]
    fn destructor_stores_the_vtable_then_runs_the_base() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_DESTRUCT, |e, a| {
            assert_eq!(e.mem.u32(a[0]), NI_COLLISION_OBJECT_VTABLE);
            Ret::default()
        });
        let this = e.mem.alloc(0x0c);
        e.call_log = Some(vec![]);
        e.call(0x00a8_0fc0, &args![this]);
        assert_eq!(calls_to(&e, NI_OBJECT_DESTRUCT), vec![vec![this]]);
    }
}
