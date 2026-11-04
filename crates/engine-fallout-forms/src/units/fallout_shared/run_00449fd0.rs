//! `fallout shared/run_00449fd0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006815c0` (`ECX` = a pointer): answers `ECX` itself.
const ANSWER_THIS: u32 = 0x0068_15c0;
/// `0065d3a0` (`ECX` = object, one word: an allocator by value): the base constructor.
const BASE_CONSTRUCT: u32 = 0x0065_d3a0;
/// `0044a240` (`ECX` = object, one word): run last by `00449fd0` with 0.
const FINISH: u32 = 0x0044_a240;
/// `0065f550` (`ECX` = object, address of an 8-byte result): answers the
/// address of the two words it produced.
const FIRST_PAIR: u32 = 0x0065_f550;
/// `0044a170` (`ECX` = object, address of an 8-byte result): likewise.
const SECOND_PAIR: u32 = 0x0044_a170;
/// `0044a1a0` (`ECX` = object, address of an 8-byte result, four words).
const COMBINE: u32 = 0x0044_a1a0;

// Translated from 00449fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00449fd0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The engine map gives it no name. `thiscall`, no stack arguments, answers
/// `this`: calls `006815c0` with the address of a stack slot holding `this`
/// (the compiler pushed `ECX` and passed `ESP`), then `0065d3a0` on the object with that
/// slot's word as its one stack argument (the push stays for it), then `0044a240` on it with
/// 0. The exception-unwinding frame is not
/// translated.
pub fn fn_00449fd0(e: &mut Engine, this: Ptr) -> Ptr {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), this.addr());
        e.call(ANSWER_THIS, &args![slot]);
        // The pushed word stays on the stack as 0065d3a0's by-value argument (it pops it,
        // `RET 4`).
        let argument = e.mem.u32(slot.addr());
        e.call(BASE_CONSTRUCT, &args![this, argument]);
        e.call(FINISH, &args![this, 0u32]);
    });
    this
}

// Translated from 0044a0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map gives it no name. `thiscall`, no stack arguments: takes
/// the two words `0065f550` produces (into an 8-byte stack result) and the
/// two words `0044a170` produces, and passes them, second pair first, with a
/// third 8-byte stack result to `0044a1a0`. The decompiler shows no result;
/// the function leaves `EAX` as `0044a1a0` left it, returned here.
pub fn fn_0044a0a0(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(24, |e, buffer| {
        let first_result = buffer;
        let second_result = buffer.byte_add(8);
        let combined = buffer.byte_add(16);
        let first = e.call(FIRST_PAIR, &args![this, first_result]).u32();
        let (first_low, first_high) = (e.mem.u32(first), e.mem.u32(first + 4));
        let second = e.call(SECOND_PAIR, &args![this, second_result]).u32();
        let (second_low, second_high) = (e.mem.u32(second), e.mem.u32(second + 4));
        e.call(
            COMBINE,
            &args![
                this,
                combined,
                second_low,
                second_high,
                first_low,
                first_high
            ],
        )
        .u32()
    })
}

/// `std::vector<unsigned long>::_Tidy` (engine map, `0065f240`) and the
/// base destructor run after it (`0065cfb0`), both `thiscall`.
const TIDY: u32 = 0x0065_f240;
const BASE_DESTRUCT: u32 = 0x0065_cfb0;

// Translated from 0044a040 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0044a040 - the exception-unwinding frame (FS:[0]) is not translated.
/// The engine map gives it no name (the destructor matching `00449fd0`):
/// `_Tidy` (`0065f240`) on `this`, then the base destructor `0065cfb0`. The
/// exception-unwinding frame (and the cookie word it stores, which nothing
/// checks) is not translated.
pub fn fn_0044a040(e: &mut Engine, this: Ptr) {
    e.call(TIDY, &args![this]);
    e.call(BASE_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00449fd0, fn_00449fd0(Ptr) -> Ptr),
        entry!(0x0044a0a0, fn_0044a0a0(Ptr) -> u32),
        entry!(0x0044a040, fn_0044a040(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00449fd0_runs_its_three_callees_and_returns_this() {
        let mut e = Engine::new();
        e.register_double(ANSWER_THIS, |e, a| {
            // The slot the first callee gets holds the object address.
            assert_eq!(e.mem.u32(a[0]), 0x7000);
            Ret::default()
        });
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(FINISH, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00449fd0, &args![0x7000u32]).u32(), 0x7000);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log[1..].iter().map(|c| c.0).collect();
        assert_eq!(order, vec![ANSWER_THIS, BASE_CONSTRUCT, FINISH]);
        assert_eq!(log[2].1, vec![0x7000, 0x7000]);
        assert_eq!(log[3].1, vec![0x7000, 0]);
    }

    #[test]
    fn fn_0044a0a0_passes_the_second_pair_first() {
        let mut e = Engine::new();
        e.register(FIRST_PAIR, |e, a| {
            e.mem.set_u32(a[1], 11);
            e.mem.set_u32(a[1] + 4, 12);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(SECOND_PAIR, |e, a| {
            e.mem.set_u32(a[1], 21);
            e.mem.set_u32(a[1] + 4, 22);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(COMBINE, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044a0a0, &args![0x7000u32]).u32(), 0x1234);
        let combine = calls_to(&e, COMBINE);
        assert_eq!(combine.len(), 1);
        assert_eq!(combine[0][0], 0x7000);
        assert_eq!(combine[0][2..], [21, 22, 11, 12]);
        let first = calls_to(&e, FIRST_PAIR);
        let second = calls_to(&e, SECOND_PAIR);
        assert_eq!(first[0][0], 0x7000);
        assert_eq!(second[0][0], 0x7000);
        // Three distinct 8-byte results.
        let mut results = vec![first[0][1], second[0][1], combine[0][1]];
        results.sort_unstable();
        results.dedup();
        assert_eq!(results.len(), 3);
        assert!(results.windows(2).all(|w| w[1] - w[0] >= 8));
    }
}

#[cfg(test)]
mod b014_tests {
    use super::*;

    #[test]
    fn destructor_tidies_then_runs_the_base() {
        let mut e = Engine::new();
        e.register(TIDY, |_, _| Ret::default());
        e.register(BASE_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0044a040, &args![0x1000u32]);
        let log = e.call_log.take().unwrap();
        let order: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![(TIDY, vec![0x1000]), (BASE_DESTRUCT, vec![0x1000])]
        );
    }

    #[test]
    fn fn_00449fd0_passes_the_copied_slot_word_to_the_base_constructor() {
        let mut e = Engine::new();
        // The copy step leaves a different word in the slot: that word is
        // the base constructor's stack argument.
        e.register(ANSWER_THIS, |e, a| {
            e.mem.set_u32(a[0], 0x0bad_0001);
            a[0].into_ret()
        });
        e.register(BASE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FINISH, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00449fd0, &args![0x7000u32]);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log.iter().filter(|c| c.0 == BASE_CONSTRUCT).collect();
        assert_eq!(base.len(), 1);
        assert_eq!(base[0].1, vec![0x7000, 0x0bad_0001]);
    }
}
