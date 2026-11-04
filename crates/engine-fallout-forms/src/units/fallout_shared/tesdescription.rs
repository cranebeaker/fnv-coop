//! `fallout shared/tesdescription.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00482f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0x620`. The engine map has no name for it; the
/// `TESForm` constructor (`00483370`) calls it on the data handler
/// (`011c3f2c`) and assigns a new form ID only when it answers 0.
pub fn fn_00482f20(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x620)
}

/// `TESDescription::cDescription` (Xbox PDB static, `011c5498`, a
/// `BSStringT`): the shared description text; `pCurrentDescription`
/// (`011c5490`): the description that last filled it.
const DESCRIPTION_TEXT: u32 = 0x011c_5498;
const DESCRIPTION_OWNER: u32 = 0x011c_5490;
/// `BSStringT::Set` (`004037f0`, `thiscall (string, text, length)`), the
/// string's length (`004048e0`, `thiscall`) and its text (`00559450`,
/// `thiscall`: the word at the string).
const STRING_SET: u32 = 0x0040_37f0;
const STRING_LENGTH: u32 = 0x0040_48e0;
const STRING_TEXT: u32 = 0x0055_9450;
/// The empty C string (`01011584`).
const EMPTY_STRING: u32 = 0x0101_1584;
/// `TESDescription::lFileOffset` (Xbox PDB) +4.
const DESCRIPTION_OFFSET: u32 = 0x4;
/// The data handler pointer (`011c3f2c`).
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl (object, offset, from, to,
/// reference)`) with the type descriptors it is given here (`011838f8`
/// from, `01183028` to).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_FROM: u32 = 0x0118_38f8;
const CAST_TO: u32 = 0x0118_3028;
/// The call `LoadDescriptionData` makes on its second argument (`00467bb0`,
/// `thiscall`) and the one it hands the answer to on its first (`006ecd40`,
/// `thiscall`, one word).
const DATA_SOURCE: u32 = 0x0046_7bb0;
const DATA_TARGET: u32 = 0x006e_cd40;
/// `TESForm::GetFile` (`00484e60`, `thiscall (form, index)`),
/// `TESFile::GetThreadSafeFile` (`004739b0`), `TESFile::SetOffset`
/// (`004723a0`, one word), `TESFile::GetTESForm` (`00472660`, answers a
/// byte), the form's type (`00401170`), `TESFile::GetTESChunk` (`004726b0`),
/// the chunk's size (`00401660`), `TESFile::GetChunkData_ov3` (`00472890`,
/// two words) and the next chunk (`004726f0`, answers a byte).
const FORM_GET_FILE: u32 = 0x0048_4e60;
const FILE_GET_THREAD_SAFE: u32 = 0x0047_39b0;
const FILE_SET_OFFSET: u32 = 0x0047_23a0;
const FILE_GET_FORM_TYPE: u32 = 0x0047_2660;
const FORM_TYPE: u32 = 0x0040_1170;
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_GET_CHUNK_DATA: u32 = 0x0047_2890;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
/// The form type table (`01187000`, 0xC bytes per type; its first byte the
/// record type the file must hold).
const FORM_TYPE_TABLE: u32 = 0x0118_7000;

// Translated from 00482c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDescription::InitializeDataComponent` (Xbox PDB): empties the shared
/// description text (`BSStringT::Set(0, 0)`), forgets its owner and clears
/// the description's offset (+4).
pub fn tes_description_initialize_data_component(e: &mut Engine, this: Ptr) {
    e.call(STRING_SET, &args![DESCRIPTION_TEXT, 0u32, 0u32]);
    e.mem.set_u32(DESCRIPTION_OWNER, 0);
    e.mem
        .set_u32(this.addr().wrapping_add(DESCRIPTION_OFFSET), 0);
}

// Translated from 00482d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDescription::LoadDescriptionData` (Xbox PDB, `cdecl`): when both are
/// non-null, gives `first`'s `006ecd40` the answer of `second`'s `00467bb0`.
pub fn tes_description_load_description_data(e: &mut Engine, first: Ptr, second: Ptr) {
    if first.is_null() || second.is_null() {
        return;
    }
    let value = e.call(DATA_SOURCE, &args![second]).u32();
    e.call(DATA_TARGET, &args![first, value]);
}

// Translated from 00483080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): sets the shared
/// description text to `text` (`BSStringT::Set(text, 0)`).
pub fn fn_00483080(e: &mut Engine, _this: Ptr, text: Ptr) {
    e.call(STRING_SET, &args![DESCRIPTION_TEXT, text, 0u32]);
}

// Translated from 00482f40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00482f40 - the /GS cookie check (00ec408c) is not translated; the chunk buffer from _RtlCheckStack12 (00ec5ec0, the _alloca) is a with_stack block of the chunk size (its alignment is not modelled)
/// `TESDescription::LoadDescription` (Xbox PDB): with a form, unless this
/// description already fills the shared text with something, takes the
/// text over (empties it), finds the form's file (`GetFile(-1)`, its
/// thread-safe copy) and, when the description has an offset, seeks there
/// (`SetOffset`); when the record is of the form's type (the type table's
/// byte), walks its chunks for `chunk` and copies that chunk's data
/// (`GetChunkData`, into a stack buffer of the chunk's size) into the
/// shared text.
pub fn tes_description_load_description(e: &mut Engine, this: Ptr, form: Ptr, chunk: u32) {
    if form.is_null() {
        return;
    }
    if e.mem.u32(DESCRIPTION_OWNER) == this.addr()
        && e.call(STRING_LENGTH, &args![DESCRIPTION_TEXT]).u32() != 0
    {
        return;
    }
    e.mem.set_u32(DESCRIPTION_OWNER, this.addr());
    e.call(STRING_SET, &args![DESCRIPTION_TEXT, 0u32, 0u32]);
    let file = e.call(FORM_GET_FILE, &args![form, 0xffff_ffffu32]).u32();
    if file == 0 || e.mem.u32(this.addr().wrapping_add(DESCRIPTION_OFFSET)) == 0 {
        return;
    }
    let file = e.call(FILE_GET_THREAD_SAFE, &args![file]).u32();
    let offset = e.mem.u32(this.addr().wrapping_add(DESCRIPTION_OFFSET));
    if !e.call(FILE_SET_OFFSET, &args![file, offset]).bool() {
        return;
    }
    let record_type = e.call(FILE_GET_FORM_TYPE, &args![file]).u8();
    let form_type = e.call(FORM_TYPE, &args![form]).u32();
    let expected = e
        .mem
        .u8(FORM_TYPE_TABLE.wrapping_add(form_type.wrapping_mul(0xc)));
    if record_type != expected {
        return;
    }
    loop {
        let mut found = false;
        if e.call(FILE_GET_CHUNK, &args![file]).u32() == chunk {
            let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
            e.with_stack(size, |e, buffer| {
                e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0u32]);
                fn_00483080(e, this, buffer);
            });
            found = true;
        }
        if found || !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            return;
        }
    }
}

