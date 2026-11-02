//! `Havok SDK/hkclassmemberaccessor.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkClass::getMemberByName` (`00d46fe0`, `this` = the class record, `(name)`).
const CLASS_GET_MEMBER_BY_NAME: u32 = 0x00d4_6fe0;

// An accessor is two words: the address of the member's data (`+0`, 0 when the member was not
// found) and the member record (`+4`).

// Translated from 00d87130 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = an accessor, then `(address, member)`, `RET 8`): stores the member
/// record (`+4`) and the address of its data, the base `address` plus the member's 16-bit
/// offset (`+0x12`), at `+0`.
pub fn fn_00d87130(e: &mut Engine, this: Ptr, address: u32, member: Ptr) {
    let offset = u32::from(e.mem.u16(member.addr() + 0x12));
    e.mem.set_u32(this.addr() + 4, member.addr());
    e.mem.set_u32(this.addr(), offset.wrapping_add(address));
}

// Translated from 00d87150 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = an accessor, then `(address, class, name)`, `RET 0xC`): looks the
/// member up by name in the class record (`hkClass::getMemberByName`, `00d46fe0`) and stores
/// it at `+4`. The data address (`+0`) is 0 when there is no such member, otherwise the base
/// `address` plus the member's 16-bit offset (`+0x12`).
pub fn fn_00d87150(e: &mut Engine, this: Ptr, address: u32, class: Ptr, name: u32) {
    e.mem.set_u32(this.addr(), 0);
    let member = e.call(CLASS_GET_MEMBER_BY_NAME, &args![class, name]).u32();
    e.mem.set_u32(this.addr() + 4, member);
    if member != 0 {
        let offset = u32::from(e.mem.u16(member + 0x12));
        e.mem.set_u32(this.addr(), offset.wrapping_add(address));
    }
}

// Translated from 00d87180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMemberAccessor::hkClassMemberAccessor` (Xbox PDB; `this` = the accessor, then
/// `(address, member)`, `RET 8`): [`fn_00d87130`]; answers `this`.
pub fn hk_class_member_accessor_construct(
    e: &mut Engine,
    this: Ptr,
    address: u32,
    member: Ptr,
) -> Ptr {
    fn_00d87130(e, this, address, member);
    this
}

// Translated from 00d871a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMemberAccessor::hkClassMemberAccessor` (the second overload, Xbox PDB; `this` = the
/// accessor, then `(source, name)`, `RET 8`): [`fn_00d87150`] with the two words of `source`
/// (the base address, then the class record) and the member `name`; answers `this`.
pub fn hk_class_member_accessor_construct_ov2(
    e: &mut Engine,
    this: Ptr,
    source: Ptr,
    name: u32,
) -> Ptr {
    let address = e.mem.u32(source.addr());
    let class = e.mem.u32(source.addr() + 4);
    fn_00d87150(e, this, address, Ptr::new(class), name);
    this
}

// Translated from 00d871e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMemberAccessor::isOk` (Xbox PDB; `this` = the accessor, then `out`, `RET 4`):
/// stores at the byte `out` whether the data address is not 0; answers `out`.
pub fn hk_class_member_accessor_is_ok(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let ok = e.mem.u32(this.addr()) != 0;
    e.mem.set_u8(out.addr(), u8::from(ok));
    out.addr()
}

// Translated from 00d87200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMemberAccessor::getAddress` (Xbox PDB): the data address (`+0`).
pub fn hk_class_member_accessor_get_address(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr())
}

/// The address of element `index` of `stride`-byte elements starting at the data address.
fn element_address(e: &Engine, this: Ptr, index: i32, stride: u32) -> u32 {
    e.mem
        .u32(this.addr())
        .wrapping_add((index as u32).wrapping_mul(stride))
}

// Translated from 00d87210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMemberAccessor::asPointer` (Xbox PDB; `this` = the accessor, then `index`,
/// `RET 4`): the address of element `index` of four-byte elements at the data address.
pub fn hk_class_member_accessor_as_pointer(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 4)
}

