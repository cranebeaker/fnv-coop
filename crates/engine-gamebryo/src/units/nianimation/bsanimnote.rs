//! `NiAnimation/bsanimnote.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSAnimNotes::ms_RTTI` (the class's `NiRTTI` record).
const BS_ANIM_NOTES_RTTI: u32 = 0x011f_3f00;

// Translated from 00a44920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNotes::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_anim_notes_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_ANIM_NOTES_RTTI)
}

/// `00a5d6e0`: the jump target of `00a44910`.
const JUMP_TARGET_00A44910: u32 = 0x00a5_d6e0;

// Translated from 00a44910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNotes::GetViewerStrings` (Xbox PDB): a 5-byte jump to `00a5d6e0` (engine map: `NiObject::GetViewerStrings`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?GetViewerStrings@BSAnimNotes@@UAAXPAV?$NiTPrimitiveArray@PAD@@@Z`.
pub fn bs_anim_notes_get_viewer_strings(e: &mut Engine, this: Ptr, arg: Ptr) {
    e.call(JUMP_TARGET_00A44910, &args![this, arg]);
}

// Translated from 00a44840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, one stack word it never reads, `RET 4`): answers `this`.
pub fn fn_00a44840(_e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    this
}

/// `BSAnimNote` (Xbox PDB, 0x10 bytes; same offsets on PC): `eType` +8,
/// `fTime` +0xC.
const NOTE_TYPE: u32 = 0x8;
const NOTE_TIME: u32 = 0xc;
/// `BSAnimNotes` (Xbox PDB, 0x10 bytes): `NoteA` +8 (`BSAnimNote**`),
/// `usNumNotes` +0xC.
const NOTES: u32 = 0x8;
const NUM_NOTES: u32 = 0xc;
/// The vtable `BSAnimNote`'s destructor puts back (`01097bf4`) and
/// `BSAnimNotes`'s vtable (`01097dd4`); `BSAnimNotes`' size.
const BS_ANIM_NOTE_VTABLE: u32 = 0x0109_7bf4;
const BS_ANIM_NOTES_VTABLE: u32 = 0x0109_7dd4;
const BS_ANIM_NOTES_SIZE: u32 = 0x10;
/// `NiObject`'s functions (`niobject.cpp`): the constructor (`00a5d3a0`),
/// the destructor body (`00a5d3d0`), the member copy (`00a5d450`, `(this,
/// clone, cloning)`), `IsEqual` (`00a5d470`), `RegisterStreamables`
/// (`00a5d2f0`), `LoadBinary` (`00a5d610`) and `GetViewerStrings`
/// (`00a5d6e0`); `00a89af0`, the base step both `LinkObject` and
/// `SaveBinary` run first (`thiscall (this, stream)`).
const NI_OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
const NI_OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
const NI_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
const NI_OBJECT_BASE_STEP: u32 = 0x00a8_9af0;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, `thiscall (stream)`) and the
/// link read `LoadBinary` runs per note (`00a645b0`, `thiscall (stream)`).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiStream`'s `SaveLinkID` slot (+0x30) and the objects'
/// `RegisterStreamables` slot (+0x54).
const STREAM_SLOT_SAVE_LINK_ID: u32 = 0x30;
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
/// `InterlockedIncrement` (import slot, one word: the address).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// The memory manager (`011f6238`) with its allocate (`00aa3e40`) and free
/// (`00aa4060`), both `thiscall`, one word; `00aa13e0` (`cdecl`) the Ni
/// allocator and `00aa1460` (`cdecl (block, size)`) the sized delete.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// The viewer-string makers (`cdecl`): the float form (`00a7d550`) and the
/// string form (`00a7d5e0`); the names `"Time"` and `"Type"` and the values
/// `"GrabIK"` and `"AnimNote"`.
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
const TIME_NAME: u32 = 0x0109_7e74;
const TYPE_NAME: u32 = 0x0109_7e6c;
const GRAB_IK_TEXT: u32 = 0x0109_7dc4;
const ANIM_NOTE_TEXT: u32 = 0x0109_7e60;

// Translated from 00a44830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNote::~BSAnimNote` (Xbox PDB): the vtable at `01097bf4`, then the
/// `NiObject` destructor body (a tail jump).
pub fn bs_anim_note_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BS_ANIM_NOTE_VTABLE);
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]);
}

