//! `Havok SDK/run_00dfaf80` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `00dfaf80` is called from `hkCalcMultiPenetrationDepth` (engine map name); none of the three
//! has a map name of its own, so they are named by address.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00e2d440`: called on `this` with no arguments by `00dfaf80`.
const SET_UP_AFTER_FILL: u32 = 0x00e2_d440;
/// `00e63af0` (`this`, `(arg, first float out, second float out)`) and `00e2d0d0` (`this`,
/// `(object, arg, out, out)`), called by `00dfb040`.
const PROBE: u32 = 0x00e6_3af0;
const SOLVE: u32 = 0x00e2_d0d0;
/// The two `float` constants `00dfb040` stores in the first two words of its `this`.
const FIRST_LIMIT: u32 = 0x010d_bb3c;
const SECOND_LIMIT: u32 = 0x0104_da48;

// Translated from 00dfaf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills a 0x114-byte record from three 4x4 matrices (`a`, `b`, `c`; each of four 16-byte
/// columns): word +8 gets 4; column `k` of `a` goes to `0x30 + 0x40 * k`, of `b` to `0x40 +
/// 0x40 * k` and of `c` to `0x20 + 0x40 * k`; the words at `0x50 + 0x40 * k` get 0; then
/// calls `00e2d440(this)` and clears the word at +0xC.
pub fn fn_00dfaf80(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr, c: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 8, 4);
    for column in 0..4u32 {
        let slot = base + 0x40 * column;
        copy_column(e, slot + 0x30, a.addr() + 0x10 * column);
        copy_column(e, slot + 0x40, b.addr() + 0x10 * column);
        copy_column(e, slot + 0x20, c.addr() + 0x10 * column);
        e.mem.set_u32(slot + 0x50, 0);
    }
    e.call(SET_UP_AFTER_FILL, &args![this]);
    e.mem.set_u32(base + 0xc, 0);
}

