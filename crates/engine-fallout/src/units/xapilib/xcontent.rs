//! `XAPILIB/xcontent.c` (Xbox PDB source unit), subsystem `XAPILIB`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The callees: `0084fec0` (`thiscall`, two words: a word and a buffer), `0084ff30`
/// (`thiscall`, one word: a buffer), `00406d30` and `00406d50` (cdecl, three words: a
/// destination, its size and a source string), and the string at `0107fac4`.
const CALLEE_0084FEC0: u32 = 0x0084_fec0;
const CALLEE_0084FF30: u32 = 0x0084_ff30;
const CALLEE_00406D30: u32 = 0x0040_6d30;
const CALLEE_00406D50: u32 = 0x0040_6d50;
const SUFFIX_0107FAC4: u32 = 0x0107_fac4;

// Translated from 0084ff90 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0084ff90 - the /GS security cookie check (00ec408c) is not translated.
/// The engine map names it `XContentClose`, which its body does not fit, so it keeps its
/// address (`thiscall`, three words: a word, a destination of 0x104 bytes and a flag whose low
/// byte counts, `RET 0xc`). `0084fec0` fills a first local buffer from the word and `0084ff30`
/// a second; the destination gets the second (`00406d30` with 0x104) and then the first
/// (`00406d50` with 0x104), and with the flag set also the string at `0107fac4` (`00406d50`).
/// The two buffers keep the exe's frame offsets (`EBP - 0x218` and `EBP - 0x110` of a
/// 0x21c-byte frame).
pub fn fn_0084ff90(e: &mut Engine, this: u32, word: u32, destination: u32, flag: u32) {
    e.with_stack(0x21c, |e, frame| {
        let first = frame.addr().wrapping_add(0x21c - 0x218);
        let second = frame.addr().wrapping_add(0x21c - 0x110);
        e.call(CALLEE_0084FEC0, &args![this, word, first]);
        e.call(CALLEE_0084FF30, &args![this, second]);
        e.call(CALLEE_00406D30, &args![destination, 0x104u32, second]);
        e.call(CALLEE_00406D50, &args![destination, 0x104u32, first]);
        if flag as u8 != 0 {
            e.call(
                CALLEE_00406D50,
                &args![destination, 0x104u32, SUFFIX_0107FAC4],
            );
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0084_ff90, fn_0084ff90(u32, u32, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_destination_gets_both_buffers_and_the_suffix_with_the_flag() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(CALLEE_0084FEC0, |e, a| {
            e.mem.write(a[2], b"name\0");
            Ret::default()
        });
        e.register(CALLEE_0084FF30, |e, a| {
            e.mem.write(a[1], b"root\\\0");
            Ret::default()
        });
        // Doubles: 00406d30 copies, 00406d50 appends.
        e.register(CALLEE_00406D30, |e, a| {
            let s = e.mem.cstr(a[2]);
            e.mem.write(a[0], &s);
            e.mem.set_u8(a[0] + s.len() as u32, 0);
            Ret::default()
        });
        e.register(CALLEE_00406D50, |e, a| {
            let mut s = e.mem.cstr(a[0]);
            s.extend(e.mem.cstr(a[2]));
            e.mem.write(a[0], &s);
            e.mem.set_u8(a[0] + s.len() as u32, 0);
            Ret::default()
        });
        e.map(SUFFIX_0107FAC4, 4);
        e.mem.write(SUFFIX_0107FAC4, b".x\0");
        let out = e.mem.alloc(0x104);
        e.call(0x0084_ff90, &args![0x100u32, 7u32, out, 0x100u32]);
        assert_eq!(e.mem.cstr(out), b"root\\name".to_vec());
        fn_0084ff90(&mut e, 0x100, 7, out, 1);
        assert_eq!(e.mem.cstr(out), b"root\\name.x".to_vec());
        let log = e.call_log.take().unwrap();
        let firsts: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0084FEC0).collect();
        assert_eq!(firsts[0].1[..2], [0x100, 7]);
        let seconds: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0084FF30).collect();
        assert_eq!(seconds[0].1[0], 0x100);
    }
}
