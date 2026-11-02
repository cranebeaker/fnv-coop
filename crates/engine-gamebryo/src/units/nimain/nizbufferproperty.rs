//! `NiMain/nizbufferproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{ALLOCATE, OBJECT_BASE_CONSTRUCTOR};

/// `NiZBufferProperty`'s vtable (its RTTI locator names `.?AVNiZBufferProperty@@`).
const NI_ZBUFFER_PROPERTY_VTABLE: u32 = 0x0109_b7fc;
/// `NiProperty`'s `IsEqual` (`00a62140`, `thiscall`, the other object; the engine map names
/// it `NiSequenceStreamHelper::IsEqual`, folded code).
const PROPERTY_IS_EQUAL: u32 = 0x00a6_2140;
/// `00a5d450` (`thiscall`, (clone, cloning process)): records the clone in the cloning
/// process's clone map; `00a5d7a0` (same words): the property base's member copy.
const OBJECT_RECORD_CLONE: u32 = 0x00a5_d450;
const PROPERTY_COPY_MEMBERS: u32 = 0x00a5_d7a0;
/// `NiProperty::LoadBinary` (`00a5d7f0`, `thiscall`, the stream) and
/// `NiProperty::GetViewerStrings` (`00a5d850`, `thiscall`, the strings array).
const PROPERTY_LOAD_BINARY: u32 = 0x00a5_d7f0;
const PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00a5_d850;
/// `NiGetViewerString` of a class name (`00a7d330`, `cdecl`, one word), of a `bool`
/// (`00a7d370`, `cdecl`, name and the word holding the byte) and of a C string (`00a7d5e0`,
/// `cdecl`, name and string).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const GET_VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
/// `0096ad30` (`thiscall`, the new size) grows an `NiTArray`; `0096ae90` (`thiscall`, index
/// and the address of the element) stores an element.
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The first word of `NiZBufferProperty`'s `NiRTTI` (its class name).
const ZBUFFER_PROPERTY_RTTI_NAME: u32 = 0x011f_443c;
/// The names `GetViewerStrings` passes: `"m_bZTest"`, `"m_bZWrite"`, `"Test Function"`.
const NAME_Z_TEST: u32 = 0x0109_b920;
const NAME_Z_WRITE: u32 = 0x0109_b914;
const NAME_TEST_FUNCTION: u32 = 0x0109_b890;
/// The test function names `GetViewerStrings` puts in its local table, in order:
/// `"TEST_ALWAYS"`, `"TEST_LESS"`, `"TEST_EQUAL"`, `"TEST_LESSEQUAL"`, `"TEST_GREATER"`,
/// `"TEST_NOTEQUAL"`, `"TEST_GREATEREQUAL"`, `"TEST_NEVER"`.
const TEST_FUNCTION_NAMES: [u32; 8] = [
    0x0109_b908,
    0x0109_b8fc,
    0x0109_b8f0,
    0x0109_b8e0,
    0x0109_b8d0,
    0x0109_b8c0,
    0x0109_b8ac,
    0x0109_b8a0,
];

// Translated from 00a5f480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiZBufferProperty::IsEqual` (Xbox PDB): the property base's comparison (`00a62140`),
/// then the flag word's (+0x18) z-test bit (0), z-write bit (1) and test function (bits
/// 2-5); only the low byte is compared. True only when all match.
pub fn ni_zbuffer_property_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(PROPERTY_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if (e.mem.u8(other.addr() + 0x18) ^ e.mem.u8(this.addr() + 0x18)) & 1 != 0 {
        return false;
    }
    let other_flags = e.mem.u8(other.addr() + 0x18);
    let flags = e.mem.u8(this.addr() + 0x18);
    if ((other_flags >> 1) ^ (flags >> 1)) & 1 != 0 {
        return false;
    }
    (other_flags ^ flags) & 0x3c == 0
}

