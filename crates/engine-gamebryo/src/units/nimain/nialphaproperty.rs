//! `NiMain/nialphaproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a47360`: the jump target of `00a950c0`.
const JUMP_TARGET_00A950C0: u32 = 0x00a4_7360;

/// `00a62130`: the jump target of `00a99e40`.
const JUMP_TARGET_00A99E40: u32 = 0x00a6_2130;

// Translated from 00a950c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty::LinkObject` (Xbox PDB): a 5-byte jump to `00a47360` (engine map: `NiSequenceStreamHelper::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiAlphaProperty@@UAAXAAVNiStream@@@Z`.
pub fn ni_alpha_property_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A950C0, &args![this, stream]);
}

// Translated from 00a99e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a62130` (engine map: `NiSequenceStreamHelper::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiAlphaProperty@@UAA_NAAVNiStream@@@Z`.
pub fn ni_alpha_property_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A99E40, &args![this, stream]).bool()
}

use crate::units::nimain::nimainsdm::{assign_ref_counted, ALLOCATE, OBJECT_BASE_CONSTRUCTOR};

/// `011f4410`: the pointer `00a5cef0` stores the object it builds in.
const ALPHA_PROPERTY_SLOT: u32 = 0x011f_4410;

// Translated from 00a5cef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nialphaproperty.cpp`; run by the start-up `00a89cc0`,
/// no arguments): builds a 0x1c-byte object (allocation `00aa13e0`, base constructor
/// `00a5c200`, vtable `010162dc`, the word at `+0x18` set to 0xec, the byte at `+0x1a` to
/// 0) and stores it, counted, in `011f4410`, releasing the object held there before. A
/// failed allocation stores null.
pub fn fn_00a5cef0(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, 0x0101_62dc);
        e.mem.set_u16(block + 0x18, 0xec);
        e.mem.set_u8(block + 0x1a, 0);
        block
    };
    assign_ref_counted(e, ALPHA_PROPERTY_SLOT, object);
}

/// `NiAlphaProperty`'s vtable.
const NI_ALPHA_PROPERTY_VTABLE: u32 = 0x0101_62dc;
/// `00a5d450` (`thiscall`, (clone, cloning process)): records the clone in the cloning
/// process's clone map; `00a5d7a0` (same words): the property base's member copy.
const OBJECT_RECORD_CLONE: u32 = 0x00a5_d450;
const PROPERTY_COPY_MEMBERS: u32 = 0x00a5_d7a0;
/// `NiProperty::LoadBinary` (`thiscall`, the stream).
const PROPERTY_LOAD_BINARY: u32 = 0x00a5_d7f0;
/// `011f4404`: a byte that, when set, makes every alpha property clone a copy.
const ALWAYS_COPY_ALPHA: u32 = 0x011f_4404;
/// The import slot of `InterlockedDecrement`.
const INTERLOCKED_DECREMENT_SLOT: u32 = 0x00fd_f1c4;

/// A new alpha property (0x1c bytes, `operator new`, the base constructor, flags `0xec`,
/// threshold 0, the vtable `010162dc`); null when the allocation fails.
fn new_alpha_property(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
    e.mem.set_u16(block + 0x18, 0xec);
    e.mem.set_u32(block, NI_ALPHA_PROPERTY_VTABLE);
    e.mem.set_u8(block + 0x1a, 0);
    block
}

// Translated from 00a5ceb0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiAlphaProperty`): a new alpha property (flags `0xec`, threshold 0), or null.
pub fn fn_00a5ceb0(e: &mut Engine) -> Ptr {
    Ptr::new(new_alpha_property(e))
}

// Translated from 00a5ce30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty::CreateClone` (Xbox PDB): a property without controllers (`NiObjectNET`
/// +0xc null), unless the byte at `011f4404` is set, is its own clone (recorded by
/// `00a5d450`); otherwise a new alpha property (null when the allocation fails) given the
/// base's members (`00a5d7a0`) and this one's flags (+0x18) and threshold (+0x1a). Answers
/// the clone.
pub fn ni_alpha_property_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    if e.mem.u32(this.addr() + 0xc) == 0 && e.mem.u8(ALWAYS_COPY_ALPHA) == 0 {
        e.call(OBJECT_RECORD_CLONE, &args![this, this, cloning_process]);
        return this;
    }
    let clone = new_alpha_property(e);
    e.call(PROPERTY_COPY_MEMBERS, &args![this, clone, cloning_process]);
    let flags = e.mem.u16(this.addr() + 0x18);
    e.mem.set_u16(clone.wrapping_add(0x18), flags);
    let threshold = e.mem.u8(this.addr() + 0x1a);
    e.mem.set_u8(clone.wrapping_add(0x1a), threshold);
    Ptr::new(clone)
}

