//! `fallout shared/run_00507d80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The engine map gives none of these functions a name (`range-sub`), so the
//! classes they work on are unconfirmed: the float getters read fields of
//! `ECX`, and `005082b0` blends a value toward an upper and a lower bound over
//! three time spans, which `00507da0`, `00508090` and `00508140` feed with
//! different getters.

#[allow(unused_imports)]
use crate::prelude::*;

/// The `double` 0.0 at `01012060` the comparisons test against.
const ZERO_01012060: u32 = 0x0101_2060;
/// The writable `float` at `011c84f8` that `005081d0` and `00508240` scale their argument by.
const SCALE_011C84F8: u32 = 0x011c_84f8;
/// The `double` at `01026998` (0.33f widened) `005082b0` compares its factor with.
const FACTOR_FLOOR_01026998: u32 = 0x0102_6998;
/// The `float` at `01026994` (0.33) `005082b0` raises its factor to.
const FACTOR_RAISED_01026994: u32 = 0x0102_6994;

/// The high byte of the x87 status word (`FNSTSW AX`, then `AH`) after
/// `FCOM st0, operand`: C0 (`0x01`) when `st0 < operand`, C3 (`0x40`) when
/// equal, nothing when greater, C0 | C2 | C3 (`0x45`) when unordered.
fn fcom(st0: f64, operand: f64) -> u8 {
    if st0.is_nan() || operand.is_nan() {
        0x45
    } else if st0 < operand {
        0x01
    } else if st0 == operand {
        0x40
    } else {
        0x00
    }
}

/// The parity flag after `TEST AH, mask` (set when the masked byte has an
/// even number of 1 bits), which `JP` / `JNP` test.
fn parity_even(status: u8, mask: u8) -> bool {
    (status & mask).count_ones() % 2 == 0
}

/// The low word `FISTP qword` leaves with the rounding mode set to truncate
/// (`OR 0xc00`): out of range and NaN give the integer indefinite
/// `0x8000_0000_0000_0000`, whose low word is 0.
fn fistp_truncate_low(value: f64) -> u32 {
    let converted = if value.is_finite()
        && (-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0).contains(&value)
    {
        value.trunc() as i64
    } else {
        i64::MIN
    };
    converted as u32
}

// Translated from 00507d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns what `00559450` returns for the member at `this + 0xdc` (that
/// function reads the word at its `this`).
pub fn fn_00507d80(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0055_9450, &args![this.addr() + 0xdc]).u32()
}

// Translated from 00507da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the blend `005082b0` on `value`, `step`, `time` and `active` with
/// the five spans and bounds read through `0051b8e0`, `006ca4e0`, `00507e10`,
/// `00644930` and `008d01e0` (called in reverse order, as the arguments are
/// pushed).
pub fn fn_00507da0(
    e: &mut Engine,
    this: Ptr,
    value: f32,
    step: f32,
    time: f32,
    active: bool,
) -> f32 {
    let lower = e.call(0x008d_01e0, &args![this]).f32();
    let upper = e.call(0x0064_4930, &args![this]).f32();
    let second_span = fn_00507e10(e, this);
    let decay_span = e.call(0x006c_a4e0, &args![this]).f32();
    let first_span = e.call(0x0051_b8e0, &args![this]).f32();
    fn_005082b0(
        e,
        this,
        x87_quiet(value),
        x87_quiet(step),
        x87_quiet(time),
        active,
        first_span,
        decay_span,
        second_span,
        upper,
        lower,
    )
}

// Translated from 00507e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `this + 0x8c`.
pub fn fn_00507e10(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x8c))
}

