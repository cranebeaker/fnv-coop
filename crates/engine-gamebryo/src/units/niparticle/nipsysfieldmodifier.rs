//! `NiParticle/nipsysfieldmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c32100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysFieldModifier::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c31750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x30 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_field_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_1750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x30u32]);
    }
    this
}

// Translated from 00c320f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysFieldModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27e0.
pub fn ni_p_sys_field_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27e0
}

use crate::units::niparticle::nipsysmodifier::{
    add_class_name, add_labelled, stream_read, stream_write, NI_GET_VIEWER_STRING_BOOL,
    NI_GET_VIEWER_STRING_FIXED, NI_GET_VIEWER_STRING_FLOAT, READ_LINK_ID, STREAM_SAVE_LINK_ID,
};

layout! {
    /// `NiPSysFieldModifier` (Xbox PDB), 0x30 bytes; the PC code uses the
    /// same offsets. The floats are declared as their bit patterns.
    pub struct NiPSysFieldModifier: 0x30 {
        /// `m_pkFieldObj` (Xbox PDB): `NiAVObject*`.
        0x18 m_pkFieldObj: u32,
        /// `m_fMagnitude` (Xbox PDB).
        0x1C m_fMagnitude: u32,
        /// `m_fAttenuation` (Xbox PDB).
        0x20 m_fAttenuation: u32,
        /// `m_bUseMaxDistance` (Xbox PDB).
        0x24 m_bUseMaxDistance: u8,
        /// `m_fMaxDistance` (Xbox PDB).
        0x28 m_fMaxDistance: u32,
        /// `m_fMaxDistanceSqr` (Xbox PDB).
        0x2C m_fMaxDistanceSqr: u32,
    }
}

/// `NiPSysModifier::IsEqual` (`00c31830`), `LoadBinary` (`00c318b0`),
/// `SaveBinary` (`00c31950`) and `GetViewerStrings` (`00c319e0`):
/// `thiscall`, one word.
const MODIFIER_IS_EQUAL: u32 = 0x00c3_1830;
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
const MODIFIER_SAVE_BINARY: u32 = 0x00c3_1950;
const MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_19e0;
/// The `NiRTTI` record of `NiPSysFieldModifier` (`012027e0`).
const RTTI: u32 = 0x0120_27e0;
/// Labels: `"Field Object"`, `"Magnitude"`, `"Attenuation"`,
/// `"Use Max Distance"`, `"Max Distance"`, and `"None"` (the field object's
/// name when there is none).
const LABEL_FIELD_OBJECT: u32 = 0x010c_0c24;
const LABEL_MAGNITUDE: u32 = 0x0106_2590;
const LABEL_ATTENUATION: u32 = 0x010a_1574;
const LABEL_USE_MAX_DISTANCE: u32 = 0x010c_0c10;
const LABEL_MAX_DISTANCE: u32 = 0x010c_0c00;
const TEXT_NONE: u32 = 0x0102_909c;
/// `NiObjectNET::m_kName` (Xbox PDB), +8 in the field object.
const OBJECT_NAME: u32 = 0x8;

