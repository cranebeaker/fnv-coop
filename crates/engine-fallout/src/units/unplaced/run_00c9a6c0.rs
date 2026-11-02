//! `(unplaced)/run_00c9a6c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The float `-1.0` (`01012054`) stored in the first field.
const MINUS_ONE: u32 = 0x0101_2054;
/// The double `100.0` (`01017a40`) zero is divided by for the other two.
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;

// Translated from 00c9a6c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, no stack words): the element constructor `bhkBlendController::DoHit`
/// (`00c9b250`) hands the array constructor for three 12-byte elements (pushed at
/// `00c9b353`). Stores the float at `01012054` (-1.0) at +0, then `0.0 / [01017a40]` (the
/// double 100.0; `FLDZ`, `FDIV double`, rounded through a float local) at +4 and +8.
/// Returns nothing.
pub fn fn_00c9a6c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let first = x87_quiet_bits(e.mem.u32(MINUS_ONE));
    e.mem.set_u32(base, first);
    let divisor = f64::from_bits(e.mem.u64(HUNDRED_DOUBLE));
    let local = x87_store32(x87_div(0.0, divisor));
    let rest = x87_quiet_bits(local);
    e.mem.set_u32(base + 4, rest);
    e.mem.set_u32(base + 8, rest);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c9_a6c0, fn_00c9a6c0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rig(divisor: f64) -> Engine {
        let mut e = Engine::new();
        e.map(MINUS_ONE, 4);
        e.set_global(MINUS_ONE, -1.0f32);
        e.map(HUNDRED_DOUBLE, 8);
        e.set_global(HUNDRED_DOUBLE, divisor);
        e
    }

    #[test]
    fn the_element_is_minus_one_then_two_zeros() {
        let mut e = rig(100.0);
        let this = e.mem.alloc(12);
        for offset in [0, 4, 8] {
            e.mem.set_u32(this + offset, 0xdddd_dddd);
        }
        e.call(0x00c9_a6c0, &args![this]);
        assert_eq!(e.mem.f32(this), -1.0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
    }

    #[test]
    fn the_quotient_follows_the_divisor() {
        // A negative divisor gives -0.0; a zero one the default NaN of 0/0.
        let mut e = rig(-100.0);
        let this = e.mem.alloc(12);
        e.call(0x00c9_a6c0, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 8), 0x8000_0000);
        let mut e = rig(0.0);
        let this = e.mem.alloc(12);
        e.call(0x00c9_a6c0, &args![this]);
        assert!(e.mem.f32(this + 4).is_nan());
        assert_eq!(e.mem.u32(this + 4), e.mem.u32(this + 8));
    }
}
