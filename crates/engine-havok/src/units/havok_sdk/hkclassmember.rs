//! `Havok SDK/hkclassmember.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The exe's table of member-type properties (12-byte records indexed by the member type):
/// the record's size at `+8` and alignment at `+0xA` are 16-bit signed.
const TYPE_PROPERTIES: u32 = 0x011b_6ab0;
/// `hkCustomAttributes::getAttribute` (`00d46cf0`, `this` = the attribute list, `(name)`).
const CUSTOM_ATTRIBUTES_GET_ATTRIBUTE: u32 = 0x00d4_6cf0;
/// `hkClass::getObjectSize` (`00d470d0`, `this` = the class record).
const CLASS_GET_OBJECT_SIZE: u32 = 0x00d4_70d0;
/// `hkClass::getNumMembers` (`00d46f00`, `this` = the class record).
const CLASS_GET_NUM_MEMBERS: u32 = 0x00d4_6f00;
/// `hkClass::getMember` (`00d46f20`, `this` = the class record, `(index)`).
const CLASS_GET_MEMBER: u32 = 0x00d4_6f20;

/// The signed 16-bit field at `offset` of the type-properties record of `kind`.
fn type_property(e: &Engine, kind: u32, offset: u32) -> i32 {
    i32::from(e.mem.i16(TYPE_PROPERTIES + kind.wrapping_mul(12) + offset))
}

// Translated from 00d868a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMember::getClassMemberTypeProperties` (Xbox PDB; cdecl `(type)`): the address of the
/// type's twelve-byte record in the exe's table (`011b6ab0 + 12 * type`).
pub fn hk_class_member_get_class_member_type_properties(_e: &mut Engine, kind: u32) -> u32 {
    TYPE_PROPERTIES.wrapping_add(kind.wrapping_mul(12))
}

// Translated from 00d86910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMember::getStructClass` (Xbox PDB): the member's class record (`+4`).
pub fn hk_class_member_get_struct_class(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 4)
}

// Translated from 00d86920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMember::getClass` (Xbox PDB): the member's class record (`+4`).
pub fn hk_class_member_get_class(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 4)
}

// Translated from 00d86930 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the member's enum record (`+8`).
pub fn fn_00d86930(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 8)
}

// Translated from 00d86940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMember::getCstyleArraySize` (Xbox PDB): the member's signed 16-bit c-array size
/// (`+0xE`), sign-extended.
pub fn hk_class_member_get_cstyle_array_size(e: &mut Engine, this: Ptr) -> i32 {
    i32::from(e.mem.i16(this.addr() + 0xe))
}

// Translated from 00d86960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMember::getAttribute` (Xbox PDB; `this` = a member record, then `name`, `RET 4`):
/// the member's attribute list (`+0x14`) searched by `hkCustomAttributes::getAttribute`
/// (`00d46cf0`); 0 when the member has none.
pub fn hk_class_member_get_attribute(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let attributes = e.mem.u32(this.addr() + 0x14);
    if attributes == 0 {
        return 0;
    }
    e.call(CUSTOM_ATTRIBUTES_GET_ATTRIBUTE, &args![attributes, name])
        .u32()
}

