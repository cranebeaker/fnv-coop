//! `NiXenonRenderer/run_00e715a0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The shader's virtual slots the draw runs through (`NiD3DShader`'s in the Xbox PDB:
/// `PreProcessPipeline`, `UpdatePipeline`, `SetupRenderingPass`, `SetupTransformations`,
/// `SetupShaderPrograms`, `PrepareGeometryForRendering`, `PostRender`,
/// `PostProcessPipeline`, `FirstPass`, `NextPass`).
const PRE_PROCESS_PIPELINE: u32 = 0x54;
const UPDATE_PIPELINE: u32 = 0x58;
const SETUP_RENDERING_PASS: u32 = 0x5c;
const SETUP_TRANSFORMATIONS: u32 = 0x60;
const SETUP_SHADER_PROGRAMS: u32 = 0x68;
const PREPARE_GEOMETRY: u32 = 0x6c;
const POST_RENDER: u32 = 0x70;
const POST_PROCESS_PIPELINE: u32 = 0x74;
const FIRST_PASS: u32 = 0x90;
const NEXT_PASS: u32 = 0x94;
/// The device's draw slots (`IDirect3DDevice9` order, the device pushed first):
/// `DrawPrimitive` (+0x144) and `DrawIndexedPrimitive` (+0x148).
const DRAW_PRIMITIVE: u32 = 0x144;
const DRAW_INDEXED_PRIMITIVE: u32 = 0x148;

// Translated from 00e715a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the renderer, six stack words, the second never
/// read): draws `geometry` with its shader (+0xc0) pass by pass. The shader's
/// `PreProcessPipeline` (slot +0x54) gets the geometry, `a3`, `a6`, the renderer's +0xc and
/// +0x10, `a4`, `a5`; a non-zero answer ends the draw. Then `UpdatePipeline` (+0x58) with the
/// same words and, for each pass (`FirstPass` +0x90, then `NextPass` +0x94 while non-zero):
/// `SetupRenderingPass` (+0x5c, the same words), `SetupTransformations` (+0x60: geometry,
/// `a3`, 0, the pass word, +0xc, +0x10, `a4`, `a5`), `PrepareGeometryForRendering` (+0x6c:
/// geometry, 0, the pass word, +0xc) whose answer is the buffer data, `SetupShaderPrograms`
/// (+0x68: geometry, `a3`, 0, the data, +0xc, +0x10, `a4`, `a5`), the draw, and `PostRender`
/// (+0x70, as +0x68). The pass word is `a6` on the first pass and the previous pass's buffer
/// data after it. At the end `PostProcessPipeline` (+0x74, the first form with the pass
/// word) and the render-state object's (+0x8b8) slot +0xf8 with 0.
///
/// The draw: without strips (data +0x34 zero) one `DrawPrimitive` (type +0x3c, start +0x38,
/// count +0x40). Otherwise for each of the data's strips (+0x48): its length is its 16-bit
/// entry in the array at +0x4c less 2, or (no array) the data's +0x40, replaced by the word
/// +0xc4 of what the geometry's slot +0x2c answers when that is not null (asked twice). When
/// the geometry's slot +0x28 answers null the strip is one `DrawIndexedPrimitive` (type
/// +0x3c, base vertex +0x38, 0, vertex count +0x18, the running start index, the length);
/// otherwise each of that object's 0x14-byte partitions (at +0xc4; one when its byte +0xcd
/// is set, else its count +0xc8) whose byte +0x10 is set is drawn with its own start
/// (word +0, or 0 with +0xcd set) and count (word +0xc, or the `u16` at +0x40 of the object
/// at +0xb8 with +0xcd set). The start index then moves on by the length plus 2.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e715a0(
    e: &mut Engine,
    this: Ptr,
    geometry: Ptr,
    _unused_2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
) {
    let base = this.addr();
    let geometry = geometry.addr();
    let shader = if geometry != 0 {
        e.mem.u32(geometry + 0xc0)
    } else {
        0
    };
    let common = |e: &Engine, pass_word: u32| {
        args![
            geometry,
            a3,
            pass_word,
            e.mem.u32(base + 0xc),
            e.mem.u32(base + 0x10),
            a4,
            a5
        ]
    };
    let words = common(e, a6);
    if e.vcall(shader, PRE_PROCESS_PIPELINE, &words).u32() != 0 {
        return;
    }
    let words = common(e, a6);
    e.vcall(shader, UPDATE_PIPELINE, &words);
    let mut pass_word = a6;
    let mut more = e.vcall(shader, FIRST_PASS, &args![]).u32() != 0;
    while more {
        let words = common(e, pass_word);
        e.vcall(shader, SETUP_RENDERING_PASS, &words);
        let (c, d) = (e.mem.u32(base + 0xc), e.mem.u32(base + 0x10));
        e.vcall(
            shader,
            SETUP_TRANSFORMATIONS,
            &args![geometry, a3, 0u32, pass_word, c, d, a4, a5],
        );
        let c = e.mem.u32(base + 0xc);
        let data = e
            .vcall(
                shader,
                PREPARE_GEOMETRY,
                &args![geometry, 0u32, pass_word, c],
            )
            .u32();
        pass_word = data;
        let (c, d) = (e.mem.u32(base + 0xc), e.mem.u32(base + 0x10));
        e.vcall(
            shader,
            SETUP_SHADER_PROGRAMS,
            &args![geometry, a3, 0u32, data, c, d, a4, a5],
        );
        draw(e, base, geometry, data);
        let (c, d) = (e.mem.u32(base + 0xc), e.mem.u32(base + 0x10));
        e.vcall(
            shader,
            POST_RENDER,
            &args![geometry, a3, 0u32, data, c, d, a4, a5],
        );
        more = e.vcall(shader, NEXT_PASS, &args![]).u32() != 0;
    }
    let words = common(e, pass_word);
    e.vcall(shader, POST_PROCESS_PIPELINE, &words);
    let state = e.mem.u32(base + 0x8b8);
    e.vcall(state, 0xf8, &args![0u32]);
}

