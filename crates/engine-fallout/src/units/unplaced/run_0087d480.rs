//! `(unplaced)/run_0087d480` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Vtable of `BSSimpleArray<NiPointer<NiNode>,1024>` (RTTI
/// `.?AV?$BSSimpleArray@V?$NiPointer@VNiNode@@@@$0EAA@@@`).
const ARRAY_VTABLE: u32 = 0x0108_40e4;
/// The array's release (`006c6200`, thiscall, one word; the engine map's
/// name, `BSSimpleArray<NiPointer<ParticleShaderProperty>_1024>::_DestructItems`,
/// is of folded code).
const ARRAY_RELEASE_006C6200: u32 = 0x006c_6200;

// Translated from 0087d480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPointer<NiNode>,1024>`'s destructor body (by the
/// vtable it stores; unnamed in the engine map): the vtable, then
/// `006c6200(this, 1)`.
pub fn fn_0087d480(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE);
    e.call(ARRAY_RELEASE_006C6200, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0087_d480, fn_0087d480(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_vtable_then_releases() {
        let mut e = Engine::new();
        e.register(ARRAY_RELEASE_006C6200, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0087_d480, &args![this]);
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 4), ARRAY_VTABLE);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == ARRAY_RELEASE_006C6200)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![this, 1]);
    }
}