// Translated from 00d86980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMember::getSizeInBytes` (Xbox PDB; `this` = a member record): by the member's type
/// byte (`+0xC`), as the exe's switch (its byte table sends types 1 to 0x12, 0x14 to 0x16,
/// 0x1A to 0x1E, 0x20 and 0x21 to the plain case, 0x18 and 0x1F to the array case, 0x19 to the
/// struct case and everything else to -1):
/// - plain: the type's size in the properties table times the c-array size (1 when it is 0);
/// - array (0x18, 0x1F): the size of the subtype (`+0xD`) times the c-array size (1 when 0);
/// - struct (0x19): the member class's object size (`hkClass::getObjectSize`, `00d470d0`)
///   times the c-array size when that is not 0;
/// - otherwise -1.
pub fn hk_class_member_get_size_in_bytes(e: &mut Engine, this: Ptr) -> i32 {
    let t = this.addr();
    let kind = u32::from(e.mem.u8(t + 0xc));
    match kind {
        1..=0x12 | 0x14..=0x16 | 0x1a..=0x1e | 0x20 | 0x21 => {
            if hk_class_member_get_cstyle_array_size(e, this) == 0 {
                type_property(e, kind, 8)
            } else {
                let count = hk_class_member_get_cstyle_array_size(e, this);
                type_property(e, kind, 8).wrapping_mul(count)
            }
        }
        0x18 | 0x1f => {
            let count = if hk_class_member_get_cstyle_array_size(e, this) == 0 {
                1
            } else {
                hk_class_member_get_cstyle_array_size(e, this)
            };
            let subtype = u32::from(e.mem.u8(t + 0xd));
            type_property(e, subtype, 8).wrapping_mul(count)
        }
        0x19 => {
            if hk_class_member_get_cstyle_array_size(e, this) == 0 {
                let class = hk_class_member_get_struct_class(e, this);
                e.call(CLASS_GET_OBJECT_SIZE, &args![class]).i32()
            } else {
                let count = hk_class_member_get_cstyle_array_size(e, this);
                let class = hk_class_member_get_struct_class(e, this);
                let size = e.call(CLASS_GET_OBJECT_SIZE, &args![class]).i32();
                size.wrapping_mul(count)
            }
        }
        _ => -1,
    }
}

// Translated from 00d86a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMember::getAlignment` (Xbox PDB; `this` = a member record): the alignment of the
/// member's type (the subtype for types 0x18 and 0x1F). For a struct (0x19) it starts from 1
/// and takes the largest alignment among the member class's members (`hkClass::getNumMembers`
/// `00d46f00`, `hkClass::getMember` `00d46f20`, each member through this function); for other
/// types it is the properties table's signed 16-bit alignment (`+0xA`). When the member's
/// flags word (`+0x10`) has bit 0x80 or 0x100, the alignment is raised to 8 (bit 0x100 clear)
/// or 16 (set) if that is more.
pub fn hk_class_member_get_alignment(e: &mut Engine, this: Ptr) -> i32 {
    let t = this.addr();
    let mut kind = u32::from(e.mem.u8(t + 0xc));
    if kind == 0x18 || kind == 0x1f {
        kind = u32::from(e.mem.u8(t + 0xd));
    }
    let mut alignment = if kind == 0x19 {
        let mut largest = (kind - 0x18) as i32;
        let mut index = 0i32;
        let class = e.mem.u32(t + 4);
        if e.call(CLASS_GET_NUM_MEMBERS, &args![class]).i32() > 0 {
            loop {
                let class = e.mem.u32(t + 4);
                let member = e.call(CLASS_GET_MEMBER, &args![class, index]).u32();
                if hk_class_member_get_alignment(e, Ptr::new(member)) > largest {
                    largest = hk_class_member_get_alignment(e, Ptr::new(member));
                }
                let class = e.mem.u32(t + 4);
                index = index.wrapping_add(1);
                if index >= e.call(CLASS_GET_NUM_MEMBERS, &args![class]).i32() {
                    break;
                }
            }
        }
        largest
    } else {
        type_property(e, kind, 0xa)
    };
    let flags = u32::from(e.mem.u16(t + 0x10));
    if flags & 0x180 != 0 {
        let raised = i32::from((flags >> 8) & 1 != 0) * 8 + 8;
        if raised > alignment {
            alignment = raised;
        }
    }
    alignment
}

// Translated from 00d86b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the member's subtype byte (`+0xD`).
pub fn fn_00d86b30(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u8(this.addr() + 0xd))
}

// Translated from 00d86b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMember::getArrayMemberSize` (Xbox PDB; `this` = a member record): -1 when the
/// subtype (`+0xD`) is 0x18 or 0x1F; the member class's object size
/// (`hkClass::getObjectSize`, `00d470d0`) when it is 0x19; otherwise the subtype's size in the
/// properties table.
pub fn hk_class_member_get_array_member_size(e: &mut Engine, this: Ptr) -> i32 {
    if fn_00d86b30(e, this) == 0x18 {
        return -1;
    }
    if fn_00d86b30(e, this) == 0x1f {
        return -1;
    }
    if fn_00d86b30(e, this) == 0x19 {
        let class = hk_class_member_get_struct_class(e, this);
        return e.call(CLASS_GET_OBJECT_SIZE, &args![class]).i32();
    }
    let subtype = fn_00d86b30(e, this);
    type_property(e, subtype, 8)
}