/// The draw of one pass of `fn_00e715a0` (see there).
fn draw(e: &mut Engine, base: u32, geometry: u32, data: u32) {
    if e.mem.u32(data + 0x34) == 0 {
        let device = e.mem.u32(base + 0x288);
        let (kind, start, count) = (
            e.mem.u32(data + 0x3c),
            e.mem.u32(data + 0x38),
            e.mem.u32(data + 0x40),
        );
        e.vcall(device, DRAW_PRIMITIVE, &args![kind, start, count]);
        return;
    }
    let mut start = 0u32;
    let mut strip = 0u32;
    if e.mem.u32(data + 0x48) == 0 {
        return;
    }
    loop {
        let lengths = e.mem.u32(data + 0x4c);
        let length = if lengths != 0 {
            u32::from(e.mem.u16(lengths.wrapping_add(strip * 2))).wrapping_sub(2)
        } else {
            let mut length = e.mem.u32(data + 0x40);
            if e.vcall(geometry, 0x2c, &args![]).u32() != 0 {
                let object = e.vcall(geometry, 0x2c, &args![]).u32();
                length = e.mem.u32(object + 0xc4);
            }
            length
        };
        if e.vcall(geometry, 0x28, &args![]).u32() == 0 {
            let device = e.mem.u32(base + 0x288);
            let words = args![
                e.mem.u32(data + 0x3c),
                e.mem.u32(data + 0x38),
                0u32,
                e.mem.u32(data + 0x18),
                start,
                length
            ];
            e.vcall(device, DRAW_INDEXED_PRIMITIVE, &words);
        } else {
            let partitions = e.vcall(geometry, 0x28, &args![]).u32();
            let single = e.mem.u8(partitions + 0xcd) != 0;
            let count = if single {
                1
            } else {
                e.mem.u32(partitions + 0xc8)
            };
            let mut offset = 0u32;
            for _ in 0..count {
                let entry = offset.wrapping_add(e.mem.u32(partitions + 0xc4));
                if e.mem.u8(entry + 0x10) != 0 {
                    let flag = e.mem.u8(partitions + 0xcd);
                    let primitives = if flag != 0 {
                        let object = e.mem.u32(partitions + 0xb8);
                        u32::from(e.mem.u16(object + 0x40))
                    } else {
                        e.mem.u32(entry + 0xc)
                    };
                    let first = if flag != 0 { 0 } else { e.mem.u32(entry) };
                    let device = e.mem.u32(base + 0x288);
                    let words = args![
                        e.mem.u32(data + 0x3c),
                        e.mem.u32(data + 0x38),
                        0u32,
                        e.mem.u32(data + 0x18),
                        first,
                        primitives
                    ];
                    e.vcall(device, DRAW_INDEXED_PRIMITIVE, &words);
                }
                offset = offset.wrapping_add(0x14);
            }
        }
        start = start.wrapping_add(length).wrapping_add(2);
        strip += 1;
        if strip >= e.mem.u32(data + 0x48) {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e7_15a0,
        fn_00e715a0(Ptr, Ptr, u32, u32, u32, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Every virtual slot of the test objects points at `SLOTS + slot`; its double records
    /// `(slot, words)` and answers the next queued answer for the slot, or 0.
    const SLOTS: u32 = 0x0e50_0000;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Answers = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    struct Scene {
        e: Engine,
        calls: Calls,
        answers: Answers,
        renderer: u32,
        geometry: u32,
        shader: u32,
        device: u32,
        state: u32,
        data: u32,
        vtable: u32,
    }

    impl Scene {
        fn answer(&self, slot: u32, values: &[u32]) {
            self.answers.borrow_mut().push((slot, values.to_vec()));
        }

        fn object(&mut self, size: u32) -> u32 {
            let object = self.e.mem.alloc(size);
            self.e.mem.set_u32(object, self.vtable);
            object
        }

        fn run(&mut self) -> Vec<(u32, Vec<u32>)> {
            self.calls.borrow_mut().clear();
            let words = args![self.renderer, self.geometry, 2u32, 3u32, 4u32, 5u32, 6u32];
            self.e.call(0x00e7_15a0, &words);
            self.calls.borrow().clone()
        }

        fn draws(&mut self) -> Vec<(u32, Vec<u32>)> {
            self.run()
                .into_iter()
                .filter(|c| c.0 == DRAW_PRIMITIVE || c.0 == DRAW_INDEXED_PRIMITIVE)
                .collect()
        }
    }

    fn scene() -> Scene {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x200);
        let calls: Calls = Rc::new(RefCell::new(vec![]));
        let answers: Answers = Rc::new(RefCell::new(vec![]));
        for slot in (0..0x200).step_by(4) {
            e.mem.set_u32(vtable + slot, SLOTS + slot);
            let record = calls.clone();
            let queued = answers.clone();
            e.register_double(SLOTS + slot, move |_, a| {
                record.borrow_mut().push((slot, a.to_vec()));
                let mut queued = queued.borrow_mut();
                let answer = match queued.iter_mut().find(|(s, q)| *s == slot && !q.is_empty()) {
                    Some((_, queue)) => queue.remove(0),
                    None => 0,
                };
                answer.into_ret()
            });
        }
        let mut s = Scene {
            e,
            calls,
            answers,
            renderer: 0,
            geometry: 0,
            shader: 0,
            device: 0,
            state: 0,
            data: 0,
            vtable,
        };
        s.shader = s.object(8);
        s.device = s.object(8);
        s.state = s.object(8);
        s.geometry = s.object(0xc4);
        let (geometry, shader) = (s.geometry, s.shader);
        s.e.mem.set_u32(geometry + 0xc0, shader);
        let renderer = s.e.mem.alloc(0x8bc);
        s.e.mem.set_u32(renderer + 0xc, 0xc0c0);
        s.e.mem.set_u32(renderer + 0x10, 0x1010);
        s.e.mem.set_u32(renderer + 0x288, s.device);
        s.e.mem.set_u32(renderer + 0x8b8, s.state);
        s.renderer = renderer;
        let data = s.e.mem.alloc(0x50);
        for offset in [0x18u32, 0x38, 0x3c, 0x40] {
            s.e.mem.set_u32(data + offset, offset);
        }
        s.data = data;
        s
    }

    #[test]
    fn fn_00e715a0_stops_when_pre_processing_refuses() {
        let mut s = scene();
        s.answer(PRE_PROCESS_PIPELINE, &[1]);
        let calls = s.run();
        assert_eq!(
            calls,
            vec![(
                PRE_PROCESS_PIPELINE,
                vec![s.shader, s.geometry, 3, 6, 0xc0c0, 0x1010, 4, 5]
            )]
        );
    }

    #[test]
    fn fn_00e715a0_without_passes_updates_and_post_processes() {
        let mut s = scene();
        let calls = s.run();
        let slots: Vec<u32> = calls.iter().map(|c| c.0).collect();
        assert_eq!(
            slots,
            vec![
                PRE_PROCESS_PIPELINE,
                UPDATE_PIPELINE,
                FIRST_PASS,
                POST_PROCESS_PIPELINE,
                0xf8
            ]
        );
        assert_eq!(
            calls[1].1,
            vec![s.shader, s.geometry, 3, 6, 0xc0c0, 0x1010, 4, 5]
        );
        assert_eq!(
            calls[3].1,
            vec![s.shader, s.geometry, 3, 6, 0xc0c0, 0x1010, 4, 5]
        );
        assert_eq!(calls[4].1, vec![s.state, 0]);
    }

    #[test]
    fn fn_00e715a0_runs_each_pass_and_carries_the_buffer_data() {
        let mut s = scene();
        let data = s.data;
        s.answer(FIRST_PASS, &[1]);
        s.answer(NEXT_PASS, &[1, 0]);
        s.answer(PREPARE_GEOMETRY, &[data, data]);
        let calls = s.run();
        let slots: Vec<u32> = calls.iter().map(|c| c.0).collect();
        let pass = [
            SETUP_RENDERING_PASS,
            SETUP_TRANSFORMATIONS,
            PREPARE_GEOMETRY,
            SETUP_SHADER_PROGRAMS,
            DRAW_PRIMITIVE,
            POST_RENDER,
            NEXT_PASS,
        ];
        let mut expected = vec![PRE_PROCESS_PIPELINE, UPDATE_PIPELINE, FIRST_PASS];
        expected.extend(pass);
        expected.extend(pass);
        expected.extend([POST_PROCESS_PIPELINE, 0xf8]);
        assert_eq!(slots, expected);
        let (g, sh) = (s.geometry, s.shader);
        // First pass: the pass word is `a6` (6).
        assert_eq!(calls[3].1, vec![sh, g, 3, 6, 0xc0c0, 0x1010, 4, 5]);
        assert_eq!(calls[4].1, vec![sh, g, 3, 0, 6, 0xc0c0, 0x1010, 4, 5]);
        assert_eq!(calls[5].1, vec![sh, g, 0, 6, 0xc0c0]);
        assert_eq!(calls[6].1, vec![sh, g, 3, 0, data, 0xc0c0, 0x1010, 4, 5]);
        assert_eq!(calls[7].1, vec![s.device, 0x3c, 0x38, 0x40]);
        assert_eq!(calls[8].1, vec![sh, g, 3, 0, data, 0xc0c0, 0x1010, 4, 5]);
        // Second pass: the previous pass's data.
        assert_eq!(calls[10].1, vec![sh, g, 3, data, 0xc0c0, 0x1010, 4, 5]);
        assert_eq!(calls[12].1, vec![sh, g, 0, data, 0xc0c0]);
        assert_eq!(calls[17].1, vec![sh, g, 3, data, 0xc0c0, 0x1010, 4, 5]);
    }

    #[test]
    fn fn_00e715a0_draws_strips_from_the_length_array() {
        let mut s = scene();
        let data = s.data;
        s.answer(FIRST_PASS, &[1]);
        s.answer(PREPARE_GEOMETRY, &[data]);
        let lengths = s.e.mem.alloc(4);
        s.e.mem.set_u16(lengths, 7);
        s.e.mem.set_u16(lengths + 2, 4);
        s.e.mem.set_u32(data + 0x34, 1);
        s.e.mem.set_u32(data + 0x48, 2);
        s.e.mem.set_u32(data + 0x4c, lengths);
        let draws = s.draws();
        assert_eq!(
            draws,
            vec![
                (
                    DRAW_INDEXED_PRIMITIVE,
                    vec![s.device, 0x3c, 0x38, 0, 0x18, 0, 5]
                ),
                (
                    DRAW_INDEXED_PRIMITIVE,
                    vec![s.device, 0x3c, 0x38, 0, 0x18, 7, 2]
                ),
            ]
        );
    }

    #[test]
    fn fn_00e715a0_strip_length_from_the_geometry_object() {
        let mut s = scene();
        let data = s.data;
        let holder = s.e.mem.alloc(0xc8);
        s.e.mem.set_u32(holder + 0xc4, 9);
        s.answer(FIRST_PASS, &[1]);
        s.answer(PREPARE_GEOMETRY, &[data]);
        s.answer(0x2c, &[0, 1, holder]);
        s.e.mem.set_u32(data + 0x34, 1);
        s.e.mem.set_u32(data + 0x48, 2);
        let draws = s.draws();
        // Strip 0: the data's +0x40; strip 1: the holder's +0xc4 (asked twice).
        assert_eq!(draws[0].1[5..], [0, 0x40]);
        assert_eq!(draws[1].1[5..], [0x42, 9]);
    }

    #[test]
    fn fn_00e715a0_draws_the_partitions() {
        let mut s = scene();
        let data = s.data;
        let partitions = s.e.mem.alloc(0xd0);
        let entries = s.e.mem.alloc(0x3c);
        for (i, (start, count, on)) in [(10u32, 3u32, 1u8), (20, 4, 0), (30, 5, 1)]
            .iter()
            .enumerate()
        {
            let entry = entries + 0x14 * i as u32;
            s.e.mem.set_u32(entry, *start);
            s.e.mem.set_u32(entry + 0xc, *count);
            s.e.mem.set_u8(entry + 0x10, *on);
        }
        s.e.mem.set_u32(partitions + 0xc4, entries);
        s.e.mem.set_u32(partitions + 0xc8, 3);
        s.answer(FIRST_PASS, &[1]);
        s.answer(PREPARE_GEOMETRY, &[data]);
        s.answer(0x28, &[1, partitions]);
        s.e.mem.set_u32(data + 0x34, 1);
        s.e.mem.set_u32(data + 0x48, 1);
        let draws = s.draws();
        assert_eq!(
            draws,
            vec![
                (
                    DRAW_INDEXED_PRIMITIVE,
                    vec![s.device, 0x3c, 0x38, 0, 0x18, 10, 3]
                ),
                (
                    DRAW_INDEXED_PRIMITIVE,
                    vec![s.device, 0x3c, 0x38, 0, 0x18, 30, 5]
                ),
            ]
        );
        // One partition with +0xcd set: start 0, the count from +0xb8's +0x40.
        let mut s = scene();
        let data = s.data;
        let partitions = s.e.mem.alloc(0xd0);
        let counts = s.e.mem.alloc(0x44);
        s.e.mem.set_u16(counts + 0x40, 0x21);
        let entries = s.e.mem.alloc(0x14);
        s.e.mem.set_u8(entries + 0x10, 1);
        s.e.mem.set_u32(entries, 77);
        s.e.mem.set_u32(partitions + 0xc4, entries);
        s.e.mem.set_u32(partitions + 0xb8, counts);
        s.e.mem.set_u8(partitions + 0xcd, 1);
        s.answer(FIRST_PASS, &[1]);
        s.answer(PREPARE_GEOMETRY, &[data]);
        s.answer(0x28, &[1, partitions]);
        s.e.mem.set_u32(data + 0x34, 1);
        s.e.mem.set_u32(data + 0x48, 1);
        let draws = s.draws();
        assert_eq!(
            draws,
            vec![(
                DRAW_INDEXED_PRIMITIVE,
                vec![s.device, 0x3c, 0x38, 0, 0x18, 0, 0x21]
            )]
        );
    }
}
