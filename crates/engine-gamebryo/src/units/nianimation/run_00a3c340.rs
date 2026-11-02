//! `NiAnimation/run_00a3c340` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `NiFloatInterpolator` (RTTI `.?AVNiFloatInterpolator@@`).
const NI_FLOAT_INTERPOLATOR_VTABLE: u32 = 0x0109_6f34;
/// The `float` the constructor gives `m_fFloatValue` (`0109a918`; the
/// Xbox PDB's `NiInterpolator::INVALID_FLOAT` by its use, not confirmed).
const INITIAL_FLOAT_VALUE: u32 = 0x0109_a918;
/// `NiFloatInterpolator` (Xbox PDB): `m_fFloatValue` +0x0c,
/// `m_spFloatData` +0x10 (`NiPointer<NiFloatData>`), `m_uiLastIdx` +0x14.
const FLOAT_VALUE: u32 = 0x0c;
const FLOAT_DATA: u32 = 0x10;
const LAST_INDEX: u32 = 0x14;
/// `NiKeyBasedInterpolator::NiKeyBasedInterpolator` (`00a3ca10`).
const NI_KEY_BASED_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_ca10;
/// Import slot of `InterlockedIncrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

// Translated from 00a3c340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpolator::NiFloatInterpolator(NiFloatData*)` (by the vtable
/// it stores; unnamed in the engine map): the `NiKeyBasedInterpolator`
/// constructor, the vtable, `m_fFloatValue` from the `float` at `0109a918`
/// (an x87 `FLD`/`FSTP` copy), `m_spFloatData = data` (its count
/// incremented when non-null) and `m_uiLastIdx = 0`. Returns `this`.
pub fn ni_float_interpolator_construct(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_KEY_BASED_INTERPOLATOR_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, NI_FLOAT_INTERPOLATOR_VTABLE);
    let value = e.global::<u32>(INITIAL_FLOAT_VALUE);
    e.mem
        .set_u32(base.wrapping_add(FLOAT_VALUE), x87_quiet_bits(value));
    e.mem.set_u32(base.wrapping_add(FLOAT_DATA), data.addr());
    if data.addr() != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![data.addr().wrapping_add(4)]);
    }
    e.mem.set_u32(base.wrapping_add(LAST_INDEX), 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a3_c340,
        ni_float_interpolator_construct(Ptr, Ptr) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_holds_the_data_and_clears_the_index() {
        let mut e = Engine::new();
        e.register(NI_KEY_BASED_INTERPOLATOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], value);
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        e.map(INITIAL_FLOAT_VALUE, 4);
        e.mem.set_u32(INITIAL_FLOAT_VALUE, 0xffa0_0000);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + LAST_INDEX, 9);
        let data = e.mem.alloc(0x18);
        e.mem.set_u32(data + 4, 2);
        assert_eq!(e.call(0x00a3_c340, &args![this, data]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_FLOAT_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + FLOAT_VALUE), 0xffe0_0000);
        assert_eq!(e.mem.u32(this + FLOAT_DATA), data);
        assert_eq!(e.mem.u32(data + 4), 3);
        assert_eq!(e.mem.u32(this + LAST_INDEX), 0);
        // Without data nothing is counted.
        e.mem.set_u32(INITIAL_FLOAT_VALUE, (-f32::MAX).to_bits());
        e.call_log = Some(vec![]);
        e.call(0x00a3_c340, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + FLOAT_DATA), 0);
        assert_eq!(e.mem.f32(this + FLOAT_VALUE), -f32::MAX);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|call| call.0 == INTERLOCKED_INCREMENT));
    }
}
