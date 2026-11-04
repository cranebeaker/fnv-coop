//! `(unplaced)/run_00eea550` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The handle of the empty `NiFixedString` (the word at `0109b220`).
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// Import slot of `InterlockedIncrement` and `NiRefObject::ms_uiObjects` (`011f4400`).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const REF_OBJECT_COUNT: u32 = 0x011f_4400;
/// The vtables the constructor stores: `NiRefObject` (`0101dce4`), then
/// `NiMaterialResource` (`01101464`; both named by their RTTI).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const NI_MATERIAL_RESOURCE_VTABLE: u32 = 0x0110_1464;

// Translated from 00eea550 (decompiled, FalloutNV.exe 1.4.0.525)
/// A setter (unnamed in the map; `thiscall`, one word): stores `value` at `this + 0x20`
/// (`NiMaterialResource::m_eSource` (Xbox PDB) on the resource the GPU program cache sets).
pub fn fn_00eea550(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x20), value);
}

// Translated from 00eea560 (decompiled, FalloutNV.exe 1.4.0.525)
/// A setter (unnamed in the map; `thiscall`, one word): stores `value` at `this + 0x24`
/// (`NiMaterialResource::m_uiExtraData` (Xbox PDB) on the resource the GPU program cache
/// sets).
pub fn fn_00eea560(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x24), value);
}

// Translated from 00eea570 (decompiled, FalloutNV.exe 1.4.0.525)
/// A setter (unnamed in the map; `thiscall`, one word): stores `value` at `this + 0x28`
/// (`NiMaterialResource::m_eObjectType` (Xbox PDB) on the resource the GPU program cache
/// sets).
pub fn fn_00eea570(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x28), value);
}

// Translated from 00eea580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiMaterialResource` constructor (unnamed in the map; `thiscall`, no stack words):
/// the inlined `NiRefObject` constructor (its vtable, `m_uiRefCount` 0,
/// `ms_uiObjects` raised through `InterlockedIncrement`), then the `NiMaterialResource`
/// vtable, the five `NiFixedString` handles +8..+0x18 set to the empty handle (re-read from
/// `0109b220` for each, not counted), +0x20, +0x24, +0x28 cleared and `m_uiCount` (+0x1c)
/// set to 1. Answers `this`.
pub fn fn_00eea580(e: &mut Engine, this: Ptr) -> Ptr {
    let resource = this.addr();
    e.mem.set_u32(resource, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(resource.wrapping_add(4), 0);
    e.call(INTERLOCKED_INCREMENT, &args![REF_OBJECT_COUNT]);
    e.mem.set_u32(resource, NI_MATERIAL_RESOURCE_VTABLE);
    for offset in [8u32, 0xc, 0x10, 0x14, 0x18] {
        let empty = e.mem.u32(EMPTY_FIXED_STRING);
        e.mem.set_u32(resource.wrapping_add(offset), empty);
    }
    e.mem.set_u32(resource.wrapping_add(0x20), 0);
    e.mem.set_u32(resource.wrapping_add(0x24), 0);
    e.mem.set_u32(resource.wrapping_add(0x28), 0);
    e.mem.set_u32(resource.wrapping_add(0x1c), 1);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee_a550, fn_00eea550(Ptr, u32)),
        entry!(0x00ee_a560, fn_00eea560(Ptr, u32)),
        entry!(0x00ee_a570, fn_00eea570(Ptr, u32)),
        entry!(0x00ee_a580, fn_00eea580(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00eea550_stores_the_word_at_0x20() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x2c);
        e.call(0x00ee_a550, &args![this, 0x1234u32]);
        assert_eq!(e.mem.u32(this + 0x20), 0x1234);
        assert_eq!(e.mem.u32(this + 0x24), 0);
    }

    #[test]
    fn fn_00eea560_stores_the_word_at_0x24() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x2c);
        e.call(0x00ee_a560, &args![this, 0x5678u32]);
        assert_eq!(e.mem.u32(this + 0x24), 0x5678);
        assert_eq!(e.mem.u32(this + 0x20), 0);
    }

    #[test]
    fn fn_00eea570_stores_the_word_at_0x28() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x2c);
        e.call(0x00ee_a570, &args![this, 0x9abcu32]);
        assert_eq!(e.mem.u32(this + 0x28), 0x9abc);
        assert_eq!(e.mem.u32(this + 0x24), 0);
    }

    #[test]
    fn fn_00eea580_builds_an_empty_resource_and_counts_the_object() {
        let mut e = Engine::new();
        // The import is registered only with the exe: a counting double.
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.map(EMPTY_FIXED_STRING & !0xfff, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x0300_0008);
        e.map(REF_OBJECT_COUNT & !0xfff, 0x1000);
        e.mem.set_u32(REF_OBJECT_COUNT, 7);
        let this = e.mem.alloc(0x2c);
        for offset in (0..0x2c).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        assert_eq!(e.call(0x00ee_a580, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_MATERIAL_RESOURCE_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        for offset in [8u32, 0xc, 0x10, 0x14, 0x18] {
            assert_eq!(e.mem.u32(this + offset), 0x0300_0008);
        }
        assert_eq!(e.mem.u32(this + 0x1c), 1);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(REF_OBJECT_COUNT), 8);
    }
}
