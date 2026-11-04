//! `fallout shared/pathfinding/run_006a9cb0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NavMeshSearchHide`'s vtable (`0106b7dc`, RTTI `.?AVNavMeshSearchHide@@`).
const VTABLE_NAV_MESH_SEARCH_HIDE: u32 = 0x0106_b7dc;
/// `006a6a40` (`thiscall`): the base part's constructor.
const BASE_CONSTRUCT_006A6A40: u32 = 0x006a_6a40;
/// `006815c0` (`thiscall`): returns `this` (the members at `+0x20bc` and
/// `+0x20cc`).
const SELF_006815C0: u32 = 0x0068_15c0;

// Translated from 006a9cb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a9cb0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs a `NavMeshSearchHide` at `this` (`thiscall`, plain `RET`): the
/// base part (`006a6a40`), the vtable `0106b7dc`, `006815c0` on the members
/// at `+0x20bc` and `+0x20cc`, then the word at `+0x20a4` is cleared.
/// Returns `this`.
pub fn fn_006a9cb0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_006A6A40, &args![this]);
    e.mem.set_u32(base, VTABLE_NAV_MESH_SEARCH_HIDE);
    e.call(SELF_006815C0, &args![base + 0x20bc]);
    e.call(SELF_006815C0, &args![base + 0x20cc]);
    e.mem.set_u32(base + 0x20a4, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006a_9cb0, fn_006a9cb0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_006a9cb0_constructs_a_hide_search() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT_006A6A40, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            e.mem.set_u32(a[0] + 0x20a4, 0x2222);
            Ret::default()
        });
        e.register(SELF_006815C0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20d0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_9cb0, &args![this]).u32(), this);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x006a_9cb0)
            .collect();
        assert_eq!(
            log,
            vec![
                (BASE_CONSTRUCT_006A6A40, vec![this]),
                (SELF_006815C0, vec![this + 0x20bc]),
                (SELF_006815C0, vec![this + 0x20cc]),
            ]
        );
        assert_eq!(e.mem.u32(this), VTABLE_NAV_MESH_SEARCH_HIDE);
        assert_eq!(e.mem.u32(this + 0x20a4), 0);
    }
}
