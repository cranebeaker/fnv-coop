//! `NiMain/nimaterialresource.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00eea750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialResource::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00eea5e0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x2c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_material_resource_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00ee_a5e0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x2cu32]);
    }
    this
}

/// `NiMaterialResource` (Xbox PDB) `m_kVariable` (+0x14), one of its five `NiFixedString`
/// handles: `m_kType` (+8), `m_kSemantic` (+0xc), `m_kLabel` (+0x10), `m_kVariable`,
/// `m_kDefaultValue` (+0x18); a handle's count is 8 bytes before it.
const VARIABLE: u32 = 0x14;
/// The handle of the empty `NiFixedString` (the word at `0109b220`), which is never counted.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// Import slot of `InterlockedDecrement`, `NiRefObject::ms_uiObjects` (`011f4400`) and
/// `NiRefObject`'s vtable (`0101dce4`).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const REF_OBJECT_COUNT: u32 = 0x011f_4400;
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;

// Translated from 00eea510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterialResource::GetVariable` (Xbox PDB): the address of `m_kVariable` (+0x14).
pub fn ni_material_resource_get_variable(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(VARIABLE))
}

// Translated from 00eea5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; the destructor body `00eea750`
/// runs): the five string handles released in reverse order (+0x18 down to +8; each count
/// dropped unless the handle is the empty string's), then the inlined `NiRefObject`
/// destructor (its vtable, `ms_uiObjects` dropped).
pub fn fn_00eea5e0(e: &mut Engine, this: Ptr) {
    let resource = this.addr();
    for offset in [0x18u32, 0x14, 0x10, 0xc, 8] {
        let handle = e.mem.u32(resource + offset);
        if handle != e.mem.u32(EMPTY_FIXED_STRING) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    }
    e.mem.set_u32(resource, NI_REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![REF_OBJECT_COUNT]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00eea750, ni_material_resource_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ee_a510, ni_material_resource_get_variable(Ptr) -> Ptr),
        entry!(0x00ee_a5e0, fn_00eea5e0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_material_resource_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ee_a5e0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00eea750, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00eea750, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x2c]]);
        assert_eq!(calls_to(&e, 0x00ee_a5e0), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;

    #[test]
    fn get_variable_answers_the_field_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ee_a510, &args![0x5000u32]).u32(), 0x5014);
    }

    #[test]
    fn destructor_body_drops_the_counted_strings_then_the_object_count() {
        let mut e = Engine::new();
        e.map(EMPTY_FIXED_STRING & !0xfff, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x0300_0008);
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        let this = e.mem.alloc(0x2c);
        for (i, offset) in [8u32, 0xc, 0x10, 0x14, 0x18].iter().enumerate() {
            e.mem.set_u32(this + offset, 0x0310_0008 + 0x100 * i as u32);
        }
        // m_kLabel is the empty string: not counted.
        e.mem.set_u32(this + 0x10, 0x0300_0008);
        e.call_log = Some(vec![]);
        e.call(0x00ee_a5e0, &args![this]);
        let dropped: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .filter(|c| c.0 == INTERLOCKED_DECREMENT)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(
            dropped,
            vec![
                0x0310_0400,
                0x0310_0300,
                0x0310_0100,
                0x0310_0000,
                REF_OBJECT_COUNT
            ]
        );
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
    }
}
