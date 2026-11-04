//! `(unplaced)/run_0049f1c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name
/// `.?AV?$BSSimpleArray@VNiPoint3@@$0EAA@@` (read at the word before the
/// vtable: its complete object locator, then the type descriptor).
const POINT_ARRAY_VTABLE: u32 = 0x0101_e114;
/// `0049f990` (unnamed in the engine map; `thiscall`, `RET 8`): the
/// array's set-up with two words (both 0 here).
const ARRAY_INIT_0049F990: u32 = 0x0049_f990;
/// `008454f0` (unnamed in the engine map; `thiscall`, `RET 4`): the
/// array's release with one word (1 here).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 0049f1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray<NiPoint3,1024>` instance (RTTI of its
/// vtable; unnamed in the engine map): stores the vtable, then
/// `0049f990(this, 0, 0)`. Returns `this`.
pub fn fn_0049f1c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), POINT_ARRAY_VTABLE);
    e.call(ARRAY_INIT_0049F990, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0049f1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray<NiPoint3,1024>` instance (RTTI of its
/// vtable; unnamed in the engine map): stores the vtable, then
/// `008454f0(this, 1)`.
pub fn fn_0049f1f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), POINT_ARRAY_VTABLE);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0049_f1c0, fn_0049f1c0(Ptr) -> Ptr),
        entry!(0x0049_f1f0, fn_0049f1f0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0049f1c0_sets_the_vtable_and_initializes_empty() {
        let mut e = Engine::new();
        e.register(ARRAY_INIT_0049F990, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0049_f1c0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), POINT_ARRAY_VTABLE);
        let init: Vec<_> = log.iter().filter(|c| c.0 == ARRAY_INIT_0049F990).collect();
        assert_eq!(init.len(), 1);
        assert_eq!(init[0].1, vec![this, 0, 0]);
    }

    #[test]
    fn at_0049f1f0_sets_the_vtable_then_releases() {
        let mut e = Engine::new();
        e.register(ARRAY_RELEASE_008454F0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0049_f1f0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), POINT_ARRAY_VTABLE);
        let release: Vec<_> = log
            .iter()
            .filter(|c| c.0 == ARRAY_RELEASE_008454F0)
            .collect();
        assert_eq!(release[0].1, vec![this, 1]);
    }
}
