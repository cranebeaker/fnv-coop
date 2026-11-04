//! `(unplaced)/run_00521480` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005214d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in one of six words chosen by bits 0x1, 0x2 and 0x4 of the
/// byte `bits` (`RET 8`): 0x1 with 0x2 gives +0x26C (with 0x4) or +0x260;
/// 0x1 alone +0x268 (with 0x4) or +0x254; 0x2 alone +0x264 (with 0x4) or
/// +0x258; 0x4 alone +0x25C; none of them stores nothing. The class is not
/// confirmed.
pub fn fn_005214d0(e: &mut Engine, this: Ptr, bits: u8, value: u32) {
    let base = this.addr();
    let offset = if bits & 0x1 != 0 {
        if bits & 0x2 != 0 {
            if bits & 0x4 != 0 {
                0x26c
            } else {
                0x260
            }
        } else if bits & 0x4 != 0 {
            0x268
        } else {
            0x254
        }
    } else if bits & 0x2 != 0 {
        if bits & 0x4 != 0 {
            0x264
        } else {
            0x258
        }
    } else if bits & 0x4 != 0 {
        0x25c
    } else {
        return;
    };
    e.mem.set_u32(base + offset, value);
}

// Translated from 00521480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at +0x350 for `which` 1, +0x354 for 2, +0x358 for 4;
/// any other `which` stores nothing (`RET 8`).
pub fn fn_00521480(e: &mut Engine, this: Ptr, which: u32, value: u32) {
    let base = this.addr();
    match which {
        1 => e.mem.set_u32(base + 0x350, value),
        2 => e.mem.set_u32(base + 0x354, value),
        4 => e.mem.set_u32(base + 0x358, value),
        _ => {}
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0052_14d0, fn_005214d0(Ptr, u8, u32)),
        entry!(0x0052_1480, fn_00521480(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    /// The offsets the call wrote to (words that became nonzero).
    fn written(addr: u32, selector: u32) -> Vec<u32> {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x360);
        e.call(addr, &[this, selector, 0x1234]);
        (0..0x360)
            .step_by(4)
            .filter(|o| e.mem.u32(this + o) != 0)
            .inspect(|o| assert_eq!(e.mem.u32(this + o), 0x1234))
            .collect()
    }

    #[test]
    fn at_005214d0_picks_the_word_by_bits() {
        for (bits, expected) in [
            (7u32, vec![0x26c]),
            (3, vec![0x260]),
            (5, vec![0x268]),
            (1, vec![0x254]),
            (6, vec![0x264]),
            (2, vec![0x258]),
            (4, vec![0x25c]),
            (0, vec![]),
            (0xf8, vec![]),
            (0x107, vec![0x26c]),
        ] {
            assert_eq!(written(0x0052_14d0, bits), expected, "bits {bits:#x}");
        }
    }

    #[test]
    fn at_00521480_picks_the_word_by_value() {
        for (which, expected) in [
            (1u32, vec![0x350]),
            (2, vec![0x354]),
            (4, vec![0x358]),
            (3, vec![]),
            (0, vec![]),
            (0x101, vec![]),
        ] {
            assert_eq!(written(0x0052_1480, which), expected, "which {which:#x}");
        }
    }
}