// Translated from 00507e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the state block `state` (nothing when it is null):
/// - `+0x38` = the blend `00508090` of its old value; the 16 bytes at `+0x2c`
///   are copied to `+0xc`; `+0x18` grows by the factor `00508240(span)` and
///   is clamped through `00404010(0, x)` and `0040ebd0(1, x)`;
/// - the same for `+0x48` (`00508140`), `+0x3c` → `+0x1c` and `+0x28`
///   (`005081d0`);
/// - `+0x4c` and `+0x50` grow by `step` times the floats `00508050` and
///   `00639aa0` return;
/// - `+0x70` is the truncated integer of `004b3ab0` applied to the floats
///   `00507b20`, `004a0d90`, `00508070`, `004e3d00` and a value clamped from
///   `span` through `00404010` and `0040ebd0`.
pub fn fn_00507e30(
    e: &mut Engine,
    this: Ptr,
    state: Ptr,
    step: f32,
    time: f32,
    span: f32,
    active: bool,
) {
    if state.is_null() {
        return;
    }
    let s = state.addr();
    let step = x87_quiet(step);
    let time = x87_quiet(time);
    let span = x87_quiet(span);

    let old = x87_quiet(e.mem.f32(s + 0x38));
    let blended = fn_00508090(e, this, old, step, time, active);
    e.mem.set_f32(s + 0x38, blended);
    for i in 0..4 {
        let w = e.mem.u32(s + 0x2c + 4 * i);
        e.mem.set_u32(s + 0xc + 4 * i, w);
    }
    let factor = fn_00508240(e, this, span);
    let field = e.mem.f32(s + 0x18);
    e.mem.set_f32(
        s + 0x18,
        (f64::from(factor) * f64::from(field) + f64::from(field)) as f32,
    );
    clamp_unit(e, s + 0x18);

    let old = x87_quiet(e.mem.f32(s + 0x48));
    let blended = fn_00508140(e, this, old, step, time, active);
    e.mem.set_f32(s + 0x48, blended);
    for i in 0..4 {
        let w = e.mem.u32(s + 0x3c + 4 * i);
        e.mem.set_u32(s + 0x1c + 4 * i, w);
    }
    let factor = fn_005081d0(e, this, span);
    let field = e.mem.f32(s + 0x28);
    e.mem.set_f32(
        s + 0x28,
        (f64::from(factor) * f64::from(field) + f64::from(field)) as f32,
    );
    clamp_unit(e, s + 0x28);

    let rate = fn_00508050(e, this);
    let field = e.mem.f32(s + 0x4c);
    e.mem.set_f32(
        s + 0x4c,
        (f64::from(rate) * f64::from(step) + f64::from(field)) as f32,
    );
    let rate = e.call(0x0063_9aa0, &args![this]).f64();
    let field = e.mem.f32(s + 0x50);
    e.mem
        .set_f32(s + 0x50, (rate * f64::from(step) + f64::from(field)) as f32);

    let limit = fn_00508070(e, this);
    let low = e
        .call(0x0040_4010, &args![limit.to_bits(), span.to_bits()])
        .f32();
    let cap = e.call(0x004e_3d00, &args![this]).f32();
    let clamped = x87_quiet(
        e.call(0x0040_ebd0, &args![cap.to_bits(), low.to_bits()])
            .f32(),
    );
    let fourth = e.call(0x004e_3d00, &args![this]).f32();
    let third = fn_00508070(e, this);
    let second = e.call(0x004a_0d90, &args![this]).f32();
    let first = e.call(0x0050_7b20, &args![this]).f32();
    let mapped = e
        .call(
            0x004b_3ab0,
            &args![
                first.to_bits(),
                second.to_bits(),
                third.to_bits(),
                fourth.to_bits(),
                clamped.to_bits()
            ],
        )
        .f64();
    e.mem.set_u32(s + 0x70, fistp_truncate_low(mapped));
}

/// `00507e30`'s clamp of the float at `field`: `0040ebd0(1.0, 00404010(0.0, x))`.
fn clamp_unit(e: &mut Engine, field: u32) {
    let value = x87_quiet(e.mem.f32(field));
    let low = e
        .call(0x0040_4010, &args![0.0f32.to_bits(), value.to_bits()])
        .f32();
    let clamped = e
        .call(0x0040_ebd0, &args![1.0f32.to_bits(), low.to_bits()])
        .f32();
    e.mem.set_f32(field, clamped);
}

// Translated from 00508050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `this + 0x44`.
pub fn fn_00508050(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x44))
}

// Translated from 00508070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `this + 0x110`.
pub fn fn_00508070(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x110))
}

// Translated from 00508090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the blend `005082b0` on `value`, `step`, `time` and `active` with
/// the spans and bounds read through `00759450`, `00508120`, `00508100`,
/// `009a7c40` and `009a9350` (called in reverse order).
pub fn fn_00508090(
    e: &mut Engine,
    this: Ptr,
    value: f32,
    step: f32,
    time: f32,
    active: bool,
) -> f32 {
    let lower = e.call(0x009a_9350, &args![this]).f32();
    let upper = e.call(0x009a_7c40, &args![this]).f32();
    let second_span = fn_00508100(e, this);
    let decay_span = fn_00508120(e, this);
    let first_span = e.call(0x0075_9450, &args![this]).f32();
    fn_005082b0(
        e,
        this,
        x87_quiet(value),
        x87_quiet(step),
        x87_quiet(time),
        active,
        first_span,
        decay_span,
        second_span,
        upper,
        lower,
    )
}

