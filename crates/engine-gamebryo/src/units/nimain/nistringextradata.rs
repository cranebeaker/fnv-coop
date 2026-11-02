//! `NiMain/nistringextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiStringExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_STRING_EXTRA_DATA_RTTI: u32 = 0x011f_4a38;

// Translated from 00a74220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStringExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_string_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_STRING_EXTRA_DATA_RTTI)
}

use crate::units::nimain::nimainsdm::{
    acquire_fixed_string, release_fixed_string, ALLOCATE, EMPTY_FIXED_STRING,
};

/// The `NiStringExtraData` vtable (RTTI `.?AVNiStringExtraData@@`).
const STRING_EXTRA_DATA_VTABLE: u32 = 0x0109_d39c;
/// The `NiExtraData` functions this unit runs (`niextradata.cpp`): the default
/// constructor (`00a7b2e0`), the destructor body (`00a7b300`), the member copy
/// (`00a7b330`, `RET 8`) and `NiExtraData::LoadBinary` (`00a7b4a0`, `RET 4`).
const EXTRA_DATA_CONSTRUCTOR: u32 = 0x00a7_b2e0;
const EXTRA_DATA_DESTRUCTOR: u32 = 0x00a7_b300;
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
const EXTRA_DATA_LOAD_BINARY: u32 = 0x00a7_b4a0;
/// `NiStream::LoadCStringAsFixedString` (`00a64ea0`) and `NiStream::LoadFixedString`
/// (`00a64df0`), both `thiscall` on the stream with the address of the handle.
const LOAD_C_STRING_AS_FIXED_STRING: u32 = 0x00a6_4ea0;
const LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const SIZED_DELETE: u32 = 0x00aa_1460;
/// The string of an `NiStringExtraData` (`m_kString`, a fixed-string handle) at `+0x0c`.
const STRING: u32 = 0xc;

// Translated from 00a74230 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in Ghidra's project (read from the bytes; the class factory of
/// `NiStringExtraData`, `cdecl`, no argument): allocates 0x10 bytes (`00aa13e0`); a null
/// block answers null, otherwise the `NiExtraData` constructor (`00a7b2e0`) runs on it,
/// the vtable `0109d39c` is set and the string is the empty string's handle (`0109b220`,
/// not counted). Answers the object.
pub fn fn_00a74230(e: &mut Engine) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(EXTRA_DATA_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, STRING_EXTRA_DATA_VTABLE);
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.mem.set_u32(block + STRING, empty);
    Ptr::new(block)
}

// Translated from 00a74260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStringExtraData::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`):
/// `NiExtraData::LoadBinary` (`00a7b4a0`), then the string at `+0x0c`: as a C string
/// (`00a64ea0`) for a stream older than `0x14010001` (its version at `+0xd8`), as a fixed
/// string (`00a64df0`) otherwise.
pub fn ni_string_extra_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_LOAD_BINARY, &args![this, stream]);
    let string = this.addr().wrapping_add(STRING);
    if e.mem.u32(stream.addr() + 0xd8) < 0x1401_0001 {
        e.call(LOAD_C_STRING_AS_FIXED_STRING, &args![stream, string]);
    } else {
        e.call(LOAD_FIXED_STRING, &args![stream, string]);
    }
}

