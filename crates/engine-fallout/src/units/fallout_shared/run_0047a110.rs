//! `fallout shared/run_0047a110` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0047a130 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0047a130 - the exception-unwinding frame (FS:[0]) is not translated.
/// The constructor of the form component `BGSMessageIcon` (the engine map
/// gives it no name; the RTTI name of the vtable it stores, `0101ab58`, is
/// `.?AVBGSMessageIcon@@`): runs the base constructor (`00402d30`), stores
/// the vtable at `+0`, runs `00402c20` on the member at `+4`, then `0047a200`
/// on the component. Returns `this`. The exception-unwinding frame is not
/// translated.
pub fn fn_0047a130(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_ab58);
    e.call(0x0040_2c20, &args![this.byte_add(4)]);
    e.call(0x0047_a200, &args![this]);
    this
}

/// `00877a30`: the address of the value slot for a key (`thiscall`, one
/// word).
const VALUE_SLOT_00877A30: u32 = 0x0087_7a30;
/// `BGSMessageIcon::ClearDataComponent` (engine map), `thiscall`.
const MESSAGE_ICON_CLEAR: u32 = 0x0047_a220;
/// `00402c40`: the destructor of the member at `+4` (`thiscall`).
const MEMBER_DESTRUCTOR_00402C40: u32 = 0x0040_2c40;

// Translated from 0047a110 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: stores the word at `value` into the slot `00877a30` gives
/// for `key` (a map's set-at).
pub fn fn_0047a110(e: &mut Engine, this: Ptr, key: u32, value: Ptr) {
    let slot = e.call(VALUE_SLOT_00877A30, &args![this, key]).u32();
    let word = e.mem.u32(value.addr());
    e.mem.set_u32(slot, word);
}

// Translated from 0047a1a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0047a1a0 - the C++ exception frame (FS:[0]) is not translated
/// The `BGSMessageIcon` destructor (by the vtable it sets, `0101ab58`):
/// `BGSMessageIcon::ClearDataComponent`, then `00402c40` on the member at
/// `+4`.
pub fn fn_0047a1a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_ab58);
    e.call(MESSAGE_ICON_CLEAR, &args![this]);
    e.call(MEMBER_DESTRUCTOR_00402C40, &args![this.byte_add(4)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047a130, fn_0047a130(Ptr) -> Ptr),
        entry!(0x0047_a110, fn_0047a110(Ptr, u32, Ptr)),
        entry!(0x0047_a1a0, fn_0047a1a0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0047a130_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0047_a130,
            size: 0x20,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_2d30, &[A::T(0x0)]),
                (0x0040_2c20, &[A::T(0x4)]),
                (0x0047_a200, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0101_ab58)],
        });
    }
}

#[cfg(test)]
mod b040_tests {
    use super::*;

    #[test]
    fn set_at_writes_the_slot() {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        e.register_double(VALUE_SLOT_00877A30, move |_, a| {
            assert_eq!(a, &[0x100, 7]);
            slot.into_ret()
        });
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0xabcd);
        e.call(0x0047_a110, &args![0x100u32, 7u32, value]);
        assert_eq!(e.mem.u32(slot), 0xabcd);
    }

    #[test]
    fn message_icon_destructor_clears_and_destroys_the_member() {
        let mut e = Engine::new();
        e.register(MESSAGE_ICON_CLEAR, |_, _| Ret::default());
        e.register(MEMBER_DESTRUCTOR_00402C40, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0047_a1a0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0101_ab58);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (MESSAGE_ICON_CLEAR, vec![this]),
                (MEMBER_DESTRUCTOR_00402C40, vec![this + 4])
            ]
        );
    }
}
