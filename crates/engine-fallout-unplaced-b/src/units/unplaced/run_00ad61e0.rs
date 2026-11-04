//! `(unplaced)/run_00ad61e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// A vector of `std::string`s as these functions use it: the allocator at
// +0x08, the first string at +0x0C, the end of the strings at +0x10 and the
// end of the storage at +0x14; a string is 0x1C bytes.

/// The size of a `std::string`.
const STRING_SIZE: u32 = 0x1c;
/// The `std::string` assignment from part of another string (`0044a580`,
/// thiscall: source, position, count).
const STRING_ASSIGN: u32 = 0x0044_a580;
/// `0044a730` (thiscall, two words): called with (0, 0) on a string being
/// set up empty and with (1, 0) on one going away.
const STRING_TIDY: u32 = 0x0044_a730;
/// `std::string::npos` (`010173f0`, all ones in the exe).
const STRING_NPOS: u32 = 0x0101_73f0;
/// Copies strings over others (`00ad5a80`, cdecl, six words).
const COPY_STRINGS: u32 = 0x00ad_5a80;
/// Swaps two strings (`00ad6bb0`, thiscall: the other string).
const SWAP_STRINGS: u32 = 0x00ad_6bb0;
/// The vector's clear (`00acac40`, thiscall) and its storage allocation
/// for a count (`00aced20`, thiscall; answers in AL), both placed in
/// `FaceGen/algs.obj` by the engine map.
const VECTOR_CLEAR: u32 = 0x00ac_ac40;
const VECTOR_ALLOCATE: u32 = 0x00ac_ed20;
/// Frees a block (`00401030`, cdecl).
const FREE: u32 = 0x0040_1030;

/// Vector fields.
const VECTOR_ALLOCATOR: u32 = 0x8;
const VECTOR_FIRST: u32 = 0xc;
const VECTOR_LAST: u32 = 0x10;
const VECTOR_END: u32 = 0x14;

/// The number of strings between two addresses (signed division).
fn string_count(first: u32, last: u32) -> u32 {
    ((last.wrapping_sub(first) as i32) / STRING_SIZE as i32) as u32
}

/// Gives each string of `[first, last)` its going-away call (`0044a730`
/// with 1, 0). The code's free of each one after it tests a constant 0,
/// which the compiler folded, so it never runs and is left out.
fn destroy_strings(e: &mut Engine, first: u32, last: u32) {
    let mut at = first;
    while at != last {
        e.call(STRING_TIDY, &[at, 1, 0]);
        at = at.wrapping_add(STRING_SIZE);
    }
}

