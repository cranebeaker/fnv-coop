//! `Havok SDK/hkaanimation.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::reserve_at_least;

/// The animation's vtable (`010ddb3c`).
const ANIMATION_VTABLE: u32 = 0x010d_db3c;
/// `hkReferencedObject::addReference` and `removeReference` (`00c90510`, `00c905b0`, thiscall,
/// no stack words).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkaAnnotationTrack::~hkaAnnotationTrack` (Xbox PDB, `00dfb5a0`, thiscall) and the copy
/// constructor of the same 0x10-byte track (`00e4e420`, thiscall, one word: the source; no PDB
/// name).
const TRACK_DESTRUCT: u32 = 0x00df_b5a0;
const TRACK_COPY_CONSTRUCT: u32 = 0x00e4_e420;
/// The `hkStringPtr` functions: `operator=` (`00cb0c30`, thiscall, one word: the address of the
/// source string pointer), the destructor (`00cb0c00`) and the copy constructor (`00cb0bd0`,
/// one word: the source).
const STRING_ASSIGN: u32 = 0x00cb_0c30;
const STRING_DESTRUCT: u32 = 0x00cb_0c00;
const STRING_COPY_CONSTRUCT: u32 = 0x00cb_0bd0;
/// The `double` the wrapped end of a span is given when the span does not pass the end of the
/// animation (`0104e140`, a very large negative number).
const NO_WRAP: u32 = 0x0104_e140;

