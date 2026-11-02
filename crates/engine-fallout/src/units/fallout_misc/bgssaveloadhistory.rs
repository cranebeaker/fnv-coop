//! `fallout/misc/saveload/bgssaveloadhistory.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BGSSaveLoadHistory` (Xbox PDB, 0x10 bytes) is a single member, `Notes`,
//! a `BSSimpleArray<char *, 1024>` at offset 0, so `this` is typed as the
//! shared [`BSSimpleArray`]. Each note is a heap string the history owns
//! (allocated with `00401000`, freed with `00401030`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// `00822600` / `00822630`: the array's base constructor and destructor
/// (the code the class's own constructor and destructor wrap).
const ARRAY_BASE_CONSTRUCT: u32 = 0x0082_2600;
const ARRAY_BASE_DESTRUCT: u32 = 0x0082_2630;
/// `0084b5c0` (the decompiler calls it `BSSimpleArray<Actor *>::SetReservedSize`
/// from the shared body): sets the reserved size.
const ARRAY_SET_RESERVED_SIZE: u32 = 0x0084_b5c0;
/// `006f2290` (the map's `BSSimpleArray<TESObjectWEAP *>::SetSize`, the
/// shared body): `SetSize(this, 0, 0)` empties the array.
const ARRAY_SET_SIZE: u32 = 0x006f_2290;
/// `007cb2e0`: appends the word at the pointer it is given (the decompiler
/// calls it `AddUninitialized`) and returns its index.
const ARRAY_ADD: u32 = 0x007c_b2e0;
/// `009a4320` (the map's `BSSimpleArray<BGSBodyPart *>::_MoveItems`):
/// `(this, index, count)` removes `count` items from `index`.
const ARRAY_MOVE_ITEMS: u32 = 0x009a_4320;
/// `0044ddc0` / `00620b80`: the array's size (word at `+8`).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
const ARRAY_SIZE_OV2: u32 = 0x0062_0b80;
/// `006a7ad0`: address of element `index` (`this + 4` buffer, 4 bytes each).
const ARRAY_ELEMENT: u32 = 0x006a_7ad0;
/// `0084e320`: the string at element `index` of an array (`__thiscall`).
const ARRAY_STRING: u32 = 0x0084_e320;
/// `00401000` / `00401030`: operator new / operator delete.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00407000`: `vsprintf`-style `(buffer, format, argument list)`, cdecl,
/// returns the length.
const FORMAT_LIST: u32 = 0x0040_7000;
/// `00406d30`: `strcpy_s`-style `(destination, size, source)`, cdecl.
const COPY_STRING: u32 = 0x0040_6d30;
/// `00408d60`: pointer to the value byte of the setting object it is called
/// on (`+4`; a static zero for a null object).
const SETTING_VALUE: u32 = 0x0040_8d60;
/// The setting object `AddNoteVarArgs` consults (`011de0a4`): notes are
/// recorded only while its byte is not zero. The game's name for it is not
/// known.
const NOTES_ENABLED_SETTING: u32 = 0x011d_e0a4;
/// Size of the stack buffer the note is formatted into.
const NOTE_BUFFER_SIZE: u32 = 1024;

/// `BGSSaveGameBuffer::BGSSaveGameBuffer` / `~BGSSaveGameBuffer`,
/// `Save` (to a file), `SaveString`, and the save-side `SaveGameData`
/// (`(buffer, data, size, 0)`), as the history's `Save` uses them.
const SAVE_BUFFER_CONSTRUCT: u32 = 0x0086_5bd0;
const SAVE_BUFFER_DESTRUCT: u32 = 0x0086_5c10;
const SAVE_BUFFER_SAVE: u32 = 0x0086_5c40;
const SAVE_BUFFER_SAVE_DATA: u32 = 0x0086_5e50;
const SAVE_BUFFER_SAVE_STRING: u32 = 0x0086_5e70;
/// Size of the `BGSSaveGameBuffer` the exe keeps on its stack.
const SAVE_BUFFER_SIZE: u32 = 0x18;
/// `BGSLoadGameBuffer::BGSLoadGameBuffer` / `~BGSLoadGameBuffer`, `Load`
/// (from a file), the load-side `LoadGameData` (`(buffer, &data, size)`) and
/// `LoadString`.
const LOAD_BUFFER_CONSTRUCT: u32 = 0x0086_46b0;
const LOAD_BUFFER_DESTRUCT: u32 = 0x0086_46f0;
const LOAD_BUFFER_LOAD: u32 = 0x0086_4740;
const LOAD_BUFFER_LOAD_DATA: u32 = 0x0086_4980;
const LOAD_BUFFER_LOAD_STRING: u32 = 0x0086_49a0;
/// Size of the `BGSLoadGameBuffer` the exe keeps on its stack.
const LOAD_BUFFER_SIZE: u32 = 0x10;

