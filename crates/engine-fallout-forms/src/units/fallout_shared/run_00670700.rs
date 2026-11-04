//! `fallout shared/run_00670700` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00670700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a percentage is above a limit (`cdecl`, two floats; the engine
/// map gives it no name): `value / total` times the `double` at `01017a40`,
/// truncated (`_ftol2`), or 100 when `total` is at most the `double` at
/// `01012060` (or unordered), compared (signed) with the word `0043d4d0`
/// gives for `011d6458`.
pub fn fn_00670700(e: &mut Engine, value: f32, total: f32) -> bool {
    let loaded = x87_load32(total.to_bits());
    let zero = x87_load64(e.mem.u64(0x0101_2060));
    let percent = if loaded <= zero || loaded.is_nan() {
        100
    } else {
        let ratio = x87_div(x87_load32(value.to_bits()), x87_m32(total.to_bits()));
        let scaled = x87_mul(ratio, x87_load64(e.mem.u64(0x0101_7a40)));
        e.call(0x00ec_62c0, &args![scaled]).i32()
    };
    let limit = e.call(0x0043_d4d0, &args![0x011d_6458u32]).u32();
    percent > e.mem.i32(limit)
}

// Translated from 00670760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears three parts (`thiscall`, no stack words; the engine map gives it
/// no name): `00401080` on `this` and `00407a90` on +6 and +8, each with a
/// zero byte (`cdecl`).
pub fn fn_00670760(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0040_1080, &args![base, 0u32]);
    e.call(0x0040_7a90, &args![base + 6, 0u32]);
    e.call(0x0040_7a90, &args![base + 8, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_0700, fn_00670700(f32, f32) -> bool),
        entry!(0x0067_0760, fn_00670760(Ptr)),
    ]
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    #[test]
    fn percentage_above_the_limit() {
        let mut e = Engine::new();
        e.map(0x0101_2060, 8);
        e.set_global::<f64>(0x0101_2060, 0.0);
        e.map(0x0101_7a40, 8);
        e.set_global::<f64>(0x0101_7a40, 100.0);
        e.register(0x00ec_62c0, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as i32 as u32).into_ret()
        });
        let limit = e.mem.alloc(4);
        e.mem.set_u32(limit, 50);
        e.register_double(0x0043_d4d0, move |_, _| limit.into_ret());
        assert!(e.call(0x0067_0700, &args![3.0f32, 4.0f32]).bool());
        assert!(!e.call(0x0067_0700, &args![2.0f32, 4.0f32]).bool());
        // A zero or NaN total counts as 100.
        assert!(e.call(0x0067_0700, &args![0.0f32, 0.0f32]).bool());
        assert!(e.call(0x0067_0700, &args![0.0f32, f32::NAN]).bool());
        e.mem.set_u32(limit, 100);
        assert!(!e.call(0x0067_0700, &args![0.0f32, -1.0f32]).bool());
    }

    #[test]
    fn clears_three_parts() {
        let mut e = Engine::new();
        e.register(0x0040_1080, |_, _| Ret::default());
        e.register(0x0040_7a90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_0760, &args![0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0040_1080, vec![0x5000, 0]),
                (0x0040_7a90, vec![0x5006, 0]),
                (0x0040_7a90, vec![0x5008, 0])
            ]
        );
    }
}
