//! `Havok SDK/hkpcapsuletriangleagent3.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The "agent3" of a capsule against a triangle works on caller-held state instead of an
//! agent object: three 16-bit contact point ids (`0xFFFF` = none) and three 8-byte cache
//! entries per triangle, a contact manager and an output block whose cursor (`+0`) takes the
//! 0x30-byte contact records and whose `+0x3080` may hold a list of requests.
//!
//! The functions that start a profiling record (`RDTSC` time stamps from the host clock) are
//! otherwise complete: none of them has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpbvagent::LARGE_FLOAT;
use crate::units::havok_sdk::hkpcapsulecapsuleagent::{
    load_matrix, radius_bits, store_ends, transform_point, world_ends, LABEL_END,
};
use crate::units::havok_sdk::hkpcapsuletriangleagent::CAPS_VS_TRI;
use crate::units::havok_sdk::hkpsphereshape::{copy_vec, load_vec, store_vec};
use crate::units::havok_sdk::run_00cf5070::profile_record;

/// The profiling label `"TtCapsTri3"`.
const LABEL_PROCESS: u32 = 0x010c_eeec;

/// The frame of the function: the cache entries the utility answers with (8 bytes each),
/// the capsule's end points and the triangle's vertices in world space, the three contacts
/// (0x20 bytes each: a point and a normal whose lane 3 is the gap), a count byte and a copy
/// of a normal.
const CACHE_OUT: u32 = 0x78;
const ENDS: u32 = 0x90;
const TRIANGLE: u32 = 0xb0;
const CONTACTS: u32 = 0xe0;
const COUNT: u32 = 0x13;
const COPY: u32 = 0x20;
const FRAME_SIZE: u32 = 0x140;
/// The id of "no contact point".
const NO_ID: u32 = 0xffff;

