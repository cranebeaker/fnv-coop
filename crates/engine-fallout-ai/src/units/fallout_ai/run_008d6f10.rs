//! `fallout/ai/run_008d6f10` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d6f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `+0x40` of `this` (a plain field getter).
pub fn fn_008d6f30(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x40)
}

/// The `thiscall` test on the member at +0xA0 (`00822510`, with 0, `bool`
/// in `AL`), the set-up `008d6bb0` (`thiscall (this, object)`), the
/// culling-process member's two setters (`0041fd00` and `00a694a0`,
/// `thiscall (member, value)`), the value read from the object (`0045bbe0`,
/// `thiscall`), and the registration with the list at `011e0e80`
/// (`0096bbc0`, `thiscall (list, this)`).
const MEMBER_TEST: u32 = 0x0082_2510;
const SET_UP: u32 = 0x008d_6bb0;
const FIRST_SETTER: u32 = 0x0041_fd00;
const OBJECT_VALUE: u32 = 0x0045_bbe0;
const SECOND_SETTER: u32 = 0x00a6_94a0;
const REGISTER: u32 = 0x0096_bbc0;
const REGISTER_LIST: u32 = 0x011e_0e80;

// Translated from 008d6f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands `object` to this (the engine map has no name): when
/// `00822510` on the member at +0xA0 (with 0) answers true, the full set-up
/// `008d6bb0` runs; otherwise only the member at +8 is given the object (`0041fd00`)
/// and the object's value (`0045bbe0`, `00a694a0`). Either way `this` is
/// registered with the list at `011e0e80`.
pub fn fn_008d6f50(e: &mut Engine, this: Ptr, object: u32) {
    if e.call(MEMBER_TEST, &args![this.addr() + 0xa0, 0u32]).bool() {
        e.call(SET_UP, &args![this, object]);
    } else {
        e.call(FIRST_SETTER, &args![this.addr() + 8, object]);
        let value = e.call(OBJECT_VALUE, &args![object]).u32();
        e.call(SECOND_SETTER, &args![this.addr() + 8, value]);
    }
    e.call(REGISTER, &args![REGISTER_LIST, this]);
}

// Translated from 008d6f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0043b370` on `this` with the low byte of `flag` and `0x100000`.
pub fn fn_008d6f10(e: &mut Engine, this: Ptr, flag: u32) {
    e.call(0x0043_b370, &args![this, flag & 0xff, 0x0010_0000u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_6f10, fn_008d6f10(Ptr, u32)),
        entry!(0x008d6f30, fn_008d6f30(Ptr) -> u32),
        entry!(0x008d_6f50, fn_008d6f50(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_field_at_offset_0x40() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x50));
        e.mem.set_u32(object.addr() + 0x40, 0x1234_5678);
        assert_eq!(e.call(0x008d_6f30, &args![object]).u32(), 0x1234_5678);
    }

    #[test]
    fn handing_an_object_sets_up_or_sets_the_member_then_registers() {
        let mut e = Engine::new();
        for addr in [SET_UP, FIRST_SETTER, SECOND_SETTER, REGISTER] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(OBJECT_VALUE, |_, a| (a[0] + 1).into_ret());
        let this = e.mem.alloc(0xa4);
        e.register(MEMBER_TEST, |_, _| 0x100u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008d_6f50, &args![this, 0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (MEMBER_TEST, vec![this + 0xa0, 0]),
                (FIRST_SETTER, vec![this + 8, 0x5000]),
                (OBJECT_VALUE, vec![0x5000]),
                (SECOND_SETTER, vec![this + 8, 0x5001]),
                (REGISTER, vec![REGISTER_LIST, this])
            ]
        );
        e.register(MEMBER_TEST, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x008d_6f50, &args![this, 0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[2], (SET_UP, vec![this, 0x5000]));
        assert_eq!(log.len(), 4);
    }
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    #[test]
    fn fn_008d6f10_passes_the_byte_and_the_mask() {
        let mut e = Engine::new();
        e.register(0x0043_b370, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008d_6f10, &[0x9000, 0x0301]);
        let log = e.call_log.as_ref().unwrap();
        assert_eq!(log[1], (0x0043_b370, vec![0x9000, 1, 0x0010_0000]));
    }
}