// Translated from 00a742c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStringExtraData::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, `RET 4`):
/// sets the vtable `0109d39c`, releases the string (unless it is the empty string's),
/// runs the `NiExtraData` destructor body (`00a7b300`) and, when bit 0 of `flags` is set,
/// the sized delete of 0x10 bytes (`00aa1460`). Answers `this`.
pub fn ni_string_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), STRING_EXTRA_DATA_VTABLE);
    let string = e.mem.u32(this.addr() + STRING);
    release_fixed_string(e, string);
    e.call(EXTRA_DATA_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00a74300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStringExtraData::CreateClone` (Xbox PDB; `thiscall`, `RET 4`): a new object as
/// `00a74230` makes it (null when the allocation fails), the `NiExtraData` member copy
/// (`00a7b330`) from `this` to it with `cloning`, then the string of `this` given to it
/// (counted, its old one released) when the two differ. The clone is read even when the
/// allocation failed, as the exe does. Answers the clone.
pub fn ni_string_extra_data_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let clone = fn_00a74230(e).addr();
    e.call(EXTRA_DATA_COPY_MEMBERS, &args![this, clone, cloning]);
    let string = e.mem.u32(this.addr() + STRING);
    let held = clone.wrapping_add(STRING);
    if e.mem.u32(held) != string {
        acquire_fixed_string(e, string);
        let old = e.mem.u32(held);
        release_fixed_string(e, old);
        e.mem.set_u32(held, string);
    }
    Ptr::new(clone)
}

/// `NiExtraData::RegisterStreamables` (`00a7b290`), `NiExtraData::SaveBinary` (`00a7b2c0`)
/// and `NiExtraData::GetViewerStrings` (`00a7b580`), `__thiscall` with one word.
const EXTRA_DATA_REGISTER_STREAMABLES: u32 = 0x00a7_b290;
const EXTRA_DATA_SAVE_BINARY: u32 = 0x00a7_b2c0;
const EXTRA_DATA_GET_VIEWER_STRINGS: u32 = 0x00a7_b580;
/// `NiStream`'s virtual +0x20 (one word: the address of a fixed string it registers) and
/// `NiStream::SaveFixedString` (`00a641f0`, `__thiscall`, the address of the handle).
const STREAM_REGISTER_FIXED_STRING_SLOT: u32 = 0x20;
const SAVE_FIXED_STRING: u32 = 0x00a6_41f0;
/// `NiGetViewerString` (`00a7d330`, cdecl: a name) and `NiGetViewerString_ov11` (`00a7d5e0`,
/// cdecl: a label and a fixed string); the string array's `SetSize` (`0096ad30`,
/// `__thiscall`: the new size) and `SetAt` (`0096ae90`, `__thiscall`: index, address of the
/// element); the label `"m_kString"`.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_FIXED: u32 = 0x00a7_d5e0;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
const LABEL_STRING: u32 = 0x0109_d430;

// Translated from 00a741d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStringExtraData::RegisterStreamables` (Xbox PDB): false when
/// `NiExtraData::RegisterStreamables` (`00a7b290`) is; otherwise the string (+0x0c) is
/// registered with the stream (its virtual +0x20) and the answer is true.
pub fn ni_string_extra_data_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if !e
        .call(EXTRA_DATA_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    e.vcall(
        stream.addr(),
        STREAM_REGISTER_FIXED_STRING_SLOT,
        &args![this.addr() + STRING],
    );
    true
}

// Translated from 00a74200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStringExtraData::SaveBinary` (Xbox PDB): `NiExtraData::SaveBinary` (`00a7b2c0`), then
/// the string (+0x0c) through `NiStream::SaveFixedString` (`00a641f0`).
pub fn ni_string_extra_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_SAVE_BINARY, &args![this, stream]);
    e.call(SAVE_FIXED_STRING, &args![stream, this.addr() + STRING]);
}

// Translated from 00a742a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `NiStringExtraData` vtable's slot +0x5c (`0109d3f8`, the
/// `IsEqual` slot of `NiObject`'s table) holds it: false for a null `other`, otherwise
/// whether both strings (+0x0c) are the same handle. The base class is not asked.
pub fn fn_00a742a0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if other.is_null() {
        return false;
    }
    e.mem.u32(this.addr() + STRING) == e.mem.u32(other.addr() + STRING)
}

/// The inlined `NiTArray<char *>::Add`: grows the array to its size plus `m_usGrowBy` (+0xe)
/// when the size (+0xa) has reached the capacity (+0x8), then stores `value` at the old size
/// through a stack word.
fn add_viewer_string(e: &mut Engine, strings: u32, value: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    let capacity = u32::from(e.mem.u16(strings + 0x8));
    if size >= capacity {
        let grow_by = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow_by + size]);
    }
    e.with_stack(4, |e, element| {
        e.mem.set_u32(element.addr(), value);
        e.call(STRING_ARRAY_SET_AT, &args![strings, size, element]);
    });
}

// Translated from 00a74380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStringExtraData::GetViewerStrings` (Xbox PDB): `NiExtraData`'s strings (`00a7b580`),
/// then the class name (`NiGetViewerString` of `ms_RTTI`'s first word) and `"m_kString"`
/// with the string (`NiGetViewerString_ov11`), each appended to the array.
pub fn ni_string_extra_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let strings = strings.addr();
    e.call(EXTRA_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(NI_STRING_EXTRA_DATA_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    add_viewer_string(e, strings, text);
    let string = e.mem.u32(this.addr() + STRING);
    let text = e
        .call(NI_GET_VIEWER_STRING_FIXED, &args![LABEL_STRING, string])
        .u32();
    add_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a74220, ni_string_extra_data_get_rtti() -> Ptr),
        entry!(0x00a7_4230, fn_00a74230() -> Ptr),
        entry!(0x00a7_4260, ni_string_extra_data_load_binary(Ptr, Ptr)),
        entry!(
            0x00a7_42c0,
            ni_string_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a7_4300, ni_string_extra_data_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00a7_41d0, ni_string_extra_data_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a7_4200, ni_string_extra_data_save_binary(Ptr, Ptr)),
        entry!(0x00a7_42a0, fn_00a742a0(Ptr, Ptr) -> bool),
        entry!(
            0x00a7_4380,
            ni_string_extra_data_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_string_extra_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a74220, &args![]).u32(),
            NI_STRING_EXTRA_DATA_RTTI
        );
    }
}

