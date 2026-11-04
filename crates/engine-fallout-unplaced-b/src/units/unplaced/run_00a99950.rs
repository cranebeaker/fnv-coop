//! `(unplaced)/run_00a99950` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiIntegersExtraData`'s vtable (its RTTI names the class).
const INTEGERS_EXTRA_DATA_VTABLE: u32 = 0x010a_0c7c;
/// The `NiExtraData` constructor and destructor (thiscall; the engine map
/// puts both in `NiMain/niextradata.cpp`).
const EXTRA_DATA_CONSTRUCTOR: u32 = 0x00a7_b2e0;
const EXTRA_DATA_DESTRUCTOR: u32 = 0x00a7_b300;
/// Sets the array (`00a99800`, thiscall: count, values).
const SET_ARRAY: u32 = 0x00a9_9800;
/// Frees a block through the Ni allocator (`00aa10f0`, cdecl; null is
/// ignored).
const NI_FREE: u32 = 0x00aa_10f0;

// Translated from 00a99950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `NiIntegersExtraData` (the class of the vtable it stores;
/// no caller or vtable entry in the exe): its vtable, the value array (+0x10)
/// freed through `00aa10f0` and cleared, then the `NiExtraData` destructor
/// `00a7b300`, whose EAX it passes on (a tail jump).
pub fn fn_00a99950(e: &mut Engine, this: u32) -> u32 {
    let values = e.mem.u32(this + 0x10);
    e.mem.set_u32(this, INTEGERS_EXTRA_DATA_VTABLE);
    e.call(NI_FREE, &[values]);
    e.mem.set_u32(this + 0x10, 0);
    e.call(EXTRA_DATA_DESTRUCTOR, &[this]).u32()
}

// Translated from 00a99980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiIntegersExtraData` from `count` values (called by
/// `00c61330`): the `NiExtraData` constructor `00a7b2e0`, its vtable, the
/// array word +0x10 cleared, then `00a99800(count, values)`. Returns `this`.
pub fn fn_00a99980(e: &mut Engine, this: u32, count: u32, values: u32) -> u32 {
    e.call(EXTRA_DATA_CONSTRUCTOR, &[this]);
    e.mem.set_u32(this, INTEGERS_EXTRA_DATA_VTABLE);
    e.mem.set_u32(this + 0x10, 0);
    e.call(SET_ARRAY, &[this, count, values]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9_9950, fn_00a99950(u32) -> u32),
        entry!(0x00a9_9980, fn_00a99980(u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructor_frees_the_values_and_chains() {
        let mut e = Engine::new();
        e.register(NI_FREE, |_, _| 0u32.into_ret());
        e.register(EXTRA_DATA_DESTRUCTOR, |_, _| 0x55u32.into_ret());
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, 0x5000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_9950, &[this]).u32(), 0x55);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(NI_FREE, vec![0x5000]), (EXTRA_DATA_DESTRUCTOR, vec![this])]
        );
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this), INTEGERS_EXTRA_DATA_VTABLE);
    }

    #[test]
    fn constructor_sets_the_array() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(SET_ARRAY, |e, a| {
            assert_eq!(e.mem.u32(a[0]), INTEGERS_EXTRA_DATA_VTABLE);
            assert_eq!(e.mem.u32(a[0] + 0x10), 0);
            0u32.into_ret()
        });
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_9980, &[this, 3, 0x6000]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (EXTRA_DATA_CONSTRUCTOR, vec![this]),
                (SET_ARRAY, vec![this, 3, 0x6000])
            ]
        );
    }
}
