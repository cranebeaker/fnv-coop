//! `fallout shared/run_004fb970` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Handles the object when the byte of the other object is clear
/// (`0050fbf0`, `thiscall`, one stack word; the engine map puts it in
/// `tesobject.cpp`).
const HANDLE_OBJECT: u32 = 0x0050_fbf0;
/// The global pointer whose `+0x3c` is an embedded object with a vtable.
const EMBEDDED_OWNER: u32 = 0x011c_a284;
/// The global pointer that receives the embedded object's answer.
const ANSWER_TARGET: u32 = 0x011c_3b3c;
/// Takes the answer (`0045a5e0`, `thiscall`, one stack word).
const TAKE_ANSWER: u32 = 0x0045_a5e0;
/// The scalar deleting destructor the thunk `004fb9f0` jumps to
/// (`004facf0`, `RET 4`: the flags).
const SCALAR_DELETING_DESTRUCTOR: u32 = 0x004f_acf0;
/// Destroys the component at `+0x14` (`0051b200`).
const DESTROY_COMPONENT: u32 = 0x0051_b200;
/// Destroys the component at `+0x0c` (`00475380`).
const DESTROY_REFERENCE_COMPONENT: u32 = 0x0047_5380;
/// The base destructor body (`00591300`).
const DESTROY_BASE: u32 = 0x0059_1300;

// Translated from 004fb970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0xd9` of `this`. `__fastcall`, no stack words. The engine
/// map has no name for it.
pub fn fn_004fb970(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xd9)
}

// Translated from 004fb990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dispatches on the byte at `+0xd9` of `other` (`004fb970`): when it is
/// clear, `0050fbf0` handles `other` on `this`; otherwise the object at
/// `+0x3c` of the global owner (`0x011ca284`) is asked through its vtable
/// slot `0x14` (no arguments) and the answer is handed to the global target
/// (`0x011c3b3c`) through `0045a5e0`. `__thiscall`, one stack word.
pub fn fn_004fb990(e: &mut Engine, this: Ptr, other: Ptr) {
    if fn_004fb970(e, other) == 0 {
        e.call(HANDLE_OBJECT, &args![this, other]);
        return;
    }
    let owner = e.global::<u32>(EMBEDDED_OWNER);
    let answer = e.vcall(owner + 0x3c, 0x14, &args![]).u32();
    let target = e.global::<u32>(ANSWER_TARGET);
    e.call(TAKE_ANSWER, &args![target, answer]);
}

// Translated from 004fb9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: the scalar deleting destructor `004facf0` for an object whose
/// secondary base sits at `+0x3c` (`this` is moved back by `0x3c` and the
/// flags word is passed on). Returns what it returns (the object).
pub fn fn_004fb9f0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(
        SCALAR_DELETING_DESTRUCTOR,
        &args![this.addr().wrapping_sub(0x3c), flags],
    )
    .u32()
}

// Translated from 004fba00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fba00 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x22 (0x74 bytes); `ECX` is the object.
/// The vtable it stores at `+0` (`0x01024e84`) has the RTTI name `BGSMovableStatic`, so this is that class's constructor.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004fba00(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_2d00, &args![this]);
    e.call(0x0047_5350, &args![base + 0xc]);
    e.call(0x0051_b140, &args![base + 0x14]);
    e.mem.set_u32(base, 0x0102_4e84);
    e.mem.set_u32(base + 0xc, 0x0102_4e70);
    e.mem.set_u32(base + 0x14, 0x0102_4cec);
    e.mem.set_u32(base + 0x44, 0x0102_4cc8);
    e.call(0x0086_4490, &args![base + 0x70]);
    e.call(0x004f_15a0, &args![base + 0x14, 0x22]);
    e.call(0x004f_bb50, &args![base + 0x14]);
    this
}

// Translated from 004fbac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fbac0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Destructor body of `BGSMovableStatic` (the RTTI name of the vtable stored
/// at `+0`): puts the vtables back (`+0`, `+0x0c`, `+0x14`, `+0x44`), then
/// destroys the component at `+0x14` (`0051b200`) and at `+0x0c`
/// (`00475380`) and runs the base destructor body (`00591300`). The
/// exception-unwinding frame is not translated.
pub fn fn_004fbac0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_4e84);
    e.mem.set_u32(base + 0xc, 0x0102_4e70);
    e.mem.set_u32(base + 0x14, 0x0102_4cec);
    e.mem.set_u32(base + 0x44, 0x0102_4cc8);
    e.call(DESTROY_COMPONENT, &args![base + 0x14]);
    e.call(DESTROY_REFERENCE_COMPONENT, &args![base + 0xc]);
    e.call(DESTROY_BASE, &args![this]);
}