// Translated from 00d86b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClassMember::getEnumValue` (Xbox PDB; `this` = a member record, then `value`, `RET 4`):
/// the integer at the address `value`, read with the member's size in bytes
/// ([`hk_class_member_get_size_in_bytes`]): a signed byte for 1, a signed 16-bit word for 2,
/// a 32-bit word for 4, and 0 for any other size.
pub fn hk_class_member_get_enum_value(e: &mut Engine, this: Ptr, value: Ptr) -> i32 {
    match hk_class_member_get_size_in_bytes(e, this) {
        1 => i32::from(e.mem.i8(value.addr())),
        2 => i32::from(e.mem.i16(value.addr())),
        4 => e.mem.i32(value.addr()),
        _ => 0,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d868a0, hk_class_member_get_class_member_type_properties(u32) -> u32),
        entry!(0x00d86910, hk_class_member_get_struct_class(Ptr) -> u32),
        entry!(0x00d86920, hk_class_member_get_class(Ptr) -> u32),
        entry!(0x00d86930, fn_00d86930(Ptr) -> u32),
        entry!(0x00d86940, hk_class_member_get_cstyle_array_size(Ptr) -> i32),
        entry!(0x00d86960, hk_class_member_get_attribute(Ptr, u32) -> u32),
        entry!(0x00d86980, hk_class_member_get_size_in_bytes(Ptr) -> i32),
        entry!(0x00d86a90, hk_class_member_get_alignment(Ptr) -> i32),
        entry!(0x00d86b30, fn_00d86b30(Ptr) -> u32),
        entry!(0x00d86b40, hk_class_member_get_array_member_size(Ptr) -> i32),
        entry!(0x00d86b80, hk_class_member_get_enum_value(Ptr, Ptr) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine with the properties table mapped: the size and alignment of type `n` are
    /// `10 * n` and `n` for the types the tests use, and the object-size, member-count and
    /// member-lookup callees of the class unit read the class records the tests build.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011b_6000, 0x1000);
        for kind in 0..0x22u32 {
            e.mem
                .set_i16(TYPE_PROPERTIES + kind * 12 + 8, (kind * 10) as i16);
            e.mem
                .set_i16(TYPE_PROPERTIES + kind * 12 + 0xa, kind as i16);
        }
        e.register(CLASS_GET_OBJECT_SIZE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        e.register(CLASS_GET_NUM_MEMBERS, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x1c),
            ..Ret::default()
        });
        e.register(CLASS_GET_MEMBER, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x18) + a[1] * 0x18,
            ..Ret::default()
        });
        e
    }

    /// A member record with a type, a subtype, a c-array size and a flags word.
    fn member(e: &mut Engine, kind: u8, subtype: u8, array_size: i16, flags: u16) -> u32 {
        let m = e.mem.alloc(0x18);
        e.mem.set_u8(m + 0xc, kind);
        e.mem.set_u8(m + 0xd, subtype);
        e.mem.set_i16(m + 0xe, array_size);
        e.mem.set_u16(m + 0x10, flags);
        m
    }

    #[test]
    fn simple_accessors_read_their_fields() {
        let mut e = engine();
        let m = member(&mut e, 4, 9, -3, 0);
        e.mem.set_u32(m + 4, 0x1111);
        e.mem.set_u32(m + 8, 0x2222);
        assert_eq!(e.call(0x00d868a0, &args![5u32]).u32(), TYPE_PROPERTIES + 60);
        assert_eq!(e.call(0x00d86910, &args![m]).u32(), 0x1111);
        assert_eq!(e.call(0x00d86920, &args![m]).u32(), 0x1111);
        assert_eq!(e.call(0x00d86930, &args![m]).u32(), 0x2222);
        assert_eq!(e.call(0x00d86940, &args![m]).i32(), -3);
        assert_eq!(e.call(0x00d86b30, &args![m]).u32(), 9);
    }

    #[test]
    fn attributes_are_searched_through_the_custom_attribute_list() {
        let mut e = engine();
        let m = member(&mut e, 4, 0, 0, 0);
        e.register_double(CUSTOM_ATTRIBUTES_GET_ATTRIBUTE, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        e.call_log = Some(Vec::new());
        assert_eq!(e.call(0x00d86960, &args![m, 0x40u32]).u32(), 0);
        e.mem.set_u32(m + 0x14, 0x5000);
        assert_eq!(e.call(0x00d86960, &args![m, 0x40u32]).u32(), 0x41);
        let log = e.call_log.as_ref().unwrap();
        let searches: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == CUSTOM_ATTRIBUTES_GET_ATTRIBUTE)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(searches, vec![&vec![0x5000, 0x40]]);
    }

    #[test]
    fn size_in_bytes_of_plain_members_multiplies_by_the_c_array_size() {
        let mut e = engine();
        let single = member(&mut e, 4, 0, 0, 0);
        let array = member(&mut e, 4, 0, 5, 0);
        let negative = member(&mut e, 0x21, 0, -2, 0);
        assert_eq!(e.call(0x00d86980, &args![single]).i32(), 40);
        assert_eq!(e.call(0x00d86980, &args![array]).i32(), 200);
        assert_eq!(e.call(0x00d86980, &args![negative]).i32(), -660);
    }

    #[test]
    fn size_in_bytes_of_array_members_uses_the_subtype() {
        let mut e = engine();
        let single = member(&mut e, 0x18, 3, 0, 0);
        let array = member(&mut e, 0x1f, 3, 4, 0);
        assert_eq!(e.call(0x00d86980, &args![single]).i32(), 30);
        assert_eq!(e.call(0x00d86980, &args![array]).i32(), 120);
    }

    #[test]
    fn size_in_bytes_of_struct_members_uses_the_class_object_size() {
        let mut e = engine();
        let class = e.mem.alloc(0x30);
        e.mem.set_u32(class + 8, 0x24);
        let single = member(&mut e, 0x19, 0, 0, 0);
        let array = member(&mut e, 0x19, 0, 3, 0);
        e.mem.set_u32(single + 4, class);
        e.mem.set_u32(array + 4, class);
        assert_eq!(e.call(0x00d86980, &args![single]).i32(), 0x24);
        assert_eq!(e.call(0x00d86980, &args![array]).i32(), 0x6c);
    }

    #[test]
    fn size_in_bytes_is_minus_one_for_types_outside_the_switch() {
        let mut e = engine();
        for kind in [0u8, 0x13, 0x17, 0x22, 0xff] {
            let m = member(&mut e, kind, 0, 0, 0);
            assert_eq!(e.call(0x00d86980, &args![m]).i32(), -1, "type {kind:#x}");
        }
        for kind in [1u8, 0x12, 0x14, 0x16, 0x1a, 0x1e, 0x20, 0x21] {
            let m = member(&mut e, kind, 0, 0, 0);
            assert_eq!(
                e.call(0x00d86980, &args![m]).i32(),
                i32::from(kind) * 10,
                "type {kind:#x}"
            );
        }
    }

    #[test]
    fn alignment_is_the_type_alignment_raised_by_the_flags() {
        let mut e = engine();
        let plain = member(&mut e, 4, 0, 0, 0);
        let eight = member(&mut e, 4, 0, 0, 0x80);
        let sixteen = member(&mut e, 4, 0, 0, 0x100);
        let sixteen_both = member(&mut e, 4, 0, 0, 0x180);
        let big = member(&mut e, 0x20, 0, 0, 0x100);
        let array = member(&mut e, 0x18, 2, 3, 0x80);
        assert_eq!(e.call(0x00d86a90, &args![plain]).i32(), 4);
        assert_eq!(e.call(0x00d86a90, &args![eight]).i32(), 8);
        assert_eq!(e.call(0x00d86a90, &args![sixteen]).i32(), 16);
        assert_eq!(e.call(0x00d86a90, &args![sixteen_both]).i32(), 16);
        // The type's own alignment (0x20) stays when it is larger.
        assert_eq!(e.call(0x00d86a90, &args![big]).i32(), 0x20);
        // An array member takes the subtype's alignment.
        assert_eq!(e.call(0x00d86a90, &args![array]).i32(), 8);
    }

    #[test]
    fn alignment_of_a_struct_is_the_largest_member_alignment() {
        let mut e = engine();
        let class = e.mem.alloc(0x30);
        let members = e.mem.alloc(0x18 * 3);
        for (i, kind) in [3u8, 9, 5].into_iter().enumerate() {
            let at = members + 0x18 * i as u32;
            e.mem.set_u8(at + 0xc, kind);
        }
        e.mem.set_u32(class + 0x18, members);
        e.mem.set_u32(class + 0x1c, 3);
        let m = member(&mut e, 0x19, 0, 0, 0);
        e.mem.set_u32(m + 4, class);
        assert_eq!(e.call(0x00d86a90, &args![m]).i32(), 9);
        // Without members the start value 1 is the answer.
        e.mem.set_u32(class + 0x1c, 0);
        assert_eq!(e.call(0x00d86a90, &args![m]).i32(), 1);
        // The flags still raise it.
        e.mem.set_u16(m + 0x10, 0x100);
        assert_eq!(e.call(0x00d86a90, &args![m]).i32(), 16);
        // An array of structs goes through the subtype.
        let array = member(&mut e, 0x1f, 0x19, 2, 0);
        e.mem.set_u32(array + 4, class);
        e.mem.set_u32(class + 0x1c, 3);
        assert_eq!(e.call(0x00d86a90, &args![array]).i32(), 9);
    }

    #[test]
    fn array_member_size_depends_on_the_subtype() {
        let mut e = engine();
        let class = e.mem.alloc(0x30);
        e.mem.set_u32(class + 8, 0x44);
        let plain = member(&mut e, 0x18, 5, 0, 0);
        let nested = member(&mut e, 0x18, 0x18, 0, 0);
        let nested_other = member(&mut e, 0x18, 0x1f, 0, 0);
        let structs = member(&mut e, 0x18, 0x19, 0, 0);
        e.mem.set_u32(structs + 4, class);
        assert_eq!(e.call(0x00d86b40, &args![plain]).i32(), 50);
        assert_eq!(e.call(0x00d86b40, &args![nested]).i32(), -1);
        assert_eq!(e.call(0x00d86b40, &args![nested_other]).i32(), -1);
        assert_eq!(e.call(0x00d86b40, &args![structs]).i32(), 0x44);
    }

    #[test]
    fn enum_value_reads_the_member_sized_integer() {
        let mut e = engine();
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0xffff_fff0);
        // Type 1 has size 10 in this table; set the sizes the cases need.
        e.mem.set_i16(TYPE_PROPERTIES + 12 + 8, 1);
        e.mem.set_i16(TYPE_PROPERTIES + 24 + 8, 2);
        e.mem.set_i16(TYPE_PROPERTIES + 36 + 8, 4);
        e.mem.set_i16(TYPE_PROPERTIES + 48 + 8, 8);
        let byte = member(&mut e, 1, 0, 0, 0);
        let word = member(&mut e, 2, 0, 0, 0);
        let dword = member(&mut e, 3, 0, 0, 0);
        let wide = member(&mut e, 4, 0, 0, 0);
        let unknown = member(&mut e, 0x13, 0, 0, 0);
        assert_eq!(e.call(0x00d86b80, &args![byte, data]).i32(), -16);
        assert_eq!(e.call(0x00d86b80, &args![word, data]).i32(), -16);
        assert_eq!(e.call(0x00d86b80, &args![dword, data]).i32(), -16);
        assert_eq!(e.call(0x00d86b80, &args![wide, data]).i32(), 0);
        assert_eq!(e.call(0x00d86b80, &args![unknown, data]).i32(), 0);
        // Sign extension: a positive byte and a positive word.
        e.mem.set_u32(data, 0x0001_7f7f);
        assert_eq!(e.call(0x00d86b80, &args![byte, data]).i32(), 0x7f);
        assert_eq!(e.call(0x00d86b80, &args![word, data]).i32(), 0x7f7f);
        assert_eq!(e.call(0x00d86b80, &args![dword, data]).i32(), 0x1_7f7f);
    }
}
