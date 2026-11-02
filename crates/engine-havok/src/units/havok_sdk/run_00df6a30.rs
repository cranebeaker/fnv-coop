//! `Havok SDK/run_00df6a30` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The float-returning routine the thunks below end in (`00df6a60`).
const QUERY: u32 = 0x00df_6a60;

// Translated from 00df6a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). Thiscall, no stack words: the float at offset 4
/// of the object `this + 8` points to (`FLD`, so a signalling NaN comes back quieted).
pub fn fn_00df6a30(e: &mut Engine, this: Ptr) -> f32 {
    let inner = e.mem.u32(this.addr().wrapping_add(8));
    x87_quiet(e.mem.f32(inner.wrapping_add(4)))
}

// Translated from 00df6a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). Thiscall with one word `out`: stores a byte at
/// `out` that is 1 when the signed dword at offset 0x14 of the object `this + 8` points to is
/// greater than zero (`SETG`), else 0; returns `out`.
pub fn fn_00df6a40(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let inner = e.mem.u32(this.addr().wrapping_add(8));
    let count = e.mem.i32(inner.wrapping_add(0x14));
    e.mem.set_u8(out.addr(), u8::from(count > 0));
    out
}

// Translated from 00df6a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). Thiscall with one word `slot`, a pointer to a
/// pointer; answers the float at offset 4 of the object at `this + 0x10` (`FLD`, quieted).
///
/// When `*slot` is not null and `this + 0x14` differs from `slot`, it asks the object at
/// `*(this + 0xc) + 8` (virtual slot offset 4, three words: a byte out slot, `this + 0x14`,
/// `slot`) whether to go on. The out slot is the stack argument word of the exe itself, which
/// holds `slot` when the call starts; the call returns a pointer to the byte. When that byte is
/// not zero, it picks a handler by a byte of the object `*slot` points to, at `((*(this + 0x18)
/// + 0xd) << 5) + *(*slot + 0xc) + *(this + 0x20)`, from the 20-byte-stride table at
/// `*(this + 0x20) + 0x9a8`, and calls it (cdecl, four words: `*(this + 0x14)`, `slot`,
/// `this + 0x20`, `*(this + 0x10)`). Every field after the virtual call is read again.
pub fn fn_00df6a60(e: &mut Engine, this: Ptr, slot: Ptr) -> f32 {
    let base = this.addr();
    let target = e.mem.u32(slot.addr());
    if target != 0 {
        let other = e.mem.u32(base.wrapping_add(0x14));
        if other != slot.addr() {
            let filter = e.mem.u32(base.wrapping_add(0xc)).wrapping_add(8);
            let verdict = e.with_stack(4, |e, frame| {
                e.mem.set_u32(frame.addr(), slot.addr());
                let answer = e.vcall(filter, 4, &args![frame, other, slot]).u32();
                e.mem.u8(answer)
            });
            if verdict != 0 {
                let table = e.mem.u32(base.wrapping_add(0x20));
                let row = e.mem.u32(base.wrapping_add(0x18)).wrapping_add(0xd) << 5;
                let row = row.wrapping_add(e.mem.u32(target.wrapping_add(0xc)));
                let index = u32::from(e.mem.u8(row.wrapping_add(table)));
                let handler = e.mem.u32(
                    table
                        .wrapping_add(index.wrapping_mul(5).wrapping_mul(4))
                        .wrapping_add(0x9a8),
                );
                let other = e.mem.u32(base.wrapping_add(0x14));
                let owner = e.mem.u32(base.wrapping_add(0x10));
                e.call(handler, &args![other, slot, base.wrapping_add(0x20), owner]);
            }
        }
    }
    let inner = e.mem.u32(base.wrapping_add(0x10));
    x87_quiet(e.mem.f32(inner.wrapping_add(4)))
}

// Translated from 00df6af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). Thiscall with two stack words, only the first
/// read: runs `00df6a60` on the same `this` with `object + (signed byte at object + 5)` and
/// answers its float.
pub fn fn_00df6af0(e: &mut Engine, this: Ptr, object: Ptr, _unused_2: u32) -> f32 {
    let shift = i32::from(e.mem.u8(object.addr().wrapping_add(5)) as i8);
    let adjusted = object.addr().wrapping_add(shift as u32);
    e.call(QUERY, &args![this, adjusted]).f32()
}

