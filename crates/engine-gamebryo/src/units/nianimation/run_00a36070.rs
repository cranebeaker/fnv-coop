//! `NiAnimation/run_00a36070` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiInterpController` constructor (`00a2f8c0`, `thiscall`; the
/// engine map has no name), the controller's array set-up (`00a35a80`,
/// `thiscall (this, interpolators, links)`) and
/// `BSMultiTargetTreadTransfController`'s vtable (`01096694`).
const BASE_CONSTRUCTOR: u32 = 0x00a2_f8c0;
const SET_UP_ARRAYS: u32 = 0x00a3_5a80;
const MULTI_TARGET_VTABLE: u32 = 0x0109_6694;

// Translated from 00a36070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSMultiTargetTreadTransfController` constructor (the engine map has
/// no name; it writes that class's vtable): the base constructor
/// `00a2f8c0`, no interpolators or links (arrays null, counts 0), the
/// vtable, then the arrays set up for `interpolators` and `links`
/// (`00a35a80`). Answers `this`.
pub fn fn_00a36070(e: &mut Engine, this: Ptr, interpolators: u32, links: u32) -> Ptr {
    let object = this.addr();
    e.call(BASE_CONSTRUCTOR, &args![object]);
    e.mem.set_u32(object + 0x34, 0);
    e.mem.set_u32(object + 0x3c, 0);
    e.mem.set_u16(object + 0x38, 0);
    e.mem.set_u32(object, MULTI_TARGET_VTABLE);
    e.mem.set_u16(object + 0x40, 0);
    e.call(SET_UP_ARRAYS, &args![object, interpolators, links]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a3_6070, fn_00a36070(Ptr, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_target_constructor_clears_the_arrays_then_sets_them_up() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |e, a| {
            for offset in (0x34..0x44).step_by(4) {
                e.mem.set_u32(a[0] + offset, 0xffff_ffff);
            }
            Ret::default()
        });
        e.register(SET_UP_ARRAYS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x44);
        assert_eq!(e.call(0x00a3_6070, &args![this, 2u32, 5u32]).u32(), this);
        assert_eq!(e.mem.u32(this), MULTI_TARGET_VTABLE);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(e.mem.u16(this + 0x38), 0);
        assert_eq!(e.mem.u16(this + 0x40), 0);
        let log = e.call_log.take().unwrap();
        let set_up: Vec<_> = log.iter().filter(|call| call.0 == SET_UP_ARRAYS).collect();
        assert_eq!(set_up.len(), 1);
        assert_eq!(set_up[0].1, vec![this, 2, 5]);
    }
}
