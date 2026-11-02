//! `NiMain/niditherproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiDitherProperty::ms_RTTI` (the class's `NiRTTI` record).
const NI_DITHER_PROPERTY_RTTI: u32 = 0x011f_5f48;

// Translated from 00a9a5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDitherProperty::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_dither_property_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_DITHER_PROPERTY_RTTI)
}

/// `NiProperty::LoadBinary` (`00a5d7f0`, `RET 4`).
const NI_PROPERTY_LOAD_BINARY: u32 = 0x00a5_d7f0;
/// `NiStream::m_uiNifFileVersion` (Xbox PDB) +0xD8.
const STREAM_FILE_VERSION: u32 = 0xd8;
/// `NiStream::m_pkIstr` (Xbox PDB) +0x24C: the `NiBinaryStream*` read from.
const STREAM_INPUT: u32 = 0x24c;
/// `NiStream::m_usNiPropertyFlags` (Xbox PDB) +0x288.
const STREAM_PROPERTY_FLAGS: u32 = 0x288;
/// `NiBinaryStream::m_uiAbsoluteCurrentPos` (Xbox PDB) +4.
const BINARY_STREAM_POSITION: u32 = 0x4;
/// `NiBinaryStream::m_pfnRead` (Xbox PDB) +8: a `cdecl` function pointer
/// `(stream, buffer, size, &component size, component count)` returning the
/// number of bytes read.
const BINARY_STREAM_READ: u32 = 0x8;
/// The first file version whose dither property stores its own flags.
const OWN_FLAGS_VERSION: u32 = 0x0a00_0102;
/// `NiDitherProperty::m_uFlags` (Xbox PDB) +0x18.
const DITHER_FLAGS: u32 = 0x18;

// Translated from 00a95190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDitherProperty::LoadBinary` (Xbox PDB): `NiProperty::LoadBinary`, then
/// the flags at +0x18: for files before version `0x0A000102` bit 0 of the
/// stream's `m_usNiPropertyFlags` (+0x288); otherwise 2 bytes read into them
/// through the input stream's `m_pfnRead` (`(istr, this + 0x18, 2,
/// &component size = 2, 1)`; the component size is the stream argument's
/// stack word, overwritten with 2), the stream's position advanced by the
/// bytes read.
pub fn ni_dither_property_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_PROPERTY_LOAD_BINARY, &args![this, stream]);
    if e.mem.u32(stream.addr().wrapping_add(STREAM_FILE_VERSION)) < OWN_FLAGS_VERSION {
        let flags = e.mem.u16(stream.addr().wrapping_add(STREAM_PROPERTY_FLAGS)) & 1;
        e.mem.set_u16(this.addr().wrapping_add(DITHER_FLAGS), flags);
        return;
    }
    let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    e.with_stack(4, |e, component_size| {
        e.mem.set_u32(component_size.addr(), 2);
        let got = e
            .call(
                read,
                &args![
                    input,
                    this.addr().wrapping_add(DITHER_FLAGS),
                    2u32,
                    component_size,
                    1u32
                ],
            )
            .u32();
        let position = input.wrapping_add(BINARY_STREAM_POSITION);
        let advanced = e.mem.u32(position).wrapping_add(got);
        e.mem.set_u32(position, advanced);
    });
}

/// `NiDitherProperty`'s vtable.
const NI_DITHER_PROPERTY_VTABLE: u32 = 0x010a_0f6c;
/// `operator new` (`cdecl`, the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `00a5c200` (unit `niobjectnet.cpp`, `thiscall`, no stack words): the base constructor
/// the property's construction starts with.
const OBJECT_BASE_CONSTRUCT: u32 = 0x00a5_c200;
/// `00a5d7a0` (unnamed in the engine map; `thiscall`: the clone and the cloning process):
/// the property base's member copy that `CreateClone` runs.
const PROPERTY_COPY_MEMBERS: u32 = 0x00a5_d7a0;
/// `NiProperty::GetViewerStrings` (`00a5d850`, `thiscall`, the string array).
const PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00a5_d850;
/// `NiGetViewerString` (`00a7d330`, the class name) and `NiGetViewerString_ov2`
/// (`00a7d370`, a name and a bool word, of which it reads the low byte), both `cdecl`,
/// answering a new string.
const VIEWER_STRING_CLASS: u32 = 0x00a7_d330;
const VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
/// The string array's grow (`0096ad30`, `thiscall`, the new size) and set-at (`0096ae90`,
/// `thiscall`, (index, address of the string)), both unnamed in the engine map.
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The literal `"m_bDither"`.
const TEXT_M_B_DITHER: u32 = 0x010a_1000;

