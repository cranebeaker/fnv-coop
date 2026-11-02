//! `NiMain/nispecularproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSpecularProperty::ms_RTTI` (the class's `NiRTTI` record).
const NI_SPECULAR_PROPERTY_RTTI: u32 = 0x011f_5ecc;

// Translated from 00a950d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpecularProperty::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_specular_property_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SPECULAR_PROPERTY_RTTI)
}

/// The allocation `00aa13e0` (cdecl), the `NiObjectNET` constructor body `00a5c200`, the member
/// copy `00a5d7a0` (`__thiscall`: clone, cloning process) and `NiProperty::GetViewerStrings`
/// (`00a5d850`).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const OBJECT_NET_CONSTRUCT: u32 = 0x00a5_c200;
const OBJECT_NET_COPY_MEMBERS: u32 = 0x00a5_d7a0;
const NI_PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00a5_d850;
/// `NiSpecularProperty`'s vtable (`010a02e4`), size and flags word (+0x18, `u16`; bit 0 is
/// `m_bSpec`).
const NI_SPECULAR_PROPERTY_VTABLE: u32 = 0x010a_02e4;
const NI_SPECULAR_PROPERTY_SIZE: u32 = 0x1c;
const FLAGS: u32 = 0x18;
/// `NiGetViewerString` (`00a7d330`), `NiGetViewerString_ov2` (`00a7d370`, a label and a
/// `bool` word), the string array's `SetSize` (`0096ad30`) and `SetAt` (`0096ae90`), and
/// `"m_bSpec"`.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
const LABEL_SPEC: u32 = 0x010a_0378;

// Translated from 00a95110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpecularProperty::CreateClone` (Xbox PDB; `__thiscall`, the cloning process): a new
/// property (0x1c bytes; `NiObjectNET`'s constructor, the vtable `010a02e4`, flags 0) or null
/// when the allocation fails, the member copy (`00a5d7a0`) into it, then the flags word copied
/// (also into the null clone). Answers the clone.
pub fn ni_specular_property_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![NI_SPECULAR_PROPERTY_SIZE]).u32();
    let clone = if block != 0 {
        e.call(OBJECT_NET_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, NI_SPECULAR_PROPERTY_VTABLE);
        e.mem.set_u16(block + FLAGS, 0);
        block
    } else {
        0
    };
    e.call(OBJECT_NET_COPY_MEMBERS, &args![this, clone, cloning]);
    let flags = e.mem.u16(this.addr() + FLAGS);
    e.mem.set_u16(clone.wrapping_add(FLAGS), flags);
    Ptr::new(clone)
}

/// The inlined `NiTArray<char *>::Add` through the stack word at `slot`.
fn append(e: &mut Engine, strings: u32, slot: u32, value: u32) {
    e.mem.set_u32(slot, value);
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let grow_by = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow_by + size]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, size, slot]);
}

// Translated from 00a951f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpecularProperty::GetViewerStrings` (Xbox PDB): `NiProperty`'s strings (`00a5d850`), the
/// class name, then `"m_bSpec"` (flag bit 0; the `bool` word is the argument's stack word, which
/// then holds the class name string, with its low byte replaced), each appended to the array.
pub fn ni_specular_property_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(NI_PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    let strings = strings.addr();
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.mem.u32(NI_SPECULAR_PROPERTY_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
        append(e, strings, slot, text);
        let spec = (e.mem.u32(slot) & 0xffff_ff00) | u32::from(e.mem.u8(this.addr() + FLAGS) & 1);
        let text = e
            .call(NI_GET_VIEWER_STRING_BOOL, &args![LABEL_SPEC, spec])
            .u32();
        append(e, strings, slot, text);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a950d0, ni_specular_property_get_rtti() -> Ptr),
        entry!(0x00a95110, ni_specular_property_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a951f0,
            ni_specular_property_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_specular_property_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a950d0, &args![]).u32(),
            NI_SPECULAR_PROPERTY_RTTI
        );
    }
}

#[cfg(test)]
mod n019_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn create_clone_copies_the_flags() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| word(e.mem.alloc(a[0])));
        e.register(OBJECT_NET_CONSTRUCT, |_, _| Ret::default());
        e.register(OBJECT_NET_COPY_MEMBERS, |_, _| Ret::default());
        let property = e.mem.alloc(NI_SPECULAR_PROPERTY_SIZE);
        e.mem.set_u16(property + FLAGS, 0x8001);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a9_5110, &args![property, 0x44u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_SPECULAR_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(clone + FLAGS), 0x8001);
        assert_eq!(
            calls_to(&e, OBJECT_NET_COPY_MEMBERS),
            vec![vec![property, clone, 0x44]]
        );
        assert_eq!(calls_to(&e, OBJECT_NET_CONSTRUCT), vec![vec![clone]]);
    }

    #[test]
    fn get_viewer_strings_adds_the_spec_flag() {
        let mut e = Engine::new();
        e.map(NI_SPECULAR_PROPERTY_RTTI, 4);
        e.mem.set_u32(NI_SPECULAR_PROPERTY_RTTI, 0x010a_0400);
        e.register(NI_PROPERTY_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x2468_ace0));
        e.register(NI_GET_VIEWER_STRING_BOOL, |_, a| word(a[1]));
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(e.mem.u32(a[0] + 4) + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(strings + 4, array);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 2);
        let property = e.mem.alloc(NI_SPECULAR_PROPERTY_SIZE);
        e.mem.set_u16(property + FLAGS, 3);
        e.call_log = Some(vec![]);
        e.call(0x00a9_51f0, &args![property, strings]);
        assert_eq!(e.mem.u32(array), 0x2468_ace0);
        assert_eq!(e.mem.u32(array + 4), 0x2468_ac01);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![strings, 3]]);
    }
}
