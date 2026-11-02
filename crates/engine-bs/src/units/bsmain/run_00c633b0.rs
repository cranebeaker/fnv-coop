//! `BSMain/run_00c633b0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
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
/// The `BSParentVelocityModifier` virtual table (`010c37fc`, by RTTI).
const PARENT_VELOCITY_MODIFIER_VTABLE: u32 = 0x010c_37fc;
/// The global `NiPoint3` copied as the zero point (`011f426c`).
const ZERO_POINT: u32 = 0x011f_426c;

// Translated from 00c633b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c633b0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f303a8, state variable) is not translated
/// The `BSParentVelocityModifier` constructor (by its virtual table; unnamed in the engine
/// map; `thiscall`, `RET 4`: name text). The name becomes a pooled handle in a stack local
/// (`00a5b690`; the empty handle for a null text), `NiPSysModifier`'s constructor
/// (`00c316a0`) gets its address and order 3000, then the handle is released (unless it is
/// the empty one). Then the class's virtual table, the zero point (`011f426c`) at +0x24 and
/// +0x30, and 0.0 at +0x20 and +0x18. Answers `this`.
pub fn fn_00c633b0(e: &mut Engine, this: Ptr, name: u32) -> Ptr {
    e.with_stack(4, |e, slot| {
        let handle = if name == 0 {
            e.mem.u32(FIXED_STRING_EMPTY)
        } else {
            e.call(FIXED_STRING_FROM_TEXT, &args![name]).u32()
        };
        e.mem.set_u32(slot.addr(), handle);
        e.call(MODIFIER_CONSTRUCTOR, &args![this, slot, 0xbb8u32]);
        let handle = e.mem.u32(slot.addr());
        if handle != e.mem.u32(FIXED_STRING_EMPTY) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    });
    let address = this.addr();
    e.mem.set_u32(address, PARENT_VELOCITY_MODIFIER_VTABLE);
    for offset in [0x24u32, 0x30] {
        for k in 0..3u32 {
            let word = e.mem.u32(ZERO_POINT + 4 * k);
            e.mem.set_u32(address + offset + 4 * k, word);
        }
    }
    e.mem.set_f32(address + 0x20, 0.0);
    e.mem.set_f32(address + 0x18, 0.0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c6_33b0, fn_00c633b0(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn the_modifier_starts_still() {
        let mut e = Engine::new();
        e.map(FIXED_STRING_EMPTY, 4);
        e.mem.set_u32(FIXED_STRING_EMPTY, 0x0500_0008);
        e.map(ZERO_POINT, 12);
        e.mem.set_f32(ZERO_POINT + 4, 0.25);
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x3c);
        e.mem.write(this, &[0xaa; 0x3c]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c6_33b0, &args![this, 0u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|c| c.0 == MODIFIER_CONSTRUCTOR && c.1[2] == 3000));
        assert!(!log.iter().any(|c| c.0 == INTERLOCKED_DECREMENT));
        assert_eq!(e.mem.u32(this), PARENT_VELOCITY_MODIFIER_VTABLE);
        assert_eq!(e.mem.f32(this + 0x28), 0.25);
        assert_eq!(e.mem.f32(this + 0x34), 0.25);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0xaaaa_aaaa);
    }
}