/// Appends `text` to a viewer-string array as the exe's inlined `Add` does: when the 16-bit
/// size (+0xa) has reached the capacity (+0x8) the array grows to its step (+0xe) plus the
/// size, then the string is stored at the old size (through a stack word holding it).
fn push_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// Allocates 0x1c bytes and builds an `NiDitherProperty` in them (the base constructor
/// `00a5c200`, the vtable `010a0f6c`, flags 0); null when the allocation fails.
fn new_dither_property(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(OBJECT_BASE_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, NI_DITHER_PROPERTY_VTABLE);
    e.mem.set_u16(block + DITHER_FLAGS, 0);
    block
}

// Translated from 00a9a5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDitherProperty::CreateClone` (Xbox PDB): a new dither property (null when the
/// allocation fails), the property member copy (`00a5d7a0`) into it, then this one's flags
/// (+0x18) copied (written at 0x18 for a null clone, as the exe does). Answers the clone.
pub fn ni_dither_property_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let clone = new_dither_property(e);
    e.call(PROPERTY_COPY_MEMBERS, &args![this, clone, cloning_process]);
    let flags = e.mem.u16(this.addr() + DITHER_FLAGS);
    e.mem.set_u16(clone.wrapping_add(DITHER_FLAGS), flags);
    Ptr::new(clone)
}

// Translated from 00a9a640 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiDitherProperty`): a new dither property with flags 0, or null.
pub fn fn_00a9a640(e: &mut Engine) -> Ptr {
    Ptr::new(new_dither_property(e))
}

// Translated from 00a9a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDitherProperty::GetViewerStrings` (Xbox PDB): `NiProperty::GetViewerStrings`, the
/// class name (`NiGetViewerString` of `ms_RTTI`'s name word), then `"m_bDither"` with bit 0
/// of the flags (+0x18). The exe builds that bool word in the stack word still holding the
/// class name's string, so its upper three bytes are that string's (the callee reads only
/// the low byte).
pub fn ni_dither_property_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(NI_DITHER_PROPERTY_RTTI);
    let text = e.call(VIEWER_STRING_CLASS, &args![name]).u32();
    push_viewer_string(e, strings, text);
    let dither = u32::from(e.mem.u8(this.addr() + DITHER_FLAGS) & 1);
    let word = (text & 0xffff_ff00) | dither;
    let text = e
        .call(VIEWER_STRING_BOOL, &args![TEXT_M_B_DITHER, word])
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9a5e0, ni_dither_property_get_rtti() -> Ptr),
        entry!(0x00a95190, ni_dither_property_load_binary(Ptr, Ptr)),
        entry!(
            0x00a9_a5f0,
            ni_dither_property_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00a9_a640, fn_00a9a640() -> Ptr),
        entry!(0x00a9_a670, ni_dither_property_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_dither_property_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a9a5e0, &args![]).u32(), NI_DITHER_PROPERTY_RTTI);
    }
}

#[cfg(test)]
mod b014_tests {
    use super::*;

    /// A stand-in `m_pfnRead` (at a made-up address): writes `0x1234` into
    /// the buffer and answers 2 bytes read.
    const FAKE_READ: u32 = 0x7100_0000;

