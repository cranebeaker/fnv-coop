//! `fallout/ai/run_00982b20` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00865e50` (`thiscall(address, size, word)` on a save buffer) and
/// `00864980` (`thiscall(address, size)` on a load buffer).
const BUFFER_WRITE: u32 = 0x0086_5e50;
const BUFFER_READ: u32 = 0x0086_4980;

// Translated from 00982b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The smaller of two bytes (cdecl, the low byte of each word).
pub fn fn_00982b20(_e: &mut Engine, first: u32, second: u32) -> u8 {
    let first = first as u8;
    let second = second as u8;
    if first < second {
        first
    } else {
        second
    }
}

// Translated from 00982b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the six bytes of `this` (in the exe's order: 2, 3, 0, 1, 4, 5).
pub fn fn_00982b50(e: &mut Engine, this: Ptr) {
    for offset in [2, 3, 0, 1, 4, 5] {
        e.mem.set_u8(this.addr() + offset, 0);
    }
}

// Translated from 00982b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A bit mask: `byte 2` ones shifted left by `byte 3`
/// (`((2 << (byte 2 - 1)) - 1) << byte 3`, the shift counts taken mod 32
/// as `SHL` does).
pub fn fn_00982b90(e: &mut Engine, this: Ptr) -> u32 {
    let width = u32::from(e.mem.u8(this.addr() + 2)).wrapping_sub(1);
    let shift = u32::from(e.mem.u8(this.addr() + 3));
    2u32.wrapping_shl(width).wrapping_sub(1).wrapping_shl(shift)
}

// Translated from 00982bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x15.
pub fn fn_00982bc0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x15)
}

// Translated from 00982be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the byte at +0x4E is set and `006e4820` does not hold.
pub fn fn_00982be0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x4e) != 0 && !e.call(0x006e_4820, &args![this]).bool()
}

// Translated from 00982c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the six bytes of `this` one at a time (`00865e50` on `buffer`,
/// size 1, 0).
pub fn fn_00982c20(e: &mut Engine, this: Ptr, buffer: u32) {
    for offset in 0..6 {
        e.call(
            BUFFER_WRITE,
            &args![buffer, this.addr() + offset, 1u32, 0u32],
        );
    }
}

// Translated from 00982ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the six bytes of `this` one at a time (`00864980` on `buffer`,
/// size 1).
pub fn fn_00982ca0(e: &mut Engine, this: Ptr, buffer: u32) {
    for offset in 0..6 {
        e.call(BUFFER_READ, &args![buffer, this.addr() + offset, 1u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0098_2b20, fn_00982b20(u32, u32) -> u8),
        entry!(0x0098_2b50, fn_00982b50(Ptr)),
        entry!(0x0098_2b90, fn_00982b90(Ptr) -> u32),
        entry!(0x0098_2bc0, fn_00982bc0(Ptr) -> u8),
        entry!(0x0098_2be0, fn_00982be0(Ptr) -> bool),
        entry!(0x0098_2c20, fn_00982c20(Ptr, u32)),
        entry!(0x0098_2ca0, fn_00982ca0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00982b20_answers_the_smaller_byte() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0098_2b20, &[0x1ff, 0x10]).u8(), 0x10);
        assert_eq!(e.call(0x0098_2b20, &[3, 0x204]).u8(), 3);
    }

    #[test]
    fn the_byte_fields_clear_mask_and_read() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        e.mem.write(this, &[1, 2, 3, 4, 5, 6, 7]);
        e.call(0x0098_2b50, &[this]);
        assert_eq!(e.mem.bytes(this, 7), vec![0, 0, 0, 0, 0, 0, 7]);
        e.mem.set_u8(this + 2, 3);
        e.mem.set_u8(this + 3, 4);
        assert_eq!(e.call(0x0098_2b90, &[this]).u32(), 0x70);
        e.mem.set_u8(this + 2, 0);
        e.mem.set_u8(this + 3, 0);
        assert_eq!(e.call(0x0098_2b90, &[this]).u32(), 0xffff_ffff);
        e.mem.set_u8(this + 0x15, 9);
        assert_eq!(e.call(0x0098_2bc0, &[this]).u8(), 9);
    }

    #[test]
    fn fn_00982be0_needs_the_byte_and_a_failed_test() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        e.register(0x006e_4820, |_, _| Ret::default());
        assert!(!e.call(0x0098_2be0, &[this]).bool());
        e.mem.set_u8(this + 0x4e, 1);
        assert!(e.call(0x0098_2be0, &[this]).bool());
        e.register(0x006e_4820, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        assert!(!e.call(0x0098_2be0, &[this]).bool());
    }

    #[test]
    fn the_six_bytes_are_saved_and_loaded_one_by_one() {
        let mut e = Engine::new();
        e.register(BUFFER_WRITE, |_, _| Ret::default());
        e.register(BUFFER_READ, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0098_2c20, &[0x9000, 0xb0f0]);
        e.call(0x0098_2ca0, &[0x9000, 0xb0f1]);
        let writes = calls_to(&e, BUFFER_WRITE);
        assert_eq!(writes.len(), 6);
        assert_eq!(writes[5], vec![0xb0f0, 0x9005, 1, 0]);
        let reads = calls_to(&e, BUFFER_READ);
        assert_eq!(reads[0], vec![0xb0f1, 0x9000, 1]);
        assert_eq!(reads[5], vec![0xb0f1, 0x9005, 1]);
    }
}