// Translated from 00e2f270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaAnimation::clearAllCacheKeys` (Xbox PDB), thiscall, `RET 4`: does nothing (the base
/// animation has no cache; the one stack word is not read).
pub fn hka_animation_clear_all_cache_keys(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

/// The end of a span that passes the end of the animation, as a float: when the animation
/// lasts less than `end` (`FCOMP` with the parity test: ordered less only) `end - duration`,
/// else the `double` at `0104e140`.
fn wrapped_end(e: &Engine, duration: f32, end: f32) -> f32 {
    if f64::from(duration) < f64::from(end) {
        (f64::from(end) - f64::from(duration)) as f32
    } else {
        e.mem.f64(NO_WRAP) as f32
    }
}

/// Whether an annotation at `time` belongs to the span that starts at `start` and ends at
/// `end` (and, when the span wrapped, also to everything below `wrapped`): ordered compares
/// only, so a NaN time never counts.
fn annotation_in_span(time: f32, start: f32, end: f32, wrapped: f32) -> bool {
    let time = f64::from(time);
    if f64::from(start) <= time && time < f64::from(end) {
        return true;
    }
    time < f64::from(wrapped)
}

// Translated from 00e2f280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaAnimation::getNumAnnotations` (Xbox PDB), thiscall, `RET 8` (`time`, `delta`): counts
/// the annotations of all tracks (the track array at +0x1c, count at +0x20, tracks of 0x10
/// bytes with the annotation array's data at +4 and size at +8, annotations of 8 bytes with
/// the time first) that lie in the span `time` to `time + delta` (rounded to a float) or, when
/// that end is past the duration (+0xc), below the wrapped end. The compiler unrolled the
/// loop over the annotations by four; the test is the same for every annotation.
pub fn hka_animation_get_num_annotations(e: &mut Engine, this: Ptr, time: f32, delta: f32) -> i32 {
    let me = this.addr();
    let end = (f64::from(time) + f64::from(delta)) as f32;
    let duration = e.mem.f32(me + 0xc);
    let wrapped = wrapped_end(e, duration, end);
    let track_count = e.mem.i32(me + 0x20);
    let mut count = 0i32;
    if track_count <= 0 {
        return 0;
    }
    let mut track = e.mem.u32(me + 0x1c);
    for _ in 0..track_count {
        let annotations = e.mem.i32(track + 8);
        let data = e.mem.u32(track + 4);
        for index in 0..annotations.max(0) as u32 {
            let annotation_time = e.mem.f32(data.wrapping_add(8 * index));
            if annotation_in_span(annotation_time, time, end, wrapped) {
                count += 1;
            }
        }
        track = track.wrapping_add(0x10);
    }
    count
}

// Translated from 00e2f3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaAnimation::getAnnotations` (Xbox PDB), thiscall, `RET 0x10` (`time`, `delta`, `out`,
/// `max_count`): the annotations counted by `getNumAnnotations`, written to the 12-byte
/// records at `out` (track index word, time, a string pointer assigned from the annotation's
/// text), in track order, until `max_count` records are written (0 returns 0 at once).
/// Returns the number of records written.
pub fn hka_animation_get_annotations(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    delta: f32,
    out: Ptr,
    max_count: u32,
) -> u32 {
    if max_count == 0 {
        return 0;
    }
    let me = this.addr();
    let end = (f64::from(time) + f64::from(delta)) as f32;
    let duration = e.mem.f32(me + 0xc);
    let wrapped = wrapped_end(e, duration, end);
    let track_count = e.mem.i32(me + 0x20);
    let mut written = 0u32;
    if track_count <= 0 {
        return 0;
    }
    let mut track_index = 0u16;
    loop {
        let track = e
            .mem
            .u32(me + 0x1c)
            .wrapping_add(u32::from(track_index) << 4);
        let annotations = e.mem.i32(track + 8);
        for index in 0..annotations.max(0) as u32 {
            let data = e.mem.u32(track + 4);
            let annotation = data.wrapping_add(8 * index);
            let annotation_time = e.mem.f32(annotation);
            if !annotation_in_span(annotation_time, time, end, wrapped) {
                continue;
            }
            let record = out.addr().wrapping_add(12u32.wrapping_mul(written));
            e.mem.set_u16(record, track_index);
            let data = e.mem.u32(track + 4);
            let annotation = data.wrapping_add(8 * index);
            let copied = x87_quiet(e.mem.f32(annotation));
            e.mem.set_f32(record + 4, copied);
            e.call(STRING_ASSIGN, &args![record + 8, annotation + 4]);
            written += 1;
            if written == max_count {
                return written;
            }
        }
        track_index = track_index.wrapping_add(1);
        if i32::from(track_index) >= track_count {
            break;
        }
    }
    written
}

// Translated from 00e2f500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaAnimation::hkaAnimation` (Xbox PDB; this overload copies another animation), thiscall,
/// `RET 4`, returns `this`: sets the vtable, the reference count word (+6) to 1, copies the
/// words at +8, +0x10 and +0x14, the duration (+0xc) and the extracted-motion pointer (+0x18,
/// which gets a reference added when set), and copies the annotation track array (+0x1c data,
/// +0x20 size, +0x24 capacity): makes room, destroys surplus tracks, assigns (`00e2f680`) the
/// tracks both arrays have and copy-constructs (`00e4e420`) the rest.
pub fn hka_animation_hka_animation(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let me = this.addr();
    let src = source.addr();
    e.mem.set_u32(me, ANIMATION_VTABLE);
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 0x18, 0);
    e.mem.set_u32(me + 0x1c, 0);
    e.mem.set_u32(me + 0x20, 0);
    e.mem.set_u32(me + 0x24, 0x8000_0000);
    let word = e.mem.u32(src + 8);
    e.mem.set_u32(me + 8, word);
    let duration = x87_quiet(e.mem.f32(src + 0xc));
    e.mem.set_f32(me + 0xc, duration);
    let word = e.mem.u32(src + 0x10);
    e.mem.set_u32(me + 0x10, word);
    let word = e.mem.u32(src + 0x14);
    e.mem.set_u32(me + 0x14, word);
    let motion = e.mem.u32(src + 0x18);
    if motion != 0 {
        e.call(ADD_REFERENCE, &args![motion]);
    }
    // The pointer was zeroed above, so this release is never made (the compiler kept the
    // assignment sequence of a reference-counted pointer).
    let own = e.mem.u32(me + 0x18);
    if own != 0 {
        e.call(REMOVE_REFERENCE, &args![own]);
    }
    let motion = e.mem.u32(src + 0x18);
    e.mem.set_u32(me + 0x18, motion);

    let header = me + 0x1c;
    let old_size = e.mem.i32(header + 4);
    let new_size = e.mem.i32(src + 0x20);
    // The number of tracks both arrays have.
    let common = if new_size > old_size {
        old_size
    } else {
        new_size
    };
    reserve_at_least(e, header, new_size, 0x10);
    // Surplus tracks of the destination are destroyed (only for a negative source size, as the
    // destination starts empty).
    let data = e.mem.u32(header);
    let mut surplus_at = data.wrapping_add((new_size as u32) << 4);
    for _ in 0..(old_size.wrapping_sub(new_size)).max(0) {
        e.call(TRACK_DESTRUCT, &args![surplus_at]);
        surplus_at = surplus_at.wrapping_add(0x10);
    }
    // Tracks present in both are assigned. Unreachable here: the destination has no tracks, so
    // `common` is never positive.
    let src_data = e.mem.u32(src + 0x1c);
    let dest_data = e.mem.u32(header);
    for index in 0..common.max(0) as u32 {
        fn_00e2f680(
            e,
            Ptr::new(dest_data.wrapping_add(index << 4)),
            Ptr::new(src_data.wrapping_add(index << 4)),
        );
    }
    // The remaining source tracks are copy-constructed in place.
    let src_data = e.mem.u32(src + 0x1c).wrapping_add((common as u32) << 4);
    let dest_data = e.mem.u32(header).wrapping_add((common as u32) << 4);
    for index in 0..new_size.wrapping_sub(common).max(0) as u32 {
        let dest = dest_data.wrapping_add(index << 4);
        if dest != 0 {
            e.call(
                TRACK_COPY_CONSTRUCT,
                &args![dest, src_data.wrapping_add(index << 4)],
            );
        }
    }
    e.mem.set_u32(header + 4, new_size as u32);
    this
}