// Translated from 00ad61e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the vector of strings `other` to `this` (thiscall; a
/// `std::vector<std::string>::operator=` shape, called by `00ad26f0`,
/// `00ad5180` and `00ad60c0`): nothing when they are the same; cleared
/// (`00acac40`) when `other` is empty; when `other` has no more strings than
/// `this`, copies them over (`00ad5a80`), lets the rest go and moves the end;
/// when they fit in the storage, copies over the strings `this` has
/// ([`fn_00ad6690`]) and constructs the rest after them ([`fn_00ad68c0`]);
/// otherwise lets every string go, frees the storage (`00401030`), and when
/// `00aced20` gets storage for the count, constructs them all. Answers
/// `this`. The iterator-tag words the code pushes after the ranges (zero or
/// uninitialized stack bytes) are never read by the callees and are 0 here.
pub fn fn_00ad61e0(e: &mut Engine, this: u32, other: u32) -> u32 {
    if this == other {
        return this;
    }
    let other_first = e.mem.u32(other + VECTOR_FIRST);
    let other_last = e.mem.u32(other + VECTOR_LAST);
    if string_count(other_first, other_last) == 0 {
        e.call(VECTOR_CLEAR, &[this]);
        return this;
    }
    let wanted = string_count(other_first, other_last);
    let have = string_count(
        e.mem.u32(this + VECTOR_FIRST),
        e.mem.u32(this + VECTOR_LAST),
    );
    if wanted <= have {
        let first = e.mem.u32(this + VECTOR_FIRST);
        let other_last = e.mem.u32(other + VECTOR_LAST);
        let other_first = e.mem.u32(other + VECTOR_FIRST);
        let copied = e
            .call(COPY_STRINGS, &[other_first, other_last, first, 0, 0, 0])
            .u32();
        let last = e.mem.u32(this + VECTOR_LAST);
        destroy_strings(e, copied, last);
        let count = string_count(
            e.mem.u32(other + VECTOR_FIRST),
            e.mem.u32(other + VECTOR_LAST),
        );
        let end = count
            .wrapping_mul(STRING_SIZE)
            .wrapping_add(e.mem.u32(this + VECTOR_FIRST));
        e.mem.set_u32(this + VECTOR_LAST, end);
        return this;
    }
    let wanted = string_count(
        e.mem.u32(other + VECTOR_FIRST),
        e.mem.u32(other + VECTOR_LAST),
    );
    let capacity = if e.mem.u32(this + VECTOR_FIRST) == 0 {
        0
    } else {
        string_count(e.mem.u32(this + VECTOR_FIRST), e.mem.u32(this + VECTOR_END))
    };
    if wanted <= capacity {
        let have = string_count(
            e.mem.u32(this + VECTOR_FIRST),
            e.mem.u32(this + VECTOR_LAST),
        );
        let middle = have
            .wrapping_mul(STRING_SIZE)
            .wrapping_add(e.mem.u32(other + VECTOR_FIRST));
        let first = e.mem.u32(this + VECTOR_FIRST);
        let other_first = e.mem.u32(other + VECTOR_FIRST);
        fn_00ad6690(e, other_first, middle, first);
        let last = e.mem.u32(this + VECTOR_LAST);
        let other_last = e.mem.u32(other + VECTOR_LAST);
        let end = fn_00ad68c0(e, middle, other_last, last, this + VECTOR_ALLOCATOR, 0, 0);
        e.mem.set_u32(this + VECTOR_LAST, end);
        return this;
    }
    if e.mem.u32(this + VECTOR_FIRST) != 0 {
        let last = e.mem.u32(this + VECTOR_LAST);
        let first = e.mem.u32(this + VECTOR_FIRST);
        destroy_strings(e, first, last);
        // The code also divides the storage size here and never uses it.
        let _capacity = string_count(e.mem.u32(this + VECTOR_FIRST), e.mem.u32(this + VECTOR_END));
        let first = e.mem.u32(this + VECTOR_FIRST);
        e.call(FREE, &[first]);
    }
    let count = string_count(
        e.mem.u32(other + VECTOR_FIRST),
        e.mem.u32(other + VECTOR_LAST),
    );
    if e.call(VECTOR_ALLOCATE, &[this, count]).u8() != 0 {
        let first = e.mem.u32(this + VECTOR_FIRST);
        let other_last = e.mem.u32(other + VECTOR_LAST);
        let other_first = e.mem.u32(other + VECTOR_FIRST);
        let end = fn_00ad68c0(
            e,
            other_first,
            other_last,
            first,
            this + VECTOR_ALLOCATOR,
            0,
            0,
        );
        e.mem.set_u32(this + VECTOR_LAST, end);
    }
    this
}

// Translated from 00ad6690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the strings `[first, last)` over the ones from `dest` on (cdecl,
/// three words; called by [`fn_00ad61e0`]): `00ad5a80` with them and its
/// three iterator-tag words (a 0 and two uninitialized stack bytes, which
/// it never reads; 0 here), whose answer it passes on.
pub fn fn_00ad6690(e: &mut Engine, first: u32, last: u32, dest: u32) -> u32 {
    e.call(COPY_STRINGS, &[first, last, dest, 0, 0, 0]).u32()
}

// Translated from 00ad66e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the strings `[first, last)` into new strings from `dest` on
/// (cdecl, six words, the last three never read; called by `00ad07c0` and
/// `00ad5dd0`): with a local empty string (`0044a730` with 0, 0), each new
/// string is constructed as its copy ([`fn_00ad6990`]) and then swapped with
/// the source (`00ad6bb0`); the local goes away (`0044a730` with 1, 0).
/// Answers the end of the new strings. The exception handler that would
/// destroy the strings made so far is not translated.
// Translation incomplete: 00ad66e0 - C++ exception unwinding (SEH frame state, its catch block) and the /GS stack cookie check (00ec408c) are not translated.
pub fn fn_00ad66e0(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    e.with_stack(STRING_SIZE, |e, empty| {
        let empty = empty.addr();
        e.call(STRING_TIDY, &[empty, 0, 0]);
        let mut from = first;
        let mut to = dest;
        while from != last {
            fn_00ad6990(e, to, empty);
            e.call(SWAP_STRINGS, &[to, from]);
            to = to.wrapping_add(STRING_SIZE);
            from = from.wrapping_add(STRING_SIZE);
        }
        e.call(STRING_TIDY, &[empty, 1, 0]);
        to
    })
}

