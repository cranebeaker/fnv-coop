//! `fallout shared/tesattributes.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `memcpy` wrapper (`00401460`, `cdecl`: destination, source, size).
const MEMCPY: u32 = 0x0040_1460;
/// The folded `return 7` (`009d2510`, `ECX` = attributes): the size of the
/// attribute data.
const ATTRIBUTES_SIZE: u32 = 0x009d_2510;

// Translated from 00480130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the `TESAttributes` data from a buffer (unnamed in the map; `ECX`,
/// one stack word, `RET 4`): copies 7 bytes from the source to `+4`
/// (`00401460`) and answers the size `009d2510` gives, the number of bytes
/// used (`TESForm::LoadData` advances by it).
pub fn fn_00480130(e: &mut Engine, this: Ptr, source: Ptr) -> u32 {
    e.call(MEMCPY, &args![this.byte_add(4), source, 7u32]);
    e.call(ATTRIBUTES_SIZE, &args![this]).u32()
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable delta, source type, target type,
/// is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptor `.?AVBaseFormComponent@@`.
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
/// RTTI type descriptor `.?AVTESAttributes@@`.
const TYPE_TES_ATTRIBUTES: u32 = 0x0118_6c74;
/// RTTI type descriptor `.?AVTESForm@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// `ActorValue::ToActorValue` (engine map, `0066ec10`, `cdecl`, two words).
const TO_ACTOR_VALUE: u32 = 0x0066_ec10;

// Translated from 0047ffc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAttributes::InitializeDataComponent` (Xbox PDB), `ECX`, no stack words: sets each of
/// the 7 attribute bytes (`+4` to `+0xa`) to 5.
pub fn tes_attributes_initialize_data_component(e: &mut Engine, this: Ptr) {
    for index in 0..7u32 {
        e.mem.set_u8(this.addr() + index + 4, 5);
    }
}

// Translated from 00480000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAttributes::CopyComponent` (Xbox PDB), `ECX` and one stack word (`RET 4`): casts the
/// source component (`__RTDynamicCast` from `BaseFormComponent` to `TESAttributes`); when it
/// is one, for each attribute index 0 to 6 asks `ActorValue::ToActorValue(0, index)` and
/// copies the source's value for that code (`00480160`) into this attribute byte `+4 + index`.
pub fn tes_attributes_copy_component(e: &mut Engine, this: Ptr, source: Ptr) {
    let attributes = e
        .call(
            DYNAMIC_CAST,
            &args![
                source,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_ATTRIBUTES,
                0u32
            ],
        )
        .u32();
    if attributes == 0 {
        return;
    }
    for index in 0..7u32 {
        let code = e.call(TO_ACTOR_VALUE, &args![0u32, index & 0xff]).u32();
        let value = fn_00480160(e, Ptr::new(attributes), code) as u8;
        e.mem.set_u8(this.addr() + index + 4, value);
    }
}

// Translated from 00480160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range), `ECX` and one stack word (`RET 4`): the byte
/// at `this + code - 1` (the attribute an actor-value code names; codes start at 5, the first
/// attribute byte at `+4`). The exe answers it in `AL` after `EAX = this + code`
/// (`0048016a`), so `EAX`'s upper three bytes are that sum's (the oracle compares the whole
/// register; the callers forward `EAX`).
pub fn fn_00480160(e: &mut Engine, this: Ptr, code: u32) -> u32 {
    let sum = this.addr().wrapping_add(code);
    (sum & 0xffff_ff00) | u32::from(e.mem.u8(sum.wrapping_sub(1)))
}

// Translated from 00480180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAttributes::SetAttributeValue` (Xbox PDB), `ECX` and three stack words (`RET 0xc`):
/// stores `value` at `this + code - 1`; when `mark` is set, casts `this` (`__RTDynamicCast`
/// from `TESAttributes` to `TESForm`) and, when it is one, calls the form's vtable slot +0x48
/// with 4.
pub fn tes_attributes_set_attribute_value(
    e: &mut Engine,
    this: Ptr,
    code: u32,
    value: u8,
    mark: u8,
) {
    e.mem
        .set_u8(this.addr().wrapping_add(code).wrapping_sub(1), value);
    if mark == 0 {
        return;
    }
    let form = e
        .call(
            DYNAMIC_CAST,
            &args![this, 0u32, TYPE_TES_ATTRIBUTES, TYPE_TES_FORM, 0u32],
        )
        .u32();
    if form != 0 {
        e.vcall(form, 0x48, &args![4u32]);
    }
}

// Translated from 00480070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAttributes::CompareComponent` (Xbox PDB; `RET 4`): true when
/// `other` is not a `TESAttributes` (`__RTDynamicCast` from
/// `BaseFormComponent`) or when one of the 7 attribute bytes (+4..+0xA)
/// differs from the cast's value for that index (`ToActorValue(0, index)`,
/// then `00480160`'s low byte).
pub fn tes_attributes_compare_component(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let attributes = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_ATTRIBUTES,
                0u32
            ],
        )
        .u32();
    if attributes == 0 {
        return true;
    }
    for index in 0..7u32 {
        let mine = e.mem.u8(this.addr() + index + 4);
        let code = e.call(TO_ACTOR_VALUE, &args![0u32, index & 0xff]).u32();
        let theirs = fn_00480160(e, Ptr::new(attributes), code) as u8;
        if mine != theirs {
            return true;
        }
    }
    false
}

