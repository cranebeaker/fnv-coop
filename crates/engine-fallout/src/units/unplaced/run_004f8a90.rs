//! `(unplaced)/run_004f8a90` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f8b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f8b00 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x58 (0x60 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0102_4214`) is `BGSAddonNode`, so this is the constructor of `BGSAddonNode`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004f8b00(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_a3d0, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_4214);
    e.mem.set_u32(base + 0x30, 0x0102_41f0);
    e.mem.set_u32(base + 0x50, 0xffff_ffff);
    e.call(0x004f_15a0, &args![this, 0x58u32]);
    e.call(0x004f_8c50, &args![this]);
    this
}

// Translated from 004f8a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its two words to `00853130` on the member at `+0x10C` (the
/// object's class is not confirmed) and answers its byte: the exe ends on
/// that call and leaves its `EAX`, whose `AL` the callers keep.
pub fn fn_004f8a90(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u8 {
    e.call(
        0x0085_3130,
        &args![this.addr().wrapping_add(0x10c), first, second],
    )
    .u8()
}

// Translated from 004f8ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the string `text` into the 0x104-byte buffer at `+4`
/// (`00406d30`, cdecl `(buffer, size, text)`), or empties the buffer when
/// `text` is null (the object's class is not confirmed).
pub fn fn_004f8ac0(e: &mut Engine, this: Ptr, text: u32) {
    let buffer = this.addr().wrapping_add(4);
    if text != 0 {
        e.call(0x0040_6d30, &args![buffer, 0x104u32, text]);
    } else {
        e.mem.set_u8(buffer, 0);
    }
}

// Translated from 004f8b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAddonNode`'s scalar deleting destructor (the class by the vtable
/// its destructor `004f8bc0` stores): runs the destructor and frees `this`
/// with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004f8b90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_004f8bc0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004f8bc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f8bc0 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// `BGSAddonNode`'s destructor (its vtables `01024214` and, at `+0x30`,
/// `010241f0`): puts back both vtables, tells the object at `011c3f2c`
/// about it when there is one (`00461930`), then destroys the component at
/// `+0x30` (`0048a490`) and the base (`0050e9f0`).
pub fn fn_004f8bc0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_4214);
    e.mem.set_u32(base.wrapping_add(0x30), 0x0102_41f0);
    let handler: u32 = e.global(0x011c_3f2c);
    if handler != 0 {
        e.call(0x0046_1930, &args![handler, this]);
    }
    e.call(0x0048_a490, &args![base.wrapping_add(0x30)]);
    e.call(0x0050_e9f0, &args![this]);
}

// Translated from 004f8c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSAddonNode`'s field initializer (called by its constructor
/// `004f8b00`): clears the word at `+0x54`, the 4 bytes at `+0x58`
/// (`00403d30`, cdecl `(buffer, 0, 4)`) and the word at `+0x5C`.
pub fn fn_004f8c50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base.wrapping_add(0x54), 0);
    e.call(0x0040_3d30, &args![base.wrapping_add(0x58), 0u32, 4u32]);
    e.mem.set_u32(base.wrapping_add(0x5c), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_8b00, fn_004f8b00(Ptr) -> Ptr),
        entry!(0x004f_8a90, fn_004f8a90(Ptr, u32, u32) -> u8),
        entry!(0x004f_8ac0, fn_004f8ac0(Ptr, u32)),
        entry!(0x004f_8b90, fn_004f8b90(Ptr, u32) -> Ptr),
        entry!(0x004f_8bc0, fn_004f8bc0(Ptr)),
        entry!(0x004f_8c50, fn_004f8c50(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004f8b00_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_8b00,
            size: 0x60,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_a3d0, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x58)]),
                (0x004f_8c50, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_4214),
                (0x30, 4, 0x0102_41f0),
                (0x50, 4, 0xffff_ffff),
            ],
        });
    }

    fn calls_to(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == target)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_004f8a90_forwards_to_the_member() {
        let mut e = Engine::new();
        e.register(0x0085_3130, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_8a90, &args![0x1000u32, 7u32, 9u32]);
        assert_eq!(calls_to(&e, 0x0085_3130), vec![vec![0x110c, 7, 9]]);
    }

    #[test]
    fn fn_004f8ac0_copies_or_empties_the_buffer() {
        let mut e = Engine::new();
        e.register(0x0040_6d30, |_, _| Ret::default());
        let this = e.mem.alloc(0x108);
        e.mem.set_u8(this + 4, b'x');
        e.call_log = Some(vec![]);
        e.call(0x004f_8ac0, &args![this, 0x5000u32]);
        assert_eq!(
            calls_to(&e, 0x0040_6d30),
            vec![vec![this + 4, 0x104, 0x5000]]
        );
        assert_eq!(e.mem.u8(this + 4), b'x');
        e.call(0x004f_8ac0, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 4), 0);
        assert_eq!(calls_to(&e, 0x0040_6d30).len(), 1);
    }

    #[test]
    fn fn_004f8b90_frees_only_with_flag_bit_zero() {
        let mut e = Engine::new();
        for target in [0x0048_a490, 0x0050_e9f0, 0x0040_1030] {
            e.register(target, |_, _| Ret::default());
        }
        e.map(0x011c_3000, 0x1000);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_8b90, &args![this, 0u32]).u32(), this);
        assert_eq!(e.call(0x004f_8b90, &args![this, 3u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x0040_1030), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x0050_e9f0).len(), 2);
    }

    #[test]
    fn fn_004f8bc0_restores_vtables_and_destroys_parts() {
        let mut e = Engine::new();
        for target in [0x0048_a490, 0x0050_e9f0, 0x0046_1930] {
            e.register(target, |_, _| Ret::default());
        }
        e.map(0x011c_3000, 0x1000);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x004f_8bc0, &args![this]);
        assert!(calls_to(&e, 0x0046_1930).is_empty());
        e.set_global(0x011c_3f2c, 0x4444u32);
        e.call(0x004f_8bc0, &args![this]);
        assert_eq!(calls_to(&e, 0x0046_1930), vec![vec![0x4444, this]]);
        assert_eq!(e.mem.u32(this), 0x0102_4214);
        assert_eq!(e.mem.u32(this + 0x30), 0x0102_41f0);
        assert_eq!(calls_to(&e, 0x0048_a490), vec![vec![this + 0x30]; 2]);
        assert_eq!(calls_to(&e, 0x0050_e9f0), vec![vec![this]; 2]);
    }

    #[test]
    fn fn_004f8c50_clears_its_fields() {
        let mut e = Engine::new();
        e.register(0x0040_3d30, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x54, 1);
        e.mem.set_u32(this + 0x5c, 2);
        e.call_log = Some(vec![]);
        e.call(0x004f_8c50, &args![this]);
        assert_eq!(e.mem.u32(this + 0x54), 0);
        assert_eq!(e.mem.u32(this + 0x5c), 0);
        assert_eq!(calls_to(&e, 0x0040_3d30), vec![vec![this + 0x58, 0, 4]]);
    }

    #[test]
    fn fn_004f8a90_answers_the_lookups_byte() {
        let mut e = Engine::new();
        e.register(0x0085_3130, |_, _| 1u8.into_ret());
        assert_eq!(e.call(0x004f_8a90, &args![0x1000u32, 7u32, 9u32]).u8(), 1);
    }
}