#[cfg(test)]
mod string_extra_data_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    const EMPTY: u32 = 0x0109_b228;

    fn handle(e: &mut Engine, count: u32) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, count);
        block + 8
    }

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.register(EXTRA_DATA_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(EXTRA_DATA_DESTRUCTOR, |_, _| Ret::default());
        e.register(EXTRA_DATA_COPY_MEMBERS, |_, _| Ret::default());
        e.register(EXTRA_DATA_LOAD_BINARY, |_, _| Ret::default());
        e.register(LOAD_C_STRING_AS_FIXED_STRING, |_, _| Ret::default());
        e.register(LOAD_FIXED_STRING, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn the_factory_builds_an_empty_string_extra_data() {
        let mut e = engine();
        let object = e.call(0x00a7_4230, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, EXTRA_DATA_CONSTRUCTOR), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), STRING_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(object + STRING), EMPTY);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a7_4230, &args![]).u32(), 0);
    }

    #[test]
    fn loading_reads_the_string_by_stream_version() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 0xd8, 0x1401_0000);
        e.call(0x00a7_4260, &args![this, stream]);
        e.mem.set_u32(stream + 0xd8, 0x1401_0001);
        e.call(0x00a7_4260, &args![this, stream]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_LOAD_BINARY),
            vec![vec![this, stream]; 2]
        );
        assert_eq!(
            calls_to(&e, LOAD_C_STRING_AS_FIXED_STRING),
            vec![vec![stream, this + 0xc]]
        );
        assert_eq!(
            calls_to(&e, LOAD_FIXED_STRING),
            vec![vec![stream, this + 0xc]]
        );
    }

    #[test]
    fn the_deleting_destructor_releases_the_string_and_deletes_on_bit_0() {
        let mut e = engine();
        let string = handle(&mut e, 2);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + STRING, string);
        assert_eq!(e.call(0x00a7_42c0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(string - 8), 1);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        e.mem.set_u32(this + STRING, EMPTY);
        e.call(0x00a7_42c0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, EXTRA_DATA_DESTRUCTOR), vec![vec![this]; 2]);
        assert_eq!(e.mem.u32(this), STRING_EXTRA_DATA_VTABLE);
    }

    #[test]
    fn a_clone_copies_the_members_and_counts_the_string() {
        let mut e = engine();
        let string = handle(&mut e, 1);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + STRING, string);
        let clone = e.call(0x00a7_4300, &args![this, 0x55u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), STRING_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(clone + STRING), string);
        assert_eq!(e.mem.u32(string - 8), 2);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_COPY_MEMBERS),
            vec![vec![this, clone, 0x55]]
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

    const STREAM_DOUBLE: u32 = 0x0070_b000;

    #[test]
    fn register_streamables_registers_the_string() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_REGISTER_STREAMABLES, |_, a| {
            word(u32::from(a[0] != 0x40))
        });
        e.register(STREAM_DOUBLE, |_, _| Ret::default());
        let stream = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(stream, vtable);
        e.mem.set_u32(vtable + 0x20, STREAM_DOUBLE);
        let data = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a7_41d0, &args![data, stream]).bool());
        assert_eq!(calls_to(&e, STREAM_DOUBLE), vec![vec![stream, data + 0xc]]);
        assert!(!e.call(0x00a7_41d0, &args![0x40u32, stream]).bool());
        assert_eq!(calls_to(&e, STREAM_DOUBLE).len(), 1);
    }

    #[test]
    fn save_binary_saves_the_base_then_the_string() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_SAVE_BINARY, |_, _| Ret::default());
        e.register(SAVE_FIXED_STRING, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a7_4200, &args![0x100u32, 0x200u32]);
        let log = e.call_log.take().unwrap();
        let order: Vec<_> = log.iter().skip(1).cloned().collect();
        assert_eq!(
            order,
            vec![
                (EXTRA_DATA_SAVE_BINARY, vec![0x100, 0x200]),
                (SAVE_FIXED_STRING, vec![0x200, 0x10c]),
            ]
        );
    }

    #[test]
    fn fn_00a742a0_compares_the_string_handles() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        e.mem.set_u32(a + 0xc, 0x1234);
        assert!(!e.call(0x00a7_42a0, &args![a, 0u32]).bool());
        assert!(!e.call(0x00a7_42a0, &args![a, b]).bool());
        e.mem.set_u32(b + 0xc, 0x1234);
        assert!(e.call(0x00a7_42a0, &args![a, b]).bool());
    }

    #[test]
    fn get_viewer_strings_appends_the_class_and_string() {
        let mut e = Engine::new();
        e.map(NI_STRING_EXTRA_DATA_RTTI, 8);
        e.mem.set_u32(NI_STRING_EXTRA_DATA_RTTI, 0x0109_d400);
        e.register(EXTRA_DATA_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, a| word(a[0] + 1));
        e.register(NI_GET_VIEWER_STRING_FIXED, |_, a| word(a[1] + 2));
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x10 + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x20);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 1);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data + 0xc, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x00a7_4380, &args![data, strings]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_GET_VIEWER_STRINGS),
            vec![vec![data, strings]]
        );
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FIXED),
            vec![vec![LABEL_STRING, 0x5000]]
        );
        assert_eq!(e.mem.u32(strings + 0x10), 0x0109_d401);
        assert_eq!(e.mem.u32(strings + 0x14), 0x5002);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![strings, 2]]);
    }
}
