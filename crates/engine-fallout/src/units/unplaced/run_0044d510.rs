//! `(unplaced)/run_0044d510` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0044d710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0044d6f0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0044d710(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_d6f0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `00529030` (thiscall, `RET 8`).
const BEGIN: u32 = 0x0052_9030;
/// `00529000` (thiscall, no stack words).
const END: u32 = 0x0052_9000;
/// `0044df00` (thiscall on `this + 0x14`, returns a pointer).
const MEMBER_GET: u32 = 0x0044_df00;
/// `0044df20` (thiscall on that pointer, returns `bool` in `AL`).
const MEMBER_TEST: u32 = 0x0044_df20;
/// `0092c820` (thiscall, `RET 4`).
const RECEIVE: u32 = 0x0092_c820;
/// `Error` (`0040fbe0`, engine map): an empty function in this build, called with no words.
const ERROR: u32 = 0x0040_fbe0;
/// `TlsGetValue` import slot (`kernel32`, stdcall: the index).
const TLS_GET_VALUE: u32 = 0x00fd_f044;
/// `0044e3a0` (thiscall, `RET 4`).
const CREATE_VALUE: u32 = 0x0044_e3a0;
/// `0065e750` (cdecl, two words).
const ITERATOR_CATEGORY: u32 = 0x0065_e750;
/// `0065fd70` (cdecl, three words; returns a pointer to a byte).
const POINTER_CATEGORY: u32 = 0x0065_fd70;
/// `0065fe40` (cdecl, one word).
const UNCHECKED: u32 = 0x0065_fe40;
/// `0044e4a0` (cdecl, six words).
const COPY_BODY: u32 = 0x0044_e4a0;
/// `bad_alloc` constructor (`0044d6c0`, engine map; thiscall, `RET 4`).
const BAD_ALLOC_CONSTRUCTOR: u32 = 0x0044_d6c0;
/// `__CxxThrowException@8` (`00ec7894`): throws the object with its throw information.
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// The throw information of `bad_alloc` passed to it.
const BAD_ALLOC_THROW_INFO: u32 = 0x0114_0ef8;
/// The game's `operator new` (`00401000`, cdecl: size in bytes).
const OPERATOR_NEW: u32 = 0x0040_1000;

// Translated from 0044d510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `ok = 00529030(this, first, second)`; when it is true, passes
/// `0044df00(this + 0x14) + 4` to `0092c820` on `target`, calls `Error` (empty) and clears
/// `ok` when `0044df20` on `0044df00(this + 0x14)` is true; in every case ends with
/// `00529000(this)`. Returns `ok`.
pub fn fn_0044d510(e: &mut Engine, this: Ptr, first: u32, second: u32, target: u32) -> bool {
    let base = this.addr();
    let mut ok = e.call(BEGIN, &args![base, first, second]).u8();
    if ok != 0 {
        let member = e.call(MEMBER_GET, &args![base + 0x14]).u32();
        e.call(RECEIVE, &args![target, member.wrapping_add(4)]);
        e.call(ERROR, &args![]);
        let member = e.call(MEMBER_GET, &args![base + 0x14]).u32();
        if e.call(MEMBER_TEST, &args![member]).u8() != 0 {
            ok = 0;
        }
    }
    e.call(END, &args![base]);
    ok != 0
}

// Translated from 0044d5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TlsGetValue(index)`; returns its result (`EAX`, which the decompiler drops).
pub fn fn_0044d5b0(e: &mut Engine, index: u32) -> u32 {
    e.call(TLS_GET_VALUE, &args![index]).u32()
}

// Translated from 0044d5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The thread-local value of the index at +4 (`0044d5b0`); when that is 0, the result of
/// `0044e3a0(this, argument)` instead.
pub fn fn_0044d5c0(e: &mut Engine, this: Ptr, argument: u32) -> u32 {
    let index = e.mem.u32(this.addr() + 4);
    let value = fn_0044d5b0(e, index);
    if value != 0 {
        return value;
    }
    e.call(CREATE_VALUE, &args![this, argument]).u32()
}

