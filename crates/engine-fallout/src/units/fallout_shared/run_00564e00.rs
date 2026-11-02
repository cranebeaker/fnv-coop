//! `fallout shared/run_00564e00` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The object has a word at `+8` (read by `0044ddc0`, written by
//! `00403550`), a pointer at `+0x20` (read by `007af430`) and an extra data
//! list at `+0x44` (`005d43c0`); bit `0x8000` of the word at `+8` of it and
//! of the object at `+0x20` is what some of these functions test. The class
//! is unconfirmed (the engine map has no name for any of them).

#[allow(unused_imports)]
use crate::prelude::*;

/// The pointer at `+0x20` (`007af430`; the engine map names the body
/// `BGSSaveFormBuffer::GetForm`, which may belong to other folded code).
const RELATED_OBJECT: u32 = 0x007a_f430;
/// The word at `+8` of the object (`0044ddc0`).
const GET_FLAGS: u32 = 0x0044_ddc0;
/// Stores the word at `+8` of the object (`00403550`, one stack word).
const SET_FLAGS: u32 = 0x0040_3550;
/// Finds the extra data of the type given (`00527080`, `thiscall`, one stack
/// word), or null.
const FIND_EXTRA: u32 = 0x0052_7080;
/// The extra data list of the object (`005d43c0`: `this + 0x44`; takes no
/// stack word).
const EXTRA_LIST: u32 = 0x005d_43c0;
/// Adds the extra data to the list (`0040ff60`, `thiscall`, one stack word;
/// engine map: `BaseExtraList::AddExtra`). The game's `005d43c0` call before
/// it leaves the extra on the stack, which this consumes.
const ADD_EXTRA: u32 = 0x0040_ff60;
/// Removes the extra data of the type given from the list (`00410140`,
/// `thiscall`, one stack word; engine map: `BaseExtraList::RemoveExtra_ov2`).
const REMOVE_EXTRA: u32 = 0x0041_0140;
/// The constructor of the 0x14-byte extra data (`00411e40`).
const EXTRA_CONSTRUCT: u32 = 0x0041_1e40;
/// `operator new` (`00401000`, `cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
/// Sets the flags of an extra data (`0041fd00`, `thiscall`, one stack word;
/// engine map: `NonActorMagicCaster::SetCurrentSpell`, probably folded).
const SET_EXTRA_FLAGS: u32 = 0x0041_fd00;
/// The word at `+0x0c` of an extra data (`0084e3a0`), which these
/// functions use as its flags.
const GET_EXTRA_FLAGS: u32 = 0x0084_e3a0;
/// The type number of the extra data these functions handle.
const EXTRA_TYPE: u32 = 0x92;

// Translated from 00564e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the object at `+0x20` exists and has bit `0x8000` set in its
/// word at `+8` (`00564e40`). `__fastcall`, no stack words.
pub fn fn_00564e00(e: &mut Engine, this: Ptr) -> bool {
    let mut result = false;
    if e.call(RELATED_OBJECT, &args![this]).u32() != 0 {
        let related = e.call(RELATED_OBJECT, &args![this]).ptr::<()>();
        if fn_00564e40(e, related) {
            result = true;
        }
    }
    result
}

// Translated from 00564e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit `0x8000` of the word at `+8` of `this`. `__fastcall`.
pub fn fn_00564e40(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 8) & 0x8000 != 0
}

// Translated from 00564e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit `0x8000` is set in the word at `+8` of the object
/// (`0044ddc0`), or when the object at `+0x20` exists and has it set in its
/// word at `+8` (`00564e40`).
/// `__fastcall`.
pub fn fn_00564e60(e: &mut Engine, this: Ptr) -> bool {
    let flags = e.call(GET_FLAGS, &args![this]).u32();
    if flags & 0x8000 != 0 {
        return true;
    }
    if e.call(RELATED_OBJECT, &args![this]).u32() != 0 {
        let related = e.call(RELATED_OBJECT, &args![this]).ptr::<()>();
        if fn_00564e40(e, related) {
            return true;
        }
    }
    false
}

// Translated from 00564eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` non-zero) or clears bit `0x8000` of the word at `+8` of the
/// object and stores it back (`00403550`). `__thiscall`, one stack word (a byte).
pub fn fn_00564eb0(e: &mut Engine, this: Ptr, set: u8) {
    let mut flags = e.call(GET_FLAGS, &args![this]).u32();
    if set != 0 {
        flags |= 0x8000;
    } else {
        flags &= 0xffff_7fff;
    }
    e.call(SET_FLAGS, &args![this, flags]);
}

