//! `fallout shared/pathfinding/run_006a9200` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006a92c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0106b78c` (RTTI `.?AVNavMeshSearchFlee@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `006a7bc0`. One word, plain `RET`, returns nothing.
pub fn fn_006a92c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_b78c);
    e.call(0x006a_7bc0, &args![this]);
}

/// `NavMeshSearchFlee`'s vtable (`0106b78c`, RTTI `.?AVNavMeshSearchFlee@@`).
const VTABLE_NAV_MESH_SEARCH_FLEE: u32 = 0x0106_b78c;
/// `006a7b10` (`thiscall`): the `NavMeshSearch` part's constructor.
const BASE_CONSTRUCT: u32 = 0x006a_7b10;
/// `006815c0` (`thiscall`): returns `this` (the members at `+0x2070` and
/// `+0x2084`).
const SELF_006815C0: u32 = 0x0068_15c0;
/// `00401030` (cdecl: block): frees a block.
const FREE_BLOCK: u32 = 0x0040_1030;

// Translated from 006a9200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a9200 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs a `NavMeshSearchFlee` at `this` (`thiscall`, plain `RET`): the
/// base part (`006a7b10`), the vtable `0106b78c`, `006815c0` on the members
/// at `+0x2070` and `+0x2084`, then the word at `+0x2058` and the byte at
/// `+0x2080` are cleared. Returns `this`.
pub fn fn_006a9200(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, VTABLE_NAV_MESH_SEARCH_FLEE);
    e.call(SELF_006815C0, &args![base + 0x2070]);
    e.call(SELF_006815C0, &args![base + 0x2084]);
    e.mem.set_u32(base + 0x2058, 0);
    e.mem.set_u8(base + 0x2080, 0);
    this
}

// Translated from 006a9290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `NavMeshSearchFlee` (`thiscall`;
/// flags; `RET 4`): the destructor `006a92c0`, then the block is freed
/// (`00401030`) when bit 0 of `flags` is set. Returns `this`.
pub fn fn_006a9290(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006a92c0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006a_92c0, fn_006a92c0(Ptr)),
        entry!(0x006a_9200, fn_006a9200(Ptr) -> Ptr),
        entry!(0x006a_9290, fn_006a9290(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_006a92c0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x006a_7bc0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006a_92c0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x006a_7bc0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0106_b78c);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0106_b78c,
            "the vtable was stored before the call"
        );
    }

    #[test]
    fn at_006a9200_constructs_a_flee_search() {
        use super::*;
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            e.mem.set_u32(a[0] + 0x2058, 0x2222);
            e.mem.set_u32(a[0] + 0x2080, 0x3333_3333);
            Ret::default()
        });
        e.register(SELF_006815C0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x2090);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_9200, &args![this]).u32(), this);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x006a_9200)
            .collect();
        assert_eq!(
            log,
            vec![
                (BASE_CONSTRUCT, vec![this]),
                (SELF_006815C0, vec![this + 0x2070]),
                (SELF_006815C0, vec![this + 0x2084]),
            ]
        );
        assert_eq!(e.mem.u32(this), VTABLE_NAV_MESH_SEARCH_FLEE);
        assert_eq!(e.mem.u32(this + 0x2058), 0);
        // Only the byte at +0x2080 is cleared.
        assert_eq!(e.mem.u32(this + 0x2080), 0x3333_3300);
    }

    #[test]
    fn at_006a9290_destroys_and_frees_on_bit_0() {
        use super::*;
        let mut e = Engine::new();
        e.register(0x006a_7bc0, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        for (flags, frees) in [(0u32, 0usize), (1, 1), (0xfe, 0)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006a_9290, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(this), VTABLE_NAV_MESH_SEARCH_FLEE);
            assert_eq!(log.iter().filter(|(a, _)| *a == 0x006a_7bc0).count(), 1);
            let freed: Vec<_> = log.iter().filter(|(a, _)| *a == FREE_BLOCK).collect();
            assert_eq!(freed.len(), frees);
            if frees == 1 {
                assert_eq!(freed[0].1, vec![this]);
            }
        }
    }
}