// Translated from 00a5f510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiZBufferProperty::CreateClone` (Xbox PDB): a property without controllers
/// (`NiObjectNET` +0xc null) is its own clone (recorded by `00a5d450`); otherwise a new
/// z-buffer property (0x1c bytes, the base constructor, the vtable, flags 0xf; null when the
/// allocation fails) given the base's members (`00a5d7a0`) and this one's flags (+0x18).
/// Answers the clone.
pub fn ni_zbuffer_property_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    if e.mem.u32(this.addr() + 0xc) == 0 {
        e.call(OBJECT_RECORD_CLONE, &args![this, this, cloning_process]);
        return this;
    }
    let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
    let clone = if block == 0 {
        0
    } else {
        e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, NI_ZBUFFER_PROPERTY_VTABLE);
        e.mem.set_u16(block + 0x18, 0xf);
        block
    };
    e.call(PROPERTY_COPY_MEMBERS, &args![this, clone, cloning_process]);
    let flags = e.mem.u16(this.addr() + 0x18);
    e.mem.set_u16(clone.wrapping_add(0x18), flags);
    Ptr::new(clone)
}

/// Reads `size` bytes from the stream's input (`NiStream` +0x24c, the read function +8, the
/// position +4) into `destination`, the byte-count word at `count` set to `size` first.
fn stream_read(e: &mut Engine, stream: Ptr, destination: u32, size: u32, count: u32) {
    let input = e.mem.u32(stream.addr() + 0x24c);
    let function = e.mem.u32(input + 8);
    e.mem.set_u32(count, size);
    let done = e
        .call(function, &args![input, destination, size, count, 1u32])
        .u32();
    let position = e.mem.u32(input + 4).wrapping_add(done);
    e.mem.set_u32(input + 4, position);
}

// Translated from 00a5f580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiZBufferProperty::LoadBinary` (Xbox PDB): `NiProperty::LoadBinary`; for files older
/// than version `0x0a000102` (`NiStream` +0xd8) the flags (+0x18) are the stream's word at
/// +0x288 cut to its low 2 bits, newer ones read them (2 bytes). Versions `0x04010005` up to
/// (not including) `0x14010002` then read a 4-byte test function into the local word (which
/// starts as `this`), shifted left by 2 into bits 2-5 (`(flags & 0xffc3) | word * 4`, 16
/// bits, so higher bits of the word land in the flags too). The stream argument's stack word
/// is the byte-count word.
pub fn ni_zbuffer_property_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PROPERTY_LOAD_BINARY, &args![this, stream]);
    let flags_at = this.addr() + 0x18;
    e.with_stack(8, |e, frame| {
        let word = frame.addr();
        let count = frame.addr() + 4;
        e.mem.set_u32(word, this.addr());
        if e.mem.u32(stream.addr() + 0xd8) < 0x0a00_0102 {
            let flags = e.mem.u16(stream.addr() + 0x288) & 3;
            e.mem.set_u16(flags_at, flags);
        } else {
            stream_read(e, stream, flags_at, 2, count);
        }
        let version = e.mem.u32(stream.addr() + 0xd8);
        if (0x0401_0005..0x1401_0002).contains(&version) {
            stream_read(e, stream, word, 4, count);
            let function = e.mem.u32(word).wrapping_mul(4);
            let flags = (e.mem.u16(flags_at) & 0xffc3) | function as u16;
            e.mem.set_u16(flags_at, flags);
        }
    });
}

/// Appends the string pointer held in the word at `slot` to the viewer-string array: when
/// the count (`m_usSize`, +0xa) is not below the capacity (`m_usMaxSize`, +8) the array first
/// grows to count + `m_usGrowBy` (+0xe, `0096ad30`); then `0096ae90` stores it at the old
/// count.
fn append_viewer_string(e: &mut Engine, strings: Ptr, slot: u32) {
    let count = e.mem.u16(strings.addr() + 0xa) as u32;
    let capacity = e.mem.u16(strings.addr() + 8) as u32;
    if count >= capacity {
        let grow_by = e.mem.u16(strings.addr() + 0xe) as u32;
        e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
    }
    e.call(ARRAY_SET_AT, &args![strings, count, slot]);
}

/// The word the game pushes for a byte argument it writes into the low byte of its stack word
/// at `slot`: that byte, over the upper three bytes the word already holds.
fn byte_in_slot(e: &mut Engine, slot: u32, value: u8) -> u32 {
    e.mem.set_u8(slot, value);
    e.mem.u32(slot)
}