// Translated from 00a5cf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nialphaproperty.cpp`; the shut-down counterpart of
/// `00a5cef0`): releases the alpha property held in `011f4410` (its slot `+4` when the count
/// reaches 0) and clears the word; nothing when it is null.
pub fn fn_00a5cf60(e: &mut Engine) {
    let held = e.mem.u32(ALPHA_PROPERTY_SLOT);
    if held == 0 {
        return;
    }
    if e.call(INTERLOCKED_DECREMENT_SLOT, &args![held.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(held, 4, &args![]);
    }
    e.mem.set_u32(ALPHA_PROPERTY_SLOT, 0);
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

// Translated from 00a5cf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty::LoadBinary` (Xbox PDB): `NiProperty::LoadBinary`; for files older than
/// version `0x0a000102` (`NiStream` +0xd8) the flags (+0x18) are the stream's word at +0x288
/// cut to its low 14 bits, newer ones read them (2 bytes); then the 1-byte threshold
/// (+0x1a). The stream argument's stack word is the byte-count word.
pub fn ni_alpha_property_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PROPERTY_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        if e.mem.u32(stream.addr() + 0xd8) < 0x0a00_0102 {
            let flags = e.mem.u16(stream.addr() + 0x288) & 0x3fff;
            e.mem.set_u16(this.addr() + 0x18, flags);
        } else {
            stream_read(e, stream, this.addr() + 0x18, 2, count.addr());
        }
        stream_read(e, stream, this.addr() + 0x1a, 1, count.addr());
    });
}

/// `NiProperty`'s `IsEqual` (`00a62140`, `thiscall`, the other object; the engine map names
/// it `NiSequenceStreamHelper::IsEqual`, folded code), `SaveBinary` (`00a5d7b0`, `thiscall`,
/// the stream) and `GetViewerStrings` (`00a5d850`, `thiscall`, the strings array).
const PROPERTY_IS_EQUAL: u32 = 0x00a6_2140;
const PROPERTY_SAVE_BINARY: u32 = 0x00a5_d7b0;
const PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00a5_d850;
/// `00aa1070` (unnamed, `cdecl`, one word): allocates the text of a viewer string.
const ALLOCATE_TEXT: u32 = 0x00aa_1070;
/// `NiSprintf` (`00a305b0`, `cdecl`: buffer, size, format, then the format's arguments).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `NiGetViewerString` of a class name (`00a7d330`, `cdecl`, one word), of a `bool`
/// (`00a7d370`, `cdecl`, name and the word holding the byte) and of an unsigned char
/// (`00a7d410`, same words).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const GET_VIEWER_STRING_UCHAR: u32 = 0x00a7_d410;
/// `0096ad30` (`thiscall`, the new size) grows an `NiTArray`; `0096ae90` (`thiscall`, index
/// and the address of the element) stores an element.
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The first word of `NiAlphaProperty`'s `NiRTTI` (its class name).
const ALPHA_PROPERTY_RTTI_NAME: u32 = 0x011f_4408;
/// `"%s = UNKNOWN!!!"`: the format both enum viewer strings use for a value they do not name.
const FORMAT_UNKNOWN: u32 = 0x0109_b2ec;
/// The member names `GetViewerStrings` passes: `"m_bAlpha"`, `"m_eSrcBlend"`,
/// `"m_eDestBlend"`, `"m_bAlphaTest"`, `"m_eTestMode"`, `"m_ucTestRef"`, `"m_bNoSorter"`.
const NAME_ALPHA: u32 = 0x0109_b4d4;
const NAME_SRC_BLEND: u32 = 0x0109_b4c8;
const NAME_DEST_BLEND: u32 = 0x0109_b4b8;
const NAME_ALPHA_TEST: u32 = 0x0109_b4a8;
const NAME_TEST_MODE: u32 = 0x0109_b49c;
const NAME_TEST_REF: u32 = 0x0109_b490;
const NAME_NO_SORTER: u32 = 0x0109_b484;