// Translated from 00c32060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysFieldModifier::IsEqual` (Xbox PDB): `NiPSysModifier::IsEqual`;
/// both or neither must have a field object, two field objects must be
/// equal (the first's `IsEqual`, slot +0x5c); then equal magnitudes and
/// attenuations (`FUCOMPP`: a NaN is unequal), the same use-max-distance
/// byte and equal maximum distances.
pub fn ni_p_sys_field_modifier_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysFieldModifier>,
    other: Ptr<NiPSysFieldModifier>,
) -> bool {
    if !e.call(MODIFIER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let mine = e.get(this, NiPSysFieldModifier::m_pkFieldObj);
    let theirs = e.get(other, NiPSysFieldModifier::m_pkFieldObj);
    if (mine != 0) != (theirs != 0) {
        return false;
    }
    if mine != 0 && !e.vcall(mine, 0x5c, &args![theirs]).bool() {
        return false;
    }
    let same_float =
        |e: &Engine, field| x87_load32(e.get(other, field)) == x87_load32(e.get(this, field));
    same_float(e, NiPSysFieldModifier::m_fMagnitude)
        && same_float(e, NiPSysFieldModifier::m_fAttenuation)
        && e.get(other, NiPSysFieldModifier::m_bUseMaxDistance)
            == e.get(this, NiPSysFieldModifier::m_bUseMaxDistance)
        && same_float(e, NiPSysFieldModifier::m_fMaxDistance)
}

// Translated from 00c32120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysFieldModifier::LoadBinary` (Xbox PDB): `NiPSysModifier`'s
/// `LoadBinary`, a link id (`00a645b0`, the field object), the magnitude
/// and attenuation (4 bytes each), one byte read into the low byte of the
/// stack word that holds the stream argument (the use-max-distance flag is
/// whether it is non-zero), the maximum distance (4 bytes), then its square
/// (`FMUL ST0`) into `m_fMaxDistanceSqr`. The reads' component sizes go
/// through the function's own stack word.
pub fn ni_p_sys_field_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysFieldModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(MODIFIER_LOAD_BINARY, &args![this, s]);
    e.call(READ_LINK_ID, &args![s]);
    e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let argument_slot = frame.addr() + 4;
        e.mem.set_u32(argument_slot, s);
        let field = |f: Field<NiPSysFieldModifier, u32>| this.addr() + f.off;
        stream_read(e, s, field(NiPSysFieldModifier::m_fMagnitude), 4, 4, local);
        stream_read(
            e,
            s,
            field(NiPSysFieldModifier::m_fAttenuation),
            4,
            4,
            local,
        );
        stream_read(e, s, argument_slot, 1, 1, local);
        let flag = u8::from(e.mem.u8(argument_slot) != 0);
        e.set(this, NiPSysFieldModifier::m_bUseMaxDistance, flag);
        stream_read(
            e,
            s,
            field(NiPSysFieldModifier::m_fMaxDistance),
            4,
            4,
            local,
        );
    });
    let distance = x87_load32(e.get(this, NiPSysFieldModifier::m_fMaxDistance));
    e.set(
        this,
        NiPSysFieldModifier::m_fMaxDistanceSqr,
        x87_store32(x87_mul(distance, distance)),
    );
}

// Translated from 00c321f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysFieldModifier::SaveBinary` (Xbox PDB): `NiPSysModifier`'s
/// `SaveBinary`, the field object's link id (stream slot +0x30), the
/// magnitude and attenuation (4 bytes each), the use-max-distance byte
/// (written from the low byte of the stack word that held the stream
/// argument) and the maximum distance (4 bytes); the component sizes go
/// through the function's own stack word.
pub fn ni_p_sys_field_modifier_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysFieldModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(MODIFIER_SAVE_BINARY, &args![this, s]);
    let object = e.get(this, NiPSysFieldModifier::m_pkFieldObj);
    e.vcall(s, STREAM_SAVE_LINK_ID, &args![object]);
    e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let argument_slot = frame.addr() + 4;
        e.mem.set_u32(argument_slot, s);
        let field = |f: Field<NiPSysFieldModifier, u32>| this.addr() + f.off;
        stream_write(e, s, field(NiPSysFieldModifier::m_fMagnitude), 4, 4, local);
        stream_write(
            e,
            s,
            field(NiPSysFieldModifier::m_fAttenuation),
            4,
            4,
            local,
        );
        let flag = e.get(this, NiPSysFieldModifier::m_bUseMaxDistance);
        e.mem.set_u8(argument_slot, flag);
        stream_write(e, s, argument_slot, 1, 1, local);
        stream_write(
            e,
            s,
            field(NiPSysFieldModifier::m_fMaxDistance),
            4,
            4,
            local,
        );
    });
}

