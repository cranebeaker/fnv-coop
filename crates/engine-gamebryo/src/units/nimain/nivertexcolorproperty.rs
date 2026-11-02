//! `NiMain/nivertexcolorproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{ALLOCATE, OBJECT_BASE_CONSTRUCTOR};

/// `NiVertexColorProperty`'s vtable (its RTTI locator names `.?AVNiVertexColorProperty@@`).
const NI_VERTEX_COLOR_PROPERTY_VTABLE: u32 = 0x0109_b72c;
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
/// `00aa1070` (unnamed, `cdecl`, one word): allocates the text of a viewer string.
const ALLOCATE_TEXT: u32 = 0x00aa_1070;
/// `NiSprintf` (`00a305b0`, `cdecl`: buffer, size, format, then the format's arguments).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `NiGetViewerString` of a class name (`00a7d330`, `cdecl`, one word).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
/// `0096ad30` (`thiscall`, the new size) grows an `NiTArray`; `0096ae90` (`thiscall`, index
/// and the address of the element) stores an element.
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The first word of `NiVertexColorProperty`'s `NiRTTI` (its class name).
const VERTEX_COLOR_PROPERTY_RTTI_NAME: u32 = 0x011f_4430;
/// `"%s = UNKNOWN!!!"`: the format both enum viewer strings use for a value they do not name.
const FORMAT_UNKNOWN: u32 = 0x0109_b2ec;
/// `"%s = SOURCE_IGNORE"`, `"%s = SOURCE_EMISSIVE"`, `"%s = SOURCE_AMB_DIFF"`.
const FORMAT_SOURCE_IGNORE: u32 = 0x0109_b6c0;
const FORMAT_SOURCE_EMISSIVE: u32 = 0x0109_b6d4;
const FORMAT_SOURCE_AMB_DIFF: u32 = 0x0109_b6ec;
/// `"%s = LIGHTING_E"`, `"%s = LIGHTING_E_A_D"`.
const FORMAT_LIGHTING_E: u32 = 0x0109_b704;
const FORMAT_LIGHTING_E_A_D: u32 = 0x0109_b714;
/// The names `GetViewerStrings` passes: `"Source Mode"`, `"Lighting Mode"`.
const NAME_SOURCE_MODE: u32 = 0x0109_b7d0;
const NAME_LIGHTING_MODE: u32 = 0x0109_b7c0;

/// A new vertex colour property (0x1c bytes, `operator new`, the base constructor, the
/// vtable, flags 8); null when the allocation fails.
fn new_vertex_color_property(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, NI_VERTEX_COLOR_PROPERTY_VTABLE);
    e.mem.set_u16(block + 0x18, 8);
    block
}

// Translated from 00a5f090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVertexColorProperty::IsEqual` (Xbox PDB): the property base's comparison
/// (`00a62140`), then the flag word's (+0x18) source mode (bits 4-5) and lighting mode
/// (bit 3). True only when all match.
pub fn ni_vertex_color_property_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(PROPERTY_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let differ = e.mem.u8(other.addr() + 0x18) ^ e.mem.u8(this.addr() + 0x18);
    if differ & 0x30 != 0 {
        return false;
    }
    differ & 8 == 0
}

/// The text of an enum viewer string: `strlen(name) + 0x1a` bytes from `00aa1070`, written by
/// `NiSprintf` with `format` and `name`. Answers the text.
fn enum_viewer_string(e: &mut Engine, name: Ptr, format: u32) -> Ptr {
    let size = (e.mem.cstr(name.addr()).len() as u32).wrapping_add(0x1a);
    let text = e.call(ALLOCATE_TEXT, &args![size]).u32();
    e.call(NI_SPRINTF, &args![text, size, format, name]);
    Ptr::new(text)
}

