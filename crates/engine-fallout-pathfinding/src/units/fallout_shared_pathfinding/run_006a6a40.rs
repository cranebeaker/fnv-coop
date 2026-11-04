//! `fallout shared/pathfinding/run_006a6a40` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NavMeshSearch`'s vtable (`0106b63c`; RTTI `.?AVNavMeshSearch@@`).
const VTABLE_NAV_MESH_SEARCH: u32 = 0x0106_b63c;
/// `006a7b10`: the base constructor (`thiscall`).
const BASE_CONSTRUCT: u32 = 0x006a_7b10;
/// `00501cd0`: the constructor of the member at `+0x208c` (`thiscall`).
const MEMBER_CONSTRUCT_00501CD0: u32 = 0x0050_1cd0;

// Translated from 006a6a40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a6a40 - the C++ exception frame (FS:[0]) is not translated
/// The `NavMeshSearch` constructor (by the vtable it sets): the base
/// constructor, the vtable, the member at `+0x208c` (`00501cd0`), and the
/// words at `+0x2058`, `+0x2084` and `+0x2088` cleared. Returns `this`.
pub fn fn_006a6a40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VTABLE_NAV_MESH_SEARCH);
    e.call(MEMBER_CONSTRUCT_00501CD0, &args![base + 0x208c]);
    e.mem.set_u32(base + 0x2058, 0);
    e.mem.set_u32(base + 0x2084, 0);
    e.mem.set_u32(base + 0x2088, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006a_6a40, fn_006a6a40(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_sets_vtable_member_and_words() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        e.register(MEMBER_CONSTRUCT_00501CD0, |_, _| Ret::default());
        let this = e.mem.alloc(0x2100);
        for offset in [0x2058u32, 0x2084, 0x2088] {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_6a40, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_NAV_MESH_SEARCH);
        for offset in [0x2058u32, 0x2084, 0x2088] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(MEMBER_CONSTRUCT_00501CD0, vec![this + 0x208c])));
    }
}
