//! `fallout shared/pathfinding/run_006aa620` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NavMeshSearchLOS`'s vtable (`0106b830`, RTTI `.?AVNavMeshSearchLOS@@`).
const VTABLE_NAV_MESH_SEARCH_LOS: u32 = 0x0106_b830;
/// `006a7b10` (`thiscall`): the `NavMeshSearch` part's constructor.
const BASE_CONSTRUCT: u32 = 0x006a_7b10;
/// `00633c90` (`thiscall`; one word; `RET 4`): run on the member at
/// `+0x2070` with 0.
const MEMBER_CONSTRUCT_00633C90: u32 = 0x0063_3c90;
/// The `float` the constructor loads into `+0x209c` (`0106b7f0`; the exe
/// holds `FLT_MAX` there).
const INITIAL_DISTANCE: u32 = 0x0106_b7f0;

// Translated from 006aa620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006aa620 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs a `NavMeshSearchLOS` at `this` (`thiscall`, plain `RET`): the
/// base part (`006a7b10`), the vtable `0106b830`, `00633c90` on `+0x2070`
/// with 0, then the word at `+0x2058` is 0, the byte at `+0x208c` 0xFD, the
/// byte at `+0x2074` 0, the words at `+0x2090`, `+0x2094`, `+0x2098` 0 and
/// the `float` at `+0x209c` the one at `0106b7f0` (an `FLD`/`FSTP` copy).
/// Returns `this`.
pub fn fn_006aa620(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, VTABLE_NAV_MESH_SEARCH_LOS);
    e.call(MEMBER_CONSTRUCT_00633C90, &args![base + 0x2070, 0u32]);
    e.mem.set_u32(base + 0x2058, 0);
    e.mem.set_u8(base + 0x208c, 0xfd);
    e.mem.set_u8(base + 0x2074, 0);
    e.mem.set_u32(base + 0x2090, 0);
    e.mem.set_u32(base + 0x2094, 0);
    e.mem.set_u32(base + 0x2098, 0);
    let distance = x87_quiet_bits(e.global::<u32>(INITIAL_DISTANCE));
    e.mem.set_u32(base + 0x209c, distance);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006a_a620, fn_006aa620(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_006aa620_constructs_a_los_search() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT, |e, a| {
            for offset in (0..0x20a0).step_by(4) {
                e.mem.set_u32(a[0] + offset, 0xcccc_cccc);
            }
            Ret::default()
        });
        e.register(MEMBER_CONSTRUCT_00633C90, |_, _| Ret::default());
        e.map(INITIAL_DISTANCE & !0xfff, 0x1000);
        // A signalling NaN comes out of the FLD/FSTP copy quieted.
        e.set_global(INITIAL_DISTANCE, 0x7f80_0001u32);
        let this = e.mem.alloc(0x20a0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_a620, &args![this]).u32(), this);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x006a_a620)
            .collect();
        assert_eq!(
            log,
            vec![
                (BASE_CONSTRUCT, vec![this]),
                (MEMBER_CONSTRUCT_00633C90, vec![this + 0x2070, 0]),
            ]
        );
        assert_eq!(e.mem.u32(this), VTABLE_NAV_MESH_SEARCH_LOS);
        assert_eq!(e.mem.u32(this + 0x2058), 0);
        assert_eq!(e.mem.u32(this + 0x208c), 0xcccc_ccfd);
        assert_eq!(e.mem.u32(this + 0x2074), 0xcccc_cc00);
        for offset in [0x2090, 0x2094, 0x2098] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x209c), 0x7fc0_0001);
        e.set_global(INITIAL_DISTANCE, f32::MAX);
        e.call(0x006a_a620, &args![this]);
        assert_eq!(e.mem.f32(this + 0x209c), f32::MAX);
    }
}