// Translated from 00a5f0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVertexColorProperty::GetViewerString` (Xbox PDB), `cdecl`: `"<name> = SOURCE_IGNORE"`
/// for source mode 0, `SOURCE_EMISSIVE` for 1, `SOURCE_AMB_DIFF` for 2, `"<name> =
/// UNKNOWN!!!"` for any other value. The text is `strlen(name) + 0x1a` bytes from
/// `00aa1070`.
pub fn ni_vertex_color_property_get_viewer_string(e: &mut Engine, name: Ptr, mode: u32) -> Ptr {
    let format = match mode {
        0 => FORMAT_SOURCE_IGNORE,
        1 => FORMAT_SOURCE_EMISSIVE,
        2 => FORMAT_SOURCE_AMB_DIFF,
        _ => FORMAT_UNKNOWN,
    };
    enum_viewer_string(e, name, format)
}

// Translated from 00a5f160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVertexColorProperty::GetViewerString_ov2` (Xbox PDB), `cdecl`: `"<name> =
/// LIGHTING_E"` for lighting mode 0, `LIGHTING_E_A_D` for 1, `"<name> = UNKNOWN!!!"` for any
/// other value.
pub fn ni_vertex_color_property_get_viewer_string_ov2(e: &mut Engine, name: Ptr, mode: u32) -> Ptr {
    let format = match mode {
        0 => FORMAT_LIGHTING_E,
        1 => FORMAT_LIGHTING_E_A_D,
        _ => FORMAT_UNKNOWN,
    };
    enum_viewer_string(e, name, format)
}

// Translated from 00a5f210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVertexColorProperty::CreateClone` (Xbox PDB): a property without controllers
/// (`NiObjectNET` +0xc null) is its own clone (recorded by `00a5d450`); otherwise a new
/// vertex colour property (null when the allocation fails) given the base's members
/// (`00a5d7a0`) and this one's flags (+0x18). Answers the clone.
pub fn ni_vertex_color_property_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning_process: Ptr,
) -> Ptr {
    if e.mem.u32(this.addr() + 0xc) == 0 {
        e.call(OBJECT_RECORD_CLONE, &args![this, this, cloning_process]);
        return this;
    }
    let clone = new_vertex_color_property(e);
    e.call(PROPERTY_COPY_MEMBERS, &args![this, clone, cloning_process]);
    let flags = e.mem.u16(this.addr() + 0x18);
    e.mem.set_u16(clone.wrapping_add(0x18), flags);
    Ptr::new(clone)
}