// Translated from 00a44850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSAnimNote`'s member copy, `thiscall (this,
/// clone, cloning)`): `NiObject`'s member copy, then the type and the time
/// (`FLD`/`FSTP`).
pub fn fn_00a44850(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(NI_OBJECT_COPY_MEMBERS, &args![this, clone, cloning]);
    let note_type = e.mem.u32(this.addr() + NOTE_TYPE);
    e.mem
        .set_u32(clone.addr().wrapping_add(NOTE_TYPE), note_type);
    let time = x87_quiet_bits(e.mem.u32(this.addr() + NOTE_TIME));
    e.mem.set_u32(clone.addr().wrapping_add(NOTE_TIME), time);
}

// Translated from 00a44880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNote::IsEqual` (Xbox PDB): `NiObject::IsEqual`, then the same type
/// and the same time (x87 compare: a NaN is never equal).
pub fn bs_anim_note_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.mem.u32(this.addr() + NOTE_TYPE) != e.mem.u32(other.addr() + NOTE_TYPE) {
        return false;
    }
    x87_load32(e.mem.u32(this.addr() + NOTE_TIME))
        == x87_load32(e.mem.u32(other.addr() + NOTE_TIME))
}

// Translated from 00a448c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNotes::RegisterStreamables` (Xbox PDB): `NiObject`'s
/// `RegisterStreamables`; then, while the answer is true, each note
/// registers itself (vtable +0x54) and its answer becomes the answer.
pub fn bs_anim_notes_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    let mut answer = e
        .call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool();
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(this.addr() + NUM_NOTES)) && answer {
        let notes = e.mem.u32(this.addr() + NOTES);
        let note = e.mem.u32(notes.wrapping_add(4 * index));
        answer = e
            .vcall(note, SLOT_REGISTER_STREAMABLES, &args![stream])
            .bool();
        index = index.wrapping_add(1);
    }
    answer
}

// Translated from 00a44930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNote::ParseTextKey` (Xbox PDB): the note's time becomes the text
/// key's time (its first float, through `FLD`/`FSTP`); true.
pub fn bs_anim_note_parse_text_key(e: &mut Engine, this: Ptr, key: Ptr) -> bool {
    let time = x87_quiet_bits(e.mem.u32(key.addr()));
    e.mem.set_u32(this.addr() + NOTE_TIME, time);
    true
}

// Translated from 00a44940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSAnimNotes` factory (registered through `NiStream::RegisterLoader`
/// at `00a2d8ad`; Ghidra made no function of it): 0x10 bytes from the Ni
/// allocator, the `NiObject` constructor and the class's vtable (the notes
/// are left as allocated). Null when the allocation fails.
pub fn fn_00a44940(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![BS_ANIM_NOTES_SIZE]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(NI_OBJECT_CONSTRUCTOR, &args![object]);
    e.mem.set_u32(object, BS_ANIM_NOTES_VTABLE);
    Ptr::new(object)
}

// Translated from 00a44970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNotes`' destructor body (unnamed in the engine map): the class's
/// vtable, each note's deleting destructor (vtable +0 with 1), the note
/// array freed through the memory manager, then the `NiObject` destructor
/// body (a tail jump).
pub fn fn_00a44970(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BS_ANIM_NOTES_VTABLE);
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(this.addr() + NUM_NOTES)) {
        let notes = e.mem.u32(this.addr() + NOTES);
        let note = e.mem.u32(notes.wrapping_add(4 * index));
        if note != 0 {
            e.vcall(note, 0, &args![1u32]);
        }
        index = index.wrapping_add(1);
    }
    let notes = e.mem.u32(this.addr() + NOTES);
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, notes]);
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]);
}

// Translated from 00a449c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNotes::LinkObject` (Xbox PDB): the base step (`00a89af0`), then
/// each note slot takes the stream's next linked object
/// (`GetObjectFromLinkID`), counted (`InterlockedIncrement` of its +4; the
/// exe does not test it for null).
pub fn bs_anim_notes_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_BASE_STEP, &args![this, stream]);
    let mut index = 0u32;
    while (index as i32) < i32::from(e.mem.u16(this.addr() + NUM_NOTES)) {
        let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        let notes = e.mem.u32(this.addr() + NOTES);
        e.mem.set_u32(notes.wrapping_add(4 * index), object);
        let notes = e.mem.u32(this.addr() + NOTES);
        let note = e.mem.u32(notes.wrapping_add(4 * index));
        e.call(INTERLOCKED_INCREMENT, &args![note.wrapping_add(4)]);
        index = index.wrapping_add(1);
    }
}