// Translated from 00a5cb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty::IsEqual` (Xbox PDB): the property base's comparison (`00a62140`), then
/// the flag word's (+0x18) fields one by one (bit 0, bits 1-4, bits 5-8, bit 9, bits 10-12),
/// the threshold byte (+0x1a) and bit 13. True only when all match.
pub fn ni_alpha_property_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(PROPERTY_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if (e.mem.u8(this.addr() + 0x18) ^ e.mem.u8(other.addr() + 0x18)) & 1 != 0 {
        return false;
    }
    let other_flags = e.mem.u16(other.addr() + 0x18) as u32;
    let flags = e.mem.u16(this.addr() + 0x18) as u32;
    let differ = other_flags ^ flags;
    if differ & 0x1e != 0 || differ & 0x1e0 != 0 {
        return false;
    }
    if ((other_flags >> 9) ^ (flags >> 9)) & 1 != 0 {
        return false;
    }
    if differ & 0x1c00 != 0 {
        return false;
    }
    if e.mem.u8(this.addr() + 0x1a) != e.mem.u8(other.addr() + 0x1a) {
        return false;
    }
    ((other_flags >> 0xd) ^ (flags >> 0xd)) & 1 == 0
}

/// The text of an enum viewer string: `strlen(name) + 0x16` bytes from `00aa1070`, written by
/// `NiSprintf` with `format` and `name`. Answers the text.
fn enum_viewer_string(e: &mut Engine, name: Ptr, format: u32) -> Ptr {
    let size = (e.mem.cstr(name.addr()).len() as u32).wrapping_add(0x16);
    let text = e.call(ALLOCATE_TEXT, &args![size]).u32();
    e.call(NI_SPRINTF, &args![text, size, format, name]);
    Ptr::new(text)
}

// Translated from 00a5cb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty::GetViewerString` (Xbox PDB), `cdecl`: `"<name> = ALPHA_..."` for a blend
/// mode 0 to 10 (`ALPHA_ONE`, `ALPHA_ZERO`, `ALPHA_SRCCOLOR`, `ALPHA_INVSRCCOLOR`,
/// `ALPHA_DESTCOLOR`, `ALPHA_INVDESTCOLOR`, `ALPHA_SRCALPHA`, `ALPHA_INVSRCALPHA`,
/// `ALPHA_DESTALPHA`, `ALPHA_INVDESTALPHA`, `ALPHA_SRCALPHASAT`; the formats at `0109b3dc`
/// down to `0109b2fc`), `"<name> = UNKNOWN!!!"` for any other value. The compiler's jump
/// table is the `match`.
pub fn ni_alpha_property_get_viewer_string(e: &mut Engine, name: Ptr, mode: u32) -> Ptr {
    let format = match mode {
        0 => 0x0109_b3dc,
        1 => 0x0109_b3cc,
        2 => 0x0109_b3b8,
        3 => 0x0109_b3a0,
        4 => 0x0109_b388,
        5 => 0x0109_b370,
        6 => 0x0109_b35c,
        7 => 0x0109_b344,
        8 => 0x0109_b32c,
        9 => 0x0109_b314,
        10 => 0x0109_b2fc,
        _ => FORMAT_UNKNOWN,
    };
    enum_viewer_string(e, name, format)
}

// Translated from 00a5cd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty::GetViewerString_ov2` (Xbox PDB), `cdecl`: `"<name> = TEST_..."` for a test
/// mode 0 to 7 (`TEST_ALWAYS`, `TEST_LESS`, `TEST_EQUAL`, `TEST_LESSEQUAL`, `TEST_GREATER`,
/// `TEST_NOTEQUAL`, `TEST_GREATEREQUAL`, `TEST_NEVER`; the formats at `0109b470` down to
/// `0109b3ec`), `"<name> = UNKNOWN!!!"` for any other value.
pub fn ni_alpha_property_get_viewer_string_ov2(e: &mut Engine, name: Ptr, mode: u32) -> Ptr {
    let format = match mode {
        0 => 0x0109_b470,
        1 => 0x0109_b460,
        2 => 0x0109_b450,
        3 => 0x0109_b43c,
        4 => 0x0109_b428,
        5 => 0x0109_b414,
        6 => 0x0109_b3fc,
        7 => 0x0109_b3ec,
        _ => FORMAT_UNKNOWN,
    };
    enum_viewer_string(e, name, format)
}

