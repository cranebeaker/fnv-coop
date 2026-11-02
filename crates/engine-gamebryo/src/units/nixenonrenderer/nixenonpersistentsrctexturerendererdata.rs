//! `NiXenonRenderer/nixenonpersistentsrctexturerendererdata.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PC build's class is `NiDX9PersistentSrcTextureRendererData` (RTTI of its vtable
//! `010f099c`); its virtuals here mostly pass on to the base class
//! `NiPersistentSrcTextureRendererData` (unit `NiMain/nipersistentsrctexturerendererdata.cpp`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// `NiPersistentSrcTextureRendererData::LinkObject` (`00a98110`, `thiscall`, one stack word).
const BASE_LINK_OBJECT: u32 = 0x00a9_8110;
/// `NiPersistentSrcTextureRendererData::RegisterStreamables` (`00a98170`, `thiscall`, one
/// stack word, a `bool` in AL).
const BASE_REGISTER_STREAMABLES: u32 = 0x00a9_8170;
/// `NiPersistentSrcTextureRendererData::SaveBinary` (`00a984b0`, `thiscall`, one stack word).
const BASE_SAVE_BINARY: u32 = 0x00a9_84b0;
/// `NiPersistentSrcTextureRendererData::GetViewerStrings` (`00a98740`, `thiscall`, one stack
/// word).
const BASE_GET_VIEWER_STRINGS: u32 = 0x00a9_8740;
/// The base class's destructor body (`00a98240`, `thiscall`, no stack words).
const BASE_DESTRUCTOR: u32 = 0x00a9_8240;
/// `NiGetViewerString` (`00a7d330`, `cdecl`, one word: the text) and the two
/// `NiTPrimitiveArray<char*>` methods used to append its result: the resize (`0096ad30`,
/// `thiscall`, one word: the new size) and the store (`0096ae90`, `thiscall`, two words: the
/// index and the address of the element).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// Gamebryo's sized object release (`00aa1460`, `cdecl`: object, size).
const NI_DELETE: u32 = 0x00aa_1460;
/// `NiDX9PersistentSrcTextureRendererData`'s vtable (`010f099c`, RTTI).
const RENDERER_DATA_VTABLE: u32 = 0x010f_099c;
/// The record `GetRTTI` returns (`0126f9f8`); its first word is the text given to
/// `NiGetViewerString`.
const RENDERER_DATA_RTTI: u32 = 0x0126_f9f8;
/// The record `GetStreamableRTTI` returns (`011f5f04`).
const STREAMABLE_RTTI: u32 = 0x011f_5f04;

// Translated from 00e91700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable `010f099c` +0x50, `LinkObject`'s slot): a jump to
/// `NiPersistentSrcTextureRendererData::LinkObject` (`00a98110`) with the same `this` and
/// stream.
pub fn fn_00e91700(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
}

// Translated from 00e91710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable `010f099c` +0x54, `RegisterStreamables`' slot): the base
/// class's `RegisterStreamables` (`00a98170`) on the same `this` and stream, its byte result
/// made a `bool`.
pub fn fn_00e91710(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(BASE_REGISTER_STREAMABLES, &args![this, stream]).u8() != 0
}

// Translated from 00e91730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable `010f099c` +0x58, `SaveBinary`'s slot): a jump to
/// `NiPersistentSrcTextureRendererData::SaveBinary` (`00a984b0`) with the same `this` and
/// stream.
pub fn fn_00e91730(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
}

// Translated from 00e91740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable `010f099c` +8, `GetRTTI`'s slot): the address of the
/// record at `0126f9f8`.
pub fn fn_00e91740(_e: &mut Engine) -> Ptr {
    Ptr::new(RENDERER_DATA_RTTI)
}

// Translated from 00e91750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiDX9PersistentSrcTextureRendererData`'s scalar deleting
/// destructor (vtable `010f099c` +0): stores the vtable, runs the base destructor body
/// (`00a98240`) and, when `flags & 1`, frees the 0x94 bytes. Returns `this`.
pub fn fn_00e91750(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), RENDERER_DATA_VTABLE);
    e.call(BASE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x94u32]);
    }
    this
}

// Translated from 00e91780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonPersistentSrcTextureRendererData::GetStreamableRTTI` (Xbox PDB): the address of the
/// record at `011f5f04`.
pub fn ni_xenon_persistent_src_texture_renderer_data_get_streamable_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(STREAMABLE_RTTI)
}