// Translated from 00a5f640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiZBufferProperty::GetViewerStrings` (Xbox PDB): the property base's strings
/// (`00a5d850`), then four appended to `strings`: the class name (`NiGetViewerString` of the
/// first word of the `NiRTTI` at `011f443c`), `m_bZTest` (flag bit 0), `m_bZWrite` (bit 1)
/// and `"Test Function"` with the name of the test function (bits 2-5) from the local table
/// of eight names.
///
/// The game keeps each string pointer in the `strings` argument's stack word and passes the
/// boolean values by writing their byte into the low byte of that same word, as here. The
/// test function is 4 bits wide and the table has eight entries: values 8 to 15 read the
/// stack words above the table, as the exe does (the return address word, which the
/// engine's stack holds as zero, then the argument's own word, which holds the `m_bZWrite`
/// string, then six words of the caller's stack, zero here).
pub fn ni_zbuffer_property_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    // Frame: the table (8 words), the return address word, the argument word, the caller's
    // six words above it.
    e.with_stack(0x40, |e, frame| {
        let table = frame.addr();
        let slot = table + 0x24;
        e.mem.set_u32(slot, strings.addr());
        let class_name = e.global::<u32>(ZBUFFER_PROPERTY_RTTI_NAME);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);

        let flags = e.mem.u8(this.addr() + 0x18);
        let word = byte_in_slot(e, slot, flags & 1);
        let text = e
            .call(GET_VIEWER_STRING_BOOL, &args![NAME_Z_TEST, word])
            .u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);

        let flags = e.mem.u8(this.addr() + 0x18);
        let word = byte_in_slot(e, slot, (flags >> 1) & 1);
        let text = e
            .call(GET_VIEWER_STRING_BOOL, &args![NAME_Z_WRITE, word])
            .u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);

        let index = (e.mem.u16(this.addr() + 0x18) as u32 >> 2) & 0xf;
        for (i, name) in TEST_FUNCTION_NAMES.iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *name);
        }
        let function_name = e.mem.u32(table + 4 * index);
        let text = e
            .call(
                GET_VIEWER_STRING_TEXT,
                &args![NAME_TEST_FUNCTION, function_name],
            )
            .u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5_f480, ni_zbuffer_property_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a5_f510, ni_zbuffer_property_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a5_f580, ni_zbuffer_property_load_binary(Ptr, Ptr)),
        entry!(
            0x00a5_f640,
            ni_zbuffer_property_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        for double in [
            OBJECT_RECORD_CLONE,
            PROPERTY_COPY_MEMBERS,
            PROPERTY_LOAD_BINARY,
            PROPERTY_GET_VIEWER_STRINGS,
            ARRAY_SET_SIZE,
            ARRAY_SET_AT,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn is_equal_compares_the_low_flag_bits() {
        let mut e = engine();
        e.register(PROPERTY_IS_EQUAL, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x1c);
        let other = e.mem.alloc(0x1c);
        e.mem.set_u16(this + 0x18, 0x3f);
        e.mem.set_u16(other + 0x18, 0xff7f);
        assert!(e.call(0x00a5_f480, &args![this, other]).bool());
        assert_eq!(calls_to(&e, PROPERTY_IS_EQUAL), vec![vec![this, other]]);
        for bit in 0..6 {
            e.mem.set_u16(other + 0x18, 0x3f ^ (1 << bit));
            assert!(!e.call(0x00a5_f480, &args![this, other]).bool());
        }
        e.mem.set_u16(other + 0x18, 0x3f);
        e.register(PROPERTY_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a5_f480, &args![this, other]).bool());
    }

    #[test]
    fn create_clone_shares_or_copies() {
        let mut e = engine();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u16(this + 0x18, 0x1234);
        assert_eq!(e.call(0x00a5_f510, &args![this, 0x66u32]).u32(), this);
        assert_eq!(
            calls_to(&e, OBJECT_RECORD_CLONE),
            vec![vec![this, this, 0x66]]
        );
        e.mem.set_u32(this + 0xc, 1);
        let clone = e.call(0x00a5_f510, &args![this, 0x66u32]).u32();
        assert_ne!(clone, this);
        assert_eq!(e.mem.u32(clone), NI_ZBUFFER_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(clone + 0x18), 0x1234);
        assert_eq!(
            calls_to(&e, PROPERTY_COPY_MEMBERS),
            vec![vec![this, clone, 0x66]]
        );
        assert_eq!(calls_to(&e, OBJECT_BASE_CONSTRUCTOR), vec![vec![clone]]);
    }

    fn stream_engine(words: Vec<u32>) -> (Engine, u32, u32) {
        let mut e = engine();
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, 0x0300_0000);
        let mut next = words.into_iter();
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (a[2], 1));
            let value = next.next().unwrap();
            e.mem.write(a[1], &value.to_le_bytes()[..a[2] as usize]);
            a[2].into_ret()
        });
        (e, stream, input)
    }

    #[test]
    fn load_binary_reads_by_version() {
        let this_size = 0x1c;
        // Newest files: the flags only.
        let (mut e, stream, input) = stream_engine(vec![0xabcd]);
        let this = e.mem.alloc(this_size);
        e.mem.set_u32(stream + 0xd8, 0x1401_0002);
        e.call(0x00a5_f580, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 0x18), 0xabcd);
        assert_eq!(e.mem.u32(input + 4), 2);
        // Middle versions: the flags, then the test function merged in.
        let (mut e, stream, input) = stream_engine(vec![0xffff, 5]);
        let this = e.mem.alloc(this_size);
        e.mem.set_u32(stream + 0xd8, 0x0a00_0102);
        e.call(0x00a5_f580, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 0x18), 0xffd7);
        assert_eq!(e.mem.u32(input + 4), 6);
        // Old files between 0x04010005 and 0x0a000102: the stream's word, then the word.
        let (mut e, stream, _) = stream_engine(vec![0x401]);
        let this = e.mem.alloc(this_size);
        e.mem.set_u32(stream + 0xd8, 0x0401_0005);
        e.mem.set_u16(stream + 0x288, 0xffff);
        e.call(0x00a5_f580, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 0x18), 0x1007);
        // Older still: the stream's word only.
        let (mut e, stream, input) = stream_engine(vec![]);
        let this = e.mem.alloc(this_size);
        e.mem.set_u32(stream + 0xd8, 0x0401_0004);
        e.mem.set_u16(stream + 0x288, 0xfffe);
        e.call(0x00a5_f580, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 0x18), 2);
        assert_eq!(e.mem.u32(input + 4), 0);
    }

    #[test]
    fn viewer_strings_list_the_fields_in_order() {
        let mut e = engine();
        e.register(GET_VIEWER_STRING, |_, _| 0x1234_5600u32.into_ret());
        e.register(GET_VIEWER_STRING_BOOL, |_, a| {
            (0x0700_0000 | (a[1] & 1)).into_ret()
        });
        e.register(GET_VIEWER_STRING_TEXT, |_, a| a[1].into_ret());
        e.mem.set_u32(ZBUFFER_PROPERTY_RTTI_NAME, 0x0109_b000);
        let this = e.mem.alloc(0x1c);
        let strings = e.mem.alloc(0x10);
        for (flags, function_name) in [
            (0x09u16, 0x0109_b8f0u32),
            (0x0a, 0x0109_b8f0),
            (0x22, 0),
            (0x26, 0x0700_0001),
        ] {
            e.call_log = Some(vec![]);
            e.mem.set_u16(this + 0x18, flags);
            e.call(0x00a5_f640, &args![this, strings]);
            let bools = calls_to(&e, GET_VIEWER_STRING_BOOL);
            assert_eq!(
                bools[0],
                vec![NAME_Z_TEST, 0x1234_5600 | (flags as u32 & 1)]
            );
            let z_write = (flags as u32 >> 1) & 1;
            assert_eq!(bools[1], vec![NAME_Z_WRITE, 0x0700_0000 | z_write]);
            assert_eq!(
                calls_to(&e, GET_VIEWER_STRING_TEXT),
                vec![vec![NAME_TEST_FUNCTION, function_name]]
            );
            assert_eq!(calls_to(&e, ARRAY_SET_AT).len(), 4);
        }
    }
}