// Translated from 00480100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the `TESAttributes` data into a buffer (unnamed in the map; one
/// stack word, `RET 4`): copies the 7 bytes at +4 to the buffer
/// (`00401460`) and answers the size `009d2510` gives.
pub fn fn_00480100(e: &mut Engine, this: Ptr, buffer: Ptr) -> u32 {
    e.call(MEMCPY, &args![buffer, this.byte_add(4), 7u32]);
    e.call(ATTRIBUTES_SIZE, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00480130, fn_00480130(Ptr, Ptr) -> u32),
        entry!(0x0047_ffc0, tes_attributes_initialize_data_component(Ptr)),
        entry!(0x0048_0000, tes_attributes_copy_component(Ptr, Ptr)),
        entry!(0x0048_0160, fn_00480160(Ptr, u32) -> u32),
        entry!(
            0x0048_0180,
            tes_attributes_set_attribute_value(Ptr, u32, u8, u8)
        ),
        entry!(0x0048_0070, tes_attributes_compare_component(Ptr, Ptr) -> bool),
        entry!(0x0048_0100, fn_00480100(Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod component_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_0047ffc0_sets_seven_attributes_to_5() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xaa; 0x10]);
        e.call(0x0047_ffc0, &args![this]);
        let mut want = vec![0xaa; 0x10];
        want[4..0xb].fill(5);
        assert_eq!(e.mem.bytes(this, 0x10), want);
    }

    #[test]
    fn fn_00480160_reads_the_byte_before_this_plus_code() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u8(this + 4, 0x42);
        e.mem.set_u8(this + 0xa, 0x17);
        assert_eq!(e.call(0x0048_0160, &args![this, 5u32]).u32() & 0xff, 0x42);
        assert_eq!(e.call(0x0048_0160, &args![this, 0xbu32]).u32() & 0xff, 0x17);
        // EAX keeps the upper bytes of `this + code` (0048016a).
        assert_eq!(
            e.call(0x0048_0160, &args![this, 5u32]).u32() & 0xffff_ff00,
            (this + 5) & 0xffff_ff00
        );
    }

    #[test]
    fn fn_00480000_copies_each_value_from_a_cast_source() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let source = e.mem.alloc(0x20);
        let cast = e.mem.alloc(0x20);
        for i in 0..0x20u32 {
            e.mem.set_u8(cast + i, 0x60 + i as u8);
        }
        e.register_double(DYNAMIC_CAST, move |_, _| cast.into_ret());
        // Codes 10 - index: values from cast + 9 - index.
        e.register(TO_ACTOR_VALUE, |_, a| (10 - a[1]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0048_0000, &args![this, source]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![
                source,
                0,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_ATTRIBUTES,
                0
            ]]
        );
        let codes = calls(&e, TO_ACTOR_VALUE);
        assert_eq!(codes.len(), 7);
        assert_eq!(codes[6], vec![0, 6]);
        assert_eq!(
            e.mem.bytes(this + 4, 7),
            vec![0x69, 0x68, 0x67, 0x66, 0x65, 0x64, 0x63]
        );
    }

    #[test]
    fn fn_00480000_without_a_cast_copies_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0048_0000, &args![this, 0x1234u32]);
        assert!(calls(&e, TO_ACTOR_VALUE).is_empty());
        assert_eq!(e.mem.bytes(this, 0x10), vec![0; 0x10]);
    }

    #[test]
    fn fn_00480180_stores_and_marks_the_form() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let form = e.mem.alloc(8);
        e.put_vtable(0x0f00_3000, &[0; 0x13]);
        e.mem.set_u32(0x0f00_3048, 0x0f00_3100);
        e.mem.set_u32(form, 0x0f00_3000);
        e.register(0x0f00_3100, |_, _| Ret::default());
        e.register_double(DYNAMIC_CAST, move |_, _| form.into_ret());
        e.call_log = Some(vec![]);
        // Unmarked: stored only.
        e.call(0x0048_0180, &args![this, 6u32, 0x33u32, 0u32]);
        assert_eq!(e.mem.u8(this + 5), 0x33);
        assert!(calls(&e, DYNAMIC_CAST).is_empty());
        e.call(0x0048_0180, &args![this, 7u32, 0x44u32, 1u32]);
        assert_eq!(e.mem.u8(this + 6), 0x44);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![this, 0, TYPE_TES_ATTRIBUTES, TYPE_TES_FORM, 0]]
        );
        assert_eq!(calls(&e, 0x0f00_3100), vec![vec![form, 4]]);
        // Not a form: no call.
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call(0x0048_0180, &args![this, 7u32, 0x45u32, 1u32]);
        assert_eq!(calls(&e, 0x0f00_3100).len(), 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_seven_bytes_and_answers_the_size() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(ATTRIBUTES_SIZE, |_, _| Ret {
            eax: 9,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = Ptr::<()>::new(0x0400_0000);
        let source = Ptr::<()>::new(0x0500_0000);
        let used = e.call(0x0048_0130, &args![this, source]).u32();
        assert_eq!(used, 9);
        let calls: Vec<(u32, Vec<u32>)> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == MEMCPY || c.0 == ATTRIBUTES_SIZE)
            .collect();
        assert_eq!(
            calls,
            vec![
                (MEMCPY, vec![0x0400_0004, 0x0500_0000, 7]),
                (ATTRIBUTES_SIZE, vec![0x0400_0000]),
            ]
        );
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    #[test]
    fn compare_component_checks_the_seven_bytes() {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        // Code = index + 5, so `00480160` reads the byte at +4 + index.
        e.register(TO_ACTOR_VALUE, |_, a| (a[1] + 5).into_ret());
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        for i in 0..7 {
            e.mem.set_u8(this + 4 + i, i as u8 + 1);
            e.mem.set_u8(other + 4 + i, i as u8 + 1);
        }
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0048_0070, &args![this, other]).bool());
        let log = e.call_log.take().unwrap();
        let asked: Vec<Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == TO_ACTOR_VALUE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(asked, (0..7).map(|i| vec![0, i]).collect::<Vec<_>>());
        e.mem.set_u8(other + 4 + 3, 0x40);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_0070, &args![this, other]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == TO_ACTOR_VALUE).count(), 4);
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        assert!(e.call(0x0048_0070, &args![this, other]).bool());
    }

    #[test]
    fn write_copies_seven_bytes_out_and_answers_the_size() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(ATTRIBUTES_SIZE, |_, _| 7u32.into_ret());
        e.call_log = Some(vec![]);
        let size = e.call(0x0048_0100, &args![0x5000u32, 0x6000u32]).u32();
        assert_eq!(size, 7);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (MEMCPY, vec![0x6000, 0x5004, 7]),
                (ATTRIBUTES_SIZE, vec![0x5000])
            ]
        );
    }
}
