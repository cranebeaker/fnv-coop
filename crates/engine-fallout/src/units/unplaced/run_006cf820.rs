//! `(unplaced)/run_006cf820` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Vtable of `BSSimpleArray<PathBuilder::VirtualNodeSegment,1024>` (RTTI
/// `.?AV?$BSSimpleArray@UVirtualNodeSegment@PathBuilder@@$0EAA@@@`).
const ARRAY_VTABLE: u32 = 0x0106_c9b8;
/// The array's release (`008454f0`, thiscall, one word: 1 here).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 006cf820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathBuilder::VirtualNodeSegment,1024>`'s destructor body
/// (by the vtable it stores; unnamed in the engine map): the vtable, then
/// `008454f0(this, 1)`.
pub fn fn_006cf820(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006c_f820, fn_006cf820(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_vtable_then_releases() {
        let mut e = Engine::new();
        e.register(ARRAY_RELEASE_008454F0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x006c_f820, &args![this]);
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 4), ARRAY_VTABLE);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == ARRAY_RELEASE_008454F0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![this, 1]);
    }
}
