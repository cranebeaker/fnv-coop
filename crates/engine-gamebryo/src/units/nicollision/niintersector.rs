//! `NiCollision/niintersector.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `NiIntersector` (Xbox PDB), 0x38 bytes: the base of the moving-volume intersectors. Its
    /// virtual slots: +0 the destructor, +4 `Test`, +8 `Find`, +0xc `Pseudodistance` (the two
    /// velocities and a time, the value in ST0), +0x10 `ComputeContactInformation` (the two
    /// velocities).
    pub struct NiIntersector: 0x38 {
        /// `m_fMaxTime` (Xbox PDB).
        0x04 m_fMaxTime: u32,
        /// `m_fDerivativeTimeStep` (Xbox PDB).
        0x08 m_fDerivativeTimeStep: u32,
        /// `m_fInvDerivativeTimeStep` (Xbox PDB).
        0x0C m_fInvDerivativeTimeStep: u32,
        /// `m_fPseudodistanceThreshold` (Xbox PDB).
        0x10 m_fPseudodistanceThreshold: u32,
        /// `m_iMaxIterations` (Xbox PDB).
        0x14 m_iMaxIterations: i32,
        /// `m_eIntersectionType` (Xbox PDB): 1 when the volumes do not meet in time, 2 when they
        /// touch at the contact time, 3 when they overlap at the start (by `Find`).
        0x18 m_eIntersectionType: u32,
        /// `m_fContactTime` (Xbox PDB).
        0x1C m_fContactTime: u32,
        /// `m_kContactPoint` (Xbox PDB), x (y and z follow).
        0x20 m_kContactPoint: u32,
        /// `m_kContactNormal` (Xbox PDB), x (y and z follow).
        0x2C m_kContactNormal: u32,
    }
}

/// `NiIntersector`'s vtable (`010f1a84`).
const NI_INTERSECTOR_VTABLE: u32 = 0x010f_1a84;
/// The `float` at `010f1a7c` (`FLT_MAX` in the executable): the contact point, normal and time
/// before a find, and the time of "no contact".
const NO_CONTACT_010F1A7C: u32 = 0x010f_1a7c;
/// The `float` at `0104da4c` (`2^-23` in the executable) a starting pseudodistance at or below
/// which counts as touching.
const TOUCHING_0104DA4C: u32 = 0x0104_da4c;
/// The `float` 0.0 at `01011d78` the end derivative is compared with.
const ZERO_01011D78: u32 = 0x0101_1d78;
/// `004390c0` (thiscall on a point, the other point): whether the three floats are equal.
const POINTS_EQUAL: u32 = 0x0043_90c0;
/// The memory manager instance and its `Deallocate` (`00aa4060`, thiscall: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

/// `FLD dword ptr [address]`.
fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

/// A `float` memory operand of an arithmetic instruction.
fn operand(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

// Translated from 00eae6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiIntersector::Test` (Xbox PDB): jumps to the object's `Find` (virtual slot +8) with the same
/// words; whatever it leaves is the answer (registered with the whole result passed on).
pub fn ni_intersector_test(e: &mut Engine, this: Ptr, velocity0: Ptr, velocity1: Ptr) -> Ret {
    e.vcall(this.addr(), 8, &args![velocity0, velocity1])
}

/// The uniform-form entry of [`ni_intersector_test`]: the tail jump's result unchanged.
fn ni_intersector_test_entry(e: &mut Engine, words: &[u32]) -> Ret {
    ni_intersector_test(
        e,
        Ptr::new(words[0]),
        Ptr::new(words[1]),
        Ptr::new(words[2]),
    )
}

// Translated from 00eae6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The contact time getter: `m_fContactTime` (+0x1c) as `FLD` leaves it in ST0.
pub fn fn_00eae6f0(e: &mut Engine, this: Ptr<NiIntersector>) -> f64 {
    x87_load32(e.get(this, NiIntersector::m_fContactTime))
}

// Translated from 00eae700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The contact point getter: the address of `m_kContactPoint` (+0x20).
pub fn fn_00eae700(_e: &mut Engine, this: Ptr<NiIntersector>) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x20))
}

