//! `SpeedTree/run_00b3c5a0` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The CRT `double` function the sum goes through (`00ec6054`, cdecl, a
/// `double`, the result in ST0; the square root by its use, not
/// confirmed).
const DOUBLE_FUNCTION_00EC6054: u32 = 0x00ec_6054;
/// The vector's component count (+0x14; the `float`s start at +0).
const COMPONENT_COUNT: u32 = 0x14;

// Translated from 00b3c5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The length of a vector of up to five `float`s (unnamed in the engine
/// map): the sum of the squares of the first `count` (+0x14, a signed
/// count) components, each step `FMUL`, `FADD` and stored as a `float`,
/// then `00ec6054(sum)` rounded to a `float`.
pub fn fn_00b3c5a0(e: &mut Engine, this: Ptr) -> f32 {
    let base = this.addr();
    let mut sum = x87_store32(0.0);
    let mut i: i32 = 0;
    while i < e.mem.i32(base + COMPONENT_COUNT) {
        let component = e.mem.u32(base.wrapping_add((i as u32).wrapping_mul(4)));
        let square = x87_mul(x87_load32(component), x87_m32(component));
        sum = x87_store32(x87_add(square, x87_m32(sum)));
        i += 1;
    }
    let result = e
        .call(DOUBLE_FUNCTION_00EC6054, &args![x87_load32(sum)])
        .f64();
    f32::from_bits(x87_store32(result))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b3_c5a0, fn_00b3c5a0(Ptr) -> f32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_the_squares_then_calls_the_double_function() {
        let mut e = Engine::new();
        // Returns its argument plus one.
        e.register(DOUBLE_FUNCTION_00EC6054, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)) + 1.0,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x18);
        for (i, value) in [3.0f32, 4.0, 12.0].iter().enumerate() {
            e.mem.set_f32(this + 4 * i as u32, *value);
        }
        e.mem.set_i32(this + COMPONENT_COUNT, 2);
        assert_eq!(e.call(0x00b3_c5a0, &args![this]).f64(), 26.0);
        e.mem.set_i32(this + COMPONENT_COUNT, 3);
        assert_eq!(e.call(0x00b3_c5a0, &args![this]).f64(), 170.0);
        // A negative count sums nothing.
        e.mem.set_i32(this + COMPONENT_COUNT, -1);
        assert_eq!(e.call(0x00b3_c5a0, &args![this]).f64(), 1.0);
    }
}