    #[test]
    fn ni_dither_property_load_binary_takes_old_flags_or_reads_its_own() {
        for version in [0x0a00_0101u32, 0x0a00_0102] {
            let mut e = Engine::new();
            e.register(NI_PROPERTY_LOAD_BINARY, |_, _| Ret::default());
            e.register(FAKE_READ, |e, a| {
                assert_eq!(&a[2..], [2, a[3], 1]);
                assert_eq!(e.mem.u32(a[3]), 2);
                e.mem.set_u16(a[1], 0x1234);
                Ret {
                    eax: 2,
                    ..Ret::default()
                }
            });
            let this: Ptr = Ptr::new(e.mem.alloc(0x1c));
            let stream = e.mem.alloc(0x5c4);
            let input = e.mem.alloc(0x10);
            e.mem.set_u32(input + 4, 7);
            e.mem.set_u32(input + 8, FAKE_READ);
            e.mem.set_u32(stream + 0xd8, version);
            e.mem.set_u32(stream + 0x24c, input);
            e.mem.set_u16(stream + 0x288, 0xfff3);
            e.call_log = Some(vec![]);
            e.call(0x00a95190, &args![this, stream]);
            let log = e.call_log.take().unwrap();
            let parent = log.iter().find(|c| c.0 == NI_PROPERTY_LOAD_BINARY).unwrap();
            assert_eq!(parent.1, [this.addr(), stream]);
            if version < 0x0a00_0102 {
                assert_eq!(e.mem.u16(this.addr() + 0x18), 1);
                assert!(log.iter().all(|c| c.0 != FAKE_READ));
                assert_eq!(e.mem.u32(input + 4), 7);
            } else {
                assert_eq!(e.mem.u16(this.addr() + 0x18), 0x1234);
                let read = log.iter().find(|c| c.0 == FAKE_READ).unwrap();
                assert_eq!(&read.1[..2], [input, this.addr() + 0x18]);
                assert_eq!(e.mem.u32(input + 4), 9);
            }
        }
    }
}

#[cfg(test)]
mod n016_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xee; 0x1c]);
            block.into_ret()
        });
        for double in [
            OBJECT_BASE_CONSTRUCT,
            PROPERTY_COPY_MEMBERS,
            PROPERTY_GET_VIEWER_STRINGS,
            STRINGS_GROW,
            STRINGS_SET_AT,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn create_clone_copies_the_flags() {
        let mut e = engine();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u16(this + DITHER_FLAGS, 0x8001);
        let clone = e.call(0x00a9_a5f0, &args![this, 0x77u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_DITHER_PROPERTY_VTABLE);
        assert_eq!(calls_to(&e, OBJECT_BASE_CONSTRUCT), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, PROPERTY_COPY_MEMBERS),
            vec![vec![this, clone, 0x77]]
        );
        assert_eq!(e.mem.u16(clone + DITHER_FLAGS), 0x8001);
    }

    #[test]
    fn create_object_builds_an_empty_property() {
        let mut e = engine();
        let object = e.call(0x00a9_a640, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_DITHER_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(object + DITHER_FLAGS), 0);
        assert_eq!(e.mem.u32(object + 4), 0xeeee_eeee);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_a640, &args![]).u32(), 0);
    }

    #[test]
    fn get_viewer_strings_adds_the_dither_bit() {
        let mut e = engine();
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(NI_DITHER_PROPERTY_RTTI, 0x0123_0000);
        e.register(VIEWER_STRING_CLASS, |_, _| 0x0e00_1234u32.into_ret());
        e.register(VIEWER_STRING_BOOL, |_, _| 0x0e00_0002u32.into_ret());
        let this = e.mem.alloc(0x1c);
        e.mem.set_u16(this + DITHER_FLAGS, 3);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.call(0x00a9_a670, &args![this, strings]);
        assert_eq!(
            calls_to(&e, PROPERTY_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls_to(&e, VIEWER_STRING_CLASS), vec![vec![0x0123_0000]]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_BOOL),
            vec![vec![TEXT_M_B_DITHER, 0x0e00_1201]]
        );
        assert_eq!(calls_to(&e, STRINGS_SET_AT).len(), 2);
    }
}