// Translated from 00eae710 (decompiled, FalloutNV.exe 1.4.0.525)
/// The contact normal getter: the address of `m_kContactNormal` (+0x2c).
pub fn fn_00eae710(_e: &mut Engine, this: Ptr<NiIntersector>) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x2c))
}

// Translated from 00eae720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiIntersector` constructor (`RET 0x10`): the vtable, the contact point and normal set to
/// the float at `010f1a7c` (each `FST` of the loaded value), `m_iMaxIterations`, the type 0,
/// the maximum time and the time step (`FLD`/`FSTP`), the inverse step `1 / step`, the threshold,
/// and the contact time also the float at `010f1a7c`. Answers `this`.
pub fn fn_00eae720(
    e: &mut Engine,
    this: Ptr<NiIntersector>,
    max_time: f32,
    derivative_time_step: f32,
    pseudodistance_threshold: f32,
    max_iterations: i32,
) -> Ptr<NiIntersector> {
    let base = this.addr();
    let no_contact = x87_quiet_bits(e.mem.u32(NO_CONTACT_010F1A7C));
    e.mem.set_u32(base, NI_INTERSECTOR_VTABLE);
    for offset in (0x20..0x38u32).step_by(4) {
        e.mem.set_u32(base + offset, no_contact);
    }
    e.set(this, NiIntersector::m_iMaxIterations, max_iterations);
    e.set(this, NiIntersector::m_eIntersectionType, 0);
    e.set(
        this,
        NiIntersector::m_fMaxTime,
        x87_quiet_bits(max_time.to_bits()),
    );
    let step = x87_load32(derivative_time_step.to_bits());
    e.set(
        this,
        NiIntersector::m_fDerivativeTimeStep,
        x87_store32(step),
    );
    e.set(
        this,
        NiIntersector::m_fInvDerivativeTimeStep,
        x87_store32(x87_div(1.0, step)),
    );
    e.set(
        this,
        NiIntersector::m_fPseudodistanceThreshold,
        x87_quiet_bits(pseudodistance_threshold.to_bits()),
    );
    e.set(this, NiIntersector::m_fContactTime, no_contact);
    this
}