// Translated from 00d39390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCapsuleTriangleAgent3::process` (name from the engine map, unconfirmed; cdecl): profiled
/// (`"TtCapsTri3"`). The capsule of body A (`[input]`) and the triangle of body B
/// (`[input + 4]`) are moved to world space and given to `hkCollideCapsuleUtilCapsVsTri`
/// (`00cd6520`) with the radii, the cache entries at `ids + 8`, the large float, the search
/// flag 1, a frame for three contacts and a frame for three new cache entries. The `ids`
/// are three 16-bit contact point ids. Each contact whose gap (lane 3 of the normal) is below
/// the tolerance (`[[input + 0xC] + 8]`, x87 compares) is kept: with no id yet, one is asked of
/// the contact manager (`[input + 0x10]`; with a list at `[output + 0x3080]` its slot `0x10`
/// is asked and, when it answers 0, a request `(output cursor, counts, ids)` is added to the
/// list; else slot `0xC` adds the contact and answers the id); when the cache entry has
/// features the triangle shape (slot `0x40`) is asked about them, answer 0 dropping the contact
/// (its id is given back by slot `0x14` with `[output + 4]`) and answer 1 taking the changed
/// vector's first three lanes. A kept contact is appended to the output (0x30 bytes: point,
/// normal, id) and counted in the byte `counts + 2`; contacts at or above the tolerance (or
/// unordered) give their id back. The contact of the smaller gap among the first two (the
/// second also when equal or unordered) has its normal copied to `closest` and, when the list
/// exists, its output record added to the list at `[list + 4]`. Answers `ids + 0x20`.
pub fn hk_capsule_triangle_agent3_process(
    e: &mut Engine,
    input: Ptr,
    counts: Ptr,
    ids: Ptr,
    closest: Ptr,
    output: Ptr,
) -> u32 {
    profile_record(e, LABEL_PROCESS);
    let (input, counts, ids, closest, output) = (
        input.addr(),
        counts.addr(),
        ids.addr(),
        closest.addr(),
        output.addr(),
    );
    let body_a = e.mem.u32(input);
    let body_b = e.mem.u32(input + 4);
    let shape_a = e.mem.u32(body_a);
    let shape_b = e.mem.u32(body_b);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let ends = world_ends(e, body_a);
        store_ends(e, frame + ENDS, ends);
        let rows = load_matrix(e, e.mem.u32(body_b + 8));
        for (n, offset) in [0x20u32, 0x30, 0x40].into_iter().enumerate() {
            let vertex = transform_point(&rows, load_vec(e, shape_b + offset));
            store_vec(e, frame + TRIANGLE + 0x10 * n as u32, vertex);
        }
        let large = x87_quiet_bits(e.mem.u32(LARGE_FLOAT));
        let radius_b = radius_bits(e, body_b);
        let radius_a = radius_bits(e, body_a);
        e.call(
            CAPS_VS_TRI,
            &args![
                frame + ENDS,
                radius_a,
                frame + TRIANGLE,
                radius_b,
                ids + 8,
                large,
                1u32,
                frame + CONTACTS,
                frame + CACHE_OUT
            ],
        );
        let gap_of = |e: &Engine, n: u32| e.mem.f32(frame + CONTACTS + 0x20 * n + 0x1c);
        let flag = u32::from(!matches!(
            gap_of(e, 1).partial_cmp(&gap_of(e, 0)),
            Some(std::cmp::Ordering::Greater)
        ));
        let mut kept = 0u8;
        for n in 0..3u32 {
            let contact = frame + CONTACTS + 0x20 * n;
            let normal = contact + 0x10;
            let entry = frame + CACHE_OUT + 8 * n;
            let object = e.mem.u32(input + 0xc);
            let tolerance = e.mem.f32(object + 8);
            let mut id = u32::from(e.mem.u16(ids + 2 * n));
            let manager = e.mem.u32(input + 0x10);
            if tolerance > e.mem.f32(normal + 0xc) {
                if id == NO_ID {
                    let list = e.mem.u32(output + 0x3080);
                    if list != 0 {
                        if e.vcall(manager, 0x10, &args![1u32]).u32() != 0 {
                            e.mem.set_u16(ids + 2 * n, id as u16);
                            continue;
                        }
                        let cursor = e.mem.u32(list);
                        e.mem.set_u32(list, cursor + 0xc);
                        let first = e.mem.u32(output);
                        e.mem.set_u32(cursor, first);
                        e.mem.set_u32(cursor + 8, ids);
                        e.mem.set_u32(cursor + 4, counts);
                    } else {
                        let first = e.mem.u32(input);
                        let second = e.mem.u32(input + 4);
                        id = e
                            .vcall(
                                manager,
                                0xc,
                                &args![first, second, object, output, 0u32, contact],
                            )
                            .u32()
                            & 0xffff;
                        if id == NO_ID {
                            e.mem.set_u16(ids + 2 * n, id as u16);
                            continue;
                        }
                    }
                }
                let features = e.mem.u8(entry);
                e.mem.set_u8(frame + COUNT, features);
                if features != 0 {
                    copy_vec(e, normal, frame + COPY);
                    let motion_b = e.mem.u32(e.mem.u32(input + 4) + 8);
                    let motion_a = e.mem.u32(e.mem.u32(input) + 8);
                    let answer = e
                        .vcall(
                            shape_b,
                            0x40,
                            &args![
                                entry + 2,
                                frame + COUNT,
                                contact,
                                motion_b,
                                shape_a,
                                motion_a,
                                frame + COPY
                            ],
                        )
                        .u32();
                    if answer == 0 {
                        if id == NO_ID {
                            continue;
                        }
                        let manager = e.mem.u32(input + 0x10);
                        let word = e.mem.u32(output + 4);
                        e.vcall(manager, 0x14, &args![id, word]);
                        e.mem.set_u16(ids + 2 * n, NO_ID as u16);
                        continue;
                    }
                    if answer == 1 {
                        for lane in 0..3 {
                            let word = e.mem.u32(frame + COPY + 4 * lane);
                            e.mem.set_u32(normal + 4 * lane, word);
                        }
                    }
                }
                let cursor = e.mem.u32(output);
                e.mem.set_u32(output, cursor + 0x30);
                kept += 1;
                copy_vec(e, contact, cursor);
                copy_vec(e, normal, cursor + 0x10);
                e.mem.set_u32(cursor + 0x20, id);
                if n == flag {
                    let list = e.mem.u32(output + 0x3080);
                    if list != 0 {
                        let slot = e.mem.u32(list + 4);
                        e.mem.set_u32(slot, cursor);
                        e.mem.set_u32(list + 4, slot + 4);
                    }
                }
            } else if id != NO_ID {
                let word = e.mem.u32(output + 4);
                e.vcall(manager, 0x14, &args![id, word]);
                id = NO_ID;
            }
            e.mem.set_u16(ids + 2 * n, id as u16);
        }
        copy_vec(e, frame + CONTACTS + 0x20 * flag + 0x10, closest);
        e.mem.set_u8(counts + 2, kept);
    });
    profile_record(e, LABEL_END);
    ids + 0x20
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d3_9390,
        hk_capsule_triangle_agent3_process(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, monitor, new_log, profile_labels, Log,
    };
    use crate::units::havok_sdk::hkpcapsulecapsuleagent::fixtures::{
        capsule_a, shaped_body, IDENTITY,
    };
    use crate::units::havok_sdk::hkpsphereshape::Vec4;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const MANAGER: u32 = 0x0900_0000;
    const ADD_CONTACT: u32 = MANAGER + 0xc;
    const REQUEST_ID: u32 = MANAGER + 0x10;
    const GIVE_BACK: u32 = MANAGER + 0x14;
    const ASK: u32 = 0x0930_0040;
    const ASK_SEEN: u32 = 0x0930_0041;
    /// The word at `+4` of the output block.
    const OUTPUT_WORD: u32 = 0x4444;
    /// What the shape answering 1 leaves in the vector it was given.
    const CHANGED: Vec4 = [9.0, 8.0, 7.0, 6.0];
    const VERTICES: [Vec4; 3] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
    ];

    /// What a scene is built from: the gap of every contact the utility makes, the feature
    /// count of its cache entries, the shape's answer, the answer of the manager's request
    /// slot (`None`: no list in the output), the ids and the tolerance.
    #[derive(Clone, Copy)]
    struct Settings {
        gaps: [f32; 3],
        features: [u8; 3],
        ask: u32,
        request: Option<u32>,
        ids: [u16; 3],
        tolerance: f32,
    }

    impl Default for Settings {
        fn default() -> Self {
            Settings {
                gaps: [0.1, 0.2, 0.3],
                features: [0; 3],
                ask: 0,
                request: None,
                ids: [0xffff; 3],
                tolerance: 1.0,
            }
        }
    }

    struct Scene {
        e: Engine,
        log: Log,
        input: u32,
        manager: u32,
        counts: u32,
        ids: u32,
        closest: u32,
        output: u32,
        area: u32,
        list: u32,
        requests: u32,
        sublist: u32,
        stream: u32,
        records: u32,
        /// The contact frame of the utility's call.
        frame: Rc<Cell<u32>>,
        /// The vectors the utility saw: the two end points and the three vertices.
        seen: Rc<RefCell<Vec<Vec4>>>,
    }

    fn scene(settings: Settings) -> Scene {
        let mut e = engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        e.mem.set_u32(LARGE_FLOAT, 0x7fa0_0001);
        let a = capsule_a(&mut e);
        let b = shaped_body(&mut e, 0.25, &VERTICES, IDENTITY);
        let shape_b = e.mem.u32(b);
        let shape_vtable = e.mem.alloc(0x80);
        e.mem.set_u32(shape_vtable + 0x40, ASK);
        e.mem.set_u32(shape_b, shape_vtable);
        let object = e.mem.alloc(0x20);
        e.mem.set_f32(object + 8, settings.tolerance);
        let manager_vtable = e.mem.alloc(0x40);
        for slot in [0xc, 0x10, 0x14] {
            e.mem.set_u32(manager_vtable + slot, MANAGER + slot);
        }
        let manager = e.mem.alloc(0x10);
        e.mem.set_u32(manager, manager_vtable);
        let input = e.mem.alloc(0x20);
        for (offset, word) in [(0, a), (4, b), (0xc, object), (0x10, manager)] {
            e.mem.set_u32(input + offset, word);
        }
        let counts = e.mem.alloc(8);
        let ids = e.mem.alloc(0x40);
        for (n, id) in settings.ids.into_iter().enumerate() {
            e.mem.set_u16(ids + 2 * n as u32, id);
        }
        let closest = e.mem.alloc(0x10);
        let output = e.mem.alloc(0x3100);
        let area = e.mem.alloc(0x200);
        e.mem.set_u32(output, area);
        e.mem.set_u32(output + 4, OUTPUT_WORD);
        let list = e.mem.alloc(0x10);
        let requests = e.mem.alloc(0x40);
        let sublist = e.mem.alloc(0x40);
        e.mem.set_u32(list, requests);
        e.mem.set_u32(list + 4, sublist);
        if settings.request.is_some() {
            e.mem.set_u32(output + 0x3080, list);
        }

        let frame = Rc::new(Cell::new(0));
        let seen = Rc::new(RefCell::new(Vec::new()));
        let (frame_copy, seen_copy, copy) = (Rc::clone(&frame), Rc::clone(&seen), Rc::clone(&log));
        e.register_double(CAPS_VS_TRI, move |e, args| {
            copy.borrow_mut().push((CAPS_VS_TRI, args.to_vec()));
            frame_copy.set(args[7]);
            for (pointer, count) in [(args[0], 2), (args[2], 3)] {
                for n in 0..count {
                    seen_copy.borrow_mut().push(load_vec(e, pointer + 0x10 * n));
                }
            }
            for (n, gap) in settings.gaps.into_iter().enumerate() {
                let contact = args[7] + 0x20 * n as u32;
                store_vec(e, contact, point(n));
                store_vec(e, contact + 0x10, normal(n, gap));
                let entry = args[8] + 8 * n as u32;
                e.mem.set_u8(entry, settings.features[n]);
                e.mem.set_u16(entry + 2, 10 * n as u16 + 1);
            }
            Ret::default()
        });
        let added = Rc::new(Cell::new(0u32));
        let copy = Rc::clone(&log);
        e.register_double(ADD_CONTACT, move |_, args| {
            copy.borrow_mut().push((ADD_CONTACT, args.to_vec()));
            added.set(added.get() + 1);
            Ret {
                eax: 0x2f + added.get(),
                ..Ret::default()
            }
        });
        let copy = Rc::clone(&log);
        let request = settings.request.unwrap_or(0);
        e.register_double(REQUEST_ID, move |_, args| {
            copy.borrow_mut().push((REQUEST_ID, args.to_vec()));
            Ret {
                eax: request,
                ..Ret::default()
            }
        });
        let copy = Rc::clone(&log);
        e.register_double(GIVE_BACK, move |_, args| {
            copy.borrow_mut().push((GIVE_BACK, args.to_vec()));
            Ret::default()
        });
        let copy = Rc::clone(&log);
        e.register_double(ASK, move |e, args| {
            copy.borrow_mut().push((ASK, args.to_vec()));
            let vertex = u32::from(e.mem.u16(args[1]));
            let count = u32::from(e.mem.u8(args[2]));
            copy.borrow_mut().push((ASK_SEEN, vec![vertex, count]));
            if settings.ask == 1 {
                store_vec(e, args[7], CHANGED);
            }
            Ret {
                eax: settings.ask,
                ..Ret::default()
            }
        });
        Scene {
            e,
            log,
            input,
            manager,
            counts,
            ids,
            closest,
            output,
            area,
            list,
            requests,
            sublist,
            stream,
            records,
            frame,
            seen,
        }
    }

    /// The point and the normal of contact `n` that the utility's double makes.
    fn point(n: usize) -> Vec4 {
        [n as f32 + 1.0, 2.0, 3.0, 4.0]
    }

    fn normal(n: usize, gap: f32) -> Vec4 {
        [0.5 + n as f32, 0.25, 0.125, gap]
    }

    fn bits(vector: Vec4) -> Vec<u32> {
        vector.map(f32::to_bits).to_vec()
    }

    impl Scene {
        fn run(&mut self) -> u32 {
            self.e
                .call(
                    0x00d3_9390,
                    &[self.input, self.counts, self.ids, self.closest, self.output],
                )
                .u32()
        }

        fn ids(&self) -> Vec<u16> {
            (0..3).map(|n| self.e.mem.u16(self.ids + 2 * n)).collect()
        }

        /// The output records written since `area`, as (point, normal, id).
        fn records(&self) -> Vec<(Vec4, Vec4, u32)> {
            let end = self.e.mem.u32(self.output);
            (self.area..end)
                .step_by(0x30)
                .map(|at| {
                    (
                        load_vec(&self.e, at),
                        load_vec(&self.e, at + 0x10),
                        self.e.mem.u32(at + 0x20),
                    )
                })
                .collect()
        }

        fn closest(&self) -> Vec<u32> {
            (0..4)
                .map(|n| self.e.mem.u32(self.closest + 4 * n))
                .collect()
        }
    }

    #[test]
    fn new_contacts_get_ids_and_are_appended_00d39390() {
        let mut s = scene(Settings::default());
        let result = s.run();
        assert_eq!(result, s.ids + 0x20);
        let frame = s.frame.get();
        assert_eq!(
            calls(&s.log, CAPS_VS_TRI)[0],
            vec![
                frame - 0x50,
                1.0f32.to_bits(),
                frame - 0x30,
                0.25f32.to_bits(),
                s.ids + 8,
                0x7fe0_0001,
                1,
                frame,
                frame - 0x68,
            ]
        );
        let world: Vec<Vec4> = vec![
            [8.0, 21.0, 33.0, 1.0],
            [8.0, 21.0, 34.0, 1.0],
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 1.0],
        ];
        assert_eq!(*s.seen.borrow(), world);
        let object = s.e.mem.u32(s.input + 0xc);
        let (a, b) = (s.e.mem.u32(s.input), s.e.mem.u32(s.input + 4));
        let adds: Vec<Vec<u32>> = (0..3)
            .map(|n| vec![s.manager, a, b, object, s.output, 0, frame + 0x20 * n])
            .collect();
        assert_eq!(calls(&s.log, ADD_CONTACT), adds);
        assert_eq!(s.ids(), vec![0x30, 0x31, 0x32]);
        assert_eq!(
            s.records(),
            vec![
                (point(0), normal(0, 0.1), 0x30),
                (point(1), normal(1, 0.2), 0x31),
                (point(2), normal(2, 0.3), 0x32),
            ]
        );
        assert_eq!(s.e.mem.u8(s.counts + 2), 3);
        // The second gap is larger: the first contact's normal is the answer.
        assert_eq!(s.closest(), bits(normal(0, 0.1)));
        assert!(calls(&s.log, ASK).is_empty());
        assert_eq!(
            profile_labels(&s.e, s.stream, s.records),
            vec![LABEL_PROCESS, LABEL_END]
        );
    }

    #[test]
    fn far_or_unordered_contacts_give_their_ids_back_00d39390() {
        let mut s = scene(Settings {
            gaps: [2.0, f32::NAN, 0.5],
            ids: [0x21, 0x22, 0xffff],
            ..Settings::default()
        });
        s.run();
        assert_eq!(
            calls(&s.log, GIVE_BACK),
            vec![
                vec![s.manager, 0x21, OUTPUT_WORD],
                vec![s.manager, 0x22, OUTPUT_WORD]
            ]
        );
        assert_eq!(calls(&s.log, ADD_CONTACT).len(), 1);
        assert_eq!(s.ids(), vec![0xffff, 0xffff, 0x30]);
        assert_eq!(s.records(), vec![(point(2), normal(2, 0.5), 0x30)]);
        assert_eq!(s.e.mem.u8(s.counts + 2), 1);
        // An unordered gap picks the second contact.
        assert_eq!(s.closest(), bits(normal(1, f32::NAN)));
    }

    #[test]
    fn known_ids_are_kept_and_equal_gaps_pick_the_second_00d39390() {
        let mut s = scene(Settings {
            gaps: [0.1, 0.1, 0.3],
            ids: [0x40, 0x41, 0x42],
            ..Settings::default()
        });
        s.run();
        assert!(calls(&s.log, ADD_CONTACT).is_empty());
        assert!(calls(&s.log, GIVE_BACK).is_empty());
        assert_eq!(s.ids(), vec![0x40, 0x41, 0x42]);
        assert_eq!(
            s.records()
                .iter()
                .map(|record| record.2)
                .collect::<Vec<_>>(),
            vec![0x40, 0x41, 0x42]
        );
        assert_eq!(s.closest(), bits(normal(1, 0.1)));
    }

    #[test]
    fn a_refused_request_leaves_the_contact_out_00d39390() {
        let mut s = scene(Settings {
            request: Some(1),
            ..Settings::default()
        });
        s.run();
        assert_eq!(calls(&s.log, REQUEST_ID), vec![vec![s.manager, 1]; 3]);
        assert!(calls(&s.log, ADD_CONTACT).is_empty());
        assert!(s.records().is_empty());
        assert_eq!(s.ids(), vec![0xffff; 3]);
        assert_eq!(s.e.mem.u8(s.counts + 2), 0);
        assert_eq!(s.e.mem.u32(s.list), s.requests);
        assert_eq!(s.e.mem.u32(s.list + 4), s.sublist);
    }

    #[test]
    fn granted_requests_go_to_the_list_with_the_output_cursor_00d39390() {
        let mut s = scene(Settings {
            request: Some(0),
            ..Settings::default()
        });
        s.run();
        assert!(calls(&s.log, ADD_CONTACT).is_empty());
        assert_eq!(s.e.mem.u32(s.list), s.requests + 0x24);
        for n in 0..3 {
            let request = s.requests + 0xc * n;
            let words: Vec<u32> = (0..3).map(|word| s.e.mem.u32(request + 4 * word)).collect();
            assert_eq!(words, vec![s.area + 0x30 * n, s.counts, s.ids]);
        }
        // The contacts are appended with no id; the one picked for the closest point (the
        // first) is added to the second list.
        assert_eq!(s.ids(), vec![0xffff; 3]);
        assert_eq!(s.records().len(), 3);
        assert!(s.records().iter().all(|record| record.2 == 0xffff));
        assert_eq!(s.e.mem.u32(s.list + 4), s.sublist + 4);
        assert_eq!(s.e.mem.u32(s.sublist), s.area);
    }

    #[test]
    fn features_ask_the_triangle_shape_00d39390() {
        // Answer 0 drops the contact and gives its id back.
        let mut s = scene(Settings {
            features: [1; 3],
            ids: [0x50, 0xffff, 0x52],
            ..Settings::default()
        });
        s.run();
        let frame = s.frame.get();
        let motion_a = s.e.mem.u32(s.e.mem.u32(s.input) + 8);
        let b = s.e.mem.u32(s.input + 4);
        let shape_a = s.e.mem.u32(s.e.mem.u32(s.input));
        let ask = calls(&s.log, ASK);
        assert_eq!(ask.len(), 3);
        assert_eq!(
            ask[0][1..],
            [
                frame - 0x66,
                frame - 0xcd,
                frame,
                s.e.mem.u32(b + 8),
                shape_a,
                motion_a,
                frame - 0xc0
            ]
        );
        assert_eq!(calls(&s.log, ASK_SEEN)[0], vec![1, 1]);
        assert_eq!(
            calls(&s.log, GIVE_BACK),
            vec![
                vec![s.manager, 0x50, OUTPUT_WORD],
                vec![s.manager, 0x30, OUTPUT_WORD],
                vec![s.manager, 0x52, OUTPUT_WORD]
            ]
        );
        assert!(s.records().is_empty());
        assert_eq!(s.ids(), vec![0xffff; 3]);
        assert_eq!(s.e.mem.u8(s.counts + 2), 0);

        // Answer 1 takes the changed vector's first three lanes, 2 keeps the normal.
        for (answer, expected) in [(1, [9.0, 8.0, 7.0]), (2, [0.5, 0.25, 0.125])] {
            let mut s = scene(Settings {
                features: [1; 3],
                ask: answer,
                ids: [0x50, 0x51, 0x52],
                ..Settings::default()
            });
            s.run();
            let records = s.records();
            assert_eq!(records.len(), 3, "answer {answer}");
            let gaps: Vec<f32> = records.iter().map(|record| record.1[3]).collect();
            assert_eq!(gaps, vec![0.1, 0.2, 0.3]);
            assert_eq!(records[0].1[..3], expected);
            assert!(calls(&s.log, GIVE_BACK).is_empty());
        }
    }

    #[test]
    fn an_answer_0_without_an_id_drops_the_contact_quietly_00d39390() {
        let mut s = scene(Settings {
            features: [1; 3],
            request: Some(0),
            ..Settings::default()
        });
        s.run();
        assert_eq!(calls(&s.log, ASK).len(), 3);
        assert!(calls(&s.log, GIVE_BACK).is_empty());
        assert!(s.records().is_empty());
        assert_eq!(s.ids(), vec![0xffff; 3]);
        assert_eq!(s.e.mem.u32(s.list), s.requests + 0x24);
    }
}