// Translated from 0044d600 (decompiled, FalloutNV.exe 1.4.0.525)
/// A copy wrapper in the shape of the C++ library's checked `copy`: the iterator category
/// `0065e750(&first, &third)`, the pointer category byte `*0065fd70(&tag, &first, &third)`,
/// then `0044e4a0(0065fe40(&first), 0065fe40(&second), third, pointer category, iterator
/// category, 0)`; the outer call's words are pushed between the inner calls, which pop only
/// their own. `&first` and the others are the addresses of the argument slots; `third` is read
/// back from its slot when pushed. Returns the outer call's `EAX`.
pub fn fn_0044d600(e: &mut Engine, first: u32, second: u32, third: u32) -> u32 {
    // The argument slots (EBP+8, +0xc, +0x10) and the local word at EBP-4 (bytes at -1, -2,
    // -3).
    e.with_stack(0x10, |e, frame| {
        let locals = frame.addr();
        let slots = locals + 4;
        e.mem.set_u32(slots, first);
        e.mem.set_u32(slots + 4, second);
        e.mem.set_u32(slots + 8, third);
        let zero_tag = locals + 3;
        let category_byte = locals + 2;
        let pointer_tag = locals + 1;
        e.mem.set_u8(zero_tag, 0);
        let category = e.call(ITERATOR_CATEGORY, &args![slots, slots + 8]).u8();
        e.mem.set_u8(category_byte, category);
        let zero = u32::from(e.mem.u8(zero_tag));
        let category = u32::from(e.mem.u8(category_byte));
        let tag = e
            .call(POINTER_CATEGORY, &args![pointer_tag, slots, slots + 8])
            .u32();
        let pointer_category = u32::from(e.mem.u8(tag));
        let third = e.mem.u32(slots + 8);
        let unchecked_second = e.call(UNCHECKED, &args![slots + 4]).u32();
        let unchecked_first = e.call(UNCHECKED, &args![slots]).u32();
        e.call(
            COPY_BODY,
            &args![
                unchecked_first,
                unchecked_second,
                third,
                pointer_category,
                category,
                zero
            ],
        )
        .u32()
    })
}

// Translated from 0044d670 (decompiled, FalloutNV.exe 1.4.0.525)
/// An array allocation of `count` 4-byte elements: a count of 0 allocates 0 bytes; when
/// `0xffffffff / count` is below 4 it constructs a `bad_alloc` (`0044d6c0`, with 0) in a
/// local and throws it (`00ec7894`, throw information `01140ef8`), which does not return (the
/// bytes after the throw are the allocation, reached only through it). Otherwise returns
/// `operator new(count * 4)` (`00401000`; its `EAX`, which the decompiler drops).
pub fn fn_0044d670(e: &mut Engine, count: u32) -> u32 {
    // A count of 0 skips the division (`JA`); it stays 0.
    if let Some(quotient) = 0xffff_ffffu32.checked_div(count) {
        if quotient < 4 {
            e.with_stack(0xc, |e, exception| {
                e.call(BAD_ALLOC_CONSTRUCTOR, &args![exception, 0u32]);
                e.call(CXX_THROW_EXCEPTION, &args![exception, BAD_ALLOC_THROW_INFO]);
            });
        }
    }
    e.call(OPERATOR_NEW, &args![count << 2]).u32()
}

/// `0044e460` (cdecl; five words pushed, four read).
const FORWARD_0044E460: u32 = 0x0044_e460;
/// `std::exception::exception` (`00ec78f1`, `ECX`, one stack word: the address of the
/// message pointer).
const EXCEPTION_CONSTRUCTOR: u32 = 0x00ec_78f1;
/// `std::exception::~exception` (engine map, `00ec79a1`, `ECX`).
const EXCEPTION_DESTRUCTOR: u32 = 0x00ec_79a1;
/// The vtable both [`fn_0044d6c0`] and [`fn_0044d6f0`] store (the `bad_alloc` vtable, by
/// the throw information [`fn_0044d670`] throws it with; not checked further).
const BAD_ALLOC_VTABLE: u32 = 0x0101_76f8;

// Translated from 0044d580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words, plain `RET`; called by `erase` at
/// `0044a6fb` and by `007ab0a0`, `007ab250`, ...): forwards its four words to `0044e460`
/// with a fifth, the tag byte `0065e750` answers. `0065e750` is called with nothing pushed;
/// it answers the top byte of the `ECX` it pushes, which is the `ECX` this function was
/// entered with, so that register is this function's first parameter `ecx` (as in
/// `0065e750`'s translation); the two words above its return address are this frame's
/// pushed `ECX` and the saved `EBP` (0 here), which it does not read.
pub fn fn_0044d580(
    e: &mut Engine,
    ecx: u32,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> u32 {
    let tag = e.call(ITERATOR_CATEGORY, &args![ecx, ecx, 0u32]).u8();
    e.call(
        FORWARD_0044E460,
        &args![first, second, third, fourth, u32::from(tag)],
    )
    .u32()
}

// Translated from 0044d6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bad_alloc` constructor (the engine map's name for `0044d6c0` in this unit's
/// constants; `ECX` and one stack word, `RET 4`): `std::exception::exception` with the
/// address of the `message` argument's slot, then the vtable `010176f8`. Answers `this`.
pub fn fn_0044d6c0(e: &mut Engine, this: Ptr, message: u32) -> Ptr {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), message);
        e.call(EXCEPTION_CONSTRUCTOR, &args![this, slot]);
    });
    e.mem.set_u32(this.addr(), BAD_ALLOC_VTABLE);
    this
}