// Translated from 00e2f680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assignment of an annotation track (thiscall, `RET 4`, returns `this`; the identity is not
/// confirmed by a PDB name): a track is 0x10 bytes, a string pointer, then the array of
/// annotations (data +4, size +8, capacity +0xC) of 8 bytes (time, string pointer). Assigns
/// the name, makes room for the source's annotations, destroys the surplus strings, copies the
/// time and assigns the text of the annotations both have, copy-constructs the rest and sets
/// the size.
pub fn fn_00e2f680(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let me = this.addr();
    let src = source.addr();
    e.call(STRING_ASSIGN, &args![me, src]);
    let header = me + 4;
    let old_size = e.mem.i32(header + 4);
    let new_size = e.mem.i32(src + 8);
    let common = if new_size > old_size {
        old_size
    } else {
        new_size
    };
    reserve_at_least(e, header, new_size, 8);
    let data = e.mem.u32(header);
    let mut text_at = data.wrapping_add((new_size as u32) << 3).wrapping_add(4);
    for _ in 0..old_size.wrapping_sub(new_size).max(0) {
        e.call(STRING_DESTRUCT, &args![text_at]);
        text_at = text_at.wrapping_add(8);
    }
    let src_data = e.mem.u32(src + 4);
    let dest_data = e.mem.u32(me + 4);
    for index in 0..common.max(0) as u32 {
        let from = src_data.wrapping_add(index << 3);
        let to = dest_data.wrapping_add(index << 3);
        let time = x87_quiet(e.mem.f32(from));
        e.mem.set_f32(to, time);
        e.call(STRING_ASSIGN, &args![to + 4, from + 4]);
    }
    let src_data = e.mem.u32(src + 4).wrapping_add((common as u32) << 3);
    let dest_data = e.mem.u32(me + 4).wrapping_add((common as u32) << 3);
    for index in 0..new_size.wrapping_sub(common).max(0) as u32 {
        let from = src_data.wrapping_add(index << 3);
        let to = dest_data.wrapping_add(index << 3);
        if to != 0 {
            let time = x87_quiet(e.mem.f32(from));
            e.mem.set_f32(to, time);
            e.call(STRING_COPY_CONSTRUCT, &args![to + 4, from + 4]);
        }
    }
    e.mem.set_u32(me + 8, new_size as u32);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e2_f270, hka_animation_clear_all_cache_keys(Ptr, u32)),
        entry!(0x00e2_f280, hka_animation_get_num_annotations(Ptr, f32, f32) -> i32),
        entry!(0x00e2_f3e0, hka_animation_get_annotations(Ptr, f32, f32, Ptr, u32) -> u32),
        entry!(0x00e2_f500, hka_animation_hka_animation(Ptr, Ptr) -> Ptr),
        entry!(0x00e2_f680, fn_00e2f680(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const MEMORY_ROUTER: u32 = 0x00c8_5750;
    const ARRAY_RESERVE: u32 = 0x00c9_08c0;

    struct Setup {
        e: Engine,
        log: Log,
        alloc: u32,
    }

    fn p(address: u32) -> Ptr {
        Ptr::new(address)
    }

    fn setup() -> Setup {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(NO_WRAP, -3.0e38);
        let log: Log = Rc::default();
        let alloc = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, alloc);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [
            ADD_REFERENCE,
            REMOVE_REFERENCE,
            TRACK_DESTRUCT,
            TRACK_COPY_CONSTRUCT,
            STRING_ASSIGN,
            STRING_DESTRUCT,
            STRING_COPY_CONSTRUCT,
        ] {
            let log = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let reserve_log = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            reserve_log.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            // A new storage of `capacity` elements that takes over the elements the array holds.
            let (array, capacity, element) = (a[1], a[2], a[3]);
            let data = e.mem.alloc(capacity * element);
            let old = e.mem.u32(array);
            let size = e.mem.u32(array + 4);
            if old != 0 {
                for at in 0..size * element {
                    let byte = e.mem.u8(old + at);
                    e.mem.set_u8(data + at, byte);
                }
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        Setup { e, log, alloc }
    }

    fn calls_to(s: &Setup, address: u32) -> Vec<Vec<u32>> {
        s.log
            .borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Gives the track record at `track` an annotation array (header at +4: data, size, capacity)
    /// of 8-byte records: the time and a text marker `0x7000 + index`.
    fn fill_track(e: &mut Engine, track: u32, times: &[f32]) {
        let data = e.mem.alloc(8 * times.len() as u32 + 8);
        for (index, time) in times.iter().enumerate() {
            e.mem.set_f32(data + 8 * index as u32, *time);
            e.mem
                .set_u32(data + 8 * index as u32 + 4, 0x7000 + index as u32);
        }
        e.mem.set_u32(track + 4, data);
        e.mem.set_u32(track + 8, times.len() as u32);
        e.mem.set_u32(track + 0xc, times.len() as u32);
    }

    /// An animation of the given duration whose tracks hold the given annotation times.
    fn animation(e: &mut Engine, duration: f32, tracks: &[&[f32]]) -> u32 {
        let this = e.mem.alloc(0x40);
        let array = e.mem.alloc(0x10 * tracks.len() as u32 + 0x10);
        for (index, times) in tracks.iter().enumerate() {
            fill_track(e, array + 0x10 * index as u32, times);
        }
        e.mem.set_f32(this + 0xc, duration);
        e.mem.set_u32(this + 0x1c, array);
        e.mem.set_u32(this + 0x20, tracks.len() as u32);
        this
    }

    #[test]
    fn clear_all_cache_keys_does_nothing() {
        let mut s = setup();
        let this = animation(&mut s.e, 10.0, &[&[1.0]]);
        hka_animation_clear_all_cache_keys(&mut s.e, p(this), 0x1234);
        assert_eq!(s.e.mem.f32(this + 0xc), 10.0);
        assert!(s.log.borrow().is_empty());
    }

    #[test]
    fn num_annotations_counts_the_ones_inside_the_span() {
        let mut s = setup();
        // Span 1.0 .. 3.0 in an animation of 10: 1.0, 2.5 and 2.0 are inside; 0.5, 3.0, 5.0, 9.0 and a NaN
        // are not (the NaN compares unordered). Six annotations in the first track exercise the
        // unrolled part, the second track has two.
        let this = animation(
            &mut s.e,
            10.0,
            &[&[0.5, 1.0, 2.5, 3.0, 5.0, f32::NAN], &[2.0, 9.0]],
        );
        assert_eq!(
            hka_animation_get_num_annotations(&mut s.e, p(this), 1.0, 2.0),
            3
        );
    }

    #[test]
    fn num_annotations_counts_below_the_wrapped_end_when_the_span_passes_the_duration() {
        let mut s = setup();
        // Span 9.0 .. 11.0 in an animation of 10: wrapped end 1.0. 9.0, 9.5 and 10.5 are inside
        // the span, 0.5 is below the wrapped end, 1.0 is not below it, 11.0 is not inside.
        let this = animation(&mut s.e, 10.0, &[&[0.5, 1.0, 9.0, 9.5, 10.5, 11.0]]);
        assert_eq!(
            hka_animation_get_num_annotations(&mut s.e, p(this), 9.0, 2.0),
            4
        );
    }

    #[test]
    fn num_annotations_handles_empty_and_negative_counts() {
        let mut s = setup();
        let none = animation(&mut s.e, 10.0, &[]);
        assert_eq!(
            hka_animation_get_num_annotations(&mut s.e, p(none), 0.0, 1.0),
            0
        );
        let this = animation(&mut s.e, 10.0, &[&[], &[0.5]]);
        assert_eq!(
            hka_animation_get_num_annotations(&mut s.e, p(this), 0.0, 1.0),
            1
        );
        // A negative annotation count in a track is skipped.
        let array = s.e.mem.u32(this + 0x1c);
        s.e.mem.set_u32(array + 8, 0xffff_fffe);
        assert_eq!(
            hka_animation_get_num_annotations(&mut s.e, p(this), 0.0, 1.0),
            1
        );
        // A negative track count is no tracks.
        s.e.mem.set_u32(this + 0x20, 0xffff_ffff);
        assert_eq!(
            hka_animation_get_num_annotations(&mut s.e, p(this), 0.0, 1.0),
            0
        );
    }

    #[test]
    fn annotations_are_written_as_records_with_the_track_index() {
        let mut s = setup();
        let this = animation(&mut s.e, 10.0, &[&[0.5, 1.5], &[2.0, 4.0], &[2.5]]);
        let out = s.e.mem.alloc(12 * 8);
        let written = hka_animation_get_annotations(&mut s.e, p(this), 1.0, 2.0, p(out), 8);
        assert_eq!(written, 3);
        assert_eq!(s.e.mem.u16(out), 0);
        assert_eq!(s.e.mem.f32(out + 4), 1.5);
        assert_eq!(s.e.mem.u16(out + 12), 1);
        assert_eq!(s.e.mem.f32(out + 16), 2.0);
        assert_eq!(s.e.mem.u16(out + 24), 2);
        assert_eq!(s.e.mem.f32(out + 28), 2.5);
        // The text of each is assigned into the record's string pointer at +8, from the
        // annotation's string pointer.
        let array = s.e.mem.u32(this + 0x1c);
        let text = |track: u32, index: u32| s.e.mem.u32(array + 0x10 * track + 4) + 8 * index + 4;
        let expected = vec![
            vec![out + 8, text(0, 1)],
            vec![out + 20, text(1, 0)],
            vec![out + 32, text(2, 0)],
        ];
        assert_eq!(calls_to(&s, STRING_ASSIGN), expected);
    }

    #[test]
    fn annotations_stop_at_the_maximum_and_use_the_wrapped_end() {
        let mut s = setup();
        let this = animation(&mut s.e, 10.0, &[&[0.5, 9.0], &[9.5, 10.5]]);
        let out = s.e.mem.alloc(12 * 8);
        // Span 9.0 .. 11.0 wraps: everything below 1.0 counts too, so all four annotations are
        // found; the maximum of 2 stops after the second record.
        let written = hka_animation_get_annotations(&mut s.e, p(this), 9.0, 2.0, p(out), 2);
        assert_eq!(written, 2);
        assert_eq!(s.e.mem.f32(out + 4), 0.5);
        assert_eq!(s.e.mem.f32(out + 16), 9.0);
        assert_eq!(calls_to(&s, STRING_ASSIGN).len(), 2);
        // The same with room for all of them.
        let written = hka_animation_get_annotations(&mut s.e, p(this), 9.0, 2.0, p(out), 8);
        assert_eq!(written, 4);
        assert_eq!(s.e.mem.u16(out + 36), 1);
    }

    #[test]
    fn annotations_need_room_and_tracks() {
        let mut s = setup();
        let this = animation(&mut s.e, 10.0, &[&[1.5]]);
        let out = s.e.mem.alloc(12);
        s.e.mem.set_u32(out, 0xaaaa_aaaa);
        assert_eq!(
            hka_animation_get_annotations(&mut s.e, p(this), 1.0, 2.0, p(out), 0),
            0
        );
        assert_eq!(s.e.mem.u32(out), 0xaaaa_aaaa);
        assert!(calls_to(&s, STRING_ASSIGN).is_empty());
        s.e.mem.set_u32(this + 0x20, 0);
        assert_eq!(
            hka_animation_get_annotations(&mut s.e, p(this), 1.0, 2.0, p(out), 4),
            0
        );
        // A track without annotations (and a negative count) writes nothing.
        let empty = animation(&mut s.e, 10.0, &[&[], &[]]);
        assert_eq!(
            hka_animation_get_annotations(&mut s.e, p(empty), 0.0, 5.0, p(out), 4),
            0
        );
    }

    fn source_animation(s: &mut Setup, tracks: &[&[f32]]) -> u32 {
        let source = animation(&mut s.e, 4.5, tracks);
        s.e.mem.set_u32(source + 8, 0x1111);
        s.e.mem.set_u32(source + 0x10, 7);
        s.e.mem.set_u32(source + 0x14, 3);
        source
    }

    #[test]
    fn copy_construct_copies_the_header_and_constructs_the_tracks() {
        let mut s = setup();
        let source = source_animation(&mut s, &[&[1.0], &[2.0, 3.0]]);
        s.e.mem.set_u32(source + 0x18, 0x5000);
        let this = s.e.mem.alloc(0x40);
        let returned = hka_animation_hka_animation(&mut s.e, p(this), p(source));
        assert_eq!(returned, p(this));
        assert_eq!(s.e.mem.u32(this), ANIMATION_VTABLE);
        assert_eq!(s.e.mem.u16(this + 6), 1);
        assert_eq!(s.e.mem.u32(this + 8), 0x1111);
        assert_eq!(s.e.mem.f32(this + 0xc), 4.5);
        assert_eq!(s.e.mem.u32(this + 0x10), 7);
        assert_eq!(s.e.mem.u32(this + 0x14), 3);
        assert_eq!(s.e.mem.u32(this + 0x18), 0x5000);
        assert_eq!(calls_to(&s, ADD_REFERENCE), vec![vec![0x5000]]);
        assert!(calls_to(&s, REMOVE_REFERENCE).is_empty());
        // Room for 2 tracks of 0x10 bytes is reserved in the (empty) array.
        assert_eq!(
            calls_to(&s, ARRAY_RESERVE),
            vec![vec![s.alloc, this + 0x1c, 2, 0x10]]
        );
        let data = s.e.mem.u32(this + 0x1c);
        let source_data = s.e.mem.u32(source + 0x1c);
        assert_eq!(s.e.mem.u32(this + 0x20), 2);
        assert_eq!(
            calls_to(&s, TRACK_COPY_CONSTRUCT),
            vec![
                vec![data, source_data],
                vec![data + 0x10, source_data + 0x10]
            ]
        );
        assert!(calls_to(&s, TRACK_DESTRUCT).is_empty());
    }

    #[test]
    fn copy_construct_without_motion_or_tracks() {
        let mut s = setup();
        let source = source_animation(&mut s, &[]);
        let this = s.e.mem.alloc(0x40);
        hka_animation_hka_animation(&mut s.e, p(this), p(source));
        assert_eq!(s.e.mem.u32(this + 0x18), 0);
        assert!(calls_to(&s, ADD_REFERENCE).is_empty());
        assert_eq!(s.e.mem.u32(this + 0x20), 0);
        assert!(calls_to(&s, TRACK_COPY_CONSTRUCT).is_empty());
        assert!(calls_to(&s, ARRAY_RESERVE).is_empty());
    }

    #[test]
    fn copy_construct_destroys_surplus_tracks_for_a_negative_source_size() {
        let mut s = setup();
        let source = source_animation(&mut s, &[]);
        s.e.mem.set_u32(source + 0x20, -2i32 as u32);
        let this = s.e.mem.alloc(0x40);
        hka_animation_hka_animation(&mut s.e, p(this), p(source));
        // The destination holds no tracks, so the "surplus" starts below the (null) storage.
        let first = (-2i32 as u32) << 4;
        assert_eq!(
            calls_to(&s, TRACK_DESTRUCT),
            vec![vec![first], vec![first.wrapping_add(0x10)]]
        );
        assert_eq!(s.e.mem.u32(this + 0x20), -2i32 as u32);
    }

    fn annotation_track(e: &mut Engine, name: u32, times: &[f32]) -> u32 {
        let track = e.mem.alloc(0x10);
        e.mem.set_u32(track, name);
        fill_track(e, track, times);
        track
    }

    #[test]
    fn track_assign_into_an_empty_track_constructs_every_annotation() {
        let mut s = setup();
        let source = annotation_track(&mut s.e, 0x6001, &[1.0, 2.0]);
        let this = annotation_track(&mut s.e, 0x6002, &[]);
        s.e.mem.set_u32(this + 4, 0);
        s.e.mem.set_u32(this + 0xc, 0x8000_0000);
        let returned = fn_00e2f680(&mut s.e, p(this), p(source));
        assert_eq!(returned, p(this));
        assert_eq!(calls_to(&s, STRING_ASSIGN), vec![vec![this, source]]);
        assert_eq!(
            calls_to(&s, ARRAY_RESERVE),
            vec![vec![s.alloc, this + 4, 2, 8]]
        );
        let data = s.e.mem.u32(this + 4);
        let source_data = s.e.mem.u32(source + 4);
        assert_eq!(s.e.mem.f32(data), 1.0);
        assert_eq!(s.e.mem.f32(data + 8), 2.0);
        assert_eq!(s.e.mem.u32(this + 8), 2);
        assert_eq!(
            calls_to(&s, STRING_COPY_CONSTRUCT),
            vec![
                vec![data + 4, source_data + 4],
                vec![data + 12, source_data + 12]
            ]
        );
    }

    #[test]
    fn track_assign_assigns_the_common_annotations_and_destroys_the_surplus() {
        let mut s = setup();
        let source = annotation_track(&mut s.e, 0x6001, &[5.0]);
        let this = annotation_track(&mut s.e, 0x6002, &[1.0, 2.0, 3.0]);
        let data = s.e.mem.u32(this + 4);
        fn_00e2f680(&mut s.e, p(this), p(source));
        let source_data = s.e.mem.u32(source + 4);
        // One annotation in common: its time is copied and its text assigned; the other two
        // strings are destroyed; nothing is constructed and no storage is reserved.
        assert_eq!(s.e.mem.f32(data), 5.0);
        assert_eq!(
            calls_to(&s, STRING_ASSIGN),
            vec![vec![this, source], vec![data + 4, source_data + 4]]
        );
        assert_eq!(
            calls_to(&s, STRING_DESTRUCT),
            vec![vec![data + 12], vec![data + 20]]
        );
        assert!(calls_to(&s, STRING_COPY_CONSTRUCT).is_empty());
        assert!(calls_to(&s, ARRAY_RESERVE).is_empty());
        assert_eq!(s.e.mem.u32(this + 8), 1);
    }

    #[test]
    fn track_assign_mixes_assignment_and_construction() {
        let mut s = setup();
        let source = annotation_track(&mut s.e, 0x6001, &[5.0, 6.0, 7.0]);
        let this = annotation_track(&mut s.e, 0x6002, &[1.0]);
        // The destination array has room for the grown size, so nothing is reserved.
        let data = s.e.mem.alloc(8 * 4);
        s.e.mem.set_u32(data, 0);
        s.e.mem.set_f32(data, 1.0);
        s.e.mem.set_u32(this + 4, data);
        s.e.mem.set_u32(this + 0xc, 4);
        fn_00e2f680(&mut s.e, p(this), p(source));
        let source_data = s.e.mem.u32(source + 4);
        assert!(calls_to(&s, ARRAY_RESERVE).is_empty());
        assert_eq!(s.e.mem.f32(data), 5.0);
        assert_eq!(s.e.mem.f32(data + 8), 6.0);
        assert_eq!(s.e.mem.f32(data + 16), 7.0);
        assert_eq!(
            calls_to(&s, STRING_ASSIGN),
            vec![vec![this, source], vec![data + 4, source_data + 4]]
        );
        assert_eq!(
            calls_to(&s, STRING_COPY_CONSTRUCT),
            vec![
                vec![data + 12, source_data + 12],
                vec![data + 20, source_data + 20]
            ]
        );
        assert_eq!(s.e.mem.u32(this + 8), 3);
    }
}