/// Copies one 16-byte column as four words.
fn copy_column(e: &mut Engine, to: u32, from: u32) {
    for offset in (0..0x10).step_by(4) {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
}

// Translated from 00dfb040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the two words `out_a` and `out_b` point to, stores the two `float` constants at
/// `010dbb3c` and `0104da48` (through the x87) in `this` and `this + 4`, asks `00e63af0`
/// (`object`, `(arg, this, this + 4)`) and keeps its answer in `object + 0x44`. When the
/// answer is 1 returns `00e2d0d0(this, (object, arg, out_a, out_b))`, else 2.
pub fn fn_00dfb040(
    e: &mut Engine,
    this: Ptr,
    object: Ptr,
    arg: u32,
    out_a: Ptr,
    out_b: Ptr,
) -> u32 {
    e.mem.set_u32(out_b.addr(), 0);
    e.mem.set_u32(out_a.addr(), 0);
    let first = x87_quiet_bits(e.global::<u32>(FIRST_LIMIT));
    e.mem.set_u32(this.addr(), first);
    let second = x87_quiet_bits(e.global::<u32>(SECOND_LIMIT));
    e.mem.set_u32(this.addr() + 4, second);
    let answer = e
        .call(PROBE, &args![object, arg, this, this.addr() + 4])
        .u32();
    e.mem.set_u32(object.addr() + 0x44, answer);
    if answer == 1 {
        e.call(SOLVE, &args![this, object, arg, out_a, out_b]).u32()
    } else {
        2
    }
}

// Translated from 00dfb0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Of the `count` (word +0x10) 0x50-byte entries starting at `this + 0xF70`, returns the
/// first one with the smallest `float` at its offset +0x40 (a later entry replaces the best
/// only when its value is strictly smaller; a NaN never does).
pub fn fn_00dfb0b0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr() + 0xf70;
    let count = e.mem.i32(this.addr() + 0x10);
    let mut best = base;
    let mut smallest = x87_quiet(e.mem.f32(this.addr() + 0xfb0));
    for index in 1..count.max(1) {
        let entry = base.wrapping_add(0x50 * index as u32);
        let value = x87_quiet(e.mem.f32(entry + 0x40));
        if value < smallest {
            smallest = value;
            best = entry;
        }
    }
    Ptr::new(best)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_af80, fn_00dfaf80(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00df_b040, fn_00dfb040(Ptr, Ptr, u32, Ptr, Ptr) -> u32),
        entry!(0x00df_b0b0, fn_00dfb0b0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_copies_the_columns_to_their_slots() {
        let mut e = Engine::new();
        let record = e.mem.alloc(0x120);
        for offset in (0..0x120).step_by(4) {
            e.mem.set_u32(record + offset, 0xdead_beef);
        }
        let matrices: Vec<u32> = (0..3u32)
            .map(|m| {
                let at = e.mem.alloc(0x40);
                for offset in (0..0x40).step_by(4) {
                    e.mem.set_u32(at + offset, 0x1000 * (m + 1) + offset);
                }
                at
            })
            .collect();
        e.register_double(SET_UP_AFTER_FILL, move |e, a| {
            // the call happens before +0xC is cleared
            e.mem.set_u32(a[0] + 0xc, 0x77);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(
            0x00df_af80,
            &args![record, matrices[0], matrices[1], matrices[2]],
        );
        assert_eq!(e.mem.u32(record + 8), 4);
        assert_eq!(e.mem.u32(record + 0xc), 0);
        for column in 0..4u32 {
            let slot = record + 0x40 * column;
            for offset in (0..0x10).step_by(4) {
                let source = 0x10 * column + offset;
                assert_eq!(e.mem.u32(slot + 0x30 + offset), 0x1000 + source);
                assert_eq!(e.mem.u32(slot + 0x40 + offset), 0x2000 + source);
                assert_eq!(e.mem.u32(slot + 0x20 + offset), 0x3000 + source);
            }
            assert_eq!(e.mem.u32(slot + 0x50), 0);
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == SET_UP_AFTER_FILL).count(), 1);
    }

    fn probe_rig(e: &mut Engine, answer: u32) -> (u32, u32, u32, u32) {
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(FIRST_LIMIT, 0x33d6_bf95);
        e.mem.set_u32(SECOND_LIMIT, 0x7f80_0001);
        let this = e.mem.alloc(0x10);
        let object = e.mem.alloc(0x60);
        let out_a = e.mem.alloc(4);
        let out_b = e.mem.alloc(4);
        e.mem.set_u32(out_a, 5);
        e.mem.set_u32(out_b, 6);
        e.register_double(PROBE, move |_, _| Ret {
            eax: answer,
            ..Ret::default()
        });
        e.register(SOLVE, |_, a| Ret {
            eax: 0x40 + a.len() as u32,
            ..Ret::default()
        });
        (this, object, out_a, out_b)
    }

    #[test]
    fn probe_one_runs_the_solver() {
        let mut e = Engine::new();
        let (this, object, out_a, out_b) = probe_rig(&mut e, 1);
        e.call_log = Some(vec![]);
        let got = e
            .call(0x00df_b040, &args![this, object, 0x99u32, out_a, out_b])
            .u32();
        assert_eq!(got, 0x45);
        assert_eq!(e.mem.u32(out_a), 0);
        assert_eq!(e.mem.u32(out_b), 0);
        assert_eq!(e.mem.u32(this), 0x33d6_bf95);
        assert_eq!(e.mem.u32(this + 4), 0x7fc0_0001);
        assert_eq!(e.mem.u32(object + 0x44), 1);
        let log = e.call_log.take().unwrap();
        let probe: Vec<_> = log.iter().filter(|c| c.0 == PROBE).collect();
        assert_eq!(probe[0].1, vec![object, 0x99, this, this + 4]);
        let solve: Vec<_> = log.iter().filter(|c| c.0 == SOLVE).collect();
        assert_eq!(solve[0].1, vec![this, object, 0x99, out_a, out_b]);
    }

    #[test]
    fn probe_other_than_one_returns_two_without_solving() {
        let mut e = Engine::new();
        let (this, object, out_a, out_b) = probe_rig(&mut e, 3);
        e.register(SOLVE, |_, _| panic!("solver called"));
        let got = e
            .call(0x00df_b040, &args![this, object, 0u32, out_a, out_b])
            .u32();
        assert_eq!(got, 2);
        assert_eq!(e.mem.u32(object + 0x44), 3);
    }

    fn entries(e: &mut Engine, count: i32, values: &[f32]) -> u32 {
        let this = e.mem.alloc(0x1400);
        e.mem.set_i32(this + 0x10, count);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(this + 0xf70 + 0x50 * i as u32 + 0x40, *v);
        }
        this
    }

    #[test]
    fn minimum_entry_is_the_first_strictly_smaller() {
        let mut e = Engine::new();
        let this = entries(&mut e, 6, &[3.0, 2.0, 2.0, 5.0, 1.0, 1.0]);
        let got = e.call(0x00df_b0b0, &args![this]).u32();
        assert_eq!(got, this + 0xf70 + 0x50 * 4);
        let this = entries(&mut e, 4, &[1.0, 2.0, 1.0, 0.5]);
        assert_eq!(
            e.call(0x00df_b0b0, &args![this]).u32(),
            this + 0xf70 + 0x50 * 3
        );
    }

    #[test]
    fn minimum_entry_with_few_entries_or_nan_stays_on_the_first() {
        let mut e = Engine::new();
        let this = entries(&mut e, 1, &[9.0]);
        assert_eq!(e.call(0x00df_b0b0, &args![this]).u32(), this + 0xf70);
        let this = entries(&mut e, 0, &[9.0]);
        assert_eq!(e.call(0x00df_b0b0, &args![this]).u32(), this + 0xf70);
        let this = entries(&mut e, 3, &[f32::NAN, 1.0, 0.0]);
        assert_eq!(e.call(0x00df_b0b0, &args![this]).u32(), this + 0xf70);
    }
}
