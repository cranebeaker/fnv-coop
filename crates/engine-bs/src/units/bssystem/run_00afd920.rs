//! `BSSystem/run_00afd920` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00afdbc0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): whether `address` lies in the
/// block that starts at the word +0xC and holds (word +0x8) 16-byte entries (unsigned
/// compares, the end `start + count * 16` wrapping as the exe's `SHL`/`ADD`).
pub fn fn_00afdbc0(e: &mut Engine, this: u32, address: u32) -> bool {
    let start = e.mem.u32(this + 0xc);
    if address < start {
        return false;
    }
    let end = (e.mem.u32(this + 0x8) << 4).wrapping_add(start);
    address < end
}

/// `00a62150` (cdecl, two words), called on the object's +4 with 1.
const CALLEE_00A62150: u32 = 0x00a6_2150;

// Translated from 00afd960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, no stack words): `00a62150(this + 4, 1)`, whose
/// answer (EAX) it passes on.
pub fn fn_00afd960(e: &mut Engine, this: u32) -> u32 {
    e.call(CALLEE_00A62150, &args![this.wrapping_add(4), 1u32])
        .u32()
}

/// The C runtime's byte swaps of a word (`00ec3bad`, `_byteswap_ulong`) and of a 16-bit value
/// (`00ec3bda`), cdecl, one word.
const BYTESWAP_ULONG: u32 = 0x00ec_3bad;
const BYTESWAP_USHORT: u32 = 0x00ec_3bda;

/// The word at `at` byte-swapped in place (`00ec3bad`).
fn swap_word(e: &mut Engine, at: u32) {
    let value = e.mem.u32(at);
    let swapped = e.call(BYTESWAP_ULONG, &args![value]).u32();
    e.mem.set_u32(at, swapped);
}

// Translated from 00afd920 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the word at +0 byte-swapped in place
/// (`00ec3bad`). The byte locals the exe sets to 0 first are never read.
pub fn fn_00afd920(e: &mut Engine, this: u32) {
    swap_word(e, this);
}

// Translated from 00afd980 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the eight bytes at +0 reversed in place
/// (both words read into a local first, then byte `i` takes the local's byte `7 - i`).
pub fn fn_00afd980(e: &mut Engine, this: u32) {
    let local = e.mem.bytes(this, 8);
    for i in 0..8u32 {
        e.mem.set_u8(this.wrapping_add(i), local[(7 - i) as usize]);
    }
}

// Translated from 00afd9d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the words at +0, +4 and +8
/// byte-swapped (`00ec3bad`), the word at +0xc through [`fn_00afd920`], the words at +0x10,
/// +0x14, +0x18 and +0x1c, and last the 16-bit value at +0x20 (`00ec3bda` on it
/// zero-extended, its low 16 bits stored).
pub fn fn_00afd9d0(e: &mut Engine, this: u32) {
    for offset in [0u32, 4, 8] {
        swap_word(e, this.wrapping_add(offset));
    }
    fn_00afd920(e, this.wrapping_add(0xc));
    for offset in [0x10u32, 0x14, 0x18, 0x1c] {
        swap_word(e, this.wrapping_add(offset));
    }
    let short = u32::from(e.mem.u16(this.wrapping_add(0x20)));
    let swapped = e.call(BYTESWAP_USHORT, &args![short]).u32() as u16;
    e.mem.set_u16(this.wrapping_add(0x20), swapped);
}

// Translated from 00afdb20 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the words at +8 and +0xc byte-swapped
/// (`00ec3bad`).
pub fn fn_00afdb20(e: &mut Engine, this: u32) {
    swap_word(e, this.wrapping_add(8));
    swap_word(e, this.wrapping_add(0xc));
}

// Translated from 00afdb80 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the word at +8 byte-swapped
/// (`00ec3bad`).
pub fn fn_00afdb80(e: &mut Engine, this: u32) {
    swap_word(e, this.wrapping_add(8));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_d920, fn_00afd920(u32)),
        entry!(0x00af_d980, fn_00afd980(u32)),
        entry!(0x00af_d9d0, fn_00afd9d0(u32)),
        entry!(0x00af_db20, fn_00afdb20(u32)),
        entry!(0x00af_db80, fn_00afdb80(u32)),
        entry!(0x00af_dbc0, fn_00afdbc0(u32, u32) -> bool),
        entry!(0x00af_d960, fn_00afd960(u32) -> u32),
    ]
}

#[cfg(test)]
mod n083_tests {
    use super::*;

    #[test]
    fn calls_on_the_word_after_the_vtable() {
        let mut e = Engine::new();
        e.register(CALLEE_00A62150, |_, _| 0x55u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00af_d960, &args![0x1000u32]).u32(), 0x55);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00A62150).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, [0x1004, 1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00afdbc0_tests_the_range() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x8, 3);
        e.mem.set_u32(this + 0xc, 0x1000);
        let inside = |e: &mut Engine, address: u32| e.call(0x00af_dbc0, &[this, address]).bool();
        assert!(!inside(&mut e, 0xfff));
        assert!(inside(&mut e, 0x1000));
        assert!(inside(&mut e, 0x102f));
        assert!(!inside(&mut e, 0x1030));
    }
}

#[cfg(test)]
mod swap_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        // 00ec3bda is in engine-fallout: a double that swaps the low 16 bits.
        e.register(BYTESWAP_USHORT, |_, a| {
            u32::from((a[0] as u16).swap_bytes()).into_ret()
        });
        e
    }

    #[test]
    fn the_word_swaps() {
        let mut e = engine();
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this, 0x1122_3344);
        e.call(0x00af_d920, &args![this]);
        assert_eq!(e.mem.u32(this), 0x4433_2211);
        e.mem.set_u32(this + 8, 0x0102_0304);
        e.mem.set_u32(this + 0xc, 0x0506_0708);
        e.call(0x00af_db80, &args![this]);
        assert_eq!(e.mem.u32(this + 8), 0x0403_0201);
        e.call(0x00af_db20, &args![this]);
        assert_eq!(e.mem.u32(this + 8), 0x0102_0304);
        assert_eq!(e.mem.u32(this + 0xc), 0x0807_0605);
    }

    #[test]
    fn eight_bytes_reverse() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[1, 2, 3, 4, 5, 6, 7, 8]);
        e.call(0x00af_d980, &args![this]);
        assert_eq!(e.mem.bytes(this, 8), vec![8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn the_record_swaps_eight_words_and_a_short() {
        let mut e = engine();
        let this = e.mem.alloc(0x24);
        for i in 0..8u32 {
            e.mem.set_u32(this + 4 * i, 0x0102_0300 + i);
        }
        e.mem.set_u32(this + 0x20, 0xaaaa_1234);
        e.call(0x00af_d9d0, &args![this]);
        for i in 0..8u32 {
            assert_eq!(e.mem.u32(this + 4 * i), (0x0102_0300 + i).swap_bytes());
        }
        assert_eq!(e.mem.u32(this + 0x20), 0xaaaa_3412);
    }
}