/// Copies the variable-argument words into game memory as the `va_list`
/// the formatting function takes, runs `body` with its address and frees
/// it. A call without arguments gets an empty (zeroed) list; the game's
/// `AddNote` passes the address of the stack words after its fixed
/// arguments, whatever they are.
fn with_va_list<R>(e: &mut Engine, words: &[u32], body: impl FnOnce(&mut Engine, u32) -> R) -> R {
    let list = e.mem.alloc(4 * words.len().max(1) as u32);
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(list + 4 * i as u32, *word);
    }
    let result = body(e, list);
    e.mem.free(list);
    result
}

// Translated from 0084ded0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadHistory::BGSSaveLoadHistory` (Xbox PDB): constructs the
/// array and reserves room for 0x40 notes.
// Translation incomplete: 0084ded0 - the compiler's exception-unwind frame (FS:[0]) is not translated.
pub fn bgs_save_load_history_construct(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
) -> Ptr<BSSimpleArray> {
    e.call(ARRAY_BASE_CONSTRUCT, &args![this]);
    e.call(ARRAY_SET_RESERVED_SIZE, &args![this, 0x40u32]);
    this
}

// Translated from 0084df90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees every note string (the array size is re-read each time round) and
/// empties the array with `SetSize(0, 0)`. The map has no name for it; it is
/// the body of the destructor.
pub fn fn_0084df90(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    let mut index = 0u32;
    while index < e.call(ARRAY_SIZE, &args![this]).u32() {
        let slot = e.call(ARRAY_ELEMENT, &args![this, index]).u32();
        let note = e.mem.u32(slot);
        e.call(OPERATOR_DELETE, &args![note]);
        index += 1;
    }
    e.call(ARRAY_SET_SIZE, &args![this, 0u32, 0u32]);
}

// Translated from 0084df30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadHistory::~BGSSaveLoadHistory` (the Xbox PDB name is
/// inferred: `00847450`, the scalar deleting destructor called from the
/// owner's destruction, calls it, and `0084ded0` constructs the same
/// member): frees the notes, then runs the array's base destructor.
// Translation incomplete: 0084df30 - the compiler's exception-unwind frame (FS:[0]) is not translated.
pub fn bgs_save_load_history_destruct(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    fn_0084df90(e, this);
    e.call(ARRAY_BASE_DESTRUCT, &args![this]);
}

// Translated from 0084e020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadHistory::AddNoteVarArgs` (Xbox PDB, `__thiscall`, two stack
/// words): when the setting at `011de0a4` is on, formats `format` with the
/// argument list `list` into a 1024-byte buffer and, if that produced text,
/// appends a heap copy of it to the notes.
// Translation incomplete: 0084e020 - the /GS stack security cookie check (00ec408c) is not modeled.
pub fn bgs_save_load_history_add_note_var_args(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    format: u32,
    list: u32,
) {
    let enabled = e.call(SETTING_VALUE, &args![NOTES_ENABLED_SETTING]).u32();
    if e.mem.u8(enabled) == 0 {
        return;
    }
    e.with_stack(NOTE_BUFFER_SIZE, |e, buffer| {
        let length = e.call(FORMAT_LIST, &args![buffer, format, list]).i32();
        if length > 0 {
            let size = (length + 1) as u32;
            let note = e.call(OPERATOR_NEW, &args![size]).u32();
            e.call(COPY_STRING, &args![note, size, buffer]);
            e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), note);
                e.call(ARRAY_ADD, &args![this, local]);
            });
        }
    });
}