// Translated from 00564f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the object has the extra data `0x92` (`00527080`) and bit 1 of
/// its flags (`0084e3a0`) is set. `__fastcall`, no stack words.
pub fn fn_00564f00(e: &mut Engine, this: Ptr) -> bool {
    let extra = e.call(FIND_EXTRA, &args![this, EXTRA_TYPE]).u32();
    if extra != 0 {
        let flags = e.call(GET_EXTRA_FLAGS, &args![extra]).u32();
        if flags & 2 != 0 {
            return true;
        }
    }
    false
}

// Translated from 00564f40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00564f40 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Sets (`set` non-zero) or clears bit 1 in the flags of the object's extra
/// data `0x92`. The extra data is created and added when it is missing and
/// `set` is exactly 1. When it exists: its flags are updated (`0041fd00`),
/// it is removed again when they become zero (`00410140`), and the object's
/// virtual slot `0x48` is called with `0x80000000`. `__thiscall`, one stack
/// word (a byte). The exception-unwinding frame is not translated.
pub fn fn_00564f40(e: &mut Engine, this: Ptr, set: u8) {
    let mut extra = e.call(FIND_EXTRA, &args![this, EXTRA_TYPE]).u32();
    if set == 1 && extra == 0 {
        let block = e.call(ALLOCATE, &args![0x14u32]).u32();
        let created = if block != 0 {
            e.call(EXTRA_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        let list = e.call(EXTRA_LIST, &args![this]).u32();
        extra = e.call(ADD_EXTRA, &args![list, created]).u32();
    }
    if extra != 0 {
        let mut flags = e.call(GET_EXTRA_FLAGS, &args![extra]).u32();
        if set != 0 {
            flags |= 2;
        } else {
            flags &= 0xffff_fffd;
        }
        e.call(SET_EXTRA_FLAGS, &args![extra, flags]);
        if flags == 0 {
            let list = e.call(EXTRA_LIST, &args![this]).u32();
            e.call(REMOVE_EXTRA, &args![list, EXTRA_TYPE]);
        }
        e.vcall(this.addr(), 0x48, &args![0x8000_0000u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00564e00, fn_00564e00(Ptr) -> bool),
        entry!(0x00564e40, fn_00564e40(Ptr) -> bool),
        entry!(0x00564e60, fn_00564e60(Ptr) -> bool),
        entry!(0x00564eb0, fn_00564eb0(Ptr, u8)),
        entry!(0x00564f00, fn_00564f00(Ptr) -> bool),
        entry!(0x00564f40, fn_00564f40(Ptr, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn bit_0x8000_of_the_word_at_8() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        let this = Ptr::<()>::new(object);
        assert!(!e.call(0x0056_4e40, &args![this]).bool());
        e.mem.set_u32(object + 8, 0x8000);
        assert!(e.call(0x0056_4e40, &args![this]).bool());
        e.mem.set_u32(object + 8, 0xffff_7fff);
        assert!(!e.call(0x0056_4e40, &args![this]).bool());
    }

    /// An object whose related object (`007af430`) is `related`, or null.
    fn with_related(e: &mut Engine, bits: u32, related: bool) -> Ptr {
        let object = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(other + 8, bits);
        e.register_double(RELATED_OBJECT, move |_, _| {
            ret(if related { other } else { 0 })
        });
        Ptr::new(object)
    }

    #[test]
    fn the_related_object_decides_the_first_test() {
        let mut e = Engine::new();
        let this = with_related(&mut e, 0x8000, true);
        assert!(e.call(0x0056_4e00, &args![this]).bool());
        let this = with_related(&mut e, 0x4000, true);
        assert!(!e.call(0x0056_4e00, &args![this]).bool());
        let this = with_related(&mut e, 0x8000, false);
        assert!(!e.call(0x0056_4e00, &args![this]).bool());
    }

    #[test]
    fn the_flag_word_or_the_related_object_decides_the_second_test() {
        let mut e = Engine::new();
        let this = with_related(&mut e, 0, true);
        e.register(GET_FLAGS, |_, _| ret(0x8000));
        assert!(e.call(0x0056_4e60, &args![this]).bool());
        e.register(GET_FLAGS, |_, _| ret(0x0001));
        assert!(!e.call(0x0056_4e60, &args![this]).bool());
        let this = with_related(&mut e, 0x8000, true);
        assert!(e.call(0x0056_4e60, &args![this]).bool());
        let this = with_related(&mut e, 0x8000, false);
        assert!(!e.call(0x0056_4e60, &args![this]).bool());
    }

    #[test]
    fn the_flag_bit_is_set_or_cleared_and_stored() {
        let mut e = Engine::new();
        e.register(GET_FLAGS, |_, _| ret(0x1234_8001));
        e.register(SET_FLAGS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0056_4eb0, &args![Ptr::<()>::new(0x5000), 0u32]);
        e.call(0x0056_4eb0, &args![Ptr::<()>::new(0x5000), 1u32]);
        e.call(0x0056_4eb0, &args![Ptr::<()>::new(0x5000), 2u32]);
        let stores: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == SET_FLAGS)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(
            stores,
            vec![
                vec![0x5000, 0x1234_0001],
                vec![0x5000, 0x1234_8001],
                vec![0x5000, 0x1234_8001],
            ]
        );
    }

    #[test]
    fn the_extra_flags_bit_is_tested_when_the_extra_exists() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(0x6000);
        e.register(FIND_EXTRA, |_, a| {
            assert_eq!(a[1], 0x92);
            ret(0x7000)
        });
        e.register(GET_EXTRA_FLAGS, |_, _| ret(2));
        assert!(e.call(0x0056_4f00, &args![this]).bool());
        e.register(GET_EXTRA_FLAGS, |_, _| ret(5));
        assert!(!e.call(0x0056_4f00, &args![this]).bool());
        e.register(FIND_EXTRA, |_, _| ret(0));
        assert!(!e.call(0x0056_4f00, &args![this]).bool());
    }

    /// An object with a vtable whose slot 0x48 is a recording double, and
    /// doubles for the extra data functions. Returns (object, log).
    /// Calls recorded by the doubles: address and argument words.
    type CallLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn extra_world(e: &mut Engine, existing: u32, flags: u32) -> (Ptr, CallLog) {
        let object = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x80);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable + 0x48, 0x0900_0048);
        let log: CallLog = Rc::default();
        for (address, result) in [
            (0x0900_0048, 0),
            (FIND_EXTRA, existing),
            (ALLOCATE, 0x8100),
            (EXTRA_CONSTRUCT, 0x8100),
            (EXTRA_LIST, 0x8200),
            (ADD_EXTRA, 0x8300),
            (GET_EXTRA_FLAGS, flags),
            (SET_EXTRA_FLAGS, 0),
            (REMOVE_EXTRA, 0),
        ] {
            let log = log.clone();
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                ret(result)
            });
        }
        (Ptr::new(object), log)
    }

    #[test]
    fn setting_creates_the_missing_extra_and_updates_its_flags() {
        let mut e = Engine::new();
        let (this, log) = extra_world(&mut e, 0, 0);
        e.call(0x0056_4f40, &args![this, 1u32]);
        let addresses: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            addresses,
            vec![
                FIND_EXTRA,
                ALLOCATE,
                EXTRA_CONSTRUCT,
                EXTRA_LIST,
                ADD_EXTRA,
                GET_EXTRA_FLAGS,
                SET_EXTRA_FLAGS,
                0x0900_0048,
            ]
        );
        let log = log.borrow();
        assert_eq!(log[4].1, vec![0x8200, 0x8100]);
        assert_eq!(log[6].1, vec![0x8300, 2]);
        assert_eq!(log[7].1, vec![this.addr(), 0x8000_0000]);
    }

    #[test]
    fn clearing_the_last_bit_removes_the_extra() {
        let mut e = Engine::new();
        let (this, log) = extra_world(&mut e, 0x8400, 2);
        e.call(0x0056_4f40, &args![this, 0u32]);
        let addresses: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            addresses,
            vec![
                FIND_EXTRA,
                GET_EXTRA_FLAGS,
                SET_EXTRA_FLAGS,
                EXTRA_LIST,
                REMOVE_EXTRA,
                0x0900_0048,
            ]
        );
        let log = log.borrow();
        assert_eq!(log[2].1, vec![0x8400, 0]);
        assert_eq!(log[4].1, vec![0x8200, 0x92]);
    }

    #[test]
    fn a_missing_extra_is_not_created_for_other_values_than_1() {
        let mut e = Engine::new();
        let (this, log) = extra_world(&mut e, 0, 0);
        e.call(0x0056_4f40, &args![this, 0u32]);
        e.call(0x0056_4f40, &args![this, 2u32]);
        let addresses: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(addresses, vec![FIND_EXTRA, FIND_EXTRA]);
    }
}
