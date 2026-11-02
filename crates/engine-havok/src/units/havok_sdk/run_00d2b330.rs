//! `Havok SDK/run_00d2b330` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d2b330 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, two words `source` and `buffer`; descriptions, not confirmed): calls
/// `00d61c10` on `buffer` with `source` (thiscall), then `00d61840` on `buffer` with
/// `(0x2f, 0x5c, 1)` (`'/'`, `'\\'`: the exe passes these two characters and a flag), and answers
/// the word at `buffer + 0` (the string pointer the buffer holds).
pub fn fn_00d2b330(e: &mut Engine, source: u32, buffer: Ptr) -> u32 {
    e.call(0x00d6_1c10, &args![buffer, source]);
    e.call(0x00d6_1840, &args![buffer, 0x2fu32, 0x5cu32, 1u32]);
    e.mem.u32(buffer.addr())
}

// Translated from 00d2b360 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words (cdecl, nothing popped): `text` and `buffer`, a
/// string buffer (data pointer at +0, size at +4, capacity and flags at +8). Runs on `buffer`
/// (`this` in `ECX`): `00d61c10(text)`, `00d61840(0x5c, 0x2f, 1)`, `00d62190(0x010ce598,
/// 0x01063cbc, 1)`, then `00d61560(0x010ce594)` and, when that answers non-zero, `00d617d0(2)`.
/// Returns the word at +0 of `buffer` (its data pointer). Ghidra made no function here; the address is stored in the data word at 011b3dc4.
pub fn fn_00d2b360(e: &mut Engine, text: u32, buffer: u32) -> u32 {
    e.call(0x00d6_1c10, &args![buffer, text]);
    e.call(0x00d6_1840, &args![buffer, 0x5cu32, 0x2fu32, 1u32]);
    e.call(
        0x00d6_2190,
        &args![buffer, 0x010c_e598u32, 0x0106_3cbcu32, 1u32],
    );
    if e.call(0x00d6_1560, &args![buffer, 0x010c_e594u32]).u32() != 0 {
        e.call(0x00d6_17d0, &args![buffer, 2u32]);
    }
    e.mem.u32(buffer)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_b330, fn_00d2b330(u32, Ptr) -> u32),
        entry!(0x00d2_b360, fn_00d2b360(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d2b330_sets_the_buffer_and_swaps_the_separators() {
        let mut e = Engine::new();
        // The first double gives the buffer its string pointer.
        e.register(0x00d6_1c10, |e, a| {
            e.mem.set_u32(a[0], 0x0123_4560 + a[1]);
            Ret::default()
        });
        e.register(0x00d6_1840, |_, _| Ret::default());
        let buffer = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let answer = e.call(0x00d2_b330, &args![7u32, buffer]).u32();
        assert_eq!(answer, 0x0123_4567);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x00d6_1c10, vec![buffer, 7]),
                (0x00d6_1840, vec![buffer, 0x2f, 0x5c, 1])
            ]
        );
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles for the five string functions; the one at `00d61560` answers `found`.
    fn string_doubles(e: &mut Engine, found: u32) -> CallLog {
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        for address in [
            0x00d6_1c10u32,
            0x00d6_1840,
            0x00d6_2190,
            0x00d6_1560,
            0x00d6_17d0,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax: if address == 0x00d6_1560 { found } else { 0 },
                    ..Ret::default()
                }
            });
        }
        log
    }

    /// Without a match the last call is the search and the buffer's data pointer comes back.
    #[test]
    fn fn_00d2b360_runs_the_string_steps_in_order() {
        let mut e = Engine::new();
        let log = string_doubles(&mut e, 0);
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(buffer, 0x7000);
        assert_eq!(e.call(0x00d2_b360, &args![0x1234u32, buffer]).u32(), 0x7000);
        assert_eq!(
            *log.borrow(),
            vec![
                (0x00d6_1c10, vec![buffer, 0x1234]),
                (0x00d6_1840, vec![buffer, 0x5c, 0x2f, 1]),
                (0x00d6_2190, vec![buffer, 0x010c_e598, 0x0106_3cbc, 1]),
                (0x00d6_1560, vec![buffer, 0x010c_e594]),
            ]
        );
    }

    /// A non-zero answer of the search is followed by the call with 2.
    #[test]
    fn fn_00d2b360_finishes_with_the_second_step_after_a_match() {
        let mut e = Engine::new();
        let log = string_doubles(&mut e, 1);
        let buffer = e.mem.alloc(0x10);
        e.call(0x00d2_b360, &args![0x1234u32, buffer]);
        assert_eq!(
            log.borrow().last().unwrap(),
            &(0x00d6_17d0, vec![buffer, 2])
        );
    }
}