// Translated from 0084dff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadHistory::AddNote` (Xbox PDB, cdecl with variable arguments):
/// hands `format` and the address of the argument words to
/// [`bgs_save_load_history_add_note_var_args`].
pub fn bgs_save_load_history_add_note(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    format: u32,
    arguments: &[u32],
) {
    with_va_list(e, arguments, |e, list| {
        bgs_save_load_history_add_note_var_args(e, this, format, list)
    });
}

/// Registered form of [`bgs_save_load_history_add_note`]: `this`, the
/// format, then any number of argument words.
fn add_note_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() >= 2,
        "AddNote needs this and the format, got {} words",
        words.len()
    );
    bgs_save_load_history_add_note(e, Ptr::new(words[0]), words[1], &words[2..]);
    Ret::default()
}

// Translated from 0084e0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadHistory::Save` (Xbox PDB, `__thiscall`, one stack word):
/// writes the note count (4 bytes) and each note string through a
/// `BGSSaveGameBuffer` local, then saves the buffer into `file`.
// Translation incomplete: 0084e0d0 - the compiler's exception-unwind frame (FS:[0]) is not translated.
pub fn bgs_save_load_history_save(e: &mut Engine, this: Ptr<BSSimpleArray>, file: Ptr) {
    e.with_stack(SAVE_BUFFER_SIZE, |e, buffer| {
        e.call(SAVE_BUFFER_CONSTRUCT, &args![buffer]);
        e.with_stack(4, |e, count| {
            let size = e.call(ARRAY_SIZE, &args![this]).u32();
            e.mem.set_u32(count.addr(), size);
            e.call(SAVE_BUFFER_SAVE_DATA, &args![buffer, count, 4u32, 0u32]);
            let mut index = 0u32;
            while index < e.mem.u32(count.addr()) {
                let slot = e.call(ARRAY_ELEMENT, &args![this, index]).u32();
                let note = e.mem.u32(slot);
                e.call(SAVE_BUFFER_SAVE_STRING, &args![buffer, note, 0u32]);
                index += 1;
            }
        });
        e.call(SAVE_BUFFER_SAVE, &args![buffer, file]);
        e.call(SAVE_BUFFER_DESTRUCT, &args![buffer]);
    });
}

// Translated from 0084e190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadHistory::Load` (Xbox PDB, `__thiscall`, one stack word):
/// loads `file` into a `BGSLoadGameBuffer` local, reads the note count
/// (4 bytes) and appends each loaded string to the notes.
// Translation incomplete: 0084e190 - the compiler's exception-unwind frame (FS:[0]) is not translated.
pub fn bgs_save_load_history_load(e: &mut Engine, this: Ptr<BSSimpleArray>, file: Ptr) {
    e.with_stack(LOAD_BUFFER_SIZE, |e, buffer| {
        e.call(LOAD_BUFFER_CONSTRUCT, &args![buffer]);
        e.call(LOAD_BUFFER_LOAD, &args![buffer, file]);
        e.with_stack(4, |e, count| {
            e.call(LOAD_BUFFER_LOAD_DATA, &args![buffer, count, 4u32]);
            let mut index = 0u32;
            while index < e.mem.u32(count.addr()) {
                let note = e.call(LOAD_BUFFER_LOAD_STRING, &args![buffer, 0u32]).u32();
                e.with_stack(4, |e, local| {
                    e.mem.set_u32(local.addr(), note);
                    e.call(ARRAY_ADD, &args![this, local]);
                });
                index += 1;
            }
        });
        e.call(LOAD_BUFFER_DESTRUCT, &args![buffer]);
    });
}

// Translated from 0084e250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadHistory::Rewind` (Xbox PDB, `__thiscall`, one stack word):
/// when `index` is below the note count, frees the notes from `index` on
/// and removes them from the array.
pub fn bgs_save_load_history_rewind(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32) {
    let count = e.call(ARRAY_SIZE_OV2, &args![this]).u32();
    if index >= count {
        return;
    }
    for note_index in index..count {
        let slot = e.call(ARRAY_ELEMENT, &args![this, note_index]).u32();
        let note = e.mem.u32(slot);
        e.call(OPERATOR_DELETE, &args![note]);
    }
    e.call(ARRAY_MOVE_ITEMS, &args![this, index, count - index]);
}

