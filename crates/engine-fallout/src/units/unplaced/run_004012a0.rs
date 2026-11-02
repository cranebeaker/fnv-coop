//! `(unplaced)/run_004012a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesfile::TESFile;

/// The byte global [`fn_00401500`] answers. Other translations read it as the
/// save writer's big-endian flag; this function only returns it.
const BYTE_ORDER_FLAG_011C54BA: u32 = 0x011c_54ba;

// Translated from 00401500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the byte global at `011c54ba` in `AL` (no arguments, no other
/// effect). The engine map gives this folded getter no name.
pub fn fn_00401500(e: &mut Engine) -> u8 {
    e.global(BYTE_ORDER_FLAG_011C54BA)
}

// Translated from 00401660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers `m_actualChunkSize` (Xbox PDB, `+0x25C`) of a `TESFile`: the size
/// of the current chunk. The engine map gives it no name.
pub fn fn_00401660(e: &mut Engine, file: Ptr<TESFile>) -> u32 {
    e.get(file, TESFile::m_actualChunkSize)
}

// Translated from 00401680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers `bMustEndianConvert` (Xbox PDB, the byte at `+0x299`) of a
/// `TESFile` in `AL`. The engine map gives it no name.
pub fn fn_00401680(e: &mut Engine, file: Ptr<TESFile>) -> bool {
    e.get(file, TESFile::bMustEndianConvert)
}

// Translated from 004012a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004012d0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004012a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_12d0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `004019a0` (the engine map calls it `WaitForSingleObjectEx`, which its body is not): `cdecl`,
/// one stack word; pushes it and calls the import `InterlockedDecrement` (slot `00fdf1c4`), and
/// returns the decremented value.
const INTERLOCKED_DECREMENT_THUNK: u32 = 0x0040_19a0;
/// Vtable slot +4 of the object (byte offset), called with no arguments.
const SLOT_FOUR: u32 = 0x04;

// Translated from 00401970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the decompiler's `WaitForSingleObjectEx` is `004019a0`'s wrong
/// name). `__fastcall`: decrements the 32-bit counter at `+4` of `this` (the reference count of
/// an `NiRefObject`-shaped object) through `004019a0` and, when the result is zero, calls virtual
/// slot +4 of `this`.
pub fn fn_00401970(e: &mut Engine, this: Ptr) {
    let remaining = e
        .call(
            INTERLOCKED_DECREMENT_THUNK,
            &args![this.addr().wrapping_add(4)],
        )
        .u32();
    if remaining == 0 {
        e.vcall(this.addr(), SLOT_FOUR, &args![]);
    }
}

/// The number of `BGSDehydrationStage` forms (the class whose destructor `004012d0` removes
/// them) in the sorted list at [`SORTED_FORMS_011C3184`] (a word).
const SORTED_FORM_COUNT_011C3180: u32 = 0x011c_3180;
/// The list itself: a heap array of form pointers, kept in ascending order of the word at
/// +0x18 of each form (the word `00500940` points at and `009611e0` reads).
const SORTED_FORMS_011C3184: u32 = 0x011c_3184;
/// `004024e0` (cdecl, one word: the element count): allocates a zeroed array of pointers.
const ALLOCATE_POINTER_ARRAY: u32 = 0x0040_24e0;
/// `00401030` (cdecl, one word): frees a block.
const FREE_BLOCK: u32 = 0x0040_1030;
/// `00500940` (`thiscall`, no stack word): the address `this+0x18`.
const SORT_KEY_ADDRESS: u32 = 0x0050_0940;

// Translated from 00401930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): the form at `index` in the sorted list at
/// `011c3184`, or 0 when `index` is not below the count at `011c3180`.
pub fn fn_00401930(e: &mut Engine, index: u32) -> Ptr {
    let count: u32 = e.global(SORTED_FORM_COUNT_011C3180);
    if index < count {
        let forms: u32 = e.global(SORTED_FORMS_011C3184);
        Ptr::new(e.mem.u32(forms.wrapping_add(index.wrapping_mul(4))))
    } else {
        Ptr::NULL
    }
}