/// Writes `size` bytes at `source` to the stream's output (`NiStream` +0x250, the write
/// function +0xc, the position +4), the byte-count word at `count` set to `size` first.
fn stream_write(e: &mut Engine, stream: Ptr, source: u32, size: u32, count: u32) {
    let output = e.mem.u32(stream.addr() + 0x250);
    let function = e.mem.u32(output + 0xc);
    e.mem.set_u32(count, size);
    let done = e
        .call(function, &args![output, source, size, count, 1u32])
        .u32();
    let position = e.mem.u32(output + 4).wrapping_add(done);
    e.mem.set_u32(output + 4, position);
}

// Translated from 00a5d020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty::SaveBinary` (Xbox PDB): the property base's `SaveBinary` (`00a5d7b0`),
/// then the flags (+0x18, 2 bytes) and the threshold (+0x1a, 1 byte). The stream argument's
/// stack word is the byte-count word.
pub fn ni_alpha_property_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PROPERTY_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        stream_write(e, stream, this.addr() + 0x18, 2, count.addr());
        stream_write(e, stream, this.addr() + 0x1a, 1, count.addr());
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

// Translated from 00a5d080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaProperty::GetViewerStrings` (Xbox PDB): the property base's strings (`00a5d850`),
/// then eight appended to `strings`: the class name (`NiGetViewerString` of the first word of
/// the `NiRTTI` at `011f4408`), `m_bAlpha` (flag bit 0), `m_eSrcBlend` (bits 1-4,
/// [`ni_alpha_property_get_viewer_string`]), `m_eDestBlend` (bits 5-8), `m_bAlphaTest` (bit
/// 9), `m_eTestMode` (bits 10-12, [`ni_alpha_property_get_viewer_string_ov2`]),
/// `m_ucTestRef` (the threshold byte +0x1a) and `m_bNoSorter` (bit 13).
///
/// The game keeps each string pointer in the `strings` argument's stack word and passes the
/// boolean and byte values by writing their byte into the low byte of that same word, so the
/// word pushed holds the previous string pointer's upper three bytes, as here.
pub fn ni_alpha_property_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, strings.addr());
        let class_name = e.global::<u32>(ALPHA_PROPERTY_RTTI_NAME);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);

        let flags = e.mem.u8(this.addr() + 0x18);
        let word = byte_in_slot(e, slot, flags & 1);
        let text = e
            .call(GET_VIEWER_STRING_BOOL, &args![NAME_ALPHA, word])
            .u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);

        let flags = e.mem.u16(this.addr() + 0x18) as u32;
        let text =
            ni_alpha_property_get_viewer_string(e, Ptr::new(NAME_SRC_BLEND), (flags >> 1) & 0xf);
        e.mem.set_u32(slot, text.addr());
        append_viewer_string(e, strings, slot);

        let flags = e.mem.u16(this.addr() + 0x18) as u32;
        let text =
            ni_alpha_property_get_viewer_string(e, Ptr::new(NAME_DEST_BLEND), (flags >> 5) & 0xf);
        e.mem.set_u32(slot, text.addr());
        append_viewer_string(e, strings, slot);

        let flags = e.mem.u16(this.addr() + 0x18);
        let word = byte_in_slot(e, slot, ((flags >> 9) as u8) & 1);
        let text = e
            .call(GET_VIEWER_STRING_BOOL, &args![NAME_ALPHA_TEST, word])
            .u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);

        let flags = e.mem.u16(this.addr() + 0x18) as u32;
        let text =
            ni_alpha_property_get_viewer_string_ov2(e, Ptr::new(NAME_TEST_MODE), (flags >> 10) & 7);
        e.mem.set_u32(slot, text.addr());
        append_viewer_string(e, strings, slot);

        let threshold = e.mem.u8(this.addr() + 0x1a);
        let word = byte_in_slot(e, slot, threshold);
        let text = e
            .call(GET_VIEWER_STRING_UCHAR, &args![NAME_TEST_REF, word])
            .u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);

        let flags = e.mem.u16(this.addr() + 0x18);
        let word = byte_in_slot(e, slot, ((flags >> 0xd) as u8) & 1);
        let text = e
            .call(GET_VIEWER_STRING_BOOL, &args![NAME_NO_SORTER, word])
            .u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5_ceb0, fn_00a5ceb0() -> Ptr),
        entry!(0x00a5_ce30, ni_alpha_property_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a5_cf60, fn_00a5cf60()),
        entry!(0x00a5_cf90, ni_alpha_property_load_binary(Ptr, Ptr)),
        entry!(0x00a9_50c0, ni_alpha_property_link_object(Ptr, Ptr)),
        entry!(0x00a9_9e40, ni_alpha_property_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a5_cef0, fn_00a5cef0()),
        entry!(0x00a5_cb10, ni_alpha_property_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a5_cb90, ni_alpha_property_get_viewer_string(Ptr, u32) -> Ptr),
        entry!(0x00a5_cd00, ni_alpha_property_get_viewer_string_ov2(Ptr, u32) -> Ptr),
        entry!(0x00a5_d020, ni_alpha_property_save_binary(Ptr, Ptr)),
        entry!(0x00a5_d080, ni_alpha_property_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a950c0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_50c0;
        const TARGET: u32 = 0x00a4_7360;
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
    fn thunk_00a99e40_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_9e40;
        const TARGET: u32 = 0x00a6_2130;
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

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn builds_the_object_and_replaces_the_held_one() {
        let run = run_default_instance(0x00a5_cef0, ALPHA_PROPERTY_SLOT, |_| {});
        let e = &run.engine;
        assert_ne!(run.object, 0);
        assert_ne!(run.object, run.previous);
        assert_eq!(e.mem.u32(run.object), 0x0101_62dc);
        assert_eq!(e.mem.u16(run.object + 0x18), 0xec);
        assert_eq!(e.mem.u8(run.object + 0x1a), 0);
        assert_eq!(e.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let run = run_default_instance(0x00a5_cef0, ALPHA_PROPERTY_SLOT, |e| {
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
        });
        assert_eq!(run.object, 0);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }
}

#[cfg(test)]
mod alpha_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        for double in [
            OBJECT_RECORD_CLONE,
            PROPERTY_COPY_MEMBERS,
            PROPERTY_LOAD_BINARY,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn create_object_builds_the_default_property() {
        let mut e = engine();
        let object = e.call(0x00a5_ceb0, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_ALPHA_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(object + 0x18), 0xec);
        assert_eq!(e.mem.u8(object + 0x1a), 0);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a5_ceb0, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_shares_or_copies() {
        let mut e = engine();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u16(this + 0x18, 0x1234);
        e.mem.set_u8(this + 0x1a, 0x80);
        assert_eq!(e.call(0x00a5_ce30, &args![this, 0x66u32]).u32(), this);
        assert_eq!(
            calls_to(&e, OBJECT_RECORD_CLONE),
            vec![vec![this, this, 0x66]]
        );
        // The always-copy byte makes a copy.
        e.mem.set_u8(ALWAYS_COPY_ALPHA, 1);
        let clone = e.call(0x00a5_ce30, &args![this, 0x66u32]).u32();
        assert_ne!(clone, this);
        assert_eq!(e.mem.u32(clone), NI_ALPHA_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(clone + 0x18), 0x1234);
        assert_eq!(e.mem.u8(clone + 0x1a), 0x80);
        assert_eq!(
            calls_to(&e, PROPERTY_COPY_MEMBERS),
            vec![vec![this, clone, 0x66]]
        );
        // So do controllers.
        e.mem.set_u8(ALWAYS_COPY_ALPHA, 0);
        e.mem.set_u32(this + 0xc, 1);
        assert_ne!(e.call(0x00a5_ce30, &args![this, 0x66u32]).u32(), this);
    }

    #[test]
    fn release_of_the_default_alpha_property() {
        let mut e = engine();
        e.put_vtable(0x0300_1000, &[0, 0x0300_1004]);
        e.register(0x0300_1004, |_, _| Ret::default());
        let held = e.mem.alloc(0x1c);
        e.mem.set_u32(held, 0x0300_1000);
        e.mem.set_u32(held + 4, 1);
        e.mem.set_u32(ALPHA_PROPERTY_SLOT, held);
        e.call(0x00a5_cf60, &args![]);
        assert_eq!(e.mem.u32(ALPHA_PROPERTY_SLOT), 0);
        assert_eq!(calls_to(&e, 0x0300_1004), vec![vec![held]]);
        e.call(0x00a5_cf60, &args![]);
        assert_eq!(calls_to(&e, 0x0300_1004).len(), 1);
    }

    #[test]
    fn load_binary_reads_flags_and_threshold() {
        let mut e = engine();
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, 0x0300_0000);
        e.register(0x0300_0000, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (a[2], 1));
            let value: u32 = if a[2] == 2 { 0xabcd } else { 0x7f };
            e.mem.write(a[1], &value.to_le_bytes()[..a[2] as usize]);
            a[2].into_ret()
        });
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(stream + 0xd8, 0x0a00_0102);
        e.call(0x00a5_cf90, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 0x18), 0xabcd);
        assert_eq!(e.mem.u8(this + 0x1a), 0x7f);
        assert_eq!(e.mem.u32(input + 4), 3);
        e.mem.set_u32(stream + 0xd8, 0x0a00_0101);
        e.mem.set_u16(stream + 0x288, 0xffff);
        e.call(0x00a5_cf90, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 0x18), 0x3fff);
        assert_eq!(e.mem.u32(input + 4), 4);
    }
}