// Translated from 00a44a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNotes::_scalar_deleting_destructor_` (Xbox PDB): the destructor
/// body (`fn_00a44970`), then the sized delete of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bs_anim_notes_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00a44970(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, BS_ANIM_NOTES_SIZE]);
    }
    this
}

// Translated from 00a44a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNote::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, then the
/// type and the time (4 bytes each; the component size in the stream
/// argument's stack word).
pub fn bs_anim_note_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    use crate::units::nianimation::niblendinterpolator::stream_read;
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, argument| {
        let argument = argument.addr();
        stream_read(e, stream, this.addr() + NOTE_TYPE, 4, argument);
        stream_read(e, stream, this.addr() + NOTE_TIME, 4, argument);
    });
}

// Translated from 00a44a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNote::SaveBinary` (Xbox PDB): the base step (`00a89af0`), then
/// the type and the time (4 bytes each; the component size in the stream
/// argument's stack word).
pub fn bs_anim_note_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    use crate::units::nianimation::niblendinterpolator::stream_write;
    e.call(NI_OBJECT_BASE_STEP, &args![this, stream]);
    e.with_stack(4, |e, argument| {
        let argument = argument.addr();
        stream_write(e, stream, this.addr() + NOTE_TYPE, 4, argument);
        stream_write(e, stream, this.addr() + NOTE_TIME, 4, argument);
    });
}

// Translated from 00a44af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNotes::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, the note
/// count (2 bytes), with notes a note array of `4 * count` bytes (all ones
/// on overflow) from the memory manager, then one link read per note
/// (`00a645b0`; its answer is not kept here, `LinkObject` resolves the
/// links).
pub fn bs_anim_notes_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    use crate::units::nianimation::niblendinterpolator::stream_read;
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, argument| {
        stream_read(e, stream, this.addr() + NUM_NOTES, 2, argument.addr());
    });
    let count = u32::from(e.mem.u16(this.addr() + NUM_NOTES));
    if count != 0 {
        let bytes = count.checked_mul(4).unwrap_or(u32::MAX);
        let array = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
            .u32();
        e.mem.set_u32(this.addr() + NOTES, array);
    }
    let mut index = 0u32;
    while (index as i32) < i32::from(e.mem.u16(this.addr() + NUM_NOTES)) {
        e.call(STREAM_READ_LINK_ID, &args![stream]);
        index = index.wrapping_add(1);
    }
}

// Translated from 00a44b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNotes::SaveBinary` (Xbox PDB): the base step (`00a89af0`), the
/// note count (2 bytes; the component size in the stream argument's stack
/// word), then each note's link (`SaveLinkID`, the stream's vtable +0x30).
pub fn bs_anim_notes_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    use crate::units::nianimation::niblendinterpolator::stream_write;
    e.call(NI_OBJECT_BASE_STEP, &args![this, stream]);
    e.with_stack(4, |e, argument| {
        stream_write(e, stream, this.addr() + NUM_NOTES, 2, argument.addr());
    });
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(this.addr() + NUM_NOTES)) {
        let notes = e.mem.u32(this.addr() + NOTES);
        let note = e.mem.u32(notes.wrapping_add(4 * index));
        e.vcall(stream.addr(), STREAM_SLOT_SAVE_LINK_ID, &args![note]);
        index = index.wrapping_add(1);
    }
}

// Translated from 00a44bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNote::GetViewerStrings` (Xbox PDB): `NiObject`'s strings, then
/// `"Time"` with the time, `"Type"` `"GrabIK"` when the type is 1, and
/// always `"Type"` `"AnimNote"` (all through the argument's stack word).
pub fn bs_anim_note_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    use crate::units::nianimation::niblendinterpolator::append_viewer_string;
    e.call(NI_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let time = x87_quiet_bits(e.mem.u32(this.addr() + NOTE_TIME));
        let text = e.call(VIEWER_STRING_FLOAT, &args![TIME_NAME, time]).u32();
        append_viewer_string(e, strings, slot, text);
        if e.mem.u32(this.addr() + NOTE_TYPE) == 1 {
            let text = e
                .call(VIEWER_STRING_TEXT, &args![TYPE_NAME, GRAB_IK_TEXT])
                .u32();
            append_viewer_string(e, strings, slot, text);
        }
        let text = e
            .call(VIEWER_STRING_TEXT, &args![TYPE_NAME, ANIM_NOTE_TEXT])
            .u32();
        append_viewer_string(e, strings, slot, text);
    });
}