// Translated from 004fbb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at `+0x58` of `this`. `__fastcall`.
pub fn fn_004fbb50(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x58, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004fb970, fn_004fb970(Ptr) -> u8),
        entry!(0x004fb990, fn_004fb990(Ptr, Ptr)),
        entry!(0x004fb9f0, fn_004fb9f0(Ptr, u32) -> u32),
        entry!(0x004fba00, fn_004fba00(Ptr) -> Ptr),
        entry!(0x004fbac0, fn_004fbac0(Ptr)),
        entry!(0x004fbb50, fn_004fbb50(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_004fba00_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_ba00,
            size: 0x74,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_2d00, &[A::T(0x0)]),
                (0x0047_5350, &[A::T(0xc)]),
                (0x0051_b140, &[A::T(0x14)]),
                (0x0086_4490, &[A::T(0x70)]),
                (0x004f_15a0, &[A::T(0x14), A::V(0x22)]),
                (0x004f_bb50, &[A::T(0x14)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_4e84),
                (0xc, 4, 0x0102_4e70),
                (0x14, 4, 0x0102_4cec),
                (0x44, 4, 0x0102_4cc8),
            ],
        });
    }

    #[test]
    fn the_flag_byte_is_read_from_0xd9() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x100);
        assert_eq!(e.call(0x004f_b970, &args![Ptr::<()>::new(object)]).u8(), 0);
        e.mem.set_u8(object + 0xd9, 7);
        assert_eq!(e.call(0x004f_b970, &args![Ptr::<()>::new(object)]).u8(), 7);
    }

    #[test]
    fn a_clear_byte_hands_the_object_to_the_handler() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x10));
        let other: Ptr = Ptr::new(e.mem.alloc(0x100));
        e.register(HANDLE_OBJECT, |_, a| ret(a[0] ^ a[1]));
        e.call_log = Some(vec![]);
        e.call(0x004f_b990, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (HANDLE_OBJECT, vec![this.addr(), other.addr()]));
    }

    #[test]
    fn a_set_byte_asks_the_owner_and_passes_the_answer_on() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x10));
        let other: Ptr = Ptr::new(e.mem.alloc(0x100));
        e.mem.set_u8(other.addr() + 0xd9, 1);
        // The owner holds an embedded object at +0x3c whose vtable slot 0x14
        // answers 0x4242.
        let owner = e.mem.alloc(0x80);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0x3c, vtable);
        e.mem.set_u32(vtable + 0x14, 0x0900_0014);
        e.map(EMBEDDED_OWNER, 4);
        e.map(ANSWER_TARGET, 4);
        e.mem.set_u32(EMBEDDED_OWNER, owner);
        e.mem.set_u32(ANSWER_TARGET, 0x7777);
        e.register(0x0900_0014, |_, a| {
            ret(if a.len() == 1 { 0x4242 } else { 0 })
        });
        e.register(TAKE_ANSWER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_b990, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0900_0014, vec![owner + 0x3c]));
        assert_eq!(log[2], (TAKE_ANSWER, vec![0x7777, 0x4242]));
        assert!(!log.iter().any(|(a, _)| *a == HANDLE_OBJECT));
    }

    #[test]
    fn the_thunk_moves_this_back_and_passes_the_flags() {
        let mut e = Engine::new();
        e.register(SCALAR_DELETING_DESTRUCTOR, |_, a| ret(a[0] + a[1]));
        let result = e.call(0x004f_b9f0, &args![Ptr::<()>::new(0x1000), 3u32]);
        assert_eq!(result.u32(), 0x1000 - 0x3c + 3);
    }

    #[test]
    fn the_destructor_restores_the_vtables_and_destroys_the_components() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x74);
        for address in [DESTROY_COMPONENT, DESTROY_REFERENCE_COMPONENT, DESTROY_BASE] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x004f_bac0, &args![Ptr::<()>::new(object)]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (DESTROY_COMPONENT, vec![object + 0x14]),
                (DESTROY_REFERENCE_COMPONENT, vec![object + 0xc]),
                (DESTROY_BASE, vec![object]),
            ]
        );
        assert_eq!(e.mem.u32(object), 0x0102_4e84);
        assert_eq!(e.mem.u32(object + 0xc), 0x0102_4e70);
        assert_eq!(e.mem.u32(object + 0x14), 0x0102_4cec);
        assert_eq!(e.mem.u32(object + 0x44), 0x0102_4cc8);
    }

    #[test]
    fn the_word_at_0x58_is_cleared() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x74);
        e.mem.set_u32(object + 0x58, 99);
        e.call(0x004f_bb50, &args![Ptr::<()>::new(object)]);
        assert_eq!(e.mem.u32(object + 0x58), 0);
    }
}
