//! `fallout shared/pathfinding/run_006f4cd0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006f4d00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f4d00 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// A `VirtualPathingNode` constructor from a location (no name in the
/// engine map; the same shape as `006f4da0` without a door): the location at
/// +4 constructed (`006dcbb0`) and assigned (`006dcce0`), no door (+0x2c),
/// flags 2, and flag 1 when `PathingLocation::ResolveNavMeshInfo`
/// (`006dd6f0`, with 0) succeeds. Returns `this`.
pub fn fn_006f4d00(e: &mut Engine, this: Ptr, location: u32) -> Ptr {
    let base = this.addr();
    e.call(0x006d_cbb0, &args![base + 4]);
    e.call(0x006d_cce0, &args![base + 4, location]);
    e.mem.set_u32(base + 0x2c, 0);
    e.mem.set_u32(base, 2);
    if e.call(0x006d_d6f0, &args![base + 4, 0u32]).bool() {
        let flags = e.mem.u32(base);
        e.mem.set_u32(base, flags | 1);
    }
    this
}

// Translated from 006f4cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `VirtualPathingNode` default constructor (no name in the engine
/// map; `thiscall`, plain `RET`): the location at +4 constructed
/// (`006dcbb0`), the flags (+0) and the door reference (+0x2c) cleared.
/// Returns `this`.
pub fn fn_006f4cd0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x006d_cbb0, &args![base + 4]);
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 0x2c, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f_4d00, fn_006f4d00(Ptr, u32) -> Ptr),
        entry!(0x006f_4cd0, fn_006f4cd0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_006f4d00_assigns_the_location_and_resolves_it() {
        for resolves in [false, true] {
            let mut e = Engine::new();
            e.register(0x006d_cbb0, |_, _| Ret::default());
            e.register(0x006d_cce0, |_, _| Ret::default());
            e.register_double(0x006d_d6f0, move |_, _| Ret {
                eax: u32::from(resolves),
                ..Ret::default()
            });
            let node = e.mem.alloc(0x30);
            e.mem.set_u32(node + 0x2c, 0x1234);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006f_4d00, &args![node, 0x5000u32]).u32(), node);
            let log = e.call_log.take().unwrap();
            let assigns: Vec<_> = log.iter().filter(|(a, _)| *a == 0x006d_cce0).collect();
            assert_eq!(assigns[0].1, vec![node + 4, 0x5000]);
            let resolve: Vec<_> = log.iter().filter(|(a, _)| *a == 0x006d_d6f0).collect();
            assert_eq!(resolve[0].1, vec![node + 4, 0]);
            assert_eq!(e.mem.u32(node), if resolves { 3 } else { 2 });
            assert_eq!(e.mem.u32(node + 0x2c), 0);
        }
    }

    #[test]
    fn fn_006f4cd0_constructs_an_empty_node() {
        use super::*;
        let mut e = Engine::new();
        e.register(0x006d_cbb0, |_, _| Ret::default());
        let node = e.mem.alloc(0x30);
        e.mem.set_u32(node, 7);
        e.mem.set_u32(node + 0x2c, 8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_4cd0, &args![node]).u32(), node);
        let log = e.call_log.take().unwrap();
        let built: Vec<_> = log.iter().filter(|c| c.0 == 0x006d_cbb0).collect();
        assert_eq!(built.len(), 1);
        assert_eq!(built[0].1, vec![node + 4]);
        assert_eq!(e.mem.u32(node), 0);
        assert_eq!(e.mem.u32(node + 0x2c), 0);
    }
}
