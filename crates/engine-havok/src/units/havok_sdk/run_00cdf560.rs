//! `Havok SDK/run_00cdf560` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable word `00cdf560` writes (`010cb798`).
const OBJECT_VTABLE: u32 = 0x010c_b798;

/// Copies the `float` at `source` to `destination` through the x87 stack (`FLD`/`FSTP`): a
/// signalling NaN comes out quiet.
fn copy_float(e: &mut Engine, source: u32, destination: u32) {
    let bits = x87_quiet_bits(e.mem.u32(source));
    e.mem.set_u32(destination, bits);
}

// Translated from 00cdf560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (the class is unconfirmed; the `RET 4` takes one word it never reads): sets the
/// 16-bit word at `+6` to 1, the byte at `+8` to 0 and later to 1, the vtable word, and copies
/// floats from the exe's data through the x87 stack: `010cb790` to `+0xc`, `0108cb00` to
/// `+0x10`, `01030ff0` to `+0x14`, `010162c0` to `+0x1c`; `1.0` goes to `+0x18` and `+0x20`.
/// Returns `this`.
pub fn fn_00cdf560(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u8(base + 8, 0);
    copy_float(e, 0x010c_b790, base + 0xc);
    e.mem.set_u32(base, OBJECT_VTABLE);
    copy_float(e, 0x0108_cb00, base + 0x10);
    e.mem.set_u8(base + 8, 1);
    copy_float(e, 0x0103_0ff0, base + 0x14);
    e.mem.set_u32(base + 0x18, 0x3f80_0000);
    e.mem.set_u32(base + 0x20, 0x3f80_0000);
    copy_float(e, 0x0101_62c0, base + 0x1c);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cd_f560, fn_00cdf560(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every default lands where the exe puts it (a signalling NaN source comes out quiet) and
    /// the other words are left alone.
    #[test]
    fn constructor_writes_the_defaults() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(0x010c_b790, 0xc974_2400);
        e.mem.set_u32(0x0108_cb00, 0x4974_2400);
        e.mem.set_u32(0x0103_0ff0, 0x7f80_0001);
        e.mem.set_u32(0x0101_62c0, 0x4000_0000);
        let this = e.mem.alloc(0x30);
        e.mem.write(this, &[0xaa; 0x30]);
        let got = e.call(0x00cd_f560, &args![this, 0x1234u32]).u32();
        assert_eq!(got, this);
        let mut image = [0xaau8; 0x30];
        for (offset, word) in [
            (0x0, OBJECT_VTABLE),
            (0xc, 0xc974_2400),
            (0x10, 0x4974_2400),
            (0x14, 0x7fc0_0001),
            (0x18, 0x3f80_0000),
            (0x1c, 0x4000_0000),
            (0x20, 0x3f80_0000),
        ] {
            image[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
        }
        image[6] = 1;
        image[7] = 0;
        image[8] = 1;
        assert_eq!(e.mem.bytes(this, 0x30), image.to_vec());
    }
}