// Translated from 004018b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one `float` word): walks the sorted list from the
/// start and returns the first form whose word read by `009611e0` (the unsigned word at
/// +0x18, compared as a whole number) is at most `limit`; 0 if none is (or `limit` is NaN:
/// the `FCOMPP` test treats unordered as "less").
pub fn fn_004018b0(e: &mut Engine, limit: f32) -> Ptr {
    let mut found = Ptr::NULL;
    let mut index = 0u32;
    while index < e.global::<u32>(SORTED_FORM_COUNT_011C3180) && found.is_null() {
        let form = fn_00401930(e, index);
        if !form.is_null() {
            let key = e.call(0x0096_11e0, &args![form]).u32();
            // FILD qword (the word zero-extended), FLD the limit, FCOMPP; C0 clear means
            // limit >= key and neither is NaN.
            if f64::from(limit) >= f64::from(key) {
                found = form;
            }
        }
        index += 1;
    }
    found
}

// Translated from 004016a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): inserts `form` into the sorted list. A new
/// array of `count + 1` pointers (the count truncated to a byte, as the code keeps it) comes
/// from `004024e0`; the old entries are copied over with `form` placed before the first
/// entry whose key (the word at `00500940`'s address) is below `form`'s, or at the end. The
/// old array is freed through `00401030`, and the new count (the byte) and array are stored.
pub fn fn_004016a0(e: &mut Engine, form: Ptr) {
    let new_count = e.global::<u32>(SORTED_FORM_COUNT_011C3180).wrapping_add(1) as u8;
    let array = e
        .call(ALLOCATE_POINTER_ARRAY, &args![u32::from(new_count)])
        .u32();
    let mut written = 0u32;
    let mut inserted = false;
    let mut index = 0u32;
    while index < e.global::<u32>(SORTED_FORM_COUNT_011C3180) {
        let own_key = e.call(SORT_KEY_ADDRESS, &args![form]).u32();
        let forms: u32 = e.global(SORTED_FORMS_011C3184);
        let other = e.mem.u32(forms.wrapping_add(index.wrapping_mul(4)));
        let other_key = e.call(SORT_KEY_ADDRESS, &args![other]).u32();
        if e.mem.u32(own_key) > e.mem.u32(other_key) && !inserted {
            e.mem
                .set_u32(array.wrapping_add(written.wrapping_mul(4)), form.addr());
            written = written.wrapping_add(1);
            inserted = true;
        }
        let forms: u32 = e.global(SORTED_FORMS_011C3184);
        let entry = e.mem.u32(forms.wrapping_add(index.wrapping_mul(4)));
        e.mem
            .set_u32(array.wrapping_add(written.wrapping_mul(4)), entry);
        written = written.wrapping_add(1);
        index += 1;
    }
    if !inserted {
        e.mem
            .set_u32(array.wrapping_add(written.wrapping_mul(4)), form.addr());
    }
    let old: u32 = e.global(SORTED_FORMS_011C3184);
    e.call(FREE_BLOCK, &args![old]);
    e.set_global(SORTED_FORM_COUNT_011C3180, u32::from(new_count));
    e.set_global(SORTED_FORMS_011C3184, array);
}

// Translated from 004017a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): removes `form` from the sorted list. With
/// an empty list, or when `form` is not in it, nothing happens. Otherwise a new array of
/// `count - 1` pointers comes from `004024e0`; when that is not null, every entry but
/// `form` is copied into it. The old array is freed through `00401030` and the new count and
/// array (null if the allocation gave null) are stored.
pub fn fn_004017a0(e: &mut Engine, form: Ptr) {
    if e.global::<u32>(SORTED_FORM_COUNT_011C3180) == 0 {
        return;
    }
    let mut found = false;
    let mut index = 0u32;
    while !found && index < e.global::<u32>(SORTED_FORM_COUNT_011C3180) {
        let forms: u32 = e.global(SORTED_FORMS_011C3184);
        if e.mem.u32(forms.wrapping_add(index.wrapping_mul(4))) == form.addr() {
            found = true;
        }
        index += 1;
    }
    if !found {
        return;
    }
    let new_count = e.global::<u32>(SORTED_FORM_COUNT_011C3180).wrapping_sub(1);
    let array = e.call(ALLOCATE_POINTER_ARRAY, &args![new_count]).u32();
    if array != 0 {
        let mut written = 0u32;
        let mut index = 0u32;
        while index < e.global::<u32>(SORTED_FORM_COUNT_011C3180) {
            let forms: u32 = e.global(SORTED_FORMS_011C3184);
            let entry = e.mem.u32(forms.wrapping_add(index.wrapping_mul(4)));
            if entry != form.addr() {
                e.mem
                    .set_u32(array.wrapping_add(written.wrapping_mul(4)), entry);
                written = written.wrapping_add(1);
            }
            index += 1;
        }
    }
    let old: u32 = e.global(SORTED_FORMS_011C3184);
    e.call(FREE_BLOCK, &args![old]);
    e.set_global(SORTED_FORM_COUNT_011C3180, new_count);
    e.set_global(SORTED_FORMS_011C3184, array);
}

