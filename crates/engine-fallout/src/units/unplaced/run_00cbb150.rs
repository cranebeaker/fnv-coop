//! `(unplaced)/run_00cbb150` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The absolute-value mask (`01017d10`, four words) and the tolerance
/// (`01017d00`, a `float`) the code reads.
const ABS_MASK: u32 = 0x0101_7d10;
const TOLERANCE: u32 = 0x0101_7d00;
/// `hkpEntity::activate` (`00c9c1d0`, engine map; thiscall, no stack
/// words).
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
/// The stored vector (+0x30, 16 bytes) and the object pointer (+0x18) whose
/// word at +8 must be set for the activation.
const STORED_VECTOR: u32 = 0x30;
const OBJECT: u32 = 0x18;

// Translated from 00cbb150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a new 4-float vector (thiscall, `RET 4`; unnamed in the engine
/// map). Unless the first three lanes of `|new - stored|` (`SUBPS`, then
/// `ANDPS` with the mask at `01017d10`) are all `<=` the tolerance at
/// `01017d00` (`CMPLEPS`, `MOVMSKPS & 7`), the object at +0x18 is activated
/// (`hkpEntity::activate`) when it and its word at +8 are non-zero. Then the
/// 16 bytes of `vector` are copied to +0x30.
pub fn fn_00cbb150(e: &mut Engine, this: Ptr, vector: Ptr) {
    let base = this.addr();
    let source = vector.addr();
    let lane = |e: &Engine, address: u32, i: u32| e.mem.f32(address + i * 4);
    let new: crate::sse::Xmm = std::array::from_fn(|i| lane(e, source, i as u32));
    let stored: crate::sse::Xmm = std::array::from_fn(|i| lane(e, base + STORED_VECTOR, i as u32));
    let difference = crate::sse::subps(new, stored);
    let absolute: crate::sse::Xmm = std::array::from_fn(|i| {
        f32::from_bits(difference[i].to_bits() & e.mem.u32(ABS_MASK + i as u32 * 4))
    });
    let tolerance = e.global::<f32>(TOLERANCE);
    let within = crate::sse::cmpps(absolute, [tolerance; 4], 2);
    let mask = (0..4).fold(0u32, |mask, i| mask | ((within[i].to_bits() >> 31) << i));
    if mask & 7 != 7 {
        let object = e.mem.u32(base + OBJECT);
        if object != 0 && e.mem.u32(object + 8) != 0 {
            e.call(ENTITY_ACTIVATE, &args![object]);
        }
    }
    let bytes = e.mem.bytes(source, 16);
    e.mem.write(base + STORED_VECTOR, &bytes);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cb_b150, fn_00cbb150(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(e: &mut Engine) -> (u32, u32) {
        e.map(ABS_MASK, 16);
        for i in 0..4 {
            e.mem.set_u32(ABS_MASK + i * 4, 0x7fff_ffff);
        }
        e.map(TOLERANCE, 4);
        e.mem.set_u32(TOLERANCE, 0x3a83_126f);
        e.register(ENTITY_ACTIVATE, |_, _| Ret::default());
        let object = e.mem.alloc(0xc);
        e.mem.set_u32(object + 8, 1);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + OBJECT, object);
        for (i, value) in [1.0f32, 2.0, 3.0, 4.0].iter().enumerate() {
            e.mem.set_f32(this + STORED_VECTOR + i as u32 * 4, *value);
        }
        (this, object)
    }

    fn store(e: &mut Engine, this: u32, values: [f32; 4]) -> bool {
        let vector = e.mem.alloc(16);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(vector + i as u32 * 4, *value);
        }
        e.call_log = Some(vec![]);
        e.call(0x00cb_b150, &args![this, vector]);
        assert_eq!(
            e.mem.bytes(this + STORED_VECTOR, 16),
            e.mem.bytes(vector, 16)
        );
        e.call_log
            .take()
            .unwrap()
            .iter()
            .any(|(a, _)| *a == ENTITY_ACTIVATE)
    }

    #[test]
    fn a_small_change_in_the_first_three_lanes_does_not_activate() {
        let mut e = Engine::new();
        let (this, _) = setup(&mut e);
        // Within 0.001 on x, y, z; w may change freely.
        assert!(!store(&mut e, this, [1.0005, 1.9995, 3.0, 40.0]));
    }

    #[test]
    fn a_larger_change_or_a_nan_activates_the_object() {
        let mut e = Engine::new();
        let (this, object) = setup(&mut e);
        assert!(store(&mut e, this, [1.0, 2.0, 3.01, 4.0]));
        assert!(store(&mut e, this, [f32::NAN, 2.0, 3.01, 4.0]));
        // Without the object's word at +8, or without an object, nothing.
        e.mem.set_u32(object + 8, 0);
        assert!(!store(&mut e, this, [9.0, 2.0, 3.0, 4.0]));
        e.mem.set_u32(this + OBJECT, 0);
        assert!(!store(&mut e, this, [1.0, 2.0, 3.0, 4.0]));
    }
}
