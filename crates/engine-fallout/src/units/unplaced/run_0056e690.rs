//! `(unplaced)/run_0056e690` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `bhkSphereRepShape` and `bhkConvexShape` constructors (classes named
//! by their vtables' RTTI type descriptors), each storing its vtable and
//! counting the instance in a global.

#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkRefObject::bhkRefObject` (engine map), `0056e6f0`.
const REF_OBJECT_CONSTRUCT: u32 = 0x0056_e6f0;
/// `bhkConvexShape` vtable (RTTI `.?AVbhkConvexShape@@`).
const VTABLE_0056E690: u32 = 0x0103_0a24;
/// Instance counter incremented by [`fn_0056e690`].
const COUNT_0056E690: u32 = 0x0126_8250;
/// `bhkSphereRepShape` vtable (RTTI `.?AVbhkSphereRepShape@@`).
const VTABLE_0056E6C0: u32 = 0x0103_0b0c;
/// Instance counter incremented by [`fn_0056e6c0`].
const COUNT_0056E6C0: u32 = 0x0126_8274;

// Translated from 0056e6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSphereRepShape` constructor (no Xbox PDB name; the class from the
/// vtable's RTTI): `bhkRefObject::bhkRefObject`
/// (`0056e6f0`), then the vtable `01030b0c` and one more instance in the
/// counter at `01268274`. Returns `this`.
pub fn fn_0056e6c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_0056E6C0);
    let count = e.mem.u32(COUNT_0056E6C0).wrapping_add(1);
    e.mem.set_u32(COUNT_0056E6C0, count);
    this
}

// Translated from 0056e690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexShape` constructor (no Xbox PDB name; the class from the
/// vtable's RTTI): the `bhkSphereRepShape` constructor ([`fn_0056e6c0`]),
/// then the vtable `01030a24` and one more instance in the counter at
/// `01268250`. Returns `this`.
pub fn fn_0056e690(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0056e6c0(e, this);
    e.mem.set_u32(this.addr(), VTABLE_0056E690);
    let count = e.mem.u32(COUNT_0056E690).wrapping_add(1);
    e.mem.set_u32(COUNT_0056E690, count);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0056e690, fn_0056e690(Ptr) -> Ptr),
        entry!(0x0056e6c0, fn_0056e6c0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(COUNT_0056E690, 3u32);
        e.set_global(COUNT_0056E6C0, 7u32);
        e.register(REF_OBJECT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        e
    }

    #[test]
    fn base_constructor_sets_its_vtable_and_counts() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0056_e6c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_0056E6C0);
        assert_eq!(e.mem.u32(COUNT_0056E6C0), 8);
        assert_eq!(e.mem.u32(COUNT_0056E690), 3);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(REF_OBJECT_CONSTRUCT, vec![this])));
    }

    #[test]
    fn derived_constructor_runs_the_base_then_counts_itself() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0056_e690, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_0056E690);
        assert_eq!(e.mem.u32(COUNT_0056E6C0), 8);
        assert_eq!(e.mem.u32(COUNT_0056E690), 4);
    }
}
