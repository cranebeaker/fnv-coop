//! `SpeedTree/fileaccess.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `CTreeFileAccess`, the cursor over a tree file held in memory (`m_nIndex` at `+0` and the
//! file's bytes in a `std::vector<unsigned char>` at `+4`, whose first and last pointers sit at
//! `+0x10` and `+0x14`; layout in `billboardleaf`): the readers of a float, an int, a string and
//! a 3-float vector, and a reader of an int that also skips the next word.
//!
//! x87 note: `ParseFloat` returns the float in `ST0` (`FLD dword`), so a signalling NaN comes
//! out quiet ([`x87_quiet_bits`]); `ParseVector3` stores what it gets (`FSTP dword`).

#[allow(unused_imports)]
use crate::prelude::*;

use super::billboardleaf::CTreeFileAccess;
use super::branch::INVALID_PARAMETER;

/// The string's `_Tidy(built, new_size)` (`0044a730`, `thiscall`), its `assign(string, pos,
/// count)` (`0044a580`, `thiscall`), the append of `count` copies of a character (`00aadeb0`,
/// `thiscall`: the count, the character) and `std::string::npos` (`010173f0`, read from memory).
const STRING_TIDY: u32 = 0x0044_a730;
const STRING_ASSIGN: u32 = 0x0044_a580;
const STRING_APPEND_CHARS: u32 = 0x00aa_deb0;
const STRING_NPOS: u32 = 0x0101_73f0;