// Translated from 00a5f280 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiVertexColorProperty`): a new vertex colour property (flags 8), or null.
pub fn fn_00a5f280(e: &mut Engine) -> Ptr {
    Ptr::new(new_vertex_color_property(e))
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

// Translated from 00a5f2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVertexColorProperty::LoadBinary` (Xbox PDB): `NiProperty::LoadBinary`; for files older
/// than version `0x0a000102` (`NiStream` +0xd8) the flags (+0x18) are the stream's word at
/// +0x288 cut to its low 3 bits, newer ones read them (2 bytes). Files older than
/// `0x14010002` then read two 4-byte words into the local word (which starts as `this`): the
/// first shifted left by 4 replaces bits 4-5 of the flags (`(flags & 0xffcf) | word << 4`,
/// 16 bits, so higher bits of the word land in the flags too), the second shifted left by 3
/// replaces bit 3 the same way (`(flags & 0xfff7) | word << 3`). The stream argument's stack
/// word is the byte-count word.
pub fn ni_vertex_color_property_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PROPERTY_LOAD_BINARY, &args![this, stream]);
    let flags_at = this.addr() + 0x18;
    e.with_stack(8, |e, frame| {
        let word = frame.addr();
        let count = frame.addr() + 4;
        e.mem.set_u32(word, this.addr());
        if e.mem.u32(stream.addr() + 0xd8) < 0x0a00_0102 {
            let flags = e.mem.u16(stream.addr() + 0x288) & 7;
            e.mem.set_u16(flags_at, flags);
        } else {
            stream_read(e, stream, flags_at, 2, count);
        }
        if e.mem.u32(stream.addr() + 0xd8) < 0x1401_0002 {
            stream_read(e, stream, word, 4, count);
            let source = e.mem.u32(word) << 4;
            let flags = (e.mem.u16(flags_at) & 0xffcf) | source as u16;
            e.mem.set_u16(flags_at, flags);
            stream_read(e, stream, word, 4, count);
            let lighting = e.mem.u32(word).wrapping_mul(8);
            let flags = (e.mem.u16(flags_at) & 0xfff7) | lighting as u16;
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

// Translated from 00a5f3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVertexColorProperty::GetViewerStrings` (Xbox PDB): the property base's strings
/// (`00a5d850`), then three appended to `strings`: the class name (`NiGetViewerString` of
/// the first word of the `NiRTTI` at `011f4430`), `"Source Mode"` (flag bits 4-5,
/// [`ni_vertex_color_property_get_viewer_string`]) and `"Lighting Mode"` (bit 3,
/// [`ni_vertex_color_property_get_viewer_string_ov2`]). Each string pointer is kept in the
/// `strings` argument's stack word, whose address is passed.
pub fn ni_vertex_color_property_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.global::<u32>(VERTEX_COLOR_PROPERTY_RTTI_NAME);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        append_viewer_string(e, strings, slot);

        let flags = e.mem.u16(this.addr() + 0x18) as u32;
        let text = ni_vertex_color_property_get_viewer_string(
            e,
            Ptr::new(NAME_SOURCE_MODE),
            (flags >> 4) & 3,
        );
        e.mem.set_u32(slot, text.addr());
        append_viewer_string(e, strings, slot);

        let flags = e.mem.u16(this.addr() + 0x18) as u32;
        let text = ni_vertex_color_property_get_viewer_string_ov2(
            e,
            Ptr::new(NAME_LIGHTING_MODE),
            (flags >> 3) & 1,
        );
        e.mem.set_u32(slot, text.addr());
        append_viewer_string(e, strings, slot);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5_f090, ni_vertex_color_property_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a5_f0d0, ni_vertex_color_property_get_viewer_string(Ptr, u32) -> Ptr),
        entry!(
            0x00a5_f160,
            ni_vertex_color_property_get_viewer_string_ov2(Ptr, u32) -> Ptr
        ),
        entry!(0x00a5_f210, ni_vertex_color_property_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a5_f280, fn_00a5f280() -> Ptr),
        entry!(0x00a5_f2b0, ni_vertex_color_property_load_binary(Ptr, Ptr)),
        entry!(
            0x00a5_f3a0,
            ni_vertex_color_property_get_viewer_strings(Ptr, Ptr)
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
            NI_SPRINTF,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.register(ALLOCATE_TEXT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn is_equal_compares_source_and_lighting_modes() {
        let mut e = engine();
        e.register(PROPERTY_IS_EQUAL, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x1c);
        let other = e.mem.alloc(0x1c);
        e.mem.set_u16(this + 0x18, 0x38);
        e.mem.set_u16(other + 0x18, 0xffff ^ 0xc7);
        assert!(e.call(0x00a5_f090, &args![this, other]).bool());
        assert_eq!(calls_to(&e, PROPERTY_IS_EQUAL), vec![vec![this, other]]);
        for bit in [3, 4, 5] {
            e.mem.set_u16(other + 0x18, 0x38 ^ (1 << bit));
            assert!(!e.call(0x00a5_f090, &args![this, other]).bool());
        }
        e.mem.set_u16(other + 0x18, 0x38);
        e.register(PROPERTY_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a5_f090, &args![this, other]).bool());
    }

    #[test]
    fn mode_strings_pick_the_format_by_value() {
        let mut e = engine();
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"abc");
        for (function, mode, format) in [
            (0x00a5_f0d0u32, 0u32, FORMAT_SOURCE_IGNORE),
            (0x00a5_f0d0, 1, FORMAT_SOURCE_EMISSIVE),
            (0x00a5_f0d0, 2, FORMAT_SOURCE_AMB_DIFF),
            (0x00a5_f0d0, 3, FORMAT_UNKNOWN),
            (0x00a5_f160, 0, FORMAT_LIGHTING_E),
            (0x00a5_f160, 1, FORMAT_LIGHTING_E_A_D),
            (0x00a5_f160, 2, FORMAT_UNKNOWN),
        ] {
            e.call_log = Some(vec![]);
            let text = e.call(function, &args![name, mode]).u32();
            assert_eq!(calls_to(&e, ALLOCATE_TEXT), vec![vec![0x1d]]);
            assert_eq!(
                calls_to(&e, NI_SPRINTF),
                vec![vec![text, 0x1d, format, name]]
            );
        }
    }

    #[test]
    fn create_clone_shares_or_copies() {
        let mut e = engine();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u16(this + 0x18, 0x1234);
        assert_eq!(e.call(0x00a5_f210, &args![this, 0x66u32]).u32(), this);
        assert_eq!(
            calls_to(&e, OBJECT_RECORD_CLONE),
            vec![vec![this, this, 0x66]]
        );
        e.mem.set_u32(this + 0xc, 1);
        let clone = e.call(0x00a5_f210, &args![this, 0x66u32]).u32();
        assert_ne!(clone, this);
        assert_eq!(e.mem.u32(clone), NI_VERTEX_COLOR_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(clone + 0x18), 0x1234);
        assert_eq!(
            calls_to(&e, PROPERTY_COPY_MEMBERS),
            vec![vec![this, clone, 0x66]]
        );
    }

    #[test]
    fn create_object_builds_the_default_property() {
        let mut e = engine();
        let object = e.call(0x00a5_f280, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_VERTEX_COLOR_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(object + 0x18), 8);
        assert_eq!(calls_to(&e, OBJECT_BASE_CONSTRUCTOR), vec![vec![object]]);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a5_f280, &args![]).u32(), 0);
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
        // Newest files: the flags only.
        let (mut e, stream, input) = stream_engine(vec![0xabcd]);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(stream + 0xd8, 0x1401_0002);
        e.call(0x00a5_f2b0, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 0x18), 0xabcd);
        assert_eq!(e.mem.u32(input + 4), 2);
        // Middle versions: the flags, then the two words merged in.
        let (mut e, stream, input) = stream_engine(vec![0xffff, 2, 0]);
        e.mem.set_u32(stream + 0xd8, 0x1401_0001);
        e.call(0x00a5_f2b0, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 0x18), 0xffe7);
        assert_eq!(e.mem.u32(input + 4), 10);
        // Old files: the flags from the stream's word; a large word reaches higher bits.
        let (mut e, stream, _) = stream_engine(vec![0x100, 1]);
        e.mem.set_u32(stream + 0xd8, 0x0a00_0101);
        e.mem.set_u16(stream + 0x288, 0xffff);
        e.call(0x00a5_f2b0, &args![this, stream]);
        assert_eq!(e.mem.u16(this + 0x18), 0x100f);
    }

    #[test]
    fn viewer_strings_list_the_fields_in_order() {
        let mut e = engine();
        e.register(GET_VIEWER_STRING, |_, _| 0x1234_5600u32.into_ret());
        e.mem.set_u32(VERTEX_COLOR_PROPERTY_RTTI_NAME, 0x0109_b000);
        let mut stored = vec![];
        let this = e.mem.alloc(0x1c);
        e.mem.set_u16(this + 0x18, 0x18);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xa, 4);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xe, 2);
        e.call(0x00a5_f3a0, &args![this, strings]);
        assert_eq!(
            calls_to(&e, PROPERTY_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x0109_b000]]);
        let formats: Vec<_> = calls_to(&e, NI_SPRINTF)
            .iter()
            .map(|w| (w[2], w[3]))
            .collect();
        assert_eq!(
            formats,
            vec![
                (FORMAT_SOURCE_EMISSIVE, NAME_SOURCE_MODE),
                (FORMAT_LIGHTING_E_A_D, NAME_LIGHTING_MODE)
            ]
        );
        for set in calls_to(&e, ARRAY_SET_AT) {
            assert_eq!((set[0], set[1]), (strings, 4));
            stored.push(set[2]);
        }
        assert_eq!(stored.len(), 3);
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![strings, 6]; 3]);
    }
}