// Translated from 00ad6800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies 0x38-byte items (two strings each, at +0 and +0x1C) from
/// `[first, last)` backwards to end at `dest_end` (cdecl, six words, the
/// last three never read; a `std::copy_backward` shape, called by
/// `00ad6000`): each string assigned with `0044a580(source, 0, npos)`.
/// Answers `dest_end - (last - first) / 0x38 * 0x38` (signed division).
pub fn fn_00ad6800(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest_end: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let count = (last.wrapping_sub(first) as i32) / 0x38;
    let start = dest_end.wrapping_sub((count as u32).wrapping_mul(0x38));
    let mut from = last;
    let mut to = dest_end;
    while first != from {
        from = from.wrapping_sub(0x38);
        to = to.wrapping_sub(0x38);
        let npos = e.mem.u32(STRING_NPOS);
        e.call(STRING_ASSIGN, &[to, from, 0, npos]);
        let npos = e.mem.u32(STRING_NPOS);
        e.call(
            STRING_ASSIGN,
            &[
                to.wrapping_add(STRING_SIZE),
                from.wrapping_add(STRING_SIZE),
                0,
                npos,
            ],
        );
    }
    start
}

// Translated from 00ad68c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs copies of the strings `[first, last)` from `dest` on (cdecl,
/// six words: the ranges, then the allocator and two iterator-tag words,
/// never read; a `std::uninitialized_copy` shape, called by `00ad3150` and
/// [`fn_00ad61e0`]), each through [`fn_00ad6990`]. Answers the end of the
/// new strings. The exception handler that would destroy the strings made
/// so far is not translated.
// Translation incomplete: 00ad68c0 - C++ exception unwinding (SEH frame state, its catch block) is not translated.
pub fn fn_00ad68c0(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let mut from = first;
    let mut to = dest;
    while from != last {
        fn_00ad6990(e, to, from);
        to = to.wrapping_add(STRING_SIZE);
        from = from.wrapping_add(STRING_SIZE);
    }
    to
}

