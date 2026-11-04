//! `(unplaced)/run_007123a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A byte flag at +0x60 and a float at +0x5C of an object the engine map
//! does not name, and two helpers that reach that object as entry 15 of
//! their `this` through `004ebbc0` (a getter the decompiler mislabels with a
//! library name; it returns the word at the address `00877a30` gives for
//! the index). `InterfaceManager`'s `AddToEnterStack` and `00711ea0` call
//! them.

#[allow(unused_imports)]
use crate::prelude::*;

/// `004ebbc0` (engine map: `teswater.cpp`): the object stored for an
/// index (thiscall, one word).
const ENTRY_FOR_INDEX: u32 = 0x004e_bbc0;
/// The index both helpers pass.
const ENTRY_INDEX: u32 = 0xf;
/// The flag that enables the float (+0x60) and the float (+0x5C).
const FLAG_OFFSET: u32 = 0x60;
const VALUE_OFFSET: u32 = 0x5c;

// Translated from 007123a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the +0x60 flag of entry 15 (`fn_007123d0`).
pub fn fn_007123a0(e: &mut Engine, this: Ptr, flag: u8) {
    let entry = e.call(ENTRY_FOR_INDEX, &args![this, ENTRY_INDEX]).u32();
    fn_007123d0(e, Ptr::new(entry), flag);
}

// Translated from 007123d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte flag at +0x60.
pub fn fn_007123d0(e: &mut Engine, this: Ptr, flag: u8) {
    e.mem.set_u8(this.addr() + FLAG_OFFSET, flag);
}

// Translated from 007123f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the float of entry 15 (`fn_00712420`); the value is copied through
/// the x87 stack on the way.
pub fn fn_007123f0(e: &mut Engine, this: Ptr, value: f32) {
    let entry = e.call(ENTRY_FOR_INDEX, &args![this, ENTRY_INDEX]).u32();
    fn_00712420(e, Ptr::new(entry), x87_quiet(value));
}

// Translated from 00712420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at +0x5C (through the x87 stack) when the +0x60 flag is
/// set.
pub fn fn_00712420(e: &mut Engine, this: Ptr, value: f32) {
    if e.mem.u8(this.addr() + FLAG_OFFSET) != 0 {
        e.mem
            .set_u32(this.addr() + VALUE_OFFSET, x87_quiet_bits(value.to_bits()));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007123a0, fn_007123a0(Ptr, u8)),
        entry!(0x007123d0, fn_007123d0(Ptr, u8)),
        entry!(0x007123f0, fn_007123f0(Ptr, f32)),
        entry!(0x00712420, fn_00712420(Ptr, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The entry getter returns the word at `this + 4 * index` here.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ENTRY_FOR_INDEX, |e, a| {
            e.mem.u32(a[0] + 4 * a[1]).into_ret()
        });
        e
    }

    fn holder(e: &mut Engine) -> (Ptr, Ptr) {
        let holder: Ptr = Ptr::new(e.mem.alloc(0x40));
        let entry: Ptr = Ptr::new(e.mem.alloc(0x64));
        e.mem.set_u32(holder.addr() + 4 * ENTRY_INDEX, entry.addr());
        (holder, entry)
    }

    #[test]
    fn set_flag_through_entry_15() {
        let mut e = engine();
        let (holder, entry) = holder(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0071_23a0, &args![holder, 1u8]);
        let log = e.call_log.take().unwrap();
        let getter = log.iter().find(|(a, _)| *a == ENTRY_FOR_INDEX).unwrap();
        assert_eq!(getter.1, vec![holder.addr(), 15]);
        assert_eq!(e.mem.u8(entry.addr() + 0x60), 1);
    }

    #[test]
    fn set_flag_stores_the_byte() {
        let mut e = engine();
        let object: Ptr = Ptr::new(e.mem.alloc(0x64));
        e.call(0x0071_23d0, &args![object, 0x5au8]);
        assert_eq!(e.mem.u8(object.addr() + 0x60), 0x5a);
    }

    #[test]
    fn set_value_through_entry_15_quiets_a_signalling_nan() {
        let mut e = engine();
        let (holder, entry) = holder(&mut e);
        e.mem.set_u8(entry.addr() + 0x60, 1);
        e.call(0x0071_23f0, &args![holder, 2.5f32]);
        assert_eq!(e.mem.f32(entry.addr() + 0x5c), 2.5);
        e.call(0x0071_23f0, &args![holder, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(entry.addr() + 0x5c), 0x7fc0_0001);
    }

    #[test]
    fn set_value_only_with_the_flag() {
        let mut e = engine();
        let object: Ptr = Ptr::new(e.mem.alloc(0x64));
        e.call(0x0071_2420, &args![object, 3.0f32]);
        assert_eq!(e.mem.u32(object.addr() + 0x5c), 0);
        e.mem.set_u8(object.addr() + 0x60, 1);
        e.call(0x0071_2420, &args![object, 3.0f32]);
        assert_eq!(e.mem.f32(object.addr() + 0x5c), 3.0);
    }
}
