//! `BSMain/run_00c5fc70` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The pooled `NiFixedString` maker (`00a5b690`, `cdecl`: text) and the empty handle
/// (`0109b220`, never counted).
const FIXED_STRING_FROM_TEXT: u32 = 0x00a5_b690;
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiPSysModifier`'s constructor (`00c316a0`; `thiscall`, `RET 8`: address of the name
/// handle, order).
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_16a0;
/// The `BSPSysStripUpdateModifier` virtual table (`010c32c4`, by RTTI).
const STRIP_UPDATE_MODIFIER_VTABLE: u32 = 0x010c_32c4;

// Translated from 00c5fc70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5fc70 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2ff98, state variable) is not translated
/// The `BSPSysStripUpdateModifier` constructor (by its virtual table; unnamed in the engine
/// map; `thiscall`, `RET 8`: name text, a `float`). The name becomes a pooled handle in a
/// stack local (`00a5b690`; the empty handle for a null text), `NiPSysModifier`'s
/// constructor (`00c316a0`) gets its address and order 2500, then the handle is released
/// (`InterlockedDecrement` of the count 8 bytes before it, unless it is the empty one). Then
/// the class's virtual table and the `float` at +0x18 (copied through the x87 stack). Answers
/// `this`.
pub fn fn_00c5fc70(e: &mut Engine, this: Ptr, name: u32, value: f32) -> Ptr {
    e.with_stack(4, |e, slot| {
        let handle = if name == 0 {
            e.mem.u32(FIXED_STRING_EMPTY)
        } else {
            e.call(FIXED_STRING_FROM_TEXT, &args![name]).u32()
        };
        e.mem.set_u32(slot.addr(), handle);
        e.call(MODIFIER_CONSTRUCTOR, &args![this, slot, 0x9c4u32]);
        let handle = e.mem.u32(slot.addr());
        if handle != e.mem.u32(FIXED_STRING_EMPTY) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    });
    e.mem.set_u32(this.addr(), STRIP_UPDATE_MODIFIER_VTABLE);
    e.mem
        .set_u32(this.addr() + 0x18, x87_quiet_bits(value.to_bits()));
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c5_fc70, fn_00c5fc70(Ptr, u32, f32) -> Ptr)]
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn the_name_is_pooled_for_the_base_and_released() {
        let mut e = Engine::new();
        e.map(FIXED_STRING_EMPTY, 4);
        e.mem.set_u32(FIXED_STRING_EMPTY, 0x0500_0008);
        e.register(FIXED_STRING_FROM_TEXT, |_, _| 0x0600_0008u32.into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let inner = seen.clone();
        e.register_double(MODIFIER_CONSTRUCTOR, move |e, a| {
            inner.borrow_mut().push((e.mem.u32(a[1]), a[2]));
            Ret::default()
        });
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        let result = e.call(0x00c5_fc70, &args![this, 0x0700_0000u32, 0x7f80_0001u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(result.u32(), this);
        assert_eq!(*seen.borrow(), vec![(0x0600_0008, 2500)]);
        assert!(log.contains(&(INTERLOCKED_DECREMENT, vec![0x0600_0000])));
        assert_eq!(e.mem.u32(this), STRIP_UPDATE_MODIFIER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), 0x7fc0_0001);
        // A null name uses the empty handle, which is not released.
        e.call_log = Some(vec![]);
        e.call(0x00c5_fc70, &args![this, 0u32, 1.5f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(seen.borrow()[1], (0x0500_0008, 2500));
        assert!(!log.iter().any(|c| c.0 == INTERLOCKED_DECREMENT));
        assert_eq!(e.mem.f32(this + 0x18), 1.5);
    }
}