// Translated from 00508100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `this + 0x30`.
pub fn fn_00508100(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x30))
}

// Translated from 00508120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `this + 0x34`.
pub fn fn_00508120(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x34))
}

// Translated from 00508140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the blend `005082b0` on `value`, `step`, `time` and `active` with
/// the spans and bounds read through `009d1b50`, `00639b20`, `005081b0`,
/// `0051b860` and `009cdac0` (called in reverse order).
pub fn fn_00508140(
    e: &mut Engine,
    this: Ptr,
    value: f32,
    step: f32,
    time: f32,
    active: bool,
) -> f32 {
    let lower = e.call(0x009c_dac0, &args![this]).f32();
    let upper = e.call(0x0051_b860, &args![this]).f32();
    let second_span = fn_005081b0(e, this);
    let decay_span = e.call(0x0063_9b20, &args![this]).f32();
    let first_span = e.call(0x009d_1b50, &args![this]).f32();
    fn_005082b0(
        e,
        this,
        x87_quiet(value),
        x87_quiet(step),
        x87_quiet(time),
        active,
        first_span,
        decay_span,
        second_span,
        upper,
        lower,
    )
}

// Translated from 005081b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `this + 0x58`.
pub fn fn_005081b0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x58))
}

/// The shared body of `005081d0` and `00508240`: 1.0 when `gate` is not
/// above 0.0 (NaN included); otherwise
/// `004e44b0((rate * (span * scale)) as f32) * gate`, rounded to `float`.
fn scaled_factor(e: &mut Engine, this: Ptr, span: f32, gate_fn: u32, rate_fn: u32) -> f32 {
    let gate = e.call(gate_fn, &args![this]).f64();
    let zero = e.global::<f64>(ZERO_01012060);
    if fcom(gate, zero) & 0x41 != 0 {
        return 1.0;
    }
    let gate = e.call(gate_fn, &args![this]).f64();
    let scaled = f64::from(span) * f64::from(e.global::<f32>(SCALE_011C84F8));
    let rate = e.call(rate_fn, &args![this]).f64();
    let argument = (rate * scaled) as f32;
    let result = e.call(0x004e_44b0, &args![argument.to_bits()]).f64();
    x87_quiet((result * gate) as f32)
}

// Translated from 005081d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// 1.0 when the float `009cdae0` returns is not above 0.0; otherwise
/// `004e44b0(0069ef80() * span * [011c84f8]) * 009cdae0()`.
pub fn fn_005081d0(e: &mut Engine, this: Ptr, span: f32) -> f32 {
    scaled_factor(e, this, x87_quiet(span), 0x009c_dae0, 0x0069_ef80)
}

// Translated from 00508240 (decompiled, FalloutNV.exe 1.4.0.525)
/// 1.0 when the float `00598040` returns is not above 0.0; otherwise
/// `004e44b0(00792760() * span * [011c84f8]) * 00598040()`.
pub fn fn_00508240(e: &mut Engine, this: Ptr, span: f32) -> f32 {
    scaled_factor(e, this, x87_quiet(span), 0x0059_8040, 0x0079_2760)
}

