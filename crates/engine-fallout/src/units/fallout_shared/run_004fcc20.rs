//! `fallout shared/run_004fcc20` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The float constants the initializer copies (`FLD dword`/`FSTP dword`):
/// `01013974` (1000.0 in the exe), `01022958` (10000.0) and `01016248`
/// (0.5).
const CONSTANT_AT_08: u32 = 0x0101_3974;
const CONSTANT_AT_0C: u32 = 0x0102_2958;
const CONSTANT_AT_30: u32 = 0x0101_6248;

// Translated from 004fcc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): fills a data
/// block of at least 0x54 bytes, in the exe's order: the word 0x10000 at +0,
/// 0.0 (`FLDZ`) at +4, the constants at `01013974` and `01022958` at +8 and
/// +0xc, 0.0 at +0x18, +0x1c and +0x20, zero words at +0x10, +0x14, +0x24 and
/// +0x28, 0.0 at +0x2c, the constant at `01016248` at +0x30, 0.0 at +0x34,
/// zero words at +0x38, +0x3c and +0x40, 0.0 at +0x44, +0x48 and +0x4c, and
/// 1.0 (`FLD1`) at +0x50. The owning class is not confirmed.
pub fn fn_004fcc20(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let zero = x87_store32(0.0);
    let copy = |e: &Engine, constant: u32| x87_quiet_bits(e.global::<u32>(constant));
    e.mem.set_u32(base, 0x10000);
    e.mem.set_u32(base.wrapping_add(4), zero);
    let value = copy(e, CONSTANT_AT_08);
    e.mem.set_u32(base.wrapping_add(8), value);
    let value = copy(e, CONSTANT_AT_0C);
    e.mem.set_u32(base.wrapping_add(0xc), value);
    for offset in [0x18u32, 0x1c, 0x20] {
        e.mem.set_u32(base.wrapping_add(offset), zero);
    }
    for offset in [0x10u32, 0x14, 0x24, 0x28] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.mem.set_u32(base.wrapping_add(0x2c), zero);
    let value = copy(e, CONSTANT_AT_30);
    e.mem.set_u32(base.wrapping_add(0x30), value);
    e.mem.set_u32(base.wrapping_add(0x34), zero);
    for offset in [0x38u32, 0x3c, 0x40] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    for offset in [0x44u32, 0x48, 0x4c] {
        e.mem.set_u32(base.wrapping_add(offset), zero);
    }
    e.mem.set_u32(base.wrapping_add(0x50), x87_store32(1.0));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f_cc20, fn_004fcc20(Ptr))]
}

#[cfg(test)]
mod b025_tests {
    use super::*;

    #[test]
    fn fn_004fcc20_fills_the_block() {
        let mut e = Engine::new();
        e.map(0x0101_3000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.map(0x0102_2000, 0x1000);
        e.set_global::<f32>(CONSTANT_AT_08, 1000.0);
        e.set_global::<f32>(CONSTANT_AT_0C, 10000.0);
        // A signalling NaN is quieted by the FLD/FSTP copy.
        e.set_global::<u32>(CONSTANT_AT_30, 0x7f80_0001);
        let this = e.mem.alloc(0x58);
        e.mem.write(this, &[0xee; 0x58]);
        e.call(0x004f_cc20, &args![this]);
        let words: Vec<u32> = (0..0x15).map(|i| e.mem.u32(this + 4 * i)).collect();
        let mut expected = vec![0u32; 0x15];
        expected[0] = 0x10000;
        expected[2] = 1000.0f32.to_bits();
        expected[3] = 10000.0f32.to_bits();
        expected[0xc] = 0x7fc0_0001;
        expected[0x14] = 1.0f32.to_bits();
        assert_eq!(words, expected);
        assert_eq!(e.mem.u32(this + 0x54), 0xeeee_eeee);
    }
}