// Translated from 00a84c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimNote::RegisterStreamables` (Xbox PDB): `NiObject`'s
/// `RegisterStreamables`, as a bool (`SETNZ`).
pub fn bs_anim_note_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a44920, bs_anim_notes_get_rtti() -> Ptr),
        entry!(0x00a4_4910, bs_anim_notes_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00a44840, fn_00a44840(Ptr, u32) -> Ptr),
        entry!(0x00a4_4830, bs_anim_note_destructor(Ptr)),
        entry!(0x00a4_4850, fn_00a44850(Ptr, Ptr, Ptr)),
        entry!(0x00a4_4880, bs_anim_note_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a4_48c0,
            bs_anim_notes_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a4_4930, bs_anim_note_parse_text_key(Ptr, Ptr) -> bool),
        entry!(0x00a4_4940, fn_00a44940() -> Ptr),
        entry!(0x00a4_4970, fn_00a44970(Ptr)),
        entry!(0x00a4_49c0, bs_anim_notes_link_object(Ptr, Ptr)),
        entry!(
            0x00a4_4a10,
            bs_anim_notes_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a4_4a30, bs_anim_note_load_binary(Ptr, Ptr)),
        entry!(0x00a4_4a90, bs_anim_note_save_binary(Ptr, Ptr)),
        entry!(0x00a4_4af0, bs_anim_notes_load_binary(Ptr, Ptr)),
        entry!(0x00a4_4b80, bs_anim_notes_save_binary(Ptr, Ptr)),
        entry!(0x00a4_4bf0, bs_anim_note_get_viewer_strings(Ptr, Ptr)),
        entry!(
            0x00a8_4c70,
            bs_anim_note_register_streamables(Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_anim_notes_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a44920, &args![]).u32(), BS_ANIM_NOTES_RTTI);
    }

    #[test]
    fn thunk_00a44910_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_4910;
        const TARGET: u32 = 0x00a5_d6e0;
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
    fn fn_00a44840_answers_this() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a44840, &args![0x1234u32, 7u32]).u32(), 0x1234);
    }
}

#[cfg(test)]
mod a07_tests {
    use super::*;
    use crate::units::nianimation::niblendinterpolator::a07_test_support::{
        calls, stream, viewer_strings,
    };

    const TEST_VTABLE: u32 = 0x7400_0000;
    fn slot(offset: u32) -> u32 {
        0x7500_0000 + offset
    }