#[cfg(test)]
mod n015_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::calls_to;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn is_equal_compares_every_field() {
        let mut e = engine();
        e.register(PROPERTY_IS_EQUAL, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x1c);
        let other = e.mem.alloc(0x1c);
        e.mem.set_u16(this + 0x18, 0x3fff);
        e.mem.set_u16(other + 0x18, 0x3fff);
        e.mem.set_u8(this + 0x1a, 7);
        e.mem.set_u8(other + 0x1a, 7);
        assert!(e.call(0x00a5_cb10, &args![this, other]).bool());
        assert_eq!(calls_to(&e, PROPERTY_IS_EQUAL), vec![vec![this, other]]);
        // Bits 14 and 15 are not compared.
        e.mem.set_u16(other + 0x18, 0xffff);
        assert!(e.call(0x00a5_cb10, &args![this, other]).bool());
        for bit in 0..14 {
            e.mem.set_u16(other + 0x18, 0x3fff ^ (1 << bit));
            assert!(
                !e.call(0x00a5_cb10, &args![this, other]).bool(),
                "bit {bit}"
            );
        }
        e.mem.set_u16(other + 0x18, 0x3fff);
        e.mem.set_u8(other + 0x1a, 8);
        assert!(!e.call(0x00a5_cb10, &args![this, other]).bool());
        e.mem.set_u8(other + 0x1a, 7);
        e.register(PROPERTY_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a5_cb10, &args![this, other]).bool());
    }

    fn sprintf_engine() -> Engine {
        let mut e = engine();
        e.register(ALLOCATE_TEXT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        e
    }

    #[test]
    fn blend_mode_strings_pick_the_format_by_value() {
        let mut e = sprintf_engine();
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"abc");
        let text = e.call(0x00a5_cb90, &args![name, 0u32]).u32();
        assert_eq!(calls_to(&e, ALLOCATE_TEXT), vec![vec![0x19]]);
        assert_eq!(
            calls_to(&e, NI_SPRINTF),
            vec![vec![text, 0x19, 0x0109_b3dc, name]]
        );
        for (mode, format) in [
            (10u32, 0x0109_b2fcu32),
            (6, 0x0109_b35c),
            (11, FORMAT_UNKNOWN),
        ] {
            e.call_log = Some(vec![]);
            e.call(0x00a5_cb90, &args![name, mode]);
            assert_eq!(calls_to(&e, NI_SPRINTF)[0][2], format);
        }
    }

    #[test]
    fn test_mode_strings_pick_the_format_by_value() {
        let mut e = sprintf_engine();
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"m_e");
        for (mode, format) in [
            (0u32, 0x0109_b470u32),
            (3, 0x0109_b43c),
            (7, 0x0109_b3ec),
            (8, FORMAT_UNKNOWN),
            (0xffff_ffff, FORMAT_UNKNOWN),
        ] {
            e.call_log = Some(vec![]);
            let text = e.call(0x00a5_cd00, &args![name, mode]).u32();
            assert_eq!(
                calls_to(&e, NI_SPRINTF),
                vec![vec![text, 0x19, format, name]]
            );
        }
    }

    #[test]
    fn save_binary_writes_flags_and_threshold() {
        let mut e = engine();
        e.register(PROPERTY_SAVE_BINARY, |_, _| Ret::default());
        let stream = e.mem.alloc(0x300);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u32(output + 0xc, 0x0300_0000);
        e.mem.set_u32(output + 4, 10);
        e.register(0x0300_0000, |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            a[2].into_ret()
        });
        let this = e.mem.alloc(0x1c);
        e.call(0x00a5_d020, &args![this, stream]);
        assert_eq!(calls_to(&e, PROPERTY_SAVE_BINARY), vec![vec![this, stream]]);
        let writes = calls_to(&e, 0x0300_0000);
        assert_eq!(writes.len(), 2);
        assert_eq!(
            (writes[0][0], writes[0][1], writes[0][2], writes[0][4]),
            (output, this + 0x18, 2, 1)
        );
        assert_eq!((writes[1][1], writes[1][2]), (this + 0x1a, 1));
        assert_eq!(e.mem.u32(output + 4), 13);
    }

    #[test]
    fn viewer_strings_list_the_fields_in_order() {
        let mut e = sprintf_engine();
        e.register(PROPERTY_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x1234_5600u32.into_ret());
        e.register(GET_VIEWER_STRING_BOOL, |_, a| {
            (0x0700_0000 | a[1]).into_ret()
        });
        e.register(GET_VIEWER_STRING_UCHAR, |_, a| {
            (0x0800_0000 | a[1]).into_ret()
        });
        e.register(ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        e.map(0x0109_b000, 0x1000);
        e.mem.set_u32(ALPHA_PROPERTY_RTTI_NAME, 0x0109_b000);
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xe, 3);
        let this = e.mem.alloc(0x1c);
        // Alpha on, source 5, destination 9, test on, test mode 6, no-sorter on.
        let flags: u16 = 1 | (5 << 1) | (9 << 5) | (1 << 9) | (6 << 10) | (1 << 13);
        e.mem.set_u16(this + 0x18, flags);
        e.mem.set_u8(this + 0x1a, 0x80);
        e.call(0x00a5_d080, &args![this, strings]);
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x0109_b000]]);
        assert_eq!(
            calls_to(&e, ARRAY_SET_SIZE),
            vec![vec![strings, 5], vec![strings, 8]]
        );
        assert_eq!(e.mem.u16(strings + 0xa), 8);
        let sprintf = calls_to(&e, NI_SPRINTF);
        assert_eq!(sprintf[0][2..], [0x0109_b370, NAME_SRC_BLEND]);
        assert_eq!(sprintf[1][2..], [0x0109_b314, NAME_DEST_BLEND]);
        assert_eq!(sprintf[2][2..], [0x0109_b3fc, NAME_TEST_MODE]);
        let bools = calls_to(&e, GET_VIEWER_STRING_BOOL);
        // Each byte goes over the previous string pointer's upper bytes.
        assert_eq!(bools[0], vec![NAME_ALPHA, 0x1234_5601]);
        let dest_text = sprintf[1][0];
        assert_eq!(bools[1], vec![NAME_ALPHA_TEST, (dest_text & !0xff) | 1]);
        let test_ref = calls_to(&e, GET_VIEWER_STRING_UCHAR);
        let mode_text = sprintf[2][0];
        assert_eq!(
            test_ref,
            vec![vec![NAME_TEST_REF, (mode_text & !0xff) | 0x80]]
        );
        assert_eq!(bools[2][0], NAME_NO_SORTER);
        assert_eq!(bools[2][1] & 0xff, 1);
        assert_eq!(e.mem.u32(base), 0x1234_5600);
        assert_eq!(e.mem.u32(base + 4), 0x0700_0000 | 0x1234_5601);
        assert_eq!(e.mem.u32(base + 8), sprintf[0][0]);
        assert_eq!(e.mem.u32(base + 0x18), 0x0800_0000 | test_ref[0][1]);
    }
}