// Translated from 00482e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDescription::GetDescription` (Xbox PDB): unless the data handler's
/// byte at +0x620 is set (`fn_00482f20`), loads the description of `form`
/// (when null, this object cast to the form class by `__RTDynamicCast`)
/// from `chunk`; answers the shared text (`""` when it has none).
pub fn tes_description_get_description(e: &mut Engine, this: Ptr, form: Ptr, chunk: u32) -> Ptr {
    let handler = Ptr::new(e.mem.u32(DATA_HANDLER));
    if fn_00482f20(e, handler) == 0 {
        let mut source = form;
        if source.is_null() {
            source = e
                .call(DYNAMIC_CAST, &args![this, 0u32, CAST_FROM, CAST_TO, 0u32])
                .ptr();
        }
        tes_description_load_description(e, this, source, chunk);
    }
    if e.call(STRING_TEXT, &args![DESCRIPTION_TEXT]).u32() != 0 {
        e.call(STRING_TEXT, &args![DESCRIPTION_TEXT]).ptr()
    } else {
        Ptr::new(EMPTY_STRING)
    }
}

/// The `DESC` chunk (`0x43534544`).
const CHUNK_DESC: u32 = 0x4353_4544;
/// `TESForm::AddChunkArray` (`004856f0`, `cdecl (chunk, data, size)`).
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// The type descriptor of `BaseFormComponent` (`01183040`, its name read at
/// +8); `CAST_FROM` is `TESDescription`'s.
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
/// `BSStringT::StrCmp` (`00408a80`, `thiscall (string, text, flag)`; flag 0
/// compares with `00408b20`).
const STRING_COMPARE: u32 = 0x0040_8a80;
/// The getter `CopyComponent` uses for the other description's offset
/// (`00726070`, `thiscall`, no name confirmed).
const OTHER_OFFSET: u32 = 0x0072_6070;

