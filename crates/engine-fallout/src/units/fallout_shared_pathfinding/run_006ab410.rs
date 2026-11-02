//! `fallout shared/pathfinding/run_006ab410` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NavMeshSearchMaxCost`'s vtable (`0106b87c`, RTTI
/// `.?AVNavMeshSearchMaxCost@@`).
const VTABLE_NAV_MESH_SEARCH_MAX_COST: u32 = 0x0106_b87c;
/// `006a6a40` (`thiscall`): the base part's constructor.
const BASE_CONSTRUCT_006A6A40: u32 = 0x006a_6a40;

// Translated from 006ab410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a `NavMeshSearchMaxCost` at `this` (`thiscall`, plain `RET`):
/// the base part (`006a6a40`), the vtable `0106b87c`, the `float` at
/// `+0x20a4` 0.0 and the byte at `+0x20a8` 0. Returns `this`.
pub fn fn_006ab410(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_006A6A40, &args![this]);
    e.mem.set_u32(base, VTABLE_NAV_MESH_SEARCH_MAX_COST);
    e.mem.set_u32(base + 0x20a4, x87_store32(0.0));
    e.mem.set_u8(base + 0x20a8, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006a_b410, fn_006ab410(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_006ab410_constructs_a_max_cost_search() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT_006A6A40, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            e.mem.set_u32(a[0] + 0x20a4, 0x2222);
            e.mem.set_u32(a[0] + 0x20a8, 0x3333_3333);
            Ret::default()
        });
        let this = e.mem.alloc(0x20ac);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_b410, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == BASE_CONSTRUCT_006A6A40)
            .collect();
        assert_eq!(base.len(), 1);
        assert_eq!(base[0].1, vec![this]);
        assert_eq!(e.mem.u32(this), VTABLE_NAV_MESH_SEARCH_MAX_COST);
        assert_eq!(e.mem.u32(this + 0x20a4), 0);
        assert_eq!(e.mem.u32(this + 0x20a8), 0x3333_3300);
    }
}
