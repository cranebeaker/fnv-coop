//! `(unplaced)/run_006426a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The scope timer's constructor (`thiscall` on the timer, `(0x21, 1, source file, line)`)
/// and destructor (`thiscall`), and the source file name `...\Sky\Sun.cpp` (`0104f2ac`).
const TIMER_SCOPE_CONSTRUCT: u32 = 0x0040_4eb0;
const TIMER_SCOPE_DESTROY: u32 = 0x0040_4ee0;
const SUN_SOURCE_FILE_NAME: u32 = 0x0104_f2ac;
/// Callees: `006fa820` (`thiscall`, no arguments, answers an object), `0066b0d0`
/// (`thiscall` on `this + 0x28`, `(accumulator)`), the float getter at +0x98 `008d01e0`
/// (ST0), `BSShaderAccumulator::DoSunOcclusionQueury` (`thiscall` on the accumulator,
/// three words), and the `double` 25.0 at `0104f2f0`.
const OBJECT_006FA820: u32 = 0x006f_a820;
const SET_ACCUMULATOR_0066B0D0: u32 = 0x0066_b0d0;
const FLOAT_AT_98: u32 = 0x008d_01e0;
const DO_SUN_OCCLUSION_QUERY: u32 = 0x00b6_4bf0;
const SCALE_25: u32 = 0x0104_f2f0;

// Translated from 006427a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A float setter (unnamed in the map; `thiscall`, `(value)`): `value` stored at
/// `this + 0x98` (through the x87 stack, so a signalling NaN is stored quiet).
pub fn fn_006427a0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(0x98),
        x87_quiet_bits(value.to_bits()),
    );
}

// Translated from 006426a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(first, accumulator, third)`; `Sun.cpp` line 0x222 by its
/// scope timer): within the scope timer, when `006fa820` answers an object, the byte at
/// `this + 0x24` is set and `accumulator` is not null: the accumulator goes to `0066b0d0` on
/// `this + 0x28`; the object's float at +0x98 is divided by 25 (`0104f2f0`) around
/// `BSShaderAccumulator::DoSunOcclusionQueury(object, first, third)` on the accumulator and
/// multiplied back by 25 after it (each result stored as a float through `fn_006427a0`).
// Translation incomplete: 006426a0 - SEH exception unwinding is unsupported.
pub fn fn_006426a0(e: &mut Engine, this: Ptr, first: u32, accumulator: Ptr, third: u32) {
    e.with_stack(4, |e, timer| {
        e.call(
            TIMER_SCOPE_CONSTRUCT,
            &args![timer, 0x21u32, 1u32, SUN_SOURCE_FILE_NAME, 0x222u32],
        );
        let object = e.call(OBJECT_006FA820, &args![this]).u32();
        if object != 0 && e.mem.u8(this.addr() + 0x24) != 0 && !accumulator.is_null() {
            e.call(
                SET_ACCUMULATOR_0066B0D0,
                &args![this.addr() + 0x28, accumulator],
            );
            let object = e.call(OBJECT_006FA820, &args![this]).u32();
            let scale = e.global::<f64>(SCALE_25);
            let value = e.call(FLOAT_AT_98, &args![object]).f64();
            let lowered = x87_quiet_bits(x87_store32(x87_div(value, scale)));
            fn_006427a0(e, Ptr::new(object), f32::from_bits(lowered));
            e.call(
                DO_SUN_OCCLUSION_QUERY,
                &args![accumulator, object, first, third],
            );
            let value = e.call(FLOAT_AT_98, &args![object]).f64();
            let raised = x87_quiet_bits(x87_store32(x87_mul(value, scale)));
            fn_006427a0(e, Ptr::new(object), f32::from_bits(raised));
        }
        e.call(TIMER_SCOPE_DESTROY, &args![timer]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006427a0, fn_006427a0(Ptr, f32)),
        entry!(0x006426a0, fn_006426a0(Ptr, u32, Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_006427a0_stores_the_float_at_0x98() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x9c);
        e.call(0x006427a0, &args![this, 7.5f32]);
        assert_eq!(e.mem.f32(this + 0x98), 7.5);
    }

    #[test]
    fn fn_006426a0_scales_around_the_occlusion_query() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(SCALE_25, 8);
        e.set_global::<f64>(SCALE_25, 25.0);
        let object = e.mem.alloc(0x9c);
        e.mem.set_f32(object + 0x98, 50.0);
        e.register_double(OBJECT_006FA820, move |_, _| Ret {
            eax: object,
            ..Ret::default()
        });
        e.register(FLOAT_AT_98, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 0x98)),
            ..Ret::default()
        });
        e.register(DO_SUN_OCCLUSION_QUERY, |e, a| {
            assert_eq!(e.mem.f32(a[1] + 0x98), 2.0);
            Ret::default()
        });
        for addr in [
            TIMER_SCOPE_CONSTRUCT,
            TIMER_SCOPE_DESTROY,
            SET_ACCUMULATOR_0066B0D0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x30);
        // The byte at +0x24 clear: nothing but the timer.
        e.call(0x006426a0, &args![this, 1u32, 0x7000u32, 3u32]);
        assert!(calls_at(&e, DO_SUN_OCCLUSION_QUERY).is_empty());
        e.mem.set_u8(this + 0x24, 1);
        e.call(0x006426a0, &args![this, 1u32, 0u32, 3u32]);
        assert!(calls_at(&e, DO_SUN_OCCLUSION_QUERY).is_empty());
        e.call(0x006426a0, &args![this, 1u32, 0x7000u32, 3u32]);
        assert_eq!(
            calls_at(&e, DO_SUN_OCCLUSION_QUERY),
            vec![vec![0x7000, object, 1, 3]]
        );
        assert_eq!(
            calls_at(&e, SET_ACCUMULATOR_0066B0D0),
            vec![vec![this + 0x28, 0x7000]]
        );
        assert_eq!(e.mem.f32(object + 0x98), 50.0);
        let timers = calls_at(&e, TIMER_SCOPE_CONSTRUCT);
        assert_eq!(timers.len(), 3);
        assert_eq!(timers[0][1..], [0x21, 1, SUN_SOURCE_FILE_NAME, 0x222]);
        assert_eq!(calls_at(&e, TIMER_SCOPE_DESTROY).len(), 3);
    }
}
