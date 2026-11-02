//! `Havok SDK/hkpkdtreeworldlinearcaster.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Offset of the hit fraction in the record `this + 0xC` points to.
const HIT_FRACTION: u32 = 4;

// Translated from 00d983f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKdTreeWorldLinearCaster::hasHit` (Xbox PDB): writes to the byte at `out` whether
/// `1.0` is greater than the hit fraction at `+4` of the record `this + 0xC` points to
/// (false for a NaN fraction, as the x87 compare is unordered).
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hkp_kd_tree_world_linear_caster_has_hit(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let record = e.mem.u32(this.addr() + 0xc);
    let found = 1.0f32 > e.mem.f32(record + HIT_FRACTION);
    e.mem.set_u8(out.addr(), u8::from(found));
    out
}

// Translated from 00d98420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKdTreeWorldLinearCaster::getHitFraction` (Xbox PDB): the float at `+4` of the record
/// `this + 0xC` points to, loaded on the x87 stack.
pub fn hkp_kd_tree_world_linear_caster_get_hit_fraction(e: &mut Engine, this: Ptr) -> f32 {
    let record = e.mem.u32(this.addr() + 0xc);
    x87_quiet(e.mem.f32(record + HIT_FRACTION))
}

// Translated from 00d98430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKdTreeWorldLinearCaster::addMidPhaseHit` (Xbox PDB): `result` is the byte this
/// writes (1 when the candidate was handed on, else 0). The candidate's first word is its
/// object; with none, or when it is the object already held at `this + 0x14`, nothing
/// happens. The filter at `this + 8` (slot `+4`, `(byte, this + 0x14 object, candidate)`,
/// returning a pointer to a byte) can reject it. Otherwise the 5-argument cdecl function
/// the dispatcher at `this + 0x20` selects for the pair is called: the byte at
/// `dispatcher + (([this + 0x18] + 0xD) << 5) + [object + 0xC]` indexes a table of 20-byte
/// entries at `dispatcher + 0x9AC`, whose first word is the function, called with
/// `([this + 0x14], candidate, this + 0x20, [this + 0xC], [this + 0x10])`.
/// Answers `result`: the exe leaves it in `EAX` at every `RET` (the hkBool result returned
/// through a pointer), and virtual call sites test the flag through `EAX` (e.g. 00d1b726).
pub fn hkp_kd_tree_world_linear_caster_add_mid_phase_hit(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    candidate: Ptr,
) -> Ptr {
    let this = this.addr();
    let candidate = candidate.addr();
    let object = e.mem.u32(candidate);
    let held = e.mem.u32(this + 0x14);
    let handed_on = if object == 0 || held == candidate {
        false
    } else {
        e.with_stack(4, |e, flag| {
            let filter = e.mem.u32(this + 8);
            let verdict = e.vcall(filter, 4, &args![flag, held, candidate]).u32();
            if e.mem.u8(verdict) == 0 {
                return false;
            }
            let dispatcher = e.mem.u32(this + 0x20);
            let row = (e.mem.u32(this + 0x18).wrapping_add(0xd) << 5)
                .wrapping_add(e.mem.u32(object + 0xc));
            let entry = u32::from(e.mem.u8(dispatcher.wrapping_add(row)));
            let function = e.mem.u32(dispatcher.wrapping_add(entry * 20 + 0x9ac));
            let user = e.mem.u32(this + 0xc);
            let output = e.mem.u32(this + 0x10);
            e.call(function, &args![held, candidate, this + 0x20, user, output]);
            true
        })
    };
    e.mem.set_u8(result.addr(), u8::from(handed_on));
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d9_83f0,
            hkp_kd_tree_world_linear_caster_has_hit(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00d9_8420,
            hkp_kd_tree_world_linear_caster_get_hit_fraction(Ptr) -> f32
        ),
        entry!(
            0x00d9_8430,
            hkp_kd_tree_world_linear_caster_add_mid_phase_hit(Ptr, Ptr, Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, new_log,
    };

    const FILTER: u32 = 0x0900_1000;
    const HANDLER: u32 = 0x0900_3000;

    fn caster_with_fraction(e: &mut Engine, bits: u32) -> u32 {
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record + 4, bits);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0xc, record);
        this
    }

    #[test]
    fn has_hit_00d983f0_is_true_for_fractions_below_one() {
        let mut e = engine();
        let out = e.mem.alloc(4);
        for (bits, expected) in [
            (0.5f32.to_bits(), 1u8),
            (1.0f32.to_bits(), 0),
            (2.0f32.to_bits(), 0),
            (f32::NAN.to_bits(), 0),
            ((-1.0f32).to_bits(), 1),
        ] {
            let this = caster_with_fraction(&mut e, bits);
            e.mem.set_u8(out, 0xee);
            e.call(0x00d9_83f0, &args![this, out]);
            assert_eq!(e.mem.u8(out), expected, "fraction bits {bits:08x}");
            assert_eq!(e.mem.u8(out + 1), 0);
        }
    }

    #[test]
    fn fn_00d983f0_answers_its_result_pointer() {
        let mut e = engine();
        let out = e.mem.alloc(4);
        let this = caster_with_fraction(&mut e, 0.5f32.to_bits());
        // EAX is `out` at both RETs (the hkBool result pointer the callers read through).
        assert_eq!(e.call(0x00d9_83f0, &args![this, out]).u32(), out);
    }

    #[test]
    fn get_hit_fraction_00d98420_loads_the_fraction_quietly() {
        let mut e = engine();
        let this = caster_with_fraction(&mut e, 0.25f32.to_bits());
        assert_eq!(e.call(0x00d9_8420, &args![this]).f32(), 0.25);
        let this = caster_with_fraction(&mut e, 0x7f80_0001);
        assert_eq!(
            e.call(0x00d9_8420, &args![this]).f32().to_bits(),
            0x7fc0_0001
        );
    }

    /// A caster whose filter answers `verdict`, whose held object is `held`, and whose
    /// dispatcher maps the pair (`[this + 0x18] = 2`, `[object + 0xC] = 3`) to table entry 2,
    /// whose function is the logging double `HANDLER`.
    fn add_hit_setup(
        e: &mut Engine,
        log: &crate::units::havok_sdk::hkpbvagent::test_support::Log,
        verdict: u8,
        held: u32,
    ) -> (u32, u32, u32) {
        let filter = logging_object(e, log, FILTER);
        let byte = e.mem.alloc(4);
        e.mem.set_u8(byte, verdict);
        log_double(e, log, FILTER + 4, byte);
        log_double(e, log, HANDLER, 0);
        let dispatcher = e.mem.alloc(0xa40);
        e.mem.set_u8(dispatcher + (0xf << 5) + 3, 2);
        e.mem.set_u32(dispatcher + 0x9ac + 2 * 20, HANDLER);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 0xc, 3);
        let candidate = e.mem.alloc(0x10);
        e.mem.set_u32(candidate, object);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 8, filter);
        e.mem.set_u32(this + 0xc, 0x1111);
        e.mem.set_u32(this + 0x10, 0x2222);
        e.mem.set_u32(this + 0x14, held);
        e.mem.set_u32(this + 0x18, 2);
        e.mem.set_u32(this + 0x20, dispatcher);
        (this, candidate, filter)
    }

    #[test]
    fn add_mid_phase_hit_00d98430_dispatches_an_accepted_pair() {
        let mut e = engine();
        let log = new_log();
        let (this, candidate, filter) = add_hit_setup(&mut e, &log, 1, 0x5000);
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 0xee);
        e.call(0x00d9_8430, &args![this, result, candidate]);
        assert_eq!(e.mem.u8(result), 1);
        let asked = calls(&log, FILTER + 4);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][0], filter);
        assert_eq!(asked[0][2..], [0x5000, candidate]);
        assert_eq!(
            calls(&log, HANDLER),
            vec![vec![0x5000, candidate, this + 0x20, 0x1111, 0x2222]]
        );
    }

    #[test]
    fn fn_00d98430_answers_its_result_pointer() {
        let mut e = engine();
        let log = new_log();
        let (this, candidate, _filter) = add_hit_setup(&mut e, &log, 1, 0x5000);
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 0xee);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(0x00d9_8430, &args![this, result, candidate]).u32(),
            args![result][0]
        );
    }

    #[test]
    fn add_mid_phase_hit_00d98430_stops_at_a_rejecting_filter() {
        let mut e = engine();
        let log = new_log();
        let (this, candidate, _) = add_hit_setup(&mut e, &log, 0, 0x5000);
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 0xee);
        e.call(0x00d9_8430, &args![this, result, candidate]);
        assert_eq!(e.mem.u8(result), 0);
        assert_eq!(calls(&log, FILTER + 4).len(), 1);
        assert!(calls(&log, HANDLER).is_empty());
    }

    #[test]
    fn add_mid_phase_hit_00d98430_skips_the_held_object_and_missing_objects() {
        let mut e = engine();
        let log = new_log();
        let (this, candidate, _) = add_hit_setup(&mut e, &log, 1, 0);
        let result = e.mem.alloc(4);
        // The candidate is the object already held.
        e.mem.set_u32(this + 0x14, candidate);
        e.mem.set_u8(result, 0xee);
        e.call(0x00d9_8430, &args![this, result, candidate]);
        assert_eq!(e.mem.u8(result), 0);
        // No object.
        e.mem.set_u32(this + 0x14, 0x5000);
        e.mem.set_u32(candidate, 0);
        e.mem.set_u8(result, 0xee);
        e.call(0x00d9_8430, &args![this, result, candidate]);
        assert_eq!(e.mem.u8(result), 0);
        assert!(log.borrow().is_empty());
    }
}