    /// Doubles for every slot of the test vtable (`RegisterStreamables`
    /// answers the object's word +8 being 0), the interlocked increment
    /// counting in place, and the `NiObject` functions (`IsEqual` and
    /// `RegisterStreamables` answer 1).
    fn engine() -> Engine {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x40).map(|i| slot(4 * i)).collect();
        e.put_vtable(TEST_VTABLE, &slots);
        for i in 0..0x40 {
            e.register(slot(4 * i), |_, _| Ret::default());
        }
        e.register(slot(SLOT_REGISTER_STREAMABLES), |e, a| {
            (e.mem.u32(a[0] + 8) == 0).into_ret()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let n = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], n);
            n.into_ret()
        });
        for f in [
            NI_OBJECT_CONSTRUCTOR,
            NI_OBJECT_DESTRUCTOR,
            NI_OBJECT_COPY_MEMBERS,
            NI_OBJECT_LOAD_BINARY,
            NI_OBJECT_GET_VIEWER_STRINGS,
            NI_OBJECT_BASE_STEP,
            MEMORY_MANAGER_FREE,
            SIZED_DELETE,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(NI_OBJECT_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e
    }

    fn note(e: &mut Engine, note_type: u32, time: f32) -> u32 {
        let object = e.mem.alloc(0x1c);
        e.mem.set_u32(object, TEST_VTABLE);
        e.mem.set_u32(object + NOTE_TYPE, note_type);
        e.mem.set_f32(object + NOTE_TIME, time);
        object
    }

    /// A `BSAnimNotes` holding `notes`.
    fn notes(e: &mut Engine, notes: &[u32]) -> u32 {
        let this = e.mem.alloc(BS_ANIM_NOTES_SIZE);
        let array = e.mem.alloc(4 * (notes.len() as u32).max(1));
        for (i, n) in notes.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *n);
        }
        e.mem.set_u32(this + NOTES, array);
        e.mem.set_u16(this + NUM_NOTES, notes.len() as u16);
        this
    }

    #[test]
    fn destructor_restores_the_vtable() {
        let mut e = engine();
        let this = note(&mut e, 0, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_4830, &args![this]);
        assert_eq!(e.mem.u32(this), BS_ANIM_NOTE_VTABLE);
        assert_eq!(calls(&e, NI_OBJECT_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn copy_members_copies_type_and_time() {
        let mut e = engine();
        let (this, clone) = (note(&mut e, 2, 1.5), note(&mut e, 0, 0.0));
        e.call_log = Some(vec![]);
        e.call(0x00a4_4850, &args![this, clone, 0x40u32]);
        assert_eq!(
            calls(&e, NI_OBJECT_COPY_MEMBERS),
            vec![vec![this, clone, 0x40]]
        );
        assert_eq!(e.mem.u32(clone + NOTE_TYPE), 2);
        assert_eq!(e.mem.f32(clone + NOTE_TIME), 1.5);
    }

    #[test]
    fn is_equal_compares_type_and_time() {
        let mut e = engine();
        let (a, b) = (note(&mut e, 1, 2.0), note(&mut e, 1, 2.0));
        assert!(e.call(0x00a4_4880, &args![a, b]).bool());
        e.mem.set_u32(b + NOTE_TYPE, 2);
        assert!(!e.call(0x00a4_4880, &args![a, b]).bool());
        e.mem.set_u32(b + NOTE_TYPE, 1);
        e.mem.set_f32(b + NOTE_TIME, 3.0);
        assert!(!e.call(0x00a4_4880, &args![a, b]).bool());
        e.register(NI_OBJECT_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a4_4880, &args![a, a]).bool());
    }

    #[test]
    fn notes_register_streamables_stops_at_the_first_false() {
        let mut e = engine();
        let (a, b, c) = (
            note(&mut e, 0, 0.0),
            note(&mut e, 0, 0.0),
            note(&mut e, 0, 0.0),
        );
        e.mem.set_u32(b + NOTE_TYPE, 1);
        let this = notes(&mut e, &[a, b, c]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a4_48c0, &args![this, 0x99u32]).bool());
        assert_eq!(
            calls(&e, slot(SLOT_REGISTER_STREAMABLES)),
            vec![vec![a, 0x99], vec![b, 0x99]]
        );
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a4_48c0, &args![this, 0x99u32]).bool());
        assert!(calls(&e, slot(SLOT_REGISTER_STREAMABLES)).is_empty());
    }

    #[test]
    fn parse_text_key_takes_the_key_time() {
        let mut e = engine();
        let this = note(&mut e, 0, 0.0);
        let key = e.mem.alloc(8);
        e.mem.set_f32(key, 4.5);
        assert!(e.call(0x00a4_4930, &args![this, key]).bool());
        assert_eq!(e.mem.f32(this + NOTE_TIME), 4.5);
    }

    #[test]
    fn notes_factory_and_destructors() {
        let mut e = engine();
        let object = e.call(0x00a4_4940, &args![]).u32();
        assert_eq!(e.mem.u32(object), BS_ANIM_NOTES_VTABLE);
        let a = note(&mut e, 0, 0.0);
        let this = notes(&mut e, &[a, 0]);
        let array = e.mem.u32(this + NOTES);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_4a10, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, slot(0)), vec![vec![a, 1]]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, array]]
        );
        assert_eq!(calls(&e, NI_OBJECT_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(
            calls(&e, SIZED_DELETE),
            vec![vec![this, BS_ANIM_NOTES_SIZE]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00a4_4970, &args![this]);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_4940, &args![]).u32(), 0);
    }

    #[test]
    fn link_object_resolves_and_counts_each_note() {
        let mut e = engine();
        let (a, b) = (note(&mut e, 0, 0.0), note(&mut e, 0, 0.0));
        let queue = std::rc::Rc::new(std::cell::RefCell::new(vec![b, a]));
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            queue.borrow_mut().pop().unwrap().into_ret()
        });
        let this = notes(&mut e, &[0, 0]);
        e.call(0x00a4_49c0, &args![this, 0x55u32]);
        let array = e.mem.u32(this + NOTES);
        assert_eq!([e.mem.u32(array), e.mem.u32(array + 4)], [a, b]);
        assert_eq!([e.mem.u32(a + 4), e.mem.u32(b + 4)], [1, 1]);
    }

    #[test]
    fn note_load_and_save_binary_move_type_and_time() {
        let mut e = engine();
        let mut bytes = vec![];
        bytes.extend(2u32.to_le_bytes());
        bytes.extend(1.25f32.to_le_bytes());
        let (s, written) = stream(&mut e, 0, &bytes);
        let this = note(&mut e, 0, 0.0);
        e.call(0x00a4_4a30, &args![this, s]);
        assert_eq!(e.mem.u32(this + NOTE_TYPE), 2);
        assert_eq!(e.mem.f32(this + NOTE_TIME), 1.25);
        e.call(0x00a4_4a90, &args![this, s]);
        assert_eq!(*written.borrow(), bytes);
    }

    #[test]
    fn notes_load_binary_allocates_and_reads_each_link() {
        let mut e = engine();
        e.register(STREAM_READ_LINK_ID, |_, _| 0u32.into_ret());
        let (s, _) = stream(&mut e, 0, &3u16.to_le_bytes());
        let this = notes(&mut e, &[]);
        e.mem.set_u32(this + NOTES, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_4af0, &args![this, s]);
        assert_eq!(e.mem.u16(this + NUM_NOTES), 3);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 12]]
        );
        assert_ne!(e.mem.u32(this + NOTES), 0);
        assert_eq!(calls(&e, STREAM_READ_LINK_ID).len(), 3);
        let (s, _) = stream(&mut e, 0, &0u16.to_le_bytes());
        e.call_log = Some(vec![]);
        e.call(0x00a4_4af0, &args![this, s]);
        assert!(calls(&e, MEMORY_MANAGER_ALLOCATE).is_empty());
    }

    #[test]
    fn notes_save_binary_writes_the_count_and_the_links() {
        let mut e = engine();
        let (s, written) = stream(&mut e, 0, &[]);
        e.mem.set_u32(s.addr(), TEST_VTABLE);
        let this = notes(&mut e, &[0x111, 0x222]);
        e.call_log = Some(vec![]);
        e.call(0x00a4_4b80, &args![this, s]);
        assert_eq!(*written.borrow(), vec![2, 0]);
        assert_eq!(
            calls(&e, slot(STREAM_SLOT_SAVE_LINK_ID)),
            vec![vec![s.addr(), 0x111], vec![s.addr(), 0x222]]
        );
    }

    #[test]
    fn get_viewer_strings_adds_time_and_type() {
        let mut e = engine();
        let (strings, stored) = viewer_strings(&mut e);
        let this = note(&mut e, 1, 0.5);
        e.call(0x00a4_4bf0, &args![this, strings]);
        assert_eq!(*stored.borrow(), vec![TIME_NAME, TYPE_NAME, TYPE_NAME]);
        let other = note(&mut e, 2, 0.5);
        stored.borrow_mut().clear();
        e.call_log = Some(vec![]);
        e.call(0x00a4_4bf0, &args![other, strings]);
        assert_eq!(*stored.borrow(), vec![TIME_NAME, TYPE_NAME]);
        assert_eq!(
            calls(&e, VIEWER_STRING_TEXT),
            vec![vec![TYPE_NAME, ANIM_NOTE_TEXT]]
        );
    }

    #[test]
    fn note_register_streamables_is_the_base_as_a_bool() {
        let mut e = engine();
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| 0x100u32.into_ret());
        assert_eq!(e.call(0x00a8_4c70, &args![1u32, 2u32]).u32(), 0);
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| 0x2u32.into_ret());
        assert_eq!(e.call(0x00a8_4c70, &args![1u32, 2u32]).u32(), 1);
    }
}