// Translated from 00401330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): removes the form from the sorted
/// list (`004017a0`). The destructor `004012d0` runs it.
pub fn fn_00401330(e: &mut Engine, this: Ptr) {
    fn_004017a0(e, this);
}

// Translated from 004012d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004012d0 - the exception-unwinding frame (FS:[0] chain, handler 00ef8ae8, state variable) is not translated.
/// Destructor of `BGSDehydrationStage` (the RTTI name of the vtable `0x0101_144c` it stores;
/// unnamed in the engine map, the decompiler shows a library name folded onto it; `ECX` only, plain `RET`): stores
/// that vtable, removes the form from the sorted list (`00401330`), then runs `00483630`
/// (the `TESForm` destructor).
pub fn fn_004012d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_144c);
    fn_00401330(e, this);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 00401350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`; an `InitItem` shape): unless
/// `004013e0` says otherwise, reads the word `00441110` returns (the word at +0x1c); when it
/// is not 0, resolves it in a stack slot through `00485d50` (cdecl, the slot and the form's
/// file from `00484e60` with -1), looks the result up with `004839c0` (cdecl, one word),
/// casts that with `__RTDynamicCast` (`00ec43fb`, cdecl: the pointer, 0, the type
/// descriptors `0x0118_3028` and `0x0118_3060`, 0) and gives the cast to `0050f9a0`. Then
/// `00484ab0` with 1 and the form goes into the sorted list (`004016a0`).
pub fn fn_00401350(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    let word = e.call(0x0044_1110, &args![this]).u32();
    if word != 0 {
        let resolved = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), word);
            let file = e.call(0x0048_4e60, &args![this, 0xffff_ffffu32]).u32();
            e.call(0x0048_5d50, &args![slot, file]);
            e.mem.u32(slot.addr())
        });
        let form = e.call(0x0048_39c0, &args![resolved]).u32();
        let cast = e
            .call(
                0x00ec_43fb,
                &args![form, 0u32, 0x0118_3028u32, 0x0118_3060u32, 0u32],
            )
            .u32();
        e.call(0x0050_f9a0, &args![this, cast]);
    }
    e.call(0x0048_4ab0, &args![this, 1u32]);
    fn_004016a0(e, this);
}

// Translated from 00401510 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00401510 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// The `BGSDehydrationStage` `Load` (form type 0x76; unnamed in the engine map; `ECX` and one stack word, the
/// file, `RET 4`): false unless `TESFile::GetTESForm`'s `AL` is 0x76. Otherwise vtable slot
/// +0x18 (no arguments), `TESForm::LoadForm`, `00484ab0` with 0, then each chunk: `DATA`
/// through `004861f0` (`TESForm::LoadData`) with the file, `00500940`'s address (+0x18) and
/// 8, followed by `00462230` on +0x18 when `00401680` says the file needs byte swapping;
/// `EDID` (a stack buffer of the chunk's size, read with at most 0x200 bytes, given to
/// vtable slot +0x134); `OBND` (vtable slot +0xe0 with the file). Then true.
pub fn fn_00401510(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(0x0047_2660, &args![file]).u8() != 0x76 {
        return false;
    }
    let base = this.addr();
    e.vcall(base, 0x18, &args![]);
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            // DATA
            0x4154_4144 => {
                let data = e.call(SORT_KEY_ADDRESS, &args![this]).u32();
                e.call(0x0048_61f0, &args![this, file, data, 8u32]);
            }
            // EDID
            0x4449_4445 => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                e.with_stack(size, |e, name| {
                    e.call(0x0047_2890, &args![file, name, 0x200u32]);
                    e.vcall(base, 0x134, &args![name]);
                });
            }
            // OBND
            0x444e_424f => {
                e.vcall(base, 0xe0, &args![file]);
            }
            _ => {}
        }
        if chunk == 0x4154_4144 && e.call(0x0040_1680, &args![file]).bool() {
            e.call(0x0046_2230, &args![base + 0x18]);
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    true
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable delta, source type, target type,
/// is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVTESForm@@` and `.?AVBGSDehydrationStage@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_DEHYDRATION_STAGE: u32 = 0x0118_3004;
/// `TESForm::CopyAllComponents` (Xbox PDB, `004851b0`, `ECX`, one stack word: the source).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
/// `TESForm::CompareAllComponents` (Xbox PDB, `00485270`, `ECX`, one stack word), `AL`.
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `00401460` (`cdecl`, three words: destination, source, size).
const COPY_BYTES_00401460: u32 = 0x0040_1460;
/// The CRT's `memcmp` (`00ec4835`, `cdecl`).
const MEMCMP: u32 = 0x00ec_4835;

// Translated from 00401400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDehydrationStage`'s `Copy` slot (+0x108 of its vtable `0101144c`; unnamed in the
/// engine map; `ECX` and one stack word, `RET 4`): casts `source` from `TESForm` to
/// `BGSDehydrationStage`; when that answers a stage, `TESForm::CopyAllComponents` from it,
/// then `00401460` copies 8 bytes from its stage data (`00500940`) into this one's.
pub fn fn_00401400(e: &mut Engine, this: Ptr, source: Ptr) {
    let stage = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, TYPE_TES_FORM, TYPE_DEHYDRATION_STAGE, 0u32],
        )
        .u32();
    if stage == 0 {
        return;
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, stage]);
    let from = e.call(SORT_KEY_ADDRESS, &args![stage]).u32();
    let to = e.call(SORT_KEY_ADDRESS, &args![this]).u32();
    e.call(COPY_BYTES_00401460, &args![to, from, 8u32]);
}