// Translated from 00d87220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMemberAccessor::asCstring` (Xbox PDB; `this` = the accessor, then `index`,
/// `RET 4`): the address of element `index` of four-byte elements at the data address.
pub fn hk_class_member_accessor_as_cstring(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 4)
}

// Translated from 00d87230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMemberAccessor::asStringPtr` (Xbox PDB; `this` = the accessor, then `index`,
/// `RET 4`): the address of element `index` of four-byte elements at the data address.
pub fn hk_class_member_accessor_as_string_ptr(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 4)
}

// Translated from 00d87250 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = the accessor, then `index`, `RET 4`): the address of element `index`
/// of four-byte elements at the data address.
pub fn fn_00d87250(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 4)
}

// Translated from 00d87260 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = the accessor, then `index`, `RET 4`): the address of element `index`
/// of two-byte elements at the data address.
pub fn fn_00d87260(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 2)
}

// Translated from 00d87270 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = the accessor, then `index`, `RET 4`): the address of element `index`
/// of four-byte elements at the data address.
pub fn fn_00d87270(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 4)
}

// Translated from 00d87280 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = the accessor, then `index`, `RET 4`): the address of element `index`
/// of four-byte elements at the data address.
pub fn fn_00d87280(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 4)
}

// Translated from 00d87290 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = the accessor, then `index`, `RET 4`): the address of element `index`
/// of eight-byte elements at the data address.
pub fn fn_00d87290(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 8)
}

// Translated from 00d872a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = the accessor, then `index`, `RET 4`): the address of element `index`
/// of eight-byte elements at the data address.
pub fn fn_00d872a0(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 8)
}

// Translated from 00d872b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = the accessor, then `index`, `RET 4`): the address of element `index`
/// of four-byte elements at the data address.
pub fn fn_00d872b0(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 4)
}

// Translated from 00d872c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = the accessor, then `index`, `RET 4`): the address of element `index`
/// of two-byte elements at the data address.
pub fn fn_00d872c0(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 2)
}

// Translated from 00d872d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = the accessor, then `index`, `RET 4`): the address of element `index`
/// of two-byte elements at the data address.
pub fn fn_00d872d0(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 2)
}

// Translated from 00d872e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = the accessor, then `index`, `RET 4`): the address of element `index`
/// of one-byte elements at the data address (the data address plus `index`).
pub fn fn_00d872e0(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 1)
}

// Translated from 00d872f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = the accessor, then `index`, `RET 4`): the address of element `index`
/// of one-byte elements at the data address (the data address plus `index`).
pub fn fn_00d872f0(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 1)
}

// Translated from 00d87340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMemberAccessor::asSimpleArray` (Xbox PDB; `this` = the accessor, then `index`,
/// `RET 4`): the address of element `index` of eight-byte elements at the data address.
pub fn hk_class_member_accessor_as_simple_array(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 8)
}

// Translated from 00d87350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMemberAccessor::asHomogeneousArray` (Xbox PDB; `this` = the accessor, then
/// `index`, `RET 4`): the address of element `index` of twelve-byte elements at the data
/// address.
pub fn hk_class_member_accessor_as_homogeneous_array(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 12)
}

// Translated from 00d87360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMemberAccessor::asVariant` (Xbox PDB; `this` = the accessor, then `index`,
/// `RET 4`): the address of element `index` of eight-byte elements at the data address.
pub fn hk_class_member_accessor_as_variant(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    element_address(e, this, index, 8)
}

// Translated from 00d87370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMemberAccessor::getClassMember` (Xbox PDB): the member record (`+4`).
pub fn hk_class_member_accessor_get_class_member(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 4)
}

// Translated from 00d873b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`this` = an accessor, then `(first, second)`, `RET 8`): stores the two words
/// at `+0` and `+4`; answers `this`.
pub fn fn_00d873b0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.mem.set_u32(this.addr(), first);
    e.mem.set_u32(this.addr() + 4, second);
    this
}

