//! `Havok SDK/run_00df6db0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00df6db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x4` (`SUB ECX,0x4`) and jumps to `00df6dd0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00df6db0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_6dd0, &args![this.addr().wrapping_sub(0x4), flags])
        .u32()
}

// Translated from 00df6dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00df6dd0`, the same deleting
/// destructor shape `fn_00df6db0` reaches (`RET 4`, bit 0 of the one stack word). The caller gets
/// its `EAX`; the class is unconfirmed.
pub fn fn_00df6dc0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_6dd0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// The three vtable words `00df6dd0` stores and the CRT `free` it calls (`00401030`, `cdecl`).
const VTABLE_WORD_010CCF8C: u32 = 0x010c_cf8c;
const VTABLE_WORD_010CCFAC: u32 = 0x010c_cfac;
const VTABLE_WORD_010C3BB8: u32 = 0x010c_3bb8;
const FREE_00401030: u32 = 0x0040_1030;

// Translated from 00df6dd0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00df6dd0 to the `RET 4` at 00df6dfa, inside the extent the
/// map gives `00df6dc0`): the deleting destructor the thunks `00df6db0` and `00df6dc0` jump to,
/// `thiscall` with one stack word. Stores `010ccf8c` at `+8`, `010ccfac` at `+4` and `010c3bb8`
/// at `+0`; with bit 0 of the word set frees the object (`00401030`). Answers `this`.
pub fn fn_00df6dd0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    let base = this.addr();
    e.mem.set_u32(base + 8, VTABLE_WORD_010CCF8C);
    e.mem.set_u32(base + 4, VTABLE_WORD_010CCFAC);
    e.mem.set_u32(base, VTABLE_WORD_010C3BB8);
    if flags & 1 != 0 {
        e.call(FREE_00401030, &args![base]);
    }
    base
}

/// The float at `0104da48`: the start value of the `+0xc` field of the record the contact
/// handlers get.
const FIRST_FLOAT: u32 = 0x0104_da48;
/// The constant stored in the first word of that record (`0108f288`).
const RECORD_VTABLE: u32 = 0x0108_f288;
/// `InterlockedExchangeAdd` import slot (stdcall: the counter, the addend; answers the old value).
const INTERLOCKED_EXCHANGE_ADD: u32 = 0x00fd_f1ec;

