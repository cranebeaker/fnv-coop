//! `fallout shared/run_00680890` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The two-word list node constructor (`0096a2d0`: item and next, both 0).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// The two calls of `006808b0`, both on `this`.
const CLEAR_ITEMS: u32 = 0x0068_0fe0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;

// Translated from 00680890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a condition list (the engine map has no name for it; the
/// decompiler's `QuickBitSet` name for the callee is a folded library name):
/// a forwarder to `0096a2d0`. Returns `this`.
pub fn fn_00680890(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(LIST_CONSTRUCT, &args![this]);
    this
}

// Translated from 006808b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor-shaped clean-up of a condition list (the decompiler's
/// `CAtlWinModule` name is a folded library name): runs `00680fe0` on
/// `this`, then `0046ffb0`. The quest stage item's destructor (`0060f5f0`)
/// runs it on the item's condition list (`+4`).
// Translation incomplete: 006808b0 - SEH exception unwinding (FS:[0] frame state) is not translated.
pub fn fn_006808b0(e: &mut Engine, this: Ptr) {
    e.call(CLEAR_ITEMS, &args![this]);
    e.call(LIST_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00680890, fn_00680890(Ptr) -> Ptr),
        entry!(0x006808b0, fn_006808b0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwards_to_the_list_constructor_and_returns_this() {
        let mut e = Engine::new();
        e.register(LIST_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let back = e.call(0x0068_0890, &args![0x4000u32]).u32();
        assert_eq!(back, 0x4000);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (LIST_CONSTRUCT, vec![0x4000]));
    }

    #[test]
    fn destructor_clears_then_destroys_the_list() {
        let mut e = Engine::new();
        e.register(CLEAR_ITEMS, |_, _| Ret::default());
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0068_08b0, &args![0x4000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(CLEAR_ITEMS, vec![0x4000]), (LIST_DESTRUCT, vec![0x4000])]
        );
    }
}