// Translated from 00ad6990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a copy of the string `source` at `place` (cdecl; a placement
/// copy-construction, called by `00ad5b00`, [`fn_00ad66e0`] and
/// [`fn_00ad68c0`]): nothing for a null `place`; otherwise `0044a730` with
/// 0, 0 and the assignment `0044a580(source, 0, npos)`.
// Translation incomplete: 00ad6990 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_00ad6990(e: &mut Engine, place: u32, source: u32) {
    if place == 0 {
        return;
    }
    e.call(STRING_TIDY, &[place, 0, 0]);
    let npos = e.mem.u32(STRING_NPOS);
    e.call(STRING_ASSIGN, &[place, source, 0, npos]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ad_61e0, fn_00ad61e0(u32, u32) -> u32),
        entry!(0x00ad_6690, fn_00ad6690(u32, u32, u32) -> u32),
        entry!(0x00ad_66e0, fn_00ad66e0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ad_6800, fn_00ad6800(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ad_68c0, fn_00ad68c0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ad_6990, fn_00ad6990(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn string_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.mem.set_u32(STRING_NPOS, u32::MAX);
        for addr in [STRING_ASSIGN, STRING_TIDY, SWAP_STRINGS, VECTOR_CLEAR, FREE] {
            e.register(addr, |_, a| a[0].into_ret());
        }
        // The copy answers the end of the copied range.
        e.register(COPY_STRINGS, |_, a| {
            (a[2] + (a[1] - a[0]) / STRING_SIZE * STRING_SIZE).into_ret()
        });
        e
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A vector with `count` strings at `first` and room for `room`.
    fn vector(e: &mut Engine, first: u32, count: u32, room: u32) -> u32 {
        let v = e.mem.alloc(0x18);
        e.mem.set_u32(v + VECTOR_FIRST, first);
        e.mem.set_u32(v + VECTOR_LAST, first + count * STRING_SIZE);
        e.mem.set_u32(v + VECTOR_END, first + room * STRING_SIZE);
        v
    }

    #[test]
    fn assignment_to_itself_or_from_empty() {
        let mut e = string_engine();
        let this = vector(&mut e, 0x1000, 2, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_61e0, &[this, this]).u32(), this);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        let empty = vector(&mut e, 0x5000, 0, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_61e0, &[this, empty]).u32(), this);
        assert_eq!(
            calls(&e.call_log.take().unwrap(), VECTOR_CLEAR),
            vec![vec![this]]
        );
    }

    #[test]
    fn assignment_from_a_shorter_vector_copies_and_trims() {
        let mut e = string_engine();
        let this = vector(&mut e, 0x1000, 3, 3);
        let other = vector(&mut e, 0x5000, 1, 1);
        e.call_log = Some(vec![]);
        e.call(0x00ad_61e0, &[this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, COPY_STRINGS),
            vec![vec![0x5000, 0x501c, 0x1000, 0, 0, 0]]
        );
        assert_eq!(
            calls(&log, STRING_TIDY),
            vec![vec![0x101c, 1, 0], vec![0x1038, 1, 0]]
        );
        assert_eq!(e.mem.u32(this + VECTOR_LAST), 0x101c);
    }

    #[test]
    fn assignment_within_the_storage_copies_then_constructs() {
        let mut e = string_engine();
        let this = vector(&mut e, 0x1000, 1, 4);
        let other = vector(&mut e, 0x5000, 3, 3);
        e.call_log = Some(vec![]);
        e.call(0x00ad_61e0, &[this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, COPY_STRINGS),
            vec![vec![0x5000, 0x501c, 0x1000, 0, 0, 0]]
        );
        assert_eq!(
            calls(&log, STRING_ASSIGN),
            vec![
                vec![0x101c, 0x501c, 0, u32::MAX],
                vec![0x1038, 0x5038, 0, u32::MAX]
            ]
        );
        assert_eq!(e.mem.u32(this + VECTOR_LAST), 0x1054);
    }

    #[test]
    fn assignment_beyond_the_storage_reallocates() {
        for allocated in [true, false] {
            let mut e = string_engine();
            let this = vector(&mut e, 0x1000, 1, 1);
            let other = vector(&mut e, 0x5000, 2, 2);
            e.register_double(VECTOR_ALLOCATE, move |e, a| {
                if allocated {
                    e.mem.set_u32(a[0] + VECTOR_FIRST, 0x9000);
                }
                u32::from(allocated).into_ret()
            });
            e.call_log = Some(vec![]);
            e.call(0x00ad_61e0, &[this, other]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, STRING_TIDY)[0], vec![0x1000, 1, 0]);
            assert_eq!(calls(&log, FREE), vec![vec![0x1000]]);
            assert_eq!(calls(&log, VECTOR_ALLOCATE), vec![vec![this, 2]]);
            if allocated {
                assert_eq!(calls(&log, STRING_ASSIGN).len(), 2);
                assert_eq!(e.mem.u32(this + VECTOR_LAST), 0x9038);
            } else {
                assert!(calls(&log, STRING_ASSIGN).is_empty());
            }
        }
        // An empty vector without storage frees nothing.
        let mut e = string_engine();
        e.register(VECTOR_ALLOCATE, |_, _| 0u32.into_ret());
        let this = vector(&mut e, 0, 0, 0);
        let other = vector(&mut e, 0x5000, 1, 1);
        e.call_log = Some(vec![]);
        e.call(0x00ad_61e0, &[this, other]);
        assert!(calls(&e.call_log.take().unwrap(), FREE).is_empty());
    }

    #[test]
    fn copy_wrapper_passes_the_answer_on() {
        let mut e = string_engine();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_6690, &[0x5000, 0x5038, 0x1000]).u32(), 0x1038);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (COPY_STRINGS, vec![0x5000, 0x5038, 0x1000, 0, 0, 0])
        );
    }

    #[test]
    fn moves_construct_and_swap() {
        let mut e = string_engine();
        e.call_log = Some(vec![]);
        let end = e
            .call(0x00ad_66e0, &[0x5000, 0x5038, 0x1000, 0, 0, 0])
            .u32();
        assert_eq!(end, 0x1038);
        let log = e.call_log.take().unwrap();
        let empty = log[1].1[0];
        assert_eq!(log[1], (STRING_TIDY, vec![empty, 0, 0]));
        assert_eq!(
            calls(&log, SWAP_STRINGS),
            vec![vec![0x1000, 0x5000], vec![0x101c, 0x501c]]
        );
        assert_eq!(
            calls(&log, STRING_ASSIGN)[1],
            vec![0x101c, empty, 0, u32::MAX]
        );
        assert_eq!(log.last().unwrap(), &(STRING_TIDY, vec![empty, 1, 0]));
    }

    #[test]
    fn pairs_copy_backwards() {
        let mut e = string_engine();
        e.call_log = Some(vec![]);
        let start = e
            .call(0x00ad_6800, &[0x5000, 0x5070, 0x2070, 0, 0, 0])
            .u32();
        assert_eq!(start, 0x2000);
        assert_eq!(
            calls(&e.call_log.take().unwrap(), STRING_ASSIGN),
            vec![
                vec![0x2038, 0x5038, 0, u32::MAX],
                vec![0x2054, 0x5054, 0, u32::MAX],
                vec![0x2000, 0x5000, 0, u32::MAX],
                vec![0x201c, 0x501c, 0, u32::MAX],
            ]
        );
    }

    #[test]
    fn copies_are_constructed_in_place() {
        let mut e = string_engine();
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ad_68c0, &[0x5000, 0x5038, 0x1000, 0, 0, 0])
                .u32(),
            0x1038
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, STRING_TIDY),
            vec![vec![0x1000, 0, 0], vec![0x101c, 0, 0]]
        );
        // A null place constructs nothing.
        e.call_log = Some(vec![]);
        e.call(0x00ad_6990, &[0, 0x5000]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x00ad_6990, &[0x1000, 0x5000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (STRING_TIDY, vec![0x1000, 0, 0]),
                (STRING_ASSIGN, vec![0x1000, 0x5000, 0, u32::MAX])
            ]
        );
    }
}