// Translated from 00eae770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiIntersector`'s scalar deleting destructor: sets the vtable and, when bit 0 of `flags` is
/// set, frees `this` through the memory manager (`00aa4060`). Answers `this`.
pub fn fn_00eae770(e: &mut Engine, this: Ptr<NiIntersector>, flags: u32) -> Ptr<NiIntersector> {
    e.mem.set_u32(this.addr(), NI_INTERSECTOR_VTABLE);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The intersector's `Pseudodistance` (virtual slot +0xc) at the time with the bits `time`,
/// stored as a float.
fn pseudodistance(e: &mut Engine, this: u32, velocity0: Ptr, velocity1: Ptr, time: u32) -> u32 {
    x87_store32(e.vcall(this, 0xc, &args![velocity0, velocity1, time]).f64())
}

/// Records the outcome: the time (bits) and the type, then for a contact (types 2 and 3) the
/// object's `ComputeContactInformation` (virtual slot +0x10).
fn finish(
    e: &mut Engine,
    this: Ptr<NiIntersector>,
    kind: u32,
    time: u32,
    velocity0: Ptr,
    velocity1: Ptr,
) {
    e.set(this, NiIntersector::m_fContactTime, time);
    e.set(this, NiIntersector::m_eIntersectionType, kind);
    if kind != 1 {
        e.vcall(this.addr(), 0x10, &args![velocity0, velocity1]);
    }
}

// Translated from 00eae7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiIntersector::Find` (Xbox PDB): finds the first time in `[0, m_fMaxTime]` at which the
/// pseudodistance (virtual slot +0xc, each value stored as a float) reaches the threshold, by
/// Newton steps with a backward difference of `m_fDerivativeTimeStep`:
/// - `p(0) <= -threshold`: overlapping at the start, type 3 at time 0, contact information.
/// - `p(0) <= 2^-23` (the float at `0104da4c`): touching, type 2 at time 0, contact information.
/// - equal velocities (`004390c0`): type 1, time `FLT_MAX` (`010f1a7c`).
/// - the derivative at 0, `(p(0) - p(-step)) / step`, at least 0 (separating): type 1.
/// - `p(max) > 0` and the derivative at the end, `(p(max) - p(max - step)) / step`, below 0
///   (the float at `01011d78`): type 1.
/// - otherwise from `t = 0` (with the derivative at 0), at most `m_iMaxIterations` times:
///   `t -= p / d` (stored); past the maximum time gives type 1; `p(t)` at most the threshold is
///   the contact; a derivative at `t` of at least 0 gives type 1. When the steps run out (or the
///   limit is below 1) the contact is at the last `t`. A contact is type 2 at `t`, with the
///   contact information. Unordered comparisons go on with the search.
pub fn ni_intersector_find(
    e: &mut Engine,
    this: Ptr<NiIntersector>,
    velocity0: Ptr,
    velocity1: Ptr,
) {
    let base = this.addr();
    let no_contact = x87_quiet_bits(e.mem.u32(NO_CONTACT_010F1A7C));
    let start = pseudodistance(e, base, velocity0, velocity1, 0);
    let start_value = x87_load32(start);
    if -load(e, base + 0x10) >= start_value {
        finish(e, this, 3, 0, velocity0, velocity1);
        return;
    }
    if start_value <= operand(e, TOUCHING_0104DA4C) {
        finish(e, this, 2, 0, velocity0, velocity1);
        return;
    }
    if e.call(POINTS_EQUAL, &args![velocity0, velocity1]).bool() {
        finish(e, this, 1, no_contact, velocity0, velocity1);
        return;
    }
    let back = x87_store32(x87_sub(0.0, load(e, base + 8)));
    let before = pseudodistance(e, base, velocity0, velocity1, back);
    let slope = x87_store32(x87_mul(
        x87_sub(start_value, x87_m32(before)),
        operand(e, base + 0xc),
    ));
    if x87_m32(slope) >= 0.0 {
        finish(e, this, 1, no_contact, velocity0, velocity1);
        return;
    }
    let max_time = x87_quiet_bits(e.mem.u32(base + 4));
    let at_end = pseudodistance(e, base, velocity0, velocity1, max_time);
    if 0.0 < x87_m32(at_end) {
        let near_end = x87_store32(x87_sub(load(e, base + 4), operand(e, base + 8)));
        let near_end = pseudodistance(e, base, velocity0, velocity1, x87_quiet_bits(near_end));
        let end_slope = x87_store32(x87_mul(
            x87_sub(x87_load32(at_end), x87_m32(near_end)),
            operand(e, base + 0xc),
        ));
        if x87_load32(end_slope) < operand(e, ZERO_01011D78) {
            finish(e, this, 1, no_contact, velocity0, velocity1);
            return;
        }
    }
    let mut time = 0u32;
    let mut value = x87_load32(start);
    let mut derivative = x87_load32(slope);
    let mut iteration = 1i32;
    while iteration <= e.get(this, NiIntersector::m_iMaxIterations) {
        time = x87_store32(x87_sub(x87_load32(time), x87_div(value, derivative)));
        let next = x87_load32(time);
        if load(e, base + 4) < next {
            finish(e, this, 1, no_contact, velocity0, velocity1);
            return;
        }
        let distance = pseudodistance(e, base, velocity0, velocity1, x87_quiet_bits(time));
        if load(e, base + 0x10) >= x87_load32(distance) {
            break;
        }
        let earlier = x87_store32(x87_sub(next, operand(e, base + 8)));
        let earlier = pseudodistance(e, base, velocity0, velocity1, x87_quiet_bits(earlier));
        value = x87_load32(distance);
        let step_slope = x87_store32(x87_mul(
            x87_sub(value, x87_m32(earlier)),
            operand(e, base + 0xc),
        ));
        derivative = x87_load32(step_slope);
        if derivative >= 0.0 {
            finish(e, this, 1, no_contact, velocity0, velocity1);
            return;
        }
        iteration += 1;
    }
    finish(e, this, 2, x87_quiet_bits(time), velocity0, velocity1);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00ea_e6e0, ni_intersector_test_entry as AbiFn),
        entry!(0x00ea_e6f0, fn_00eae6f0(Ptr<NiIntersector>) -> f64),
        entry!(0x00ea_e700, fn_00eae700(Ptr<NiIntersector>) -> Ptr),
        entry!(0x00ea_e710, fn_00eae710(Ptr<NiIntersector>) -> Ptr),
        entry!(
            0x00ea_e720,
            fn_00eae720(Ptr<NiIntersector>, f32, f32, f32, i32) -> Ptr<NiIntersector>
        ),
        entry!(
            0x00ea_e770,
            fn_00eae770(Ptr<NiIntersector>, u32) -> Ptr<NiIntersector>
        ),
        entry!(
            0x00ea_e7a0,
            ni_intersector_find(Ptr<NiIntersector>, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    const PSEUDODISTANCE: u32 = 0x00ab_2000;
    const CONTACT_INFORMATION: u32 = 0x00ab_2001;
    const FIND: u32 = 0x00ab_2002;

    fn constants(e: &mut Engine) {
        e.map(0x010f_1000, 0x1000);
        e.map(0x0104_d000, 0x1000);
        e.map(0x0101_1000, 0x1000);
        e.mem.set_u32(NO_CONTACT_010F1A7C, 0x7f7f_ffff);
        e.mem.set_u32(TOUCHING_0104DA4C, 0x3400_0000);
    }

    /// An intersector built by the constructor whose pseudodistance is `p(t)`.
    fn intersector(e: &mut Engine, max_time: f32, max_iterations: i32, p: fn(f32) -> f32) -> u32 {
        constants(e);
        let vtable = e.mem.alloc(0x14);
        e.mem.set_u32(vtable + 8, FIND);
        e.mem.set_u32(vtable + 0xc, PSEUDODISTANCE);
        e.mem.set_u32(vtable + 0x10, CONTACT_INFORMATION);
        e.register_double(PSEUDODISTANCE, move |_, a| Ret {
            st0: f64::from(p(f32::from_bits(a[3]))),
            ..Ret::default()
        });
        e.register(CONTACT_INFORMATION, |_, _| Ret::default());
        e.register(POINTS_EQUAL, |e, a| {
            (0..3)
                .all(|i| e.mem.u32(a[0] + 4 * i) == e.mem.u32(a[1] + 4 * i))
                .into_ret()
        });
        let this = e.mem.alloc(0x38);
        let r = e.call(
            0x00ea_e720,
            &args![this, max_time, 0.001f32, 0.0001f32, max_iterations],
        );
        assert_eq!(r.u32(), this);
        e.mem.set_u32(this, vtable);
        this
    }

    fn velocities(e: &mut Engine) -> (u32, u32) {
        let v0 = e.mem.alloc(12);
        let v1 = e.mem.alloc(12);
        e.mem.set_f32(v1 + 8, 1.0);
        (v0, v1)
    }

    fn find(e: &mut Engine, this: u32, v0: u32, v1: u32) -> (u32, f32) {
        e.call_log = Some(vec![]);
        e.call(0x00ea_e7a0, &args![this, v0, v1]);
        (e.mem.u32(this + 0x18), e.mem.f32(this + 0x1c))
    }

    #[test]
    fn constructor_and_getters() {
        let mut e = Engine::new();
        let this = intersector(&mut e, 2.0, 7, |_| 0.0);
        assert_eq!(e.mem.f32(this + 4), 2.0);
        assert_eq!(e.mem.f32(this + 8), 0.001);
        assert_eq!(e.mem.f32(this + 0xc), (1.0f64 / f64::from(0.001f32)) as f32);
        assert_eq!(e.mem.f32(this + 0x10), 0.0001);
        assert_eq!(e.mem.u32(this + 0x14), 7);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        for offset in [0x1cu32, 0x20, 0x28, 0x2c, 0x34] {
            assert_eq!(e.mem.u32(this + offset), 0x7f7f_ffff);
        }
        assert_eq!(e.call(0x00ea_e6f0, &args![this]).f64(), f64::from(f32::MAX));
        assert_eq!(e.call(0x00ea_e700, &args![this]).u32(), this + 0x20);
        assert_eq!(e.call(0x00ea_e710, &args![this]).u32(), this + 0x2c);
        // Test jumps to Find with the same words and passes its result on.
        e.register(FIND, |_, _| Ret {
            eax: 0x55,
            st0: 1.5,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let r = e.call(0x00ea_e6e0, &args![this, 1u32, 2u32]);
        assert_eq!((r.eax, r.st0), (0x55, 1.5));
        assert_eq!(calls_to(&e, FIND), vec![vec![this, 1, 2]]);
    }

    #[test]
    fn deleting_destructor_frees_with_bit_zero() {
        let mut e = Engine::new();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ea_e770, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_INTERSECTOR_VTABLE);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00ea_e770, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }

    #[test]
    fn find_at_the_start() {
        let mut e = Engine::new();
        let (v0, v1) = velocities(&mut e);
        let overlapping = intersector(&mut e, 10.0, 8, |_| -1.0);
        assert_eq!(find(&mut e, overlapping, v0, v1), (3, 0.0));
        assert_eq!(
            calls_to(&e, CONTACT_INFORMATION),
            vec![vec![overlapping, v0, v1]]
        );
        let touching = intersector(&mut e, 10.0, 8, |_| 0.0);
        assert_eq!(find(&mut e, touching, v0, v1), (2, 0.0));
        let apart = intersector(&mut e, 10.0, 8, |t| 4.0 - 2.0 * t);
        assert_eq!(find(&mut e, apart, v0, v0), (1, f32::MAX));
        assert!(calls_to(&e, CONTACT_INFORMATION).is_empty());
    }

    #[test]
    fn find_rejects_separating_and_short_approaches() {
        let mut e = Engine::new();
        let (v0, v1) = velocities(&mut e);
        let separating = intersector(&mut e, 10.0, 8, |t| 4.0 + 2.0 * t);
        assert_eq!(find(&mut e, separating, v0, v1), (1, f32::MAX));
        let short = intersector(&mut e, 10.0, 8, |t| 4.0 - 0.1 * t);
        assert_eq!(find(&mut e, short, v0, v1), (1, f32::MAX));
        // The first Newton step overshoots the maximum time.
        let curved = intersector(&mut e, 5.0, 8, |t| 4.0 - 0.5 * t - t * t);
        assert_eq!(find(&mut e, curved, v0, v1), (1, f32::MAX));
        assert!(calls_to(&e, CONTACT_INFORMATION).is_empty());
    }

    #[test]
    fn find_steps_to_the_contact() {
        let mut e = Engine::new();
        let (v0, v1) = velocities(&mut e);
        let approaching = intersector(&mut e, 10.0, 8, |t| 4.0 - 2.0 * t);
        let (kind, time) = find(&mut e, approaching, v0, v1);
        assert_eq!(kind, 2);
        assert!((time - 2.0).abs() < 0.01, "{time}");
        assert_eq!(calls_to(&e, CONTACT_INFORMATION).len(), 1);
        // No steps allowed: the contact stays at time 0.
        let no_steps = intersector(&mut e, 10.0, 0, |t| 4.0 - 2.0 * t);
        assert_eq!(find(&mut e, no_steps, v0, v1), (2, 0.0));
    }
}
