//! `BSHavok/run_00c6b540` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The three calls every path ends with (all thiscall on the object, no words), the call
/// that is the only one on the "flag clear" paths (`00c68a40`), and the call that is given
/// the last chunk of the list at `+0x2C` (`00c674d0`, `(this, chunk, count)`). All are
/// `bhkworld.obj` functions (per the exe's map).
const CALL_00C68A40: u32 = 0x00c6_8a40;
const CALL_00C676E0: u32 = 0x00c6_76e0;
const CALL_00C6AAC0: u32 = 0x00c6_aac0;
const CALL_00C67850: u32 = 0x00c6_7850;
const CALL_00C674D0: u32 = 0x00c6_74d0;
/// The flag byte at `012677a2` that records the list has been trimmed once, and the
/// limit word at `011afe80`.
const TRIMMED_FLAG: u32 = 0x0126_77a2;
const TRIM_LIMIT: u32 = 0x011a_fe80;

// Translated from 00c6b540 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (called by the exterior cell attach / detach code with the Havok world and
/// a 0 flag, which described it as taking one off the counter at `+0x18`). `flag`'s low
/// byte is the only part read. With the counter at `+0x18` already 0 it only runs
/// `00c68a40` when the list size at `+0x30` is not 0 and the flag is clear. Otherwise it
/// takes one off the counter and, when that makes it 0: with the flag clear runs
/// `00c68a40`; with the flag set and a non-empty list that has not been trimmed (the
/// byte at `012677a2` is 0) caps the size at 3000, hands the last `min(size, limit)`
/// entries of the list at `+0x2C` to `00c674d0` (`limit` is the word at `011afe80`),
/// shortens the size by that many and sets the byte. Then `00c676e0`, `00c6aac0` and
/// `00c67850` run. The class is unconfirmed.
pub fn fn_00c6b540(e: &mut Engine, this: Ptr, flag: u32) {
    let me = this.addr();
    let counter = e.mem.u32(me + 0x18);
    if counter == 0 {
        if e.mem.u32(me + 0x30) != 0 && flag as u8 == 0 {
            e.call(CALL_00C68A40, &args![this]);
        }
        return;
    }
    let counter = counter - 1;
    e.mem.set_u32(me + 0x18, counter);
    if counter != 0 {
        return;
    }
    if flag as u8 == 0 {
        e.call(CALL_00C68A40, &args![this]);
    } else {
        let size = e.mem.u32(me + 0x30);
        if size != 0 && e.mem.u8(TRIMMED_FLAG) == 0 {
            if size >= 0xbb8 {
                e.mem.set_u32(me + 0x30, 0xbb8);
            }
            let size = e.mem.u32(me + 0x30);
            let count = size.min(e.mem.u32(TRIM_LIMIT));
            let rest = size - count;
            let list = e.mem.u32(me + 0x2c);
            let chunk = list.wrapping_add(rest.wrapping_mul(4));
            e.call(CALL_00C674D0, &args![this, chunk, count]);
            e.mem.set_u32(me + 0x30, rest);
            e.mem.set_u8(TRIMMED_FLAG, 1);
        }
    }
    e.call(CALL_00C676E0, &args![this]);
    e.call(CALL_00C6AAC0, &args![this]);
    e.call(CALL_00C67850, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c6_b540, fn_00c6b540(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::super::bhklimitedhingeconstraint::support::*;
    use super::*;
    use std::rc::Rc;

    fn world() -> (Engine, Calls) {
        let mut e = Engine::new();
        e.map(0x0126_7000, 0x1000);
        e.map(0x011a_f000, 0x1000);
        e.set_global(TRIM_LIMIT, 10u32);
        let log: Calls = Rc::default();
        for address in [
            CALL_00C68A40,
            CALL_00C676E0,
            CALL_00C6AAC0,
            CALL_00C67850,
            CALL_00C674D0,
        ] {
            record(&mut e, address, 0, &log);
        }
        (e, log)
    }

    fn order(log: &Calls) -> Vec<u32> {
        log.borrow().iter().map(|c| c.0).collect()
    }

    #[test]
    fn a_zero_counter_only_flushes_for_a_clear_flag_and_a_non_empty_list() {
        let (mut e, log) = world();
        let this = e.mem.alloc(0x40);
        e.call(0x00c6_b540, &args![this, 0u32]);
        assert!(order(&log).is_empty());
        e.mem.set_u32(this + 0x30, 5);
        e.call(0x00c6_b540, &args![this, 1u32]);
        assert!(order(&log).is_empty());
        // Only the low byte of the flag counts.
        e.call(0x00c6_b540, &args![this, 0x100u32]);
        assert_eq!(order(&log), [CALL_00C68A40]);
        assert_eq!(calls_to(&log, CALL_00C68A40), [[this]]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
    }

    #[test]
    fn a_counter_above_one_is_only_counted_down() {
        let (mut e, log) = world();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 2);
        e.call(0x00c6_b540, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x18), 1);
        assert!(order(&log).is_empty());
    }

    #[test]
    fn the_last_count_with_a_clear_flag_flushes_and_runs_the_three_calls() {
        let (mut e, log) = world();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 1);
        e.mem.set_u32(this + 0x30, 5);
        e.call(0x00c6_b540, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(
            order(&log),
            [CALL_00C68A40, CALL_00C676E0, CALL_00C6AAC0, CALL_00C67850]
        );
        assert_eq!(e.mem.u32(this + 0x30), 5);
    }

    #[test]
    fn the_last_count_with_the_flag_set_trims_the_list_once() {
        let (mut e, log) = world();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 1);
        e.mem.set_u32(this + 0x2c, 0x7000);
        e.mem.set_u32(this + 0x30, 4000);
        e.call(0x00c6_b540, &args![this, 1u32]);
        // The size is capped at 3000, the last 10 entries are handed over.
        assert_eq!(
            calls_to(&log, CALL_00C674D0),
            [[this, 0x7000 + (3000 - 10) * 4, 10]]
        );
        assert_eq!(e.mem.u32(this + 0x30), 2990);
        assert_eq!(e.mem.u8(TRIMMED_FLAG), 1);
        assert_eq!(
            order(&log),
            [CALL_00C674D0, CALL_00C676E0, CALL_00C6AAC0, CALL_00C67850]
        );
        // Already trimmed: only the three calls run.
        e.mem.set_u32(this + 0x18, 1);
        e.call(0x00c6_b540, &args![this, 1u32]);
        assert_eq!(calls_to(&log, CALL_00C674D0).len(), 1);
        assert_eq!(order(&log).len(), 7);
        // A short list is handed over whole, and an empty one is left alone.
        e.mem.set_u8(TRIMMED_FLAG, 0);
        e.mem.set_u32(this + 0x18, 1);
        e.mem.set_u32(this + 0x30, 4);
        e.call(0x00c6_b540, &args![this, 1u32]);
        assert_eq!(calls_to(&log, CALL_00C674D0)[1], [this, 0x7000, 4]);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        e.mem.set_u8(TRIMMED_FLAG, 0);
        e.mem.set_u32(this + 0x18, 1);
        e.call(0x00c6_b540, &args![this, 1u32]);
        assert_eq!(calls_to(&log, CALL_00C674D0).len(), 2);
        assert_eq!(e.mem.u8(TRIMMED_FLAG), 0);
    }
}