// Translated from 00df6b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). Thiscall with one stack word: a tail jump to
/// `00df6a60` with `this - 4` and the word at offset 0xc of `object`; answers its float.
pub fn fn_00df6b10(e: &mut Engine, this: Ptr, object: Ptr) -> f32 {
    let slot = e.mem.u32(object.addr().wrapping_add(0xc));
    e.call(QUERY, &args![this.addr().wrapping_sub(4), slot])
        .f32()
}

// Translated from 00df6b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). Thiscall with two stack words `out` and
/// `slot`: runs `00df6a60` on `this - 8` with `slot`, discards its float (`FSTP ST0`), stores
/// the byte 0 at `out` and returns `out`.
pub fn fn_00df6b30(e: &mut Engine, this: Ptr, out: Ptr, slot: Ptr) -> Ptr {
    e.call(QUERY, &args![this.addr().wrapping_sub(8), slot]);
    e.mem.set_u8(out.addr(), 0);
    out
}

// Translated from 00df6850 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a job function: `imm 00da3465` in the gap list): cdecl, two stack words
/// (nothing popped): `owner` and `job`. Appends the timer record `010db904`, then walks the `count`
/// entries (0x30 bytes each) of the array at `job + 0x2c` (`count` at `job + 0x30`, signed). For each
/// entry it zeroes the word at +0x28 and keeps the entry's word at +0x24 in a local cell, passes
/// that cell by address and starts with the budget in the word at +0x20; then, for each of the three
/// job words that is not 0 (at `job + 0x20`, `+0x24`, `+0x28`), it calls `00d1d540`, `00d1d4d0` and
/// `00d1b930` in that order with (the word, the entry, the cell, the budget left); the first two add
/// their answer to the entry's word at +0x28 and take it off the budget, the third only adds it.
/// (The exe keeps the cell in the stack slot of the `job` argument.) Then it appends the timer
/// record `010c41f8` and returns the answer of `00d04e80` called on `owner` with (`job`, `job`, 0).
/// Ghidra made no function here; the address is an immediate operand at 00da3465.
pub fn fn_00df6850(e: &mut Engine, owner: u32, job: u32) -> u32 {
    crate::units::havok_sdk::hkpworld::profile_record(e, 0x010d_b904);
    let count = e.mem.i32(job + 0x30);
    let mut entry = e.mem.u32(job + 0x2c);
    if count > 0 {
        e.with_stack(4, |e, cell| {
            let cell = cell.addr();
            for _ in 0..count {
                let kept = e.mem.u32(entry + 0x24);
                let mut budget = e.mem.u32(entry + 0x20);
                e.mem.set_u32(entry + 0x28, 0);
                e.mem.set_u32(cell, kept);
                let first = e.mem.u32(job + 0x20);
                if first != 0 {
                    let used = e
                        .call(0x00d1_d540, &args![first, entry, cell, budget])
                        .u32();
                    let total = e.mem.u32(entry + 0x28).wrapping_add(used);
                    e.mem.set_u32(entry + 0x28, total);
                    budget = budget.wrapping_sub(used);
                }
                let second = e.mem.u32(job + 0x24);
                if second != 0 {
                    let used = e
                        .call(0x00d1_d4d0, &args![second, entry, cell, budget])
                        .u32();
                    let total = e.mem.u32(entry + 0x28).wrapping_add(used);
                    e.mem.set_u32(entry + 0x28, total);
                    budget = budget.wrapping_sub(used);
                }
                let third = e.mem.u32(job + 0x28);
                if third != 0 {
                    let used = e
                        .call(0x00d1_b930, &args![third, entry, cell, budget])
                        .u32();
                    let total = e.mem.u32(entry + 0x28).wrapping_add(used);
                    e.mem.set_u32(entry + 0x28, total);
                }
                entry = entry.wrapping_add(0x30);
            }
        });
    }
    crate::units::havok_sdk::hkpworld::profile_record(e, 0x010c_41f8);
    e.call(0x00d0_4e80, &args![owner, job, job, 0u32]).u32()
}
// Translated from 00df6950 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a job function: `imm 00da345b` in the gap list): cdecl, two stack words
/// (nothing popped): `owner` and `job`. Appends the timer record `010db918`, then walks the `count`
/// entries (0x30 bytes each) of the array at `job + 0x24` (`count` at `job + 0x28`, signed). For
/// each entry it zeroes the word at +0x28, keeps the word at +0x24 in a local cell and, for each of
/// the non-zero pointers of the list at `job + 0x30` (length at `job + 0x2c`, read again after each
/// step), calls `00d1b930` with (the pointer, the entry, the cell, the budget left), adds the answer
/// to the entry's word at +0x28 and takes it off the budget (which starts as the word at +0x20).
/// (The exe keeps the cell in a stack slot of its own.) Then it appends the timer record
/// `010c41f8` and returns the answer of `00d04e80` called on `owner` with (`job`, `job`, 0). Ghidra made no function here; the address is an immediate operand at 00da345b.
pub fn fn_00df6950(e: &mut Engine, owner: u32, job: u32) -> u32 {
    crate::units::havok_sdk::hkpworld::profile_record(e, 0x010d_b918);
    let count = e.mem.i32(job + 0x28);
    let mut entry = e.mem.u32(job + 0x24);
    if count > 0 {
        e.with_stack(4, |e, cell| {
            let cell = cell.addr();
            for _ in 0..count {
                let kept = e.mem.u32(entry + 0x24);
                let mut budget = e.mem.u32(entry + 0x20);
                e.mem.set_u32(entry + 0x28, 0);
                e.mem.set_u32(cell, kept);
                if e.mem.i32(job + 0x2c) > 0 {
                    let mut index = 0u32;
                    loop {
                        let pointer = e.mem.u32(job + 0x30 + 4 * index);
                        if pointer != 0 {
                            let used = e
                                .call(0x00d1_b930, &args![pointer, entry, cell, budget])
                                .u32();
                            let total = e.mem.u32(entry + 0x28).wrapping_add(used);
                            e.mem.set_u32(entry + 0x28, total);
                            budget = budget.wrapping_sub(used);
                        }
                        index += 1;
                        if (index as i32) >= e.mem.i32(job + 0x2c) {
                            break;
                        }
                    }
                }
                entry = entry.wrapping_add(0x30);
            }
        });
    }
    crate::units::havok_sdk::hkpworld::profile_record(e, 0x010c_41f8);
    e.call(0x00d0_4e80, &args![owner, job, job, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_6a30, fn_00df6a30(Ptr) -> f32),
        entry!(0x00df_6a40, fn_00df6a40(Ptr, Ptr) -> Ptr),
        entry!(0x00df_6a60, fn_00df6a60(Ptr, Ptr) -> f32),
        entry!(0x00df_6af0, fn_00df6af0(Ptr, Ptr, u32) -> f32),
        entry!(0x00df_6b10, fn_00df6b10(Ptr, Ptr) -> f32),
        entry!(0x00df_6b30, fn_00df6b30(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00df_6850, fn_00df6850(u32, u32) -> u32),
        entry!(0x00df_6950, fn_00df6950(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;

    const VERDICT: u32 = 0x0a00_0010;
    const HANDLER: u32 = 0x0a00_0020;

    struct Rig {
        e: Engine,
        this: u32,
        slot: u32,
        owner: u32,
        verdict: Rc<Cell<u8>>,
        verdict_calls: CallLog,
        handler_calls: CallLog,
    }

    /// An object whose `+0xc` holder carries the filter at `+8`, a table whose row byte selects
    /// handler 2, and a slot pointing at a target object.
    fn rig() -> Rig {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let holder = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x10);
        let owner = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        let table = e.mem.alloc(0x1000);
        let target = e.mem.alloc(0x20);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, target);
        e.mem.set_u32(target + 0xc, 0x10);
        e.mem.set_u32(holder + 8, vtable);
        e.mem.set_u32(vtable + 4, VERDICT);
        e.mem.set_f32(owner + 4, 2.5);
        e.mem.set_u32(this + 0xc, holder);
        e.mem.set_u32(this + 0x10, owner);
        e.mem.set_u32(this + 0x14, other);
        e.mem.set_u32(this + 0x18, 3);
        e.mem.set_u32(this + 0x20, table);
        e.mem.set_u8(table + 0x200 + 0x10, 2);
        e.mem.set_u32(table + 0x9a8 + 2 * 20, HANDLER);
        let verdict = Rc::new(Cell::new(1u8));
        let verdict_calls: CallLog = Rc::default();
        let handler_calls: CallLog = Rc::default();
        {
            let verdict = verdict.clone();
            let log = verdict_calls.clone();
            e.register_double(VERDICT, move |e, a| {
                let mut words = a.to_vec();
                words.push(e.mem.u32(a[1]));
                log.borrow_mut().push(words);
                e.mem.set_u8(a[1], verdict.get());
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            });
        }
        {
            let log = handler_calls.clone();
            e.register_double(HANDLER, move |_, a| {
                log.borrow_mut().push(a.to_vec());
                Ret::default()
            });
        }
        Rig {
            e,
            this,
            slot,
            owner,
            verdict,
            verdict_calls,
            handler_calls,
        }
    }

    #[test]
    fn the_getter_reads_the_float_behind_the_inner_object() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let inner = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, inner);
        e.mem.set_f32(inner + 4, -1.25);
        assert_eq!(e.call(0x00df_6a30, &args![this]).f32(), -1.25);
        e.mem.set_u32(inner + 4, 0x7f80_0001);
        assert_eq!(fn_00df6a30(&mut e, Ptr::new(this)).to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn the_flag_is_set_for_a_positive_count_only() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let inner = e.mem.alloc(0x20);
        let out = e.mem.alloc(4);
        e.mem.set_u32(this + 8, inner);
        for (count, flag) in [(5u32, 1u8), (0, 0), (0xffff_ffff, 0), (0x8000_0000, 0)] {
            e.mem.set_u32(inner + 0x14, count);
            e.mem.set_u32(out, 0xaabb_ccdd);
            let result = e.call(0x00df_6a40, &args![this, out]).u32();
            assert_eq!(result, out);
            assert_eq!(e.mem.u32(out), 0xaabb_cc00 | u32::from(flag));
        }
    }

    #[test]
    fn a_vetted_slot_runs_the_handler_the_table_selects() {
        let mut r = rig();
        let result = r.e.call(0x00df_6a60, &args![r.this, r.slot]).f32();
        assert_eq!(result, 2.5);
        let other = r.e.mem.u32(r.this + 0x14);
        let verdicts = r.verdict_calls.borrow();
        assert_eq!(verdicts.len(), 1);
        let holder = r.e.mem.u32(r.this + 0xc);
        assert_eq!(verdicts[0][0], holder + 8);
        assert_eq!(verdicts[0][2], other);
        assert_eq!(verdicts[0][3], r.slot);
        assert_eq!(verdicts[0][4], r.slot);
        assert_eq!(
            *r.handler_calls.borrow(),
            vec![vec![other, r.slot, r.this + 0x20, r.owner]]
        );
    }

    #[test]
    fn a_refused_slot_runs_no_handler() {
        let mut r = rig();
        r.verdict.set(0);
        let result = r.e.call(0x00df_6a60, &args![r.this, r.slot]).f32();
        assert_eq!(result, 2.5);
        assert_eq!(r.verdict_calls.borrow().len(), 1);
        assert!(r.handler_calls.borrow().is_empty());
    }

    #[test]
    fn the_current_slot_and_an_empty_one_skip_the_check() {
        let mut r = rig();
        r.e.mem.set_u32(r.this + 0x14, r.slot);
        assert_eq!(r.e.call(0x00df_6a60, &args![r.this, r.slot]).f32(), 2.5);
        r.e.mem.set_u32(r.this + 0x14, 0x1234);
        r.e.mem.set_u32(r.slot, 0);
        assert_eq!(r.e.call(0x00df_6a60, &args![r.this, r.slot]).f32(), 2.5);
        assert!(r.verdict_calls.borrow().is_empty());
        assert!(r.handler_calls.borrow().is_empty());
    }

    #[test]
    fn the_answer_is_quieted() {
        let mut r = rig();
        r.verdict.set(0);
        r.e.mem.set_u32(r.owner + 4, 0xff80_0001);
        let bits = fn_00df6a60(&mut r.e, Ptr::new(r.this), Ptr::new(r.slot)).to_bits();
        assert_eq!(bits, 0xffc0_0001);
    }

    #[test]
    fn the_thunks_forward_to_the_query() {
        let mut e = Engine::new();
        let log: CallLog = Rc::default();
        {
            let log = log.clone();
            e.register_double(QUERY, move |_, a| {
                log.borrow_mut().push(a.to_vec());
                Ret {
                    eax: 0x77,
                    st0: 4.5,
                    ..Ret::default()
                }
            });
        }
        let object = e.mem.alloc(0x20);
        e.mem.set_u8(object + 5, 0xfc);
        e.mem.set_u32(object + 0xc, 0x4321);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x1122_3344);
        let first = e.call(0x00df_6af0, &args![0x1000u32, object, 0u32]).f32();
        assert_eq!(first, 4.5);
        assert_eq!(e.call(0x00df_6b10, &args![0x2000u32, object]).f32(), 4.5);
        let third = e.call(0x00df_6b30, &args![0x3000u32, out, 0x5555u32]).u32();
        assert_eq!(third, out);
        assert_eq!(e.mem.u32(out), 0x1122_3300);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![0x1000, object - 4],
                vec![0x2000 - 4, 0x4321],
                vec![0x3000 - 8, 0x5555],
            ]
        );
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::units::havok_sdk::hkpworld::profiling_support::{labels, profiling_engine};
    type CalleeLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A job with `budgets.len()` 0x30-byte entries (budget = the word at +0x20, kept word at +0x24)
    /// and the three job words; doubles for the three callees log (callee, words and the kept cell
    /// value) and answer 3, 4 and 5.
    fn job_with_entries(e: &mut Engine, budgets: &[u32], words: [u32; 3]) -> (u32, u32, CalleeLog) {
        let job = e.mem.alloc(0x40);
        let entries = e.mem.alloc(0x30 * budgets.len().max(1) as u32);
        e.mem.set_u32(job + 0x2c, entries);
        e.mem.set_u32(job + 0x30, budgets.len() as u32);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(job + 0x20 + 4 * i as u32, *word);
        }
        for (i, budget) in budgets.iter().enumerate() {
            let entry = entries + 0x30 * i as u32;
            e.mem.set_u32(entry + 0x20, *budget);
            e.mem.set_u32(entry + 0x24, 0x500 + i as u32);
            e.mem.set_u32(entry + 0x28, 0xdead);
        }
        let log: CalleeLog = Rc::new(RefCell::new(Vec::new()));
        for (address, used) in [(0x00d1_d540u32, 3u32), (0x00d1_d4d0, 4), (0x00d1_b930, 5)] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |e, a| {
                sink.borrow_mut()
                    .push((address, vec![a[0], a[1], a[3], e.mem.u32(a[2])]));
                Ret {
                    eax: used,
                    ..Ret::default()
                }
            });
        }
        (job, entries, log)
    }

    /// Each entry gets the three calls in order with the shrinking budget; the first two answers are
    /// summed into the entry's +0x28 and taken off the budget, the third is only added.
    #[test]
    fn fn_00df6850_walks_the_entries_and_accumulates() {
        let (mut e, stream, area) = profiling_engine(4);
        let (job, entries, log) = job_with_entries(&mut e, &[100, 50], [0x11, 0x22, 0x33]);
        e.register_double(0x00d0_4e80, |_, a| Ret {
            eax: a[1] + a[2] + a[3],
            ..Ret::default()
        });
        let result = e.call(0x00df_6850, &args![0x4000u32, job]).u32();
        assert_eq!(result, job + job);
        assert_eq!(
            *log.borrow(),
            vec![
                (0x00d1_d540, vec![0x11, entries, 100, 0x500]),
                (0x00d1_d4d0, vec![0x22, entries, 97, 0x500]),
                (0x00d1_b930, vec![0x33, entries, 93, 0x500]),
                (0x00d1_d540, vec![0x11, entries + 0x30, 50, 0x501]),
                (0x00d1_d4d0, vec![0x22, entries + 0x30, 47, 0x501]),
                (0x00d1_b930, vec![0x33, entries + 0x30, 43, 0x501]),
            ]
        );
        assert_eq!(e.mem.u32(entries + 0x28), 12);
        assert_eq!(e.mem.u32(entries + 0x30 + 0x28), 12);
        assert_eq!(labels(&e, stream, area), vec![0x010d_b904, 0x010c_41f8]);
    }

    /// Job words that are 0 skip their call; no entries means no calls at all but both timer records.
    #[test]
    fn fn_00df6850_skips_null_job_words_and_empty_arrays() {
        let (mut e, stream, area) = profiling_engine(4);
        let (job, entries, log) = job_with_entries(&mut e, &[10], [0, 0x22, 0]);
        e.register_double(0x00d0_4e80, |_, _| Ret::default());
        e.call(0x00df_6850, &args![0u32, job]);
        assert_eq!(
            *log.borrow(),
            vec![(0x00d1_d4d0, vec![0x22, entries, 10, 0x500])]
        );
        assert_eq!(e.mem.u32(entries + 0x28), 4);

        let (job, _, log) = job_with_entries(&mut e, &[], [0x11, 0x22, 0x33]);
        e.call(0x00df_6850, &args![0u32, job]);
        assert!(log.borrow().is_empty());
        assert_eq!(
            labels(&e, stream, area),
            vec![0x010d_b904, 0x010c_41f8, 0x010d_b904, 0x010c_41f8]
        );
    }

    /// A full profiling stream (cursor at the limit) gets no record.
    #[test]
    fn fn_00df6850_writes_no_timer_record_into_a_full_stream() {
        let (mut e, _, area) = profiling_engine(0);
        let (job, _, _) = job_with_entries(&mut e, &[], [0, 0, 0]);
        e.register_double(0x00d0_4e80, |_, _| Ret::default());
        e.call(0x00df_6850, &args![0u32, job]);
        assert_eq!(e.mem.u32(area), 0);
    }
    /// A job for 00df6950: `budgets.len()` entries at +0x24 (count at +0x28) and `pointers` at +0x30
    /// (length at +0x2c); the callee at 00d1b930 answers 5 and logs its words (and the cell value).
    fn pointer_job(
        e: &mut Engine,
        budgets: &[u32],
        pointers: &[u32],
    ) -> (u32, u32, Rc<RefCell<Vec<Vec<u32>>>>) {
        let job = e.mem.alloc(0x40 + 4 * pointers.len() as u32);
        let entries = e.mem.alloc(0x30 * budgets.len().max(1) as u32);
        e.mem.set_u32(job + 0x24, entries);
        e.mem.set_u32(job + 0x28, budgets.len() as u32);
        e.mem.set_u32(job + 0x2c, pointers.len() as u32);
        for (i, pointer) in pointers.iter().enumerate() {
            e.mem.set_u32(job + 0x30 + 4 * i as u32, *pointer);
        }
        for (i, budget) in budgets.iter().enumerate() {
            let entry = entries + 0x30 * i as u32;
            e.mem.set_u32(entry + 0x20, *budget);
            e.mem.set_u32(entry + 0x24, 0x500 + i as u32);
            e.mem.set_u32(entry + 0x28, 0xdead);
        }
        let log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(0x00d1_b930, move |e, a| {
            sink.borrow_mut()
                .push(vec![a[0], a[1], a[3], e.mem.u32(a[2])]);
            Ret {
                eax: 5,
                ..Ret::default()
            }
        });
        e.register_double(0x00d0_4e80, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        (job, entries, log)
    }

    /// The pointers are tried in order for each entry; null ones are skipped; the budget shrinks.
    #[test]
    fn fn_00df6950_calls_the_helper_for_each_pointer_of_each_entry() {
        let (mut e, stream, area) = profiling_engine(2);
        let (job, entries, log) = pointer_job(&mut e, &[100, 7], &[0x11, 0, 0x33]);
        assert_eq!(e.call(0x00df_6950, &args![0x4000u32, job]).u32(), job);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![0x11, entries, 100, 0x500],
                vec![0x33, entries, 95, 0x500],
                vec![0x11, entries + 0x30, 7, 0x501],
                vec![0x33, entries + 0x30, 2, 0x501],
            ]
        );
        assert_eq!(e.mem.u32(entries + 0x28), 10);
        assert_eq!(e.mem.u32(entries + 0x30 + 0x28), 10);
        assert_eq!(labels(&e, stream, area), vec![0x010d_b918, 0x010c_41f8]);
    }

    /// No entries or no pointers leave the entries' words alone (zeroed or not) and call nothing.
    #[test]
    fn fn_00df6950_with_no_pointers_only_clears_the_totals() {
        let (mut e, _, _) = profiling_engine(2);
        let (job, entries, log) = pointer_job(&mut e, &[9], &[]);
        e.call(0x00df_6950, &args![0u32, job]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(entries + 0x28), 0);
        let (job, _, log) = pointer_job(&mut e, &[], &[0x11]);
        e.call(0x00df_6950, &args![0u32, job]);
        assert!(log.borrow().is_empty());
    }
}