/// The checked read position of a word: the index, advanced by 4, must be below the file's byte
/// count (`last - first`), else the invalid-parameter path runs; returns the address of the
/// word at the old index.
fn next_word(e: &mut Engine, this: Ptr<CTreeFileAccess>) -> u32 {
    let index = e.get(this, CTreeFileAccess::m_nIndex);
    let size = e
        .get(this, CTreeFileAccess::m_vMemoryLast)
        .addr()
        .wrapping_sub(e.get(this, CTreeFileAccess::m_vMemoryFirst).addr());
    if index >= size {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let first = e.get(this, CTreeFileAccess::m_vMemoryFirst).addr();
    let index = e.get(this, CTreeFileAccess::m_nIndex);
    e.set(this, CTreeFileAccess::m_nIndex, index.wrapping_add(4));
    first.wrapping_add(index)
}

/// The bits `ParseFloat` leaves in `ST0`: the float at the next word through `FLD dword`.
fn parse_float_bits(e: &mut Engine, this: Ptr<CTreeFileAccess>) -> u32 {
    let word = next_word(e, this);
    x87_quiet_bits(e.mem.u32(word))
}

// Translated from 00b3ed20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CTreeFileAccess::ParseFloat` (Xbox PDB; `fastcall`, the float in `ST0`): reads the float at
/// the cursor and advances it by 4; a cursor at or past the end of the file takes the
/// invalid-parameter path.
pub fn c_tree_file_access_parse_float(e: &mut Engine, this: Ptr<CTreeFileAccess>) -> f32 {
    f32::from_bits(parse_float_bits(e, this))
}

// Translated from 00b3ed80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CTreeFileAccess::ParseInt` (Xbox PDB; `fastcall`): reads the int at the cursor and advances
/// it by 4; a cursor at or past the end of the file takes the invalid-parameter path.
pub fn c_tree_file_access_parse_int(e: &mut Engine, this: Ptr<CTreeFileAccess>) -> i32 {
    let word = next_word(e, this);
    e.mem.i32(word)
}

// Translated from 00b3ede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the int at the cursor and advances the cursor by 8 (identified from the code, the
/// engine map has no name; `fastcall`): the int is the first of two words, the second skipped.
/// A cursor at or past the end of the file takes the invalid-parameter path.
pub fn fn_00b3ede0(e: &mut Engine, this: Ptr<CTreeFileAccess>) -> i32 {
    let word = next_word(e, this);
    let index = e.get(this, CTreeFileAccess::m_nIndex);
    e.set(this, CTreeFileAccess::m_nIndex, index.wrapping_add(4));
    e.mem.i32(word)
}

// Translated from 00b3ee40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3ee40 - the FS:[0] exception frame (destroying the local string if a
// step throws) and the /GS cookie check (`00ec408c`) are not translated.
/// `CTreeFileAccess::ParseString` (Xbox PDB; `thiscall`: the string to fill; the exe returns it
/// in `EAX`): builds an empty local string (`0044a730` with 0 and 0), reads a count
/// (`ParseInt`) and appends that many bytes read at the cursor (one at a time, the cursor
/// advancing by 1, each checked against the end of the file) as characters (`00aadeb0`), then
/// resets `out` to the empty string (`0044a730` with 0 and 0), assigns it the whole local
/// (`0044a580`) and tidies the local (`0044a730` with 1 and 0). Returns `out`.
pub fn c_tree_file_access_parse_string(
    e: &mut Engine,
    this: Ptr<CTreeFileAccess>,
    out: Ptr,
) -> Ptr {
    e.with_stack(0x1C, |e, local| {
        e.call(STRING_TIDY, &args![local, 0u32, 0u32]);
        let count = c_tree_file_access_parse_int(e, this);
        let mut at: i32 = 0;
        while at < count {
            let index = e.get(this, CTreeFileAccess::m_nIndex);
            e.set(this, CTreeFileAccess::m_nIndex, index.wrapping_add(1));
            let size = e
                .get(this, CTreeFileAccess::m_vMemoryLast)
                .addr()
                .wrapping_sub(e.get(this, CTreeFileAccess::m_vMemoryFirst).addr());
            if index >= size {
                e.call(INVALID_PARAMETER, &args![]);
            }
            let byte = e.mem.u8(e
                .get(this, CTreeFileAccess::m_vMemoryFirst)
                .addr()
                .wrapping_add(index));
            e.call(STRING_APPEND_CHARS, &args![local, 1u32, u32::from(byte)]);
            at += 1;
        }
        e.call(STRING_TIDY, &args![out, 0u32, 0u32]);
        let npos = e.global::<u32>(STRING_NPOS);
        e.call(STRING_ASSIGN, &args![out, local, 0u32, npos]);
        e.call(STRING_TIDY, &args![local, 1u32, 0u32]);
    });
    out
}

// Translated from 00b3ef80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CTreeFileAccess::ParseVector3` (Xbox PDB; `thiscall`: the vector to fill): reads three
/// floats (`ParseFloat`, each stored through the x87 stack into a zero-initialised local) and
/// copies them to `out`. Returns `out`.
pub fn c_tree_file_access_parse_vector3(
    e: &mut Engine,
    this: Ptr<CTreeFileAccess>,
    out: Ptr,
) -> Ptr {
    let mut local = [0u32; 3];
    for slot in local.iter_mut() {
        *slot = parse_float_bits(e, this);
    }
    for (i, bits) in local.iter().enumerate() {
        e.mem.set_u32(out.addr() + 4 * i as u32, *bits);
    }
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b3ed20,
            c_tree_file_access_parse_float(Ptr<CTreeFileAccess>) -> f32
        ),
        entry!(
            0x00b3ed80,
            c_tree_file_access_parse_int(Ptr<CTreeFileAccess>) -> i32
        ),
        entry!(0x00b3ede0, fn_00b3ede0(Ptr<CTreeFileAccess>) -> i32),
        entry!(
            0x00b3ee40,
            c_tree_file_access_parse_string(Ptr<CTreeFileAccess>, Ptr) -> Ptr
        ),
        entry!(
            0x00b3ef80,
            c_tree_file_access_parse_vector3(Ptr<CTreeFileAccess>, Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        e.register(INVALID_PARAMETER, |_, _| panic!("invalid parameter"));
        e
    }

    fn panics(action: impl FnOnce()) -> bool {
        catch_unwind(AssertUnwindSafe(action)).is_err()
    }

    /// A file access over `bytes`, the cursor at `index`.
    fn file(e: &mut Engine, bytes: &[u8], index: u32) -> Ptr<CTreeFileAccess> {
        let file: Ptr<CTreeFileAccess> = e.new_object();
        let memory = e.mem.alloc(bytes.len().max(1) as u32);
        e.mem.write(memory, bytes);
        e.set(file, CTreeFileAccess::m_nIndex, index);
        e.set(file, CTreeFileAccess::m_vMemoryFirst, Ptr::new(memory));
        e.set(
            file,
            CTreeFileAccess::m_vMemoryLast,
            Ptr::new(memory + bytes.len() as u32),
        );
        file
    }

    fn words(values: &[u32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    #[test]
    fn floats_and_ints_advance_the_cursor() {
        let mut e = engine();
        let f = file(&mut e, &words(&[1.5f32.to_bits(), 0xFFFF_FFFE]), 0);
        let r = e.call(0x00b3_ed20, &args![f]);
        assert_eq!(r.f32(), 1.5);
        assert_eq!(e.get(f, CTreeFileAccess::m_nIndex), 4);
        let r = e.call(0x00b3_ed80, &args![f]);
        assert_eq!(r.i32(), -2);
        assert_eq!(e.get(f, CTreeFileAccess::m_nIndex), 8);
        // The cursor at the end is an invalid parameter, for both readers.
        assert!(panics(|| {
            c_tree_file_access_parse_int(&mut e, f);
        }));
        assert!(panics(|| {
            c_tree_file_access_parse_float(&mut e, f);
        }));
        assert_eq!(e.get(f, CTreeFileAccess::m_nIndex), 8);
    }

    #[test]
    fn a_float_comes_out_quiet() {
        let mut e = engine();
        let f = file(&mut e, &words(&[0x7F80_0001]), 0);
        let r = e.call(0x00b3_ed20, &args![f]);
        assert_eq!(r.f32().to_bits(), 0x7FC0_0001);
    }

    #[test]
    fn read_and_skip_advances_by_eight() {
        let mut e = engine();
        let f = file(&mut e, &words(&[7, 8, 9]), 0);
        assert_eq!(e.call(0x00b3_ede0, &args![f]).i32(), 7);
        assert_eq!(e.get(f, CTreeFileAccess::m_nIndex), 8);
        assert_eq!(e.call(0x00b3_ede0, &args![f]).i32(), 9);
        assert_eq!(e.get(f, CTreeFileAccess::m_nIndex), 16);
        assert!(panics(|| {
            fn_00b3ede0(&mut e, f);
        }));
    }

    #[test]
    fn vector3_reads_three_floats() {
        let mut e = engine();
        let f = file(
            &mut e,
            &words(&[1.0f32.to_bits(), 0x7F80_0001, (-2.5f32).to_bits(), 9]),
            0,
        );
        let out = e.mem.alloc(12);
        let r = e.call(0x00b3_ef80, &args![f, Ptr::<()>::new(out)]);
        assert_eq!(r.u32(), out);
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(e.mem.u32(out + 4), 0x7FC0_0001);
        assert_eq!(e.mem.f32(out + 8), -2.5);
        assert_eq!(e.get(f, CTreeFileAccess::m_nIndex), 12);
        // Fewer than three floats left: the third read is invalid.
        let short = file(&mut e, &words(&[1, 2]), 0);
        assert!(panics(|| {
            c_tree_file_access_parse_vector3(&mut e, short, Ptr::new(out));
        }));
    }

    fn string_doubles(e: &mut Engine) -> Calls {
        let log: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in [STRING_TIDY, STRING_ASSIGN, STRING_APPEND_CHARS] {
            let sink = log.clone();
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        log
    }

    #[test]
    fn string_is_built_from_the_counted_bytes() {
        let mut e = engine();
        let log = string_doubles(&mut e);
        e.set_global(STRING_NPOS, 0xFFFF_FFFFu32);
        let mut bytes = words(&[3]);
        bytes.extend_from_slice(b"abcZ");
        let f = file(&mut e, &bytes, 0);
        let out = e.mem.alloc(0x1C);
        let r = e.call(0x00b3_ee40, &args![f, Ptr::<()>::new(out)]);
        assert_eq!(r.u32(), out);
        assert_eq!(e.get(f, CTreeFileAccess::m_nIndex), 7);
        let calls = log.borrow().clone();
        let local = calls[0].1[0];
        assert_eq!(calls[0], (STRING_TIDY, vec![local, 0, 0]));
        let appended: Vec<(u32, Vec<u32>)> = ["a", "b", "c"]
            .iter()
            .map(|c| {
                (
                    STRING_APPEND_CHARS,
                    vec![local, 1, u32::from(c.as_bytes()[0])],
                )
            })
            .collect();
        assert_eq!(calls[1..4], appended[..]);
        assert_eq!(calls[4], (STRING_TIDY, vec![out, 0, 0]));
        assert_eq!(calls[5], (STRING_ASSIGN, vec![out, local, 0, 0xFFFF_FFFF]));
        assert_eq!(calls[6], (STRING_TIDY, vec![local, 1, 0]));
        assert_eq!(calls.len(), 7);
    }

    #[test]
    fn string_with_a_count_beyond_the_file_is_invalid() {
        let mut e = engine();
        string_doubles(&mut e);
        // A count of 0 builds the empty string; a count of 5 over two bytes runs out.
        let f = file(&mut e, &words(&[0]), 0);
        let out = e.mem.alloc(0x1C);
        e.call(0x00b3_ee40, &args![f, Ptr::<()>::new(out)]);
        assert_eq!(e.get(f, CTreeFileAccess::m_nIndex), 4);
        let mut bytes = words(&[5]);
        bytes.extend_from_slice(b"ab");
        let f = file(&mut e, &bytes, 0);
        assert!(panics(|| {
            c_tree_file_access_parse_string(&mut e, f, Ptr::new(out));
        }));
        // A negative count appends nothing.
        let f = file(&mut e, &words(&[0xFFFF_FFFF]), 0);
        e.call(0x00b3_ee40, &args![f, Ptr::<()>::new(out)]);
        assert_eq!(e.get(f, CTreeFileAccess::m_nIndex), 4);
    }
}