// Translated from 00df6e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (it runs the contact dispatcher table of `a2` over a list of body
/// pairs; class unconfirmed). Cdecl, four words: `a1` (its float at `+0x20` goes into the second
/// record), `a2` (its first word is the dispatcher table, the word at `+0xc` goes into the second
/// record), `a3` (`count` records of 0x20 bytes: body pointer pair at `+0`/`+4`, output buffer
/// at `+8`, a word at `+0xc` (the first record carries it; also the capacity the counter is checked
/// against), the stored result count at `+0x10`, the counter pointer at `+0x14`) and `count`.
///
/// For each record it picks the handler from the table at `*a2 + 0x9a8` (20-byte stride) by the
/// byte at `*a2 + ((shape(a).type + 0xd) << 5) + shape(b).type`, where the shape is the first
/// word of each body and the type the word at `+0xc` of it, and calls it (cdecl, four words:
/// both bodies, the second record and the first) with a 0x774-byte frame, in which the first
/// record at `+8` carries `0108f288`, the float `0104da48` at `+0xc`, the result buffer (frame
/// `+0x74`, 0x700 bytes) at `+0x10` and `+0x14` and the record's word at `+0x18`; the handler
/// writes its result count at `+0x1c`. A record with a counter pointer reserves that many
/// 0x70-byte results with `InterlockedExchangeAdd` (and gives them back, copying nothing, when
/// the new total passes the capacity word); a record without one writes its results at the start
/// of its output buffer. The results (0x70 bytes each) are copied there, and the
/// result count (16 bit) is stored at `+0x10`.
pub fn fn_00df6e00(e: &mut Engine, a1: Ptr, a2: Ptr, a3: Ptr, count: i32) {
    e.with_stack(0x774, |e, frame| {
        let s = frame.addr();
        let buffer = s.wrapping_add(0x74);
        let first = x87_quiet_bits(e.mem.u32(FIRST_FLOAT));
        e.mem.set_u32(s.wrapping_add(0xc), first);
        let height = x87_quiet_bits(e.mem.u32(a1.addr().wrapping_add(0x20)));
        e.mem.set_u32(s.wrapping_add(0x10), buffer);
        e.mem.set_u32(s.wrapping_add(0x2c), height);
        let dispatcher_word = e.mem.u32(a2.addr().wrapping_add(0xc));
        let dispatcher = e.mem.u32(a2.addr());
        e.mem.set_u32(s.wrapping_add(0x14), buffer);
        e.mem.set_u32(s.wrapping_add(0x8), RECORD_VTABLE);
        e.mem.set_u32(s.wrapping_add(0x18), 1);
        e.mem.set_u32(s.wrapping_add(0x1c), 0);
        e.mem.set_u32(s.wrapping_add(0x24), dispatcher);
        e.mem.set_u32(s.wrapping_add(0x30), dispatcher_word);
        if count <= 0 {
            return;
        }
        let mut rec = a3.addr().wrapping_add(0xc);
        e.mem.set_u32(s.wrapping_add(4), count as u32);
        loop {
            let body_a = e.mem.u32(rec.wrapping_sub(0xc));
            let first = x87_quiet_bits(e.mem.u32(FIRST_FLOAT));
            let shape_a = e.mem.u32(body_a);
            let body_b = e.mem.u32(rec.wrapping_sub(8));
            let shape_b = e.mem.u32(body_b);
            e.mem.set_u32(s.wrapping_add(0xc), first);
            e.mem.set_u32(s.wrapping_add(0x10), buffer);
            e.mem.set_u32(s.wrapping_add(0x14), buffer);
            let word = e.mem.u32(rec);
            e.mem.set_u32(s.wrapping_add(0x18), word);
            let dispatcher = e.mem.u32(a2.addr());
            e.mem.set_u32(s.wrapping_add(0x8), RECORD_VTABLE);
            e.mem.set_u32(s.wrapping_add(0x1c), 0);
            let row = e.mem.u32(shape_a.wrapping_add(0xc)).wrapping_add(0xd) << 5;
            let row = row.wrapping_add(e.mem.u32(shape_b.wrapping_add(0xc)));
            let kind = u32::from(e.mem.u8(row.wrapping_add(dispatcher)));
            let handler = e.mem.u32(
                dispatcher
                    .wrapping_add(kind.wrapping_mul(5).wrapping_mul(4))
                    .wrapping_add(0x9a8),
            );
            e.call(
                handler,
                &args![body_a, body_b, s.wrapping_add(0x24), s.wrapping_add(8)],
            );
            let counter = e.mem.u32(rec.wrapping_add(8));
            let mut keep = true;
            let mut target = e.mem.u32(rec.wrapping_sub(4));
            if counter != 0 {
                let results = e.mem.u32(s.wrapping_add(0x1c));
                let old = e
                    .call(INTERLOCKED_EXCHANGE_ADD, &args![counter, results])
                    .u32();
                let total = old.wrapping_add(results) as i32;
                if total > e.mem.i32(rec) {
                    let counter = e.mem.u32(rec.wrapping_add(8));
                    e.call(
                        INTERLOCKED_EXCHANGE_ADD,
                        &args![counter, results.wrapping_neg()],
                    );
                    keep = false;
                } else {
                    target = old
                        .wrapping_mul(0x70)
                        .wrapping_add(e.mem.u32(rec.wrapping_sub(4)));
                }
            }
            if keep && target != 0 {
                let results = u32::from(e.mem.u16(s.wrapping_add(0x1c)));
                e.mem.set_u32(rec.wrapping_add(4), results);
                // Seven 16-byte pieces per result, copied a dword at a time.
                let mut left = results.wrapping_mul(7) as i32;
                let mut from = buffer;
                let mut to = target;
                while left > 0 {
                    for lane in 0..4 {
                        let word = e.mem.u32(from.wrapping_add(4 * lane));
                        e.mem.set_u32(to.wrapping_add(4 * lane), word);
                    }
                    left -= 1;
                    from = from.wrapping_add(0x10);
                    to = to.wrapping_add(0x10);
                }
            }
            rec = rec.wrapping_add(0x20);
            let remaining = e.mem.u32(s.wrapping_add(4)).wrapping_sub(1);
            e.mem.set_u32(s.wrapping_add(4), remaining);
            if remaining == 0 {
                break;
            }
        }
    })
}