// Translated from 0044d6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body the scalar deleting destructor [`fn_0044d710`] runs (`ECX`, plain
/// `RET`): the vtable `010176f8`, then `std::exception::~exception`.
pub fn fn_0044d6f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BAD_ALLOC_VTABLE);
    e.call(EXCEPTION_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044_d580, fn_0044d580(u32, u32, u32, u32, u32) -> u32),
        entry!(0x0044_d6c0, fn_0044d6c0(Ptr, u32) -> Ptr),
        entry!(0x0044_d6f0, fn_0044d6f0(Ptr)),
        entry!(0x0044_d710, fn_0044d710(Ptr, u32) -> Ptr),
        entry!(0x0044_d510, fn_0044d510(Ptr, u32, u32, u32) -> bool),
        entry!(0x0044_d5b0, fn_0044d5b0(u32) -> u32),
        entry!(0x0044_d5c0, fn_0044d5c0(Ptr, u32) -> u32),
        entry!(0x0044_d600, fn_0044d600(u32, u32, u32) -> u32),
        entry!(0x0044_d670, fn_0044d670(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0044d710_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_d6f0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_d710, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_d710)
                .collect();
            let mut expected = vec![(0x0044_d6f0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn log_without(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != top)
            .collect()
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn checked_call_reports_and_clears_the_result_by_the_member_test() {
        for (begin, test, want) in [(0u32, 0u32, false), (1, 0, true), (1, 1, false)] {
            let mut e = Engine::new();
            e.register_double(BEGIN, move |_, _| ret(0xab00 | begin));
            e.register(END, |_, _| Ret::default());
            e.register(MEMBER_GET, |_, _| ret(0x5000));
            e.register_double(MEMBER_TEST, move |_, _| ret(0xff00 | test));
            e.register(RECEIVE, |_, _| Ret::default());
            e.register(ERROR, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            let r = e.call(0x0044_d510, &args![0x100u32, 1u32, 2u32, 0x300u32]);
            assert_eq!(r.bool(), want);
            let log = log_without(&mut e, 0x0044_d510);
            let mut expected = vec![(BEGIN, vec![0x100, 1, 2])];
            if begin != 0 {
                expected.extend([
                    (MEMBER_GET, vec![0x114]),
                    (RECEIVE, vec![0x300, 0x5004]),
                    (ERROR, vec![]),
                    (MEMBER_GET, vec![0x114]),
                    (MEMBER_TEST, vec![0x5000]),
                ]);
            }
            expected.push((END, vec![0x100]));
            assert_eq!(log, expected);
        }
    }

    #[test]
    fn thread_local_value_or_the_created_one() {
        let mut e = Engine::new();
        e.register(TLS_GET_VALUE, |_, a| {
            ret(if a[0] == 7 { 0x7777 } else { 0 })
        });
        e.register(CREATE_VALUE, |_, a| ret(a[1] + 1));
        assert_eq!(e.call(0x0044_d5b0, &args![7u32]).u32(), 0x7777);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_d5c0, &args![this, 0x40u32]).u32(), 0x7777);
        assert!(log_without(&mut e, 0x0044_d5c0)
            .iter()
            .all(|(a, _)| *a != CREATE_VALUE));
        e.mem.set_u32(this + 4, 8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_d5c0, &args![this, 0x40u32]).u32(), 0x41);
        assert_eq!(
            log_without(&mut e, 0x0044_d5c0),
            vec![(TLS_GET_VALUE, vec![8]), (CREATE_VALUE, vec![this, 0x40])]
        );
    }

    #[test]
    fn copy_wrapper_passes_the_outer_words_in_push_order() {
        let mut e = Engine::new();
        e.register(ITERATOR_CATEGORY, |e, a| {
            assert_eq!(e.mem.u32(a[0]), 0x11);
            assert_eq!(e.mem.u32(a[1]), 0x33);
            ret(0x1205)
        });
        e.register(POINTER_CATEGORY, |e, a| {
            e.mem.set_u8(a[0], 0x09);
            // The callee may move the third slot; the outer call reads it back.
            e.mem.set_u32(a[2], 0x34);
            ret(a[0])
        });
        e.register(UNCHECKED, |e, a| ret(e.mem.u32(a[0]) + 0x1000));
        e.register(COPY_BODY, |_, _| ret(0xbeef));
        e.call_log = Some(vec![]);
        let r = e.call(0x0044_d600, &args![0x11u32, 0x22u32, 0x33u32]);
        assert_eq!(r.u32(), 0xbeef);
        let outer: Vec<_> = log_without(&mut e, 0x0044_d600)
            .into_iter()
            .filter(|(a, _)| *a == COPY_BODY)
            .collect();
        assert_eq!(
            outer,
            vec![(COPY_BODY, vec![0x1011, 0x1022, 0x34, 0x09, 0x05, 0])]
        );
    }

    #[test]
    fn array_allocation_throws_on_overflow() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, a| ret(0x8000u32.wrapping_add(a[0])));
        e.register(BAD_ALLOC_CONSTRUCTOR, |_, _| Ret::default());
        e.register(CXX_THROW_EXCEPTION, |_, _| Ret::default());
        assert_eq!(e.call(0x0044_d670, &args![0u32]).u32(), 0x8000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_d670, &args![3u32]).u32(), 0x800c);
        assert!(log_without(&mut e, 0x0044_d670)
            .iter()
            .all(|(a, _)| *a != CXX_THROW_EXCEPTION));
        // 0xffffffff / 0x40000000 = 3 < 4: throws.
        e.call_log = Some(vec![]);
        e.call(0x0044_d670, &args![0x4000_0000u32]);
        let log = log_without(&mut e, 0x0044_d670);
        let exception = log[0].1[0];
        assert_eq!(log[0], (BAD_ALLOC_CONSTRUCTOR, vec![exception, 0]));
        assert_eq!(
            log[1],
            (CXX_THROW_EXCEPTION, vec![exception, BAD_ALLOC_THROW_INFO])
        );
        // 0xffffffff / 0x3fffffff = 4: no throw.
        e.call_log = Some(vec![]);
        e.call(0x0044_d670, &args![0x3fff_ffffu32]);
        let log = log_without(&mut e, 0x0044_d670);
        assert_eq!(log, vec![(OPERATOR_NEW, vec![0xffff_fffc])]);
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    #[test]
    fn fn_0044d580_adds_the_tag_byte_of_ecx() {
        let mut e = Engine::new();
        e.register(ITERATOR_CATEGORY, |_, a| (a[0] >> 24).into_ret());
        e.register(FORWARD_0044E460, |_, _| 0x7777u32.into_ret());
        e.call_log = Some(vec![]);
        let r = e.call(0x0044_d580, &args![0xab00_0000u32, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(r.u32(), 0x7777);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (ITERATOR_CATEGORY, vec![0xab00_0000, 0xab00_0000, 0])
        );
        assert_eq!(log[2], (FORWARD_0044E460, vec![1, 2, 3, 4, 0xab]));
    }

    #[test]
    fn fn_0044d6c0_builds_the_exception_and_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(EXCEPTION_CONSTRUCTOR, |e, a| {
            // The message pointer is read through the slot's address.
            assert_eq!(e.mem.u32(a[1]), 0x0123_4567);
            e.mem.set_u32(a[0], 0x0010_f1fa);
            a[0].into_ret()
        });
        let object = e.mem.alloc(0xc);
        let r = e.call(0x0044_d6c0, &args![object, 0x0123_4567u32]).u32();
        assert_eq!(r, object);
        assert_eq!(e.mem.u32(object), BAD_ALLOC_VTABLE);
    }

    #[test]
    fn fn_0044d6f0_restores_the_vtable_then_destroys_the_exception() {
        let mut e = Engine::new();
        e.register(EXCEPTION_DESTRUCTOR, |e, a| {
            assert_eq!(e.mem.u32(a[0]), BAD_ALLOC_VTABLE);
            Ret::default()
        });
        let object = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        e.call(0x0044_d6f0, &args![object]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (EXCEPTION_DESTRUCTOR, vec![object]));
    }
}