// Translated from 00482d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no arguments): the length of the
/// shared description text (`004048e0` on `011c5498`); `this` is not read.
pub fn fn_00482d30(e: &mut Engine, _this: Ptr) -> u32 {
    e.call(STRING_LENGTH, &args![DESCRIPTION_TEXT]).u32()
}

// Translated from 00482cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDescription::SaveDescriptionData` (Xbox PDB, `cdecl (description,
/// chunk)`): with a description, adds `chunk` holding its text and the
/// terminating zero (virtual slot `+0x10` with form 0 and `DESC`, length
/// `fn_00482d30 + 1`), or a single zero byte when the shared text is empty.
pub fn tes_description_save_description_data(e: &mut Engine, description: Ptr, chunk: u32) {
    if description.is_null() {
        return;
    }
    if fn_00482d30(e, description) != 0 {
        let size = fn_00482d30(e, description).wrapping_add(1);
        let text = e
            .vcall(description.addr(), 0x10, &args![0u32, CHUNK_DESC])
            .u32();
        e.call(ADD_CHUNK_ARRAY, &args![chunk, text, size]);
    } else {
        e.with_stack(4, |e, zero| {
            e.mem.set_u8(zero.addr(), 0);
            e.call(ADD_CHUNK_ARRAY, &args![chunk, zero, 1u32]);
        });
    }
}

// Translated from 00482cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDescription::Save` (Xbox PDB): `SaveDescriptionData(this, DESC)`.
pub fn tes_description_save(e: &mut Engine, this: Ptr) {
    tes_description_save_description_data(e, this, CHUNK_DESC);
}

// Translated from 00482d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDescription::CopyComponent` (Xbox PDB): when `other` casts from
/// `BaseFormComponent` to `TESDescription`, sets the shared text to its
/// text (slot `+0x10`, form 0, `DESC`) and takes its offset (`00726070`)
/// into +4.
pub fn tes_description_copy_component(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_BASE_FORM_COMPONENT, CAST_FROM, 0u32],
        )
        .u32();
    if source == 0 {
        return;
    }
    let text = e.vcall(source, 0x10, &args![0u32, CHUNK_DESC]).u32();
    e.call(STRING_SET, &args![DESCRIPTION_TEXT, text, 0u32]);
    let offset = e.call(OTHER_OFFSET, &args![source]).u32();
    e.mem
        .set_u32(this.addr().wrapping_add(DESCRIPTION_OFFSET), offset);
}

// Translated from 00482df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDescription::CompareComponent` (Xbox PDB): true (different) when
/// `other` is no `TESDescription`; false when both lengths (`fn_00482d30`)
/// are 0; otherwise true when the lengths differ or `StrCmp` of the shared
/// text with `other`'s text (slot `+0x10`, form 0, `DESC`) is non-zero.
pub fn tes_description_compare_component(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let source = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_BASE_FORM_COMPONENT, CAST_FROM, 0u32],
        )
        .ptr();
    if source.is_null() {
        return true;
    }
    if fn_00482d30(e, this) == 0 && fn_00482d30(e, source) == 0 {
        return false;
    }
    let own = fn_00482d30(e, this);
    if own != fn_00482d30(e, source) {
        return true;
    }
    let text = e.vcall(source.addr(), 0x10, &args![0u32, CHUNK_DESC]).u32();
    e.call(STRING_COMPARE, &args![DESCRIPTION_TEXT, text, 0u32])
        .u32()
        != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00482f20, fn_00482f20(Ptr) -> u8),
        entry!(0x00482c80, tes_description_initialize_data_component(Ptr)),
        entry!(0x00482d50, tes_description_load_description_data(Ptr, Ptr)),
        entry!(0x00483080, fn_00483080(Ptr, Ptr)),
        entry!(0x00482f40, tes_description_load_description(Ptr, Ptr, u32)),
        entry!(0x00482e90, tes_description_get_description(Ptr, Ptr, u32) -> Ptr),
        entry!(0x0048_2d30, fn_00482d30(Ptr) -> u32),
        entry!(0x0048_2cd0, tes_description_save_description_data(Ptr, u32)),
        entry!(0x0048_2cb0, tes_description_save(Ptr)),
        entry!(0x0048_2d80, tes_description_copy_component(Ptr, Ptr)),
        entry!(0x0048_2df0, tes_description_compare_component(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_byte_at_0x620() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x624);
        e.mem.set_u32(object + 0x620, 0x1122_3301);
        assert_eq!(e.call(0x0048_2f20, &[object]).u32() & 0xff, 1);
        e.mem.set_u8(object + 0x620, 0);
        assert_eq!(e.call(0x0048_2f20, &[object]).u32() & 0xff, 0);
    }
}