// Translated from 00df6f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a job function: `imm 00da3447` in the gap list): cdecl, two stack words
/// (nothing popped): `owner` and `job`. Appends the timer record `010db978`, calls `00df6e00` with
/// (`job`, the words at `job + 0x18`, `+0x28` and `+0x2c`), appends the timer record `010c41f8` and
/// returns the answer of `00d04e80` called on `owner` with (`job`, `job`, 0). Ghidra made no function here; the address is an immediate operand at 00da3447.
pub fn fn_00df6f80(e: &mut Engine, owner: u32, job: u32) -> u32 {
    crate::units::havok_sdk::hkpworld::profile_record(e, 0x010d_b978);
    let first = e.mem.u32(job + 0x18);
    let second = e.mem.u32(job + 0x28);
    let third = e.mem.u32(job + 0x2c);
    e.call(0x00df_6e00, &args![job, first, second, third]);
    crate::units::havok_sdk::hkpworld::profile_record(e, 0x010c_41f8);
    e.call(0x00d0_4e80, &args![owner, job, job, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_6db0, fn_00df6db0(Ptr, u32) -> u32),
        entry!(0x00df_6dc0, fn_00df6dc0(Ptr, u32) -> u32),
        entry!(0x00df_6dd0, fn_00df6dd0(Ptr, u32) -> u32),
        entry!(0x00df_6e00, fn_00df6e00(Ptr, Ptr, Ptr, i32)),
        entry!(0x00df_6f80, fn_00df6f80(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00df6db0() {
        assert_eq!(check(0x00df_6db0, 0x00df_6dd0, 0x4), 0xdead_beef);
    }
}

#[cfg(test)]
mod contact_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type HandlerLog = Rc<RefCell<Vec<Vec<u32>>>>;

    const HANDLER: u32 = 0x0a00_0100;

    struct Rig {
        e: Engine,
        a1: u32,
        a2: u32,
        list: u32,
        dispatcher: u32,
        bodies: [u32; 2],
        log: HandlerLog,
    }

    /// A dispatcher table whose byte for the (1, 2) type pair picks the handler at index 1; each
    /// handler call writes `results` 0x70-byte results (the dword `0x1000 + k` at each dword
    /// `k` of result `r`, plus `r << 16`) into the buffer it is given and reports the count.
    fn rig(results: u16) -> Rig {
        let mut e = Engine::new();
        let a1 = e.mem.alloc(0x40);
        let a2 = e.mem.alloc(0x20);
        let dispatcher = e.mem.alloc(0x1000);
        let shape_a = e.mem.alloc(0x20);
        let shape_b = e.mem.alloc(0x20);
        let body_a = e.mem.alloc(0x10);
        let body_b = e.mem.alloc(0x10);
        let list = e.mem.alloc(0x20 * 3);
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(0x0104_da48, 0x7f80_0001);
        e.mem.set_f32(a1 + 0x20, 6.5);
        e.mem.set_u32(a2, dispatcher);
        e.mem.set_u32(a2 + 0xc, 0xcafe);
        e.mem.set_u32(body_a, shape_a);
        e.mem.set_u32(body_b, shape_b);
        e.mem.set_u32(shape_a + 0xc, 1);
        e.mem.set_u32(shape_b + 0xc, 2);
        e.mem.set_u8(dispatcher + ((1 + 0xd) << 5) + 2, 1);
        e.mem.set_u32(dispatcher + 0x9a8 + 20, HANDLER);
        e.register_double(INTERLOCKED_EXCHANGE_ADD, |e, a| {
            let old = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], old.wrapping_add(a[1]));
            Ret {
                eax: old,
                ..Ret::default()
            }
        });
        let log: HandlerLog = Rc::default();
        {
            let log = log.clone();
            e.register_double(HANDLER, move |e, a| {
                let mut words = a.to_vec();
                for offset in [0u32, 4, 8, 0xc, 0x10, 0x14] {
                    words.push(e.mem.u32(a[3] + offset));
                }
                for offset in [0u32, 8, 0xc] {
                    words.push(e.mem.u32(a[2] + offset));
                }
                log.borrow_mut().push(words);
                let buffer = e.mem.u32(a[3] + 8);
                for r in 0..u32::from(results) {
                    for k in 0..0x1cu32 {
                        e.mem
                            .set_u32(buffer + r * 0x70 + 4 * k, 0x1000 + k + (r << 16));
                    }
                }
                e.mem.set_u8(a[3] + 0x14, results as u8);
                e.mem.set_u8(a[3] + 0x15, (results >> 8) as u8);
                Ret::default()
            });
        }
        Rig {
            e,
            a1,
            a2,
            list,
            dispatcher,
            bodies: [body_a, body_b],
            log,
        }
    }

    /// Fills record `index` of the list.
    fn record(r: &mut Rig, index: u32, output: u32, word: u32, counter: u32) {
        let at = r.list + 0x20 * index;
        r.e.mem.set_u32(at, r.bodies[0]);
        r.e.mem.set_u32(at + 4, r.bodies[1]);
        r.e.mem.set_u32(at + 8, output);
        r.e.mem.set_u32(at + 0xc, word);
        r.e.mem.set_u32(at + 0x10, 0xdead_beef);
        r.e.mem.set_u32(at + 0x14, counter);
    }

    #[test]
    fn the_handler_gets_the_two_records_and_the_results_are_copied() {
        let mut r = rig(2);
        let output = r.e.mem.alloc(0x400);
        record(&mut r, 0, output, 7, 0);
        r.e.call(0x00df_6e00, &args![r.a1, r.a2, r.list, 1i32]);
        let log = r.log.borrow();
        assert_eq!(log.len(), 1);
        let call = &log[0];
        assert_eq!(call[0], r.bodies[0]);
        assert_eq!(call[1], r.bodies[1]);
        // The first record: vtable, the quieted float, buffers, the record word, a cleared count.
        assert_eq!(call[4], 0x0108_f288);
        assert_eq!(call[5], 0x7fc0_0001);
        assert_eq!(call[6], call[7]);
        assert_eq!(call[6], call[2] + 0x74 - 0x24);
        assert_eq!(call[8], 7);
        assert_eq!(call[9], 0);
        // The second record: the dispatcher, the float of the first argument, the other word.
        assert_eq!(call[10], r.dispatcher);
        assert_eq!(call[11], 6.5f32.to_bits());
        assert_eq!(call[12], 0xcafe);
        for result in 0..2u32 {
            for k in 0..0x1cu32 {
                assert_eq!(
                    r.e.mem.u32(output + result * 0x70 + 4 * k),
                    0x1000 + k + (result << 16)
                );
            }
        }
        assert_eq!(r.e.mem.u32(output + 0xe0), 0);
        assert_eq!(r.e.mem.u32(r.list + 0x10), 2);
    }

    #[test]
    fn a_counter_reserves_the_slots_after_the_ones_already_taken() {
        let mut r = rig(2);
        let output = r.e.mem.alloc(0x400);
        let counter = r.e.mem.alloc(4);
        r.e.mem.set_u32(counter, 3);
        record(&mut r, 0, output, 10, counter);
        r.e.call(0x00df_6e00, &args![r.a1, r.a2, r.list, 1i32]);
        assert_eq!(r.e.mem.u32(counter), 5);
        assert_eq!(r.e.mem.u32(output + 3 * 0x70), 0x1000);
        assert_eq!(r.e.mem.u32(output + 4 * 0x70 + 4), 0x1001 + (1 << 16));
        assert_eq!(r.e.mem.u32(output), 0);
        assert_eq!(r.e.mem.u32(r.list + 0x10), 2);
    }

    #[test]
    fn passing_the_capacity_gives_the_reservation_back_and_copies_nothing() {
        let mut r = rig(2);
        let output = r.e.mem.alloc(0x800);
        let counter = r.e.mem.alloc(4);
        r.e.mem.set_u32(counter, 9);
        record(&mut r, 0, output, 10, counter);
        r.e.call(0x00df_6e00, &args![r.a1, r.a2, r.list, 1i32]);
        assert_eq!(r.e.mem.u32(counter), 9);
        assert_eq!(r.e.mem.u32(output + 9 * 0x70), 0);
        assert_eq!(r.e.mem.u32(r.list + 0x10), 0xdead_beef);
        // Exactly at the capacity is still accepted.
        r.e.mem.set_u32(counter, 8);
        r.e.call(0x00df_6e00, &args![r.a1, r.a2, r.list, 1i32]);
        assert_eq!(r.e.mem.u32(counter), 10);
        assert_eq!(r.e.mem.u32(r.list + 0x10), 2);
    }

    #[test]
    fn a_record_without_an_output_buffer_stores_nothing() {
        let mut r = rig(1);
        record(&mut r, 0, 0, 1, 0);
        r.e.call(0x00df_6e00, &args![r.a1, r.a2, r.list, 1i32]);
        assert_eq!(r.e.mem.u32(r.list + 0x10), 0xdead_beef);
        assert_eq!(r.log.borrow().len(), 1);
    }

    #[test]
    fn a_handler_with_no_results_stores_a_zero_count() {
        let mut r = rig(0);
        let output = r.e.mem.alloc(0x100);
        record(&mut r, 0, output, 1, 0);
        r.e.call(0x00df_6e00, &args![r.a1, r.a2, r.list, 1i32]);
        assert_eq!(r.e.mem.u32(r.list + 0x10), 0);
        assert_eq!(r.e.mem.u32(output), 0);
    }

    #[test]
    fn every_record_runs_and_a_zero_count_runs_none() {
        let mut r = rig(1);
        let first = r.e.mem.alloc(0x100);
        let second = r.e.mem.alloc(0x100);
        record(&mut r, 0, first, 5, 0);
        record(&mut r, 1, second, 6, 0);
        r.e.call(0x00df_6e00, &args![r.a1, r.a2, r.list, 2i32]);
        assert_eq!(r.log.borrow().len(), 2);
        assert_eq!(r.log.borrow()[1][8], 6);
        assert_eq!(r.e.mem.u32(first), 0x1000);
        assert_eq!(r.e.mem.u32(second), 0x1000);
        r.e.call(0x00df_6e00, &args![r.a1, r.a2, r.list, 0i32]);
        r.e.call(0x00df_6e00, &args![r.a1, r.a2, r.list, -1i32]);
        assert_eq!(r.log.borrow().len(), 2);
    }

    #[test]
    fn the_second_thunk_moves_this_back_by_eight() {
        let mut e = Engine::new();
        let log: HandlerLog = Rc::default();
        {
            let log = log.clone();
            e.register_double(0x00df_6dd0, move |_, a| {
                log.borrow_mut().push(a.to_vec());
                Ret {
                    eax: 0x99,
                    ..Ret::default()
                }
            });
        }
        assert_eq!(e.call(0x00df_6dc0, &args![0x3000u32, 1u32]).u32(), 0x99);
        assert_eq!(*log.borrow(), vec![vec![0x3000 - 8, 1]]);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::units::havok_sdk::hkpworld::profiling_support::{labels, profiling_engine};
    #[test]
    fn fn_00df6f80_wraps_the_worker_call_in_timer_records() {
        let (mut e, stream, area) = profiling_engine(2);
        let job = e.mem.alloc(0x40);
        e.mem.set_u32(job + 0x18, 0x11);
        e.mem.set_u32(job + 0x28, 0x22);
        e.mem.set_u32(job + 0x2c, 0x33);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00df_6e00, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.register_double(0x00d0_4e80, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x00df_6f80, &args![0x4000u32, job]).u32(), 0x4001);
        assert_eq!(*seen.borrow(), vec![vec![job, 0x11, 0x22, 0x33]]);
        assert_eq!(labels(&e, stream, area), vec![0x010d_b978, 0x010c_41f8]);
    }
}

#[cfg(test)]
mod gap1_tests {
    use super::*;

    #[test]
    fn fn_00df6dd0_stores_the_vtables_and_frees_on_bit_zero() {
        let mut e = Engine::new();
        e.register(FREE_00401030, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x00df_6dd0, &args![this, 0u32]).u32(), this);
        let words: Vec<u32> = (0..3).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(
            words,
            vec![
                VTABLE_WORD_010C3BB8,
                VTABLE_WORD_010CCFAC,
                VTABLE_WORD_010CCF8C
            ]
        );
        let frees = |e: &Engine| {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == FREE_00401030)
                .count()
        };
        assert_eq!(frees(&e), 0);
        assert_eq!(e.call(0x00df_6dd0, &args![this, 3u32]).u32(), this);
        assert_eq!(frees(&e), 1);
        // The thunk 00df6dc0 now reaches it with `this - 8`.
        assert_eq!(e.call(0x00df_6dc0, &args![this + 8, 0u32]).u32(), this);
    }
}