// Translated from 0084e2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadHistory::AddHistory` (Xbox PDB, `__thiscall`, one stack
/// word): adds each note of the history `other` to this one through
/// `AddNote`, as the format string with no arguments.
pub fn bgs_save_load_history_add_history(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    other: Ptr<BSSimpleArray>,
) {
    let count = e.call(ARRAY_SIZE_OV2, &args![other]).u32();
    for index in 0..count {
        let note = e.call(ARRAY_STRING, &args![other, index]).u32();
        bgs_save_load_history_add_note(e, this, note, &[]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0084ded0,
            bgs_save_load_history_construct(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x0084df30,
            bgs_save_load_history_destruct(Ptr<BSSimpleArray>)
        ),
        entry!(0x0084df90, fn_0084df90(Ptr<BSSimpleArray>)),
        (0x0084dff0, add_note_entry as AbiFn),
        entry!(
            0x0084e020,
            bgs_save_load_history_add_note_var_args(Ptr<BSSimpleArray>, u32, u32)
        ),
        entry!(
            0x0084e0d0,
            bgs_save_load_history_save(Ptr<BSSimpleArray>, Ptr)
        ),
        entry!(
            0x0084e190,
            bgs_save_load_history_load(Ptr<BSSimpleArray>, Ptr)
        ),
        entry!(
            0x0084e250,
            bgs_save_load_history_rewind(Ptr<BSSimpleArray>, u32)
        ),
        entry!(
            0x0084e2c0,
            bgs_save_load_history_add_history(Ptr<BSSimpleArray>, Ptr<BSSimpleArray>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn word_ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An array object whose buffer holds `notes` (as fake string
    /// addresses) and doubles for the array helpers that read it.
    fn history(e: &mut Engine, notes: &[u32]) -> Ptr<BSSimpleArray> {
        let array = e.new_object::<BSSimpleArray>();
        let buffer = e.mem.alloc(4 * notes.len().max(1) as u32);
        for (i, note) in notes.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *note);
        }
        e.mem.set_u32(array.addr() + 4, buffer);
        e.mem.set_u32(array.addr() + 8, notes.len() as u32);
        for size_getter in [0x0044_ddc0, 0x0062_0b80] {
            e.register(size_getter, |e, a| word_ret(e.mem.u32(a[0] + 8)));
        }
        e.register(0x006a_7ad0, |e, a| word_ret(e.mem.u32(a[0] + 4) + 4 * a[1]));
        e.register(0x0084_e320, |e, a| {
            word_ret(e.mem.u32(e.mem.u32(a[0] + 4) + 4 * a[1]))
        });
        array
    }

    #[test]
    fn constructor_builds_the_array_and_reserves_0x40() {
        let mut e = Engine::new();
        let array = e.new_object::<BSSimpleArray>();
        e.register(0x0082_2600, |_, _| Ret::default());
        e.register(0x0084_b5c0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0084_ded0, &[array.addr()]).u32(), array.addr());
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![
                (0x0084_ded0, vec![array.addr()]),
                (0x0082_2600, vec![array.addr()]),
                (0x0084_b5c0, vec![array.addr(), 0x40]),
            ]
        );
    }

    #[test]
    fn destructor_and_clear_free_every_note_then_empty_the_array() {
        let mut e = Engine::new();
        let array = history(&mut e, &[0x111, 0x222]);
        e.register(0x0040_1030, |_, _| Ret::default());
        e.register(0x006f_2290, |_, _| Ret::default());
        e.register(0x0082_2630, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0084_df30, &[array.addr()]);
        let log = e.call_log.take().unwrap();
        let freed: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == 0x0040_1030)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(freed, [0x111, 0x222]);
        let tail: Vec<(u32, Vec<u32>)> = log[log.len() - 2..].to_vec();
        assert_eq!(
            tail,
            vec![
                (0x006f_2290, vec![array.addr(), 0, 0]),
                (0x0082_2630, vec![array.addr()]),
            ]
        );
        // The clear on its own does not run the base destructor.
        e.call_log = Some(vec![]);
        e.call(0x0084_df90, &[array.addr()]);
        assert!(!e
            .call_log
            .take()
            .unwrap()
            .iter()
            .any(|c| c.0 == 0x0082_2630));
    }

    fn add_note_doubles(e: &mut Engine, enabled: u8) -> Rc<RefCell<Vec<u32>>> {
        let setting = e.mem.alloc(8);
        e.mem.set_u8(setting + 4, enabled);
        e.register_double(0x0040_8d60, move |_, a| {
            assert_eq!(a[0], 0x011d_e0a4);
            word_ret(setting + 4)
        });
        // The formatter writes "ab" and reports its length; the format and
        // list arguments are recorded.
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = seen.clone();
        e.register_double(0x0040_7000, move |e, a| {
            record.borrow_mut().extend_from_slice(&a[1..]);
            e.mem.write(a[0], b"ab\0");
            word_ret(2)
        });
        e.register(0x0040_1000, |e, a| {
            assert_eq!(a[0], 3);
            word_ret(e.mem.alloc(a[0]))
        });
        e.register(0x0040_6d30, |e, a| {
            assert_eq!(a[1], 3);
            let text = e.mem.bytes(a[2], 3);
            e.mem.write(a[0], &text);
            Ret::default()
        });
        e.register(0x007c_b2e0, |e, a| {
            // Appends the word the pointer refers to.
            let size = e.mem.u32(a[0] + 8);
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(buffer + 4 * size, e.mem.u32(a[1]));
            e.mem.set_u32(a[0] + 8, size + 1);
            word_ret(size)
        });
        seen
    }

    #[test]
    fn add_note_formats_and_appends_a_heap_copy() {
        let mut e = Engine::new();
        let array = history(&mut e, &[]);
        let buffer = e.mem.alloc(16);
        e.mem.set_u32(array.addr() + 4, buffer);
        let seen = add_note_doubles(&mut e, 1);
        // Variable arguments: the format and two words, copied to a list.
        e.call(0x0084_dff0, &[array.addr(), 0x5000, 7, 8]);
        assert_eq!(e.mem.u32(array.addr() + 8), 1);
        let note = e.mem.u32(buffer);
        assert_eq!(e.mem.bytes(note, 3), b"ab\0");
        let seen = seen.borrow();
        assert_eq!(seen[0], 0x5000);
        // `seen[1]` is the list's address (freed afterwards).
        assert_eq!(seen.len(), 2);
    }

    #[test]
    fn add_note_var_args_records_nothing_when_off_or_when_the_text_is_empty() {
        let mut e = Engine::new();
        let array = history(&mut e, &[]);
        add_note_doubles(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x0084_e020, &[array.addr(), 0x5000, 0x6000]);
        assert_eq!(
            e.call_log
                .take()
                .unwrap()
                .iter()
                .map(|c| c.0)
                .collect::<Vec<_>>(),
            [0x0084_e020, 0x0040_8d60]
        );
        // On, but the formatter reports no text.
        add_note_doubles(&mut e, 1);
        e.register(0x0040_7000, |_, _| word_ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0084_e020, &[array.addr(), 0x5000, 0x6000]);
        assert_eq!(
            e.call_log
                .take()
                .unwrap()
                .iter()
                .map(|c| c.0)
                .collect::<Vec<_>>(),
            [0x0084_e020, 0x0040_8d60, 0x0040_7000]
        );
        assert_eq!(e.mem.u32(array.addr() + 8), 0);
    }

    #[test]
    fn save_writes_the_count_then_each_string_then_the_buffer() {
        let mut e = Engine::new();
        let array = history(&mut e, &[0x111, 0x222]);
        e.register(0x0086_5bd0, |_, _| Ret::default());
        e.register(0x0086_5c10, |_, _| Ret::default());
        e.register(0x0086_5c40, |_, _| Ret::default());
        e.register(0x0086_5e50, |e, a| {
            // The count is written as 4 bytes from the local.
            assert_eq!((a[2], a[3], e.mem.u32(a[1])), (4, 0, 2));
            Ret::default()
        });
        e.register(0x0086_5e70, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0084_e0d0, &[array.addr(), 0x7000]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            calls,
            [
                0x0084_e0d0,
                0x0086_5bd0,
                0x0044_ddc0,
                0x0086_5e50,
                0x006a_7ad0,
                0x0086_5e70,
                0x006a_7ad0,
                0x0086_5e70,
                0x0086_5c40,
                0x0086_5c10,
            ]
        );
        let buffer = log[1].1[0];
        assert_eq!(log[5].1, vec![buffer, 0x111, 0]);
        assert_eq!(log[7].1, vec![buffer, 0x222, 0]);
        assert_eq!(log[8].1, vec![buffer, 0x7000]);
        assert_eq!(log[9].1, vec![buffer]);
    }

    #[test]
    fn load_reads_the_count_then_appends_each_string() {
        let mut e = Engine::new();
        let array = history(&mut e, &[]);
        let buffer = e.mem.alloc(16);
        e.mem.set_u32(array.addr() + 4, buffer);
        e.register(0x0086_46b0, |_, _| Ret::default());
        e.register(0x0086_46f0, |_, _| Ret::default());
        e.register(0x0086_4740, |_, _| Ret::default());
        e.register(0x0086_4980, |e, a| {
            assert_eq!(a[2], 4);
            e.mem.set_u32(a[1], 2);
            Ret::default()
        });
        let next = Rc::new(RefCell::new(0x300u32));
        e.register_double(0x0086_49a0, move |_, a| {
            assert_eq!(a[1], 0);
            let mut next = next.borrow_mut();
            *next += 0x100;
            word_ret(*next)
        });
        e.register(0x007c_b2e0, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(buffer + 4 * size, e.mem.u32(a[1]));
            e.mem.set_u32(a[0] + 8, size + 1);
            word_ret(size)
        });
        e.call_log = Some(vec![]);
        e.call(0x0084_e190, &[array.addr(), 0x7000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1].0, 0x0086_46b0);
        assert_eq!((log[2].0, log[2].1[1]), (0x0086_4740, 0x7000));
        assert_eq!(log.last().unwrap().0, 0x0086_46f0);
        assert_eq!(e.mem.u32(array.addr() + 8), 2);
        assert_eq!(e.mem.u32(buffer), 0x400);
        assert_eq!(e.mem.u32(buffer + 4), 0x500);
    }

    #[test]
    fn rewind_frees_from_the_index_and_removes_them() {
        let mut e = Engine::new();
        let array = history(&mut e, &[0x111, 0x222, 0x333]);
        e.register(0x0040_1030, |_, _| Ret::default());
        e.register(0x009a_4320, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0084_e250, &[array.addr(), 1]);
        let log = e.call_log.take().unwrap();
        let freed: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == 0x0040_1030)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(freed, [0x222, 0x333]);
        assert_eq!(
            log.last().unwrap(),
            &(0x009a_4320, vec![array.addr(), 1, 2])
        );

        // An index at or past the count does nothing beyond reading it.
        e.call_log = Some(vec![]);
        e.call(0x0084_e250, &[array.addr(), 3]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn add_history_adds_every_note_of_the_other_history() {
        let mut e = Engine::new();
        let other = history(&mut e, &[0x111, 0x222]);
        let array = e.new_object::<BSSimpleArray>();
        e.register(0x0040_8d60, |_, _| word_ret(0x1000));
        e.mem.map(0x1000, 4);
        e.mem.set_u8(0x1000, 0); // Setting off: AddNote returns early.
        e.call_log = Some(vec![]);
        e.call(0x0084_e2c0, &[array.addr(), other.addr()]);
        let log = e.call_log.take().unwrap();
        let strings: Vec<(u32, u32)> = log
            .iter()
            .filter(|c| c.0 == 0x0084_e320)
            .map(|c| (c.1[0], c.1[1]))
            .collect();
        assert_eq!(strings, [(other.addr(), 0), (other.addr(), 1)]);
        assert_eq!(log.iter().filter(|c| c.0 == 0x0040_8d60).count(), 2);
    }
}
