//! `BSAudio/bsxenonaudiolistener.cpp` (Xbox PDB source unit), subsystem `BSAudio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSAudioListener::iEvironmentType` (Xbox PDB spelling), +0x44.
const ENVIRONMENT_TYPE: u32 = 0x44;

// Translated from 00aeab50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioListener::SetEnvironmentType` (Xbox PDB): stores `kind` in
/// `iEvironmentType` (+0x44).
pub fn bs_audio_listener_set_environment_type(e: &mut Engine, this: Ptr, kind: u32) {
    e.mem
        .set_u32(this.addr().wrapping_add(ENVIRONMENT_TYPE), kind);
}

// Translated from 00aeab70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The listener's environment-type getter (the vtable slot after the
/// setter; `GetEnvironmentType` in the Xbox PDB's order, not confirmed):
/// returns `iEvironmentType` (+0x44).
pub fn fn_00aeab70(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(ENVIRONMENT_TYPE))
}

/// `BSAudioListener`'s virtual table (`010a3bb4`, RTTI `.?AVBSAudioListener@@`), the heap the
/// deleting destructor frees into (`011f6238`) and its free (`00aa4060`, `thiscall`, one
/// word).
const LISTENER_VTABLE: u32 = 0x010a_3bb4;
const HEAP: u32 = 0x011f_6238;
const HEAP_FREE: u32 = 0x00aa_4060;
/// `BSAudioListener::fRolloffFactor` (+0x3c) and `fGain` (+0x40) (Xbox PDB).
const ROLLOFF_FACTOR: u32 = 0x3c;
const GAIN: u32 = 0x40;

// Translated from 00aeaad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioListener::GetPosition` (Xbox PDB; `thiscall`, no stack words): the address of
/// `ListenerPosition` (+4).
pub fn bs_audio_listener_get_position(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(4))
}

// Translated from 00aeaaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioListener::GetFrontOrientation` (Xbox PDB; `thiscall`, no stack words): the
/// address of `FrontOrientation` (+0x10).
pub fn bs_audio_listener_get_front_orientation(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x10))
}

// Translated from 00aeab10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; slot +0x1c of the
/// `BSAudioListener` vtable, `GetTopOrientation` in the Xbox PDB's order, not confirmed): the
/// address of +0x1c (`TopOrientation` in the Xbox PDB layout).
pub fn fn_00aeab10(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x1c))
}

// Translated from 00aeab30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioListener::SetGain` (Xbox PDB; `thiscall`, one word): stores `gain` in `fGain`
/// (+0x40), copied through `FLD`/`FSTP` (a signalling NaN arrives quieted).
pub fn bs_audio_listener_set_gain(e: &mut Engine, this: Ptr, gain: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(GAIN),
        x87_quiet_bits(gain.to_bits()),
    );
}

// Translated from 00aeab90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioListener::SetRolloffFactor` (Xbox PDB; `thiscall`, one word): stores `factor` in
/// `fRolloffFactor` (+0x3c), copied through `FLD`/`FSTP` (a signalling NaN arrives quieted).
pub fn bs_audio_listener_set_rolloff_factor(e: &mut Engine, this: Ptr, factor: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(ROLLOFF_FACTOR),
        x87_quiet_bits(factor.to_bits()),
    );
}

// Translated from 00aeabb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudioListener::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, one word of
/// flags): the inlined destructor (its virtual table `010a3bb4`, `iEvironmentType` (+0x44)
/// cleared), then with flag bit 0 the block freed into the heap at `011f6238` (`00aa4060`).
/// Answers `this`.
pub fn bs_audio_listener_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), LISTENER_VTABLE);
    e.mem.set_u32(this.addr().wrapping_add(ENVIRONMENT_TYPE), 0);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ae_aad0, bs_audio_listener_get_position(Ptr) -> Ptr),
        entry!(
            0x00ae_aaf0,
            bs_audio_listener_get_front_orientation(Ptr) -> Ptr
        ),
        entry!(0x00ae_ab10, fn_00aeab10(Ptr) -> Ptr),
        entry!(0x00ae_ab30, bs_audio_listener_set_gain(Ptr, f32)),
        entry!(0x00ae_ab90, bs_audio_listener_set_rolloff_factor(Ptr, f32)),
        entry!(
            0x00ae_abb0,
            bs_audio_listener_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00ae_ab50,
            bs_audio_listener_set_environment_type(Ptr, u32)
        ),
        entry!(0x00ae_ab70, fn_00aeab70(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_type_round_trip() {
        let mut e = Engine::new();
        let listener = e.mem.alloc(0x7c);
        e.call(0x00ae_ab50, &args![listener, 6u32]);
        assert_eq!(e.mem.u32(listener + 0x44), 6);
        e.mem.set_u32(listener + 0x44, 9);
        assert_eq!(e.call(0x00ae_ab70, &args![listener]).u32(), 9);
    }
}

#[cfg(test)]
mod n082_tests {
    use super::*;

    #[test]
    fn getters_answer_the_vector_addresses() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ae_aad0, &args![0x5000u32]).u32(), 0x5004);
        assert_eq!(e.call(0x00ae_aaf0, &args![0x5000u32]).u32(), 0x5010);
        assert_eq!(e.call(0x00ae_ab10, &args![0x5000u32]).u32(), 0x501c);
    }

    #[test]
    fn set_gain_stores_at_0x40_quieting_a_signalling_nan() {
        let mut e = Engine::new();
        let listener = e.mem.alloc(0x48);
        e.call(0x00ae_ab30, &args![listener, 0.75f32]);
        assert_eq!(e.mem.f32(listener + 0x40), 0.75);
        assert_eq!(e.mem.u32(listener + 0x3c), 0);
        e.call(0x00ae_ab30, &args![listener, 0xff80_0001u32]);
        assert_eq!(e.mem.u32(listener + 0x40), 0xffc0_0001);
    }

    #[test]
    fn set_rolloff_factor_stores_at_0x3c_quieting_a_signalling_nan() {
        let mut e = Engine::new();
        let listener = e.mem.alloc(0x48);
        e.call(0x00ae_ab90, &args![listener, 2.5f32]);
        assert_eq!(e.mem.f32(listener + 0x3c), 2.5);
        assert_eq!(e.mem.u32(listener + 0x40), 0);
        e.call(0x00ae_ab90, &args![listener, 0x7f80_0002u32]);
        assert_eq!(e.mem.u32(listener + 0x3c), 0x7fc0_0002);
    }

    #[test]
    fn scalar_deleting_destructor_resets_and_frees_only_with_flag_bit_0() {
        let mut e = Engine::new();
        e.register(HEAP_FREE, |_, _| Ret::default());
        let listener = e.mem.alloc(0x48);
        e.mem.set_u32(listener, 0x1234);
        e.mem.set_u32(listener + 0x44, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_abb0, &args![listener, 2u32]).u32(), listener);
        assert_eq!(e.mem.u32(listener), LISTENER_VTABLE);
        assert_eq!(e.mem.u32(listener + 0x44), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.call(0x00ae_abb0, &args![listener, 1u32]);
        let frees: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == HEAP_FREE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(frees, vec![vec![HEAP, listener]]);
    }
}