// Translated from 00401480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDehydrationStage`'s `Compare` slot (+0x10c of its vtable `0101144c`; unnamed in the
/// engine map; `ECX` and one stack word, `RET 4`): true (different) when `other` is not a
/// `BGSDehydrationStage` (`__RTDynamicCast` from `TESForm`), when
/// `TESForm::CompareAllComponents` answers true, or when `memcmp` finds the 8 bytes of the
/// two stage datas (`00500940`; this one's first) different; false otherwise.
pub fn fn_00401480(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let stage = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_TES_FORM, TYPE_DEHYDRATION_STAGE, 0u32],
        )
        .u32();
    if stage == 0 {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, stage]).u8() != 0 {
        return true;
    }
    let theirs = e.call(SORT_KEY_ADDRESS, &args![stage]).u32();
    let mine = e.call(SORT_KEY_ADDRESS, &args![this]).u32();
    e.call(MEMCMP, &args![mine, theirs, 8u32]).u32() != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_1400, fn_00401400(Ptr, Ptr)),
        entry!(0x0040_1480, fn_00401480(Ptr, Ptr) -> bool),
        entry!(0x0040_1970, fn_00401970(Ptr)),
        entry!(0x00401500, fn_00401500() -> u8),
        entry!(0x00401660, fn_00401660(Ptr<TESFile>) -> u32),
        entry!(0x00401680, fn_00401680(Ptr<TESFile>) -> bool),
        entry!(0x0040_12a0, fn_004012a0(Ptr, u32) -> Ptr),
        entry!(0x0040_1930, fn_00401930(u32) -> Ptr),
        entry!(0x0040_18b0, fn_004018b0(f32) -> Ptr),
        entry!(0x0040_16a0, fn_004016a0(Ptr)),
        entry!(0x0040_17a0, fn_004017a0(Ptr)),
        entry!(0x0040_1330, fn_00401330(Ptr)),
        entry!(0x0040_12d0, fn_004012d0(Ptr)),
        entry!(0x0040_1350, fn_00401350(Ptr)),
        entry!(0x0040_1510, fn_00401510(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_byte_global() {
        let mut e = Engine::new();
        e.map(BYTE_ORDER_FLAG_011C54BA, 4);
        assert_eq!(e.call(0x0040_1500, &args![]).u32() & 0xff, 0);
        e.set_global::<u8>(BYTE_ORDER_FLAG_011C54BA, 1);
        // The neighbouring byte is not part of the result.
        e.set_global::<u8>(BYTE_ORDER_FLAG_011C54BA + 1, 0xff);
        assert_eq!(e.call(0x0040_1500, &args![]).u32() & 0xff, 1);
    }

    #[test]
    fn answers_the_chunk_size_and_endian_flag() {
        let mut e = Engine::new();
        let file = e.new_object::<TESFile>();
        e.set(file, TESFile::m_actualChunkSize, 0x1234);
        assert_eq!(e.call(0x0040_1660, &args![file]).u32(), 0x1234);
        assert!(!e.call(0x0040_1680, &args![file]).bool());
        e.set(file, TESFile::bMustEndianConvert, true);
        assert!(e.call(0x0040_1680, &args![file]).bool());
    }
    #[test]
    fn at_004012a0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_12d0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_12a0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_12a0)
                .collect();
            let mut expected = vec![(0x0040_12d0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_00401970_calls_slot_4_only_when_the_decrement_reaches_zero() {
        for (remaining, released) in [(0u32, true), (1, false), (7, false)] {
            let mut e = Engine::new();
            e.call_log = Some(vec![]);
            let object = e.mem.alloc(8);
            let table = 0x0f00_4100;
            e.put_vtable(table, &[0, 0x0f00_5000]);
            e.mem.set_u32(object, table);
            e.register_double(0x0040_19a0, move |_, _| remaining.into_ret());
            e.register(0x0f00_5000, |_, _| Ret::default());
            e.call(0x0040_1970, &args![object]);
            let log = e.call_log.take().unwrap();
            let decrements: Vec<&Vec<u32>> = log
                .iter()
                .filter(|c| c.0 == 0x0040_19a0)
                .map(|c| &c.1)
                .collect();
            assert_eq!(decrements, vec![&vec![object + 4]]);
            let releases: Vec<&Vec<u32>> = log
                .iter()
                .filter(|c| c.0 == 0x0f00_5000)
                .map(|c| &c.1)
                .collect();
            if released {
                assert_eq!(releases, vec![&vec![object]]);
            } else {
                assert!(releases.is_empty());
            }
        }
    }

    /// The sorted list globals mapped, holding `forms` (a heap array).
    fn sorted_list(e: &mut Engine, forms: &[u32]) -> u32 {
        e.map(SORTED_FORM_COUNT_011C3180, 8);
        let array = e.mem.alloc(4 * forms.len().max(1) as u32);
        for (index, form) in forms.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *form);
        }
        e.set_global(SORTED_FORM_COUNT_011C3180, forms.len() as u32);
        e.set_global(SORTED_FORMS_011C3184, array);
        array
    }

    /// The list's current entries.
    fn list_entries(e: &Engine) -> Vec<u32> {
        let count: u32 = e.global(SORTED_FORM_COUNT_011C3180);
        let array: u32 = e.global(SORTED_FORMS_011C3184);
        (0..count)
            .map(|index| e.mem.u32(array + 4 * index))
            .collect()
    }

    /// A form whose sort key (the word at +0x18) is `key`.
    fn form_with_key(e: &mut Engine, key: u32) -> u32 {
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(form + 0x18, key);
        form
    }

    /// Doubles for the list's helpers: the allocator gives a fresh zeroed array (or null when
    /// `fail`), the key getters behave as the exe's.
    fn list_helpers(e: &mut Engine, fail: bool) {
        e.register_double(ALLOCATE_POINTER_ARRAY, move |e, a| Ret {
            eax: if fail || a[0] == 0 {
                0
            } else {
                e.mem.alloc(4 * a[0])
            },
            ..Ret::default()
        });
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.register(SORT_KEY_ADDRESS, |_, a| Ret {
            eax: a[0] + 0x18,
            ..Ret::default()
        });
        e.register(0x0096_11e0, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x18),
            ..Ret::default()
        });
    }

    #[test]
    fn fn_00401930_answers_the_entry_or_null_past_the_end() {
        let mut e = Engine::new();
        sorted_list(&mut e, &[0x10, 0x20]);
        assert_eq!(e.call(0x0040_1930, &args![0u32]).u32(), 0x10);
        assert_eq!(e.call(0x0040_1930, &args![1u32]).u32(), 0x20);
        assert_eq!(e.call(0x0040_1930, &args![2u32]).u32(), 0);
        assert_eq!(e.call(0x0040_1930, &args![0xffff_ffffu32]).u32(), 0);
    }

    #[test]
    fn fn_004018b0_finds_the_first_form_with_a_key_at_most_the_limit() {
        let mut e = Engine::new();
        list_helpers(&mut e, false);
        let high = form_with_key(&mut e, 30);
        let middle = form_with_key(&mut e, 20);
        let low = form_with_key(&mut e, 10);
        sorted_list(&mut e, &[high, 0, middle, low]);
        for (limit, expected) in [
            (35.0f32, high),
            (30.0, high),
            (29.5, middle),
            (10.0, low),
            (9.0, 0),
            (f32::NAN, 0),
        ] {
            assert_eq!(
                e.call(0x0040_18b0, &args![limit.to_bits()]).u32(),
                expected,
                "{limit}"
            );
        }
        // Keys are unsigned words.
        let huge = form_with_key(&mut e, 0x8000_0000);
        sorted_list(&mut e, &[huge]);
        assert_eq!(e.call(0x0040_18b0, &args![1.0f32.to_bits()]).u32(), 0);
        assert_eq!(e.call(0x0040_18b0, &args![3.0e9f32.to_bits()]).u32(), huge);
    }

    #[test]
    fn fn_004016a0_inserts_before_the_first_lower_key() {
        let mut e = Engine::new();
        list_helpers(&mut e, false);
        let a = form_with_key(&mut e, 30);
        let b = form_with_key(&mut e, 10);
        let old = sorted_list(&mut e, &[a, b]);
        let middle = form_with_key(&mut e, 20);
        e.call_log = Some(vec![]);
        e.call(0x0040_16a0, &args![middle]);
        assert_eq!(list_entries(&e), vec![a, middle, b]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(ALLOCATE_POINTER_ARRAY, vec![3])));
        assert!(log.contains(&(FREE_BLOCK, vec![old])));
        // Equal and lower keys go to the end; a higher one to the front.
        let last = form_with_key(&mut e, 10);
        e.call(0x0040_16a0, &args![last]);
        assert_eq!(list_entries(&e), vec![a, middle, b, last]);
        let first = form_with_key(&mut e, 99);
        e.call(0x0040_16a0, &args![first]);
        assert_eq!(list_entries(&e), vec![first, a, middle, b, last]);
    }

    #[test]
    fn fn_004016a0_keeps_the_count_in_a_byte() {
        let mut e = Engine::new();
        list_helpers(&mut e, false);
        sorted_list(&mut e, &[]);
        e.set_global(SORTED_FORM_COUNT_011C3180, 0xffu32);
        let array = e.mem.alloc(4 * 0xff);
        e.set_global(SORTED_FORMS_011C3184, array);
        let form = form_with_key(&mut e, 1);
        e.call_log = Some(vec![]);
        // 0xff + 1 truncates to 0: the allocator is asked for 0 and gives null, and the
        // copy loop writes through the null array, so only check what happens before.
        e.map(0, 0x1000);
        e.call(0x0040_16a0, &args![form]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(ALLOCATE_POINTER_ARRAY, vec![0])));
        assert_eq!(e.global::<u32>(SORTED_FORM_COUNT_011C3180), 0);
    }

    #[test]
    fn fn_004017a0_removes_a_listed_form_only() {
        let mut e = Engine::new();
        list_helpers(&mut e, false);
        let old = sorted_list(&mut e, &[0x10, 0x20, 0x30]);
        e.call_log = Some(vec![]);
        e.call(0x0040_17a0, &args![0x40u32]);
        assert_eq!(list_entries(&e), vec![0x10, 0x20, 0x30]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x0040_17a0, &args![0x20u32]);
        assert_eq!(list_entries(&e), vec![0x10, 0x30]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(ALLOCATE_POINTER_ARRAY, vec![2])));
        assert!(log.contains(&(FREE_BLOCK, vec![old])));
        // An empty list is left alone.
        sorted_list(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x0040_17a0, &args![0x10u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_004017a0_stores_a_null_array_when_allocation_fails() {
        let mut e = Engine::new();
        list_helpers(&mut e, true);
        sorted_list(&mut e, &[0x10, 0x20]);
        e.call(0x0040_17a0, &args![0x10u32]);
        assert_eq!(e.global::<u32>(SORTED_FORM_COUNT_011C3180), 1);
        assert_eq!(e.global::<u32>(SORTED_FORMS_011C3184), 0);
    }

    #[test]
    fn fn_00401330_and_the_destructor_remove_the_form() {
        let mut e = Engine::new();
        list_helpers(&mut e, false);
        e.register(0x0048_3630, |_, _| Ret::default());
        let form = form_with_key(&mut e, 5);
        sorted_list(&mut e, &[0x10, form]);
        e.call(0x0040_1330, &args![form]);
        assert_eq!(list_entries(&e), vec![0x10]);
        sorted_list(&mut e, &[form, 0x10]);
        e.call_log = Some(vec![]);
        e.call(0x0040_12d0, &args![form]);
        assert_eq!(list_entries(&e), vec![0x10]);
        assert_eq!(e.mem.u32(form), 0x0101_144c);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap(), &(0x0048_3630, vec![form]));
    }

    #[test]
    fn fn_00401350_resolves_the_linked_form_and_joins_the_list() {
        for (checked, word) in [(true, 7u32), (false, 0), (false, 7)] {
            let mut e = Engine::new();
            list_helpers(&mut e, false);
            sorted_list(&mut e, &[]);
            let form = form_with_key(&mut e, 5);
            e.register_double(0x0040_13e0, move |_, _| Ret {
                eax: u32::from(checked),
                ..Ret::default()
            });
            e.register_double(0x0044_1110, move |_, _| Ret {
                eax: word,
                ..Ret::default()
            });
            e.register(0x0048_4e60, |_, _| Ret {
                eax: 0x9000,
                ..Ret::default()
            });
            e.register(0x0048_5d50, |e, a| {
                let value = e.mem.u32(a[0]);
                e.mem.set_u32(a[0], value + 0x100);
                Ret::default()
            });
            e.register(0x0048_39c0, |_, a| Ret {
                eax: a[0] + 0x1000,
                ..Ret::default()
            });
            e.register(0x00ec_43fb, |_, a| Ret {
                eax: a[0] + 1,
                ..Ret::default()
            });
            e.register(0x0050_f9a0, |_, _| Ret::default());
            e.register(0x0048_4ab0, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x0040_1350, &args![form]);
            let log = e.call_log.take().unwrap();
            let called = |address: u32| -> Vec<Vec<u32>> {
                log.iter()
                    .filter(|call| call.0 == address)
                    .map(|call| call.1.clone())
                    .collect()
            };
            if checked {
                assert_eq!(log.len(), 2);
                assert!(list_entries(&e).is_empty());
                continue;
            }
            assert_eq!(called(0x0048_4ab0), vec![vec![form, 1]]);
            assert_eq!(list_entries(&e), vec![form]);
            if word == 0 {
                assert!(called(0x0050_f9a0).is_empty());
            } else {
                assert_eq!(called(0x0048_4e60), vec![vec![form, 0xffff_ffff]]);
                assert_eq!(called(0x0048_5d50)[0][1], 0x9000);
                assert_eq!(called(0x0048_39c0), vec![vec![0x107]]);
                assert_eq!(
                    called(0x00ec_43fb),
                    vec![vec![0x1107, 0, 0x0118_3028, 0x0118_3060, 0]]
                );
                assert_eq!(called(0x0050_f9a0), vec![vec![form, 0x1108]]);
            }
        }
    }

    #[test]
    fn fn_00401510_loads_the_record() {
        use std::cell::Cell;
        use std::rc::Rc;
        for (form_type, swaps) in [(0x75u32, false), (0x76, false), (0x76, true)] {
            let mut e = Engine::new();
            let form = form_with_key(&mut e, 0);
            let file = 0x5000u32;
            e.put_vtable(0x0300_0000, &[0; 0x80]);
            e.mem.set_u32(0x0300_0000 + 0x18, 0x0300_1018);
            e.mem.set_u32(0x0300_0000 + 0xe0, 0x0300_10e0);
            e.mem.set_u32(0x0300_0000 + 0x134, 0x0300_1134);
            e.mem.set_u32(form, 0x0300_0000);
            let chunks = [0x4154_4144u32, 0x4449_4445, 0x444e_424f, 0x5858_5858];
            let position = Rc::new(Cell::new(0usize));
            let current = position.clone();
            e.register_double(0x0047_2660, move |_, _| Ret {
                eax: form_type,
                ..Ret::default()
            });
            e.register_double(0x0047_26b0, move |_, _| Ret {
                eax: chunks.get(current.get()).copied().unwrap_or(0),
                ..Ret::default()
            });
            e.register_double(0x0047_26f0, move |_, _| {
                position.set(position.get() + 1);
                Ret {
                    eax: u32::from(position.get() < 4),
                    ..Ret::default()
                }
            });
            e.register_double(0x0040_1680, move |_, _| Ret {
                eax: u32::from(swaps),
                ..Ret::default()
            });
            e.register(0x0040_1660, |_, _| Ret {
                eax: 0x10,
                ..Ret::default()
            });
            for address in [
                0x0300_1018u32,
                0x0300_10e0,
                0x0300_1134,
                0x0048_5110,
                0x0048_4ab0,
                0x0048_61f0,
                0x0047_2890,
                0x0046_2230,
            ] {
                e.register(address, |_, _| Ret::default());
            }
            e.register(SORT_KEY_ADDRESS, |_, a| Ret {
                eax: a[0] + 0x18,
                ..Ret::default()
            });
            e.call_log = Some(vec![]);
            let loaded = e.call(0x0040_1510, &args![form, file]).bool();
            assert_eq!(loaded, form_type == 0x76);
            let log: Vec<_> = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|call| {
                    ![
                        0x0040_1510,
                        0x0047_2660,
                        0x0047_26b0,
                        0x0047_26f0,
                        0x0040_1660,
                    ]
                    .contains(&call.0)
                })
                .collect();
            if !loaded {
                assert!(log.is_empty());
                continue;
            }
            let name = log
                .iter()
                .find(|call| call.0 == 0x0047_2890)
                .map(|call| call.1[1])
                .unwrap();
            let mut expected = vec![
                (0x0300_1018, vec![form]),
                (0x0048_5110, vec![form, file]),
                (0x0048_4ab0, vec![form, 0]),
                (SORT_KEY_ADDRESS, vec![form]),
                (0x0048_61f0, vec![form, file, form + 0x18, 8]),
                (0x0040_1680, vec![file]),
            ];
            if swaps {
                expected.push((0x0046_2230, vec![form + 0x18]));
            }
            expected.extend([
                (0x0047_2890, vec![file, name, 0x200]),
                (0x0300_1134, vec![form, name]),
                (0x0300_10e0, vec![form, file]),
            ]);
            assert_eq!(log, expected);
        }
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// An engine whose cast answers its object when `cast` is set (else null), with the
    /// component compare answering `differ` and `memcmp` answering `bytes_differ`.
    fn engine(cast: bool, differ: bool, bytes_differ: bool) -> Engine {
        let mut e = Engine::new();
        e.register(
            DYNAMIC_CAST,
            if cast {
                |_, a| a[0].into_ret()
            } else {
                |_, _| 0u32.into_ret()
            },
        );
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(
            COMPARE_ALL_COMPONENTS,
            if differ {
                |_, _| 0x0101u32.into_ret()
            } else {
                |_, _| 0x0100u32.into_ret()
            },
        );
        e.register(SORT_KEY_ADDRESS, |_, a| (a[0] + 0x18).into_ret());
        e.register(COPY_BYTES_00401460, |_, _| Ret::default());
        e.register(
            MEMCMP,
            if bytes_differ {
                |_, _| 0xffff_ffffu32.into_ret()
            } else {
                |_, _| 0u32.into_ret()
            },
        );
        e.call_log = Some(vec![]);
        e
    }

    fn log_without(e: &mut Engine, top: u32) -> Log {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != top)
            .collect()
    }

    #[test]
    fn fn_00401400_copies_components_and_data_from_a_stage() {
        let mut e = engine(true, false, false);
        e.call(0x0040_1400, &args![0x1000u32, 0x2000u32]);
        let log = log_without(&mut e, 0x0040_1400);
        assert_eq!(
            log,
            vec![
                (
                    DYNAMIC_CAST,
                    vec![0x2000, 0, TYPE_TES_FORM, TYPE_DEHYDRATION_STAGE, 0]
                ),
                (COPY_ALL_COMPONENTS, vec![0x1000, 0x2000]),
                (SORT_KEY_ADDRESS, vec![0x2000]),
                (SORT_KEY_ADDRESS, vec![0x1000]),
                (COPY_BYTES_00401460, vec![0x1018, 0x2018, 8]),
            ]
        );
        let mut e = engine(false, false, false);
        e.call(0x0040_1400, &args![0x1000u32, 0x2000u32]);
        assert_eq!(log_without(&mut e, 0x0040_1400).len(), 1);
    }

    #[test]
    fn fn_00401480_compares_type_components_and_data() {
        let compare = |cast, differ, bytes| {
            let mut e = engine(cast, differ, bytes);
            let r = e.call(0x0040_1480, &args![0x1000u32, 0x2000u32]).bool();
            (r, log_without(&mut e, 0x0040_1480))
        };
        assert!(compare(false, false, false).0);
        let (r, log) = compare(true, true, false);
        assert!(r);
        assert!(log.iter().all(|(a, _)| *a != MEMCMP));
        assert!(compare(true, false, true).0);
        let (r, log) = compare(true, false, false);
        assert!(!r);
        assert_eq!(log.last().unwrap(), &(MEMCMP, vec![0x1018, 0x2018, 8]));
    }
}