// Translated from 005082b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `value` between `upper` and `lower` by `time` against three spans
/// (`ECX` is not read).
///
/// With `active` set: while `time < first_span + second_span` it only
/// clears `active`; otherwise it lowers `value` by
/// `00ba52d0() * step * factor` (floored at 0 through `00404010`), where
/// `factor` is `00408840(upper - lower) / decay_span` when `decay_span > 0`
/// and `upper != lower`, else 0, and at least the float at `01026994` when
/// below the double at `01026998`. If `active` is still set the result is
/// `value`.
///
/// Otherwise: `time < first_span` raises `value` by
/// `step / first_span * upper` (through `0040ebd0(upper, x)`); then
/// `time - first_span < second_span` gives `upper`; then
/// `time - first_span - second_span < decay_span` lowers `value` by
/// `00408840(upper - lower) * step / decay_span` (through
/// `00404010(lower, x)`); past that, `lower`.
#[allow(clippy::too_many_arguments)]
pub fn fn_005082b0(
    e: &mut Engine,
    _this: Ptr,
    value: f32,
    step: f32,
    time: f32,
    active: bool,
    first_span: f32,
    decay_span: f32,
    second_span: f32,
    upper: f32,
    lower: f32,
) -> f32 {
    let zero = e.global::<f64>(ZERO_01012060);
    let mut value = x87_quiet(value);
    let mut active = active;
    if active {
        let held = f64::from(first_span) + f64::from(second_span);
        if !parity_even(fcom(f64::from(time), held), 0x05) {
            active = false;
        } else {
            let mut factor = 0.0f32;
            if fcom(f64::from(decay_span), zero) & 0x41 == 0
                && parity_even(fcom(f64::from(upper) - f64::from(lower), zero), 0x44)
            {
                let inverse = 1.0 / f64::from(decay_span);
                let range = (f64::from(upper) - f64::from(lower)) as f32;
                let curve = e.call(0x0040_8840, &args![range.to_bits()]).f64();
                factor = (curve * inverse) as f32;
            }
            if !parity_even(
                fcom(f64::from(factor), e.global::<f64>(FACTOR_FLOOR_01026998)),
                0x05,
            ) {
                factor = x87_quiet(e.global::<f32>(FACTOR_RAISED_01026994));
            }
            let ticks = e.call(0x00ba_52d0, &args![]).u32();
            let drop = (f64::from(ticks) * f64::from(step) * f64::from(factor)) as f32;
            let lowered = (f64::from(value) - f64::from(drop)) as f32;
            value = e
                .call(0x0040_4010, &args![0.0f32.to_bits(), lowered.to_bits()])
                .f32();
        }
    }
    if active {
        return x87_quiet(value);
    }
    if fcom(f64::from(first_span), f64::from(time)) & 0x41 == 0 {
        let raised =
            (f64::from(step) / f64::from(first_span) * f64::from(upper) + f64::from(value)) as f32;
        value = e
            .call(0x0040_ebd0, &args![upper.to_bits(), raised.to_bits()])
            .f32();
    } else if fcom(
        f64::from(second_span),
        f64::from(time) - f64::from(first_span),
    ) & 0x41
        == 0
    {
        value = x87_quiet(upper);
    } else if fcom(
        f64::from(decay_span),
        f64::from(time) - f64::from(first_span) - f64::from(second_span),
    ) & 0x41
        == 0
    {
        let rate = f64::from(step) / f64::from(decay_span);
        let range = (f64::from(upper) - f64::from(lower)) as f32;
        let curve = e.call(0x0040_8840, &args![range.to_bits()]).f64();
        let lowered = (f64::from(value) - curve * rate) as f32;
        value = e
            .call(0x0040_4010, &args![lower.to_bits(), lowered.to_bits()])
            .f32();
    } else {
        value = x87_quiet(lower);
    }
    x87_quiet(value)
}

// Translated from 00508450 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00508450 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x26 (0x9c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00508450(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_b560, &args![this]);
    e.call(0x0051_04f0, &args![base + 0xc]);
    e.mem.set_u32(base, 0x0102_6bd0);
    e.mem.set_u32(base + 0xc, 0x0102_6a4c);
    e.mem.set_u32(base + 0x3c, 0x0102_6a34);
    e.mem.set_u32(base + 0x48, 0x0102_6a10);
    e.mem.set_u32(base + 0x68, 0x0102_69fc);
    e.mem.set_u32(base + 0x74, 0x0102_69e8);
    e.mem.set_u32(base + 0x7c, 0x0102_69d8);
    e.call(0x004f_15a0, &args![base + 0xc, 0x26]);
    e.call(0x0048_4ab0, &args![base + 0xc, 0x1]);
    this
}

