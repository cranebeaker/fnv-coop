//! `fallout shared/havok/run_00624070` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006815c0`: thiscall on the same object.
const INITIALISE: u32 = 0x0068_15c0;

// Translated from 006240d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall`: runs `006815c0` on `this` and returns `this`.
pub fn fn_006240d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(INITIALISE, &args![this]);
    this
}

/// `0043b4d0` (unnamed in the engine map): `thiscall`, plain `RET`; the layer of a filter
/// record.
const LAYER_OF: u32 = 0x0043_b4d0;
/// `004d9da0` (unnamed in the engine map): `thiscall`, plain `RET`.
const KIND_OF: u32 = 0x004d_9da0;

// Translated from 00624070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: whether the record's layer (`0043b4d0`) is 8 or 0x1d.
pub fn fn_00624070(e: &mut Engine, this: Ptr) -> bool {
    let layer = e.call(LAYER_OF, &args![this]).u32();
    layer == 8 || layer == 0x1d
}

// Translated from 006240b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: answers what `004d9da0` answers for `this` (left in
/// `EAX`).
pub fn fn_006240b0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(KIND_OF, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006240d0, fn_006240d0(Ptr) -> Ptr),
        entry!(0x0062_4070, fn_00624070(Ptr) -> bool),
        entry!(0x0062_40b0, fn_006240b0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod record_tests {
    use super::*;

    #[test]
    fn at_00624070_accepts_layers_8_and_0x1d() {
        let mut e = Engine::new();
        e.register(LAYER_OF, |e, a| e.mem.u32(a[0]).into_ret());
        let record = e.mem.alloc(4);
        for (layer, expected) in [(8u32, true), (0x1d, true), (7, false), (0x1e, false)] {
            e.mem.set_u32(record, layer);
            assert_eq!(e.call(0x0062_4070, &args![record]).bool(), expected);
        }
    }

    #[test]
    fn at_006240b0_answers_the_kind() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(KIND_OF, |_, _| 0x16u32.into_ret());
        assert_eq!(e.call(0x0062_40b0, &args![0x4000u32]).u32(), 0x16);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|(a, w)| *a == KIND_OF && w == &vec![0x4000]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calls_006815c0_on_this_and_returns_this() {
        let mut e = Engine::new();
        e.register(INITIALISE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = Ptr::<()>::new(0x4000);
        assert_eq!(e.call(0x0062_40d0, &args![this]).ptr::<()>(), this);
        let log = e.call_log.take().unwrap();
        let inner = log.iter().find(|(addr, _)| *addr == INITIALISE).unwrap();
        assert_eq!(inner.1, vec![0x4000]);
    }
}