// Translated from 00d873d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassAccessor::hkClassAccessor` (Xbox PDB; `this` = the new accessor, then `source`,
/// `RET 4`): copies the two words of `source`; answers `this`.
pub fn hk_class_accessor_construct(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let first = e.mem.u32(source.addr());
    e.mem.set_u32(this.addr(), first);
    let second = e.mem.u32(source.addr() + 4);
    e.mem.set_u32(this.addr() + 4, second);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d87130, fn_00d87130(Ptr, u32, Ptr)),
        entry!(0x00d87150, fn_00d87150(Ptr, u32, Ptr, u32)),
        entry!(0x00d87180, hk_class_member_accessor_construct(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00d871a0, hk_class_member_accessor_construct_ov2(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00d871e0, hk_class_member_accessor_is_ok(Ptr, Ptr) -> u32),
        entry!(0x00d87200, hk_class_member_accessor_get_address(Ptr) -> u32),
        entry!(0x00d87210, hk_class_member_accessor_as_pointer(Ptr, i32) -> u32),
        entry!(0x00d87220, hk_class_member_accessor_as_cstring(Ptr, i32) -> u32),
        entry!(0x00d87230, hk_class_member_accessor_as_string_ptr(Ptr, i32) -> u32),
        entry!(0x00d87250, fn_00d87250(Ptr, i32) -> u32),
        entry!(0x00d87260, fn_00d87260(Ptr, i32) -> u32),
        entry!(0x00d87270, fn_00d87270(Ptr, i32) -> u32),
        entry!(0x00d87280, fn_00d87280(Ptr, i32) -> u32),
        entry!(0x00d87290, fn_00d87290(Ptr, i32) -> u32),
        entry!(0x00d872a0, fn_00d872a0(Ptr, i32) -> u32),
        entry!(0x00d872b0, fn_00d872b0(Ptr, i32) -> u32),
        entry!(0x00d872c0, fn_00d872c0(Ptr, i32) -> u32),
        entry!(0x00d872d0, fn_00d872d0(Ptr, i32) -> u32),
        entry!(0x00d872e0, fn_00d872e0(Ptr, i32) -> u32),
        entry!(0x00d872f0, fn_00d872f0(Ptr, i32) -> u32),
        entry!(0x00d87340, hk_class_member_accessor_as_simple_array(Ptr, i32) -> u32),
        entry!(0x00d87350, hk_class_member_accessor_as_homogeneous_array(Ptr, i32) -> u32),
        entry!(0x00d87360, hk_class_member_accessor_as_variant(Ptr, i32) -> u32),
        entry!(0x00d87370, hk_class_member_accessor_get_class_member(Ptr) -> u32),
        entry!(0x00d873b0, fn_00d873b0(Ptr, u32, u32) -> Ptr),
        entry!(0x00d873d0, hk_class_accessor_construct(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A member record whose 16-bit offset (`+0x12`) is `offset`.
    fn member(e: &mut Engine, offset: u16) -> u32 {
        let m = e.mem.alloc(0x18);
        e.mem.set_u16(m + 0x12, offset);
        m
    }

    #[test]
    fn constructor_adds_the_member_offset_to_the_address() {
        let mut e = Engine::new();
        let m = member(&mut e, 0x24);
        let accessor = e.mem.alloc(8);
        assert_eq!(
            e.call(0x00d87180, &args![accessor, 0x1000u32, m]).u32(),
            accessor
        );
        assert_eq!(e.mem.u32(accessor), 0x1024);
        assert_eq!(e.mem.u32(accessor + 4), m);
        let other = e.mem.alloc(8);
        e.call(0x00d87130, &args![other, 0x2000u32, m]);
        assert_eq!(e.mem.u32(other), 0x2024);
        assert_eq!(e.mem.u32(other + 4), m);
    }

    #[test]
    fn constructor_by_name_looks_the_member_up_in_the_class() {
        let mut e = Engine::new();
        let m = member(&mut e, 0x10);
        e.register_double(CLASS_GET_MEMBER_BY_NAME, move |_, a| Ret {
            eax: if a[1] == 0x77 { m } else { 0 },
            ..Ret::default()
        });
        let accessor = e.mem.alloc(8);
        e.mem.set_u32(accessor, 0xdead);
        e.call_log = Some(Vec::new());
        e.call(0x00d87150, &args![accessor, 0x500u32, 0x6000u32, 0x77u32]);
        assert_eq!(e.mem.u32(accessor), 0x510);
        assert_eq!(e.mem.u32(accessor + 4), m);
        // An unknown name leaves a null address and a null member.
        e.call(0x00d87150, &args![accessor, 0x500u32, 0x6000u32, 0x78u32]);
        assert_eq!(e.mem.u32(accessor), 0);
        assert_eq!(e.mem.u32(accessor + 4), 0);
        let lookups: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == CLASS_GET_MEMBER_BY_NAME)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(lookups, vec![vec![0x6000, 0x77], vec![0x6000, 0x78]]);
    }

    #[test]
    fn second_constructor_unpacks_the_source_pair() {
        let mut e = Engine::new();
        let m = member(&mut e, 4);
        e.register_double(CLASS_GET_MEMBER_BY_NAME, move |_, _| Ret {
            eax: m,
            ..Ret::default()
        });
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 0x3000);
        e.mem.set_u32(source + 4, 0x7000);
        let accessor = e.mem.alloc(8);
        e.call_log = Some(Vec::new());
        assert_eq!(
            e.call(0x00d871a0, &args![accessor, source, 0x99u32]).u32(),
            accessor
        );
        assert_eq!(e.mem.u32(accessor), 0x3004);
        assert_eq!(e.mem.u32(accessor + 4), m);
        let log = e.call_log.as_ref().unwrap();
        assert!(log.contains(&(CLASS_GET_MEMBER_BY_NAME, vec![0x7000, 0x99])));
    }

    #[test]
    fn is_ok_and_getters_read_the_two_words() {
        let mut e = Engine::new();
        let accessor = e.mem.alloc(8);
        let out = e.mem.alloc(8);
        e.mem.set_u8(out, 0x55);
        assert_eq!(e.call(0x00d871e0, &args![accessor, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 0);
        e.mem.set_u32(accessor, 0x1234);
        e.mem.set_u32(accessor + 4, 0x5678);
        e.call(0x00d871e0, &args![accessor, out]);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(e.call(0x00d87200, &args![accessor]).u32(), 0x1234);
        assert_eq!(e.call(0x00d87370, &args![accessor]).u32(), 0x5678);
    }

    #[test]
    fn element_addresses_scale_the_index_by_the_element_size() {
        let mut e = Engine::new();
        let accessor = e.mem.alloc(8);
        e.mem.set_u32(accessor, 0x1000);
        let strides: [(u32, u32); 17] = [
            (0x00d87210, 4),
            (0x00d87220, 4),
            (0x00d87230, 4),
            (0x00d87250, 4),
            (0x00d87260, 2),
            (0x00d87270, 4),
            (0x00d87280, 4),
            (0x00d87290, 8),
            (0x00d872a0, 8),
            (0x00d872b0, 4),
            (0x00d872c0, 2),
            (0x00d872d0, 2),
            (0x00d872e0, 1),
            (0x00d872f0, 1),
            (0x00d87340, 8),
            (0x00d87350, 12),
            (0x00d87360, 8),
        ];
        for (address, stride) in strides {
            assert_eq!(
                e.call(address, &args![accessor, 3i32]).u32(),
                0x1000 + 3 * stride,
                "{address:08x}"
            );
            assert_eq!(
                e.call(address, &args![accessor, -1i32]).u32(),
                0x1000 - stride,
                "{address:08x}"
            );
        }
    }

    #[test]
    fn pair_setter_and_copy_constructor() {
        let mut e = Engine::new();
        let accessor = e.mem.alloc(8);
        assert_eq!(
            e.call(0x00d873b0, &args![accessor, 0xaaaau32, 0xbbbbu32])
                .u32(),
            accessor
        );
        assert_eq!(e.mem.u32(accessor), 0xaaaa);
        assert_eq!(e.mem.u32(accessor + 4), 0xbbbb);
        let copy = e.mem.alloc(8);
        assert_eq!(e.call(0x00d873d0, &args![copy, accessor]).u32(), copy);
        assert_eq!(e.mem.u32(copy), 0xaaaa);
        assert_eq!(e.mem.u32(copy + 4), 0xbbbb);
    }
}