// Translated from 00c322b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysFieldModifier::GetViewerStrings` (Xbox PDB):
/// `NiPSysModifier::GetViewerStrings`, then the class name (`NiRTTI`
/// `012027e0`), `"Field Object"` (the object's name handle, or the text
/// `"None"` without one), `"Magnitude"`, `"Attenuation"`,
/// `"Use Max Distance"` and `"Max Distance"` appended (the floats pushed
/// through `FLD`/`FSTP`).
pub fn ni_p_sys_field_modifier_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysFieldModifier>,
    strings: Ptr,
) {
    e.call(MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    let strings = strings.addr();
    add_class_name(e, strings, RTTI);
    let object = e.get(this, NiPSysFieldModifier::m_pkFieldObj);
    let name = if object == 0 {
        TEXT_NONE
    } else {
        e.mem.u32(object + OBJECT_NAME)
    };
    add_labelled(
        e,
        strings,
        NI_GET_VIEWER_STRING_FIXED,
        LABEL_FIELD_OBJECT,
        name,
    );
    let magnitude = x87_quiet_bits(e.get(this, NiPSysFieldModifier::m_fMagnitude));
    add_labelled(
        e,
        strings,
        NI_GET_VIEWER_STRING_FLOAT,
        LABEL_MAGNITUDE,
        magnitude,
    );
    let attenuation = x87_quiet_bits(e.get(this, NiPSysFieldModifier::m_fAttenuation));
    add_labelled(
        e,
        strings,
        NI_GET_VIEWER_STRING_FLOAT,
        LABEL_ATTENUATION,
        attenuation,
    );
    let use_max = u32::from(e.get(this, NiPSysFieldModifier::m_bUseMaxDistance));
    add_labelled(
        e,
        strings,
        NI_GET_VIEWER_STRING_BOOL,
        LABEL_USE_MAX_DISTANCE,
        use_max,
    );
    let max_distance = x87_quiet_bits(e.get(this, NiPSysFieldModifier::m_fMaxDistance));
    add_labelled(
        e,
        strings,
        NI_GET_VIEWER_STRING_FLOAT,
        LABEL_MAX_DISTANCE,
        max_distance,
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c3_2060,
            ni_p_sys_field_modifier_is_equal(
                Ptr<NiPSysFieldModifier>,
                Ptr<NiPSysFieldModifier>,
            ) -> bool
        ),
        entry!(
            0x00c3_2120,
            ni_p_sys_field_modifier_load_binary(Ptr<NiPSysFieldModifier>, Ptr)
        ),
        entry!(
            0x00c3_21f0,
            ni_p_sys_field_modifier_save_binary(Ptr<NiPSysFieldModifier>, Ptr)
        ),
        entry!(
            0x00c3_22b0,
            ni_p_sys_field_modifier_get_viewer_strings(Ptr<NiPSysFieldModifier>, Ptr)
        ),
        entry!(0x00c320f0, ni_p_sys_field_modifier_get_r_t_t_i() -> u32),
        entry!(0x00c32100, ni_p_sys_field_modifier_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_field_modifier_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c3_1750, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00c32100, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c32100, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x30]]);
        assert_eq!(calls_to(&e, 0x00c3_1750), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_field_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_20f0, &args![]).u32(), 0x0120_27e0);
    }
}