// Translated from 00e91a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonPersistentSrcTextureRendererData::GetViewerStrings` (Xbox PDB): the base class's
/// strings (`00a98740`), then `NiGetViewerString` of the class name (the first word of
/// `0126f9f8`) appended to `strings`: when the size (`m_usSize`) has reached the capacity
/// (`m_usMaxSize`) the array is first resized to the size plus `m_usGrowBy`, then the string
/// is stored at the old size.
pub fn ni_xenon_persistent_src_texture_renderer_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr<NiTArray>,
) {
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(RENDERER_DATA_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    let size = u32::from(e.get(strings, NiTArray::m_usSize));
    let max_size = u32::from(e.get(strings, NiTArray::m_usMaxSize));
    if size >= max_size {
        let grow_by = u32::from(e.get(strings, NiTArray::m_usGrowBy));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![strings, size, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_1700, fn_00e91700(Ptr, Ptr)),
        entry!(0x00e9_1710, fn_00e91710(Ptr, Ptr) -> bool),
        entry!(0x00e9_1730, fn_00e91730(Ptr, Ptr)),
        entry!(0x00e9_1740, fn_00e91740() -> Ptr),
        entry!(0x00e9_1750, fn_00e91750(Ptr, u32) -> Ptr),
        entry!(
            0x00e9_1780,
            ni_xenon_persistent_src_texture_renderer_data_get_streamable_rtti() -> Ptr
        ),
        entry!(
            0x00e9_1a90,
            ni_xenon_persistent_src_texture_renderer_data_get_viewer_strings(Ptr, Ptr<NiTArray>)
        ),
    ]
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn fn_00e91700_passes_on_to_the_base_link_object() {
        let mut e = Engine::new();
        e.register(BASE_LINK_OBJECT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00e9_1700, &args![0x0e10_0000u32, 0x0e20_0000u32]);
        assert_eq!(
            calls_to(&e, BASE_LINK_OBJECT),
            vec![vec![0x0e10_0000, 0x0e20_0000]]
        );
    }

    #[test]
    fn fn_00e91710_makes_the_base_result_a_bool() {
        let mut e = Engine::new();
        // Only AL counts: 0x100 is false, 0x2 is true.
        e.register(BASE_REGISTER_STREAMABLES, |e, a| e.mem.u32(a[1]).into_ret());
        let stream = e.mem.alloc(4);
        e.mem.set_u32(stream, 0x100);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_1710, &args![0x0e10_0000u32, stream]).bool());
        assert_eq!(
            calls_to(&e, BASE_REGISTER_STREAMABLES),
            vec![vec![0x0e10_0000, stream]]
        );
        e.mem.set_u32(stream, 2);
        assert_eq!(
            e.call(0x00e9_1710, &args![0x0e10_0000u32, stream]).u32() & 0xff,
            1
        );
    }

    #[test]
    fn fn_00e91730_passes_on_to_the_base_save_binary() {
        let mut e = Engine::new();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00e9_1730, &args![0x0e10_0000u32, 0x0e20_0000u32]);
        assert_eq!(
            calls_to(&e, BASE_SAVE_BINARY),
            vec![vec![0x0e10_0000, 0x0e20_0000]]
        );
    }

    #[test]
    fn fn_00e91740_returns_the_rtti_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e9_1740, &args![]).u32(), RENDERER_DATA_RTTI);
    }

    #[test]
    fn fn_00e91750_destroys_and_frees_with_bit_zero() {
        let mut e = Engine::new();
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        e.register(NI_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x94);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_1750, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), RENDERER_DATA_VTABLE);
        assert_eq!(calls_to(&e, BASE_DESTRUCTOR), vec![vec![this]]);
        assert!(calls_to(&e, NI_DELETE).is_empty());
        e.call(0x00e9_1750, &args![this, 1u32]);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this, 0x94]]);
    }

    #[test]
    fn get_streamable_rtti_returns_the_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e9_1780, &args![]).u32(), STREAMABLE_RTTI);
    }

    fn viewer_strings_engine(size: u16, max_size: u16) -> (Engine, Ptr<NiTArray>) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(RENDERER_DATA_RTTI, 0x0e30_0000);
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        // The store double records the element it is given in place of the index.
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let element = e.mem.u32(a[2]);
            e.mem.set_u32(a[0], element);
            Ret::default()
        });
        let strings = e.new_object::<NiTArray>();
        e.set(strings, NiTArray::m_usSize, size);
        e.set(strings, NiTArray::m_usMaxSize, max_size);
        e.set(strings, NiTArray::m_usGrowBy, 4);
        e.call_log = Some(vec![]);
        (e, strings)
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name() {
        let (mut e, strings) = viewer_strings_engine(2, 5);
        e.call(0x00e9_1a90, &args![0x0e10_0000u32, strings]);
        assert_eq!(
            calls_to(&e, BASE_GET_VIEWER_STRINGS),
            vec![vec![0x0e10_0000, strings.addr()]]
        );
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x0e30_0000]]);
        assert!(calls_to(&e, STRING_ARRAY_SET_SIZE).is_empty());
        let stores = calls_to(&e, STRING_ARRAY_SET_AT);
        assert_eq!(stores.len(), 1);
        assert_eq!(&stores[0][..2], &[strings.addr(), 2]);
        assert_eq!(e.mem.u32(strings.addr()), 0x0e30_0001);
    }

    #[test]
    fn get_viewer_strings_grows_a_full_array_first() {
        let (mut e, strings) = viewer_strings_engine(5, 5);
        e.call(0x00e9_1a90, &args![0x0e10_0000u32, strings]);
        assert_eq!(
            calls_to(&e, STRING_ARRAY_SET_SIZE),
            vec![vec![strings.addr(), 9]]
        );
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_AT)[0][1], 5);
    }
}