#[cfg(test)]
mod b014_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
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
        e.map(0x011c_3000, 0x3000);
        e.map(0x0118_7000, 0x1000);
        e.register(STRING_SET, |_, _| Ret::default());
        e
    }

    #[test]
    fn initialize_data_component_empties_the_shared_text() {
        let mut e = engine();
        e.mem.set_u32(DESCRIPTION_OWNER, 0x1234);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 99);
        e.call_log = Some(vec![]);
        e.call(0x00482c80, &args![this]);
        assert_eq!(calls(&e, STRING_SET), vec![vec![DESCRIPTION_TEXT, 0, 0]]);
        assert_eq!(e.mem.u32(DESCRIPTION_OWNER), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    #[test]
    fn load_description_data_needs_both() {
        let mut e = engine();
        e.register(DATA_SOURCE, |_, a| (a[0] + 1).into_ret());
        e.register(DATA_TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00482d50, &args![0u32, 0x2000u32]);
        e.call(0x00482d50, &args![0x1000u32, 0u32]);
        assert!(calls(&e, DATA_TARGET).is_empty());
        e.call(0x00482d50, &args![0x1000u32, 0x2000u32]);
        assert_eq!(calls(&e, DATA_TARGET), vec![vec![0x1000, 0x2001]]);
    }

    #[test]
    fn set_text() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x00483080, &args![0x1000u32, 0x5555u32]);
        assert_eq!(
            calls(&e, STRING_SET),
            vec![vec![DESCRIPTION_TEXT, 0x5555, 0]]
        );
    }

    /// Doubles for the file walk: the form's file is `0x7000`, its chunks
    /// are 1, 2, 3 (the double's state at `0x7300_0000`), the record type 5
    /// and the form type 2.
    fn file_engine() -> Engine {
        let mut e = engine();
        e.map(0x7300_0000, 0x10);
        e.register(STRING_LENGTH, |_, _| 0u32.into_ret());
        e.register(FORM_GET_FILE, |_, _| 0x7000u32.into_ret());
        e.register(FILE_GET_THREAD_SAFE, |_, a| a[0].into_ret());
        e.register(FILE_SET_OFFSET, |_, _| true.into_ret());
        e.register(FILE_GET_FORM_TYPE, |_, _| 5u32.into_ret());
        e.register(FORM_TYPE, |_, _| 2u32.into_ret());
        e.register(FILE_GET_CHUNK, |e, _| {
            (e.mem.u32(0x7300_0000) + 1).into_ret()
        });
        e.register(FILE_NEXT_CHUNK, |e, _| {
            let n = e.mem.u32(0x7300_0000) + 1;
            e.mem.set_u32(0x7300_0000, n);
            (n < 3).into_ret()
        });
        e.register(FILE_CHUNK_SIZE, |_, _| 16u32.into_ret());
        e.register(FILE_GET_CHUNK_DATA, |e, a| {
            e.mem.set_cstr(a[1], b"text");
            Ret::default()
        });
        e.mem.set_u8(FORM_TYPE_TABLE + 2 * 0xc, 5);
        e
    }

    #[test]
    fn load_description_copies_the_matching_chunk() {
        let mut e = file_engine();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x40);
        e.call_log = Some(vec![]);
        e.call(0x00482f40, &args![this, 0x9000u32, 2u32]);
        assert_eq!(e.mem.u32(DESCRIPTION_OWNER), this);
        let sets = calls(&e, STRING_SET);
        assert_eq!(sets.len(), 2);
        assert_eq!(sets[1][2], 0);
        assert_eq!(calls(&e, FILE_SET_OFFSET), vec![vec![0x7000, 0x40]]);
        assert_eq!(calls(&e, FILE_GET_CHUNK_DATA).len(), 1);
        assert_eq!(calls(&e, FILE_NEXT_CHUNK).len(), 1);
    }

    #[test]
    fn load_description_stops_early() {
        // No form.
        let mut e = file_engine();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x40);
        e.call_log = Some(vec![]);
        e.call(0x00482f40, &args![this, 0u32, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // Already the owner with text.
        e.register(STRING_LENGTH, |_, _| 3u32.into_ret());
        e.mem.set_u32(DESCRIPTION_OWNER, this);
        e.call_log = Some(vec![]);
        e.call(0x00482f40, &args![this, 0x9000u32, 2u32]);
        assert!(calls(&e, STRING_SET).is_empty());
        // No offset.
        e.mem.set_u32(DESCRIPTION_OWNER, 0);
        e.mem.set_u32(this + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00482f40, &args![this, 0x9000u32, 2u32]);
        assert!(calls(&e, FILE_SET_OFFSET).is_empty());
        // The wrong record type.
        e.mem.set_u32(DESCRIPTION_OWNER, 0);
        e.mem.set_u32(this + 4, 0x40);
        e.mem.set_u8(FORM_TYPE_TABLE + 2 * 0xc, 6);
        e.call_log = Some(vec![]);
        e.call(0x00482f40, &args![this, 0x9000u32, 2u32]);
        assert!(calls(&e, FILE_GET_CHUNK).is_empty());
        assert_eq!(calls(&e, FILE_SET_OFFSET).len(), 1);
        // The chunk is never found: every chunk is looked at.
        e.mem.set_u32(DESCRIPTION_OWNER, 0);
        e.mem.set_u8(FORM_TYPE_TABLE + 2 * 0xc, 5);
        e.call_log = Some(vec![]);
        e.call(0x00482f40, &args![this, 0x9000u32, 7u32]);
        assert!(calls(&e, FILE_GET_CHUNK_DATA).is_empty());
        assert_eq!(calls(&e, FILE_GET_CHUNK).len(), 3);
    }

    #[test]
    fn get_description_loads_unless_the_handler_forbids_and_answers_the_text() {
        let mut e = engine();
        let handler = e.mem.alloc(0x624);
        e.mem.set_u32(DATA_HANDLER, handler);
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.register(STRING_TEXT, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        // No form: the cast; a null result loads nothing; no text: "".
        assert_eq!(
            e.call(0x00482e90, &args![this, 0u32, 1u32]).u32(),
            EMPTY_STRING
        );
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![this, 0, CAST_FROM, CAST_TO, 0]]
        );
        // The handler's byte set: nothing is loaded; the text is answered.
        e.mem.set_u8(handler + 0x620, 1);
        e.register(STRING_TEXT, |_, _| 0x6000u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00482e90, &args![this, 0x9000u32, 1u32]).u32(),
            0x6000
        );
        assert!(calls(&e, DYNAMIC_CAST).is_empty());
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    const TEXT_SLOT: u32 = 0x7100_0010;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The shared length is the word at `0x7300_0000`; descriptions answer
    /// `0x6000 + this` from slot `+0x10`.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x3000);
        e.map(0x7300_0000, 0x10);
        e.register(STRING_SET, |_, _| Ret::default());
        e.register(STRING_LENGTH, |e, _| e.mem.u32(0x7300_0000).into_ret());
        e.register(TEXT_SLOT, |_, a| (0x6000 + a[0]).into_ret());
        e.register(ADD_CHUNK_ARRAY, |e, a| {
            if a[2] == 1 {
                let first = e.mem.u8(a[1]);
                e.mem.set_u32(0x7300_0004, u32::from(first));
            }
            Ret::default()
        });
        e
    }

    fn description(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, TEXT_SLOT);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn length_of_the_shared_text() {
        let mut e = engine();
        e.mem.set_u32(0x7300_0000, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_2d30, &args![0x1234u32]).u32(), 7);
        assert_eq!(calls(&e, STRING_LENGTH), vec![vec![DESCRIPTION_TEXT]]);
    }

    #[test]
    fn save_writes_the_text_or_one_zero_byte() {
        let mut e = engine();
        let this = description(&mut e);
        e.mem.set_u32(0x7300_0000, 4);
        e.call_log = Some(vec![]);
        e.call(0x0048_2cb0, &args![this]);
        assert_eq!(
            calls(&e, ADD_CHUNK_ARRAY),
            vec![vec![CHUNK_DESC, 0x6000 + this, 5]]
        );
        assert_eq!(calls(&e, TEXT_SLOT), vec![vec![this, 0, CHUNK_DESC]]);
        // Empty text: one zero byte, under the given chunk.
        e.mem.set_u32(0x7300_0000, 0);
        e.mem.set_u32(0x7300_0004, 0xff);
        e.call_log = Some(vec![]);
        e.call(0x0048_2cd0, &args![this, 0x1122_3344u32]);
        let added = calls(&e, ADD_CHUNK_ARRAY);
        assert_eq!(added.len(), 1);
        assert_eq!((added[0][0], added[0][2]), (0x1122_3344, 1));
        assert_eq!(e.mem.u32(0x7300_0004), 0);
        // No description: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0048_2cd0, &args![0u32, 0x1122_3344u32]);
        assert!(calls(&e, ADD_CHUNK_ARRAY).is_empty());
    }

    #[test]
    fn copy_component_takes_the_text_and_offset() {
        let mut e = engine();
        let this = description(&mut e);
        let other = description(&mut e);
        e.register(OTHER_OFFSET, |_, a| (a[0] + 1).into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0048_2d80, &args![this, other]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, TYPE_BASE_FORM_COMPONENT, CAST_FROM, 0]]
        );
        assert_eq!(
            calls(&e, STRING_SET),
            vec![vec![DESCRIPTION_TEXT, 0x6000 + other, 0]]
        );
        assert_eq!(e.mem.u32(this + 4), other + 1);
        // Not a description: nothing.
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.mem.set_u32(this + 4, 9);
        e.call_log = Some(vec![]);
        e.call(0x0048_2d80, &args![this, other]);
        assert!(calls(&e, STRING_SET).is_empty());
        assert_eq!(e.mem.u32(this + 4), 9);
    }

    #[test]
    fn compare_component_branches() {
        let mut e = engine();
        let this = description(&mut e);
        let other = description(&mut e);
        e.register(STRING_COMPARE, |e, _| e.mem.u32(0x7300_0008).into_ret());
        // Not a description: different.
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0048_2df0, &args![this, other]).bool());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        // Both empty: same, nothing compared.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0048_2df0, &args![this, other]).bool());
        assert!(calls(&e, STRING_COMPARE).is_empty());
        // Same length, StrCmp 0: same; non-zero: different.
        e.mem.set_u32(0x7300_0000, 3);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0048_2df0, &args![this, other]).bool());
        assert_eq!(
            calls(&e, STRING_COMPARE),
            vec![vec![DESCRIPTION_TEXT, 0x6000 + other, 0]]
        );
        e.mem.set_u32(0x7300_0008, 1);
        assert!(e.call(0x0048_2df0, &args![this, other]).bool());
    }

    #[test]
    fn compare_component_different_lengths() {
        let mut e = engine();
        let this = description(&mut e);
        let other = description(&mut e);
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(STRING_COMPARE, |_, _| 0u32.into_ret());
        // The length double answers 3 then 4.
        let mut n = 0u32;
        e.register_double(STRING_LENGTH, move |_, _| {
            n += 1;
            (2 + n).into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_2df0, &args![this, other]).bool());
        assert!(calls(&e, STRING_COMPARE).is_empty());
    }
}
