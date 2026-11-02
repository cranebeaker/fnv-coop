//! `NiAnimation/nisequencestreamhelper.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSequenceStreamHelper::ms_RTTI` (the class's `NiRTTI` record).
const NI_SEQUENCE_STREAM_HELPER_RTTI: u32 = 0x011f_3f38;

// Translated from 00a47370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSequenceStreamHelper::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_sequence_stream_helper_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SEQUENCE_STREAM_HELPER_RTTI)
}

/// `00a5c080`: the jump target of `00a47350`.
const JUMP_TARGET_00A47350: u32 = 0x00a5_c080;

/// `00a5c0e0`: the jump target of `00a47360`.
const JUMP_TARGET_00A47360: u32 = 0x00a5_c0e0;

/// `00a5c5b0`: the jump target of `00a5d7b0`.
const JUMP_TARGET_00A5D7B0: u32 = 0x00a5_c5b0;

/// `00a5c180`: the jump target of `00a62130`.
const JUMP_TARGET_00A62130: u32 = 0x00a5_c180;

/// `00a5c820`: the jump target of `00a62140`.
const JUMP_TARGET_00A62140: u32 = 0x00a5_c820;

// Translated from 00a47350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSequenceStreamHelper::LoadBinary` (Xbox PDB): a 5-byte jump to `00a5c080` (engine map: `NiObjectNET::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiSequenceStreamHelper@@UAAXAAVNiStream@@@Z`.
pub fn ni_sequence_stream_helper_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A47350, &args![this, stream]);
}

// Translated from 00a47360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSequenceStreamHelper::LinkObject` (Xbox PDB): a 5-byte jump to `00a5c0e0` (engine map: `NiObjectNET::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiSequenceStreamHelper@@UAAXAAVNiStream@@@Z`.
pub fn ni_sequence_stream_helper_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A47360, &args![this, stream]);
}

// Translated from 00a5d7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSequenceStreamHelper::SaveBinary` (Xbox PDB): a 5-byte jump to `00a5c5b0` (engine map: `NiObjectNET::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiSequenceStreamHelper@@UAAXAAVNiStream@@@Z`.
pub fn ni_sequence_stream_helper_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A5D7B0, &args![this, stream]);
}

// Translated from 00a62130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSequenceStreamHelper::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a5c180` (engine map: `NiObjectNET::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiSequenceStreamHelper@@UAA_NAAVNiStream@@@Z`.
pub fn ni_sequence_stream_helper_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00A62130, &args![this, stream]).bool()
}

// Translated from 00a62140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSequenceStreamHelper::IsEqual` (Xbox PDB): a 5-byte jump to `00a5c820` (engine map: `NiObjectNET::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiSequenceStreamHelper@@UAA_NPAVNiObject@@@Z`.
pub fn ni_sequence_stream_helper_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A62140, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a47370, ni_sequence_stream_helper_get_rtti() -> Ptr),
        entry!(0x00a4_7350, ni_sequence_stream_helper_load_binary(Ptr, Ptr)),
        entry!(0x00a4_7360, ni_sequence_stream_helper_link_object(Ptr, Ptr)),
        entry!(0x00a5_d7b0, ni_sequence_stream_helper_save_binary(Ptr, Ptr)),
        entry!(0x00a6_2130, ni_sequence_stream_helper_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a6_2140, ni_sequence_stream_helper_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_sequence_stream_helper_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a47370, &args![]).u32(),
            NI_SEQUENCE_STREAM_HELPER_RTTI
        );
    }

    #[test]
    fn thunk_00a47350_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_7350;
        const TARGET: u32 = 0x00a5_c080;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a47360_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_7360;
        const TARGET: u32 = 0x00a5_c0e0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a5d7b0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a5_d7b0;
        const TARGET: u32 = 0x00a5_c5b0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a62130_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a6_2130;
        const TARGET: u32 = 0x00a5_c180;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00a62140_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a6_2140;
        const TARGET: u32 = 0x00a5_c820;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}