// Translated from 00508510 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00508510 - the exception-unwinding frame (FS:[0]) and its security cookie are not translated.
/// Destructor of the form type 0x26 object `00508450` builds: stores the
/// same seven vtable pointers, then runs `00483710` and `00510640` on the
/// component at `+0xc`.
pub fn fn_00508510(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_6bd0);
    e.mem.set_u32(base + 0xc, 0x0102_6a4c);
    e.mem.set_u32(base + 0x3c, 0x0102_6a34);
    e.mem.set_u32(base + 0x48, 0x0102_6a10);
    e.mem.set_u32(base + 0x68, 0x0102_69fc);
    e.mem.set_u32(base + 0x74, 0x0102_69e8);
    e.mem.set_u32(base + 0x7c, 0x0102_69d8);
    e.call(0x0048_3710, &args![base + 0xc]);
    e.call(0x0051_0640, &args![base + 0xc]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_7d80, fn_00507d80(Ptr) -> u32),
        entry!(0x0050_7da0, fn_00507da0(Ptr, f32, f32, f32, bool) -> f32),
        entry!(0x0050_7e10, fn_00507e10(Ptr) -> f32),
        entry!(0x0050_7e30, fn_00507e30(Ptr, Ptr, f32, f32, f32, bool)),
        entry!(0x0050_8050, fn_00508050(Ptr) -> f32),
        entry!(0x0050_8070, fn_00508070(Ptr) -> f32),
        entry!(0x0050_8090, fn_00508090(Ptr, f32, f32, f32, bool) -> f32),
        entry!(0x0050_8100, fn_00508100(Ptr) -> f32),
        entry!(0x0050_8120, fn_00508120(Ptr) -> f32),
        entry!(0x0050_8140, fn_00508140(Ptr, f32, f32, f32, bool) -> f32),
        entry!(0x0050_81b0, fn_005081b0(Ptr) -> f32),
        entry!(0x0050_81d0, fn_005081d0(Ptr, f32) -> f32),
        entry!(0x0050_8240, fn_00508240(Ptr, f32) -> f32),
        entry!(
            0x0050_82b0,
            fn_005082b0(Ptr, f32, f32, f32, bool, f32, f32, f32, f32, f32) -> f32
        ),
        entry!(0x0050_8450, fn_00508450(Ptr) -> Ptr),
        entry!(0x0050_8510, fn_00508510(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret_f32(v: f32) -> Ret {
        v.into_ret()
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine_with_constants() -> Engine {
        let mut e = Engine::new();
        e.map(ZERO_01012060, 8);
        e.map(SCALE_011C84F8, 4);
        e.map(FACTOR_RAISED_01026994, 0xc);
        e.set_global(ZERO_01012060, 0.0f64);
        e.set_global(FACTOR_FLOOR_01026998, f64::from(0.33f32));
        e.set_global(FACTOR_RAISED_01026994, 0.33f32);
        e
    }

    fn object(e: &mut Engine, size: u32) -> Ptr {
        Ptr::new(e.mem.alloc(size))
    }

    #[test]
    fn fn_00507d80_forwards_the_member_at_0xdc() {
        let mut e = Engine::new();
        e.register(0x0055_9450, |_, _| 0x1234u32.into_ret());
        e.call_log = Some(vec![]);
        let this = object(&mut e, 0x100);
        assert_eq!(e.call(0x0050_7d80, &args![this]).u32(), 0x1234);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0055_9450), vec![vec![this.addr() + 0xdc]]);
    }

    #[test]
    fn float_getters_read_their_fields() {
        let mut e = Engine::new();
        let this = object(&mut e, 0x120);
        for (addr, off, v) in [
            (0x0050_7e10u32, 0x8cu32, 1.5f32),
            (0x0050_8050, 0x44, 2.5),
            (0x0050_8070, 0x110, 3.5),
            (0x0050_8100, 0x30, 4.5),
            (0x0050_8120, 0x34, 5.5),
            (0x0050_81b0, 0x58, 6.5),
        ] {
            e.mem.set_f32(this.addr() + off, v);
            assert_eq!(e.call(addr, &args![this]).f32(), v, "{addr:08x}");
        }
        // FLD quiets a signalling NaN.
        e.mem.set_u32(this.addr() + 0x8c, 0x7f80_0001);
        assert_eq!(fn_00507e10(&mut e, this).to_bits(), 0x7fc0_0001);
    }

    /// Doubles for the five span/bound getters of one of the three wrappers;
    /// `005082b0` runs for real with `active` clear and `time` past every
    /// span, so the result is `lower`.
    fn wrapper_case(addr: u32, getters: [u32; 3], own: [(u32, f32); 2]) {
        let mut e = engine_with_constants();
        let this = object(&mut e, 0x120);
        // lower, upper, first_span through doubles; the two own getters read fields.
        e.register(getters[0], |_, _| ret_f32(0.25));
        e.register(getters[1], |_, _| ret_f32(0.75));
        e.register(getters[2], |_, _| ret_f32(1.0));
        for (off, v) in own {
            e.mem.set_f32(this.addr() + off, v);
        }
        e.call_log = Some(vec![]);
        let r = e
            .call(addr, &args![this, 0.5f32, 0.1f32, 10.0f32, false])
            .f32();
        assert_eq!(r, 0.25);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).skip(1).collect();
        assert_eq!(order, vec![getters[0], getters[1], getters[2]]);
    }

    #[test]
    fn fn_00507da0_blends_with_its_getters() {
        let mut e = engine_with_constants();
        let this = object(&mut e, 0x120);
        e.register(0x008d_01e0, |_, _| ret_f32(0.25));
        e.register(0x0064_4930, |_, _| ret_f32(0.75));
        e.register(0x006c_a4e0, |_, _| ret_f32(2.0));
        e.register(0x0051_b8e0, |_, _| ret_f32(1.0));
        e.mem.set_f32(this.addr() + 0x8c, 3.0);
        e.call_log = Some(vec![]);
        // time 2.0: past first_span (1.0), within second_span (3.0): upper.
        let r = e
            .call(0x0050_7da0, &args![this, 0.5f32, 0.1f32, 2.0f32, false])
            .f32();
        assert_eq!(r, 0.75);
        let order: Vec<u32> = e.call_log.take().unwrap().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                0x0050_7da0,
                0x008d_01e0,
                0x0064_4930,
                0x006c_a4e0,
                0x0051_b8e0
            ]
        );
    }

    #[test]
    fn fn_00508090_blends_with_its_getters() {
        wrapper_case(
            0x0050_8090,
            [0x009a_9350, 0x009a_7c40, 0x0075_9450],
            [(0x30, 1.0), (0x34, 1.0)],
        );
    }

    #[test]
    fn fn_00508140_blends_with_its_getters() {
        let mut e = engine_with_constants();
        let this = object(&mut e, 0x120);
        e.register(0x009c_dac0, |_, _| ret_f32(0.25));
        e.register(0x0051_b860, |_, _| ret_f32(0.75));
        e.register(0x0063_9b20, |_, _| ret_f32(1.0));
        e.register(0x009d_1b50, |_, _| ret_f32(1.0));
        e.mem.set_f32(this.addr() + 0x58, 1.0);
        let r = e
            .call(0x0050_8140, &args![this, 0.5f32, 0.1f32, 10.0f32, false])
            .f32();
        assert_eq!(r, 0.25);
    }

    #[test]
    fn fn_005081d0_and_00508240_scale_or_return_one() {
        for (addr, gate_fn, rate_fn) in [
            (0x0050_81d0u32, 0x009c_dae0u32, 0x0069_ef80u32),
            (0x0050_8240, 0x0059_8040, 0x0079_2760),
        ] {
            let mut e = engine_with_constants();
            let this = object(&mut e, 0x100);
            e.set_global(SCALE_011C84F8, 2.0f32);
            e.register(gate_fn, |_, _| ret_f32(0.0));
            assert_eq!(e.call(addr, &args![this, 3.0f32]).f32(), 1.0);
            e.register(gate_fn, |_, _| ret_f32(f32::NAN));
            assert_eq!(e.call(addr, &args![this, 3.0f32]).f32(), 1.0);

            e.register(gate_fn, |_, _| ret_f32(0.5));
            e.register(rate_fn, |_, _| ret_f32(4.0));
            e.register(0x004e_44b0, |_, a| (f32::from_bits(a[0]) + 1.0).into_ret());
            e.call_log = Some(vec![]);
            // 004e44b0(4 * 3 * 2 = 24) = 25; 25 * 0.5.
            assert_eq!(e.call(addr, &args![this, 3.0f32]).f32(), 12.5);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, 0x004e_44b0), vec![vec![24.0f32.to_bits()]]);
            assert_eq!(calls_to(&log, gate_fn).len(), 2);
        }
    }

    fn blend(e: &mut Engine, words: [f32; 8], active: bool) -> f32 {
        let this = Ptr::new(0);
        fn_005082b0(
            e, this, words[0], words[1], words[2], active, words[3], words[4], words[5], words[6],
            words[7],
        )
    }

    fn register_clamps(e: &mut Engine) {
        // 00404010: the larger of the two (its JP test); 0040ebd0: the smaller.
        e.register(0x0040_4010, |_, a| {
            f32::from_bits(a[0]).max(f32::from_bits(a[1])).into_ret()
        });
        e.register(0x0040_ebd0, |_, a| {
            f32::from_bits(a[0]).min(f32::from_bits(a[1])).into_ret()
        });
        e.register(0x0040_8840, |_, a| (f32::from_bits(a[0]) * 2.0).into_ret());
    }

    #[test]
    fn fn_005082b0_inactive_branches() {
        let mut e = engine_with_constants();
        register_clamps(&mut e);
        // Rising: 0.2 + 0.5 / 1.0 * 1.0 = 0.7, capped at upper 1.0.
        assert_eq!(
            blend(&mut e, [0.2, 0.5, 0.5, 1.0, 2.0, 1.0, 1.0, 0.0], false),
            0.7
        );
        // Holding: time - first = 0.5 < second 1.0.
        assert_eq!(
            blend(&mut e, [0.2, 0.5, 1.5, 1.0, 2.0, 1.0, 1.0, 0.0], false),
            1.0
        );
        // Decaying: time - first - second = 0.5 < decay 2.0:
        // 0.9 - 00408840(1.0) * 0.5 / 2.0 = 0.9 - 0.5.
        assert_eq!(
            blend(&mut e, [0.9, 0.5, 2.5, 1.0, 2.0, 1.0, 1.0, 0.0], false),
            0.9f32 - 0.5
        );
        // Past every span: lower.
        assert_eq!(
            blend(&mut e, [0.9, 0.5, 9.0, 1.0, 2.0, 1.0, 1.0, 0.25], false),
            0.25
        );
        // NaN time: no span test passes, lower.
        assert_eq!(
            blend(
                &mut e,
                [0.9, 0.5, f32::NAN, 1.0, 2.0, 1.0, 1.0, 0.25],
                false
            ),
            0.25
        );
    }

    #[test]
    fn fn_005082b0_active_branches() {
        let mut e = engine_with_constants();
        register_clamps(&mut e);
        e.register(0x00ba_52d0, |_, _| 2u32.into_ret());
        // time 0.5 < first + second (2.0): active is cleared, then the rise runs:
        // 0.2 + 0.5 / 1.0 * 1.0 = 0.7.
        assert_eq!(
            blend(&mut e, [0.2, 0.5, 0.5, 1.0, 2.0, 1.0, 1.0, 0.0], true),
            0.7
        );
        // time 3.0: factor = 00408840(1.0) / 2.0 = 1.0; drop = 2 * 0.1 * 1.0.
        let r = blend(&mut e, [0.9, 0.1, 3.0, 1.0, 2.0, 1.0, 1.0, 0.0], true);
        let drop = (2.0 * f64::from(0.1f32)) as f32;
        assert_eq!(r, (f64::from(0.9f32) - f64::from(drop)) as f32);
        // upper == lower: factor 0 is raised to 0.33; drop = 2 * 0.1 * 0.33.
        let r = blend(&mut e, [0.9, 0.1, 3.0, 1.0, 2.0, 1.0, 0.5, 0.5], true);
        let drop = (2.0 * f64::from(0.1f32) * f64::from(0.33f32)) as f32;
        assert_eq!(r, (f64::from(0.9f32) - f64::from(drop)) as f32);
        // Floored at 0 through 00404010.
        let r = blend(&mut e, [0.1, 1.0, 3.0, 1.0, 2.0, 1.0, 1.0, 0.0], true);
        assert_eq!(r, 0.0);
    }

    #[test]
    fn fn_00507e30_skips_a_null_state() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = object(&mut e, 0x120);
        e.call(
            0x0050_7e30,
            &args![this, 0u32, 1.0f32, 2.0f32, 3.0f32, false],
        );
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00507e30_updates_the_state() {
        let mut e = engine_with_constants();
        register_clamps(&mut e);
        let this = object(&mut e, 0x120);
        let state = object(&mut e, 0x80);
        let s = state.addr();
        // 00508090 and 00508140 run for real: every getter of the blends
        // returns 0.125, so with the time past all spans each gives `lower`.
        for addr in [
            0x009a_9350u32,
            0x009a_7c40,
            0x0075_9450,
            0x009c_dac0,
            0x0051_b860,
            0x0063_9b20,
            0x009d_1b50,
        ] {
            e.register(addr, |_, _| ret_f32(0.125));
        }
        e.mem.set_f32(this.addr() + 0x30, 0.125);
        e.mem.set_f32(this.addr() + 0x34, 0.125);
        e.mem.set_f32(this.addr() + 0x58, 0.125);
        // 00508140's lower bound: +0x48 becomes 0.75.
        e.register(0x009c_dac0, |_, _| ret_f32(0.75));
        e.mem.set_f32(this.addr() + 0x44, 2.0);
        e.mem.set_f32(this.addr() + 0x110, 0.5);
        // 00508240 / 005081d0 gates are 0: factor 1.0.
        e.register(0x0059_8040, |_, _| ret_f32(0.0));
        e.register(0x009c_dae0, |_, _| ret_f32(0.0));
        e.register(0x0063_9aa0, |_, _| ret_f32(3.0));
        e.register(0x004e_3d00, |_, _| ret_f32(4.0));
        e.register(0x004a_0d90, |_, _| ret_f32(5.0));
        e.register(0x0050_7b20, |_, _| ret_f32(6.0));
        e.register(0x004b_3ab0, |_, _| (-7.75f64).into_ret());
        for i in 0..4 {
            e.mem.set_u32(s + 0x2c + 4 * i, 0x100 + i);
            e.mem.set_u32(s + 0x3c + 4 * i, 0x200 + i);
        }
        e.mem.set_f32(s + 0x4c, 1.0);
        e.mem.set_f32(s + 0x50, 1.0);
        e.call_log = Some(vec![]);
        e.call(
            0x0050_7e30,
            &args![this, state, 0.5f32, 10.0f32, 2.0f32, false],
        );
        assert_eq!(e.mem.f32(s + 0x38), 0.125);
        assert_eq!(e.mem.f32(s + 0x48), 0.75);
        // The 16 bytes at +0x2c (ending with the new +0x38) and +0x3c (ending
        // with the new +0x48) are copied to +0xc and +0x1c.
        for i in 0..3 {
            assert_eq!(e.mem.u32(s + 0xc + 4 * i), 0x100 + i);
            assert_eq!(e.mem.u32(s + 0x1c + 4 * i), 0x200 + i);
        }
        // 0.125 * 1 + 0.125 = 0.25; 0.75 * 1 + 0.75 = 1.5, clamped to 1.0.
        assert_eq!(e.mem.f32(s + 0x18), 0.25);
        assert_eq!(e.mem.f32(s + 0x28), 1.0);
        // 2.0 * 0.5 + 1.0; 3.0 * 0.5 + 1.0.
        assert_eq!(e.mem.f32(s + 0x4c), 2.0);
        assert_eq!(e.mem.f32(s + 0x50), 2.5);
        // Truncated -7.75 → -7.
        assert_eq!(e.mem.u32(s + 0x70), (-7i32) as u32);
        let log = e.call_log.take().unwrap();
        // max(0x110 = 0.5, span 2.0) = 2.0, min(004e3d00 = 4.0, 2.0) = 2.0.
        assert_eq!(
            calls_to(&log, 0x004b_3ab0),
            vec![vec![
                6.0f32.to_bits(),
                5.0f32.to_bits(),
                0.5f32.to_bits(),
                4.0f32.to_bits(),
                2.0f32.to_bits()
            ]]
        );
        assert_eq!(
            calls_to(&log, 0x0040_4010)[0],
            vec![0.0f32.to_bits(), 0.25f32.to_bits()]
        );
    }

    #[test]
    fn fistp_truncates_and_gives_indefinite_low_word() {
        assert_eq!(fistp_truncate_low(2.9), 2);
        assert_eq!(fistp_truncate_low(-2.9), (-2i32) as u32);
        assert_eq!(fistp_truncate_low(f64::NAN), 0);
        assert_eq!(fistp_truncate_low(1e30), 0);
        assert_eq!(fistp_truncate_low(4_294_967_297.0), 1);
    }

    #[test]
    fn fn_00508450_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_8450,
            size: 0x9c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_b560, &[A::T(0x0)]),
                (0x0051_04f0, &[A::T(0xc)]),
                (0x004f_15a0, &[A::T(0xc), A::V(0x26)]),
                (0x0048_4ab0, &[A::T(0xc), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_6bd0),
                (0xc, 4, 0x0102_6a4c),
                (0x3c, 4, 0x0102_6a34),
                (0x48, 4, 0x0102_6a10),
                (0x68, 4, 0x0102_69fc),
                (0x74, 4, 0x0102_69e8),
                (0x7c, 4, 0x0102_69d8),
            ],
        });
    }

    #[test]
    fn fn_00508510_tears_the_object_down() {
        let mut e = Engine::new();
        e.register(0x0048_3710, |_, _| Ret::default());
        e.register(0x0051_0640, |_, _| Ret::default());
        let this = object(&mut e, 0x9c);
        e.call_log = Some(vec![]);
        e.call(0x0050_8510, &args![this]);
        let b = this.addr();
        assert_eq!(e.mem.u32(b), 0x0102_6bd0);
        assert_eq!(e.mem.u32(b + 0x7c), 0x0102_69d8);
        assert_eq!(e.mem.u32(b + 0x48), 0x0102_6a10);
        let log = e.call_log.take().unwrap();
        let order: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![(0x0048_3710, vec![b + 0xc]), (0x0051_0640, vec![b + 0xc])]
        );
    }
}