#[cfg(test)]
mod field_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream;
    use crate::units::niparticle::nipsysmodifier::test_stream::calls_to;

    fn field(e: &mut Engine) -> Ptr<NiPSysFieldModifier> {
        let m = e.new_object::<NiPSysFieldModifier>();
        e.set(m, NiPSysFieldModifier::m_fMagnitude, 2.0f32.to_bits());
        e.set(m, NiPSysFieldModifier::m_fAttenuation, 0.5f32.to_bits());
        e.set(m, NiPSysFieldModifier::m_bUseMaxDistance, 1);
        e.set(m, NiPSysFieldModifier::m_fMaxDistance, 10.0f32.to_bits());
        m
    }

    #[test]
    fn is_equal_checks_objects_then_fields() {
        const OBJECT_IS_EQUAL: u32 = 0x0f50_0000;
        let mut e = Engine::new();
        e.register(MODIFIER_IS_EQUAL, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(OBJECT_IS_EQUAL, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0] + 4) == e.mem.u32(a[1] + 4)),
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x60);
        e.mem.set_u32(vtable + 0x5c, OBJECT_IS_EQUAL);
        let object = |e: &mut Engine, tag: u32| {
            let o = e.mem.alloc(8);
            e.mem.set_u32(o, vtable);
            e.mem.set_u32(o + 4, tag);
            o
        };
        let a = field(&mut e);
        let b = field(&mut e);
        assert!(e.call(0x00c3_2060, &args![a, b]).bool());
        let o1 = object(&mut e, 1);
        e.set(a, NiPSysFieldModifier::m_pkFieldObj, o1);
        assert!(!e.call(0x00c3_2060, &args![a, b]).bool());
        assert!(!e.call(0x00c3_2060, &args![b, a]).bool());
        let o2 = object(&mut e, 2);
        e.set(b, NiPSysFieldModifier::m_pkFieldObj, o2);
        assert!(!e.call(0x00c3_2060, &args![a, b]).bool());
        e.mem.set_u32(o2 + 4, 1);
        assert!(e.call(0x00c3_2060, &args![a, b]).bool());
        for (off, word) in [(0x1cu32, 3.0f32.to_bits()), (0x20, 0x7fc0_0000), (0x28, 0)] {
            let c = field(&mut e);
            e.set(c, NiPSysFieldModifier::m_pkFieldObj, o2);
            e.mem.set_u32(c.addr() + off, word);
            assert!(!e.call(0x00c3_2060, &args![a, c]).bool(), "{off:#x}");
        }
        let c = field(&mut e);
        e.set(c, NiPSysFieldModifier::m_pkFieldObj, o2);
        e.set(c, NiPSysFieldModifier::m_bUseMaxDistance, 0);
        assert!(!e.call(0x00c3_2060, &args![a, c]).bool());
        e.register(MODIFIER_IS_EQUAL, |_, _| Ret::default());
        assert!(!e.call(0x00c3_2060, &args![a, b]).bool());
    }

    #[test]
    fn load_binary_reads_fields_and_squares_the_distance() {
        for (flag, expected) in [(0u8, 0u8), (5, 1)] {
            let mut e = Engine::new();
            e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
            e.register(READ_LINK_ID, |_, _| Ret::default());
            let mut bytes = 2.0f32.to_le_bytes().to_vec();
            bytes.extend_from_slice(&0.5f32.to_le_bytes());
            bytes.push(flag);
            bytes.extend_from_slice(&3.0f32.to_le_bytes());
            let stream = test_stream::new(&mut e, 0x1402_0007, &bytes);
            let this = e.new_object::<NiPSysFieldModifier>();
            e.call_log = Some(vec![]);
            e.call(0x00c3_2120, &args![this, stream]);
            assert_eq!(
                calls_to(&e, MODIFIER_LOAD_BINARY),
                vec![vec![this.addr(), stream]]
            );
            assert_eq!(calls_to(&e, READ_LINK_ID), vec![vec![stream]]);
            assert_eq!(
                e.get(this, NiPSysFieldModifier::m_fMagnitude),
                2.0f32.to_bits()
            );
            assert_eq!(
                e.get(this, NiPSysFieldModifier::m_fAttenuation),
                0.5f32.to_bits()
            );
            assert_eq!(
                e.get(this, NiPSysFieldModifier::m_bUseMaxDistance),
                expected
            );
            assert_eq!(
                e.get(this, NiPSysFieldModifier::m_fMaxDistance),
                3.0f32.to_bits()
            );
            assert_eq!(
                e.get(this, NiPSysFieldModifier::m_fMaxDistanceSqr),
                9.0f32.to_bits()
            );
            assert_eq!(test_stream::position(&e, stream), 13);
        }
    }

    #[test]
    fn save_binary_writes_link_and_fields() {
        const SAVE_LINK: u32 = 0x0f50_0010;
        let mut e = Engine::new();
        e.register(MODIFIER_SAVE_BINARY, |_, _| Ret::default());
        e.register(SAVE_LINK, |_, _| Ret::default());
        let writes = test_stream::writes(&mut e);
        let stream = test_stream::new_output(&mut e);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(stream, vtable);
        e.mem.set_u32(vtable + STREAM_SAVE_LINK_ID, SAVE_LINK);
        let this = field(&mut e);
        e.set(this, NiPSysFieldModifier::m_pkFieldObj, 0x7700);
        e.call_log = Some(vec![]);
        e.call(0x00c3_21f0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, MODIFIER_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(calls_to(&e, SAVE_LINK), vec![vec![stream, 0x7700]]);
        assert_eq!(
            *writes.borrow(),
            vec![
                (2.0f32.to_le_bytes().to_vec(), 4),
                (0.5f32.to_le_bytes().to_vec(), 4),
                (vec![1], 1),
                (10.0f32.to_le_bytes().to_vec(), 4),
            ]
        );
    }

    #[test]
    fn get_viewer_strings_appends_object_name_and_fields() {
        let mut e = Engine::new();
        test_stream::global_word(&mut e, RTTI, 0x0abc_0007);
        e.register(MODIFIER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        let stored = test_stream::viewer_strings(&mut e);
        let strings = test_stream::strings(&mut e);
        let this = field(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c3_22b0, &args![this, strings]);
        assert_eq!(
            *stored.borrow(),
            vec![
                0x0abc_0007,
                LABEL_FIELD_OBJECT,
                LABEL_MAGNITUDE,
                LABEL_ATTENUATION,
                LABEL_USE_MAX_DISTANCE,
                LABEL_MAX_DISTANCE
            ]
        );
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FIXED),
            vec![vec![LABEL_FIELD_OBJECT, TEXT_NONE]]
        );
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FLOAT),
            vec![
                vec![LABEL_MAGNITUDE, 2.0f32.to_bits()],
                vec![LABEL_ATTENUATION, 0.5f32.to_bits()],
                vec![LABEL_MAX_DISTANCE, 10.0f32.to_bits()],
            ]
        );
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_BOOL),
            vec![vec![LABEL_USE_MAX_DISTANCE, 1]]
        );
        // With a field object its name handle is shown.
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 8, 0x4321);
        e.set(this, NiPSysFieldModifier::m_pkFieldObj, object);
        e.call_log = Some(vec![]);
        e.call(0x00c3_22b0, &args![this, strings]);
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FIXED),
            vec![vec![LABEL_FIELD_OBJECT, 0x4321]]
        );
    }
}
